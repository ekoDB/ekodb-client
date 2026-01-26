/**
 * Field Projection Examples for Kotlin Client
 *
 * Demonstrates how to use selectFields and excludeFields to optimize
 * data transfer and improve query performance.
 *
 * Shows comparison between full object queries and projected queries.
 */

package io.ekodb.client.examples

import io.ekodb.client.*
import io.ekodb.client.types.FieldType
import io.ekodb.client.types.Record
import io.github.cdimascio.dotenv.dotenv
import kotlinx.coroutines.runBlocking
import java.time.Instant

const val TEST_COLLECTION = "projection_test_users_kt"

fun main() = runBlocking {
    val dotenv = dotenv()
    val baseUrl = dotenv["API_BASE_URL"] ?: "http://localhost:8080"
    val apiKey = dotenv["API_BASE_KEY"] ?: "a-test-api-key-from-ekodb"

    val client = EkoDBClient.builder()
        .baseUrl(baseUrl)
        .apiKey(apiKey)
        .build()

    println("Client created\n")

    // Setup: Create test collection and insert test data
    println("Setting up test data...")
    // Try to delete existing collection (ignore error if doesn't exist)
    try {
        client.deleteCollection(TEST_COLLECTION)
    } catch (e: Exception) {
        // Collection may not exist, that's fine
    }

    // Insert test users with various fields
    val testUsers = listOf(
        Record.new()
            .insert("name", "Alice Johnson")
            .insert("email", "alice@example.com")
            .insert("status", "active")
            .insert("user_role", "admin")
            .insert("age", 30)
            .insert("bio", "Software engineer")
            .insert("avatar_url", "https://example.com/alice.jpg")
            .insert("password", "secret123")
            .insert("api_key", "key123")
            .insert("secret_token", "token123")
            .insert("created_at", Instant.now().toString()),
        Record.new()
            .insert("name", "Bob Smith")
            .insert("email", "bob@example.com")
            .insert("status", "active")
            .insert("user_role", "user")
            .insert("age", 25)
            .insert("bio", "Designer")
            .insert("avatar_url", "https://example.com/bob.jpg")
            .insert("password", "secret456")
            .insert("api_key", "key456")
            .insert("secret_token", "token456")
            .insert("created_at", Instant.now().toString()),
        Record.new()
            .insert("name", "Carol White")
            .insert("email", "carol@example.com")
            .insert("status", "inactive")
            .insert("user_role", "user")
            .insert("age", 35)
            .insert("bio", "Manager")
            .insert("avatar_url", "https://example.com/carol.jpg")
            .insert("password", "secret789")
            .insert("api_key", "key789")
            .insert("secret_token", "token789")
            .insert("created_at", Instant.now().toString()),
        Record.new()
            .insert("name", "Dave Brown")
            .insert("email", "dave@example.com")
            .insert("status", "active")
            .insert("user_role", "admin")
            .insert("age", 45)
            .insert("bio", "CTO")
            .insert("avatar_url", "https://example.com/dave.jpg")
            .insert("password", "secretabc")
            .insert("api_key", "keyabc")
            .insert("secret_token", "tokenabc")
            .insert("created_at", Instant.now().toString())
    )

    val insertedIds = mutableListOf<String>()
    for (user in testUsers) {
        val result = client.insert(TEST_COLLECTION, user)
        val id = (result["id"] as? FieldType.StringValue)?.value
        if (id != null) {
            insertedIds.add(id)
        }
    }
    println("Inserted ${insertedIds.size} test users\n")

    // Example 1: Select specific fields
    println("Example 1: Select specific fields")
    val usersQuery = QueryBuilder.new()
        .eq("status", "active")
        .selectFields("id", "name", "email")
        .limit(10)
        .build()

    val users = client.find(TEST_COLLECTION, usersQuery)
    println("Fetched ${users.size} users with only 3 fields each")

    // Example 2: Exclude sensitive fields
    println("\nExample 2: Exclude sensitive fields")
    val adminsQuery = QueryBuilder.new()
        .eq("user_role", "admin")
        .excludeFields("password", "api_key", "secret_token")
        .build()

    val admins = client.find(TEST_COLLECTION, adminsQuery)
    println("Fetched ${admins.size} admins without sensitive data")
    // Verify sensitive fields are excluded
    if (admins.isNotEmpty()) {
        val hasPassword = admins[0].containsKey("password")
        println("  Password field excluded: ${!hasPassword}")
    }

    // Example 3: Complex query with projection
    println("\nExample 3: Complex query with projection")
    val activeUsersQuery = QueryBuilder.new()
        .eq("status", "active")
        .gte("age", 18)
        .lt("age", 65)
        .selectFields("id", "name", "email", "age", "created_at")
        .sortDesc("created_at")
        .limit(50)
        .build()

    val activeUsers = client.find(TEST_COLLECTION, activeUsersQuery)
    println("Fetched ${activeUsers.size} active users with profile fields")

    // Example 4: Find by ID with projection
    println("\nExample 4: Find by ID with projection")
    if (insertedIds.isNotEmpty()) {
        val profileQuery = QueryBuilder.new()
            .eq("id", insertedIds[0])
            .selectFields("id", "name", "email", "bio", "avatar_url")
            .build()

        val userProfiles = client.find(TEST_COLLECTION, profileQuery)
        if (userProfiles.isNotEmpty()) {
            val name = getValue<String>(userProfiles[0]["name"]) ?: "N/A"
            println("Fetched user profile: $name")
        } else {
            println("Fetched user profile: N/A")
        }
    }

    // Example 5: Compare full vs projected data
    println("\nExample 5: Compare full vs projected data")
    val fullQuery = QueryBuilder.new()
        .eq("status", "active")
        .build()

    val projectedQuery = QueryBuilder.new()
        .eq("status", "active")
        .selectFields("id", "name", "email")
        .build()

    val fullUsers = client.find(TEST_COLLECTION, fullQuery)
    val projectedUsers = client.find(TEST_COLLECTION, projectedQuery)

    val fullFields = if (fullUsers.isNotEmpty()) fullUsers[0].size else 0
    val projectedFields = if (projectedUsers.isNotEmpty()) projectedUsers[0].size else 0

    println("Full query returned $fullFields fields per user")
    println("Projected query returned $projectedFields fields per user")

    // Cleanup
    println("\nCleaning up test data...")
    client.deleteCollection(TEST_COLLECTION)
    println("Cleanup complete")

    println("\nAll projection examples completed successfully!")

    client.close()
}
