# Refactoring Summary for Test Coverage Improvement

## Current State

**Overall Coverage: 36.86% (87/236 lines)**

### Coverage Breakdown

| Module | Covered | Total | Coverage | Priority |
|--------|---------|-------|----------|----------|
| main.rs | 0 | 123 | 0% | 🔴 Critical |
| rest_handlers.rs | 0 | 24 | 0% | 🟡 Medium |
| openapi.rs | 0 | 2 | 0% | 🟢 Low |
| tools.rs | 35 | 35 | 100% | ✅ Done |
| resources.rs | 23 | 23 | 100% | ✅ Done |
| prompts.rs | 23 | 23 | 100% | ✅ Done |
| mcp.rs | 6 | 6 | 100% | ✅ Done |

## Quick Wins - Extractable Functions from main.rs

### 1. **parse_log_level** (1 line → 8 lines testable)
- Difficulty: ⭐ Easy
- Impact: 🎯 High
- Estimated Coverage Gain: ~3%

### 2. **format_listen_address** (1 line → Already pure)
- Difficulty: ⭐ Easy  
- Impact: 🎯 High
- Estimated Coverage Gain: ~2%

### 3. **build_health_response** (4 lines testable)
- Difficulty: ⭐⭐ Medium
- Impact: 🎯 High
- Estimated Coverage Gain: ~3%

### 4. **build_version_response** (4 lines testable)
- Difficulty: ⭐ Easy
- Impact: 🎯 High
- Estimated Coverage Gain: ~3%

### 5. **handle_jsonrpc_request** (40+ lines testable)
- Difficulty: ⭐⭐⭐ Complex
- Impact: 🎯🎯🎯 Very High
- Estimated Coverage Gain: ~25%

## Expected Coverage After Refactoring

| Phase | Functions Extracted | Lines Testable | New Coverage | Total Coverage |
|-------|---------------------|----------------|--------------|----------------|
| **Current** | - | 87 | - | 36.86% |
| **Phase 1** (Easy wins) | 4 functions | +15 lines | +6% | ~43% |
| **Phase 2** (JSON-RPC) | 1 function | +40 lines | +17% | ~60% |
| **Phase 3** (REST handlers) | 3-4 functions | +20 lines | +8% | **~68%** |

## Recommended Implementation Order

### Week 1: Quick Wins (Est. 2-3 hours)

1. ✅ Extract `parse_log_level` 
2. ✅ Extract `format_listen_address`
3. ✅ Extract `build_health_response`
4. ✅ Extract `build_version_response`
5. ✅ Write 4 test files (one per function)

**Expected Result: ~43% coverage**

### Week 2: JSON-RPC Refactoring (Est. 4-6 hours)

1. ✅ Extract `handle_jsonrpc_request` logic
2. ✅ Write comprehensive tests:
   - test_jsonrpc_initialize
   - test_jsonrpc_tools_list
   - test_jsonrpc_tools_call
   - test_jsonrpc_unknown_method
   - test_jsonrpc_missing_params

**Expected Result: ~60% coverage**

### Week 3: REST Handlers (Est. 3-4 hours)

1. ✅ Extract pure logic from rest_handlers.rs
2. ✅ Test response building functions
3. ✅ Test request parsing logic

**Expected Result: ~68% coverage**

## Benefits of Refactoring

### 1. **Better Testability**
- Pure functions are easy to test
- No async/await complexity in tests
- No mocking required for basic logic

### 2. **Improved Code Quality**
- Separation of concerns
- Easier to understand and maintain
- Business logic separated from infrastructure

### 3. **Faster Test Execution**
- Pure function tests run instantly
- No need to spin up HTTP servers
- Parallel test execution possible

### 4. **Better Error Messages**
- Detailed assertions on pure functions
- Easier debugging when tests fail

### 5. **Refactoring Safety**
- Tests prevent regressions
- Confidence when making changes
- Documentation through tests

## Implementation Template

For each function extraction, follow this pattern:

```rust
// Step 1: Extract to lib.rs
pub fn extracted_function(params) -> ReturnType {
    // Pure logic here
}

// Step 2: Update main.rs to use it
fn original_function() {
    let result = extracted_function(params);
    // Use result
}

// Step 3: Create test file tests/test_extracted_function.rs
use your_crate::extracted_function;

#[test]
fn test_extracted_function() {
    let result = extracted_function(test_params);
    assert_eq!(result, expected);
}
```

## Files to Create

### New Test Files (7 total)
1. `tests/test_parse_log_level.rs`
2. `tests/test_format_listen_address.rs`
3. `tests/test_build_health_response.rs`
4. `tests/test_build_version_response.rs`
5. `tests/test_handle_jsonrpc_initialize.rs`
6. `tests/test_handle_jsonrpc_tools_list.rs`
7. `tests/test_handle_jsonrpc_tools_call.rs`

### Updated Files (2 total)
1. `src/lib.rs` - Add extracted functions
2. `src/main.rs` - Use extracted functions

## Measuring Success

Run after each phase:
```bash
# Generate coverage
just coverage

# View coverage report
xdg-open coverage/tarpaulin-report.html
```

Look for:
- ✅ Green lines (covered)
- ❌ Red lines (not covered)  
- 📊 Overall percentage increase

## Coverage Goals

| Milestone | Coverage | Status |
|-----------|----------|--------|
| Current | 36.86% | ✅ Baseline |
| Phase 1 Complete | 43% | 🎯 Target |
| Phase 2 Complete | 60% | 🎯 Target |
| Phase 3 Complete | 68% | 🎯 Target |
| Stretch Goal | 75%+ | 🌟 Aspirational |

## Notes

- **Don't test main()**: The main function and run_*_mode functions are integration points, not business logic
- **Focus on pure functions**: Extract and test logic, not infrastructure
- **One test per file**: Follow existing convention
- **Keep tests simple**: Test one thing at a time
- **Use descriptive names**: Test names should explain what they test

## Next Steps

1. Review REFACTORING_GUIDE.md for detailed code examples
2. Start with Phase 1 (quick wins)
3. Run `just coverage` after each function extraction
4. Track progress and adjust as needed

---

**Remember**: The goal is not 100% coverage, but 70-80% coverage of testable business logic. Infrastructure code (async handlers, server setup) doesn't need unit tests - that's what integration tests are for!