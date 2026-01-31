/**
 * User Functions API Example - Using @ekodb/ekodb-client library
 *
 * This example demonstrates CRUD operations for User Functions.
 * User Functions are reusable sequences of Functions that can be called by Scripts.
 */

import { EkoDBClient, UserFunction } from "@ekodb/ekodb-client";
import * as dotenv from "dotenv";

dotenv.config();

const BASE_URL = process.env.API_BASE_URL || "http://localhost:8080";
const API_KEY = process.env.API_BASE_KEY || "a-test-api-key-from-ekodb";

async function main() {
  const client = new EkoDBClient(BASE_URL, API_KEY);
  await client.init();
  console.log("✓ Client created");

  // Example 1: Create a User Function
  console.log("\n=== Create User Function ===");
  const userFunc: UserFunction = {
    label: "get_active_users_ts",
    name: "Get Active Users",
    description: "Fetches all users and filters by active status",
    version: "1.0.0",
    parameters: {
      collection: {
        required: true,
        description: "Collection to query",
      },
    },
    functions: [{ type: "FindAll", collection: "{{collection}}" }],
    tags: ["users", "query"],
  };

  try {
    const funcId = await client.saveUserFunction(userFunc);
    console.log(`Created user function with ID: ${funcId}`);
  } catch (error) {
    console.log(`SaveUserFunction error: ${error}`);
  }

  // Example 2: Get User Function by label
  console.log("\n=== Get User Function ===");
  try {
    const retrieved = await client.getUserFunction("get_active_users_ts");
    console.log(`Retrieved: ${retrieved.label} - ${retrieved.name}`);
    console.log(`Description: ${retrieved.description}`);
  } catch (error) {
    console.log(`GetUserFunction error: ${error}`);
  }

  // Example 3: List all User Functions
  console.log("\n=== List All User Functions ===");
  try {
    const allFuncs = await client.listUserFunctions();
    console.log(`Found ${allFuncs.length} user functions:`);
    for (const f of allFuncs) {
      console.log(`  - ${f.label}: ${f.name}`);
    }
  } catch (error) {
    console.log(`ListUserFunctions error: ${error}`);
  }

  // Example 4: Update User Function
  console.log("\n=== Update User Function ===");
  try {
    userFunc.description = "Fetches all users - updated description";
    await client.updateUserFunction("get_active_users_ts", userFunc);
    console.log("User function updated successfully");
  } catch (error) {
    console.log(`UpdateUserFunction error: ${error}`);
  }

  // Example 5: Delete User Function
  console.log("\n=== Delete User Function ===");
  try {
    await client.deleteUserFunction("get_active_users_ts");
    console.log("User function deleted successfully");
  } catch (error) {
    console.log(`DeleteUserFunction error: ${error}`);
  }

  console.log("\n✓ User Functions API example complete");
}

main().catch((error) => {
  console.error("❌ Error:", error);
  process.exit(1);
});
