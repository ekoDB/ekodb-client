"""ekoDB Python Client - KV Links Integration Example

Demonstrates linking and unlinking documents to KV keys:
  kv_set -> kv_link -> kv_get_links -> kv_unlink
"""

import asyncio
import os

from ekodb_client import Client


async def main():
    print("=== ekoDB KV Links Example (Python) ===\n")

    base_url = os.getenv("API_BASE_URL", "http://localhost:8080")
    api_key = os.getenv("API_BASE_KEY")
    if not api_key:
        raise ValueError("API_BASE_KEY environment variable is required")

    client = Client.new(base_url, api_key)

    collection = "kv_links_example_py"

    try:
        # 1. Set a KV key to link documents to
        print("--- kv_set ---")
        await client.kv_set("app:config", {"version": "3.0", "env": "staging"})
        print("Set KV key: app:config")

        # 2. Insert a document to link
        print("\n--- insert document ---")
        doc = await client.insert(
            collection, {"name": "Settings Override", "priority": 1}
        )
        doc_id = doc["id"]
        print(f"Inserted document: {doc_id}")

        # 3. Link the document to the KV key
        print("\n--- kv_link ---")
        link_result = await client.kv_link("app:config", collection, doc_id)
        print(f"Linked: {link_result}")

        # 4. Get all links for the KV key
        print("\n--- kv_get_links ---")
        links = await client.kv_get_links("app:config")
        print(f"Links for app:config: {links}")

        # 5. Unlink the document
        print("\n--- kv_unlink ---")
        unlink_result = await client.kv_unlink("app:config", collection, doc_id)
        print(f"Unlinked: {unlink_result}")

        # 6. Verify links are empty
        print("\n--- kv_get_links (verify empty) ---")
        links_after = await client.kv_get_links("app:config")
        print(f"Links after unlink: {links_after}")

    finally:
        # Cleanup
        print("\n--- cleanup ---")
        await client.kv_delete("app:config")
        await client.delete_collection(collection)
        print("Cleaned up KV key and collection")

    print("\n=== KV links example completed ===")


if __name__ == "__main__":
    asyncio.run(main())
