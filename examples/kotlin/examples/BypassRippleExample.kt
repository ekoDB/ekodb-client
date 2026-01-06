package io.ekodb.client.examples

import io.ekodb.client.EkoDBClient
import io.ekodb.client.getStringValue
import io.ekodb.client.types.Record
import kotlinx.coroutines.runBlocking

/**
 * Example demonstrating bypass_ripple usage (Kotlin)
 *
 * The bypass_ripple parameter allows you to skip ripple propagation for specific operations,
 * which is useful in multi-node deployments when you want to prevent data from being
 * automatically replicated to other nodes.
 */
fun main() = runBlocking {
    println("=== Bypass Ripple Example ===\n")

    val baseUrl = System.getenv("API_BASE_URL") ?: "http://localhost:8080"
    val apiKey = System.getenv("API_BASE_KEY") ?: "a-test-api-key-from-ekodb"

    val client = EkoDBClient.builder()
        .baseUrl(baseUrl)
        .apiKey(apiKey)
        .build()

    val collection = "products"

    // Basic insert (no bypass)
    println("1. Basic insert (ripple enabled):")
    val record1 = Record.new()
        .insert("name", "Product 1")
        .insert("price", 100)
    val result1 = client.insert(collection, record1)
    println("   Inserted with ripple: $result1\n")

    // Insert with bypass_ripple
    println("2. Insert with bypass_ripple:")
    val record2 = Record.new()
        .insert("name", "Product 2")
        .insert("price", 200)
    val result2 = client.insert(collection, record2, bypassRipple = true)
    println("   Inserted with bypass_ripple: $result2\n")

    // Update with bypass_ripple
    println("3. Update with bypass_ripple:")
    val userId = getStringValue(result1.fields["id"]) ?: error("No ID found")
    val updateRecord = Record.new()
        .insert("price", 150)
    val result3 = client.update(collection, userId, updateRecord, bypassRipple = true)
    println("   Updated with bypass_ripple: $result3\n")

    // Delete with bypass_ripple (use result2's ID since it exists)
    println("4. Delete with bypass_ripple:")
    val deleteId = getStringValue(result2.fields["id"]) ?: error("No ID found")
    client.delete(collection, deleteId, bypassRipple = true)
    println("   Deleted with bypass_ripple\n")

    // Batch operations with bypass_ripple
    println("5. Batch insert with bypass_ripple:")
    val records = listOf(
        Record.new()
            .insert("name", "Batch 1")
            .insert("price", 10),
        Record.new()
            .insert("name", "Batch 2")
            .insert("price", 20)
    )
    val batchResults = client.batchInsert(collection, records, bypassRipple = true)
    println("   Batch inserted with bypass_ripple: ${batchResults.successful.size} records\n")

    // Upsert with bypass_ripple (use existing record's ID)
    println("6. Upsert with bypass_ripple:")
    val upsertRecord = Record.new()
        .insert("name", "Upsert Product")
        .insert("price", 500)
    val upsertResult = client.upsert(collection, userId, upsertRecord, bypassRipple = true)
    println("   Upserted with bypass_ripple: $upsertResult\n")

    // Cleanup
    client.deleteCollection(collection)
    println("✅ All bypass_ripple operations completed successfully!")
}
