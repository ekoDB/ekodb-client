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


async def document_ttl_examples():
    """Run document TTL examples using ekodb_client"""
    # Create ekoDB client
    client = Client.new(BASE_URL, API_KEY)
    print("✓ Client created")

    collection = "client_document_ttl_python"

    try:
        # Example 1: Insert document with TTL (1 hour)
        print("\n=== Insert Document with TTL (1 hour) ===")
        doc1 = await client.insert(
            collection,
            {"name": "Cache Entry 1", "value": "This will expire in 1 hour"},
            ttl="1h",
        )
        doc1_id = doc1.get("id")
        print(f"✓ Inserted document: {doc1_id}")

        # Example 2: Insert document with TTL (5 minutes)
        print("\n=== Insert Document with TTL (5 minutes) ===")
        doc2 = await client.insert(
            collection,
            {"name": "Cache Entry 2", "value": "This will expire in 5 minutes"},
            ttl="5m",
        )
        doc2_id = doc2.get("id")
        print(f"✓ Inserted document: {doc2_id}")

        # Example 3: Query documents
        print("\n=== Query Documents ===")
        docs = await client.find(collection, limit=100)
        print(f"✓ Found {len(docs)} documents with TTL")

        # Example 4: Update document (TTL persists)
        print("\n=== Update Document ===")
        if doc1_id:
            updated = await client.update(
                collection,
                doc1_id,
                {"name": "Updated Cache Entry", "value": "Updated value"},
            )
            print("✓ Updated document")

        # Example 5: Delete document
        print("\n=== Delete Document ===")
        if doc1_id:
            await client.delete(collection, doc1_id)
            print("✓ Deleted document")

        # Cleanup: Delete the collection
        print("\n=== Cleanup ===")
        await client.delete_collection(collection)
        print("✓ Deleted collection")

        print("\n✓ All document TTL operations completed successfully")
        print(
            "\n💡 Note: Documents with TTL will automatically expire after the specified duration"
        )

    except Exception as e:
        print(f"Error: {e}")
        raise


if __name__ == "__main__":
    asyncio.run(document_ttl_examples())
