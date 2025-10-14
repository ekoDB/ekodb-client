# ekoDB Client Libraries

Official client libraries for ekoDB - a high-performance document database.

## 📦 Available Clients

### 🦀 Rust Client (`ekodb_client`)

[![Crates.io](https://img.shields.io/crates/v/ekodb_client)](https://crates.io/crates/ekodb_client)
[![docs.rs](https://docs.rs/ekodb_client/badge.svg)](https://docs.rs/ekodb_client)

```bash
cargo add ekodb_client
```

### 🐍 Python Client (`ekodb-client`)

[![PyPI](https://img.shields.io/pypi/v/ekodb-client)](https://pypi.org/project/ekodb-client/)

```bash
pip install ekodb-client
```

### 🔷 Go Client

[![Go Reference](https://pkg.go.dev/badge/github.com/ekoDB/ekodb-client/ekodb-client-go.svg)](https://pkg.go.dev/github.com/ekoDB/ekodb-client/ekodb-client-go)

```bash
go get github.com/ekoDB/ekodb-client/ekodb-client-go
```

### 📘 TypeScript/JavaScript Client

[![npm](https://img.shields.io/npm/v/@ekodb/client)](https://www.npmjs.com/package/@ekodb/client)

```bash
npm install @ekodb/client
```

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

## 📚 Documentation

- [Rust Client Documentation](https://docs.rs/ekodb_client)
- [Python Client Documentation](./ekodb-client-py/README.md)
- [Go Client Documentation](./ekodb-client-go/README.md)
- [TypeScript Client Documentation](./ekodb-client-ts/README.md)
- [Examples](./examples/) - Code examples for all languages

## 🛠️ Development

See [COMMANDS.md](./COMMANDS.md) for detailed command reference.

### Quick Start

```bash
# Build Rust client
make build

# Build Python client
make build-python-client

# Build TypeScript client
make build-typescript-client

# Run all tests
make test

# Run examples for all languages
make test-examples
```

### Deployment

```bash
# Deploy Python client to PyPI
make bump-client-py      # Bump version
make deploy-client-py    # Build & publish (Linux + macOS + source)

# Deploy Rust client to crates.io
make deploy-client-rust

# Deploy TypeScript client to npm
make deploy-client-ts

# Deploy Go client (via git tags)
make deploy-client-go
```

## License

MIT
