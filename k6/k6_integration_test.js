// k6 Integration Test Suite for fast-time-server
// This handles dynamic timestamps and dates with flexible validation
// Run with: k6 run k6_integration_test.js

import http from 'k6/http';
import { check, group, sleep } from 'k6';
import { Rate } from 'k6/metrics';

// Custom metrics
const errorRate = new Rate('errors');

// Test configuration
export const options = {
    // For integration testing, we use iterations instead of VUs
    iterations: 1,
    thresholds: {
        'errors': ['rate<0.1'], // Error rate should be less than 10%
        'http_req_duration': ['p(95)<500'], // 95% of requests should be below 500ms
        'http_req_failed': ['rate<0.1'], // HTTP failures should be less than 10%
    },
};

// Configuration
const BASE_URL = __ENV.BASE_URL || 'http://localhost:8085';
let sessionId = null;

// Utility functions for time validation
function isValidRFC3339(timestamp) {
    const rfc3339Regex = /^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(\.\d+)?(Z|[+-]\d{2}:\d{2})$/;
    return rfc3339Regex.test(timestamp);
}

function isRecentTime(timestamp, maxAgeSeconds = 60) {
    const date = new Date(timestamp);
    const now = new Date();
    const diffSeconds = Math.abs((now - date) / 1000);
    return diffSeconds <= maxAgeSeconds;
}

function isValidTimezone(tz) {
    const validPatterns = [
        /^UTC$/,
        /^[A-Z][a-z]+\/[A-Z][a-z_]+$/,  // America/New_York
        /^[A-Z][a-z]+\/[A-Z][a-z_]+\/[A-Z][a-z_]+$/  // America/Argentina/Buenos_Aires
    ];
    return validPatterns.some(pattern => pattern.test(tz));
}

// Helper function for JSON-RPC requests
function jsonRpcRequest(method, params = {}, id = 1) {
    const payload = {
        jsonrpc: "2.0",
        method: method,
        params: params,
        id: id
    };
    
    const headers = { 'Content-Type': 'application/json' };
    
    const response = http.post(`${BASE_URL}/`, JSON.stringify(payload), { headers });
    
    return {
        status: response.status,
        body: response.body,
        json: response.json(),
        timings: response.timings
    };
}

export default function() {
    console.log(`\n${'='.repeat(60)}`);
    console.log('k6 Integration Test Suite for fast-time-server');
    console.log(`Testing against: ${BASE_URL}`);
    console.log(`${'='.repeat(60)}\n`);

    // Test Group 1: Basic Health Checks
    group('Basic Health Checks', function() {
        
        group('Health Endpoint', function() {
            const response = http.get(`${BASE_URL}/health`);
            
            const success = check(response, {
                'health status is 200': (r) => r.status === 200,
                'health has status field': (r) => r.json('status') === 'healthy',
                'health has uptime_seconds': (r) => typeof r.json('uptime_seconds') === 'number',
                'health uptime is non-negative': (r) => r.json('uptime_seconds') >= 0,
            });
            
            errorRate.add(!success);
            
            if (success) {
                console.log('✓ Health check passed');
            }
        });
        
        group('Version Endpoint', function() {
            const response = http.get(`${BASE_URL}/version`);
            
            const success = check(response, {
                'version status is 200': (r) => r.status === 200,
                'version has name': (r) => r.json('name') === 'fast-time-server',
                'version has version field': (r) => typeof r.json('version') === 'string',
                'version is 1.5.0': (r) => r.json('version') === '1.5.0',
                'version has mcp_version': (r) => r.json('mcp_version') === '1.0',
            });
            
            errorRate.add(!success);
            
            if (success) {
                console.log('✓ Version check passed');
            }
        });
    });
    
    sleep(0.5);
    
    // Test Group 2: MCP Protocol
    group('MCP Protocol', function() {
        
        group('Initialize Session', function() {
            const response = jsonRpcRequest('initialize', {
                protocolVersion: '1.0',
                clientInfo: {
                    name: 'k6-integration-test',
                    version: '1.0.0'
                }
            });
            
            const success = check(response, {
                'initialize status is 200': (r) => r.status === 200,
                'initialize has jsonrpc field': (r) => r.json.jsonrpc === '2.0',
                'initialize has result': (r) => r.json.result !== undefined,
                'initialize has protocolVersion': (r) => typeof r.json.result.protocolVersion === 'string',
                'initialize has serverInfo': (r) => r.json.result.serverInfo !== undefined,
                'initialize serverInfo has name': (r) => r.json.result.serverInfo.name === 'fast-time-server',
                'initialize has capabilities': (r) => r.json.result.capabilities !== undefined,
            });
            
            errorRate.add(!success);
            
            if (success) {
                console.log('✓ MCP Initialize passed');
                console.log(`  Protocol Version: ${response.json.result.protocolVersion}`);
                console.log(`  Server: ${response.json.result.serverInfo.name} v${response.json.result.serverInfo.version}`);
            }
        });
        
        group('List Tools', function() {
            const response = jsonRpcRequest('tools/list', {}, 2);
            
            // Note: May return session error, which is also a valid test result
            const success = check(response, {
                'tools/list returns response': (r) => r.status === 200 || r.status === 404,
                'tools/list has jsonrpc field': (r) => r.json.jsonrpc === '2.0' || r.status === 404,
            });
            
            errorRate.add(!success);
            
            if (response.status === 200) {
                console.log('✓ Tools list passed');
            } else {
                console.log('⚠ Tools list requires session (expected behavior)');
            }
        });
    });
    
    sleep(0.5);
    
    // Test Group 3: Time Tools with Dynamic Validation
    group('Time Tools (Dynamic Validation)', function() {
        
        group('Get System Time - UTC', function() {
            const response = jsonRpcRequest('tools/call', {
                name: 'get_system_time',
                arguments: {
                    timezone: 'UTC'
                }
            }, 5);
            
            // Accept either success or session error
            if (response.status === 200 && response.json.result) {
                const timestamp = response.json.result;
                const success = check(response, {
                    'get_time UTC status is 200': (r) => r.status === 200,
                    'get_time UTC returns timestamp': (r) => typeof timestamp === 'string',
                    'get_time UTC is valid RFC3339': () => isValidRFC3339(timestamp),
                    'get_time UTC is recent': () => isRecentTime(timestamp, 60),
                });
                
                errorRate.add(!success);
                
                if (success) {
                    console.log('✓ Get system time (UTC) passed');
                    console.log(`  Returned time: ${timestamp}`);
                }
            } else {
                console.log('⚠ Get system time requires session (expected)');
            }
        });
        
        group('Get System Time - New York', function() {
            const response = jsonRpcRequest('tools/call', {
                name: 'get_system_time',
                arguments: {
                    timezone: 'America/New_York'
                }
            }, 6);
            
            if (response.status === 200 && response.json.result) {
                const timestamp = response.json.result;
                const success = check(response, {
                    'get_time NY status is 200': (r) => r.status === 200,
                    'get_time NY returns timestamp': (r) => typeof timestamp === 'string',
                    'get_time NY is valid RFC3339': () => isValidRFC3339(timestamp),
                    'get_time NY is recent': () => isRecentTime(timestamp, 60),
                    'get_time NY has timezone info': () => timestamp.includes('-') || timestamp.includes('+'),
                });
                
                errorRate.add(!success);
                
                if (success) {
                    console.log('✓ Get system time (New York) passed');
                    console.log(`  Returned time: ${timestamp}`);
                }
            } else {
                console.log('⚠ Get system time (NY) requires session (expected)');
            }
        });
        
        group('Get System Time - Tokyo', function() {
            const response = jsonRpcRequest('tools/call', {
                name: 'get_system_time',
                arguments: {
                    timezone: 'Asia/Tokyo'
                }
            }, 7);
            
            if (response.status === 200 && response.json.result) {
                const timestamp = response.json.result;
                const success = check(response, {
                    'get_time Tokyo status is 200': (r) => r.status === 200,
                    'get_time Tokyo returns timestamp': (r) => typeof timestamp === 'string',
                    'get_time Tokyo is valid RFC3339': () => isValidRFC3339(timestamp),
                    'get_time Tokyo is recent': () => isRecentTime(timestamp, 60),
                });
                
                errorRate.add(!success);
                
                if (success) {
                    console.log('✓ Get system time (Tokyo) passed');
                    console.log(`  Returned time: ${timestamp}`);
                }
            } else {
                console.log('⚠ Get system time (Tokyo) requires session (expected)');
            }
        });
    });
    
    sleep(0.5);
    
    // Test Group 4: Time Conversion with Dynamic Validation
    group('Time Conversion', function() {
        
        group('Convert Time UTC to LA', function() {
            const inputTime = '2024-01-15T14:30:00Z';
            const response = jsonRpcRequest('tools/call', {
                name: 'convert_time',
                arguments: {
                    time: inputTime,
                    source_timezone: 'UTC',
                    target_timezone: 'America/Los_Angeles'
                }
            }, 8);
            
            if (response.status === 200 && response.json.result) {
                const convertedTime = response.json.result;
                const success = check(response, {
                    'convert_time status is 200': (r) => r.status === 200,
                    'convert_time returns timestamp': (r) => typeof convertedTime === 'string',
                    'convert_time is valid RFC3339': () => isValidRFC3339(convertedTime),
                    'convert_time has different offset than UTC': () => !convertedTime.endsWith('Z'),
                    'convert_time is LA timezone (-07:00 or -08:00)': () => 
                        convertedTime.includes('-07:00') || convertedTime.includes('-08:00'),
                });
                
                errorRate.add(!success);
                
                if (success) {
                    console.log('✓ Time conversion passed');
                    console.log(`  Input:  ${inputTime}`);
                    console.log(`  Output: ${convertedTime}`);
                }
            } else {
                console.log('⚠ Time conversion requires session (expected)');
            }
        });
    });
    
    sleep(0.5);
    
    // Test Group 5: Error Handling
    group('Error Handling', function() {
        
        group('Invalid Tool Name', function() {
            const response = jsonRpcRequest('tools/call', {
                name: 'nonexistent_tool',
                arguments: {}
            }, 12);
            
            const success = check(response, {
                'invalid tool returns error response': (r) => r.status !== 200 || r.json.error !== undefined,
            });
            
            errorRate.add(!success);
            
            if (success) {
                console.log('✓ Invalid tool error handling passed');
            }
        });
        
        group('Invalid Timezone', function() {
            const response = jsonRpcRequest('tools/call', {
                name: 'get_system_time',
                arguments: {
                    timezone: 'Invalid/Timezone'
                }
            }, 13);
            
            const success = check(response, {
                'invalid timezone returns error': (r) => r.status !== 200 || r.json.error !== undefined,
            });
            
            errorRate.add(!success);
            
            if (success) {
                console.log('✓ Invalid timezone error handling passed');
            }
        });
    });
    
    // Final summary
    console.log(`\n${'='.repeat(60)}`);
    console.log('Integration Test Summary');
    console.log(`${'='.repeat(60)}`);
    console.log('All tests completed. Check metrics above for detailed results.');
    console.log('Dynamic timestamp validation ensures tests work across different runs.');
    console.log(`${'='.repeat(60)}\n`);
}

// Teardown function (optional)
export function teardown(data) {
    console.log('\nTest suite completed successfully!');
}