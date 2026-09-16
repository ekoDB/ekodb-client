#!/usr/bin/env bash

# Source the workspace's canonical example configuration without replacing
# values explicitly supplied by the caller (for example, CI or staging URLs).
ekodb_env_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)

ekodb_api_key_was_set=${API_BASE_KEY+x}
ekodb_api_key=${API_BASE_KEY-}
ekodb_api_url_was_set=${API_BASE_URL+x}
ekodb_api_url=${API_BASE_URL-}
ekodb_ws_url_was_set=${WS_BASE_URL+x}
ekodb_ws_url=${WS_BASE_URL-}

if [[ -f "$ekodb_env_root/.env" ]]; then
    ekodb_allexport_was_set=
    case $- in
        *a*) ekodb_allexport_was_set=1 ;;
    esac
    set -a
    # shellcheck disable=SC1091
    if ! source "$ekodb_env_root/.env"; then
        [[ -n "$ekodb_allexport_was_set" ]] || set +a
        echo "Failed to load $ekodb_env_root/.env" >&2
        return 1 2>/dev/null || exit 1
    fi
    [[ -n "$ekodb_allexport_was_set" ]] || set +a
fi

if [[ -n "$ekodb_api_key_was_set" ]]; then
    export API_BASE_KEY=$ekodb_api_key
fi
if [[ -n "$ekodb_api_url_was_set" ]]; then
    export API_BASE_URL=$ekodb_api_url
fi
if [[ -n "$ekodb_ws_url_was_set" ]]; then
    export WS_BASE_URL=$ekodb_ws_url
fi

: "${API_BASE_URL:=http://localhost:8080}"
: "${WS_BASE_URL:=ws://localhost:8080}"
export API_BASE_URL WS_BASE_URL

if [[ -z ${API_BASE_KEY:-} ]]; then
    echo "API_BASE_KEY must be set by the caller or in $ekodb_env_root/.env" >&2
    return 1 2>/dev/null || exit 1
fi

unset ekodb_env_root ekodb_api_key_was_set ekodb_api_key
unset ekodb_api_url_was_set ekodb_api_url ekodb_ws_url_was_set ekodb_ws_url
unset ekodb_allexport_was_set
