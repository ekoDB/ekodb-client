"""
CRUD Functions Example - Basic Data Operations

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


async def run_examples(client, script_ids):
    print("🚀 ekoDB Python CRUD Functions Example\n")

    # Setup test data
    print("📋 Setting up test data...")
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
        user_id = field_value(user, "id")
        if not user_id:
            raise AssertionError(f"insert {i} did not return an ID")
        users.append(user_id)
    if len(set(users)) != 10:
        raise AssertionError("user inserts did not return 10 unique IDs")
    print(f"✅ Created {len(users)} test users\n")

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
    script_id1 = await save_or_update(client, script1)
    script_ids.append(script_id1)
    print("✅ Function saved")

    result1 = await client.call_function("list_all_users_py", None)
    print(f"📊 Found {len(result1['records'])} users")
    if len(result1["records"]) != 10:
        raise AssertionError("list function did not return all 10 users")
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
    script_id2 = await save_or_update(client, script2)
    script_ids.append(script_id2)
    print("✅ Function saved")

    result2 = await client.call_function("users_by_status_py", None)
    print("📊 User counts by status:")
    status_counts = {}
    for group in result2["records"]:
        status = field_value(group, "status")
        count = field_value(group, "count")
        print(f"   {status}: {count} users")
        status_counts[status] = count
    if status_counts != {"active": 7, "inactive": 3}:
        raise AssertionError(f"unexpected status groups: {status_counts!r}")
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
    script_id3 = await save_or_update(client, script3)
    script_ids.append(script_id3)
    print("✅ Function saved")

    result3 = await client.call_function("avg_score_by_role_py", None)
    print("📊 Average score by role:")
    role_averages = {}
    for group in result3["records"]:
        print(f"   {group}")
        role_averages[field_value(group, "role")] = field_value(group, "avg_score")
    if role_averages != {"admin": 20.0, "user": 70.0}:
        raise AssertionError(f"unexpected role averages: {role_averages!r}")
    print(f"⏱️  Execution time: {result3['stats']['execution_time_ms']}ms\n")


async def main():
    from ekodb_client import Client

    client = Client.new(BASE_URL, API_KEY)
    collection = "crud_users_py"
    script_ids = []
    try:
        await client.delete_collection(collection)
    except Exception as error:
        if not _is_not_found_error(error):
            raise

    operation_error = None
    try:
        await run_examples(client, script_ids)
    except BaseException as error:  # noqa: BLE001 - cleanup must run on cancellation
        operation_error = error

    print("🧹 Cleaning up...")
    cleanup_errors = []
    for script_id in reversed(script_ids):
        try:
            await client.delete_function(script_id)
        except Exception as error:  # noqa: BLE001 - attempt every cleanup
            cleanup_errors.append(f"function {script_id}: {error}")
    try:
        await client.delete_collection(collection)
    except Exception as error:  # noqa: BLE001 - preserve the operation error
        if not _is_not_found_error(error):
            cleanup_errors.append(f"collection {collection}: {error}")

    if operation_error is not None:
        if cleanup_errors:
            operation_error.add_note(
                "cleanup also failed: " + "; ".join(cleanup_errors)
            )
            print("⚠️  Cleanup errors: " + "; ".join(cleanup_errors))
        raise operation_error
    if cleanup_errors:
        raise RuntimeError("Cleanup failed: " + "; ".join(cleanup_errors))

    print("✅ Cleanup complete\n")
    print("✅ All CRUD script examples finished!")


if __name__ == "__main__":
    asyncio.run(main())
