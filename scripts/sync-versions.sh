#!/bin/bash

# Enforce a single source-of-truth version across every client language and every
# README install snippet.
#
# Source of truth: the Rust workspace client version in ekodb_client/Cargo.toml.
# This script is NON-interactive and IDEMPOTENT — it does not choose or bump a
# version, it only propagates the one already declared in the Rust manifest. That
# is what makes it safe to run from `make fmt`: on a synced tree it changes
# nothing, and any drift (a downstream manifest or a README snippet left behind by
# a release) shows up as a diff instead of shipping stale.
#
# To CHANGE the released version, edit ekodb_client/Cargo.toml (or run the
# interactive `make bump-version`) and then run this / `make fmt`.
#
# Note: version literals inside README code fences (```kotlin ... ```) cannot use
# HTML-comment markers the way prose stats do — a comment renders literally inside
# a code block — so these are matched by their package-coordinate shape instead.
# The npm and crates.io shields badges are omitted on purpose: shields fetches the
# live published version, so they are already dynamic.

set -e

CYAN='\033[36m'
GREEN='\033[32m'
YELLOW='\033[33m'
RED='\033[31m'
RESET='\033[0m'

SOT_FILE="ekodb_client/Cargo.toml"

if [ ! -f "$SOT_FILE" ]; then
    echo -e "${RED}❌ Source-of-truth manifest not found: $SOT_FILE${RESET}"
    echo -e "${YELLOW}   Run this from the ekodb-client repo root.${RESET}"
    exit 1
fi

VERSION=$(awk -F'"' '/^version = /{print $2; exit}' "$SOT_FILE")

# Never let an extraction failure blank out every version file.
if ! echo "$VERSION" | grep -qE '^[0-9]+\.[0-9]+\.[0-9]+([-.][0-9A-Za-z.]+)?$'; then
    echo -e "${RED}❌ Could not read a valid version from $SOT_FILE (got: '$VERSION')${RESET}"
    exit 1
fi

echo -e "${CYAN}🔢 Syncing all clients to source-of-truth version ${GREEN}$VERSION${CYAN} (from $SOT_FILE)${RESET}"

# --- Downstream language manifests -----------------------------------------
# Each sed is anchored so it only touches the manifest's own version field (and,
# for Python, the path-dependency pin on ekodb_client), never a transitive dep.
set_manifest() { # $1=file  $2=sed-expression
    [ -f "$1" ] || { echo -e "${RED}❌ Missing $1${RESET}"; exit 1; }
    sed -i.bak -E "$2" "$1"
    rm -f "$1.bak"
}

set_manifest "ekodb-client-py/Cargo.toml"     "s/^version = \"[^\"]*\"/version = \"$VERSION\"/"
set_manifest "ekodb-client-py/Cargo.toml"     "s/(ekodb_client = \{ version = \")[^\"]*(\")/\1$VERSION\2/"
set_manifest "ekodb-client-py/pyproject.toml"  "s/^version = \"[^\"]*\"/version = \"$VERSION\"/"
set_manifest "ekodb-client-ts/package.json"    "s/(\"version\"[[:space:]]*:[[:space:]]*\")[^\"]*(\")/\1$VERSION\2/"
set_manifest "ekodb-client-kt/build.gradle.kts" "s/^version = \"[^\"]*\"/version = \"$VERSION\"/"

# --- README install snippets (matched by package coordinate) ----------------
sync_readme() { # $1=file  $2=sed-expression
    [ -f "$1" ] || return 0
    sed -i.bak -E "$2" "$1"
    rm -f "$1.bak"
}

# Kotlin Maven coordinate: io.ekodb:ekodb-client-kt:X.Y.Z (Kotlin + Groovy DSL).
for f in README.md ekodb-client-kt/README.md; do
    sync_readme "$f" "s/(ekodb-client-kt:)[0-9]+\.[0-9]+\.[0-9]+([-.][0-9A-Za-z.]+)?/\1$VERSION/g"
done

# Rust crate install: ekodb_client = "X[.Y[.Z]]"  (the "{ version = ... }" form in
# manifests is a different shape and is handled by set_manifest above).
sync_readme "ekodb_client/README.md" "s/(ekodb_client = \")[0-9]+(\.[0-9]+)+(\")/\1$VERSION\3/g"

echo -e "${GREEN}✅ All client manifests and README install snippets are at $VERSION${RESET}"
