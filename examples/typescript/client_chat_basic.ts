/**
 * ekoDB TypeScript Client - Basic Chat Example
 *
 * This example demonstrates basic chat functionality with ekoDB.
 */

import { EkoDBClient } from "@ekodb/ekodb-client";
import * as dotenv from "dotenv";

dotenv.config();

async function main() {
  console.log("=== ekoDB Chat Basic Example ===\n");

  // Create client
  const client = new EkoDBClient(
    process.env.API_BASE_URL || "http://localhost:8080",
    process.env.API_BASE_KEY || "",
  );
  await client.init();

  const collection = "client_chat_basic_ts";
  let chatId: string | undefined;
  let primaryError: unknown;

  try {
    // Step 1: Insert sample data
    console.log("=== Inserting Sample Data ===");
    await client.insert(collection, {
      name: "ekoDB",
      description: "A high-performance database product with AI capabilities",
      price: 99,
    });
    await client.insert(collection, {
      name: "ekoDB Pro",
      description: "Enterprise edition product with advanced features",
      price: 299,
    });
    await client.insert(collection, {
      name: "ekoDB Cloud",
      description: "Fully managed cloud database service product",
      price: 499,
    });
    console.log("✓ Inserted 3 sample documents\n");

    // Step 2: Create a chat session
    console.log("=== Creating Chat Session ===");
    const session = await client.createChatSession({
      collections: [
        {
          collection_name: collection,
          fields: [], // Empty = search all fields
        },
      ],
      llm_provider: "openai",
      llm_model: "gpt-4o-mini",
      system_prompt: "You are a helpful assistant for product information.",
    });
    chatId = session.chat_id;
    console.log(`✓ Created session: ${chatId}\n`);

    // Step 3: Send a chat message
    console.log("=== Sending Chat Message ===");
    const response = await client.chatMessage(chatId, {
      message: "What products are available and what are their prices?",
    });

    console.log(`Message ID: ${response.message_id}`);
    console.log("\n=== AI Response ===");
    response.responses.forEach((r) => console.log(r));
    console.log(
      `\n=== Context Used (${response.context_snippets.length} snippets) ===`,
    );
    response.context_snippets.forEach((snippet, i) => {
      console.log(`  Snippet ${i + 1}:`, snippet);
    });
    console.log(`\nExecution Time: ${response.execution_time_ms}ms`);

    if (response.token_usage) {
      console.log(`\n=== Token Usage ===`);
      console.log(`Prompt tokens: ${response.token_usage.prompt_tokens}`);
      console.log(
        `Completion tokens: ${response.token_usage.completion_tokens}`,
      );
      console.log(`Total tokens: ${response.token_usage.total_tokens}`);
    }
  } catch (error) {
    primaryError = error;
    throw error;
  } finally {
    console.log("\n=== Cleanup ===");
    const cleanup = await Promise.allSettled([
      ...(chatId ? [client.deleteChatSession(chatId)] : []),
      client.deleteCollection(collection),
    ]);
    const failures = cleanup.filter(
      (result): result is PromiseRejectedResult => result.status === "rejected",
    );
    if (failures.length > 0) {
      throw new AggregateError(
        [primaryError, ...failures.map((failure) => failure.reason)].filter(
          Boolean,
        ),
        "Chat example or cleanup failed",
      );
    }
    if (chatId) console.log("✓ Deleted session");
    console.log("✓ Deleted collection");
  }
  console.log("\n✓ Chat completed successfully");
}

main().catch((error) => {
  console.error("❌ Error:", error);
  process.exit(1);
});
