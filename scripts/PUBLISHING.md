# Publishing ekoDB Client Packages

This directory contains scripts to publish ekoDB client packages to their
respective package registries.

## Prerequisites

### All Languages

- Ensure all tests pass: `make test-ci`
- Update version numbers in package manifests
- Update CHANGELOG.md with release notes

### Rust (crates.io)

- Install Rust: https://rustup.rs/
- Login to crates.io: `cargo login`
- Get API token from: https://crates.io/settings/tokens

### Go (GitHub/pkg.go.dev)

- Git repository initialized
- Remote set to: `git@github.com:ekoDB/ekodb-client-go.git`
- **Note:** Go client should be in a separate public repository

### Python (PyPI)

- Install maturin: `pip install maturin`
- Create PyPI API token: https://pypi.org/manage/account/token/
- Set token: `export MATURIN_PYPI_TOKEN=pypi-...`
- Or create `~/.pypirc`:
  ```ini
  [pypi]
  username = __token__
  password = pypi-...
  ```
- **Note:** Maturin handles both building and publishing - no need for twine!

### TypeScript (npm)

- Install Node.js: https://nodejs.org/
- Login to npm: `npm login`
- For scoped packages (@ekodb/ekodb-client), ensure you have access

## Quick Start

### Publish All Packages

```bash
./scripts/publish-all.sh --all
```

### Publish Individual Packages

```bash
# Rust only
./scripts/publish-all.sh --rust

# Python only
./scripts/publish-all.sh --python

# TypeScript only
./scripts/publish-all.sh --typescript

# Go only
./scripts/publish-all.sh --go

# Multiple packages
./scripts/publish-all.sh --rust --python
```

### Direct Script Execution

```bash
# Rust
./scripts/publish-rust.sh

# Go
./scripts/publish-go.sh

# Python
./scripts/publish-python.sh

# TypeScript
./scripts/publish-typescript.sh
```

## Publishing Workflow

### 1. Pre-Release Checklist

- [ ] All tests passing
- [ ] Version numbers updated
- [ ] CHANGELOG.md updated
- [ ] README.md examples tested
- [ ] Documentation reviewed

### 2. Version Numbers

Update version in each package manifest:

**Rust** (`ekodb_client/Cargo.toml`):

```toml
[package]
version = "0.1.0"
```

**Go** (git tag):

```bash
git tag v0.1.0
```

**Python** (`ekodb-client-py/Cargo.toml`):

```toml
[package]
version = "0.1.0"
```

**TypeScript** (`ekodb-client-ts/package.json`):

```json
{
  "version": "0.1.0"
}
```

### 3. Run Publishing Script

```bash
# Test in dry-run mode first (where available)
./scripts/publish-rust.sh  # Has --dry-run built in

# Publish for real
./scripts/publish-all.sh --all
```

### 4. Verify Publication

**Rust:**

- https://crates.io/crates/ekodb_client
- https://docs.rs/ekodb_client

**Go:**

- https://pkg.go.dev/github.com/ekoDB/ekodb-client-go

**Python:**

- https://pypi.org/project/ekodb-client/

**TypeScript:**

- https://www.npmjs.com/package/@ekodb/ekodb-client

### 5. Post-Release

- [ ] Create GitHub release with tag
- [ ] Announce on social media/blog
- [ ] Update documentation website
- [ ] Monitor for issues

## Troubleshooting

### Rust: "failed to verify package tarball"

```bash
# Allow dirty working directory
cargo publish --allow-dirty
```

### Go: "repository not found"

```bash
# Ensure the repository is public and pushed
git remote -v
git push origin main
git push origin v0.1.0
```

### Python: "invalid authentication credentials"

```bash
# Check your PyPI token
echo $MATURIN_PYPI_TOKEN

# Or re-login
maturin publish --username __token__ --password pypi-...
```

### TypeScript: "need to login"

```bash
# Login to npm
npm login

# Check login status
npm whoami
```

### TypeScript: "scope not found"

```bash
# For scoped packages, ensure public access
npm publish --access public
```

## Version Strategy

We follow [Semantic Versioning](https://semver.org/):

- **MAJOR** (1.0.0): Breaking changes
- **MINOR** (0.1.0): New features, backwards compatible
- **PATCH** (0.0.1): Bug fixes, backwards compatible

### Pre-1.0.0 Releases

- 0.1.0 - Initial public release
- 0.2.0 - Feature additions
- 0.2.1 - Bug fixes

## CI/CD Integration

These scripts can be integrated into CI/CD pipelines:

```yaml
# .github/workflows/publish.yml
name: Publish Packages

on:
  push:
    tags:
      - "v*"

jobs:
  publish:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Publish Rust
        run: ./scripts/publish-rust.sh
        env:
          CARGO_REGISTRY_TOKEN: ${{ secrets.CARGO_TOKEN }}

      - name: Publish Python
        run: ./scripts/publish-python.sh
        env:
          MATURIN_PYPI_TOKEN: ${{ secrets.PYPI_TOKEN }}

      - name: Publish TypeScript
        run: ./scripts/publish-typescript.sh
        env:
          NPM_TOKEN: ${{ secrets.NPM_TOKEN }}
```

## Support

For issues with publishing:

- Check package registry status pages
- Review authentication credentials
- Ensure version numbers are incremented
- Check for existing versions

For questions, contact the ekoDB team.
