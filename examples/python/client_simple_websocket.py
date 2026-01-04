#!/usr/bin/env python3
"""Simple WebSocket Operations - Using ekodb_client Python library

This example demonstrates WebSocket queries using the Python client library.
Compare with simple_websocket.py to see the difference!
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


async def simple_websocket_examples():
    """Run simple WebSocket examples using ekodb_client"""
    # Create ekoDB client
    client = Client.new(BASE_URL, API_KEY)
    print("✓ Client created")

    collection = "client_simple_websocket_python"

    try:
        # Step 1: Insert some test data first via HTTP
        print("\n=== Inserting Test Data ===")
        doc = await client.insert(
            collection, {"name": "WebSocket Test Record", "value": 42, "active": True}
        )
        doc_id = doc.get("id")
        print(f"✓ Inserted test record: {doc_id}")

        # Step 2: Connect to WebSocket and query data
        print("\n=== Connecting to WebSocket ===")
        ws_client = await client.websocket(WS_URL)
        print("✓ WebSocket connected")

        # Step 3: Query data via WebSocket
        print("\n=== Querying Data via WebSocket ===")
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

        print("\n✓ WebSocket example completed successfully")

    except Exception as e:
        print(f"Error: {e}")
        raise


if __name__ == "__main__":
    asyncio.run(simple_websocket_examples())
