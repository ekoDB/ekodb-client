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
SESSION_KEY = "kv_operations:direct:py:session:user123"
PRODUCT_KEYS = [
    "kv_operations:direct:py:cache:product:1",
    "kv_operations:direct:py:cache:product:2",
    "kv_operations:direct:py:cache:product:3",
]


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


def run_examples():
    # Example 1: Set a key-value pair
    print("\n=== KV Set ===")
    request(
        "POST",
        f"/api/kv/set/{SESSION_KEY}",
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
    get_value = request("GET", f"/api/kv/get/{SESSION_KEY}")
    print(f"Retrieved value: {get_value.get('value') if get_value else None}")

    # Example 3: Set multiple keys
    print("\n=== Set Multiple Keys ===")
    keys = PRODUCT_KEYS

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
    request("DELETE", f"/api/kv/delete/{SESSION_KEY}")
    print("✓ Deleted key: session:user123")

    # Verify deletion
    verify_delete = request("GET", f"/api/kv/get/{SESSION_KEY}")
    if verify_delete is None:
        print("✓ Verified: Key successfully deleted (not found)")
    else:
        print("✗ Warning: Key still exists after delete!")
        raise RuntimeError("session key still exists after deletion")

    # Example 6: Delete multiple keys
    print("\n=== Delete Multiple Keys ===")
    for key in keys:
        request("DELETE", f"/api/kv/delete/{key}")
    print(f"✓ Deleted {len(keys)} keys")


def cleanup_owned_keys():
    errors = []
    for key in [SESSION_KEY, *PRODUCT_KEYS]:
        try:
            request("DELETE", f"/api/kv/delete/{key}")
        except Exception as exc:
            errors.append(f"{key}: {exc}")
    if errors:
        raise RuntimeError("failed to clean owned KV keys: " + "; ".join(errors))


def main():
    print("=== Key-Value Operations (Direct HTTP) ===\n")

    get_auth_token()
    print("✓ Authentication successful")
    cleanup_owned_keys()

    operation_error = None
    try:
        run_examples()
    except Exception as exc:
        operation_error = exc

    cleanup_error = None
    try:
        cleanup_owned_keys()
    except Exception as exc:
        cleanup_error = exc

    if operation_error is not None:
        if cleanup_error is not None:
            operation_error.add_note(f"Cleanup also failed: {cleanup_error}")
        raise operation_error
    if cleanup_error is not None:
        raise cleanup_error

    print("\n✓ All KV operations completed successfully")


if __name__ == "__main__":
    main()
