// Test: build_version_response creates correct JSON structure

use fast_time_server::build_version_response;

#[test]
fn test_build_version_response() {
    // Test with various name and version combinations
    let response = build_version_response("test-server", "1.0.0");
    assert_eq!(
        response.get("name").unwrap().as_str().unwrap(),
        "test-server"
    );
    assert_eq!(response.get("version").unwrap().as_str().unwrap(), "1.0.0");
    assert_eq!(
        response.get("mcp_version").unwrap().as_str().unwrap(),
        "1.0"
    );

    let response = build_version_response("fast-time-server", "1.5.0");
    assert_eq!(
        response.get("name").unwrap().as_str().unwrap(),
        "fast-time-server"
    );
    assert_eq!(response.get("version").unwrap().as_str().unwrap(), "1.5.0");
    assert_eq!(
        response.get("mcp_version").unwrap().as_str().unwrap(),
        "1.0"
    );

    // Test with different naming styles
    let response = build_version_response("MyServer", "2.3.4-beta");
    assert_eq!(response.get("name").unwrap().as_str().unwrap(), "MyServer");
    assert_eq!(
        response.get("version").unwrap().as_str().unwrap(),
        "2.3.4-beta"
    );
    assert_eq!(
        response.get("mcp_version").unwrap().as_str().unwrap(),
        "1.0"
    );

    // Verify JSON structure has exactly 3 fields
    let response = build_version_response("test", "1.0");
    let response_obj = response.as_object().unwrap();
    assert_eq!(response_obj.len(), 3);
    assert!(response_obj.contains_key("name"));
    assert!(response_obj.contains_key("version"));
    assert!(response_obj.contains_key("mcp_version"));

    // Verify MCP version is always "1.0"
    let response = build_version_response("any-name", "any-version");
    assert_eq!(
        response.get("mcp_version").unwrap().as_str().unwrap(),
        "1.0"
    );
}
