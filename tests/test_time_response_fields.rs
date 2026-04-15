// -*- coding: utf-8 -*-
// test_time_response_fields.rs - Test TimeResponse has all required fields
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
fn test_time_response_fields() {
    let response = TimeResponse {
        time: "2026-01-01T00:00:00Z".to_string(),
        timezone: "America/New_York".to_string(),
        unix: 1735689600,
        utc: "2026-01-01T05:00:00Z".to_string(),
    };
    
    let json = serde_json::to_value(&response).expect("Should serialize to JSON value");
    
    // Verify all fields exist
    assert!(json.is_object(), "Response should be an object");
    let obj = json.as_object().unwrap();
    
    assert_eq!(obj.len(), 4, "Should have exactly 4 fields");
    assert!(obj.contains_key("time"), "Should have 'time' field");
    assert!(obj.contains_key("timezone"), "Should have 'timezone' field");
    assert!(obj.contains_key("unix"), "Should have 'unix' field");
    assert!(obj.contains_key("utc"), "Should have 'utc' field");
}