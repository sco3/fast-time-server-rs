// -*- coding: utf-8 -*-
// test_openapi_version_format.rs - Test OpenAPI version is correctly formatted
//
// Copyright 2025
// SPDX-License-Identifier: Apache-2.0

use fast_time_server::openapi::get_openapi_spec;

#[test]
fn test_openapi_version_format() {
    let spec = get_openapi_spec();
    
    let openapi_version = spec.get("openapi")
        .and_then(|v| v.as_str())
        .expect("openapi field should be a string");
    
    assert_eq!(openapi_version, "3.0.0", "OpenAPI version should be 3.0.0");
}