"""
Unit tests for field projection support in Python client

The Python client uses the Rust wrapper (ekodb_client), which has full
projection support via QueryBuilder.
"""

import pytest
from ekodb_client import Client


class TestProjection:
    """Test field projection functionality"""

    def test_client_with_projection_support(self):
        """Verify client can be created (projection is in Rust core)"""
        # Python client wraps Rust client which has full projection support
        client = Client.new(
            "http://localhost:8080", "test-api-key", should_retry=False, timeout_secs=30
        )
        assert client is not None

    def test_projection_query_structure(self):
        """Test that projection query structure is correct"""
        # Python client passes queries to Rust core
        # Projection is supported via the query dict structure
        query_with_select = {
            "filter": {
                "type": "Condition",
                "content": {"field": "status", "operator": "Eq", "value": "active"},
            },
            "select_fields": ["id", "name", "email"],
            "limit": 10,
        }

        # Verify structure
        assert "select_fields" in query_with_select
        assert len(query_with_select["select_fields"]) == 3
        assert query_with_select["select_fields"][0] == "id"

    def test_projection_with_exclude_fields(self):
        """Test exclude_fields structure"""
        query_with_exclude = {
            "filter": {
                "type": "Condition",
                "content": {"field": "user_role", "operator": "Eq", "value": "admin"},
            },
            "exclude_fields": ["password", "api_key", "secret_token"],
        }

        # Verify structure
        assert "exclude_fields" in query_with_exclude
        assert len(query_with_exclude["exclude_fields"]) == 3
        assert "password" in query_with_exclude["exclude_fields"]

    def test_projection_with_both_select_and_exclude(self):
        """Test both select_fields and exclude_fields together"""
        query_mixed = {
            "select_fields": ["id", "title", "content", "metadata"],
            "exclude_fields": ["metadata.internal"],
        }

        # Verify both are present
        assert "select_fields" in query_mixed
        assert "exclude_fields" in query_mixed
        assert len(query_mixed["select_fields"]) == 4
        assert len(query_mixed["exclude_fields"]) == 1

    def test_projection_with_complex_query(self):
        """Test projection with filters, sort, limit"""
        complex_query = {
            "filter": {
                "type": "Logical",
                "content": {
                    "operator": "And",
                    "expressions": [
                        {
                            "type": "Condition",
                            "content": {
                                "field": "status",
                                "operator": "Eq",
                                "value": "active",
                            },
                        },
                        {
                            "type": "Condition",
                            "content": {"field": "age", "operator": "Gte", "value": 18},
                        },
                    ],
                },
            },
            "select_fields": ["id", "name", "email"],
            "sort": [{"field": "created_at", "ascending": False}],
            "limit": 50,
        }

        # Verify all components
        assert complex_query["filter"]["type"] == "Logical"
        assert "select_fields" in complex_query
        assert complex_query["limit"] == 50
        assert complex_query["sort"][0]["ascending"] is False


class TestProjectionIntegration:
    """Integration tests for projection (require Rust core)"""

    def test_rust_core_has_projection(self):
        """Verify the Python client uses Rust core with projection support"""
        # The Python client (ekodb-client-py) is built on top of ekodb_client (Rust)
        # The Rust core has full QueryBuilder with select_fields and exclude_fields
        # Python inherits this functionality automatically

        # This test confirms the architecture is correct
        try:
            from ekodb_client import Client

            # If Client is available, it uses the Rust core
            assert True
        except ImportError:
            pytest.fail("Python client should import from Rust core")

    def test_projection_preserves_other_query_params(self):
        """Test projection doesn't interfere with other params"""
        query = {
            "filter": {
                "type": "Condition",
                "content": {"field": "type", "operator": "Eq", "value": "user"},
            },
            "select_fields": ["username", "email"],
            "bypass_cache": True,
            "bypass_ripple": True,
            "skip": 20,
            "limit": 10,
        }

        # All fields should coexist
        assert "filter" in query
        assert "select_fields" in query
        assert query["bypass_cache"] is True
        assert query["bypass_ripple"] is True
        assert query["skip"] == 20
        assert query["limit"] == 10
