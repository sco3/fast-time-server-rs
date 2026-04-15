// Test: JsonRpcResponse::success creates proper response

use fast_time_server::mcp::JsonRpcResponse;
use serde_json::json;

#[test]
fn test_jsonrpc_response_success() {
    let id = Some(json!(1));
    let result = json!({"time": "2025-01-15T12:00:00Z"});

    let response = JsonRpcResponse::success(id.clone(), result.clone());

    // Verify jsonrpc version
    assert_eq!(response.jsonrpc, "2.0", "Should have jsonrpc version 2.0");

    // Verify result is present
    assert!(response.result.is_some(), "Should have result field");
    assert_eq!(
        response.result.unwrap(),
        result,
        "Result should match input"
    );

    // Verify error is None
    assert!(response.error.is_none(), "Error should be None for success");

    // Verify id is preserved
    assert_eq!(response.id, id, "ID should be preserved");
}
