/**
 * CRUD Functions Example - Basic Data Operations
 *
 * Demonstrates basic CRUD operations using scripts:
 * - FindAll queries
 * - Group aggregations
 * - Simple data transformations
 */

import { EkoDBClient, Stage, UserFunction } from "@ekodb/ekodb-client";
import * as dotenv from "dotenv";

dotenv.config();

const BASE_URL = process.env.API_BASE_URL || "http://localhost:8080";
const API_KEY = process.env.API_BASE_KEY || "a-test-api-key-from-ekodb";

/** True when a save failed because the function label already exists (HTTP 409). */
function isAlreadyExistsError(error: unknown): boolean {
  const message = error instanceof Error ? error.message : String(error);
  return message.includes("status 409") || message.includes("already exists");
}

/**
 * Idempotent save: create the function, or update it in place if a function
 * with the same label already exists. Returns the function's id so downstream
 * cleanup-by-id continues to work.
 */
async function saveOrUpdate(
  client: EkoDBClient,
  script: UserFunction,
): Promise<string> {
  try {
    return await client.saveFunction(script);
  } catch (error) {
    if (!isAlreadyExistsError(error)) throw error;
    await client.updateFunction(script.label, script);
    console.log(`Function '${script.label}' already existed — updated instead`);
    const existing = await client.getFunction(script.label);
    return existing.id ?? script.label;
  }
}

function scalar(value: any): any {
  return value && typeof value === "object" && "value" in value
    ? value.value
    : value;
}

function requireRecordCount(
  result: any,
  expected: number,
  label: string,
): any[] {
  if (!Array.isArray(result.records)) {
    throw new Error(`${label} response omitted records`);
  }
  if (result.records.length !== expected) {
    throw new Error(
      `${label} returned ${result.records.length} records; expected ${expected}`,
    );
  }
  return result.records;
}

async function main() {
  const client = new EkoDBClient(BASE_URL, API_KEY);
  const suffix = `${process.pid}_${Date.now()}`;
  const collection = `crud_users_ts_${suffix}`;
  const listLabel = `list_all_users_ts_${suffix}`;
  const statusLabel = `users_by_status_ts_${suffix}`;
  const roleLabel = `avg_score_by_role_ts_${suffix}`;
  const scriptIds: string[] = [];
  let ownsCollection = false;
  let primaryError: unknown;

  console.log("🚀 ekoDB TypeScript CRUD Functions Example\n");

  try {
    // Setup test data
    console.log("📋 Setting up test data...");
    ownsCollection = true;
    for (let i = 1; i <= 10; i++) {
      await client.insert(collection, {
        name: `User ${i}`,
        email: `user${i}@example.com`,
        status: i % 3 === 0 ? "inactive" : "active",
        score: i * 10,
        role: i <= 3 ? "admin" : "user",
      });
    }
    console.log("✅ Created 10 test users\n");

    // Example 1: List All Users
    console.log("📝 Example 1: List All Users\n");
    const script1 = {
      label: listLabel,
      name: "List All Users",
      version: "1.0",
      parameters: {},
      functions: [Stage.findAll(collection)],
      tags: ["users", "list"],
    };
    const scriptId1 = await saveOrUpdate(client, script1);
    scriptIds.push(scriptId1);
    console.log("✅ Function saved");

    const result1 = await client.callFunction(scriptId1);
    const userRecords = requireRecordCount(result1, 10, "List users");
    console.log(`📊 Found ${userRecords.length} users`);
    console.log(`⏱️  Execution time: ${result1.stats?.execution_time_ms}ms\n`);

    // Example 2: Count Users by Status
    console.log("📝 Example 2: Count Users by Status\n");
    const script2 = {
      label: statusLabel,
      name: "Users by Status",
      version: "1.0",
      parameters: {},
      functions: [
        Stage.findAll(collection),
        Stage.group(
          ["status"],
          [{ output_field: "count", operation: "Count" }],
        ),
      ],
      tags: ["users", "analytics"],
    };
    const scriptId2 = await saveOrUpdate(client, script2);
    scriptIds.push(scriptId2);
    console.log("✅ Function saved");

    const result2 = await client.callFunction(scriptId2);
    const statusRecords = requireRecordCount(
      result2,
      2,
      "Group users by status",
    );
    const statusCounts = Object.fromEntries(
      statusRecords.map((group: any) => [
        String(scalar(group.status)),
        Number(scalar(group.count)),
      ]),
    );
    if (statusCounts.active !== 7 || statusCounts.inactive !== 3) {
      throw new Error(
        `Status counts were ${JSON.stringify(statusCounts)}; expected active=7 and inactive=3`,
      );
    }
    console.log("📊 User counts by status:");
    statusRecords.forEach((group: any) => {
      const status = group.status?.value || group.status;
      const count = group.count?.value || group.count;
      console.log(`   ${status}: ${count} users`);
    });
    console.log(`⏱️  Execution time: ${result2.stats?.execution_time_ms}ms\n`);

    // Example 3: Average Score by Role
    console.log("📝 Example 3: Average Score by Role\n");
    const script3 = {
      label: roleLabel,
      name: "Average Score by Role",
      version: "1.0",
      parameters: {},
      functions: [
        Stage.findAll(collection),
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
    const scriptId3 = await saveOrUpdate(client, script3);
    scriptIds.push(scriptId3);
    console.log("✅ Function saved");

    const result3 = await client.callFunction(scriptId3);
    const roleRecords = requireRecordCount(result3, 2, "Group users by role");
    const roles = Object.fromEntries(
      roleRecords.map((group: any) => [
        String(scalar(group.role)),
        {
          count: Number(scalar(group.count)),
          average: Number(scalar(group.avg_score)),
        },
      ]),
    );
    if (
      roles.admin?.count !== 3 ||
      roles.admin?.average !== 20 ||
      roles.user?.count !== 7 ||
      roles.user?.average !== 70
    ) {
      throw new Error(
        `Role aggregates were ${JSON.stringify(roles)}; expected admin=3/20 and user=7/70`,
      );
    }
    console.log("📊 Average score by role:");
    roleRecords.forEach((group: any) =>
      console.log(`   ${JSON.stringify(group)}`),
    );
    console.log(`⏱️  Execution time: ${result3.stats?.execution_time_ms}ms\n`);
  } catch (error) {
    primaryError = error;
    throw error;
  } finally {
    console.log("🧹 Cleaning up...");
    const cleanupErrors: unknown[] = [];
    for (const scriptId of scriptIds) {
      try {
        await client.deleteFunction(scriptId);
      } catch (error) {
        const message = error instanceof Error ? error.message : String(error);
        if (!message.includes("status 404") && !/not found/i.test(message)) {
          cleanupErrors.push(error);
        }
      }
    }
    if (ownsCollection) {
      try {
        await client.deleteCollection(collection);
      } catch (error) {
        const message = error instanceof Error ? error.message : String(error);
        if (!message.includes("status 404") && !/not found/i.test(message)) {
          cleanupErrors.push(error);
        }
      }
    }
    if (cleanupErrors.length > 0) {
      if (primaryError) console.error("Cleanup errors:", cleanupErrors);
      else
        throw new AggregateError(
          cleanupErrors,
          "CRUD functions cleanup failed",
        );
    } else {
      console.log("✅ Cleanup complete\n");
    }
  }

  console.log("✅ All CRUD script examples finished!");
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
