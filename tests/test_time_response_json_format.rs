// -*- coding: utf-8 -*-
// test_time_response_json_format.rs - Test TimeResponse JSON format is correct
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
fn test_time_response_json_format() {
    let response = TimeResponse {
        time: "2026-04-15T12:30:45+01:00".to_string(),
        timezone: "Europe/London".to_string(),
        unix: 1776024645,
        utc: "2026-04-15T11:30:45Z".to_string(),
    };

    let json = serde_json::to_string_pretty(&response).expect("Should serialize to pretty JSON");

    // Verify JSON contains expected structure
    assert!(
        json.contains("\"time\""),
        "JSON should contain 'time' field"
    );
    assert!(
        json.contains("\"timezone\""),
        "JSON should contain 'timezone' field"
    );
    assert!(
        json.contains("\"unix\""),
        "JSON should contain 'unix' field"
    );
    assert!(json.contains("\"utc\""), "JSON should contain 'utc' field");

    // Verify JSON contains expected values
    assert!(
        json.contains("Europe/London"),
        "JSON should contain timezone value"
    );
    assert!(
        json.contains("1776024645"),
        "JSON should contain unix timestamp"
    );
}
