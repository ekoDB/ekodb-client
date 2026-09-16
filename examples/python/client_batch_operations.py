#!/usr/bin/env python3
"""Batch Operations Examples - Using ekodb_client Python library

This example demonstrates batch operations using the Python client library.
Compare with batch_operations.py to see the difference!
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


def field_value(record, key):
    """Return a direct field or the value inside an ekoDB typed wrapper."""
    value = record.get(key)
    if isinstance(value, dict) and "value" in value:
        return value["value"]
    return value


def is_not_found(error):
    message = str(error).lower()
    return "404" in message or "not found" in message


async def batch_operations_examples():
    """Run batch operations examples using ekodb_client"""
    # Create ekoDB client
    client = Client.new(BASE_URL, API_KEY)
    print("✓ Client created")

    collection = "client_batch_operations_python"
    operation_error = None

    try:
        await client.delete_collection(collection)
    except Exception as exc:
        if not is_not_found(exc):
            raise

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
        inserted_ids = [field_value(doc, "id") for doc in inserted]
        if len(inserted) != 5 or any(not doc_id for doc_id in inserted_ids):
            raise AssertionError("batch insert did not return five non-empty IDs")
        if len(set(inserted_ids)) != 5:
            raise AssertionError("batch insert returned duplicate IDs")
        print(f"✓ Batch inserted {len(inserted_ids)} records")

        # Verify the inserts
        all_docs = await client.find(collection, limit=100)
        print(f"✓ Verified: Found {len(all_docs)} total records in collection")
        if len(all_docs) != 5:
            raise AssertionError(f"expected 5 inserted records, found {len(all_docs)}")

        # Example 2: Batch Update
        print("\n=== Batch Update ===")
        updates = [
            (doc_id, {"status": "active", "updated": True})
            for doc_id in inserted_ids[:3]
        ]

        updated = await client.batch_update(collection, updates)
        print(f"✓ Batch updated {len(updated)} records")
        if len(updated) != 3:
            raise AssertionError(f"expected 3 updated records, got {len(updated)}")
        for doc_id in inserted_ids[:3]:
            record = await client.find_by_id(collection, doc_id)
            if (
                field_value(record, "status") != "active"
                or field_value(record, "updated") is not True
            ):
                raise AssertionError(f"record {doc_id} did not contain update fields")

        # Example 3: Batch Delete
        print("\n=== Batch Delete ===")
        delete_ids = inserted_ids[:3]
        deleted_count = await client.batch_delete(collection, delete_ids)
        print(f"✓ Batch deleted {deleted_count} records")
        if deleted_count != 3:
            raise AssertionError(f"expected 3 deleted records, got {deleted_count}")
        remaining = await client.find(collection, limit=100)
        remaining_ids = {field_value(record, "id") for record in remaining}
        if len(remaining) != 2 or remaining_ids.intersection(delete_ids):
            raise AssertionError(
                "batch delete did not leave exactly the two expected records"
            )

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

    print("\n✓ All batch operations completed successfully")


if __name__ == "__main__":
    asyncio.run(batch_operations_examples())
