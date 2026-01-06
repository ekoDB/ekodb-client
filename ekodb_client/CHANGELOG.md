# Changelog

All notable changes to ekodb_client will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

### Fixed

### Changed

## [0.8.0] - 2026-01-06

### Added

#### Core Features

- **Convenience methods** across all client libraries (Rust, TypeScript,
  JavaScript, Python, Kotlin, Go):
  - `upsert()` - Insert or update with automatic fallback (atomic
    insert-or-update semantics)
  - `findOne()` - Find single record by field value
  - `exists()` - Efficient existence check without fetching full record
  - `paginate()` - Simplified pagination with page/pageSize parameters
  - `textSearch()` - Full-text search helper (where supported)

#### API Improvements

- **Options structs** for cleaner method signatures:
  - `InsertOptions` - ttl, bypass_ripple, transaction_id, bypass_cache
  - `UpdateOptions` - bypass_ripple, transaction_id, bypass_cache
  - `UpsertOptions` - bypass_ripple, transaction_id
  - `DeleteOptions` - bypass_ripple, transaction_id
  - `FindOptions` - bypass_cache, transaction_id
- Builder pattern for all options structs

#### Examples

- **New bypass_ripple examples** demonstrating ripple control in multi-node
  deployments (all languages)
- **New convenience methods examples** showcasing ergonomic API helpers (all
  languages)
- **Enhanced schema examples** with proper error handling for existing schemas

#### Testing & Quality

- Comprehensive unit tests for all convenience methods (227+ new tests)
- Test coverage for options builders and edge cases
- Integration test suite improvements across all languages

### Fixed

- **Schema example error handling** in TypeScript/JavaScript:
  - Previously crashed with exit code 1 when schema already exists
  - Now logs warning and continues execution (matching Go behavior)
  - Prevents test suite failures on repeated runs
- Error handling consistency across all client implementations

### Changed

- Updated examples to use new convenience methods where appropriate
- Improved error messages and documentation
- Enhanced type safety with stricter option types

## [0.7.1] - 2026-01-03

### Fixed

- Kotlin client: JsonElement type handling for JsonObject, JsonArray,
  JsonPrimitive, and JsonNull
- Kotlin client: Record and Query serialization improvements
- Error handling messaging improvements
- Gradle wrapper and CI configuration issues
- Rust toolchain action naming
- Naming conventions standardization
- Kotlin version mismatch resolution

### Changed

- Updated dependencies:
  - tokio-tungstenite: 0.21.0 → 0.28.0
  - log: 0.4.28 → 0.4.29
  - rmp-serde: 1.3.0 → 1.3.1
  - mockito: 1.7.0 → 1.7.1
  - @msgpack/msgpack (TypeScript)
  - serde_json: 1.0.145 → 1.0.148
  - kotlinx-coroutines-core and kotlinx-coroutines-test
  - golang.org/x/net
  - ktor-client-core
- Re-ran test examples with updated configurations

## [0.7.0] - 2026-01-03

### Added

- Dependabot configuration for automated dependency management

### Fixed

- Utility function doc-tests
- Client transaction examples
- Document TTL examples

## [0.6.1] - 2026-01-02

### Added

- Type-specific getValue helpers for all client libraries
- getStringValue, getIntValue, getBoolValue, etc. utility functions

### Changed

- Python client: PyO3 upgrade from 0.20 to 0.27
- Re-ran examples with updated type utilities

### Fixed

- Code scanning workflow permissions

## [0.6.0] - 2025-12-31

### Added

- Functions and Scripts support across all clients
- Function tags and versioning (now optional)
- Chat completions examples
- Show function example

### Changed

- Re-ran all examples with Functions support
- Updated examples to demonstrate new functionality

### Fixed

- Python publish scripts

## [0.5.0] - 2025-12-21

### Added

- Self-improving RAG (Retrieval-Augmented Generation) examples
- Enhanced tag examples and documentation

### Changed

- Updated example execution locations and paths
- Improved example list validation

## [0.4.0] - 2025-12-10

### Added

- **Functions and Scripts** - Complete implementation across all client
  libraries
- Transaction support with examples
- Dynamic examples output
- Restore deleted records functionality
- Multiple function and script examples:
  - Basic function operations
  - Function composition
  - Complete CRUD functions
  - Advanced function patterns

### Changed

- FieldType JSON serialization (now uses fromValue)

## [0.3.0] - 2025-11-05

### Added

- **MessagePack serialization** support for binary data transfer
- **Gzip compression** for reduced bandwidth usage
- **Kotlin client library** with full feature parity
  - Maven publishing support
  - Comprehensive examples
  - Coroutine-based async API

### Changed

- Updated all clients to support MessagePack + Gzip
- Enhanced test examples with compression benchmarks
- Updated dependencies across all packages

### Fixed

- Kotlin publishing scripts

## [0.2.1] - 2025-10-14

### Changed

- Updated README with correct package locations
- Documentation improvements

## [0.2.0] - 2025-10-14

### Fixed

- Updated links to Go repository
- Repository reference corrections

## [0.1.8] - 2025-10-14

### Fixed

- Version bumping script now correctly updates ekodb_client dependency in Python
  client
- Dependency synchronization between Rust and Python packages

## [0.1.7] - 2025-10-14

### Added

- Initial multi-language client library release
- Rust client (ekodb_client)
- TypeScript/JavaScript client (@ekodb/ekodb-client)
- Python client (ekodb-client-py with PyO3 bindings)
- Go client (ekodb-client-go)
- Core CRUD operations: insert, find, update, delete, batch operations
- Query builder with filtering, sorting, pagination
- WebSocket support for real-time updates
- Schema management
- Collection management
- Full-text search capabilities
- Vector search support
- Authentication and token management
- Automatic retry with exponential backoff
- Comprehensive examples for all operations
- Type-safe APIs with strong typing
- Error handling with detailed error types
