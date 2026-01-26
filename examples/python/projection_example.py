"""
Field Projection Examples for Python Client

Demonstrates how to use select_fields and exclude_fields to optimize
data transfer and improve query performance.

Shows comparison between full object queries and projected queries.
"""

import asyncio
import os
from datetime import datetime
from dotenv import load_dotenv
from ekodb_client import Client

load_dotenv()

BASE_URL = os.getenv("API_BASE_URL", "http://localhost:8080")
API_KEY = os.getenv("API_BASE_KEY", "a-test-api-key-from-ekodb")

TEST_COLLECTION = "projection_test_users_py"


async def main():
    # Initialize client
    client = Client.new(
        BASE_URL,
        API_KEY,
        should_retry=True,
        max_retries=3,
        timeout_secs=30,
    )
    print("Client created\n")

    # Setup: Create test collection and insert test data
    print("Setting up test data...")
    try:
        await client.delete_collection(TEST_COLLECTION)
    except Exception:
        # Collection may not exist, that's fine
        pass

    # Insert test users with various fields
    test_users = [
        {
            "name": "Alice Johnson",
            "email": "alice@example.com",
            "status": "active",
            "user_role": "admin",
            "age": 30,
            "bio": "Software engineer",
            "avatar_url": "https://example.com/alice.jpg",
            "password": "secret123",
            "api_key": "key123",
            "secret_token": "token123",
            "created_at": datetime.now().isoformat(),
        },
        {
            "name": "Bob Smith",
            "email": "bob@example.com",
            "status": "active",
            "user_role": "user",
            "age": 25,
            "bio": "Designer",
            "avatar_url": "https://example.com/bob.jpg",
            "password": "secret456",
            "api_key": "key456",
            "secret_token": "token456",
            "created_at": datetime.now().isoformat(),
        },
        {
            "name": "Carol White",
            "email": "carol@example.com",
            "status": "inactive",
            "user_role": "user",
            "age": 35,
            "bio": "Manager",
            "avatar_url": "https://example.com/carol.jpg",
            "password": "secret789",
            "api_key": "key789",
            "secret_token": "token789",
            "created_at": datetime.now().isoformat(),
        },
        {
            "name": "Dave Brown",
            "email": "dave@example.com",
            "status": "active",
            "user_role": "admin",
            "age": 45,
            "bio": "CTO",
            "avatar_url": "https://example.com/dave.jpg",
            "password": "secretabc",
            "api_key": "keyabc",
            "secret_token": "tokenabc",
            "created_at": datetime.now().isoformat(),
        },
    ]

    inserted_ids = []
    for user in test_users:
        result = await client.insert(TEST_COLLECTION, user)
        inserted_ids.append(result["id"])
    print(f"Inserted {len(inserted_ids)} test users\n")

    # Example 1: Select specific fields to reduce data transfer
    print("Example 1: Select specific fields")
    users_query = {
        "filter": {
            "type": "Condition",
            "content": {"field": "status", "operator": "Eq", "value": "active"},
        },
        "select_fields": ["id", "name", "email"],
        "limit": 10,
    }

    users = await client.find(TEST_COLLECTION, users_query)
    print(f"Fetched {len(users)} users with only 3 fields each")

    # Example 2: Exclude sensitive fields
    print("\nExample 2: Exclude sensitive fields")
    admins_query = {
        "filter": {
            "type": "Condition",
            "content": {"field": "user_role", "operator": "Eq", "value": "admin"},
        },
        "exclude_fields": ["password", "api_key", "secret_token"],
    }

    admins = await client.find(TEST_COLLECTION, admins_query)
    print(f"Fetched {len(admins)} admins without sensitive data")
    # Verify sensitive fields are excluded
    if len(admins) > 0:
        has_password = "password" in admins[0]
        print(f"  Password field excluded: {not has_password}")

    # Example 3: Complex query with projection
    print("\nExample 3: Complex query with projection")
    active_users_query = {
        "filter": {
            "type": "Logical",
            "content": {
                "operator": "And",
                "expressions": [
                    {
                        "type": "Condition",
                        "content": {
                            "field": "status",
                            "operator": "Eq",
                            "value": "active",
                        },
                    },
                    {
                        "type": "Condition",
                        "content": {"field": "age", "operator": "Gte", "value": 18},
                    },
                    {
                        "type": "Condition",
                        "content": {"field": "age", "operator": "Lt", "value": 65},
                    },
                ],
            },
        },
        "select_fields": ["id", "name", "email", "age", "created_at"],
        "sort": [{"field": "created_at", "ascending": False}],
        "limit": 50,
    }

    active_users = await client.find(TEST_COLLECTION, active_users_query)
    print(f"Fetched {len(active_users)} active users with profile fields")

    # Example 4: Find by ID with projection
    print("\nExample 4: Find by ID with projection")
    profile_query = {
        "filter": {
            "type": "Condition",
            "content": {"field": "id", "operator": "Eq", "value": inserted_ids[0]},
        },
        "select_fields": ["id", "name", "email", "bio", "avatar_url"],
    }

    user_profiles = await client.find(TEST_COLLECTION, profile_query)
    if len(user_profiles) > 0:
        user_profile = user_profiles[0]
        print(f"Fetched user profile: {user_profile.get('name', 'N/A')}")
    else:
        print("Fetched user profile: N/A")

    # Example 5: Compare full vs projected data
    print("\nExample 5: Compare full vs projected data")
    full_query = {
        "filter": {
            "type": "Condition",
            "content": {"field": "status", "operator": "Eq", "value": "active"},
        },
    }

    projected_query = {
        "filter": {
            "type": "Condition",
            "content": {"field": "status", "operator": "Eq", "value": "active"},
        },
        "select_fields": ["id", "name", "email"],
    }

    full_users = await client.find(TEST_COLLECTION, full_query)
    projected_users = await client.find(TEST_COLLECTION, projected_query)

    full_fields = len(full_users[0].keys()) if len(full_users) > 0 else 0
    projected_fields = len(projected_users[0].keys()) if len(projected_users) > 0 else 0

    print(f"Full query returned {full_fields} fields per user")
    print(f"Projected query returned {projected_fields} fields per user")

    # Cleanup
    print("\nCleaning up test data...")
    await client.delete_collection(TEST_COLLECTION)
    print("Cleanup complete")

    print("\nAll projection examples completed successfully!")


# Run examples
if __name__ == "__main__":
    asyncio.run(main())
