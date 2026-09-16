/**
 * AI Functions Example - Chat and Embed Operations
 *
 * Demonstrates AI operations in scripts:
 * - Chat completions with context
 * - Embedding generation
 * - Simple AI workflows
 */

import {
  EkoDBClient,
  Stage,
  ChatMessage,
  UserFunction,
} from "@ekodb/ekodb-client";
import * as dotenv from "dotenv";

dotenv.config();

const BASE_URL = process.env.API_BASE_URL || "http://localhost:8080";
const API_KEY = process.env.API_BASE_KEY || "a-test-api-key-from-ekodb";

/** True when a save failed because the function label already exists (HTTP 409). */
function isAlreadyExistsError(error: unknown): boolean {
  const message = error instanceof Error ? error.message : String(error);
  return message.includes("status 409") || message.includes("already exists");
}

/**
 * Idempotent save: create the function, or update it in place if a function
 * with the same label already exists. Returns the function's id so downstream
 * cleanup-by-id continues to work.
 */
async function saveOrUpdate(
  client: EkoDBClient,
  script: UserFunction,
): Promise<string> {
  try {
    return await client.saveFunction(script);
  } catch (error) {
    if (!isAlreadyExistsError(error)) throw error;
    await client.updateFunction(script.label, script);
    console.log(`Function '${script.label}' already existed — updated instead`);
    const existing = await client.getFunction(script.label);
    if (!existing.id) {
      throw new Error(
        `Updated function '${script.label}' did not include an id`,
      );
    }
    return existing.id;
  }
}

async function main() {
  const client = new EkoDBClient(BASE_URL, API_KEY);
  const scriptIds: string[] = [];
  let primaryError: unknown;

  console.log("🚀 ekoDB TypeScript AI Functions Example\n");
  try {
    // Setup test data
    console.log("📋 Setting up test data...");
    try {
      await client.deleteCollection("ai_articles_ts");
    } catch (e) {}

    const articles = [
      {
        title: "Getting Started with ekoDB",
        content: "ekoDB is a high-performance database with AI capabilities...",
        status: "published",
      },
      {
        title: "Advanced Query Patterns",
        content: "Learn how to build complex queries in ekoDB...",
        status: "published",
      },
    ];

    for (const article of articles) {
      await client.insert("ai_articles_ts", article);
    }
    console.log(`✅ Created ${articles.length} articles\n`);

    // Example 1: Simple Chat Completion
    console.log("📝 Example 1: Simple Chat Completion\n");
    const script1 = {
      label: "ai_assistant_ts",
      name: "AI Chat Assistant",
      description: "Simple AI chat completion",
      version: "1.0",
      parameters: {},
      functions: [
        Stage.chat(
          [
            ChatMessage.system(
              "You are a helpful database assistant. Be concise.",
            ),
            ChatMessage.user(
              "What are the benefits of using vector databases?",
            ),
          ],
          "gpt-4o-mini",
          0.7,
        ),
      ],
      tags: ["ai", "chat"],
    };
    const scriptId1 = await saveOrUpdate(client, script1);
    scriptIds.push(scriptId1);
    console.log("✅ Chat script saved");

    const result1 = await client.callFunction(scriptId1);
    console.log("🤖 AI Response:");
    const response =
      result1.records?.[0]?.response?.value ?? result1.records?.[0]?.response;
    if (typeof response !== "string" || response.trim().length === 0) {
      throw new Error("Chat function returned no response text");
    }
    console.log(`   ${response}`);
    console.log(`⏱️  Execution time: ${result1.stats?.execution_time_ms}ms\n`);

    // Example 2: Embed Generation
    console.log("📝 Example 2: Generate Embeddings\n");
    const script2 = {
      label: "generate_embedding_ts",
      name: "Generate Embedding",
      description: "Generate embedding for text",
      version: "1.0",
      parameters: {},
      functions: [
        Stage.findAll("ai_articles_ts"),
        Stage.embed("content", "embedding"),
      ],
      tags: ["ai", "embed"],
    };
    const scriptId2 = await saveOrUpdate(client, script2);
    scriptIds.push(scriptId2);
    console.log("✅ Embed script saved");

    const result2 = await client.callFunction(scriptId2);
    if (result2.records.length !== articles.length) {
      throw new Error(
        `Embedding function returned ${result2.records.length} records; expected ${articles.length}`,
      );
    }
    const dimensions = result2.records.map((record) => {
      const embedding = record.embedding?.value ?? record.embedding;
      if (!Array.isArray(embedding) || embedding.length === 0) {
        throw new Error(
          "Embedding function returned an empty embedding vector",
        );
      }
      return embedding.length;
    });
    console.log(`📊 Generated ${dimensions.length} embeddings`);
    console.log(`   Dimensions: ${dimensions[0]}`);
    console.log(`⏱️  Execution time: ${result2.stats?.execution_time_ms}ms\n`);
  } catch (error) {
    primaryError = error;
    throw error;
  } finally {
    console.log("🧹 Cleaning up...");
    const cleanupErrors: unknown[] = [];
    for (const scriptId of scriptIds.slice().reverse()) {
      try {
        await client.deleteFunction(scriptId);
      } catch (error) {
        cleanupErrors.push(error);
      }
    }
    try {
      await client.deleteCollection("ai_articles_ts");
    } catch (error) {
      cleanupErrors.push(error);
    }
    if (cleanupErrors.length > 0) {
      if (primaryError !== undefined) cleanupErrors.unshift(primaryError);
      throw new AggregateError(
        cleanupErrors,
        primaryError === undefined
          ? "AI functions cleanup failed"
          : "AI functions example and cleanup failed",
      );
    }
    console.log("✅ Cleanup complete\n");
  }

  console.log("✅ All AI script examples finished!");
  console.log("\n💡 This example demonstrates:");
  console.log("   ✅ Chat completions with system/user messages");
  console.log("   ✅ Embedding generation for text");
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
