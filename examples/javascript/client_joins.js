/**
 * Join Operations Example
 *
 * Demonstrates multi-collection joins:
 * - Single collection joins
 * - Multi-collection joins
 * - Using joins with queries
 */

const { EkoDBClient, QueryBuilder, JoinBuilder } = require("@ekodb/ekodb-client");
require("dotenv").config();

async function main() {
  const client = new EkoDBClient(
    process.env.API_BASE_URL || "http://localhost:8080",
    process.env.API_BASE_KEY || "a-test-api-key-from-ekodb"
  );
  await client.init();

  console.log("=== Join Operations Examples ===\n");

  // Use unique collection names
  const usersCollection = "joins_users_client_js";
  const departmentsCollection = "joins_departments_client_js";
  const profilesCollection = "joins_profiles_client_js";
  const ordersCollection = "joins_orders_client_js";

  // Cleanup any existing test collections
  try {
    await client.deleteCollection(usersCollection);
    await client.deleteCollection(departmentsCollection);
    await client.deleteCollection(profilesCollection);
    await client.deleteCollection(ordersCollection);
  } catch (e) {
    // Ignore errors if collections don't exist
  }

  try {
    // Setup: Create sample data
    console.log("Setting up sample data...");

    // Create departments first (need their IDs for users)
    const dept1 = await client.insert(departmentsCollection, {
      name: "Engineering",
      location: "Building A",
    });

    const dept2 = await client.insert(departmentsCollection, {
      name: "Sales",
      location: "Building B",
    });

    // Create users with actual department IDs
    const user1 = await client.insert(usersCollection, {
      name: "Alice Johnson",
      email: "alice@example.com",
      department_id: dept1.id,
    });

    const user2 = await client.insert(usersCollection, {
      name: "Bob Smith",
      email: "bob@example.com",
      department_id: dept2.id,
    });

    // Create orders
    await client.insert(ordersCollection, {
      user_id: user1.id,
      product: "Laptop",
      amount: 1200,
      status: "completed",
    });

    await client.insert(ordersCollection, {
      user_id: user1.id,
      product: "Mouse",
      amount: 25,
      status: "completed",
    });

    await client.insert(ordersCollection, {
      user_id: user2.id,
      product: "Keyboard",
      amount: 75,
      status: "pending",
    });

    console.log("✅ Sample data created\n");

    // Example 1: Simple single collection join
    console.log("1. Single collection join (users with departments):");
    const join1 = JoinBuilder.single(
      departmentsCollection,
      "department_id",
      "id",
      "department"
    );

    const query1 = new QueryBuilder().join(join1).limit(10).build();

    const results1 = await client.find(usersCollection, query1);
    console.log(`Found ${results1.length} users with department data:`);
    results1.forEach((user) => {
      const userName = user.name?.value || user.name || "Unknown";
      // Join returns an array, get first element
      const depts = user.department || [];
      const dept = depts[0] || {};
      const deptName = dept.name?.value || dept.name || "No department";
      console.log(`  - ${userName}: ${deptName}`);
    });
    console.log();

    // Example 2: Join with filtering
    console.log("2. Join with filtering:");
    const join2 = JoinBuilder.single(
      departmentsCollection,
      "department_id",
      "id",
      "department"
    );

    const query2 = new QueryBuilder()
      .eq("department_id", dept1.id)
      .join(join2)
      .build();

    const results2 = await client.find(usersCollection, query2);
    console.log(`Found ${results2.length} users in Engineering:`);
    results2.forEach((user) => {
      const userName = user.name?.value || user.name || "Unknown";
      // Join returns an array, get first element
      const depts = user.department || [];
      const dept = depts[0] || {};
      const location = dept.location?.value || dept.location || "Unknown";
      console.log(`  - ${userName}: ${location}`);
    });
    console.log();

    // Example 3: Join with user profiles
    console.log("3. Join with user profiles:");

    // Create profiles
    await client.insert(profilesCollection, {
      user_id: user1.id,
      bio: "Senior Software Engineer",
      skills: ["JavaScript", "TypeScript", "React"],
    });

    await client.insert(profilesCollection, {
      user_id: user2.id,
      bio: "Sales Manager",
      skills: ["Negotiation", "CRM", "Communication"],
    });

    const join3 = JoinBuilder.single(
      profilesCollection,
      "id",
      "user_id",
      "profile"
    );

    const query3 = new QueryBuilder().join(join3).limit(10).build();

    const results3 = await client.find(usersCollection, query3);
    console.log(`Found ${results3.length} users with profile data:`);
    results3.forEach((user) => {
      const userName = user.name?.value || user.name || "Unknown";
      // Join returns an array, get first element
      const profiles = user.profile || [];
      const profile = profiles[0] || {};
      const bio = profile.bio?.value || profile.bio || "N/A";
      console.log(`  - ${userName}: ${bio}`);
    });
    console.log();

    // Example 4: Join orders with user data
    console.log("4. Join orders with user data:");
    const join4 = JoinBuilder.single(usersCollection, "user_id", "id", "user");

    const query4 = new QueryBuilder()
      .eq("status", "completed")
      .join(join4)
      .sortDesc("amount")
      .build();

    const results4 = await client.find(ordersCollection, query4);
    console.log(`Found ${results4.length} completed orders:`);
    results4.forEach((order) => {
      const product = order.product?.value || order.product || "Unknown";
      const amount = order.amount?.value || order.amount || 0;
      // Join returns an array, get first element
      const users = order.user || [];
      const user = users[0] || {};
      const userName = user.name?.value || user.name || "Unknown";
      console.log(`  - ${product} ($${amount}) by ${userName}`);
    });
    console.log();

    // Example 5: Complex join with multiple conditions
    console.log("5. Complex join with multiple conditions:");
    const join5 = JoinBuilder.single(
      departmentsCollection,
      "department_id",
      "id",
      "department"
    );

    const query5 = new QueryBuilder()
      .contains("email", "@example.com")
      .join(join5)
      .sortAsc("name")
      .limit(5)
      .build();

    const results5 = await client.find(usersCollection, query5);
    console.log(`Found ${results5.length} users with example.com emails:`);
    results5.forEach((user) => {
      const userName = user.name?.value || user.name || "Unknown";
      const email = user.email?.value || user.email || "Unknown";
      // Join returns an array, get first element
      const depts = user.department || [];
      const dept = depts[0] || {};
      const location = dept.location?.value || dept.location || "N/A";
      console.log(`  - ${userName} (${email}): ${location}`);
    });
    console.log();

    // Cleanup
    console.log("=== Cleanup ===");
    await client.deleteCollection(usersCollection);
    await client.deleteCollection(departmentsCollection);
    await client.deleteCollection(profilesCollection);
    await client.deleteCollection(ordersCollection);
    console.log("✅ Deleted test collections");

    console.log("\n✅ Join operations examples completed!");
  } catch (error) {
    // Cleanup on error
    try {
      await client.deleteCollection(usersCollection);
      await client.deleteCollection(departmentsCollection);
      await client.deleteCollection(profilesCollection);
      await client.deleteCollection(ordersCollection);
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
