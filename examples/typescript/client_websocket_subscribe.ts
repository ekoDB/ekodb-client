/**
 * WebSocket Subscription Example - Real-time mutation notifications
 *
 * Demonstrates subscribing to collection changes via WebSocket and
 * receiving push notifications when records are inserted, updated, or deleted.
 *
 * Requires: ekoDB server running on localhost:8080
 */

import WebSocket from "ws";
import * as dotenv from "dotenv";

dotenv.config();

const BASE_URL = process.env.API_BASE_URL || "http://localhost:8080";
const WS_URL = process.env.WS_BASE_URL || "ws://localhost:8080";
const API_KEY = process.env.API_BASE_KEY || "a-test-api-key-from-ekodb";

async function getAuthToken(): Promise<string> {
  const res = await fetch(`${BASE_URL}/api/auth/token`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ api_key: API_KEY }),
  });
  const data = (await res.json()) as { token: string };
  return data.token;
}

async function insertRecord(
  token: string,
  collection: string,
  record: object,
): Promise<any> {
  const res = await fetch(`${BASE_URL}/api/insert/${collection}`, {
    method: "POST",
    headers: {
      Authorization: `Bearer ${token}`,
      "Content-Type": "application/json",
    },
    body: JSON.stringify(record),
  });
  return res.json();
}

async function main() {
  console.log("=== WebSocket Subscription Example ===\n");

  const collection = "ws_subscribe_example_ts";

  // Step 1: Authenticate
  const token = await getAuthToken();
  console.log("✓ Authentication successful");

  // Step 2: Connect to WebSocket
  const ws = new WebSocket(`${WS_URL}/api/ws`, {
    headers: { Authorization: `Bearer ${token}` },
  });

  await new Promise<void>((resolve, reject) => {
    ws.on("open", resolve);
    ws.on("error", reject);
  });
  console.log("✓ WebSocket connected");

  // Step 3: Subscribe to collection
  console.log(`\n=== Subscribing to '${collection}' ===`);

  const subscribePromise = new Promise<any>((resolve) => {
    ws.once("message", (data: Buffer) => {
      resolve(JSON.parse(data.toString()));
    });
  });

  ws.send(
    JSON.stringify({
      type: "Subscribe",
      payload: { collection },
    }),
  );

  const subResponse = await subscribePromise;
  console.log(
    `✓ Subscribed (subscription_id: ${subResponse.payload.data.subscription_id})`,
  );

  // Step 4: Listen for notifications in the background
  console.log("\n=== Listening for mutation notifications ===");

  ws.on("message", (data: Buffer) => {
    const msg = JSON.parse(data.toString());
    if (msg.type === "MutationNotification") {
      console.log(`\n  📡 Notification received:`);
      console.log(`     Event:      ${msg.payload.event}`);
      console.log(`     Collection: ${msg.payload.collection}`);
      console.log(`     Record IDs: ${msg.payload.record_ids.join(", ")}`);
      console.log(`     Timestamp:  ${msg.payload.timestamp}`);
      if (msg.payload.records) {
        console.log(
          `     Records:    ${JSON.stringify(msg.payload.records).slice(0, 100)}...`,
        );
      }
    }
  });

  // Step 5: Perform mutations via REST API to trigger notifications
  console.log("\nInserting a record...");
  const insertResult = await insertRecord(token, collection, {
    name: "Alice",
    role: "engineer",
    active: true,
  });
  console.log(`✓ Inserted record: ${insertResult.id}`);

  // Give time for notification to arrive
  await new Promise((r) => setTimeout(r, 1000));

  console.log("\nInserting another record...");
  await insertRecord(token, collection, {
    name: "Bob",
    role: "designer",
    active: true,
  });

  await new Promise((r) => setTimeout(r, 1000));

  // Step 6: Unsubscribe
  console.log("\n=== Unsubscribing ===");

  const unsubPromise = new Promise<any>((resolve, reject) => {
    const timeout = setTimeout(() => {
      ws.removeListener("message", handler);
      reject(new Error("Unsubscribe timed out after 5 seconds"));
    }, 5000);

    function handler(data: Buffer) {
      const msg = JSON.parse(data.toString());
      if (msg.type === "Success") {
        clearTimeout(timeout);
        ws.removeListener("message", handler);
        resolve(msg);
      }
    }

    ws.on("message", handler);
  });

  ws.send(
    JSON.stringify({
      type: "Unsubscribe",
      payload: { collection },
    }),
  );

  const unsubResponse = await unsubPromise;
  console.log(`✓ Unsubscribed: ${JSON.stringify(unsubResponse.payload.data)}`);

  // Step 7: Cleanup
  ws.close();
  console.log("\n✓ WebSocket subscription example completed successfully");
}

main().catch((error) => {
  console.error("❌ Error:", error);
  process.exit(1);
});
