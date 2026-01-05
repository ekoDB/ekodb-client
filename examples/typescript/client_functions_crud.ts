/**
 * CRUD Scripts Example - Basic Data Operations
 *
 * Demonstrates basic CRUD operations using scripts:
 * - FindAll queries
 * - Group aggregations
 * - Simple data transformations
 */

import { EkoDBClient, Stage } from "@ekodb/ekodb-client";
import * as dotenv from "dotenv";

dotenv.config();

const BASE_URL = process.env.API_BASE_URL || "http://localhost:8080";
const API_KEY = process.env.API_BASE_KEY || "a-test-api-key-from-ekodb";

async function main() {
  const client = new EkoDBClient(BASE_URL, API_KEY);

  console.log("🚀 ekoDB TypeScript CRUD Scripts Example\n");

  // Setup test data
  console.log("📋 Setting up test data...");
  try {
    await client.deleteCollection("crud_users_ts");
  } catch (e) {}

  for (let i = 1; i <= 10; i++) {
    await client.insert("crud_users_ts", {
      name: `User ${i}`,
      email: `user${i}@example.com`,
      status: i % 3 === 0 ? "inactive" : "active",
      score: i * 10,
      role: i <= 3 ? "admin" : "user",
    });
  }
  console.log("✅ Created 10 test users\n");

  const scriptIds: string[] = [];

  // Example 1: List All Users
  console.log("📝 Example 1: List All Users\n");
  const script1 = {
    label: "list_all_users_ts",
    name: "List All Users",
    version: "1.0",
    parameters: {},
    functions: [Stage.findAll("crud_users_ts")],
    tags: ["users", "list"],
  };
  const scriptId1 = await client.saveScript(script1);
  scriptIds.push(scriptId1);
  console.log("✅ Script saved");

  const result1 = await client.callScript("list_all_users_ts");
  console.log(`📊 Found ${result1.records?.length || 0} users`);
  console.log(`⏱️  Execution time: ${result1.stats?.execution_time_ms}ms\n`);

  // Example 2: Count Users by Status
  console.log("📝 Example 2: Count Users by Status\n");
  const script2 = {
    label: "users_by_status_ts",
    name: "Users by Status",
    version: "1.0",
    parameters: {},
    functions: [
      Stage.findAll("crud_users_ts"),
      Stage.group(["status"], [{ output_field: "count", operation: "Count" }]),
    ],
    tags: ["users", "analytics"],
  };
  const scriptId2 = await client.saveScript(script2);
  scriptIds.push(scriptId2);
  console.log("✅ Script saved");

  const result2 = await client.callScript("users_by_status_ts");
  console.log("📊 User counts by status:");
  result2.records?.forEach((group: any) => {
    const status = group.status?.value || group.status;
    const count = group.count?.value || group.count;
    console.log(`   ${status}: ${count} users`);
  });
  console.log(`⏱️  Execution time: ${result2.stats?.execution_time_ms}ms\n`);

  // Example 3: Average Score by Role
  console.log("📝 Example 3: Average Score by Role\n");
  const script3 = {
    label: "avg_score_by_role_ts",
    name: "Average Score by Role",
    version: "1.0",
    parameters: {},
    functions: [
      Stage.findAll("crud_users_ts"),
      Stage.group(
        ["role"],
        [
          {
            output_field: "avg_score",
            operation: "Average",
            input_field: "score",
          },
          { output_field: "count", operation: "Count" },
        ],
      ),
    ],
    tags: ["users", "analytics"],
  };
  const scriptId3 = await client.saveScript(script3);
  scriptIds.push(scriptId3);
  console.log("✅ Script saved");

  const result3 = await client.callScript("avg_score_by_role_ts");
  console.log("📊 Average score by role:");
  result3.records?.forEach((group: any) =>
    console.log(`   ${JSON.stringify(group)}`),
  );
  console.log(`⏱️  Execution time: ${result3.stats?.execution_time_ms}ms\n`);

  // Cleanup
  console.log("🧹 Cleaning up...");
  for (const scriptId of scriptIds) {
    try {
      await client.deleteScript(scriptId);
    } catch (e) {}
  }
  try {
    await client.deleteCollection("crud_users_ts");
  } catch (e) {}
  console.log("✅ Cleanup complete\n");

  console.log("✅ All CRUD script examples finished!");
}

main().catch(console.error);
