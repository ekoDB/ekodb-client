/**
 * Collection Management - Using @ekodb/ekodb-client library
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

  const collection = 'client_collection_management_js';

  console.log('\n=== Create Collection (via insert) ===');
  const record = { name: 'Test Collection', value: 'Initial record' };
  const inserted = await client.insert(collection, record);
  console.log(`Collection created with first record: ${inserted.id}`);

  console.log('\n=== List Collections ===');
  const collections = await client.listCollections();
  console.log(`Total collections: ${collections.length}`);
  console.log(`Sample collections: ${collections.slice(0, 5)}`);

  console.log('\n=== Count Documents ===');
  const docs = await client.find(collection, { limit: 1000 });
  console.log(`Document count: ${docs.length}`);

  console.log('\n=== Delete Collection ===');
  await client.deleteCollection(collection);
  console.log('Collection deleted successfully');

  console.log('\n=== Verify Deletion ===');
  const collectionsAfter = await client.listCollections();
  const exists = collectionsAfter.includes(collection);
  console.log(`Collection still exists: ${exists}`);

  console.log('\n✓ All collection management operations completed successfully');
}

main().catch(console.error);
