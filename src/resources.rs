// -*- coding: utf-8 -*-
// resources.rs - MCP resource implementations
//
// Copyright 2025
// SPDX-License-Identifier: Apache-2.0

use crate::AppState;
use serde_json::json;

/// Get timezone information resource
#[must_use]
pub fn get_timezone_info() -> serde_json::Value {
    json!({
        "timezones": [
            {
                "id": "America/New_York",
                "name": "Eastern Time",
                "offset": "-05:00",
                "dst": true,
                "abbreviation": "EST/EDT",
                "major_cities": ["New York", "Toronto", "Montreal"],
                "population": 141_000_000
            },
            {
                "id": "Europe/London",
                "name": "Greenwich Mean Time",
                "offset": "+00:00",
                "dst": true,
                "abbreviation": "GMT/BST",
                "major_cities": ["London", "Dublin", "Lisbon"],
                "population": 67_000_000
            },
            {
                "id": "Asia/Tokyo",
                "name": "Japan Standard Time",
                "offset": "+09:00",
                "dst": false,
                "abbreviation": "JST",
                "major_cities": ["Tokyo", "Osaka", "Yokohama"],
                "population": 127_000_000
            }
        ],
        "timezone_groups": {
            "us_timezones": ["America/New_York", "America/Chicago", "America/Denver", "America/Los_Angeles"],
            "europe_timezones": ["Europe/London", "Europe/Paris", "Europe/Berlin", "Europe/Moscow"],
            "asia_timezones": ["Asia/Tokyo", "Asia/Shanghai", "Asia/Singapore", "Asia/Dubai"]
        }
    })
}

/// Get current world times resource (non-cached version)
#[must_use]
pub fn get_current_world_times() -> serde_json::Value {
    use chrono::Utc;
    use chrono_tz::Tz;

    let cities = vec![
        ("New York", "America/New_York"),
        ("Los Angeles", "America/Los_Angeles"),
        ("London", "Europe/London"),
        ("Paris", "Europe/Paris"),
        ("Tokyo", "Asia/Tokyo"),
        ("Sydney", "Australia/Sydney"),
        ("Dubai", "Asia/Dubai"),
    ];

    let now = Utc::now();
    let mut times = serde_json::Map::new();

    for (city, tz_str) in cities {
        if let Ok(tz) = tz_str.parse::<Tz>() {
            let local_time = now.with_timezone(&tz);
            times.insert(
                city.to_string(),
                json!(local_time.format("%Y-%m-%d %H:%M:%S %Z").to_string()),
            );
        }
    }

    json!({
        "last_updated": now.to_rfc3339(),
        "times": times
    })
}

/// Get current world times resource (cached version)
pub async fn get_current_world_times_cached(state: &AppState) -> serde_json::Value {
    use chrono::Utc;

    let cities = vec![
        ("New York", "America/New_York"),
        ("Los Angeles", "America/Los_Angeles"),
        ("London", "Europe/London"),
        ("Paris", "Europe/Paris"),
        ("Tokyo", "Asia/Tokyo"),
        ("Sydney", "Australia/Sydney"),
        ("Dubai", "Asia/Dubai"),
    ];

    let now = Utc::now();
    let mut times = serde_json::Map::new();

    for (city, tz_str) in cities {
        if let Ok(tz) = state.load_timezone(tz_str).await {
            let local_time = now.with_timezone(&tz);
            times.insert(
                city.to_string(),
                json!(local_time.format("%Y-%m-%d %H:%M:%S %Z").to_string()),
            );
        }
    }

    json!({
        "last_updated": now.to_rfc3339(),
        "times": times
    })
}

/// Get time formats resource
#[must_use]
pub fn get_time_formats() -> serde_json::Value {
    json!({
        "input_formats": [
            "2006-01-02 15:04:05",
            "2006-01-02T15:04:05Z",
            "2006-01-02T15:04:05-07:00",
            "Jan 2, 2006 3:04 PM"
        ],
        "output_formats": {
            "iso8601": "2006-01-02T15:04:05Z07:00",
            "rfc3339": "2006-01-02T15:04:05Z",
            "rfc822": "Mon, 02 Jan 2006 15:04:05 MST"
        }
    })
}

/// Get business hours resource
#[must_use]
pub fn get_business_hours() -> serde_json::Value {
    json!({
        "regions": {
            "north_america": {
                "standard_hours": "9:00 AM - 5:00 PM",
                "lunch_break": "12:00 PM - 1:00 PM",
                "working_days": ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday"]
            },
            "europe": {
                "standard_hours": "9:00 AM - 6:00 PM",
                "lunch_break": "1:00 PM - 2:00 PM",
                "working_days": ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday"]
            }
        }
    })
}
