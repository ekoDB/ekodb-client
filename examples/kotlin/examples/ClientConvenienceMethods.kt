package io.ekodb.client.examples

import io.ekodb.client.EkoDBClient
import io.ekodb.client.types.Record
import kotlinx.coroutines.runBlocking

/**
 * ekoDB Convenience Methods Example (Kotlin)
 *
 * This example demonstrates the convenience methods added to the ekoDB Kotlin client:
 * - upsert: Insert or update a record
 * - findOne: Find a single record by field value
 * - exists: Check if a record exists
 * - paginate: Paginate through records
 */
fun main() = runBlocking {
    println("=== ekoDB Convenience Methods Example ===\n")

    val baseUrl = System.getenv("API_BASE_URL") ?: "http://localhost:8080"
    val apiKey = System.getenv("API_BASE_KEY") ?: "a-test-api-key-from-ekodb"
    
    val client = EkoDBClient.builder()
        .baseUrl(baseUrl)
        .apiKey(apiKey)
        .build()

    val collection = "convenience_methods_example_kt"

    println("=== Native Object Creation ===")
    // Kotlin uses Record objects with builder pattern
    val user1 = Record.new()
        .insert("name", "Alice Johnson")
        .insert("email", "alice@example.com")
        .insert("age", 28)
        .insert("active", true)

    val inserted = client.insert(collection, user1)
    println("✓ Created record: $inserted")

    println("\n=== Upsert Operation ===")
    val userId = inserted.fields["id"]?.toString() ?: throw Exception("No ID returned")

    // First upsert - will update (record exists)
    val user2 = Record.new()
        .insert("name", "Alice Johnson")
        .insert("email", "alice.j@newdomain.com")
        .insert("age", 29)

    val upserted1 = client.upsert(collection, userId, user2)
    println("✓ First upsert (update): $upserted1")

    // Second upsert - will insert (different ID, doesn't exist)
    val user3 = Record.new()
        .insert("name", "Bob Smith")
        .insert("email", "bob@example.com")
        .insert("age", 35)

    val upserted2 = client.upsert(collection, "new-user-id", user3)
    println("✓ Second upsert (insert): $upserted2")

    println("\n=== Find One Operation ===")
    // Find a single record by any field
    val foundUser = client.findOne(collection, "email", "alice.j@newdomain.com")
    if (foundUser != null) {
        println("✓ Found user by email: $foundUser")
    }

    // Try finding a non-existent user
    val notFound = client.findOne(collection, "email", "nonexistent@example.com")
    if (notFound == null) {
        println("✓ User not found (as expected)")
    }

    println("\n=== Exists Check ===")
    // Check existence without fetching the record
    val exists = client.exists(collection, userId)
    println("✓ Record exists: $exists")

    // Check non-existent record
    val notExists = client.exists(collection, "fake-id-123")
    println("✓ Fake record exists: $notExists (should be false)")

    println("\n=== Pagination ===")
    // Insert more records for pagination demo
    for (i in 1..25) {
        val record = Record.new()
            .insert("name", "User $i")
            .insert("email", "user$i@example.com")
            .insert("index", i)
        
        client.insert(collection, record)
    }
    println("✓ Inserted 25 records for pagination")

    // Paginate through records
    val pageSize = 10
    for (page in 1..3) {
        val records = client.paginate(collection, page, pageSize)
        val expected = if (page == 3) "~7" else "10"
        println("✓ Page $page: ${records.size} records (expected $expected)")
    }

    println("\n=== Cleanup ===")
    client.deleteCollection(collection)
    println("✓ Deleted collection")

    println("\n✅ All convenience methods demonstrated successfully!")
}
