#!/usr/bin/env bash
set -euo pipefail
root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
fixture=$(mktemp -d "${TMPDIR:-/tmp}/ekodb-bump-test.XXXXXX")
results=$(mktemp -d "${TMPDIR:-/tmp}/ekodb-bump-results.XXXXXX")
trap 'rm -rf "$fixture" "$results"' EXIT
mkdir -p "$fixture/scripts" "$fixture/ekodb_client/src" "$fixture/ekodb-client-py/src" "$fixture/ekodb-client-ts" "$fixture/ekodb-client-kt"
cp "$root/Makefile" "$fixture/Makefile"
cp "$root/scripts/sync-versions.sh" "$fixture/scripts/sync-versions.sh"
cat > "$fixture/Cargo.toml" <<'TOML'
[workspace]
members = ["ekodb_client"]
exclude = ["ekodb-client-py"]
resolver = "2"
TOML
cat > "$fixture/ekodb_client/Cargo.toml" <<'TOML'
[package]
name = "ekodb_client"
version = "1.2.3"
edition = "2021"
TOML
cat > "$fixture/ekodb-client-py/Cargo.toml" <<'TOML'
[package]
name = "synthetic-python-binding"
version = "1.2.3"
edition = "2021"
[dependencies]
ekodb_client = { version = "1.2.3", path = "../ekodb_client" }
TOML
printf '' > "$fixture/ekodb_client/src/lib.rs"
printf '' > "$fixture/ekodb-client-py/src/lib.rs"
printf 'version = "1.2.3"\n' > "$fixture/ekodb-client-py/pyproject.toml"
printf 'version = "1.2.3"\n' > "$fixture/ekodb-client-kt/build.gradle.kts"
printf '{"name":"synthetic-sdk","version":"1.2.3"}\n' > "$fixture/ekodb-client-ts/package.json"
printf '{"name":"synthetic-sdk","version":"1.2.3","lockfileVersion":3,"packages":{"":{"name":"synthetic-sdk","version":"1.2.3"}}}\n' > "$fixture/ekodb-client-ts/package-lock.json"
printf '# Changelog\n\n## [Unreleased]\n\n- Synthetic behavior change.\n\n## [1.2.3] - 2000-01-01\n\n- Earlier change.\n' > "$fixture/ekodb_client/CHANGELOG.md"
# Invalid input must fail before changing any fixture file, including the notes.
snapshot() {
    python3 - "$fixture" <<'SNAPSHOT'
import hashlib, json, pathlib, sys
root = pathlib.Path(sys.argv[1])
print(json.dumps({str(p.relative_to(root)): hashlib.sha256(p.read_bytes()).hexdigest()
                  for p in sorted(root.rglob('*')) if p.is_file()}, sort_keys=True))
SNAPSHOT
}
invalid_versions=('invalid' '1.2.3-..' '01.2.3' '1.02.3' '1.2.03' '1.2.3-01' '1.2.3-rc.01' '1.2.3-' '1.2.3-rc..1')
failures=0
for version in "${invalid_versions[@]}"; do
    snapshot > "$results/before.json"
    if make --silent -C "$fixture" bump-version VERSION="$version" >"$results/output" 2>&1; then
        echo "bump-version accepted invalid version: $version" >&2
        failures=$((failures + 1))
    fi
    snapshot > "$results/after.json"
    if ! cmp -s "$results/before.json" "$results/after.json"; then
        echo "bump-version changed files for invalid version: $version" >&2
        failures=$((failures + 1))
    fi
    # Restore the synthetic source after detecting a pre-validation mutation.
    sed -i.bak 's/^version = "[^"]*"/version = "1.2.3"/' "$fixture/ekodb_client/Cargo.toml"
    rm -f "$fixture/ekodb_client/Cargo.toml.bak"
done
[ "$failures" -eq 0 ] || exit 1
# Validation-only mode must work without a source manifest and touch no files.
mkdir -p "$results/no-manifests"
(cd "$results/no-manifests" && bash "$root/scripts/sync-versions.sh" --check-version 3.0.0-rc-1)
[ -z "$(find "$results/no-manifests" -type f -print)" ]
# Normal synchronization rejects an invalid source without changing downstreams.
for version in "${invalid_versions[@]}"; do
    sed -i.bak "s/^version = \"[^\"]*\"/version = \"$version\"/" "$fixture/ekodb_client/Cargo.toml"
    rm -f "$fixture/ekodb_client/Cargo.toml.bak"
    snapshot > "$results/before.json"
    if make --silent -C "$fixture" sync-versions > "$results/output" 2>&1; then
        echo "sync-versions accepted invalid version: $version" >&2
        exit 1
    fi
    snapshot > "$results/after.json"
    cmp -s "$results/before.json" "$results/after.json"
done
sed -i.bak 's/^version = "[^"]*"/version = "1.2.3"/' "$fixture/ekodb_client/Cargo.toml"
rm -f "$fixture/ekodb_client/Cargo.toml.bak"
# A non-UTC local timezone must not control the generated release date.
output=$(TZ=Pacific/Honolulu make --silent -C "$fixture" bump-version VERSION=2.0.0)
grep -Fq "## [2.0.0] - $(date -u +%Y-%m-%d)" "$fixture/ekodb_client/CHANGELOG.md"
grep -Fq '## [1.2.3] - 2000-01-01' "$fixture/ekodb_client/CHANGELOG.md"
! grep -Fq '## [Unreleased]' "$fixture/ekodb_client/CHANGELOG.md"
printf '%s\n' "$output" | grep -Fq 'Suggested commit subject: chore(*): v2.0.0'
python3 - "$fixture" <<'PY'
import json, pathlib, sys
root = pathlib.Path(sys.argv[1])
for name in ('ekodb_client/Cargo.toml', 'ekodb-client-py/Cargo.toml', 'ekodb-client-py/pyproject.toml', 'ekodb-client-kt/build.gradle.kts'):
    assert 'version = "2.0.0"' in (root / name).read_text(), name
assert json.loads((root / 'ekodb-client-ts/package.json').read_text())['version'] == '2.0.0'
assert json.loads((root / 'ekodb-client-ts/package-lock.json').read_text())['packages']['']['version'] == '2.0.0'
assert 'version = "2.0.0"' in (root / 'Cargo.lock').read_text()
assert 'version = "2.0.0"' in (root / 'ekodb-client-py/Cargo.lock').read_text()
PY
# The default is the next patch, still generated through the same target.
printf '## [Unreleased]\n\n- Next synthetic change.\n' > "$fixture/ekodb_client/CHANGELOG.md"
output=$(make --silent -C "$fixture" bump-version)
printf '%s\n' "$output" | grep -Fq 'Suggested commit subject: chore(*): v2.0.1'
grep -Fq 'version = "2.0.1"' "$fixture/ekodb_client/Cargo.toml"
# Every supported suffix must synchronize in full, including replacing a previous
# hyphenated prerelease in README examples and remaining idempotent on repetition.
mkdir -p "$fixture/ekodb-client-kt"
printf 'implementation("io.ekodb:ekodb-client-kt:2.0.1")\n' > "$fixture/README.md"
printf 'ekodb_client = "2.0.1"\n' > "$fixture/ekodb_client/README.md"
cat > "$fixture/ekodb-client-kt/README.md" <<'README'
implementation("io.ekodb:ekodb-client-kt:2.0.1")
<artifactId>ekodb-client-kt</artifactId>
<version>2.0.1</version>
README
for version in 2.1.0-1 2.1.0-rc.1 2.1.0-rc-1 2.1.0-alpha-1.beta 2.1.0; do
    printf '## [Unreleased]\n\n- Next synthetic change.\n' > "$fixture/ekodb_client/CHANGELOG.md"
    output=$(make --silent -C "$fixture" bump-version VERSION="$version")
    python3 - "$fixture" "$version" <<'VERSIONS'
import json, pathlib, sys
root, version = pathlib.Path(sys.argv[1]), sys.argv[2]
for name in ('ekodb_client/Cargo.toml', 'ekodb-client-py/Cargo.toml', 'ekodb-client-py/pyproject.toml', 'ekodb-client-kt/build.gradle.kts', 'Cargo.lock', 'ekodb-client-py/Cargo.lock'):
    assert f'version = "{version}"' in (root / name).read_text(), name
assert json.loads((root / 'ekodb-client-ts/package.json').read_text())['version'] == version
assert json.loads((root / 'ekodb-client-ts/package-lock.json').read_text())['packages']['']['version'] == version
assert (root / 'README.md').read_text() == f'implementation("io.ekodb:ekodb-client-kt:{version}")\n'
assert (root / 'ekodb_client/README.md').read_text() == f'ekodb_client = "{version}"\n'
assert (root / 'ekodb-client-kt/README.md').read_text() == f'implementation("io.ekodb:ekodb-client-kt:{version}")\n<artifactId>ekodb-client-kt</artifactId>\n<version>{version}</version>\n'
VERSIONS
    snapshot > "$results/before.json"
    make --silent -C "$fixture" sync-versions > "$results/output" 2>&1
    snapshot > "$results/after.json"
    cmp -s "$results/before.json" "$results/after.json"
    if [[ "$version" == *-* ]]; then prefix='pre-release '; else prefix=''; fi
    printf '%s\n' "$output" | grep -Fq "Suggested commit subject: chore(*): ${prefix}v${version}"
done
echo 'Version bump fixture checks passed (invalid input unchanged, explicit/default/prerelease versions, UTC notes, manifests, README synchronization, generated locks).'
