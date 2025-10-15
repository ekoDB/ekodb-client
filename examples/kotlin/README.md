# Kotlin Examples

This directory contains examples for the ekoDB Kotlin client.

## Prerequisites

- JDK 17 or higher
- Gradle 8.5 or higher (or use the wrapper in the parent directory)
- ekoDB server running (default: http://localhost:8080)
- API key set in environment variable `API_BASE_KEY`

## Running Examples

```bash
# Set environment variables
export API_BASE_URL="http://localhost:8080"
export API_BASE_KEY="your-api-key"

# Source the .env file
source .env

# Run from the ekodb-client-kt directory
cd ../../ekodb-client-kt
./gradlew run
```

## Available Examples

### Core Operations

- **ClientSimpleCrud.kt** - Basic CRUD operations (Create, Read, Update, Delete)
- **ClientBatchOperations.kt** - Batch insert, update, and delete operations
- **ClientQueryBuilder.kt** - Complex queries with filters, sorting, and
  pagination
- **ClientDocumentTtl.kt** - Working with time-to-live for automatic expiration

### Collection Management

- **ClientCollectionManagement.kt** - Create, list, and manage collections with
  schemas
- **ClientSchemaManagement.kt** - Advanced schema management operations

### Key-Value Store

- **ClientKvOperations.kt** - Key-value store operations with TTL support

### Search

- **ClientSearch.kt** - Full-text search operations

### WebSocket

- **ClientSimpleWebsocket.kt** - Real-time WebSocket queries
- **ClientWebsocketTtl.kt** - WebSocket operations with TTL documents

### Chat/AI

- **ClientChatBasic.kt** - Basic AI chat with context search
- **ClientChatSessions.kt** - Session management and history
- **ClientChatAdvanced.kt** - Message regeneration, editing, and advanced
  features

## Example Code Structure

Each example demonstrates:

- Client initialization with builder pattern
- Specific feature usage
- Error handling
- Resource cleanup
- Best practices

## Features Demonstrated

- ✅ CRUD operations (insert, find, update, delete)
- ✅ Batch operations (batch insert, update, delete)
- ✅ Query building with filters and sorting
- ✅ Collection management and schemas
- ✅ Key-value store with TTL
- ✅ Document TTL for automatic expiration
- ✅ Authentication and token management
