/**
 * Function Composition Example for ekoDB JavaScript Client
 *
 * Demonstrates calling Functions within Functions using CallFunction
 * Shows how to build reusable logic blocks and compose complex workflows
 */

const { EkoDBClient } = require("@ekodb/ekodb-client");
const dotenv = require("dotenv");

dotenv.config();

const API_BASE_URL = process.env.API_BASE_URL || "http://localhost:8080";
const API_BASE_KEY = process.env.API_BASE_KEY || "a-test-api-key-from-ekodb";
const RUN_SUFFIX = `${process.pid}_${Date.now()}`;
const TEST_COLLECTION = `function_composition_users_js_${RUN_SUFFIX}`;
const FETCH_USER_LABEL = `fc_fetch_user_js_${RUN_SUFFIX}`;
const GET_USER_WRAPPER_LABEL = `fc_get_user_wrapper_js_${RUN_SUFFIX}`;
const FETCH_AND_STORE_LABEL = `fc_fetch_store_user_js_${RUN_SUFFIX}`;
const SWR_USER_LABEL = `fc_swr_user_js_${RUN_SUFFIX}`;
const VALIDATE_USER_LABEL = `fc_validate_user_js_${RUN_SUFFIX}`;
const FETCH_SLIM_LABEL = `fc_fetch_slim_user_js_${RUN_SUFFIX}`;
const GET_VERIFIED_LABEL = `fc_get_verified_user_js_${RUN_SUFFIX}`;
const USER_CACHE_KEY = `fc:user_cache_js:${RUN_SUFFIX}:1`;

function isNotFoundError(error) {
  const message = error instanceof Error ? error.message : String(error);
  return message.includes("status 404") || /not found/i.test(message);
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
async function saveOrUpdate(client, script, functionIds) {
  let id;
  try {
    id = await client.saveFunction(script);
  } catch (error) {
    if (error.message && error.message.includes("already exists")) {
      await client.updateFunction(script.label, script);
      console.log(
        `ℹ️  Function '${script.label}' already existed — updated instead`,
      );
      const existing = await client.getFunction(script.label);
      id = existing.id || script.label;
    } else {
      throw error;
    }
  }
  functionIds.add(id);
  return id;
}

async function setupTestData(client) {
  console.log("📋 Setting up test data...\n");

  for (let i = 1; i <= 5; i++) {
    await client.insert(TEST_COLLECTION, {
      user_code: `user_${i}`,
      name: `User ${i}`,
      department: i <= 2 ? "engineering" : "sales",
      score: i * 20,
    });
  }

  console.log("✅ Test data ready\n");
}

async function basicCompositionExample(client, functionIds) {
  console.log("📝 Example 1: Basic Function Composition\n");
  console.log("Building reusable functions that call each other...\n");

  // Step 1: Create reusable "fetch_user" function
  const fetchUser = {
    label: FETCH_USER_LABEL,
    name: "Fetch user by code",
    parameters: { user_code: { required: true } },
    functions: [
      {
        type: "FindOne",
        collection: TEST_COLLECTION,
        key: "user_code",
        value: "{{user_code}}",
      },
    ],
  };

  await saveOrUpdate(client, fetchUser, functionIds);
  console.log("✅ Saved reusable function: fetch_user");

  // Step 2: Create wrapper that CALLS fetch_user
  const getUserWrapper = {
    label: GET_USER_WRAPPER_LABEL,
    name: "Wrapper that calls fetch_user",
    parameters: { user_code: { required: true } },
    functions: [
      {
        type: "CallFunction",
        function_label: FETCH_USER_LABEL,
        // params omitted - inherits user_code from parent scope
      },
      {
        type: "Project",
        fields: ["name", "department"],
        exclude: false,
      },
    ],
  };

  const getUserWrapperId = await saveOrUpdate(
    client,
    getUserWrapper,
    functionIds,
  );
  console.log(
    "✅ Saved composed function: get_user_wrapper (calls fetch_user + projects fields)\n",
  );

  // Step 3: Call the composed function
  const result = await client.callFunction(getUserWrapperId, {
    user_code: "user_1",
  });

  console.log("📊 Result from composed function:");
  console.log(`   Records: ${result.records.length}`);
  if (result.records.length > 0) {
    const record = result.records[0];
    console.log(`   Name: ${JSON.stringify(record.name)}`);
    console.log(`   Department: ${JSON.stringify(record.department)}\n`);
  }

  console.log("🎯 Key Benefit: fetch_user can be reused by ANY function!");
  console.log("   No code duplication, single source of truth\n");
}

async function swrCompositionExample(client, functionIds) {
  console.log("📝 Example 2: SWR Pattern with Function Composition\n");
  console.log(
    "Using KV cache + CallFunction for fast cache-aside pattern...\n",
  );

  // Step 1: Create reusable fetch and store function
  // Using jsonplaceholder.typicode.com - a reliable free API for testing
  // This function fetches from API and stores in KV cache
  const fetchAndStore = {
    label: FETCH_AND_STORE_LABEL,
    name: "Fetch user from API and cache in KV",
    parameters: { user_id: { required: true } },
    functions: [
      {
        type: "HttpRequest",
        url: "https://jsonplaceholder.typicode.com/users/{{user_id}}",
        method: "GET",
        headers: {
          Accept: "application/json",
        },
      },
      // Store in KV cache (much faster than collection for cache lookups)
      {
        type: "KvSet",
        key: `fc:user_cache_js:${RUN_SUFFIX}:{{user_id}}`,
        value: "{{http_response}}",
        ttl: 300, // 5 minute cache
      },
    ],
  };

  await saveOrUpdate(client, fetchAndStore, functionIds);
  console.log("✅ Saved reusable function: fetch_and_store_user (uses KV)");

  // Step 2: Create SWR function that CALLS the reusable function
  // Pattern: KV cache check → populate if missing → return
  const swrUser = {
    label: SWR_USER_LABEL,
    name: "SWR pattern for user data (KV-based)",
    parameters: { user_id: { required: true } },
    functions: [
      // Check KV cache first (O(1) lookup - much faster than FindById)
      {
        type: "KvGet",
        key: `fc:user_cache_js:${RUN_SUFFIX}:{{user_id}}`,
      },
      {
        type: "If",
        condition: {
          // KvGet returns { value: ... } on hit, { value: null } on miss
          // So we check if "value" is not null to detect cache hit
          type: "Not",
          value: {
            condition: {
              type: "FieldEquals",
              value: { field: "value", value: null },
            },
          },
        },
        then_functions: [
          // Cache hit - project the value field
          {
            type: "Project",
            fields: ["value"],
            exclude: false,
          },
        ],
        else_functions: [
          // Cache miss - call reusable function to fetch and store
          // Explicitly pass user_id to the function
          {
            type: "CallFunction",
            function_label: FETCH_AND_STORE_LABEL,
            params: { user_id: "{{user_id}}" },
          },
          // After storing, retrieve the cached value to return it
          {
            type: "KvGet",
            key: `fc:user_cache_js:${RUN_SUFFIX}:{{user_id}}`,
          },
          {
            type: "Project",
            fields: ["value"],
            exclude: false,
          },
        ],
      },
    ],
  };

  const swrUserId = await saveOrUpdate(client, swrUser, functionIds);
  console.log("✅ Saved SWR function using composition: swr_user\n");

  // Step 3: Test cache miss
  console.log("First call (cache miss - will fetch from API):");
  const start1 = Date.now();
  const result1 = await client.callFunction(swrUserId, { user_id: "1" });
  const duration1 = Date.now() - start1;

  console.log(`   ⏱️  Duration: ${duration1}ms`);
  console.log(`   📊 Records: ${result1.records.length}`);
  if (result1.records.length > 0) {
    const preview = JSON.stringify(result1.records[0], null, 2).substring(
      0,
      200,
    );
    console.log(`   📦 Data: ${preview}...\n`);
  } else {
    console.log();
  }

  // Step 4: Test cache hit
  console.log("Second call (cache hit - from cache):");
  const start2 = Date.now();
  const result2 = await client.callFunction(swrUserId, { user_id: "1" });
  const duration2 = Date.now() - start2;

  console.log(`   ⏱️  Duration: ${duration2}ms`);
  console.log(`   📊 Records: ${result2.records.length}`);
  if (result2.records.length > 0) {
    const preview = JSON.stringify(result2.records[0], null, 2).substring(
      0,
      200,
    );
    console.log(`   📦 Data: ${preview}...`);
  }
  if (duration2 > 0) {
    const speedup = duration1 / duration2;
    console.log(`   🚀 Cache speedup: ${speedup.toFixed(1)}x faster!\n`);
  }
}

async function nestedCompositionExample(client, functionIds) {
  console.log("📝 Example 3: Multi-Level Function Composition\n");
  console.log("Building complex workflows from small, reusable pieces...\n");

  // Level 1: Base function
  const validateUser = {
    label: VALIDATE_USER_LABEL,
    name: "Check if user exists",
    parameters: { user_code: { required: true } },
    functions: [
      {
        type: "FindOne",
        collection: TEST_COLLECTION,
        key: "user_code",
        value: "{{user_code}}",
      },
    ],
  };

  await saveOrUpdate(client, validateUser, functionIds);
  console.log("✅ Level 1 function: validate_user");

  // Level 2: Calls validate_user + projects
  const fetchSlim = {
    label: FETCH_SLIM_LABEL,
    name: "Validate and slim down user",
    parameters: { user_code: { required: true } },
    functions: [
      {
        type: "CallFunction",
        function_label: VALIDATE_USER_LABEL,
        // params omitted - inherits user_code from parent scope
      },
      {
        type: "Project",
        fields: ["name", "department"],
        exclude: false,
      },
    ],
  };

  await saveOrUpdate(client, fetchSlim, functionIds);
  console.log("✅ Level 2 function: fetch_slim_user (calls validate_user)");

  // Level 3: Calls fetch_slim (demonstrates 3-level nesting)
  const getVerifiedUser = {
    label: GET_VERIFIED_LABEL,
    name: "Get verified and validated user",
    parameters: { user_code: { required: true } },
    functions: [
      {
        type: "CallFunction",
        function_label: FETCH_SLIM_LABEL,
        // params omitted - inherits user_code from parent scope
      },
    ],
  };

  const getVerifiedUserId = await saveOrUpdate(
    client,
    getVerifiedUser,
    functionIds,
  );
  console.log(
    "✅ Level 3 function: get_verified_user (calls fetch_slim_user)\n",
  );

  // Execute 3-level nested composition
  const result = await client.callFunction(getVerifiedUserId, {
    user_code: "user_1",
  });

  console.log("📊 Result from 3-level nested composition:");
  console.log(`   Records: ${result.records.length}`);
  if (result.records.length > 0) {
    const record = result.records[0];
    const name = record.name?.value || record.name || "Unknown";
    const department =
      record.department?.value || record.department || "Unknown";
    console.log(`   Name: ${name}`);
    console.log(`   Department: ${department}\n`);
  }

  console.log(
    "🎯 Key Benefit: Each function is independently testable and reusable!",
  );
  console.log("   - validate_user: Used in 100 different workflows");
  console.log("   - fetch_slim_user: Used in 50 workflows");
  console.log("   - get_verified_user: Specific workflow\n");
}

async function main() {
  const client = new EkoDBClient(API_BASE_URL, API_BASE_KEY);
  const functionIds = new Set();
  let primaryError;

  console.log("=== ekoDB Function Composition Examples ===\n");

  try {
    await setupTestData(client);
    await basicCompositionExample(client, functionIds);
    await swrCompositionExample(client, functionIds);
    await nestedCompositionExample(client, functionIds);
  } catch (error) {
    primaryError = error;
    throw error;
  } finally {
    const cleanupErrors = [];
    for (const id of [...functionIds].reverse()) {
      try {
        await client.deleteFunction(id);
      } catch (error) {
        if (!isNotFoundError(error)) cleanupErrors.push(error);
      }
    }
    try {
      await client.kvDelete(USER_CACHE_KEY);
    } catch (error) {
      if (!isNotFoundError(error)) cleanupErrors.push(error);
    }
    try {
      await client.deleteCollection(TEST_COLLECTION);
    } catch (error) {
      if (!isNotFoundError(error)) cleanupErrors.push(error);
    }
    if (cleanupErrors.length > 0) {
      throw new AggregateError(
        primaryError === undefined
          ? cleanupErrors
          : [primaryError, ...cleanupErrors],
        "Function composition or cleanup failed",
      );
    }
  }

  console.log("\n✅ All composition examples completed!");
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
