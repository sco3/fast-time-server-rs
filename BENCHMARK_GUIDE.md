# Benchmark Guide for fast-time-server

## Overview

This guide explains how to benchmark the fast-time-server using the `hey` load testing tool.

## Prerequisites

1. Install `hey` load testing tool:
   ```bash
   go install github.com/rakyll/hey@latest
   ```

2. Build the server:
   ```bash
   make build
   ```

## Benchmarking REST API (Recommended)

### Step 1: Start Server in REST Mode

In one terminal:
```bash
make run-rest
```

This starts the server on port 8080 with REST API endpoints at `/api/v1/*`

### Step 2: Run REST API Benchmark

In another terminal:
```bash
make bench-rest
```

This will:
- Send 100,000 requests to `/api/v1/convert`
- Use 100 concurrent connections
- Use the REST API payload from `payload-rest.json`

### Step 3: Test Before Full Benchmark

Run the test script first to verify everything is set up correctly:
```bash
./test-rest-benchmark.sh
```

## Benchmarking MCP HTTP Protocol

### Step 1: Start Server in DUAL Mode

In one terminal:
```bash
make run-dual
```

This starts the server with both SSE and HTTP MCP endpoints

### Step 2: Run MCP HTTP Benchmark

In another terminal:
```bash
make bench
```

This will:
- Send 100,000 requests to `/http` (MCP protocol endpoint)
- Use 100 concurrent connections
- Use the MCP JSON-RPC payload from `payload.json`

## Payload Files

### REST API Payload (`payload-rest.json`)
```json
{
  "time": "2025-06-21T09:00:00Z",
  "from_timezone": "Europe/Berlin",
  "to_timezone": "Europe/Dublin"
}
```

### MCP Protocol Payload (`payload.json`)
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "convert_time",
    "arguments": {
      "source_timezone": "Europe/Berlin",
      "target_timezone": "Europe/Dublin",
      "time": "2025-06-21 09:00:00"
    }
  }
}
```

## REST API Endpoints

- `GET /api/v1/time` - Get current time (UTC by default)
- `GET /api/v1/time/{timezone}` - Get current time in specific timezone
- `POST /api/v1/convert` - Convert time between timezones
- `POST /api/v1/convert/batch` - Batch convert multiple times
- `GET /api/v1/timezones` - List available timezones
- `GET /api/v1/docs` - API documentation (Swagger UI)
- `GET /health` - Health check
- `GET /version` - Version information

## Expected Performance

On modern hardware, you should see:
- **Throughput**: 10,000+ requests/second
- **Latency p50**: < 5ms
- **Latency p99**: < 20ms

## Troubleshooting

### Server not running
```bash
# Check if server is running
curl http://localhost:8080/health

# If not, start it
make run-rest
```

### Wrong transport mode
The REST benchmark requires the server to be running in REST mode:
```bash
./dist/fast-time-server -transport=rest -port=8080
```

### Port already in use
```bash
# Find what's using port 8080
lsof -i :8080

# Kill it if needed
kill -9 <PID>
```

## Custom Benchmarks

### Adjust concurrency and request count
```bash
hey -m POST -T 'application/json' \
    -D payload-rest.json \
    -n 50000 -c 50 \
    http://localhost:8080/api/v1/convert
```

### Test different endpoint
```bash
# Test GET time endpoint
hey -n 100000 -c 100 http://localhost:8080/api/v1/time/America/New_York
```

### Test with authentication
If running with auth token:
```bash
hey -m POST -T 'application/json' \
    -H "Authorization: Bearer secret123" \
    -D payload-rest.json \
    -n 100000 -c 100 \
    http://localhost:8080/api/v1/convert