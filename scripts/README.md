# ekoDB Publishing Scripts

Automated scripts for publishing ekoDB client packages to their respective
package registries.

## 📁 Files

| Script                   | Purpose                                           |
| ------------------------ | ------------------------------------------------- |
| `publish-all.sh`         | Master script to publish all or selected packages |
| `publish-rust.sh`        | Publish Rust client to crates.io                  |
| `publish-python.sh`      | Publish Python client to PyPI                     |
| `publish-typescript.sh`  | Publish TypeScript client to npm                  |
| `check-publish-ready.sh` | Verify all packages are ready to publish          |
| `PUBLISHING.md`          | Comprehensive publishing guide                    |
| `QUICK_PUBLISH.md`       | Quick reference for publishing                    |

**Note:** Go client is in a separate repository at
[github.com/ekoDB/ekodb-client-go](https://github.com/ekoDB/ekodb-client-go)

## 🚀 Quick Start

### 1. Check if ready to publish

```bash
./scripts/check-publish-ready.sh
```

### 2. Publish all packages

```bash
./scripts/publish-all.sh --all
```

### 3. Or publish individually

```bash
./scripts/publish-all.sh --rust
./scripts/publish-all.sh --python
./scripts/publish-all.sh --typescript

# Go client (from its own directory)
cd ekodb-client-go && ./publish.sh
```

## 📦 Package Registries

| Language       | Registry                         | Installation Command                      |
| -------------- | -------------------------------- | ----------------------------------------- |
| **Rust**       | [crates.io](https://crates.io)   | `cargo add ekodb_client`                  |
| **Go**         | [pkg.go.dev](https://pkg.go.dev) | `go get github.com/ekoDB/ekodb-client-go` |
| **Python**     | [PyPI](https://pypi.org)         | `pip install ekodb-client`                |
| **TypeScript** | [npm](https://npmjs.com)         | `npm install @ekodb/ekodb-client`         |

## 🔑 Authentication Required

Before first publish, set up authentication. All scripts automatically load
tokens from `.env` if it exists.

### Option 1: Use .env file (Recommended)

Add tokens to `.env` in the repository root:

```bash
# .env
MATURIN_PYPI_TOKEN=pypi-...
NPM_TOKEN=npm_...
CARGO_REGISTRY_TOKEN=cio...
```

Scripts will automatically load these tokens!

### Option 2: Manual authentication

**Rust (crates.io)**

```bash
cargo login
# Get token from: https://crates.io/settings/tokens
```

**Python (PyPI)**

```bash
export MATURIN_PYPI_TOKEN=pypi-...
# Get token from: https://pypi.org/manage/account/token/
```

**TypeScript (npm)**

```bash
npm login
# Enter your npm credentials
```

**Go (GitHub)**

```bash
# No special auth needed
# Uses git credentials for pushing tags
```

## 📋 Pre-Publish Checklist

- [ ] All tests pass: `make test-ci`
- [ ] Version numbers updated in all package manifests
- [ ] CHANGELOG.md updated
- [ ] README examples tested
- [ ] Authentication configured for target registries

## 🔄 Publishing Workflow

```bash
# 1. Check readiness
./scripts/check-publish-ready.sh

# 2. Update versions
# Edit: ekodb_client/Cargo.toml
# Edit: ekodb-client-py/Cargo.toml
# Edit: ekodb-client-ts/package.json
# Git tag for Go: v0.1.0

# 3. Commit changes
git add .
git commit -m "chore: bump version to 0.1.0"
git push

# 4. Publish
./scripts/publish-all.sh --all

# 5. Verify
# Check package registry pages (see links above)

# 6. Create GitHub release
git tag v0.1.0
git push origin v0.1.0
```

## 📚 Documentation

- **Full Guide:** [PUBLISHING.md](./PUBLISHING.md)
- **Quick Reference:** [QUICK_PUBLISH.md](./QUICK_PUBLISH.md)

## 🐛 Troubleshooting

### "Not logged in" errors

Run the appropriate login command for your package registry (see Authentication
section above).

### "Version already exists"

Increment the version number in the package manifest and try again.

### "Tests failed"

Fix failing tests before publishing:

```bash
make test-ci
```

### Go: "Repository not found"

Ensure the Go client is in a separate public repository at
`github.com/ekoDB/ekodb-client-go`.

## 🎯 CI/CD Integration

These scripts can be integrated into GitHub Actions or other CI/CD systems. See
[PUBLISHING.md](./PUBLISHING.md) for examples.

## 📞 Support

For issues or questions:

1. Check [PUBLISHING.md](./PUBLISHING.md) for detailed troubleshooting
2. Review package registry documentation
3. Contact the ekoDB team

## 🔐 Security Notes

- Never commit API tokens or credentials to the repository
- Use environment variables or secure credential storage
- Rotate tokens periodically
- Use scoped tokens with minimal required permissions

## 📊 Version Strategy

We follow [Semantic Versioning](https://semver.org/):

- **MAJOR.MINOR.PATCH** (e.g., 1.2.3)
- Pre-1.0.0 releases indicate API may change

Current versions:

- Check individual package manifests for current versions
- All client packages should maintain version parity when possible
