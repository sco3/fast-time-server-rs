// -*- coding: utf-8 -*-
// test_openapi_endpoint.rs - Test OpenAPI REST endpoint
//
// Copyright 2025
// SPDX-License-Identifier: Apache-2.0

#[test]
fn test_openapi_endpoint() {
    // This test verifies that the OpenAPI spec function returns valid data
    // Integration testing with actual HTTP requests would require starting a server

    let spec = fast_time_server::openapi::get_openapi_spec();

    // Verify it's a valid JSON object
    assert!(spec.is_object(), "OpenAPI spec should be a JSON object");

    // Verify required OpenAPI fields
    assert_eq!(
        spec.get("openapi").and_then(|v| v.as_str()).unwrap(),
        "3.0.0"
    );

    // Verify info section
    let info = spec.get("info").expect("Should have info field");
    assert_eq!(
        info.get("title").and_then(|v| v.as_str()).unwrap(),
        "Fast Time Server API"
    );

    // Verify paths exist
    let paths = spec
        .get("paths")
        .and_then(|v| v.as_object())
        .expect("Should have paths");
    assert!(
        paths.contains_key("/api/v1/time"),
        "Should have /api/v1/time endpoint"
    );

    // Verify the time endpoint has GET method
    let time_path = paths
        .get("/api/v1/time")
        .and_then(|v| v.as_object())
        .unwrap();
    assert!(
        time_path.contains_key("get"),
        "Time endpoint should have GET method"
    );
}
