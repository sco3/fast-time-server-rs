// -*- coding: utf-8 -*-
// test_timezone_info_fields.rs - Test all timezones have required fields
//
// Copyright 2025
// SPDX-License-Identifier: Apache-2.0

use fast_time_server::resources::get_timezone_info;

#[test]
fn test_timezone_info_fields() {
    let info = get_timezone_info();
    let timezones = info.get("timezones")
        .and_then(|v| v.as_array())
        .expect("timezones should be an array");
    
    for timezone in timezones {
        // Each timezone must have these fields
        assert!(timezone.get("id").is_some(), "Timezone must have 'id' field");
        assert!(timezone.get("name").is_some(), "Timezone must have 'name' field");
        assert!(timezone.get("offset").is_some(), "Timezone must have 'offset' field");
        assert!(timezone.get("dst").is_some(), "Timezone must have 'dst' field");
        assert!(timezone.get("abbreviation").is_some(), "Timezone must have 'abbreviation' field");
        assert!(timezone.get("major_cities").is_some(), "Timezone must have 'major_cities' field");
        assert!(timezone.get("population").is_some(), "Timezone must have 'population' field");
        
        // Verify field types
        assert!(timezone.get("id").unwrap().is_string(), "id should be string");
        assert!(timezone.get("name").unwrap().is_string(), "name should be string");
        assert!(timezone.get("offset").unwrap().is_string(), "offset should be string");
        assert!(timezone.get("dst").unwrap().is_boolean(), "dst should be boolean");
        assert!(timezone.get("abbreviation").unwrap().is_string(), "abbreviation should be string");
        assert!(timezone.get("major_cities").unwrap().is_array(), "major_cities should be array");
        assert!(timezone.get("population").unwrap().is_number(), "population should be number");
    }
}