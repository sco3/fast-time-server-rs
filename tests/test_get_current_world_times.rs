// -*- coding: utf-8 -*-
// test_get_current_world_times.rs - Test get_current_world_times returns valid data
//
// Copyright 2025
// SPDX-License-Identifier: Apache-2.0

use fast_time_server::resources::get_current_world_times;

#[test]
fn test_get_current_world_times() {
    let times = get_current_world_times();

    // Verify it's a valid JSON object
    assert!(times.is_object(), "World times should be a JSON object");

    // Verify required fields exist
    assert!(
        times.get("last_updated").is_some(),
        "Should have 'last_updated' field"
    );
    assert!(times.get("times").is_some(), "Should have 'times' field");

    // Verify last_updated is a string (RFC3339 format)
    let last_updated = times
        .get("last_updated")
        .and_then(|v| v.as_str())
        .expect("last_updated should be a string");
    assert!(!last_updated.is_empty(), "last_updated should not be empty");

    // Verify times object
    let times_obj = times
        .get("times")
        .and_then(|v| v.as_object())
        .expect("times should be an object");

    // Should have multiple cities
    assert!(!times_obj.is_empty(), "times object should not be empty");

    // Check for expected cities
    let expected_cities = vec![
        "New York",
        "Los Angeles",
        "London",
        "Paris",
        "Tokyo",
        "Sydney",
        "Dubai",
    ];
    for city in expected_cities {
        assert!(times_obj.contains_key(city), "Should have time for {city}");
        let time_str = times_obj
            .get(city)
            .and_then(|v| v.as_str())
            .expect("City time should be a string");
        assert!(!time_str.is_empty(), "Time for {city} should not be empty");
    }
}
