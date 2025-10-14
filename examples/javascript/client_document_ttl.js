/**
 * Document TTL - Using @ekodb/ekodb-client library
 */

const { EkoDBClient } = require('@ekodb/ekodb-client');
const dotenv = require('dotenv');

dotenv.config();

const BASE_URL = process.env.API_BASE_URL || 'http://localhost:8080';
const API_KEY = process.env.API_BASE_KEY || 'a-test-api-key-from-ekodb';

async function main() {
  const client = new EkoDBClient(BASE_URL, API_KEY);
  await client.init();
  console.log('✓ Client created');

  const collection = 'client_document_ttl_js';

  console.log('\n=== Insert Document with TTL (1 hour) ===');
  const record1 = { name: 'Cache Entry 1', value: 'This will expire in 1 hour' };
  const doc1 = await client.insert(collection, record1, '1h');
  console.log(`✓ Inserted document: ${doc1.id}`);

  console.log('\n=== Insert Document with TTL (5 minutes) ===');
  const record2 = { name: 'Cache Entry 2', value: 'This will expire in 5 minutes' };
  const doc2 = await client.insert(collection, record2, '5m');
  console.log(`✓ Inserted document: ${doc2.id}`);

  console.log('\n=== Query Documents ===');
  const docs = await client.find(collection, { limit: 100 });
  console.log(`✓ Found ${docs.length} documents with TTL`);

  console.log('\n=== Update Document ===');
  const updateData = { name: 'Updated Cache Entry', value: 'Updated value' };
  await client.update(collection, doc1.id, updateData);
  console.log('✓ Updated document');

  console.log('\n=== Delete Document ===');
  await client.delete(collection, doc1.id);
  console.log('✓ Deleted document');

  // Cleanup: Delete the collection
  console.log('\n=== Cleanup ===');
  await client.deleteCollection(collection);
  console.log('✓ Deleted collection');

  console.log('\n✓ All document TTL operations completed successfully');
  console.log('\n💡 Note: Documents with TTL will automatically expire after the specified duration');
}

main().catch(console.error);
