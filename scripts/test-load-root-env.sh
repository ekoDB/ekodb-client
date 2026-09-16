#!/usr/bin/env bash
set -euo pipefail

source_loader=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/load-root-env.sh
test_root=$(mktemp -d)
trap 'rm -rf "$test_root"' EXIT
mkdir -p "$test_root/scripts"
cp "$source_loader" "$test_root/scripts/load-root-env.sh"

printf '%s\n' 'API_BASE_KEY=file-key' 'API_BASE_URL=http://file' 'WS_BASE_URL=ws://file' > "$test_root/.env"
(
    unset API_BASE_KEY API_BASE_URL WS_BASE_URL
    source "$test_root/scripts/load-root-env.sh"
    [[ $API_BASE_KEY == file-key && $API_BASE_URL == http://file && $WS_BASE_URL == ws://file ]]
    [[ $- != *a* ]]
)
(
    export API_BASE_KEY=caller-key API_BASE_URL=http://caller WS_BASE_URL=ws://caller
    set -a
    source "$test_root/scripts/load-root-env.sh"
    [[ $API_BASE_KEY == caller-key && $API_BASE_URL == http://caller && $WS_BASE_URL == ws://caller ]]
    [[ $- == *a* ]]
)

rm "$test_root/.env"
(
    export API_BASE_KEY=injected-key
    unset API_BASE_URL WS_BASE_URL
    source "$test_root/scripts/load-root-env.sh"
    [[ $API_BASE_URL == http://localhost:8080 && $WS_BASE_URL == ws://localhost:8080 ]]
)
if (unset API_BASE_KEY API_BASE_URL WS_BASE_URL; source "$test_root/scripts/load-root-env.sh"); then
    echo "missing API_BASE_KEY unexpectedly succeeded" >&2
    exit 1
fi

printf '%s\n' 'return 7' > "$test_root/.env"
if (export API_BASE_KEY=injected-key; source "$test_root/scripts/load-root-env.sh"); then
    echo "failing .env unexpectedly succeeded" >&2
    exit 1
fi

echo "Root environment loader tests passed."
