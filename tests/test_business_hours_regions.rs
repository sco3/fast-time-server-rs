// -*- coding: utf-8 -*-
// test_business_hours_regions.rs - Test business hours regions have correct structure
//
// Copyright 2025
// SPDX-License-Identifier: Apache-2.0

use fast_time_server::resources::get_business_hours;

#[test]
fn test_business_hours_regions() {
    let hours = get_business_hours();
    let regions = hours.get("regions")
        .and_then(|v| v.as_object())
        .expect("regions should be an object");
    
    // Test each region has required fields
    for (region_name, region_data) in regions {
        let region_obj = region_data.as_object()
            .unwrap_or_else(|| panic!("{} should be an object", region_name));
        
        assert!(region_obj.contains_key("standard_hours"), 
            "{} should have standard_hours", region_name);
        assert!(region_obj.contains_key("lunch_break"), 
            "{} should have lunch_break", region_name);
        assert!(region_obj.contains_key("working_days"), 
            "{} should have working_days", region_name);
        
        // Verify types
        assert!(region_obj.get("standard_hours").unwrap().is_string(), 
            "{} standard_hours should be string", region_name);
        assert!(region_obj.get("lunch_break").unwrap().is_string(), 
            "{} lunch_break should be string", region_name);
        assert!(region_obj.get("working_days").unwrap().is_array(), 
            "{} working_days should be array", region_name);
    }
}