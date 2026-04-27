// Test: convert_time with basic conversion

use chrono::DateTime;
use fast_time_server::tools::convert_time;

#[test]
fn test_convert_time_basic() {
    let input_str = "2025-06-21T16:00:00Z";
    let result = convert_time(input_str, "UTC", "America/New_York");

    assert!(result.is_ok(), "Should successfully convert time");
    let converted = result.unwrap();

    // Parse both
    let original_dt = DateTime::parse_from_rfc3339(input_str).unwrap();
    let converted_dt = DateTime::parse_from_rfc3339(&converted).expect("Output not RFC3339");

    // Print for visibility if it fails
    println!(
        "Original:  {} (TS: {})",
        original_dt,
        original_dt.timestamp()
    );
    println!(
        "Converted: {} (TS: {})",
        converted_dt,
        converted_dt.timestamp()
    );

    assert_eq!(
        original_dt.timestamp(),
        converted_dt.timestamp(),
        "The absolute moment in time changed! Check if convert_time is double-applying offsets."
    );
}
