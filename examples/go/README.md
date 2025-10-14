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

- [Document TTL](ttl-caching/document_ttl.go) - Basic document-level TTL
  operations
- [WebSocket TTL](ttl-caching/websocket_ttl.go) - Real-time TTL events via
  WebSocket

### Basic Operations

- [Simple CRUD](basic-crud/simple_crud.go) - Basic create, read, update, delete
  operations
- [Batch Operations](performance/batch.go) - Efficient batch operations
- [Real-time Updates](websocket/realtime.go) - WebSocket-based real-time updates

## Running Examples

1. Start the ekoDB server
2. Set your JWT token in the example file or use environment variables
3. Run an example:

   ```bash
   go run ttl-caching/document_ttl.go
   ```

## Important Notes

- Use goroutines for concurrent operations
- Always set `bypassCache: true` in WebSocket subscriptions for real-time data
- Use proper error handling in production code
- Check the [TTL Documentation](../../documentation/TTL.md) for best practices
