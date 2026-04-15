// Test: convert_time with basic conversion

use chrono::DateTime;
use fast_time_server::tools::convert_time;

#[test]
fn test_convert_time_basic() {
    // Convert 16:00 UTC to America/New_York (should be 12:00 or 11:00 depending on DST)
    let result = convert_time("2025-06-21T16:00:00Z", "UTC", "America/New_York");

    assert!(result.is_ok(), "Should successfully convert time");

    let converted = result.unwrap();

    // Verify it's valid RFC3339 format
    let parsed = DateTime::parse_from_rfc3339(&converted);
    assert!(
        parsed.is_ok(),
        "Should be valid RFC3339 format: {}",
        converted
    );

    // Verify it contains timezone offset for New York
    assert!(
        converted.contains("-04:00") || converted.contains("-05:00"),
        "Should have NY timezone offset: {}",
        converted
    );
}
