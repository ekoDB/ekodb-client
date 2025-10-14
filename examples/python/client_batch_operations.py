#!/usr/bin/env python3
"""Batch Operations Examples - Using ekodb_client Python library

This example demonstrates batch operations using the Python client library.
Compare with ttl-caching/batch_operations.py to see the difference!
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


async def batch_operations_examples():
    """Run batch operations examples using ekodb_client"""
    # Create ekoDB client
    client = Client.new(BASE_URL, API_KEY)
    print("✓ Client created")

    collection = "client_batch_operations_python"

    try:
        # Example 1: Batch Insert
        print("\n=== Batch Insert ===")
        records = [
            {
                "name": f"User {i+1}",
                "email": f"user{i+1}@example.com",
                "age": 25 + (i * 5),
            }
            for i in range(5)
        ]

        inserted = await client.batch_insert(collection, records)
        inserted_ids = [doc.get("id") for doc in inserted if doc.get("id")]
        print(f"✓ Batch inserted {len(inserted_ids)} records")

        # Verify the inserts
        all_docs = await client.find(collection, limit=100)
        print(f"✓ Verified: Found {len(all_docs)} total records in collection")

        # Example 2: Batch Update
        print("\n=== Batch Update ===")
        updates = [
            (doc_id, {"status": "active", "updated": True})
            for doc_id in inserted_ids[:3]
            if doc_id
        ]

        updated = await client.batch_update(collection, updates)
        print(f"✓ Batch updated {len(updated)} records")

        # Example 3: Batch Delete
        print("\n=== Batch Delete ===")
        delete_ids = [doc_id for doc_id in inserted_ids[:3] if doc_id]
        deleted_count = await client.batch_delete(collection, delete_ids)
        print(f"✓ Batch deleted {deleted_count} records")

        # Cleanup: Delete the collection
        print("\n=== Cleanup ===")
        await client.delete_collection(collection)
        print("✓ Deleted collection")

        print("\n✓ All batch operations completed successfully")

    except Exception as e:
        print(f"Error: {e}")
        raise


if __name__ == "__main__":
    asyncio.run(batch_operations_examples())
