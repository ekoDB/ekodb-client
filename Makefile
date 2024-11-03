.PHONY: all build test clean

# Default target
all: all-message-start clean build test all-message-end

# Build all clients
build: build-message-start build-nodejs build-python build-go build-message-end

# Test all clients
test: test-message-start test-nodejs test-python test-go test-message-end

# Clean all builds
clean: clean-message-start clean-nodejs clean-python clean-go clean-message-end

# Node.js targets
build-nodejs:
	cd clients/nodejs && npm install && npm run build && echo "Node.js client built successfully"

test-nodejs:
	cd clients/nodejs && npm test && echo "Node.js client tests passed successfully"

clean-nodejs:
	cd clients/nodejs && npm run clean && echo "Node.js client cleaned successfully"

# Python targets
build-python:
	cd clients/python && python3 -m pip install -e ".[dev]" && echo "Python client built successfully"

test-python:
	cd clients/python && python3 -m pytest tests/ --cov=ekodb && echo "Python client tests passed successfully"

clean-python:
	cd clients/python && rm -rf dist/ build/ *.egg-info/ .pytest_cache/ .coverage htmlcov/ && echo "Python client cleaned successfully"

# Go targets
build-go:
	cd clients/go && make build && echo "Go client built successfully"

test-go:
	cd clients/go && make test && echo "Go client tests passed successfully"

clean-go:
	cd clients/go && make clean && echo "Go client cleaned successfully"

#
# Helpers
#

# Messages
all-message-start:
	@echo "Building all clients..."

all-message-end:
	@echo "All clients built successfully"

build-message-start:
	@echo "Building client..."

build-message-end:
	@echo "Client built successfully"

test-message-start:
	@echo "Testing client..."

test-message-end:
	@echo "Client tests passed successfully"

clean-message-start:
	@echo "Cleaning client..."

clean-message-end:
	@echo "Client cleaned successfully"