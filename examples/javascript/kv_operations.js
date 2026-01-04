/**
 * Key-Value Operations using Direct HTTP Requests
 *
 * Demonstrates KV set, get, and delete operations
 * using raw HTTP/fetch API - no client library required
 */

const dotenv = require("dotenv");

dotenv.config();

const BASE_URL = process.env.API_BASE_URL || "http://localhost:8080";
const API_KEY = process.env.API_BASE_KEY || "a-test-api-key-from-ekodb";

let authToken = null;

async function getAuthToken() {
  if (authToken) return authToken;

  const response = await fetch(`${BASE_URL}/api/auth/token`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ api_key: API_KEY }),
  });

  if (!response.ok) {
    throw new Error(`Auth failed: ${response.status}`);
  }

  const data = await response.json();
  authToken = data.token;
  return authToken;
}

async function request(method, path, body = null) {
  const token = await getAuthToken();
  const options = {
    method,
    headers: {
      Authorization: `Bearer ${token}`,
      "Content-Type": "application/json",
    },
  };

  if (body) {
    options.body = JSON.stringify(body);
  }

  const response = await fetch(`${BASE_URL}${path}`, options);

  if (!response.ok && response.status !== 404) {
    throw new Error(`HTTP ${response.status}: ${await response.text()}`);
  }

  if (response.status === 404) {
    return null;
  }

  const text = await response.text();
  if (!text) return {};
  return JSON.parse(text);
}

async function main() {
  console.log("=== Key-Value Operations (Direct HTTP) ===\n");

  await getAuthToken();
  console.log("✓ Authentication successful");

  // Example 1: Set a key-value pair
  console.log("\n=== KV Set ===");
  await request("POST", "/api/kv/set/session:user123", {
    value: {
      userId: 123,
      username: "john_doe",
    },
  });
  console.log("✓ Set key: session:user123");

  // Example 2: Get a key-value pair
  console.log("\n=== KV Get ===");
  const getValue = await request("GET", "/api/kv/get/session:user123");
  console.log("Retrieved value:", getValue?.value);

  // Example 3: Set multiple keys
  console.log("\n=== Set Multiple Keys ===");
  const keys = ["cache:product:1", "cache:product:2", "cache:product:3"];

  for (let i = 0; i < keys.length; i++) {
    await request("POST", `/api/kv/set/${keys[i]}`, {
      value: {
        name: `Product ${i + 1}`,
        price: 29.99 + i * 10.0,
      },
    });
  }
  console.log(`✓ Set ${keys.length} keys`);

  // Example 4: Get multiple keys
  console.log("\n=== Get Multiple Keys ===");
  for (const key of keys) {
    const result = await request("GET", `/api/kv/get/${key}`);
    console.log(`${key}:`, result?.value);
  }

  // Example 5: Delete a key
  console.log("\n=== KV Delete ===");
  await request("DELETE", "/api/kv/delete/session:user123");
  console.log("✓ Deleted key: session:user123");

  // Verify deletion
  const verifyDelete = await request("GET", "/api/kv/get/session:user123");
  if (verifyDelete === null) {
    console.log("✓ Verified: Key successfully deleted (not found)");
  } else {
    console.log("✗ Warning: Key still exists after delete!");
  }

  // Example 6: Delete multiple keys
  console.log("\n=== Delete Multiple Keys ===");
  for (const key of keys) {
    await request("DELETE", `/api/kv/delete/${key}`);
  }
  console.log(`✓ Deleted ${keys.length} keys`);

  console.log("\n✓ All KV operations completed successfully");
}

main().catch((error) => {
  console.error("Error:", error);
  process.exit(1);
});
