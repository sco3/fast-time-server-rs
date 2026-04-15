// Test: generate_schedule_meeting_prompt creates proper prompt

use fast_time_server::prompts::generate_schedule_meeting_prompt;

#[test]
fn test_generate_schedule_meeting_prompt() {
    let participants = "New York,London,Tokyo";
    let prompt = generate_schedule_meeting_prompt(participants, None, None, None);

    // Verify prompt contains participants
    assert!(
        prompt.contains("New York"),
        "Prompt should mention New York"
    );
    assert!(prompt.contains("London"), "Prompt should mention London");
    assert!(prompt.contains("Tokyo"), "Prompt should mention Tokyo");

    // Verify prompt contains default values
    assert!(
        prompt.contains("60 minutes"),
        "Should have default duration"
    );
    assert!(
        prompt.contains("9 AM - 5 PM"),
        "Should have default preferred hours"
    );
    assert!(
        prompt.contains("next 7 days"),
        "Should have default date range"
    );

    // Test with custom values
    let custom_prompt = generate_schedule_meeting_prompt(
        participants,
        Some("90"),
        Some("10 AM - 4 PM"),
        Some("next 3 days"),
    );
    assert!(
        custom_prompt.contains("90 minutes"),
        "Should use custom duration"
    );
    assert!(
        custom_prompt.contains("10 AM - 4 PM"),
        "Should use custom preferred hours"
    );
    assert!(
        custom_prompt.contains("next 3 days"),
        "Should use custom date range"
    );
}
