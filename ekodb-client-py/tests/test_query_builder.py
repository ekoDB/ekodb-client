"""Tests for QueryBuilder."""

from ekodb_client.query_builder import QueryBuilder


class TestQueryBuilderComparison:
    def test_eq(self):
        query = QueryBuilder().eq("status", "active").build()
        assert query["filter"] == {
            "type": "Condition",
            "content": {"field": "status", "operator": "Eq", "value": "active"},
        }

    def test_ne(self):
        query = QueryBuilder().ne("status", "deleted").build()
        assert query["filter"]["content"]["operator"] == "Ne"

    def test_gt(self):
        query = QueryBuilder().gt("age", 18).build()
        assert query["filter"]["content"]["operator"] == "Gt"
        assert query["filter"]["content"]["value"] == 18

    def test_gte(self):
        query = QueryBuilder().gte("age", 18).build()
        assert query["filter"]["content"]["operator"] == "Gte"

    def test_lt(self):
        query = QueryBuilder().lt("age", 65).build()
        assert query["filter"]["content"]["operator"] == "Lt"

    def test_lte(self):
        query = QueryBuilder().lte("score", 100).build()
        assert query["filter"]["content"]["operator"] == "Lte"

    def test_in_array(self):
        query = QueryBuilder().in_array("status", ["active", "pending"]).build()
        assert query["filter"]["content"]["operator"] == "In"
        assert query["filter"]["content"]["value"] == ["active", "pending"]

    def test_nin(self):
        query = QueryBuilder().nin("status", ["deleted", "archived"]).build()
        assert query["filter"]["content"]["operator"] == "NotIn"

    def test_contains(self):
        query = QueryBuilder().contains("email", "@example.com").build()
        assert query["filter"]["content"]["operator"] == "Contains"

    def test_starts_with(self):
        query = QueryBuilder().starts_with("name", "John").build()
        assert query["filter"]["content"]["operator"] == "StartsWith"

    def test_ends_with(self):
        query = QueryBuilder().ends_with("email", "@example.com").build()
        assert query["filter"]["content"]["operator"] == "EndsWith"

    def test_regex(self):
        query = QueryBuilder().regex("email", "^.*@example\\.com$").build()
        assert query["filter"]["content"]["operator"] == "Contains"


class TestQueryBuilderLogical:
    def test_and(self):
        query = (
            QueryBuilder()
            .and_(
                [
                    QueryBuilder.condition("age", "Gte", 18),
                    QueryBuilder.condition("status", "Eq", "active"),
                ]
            )
            .build()
        )
        f = query["filter"]
        assert f["type"] == "Logical"
        assert f["content"]["operator"] == "And"
        assert len(f["content"]["expressions"]) == 2

    def test_or(self):
        query = (
            QueryBuilder()
            .or_(
                [
                    QueryBuilder.condition("status", "Eq", "active"),
                    QueryBuilder.condition("status", "Eq", "pending"),
                ]
            )
            .build()
        )
        f = query["filter"]
        assert f["type"] == "Logical"
        assert f["content"]["operator"] == "Or"
        assert len(f["content"]["expressions"]) == 2

    def test_not(self):
        query = (
            QueryBuilder()
            .not_(QueryBuilder.condition("status", "Eq", "deleted"))
            .build()
        )
        f = query["filter"]
        assert f["type"] == "Logical"
        assert f["content"]["operator"] == "Not"
        assert len(f["content"]["expressions"]) == 1

    def test_raw_filter(self):
        raw = {
            "type": "Condition",
            "content": {"field": "x", "operator": "Eq", "value": 1},
        }
        query = QueryBuilder().raw_filter(raw).build()
        assert query["filter"] == raw

    def test_multiple_filters_auto_and(self):
        query = QueryBuilder().eq("a", 1).eq("b", 2).build()
        f = query["filter"]
        assert f["type"] == "Logical"
        assert f["content"]["operator"] == "And"
        assert len(f["content"]["expressions"]) == 2


class TestQueryBuilderSorting:
    def test_sort_asc(self):
        query = QueryBuilder().sort_asc("name").build()
        assert query["sort"] == [{"field": "name", "ascending": True}]

    def test_sort_desc(self):
        query = QueryBuilder().sort_desc("created_at").build()
        assert query["sort"] == [{"field": "created_at", "ascending": False}]

    def test_multiple_sorts(self):
        query = QueryBuilder().sort_desc("created_at").sort_asc("name").build()
        assert len(query["sort"]) == 2
        assert query["sort"][0] == {"field": "created_at", "ascending": False}
        assert query["sort"][1] == {"field": "name", "ascending": True}


class TestQueryBuilderPagination:
    def test_limit(self):
        query = QueryBuilder().limit(50).build()
        assert query["limit"] == 50

    def test_skip(self):
        query = QueryBuilder().skip(100).build()
        assert query["skip"] == 100

    def test_page(self):
        query = QueryBuilder().page(2, 20).build()
        assert query["limit"] == 20
        assert query["skip"] == 40  # page * page_size


class TestQueryBuilderJoin:
    def test_join(self):
        join_config = {
            "collections": ["users"],
            "local_field": "user_id",
            "foreign_field": "id",
            "as_field": "user",
        }
        query = QueryBuilder().join(join_config).build()
        assert query["join"] == join_config


class TestQueryBuilderFlags:
    def test_bypass_cache(self):
        query = QueryBuilder().bypass_cache(True).build()
        assert query["bypass_cache"] is True

    def test_bypass_ripple(self):
        query = QueryBuilder().bypass_ripple(True).build()
        assert query["bypass_ripple"] is True

    def test_defaults(self):
        query = QueryBuilder().build()
        assert query["bypass_cache"] is False
        assert query["bypass_ripple"] is False


class TestQueryBuilderProjection:
    def test_select_fields(self):
        query = QueryBuilder().select_fields(["name", "email"]).build()
        assert query["select_fields"] == ["name", "email"]

    def test_exclude_fields(self):
        query = QueryBuilder().exclude_fields(["password"]).build()
        assert query["exclude_fields"] == ["password"]


class TestQueryBuilderConditionHelper:
    def test_condition_creates_dict(self):
        cond = QueryBuilder.condition("status", "Eq", "active")
        assert cond == {
            "type": "Condition",
            "content": {"field": "status", "operator": "Eq", "value": "active"},
        }


class TestQueryBuilderChaining:
    def test_full_chain(self):
        query = (
            QueryBuilder()
            .eq("status", "active")
            .gte("age", 18)
            .contains("email", "@example.com")
            .sort_desc("created_at")
            .limit(10)
            .skip(20)
            .bypass_cache(True)
            .select_fields(["name", "email"])
            .build()
        )

        assert "filter" in query
        assert query["sort"] == [{"field": "created_at", "ascending": False}]
        assert query["limit"] == 10
        assert query["skip"] == 20
        assert query["bypass_cache"] is True
        assert query["select_fields"] == ["name", "email"]

    def test_empty_query(self):
        query = QueryBuilder().build()
        assert "filter" not in query
        assert "sort" not in query
        assert "limit" not in query
        assert "skip" not in query
