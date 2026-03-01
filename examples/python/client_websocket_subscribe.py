#!/usr/bin/env python3
"""
WebSocket Subscription Example - Real-time mutation notifications

Demonstrates subscribing to collection changes via WebSocket and
receiving push notifications when records are inserted, updated, or deleted.

Requires: ekoDB server running on localhost:8080
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


def insert_record(token, collection, record):
    response = requests.post(
        f"{BASE_URL}/api/insert/{collection}",
        json=record,
        headers={
            "Authorization": f"Bearer {token}",
            "Content-Type": "application/json",
        },
    )
    response.raise_for_status()
    return response.json()


async def main():
    print("=== WebSocket Subscription Example (Python) ===\n")

    collection = "ws_subscribe_example_py"

    # Step 1: Authenticate
    token = get_auth_token()
    print("✓ Authentication successful")

    # Step 2: Connect to WebSocket
    headers = {"Authorization": f"Bearer {token}"}

    async with websockets.connect(
        f"{WS_URL}/api/ws", extra_headers=headers
    ) as websocket:
        print("✓ WebSocket connected")

        # Step 3: Subscribe to collection
        print(f"\n=== Subscribing to '{collection}' ===")
        await websocket.send(
            json.dumps(
                {
                    "type": "Subscribe",
                    "payload": {"collection": collection},
                }
            )
        )

        response = json.loads(await asyncio.wait_for(websocket.recv(), timeout=10.0))
        sub_id = response["payload"]["data"]["subscription_id"]
        print(f"✓ Subscribed (subscription_id: {sub_id})")

        # Step 4: Insert records via REST API to trigger notifications
        print("\n=== Performing mutations to trigger notifications ===")

        print("Inserting record 1...")
        result1 = insert_record(
            token,
            collection,
            {"name": "Alice", "role": "engineer", "active": True},
        )
        print(f"✓ Inserted: {result1['id']}")

        # Wait for notification
        try:
            notification = json.loads(
                await asyncio.wait_for(websocket.recv(), timeout=5.0)
            )
            if notification.get("type") == "MutationNotification":
                payload = notification["payload"]
                print(f"\n  📡 Notification received:")
                print(f"     Event:      {payload['event']}")
                print(f"     Collection: {payload['collection']}")
                print(f"     Record IDs: {', '.join(payload['record_ids'])}")
                print(f"     Timestamp:  {payload['timestamp']}")
        except asyncio.TimeoutError:
            print("  ⏳ No notification within timeout")

        print("\nInserting record 2...")
        result2 = insert_record(
            token,
            collection,
            {"name": "Bob", "role": "designer", "active": True},
        )
        print(f"✓ Inserted: {result2['id']}")

        try:
            notification = json.loads(
                await asyncio.wait_for(websocket.recv(), timeout=5.0)
            )
            if notification.get("type") == "MutationNotification":
                payload = notification["payload"]
                print(f"\n  📡 Notification received:")
                print(f"     Event:      {payload['event']}")
                print(f"     Record IDs: {', '.join(payload['record_ids'])}")
        except asyncio.TimeoutError:
            print("  ⏳ No notification within timeout")

        # Step 5: Unsubscribe
        print("\n=== Unsubscribing ===")
        await websocket.send(
            json.dumps(
                {
                    "type": "Unsubscribe",
                    "payload": {"collection": collection},
                }
            )
        )

        unsub_response = json.loads(
            await asyncio.wait_for(websocket.recv(), timeout=10.0)
        )
        print(f"✓ Unsubscribed: {unsub_response['payload']['data']}")

    print("\n✓ WebSocket subscription example completed successfully")


if __name__ == "__main__":
    asyncio.run(main())
