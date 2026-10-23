# JavaScript/Node.js Examples for ekoDB

This directory contains JavaScript/Node.js examples demonstrating ekoDB
features.

## AI example costs

The matching chat, AI-function, RAG and raw-completion examples select Luna
models and bound completions, preserving the smaller existing raw-completion
limits. The WebSocket client-tool scenario uses GPT-5.6 Luna for compatibility.
See the shared [AI example cost policy](../README.md#ai-example-costs) before
executing live examples; compilation does not incur provider-generation charges.

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

- [Document TTL](document_ttl.js) - TTL expiration verification test
- [WebSocket TTL](websocket_ttl.js) - Real-time TTL events via WebSocket

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
   node document_ttl.js
   ```

## Important Notes

- Always set `bypassCache: true` in WebSocket subscriptions for real-time data
- Use proper error handling in production code
- Check the [TTL Documentation](../../documentation/TTL.md) for best practices
