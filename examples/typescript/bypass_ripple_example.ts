/**
 * Example demonstrating bypass_ripple usage (TypeScript)
 *
 * The bypass_ripple parameter allows you to skip ripple propagation for specific operations,
 * which is useful in multi-node deployments when you want to prevent data from being
 * automatically replicated to other nodes.
 */

import { EkoDBClient } from "@ekodb/ekodb-client";
import * as dotenv from "dotenv";

dotenv.config();

async function main() {
  console.log("=== Bypass Ripple Example ===\n");

  const client = new EkoDBClient(
    process.env.API_BASE_URL || "http://localhost:8080",
    process.env.API_BASE_KEY || "a-test-api-key-from-ekodb",
  );

  const collection = "products";

  // Basic insert (no bypass)
  console.log("1. Basic insert (ripple enabled):");
  const record1 = { name: "Product 1", price: 100 };
  const result1 = await client.insert(collection, record1);
  console.log(`   Inserted with ripple: ${JSON.stringify(result1)}\n`);

  // Insert with bypass_ripple
  console.log("2. Insert with bypass_ripple:");
  const record2 = { name: "Product 2", price: 200 };
  const result2 = await client.insert(collection, record2, {
    bypassRipple: true,
  });
  console.log(`   Inserted with bypass_ripple: ${JSON.stringify(result2)}\n`);

  // Update with bypass_ripple
  console.log("3. Update with bypass_ripple:");
  const userId = result1.id as string;
  const updateRecord = { price: 150 };
  const result3 = await client.update(collection, userId, updateRecord, {
    bypassRipple: true,
  });
  console.log(`   Updated with bypass_ripple: ${JSON.stringify(result3)}\n`);

  // Delete with bypass_ripple
  console.log("4. Delete with bypass_ripple:");
  await client.delete(collection, userId, { bypassRipple: true });
  console.log("   Deleted with bypass_ripple\n");

  // Batch operations with bypass_ripple
  console.log("5. Batch insert with bypass_ripple:");
  const records = [
    { name: "Batch 1", price: 10 },
    { name: "Batch 2", price: 20 },
  ];
  const batchResults = await client.batchInsert(collection, records, {
    bypassRipple: true,
  });
  console.log(
    `   Batch inserted with bypass_ripple: ${batchResults.successful.length} records\n`,
  );

  // Upsert with bypass_ripple
  console.log("6. Upsert with bypass_ripple:");
  const upsertRecord = { name: "Upsert Product", price: 500 };
  const upsertResult = await client.upsert(
    collection,
    "custom-id",
    upsertRecord,
    { bypassRipple: true },
  );
  console.log(
    `   Upserted with bypass_ripple: ${JSON.stringify(upsertResult)}\n`,
  );

  // Cleanup
  await client.deleteCollection(collection);
  console.log("✅ All bypass_ripple operations completed successfully!");
}

main().catch(console.error);
