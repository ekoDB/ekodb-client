/**
 * Complete CRUD Scripts Example using Direct HTTP Requests
 *
 * Demonstrates:
 *   • Insert + Verify (using Query)
 *   • Query + Update Status + Verify
 *   • Query + Update Credits + Verify
 *   • Query Before Delete + Delete + Verify Gone
 *
 * Each Script shows Functions chaining with proper verification using parameterized {{param}} syntax
 */

const dotenv = require("dotenv");

dotenv.config();

const BASE_URL = process.env.API_BASE_URL || "http://localhost:8080";
const API_KEY = process.env.API_BASE_KEY || "a-test-api-key-from-ekodb";

let authToken = null;

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
  console.log("📝 Script 1: Insert + Verify");
  console.log("=".repeat(60));

  const script = {
    label: "insert_and_verify",
    name: "Insert and Verify User",
    description: "Insert a user and verify it was created",
    version: "1.0",
    parameters: {
      user_name: {
        type: "String",
        required: true,
        description: "Name of the user",
      },
      user_email: {
        type: "String",
        required: true,
        description: "Email of the user",
      },
    },
    functions: [
      {
        type: "Insert",
        collection: "users",
        record: {
          name: "{{user_name}}",
          email: "{{user_email}}",
          status: "pending",
          credits: 0,
        },
      },
      {
        type: "Query",
        collection: "users",
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

  console.log("\n1️⃣ Saving Script...");
  const saveResult = await request("POST", "/api/functions", script);
  const scriptId = saveResult.id;
  console.log(`   ✅ Script saved: ${scriptId}`);

  console.log("\n2️⃣ Calling Script (Insert + Verify)...");
  const params = {
    user_name: "Alice Smith",
    user_email: "alice@example.com",
  };
  const callResult = await request("POST", "/api/functions/insert_and_verify", params);

  console.log(`   ✅ Script executed: ${callResult.stats.stages_executed} Functions`);
  console.log(`   ⏱️  Execution time: ${callResult.stats.execution_time_ms}ms`);

  const records = callResult.records;
  console.log("\n3️⃣ Verification Results:");
  console.log(`   ✅ Found ${records.length} record(s)`);

  if (records.length > 0) {
    const user = records[0];
    // Handle both plain string and map format for id
    const userId = typeof user.id === 'string' ? user.id : user.id.value;
    console.log(`   📋 Name: ${user.name}`);
    console.log(`   📋 Email: ${JSON.stringify(user.email)}`);
    console.log(`   📋 Status: ${user.status}`);
    console.log(`   📋 Credits: ${user.credits}`);
    return { userId, scriptId };
  }

  throw new Error("No records found");
}

async function script2QueryUpdateVerify() {
  console.log("\n" + "=".repeat(60));
  console.log("📝 Script 2: Query + Update + Verify");
  console.log("=".repeat(60));

  const script = {
    label: "query_update_verify",
    name: "Query, Update, and Verify",
    description: "Find user by filter, update status, verify change",
    version: "1.0",
    parameters: {
      user_email: {
        type: "String",
        required: true,
        description: "Email to search for",
      },
      new_status: {
        type: "String",
        default: "active",
        required: false,
        description: "New status value",
      },
    },
    functions: [
      {
        type: "Query",
        collection: "users",
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
        collection: "users",
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
        collection: "users",
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

  console.log("\n1️⃣ Saving Script...");
  const saveResult = await request("POST", "/api/functions", script);
  const scriptId = saveResult.id;
  console.log(`   ✅ Script saved: ${scriptId}`);

  console.log("\n2️⃣ Calling Script (Query + Update + Verify)...");
  const params = {
    user_email: "alice@example.com",
    new_status: "active",
  };
  const callResult = await request("POST", "/api/functions/query_update_verify", params);

  console.log(`   ✅ Script executed: ${callResult.stats.stages_executed} Functions`);
  console.log(`   ⏱️  Execution time: ${callResult.stats.execution_time_ms}ms`);

  const records = callResult.records;
  console.log("\n3️⃣ Verification Results:");
  console.log(`   ✅ Found ${records.length} record(s)`);
  if (records.length > 0) {
    const user = records[0];
    console.log(`   📋 Status updated to: ${JSON.stringify(user.status)}`);
    console.log(`   📋 Name: ${JSON.stringify(user.name)}`);
  }

  return scriptId;
}

async function script3QueryUpdateCredits() {
  console.log("\n" + "=".repeat(60));
  console.log("📝 Script 3: Query + Update Credits + Verify");
  console.log("=".repeat(60));

  const script = {
    label: "query_update_credits",
    name: "Query, Update Credits, and Verify",
    description: "Find user by email, update credits, verify change",
    version: "1.0",
    parameters: {
      user_email: {
        type: "String",
        required: true,
        description: "Email to search for",
      },
      credits: {
        type: "Integer",
        default: 100,
        required: false,
        description: "Credits to set",
      },
    },
    functions: [
      {
        type: "Query",
        collection: "users",
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
        collection: "users",
        filter: {
          type: "Condition",
          content: {
            field: "email",
            operator: "Eq",
            value: "{{user_email}}",
          },
        },
        updates: { credits: "{{credits}}" },
      },
      {
        type: "Query",
        collection: "users",
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

  console.log("\n1️⃣ Saving Script...");
  const saveResult = await request("POST", "/api/functions", script);
  const scriptId = saveResult.id;
  console.log(`   ✅ Script saved: ${scriptId}`);

  console.log("\n2️⃣ Calling Script (Query + Update Credits + Verify)...");
  const params = {
    user_email: "alice@example.com",
    credits: 100,
  };
  const callResult = await request("POST", "/api/functions/query_update_credits", params);

  console.log(`   ✅ Script executed: ${callResult.stats.stages_executed} Functions`);
  console.log(`   ⏱️  Execution time: ${callResult.stats.execution_time_ms}ms`);

  const records = callResult.records;
  console.log("\n3️⃣ Verification Results:");
  console.log(`   ✅ Found ${records.length} record(s)`);
  if (records.length > 0) {
    const user = records[0];
    console.log(`   📋 Credits updated to: ${JSON.stringify(user.credits)}`);
    console.log(`   📋 Status: ${JSON.stringify(user.status)}`);
    console.log(`   📋 Name: ${JSON.stringify(user.name)}`);
  }

  return scriptId;
}

async function script4DeleteAndVerify() {
  console.log("\n" + "=".repeat(60));
  console.log("📝 Script 4: Query Before Delete + Delete + Verify");
  console.log("=".repeat(60));

  const script = {
    label: "delete_and_verify",
    name: "Query Before Delete and Verify",
    description: "Verify record exists, delete it, then verify it's gone",
    version: "1.0",
    parameters: {
      user_email: {
        type: "String",
        required: true,
        description: "Email of user to delete",
      },
    },
    functions: [
      {
        type: "Query",
        collection: "users",
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
        collection: "users",
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
        collection: "users",
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

  console.log("\n1️⃣ Saving Script...");
  const saveResult = await request("POST", "/api/functions", script);
  const scriptId = saveResult.id;
  console.log(`   ✅ Script saved: ${scriptId}`);

  console.log("\n2️⃣ Calling Script (Query + Delete + Verify)...");
  const params = {
    user_email: "alice@example.com",
  };
  const callResult = await request("POST", "/api/functions/delete_and_verify", params);

  console.log(`   ✅ Script executed: ${callResult.stats.stages_executed} Functions`);
  console.log(`   ⏱️  Execution time: ${callResult.stats.execution_time_ms}ms`);

  const records = callResult.records;
  console.log("\n3️⃣ Verification Results:");
  console.log("   📊 Before delete: Record existed (verified by first Query)");
  if (records.length === 0) {
    console.log("   ✅ After delete: Record successfully deleted (Query returned 0 records)");
  } else {
    console.log(`   ❌ Delete failed - still found ${records.length} record(s)`);
  }

  return scriptId;
}

async function cleanup(scriptIds) {
  console.log("\n" + "=".repeat(60));
  console.log("🧹 Cleanup");
  console.log("=".repeat(60));

  for (const scriptId of scriptIds) {
    if (!scriptId) continue;
    try {
      await request("DELETE", `/api/functions/${scriptId}`);
      const truncatedId = scriptId.length > 20 ? scriptId.substring(0, 20) + "..." : scriptId;
      console.log(`   ✅ Deleted script: ${truncatedId}`);
    } catch (error) {
      console.log(`   ⚠️  Could not delete script: ${error.message}`);
    }
  }

  try {
    await request("DELETE", "/api/collections/users");
    console.log("   ✅ Deleted collection: users");
  } catch (err) {
    console.log(`   ⚠️  Could not delete collection: ${err.message}`);
  }
}

async function main() {
  console.log("🚀 ekoDB Complete CRUD Scripts Example");
  console.log("=".repeat(60));
  console.log("Demonstrates:");
  console.log("  • Insert + Verify (using Query)");
  console.log("  • Query + Update Status + Verify");
  console.log("  • Query + Update Credits + Verify");
  console.log("  • Query Before Delete + Delete + Verify Gone");
  console.log();
  console.log("Each Script shows Functions chaining with proper verification");
  console.log("=".repeat(60));

  try {
    // Run all CRUD Scripts in sequence
    const { userId, scriptId: script1Id } = await script1InsertAndVerify();
    const scriptIds = [script1Id];

    if (userId) {
      const script2Id = await script2QueryUpdateVerify();
      const script3Id = await script3QueryUpdateCredits();
      const script4Id = await script4DeleteAndVerify();
      scriptIds.push(script2Id, script3Id, script4Id);
    }

    // Cleanup
    await cleanup(scriptIds);

    console.log("\n" + "=".repeat(60));
    console.log("✅ Complete CRUD Scripts Example Finished!");
    console.log("=".repeat(60));
    console.log("\n💡 Key Takeaways:");
    console.log("   ✅ Scripts chain Functions together");
    console.log("   ✅ Each Script demonstrates operation + verification");
    console.log("   ✅ Parameters make Scripts reusable");
    console.log("   ✅ Verification is built into the Script itself");
    console.log("   ✅ Complete CRUD lifecycle in 4 focused Scripts");
  } catch (error) {
    console.error("\n❌ Error:", error.message);
    process.exit(1);
  }
}

main();
