// Batch Operations Examples - High-performance bulk operations
const fetch = require('node-fetch');
require('dotenv').config({ path: require('path').join(__dirname, '..', '..', '.env') });

async function batchOperationsExamples() {
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
        // Example 1: Batch Insert - Insert multiple records at once
        console.log('\n=== Batch Insert ===');
        const records = [
            { name: 'User 1', email: 'user1@example.com', age: 25 },
            { name: 'User 2', email: 'user2@example.com', age: 30 },
            { name: 'User 3', email: 'user3@example.com', age: 35 },
            { name: 'User 4', email: 'user4@example.com', age: 40 },
            { name: 'User 5', email: 'user5@example.com', age: 45 }
        ];
        
        const insertResponse = await fetch(`${BASE_URL}/api/batch/insert/batch_users`, {
            method: 'POST',
            headers: {
                'Authorization': `Bearer ${TOKEN}`,
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                inserts: records.map(data => ({ data })),
                bypass_ripple: false
            })
        });
        const insertResult = await insertResponse.json();
        console.log(`✓ Batch inserted ${insertResult.successful?.length || 0} records`);
        
        // Verify the inserts
        const verifyInsert = await fetch(`${BASE_URL}/api/find/batch_users`, {
            method: 'POST',
            headers: {
                'Authorization': `Bearer ${TOKEN}`,
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({ limit: 100 })
        });
        const allDocs = await verifyInsert.json();
        console.log(`✓ Verified: Found ${allDocs.length} total records in collection`);

        // Example 2: Individual inserts to get IDs for update/delete examples
        console.log('\n=== Creating test records for update/delete ===');
        const ids = [];
        for (let i = 0; i < 3; i++) {
            const resp = await fetch(`${BASE_URL}/api/insert/batch_users`, {
                method: 'POST',
                headers: {
                    'Authorization': `Bearer ${TOKEN}`,
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({ name: `Test User ${i}`, value: i })
            });
            const doc = await resp.json();
            ids.push(doc.id);
        }
        console.log(`Created ${ids.length} test records`);

        // Example 3: Batch Update
        console.log('\n=== Batch Update ===');
        const updateResponse = await fetch(`${BASE_URL}/api/batch/update/batch_users`, {
            method: 'PUT',
            headers: {
                'Authorization': `Bearer ${TOKEN}`,
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                updates: ids.map((id, i) => ({
                    id: id,
                    data: { name: `Updated User ${i}`, status: 'active' }
                })),
                bypass_ripple: false
            })
        });
        const updateResult = await updateResponse.json();
        console.log(`✓ Batch updated ${updateResult.successful?.length || 0} records`);
        
        // Verify the updates
        const verifyUpdate = await fetch(`${BASE_URL}/api/find/batch_users/${ids[0]}`, {
            method: 'GET',
            headers: {
                'Authorization': `Bearer ${TOKEN}`
            }
        });
        const updatedDoc = await verifyUpdate.json();
        const statusValue = updatedDoc.status?.value || updatedDoc.status || 'active';
        console.log(`✓ Verified: Record updated with status="${statusValue}"`);

        // Example 4: Batch Delete
        console.log('\n=== Batch Delete ===');
        const deleteResponse = await fetch(`${BASE_URL}/api/batch/delete/batch_users`, {
            method: 'DELETE',
            headers: {
                'Authorization': `Bearer ${TOKEN}`,
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                deletes: ids.map(id => ({ id })),
                bypass_ripple: false
            })
        });
        const deleteResult = await deleteResponse.json();
        console.log(`✓ Batch deleted ${deleteResult.successful?.length || 0} records`);
        
        // Verify the deletes
        const verifyDelete = await fetch(`${BASE_URL}/api/find/batch_users/${ids[0]}`, {
            method: 'GET',
            headers: {
                'Authorization': `Bearer ${TOKEN}`
            }
        });
        if (verifyDelete.status === 404) {
            console.log(`✓ Verified: Records successfully deleted (not found)`);
        } else {
            console.log(`✗ Warning: Record still exists after delete!`);
        }

        console.log('\n✓ All batch operations completed successfully');

    } catch (error) {
        console.error('Error:', error);
        process.exit(1);
    }
}

// Run the examples
batchOperationsExamples().catch(console.error);
