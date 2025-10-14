#!/usr/bin/env python3
"""Simple WebSocket Example - Following YCSB benchmark patterns"""

import asyncio
import os
import json
from pathlib import Path
import aiohttp
import websockets
from dotenv import load_dotenv

# Load environment variables
env_path = Path(__file__).parent.parent.parent / ".env"
load_dotenv(env_path)

BASE_URL = os.getenv("API_BASE_URL", "http://localhost:8080")
WS_URL = os.getenv("WS_BASE_URL", "ws://localhost:8080")
API_KEY = os.getenv("API_BASE_KEY", "a-test-api-key-from-ekodb")


async def get_auth_token():
    """Get authentication token via HTTP"""
    async with aiohttp.ClientSession() as session:
        async with session.post(
            f"{BASE_URL}/api/auth/token",
            json={"api_key": API_KEY},
            headers={"Content-Type": "application/json"},
        ) as response:
            data = await response.json()
            return data["token"]


async def simple_websocket_example():
    """Run simple WebSocket example"""
    # Step 1: Get authentication token
    token = await get_auth_token()
    print("✓ Authentication successful")

    try:
        # Step 2: Insert some test data first
        print("\n=== Inserting Test Data ===")
        async with aiohttp.ClientSession() as session:
            async with session.post(
                f"{BASE_URL}/api/insert/websocket_test",
                headers={
                    "Authorization": f"Bearer {token}",
                    "Content-Type": "application/json",
                },
                json={"name": "WebSocket Test Record", "value": 42, "active": True},
            ) as response:
                result = await response.json()
                print(f'✓ Inserted test record: {result["id"]}')

        # Step 3: Connect to WebSocket with Authorization header
        print("\n=== Connecting to WebSocket ===")
        async with websockets.connect(
            f"{WS_URL}/api/ws", extra_headers={"Authorization": f"Bearer {token}"}
        ) as ws:
            print("✓ WebSocket connected")

            # Step 4: Query data via WebSocket
            print("\n=== Querying Data via WebSocket ===")
            message_id = str(asyncio.get_event_loop().time())

            await ws.send(
                json.dumps(
                    {
                        "type": "FindAll",
                        "messageId": message_id,
                        "payload": {"collection": "websocket_test"},
                    }
                )
            )

            # Wait for response
            response_data = await asyncio.wait_for(ws.recv(), timeout=5.0)
            response = json.loads(response_data)
            print(f"Response: {json.dumps(response, indent=2)}")

            # Show the data we got back
            if response.get("payload") and response["payload"].get("data"):
                print(
                    f'✓ Retrieved {len(response["payload"]["data"])} record(s) via WebSocket'
                )

            print("\n✓ WebSocket example completed successfully")

    except Exception as e:
        print(f"Error: {e}")
        raise


if __name__ == "__main__":
    asyncio.run(simple_websocket_example())
