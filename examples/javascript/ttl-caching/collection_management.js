// Collection Management Examples - Administrative operations
const fetch = require('node-fetch');
require('dotenv').config({ path: require('path').join(__dirname, '..', '..', '.env') });

async function collectionManagementExamples() {
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
        // Example 1: Create a collection by inserting data
        console.log('\n=== Create Collection (via insert) ===');
        const insertResponse = await fetch(`${BASE_URL}/api/insert/demo_collection`, {
            method: 'POST',
            headers: {
                'Authorization': `Bearer ${TOKEN}`,
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                name: 'Demo Record',
                description: 'This creates the collection automatically'
            })
        });
        const insertResult = await insertResponse.json();
        console.log('Collection created with first record:', insertResult.id);

        // Example 2: List all collections
        console.log('\n=== List Collections ===');
        const listResponse = await fetch(`${BASE_URL}/api/collections`, {
            method: 'GET',
            headers: {
                'Authorization': `Bearer ${TOKEN}`
            }
        });
        const collections = await listResponse.json();
        const collectionList = Array.isArray(collections) ? collections : (collections.collections || []);
        console.log('Total collections:', collectionList.length);
        console.log('Sample collections:', collectionList.slice(0, 5));

        // Example 3: Count documents in collection
        console.log('\n=== Count Documents ===');
        const countResponse = await fetch(`${BASE_URL}/api/find/demo_collection`, {
            method: 'POST',
            headers: {
                'Authorization': `Bearer ${TOKEN}`,
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({ limit: 1000 })
        });
        const docs = await countResponse.json();
        console.log('Document count:', Array.isArray(docs) ? docs.length : 'N/A');

        // Example 4: Delete collection
        console.log('\n=== Delete Collection ===');
        const deleteResponse = await fetch(`${BASE_URL}/api/collections/demo_collection`, {
            method: 'DELETE',
            headers: {
                'Authorization': `Bearer ${TOKEN}`
            }
        });
        
        if (deleteResponse.ok) {
            console.log('Collection deleted successfully');
        } else {
            const error = await deleteResponse.text();
            console.log('Delete response:', error);
        }

        // Example 5: Verify deletion
        console.log('\n=== Verify Deletion ===');
        const verifyResponse = await fetch(`${BASE_URL}/api/collections`, {
            method: 'GET',
            headers: {
                'Authorization': `Bearer ${TOKEN}`
            }
        });
        const remainingCollections = await verifyResponse.json();
        const remainingList = Array.isArray(remainingCollections) ? remainingCollections : (remainingCollections.collections || []);
        const stillExists = remainingList.some(c => 
            (c.name || c) === 'demo_collection'
        );
        console.log('Collection still exists:', stillExists);

        console.log('\n✓ All collection management operations completed successfully');

    } catch (error) {
        console.error('Error:', error);
        process.exit(1);
    }
}

// Run the examples
collectionManagementExamples().catch(console.error);
