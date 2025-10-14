# JavaScript/Node.js Examples for ekoDB

This directory contains JavaScript/Node.js examples demonstrating ekoDB
features.

## Prerequisites

- Node.js 14.x or higher
- npm or yarn

## Installation

```bash
npm install
# or
yarn install
```

## Examples

### TTL Features

- [Document TTL](ttl-caching/document_ttl.js) - Basic document-level TTL
  operations
- [WebSocket TTL](ttl-caching/websocket_ttl.js) - Real-time TTL events via
  WebSocket

### Basic Operations

- [Simple CRUD](basic-crud/simple_crud.js) - Basic create, read, update, delete
  operations
- [Batch Operations](performance/batch.js) - Efficient batch operations
- [Real-time Updates](websocket/realtime.js) - WebSocket-based real-time updates

## Running Examples

1. Start the ekoDB server
2. Set your JWT token in the example file or use environment variables
3. Run an example:

   ```bash
   node ttl-caching/document_ttl.js
   ```

## Important Notes

- Always set `bypassCache: true` in WebSocket subscriptions for real-time data
- Use proper error handling in production code
- Check the [TTL Documentation](../../documentation/TTL.md) for best practices
