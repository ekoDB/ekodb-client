# @ekodb-client/nodejs

A serverside client interface utility for ekoDB, providing a simple and type-safe way to interact with ekoDB databases.

## Installation

```bash
npm install @ekodb-client/nodejs
```

## Features

- Type-safe database operations with TypeScript support
- Comprehensive CRUD operations (Create, Read, Update, Delete)
- Batch operations support
- Built-in authentication handling
- Flexible query filtering and sorting
- Schema management
- Automatic token refresh
- Robust error handling

## Quick Start

```typescript
import EkoDBClient from '@ekodb-client/nodejs';

// Initialize the client
const client = new EkoDBClient({
  baseURL: 'http://your-ekodb-server.com', // Optional, defaults to http://localhost:8080
  apiKey: 'your-api-key'
});

// Example: Insert a record
interface User {
  name: string;
  age: number;
}

const user = {
  name: { type: 'String', value: 'John Doe' },
  age: { type: 'Integer', value: 30 }
};

const userId = await client.insert<User>('users', user);
```

## API Reference

### Constructor

```typescript
const client = new EkoDBClient({
  baseURL?: string,  // Optional: API base URL
  apiKey: string     // Required: Your API key
});
```

### Basic Operations

#### Insert

```typescript
// Single insert
const id = await client.insert<T>(collection: string, record: TypedRecord<T>): Promise<string>

// Batch insert
const result = await client.batchInsert<T>(collection: string, records: TypedRecord<T>[]): Promise<BatchInsertResponse>
```

#### Find

```typescript
// Find multiple records
const records = await client.find<T>(collection: string, options?: FindOptions): Promise<TypedRecord<T>[]>

// Find by ID
const record = await client.findById<T>(collection: string, id: string): Promise<TypedRecord<T>>
```

#### Update

```typescript
// Single update
const success = await client.update<T>(collection: string, id: string, updates: Partial<TypedRecord<T>>): Promise<boolean>

// Batch update
const result = await client.batchUpdate<T>(collection: string, updates: Record<string, Partial<TypedRecord<T>>>): Promise<BatchUpdateResponse>
```

#### Delete

```typescript
// Single delete
const success = await client.delete(collection: string, id: string): Promise<boolean>

// Batch delete
const result = await client.batchDelete(collection: string, ids: string[]): Promise<BatchDeleteResponse>
```

#### Schema Operations

```typescript
const schema = await client.getSchema(collection: string): Promise<CollectionSchema>
```

### Query Options

The `find` method supports various query options:

```typescript
interface FindOptions {
  filter?: Filter;              // Query filter
  sort?: SortOption[];         // Sort criteria
  limit?: number;              // Maximum number of records to return
  skip?: number;               // Number of records to skip
}
```

#### Filter Examples

```typescript
// Simple equality filter
const filter = {
  type: 'Condition',
  content: {
    field: 'age',
    operator: 'Gt',
    value: { type: 'Integer', value: 25 }
  }
};

// Logical AND filter
const complexFilter = {
  type: 'Logical',
  content: {
    operator: 'And',
    expressions: [
      {
        type: 'Condition',
        content: {
          field: 'age',
          operator: 'Gt',
          value: { type: 'Integer', value: 25 }
        }
      },
      {
        type: 'Condition',
        content: {
          field: 'name',
          operator: 'Contains',
          value: { type: 'String', value: 'John' }
        }
      }
    ]
  }
};
```

## Error Handling

The client includes a custom `EkoDBError` class for error handling:

```typescript
try {
  await client.insert('users', user);
} catch (error) {
  if (error instanceof EkoDBError) {
    console.error(`Operation failed: ${error.message}`);
    console.error(`Status code: ${error.statusCode}`);
    console.error(`Response: ${error.response}`);
  }
}
```

## Development

### Setup

```bash
cd ekodb-client/client/nodejs

# Install dependencies
npm install
```

### Scripts

- `npm run build` - Build the project
- `npm run clean` - Clean the build directory
- `npm test` - Run tests
- `npm run lint` - Run linting
- `npm run format` - Format code using Prettier

### Testing

The project uses Jest for testing. Tests can be run with:

```bash
npm test
```

## License

MIT

## Author

Sean M. Vazquez <sean.m.vazquez@gmail.com>
