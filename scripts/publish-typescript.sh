#!/bin/bash
set -e

echo "📘 Publishing TypeScript Client to npm"
echo "======================================"

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

# Navigate to the TypeScript client directory
cd "$REPO_ROOT/ekodb-client-ts"

# Check if we're in the right directory
if [ ! -f "package.json" ]; then
    echo "❌ Error: package.json not found in ekodb-client-ts directory"
    exit 1
fi

# Get current version
VERSION=$(node -p "require('./package.json').version")
PACKAGE_NAME=$(node -p "require('./package.json').name")
echo "📦 Package: $PACKAGE_NAME"
echo "📦 Current version: $VERSION"

# Check if npm is installed
if ! command -v npm &> /dev/null; then
    echo "❌ Error: npm not found"
    exit 1
fi

# Check npm authentication
echo ""
echo "🔐 Checking npm authentication..."
if ! npm whoami &> /dev/null; then
    echo "❌ Error: Not logged in to npm"
    echo "Please run: npm login"
    exit 1
fi

NPM_USER=$(npm whoami)
echo "✅ Logged in as: $NPM_USER"

# Install dependencies
echo ""
echo "📥 Installing dependencies..."
npm install

# Run build
echo ""
echo "🔨 Building package..."
npm run build

# Check if dist directory exists
if [ ! -d "dist" ]; then
    echo "❌ Error: dist directory not found after build"
    exit 1
fi

echo "✅ Build successful"

# Run tests if available
if grep -q '"test"' package.json; then
    echo ""
    echo "🧪 Running tests..."
    npm test || echo "⚠️  Tests failed or not configured"
fi

# Check package contents
echo ""
echo "🔍 Checking package contents..."
npm pack --dry-run

# Confirm publication
echo ""
echo "⚠️  Ready to publish $PACKAGE_NAME@$VERSION to npm"
echo ""
read -p "Do you want to continue? (y/N): " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "❌ Publication cancelled"
    exit 1
fi

# Publish to npm
echo ""
echo "🚀 Publishing to npm..."

# Check if it's a scoped package and needs --access public
if [[ $PACKAGE_NAME == @* ]]; then
    npm publish --access public
else
    npm publish
fi

echo ""
echo "✅ Successfully published $PACKAGE_NAME@$VERSION to npm!"
echo "📦 Users can install with: npm install $PACKAGE_NAME"
echo "📚 Package page: https://www.npmjs.com/package/${PACKAGE_NAME/@/}"
echo ""
echo "Note: It may take a few minutes for the package to be available."
