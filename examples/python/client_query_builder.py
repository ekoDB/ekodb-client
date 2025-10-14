"""
Query Builder Example

Demonstrates using the QueryBuilder for complex queries with:
- Comparison operators (eq, gt, lt, etc.)
- String operators (contains, startsWith, endsWith)
- Logical operators (and, or, not)
- Sorting and pagination
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
    # Create client
    base_url = os.getenv("API_BASE_URL", "http://localhost:8080")
    api_key = os.getenv("API_BASE_KEY", "a-test-api-key-from-ekodb")
    client = Client.new(base_url, api_key)

    print("=== Query Builder Examples ===\n")

    collection = "query_builder_client_py"

    # Cleanup any existing test collection
    try:
        await client.delete_collection(collection)
    except:
        pass  # Ignore errors if collection doesn't exist

    # Setup: Insert test data
    print("Setting up test data...")

    await client.insert(
        collection,
        {
            "name": "Alice Johnson",
            "email": "alice@example.com",
            "age": 28,
            "status": "active",
            "role": "user",
            "country": "USA",
        },
    )

    await client.insert(
        collection,
        {
            "name": "Bob Smith",
            "email": "bob@example.com",
            "age": 35,
            "status": "active",
            "role": "admin",
            "country": "Canada",
        },
    )

    await client.insert(
        collection,
        {
            "name": "Charlie Brown",
            "email": "charlie@test.com",
            "age": 42,
            "status": "inactive",
            "role": "moderator",
            "country": "UK",
        },
    )

    print("✅ Test data created\n")

    # Example 1: Simple equality query
    print("1. Simple equality query:")
    query1 = {"status": {"$eq": "active"}, "$limit": 5}

    results1 = await client.find(collection, query1)
    print(f"Found {len(results1)} active users\n")

    # Example 2: Range query with sorting
    print("2. Range query with sorting:")
    query2 = {"age": {"$gte": 18, "$lte": 65}, "$sort": {"age": 1}, "$limit": 10}

    results2 = await client.find(collection, query2)
    print(f"Found {len(results2)} users aged 18-65\n")

    # Example 3: String operations
    print("3. String operations:")
    query3 = {"email": {"$contains": "@example.com"}, "$sort": {"name": 1}}

    results3 = await client.find(collection, query3)
    print(f"Found {len(results3)} users with @example.com emails\n")

    # Example 4: IN operator
    print("4. IN operator:")
    query4 = {"role": {"$in": ["admin", "moderator", "editor"]}, "$limit": 20}

    results4 = await client.find(collection, query4)
    print(f"Found {len(results4)} privileged users\n")

    # Example 5: Complex query with multiple conditions
    print("5. Complex query with multiple conditions:")
    query5 = {
        "status": {"$eq": "active"},
        "age": {"$gt": 21},
        "country": {"$startsWith": "US"},
        "$limit": 15,
    }

    results5 = await client.find(collection, query5)
    print(f"Found {len(results5)} active US users over 21\n")

    # Example 6: Pagination
    print("6. Pagination:")
    page_size = 2
    page = 0

    query6 = {"$limit": page_size, "$offset": page * page_size, "$sort": {"name": 1}}

    results6 = await client.find(collection, query6)
    print(f"Page 1: {len(results6)} users\n")

    # Example 7: NOT IN operator
    print("7. NOT IN operator:")
    query7 = {"status": {"$nin": ["inactive", "banned"]}, "$limit": 10}

    results7 = await client.find(collection, query7)
    print(f"Found {len(results7)} valid users\n")

    # Example 8: Using bypass flags
    print("8. Using bypass flags:")
    query8 = {"status": {"$eq": "active"}, "$bypass_cache": True, "$limit": 10}

    results8 = await client.find(collection, query8)
    print(f"Found {len(results8)} users (bypassed cache)\n")

    # Cleanup
    print("=== Cleanup ===")
    await client.delete_collection(collection)
    print("✅ Deleted test collection\n")

    print("✅ Query Builder examples completed!")


if __name__ == "__main__":
    asyncio.run(main())
