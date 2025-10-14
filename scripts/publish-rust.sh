#!/bin/bash
set -e

echo "🦀 Publishing Rust Client to crates.io"
echo "========================================"

# Get paths
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# Load .env file if it exists
if [ -f "$REPO_ROOT/.env" ]; then
    echo "🔐 Loading environment variables from .env..."
    set -a
    source "$REPO_ROOT/.env"
    set +a
    echo "✅ Environment variables loaded"
    echo ""
fi

# Navigate to the Rust client directory
cd "$REPO_ROOT/ekodb_client"

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Error: Cargo.toml not found in ekodb_client directory"
    exit 1
fi

# Get current version
VERSION=$(grep '^version = ' Cargo.toml | head -1 | cut -d'"' -f2)
echo "📦 Current version: $VERSION"

# Check if already logged in to crates.io
echo ""
echo "🔐 Checking crates.io authentication..."
if ! cargo login --help &> /dev/null; then
    echo "❌ Error: cargo not found"
    exit 1
fi

# Run tests before publishing
echo ""
echo "🧪 Running tests..."
cargo test --lib

# Build the package
echo ""
echo "🔨 Building package..."
cargo build --release

# Check the package
echo ""
echo "🔍 Checking package..."
cargo package --allow-dirty

# Dry run
echo ""
echo "🏃 Performing dry run..."
cargo publish --dry-run --allow-dirty

# Confirm publication
echo ""
echo "⚠️  Ready to publish ekodb_client v$VERSION to crates.io"
read -p "Do you want to continue? (y/N): " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "❌ Publication cancelled"
    exit 1
fi

# Publish
echo ""
echo "🚀 Publishing to crates.io..."
cargo publish --allow-dirty

echo ""
echo "✅ Successfully published ekodb_client v$VERSION to crates.io!"
echo "📦 Users can install with: cargo add ekodb_client"
echo "📚 Documentation will be available at: https://docs.rs/ekodb_client"
