# Rust Examples for ekoDB

This directory contains Rust examples demonstrating ekoDB features using **two
approaches**:

1. **Raw HTTP/WebSocket** - Direct API calls using `reqwest` and
   `tokio-tungstenite`
2. **ekodb_client Library** - Type-safe client library with auto-retry and
   better ergonomics

## Prerequisites

- Rust 1.70 or higher
- Cargo package manager

## Building

```bash
cargo build
```

## Examples

### 🔧 Raw HTTP/WebSocket Examples

These examples show direct API usage without the client library:

- **[simple_crud.rs](examples/simple_crud.rs)** - CRUD operations with raw HTTP

  ```bash
  cargo run --example simple_crud
  ```

- **[batch_operations.rs](examples/batch_operations.rs)** - Batch
  insert/update/delete

  ```bash
  cargo run --example batch_operations
  ```

- **[kv_operations.rs](examples/kv_operations.rs)** - Key-value store operations

  ```bash
  cargo run --example kv_operations
  ```

- **[collection_management.rs](examples/collection_management.rs)** - Collection
  management

  ```bash
  cargo run --example collection_management
  ```

- **[simple_websocket.rs](examples/simple_websocket.rs)** - WebSocket
  subscriptions

  ```bash
  cargo run --example simple_websocket
  ```

- **[document_ttl.rs](examples/document_ttl.rs)** - Document TTL

  ```bash
  cargo run --example document_ttl
  ```

- **[websocket_ttl.rs](examples/websocket_ttl.rs)** - Real-time TTL events
  ```bash
  cargo run --example websocket_ttl
  ```

### 📦 ekodb_client Library Examples

These examples show the same operations using the type-safe client library:

- **[client_simple_crud.rs](examples/client_simple_crud.rs)** - CRUD with
  ekodb_client

  ```bash
  cargo run --example client_simple_crud
  ```

- **[client_batch_operations.rs](examples/client_batch_operations.rs)** - Batch
  operations

  ```bash
  cargo run --example client_batch_operations
  ```

- **[client_kv_operations.rs](examples/client_kv_operations.rs)** - KV
  operations (planned)

  ```bash
  cargo run --example client_kv_operations
  ```

- **[client_collection_management.rs](examples/client_collection_management.rs)** -
  Collections (planned)
  ```bash
  cargo run --example client_collection_management
  ```

## Running Examples

1. Start the ekoDB server
2. Set your JWT token in the example file or use environment variables
3. Run an example:

   ```bash
   cargo run --example document_ttl
   ```

## Important Notes

- Use `tokio` for async operations
- Always set `bypassCache: true` in WebSocket subscriptions for real-time data
- Use proper error handling in production code
- Check the [TTL Documentation](../../documentation/TTL.md) for best practices
