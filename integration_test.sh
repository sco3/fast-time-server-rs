#!/bin/bash
# Integration test script for fast-time-server Go implementation
# This script starts the server, sends various requests, and records responses

set -e

# Configuration
PORT=8085
SERVER_BIN="./fast-time-server-go"
OUTPUT_DIR="test_responses"
SERVER_PID=""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Create output directory
mkdir -p "$OUTPUT_DIR"

# Cleanup function
cleanup() {
    echo -e "${YELLOW}Cleaning up...${NC}"
    if [ ! -z "$SERVER_PID" ]; then
        kill $SERVER_PID 2>/dev/null || true
        wait $SERVER_PID 2>/dev/null || true
    fi
    echo -e "${GREEN}Cleanup complete${NC}"
}

# Set trap to cleanup on exit
trap cleanup EXIT INT TERM

# Function to wait for server to be ready
wait_for_server() {
    echo -e "${YELLOW}Waiting for server to be ready...${NC}"
    for i in {1..30}; do
        if curl -s http://localhost:$PORT/health > /dev/null 2>&1; then
            echo -e "${GREEN}Server is ready!${NC}"
            return 0
        fi
        sleep 0.5
    done
    echo -e "${RED}Server failed to start within timeout${NC}"
    return 1
}

# Function to test an endpoint
test_endpoint() {
    local name=$1
    local method=$2
    local url=$3
    local data=$4
    local output_file="$OUTPUT_DIR/${name}.json"
    
    echo -e "${YELLOW}Testing: $name${NC}"
    
    if [ -z "$data" ]; then
        # GET request
        curl -s -w "\nHTTP Status: %{http_code}\n" \
            -X "$method" \
            "http://localhost:$PORT$url" \
            | tee "$output_file"
    else
        # POST request with data
        curl -s -w "\nHTTP Status: %{http_code}\n" \
            -X "$method" \
            -H "Content-Type: application/json" \
            -d "$data" \
            "http://localhost:$PORT$url" \
            | tee "$output_file"
    fi
    
    echo ""
    
    if [ -f "$output_file" ]; then
        echo -e "${GREEN}✓ Response saved to $output_file${NC}"
    else
        echo -e "${RED}✗ Failed to save response${NC}"
    fi
    echo ""
}

# Start the server
echo -e "${GREEN}=== Starting Integration Tests ===${NC}"
echo -e "${YELLOW}Building server...${NC}"

if [ ! -f "$SERVER_BIN" ]; then
    echo -e "${RED}Server binary not found. Running go build...${NC}"
    go build -o "$SERVER_BIN"
fi

echo -e "${YELLOW}Starting server on port $PORT...${NC}"
$SERVER_BIN --transport http --port $PORT --log-level error &
SERVER_PID=$!

echo -e "${YELLOW}Server PID: $SERVER_PID${NC}"

# Wait for server to be ready
if ! wait_for_server; then
    echo -e "${RED}Failed to start server${NC}"
    exit 1
fi

echo -e "${GREEN}=== Running Tests ===${NC}"
echo ""

# Test 1: Health endpoint
test_endpoint "01_health" "GET" "/health"

# Test 2: Version endpoint
test_endpoint "02_version" "GET" "/version"

# Test 3: Initialize MCP session
test_endpoint "03_initialize" "POST" "/" \
'{
  "jsonrpc": "2.0",
  "method": "initialize",
  "params": {
    "protocolVersion": "1.0",
    "clientInfo": {
      "name": "integration-test",
      "version": "1.0.0"
    }
  },
  "id": 1
}'

# Test 4: List available tools
test_endpoint "04_tools_list" "POST" "/" \
'{
  "jsonrpc": "2.0",
  "method": "tools/list",
  "id": 2
}'

# Test 5: List resources
test_endpoint "05_resources_list" "POST" "/" \
'{
  "jsonrpc": "2.0",
  "method": "resources/list",
  "id": 3
}'

# Test 6: List prompts
test_endpoint "06_prompts_list" "POST" "/" \
'{
  "jsonrpc": "2.0",
  "method": "prompts/list",
  "id": 4
}'

# Test 7: Call get_system_time tool (UTC)
test_endpoint "07_get_time_utc" "POST" "/" \
'{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "get_system_time",
    "arguments": {
      "timezone": "UTC"
    }
  },
  "id": 5
}'

# Test 8: Call get_system_time tool (New York)
test_endpoint "08_get_time_newyork" "POST" "/" \
'{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "get_system_time",
    "arguments": {
      "timezone": "America/New_York"
    }
  },
  "id": 6
}'

# Test 9: Call get_system_time tool (Tokyo)
test_endpoint "09_get_time_tokyo" "POST" "/" \
'{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "get_system_time",
    "arguments": {
      "timezone": "Asia/Tokyo"
    }
  },
  "id": 7
}'

# Test 10: Call convert_time tool
test_endpoint "10_convert_time" "POST" "/" \
'{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "convert_time",
    "arguments": {
      "time": "2024-01-15T14:30:00Z",
      "source_timezone": "UTC",
      "target_timezone": "America/Los_Angeles"
    }
  },
  "id": 8
}'

# Test 11: Read timezone info resource
test_endpoint "11_resource_timezone_info" "POST" "/" \
'{
  "jsonrpc": "2.0",
  "method": "resources/read",
  "params": {
    "uri": "timezone://info"
  },
  "id": 9
}'

# Test 12: Read current world times resource
test_endpoint "12_resource_world_times" "POST" "/" \
'{
  "jsonrpc": "2.0",
  "method": "resources/read",
  "params": {
    "uri": "time://current/world"
  },
  "id": 10
}'

# Test 13: Get prompt - compare timezones
test_endpoint "13_prompt_compare_timezones" "POST" "/" \
'{
  "jsonrpc": "2.0",
  "method": "prompts/get",
  "params": {
    "name": "compare_timezones",
    "arguments": {
      "timezones": "America/New_York,Europe/London,Asia/Tokyo"
    }
  },
  "id": 11
}'

# Test 14: Error handling - invalid tool
test_endpoint "14_error_invalid_tool" "POST" "/" \
'{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "nonexistent_tool",
    "arguments": {}
  },
  "id": 12
}'

# Test 15: Error handling - invalid timezone
test_endpoint "15_error_invalid_timezone" "POST" "/" \
'{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "get_system_time",
    "arguments": {
      "timezone": "Invalid/Timezone"
    }
  },
  "id": 13
}'

echo -e "${GREEN}=== Test Summary ===${NC}"
echo "Total tests: 15"
echo "Responses saved in: $OUTPUT_DIR/"
echo ""

# Create a summary file
SUMMARY_FILE="$OUTPUT_DIR/test_summary.txt"
cat > "$SUMMARY_FILE" << EOF
Integration Test Summary
========================
Date: $(date)
Server: fast-time-server Go implementation
Port: $PORT

Tests Executed:
1. Health check endpoint
2. Version endpoint
3. MCP Initialize
4. List tools
5. List resources
6. List prompts
7. Get system time (UTC)
8. Get system time (New York)
9. Get system time (Tokyo)
10. Convert time (UTC to Los Angeles)
11. Read timezone info resource
12. Read world times resource
13. Get compare timezones prompt
14. Error handling - invalid tool
15. Error handling - invalid timezone

All responses have been saved to individual JSON files in this directory.
These can be used for:
- Validating API responses
- Creating test fixtures
- Comparing with other implementations
- Regression testing
EOF

echo -e "${GREEN}Summary saved to: $SUMMARY_FILE${NC}"

# Create a playback script
PLAYBACK_SCRIPT="$OUTPUT_DIR/playback_tests.sh"
cat > "$PLAYBACK_SCRIPT" << 'PLAYBACK_EOF'
#!/bin/bash
# Playback script to replay the recorded test scenarios
# Usage: ./playback_tests.sh [server_url]

SERVER_URL="${1:-http://localhost:8080}"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "Replaying tests against: $SERVER_URL"
echo ""

# Function to replay a test
replay_test() {
    local test_file=$1
    local test_name=$(basename "$test_file" .json)
    
    echo "Replaying: $test_name"
    
    # Extract the original request (everything before "HTTP Status:")
    if grep -q "POST" <<< "$test_name"; then
        # This was a POST request, extract JSON
        curl -s -X POST \
            -H "Content-Type: application/json" \
            -d @"$test_file" \
            "$SERVER_URL/"
    else
        # GET request
        curl -s "$SERVER_URL/health"
    fi
    
    echo ""
}

# Replay all tests
for test_file in "$SCRIPT_DIR"/*.json; do
    if [ -f "$test_file" ] && [ "$(basename "$test_file")" != "test_summary.txt" ]; then
        replay_test "$test_file"
    fi
done

echo "Playback complete"
PLAYBACK_EOF

chmod +x "$PLAYBACK_SCRIPT"
echo -e "${GREEN}Playback script created: $PLAYBACK_SCRIPT${NC}"

echo ""
echo -e "${GREEN}=== Integration Tests Complete ===${NC}"
echo -e "${YELLOW}To replay these tests against another server:${NC}"
echo -e "  $PLAYBACK_SCRIPT http://localhost:8080"