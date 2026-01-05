/**
 * Query Builder Example
 *
 * Demonstrates using the QueryBuilder for complex queries with:
 * - Comparison operators (eq, gt, lt, etc.)
 * - String operators (contains, startsWith, endsWith)
 * - Logical operators (and, or, not)
 * - Sorting and pagination
 */

const { EkoDBClient, QueryBuilder } = require("@ekodb/ekodb-client");
const dotenv = require("dotenv");

dotenv.config();

async function main() {
  const client = new EkoDBClient(
    process.env.API_BASE_URL || "http://localhost:8080",
    process.env.API_BASE_KEY || "a-test-api-key-from-ekodb"
  );
  await client.init();

  console.log("=== Query Builder Examples ===\n");

  const collection = "query_builder_client_js";

  // Cleanup any existing test collection
  try {
    await client.deleteCollection(collection);
  } catch (e) {
    // Ignore errors if collection doesn't exist
  }

  try {
    // Setup: Insert test data
    console.log("Setting up test data...");

    await client.insert(collection, {
      name: "Alice Johnson",
      email: "alice@example.com",
      age: 28,
      status: "active",
      role: "user",
      country: "USA",
      created_at: new Date().toISOString(),
    });

    await client.insert(collection, {
      name: "Bob Smith",
      email: "bob@example.com",
      age: 35,
      status: "active",
      role: "admin",
      country: "Canada",
      created_at: new Date().toISOString(),
    });

    await client.insert(collection, {
      name: "Charlie Brown",
      email: "charlie@test.com",
      age: 42,
      status: "inactive",
      role: "moderator",
      country: "UK",
      created_at: new Date().toISOString(),
    });

    console.log("✅ Test data created\n");

    // Example 1: Simple equality query
    console.log("1. Simple equality query:");
    const query1 = new QueryBuilder().eq("status", "active").limit(5).build();

    const results1 = await client.find(collection, query1);
    console.log(`Found ${results1.length} active users\n`);

    // Example 2: Range query with sorting
    console.log("2. Range query with sorting:");
    const query2 = new QueryBuilder()
      .gte("age", 18)
      .lte("age", 65)
      .sortDesc("created_at")
      .limit(10)
      .build();

    const results2 = await client.find(collection, query2);
    console.log(`Found ${results2.length} users aged 18-65\n`);

    // Example 3: String operations
    console.log("3. String operations:");
    const query3 = new QueryBuilder()
      .contains("email", "@example.com")
      .sortAsc("name")
      .build();

    const results3 = await client.find(collection, query3);
    console.log(`Found ${results3.length} users with @example.com emails\n`);

    // Example 4: IN operator
    console.log("4. IN operator:");
    const query4 = new QueryBuilder()
      .in("role", ["admin", "moderator", "editor"])
      .sortDesc("last_login")
      .limit(20)
      .build();

    const results4 = await client.find(collection, query4);
    console.log(`Found ${results4.length} privileged users\n`);

    // Example 5: Complex query with multiple conditions
    console.log("5. Complex query with multiple conditions:");
    const query5 = new QueryBuilder()
      .eq("status", "active")
      .gt("age", 21)
      .startsWith("country", "US")
      .sortDesc("created_at")
      .limit(15)
      .build();

    const results5 = await client.find(collection, query5);
    console.log(`Found ${results5.length} active US users over 21\n`);

    // Example 6: Pagination
    console.log("6. Pagination:");
    const pageSize = 10;
    const page = 0;

    const query6 = new QueryBuilder()
      .eq("status", "active")
      .page(page, pageSize)
      .sortAsc("name")
      .build();

    const results6 = await client.find(collection, query6);
    console.log(`Page ${page + 1}: ${results6.length} users\n`);

    // Example 7: NOT IN operator
    console.log("7. NOT IN operator:");
    const query7 = new QueryBuilder()
      .nin("status", ["deleted", "banned", "suspended"])
      .limit(20)
      .build();

    const results7 = await client.find(collection, query7);
    console.log(`Found ${results7.length} valid users\n`);

    // Example 8: Using bypass flags
    console.log("8. Using bypass flags:");
    const query8 = new QueryBuilder()
      .eq("status", "active")
      .bypassCache(true)
      .limit(5)
      .build();

    const results8 = await client.find(collection, query8);
    console.log(`Found ${results8.length} users (bypassed cache)\n`);

    // Cleanup
    console.log("=== Cleanup ===");
    await client.deleteCollection(collection);
    console.log("✅ Deleted test collection");

    console.log("\n✅ Query Builder examples completed!");
  } catch (error) {
    // Cleanup on error
    try {
      await client.deleteCollection(collection);
    } catch (e) {
      // Ignore cleanup errors
    }
    throw error;
  }
}

main().catch((error) => {
  console.error("❌ Error:", error);
  process.exit(1);
});
