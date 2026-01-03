#!/usr/bin/env python3
"""WebSocket with TTL Examples"""

import asyncio
import json
import os
import aiohttp
import websockets


async def websocket_ttl_examples():
    """Examples of using WebSocket with TTL features"""
    BASE_URL = os.getenv("API_BASE_URL", "http://localhost:8080")
    WS_URL = os.getenv("WS_BASE_URL", "ws://localhost:8080")
    API_KEY = os.getenv("API_BASE_KEY", "a-test-api-key-from-ekodb")

    # Get authentication token
    async with aiohttp.ClientSession() as session:
        async with session.post(
            f"{BASE_URL}/api/auth/token", json={"api_key": API_KEY}
        ) as response:
            token_data = await response.json()
            TOKEN = token_data["token"]

    print("✓ Authentication successful")

    try:
        async with aiohttp.ClientSession() as session:
            # Insert test data with TTL
            print("\n=== Insert Test Data with TTL ===")
            headers = {"Authorization": f"Bearer {TOKEN}"}
            async with session.post(
                f"{BASE_URL}/api/insert/ws_ttl_test",
                headers=headers,
                json={"name": "WebSocket TTL Test", "value": 42, "ttl": 3600},
            ) as response:
                doc = await response.json()
                print(f'✓ Inserted document with TTL: {doc.get("id")}')

        # Query via WebSocket
        print("\n=== Query via WebSocket ===")
        headers = {"Authorization": f"Bearer {TOKEN}"}
        async with websockets.connect(f"{WS_URL}/api/ws", extra_headers=headers) as ws:
            print("✓ WebSocket connected")

            # Send query
            message_id = str(asyncio.get_event_loop().time())
            await ws.send(
                json.dumps(
                    {
                        "type": "FindAll",
                        "messageId": message_id,
                        "payload": {"collection": "ws_ttl_test"},
                    }
                )
            )

            # Read response
            response = await asyncio.wait_for(ws.recv(), timeout=5.0)
            data = json.loads(response)

            if "payload" in data and "data" in data["payload"]:
                records = data["payload"]["data"]
                print(f"✓ Retrieved {len(records)} record(s) via WebSocket")

        print("\n✓ WebSocket TTL example completed successfully")

    except Exception as error:
        print(f"Error: {error}")
        exit(1)


if __name__ == "__main__":
    asyncio.run(websocket_ttl_examples())
