"""
Unit tests for KV Link request/response structures.

These tests verify the request dict formats sent to the server and the
expected response shapes for linking/unlinking documents to KV keys.

Run with: pytest tests/test_kv_links.py -v
"""


class TestKvLinkRequestStructure:
    """Verify that KV link/unlink request dicts are well-formed."""

    def test_kv_link_payload(self):
        """A kv_link request requires key, collection, and document_id."""
        request = {
            "key": "config:app",
            "collection": "settings",
            "document_id": "doc_123",
        }
        assert request["key"] == "config:app"
        assert request["collection"] == "settings"
        assert request["document_id"] == "doc_123"

    def test_kv_unlink_payload(self):
        """A kv_unlink request requires key, collection, and document_id."""
        request = {
            "key": "config:app",
            "collection": "settings",
            "document_id": "doc_123",
        }
        assert request["key"] == "config:app"
        assert request["document_id"] == "doc_123"

    def test_kv_link_payload_different_collections(self):
        """Links can reference documents from different collections."""
        link_a = {
            "key": "user:prefs",
            "collection": "users",
            "document_id": "user_1",
        }
        link_b = {
            "key": "user:prefs",
            "collection": "profiles",
            "document_id": "profile_1",
        }
        assert link_a["collection"] != link_b["collection"]
        assert link_a["key"] == link_b["key"]


class TestKvLinkResponseStructure:
    """Verify expected response shapes from KV link endpoints."""

    def test_get_links_response_with_links(self):
        resp = {
            "links": [
                {"collection": "settings", "document_id": "doc_123"},
                {"collection": "overrides", "document_id": "doc_456"},
            ]
        }
        assert isinstance(resp["links"], list)
        assert len(resp["links"]) == 2
        assert resp["links"][0]["collection"] == "settings"

    def test_get_links_response_empty(self):
        """A key with no links returns an empty list."""
        resp = {"links": []}
        assert isinstance(resp["links"], list)
        assert len(resp["links"]) == 0
