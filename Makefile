# Makefile for ekoDB Client Libraries

# Environment variables
CARGO := cargo
CLIENT_DIR := ekodb_client
CLIENT_PY_DIR := ekodb-client-py
CLIENT_TS_DIR := ekodb-client-ts
CLIENT_GO_DIR := ekodb-client-go
CLIENT_KT_DIR := ekodb-client-kt

# Color codes for pretty output
CYAN := \033[36m
GREEN := \033[32m
YELLOW := \033[33m
RED := \033[31m
RESET := \033[0m

.PHONY: all build build-release build-client build-python-client build-typescript-client build-examples test test-ci test-client test-examples test-examples-direct test-examples-client test-examples-rust test-examples-python test-examples-go test-examples-typescript test-examples-javascript test-examples-transactions test-examples-scripts test-examples-scripts-crud test-examples-swr test-examples-ts-swr test-examples-py-swr test-examples-go-swr test-examples-rust-swr test-examples-kt-swr clean check fmt fmt-rust fmt-rust-client fmt-rust-examples fmt-python fmt-go fmt-typescript fmt-md format install install-rust install-python install-typescript install-go setup install-hooks deps-check deps-update deploy-client deploy-client-rust deploy-client-py deploy-client-py-simple deploy-client-go deploy-client-ts bump-version bump-client-py docs-client

# ASCII Banner for ekoDB
BANNER := \
	\ "███████╗ ██╗  ██═╗██████╗ ██████═╗╔██████╗  " "\n" \
		"██╔════╝ ██╚██║  ██╔═══██╗██   ██║║██  ██║   " "\n" \
		"███████╗ ████═╝  ██║   ██║██    ██║███████ " "\n" \
		"██     ║ ██╔██╗  ██║   ██║██    ██║██   ██ " "\n" \
		"███████║ ██║  ██ ║██████╔╝███████║║███████ " "\n" \
		"╚══════╝ ╚═╝  ╚══╝ ╚════╝ ╚══════╝ ╚═════╝  " "\n"

# Language Sub-Banner
LANGUAGES := \
	"         🦀 Rust  •  🐍 Python  •  📘 TypeScript  •  🟣 Kotlin" "\n"

# Default target
all: build

help:
	@echo $(BANNER)
	@echo $(LANGUAGES)
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
	@echo "  🛠️  $(GREEN)make build$(RESET)              - Build ALL client libraries (Rust, Python, TS, Kotlin)"
	@echo "  🚀 $(GREEN)make build-release$(RESET)      - Build Rust client (release mode)"
	@echo "  🦀 $(GREEN)make build-client$(RESET)       - Build Rust client only"
	@echo "  🐍 $(GREEN)make build-python-client$(RESET) - Build Python client only"
	@echo "  📘 $(GREEN)make build-typescript-client$(RESET) - Build TypeScript client only"
	@echo "  🟣 $(GREEN)make build-kotlin-client$(RESET) - Build Kotlin client only"
	@echo "  🧪 $(GREEN)make test$(RESET)               - Run Rust client tests"
	@echo "  📚 $(GREEN)make docs$(RESET)               - Generate Rust client documentation"
	@echo "  🖌️  $(GREEN)make fmt$(RESET)                - Format all code (Rust + Python + Go + TS + Markdown)"

	@echo ""
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@echo "🧪 $(CYAN)UNIT TESTING$(RESET)"
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@echo "  🧪 $(GREEN)make test$(RESET)           - Run ALL unit tests (Rust+TS+Python+Kotlin = 527 tests)"
	@echo "  🦀 $(GREEN)make test-rust$(RESET)      - Run Rust client tests (253 tests)"
	@echo "  📘 $(GREEN)make test-typescript$(RESET) - Run TypeScript client tests (108 tests)"
	@echo "  🐍 $(GREEN)make test-python$(RESET)    - Run Python client tests (66 tests)"
	@echo "  🟣 $(GREEN)make test-kotlin$(RESET)    - Run Kotlin client tests (100 tests)"
	@echo "  🤖 $(GREEN)make test-ci$(RESET)        - Run optimized CI tests"
	@echo ""
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@echo "🔗 $(CYAN)INTEGRATION TESTING$(RESET)"
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@echo "  🔗 $(GREEN)make test-examples$(RESET) - Run ALL integration tests (includes transactions)"
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
	@echo "  💳 $(GREEN)make test-examples-transactions$(RESET) - Run transaction examples (Python, Go, JS, Rust, Kotlin)"
	@echo "  📜 $(GREEN)make test-examples-scripts$(RESET) - Run all Scripts/Functions examples (http_functions + crud_scripts)"
	@echo "  📚 $(GREEN)make test-examples-scripts-crud$(RESET) - Run CRUD Scripts examples only (all languages)"
	@echo "  🤖 $(GREEN)make test-examples-rag$(RESET) - Run RAG Conversation System examples (Rust, Python, TypeScript)"
	@echo "  🌐 $(GREEN)make test-examples-swr$(RESET) - Run SWR (Stale-While-Revalidate) edge cache examples (all languages)"
	@echo "     $(GREEN)make test-examples-ts-swr$(RESET) - TypeScript SWR examples only"
	@echo "     $(GREEN)make test-examples-py-swr$(RESET) - Python SWR examples only"
	@echo "     $(GREEN)make test-examples-go-swr$(RESET) - Go SWR examples only"
	@echo "     $(GREEN)make test-examples-rust-swr$(RESET) - Rust SWR examples only"
	@echo "     $(GREEN)make test-examples-kt-swr$(RESET) - Kotlin SWR examples only"
	@echo ""
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@echo "🚀 $(CYAN)DEPLOYMENT$(RESET)"
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@echo "  🚀 $(GREEN)make deploy-all$(RESET)         - Deploy ALL clients (Rust + Python + TS + Kotlin)"
	@echo "  🦀 $(GREEN)make deploy-client-rust$(RESET) - Deploy Rust client (uses scripts/publish-rust.sh)"
	@echo "  🐍 $(GREEN)make deploy-client-py$(RESET)   - Deploy Python client (multi-platform: Linux+macOS+source)"
	@echo "  🐍 $(GREEN)make deploy-client-py-simple$(RESET) - Deploy Python client (current platform, uses script)"
	@echo "  📘 $(GREEN)make deploy-client-ts$(RESET)   - Deploy TypeScript client (uses scripts/publish-typescript.sh)"
	@echo "  🟣 $(GREEN)make deploy-client-kt$(RESET)   - Deploy Kotlin client to Maven Central (uses scripts/publish-kotlin.sh)"
	@echo "  🔷 $(GREEN)make deploy-client-go$(RESET)   - Show Go client deployment instructions"
	@echo "  🔢 $(GREEN)make bump-version$(RESET)       - Bump version for ALL clients (Rust + Python + TS + Kotlin)"
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
	@echo "  📦 $(GREEN)make deps-check-all$(RESET) - Check for outdated dependencies (Rust + TS + Kotlin)"
	@echo "  📦 $(GREEN)make deps-update-all$(RESET) - Update all dependencies within constraints"
	@echo "     $(GREEN)make deps-check$(RESET)       - Check Rust dependencies only"
	@echo "     $(GREEN)make deps-update$(RESET)      - Update Rust dependencies only"
	@echo "     $(GREEN)make deps-check-rust$(RESET)  - Detailed Rust dependency check"
	@echo "     $(GREEN)make deps-check-typescript$(RESET) - Check TypeScript/npm dependencies"
	@echo "     $(GREEN)make deps-check-kotlin$(RESET) - Check Kotlin/Gradle dependencies"
	@echo "  📋 $(GREEN)make examples-ls$(RESET)  - Generate comprehensive examples inventory"
	@echo "  🔍 $(GREEN)make examples-ls-check$(RESET) - Validate examples inventory against committed snapshot"
	@echo "  🔄 $(GREEN)make examples-ls-badge$(RESET) - Update README badge with current example count"
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
build: build-client build-python-client build-typescript-client build-kotlin-client
	@echo "✅ $(GREEN)All client libraries built!$(RESET)"

build-release:
	@echo "🚀 $(CYAN)Building release version (Rust client only)...$(RESET)"
	$(CARGO) build -p ekodb_client --release
	@echo "✅ $(GREEN)Release build complete!$(RESET)"

# Build all examples across all languages
build-examples:
	@echo "🔨 $(CYAN)Building ALL examples...$(RESET)"
	@echo "🦀 $(CYAN)Building Rust examples...$(RESET)"
	@cd examples/rust && cargo build --examples
	@echo "✅ Rust examples built"
	@echo "📘 $(CYAN)Building TypeScript examples...$(RESET)"
	@cd examples/typescript && npm install && npm run build
	@echo "✅ TypeScript examples built"
	@echo "🔷 $(CYAN)Checking Go examples...$(RESET)"
	@cd examples/go && go build ./...
	@echo "✅ Go examples verified"
	@echo "✅ $(GREEN)All examples built successfully!$(RESET)"

# Client library deployment targets
deploy-client: deploy-client-rust
	@echo "✅ $(GREEN)All client libraries deployed!$(RESET)"

deploy-all: ## Deploy all client libraries (Rust, Python, TypeScript, Kotlin)
	@echo "🚀 $(CYAN)Deploying all client libraries...$(RESET)"
	@./scripts/publish-all.sh --all
	@echo "✅ $(GREEN)All client libraries deployed!$(RESET)"

# Version bumping targets
bump-version:
	@echo "🔢 $(CYAN)Bumping all client versions...$(RESET)"
	@RUST_VERSION=$$(grep '^version = ' ekodb_client/Cargo.toml | head -1 | cut -d'"' -f2); \
	PYTHON_VERSION=$$(grep '^version = ' ekodb-client-py/Cargo.toml | head -1 | cut -d'"' -f2); \
	TS_VERSION=$$(grep '"version":' ekodb-client-ts/package.json | head -1 | cut -d'"' -f4); \
	KOTLIN_VERSION=$$(grep '^version = ' ekodb-client-kt/build.gradle.kts | head -1 | cut -d'"' -f2); \
	echo ""; \
	echo "$(YELLOW)📦 Current versions:$(RESET)"; \
	echo "  Rust:       $$RUST_VERSION"; \
	echo "  Python:     $$PYTHON_VERSION"; \
	echo "  TypeScript: $$TS_VERSION"; \
	echo "  Kotlin:     $$KOTLIN_VERSION"; \
	echo ""; \
	read -p "Enter new version (e.g. current: '$$RUST_VERSION'): " NEW_VERSION; \
	if [ -z "$$NEW_VERSION" ]; then \
		echo "$(RED)❌ No version provided$(RESET)"; \
		exit 1; \
	fi; \
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
	echo "$(CYAN)Updating Kotlin client...$(RESET)"; \
	sed -i '' "s/^version = \"[^\"]*\"/version = \"$$NEW_VERSION\"/" ekodb-client-kt/build.gradle.kts; \
	echo "  ✅ ekodb-client-kt/build.gradle.kts"; \
	echo ""; \
	echo "$(GREEN)✅ All versions bumped to $$NEW_VERSION$(RESET)"; \
	echo ""; \
	echo "$(CYAN)Running make fmt...$(RESET)"; \
	$(MAKE) fmt; \
	echo ""; \
	echo "$(CYAN)Running make build...$(RESET)"; \
	$(MAKE) build; \
	echo ""; \
	echo "$(GREEN)✅ Format and build complete!$(RESET)"; \
	echo ""; \
	echo "$(YELLOW)💡 Next steps:$(RESET)"; \
	echo "  1. Review changes: git diff"; \
	echo "  2. Commit: git add -A && git commit -m 'Bump version to $$NEW_VERSION'"; \
	echo "  3. Publish: make deploy-all (or individual deploy commands)"

bump-client-py:
	@echo "🔢 $(CYAN)Bumping Python client version...$(RESET)"
	@CURRENT_VERSION=$$(grep '^version = ' ekodb-client-py/Cargo.toml | head -1 | cut -d'"' -f2); \
	echo ""; \
	read -p "Enter new version (e.g. current version: '$$CURRENT_VERSION'): " NEW_VERSION; \
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
			ghcr.io/pyo3/maturin build --release --manylinux 2_28 --no-default-features --features rustls && \
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
			ghcr.io/pyo3/maturin build --release --manylinux 2_28 --no-default-features --features rustls && \
		echo "  🍎 Building for macOS (current platform)..." && \
		cd ekodb-client-py && python3 -m maturin build --release --no-default-features --features rustls --sdist && \
		echo "📤 $(CYAN)Publishing all wheels to PyPI...$(RESET)" && \
		python3 -m twine upload --skip-existing target/wheels/* -u __token__ -p "$$MATURIN_PYPI_TOKEN"; \
	fi
	@echo "✅ $(GREEN)Python client published!$(RESET)"
	@echo "💡 $(YELLOW)Note: Windows wheels require a Windows machine or CI/CD$(RESET)"

# Simple Python deployment (current platform only, uses script)
deploy-client-py-simple:
	@./scripts/publish-python.sh

deploy-client-ts:
	@./scripts/publish-typescript.sh

deploy-client-kt:
	@echo "🟣 $(CYAN)Publishing Kotlin client to Maven Central...$(RESET)"
	@chmod +x scripts/publish-kotlin.sh
	@./scripts/publish-kotlin.sh

deploy-client-kotlin: deploy-client-kt

# Test targets - runs ALL unit tests across all client libraries
test:
	@RUST_COUNT=0; TS_COUNT=0; PY_COUNT=0; KT_COUNT=0; \
	echo "🦀 $(CYAN)Running Rust client tests...$(RESET)"; \
	RUST_OUTPUT=$$($(CARGO) test -p ekodb_client 2>&1); \
	echo "$$RUST_OUTPUT"; \
	RUST_COUNT=$$(echo "$$RUST_OUTPUT" | grep -E "^test result:" | grep -oE "[0-9]+ passed" | awk '{sum+=$$1} END {print sum}'); \
	echo "✅ $(GREEN)Rust tests complete!$(RESET)"; \
	echo "📘 $(CYAN)Running TypeScript client tests...$(RESET)"; \
	TS_OUTPUT=$$(cd $(CLIENT_TS_DIR) && npm test 2>&1); \
	echo "$$TS_OUTPUT"; \
	TS_COUNT=$$(echo "$$TS_OUTPUT" | grep -oE "Tests\s+[0-9]+ passed" | grep -oE "[0-9]+"); \
	echo "✅ $(GREEN)TypeScript tests complete!$(RESET)"; \
	echo "🐍 $(CYAN)Running Python client tests...$(RESET)"; \
	PY_OUTPUT=$$(cd $(CLIENT_PY_DIR) && python3 -m pytest tests/ -v 2>&1); \
	echo "$$PY_OUTPUT"; \
	PY_COUNT=$$(echo "$$PY_OUTPUT" | grep -oE "[0-9]+ passed" | grep -oE "[0-9]+"); \
	echo "✅ $(GREEN)Python tests complete!$(RESET)"; \
	echo "🟣 $(CYAN)Running Kotlin client tests...$(RESET)"; \
	(cd $(CLIENT_KT_DIR) && ./gradlew test --quiet); \
	KT_COUNT=$$(grep -rh "@Test" ./$(CLIENT_KT_DIR)/src/test --include="*.kt" 2>/dev/null | wc -l | tr -d ' '); \
	echo "✅ $(GREEN)Kotlin tests complete!$(RESET)"; \
	echo ""; \
	echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"; \
	echo "✅ $(GREEN)All unit tests complete!$(RESET)"; \
	echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"; \
	RUST_COUNT=$${RUST_COUNT:-0}; TS_COUNT=$${TS_COUNT:-0}; PY_COUNT=$${PY_COUNT:-0}; KT_COUNT=$${KT_COUNT:-0}; \
	TOTAL=$$((RUST_COUNT + TS_COUNT + PY_COUNT + KT_COUNT)); \
	printf "  🦀 Rust:       %3d tests\n" "$$RUST_COUNT"; \
	printf "  📘 TypeScript: %3d tests\n" "$$TS_COUNT"; \
	printf "  🐍 Python:     %3d tests\n" "$$PY_COUNT"; \
	printf "  🟣 Kotlin:     %3d tests\n" "$$KT_COUNT"; \
	echo "  ─────────────────────"; \
	printf "  📊 Total:      %3d tests\n" "$$TOTAL"; \
	echo ""

test-rust:
	@echo "🦀 $(CYAN)Running Rust client tests...$(RESET)"
	@$(CARGO) test -p ekodb_client
	@echo "✅ $(GREEN)Rust tests complete!$(RESET)"

test-typescript:
	@echo "📘 $(CYAN)Running TypeScript client tests...$(RESET)"
	@cd $(CLIENT_TS_DIR) && npm test
	@echo "✅ $(GREEN)TypeScript tests complete!$(RESET)"

test-python:
	@echo "🐍 $(CYAN)Running Python client tests...$(RESET)"
	@cd $(CLIENT_PY_DIR) && python3 -m pytest tests/ -v
	@echo "✅ $(GREEN)Python tests complete!$(RESET)"

test-kotlin:
	@echo "🟣 $(CYAN)Running Kotlin client tests...$(RESET)"
	@cd $(CLIENT_KT_DIR) && ./gradlew test --quiet
	@echo "✅ $(GREEN)Kotlin tests complete!$(RESET)"

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
#   - Direct API: Raw HTTP/WebSocket calls
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

# Run all examples (all languages, both direct and client, including transactions)
test-examples: examples-ls-check
	@echo "make test-examples" > examples/test-examples.md
	@$(MAKE) test-examples-rust test-examples-python test-examples-go test-examples-typescript test-examples-javascript test-examples-kotlin test-examples-rag test-examples-swr test-examples-fcomp 2>&1 | tee -a examples/test-examples.md
	@echo "✅ $(GREEN)All integration tests complete!$(RESET)"

# Run direct API examples (using raw HTTP/WebSocket calls, including transactions)
test-examples-direct: test-examples-rust-direct test-examples-python-direct test-examples-go-direct test-examples-javascript-direct
	@echo "✅ $(GREEN)Direct API examples complete!$(RESET)"

# Run client library examples (using language-specific clients)
test-examples-client: test-examples-rust-client test-examples-python-client test-examples-go-client test-examples-typescript-client test-examples-javascript-client
	@echo "✅ $(GREEN)All client library examples complete!$(RESET)"

# Run transaction examples (all languages with direct API support)
test-examples-transactions:
	@echo ""
	@echo "💳 $(CYAN)Running Transaction Examples (Direct API)...$(RESET)"
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@echo ""
	@echo "🐍 $(YELLOW)Python Transactions...$(RESET)"
	@cd examples/python && python3 transactions.py
	@echo ""
	@echo "🔷 $(YELLOW)Go Transactions...$(RESET)"
	@cd examples/go && go run transactions.go
	@echo ""
	@echo "� $(YELLOW)JavaScript Transactions...$(RESET)"
	@cd examples/javascript && node transactions.js
	@echo ""
	@echo "🦀 $(YELLOW)Rust Transactions...$(RESET)"
	@cd examples/rust && cargo run --example transactions --quiet
	@echo ""
	@echo "🟣 $(YELLOW)Kotlin Transactions...$(RESET)"
	@if [ -f .env ]; then . ./.env; fi && \
		export JAVA_HOME=$$(/usr/libexec/java_home -v 17) && export PATH=$$JAVA_HOME/bin:$$PATH && \
		cd examples/kotlin && API_BASE_URL=$$API_BASE_URL API_BASE_KEY=$$API_BASE_KEY ./gradlew run -PmainClass="io.ekodb.client.examples.ClientTransactionsKt" --quiet
	@echo ""
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@echo "✅ $(GREEN)All transaction examples completed successfully!$(RESET)"

# ============================================================================
# Scripts/Functions Examples (http_functions + crud_scripts)
# ============================================================================
test-examples-scripts:
	@echo ""
	@echo "📜 $(CYAN)Running Scripts/Functions Examples (Direct API)...$(RESET)"
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@echo ""
	@echo "🐍 $(YELLOW)Python Scripts (http_functions + crud_scripts)...$(RESET)"
	@cd examples/python && python3 http_functions.py && python3 crud_scripts.py
	@echo ""
	@echo "🔷 $(YELLOW)Go Scripts (http_functions + crud_scripts)...$(RESET)"
	@cd examples/go && go run http_functions.go && go run crud_scripts.go
	@echo ""
	@echo "📦 $(YELLOW)JavaScript Scripts (http_functions + crud_scripts)...$(RESET)"
	@cd examples/javascript && node http_functions.js && node crud_scripts.js
	@echo ""
	@echo "🦀 $(YELLOW)Rust Scripts (http_functions + crud_scripts)...$(RESET)"
	@cd examples/rust && cargo run --example http_functions --quiet && cargo run --example crud_scripts --quiet
	@echo ""
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@echo "✅ $(GREEN)All Scripts/Functions examples completed successfully!$(RESET)"

# Run only CRUD Scripts examples
test-examples-scripts-crud:
	@echo ""
	@echo "📚 $(CYAN)Running CRUD Scripts Examples (Direct API)...$(RESET)"
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@echo ""
	@echo "🐍 $(YELLOW)Python CRUD Scripts...$(RESET)"
	@cd examples/python && python3 crud_scripts.py
	@echo ""
	@echo "🔷 $(YELLOW)Go CRUD Scripts...$(RESET)"
	@cd examples/go && go run crud_scripts.go
	@echo ""
	@echo "📦 $(YELLOW)JavaScript CRUD Scripts...$(RESET)"
	@cd examples/javascript && node crud_scripts.js
	@echo ""
	@echo "🦀 $(YELLOW)Rust CRUD Scripts...$(RESET)"
	@cd examples/rust && cargo run --example crud_scripts --quiet
	@echo ""
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@echo "✅ $(GREEN)All CRUD Scripts examples completed successfully!$(RESET)"

# ============================================================================
# RAG Conversation System Examples
# ============================================================================
test-examples-rag:
	@echo "make test-examples-rag" > test-examples-rag.md
	@$(MAKE) run-rag-examples 2>&1 | tee -a test-examples-rag.md
	@echo "✅ $(GREEN)All RAG examples complete! Output saved to test-examples-rag.md$(RESET)"

run-rag-examples:
	@echo ""
	@echo "🤖 $(CYAN)RAG Conversation System Examples$(RESET)"
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@echo ""
	@echo "$(YELLOW)Prerequisites:$(RESET)"
	@echo "  1. ekoDB server running (make run in ekodb/)"
	@echo "  2. OPENAI_API_KEY set in server environment"
	@echo "  3. API_BASE_URL and API_BASE_KEY exported in your shell"
	@echo ""
	@echo "$(CYAN)Building Rust client library...$(RESET)"
	@cd ekodb_client && cargo build --release --quiet
	@echo "✓ Rust client built"
	@echo ""
	@echo "$(CYAN)Building Python client bindings...$(RESET)"
	@cd ekodb-client-py && maturin build --release --quiet
	@pip3 install --force-reinstall --quiet $$(ls -t ekodb-client-py/target/wheels/ekodb_client-*.whl | head -n 1)
	@echo "✓ Python client built and installed"
	@echo ""
	@echo "$(CYAN)Building TypeScript client library...$(RESET)"
	@cd ekodb-client-ts && npm run build --silent
	@echo "✓ TypeScript client built"
	@echo ""
	@echo "$(CYAN)Installing TypeScript client in examples...$(RESET)"
	@cd examples/typescript && npm install --silent
	@echo "✓ TypeScript client installed"
	@echo ""
	@echo "$(CYAN)Building TypeScript example...$(RESET)"
	@cd examples/typescript && npm run build --silent
	@echo "✓ TypeScript example built"
	@echo ""
	@echo "$(CYAN)Building Go client library...$(RESET)"
	@cd ../ekodb-client-go && go build -o /dev/null . 2>&1 | head -5 || true
	@echo "✓ Go client built"
	@echo ""
	@echo "$(CYAN)Building Go RAG example...$(RESET)"
	@cd examples/go && go build -o rag_conversation_system rag_conversation_system.go
	@echo "✓ Go example built"
	@echo ""
	@echo "$(CYAN)Building Kotlin client library...$(RESET)"
	@cd ekodb-client-kt && ./gradlew build -x test --quiet
	@echo "✓ Kotlin client built"
	@echo ""
	@echo "$(CYAN)Building Kotlin RAG example...$(RESET)"
	@cd examples/kotlin && ./gradlew build -x test --quiet
	@echo "✓ Kotlin example built"
	@echo ""
	@echo "$(CYAN)Running Rust RAG Example...$(RESET)"
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@cd examples/rust && cargo run --example rag_conversation_system --quiet
	@echo ""
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@echo "$(CYAN)Running Python RAG Example...$(RESET)"
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@cd examples/python && python3 rag_conversation_system.py
	@echo ""
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@echo "$(CYAN)Running TypeScript RAG Example...$(RESET)"
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@cd examples/typescript && node dist/rag_conversation_system.js
	@echo ""
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@echo "$(CYAN)Running Go RAG Example...$(RESET)"
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@cd examples/go && ./rag_conversation_system
	@echo ""
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@echo "$(CYAN)Running Kotlin RAG Example...$(RESET)"
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@cd examples/kotlin && export $$(grep -v '^#' .env | grep -v '^$$' | xargs) && ./gradlew run -PmainClass="io.ekodb.client.examples.RagConversationSystemKt" --quiet
	@echo ""
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@echo "✅ $(GREEN)RAG Examples Complete!$(RESET)"
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@echo ""
	@echo "$(GREEN)What you just saw across 5 languages:$(RESET)"
	@echo "  ✓ Embeddings generated via ekoDB Functions"
	@echo "  ✓ Hybrid search (semantic + keyword)"
	@echo "  ✓ Text search with stemming"
	@echo "  ✓ Cross-conversation context retrieval"
	@echo "  ✓ Simple client helpers wrapping powerful AI"
	@echo ""
	@echo "$(CYAN)Mission: AI for All 🚀$(RESET) - Making RAG accessible to everyone!"
	@echo ""

# ============================================================================
# SWR (Stale-While-Revalidate) Edge Cache Examples
# ============================================================================
.PHONY: test-examples-swr test-examples-ts-swr test-examples-py-swr test-examples-go-swr test-examples-rust-swr test-examples-kt-swr

test-examples-swr:
	@echo ""
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@echo "🌐 $(CYAN)Testing SWR (Stale-While-Revalidate) Pattern Examples$(RESET)"
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@echo ""
	@$(MAKE) test-examples-ts-swr test-examples-py-swr test-examples-go-swr test-examples-rust-swr test-examples-kt-swr
	@echo ""
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@echo "✅ $(GREEN)All SWR Examples Complete!$(RESET)"
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@echo ""
	@echo "$(GREEN)What you just saw - ekoDB as Edge Cache:$(RESET)"
	@echo "  ✓ FindById → Check cache"
	@echo "  ✓ If/Else → Conditional execution"
	@echo "  ✓ HttpRequest → External API calls"
	@echo "  ✓ Insert with TTL → Auto-expiring cache"
	@echo "  ✓ Sub-millisecond cache hits"
	@echo "  ✓ No Redis, no CDN, no cache invalidation logic needed"
	@echo ""
	@echo "$(CYAN)Your DATABASE is your EDGE! 🚀$(RESET)"
	@echo ""

test-examples-ts-swr: build-typescript-client
	@echo ""
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@echo "$(CYAN)Running TypeScript SWR Examples...$(RESET)"
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@cd examples/typescript && npx tsx client_swr_pattern.ts
	@cd examples/typescript && npx tsx client_edge_cache.ts
	@echo "✅ $(GREEN)TypeScript SWR examples complete!$(RESET)"

test-examples-py-swr: build-python-client
	@echo ""
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@echo "$(CYAN)Running Python SWR Examples...$(RESET)"
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@cd examples/python && python3 swr_pattern.py
	@echo "✅ $(GREEN)Python SWR examples complete!$(RESET)"

test-examples-go-swr:
	@echo ""
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@echo "$(CYAN)Running Go SWR Examples...$(RESET)"
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@cd examples/go && go run swr_pattern.go
	@echo "✅ $(GREEN)Go SWR examples complete!$(RESET)"

test-examples-rust-swr: build-client
	@echo ""
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@echo "$(CYAN)Running Rust SWR Examples...$(RESET)"
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@cd examples/rust && cargo run --example swr_pattern
	@echo "✅ $(GREEN)Rust SWR examples complete!$(RESET)"

test-examples-rust-fcomp: build-client
	@echo ""
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@echo "$(CYAN)Running Rust Function Composition Examples...$(RESET)"
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@cd examples/rust && cargo run --example client_function_composition
	@echo "✅ $(GREEN)Rust function composition examples complete!$(RESET)"

test-examples-rs-fcomp: test-examples-rust-fcomp

# ============================================================================
# Function Composition Examples (CallFunction)
# ============================================================================
.PHONY: test-examples-fcomp test-examples-ts-fcomp test-examples-py-fcomp test-examples-go-fcomp test-examples-js-fcomp test-examples-kt-fcomp

test-examples-fcomp:
	@echo ""
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@echo "🔗 $(CYAN)Testing Function Composition Examples$(RESET)"
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@echo ""
	@$(MAKE) test-examples-rust-fcomp test-examples-ts-fcomp test-examples-py-fcomp test-examples-go-fcomp test-examples-js-fcomp
	@echo ""
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@echo "✅ $(GREEN)All Function Composition Examples Complete!$(RESET)"
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@echo ""
	@echo "$(GREEN)What you just saw - CallFunction composability:$(RESET)"
	@echo "  ✓ Reusable Script building blocks"
	@echo "  ✓ Scripts calling other Scripts"
	@echo "  ✓ Clean SWR patterns via composition"
	@echo "  ✓ Multi-level nesting (arbitrary depth)"
	@echo "  ✓ No code duplication"
	@echo "  ✓ Single source of truth"
	@echo ""
	@echo "$(CYAN)Build complex workflows from simple pieces! 🚀$(RESET)"
	@echo ""

test-examples-ts-fcomp: build-typescript-client
	@echo ""
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@echo "$(CYAN)Running TypeScript Function Composition Examples...$(RESET)"
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@cd examples/typescript && npx tsx client_function_composition.ts
	@echo "✅ $(GREEN)TypeScript function composition examples complete!$(RESET)"

test-examples-py-fcomp: build-python-client
	@echo ""
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@echo "$(CYAN)Running Python Function Composition Examples...$(RESET)"
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@cd examples/python && python3 client_function_composition.py
	@echo "✅ $(GREEN)Python function composition examples complete!$(RESET)"

test-examples-go-fcomp:
	@echo ""
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@echo "$(CYAN)Running Go Function Composition Examples...$(RESET)"
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@cd examples/go && go run client_function_composition.go
	@echo "✅ $(GREEN)Go function composition examples complete!$(RESET)"

test-examples-js-fcomp: build-javascript-client
	@echo ""
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@echo "$(CYAN)Running JavaScript Function Composition Examples...$(RESET)"
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@cd examples/javascript && node client_function_composition.js
	@echo "✅ $(GREEN)JavaScript function composition examples complete!$(RESET)"

test-examples-kt-fcomp:
	@echo ""
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@echo "$(CYAN)Running Kotlin Function Composition Examples...$(RESET)"
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@if [ -f .env ]; then \
		. ./.env && \
		export JAVA_HOME=$$(/usr/libexec/java_home -v 17) && export PATH=$$JAVA_HOME/bin:$$PATH && \
		cd examples/kotlin && \
		API_BASE_URL=$$API_BASE_URL WS_BASE_URL=$$WS_BASE_URL API_BASE_KEY=$$API_BASE_KEY ./gradlew run -PmainClass=io.ekodb.client.examples.ClientFunctionCompositionKt --no-daemon; \
	else \
		echo "$(RED)✗ .env file not found$(RESET)"; \
		exit 1; \
	fi
	@echo "✅ $(GREEN)Kotlin function composition examples complete!$(RESET)"

test-examples-kt-swr:
	@echo ""
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@echo "$(CYAN)Running Kotlin SWR Examples...$(RESET)"
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@if [ -f .env ]; then \
		. ./.env && \
		export JAVA_HOME=$$(/usr/libexec/java_home -v 17) && export PATH=$$JAVA_HOME/bin:$$PATH && \
		cd examples/kotlin && \
		API_BASE_URL=$$API_BASE_URL WS_BASE_URL=$$WS_BASE_URL API_BASE_KEY=$$API_BASE_KEY ./gradlew run -PmainClass=io.ekodb.client.examples.SwrPatternKt --no-daemon; \
	else \
		echo "$(RED)✗ .env file not found$(RESET)"; \
		exit 1; \
	fi
	@echo "✅ $(GREEN)Kotlin SWR examples complete!$(RESET)"

# ============================================================================
# TTL Verification Tests (verify TTL expiration actually works)
# ============================================================================
.PHONY: test-examples-ttl test-examples-ttl-rust test-examples-ttl-go test-examples-ttl-js test-examples-ttl-py test-examples-ttl-ts

test-examples-ttl:
	@echo ""
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@echo "⏰ $(CYAN)TTL Expiration Verification Tests$(RESET)"
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@echo ""
	@echo "$(YELLOW)These tests VERIFY that TTL expiration actually works by:$(RESET)"
	@echo "  1. Inserting documents with short TTL (3 seconds)"
	@echo "  2. Verifying documents exist immediately"
	@echo "  3. Waiting for TTL to expire"
	@echo "  4. Verifying documents are GONE"
	@echo ""
	@$(MAKE) test-examples-ttl-rust test-examples-ttl-go test-examples-ttl-js test-examples-ttl-py test-examples-ttl-ts
	@echo ""
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@echo "✅ $(GREEN)All TTL Verification Tests Passed!$(RESET)"
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"

test-examples-ttl-rust: build-client
	@echo ""
	@echo "🦀 $(YELLOW)Rust TTL Verification Tests...$(RESET)"
	@cd examples/rust && cargo run --example ttl_expiration_test
	@echo "✅ $(GREEN)Rust TTL tests complete!$(RESET)"

test-examples-ttl-go:
	@echo ""
	@echo "🔷 $(YELLOW)Go TTL Verification Tests...$(RESET)"
	@cd examples/go/ttl && go run document_ttl.go
	@cd examples/go/ttl && go run websocket_ttl.go
	@echo "✅ $(GREEN)Go TTL tests complete!$(RESET)"

test-examples-ttl-js:
	@echo ""
	@echo "📦 $(YELLOW)JavaScript TTL Verification Tests...$(RESET)"
	@cd examples/javascript/ttl && node document_ttl.js
	@cd examples/javascript/ttl && node websocket_ttl.js
	@echo "✅ $(GREEN)JavaScript TTL tests complete!$(RESET)"

test-examples-ttl-py:
	@echo ""
	@echo "🐍 $(YELLOW)Python TTL Verification Tests...$(RESET)"
	@cd examples/python/ttl && python3 document_ttl.py
	@cd examples/python/ttl && python3 websocket_ttl.py
	@echo "✅ $(GREEN)Python TTL tests complete!$(RESET)"

test-examples-ttl-ts: build-typescript-client
	@echo ""
	@echo "📘 $(YELLOW)TypeScript TTL Verification Tests...$(RESET)"
	@cd examples/typescript && npx tsx ttl_expiration_test.ts
	@echo "✅ $(GREEN)TypeScript TTL tests complete!$(RESET)"

# ============================================================================
# Rust Examples (both direct + client)
# ============================================================================
test-examples-rust:
	@echo "make test-examples-rust" > examples/rust/test-examples-rs.md
	@$(MAKE) test-examples-rust-direct test-examples-rust-client 2>&1 | tee -a examples/rust/test-examples-rs.md
	@echo "✅ $(GREEN)All Rust integration tests complete!$(RESET)"

test-examples-rs: test-examples-rust
test-examples-rs-direct: test-examples-rust-direct
test-examples-rs-client: test-examples-rust-client

test-examples-rust-direct:
	@echo "🧪 $(CYAN)Running Rust examples (direct HTTP/WebSocket)...$(RESET)"
	@cd examples/rust && cargo run --example simple_crud && cargo run --example simple_websocket && cargo run --example batch_operations && cargo run --example kv_operations && cargo run --example collection_management && cargo run --example document_ttl && cargo run --example websocket_ttl && cargo run --example http_functions && cargo run --example transactions
	@echo "✅ $(GREEN)Rust direct examples complete!$(RESET)"

test-examples-rust-client: build-client
	@echo "🧪 $(CYAN)Running Rust client library examples...$(RESET)"
	@cd examples/rust && cargo run --example client_simple_crud
	@cd examples/rust && cargo run --example client_simple_websocket
	@cd examples/rust && cargo run --example client_batch_operations
	@cd examples/rust && cargo run --example client_collection_management
	@cd examples/rust && cargo run --example client_kv_operations
	@cd examples/rust && cargo run --example client_transactions
	@cd examples/rust && cargo run --example client_query_builder
	@cd examples/rust && cargo run --example client_search
	@cd examples/rust && cargo run --example client_schema
	@cd examples/rust && cargo run --example client_joins
	@cd examples/rust && cargo run --example client_document_ttl
	@cd examples/rust && cargo run --example client_websocket_ttl
	@cd examples/rust && cargo run --example client_edge_cache
	@cd examples/rust && cargo run --example client_functions
	@cd examples/rust && cargo run --example client_function_composition
	@cd examples/rust && cargo run --example client_functions_complete
	@cd examples/rust && cargo run --example client_functions_kv_wrapped
	@cd examples/rust && cargo run --example client_swr_pattern
	@cd examples/rust && cargo run --example client_functions_advanced
	@cd examples/rust && cargo run --example client_functions_ai
	@cd examples/rust && cargo run --example client_functions_crud
	@cd examples/rust && cargo run --example client_functions_search
	@cd examples/rust && cargo run --example client_chat_basic
	@cd examples/rust && cargo run --example client_chat_advanced
	@cd examples/rust && cargo run --example client_chat_sessions
	@echo "✅ $(GREEN)Rust client examples complete!$(RESET)"

# ============================================================================
# Python Examples (both direct + client)
# ============================================================================
test-examples-python:
	@echo "make test-examples-python" > examples/python/text-examples-py.md
	@$(MAKE) test-examples-python-direct test-examples-python-client 2>&1 | tee -a examples/python/text-examples-py.md
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
	@cd examples/python && python3 client_simple_websocket.py
	@cd examples/python && python3 client_batch_operations.py
	@cd examples/python && python3 client_collection_management.py
	@cd examples/python && python3 client_kv_operations.py
	@cd examples/python && python3 client_transactions.py
	@cd examples/python && python3 client_query_builder.py
	@cd examples/python && python3 client_search.py
	@cd examples/python && python3 client_schema.py
	@cd examples/python && python3 client_joins.py
	@cd examples/python && python3 client_document_ttl.py
	@cd examples/python && python3 client_websocket_ttl.py
	@cd examples/python && python3 client_edge_cache.py
	@cd examples/python && python3 client_functions.py
	@cd examples/python && python3 client_function_composition.py
	@cd examples/python && python3 client_functions_complete.py
	@cd examples/python && python3 client_functions_kv_wrapped.py
	@cd examples/python && python3 client_swr_pattern.py
	@cd examples/python && python3 client_functions_advanced.py
	@cd examples/python && python3 client_functions_ai.py
	@cd examples/python && python3 client_functions_crud.py
	@cd examples/python && python3 client_functions_search.py
	@cd examples/python && python3 client_chat_basic.py
	@cd examples/python && python3 client_chat_advanced.py
	@cd examples/python && python3 client_chat_sessions.py
	@echo "✅ $(GREEN)Python client examples complete!$(RESET)"

# ============================================================================
# Go Examples (both direct + client)
# ============================================================================
test-examples-go:
	@echo "make test-examples-go" > examples/go/test-examples-go.md
	@$(MAKE) test-examples-go-direct test-examples-go-client 2>&1 | tee -a examples/go/test-examples-go.md
	@echo "✅ $(GREEN)All Go integration tests complete!$(RESET)"

test-examples-go-direct:
	@echo "🧪 $(CYAN)Running Go examples (direct HTTP/WebSocket)...$(RESET)"
	@cd examples/go && go run test_runner.go
	@echo "✅ $(GREEN)Go direct examples complete!$(RESET)"

test-examples-go-client:
	@echo "🧪 $(CYAN)Running Go client library examples...$(RESET)"
	@cd examples/go && go run client_simple_crud.go
	@cd examples/go && go run client_simple_websocket.go
	@cd examples/go && go run client_batch_operations.go
	@cd examples/go && go run client_collection_management.go
	@cd examples/go && go run client_kv_operations.go
	@cd examples/go && go run client_transactions.go
	@cd examples/go && go run client_query_builder.go
	@cd examples/go && go run client_search.go
	@cd examples/go && go run client_schema.go
	@cd examples/go && go run client_joins.go
	@cd examples/go && go run client_document_ttl.go
	@cd examples/go && go run client_websocket_ttl.go
	@cd examples/go && go run client_edge_cache.go
	@cd examples/go && go run client_functions.go
	@cd examples/go && go run client_function_composition.go
	@cd examples/go && go run client_functions_complete.go
	@cd examples/go && go run client_functions_kv_wrapped.go
	@cd examples/go && go run client_swr_pattern.go
	@cd examples/go && go run client_functions_advanced.go
	@cd examples/go && go run client_functions_ai.go
	@cd examples/go && go run client_functions_crud.go
	@cd examples/go && go run client_functions_search.go
	@cd examples/go && go run client_chat_basic.go
	@cd examples/go && go run client_chat_advanced.go
	@cd examples/go && go run client_chat_sessions.go
	@echo "✅ $(GREEN)Go client examples complete!$(RESET)"

# ============================================================================
# TypeScript Examples (client only - no direct examples)
# ============================================================================
test-examples-typescript:
	@echo "make test-examples-typescript" > examples/typescript/test-examples-ts.md
	@$(MAKE) test-examples-typescript-client 2>&1 | tee -a examples/typescript/test-examples-ts.md
	@echo "✅ $(GREEN)All TypeScript integration tests complete!$(RESET)"

test-examples-ts: test-examples-typescript
test-examples-ts-client: test-examples-typescript-client

build-typescript-client:
	@echo "📦 $(CYAN)Building TypeScript client library...$(RESET)"
	@cd ekodb-client-ts && npm install && npm run build
	@echo "✅ $(GREEN)TypeScript client built!$(RESET)"

# JavaScript uses the TypeScript client (npm package)
build-javascript-client: build-typescript-client

test-examples-typescript-client: build-typescript-client
	@echo "🧪 $(CYAN)Running TypeScript client library examples...$(RESET)"
	@cd examples/typescript && npm install
	@cd examples/typescript && npx tsx client_simple_crud.ts
	@cd examples/typescript && npx tsx client_simple_websocket.ts
	@cd examples/typescript && npx tsx client_batch_operations.ts
	@cd examples/typescript && npx tsx client_collection_management.ts
	@cd examples/typescript && npx tsx client_kv_operations.ts
	@cd examples/typescript && npx tsx client_transactions.ts
	@cd examples/typescript && npx tsx client_query_builder.ts
	@cd examples/typescript && npx tsx client_search.ts
	@cd examples/typescript && npx tsx client_schema.ts
	@cd examples/typescript && npx tsx client_joins.ts
	@cd examples/typescript && npx tsx client_document_ttl.ts
	@cd examples/typescript && npx tsx client_websocket_ttl.ts
	@cd examples/typescript && npx tsx client_edge_cache.ts
	@cd examples/typescript && npx tsx client_functions.ts
	@cd examples/typescript && npx tsx client_function_composition.ts
	@cd examples/typescript && npx tsx client_functions_complete.ts
	@cd examples/typescript && npx tsx client_functions_kv_wrapped.ts
	@cd examples/typescript && npx tsx client_swr_pattern.ts
	@cd examples/typescript && npx tsx client_functions_advanced.ts
	@cd examples/typescript && npx tsx client_functions_ai.ts
	@cd examples/typescript && npx tsx client_functions_crud.ts
	@cd examples/typescript && npx tsx client_functions_search.ts
	@cd examples/typescript && npx tsx client_chat_basic.ts
	@cd examples/typescript && npx tsx client_chat_advanced.ts
	@cd examples/typescript && npx tsx client_chat_sessions.ts
	@echo "✅ $(GREEN)TypeScript client examples complete!$(RESET)"

# ============================================================================
# JavaScript Examples (both direct + client)
# ============================================================================
test-examples-javascript:
	@echo "make test-examples-javascript" > examples/javascript/test-examples-js.md
	@$(MAKE) test-examples-javascript-direct test-examples-javascript-client 2>&1 | tee -a examples/javascript/test-examples-js.md
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
	@cd examples/javascript && node client_simple_websocket.js
	@cd examples/javascript && node client_batch_operations.js
	@cd examples/javascript && node client_collection_management.js
	@cd examples/javascript && node client_kv_operations.js
	@cd examples/javascript && node client_transactions.js
	@cd examples/javascript && node client_query_builder.js
	@cd examples/javascript && node client_search.js
	@cd examples/javascript && node client_schema.js
	@cd examples/javascript && node client_joins.js
	@cd examples/javascript && node client_document_ttl.js
	@cd examples/javascript && node client_websocket_ttl.js
	@cd examples/javascript && node client_edge_cache.js
	@cd examples/javascript && node client_functions.js
	@cd examples/javascript && node client_function_composition.js
	@cd examples/javascript && node client_functions_complete.js
	@cd examples/javascript && node client_functions_kv_wrapped.js
	@cd examples/javascript && node client_functions_crud.js
	@cd examples/javascript && node client_functions_search.js
	@cd examples/javascript && node client_functions_advanced.js
	@cd examples/javascript && node client_functions_ai.js
	@cd examples/javascript && node client_swr_pattern.js
	@cd examples/javascript && node client_chat_basic.js
	@cd examples/javascript && node client_chat_advanced.js
	@cd examples/javascript && node client_chat_sessions.js
	@echo "✅ $(GREEN)JavaScript client examples complete!$(RESET)"

# ============================================================================
# Kotlin Examples (client + transactions)
# ============================================================================
test-examples-kotlin:
	@echo "make test-examples-kotlin" > examples/kotlin/test-examples-kt.md
	@$(MAKE) test-examples-kotlin-client 2>&1 | tee -a examples/kotlin/test-examples-kt.md
	@echo "🟣 $(YELLOW)Kotlin Transactions...$(RESET)"
	@if [ -f .env ]; then . ./.env; fi && \
		export JAVA_HOME=$$(/usr/libexec/java_home -v 17) && export PATH=$$JAVA_HOME/bin:$$PATH && \
		cd examples/kotlin && API_BASE_URL=$$API_BASE_URL API_BASE_KEY=$$API_BASE_KEY ./gradlew run -PmainClass="io.ekodb.client.examples.ClientTransactionsKt" --quiet 2>&1 | tee -a test-examples-kt.md
	@echo "✅ $(GREEN)All Kotlin integration tests complete!$(RESET)"

test-examples-kt: test-examples-kotlin

build-kotlin-client:
	@echo "🟣 $(CYAN)Building Kotlin client library...$(RESET)"
	@cd ekodb-client-kt && export JAVA_HOME=$$(/usr/libexec/java_home -v 17) && export PATH=$$JAVA_HOME/bin:$$PATH && ./gradlew build --no-daemon
	@echo "✅ $(GREEN)Kotlin client built!$(RESET)"

test-examples-kotlin-client: build-kotlin-client
	@echo "🧪 $(CYAN)Running Kotlin client library examples...$(RESET)"
	@if [ -f .env ]; then \
		. ./.env && \
		export JAVA_HOME=$$(/usr/libexec/java_home -v 17) && export PATH=$$JAVA_HOME/bin:$$PATH && \
		cd examples/kotlin && \
		API_BASE_URL=$$API_BASE_URL WS_BASE_URL=$$WS_BASE_URL API_BASE_KEY=$$API_BASE_KEY ./gradlew run -PmainClass=io.ekodb.client.examples.ClientSimpleCrudKt --no-daemon && \
		API_BASE_URL=$$API_BASE_URL WS_BASE_URL=$$WS_BASE_URL API_BASE_KEY=$$API_BASE_KEY ./gradlew run -PmainClass=io.ekodb.client.examples.ClientSimpleWebsocketKt --no-daemon && \
		API_BASE_URL=$$API_BASE_URL WS_BASE_URL=$$WS_BASE_URL API_BASE_KEY=$$API_BASE_KEY ./gradlew run -PmainClass=io.ekodb.client.examples.ClientBatchOperationsKt --no-daemon && \
		API_BASE_URL=$$API_BASE_URL WS_BASE_URL=$$WS_BASE_URL API_BASE_KEY=$$API_BASE_KEY ./gradlew run -PmainClass=io.ekodb.client.examples.ClientCollectionManagementKt --no-daemon && \
		API_BASE_URL=$$API_BASE_URL WS_BASE_URL=$$WS_BASE_URL API_BASE_KEY=$$API_BASE_KEY ./gradlew run -PmainClass=io.ekodb.client.examples.ClientKvOperationsKt --no-daemon && \
		API_BASE_URL=$$API_BASE_URL WS_BASE_URL=$$WS_BASE_URL API_BASE_KEY=$$API_BASE_KEY ./gradlew run -PmainClass=io.ekodb.client.examples.ClientTransactionsKt --no-daemon && \
		API_BASE_URL=$$API_BASE_URL WS_BASE_URL=$$WS_BASE_URL API_BASE_KEY=$$API_BASE_KEY ./gradlew run -PmainClass=io.ekodb.client.examples.ClientQueryBuilderKt --no-daemon && \
		API_BASE_URL=$$API_BASE_URL WS_BASE_URL=$$WS_BASE_URL API_BASE_KEY=$$API_BASE_KEY ./gradlew run -PmainClass=io.ekodb.client.examples.ClientSearchKt --no-daemon && \
		API_BASE_URL=$$API_BASE_URL WS_BASE_URL=$$WS_BASE_URL API_BASE_KEY=$$API_BASE_KEY ./gradlew run -PmainClass=io.ekodb.client.examples.ClientSchemaManagementKt --no-daemon && \
		API_BASE_URL=$$API_BASE_URL WS_BASE_URL=$$WS_BASE_URL API_BASE_KEY=$$API_BASE_KEY ./gradlew run -PmainClass=io.ekodb.client.examples.ClientJoinsKt --no-daemon && \
		API_BASE_URL=$$API_BASE_URL WS_BASE_URL=$$WS_BASE_URL API_BASE_KEY=$$API_BASE_KEY ./gradlew run -PmainClass=io.ekodb.client.examples.ClientDocumentTtlKt --no-daemon && \
		API_BASE_URL=$$API_BASE_URL WS_BASE_URL=$$WS_BASE_URL API_BASE_KEY=$$API_BASE_KEY ./gradlew run -PmainClass=io.ekodb.client.examples.ClientWebsocketTtlKt --no-daemon && \
		API_BASE_URL=$$API_BASE_URL WS_BASE_URL=$$WS_BASE_URL API_BASE_KEY=$$API_BASE_KEY ./gradlew run -PmainClass=io.ekodb.client.examples.ClientEdgeCacheKt --no-daemon && \
		API_BASE_URL=$$API_BASE_URL WS_BASE_URL=$$WS_BASE_URL API_BASE_KEY=$$API_BASE_KEY ./gradlew run -PmainClass=io.ekodb.client.examples.ClientFunctionsKt --no-daemon && \
		API_BASE_URL=$$API_BASE_URL WS_BASE_URL=$$WS_BASE_URL API_BASE_KEY=$$API_BASE_KEY ./gradlew run -PmainClass=io.ekodb.client.examples.ClientFunctionsCompleteKt --no-daemon && \
		API_BASE_URL=$$API_BASE_URL WS_BASE_URL=$$WS_BASE_URL API_BASE_KEY=$$API_BASE_KEY ./gradlew run -PmainClass=io.ekodb.client.examples.ClientFunctionsKvWrappedKt --no-daemon && \
		API_BASE_URL=$$API_BASE_URL WS_BASE_URL=$$WS_BASE_URL API_BASE_KEY=$$API_BASE_KEY ./gradlew run -PmainClass=io.ekodb.client.examples.ClientSwrPatternKt --no-daemon && \
		API_BASE_URL=$$API_BASE_URL WS_BASE_URL=$$WS_BASE_URL API_BASE_KEY=$$API_BASE_KEY ./gradlew run -PmainClass=io.ekodb.client.examples.ClientFunctionsAdvancedKt --no-daemon && \
		API_BASE_URL=$$API_BASE_URL WS_BASE_URL=$$WS_BASE_URL API_BASE_KEY=$$API_BASE_KEY ./gradlew run -PmainClass=io.ekodb.client.examples.ClientFunctionsAiKt --no-daemon && \
		API_BASE_URL=$$API_BASE_URL WS_BASE_URL=$$WS_BASE_URL API_BASE_KEY=$$API_BASE_KEY ./gradlew run -PmainClass=io.ekodb.client.examples.ClientFunctionsCrudKt --no-daemon && \
		API_BASE_URL=$$API_BASE_URL WS_BASE_URL=$$WS_BASE_URL API_BASE_KEY=$$API_BASE_KEY ./gradlew run -PmainClass=io.ekodb.client.examples.ClientFunctionsSearchKt --no-daemon && \
		API_BASE_URL=$$API_BASE_URL WS_BASE_URL=$$WS_BASE_URL API_BASE_KEY=$$API_BASE_KEY ./gradlew run -PmainClass=io.ekodb.client.examples.ClientChatBasicKt --no-daemon && \
		API_BASE_URL=$$API_BASE_URL WS_BASE_URL=$$WS_BASE_URL API_BASE_KEY=$$API_BASE_KEY ./gradlew run -PmainClass=io.ekodb.client.examples.ClientChatAdvancedKt --no-daemon && \
		API_BASE_URL=$$API_BASE_URL WS_BASE_URL=$$WS_BASE_URL API_BASE_KEY=$$API_BASE_KEY ./gradlew run -PmainClass=io.ekodb.client.examples.ClientChatSessionsKt --no-daemon; \
	else \
		echo "$(RED)❌ .env file not found$(RESET)"; \
		echo "$(YELLOW)💡 Create .env file with API_BASE_URL, WS_BASE_URL, and API_BASE_KEY$(RESET)"; \
		exit 1; \
	fi
	@echo "✅ $(GREEN)Kotlin client examples complete!$(RESET)"

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
fmt: examples-ls examples-ls-badge fmt-rust fmt-python fmt-go fmt-typescript fmt-md
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
		gofmt -w examples/go/; \
		echo "✅ $(GREEN)Go formatting complete with gofmt!$(RESET)"; \
		echo "💡 $(YELLOW)Note: Go client is in separate repo: github.com/ekoDB/ekodb-client-go$(RESET)"; \
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
		find . -name "*.md" -not -path "./target/*" -not -path "./node_modules/*" -not -name "test-examples*.md" -exec markdownlint --fix {} \; 2>/dev/null || true; \
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
	@echo "🔷 $(CYAN)Installing Go example dependencies...$(RESET)"
	@cd examples/go && go mod download
	@echo "✅ $(GREEN)Go example dependencies installed!$(RESET)"
	@echo "💡 $(YELLOW)Note: Go client is in separate repo: github.com/ekoDB/ekodb-client-go$(RESET)"

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

# ============================================================================
# Dependency Management - All Packages
# ============================================================================

# Check for outdated dependencies (Rust only)
deps-check:
	@echo "📦 $(CYAN)Checking for outdated Rust dependencies...$(RESET)"
	@if command -v cargo-outdated > /dev/null; then \
		cargo outdated; \
	else \
		echo "$(YELLOW)cargo-outdated is not installed.$(RESET)"; \
		echo "$(YELLOW)Run 'cargo install cargo-outdated' to install it.$(RESET)"; \
	fi
	@echo "✅ $(GREEN)Rust dependencies check complete!$(RESET)"

# Update dependencies within Cargo.toml constraints (Rust only)
deps-update:
	@echo "📦 $(CYAN)Updating Rust dependencies within constraints...$(RESET)"
	$(CARGO) update
	@echo "✅ $(GREEN)Rust dependencies updated successfully!$(RESET)"
	@echo "💡 $(YELLOW)Run 'make deps-check' to see if any dependencies still need updating$(RESET)"

# Check all packages for outdated dependencies
deps-check-all: deps-check-rust deps-check-typescript deps-check-kotlin
	@echo "✅ $(GREEN)All dependency checks complete!$(RESET)"

# Update all packages' dependencies
deps-update-all: deps-update-rust deps-update-typescript deps-update-kotlin
	@echo "✅ $(GREEN)All dependencies updated!$(RESET)"

# Rust dependency checks (detailed)
deps-check-rust:
	@echo "🦀 $(CYAN)Checking Rust workspace dependencies...$(RESET)"
	@echo ""
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@echo "📦 Workspace Root & ekodb_client"
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@if command -v cargo-outdated > /dev/null; then \
		cargo outdated; \
	else \
		echo "$(RED)❌ cargo-outdated not installed$(RESET)"; \
		echo "$(YELLOW)Run: cargo install cargo-outdated$(RESET)"; \
		exit 1; \
	fi
	@echo ""
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@echo "📦 Python Bindings (ekodb-client-py)"
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@cd ekodb-client-py && cargo outdated
	@echo ""
	@echo "💡 $(YELLOW)Note: Many 'Removed' entries are transitive dependencies - safe to ignore$(RESET)"
	@echo "💡 $(YELLOW)Focus on direct dependencies with major version updates$(RESET)"

# TypeScript dependency checks
deps-check-typescript:
	@echo "📘 $(CYAN)Checking TypeScript/npm dependencies...$(RESET)"
	@echo ""
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@echo "📦 ekodb-client-ts"
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@if [ -d "ekodb-client-ts" ]; then \
		cd ekodb-client-ts && npm outdated || true; \
		echo ""; \
		echo "$(YELLOW)💡 To update: cd ekodb-client-ts && npm update$(RESET)"; \
		echo "$(YELLOW)💡 For major updates: npm install -g npm-check-updates && ncu -u$(RESET)"; \
	else \
		echo "$(RED)❌ ekodb-client-ts directory not found$(RESET)"; \
	fi

# Kotlin dependency checks  
deps-check-kotlin:
	@echo "🟣 $(CYAN)Checking Kotlin/Gradle dependencies...$(RESET)"
	@echo ""
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@echo "📦 ekodb-client-kt"
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@if [ -d "ekodb-client-kt" ]; then \
		cd ekodb-client-kt && ./gradlew dependencyUpdates --no-daemon || { \
			echo "$(YELLOW)⚠️  dependencyUpdates plugin not configured$(RESET)"; \
			echo "$(YELLOW)Current versions in build.gradle.kts:$(RESET)"; \
			echo "  - Kotlin: $$(grep 'kotlin("jvm")' build.gradle.kts | grep -o 'version "[^"]*"' | cut -d'"' -f2)"; \
			echo "  - Coroutines: $$(grep 'kotlinx-coroutines-core' build.gradle.kts | grep -o '[0-9]\+\.[0-9]\+\.[0-9]\+')"; \
			echo "  - Serialization: $$(grep 'kotlinx-serialization-json' build.gradle.kts | grep -o '[0-9]\+\.[0-9]\+\.[0-9]\+')"; \
			echo "  - Ktor: $$(grep 'ktor-client-core' build.gradle.kts | grep -o '[0-9]\+\.[0-9]\+\.[0-9]\+')"; \
			echo ""; \
			echo "$(YELLOW)💡 To add dependencyUpdates plugin, add to build.gradle.kts:$(RESET)"; \
			echo "$(YELLOW)   plugins { id(\"com.github.ben-manes.versions\") version \"0.51.0\" }$(RESET)"; \
		}; \
	else \
		echo "$(RED)❌ ekodb-client-kt directory not found$(RESET)"; \
	fi

# Rust dependency updates (detailed)
deps-update-rust:
	@echo "🦀 $(CYAN)Updating Rust workspace dependencies...$(RESET)"
	@echo ""
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@echo "📦 Workspace Root & ekodb_client"
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	$(CARGO) update --workspace
	@echo ""
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@echo "📦 Python Bindings (ekodb-client-py)"
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@cd ekodb-client-py && cargo update
	@echo ""
	@echo "✅ $(GREEN)Rust dependencies updated!$(RESET)"
	@echo "💡 $(YELLOW)Run 'make deps-check-rust' to see remaining updates$(RESET)"

# TypeScript dependency updates
deps-update-typescript:
	@echo "📘 $(CYAN)Updating TypeScript/npm dependencies...$(RESET)"
	@echo ""
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@echo "📦 ekodb-client-ts"
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@if [ -d "ekodb-client-ts" ]; then \
		cd ekodb-client-ts && npm update && npm audit fix || true; \
		echo ""; \
		echo "✅ $(GREEN)TypeScript dependencies updated!$(RESET)"; \
		echo "💡 $(YELLOW)For major updates: npm install -g npm-check-updates && cd ekodb-client-ts && ncu -u && npm install$(RESET)"; \
	else \
		echo "$(RED)❌ ekodb-client-ts directory not found$(RESET)"; \
	fi

# Kotlin dependency updates
deps-update-kotlin:
	@echo "🟣 $(CYAN)Updating Kotlin/Gradle dependencies...$(RESET)"
	@echo ""
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@echo "📦 ekodb-client-kt"
	@echo "$(CYAN)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━$(RESET)"
	@echo "$(YELLOW)⚠️  Gradle requires manual dependency updates in build.gradle.kts$(RESET)"
	@echo "$(YELLOW)Run 'make deps-check-kotlin' to see available updates$(RESET)"
	@echo ""
	@if [ -d "ekodb-client-kt" ]; then \
		cd ekodb-client-kt && ./gradlew dependencies --no-daemon --configuration runtimeClasspath || true; \
	else \
		echo "$(RED)❌ ekodb-client-kt directory not found$(RESET)"; \
	fi

# ============================================================================
# Examples Inventory
# ============================================================================

examples-ls:
	@echo "📋 $(CYAN)Generating examples inventory...$(RESET)"
	@chmod +x scripts/generate_examples_list.sh
	@./scripts/generate_examples_list.sh
	@echo "✅ $(GREEN)Examples list generated!$(RESET)"

examples-ls-check:
	@echo "🔍 $(CYAN)Checking examples inventory against committed snapshot...$(RESET)"
	@chmod +x scripts/generate_examples_list.sh
	@./scripts/generate_examples_list.sh --temp
	@if ! diff -I "^# Generated:" examples_list.txt examples_list.txt.tmp > /dev/null 2>&1; then \
		echo "$(RED)❌ Examples inventory has changed!$(RESET)"; \
		echo "$(YELLOW)Differences found:$(RESET)"; \
		diff -I "^# Generated:" examples_list.txt examples_list.txt.tmp || true; \
		echo "$(YELLOW)Run 'make examples-ls' to update the snapshot$(RESET)"; \
		rm -f examples_list.txt.tmp examples_list.json.tmp; \
		exit 1; \
	else \
		echo "$(GREEN)✅ Examples inventory is up to date!$(RESET)"; \
		rm -f examples_list.txt.tmp examples_list.json.tmp; \
	fi

examples-ls-badge:
	@echo "🔄 $(CYAN)Updating README badge with current example statistics...$(RESET)"
	@chmod +x scripts/update_examples_badge.sh
	@./scripts/update_examples_badge.sh
	@echo "✅ $(GREEN)README badge updated!$(RESET)"
