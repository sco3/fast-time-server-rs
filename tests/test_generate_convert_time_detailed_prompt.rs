// Test: generate_convert_time_detailed_prompt creates proper prompt

use fast_time_server::prompts::generate_convert_time_detailed_prompt;

#[test]
fn test_generate_convert_time_detailed_prompt() {
    let time = "2025-01-15T12:00:00Z";
    let from_tz = "UTC";
    let to_tzs = "America/New_York,Europe/London";

    // Test without context
    let prompt = generate_convert_time_detailed_prompt(time, from_tz, to_tzs, false);

    assert!(
        prompt.contains("2025-01-15T12:00:00Z"),
        "Prompt should contain the time"
    );
    assert!(prompt.contains("UTC"), "Prompt should mention UTC");
    assert!(
        prompt.contains("America/New_York"),
        "Prompt should mention America/New_York"
    );
    assert!(
        prompt.contains("Europe/London"),
        "Prompt should mention Europe/London"
    );
    assert!(
        !prompt.contains("Day of week"),
        "Should not include context without flag"
    );

    // Test with context
    let prompt_with_context = generate_convert_time_detailed_prompt(time, from_tz, to_tzs, true);

    assert!(
        prompt_with_context.contains("Day of week"),
        "Should include day of week with context"
    );
    assert!(
        prompt_with_context.contains("business day"),
        "Should mention business day"
    );
    assert!(
        prompt_with_context.contains("Time until/since"),
        "Should mention time until/since"
    );
}
