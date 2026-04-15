// -*- coding: utf-8 -*-
// k6_openapi_test.js - k6 load test for OpenAPI endpoints
//
// Copyright 2025
// SPDX-License-Identifier: Apache-2.0
//
// Usage:
//   k6 run k6_openapi_test.js
//   k6 run --vus 10 --duration 30s k6_openapi_test.js

import http from 'k6/http';
import { check, group, sleep } from 'k6';
import { Rate } from 'k6/metrics';

// Custom metrics
const errorRate = new Rate('errors');

// Test configuration
export const options = {
    vus: 5,
    duration: '10s',
    thresholds: {
        http_req_duration: ['p(95)<500'], // 95% of requests should be below 500ms
        http_req_failed: ['rate<0.01'],   // Error rate should be less than 1%
        errors: ['rate<0.05'],             // Custom error rate should be less than 5%
    },
};

const BASE_URL = __ENV.BASE_URL || 'http://localhost:8080';

export default function () {
    group('OpenAPI Spec Endpoint', function () {
        const res = http.get(`${BASE_URL}/api/v1/openapi.json`);
        
        const success = check(res, {
            'status is 200': (r) => r.status === 200,
            'response is JSON': (r) => r.headers['Content-Type'] && r.headers['Content-Type'].includes('application/json'),
            'has openapi field': (r) => {
                try {
                    const body = JSON.parse(r.body);
                    return body.openapi === '3.0.0';
                } catch (e) {
                    return false;
                }
            },
            'has info field': (r) => {
                try {
                    const body = JSON.parse(r.body);
                    return body.info && body.info.title === 'Fast Time Server API';
                } catch (e) {
                    return false;
                }
            },
            'has paths field': (r) => {
                try {
                    const body = JSON.parse(r.body);
                    return body.paths && typeof body.paths === 'object';
                } catch (e) {
                    return false;
                }
            },
            'has /api/v1/time path': (r) => {
                try {
                    const body = JSON.parse(r.body);
                    return body.paths && body.paths['/api/v1/time'] !== undefined;
                } catch (e) {
                    return false;
                }
            },
            'response time < 500ms': (r) => r.timings.duration < 500,
        });
        
        errorRate.add(!success);
    });
    
    group('API Docs Endpoint', function () {
        const res = http.get(`${BASE_URL}/api/v1/docs`);
        
        const success = check(res, {
            'status is 200': (r) => r.status === 200,
            'response is HTML': (r) => r.headers['Content-Type'] && r.headers['Content-Type'].includes('text/html'),
            'contains Swagger UI': (r) => r.body.includes('swagger-ui'),
            'contains openapi.json reference': (r) => r.body.includes('/api/v1/openapi.json'),
            'response time < 500ms': (r) => r.timings.duration < 500,
        });
        
        errorRate.add(!success);
    });
    
    sleep(0.1); // Small delay between iterations
}

export function handleSummary(data) {
    return {
        'stdout': textSummary(data, { indent: ' ', enableColors: true }),
        'k6_openapi_test_summary.json': JSON.stringify(data),
    };
}

function textSummary(data, options) {
    const indent = options.indent || '';
    const enableColors = options.enableColors || false;
    
    let summary = '\n';
    summary += `${indent}✓ checks.........................: ${data.metrics.checks.values.passes}/${data.metrics.checks.values.passes + data.metrics.checks.values.fails} (${(data.metrics.checks.values.rate * 100).toFixed(2)}%)\n`;
    summary += `${indent}✗ check failures................: ${data.metrics.checks.values.fails}\n`;
    summary += `${indent}  data_received.................: ${(data.metrics.data_received.values.count / 1024).toFixed(2)} KB\n`;
    summary += `${indent}  data_sent....................: ${(data.metrics.data_sent.values.count / 1024).toFixed(2)} KB\n`;
    summary += `${indent}  http_req_blocked.............: avg=${data.metrics.http_req_blocked.values.avg.toFixed(2)}ms\n`;
    summary += `${indent}  http_req_connecting..........: avg=${data.metrics.http_req_connecting.values.avg.toFixed(2)}ms\n`;
    summary += `${indent}  http_req_duration............: avg=${data.metrics.http_req_duration.values.avg.toFixed(2)}ms p(95)=${data.metrics.http_req_duration.values['p(95)'].toFixed(2)}ms\n`;
    summary += `${indent}  http_req_failed..............: ${(data.metrics.http_req_failed.values.rate * 100).toFixed(2)}%\n`;
    summary += `${indent}  http_reqs....................: ${data.metrics.http_reqs.values.count} (${data.metrics.http_reqs.values.rate.toFixed(2)}/s)\n`;
    summary += `${indent}  iterations...................: ${data.metrics.iterations.values.count}\n`;
    summary += `${indent}  vus..........................: ${data.metrics.vus.values.value}\n`;
    
    return summary;
}