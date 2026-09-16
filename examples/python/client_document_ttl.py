#!/usr/bin/env python3
"""Document TTL Examples - Using ekodb_client Python library

This example demonstrates documents with TTL using the Python client library.
Compare with document_ttl.py to see the difference!
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
    value = record.get(key)
    if isinstance(value, dict) and "value" in value:
        return value["value"]
    return value


def is_not_found(error):
    message = str(error).lower()
    return "404" in message or "not found" in message


async def document_ttl_examples():
    """Run document TTL examples using ekodb_client"""
    # Create ekoDB client
    client = Client.new(BASE_URL, API_KEY)
    print("✓ Client created")

    collection = "client_document_ttl_python"
    operation_error = None

    try:
        await client.delete_collection(collection)
    except Exception as exc:
        if not is_not_found(exc):
            raise

    try:
        # Example 1: Insert document with TTL (1 hour)
        print("\n=== Insert Document with TTL (1 hour) ===")
        doc1 = await client.insert(
            collection,
            {"name": "Cache Entry 1", "value": "This will expire in 1 hour"},
            ttl="1h",
        )
        doc1_id = field_value(doc1, "id")
        if not doc1_id:
            raise AssertionError("first insert response did not include an ID")
        print(f"✓ Inserted document: {doc1_id}")

        # Example 2: Insert document with TTL (5 minutes)
        print("\n=== Insert Document with TTL (5 minutes) ===")
        doc2 = await client.insert(
            collection,
            {"name": "Cache Entry 2", "value": "This will expire in 5 minutes"},
            ttl="5m",
        )
        doc2_id = field_value(doc2, "id")
        if not doc2_id or doc2_id == doc1_id:
            raise AssertionError("second insert did not return a distinct non-empty ID")
        print(f"✓ Inserted document: {doc2_id}")

        # Example 3: Query documents
        print("\n=== Query Documents ===")
        docs = await client.find(collection, limit=100)
        print(f"✓ Found {len(docs)} documents with TTL")
        if len(docs) != 2 or {field_value(doc, "id") for doc in docs} != {
            doc1_id,
            doc2_id,
        }:
            raise AssertionError(
                "TTL query did not return exactly the inserted documents"
            )

        # Example 4: Update document (TTL persists)
        print("\n=== Update Document ===")
        updated = await client.update(
            collection,
            doc1_id,
            {"name": "Updated Cache Entry", "value": "Updated value"},
        )
        if (
            field_value(updated, "name") != "Updated Cache Entry"
            or field_value(updated, "value") != "Updated value"
        ):
            raise AssertionError(
                "updated document did not contain the requested values"
            )
        print("✓ Updated document")

        # Example 5: Delete document
        print("\n=== Delete Document ===")
        await client.delete(collection, doc1_id)
        print("✓ Deleted document")
        remaining = await client.find(collection, limit=100)
        if len(remaining) != 1 or field_value(remaining[0], "id") != doc2_id:
            raise AssertionError("delete did not leave exactly the second TTL document")

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

    print("\n✓ All document TTL operations completed successfully")
    print(
        "\n💡 Note: Documents with TTL will automatically expire after the specified duration"
    )


if __name__ == "__main__":
    asyncio.run(document_ttl_examples())
