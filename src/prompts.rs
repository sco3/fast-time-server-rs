// -*- coding: utf-8 -*-
// prompts.rs - MCP prompt implementations
//
// Copyright 2025
// SPDX-License-Identifier: Apache-2.0

/// Generate compare timezones prompt
pub fn generate_compare_timezones_prompt(timezones: &str, reference_time: Option<&str>) -> String {
    let mut prompt = format!("Compare the current time across these time zones: {}\n", timezones);
    if let Some(ref_time) = reference_time {
        prompt.push_str(&format!("Reference time: {}\n", ref_time));
    }
    prompt.push_str("\nShow:\n");
    prompt.push_str("1. The current time in each timezone\n");
    prompt.push_str("2. The time difference from the first timezone\n");
    prompt.push_str("3. Whether it's business hours (9 AM - 5 PM)\n");
    prompt.push_str("4. The day of the week\n");
    prompt
}

/// Generate schedule meeting prompt
pub fn generate_schedule_meeting_prompt(
    participants: &str,
    duration: Option<&str>,
    preferred_hours: Option<&str>,
    date_range: Option<&str>,
) -> String {
    let duration = duration.unwrap_or("60");
    let preferred_hours = preferred_hours.unwrap_or("9 AM - 5 PM");
    let date_range = date_range.unwrap_or("next 7 days");
    
    format!(
        "Find the best meeting time for participants in: {}\n\n\
        Meeting details:\n\
        - Duration: {} minutes\n\
        - Preferred hours: {} local time\n\
        - Date range: {}\n",
        participants, duration, preferred_hours, date_range
    )
}

/// Generate convert time detailed prompt
pub fn generate_convert_time_detailed_prompt(
    time: &str,
    from_tz: &str,
    to_tzs: &str,
    include_context: bool,
) -> String {
    let mut prompt = format!("Convert {} from {} to: {}\n", time, from_tz, to_tzs);
    
    if include_context {
        prompt.push_str("\nAlso provide:\n");
        prompt.push_str("1. Day of week in each timezone\n");
        prompt.push_str("2. Whether it's a business day\n");
        prompt.push_str("3. Time until/since this moment\n");
    }
    
    prompt
}