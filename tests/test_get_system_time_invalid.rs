// Test: get_system_time with invalid timezone

use fast_time_server::tools::get_system_time;

#[test]
fn test_get_system_time_invalid() {
    let result = get_system_time("Invalid/Timezone");
    
    assert!(result.is_err(), "Should return error for invalid timezone");
    
    let error = result.unwrap_err();
    assert!(error.contains("Invalid timezone"), 
        "Error should mention invalid timezone: {}", error);
}