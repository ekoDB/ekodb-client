#!/usr/bin/env python3
"""Simple CRUD Examples - Following benchmark patterns"""

import asyncio
import os
import json
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


async def simple_crud_examples():
    """Run simple CRUD examples"""
    async with aiohttp.ClientSession() as session:
        # Get authentication token
        token = await get_auth_token(session)
        headers = {
            "Authorization": f"Bearer {token}",
            "Content-Type": "application/json",
        }
        print("✓ Authentication successful")

        try:
            # Example 1: Insert a document
            print("\n=== Insert Document ===")
            async with session.post(
                f"{BASE_URL}/api/insert/test_collection",
                headers=headers,
                json={"name": "Test Record", "value": 42, "active": True},
            ) as response:
                doc = await response.json()
                print(f"Inserted: {doc}")
                doc_id = doc["id"]

            # Example 2: Find by ID
            print("\n=== Find by ID ===")
            async with session.get(
                f"{BASE_URL}/api/find/test_collection/{doc_id}",
                headers={"Authorization": f"Bearer {token}"},
            ) as response:
                found_doc = await response.json()
                print(f"Found: {found_doc}")

            # Example 3: Find with query
            print("\n=== Find with Query ===")
            async with session.post(
                f"{BASE_URL}/api/find/test_collection",
                headers=headers,
                json={
                    "filter": {
                        "type": "Condition",
                        "content": {"field": "active", "operator": "Eq", "value": True},
                    },
                    "limit": 10,
                },
            ) as response:
                docs = await response.json()
                print(
                    f'Found documents: {len(docs) if isinstance(docs, list) else "N/A"}'
                )

            # Example 4: Update document
            print("\n=== Update Document ===")
            async with session.put(
                f"{BASE_URL}/api/update/test_collection/{doc_id}",
                headers=headers,
                json={"name": "Updated Record", "value": 100},
            ) as response:
                updated = await response.json()
                print(f"Updated: {updated}")

            # Example 5: Delete document
            print("\n=== Delete Document ===")
            async with session.delete(
                f"{BASE_URL}/api/delete/test_collection/{doc_id}",
                headers={"Authorization": f"Bearer {token}"},
            ) as response:
                print("Deleted document")

            print("\n✓ All CRUD operations completed successfully")

        except Exception as e:
            print(f"Error: {e}")
            raise


if __name__ == "__main__":
    asyncio.run(simple_crud_examples())
