# @ekodb-client/python

A Python client library for interacting with EkoDB databases, providing a type-safe and intuitive interface.

## Installation

```bash
pip install ekodb-client
```

## Features

- Async/await support for all operations
- Type-safe database operations with Python type hints
- Comprehensive CRUD operations (Create, Read, Update, Delete)
- Batch operations support
- Built-in authentication handling
- Flexible query filtering and sorting
- Schema management
- Automatic token refresh
- Robust error handling with custom exceptions

## Quick Start

```python
import asyncio
from ekodb import EkoDBClient, TypedValue, ValueType

# Initialize the client
client = EkoDBClient({
    "base_url": "http://your-ekodb-server.com",  # Optional, defaults to http://localhost:8080
    "api_key": "your-api-key"
})

# Example: Insert a record
async def create_user():
    user = {
        "name": TypedValue(type=ValueType.STRING, value="John Doe"),
        "age": TypedValue(type=ValueType.INTEGER, value=30)
    }
    
    user_id = await client.insert("users", user)
    return user_id

# Run the async function
user_id = asyncio.run(create_user())
```

## API Reference

### Constructor

```python
client = EkoDBClient({
    "base_url": str,  # Optional: API base URL
    "api_key": str    # Required: Your API key
})
```

### Basic Operations

#### Insert

```python
# Single insert
id = await client.insert(collection: str, record: Dict[str, TypedValue]) -> str

# Batch insert
result = await client.batch_insert(collection: str, records: List[Dict[str, TypedValue]]) -> BatchInsertResponse
```

#### Find

```python
# Find multiple records
records = await client.find(
    collection: str, 
    options: Optional[FindOptions] = None
) -> List[Dict[str, TypedValue]]

# Find by ID
record = await client.find_by_id(
    collection: str, 
    id: str
) -> Optional[Dict[str, TypedValue]]
```

#### Update

```python
# Single update
success = await client.update(
    collection: str,
    id: str,
    updates: Dict[str, TypedValue]
) -> bool

# Batch update
result = await client.batch_update(
    collection: str,
    updates: Dict[str, Dict[str, TypedValue]]
) -> BatchUpdateResponse
```

#### Delete

```python
# Single delete
success = await client.delete(collection: str, id: str) -> bool

# Batch delete
result = await client.batch_delete(
    collection: str,
    ids: List[str]
) -> BatchDeleteResponse
```

#### Schema Operations

```python
schema = await client.get_schema(collection: str) -> CollectionSchema
```

### Query Options

The `find` method supports various query options:

```python
class FindOptions:
    filter: Optional[Filter] = None    # Query filter
    sort: Optional[List[SortOption]]   # Sort criteria
    limit: Optional[int] = None        # Maximum number of records
    skip: Optional[int] = None         # Number of records to skip
```

#### Filter Examples

```python
# Simple equality filter
filter = ConditionFilter(
    content={
        "field": "age",
        "operator": FilterOperator.GT,
        "value": TypedValue(type=ValueType.INTEGER, value=25)
    }
)

# Logical AND filter
complex_filter = LogicalFilter(
    content={
        "operator": LogicalOperator.AND,
        "expressions": [
            ConditionFilter(
                content={
                    "field": "age",
                    "operator": FilterOperator.GT,
                    "value": TypedValue(type=ValueType.INTEGER, value=25)
                }
            ),
            ConditionFilter(
                content={
                    "field": "name",
                    "operator": FilterOperator.CONTAINS,
                    "value": TypedValue(type=ValueType.STRING, value="John")
                }
            )
        ]
    }
)
```

## Error Handling

The client includes a custom `EkoDBError` class for error handling:

```python
try:
    await client.insert("users", user)
except EkoDBError as error:
    print(f"Operation failed: {error.message}")
    print(f"Status code: {error.status_code}")
    print(f"Response: {error.response}")
```

## Development

### Setup

```bash
# Clone the repository
git clone <repository-url>
cd ekodb-client

# Create and activate virtual environment
python -m venv venv
source venv/bin/activate  # Linux/Mac
.\venv\Scripts\activate   # Windows

# Install dependencies
pip install -e ".[dev]"
```

### Testing

The project uses pytest for testing. Tests can be run with:

```bash
pytest
```

Additional test options:

- `pytest -v` - Verbose output
- `pytest --cov=ekodb` - Coverage report
- `pytest -k "test_name"` - Run specific test

## License

MIT
