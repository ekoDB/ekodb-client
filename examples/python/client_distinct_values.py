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

COLLECTION = "distinct_values_example_py"

# Load environment variables
env_path = Path(__file__).parent.parent / ".env"
load_dotenv(env_path)


async def run_example(client):
    print("=== Distinct Values Example ===\n")

    collection = COLLECTION

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


async def cleanup(client):
    try:
        await client.delete_collection(COLLECTION)
    except Exception as error:
        if "404" not in str(error) and "not found" not in str(error).lower():
            raise


async def main():
    client = Client.new(
        os.getenv("API_BASE_URL", "http://localhost:8080"),
        os.getenv("API_BASE_KEY", "a-test-api-key-from-ekodb"),
    )
    await cleanup(client)
    primary_error = None
    try:
        await run_example(client)
    except BaseException as error:  # noqa: BLE001 - cleanup must run on cancellation
        primary_error = error
    cleanup_error = None
    try:
        await cleanup(client)
    except Exception as error:  # noqa: BLE001 - preserve the primary failure
        cleanup_error = error
    if primary_error is not None:
        if cleanup_error is not None:
            primary_error.add_note(f"Cleanup also failed: {cleanup_error}")
            print(f"⚠️  Cleanup also failed: {cleanup_error}")
        raise primary_error
    if cleanup_error is not None:
        raise cleanup_error
    print("Cleanup done.")


if __name__ == "__main__":
    asyncio.run(main())
