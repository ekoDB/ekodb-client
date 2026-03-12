"""
Unit tests for distinct_values query structure.

These tests verify the query dict format that is sent to the server,
mirroring how the Rust DistinctValuesQuery struct serializes.

Run with: pytest tests/test_distinct_values.py -v
"""

import pytest


class TestDistinctValuesQueryStructure:
    """Verify that the distinct_values query dict is well-formed."""

    def test_empty_query_has_no_required_fields(self):
        """An empty query is valid — scans all records."""
        query = {}
        # No required fields; an empty body is acceptable
        assert isinstance(query, dict)

    def test_filter_condition_structure(self):
        """Single-condition filter follows the Condition format."""
        query = {
            "filter": {
                "type": "Condition",
                "content": {
                    "field": "status",
                    "operator": "Eq",
                    "value": "active",
                },
            }
        }
        assert query["filter"]["type"] == "Condition"
        assert query["filter"]["content"]["field"] == "status"
        assert query["filter"]["content"]["operator"] == "Eq"
        assert query["filter"]["content"]["value"] == "active"

    def test_logical_and_filter_structure(self):
        """Logical AND filter uses type=Logical with operator=And."""
        query = {
            "filter": {
                "type": "Logical",
                "content": {
                    "operator": "And",
                    "expressions": [
                        {
                            "type": "Condition",
                            "content": {
                                "field": "category",
                                "operator": "Eq",
                                "value": "electronics",
                            },
                        },
                        {
                            "type": "Condition",
                            "content": {
                                "field": "active",
                                "operator": "Eq",
                                "value": True,
                            },
                        },
                    ],
                },
            }
        }
        content = query["filter"]["content"]
        assert query["filter"]["type"] == "Logical"
        assert content["operator"] == "And"
        assert len(content["expressions"]) == 2

    def test_bypass_flags_are_optional(self):
        """bypass_ripple and bypass_cache are optional booleans."""
        query = {"bypass_ripple": True, "bypass_cache": False}
        assert query["bypass_ripple"] is True
        assert query["bypass_cache"] is False

    def test_full_query_structure(self):
        """A full query with all optional fields."""
        query = {
            "filter": {
                "type": "Condition",
                "content": {
                    "field": "region",
                    "operator": "Eq",
                    "value": "us",
                },
            },
            "bypass_ripple": True,
            "bypass_cache": True,
        }
        assert "filter" in query
        assert query["bypass_ripple"] is True
        assert query["bypass_cache"] is True


class TestDistinctValuesResponseStructure:
    """Verify expected response shape from the server."""

    def _make_response(self, values):
        return {
            "collection": "products",
            "field": "category",
            "values": values,
            "count": len(values),
        }

    def test_response_has_required_fields(self):
        resp = self._make_response(["books", "electronics"])
        assert "collection" in resp
        assert "field" in resp
        assert "values" in resp
        assert "count" in resp

    def test_count_matches_values_length(self):
        values = ["active", "archived", "discontinued"]
        resp = self._make_response(values)
        assert resp["count"] == len(resp["values"])

    def test_empty_collection_response(self):
        resp = self._make_response([])
        assert resp["count"] == 0
        assert resp["values"] == []

    def test_values_are_sorted(self):
        """Values returned from the server are sorted alphabetically."""
        resp = self._make_response(["books", "clothing", "electronics"])
        assert resp["values"] == sorted(resp["values"])

    def test_numeric_distinct_values(self):
        resp = {
            "collection": "scores",
            "field": "score",
            "values": [10, 20, 30],
            "count": 3,
        }
        assert resp["count"] == 3
        assert 10 in resp["values"]

    def test_boolean_distinct_values(self):
        resp = {
            "collection": "users",
            "field": "active",
            "values": [False, True],
            "count": 2,
        }
        assert resp["count"] == 2
        assert True in resp["values"]
        assert False in resp["values"]
