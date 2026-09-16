/**
 * Complete CRUD Functions Example using Direct HTTP Requests
 *
 * Demonstrates:
 *   • Insert + Verify (using Query)
 *   • Query + Update Status + Verify
 *   • Query + Update Credits + Verify
 *   • Query Before Delete + Delete + Verify Gone
 *
 * Each function shows Functions chaining with proper verification using parameterized {{param}} syntax
 */

const dotenv = require("dotenv");

dotenv.config();

const BASE_URL = process.env.API_BASE_URL || "http://localhost:8080";
const API_KEY = process.env.API_BASE_KEY || "a-test-api-key-from-ekodb";
const TEST_COLLECTION = "crud_functions_users_js";
const FUNCTION_LABELS = [
  "insert_and_verify_js",
  "query_update_verify_js",
  "query_update_credits_js",
  "delete_and_verify_js",
];

let authToken = null;

function getValue(field) {
  return field &&
    typeof field === "object" &&
    "type" in field &&
    "value" in field
    ? field.value
    : field;
}

async function getAuthToken() {
  if (authToken) return authToken;

  const response = await fetch(`${BASE_URL}/api/auth/token`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ api_key: API_KEY }),
  });

  if (!response.ok) {
    throw new Error(`Auth failed: ${response.status}`);
  }

  const data = await response.json();
  authToken = data.token;
  return authToken;
}

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
    const text = await response.text();
    throw new Error(`HTTP ${response.status}: ${text}`);
  }

  return response.json();
}

async function script1InsertAndVerify() {
  console.log("\n" + "=".repeat(60));
  console.log("📝 Function 1: Insert + Verify");
  console.log("=".repeat(60));

  const script = {
    label: FUNCTION_LABELS[0],
    name: "Insert and Verify User",
    description: "Insert a user and verify it was created",
    version: "1.0",
    parameters: {
      user_name: {
        required: true,
        description: "Name of the user",
      },
      user_email: {
        required: true,
        description: "Email of the user",
      },
    },
    functions: [
      {
        type: "Insert",
        collection: TEST_COLLECTION,
        record: {
          name: "{{user_name}}",
          email: "{{user_email}}",
          status: "pending",
          credits: 0,
        },
      },
      {
        type: "Query",
        collection: TEST_COLLECTION,
        filter: {
          type: "Condition",
          content: {
            field: "email",
            operator: "Eq",
            value: "{{user_email}}",
          },
        },
      },
    ],
    tags: ["crud", "insert", "verification"],
  };

  console.log("\n1️⃣ Saving Function...");
  const saveResult = await request("POST", "/api/functions", script);
  const scriptId = saveResult.id;
  console.log(`   ✅ Function saved: ${scriptId}`);

  console.log("\n2️⃣ Calling Function (Insert + Verify)...");
  const params = {
    user_name: "Alice Smith",
    user_email: "alice@example.com",
  };
  const callResult = await request(
    "POST",
    `/api/functions/${scriptId}`,
    params,
  );

  console.log(
    `   ✅ Function executed: ${callResult.stats.stages_executed} Functions`,
  );
  console.log(`   ⏱️  Execution time: ${callResult.stats.execution_time_ms}ms`);

  const records = callResult.records;
  console.log("\n3️⃣ Verification Results:");
  console.log(`   ✅ Found ${records.length} record(s)`);

  if (records.length === 1) {
    const user = records[0];
    // Handle both plain string and map format for id
    const userId = typeof user.id === "string" ? user.id : user.id.value;
    console.log(`   📋 Name: ${getValue(user.name)}`);
    console.log(`   📋 Email: ${getValue(user.email)}`);
    console.log(`   📋 Status: ${getValue(user.status)}`);
    console.log(`   📋 Credits: ${getValue(user.credits)}`);
    return { userId, scriptId };
  }

  throw new Error(
    `Insert verification expected 1 record, found ${records.length}`,
  );
}

async function script2QueryUpdateVerify() {
  console.log("\n" + "=".repeat(60));
  console.log("📝 Function 2: Query + Update + Verify");
  console.log("=".repeat(60));

  const script = {
    label: FUNCTION_LABELS[1],
    name: "Query, Update, and Verify",
    description: "Find user by filter, update status, verify change",
    version: "1.0",
    parameters: {
      user_email: {
        required: true,
        description: "Email to search for",
      },
      new_status: {
        default: "active",
        required: false,
        description: "New status value",
      },
    },
    functions: [
      {
        type: "Query",
        collection: TEST_COLLECTION,
        filter: {
          type: "Condition",
          content: {
            field: "email",
            operator: "Eq",
            value: "{{user_email}}",
          },
        },
      },
      {
        type: "Update",
        collection: TEST_COLLECTION,
        filter: {
          type: "Condition",
          content: {
            field: "email",
            operator: "Eq",
            value: "{{user_email}}",
          },
        },
        updates: { status: "{{new_status}}" },
      },
      {
        type: "Query",
        collection: TEST_COLLECTION,
        filter: {
          type: "Condition",
          content: {
            field: "email",
            operator: "Eq",
            value: "{{user_email}}",
          },
        },
      },
    ],
    tags: ["crud", "update", "verification"],
  };

  console.log("\n1️⃣ Saving Function...");
  const saveResult = await request("POST", "/api/functions", script);
  const scriptId = saveResult.id;
  console.log(`   ✅ Function saved: ${scriptId}`);

  console.log("\n2️⃣ Calling Function (Query + Update + Verify)...");
  const params = {
    user_email: "alice@example.com",
    new_status: "active",
  };
  const callResult = await request(
    "POST",
    `/api/functions/${scriptId}`,
    params,
  );

  console.log(
    `   ✅ Function executed: ${callResult.stats.stages_executed} Functions`,
  );
  console.log(`   ⏱️  Execution time: ${callResult.stats.execution_time_ms}ms`);

  const records = callResult.records;
  console.log("\n3️⃣ Verification Results:");
  console.log(`   ✅ Found ${records.length} record(s)`);
  if (records.length !== 1)
    throw new Error(
      `Status verification expected 1 record, found ${records.length}`,
    );
  const user = records[0];
  if (getValue(user.status) !== "active")
    throw new Error(
      `Status verification expected active, found ${JSON.stringify(user.status)}`,
    );
  console.log(`   📋 Status updated to: ${JSON.stringify(user.status)}`);
  console.log(`   📋 Name: ${JSON.stringify(user.name)}`);

  return scriptId;
}

async function script3QueryUpdateCredits() {
  console.log("\n" + "=".repeat(60));
  console.log("📝 Function 3: Query + Update Credits + Verify");
  console.log("=".repeat(60));

  const script = {
    label: FUNCTION_LABELS[2],
    name: "Query, Update Credits, and Verify",
    description: "Find user by email, update credits, verify change",
    version: "1.0",
    parameters: {
      user_email: {
        required: true,
        description: "Email to search for",
      },
    },
    functions: [
      {
        type: "Query",
        collection: TEST_COLLECTION,
        filter: {
          type: "Condition",
          content: {
            field: "email",
            operator: "Eq",
            value: "{{user_email}}",
          },
        },
      },
      {
        type: "Update",
        collection: TEST_COLLECTION,
        filter: {
          type: "Condition",
          content: {
            field: "email",
            operator: "Eq",
            value: "{{user_email}}",
          },
        },
        updates: { credits: 100 },
      },
      {
        type: "Query",
        collection: TEST_COLLECTION,
        filter: {
          type: "Condition",
          content: {
            field: "email",
            operator: "Eq",
            value: "{{user_email}}",
          },
        },
      },
    ],
    tags: ["crud", "update", "verification"],
  };

  console.log("\n1️⃣ Saving Function...");
  const saveResult = await request("POST", "/api/functions", script);
  const scriptId = saveResult.id;
  console.log(`   ✅ Function saved: ${scriptId}`);

  console.log("\n2️⃣ Calling Function (Query + Update Credits + Verify)...");
  const params = {
    user_email: "alice@example.com",
  };
  const callResult = await request(
    "POST",
    `/api/functions/${scriptId}`,
    params,
  );

  console.log(
    `   ✅ Function executed: ${callResult.stats.stages_executed} Functions`,
  );
  console.log(`   ⏱️  Execution time: ${callResult.stats.execution_time_ms}ms`);

  const records = callResult.records;
  console.log("\n3️⃣ Verification Results:");
  console.log(`   ✅ Found ${records.length} record(s)`);
  if (records.length !== 1)
    throw new Error(
      `Credits verification expected 1 record, found ${records.length}`,
    );
  const user = records[0];
  if (getValue(user.credits) !== 100)
    throw new Error(
      `Credits verification expected 100, found ${JSON.stringify(user.credits)}`,
    );
  console.log(`   📋 Credits updated to: ${JSON.stringify(user.credits)}`);
  console.log(`   📋 Status: ${JSON.stringify(user.status)}`);
  console.log(`   📋 Name: ${JSON.stringify(user.name)}`);

  return scriptId;
}

async function script4DeleteAndVerify() {
  console.log("\n" + "=".repeat(60));
  console.log("📝 Function 4: Query Before Delete + Delete + Verify");
  console.log("=".repeat(60));

  const script = {
    label: FUNCTION_LABELS[3],
    name: "Query Before Delete and Verify",
    description: "Verify record exists, delete it, then verify it's gone",
    version: "1.0",
    parameters: {
      user_email: {
        required: true,
        description: "Email of user to delete",
      },
    },
    functions: [
      {
        type: "Query",
        collection: TEST_COLLECTION,
        filter: {
          type: "Condition",
          content: {
            field: "email",
            operator: "Eq",
            value: "{{user_email}}",
          },
        },
      },
      {
        type: "Delete",
        collection: TEST_COLLECTION,
        filter: {
          type: "Condition",
          content: {
            field: "email",
            operator: "Eq",
            value: "{{user_email}}",
          },
        },
      },
      {
        type: "Query",
        collection: TEST_COLLECTION,
        filter: {
          type: "Condition",
          content: {
            field: "email",
            operator: "Eq",
            value: "{{user_email}}",
          },
        },
      },
    ],
    tags: ["crud", "delete", "verification"],
  };

  console.log("\n1️⃣ Saving Function...");
  const saveResult = await request("POST", "/api/functions", script);
  const scriptId = saveResult.id;
  console.log(`   ✅ Function saved: ${scriptId}`);

  console.log("\n2️⃣ Calling Function (Query + Delete + Verify)...");
  const params = {
    user_email: "alice@example.com",
  };
  const callResult = await request(
    "POST",
    `/api/functions/${scriptId}`,
    params,
  );

  console.log(
    `   ✅ Function executed: ${callResult.stats.stages_executed} Functions`,
  );
  console.log(`   ⏱️  Execution time: ${callResult.stats.execution_time_ms}ms`);

  const records = callResult.records;
  console.log("\n3️⃣ Verification Results:");
  console.log("   📊 Before delete: Record existed (verified by first Query)");
  if (records.length === 0) {
    console.log(
      "   ✅ After delete: Record successfully deleted (Query returned 0 records)",
    );
  } else {
    throw new Error(`Delete failed - still found ${records.length} record(s)`);
  }

  return scriptId;
}

async function cleanup(scriptIds, allowMissingCollection = false) {
  console.log("\n" + "=".repeat(60));
  console.log("🧹 Cleanup");
  console.log("=".repeat(60));

  const errors = [];
  try {
    const functions = await request("GET", "/api/functions");
    for (const fn of functions) {
      if (FUNCTION_LABELS.includes(fn.label) && !scriptIds.includes(fn.id)) {
        scriptIds.push(fn.id);
      }
    }
  } catch (error) {
    errors.push(error);
  }
  for (const scriptId of scriptIds) {
    if (!scriptId) continue;
    try {
      await request("DELETE", `/api/functions/${scriptId}`);
      const truncatedId =
        scriptId.length > 20 ? scriptId.substring(0, 20) + "..." : scriptId;
      console.log(`   ✅ Deleted function: ${truncatedId}`);
    } catch (error) {
      console.log(`   ⚠️  Could not delete function: ${error.message}`);
      errors.push(error);
    }
  }

  try {
    await request("DELETE", `/api/collections/${TEST_COLLECTION}`);
    console.log(`   ✅ Deleted collection: ${TEST_COLLECTION}`);
  } catch (err) {
    const missing = /HTTP 404|not found/i.test(err.message);
    if (!allowMissingCollection || !missing) {
      console.log(`   ⚠️  Could not delete collection: ${err.message}`);
      errors.push(err);
    }
  }
  if (errors.length > 0)
    throw new AggregateError(
      errors,
      `Cleanup failed for ${errors.length} resource(s)`,
    );
}

async function main() {
  console.log("🚀 ekoDB Complete CRUD Functions Example");
  console.log("=".repeat(60));
  console.log("Demonstrates:");
  console.log("  • Insert + Verify (using Query)");
  console.log("  • Query + Update Status + Verify");
  console.log("  • Query + Update Credits + Verify");
  console.log("  • Query Before Delete + Delete + Verify Gone");
  console.log();
  console.log(
    "Each function shows Functions chaining with proper verification",
  );
  console.log("=".repeat(60));

  const scriptIds = [];
  let primaryError;
  try {
    await cleanup(scriptIds, true);
    scriptIds.length = 0;
    // Run all CRUD Functions in sequence
    const { userId, scriptId: script1Id } = await script1InsertAndVerify();
    scriptIds.push(script1Id);

    if (userId) {
      const script2Id = await script2QueryUpdateVerify();
      const script3Id = await script3QueryUpdateCredits();
      const script4Id = await script4DeleteAndVerify();
      scriptIds.push(script2Id, script3Id, script4Id);
    }
  } catch (error) {
    primaryError = error;
    throw error;
  } finally {
    try {
      await cleanup(scriptIds);
    } catch (cleanupError) {
      if (primaryError !== undefined) {
        throw new AggregateError(
          [primaryError, cleanupError],
          "CRUD functions example and cleanup failed",
        );
      }
      throw cleanupError;
    }
  }

  console.log("\n" + "=".repeat(60));
  console.log("✅ Complete CRUD Functions Example Finished!");
  console.log("=".repeat(60));
  console.log("\n💡 Key Takeaways:");
  console.log("   ✅ Functions chain Functions together");
  console.log("   ✅ Each function demonstrates operation + verification");
  console.log("   ✅ Parameters make functions reusable");
  console.log("   ✅ Verification is built into the function itself");
  console.log("   ✅ Complete CRUD lifecycle in 4 focused functions");
}

main().catch((error) => {
  console.error("\n❌ Error:", error);
  process.exit(1);
});
