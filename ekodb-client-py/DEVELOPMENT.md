# ekoDB Python Package Development

This package provides Python bindings for the ekoDB Rust client library.

## Architecture

```
ekodb-py/
├── src/lib.rs          # PyO3 bindings (wraps ekodb_client)
├── python/ekodb/       # Python package wrapper
├── Cargo.toml          # Rust dependencies
└── pyproject.toml      # Python package metadata
```

## Prerequisites

```bash
# Install maturin (Python package builder for Rust extensions)
pip install maturin

# Or with pipx for global installation
pipx install maturin
```

## Building

### Development Build

```bash
# From project root
make install-python-dev

# Or manually
cd ekodb-py
maturin develop
```

### Release Build

```bash
# From project root
make build-python

# Or manually
cd ekodb-py
maturin build --release
```

## Testing

```bash
# Run the example
make test-examples-python-client

# Or manually
cd examples/python/client
python3 simple_crud.py
```

## How It Works

1. **`ekodb_client`** (Rust) - Core client library with all functionality
2. **`ekodb-py/src/lib.rs`** (Rust + PyO3) - Thin wrapper exposing Rust
   functions to Python
3. **`python/ekodb/__init__.py`** (Python) - Python package that imports the
   compiled extension

The Python package reuses 100% of the Rust client logic, ensuring consistency
and performance.

## Key Files

- **`src/lib.rs`** - PyO3 bindings that wrap `ekodb_client::Client`
- **`Cargo.toml`** - Defines the `cdylib` crate type for Python extension
- **`pyproject.toml`** - Python package metadata for maturin
- **`python/ekodb/__init__.py`** - Python module that exports the compiled
  extension

## Notes

- This package is **excluded** from the main Cargo workspace
- Build with `maturin`, not `cargo build`
- The compiled extension is named `_ekodb` (with underscore)
- Python imports it as `from ekodb import Client`
