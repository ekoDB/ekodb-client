#!/bin/bash

# Script to publish Kotlin client to Maven Central
# This script handles the entire publishing workflow

set -e

# Colors for output
CYAN='\033[36m'
GREEN='\033[32m'
YELLOW='\033[33m'
RED='\033[31m'
RESET='\033[0m'

echo -e "${CYAN}📦 ekoDB Kotlin Client Publishing Script${RESET}"
echo ""

# Change to Kotlin client directory
cd ekodb-client-kt

# Get current version
VERSION=$(grep "^version = " build.gradle.kts | cut -d'"' -f2)
echo -e "${CYAN}Current version: ${GREEN}$VERSION${RESET}"
echo ""

# Create upload directory (clean it first)
UPLOAD_DIR="upload-to-maven"
rm -rf "$UPLOAD_DIR"
mkdir -p "$UPLOAD_DIR"

# Also remove any old bundle ZIPs
rm -f ekodb-client-kt-*-bundle.zip

# Check if credentials are configured
if [ -z "$OSSRH_USERNAME" ] && [ -z "$(grep ossrhUsername ~/.gradle/gradle.properties 2>/dev/null)" ]; then
    echo -e "${RED}❌ Sonatype credentials not found!${RESET}"
    echo -e "${YELLOW}Please configure credentials in ~/.gradle/gradle.properties or set environment variables${RESET}"
    echo -e "${YELLOW}See PUBLISHING.md for setup instructions${RESET}"
    exit 1
fi

# Check if signing is configured
if [ -z "$SIGNING_KEY" ] && [ -z "$(grep signing.keyId ~/.gradle/gradle.properties 2>/dev/null)" ]; then
    echo -e "${YELLOW}⚠️  Warning: GPG signing not configured${RESET}"
    echo -e "${YELLOW}Publications to Maven Central require signing${RESET}"
    echo -e "${YELLOW}See PUBLISHING.md for GPG setup instructions${RESET}"
    echo ""
    read -p "Continue anyway? (y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

# Run tests
echo -e "${CYAN}🧪 Running tests...${RESET}"
./gradlew test
echo -e "${GREEN}✅ Tests passed!${RESET}"
echo ""

# Build and publish to local Maven
echo -e "${CYAN}🛠️  Building and signing artifacts...${RESET}"
./gradlew clean build publishToMavenLocal
echo -e "${GREEN}✅ Build complete!${RESET}"
echo ""

# Build sources and javadoc JARs
echo -e "${CYAN}📦 Building sources and javadoc JARs...${RESET}"
./gradlew sourcesJar javadocJar

# Copy artifacts to upload directory with proper structure
echo -e "${CYAN}📦 Copying artifacts to $UPLOAD_DIR...${RESET}"
LOCAL_REPO="$HOME/.m2/repository/io/ekodb/ekodb-client-kt/$VERSION"
BUILD_LIBS="build/libs"

if [ ! -d "$LOCAL_REPO" ]; then
    echo -e "${RED}❌ Artifacts not found in local Maven repository${RESET}"
    exit 1
fi

# Create proper directory structure: io/ekodb/ekodb-client-kt/0.1.0/
BUNDLE_DIR="$UPLOAD_DIR/io/ekodb/ekodb-client-kt/$VERSION"
mkdir -p "$BUNDLE_DIR"

# Copy main artifacts
cp "$LOCAL_REPO"/*.jar "$BUNDLE_DIR/" 2>/dev/null || true
cp "$LOCAL_REPO"/*.pom "$BUNDLE_DIR/" 2>/dev/null || true
cp "$LOCAL_REPO"/*.module "$BUNDLE_DIR/" 2>/dev/null || true
cp "$LOCAL_REPO"/*.asc "$BUNDLE_DIR/" 2>/dev/null || true

# Copy sources and javadoc from build/libs if not in local repo
cp "$BUILD_LIBS"/*-sources.jar "$BUNDLE_DIR/" 2>/dev/null || true
cp "$BUILD_LIBS"/*-javadoc.jar "$BUNDLE_DIR/" 2>/dev/null || true

# Generate checksums (md5 and sha1)
echo -e "${CYAN}🔐 Generating checksums...${RESET}"
cd "$BUNDLE_DIR"
for file in *.jar *.pom *.module; do
    if [ -f "$file" ]; then
        md5 -q "$file" > "$file.md5"
        shasum -a 1 "$file" | cut -d' ' -f1 > "$file.sha1"
    fi
done
cd - > /dev/null

echo -e "${GREEN}✅ Artifacts prepared with checksums${RESET}"
echo ""

# List files
echo -e "${CYAN}📋 Files in bundle:${RESET}"
ls -lh "$BUNDLE_DIR"
echo ""

# Create bundle ZIP
echo -e "${CYAN}📦 Creating bundle ZIP...${RESET}"
BUNDLE_NAME="ekodb-client-kt-$VERSION-bundle.zip"
cd "$UPLOAD_DIR"
zip -r "../$BUNDLE_NAME" io/
cd ..
echo -e "${GREEN}✅ Bundle created: ${CYAN}$BUNDLE_NAME${RESET}"
echo ""

# Open the directory
open .

echo ""
echo -e "${YELLOW}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${RESET}"
echo -e "${CYAN}📤 Upload Instructions${RESET}"
echo -e "${YELLOW}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${RESET}"
echo ""
echo -e "1. Go to: ${CYAN}https://central.sonatype.com/publishing${RESET}"
echo -e "2. Click ${GREEN}'Publish Component'${RESET}"
echo -e "3. Upload the bundle: ${GREEN}$BUNDLE_NAME${RESET}"
echo -e "4. Click ${GREEN}'Publish'${RESET}"
echo ""
echo -e "${YELLOW}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${RESET}"
echo ""
echo -e "${CYAN}After publishing:${RESET}"
echo -e "  • Wait ~10 minutes for Maven Central search"
echo -e "  • Wait ~2 hours for full mirror sync"
echo -e "  • Verify: ${CYAN}https://central.sonatype.com/artifact/io.ekodb/ekodb-client-kt${RESET}"
echo -e "  • Create tag: ${CYAN}git tag kt-v$VERSION && git push origin kt-v$VERSION${RESET}"
echo ""
echo -e "${GREEN}🎉 Ready to publish version $VERSION!${RESET}"
