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
```

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

- ✅ Examples for all languages (Rust, Python, Go, TypeScript, JavaScript)
- ✅ Both direct API and client library examples
- ✅ Automatic client building before running examples
