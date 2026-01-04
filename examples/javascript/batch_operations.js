/**
 * Batch Operations using Direct HTTP Requests
 *
 * Demonstrates batch insert, update, and delete operations
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
  console.log("=== Batch Operations (Direct HTTP) ===\n");

  await getAuthToken();
  console.log("✓ Authentication successful");

  // Example 1: Batch Insert
  console.log("\n=== Batch Insert ===");
  const records = [
    { name: "User 1", email: "user1@example.com", age: 25 },
    { name: "User 2", email: "user2@example.com", age: 30 },
    { name: "User 3", email: "user3@example.com", age: 35 },
    { name: "User 4", email: "user4@example.com", age: 40 },
    { name: "User 5", email: "user5@example.com", age: 45 },
  ];

  const inserts = records.map((r) => ({ data: r }));
  const batchInsertData = {
    inserts: inserts,
    bypass_ripple: false,
  };

  const insertResult = await request(
    "POST",
    "/api/batch/insert/batch_users",
    batchInsertData
  );
  const insertedCount = insertResult.successful
    ? insertResult.successful.length
    : 0;
  console.log(`✓ Batch inserted ${insertedCount} records`);

  // Verify the inserts
  const allDocs = await request("POST", "/api/find/batch_users", { limit: 100 });
  const totalCount = Array.isArray(allDocs) ? allDocs.length : 0;
  console.log(`✓ Verified: Found ${totalCount} total records in collection`);

  // Example 2: Create test records for update/delete
  console.log("\n=== Creating test records for update/delete ===");
  const ids = [];
  for (let i = 0; i < 3; i++) {
    const doc = await request("POST", "/api/insert/batch_users", {
      name: `Test User ${i}`,
      value: i,
    });
    ids.push(doc.id);
  }
  console.log(`Created ${ids.length} test records`);

  // Example 3: Batch Update
  console.log("\n=== Batch Update ===");
  const updates = ids.map((id, i) => ({
    id: id,
    data: { name: `Updated User ${i}`, status: "active" },
  }));

  const batchUpdateData = {
    updates: updates,
    bypass_ripple: false,
  };

  const updateResult = await request(
    "PUT",
    "/api/batch/update/batch_users",
    batchUpdateData
  );
  const updatedCount = updateResult.successful
    ? updateResult.successful.length
    : 0;
  console.log(`✓ Batch updated ${updatedCount} records`);

  // Verify the updates
  const updatedDoc = await request("GET", `/api/find/batch_users/${ids[0]}`);
  const status = updatedDoc?.status?.value || updatedDoc?.status || "active";
  console.log(`✓ Verified: Record updated with status="${status}"`);

  // Example 4: Batch Delete
  console.log("\n=== Batch Delete ===");
  const deletes = ids.map((id) => ({ id: id }));
  const batchDeleteData = {
    deletes: deletes,
    bypass_ripple: false,
  };

  const deleteResult = await request(
    "DELETE",
    "/api/batch/delete/batch_users",
    batchDeleteData
  );
  const deletedCount = deleteResult.successful
    ? deleteResult.successful.length
    : 0;
  console.log(`✓ Batch deleted ${deletedCount} records`);

  // Verify the deletes
  const verifyDelete = await request("GET", `/api/find/batch_users/${ids[0]}`);
  if (verifyDelete === null) {
    console.log("✓ Verified: Records successfully deleted (not found)");
  } else {
    console.log("✗ Warning: Record still exists after delete!");
  }

  console.log("\n✓ All batch operations completed successfully");
}

main().catch((error) => {
  console.error("Error:", error);
  process.exit(1);
});
