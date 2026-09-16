/**
 * Key-Value Operations - Using @ekodb/ekodb-client library
 */

import { EkoDBClient } from "@ekodb/ekodb-client";
import * as dotenv from "dotenv";

dotenv.config();

const BASE_URL = process.env.API_BASE_URL || "http://localhost:8080";
const API_KEY = process.env.API_BASE_KEY || "a-test-api-key-from-ekodb";

async function main() {
  const client = new EkoDBClient(BASE_URL, API_KEY);
  await client.init();
  const prefix = `kv_ops_ts_${process.pid}_${Date.now()}`;
  const sessionKey = `${prefix}:session:user123`;
  const keys = [1, 2, 3].map((id) => `${prefix}:cache:product:${id}`);
  const allOwnedKeys = [sessionKey, ...keys];
  let primaryError: unknown;

  try {
    console.log("✓ Client created");

    console.log("\n=== KV Set ===");
    await client.kvSet(sessionKey, { userId: 123, username: "john_doe" });
    console.log("✓ Set key: session:user123");

    console.log("\n=== KV Get ===");
    const value = await client.kvGet(sessionKey);
    console.log("Retrieved value:", value);

    console.log("\n=== KV Batch Set ===");
    const batchEntries = [
      { key: keys[0], value: { name: "Product 1", price: 29.99 } },
      { key: keys[1], value: { name: "Product 2", price: 39.99 } },
      { key: keys[2], value: { name: "Product 3", price: 49.99 } },
    ];
    const setResults = await client.kvBatchSet(batchEntries);
    console.log(`✓ Batch set ${setResults.length} keys`);
    setResults.forEach(([key, wasSet]) => {
      console.log(`  ${key}: ${wasSet ? "success" : "failed"}`);
    });

    console.log("\n=== KV Batch Get ===");
    const batchValues = await client.kvBatchGet(keys);
    console.log(`✓ Batch retrieved ${batchValues.length} values`);
    batchValues.forEach((val, i) => {
      console.log(`  ${keys[i]}:`, val);
    });

    console.log("\n=== KV Exists ===");
    const exists = await client.kvExists(sessionKey);
    console.log(`Key exists: ${exists}`);

    console.log("\n=== KV Find (Pattern Query) ===");
    const cacheResults = await client.kvFind({
      pattern: `${prefix}:cache:product:.*`,
    });
    console.log(
      `Found ${cacheResults.length} keys matching 'cache:product:.*'`,
    );

    console.log("\n=== KV Query (Alias for Find) ===");
    const allResults = await client.kvQuery({});
    console.log(`Total keys in store: ${allResults.length}`);

    console.log("\n=== KV Delete ===");
    await client.kvDelete(sessionKey);
    console.log("✓ Deleted key: session:user123");

    // Verify deletion with kvExists
    const existsAfterDelete = await client.kvExists(sessionKey);
    console.log(`✓ Verified: Key exists after delete: ${existsAfterDelete}`);

    console.log("\n=== KV Batch Delete ===");
    const deleteResults = await client.kvBatchDelete(keys);
    console.log(`✓ Batch deleted ${deleteResults.length} keys`);
    deleteResults.forEach(([key, wasDeleted]) => {
      console.log(`  ${key}: ${wasDeleted ? "deleted" : "not found"}`);
    });

    console.log("\n✓ All KV operations completed successfully");
  } catch (error) {
    primaryError = error;
    throw error;
  } finally {
    const cleanupErrors: unknown[] = [];
    for (const key of allOwnedKeys) {
      try {
        await client.kvDelete(key);
      } catch (error) {
        const message = error instanceof Error ? error.message : String(error);
        if (!message.includes("status 404") && !/not found/i.test(message)) {
          cleanupErrors.push(error);
        }
      }
    }
    if (cleanupErrors.length > 0) {
      throw new AggregateError(
        primaryError === undefined
          ? cleanupErrors
          : [primaryError, ...cleanupErrors],
        "KV operations example or cleanup failed",
      );
    }
  }
}

main().catch((error) => {
  console.error("❌ Error:", error);
  process.exit(1);
});
