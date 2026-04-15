// -*- coding: utf-8 -*-
// test_openapi_has_required_fields.rs - Test OpenAPI spec contains all required fields
//
// Copyright 2025
// SPDX-License-Identifier: Apache-2.0

use fast_time_server::openapi::get_openapi_spec;

#[test]
fn test_openapi_has_required_fields() {
    let spec = get_openapi_spec();
    
    // Check top-level required fields
    assert!(spec.get("openapi").is_some(), "Missing 'openapi' field");
    assert!(spec.get("info").is_some(), "Missing 'info' field");
    assert!(spec.get("servers").is_some(), "Missing 'servers' field");
    assert!(spec.get("paths").is_some(), "Missing 'paths' field");
    
    // Check info object required fields
    let info = spec.get("info").unwrap();
    assert!(info.get("title").is_some(), "Missing 'info.title' field");
    assert!(info.get("description").is_some(), "Missing 'info.description' field");
    assert!(info.get("version").is_some(), "Missing 'info.version' field");
}