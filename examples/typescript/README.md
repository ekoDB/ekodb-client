# TypeScript Examples for ekoDB

This directory contains TypeScript examples demonstrating ekoDB features with
full type safety.

## Prerequisites

- Node.js 18.x or higher
- npm or yarn
- TypeScript 5.x

## Installation

```bash
npm install
# or
yarn install
```

## Examples

### Basic Operations

- **[client_simple_crud.ts](client_simple_crud.ts)** - Basic CRUD operations

  ```bash
  npx tsx client_simple_crud.ts
  ```

- **[client_batch_operations.ts](client_batch_operations.ts)** - Efficient batch
  insert/update/delete

  ```bash
  npx tsx client_batch_operations.ts
  ```

- **[client_kv_operations.ts](client_kv_operations.ts)** - Key-value store
  operations

  ```bash
  npx tsx client_kv_operations.ts
  ```

- **[client_collection_management.ts](client_collection_management.ts)** -
  Collection creation and management
  ```bash
  npx tsx client_collection_management.ts
  ```

### Advanced Query Features

- **[client_query_builder.ts](client_query_builder.ts)** - Advanced query
  building and filtering

  ```bash
  npx tsx client_query_builder.ts
  ```

- **[client_search.ts](client_search.ts)** - Full-text search capabilities

  ```bash
  npx tsx client_search.ts
  ```

- **[client_joins.ts](client_joins.ts)** - Relational joins between collections

  ```bash
  npx tsx client_joins.ts
  ```

- **[client_schema.ts](client_schema.ts)** - Schema definition and validation
  ```bash
  npx tsx client_schema.ts
  ```

### Chat Features

- **[client_chat_basic.ts](client_chat_basic.ts)** - Basic chat operations

  ```bash
  npx tsx client_chat_basic.ts
  ```

- **[client_chat_advanced.ts](client_chat_advanced.ts)** - Advanced chat
  features with streaming

  ```bash
  npx tsx client_chat_advanced.ts
  ```

- **[client_chat_sessions.ts](client_chat_sessions.ts)** - Managing chat
  sessions and history
  ```bash
  npx tsx client_chat_sessions.ts
  ```

### Real-time & TTL

- **[client_simple_websocket.ts](client_simple_websocket.ts)** - WebSocket
  subscriptions for real-time updates

  ```bash
  npx tsx client_simple_websocket.ts
  ```

- **[client_document_ttl.ts](client_document_ttl.ts)** - Document-level TTL
  operations

  ```bash
  npx tsx client_document_ttl.ts
  ```

- **[client_websocket_ttl.ts](client_websocket_ttl.ts)** - Real-time TTL events
  via WebSocket
  ```bash
  npx tsx client_websocket_ttl.ts
  ```

## Running Examples

1. Start the ekoDB server
2. Set your environment variables in `.env` file
3. Run an example using tsx:

   ```bash
   npx tsx <example_name>.ts
   ```

   Or compile and run:

   ```bash
   npm run build
   node dist/<example_name>.js
   ```

## Important Notes

- TypeScript provides full type safety and better IDE support
- Always set `bypassCache: true` in WebSocket subscriptions for real-time data
- Use proper error handling in production code
- Check the [TTL Documentation](../../documentation/TTL.md) for best practices
