/**
 * WebSocket Chat Streaming Example - Real-time LLM responses
 *
 * Demonstrates using the WebSocket client to send a chat message
 * and receive streaming responses, including tool calls.
 *
 * Requires: ekoDB server running on localhost:8080
 */

import { EkoDBClient, WebSocketClient } from "@ekodb/ekodb-client";

const BASE_URL = process.env.API_BASE_URL || "http://localhost:8080";
const WS_URL = process.env.WS_BASE_URL || "ws://localhost:8080";
const API_KEY = process.env.API_BASE_KEY || "a-test-api-key-from-ekodb";

async function cleanupChat(
  client: EkoDBClient,
  chatId: string | undefined,
  ws: WebSocketClient | undefined,
  primaryError: unknown,
): Promise<void> {
  const failures: unknown[] = [];
  if (primaryError !== undefined) failures.push(primaryError);
  try {
    ws?.close();
  } catch (error) {
    failures.push(error);
  }
  if (chatId !== undefined) {
    try {
      await client.deleteChatSession(chatId);
    } catch (error) {
      failures.push(error);
    }
  }
  if (failures.length === 1) throw failures[0];
  if (failures.length > 1) {
    const combined = new Error(failures.map(String).join("; "));
    (combined as Error & { cause?: unknown }).cause = primaryError;
    throw combined;
  }
}

async function main() {
  console.log("=== WebSocket Chat Streaming Example (TypeScript) ===\n");

  // Create HTTP client for session management
  const client = new EkoDBClient({ baseURL: BASE_URL, apiKey: API_KEY });

  let chatId: string | undefined;
  let ws: WebSocketClient | undefined;
  let primaryError: unknown;

  try {
    // Create a chat session
    const session = await client.createChatSession({
      collections: [],
      llm_provider: "openai",
      system_prompt: "You are a helpful assistant.",
    });
    const activeChatId = session.chat_id;
    chatId = activeChatId;
    console.log(`Created chat session: ${activeChatId}`);

    // Get auth token for WebSocket
    await client.refreshToken();
    const token = (await client.getToken())!;

    // Connect WebSocket
    ws = new WebSocketClient(WS_URL, token);

    // Send a chat message and stream the response
    console.log("\nSending message: 'What is the capital of France?'");
    const stream = await ws.chatSend(
      activeChatId,
      "What is the capital of France?",
    );

    let fullResponse = "";
    let completed = false;
    let streamError: Error | undefined;
    const pendingToolResults: Promise<void>[] = [];

    stream.on("event", (event) => {
      switch (event.type) {
        case "chunk":
          fullResponse += event.content;
          process.stdout.write(event.content);
          break;

        case "end":
          completed = true;
          console.log(`\n\n--- Stream ended ---`);
          console.log(`Message ID: ${event.messageId}`);
          console.log(`Execution time: ${event.executionTimeMs}ms`);
          if (event.tokenUsage) {
            console.log(`Token usage: ${JSON.stringify(event.tokenUsage)}`);
          }
          break;

        case "toolCall":
          console.log(
            `\n[Tool Call] ${event.toolName}(${JSON.stringify(event.arguments)})`,
          );
          // Send tool result back
          pendingToolResults.push(
            ws!
              .sendToolResult(activeChatId, event.callId, true, {
                result: "Tool executed successfully",
              })
              .catch((error) => {
                console.error(`Failed to send tool result: ${error}`);
                streamError = new Error(`Failed to send tool result: ${error}`);
              }),
          );
          break;

        case "error":
          console.error(`\n[Error] ${event.error}`);
          streamError = new Error(`WebSocket chat failed: ${event.error}`);
          break;
      }
    });

    // Wait for stream to close
    await new Promise<void>((resolve) => stream.on("close", resolve));
    await Promise.all(pendingToolResults);
    if (streamError) throw streamError;
    if (!completed) throw new Error("WebSocket chat ended before an end event");

    console.log(`\nFull response: ${fullResponse.substring(0, 200)}...`);
  } catch (error) {
    primaryError = error;
  }
  await cleanupChat(client, chatId, ws, primaryError);
}

async function chatWithClientTools() {
  console.log("\n=== Chat with Client Tools ===\n");

  const client = new EkoDBClient({ baseURL: BASE_URL, apiKey: API_KEY });
  let ws: WebSocketClient | undefined;
  let chatId: string | undefined;
  let primaryError: unknown;

  try {
    await client.refreshToken();
    const token = (await client.getToken())!;
    ws = new WebSocketClient(WS_URL, token);

    const session = await client.createChatSession({
      collections: [],
      llm_provider: "openai",
      system_prompt: "You are a helpful assistant with access to tools.",
    });
    const activeChatId = session.chat_id;
    chatId = activeChatId;

    // Register client-side tools
    await ws.registerClientTools(activeChatId, [
      {
        name: "get_weather",
        description: "Get the current weather for a location",
        parameters: {
          type: "object",
          properties: {
            location: { type: "string", description: "City name" },
          },
          required: ["location"],
        },
      },
    ]);
    console.log("Registered client tools");

    // Send message that might trigger tool use
    const stream = await ws.chatSend(
      activeChatId,
      "What's the weather in Paris?",
    );
    let completed = false;
    let streamError: Error | undefined;
    const pendingToolResults: Promise<void>[] = [];

    stream.on("event", (event) => {
      switch (event.type) {
        case "chunk":
          process.stdout.write(event.content);
          break;

        case "toolCall":
          console.log(
            `\n[Tool Call] ${event.toolName}(${JSON.stringify(event.arguments)})`,
          );
          pendingToolResults.push(
            ws!
              .sendToolResult(activeChatId, event.callId, true, {
                temperature: "22°C",
                condition: "Sunny",
              })
              .catch((error) => {
                console.error(`Failed to send tool result: ${error}`);
                streamError = new Error(`Failed to send tool result: ${error}`);
              }),
          );
          break;

        case "end":
          completed = true;
          console.log(`\n--- Done (${event.executionTimeMs}ms) ---`);
          break;

        case "error":
          console.error(`\n[Error] ${event.error}`);
          streamError = new Error(`WebSocket chat failed: ${event.error}`);
          break;
      }
    });

    await new Promise<void>((resolve) => stream.on("close", resolve));
    await Promise.all(pendingToolResults);
    if (streamError) throw streamError;
    if (!completed) throw new Error("WebSocket chat ended before an end event");
  } catch (error) {
    primaryError = error;
  }
  await cleanupChat(client, chatId, ws, primaryError);
}

const mode = process.env.MODE || "basic";
if (mode === "client-tools") {
  chatWithClientTools().catch((error) => {
    console.error(error);
    process.exit(1);
  });
} else {
  main().catch((error) => {
    console.error(error);
    process.exit(1);
  });
}
