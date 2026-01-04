/**
 * Collection Management using Direct HTTP Requests
 *
 * Demonstrates collection creation, listing, and deletion
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
  console.log("=== Collection Management (Direct HTTP) ===\n");

  await getAuthToken();
  console.log("✓ Authentication successful");

  // Example 1: Create a collection (via insert)
  console.log("\n=== Create Collection (via insert) ===");
  const insertResult = await request("POST", "/api/insert/demo_collection", {
    name: "Demo Collection",
    description: "Created for testing",
    active: true,
  });
  console.log(`Collection created with first record: ${insertResult.id}`);

  // Example 2: List all collections
  console.log("\n=== List Collections ===");
  const collectionsData = await request("GET", "/api/collections");
  const collections = collectionsData.collections || [];
  console.log(`Total collections: ${collections.length}`);
  console.log("Sample collections:", collections.slice(0, 5));

  // Example 3: Count documents in collection
  console.log("\n=== Count Documents ===");
  const docs = await request("POST", "/api/find/demo_collection", {
    limit: 1000,
  });
  const docCount = Array.isArray(docs) ? docs.length : 0;
  console.log(`Document count: ${docCount}`);

  // Example 4: Delete collection
  console.log("\n=== Delete Collection ===");
  await request("DELETE", "/api/collections/demo_collection");
  console.log("Collection deleted successfully");

  // Example 5: Verify deletion
  console.log("\n=== Verify Deletion ===");
  const updatedCollectionsData = await request("GET", "/api/collections");
  const updatedCollections = updatedCollectionsData.collections || [];
  const stillExists = updatedCollections.includes("demo_collection");
  console.log(`Collection still exists: ${stillExists}`);

  console.log("\n✓ All collection management operations completed successfully");
}

main().catch((error) => {
  console.error("Error:", error);
  process.exit(1);
});
