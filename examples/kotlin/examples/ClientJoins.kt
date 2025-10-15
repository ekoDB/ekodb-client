package io.ekodb.client.examples

import io.ekodb.client.EkoDBClient
import io.ekodb.client.QueryBuilder
import io.ekodb.client.types.FieldType
import io.ekodb.client.types.Record
import kotlinx.coroutines.runBlocking

/**
 * Join Operations example - Demonstrates joining data across multiple collections
 */
fun main() = runBlocking {
    val baseUrl = System.getenv("API_BASE_URL") ?: "http://localhost:8080"
    val apiKey = System.getenv("API_BASE_KEY") ?: error("API_BASE_KEY environment variable not set")
    
    val client = EkoDBClient.builder()
        .baseUrl(baseUrl)
        .apiKey(apiKey)
        .build()
    
    println("=== ekoDB Kotlin Client - Join Operations Example ===\n")
    
    // Use unique collection names
    val usersCollection = "joins_users_kotlin"
    val departmentsCollection = "joins_departments_kotlin"
    val profilesCollection = "joins_profiles_kotlin"
    val ordersCollection = "joins_orders_kotlin"
    
    try {
        // Cleanup any existing test collections
        try { client.deleteCollection(usersCollection) } catch (e: Exception) { }
        try { client.deleteCollection(departmentsCollection) } catch (e: Exception) { }
        try { client.deleteCollection(profilesCollection) } catch (e: Exception) { }
        try { client.deleteCollection(ordersCollection) } catch (e: Exception) { }
        
        // Setup: Create sample data
        println("=== Setting up sample data ===")
        
        // Create users
        val user1 = client.insert(usersCollection, Record.new()
            .insert("name", "Alice Johnson")
            .insert("email", "alice@example.com")
            .insert("department_id", "dept-001")
        )
        val user1Id = (user1["id"] as? FieldType.StringValue)?.value ?: error("No ID")
        
        val user2 = client.insert(usersCollection, Record.new()
            .insert("name", "Bob Smith")
            .insert("email", "bob@example.com")
            .insert("department_id", "dept-002")
        )
        val user2Id = (user2["id"] as? FieldType.StringValue)?.value ?: error("No ID")
        
        // Create departments
        client.insert(departmentsCollection, Record.new()
            .insert("id", "dept-001")
            .insert("name", "Engineering")
            .insert("location", "Building A")
        )
        
        client.insert(departmentsCollection, Record.new()
            .insert("id", "dept-002")
            .insert("name", "Sales")
            .insert("location", "Building B")
        )
        
        // Create orders
        client.insert(ordersCollection, Record.new()
            .insert("user_id", user1Id!!)
            .insert("product", "Laptop")
            .insert("amount", 1200)
            .insert("status", "completed")
        )
        
        client.insert(ordersCollection, Record.new()
            .insert("user_id", user1Id!!)
            .insert("product", "Mouse")
            .insert("amount", 25)
            .insert("status", "completed")
        )
        
        client.insert(ordersCollection, Record.new()
            .insert("user_id", user2Id!!)
            .insert("product", "Keyboard")
            .insert("amount", 75)
            .insert("status", "pending")
        )
        
        println("✓ Sample data created\n")
        
        // Example 1: Simple single collection join
        println("=== Example 1: Single collection join (users with departments) ===")
        
        val query1 = QueryBuilder.new()
            .join(mapOf(
                "collection" to departmentsCollection,
                "local_field" to "department_id",
                "foreign_field" to "id",
                "as" to "department"
            ))
            .limit(10)
            .build()
        
        val results1 = client.find(usersCollection, query1)
        println("✓ Found ${results1.size} users with department data")
        results1.forEach { user ->
            val name = (user["name"] as? FieldType.ObjectValue)?.value?.get("value") as? FieldType.StringValue
            println("  - ${name?.value ?: "Unknown"}")
        }
        println()
        
        // Example 2: Join with filtering
        println("=== Example 2: Join with filtering ===")
        
        val query2 = QueryBuilder.new()
            .eq("department_id", "dept-001")
            .join(mapOf(
                "collection" to departmentsCollection,
                "local_field" to "department_id",
                "foreign_field" to "id",
                "as" to "department"
            ))
            .build()
        
        val results2 = client.find(usersCollection, query2)
        println("✓ Found ${results2.size} users in Engineering")
        results2.forEach { user ->
            val name = (user["name"] as? FieldType.ObjectValue)?.value?.get("value") as? FieldType.StringValue
            println("  - ${name?.value ?: "Unknown"}")
        }
        println()
        
        // Example 3: Multi-collection join
        println("=== Example 3: Multi-collection join (users with departments and profiles) ===")
        
        // Create profiles
        client.insert(profilesCollection, Record.new()
            .insert("id", user1Id!!)
            .insert("bio", "Senior Software Engineer")
        )
        
        client.insert(profilesCollection, Record.new()
            .insert("id", user2Id!!)
            .insert("bio", "Sales Manager")
        )
        
        val query3 = QueryBuilder.new()
            .join(mapOf(
                "collection" to departmentsCollection,
                "local_field" to "department_id",
                "foreign_field" to "id",
                "as" to "department"
            ))
            .limit(10)
            .build()
        
        val results3 = client.find(usersCollection, query3)
        println("✓ Found ${results3.size} users with department join")
        results3.forEach { user ->
            val name = (user["name"] as? FieldType.ObjectValue)?.value?.get("value") as? FieldType.StringValue
            println("  - ${name?.value ?: "Unknown"}")
        }
        println()
        
        // Example 4: Join orders with user data
        println("=== Example 4: Join orders with user data ===")
        
        val query4 = QueryBuilder.new()
            .eq("status", "completed")
            .join(mapOf(
                "collection" to usersCollection,
                "local_field" to "user_id",
                "foreign_field" to "id",
                "as" to "user"
            ))
            .build()
        
        val results4 = client.find(ordersCollection, query4)
        println("✓ Found ${results4.size} completed orders")
        results4.forEach { order ->
            val product = (order["product"] as? FieldType.ObjectValue)?.value?.get("value") as? FieldType.StringValue
            val amount = (order["amount"] as? FieldType.ObjectValue)?.value?.get("value") as? FieldType.IntegerValue
            println("  - ${product?.value ?: "Unknown"} (\$${amount?.value ?: 0})")
        }
        println()
        
        // Example 5: Complex join with multiple conditions
        println("=== Example 5: Complex join with multiple conditions ===")
        
        val query5 = QueryBuilder.new()
            .contains("email", "@example.com")
            .join(mapOf(
                "collection" to departmentsCollection,
                "local_field" to "department_id",
                "foreign_field" to "id",
                "as" to "department"
            ))
            .sortAscending("name")
            .limit(5)
            .build()
        
        val results5 = client.find(usersCollection, query5)
        println("✓ Found ${results5.size} users with example.com emails")
        results5.forEach { user ->
            val name = (user["name"] as? FieldType.ObjectValue)?.value?.get("value") as? FieldType.StringValue
            val email = (user["email"] as? FieldType.ObjectValue)?.value?.get("value") as? FieldType.StringValue
            println("  - ${name?.value ?: "Unknown"} (${email?.value ?: "Unknown"})")
        }
        println()
        
        // Cleanup
        println("=== Cleanup ===")
        client.deleteCollection(usersCollection)
        client.deleteCollection(departmentsCollection)
        client.deleteCollection(profilesCollection)
        client.deleteCollection(ordersCollection)
        println("✓ Deleted test collections")
        
    } finally {
        client.close()
        println("\n✓ Join operations example completed successfully")
    }
}
