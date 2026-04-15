#!/usr/bin/env bash
# -*- coding: utf-8 -*-
# test_openapi_integration.sh - Integration test for OpenAPI endpoints
#
# Copyright 2025
# SPDX-License-Identifier: Apache-2.0
#
# Tests OpenAPI endpoints on both Go and Rust servers

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
GO_PORT=8080
RUST_PORT=8081
WAIT_TIME=3

echo "========================================"
echo "OpenAPI Endpoints Integration Test"
echo "========================================"
echo ""

# Function to wait for server to be ready
wait_for_server() {
    local port=$1
    local server_name=$2
    local max_attempts=10
    local attempt=0
    
    echo "Waiting for $server_name to be ready on port $port..."
    while [ $attempt -lt $max_attempts ]; do
        if curl -s http://localhost:$port/health > /dev/null 2>&1; then
            echo -e "${GREEN}✓${NC} $server_name is ready"
            return 0
        fi
        attempt=$((attempt + 1))
        echo "  Attempt $attempt/$max_attempts..."
        sleep 1
    done
    
    echo -e "${RED}✗${NC} $server_name failed to start"
    return 1
}

# Function to test OpenAPI endpoint
test_openapi_endpoint() {
    local port=$1
    local server_name=$2
    
    echo ""
    echo "Testing $server_name OpenAPI endpoint..."
    
    # Test /api/v1/openapi.json
    echo "  Testing GET /api/v1/openapi.json..."
    local response=$(curl -s -w "\n%{http_code}" http://localhost:$port/api/v1/openapi.json)
    local body=$(echo "$response" | head -n -1)
    local status=$(echo "$response" | tail -n 1)
    
    if [ "$status" != "200" ]; then
        echo -e "${RED}✗${NC} Failed: HTTP $status"
        return 1
    fi
    
    # Validate JSON structure
    if ! echo "$body" | jq -e '.openapi' > /dev/null 2>&1; then
        echo -e "${RED}✗${NC} Failed: Missing 'openapi' field"
        return 1
    fi
    
    local openapi_version=$(echo "$body" | jq -r '.openapi')
    if [ "$openapi_version" != "3.0.0" ]; then
        echo -e "${RED}✗${NC} Failed: Expected OpenAPI 3.0.0, got $openapi_version"
        return 1
    fi
    
    if ! echo "$body" | jq -e '.info.title' > /dev/null 2>&1; then
        echo -e "${RED}✗${NC} Failed: Missing 'info.title' field"
        return 1
    fi
    
    local title=$(echo "$body" | jq -r '.info.title')
    if [ "$title" != "Fast Time Server API" ]; then
        echo -e "${RED}✗${NC} Failed: Expected title 'Fast Time Server API', got '$title'"
        return 1
    fi
    
    if ! echo "$body" | jq -e '.paths."/api/v1/time"' > /dev/null 2>&1; then
        echo -e "${RED}✗${NC} Failed: Missing '/api/v1/time' path"
        return 1
    fi
    
    echo -e "${GREEN}✓${NC} OpenAPI spec endpoint works correctly"
    
    # Test /api/v1/docs
    echo "  Testing GET /api/v1/docs..."
    local docs_response=$(curl -s -w "\n%{http_code}" http://localhost:$port/api/v1/docs)
    local docs_body=$(echo "$docs_response" | head -n -1)
    local docs_status=$(echo "$docs_response" | tail -n 1)
    
    if [ "$docs_status" != "200" ]; then
        echo -e "${RED}✗${NC} Failed: HTTP $docs_status"
        return 1
    fi
    
    if ! echo "$docs_body" | grep -q "swagger-ui"; then
        echo -e "${RED}✗${NC} Failed: Docs page doesn't contain Swagger UI"
        return 1
    fi
    
    if ! echo "$docs_body" | grep -q "/api/v1/openapi.json"; then
        echo -e "${RED}✗${NC} Failed: Docs page doesn't reference openapi.json"
        return 1
    fi
    
    echo -e "${GREEN}✓${NC} API docs endpoint works correctly"
    
    return 0
}

# Test Go server
echo ""
echo "========== Testing Go Server =========="
echo "Building Go server..."
go build -o fast-time-server-go main.go openapi.go rest_handlers.go 2>&1 | grep -v "^#" || true

echo "Starting Go server on port $GO_PORT..."
./fast-time-server-go --transport rest --port $GO_PORT > /tmp/go-server.log 2>&1 &
GO_PID=$!
echo "Go server PID: $GO_PID"

if wait_for_server $GO_PORT "Go server"; then
    if test_openapi_endpoint $GO_PORT "Go"; then
        echo -e "${GREEN}✓ Go server tests passed${NC}"
        GO_RESULT=0
    else
        echo -e "${RED}✗ Go server tests failed${NC}"
        GO_RESULT=1
    fi
else
    echo -e "${RED}✗ Go server failed to start${NC}"
    cat /tmp/go-server.log
    GO_RESULT=1
fi

echo "Stopping Go server..."
kill $GO_PID 2>/dev/null || true
wait $GO_PID 2>/dev/null || true

# Test Rust server
echo ""
echo "========== Testing Rust Server =========="
echo "Building Rust server..."
cargo build --release --quiet

echo "Starting Rust server on port $RUST_PORT..."
cargo run --release -- --transport rest --port $RUST_PORT > /tmp/rust-server.log 2>&1 &
RUST_PID=$!
echo "Rust server PID: $RUST_PID"

if wait_for_server $RUST_PORT "Rust server"; then
    if test_openapi_endpoint $RUST_PORT "Rust"; then
        echo -e "${GREEN}✓ Rust server tests passed${NC}"
        RUST_RESULT=0
    else
        echo -e "${RED}✗ Rust server tests failed${NC}"
        RUST_RESULT=1
    fi
else
    echo -e "${RED}✗ Rust server failed to start${NC}"
    cat /tmp/rust-server.log
    RUST_RESULT=1
fi

echo "Stopping Rust server..."
kill $RUST_PID 2>/dev/null || true
wait $RUST_PID 2>/dev/null || true

# Summary
echo ""
echo "========================================"
echo "Test Summary"
echo "========================================"
if [ $GO_RESULT -eq 0 ]; then
    echo -e "Go server:   ${GREEN}✓ PASSED${NC}"
else
    echo -e "Go server:   ${RED}✗ FAILED${NC}"
fi

if [ $RUST_RESULT -eq 0 ]; then
    echo -e "Rust server: ${GREEN}✓ PASSED${NC}"
else
    echo -e "Rust server: ${RED}✗ FAILED${NC}"
fi

echo ""

# Exit with error if any test failed
if [ $GO_RESULT -ne 0 ] || [ $RUST_RESULT -ne 0 ]; then
    echo -e "${RED}Some tests failed${NC}"
    exit 1
else
    echo -e "${GREEN}All tests passed!${NC}"
    exit 0
fi
