// -*- coding: utf-8 -*-
// test_time_query_optional_field.rs - Test TimeQuery timezone field is optional
//
// Copyright 2025
// SPDX-License-Identifier: Apache-2.0

#[derive(serde::Deserialize, Debug)]
struct TimeQuery {
    timezone: Option<String>,
}

#[test]
fn test_time_query_optional_field() {
    // Test with null timezone
    let json_with_null = r#"{"timezone":null}"#;
    let query: TimeQuery = serde_json::from_str(json_with_null)
        .expect("Should deserialize with null timezone");
    
    assert!(query.timezone.is_none(), "Null timezone should be None");
    
    // Test with empty object
    let json_empty = r#"{}"#;
    let query: TimeQuery = serde_json::from_str(json_empty)
        .expect("Should deserialize empty object");
    
    assert!(query.timezone.is_none(), "Missing timezone should be None");
    
    // Test with timezone present
    let json_with_value = r#"{"timezone":"UTC"}"#;
    let query: TimeQuery = serde_json::from_str(json_with_value)
        .expect("Should deserialize with timezone");
    
    assert!(query.timezone.is_some(), "Provided timezone should be Some");
    assert_eq!(query.timezone.unwrap(), "UTC");
}