package io.ekodb.client.examples

import io.ekodb.client.EkoDBClient
import io.ekodb.client.QueryBuilder
import io.ekodb.client.types.Record
import kotlinx.coroutines.runBlocking

/**
 * Query builder example - Build complex queries with filters, sorting, and pagination
 */
fun main() = runBlocking {
    val baseUrl = System.getenv("API_BASE_URL") ?: "http://localhost:8080"
    val apiKey = System.getenv("API_BASE_KEY") ?: error("API_BASE_KEY environment variable not set")
    
    val client = EkoDBClient.builder()
        .baseUrl(baseUrl)
        .apiKey(apiKey)
        .build()
    
    val collection = "kotlin_query_example"
    
    println("=== ekoDB Kotlin Client - Query Builder Example ===\n")
    
    try {
        // Insert sample data
        println("=== Inserting Sample Data ===")
        val sampleData = listOf(
            Record.new().insert("name", "Alice").insert("age", 28).insert("city", "NYC").insert("score", 85),
            Record.new().insert("name", "Bob").insert("age", 35).insert("city", "LA").insert("score", 92),
            Record.new().insert("name", "Charlie").insert("age", 22).insert("city", "NYC").insert("score", 78),
            Record.new().insert("name", "Diana").insert("age", 31).insert("city", "Chicago").insert("score", 95),
            Record.new().insert("name", "Eve").insert("age", 27).insert("city", "LA").insert("score", 88)
        )
        
        client.batchInsert(collection, sampleData)
        println("✓ Inserted ${sampleData.size} records\n")
        
        // Example 1: Simple equality query
        println("=== Query 1: Equality (city = 'NYC') ===")
        val query1 = QueryBuilder.new()
            .eq("city", "NYC")
            .build()
        
        val results1 = client.find(collection, query1)
        println("✓ Found ${results1.size} records in NYC\n")
        
        // Example 2: Range query
        println("=== Query 2: Range (age >= 25 AND age < 32) ===")
        val query2 = QueryBuilder.new()
            .gte("age", 25)
            .lt("age", 32)
            .build()
        
        val results2 = client.find(collection, query2)
        println("✓ Found ${results2.size} records with age 25-31\n")
        
        // Example 3: Query with sorting
        println("=== Query 3: Sort by score (descending) ===")
        val query3 = QueryBuilder.new()
            .sortDesc("score")
            .limit(3)
            .build()
        
        val results3 = client.find(collection, query3)
        println("✓ Top 3 scores:")
        results3.forEach { record ->
            println("  - Score: ${record.fields["score"]}")
        }
        println()
        
        // Example 4: Complex query with multiple conditions
        println("=== Query 4: Complex (score > 80 AND age >= 25) ===")
        val query4 = QueryBuilder.new()
            .gt("score", 80)
            .gte("age", 25)
            .sortAsc("age")
            .build()
        
        val results4 = client.find(collection, query4)
        println("✓ Found ${results4.size} high-scoring adults\n")
        
        // Example 5: IN query
        println("=== Query 5: IN (city IN ['NYC', 'LA']) ===")
        val query5 = QueryBuilder.new()
            .inArray("city", listOf("NYC", "LA"))
            .build()
        
        val results5 = client.find(collection, query5)
        println("✓ Found ${results5.size} records in NYC or LA\n")
        
        // Example 6: Pagination
        println("=== Query 6: Pagination (skip 2, limit 2) ===")
        val query6 = QueryBuilder.new()
            .sortAsc("name")
            .skip(2)
            .limit(2)
            .build()
        
        val results6 = client.find(collection, query6)
        println("✓ Page 2 (2 records):")
        results6.forEach { record ->
            println("  - ${record.fields["name"]}")
        }
        println()
        
        // Example 7: Contains (substring search)
        println("=== Query 7: Contains (name contains 'a') ===")
        val query7 = QueryBuilder.new()
            .contains("name", "a")
            .build()
        
        val results7 = client.find(collection, query7)
        println("✓ Found ${results7.size} names containing 'a'\n")
        
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
        println("\n=== Example Complete ===")
    }
}
