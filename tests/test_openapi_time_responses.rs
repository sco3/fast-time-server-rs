// -*- coding: utf-8 -*-
// test_openapi_time_responses.rs - Test /api/v1/time endpoint responses
//
// Copyright 2025
// SPDX-License-Identifier: Apache-2.0

use fast_time_server::openapi::get_openapi_spec;

#[test]
fn test_openapi_time_responses() {
    let spec = get_openapi_spec();
    let paths = spec
        .get("paths")
        .and_then(|v| v.as_object())
        .expect("paths should be an object");

    let time_path = paths
        .get("/api/v1/time")
        .and_then(|v| v.as_object())
        .expect("/api/v1/time should be an object");

    let get_method = time_path
        .get("get")
        .and_then(|v| v.as_object())
        .expect("get should be an object");

    let responses = get_method
        .get("responses")
        .and_then(|v| v.as_object())
        .expect("responses should be an object");

    assert!(responses.contains_key("200"), "Should have 200 response");

    let success_response = responses
        .get("200")
        .and_then(|v| v.as_object())
        .expect("200 response should be an object");

    let description = success_response
        .get("description")
        .and_then(|v| v.as_str())
        .expect("response description should be a string");
    assert_eq!(description, "Current time information");
}
