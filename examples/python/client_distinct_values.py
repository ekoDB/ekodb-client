"""
Distinct Values Example

Demonstrates how to retrieve all unique values for a field in a collection,
with and without filters.

Run with: python client_distinct_values.py
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
    base_url = os.getenv("API_BASE_URL", "http://localhost:8080")
    api_key = os.getenv("API_BASE_KEY", "a-test-api-key-from-ekodb")
    client = Client.new(base_url, api_key)

    print("=== Distinct Values Example ===\n")

    collection = "distinct_values_example_py"

    # Cleanup from previous runs
    try:
        await client.delete_collection(collection)
    except Exception:
        pass

    # -------------------------------------------------------------------------
    # Step 1: Insert sample data
    # -------------------------------------------------------------------------
    print("Inserting sample products...")

    products = [
        {"name": "Widget A", "category": "electronics", "status": "active"},
        {"name": "Widget B", "category": "electronics", "status": "active"},
        {"name": "Gadget X", "category": "electronics", "status": "discontinued"},
        {"name": "Book 1", "category": "books", "status": "active"},
        {"name": "Book 2", "category": "books", "status": "active"},
        {"name": "Book 3", "category": "books", "status": "archived"},
        {"name": "Shirt M", "category": "clothing", "status": "active"},
        {"name": "Shirt L", "category": "clothing", "status": "active"},
    ]

    for product in products:
        await client.insert(collection, product)

    print(f"Inserted {len(products)} products\n")

    # -------------------------------------------------------------------------
    # Step 2: All distinct categories (no filter)
    # -------------------------------------------------------------------------
    print("=== Distinct Categories (all products) ===")
    resp = await client.distinct_values(collection, "category")
    print(f"Found {resp['count']} distinct categories:")
    for v in resp["values"]:
        print(f"  - {v}")
    print()

    # -------------------------------------------------------------------------
    # Step 3: All distinct statuses (no filter)
    # -------------------------------------------------------------------------
    print("=== Distinct Statuses (all products) ===")
    resp = await client.distinct_values(collection, "status")
    print(f"Found {resp['count']} distinct statuses:")
    for v in resp["values"]:
        print(f"  - {v}")
    print()

    # -------------------------------------------------------------------------
    # Step 4: Distinct statuses within electronics only
    # -------------------------------------------------------------------------
    print("=== Distinct Statuses in Electronics ===")
    electronics_filter = {
        "type": "Condition",
        "content": {
            "field": "category",
            "operator": "Eq",
            "value": "electronics",
        },
    }
    resp = await client.distinct_values(collection, "status", filter=electronics_filter)
    print(f"Found {resp['count']} distinct statuses for electronics:")
    for v in resp["values"]:
        print(f"  - {v}")
    print()

    # -------------------------------------------------------------------------
    # Cleanup
    # -------------------------------------------------------------------------
    await client.delete_collection(collection)
    print("Cleanup done.")


if __name__ == "__main__":
    asyncio.run(main())
