/**
 * Simple CRUD Operations - Using @ekodb/ekodb-client library
 * 
 * This example demonstrates CRUD operations using the JavaScript client library.
 * Compare with ttl-caching/simple_crud.js to see the difference!
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
  console.log('✓ Client created (token exchange happens automatically)');

  const collection = 'client_simple_crud_js';

  // Example 1: Insert a document
  console.log('\n=== Insert Document ===');
  const record = {
    name: 'Test Record',
    value: 42,
    active: true,
  };

  const inserted = await client.insert(collection, record);
  const docID = inserted.id;
  console.log('Inserted:', inserted);

  // Example 2: Find by ID
  console.log('\n=== Find by ID ===');
  const found = await client.findById(collection, docID);
  console.log('Found:', found);

  // Example 3: Find with query
  console.log('\n=== Find with Query ===');
  const docs = await client.find(collection, { limit: 10 });
  console.log(`Found documents: ${docs.length}`);

  // Example 4: Update document
  console.log('\n=== Update Document ===');
  const updateData = {
    name: 'Updated Record',
    value: 100,
  };
  const updated = await client.update(collection, docID, updateData);
  console.log('Updated:', updated);

  // Example 5: Delete document
  console.log('\n=== Delete Document ===');
  await client.delete(collection, docID);
  console.log('Deleted document');

  // Cleanup: Delete the collection
  console.log('\n=== Cleanup ===');
  await client.deleteCollection(collection);
  console.log('✓ Deleted collection');

  console.log('\n✓ All CRUD operations completed successfully');
}

main().catch((error) => {
  console.error("❌ Error:", error);
  process.exit(1);
});
