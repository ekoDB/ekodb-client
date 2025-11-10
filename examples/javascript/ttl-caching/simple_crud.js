// Simple CRUD Examples - Following benchmark patterns
const fetch = require('node-fetch');
require('dotenv').config({ path: require('path').join(__dirname, '..', '..', '.env') });

async function simpleCRUDExamples() {
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
        // Example 1: Insert a document
        console.log('\n=== Insert Document ===');
        const insertResponse = await fetch(`${BASE_URL}/api/insert/test_collection`, {
            method: 'POST',
            headers: {
                'Authorization': `Bearer ${TOKEN}`,
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                name: 'Test Record',
                value: 42,
                active: true
            })
        });
        const doc = await insertResponse.json();
        console.log('Inserted:', doc);

        // Example 2: Find by ID
        console.log('\n=== Find by ID ===');
        const findByIdResponse = await fetch(`${BASE_URL}/api/find/test_collection/${doc.id}`, {
            method: 'GET',
            headers: {
                'Authorization': `Bearer ${TOKEN}`
            }
        });
        const foundDoc = await findByIdResponse.json();
        console.log('Found:', foundDoc);

        // Example 3: Find with query
        console.log('\n=== Find with Query ===');
        const findResponse = await fetch(`${BASE_URL}/api/find/test_collection`, {
            method: 'POST',
            headers: {
                'Authorization': `Bearer ${TOKEN}`,
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                filter: {
                    type: 'Condition',
                    content: {
                        field: 'active',
                        operator: 'Eq',
                        value: true
                    }
                },
                limit: 10
            })
        });
        const docs = await findResponse.json();
        console.log('Found documents:', docs.length);

        // Example 4: Update document
        console.log('\n=== Update Document ===');
        const updateResponse = await fetch(`${BASE_URL}/api/update/test_collection/${doc.id}`, {
            method: 'PUT',
            headers: {
                'Authorization': `Bearer ${TOKEN}`,
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                name: 'Updated Record',
                value: 100
            })
        });
        const updated = await updateResponse.json();
        console.log('Updated:', updated);

        // Example 5: Delete document
        console.log('\n=== Delete Document ===');
        await fetch(`${BASE_URL}/api/delete/test_collection/${doc.id}`, {
            method: 'DELETE',
            headers: {
                'Authorization': `Bearer ${TOKEN}`
            }
        });
        console.log('Deleted document');

        console.log('\n✓ All CRUD operations completed successfully');

    } catch (error) {
        console.error('Error:', error);
        process.exit(1);
    }
}

// Run the examples
simpleCRUDExamples().catch((error) => {
  console.error("❌ Error:", error);
  process.exit(1);
});
