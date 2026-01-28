#!/usr/bin/env python3
"""User Functions API Example - Using ekodb_client Python library

This example demonstrates CRUD operations for User Functions.
User Functions are reusable sequences of Functions that can be called by Scripts.
"""

import asyncio
import os
from pathlib import Path
from dotenv import load_dotenv

from ekodb_client import Client

# Load environment variables
env_path = Path(__file__).parent.parent / ".env"
load_dotenv(env_path)

BASE_URL = os.getenv("API_BASE_URL", "http://localhost:8080")
API_KEY = os.getenv("API_BASE_KEY", "a-test-api-key-from-ekodb")


async def user_functions_examples():
    """Run User Functions API examples"""
    print("✓ Client created")

    # Create ekoDB client
    client = Client.new(BASE_URL, API_KEY)

    # Example 1: Create a User Function
    print("\n=== Create User Function ===")
    user_func = {
        "label": "get_active_users_py",
        "name": "Get Active Users",
        "description": "Fetches all users and filters by active status",
        "version": "1.0.0",
        "parameters": {
            "collection": {
                "required": True,
                "description": "Collection to query",
            }
        },
        "functions": [{"type": "FindAll", "collection": "{{collection}}"}],
        "tags": ["users", "query"],
    }

    try:
        func_id = await client.save_user_function(user_func)
        print(f"Created user function with ID: {func_id}")
    except Exception as e:
        print(f"SaveUserFunction error: {e}")

    # Example 2: Get User Function by label
    print("\n=== Get User Function ===")
    try:
        retrieved = await client.get_user_function("get_active_users_py")
        print(f"Retrieved: {retrieved.get('label')} - {retrieved.get('name')}")
        print(f"Description: {retrieved.get('description')}")
    except Exception as e:
        print(f"GetUserFunction error: {e}")

    # Example 3: List all User Functions
    print("\n=== List All User Functions ===")
    try:
        all_funcs = await client.list_user_functions()
        print(f"Found {len(all_funcs)} user functions:")
        for f in all_funcs:
            print(f"  - {f.get('label')}: {f.get('name')}")
    except Exception as e:
        print(f"ListUserFunctions error: {e}")

    # Example 4: Update User Function
    print("\n=== Update User Function ===")
    try:
        user_func["description"] = "Fetches all users - updated description"
        await client.update_user_function("get_active_users_py", user_func)
        print("User function updated successfully")
    except Exception as e:
        print(f"UpdateUserFunction error: {e}")

    # Example 5: Delete User Function
    print("\n=== Delete User Function ===")
    try:
        await client.delete_user_function("get_active_users_py")
        print("User function deleted successfully")
    except Exception as e:
        print(f"DeleteUserFunction error: {e}")

    print("\n✓ User Functions API example complete")


if __name__ == "__main__":
    asyncio.run(user_functions_examples())
