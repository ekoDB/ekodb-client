#!/usr/bin/env python3
"""Collection Utilities Example - Using ekodb_client Python library

This example demonstrates collection utility methods:
- collection_exists: Check if a collection exists
- count_documents: Count documents in a collection
"""

import asyncio
import os
import uuid
from pathlib import Path

from dotenv import load_dotenv

from ekodb_client import Client

# Load environment variables
env_path = Path(__file__).parent.parent / ".env"
load_dotenv(env_path)

BASE_URL = os.getenv("API_BASE_URL", "http://localhost:8080")
API_KEY = os.getenv("API_BASE_KEY", "a-test-api-key-from-ekodb")


def is_not_found(error):
    message = str(error).lower()
    return "404" in message or "not found" in message


async def collection_utils_examples():
    """Run Collection Utilities API examples"""
    print("✓ Client created")

    # Create ekoDB client
    client = Client.new(BASE_URL, API_KEY)

    collection = "collection_utils_test_py"
    missing_collection = f"nonexistent_collection_xyz_py_{uuid.uuid4().hex}"
    operation_error = None

    # Start from a deterministic state without touching any collection this
    # example does not own.
    try:
        await client.delete_collection(collection)
    except Exception as exc:
        if not is_not_found(exc):
            raise

    try:
        # Example 1: Check if collection exists (should be false initially)
        print("\n=== Check Collection Exists (Before Creation) ===")
        try:
            exists = await client.collection_exists(collection)
        except Exception as exc:
            print(f"CollectionExists error: {exc}")
            raise
        print(f"Collection '{collection}' exists: {exists}")
        if exists:
            raise AssertionError(f"collection '{collection}' unexpectedly exists")

        # Example 2: Create some test documents
        print("\n=== Creating Test Documents ===")
        for i in range(1, 6):
            record = {
                "name": f"Test Document {i}",
                "index": i,
            }
            try:
                await client.insert(collection, record)
            except Exception as exc:
                print(f"Insert error: {exc}")
                raise
        print("Created 5 test documents")

        # Example 3: Check if collection exists now (should be true)
        print("\n=== Check Collection Exists (After Creation) ===")
        try:
            exists = await client.collection_exists(collection)
        except Exception as exc:
            print(f"CollectionExists error: {exc}")
            raise
        print(f"Collection '{collection}' exists: {exists}")
        if not exists:
            raise AssertionError(f"collection '{collection}' was not created")

        # Example 4: Count documents in collection
        print("\n=== Count Documents ===")
        try:
            count = await client.count_documents(collection)
        except Exception as exc:
            print(f"CountDocuments error: {exc}")
            raise
        print(f"Document count in '{collection}': {count}")
        if count != 5:
            raise AssertionError(f"expected 5 documents, found {count}")

        # Example 5: Check non-existent collection
        print("\n=== Check Non-Existent Collection ===")
        try:
            exists = await client.collection_exists(missing_collection)
        except Exception as exc:
            print(f"CollectionExists error: {exc}")
            raise
        print(f"Collection 'nonexistent_collection_xyz' exists: {exists}")
        if exists:
            raise AssertionError(
                f"collection '{missing_collection}' unexpectedly exists"
            )
    except Exception as exc:  # noqa: BLE001 - preserve the primary example failure
        print(f"Collection utilities error: {exc}")
        operation_error = exc

    # Cleanup: Delete the test collection
    print("\n=== Cleanup ===")
    try:
        try:
            await client.delete_collection(collection)
            print(f"Deleted collection '{collection}'")
        except Exception as exc:
            if not is_not_found(exc):
                raise
        if await client.collection_exists(collection):
            raise AssertionError(
                f"collection '{collection}' still exists after cleanup"
            )
    except Exception as cleanup_error:
        print(f"DeleteCollection error: {cleanup_error}")
        if operation_error is not None:
            operation_error.add_note(f"cleanup also failed: {cleanup_error}")
        else:
            raise

    if operation_error is not None:
        raise operation_error

    print("\n✓ Collection Utilities example complete")


if __name__ == "__main__":
    asyncio.run(collection_utils_examples())
