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
    assert!(value.is_string(), "Result should be a string");
}