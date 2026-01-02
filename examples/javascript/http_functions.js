/**
 * Scripts Example using Direct HTTP Requests
 *
 * Demonstrates using scripts with raw HTTP/fetch API
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
  console.log("📝 Example 1: Simple Query Function with Filter\n");

  const function1 = {
    label: "get_active_users",
    name: "Get Active Users",
    description: "Query users with active status",
    version: "1.0",
    parameters: {},
    functions: [
      {
        type: "Query",
        collection: "users",
        filter: {
          type: "Condition",
          content: {
            field: "status",
            operator: "Eq",
            value: "active",
          },
        },
      },
    ],
    tags: ["users", "query"],
  };

  // Save script
  const saveResult = await request("POST", "/api/functions", function1);
  console.log(`✅ Script saved: ${saveResult.id}`);

  // Call script (can use label)
  const callResult = await request(
    "POST",
    "/api/functions/get_active_users",
    {},
  );
  console.log(`📊 Found ${callResult.records.length} active users\n`);

  return saveResult.id;
}

async function parameterizedPaginationFunction() {
  console.log("📝 Example 2: Parameterized Pagination with Limit/Skip\n");

  const function2 = {
    label: "get_active_users_paginated",
    name: "Get Active Users (Paginated)",
    version: "1.0",
    parameters: {
      page_size: {
        type: "Integer",
        required: false,
        default: 5,
      },
      page_offset: {
        type: "Integer",
        required: false,
        default: 0,
      },
    },
    functions: [
      {
        type: "Query",
        collection: "users",
        filter: {
          type: "Condition",
          content: {
            field: "status",
            operator: "Eq",
            value: "active",
          },
        },
        sort: [{ field: "score", ascending: false }],
        limit: "{{page_size}}",
        skip: "{{page_offset}}",
      },
    ],
    tags: ["users", "pagination"],
  };

  const saveResult = await request("POST", "/api/functions", function2);
  console.log(`✅ Script saved: ${saveResult.id}`);

  // Call with page 1 (first 3 users)
  let callResult = await request(
    "POST",
    "/api/functions/get_active_users_paginated",
    { page_size: 3, page_offset: 0 }
  );
  console.log(
    `📊 Page 1: Found ${callResult.records.length} users (limit=3, skip=0)`
  );

  // Call with page 2 (next 3 users)
  callResult = await request(
    "POST",
    "/api/functions/get_active_users_paginated",
    { page_size: 3, page_offset: 3 }
  );
  console.log(
    `📊 Page 2: Found ${callResult.records.length} users (limit=3, skip=3)\n`
  );
}

async function aggregationFunction() {
  console.log("📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)\n");

  const function3 = {
    label: "user_stats",
    name: "User Statistics by Status",
    version: "1.0",
    parameters: {},
    functions: [
      {
        type: "Query",
        collection: "users",
        filter: {
          type: "Condition",
          content: {
            field: "age",
            operator: "Gt",
            value: 20,
          },
        },
      },
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
          {
            output_field: "max_score",
            operation: "Max",
            input_field: "score",
          },
        ],
      },
    ],
    tags: ["analytics", "pipeline"],
  };

  const saveResult = await request("POST", "/api/functions", function3);
  console.log(`✅ Script saved: ${saveResult.id}`);

  const callResult = await request("POST", "/api/functions/user_stats", {});
  console.log(
    `📊 Pipeline Results: Filtered (age>20) → Grouped by status → ${callResult.records.length} groups`,
  );
  callResult.records.forEach((record) => {
    console.log(`   ${JSON.stringify(record)}`);
  });
  console.log();

  return saveResult.id;
}

async function functionManagement(getActiveUsersId, userStatsId) {
  console.log("📝 Example 4: Function Management\n");

  // List all scripts
  const scripts = await request("GET", "/api/functions");
  console.log(`📋 Total scripts: ${scripts.length}`);

  // Get specific script (requires encrypted ID)
  const script = await request("GET", `/api/functions/${getActiveUsersId}`);
  console.log(`🔍 Retrieved script: ${script.name}`);

  // Update script (requires encrypted ID)
  const updated = {
    label: "get_active_users",
    name: "Get Active Users (Updated)",
    description: "Updated description",
    version: "1.1",
    parameters: {},
    functions: [{ type: "FindAll", collection: "users" }],
    tags: ["users"],
  };
  await request("PUT", `/api/functions/${getActiveUsersId}`, updated);
  console.log("✏️  Script updated");

  // Delete script (requires encrypted ID)
  await request("DELETE", `/api/functions/${userStatsId}`);
  console.log("🗑️  Script deleted\n");

  console.log("ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID");
  console.log("ℹ️  Only CALL can use either ID or label\n");
}

async function main() {
  console.log("🚀 ekoDB Scripts Example (JavaScript/HTTP)\n");

  try {
    await setupTestData();
    const getActiveUsersId = await simpleQueryFunction();
    await parameterizedPaginationFunction();
    const userStatsId = await aggregationFunction();
    await functionManagement(getActiveUsersId, userStatsId);

    console.log("✅ All examples completed!");
  } catch (error) {
    console.error("❌ Error:", error.message);
    process.exit(1);
  }
}

main();
