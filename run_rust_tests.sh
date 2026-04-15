#!/bin/bash
# Run Rust tests for fast-time-server

set -e

echo "🦀 Running Rust Unit Tests for fast-time-server"
echo "=============================================="
echo ""

# Check if cargo is available
if ! command -v cargo &> /dev/null; then
    echo "❌ Cargo not found. Please install Rust: https://rustup.rs/"
    exit 1
fi

echo "📦 Building project..."
cargo build --lib

echo ""
echo "🧪 Running all tests..."
cargo test --lib -- --test-threads=1

echo ""
echo "📊 Test Coverage Summary:"
echo "========================="
cargo test --lib 2>&1 | grep "test result:"

echo ""
echo "✅ All tests completed!"
echo ""
echo "To run specific tests:"
echo "  cargo test test_get_system_time_utc"
echo ""
echo "To run with output:"
echo "  cargo test -- --nocapture"
echo ""
echo "To see test list:"
echo "  cargo test -- --list"