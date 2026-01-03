// WebSocket with TTL Examples
const WebSocket = require('ws');
const fetch = require('node-fetch');
require('dotenv').config({ path: require('path').join(__dirname, '..', '..', '.env') });

async function websocketTTLExamples() {
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
        // Insert test data with TTL
        console.log('\n=== Insert Test Data with TTL ===');
        const doc = await fetch(`${BASE_URL}/api/insert/ws_ttl_test`, {
            method: 'POST',
            headers: {
                'Authorization': `Bearer ${TOKEN}`,
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                name: 'WebSocket TTL Test',
                value: 42,
                ttl: 3600
            })
        }).then(res => res.json());
        console.log(`✓ Inserted document with TTL: ${doc.id}`);

        // Query via WebSocket
        console.log('\n=== Query via WebSocket ===');
        const ws = new WebSocket(`${WS_URL}/api/ws`, {
            headers: {
                'Authorization': `Bearer ${TOKEN}`
            }
        });

        await new Promise((resolve, reject) => {
            ws.on('open', () => {
                console.log('✓ WebSocket connected');
                resolve();
            });
            ws.on('error', reject);
        });

        const messageId = String(Date.now());
        ws.send(JSON.stringify({
            type: 'FindAll',
            messageId: messageId,
            payload: {
                collection: 'ws_ttl_test'
            }
        }));

        const response = await new Promise((resolve, reject) => {
            const timeout = setTimeout(() => reject(new Error('Timeout')), 5000);
            ws.on('message', (data) => {
                clearTimeout(timeout);
                resolve(JSON.parse(data.toString()));
            });
        });

        console.log(`✓ Retrieved ${response.payload.data.length} record(s) via WebSocket`);

        ws.close();
        console.log('\n✓ WebSocket TTL example completed successfully');

    } catch (error) {
        console.error('Error:', error);
        process.exit(1);
    }
}

// Run the examples
websocketTTLExamples().catch((error) => {
  console.error("❌ Error:", error);
  process.exit(1);
});
