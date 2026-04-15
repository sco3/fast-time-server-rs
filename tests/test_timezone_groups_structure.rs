// -*- coding: utf-8 -*-
// test_timezone_groups_structure.rs - Test timezone groups structure
//
// Copyright 2025
// SPDX-License-Identifier: Apache-2.0

use fast_time_server::resources::get_timezone_info;

#[test]
fn test_timezone_groups_structure() {
    let info = get_timezone_info();
    let groups = info.get("timezone_groups")
        .and_then(|v| v.as_object())
        .expect("timezone_groups should be an object");
    
    // Check US timezones group
    let us_timezones = groups.get("us_timezones")
        .and_then(|v| v.as_array())
        .expect("us_timezones should be an array");
    assert_eq!(us_timezones.len(), 4, "US should have 4 timezones");
    assert!(us_timezones.iter().any(|v| v.as_str() == Some("America/New_York")));
    
    // Check Europe timezones group
    let europe_timezones = groups.get("europe_timezones")
        .and_then(|v| v.as_array())
        .expect("europe_timezones should be an array");
    assert_eq!(europe_timezones.len(), 4, "Europe should have 4 timezones");
    assert!(europe_timezones.iter().any(|v| v.as_str() == Some("Europe/London")));
    
    // Check Asia timezones group
    let asia_timezones = groups.get("asia_timezones")
        .and_then(|v| v.as_array())
        .expect("asia_timezones should be an array");
    assert_eq!(asia_timezones.len(), 4, "Asia should have 4 timezones");
    assert!(asia_timezones.iter().any(|v| v.as_str() == Some("Asia/Tokyo")));
}