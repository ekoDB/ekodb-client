# ekoDB Client Libraries

Official client libraries for ekoDB - a high-performance document database.

[![Examples](https://img.shields.io/badge/Examples-125_Working-success)](examples/)
[![Languages](https://img.shields.io/badge/Languages-6-blue)](examples/)
[![Client Libraries](https://img.shields.io/badge/Client_Libraries-5-purple)](examples/)

**🚀 Quick Stats:**

- **125 working examples** - 89 client library + 36 direct API examples
- **6 programming languages** - Rust, Python, Go, TypeScript, JavaScript, Kotlin
- **5 full-featured clients** - Complete feature parity across Rust, Python, Go,
  TypeScript, Kotlin
- **14 examples per client** - CRUD, batch ops, queries, joins, search,
  WebSocket, AI/chat, and more
- **100% tested** - All examples run in CI/CD

## 📦 Available Clients

### 🦀 Rust Client (`ekodb_client`)

[![Crates.io](https://img.shields.io/crates/v/ekodb_client)](https://crates.io/crates/ekodb_client)
[![docs.rs](https://docs.rs/ekodb_client/badge.svg)](https://docs.rs/ekodb_client)

```bash
cargo add ekodb_client
```

### 🐍 Python Client (`ekodb-client`)

[![PyPI](https://img.shields.io/pypi/v/ekodb-client)](https://pypi.org/project/ekodb-client/)
[![Python Version](https://img.shields.io/pypi/pyversions/ekodb-client)](https://pypi.org/project/ekodb-client/)

```bash
pip install ekodb-client
```

### 🔷 Go Client

[![Go Reference](https://pkg.go.dev/badge/github.com/ekoDB/ekodb-client-go.svg)](https://pkg.go.dev/github.com/ekoDB/ekodb-client-go)
[![Go Report Card](https://goreportcard.com/badge/github.com/ekoDB/ekodb-client-go)](https://goreportcard.com/report/github.com/ekoDB/ekodb-client-go)

```bash
go get github.com/ekoDB/ekodb-client-go
```

**Note:** Go client is maintained in a separate repository:
[github.com/ekoDB/ekodb-client-go](https://github.com/ekoDB/ekodb-client-go)

### 📘 TypeScript/JavaScript Client

[![npm](https://img.shields.io/npm/v/@ekodb/ekodb-client)](https://www.npmjs.com/package/@ekodb/ekodb-client)
[![npm downloads](https://img.shields.io/npm/dm/@ekodb/ekodb-client)](https://www.npmjs.com/package/@ekodb/ekodb-client)

```bash
npm install @ekodb/ekodb-client
```

### 🟣 Kotlin Client (`ekodb-client-kt`)

[![Maven Central](https://img.shields.io/maven-central/v/io.ekodb/ekodb-client-kt)](https://central.sonatype.com/artifact/io.ekodb/ekodb-client-kt)

```kotlin
// Gradle (Kotlin DSL)
implementation("io.ekodb:ekodb-client-kt:0.1.0")
```

**Features:**

- ✅ Full coroutines support
- ✅ Type-safe operations
- ✅ Fluent query builder with joins
- ✅ WebSocket real-time queries
- ✅ AI/Chat integration
- ✅ Schema management
- ✅ Full-text search
- ✅ 14 comprehensive examples

## Features

All clients provide:

- ✅ Type-safe database operations
- ✅ Comprehensive CRUD operations
- ✅ Batch operations (insert, update, delete)
- ✅ Authentication handling
- ✅ Query filtering and sorting with joins
- ✅ Schema management and validation
- ✅ Full-text search capabilities
- ✅ WebSocket real-time queries
- ✅ AI/Chat integration with context retrieval
- ✅ TTL (time-to-live) support
- ✅ Key-value operations
- ✅ Error handling and retries

Each client also includes language-specific features:

- **Rust**: Tokio async runtime, strong type safety, zero-cost abstractions
- **Python**: Async/await support, type hints, modern Python 3.8+
- **Go**: Context support, idiomatic error handling, goroutines
- **TypeScript/JavaScript**: TypeScript support, Promises, Node.js compatibility
- **Kotlin**: Coroutines, null safety, extension functions, fluent APIs

## 📚 Documentation

- [Rust Client Documentation](https://docs.rs/ekodb_client)
- [Python Client Documentation](./ekodb-client-py/README.md)
- [Go Client Documentation](https://github.com/ekoDB/ekodb-client-go) - Separate
  repository
- [TypeScript Client Documentation](./ekodb-client-ts/README.md)
- [Kotlin Client Documentation](./ekodb-client-kt/README.md)
- [Examples](./examples/) - Code examples across all available languages (14
  examples per client)

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

# Build Kotlin client
make build-kotlin-client

# Run all tests
make test

# Run examples for all languages
make test-examples

# Run Kotlin examples specifically
make test-examples-kt
```

### Deployment

```bash
# Deploy Rust client to crates.io
make deploy-client-rust

# Deploy Python client to PyPI
make bump-client-py      # Bump version
make deploy-client-py    # Build & publish (Linux + macOS + source)

# Deploy TypeScript client to npm
make deploy-client-ts

# Deploy Go client (via git tags)
make deploy-client-go

# Deploy Kotlin client to Maven Central
cd ekodb-client-kt && ./gradlew publish
```

## License

MIT
