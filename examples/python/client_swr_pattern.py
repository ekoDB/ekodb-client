"""
SWR (Stale-While-Revalidate) Pattern Example

Demonstrates using ekoDB Functions as an SWR cache:
- Check cache first, return if exists
- On cache miss, fetch from external API
- Store result with TTL for auto-expiration
"""

import asyncio
import json
import os
import time
from datetime import datetime, timezone
from pathlib import Path

from dotenv import load_dotenv

env_path = Path(__file__).parent.parent / ".env"
load_dotenv(env_path)

BASE_URL = os.getenv("API_BASE_URL", "http://localhost:8080")
API_KEY = os.getenv("API_BASE_KEY", "a-test-api-key-from-ekodb")
FUNCTION_LABEL = "fetch_api_user_py"
KV_KEY_TEMPLATE = "api:user:py:{{user_id}}"
KV_KEY = "api:user:py:1"


def _is_already_exists_error(err):
    """Detect the server's 409 'function already exists' response."""
    msg = str(err)
    return "409" in msg or "already exists" in msg


def unwrap(value):
    while isinstance(value, dict) and set(value) == {"type", "value"}:
        value = value["value"]
    return value


def user_payload(result):
    records = result.get("records")
    if not isinstance(records, list) or len(records) != 1:
        raise AssertionError(f"expected one SWR record, got {records!r}")
    value = unwrap(records[0].get("value"))
    if (
        not isinstance(value, dict)
        or value.get("id") != 1
        or value.get("name") != "Leanne Graham"
    ):
        raise AssertionError(f"unexpected SWR user payload: {value!r}")
    return value


async def save_or_update(client, script):
    """Save a function, falling back to an update if its label already exists."""
    label = script["label"]
    try:
        return await client.save_function(script)
    except Exception as e:
        if not _is_already_exists_error(e):
            raise
        await client.update_function(label, script)
        print(f"ℹ️  Function '{label}' already existed — updated instead")
        existing = await client.get_function(label)
        function_id = existing.get("id")
        if not function_id:
            raise RuntimeError(f"Updated function '{label}' did not include an id")
        return function_id


async def main():
    from ekodb_client import Client, Stage

    client = Client.new(BASE_URL, API_KEY)

    # This SWR example uses KV operations (kv_get/kv_set) — no collection needed

    print("=== ekoDB SWR (Stale-While-Revalidate) Pattern ===\n")

    print("Step 1: Create SWR function that acts as edge cache")

    # Using jsonplaceholder.typicode.com - a reliable free API for testing
    swr_script = {
        "label": FUNCTION_LABEL,
        "name": "Fetch User with Cache",
        "description": "SWR pattern: Check cache, fetch from API if stale",
        "parameters": {
            "user_id": {"required": True, "description": "User ID to fetch"},
            "ttl": {
                "required": False,
                "default": 300,
                "description": "Cache TTL in seconds",
            },
            "cached_at": {
                "required": True,
                "description": "UTC timestamp when cached",
            },
        },
        "version": "1.0",
        "tags": ["swr", "user", "cache"],
        "functions": [
            # Check KV cache for user data
            Stage.kv_get(KV_KEY_TEMPLATE),
            Stage.if_condition(
                # KvGet returns {value: ...} on hit, {value: null} on miss
                # So we check if "value" is not null to detect cache hit
                {
                    "type": "Not",
                    "value": {
                        "condition": {
                            "type": "FieldEquals",
                            "value": {"field": "value", "value": None},
                        }
                    },
                },
                # Cache hit - return cached data
                [Stage.project(["value"], False)],
                # Cache miss - fetch from API and cache
                [
                    Stage.http_request(
                        "https://jsonplaceholder.typicode.com/users/{{user_id}}",
                        "GET",
                        {"Accept": "application/json"},
                    ),
                    # Store in KV with 5 minute TTL
                    Stage.kv_set(
                        KV_KEY_TEMPLATE,
                        "{{http_response}}",
                        300,
                    ),
                    # Retrieve the cached data to return
                    Stage.kv_get(KV_KEY_TEMPLATE),
                    Stage.project(["value"], False),
                ],
            ),
        ],
    }

    if await client.kv_exists(KV_KEY):
        await client.kv_delete(KV_KEY)

    script_id = None
    operation_error = None
    try:
        script_id = await save_or_update(client, swr_script)
        print(f"✓ Created SWR script: {swr_script['label']} ({script_id})\n")

        print("Step 2: First call - Cache miss, fetches from API")
        result1 = await client.call_function(
            FUNCTION_LABEL,
            {
                "user_id": "1",
                "ttl": 300,
                "cached_at": datetime.now(timezone.utc).isoformat(),
            },
        )
        print(f"Result: {json.dumps(result1, indent=2)}")
        payload1 = user_payload(result1)
        print("✓ Data fetched from external API and cached\n")

        print("Step 3: Second call - Cache hit, instant response from ekoDB")
        start = time.time()
        result2 = await client.call_function(
            FUNCTION_LABEL,
            {"user_id": "1", "cached_at": datetime.now(timezone.utc).isoformat()},
        )
        duration = (time.time() - start) * 1000
        print(f"Response time: {duration:.0f}ms (served from cache)")
        print(f"Result: {json.dumps(result2, indent=2)}")
        payload2 = user_payload(result2)
        if payload2 != payload1:
            raise AssertionError("cache hit returned different user data")
        print("✓ Lightning fast cache hit\n")
    except BaseException as error:  # noqa: BLE001 - cleanup must run on cancellation
        operation_error = error

    # Cleanup
    print("🧹 Cleaning up...")
    cleanup_errors = []
    if script_id is not None:
        try:
            await client.delete_function(script_id)
        except Exception as error:  # noqa: BLE001 - attempt every cleanup
            cleanup_errors.append(f"function {script_id}: {error}")
    try:
        if await client.kv_exists(KV_KEY):
            await client.kv_delete(KV_KEY)
    except Exception as error:  # noqa: BLE001 - preserve the operation error
        cleanup_errors.append(f"KV key {KV_KEY}: {error}")

    if operation_error is not None:
        if cleanup_errors:
            operation_error.add_note(
                "cleanup also failed: " + "; ".join(cleanup_errors)
            )
            print("⚠️  Cleanup errors: " + "; ".join(cleanup_errors))
        raise operation_error
    if cleanup_errors:
        raise RuntimeError("Cleanup failed: " + "; ".join(cleanup_errors))
    print("✓ Cleanup complete\n")

    print("=== SWR Pattern Summary ===")
    print("✅ Cache miss → Fetch from API → Store in ekoDB")
    print("✅ Cache hit → Instant response from ekoDB")
    print("✅ TTL handles automatic cache invalidation")


if __name__ == "__main__":
    asyncio.run(main())
