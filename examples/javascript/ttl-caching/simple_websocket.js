// Simple WebSocket Example - Following YCSB benchmark patterns
const WebSocket = require('ws');
const fetch = require('node-fetch');
require('dotenv').config({ path: require('path').join(__dirname, '..', '..', '.env') });

async function simpleWebSocketExample() {
    const BASE_URL = process.env.API_BASE_URL || 'http://localhost:8080';
    const WS_URL = process.env.WS_BASE_URL || 'ws://localhost:8080';
    const API_KEY = process.env.API_BASE_KEY || 'a-test-api-key-from-ekodb';
    
    // Step 1: Get authentication token via HTTP
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
        // Step 2: Insert some test data first
        console.log('\n=== Inserting Test Data ===');
        const insertResponse = await fetch(`${BASE_URL}/api/insert/websocket_test`, {
            method: 'POST',
            headers: {
                'Authorization': `Bearer ${TOKEN}`,
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                name: 'WebSocket Test Record',
                value: 42,
                active: true
            })
        });
        const insertResult = await insertResponse.json();
        console.log(`✓ Inserted test record: ${insertResult.id}`);

        // Step 3: Connect to WebSocket with Authorization header
        console.log('\n=== Connecting to WebSocket ===');
        const ws = new WebSocket(`${WS_URL}/api/ws`, {
            headers: {
                'Authorization': `Bearer ${TOKEN}`
            }
        });

        // Wait for connection
        await new Promise((resolve, reject) => {
            ws.on('open', () => {
                console.log('✓ WebSocket connected');
                resolve();
            });
            ws.on('error', (error) => {
                console.error('WebSocket error:', error.message);
                reject(error);
            });
        });

        // Step 4: Send a query via WebSocket
        console.log('\n=== Querying Data via WebSocket ===');
        const messageId = String(Date.now());
        
        ws.send(JSON.stringify({
            type: 'FindAll',
            messageId: messageId,
            payload: {
                collection: 'websocket_test'
            }
        }));

        // Wait for response
        const response = await new Promise((resolve, reject) => {
            const timeout = setTimeout(() => {
                reject(new Error('Response timeout'));
            }, 5000);

            ws.on('message', (data) => {
                clearTimeout(timeout);
                const message = JSON.parse(data.toString());
                resolve(message);
            });
        });

        console.log('Response:', JSON.stringify(response, null, 2));
        
        // Show the data we got back
        if (response.payload && response.payload.data) {
            console.log(`✓ Retrieved ${response.payload.data.length} record(s) via WebSocket`);
        }

        // Close connection
        ws.close();
        console.log('\n✓ WebSocket example completed successfully');

    } catch (error) {
        console.error('Error:', error);
        process.exit(1);
    }
}

// Run the example
simpleWebSocketExample().catch(console.error);
