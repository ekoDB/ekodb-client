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
total_languages=$(grep "Languages:" "$EXAMPLES_LIST" | grep -o '[0-9]\+' | head -1)
total_clients=$(grep "Full-Featured Clients:" "$EXAMPLES_LIST" | grep -o '[0-9]\+' | head -1)
# Per-client example count (a single number, or a MIN-MAX range if the clients
# ever diverge from parity). Kept as the emitted string so a range flows through.
per_client=$(grep "Examples Per Client:" "$EXAMPLES_LIST" | awk -F': ' '{print $2}' | tr -d ' ')

# Fail fast if any value came back empty (e.g. a line was renamed in
# examples_list.txt) — otherwise the sed steps below would rewrite the README
# stats and badges with blanks. set -e does not catch this on its own.
if [ -z "$total_examples" ] || [ -z "$total_direct" ] || [ -z "$total_client" ] || \
   [ -z "$total_languages" ] || [ -z "$total_clients" ] || [ -z "$per_client" ]; then
    echo -e "${RED}❌ Failed to parse one or more counts from $EXAMPLES_LIST (a line may have been renamed).${RESET}"
    echo -e "${YELLOW}   Aborting so the README is not rewritten with blanks — run 'make examples-ls' to regenerate.${RESET}"
    exit 1
fi

echo -e "  Found counts:"
echo -e "    Total: ${GREEN}$total_examples${RESET}"
echo -e "    Direct: ${GREEN}$total_direct${RESET}"
echo -e "    Client: ${GREEN}$total_client${RESET}"
echo -e "    Languages: ${GREEN}$total_languages${RESET}"
echo -e "    Clients: ${GREEN}$total_clients${RESET}"

# Rewrite prose numbers between HTML-comment markers (<!--ex:NAME-->value<!--
# /ex:NAME-->). Markers render invisibly and make injection wording-independent,
# so re-phrasing a Quick Stats line can never orphan the updater. (The previous
# broad regexes — e.g. s/**N working examples**/ — would silently stop matching
# the moment that phrasing changed, drifting the counts. This is the same
# marker convention used by the sibling test-inventory updater.)
inject_stat() { # $1=file  $2=marker-name  $3=value
    sed -i.bak -E "s|<!--$2-->[^<]*<!--/$2-->|<!--$2-->$3<!--/$2-->|g" "$1"
    rm -f "$1.bak"
}

if [ -f "$README" ]; then
    # Badges live inside shields.io image URLs, which cannot hold a comment
    # marker, so they are still matched by their fixed URL shape.
    sed -i.bak "s/Examples-[0-9]*_Working/Examples-${total_examples}_Working/g" "$README"
    sed -i.bak "s/Languages-[0-9]*-blue/Languages-${total_languages}-blue/g" "$README"
    sed -i.bak "s/Client_Libraries-[0-9]*-purple/Client_Libraries-${total_clients}-purple/g" "$README"
    rm -f "${README}.bak"

    # Prose Quick Stats numbers via markers.
    inject_stat "$README" "ex:total"     "$total_examples"
    inject_stat "$README" "ex:client"    "$total_client"
    inject_stat "$README" "ex:direct"    "$total_direct"
    inject_stat "$README" "ex:languages" "$total_languages"
    inject_stat "$README" "ex:clients"   "$total_clients"
    inject_stat "$README" "ex:perclient" "$per_client"

    echo -e "${GREEN}✅ README.md updated successfully!${RESET}"
    echo -e "  Badge: Examples-${total_examples}_Working"
    echo -e "  Stats: $total_examples total ($total_client client + $total_direct direct)"
else
    echo -e "${RED}❌ $README not found!${RESET}"
    exit 1
fi
