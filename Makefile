.PHONY: all build test clean

# Default target
all: build

# Build all clients (currently only nodejs)
build: build-nodejs

# Test all clients (currently only nodejs)
test: test-nodejs

# Clean all builds (currently only nodejs)
clean: clean-nodejs

# Node.js targets
build-nodejs:
	cd clients/nodejs && npm install && npm run build

test-nodejs:
	cd clients/nodejs && npm test

clean-nodejs:
	cd clients/nodejs && npm run clean

# Future targets (commented out)
# build-python:
# 	@echo "Python client not implemented yet"

# build-go:
# 	@echo "Go client not implemented yet"