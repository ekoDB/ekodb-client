#!/usr/bin/env python3
"""Key-Value Store Operations Examples - Fast caching and session management"""

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


async def kv_operations_examples():
    """Run KV operations examples"""
    async with aiohttp.ClientSession() as session:
        # Get authentication token
        token = await get_auth_token(session)
        headers = {
            "Authorization": f"Bearer {token}",
            "Content-Type": "application/json",
        }
        print("✓ Authentication successful")

        try:
            # Example 1: Set a key-value pair
            print("\n=== KV Set ===")
            async with session.post(
                f"{BASE_URL}/api/kv/set/session:user123",
                headers=headers,
                json={"value": {"userId": 123, "username": "john_doe"}},
            ) as response:
                await response.json()
                print("✓ Set key: session:user123")

            # Example 2: Get a value
            print("\n=== KV Get ===")
            async with session.get(
                f"{BASE_URL}/api/kv/get/session:user123",
                headers={"Authorization": f"Bearer {token}"},
            ) as response:
                result = await response.json()
                print(f'Retrieved value: {result["value"]}')

            # Example 3: Set multiple keys
            print("\n=== Set Multiple Keys ===")
            keys = ["cache:product:1", "cache:product:2", "cache:product:3"]
            for i, key in enumerate(keys):
                async with session.post(
                    f"{BASE_URL}/api/kv/set/{key}",
                    headers=headers,
                    json={
                        "value": {"name": f"Product {i+1}", "price": 29.99 + (i * 10)}
                    },
                ) as response:
                    await response.json()
            print(f"✓ Set {len(keys)} keys")

            # Example 4: Get multiple keys
            print("\n=== Get Multiple Keys ===")
            for key in keys:
                async with session.get(
                    f"{BASE_URL}/api/kv/get/{key}",
                    headers={"Authorization": f"Bearer {token}"},
                ) as response:
                    data = await response.json()
                    print(f'{key}: {data["value"]}')

            # Example 5: Delete a key
            print("\n=== KV Delete ===")
            async with session.delete(
                f"{BASE_URL}/api/kv/delete/session:user123",
                headers={"Authorization": f"Bearer {token}"},
            ) as response:
                if response.ok:
                    print("✓ Deleted key: session:user123")

            # Verify deletion
            async with session.get(
                f"{BASE_URL}/api/kv/get/session:user123",
                headers={"Authorization": f"Bearer {token}"},
            ) as verify_resp:
                if verify_resp.status == 404:
                    print(f"✓ Verified: Key successfully deleted (not found)")
                else:
                    print(f"✗ Warning: Key still exists after delete!")

            # Example 6: Delete multiple keys
            print("\n=== Delete Multiple Keys ===")
            for key in keys:
                async with session.delete(
                    f"{BASE_URL}/api/kv/delete/{key}",
                    headers={"Authorization": f"Bearer {token}"},
                ) as response:
                    await response.json()
            print(f"✓ Deleted {len(keys)} keys")

            print("\n✓ All KV operations completed successfully")

        except Exception as e:
            print(f"Error: {e}")
            raise


if __name__ == "__main__":
    asyncio.run(kv_operations_examples())
