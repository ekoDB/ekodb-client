.PHONY: all build test clean integration-test

# Default target
all: all-message-start clean build test all-message-end

# Build all clients
build: build-message-start build-nodejs build-python build-go build-message-end

# Test all clients
test: test-message-start test-nodejs test-python test-go test-message-end

# Integration tests for all clients
integration-test: integration-test-message-start integration-test-nodejs integration-test-python integration-test-go integration-test-message-end

# Clean all builds
clean: clean-message-start clean-nodejs clean-python clean-go clean-message-end

# Node.js targets
build-nodejs:
	cd clients/nodejs && npm install && npm run build && echo "Node.js client built successfully"

test-nodejs:
	cd clients/nodejs && npm test && echo "Node.js client tests passed successfully"

integration-test-nodejs:
	@echo "Running Node.js integration tests..."
	cd clients/nodejs/examples && \
	npm install && \
	npm run test && \
	echo "Node.js integration tests passed successfully"

clean-nodejs:
	cd clients/nodejs && npm run clean && echo "Node.js client cleaned successfully"
	rm -rf clients/nodejs/examples/node_modules clients/nodejs/examples/dist

# Python targets
build-python:
	cd clients/python && python3 -m pip install -e ".[dev]" && echo "Python client built successfully"

test-python:
	cd clients/python && python3 -m pytest tests/ --cov=ekodb && echo "Python client tests passed successfully"

integration-test-python:
	@echo "Running Python integration tests..."
	cd clients/python/examples && \
	python3 -m venv venv && \
	. venv/bin/activate && \
	pip install -e .. && \
	python test.py && \
	echo "Python integration tests passed successfully"

clean-python:
	cd clients/python && rm -rf dist/ build/ *.egg-info/ .pytest_cache/ .coverage htmlcov/ && echo "Python client cleaned successfully"
	rm -rf clients/python/examples/venv clients/python/examples/__pycache__ clients/python/examples/*.pyc

# Go targets
build-go:
	cd clients/go && make build && echo "Go client built successfully"

test-go:
	cd clients/go && make test && echo "Go client tests passed successfully"

integration-test-go:
	@echo "Running Go integration tests..."
	cd clients/go/examples && \
	go mod tidy && \
	go run main.go && \
	echo "Go integration tests passed successfully"

clean-go:
	cd clients/go && make clean && echo "Go client cleaned successfully"
	rm -rf clients/go/examples/tmp

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

integration-test-message-start:
	@echo "Running integration tests against EkoDB at http://0.0.0.0:8080..."

integration-test-message-end:
	@echo "All integration tests completed successfully"