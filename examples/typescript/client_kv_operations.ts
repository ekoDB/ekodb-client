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
  console.log("✓ Client created");

  console.log("\n=== KV Set ===");
  await client.kvSet("session:user123", { userId: 123, username: "john_doe" });
  console.log("✓ Set key: session:user123");

  console.log("\n=== KV Get ===");
  const value = await client.kvGet("session:user123");
  console.log("Retrieved value:", value);

  console.log("\n=== Set Multiple Keys ===");
  const keys = ["cache:product:1", "cache:product:2", "cache:product:3"];
  for (let i = 0; i < keys.length; i++) {
    await client.kvSet(keys[i], {
      name: `Product ${i + 1}`,
      price: 29.99 + i * 10,
    });
  }
  console.log(`✓ Set ${keys.length} keys`);

  console.log("\n=== Get Multiple Keys ===");
  for (const key of keys) {
    const val = await client.kvGet(key);
    console.log(`${key}:`, val);
  }

  console.log("\n=== KV Delete ===");
  await client.kvDelete("session:user123");
  console.log("✓ Deleted key: session:user123");

  try {
    await client.kvGet("session:user123");
  } catch {
    console.log("✓ Verified: Key successfully deleted (not found)");
  }

  console.log("\n=== Delete Multiple Keys ===");
  for (const key of keys) {
    await client.kvDelete(key);
  }
  console.log(`✓ Deleted ${keys.length} keys`);

  console.log("\n✓ All KV operations completed successfully");
}

main().catch((error) => {
  console.error("❌ Error:", error);
  process.exit(1);
});
