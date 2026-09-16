#!/usr/bin/env python3
"""
Simple WebSocket Operations using Direct API

Demonstrates WebSocket connection and querying
using raw WebSocket API - no client library required
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
COLLECTION = "simple_websocket_example_py"


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
        f"{BASE_URL}/api/insert/{COLLECTION}",
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


def delete_test_collection(token):
    response = requests.delete(
        f"{BASE_URL}/api/collections/{COLLECTION}",
        headers={"Authorization": f"Bearer {token}"},
    )
    if response.status_code != 404:
        response.raise_for_status()


async def run_example(token):
    # Step 2: Insert test data first
    print("\n=== Inserting Test Data ===")
    insert_result = insert_test_data(token)
    print(f"✓ Inserted test record: {insert_result['id']}")

    # Step 3: Connect to WebSocket
    print("\n=== Connecting to WebSocket ===")

    headers = {"Authorization": f"Bearer {token}"}

    async with websockets.connect(
        f"{WS_URL}/api/ws", additional_headers=headers
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
                        "collection": COLLECTION,
                    },
                }
            )
        )

        # Wait for response
        try:
            response_text = await asyncio.wait_for(websocket.recv(), timeout=10.0)
            response = json.loads(response_text)
            print(f"Response: {json.dumps(response, indent=2)}")

            records = response.get("payload", {}).get("data")
            if not isinstance(records, list):
                raise RuntimeError("WebSocket response did not contain payload.data")
            if len(records) != 1:
                raise RuntimeError(
                    f"Expected exactly 1 WebSocket record, got {len(records)}"
                )
            print("✓ Retrieved 1 record via WebSocket")

        except asyncio.TimeoutError:
            raise RuntimeError("WebSocket timeout")


async def main():
    print("=== Simple WebSocket Operations (Direct API) ===\n")

    # Step 1: Get authentication token
    token = get_auth_token()
    print("✓ Authentication successful")
    delete_test_collection(token)

    operation_error = None
    try:
        await run_example(token)
    except Exception as error:
        operation_error = error

    cleanup_error = None
    try:
        delete_test_collection(token)
    except Exception as error:
        cleanup_error = error

    if operation_error is not None:
        if cleanup_error is not None:
            operation_error.add_note(f"Cleanup also failed: {cleanup_error}")
        raise operation_error
    if cleanup_error is not None:
        raise cleanup_error

    print("\n✓ WebSocket example completed successfully")


if __name__ == "__main__":
    asyncio.run(main())
