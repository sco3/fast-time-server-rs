// -*- coding: utf-8 -*-
// lib.rs - Library exports for fast-time-server
//
// Copyright 2025
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub mod mcp;
pub mod openapi;
pub mod prompts;
pub mod resources;
pub mod tools;

// Re-export AppState for tests and rest_handlers
#[derive(Clone)]
pub struct AppState {
    pub auth_token: Option<String>,
    pub start_time: std::time::Instant,
    pub timezone_cache: Arc<RwLock<HashMap<String, chrono_tz::Tz>>>,
}

// Don't export rest_handlers module as it has runtime dependencies

// ============================================================================
// Phase 1 Refactoring: Extracted Pure Functions for Testing
// ============================================================================

/// Parse log level string to tracing filter string
///
/// # Examples
/// ```
/// use fast_time_server::parse_log_level;
/// assert_eq!(parse_log_level("debug"), "debug");
/// assert_eq!(parse_log_level("INFO"), "info");
/// assert_eq!(parse_log_level("invalid"), "info"); // defaults to info
/// ```
pub fn parse_log_level(level: &str) -> &'static str {
    match level.to_lowercase().as_str() {
        "debug" => "debug",
        "info" => "info",
        "warn" | "warning" => "warn",
        "error" => "error",
        "none" | "off" => "off",
        _ => "info", // default
    }
}

/// Format listen address from optional full address or host+port
///
/// # Examples
/// ```
/// use fast_time_server::format_listen_address;
///
/// // With explicit address - use it directly
/// assert_eq!(format_listen_address(Some("127.0.0.1:9090"), "0.0.0.0", 8080), "127.0.0.1:9090");
///
/// // Without explicit address - construct from host and port
/// assert_eq!(format_listen_address(None, "0.0.0.0", 8080), "0.0.0.0:8080");
/// assert_eq!(format_listen_address(None, "localhost", 3000), "localhost:3000");
/// ```
pub fn format_listen_address(addr: Option<&str>, host: &str, port: u16) -> String {
    addr.map(String::from)
        .unwrap_or_else(|| format!("{}:{}", host, port))
}

/// Build health check response JSON
///
/// # Examples
/// ```
/// use fast_time_server::build_health_response;
///
/// let response = build_health_response(42);
/// assert_eq!(response.get("status").unwrap().as_str().unwrap(), "healthy");
/// assert_eq!(response.get("uptime_seconds").unwrap().as_u64().unwrap(), 42);
/// ```
pub fn build_health_response(uptime_secs: u64) -> serde_json::Value {
    serde_json::json!({
        "status": "healthy",
        "uptime_seconds": uptime_secs
    })
}

/// Build version information response JSON
///
/// # Examples
/// ```
/// use fast_time_server::build_version_response;
///
/// let response = build_version_response("test-server", "1.0.0");
/// assert_eq!(response.get("name").unwrap().as_str().unwrap(), "test-server");
/// assert_eq!(response.get("version").unwrap().as_str().unwrap(), "1.0.0");
/// assert_eq!(response.get("mcp_version").unwrap().as_str().unwrap(), "1.0");
/// ```
pub fn build_version_response(name: &str, version: &str) -> serde_json::Value {
    serde_json::json!({
        "name": name,
        "version": version,
        "mcp_version": "1.0"
    })
}
impl AppState {
    /// Load timezone with caching (similar to Go's loadLocation function)
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
            .map_err(|_| format!("invalid timezone {}", name))?;
        self.timezone_cache
            .write()
            .await
            .insert(name.to_string(), tz);
        Ok(tz)
    }
}
