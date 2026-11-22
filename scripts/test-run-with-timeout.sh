#!/usr/bin/env bash
set -euo pipefail

root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
timeout_helper=$root/scripts/run-with-timeout.pl

# Timed examples are non-interactive. Even when the caller has readable stdin,
# the wrapped command must see EOF so a terminal read cannot suspend its process
# group after the example has otherwise completed.
printf 'unexpected input\n' | perl "$timeout_helper" 5 bash -c '
    if IFS= read -r _line; then
        exit 1
    fi
'

echo "Timeout helper stdin-detachment test passed."
