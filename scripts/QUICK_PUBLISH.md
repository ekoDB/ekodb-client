# Quick Publishing Guide

## 🚀 One-Command Publishing

```bash
# Publish all packages
./scripts/publish-all.sh --all

# Publish specific packages
./scripts/publish-all.sh --rust --python
```

## 📋 Pre-Flight Checklist

Before publishing, ensure:

```bash
# 1. Tests pass
make test-ci

# 2. Update versions in:
#    - ekodb_client/Cargo.toml
#    - ekodb-client-py/Cargo.toml
#    - ekodb-client-ts/package.json
#    - Git tag for Go (v0.1.0)

# 3. Update CHANGELOG.md

# 4. Commit changes
git add .
git commit -m "chore: bump version to 0.1.0"
git push
```

## 🔑 Authentication Setup

### First Time Only

**Rust:**

```bash
cargo login
# Paste token from https://crates.io/settings/tokens
```

**Python:**

```bash
export MATURIN_PYPI_TOKEN=pypi-...
# Get token from https://pypi.org/manage/account/token/
```

**TypeScript:**

```bash
npm login
# Enter credentials
```

**Go:**

```bash
# No authentication needed, uses git tags
# Ensure repo is public at github.com/ekoDB/ekodb-client-go
```

## 📦 Individual Package Publishing

### Rust

```bash
./scripts/publish-rust.sh
# Publishes to crates.io
# Users install: cargo add ekodb_client
```

### Go

```bash
./scripts/publish-go.sh
# Creates and pushes git tag
# Users install: go get github.com/ekoDB/ekodb-client-go@v0.1.0
```

### Python

```bash
./scripts/publish-python.sh
# Publishes to PyPI
# Users install: pip install ekodb-client
```

### TypeScript

```bash
./scripts/publish-typescript.sh
# Publishes to npm
# Users install: npm install @ekodb/ekodb-client
```

## ✅ Verify Publication

After publishing, check:

- **Rust:** https://crates.io/crates/ekodb_client
- **Go:** https://pkg.go.dev/github.com/ekoDB/ekodb-client-go
- **Python:** https://pypi.org/project/ekodb-client/
- **TypeScript:** https://www.npmjs.com/package/@ekodb/ekodb-client

## 🐛 Common Issues

**"Not logged in"**

```bash
# Rust
cargo login

# Python
export MATURIN_PYPI_TOKEN=pypi-...

# TypeScript
npm login
```

**"Version already exists"**

```bash
# Bump version number in package manifest
# Then re-run publish script
```

**"Tests failed"**

```bash
# Fix tests first
make test-ci
```

## 📞 Need Help?

See full documentation: `./scripts/PUBLISHING.md`
