/**
 * Scripts Example for ekoDB TypeScript Client
 *
 * Demonstrates creating, managing, and executing scripts
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

async function simpleQueryScript(client: EkoDBClient): Promise<string> {
  console.log("📝 Example 1: Simple Query Script\n");

  const script = {
    label: "get_active_users",
    name: "Get Active Users",
    description: "Retrieve all active users",
    version: "1.0",
    parameters: {},
    functions: [Stage.findAll("users")],
    tags: ["users", "query"],
  };

  const id = await client.saveScript(script);
  console.log(`✅ Script saved: ${id}`);

  const result = await client.callScript("get_active_users");
  console.log(`📊 Found ${result.records.length} active users\n`);

  return id;
}

async function parameterizedScript(client: EkoDBClient): Promise<string> {
  console.log("📝 Example 2: Parameterized Script\n");

  const script = {
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
    functions: [Stage.findAll("users")],
    tags: ["users", "parameterized"],
  };

  const id = await client.saveScript(script);
  console.log(`✅ Script saved: ${id}`);

  const params = { status: "active", limit: 3 };
  const result = await client.callScript("get_users_by_status", params);
  console.log(`📊 Found ${result.records.length} users (limited)\n`);

  return id;
}

async function aggregationScript(client: EkoDBClient): Promise<string> {
  console.log("📝 Example 3: Aggregation Script\n");

  const script = {
    label: "user_stats",
    name: "User Statistics",
    version: "1.0",
    parameters: {},
    functions: [
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

  const id = await client.saveScript(script);
  console.log(`✅ Script saved: ${id}`);

  const result = await client.callScript("user_stats");
  console.log(`📊 Statistics: ${result.records.length} groups`);
  result.records.forEach((record) =>
    console.log(`   ${JSON.stringify(record)}`),
  );
  console.log();

  return id;
}

async function scriptManagement(
  client: EkoDBClient,
  getActiveUsersId: string,
  getUsersByStatusId: string,
  userStatsId: string,
): Promise<void> {
  console.log("📝 Example 4: Script Management\n");

  // List all scripts
  const scripts = await client.listScripts();
  console.log(`📋 Total scripts: ${scripts.length}`);

  // Get specific script by ID
  const script = await client.getScript(getActiveUsersId);
  console.log(`🔍 Retrieved script: ${script.name}`);

  // Update script by ID
  const updated = {
    label: "get_active_users_updated",
    name: "Get Active Users (Updated)",
    description: "Updated description",
    version: "1.1",
    parameters: {},
    functions: [Stage.findAll("users")],
    tags: ["users"],
  };
  await client.updateScript(getActiveUsersId, updated);
  console.log("✏️  Script updated");

  // Delete script by ID
  await client.deleteScript(userStatsId);
  console.log("🗑️  Script deleted\n");

  console.log(
    "ℹ️  Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.\n",
  );
}

async function main(): Promise<void> {
  console.log("🚀 ekoDB Scripts Example (TypeScript)\n");

  const baseUrl = process.env.API_BASE_URL || "http://localhost:8080";
  const apiKey = process.env.API_BASE_KEY;

  if (!apiKey) {
    console.error("❌ API_BASE_KEY environment variable not set");
    process.exit(1);
  }

  const client = new EkoDBClient(baseUrl, apiKey);
  await client.init();

  // Run examples and track IDs
  await setupTestData(client);
  const getActiveUsersId = await simpleQueryScript(client);
  const getUsersByStatusId = await parameterizedScript(client);
  const userStatsId = await aggregationScript(client);
  await scriptManagement(
    client,
    getActiveUsersId,
    getUsersByStatusId,
    userStatsId,
  );

  console.log("✅ All examples completed!");
}

main().catch((error) => {
  console.error("❌ Error:", error);
  process.exit(1);
});
