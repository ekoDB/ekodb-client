package io.ekodb.client.examples

import io.ekodb.client.EkoDBClient
import io.github.cdimascio.dotenv.dotenv
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.buildJsonObject
import kotlinx.serialization.json.put

/**
 * KV document linking example.
 *
 * Exercises kvSet -> kvLink -> kvGetLinks -> kvUnlink.
 */
fun main() = runBlocking {
    val dotenv = dotenv()
    val baseUrl = dotenv["API_BASE_URL"] ?: "http://localhost:8080"
    val apiKey = dotenv["API_BASE_KEY"] ?: "a-test-api-key-from-ekodb"

    val client = EkoDBClient.builder()
        .baseUrl(baseUrl)
        .apiKey(apiKey)
        .build()

    println("=== ekoDB Kotlin Client - KV Links Example ===\n")

    try {
        // 1. Set a KV entry to link documents to
        println("--- Setting KV entry ---")
        client.kvSet("user:alice", buildJsonObject {
            put("name", "Alice Johnson")
            put("email", "alice@example.com")
        })
        println("Set key: user:alice")

        // 2. Link documents to the KV key
        println("\n--- Linking documents ---")
        val link1 = client.kvLink("user:alice", "orders", "order_001")
        println("Linked order_001: $link1")

        val link2 = client.kvLink("user:alice", "orders", "order_002")
        println("Linked order_002: $link2")

        val link3 = client.kvLink("user:alice", "invoices", "inv_100")
        println("Linked inv_100: $link3")

        // 3. Get all links for the key
        println("\n--- Getting links ---")
        val links = client.kvGetLinks("user:alice")
        println("Links for user:alice: $links")

        // 4. Unlink a document
        println("\n--- Unlinking document ---")
        val unlinked = client.kvUnlink("user:alice", "orders", "order_002")
        println("Unlinked order_002: $unlinked")

        // 5. Verify remaining links
        println("\n--- Verifying remaining links ---")
        val remaining = client.kvGetLinks("user:alice")
        println("Remaining links: $remaining")

        // Cleanup
        println("\n--- Cleanup ---")
        client.kvDelete("user:alice")
        println("Deleted key: user:alice")

    } finally {
        client.close()
        println("\n=== Example Complete ===")
    }
}
