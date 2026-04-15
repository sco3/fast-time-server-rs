// -*- coding: utf-8 -*-
// test_openapi_info_values.rs - Test OpenAPI info object contains correct values
//
// Copyright 2025
// SPDX-License-Identifier: Apache-2.0

use fast_time_server::openapi::get_openapi_spec;

#[test]
fn test_openapi_info_values() {
    let spec = get_openapi_spec();
    let info = spec.get("info").expect("info field should exist");

    let title = info
        .get("title")
        .and_then(|v| v.as_str())
        .expect("title should be a string");
    assert_eq!(title, "Fast Time Server API");

    let description = info
        .get("description")
        .and_then(|v| v.as_str())
        .expect("description should be a string");
    assert_eq!(description, "REST API for time-related operations");

    let version = info
        .get("version")
        .and_then(|v| v.as_str())
        .expect("version should be a string");
    assert_eq!(version, "1.0.0");
}
