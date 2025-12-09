#!/usr/bin/env python3
"""
Scripts Example using Direct HTTP Requests

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
        await request(session, "POST", "/api/insert/users", record, token)

    print("✅ Test data ready\n")


async def simple_query_function(session, token):
    """Example 1: Simple Query Function with Filter"""
    print("📝 Example 1: Simple Query Function with Filter\n")

    function1 = {
        "label": "get_active_users",
        "name": "Get Active Users",
        "description": "Query users with active status",
        "version": "1.0",
        "parameters": {},
        "functions": [
            {
                "type": "Query",
                "collection": "users",
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

    # Save script
    save_result = await request(session, "POST", "/api/functions", function1, token)
    print(f"✅ Script saved: {save_result['id']}")

    # Call script (can use label)
    call_result = await request(
        session, "POST", "/api/functions/get_active_users", {}, token
    )
    print(f"📊 Found {len(call_result['records'])} active users\n")

    return save_result["id"]


async def parameterized_pagination_function(session, token):
    """Example 2: Parameterized Pagination with Limit/Skip"""
    print("📝 Example 2: Parameterized Pagination with Limit/Skip\n")

    function2 = {
        "label": "get_active_users_paginated",
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
                "collection": "users",
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

    save_result = await request(session, "POST", "/api/functions", function2, token)
    print(f"✅ Script saved: {save_result['id']}")

    # Call with page 1 (first 3 users)
    call_result = await request(
        session,
        "POST",
        "/api/functions/get_active_users_paginated",
        {"page_size": 3, "page_offset": 0},
        token,
    )
    print(f"📊 Page 1: Found {len(call_result['records'])} users (limit=3, skip=0)\n")

    # Call with page 2 (next 3 users)
    call_result = await request(
        session,
        "POST",
        "/api/functions/get_active_users_paginated",
        {"page_size": 3, "page_offset": 3},
        token,
    )
    print(f"📊 Page 2: Found {len(call_result['records'])} users (limit=3, skip=3)\n")


async def aggregation_function(session, token):
    """Example 3: Multi-Stage Pipeline (Query → Group → Calculate)"""
    print("📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)\n")

    function3 = {
        "label": "user_stats",
        "name": "User Statistics by Status",
        "version": "1.0",
        "parameters": {},
        "functions": [
            {
                "type": "Query",
                "collection": "users",
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

    save_result = await request(session, "POST", "/api/functions", function3, token)
    print(f"✅ Script saved: {save_result['id']}")

    call_result = await request(session, "POST", "/api/functions/user_stats", {}, token)
    print(
        f"📊 Pipeline Results: Filtered (age>20) → Grouped by status → {len(call_result['records'])} groups"
    )
    for record in call_result["records"]:
        print(f"   {record}")
    print()

    return save_result["id"]


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
        "label": "get_active_users",
        "name": "Get Active Users (Updated)",
        "description": "Updated description",
        "version": "1.1",
        "parameters": {},
        "functions": [{"type": "FindAll", "collection": "users"}],
        "tags": ["users"],
    }
    await request(
        session, "PUT", f"/api/functions/{get_active_users_id}", updated, token
    )
    print("✏️  Script updated")

    # Delete script (requires encrypted ID)
    await request(session, "DELETE", f"/api/functions/{user_stats_id}", None, token)
    print("🗑️  Script deleted\n")

    print("ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID")
    print("ℹ️  Only CALL can use either ID or label\n")


async def main():
    """Main function"""
    print("🚀 ekoDB Scripts Example (Python/HTTP)\n")

    try:
        async with aiohttp.ClientSession() as session:
            token = await get_auth_token(session)
            print("✓ Authentication successful\n")

            await setup_test_data(session, token)
            get_active_users_id = await simple_query_function(session, token)
            await parameterized_pagination_function(session, token)
            user_stats_id = await aggregation_function(session, token)
            await function_management(
                session, token, get_active_users_id, user_stats_id
            )

            print("✅ All examples completed!")

    except Exception as error:
        print(f"❌ Error: {error}")
        raise


if __name__ == "__main__":
    asyncio.run(main())
