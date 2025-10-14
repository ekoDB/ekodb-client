#!/bin/bash
set -e

echo "🐍 Publishing Python Client to PyPI"
echo "===================================="

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

# Navigate to the Python client directory
cd "$REPO_ROOT/ekodb-client-py"

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Error: Cargo.toml not found in ekodb-client-py directory"
    exit 1
fi

# Get current version
VERSION=$(grep '^version = ' Cargo.toml | head -1 | cut -d'"' -f2)
echo "📦 Current version: $VERSION"

# Check if maturin is installed
if ! command -v maturin &> /dev/null; then
    echo ""
    echo "❌ Error: maturin not found"
    echo "Install with: pip install maturin"
    exit 1
fi

echo "✅ maturin installed ($(maturin --version))"

# Check PyPI credentials
echo ""
echo "🔐 Checking PyPI authentication..."
if [ -n "$MATURIN_PYPI_TOKEN" ]; then
    echo "✅ MATURIN_PYPI_TOKEN environment variable is set"
elif [ -f "$HOME/.pypirc" ]; then
    echo "✅ Found ~/.pypirc configuration file"
else
    echo "⚠️  PyPI credentials not configured"
    echo ""
    echo "Please set up authentication using ONE of these methods:"
    echo ""
    echo "Method 1 (Recommended): Environment variable"
    echo "  export MATURIN_PYPI_TOKEN=pypi-..."
    echo "  Get token from: https://pypi.org/manage/account/token/"
    echo ""
    echo "Method 2: Configuration file"
    echo "  Create ~/.pypirc with:"
    echo "  [pypi]"
    echo "  username = __token__"
    echo "  password = pypi-..."
    echo ""
    read -p "Continue anyway? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi
echo ""

# Run Rust tests
echo "🧪 Running Rust tests..."
cd ../ekodb_client
cargo test --lib
cd ../ekodb-client-py

# Build the package
echo ""
echo "🔨 Building wheels..."
maturin build --release

# List built wheels
echo ""
echo "📦 Built wheels:"
ls -lh target/wheels/

# Test build in a virtual environment
echo ""
echo "🧪 Testing build..."
python3 -m venv test_env
source test_env/bin/activate
pip install target/wheels/*.whl
python -c "import ekodb_client; print('✅ Import successful')" || echo "❌ Import failed"
deactivate
rm -rf test_env

# Confirm publication
echo ""
echo "⚠️  Ready to publish ekodb-client v$VERSION to PyPI"
echo ""
echo "Publishing to PyPI (production):"
read -p "Do you want to continue? (y/N): " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "❌ Publication cancelled"
    exit 1
fi

# Publish to PyPI
echo ""
echo "🚀 Publishing to PyPI..."
maturin publish --username __token__

echo ""
echo "✅ Successfully published ekodb-client v$VERSION to PyPI!"
echo "📦 Users can install with: pip install ekodb-client"
echo "📚 Package page: https://pypi.org/project/ekodb-client/"
echo ""
echo "Note: It may take a few minutes for the package to be available."
