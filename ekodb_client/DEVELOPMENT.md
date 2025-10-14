# ekoDB Client Development Guide

## Overview

This is the official Rust client library for ekoDB. It provides a type-safe,
async interface for interacting with ekoDB instances.

## Project Structure

```
ekodb_client/
├── src/
│   ├── lib.rs          # Public API exports
│   ├── client.rs       # Main client and builder
│   ├── types.rs        # Public API types (FieldType, Record, Query)
│   ├── error.rs        # Error types
│   ├── auth.rs         # Authentication management
│   ├── http.rs         # HTTP client implementation
│   ├── retry.rs        # Retry logic with exponential backoff
│   ├── query.rs        # Query builder
│   ├── batch.rs        # Batch operations (stub)
│   └── websocket.rs    # WebSocket client (stub)
├── examples/
│   └── simple_crud.rs  # Basic CRUD example
├── tests/
│   └── integration_test.rs
├── Cargo.toml
└── README.md
```

## Development

### Building

```bash
# Build the client
cargo build -p ekodb_client

# Build with all features
cargo build -p ekodb_client --all-features

# Build in release mode
cargo build -p ekodb_client --release
```

### Testing

```bash
# Run unit tests
cargo test -p ekodb_client

# Run integration tests (requires running server)
cargo test -p ekodb_client -- --ignored

# Run specific test
cargo test -p ekodb_client test_retry_success

# Run tests with output
cargo test -p ekodb_client -- --nocapture
```

### Running Examples

```bash
# Set environment variables
export API_BASE_URL="http://localhost:8080"
export API_BASE_KEY="your-api-token"

# Run example
cargo run -p ekodb_client --example simple_crud
```

### Code Quality

```bash
# Format code
cargo fmt -p ekodb_client

# Run clippy
cargo clippy -p ekodb_client

# Check for unused dependencies
cargo udeps -p ekodb_client
```

## Adding New Features

### 1. Adding a New API Endpoint

1. Add method to `HttpClient` in `src/http.rs`
2. Add public method to `Client` in `src/client.rs`
3. Add tests in `tests/integration_test.rs`
4. Add example usage to README

Example:

```rust
// In src/http.rs
pub async fn new_operation(&self, param: &str, token: &str) -> Result<Response> {
    let url = self.base_url.join(&format!("/api/new_operation/{}", param))?;

    self.retry_policy
        .execute(|| async {
            let response = self
                .client
                .post(url.clone())
                .header("Authorization", format!("Bearer {}", token))
                .send()
                .await?;

            self.handle_response(response).await
        })
        .await
}

// In src/client.rs
pub async fn new_operation(&self, param: &str) -> Result<Response> {
    let token = self.auth.get_token().await?;
    self.http.new_operation(param, &token).await
}
```

### 2. Adding a New Type

1. Add type to `src/types.rs`
2. Implement `From` traits if needed
3. Add tests
4. Update documentation

### 3. Adding a New Error Type

1. Add variant to `Error` enum in `src/error.rs`
2. Implement error handling in `src/http.rs`
3. Add tests

## Testing Against Live Server

```bash
# Start ekoDB server
cd ekodb_server
cargo run

# In another terminal, run integration tests
export API_BASE_URL="http://localhost:8080"
export API_BASE_KEY="your-token"
cargo test -p ekodb_client -- --ignored
```

## Performance Testing

```bash
# Run benchmarks (when added)
cargo bench -p ekodb_client

# Profile with flamegraph
cargo flamegraph --example simple_crud
```

## Documentation

### Generating Docs

```bash
# Generate and open documentation
cargo doc -p ekodb_client --open

# Generate with private items
cargo doc -p ekodb_client --document-private-items
```

### Documentation Guidelines

- All public items must have doc comments
- Include examples in doc comments
- Use `# Example` sections
- Mark examples as `no_run` if they require a server

## Release Checklist

- [ ] Update version in `Cargo.toml`
- [ ] Update `CHANGELOG.md`
- [ ] Run all tests
- [ ] Run clippy with no warnings
- [ ] Update README if needed
- [ ] Generate and review docs
- [ ] Create git tag
- [ ] Publish to crates.io (when ready)

## Future Enhancements

### High Priority

- [ ] WebSocket support for real-time subscriptions
- [ ] Batch operations (insert_many, update_many)
- [ ] JWT token refresh logic
- [ ] Connection pooling improvements

### Medium Priority

- [ ] Metrics and observability
- [ ] Request/response logging
- [ ] Custom retry policies
- [ ] Circuit breaker pattern

### Low Priority

- [ ] Caching layer
- [ ] Compression support
- [ ] Custom serialization formats
- [ ] Mock server for testing

## Common Issues

### Issue: Tests fail with connection errors

**Solution**: Make sure ekoDB server is running on the expected URL

### Issue: Authentication errors

**Solution**: Verify API_BASE_KEY environment variable is set correctly

### Issue: Timeout errors

**Solution**: Increase timeout in client builder or check server performance

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Run `cargo fmt` and `cargo clippy`
6. Submit a pull request

## Support

For issues and questions:

- GitHub Issues: (to be added)
- Documentation: https://docs.rs/ekodb_client
- Examples: See `examples/` directory
