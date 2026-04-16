// -*- coding: utf-8 -*-
// test_time_formats_structure.rs - Test time formats structure is correct
//
// Copyright 2025
// SPDX-License-Identifier: Apache-2.0

use fast_time_server::resources::get_time_formats;

#[test]
fn test_time_formats_structure() {
    let formats = get_time_formats();

    let input_formats = formats
        .get("input_formats")
        .and_then(|v| v.as_array())
        .expect("input_formats should be an array");

    // All input formats should be strings
    for format in input_formats {
        assert!(format.is_string(), "Each input format should be a string");
    }

    let output_formats = formats
        .get("output_formats")
        .and_then(|v| v.as_object())
        .expect("output_formats should be an object");

    // All output formats should be strings
    for (key, value) in output_formats {
        assert!(
            value.is_string(),
            "Output format '{key}' should be a string"
        );
        assert!(
            !value.as_str().unwrap().is_empty(),
            "Output format '{key}' should not be empty"
        );
    }
}
