/**
 * ekoDB TypeScript Client - Raw Completion Stream (SSE) Example
 *
 * Demonstrates using SSE streaming for raw LLM completions.
 * SSE streaming keeps the connection alive with heartbeat events, making it
 * ideal for deployed instances where reverse proxies may kill idle connections.
 */

import { EkoDBClient } from "@ekodb/ekodb-client";
import * as dotenv from "dotenv";

dotenv.config();

async function main() {
  console.log("=== ekoDB Raw Completion Stream (SSE) Example ===\n");

  const client = new EkoDBClient(
    process.env.API_BASE_URL || "http://localhost:8080",
    process.env.API_BASE_KEY || "",
  );
  await client.init();

  // --- 1. Basic streaming raw completion ---
  console.log("--- Basic SSE Raw Completion ---");
  const response1 = await client.rawCompletionStream({
    system_prompt: "You are a helpful assistant. Keep responses concise.",
    message: "What are the three primary colors?",
    max_tokens: 256,
  });
  console.log(`Response: ${response1.content}\n`);

  // --- 2. Structured output via SSE ---
  console.log("--- Structured Output via SSE ---");
  const response2 = await client.rawCompletionStream({
    system_prompt: "Output only valid JSON. No other text.",
    message:
      "Return a JSON array of 3 planets with name and diameter_km fields.",
    max_tokens: 512,
  });
  console.log(`JSON response: ${response2.content}\n`);

  // --- 3. Comparison: blocking HTTP vs SSE ---
  // On deployed instances behind reverse proxies, rawCompletion() may timeout.
  // rawCompletionStream() keeps the connection alive with SSE heartbeats.
  console.log("--- Blocking HTTP (for comparison) ---");
  const response3 = await client.rawCompletion({
    system_prompt: "You are a helpful assistant.",
    message: "Say hello in one sentence.",
    max_tokens: 64,
  });
  console.log(`Blocking response: ${response3.content}\n`);

  console.log("=== Done ===");
}

main().catch(console.error);
