#!/usr/bin/env python3
"""
Function Composition Example for ekoDB Python Client

Demonstrates calling Functions within Functions using CallFunction
Shows how to build reusable logic blocks and compose complex workflows
"""

import asyncio
import json
import os
import time
from pathlib import Path

from dotenv import load_dotenv

from ekodb_client import Client

# Load environment variables
env_path = Path(__file__).parent.parent / ".env"
load_dotenv(env_path)

BASE_URL = os.getenv("API_BASE_URL", "http://localhost:8080")
API_KEY = os.getenv("API_BASE_KEY", "a-test-api-key-from-ekodb")
COLLECTION = "function_composition_users_py"
FETCH_USER = "fetch_user_py"
GET_USER_WRAPPER = "get_user_wrapper_py"
FETCH_AND_STORE_USER = "fetch_and_store_user_py"
SWR_USER = "swr_user_py"
VALIDATE_USER = "validate_user_py"
FETCH_SLIM_USER = "fetch_slim_user_py"
GET_VERIFIED_USER = "get_verified_user_py"
USER_CACHE_KEY = "user_cache_py:{{user_id}}"
USER_CACHE_ENTRY = "user_cache_py:1"


def _is_already_exists_error(err):
    """Detect the server's 409 'function already exists' response."""
    msg = str(err)
    return "409" in msg or "already exists" in msg


def _is_not_found_error(err):
    msg = str(err).lower()
    return "404" in msg or "not found" in msg


def field_value(record, key):
    value = record.get(key)
    if isinstance(value, dict) and "value" in value:
        return value["value"]
    return value


def swr_payload(result):
    records = result.get("records")
    if not isinstance(records, list) or len(records) != 1:
        raise AssertionError(f"expected one SWR record, got {records!r}")
    value = field_value(records[0], "value")
    if (
        not isinstance(value, dict)
        or value.get("id") != 1
        or value.get("name") != "Leanne Graham"
    ):
        raise AssertionError(f"unexpected SWR payload: {value!r}")
    return value


async def save_or_update(client, script, function_ids):
    """Save a function, falling back to an update if its label already exists.

    The server returns HTTP 409 when a function with the same label is already
    saved. In that case we update the existing definition (PUT by label) and
    return the label as the identifier so the example is idempotent.
    """
    label = script["label"]
    try:
        function_id = await client.save_function(script)
    except Exception as e:
        if not _is_already_exists_error(e):
            raise
        await client.update_function(label, script)
        print(f"ℹ️  Function '{label}' already existed — updated instead")
        existing = await client.get_function(label)
        function_id = existing.get("id")
        if not function_id:
            raise RuntimeError(f"Updated function '{label}' did not include an id")
    function_ids.append(function_id)
    return function_id


async def setup_test_data(client):
    """Create test users"""
    print("📋 Setting up test data...\n")

    inserted_ids = []
    for i in range(1, 6):
        inserted = await client.insert(
            COLLECTION,
            {
                "user_code": f"user_{i}",
                "name": f"User {i}",
                "department": "engineering" if i <= 2 else "sales",
                "score": i * 20,
            },
        )
        inserted_id = field_value(inserted, "id")
        if not inserted_id:
            raise AssertionError(f"user insert {i} did not return an ID")
        inserted_ids.append(inserted_id)
    if len(set(inserted_ids)) != 5:
        raise AssertionError("user inserts did not return 5 unique IDs")

    print("✅ Test data ready\n")


async def basic_composition_example(client, function_ids):
    """Example 1: Basic Function Composition"""
    print("📝 Example 1: Basic Function Composition\n")
    print("Building reusable functions that call each other...\n")

    # Step 1: Create reusable "fetch_user" function
    fetch_user = {
        "label": FETCH_USER,
        "name": "Fetch user by code",
        "parameters": {"user_code": {"required": True}},
        "functions": [
            {
                "type": "FindOne",
                "collection": COLLECTION,
                "key": "user_code",
                "value": "{{user_code}}",
            }
        ],
    }

    await save_or_update(client, fetch_user, function_ids)
    print(f"✅ Saved reusable function: {FETCH_USER}")

    # Step 2: Create wrapper that CALLS fetch_user
    get_user_wrapper = {
        "label": GET_USER_WRAPPER,
        "name": "Wrapper that calls fetch_user",
        "parameters": {"user_code": {"required": True}},
        "functions": [
            {
                "type": "CallFunction",
                "function_label": FETCH_USER,
                # params omitted - inherits user_code from parent scope
            },
            {"type": "Project", "fields": ["name", "department"], "exclude": False},
        ],
    }

    await save_or_update(client, get_user_wrapper, function_ids)
    print(
        f"✅ Saved composed function: {GET_USER_WRAPPER} (calls {FETCH_USER} + projects fields)\n"
    )

    # Step 3: Call the composed function
    result = await client.call_function(GET_USER_WRAPPER, {"user_code": "user_1"})

    print("📊 Result from composed function:")
    print(f"   Records: {len(result['records'])}")
    if len(result["records"]) != 1:
        raise AssertionError("composed function did not return exactly one user")
    record = result["records"][0]
    print(f"   Name: {json.dumps(record.get('name'))}")
    print(f"   Department: {json.dumps(record.get('department'))}\n")
    if (
        field_value(record, "name") != "User 1"
        or field_value(record, "department") != "engineering"
    ):
        raise AssertionError(f"unexpected composed user: {record!r}")

    print("🎯 Key Benefit: fetch_user can be reused by ANY function!")
    print("   No code duplication, single source of truth\n")


async def swr_composition_example(client, function_ids):
    """Example 2: SWR Pattern with Function Composition"""
    print("📝 Example 2: SWR Pattern with Function Composition\n")
    print("Using KV cache + CallFunction for fast cache-aside pattern...\n")

    # Step 1: Create reusable fetch and store function
    # Using jsonplaceholder.typicode.com - a reliable free API for testing
    # This function fetches from API and stores in KV cache
    fetch_and_store = {
        "label": FETCH_AND_STORE_USER,
        "name": "Fetch user from API and cache in KV",
        "parameters": {"user_id": {"required": True}},
        "functions": [
            {
                "type": "HttpRequest",
                "url": "https://jsonplaceholder.typicode.com/users/{{user_id}}",
                "method": "GET",
                "headers": {"Accept": "application/json"},
            },
            # Store in KV cache (much faster than collection for cache lookups)
            {
                "type": "KvSet",
                "key": USER_CACHE_KEY,
                "value": "{{http_response}}",
                "ttl": 300,  # 5 minute cache
            },
        ],
    }

    await save_or_update(client, fetch_and_store, function_ids)
    print(f"✅ Saved reusable function: {FETCH_AND_STORE_USER} (uses KV)")

    # Step 2: Create SWR function that CALLS the reusable function
    # Pattern: KV cache check → populate if missing → return
    swr_user = {
        "label": SWR_USER,
        "name": "SWR pattern for user data (KV-based)",
        "parameters": {"user_id": {"required": True}},
        "functions": [
            # Check KV cache first (O(1) lookup - much faster than FindById)
            {
                "type": "KvGet",
                "key": USER_CACHE_KEY,
            },
            {
                "type": "If",
                # KvGet returns { value: ... } on hit, { value: null } on miss
                # So we check if "value" is not null to detect cache hit
                "condition": {
                    "type": "Not",
                    "value": {
                        "condition": {
                            "type": "FieldEquals",
                            "value": {"field": "value", "value": None},
                        }
                    },
                },
                "then_functions": [
                    # Cache hit - project the value field
                    {"type": "Project", "fields": ["value"], "exclude": False}
                ],
                "else_functions": [
                    # Cache miss - call reusable function to fetch and store
                    # Explicitly pass user_id to the function
                    {
                        "type": "CallFunction",
                        "function_label": FETCH_AND_STORE_USER,
                        "params": {"user_id": "{{user_id}}"},
                    },
                    # After storing, retrieve the cached value to return it
                    {"type": "KvGet", "key": USER_CACHE_KEY},
                    {"type": "Project", "fields": ["value"], "exclude": False},
                ],
            },
        ],
    }

    await save_or_update(client, swr_user, function_ids)
    print(f"✅ Saved SWR function using composition: {SWR_USER}\n")

    # Step 3: Test cache miss
    print("First call (cache miss - will fetch from API):")
    start = time.time()
    result1 = await client.call_function(SWR_USER, {"user_id": "1"})
    duration1 = time.time() - start

    print(f"   ⏱️  Duration: {duration1 * 1000:.1f}ms")
    print(f"   📊 Records: {len(result1['records'])}")
    if result1["records"]:
        print(f"   📦 Data: {json.dumps(result1['records'][0], indent=6)[:200]}...\n")
    else:
        print()
    payload1 = swr_payload(result1)

    # Step 4: Test cache hit
    print("Second call (cache hit - from cache):")
    start = time.time()
    result2 = await client.call_function(SWR_USER, {"user_id": "1"})
    duration2 = time.time() - start

    print(f"   ⏱️  Duration: {duration2 * 1000:.1f}ms")
    print(f"   📊 Records: {len(result2['records'])}")
    if result2["records"]:
        print(f"   📦 Data: {json.dumps(result2['records'][0], indent=6)[:200]}...")
    payload2 = swr_payload(result2)
    if payload2 != payload1:
        raise AssertionError("cache hit returned different user data")
    if duration2 > 0:
        speedup = duration1 / duration2
        print(f"   🚀 Cache speedup: {speedup:.1f}x faster!\n")


async def nested_composition_example(client, function_ids):
    """Example 3: Multi-Level Function Composition"""
    print("📝 Example 3: Multi-Level Function Composition\n")
    print("Building complex workflows from small, reusable pieces...\n")

    # Level 1: Base function
    validate_user = {
        "label": VALIDATE_USER,
        "name": "Check if user exists",
        "parameters": {"user_code": {"required": True}},
        "functions": [
            {
                "type": "FindOne",
                "collection": COLLECTION,
                "key": "user_code",
                "value": "{{user_code}}",
            }
        ],
    }

    await save_or_update(client, validate_user, function_ids)
    print(f"✅ Level 1 function: {VALIDATE_USER}")

    # Level 2: Calls validate_user + projects
    fetch_slim = {
        "label": FETCH_SLIM_USER,
        "name": "Validate and slim down user",
        "parameters": {"user_code": {"required": True}},
        "functions": [
            {
                "type": "CallFunction",
                "function_label": VALIDATE_USER,
                # params omitted - inherits user_code from parent scope
            },
            {"type": "Project", "fields": ["name", "department"], "exclude": False},
        ],
    }

    await save_or_update(client, fetch_slim, function_ids)
    print(f"✅ Level 2 function: {FETCH_SLIM_USER} (calls {VALIDATE_USER})")

    # Level 3: Calls fetch_slim (demonstrates 3-level nesting)
    get_verified_user = {
        "label": GET_VERIFIED_USER,
        "name": "Get verified and validated user",
        "parameters": {"user_code": {"required": True}},
        "functions": [
            {
                "type": "CallFunction",
                "function_label": FETCH_SLIM_USER,
                # params omitted - inherits user_code from parent scope
            },
        ],
    }

    await save_or_update(client, get_verified_user, function_ids)
    print(f"✅ Level 3 function: {GET_VERIFIED_USER} (calls {FETCH_SLIM_USER})\n")

    # Execute 3-level nested composition
    result = await client.call_function(GET_VERIFIED_USER, {"user_code": "user_1"})

    print("📊 Result from 3-level nested composition:")
    print(f"   Records: {len(result['records'])}")
    if len(result["records"]) != 1:
        raise AssertionError("nested composition did not return exactly one user")
    if result["records"]:
        record = result["records"][0]
        name_field = record.get("name", {})
        dept_field = record.get("department", {})

        name = (
            name_field.get("value")
            if isinstance(name_field, dict)
            else name_field or "Unknown"
        )
        department = (
            dept_field.get("value")
            if isinstance(dept_field, dict)
            else dept_field or "Unknown"
        )

        print(f"   Name: {name}")
        print(f"   Department: {department}\n")
        if name != "User 1" or department != "engineering":
            raise AssertionError(f"unexpected nested user: {record!r}")

    print("🎯 Key Benefit: Each function is independently testable and reusable!")
    print("   - validate_user: Used in 100 different workflows")
    print("   - fetch_slim_user: Used in 50 workflows")
    print("   - get_verified_user: Specific workflow\n")


async def main():
    # Initialize client
    client = Client.new(BASE_URL, API_KEY)

    print("=== ekoDB Function Composition Examples ===\n")

    function_ids = []
    try:
        await client.delete_collection(COLLECTION)
    except Exception as error:
        if not _is_not_found_error(error):
            raise
    if await client.kv_exists(USER_CACHE_ENTRY):
        await client.kv_delete(USER_CACHE_ENTRY)
    operation_error = None
    try:
        await setup_test_data(client)
        await basic_composition_example(client, function_ids)
        await swr_composition_example(client, function_ids)
        await nested_composition_example(client, function_ids)
    except BaseException as error:  # noqa: BLE001 - cleanup must run on cancellation
        operation_error = error

    cleanup_errors = []
    for function_id in reversed(function_ids):
        try:
            await client.delete_function(function_id)
        except Exception as error:  # noqa: BLE001 - attempt every cleanup
            cleanup_errors.append(f"function {function_id}: {error}")
    try:
        await client.delete_collection(COLLECTION)
    except Exception as error:  # noqa: BLE001 - preserve the operation error
        if not _is_not_found_error(error):
            cleanup_errors.append(f"collection {COLLECTION}: {error}")
    try:
        if await client.kv_exists(USER_CACHE_ENTRY):
            await client.kv_delete(USER_CACHE_ENTRY)
    except Exception as error:  # noqa: BLE001 - preserve the operation error
        cleanup_errors.append(f"KV key {USER_CACHE_ENTRY}: {error}")

    if operation_error is not None:
        if cleanup_errors:
            print("⚠️  Cleanup errors: " + "; ".join(cleanup_errors))
            operation_error.add_note(
                "cleanup also failed: " + "; ".join(cleanup_errors)
            )
        raise operation_error
    if cleanup_errors:
        raise RuntimeError("Cleanup failed: " + "; ".join(cleanup_errors))

    print("\n✅ Cleanup complete")
    print("✅ All composition examples completed!")


if __name__ == "__main__":
    asyncio.run(main())
