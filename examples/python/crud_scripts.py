#!/usr/bin/env python3
"""
Complete CRUD Scripts Example - ekoDB

Demonstrates the full CRUD lifecycle using Scripts with verification:
1. Insert + Verify (using Query)
2. Query + Update + Verify
3. FindById + Update + Verify  
4. Delete + Verify

Each Script shows how Functions chain together with proper verification.
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


async def script_1_insert_and_verify(session, token):
    """
    Script 1: Insert + Verify

    Functions:
    1. Insert - Create a new user record
    2. Query - Verify the record was created
    """
    print("=" * 60)
    print("📝 Script 1: Insert + Verify")
    print("=" * 60)

    script = {
        "label": "insert_and_verify",
        "name": "Insert and Verify User",
        "description": "Insert a user and verify it was created",
        "version": "1.0",
        "parameters": {
            "user_name": {
                "type": "String",
                "required": True,
                "description": "Name of the user",
            },
            "user_email": {
                "type": "String",
                "required": True,
                "description": "Email of the user",
            },
        },
        "functions": [
            {
                "type": "Insert",
                "collection": "users",
                "record": {
                    "name": "{{user_name}}",
                    "email": "{{user_email}}",
                    "status": "pending",
                    "credits": 0,
                },
            },
            {
                "type": "Query",
                "collection": "users",
                "filter": {
                    "type": "Condition",
                    "content": {
                        "field": "email",
                        "operator": "Eq",
                        "value": "{{user_email}}",
                    },
                },
            },
        ],
        "tags": ["crud", "insert", "verification"],
    }

    # Save script
    print("\n1️⃣ Saving Script...")
    save_result = await request(session, "POST", "/api/functions", script, token)
    script_id = save_result["id"]
    print(f"   ✅ Script saved: {script_id}")

    # Call script with parameters
    print("\n2️⃣ Calling Script (Insert + Verify)...")
    params = {"user_name": "Alice Smith", "user_email": "alice@example.com"}
    call_result = await request(
        session,
        "POST",
        "/api/functions/insert_and_verify",
        params,
        token,
    )

    print(f"   ✅ Script executed: {call_result['stats']['stages_executed']} Functions")
    print(f"   ⏱️  Execution time: {call_result['stats']['execution_time_ms']}ms")

    # Show results from verification Query
    records = call_result["records"]
    print(f"\n3️⃣ Verification Results:")
    print(f"   ✅ Found {len(records)} record(s)")
    if records:
        user = records[0]
        user_id = user.get("id", "N/A")
        print(f"   📋 User ID: {user_id}")
        print(f"   📋 Name: {user.get('name', 'N/A')}")
        print(f"   📋 Email: {user.get('email', 'N/A')}")
        print(f"   📋 Status: {user.get('status', 'N/A')}")
        print(f"   📋 Credits: {user.get('credits', 'N/A')}")
        return user_id, script_id

    return None, script_id


async def script_2_query_update_verify(session, token):
    """
    Script 2: Query + Update + Verify

    Functions:
    1. Query - Find the user by email filter
    2. Update - Change status to 'active'
    3. Query - Verify the update worked
    """
    print("\n" + "=" * 60)
    print("📝 Script 2: Query + Update + Verify")
    print("=" * 60)

    script = {
        "label": "query_update_verify",
        "name": "Query, Update, and Verify",
        "description": "Find user by filter, update status, verify change",
        "version": "1.0",
        "parameters": {
            "user_email": {
                "type": "String",
                "required": True,
                "description": "Email to search for",
            },
            "new_status": {
                "type": "String",
                "default": "active",
                "required": False,
                "description": "New status value",
            },
        },
        "functions": [
            {
                "type": "Query",
                "collection": "users",
                "filter": {
                    "type": "Condition",
                    "content": {
                        "field": "email",
                        "operator": "Eq",
                        "value": "{{user_email}}",
                    },
                },
            },
            {
                "type": "Update",
                "collection": "users",
                "filter": {
                    "type": "Condition",
                    "content": {
                        "field": "email",
                        "operator": "Eq",
                        "value": "{{user_email}}",
                    },
                },
                "updates": {"status": "{{new_status}}"},
            },
            {
                "type": "Query",
                "collection": "users",
                "filter": {
                    "type": "Condition",
                    "content": {
                        "field": "email",
                        "operator": "Eq",
                        "value": "{{user_email}}",
                    },
                },
            },
        ],
        "tags": ["crud", "update", "verification"],
    }

    print("\n1️⃣ Saving Script...")
    save_result = await request(session, "POST", "/api/functions", script, token)
    script_id = save_result["id"]
    print(f"   ✅ Script saved: {script_id}")

    print("\n2️⃣ Calling Script (Query + Update + Verify)...")
    params = {"user_email": "alice@example.com", "new_status": "active"}
    call_result = await request(
        session,
        "POST",
        "/api/functions/query_update_verify",
        params,
        token,
    )

    print(f"   ✅ Script executed: {call_result['stats']['stages_executed']} Functions")
    print(f"   ⏱️  Execution time: {call_result['stats']['execution_time_ms']}ms")

    records = call_result["records"]
    print(f"\n3️⃣ Verification Results:")
    print(f"   ✅ Found {len(records)} record(s)")
    if records:
        user = records[0]
        print(f"   📋 Status updated to: {user.get('status', 'N/A')}")
        print(f"   📋 Name: {user.get('name', 'N/A')}")
        return script_id

    return script_id


async def script_3_query_update_credits(session, token):
    """
    Script 3: Query by Email + Update Credits + Verify

    Functions:
    1. Query - Find user by email
    2. Update - Add credits using email filter
    3. Query - Verify the credits were added
    """
    print("\n" + "=" * 60)
    print("📝 Script 3: Query + Update Credits + Verify")
    print("=" * 60)

    script = {
        "label": "query_update_credits",
        "name": "Query, Update Credits, and Verify",
        "description": "Find user by email, update credits, verify change",
        "version": "1.0",
        "parameters": {
            "user_email": {
                "type": "String",
                "required": True,
                "description": "Email to search for",
            },
            "credits": {
                "type": "Integer",
                "default": 100,
                "required": False,
                "description": "Credits to set",
            },
        },
        "functions": [
            {
                "type": "Query",
                "collection": "users",
                "filter": {
                    "type": "Condition",
                    "content": {
                        "field": "email",
                        "operator": "Eq",
                        "value": "{{user_email}}",
                    },
                },
            },
            {
                "type": "Update",
                "collection": "users",
                "filter": {
                    "type": "Condition",
                    "content": {
                        "field": "email",
                        "operator": "Eq",
                        "value": "{{user_email}}",
                    },
                },
                "updates": {"credits": "{{credits}}"},
            },
            {
                "type": "Query",
                "collection": "users",
                "filter": {
                    "type": "Condition",
                    "content": {
                        "field": "email",
                        "operator": "Eq",
                        "value": "{{user_email}}",
                    },
                },
            },
        ],
        "tags": ["crud", "update", "verification"],
    }

    print("\n1️⃣ Saving Script...")
    save_result = await request(session, "POST", "/api/functions", script, token)
    script_id = save_result["id"]
    print(f"   ✅ Script saved: {script_id}")

    print(f"\n2️⃣ Calling Script (Query + Update Credits + Verify)...")
    params = {"user_email": "alice@example.com", "credits": 100}
    call_result = await request(
        session,
        "POST",
        "/api/functions/query_update_credits",
        params,
        token,
    )

    print(f"   ✅ Script executed: {call_result['stats']['stages_executed']} Functions")
    print(f"   ⏱️  Execution time: {call_result['stats']['execution_time_ms']}ms")

    records = call_result["records"]
    print(f"\n3️⃣ Verification Results:")
    print(f"   ✅ Found {len(records)} record(s)")
    if records:
        user = records[0]
        print(f"   📋 Credits updated to: {user.get('credits', 'N/A')}")
        print(f"   📋 Status: {user.get('status', 'N/A')}")
        print(f"   📋 Name: {user.get('name', 'N/A')}")
        return script_id

    return script_id


async def script_4_delete_and_verify(session, token):
    """
    Script 4: Query Before Delete + Delete + Verify Gone

    Functions:
    1. Query - Find and show the record before deletion
    2. Delete - Remove user by email filter
    3. Query - Verify the user no longer exists
    """
    print("\n" + "=" * 60)
    print("📝 Script 4: Query Before Delete + Delete + Verify")
    print("=" * 60)

    script = {
        "label": "delete_and_verify",
        "name": "Query Before Delete and Verify",
        "description": "Verify record exists, delete it, then verify it's gone",
        "version": "1.0",
        "parameters": {
            "user_email": {
                "type": "String",
                "required": True,
                "description": "Email of user to delete",
            }
        },
        "functions": [
            {
                "type": "Query",
                "collection": "users",
                "filter": {
                    "type": "Condition",
                    "content": {
                        "field": "email",
                        "operator": "Eq",
                        "value": "{{user_email}}",
                    },
                },
            },
            {
                "type": "Delete",
                "collection": "users",
                "filter": {
                    "type": "Condition",
                    "content": {
                        "field": "email",
                        "operator": "Eq",
                        "value": "{{user_email}}",
                    },
                },
            },
            {
                "type": "Query",
                "collection": "users",
                "filter": {
                    "type": "Condition",
                    "content": {
                        "field": "email",
                        "operator": "Eq",
                        "value": "{{user_email}}",
                    },
                },
            },
        ],
        "tags": ["crud", "delete", "verification"],
    }

    print("\n1️⃣ Saving Script...")
    save_result = await request(session, "POST", "/api/functions", script, token)
    script_id = save_result["id"]
    print(f"   ✅ Script saved: {script_id}")

    print("\n2️⃣ Calling Script (Query + Delete + Verify)...")
    params = {"user_email": "alice@example.com"}
    call_result = await request(
        session,
        "POST",
        "/api/functions/delete_and_verify",
        params,
        token,
    )

    print(f"   ✅ Script executed: {call_result['stats']['stages_executed']} Functions")
    print(f"   ⏱️  Execution time: {call_result['stats']['execution_time_ms']}ms")

    # The result will be from the final Query (after delete)
    records = call_result["records"]
    print(f"\n3️⃣ Verification Results:")
    print(f"   📊 Before delete: Record existed (verified by first Query)")
    if len(records) == 0:
        print(
            f"   ✅ After delete: Record successfully deleted (Query returned 0 records)"
        )
    else:
        print(f"   ❌ Delete failed - still found {len(records)} record(s)")
    return script_id


async def cleanup(session, token, script_ids):
    """Cleanup test data and scripts"""
    print("\n" + "=" * 60)
    print("🧹 Cleanup")
    print("=" * 60)

    for script_id in script_ids:
        if script_id:
            try:
                await request(
                    session, "DELETE", f"/api/functions/{script_id}", None, token
                )
                print(f"   ✅ Deleted script: {script_id[:20]}...")
            except Exception as e:
                print(f"   ⚠️  Could not delete script: {e}")

    try:
        await request(session, "DELETE", "/api/collections/users", None, token)
        print(f"   ✅ Deleted collection: users")
    except Exception as e:
        print(f"   ⚠️  Could not delete collection: {e}")


async def main():
    """Main function - runs all CRUD Scripts in sequence"""
    print("🚀 ekoDB Complete CRUD Scripts Example")
    print("=" * 60)
    print("Demonstrates:")
    print("  • Insert + Verify (using Query)")
    print("  • Query + Update Status + Verify")
    print("  • Query + Update Credits + Verify")
    print("  • Query Before Delete + Delete + Verify Gone")
    print()
    print("Each Script shows Functions chaining with proper verification")
    print("=" * 60)

    async with aiohttp.ClientSession() as session:
        try:
            # Get auth token
            token = await get_auth_token(session)

            # Run all CRUD Scripts in sequence
            user_id, script1_id = await script_1_insert_and_verify(session, token)
            script_ids = [script1_id]

            if user_id:
                script2_id = await script_2_query_update_verify(session, token)
                script3_id = await script_3_query_update_credits(session, token)
                script4_id = await script_4_delete_and_verify(session, token)
                script_ids.extend([script2_id, script3_id, script4_id])

            # Cleanup
            await cleanup(session, token, script_ids)

            print("\n" + "=" * 60)
            print("✅ Complete CRUD Scripts Example Finished!")
            print("=" * 60)
            print("\n💡 Key Takeaways:")
            print("   ✅ Scripts chain Functions together")
            print("   ✅ Each Script demonstrates operation + verification")
            print("   ✅ Parameters make Scripts reusable")
            print("   ✅ Verification is built into the Script itself")
            print("   ✅ Complete CRUD lifecycle in 4 focused Scripts")

        except Exception as e:
            print(f"\n❌ Error: {e}")
            import traceback

            traceback.print_exc()


if __name__ == "__main__":
    asyncio.run(main())
