/**
 * ekoDB Convenience Methods Example (TypeScript)
 *
 * This example demonstrates the convenience methods added to the ekoDB TypeScript client:
 * - upsert: Insert or update a record
 * - findOne: Find a single record by field value
 * - exists: Check if a record exists
 * - paginate: Paginate through records
 */

import { EkoDBClient } from "@ekodb/ekodb-client";
import * as dotenv from "dotenv";

dotenv.config();

async function main() {
  console.log("=== ekoDB Convenience Methods Example ===\n");

  // Create client
  const BASE_URL = process.env.API_BASE_URL || "http://localhost:8080";
  const API_KEY = process.env.API_BASE_KEY || "a-test-api-key-from-ekodb";
  const client = new EkoDBClient(BASE_URL, API_KEY);

  await client.init();

  const collection = "convenience_methods_example_ts";

  console.log("=== Native Object Creation ===");
  // TypeScript uses plain objects natively - no builder needed!
  const user1 = {
    name: "Alice Johnson",
    email: "alice@example.com",
    age: 28,
    active: true,
  };

  const inserted = await client.insert(collection, user1);
  console.log("✓ Created record with plain object:", inserted);

  console.log("\n=== Upsert Operation ===");
  const userId = inserted.id as string;

  // First upsert - will update (record exists)
  const user2 = {
    name: "Alice Johnson",
    email: "alice.j@newdomain.com",
    age: 29,
  };

  const upserted1 = await client.upsert(collection, userId, user2);
  console.log("✓ First upsert (update):", upserted1);

  // Second upsert - will insert (different ID, doesn't exist)
  const user3 = {
    name: "Bob Smith",
    email: "bob@example.com",
    age: 35,
  };

  const upserted2 = await client.upsert(collection, "new-user-id", user3);
  console.log("✓ Second upsert (insert):", upserted2);

  console.log("\n=== Find One Operation ===");
  // Find a single record by any field
  const foundUser = await client.findOne(
    collection,
    "email",
    "alice.j@newdomain.com",
  );
  if (foundUser) {
    console.log("✓ Found user by email:", foundUser);
  }

  // Try finding a non-existent user
  const notFound = await client.findOne(
    collection,
    "email",
    "nonexistent@example.com",
  );
  if (!notFound) {
    console.log("✓ User not found (as expected)");
  }

  console.log("\n=== Exists Check ===");
  // Check existence without fetching the record
  const exists = await client.exists(collection, userId);
  console.log(`✓ Record exists: ${exists}`);

  // Check non-existent record
  const notExists = await client.exists(collection, "fake-id-123");
  console.log(`✓ Fake record exists: ${notExists} (should be false)`);

  console.log("\n=== Pagination ===");
  // Insert more records for pagination demo
  const insertPromises = [];
  for (let i = 1; i <= 25; i++) {
    insertPromises.push(
      client.insert(collection, {
        name: `User ${i}`,
        email: `user${i}@example.com`,
        index: i,
      }),
    );
  }
  await Promise.all(insertPromises);
  console.log("✓ Inserted 25 records for pagination");

  // Paginate through records
  const pageSize = 10;
  for (let page = 1; page <= 3; page++) {
    const records = await client.paginate(collection, page, pageSize);
    const expected = page === 3 ? "~7" : "10";
    console.log(
      `✓ Page ${page}: ${records.length} records (expected ${expected})`,
    );
  }

  console.log("\n=== Cleanup ===");
  await client.deleteCollection(collection);
  console.log("✓ Deleted collection");

  console.log("\n✅ All convenience methods demonstrated successfully!");
}

main().catch((error) => {
  console.error("Error:", error);
  process.exit(1);
});
