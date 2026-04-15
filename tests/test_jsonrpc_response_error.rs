// Test: JsonRpcResponse::error creates proper error response

use fast_time_server::mcp::JsonRpcResponse;
use serde_json::json;

#[test]
fn test_jsonrpc_response_error() {
    let id = Some(json!(2));
    let code = -32601;
    let message = "Method not found".to_string();
    
    let response = JsonRpcResponse::error(id.clone(), code, message.clone());
    
    // Verify jsonrpc version
    assert_eq!(response.jsonrpc, "2.0", "Should have jsonrpc version 2.0");
    
    // Verify result is None
    assert!(response.result.is_none(), "Result should be None for error");
    
    // Verify error is present
    assert!(response.error.is_some(), "Error should be present");
    
    let error = response.error.unwrap();
    assert_eq!(error.code, code, "Error code should match");
    assert_eq!(error.message, message, "Error message should match");
    
    // Verify id is preserved
    assert_eq!(response.id, id, "ID should be preserved");
}