# @ekodb-client/go

A Go client library for interacting with EkoDB databases, providing a type-safe and idiomatic interface.

## Installation

```bash
go get github.com/ekodb/ekodb-client/go
```

## Features

- Context support for all operations
- Type-safe database operations
- Comprehensive CRUD operations
- Batch operations support
- Built-in authentication handling
- Flexible query filtering and sorting
- Schema management
- Automatic token refresh
- Robust error handling with custom error types

## Quick Start

```go
package main

import (
    "context"
    "log"
    
    "github.com/ekodb/ekodb-client/go"
)

func main() {
    // Initialize client
    client := ekodb.NewClient(&ekodb.ClientConfig{
        BaseURL: "http://your-ekodb-server.com", // Optional, defaults to http://localhost:8080
        APIKey:  "your-api-key",
    })
    
    // Create a record
    record := ekodb.Record{
        "name": ekodb.TypedValue{
            Type:  ekodb.StringType,
            Value: "John Doe",
        },
        "age": ekodb.TypedValue{
            Type:  ekodb.IntegerType,
            Value: 30,
        },
    }
    
    ctx := context.Background()
    id, err := client.Insert(ctx, "users", record)
    if err != nil {
        log.Fatalf("Failed to insert record: %v", err)
    }
    log.Printf("Inserted record with ID: %s", id)
}
```

## API Reference

### Client Configuration

```go
cfg := &ekodb.ClientConfig{
    BaseURL:    "http://your-ekodb-server.com", // Optional
    APIKey:     "your-api-key",                 // Required
    HTTPClient: &http.Client{},                 // Optional, defaults to http.DefaultClient
}
client := ekodb.NewClient(cfg)
```

### Basic Operations

#### Insert

```go
// Single insert
id, err := client.Insert(ctx, collection string, record Record) (string, error)

// Batch insert
result, err := client.BatchInsert(ctx, collection string, records []Record) (*BatchInsertResponse, error)
```

#### Find

```go
// Find multiple records
records, err := client.Find(ctx, collection string, opts *FindOptions) ([]Record, error)

// Find by ID
record, err := client.FindByID(ctx, collection, id string) (*Record, error)
```

#### Update

```go
// Single update
success, err := client.Update(ctx, collection, id string, updates Record) (bool, error)

// Batch update
result, err := client.BatchUpdate(ctx, collection string, updates map[string]Record) (*BatchUpdateResponse, error)
```

#### Delete

```go
// Single delete
success, err := client.Delete(ctx, collection, id string) (bool, error)

// Batch delete
result, err := client.BatchDelete(ctx, collection string, ids []string) (*BatchDeleteResponse, error)
```

#### Schema Operations

```go
schema, err := client.GetSchema(ctx, collection string) (*CollectionSchema, error)
```

### Query Options

The `Find` method supports various query options:

```go
opts := &ekodb.FindOptions{
    Filter: Filter,     // Query filter
    Sort:   []SortOption, // Sort criteria
    Limit:  int,          // Maximum number of records
    Skip:   int,          // Number of records to skip
}
```

#### Filter Examples

```go
// Simple equality filter
filter := &ekodb.ConditionFilter{
    Type: "Condition",
    Content: ekodb.ConditionContent{
        Field:    "age",
        Operator: ekodb.GtOp,
        Value: ekodb.TypedValue{
            Type:  ekodb.IntegerType,
            Value: 25,
        },
    },
}

// Logical AND filter
complexFilter := &ekodb.LogicalFilter{
    Type: "Logical",
    Content: ekodb.LogicalContent{
        Operator: ekodb.AndOp,
        Expressions: []ekodb.Filter{
            &ekodb.ConditionFilter{
                Type: "Condition",
                Content: ekodb.ConditionContent{
                    Field:    "age",
                    Operator: ekodb.GtOp,
                    Value: ekodb.TypedValue{
                        Type:  ekodb.IntegerType,
                        Value: 25,
                    },
                },
            },
            &ekodb.ConditionFilter{
                Type: "Condition",
                Content: ekodb.ConditionContent{
                    Field:    "name",
                    Operator: ekodb.ContainsOp,
                    Value: ekodb.TypedValue{
                        Type:  ekodb.StringType,
                        Value: "John",
                    },
                },
            },
        },
    },
}
```

### Error Handling

The client includes custom error types for better error handling:

```go
if err := client.Insert(ctx, "users", record); err != nil {
    switch {
    case errors.Is(err, ekodb.ErrNotFound):
        log.Printf("Record not found")
    case err, ok := err.(*ekodb.APIError):
        log.Printf("API error: %s (Status: %d)", err.Message, err.StatusCode)
    default:
        log.Printf("Unexpected error: %v", err)
    }
}
```

### Context Support

All operations support context for timeout and cancellation:

```go
ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
defer cancel()

record, err := client.FindByID(ctx, "users", "user-123")
if err != nil {
    log.Printf("Failed to fetch user: %v", err)
}
```

## Development

### Setup

```bash
# Clone the repository
git clone <repository-url>
cd ekodb-go

# Install dependencies
go mod tidy

# Run tests
go test ./...
```

### Testing

The project uses the standard Go testing package. Tests can be run with:

```bash
go test ./...
```

Additional test options:

- `go test -v ./...` - Verbose output
- `go test -race ./...` - Race condition detection
- `go test -cover ./...` - Coverage report

## License

MIT
