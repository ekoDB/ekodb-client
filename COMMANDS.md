# ekoDB Client Libraries - Quick Reference

## 🚀 Deployment Commands

### Python Client (PyPI)

**Option 1: Multi-platform (Recommended)**

```bash
# Bump version
make bump-client-py
# Enter new version (e.g., 0.1.5)

# Deploy to PyPI (builds Linux + macOS wheels + source dist)
make deploy-client-py
```

**What it does:**

- Cleans old wheels
- Builds Linux wheel via Docker (ARM64 + x86_64)
- Builds macOS wheel locally
- Creates source distribution (.tar.gz)
- Uploads all to PyPI using twine

**Requirements:**

- `MATURIN_PYPI_TOKEN` in `.env` file
- Docker running (for Linux builds)
- `twine` installed (auto-installed if missing)

**Option 2: Simple (Current Platform)**

```bash
# Uses the publish script
make deploy-client-py-simple
# OR
./scripts/publish-python.sh
```

**What it does:**

- Runs tests before publishing
- Builds wheel for current platform only
- Tests the build in a virtual environment
- Prompts for confirmation before publishing
- Publishes to PyPI

### Rust Client (crates.io)

```bash
# Uses the publish script
make deploy-client-rust
# OR
./scripts/publish-rust.sh
```

**What it does:**

- Runs tests before publishing
- Performs dry-run
- Prompts for confirmation
- Publishes `ekodb_client` to crates.io

### Go Client (pkg.go.dev)

```bash
make deploy-client-go
```

**What it does:**

- Shows instructions for tagging and pushing
- Go modules are published via git tags

### TypeScript Client (npm)

```bash
# Uses the publish script
make deploy-client-ts
# OR
./scripts/publish-typescript.sh
```

**What it does:**

- Installs dependencies
- Builds TypeScript client
- Runs tests (if available)
- Prompts for confirmation
- Publishes to npm

### Publish All Clients

```bash
./scripts/publish-all.sh
```

**What it does:**

- Publishes Rust, Python, and TypeScript clients in sequence
- Uses the individual publish scripts

## 🛠️ Setup & Build Commands

### Initial Setup

```bash
# Install ALL client libraries (Rust, Python, TypeScript, Go)
make install
```

**What it does:**

- Builds Rust client (release mode)
- Builds and installs Python client locally
- Installs TypeScript client dependencies
- Downloads Go client dependencies
- Makes all scripts executable

**Install individual clients:**

```bash
make install-rust        # Rust only
make install-python      # Python only
make install-typescript  # TypeScript only
make install-go          # Go only
```

**Alternative: Setup without installing**

```bash
# Just fetch dependencies and set up environment
make setup
```

### Build All Clients

```bash
# Build ALL client libraries (Rust, Python, TypeScript)
make build
```

**What it does:**

- Builds Rust client (`ekodb_client`)
- Builds Python client (via maturin)
- Builds TypeScript client (npm build)

### Build Individual Clients

```bash
# Build Rust client only
make build-client

# Build Python client only
make build-python-client

# Build TypeScript client only
make build-typescript-client

# Build Rust client in release mode
make build-release
```

## 🧪 Testing Commands

```bash
# Test Rust client
make test

# Test all examples (all languages)
make test-examples

# Test specific language examples
make test-examples-rust      # or test-examples-rs
make test-examples-python    # or test-examples-py
make test-examples-go
make test-examples-typescript # or test-examples-ts
make test-examples-javascript # or test-examples-js

# Test client libraries only
make test-examples-client
make test-examples-rust-client
make test-examples-python-client
make test-examples-go-client
make test-examples-typescript-client
make test-examples-javascript-client
make test-examples-kotlin-client
```

Offline parity and compilation checks do not require a server:

```bash
make example-parity-check
EKODB_CLIENT_GO_DIR=/path/to/ekodb-client-go-worktree make check-client-examples
```

The Go override ensures a client worktree compiles examples against the paired
Go feature worktree instead of a sibling default-branch checkout.

### Search and Schema Compatibility

```bash
# Shared search request fixtures (Rust, TypeScript, Kotlin)
cargo test -p ekodb_client --test search_wire
(cd ekodb-client-ts && npm test -- src/search-wire.test.ts)
(cd ekodb-client-kt && ./gradlew test --tests io.ekodb.client.SearchTest)

# Shared Vector insertion fixture (ordinary arrays stay distinct)
cargo test -p ekodb_client --test vector_record_wire
(cd ekodb-client-ts && npm test -- src/vector-record-wire.test.ts)
(cd ekodb-client-kt && ./gradlew test --tests io.ekodb.client.VectorRecordTest)

# Kotlin schema types and collection error handling
(cd ekodb-client-kt && ./gradlew test --tests io.ekodb.client.SchemaBuilderTest --tests io.ekodb.client.CollectionErrorTest)
```

`test-fixtures/search-requests.json` covers filtered vectors, custom hybrid
weights, named vector fields, and weighted fuzzy text search. Each client
constructs and compares the complete request. Rust's unset cache flags and limit
serialize as null; Kotlin and TypeScript omit them. The shared cases set those
options explicitly; separate tests preserve the unset-value differences.
Comparisons ignore object key order but retain all keys, nulls, arrays, and
numeric values.

`test-fixtures/schema-probe-0.72.2.json` contains redacted schema responses from
server 0.72.2, including collection cleanup results. Kotlin schema tests use the
accepted field types as regression fixtures. These commands run locally without
a live server; they do not measure ranking or ANN recall.

The document Vector envelope is `{"type":"Vector","value":[...]}`. Rust's
`FieldType::vector`, TypeScript's `Field.vector`, Python's `field_vector`, and
Kotlin's `FieldType.vector` all construct that envelope.
`test-fixtures/vector-record.json` compares complete insertion payloads without
collapsing Array and Vector. Rust and TypeScript codec tests retain the same
distinction in MessagePack; this is not a live binary-transport claim. Kotlin
HTTP MessagePack/CBOR remains experimental and unverified; this fix validates
JSON. Search query vectors use a separate plain numeric array contract.

```bash
# Explicit opt-in; provide these through your authorized environment.
# Creates one unique collection, runs bounded checks, and deletes it in finally.
: "${EKODB_BASE_URL:?required}" "${EKODB_API_KEY:?required}"
(cd ekodb-client-kt && ./gradlew vectorLiveTest)
```

The task fails if credentials are missing; normal `test` never runs it. Review
`ekodb-client-kt/build/vector-live-contract.json` before saving evidence.
[Redacted live evidence](test-fixtures/vector-live-contract.json) records the
2026-09-09 UTC create/readback → public Kotlin insert/fetch → named-field cosine
search → Boolean-filtered search → upsert/search → bad-array 400 → cleanup run.
The nearest score was 1.0; filtering it out returned the eligible record at 0.8.
The invalid Array insert preserved its body and made exactly one request.
Authenticated `/api/health` exposed version 0.72.2, commit `b5014f0`; the
unauthenticated response exposed no version. This is separate from the earlier
schema-only probe, which did **not** test record insertion. The preliminary
protocol probe also accepted an empty tagged vector on an unconstrained schema.
Cleanup is verified through collection listing: GET collection metadata can
return an empty schema after deletion. This bounded test makes no broad
prefilter, empty-vector search, recall, or ANN guarantees.

## 📚 Documentation & Formatting

### Generate Documentation

```bash
# Generate Rust client docs
make docs
```

### Format Code

**Format everything (all languages):**

```bash
make fmt
# OR
make format
```

This formats:

- ✅ Rust code (client + examples) using `cargo fmt`
- ✅ Python code (client + examples) using `black` (or `autopep8`)
- ✅ Go code (client + examples) using `gofmt`
- ✅ TypeScript/JavaScript code using `prettier`
- ✅ Markdown files using `prettier` (or `markdownlint`)

**Format specific languages:**

```bash
# Rust only
make fmt-rust
make fmt-rust-client    # Just the client library
make fmt-rust-examples  # Just the examples

# Python only
make fmt-python

# Go only
make fmt-go

# TypeScript/JavaScript only
make fmt-typescript

# Markdown only
make fmt-md
```

**Auto-installation:**

- Missing formatters are automatically installed when you run the commands
- `black` for Python (via pip)
- `prettier` for TypeScript/Markdown (via npm)
- `gofmt` comes with Go installation

## 🧹 Cleanup

```bash
# Clean build artifacts
make clean

# Clean everything including dependencies
make clean-all
```

## 📝 Environment Setup

Create a `.env` file in the root with:

```bash
# PyPI Token (for Python client deployment)
MATURIN_PYPI_TOKEN=pypi-your-token-here

# npm Token (for TypeScript client deployment)
NPM_TOKEN=npm_your-token-here
```

## 🔄 Typical Workflow

### Releasing a new Python client version:

```bash
# 1. Make your changes to ekodb-client-py/

# 2. Bump the version
make bump-client-py
# Enter: 0.1.5

# 3. Test locally
make build-python-client
make test-examples-python-client

# 4. Deploy to PyPI
make deploy-client-py

# 5. Commit and tag
git add -A
git commit -m "feat(python-client): bump to v0.1.5"
git tag ekodb-client-py/v0.1.5
git push origin main --tags
```

### Releasing a new Rust client version:

```bash
# 1. Update version in ekodb_client/Cargo.toml

# 2. Test
make test

# 3. Deploy
make deploy-client-rust

# 4. Commit and tag
git add -A
git commit -m "feat(rust-client): bump to v0.1.3"
git tag ekodb_client/v0.1.3
git push origin main --tags
```

## 🎯 Key Features

### Python Client Deployment

- ✅ Multi-platform support (Linux ARM64, macOS ARM64)
- ✅ Source distribution for other platforms
- ✅ Uses rustls (no OpenSSL dependency)
- ✅ Automatic wheel cleanup before build
- ✅ Auto-installs twine if missing

### Version Bumping

- ✅ Interactive version prompt
- ✅ Updates both `Cargo.toml` and `pyproject.toml`
- ✅ Shows which files were updated

### Testing

- ✅ Examples for all languages (Rust, Python, Go, TypeScript/JavaScript,
  Kotlin)
- ✅ Both direct API and client library examples
- ✅ Automatic client building before running examples
