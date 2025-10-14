#!/usr/bin/env python3
"""WebSocket TTL Examples - Using ekodb_client Python library

This example demonstrates WebSocket queries with TTL documents using the Python client library.
Compare with ttl-caching/websocket_ttl.py to see the difference!
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
WS_URL = os.getenv("WS_BASE_URL", "ws://localhost:8080")
API_KEY = os.getenv("API_BASE_KEY", "a-test-api-key-from-ekodb")


async def websocket_ttl_examples():
    """Run WebSocket TTL examples using ekodb_client"""
    # Create ekoDB client
    client = Client.new(BASE_URL, API_KEY)
    print("✓ Client created")

    collection = "client_websocket_ttl_python"

    try:
        # Step 1: Insert test data with TTL
        print("\n=== Insert Test Data with TTL ===")
        doc = await client.insert(
            collection,
            {"name": "WebSocket TTL Test", "value": 42, "active": True},
            ttl="1h",
        )
        doc_id = doc.get("id")
        print(f"✓ Inserted document with TTL: {doc_id}")

        # Step 2: Query via WebSocket
        print("\n=== Query via WebSocket ===")
        ws_client = await client.websocket(WS_URL)
        print("✓ WebSocket connected")

        records = await ws_client.find_all(collection)
        print(f"✓ Retrieved {len(records)} record(s) via WebSocket")

        # Show some of the data
        for i, record in enumerate(records[:3], 1):
            field_count = len(record) if isinstance(record, dict) else 0
            print(f"  Record {i}: {field_count} fields")

        # Cleanup: Delete the collection
        print("\n=== Cleanup ===")
        await client.delete_collection(collection)
        print("✓ Deleted collection")

        print("\n✓ WebSocket TTL example completed successfully")
        print(
            "\n💡 Note: Documents with TTL will automatically expire after the specified duration"
        )

    except Exception as e:
        print(f"Error: {e}")
        raise


if __name__ == "__main__":
    asyncio.run(websocket_ttl_examples())
