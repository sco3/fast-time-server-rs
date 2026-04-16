# justfile for fast-time-server
# Run commands with: just <target>

# Default recipe to display help
default:
    @just --list

# Build the Rust project
build:
    cargo build --release

# Build the Go project
build-go:
    go build -o fast-time-server-go main.go

# Run all Rust tests
test:
    cargo test --tests

# Run all Go tests
test-go:
    go test -v ./...

# Run specific Rust test
test-one TEST:
    cargo test --test {{TEST}}

# Generate coverage report using llvm-cov
coverage:
    @echo "Generating Rust test coverage with llvm-cov..."
    @command -v cargo-llvm-cov >/dev/null 2>&1 || { \
        echo "cargo-llvm-cov not installed"; \
        echo "Install with: cargo install cargo-llvm-cov"; \
        exit 1; \
    }
    cargo llvm-cov --tests --html
    @echo "Coverage report generated in target/llvm-cov/html/index.html"

# Clean build artifacts
clean:
    cargo clean
    go clean
    rm -rf coverage dist target/

# Format code
fmt:
    cargo fmt
    go fmt ./...

# Run linter
lint:
    cargo clippy --tests -- -D warnings -W clippy::pedantic

# Install test coverage tools
install-coverage-tools:
    @echo "Installing coverage tools..."
    cargo install cargo-llvm-cov
    @echo "Coverage tools installed"

# Run benchmarks
bench:
    @echo "Running Go benchmarks..."
    go test -bench=. -benchmem ./...

# Run REST API benchmark with server lifecycle management (requires hey)
bench-rest:
    #!/usr/bin/env bash
    set -euo pipefail

    # Check for hey
    if ! command -v hey >/dev/null; then
        echo "hey not installed. Install with: go install github.com/rakyll/hey@latest"
        exit 1
    fi

    echo "Building server..."
    cargo build --release --quiet

    echo "Starting server in background..."
    cargo run --release -- --transport rest --port 8080 > /tmp/fast-time-server.log 2>&1 &
    SERVER_PID=$!
    echo $SERVER_PID > /tmp/fast-time-server.pid

    echo "Waiting for server to be ready..."
    for i in {1..5}; do
        if curl -s http://localhost:8080/health > /dev/null 2>&1; then
            echo "Server is ready"
            break
        fi
        echo "  Waiting... ($i/5)"
        sleep 1
    done

    # Check if server is actually ready
    if ! curl -s http://localhost:8080/health > /dev/null 2>&1; then
        echo "Server failed to start"
        cat /tmp/fast-time-server.log
        kill $SERVER_PID 2>/dev/null || true
        rm -f /tmp/fast-time-server.pid /tmp/fast-time-server.log
        exit 1
    fi

    echo "Running REST API benchmarks..."
    hey -m POST -T 'application/json' \
        -D payload-rest.json \
        -n 100000 -c 100 http://localhost:8080/api/v1/convert || true

    echo "Stopping server..."
    kill $SERVER_PID 2>/dev/null || true
    rm -f /tmp/fast-time-server.pid /tmp/fast-time-server.log

    echo "Benchmark complete"

# Run REST API benchmark (assumes server is already running)
bench-rest-quick:
    @command -v hey >/dev/null || { echo '"hey" not installed. Install with: go install github.com/rakyll/hey@latest'; exit 1; }
    @echo "Running REST API benchmark (server must be running on :8080)..."
    hey -m POST -T 'application/json' \
        -D payload-rest.json \
        -n 100000 -c 100 http://localhost:8080/api/v1/convert

# Start server in REST mode
run-rest:
    cargo run --release -- --transport rest --port 8080

# Start server in STDIO mode
run-stdio:
    cargo run --release -- --transport stdio

# Show test coverage summary
coverage-summary:
    @command -v cargo-llvm-cov >/dev/null 2>&1 || { \
        echo "cargo-llvm-cov not installed"; \
        echo "Install with: cargo install cargo-llvm-cov"; \
        exit 1; \
    }
    cargo llvm-cov --tests --summary-only

# Watch and run tests on file changes (requires cargo-watch)
watch:
    @command -v cargo-watch >/dev/null 2>&1 || { \
        echo "cargo-watch not installed"; \
        echo "Install with: cargo install cargo-watch"; \
        exit 1; \
    }
    cargo watch -x 'test --tests'

# CI/CD: Run all checks
ci: fmt lint test coverage-summary
    @echo "All CI checks passed!"