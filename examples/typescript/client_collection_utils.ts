/**
 * Collection Utilities Example - Using @ekodb/ekodb-client library
 *
 * This example demonstrates collection utility methods:
 * - collectionExists: Check if a collection exists
 * - countDocuments: Count documents in a collection
 */

import { EkoDBClient } from "@ekodb/ekodb-client";
import * as dotenv from "dotenv";

dotenv.config();

const BASE_URL = process.env.API_BASE_URL || "http://localhost:8080";
const API_KEY = process.env.API_BASE_KEY || "a-test-api-key-from-ekodb";

async function main() {
  const client = new EkoDBClient(BASE_URL, API_KEY);
  await client.init();
  console.log("✓ Client created");

  const collection = "collection_utils_test_ts";

  // Example 1: Check if collection exists (should be false initially)
  console.log("\n=== Check Collection Exists (Before Creation) ===");
  try {
    const exists = await client.collectionExists(collection);
    console.log(`Collection '${collection}' exists: ${exists}`);
  } catch (error) {
    console.log(`CollectionExists error: ${error}`);
  }

  // Example 2: Create some test documents
  console.log("\n=== Creating Test Documents ===");
  for (let i = 1; i <= 5; i++) {
    const record = {
      name: `Test Document ${i}`,
      index: i,
    };
    await client.insert(collection, record);
  }
  console.log("Created 5 test documents");

  // Example 3: Check if collection exists now (should be true)
  console.log("\n=== Check Collection Exists (After Creation) ===");
  try {
    const exists = await client.collectionExists(collection);
    console.log(`Collection '${collection}' exists: ${exists}`);
  } catch (error) {
    console.log(`CollectionExists error: ${error}`);
  }

  // Example 4: Count documents in collection
  console.log("\n=== Count Documents ===");
  try {
    const count = await client.countDocuments(collection);
    console.log(`Document count in '${collection}': ${count}`);
  } catch (error) {
    console.log(`CountDocuments error: ${error}`);
  }

  // Example 5: Check non-existent collection
  console.log("\n=== Check Non-Existent Collection ===");
  try {
    const exists = await client.collectionExists("nonexistent_collection_xyz");
    console.log(`Collection 'nonexistent_collection_xyz' exists: ${exists}`);
  } catch (error) {
    console.log(`CollectionExists error: ${error}`);
  }

  // Cleanup: Delete the test collection
  console.log("\n=== Cleanup ===");
  try {
    await client.deleteCollection(collection);
    console.log(`Deleted collection '${collection}'`);
  } catch (error) {
    console.log(`DeleteCollection error: ${error}`);
  }

  console.log("\n✓ Collection Utilities example complete");
}

main().catch((error) => {
  console.error("❌ Error:", error);
  process.exit(1);
});
