// Test: generate_compare_timezones_prompt creates proper prompt

use fast_time_server::prompts::generate_compare_timezones_prompt;

#[test]
fn test_generate_compare_timezones_prompt() {
    let timezones = "UTC,America/New_York,Asia/Tokyo";
    let prompt = generate_compare_timezones_prompt(timezones, None);

    // Verify prompt contains the timezones
    assert!(prompt.contains("UTC"), "Prompt should mention UTC");
    assert!(
        prompt.contains("America/New_York"),
        "Prompt should mention America/New_York"
    );
    assert!(
        prompt.contains("Asia/Tokyo"),
        "Prompt should mention Asia/Tokyo"
    );

    // Verify prompt contains expected instructions
    assert!(
        prompt.contains("Compare"),
        "Prompt should contain 'Compare'"
    );
    assert!(
        prompt.contains("business hours"),
        "Prompt should mention business hours"
    );
    assert!(
        prompt.contains("day of the week"),
        "Prompt should mention day of the week"
    );

    // Test with reference time
    let prompt_with_ref =
        generate_compare_timezones_prompt(timezones, Some("2025-01-15T12:00:00Z"));
    assert!(
        prompt_with_ref.contains("Reference time"),
        "Should include reference time"
    );
    assert!(
        prompt_with_ref.contains("2025-01-15T12:00:00Z"),
        "Should contain the reference time"
    );
}
