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


async def setup_test_data(client):
    """Create test users"""
    print("📋 Setting up test data...\n")

    for i in range(1, 6):
        await client.insert(
            "users",
            {
                "user_code": f"user_{i}",
                "name": f"User {i}",
                "department": "engineering" if i <= 2 else "sales",
                "score": i * 20,
            },
        )

    print("✅ Test data ready\n")


async def basic_composition_example(client):
    """Example 1: Basic Function Composition"""
    print("📝 Example 1: Basic Function Composition\n")
    print("Building reusable functions that call each other...\n")

    # Step 1: Create reusable "fetch_user" function
    fetch_user = {
        "label": "fetch_user",
        "name": "Fetch user by code",
        "parameters": {"user_code": {"required": True}},
        "functions": [
            {
                "type": "FindOne",
                "collection": "users",
                "key": "user_code",
                "value": "{{user_code}}",
            }
        ],
    }

    await client.save_script(fetch_user)
    print("✅ Saved reusable function: fetch_user")

    # Step 2: Create wrapper that CALLS fetch_user
    get_user_wrapper = {
        "label": "get_user_wrapper",
        "name": "Wrapper that calls fetch_user",
        "parameters": {"user_code": {"required": True}},
        "functions": [
            {
                "type": "CallFunction",
                "function_label": "fetch_user",
                "params": {"user_code": "{{user_code}}"},
            },
            {"type": "Project", "fields": ["name", "department"], "exclude": False},
        ],
    }

    await client.save_script(get_user_wrapper)
    print(
        "✅ Saved composed function: get_user_wrapper (calls fetch_user + projects fields)\n"
    )

    # Step 3: Call the composed function
    result = await client.call_script("get_user_wrapper", {"user_code": "user_1"})

    print("📊 Result from composed function:")
    print(f"   Records: {len(result['records'])}")
    if result["records"]:
        record = result["records"][0]
        print(f"   Name: {json.dumps(record.get('name'))}")
        print(f"   Department: {json.dumps(record.get('department'))}\n")

    print("🎯 Key Benefit: fetch_user can be reused by ANY function!")
    print("   No code duplication, single source of truth\n")


async def swr_composition_example(client):
    """Example 2: SWR Pattern with Function Composition"""
    print("📝 Example 2: SWR Pattern with Function Composition\n")
    print("Using CallFunction to replace inline logic in SWR pattern...\n")

    # Step 1: Create reusable fetch and store function
    fetch_and_store = {
        "label": "fetch_and_store_github",
        "name": "Fetch from GitHub and store",
        "parameters": {"username": {"required": True}},
        "functions": [
            {
                "type": "HttpRequest",
                "url": "https://api.github.com/users/{{username}}",
                "method": "GET",
                "headers": {"User-Agent": "ekoDB-Client"},
            },
            {
                "type": "Insert",
                "collection": "github_cache",
                "record": {
                    "cache_key": {"type": "String", "value": "{{username}}"},
                    "data": {"type": "Object", "value": "{{http_response}}"},
                },
                "ttl": 300,  # 5 minute cache
            },
        ],
    }

    await client.save_script(fetch_and_store)
    print("✅ Saved reusable function: fetch_and_store_github")

    # Step 2: Create SWR function that CALLS the reusable function
    swr_github = {
        "label": "swr_github_user",
        "name": "SWR pattern using reusable functions",
        "parameters": {"username": {"required": True}},
        "functions": [
            {
                "type": "FindById",
                "collection": "github_cache",
                "record_id": "{{username}}",
            },
            {
                "type": "If",
                "condition": {"type": "HasRecords"},
                "then_functions": [
                    {"type": "Project", "fields": ["data"], "exclude": False}
                ],
                "else_functions": [
                    {
                        "type": "CallFunction",
                        "function_label": "fetch_and_store_github",
                        "params": {"username": "{{username}}"},
                    }
                ],
            },
        ],
    }

    await client.save_script(swr_github)
    print("✅ Saved SWR function using composition: swr_github_user\n")

    # Step 3: Test cache miss
    print("First call (cache miss - will fetch from GitHub):")
    start = time.time()
    result1 = await client.call_script("swr_github_user", {"username": "torvalds"})
    duration1 = time.time() - start

    print(f"   ⏱️  Duration: {duration1*1000:.1f}ms")
    print(f"   📊 Records: {len(result1['records'])}\n")

    # Step 4: Test cache hit
    print("Second call (cache hit - from cache):")
    start = time.time()
    result2 = await client.call_script("swr_github_user", {"username": "torvalds"})
    duration2 = time.time() - start

    print(f"   ⏱️  Duration: {duration2*1000:.1f}ms")
    print(f"   📊 Records: {len(result2['records'])}")
    if duration2 > 0:
        speedup = duration1 / duration2
        print(f"   🚀 Cache speedup: {speedup:.1f}x faster!\n")


async def nested_composition_example(client):
    """Example 3: Multi-Level Function Composition"""
    print("📝 Example 3: Multi-Level Function Composition\n")
    print("Building complex workflows from small, reusable pieces...\n")

    # Level 1: Base function
    validate_user = {
        "label": "validate_user",
        "name": "Check if user exists",
        "parameters": {"user_code": {"required": True}},
        "functions": [
            {
                "type": "FindOne",
                "collection": "users",
                "key": "user_code",
                "value": "{{user_code}}",
            }
        ],
    }

    await client.save_script(validate_user)
    print("✅ Level 1 function: validate_user")

    # Level 2: Calls validate_user + projects
    fetch_slim = {
        "label": "fetch_slim_user",
        "name": "Validate and slim down user",
        "parameters": {"user_code": {"required": True}},
        "functions": [
            {
                "type": "CallFunction",
                "function_label": "validate_user",
                "params": {"user_code": "{{user_code}}"},
            },
            {"type": "Project", "fields": ["name", "department"], "exclude": False},
        ],
    }

    await client.save_script(fetch_slim)
    print("✅ Level 2 function: fetch_slim_user (calls validate_user)")

    # Level 3: Calls fetch_slim + counts
    count_user = {
        "label": "count_validated_user",
        "name": "Get validated user and count",
        "parameters": {"user_code": {"required": True}},
        "functions": [
            {
                "type": "CallFunction",
                "function_label": "fetch_slim_user",
                "params": {"user_code": "{{user_code}}"},
            },
            {"type": "Count", "output_field": "record_count"},
        ],
    }

    await client.save_script(count_user)
    print("✅ Level 3 function: count_validated_user (calls fetch_slim_user)\n")

    # Execute 3-level nested composition
    result = await client.call_script("count_validated_user", {"user_code": "user_1"})

    print("📊 Result from 3-level nested composition:")
    print(f"   Records: {len(result['records'])}")
    if result["records"]:
        record = result["records"][0]
        name_field = record.get("name", {})
        dept_field = record.get("department", {})
        count_field = record.get("record_count", {})

        name = (
            name_field.get("value")
            if isinstance(name_field, dict)
            else name_field or "N/A"
        )
        department = (
            dept_field.get("value")
            if isinstance(dept_field, dict)
            else dept_field or "N/A"
        )
        record_count = (
            count_field.get("value")
            if isinstance(count_field, dict)
            else count_field or 0
        )

        print(f"   Name: {name}")
        print(f"   Department: {department}")
        print(f"   Record count: {record_count}\n")

    print("🎯 Key Benefit: Each function is independently testable and reusable!")
    print("   - validate_user: Used in 100 different workflows")
    print("   - fetch_slim_user: Used in 50 workflows")
    print("   - count_validated_user: Specific workflow\n")


async def main():
    # Initialize client
    client = Client.new(BASE_URL, API_KEY)

    print("=== ekoDB Function Composition Examples ===\n")

    # Setup test data
    await setup_test_data(client)

    # Run examples
    await basic_composition_example(client)
    await swr_composition_example(client)
    await nested_composition_example(client)

    print("\n✅ All composition examples completed!")


if __name__ == "__main__":
    asyncio.run(main())
