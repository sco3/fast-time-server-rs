// -*- coding: utf-8 -*-
// tools.rs - MCP tool implementations
//
// Copyright 2025
// SPDX-License-Identifier: Apache-2.0

use chrono::{DateTime, TimeZone};
use chrono_tz::Tz;
use serde_json::json;

/// Get current system time in specified timezone (non-cached version)
///
/// # Errors
/// Returns an error if the timezone is invalid.
pub fn get_system_time(timezone: &str) -> Result<String, String> {
    let tz: Tz = timezone
        .parse()
        .map_err(|_| format!("Invalid timezone: {timezone}"))?;
    let now = chrono::Utc::now().with_timezone(&tz);
    Ok(now.format("%Y-%m-%dT%H:%M:%SZ").to_string())
}

/// Get current system time in specified timezone (cached version)
///
/// # Errors
/// Returns an error if the timezone is invalid.
pub async fn get_system_time_cached(
    timezone: &str,
    app_state: &crate::AppState,
) -> Result<String, String> {
    let tz = app_state.load_timezone(timezone).await?;
    let now = chrono::Utc::now().with_timezone(&tz);
    Ok(now.format("%Y-%m-%dT%H:%M:%SZ").to_string())
}

/// Convert time between timezones (non-cached version)
///
/// # Errors
/// Returns an error if the timezones are invalid or the time format is unrecognized.
pub fn convert_time(time_str: &str, source_tz: &str, target_tz: &str) -> Result<String, String> {
    let source: Tz = source_tz
        .parse()
        .map_err(|_| format!("Invalid source timezone: {source_tz}"))?;
    let target: Tz = target_tz
        .parse()
        .map_err(|_| format!("Invalid target timezone: {target_tz}"))?;

    // Try parsing as RFC3339 first
    let parsed_time = if let Ok(dt) = DateTime::parse_from_rfc3339(time_str) {
        dt.with_timezone(&source)
    } else if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(time_str, "%Y-%m-%d %H:%M:%S") {
        source
            .from_local_datetime(&dt)
            .single()
            .ok_or("Ambiguous time")?
    } else if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(time_str, "%Y-%m-%dT%H:%M:%S") {
        source
            .from_local_datetime(&dt)
            .single()
            .ok_or("Ambiguous time")?
    } else if let Ok(date) = chrono::NaiveDate::parse_from_str(time_str, "%Y-%m-%d") {
        let dt = date.and_hms_opt(0, 0, 0).ok_or("Invalid date")?;
        source
            .from_local_datetime(&dt)
            .single()
            .ok_or("Ambiguous time")?
    } else {
        return Err(format!("Invalid time format: {time_str}"));
    };

    let converted = parsed_time.with_timezone(&target);
    Ok(converted.format("%Y-%m-%dT%H:%M:%SZ").to_string())
}

/// Convert time between timezones (cached version)
///
/// # Errors
/// Returns an error if the timezones are invalid or the time format is unrecognized.
pub async fn convert_time_cached(
    time_str: &str,
    source_tz: &str,
    target_tz: &str,
    app_state: &crate::AppState,
) -> Result<String, String> {
    let source = app_state.load_timezone(source_tz).await?;
    let target = app_state.load_timezone(target_tz).await?;

    // Try parsing as RFC3339 first
    let parsed_time = if let Ok(dt) = DateTime::parse_from_rfc3339(time_str) {
        dt.with_timezone(&source)
    } else if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(time_str, "%Y-%m-%d %H:%M:%S") {
        source
            .from_local_datetime(&dt)
            .single()
            .ok_or("Ambiguous time")?
    } else if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(time_str, "%Y-%m-%dT%H:%M:%S") {
        source
            .from_local_datetime(&dt)
            .single()
            .ok_or("Ambiguous time")?
    } else if let Ok(date) = chrono::NaiveDate::parse_from_str(time_str, "%Y-%m-%d") {
        let dt = date.and_hms_opt(0, 0, 0).ok_or("Invalid date")?;
        source
            .from_local_datetime(&dt)
            .single()
            .ok_or("Ambiguous time")?
    } else {
        return Err(format!("Invalid time format: {time_str}"));
    };

    let converted = parsed_time.with_timezone(&target);
    Ok(converted.format("%Y-%m-%dT%H:%M:%SZ").to_string())
}

/// Handle MCP tool call (non-cached version for stdio mode)
///
/// # Errors
/// Returns an error if the tool name is unknown or required parameters are missing.
pub fn handle_tool_call(
    name: &str,
    arguments: &serde_json::Value,
) -> Result<serde_json::Value, String> {
    match name {
        "get_system_time" => {
            let timezone = arguments
                .get("timezone")
                .and_then(|v| v.as_str())
                .unwrap_or("UTC");
            let time = get_system_time(timezone)?;
            Ok(json!({
                "content": [
                    {
                        "type": "text",
                        "text": time
                    }
                ]
            }))
        }
        "convert_time" => {
            let time = arguments
                .get("time")
                .and_then(|v| v.as_str())
                .ok_or("time parameter is required")?;
            let source_tz = arguments
                .get("source_timezone")
                .and_then(|v| v.as_str())
                .ok_or("source_timezone parameter is required")?;
            let target_tz = arguments
                .get("target_timezone")
                .and_then(|v| v.as_str())
                .ok_or("target_timezone parameter is required")?;

            let converted = convert_time(time, source_tz, target_tz)?;
            Ok(json!({
                "content": [
                    {
                        "type": "text",
                        "text": converted
                    }
                ]
            }))
        }
        _ => Err(format!("Unknown tool: {name}")),
    }
}

/// Handle MCP tool call with caching (for HTTP/SSE modes)
///
/// # Errors
/// Returns an error if the tool name is unknown or required parameters are missing.
pub async fn handle_tool_call_cached(
    name: &str,
    arguments: &serde_json::Value,
    app_state: &crate::AppState,
) -> Result<serde_json::Value, String> {
    match name {
        "get_system_time" => {
            let timezone = arguments
                .get("timezone")
                .and_then(|v| v.as_str())
                .unwrap_or("UTC");
            let time = get_system_time_cached(timezone, app_state).await?;
            Ok(json!({
                "content": [
                    {
                        "type": "text",
                        "text": time
                    }
                ]
            }))
        }
        "convert_time" => {
            let time = arguments
                .get("time")
                .and_then(|v| v.as_str())
                .ok_or("time parameter is required")?;
            let source_tz = arguments
                .get("source_timezone")
                .and_then(|v| v.as_str())
                .ok_or("source_timezone parameter is required")?;
            let target_tz = arguments
                .get("target_timezone")
                .and_then(|v| v.as_str())
                .ok_or("target_timezone parameter is required")?;

            let converted = convert_time_cached(time, source_tz, target_tz, app_state).await?;
            Ok(json!({
                "content": [
                    {
                        "type": "text",
                        "text": converted
                    }
                ]
            }))
        }
        _ => Err(format!("Unknown tool: {name}")),
    }
}
