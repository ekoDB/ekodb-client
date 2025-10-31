/**
 * ekoDB TypeScript Client - Advanced Chat Features Example
 *
 * This example demonstrates advanced chat features:
 * - Message regeneration
 * - Message editing
 * - Message deletion
 * - Session merging
 * - Forgotten messages (exclude from context)
 */

import { EkoDBClient, MergeStrategy } from "@ekodb/ekodb-client";
import * as dotenv from "dotenv";

dotenv.config();

async function main() {
  console.log("=== ekoDB Advanced Chat Features Example ===\n");

  // Create client
  const client = new EkoDBClient(
    process.env.API_BASE_URL || "http://localhost:8080",
    process.env.API_BASE_KEY || "",
  );
  await client.init();

  const collection = "client_chat_advanced_ts";

  // Insert sample data
  console.log("=== Inserting Sample Data ===");
  await client.insert(collection, {
    name: "ekoDB",
    description: "High-performance database product",
    price: 99,
  });
  console.log("✓ Inserted sample product\n");

  // Create a chat session
  console.log("=== Creating Chat Session ===");
  const session = await client.createChatSession({
    collections: [
      {
        collection_name: collection,
        fields: [], // Empty = search all fields
      },
    ],
    llm_provider: "openai",
    llm_model: "gpt-4",
    system_prompt: "You are a helpful product assistant.",
  });
  const chatId = session.chat_id;
  console.log(`✓ Created session: ${chatId}\n`);

  // Send initial message
  console.log("=== Sending Initial Message ===");
  const msg1 = await client.chatMessage(chatId, {
    message: "What products are available?",
  });
  console.log("✓ Message sent");
  if (msg1.responses.length > 0) {
    console.log(`  Response: ${msg1.responses[0]}\n`);
  }

  // Send a second message
  await client.chatMessage(chatId, {
    message: "What is the price of ekoDB?",
  });
  console.log("✓ Second message sent\n");

  // Get messages to find user and assistant message IDs
  const messages = await client.getChatSessionMessages(chatId, {
    limit: 10,
    sort: "desc",
  });

  const assistantMessage = messages.messages.find((msg: any) => {
    // Role is wrapped in an Object with _field_type/_field_value structure
    return msg.role?._field_value === "assistant" || msg.role === "assistant";
  });
  if (!assistantMessage) {
    throw new Error("Could not find assistant message");
  }
  const assistantMessageId = assistantMessage.id;

  const userMessage = messages.messages.find((msg: any) => {
    return msg.role?._field_value === "user" || msg.role === "user";
  });
  if (!userMessage) {
    throw new Error("Could not find user message");
  }
  const userMessageId = userMessage.id;

  // Feature 1: Regenerate Message
  console.log("=== Feature 1: Regenerate AI Response ===");
  const regenerated = await client.regenerateMessage(
    chatId,
    assistantMessageId,
  );
  console.log("✓ Message regenerated");
  if (regenerated.responses.length > 0) {
    console.log(`  New response: ${regenerated.responses[0]}\n`);
  }

  // Feature 2: Update Message Content
  console.log("=== Feature 2: Edit Message ===");
  await client.updateChatMessage(
    chatId,
    userMessageId,
    "What is the updated price of ekoDB?",
  );
  console.log("✓ Message content updated\n");

  // Feature 3: Toggle Forgotten Status
  console.log("=== Feature 3: Mark Message as Forgotten ===");
  await client.toggleForgottenMessage(chatId, userMessageId, true);
  console.log("✓ Message marked as forgotten (excluded from LLM context)\n");

  // Unmark as forgotten
  await client.toggleForgottenMessage(chatId, userMessageId, false);
  console.log("✓ Message unmarked as forgotten\n");

  // Feature 4: Session Merging
  console.log("=== Feature 4: Merge Chat Sessions ===");

  // Create a second session
  const session2 = await client.createChatSession({
    collections: [
      {
        collection_name: "products",
        fields: [], // Empty = search all fields
      },
    ],
    llm_provider: "openai",
    llm_model: "gpt-4",
  });
  const chatId2 = session2.chat_id;
  console.log(`✓ Created second session: ${chatId2}`);

  // Send a message in the second session
  await client.chatMessage(chatId2, {
    message: "Tell me more about the features",
  });
  console.log("✓ Sent message in second session");

  // Merge sessions
  const mergeResult = await client.mergeChatSessions({
    source_chat_ids: [chatId2],
    target_chat_id: chatId,
    merge_strategy: MergeStrategy.Chronological,
  });
  console.log("✓ Sessions merged successfully");
  console.log(
    `  Total messages in merged session: ${mergeResult.message_count}\n`,
  );

  // Feature 5: Delete Message
  console.log("=== Feature 5: Delete Message ===");
  await client.deleteChatMessage(chatId, userMessageId);
  console.log("✓ Message deleted\n");

  // Verify message count after deletion
  const sessionDetails = await client.getChatSession(chatId);
  console.log(`✓ Messages remaining: ${sessionDetails.message_count}\n`);

  // Cleanup
  console.log("=== Cleanup ===");
  await client.deleteChatSession(chatId);
  console.log("✓ Deleted session");
  await client.deleteCollection(collection);
  console.log("✓ Deleted collection\n");

  console.log("✓ All advanced chat features demonstrated successfully!");
}

main().catch(console.error);
