#!/usr/bin/env python3
"""
Key-Value Operations using Direct HTTP Requests

Demonstrates KV set, get, and delete operations
using raw HTTP requests - no client library required
"""

import os
import requests
from dotenv import load_dotenv

load_dotenv()

BASE_URL = os.getenv("API_BASE_URL", "http://localhost:8080")
API_KEY = os.getenv("API_BASE_KEY", "a-test-api-key-from-ekodb")

auth_token = None


def get_auth_token():
    global auth_token
    if auth_token:
        return auth_token

    response = requests.post(
        f"{BASE_URL}/api/auth/token",
        json={"api_key": API_KEY},
        headers={"Content-Type": "application/json"},
    )
    response.raise_for_status()
    auth_token = response.json()["token"]
    return auth_token


def request(method, path, body=None):
    token = get_auth_token()
    headers = {
        "Authorization": f"Bearer {token}",
        "Content-Type": "application/json",
    }

    response = requests.request(method, f"{BASE_URL}{path}", json=body, headers=headers)

    if response.status_code == 404:
        return None

    if not response.ok:
        raise Exception(f"HTTP {response.status_code}: {response.text}")

    if not response.text:
        return {}

    return response.json()


def main():
    print("=== Key-Value Operations (Direct HTTP) ===\n")

    get_auth_token()
    print("✓ Authentication successful")

    # Example 1: Set a key-value pair
    print("\n=== KV Set ===")
    request(
        "POST",
        "/api/kv/set/session:user123",
        {
            "value": {
                "userId": 123,
                "username": "john_doe",
            },
        },
    )
    print("✓ Set key: session:user123")

    # Example 2: Get a key-value pair
    print("\n=== KV Get ===")
    get_value = request("GET", "/api/kv/get/session:user123")
    print(f"Retrieved value: {get_value.get('value') if get_value else None}")

    # Example 3: Set multiple keys
    print("\n=== Set Multiple Keys ===")
    keys = ["cache:product:1", "cache:product:2", "cache:product:3"]

    for i, key in enumerate(keys):
        request(
            "POST",
            f"/api/kv/set/{key}",
            {
                "value": {
                    "name": f"Product {i + 1}",
                    "price": 29.99 + i * 10.0,
                },
            },
        )
    print(f"✓ Set {len(keys)} keys")

    # Example 4: Get multiple keys
    print("\n=== Get Multiple Keys ===")
    for key in keys:
        result = request("GET", f"/api/kv/get/{key}")
        print(f"{key}: {result.get('value') if result else None}")

    # Example 5: Delete a key
    print("\n=== KV Delete ===")
    request("DELETE", "/api/kv/delete/session:user123")
    print("✓ Deleted key: session:user123")

    # Verify deletion
    verify_delete = request("GET", "/api/kv/get/session:user123")
    if verify_delete is None:
        print("✓ Verified: Key successfully deleted (not found)")
    else:
        print("✗ Warning: Key still exists after delete!")

    # Example 6: Delete multiple keys
    print("\n=== Delete Multiple Keys ===")
    for key in keys:
        request("DELETE", f"/api/kv/delete/{key}")
    print(f"✓ Deleted {len(keys)} keys")

    print("\n✓ All KV operations completed successfully")


if __name__ == "__main__":
    main()
