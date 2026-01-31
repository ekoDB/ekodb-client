# ekoDB TypeScript Client

Official TypeScript/JavaScript client library for ekoDB.

## Installation

```bash
npm install @ekodb/ekodb-client
```

## Usage

### TypeScript

```typescript
import { EkoDBClient, RateLimitError } from "@ekodb/ekodb-client";

async function main() {
  // Create and initialize client with configuration
  const client = new EkoDBClient({
    baseURL: "http://localhost:8080",
    apiKey: "your-api-key",
    shouldRetry: true, // Enable automatic retries (default: true)
    maxRetries: 3, // Maximum retry attempts (default: 3)
    timeout: 30000, // Request timeout in ms (default: 30000)
  });
  await client.init();

  // Or use the simple constructor (backward compatible)
  // const client = new EkoDBClient("http://localhost:8080", "your-api-key");

  try {
    // Insert a document
    const record = {
      name: "John Doe",
      email: "john@example.com",
      age: 30,
    };

    const inserted = await client.insert("users", record);
    console.log("Inserted:", inserted);

    // Check rate limit status
    const rateLimitInfo = client.getRateLimitInfo();
    if (rateLimitInfo) {
      console.log(
        `Rate limit: ${rateLimitInfo.remaining}/${rateLimitInfo.limit} remaining`,
      );
    }

    // Find documents
    const results = await client.find("users", { limit: 10 });
    console.log(`Found ${results.length} documents`);
  } catch (error) {
    if (error instanceof RateLimitError) {
      console.log(`Rate limited! Retry after ${error.retryAfterSecs} seconds`);
      // Handle rate limiting manually if needed
    } else {
      throw error;
    }
  }
}

main();
```

### JavaScript

```javascript
const { EkoDBClient } = require("@ekodb/ekodb-client");

async function main() {
  const client = new EkoDBClient("http://localhost:8080", "your-api-key");
  await client.init();

  const record = {
    name: "John Doe",
    email: "john@example.com",
    age: 30,
  };

  const inserted = await client.insert("users", record);
  console.log("Inserted:", inserted);
}

main();
```

## Usage Examples

### Query Builder

```typescript
import { EkoDBClient, QueryBuilder } from "@ekodb/ekodb-client";

const client = new EkoDBClient("http://localhost:8080", "your-api-key");
await client.init();

// Simple query with operators
const query = new QueryBuilder()
  .eq("status", "active")
  .gte("age", 18)
  .lt("age", 65)
  .limit(10)
  .build();

const results = await client.find("users", query);

// Complex query with sorting and pagination
const complexQuery = new QueryBuilder()
  .in("status", ["active", "pending"])
  .contains("email", "@example.com")
  .sortDesc("created_at")
  .skip(20)
  .limit(10)
  .build();

const users = await client.find("users", complexQuery);
```

### Search Operations

```typescript
import { SearchQuery } from "@ekodb/ekodb-client";

// Basic text search
const searchQuery: SearchQuery = {
  query: "programming",
  minScore: 0.1,
  limit: 10,
};

const results = await client.search("articles", searchQuery);
for (const result of results.results) {
  console.log(`Score: ${result.score.toFixed(4)} - ${result.record.title}`);
}

// Search with field weights
const weightedSearch: SearchQuery = {
  query: "rust database",
  fields: ["title", "description"],
  weights: { title: 2.0 },
  limit: 5,
};

const searchResults = await client.search("articles", weightedSearch);
```

### Schema Management

```typescript
import { SchemaBuilder, FieldTypeSchemaBuilder } from "@ekodb/ekodb-client";

// Create a collection with schema
const schema = new SchemaBuilder()
  .addField(
    "name",
    new FieldTypeSchemaBuilder("String").required().pattern("^[a-zA-Z ]+$"),
  )
  .addField("email", new FieldTypeSchemaBuilder("String").required().unique())
  .addField("age", new FieldTypeSchemaBuilder("Integer").range(0, 150))
  .build();

await client.createCollection("users", schema);

// Get collection schema
const retrievedSchema = await client.getSchema("users");
```

### Join Operations

```typescript
import { JoinBuilder } from "@ekodb/ekodb-client";

// Single collection join
const join = JoinBuilder.single(
  "departments",
  "department_id",
  "id",
  "department",
);

const query = new QueryBuilder().join(join).limit(10).build();

const results = await client.find("users", query);

// Multi-collection join
const multiJoin = JoinBuilder.multi(
  ["departments", "profiles"],
  "department_id",
  "id",
  "related_data",
);

const multiQuery = new QueryBuilder().join(multiJoin).build();

const joinResults = await client.find("users", multiQuery);
```

## Features

- ✅ Full TypeScript support with type definitions
- ✅ CRUD operations
- ✅ Batch operations
- ✅ Key-value operations
- ✅ Collection management
- ✅ WebSocket support
- ✅ TTL support
- ✅ Automatic token management
- ✅ **Query Builder** - Fluent API for complex queries with operators, sorting,
  and pagination
- ✅ **Search** - Full-text search, fuzzy search, and field-specific search with
  scoring
- ✅ **Schema Management** - Define and enforce data schemas with validation
- ✅ **Join Operations** - Single and multi-collection joins with queries
- ✅ **Rate limiting with automatic retry** (429, 503, network errors)
- ✅ **Rate limit tracking** (`X-RateLimit-*` headers)
- ✅ **Configurable retry behavior**
- ✅ **Retry-After header support**

## API Reference

### Client Methods

#### Constructor

- `new EkoDBClient(config: ClientConfig | string, apiKey?: string)` - Create
  client with config object or legacy (baseURL, apiKey) signature

#### Configuration

- `init(): Promise<void>` - Initialize and get auth token
- `getRateLimitInfo(): RateLimitInfo | null` - Get current rate limit
  information
- `isNearRateLimit(): boolean` - Check if approaching rate limit (<10%
  remaining)

#### CRUD Operations

- `insert(collection: string, record: Record, ttl?: string): Promise<Record>`
- `find(collection: string, query?: Query): Promise<Record[]>`
- `findByID(collection: string, id: string): Promise<Record>`
- `update(collection: string, id: string, record: Record): Promise<Record>`
- `delete(collection: string, id: string): Promise<void>`

#### Batch Operations

- `batchInsert(collection: string, records: Record[]): Promise<Record[]>`
- `batchUpdate(collection: string, updates: Array<{id: string, data: Record}>): Promise<Record[]>`
- `batchDelete(collection: string, ids: string[]): Promise<number>`

#### Key-Value Operations

- `kvSet(key: string, value: any): Promise<void>`
- `kvGet(key: string): Promise<any>`
- `kvDelete(key: string): Promise<void>`

#### Query Builder

- `new QueryBuilder()` - Create a new query builder
- `.eq(field, value)` - Equal to
- `.ne(field, value)` - Not equal to
- `.gt(field, value)` - Greater than
- `.gte(field, value)` - Greater than or equal
- `.lt(field, value)` - Less than
- `.lte(field, value)` - Less than or equal
- `.in(field, values)` - In array
- `.nin(field, values)` - Not in array
- `.contains(field, value)` - String contains
- `.startsWith(field, value)` - String starts with
- `.endsWith(field, value)` - String ends with
- `.regex(field, pattern)` - Regex match
- `.sortAsc(field)` / `.sortDesc(field)` - Sorting
- `.limit(n)` / `.skip(n)` - Pagination
- `.join(joinConfig)` - Add join configuration
- `.build()` - Build the query

#### Search Operations

- `search(collection: string, query: SearchQuery): Promise<SearchResults>` -
  Full-text search

#### Schema Management

- `createCollection(collection: string, schema: Schema): Promise<void>` - Create
  collection with schema
- `getSchema(collection: string): Promise<Schema>` - Get collection schema
- `getCollection(collection: string): Promise<CollectionMetadata>` - Get
  collection metadata

#### Join Operations

- `JoinBuilder.single(collection, localField, foreignField, as)` - Single
  collection join
- `JoinBuilder.multi(collections, localField, foreignField, as)` -
  Multi-collection join

#### Collection Management

- `listCollections(): Promise<string[]>`
- `deleteCollection(collection: string): Promise<void>`
- `collectionExists(collection: string): Promise<boolean>` - Check if collection
  exists
- `countDocuments(collection: string): Promise<number>` - Count documents in
  collection

#### Chat Models

- `getChatModels(): Promise<Record<string, string[]>>` - Get all available chat
  models by provider
- `getChatModel(provider: string): Promise<string[]>` - Get models for a
  specific provider

#### User Functions

- `saveUserFunction(userFunction: object): Promise<string>` - Create a new user
  function
- `getUserFunction(label: string): Promise<object>` - Get user function by label
- `listUserFunctions(tags?: string[]): Promise<object[]>` - List all user
  functions (optionally filter by tags)
- `updateUserFunction(label: string, userFunction: object): Promise<void>` -
  Update existing user function
- `deleteUserFunction(label: string): Promise<void>` - Delete user function

#### WebSocket

- `websocket(wsURL: string): WebSocketClient`

### WebSocket Methods

- `findAll(collection: string): Promise<Record[]>`
- `close(): void`

## Examples

See the
[examples directory](https://github.com/ekoDB/ekodb-client/tree/main/examples/typescript)
for complete working examples:

- `client_simple_crud.ts` - Basic CRUD operations
- `client_query_builder.ts` - Complex queries with QueryBuilder
- `client_search.ts` - Full-text search operations
- `client_schema.ts` - Schema management
- `client_joins.ts` - Join operations
- `client_batch_operations.ts` - Batch operations
- `client_kv_operations.ts` - Key-value operations
- `client_chat_models.ts` - Chat models API
- `client_user_functions.ts` - User functions API
- And more...

## License

MIT

## Links

- [GitHub Repository](https://github.com/ekoDB/ekodb-client)
- [Examples](https://github.com/ekoDB/ekodb-client/tree/main/examples/typescript)
- [npm Package](https://www.npmjs.com/package/@ekodb/ekodb-client)
