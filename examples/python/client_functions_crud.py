"""
CRUD Scripts Example - Basic Data Operations

Demonstrates basic CRUD operations using scripts:
- FindAll queries
- Group aggregations
- Simple data transformations
"""

import asyncio
import os
from pathlib import Path

from dotenv import load_dotenv

env_path = Path(__file__).parent.parent / ".env"
load_dotenv(env_path)

BASE_URL = os.getenv("API_BASE_URL", "http://localhost:8080")
API_KEY = os.getenv("API_BASE_KEY", "a-test-api-key-from-ekodb")


async def main():
    from ekodb_client import Client, Stage

    client = Client.new(BASE_URL, API_KEY)

    print("🚀 ekoDB Python CRUD Scripts Example\n")

    # Setup test data
    print("📋 Setting up test data...")
    try:
        await client.delete_collection("crud_users_py")
    except Exception:
        pass

    users = []
    for i in range(1, 11):
        user = await client.insert(
            "crud_users_py",
            {
                "name": f"User {i}",
                "email": f"user{i}@example.com",
                "status": "inactive" if i % 3 == 0 else "active",
                "score": i * 10,
                "role": "admin" if i <= 3 else "user",
            },
        )
        users.append(user["id"])
    print(f"✅ Created {len(users)} test users\n")

    script_ids = []

    # Example 1: List All Users
    print("📝 Example 1: List All Users\n")
    script1 = {
        "label": "list_all_users_py",
        "name": "List All Users",
        "version": "1.0",
        "parameters": {},
        "functions": [{"type": "FindAll", "collection": "crud_users_py"}],
        "tags": ["users", "list"],
    }
    script_id1 = await client.save_script(script1)
    script_ids.append(script_id1)
    print("✅ Script saved")

    result1 = await client.call_script("list_all_users_py", None)
    print(f"📊 Found {len(result1['records'])} users")
    print(f"⏱️  Execution time: {result1['stats']['execution_time_ms']}ms\n")

    # Example 2: Count Users by Status
    print("📝 Example 2: Count Users by Status\n")
    script2 = {
        "label": "users_by_status_py",
        "name": "Users by Status",
        "version": "1.0",
        "parameters": {},
        "functions": [
            {"type": "FindAll", "collection": "crud_users_py"},
            {
                "type": "Group",
                "by_fields": ["status"],
                "functions": [{"output_field": "count", "operation": "Count"}],
            },
        ],
        "tags": ["users", "analytics"],
    }
    script_id2 = await client.save_script(script2)
    script_ids.append(script_id2)
    print("✅ Script saved")

    result2 = await client.call_script("users_by_status_py", None)
    print("📊 User counts by status:")
    for group in result2["records"]:
        status = group.get("status", {})
        if isinstance(status, dict):
            status = status.get("value", status)
        count = group.get("count", {})
        if isinstance(count, dict):
            count = count.get("value", count)
        print(f"   {status}: {count} users")
    print(f"⏱️  Execution time: {result2['stats']['execution_time_ms']}ms\n")

    # Example 3: Average Score by Role
    print("📝 Example 3: Average Score by Role\n")
    script3 = {
        "label": "avg_score_by_role_py",
        "name": "Average Score by Role",
        "version": "1.0",
        "parameters": {},
        "functions": [
            {"type": "FindAll", "collection": "crud_users_py"},
            {
                "type": "Group",
                "by_fields": ["role"],
                "functions": [
                    {
                        "output_field": "avg_score",
                        "operation": "Average",
                        "input_field": "score",
                    },
                    {"output_field": "count", "operation": "Count"},
                ],
            },
        ],
        "tags": ["users", "analytics"],
    }
    script_id3 = await client.save_script(script3)
    script_ids.append(script_id3)
    print("✅ Script saved")

    result3 = await client.call_script("avg_score_by_role_py", None)
    print("📊 Average score by role:")
    for group in result3["records"]:
        print(f"   {group}")
    print(f"⏱️  Execution time: {result3['stats']['execution_time_ms']}ms\n")

    # Cleanup
    print("🧹 Cleaning up...")
    for script_id in script_ids:
        try:
            await client.delete_script(script_id)
        except Exception:
            pass
    try:
        await client.delete_collection("crud_users_py")
    except Exception:
        pass
    print("✅ Cleanup complete\n")

    print("✅ All CRUD script examples finished!")


if __name__ == "__main__":
    asyncio.run(main())
