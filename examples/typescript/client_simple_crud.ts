/**
 * Simple CRUD Operations - Using @ekodb/ekodb-client library
 *
 * This example demonstrates CRUD operations using the TypeScript client library.
 * Compare with javascript/ttl-caching/simple_crud.js to see the difference!
 */

import {
  EkoDBClient,
  getValue,
  extractRecord,
  getDateTimeValue,
  getUUIDValue,
  getDecimalValue,
  getBytesValue,
  getArrayValue,
  getSetValue,
  getVectorValue,
  getObjectValue,
} from "@ekodb/ekodb-client";
import * as dotenv from "dotenv";

dotenv.config();

const BASE_URL = process.env.API_BASE_URL || "http://localhost:8080";
const API_KEY = process.env.API_BASE_KEY || "a-test-api-key-from-ekodb";

async function main() {
  // Create and initialize ekoDB client
  const client = new EkoDBClient(BASE_URL, API_KEY);
  await client.init();
  console.log("✓ Client created (token exchange happens automatically)");

  const collection = "client_simple_crud_ts";

  // Example 1: Insert a document with various field types
  console.log("\n=== Insert Document ===");
  const record = {
    name: "Test Record",
    value: 42,
    active: true,
    price: 99.99,
    created_at: new Date().toISOString(),
    user_id: "550e8400-e29b-41d4-a716-446655440000",
    tags: ["tag1", "tag2", "tag3"],
    metadata: { key: "value", nested: { deep: true } },
    embedding: [0.1, 0.2, 0.3, 0.4, 0.5],
    categories: ["electronics", "computers"],
    data: Buffer.from("hello world").toString("base64"),
  };

  const inserted = await client.insert(collection, record);
  const docID = inserted.id as string;
  console.log("Inserted:", inserted);

  // Example 2: Find by ID
  console.log("\n=== Find by ID ===");
  const found = await client.findById(collection, docID);
  console.log("Found:", found);

  // Example 2b: Extract field values using type-specific getValue utilities
  console.log("\n=== Extract Field Values (All Types) ===");
  // ekoDB returns fields as { type: "String", value: "actual_value" }
  // Use type-specific getValue helpers to extract values
  const name = getValue(found.name);
  const valueNum = getValue(found.value);
  const active = getValue(found.active);
  const price = getDecimalValue(found.price);
  const createdAt = getDateTimeValue(found.created_at);
  const userId = getUUIDValue(found.user_id);
  const tags = getArrayValue(found.tags);
  const metadata = getObjectValue(found.metadata);
  const embedding = getVectorValue(found.embedding);
  const categories = getSetValue(found.categories);
  const data = getBytesValue(found.data);

  console.log("Extracted values:");
  console.log("  name (String):", name);
  console.log("  value (Integer):", valueNum);
  console.log("  active (Boolean):", active);
  console.log("  price (Decimal):", price);
  console.log("  created_at (DateTime):", createdAt);
  console.log("  user_id (UUID):", userId);
  console.log("  tags (Array):", tags);
  console.log("  metadata (Object):", metadata);
  console.log("  embedding (Vector):", embedding);
  console.log("  categories (Set):", categories);
  console.log("  data (Bytes):", data?.length, "bytes");

  // Or extract all fields at once using extractRecord()
  const plainRecord = extractRecord(found);
  console.log("Plain record:", plainRecord);

  // Example 3: Find with query
  console.log("\n=== Find with Query ===");
  const docs = await client.find(collection, { limit: 10 });
  console.log(`Found documents: ${docs.length}`);

  // Example 4: Update document
  console.log("\n=== Update Document ===");
  const updateData = {
    name: "Updated Record",
    value: 100,
  };
  const updated = await client.update(collection, docID, updateData);
  console.log("Updated:", updated);

  // Example 5: Delete document
  console.log("\n=== Delete Document ===");
  await client.delete(collection, docID);
  console.log("Deleted document");

  // Cleanup: Delete the collection
  console.log("\n=== Cleanup ===");
  await client.deleteCollection(collection);
  console.log("✓ Deleted collection");

  console.log("\n✓ All CRUD operations completed successfully");
}

main().catch((error) => {
  console.error("❌ Error:", error);
  process.exit(1);
});
