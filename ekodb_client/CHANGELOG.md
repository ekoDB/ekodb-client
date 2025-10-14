# Changelog

All notable changes to ekodb_client will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Initial client library implementation
- Core CRUD operations (insert, find, update, delete)
- Automatic retry with exponential backoff
- Query builder for complex queries
- Type-safe API with strong Rust types
- Support for all ekoDB data types
- Error handling with detailed error types
- Connection pooling via reqwest
- Authentication management
- Examples and documentation

### Coming Soon

- WebSocket support for real-time subscriptions
- Batch operations for bulk inserts/updates/deletes
- Connection pooling improvements
- JWT token refresh
- Metrics and observability

## [0.1.0] - TBD

### Added

- Initial release of ekodb_client
- Basic CRUD operations
- Query support
- Retry logic
- Type system
