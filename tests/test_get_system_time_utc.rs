// Test: get_system_time with UTC timezone

use chrono::DateTime;
use fast_time_server::tools::get_system_time;

#[test]
fn test_get_system_time_utc() {
    let result = get_system_time("UTC");

    assert!(result.is_ok(), "Should successfully get UTC time");

    let time_str = result.unwrap();

    // Verify it's valid RFC3339 format
    let parsed = DateTime::parse_from_rfc3339(&time_str);
    assert!(
        parsed.is_ok(),
        "Should be valid RFC3339 format: {time_str}"
    );

    // Verify it contains UTC timezone indicator
    assert!(
        time_str.ends_with('Z') || time_str.contains("+00:00"),
        "UTC time should end with Z or +00:00: {time_str}"
    );
}
