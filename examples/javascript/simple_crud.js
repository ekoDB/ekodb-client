/**
 * Simple CRUD Operations using Direct HTTP Requests
 *
 * Demonstrates basic Create, Read, Update, Delete operations
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

  return response.json();
}

async function main() {
  console.log("=== Simple CRUD Operations (Direct HTTP) ===\n");

  await getAuthToken();
  console.log("✓ Authentication successful");

  // Example 1: Insert a document
  console.log("\n=== Insert Document ===");
  const doc = await request("POST", "/api/insert/test_collection", {
    name: "Test Record",
    value: 42,
    active: true,
  });
  console.log("Inserted:", doc);
  const docId = doc.id;

  // Example 2: Find by ID
  console.log("\n=== Find by ID ===");
  const foundDoc = await request("GET", `/api/find/test_collection/${docId}`);
  console.log("Found:", foundDoc);

  // Example 3: Find with query
  console.log("\n=== Find with Query ===");
  const docs = await request("POST", "/api/find/test_collection", {
    filter: {
      type: "Condition",
      content: {
        field: "active",
        operator: "Eq",
        value: true,
      },
    },
    limit: 10,
  });
  console.log("Found documents:", docs);

  // Example 4: Update document
  console.log("\n=== Update Document ===");
  const updated = await request("PUT", `/api/update/test_collection/${docId}`, {
    name: "Updated Record",
    value: 100,
  });
  console.log("Updated:", updated);

  // Example 5: Delete document
  console.log("\n=== Delete Document ===");
  await request("DELETE", `/api/delete/test_collection/${docId}`);
  console.log("Deleted document");

  console.log("\n✓ All CRUD operations completed successfully");
}

main().catch((error) => {
  console.error("Error:", error);
  process.exit(1);
});
