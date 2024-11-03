.PHONY: all build test clean

# Default target
all: build

# Build all clients
build: build-nodejs build-python build-go

# Test all clients
test: test-nodejs test-python test-go

# Clean all builds
clean: clean-nodejs clean-python clean-go

# Node.js targets
build-nodejs:
	cd clients/nodejs && npm install && npm run build

test-nodejs:
	cd clients/nodejs && npm test

clean-nodejs:
	cd clients/nodejs && npm run clean

# Python targets
build-python:
	cd clients/python && python3 -m pip install -e ".[dev]"

test-python:
	cd clients/python && python3 -m pytest tests/ --cov=ekodb

clean-python:
	cd clients/python && rm -rf dist/ build/ *.egg-info/ .pytest_cache/ .coverage htmlcov/

# Go targets
build-go:
	cd clients/go && go build ./...

test-go:
	cd clients/go && go test -v -race -cover ./...

clean-go:
	cd clients/go && go clean
	rm -rf clients/go/dist