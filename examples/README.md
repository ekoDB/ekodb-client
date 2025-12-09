# ekoDB Examples

Back to [README](../README.md)

This directory contains **84 working examples** demonstrating ekoDB's features
across 6 programming languages. Examples are organized into two categories:

- **Client Library Examples** (56 examples) - Using official client libraries
  (recommended)
- **Direct API Examples** (28 examples) - Raw HTTP/WebSocket calls for learning
  the API

Each example is self-contained, fully tested, and serves as both documentation
and integration tests.

---

> **🎉 ekoDB v0.24.0+ Compatibility**
>
> All examples are compatible with ekoDB v0.24.0+. Transaction examples have
> been significantly improved with enhanced reliability:
>
> - ✅ Commit operations correctly persist all changes
> - ✅ Rollback operations properly restore previous state
> - ✅ Works across all isolation levels and durability modes
>
> No code changes required for existing examples - the external API remains
> unchanged. See [Transaction Examples](#8-transactions) for details.

---

## Table of Contents

- [Quick Start](#-quick-start)
- [Example Types](#-example-types)
- [Example Catalog](#-example-catalog)
- [Running Examples](#-running-examples)
  - [Individual Examples](#individual-examples)
  - [Test Runners](#test-runners)
  - [Makefile Commands](#makefile-commands)
- [Integration Testing](#-integration-testing)
- [Authentication](#-authentication)
- [Environment Configuration](#-environment-configuration)
- [Collection Cleanup](#-collection-cleanup)
- [CI/CD Integration](#-cicd-integration)
- [Troubleshooting](#-troubleshooting)
- [Adding New Examples](#-adding-new-examples)

## 🚀 Quick Start

### Prerequisites

1. **Start ekoDB server** (in another terminal):

   ```bash
   cargo run --release
   ```

   Server runs at `http://localhost:8080`

2. **Environment setup** (optional - defaults work for local testing):

   ```bash
   cp .env.example .env
   ```

### Run All Examples

```bash
# From repository root
make test-examples          # All examples (direct + client)
```

This will:

- Check server availability
- Install dependencies for each language
- Run all 84 examples (28 direct + 56 client)
- Provide pass/fail summary

### Run via Makefile (Recommended)

```bash
# From repository root

# Run everything
make test-examples                    # All examples (direct + client)

# Run by type
make test-examples-direct             # Direct API examples only
make test-examples-client             # Client library examples only

# Run by language (both direct + client)
make test-examples-rust               # or: make test-examples-rs
make test-examples-python             # or: make test-examples-py
make test-examples-go
make test-examples-typescript         # or: make test-examples-ts
make test-examples-kotlin             # or: make test-examples-kt
make test-examples-javascript         # or: make test-examples-js

# Run specific language + type
make test-examples-rust-direct        # Rust direct API examples
make test-examples-rust-client        # Rust client library examples
make test-examples-python-direct      # Python direct API examples
make test-examples-python-client      # Python client library examples
# ... and so on for other languages
```

## 📋 Example Types

### Direct API Examples (28 examples)

Located in `{language}/ttl-caching/` directories. These examples use raw
HTTP/WebSocket calls to demonstrate the API directly.

**Purpose:**

- Learn the ekoDB HTTP/WebSocket API
- Understand authentication flow
- See request/response formats
- Reference for custom client implementations

**Languages:** JavaScript, Python, Go, Rust (7 examples each)

### Client Library Examples (56 examples)

Located in `{language}/client_*.{ext}` files or
`{language}/examples/Client*.{ext}`. These examples use official client
libraries for production use.

**Purpose:**

- Production-ready code patterns
- Best practices for client usage
- Advanced features (chat, search, schema management)
- Recommended for application development

**Languages:** Rust (14), Python (14), Go (14), TypeScript (14), JavaScript (7),
Kotlin (14)

**Key Difference:** Client examples automatically clean up test collections,
while direct examples use predictable collection names for learning purposes.

## 📚 Example Catalog

### Direct API Examples (Raw HTTP/WebSocket)

All direct examples are in the `ttl-caching/` subdirectories:

- **JavaScript** - 7 examples in `javascript/ttl-caching/`
- **Python** - 7 examples in `python/ttl-caching/`
- **Go** - 7 examples in `go/ttl-caching/`
- **Rust** - 7 examples in `rust/examples/`

### Client Library Examples

- **Rust** - 14 examples (`client_*.rs` in `rust/examples/`)
- **Python** - 14 examples (`client_*.py` in `python/`)
- **Go** - 14 examples (`client_*.go` in `go/`)
- **TypeScript** - 14 examples (`client_*.ts` in `typescript/`)
- **JavaScript** - 7 examples (`client_*.js` in `javascript/`)
- **Kotlin** - 14 examples (`Client*.kt` in `kotlin/examples/`)

### Feature Categories (Direct API Examples)

### 1. Basic CRUD Operations

- **JavaScript**: [simple_crud.js](./javascript/ttl-caching/simple_crud.js)
- **Python**: [simple_crud.py](./python/ttl-caching/simple_crud.py)
- **Go**: [simple_crud.go](./go/ttl-caching/simple_crud.go)
- **Rust**: [simple_crud.rs](./rust/examples/simple_crud.rs)

### 2. WebSocket Real-time Queries

- **JavaScript**:
  [simple_websocket.js](./javascript/ttl-caching/simple_websocket.js)
- **Python**: [simple_websocket.py](./python/ttl-caching/simple_websocket.py)
- **Go**: [simple_websocket.go](./go/ttl-caching/simple_websocket.go)
- **Rust**: [simple_websocket.rs](./rust/examples/simple_websocket.rs)

### 3. Batch Operations

- **JavaScript**:
  [batch_operations.js](./javascript/ttl-caching/batch_operations.js)
- **Python**: [batch_operations.py](./python/ttl-caching/batch_operations.py)
- **Go**: [batch_operations.go](./go/ttl-caching/batch_operations.go)
- **Rust**: [batch_operations.rs](./rust/examples/batch_operations.rs)

### 4. Key-Value Store

- **JavaScript**: [kv_operations.js](./javascript/ttl-caching/kv_operations.js)
- **Python**: [kv_operations.py](./python/ttl-caching/kv_operations.py)
- **Go**: [kv_operations.go](./go/ttl-caching/kv_operations.go)
- **Rust**: [kv_operations.rs](./rust/examples/kv_operations.rs)

### 5. Collection Management

- **JavaScript**:
  [collection_management.js](./javascript/ttl-caching/collection_management.js)
- **Python**:
  [collection_management.py](./python/ttl-caching/collection_management.py)
- **Go**: [collection_management.go](./go/ttl-caching/collection_management.go)
- **Rust**: [collection_management.rs](./rust/examples/collection_management.rs)

### 6. Document TTL (Time-To-Live)

- **JavaScript**: [document_ttl.js](./javascript/ttl-caching/document_ttl.js)
- **Python**: [document_ttl.py](./python/ttl-caching/document_ttl.py)
- **Go**: [document_ttl.go](./go/ttl-caching/document_ttl.go)
- **Rust**: [document_ttl.rs](./rust/examples/document_ttl.rs)

### 7. WebSocket with TTL

- **JavaScript**: [websocket_ttl.js](./javascript/ttl-caching/websocket_ttl.js)
- **Python**: [websocket_ttl.py](./python/ttl-caching/websocket_ttl.py)
- **Go**: [websocket_ttl.go](./go/ttl-caching/websocket_ttl.go)
- **Rust**: [websocket_ttl.rs](./rust/examples/websocket_ttl.rs)

### 8. Scripts/Functions (Direct HTTP)

> **Saved scripts for reusable queries and multi-stage pipelines**

- **JavaScript**: [http_functions.js](./javascript/http_functions.js)
- **Python**: [http_functions.py](./python/http_functions.py)
- **Go**: [http_functions.go](./go/http_functions.go)
- **Rust**: [http_functions.rs](./rust/examples/http_functions.rs)

**Features demonstrated:**

- Save/call/update/delete scripts via direct HTTP API
- Parameterized scripts with type validation
- Multi-stage pipelines (query + aggregation)
- Script management (list, retrieve by ID/label)
- Group aggregations (count, average, etc.)

### 9. Transactions

> **✅ Enhanced in v0.24.0:** Transaction reliability significantly improved.
> Commit and rollback operations now work correctly across all isolation levels
> and durability modes.

- **JavaScript**: [transactions.js](./javascript/transactions.js)
- **Python**: [transactions.py](./python/transactions.py)
- **Go**: [transactions.go](./go/transactions.go)
- **Rust**: [transactions.rs](./rust/examples/transactions.rs)
- **Kotlin**: [transactions.kt](./kotlin/transactions.kt)

**Features demonstrated:**

- Begin/commit/rollback transactions
- Operations within transactions (`transaction_id` parameter)
- Savepoints for partial rollback
- Multiple isolation levels (ReadCommitted, RepeatableRead, Serializable)
- Testing with different durability configurations

## 🏃 Running Examples

### Individual Examples

Each example can be run standalone:

#### JavaScript

```bash
cd javascript/ttl-caching
node simple_crud.js
node simple_websocket.js
node batch_operations.js
node kv_operations.js
node collection_management.js
node document_ttl.js
node websocket_ttl.js
```

#### Python

```bash
cd python/ttl-caching
python3 simple_crud.py
python3 simple_websocket.py
python3 batch_operations.py
python3 kv_operations.py
python3 collection_management.py
python3 document_ttl.py
python3 websocket_ttl.py
```

#### Go

```bash
cd go/ttl-caching
go run simple_crud.go
go run simple_websocket.go
go run batch_operations.go
go run kv_operations.go
go run collection_management.go
go run document_ttl.go
go run websocket_ttl.go
```

#### Rust

```bash
cd rust
cargo run --example simple_crud
cargo run --example simple_websocket
cargo run --example batch_operations
cargo run --example kv_operations
cargo run --example collection_management
cargo run --example document_ttl
cargo run --example websocket_ttl
```

### Test Runners

Each language has a test runner that executes all examples:

#### JavaScript Test Runner

```bash
cd javascript
npm install  # First time only
node test-runner.js
```

**Features:**

- Automatic server health check
- Authentication token management
- Colored output (✓/✗)
- Summary statistics

#### Python Test Runner

```bash
cd python
pip install -r requirements.txt  # First time only
python3 test_runner.py
```

#### Go Test Runner

```bash
cd go
go mod download  # First time only
go run test_runner.go
```

#### Rust Examples

```bash
cd rust
cargo run --example simple_crud  # Run individually
```

### Makefile Commands

From the repository root:

#### Run All Integration Tests

```bash
make test-examples
```

Runs all examples across all languages (JavaScript, Python, Go, Rust).

#### Run Language-Specific Tests

```bash
make test-examples-js       # JavaScript only
make test-examples-python   # Python only
make test-examples-go       # Go only
make test-examples-rust     # Rust only
```

#### Prerequisites for Makefile Commands

1. **ekoDB server must be running**:

   ```bash
   make run-release
   # or in another terminal:
   cargo run --release
   ```

2. **Dependencies** (handled automatically by test commands):
   - JavaScript: `npm install` (in examples/javascript)
   - Python: `pip install -r requirements.txt`
   - Go: `go mod download`
   - Rust: Cargo dependencies

#### Example Workflow

```bash
# Terminal 1: Start the server
make run-release

# Terminal 2: Run integration tests
make test-examples

# Or run specific language tests
make test-examples-js
make test-examples-python
make test-examples-go
make test-examples-rust
```

## 🧪 Integration Testing

### Architecture

All examples function as integration tests:

```
┌─────────────────────────────────────┐
│      Master Test Runner             │
│      (Makefile commands)            │
└──────────┬──────────────────────────┘
           │
           ├──> JavaScript Test Runner (test-runner.js)
           │    └──> 7 examples in ttl-caching/
           │
           ├──> Python Test Runner (test_runner.py)
           │    └──> 7 examples in ttl-caching/
           │
           ├──> Go Test Runner (test_runner.go)
           │    └──> 7 examples in ttl-caching/
           │
           └──> Rust Examples (cargo run --example)
                └──> 7 examples in examples/
```

### What Gets Tested

Each language tests:

- ✅ **HTTP API**: Insert, Find, Update, Delete operations
- ✅ **WebSocket API**: Real-time queries with authentication
- ✅ **Authentication**: JWT token generation and usage
- ✅ **Batch Operations**: Bulk insert, update, delete
- ✅ **Key-Value Store**: Simple KV operations
- ✅ **Collection Management**: Create, list, delete
- ✅ **TTL Features**: Document and WebSocket TTL
- ✅ **Error Handling**: Proper error responses

### Test Output

Successful test run:

```
╔════════════════════════════════════════╗
║  ekoDB JavaScript Examples Test Suite ║
╚════════════════════════════════════════╝

=== Checking Server Connection ===
✓ Server is ready

=== Getting Authentication Token ===
✓ Authentication successful

=== Running 7 Examples ===

=== Running simple_crud.js ===
✓ simple_crud.js completed successfully

=== Running simple_websocket.js ===
✓ simple_websocket.js completed successfully

[... 5 more examples ...]

╔════════════════════════════════════════╗
║           Test Summary                 ║
╚════════════════════════════════════════╝
Total: 7
Passed: 7
Failed: 0
```

### Output Features

- 🟢 **Green** - Success (✓)
- 🔴 **Red** - Failure (✗)
- 🟡 **Yellow** - Warnings
- 🔵 **Blue** - Info
- Summary statistics (Total, Passed, Failed)
- Detailed error messages if tests fail

## 🔐 Authentication

All examples use the ekoDB authentication system:

### Authentication Flow

```
1. Example starts
2. Reads API_BASE_KEY from environment (.env file)
3. Calls POST /api/auth/token with api_key
4. Receives JWT token
5. Uses token in Authorization header for all requests
```

### Default Credentials

- **API Key**: `a-test-api-key-from-ekodb` (default test key)
- **Token Generation**: Automatic via `/api/auth/token`
- **Token Usage**: `Authorization: Bearer <token>` header

### Custom API Key

To use a different API key:

1. **Edit `.env`**:

   ```bash
   API_BASE_KEY=your-custom-key-here
   ```

2. **Or set environment variable**:

   ```bash
   export API_BASE_KEY=your-custom-key-here
   ```

### Token Management

Test runners handle authentication automatically:

```javascript
async function getAuthToken() {
  const response = await fetch(`${BASE_URL}/api/auth/token`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ api_key: API_KEY }),
  });
  const data = await response.json();
  return data.token;
}
```

## ⚙️ Environment Configuration

### Environment Variables

| Variable          | Default                     | Description                     |
| ----------------- | --------------------------- | ------------------------------- |
| `API_BASE_URL`    | `http://localhost:8080`     | ekoDB server URL                |
| `API_BASE_KEY`    | `a-test-api-key-from-ekodb` | API authentication key          |
| `WS_BASE_URL`     | `ws://localhost:8080`       | WebSocket server URL            |
| `REQUEST_TIMEOUT` | `30000`                     | Request timeout in milliseconds |

### Configuration File

All examples read from `.env` file:

```bash
# .env file
API_BASE_URL=http://localhost:8080
API_BASE_KEY=a-test-api-key-from-ekodb
WS_BASE_URL=ws://localhost:8080
REQUEST_TIMEOUT=30000
```

### Setup

```bash
# Copy example file
cp .env.example .env

# Edit if needed (defaults work for local testing)
vim .env
```

## 🧹 Collection Cleanup

### Client Library Examples

All client library examples follow a **cleanup convention** to prevent test
pollution:

```javascript
// Collection naming pattern
const collection = "client_{example_name}_{language}";

// Cleanup section before completion
console.log("\n=== Cleanup ===");
await client.deleteCollection(collection);
console.log("✓ Deleted collection");
```

**Benefits:**

- ✅ No test pollution between runs
- ✅ Predictable collection names
- ✅ Easy to identify test collections
- ✅ Clean database state after tests

### Direct API Examples

Direct API examples use **predictable collection names** for learning purposes:

- `test_collection` - CRUD examples
- `batch_users` - Batch operations
- `websocket_test` - WebSocket examples
- `ttl_cache` - TTL examples
- `ws_ttl_test` - WebSocket TTL examples

These collections are **not automatically deleted** to allow inspection after
running examples. You can manually clean them up if needed:

```bash
# Using the API
curl -X DELETE http://localhost:8080/api/collection/test_collection \
  -H "Authorization: Bearer YOUR_TOKEN"

# Or restart the server with a fresh database
```

## 🚀 CI/CD Integration

These examples can be used in CI/CD pipelines for automated integration testing.

### GitHub Actions Example

```yaml
name: Integration Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest

    steps:
      - uses: actions/checkout@v3

      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable

      - name: Build ekoDB
        run: cargo build --release

      - name: Start ekoDB Server
        run: |
          cargo run --release &
          echo $! > server.pid

      - name: Wait for Server
        run: |
          for i in {1..30}; do
            if curl -s http://localhost:8080/api/health > /dev/null; then
              echo "Server ready"
              break
            fi
            sleep 1
          done

      - name: Run Integration Tests
        run: cd examples && ./run_tests.sh

      - name: Stop Server
        if: always()
        run: kill $(cat server.pid)
```

### GitLab CI Example

```yaml
integration_tests:
  stage: test
  script:
    - cargo build --release
    - cargo run --release &
    - sleep 5
    - cd examples && ./run_tests.sh
  after_script:
    - pkill ekodb || true
```

### Benefits

#### For Developers

- ✅ Verify API changes don't break examples
- ✅ Test locally before committing
- ✅ Examples stay up-to-date with API

#### For Users

- ✅ Working code examples
- ✅ Can run examples immediately
- ✅ Learn by running real code

#### For CI/CD

- ✅ Automated integration testing
- ✅ Catch regressions early
- ✅ Verify deployment health

### Comparison with Benchmarks

| Feature      | Examples (Integration Tests) | Benchmarks              |
| ------------ | ---------------------------- | ----------------------- |
| **Purpose**  | Functional correctness       | Performance measurement |
| **Speed**    | Fast (seconds)               | Slow (minutes)          |
| **Focus**    | API usage patterns           | Throughput & latency    |
| **Output**   | Pass/Fail                    | Metrics & graphs        |
| **Use Case** | CI/CD, development           | Performance tuning      |

## 🔧 Troubleshooting

### Server Not Running

**Error:**

```
✗ Server not responding at http://localhost:8080
```

**Solution:**

```bash
# Start the ekoDB server
cargo run --release

# Or use Makefile
make run-release
```

### Authentication Failed

**Error:**

```
✗ Authentication failed: 401
```

**Solution:**

- Check that your API key is registered with the server
- Default test key `a-test-api-key-from-ekodb` should work out of the box
- If using custom key, ensure it's registered

### WebSocket Connection Failed

**Error:**

```
Error: WebSocket connection failed
```

**Solution:**

- Ensure server is running
- Check that WebSocket endpoint is `/api/ws` (not `/ws`)
- Verify firewall settings
- Check that `WS_BASE_URL` is set correctly in `.env`

### Timeout Errors

**Error:**

```
✗ Example timed out
```

**Solution:**

```bash
# Increase timeout in .env
REQUEST_TIMEOUT=60000  # 60 seconds
```

### Dependency Installation Fails

**JavaScript:**

```bash
cd javascript
rm -rf node_modules package-lock.json
npm install
```

**Python:**

```bash
pip install --upgrade pip
pip install -r requirements.txt
```

**Go:**

```bash
go clean -modcache
go mod download
```

**Rust:**

```bash
cargo clean
cargo build
```

### Port Already in Use

**Error:**

```
Error: Address already in use (port 8080)
```

**Solution:**

```bash
# Find process using port 8080
lsof -i :8080

# Kill the process
kill -9 <PID>

# Or use different port
export API_BASE_URL=http://localhost:8081
export WS_BASE_URL=ws://localhost:8081
```

## ➕ Adding New Examples

To add a new example that works with the test runner:

### 1. Create Example File

Create your example in the appropriate language directory:

```bash
# JavaScript
touch javascript/ttl-caching/my_new_example.js

# Python
touch python/ttl-caching/my_new_example.py

# Go
touch go/ttl-caching/my_new_example.go

# Rust
touch rust/examples/my_new_example.rs
```

### 2. Use Environment Variables

```javascript
// JavaScript
const BASE_URL = process.env.API_BASE_URL || "http://localhost:8080";
const API_KEY = process.env.API_BASE_KEY || "a-test-api-key-from-ekodb";
```

```python
# Python
BASE_URL = os.getenv('API_BASE_URL', 'http://localhost:8080')
API_KEY = os.getenv('API_BASE_KEY', 'a-test-api-key-from-ekodb')
```

```go
// Go
baseURL := os.Getenv("API_BASE_URL")
apiKey := os.Getenv("API_BASE_KEY")
```

```rust
// Rust
let base_url = env::var("API_BASE_URL").unwrap_or_else(|_| "http://localhost:8080".to_string());
let api_key = env::var("API_BASE_KEY").unwrap_or_else(|_| "a-test-api-key-from-ekodb".to_string());
```

### 3. Implement Authentication

```javascript
// JavaScript
const response = await fetch(`${BASE_URL}/api/auth/token`, {
  method: "POST",
  headers: { "Content-Type": "application/json" },
  body: JSON.stringify({ api_key: API_KEY }),
});
const { token } = await response.json();
```

### 4. Add to Test Runner

Update the test runner's example list:

```javascript
// JavaScript: test-runner.js
const examples = [
  "ttl-caching/simple_crud.js",
  "ttl-caching/my_new_example.js", // Add here
  // ...
];
```

### 5. Test It

```bash
# Test individually
node javascript/ttl-caching/my_new_example.js

# Test with runner
cd javascript && node test-runner.js

# Test all languages
./run_tests.sh
```

### 6. Update Documentation

Add your example to this README under the appropriate category.

## 📖 Additional Documentation

For detailed API documentation and guides, see the official ekoDB documentation
at [docs.ekodb.io](https://docs.ekodb.io) (or your ekoDB server instance's
documentation)

## 📊 Summary

- **Total Examples**: 84+ examples
  - **Direct API**: 28 examples (JavaScript, Python, Go, Rust - 7 each)
  - **Client Libraries**: 56 examples (Rust: 14, Go: 14, TypeScript: 14,
    JavaScript: 7, Kotlin: 14)
  - **Transaction Examples**: Available across all languages (v0.24.0+)
- **Languages**: JavaScript, TypeScript, Python, Go, Rust, Kotlin
- **Test Coverage**: All examples tested automatically via Makefile
- **CI/CD Ready**: Easy integration into pipelines
- **Documentation**: Self-documenting code examples
- **Cleanup**: Client examples auto-cleanup, direct examples use predictable
  names
- **Feature Parity**: All client libraries have complete feature parity (14
  examples each)
- **v0.24.0 Compatible**: Transaction reliability improvements fully supported

### Quick Reference

| Command                            | Description                        |
| ---------------------------------- | ---------------------------------- |
| `make test-examples`               | Run all 84 examples                |
| `make test-examples-direct`        | Run 28 direct API examples         |
| `make test-examples-client`        | Run 56 client library examples     |
| `make test-examples-{lang}`        | Run all examples for a language    |
| `make test-examples-{lang}-direct` | Run direct examples for a language |
| `make test-examples-{lang}-client` | Run client examples for a language |

**Supported language aliases:** `rust`/`rs`, `python`/`py`, `typescript`/`ts`,
`javascript`/`js`, `go`, `kotlin`/`kt`
