#!/usr/bin/env python3
"""Collection Management Examples - Administrative operations"""

import asyncio
import os
from pathlib import Path
import aiohttp
from dotenv import load_dotenv

# Load environment variables
env_path = Path(__file__).parent.parent.parent / ".env"
load_dotenv(env_path)

BASE_URL = os.getenv("API_BASE_URL", "http://localhost:8080")
API_KEY = os.getenv("API_BASE_KEY", "a-test-api-key-from-ekodb")


async def get_auth_token(session):
    """Get authentication token"""
    async with session.post(
        f"{BASE_URL}/api/auth/token",
        json={"api_key": API_KEY},
        headers={"Content-Type": "application/json"},
    ) as response:
        data = await response.json()
        return data["token"]


async def collection_management_examples():
    """Run collection management examples"""
    async with aiohttp.ClientSession() as session:
        # Get authentication token
        token = await get_auth_token(session)
        headers = {
            "Authorization": f"Bearer {token}",
            "Content-Type": "application/json",
        }
        print("✓ Authentication successful")

        try:
            # Example 1: Create a collection by inserting data
            print("\n=== Create Collection (via insert) ===")
            async with session.post(
                f"{BASE_URL}/api/insert/demo_collection",
                headers=headers,
                json={
                    "name": "Demo Record",
                    "description": "This creates the collection automatically",
                },
            ) as response:
                result = await response.json()
                print(f'Collection created with first record: {result["id"]}')

            # Example 2: List all collections
            print("\n=== List Collections ===")
            async with session.get(
                f"{BASE_URL}/api/collections",
                headers={"Authorization": f"Bearer {token}"},
            ) as response:
                collections = await response.json()
                collection_list = (
                    collections
                    if isinstance(collections, list)
                    else collections.get("collections", [])
                )
                print(f"Total collections: {len(collection_list)}")
                print(f"Sample collections: {collection_list[:5]}")

            # Example 3: Count documents in collection
            print("\n=== Count Documents ===")
            async with session.post(
                f"{BASE_URL}/api/find/demo_collection",
                headers=headers,
                json={"limit": 1000},
            ) as response:
                docs = await response.json()
                count = len(docs) if isinstance(docs, list) else "N/A"
                print(f"Document count: {count}")

            # Example 4: Delete collection
            print("\n=== Delete Collection ===")
            async with session.delete(
                f"{BASE_URL}/api/collections/demo_collection",
                headers={"Authorization": f"Bearer {token}"},
            ) as response:
                if response.ok:
                    print("Collection deleted successfully")
                else:
                    error = await response.text()
                    print(f"Delete response: {error}")

            # Example 5: Verify deletion
            print("\n=== Verify Deletion ===")
            async with session.get(
                f"{BASE_URL}/api/collections",
                headers={"Authorization": f"Bearer {token}"},
            ) as response:
                remaining = await response.json()
                remaining_list = (
                    remaining
                    if isinstance(remaining, list)
                    else remaining.get("collections", [])
                )
                still_exists = "demo_collection" in remaining_list
                print(f"Collection still exists: {still_exists}")

            print("\n✓ All collection management operations completed successfully")

        except Exception as e:
            print(f"Error: {e}")
            raise


if __name__ == "__main__":
    asyncio.run(collection_management_examples())
