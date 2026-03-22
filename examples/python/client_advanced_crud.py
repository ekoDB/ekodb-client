"""ekoDB Python Client - Advanced CRUD Integration Example

Demonstrates advanced CRUD operations:
  - update_with_action (atomic field-level actions)
  - update_with_action_sequence (multiple atomic actions in one call)
  - restore_deleted (restore a soft-deleted document)
  - restore_collection (restore all soft-deleted documents in a collection)
"""

import asyncio
import os

from ekodb_client import Client


async def main():
    print("=== ekoDB Advanced CRUD Example (Python) ===\n")

    base_url = os.getenv("API_BASE_URL", "http://localhost:8080")
    api_key = os.getenv("API_BASE_KEY")
    if not api_key:
        raise ValueError("API_BASE_KEY environment variable is required")

    client = Client.new(base_url, api_key)

    collection = "advanced_crud_example_py"

    try:
        # 1. Insert a document to work with
        print("--- insert ---")
        doc = await client.insert(
            collection,
            {
                "name": "Counter Record",
                "count": 10,
                "score": 50.0,
                "tags": ["initial"],
            },
        )
        doc_id = doc["id"]
        print(f"Inserted: {doc_id}")

        # 2. update_with_action - increment a field
        print("\n--- update_with_action (increment) ---")
        updated = await client.update_with_action(
            collection, doc_id, "increment", "count", 5
        )
        print(f"After increment count by 5: {updated}")

        # 3. update_with_action - decrement a field
        print("\n--- update_with_action (decrement) ---")
        updated = await client.update_with_action(
            collection, doc_id, "decrement", "score", 10.0
        )
        print(f"After decrement score by 10: {updated}")

        # 4. update_with_action_sequence - multiple atomic actions at once
        print("\n--- update_with_action_sequence ---")
        updated = await client.update_with_action_sequence(
            collection,
            doc_id,
            [
                ("increment", "count", 100),
                ("set", "name", "Updated Counter"),
            ],
        )
        print(f"After action sequence: {updated}")

        # 5. Delete the document (soft delete)
        print("\n--- delete (soft) ---")
        await client.delete(collection, doc_id)
        print(f"Deleted document: {doc_id}")

        # 6. restore_deleted - bring it back
        print("\n--- restore_deleted ---")
        restored = await client.restore_deleted(collection, doc_id)
        print(f"Restored: {restored}")

        # 7. Verify the document is back
        print("\n--- verify restore ---")
        found = await client.find_by_id(collection, doc_id)
        print(f"Found after restore: {found}")

        # 8. Delete all docs, then restore_collection
        print("\n--- restore_collection ---")
        doc2 = await client.insert(collection, {"name": "Second Record", "value": 42})
        doc2_id = doc2["id"]
        await client.delete(collection, doc_id)
        await client.delete(collection, doc2_id)
        count = await client.restore_collection(collection)
        print(f"Restored {count} documents in collection")

    finally:
        # Cleanup
        print("\n--- cleanup ---")
        await client.delete_collection(collection)
        print("Cleaned up collection")

    print("\n=== Advanced CRUD example completed ===")


if __name__ == "__main__":
    asyncio.run(main())
