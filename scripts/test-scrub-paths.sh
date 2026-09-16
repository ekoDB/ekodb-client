#!/usr/bin/env bash
set -euo pipefail

test_root=$(mktemp -d)
report=$test_root/report.md
pipeline_pid=
cleanup() {
    if [[ -n $pipeline_pid ]]; then
        kill "$pipeline_pid" 2>/dev/null || true
        wait "$pipeline_pid" 2>/dev/null || true
    fi
    rm -rf "$test_root"
}
trap cleanup EXIT

scrub_prefix=/private/example/
(
    {
        printf '\033[32m%s\033[0m\n' "${scrub_prefix}first"
        sleep 2
        printf '%s\n' "\\033[31m${scrub_prefix}second\\033[0m"
        printf '%s   \n' "\\u001b[33m${scrub_prefix}third\\u001b[0m"
    } | SCRUB_PREFIX=$scrub_prefix perl -pe 'BEGIN { $| = 1 } s/\Q$ENV{SCRUB_PREFIX}\E//g; s/\e\[[0-?]*[ -\/]*[@-~]//g; s/\\(?:033|u001b)\[[0-9;]*m//g; s/[ \t]+(?=\r?$)//' | tee "$report" >/dev/null
) &
pipeline_pid=$!

# The first line must reach the report while the producer is still sleeping.
for _ in {1..20}; do
    if [[ -f $report ]] && grep -qx first "$report"; then
        break
    fi
    sleep 0.05
done
if [[ ! -f $report ]] || ! grep -qx first "$report"; then
    echo "scrub pipeline buffered live output" >&2
    exit 1
fi

wait "$pipeline_pid"
pipeline_pid=
printf '%s\n' first second third > "$test_root/expected.md"
cmp "$test_root/expected.md" "$report"

echo "Path scrubber streaming test passed."
