# Refactoring Guide for Improved Test Coverage

## Current Coverage Analysis

### Overall Coverage: 36.86% (87/236 lines)

| File | Covered | Total | Coverage | Status |
|------|---------|-------|----------|--------|
| **src/main.rs** | 0 | 123 | 0% | ❌ Needs Refactoring |
| **src/rest_handlers.rs** | 0 | 24 | 0% | ❌ Needs Refactoring |
| **src/openapi.rs** | 0 | 2 | 0% | ⚠️ Low Priority |
| **src/tools.rs** | 35 | 35 | 100% | ✅ Excellent |
| **src/resources.rs** | 23 | 23 | 100% | ✅ Excellent |
| **src/prompts.rs** | 23 | 23 | 100% | ✅ Excellent |
| **src/mcp.rs** | 6 | 6 | 100% | ✅ Excellent |

## Refactoring Opportunities in main.rs

### 1. Extract Log Level Parsing (Easy - High Impact)

**Current Code:**
```rust
fn init_logging(level: &str) {
    let filter = match level.to_lowercase().as_str() {
        "debug" => "debug",
        "info" => "info",
        "warn" => "warn",
        "error" => "error",
        _ => "info",
    };
    tracing_subscriber::fmt().with_env_filter(filter).with_target(false).init();
}
```

**Refactored Code:**
```rust
// In src/lib.rs or new src/config.rs
pub fn parse_log_level(level: &str) -> &'static str {
    match level.to_lowercase().as_str() {
        "debug" => "debug",
        "info" => "info",
        "warn" | "warning" => "warn",
        "error" => "error",
        "none" | "off" => "off",
        _ => "info",
    }
}

// In src/main.rs
fn init_logging(level: &str) {
    let filter = parse_log_level(level);
    tracing_subscriber::fmt().with_env_filter(filter).with_target(false).init();
}
```

**Test:**
```rust
// tests/test_parse_log_level.rs
use fast_time_server::config::parse_log_level;

#[test]
fn test_parse_log_level() {
    assert_eq!(parse_log_level("debug"), "debug");
    assert_eq!(parse_log_level("DEBUG"), "debug");
    assert_eq!(parse_log_level("info"), "info");
    assert_eq!(parse_log_level("warn"), "warn");
    assert_eq!(parse_log_level("warning"), "warn");
    assert_eq!(parse_log_level("error"), "error");
    assert_eq!(parse_log_level("invalid"), "info"); // default
}
```

### 2. Extract Address Formatting (Easy - High Impact)

**Current Code:**
```rust
fn get_listen_address(args: &Args) -> String {
    args.addr.clone().unwrap_or_else(|| format!("{}:{}", args.listen, args.port))
}
```

**Refactored Code:**
```rust
// In src/lib.rs
pub fn format_listen_address(addr: Option<&str>, host: &str, port: u16) -> String {
    addr.map(String::from)
        .unwrap_or_else(|| format!("{}:{}", host, port))
}

// In src/main.rs
fn get_listen_address(args: &Args) -> String {
    format_listen_address(args.addr.as_deref(), &args.listen, args.port)
}
```

**Test:**
```rust
// tests/test_format_listen_address.rs
use fast_time_server::format_listen_address;

#[test]
fn test_format_listen_address() {
    // With explicit address
    assert_eq!(
        format_listen_address(Some("127.0.0.1:9090"), "0.0.0.0", 8080),
        "127.0.0.1:9090"
    );
    
    // Without explicit address - uses host:port
    assert_eq!(
        format_listen_address(None, "0.0.0.0", 8080),
        "0.0.0.0:8080"
    );
    
    assert_eq!(
        format_listen_address(None, "localhost", 3000),
        "localhost:3000"
    );
}
```

### 3. Extract JSON Response Builders (Medium - High Impact)

**Current Code:**
```rust
async fn handle_health(State(state): State<AppState>) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "uptime_seconds": state.start_time.elapsed().as_secs()
    }))
}

async fn handle_version() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "name": APP_NAME,
        "version": APP_VERSION,
        "mcp_version": "1.0"
    }))
}
```

**Refactored Code:**
```rust
// In src/lib.rs
pub fn build_health_response(uptime_secs: u64) -> serde_json::Value {
    serde_json::json!({
        "status": "healthy",
        "uptime_seconds": uptime_secs
    })
}

pub fn build_version_response(name: &str, version: &str) -> serde_json::Value {
    serde_json::json!({
        "name": name,
        "version": version,
        "mcp_version": "1.0"
    })
}

// In src/main.rs
async fn handle_health(State(state): State<AppState>) -> Json<serde_json::Value> {
    Json(build_health_response(state.start_time.elapsed().as_secs()))
}

async fn handle_version() -> Json<serde_json::Value> {
    Json(build_version_response(APP_NAME, APP_VERSION))
}
```

**Tests:**
```rust
// tests/test_build_health_response.rs
use fast_time_server::build_health_response;

#[test]
fn test_build_health_response() {
    let response = build_health_response(42);
    
    assert_eq!(response.get("status").unwrap().as_str().unwrap(), "healthy");
    assert_eq!(response.get("uptime_seconds").unwrap().as_u64().unwrap(), 42);
}

// tests/test_build_version_response.rs
use fast_time_server::build_version_response;

#[test]
fn test_build_version_response() {
    let response = build_version_response("test-server", "1.0.0");
    
    assert_eq!(response.get("name").unwrap().as_str().unwrap(), "test-server");
    assert_eq!(response.get("version").unwrap().as_str().unwrap(), "1.0.0");
    assert_eq!(response.get("mcp_version").unwrap().as_str().unwrap(), "1.0");
}
```

### 4. Extract JSON-RPC Request Parsing (Complex - Very High Impact)

**Current Code:**
```rust
async fn handle_jsonrpc(
    State(_state): State<AppState>,
    Json(request): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let method = request.get("method").and_then(|v| v.as_str()).unwrap_or("");
    let id = request.get("id").cloned();
    
    let response = match method {
        "initialize" => JsonRpcResponse::success(id, serde_json::json!({
            "protocolVersion": "1.0",
            "capabilities": {"tools": {}, "resources": {}, "prompts": {}},
            "serverInfo": {"name": APP_NAME, "version": APP_VERSION}
        })),
        "tools/list" => JsonRpcResponse::success(id, serde_json::json!({
            "tools": [
                {"name": "get_system_time", "description": "Get current system time"},
                {"name": "convert_time", "description": "Convert time between timezones"}
            ]
        })),
        "tools/call" => {
            let params = request.get("params").ok_or(StatusCode::BAD_REQUEST)?;
            let tool_name = params.get("name").and_then(|v| v.as_str()).ok_or(StatusCode::BAD_REQUEST)?;
            let default_args = serde_json::json!({});
            let arguments = params.get("arguments").unwrap_or(&default_args);
            
            match tools::handle_tool_call(tool_name, arguments) {
                Ok(result) => JsonRpcResponse::success(id, result),
                Err(e) => JsonRpcResponse::error(id, -32000, e),
            }
        },
        _ => JsonRpcResponse::error(id, -32601, format!("Method not found: {}", method)),
    };
    
    Ok(Json(serde_json::json!(response)))
}
```

**Refactored Code:**
```rust
// In src/lib.rs or src/jsonrpc.rs
pub fn handle_jsonrpc_request(
    request: &serde_json::Value,
    app_name: &str,
    app_version: &str,
) -> Result<JsonRpcResponse, String> {
    let method = request.get("method")
        .and_then(|v| v.as_str())
        .ok_or("Missing method")?;
    let id = request.get("id").cloned();
    
    match method {
        "initialize" => Ok(JsonRpcResponse::success(id, serde_json::json!({
            "protocolVersion": "1.0",
            "capabilities": {"tools": {}, "resources": {}, "prompts": {}},
            "serverInfo": {"name": app_name, "version": app_version}
        }))),
        
        "tools/list" => Ok(JsonRpcResponse::success(id, serde_json::json!({
            "tools": [
                {"name": "get_system_time", "description": "Get current system time"},
                {"name": "convert_time", "description": "Convert time between timezones"}
            ]
        }))),
        
        "tools/call" => {
            let params = request.get("params").ok_or("Missing params")?;
            let tool_name = params.get("name")
                .and_then(|v| v.as_str())
                .ok_or("Missing tool name")?;
            let default_args = serde_json::json!({});
            let arguments = params.get("arguments").unwrap_or(&default_args);
            
            match tools::handle_tool_call(tool_name, arguments) {
                Ok(result) => Ok(JsonRpcResponse::success(id, result)),
                Err(e) => Ok(JsonRpcResponse::error(id, -32000, e)),
            }
        },
        
        _ => Ok(JsonRpcResponse::error(id, -32601, format!("Method not found: {}", method))),
    }
}

// In src/main.rs
async fn handle_jsonrpc(
    State(_state): State<AppState>,
    Json(request): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    match handle_jsonrpc_request(&request, APP_NAME, APP_VERSION) {
        Ok(response) => Ok(Json(serde_json::json!(response))),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}
```

**Tests:**
```rust
// tests/test_handle_jsonrpc_initialize.rs
use fast_time_server::handle_jsonrpc_request;
use serde_json::json;

#[test]
fn test_handle_jsonrpc_initialize() {
    let request = json!({
        "jsonrpc": "2.0",
        "method": "initialize",
        "id": 1,
        "params": {}
    });
    
    let response = handle_jsonrpc_request(&request, "test-server", "1.0.0").unwrap();
    
    assert!(response.result.is_some());
    assert!(response.error.is_none());
    
    let result = response.result.unwrap();
    assert_eq!(result.get("protocolVersion").unwrap().as_str().unwrap(), "1.0");
}

// tests/test_handle_jsonrpc_tools_list.rs
use fast_time_server::handle_jsonrpc_request;
use serde_json::json;

#[test]
fn test_handle_jsonrpc_tools_list() {
    let request = json!({
        "jsonrpc": "2.0",
        "method": "tools/list",
        "id": 2
    });
    
    let response = handle_jsonrpc_request(&request, "test-server", "1.0.0").unwrap();
    
    assert!(response.result.is_some());
    let result = response.result.unwrap();
    let tools = result.get("tools").unwrap().as_array().unwrap();
    assert_eq!(tools.len(), 2);
}

// tests/test_handle_jsonrpc_unknown_method.rs
use fast_time_server::handle_jsonrpc