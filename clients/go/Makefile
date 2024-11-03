.PHONY: all build test clean coverage help

# Set default target
.DEFAULT_GOAL := help

# Project structure
PROJECT_DIR=ekodb

# Go parameters
GOCMD=go
GOBUILD=$(GOCMD) build
GOTEST=$(GOCMD) test
GOCLEAN=$(GOCMD) clean
GOMOD=$(GOCMD) mod

# Test flags
TEST_FLAGS=-race -v -count=1
COVERAGE_FLAGS=-coverprofile=coverage.out
COVERAGE_HTML_FLAGS=-html=coverage.out

# Help target
help:
	@echo "Available targets:"
	@echo "  all          - Build (deps, test, build)"
	@echo "  deps         - Install dependencies"
	@echo "  test         - Run tests"
	@echo "  coverage     - Run tests with coverage"
	@echo "  fmt          - Format code"
	@echo "  verify       - Verify dependencies"
	@echo "  clean        - Clean build artifacts"
	@echo "  check-updates- Check for dependency updates"
	@echo "  security     - Run security check"
	@echo "  ci           - Run CI pipeline steps"

# Default target
all: deps
	@echo "Running standard build..."
	@$(MAKE) test || exit 1
	@$(MAKE) build || exit 1

# Install dependencies
deps:
	@echo "Installing dependencies..."
	@cd $(PROJECT_DIR) && $(GOMOD) download || (echo "Failed to download dependencies"; exit 1)
	@cd $(PROJECT_DIR) && $(GOMOD) tidy || (echo "Failed to tidy dependencies"; exit 1)

# Build the project
build:
	@echo "Building..."
	@cd $(PROJECT_DIR) && $(GOBUILD) ./... || (echo "Build failed"; exit 1)

# Run all tests
test:
	@echo "Running tests..."
	@cd $(PROJECT_DIR) && $(GOTEST) $(TEST_FLAGS) ./... || (echo "Tests failed"; exit 1)

# Run tests with coverage
coverage:
	@echo "Running tests with coverage..."
	@cd $(PROJECT_DIR) && $(GOTEST) $(COVERAGE_FLAGS) ./... || (echo "Coverage tests failed"; exit 1)
	@echo "Generating coverage report..."
	@cd $(PROJECT_DIR) && $(GOCMD) tool cover $(COVERAGE_HTML_FLAGS)

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	@cd $(PROJECT_DIR) && $(GOCLEAN)
	@cd $(PROJECT_DIR) && rm -f coverage.out
	@echo "Clean complete"

# Format code
fmt:
	@echo "Formatting code..."
	@cd $(PROJECT_DIR) && $(GOCMD) fmt ./...

# Verify dependencies
verify:
	@echo "Verifying dependencies..."
	@cd $(PROJECT_DIR) && $(GOMOD) verify || (echo "Dependency verification failed"; exit 1)

# Check for updates to dependencies
check-updates:
	@echo "Checking for dependency updates..."
	@cd $(PROJECT_DIR) && GO111MODULE=on $(GOCMD) list -u -m all
	@cd $(PROJECT_DIR) && $(GOMOD) tidy

# Security check
security:
	@echo "Running security check..."
	@if command -v nancy > /dev/null; then \
		cd $(PROJECT_DIR) && nancy go.sum || (echo "Security check failed"; exit 1); \
	else \
		echo "nancy not installed. Install with: go install github.com/sonatype-nexus-community/nancy@latest"; \
		exit 1; \
	fi

# CI pipeline steps
ci: deps
	@echo "Running CI pipeline..."
	@$(MAKE) verify || exit 1	
	@$(MAKE) test || exit 1

# Generate documentation
doc:
	@echo "Generating documentation..."
	@cd $(PROJECT_DIR) && $(GOCMD) doc -all

# Run benchmarks
bench:
	@echo "Running benchmarks..."
	@cd $(PROJECT_DIR) && $(GOTEST) -bench=. -benchmem ./...

# Install tools needed for development
tools:
	@echo "Installing development tools..."
	@go install github.com/sonatype-nexus-community/nancy@latest || (echo "Failed to install nancy"; exit 1)
	@echo "Development tools installed successfully"

# Update Go version in go.mod
update-go:
	@echo "Current Go version:"
	@$(GOCMD) version
	@echo "Updating go.mod to match current Go version..."
	@cd $(PROJECT_DIR) && $(GOCMD) mod tidy || (echo "Failed to tidy modules"; exit 1)
	@cd $(PROJECT_DIR) && $(GOCMD) mod edit -go=$$(go version | cut -d' ' -f3 | tr -d 'go') || (echo "Failed to update Go version"; exit 1)
	@echo "Go version updated successfully"

# Run all quality checks
quality: fmt test coverage security