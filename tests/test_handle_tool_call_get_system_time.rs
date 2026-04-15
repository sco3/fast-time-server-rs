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
    assert!(value.is_string(), "Result should be a string");
    
    // Test with default UTC
    let args_empty = json!({});
    let result_utc = handle_tool_call("get_system_time", &args_empty);
    
    assert!(result_utc.is_ok(), "Should work with no timezone argument");
}