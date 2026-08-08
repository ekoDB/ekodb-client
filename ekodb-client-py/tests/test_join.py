"""Tests for JoinConfig."""

from ekodb_client.join import JoinConfig


class TestJoinConfig:
    def test_single(self):
        join = JoinConfig.single("users", "user_id", "id", "user")
        assert join.collections == ["users"]
        assert join.local_field == "user_id"
        assert join.foreign_field == "id"
        assert join.as_field == "user"

    def test_multi(self):
        join = JoinConfig.multi(["users", "profiles"], "user_id", "id", "user_info")
        assert join.collections == ["users", "profiles"]
        assert join.local_field == "user_id"
        assert join.foreign_field == "id"
        assert join.as_field == "user_info"

    def test_constructor(self):
        join = JoinConfig(
            collections=["orders", "products"],
            local_field="order_id",
            foreign_field="id",
            as_field="order_data",
        )
        assert len(join.collections) == 2

    def test_to_dict(self):
        join = JoinConfig.single("users", "user_id", "id", "user")
        d = join.to_dict()
        assert d == {
            "collections": ["users"],
            "local_field": "user_id",
            "foreign_field": "id",
            "as_field": "user",
        }

    def test_to_dict_multi(self):
        join = JoinConfig.multi(
            ["users", "profiles", "settings"],
            "user_id",
            "id",
            "user_info",
        )
        d = join.to_dict()
        assert len(d["collections"]) == 3
        assert "users" in d["collections"]
        assert "profiles" in d["collections"]
        assert "settings" in d["collections"]

    def test_with_query_builder(self):
        """Test that JoinConfig integrates with QueryBuilder."""
        from ekodb_client.query_builder import QueryBuilder

        join = JoinConfig.single("users", "user_id", "id", "user")
        query = QueryBuilder().eq("status", "active").join(join.to_dict()).build()

        assert query["join"]["collections"] == ["users"]
        assert query["join"]["local_field"] == "user_id"
        assert query["filter"]["content"]["field"] == "status"
