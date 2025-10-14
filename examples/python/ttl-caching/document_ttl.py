#!/usr/bin/env python3
"""Document TTL Examples"""

import asyncio
import os
import aiohttp


async def document_ttl_examples():
    """Examples of document-level TTL operations"""
    BASE_URL = os.getenv("API_BASE_URL", "http://localhost:8080")
    API_KEY = os.getenv("API_BASE_KEY", "a-test-api-key-from-ekodb")

    # Get authentication token
    async with aiohttp.ClientSession() as session:
        async with session.post(
            f"{BASE_URL}/api/auth/token", json={"api_key": API_KEY}
        ) as response:
            token_data = await response.json()
            TOKEN = token_data["token"]

    print("✓ Authentication successful")

    async with aiohttp.ClientSession() as session:
        headers = {
            "Authorization": f"Bearer {TOKEN}",
            "Content-Type": "application/json",
        }

        try:
            # Example 1: Insert document with TTL (1 hour)
            print("\n=== Insert Document with TTL (1 hour) ===")
            async with session.post(
                f"{BASE_URL}/api/insert/ttl_cache",
                headers=headers,
                json={
                    "key": "session_token",
                    "value": "abc123",
                    "ttl_duration": "1h",
                    "ttl_update_on_access": True,
                },
            ) as response:
                doc1 = await response.json()
                doc1_id = doc1.get("id")
                print(f"✓ Inserted document: {doc1_id}")

            # Example 2: Insert with shorter TTL (5 minutes)
            print("\n=== Insert Document with TTL (5 minutes) ===")
            async with session.post(
                f"{BASE_URL}/api/insert/ttl_cache",
                headers=headers,
                json={
                    "key": "temp_data",
                    "value": {"important": True},
                    "ttl_duration": "5m",
                },
            ) as response:
                doc2 = await response.json()
                doc2_id = doc2.get("id")
                print(f'✓ Inserted document: {doc2_id or "created"}')

            # Example 3: Query documents
            print("\n=== Query Documents ===")
            async with session.post(
                f"{BASE_URL}/api/find/ttl_cache", headers=headers, json={"limit": 10}
            ) as response:
                docs = await response.json()
                print(f"✓ Found {len(docs)} documents with TTL")

            # Example 4: Update document
            if doc1_id:
                print("\n=== Update Document ===")
                async with session.put(
                    f"{BASE_URL}/api/update/ttl_cache/{doc1_id}",
                    headers=headers,
                    json={"value": "updated_value"},
                ) as response:
                    await response.json()
                    print("✓ Updated document")

            # Example 5: Delete document
            if doc2_id:
                print("\n=== Delete Document ===")
                async with session.delete(
                    f"{BASE_URL}/api/delete/ttl_cache/{doc2_id}", headers=headers
                ) as response:
                    await response.json()
                    print("✓ Deleted document")
            else:
                print("\n=== Delete Document ===")
                print("✓ Skipped delete (no ID returned)")

            print("\n✓ All document TTL operations completed successfully")

        except Exception as error:
            print(f"Error: {error}")
            exit(1)


if __name__ == "__main__":
    asyncio.run(document_ttl_examples())
