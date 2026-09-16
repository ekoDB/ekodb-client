/**
 * Functions Example for ekoDB TypeScript Client
 *
 * Demonstrates creating, managing, and executing scripts
 */

import {
  EkoDBClient,
  Stage,
  UserFunction,
  queryExpression,
} from "@ekodb/ekodb-client";
import * as dotenv from "dotenv";

dotenv.config();

const COLLECTION = "functions_users_client_ts";
const ACTIVE_LABEL = "get_active_users_client_ts";
const STATUS_LABEL = "get_users_by_status_client_ts";
const STATS_LABEL = "user_stats_client_ts";

/** True when a save failed because the function label already exists (HTTP 409). */
function isAlreadyExistsError(error: unknown): boolean {
  const message = error instanceof Error ? error.message : String(error);
  return message.includes("status 409") || message.includes("already exists");
}

/**
 * Idempotent save: create the function, or update it in place if a function
 * with the same label already exists. Returns the function's id either way so
 * the rest of the example (get/update/delete by id) keeps working.
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
    console.log(
      `ℹ️  Function '${script.label}' already existed — updated instead`,
    );
    const existing = await client.getFunction(script.label);
    return existing.id ?? script.label;
  }
}

async function setupTestData(client: EkoDBClient): Promise<void> {
  console.log("📋 Setting up test data...");

  for (let i = 1; i <= 10; i++) {
    const record = {
      name: `User ${i}`,
      age: 20 + i,
      status: i % 2 === 0 ? "active" : "inactive",
      score: i * 10,
    };
    await client.insert(COLLECTION, record);
  }

  console.log("✅ Test data ready\n");
}

async function simpleQueryScript(client: EkoDBClient): Promise<string> {
  console.log("📝 Example 1: Simple Query Function\n");

  const script = {
    label: ACTIVE_LABEL,
    name: "Get Active Users",
    description: "Retrieve all active users",
    version: "1.0",
    parameters: {},
    functions: [
      Stage.query(
        COLLECTION,
        queryExpression({
          type: "Condition",
          content: { field: "status", operator: "Eq", value: "active" },
        }),
      ),
    ],
    tags: ["users", "query"],
  };

  const id = await saveOrUpdate(client, script);
  console.log(`✅ Function saved: ${id}`);

  const result = await client.callFunction(id);
  console.log(`📊 Found ${result.records.length} active users\n`);

  return id;
}

async function parameterizedScript(client: EkoDBClient): Promise<string> {
  console.log("📝 Example 2: Parameterized Function\n");

  const script = {
    label: STATUS_LABEL,
    name: "Get Users By Status",
    version: "1.0",
    parameters: {
      status: {
        required: false,
        default: "active",
      },
      limit: {
        required: false,
        default: 10,
      },
    },
    functions: [
      Stage.query(
        COLLECTION,
        queryExpression({
          type: "Condition",
          content: { field: "status", operator: "Eq", value: "active" },
        }),
        undefined,
        3,
      ),
    ],
    tags: ["users", "parameterized"],
  };

  const id = await saveOrUpdate(client, script);
  console.log(`✅ Function saved: ${id}`);

  const params = { status: "active", limit: 3 };
  const result = await client.callFunction(id, params);
  console.log(`📊 Found ${result.records.length} users (limited)\n`);

  return id;
}

async function aggregationScript(client: EkoDBClient): Promise<string> {
  console.log("📝 Example 3: Aggregation Function\n");

  const script = {
    label: STATS_LABEL,
    name: "User Statistics",
    version: "1.0",
    parameters: {},
    functions: [
      Stage.findAll(COLLECTION),
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

  const id = await saveOrUpdate(client, script);
  console.log(`✅ Function saved: ${id}`);

  const result = await client.callFunction(id);
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
  console.log("📝 Example 4: UserFunction Management\n");

  // List all scripts
  const scripts = await client.listFunctions();
  console.log(`📋 Total scripts: ${scripts.length}`);

  // Get specific script by ID
  const script = await client.getFunction(getActiveUsersId);
  console.log(`🔍 Retrieved script: ${script.name}`);

  // Update script by ID
  const updated = {
    label: `${ACTIVE_LABEL}_updated`,
    name: "Get Active Users (Updated)",
    description: "Updated description",
    version: "1.1",
    parameters: {},
    functions: [Stage.findAll(COLLECTION)],
    tags: ["users"],
  };
  await client.updateFunction(getActiveUsersId, updated);
  console.log("✏️  function updated");

  await client.deleteFunction(userStatsId);
  console.log("🗑️  function deleted\n");

  console.log(
    "ℹ️  Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.\n",
  );
}

async function main(): Promise<void> {
  console.log("🚀 ekoDB Functions Example (TypeScript)\n");

  const baseUrl = process.env.API_BASE_URL || "http://localhost:8080";
  const apiKey = process.env.API_BASE_KEY;

  if (!apiKey) {
    console.error("❌ API_BASE_KEY environment variable not set");
    process.exit(1);
  }

  const client = new EkoDBClient(baseUrl, apiKey);
  await client.init();

  const scriptIds: string[] = [];
  let primaryError: unknown;
  try {
    await client.deleteCollection(COLLECTION).catch(() => undefined);
    await setupTestData(client);
    scriptIds.push(await simpleQueryScript(client));
    scriptIds.push(await parameterizedScript(client));
    scriptIds.push(await aggregationScript(client));
    await scriptManagement(client, scriptIds[0], scriptIds[1], scriptIds[2]);
    console.log("✅ All examples completed!");
  } catch (error) {
    primaryError = error;
    throw error;
  } finally {
    const cleanup = await Promise.allSettled([
      ...scriptIds.map((id) =>
        client.deleteFunction(id).catch((error) => {
          if (!String(error).includes("not found")) throw error;
        }),
      ),
      client.deleteCollection(COLLECTION),
    ]);
    const failures = cleanup.filter(
      (result): result is PromiseRejectedResult => result.status === "rejected",
    );
    if (failures.length > 0) {
      const errors = failures.map((failure) => failure.reason);
      if (primaryError !== undefined) errors.unshift(primaryError);
      throw new AggregateError(
        errors,
        primaryError === undefined
          ? "Function example cleanup failed"
          : "Function example and cleanup failed",
      );
    }
  }
}

main().catch((error) => {
  console.error("❌ Error:", error);
  process.exit(1);
});
