#!/bin/bash

# Script to update README.md with current example counts from examples_list.txt
# This ensures the badge always shows the correct number of examples

set -e

# Colors for output
CYAN='\033[36m'
GREEN='\033[32m'
YELLOW='\033[33m'
RED='\033[31m'
RESET='\033[0m'

README="README.md"
EXAMPLES_LIST="examples_list.txt"

echo -e "${CYAN}🔄 Updating README with current example statistics...${RESET}"

# Check if examples_list.txt exists
if [ ! -f "$EXAMPLES_LIST" ]; then
    echo -e "${RED}❌ $EXAMPLES_LIST not found!${RESET}"
    echo -e "${YELLOW}Run 'make examples-ls' first to generate the examples list${RESET}"
    exit 1
fi

# Extract counts from examples_list.txt
total_examples=$(grep "Total Examples:" "$EXAMPLES_LIST" | awk '{print $3}')
total_direct=$(grep "Direct API:" "$EXAMPLES_LIST" | awk '{print $3}')
total_client=$(grep "Client Libraries:" "$EXAMPLES_LIST" | awk '{print $3}')

echo -e "  Found counts:"
echo -e "    Total: ${GREEN}$total_examples${RESET}"
echo -e "    Direct: ${GREEN}$total_direct${RESET}"
echo -e "    Client: ${GREEN}$total_client${RESET}"

# Update the badge in README.md
if [ -f "$README" ]; then
    # Update Examples badge
    sed -i.bak "s/Examples-[0-9]*_Working/Examples-${total_examples}_Working/g" "$README"
    
    # Update Quick Stats section
    sed -i.bak "s/\*\*[0-9]* working examples\*\*/\*\*$total_examples working examples\*\*/g" "$README"
    sed -i.bak "s/[0-9]* client library + [0-9]* direct API examples/$total_client client library + $total_direct direct API examples/g" "$README"
    
    # Remove backup file
    rm -f "${README}.bak"
    
    echo -e "${GREEN}✅ README.md updated successfully!${RESET}"
    echo -e "  Badge: Examples-${total_examples}_Working"
    echo -e "  Stats: $total_examples total ($total_client client + $total_direct direct)"
else
    echo -e "${RED}❌ $README not found!${RESET}"
    exit 1
fi
