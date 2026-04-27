// Test: handle_tool_call for get_system_time tool

use fast_time_server::tools::handle_tool_call;
use serde_json::json;
#[test]
fn test_handle_tool_call_get_system_time() {
    // Test with timezone argument
    let args = json!({"timezone": "Europe/London"});
    let result = handle_tool_call("get_system_time", &args);

    assert!(result.is_ok(), "Should successfully call get_system_time");

    let value = result.unwrap();

    // Extract the text content from the MCP structure
    let time_str = value["content"][0]["text"]
        .as_str()
        .expect("Result should contain a text string in content[0].text");

    // Verify it looks like a timestamp (contains 'T')
    assert!(time_str.contains('T'), "Output should be an ISO 8601/RFC 3339 string");

    // Test with default UTC
    let args_empty = json!({});
    let result_utc = handle_tool_call("get_system_time", &args_empty);

    assert!(result_utc.is_ok(), "Should work with no timezone argument");

    let value_utc = result_utc.unwrap();
    let time_utc_str = value_utc["content"][0]["text"]
        .as_str()
        .unwrap();

    // UTC results usually end in Z or +00:00
    assert!(time_utc_str.ends_with('Z') || time_utc_str.contains("+00:00"));
}