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


def field_value(record, key):
    value = record.get(key)
    if isinstance(value, dict) and "value" in value:
        return value["value"]
    return value


def is_not_found(error):
    message = str(error).lower()
    return "404" in message or "not found" in message


async def simple_websocket_examples():
    """Run simple WebSocket examples using ekodb_client"""
    # Create ekoDB client
    client = Client.new(BASE_URL, API_KEY)
    print("✓ Client created")

    collection = "client_simple_websocket_python"
    operation_error = None

    try:
        await client.delete_collection(collection)
    except Exception as exc:
        if not is_not_found(exc):
            raise

    try:
        # Step 1: Insert some test data first via HTTP
        print("\n=== Inserting Test Data ===")
        doc = await client.insert(
            collection, {"name": "WebSocket Test Record", "value": 42, "active": True}
        )
        doc_id = field_value(doc, "id")
        if not doc_id:
            raise AssertionError("insert response did not include an ID")
        print(f"✓ Inserted test record: {doc_id}")

        # Step 2: Connect to WebSocket and query data
        print("\n=== Connecting to WebSocket ===")
        ws_client = await client.websocket(WS_URL)
        print("✓ WebSocket connected")

        # Step 3: Query data via WebSocket
        print("\n=== Querying Data via WebSocket ===")
        records = await ws_client.find_all(collection)
        print(f"✓ Retrieved {len(records)} record(s) via WebSocket")
        if len(records) != 1:
            raise AssertionError(f"expected 1 WebSocket record, got {len(records)}")
        record = records[0]
        expected = {
            "id": doc_id,
            "name": "WebSocket Test Record",
            "value": 42,
            "active": True,
        }
        for key, expected_value in expected.items():
            actual = field_value(record, key)
            if actual != expected_value:
                raise AssertionError(
                    f"WebSocket field {key!r}: expected {expected_value!r}, got {actual!r}"
                )

        # Show some of the data
        for i, record in enumerate(records[:3], 1):
            field_count = len(record) if isinstance(record, dict) else 0
            print(f"  Record {i}: {field_count} fields")

    except Exception as exc:  # noqa: BLE001 - preserve the primary example failure
        print(f"Error: {exc}")
        operation_error = exc

    print("\n=== Cleanup ===")
    try:
        try:
            await client.delete_collection(collection)
            print("✓ Deleted collection")
        except Exception as exc:
            if not is_not_found(exc):
                raise
    except Exception as cleanup_error:
        print(f"Cleanup error: {cleanup_error}")
        if operation_error is not None:
            operation_error.add_note(f"cleanup also failed: {cleanup_error}")
        else:
            raise

    if operation_error is not None:
        raise operation_error

    print("\n✓ WebSocket example completed successfully")


if __name__ == "__main__":
    asyncio.run(simple_websocket_examples())
