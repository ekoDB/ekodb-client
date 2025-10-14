#!/usr/bin/env python3
"""KV Operations Examples - Using ekodb_client Python library

This example demonstrates key-value operations using the Python client library.
Compare with ttl-caching/kv_operations.py to see the difference!
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

        # Example 3: Set multiple keys
        print("\n=== Set Multiple Keys ===")
        keys = ["cache:product:1", "cache:product:2", "cache:product:3"]
        for i, key in enumerate(keys):
            await client.kv_set(
                key, {"name": f"Product {i+1}", "price": 29.99 + (i * 10)}
            )
        print(f"✓ Set {len(keys)} keys")

        # Example 4: Get multiple keys
        print("\n=== Get Multiple Keys ===")
        for key in keys:
            data = await client.kv_get(key)
            print(f"{key}: {data}")

        # Example 5: Delete a key
        print("\n=== KV Delete ===")
        await client.kv_delete("session:user123")
        print("✓ Deleted key: session:user123")

        # Verify deletion
        verify_result = await client.kv_get("session:user123")
        if verify_result is None:
            print(f"✓ Verified: Key successfully deleted (not found)")
        else:
            print(f"✗ Warning: Key still exists after delete!")

        # Example 6: Delete multiple keys
        print("\n=== Delete Multiple Keys ===")
        for key in keys:
            await client.kv_delete(key)
        print(f"✓ Deleted {len(keys)} keys")

        print("\n✓ All KV operations completed successfully")

    except Exception as e:
        print(f"Error: {e}")
        raise


if __name__ == "__main__":
    asyncio.run(kv_operations_examples())
