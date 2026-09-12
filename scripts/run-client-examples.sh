#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 2 ]]; then
    echo "usage: $0 <check|run> <rust|python|go|typescript|javascript|kotlin>" >&2
    exit 2
fi

mode=$1
language=$2
root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)

run_go() {
    local go_dir="$root/examples/go"
    local go_client_dir=${EKODB_CLIENT_GO_DIR:-}
    if [[ -z "$go_client_dir" ]]; then
        local common_dir canonical_root
        common_dir=$(git -C "$root" rev-parse --path-format=absolute --git-common-dir)
        canonical_root=$(dirname "$common_dir")
        go_client_dir="$(dirname "$canonical_root")/ekodb-client-go"
    fi
    if [[ ! -f "$go_client_dir/go.mod" ]]; then
        echo "Go client checkout not found at $go_client_dir; set EKODB_CLIENT_GO_DIR to its worktree." >&2
        exit 1
    fi

    local temp_base modfile sumfile output_dir
    temp_base=$(mktemp "$go_dir/.ekodb-examples.XXXXXX")
    modfile="$temp_base.mod"
    sumfile="${modfile%.mod}.sum"
    mv "$temp_base" "$modfile"
    cp "$go_dir/go.mod" "$modfile"
    cp "$go_dir/go.sum" "$sumfile"
    output_dir=$(mktemp -d)
    trap "rm -f '$modfile' '$sumfile'; rm -rf '$output_dir'" EXIT
    (cd "$go_dir" && go mod edit -modfile="$(basename "$modfile")" \
        -replace="github.com/ekoDB/ekodb-client-go=$go_client_dir")

    local source name
    for source in "$go_dir"/client_*.go "$go_dir"/bypass_ripple_example.go "$go_dir"/projection_example.go; do
        name=$(basename "${source%.go}")
        if [[ "$mode" == "check" ]]; then
            (cd "$go_dir" && go build -modfile="$(basename "$modfile")" -o "$output_dir/$name" "$(basename "$source")")
        else
            (cd "$go_dir" && go run -modfile="$(basename "$modfile")" "$(basename "$source")")
        fi
    done
}

case "$mode:$language" in
    check:rust)
        (cd "$root/examples/rust" && cargo check --examples)
        ;;
    run:rust)
        for source in "$root"/examples/rust/examples/client_*.rs \
            "$root"/examples/rust/examples/bypass_ripple_example.rs \
            "$root"/examples/rust/examples/projection_example.rs; do
            (cd "$root/examples/rust" && cargo run --example "$(basename "${source%.rs}")")
        done
        ;;
    check:python)
        python3 -m compileall -q "$root/examples/python"
        ;;
    run:python)
        python_bin=${VENV_PY:-"$root/.venv/bin/python"}
        for source in "$root"/examples/python/client_*.py \
            "$root"/examples/python/bypass_ripple_example.py \
            "$root"/examples/python/projection_example.py; do
            "$python_bin" "$source"
        done
        ;;
    check:go|run:go)
        run_go
        ;;
    check:typescript)
        (cd "$root/examples/typescript" && npm run build)
        ;;
    run:typescript)
        for source in "$root"/examples/typescript/client_*.ts \
            "$root"/examples/typescript/bypass_ripple_example.ts \
            "$root"/examples/typescript/projection_example.ts; do
            (cd "$root/examples/typescript" && npx tsx "$(basename "$source")")
        done
        ;;
    check:javascript)
        for source in "$root"/examples/javascript/*.js; do node --check "$source"; done
        test -d "$root/examples/typescript/dist" || {
            echo "TypeScript examples must be built before checking compiled JavaScript." >&2
            exit 1
        }
        for source in "$root"/examples/typescript/dist/client_*.js; do node --check "$source"; done
        ;;
    run:javascript)
        test -d "$root/examples/typescript/dist" || {
            echo "TypeScript examples must be built before running compiled JavaScript." >&2
            exit 1
        }
        for source in "$root"/examples/javascript/client_*.js; do
            node "$source"
        done
        for source in "$root"/examples/typescript/dist/client_*.js \
            "$root"/examples/typescript/dist/bypass_ripple_example.js \
            "$root"/examples/typescript/dist/projection_example.js; do
            node "$source"
        done
        ;;
    check:kotlin)
        (cd "$root/examples/kotlin" && ./gradlew compileKotlin --no-daemon)
        ;;
    run:kotlin)
        for source in "$root"/examples/kotlin/examples/Client*.kt \
            "$root"/examples/kotlin/examples/BypassRippleExample.kt; do
            main="io.ekodb.client.examples.$(basename "${source%.kt}")Kt"
            (cd "$root/examples/kotlin" && ./gradlew run -PmainClass="$main" --no-daemon)
        done
        ;;
    *)
        echo "unsupported mode/language: $mode $language" >&2
        exit 2
        ;;
esac
