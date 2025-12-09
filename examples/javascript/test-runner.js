// Test Runner for JavaScript Examples
// This script runs all JavaScript examples as integration tests
const fetch = require('node-fetch');
const { spawn } = require('child_process');
const path = require('path');
require('dotenv').config({ path: path.join(__dirname, '..', '.env') });

const BASE_URL = process.env.API_BASE_URL || 'http://localhost:8080';
const API_KEY = process.env.API_BASE_KEY || 'a-test-api-key-from-ekodb';

// Colors for output
const colors = {
    green: '\x1b[32m',
    red: '\x1b[31m',
    yellow: '\x1b[33m',
    blue: '\x1b[34m',
    reset: '\x1b[0m'
};

function log(message, color = 'reset') {
    console.log(`${colors[color]}${message}${colors.reset}`);
}

// Check if server is running
async function checkServer() {
    log('\n=== Checking Server Connection ===', 'blue');
    
    for (let i = 0; i < 30; i++) {
        try {
            const response = await fetch(`${BASE_URL}/api/health`, { timeout: 1000 });
            if (response.ok) {
                log('✓ Server is ready', 'green');
                return true;
            }
        } catch (error) {
            process.stdout.write('.');
            await new Promise(resolve => setTimeout(resolve, 1000));
        }
    }
    
    log('\n✗ Server not responding', 'red');
    return false;
}

// Get authentication token
async function getAuthToken() {
    log('\n=== Getting Authentication Token ===', 'blue');
    
    try {
        const response = await fetch(`${BASE_URL}/api/auth/token`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ api_key: API_KEY })
        });
        
        if (!response.ok) {
            throw new Error(`Auth failed: ${response.status}`);
        }
        
        const data = await response.json();
        log('✓ Authentication successful', 'green');
        return data.token;
    } catch (error) {
        log(`✗ Authentication failed: ${error.message}`, 'red');
        throw error;
    }
}

// Run a single example file
function runExample(file, token) {
    return new Promise((resolve, reject) => {
        log(`\n=== Running ${file} ===`, 'blue');
        
        const env = {
            ...process.env,
            API_BASE_URL: BASE_URL,
            WS_BASE_URL: process.env.WS_BASE_URL || 'ws://localhost:8080',
            AUTH_TOKEN: token
        };
        
        const child = spawn('node', [file], {
            cwd: path.dirname(file),
            env,
            stdio: 'inherit'
        });
        
        child.on('exit', (code) => {
            if (code === 0) {
                log(`✓ ${path.basename(file)} completed successfully`, 'green');
                resolve();
            } else {
                log(`✗ ${path.basename(file)} failed with code ${code}`, 'red');
                reject(new Error(`Example failed: ${file}`));
            }
        });
        
        child.on('error', (error) => {
            log(`✗ ${path.basename(file)} error: ${error.message}`, 'red');
            reject(error);
        });
    });
}

// Main test runner
async function main() {
    log('\n╔════════════════════════════════════════╗', 'blue');
    log('║  ekoDB JavaScript Examples Test Suite ║', 'blue');
    log('╚════════════════════════════════════════╝', 'blue');
    
    try {
        // Check server
        const serverReady = await checkServer();
        if (!serverReady) {
            process.exit(1);
        }
        
        // Get auth token
        const token = await getAuthToken();
        
        // List of example files to run
        const examples = [
            path.join(__dirname, 'ttl-caching', 'simple_crud.js'),
            path.join(__dirname, 'ttl-caching', 'simple_websocket.js'),
            path.join(__dirname, 'ttl-caching', 'batch_operations.js'),
            path.join(__dirname, 'ttl-caching', 'kv_operations.js'),
            path.join(__dirname, 'ttl-caching', 'collection_management.js'),
            path.join(__dirname, 'ttl-caching', 'document_ttl.js'),
            path.join(__dirname, 'ttl-caching', 'websocket_ttl.js'),
            path.join(__dirname, 'http_functions.js'),
            path.join(__dirname, 'transactions.js')
        ];
        
        log(`\n=== Running ${examples.length} Examples ===`, 'yellow');
        
        let passed = 0;
        let failed = 0;
        
        for (const example of examples) {
            try {
                await runExample(example, token);
                passed++;
            } catch (error) {
                failed++;
                log(`Continuing with next example...`, 'yellow');
            }
            
            // Wait between examples
            await new Promise(resolve => setTimeout(resolve, 2000));
        }
        
        // Summary
        log('\n╔════════════════════════════════════════╗', 'blue');
        log('║           Test Summary                 ║', 'blue');
        log('╚════════════════════════════════════════╝', 'blue');
        log(`Total: ${examples.length}`, 'blue');
        log(`Passed: ${passed}`, 'green');
        log(`Failed: ${failed}`, failed > 0 ? 'red' : 'green');
        
        if (failed > 0) {
            process.exit(1);
        }
        
    } catch (error) {
        log(`\n✗ Test suite failed: ${error.message}`, 'red');
        process.exit(1);
    }
}

// Run if called directly
if (require.main === module) {
    main();
}

module.exports = { checkServer, getAuthToken, runExample };
