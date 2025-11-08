/**
 * Saved Functions Example using Direct HTTP Requests
 *
 * Demonstrates using saved functions with raw HTTP/fetch API
 * No client library required
 */

const dotenv = require("dotenv");

dotenv.config();

const BASE_URL = process.env.API_BASE_URL || "http://localhost:8080";
const API_KEY = process.env.API_BASE_KEY || "a-test-api-key-from-ekodb";

let authToken = null;

// Get authentication token
async function getAuthToken() {
  if (authToken) return authToken;

  const response = await fetch(`${BASE_URL}/api/auth/token`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({ api_key: API_KEY }),
  });

  if (!response.ok) {
    throw new Error(`Auth failed: ${response.status}`);
  }

  const data = await response.json();
  authToken = data.token;
  return authToken;
}

// Helper: Make authenticated HTTP request
async function request(method, path, body = null) {
  const token = await getAuthToken();
  const options = {
    method,
    headers: {
      Authorization: `Bearer ${token}`,
      "Content-Type": "application/json",
    },
  };

  if (body) {
    options.body = JSON.stringify(body);
  }

  const response = await fetch(`${BASE_URL}${path}`, options);

  if (!response.ok) {
    throw new Error(`HTTP ${response.status}: ${await response.text()}`);
  }

  return response.json();
}

async function setupTestData() {
  console.log("📋 Setting up test data...");

  for (let i = 1; i <= 10; i++) {
    const record = {
      name: `User ${i}`,
      age: 20 + i,
      status: i % 2 === 0 ? "active" : "inactive",
      score: i * 10,
    };

    await request("POST", "/api/insert/users", record);
  }

  console.log("✅ Test data ready\n");
}

async function simpleQueryFunction() {
  console.log("📝 Example 1: Simple Query Function\n");

  const function1 = {
    label: "get_active_users",
    name: "Get Active Users",
    description: "Retrieve all active users",
    version: "1.0",
    parameters: {},
    pipeline: [{ type: "FindAll", collection: "users" }],
    tags: ["users", "query"],
  };

  // Save function
  const saveResult = await request("POST", "/api/functions", function1);
  console.log(`✅ Function saved: ${saveResult.id}`);

  // Call function (can use label)
  const callResult = await request(
    "POST",
    "/api/functions/get_active_users",
    {},
  );
  console.log(`📊 Found ${callResult.records.length} active users\n`);

  return saveResult.id;
}

async function parameterizedFunction() {
  console.log("📝 Example 2: Parameterized Function\n");

  const function2 = {
    label: "get_users_by_status",
    name: "Get Users By Status",
    version: "1.0",
    parameters: {
      status: {
        param_type: "String",
        required: false,
        default: "active",
      },
      limit: {
        param_type: "Integer",
        required: false,
        default: 10,
      },
    },
    pipeline: [{ type: "FindAll", collection: "users" }],
    tags: ["users", "parameterized"],
  };

  const saveResult = await request("POST", "/api/functions", function2);
  console.log(`✅ Function saved: ${saveResult.id}`);

  // Call with parameters
  const callResult = await request(
    "POST",
    "/api/functions/get_users_by_status",
    { status: "active", limit: 3 },
  );
  console.log(`📊 Found ${callResult.records.length} users (limited)\n`);
}

async function aggregationFunction() {
  console.log("📝 Example 3: Aggregation Function\n");

  const function3 = {
    label: "user_stats",
    name: "User Statistics",
    version: "1.0",
    parameters: {},
    pipeline: [
      { type: "FindAll", collection: "users" },
      {
        type: "Group",
        by_fields: ["status"],
        functions: [
          { output_field: "count", operation: "Count" },
          {
            output_field: "avg_score",
            operation: "Average",
            input_field: "score",
          },
        ],
      },
    ],
    tags: ["analytics"],
  };

  const saveResult = await request("POST", "/api/functions", function3);
  console.log(`✅ Function saved: ${saveResult.id}`);

  const callResult = await request("POST", "/api/functions/user_stats", {});
  console.log(`📊 Statistics: ${callResult.records.length} groups`);
  callResult.records.forEach((record) => {
    console.log(`   ${JSON.stringify(record)}`);
  });
  console.log();

  return saveResult.id;
}

async function functionManagement(getActiveUsersId, userStatsId) {
  console.log("📝 Example 4: Function Management\n");

  // List all functions
  const functions = await request("GET", "/api/functions");
  console.log(`📋 Total functions: ${functions.length}`);

  // Get specific function (requires encrypted ID)
  const func = await request("GET", `/api/functions/${getActiveUsersId}`);
  console.log(`🔍 Retrieved function: ${func.name}`);

  // Update function (requires encrypted ID)
  const updated = {
    label: "get_active_users",
    name: "Get Active Users (Updated)",
    description: "Updated description",
    version: "1.1",
    parameters: {},
    pipeline: [{ type: "FindAll", collection: "users" }],
    tags: ["users"],
  };
  await request("PUT", `/api/functions/${getActiveUsersId}`, updated);
  console.log("✏️  Function updated");

  // Delete function (requires encrypted ID)
  await request("DELETE", `/api/functions/${userStatsId}`);
  console.log("🗑️  Function deleted\n");

  console.log("ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID");
  console.log("ℹ️  Only CALL can use either ID or label\n");
}

async function main() {
  console.log("🚀 ekoDB Saved Functions Example (JavaScript/HTTP)\n");

  try {
    await setupTestData();
    const getActiveUsersId = await simpleQueryFunction();
    await parameterizedFunction();
    const userStatsId = await aggregationFunction();
    await functionManagement(getActiveUsersId, userStatsId);

    console.log("✅ All examples completed!");
  } catch (error) {
    console.error("❌ Error:", error.message);
    process.exit(1);
  }
}

main();
