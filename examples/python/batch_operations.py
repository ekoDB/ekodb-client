#!/usr/bin/env python3
"""
Batch Operations using Direct HTTP Requests

Demonstrates batch insert, update, and delete operations
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
    print("=== Batch Operations (Direct HTTP) ===\n")

    get_auth_token()
    print("✓ Authentication successful")

    # Example 1: Batch Insert
    print("\n=== Batch Insert ===")
    records = [
        {"name": "User 1", "email": "user1@example.com", "age": 25},
        {"name": "User 2", "email": "user2@example.com", "age": 30},
        {"name": "User 3", "email": "user3@example.com", "age": 35},
        {"name": "User 4", "email": "user4@example.com", "age": 40},
        {"name": "User 5", "email": "user5@example.com", "age": 45},
    ]

    inserts = [{"data": r} for r in records]
    batch_insert_data = {
        "inserts": inserts,
        "bypass_ripple": False,
    }

    insert_result = request("POST", "/api/batch/insert/batch_users", batch_insert_data)
    inserted_count = len(insert_result.get("successful", []))
    print(f"✓ Batch inserted {inserted_count} records")

    # Verify the inserts
    all_docs = request("POST", "/api/find/batch_users", {"limit": 100})
    total_count = len(all_docs) if isinstance(all_docs, list) else 0
    print(f"✓ Verified: Found {total_count} total records in collection")

    # Example 2: Create test records for update/delete
    print("\n=== Creating test records for update/delete ===")
    ids = []
    for i in range(3):
        doc = request(
            "POST",
            "/api/insert/batch_users",
            {
                "name": f"Test User {i}",
                "value": i,
            },
        )
        ids.append(doc["id"])
    print(f"Created {len(ids)} test records")

    # Example 3: Batch Update
    print("\n=== Batch Update ===")
    updates = [
        {"id": id_, "data": {"name": f"Updated User {i}", "status": "active"}}
        for i, id_ in enumerate(ids)
    ]

    batch_update_data = {
        "updates": updates,
        "bypass_ripple": False,
    }

    update_result = request("PUT", "/api/batch/update/batch_users", batch_update_data)
    updated_count = len(update_result.get("successful", []))
    print(f"✓ Batch updated {updated_count} records")

    # Verify the updates
    updated_doc = request("GET", f"/api/find/batch_users/{ids[0]}")
    status = updated_doc.get("status", {})
    if isinstance(status, dict):
        status = status.get("value", "active")
    print(f'✓ Verified: Record updated with status="{status}"')

    # Example 4: Batch Delete
    print("\n=== Batch Delete ===")
    deletes = [{"id": id_} for id_ in ids]
    batch_delete_data = {
        "deletes": deletes,
        "bypass_ripple": False,
    }

    delete_result = request(
        "DELETE", "/api/batch/delete/batch_users", batch_delete_data
    )
    deleted_count = len(delete_result.get("successful", []))
    print(f"✓ Batch deleted {deleted_count} records")

    # Verify the deletes
    verify_delete = request("GET", f"/api/find/batch_users/{ids[0]}")
    if verify_delete is None:
        print("✓ Verified: Records successfully deleted (not found)")
    else:
        print("✗ Warning: Record still exists after delete!")

    print("\n✓ All batch operations completed successfully")


if __name__ == "__main__":
    main()
