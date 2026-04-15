// -*- coding: utf-8 -*-
// test_business_hours_working_days.rs - Test working days are correctly defined
//
// Copyright 2025
// SPDX-License-Identifier: Apache-2.0

use fast_time_server::resources::get_business_hours;

#[test]
fn test_business_hours_working_days() {
    let hours = get_business_hours();
    let regions = hours
        .get("regions")
        .and_then(|v| v.as_object())
        .expect("regions should be an object");

    let na = regions
        .get("north_america")
        .and_then(|v| v.as_object())
        .expect("north_america should be an object");

    let working_days = na
        .get("working_days")
        .and_then(|v| v.as_array())
        .expect("working_days should be an array");

    // Check expected days
    let expected_days = vec!["Monday", "Tuesday", "Wednesday", "Thursday", "Friday"];
    assert_eq!(
        working_days.len(),
        expected_days.len(),
        "Should have 5 working days"
    );

    for expected_day in expected_days {
        assert!(
            working_days
                .iter()
                .any(|v| v.as_str() == Some(expected_day)),
            "Should contain {}",
            expected_day
        );
    }

    // Verify all entries are strings
    for day in working_days {
        assert!(day.is_string(), "Each working day should be a string");
    }
}
