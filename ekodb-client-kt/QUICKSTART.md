# ekoDB Kotlin Client - Quick Start Guide

Get started with ekoDB Kotlin client in 5 minutes!

## Installation

### Gradle (Kotlin DSL)

```kotlin
dependencies {
    implementation("io.ekodb:ekodb-client-kt:0.1.0")
}
```

### Gradle (Groovy)

```groovy
dependencies {
    implementation 'io.ekodb:ekodb-client-kt:0.1.0'
}
```

### Maven

```xml
<dependency>
    <groupId>io.ekodb</groupId>
    <artifactId>ekodb-client-kt</artifactId>
    <version>0.1.0</version>
</dependency>
```

## Basic Usage

### 1. Create a Client

```kotlin
import io.ekodb.client.EkoDBClient

suspend fun main() {
    // Create client
    val client = EkoDBClient(
        baseUrl = "http://localhost:8080",
        apiKey = "your-api-key"
    )

    // Use the client
    // ... your code here

    // Close when done
    client.close()
}
```

### 2. Insert Documents

```kotlin
// Insert a single document
val user = mapOf(
    "name" to "Alice",
    "email" to "alice@example.com",
    "age" to 30
)

val inserted = client.insert("users", user)
println("Inserted: ${inserted["id"]}")
```

### 3. Query Documents

```kotlin
// Find all documents
val allUsers = client.find("users")

// Query with filters
val adults = client.query("users") {
    gte("age", 18)
    limit(10)
}

// Find by ID
val user = client.findById("users", "user-id-123")
```

### 4. Update Documents

```kotlin
// Update a document
val updates = mapOf("age" to 31)
client.update("users", "user-id-123", updates)

// Batch update
val batchUpdates = listOf(
    mapOf("id" to "user-1", "status" to "active"),
    mapOf("id" to "user-2", "status" to "active")
)
client.batchUpdate("users", batchUpdates)
```

### 5. Delete Documents

```kotlin
// Delete by ID
client.delete("users", "user-id-123")

// Delete with query
client.deleteWhere("users") {
    eq("status", "inactive")
}
```

## Advanced Features

### Query Builder

```kotlin
// Complex queries with fluent API
val results = client.query("users") {
    eq("status", "active")
    gte("age", 18)
    contains("email", "@example.com")
    sortAsc("name")
    limit(20)
    offset(0)
}
```

### Joins

```kotlin
// Join with another collection
val usersWithOrders = client.query("users") {
    join {
        collection = "orders"
        localField = "id"
        foreignField = "user_id"
        `as` = "orders"
    }
}
```

### Full-Text Search

```kotlin
// Search across all fields
val results = client.search("users", "alice engineer")

// Search with options
val searchResults = client.search("users", "alice") {
    limit(10)
    fields(listOf("name", "bio"))
}
```

### WebSocket Real-Time Queries

```kotlin
// Subscribe to real-time updates
client.subscribeToQuery("users", query {
    eq("status", "active")
}) { event ->
    when (event.type) {
        "insert" -> println("New user: ${event.data}")
        "update" -> println("User updated: ${event.data}")
        "delete" -> println("User deleted: ${event.id}")
    }
}
```

### AI/Chat Integration

```kotlin
// Create a chat session
val sessionId = client.createChatSession(
    collectionName = "documents",
    systemPrompt = "You are a helpful assistant."
)

// Send a message
val response = client.sendChatMessage(
    sessionId = sessionId,
    message = "What documents do we have?"
)

println("AI: ${response.content}")
```

### Schema Management

```kotlin
// Define a schema
val schema = mapOf(
    "name" to mapOf("type" to "string", "required" to true),
    "email" to mapOf("type" to "string", "required" to true),
    "age" to mapOf("type" to "integer", "minimum" to 0)
)

// Set schema for collection
client.setSchema("users", schema)

// Get schema
val currentSchema = client.getSchema("users")
```

### TTL (Time-To-Live)

```kotlin
// Insert with TTL (expires in 1 hour)
val tempData = mapOf(
    "session" to "abc123",
    "data" to "temporary"
)
client.insert("sessions", tempData, ttl = 3600)
```

### Batch Operations

```kotlin
// Batch insert
val users = listOf(
    mapOf("name" to "Alice", "age" to 30),
    mapOf("name" to "Bob", "age" to 25),
    mapOf("name" to "Charlie", "age" to 35)
)
val inserted = client.batchInsert("users", users)

// Batch delete
val ids = listOf("id1", "id2", "id3")
client.batchDelete("users", ids)
```

## Error Handling

```kotlin
import io.ekodb.client.EkoDBException

try {
    val user = client.findById("users", "invalid-id")
} catch (e: EkoDBException) {
    when (e.statusCode) {
        404 -> println("User not found")
        401 -> println("Unauthorized")
        else -> println("Error: ${e.message}")
    }
}
```

## Complete Example

```kotlin
import io.ekodb.client.EkoDBClient
import kotlinx.coroutines.runBlocking

fun main() = runBlocking {
    // Create client
    val client = EkoDBClient(
        baseUrl = "http://localhost:8080",
        apiKey = "your-api-key"
    )

    try {
        // Insert a user
        val user = mapOf(
            "name" to "Alice Johnson",
            "email" to "alice@example.com",
            "age" to 30,
            "role" to "engineer"
        )

        val inserted = client.insert("users", user)
        val userId = inserted["id"] as String
        println("Created user: $userId")

        // Query users
        val engineers = client.query("users") {
            eq("role", "engineer")
            gte("age", 25)
            sortAsc("name")
        }

        println("Found ${engineers.size} engineers")

        // Update user
        client.update("users", userId, mapOf("age" to 31))
        println("Updated user age")

        // Search
        val results = client.search("users", "alice engineer")
        println("Search found ${results.size} results")

        // Clean up
        client.delete("users", userId)
        println("Deleted user")

    } finally {
        client.close()
    }
}
```

## Next Steps

- [Full API Documentation](README.md)
- [Examples](../examples/kotlin/examples/)
- [Publishing Guide](PUBLISHING.md)
- [ekoDB Documentation](https://github.com/ekoDB/ekodb)

## Support

- GitHub Issues: https://github.com/ekoDB/ekodb-client/issues
- Documentation: https://github.com/ekoDB/ekodb-client
- Email: support@ekodb.io
