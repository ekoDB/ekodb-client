/**
 * CRUD Functions Example - Basic Data Operations
 *
 * Demonstrates basic CRUD operations using functions:
 * - FindAll queries
 * - Group aggregations
 * - Simple data transformations
 */

const { EkoDBClient, Stage } = require("@ekodb/ekodb-client");
const dotenv = require("dotenv");

dotenv.config();

const BASE_URL = process.env.API_BASE_URL || "http://localhost:8080";
const API_KEY = process.env.API_BASE_KEY || "a-test-api-key-from-ekodb";
const RUN_SUFFIX = `${process.pid}_${Date.now()}`;
const COLLECTION = `crud_users_js_${RUN_SUFFIX}`;
const LABELS = {
  list: `list_all_users_js_${RUN_SUFFIX}`,
  status: `users_by_status_js_${RUN_SUFFIX}`,
  role: `avg_score_by_role_js_${RUN_SUFFIX}`,
  top: `top_users_js_${RUN_SUFFIX}`,
  summary: `user_summary_js_${RUN_SUFFIX}`,
};

function scalar(value) {
  return value && typeof value === "object" && "value" in value
    ? value.value
    : value;
}

function requireRecordCount(result, expected, label) {
  if (!Array.isArray(result.records))
    throw new Error(`${label} response omitted records`);
  if (result.records.length !== expected) {
    throw new Error(
      `${label} returned ${result.records.length} records; expected ${expected}`,
    );
  }
  return result.records;
}

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
      await client.updateFunction(script.label, script);
      console.log(
        `ℹ️  Function '${script.label}' already existed — updated instead`,
      );
      const existing = await client.getFunction(script.label);
      return existing.id;
    }
    throw error;
  }
}

async function setupTestData(client) {
  console.log("📋 Setting up test data...");

  const users = [];
  for (let i = 1; i <= 10; i++) {
    const user = await client.insert(COLLECTION, {
      name: `User ${i}`,
      email: `user${i}@example.com`,
      status: i % 3 === 0 ? "inactive" : "active",
      score: i * 10,
      role: i <= 3 ? "admin" : "user",
    });
    users.push(user.id);
  }

  console.log(`✅ Created ${users.length} test users\n`);
  return users;
}

async function listUsersScript(client) {
  console.log("📝 Example 1: List All Users\n");

  const script = {
    label: LABELS.list,
    name: "List All Users",
    version: "1.0",
    parameters: {},
    functions: [Stage.findAll(COLLECTION)],
    tags: ["users", "list"],
  };

  const scriptId = await saveOrUpdate(client, script);
  console.log("✅ Function saved");

  const result = await client.callFunction(scriptId);
  const records = requireRecordCount(result, 10, "List users");
  console.log(`📊 Found ${records.length} users`);
  console.log(`⏱️  Execution time: ${result.stats.execution_time_ms}ms\n`);

  return scriptId;
}

async function countByStatusScript(client) {
  console.log("📝 Example 2: Count Users by Status\n");

  const script = {
    label: LABELS.status,
    name: "Users by Status",
    version: "1.0",
    parameters: {},
    functions: [
      Stage.findAll(COLLECTION),
      Stage.group(["status"], [{ output_field: "count", operation: "Count" }]),
    ],
    tags: ["users", "analytics"],
  };

  const scriptId = await saveOrUpdate(client, script);
  console.log("✅ Function saved");

  const result = await client.callFunction(scriptId);
  const records = requireRecordCount(result, 2, "Group users by status");
  const counts = Object.fromEntries(
    records.map((group) => [
      String(scalar(group.status)),
      Number(scalar(group.count)),
    ]),
  );
  if (counts.active !== 7 || counts.inactive !== 3) {
    throw new Error(
      `Status counts were ${JSON.stringify(counts)}; expected active=7 and inactive=3`,
    );
  }
  console.log(`📊 User counts by status:`);
  records.forEach((group) => {
    console.log(
      `   ${group.status?.value || group.status}: ${group.count?.value || group.count} users`,
    );
  });
  console.log(`⏱️  Execution time: ${result.stats.execution_time_ms}ms\n`);

  return scriptId;
}

async function avgScoreByRoleScript(client) {
  console.log("📝 Example 3: Average Score by Role\n");

  const script = {
    label: LABELS.role,
    name: "Average Score by Role",
    version: "1.0",
    parameters: {},
    functions: [
      Stage.findAll(COLLECTION),
      Stage.group(
        ["role"],
        [
          { output_field: "count", operation: "Count" },
          {
            output_field: "avg_score",
            operation: "Average",
            input_field: "score",
          },
        ],
      ),
    ],
    tags: ["users", "analytics"],
  };

  const scriptId = await saveOrUpdate(client, script);
  console.log("✅ Function saved");

  const result = await client.callFunction(scriptId);
  const records = requireRecordCount(result, 2, "Group users by role");
  const roles = Object.fromEntries(
    records.map((group) => [
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
  console.log(`📊 Average scores by role:`);
  records.forEach((group) => {
    console.log(
      `   ${group.role?.value || group.role}: ${(group.avg_score?.value || group.avg_score).toFixed(1)} (${group.count?.value || group.count} users)`,
    );
  });
  console.log(`⏱️  Execution time: ${result.stats.execution_time_ms}ms\n`);

  return scriptId;
}

async function topUsersScript(client) {
  console.log("📝 Example 4: Top Users by Score\n");

  const script = {
    label: LABELS.top,
    name: "Top Users by Score",
    version: "1.0",
    parameters: {},
    functions: [
      Stage.findAll(COLLECTION),
      Stage.project(["name", "email", "score"], false),
    ],
    tags: ["users", "leaderboard"],
  };

  const scriptId = await saveOrUpdate(client, script);
  console.log("✅ Function saved");

  const result = await client.callFunction(scriptId);
  const records = requireRecordCount(result, 10, "Top users projection");
  console.log(`📊 Users (showing first 5 of ${records.length}):`);
  records.slice(0, 5).forEach((user, i) => {
    const name = user.name?.value || user.name || "Unknown";
    const score = user.score?.value || user.score || 0;
    console.log(`   ${i + 1}. ${name} - Score: ${score}`);
  });
  console.log(`⏱️  Execution time: ${result.stats.execution_time_ms}ms\n`);

  return scriptId;
}

async function userSummaryScript(client) {
  console.log("📝 Example 5: User Summary Statistics\n");

  const script = {
    label: LABELS.summary,
    name: "User Summary Statistics",
    version: "1.0",
    parameters: {},
    functions: [
      Stage.findAll(COLLECTION),
      Stage.group(
        ["status"],
        [
          { output_field: "count", operation: "Count" },
          {
            output_field: "total_score",
            operation: "Sum",
            input_field: "score",
          },
        ],
      ),
    ],
    tags: ["users", "analytics", "summary"],
  };

  const scriptId = await saveOrUpdate(client, script);
  console.log("✅ Function saved");

  const result = await client.callFunction(scriptId);
  const records = requireRecordCount(result, 2, "User summary");
  const summary = Object.fromEntries(
    records.map((group) => [
      String(scalar(group.status)),
      {
        count: Number(scalar(group.count)),
        total: Number(scalar(group.total_score)),
      },
    ]),
  );
  if (
    summary.active?.count !== 7 ||
    summary.active?.total !== 370 ||
    summary.inactive?.count !== 3 ||
    summary.inactive?.total !== 180
  ) {
    throw new Error(
      `User summary was ${JSON.stringify(summary)}; expected exact status counts and totals`,
    );
  }
  console.log(`📊 User summary (${records.length} groups):`);
  records.forEach((group) => {
    // Debug: log raw group to see structure
    // console.log('DEBUG group:', JSON.stringify(group, null, 2));
    const status = group.status?.value || group.status || "unknown";
    const count = group.count?.value || group.count;
    const totalScore = group.total_score?.value || group.total_score;
    console.log(
      `   ${status}: ${count} users, Total Score: ${totalScore}`,
    );
  });
  console.log(`⏱️  Execution time: ${result.stats.execution_time_ms}ms\n`);

  return scriptId;
}

async function cleanup(client, scriptIds, primaryError) {
  console.log("🧹 Cleaning up...");

  const cleanupErrors = [];
  for (const id of scriptIds) {
    try {
      await client.deleteFunction(id);
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      if (!message.includes("status 404") && !/not found/i.test(message)) {
        cleanupErrors.push(error);
      }
    }
  }
  try {
    await client.deleteCollection(COLLECTION);
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    if (!message.includes("status 404") && !/not found/i.test(message))
      cleanupErrors.push(error);
  }
  if (cleanupErrors.length > 0) {
    if (primaryError) console.error("Cleanup errors:", cleanupErrors);
    else
      throw new AggregateError(cleanupErrors, "CRUD functions cleanup failed");
  } else {
    console.log("✅ Cleanup complete\n");
  }
}

async function main() {
  console.log("🚀 ekoDB CRUD Functions Example\n");

  const client = new EkoDBClient(BASE_URL, API_KEY);
  await client.init();
  const scriptIds = [];
  let primaryError;
  try {
    await setupTestData(client);
    scriptIds.push(await listUsersScript(client));
    scriptIds.push(await countByStatusScript(client));
    scriptIds.push(await avgScoreByRoleScript(client));
    scriptIds.push(await topUsersScript(client));
    scriptIds.push(await userSummaryScript(client));
  } catch (error) {
    primaryError = error;
    throw error;
  } finally {
    await cleanup(client, scriptIds, primaryError);
  }

  console.log("✅ All CRUD function examples completed!");
}

main().catch((error) => {
  console.error("❌ Error:", error);
  process.exitCode = 1;
});
