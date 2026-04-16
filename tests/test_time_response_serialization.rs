// -*- coding: utf-8 -*-
// test_time_response_serialization.rs - Test TimeResponse can be serialized
//
// Copyright 2025
// SPDX-License-Identifier: Apache-2.0

#[derive(serde::Serialize)]
struct TimeResponse {
    time: String,
    timezone: String,
    unix: i64,
    utc: String,
}

#[test]
fn test_time_response_serialization() {
    let response = TimeResponse {
        time: "2026-04-15T19:00:00+00:00".to_string(),
        timezone: "UTC".to_string(),
        unix: 1_776_024_000,
        utc: "2026-04-15T19:00:00+00:00".to_string(),
    };

    // Serialize to JSON
    let json = serde_json::to_string(&response).expect("Should serialize to JSON");

    // Verify it's valid JSON
    assert!(!json.is_empty(), "JSON should not be empty");

    // Parse back to verify structure
    let parsed: serde_json::Value = serde_json::from_str(&json).expect("Should parse as JSON");

    assert!(parsed.get("time").is_some(), "Should have 'time' field");
    assert!(
        parsed.get("timezone").is_some(),
        "Should have 'timezone' field"
    );
    assert!(parsed.get("unix").is_some(), "Should have 'unix' field");
    assert!(parsed.get("utc").is_some(), "Should have 'utc' field");

    assert_eq!(
        parsed.get("time").unwrap().as_str().unwrap(),
        "2026-04-15T19:00:00+00:00"
    );
    assert_eq!(parsed.get("timezone").unwrap().as_str().unwrap(), "UTC");
    assert_eq!(parsed.get("unix").unwrap().as_i64().unwrap(), 1_776_024_000);
    assert_eq!(
        parsed.get("utc").unwrap().as_str().unwrap(),
        "2026-04-15T19:00:00+00:00"
    );
}
