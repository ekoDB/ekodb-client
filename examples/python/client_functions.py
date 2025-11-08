#!/usr/bin/env python3
"""
Saved Functions Example for ekoDB Python Client

Demonstrates creating, managing, and executing saved functions
"""

import asyncio
import os
from pathlib import Path
from dotenv import load_dotenv

# Import the ekodb_client (Python bindings for Rust client)
from ekodb_client import Client

# Load environment variables
env_path = Path(__file__).parent.parent / ".env"
load_dotenv(env_path)

BASE_URL = os.getenv("API_BASE_URL", "http://localhost:8080")
API_KEY = os.getenv("API_BASE_KEY", "a-test-api-key-from-ekodb")


async def setup_test_data(client):
    """Insert test data"""
    print("📋 Setting up test data...")

    for i in range(1, 11):
        record = {
            "name": f"User {i}",
            "age": 20 + i,
            "status": "active" if i % 2 == 0 else "inactive",
            "score": i * 10,
        }
        await client.insert("users", record)

    print("✅ Test data ready\n")


async def simple_query_function(client):
    """Example 1: Simple query function"""
    print("📝 Example 1: Simple Query Function\n")

    function = {
        "label": "get_active_users",
        "name": "Get Active Users",
        "description": "Retrieve all active users",
        "version": "1.0",
        "parameters": {},
        "pipeline": [
            {"type": "FindAll", "collection": "users"},
        ],
        "tags": ["users", "query"],
    }

    func_id = await client.save_function(function)
    print(f"✅ Function saved: {func_id}")

    result = await client.call_function("get_active_users", None)
    print(f"📊 Found {len(result['records'])} active users\n")

    return func_id


async def parameterized_function(client):
    """Example 2: Function with parameters"""
    print("📝 Example 2: Parameterized Function\n")

    function = {
        "label": "get_users_by_status",
        "name": "Get Users By Status",
        "version": "1.0",
        "parameters": {
            "status": {
                "param_type": "String",
                "required": False,
                "default": "active",
            },
            "limit": {
                "param_type": "Integer",
                "required": False,
                "default": 10,
            },
        },
        "pipeline": [
            {"type": "FindAll", "collection": "users"},
        ],
        "tags": ["users", "parameterized"],
    }

    func_id = await client.save_function(function)
    print(f"✅ Function saved: {func_id}")

    params = {"status": "active", "limit": 3}
    result = await client.call_function("get_users_by_status", params)
    print(f"📊 Found {len(result['records'])} users (limited)\n")


async def aggregation_function(client):
    """Example 3: Aggregation function"""
    print("📝 Example 3: Aggregation Function\n")

    function = {
        "label": "user_stats",
        "name": "User Statistics",
        "version": "1.0",
        "parameters": {},
        "pipeline": [
            {"type": "FindAll", "collection": "users"},
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
                ],
            },
        ],
        "tags": ["analytics"],
    }

    func_id = await client.save_function(function)
    print(f"✅ Function saved: {func_id}")

    result = await client.call_function("user_stats", None)
    print(f"📊 Statistics: {len(result['records'])} groups")
    for record in result["records"]:
        print(f"   {record}\n")

    return func_id


async def function_management(client, get_active_users_id, user_stats_id):
    """Example 4: Function management operations"""
    print("📝 Example 4: Function Management\n")

    # List all functions
    functions = await client.list_functions(None)
    print(f"📋 Total functions: {len(functions)}")

    # Get specific function (requires ID, not label)
    func = await client.get_function(get_active_users_id)
    print(f"🔍 Retrieved function: {func['name']}")

    # Update function (requires ID, not label)
    updated = {
        "label": "get_active_users",
        "name": "Get Active Users (Updated)",
        "description": "Updated description",
        "version": "1.1",
        "parameters": {},
        "pipeline": [
            {"type": "FindAll", "collection": "users"},
        ],
        "tags": ["users"],
    }
    await client.update_function(get_active_users_id, updated)
    print("✏️  Function updated")

    # Delete function (requires ID, not label)
    await client.delete_function(user_stats_id)
    print("🗑️  Function deleted\n")

    print("ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID")
    print("ℹ️  Only CALL can use either ID or label\n")


async def main():
    """Main execution"""
    print("🚀 ekoDB Saved Functions Example (Python)\n")

    client = Client.new(BASE_URL, API_KEY)

    # Run examples
    await setup_test_data(client)
    get_active_users_id = await simple_query_function(client)
    await parameterized_function(client)
    user_stats_id = await aggregation_function(client)
    await function_management(client, get_active_users_id, user_stats_id)

    print("✅ All examples completed!")


if __name__ == "__main__":
    asyncio.run(main())
