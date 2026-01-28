#!/usr/bin/env python3
"""Collection Utilities Example - Using ekodb_client Python library

This example demonstrates collection utility methods:
- collection_exists: Check if a collection exists
- count_documents: Count documents in a collection
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


async def collection_utils_examples():
    """Run Collection Utilities API examples"""
    print("✓ Client created")

    # Create ekoDB client
    client = Client.new(BASE_URL, API_KEY)

    collection = "collection_utils_test_py"

    # Example 1: Check if collection exists (should be false initially)
    print("\n=== Check Collection Exists (Before Creation) ===")
    try:
        exists = await client.collection_exists(collection)
        print(f"Collection '{collection}' exists: {exists}")
    except Exception as e:
        print(f"CollectionExists error: {e}")

    # Example 2: Create some test documents
    print("\n=== Creating Test Documents ===")
    for i in range(1, 6):
        record = {
            "name": f"Test Document {i}",
            "index": i,
        }
        try:
            await client.insert(collection, record)
        except Exception as e:
            print(f"Insert error: {e}")
    print("Created 5 test documents")

    # Example 3: Check if collection exists now (should be true)
    print("\n=== Check Collection Exists (After Creation) ===")
    try:
        exists = await client.collection_exists(collection)
        print(f"Collection '{collection}' exists: {exists}")
    except Exception as e:
        print(f"CollectionExists error: {e}")

    # Example 4: Count documents in collection
    print("\n=== Count Documents ===")
    try:
        count = await client.count_documents(collection)
        print(f"Document count in '{collection}': {count}")
    except Exception as e:
        print(f"CountDocuments error: {e}")

    # Example 5: Check non-existent collection
    print("\n=== Check Non-Existent Collection ===")
    try:
        exists = await client.collection_exists("nonexistent_collection_xyz")
        print(f"Collection 'nonexistent_collection_xyz' exists: {exists}")
    except Exception as e:
        print(f"CollectionExists error: {e}")

    # Cleanup: Delete the test collection
    print("\n=== Cleanup ===")
    try:
        await client.delete_collection(collection)
        print(f"Deleted collection '{collection}'")
    except Exception as e:
        print(f"DeleteCollection error: {e}")

    print("\n✓ Collection Utilities example complete")


if __name__ == "__main__":
    asyncio.run(collection_utils_examples())
