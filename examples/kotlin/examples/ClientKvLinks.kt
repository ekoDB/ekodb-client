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
    val key = "user:alice_kt_${System.currentTimeMillis()}"
    val ownedLinks = listOf(
        "orders" to "order_001",
        "orders" to "order_002",
        "invoices" to "inv_100"
    )
    var primaryError: Throwable? = null

    println("=== ekoDB Kotlin Client - KV Links Example ===\n")

    try {
        // 1. Set a KV entry to link documents to
        println("--- Setting KV entry ---")
        client.kvSet(key, buildJsonObject {
            put("name", "Alice Johnson")
            put("email", "alice@example.com")
        })
        println("Set key: user:alice")

        // 2. Link documents to the KV key
        println("\n--- Linking documents ---")
        val link1 = client.kvLink(key, "orders", "order_001")
        println("Linked order_001: $link1")

        val link2 = client.kvLink(key, "orders", "order_002")
        println("Linked order_002: $link2")

        val link3 = client.kvLink(key, "invoices", "inv_100")
        println("Linked inv_100: $link3")

        // 3. Get all links for the key
        println("\n--- Getting links ---")
        val links = client.kvGetLinks(key)
        println("Links for user:alice: $links")
        check(links.size == 3) { "Expected three KV links, got ${links.size}" }

        // 4. Unlink a document
        println("\n--- Unlinking document ---")
        val unlinked = client.kvUnlink(key, "orders", "order_002")
        println("Unlinked order_002: $unlinked")

        // 5. Verify remaining links
        println("\n--- Verifying remaining links ---")
        val remaining = client.kvGetLinks(key)
        println("Remaining links: $remaining")
        check(remaining.size == 2) { "Expected two KV links after unlink, got ${remaining.size}" }

    } catch (error: Throwable) {
        primaryError = error
        throw error
    } finally {
        println("\n--- Cleanup ---")
        val cleanupErrors = mutableListOf<Throwable>()
        for ((collection, documentId) in ownedLinks) {
            try {
                client.kvUnlink(key, collection, documentId)
            } catch (error: Throwable) {
                val notFound = error.message?.let {
                    it.contains("status 404") || it.contains("not found", ignoreCase = true)
                } == true
                if (!notFound) cleanupErrors += error
            }
        }
        try {
            client.kvDelete(key)
        } catch (error: Throwable) {
            val notFound = error.message?.let {
                it.contains("status 404") || it.contains("not found", ignoreCase = true)
            } == true
            if (!notFound) cleanupErrors += error
        }
        try {
            client.close()
        } catch (error: Throwable) {
            cleanupErrors += error
        }

        val failure = primaryError
        if (failure != null) {
            cleanupErrors.forEach(failure::addSuppressed)
        } else if (cleanupErrors.isNotEmpty()) {
            val cleanupError = cleanupErrors.first()
            cleanupErrors.drop(1).forEach(cleanupError::addSuppressed)
            throw cleanupError
        }
        println("Unlinked remaining documents")
        println("Deleted key: user:alice")
        println("\n=== Example Complete ===")
    }
}
