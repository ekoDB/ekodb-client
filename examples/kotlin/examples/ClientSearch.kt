package io.ekodb.client.examples

import io.ekodb.client.EkoDBClient
import io.ekodb.client.types.Record
import io.github.cdimascio.dotenv.dotenv
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.buildJsonObject
import kotlinx.serialization.json.put

/**
 * Search example - Full-text search operations
 */
fun main() = runBlocking {
    val dotenv = dotenv()
    val baseUrl = dotenv["API_BASE_URL"] ?: "http://localhost:8080"
    val apiKey = dotenv["API_BASE_KEY"] ?: "a-test-api-key-from-ekodb"
    
    val client = EkoDBClient.builder()
        .baseUrl(baseUrl)
        .apiKey(apiKey)
        .build()
    
    val collection = "kotlin_search_example"
    
    println("=== ekoDB Kotlin Client - Search Example ===\n")
    
    try {
        // Cleanup any existing collection
        try {
            client.deleteCollection(collection)
        } catch (e: Exception) {
            // Ignore if doesn't exist
        }
        
        // Step 1: Insert sample documents
        println("=== Inserting Sample Documents ===")
        
        val docs = listOf(
            Triple("Rust Programming", "Learn Rust programming language with hands-on examples and best practices.", listOf("programming", "rust", "tutorial")),
            Triple("Python for Data Science", "Master Python for data analysis, machine learning, and visualization.", listOf("programming", "python", "data-science")),
            Triple("JavaScript Web Development", "Build modern web applications using JavaScript, React, and Node.js.", listOf("programming", "javascript", "web")),
            Triple("Database Design", "Learn database design principles, normalization, and query optimization.", listOf("database", "design", "sql")),
            Triple("Machine Learning Basics", "Introduction to machine learning algorithms and neural networks.", listOf("ai", "machine-learning", "python"))
        )
        
        for ((title, description, tags) in docs) {
            val doc = Record.new()
                .insert("title", title)
                .insert("description", description)
                .insert("tags", tags.joinToString(","))
                .insert("views", (Math.random() * 1000).toInt())
            
            client.insert(collection, doc)
        }
        println("✓ Inserted ${docs.size} sample documents\n")
        
        // Step 2: Basic text search
        println("=== Basic Text Search ===")
        
        val searchQuery = buildJsonObject {
            put("query", "programming")
            put("min_score", 0.1)
            put("limit", 3)
        }
        
        val results = client.search(collection, searchQuery)
        println("✓ Search results for 'programming':")
        println("  $results\n")
        
        // Step 3: Search with different term
        println("=== Search for 'machine learning' ===")
        
        val mlSearchQuery = buildJsonObject {
            put("query", "machine learning")
            put("limit", 5)
        }
        
        val mlResults = client.search(collection, mlSearchQuery)
        println("✓ Found results for 'machine learning'")
        println("  $mlResults\n")
        
    } finally {
        // Cleanup
        println("=== Cleanup ===")
        try {
            client.deleteCollection(collection)
            println("✓ Deleted collection: $collection")
        } catch (e: Exception) {
            println("⚠ Could not delete collection: ${e.message}")
        }
        
        client.close()
        println("\n✓ All search operations completed successfully")
    }
}
