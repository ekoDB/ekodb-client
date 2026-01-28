# ekoDB Rust Client Library

Official Rust client library for [ekoDB](https://ekodb.io) - A high-performance
database with intelligent caching, real-time capabilities, AI integration, and
automatic optimization.

[![Crates.io](https://img.shields.io/crates/v/ekodb_client.svg)](https://crates.io/crates/ekodb_client)
[![Documentation](https://docs.rs/ekodb_client/badge.svg)](https://docs.rs/ekodb_client)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

## Features

- ✅ **Async/Await**: Built on Tokio for high-performance async operations
- ✅ **Type-Safe**: Strong typing with Rust's type system and comprehensive
  error handling
- ✅ **Auto-Retry**: Automatic retry with exponential backoff for transient
  failures (429, 503, timeouts)
- ✅ **Connection Pooling**: Efficient HTTP connection management
- ✅ **Query Builder**: Fluent API for building complex queries with operators,
  sorting, and pagination
- ✅ **Search**: Full-text search, vector search, and hybrid search with scoring
- ✅ **AI Chat**: Natural language queries with context-aware AI responses
  (OpenAI, Anthropic, Perplexity)
- ✅ **Schema Management**: Define and enforce data schemas with validation
- ✅ **WebSocket Support**: Real-time queries and subscriptions with automatic
  reconnection
- ✅ **Batch Operations**: Efficient bulk inserts, updates, and deletes
- ✅ **TTL Support**: Automatic document expiration with time-to-live
- ✅ **Key-Value Operations**: Simple key-value store operations
- ✅ **Collection Management**: Create, list, count, and delete collections
- ✅ **Token Caching**: Automatic authentication token management and refresh

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
ekodb_client = "0.1"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

```rust
use ekodb_client::{Client, QueryBuilder, Record};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client (automatically handles authentication)
    let client = Client::builder()
        .base_url("http://localhost:8080")
        .api_key("your-api-key")
        .build()?;

    // Insert a record
    let mut record = Record::new();
    record.insert("name", "John Doe");
    record.insert("age", 30);
    record.insert("email", "john@example.com");

    let result = client.insert("users", record).await?;
    println!("Inserted: {:?}", result);

    // Query with the builder
    let query = QueryBuilder::new()
        .gte("age", 18)
        .eq("status", "active")
        .limit(10)
        .build();

    let users = client.find("users", query).await?;
    println!("Found {} users", users.len());

    Ok(())
}
```

## Configuration

### Environment Variables

You can configure the client using environment variables:

```bash
export API_BASE_URL="http://localhost:8080"
export API_BASE_KEY="your-api-key"

# Run example
cargo run -p ekodb_client --example simple_crud
```

```rust
use std::env;

let client = Client::builder()
    .base_url(env::var("API_BASE_URL")?)
    .api_key(env::var("API_BASE_KEY")?)
    .timeout(Duration::from_secs(30))
    .max_retries(3)
    .build()?;
```

## Examples

### CRUD Operations

```rust
use ekodb_client::{Client, FieldType, Record};

// Insert
let mut user = Record::new();
user.insert("name", "Alice");
user.insert("age", 25);
let inserted = client.insert("users", user).await?;

// Extract ID from response
let user_id = if let Some(FieldType::String(id)) = inserted.get("id") {
    id.clone()
} else {
    return Err("No ID returned".into());
};

// Find by ID
let user = client.find_by_id("users", &user_id).await?;

// Update
let mut updates = Record::new();
updates.insert("age", 26);
let updated = client.update("users", &user_id, updates).await?;

// Delete
client.delete("users", &user_id).await?;
```

### Query Builder

```rust
use ekodb_client::QueryBuilder;
use serde_json::json;

// Simple queries
let query = QueryBuilder::new()
    .eq("status", "active")
    .gte("age", 18)
    .lt("age", 65)
    .build();

let results = client.find("users", query).await?;

// Complex queries with sorting and pagination
let query = QueryBuilder::new()
    .in_array("status", vec![json!("active"), json!("pending")])
    .regex("email", r".*@example\.com$")
    .sort_desc("created_at")
    .limit(20)
    .skip(0)
    .build();

let results = client.find("users", query).await?;
```

### Batch Operations

```rust
use ekodb_client::BatchBuilder;

// Create records for batch insert
let mut user1 = Record::new();
user1.insert("name", "Alice");
user1.insert("email", "alice@example.com");

let mut user2 = Record::new();
user2.insert("name", "Bob");
user2.insert("email", "bob@example.com");

// Build and execute batch
let batch = BatchBuilder::new()
    .insert("users", user1)
    .insert("users", user2)
    .build();

let results = client.batch_insert("users", &batch.operations).await?;
println!("Batch completed: {} successful, {} failed",
    results.successful.len(), results.failed.len());
```

### WebSocket Operations

```rust
// Connect to WebSocket
let ws_client = client.websocket("ws://localhost:8080/ws").await?;

// Query via WebSocket
let results = ws_client.find_all("users").await?;
println!("Found {} users via WebSocket", results.len());

// Query with filters using ekoDB format
let query = QueryBuilder::new()
    .eq("status", "active")
    .build();
let active_users = ws_client.find("users", query).await?;
```

### TTL (Time-To-Live) Support

```rust
// Insert with 1 hour TTL
let record = Record::new()
    .with("name", "Session Data")
    .with("token", "abc123");

client.insert_with_ttl("sessions", record, "1h").await?;

// Insert with 5 minutes TTL
client.insert_with_ttl("cache", data, "5m").await?;

// Supported TTL formats: "30s", "5m", "1h", "2d"
```

### Key-Value Operations

```rust
// Set a key-value pair
client.kv_set("session:user123", json!({
    "userId": 123,
    "username": "john_doe"
})).await?;

// Get a value
let session = client.kv_get("session:user123").await?;

// Delete a key
client.kv_delete("session:user123").await?;

// Set with TTL
client.kv_set_with_ttl("cache:product:1", product_data, "1h").await?;
```

### Search Operations

```rust
use ekodb_client::SearchQuery;

// Basic text search
let search = SearchQuery::new("programming")
    .min_score(0.1)
    .limit(10);

let results = client.search("articles", search).await?;
for result in results.results {
    println!("Score: {:.4} - {:?}", result.score, result.record.get("title"));
}

// Search with field weights
let search = SearchQuery::new("rust database")
    .fields(vec!["title".to_string(), "description".to_string()])
    .weights(vec![("title".to_string(), 2.0)].into_iter().collect())
    .limit(5);

let results = client.search("articles", search).await?;
```

### AI Chat Integration

```rust
use ekodb_client::{ChatMessageRequest, CollectionConfig, CreateChatSessionRequest};

// Create a chat session
let session_request = CreateChatSessionRequest {
    collections: vec![CollectionConfig {
        collection_name: "documentation".to_string(),
        fields: vec![],
        search_options: None,
    }],
    llm_provider: "openai".to_string(),
    llm_model: Some("gpt-4".to_string()),
    system_prompt: Some("You are a helpful assistant.".to_string()),
    parent_id: None,
    branch_point_idx: None,
    max_context_messages: Some(10),
    bypass_ripple: Some(false),
};

let session = client.create_chat_session(session_request).await?;

// Send a message
let message = ChatMessageRequest::new("What is ekoDB?");
let response = client.chat_message(&session.chat_id, message).await?;

println!("AI Response: {}", response.responses[0]);
println!("Context snippets: {}", response.context_snippets.len());
```

### Chat Models API

```rust
// Get all available chat models by provider
let models = client.get_chat_models().await?;
println!("OpenAI models: {:?}", models.openai);
println!("Anthropic models: {:?}", models.anthropic);
println!("Perplexity models: {:?}", models.perplexity);

// Get models for a specific provider
let openai_models = client.get_chat_model("openai").await?;
for model in openai_models {
    println!("  - {}", model);
}
```

### User Functions API

```rust
use ekodb_client::{Function, ParameterDefinition, UserFunction};

// Create a user function using the builder pattern
let user_func = UserFunction::new("get_active_users", "Get Active Users")
    .with_description("Fetches all users and filters by active status")
    .with_version("1.0.0")
    .with_parameter(ParameterDefinition {
        name: "collection".to_string(),
        required: true,
        description: Some("Collection to query".to_string()),
        default: None,
    })
    .with_function(Function::FindAll {
        collection: "{{collection}}".to_string(),
    })
    .with_tag("users")
    .with_tag("query");

let func_id = client.save_user_function(user_func).await?;
println!("Created user function: {}", func_id);

// Get a user function by label
let func = client.get_user_function("get_active_users").await?;
println!("Retrieved: {} - {}", func.label, func.name);

// List all user functions (optionally filter by tags)
let all_funcs = client.list_user_functions(None).await?;
let tagged = client.list_user_functions(Some(vec!["users".to_string()])).await?;

// Update a user function
let updated_func = UserFunction::new("get_active_users", "Get Active Users")
    .with_description("Updated description")
    .with_version("1.0.1");
client.update_user_function("get_active_users", updated_func).await?;

// Delete a user function
client.delete_user_function("get_active_users").await?;
```

### Schema Management

```rust
use ekodb_client::{FieldTypeSchema, Schema};

// Create a collection with schema
let schema = Schema::new()
    .add_field("title", FieldTypeSchema::new("String").required())
    .add_field("email", FieldTypeSchema::new("String").required())
    .add_field("age", FieldTypeSchema::new("Integer"))
    .add_field("status", FieldTypeSchema::new("String"));

client.create_collection("users", schema).await?;

// Get schema
let schema = client.get_schema("users").await?;
for (name, field) in &schema.fields {
    println!("Field: {} - Type: {}", name, field.field_type);
}
```

### Collection Management

```rust
// List all collections
let collections = client.list_collections().await?;
println!("Collections: {:?}", collections);

// Count documents in a collection
let count = client.count("users").await?;
println!("Total users: {}", count);

// Check if collection exists
let exists = client.collection_exists("users").await?;

// Delete a collection
client.delete_collection("old_data").await?;
```

### Error Handling

```rust
use ekodb_client::Error;

match client.insert("users", record).await {
    Ok(result) => println!("Success: {:?}", result),
    Err(Error::RateLimit { retry_after_secs }) => {
        println!("Rate limited. Retry after {} seconds", retry_after_secs);
    }
    Err(Error::Auth(msg)) => {
        println!("Authentication failed: {}", msg);
    }
    Err(Error::NotFound) => {
        println!("Record not found");
    }
    Err(e) => {
        println!("Error: {}", e);
    }
}
```

## Supported Data Types

ekoDB supports the following data types:

### Primitive Types

- `String` - Text data
- `Integer` - 64-bit signed integers
- `Float` - 64-bit floating point numbers
- `Number` - Flexible numeric type (Integer, Float, or Decimal)
- `Boolean` - True/false values
- `Decimal` - High-precision decimal numbers for financial calculations
- `Null` - Explicit null values

### Date/Time Types

- `DateTime` - ISO-8601 formatted timestamps
- `Duration` - Time durations (e.g., "30s", "5m", "1h")

### Identifier Types

- `UUID` - Universally unique identifiers (auto-validated)

### Collection Types

- `Object` - Nested structures (HashMap)
- `Array` - Ordered collections
- `Set` - Unordered collections (automatically deduplicated by server)
- `Vector` - Vector embeddings for similarity search

### Binary Types

- `Binary` - Binary data (base64 encoded)
- `Bytes` - Raw byte arrays

## Running Examples

The library includes 15 comprehensive examples demonstrating all features:

```bash
# Set environment variables
export API_BASE_URL="http://localhost:8080"
export API_BASE_KEY="your-api-key"

# Run individual examples
cargo run --example client_simple_crud
cargo run --example client_batch_operations
cargo run --example client_kv_operations
cargo run --example client_collection_management
cargo run --example client_document_ttl
cargo run --example client_simple_websocket
cargo run --example client_websocket_ttl
cargo run --example client_query_builder
cargo run --example client_search
cargo run --example client_schema_management
cargo run --example client_chat_basic
cargo run --example client_chat_sessions
cargo run --example client_chat_advanced
cargo run --example client_chat_models
cargo run --example client_user_functions
```

### Available Examples

#### Basic Operations

- **client_simple_crud** - Basic CRUD operations (Create, Read, Update, Delete)
- **client_batch_operations** - Bulk insert, update, and delete operations
- **client_kv_operations** - Key-value store operations with TTL
- **client_collection_management** - Collection listing, counting, and deletion
- **client_document_ttl** - Documents with automatic expiration

#### Real-time & WebSocket

- **client_simple_websocket** - Real-time queries via WebSocket
- **client_websocket_ttl** - WebSocket queries with TTL documents

#### Advanced Queries & Search

- **client_query_builder** - Complex queries with operators, sorting, and
  pagination
- **client_search** - Full-text search, vector search, and hybrid search

#### Schema & Data Modeling

- **client_schema_management** - Define and enforce data schemas

#### AI Chat Integration

- **client_chat_basic** - Simple AI chat with context search
- **client_chat_sessions** - Multi-turn conversations with session management
- **client_chat_advanced** - Advanced chat features with streaming and context
  control
- **client_chat_models** - List available AI models by provider

#### User Functions

- **client_user_functions** - CRUD operations for reusable user functions

All examples are located in `examples/rust/examples/` directory.

## API Reference

### Client Methods

#### Document Operations

- `insert(collection, record)` - Insert a document
- `insert_with_ttl(collection, record, ttl)` - Insert with expiration
- `find_by_id(collection, id)` - Find document by ID
- `find(collection, query)` - Query documents with filters
- `find_all(collection)` - Get all documents in collection
- `update(collection, id, updates)` - Update a document
- `delete(collection, id)` - Delete a document

#### Batch Operations

- `batch_insert(collection, records)` - Bulk insert documents
- `batch_update(collection, updates)` - Bulk update documents
- `batch_delete(collection, ids)` - Bulk delete documents

#### Query Building

- `QueryBuilder::new()` - Create a new query builder
- `.eq(field, value)` - Equal to
- `.ne(field, value)` - Not equal to
- `.gt(field, value)` - Greater than
- `.gte(field, value)` - Greater than or equal
- `.lt(field, value)` - Less than
- `.lte(field, value)` - Less than or equal
- `.in_array(field, values)` - In array
- `.nin(field, values)` - Not in array
- `.regex(field, pattern)` - Regex match
- `.sort_asc(field)` / `.sort_desc(field)` - Sorting
- `.limit(n)` / `.skip(n)` - Pagination
- `.build()` - Build the query

#### Search Operations

- `search(collection, search_query)` - Full-text search
- `SearchQuery::new(query)` - Create search query
- `.fields(fields)` - Specify fields to search
- `.weights(weights)` - Field weights for scoring
- `.min_score(score)` - Minimum relevance score
- `.limit(n)` - Maximum results

#### AI Chat Operations

- `create_chat_session(request)` - Create a new chat session
- `chat_message(chat_id, message)` - Send a chat message
- `get_chat_messages(chat_id, query)` - Get message history
- `list_chat_sessions(query)` - List all chat sessions
- `update_chat_session(chat_id, request)` - Update session configuration
- `delete_chat_session(chat_id)` - Delete a chat session

#### Chat Models

- `get_chat_models()` - Get all available chat models by provider
- `get_chat_model(provider)` - Get models for a specific provider (e.g.,
  "openai", "anthropic")

#### User Functions

- `save_user_function(user_function)` - Create a new user function
- `get_user_function(label)` - Get a user function by label
- `list_user_functions(tags)` - List all user functions (optionally filter by
  tags)
- `update_user_function(label, user_function)` - Update a user function
- `delete_user_function(label)` - Delete a user function

#### Schema Management

- `create_collection(collection, schema)` - Create collection with schema
- `get_schema(collection)` - Get collection schema
- `update_schema(collection, schema)` - Update collection schema
- `Schema::new()` - Create a new schema
- `.add_field(name, field_type)` - Add field to schema
- `FieldTypeSchema::new(type)` - Create field type
- `.required()` - Mark field as required

#### Key-Value Operations

- `kv_set(key, value)` - Set a key-value pair
- `kv_set_with_ttl(key, value, ttl)` - Set with expiration
- `kv_get(key)` - Get value by key
- `kv_delete(key)` - Delete a key

#### Collection Operations

- `list_collections()` - List all collections
- `count(collection)` - Count documents in collection
- `collection_exists(collection)` - Check if collection exists
- `delete_collection(collection)` - Delete entire collection

#### WebSocket Operations

- `websocket(url)` - Connect to WebSocket endpoint
- `ws_client.find(collection, query)` - Query via WebSocket
- `ws_client.find_all(collection)` - Get all via WebSocket

## Best Practices

### Connection Management

```rust
// Create one client instance and reuse it
let client = Client::builder()
    .base_url(&base_url)
    .api_key(&api_key)
    .timeout(Duration::from_secs(30))
    .max_retries(3)
    .build()?;

// The client is thread-safe and can be cloned cheaply
let client_clone = client.clone();
```

### Handling Errors Gracefully

```rust
use ekodb_client::Error;

match client.insert("users", record).await {
    Ok(result) => {
        // Handle success
    }
    Err(Error::RateLimit { retry_after_secs }) => {
        // Wait and retry
        tokio::time::sleep(Duration::from_secs(retry_after_secs)).await;
    }
    Err(Error::Auth(msg)) => {
        // Handle authentication error
    }
    Err(e) => {
        // Handle other errors
    }
}
```

### Performance Tips

1. **Use Batch Operations** - For multiple inserts/updates/deletes, use batch
   operations instead of individual calls
2. **Reuse Client** - Create one client instance and reuse it across your
   application
3. **Set Appropriate Timeouts** - Configure timeouts based on your use case
4. **Use Pagination** - For large result sets, use `.limit()` and `.skip()` to
   paginate
5. **Index Fields** - Use schema management to define indexes on frequently
   queried fields
6. **Cache Tokens** - The client automatically caches authentication tokens

### Collection Cleanup

All client examples follow a cleanup convention to prevent test pollution:

```rust
// At the end of your example/test
println!("\n=== Cleanup ===");
client.delete_collection(collection).await?;
println!("✓ Deleted collection");
```

## Troubleshooting

### Authentication Errors

If you see authentication errors:

- Verify your API key is correct
- Check that the ekoDB server is running
- Ensure the API key is registered with the server

### Connection Timeouts

If requests are timing out:

- Increase the timeout: `.timeout(Duration::from_secs(60))`
- Check network connectivity
- Verify the server URL is correct

### Rate Limiting

The client automatically retries on rate limits (429), but you can also handle
them manually:

```rust
if let Err(Error::RateLimit { retry_after_secs }) = result {
    tokio::time::sleep(Duration::from_secs(retry_after_secs)).await;
    // Retry the operation
}
```

## License

This project is licensed under either of:

- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

at your option.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Resources

- [API Reference](https://docs.rs/ekodb_client)
- [GitHub Repository](https://github.com/ekoDB/ekodb-client)
- [Examples](https://github.com/ekoDB/ekodb-client/tree/main/examples/rust/examples)
- [crates.io Package](https://crates.io/crates/ekodb_client)
