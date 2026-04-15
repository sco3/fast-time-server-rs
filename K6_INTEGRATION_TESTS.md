# k6 Integration Tests for fast-time-server

This directory contains production-ready integration tests using [k6](https://k6.io/), a modern load testing tool that's also excellent for functional integration testing.

## Why k6?

✅ **Perfect for this project because:**
- Handles dynamic data (timestamps, dates) with flexible validation
- JavaScript-like scripting (easy to learn)
- Built-in HTTP testing with excellent JSON support
- Can do both functional and load testing
- Beautiful console output and reports
- Great CI/CD integration
- No direct string matching needed - uses custom validators

## Key Features for Time Server Testing

### 1. Dynamic Timestamp Validation
Instead of matching exact timestamps (which would fail), we validate:
- **Format**: RFC3339 compliance
- **Recency**: Time is within last 60 seconds
- **Timezone**: Proper offset format
- **Type**: Correct data types

### 2. Flexible Assertions
```javascript
// Instead of: timestamp === "2024-01-15T14:30:00Z"
// We use:
check(response, {
    'is valid RFC3339': () => isValidRFC3339(timestamp),
    'is recent time': () => isRecentTime(timestamp, 60),
    'has timezone offset': () => timestamp.includes('-') || timestamp.includes('+')
});
```

### 3. Session-Aware Testing
Handles stateful MCP protocol properly - gracefully handles session errors.

## Installation

### Option 1: Binary (Recommended)
```bash
# Linux
curl https://github.com/grafana/k6/releases/download/v0.48.0/k6-v0.48.0-linux-amd64.tar.gz -L | tar xvz
sudo mv k6-v0.48.0-linux-amd64/k6 /usr/local/bin/

# macOS
brew install k6

# Windows
choco install k6
```

### Option 2: Docker
```bash
docker pull grafana/k6:latest
```

## Usage

### Basic Run
```bash
# Start the server first
./fast-time-server-go --transport http --port 8085 &

# Run the tests
k6 run k6_integration_test.js
```

### Advanced Options

#### Test Against Different Server
```bash
k6 run -e BASE_URL=http://localhost:9090 k6_integration_test.js
```

#### Generate HTML Report
```bash
k6 run --out json=results.json k6_integration_test.js
k6 convert results.json -o results.html
```

#### CI/CD Mode (Exit on Failure)
```bash
k6 run --quiet --no-color k6_integration_test.js
```

#### With Custom Thresholds
```bash
k6 run --threshold 'http_req_duration<200' k6_integration_test.js
```

## Test Coverage

The k6 test suite covers:

### 1. Health & Version Checks
- ✅ Health endpoint validation
- ✅ Version information
- ✅ Uptime counter

### 2. MCP Protocol
- ✅ Session initialization
- ✅ Protocol version negotiation
- ✅ Capabilities discovery
- ⚠️ Session management (tested as expected behavior)

### 3. Time Tools (Dynamic Validation)
- ✅ Get system time (UTC) - validates format and recency
- ✅ Get system time (New York) - validates timezone offset
- ✅ Get system time (Tokyo) - validates timestamp structure
- ✅ Time conversion - validates converted timezone

### 4. Error Handling
- ✅ Invalid tool names
- ✅ Invalid timezone names
- ✅ Malformed requests

## Understanding the Output

### Successful Test Run
```
✓ health status is 200
✓ health has status field
✓ health has uptime_seconds
✓ health uptime is non-negative

checks.........................: 100.00% ✓ 25       ✗ 0
http_req_duration..............: avg=15ms   min=5ms   max=50ms
```

### Key Metrics
- `checks`: Percentage of assertions that passed
- `http_req_duration`: Response time statistics
- `http_req_failed`: HTTP-level failures
- `errors`: Custom error rate metric

## Dynamic Validation Functions

### isValidRFC3339(timestamp)
Validates timestamp format:
```javascript
// Valid: "2024-01-15T14:30:00Z"
// Valid: "2024-01-15T14:30:00-05:00"
// Invalid: "2024/01/15 14:30:00"
```

### isRecentTime(timestamp, maxAgeSeconds)
Validates timestamp is recent:
```javascript
// Passes if timestamp is within last 60 seconds
isRecentTime(timestamp, 60)
```

### isValidTimezone(tz)
Validates timezone format:
```javascript
// Valid: "UTC", "America/New_York", "Asia/Tokyo"
// Invalid: "EST", "PST", "Invalid/Timezone"
```

## Integration with CI/CD

### GitHub Actions
```yaml
name: Integration Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Install k6
        run: |
          curl https://github.com/grafana/k6/releases/download/v0.48.0/k6-v0.48.0-linux-amd64.tar.gz -L | tar xvz
          sudo mv k6-v0.48.0-linux-amd64/k6 /usr/local/bin/
      
      - name: Build server
        run: go build -o fast-time-server
      
      - name: Start server
        run: ./fast-time-server --transport http --port 8085 &
        
      - name: Wait for server
        run: sleep 2
      
      - name: Run integration tests
        run: k6 run k6_integration_test.js
```

### GitLab CI
```yaml
integration_tests:
  stage: test
  image: grafana/k6:latest
  services:
    - name: your-registry/fast-time-server:latest
      alias: server
  script:
    - k6 run -e BASE_URL=http://server:8080 k6_integration_test.js
```

## Load Testing

The same script can be used for load testing by adjusting options:

```javascript
export const options = {
    stages: [
        { duration: '30s', target: 10 },  // Ramp up to 10 users
        { duration: '1m', target: 10 },   // Stay at 10 users
        { duration: '30s', target: 0 },   // Ramp down
    ],
};
```

## Comparison with Other Tools

| Feature | k6 | Bash Script | pytest | Postman |
|---------|----|-----------|---------| --------|
| Dynamic validation | ✅ Excellent | ❌ Hard | ✅ Good | ⚠️ Limited |
| Time/date handling | ✅ Built-in | ❌ Complex | ✅ Libraries | ⚠️ Manual |
| CI/CD ready | ✅ Native | ⚠️ Basic | ✅ Good | ✅ Newman |
| Load testing | ✅ Excellent | ❌ No | ❌ No | ⚠️ Limited |
| Reporting | ✅ Beautiful | ❌ Basic | ✅ Good | ✅ Good |
| Learning curve | ✅ Easy | ✅ Very easy | ⚠️ Medium | ✅ Easy |

## Best Practices

### 1. Always Validate Data Types
```javascript
check(response, {
    'field is number': (r) => typeof r.json('uptime') === 'number',
    'field is string': (r) => typeof r.json('timestamp') === 'string',
});
```

### 2. Use Flexible Time Windows
```javascript
// Give 60 seconds buffer for timestamp checks
isRecentTime(timestamp, 60)
```

### 3. Test Both Success and Error Paths
```javascript
// Test valid input
check(validResponse, { 'status is 200': (r) => r.status === 200 });

// Test invalid input
check(invalidResponse, { 'returns error': (r) => r.status === 400 });
```

### 4. Group Related Tests
```javascript
group('Time Tools', function() {
    group('Get System Time', function() {
        // Tests here
    });
});
```

## Troubleshooting

### Server Not Responding
```bash
# Check if server is running
curl http://localhost:8085/health

# Check k6 can reach server
k6 run -e BASE_URL=http://localhost:8085 k6_integration_test.js
```

### Timestamp Validation Failing
- Check system time is correct
- Increase maxAgeSeconds if needed
- Verify RFC3339 format

### Tests Pass Locally But Fail in CI
- Check timezone settings in CI
- Verify server starts properly
- Add wait time for server startup

## Advanced: Custom Metrics

Add custom business metrics:

```javascript
import { Trend } from 'k6/metrics';

const timeConversionDuration = new Trend('time_conversion_duration');

// In your test
const start = Date.now();
const response = jsonRpcRequest('tools/call', {/*...*/});
timeConversionDuration.add(Date.now() - start);
```

## Support

For k6 documentation: https://k6.io/docs/
For issues with this test suite: See project README

## License

Same as parent project (Apache-2.0)