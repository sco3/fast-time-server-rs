#!/bin/bash
# Test script to verify REST API benchmark setup

set -e

echo "🚀 Testing REST API Benchmark Setup"
echo "===================================="
echo ""

# Check if hey is installed
if ! command -v hey >/dev/null 2>&1; then
    echo "❌ 'hey' is not installed"
    echo "Install it with: go install github.com/rakyll/hey@latest"
    exit 1
fi
echo "✅ 'hey' is installed"

# Check if server is running
echo ""
echo "Checking if server is running on port 8080..."
if ! curl -s http://localhost:8080/health > /dev/null 2>&1; then
    echo "❌ Server is not running on port 8080"
    echo ""
    echo "Start the server in REST mode first:"
    echo "  make run-rest"
    echo ""
    echo "Or in another terminal:"
    echo "  ./dist/fast-time-server -transport=rest -port=8080"
    exit 1
fi
echo "✅ Server is running"

# Check if it's in REST mode
echo ""
echo "Checking if server is in REST mode..."
if curl -s http://localhost:8080/api/v1/time > /dev/null 2>&1; then
    echo "✅ Server is in REST mode"
else
    echo "❌ Server is NOT in REST mode"
    echo ""
    echo "Restart the server with REST transport:"
    echo "  make run-rest"
    exit 1
fi

# Test single request
echo ""
echo "Testing single REST API request..."
RESPONSE=$(curl -s -X POST http://localhost:8080/api/v1/convert \
    -H "Content-Type: application/json" \
    -d @payload-rest.json)

if echo "$RESPONSE" | grep -q "converted_time"; then
    echo "✅ Single request successful"
    echo "   Response: $RESPONSE"
else
    echo "❌ Single request failed"
    echo "   Response: $RESPONSE"
    exit 1
fi

# Run a quick benchmark
echo ""
echo "Running quick benchmark (100 requests, 10 concurrent)..."
hey -m POST -T 'application/json' \
    -D payload-rest.json \
    -n 100 -c 10 http://localhost:8080/api/v1/convert

echo ""
echo "✅ Benchmark test completed successfully!"
echo ""
echo "To run full benchmark:"
echo "  make bench-rest"
echo ""
echo "This will run 100,000 requests with 100 concurrent connections"