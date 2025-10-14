#!/bin/bash
set -e

echo "🚀 ekoDB Client Package Publisher"
echo "=================================="
echo ""

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# Load .env file if it exists
if [ -f "$REPO_ROOT/.env" ]; then
    echo "🔐 Loading environment variables from .env..."
    set -a  # Automatically export all variables
    source "$REPO_ROOT/.env"
    set +a
    echo "✅ Environment variables loaded"
    echo ""
fi

# Function to show usage
show_usage() {
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  --all           Publish all client packages (Rust, Python, TypeScript)"
    echo "  --rust          Publish Rust client to crates.io"
    echo "  --python        Publish Python client to PyPI"
    echo "  --typescript    Publish TypeScript client to npm"
    echo "  --help          Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0 --all                    # Publish all packages"
    echo "  $0 --rust --python          # Publish only Rust and Python"
    echo "  $0 --typescript             # Publish only TypeScript"
    echo ""
    echo "Note: Go client has its own publish script in ekodb-client-go/publish.sh"
    echo "      (Go client will be moved to a separate repository)"
    echo ""
}

# Parse command line arguments
PUBLISH_RUST=false
PUBLISH_PYTHON=false
PUBLISH_TYPESCRIPT=false
PUBLISH_ALL=false

if [ $# -eq 0 ]; then
    show_usage
    exit 0
fi

while [[ $# -gt 0 ]]; do
    case $1 in
        --all)
            PUBLISH_ALL=true
            shift
            ;;
        --rust)
            PUBLISH_RUST=true
            shift
            ;;
        --python)
            PUBLISH_PYTHON=true
            shift
            ;;
        --typescript)
            PUBLISH_TYPESCRIPT=true
            shift
            ;;
        --help)
            show_usage
            exit 0
            ;;
        *)
            echo "❌ Unknown option: $1"
            show_usage
            exit 1
            ;;
    esac
done

# If --all is specified, enable all packages
if [ "$PUBLISH_ALL" = true ]; then
    PUBLISH_RUST=true
    PUBLISH_PYTHON=true
    PUBLISH_TYPESCRIPT=true
fi

# Track success/failure
FAILED_PACKAGES=()

# Publish Rust
if [ "$PUBLISH_RUST" = true ]; then
    echo ""
    echo "════════════════════════════════════════"
    echo "Publishing Rust Client"
    echo "════════════════════════════════════════"
    if bash "$SCRIPT_DIR/publish-rust.sh"; then
        echo "✅ Rust client published successfully"
    else
        echo "❌ Rust client publication failed"
        FAILED_PACKAGES+=("Rust")
    fi
fi

# Publish Python
if [ "$PUBLISH_PYTHON" = true ]; then
    echo ""
    echo "════════════════════════════════════════"
    echo "Publishing Python Client"
    echo "════════════════════════════════════════"
    if bash "$SCRIPT_DIR/publish-python.sh"; then
        echo "✅ Python client published successfully"
    else
        echo "❌ Python client publication failed"
        FAILED_PACKAGES+=("Python")
    fi
fi

# Publish TypeScript
if [ "$PUBLISH_TYPESCRIPT" = true ]; then
    echo ""
    echo "════════════════════════════════════════"
    echo "Publishing TypeScript Client"
    echo "════════════════════════════════════════"
    if bash "$SCRIPT_DIR/publish-typescript.sh"; then
        echo "✅ TypeScript client published successfully"
    else
        echo "❌ TypeScript client publication failed"
        FAILED_PACKAGES+=("TypeScript")
    fi
fi

# Summary
echo ""
echo "════════════════════════════════════════"
echo "Publication Summary"
echo "════════════════════════════════════════"

if [ ${#FAILED_PACKAGES[@]} -eq 0 ]; then
    echo "✅ All packages published successfully!"
else
    echo "❌ Some packages failed to publish:"
    for pkg in "${FAILED_PACKAGES[@]}"; do
        echo "  - $pkg"
    done
    exit 1
fi

echo ""
echo "📦 Installation Commands:"
echo "  Rust:       cargo add ekodb_client"
echo "  Python:     pip install ekodb-client"
echo "  TypeScript: npm install @ekodb/ekodb-client"
echo ""
echo "Note: Go client will be published separately from ekodb-client-go repository"
