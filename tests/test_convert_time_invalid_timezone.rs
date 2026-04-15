// Test: convert_time with invalid timezone

use fast_time_server::tools::convert_time;

#[test]
fn test_convert_time_invalid_timezone() {
    let result = convert_time("2025-06-21T16:00:00Z", "Invalid/Source", "America/New_York");

    assert!(
        result.is_err(),
        "Should return error for invalid source timezone"
    );
    assert!(result.unwrap_err().contains("Invalid source timezone"));
}
