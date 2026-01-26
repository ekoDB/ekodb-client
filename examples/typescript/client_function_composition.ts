/**
 * Function Composition Example for ekoDB TypeScript Client
 *
 * Demonstrates calling Functions within Functions using CallFunction
 * Shows how to build reusable logic blocks and compose complex workflows
 */

import { EkoDBClient } from "@ekodb/ekodb-client";
import * as dotenv from "dotenv";

dotenv.config();

const API_BASE_URL = process.env.API_BASE_URL || "http://localhost:8080";
const API_BASE_KEY = process.env.API_BASE_KEY || "a-test-api-key-from-ekodb";

async function setupTestData(client: EkoDBClient): Promise<void> {
  console.log("📋 Setting up test data...\n");

  for (let i = 1; i <= 5; i++) {
    await client.insert("users", {
      user_code: `user_${i}`,
      name: `User ${i}`,
      department: i <= 2 ? "engineering" : "sales",
      score: i * 20,
    });
  }

  console.log("✅ Test data ready\n");
}

async function basicCompositionExample(client: EkoDBClient): Promise<void> {
  console.log("📝 Example 1: Basic Function Composition\n");
  console.log("Building reusable functions that call each other...\n");

  // Step 1: Create reusable "fetch_user" function
  const fetchUser = {
    label: "fetch_user",
    name: "Fetch user by code",
    parameters: {
      user_code: { required: true },
    },
    functions: [
      {
        type: "FindOne" as const,
        collection: "users",
        key: "user_code",
        value: "{{user_code}}",
      },
    ],
  };

  await client.saveScript(fetchUser);
  console.log("✅ Saved reusable function: fetch_user");

  // Step 2: Create wrapper that CALLS fetch_user
  const getUserWrapper = {
    label: "get_user_wrapper",
    name: "Wrapper that calls fetch_user",
    parameters: {
      user_code: { required: true },
    },
    functions: [
      {
        type: "CallFunction" as const,
        function_label: "fetch_user",
        // params omitted - inherits user_code from parent scope
      },
      {
        type: "Project" as const,
        fields: ["name", "department"],
        exclude: false,
      },
    ],
  };

  await client.saveScript(getUserWrapper);
  console.log(
    "✅ Saved composed function: get_user_wrapper (calls fetch_user + projects fields)\n",
  );

  // Step 3: Call the composed function
  const result = await client.callScript("get_user_wrapper", {
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

async function swrCompositionExample(client: EkoDBClient): Promise<void> {
  console.log("📝 Example 2: SWR Pattern with Function Composition\n");
  console.log("Using CallFunction to replace inline logic in SWR pattern...\n");

  // Step 1: Create reusable fetch and store function
  // Using jsonplaceholder.typicode.com - a reliable free API for testing
  // This function ONLY fetches and stores - the caller handles retrieval
  const fetchAndStore = {
    label: "fetch_and_store_user",
    name: "Fetch user from API and cache",
    parameters: {
      user_id: { required: true },
    },
    functions: [
      {
        type: "HttpRequest" as const,
        url: "https://jsonplaceholder.typicode.com/users/{{user_id}}",
        method: "GET",
        headers: {
          Accept: "application/json",
        },
      },
      {
        type: "Insert" as const,
        collection: "user_cache",
        record: {
          id: { type: "String", value: "{{user_id}}" },
          data: { type: "Object", value: "{{http_response}}" },
        },
        ttl: 300, // 5 minute cache
      },
    ],
  };

  await client.saveScript(fetchAndStore);
  console.log("✅ Saved reusable function: fetch_and_store_user");

  // Step 2: Create SWR function that CALLS the reusable function
  // Pattern: Check cache → populate if missing → fetch and return
  const swrUser = {
    label: "swr_user",
    name: "SWR pattern for user data",
    parameters: {
      user_id: { required: true },
    },
    functions: [
      // Check if data exists in cache
      {
        type: "FindById" as const,
        collection: "user_cache",
        record_id: "{{user_id}}",
      },
      {
        type: "If" as const,
        condition: {
          type: "HasRecords" as const,
        },
        then_functions: [
          // Cache hit - just project the data field
          {
            type: "Project" as const,
            fields: ["data"],
            exclude: false,
          },
        ],
        else_functions: [
          // Cache miss - call reusable function to fetch and store
          // params omitted - inherits user_id from parent scope
          {
            type: "CallFunction" as const,
            function_label: "fetch_and_store_user",
          },
          // Project data from the inserted record
          // (Insert returns the record it created, no need for FindById)
          {
            type: "Project" as const,
            fields: ["data"],
            exclude: false,
          },
        ],
      },
    ],
  };

  await client.saveScript(swrUser);
  console.log("✅ Saved SWR function using composition: swr_user\n");

  // Step 3: Test cache miss
  console.log("First call (cache miss - will fetch from API):");
  const start1 = Date.now();
  const result1 = await client.callScript("swr_user", {
    user_id: "1",
  });
  const duration1 = Date.now() - start1;

  console.log(`   ⏱️  Duration: ${duration1}ms`);
  console.log(`   📊 Records: ${result1.records.length}\n`);

  // Step 4: Test cache hit
  console.log("Second call (cache hit - from cache):");
  const start2 = Date.now();
  const result2 = await client.callScript("swr_user", {
    user_id: "1",
  });
  const duration2 = Date.now() - start2;

  console.log(`   ⏱️  Duration: ${duration2}ms`);
  console.log(`   📊 Records: ${result2.records.length}`);
  if (duration2 > 0) {
    const speedup = duration1 / duration2;
    console.log(`   🚀 Cache speedup: ${speedup.toFixed(1)}x faster!\n`);
  }
}

async function nestedCompositionExample(client: EkoDBClient): Promise<void> {
  console.log("📝 Example 3: Multi-Level Function Composition\n");
  console.log("Building complex workflows from small, reusable pieces...\n");

  // Level 1: Base function
  const validateUser = {
    label: "validate_user",
    name: "Check if user exists",
    parameters: {
      user_code: { required: true },
    },
    functions: [
      {
        type: "FindOne" as const,
        collection: "users",
        key: "user_code",
        value: "{{user_code}}",
      },
    ],
  };

  await client.saveScript(validateUser);
  console.log("✅ Level 1 function: validate_user");

  // Level 2: Calls validate_user + projects
  const fetchSlim = {
    label: "fetch_slim_user",
    name: "Validate and slim down user",
    parameters: {
      user_code: { required: true },
    },
    functions: [
      {
        type: "CallFunction" as const,
        function_label: "validate_user",
        // params omitted - inherits user_code from parent scope
      },
      {
        type: "Project" as const,
        fields: ["name", "department"],
        exclude: false,
      },
    ],
  };

  await client.saveScript(fetchSlim);
  console.log("✅ Level 2 function: fetch_slim_user (calls validate_user)");

  // Level 3: Calls fetch_slim (demonstrates 3-level nesting)
  const getVerifiedUser = {
    label: "get_verified_user",
    name: "Get verified and validated user",
    parameters: {
      user_code: { required: true },
    },
    functions: [
      {
        type: "CallFunction" as const,
        function_label: "fetch_slim_user",
        // params omitted - inherits user_code from parent scope
      },
    ],
  };

  await client.saveScript(getVerifiedUser);
  console.log(
    "✅ Level 3 function: get_verified_user (calls fetch_slim_user)\n",
  );

  // Execute 3-level nested composition
  const result = await client.callScript("get_verified_user", {
    user_code: "user_1",
  });

  console.log("📊 Result from 3-level nested composition:");
  console.log(`   Records: ${result.records.length}`);
  if (result.records.length > 0) {
    const record = result.records[0];
    const name = (record.name as any)?.value || record.name || "Unknown";
    const department =
      (record.department as any)?.value || record.department || "Unknown";
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

  console.log("=== ekoDB Function Composition Examples ===\n");

  await setupTestData(client);
  await basicCompositionExample(client);
  await swrCompositionExample(client);
  await nestedCompositionExample(client);

  console.log("\n✅ All composition examples completed!");
}

main().catch(console.error);
