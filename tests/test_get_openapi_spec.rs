// Test: get_openapi_spec returns valid OpenAPI specification

use fast_time_server::openapi::get_openapi_spec;

#[test]
fn test_get_openapi_spec() {
    let spec = get_openapi_spec();
    
    // Verify it's a valid JSON object
    assert!(spec.is_object(), "OpenAPI spec should be a JSON object");
    
    // Verify top-level OpenAPI fields exist
    assert!(spec.get("openapi").is_some(), "Should have 'openapi' field");
    assert!(spec.get("info").is_some(), "Should have 'info' field");
    assert!(spec.get("servers").is_some(), "Should have 'servers' field");
    assert!(spec.get("paths").is_some(), "Should have 'paths' field");
    
    // Verify OpenAPI version
    assert_eq!(
        spec.get("openapi").unwrap().as_str().unwrap(),
        "3.0.0",
        "OpenAPI version should be 3.0.0"
    );
    
    // Verify info section
    let info = spec.get("info").unwrap();
    assert_eq!(
        info.get("title").unwrap().as_str().unwrap(),
        "Fast Time Server API",
        "API title should match"
    );
    assert_eq!(
        info.get("version").unwrap().as_str().unwrap(),
        "1.0.0",
        "API version should be 1.0.0"
    );
    assert!(
        info.get("description").is_some(),
        "Should have description"
    );
    
    // Verify servers array exists and has at least one server
    let servers = spec.get("servers").unwrap();
    assert!(servers.is_array(), "Servers should be an array");
    let servers_array = servers.as_array().unwrap();
    assert!(!servers_array.is_empty(), "Should have at least one server");
    
    // Verify first server has URL
    let first_server = &servers_array[0];
    assert!(
        first_server.get("url").is_some(),
        "Server should have URL"
    );
    
    // Verify paths object exists
    let paths = spec.get("paths").unwrap();
    assert!(paths.is_object(), "Paths should be an object");
    
    // Verify at least one path exists
    let paths_obj = paths.as_object().unwrap();
    assert!(!paths_obj.is_empty(), "Should have at least one path defined");
    
    // Verify /api/v1/time endpoint exists
    assert!(
        paths_obj.contains_key("/api/v1/time"),
        "Should have /api/v1/time endpoint"
    );
}