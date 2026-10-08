# Go Examples for ekoDB

This directory contains Go examples demonstrating ekoDB features.

## Prerequisites

- Go 1.19 or higher

## Installation

```bash
go mod download
```

## Examples

### TTL Features

- [Document TTL](document_ttl.go) - TTL expiration verification test
- [WebSocket TTL](websocket_ttl.go) - Real-time TTL events via WebSocket

### Basic Operations

- [Simple CRUD](basic-crud/simple_crud.go) - Basic create, read, update, delete
  operations
- [Batch Operations](performance/batch.go) - Efficient batch operations
- [Real-time Updates](websocket/realtime.go) - WebSocket-based real-time updates

### Paired Vector Search

[client_search.go](client_search.go) creates two indexed `Vector` fields per
record: `query_embedding` for a need and `document_embedding` for a
complementary offer. The same source vector targets `document_embedding` to find
offers and `query_embedding` to find similar needs. Both fields need compatible
dimensions and a shared dual-encoder model space. Its orthogonal toy vectors
only demonstrate field selection; the live example asserts different top records
for the two searches.

```bash
go run client_search.go
```

### AI & RAG Examples

- **[rag_conversation_system.go](rag_conversation_system.go)** - RAG System

  ```bash
  go run rag_conversation_system.go
  ```

  Complete RAG (Retrieval-Augmented Generation) example showing:
  - Message storage with automatic embeddings
  - Cross-conversation hybrid search (semantic + keyword)
  - Context-aware AI responses using search results
  - Dynamic search configurations per conversation
  - Text search with fuzzy matching
  - Self-improving knowledge base

## Running Examples

1. Start the ekoDB server
2. Set your JWT token in the example file or use environment variables
3. Run an example:

   ```bash
   go run document_ttl.go
   ```

## Important Notes

- Use goroutines for concurrent operations
- Always set `bypassCache: true` in WebSocket subscriptions for real-time data
- Use proper error handling in production code
- Check the [TTL Documentation](../../documentation/TTL.md) for best practices
