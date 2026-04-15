// -*- coding: utf-8 -*-
// test_resource_endpoints.rs - Test resource REST endpoints
//
// Copyright 2025
// SPDX-License-Identifier: Apache-2.0

#[test]
fn test_resource_endpoints() {
    // Test timezone-info resource
    let timezone_info = fast_time_server::resources::get_timezone_info();
    assert!(
        timezone_info.is_object(),
        "Timezone info should be a JSON object"
    );
    assert!(
        timezone_info.get("timezones").is_some(),
        "Should have timezones field"
    );
    assert!(
        timezone_info.get("timezone_groups").is_some(),
        "Should have timezone_groups field"
    );

    // Test current-world resource
    let world_times = fast_time_server::resources::get_current_world_times();
    assert!(
        world_times.is_object(),
        "World times should be a JSON object"
    );
    assert!(
        world_times.get("last_updated").is_some(),
        "Should have last_updated field"
    );
    assert!(
        world_times.get("times").is_some(),
        "Should have times field"
    );

    // Test time-formats resource
    let time_formats = fast_time_server::resources::get_time_formats();
    assert!(
        time_formats.is_object(),
        "Time formats should be a JSON object"
    );
    assert!(
        time_formats.get("input_formats").is_some(),
        "Should have input_formats field"
    );
    assert!(
        time_formats.get("output_formats").is_some(),
        "Should have output_formats field"
    );

    // Test business-hours resource
    let business_hours = fast_time_server::resources::get_business_hours();
    assert!(
        business_hours.is_object(),
        "Business hours should be a JSON object"
    );
    assert!(
        business_hours.get("regions").is_some(),
        "Should have regions field"
    );
}
