#!/usr/bin/env python3
"""
Simple CRUD Operations using Direct HTTP Requests

Demonstrates basic Create, Read, Update, Delete operations
using raw HTTP requests - no client library required
"""

import os

import requests
from dotenv import load_dotenv

load_dotenv()

BASE_URL = os.getenv("API_BASE_URL", "http://localhost:8080")
API_KEY = os.getenv("API_BASE_KEY", "a-test-api-key-from-ekodb")

auth_token = None
COLLECTION = "simple_crud_example_py"


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
    print("=== Simple CRUD Operations (Direct HTTP) ===\n")

    get_auth_token()
    print("✓ Authentication successful")

    # Example 1: Insert a document
    print("\n=== Insert Document ===")
    doc = request(
        "POST",
        f"/api/insert/{COLLECTION}",
        {
            "name": "Test Record",
            "value": 42,
            "active": True,
        },
    )
    print(f"Inserted: {doc}")
    doc_id = doc["id"]

    # Example 2: Find by ID
    print("\n=== Find by ID ===")
    found_doc = request("GET", f"/api/find/{COLLECTION}/{doc_id}")
    print(f"Found: {found_doc}")

    # Example 3: Find with query
    print("\n=== Find with Query ===")
    docs = request(
        "POST",
        f"/api/find/{COLLECTION}",
        {
            "filter": {
                "type": "Condition",
                "content": {
                    "field": "active",
                    "operator": "Eq",
                    "value": True,
                },
            },
            "limit": 10,
        },
    )
    print(f"Found documents: {docs}")

    # Example 4: Update document
    print("\n=== Update Document ===")
    updated = request(
        "PUT",
        f"/api/update/{COLLECTION}/{doc_id}",
        {
            "name": "Updated Record",
            "value": 100,
        },
    )
    print(f"Updated: {updated}")

    # Example 5: Delete document
    print("\n=== Delete Document ===")
    request("DELETE", f"/api/delete/{COLLECTION}/{doc_id}")
    print("Deleted document")


def main():
    request("DELETE", f"/api/collections/{COLLECTION}")
    operation_error = None
    try:
        run_examples()
    except BaseException as error:  # noqa: BLE001 - cleanup must run on cancellation
        operation_error = error

    try:
        request("DELETE", f"/api/collections/{COLLECTION}")
    except Exception as cleanup_error:
        if operation_error is not None:
            operation_error.add_note(f"cleanup also failed: {cleanup_error}")
        else:
            raise
    if operation_error is not None:
        raise operation_error

    print("\n✓ All CRUD operations completed successfully")


if __name__ == "__main__":
    main()
