// -*- coding: utf-8 -*-
// test_world_times_format.rs - Test world times are in correct format
//
// Copyright 2025
// SPDX-License-Identifier: Apache-2.0

use fast_time_server::resources::get_current_world_times;

#[test]
fn test_world_times_format() {
    let times = get_current_world_times();
    let times_obj = times
        .get("times")
        .and_then(|v| v.as_object())
        .expect("times should be an object");

    // Each time should be in "YYYY-MM-DD HH:MM:SS TZ" format
    for (city, time_value) in times_obj {
        let time_str = time_value.as_str().expect("Time should be a string");

        // Basic format validation: should contain date, time, and timezone
        assert!(
            time_str.contains('-'),
            "{city} time should contain date separator"
        );
        assert!(
            time_str.contains(':'),
            "{city} time should contain time separator"
        );
        assert!(
            time_str.len() > 10,
            "{city} time should have reasonable length"
        );
    }
}
