#!/usr/bin/env python3
"""Simple CRUD Examples - Using ekodb_client Python library

This example demonstrates the Python client library wrapping the Rust ekodb_client.
Compare with ttl-caching/simple_crud.py to see the difference!
"""

import asyncio
import os
from pathlib import Path
from dotenv import load_dotenv

# Import the ekodb_client (Python bindings for Rust client)
from ekodb_client import Client

# Load environment variables
env_path = Path(__file__).parent.parent / ".env"
load_dotenv(env_path)

BASE_URL = os.getenv("API_BASE_URL", "http://localhost:8080")
API_KEY = os.getenv("API_BASE_KEY", "a-test-api-key-from-ekodb")


async def simple_crud_examples():
    """Run simple CRUD examples using ekodb_client"""
    print("✓ Client created (token exchange happens automatically)")

    # Create ekoDB client (token exchange happens automatically)
    client = Client.new(BASE_URL, API_KEY)

    collection = "simple_crud_client_py"

    try:
        # Example 1: Insert a document
        print("\n=== Insert Document ===")
        doc = await client.insert(
            collection, {"name": "Test Record", "value": 42, "active": True}
        )
        print(f"Inserted: {doc}")
        doc_id = doc["id"]

        # Example 2: Find by ID
        print("\n=== Find by ID ===")
        found_doc = await client.find_by_id(collection, doc_id)
        print(f"Found: {found_doc}")

        # Example 3: Find with query
        print("\n=== Find with Query ===")
        docs = await client.find(collection, limit=10)
        print(f"Found documents: {len(docs)}")

        # Example 4: Update document
        print("\n=== Update Document ===")
        updated = await client.update(
            collection, doc_id, {"name": "Updated Record", "value": 100}
        )
        print(f"Updated: {updated}")

        # Example 5: Delete document
        print("\n=== Delete Document ===")
        await client.delete(collection, doc_id)
        print("Deleted document")

        # Cleanup: Delete the collection
        print("\n=== Cleanup ===")
        await client.delete_collection(collection)
        print("✓ Deleted collection")

        print("\n✓ All CRUD operations completed successfully")

    except Exception as e:
        print(f"Error: {e}")
        raise


if __name__ == "__main__":
    asyncio.run(simple_crud_examples())
