// -*- coding: utf-8 -*-
// fast-time-server - ultra-fast MCP server exposing time-related tools
//
// Copyright 2025
// SPDX-License-Identifier: Apache-2.0
// Authors: Mihai Criveti, Manav Gupta

use axum::{
    middleware,
    extract::State,
    http::StatusCode,
    response::sse::{Event, KeepAlive, Sse},
    routing::{get, post},
    Json, Router,
};
use clap::Parser;
use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::cors::{Any, CorsLayer};
use tracing::{error, info};

mod mcp;
mod rest_handlers;
mod resources;
mod tools;
mod prompts;
mod openapi;

use mcp::*;

const APP_NAME: &str = "fast-time-server";
const APP_VERSION: &str = "1.5.0";
const DEFAULT_PORT: u16 = 8080;

#[derive(Parser, Debug)]
#[command(name = APP_NAME, version = APP_VERSION)]
struct Args {
    #[arg(short, long, default_value = "stdio")]
    transport: String,
    #[arg(long)]
    addr: Option<String>,
    #[arg(long, default_value = "0.0.0.0")]
    listen: String,
    #[arg(short, long, default_value_t = DEFAULT_PORT)]
    port: u16,
    #[arg(long)]
    public_url: Option<String>,
    #[arg(long)]
    auth_token: Option<String>,
    #[arg(long, default_value = "info")]
    log_level: String,
}

#[derive(Clone)]
pub struct AppState {
    auth_token: Option<String>,
    start_time: std::time::Instant,
    #[allow(dead_code)]
    timezone_cache: Arc<RwLock<HashMap<String, chrono_tz::Tz>>>,
}

impl AppState {
    fn new(auth_token: Option<String>) -> Self {
        Self {
            auth_token,
            start_time: std::time::Instant::now(),
            timezone_cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    init_logging(&args.log_level);

    info!("Starting {} {}", APP_NAME, APP_VERSION);
    let state = AppState::new(args.auth_token.clone());

    match args.transport.as_str() {
        "stdio" => run_stdio_mode(state).await?,
        "http" => run_http_mode(args, state).await?,
        "sse" => run_sse_mode(args, state).await?,
        "dual" => run_dual_mode(args, state).await?,
        "rest" => run_rest_mode(args, state).await?,
        _ => {
            error!("Invalid transport mode: {}", args.transport);
            std::process::exit(1);
        }
    }
    Ok(())
}

fn init_logging(level: &str) {
    let filter = match level.to_lowercase().as_str() {
        "debug" => "debug",
        "info" => "info",
        "warn" => "warn",
        "error" => "error",
        _ => "info",
    };
    tracing_subscriber::fmt().with_env_filter(filter).with_target(false).init();
}

async fn run_stdio_mode(_state: AppState) -> anyhow::Result<()> {
    info!("Running in STDIO mode");
    tokio::signal::ctrl_c().await?;
    Ok(())
}

async fn run_http_mode(args: Args, state: AppState) -> anyhow::Result<()> {
    let addr = get_listen_address(&args);
    info!("HTTP server listening on http://{}", addr);
    let app = create_http_router(state);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn run_sse_mode(args: Args, state: AppState) -> anyhow::Result<()> {
    let addr = get_listen_address(&args);
    info!("SSE server listening on http://{}", addr);
    let app = create_sse_router(state);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn run_dual_mode(args: Args, state: AppState) -> anyhow::Result<()> {
    let addr = get_listen_address(&args);
    info!("DUAL server listening on http://{}", addr);
    let app = create_dual_router(state);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn run_rest_mode(args: Args, state: AppState) -> anyhow::Result<()> {
    let addr = get_listen_address(&args);
    info!("REST API server listening on http://{}", addr);
    let app = create_rest_router(state);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

fn get_listen_address(args: &Args) -> String {
    args.addr.clone().unwrap_or_else(|| format!("{}:{}", args.listen, args.port))
}





async fn handle_health(State(state): State<AppState>) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "uptime_seconds": state.start_time.elapsed().as_secs()
    }))
}

async fn handle_version() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "name": APP_NAME,
        "version": APP_VERSION,
        "mcp_version": "1.0"
    }))
}

async fn handle_jsonrpc(
    State(_state): State<AppState>,
    Json(request): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let method = request.get("method").and_then(|v| v.as_str()).unwrap_or("");
    let id = request.get("id").cloned();
    
    let response = match method {
        "initialize" => JsonRpcResponse::success(id, serde_json::json!({
            "protocolVersion": "1.0",
            "capabilities": {"tools": {}, "resources": {}, "prompts": {}}, "serverInfo": {"name": APP_NAME, "version": APP_VERSION}
        })),
        "tools/list" => JsonRpcResponse::success(id, serde_json::json!({
            "tools": [
                {"name": "get_system_time", "description": "Get current system time"},
                {"name": "convert_time", "description": "Convert time between timezones"}
            ]
        })),
        "tools/call" => {
            let params = request.get("params").ok_or(StatusCode::BAD_REQUEST)?;
            let tool_name = params.get("name").and_then(|v| v.as_str()).ok_or(StatusCode::BAD_REQUEST)?;
            let default_args = serde_json::json!({});
            let arguments = params.get("arguments").unwrap_or(&default_args);
            
            match tools::handle_tool_call(tool_name, arguments) {
                Ok(result) => JsonRpcResponse::success(id, result),
                Err(e) => JsonRpcResponse::error(id, -32000, e),
            }
        },
        _ => JsonRpcResponse::error(id, -32601, format!("Method not found: {}", method)),
    };
    
    Ok(Json(serde_json::json!(response)))
}

async fn handle_sse() -> Sse<impl futures::Stream<Item = Result<Event, Infallible>>> {
    let stream = async_stream::stream! {
        yield Ok(Event::default().data("Connected to SSE"));
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
            yield Ok(Event::default().event("ping").data("keepalive"));
        }
    };
    Sse::new(stream).keep_alive(KeepAlive::default())
}

async fn handle_sse_message(
    Json(request): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({"status": "received", "request": request}))
}


/// Authentication middleware - checks Bearer token
async fn auth_middleware(
    State(state): State<AppState>,
    request: axum::extract::Request,
    next: middleware::Next,
) -> Result<axum::response::Response, StatusCode> {
    // Skip auth for health and version endpoints
    let path = request.uri().path();
    if path == "/health" || path == "/version" {
        return Ok(next.run(request).await);
    }
    
    // If no auth token configured, skip authentication
    let Some(ref expected_token) = state.auth_token else {
        return Ok(next.run(request).await);
    };
    
    // Get Authorization header
    let auth_header = request
        .headers()
        .get("authorization")
        .and_then(|v| v.to_str().ok());
    
    let Some(auth_value) = auth_header else {
        return Err(StatusCode::UNAUTHORIZED);
    };
    
    // Check Bearer token format
    if !auth_value.starts_with("Bearer ") {
        return Err(StatusCode::UNAUTHORIZED);
    }
    
    let token = &auth_value[7..]; // Skip "Bearer "
    if token != expected_token {
        return Err(StatusCode::UNAUTHORIZED);
    }
    
    Ok(next.run(request).await)
}

fn create_http_router(state: AppState) -> Router {
    let router = Router::new()
        .route("/", post(handle_jsonrpc))
        .route("/health", get(handle_health))
        .route("/version", get(handle_version))
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any));
    
    if state.auth_token.is_some() {
        router
            .layer(middleware::from_fn_with_state(state.clone(), auth_middleware))
            .with_state(state)
    } else {
        router.with_state(state)
    }
}

fn create_sse_router(state: AppState) -> Router {
    let router = Router::new()
        .route("/sse", get(handle_sse))
        .route("/messages", post(handle_sse_message))
        .route("/message", post(handle_sse_message))
        .route("/health", get(handle_health))
        .route("/version", get(handle_version))
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any));
    
    if state.auth_token.is_some() {
        router
            .layer(middleware::from_fn_with_state(state.clone(), auth_middleware))
            .with_state(state)
    } else {
        router.with_state(state)
    }
}

fn create_dual_router(state: AppState) -> Router {
    let rest_routes = rest_handlers::create_rest_routes();
    let router = Router::new()
        .route("/sse", get(handle_sse))
        .route("/messages", post(handle_sse_message))
        .route("/message", post(handle_sse_message))
        .route("/http", post(handle_jsonrpc))
        .merge(rest_routes)
        .route("/health", get(handle_health))
        .route("/version", get(handle_version))
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any));
    
    if state.auth_token.is_some() {
        router
            .layer(middleware::from_fn_with_state(state.clone(), auth_middleware))
            .with_state(state)
    } else {
        router.with_state(state)
    }
}

fn create_rest_router(state: AppState) -> Router {
    let rest_routes = rest_handlers::create_rest_routes();
    let router = Router::new()
        .merge(rest_routes)
        .route("/health", get(handle_health))
        .route("/version", get(handle_version))
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any));
    
    if state.auth_token.is_some() {
        router
            .layer(middleware::from_fn_with_state(state.clone(), auth_middleware))
            .with_state(state)
    } else {
        router.with_state(state)
    }
}
