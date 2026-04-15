// Test: handle_tool_call with missing required parameters

use fast_time_server::tools::handle_tool_call;
use serde_json::json;

#[test]
fn test_handle_tool_call_missing_params() {
    // Missing time parameter
    let args = json!({
        "source_timezone": "UTC",
        "target_timezone": "America/New_York"
    });
    
    let result = handle_tool_call("convert_time", &args);
    
    assert!(result.is_err(), "Should return error for missing time parameter");
    assert!(result.unwrap_err().contains("time parameter is required"));
}