"""
Surface tests for collection-listing methods on the Python client.

The Python client wraps the Rust core via PyO3; HTTP behavior is covered by the
Rust client tests. These guard that the binding exposes the methods, including
list_user_collections (#147 parity with the Rust/TS/Kotlin clients).
"""

from ekodb_client import Client


def _client():
    return Client.new(
        "http://localhost:8080", "test-api-key", should_retry=False, timeout_secs=30
    )


def test_list_collections_exposed():
    client = _client()
    assert hasattr(client, "list_collections")
    assert callable(client.list_collections)


def test_list_user_collections_exposed():
    client = _client()
    assert hasattr(client, "list_user_collections")
    assert callable(client.list_user_collections)
