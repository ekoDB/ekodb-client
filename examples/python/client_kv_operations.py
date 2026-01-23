#!/usr/bin/env python3
"""KV Operations Examples - Using ekodb_client Python library

This example demonstrates key-value operations using the Python client library.
Compare with kv_operations.py to see the difference!
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


async def kv_operations_examples():
    """Run KV operations examples using ekodb_client"""
    # Create ekoDB client
    client = Client.new(BASE_URL, API_KEY)
    print("✓ Client created")

    try:
        # Example 1: Set a key-value pair
        print("\n=== KV Set ===")
        await client.kv_set("session:user123", {"userId": 123, "username": "john_doe"})
        print("✓ Set key: session:user123")

        # Example 2: Get a value
        print("\n=== KV Get ===")
        result = await client.kv_get("session:user123")
        print(f"Retrieved value: {result}")

        # Example 3: Batch set multiple keys
        print("\n=== KV Batch Set ===")
        batch_entries = [
            {"key": "cache:product:1", "value": {"name": "Product 1", "price": 29.99}},
            {"key": "cache:product:2", "value": {"name": "Product 2", "price": 39.99}},
            {"key": "cache:product:3", "value": {"name": "Product 3", "price": 49.99}},
        ]
        set_results = await client.kv_batch_set(batch_entries)
        print(f"✓ Batch set {len(set_results)} keys")
        for key, was_set in set_results:
            print(f"  {key}: {'success' if was_set else 'failed'}")

        # Example 4: Batch get multiple keys
        print("\n=== KV Batch Get ===")
        keys = ["cache:product:1", "cache:product:2", "cache:product:3"]
        batch_values = await client.kv_batch_get(keys)
        print(f"✓ Batch retrieved {len(batch_values)} values")
        for i, val in enumerate(batch_values):
            print(f"  {keys[i]}: {val}")

        # Example 5: Check if key exists
        print("\n=== KV Exists ===")
        exists = await client.kv_exists("session:user123")
        print(f"Key exists: {exists}")

        # Example 6: Find keys with pattern
        print("\n=== KV Find (Pattern Query) ===")
        cache_results = await client.kv_find(pattern="cache:product:.*")
        print(f"Found {len(cache_results)} keys matching 'cache:product:.*'")

        # Example 7: Query all keys
        print("\n=== KV Query (Alias for Find) ===")
        all_results = await client.kv_query()
        print(f"Total keys in store: {len(all_results)}")

        # Example 8: Delete a key
        print("\n=== KV Delete ===")
        await client.kv_delete("session:user123")
        print("✓ Deleted key: session:user123")

        # Verify deletion with kv_exists
        exists_after = await client.kv_exists("session:user123")
        print(f"✓ Verified: Key exists after delete: {exists_after}")

        # Example 9: Batch delete multiple keys
        print("\n=== KV Batch Delete ===")
        delete_results = await client.kv_batch_delete(keys)
        print(f"✓ Batch deleted {len(delete_results)} keys")
        for key, was_deleted in delete_results:
            print(f"  {key}: {'deleted' if was_deleted else 'not found'}")

        print("\n✓ All KV operations completed successfully")

    except Exception as e:
        print(f"Error: {e}")
        raise


if __name__ == "__main__":
    asyncio.run(kv_operations_examples())
