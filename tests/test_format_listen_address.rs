// Test: format_listen_address correctly formats listen addresses

use fast_time_server::format_listen_address;

#[test]
fn test_format_listen_address() {
    // Test with explicit address - should use it directly
    assert_eq!(
        format_listen_address(Some("127.0.0.1:9090"), "0.0.0.0", 8080),
        "127.0.0.1:9090"
    );
    
    assert_eq!(
        format_listen_address(Some("localhost:3000"), "ignored", 9999),
        "localhost:3000"
    );
    
    // Test without explicit address - should construct from host and port
    assert_eq!(
        format_listen_address(None, "0.0.0.0", 8080),
        "0.0.0.0:8080"
    );
    
    assert_eq!(
        format_listen_address(None, "localhost", 3000),
        "localhost:3000"
    );
    
    assert_eq!(
        format_listen_address(None, "127.0.0.1", 9090),
        "127.0.0.1:9090"
    );
    
    // Test with IPv6
    assert_eq!(
        format_listen_address(None, "::1", 8080),
        "::1:8080"
    );
    
    // Test with different ports
    assert_eq!(
        format_listen_address(None, "0.0.0.0", 80),
        "0.0.0.0:80"
    );
    
    assert_eq!(
        format_listen_address(None, "0.0.0.0", 443),
        "0.0.0.0:443"
    );
}