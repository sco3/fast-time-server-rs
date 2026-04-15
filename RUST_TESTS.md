# Rust Unit Tests for fast-time-server

## Overview

This document describes the unit tests for the Rust implementation of fast-time-server. All tests follow a strict naming convention:
- One test function per file
- File name matches function name
- Both start with `test_` prefix
- Helper functions are allowed within test files

## Test Directory Structure

```
tests/
├── test_get_system_time_utc.rs
├── test_get_system_time_timezone.rs
├── test_get_system_time_invalid.rs
├── test_convert_time_basic.rs
├── test_convert_time_formats.rs
├── test_convert_time_invalid_timezone.rs
├── test_convert_time_invalid_format.rs
├── test_handle_tool_call_get_system_time.rs
├── test_handle_tool_call_convert_time.rs
├── test_handle_tool_call_missing_params.rs
├── test_handle_tool_call_unknown_tool.rs
├── test_get_timezone_info.rs
├── test_get_current_world_times.rs
├── test_get_time_formats.rs
├── test_get_business_hours.rs
├── test_generate_compare_timezones_prompt.rs
├── test_generate_schedule_meeting_prompt.rs
├── test_generate_convert_time_detailed_prompt.rs
├── test_jsonrpc_response_success.rs
└── test_jsonrpc_response_error.rs
```

## Test Categories

### Tools Module Tests (src/tools.rs)

#### 1. `test_get_system_time_utc`
Tests getting current system time in UTC timezone.
- Verifies successful execution
- Validates RFC3339 format
- Checks UTC timezone indicator

#### 2. `test_get_system_time_timezone`
Tests getting current system time in a specific timezone (America/New_York).
- Verifies successful execution with non-UTC timezone
- Validates RFC3339 format

#### 3. `test_get_system_time_invalid`
Tests error handling for invalid timezone.
- Verifies error is returned
- Checks error message contains "Invalid timezone"

#### 4. `test_convert_time_basic`
Tests basic time conversion between timezones.
- Converts UTC to America/New_York
- Validates RFC3339 format
- Checks timezone offset is present

#### 5. `test_convert_time_formats`
Tests multiple input time formats.
- Tests: `YYYY-MM-DD HH:MM:SS`
- Tests: `YYYY-MM-DDTHH:MM:SS`
- Tests: `YYYY-MM-DD` (date only)
- Verifies all produce valid RFC3339 output

#### 6. `test_convert_time_invalid_timezone`
Tests error handling for invalid source timezone.
- Verifies error is returned
- Checks error message mentions invalid timezone

#### 7. `test_convert_time_invalid_format`
Tests error handling for invalid time format.
- Verifies error is returned
- Checks error message mentions invalid format

#### 8. `test_handle_tool_call_get_system_time`
Tests MCP tool handler for get_system_time.
- Tests with timezone argument
- Tests with default UTC (no argument)
- Verifies JSON response structure

#### 9. `test_handle_tool_call_convert_time`
Tests MCP tool handler for convert_time.
- Provides all required parameters
- Verifies successful execution
- Checks JSON response

#### 10. `test_handle_tool_call_missing_params`
Tests error handling for missing required parameters.
- Omits required 'time' parameter
- Verifies error is returned
- Checks error message

#### 11. `test_handle_tool_call_unknown_tool`
Tests error handling for unknown tool names.
- Calls non-existent tool
- Verifies error is returned
- Checks "Unknown tool" message

### Resources Module Tests (src/resources.rs)

#### 12. `test_get_timezone_info`
Tests timezone information resource.
- Verifies JSON object structure
- Checks for 'timezones' array
- Checks for 'timezone_groups' object

#### 13. `test_get_current_world_times`
Tests current world times resource.
- Verifies JSON object structure
- Checks 'last_updated' field
- Checks 'times' object with city data

#### 14. `test_get_time_formats`
Tests time formats resource.
- Verifies JSON object structure
- Checks 'input_formats' array
- Checks 'output_formats' object

#### 15. `test_get_business_hours`
Tests business hours resource.
- Verifies JSON object structure
- Checks 'regions' field
- Verifies presence of expected regions

### Prompts Module Tests (src/prompts.rs)

#### 16. `test_generate_compare_timezones_prompt`
Tests timezone comparison prompt generation.
- Verifies all timezones are mentioned
- Checks for expected instructions
- Tests with and without reference time

#### 17. `test_generate_schedule_meeting_prompt`
Tests meeting scheduler prompt generation.
- Verifies participants are mentioned
- Checks default values (duration, hours, range)
- Tests with custom values

#### 18. `test_generate_convert_time_detailed_prompt`
Tests detailed time conversion prompt.
- Verifies time and timezones are mentioned
- Tests without context flag
- Tests with context flag (extra details)

### MCP Protocol Tests (src/mcp.rs)

#### 19. `test_jsonrpc_response_success`
Tests successful JSON-RPC response creation.
- Verifies jsonrpc version "2.0"
- Checks result field is present
- Verifies error is None
- Confirms ID is preserved

#### 20. `test_jsonrpc_response_error`
Tests error JSON-RPC response creation.
- Verifies jsonrpc version "2.0"
- Checks result is None
- Verifies error object structure
- Confirms error code and message
- Confirms ID is preserved

## Running Tests

### Run All Tests
```bash
cargo test
```

### Run Specific Test
```bash
cargo test test_get_system_time_utc
```

### Run Tests with Output
```bash
cargo test -- --nocapture
```

### Run Tests in Specific Module
```bash
cargo test --test test_get_system_time_utc
```

## Test Coverage

Total test files: **20**

### Coverage by Module:
- **tools.rs**: 11 tests (55%)
- **resources.rs**: 4 tests (20%)
- **prompts.rs**: 3 tests (15%)
- **mcp.rs**: 2 tests (10%)

### Test Types:
- **Happy Path**: 12 tests (60%)
- **Error Handling**: 5 tests (25%)
- **Format Validation**: 3 tests (15%)

## Writing New Tests

To add a new test:

1. Create a new file in `tests/` directory
2. Name it `test_<functionality>.rs`
3. Create a test function with the same name: `test_<functionality>`
4. Follow the pattern:

```rust
// Test: Description of what this tests

use fast_time_server::module::function;

#[test]
fn test_functionality() {
    // Arrange
    let input = setup_test_data();
    
    // Act
    let result = function(input);
    
    // Assert
    assert!(result.is_ok(), "Should succeed");
    // More assertions...
}

// Helper functions (if needed)
fn setup_test_data() -> TestData {
    // Helper implementation
}
```

## CI/CD Integration

These tests can be integrated into CI/CD pipelines:

```yaml
# GitHub Actions example
- name: Run Rust tests
  run: cargo test --verbose
```

## Dependencies

Tests use standard Rust testing framework with these dependencies:
- `chrono` - For time/date handling
- `chrono-tz` - For timezone support
- `serde_json` - For JSON handling

No additional test dependencies required.

## Best Practices

1. **One Test Per File**: Each test file contains exactly one test function
2. **Descriptive Names**: Test names clearly describe what is being tested
3. **Clear Assertions**: Each assertion includes a descriptive message
4. **Independent Tests**: Tests don't depend on each other
5. **Fast Execution**: Tests run quickly without external dependencies
6. **Edge Cases**: Tests cover both happy paths and error scenarios