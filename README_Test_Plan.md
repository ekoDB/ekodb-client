# EkoDB Client Test Plan

## 1. Local Testing Environment

### Setup Mock EkoDB Server

Run ekodb on 0.0.0.0:8080 from local server

### Test Data Setup

Create consistent test data across all clients:

- Users collection with basic CRUD operations
- Complex queries with filters
- Batch operations
- Error scenarios

## 2. Go Client Testing

### Unit Tests (Go)

```bash
cd clients/go/ekodb
go test -v -race -coverprofile=coverage.out ./...
go tool cover -html=coverage.out -o coverage.html
```

### Integration Tests (Go)

1. Create a new directory `clients/go/ekodb/integration_tests`
2. Add integration test scenarios:
   - Authentication flow
   - CRUD operations with real data types
   - Concurrent operations
   - Error handling
   - Connection timeout/retry

## 3. Node.js Client Testing

### Unit Tests (NodeJS)

```bash
cd clients/nodejs
npm test
```

### Integration Tests (NodeJS)

1. Create `clients/nodejs/__tests__/integration/`
2. Add integration test files:
   - `auth.test.ts`
   - `crud.test.ts`
   - `batch.test.ts`
   - `error.test.ts`

### Type Testing (NodeJS)

```bash
npm run build
# Verify TypeScript definitions
npx tsd
```

## 4. Python Client Testing

### Unit Tests (Python)

```bash
cd clients/python
pytest tests/ --cov=ekodb --cov-report=html
```

### Integration Tests (Python)

1. Create `clients/python/tests/integration/`
2. Add async integration tests:
   - `test_auth.py`
   - `test_crud.py`
   - `test_batch.py`
   - `test_error.py`

### Type Checking (Python)

```bash
mypy ekodb
```

## 5. Cross-Client Verification

### Consistency Tests

Create identical operations across all clients and verify:

- Data format consistency
- Error handling consistency
- Performance characteristics
- Authentication behavior

### Load Testing

Use k6 or similar to test:

- Concurrent connections
- Request throughput
- Error rates
- Response times

## 6. Documentation Testing

Verify for each client:

- README examples work
- API documentation is complete
- Type definitions are documented
- Error scenarios are documented

## 7. Package Publishing Test

Test package installation from local build:

### Go

```bash
cd clients/go
go mod tidy
go list -m github.com/yourusername/ekodb-client/go@latest
```

### Node.js

```bash
cd clients/nodejs
npm pack
npm install -g ekodb-client-nodejs-1.0.0.tgz
```

### Python

```bash
cd clients/python
pip install -e .
python -c "import ekodb"
```
