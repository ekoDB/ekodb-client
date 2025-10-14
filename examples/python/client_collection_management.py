#!/usr/bin/env python3
"""Collection Management Examples - Using ekodb_client Python library

This example demonstrates collection management using the Python client library.
Compare with ttl-caching/collection_management.py to see the difference!
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


async def collection_management_examples():
    """Run collection management examples using ekodb_client"""
    # Create ekoDB client
    client = Client.new(BASE_URL, API_KEY)
    print("✓ Client created")

    collection = "client_collection_management_python"

    try:
        # Example 1: Create collection (via insert)
        print("\n=== Create Collection (via insert) ===")
        doc = await client.insert(collection, {"name": "First Record", "value": 1})
        print(f'Collection created with first record: "{doc["id"]}"')

        # Example 2: List collections
        print("\n=== List Collections ===")
        collections = await client.list_collections()
        print(f"Total collections: {len(collections)}")
        print(f"Sample collections: {collections[:5]}")

        # Example 3: Count documents (find with large limit)
        print("\n=== Count Documents ===")
        docs = await client.find(collection, limit=1000)
        print(f"Document count: {len(docs)}")

        # Example 4: Delete collection
        print("\n=== Delete Collection ===")
        await client.delete_collection(collection)
        print("Collection deleted successfully")

        # Example 5: Verify deletion
        print("\n=== Verify Deletion ===")
        collections_after = await client.list_collections()
        still_exists = collection in collections_after
        print(f"Collection still exists: {still_exists}")

        print("\n✓ All collection management operations completed successfully")

    except Exception as e:
        print(f"Error: {e}")
        raise


if __name__ == "__main__":
    asyncio.run(collection_management_examples())
