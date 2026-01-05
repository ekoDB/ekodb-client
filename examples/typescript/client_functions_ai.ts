/**
 * AI Scripts Example - Chat and Embed Operations
 *
 * Demonstrates AI operations in scripts:
 * - Chat completions with context
 * - Embedding generation
 * - Simple AI workflows
 */

import { EkoDBClient, Stage, ChatMessage } from "@ekodb/ekodb-client";
import * as dotenv from "dotenv";

dotenv.config();

const BASE_URL = process.env.API_BASE_URL || "http://localhost:8080";
const API_KEY = process.env.API_BASE_KEY || "a-test-api-key-from-ekodb";

async function main() {
  const client = new EkoDBClient(BASE_URL, API_KEY);

  console.log("🚀 ekoDB TypeScript AI Scripts Example\n");

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

  const scriptIds: string[] = [];

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
          ChatMessage.user("What are the benefits of using vector databases?"),
        ],
        "gpt-4",
        0.7,
      ),
    ],
    tags: ["ai", "chat"],
  };
  const scriptId1 = await client.saveScript(script1);
  scriptIds.push(scriptId1);
  console.log("✅ Chat script saved");

  const result1 = await client.callScript("ai_assistant_ts");
  console.log("🤖 AI Response:");
  if (result1.records && result1.records[0]) {
    const response =
      result1.records[0].response?.value || result1.records[0].response;
    console.log(`   ${response}`);
  }
  console.log(`⏱️  Execution time: ${result1.stats?.execution_time_ms}ms\n`);

  // Example 2: Embed Generation
  console.log("📝 Example 2: Generate Embeddings\n");
  const script2 = {
    label: "generate_embedding_ts",
    name: "Generate Embedding",
    description: "Generate embedding for text",
    version: "1.0",
    parameters: {
      text: { required: true, description: "Text to embed" },
    },
    functions: [Stage.embed("text", "embedding")],
    tags: ["ai", "embed"],
  };
  const scriptId2 = await client.saveScript(script2);
  scriptIds.push(scriptId2);
  console.log("✅ Embed script saved");

  const result2 = await client.callScript("generate_embedding_ts", {
    text: "ekoDB is a powerful database",
  });
  console.log("📊 Embedding generated");
  if (result2.records && result2.records[0]) {
    const embedding =
      result2.records[0].embedding?.value || result2.records[0].embedding;
    console.log(
      `   Dimensions: ${Array.isArray(embedding) ? embedding.length : "N/A"}`,
    );
  }
  console.log(`⏱️  Execution time: ${result2.stats?.execution_time_ms}ms\n`);

  // Cleanup
  console.log("🧹 Cleaning up...");
  for (const scriptId of scriptIds) {
    try {
      await client.deleteScript(scriptId);
    } catch (e) {}
  }
  try {
    await client.deleteCollection("ai_articles_ts");
  } catch (e) {}
  console.log("✅ Cleanup complete\n");

  console.log("✅ All AI script examples finished!");
  console.log("\n💡 This example demonstrates:");
  console.log("   ✅ Chat completions with system/user messages");
  console.log("   ✅ Embedding generation for text");
}

main().catch(console.error);
