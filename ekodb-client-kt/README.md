# ekoDB Kotlin Client Library

Official Kotlin client library for [ekoDB](https://ekodb.io) - A
high-performance database with intelligent caching, real-time capabilities, AI
integration, and automatic optimization.

[![Maven Central](https://img.shields.io/maven-central/v/io.ekodb/ekodb-client-kt)](https://central.sonatype.com/artifact/io.ekodb/ekodb-client-kt)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## Features

- ✅ **Coroutines**: Built on Kotlin coroutines for async operations
- ✅ **Type-Safe**: Leverages Kotlin's type system for compile-time safety
- ✅ **Auto-Retry**: Automatic retry with exponential backoff for transient
  failures
- ✅ **Connection Pooling**: Efficient HTTP connection management with Ktor
- ✅ **Query Builder**: Fluent API for building complex queries with joins
- ✅ **Batch Operations**: Efficient bulk inserts, updates, and deletes
- ✅ **TTL Support**: Automatic document expiration with time-to-live
- ✅ **Key-Value Operations**: Simple key-value store operations with TTL
- ✅ **Collection Management**: Create, list, count, and delete collections
- ✅ **Schema Management**: Define and validate collection schemas
- ✅ **Full-Text Search**: Powerful search capabilities across collections
- ✅ **Join Operations**: Query related data across multiple collections
- ✅ **WebSocket Support**: Real-time data streaming and queries
- ✅ **AI/Chat Integration**: Built-in AI chat with context-aware responses
  - Chat sessions with message history
  - Message regeneration and editing
  - Context retrieval from collections
  - Session branching and merging

## Installation

### Gradle (Kotlin DSL)

```kotlin
dependencies {
    implementation("io.ekodb:ekodb-client-kt:0.10.0")
}
```

### Gradle (Groovy)

```groovy
dependencies {
    implementation 'io.ekodb:ekodb-client-kt:0.10.0'
}
```

### Maven

```xml
<dependency>
    <groupId>io.ekodb</groupId>
    <artifactId>ekodb-client-kt</artifactId>
    <version>0.10.0</version>
</dependency>
```

## Quick Start

```kotlin
import io.ekodb.client.EkoDBClient
import io.ekodb.client.QueryBuilder
import io.ekodb.client.types.Record
import kotlinx.coroutines.runBlocking

fun main() = runBlocking {
    // Create a client
    val client = EkoDBClient.builder()
        .baseUrl("http://localhost:8080")
        .apiKey("your-api-key")
        .build()

    // Insert a record
    val record = Record.new()
        .insert("name", "John Doe")
        .insert("age", 30)
        .insert("email", "john@example.com")

    val result = client.insert("users", record)
    println("Inserted: $result")

    // Query with the builder
    val query = QueryBuilder.new()
        .gte("age", 18)
        .eq("status", "active")
        .limit(10)
        .build()

    val users = client.find("users", query)
    println("Found ${users.size} users")

    // Clean up
    client.close()
}
```

## Examples

### CRUD Operations

```kotlin
import io.ekodb.client.EkoDBClient
import io.ekodb.client.types.Record
import kotlinx.coroutines.runBlocking

fun main() = runBlocking {
    val client = EkoDBClient.builder()
        .baseUrl("http://localhost:8080")
        .apiKey("your-api-key")
        .build()

    // Insert
    val user = Record.new()
        .insert("name", "Alice")
        .insert("age", 25)
    val inserted = client.insert("users", user)

    // Extract ID
    val userId = (inserted["id"] as? FieldType.StringValue)?.value
        ?: error("No ID returned")

    // Find by ID
    val found = client.findById("users", userId)

    // Update
    val updates = Record.new()
        .insert("age", 26)
    val updated = client.update("users", userId, updates)

    // Delete
    client.delete("users", userId)

    client.close()
}
```

### Query Builder

```kotlin
import io.ekodb.client.QueryBuilder

// Simple queries
val query = QueryBuilder.new()
    .eq("status", "active")
    .gte("age", 18)
    .lt("age", 65)
    .build()

val results = client.find("users", query)

// Complex queries with sorting and pagination
val complexQuery = QueryBuilder.new()
    .inArray("status", listOf("active", "pending"))
    .regex("email", ".*@example\\.com$")
    .sortDesc("created_at")
    .limit(20)
    .skip(0)
    .build()

val pagedResults = client.find("users", complexQuery)
```

### Batch Operations

```kotlin
// Create records for batch insert
val user1 = Record.new()
    .insert("name", "Alice")
    .insert("email", "alice@example.com")

val user2 = Record.new()
    .insert("name", "Bob")
    .insert("email", "bob@example.com")

// Batch insert
val result = client.batchInsert("users", listOf(user1, user2))
println("Batch completed: ${result.successful.size} successful, ${result.failed.size} failed")
```

### TTL (Time-To-Live) Support

```kotlin
// Insert with 1 hour TTL
val record = Record.new()
    .insert("name", "Session Data")
    .insert("token", "abc123")

client.insertWithTtl("sessions", record, "1h")

// Insert with 5 minutes TTL
client.insertWithTtl("cache", data, "5m")

// Supported TTL formats: "30s", "5m", "1h", "2d"
```

### Key-Value Operations

```kotlin
// Set a key-value pair
client.kvSet("session:user123", mapOf(
    "userId" to 123,
    "username" to "john_doe"
))

// Get a value
val session = client.kvGet("session:user123")

// Delete a key
client.kvDelete("session:user123")

// Set with TTL
client.kvSetWithTtl("cache:product:1", productData, "1h")
```

### Collection Management

```kotlin
// List all collections
val collections = client.listCollections()
println("Collections: $collections")

// Count documents in a collection
val count = client.count("users")
println("Total users: $count")

// Check if collection exists
val exists = client.collectionExists("users")

// Delete a collection
client.deleteCollection("old_data")
```

### Schema Management

```kotlin
import kotlinx.serialization.json.buildJsonObject
import kotlinx.serialization.json.put
import kotlinx.serialization.json.putJsonObject

// Create a collection with schema
val schema = buildJsonObject {
    putJsonObject("fields") {
        putJsonObject("email") {
            put("field_type", "String")
            put("required", true)
        }
        putJsonObject("age") {
            put("field_type", "Integer")
            put("required", false)
        }
    }
}

client.createCollectionWithSchema("users", schema)

// Get collection schema
val currentSchema = client.getCollectionSchema("users")
```

### Full-Text Search

```kotlin
import kotlinx.serialization.json.buildJsonObject
import kotlinx.serialization.json.put

// Search across fields
val searchQuery = buildJsonObject {
    put("query", "programming")
    put("limit", 10)
}

val results = client.search("articles", searchQuery)
println("Found ${results.size} articles")
```

### Join Operations

```kotlin
// Simple join with another collection
val query = QueryBuilder.new()
    .eq("status", "active")
    .joinWith(
        collection = "departments",
        localField = "department_id",
        foreignField = "id",
        alias = "department"
    )
    .limit(10)
    .build()

val usersWithDepts = client.find("users", query)

// Or use the generic join method
val joinQuery = QueryBuilder.new()
    .join(mapOf(
        "collection" to "departments",
        "local_field" to "department_id",
        "foreign_field" to "id",
        "as" to "department"
    ))
    .build()
```

### WebSocket Operations

```kotlin
// Create WebSocket client
val wsClient = client.websocket("ws://localhost:8080")

// Query via WebSocket
val query = QueryBuilder.new()
    .eq("status", "active")
    .build()

val response = wsClient.find("users", query)
println("WebSocket response: $response")

// Close WebSocket
wsClient.close()
```

### AI/Chat Integration

```kotlin
import kotlinx.serialization.json.buildJsonObject
import kotlinx.serialization.json.put
import kotlinx.serialization.json.putJsonObject
import kotlinx.serialization.json.putJsonArray

// Create a chat session
val sessionRequest = buildJsonObject {
    put("provider", "openai")
    put("model", "gpt-4")
    put("system_prompt", "You are a helpful assistant.")
    putJsonArray("collections") {
        add(buildJsonObject {
            put("collection_name", "products")
            putJsonArray("fields") {}
        })
    }
}

val session = client.createChatSession(sessionRequest)
val chatId = session["chat_id"]?.jsonPrimitive?.content ?: error("No chat_id")

// Send a message
val messageRequest = buildJsonObject {
    put("message", "What products are available?")
}

val response = client.chatMessage(chatId, messageRequest)
println("AI Response: ${response["responses"]}")

// Get message history
val history = client.getChatSessionMessages(chatId, buildJsonObject {
    put("limit", 10)
})

// Regenerate a message
val regenerated = client.regenerateChatMessage(chatId, messageId)

// Update a message
val updateRequest = buildJsonObject {
    put("content", "Updated message text")
}
client.updateChatMessage(chatId, messageId, updateRequest)

// Delete chat session
client.deleteChatSession(chatId)
```

## Complete Examples

For complete, runnable examples of all features, see the
[examples directory](../examples/kotlin/examples/):

- **ClientSimpleCrud.kt** - Basic CRUD operations
- **ClientBatchOperations.kt** - Batch inserts, updates, deletes
- **ClientQueryBuilder.kt** - Complex queries with filters and sorting
- **ClientKvOperations.kt** - Key-value store operations
- **ClientCollectionManagement.kt** - Collection management
- **ClientDocumentTtl.kt** - TTL and expiration
- **ClientSchemaManagement.kt** - Schema definition and validation
- **ClientSearch.kt** - Full-text search
- **ClientJoins.kt** - Join operations across collections
- **ClientSimpleWebsocket.kt** - WebSocket basics
- **ClientWebsocketTtl.kt** - WebSocket with TTL queries
- **ClientChatBasic.kt** - Basic AI chat
- **ClientChatSessions.kt** - Chat session management
- **ClientChatAdvanced.kt** - Advanced chat features
- **ClientChatModels.kt** - Chat models API
- **ClientUserFunctions.kt** - User functions API

## API Reference

### Client Methods

#### Document Operations

- `insert(collection, record)` - Insert a document
- `insertWithTtl(collection, record, ttl)` - Insert with expiration
- `findById(collection, id)` - Find document by ID
- `find(collection, query)` - Query documents with filters
- `findAll(collection)` - Get all documents in collection
- `update(collection, id, updates)` - Update a document
- `delete(collection, id)` - Delete a document

#### Batch Operations

- `batchInsert(collection, records)` - Bulk insert documents

#### Query Building

- `QueryBuilder.new()` - Create a new query builder
- `.eq(field, value)` - Equal to
- `.ne(field, value)` - Not equal to
- `.gt(field, value)` - Greater than
- `.gte(field, value)` - Greater than or equal
- `.lt(field, value)` - Less than
- `.lte(field, value)` - Less than or equal
- `.inArray(field, values)` - In array
- `.nin(field, values)` - Not in array
- `.regex(field, pattern)` - Regex match
- `.contains(field, substring)` - Contains substring
- `.sortAsc(field)` / `.sortDesc(field)` - Sorting
- `.sortAscending(field)` / `.sortDescending(field)` - Sorting (aliases)
- `.limit(n)` / `.skip(n)` - Pagination
- `.join(joinConfig)` - Add join configuration
- `.joinWith(collection, localField, foreignField, alias)` - Simple join helper
- `.build()` - Build the query

#### Key-Value Operations

- `kvSet(key, value)` - Set a key-value pair
- `kvSetWithTtl(key, value, ttl)` - Set with expiration
- `kvGet(key)` - Get value by key
- `kvDelete(key)` - Delete a key

#### Collection Operations

- `listCollections()` - List all collections
- `count(collection)` - Count documents in collection
- `collectionExists(collection)` - Check if collection exists
- `deleteCollection(collection)` - Delete entire collection
- `createCollectionWithSchema(collection, schema)` - Create with schema
- `getCollectionSchema(collection)` - Get collection schema

#### Search Operations

- `search(collection, query)` - Full-text search across collection

#### WebSocket Operations

- `websocket(wsUrl)` - Create WebSocket client
- WebSocket client methods:
  - `find(collection, query)` - Query via WebSocket
  - `close()` - Close WebSocket connection

#### AI/Chat Operations

- `createChatSession(request)` - Create new chat session
- `chatMessage(chatId, request)` - Send message to chat
- `getChatSessionMessages(chatId, query)` - Get message history
- `getChatSession(chatId)` - Get session details
- `updateChatSession(chatId, request)` - Update session configuration
- `deleteChatSession(chatId)` - Delete chat session
- `listChatSessions(query)` - List all chat sessions
- `branchChatSession(chatId, request)` - Create session branch
- `mergeChatSessions(request)` - Merge multiple sessions
- `regenerateChatMessage(chatId, messageId)` - Regenerate AI response
- `updateChatMessage(chatId, messageId, request)` - Edit message
- `deleteChatMessage(chatId, messageId)` - Delete message
- `toggleForgottenMessage(chatId, messageId, request)` - Toggle forgotten status

#### Chat Models

- `getChatModels()` - Get all available chat models organized by provider
- `getChatModel(provider)` - Get models for a specific provider (e.g., "openai",
  "anthropic")

#### User Functions

- `saveUserFunction(userFunction)` - Create a new user function
- `getUserFunction(label)` - Get user function by its label
- `listUserFunctions(tags?)` - List all user functions (optionally filter by
  tags)
- `updateUserFunction(label, userFunction)` - Update existing user function
- `deleteUserFunction(label)` - Delete user function by label

## Best Practices

### Connection Management

```kotlin
// Create one client instance and reuse it
val client = EkoDBClient.builder()
    .baseUrl(baseUrl)
    .apiKey(apiKey)
    .timeout(30)
    .maxRetries(3)
    .build()

// Use the client throughout your application
// Close it when done
client.close()
```

### Error Handling

```kotlin
try {
    val result = client.insert("users", record)
    // Handle success
} catch (e: Exception) {
    // Handle error
    println("Error: ${e.message}")
}
```

### Performance Tips

1. **Use Batch Operations** - For multiple inserts/updates/deletes
2. **Reuse Client** - Create one client instance and reuse it
3. **Set Appropriate Timeouts** - Configure timeouts based on your use case
4. **Use Pagination** - For large result sets, use `.limit()` and `.skip()`

## Requirements

- Kotlin 1.9.22 or higher
- JVM 17 or higher
- Coroutines support

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file
for details.

## Resources

- [GitHub Repository](https://github.com/ekoDB/ekodb-client)
- [ekoDB Documentation](https://ekodb.io/docs)
- [API Reference](https://ekodb.io/docs/api)
