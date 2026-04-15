// Test: parse_log_level correctly maps log level strings

use fast_time_server::parse_log_level;

#[test]
fn test_parse_log_level() {
    // Test all supported levels
    assert_eq!(parse_log_level("debug"), "debug");
    assert_eq!(parse_log_level("info"), "info");
    assert_eq!(parse_log_level("warn"), "warn");
    assert_eq!(parse_log_level("error"), "error");
    assert_eq!(parse_log_level("off"), "off");
    assert_eq!(parse_log_level("none"), "off");

    // Test case insensitivity
    assert_eq!(parse_log_level("DEBUG"), "debug");
    assert_eq!(parse_log_level("INFO"), "info");
    assert_eq!(parse_log_level("WARN"), "warn");
    assert_eq!(parse_log_level("ERROR"), "error");

    // Test warning alias
    assert_eq!(parse_log_level("warning"), "warn");
    assert_eq!(parse_log_level("WARNING"), "warn");

    // Test default fallback for invalid input
    assert_eq!(parse_log_level("invalid"), "info");
    assert_eq!(parse_log_level("random"), "info");
    assert_eq!(parse_log_level(""), "info");
    assert_eq!(parse_log_level("trace"), "info"); // not supported, falls back
}
