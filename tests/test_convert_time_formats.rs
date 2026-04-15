// Test: convert_time with different input formats

use chrono::DateTime;
use fast_time_server::tools::convert_time;

#[test]
fn test_convert_time_formats() {
    // Test format: YYYY-MM-DD HH:MM:SS
    let result1 = convert_time("2025-06-21 16:00:00", "UTC", "Europe/London");
    assert!(result1.is_ok(), "Should handle format: YYYY-MM-DD HH:MM:SS");

    // Test format: YYYY-MM-DDTHH:MM:SS
    let result2 = convert_time("2025-06-21T16:00:00", "UTC", "Europe/London");
    assert!(result2.is_ok(), "Should handle format: YYYY-MM-DDTHH:MM:SS");

    // Test format: YYYY-MM-DD (date only)
    let result3 = convert_time("2025-06-21", "UTC", "Europe/London");
    assert!(result3.is_ok(), "Should handle format: YYYY-MM-DD");

    // Verify all produce valid RFC3339
    for result in [result1, result2, result3] {
        let time_str = result.unwrap();
        let parsed = DateTime::parse_from_rfc3339(&time_str);
        assert!(parsed.is_ok(), "Should produce valid RFC3339: {}", time_str);
    }
}
