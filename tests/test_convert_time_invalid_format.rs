// Test: convert_time with invalid time format

use fast_time_server::tools::convert_time;

#[test]
fn test_convert_time_invalid_format() {
    let result = convert_time(
        "not a valid time",
        "UTC",
        "America/New_York"
    );
    
    assert!(result.is_err(), "Should return error for invalid time format");
    assert!(result.unwrap_err().contains("Invalid time format"));
}