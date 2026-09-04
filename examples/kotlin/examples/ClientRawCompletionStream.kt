package io.ekodb.client.examples

import io.ekodb.client.EkoDBClient
import io.github.cdimascio.dotenv.dotenv
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.jsonPrimitive

/**
 * Raw Completion Stream Example (SSE)
 *
 * Demonstrates using SSE streaming for raw LLM completions.
 * SSE streaming keeps the connection alive with heartbeat events, making it
 * ideal for deployed instances where reverse proxies may kill idle connections.
 *
 * Prerequisites:
 * - Set OPENAI_API_KEY or ANTHROPIC_API_KEY on the ekoDB server
 * - Run the ekoDB server: make run
 */
fun main() = runBlocking {
    val dotenv = dotenv()
    val baseUrl = dotenv["API_BASE_URL"] ?: "http://localhost:8080"
    val apiKey = dotenv["API_BASE_KEY"] ?: "a-test-api-key-from-ekodb"

    val client = EkoDBClient.builder()
        .baseUrl(baseUrl)
        .apiKey(apiKey)
        .build()

    println("=== ekoDB Raw Completion Stream (SSE) Example ===\n")

    try {
        // --- 1. Basic streaming raw completion ---
        println("--- Basic SSE Raw Completion ---")
        val response1 = client.rawCompletionStream(
            systemPrompt = "You are a helpful assistant. Keep responses concise.",
            message = "What are the three primary colors?",
            maxTokens = 256,
        )
        println("Response: ${response1["content"]?.jsonPrimitive?.content}\n")

        // --- 2. Structured output via SSE ---
        println("--- Structured Output via SSE ---")
        val response2 = client.rawCompletionStream(
            systemPrompt = "Output only valid JSON. No other text.",
            message = "Return a JSON array of 3 planets with name and diameter_km fields.",
            maxTokens = 512,
        )
        println("JSON response: ${response2["content"]?.jsonPrimitive?.content}\n")

        // --- 3. Comparison: blocking HTTP vs SSE ---
        println("--- Blocking HTTP (for comparison) ---")
        val response3 = client.rawCompletion(
            systemPrompt = "You are a helpful assistant.",
            message = "Say hello in one sentence.",
            maxTokens = 64,
        )
        println("Blocking response: ${response3["content"]?.jsonPrimitive?.content}\n")

        println("=== Done ===")
    } finally {
        client.close()
    }
}
