// -*- coding: utf-8 -*-
// test_openapi_paths_exist.rs - Test OpenAPI paths are defined
//
// Copyright 2025
// SPDX-License-Identifier: Apache-2.0

use fast_time_server::openapi::get_openapi_spec;

#[test]
fn test_openapi_paths_exist() {
    let spec = get_openapi_spec();
    let paths = spec
        .get("paths")
        .and_then(|v| v.as_object())
        .expect("paths should be an object");

    assert!(
        paths.contains_key("/api/v1/time"),
        "Should contain /api/v1/time path"
    );
}
