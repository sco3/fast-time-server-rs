// -*- coding: utf-8 -*-
// fast-time-server - ultra-fast MCP server exposing time-related tools
//
// Copyright 2025
// SPDX-License-Identifier: Apache-2.0
// Authors: Mihai Criveti, Manav Gupta

use axum::{
    extract::State,
    http::StatusCode,
    middleware,
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
mod openapi;
mod prompts;
mod resources;
mod rest_handlers;
mod tools;

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

    /// Load timezone with caching (similar to Go's `loadLocation` function)
    ///
    /// # Errors
    ///
    /// Returns an error if the timezone name is invalid.
    pub async fn load_timezone(&self, name: &str) -> Result<chrono_tz::Tz, String> {
        // Check cache first
        {
            let cache = self.timezone_cache.read().await;
            if let Some(&tz) = cache.get(name) {
                return Ok(tz);
            }
        }

        // Parse and cache
        let tz: chrono_tz::Tz = name
            .parse()
            .map_err(|_| format!("invalid timezone {name}"))?;
        self.timezone_cache
            .write()
            .await
            .insert(name.to_string(), tz);
        Ok(tz)
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
        "warn" => "warn",
        "error" => "error",
        _ => "info",
    };
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .init();
}

#[allow(clippy::too_many_lines)]
async fn run_stdio_mode(_state: AppState) -> anyhow::Result<()> {
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

    info!("Running in STDIO mode - ready to process MCP requests");

    let stdin = tokio::io::stdin();
    let mut stdout = tokio::io::stdout();
    let mut reader = BufReader::new(stdin);
    let mut line = String::new();

    loop {
        line.clear();

        // Read a line from stdin
        let bytes_read = reader.read_line(&mut line).await?;

        // EOF means the input stream is closed
        if bytes_read == 0 {
            info!("stdin closed, shutting down");
            break;
        }

        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        // Parse the JSON-RPC request
        let request: serde_json::Value = match serde_json::from_str(trimmed) {
            Ok(req) => req,
            Err(e) => {
                error!("Failed to parse JSON-RPC request: {}", e);
                let error_response = serde_json::json!({
                    "jsonrpc": "2.0",
                    "id": null,
                    "error": {
                        "code": -32700,
                        "message": "Parse error"
                    }
                });
                let response_str = serde_json::to_string(&error_response)?;
                stdout.write_all(response_str.as_bytes()).await?;
                stdout.write_all(b"\n").await?;
                stdout.flush().await?;
                continue;
            }
        };

        // Extract the method and id
        let method = request.get("method").and_then(|m| m.as_str());
        let id = request.get("id").cloned();

        let response = match method {
            Some("initialize") => {
                info!("Handling initialize request");
                JsonRpcResponse::success(
                    id,
                    serde_json::json!({
                        "protocolVersion": "1.0",
                        "capabilities": {"tools": {}, "resources": {}, "prompts": {}},
                        "serverInfo": {"name": APP_NAME, "version": APP_VERSION}
                    }),
                )
            }
            Some("tools/list") => {
                info!("Handling tools/list request");
                JsonRpcResponse::success(
                    id,
                    serde_json::json!({
                        "tools": [
                            {"name": "get_system_time", "description": "Get current system time"},
                            {"name": "convert_time", "description": "Convert time between timezones"}
                        ]
                    }),
                )
            }
            Some("resources/list") => {
                info!("Handling resources/list request");
                JsonRpcResponse::success(
                    id,
                    serde_json::json!({
                        "resources": [
                            {"uri": "timezone://info", "name": "Timezone Information", "mimeType": "application/json"},
                            {"uri": "time://current/world", "name": "Current World Times", "mimeType": "application/json"},
                            {"uri": "time://formats", "name": "Time Formats", "mimeType": "application/json"},
                            {"uri": "time://business-hours", "name": "Business Hours", "mimeType": "application/json"}
                        ]
                    }),
                )
            }
            Some("resources/read") => {
                info!("Handling resources/read request");
                let params = request.get("params");
                if let Some(params_obj) = params {
                    let resource_uri = params_obj
                        .get("uri")
                        .and_then(|v| v.as_str())
                        .unwrap_or("");

                    let response = match resource_uri {
                        "timezone://info" => {
                            info!("resource: timezone info requested");
                            JsonRpcResponse::success(
                                id,
                                serde_json::json!({
                                    "contents": [
                                        {
                                            "uri": resource_uri,
                                            "mimeType": "application/json",
                                            "text": serde_json::to_string(&resources::get_timezone_info()).unwrap_or_default()
                                        }
                                    ]
                                }),
                            )
                        }
                        "time://current/world" => {
                            info!("resource: current world times requested");
                            JsonRpcResponse::success(
                                id,
                                serde_json::json!({
                                    "contents": [
                                        {
                                            "uri": resource_uri,
                                            "mimeType": "application/json",
                                            "text": serde_json::to_string(&resources::get_current_world_times()).unwrap_or_default()
                                        }
                                    ]
                                }),
                            )
                        }
                        "time://formats" => {
                            info!("resource: time formats requested");
                            JsonRpcResponse::success(
                                id,
                                serde_json::json!({
                                    "contents": [
                                        {
                                            "uri": resource_uri,
                                            "mimeType": "application/json",
                                            "text": serde_json::to_string(&resources::get_time_formats()).unwrap_or_default()
                                        }
                                    ]
                                }),
                            )
                        }
                        "time://business-hours" => {
                            info!("resource: business hours requested");
                            JsonRpcResponse::success(
                                id,
                                serde_json::json!({
                                    "contents": [
                                        {
                                            "uri": resource_uri,
                                            "mimeType": "application/json",
                                            "text": serde_json::to_string(&resources::get_business_hours()).unwrap_or_default()
                                        }
                                    ]
                                }),
                            )
                        }
                        _ => {
                            JsonRpcResponse::error(
                                id,
                                -32000,
                                format!("Resource not found: {resource_uri}"),
                            )
                        }
                    };
                    response
                } else {
                    JsonRpcResponse::error(
                        id,
                        -32600,
                        "Invalid Request - missing params".to_string(),
                    )
                }
            }
            Some("prompts/list") => {
                info!("Handling prompts/list request");
                JsonRpcResponse::success(
                    id,
                    serde_json::json!({
                        "prompts": []
                    }),
                )
            }
            Some("tools/call") => {
                info!("Handling tools/call request");
                let params = request.get("params");
                if let Some(params_obj) = params {
                    let tool_name = params_obj
                        .get("name")
                        .and_then(|v| v.as_str())
                        .unwrap_or("");
                    let default_args = serde_json::json!({});
                    let arguments = params_obj.get("arguments").unwrap_or(&default_args);

                    match tools::handle_tool_call(tool_name, arguments) {
                        Ok(result) => JsonRpcResponse::success(id, result),
                        Err(e) => JsonRpcResponse::error(id, -32000, e),
                    }
                } else {
                    JsonRpcResponse::error(
                        id,
                        -32600,
                        "Invalid Request - missing params".to_string(),
                    )
                }
            }
            Some(other_method) => {
                info!("Unknown method: {other_method}");
                JsonRpcResponse::error(id, -32601, format!("Method not found: {other_method}"))
            }
            None => {
                error!("Request missing 'method' field");
                JsonRpcResponse::error(id, -32600, "Invalid Request - missing method".to_string())
            }
        };

        // Write the response to stdout
        let response_str = serde_json::to_string(&serde_json::json!(response))?;
        stdout.write_all(response_str.as_bytes()).await?;
        stdout.write_all(b"\n").await?;
        stdout.flush().await?;
    }

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
    args.addr
        .clone()
        .unwrap_or_else(|| format!("{}:{}", args.listen, args.port))
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
    State(state): State<AppState>,
    Json(request): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let method = request.get("method").and_then(|v| v.as_str()).unwrap_or("");
    let id = request.get("id").cloned();

    let response = match method {
        "initialize" => JsonRpcResponse::success(
            id,
            serde_json::json!({
                "protocolVersion": "1.0",
                "capabilities": {"tools": {}, "resources": {}, "prompts": {}}, "serverInfo": {"name": APP_NAME, "version": APP_VERSION}
            }),
        ),
        "tools/list" => JsonRpcResponse::success(
            id,
            serde_json::json!({
                "tools": [
                    {"name": "get_system_time", "description": "Get current system time"},
                    {"name": "convert_time", "description": "Convert time between timezones"}
                ]
            }),
        ),
        "resources/list" => JsonRpcResponse::success(
            id,
            serde_json::json!({
                "resources": [
                    {"uri": "timezone://info", "name": "Timezone Information", "mimeType": "application/json"},
                    {"uri": "time://current/world", "name": "Current World Times", "mimeType": "application/json"},
                    {"uri": "time://formats", "name": "Time Formats", "mimeType": "application/json"},
                    {"uri": "time://business-hours", "name": "Business Hours", "mimeType": "application/json"}
                ]
            }),
        ),
        "resources/read" => {
            let params = request.get("params").ok_or(StatusCode::BAD_REQUEST)?;
            let resource_uri = params
                .get("uri")
                .and_then(|v| v.as_str())
                .ok_or(StatusCode::BAD_REQUEST)?;

            let result = match resource_uri {
                "timezone://info" => {
                    info!("resource: timezone info requested");
                    resources::get_timezone_info()
                }
                "time://current/world" => {
                    info!("resource: current world times requested");
                    resources::get_current_world_times_cached(&state).await
                }
                "time://formats" => {
                    info!("resource: time formats requested");
                    resources::get_time_formats()
                }
                "time://business-hours" => {
                    info!("resource: business hours requested");
                    resources::get_business_hours()
                }
                _ => {
                    return Err(StatusCode::NOT_FOUND);
                }
            };

            JsonRpcResponse::success(
                id,
                serde_json::json!({
                    "contents": [
                        {
                            "uri": resource_uri,
                            "mimeType": "application/json",
                            "text": serde_json::to_string(&result).unwrap_or_default()
                        }
                    ]
                }),
            )
        }
        "tools/call" => {
            let params = request.get("params").ok_or(StatusCode::BAD_REQUEST)?;
            let tool_name = params
                .get("name")
                .and_then(|v| v.as_str())
                .ok_or(StatusCode::BAD_REQUEST)?;
            let default_args = serde_json::json!({});
            let arguments = params.get("arguments").unwrap_or(&default_args);

            match tools::handle_tool_call_cached(tool_name, arguments, &state).await {
                Ok(result) => JsonRpcResponse::success(id, result),
                Err(e) => JsonRpcResponse::error(id, -32000, e),
            }
        }
        _ => JsonRpcResponse::error(id, -32601, format!("Method not found: {method}")),
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

async fn handle_sse_message(Json(request): Json<serde_json::Value>) -> Json<serde_json::Value> {
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
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );

    if state.auth_token.is_some() {
        router
            .layer(middleware::from_fn_with_state(
                state.clone(),
                auth_middleware,
            ))
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
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );

    if state.auth_token.is_some() {
        router
            .layer(middleware::from_fn_with_state(
                state.clone(),
                auth_middleware,
            ))
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
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );

    if state.auth_token.is_some() {
        router
            .layer(middleware::from_fn_with_state(
                state.clone(),
                auth_middleware,
            ))
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
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );

    if state.auth_token.is_some() {
        router
            .layer(middleware::from_fn_with_state(
                state.clone(),
                auth_middleware,
            ))
            .with_state(state)
    } else {
        router.with_state(state)
    }
}
