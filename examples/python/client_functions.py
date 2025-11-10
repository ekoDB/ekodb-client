#!/usr/bin/env python3
"""
Scripts Example for ekoDB Python Client

Demonstrates creating, managing, and executing Scripts
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


async def simple_query_script(client):
    """Example 1: Simple query script"""
    print("📝 Example 1: Simple Query Script\n")

    script = {
        "label": "get_active_users",
        "name": "Get Active Users",
        "description": "Retrieve all active users",
        "version": "1.0",
        "parameters": {},
        "functions": [
            {"type": "FindAll", "collection": "users"},
        ],
        "tags": ["users", "query"],
    }

    script_id = await client.save_script(script)
    print(f"✅ Script saved: {script_id}")

    result = await client.call_script("get_active_users", None)
    print(f"📊 Found {len(result['records'])} active users\n")

    return script_id


async def parameterized_script(client):
    """Example 2: Script with parameters"""
    print("📝 Example 2: Parameterized Script\n")

    script = {
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
        "functions": [
            {"type": "FindAll", "collection": "users"},
        ],
        "tags": ["users", "parameterized"],
    }

    script_id = await client.save_script(script)
    print(f"✅ Script saved: {script_id}")

    params = {"status": "active", "limit": 3}
    result = await client.call_script("get_users_by_status", params)
    print(f"📊 Found {len(result['records'])} users (limited)\n")

    return script_id


async def aggregation_script(client):
    """Example 3: Aggregation script"""
    print("📝 Example 3: Aggregation Script\n")

    script = {
        "label": "user_stats",
        "name": "User Statistics",
        "version": "1.0",
        "parameters": {},
        "functions": [
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

    script_id = await client.save_script(script)
    print(f"✅ Script saved: {script_id}")

    result = await client.call_script("user_stats", None)
    print(f"📊 Statistics: {len(result['records'])} groups")
    for record in result["records"]:
        print(f"   {record}\n")

    return script_id


async def script_management(client, get_active_users_id, user_stats_id):
    """Example 4: Script management operations"""
    print("📝 Example 4: Script Management\n")

    # List all scripts
    scripts = await client.list_scripts(None)
    print(f"📋 Total scripts: {len(scripts)}")

    # Get specific script by ID
    script = await client.get_script(get_active_users_id)
    print(f"🔍 Retrieved script: {script['name']}")

    # Update script by ID
    updated = {
        "label": "get_active_users_updated",
        "name": "Get Active Users (Updated)",
        "description": "Updated description",
        "version": "1.1",
        "parameters": {},
        "functions": [
            {"type": "FindAll", "collection": "users"},
        ],
        "tags": ["users"],
    }
    await client.update_script(get_active_users_id, updated)
    print("✏️  Script updated")

    # Delete script by ID
    await client.delete_script(user_stats_id)
    print("🗑️  Script deleted\n")

    print("ℹ️  Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.\n")


async def main():
    """Main execution"""
    print("🚀 ekoDB Scripts Example (Python)\n")

    client = Client.new(BASE_URL, API_KEY)

    # Run examples and track IDs
    await setup_test_data(client)
    get_active_users_id = await simple_query_script(client)
    get_users_by_status_id = await parameterized_script(client)
    user_stats_id = await aggregation_script(client)
    await script_management(client, get_active_users_id, user_stats_id)

    print("✅ All examples completed!")


if __name__ == "__main__":
    asyncio.run(main())
