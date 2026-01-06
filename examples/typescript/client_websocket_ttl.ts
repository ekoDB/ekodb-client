/**
 * WebSocket TTL - Using @ekodb/ekodb-client library
 */

import { EkoDBClient } from "@ekodb/ekodb-client";
import * as dotenv from "dotenv";

dotenv.config();

const BASE_URL = process.env.API_BASE_URL || "http://localhost:8080";
const WS_URL = process.env.WS_BASE_URL || "ws://localhost:8080";
const API_KEY = process.env.API_BASE_KEY || "a-test-api-key-from-ekodb";

async function main() {
  const client = new EkoDBClient(BASE_URL, API_KEY);
  await client.init();
  console.log("✓ Client created");

  const collection = "client_websocket_ttl_ts";

  console.log("\n=== Insert Test Data with TTL ===");
  const record = { name: "WebSocket TTL Test", value: 42, active: true };
  const inserted = await client.insert(collection, record, { ttl: "1h" });
  console.log(`✓ Inserted document with TTL: ${inserted.id}`);

  console.log("\n=== Query via WebSocket ===");
  const ws = client.websocket(WS_URL);
  console.log("✓ WebSocket connected");

  const records = await ws.findAll(collection);
  console.log(`✓ Retrieved ${records.length} record(s) via WebSocket`);

  for (let i = 0; i < Math.min(3, records.length); i++) {
    const fieldCount = Object.keys(records[i]).length;
    console.log(`  Record ${i + 1}: ${fieldCount} fields`);
  }

  ws.close();

  // Cleanup: Delete the collection
  console.log("\n=== Cleanup ===");
  await client.deleteCollection(collection);
  console.log("✓ Deleted collection");

  console.log("\n✓ WebSocket TTL example completed successfully");
  console.log(
    "\n💡 Note: Documents with TTL will automatically expire after the specified duration",
  );
}

main().catch((error) => {
  console.error("❌ Error:", error);
  process.exit(1);
});
