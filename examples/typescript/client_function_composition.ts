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
        params: {
          user_code: "{{user_code}}",
        },
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
  const fetchAndStore = {
    label: "fetch_and_store_github",
    name: "Fetch GitHub user and cache",
    parameters: {
      username: { required: true },
    },
    functions: [
      {
        type: "HttpRequest" as const,
        url: "https://api.github.com/users/{{username}}",
        method: "GET",
        headers: {
          "User-Agent": "ekoDB-Client",
        },
      },
      {
        type: "Insert" as const,
        collection: "github_cache",
        record: {
          id: "{{username}}",
          data: "{{http_response}}",
        },
        ttl: 300, // 5 minute cache
      },
    ],
  };

  await client.saveScript(fetchAndStore);
  console.log("✅ Saved reusable function: fetch_and_store_github");

  // Step 2: Create SWR function that CALLS the reusable function
  const swrGithub = {
    label: "swr_github_user",
    name: "SWR pattern for GitHub user",
    parameters: {
      username: { required: true },
    },
    functions: [
      {
        type: "FindById" as const,
        collection: "github_cache",
        record_id: "{{username}}",
      },
      {
        type: "If" as const,
        condition: {
          type: "HasRecords" as const,
        },
        then_functions: [
          {
            type: "Project" as const,
            fields: ["data"],
            exclude: false,
          },
        ],
        else_functions: [
          {
            type: "CallFunction" as const,
            function_label: "fetch_and_store_github",
            params: {
              username: "{{username}}",
            },
          },
        ],
      },
    ],
  };

  await client.saveScript(swrGithub);
  console.log("✅ Saved SWR function using composition: swr_github_user\n");

  // Step 3: Test cache miss
  console.log("First call (cache miss - will fetch from GitHub):");
  const start1 = Date.now();
  const result1 = await client.callScript("swr_github_user", {
    username: "torvalds",
  });
  const duration1 = Date.now() - start1;

  console.log(`   ⏱️  Duration: ${duration1}ms`);
  console.log(`   📊 Records: ${result1.records.length}\n`);

  // Step 4: Test cache hit
  console.log("Second call (cache hit - from cache):");
  const start2 = Date.now();
  const result2 = await client.callScript("swr_github_user", {
    username: "torvalds",
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
        params: {
          user_code: "{{user_code}}",
        },
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

  // Level 3: Calls fetch_slim + counts
  const countUser = {
    label: "count_validated_user",
    name: "Get validated user and count",
    parameters: {
      user_code: { required: true },
    },
    functions: [
      {
        type: "CallFunction" as const,
        function_label: "fetch_slim_user",
        params: {
          user_code: "{{user_code}}",
        },
      },
      {
        type: "Count" as const,
        output_field: "record_count",
      },
    ],
  };

  await client.saveScript(countUser);
  console.log(
    "✅ Level 3 function: count_validated_user (calls fetch_slim_user)\n",
  );

  // Execute 3-level nested composition
  const result = await client.callScript("count_validated_user", {
    user_code: "user_1",
  });

  console.log("📊 Result from 3-level nested composition:");
  console.log(`   Records: ${result.records.length}`);
  if (result.records.length > 0) {
    const record = result.records[0];
    console.log(`   Name: ${JSON.stringify(record.name)}`);
    console.log(`   Department: ${JSON.stringify(record.department)}`);
    console.log(`   Record count: ${JSON.stringify(record.record_count)}\n`);
  }

  console.log(
    "🎯 Key Benefit: Each function is independently testable and reusable!",
  );
  console.log("   - validate_user: Used in 100 different workflows");
  console.log("   - fetch_slim_user: Used in 50 workflows");
  console.log("   - count_validated_user: Specific workflow\n");
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
