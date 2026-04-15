// -*- coding: utf-8 -*-
// rest_handlers.rs - REST API handlers
//
// Copyright 2025
// SPDX-License-Identifier: Apache-2.0

use axum::{
    extract::{Path, Query},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::AppState;

#[derive(Serialize)]
struct TimeResponse {
    time: String,
    timezone: String,
    unix: i64,
    utc: String,
}

#[derive(Deserialize)]
struct TimeQuery {
    timezone: Option<String>,
}

/// Handle GET /api/v1/time
async fn handle_get_time(
    Query(params): Query<TimeQuery>,
) -> Result<Json<TimeResponse>, StatusCode> {
    let timezone = params.timezone.unwrap_or_else(|| "UTC".to_string());
    
    // Get current time
    let tz: chrono_tz::Tz = timezone.parse().map_err(|_| StatusCode::BAD_REQUEST)?;
    let now = chrono::Utc::now().with_timezone(&tz);
    let utc = chrono::Utc::now();
    
    Ok(Json(TimeResponse {
        time: now.to_rfc3339(),
        timezone: timezone,
        unix: now.timestamp(),
        utc: utc.to_rfc3339(),
    }))
}

/// Handle GET /api/v1/time/{timezone}
async fn handle_get_time_with_path(
    Path(timezone): Path<String>,
) -> Result<Json<TimeResponse>, StatusCode> {
    // Get current time
    let tz: chrono_tz::Tz = timezone.parse().map_err(|_| StatusCode::BAD_REQUEST)?;
    let now = chrono::Utc::now().with_timezone(&tz);
    let utc = chrono::Utc::now();
    
    Ok(Json(TimeResponse {
        time: now.to_rfc3339(),
        timezone: timezone,
        unix: now.timestamp(),
        utc: utc.to_rfc3339(),
    }))
}

/// Handle GET /api/v1/openapi.json
async fn handle_openapi_spec() -> Json<serde_json::Value> {
    Json(crate::openapi::get_openapi_spec())
}

/// Handle GET /api/v1/docs - Serve Swagger UI
async fn handle_api_docs() -> axum::response::Html<String> {
    let html = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Fast Time Server API Documentation</title>
    <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/swagger-ui-dist@5/swagger-ui.css">
</head>
<body>
    <div id="swagger-ui"></div>
    <script src="https://cdn.jsdelivr.net/npm/swagger-ui-dist@5/swagger-ui-bundle.js"></script>
    <script>
        window.onload = function() {
            SwaggerUIBundle({
                url: "/api/v1/openapi.json",
                dom_id: '#swagger-ui',
                presets: [
                    SwaggerUIBundle.presets.apis,
                    SwaggerUIBundle.SwaggerUIStandalonePreset
                ],
                layout: "BaseLayout"
            });
        }
    </script>
</body>
</html>"#;
    
    axum::response::Html(html.to_string())
}

use serde_json::json;

/// Request body for prompt execution
#[derive(Deserialize)]
struct PromptExecuteRequest {
    #[serde(flatten)]
    arguments: HashMap<String, String>,
}

/// Handle POST /api/v1/prompts/:name/execute
async fn handle_execute_prompt(
    Path(name): Path<String>,
    Json(payload): Json<PromptExecuteRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let prompt_text = match name.as_str() {
        "compare_timezones" => {
            let timezones = payload.arguments.get("timezones").map(|s| s.as_str()).unwrap_or("");
            let reference_time = payload.arguments.get("reference_time").map(|s| s.as_str());
            crate::prompts::generate_compare_timezones_prompt(timezones, reference_time)
        }
        "schedule_meeting" => {
            let participants = payload.arguments.get("participants").map(|s| s.as_str()).unwrap_or("");
            let duration = payload.arguments.get("duration").map(|s| s.as_str());
            let preferred_hours = payload.arguments.get("preferred_hours").map(|s| s.as_str());
            let date_range = payload.arguments.get("date_range").map(|s| s.as_str());
            crate::prompts::generate_schedule_meeting_prompt(participants, duration, preferred_hours, date_range)
        }
        "convert_time_detailed" => {
            let time = payload.arguments.get("time").map(|s| s.as_str()).unwrap_or("");
            let from_timezone = payload.arguments.get("from_timezone").map(|s| s.as_str()).unwrap_or("");
            let to_timezones = payload.arguments.get("to_timezones").map(|s| s.as_str()).unwrap_or("");
            let include_context = payload.arguments.get("include_context")
                .and_then(|s| s.parse::<bool>().ok())
                .unwrap_or(false);
            crate::prompts::generate_convert_time_detailed_prompt(time, from_timezone, to_timezones, include_context)
        }
        _ => {
            return Err(StatusCode::NOT_FOUND);
        }
    };

    Ok(Json(json!({
        "prompt": name,
        "arguments": payload.arguments,
        "text": prompt_text
    })))
}

/// Handle GET /api/v1/resources/:uri
async fn handle_get_resource(
    Path(uri): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let resource = match uri.as_str() {
        "timezone-info" => crate::resources::get_timezone_info(),
        "current-world" => crate::resources::get_current_world_times(),
        "time-formats" => crate::resources::get_time_formats(),
        "business-hours" => crate::resources::get_business_hours(),
        _ => return Err(StatusCode::NOT_FOUND),
    };
    
    Ok(Json(resource))
}

/// Create REST routes
pub fn create_rest_routes() -> Router<AppState> {
    Router::new()
        .route("/api/v1/time", get(handle_get_time))
        .route("/api/v1/time/:timezone", get(handle_get_time_with_path))
        .route("/api/v1/openapi.json", get(handle_openapi_spec))
        .route("/api/v1/docs", get(handle_api_docs))
        .route("/api/v1/prompts/:name/execute", axum::routing::post(handle_execute_prompt))
        .route("/api/v1/resources/:uri", get(handle_get_resource))
}
