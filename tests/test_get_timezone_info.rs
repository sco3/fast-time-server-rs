// -*- coding: utf-8 -*-
// test_get_timezone_info.rs - Test get_timezone_info returns valid timezone data
//
// Copyright 2025
// SPDX-License-Identifier: Apache-2.0

use fast_time_server::resources::get_timezone_info;

#[test]
fn test_get_timezone_info() {
    let info = get_timezone_info();
    
    // Verify it's a valid JSON object
    assert!(info.is_object(), "Timezone info should be a JSON object");
    
    // Verify top-level fields exist
    assert!(info.get("timezones").is_some(), "Should have 'timezones' field");
    assert!(info.get("timezone_groups").is_some(), "Should have 'timezone_groups' field");
    
    // Verify timezones array
    let timezones = info.get("timezones")
        .and_then(|v| v.as_array())
        .expect("timezones should be an array");
    
    assert_eq!(timezones.len(), 3, "Should have 3 timezones");
    
    // Verify first timezone (America/New_York)
    let ny = &timezones[0];
    assert_eq!(ny.get("id").and_then(|v| v.as_str()).unwrap(), "America/New_York");
    assert_eq!(ny.get("name").and_then(|v| v.as_str()).unwrap(), "Eastern Time");
    assert_eq!(ny.get("offset").and_then(|v| v.as_str()).unwrap(), "-05:00");
    assert!(ny.get("dst").and_then(|v| v.as_bool()).unwrap());
    assert_eq!(ny.get("abbreviation").and_then(|v| v.as_str()).unwrap(), "EST/EDT");
    assert_eq!(ny.get("population").and_then(|v| v.as_i64()).unwrap(), 141000000);
    
    // Verify timezone_groups
    let groups = info.get("timezone_groups")
        .and_then(|v| v.as_object())
        .expect("timezone_groups should be an object");
    
    assert!(groups.contains_key("us_timezones"), "Should have us_timezones group");
    assert!(groups.contains_key("europe_timezones"), "Should have europe_timezones group");
    assert!(groups.contains_key("asia_timezones"), "Should have asia_timezones group");
}