/**
 * Field Projection Examples using Direct HTTP Requests
 *
 * Demonstrates how to use select_fields and exclude_fields to optimize
 * data transfer and improve query performance.
 *
 * Shows comparison between full object queries and projected queries.
 */

const dotenv = require("dotenv");

dotenv.config();

const BASE_URL = process.env.API_BASE_URL || "http://localhost:8080";
const API_KEY = process.env.API_BASE_KEY || "a-test-api-key-from-ekodb";

const TEST_COLLECTION = "projection_test_users_js";

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
  console.log("=== Field Projection Examples (Direct HTTP) ===\n");

  await getAuthToken();
  console.log("Client created\n");

  // Setup: Create test collection and insert test data
  console.log("Setting up test data...");
  // Try to delete existing collection (ignore error if doesn't exist)
  try {
    await request("DELETE", `/api/collections/${TEST_COLLECTION}`);
  } catch {
    // Collection may not exist, that's fine
  }

  // Insert test users with various fields
  const testUsers = [
    {
      name: "Alice Johnson",
      email: "alice@example.com",
      status: "active",
      user_role: "admin",
      age: 30,
      bio: "Software engineer",
      avatar_url: "https://example.com/alice.jpg",
      password: "secret123",
      api_key: "key123",
      secret_token: "token123",
      created_at: new Date().toISOString(),
    },
    {
      name: "Bob Smith",
      email: "bob@example.com",
      status: "active",
      user_role: "user",
      age: 25,
      bio: "Designer",
      avatar_url: "https://example.com/bob.jpg",
      password: "secret456",
      api_key: "key456",
      secret_token: "token456",
      created_at: new Date().toISOString(),
    },
    {
      name: "Carol White",
      email: "carol@example.com",
      status: "inactive",
      user_role: "user",
      age: 35,
      bio: "Manager",
      avatar_url: "https://example.com/carol.jpg",
      password: "secret789",
      api_key: "key789",
      secret_token: "token789",
      created_at: new Date().toISOString(),
    },
    {
      name: "Dave Brown",
      email: "dave@example.com",
      status: "active",
      user_role: "admin",
      age: 45,
      bio: "CTO",
      avatar_url: "https://example.com/dave.jpg",
      password: "secretabc",
      api_key: "keyabc",
      secret_token: "tokenabc",
      created_at: new Date().toISOString(),
    },
  ];

  const insertedIds = [];
  for (const user of testUsers) {
    const result = await request("POST", `/api/insert/${TEST_COLLECTION}`, user);
    insertedIds.push(result.id);
  }
  console.log(`Inserted ${insertedIds.length} test users\n`);

  // Example 1: Select specific fields to reduce data transfer
  console.log("Example 1: Select specific fields");
  const users = await request("POST", `/api/find/${TEST_COLLECTION}`, {
    filter: {
      type: "Condition",
      content: {
        field: "status",
        operator: "Eq",
        value: "active",
      },
    },
    select_fields: ["id", "name", "email"],
    limit: 10,
  });
  console.log(`Fetched ${users.length} users with only 3 fields each`);

  // Example 2: Exclude sensitive fields
  console.log("\nExample 2: Exclude sensitive fields");
  const admins = await request("POST", `/api/find/${TEST_COLLECTION}`, {
    filter: {
      type: "Condition",
      content: {
        field: "user_role",
        operator: "Eq",
        value: "admin",
      },
    },
    exclude_fields: ["password", "api_key", "secret_token"],
  });
  console.log(`Fetched ${admins.length} admins without sensitive data`);
  // Verify sensitive fields are excluded
  if (admins.length > 0) {
    const hasPassword = "password" in admins[0];
    console.log(`  Password field excluded: ${!hasPassword}`);
  }

  // Example 3: Complex query with projection
  console.log("\nExample 3: Complex query with projection");
  const activeUsers = await request("POST", `/api/find/${TEST_COLLECTION}`, {
    filter: {
      type: "Logical",
      content: {
        operator: "And",
        expressions: [
          {
            type: "Condition",
            content: { field: "status", operator: "Eq", value: "active" },
          },
          {
            type: "Condition",
            content: { field: "age", operator: "Gte", value: 18 },
          },
          {
            type: "Condition",
            content: { field: "age", operator: "Lt", value: 65 },
          },
        ],
      },
    },
    select_fields: ["id", "name", "email", "age", "created_at"],
    sort: [{ field: "created_at", ascending: false }],
    limit: 50,
  });
  console.log(`Fetched ${activeUsers.length} active users with profile fields`);

  // Example 4: Find by ID with projection
  console.log("\nExample 4: Find by ID with projection");
  const userProfiles = await request("POST", `/api/find/${TEST_COLLECTION}`, {
    filter: {
      type: "Condition",
      content: {
        field: "id",
        operator: "Eq",
        value: insertedIds[0],
      },
    },
    select_fields: ["id", "name", "email", "bio", "avatar_url"],
  });
  const userProfile = userProfiles && userProfiles.length > 0 ? userProfiles[0] : null;
  console.log(`Fetched user profile: ${userProfile?.name || "N/A"}`);

  // Example 5: Compare full vs projected data
  console.log("\nExample 5: Compare full vs projected data");
  const fullUsers = await request("POST", `/api/find/${TEST_COLLECTION}`, {
    filter: {
      type: "Condition",
      content: { field: "status", operator: "Eq", value: "active" },
    },
  });
  const projectedUsers = await request("POST", `/api/find/${TEST_COLLECTION}`, {
    filter: {
      type: "Condition",
      content: { field: "status", operator: "Eq", value: "active" },
    },
    select_fields: ["id", "name", "email"],
  });

  const fullFields = fullUsers && fullUsers.length > 0 ? Object.keys(fullUsers[0]).length : 0;
  const projectedFields =
    projectedUsers && projectedUsers.length > 0 ? Object.keys(projectedUsers[0]).length : 0;

  console.log(`Full query returned ${fullFields} fields per user`);
  console.log(`Projected query returned ${projectedFields} fields per user`);

  // Cleanup
  console.log("\nCleaning up test data...");
  await request("DELETE", `/api/collections/${TEST_COLLECTION}`);
  console.log("Cleanup complete");

  console.log("\nAll projection examples completed successfully!");
}

main().catch((error) => {
  console.error("Error:", error);
  process.exit(1);
});
