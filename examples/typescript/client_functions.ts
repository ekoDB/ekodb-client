/**
 * Saved Functions Example for ekoDB TypeScript Client
 *
 * Demonstrates creating, managing, and executing saved functions
 */

import { EkoDBClient, Stage } from "@ekodb/ekodb-client";
import * as dotenv from "dotenv";

dotenv.config();

async function setupTestData(client: EkoDBClient): Promise<void> {
  console.log("📋 Setting up test data...");

  for (let i = 1; i <= 10; i++) {
    const record = {
      name: `User ${i}`,
      age: 20 + i,
      status: i % 2 === 0 ? "active" : "inactive",
      score: i * 10,
    };
    await client.insert("users", record);
  }

  console.log("✅ Test data ready\n");
}

async function simpleQueryFunction(client: EkoDBClient): Promise<string> {
  console.log("📝 Example 1: Simple Query Function\n");

  const function1 = {
    label: "get_active_users",
    name: "Get Active Users",
    description: "Retrieve all active users",
    version: "1.0",
    parameters: {},
    pipeline: [Stage.findAll("users")],
    tags: ["users", "query"],
  };

  const id = await client.saveFunction(function1);
  console.log(`✅ Function saved: ${id}`);

  const result = await client.callFunction("get_active_users");
  console.log(`📊 Found ${result.records.length} active users\n`);

  return id;
}

async function parameterizedFunction(client: EkoDBClient): Promise<void> {
  console.log("📝 Example 2: Parameterized Function\n");

  const function2 = {
    label: "get_users_by_status",
    name: "Get Users By Status",
    version: "1.0",
    parameters: {
      status: {
        param_type: "String" as const,
        required: false,
        default: "active",
      },
      limit: {
        param_type: "Integer" as const,
        required: false,
        default: 10,
      },
    },
    pipeline: [Stage.findAll("users")],
    tags: ["users", "parameterized"],
  };

  const id = await client.saveFunction(function2);
  console.log(`✅ Function saved: ${id}`);

  const params = { status: "active", limit: 3 };
  const result = await client.callFunction("get_users_by_status", params);
  console.log(`📊 Found ${result.records.length} users (limited)\n`);
}

async function aggregationFunction(client: EkoDBClient): Promise<string> {
  console.log("📝 Example 3: Aggregation Function\n");

  const function3 = {
    label: "user_stats",
    name: "User Statistics",
    version: "1.0",
    parameters: {},
    pipeline: [
      Stage.findAll("users"),
      Stage.group(
        ["status"], // by_fields
        [
          // functions
          { output_field: "count", operation: "Count" as const },
          {
            output_field: "avg_score",
            operation: "Average" as const,
            input_field: "score",
          },
        ],
      ),
    ],
    tags: ["analytics"],
  };

  const id = await client.saveFunction(function3);
  console.log(`✅ Function saved: ${id}`);

  const result = await client.callFunction("user_stats");
  console.log(`📊 Statistics: ${result.records.length} groups`);
  result.records.forEach((record) =>
    console.log(`   ${JSON.stringify(record)}`),
  );
  console.log();

  return id;
}

async function functionManagement(
  client: EkoDBClient,
  getActiveUsersId: string,
  userStatsId: string,
): Promise<void> {
  console.log("📝 Example 4: Function Management\n");

  // List all functions
  const functions = await client.listFunctions();
  console.log(`📋 Total functions: ${functions.length}`);

  // Get specific function (requires ID, not label)
  const func = await client.getFunction(getActiveUsersId);
  console.log(`🔍 Retrieved function: ${func.name}`);

  // Update function (requires ID, not label)
  const updated = {
    label: "get_active_users",
    name: "Get Active Users (Updated)",
    description: "Updated description",
    version: "1.1",
    parameters: {},
    pipeline: [Stage.findAll("users")],
    tags: ["users"],
  };
  await client.updateFunction(getActiveUsersId, updated);
  console.log("✏️  Function updated");

  // Delete function (requires ID, not label)
  await client.deleteFunction(userStatsId);
  console.log("🗑️  Function deleted\n");

  console.log(
    "ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID",
  );
  console.log("ℹ️  Only CALL can use either ID or label\n");
}

async function main(): Promise<void> {
  console.log("🚀 ekoDB Saved Functions Example (TypeScript)\n");

  const baseUrl = process.env.API_BASE_URL || "http://localhost:8080";
  const apiKey = process.env.API_BASE_KEY;

  if (!apiKey) {
    console.error("❌ API_BASE_KEY environment variable not set");
    process.exit(1);
  }

  const client = new EkoDBClient(baseUrl, apiKey);
  await client.init();

  // Run examples
  await setupTestData(client);
  const getActiveUsersId = await simpleQueryFunction(client);
  await parameterizedFunction(client);
  const userStatsId = await aggregationFunction(client);
  await functionManagement(client, getActiveUsersId, userStatsId);

  console.log("✅ All examples completed!");
}

main().catch((error) => {
  console.error("❌ Error:", error);
  process.exit(1);
});
