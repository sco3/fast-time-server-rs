// -*- coding: utf-8 -*-
// test_openapi_time_parameters.rs - Test /api/v1/time endpoint parameters
//
// Copyright 2025
// SPDX-License-Identifier: Apache-2.0

use fast_time_server::openapi::get_openapi_spec;

#[test]
fn test_openapi_time_parameters() {
    let spec = get_openapi_spec();
    let paths = spec.get("paths")
        .and_then(|v| v.as_object())
        .expect("paths should be an object");
    
    let time_path = paths.get("/api/v1/time")
        .and_then(|v| v.as_object())
        .expect("/api/v1/time should be an object");
    
    let get_method = time_path.get("get")
        .and_then(|v| v.as_object())
        .expect("get should be an object");
    
    let parameters = get_method.get("parameters")
        .and_then(|v| v.as_array())
        .expect("parameters should be an array");
    
    assert!(!parameters.is_empty(), "Should have at least one parameter");
    
    let timezone_param = &parameters[0];
    let name = timezone_param.get("name")
        .and_then(|v| v.as_str())
        .expect("parameter name should be a string");
    assert_eq!(name, "timezone");
    
    let in_location = timezone_param.get("in")
        .and_then(|v| v.as_str())
        .expect("parameter 'in' should be a string");
    assert_eq!(in_location, "query");
    
    let schema = timezone_param.get("schema")
        .and_then(|v| v.as_object())
        .expect("parameter schema should be an object");
    
    let param_type = schema.get("type")
        .and_then(|v| v.as_str())
        .expect("schema type should be a string");
    assert_eq!(param_type, "string");
    
    let default = schema.get("default")
        .and_then(|v| v.as_str())
        .expect("schema default should be a string");
    assert_eq!(default, "UTC");
}