/**
 * Functions Example using Direct HTTP Requests
 *
 * Demonstrates using functions with raw HTTP/fetch API
 * No client library required
 */

const dotenv = require("dotenv");

dotenv.config();

const BASE_URL = process.env.API_BASE_URL || "http://localhost:8080";
const API_KEY = process.env.API_BASE_KEY || "a-test-api-key-from-ekodb";
const TEST_COLLECTION = "functions_users_http_js";
const FUNCTION_LABELS = [
  "get_active_users_http_js",
  "get_active_users_paginated_http_js",
  "user_stats_http_js",
];

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

// Helper: Save a function idempotently via raw HTTP.
//
// The server returns HTTP 409 ("A function with label 'X' already exists.")
// when a function with the same fixed label already exists. On a 409 we issue
// PUT /api/functions/{label} with the same body (the GET/PUT/DELETE routes
// accept either the encrypted ID or the label), then GET it back by label so
// the caller still receives the function (including its encrypted `id`) for the
// downstream management flow. Non-409 errors are propagated.
async function saveOrUpdateFunction(func) {
  const token = await getAuthToken();
  const response = await fetch(`${BASE_URL}/api/functions`, {
    method: "POST",
    headers: {
      Authorization: `Bearer ${token}`,
      "Content-Type": "application/json",
    },
    body: JSON.stringify(func),
  });

  if (response.ok) {
    return response.json();
  }

  if (response.status === 409) {
    // Update the existing function by label, then read it back by label.
    await request("PUT", `/api/functions/${func.label}`, func);
    console.log(
      `ℹ️  Function '${func.label}' already existed — updated instead`,
    );
    return request("GET", `/api/functions/${func.label}`);
  }

  throw new Error(`HTTP ${response.status}: ${await response.text()}`);
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

    await request("POST", `/api/insert/${TEST_COLLECTION}`, record);
  }

  console.log("✅ Test data ready\n");
}

async function simpleQueryFunction() {
  console.log("📝 Example 1: Simple Query Function with Filter\n");

  const function1 = {
    label: FUNCTION_LABELS[0],
    name: "Get Active Users",
    description: "Query users with active status",
    version: "1.0",
    parameters: {},
    functions: [
      {
        type: "Query",
        collection: TEST_COLLECTION,
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

  // Save script (idempotent: updates by label on a 409 "already exists")
  const saveResult = await saveOrUpdateFunction(function1);
  console.log(`✅ Function saved: ${saveResult.id}`);

  // Call by the returned ID so execution does not depend on label-index visibility.
  const callResult = await request(
    "POST",
    `/api/functions/${saveResult.id}`,
    {},
  );
  if (callResult.records.length !== 5) {
    throw new Error(
      `Active-user query returned ${callResult.records.length} records; expected 5`,
    );
  }
  console.log(`📊 Found ${callResult.records.length} active users\n`);

  return saveResult.id;
}

async function parameterizedPaginationFunction() {
  console.log("📝 Example 2: Parameterized Pagination with Limit/Skip\n");

  const function2 = {
    label: FUNCTION_LABELS[1],
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
        collection: TEST_COLLECTION,
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

  const saveResult = await saveOrUpdateFunction(function2);
  console.log(`✅ Function saved: ${saveResult.id}`);

  // Call with page 1 (first 3 users)
  let callResult = await request("POST", `/api/functions/${saveResult.id}`, {
    page_size: 3,
    page_offset: 0,
  });
  if (callResult.records.length !== 3) {
    throw new Error(
      `Page 1 returned ${callResult.records.length} records; expected 3`,
    );
  }
  console.log(
    `📊 Page 1: Found ${callResult.records.length} users (limit=3, skip=0)`,
  );

  // Call with page 2 (next 3 users)
  callResult = await request("POST", `/api/functions/${saveResult.id}`, {
    page_size: 3,
    page_offset: 3,
  });
  if (callResult.records.length !== 2) {
    throw new Error(
      `Page 2 returned ${callResult.records.length} records; expected 2`,
    );
  }
  console.log(
    `📊 Page 2: Found ${callResult.records.length} users (limit=3, skip=3)\n`,
  );
  return saveResult.id;
}

async function aggregationFunction() {
  console.log(
    "📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)\n",
  );

  const function3 = {
    label: FUNCTION_LABELS[2],
    name: "User Statistics by Status",
    version: "1.0",
    parameters: {},
    functions: [
      {
        type: "Query",
        collection: TEST_COLLECTION,
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

  const saveResult = await saveOrUpdateFunction(function3);
  console.log(`✅ Function saved: ${saveResult.id}`);

  const callResult = await request(
    "POST",
    `/api/functions/${saveResult.id}`,
    {},
  );
  if (callResult.records.length !== 2) {
    throw new Error(
      `Status aggregation returned ${callResult.records.length} groups; expected 2`,
    );
  }
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
  console.log(`📋 Total functions: ${scripts.length}`);

  // Get specific script (requires encrypted ID)
  const script = await request("GET", `/api/functions/${getActiveUsersId}`);
  console.log(`🔍 Retrieved function: ${script.name}`);

  // Update script (requires encrypted ID)
  const updated = {
    label: FUNCTION_LABELS[0],
    name: "Get Active Users (Updated)",
    description: "Updated description",
    version: "1.1",
    parameters: {},
    functions: [{ type: "FindAll", collection: TEST_COLLECTION }],
    tags: ["users"],
  };
  await request("PUT", `/api/functions/${getActiveUsersId}`, updated);
  console.log("✏️  Function updated");

  // Delete script (requires encrypted ID)
  await request("DELETE", `/api/functions/${userStatsId}`);
  console.log("🗑️  Function deleted\n");

  console.log(
    "ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID",
  );
  console.log("ℹ️  Only CALL can use either ID or label\n");
}

async function main() {
  console.log("🚀 ekoDB Functions Example (JavaScript/HTTP)\n");

  let primaryError;
  try {
    await request("DELETE", `/api/collections/${TEST_COLLECTION}`).catch(
      (error) => {
        if (!error.message.includes("HTTP 404")) throw error;
      },
    );
    await setupTestData();
    const getActiveUsersId = await simpleQueryFunction();
    await parameterizedPaginationFunction();
    const userStatsId = await aggregationFunction();
    await functionManagement(getActiveUsersId, userStatsId);
    console.log("✅ All examples completed!");
  } catch (error) {
    primaryError = error;
    throw error;
  } finally {
    let functionIds = [];
    let discoveryError;
    try {
      const functions = await request("GET", "/api/functions");
      functionIds = functions
        .filter((fn) => FUNCTION_LABELS.includes(fn.label))
        .map((fn) => fn.id);
    } catch (error) {
      discoveryError = error;
    }
    const cleanup = await Promise.allSettled([
      ...functionIds.map((id) => request("DELETE", `/api/functions/${id}`)),
      request("DELETE", `/api/collections/${TEST_COLLECTION}`).catch(
        (error) => {
          if (
            !error.message.includes("HTTP 404") &&
            !error.message.includes("not found")
          )
            throw error;
        },
      ),
    ]);
    const failures = cleanup.filter((result) => result.status === "rejected");
    if (discoveryError)
      failures.push({ status: "rejected", reason: discoveryError });
    if (failures.length > 0) {
      const errors = failures.map((failure) => failure.reason);
      if (primaryError !== undefined) errors.unshift(primaryError);
      throw new AggregateError(
        errors,
        primaryError === undefined
          ? "HTTP functions cleanup failed"
          : "HTTP functions example and cleanup failed",
      );
    }
  }
}

main().catch((error) => {
  console.error("❌ Error:", error);
  process.exit(1);
});
