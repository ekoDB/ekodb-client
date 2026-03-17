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

async function main() {
  console.log("=== WebSocket Chat Streaming Example (TypeScript) ===\n");

  // Create HTTP client for session management
  const client = new EkoDBClient({ baseURL: BASE_URL, apiKey: API_KEY });

  // Create a chat session
  const session = await client.createChatSession({
    collections: [],
    llm_provider: "openai",
    system_prompt: "You are a helpful assistant.",
  });
  const chatId = session.chat_id;
  console.log(`Created chat session: ${chatId}`);

  // Get auth token for WebSocket
  await client.refreshToken();
  const token = client.getToken()!;

  // Connect WebSocket
  const ws = new WebSocketClient(WS_URL, token);

  // Send a chat message and stream the response
  console.log("\nSending message: 'What is the capital of France?'");
  const stream = await ws.chatSend(chatId, "What is the capital of France?");

  let fullResponse = "";

  stream.on("event", (event) => {
    switch (event.type) {
      case "chunk":
        fullResponse += event.content;
        process.stdout.write(event.content);
        break;

      case "end":
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
        ws.sendToolResult(chatId, event.callId, true, {
          result: "Tool executed successfully",
        });
        break;

      case "error":
        console.error(`\n[Error] ${event.error}`);
        break;
    }
  });

  // Wait for stream to close
  await new Promise<void>((resolve) => stream.on("close", resolve));

  console.log(`\nFull response: ${fullResponse.substring(0, 200)}...`);
  ws.close();
}

async function chatWithClientTools() {
  console.log("\n=== Chat with Client Tools ===\n");

  const client = new EkoDBClient({ baseURL: BASE_URL, apiKey: API_KEY });
  await client.refreshToken();
  const token = client.getToken()!;

  const ws = new WebSocketClient(WS_URL, token);

  const session = await client.createChatSession({
    collections: [],
    llm_provider: "openai",
    system_prompt: "You are a helpful assistant with access to tools.",
  });
  const chatId = session.chat_id;

  // Register client-side tools
  await ws.registerClientTools(chatId, [
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
  const stream = await ws.chatSend(chatId, "What's the weather in Paris?");

  stream.on("event", async (event) => {
    switch (event.type) {
      case "chunk":
        process.stdout.write(event.content);
        break;

      case "toolCall":
        console.log(
          `\n[Tool Call] ${event.toolName}(${JSON.stringify(event.arguments)})`,
        );
        await ws
          .sendToolResult(chatId, event.callId, true, {
            temperature: "22°C",
            condition: "Sunny",
          })
          .catch((err) => console.error(`Failed to send tool result: ${err}`));
        break;

      case "end":
        console.log(`\n--- Done (${event.executionTimeMs}ms) ---`);
        break;

      case "error":
        console.error(`\n[Error] ${event.error}`);
        break;
    }
  });

  await new Promise<void>((resolve) => stream.on("close", resolve));
  ws.close();
}

const mode = process.env.MODE || "basic";
if (mode === "client-tools") {
  chatWithClientTools().catch(console.error);
} else {
  main().catch(console.error);
}
