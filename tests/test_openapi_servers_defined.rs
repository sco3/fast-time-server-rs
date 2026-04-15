// -*- coding: utf-8 -*-
// test_openapi_servers_defined.rs - Test OpenAPI servers are properly defined
//
// Copyright 2025
// SPDX-License-Identifier: Apache-2.0

use fast_time_server::openapi::get_openapi_spec;

#[test]
fn test_openapi_servers_defined() {
    let spec = get_openapi_spec();
    let servers = spec
        .get("servers")
        .and_then(|v| v.as_array())
        .expect("servers should be an array");

    assert!(!servers.is_empty(), "servers array should not be empty");

    let first_server = &servers[0];
    let url = first_server
        .get("url")
        .and_then(|v| v.as_str())
        .expect("server url should be a string");
    assert_eq!(url, "http://localhost:8080");

    let description = first_server
        .get("description")
        .and_then(|v| v.as_str())
        .expect("server description should be a string");
    assert_eq!(description, "Local development server");
}
