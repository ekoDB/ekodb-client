// ekoDB TypeScript Client - Chat Message Stream (SSE) Example
//
// Demonstrates streaming chat responses via HTTP SSE (Server-Sent Events).

import { EkoDBClient } from "@ekodb/ekodb-client";

function combineErrors(primary: unknown, cleanup: unknown[]): unknown {
  if (cleanup.length === 0) return primary;
  if (primary === undefined && cleanup.length === 1) return cleanup[0];
  const failures = primary === undefined ? cleanup : [primary, ...cleanup];
  const combined = new Error(failures.map(String).join("; "));
  (combined as Error & { cause?: unknown }).cause = primary;
  return combined;
}

async function main() {
  console.log("=== ekoDB Chat Message Stream (SSE) Example (TypeScript) ===\n");

  const baseUrl = process.env.API_BASE_URL || "http://localhost:8080";
  const apiKey = process.env.API_BASE_KEY;
  if (!apiKey) throw new Error("API_BASE_KEY environment variable is required");

  const client = new EkoDBClient(baseUrl, apiKey);

  let chatId: string | undefined;
  let primaryError: unknown;
  try {
    // Create a chat session
    const session = await client.createChatSession({
      collections: [],
      llm_provider: "openai",
      system_prompt: "You are a helpful assistant.",
    });
    chatId = session.chat_id;
    console.log(`Created session: ${chatId}`);

    // Stream a chat message via SSE
    console.log("\nStreaming response for: 'What is ekoDB?'\n");
    const stream = client.chatMessageStream(chatId, {
      message: "What is ekoDB?",
    });
    let completed = false;
    let streamError: Error | undefined;

    stream.on("event", (event: any) => {
      switch (event.type) {
        case "chunk":
          process.stdout.write(event.content);
          break;
        case "end":
          completed = true;
          console.log("\n\n--- Stream complete ---");
          console.log(`Message ID: ${event.messageId}`);
          console.log(`Execution time: ${event.executionTimeMs}ms`);
          if (event.contextWindow) {
            console.log(`Context window: ${event.contextWindow} tokens`);
          }
          break;
        case "error":
          console.error(`Error: ${event.error}`);
          streamError = new Error(`Chat stream failed: ${event.error}`);
          break;
      }
    });

    await new Promise((resolve) => stream.on("close", resolve));
    if (streamError) throw streamError;
    if (!completed) throw new Error("Chat stream ended before an end event");
  } catch (error) {
    primaryError = error;
  }

  const cleanupErrors: unknown[] = [];
  if (chatId !== undefined) {
    try {
      await client.deleteChatSession(chatId);
    } catch (error) {
      cleanupErrors.push(error);
    }
  }
  const failure = combineErrors(primaryError, cleanupErrors);
  if (failure !== undefined) throw failure;

  console.log("\n✓ Chat message stream example completed");
}

main().catch((error) => {
  console.error(error);
  process.exit(1);
});
