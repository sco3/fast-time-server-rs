// Test: handle_tool_call for convert_time tool

use fast_time_server::tools::handle_tool_call;
use serde_json::json;

#[test]
fn test_handle_tool_call_convert_time() {
    let args = json!({
        "time": "2025-06-21T16:00:00Z",
        "source_timezone": "UTC",
        "target_timezone": "America/New_York"
    });

    let result = handle_tool_call("convert_time", &args);

    assert!(result.is_ok(), "Should successfully call convert_time");

    let value = result.unwrap();
    print!("value {value}");
    // 1. Verify the 'content' array exists and isn't empty
    let content = value.get("content").and_then(|c| c.as_array())
        .expect("Result should have a 'content' array");

    // 2. Extract the text from the first content block
    let time_text = content[0].get("text").and_then(|t| t.as_str())
        .expect("First content block should have a 'text' string");

    // 3. Now perform your assertions on the actual time string
    assert!(time_text.contains("2025-06-21T12:00:00"), "Time should be 12:00 NY time");
    assert!(time_text.contains("-04:00"), "Should have the correct New York summer offset");
}
