// -*- coding: utf-8 -*-
// test_get_time_formats.rs - Test get_time_formats returns valid format data
//
// Copyright 2025
// SPDX-License-Identifier: Apache-2.0

use fast_time_server::resources::get_time_formats;

#[test]
fn test_get_time_formats() {
    let formats = get_time_formats();

    // Verify it's a valid JSON object
    assert!(formats.is_object(), "Time formats should be a JSON object");

    // Verify required fields exist
    assert!(
        formats.get("input_formats").is_some(),
        "Should have 'input_formats' field"
    );
    assert!(
        formats.get("output_formats").is_some(),
        "Should have 'output_formats' field"
    );

    // Verify input_formats is an array
    let input_formats = formats
        .get("input_formats")
        .and_then(|v| v.as_array())
        .expect("input_formats should be an array");

    assert_eq!(input_formats.len(), 4, "Should have 4 input formats");

    // Verify some expected formats exist
    let format_strings: Vec<&str> = input_formats.iter().filter_map(|v| v.as_str()).collect();

    assert!(
        format_strings.contains(&"2006-01-02 15:04:05"),
        "Should contain basic format"
    );
    assert!(
        format_strings.contains(&"2006-01-02T15:04:05Z"),
        "Should contain ISO format"
    );

    // Verify output_formats is an object
    let output_formats = formats
        .get("output_formats")
        .and_then(|v| v.as_object())
        .expect("output_formats should be an object");

    assert!(
        output_formats.contains_key("iso8601"),
        "Should have iso8601 format"
    );
    assert!(
        output_formats.contains_key("rfc3339"),
        "Should have rfc3339 format"
    );
    assert!(
        output_formats.contains_key("rfc822"),
        "Should have rfc822 format"
    );
}
