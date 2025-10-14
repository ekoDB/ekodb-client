# Makefile for ekoDB Client Libraries

# Environment variables
CARGO := cargo
CLIENT_DIR := ekodb_client
CLIENT_PY_DIR := ekodb-client-py
CLIENT_TS_DIR := ekodb-client-ts
CLIENT_GO_DIR := ekodb-client-go

# Color codes for pretty output
CYAN := \033[36m
GREEN := \033[32m
YELLOW := \033[33m
RED := \033[31m
RESET := \033[0m

.PHONY: all build build-release build-client build-python-client build-typescript-client test test-ci test-client test-examples test-examples-direct test-examples-client test-examples-rust test-examples-python test-examples-go test-examples-typescript test-examples-javascript clean check fmt fmt-rust fmt-rust-client fmt-rust-examples fmt-python fmt-go fmt-typescript fmt-md format install install-rust install-python install-typescript install-go setup install-hooks deps-check deps-update deploy-client deploy-client-rust deploy-client-py deploy-client-py-simple deploy-client-go deploy-client-ts bump-version bump-client-py docs-client

# ASCII Banner for ekoDB
BANNER := \
	\ "███████╗ ██╗  ██═╗██████╗ ██████═╗╔██████╗  " "\n" \
		"██╔════╝ ██╚██║  ██╔═══██╗██   ██║║██  ██║   " "\n" \
		"███████╗ ████═╝  ██║   ██║██    ██║███████ " "\n" \
		"██     ║ ██╔██╗  ██║   ██║██    ██║██   ██ " "\n" \
		"███████║ ██║  ██ ║██████╔╝███████║║███████ " "\n" \
		"╚══════╝ ╚═╝  ╚══╝ ╚════╝ ╚══════╝ ╚═════╝  " "\n" \

# Default target
all: build

help:
	@echo $(BANNER)
	@echo "✨ $(CYAN)Welcome to ekoDB Client Libraries ✨$(RESET)"
	@echo ""
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@echo "📌 $(CYAN)BUILD & DEVELOPMENT$(RESET)"
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@echo "  ⚙️  $(GREEN)make install$(RESET)            - Install ALL client libraries (Rust, Python, TS, Go)"
	@echo "     $(GREEN)make install-rust$(RESET)       - Install Rust client only"
	@echo "     $(GREEN)make install-python$(RESET)     - Install Python client only"
	@echo "     $(GREEN)make install-typescript$(RESET) - Install TypeScript client only"
	@echo "     $(GREEN)make install-go$(RESET)         - Install Go client only"
	@echo "  🛠️  $(GREEN)make setup$(RESET)              - Set up the project (fetch deps, create .env)"
	@echo "  🛠️  $(GREEN)make build$(RESET)              - Build ALL client libraries (Rust, Python, TS)"
	@echo "  🚀 $(GREEN)make build-release$(RESET)      - Build Rust client (release mode)"
	@echo "  🦀 $(GREEN)make build-client$(RESET)       - Build Rust client only"
	@echo "  🐍 $(GREEN)make build-python-client$(RESET) - Build Python client only"
	@echo "  📘 $(GREEN)make build-typescript-client$(RESET) - Build TypeScript client only"
	@echo "  🧪 $(GREEN)make test$(RESET)               - Run Rust client tests"
	@echo "  📚 $(GREEN)make docs$(RESET)               - Generate Rust client documentation"
	@echo "  🖌️  $(GREEN)make fmt$(RESET)                - Format all code (Rust + Python + Go + TS + Markdown)"

	@echo ""
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@echo "🧪 $(CYAN)TESTING$(RESET)"
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@echo "  🧪 $(GREEN)make test$(RESET)         - Run Rust client tests"
	@echo "  🤖 $(GREEN)make test-ci$(RESET)      - Run optimized CI tests"
	@echo "  🔗 $(GREEN)make test-examples$(RESET) - Run integration tests (all languages)"
	@echo "  🦀 $(GREEN)make test-examples-rust$(RESET) (or rs) - Run Rust examples"
	@echo "     $(GREEN)make test-examples-rs-direct$(RESET) - Rust direct HTTP/WebSocket"
	@echo "     $(GREEN)make test-examples-rs-client$(RESET) - Rust client library"
	@echo "  🐍 $(GREEN)make test-examples-python$(RESET) (or py) - Run Python examples"
	@echo "     $(GREEN)make test-examples-py-direct$(RESET) - Python direct HTTP/WebSocket"
	@echo "     $(GREEN)make test-examples-py-client$(RESET) - Python client library"
	@echo "  🔷 $(GREEN)make test-examples-go$(RESET) - Run Go examples"
	@echo "     $(GREEN)make test-examples-go-direct$(RESET) - Go direct HTTP/WebSocket"
	@echo "     $(GREEN)make test-examples-go-client$(RESET) - Go client library"
	@echo "  📘 $(GREEN)make test-examples-typescript$(RESET) (or ts) - Run TypeScript examples"
	@echo "     $(GREEN)make test-examples-ts-client$(RESET) - TypeScript client library"
	@echo "  📦 $(GREEN)make test-examples-javascript$(RESET) (or js) - Run JavaScript examples"
	@echo "     $(GREEN)make test-examples-js-direct$(RESET) - JavaScript direct HTTP/WebSocket"
	@echo "     $(GREEN)make test-examples-js-client$(RESET) - JavaScript client library"
	@echo ""
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@echo "🚀 $(CYAN)DEPLOYMENT$(RESET)"
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@echo "  🦀 $(GREEN)make deploy-client-rust$(RESET) - Deploy Rust client (uses scripts/publish-rust.sh)"
	@echo "  🐍 $(GREEN)make deploy-client-py$(RESET)   - Deploy Python client (multi-platform: Linux+macOS+source)"
	@echo "  🐍 $(GREEN)make deploy-client-py-simple$(RESET) - Deploy Python client (current platform, uses script)"
	@echo "  🔷 $(GREEN)make deploy-client-go$(RESET)   - Show Go client deployment instructions"
	@echo "  📘 $(GREEN)make deploy-client-ts$(RESET)   - Deploy TypeScript client (uses scripts/publish-typescript.sh)"
	@echo "  🔢 $(GREEN)make bump-version$(RESET)       - Bump version for ALL clients (Rust, Python, TS)"
	@echo "  🔢 $(GREEN)make bump-client-py$(RESET)     - Bump Python client version only"
	@echo ""
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@echo "📜 $(CYAN)SCRIPTS$(RESET)"
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@echo "  📝 $(GREEN)./scripts/publish-rust.sh$(RESET)       - Publish Rust client (with tests & dry-run)"
	@echo "  📝 $(GREEN)./scripts/publish-python.sh$(RESET)     - Publish Python client (current platform)"
	@echo "  📝 $(GREEN)./scripts/publish-typescript.sh$(RESET) - Publish TypeScript client"
	@echo "  📝 $(GREEN)./scripts/publish-all.sh$(RESET)        - Publish all clients"
	@echo "  📝 $(GREEN)./scripts/check-publish-ready.sh$(RESET) - Check if ready to publish"
	@echo ""
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@echo "🛠️ $(CYAN)DEVELOPMENT TOOLS$(RESET)"
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@echo "  🖌️  $(GREEN)make fmt$(RESET)          - Format all code (Rust + Python + Go + TS + Markdown)"
	@echo "  🖌️  $(GREEN)make format$(RESET)       - Format all code (alias for fmt)"
	@echo "  🦀 $(GREEN)make fmt-rust$(RESET)     - Format all Rust code (client + examples)"
	@echo "     $(GREEN)make fmt-rust-client$(RESET)   - Format ekodb_client only"
	@echo "     $(GREEN)make fmt-rust-examples$(RESET) - Format Rust examples only"
	@echo "  🐍 $(GREEN)make fmt-python$(RESET)   - Format Python code (client + examples)"
	@echo "  🔷 $(GREEN)make fmt-go$(RESET)       - Format Go code (client + examples)"
	@echo "  📘 $(GREEN)make fmt-typescript$(RESET) - Format TypeScript/JavaScript code"
	@echo "  📝 $(GREEN)make fmt-md$(RESET)       - Format Markdown files only"
	@echo ""
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@echo "⚙️  $(CYAN)ENVIRONMENT$(RESET)"
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@echo "  💡 See .env or example.env file for environment variables"

# Workspace targets
build-client:
	@echo "🛠️  $(CYAN)Building client library...$(RESET)"
	$(CARGO) build -p ekodb_client
	@echo "✅ $(GREEN)Client build complete!$(RESET)"

test-client:
	@echo "🧪 $(CYAN)Testing client library...$(RESET)"
	$(CARGO) test -p ekodb_client
	@echo "✅ $(GREEN)Client tests complete!$(RESET)"

docs-client:
	@echo "📚 $(CYAN)Generating client library documentation...$(RESET)"
	$(CARGO) doc -p ekodb_client --no-deps --open
	@echo "✅ $(GREEN)Client documentation generated!$(RESET)"

# Alias for docs-client
docs: docs-client

# Build targets - builds all client libraries
build: build-client build-python-client build-typescript-client
	@echo "✅ $(GREEN)All client libraries built!$(RESET)"

build-release:
	@echo "🚀 $(CYAN)Building release version (Rust client only)...$(RESET)"
	$(CARGO) build -p ekodb_client --release
	@echo "✅ $(GREEN)Release build complete!$(RESET)"

# Client library deployment targets
deploy-client: deploy-client-rust
	@echo "✅ $(GREEN)All client libraries deployed!$(RESET)"

# Version bumping targets
bump-version:
	@echo "🔢 $(CYAN)Bumping all client versions...$(RESET)"
	@echo ""
	@read -p "Enter new version (e.g., 0.1.3): " NEW_VERSION; \
	if [ -z "$$NEW_VERSION" ]; then \
		echo "$(RED)❌ No version provided$(RESET)"; \
		exit 1; \
	fi; \
	echo ""; \
	echo "$(YELLOW)📦 Current versions:$(RESET)"; \
	echo "  Rust:       $$(grep '^version = ' ekodb_client/Cargo.toml | head -1 | cut -d'"' -f2)"; \
	echo "  Python:     $$(grep '^version = ' ekodb-client-py/Cargo.toml | head -1 | cut -d'"' -f2)"; \
	echo "  TypeScript: $$(grep '"version":' ekodb-client-ts/package.json | head -1 | cut -d'"' -f4)"; \
	echo ""; \
	echo "$(YELLOW)📦 New version: $$NEW_VERSION$(RESET)"; \
	echo ""; \
	read -p "Continue? (y/N): " -n 1 -r; \
	echo; \
	if [[ ! $$REPLY =~ ^[Yy]$$ ]]; then \
		echo "$(RED)❌ Cancelled$(RESET)"; \
		exit 1; \
	fi; \
	echo ""; \
	echo "$(CYAN)Updating Rust client...$(RESET)"; \
	sed -i '' "s/^version = \"[^\"]*\"/version = \"$$NEW_VERSION\"/" ekodb_client/Cargo.toml; \
	echo "  ✅ ekodb_client/Cargo.toml"; \
	echo ""; \
	echo "$(CYAN)Updating Python client...$(RESET)"; \
	sed -i '' "s/^version = \"[^\"]*\"/version = \"$$NEW_VERSION\"/" ekodb-client-py/Cargo.toml; \
	sed -i '' "s/^version = \"[^\"]*\"/version = \"$$NEW_VERSION\"/" ekodb-client-py/pyproject.toml; \
	sed -i '' "s/ekodb_client = { version = \"[^\"]*\"/ekodb_client = { version = \"$$NEW_VERSION\"/" ekodb-client-py/Cargo.toml; \
	echo "  ✅ ekodb-client-py/Cargo.toml (version + ekodb_client dependency)"; \
	echo "  ✅ ekodb-client-py/pyproject.toml"; \
	echo ""; \
	echo "$(CYAN)Updating TypeScript client...$(RESET)"; \
	sed -i '' "s/\"version\": \"[^\"]*\"/\"version\": \"$$NEW_VERSION\"/" ekodb-client-ts/package.json; \
	echo "  ✅ ekodb-client-ts/package.json"; \
	echo ""; \
	echo "$(GREEN)✅ All versions bumped to $$NEW_VERSION$(RESET)"; \
	echo ""; \
	echo "$(YELLOW)💡 Next steps:$(RESET)"; \
	echo "  1. Review changes: git diff"; \
	echo "  2. Commit: git add -A && git commit -m 'Bump version to $$NEW_VERSION'"; \
	echo "  3. Publish: make deploy-client (or individual deploy commands)"

bump-client-py:
	@echo "🔢 $(CYAN)Bumping Python client version...$(RESET)"
	@CURRENT_VERSION=$$(grep '^version = ' ekodb-client-py/Cargo.toml | head -1 | cut -d'"' -f2); \
	echo "Current version: $$CURRENT_VERSION"; \
	read -p "Enter new version (e.g., 0.1.3): " NEW_VERSION; \
	if [ -z "$$NEW_VERSION" ]; then \
		echo "$(RED)❌ No version provided$(RESET)"; \
		exit 1; \
	fi; \
	echo "Updating ekodb-client-py/Cargo.toml..."; \
	sed -i '' "s/^version = \"$$CURRENT_VERSION\"/version = \"$$NEW_VERSION\"/" ekodb-client-py/Cargo.toml; \
	echo "Updating ekodb-client-py/pyproject.toml..."; \
	sed -i '' "s/^version = \"$$CURRENT_VERSION\"/version = \"$$NEW_VERSION\"/" ekodb-client-py/pyproject.toml; \
	echo "$(GREEN)✅ Version bumped to $$NEW_VERSION$(RESET)"; \
	echo "$(YELLOW)💡 Files updated:$(RESET)"; \
	echo "  - ekodb-client-py/Cargo.toml"; \
	echo "  - ekodb-client-py/pyproject.toml"

deploy-client-rust:
	@./scripts/publish-rust.sh

# Python deployment with multi-platform builds (Linux + macOS + source)
deploy-client-py:
	@echo "🐍 $(CYAN)Publishing Python client to PyPI (multi-platform)...$(RESET)"
	@if ! python3 -m pip show twine > /dev/null 2>&1; then \
		echo "📦 $(YELLOW)Installing twine...$(RESET)"; \
		python3 -m pip install twine; \
	fi
	@if [ -f .env ]; then \
		export $$(grep -v '^#' .env | xargs) && \
		if [ -z "$$MATURIN_PYPI_TOKEN" ]; then \
			echo "$(RED)❌ MATURIN_PYPI_TOKEN not found in .env$(RESET)"; \
			echo "$(YELLOW)💡 Add to .env: MATURIN_PYPI_TOKEN=pypi-your-token-here$(RESET)"; \
			exit 1; \
		fi && \
		echo "🧹 $(CYAN)Cleaning old wheels...$(RESET)" && \
		rm -rf ekodb-client-py/target/wheels/* && \
		echo "📦 $(CYAN)Building wheels for multiple platforms...$(RESET)" && \
		echo "  🐧 Building for Linux (via Docker)..." && \
		docker run --rm \
			-v $$(pwd):/workspace \
			-w /workspace/ekodb-client-py \
			ghcr.io/pyo3/maturin build --release --manylinux 2014 --no-default-features --features rustls && \
		echo "  🍎 Building for macOS (current platform)..." && \
		cd ekodb-client-py && python3 -m maturin build --release --no-default-features --features rustls --sdist && \
		echo "📤 $(CYAN)Publishing all wheels to PyPI...$(RESET)" && \
		python3 -m twine upload --skip-existing target/wheels/* -u __token__ -p "$$MATURIN_PYPI_TOKEN"; \
	else \
		if [ -z "$$MATURIN_PYPI_TOKEN" ]; then \
			echo "$(RED)❌ MATURIN_PYPI_TOKEN not set and no .env file found$(RESET)"; \
			echo "$(YELLOW)💡 Set MATURIN_PYPI_TOKEN in your environment or create .env file$(RESET)"; \
			exit 1; \
		fi; \
		echo "🧹 $(CYAN)Cleaning old wheels...$(RESET)" && \
		rm -rf ekodb-client-py/target/wheels/* && \
		echo "📦 $(CYAN)Building wheels for multiple platforms...$(RESET)" && \
		echo "  🐧 Building for Linux (via Docker)..." && \
		docker run --rm \
			-v $$(pwd):/workspace \
			-w /workspace/ekodb-client-py \
			ghcr.io/pyo3/maturin build --release --manylinux 2014 --no-default-features --features rustls && \
		echo "  🍎 Building for macOS (current platform)..." && \
		cd ekodb-client-py && python3 -m maturin build --release --no-default-features --features rustls --sdist && \
		echo "📤 $(CYAN)Publishing all wheels to PyPI...$(RESET)" && \
		python3 -m twine upload --skip-existing target/wheels/* -u __token__ -p "$$MATURIN_PYPI_TOKEN"; \
	fi
	@echo "✅ $(GREEN)Python client published!$(RESET)"
	@echo "💡 $(YELLOW)Note: Windows wheels require a Windows machine or CI/CD$(RESET)"

deploy-client-go:
	@echo "🔷 $(CYAN)Tagging Go client for pkg.go.dev...$(RESET)"
	@echo "📝 $(YELLOW)Go modules are published via git tags$(RESET)"
	@echo "💡 $(YELLOW)To publish:$(RESET)"
	@echo "   1. Commit your changes"
	@echo "   2. Run: git tag ekodb-client-go/v0.1.1"
	@echo "   3. Run: git push origin ekodb-client-go/v0.1.1"
	@echo "   4. Go will automatically index it at pkg.go.dev"
	@echo "✅ $(GREEN)Go client deployment instructions shown!$(RESET)"

# Simple Python deployment (current platform only, uses script)
deploy-client-py-simple:
	@./scripts/publish-python.sh

deploy-client-ts:
	@./scripts/publish-typescript.sh

# Test targets
test:
	@echo "🧪 $(CYAN)Running Rust client tests...$(RESET)"
	$(CARGO) test -p ekodb_client
	@echo "✅ $(GREEN)Tests complete!$(RESET)"

test-ci:
	@echo "🧪 $(CYAN)Running CI-safe tests (optimized for CI/CD pipelines)...$(RESET)"
	@echo "📦 $(CYAN)Testing ekodb_client...$(RESET)"
	$(CARGO) test -p ekodb_client --lib
	@echo "✅ $(GREEN)CI tests complete!$(RESET)"

# ============================================================================
# Integration Test Targets - Examples
# ============================================================================
#
# Example Structure:
#   - Direct API: Raw HTTP/WebSocket calls (in ttl-caching/ subdirectories)
#   - Client Library: Using language-specific client libraries
#
# Available targets:
# ============================================================================
# Example Tests
# ============================================================================
# Usage:
#   All examples:           make test-examples
#   Direct API only:        make test-examples-direct
#   Client libraries only:  make test-examples-client
#
# Per-language (both direct + client):
#   Rust:       make test-examples-rust     (or test-examples-rs)
#   Python:     make test-examples-python   (or test-examples-py)
#   Go:         make test-examples-go
#   TypeScript: make test-examples-typescript (or test-examples-ts)
#   JavaScript: make test-examples-javascript (or test-examples-js)
#
# Per-language direct or client only:
#   make test-examples-{language}-direct
#   make test-examples-{language}-client
# ============================================================================

# Run all examples (all languages, both direct and client)
test-examples: test-examples-rust test-examples-python test-examples-go test-examples-typescript test-examples-javascript
	@echo "✅ $(GREEN)All integration tests complete!$(RESET)"

# Run direct API examples (using raw HTTP/WebSocket calls)
test-examples-direct: test-examples-rust-direct test-examples-python-direct test-examples-go-direct test-examples-javascript-direct
	@echo "✅ $(GREEN)Direct API examples complete!$(RESET)"

# Run client library examples (using language-specific clients)
test-examples-client: test-examples-rust-client test-examples-python-client test-examples-go-client test-examples-typescript-client test-examples-javascript-client
	@echo "✅ $(GREEN)All client library examples complete!$(RESET)"

# ============================================================================
# Rust Examples (both direct + client)
# ============================================================================
test-examples-rust: test-examples-rust-direct test-examples-rust-client
	@echo "✅ $(GREEN)All Rust integration tests complete!$(RESET)"

test-examples-rs: test-examples-rust
test-examples-rs-direct: test-examples-rust-direct
test-examples-rs-client: test-examples-rust-client

test-examples-rust-direct:
	@echo "🧪 $(CYAN)Running Rust examples (direct HTTP/WebSocket)...$(RESET)"
	@cd examples/rust && cargo run --example simple_crud && cargo run --example simple_websocket && cargo run --example batch_operations && cargo run --example kv_operations && cargo run --example collection_management && cargo run --example document_ttl && cargo run --example websocket_ttl
	@echo "✅ $(GREEN)Rust direct examples complete!$(RESET)"

test-examples-rust-client: build-client
	@echo "🧪 $(CYAN)Running Rust client library examples...$(RESET)"
	@cd examples/rust && cargo run --example client_simple_crud
	@cd examples/rust && cargo run --example client_batch_operations
	@cd examples/rust && cargo run --example client_kv_operations
	@cd examples/rust && cargo run --example client_collection_management
	@cd examples/rust && cargo run --example client_document_ttl
	@cd examples/rust && cargo run --example client_simple_websocket
	@cd examples/rust && cargo run --example client_websocket_ttl
	@cd examples/rust && cargo run --example client_query_builder
	@cd examples/rust && cargo run --example client_search
	@cd examples/rust && cargo run --example client_schema_management
	@cd examples/rust && cargo run --example client_chat_basic
	@cd examples/rust && cargo run --example client_chat_sessions
	@cd examples/rust && cargo run --example client_chat_advanced
	@echo "✅ $(GREEN)Rust client examples complete!$(RESET)"

# ============================================================================
# Python Examples (both direct + client)
# ============================================================================
test-examples-python: test-examples-python-direct test-examples-python-client
	@echo "✅ $(GREEN)All Python integration tests complete!$(RESET)"

test-examples-py: test-examples-python
test-examples-py-direct: test-examples-python-direct
test-examples-py-client: test-examples-python-client

test-examples-python-direct:
	@echo "🧪 $(CYAN)Running Python examples (direct HTTP/WebSocket)...$(RESET)"
	@cd examples/python && python3 test_runner.py
	@echo "✅ $(GREEN)Python direct examples complete!$(RESET)"

build-python-client:
	@echo "🐍 $(CYAN)Building Python client package...$(RESET)"
	@cd ekodb-client-py && python3 -m maturin build --release
	@echo "📦 $(CYAN)Installing Python wheel for current platform...$(RESET)"
	@WHEEL=$$(ls -t ekodb-client-py/target/wheels/*.whl | grep -v manylinux | grep -v musllinux | head -1); \
	if [ -n "$$WHEEL" ]; then \
		python3 -m pip install --force-reinstall "$$WHEEL" --user; \
	else \
		echo "$(RED)No compatible wheel found for current platform$(RESET)"; \
		exit 1; \
	fi
	@echo "✅ $(GREEN)Python client package built and installed!$(RESET)"

test-examples-python-client: build-python-client
	@echo "🧪 $(CYAN)Running Python client library examples...$(RESET)"
	@cd examples/python && python3 client_simple_crud.py
	@cd examples/python && python3 client_batch_operations.py
	@cd examples/python && python3 client_kv_operations.py
	@cd examples/python && python3 client_collection_management.py
	@cd examples/python && python3 client_document_ttl.py
	@cd examples/python && python3 client_simple_websocket.py
	@cd examples/python && python3 client_websocket_ttl.py
	@cd examples/python && python3 client_query_builder.py
	@cd examples/python && python3 client_search.py
	@cd examples/python && python3 client_schema.py
	@cd examples/python && python3 client_joins.py
	@cd examples/python && python3 client_chat_basic.py
	@cd examples/python && python3 client_chat_sessions.py
	@cd examples/python && python3 client_chat_advanced.py
	@echo "✅ $(GREEN)Python client examples complete!$(RESET)"

# ============================================================================
# Go Examples (both direct + client)
# ============================================================================
test-examples-go: test-examples-go-direct test-examples-go-client
	@echo "✅ $(GREEN)All Go integration tests complete!$(RESET)"

test-examples-go-direct:
	@echo "🧪 $(CYAN)Running Go examples (direct HTTP/WebSocket)...$(RESET)"
	@cd examples/go && go run test_runner.go
	@echo "✅ $(GREEN)Go direct examples complete!$(RESET)"

test-examples-go-client:
	@echo "🧪 $(CYAN)Running Go client library examples...$(RESET)"
	@cd examples/go && go run client_simple_crud.go
	@cd examples/go && go run client_batch_operations.go
	@cd examples/go && go run client_kv_operations.go
	@cd examples/go && go run client_collection_management.go
	@cd examples/go && go run client_document_ttl.go
	@cd examples/go && go run client_simple_websocket.go
	@cd examples/go && go run client_websocket_ttl.go
	@cd examples/go && go run client_chat_basic.go
	@cd examples/go && go run client_chat_sessions.go
	@cd examples/go && go run client_chat_advanced.go
	@cd examples/go && go run client_query_builder.go
	@cd examples/go && go run client_search.go
	@cd examples/go && go run client_schema.go
	@cd examples/go && go run client_joins.go
	@echo "✅ $(GREEN)Go client examples complete!$(RESET)"

# ============================================================================
# TypeScript Examples (client only - no direct examples)
# ============================================================================
test-examples-typescript: test-examples-typescript-client
	@echo "✅ $(GREEN)All TypeScript integration tests complete!$(RESET)"

test-examples-ts: test-examples-typescript
test-examples-ts-client: test-examples-typescript-client

build-typescript-client:
	@echo "📦 $(CYAN)Building TypeScript client library...$(RESET)"
	@cd ekodb-client-ts && npm install && npm run build
	@echo "✅ $(GREEN)TypeScript client built!$(RESET)"

test-examples-typescript-client: build-typescript-client
	@echo "🧪 $(CYAN)Running TypeScript client library examples...$(RESET)"
	@cd examples/typescript && npm install
	@cd examples/typescript && npx tsx client_simple_crud.ts
	@cd examples/typescript && npx tsx client_batch_operations.ts
	@cd examples/typescript && npx tsx client_kv_operations.ts
	@cd examples/typescript && npx tsx client_collection_management.ts
	@cd examples/typescript && npx tsx client_document_ttl.ts
	@cd examples/typescript && npx tsx client_simple_websocket.ts
	@cd examples/typescript && npx tsx client_websocket_ttl.ts
	@cd examples/typescript && npx tsx client_chat_basic.ts
	@cd examples/typescript && npx tsx client_chat_sessions.ts
	@cd examples/typescript && npx tsx client_chat_advanced.ts
	@cd examples/typescript && npx tsx client_query_builder.ts
	@cd examples/typescript && npx tsx client_search.ts
	@cd examples/typescript && npx tsx client_schema.ts
	@cd examples/typescript && npx tsx client_joins.ts
	@echo "✅ $(GREEN)TypeScript client examples complete!$(RESET)"

# ============================================================================
# JavaScript Examples (both direct + client)
# ============================================================================
test-examples-javascript: test-examples-javascript-direct test-examples-javascript-client
	@echo "✅ $(GREEN)All JavaScript integration tests complete!$(RESET)"

test-examples-js: test-examples-javascript
test-examples-js-direct: test-examples-javascript-direct
test-examples-js-client: test-examples-javascript-client

test-examples-javascript-direct:
	@echo "🧪 $(CYAN)Running JavaScript examples (direct HTTP/WebSocket)...$(RESET)"
	@cd examples/javascript && npm install && node test-runner.js
	@echo "✅ $(GREEN)JavaScript direct examples complete!$(RESET)"

test-examples-javascript-client: build-typescript-client
	@echo "🧪 $(CYAN)Running JavaScript client library examples...$(RESET)"
	@cd examples/javascript && npm install
	@cd examples/javascript && node client_simple_crud.js
	@cd examples/javascript && node client_batch_operations.js
	@cd examples/javascript && node client_kv_operations.js
	@cd examples/javascript && node client_collection_management.js
	@cd examples/javascript && node client_document_ttl.js
	@cd examples/javascript && node client_simple_websocket.js
	@cd examples/javascript && node client_websocket_ttl.js
	@echo "✅ $(GREEN)JavaScript client examples complete!$(RESET)"

# Clean targets
clean:
	@echo "🧹 $(YELLOW)Cleaning build artifacts...$(RESET)"
	$(CARGO) clean
	@echo "✅ $(GREEN)Clean complete!$(RESET)"

# Development targets
check:
	@echo "📋 $(CYAN)Running cargo check...$(RESET)"
	$(CARGO) check
	@echo "✅ $(GREEN)Check complete!$(RESET)"

# Format all code (Rust + Python + Go + TypeScript + Markdown) - this is the main formatting target
fmt: fmt-rust fmt-python fmt-go fmt-typescript fmt-md
	@echo "✅ $(GREEN)All formatting complete!$(RESET)"

# Format all Rust code (client + examples)
fmt-rust: fmt-rust-client fmt-rust-examples
	@echo "✅ $(GREEN)All Rust formatting complete!$(RESET)"

# Format individual Rust projects
fmt-rust-client:
	@echo "🖌️  $(CYAN)Formatting ekodb_client...$(RESET)"
	@cd ekodb_client && cargo fmt
	@echo "✅ $(GREEN)ekodb_client formatted!$(RESET)"

fmt-rust-examples:
	@echo "🖌️  $(CYAN)Formatting Rust examples...$(RESET)"
	@cd examples/rust && cargo fmt
	@echo "✅ $(GREEN)Rust examples formatted!$(RESET)"

# Format Python code (client + examples)
fmt-python:
	@echo "🐍 $(CYAN)Formatting Python code...$(RESET)"
	@if command -v black > /dev/null; then \
		black ekodb-client-py/ examples/python/ --line-length 88 --exclude 'target|venv|\.venv|test_env'; \
		echo "✅ $(GREEN)Python formatting complete with black!$(RESET)"; \
	elif command -v autopep8 > /dev/null; then \
		find ekodb-client-py examples/python -name "*.py" -not -path "*/target/*" -not -path "*/venv/*" -not -path "*/.venv/*" -exec autopep8 --in-place --aggressive --aggressive {} \; 2>/dev/null || true; \
		echo "✅ $(GREEN)Python formatting complete with autopep8!$(RESET)"; \
	else \
		echo "$(YELLOW)No Python formatter found. Installing black...$(RESET)"; \
		python3 -m pip install black --user; \
		black ekodb-client-py/ examples/python/ --line-length 88 --exclude 'target|venv|\.venv|test_env'; \
		echo "✅ $(GREEN)black installed and Python formatting complete!$(RESET)"; \
	fi

# Format Go code (client + examples)
fmt-go:
	@echo "🔷 $(CYAN)Formatting Go code...$(RESET)"
	@if command -v gofmt > /dev/null; then \
		gofmt -w ekodb-client-go/ examples/go/; \
		echo "✅ $(GREEN)Go formatting complete with gofmt!$(RESET)"; \
	else \
		echo "$(YELLOW)gofmt not found. Please install Go.$(RESET)"; \
	fi

# Format TypeScript/JavaScript code (client + examples)
fmt-typescript:
	@echo "📘 $(CYAN)Formatting TypeScript/JavaScript code...$(RESET)"
	@if command -v prettier > /dev/null; then \
		prettier --write "ekodb-client-ts/**/*.{ts,js,json}" "examples/typescript/**/*.{ts,js,json}" --ignore-path .gitignore; \
		echo "✅ $(GREEN)TypeScript/JavaScript formatting complete!$(RESET)"; \
	else \
		echo "$(YELLOW)prettier not found. Installing...$(RESET)"; \
		if command -v npm > /dev/null; then \
			npm install -g prettier; \
			prettier --write "ekodb-client-ts/**/*.{ts,js,json}" "examples/typescript/**/*.{ts,js,json}" --ignore-path .gitignore; \
			echo "✅ $(GREEN)prettier installed and TypeScript formatting complete!$(RESET)"; \
		else \
			echo "$(RED)npm not found. Please install Node.js and npm$(RESET)"; \
		fi; \
	fi

fmt-md:
	@echo "📝 $(CYAN)Formatting Markdown files...$(RESET)"
	@if command -v prettier > /dev/null; then \
		prettier --write "**/*.md" --prose-wrap always --print-width 80; \
		echo "✅ $(GREEN)Markdown formatting complete!$(RESET)"; \
	elif command -v markdownlint > /dev/null; then \
		find . -name "*.md" -not -path "./target/*" -not -path "./node_modules/*" -exec markdownlint --fix {} \; 2>/dev/null || true; \
		echo "✅ $(GREEN)Markdown formatting complete with markdownlint!$(RESET)"; \
	else \
		echo "$(YELLOW)No markdown formatter found. Installing prettier...$(RESET)"; \
		if command -v npm > /dev/null; then \
			npm install -g prettier; \
			prettier --write "**/*.md" --prose-wrap always --print-width 80; \
			echo "✅ $(GREEN)Prettier installed and markdown formatting complete!$(RESET)"; \
		else \
			echo "$(RED)npm not found. Please install Node.js and npm, or install prettier/markdownlint manually$(RESET)"; \
			echo "$(YELLOW)Alternative: Install with 'npm install -g prettier' or 'npm install -g markdownlint-cli'$(RESET)"; \
		fi; \
	fi

# Alias for fmt - formats everything (Rust + Markdown + examples)
format: fmt


# Install all client libraries
install: install-rust install-python install-typescript install-go
	@echo "🔧 $(CYAN)Making scripts executable...$(RESET)"
	@chmod +x scripts/*.sh
	@echo "✅ $(GREEN)All client libraries installed!$(RESET)"

# Install individual client libraries
install-rust:
	@echo "🦀 $(CYAN)Installing Rust client...$(RESET)"
	@cd ekodb_client && cargo build --release
	@echo "✅ $(GREEN)Rust client installed!$(RESET)"

install-python:
	@echo "🐍 $(CYAN)Installing Python client...$(RESET)"
	@cd ekodb-client-py && python3 -m maturin build --release
	@echo "📦 $(CYAN)Installing Python wheel for current platform...$(RESET)"
	@WHEEL=$$(ls -t ekodb-client-py/target/wheels/*.whl | grep -v manylinux | grep -v musllinux | head -1); \
	if [ -n "$$WHEEL" ]; then \
		python3 -m pip install --force-reinstall "$$WHEEL" --user; \
	else \
		echo "$(RED)No compatible wheel found for current platform$(RESET)"; \
		exit 1; \
	fi
	@echo "✅ $(GREEN)Python client installed!$(RESET)"

install-typescript:
	@echo "📘 $(CYAN)Installing TypeScript client...$(RESET)"
	@cd ekodb-client-ts && npm install && npm run build
	@echo "✅ $(GREEN)TypeScript client installed!$(RESET)"

install-go:
	@echo "🔷 $(CYAN)Installing Go client dependencies...$(RESET)"
	@cd ekodb-client-go && go mod download
	@echo "✅ $(GREEN)Go client dependencies installed!$(RESET)"

setup:
	@echo "🛠️  $(CYAN)Setting up the project...$(RESET)"
	cargo fetch
	cargo check
	@chmod +x scripts/*.sh
	@mkdir -p logs
	@if [ ! -f ".env" ] && [ -f ".env.example" ]; then \
		cp .env.example .env; \
		echo "$(YELLOW)Created .env from .env.example$(RESET)"; \
	fi
	@echo "✅ $(GREEN)Setup complete!$(RESET)"

install-hooks:
	@echo "🪝 $(CYAN)Installing Git hooks...$(RESET)"
	cp scripts/pre-commit .git/hooks/pre-commit
	chmod +x .git/hooks/pre-commit
	@echo "✅ $(GREEN)Git hooks installed!$(RESET)"

# Check for outdated dependencies
deps-check:
	@echo "📦 $(CYAN)Checking for outdated dependencies...$(RESET)"
	@if command -v cargo-outdated > /dev/null; then \
		cargo outdated; \
	else \
		echo "$(YELLOW)cargo-outdated is not installed.$(RESET)"; \
		echo "$(YELLOW)Run 'cargo install cargo-outdated' to install it.$(RESET)"; \
	fi
	@echo "✅ $(GREEN)Dependencies check complete!$(RESET)"

# Update dependencies within Cargo.toml constraints
deps-update:
	@echo "📦 $(CYAN)Updating dependencies within constraints...$(RESET)"
	$(CARGO) update
	@echo "✅ $(GREEN)Dependencies updated successfully!$(RESET)"
	@echo "💡 $(YELLOW)Run 'make deps-check' to see if any dependencies still need updating$(RESET)"
