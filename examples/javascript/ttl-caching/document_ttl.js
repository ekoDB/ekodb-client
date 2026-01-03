// Document TTL Examples using direct HTTP/WebSocket calls
const WebSocket = require('ws');
const fetch = require('node-fetch');
require('dotenv').config({ path: require('path').join(__dirname, '..', '..', '.env') });

async function documentTTLExamples() {
    const BASE_URL = process.env.API_BASE_URL || 'http://localhost:8080';
    const WS_URL = process.env.WS_BASE_URL || 'ws://localhost:8080';
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
        // Example 1: Insert document with TTL (1 hour)
        console.log('\n=== Insert Document with TTL (1 hour) ===');
        const doc1 = await fetch(`${BASE_URL}/api/insert/ttl_cache`, {
            method: 'POST',
            headers: {
                'Authorization': `Bearer ${TOKEN}`,
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                key: 'session_token',
                value: 'abc123',
                ttl: 3600
            })
        }).then(res => res.json());
        console.log(`✓ Inserted document: ${doc1.id}`);

        // Example 2: Insert with shorter TTL (integer seconds)
        console.log('\n=== Insert Document with TTL (5 minutes - integer) ===');
        const doc2 = await fetch(`${BASE_URL}/api/insert/ttl_cache`, {
            method: 'POST',
            headers: {
                'Authorization': `Bearer ${TOKEN}`,
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                key: 'temp_data',
                value: { important: true },
                ttl: 300
            })
        }).then(res => res.json());
        console.log(`✓ Inserted document: ${doc2.id || 'created'}`);

        // Example 3: Insert with duration string format
        console.log('\n=== Insert Document with TTL (30 minutes - duration string) ===');
        const doc3 = await fetch(`${BASE_URL}/api/insert/ttl_cache`, {
            method: 'POST',
            headers: {
                'Authorization': `Bearer ${TOKEN}`,
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                key: 'duration_test',
                value: 'testing duration strings',
                ttl: '30m'
            })
        }).then(res => res.json());
        console.log(`✓ Inserted document with duration string TTL: ${doc3.id || 'created'}`);

        // Example 4: Query documents
        console.log('\n=== Query Documents ===');
        const docs = await fetch(`${BASE_URL}/api/find/ttl_cache`, {
            method: 'POST',
            headers: {
                'Authorization': `Bearer ${TOKEN}`,
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                limit: 10
            })
        }).then(res => res.json());
        console.log(`✓ Found ${docs.length} documents with TTL`);

        // Example 4: Update document
        console.log('\n=== Update Document ===');
        await fetch(`${BASE_URL}/api/update/ttl_cache/${doc1.id}`, {
            method: 'PUT',
            headers: {
                'Authorization': `Bearer ${TOKEN}`,
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                value: 'updated_value'
            })
        });
        console.log('✓ Updated document');

        // Example 5: Delete document
        console.log('\n=== Delete Document ===');
        if (doc1.id) {
            await fetch(`${BASE_URL}/api/delete/ttl_cache/${doc1.id}`, {
                method: 'DELETE',
                headers: {
                    'Authorization': `Bearer ${TOKEN}`
                }
            });
            console.log('✓ Deleted document');
        } else {
            console.log('✓ Skipped delete (no ID returned)');
        }

        console.log('\n✓ All document TTL operations completed successfully');

    } catch (error) {
        console.error('Error:', error);
        process.exit(1);
    }
}

// Run the examples
documentTTLExamples().catch((error) => {
  console.error("❌ Error:", error);
  process.exit(1);
});
