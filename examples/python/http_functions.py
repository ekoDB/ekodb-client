#!/usr/bin/env python3
"""
Functions Example using Direct HTTP Requests

Demonstrates using scripts with raw HTTP/requests API
No client library required
"""

import asyncio
import os
from pathlib import Path
import aiohttp
from dotenv import load_dotenv

# Load environment variables
env_path = Path(__file__).parent.parent / ".env"
load_dotenv(env_path)

BASE_URL = os.getenv("API_BASE_URL", "http://localhost:8080")
API_KEY = os.getenv("API_BASE_KEY", "a-test-api-key-from-ekodb")
TEST_COLLECTION = "functions_users_http_py"
FUNCTION_LABELS = (
    "get_active_users_http_py",
    "get_active_users_paginated_http_py",
    "user_stats_http_py",
)


async def get_auth_token(session):
    """Get authentication token"""
    async with session.post(
        f"{BASE_URL}/api/auth/token",
        json={"api_key": API_KEY},
        headers={"Content-Type": "application/json"},
    ) as response:
        data = await response.json()
        return data["token"]


async def request(session, method, path, body=None, token=None):
    """Make authenticated HTTP request"""
    headers = {
        "Authorization": f"Bearer {token}",
        "Content-Type": "application/json",
    }

    async with session.request(
        method, f"{BASE_URL}{path}", json=body, headers=headers
    ) as response:
        if not response.ok:
            text = await response.text()
            raise Exception(f"HTTP {response.status}: {text}")
        return await response.json()


async def save_or_update_function(session, token, function):
    """Save a function via POST, updating it via PUT if the label already exists.

    POST /api/functions returns HTTP 409 when a function with the same label is
    already saved. In that case we PUT the definition to
    /api/functions/{label} (the route accepts an ID or a label) and return the
    label as the identifier — GET/PUT/DELETE all accept it the same way as a
    fresh encrypted ID.
    """
    label = function["label"]
    async with session.post(
        f"{BASE_URL}/api/functions",
        json=function,
        headers={
            "Authorization": f"Bearer {token}",
            "Content-Type": "application/json",
        },
    ) as response:
        if response.ok:
            data = await response.json()
            return data["id"]
        if response.status != 409:
            text = await response.text()
            raise Exception(f"HTTP {response.status}: {text}")

    # 409: label already exists — update the existing definition instead.
    await request(session, "PUT", f"/api/functions/{label}", function, token)
    print(f"ℹ️  Function '{label}' already existed — updated instead")
    existing = await request(session, "GET", f"/api/functions/{label}", None, token)
    return existing["id"]


async def setup_test_data(session, token):
    """Setup test data"""
    print("📋 Setting up test data...")

    for i in range(1, 11):
        record = {
            "name": f"User {i}",
            "age": 20 + i,
            "status": "active" if i % 2 == 0 else "inactive",
            "score": i * 10,
        }
        await request(session, "POST", f"/api/insert/{TEST_COLLECTION}", record, token)

    print("✅ Test data ready\n")


async def simple_query_function(session, token):
    """Example 1: Simple Query Function with Filter"""
    print("📝 Example 1: Simple Query Function with Filter\n")

    function1 = {
        "label": FUNCTION_LABELS[0],
        "name": "Get Active Users",
        "description": "Query users with active status",
        "version": "1.0",
        "parameters": {},
        "functions": [
            {
                "type": "Query",
                "collection": TEST_COLLECTION,
                "filter": {
                    "type": "Condition",
                    "content": {
                        "field": "status",
                        "operator": "Eq",
                        "value": "active",
                    },
                },
            }
        ],
        "tags": ["users", "query"],
    }

    # Save script (idempotent: updates if the label already exists)
    function_id = await save_or_update_function(session, token, function1)
    print(f"✅ Function saved: {function_id}")

    # Call script (can use label)
    call_result = await request(
        session, "POST", f"/api/functions/{FUNCTION_LABELS[0]}", {}, token
    )
    print(f"📊 Found {len(call_result['records'])} active users\n")

    return function_id


async def parameterized_pagination_function(session, token):
    """Example 2: Parameterized Pagination with Limit/Skip"""
    print("📝 Example 2: Parameterized Pagination with Limit/Skip\n")

    function2 = {
        "label": FUNCTION_LABELS[1],
        "name": "Get Active Users (Paginated)",
        "version": "1.0",
        "parameters": {
            "page_size": {
                "type": "Integer",
                "required": False,
                "default": 5,
            },
            "page_offset": {
                "type": "Integer",
                "required": False,
                "default": 0,
            },
        },
        "functions": [
            {
                "type": "Query",
                "collection": TEST_COLLECTION,
                "filter": {
                    "type": "Condition",
                    "content": {
                        "field": "status",
                        "operator": "Eq",
                        "value": "active",
                    },
                },
                "sort": [{"field": "score", "ascending": False}],
                "limit": "{{page_size}}",
                "skip": "{{page_offset}}",
            }
        ],
        "tags": ["users", "pagination"],
    }

    function_id = await save_or_update_function(session, token, function2)
    print(f"✅ Function saved: {function_id}")

    # Call with page 1 (first 3 users)
    call_result = await request(
        session,
        "POST",
        f"/api/functions/{FUNCTION_LABELS[1]}",
        {"page_size": 3, "page_offset": 0},
        token,
    )
    print(f"📊 Page 1: Found {len(call_result['records'])} users (limit=3, skip=0)\n")

    # Call with page 2 (next 3 users)
    call_result = await request(
        session,
        "POST",
        f"/api/functions/{FUNCTION_LABELS[1]}",
        {"page_size": 3, "page_offset": 3},
        token,
    )
    print(f"📊 Page 2: Found {len(call_result['records'])} users (limit=3, skip=3)\n")
    return function_id


async def aggregation_function(session, token):
    """Example 3: Multi-Stage Pipeline (Query → Group → Calculate)"""
    print("📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)\n")

    function3 = {
        "label": FUNCTION_LABELS[2],
        "name": "User Statistics by Status",
        "version": "1.0",
        "parameters": {},
        "functions": [
            {
                "type": "Query",
                "collection": TEST_COLLECTION,
                "filter": {
                    "type": "Condition",
                    "content": {
                        "field": "age",
                        "operator": "Gt",
                        "value": 20,
                    },
                },
            },
            {
                "type": "Group",
                "by_fields": ["status"],
                "functions": [
                    {"output_field": "count", "operation": "Count"},
                    {
                        "output_field": "avg_score",
                        "operation": "Average",
                        "input_field": "score",
                    },
                    {
                        "output_field": "max_score",
                        "operation": "Max",
                        "input_field": "score",
                    },
                ],
            },
        ],
        "tags": ["analytics", "pipeline"],
    }

    function_id = await save_or_update_function(session, token, function3)
    print(f"✅ Function saved: {function_id}")

    call_result = await request(
        session, "POST", f"/api/functions/{FUNCTION_LABELS[2]}", {}, token
    )
    print(
        f"📊 Pipeline Results: Filtered (age>20) → Grouped by status → {len(call_result['records'])} groups"
    )
    for record in call_result["records"]:
        print(f"   {record}")
    print()

    return function_id


async def function_management(session, token, get_active_users_id, user_stats_id):
    """Example 4: Function Management"""
    print("📝 Example 4: Function Management\n")

    # List all scripts
    scripts = await request(session, "GET", "/api/functions", None, token)
    print(f"📋 Total scripts: {len(scripts)}")

    # Get specific script (requires encrypted ID)
    script = await request(
        session, "GET", f"/api/functions/{get_active_users_id}", None, token
    )
    print(f"🔍 Retrieved script: {script['name']}")

    # Update script (requires encrypted ID)
    updated = {
        "label": FUNCTION_LABELS[0],
        "name": "Get Active Users (Updated)",
        "description": "Updated description",
        "version": "1.1",
        "parameters": {},
        "functions": [{"type": "FindAll", "collection": TEST_COLLECTION}],
        "tags": ["users"],
    }
    await request(
        session, "PUT", f"/api/functions/{get_active_users_id}", updated, token
    )
    print("✏️  Function updated")

    # Delete script (requires encrypted ID)
    await request(session, "DELETE", f"/api/functions/{user_stats_id}", None, token)
    print("🗑️  Function deleted\n")

    print("ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID")
    print("ℹ️  Only CALL can use either ID or label\n")


async def main():
    """Main function"""
    print("🚀 ekoDB Functions Example (Python/HTTP)\n")

    async with aiohttp.ClientSession() as session:
        token = await get_auth_token(session)
        print("✓ Authentication successful\n")
        function_ids = []
        try:
            try:
                await request(
                    session,
                    "DELETE",
                    f"/api/collections/{TEST_COLLECTION}",
                    None,
                    token,
                )
            except Exception as error:
                if "HTTP 404" not in str(error):
                    raise
            await setup_test_data(session, token)
            get_active_users_id = await simple_query_function(session, token)
            function_ids.append(get_active_users_id)
            function_ids.append(await parameterized_pagination_function(session, token))
            user_stats_id = await aggregation_function(session, token)
            function_ids.append(user_stats_id)
            await function_management(
                session, token, get_active_users_id, user_stats_id
            )
            print("✅ All examples completed!")
        finally:
            cleanup_errors = []
            cleanup_function_ids = list(function_ids)
            try:
                functions = await request(session, "GET", "/api/functions", None, token)
                cleanup_function_ids.extend(
                    function["id"]
                    for function in functions
                    if function.get("label") in FUNCTION_LABELS
                    and function.get("id") not in cleanup_function_ids
                )
            except Exception as error:
                cleanup_errors.append(f"discover functions: {error}")
            for function_id in cleanup_function_ids:
                try:
                    await request(
                        session,
                        "DELETE",
                        f"/api/functions/{function_id}",
                        None,
                        token,
                    )
                except Exception as error:
                    if "HTTP 404" not in str(error) and "not found" not in str(error):
                        cleanup_errors.append(f"function {function_id}: {error}")
            try:
                await request(
                    session,
                    "DELETE",
                    f"/api/collections/{TEST_COLLECTION}",
                    None,
                    token,
                )
            except Exception as error:
                if "HTTP 404" not in str(error) and "not found" not in str(error):
                    cleanup_errors.append(f"collection {TEST_COLLECTION}: {error}")
            if cleanup_errors:
                raise RuntimeError("Cleanup failed: " + "; ".join(cleanup_errors))


if __name__ == "__main__":
    asyncio.run(main())
