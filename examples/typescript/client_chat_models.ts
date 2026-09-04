/**
 * Chat Models Example - Using @ekodb/ekodb-client library
 *
 * This example demonstrates Chat Models API:
 * - getChatModels: List all available chat models by provider
 * - getChatModel: Get models for a specific provider
 */

import { EkoDBClient } from "@ekodb/ekodb-client";
import * as dotenv from "dotenv";

dotenv.config();

const BASE_URL = process.env.API_BASE_URL || "http://localhost:8080";
const API_KEY = process.env.API_BASE_KEY || "a-test-api-key-from-ekodb";

async function main() {
  const client = new EkoDBClient(BASE_URL, API_KEY);
  await client.init();
  console.log("✓ Client created");

  // Example 1: List all available chat models by provider
  console.log("\n=== List Chat Models ===");
  try {
    const models = await client.getChatModels();
    console.log("Available chat models by provider:");

    // Iterate over providers
    for (const [provider, modelList] of Object.entries(models)) {
      if (modelList.length > 0) {
        console.log(`  ${provider}:`);
        for (const modelName of modelList) {
          console.log(`    - ${modelName}`);
        }
      }
    }
  } catch (error) {
    console.log(`GetChatModels error: ${error}`);
  }

  // Example 2: Get models for a specific provider
  console.log("\n=== Get Specific Provider Models ===");
  try {
    const providerModels = await client.getChatModel("openai");
    console.log(
      `OpenAI models: ${providerModels.join(", ") || "none configured"}`,
    );
  } catch (error) {
    console.log(`GetChatModel error: ${error}`);
  }

  // Example 3: Get Anthropic models
  console.log("\n=== Get Anthropic Models ===");
  try {
    const anthropicModels = await client.getChatModel("anthropic");
    console.log(
      `Anthropic models: ${anthropicModels.join(", ") || "none configured"}`,
    );
  } catch (error) {
    console.log(`GetChatModel error: ${error}`);
  }

  // Example 4: Handle non-existent provider
  console.log("\n=== Get Non-Existent Provider ===");
  try {
    await client.getChatModel("nonexistent_provider_xyz");
    console.log("Unexpectedly found non-existent provider");
  } catch (error) {
    console.log(`Expected error for non-existent provider: ${error}`);
  }

  console.log("\n✓ Chat Models example complete");
}

main().catch((error) => {
  console.error("❌ Error:", error);
  process.exit(1);
});
