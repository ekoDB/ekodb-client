#!/usr/bin/env python3
"""Batch Operations Examples - High-performance bulk operations"""

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


async def batch_operations_examples():
    """Run batch operations examples"""
    async with aiohttp.ClientSession() as session:
        # Get authentication token
        token = await get_auth_token(session)
        headers = {
            "Authorization": f"Bearer {token}",
            "Content-Type": "application/json",
        }
        print("✓ Authentication successful")

        try:
            # Example 1: Batch Insert
            print("\n=== Batch Insert ===")
            records = [
                {"name": "User 1", "email": "user1@example.com", "age": 25},
                {"name": "User 2", "email": "user2@example.com", "age": 30},
                {"name": "User 3", "email": "user3@example.com", "age": 35},
                {"name": "User 4", "email": "user4@example.com", "age": 40},
                {"name": "User 5", "email": "user5@example.com", "age": 45},
            ]

            batch_insert_data = {
                "inserts": [{"data": data} for data in records],
                "bypass_ripple": False,
            }

            async with session.post(
                f"{BASE_URL}/api/batch/insert/batch_users",
                headers=headers,
                json=batch_insert_data,
            ) as response:
                result = await response.json()
                inserted_ids = result.get("successful", [])
                print(f"✓ Batch inserted {len(inserted_ids)} records")

                # Verify the inserts
                async with session.post(
                    f"{BASE_URL}/api/find/batch_users",
                    headers=headers,
                    json={"limit": 100},
                ) as verify_resp:
                    all_docs = await verify_resp.json()
                    print(
                        f"✓ Verified: Found {len(all_docs)} total records in collection"
                    )

            # Example 2: Create test records for update/delete
            print("\n=== Creating test records for update/delete ===")
            ids = []
            for i in range(3):
                async with session.post(
                    f"{BASE_URL}/api/insert/batch_users",
                    headers=headers,
                    json={"name": f"Test User {i}", "value": i},
                ) as response:
                    doc = await response.json()
                    ids.append(doc["id"])
            print(f"Created {len(ids)} test records")

            # Example 3: Batch Update
            print("\n=== Batch Update ===")
            batch_update_data = {
                "updates": [
                    {
                        "id": ids[i],
                        "data": {"name": f"Updated User {i}", "status": "active"},
                    }
                    for i in range(len(ids))
                ],
                "bypass_ripple": False,
            }

            async with session.put(
                f"{BASE_URL}/api/batch/update/batch_users",
                headers=headers,
                json=batch_update_data,
            ) as response:
                result = await response.json()
                print(f'✓ Batch updated {len(result.get("successful", []))} records')

                # Verify the updates
                async with session.get(
                    f"{BASE_URL}/api/find/batch_users/{ids[0]}",
                    headers={"Authorization": f"Bearer {token}"},
                ) as verify_resp:
                    updated_doc = await verify_resp.json()
                    print(
                        f'✓ Verified: Record updated with status="{updated_doc.get("status")}"'
                    )

            # Example 4: Batch Delete
            print("\n=== Batch Delete ===")
            async with session.delete(
                f"{BASE_URL}/api/batch/delete/batch_users",
                headers=headers,
                json={
                    "deletes": [{"id": id} for id in ids],
                    "bypass_ripple": False,
                },
            ) as response:
                result = await response.json()
                print(f'✓ Batch deleted {len(result.get("successful", []))} records')

                # Verify the deletes
                async with session.get(
                    f"{BASE_URL}/api/find/batch_users/{ids[0]}",
                    headers={"Authorization": f"Bearer {token}"},
                ) as verify_resp:
                    if verify_resp.status == 404:
                        print(f"✓ Verified: Records successfully deleted (not found)")
                    else:
                        print(f"✗ Warning: Record still exists after delete!")

            print("\n✓ All batch operations completed successfully")

        except Exception as e:
            print(f"Error: {e}")
            raise


if __name__ == "__main__":
    asyncio.run(batch_operations_examples())
