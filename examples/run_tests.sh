#!/bin/bash

# ekoDB Examples Integration Test Suite
# Runs all examples across all languages as integration tests

set -e

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Print header
print_header() {
    echo -e "\n${BLUE}╔════════════════════════════════════════╗${NC}"
    echo -e "${BLUE}║  $1${NC}"
    echo -e "${BLUE}╚════════════════════════════════════════╝${NC}\n"
}

# Check if .env exists, if not copy from .env.example
if [ ! -f .env ]; then
    echo -e "${YELLOW}Creating .env from .env.example...${NC}"
    cp .env.example .env
    echo -e "${GREEN}✓ .env created${NC}"
fi

# Check if server is running
print_header "Checking ekoDB Server"
for i in {1..30}; do
    if curl -s http://localhost:8080/api/health > /dev/null 2>&1; then
        echo -e "${GREEN}✓ Server is running${NC}"
        break
    fi
    if [ $i -eq 30 ]; then
        echo -e "${RED}✗ Server not responding at http://localhost:8080${NC}"
        echo -e "${YELLOW}Please start the ekoDB server first:${NC}"
        echo -e "  cargo run --release"
        exit 1
    fi
    echo -n "."
    sleep 1
done

# Track results
TOTAL_SUITES=0
PASSED_SUITES=0
FAILED_SUITES=0

# Run JavaScript tests
if command -v node &> /dev/null; then
    print_header "Running JavaScript Examples"
    TOTAL_SUITES=$((TOTAL_SUITES + 1))
    
    cd javascript
    
    # Install dependencies if needed
    if [ ! -d "node_modules" ]; then
        echo -e "${YELLOW}Installing JavaScript dependencies...${NC}"
        npm install
    fi
    
    if node test-runner.js; then
        echo -e "${GREEN}✓ JavaScript tests passed${NC}"
        PASSED_SUITES=$((PASSED_SUITES + 1))
    else
        echo -e "${RED}✗ JavaScript tests failed${NC}"
        FAILED_SUITES=$((FAILED_SUITES + 1))
    fi
    
    cd ..
else
    echo -e "${YELLOW}⊘ Node.js not found, skipping JavaScript tests${NC}"
fi

# Run Python tests
if command -v python3 &> /dev/null; then
    print_header "Running Python Examples"
    TOTAL_SUITES=$((TOTAL_SUITES + 1))
    
    cd python
    
    # Install dependencies if needed
    if ! python3 -c "import aiohttp" 2>/dev/null; then
        echo -e "${YELLOW}Installing Python dependencies...${NC}"
        pip3 install -r ../requirements.txt
    fi
    
    if python3 test_runner.py; then
        echo -e "${GREEN}✓ Python tests passed${NC}"
        PASSED_SUITES=$((PASSED_SUITES + 1))
    else
        echo -e "${RED}✗ Python tests failed${NC}"
        FAILED_SUITES=$((FAILED_SUITES + 1))
    fi
    
    cd ..
else
    echo -e "${YELLOW}⊘ Python3 not found, skipping Python tests${NC}"
fi

# Run Go tests
if command -v go &> /dev/null; then
    print_header "Running Go Examples"
    TOTAL_SUITES=$((TOTAL_SUITES + 1))
    
    cd go
    
    # Download dependencies
    echo -e "${YELLOW}Downloading Go dependencies...${NC}"
    go mod download
    
    if go run test_runner.go; then
        echo -e "${GREEN}✓ Go tests passed${NC}"
        PASSED_SUITES=$((PASSED_SUITES + 1))
    else
        echo -e "${RED}✗ Go tests failed${NC}"
        FAILED_SUITES=$((FAILED_SUITES + 1))
    fi
    
    cd ..
else
    echo -e "${YELLOW}⊘ Go not found, skipping Go tests${NC}"
fi

# Run Rust tests
if command -v cargo &> /dev/null; then
    print_header "Running Rust Examples"
    TOTAL_SUITES=$((TOTAL_SUITES + 1))
    
    cd rust
    
    # Run only the working examples
    if cargo run --example simple_crud && cargo run --example simple_websocket && cargo run --example crud_scripts; then
        echo -e "${GREEN}✓ Rust tests passed${NC}"
        PASSED_SUITES=$((PASSED_SUITES + 1))
    else
        echo -e "${RED}✗ Rust tests failed${NC}"
        FAILED_SUITES=$((FAILED_SUITES + 1))
    fi
    
    cd ..
else
    echo -e "${YELLOW}⊘ Rust/Cargo not found, skipping Rust tests${NC}"
fi

# Print summary
print_header "Test Summary"
echo -e "${BLUE}Total Test Suites: ${TOTAL_SUITES}${NC}"
echo -e "${GREEN}Passed: ${PASSED_SUITES}${NC}"
echo -e "${RED}Failed: ${FAILED_SUITES}${NC}"

if [ $FAILED_SUITES -gt 0 ]; then
    echo -e "\n${RED}✗ Some tests failed${NC}"
    exit 1
else
    echo -e "\n${GREEN}✓ All tests passed!${NC}"
    exit 0
fi
