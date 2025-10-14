#!/bin/bash

echo "🔍 Checking if packages are ready to publish"
echo "=============================================="
echo ""

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

READY=true

# Check Rust
echo "📦 Rust Client (ekodb_client)"
echo "------------------------------"
if [ -f "$REPO_ROOT/ekodb_client/Cargo.toml" ]; then
    VERSION=$(grep '^version = ' "$REPO_ROOT/ekodb_client/Cargo.toml" | head -1 | cut -d'"' -f2)
    echo "  Version: $VERSION"
    
    # Check if cargo is installed
    if command -v cargo &> /dev/null; then
        echo "  ✅ cargo installed"
        
        # Check if logged in
        if cargo login --help &> /dev/null; then
            echo "  ✅ cargo available"
        else
            echo "  ⚠️  cargo login status unknown"
        fi
    else
        echo "  ❌ cargo not installed"
        READY=false
    fi
    
    # Check if tests pass
    echo "  Running tests..."
    cd "$REPO_ROOT/ekodb_client"
    if cargo test --lib --quiet 2>/dev/null; then
        echo "  ✅ Tests pass"
    else
        echo "  ❌ Tests fail"
        READY=false
    fi
else
    echo "  ❌ Cargo.toml not found"
    READY=false
fi

echo ""

# Check Python
echo "📦 Python Client (ekodb-client-py)"
echo "-----------------------------------"
if [ -f "$REPO_ROOT/ekodb-client-py/Cargo.toml" ]; then
    VERSION=$(grep '^version = ' "$REPO_ROOT/ekodb-client-py/Cargo.toml" | head -1 | cut -d'"' -f2)
    echo "  Version: $VERSION"
    
    # Check if maturin is installed
    if command -v maturin &> /dev/null; then
        echo "  ✅ maturin installed"
    else
        echo "  ❌ maturin not installed (pip install maturin)"
        READY=false
    fi
    
    # Check if PyPI token is set
    if [ -n "$MATURIN_PYPI_TOKEN" ]; then
        echo "  ✅ MATURIN_PYPI_TOKEN set"
    elif [ -f "$HOME/.pypirc" ]; then
        echo "  ✅ ~/.pypirc found"
    else
        echo "  ⚠️  PyPI credentials not configured"
        echo "     Set MATURIN_PYPI_TOKEN or create ~/.pypirc"
    fi
    
    # Check if Rust tests pass (Python client depends on Rust client)
    echo "  Running Rust client tests..."
    cd "$REPO_ROOT/ekodb_client"
    if cargo test --lib --quiet 2>/dev/null; then
        echo "  ✅ Core tests pass"
    else
        echo "  ❌ Core tests fail"
        READY=false
    fi
else
    echo "  ❌ Cargo.toml not found"
    READY=false
fi

echo ""

# Check TypeScript
echo "📦 TypeScript Client (ekodb-client-ts)"
echo "---------------------------------------"
if [ -f "$REPO_ROOT/ekodb-client-ts/package.json" ]; then
    VERSION=$(node -p "require('$REPO_ROOT/ekodb-client-ts/package.json').version" 2>/dev/null || echo "unknown")
    PACKAGE_NAME=$(node -p "require('$REPO_ROOT/ekodb-client-ts/package.json').name" 2>/dev/null || echo "unknown")
    echo "  Package: $PACKAGE_NAME"
    echo "  Version: $VERSION"
    
    # Check if npm is installed
    if command -v npm &> /dev/null; then
        echo "  ✅ npm installed ($(npm --version))"
        
        # Check if logged in
        if npm whoami &> /dev/null; then
            NPM_USER=$(npm whoami)
            echo "  ✅ Logged in as: $NPM_USER"
        else
            echo "  ⚠️  Not logged in to npm (run: npm login)"
        fi
    else
        echo "  ❌ npm not installed"
        READY=false
    fi
    
    # Check if dependencies are installed
    cd "$REPO_ROOT/ekodb-client-ts"
    if [ -d "node_modules" ]; then
        echo "  ✅ Dependencies installed"
    else
        echo "  ⚠️  Dependencies not installed (run: npm install)"
    fi
    
    # Check if build works
    if [ -d "dist" ]; then
        echo "  ✅ Build output exists"
    else
        echo "  ⚠️  No build output (run: npm run build)"
    fi
else
    echo "  ❌ package.json not found"
    READY=false
fi

echo ""
echo "=============================================="

if [ "$READY" = true ]; then
    echo "✅ All packages are ready to publish!"
    echo ""
    echo "Next steps:"
    echo "  1. Review versions and update if needed"
    echo "  2. Update CHANGELOG.md"
    echo "  3. Run: ./scripts/publish-all.sh --all"
    exit 0
else
    echo "❌ Some packages are not ready to publish"
    echo ""
    echo "Please fix the issues above before publishing"
    exit 1
fi
