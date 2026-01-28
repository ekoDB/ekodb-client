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

  // Example 1: Select specific fields - only get id, name, email
  console.log("Example 1: Select specific fields (id, name, email only)");
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
  console.log(`  Found ${users.length} active users`);
  if (users.length > 0) {
    const fields = Object.keys(users[0]);
    console.log(`  Fields returned: ${JSON.stringify(fields)}`);
    console.log(`  First user: ${users[0].name} <${users[0].email}>`);
  }

  // Example 2: Exclude sensitive fields - hide password, api_key, secret_token
  console.log("\nExample 2: Exclude sensitive fields (password, api_key, secret_token)");
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
  console.log(`  Found ${admins.length} admins`);
  if (admins.length > 0) {
    const hasPassword = "password" in admins[0];
    const hasApiKey = "api_key" in admins[0];
    const hasToken = "secret_token" in admins[0];
    console.log("  Sensitive fields excluded:");
    console.log(`    - password: ${hasPassword ? "PRESENT (unexpected!)" : "excluded"}`);
    console.log(`    - api_key: ${hasApiKey ? "PRESENT (unexpected!)" : "excluded"}`);
    console.log(`    - secret_token: ${hasToken ? "PRESENT (unexpected!)" : "excluded"}`);
    const fields = Object.keys(admins[0]);
    console.log(`  Fields returned: ${JSON.stringify(fields)}`);
  }

  // Example 3: Complex query with projection - active users with profile fields
  console.log("\nExample 3: Complex query with projection (active users, ages 18-65)");
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
    sort: [{ field: "age", ascending: false }],
    limit: 50,
  });
  console.log(`  Found ${activeUsers.length} active users (ages 18-65)`);
  for (const user of activeUsers) {
    console.log(`    - ${user.name} (age ${user.age})`);
  }

  // Example 4: Query inactive users with profile fields
  console.log("\nExample 4: Query inactive users with profile fields");
  const inactiveUsers = await request("POST", `/api/find/${TEST_COLLECTION}`, {
    filter: {
      type: "Condition",
      content: {
        field: "status",
        operator: "Eq",
        value: "inactive",
      },
    },
    select_fields: ["id", "name", "email", "bio"],
  });
  console.log(`  Found ${inactiveUsers.length} inactive users`);
  for (const user of inactiveUsers) {
    console.log(`    - ${user.name}: ${user.bio}`);
  }

  // Example 5: Compare full vs projected data - demonstrates bandwidth savings
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

  if (fullUsers && fullUsers.length > 0 && projectedUsers && projectedUsers.length > 0) {
    const fullFields = Object.keys(fullUsers[0]);
    const projectedFields = Object.keys(projectedUsers[0]);

    console.log("  Full query:");
    console.log(`    - ${fullFields.length} fields per record`);
    console.log(`    - Fields: ${JSON.stringify(fullFields)}`);
    console.log("  Projected query:");
    console.log(`    - ${projectedFields.length} fields per record`);
    console.log(`    - Fields: ${JSON.stringify(projectedFields)}`);
    const savings = 100 - Math.floor((projectedFields.length * 100) / Math.max(fullFields.length, 1));
    console.log(`  Bandwidth savings: ~${savings}% fewer fields`);
  }

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
