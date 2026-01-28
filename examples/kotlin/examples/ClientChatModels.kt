package io.ekodb.client.examples

import io.ekodb.client.EkoDBClient
import io.github.cdimascio.dotenv.dotenv
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.JsonArray
import kotlinx.serialization.json.jsonPrimitive

/**
 * Chat Models Example - Using ekoDB Kotlin client library
 *
 * This example demonstrates Chat Models API:
 * - getChatModels: List all available chat models by provider
 * - getChatModel: Get models for a specific provider
 */
fun main() = runBlocking {
    val dotenv = dotenv()
    val baseUrl = dotenv["API_BASE_URL"] ?: "http://localhost:8080"
    val apiKey = dotenv["API_BASE_KEY"] ?: "a-test-api-key-from-ekodb"

    val client = EkoDBClient.builder()
        .baseUrl(baseUrl)
        .apiKey(apiKey)
        .build()

    println("=== ekoDB Kotlin Client - Chat Models Example ===\n")

    try {
        // Example 1: List all available chat models by provider
        println("=== List Chat Models ===")
        try {
            val models = client.getChatModels()
            println("Available chat models by provider:")

            // Iterate over providers
            for ((provider, modelList) in models) {
                val modelArray = modelList as? JsonArray
                if (modelArray != null && modelArray.isNotEmpty()) {
                    println("  $provider:")
                    for (model in modelArray) {
                        println("    - ${model.jsonPrimitive.content}")
                    }
                }
            }
        } catch (e: Exception) {
            println("GetChatModels error: ${e.message}")
        }
        println()

        // Example 2: Get models for a specific provider (OpenAI)
        println("=== Get OpenAI Models ===")
        try {
            val openaiModels = client.getChatModel("openai")
            val modelNames = openaiModels.map { it.jsonPrimitive.content }
            println("OpenAI models: ${modelNames.joinToString(", ").ifEmpty { "none configured" }}")
        } catch (e: Exception) {
            println("GetChatModel error: ${e.message}")
        }
        println()

        // Example 3: Get Anthropic models
        println("=== Get Anthropic Models ===")
        try {
            val anthropicModels = client.getChatModel("anthropic")
            val modelNames = anthropicModels.map { it.jsonPrimitive.content }
            println("Anthropic models: ${modelNames.joinToString(", ").ifEmpty { "none configured" }}")
        } catch (e: Exception) {
            println("GetChatModel error: ${e.message}")
        }
        println()

        // Example 4: Handle non-existent provider
        println("=== Get Non-Existent Provider ===")
        try {
            client.getChatModel("nonexistent_provider_xyz")
            println("Unexpectedly found non-existent provider")
        } catch (e: Exception) {
            println("Expected error for non-existent provider: ${e.message}")
        }

    } finally {
        client.close()
        println("\n=== Chat Models Example Complete ===")
    }
}
