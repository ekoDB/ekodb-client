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

    # Example 1: Select specific fields - only get id, name, email
    print("Example 1: Select specific fields (id, name, email only)")
    users_query = {
        "filter": {
            "type": "Condition",
            "content": {"field": "status", "operator": "Eq", "value": "active"},
        },
        "select_fields": ["id", "name", "email"],
        "limit": 10,
    }

    users = await client.find(TEST_COLLECTION, users_query)
    print(f"  Found {len(users)} active users")
    if len(users) > 0:
        fields = list(users[0].keys())
        print(f"  Fields returned: {fields}")
        # Show first user's data
        name = users[0].get("name", "N/A")
        email = users[0].get("email", "N/A")
        print(f"  First user: {name} <{email}>")

    # Example 2: Exclude sensitive fields - hide password, api_key, secret_token
    print("\nExample 2: Exclude sensitive fields (password, api_key, secret_token)")
    admins_query = {
        "filter": {
            "type": "Condition",
            "content": {"field": "user_role", "operator": "Eq", "value": "admin"},
        },
        "exclude_fields": ["password", "api_key", "secret_token"],
    }

    admins = await client.find(TEST_COLLECTION, admins_query)
    print(f"  Found {len(admins)} admins")
    if len(admins) > 0:
        has_password = "password" in admins[0]
        has_api_key = "api_key" in admins[0]
        has_token = "secret_token" in admins[0]
        print("  Sensitive fields excluded:")
        print(
            f"    - password: {'PRESENT (unexpected!)' if has_password else 'excluded'}"
        )
        print(
            f"    - api_key: {'PRESENT (unexpected!)' if has_api_key else 'excluded'}"
        )
        print(
            f"    - secret_token: {'PRESENT (unexpected!)' if has_token else 'excluded'}"
        )
        fields = list(admins[0].keys())
        print(f"  Fields returned: {fields}")

    # Example 3: Complex query with projection - active users with profile fields
    print("\nExample 3: Complex query with projection (active users, ages 18-65)")
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
        "sort": [{"field": "age", "ascending": False}],
        "limit": 50,
    }

    active_users = await client.find(TEST_COLLECTION, active_users_query)
    print(f"  Found {len(active_users)} active users (ages 18-65)")
    for user in active_users:
        name = user.get("name", "N/A")
        age = user.get("age", "N/A")
        print(f"    - {name} (age {age})")

    # Example 4: Query inactive users with profile fields
    print("\nExample 4: Query inactive users with profile fields")
    inactive_query = {
        "filter": {
            "type": "Condition",
            "content": {"field": "status", "operator": "Eq", "value": "inactive"},
        },
        "select_fields": ["id", "name", "email", "bio"],
    }

    inactive_users = await client.find(TEST_COLLECTION, inactive_query)
    print(f"  Found {len(inactive_users)} inactive users")
    for user in inactive_users:
        name = user.get("name", "N/A")
        bio = user.get("bio", "N/A")
        print(f"    - {name}: {bio}")

    # Example 5: Compare full vs projected data - demonstrates bandwidth savings
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

    if len(full_users) > 0 and len(projected_users) > 0:
        full_fields = list(full_users[0].keys())
        projected_fields = list(projected_users[0].keys())

        print("  Full query:")
        print(f"    - {len(full_fields)} fields per record")
        print(f"    - Fields: {full_fields}")
        print("  Projected query:")
        print(f"    - {len(projected_fields)} fields per record")
        print(f"    - Fields: {projected_fields}")
        savings = 100 - (len(projected_fields) * 100 // max(len(full_fields), 1))
        print(f"  Bandwidth savings: ~{savings}% fewer fields")

    # Cleanup
    print("\nCleaning up test data...")
    await client.delete_collection(TEST_COLLECTION)
    print("Cleanup complete")

    print("\nAll projection examples completed successfully!")


# Run examples
if __name__ == "__main__":
    asyncio.run(main())
