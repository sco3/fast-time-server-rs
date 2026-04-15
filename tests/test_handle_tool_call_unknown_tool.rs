// Test: handle_tool_call with unknown tool name

use fast_time_server::tools::handle_tool_call;
use serde_json::json;

#[test]
fn test_handle_tool_call_unknown_tool() {
    let args = json!({});
    let result = handle_tool_call("unknown_tool", &args);
    
    assert!(result.is_err(), "Should return error for unknown tool");
    assert!(result.unwrap_err().contains("Unknown tool"));
}