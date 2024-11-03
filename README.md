# ekodb-client

API Client Interface Packages to use ekoDB as a service.

## Available Clients

### Go Client

A type-safe Go client with context support for interacting with EkoDB databases.
[Learn more](./clients/go/README.md)

### Python Client

A type-safe Python client with async/await support for interacting with EkoDB databases. 
[Learn more](./clients/python/README.md)

### Node.js Client

A TypeScript-first Node.js client providing a robust interface to EkoDB databases.
[Learn more](./clients/nodejs/README.md)

## Features

All clients provide:

- Type-safe database operations
- Comprehensive CRUD operations
- Batch operations support
- Authentication handling
- Query filtering and sorting
- Schema management
- Error handling

Each client also includes language-specific features:

- Go: Context support, idiomatic error handling
- Python: Async/await support
- Node.js: TypeScript support, Promises

## Documentation

- [Go Client Documentation](./clients/go/README.md)
- [Python Client Documentation](./clients/python/README.md)
- [Node.js Client Documentation](./clients/nodejs/README.md)

## Development

Each client has its own development setup and testing procedures. Please refer to their respective README files for details.

### Building All Clients

```bash
make build
```

### Running All Tests

```bash
make test
```

### Cleaning All Builds

```bash
make clean
```

## License

MIT
