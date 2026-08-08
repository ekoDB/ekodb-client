// ekoDB TypeScript Client - Chat Message Stream (SSE) Example
//
// Demonstrates streaming chat responses via HTTP SSE (Server-Sent Events).

import { EkoDBClient } from "@ekodb/ekodb-client";

async function main() {
  console.log("=== ekoDB Chat Message Stream (SSE) Example (TypeScript) ===\n");

  const baseUrl = process.env.API_BASE_URL || "http://localhost:8080";
  const apiKey = process.env.API_BASE_KEY;
  if (!apiKey) throw new Error("API_BASE_KEY environment variable is required");

  const client = new EkoDBClient(baseUrl, apiKey);

  // Create a chat session
  const session = await client.createChatSession({
    collections: [],
    llm_provider: "openai",
    system_prompt: "You are a helpful assistant.",
  });
  const chatId = session.chat_id;
  console.log(`Created session: ${chatId}`);

  // Stream a chat message via SSE
  console.log("\nStreaming response for: 'What is ekoDB?'\n");
  const stream = client.chatMessageStream(chatId, {
    message: "What is ekoDB?",
  });

  stream.on("event", (event: any) => {
    switch (event.type) {
      case "chunk":
        process.stdout.write(event.content);
        break;
      case "end":
        console.log("\n\n--- Stream complete ---");
        console.log(`Message ID: ${event.messageId}`);
        console.log(`Execution time: ${event.executionTimeMs}ms`);
        if (event.contextWindow) {
          console.log(`Context window: ${event.contextWindow} tokens`);
        }
        break;
      case "error":
        console.error(`Error: ${event.error}`);
        break;
    }
  });

  await new Promise((resolve) => stream.on("close", resolve));
  console.log("\n✓ Chat message stream example completed");
}

main().catch(console.error);
