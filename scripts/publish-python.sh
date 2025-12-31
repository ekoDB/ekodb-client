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

# Build the package for multiple platforms
echo ""
echo "🔨 Building wheels for multiple platforms..."
echo ""

# Clean old wheels
echo "🧹 Cleaning old wheels..."
rm -rf target/wheels/*

# Build for Linux (via Docker)
echo "🐧 Building for Linux (via Docker)..."

# Check if Docker is available and running
if ! command -v docker &> /dev/null; then
    echo "⚠️  Docker not found, skipping Linux build"
    echo "   (Linux users won't be able to install)"
elif ! docker info > /dev/null 2>&1; then
    echo "🔄 Docker is not running. Attempting to start..."
    
    # Try to start Colima
    if command -v colima > /dev/null 2>&1; then
        echo "🔄 Checking Colima status..."
        if colima status 2>/dev/null | grep -q "Running"; then
            echo "⏳ Colima is already running but Docker socket isn't ready. Waiting..."
            sleep 5
            for i in {1..12}; do
                if docker info > /dev/null 2>&1; then
                    echo "✅ Docker is now ready"
                    break
                fi
                echo "⏳ Still waiting for Docker... ($i/12)"
                sleep 5
            done
        else
            echo "🔄 Starting Colima..."
            colima start || {
                echo "❌ Failed to start Colima. Skipping Linux build."
                echo "   Run 'colima start' manually to build Linux wheels."
                SKIP_LINUX=true
            }
        fi
    # Try to start Docker Desktop on macOS
    elif command -v open > /dev/null 2>&1 && [ -d "/Applications/Docker.app" ]; then
        echo "🔄 Starting Docker Desktop..."
        open -a Docker
        echo "⏳ Waiting for Docker to start..."
        for i in {1..30}; do
            if docker info > /dev/null 2>&1; then
                echo "✅ Docker is now ready"
                break
            fi
            sleep 2
        done
    else
        echo "⚠️  Could not find Docker Desktop or Colima. Skipping Linux build."
        echo "   Install Colima with: brew install colima"
        SKIP_LINUX=true
    fi
    
    # Final check
    if ! docker info > /dev/null 2>&1; then
        echo "❌ Docker is still not ready. Skipping Linux build."
        SKIP_LINUX=true
    fi
fi

# Build Linux wheel if Docker is ready
if [ "$SKIP_LINUX" != "true" ] && docker info > /dev/null 2>&1; then
    # Mount the parent directory so ekodb_client dependency can be found
    docker run --rm \
        -v "$REPO_ROOT":/workspace \
        -w /workspace/ekodb-client-py \
        ghcr.io/pyo3/maturin build --release --manylinux 2_28
    echo "✅ Linux wheel built"
else
    echo "⚠️  Skipping Linux build (Docker not available)"
    echo "   macOS and source distribution will still be built"
fi

# Build for macOS (current platform) + source distribution
echo ""
echo "🍎 Building for macOS + source distribution..."
maturin build --release --sdist
echo "✅ macOS wheel and source distribution built"

# List built wheels
echo ""
echo "📦 Built wheels:"
ls -lh target/wheels/

# Test build in a virtual environment
echo ""
echo "🧪 Testing build..."
python3 -m venv test_env
source test_env/bin/activate
# Install only the platform-specific wheel (exclude Linux wheels on macOS)
WHEEL=$(ls -t target/wheels/*.whl | grep -v manylinux | grep -v musllinux | head -1)
if [ -n "$WHEEL" ]; then
    pip install "$WHEEL"
    python -c "import ekodb_client; print('✅ Import successful')" || echo "❌ Import failed"
else
    echo "❌ No compatible wheel found for current platform"
fi
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

# Publish to PyPI using twine (uploads all wheels)
echo ""
echo "🚀 Publishing all wheels to PyPI..."
if ! command -v twine &> /dev/null; then
    echo "📦 Installing twine..."
    pip install twine
fi
twine upload --skip-existing target/wheels/* -u __token__ -p "$MATURIN_PYPI_TOKEN"

echo ""
echo "✅ Successfully published ekodb-client v$VERSION to PyPI!"
echo "📦 Users can install with: pip install ekodb-client"
echo "📚 Package page: https://pypi.org/project/ekodb-client/"
echo ""
echo "Note: It may take a few minutes for the package to be available."
