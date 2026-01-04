#!/usr/bin/env python3
"""Simple CRUD Examples - Using ekodb_client Python library

This example demonstrates the Python client library wrapping the Rust ekodb_client.
Compare with simple_crud.py to see the difference!
"""

import asyncio
import os
from pathlib import Path
from dotenv import load_dotenv

# Import the ekodb_client (Python bindings for Rust client)
from ekodb_client import (
    Client,
    get_value,
    extract_record,
    get_datetime_value,
    get_uuid_value,
    get_decimal_value,
    get_bytes_value,
    get_array_value,
    get_set_value,
    get_vector_value,
    get_object_value,
)

# Load environment variables
env_path = Path(__file__).parent.parent / ".env"
load_dotenv(env_path)

BASE_URL = os.getenv("API_BASE_URL", "http://localhost:8080")
API_KEY = os.getenv("API_BASE_KEY", "a-test-api-key-from-ekodb")


async def simple_crud_examples():
    """Run simple CRUD examples using ekodb_client"""
    print("✓ Client created (token exchange happens automatically)")

    # Create ekoDB client (token exchange happens automatically)
    client = Client.new(BASE_URL, API_KEY)

    collection = "simple_crud_client_py"

    try:
        # Example 1: Insert a document with various field types
        print("\n=== Insert Document ===")
        from datetime import datetime
        import base64

        doc = await client.insert(
            collection,
            {
                "name": "Test Record",
                "value": 42,
                "active": True,
                "price": 99.99,
                "created_at": datetime.now().isoformat(),
                "user_id": "550e8400-e29b-41d4-a716-446655440000",
                "tags": ["tag1", "tag2", "tag3"],
                "metadata": {"key": "value", "nested": {"deep": True}},
                "embedding": [0.1, 0.2, 0.3, 0.4, 0.5],
                "categories": ["electronics", "computers"],
                "data": base64.b64encode(b"hello world").decode("utf-8"),
            },
        )
        print(f"Inserted: {doc}")
        doc_id = doc["id"]

        # Example 2: Find by ID
        print("\n=== Find by ID ===")
        found_doc = await client.find_by_id(collection, doc_id)
        print(f"Found: {found_doc}")

        # Example 2b: Extract field values using type-specific getValue utilities
        print("\n=== Extract Field Values (All Types) ===")
        # ekoDB returns fields as {"type": "String", "value": "actual_value"}
        # Use type-specific getValue utilities to extract values
        name = get_value(found_doc.get("name"))
        value_num = get_value(found_doc.get("value"))
        active = get_value(found_doc.get("active"))
        price = get_decimal_value(found_doc.get("price"))
        created_at = get_datetime_value(found_doc.get("created_at"))
        user_id = get_uuid_value(found_doc.get("user_id"))
        tags = get_array_value(found_doc.get("tags"))
        metadata = get_object_value(found_doc.get("metadata"))
        embedding = get_vector_value(found_doc.get("embedding"))
        categories = get_set_value(found_doc.get("categories"))
        data = get_bytes_value(found_doc.get("data"))

        print("Extracted values:")
        print(f"  name (String): {name}")
        print(f"  value (Integer): {value_num}")
        print(f"  active (Boolean): {active}")
        print(f"  price (Decimal): {price}")
        print(f"  created_at (DateTime): {created_at}")
        print(f"  user_id (UUID): {user_id}")
        print(f"  tags (Array): {tags}")
        print(f"  metadata (Object): {metadata}")
        print(f"  embedding (Vector): {embedding}")
        print(f"  categories (Set): {categories}")
        print(f"  data (Bytes): {len(data) if data else 0} bytes")

        # Or extract all fields at once using extract_record()
        plain_record = extract_record(found_doc)
        print(f"Plain record: {plain_record}")

        # Example 3: Find with query
        print("\n=== Find with Query ===")
        docs = await client.find(collection, limit=10)
        print(f"Found documents: {len(docs)}")

        # Example 4: Update document
        print("\n=== Update Document ===")
        updated = await client.update(
            collection, doc_id, {"name": "Updated Record", "value": 100}
        )
        print(f"Updated: {updated}")

        # Example 5: Delete document
        print("\n=== Delete Document ===")
        await client.delete(collection, doc_id)
        print("Deleted document")

        # Cleanup: Delete the collection
        print("\n=== Cleanup ===")
        await client.delete_collection(collection)
        print("✓ Deleted collection")

        print("\n✓ All CRUD operations completed successfully")

    except Exception as e:
        print(f"Error: {e}")
        raise


if __name__ == "__main__":
    asyncio.run(simple_crud_examples())
