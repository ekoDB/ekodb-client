import { EkoDBClient, QueryBuilder } from "@ekodb/ekodb-client";
import * as dotenv from "dotenv";

dotenv.config();

const BASE_URL = process.env.API_BASE_URL || "http://localhost:8080";
const API_KEY = process.env.API_BASE_KEY || "a-test-api-key-from-ekodb";

const TEST_COLLECTION = "projection_test_users";

async function main() {
  const client = new EkoDBClient(BASE_URL, API_KEY);
  await client.init();
  console.log("Client created\n");

  // Setup: Create test collection and insert test data
  console.log("Setting up test data...");
  try {
    await client.deleteCollection(TEST_COLLECTION);
  } catch {
    // Collection may not exist, that's fine
  }

  // Insert test users
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

  const insertedIds: string[] = [];
  for (const user of testUsers) {
    const result = await client.insert(TEST_COLLECTION, user);
    insertedIds.push(result.id);
  }
  console.log(`Inserted ${insertedIds.length} test users\n`);

  // Example 1: Select specific fields - only get id, name, email
  console.log("Example 1: Select specific fields (id, name, email only)");
  const users = await client.find(
    TEST_COLLECTION,
    new QueryBuilder()
      .eq("status", "active")
      .selectFields("id", "name", "email")
      .limit(10)
      .build(),
  );
  console.log(`  Found ${users.length} active users`);
  if (users.length > 0) {
    const user = users[0] as Record<string, unknown>;
    const fields = Object.keys(user);
    console.log(`  Fields returned: ${JSON.stringify(fields)}`);
    console.log(`  First user: ${user.name} <${user.email}>`);
  }

  // Example 2: Exclude sensitive fields - hide password, api_key, secret_token
  console.log(
    "\nExample 2: Exclude sensitive fields (password, api_key, secret_token)",
  );
  const admins = await client.find(
    TEST_COLLECTION,
    new QueryBuilder()
      .eq("user_role", "admin")
      .excludeFields("password", "api_key", "secret_token")
      .build(),
  );
  console.log(`  Found ${admins.length} admins`);
  if (admins.length > 0) {
    const admin = admins[0] as Record<string, unknown>;
    const hasPassword = "password" in admin;
    const hasApiKey = "api_key" in admin;
    const hasToken = "secret_token" in admin;
    console.log("  Sensitive fields excluded:");
    console.log(
      `    - password: ${hasPassword ? "PRESENT (unexpected!)" : "excluded"}`,
    );
    console.log(
      `    - api_key: ${hasApiKey ? "PRESENT (unexpected!)" : "excluded"}`,
    );
    console.log(
      `    - secret_token: ${hasToken ? "PRESENT (unexpected!)" : "excluded"}`,
    );
    const fields = Object.keys(admin);
    console.log(`  Fields returned: ${JSON.stringify(fields)}`);
  }

  // Example 3: Complex query with projection - active users with profile fields
  console.log(
    "\nExample 3: Complex query with projection (active users, ages 18-65)",
  );
  const activeUsers = await client.find(
    TEST_COLLECTION,
    new QueryBuilder()
      .eq("status", "active")
      .gte("age", 18)
      .lt("age", 65)
      .selectFields("id", "name", "email", "age", "created_at")
      .sortDesc("age")
      .limit(50)
      .build(),
  );
  console.log(`  Found ${activeUsers.length} active users (ages 18-65)`);
  for (const user of activeUsers) {
    const u = user as Record<string, unknown>;
    console.log(`    - ${u.name} (age ${u.age})`);
  }

  // Example 4: Query inactive users with profile fields
  console.log("\nExample 4: Query inactive users with profile fields");
  const inactiveUsers = await client.find(
    TEST_COLLECTION,
    new QueryBuilder()
      .eq("status", "inactive")
      .selectFields("id", "name", "email", "bio")
      .build(),
  );
  console.log(`  Found ${inactiveUsers.length} inactive users`);
  for (const user of inactiveUsers) {
    const u = user as Record<string, unknown>;
    console.log(`    - ${u.name}: ${u.bio}`);
  }

  // Example 5: Compare full vs projected data - demonstrates bandwidth savings
  console.log("\nExample 5: Compare full vs projected data");
  const fullUsers = await client.find(
    TEST_COLLECTION,
    new QueryBuilder().eq("status", "active").build(),
  );
  const projectedUsers = await client.find(
    TEST_COLLECTION,
    new QueryBuilder()
      .eq("status", "active")
      .selectFields("id", "name", "email")
      .build(),
  );

  if (fullUsers.length > 0 && projectedUsers.length > 0) {
    const fullFields = Object.keys(fullUsers[0] || {});
    const projectedFields = Object.keys(projectedUsers[0] || {});

    console.log("  Full query:");
    console.log(`    - ${fullFields.length} fields per record`);
    console.log(`    - Fields: ${JSON.stringify(fullFields)}`);
    console.log("  Projected query:");
    console.log(`    - ${projectedFields.length} fields per record`);
    console.log(`    - Fields: ${JSON.stringify(projectedFields)}`);
    const savings =
      100 -
      Math.floor(
        (projectedFields.length * 100) / Math.max(fullFields.length, 1),
      );
    console.log(`  Bandwidth savings: ~${savings}% fewer fields`);
  }

  // Cleanup
  console.log("\nCleaning up test data...");
  await client.deleteCollection(TEST_COLLECTION);
  console.log("Cleanup complete");

  console.log("\nAll projection examples completed successfully!");
}

main().catch(console.error);
