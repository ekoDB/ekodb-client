/**
 * Batch Operations - Using @ekodb/ekodb-client library
 * 
 * This example demonstrates batch operations using the JavaScript client library.
 * Compare with ttl-caching/batch_operations.js to see the difference!
 */

const { EkoDBClient } = require('@ekodb/ekodb-client');
const dotenv = require('dotenv');

dotenv.config();

const BASE_URL = process.env.API_BASE_URL || 'http://localhost:8080';
const API_KEY = process.env.API_BASE_KEY || 'a-test-api-key-from-ekodb';

async function main() {
  // Create and initialize ekoDB client
  const client = new EkoDBClient(BASE_URL, API_KEY);
  await client.init();
  console.log('✓ Client created');

  const collection = 'client_batch_operations_js';

  // Example 1: Batch Insert
  console.log('\n=== Batch Insert ===');
  const records = [
    { name: 'User 1', email: 'user1@example.com', age: 25 },
    { name: 'User 2', email: 'user2@example.com', age: 30 },
    { name: 'User 3', email: 'user3@example.com', age: 35 },
    { name: 'User 4', email: 'user4@example.com', age: 40 },
    { name: 'User 5', email: 'user5@example.com', age: 45 },
  ];

  const inserted = await client.batchInsert(collection, records);
  console.log(`✓ Batch inserted ${inserted.length} records`);

  // Collect IDs
  const ids = inserted.map(rec => rec.id);

  // Verify
  const allDocs = await client.find(collection, { limit: 100 });
  console.log(`✓ Verified: Found ${allDocs.length} total records in collection`);

  // Example 2: Batch Update
  console.log('\n=== Batch Update ===');
  const updates = ids.slice(0, 3).map(id => ({
    id: id,
    data: { status: 'active', updated: true },
  }));

  const updated = await client.batchUpdate(collection, updates);
  console.log(`✓ Batch updated ${updated.length} records`);

  // Example 3: Batch Delete
  console.log('\n=== Batch Delete ===');
  const deleteIDs = ids.slice(0, 3);
  const deletedCount = await client.batchDelete(collection, deleteIDs);
  console.log(`✓ Batch deleted ${deletedCount} records`);

  // Cleanup: Delete the collection
  console.log('\n=== Cleanup ===');
  await client.deleteCollection(collection);
  console.log('✓ Deleted collection');

  console.log('\n✓ All batch operations completed successfully');
}

main().catch(console.error);
