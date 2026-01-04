/**
 * Simple WebSocket Operations using Direct API
 *
 * Demonstrates WebSocket connection and querying
 * using raw WebSocket API - no client library required
 */

const dotenv = require("dotenv");
const WebSocket = require("ws");

dotenv.config();

const BASE_URL = process.env.API_BASE_URL || "http://localhost:8080";
const WS_URL = process.env.WS_BASE_URL || "ws://localhost:8080";
const API_KEY = process.env.API_BASE_KEY || "a-test-api-key-from-ekodb";

async function getAuthToken() {
  const response = await fetch(`${BASE_URL}/api/auth/token`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ api_key: API_KEY }),
  });

  if (!response.ok) {
    throw new Error(`Auth failed: ${response.status}`);
  }

  const data = await response.json();
  return data.token;
}

async function insertTestData(token) {
  const response = await fetch(`${BASE_URL}/api/insert/websocket_test`, {
    method: "POST",
    headers: {
      Authorization: `Bearer ${token}`,
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      name: "WebSocket Test Record",
      value: 42,
      active: true,
    }),
  });

  if (!response.ok) {
    throw new Error(`Insert failed: ${response.status}`);
  }

  return response.json();
}

async function main() {
  console.log("=== Simple WebSocket Operations (Direct API) ===\n");

  // Step 1: Get authentication token
  const token = await getAuthToken();
  console.log("✓ Authentication successful");

  // Step 2: Insert test data first
  console.log("\n=== Inserting Test Data ===");
  const insertResult = await insertTestData(token);
  console.log(`✓ Inserted test record: ${insertResult.id}`);

  // Step 3: Connect to WebSocket
  console.log("\n=== Connecting to WebSocket ===");

  return new Promise((resolve, reject) => {
    const ws = new WebSocket(`${WS_URL}/api/ws`, {
      headers: {
        Authorization: `Bearer ${token}`,
      },
    });

    ws.on("open", () => {
      console.log("✓ WebSocket connected");

      // Step 4: Query data via WebSocket
      console.log("\n=== Querying Data via WebSocket ===");
      const messageId = Date.now().toString();

      ws.send(
        JSON.stringify({
          type: "FindAll",
          messageId: messageId,
          payload: {
            collection: "websocket_test",
          },
        })
      );
    });

    ws.on("message", (data) => {
      try {
        const response = JSON.parse(data.toString());
        console.log("Response:", JSON.stringify(response, null, 2));

        if (response.payload && response.payload.data) {
          const records = response.payload.data;
          console.log(`✓ Retrieved ${records.length} record(s) via WebSocket`);
        }

        console.log("\n✓ WebSocket example completed successfully");
        ws.close();
        resolve();
      } catch (error) {
        console.error("Failed to parse response:", error);
        ws.close();
        reject(error);
      }
    });

    ws.on("error", (error) => {
      console.error("WebSocket error:", error);
      reject(error);
    });

    ws.on("close", () => {
      console.log("WebSocket closed");
    });

    // Timeout after 10 seconds
    setTimeout(() => {
      ws.close();
      reject(new Error("WebSocket timeout"));
    }, 10000);
  });
}

main().catch((error) => {
  console.error("Error:", error);
  process.exit(1);
});
