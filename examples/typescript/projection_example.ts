import { EkoDBClient, QueryBuilder } from "@ekodb/ekodb-client";
import * as dotenv from "dotenv";

dotenv.config();

const BASE_URL = process.env.API_BASE_URL || "http://localhost:8080";
const API_KEY = process.env.API_BASE_KEY || "a-test-api-key-from-ekodb";

const TEST_COLLECTION = "projection_test_users";

async function main() {
  const client = new EkoDBClient(BASE_URL, API_KEY);
  await client.init();
  console.log("✓ Client created\n");

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
  console.log(`✓ Inserted ${insertedIds.length} test users\n`);

  // Example 1: Select specific fields to reduce data transfer
  console.log("Example 1: Select specific fields");
  const users = await client.find(
    TEST_COLLECTION,
    new QueryBuilder()
      .eq("status", "active")
      .selectFields("id", "name", "email")
      .limit(10)
      .build(),
  );
  console.log(`Fetched ${users.length} users with only 3 fields each`);

  // Example 2: Exclude sensitive fields
  console.log("\nExample 2: Exclude sensitive fields");
  const admins = await client.find(
    TEST_COLLECTION,
    new QueryBuilder()
      .eq("user_role", "admin")
      .excludeFields("password", "api_key", "secret_token")
      .build(),
  );
  console.log(`Fetched ${admins.length} admins without sensitive data`);
  // Verify sensitive fields are excluded
  if (admins.length > 0) {
    const admin = admins[0] as Record<string, unknown>;
    const hasPassword = "password" in admin;
    console.log(`  Password field excluded: ${!hasPassword}`);
  }

  // Example 3: Complex query with projection
  console.log("\nExample 3: Complex query with projection");
  const activeUsers = await client.find(
    TEST_COLLECTION,
    new QueryBuilder()
      .eq("status", "active")
      .gte("age", 18)
      .lt("age", 65)
      .selectFields("id", "name", "email", "age", "created_at")
      .sortDesc("created_at")
      .limit(50)
      .build(),
  );
  console.log(`Fetched ${activeUsers.length} active users with profile fields`);

  // Example 4: Find by ID with projection using query filter
  console.log("\nExample 4: Find by ID with projection");
  const userProfiles = await client.find(
    TEST_COLLECTION,
    new QueryBuilder()
      .eq("id", insertedIds[0])
      .selectFields("id", "name", "email", "bio", "avatar_url")
      .build(),
  );
  const userProfile = userProfiles[0] as Record<string, unknown> | undefined;
  console.log(`Fetched user profile: ${userProfile?.name || "N/A"}`);

  // Example 5: Demonstrate data reduction
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
  console.log(
    `Full query returned ${Object.keys(fullUsers[0] || {}).length} fields per user`,
  );
  console.log(
    `Projected query returned ${Object.keys(projectedUsers[0] || {}).length} fields per user`,
  );

  // Cleanup
  console.log("\n🧹 Cleaning up test data...");
  await client.deleteCollection(TEST_COLLECTION);
  console.log("✅ Cleanup complete");

  console.log("\n✅ All projection examples completed successfully!");
}

main().catch(console.error);
