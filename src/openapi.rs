// -*- coding: utf-8 -*-
// openapi.rs - OpenAPI specification
//
// Copyright 2025
// SPDX-License-Identifier: Apache-2.0

use serde_json::json;

/// Get OpenAPI specification for the REST API
pub fn get_openapi_spec() -> serde_json::Value {
    json!({
        "openapi": "3.0.0",
        "info": {
            "title": "Fast Time Server API",
            "description": "REST API for time-related operations",
            "version": "1.0.0"
        },
        "servers": [{
            "url": "http://localhost:8080",
            "description": "Local development server"
        }],
        "paths": {
            "/api/v1/time": {
                "get": {
                    "summary": "Get current system time",
                    "parameters": [{
                        "name": "timezone",
                        "in": "query",
                        "schema": { "type": "string", "default": "UTC" }
                    }],
                    "responses": {
                        "200": { "description": "Current time information" }
                    }
                }
            }
        }
    })
}