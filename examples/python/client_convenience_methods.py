"""
ekoDB Convenience Methods Example (Python)

This example demonstrates the convenience methods added to the ekoDB Python client:
- upsert: Insert or update a record
- find_one: Find a single record by field value
- exists: Check if a record exists
- paginate: Paginate through records
"""

import asyncio
import os
from pathlib import Path
from dotenv import load_dotenv
from ekodb_client import Client

# Load environment variables
env_path = Path(__file__).parent.parent / ".env"
load_dotenv(env_path)


async def main():
    print("=== ekoDB Convenience Methods Example ===\n")

    # Create client
    base_url = os.getenv("API_BASE_URL", "http://localhost:8080")
    api_key = os.getenv("API_BASE_KEY", "a-test-api-key-from-ekodb")

    client = Client.new(base_url, api_key)

    collection = "convenience_methods_example_py"

    print("=== Native Dict Creation ===")
    # Python uses plain dicts natively - no builder needed!
    user1 = {
        "name": "Alice Johnson",
        "email": "alice@example.com",
        "age": 28,
        "active": True,
    }

    inserted = await client.insert(collection, user1)
    print(f"✓ Created record with plain dict: {inserted}")

    print("\n=== Upsert Operation ===")
    user_id = inserted["id"]

    # First upsert - will update (record exists)
    user2 = {
        "name": "Alice Johnson",
        "email": "alice.j@newdomain.com",
        "age": 29,
    }

    upserted1 = await client.upsert(collection, user_id, user2, False)
    print(f"✓ Upsert (update existing record): {upserted1['id']}")

    # Insert another record to demonstrate upsert on it
    user3 = {
        "name": "Bob Smith",
        "email": "bob@example.com",
        "age": 35,
    }
    inserted2 = await client.insert(collection, user3)
    user3_id = inserted2["id"]
    print(f"✓ Inserted second record: {user3_id}")

    # Now upsert the second record
    user3_updated = {
        "name": "Bob Smith Jr.",
        "email": "bob.jr@example.com",
        "age": 36,
    }
    upserted2 = await client.upsert(collection, user3_id, user3_updated, False)
    print(f"✓ Upsert (update second record): {upserted2['id']}")

    print("\n=== Find One Operation ===")
    # Find a single record by any field
    found_user = await client.find_one(collection, "email", "alice.j@newdomain.com")
    if found_user:
        print(f"✓ Found user by email: {found_user}")

    # Try finding a non-existent user
    not_found = await client.find_one(collection, "email", "nonexistent@example.com")
    if not not_found:
        print("✓ User not found (as expected)")

    print("\n=== Exists Check ===")
    # Check existence without fetching the record
    exists = await client.exists(collection, user_id)
    print(f"✓ Record exists: {exists}")

    # Check non-existent record
    not_exists = await client.exists(collection, "fake-id-123")
    print(f"✓ Fake record exists: {not_exists} (should be False)")

    print("\n=== Pagination ===")
    # Insert more records for pagination demo
    insert_tasks = []
    for i in range(1, 26):
        record = {
            "name": f"User {i}",
            "email": f"user{i}@example.com",
            "index": i,
        }
        insert_tasks.append(client.insert(collection, record))

    await asyncio.gather(*insert_tasks)
    print("✓ Inserted 25 records for pagination")

    # Paginate through records
    page_size = 10
    for page in range(1, 4):
        records = await client.paginate(collection, page, page_size)
        expected = "~7" if page == 3 else "10"
        print(f"✓ Page {page}: {len(records)} records (expected {expected})")

    print("\n=== Cleanup ===")
    await client.delete_collection(collection)
    print("✓ Deleted collection")

    print("\n✅ All convenience methods demonstrated successfully!")


if __name__ == "__main__":
    asyncio.run(main())
