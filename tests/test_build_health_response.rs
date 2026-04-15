// Test: build_health_response creates correct JSON structure

use fast_time_server::build_health_response;

#[test]
fn test_build_health_response() {
    // Test with various uptime values
    let response = build_health_response(0);
    assert_eq!(response.get("status").unwrap().as_str().unwrap(), "healthy");
    assert_eq!(response.get("uptime_seconds").unwrap().as_u64().unwrap(), 0);
    
    let response = build_health_response(42);
    assert_eq!(response.get("status").unwrap().as_str().unwrap(), "healthy");
    assert_eq!(response.get("uptime_seconds").unwrap().as_u64().unwrap(), 42);
    
    let response = build_health_response(3600); // 1 hour
    assert_eq!(response.get("status").unwrap().as_str().unwrap(), "healthy");
    assert_eq!(response.get("uptime_seconds").unwrap().as_u64().unwrap(), 3600);
    
    let response = build_health_response(86400); // 1 day
    assert_eq!(response.get("status").unwrap().as_str().unwrap(), "healthy");
    assert_eq!(response.get("uptime_seconds").unwrap().as_u64().unwrap(), 86400);
    
    // Verify JSON structure has exactly 2 fields
    let response = build_health_response(100);
    let response_obj = response.as_object().unwrap();
    assert_eq!(response_obj.len(), 2);
    assert!(response_obj.contains_key("status"));
    assert!(response_obj.contains_key("uptime_seconds"));
}