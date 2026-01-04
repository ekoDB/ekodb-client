#!/usr/bin/env python3
"""
Collection Management using Direct HTTP Requests

Demonstrates collection creation, listing, and deletion
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
    print("=== Collection Management (Direct HTTP) ===\n")

    get_auth_token()
    print("✓ Authentication successful")

    # Example 1: Create a collection (via insert)
    print("\n=== Create Collection (via insert) ===")
    insert_result = request(
        "POST",
        "/api/insert/demo_collection",
        {
            "name": "Demo Collection",
            "description": "Created for testing",
            "active": True,
        },
    )
    print(f"Collection created with first record: {insert_result['id']}")

    # Example 2: List all collections
    print("\n=== List Collections ===")
    collections_data = request("GET", "/api/collections")
    collections = collections_data.get("collections", [])
    print(f"Total collections: {len(collections)}")
    print(f"Sample collections: {collections[:5]}")

    # Example 3: Count documents in collection
    print("\n=== Count Documents ===")
    docs = request("POST", "/api/find/demo_collection", {"limit": 1000})
    doc_count = len(docs) if isinstance(docs, list) else 0
    print(f"Document count: {doc_count}")

    # Example 4: Delete collection
    print("\n=== Delete Collection ===")
    request("DELETE", "/api/collections/demo_collection")
    print("Collection deleted successfully")

    # Example 5: Verify deletion
    print("\n=== Verify Deletion ===")
    updated_collections_data = request("GET", "/api/collections")
    updated_collections = updated_collections_data.get("collections", [])
    still_exists = "demo_collection" in updated_collections
    print(f"Collection still exists: {still_exists}")

    print("\n✓ All collection management operations completed successfully")


if __name__ == "__main__":
    main()
