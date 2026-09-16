/**
 * Functions Example - Using @ekodb/ekodb-client library
 *
 * Demonstrates creating, managing, and executing functions with the JavaScript client.
 * Covers: FindAll, Group, Sort, Limit, Count, and Function management operations.
 */

const { EkoDBClient, Stage, ChatMessage } = require("@ekodb/ekodb-client");
const dotenv = require("dotenv");

dotenv.config();

const BASE_URL = process.env.API_BASE_URL || "http://localhost:8080";
const API_KEY = process.env.API_BASE_KEY || "a-test-api-key-from-ekodb";
const COLLECTION = "functions_users_client_js";
const LABELS = {
  active: "get_active_users_client_js",
  status: "get_users_by_status_client_js",
  stats: "user_stats_client_js",
  top: "top_users_client_js",
  count: "count_users_client_js",
};

/**
 * Save a function idempotently.
 *
 * The server returns HTTP 409 ("A function with label 'X' already exists.")
 * when a function with the same fixed label already exists. On that error we
 * UPDATE the existing function via PUT /api/functions/{label} (the server's
 * GET/PUT/DELETE routes accept either the encrypted ID or the label), then
 * resolve and return its encrypted ID so the rest of the example keeps working.
 * Any other error is propagated.
 */
async function saveOrUpdate(client, script) {
  try {
    return await client.saveFunction(script);
  } catch (error) {
    if (error.message && error.message.includes("already exists")) {
      // updateFunction accepts the label (server resolves id-or-label)
      await client.updateFunction(script.label, script);
      console.log(
        `ℹ️  Function '${script.label}' already existed — updated instead`,
      );
      // Resolve the encrypted ID by label for downstream GET/UPDATE/DELETE
      const existing = await client.getFunction(script.label);
      return existing.id;
    }
    throw error;
  }
}

async function setupTestData(client) {
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

async function simpleQueryScript(client) {
  console.log("📝 Example 1: Simple Query Function\n");

  const script = {
    label: LABELS.active,
    name: "Get Active Users",
    description: "Retrieve all active users",
    version: "1.0",
    parameters: {},
    functions: [
      {
        type: "Query",
        collection: COLLECTION,
        filter: {
          type: "Condition",
          content: { field: "status", operator: "Eq", value: "active" },
        },
      },
    ],
    tags: ["users", "query"],
  };

  // Save script
  const scriptId = await saveOrUpdate(client, script);
  console.log(`✅ Function saved: ${scriptId}`);

  // Call script (use label)
  const result = await client.callFunction(scriptId);
  if (result.records.length !== 5) {
    throw new Error(
      `Active-user function returned ${result.records.length} records; expected 5`,
    );
  }
  console.log(`📊 Found ${result.records.length} records`);
  console.log(`⏱️  Execution time: ${result.stats.execution_time_ms}ms\n`);

  return scriptId;
}

async function parameterizedScript(client) {
  console.log("📝 Example 2: Parameterized Function\n");

  const script = {
    label: LABELS.status,
    name: "Get Users By Status",
    version: "1.0",
    parameters: {
      status: {
        required: false,
        default: "active",
        description: "Filter by user status",
      },
      limit: {
        required: false,
        default: 10,
        description: "Maximum number of results",
      },
    },
    functions: [
      {
        type: "Query",
        collection: COLLECTION,
        filter: {
          type: "Condition",
          content: {
            field: "status",
            operator: "Eq",
            value: "{{status}}",
          },
        },
        limit: "{{limit}}",
      },
    ],
    tags: ["users", "parameterized"],
  };

  const scriptId = await saveOrUpdate(client, script);
  console.log("✅ Function saved");

  // Call with parameters
  const result = await client.callFunction(scriptId, {
    status: "active",
    limit: 3,
  });
  if (result.records.length !== 3) {
    throw new Error(
      `Parameterized function returned ${result.records.length} records; expected 3`,
    );
  }
  console.log(`📊 Found ${result.records.length} users (limited)`);
  console.log(`⏱️  Execution time: ${result.stats.execution_time_ms}ms\n`);
  return scriptId;
}

async function aggregationScript(client) {
  console.log("📝 Example 3: Aggregation Function\n");

  const script = {
    label: LABELS.stats,
    name: "User Statistics",
    version: "1.0",
    parameters: {},
    functions: [
      Stage.findAll(COLLECTION),
      Stage.group(
        ["status"],
        [
          {
            output_field: "count",
            operation: "Count",
          },
          {
            output_field: "avg_score",
            operation: "Average",
            input_field: "score",
          },
        ],
      ),
    ],
    tags: ["analytics"],
  };

  const scriptId = await saveOrUpdate(client, script);
  console.log("✅ Function saved");

  const result = await client.callFunction(scriptId);
  if (result.records.length !== 2) {
    throw new Error(
      `Aggregation function returned ${result.records.length} groups; expected 2`,
    );
  }
  console.log(`📊 Statistics: ${result.records.length} groups`);
  result.records.forEach((record) => {
    console.log(`   ${JSON.stringify(record)}`);
  });
  console.log(`⏱️  Execution time: ${result.stats.execution_time_ms}ms\n`);

  return scriptId;
}

async function scriptManagement(client, getActiveUsersId, userStatsId) {
  console.log("📝 Example 4: Function Management\n");

  // List all scripts
  const scripts = await client.listFunctions();
  console.log(`📋 Total functions: ${scripts.length}`);

  // Get specific script (use encrypted ID)
  const script = await client.getFunction(getActiveUsersId);
  console.log(`🔍 Retrieved function: ${script.name}`);

  // Update script (use encrypted ID)
  const updated = {
    label: LABELS.active,
    name: "Get Active Users (Updated)",
    description: "Updated description",
    version: "1.1",
    parameters: {},
    functions: [Stage.findAll(COLLECTION)],
    tags: ["users"],
  };
  await client.updateFunction(getActiveUsersId, updated);
  console.log("✏️  Function updated");

  // Delete script (use ID). Any failure is part of the demonstrated operation.
  await client.deleteFunction(userStatsId);
  console.log("🗑️  Function deleted");
  console.log();

  console.log(
    "ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID",
  );
  console.log("ℹ️  Only CALL can use either ID or label\n");
}

async function multiStageScript(client) {
  console.log("📝 Example 5: Multi-Stage Pipeline\n");

  const script = {
    label: LABELS.top,
    name: "Top Performing Users",
    version: "1.0",
    parameters: {
      min_score: {
        required: false,
        default: 50,
      },
    },
    functions: [
      {
        type: "Query",
        collection: COLLECTION,
        filter: {
          type: "Condition",
          content: {
            field: "score",
            operator: "Gte",
            value: "{{min_score}}",
          },
        },
      },
      Stage.project(["name", "score", "status"], false),
    ],
    tags: ["analytics", "reporting"],
  };

  const scriptId = await saveOrUpdate(client, script);
  console.log("✅ Multi-stage function saved");

  const result = await client.callFunction(scriptId, { min_score: 50 });
  console.log(`📊 Pipeline executed ${result.stats.stages_executed} stages`);
  console.log(`⏱️  Total execution time: ${result.stats.execution_time_ms}ms`);
  console.log("📈 Stage breakdown:");
  result.stats.stage_stats.forEach((stage, index) => {
    console.log(
      `   ${index + 1}. ${stage.stage}: ${stage.execution_time_ms}ms (${stage.input_count} → ${stage.output_count} records)`,
    );
  });
  console.log();
  return scriptId;
}

async function countScript(client) {
  console.log("📝 Example 6: Count Users\n");

  const script = {
    label: LABELS.count,
    name: "Count All Users",
    version: "1.0",
    parameters: {},
    functions: [Stage.findAll(COLLECTION), Stage.count()],
    tags: ["users", "count"],
  };

  const scriptId = await saveOrUpdate(client, script);
  console.log("✅ Count function saved");

  const result = await client.callFunction(scriptId);
  const count =
    result.records[0]?.count?.value || result.records[0]?.count || 0;
  if (count !== 10) {
    throw new Error(`Count function returned ${count}; expected 10`);
  }
  console.log(`📊 Total user count: ${count}`);
  console.log(`⏱️  Execution time: ${result.stats.execution_time_ms}ms\n`);
  return scriptId;
}

async function cleanup(client, scriptIds) {
  console.log("🧹 Cleaning up...");

  const results = await Promise.allSettled([
    client.deleteCollection(COLLECTION),
    ...scriptIds.map((id) => client.deleteFunction(id)),
  ]);
  const failures = results.filter(
    (result) =>
      result.status === "rejected" &&
      !String(result.reason).includes("not found"),
  );
  if (failures.length > 0) {
    throw new AggregateError(
      failures.map((failure) => failure.reason),
      "Function cleanup failed",
    );
  }
  console.log("✅ Deleted collection");
  console.log("✅ Deleted test functions\n");
}

async function main() {
  console.log("🚀 ekoDB Functions Example (JavaScript Client)\n");

  // Create and initialize ekoDB client
  const client = new EkoDBClient(BASE_URL, API_KEY);
  const scriptIds = [];
  let primaryError;
  try {
    await client.init();
    console.log("✅ Client initialized (token exchange automatic)\n");

    await client.deleteCollection(COLLECTION).catch(() => undefined);
    await setupTestData(client);
    const getActiveUsersId = await simpleQueryScript(client);
    scriptIds.push(getActiveUsersId);
    scriptIds.push(await parameterizedScript(client));
    const userStatsId = await aggregationScript(client);
    scriptIds.push(userStatsId);
    await scriptManagement(client, getActiveUsersId, userStatsId);
    scriptIds.push(await multiStageScript(client));
    scriptIds.push(await countScript(client));
    console.log("✅ All examples completed successfully!");
    console.log("\n💡 Key Advantages of Using the Client:");
    console.log("   • Automatic token management");
    console.log("   • Type-safe Stage builders");
    console.log("   • ChatMessage helpers");
    console.log("   • Cleaner, more maintainable code");
    console.log("   • Built-in error handling");
  } catch (error) {
    primaryError = error;
    throw error;
  } finally {
    try {
      await cleanup(client, scriptIds);
    } catch (cleanupError) {
      if (primaryError !== undefined) {
        throw new AggregateError(
          [primaryError, cleanupError],
          "Functions example and cleanup failed",
        );
      }
      throw cleanupError;
    }
  }
}

main().catch((error) => {
  console.error("❌ Error:", error);
  process.exit(1);
});
