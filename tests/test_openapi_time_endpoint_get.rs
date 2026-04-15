// -*- coding: utf-8 -*-
// test_openapi_time_endpoint_get.rs - Test /api/v1/time GET endpoint definition
//
// Copyright 2025
// SPDX-License-Identifier: Apache-2.0

use fast_time_server::openapi::get_openapi_spec;

#[test]
fn test_openapi_time_endpoint_get() {
    let spec = get_openapi_spec();
    let paths = spec.get("paths")
        .and_then(|v| v.as_object())
        .expect("paths should be an object");
    
    let time_path = paths.get("/api/v1/time")
        .and_then(|v| v.as_object())
        .expect("/api/v1/time should be an object");
    
    assert!(time_path.contains_key("get"), "Should have GET method");
    
    let get_method = time_path.get("get")
        .and_then(|v| v.as_object())
        .expect("get should be an object");
    
    assert!(get_method.contains_key("summary"), "GET should have summary");
    assert!(get_method.contains_key("parameters"), "GET should have parameters");
    assert!(get_method.contains_key("responses"), "GET should have responses");
}