/**
 * ekoDB TypeScript Client - Chat Session Management Example
 *
 * This example demonstrates advanced chat session management including:
 * - Creating sessions
 * - Sending messages
 * - Retrieving messages
 * - Updating sessions
 * - Branching sessions
 * - Listing sessions
 * - Deleting sessions
 */

import { EkoDBClient } from "@ekodb/ekodb-client";
import * as dotenv from "dotenv";

dotenv.config();

async function main() {
  console.log("=== ekoDB Chat Session Management Example ===\n");

  // Create client
  const client = new EkoDBClient(
    process.env.API_BASE_URL || "http://localhost:8080",
    process.env.API_BASE_KEY || "",
  );
  await client.init();

  const collection = "client_chat_sessions_ts";

  // Step 1: Insert sample data
  console.log("=== Inserting Sample Data ===");
  await client.insert(collection, {
    name: "ekoDB",
    description: "A high-performance database product",
    price: 99,
  });
  console.log("✓ Inserted sample product\n");

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
    llm_model: "gpt-4",
    system_prompt: "You are a helpful assistant for product information.",
  });
  const chatId = session.chat_id;
  console.log(`✓ Created session: ${chatId}\n`);

  // Step 3: Send messages in the session
  console.log("=== Sending Messages ===");
  const msg1 = await client.chatMessage(chatId, {
    message: "What products are available?",
  });
  console.log("✓ Message 1 sent");
  console.log(`  Response: ${msg1.responses[0] || "No response"}\n`);

  const msg2 = await client.chatMessage(chatId, {
    message: "What is the price?",
  });
  console.log("✓ Message 2 sent");
  console.log(`  Response: ${msg2.responses[0] || "No response"}\n`);

  // Step 4: Get session messages
  console.log("=== Retrieving Session Messages ===");
  const messagesResponse = await client.getChatSessionMessages(chatId, {
    limit: 10,
    sort: "asc",
  });
  console.log(`✓ Retrieved ${messagesResponse.messages.length} messages\n`);

  // Step 5: Update session
  console.log("=== Updating Session ===");
  await client.updateChatSession(chatId, {
    system_prompt: "You are an expert product consultant.",
  });
  console.log("✓ Session updated\n");

  // Step 6: Branch the session
  console.log("=== Branching Session ===");
  const branched = await client.branchChatSession({
    collections: [
      {
        collection_name: "products",
        fields: [], // Empty = search all fields
      },
    ],
    llm_provider: "openai",
    llm_model: "gpt-4",
    parent_id: chatId,
    branch_point_idx: 0,
  });
  const branchId = branched.chat_id;
  console.log(`✓ Created branch: ${branchId}`);
  console.log(`  Parent: ${chatId}\n`);

  // Step 7: List all sessions
  console.log("=== Listing Sessions ===");
  const sessionsList = await client.listChatSessions({
    limit: 10,
    sort: "desc",
  });
  console.log(`✓ Found ${sessionsList.sessions.length} sessions`);
  sessionsList.sessions.forEach((s, i) => {
    console.log(`  Session ${i + 1}: ${s.chat_id} (${s.title || "Untitled"})`);
  });
  console.log();

  // Step 8: Get session details
  console.log("=== Getting Session Details ===");
  const sessionDetails = await client.getChatSession(chatId);
  console.log("✓ Session details retrieved");
  console.log(`  Messages: ${sessionDetails.message_count}\n`);

  // Step 9: Delete branch session
  console.log("=== Deleting Branch Session ===");
  await client.deleteChatSession(branchId);
  console.log(`✓ Deleted branch session: ${branchId}\n`);

  // Cleanup: Delete the collection (chat sessions are managed by server)
  console.log("=== Cleanup ===");
  await client.deleteCollection(collection);
  console.log("✓ Deleted collection\n");

  console.log("✓ All session management operations completed successfully");
}

main().catch((error) => {
  console.error("❌ Error:", error);
  process.exit(1);
});
