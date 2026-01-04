#!/usr/bin/env python3
"""
Simple WebSocket Operations using Direct API

Demonstrates WebSocket connection and querying
using raw WebSocket API - no client library required
"""

import os
import json
import asyncio
import requests
import websockets
from dotenv import load_dotenv

load_dotenv()

BASE_URL = os.getenv("API_BASE_URL", "http://localhost:8080")
WS_URL = os.getenv("WS_BASE_URL", "ws://localhost:8080")
API_KEY = os.getenv("API_BASE_KEY", "a-test-api-key-from-ekodb")


def get_auth_token():
    response = requests.post(
        f"{BASE_URL}/api/auth/token",
        json={"api_key": API_KEY},
        headers={"Content-Type": "application/json"},
    )
    response.raise_for_status()
    return response.json()["token"]


def insert_test_data(token):
    response = requests.post(
        f"{BASE_URL}/api/insert/websocket_test",
        json={
            "name": "WebSocket Test Record",
            "value": 42,
            "active": True,
        },
        headers={
            "Authorization": f"Bearer {token}",
            "Content-Type": "application/json",
        },
    )
    response.raise_for_status()
    return response.json()


async def main():
    print("=== Simple WebSocket Operations (Direct API) ===\n")

    # Step 1: Get authentication token
    token = get_auth_token()
    print("✓ Authentication successful")

    # Step 2: Insert test data first
    print("\n=== Inserting Test Data ===")
    insert_result = insert_test_data(token)
    print(f"✓ Inserted test record: {insert_result['id']}")

    # Step 3: Connect to WebSocket
    print("\n=== Connecting to WebSocket ===")

    headers = {"Authorization": f"Bearer {token}"}

    async with websockets.connect(
        f"{WS_URL}/api/ws", extra_headers=headers
    ) as websocket:
        print("✓ WebSocket connected")

        # Step 4: Query data via WebSocket
        print("\n=== Querying Data via WebSocket ===")
        message_id = str(int(asyncio.get_event_loop().time() * 1000))

        await websocket.send(
            json.dumps(
                {
                    "type": "FindAll",
                    "messageId": message_id,
                    "payload": {
                        "collection": "websocket_test",
                    },
                }
            )
        )

        # Wait for response
        try:
            response_text = await asyncio.wait_for(websocket.recv(), timeout=10.0)
            response = json.loads(response_text)
            print(f"Response: {json.dumps(response, indent=2)}")

            if response.get("payload") and response["payload"].get("data"):
                records = response["payload"]["data"]
                print(f"✓ Retrieved {len(records)} record(s) via WebSocket")

            print("\n✓ WebSocket example completed successfully")
        except asyncio.TimeoutError:
            print("✗ WebSocket timeout")


if __name__ == "__main__":
    asyncio.run(main())
