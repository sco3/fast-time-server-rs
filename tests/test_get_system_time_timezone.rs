// Test: get_system_time with specific timezone

use chrono::DateTime;
use fast_time_server::tools::get_system_time;

#[test]
fn test_get_system_time_timezone() {
    let result = get_system_time("America/New_York");

    assert!(
        result.is_ok(),
        "Should successfully get America/New_York time"
    );

    let time_str = result.unwrap();

    // Verify it's valid RFC3339 format
    let parsed = DateTime::parse_from_rfc3339(&time_str);
    assert!(
        parsed.is_ok(),
        "Should be valid RFC3339 format: {}",
        time_str
    );
}
