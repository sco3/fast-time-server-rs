// -*- coding: utf-8 -*-
// test_get_business_hours.rs - Test get_business_hours returns valid data
//
// Copyright 2025
// SPDX-License-Identifier: Apache-2.0

use fast_time_server::resources::get_business_hours;

#[test]
fn test_get_business_hours() {
    let hours = get_business_hours();
    
    // Verify it's a valid JSON object
    assert!(hours.is_object(), "Business hours should be a JSON object");
    
    // Verify required fields exist
    assert!(hours.get("regions").is_some(), "Should have 'regions' field");
    
    // Verify regions object
    let regions = hours.get("regions")
        .and_then(|v| v.as_object())
        .expect("regions should be an object");
    
    // Should have multiple regions
    assert!(!regions.is_empty(), "regions object should not be empty");
    
    // Check for expected regions
    assert!(regions.contains_key("north_america"), "Should have north_america region");
    assert!(regions.contains_key("europe"), "Should have europe region");
    
    // Verify north_america structure
    let na = regions.get("north_america")
        .and_then(|v| v.as_object())
        .expect("north_america should be an object");
    
    assert!(na.contains_key("standard_hours"), "north_america should have standard_hours");
    assert!(na.contains_key("lunch_break"), "north_america should have lunch_break");
    assert!(na.contains_key("working_days"), "north_america should have working_days");
    
    // Verify working_days is an array
    let working_days = na.get("working_days")
        .and_then(|v| v.as_array())
        .expect("working_days should be an array");
    assert_eq!(working_days.len(), 5, "Should have 5 working days");
}