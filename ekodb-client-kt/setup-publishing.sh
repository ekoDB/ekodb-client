#!/bin/bash

# Setup script for Maven Central publishing
# This script helps configure your Gradle properties for publishing

set -e

CYAN='\033[36m'
GREEN='\033[32m'
YELLOW='\033[33m'
RED='\033[31m'
RESET='\033[0m'

echo -e "${CYAN}🔧 Maven Central Publishing Setup${RESET}"
echo ""

GRADLE_PROPS="$HOME/.gradle/gradle.properties"

# Check if gradle.properties already exists
if [ -f "$GRADLE_PROPS" ]; then
    echo -e "${YELLOW}⚠️  $GRADLE_PROPS already exists${RESET}"
    echo -e "${YELLOW}We'll append the Maven Central configuration to it${RESET}"
    echo ""
else
    echo -e "${CYAN}Creating new $GRADLE_PROPS${RESET}"
    mkdir -p "$HOME/.gradle"
    touch "$GRADLE_PROPS"
fi

# Prompt for credentials
echo -e "${CYAN}Please enter your Maven Central credentials:${RESET}"
echo ""

read -p "Maven Central Username: " MAVEN_USER
read -sp "Maven Central Token/Password: " MAVEN_PASS
echo ""
read -sp "GPG Passphrase: " GPG_PASS
echo ""
echo ""

# Check if configuration already exists
if grep -q "ossrhUsername" "$GRADLE_PROPS" 2>/dev/null; then
    echo -e "${YELLOW}⚠️  Maven Central configuration already exists in gradle.properties${RESET}"
    read -p "Overwrite? (y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo -e "${YELLOW}Skipping configuration update${RESET}"
        exit 0
    fi
    
    # Remove old configuration
    sed -i.bak '/# Maven Central/,/signing.secretKeyRingFile/d' "$GRADLE_PROPS"
fi

# Append configuration
cat >> "$GRADLE_PROPS" << EOF

# Maven Central Publishing Configuration (ekoDB)
ossrhUsername=$MAVEN_USER
ossrhPassword=$MAVEN_PASS

# GPG Signing
signing.keyId=1EBBB1C3
signing.password=$GPG_PASS
signing.secretKeyRingFile=/Users/tek/.gnupg/secring.gpg
EOF

echo -e "${GREEN}✅ Configuration saved to $GRADLE_PROPS${RESET}"
echo ""
echo -e "${CYAN}Next steps:${RESET}"
echo -e "  1. Test build: ${GREEN}./gradlew clean test build${RESET}"
echo -e "  2. Publish: ${GREEN}cd .. && make deploy-client-kt${RESET}"
echo ""
echo -e "${YELLOW}⚠️  Keep your gradle.properties secure - it contains sensitive credentials!${RESET}"
