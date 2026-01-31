# ekoDB Python Client

High-performance Python client for ekoDB, built with Rust for speed and safety.

This package wraps the `ekodb_client` Rust library using PyO3 to provide a
native Python interface.

## Features

- ✅ **Fast**: Built with Rust, leveraging the same client library as the Rust
  SDK
- ✅ **Type-safe**: Strong typing with Python type hints
- ✅ **Async/await**: Full async support using Python's asyncio
- ✅ **Easy to use**: Pythonic API that feels natural
- ✅ **Complete**: All ekoDB features supported
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

## Installation

```bash
pip install ekodb
```

Or install from source:

```bash
cd ekodb-py
pip install maturin
maturin develop
```

## Quick Start

```python
import asyncio
from ekodb_client import Client, RateLimitError

async def main():
    # Create client with configuration
    client = Client.new(
        "http://localhost:8080",
        "your-api-key",
        should_retry=True,  # Enable automatic retries (default: True)
        max_retries=3,      # Maximum retry attempts (default: 3)
        timeout_secs=30     # Request timeout in seconds (default: 30)
    )

    try:
        # Insert a document
        record = await client.insert("users", {
            "name": "John Doe",
            "age": 30,
            "email": "john@example.com",
            "active": True
        })
        print(f"Inserted: {record['id']}")

        # Find by ID
        user = await client.find_by_id("users", record["id"])
        print(f"Found: {user}")

        # Find with query
        results = await client.find("users", limit=10)
        print(f"Found {len(results)} users")

        # Update
        updated = await client.update("users", record["id"], {
            "age": 31
        })
        print(f"Updated: {updated}")

        # Delete
        await client.delete("users", record["id"])
        print("Deleted")

    except RateLimitError as e:
        print(f"Rate limited! Retry after {e.retry_after_secs} seconds")

asyncio.run(main())
```

## Usage Examples

### Query Builder

```python
from ekodb_client import Client, QueryBuilder

async def main():
    client = Client.new("http://localhost:8080", "your-api-key")

    # Simple query with operators
    query = QueryBuilder() \
        .eq("status", "active") \
        .gte("age", 18) \
        .lt("age", 65) \
        .limit(10) \
        .build()

    results = await client.find("users", query)

    # Complex query with sorting and pagination
    query = QueryBuilder() \
        .in_array("status", ["active", "pending"]) \
        .contains("email", "@example.com") \
        .sort_desc("created_at") \
        .skip(20) \
        .limit(10) \
        .build()

    results = await client.find("users", query)
```

### Search Operations

```python
# Basic text search
search_query = {
    "query": "programming",
    "min_score": 0.1,
    "limit": 10
}

results = await client.search("articles", search_query)
for result in results["results"]:
    print(f"Score: {result['score']:.4f} - {result['record']['title']}")

# Search with field weights
search_query = {
    "query": "rust database",
    "fields": ["title", "description"],
    "weights": {"title": 2.0},
    "limit": 5
}

results = await client.search("articles", search_query)
```

### Schema Management

```python
# Create a collection with schema
schema = {
    "fields": {
        "name": {
            "field_type": "String",
            "required": True,
            "regex": "^[a-zA-Z ]+$"
        },
        "email": {
            "field_type": "String",
            "required": True,
            "unique": True
        },
        "age": {
            "field_type": "Integer",
            "min": 0,
            "max": 150
        }
    }
}

await client.create_collection("users", schema)

# Get collection schema
schema = await client.get_schema("users")
```

### Join Operations

```python
# Single collection join
query = {
    "join": {
        "collections": ["departments"],
        "local_field": "department_id",
        "foreign_field": "id",
        "as_field": "department"
    },
    "limit": 10
}

results = await client.find("users", query)

# Multi-collection join
query = {
    "join": [
        {
            "collections": ["departments"],
            "local_field": "department_id",
            "foreign_field": "id",
            "as_field": "department"
        },
        {
            "collections": ["profiles"],
            "local_field": "id",
            "foreign_field": "id",
            "as_field": "profile"
        }
    ],
    "limit": 10
}

results = await client.find("users", query)
```

## API Reference

### Client

#### `Client.new(base_url: str, api_key: str, should_retry: bool = True, max_retries: int = 3, timeout_secs: int = 30) -> Client`

Create a new ekoDB client.

**Parameters:**

- `base_url`: The base URL of the ekoDB server
- `api_key`: Your API key
- `should_retry`: Enable automatic retries (default: True)
- `max_retries`: Maximum number of retry attempts (default: 3)
- `timeout_secs`: Request timeout in seconds (default: 30)

**Returns:**

- A new `Client` instance

### RateLimitInfo

Rate limit information is automatically tracked and logged by the client. The
client will automatically retry on rate limit errors using the server's
`Retry-After` header.

#### Properties

- `limit: int` - Maximum requests allowed per window
- `remaining: int` - Requests remaining in current window
- `reset: int` - Unix timestamp when the rate limit resets

#### Methods

- `is_near_limit() -> bool` - Check if approaching rate limit (<10% remaining)
- `is_exceeded() -> bool` - Check if the rate limit has been exceeded
- `remaining_percentage() -> float` - Get the percentage of requests remaining

### RateLimitError

Exception raised when rate limit is exceeded (if retries are disabled or
exhausted).

#### Properties

- `retry_after_secs: int` - Number of seconds to wait before retrying

#### `await client.insert(collection: str, record: dict) -> dict`

Insert a document into a collection.

**Parameters:**

- `collection`: The collection name
- `record`: A dictionary representing the document

**Returns:**

- The inserted document with ID

#### `await client.find_by_id(collection: str, id: str) -> dict`

Find a document by ID.

**Parameters:**

- `collection`: The collection name
- `id`: The document ID

**Returns:**

- The found document

#### `await client.find(collection: str, limit: Optional[int] = None) -> List[dict]`

Find documents in a collection.

**Parameters:**

- `collection`: The collection name
- `limit`: Optional limit on number of results

**Returns:**

- List of matching documents

#### `await client.update(collection: str, id: str, updates: dict) -> dict`

Update a document.

**Parameters:**

- `collection`: The collection name
- `id`: The document ID
- `updates`: Dictionary of fields to update

**Returns:**

- The updated document

#### `await client.delete(collection: str, id: str) -> None`

Delete a document.

**Parameters:**

- `collection`: The collection name
- `id`: The document ID

#### `await client.list_collections() -> List[str]`

List all collections.

**Returns:**

- List of collection names

#### `await client.delete_collection(collection: str) -> None`

Delete a collection.

**Parameters:**

- `collection`: The collection name to delete

#### `await client.search(collection: str, query: dict) -> dict`

Perform full-text search on a collection.

**Parameters:**

- `collection`: The collection name
- `query`: Search query dictionary with fields like `query`, `fields`,
  `weights`, `min_score`, `limit`

**Returns:**

- Search results with scores and matched records

#### `await client.create_collection(collection: str, schema: dict) -> None`

Create a collection with a schema.

**Parameters:**

- `collection`: The collection name
- `schema`: Schema definition dictionary

#### `await client.get_schema(collection: str) -> dict`

Get the schema for a collection.

**Parameters:**

- `collection`: The collection name

**Returns:**

- Schema definition dictionary

#### `await client.get_collection(collection: str) -> dict`

Get collection metadata including schema.

**Parameters:**

- `collection`: The collection name

**Returns:**

- Collection metadata dictionary

#### `await client.collection_exists(collection: str) -> bool`

Check if a collection exists.

**Parameters:**

- `collection`: The collection name

**Returns:**

- True if the collection exists, False otherwise

#### `await client.count_documents(collection: str) -> int`

Count documents in a collection.

**Parameters:**

- `collection`: The collection name

**Returns:**

- Number of documents in the collection

### Chat Models

#### `await client.get_chat_models() -> dict`

Get all available chat models organized by provider.

**Returns:**

- Dictionary mapping provider names to lists of model names

#### `await client.get_chat_model(provider: str) -> list`

Get models for a specific provider.

**Parameters:**

- `provider`: The provider name (e.g., "openai", "anthropic")

**Returns:**

- List of model names for the provider

### User Functions

#### `await client.save_user_function(user_function: dict) -> str`

Create a new user function.

**Parameters:**

- `user_function`: Dictionary containing label, name, parameters, functions,
  etc.

**Returns:**

- The ID of the created user function

#### `await client.get_user_function(label: str) -> dict`

Get a user function by its label.

**Parameters:**

- `label`: The user function label

**Returns:**

- User function definition dictionary

#### `await client.list_user_functions(tags: Optional[list] = None) -> list`

List all user functions, optionally filtered by tags.

**Parameters:**

- `tags`: Optional list of tags to filter by

**Returns:**

- List of user function dictionaries

#### `await client.update_user_function(label: str, user_function: dict) -> None`

Update an existing user function.

**Parameters:**

- `label`: The user function label
- `user_function`: Updated user function definition

#### `await client.delete_user_function(label: str) -> None`

Delete a user function by its label.

**Parameters:**

- `label`: The user function label

## Examples

See the
[examples directory](https://github.com/ekoDB/ekodb-client/tree/main/examples/python)
for complete working examples:

- `client_simple_crud.py` - Basic CRUD operations
- `client_query_builder.py` - Complex queries with QueryBuilder
- `client_search.py` - Full-text search operations
- `client_schema.py` - Schema management
- `client_joins.py` - Join operations
- `client_batch_operations.py` - Batch operations
- `client_kv_operations.py` - Key-value operations
- `client_chat_models.py` - Chat models API
- `client_user_functions.py` - User functions API
- And more...

## Development

### Building

```bash
# Install maturin
pip install maturin

# Build and install in development mode
maturin develop

# Build release wheel
maturin build --release
```

### Testing

```bash
# Run Python tests
pytest

# Run with coverage
pytest --cov=ekodb
```

## License

MIT OR Apache-2.0

## Links

- [GitHub](https://github.com/ekoDB/ekodb-client)
- [Examples](https://github.com/ekoDB/ekodb-client/tree/main/examples/python)
- [PyPI Package](https://pypi.org/project/ekodb-client/)
