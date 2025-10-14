// Key-Value Store Operations Examples - Fast caching and session management
const fetch = require('node-fetch');
require('dotenv').config({ path: require('path').join(__dirname, '..', '..', '.env') });

async function kvOperationsExamples() {
    const BASE_URL = process.env.API_BASE_URL || 'http://localhost:8080';
    const API_KEY = process.env.API_BASE_KEY || 'a-test-api-key-from-ekodb';
    
    // Get authentication token
    let TOKEN = process.env.AUTH_TOKEN;
    if (!TOKEN) {
        const tokenResponse = await fetch(`${BASE_URL}/api/auth/token`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ api_key: API_KEY })
        });
        const tokenData = await tokenResponse.json();
        TOKEN = tokenData.token;
    }

    console.log('✓ Authentication successful');

    try {
        // Example 1: Set a key-value pair
        console.log('\n=== KV Set ===');
        const setValue = await fetch(`${BASE_URL}/api/kv/set/session:user123`, {
            method: 'POST',
            headers: {
                'Authorization': `Bearer ${TOKEN}`,
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                value: { userId: 123, username: 'john_doe', loginTime: new Date().toISOString() }
            })
        });
        const setResult = await setValue.json();
        console.log('✓ Set key: session:user123');

        // Example 2: Get a value
        console.log('\n=== KV Get ===');
        const getValue = await fetch(`${BASE_URL}/api/kv/get/session:user123`, {
            method: 'GET',
            headers: {
                'Authorization': `Bearer ${TOKEN}`
            }
        });
        const getResult = await getValue.json();
        console.log('Retrieved value:', getResult.value);

        // Example 3: Set multiple keys
        console.log('\n=== Set Multiple Keys ===');
        const keys = ['cache:product:1', 'cache:product:2', 'cache:product:3'];
        for (let i = 0; i < keys.length; i++) {
            await fetch(`${BASE_URL}/api/kv/set/${keys[i]}`, {
                method: 'POST',
                headers: {
                    'Authorization': `Bearer ${TOKEN}`,
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({
                    value: { name: `Product ${i+1}`, price: 29.99 + (i * 10) }
                })
            });
        }
        console.log(`✓ Set ${keys.length} keys`);

        // Example 4: Get multiple keys
        console.log('\n=== Get Multiple Keys ===');
        for (const key of keys) {
            const resp = await fetch(`${BASE_URL}/api/kv/get/${key}`, {
                method: 'GET',
                headers: {
                    'Authorization': `Bearer ${TOKEN}`
                }
            });
            const data = await resp.json();
            console.log(`${key}:`, data.value);
        }

        // Example 5: Delete a key
        console.log('\n=== KV Delete ===');
        const deleteResponse = await fetch(`${BASE_URL}/api/kv/delete/session:user123`, {
            method: 'DELETE',
            headers: {
                'Authorization': `Bearer ${TOKEN}`
            }
        });
        if (deleteResponse.ok) {
            console.log('✓ Deleted key: session:user123');
        }
        
        // Verify deletion
        const verifyDeleteResp = await fetch(`${BASE_URL}/api/kv/get/session:user123`, {
            method: 'GET',
            headers: {
                'Authorization': `Bearer ${TOKEN}`
            }
        });
        if (verifyDeleteResp.status === 404) {
            console.log(`✓ Verified: Key successfully deleted (not found)`);
        } else {
            console.log(`✗ Warning: Key still exists after delete!`);
        }

        // Example 6: Delete multiple keys
        console.log('\n=== Delete Multiple Keys ===');
        for (const key of keys) {
            await fetch(`${BASE_URL}/api/kv/delete/${key}`, {
                method: 'DELETE',
                headers: {
                    'Authorization': `Bearer ${TOKEN}`
                }
            });
        }
        console.log(`✓ Deleted ${keys.length} keys`);

        console.log('\n✓ All KV operations completed successfully');

    } catch (error) {
        console.error('Error:', error);
        process.exit(1);
    }
}

// Run the examples
kvOperationsExamples().catch(console.error);
