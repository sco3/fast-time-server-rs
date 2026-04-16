// -*- coding: utf-8 -*-
// prompts.rs - MCP prompt implementations
//
// Copyright 2025
// SPDX-License-Identifier: Apache-2.0

use std::fmt::Write;

/// Generate compare timezones prompt
#[must_use]
pub fn generate_compare_timezones_prompt(timezones: &str, reference_time: Option<&str>) -> String {
    let mut prompt = format!("Compare the current time across these time zones: {timezones}\n");
    if let Some(ref_time) = reference_time {
        let _ = writeln!(prompt, "Reference time: {ref_time}");
    }
    prompt.push_str("\nShow:\n");
    prompt.push_str("1. The current time in each timezone\n");
    prompt.push_str("2. The time difference from the first timezone\n");
    prompt.push_str("3. Whether it's business hours (9 AM - 5 PM)\n");
    prompt.push_str("4. The day of the week\n");
    prompt
}

/// Generate schedule meeting prompt
#[must_use]
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
        "Find the best meeting time for participants in: {participants}\n\n\
        Meeting details:\n\
        - Duration: {duration} minutes\n\
        - Preferred hours: {preferred_hours} local time\n\
        - Date range: {date_range}\n"
    )
}

/// Generate convert time detailed prompt
#[must_use]
pub fn generate_convert_time_detailed_prompt(
    time: &str,
    from_tz: &str,
    to_tzs: &str,
    include_context: bool,
) -> String {
    let mut prompt = format!("Convert {time} from {from_tz} to: {to_tzs}\n");

    if include_context {
        prompt.push_str("\nAlso provide:\n");
        prompt.push_str("1. Day of week in each timezone\n");
        prompt.push_str("2. Whether it's a business day\n");
        prompt.push_str("3. Time until/since this moment\n");
    }

    prompt
}
