#!/bin/bash

# Script to generate comprehensive list of examples across all client libraries
# This helps maintain accurate example counts and provides visibility into example coverage

set -e

# Colors for output
CYAN='\033[36m'
GREEN='\033[32m'
YELLOW='\033[33m'
RED='\033[31m'
RESET='\033[0m'

# Output files
EXAMPLES_JSON="examples_list.json"
EXAMPLES_TXT="examples_list.txt"

# Check if we should generate temporary files for comparison
if [ "$1" = "--temp" ]; then
    EXAMPLES_TXT="examples_list.txt.tmp"
    EXAMPLES_JSON="examples_list.json.tmp"
fi

echo -e "${CYAN}🔍 Generating comprehensive examples list...${RESET}"

# Start JSON structure
echo "[" > "$EXAMPLES_JSON"
echo "# ekoDB Client Examples Inventory" > "$EXAMPLES_TXT"
echo "# Generated: $(date)" >> "$EXAMPLES_TXT"
echo "# " >> "$EXAMPLES_TXT"

# Initialize counters
total_examples=0
total_direct=0
total_client=0

# Function to count and list examples
count_examples() {
    local lang=$1
    local pattern=$2
    local type=$3
    local dir=$4
    
    local count=0
    local examples=()
    
    if [ -d "$dir" ]; then
        while IFS= read -r file; do
            if [ -f "$file" ]; then
                local basename=$(basename "$file")
                local name="${basename%.*}"
                examples+=("$name")
                count=$((count + 1))
                
                # Add to JSON
                if [ $total_examples -gt 0 ]; then
                    echo "," >> "$EXAMPLES_JSON"
                fi
                echo "  {\"language\": \"$lang\", \"name\": \"$name\", \"type\": \"$type\", \"file\": \"$file\"}" >> "$EXAMPLES_JSON"
                total_examples=$((total_examples + 1))
            fi
        done < <(find "$dir" -maxdepth 2 -name "$pattern" -type f 2>/dev/null | sort)
    fi
    
    echo "$count"
}

# Count Direct API Examples
echo -e "${YELLOW}📋 Counting Direct API examples...${RESET}"
echo "" >> "$EXAMPLES_TXT"
echo "## Direct API Examples (Raw HTTP/WebSocket)" >> "$EXAMPLES_TXT"
echo "" >> "$EXAMPLES_TXT"

for lang in javascript python go rust; do
    if [ "$lang" = "rust" ]; then
        dir="examples/rust/examples"
        pattern="*.rs"
    else
        dir="examples/$lang/ttl-caching"
        case $lang in
            javascript) pattern="*.js" ;;
            python) pattern="*.py" ;;
            go) pattern="*.go" ;;
        esac
    fi
    
    # Only count non-client examples
    count=0
    if [ -d "$dir" ]; then
        while IFS= read -r file; do
            if [ -f "$file" ]; then
                basename=$(basename "$file")
                # Skip client_* files
                if [[ ! "$basename" =~ ^client_ ]] && [[ ! "$basename" =~ ^Client ]]; then
                    name="${basename%.*}"
                    count=$((count + 1))
                    
                    # Add to JSON
                    if [ $total_examples -gt 0 ]; then
                        echo "," >> "$EXAMPLES_JSON"
                    fi
                    echo "  {\"language\": \"$lang\", \"name\": \"$name\", \"type\": \"direct\", \"file\": \"$file\"}" >> "$EXAMPLES_JSON"
                    total_examples=$((total_examples + 1))
                    total_direct=$((total_direct + 1))
                fi
            fi
        done < <(find "$dir" -maxdepth 1 -name "$pattern" -type f 2>/dev/null | sort)
    fi
    
    echo "  $lang: $count examples" >> "$EXAMPLES_TXT"
    echo -e "  ${lang}: ${GREEN}$count${RESET} direct examples"
done

echo "" >> "$EXAMPLES_TXT"
echo "  Total Direct: $total_direct" >> "$EXAMPLES_TXT"

# Count Client Library Examples
echo -e "${YELLOW}📚 Counting Client Library examples...${RESET}"
echo "" >> "$EXAMPLES_TXT"
echo "## Client Library Examples" >> "$EXAMPLES_TXT"
echo "" >> "$EXAMPLES_TXT"

# Rust client examples
rust_client_count=$(count_examples "rust" "client_*.rs" "client" "examples/rust/examples")
echo "  rust: $rust_client_count examples" >> "$EXAMPLES_TXT"
echo -e "  rust: ${GREEN}$rust_client_count${RESET} client examples"
total_client=$((total_client + rust_client_count))

# Python client examples
python_client_count=$(count_examples "python" "client_*.py" "client" "examples/python")
echo "  python: $python_client_count examples" >> "$EXAMPLES_TXT"
echo -e "  python: ${GREEN}$python_client_count${RESET} client examples"
total_client=$((total_client + python_client_count))

# Go client examples
go_client_count=$(count_examples "go" "client_*.go" "client" "examples/go")
echo "  go: $go_client_count examples" >> "$EXAMPLES_TXT"
echo -e "  go: ${GREEN}$go_client_count${RESET} client examples"
total_client=$((total_client + go_client_count))

# TypeScript client examples
ts_client_count=$(count_examples "typescript" "client_*.ts" "client" "examples/typescript")
echo "  typescript: $ts_client_count examples" >> "$EXAMPLES_TXT"
echo -e "  typescript: ${GREEN}$ts_client_count${RESET} client examples"
total_client=$((total_client + ts_client_count))

# JavaScript client examples
js_client_count=$(count_examples "javascript" "client_*.js" "client" "examples/javascript")
echo "  javascript: $js_client_count examples" >> "$EXAMPLES_TXT"
echo -e "  javascript: ${GREEN}$js_client_count${RESET} client examples"
total_client=$((total_client + js_client_count))

# Kotlin client examples
kotlin_client_count=$(count_examples "kotlin" "Client*.kt" "client" "examples/kotlin/examples")
echo "  kotlin: $kotlin_client_count examples" >> "$EXAMPLES_TXT"
echo -e "  kotlin: ${GREEN}$kotlin_client_count${RESET} client examples"
total_client=$((total_client + kotlin_client_count))

echo "" >> "$EXAMPLES_TXT"
echo "  Total Client: $total_client" >> "$EXAMPLES_TXT"

# Close JSON array
echo "" >> "$EXAMPLES_JSON"
echo "]" >> "$EXAMPLES_JSON"

# Summary
echo "" >> "$EXAMPLES_TXT"
echo "## Summary" >> "$EXAMPLES_TXT"
echo "" >> "$EXAMPLES_TXT"
total_all=$((total_direct + total_client))
echo "  Total Examples: $total_all" >> "$EXAMPLES_TXT"
echo "  Direct API: $total_direct" >> "$EXAMPLES_TXT"
echo "  Client Libraries: $total_client" >> "$EXAMPLES_TXT"
echo "" >> "$EXAMPLES_TXT"
echo "  Languages: 6 (Rust, Python, Go, TypeScript, JavaScript, Kotlin)" >> "$EXAMPLES_TXT"
echo "  Full-Featured Clients: 5 (Rust, Python, Go, TypeScript, Kotlin)" >> "$EXAMPLES_TXT"

echo -e "${GREEN}✅ Examples list generated!${RESET}"
echo -e "  Total: ${GREEN}$total_all${RESET} examples"
echo -e "  Direct: ${GREEN}$total_direct${RESET} | Client: ${GREEN}$total_client${RESET}"
echo -e "  Files: ${CYAN}$EXAMPLES_TXT${RESET}, ${CYAN}$EXAMPLES_JSON${RESET}"
