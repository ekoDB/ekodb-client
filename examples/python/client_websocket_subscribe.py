#!/usr/bin/env python3
"""
WebSocket Subscription Example - Real-time mutation notifications

Demonstrates subscribing to collection changes via WebSocket and
receiving push notifications when records are inserted, updated, or deleted.

Requires: ekoDB server running on localhost:8080
"""

import asyncio
import json
import os

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


def delete_collection(token, collection):
    response = requests.delete(
        f"{BASE_URL}/api/collections/{collection}",
        headers={"Authorization": f"Bearer {token}"},
    )
    if response.status_code == 404:
        return False
    response.raise_for_status()
    return True


def required_id(result, operation):
    record_id = result.get("id")
    if not isinstance(record_id, str) or not record_id:
        raise AssertionError(f"{operation} did not return a non-empty record ID")
    return record_id


def validate_notification(notification, collection, record_id):
    if notification.get("type") != "MutationNotification":
        raise AssertionError(f"unexpected WebSocket message: {notification!r}")
    payload = notification.get("payload")
    if not isinstance(payload, dict):
        raise TypeError("mutation notification did not include a payload")
    if payload.get("event") != "insert":
        raise AssertionError(
            f"expected insert notification, got {payload.get('event')!r}"
        )
    if payload.get("collection") != collection:
        raise AssertionError(
            f"expected notification for {collection!r}, got {payload.get('collection')!r}"
        )
    if record_id not in payload.get("record_ids", []):
        raise AssertionError(f"notification did not include inserted ID {record_id!r}")
    if not payload.get("timestamp"):
        raise AssertionError("mutation notification did not include a timestamp")
    return payload


async def main():
    print("=== WebSocket Subscription Example (Python) ===\n")

    collection = "ws_subscribe_example_py"

    # Step 1: Authenticate
    token = get_auth_token()
    print("✓ Authentication successful")
    delete_collection(token, collection)
    operation_error = None

    try:
        # Step 2: Connect to WebSocket
        headers = {"Authorization": f"Bearer {token}"}

        async with websockets.connect(
            f"{WS_URL}/api/ws", additional_headers=headers
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

            response = json.loads(
                await asyncio.wait_for(websocket.recv(), timeout=10.0)
            )
            data = response.get("payload", {}).get("data", {})
            sub_id = data.get("subscription_id")
            if not isinstance(sub_id, str) or not sub_id:
                raise AssertionError(f"invalid subscription response: {response!r}")
            print(f"✓ Subscribed (subscription_id: {sub_id})")

            # Step 4: Insert records via REST API to trigger notifications
            print("\n=== Performing mutations to trigger notifications ===")

            print("Inserting record 1...")
            result1 = insert_record(
                token,
                collection,
                {"name": "Alice", "role": "engineer", "active": True},
            )
            record1_id = required_id(result1, "first insert")
            print(f"✓ Inserted: {record1_id}")

            # Wait for notification. A timeout is a test failure, not an optional
            # condition: receiving this push is the behavior being demonstrated.
            try:
                notification = json.loads(
                    await asyncio.wait_for(websocket.recv(), timeout=5.0)
                )
            except asyncio.TimeoutError:
                print("  ⏳ No notification within timeout")
                raise
            payload = validate_notification(notification, collection, record1_id)
            print("\n  📡 Notification received:")
            print(f"     Event:      {payload['event']}")
            print(f"     Collection: {payload['collection']}")
            print(f"     Record IDs: {', '.join(payload['record_ids'])}")
            print(f"     Timestamp:  {payload['timestamp']}")

            print("\nInserting record 2...")
            result2 = insert_record(
                token,
                collection,
                {"name": "Bob", "role": "designer", "active": True},
            )
            record2_id = required_id(result2, "second insert")
            print(f"✓ Inserted: {record2_id}")

            try:
                notification = json.loads(
                    await asyncio.wait_for(websocket.recv(), timeout=5.0)
                )
            except asyncio.TimeoutError:
                print("  ⏳ No notification within timeout")
                raise
            payload = validate_notification(notification, collection, record2_id)
            print("\n  📡 Notification received:")
            print(f"     Event:      {payload['event']}")
            print(f"     Record IDs: {', '.join(payload['record_ids'])}")

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
            unsub_data = unsub_response.get("payload", {}).get("data", {})
            if (
                unsub_data.get("collection") != collection
                or unsub_data.get("found") is not True
                or unsub_data.get("unsubscribed") is not True
            ):
                raise AssertionError(
                    f"invalid unsubscribe response: {unsub_response!r}"
                )
            print(f"✓ Unsubscribed: {unsub_data}")
    except Exception as exc:  # noqa: BLE001 - preserve the primary example failure
        print(f"WebSocket subscription error: {exc}")
        operation_error = exc

    print("\n=== Cleanup ===")
    try:
        if delete_collection(token, collection):
            print(f"✓ Deleted collection '{collection}'")
    except Exception as cleanup_error:
        print(f"Cleanup error: {cleanup_error}")
        if operation_error is not None:
            operation_error.add_note(f"cleanup also failed: {cleanup_error}")
        else:
            raise

    if operation_error is not None:
        raise operation_error

    print("\n✓ WebSocket subscription example completed successfully")


if __name__ == "__main__":
    asyncio.run(main())
