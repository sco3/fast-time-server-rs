// -*- coding: utf-8 -*-
// test_time_query_deserialization.rs - Test TimeQuery can be deserialized
//
// Copyright 2025
// SPDX-License-Identifier: Apache-2.0

#[derive(serde::Deserialize, Debug)]
struct TimeQuery {
    timezone: Option<String>,
}

#[test]
fn test_time_query_deserialization() {
    // Test with timezone provided
    let json_with_tz = r#"{"timezone":"America/New_York"}"#;
    let query: TimeQuery =
        serde_json::from_str(json_with_tz).expect("Should deserialize with timezone");

    assert!(query.timezone.is_some(), "Timezone should be present");
    assert_eq!(query.timezone.unwrap(), "America/New_York");

    // Test without timezone (should be None)
    let json_without_tz = r#"{}"#;
    let query: TimeQuery =
        serde_json::from_str(json_without_tz).expect("Should deserialize without timezone");

    assert!(query.timezone.is_none(), "Timezone should be None");
}
