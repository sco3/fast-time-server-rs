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
