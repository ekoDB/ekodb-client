"""ekoDB Python Client - KV Links Integration Example

Demonstrates linking and unlinking documents to KV keys:
  kv_set -> kv_link -> kv_get_links -> kv_unlink
"""

import asyncio
import os

from ekodb_client import Client

COLLECTION = "kv_links_example_py"
KV_KEY = "kv_links:py:app:config"


async def cleanup(client, verbose=True):
    errors = []
    try:
        await client.kv_delete(KV_KEY)
    except Exception as error:
        if "404" not in str(error) and "not found" not in str(error).lower():
            errors.append(f"KV key {KV_KEY}: {error}")
    try:
        await client.delete_collection(COLLECTION)
    except Exception as error:
        if "404" not in str(error) and "not found" not in str(error).lower():
            errors.append(f"collection {COLLECTION}: {error}")
    if errors:
        raise RuntimeError("cleanup failed: " + "; ".join(errors))
    if verbose:
        print("Cleaned up KV key and collection")


async def main():
    print("=== ekoDB KV Links Example (Python) ===\n")

    base_url = os.getenv("API_BASE_URL", "http://localhost:8080")
    api_key = os.getenv("API_BASE_KEY")
    if not api_key:
        raise ValueError("API_BASE_KEY environment variable is required")

    client = Client.new(base_url, api_key)

    await cleanup(client, verbose=False)

    operation_error = None
    try:
        # 1. Set a KV key to link documents to
        print("--- kv_set ---")
        await client.kv_set(KV_KEY, {"version": "3.0", "env": "staging"})
        print("Set KV key: app:config")

        # 2. Insert a document to link
        print("\n--- insert document ---")
        doc = await client.insert(
            COLLECTION, {"name": "Settings Override", "priority": 1}
        )
        doc_id = doc["id"]
        print(f"Inserted document: {doc_id}")

        # 3. Link the document to the KV key
        print("\n--- kv_link ---")
        link_result = await client.kv_link(KV_KEY, COLLECTION, doc_id)
        print(f"Linked: {link_result}")

        # 4. Get all links for the KV key
        print("\n--- kv_get_links ---")
        links = await client.kv_get_links(KV_KEY)
        print(f"Links for app:config: {links}")

        # 5. Unlink the document
        print("\n--- kv_unlink ---")
        unlink_result = await client.kv_unlink(KV_KEY, COLLECTION, doc_id)
        print(f"Unlinked: {unlink_result}")

        # 6. Verify links are empty
        print("\n--- kv_get_links (verify empty) ---")
        links_after = await client.kv_get_links(KV_KEY)
        print(f"Links after unlink: {links_after}")
    except Exception as error:
        operation_error = error

    print("\n--- cleanup ---")
    cleanup_error = None
    try:
        await cleanup(client)
    except Exception as error:
        cleanup_error = error

    if operation_error is not None:
        if cleanup_error is not None:
            operation_error.add_note(f"Cleanup also failed: {cleanup_error}")
        raise operation_error
    if cleanup_error is not None:
        raise cleanup_error

    print("\n=== KV links example completed ===")


if __name__ == "__main__":
    asyncio.run(main())
