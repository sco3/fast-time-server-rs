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
    cargo test --lib

# Run all Go tests
test-go:
    go test -v ./...

# Run specific Rust test
test-one TEST:
    cargo test --lib {{TEST}}

# Generate test coverage report for Rust (requires cargo-tarpaulin)
coverage:
    @echo "📊 Generating Rust test coverage report..."
    @command -v cargo-tarpaulin >/dev/null 2>&1 || { \
        echo "❌ cargo-tarpaulin not installed"; \
        echo "Install with: cargo install cargo-tarpaulin"; \
        exit 1; \
    }
    cargo tarpaulin --lib --out Html --output-dir coverage --exclude-files 'src/main.rs' --exclude-files 'src/openapi.rs'
    @echo "✅ Coverage report generated in coverage/index.html"
    @echo "Open with: xdg-open coverage/index.html"

# Generate coverage using llvm-cov (alternative)
coverage-llvm:
    @echo "📊 Generating Rust test coverage with llvm-cov..."
    @command -v cargo-llvm-cov >/dev/null 2>&1 || { \
        echo "❌ cargo-llvm-cov not installed"; \
        echo "Install with: cargo install cargo-llvm-cov"; \
        exit 1; \
    }
    cargo llvm-cov --lib --html --output-dir coverage-llvm
    @echo "✅ Coverage report generated in coverage-llvm/index.html"

# Generate coverage report and open in browser
coverage-open:
    @just coverage
    @xdg-open coverage/index.html 2>/dev/null || open coverage/index.html 2>/dev/null || echo "Open coverage/index.html manually"

# Generate Go test coverage
coverage-go:
    @echo "📊 Generating Go test coverage..."
    @mkdir -p coverage
    go test -coverprofile=coverage/go-coverage.out ./...
    go tool cover -html=coverage/go-coverage.out -o coverage/go-coverage.html
    @echo "✅ Go coverage report generated in coverage/go-coverage.html"

# Run all tests and generate coverage for both Rust and Go
coverage-all:
    @just coverage
    @just coverage-go
    @echo ""
    @echo "✅ All coverage reports generated:"
    @echo "  - Rust: coverage/index.html"
    @echo "  - Go: coverage/go-coverage.html"

# Clean build artifacts
clean:
    cargo clean
    go clean
    rm -rf coverage coverage-llvm dist target/

# Format code
fmt:
    cargo fmt
    go fmt ./...

# Run linter
lint:
    cargo clippy --lib -- -D warnings
    golangci-lint run

# Install test coverage tools
install-coverage-tools:
    @echo "📦 Installing coverage tools..."
    cargo install cargo-tarpaulin
    cargo install cargo-llvm-cov
    @echo "✅ Coverage tools installed"

# Run benchmarks
bench:
    @echo "Running Go benchmarks..."
    go test -bench=. -benchmem ./...

# Run REST API benchmark (requires hey)
bench-rest:
    @command -v hey >/dev/null || { echo '"hey" not installed'; exit 1; }
    @echo "➜ load-test convert_time via REST API /api/v1/convert endpoint"
    hey -m POST -T 'application/json' \
        -D payload-rest.json \
        -n 100000 -c 100 http://localhost:8080/api/v1/convert

# Start server in REST mode
run-rest:
    cargo run --release -- --transport rest --port 8080

# Start server in STDIO mode
run-stdio:
    cargo run --release -- --transport stdio

# Show test coverage summary (requires cargo-tarpaulin)
coverage-summary:
    @command -v cargo-tarpaulin >/dev/null 2>&1 || { \
        echo "❌ cargo-tarpaulin not installed"; \
        echo "Install with: cargo install cargo-tarpaulin"; \
        exit 1; \
    }
    @cargo tarpaulin --lib --exclude-files 'src/main.rs' --exclude-files 'src/openapi.rs' 2>&1 | grep -E '(^test|coverage)'

# Watch and run tests on file changes (requires cargo-watch)
watch:
    @command -v cargo-watch >/dev/null 2>&1 || { \
        echo "❌ cargo-watch not installed"; \
        echo "Install with: cargo install cargo-watch"; \
        exit 1; \
    }
    cargo watch -x 'test --lib'

# CI/CD: Run all checks
ci: fmt lint test coverage-summary
    @echo "✅ All CI checks passed!"
</content>
</write_file>

Now let's test that the justfile works and create a quick guide:

<write_to_file>
<path>JUSTFILE_GUIDE.md</path>
<content># Justfile Guide for fast-time-server

## What is Just?

`just` is a command runner similar to `make` but with a simpler syntax. It's a handy tool for running project commands.

## Installation

```bash
# On macOS
brew install just

# On Linux
cargo install just

# On Arch Linux
pacman -S just
```

## Available Commands

### Building

```bash
# Build Rust project (release mode)
just build

# Build Go project
just build-go
```

### Testing

```bash
# Run all Rust tests
just test

# Run all Go tests
just test-go

# Run specific Rust test
just test-one test_get_system_time_utc

# Watch and run tests on file changes (requires cargo-watch)
just watch
```

### Test Coverage

```bash
# Generate HTML coverage report for Rust (recommended)
just coverage

# Generate coverage and open in browser
just coverage-open

# Generate coverage using llvm-cov (alternative)
just coverage-llvm

# Generate Go test coverage
just coverage-go

# Generate coverage for both Rust and Go
just coverage-all

# Show coverage summary in terminal
just coverage-summary
```

#### Coverage Requirements

The coverage commands require additional tools:

```bash
# Install coverage tools
just install-coverage-tools

# Or manually:
cargo install cargo-tarpaulin
cargo install cargo-llvm-cov
```

### Code Quality

```bash
# Format code (both Rust and Go)
just fmt

# Run linters
just lint

# Run all CI checks (format, lint, test, coverage)
just ci
```

### Running the Server

```bash
# Start server in REST mode
just run-rest

# Start server in STDIO mode
just run-stdio
```

### Benchmarking

```bash
# Run Go benchmarks
just bench

# Run REST API load test (requires server running)
just bench-rest
```

### Maintenance

```bash
# Clean build artifacts
just clean

# List all available commands
just
# or
just --list
```

## Coverage Reports

After running `just coverage`, you'll find the coverage report at:
- **Rust**: `coverage/index.html`
- **Go**: `coverage/go-coverage.html`

### Coverage Output Example

```
📊 Generating Rust test coverage report...
|| Tested/Total Lines:
|| src/tools.rs: 45/50
|| src/resources.rs: 30/35
|| src/prompts.rs: 25/30
|| src/mcp.rs: 15/20
|| 
|| 82.35% coverage, 115/140 lines covered
✅ Coverage report generated in coverage/index.html
```

## Quick Start

1. **Install Just**:
   ```bash
   cargo install just
   ```

2. **See available commands**:
   ```bash
   just
   ```

3. **Run tests with coverage**:
   ```bash
   just install-coverage-tools  # First time only
   just coverage-open           # Generates and opens report
   ```

4. **Run in CI/CD**:
   ```bash
   just ci
   ```

## CI/CD Integration

### GitHub Actions Example

```yaml
name: CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      
      - name: Install Just
        run: cargo install just
      
      - name: Install coverage tools
        run: just install-coverage-tools
      
      - name: Run CI checks
        run: just ci
      
      - name: Upload coverage
        uses: codecov/codecov-action@v3
        with:
          files: ./coverage/cobertura.xml
```

### GitLab CI Example

```yaml
test:
  image: rust:latest
  before_script:
    - cargo install just
    - just install-coverage-tools
  script:
    - just ci
  coverage: '/\d+\.\d+% coverage/'
```

## Tips

1. **Tab Completion**: Enable tab completion for just:
   ```bash
   # For bash
   just --completions bash > /etc/bash_completion.d/just
   
   # For zsh
   just --completions zsh > /usr/local/share/zsh/site-functions/_just
   ```

2. **Parallel Tests**: Just runs tests in parallel by default. To run sequentially:
   ```bash
   cargo test --lib -- --test-threads=1
   ```

3. **Watching Tests**: Install `cargo-watch` for automatic test running:
   ```bash
   cargo install cargo-watch
   just watch
   ```

4. **Coverage Threshold**: Add coverage threshold checks to CI:
   ```bash
   just coverage-summary | grep -q "8[0-9]\.[0-9]*% coverage" || exit 1
   ```

## Troubleshooting

### cargo-tarpaulin fails to install
Try using Docker:
```bash
docker run --security-opt seccomp=unconfined -v "${PWD}:/volume" \
  xd009642/tarpaulin cargo tarpaulin --lib --out Html
```

### Permission denied
```bash
chmod +x justfile
```

### Command not found
Make sure `just` is in your PATH after installation.

## Additional Resources

- [Just Documentation](https://github.com/casey/just)
- [cargo-tarpaulin](https://github.com/xd009642/tarpaulin)
- [cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov)