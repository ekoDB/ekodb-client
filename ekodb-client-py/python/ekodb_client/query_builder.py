"""
QueryBuilder for constructing ekoDB queries with a fluent API.

Example:
    ```python
    from ekodb_client import QueryBuilder

    query = (
        QueryBuilder()
        .eq("status", "active")
        .gte("age", 18)
        .sort_desc("created_at")
        .limit(10)
        .build()
    )

    results = await client.find("users", query)
    ```
"""

from typing import Any, Dict, List, Optional


class QueryBuilder:
    """Fluent builder for constructing ekoDB queries."""

    def __init__(self):
        self._filters: List[Dict[str, Any]] = []
        self._sort_fields: List[Dict[str, Any]] = []
        self._limit: Optional[int] = None
        self._skip: Optional[int] = None
        self._join: Optional[Dict[str, Any]] = None
        self._bypass_cache: bool = False
        self._bypass_ripple: bool = False
        self._select_fields: Optional[List[str]] = None
        self._exclude_fields: Optional[List[str]] = None

    # =========================================================================
    # Comparison Operators
    # =========================================================================

    def _add_condition(self, field: str, operator: str, value: Any) -> "QueryBuilder":
        self._filters.append(
            {
                "type": "Condition",
                "content": {
                    "field": field,
                    "operator": operator,
                    "value": value,
                },
            }
        )
        return self

    def eq(self, field: str, value: Any) -> "QueryBuilder":
        """Add an equality filter."""
        return self._add_condition(field, "Eq", value)

    def ne(self, field: str, value: Any) -> "QueryBuilder":
        """Add a not-equal filter."""
        return self._add_condition(field, "Ne", value)

    def gt(self, field: str, value: Any) -> "QueryBuilder":
        """Add a greater-than filter."""
        return self._add_condition(field, "Gt", value)

    def gte(self, field: str, value: Any) -> "QueryBuilder":
        """Add a greater-than-or-equal filter."""
        return self._add_condition(field, "Gte", value)

    def lt(self, field: str, value: Any) -> "QueryBuilder":
        """Add a less-than filter."""
        return self._add_condition(field, "Lt", value)

    def lte(self, field: str, value: Any) -> "QueryBuilder":
        """Add a less-than-or-equal filter."""
        return self._add_condition(field, "Lte", value)

    def in_array(self, field: str, values: List[Any]) -> "QueryBuilder":
        """Add an 'in' filter (field value in list)."""
        return self._add_condition(field, "In", values)

    def nin(self, field: str, values: List[Any]) -> "QueryBuilder":
        """Add a 'not in' filter (field value not in list)."""
        return self._add_condition(field, "NotIn", values)

    def contains(self, field: str, value: str) -> "QueryBuilder":
        """Add a substring contains filter."""
        return self._add_condition(field, "Contains", value)

    def starts_with(self, field: str, value: str) -> "QueryBuilder":
        """Add a starts-with filter."""
        return self._add_condition(field, "StartsWith", value)

    def ends_with(self, field: str, value: str) -> "QueryBuilder":
        """Add an ends-with filter."""
        return self._add_condition(field, "EndsWith", value)

    def regex(self, field: str, pattern: str) -> "QueryBuilder":
        """Add a regex filter (uses Contains operator)."""
        return self._add_condition(field, "Contains", pattern)

    # =========================================================================
    # Logical Operators
    # =========================================================================

    def and_(self, conditions: List[Dict[str, Any]]) -> "QueryBuilder":
        """Combine filters with AND logic."""
        self._filters.append(
            {
                "type": "Logical",
                "content": {
                    "operator": "And",
                    "expressions": conditions,
                },
            }
        )
        return self

    def or_(self, conditions: List[Dict[str, Any]]) -> "QueryBuilder":
        """Combine filters with OR logic."""
        self._filters.append(
            {
                "type": "Logical",
                "content": {
                    "operator": "Or",
                    "expressions": conditions,
                },
            }
        )
        return self

    def not_(self, condition: Dict[str, Any]) -> "QueryBuilder":
        """Negate a filter."""
        self._filters.append(
            {
                "type": "Logical",
                "content": {
                    "operator": "Not",
                    "expressions": [condition],
                },
            }
        )
        return self

    def raw_filter(self, filter_expr: Dict[str, Any]) -> "QueryBuilder":
        """Add a raw filter expression."""
        self._filters.append(filter_expr)
        return self

    # =========================================================================
    # Sorting
    # =========================================================================

    def sort_asc(self, field: str) -> "QueryBuilder":
        """Add ascending sort on a field."""
        self._sort_fields.append({"field": field, "ascending": True})
        return self

    def sort_desc(self, field: str) -> "QueryBuilder":
        """Add descending sort on a field."""
        self._sort_fields.append({"field": field, "ascending": False})
        return self

    # =========================================================================
    # Pagination
    # =========================================================================

    def limit(self, limit: int) -> "QueryBuilder":
        """Set maximum number of results."""
        self._limit = limit
        return self

    def skip(self, skip: int) -> "QueryBuilder":
        """Set number of results to skip."""
        self._skip = skip
        return self

    def page(self, page_num: int, page_size: int) -> "QueryBuilder":
        """Set pagination (page number and size)."""
        self._skip = page_num * page_size
        self._limit = page_size
        return self

    # =========================================================================
    # Joins
    # =========================================================================

    def join(self, join_config: Dict[str, Any]) -> "QueryBuilder":
        """Add a join configuration."""
        self._join = join_config
        return self

    # =========================================================================
    # Performance Flags
    # =========================================================================

    def bypass_cache(self, bypass: bool = True) -> "QueryBuilder":
        """Bypass cache for this query."""
        self._bypass_cache = bypass
        return self

    def bypass_ripple(self, bypass: bool = True) -> "QueryBuilder":
        """Bypass ripple propagation for this query."""
        self._bypass_ripple = bypass
        return self

    # =========================================================================
    # Field Projection
    # =========================================================================

    def select_fields(self, fields: List[str]) -> "QueryBuilder":
        """Select specific fields to return."""
        self._select_fields = fields
        return self

    def exclude_fields(self, fields: List[str]) -> "QueryBuilder":
        """Exclude specific fields from results."""
        self._exclude_fields = fields
        return self

    # =========================================================================
    # Build
    # =========================================================================

    def build(self) -> Dict[str, Any]:
        """Build the final query dict.

        Returns a dict compatible with ekoDB's query format, suitable for
        passing directly to ``client.find(collection, query)``.
        """
        query: Dict[str, Any] = {}

        # Combine filters
        if self._filters:
            if len(self._filters) == 1:
                query["filter"] = self._filters[0]
            else:
                query["filter"] = {
                    "type": "Logical",
                    "content": {
                        "operator": "And",
                        "expressions": self._filters,
                    },
                }

        # Sort
        if self._sort_fields:
            query["sort"] = self._sort_fields

        # Pagination
        if self._limit is not None:
            query["limit"] = self._limit
        if self._skip is not None:
            query["skip"] = self._skip

        # Join
        if self._join is not None:
            query["join"] = self._join

        # Performance flags
        query["bypass_cache"] = self._bypass_cache
        query["bypass_ripple"] = self._bypass_ripple

        # Projection
        if self._select_fields is not None:
            query["select_fields"] = self._select_fields
        if self._exclude_fields is not None:
            query["exclude_fields"] = self._exclude_fields

        return query

    @staticmethod
    def condition(field: str, operator: str, value: Any) -> Dict[str, Any]:
        """Create a standalone condition dict for use with and_/or_/not_.

        Example:
            ```python
            query = (
                QueryBuilder()
                .or_([
                    QueryBuilder.condition("status", "Eq", "active"),
                    QueryBuilder.condition("status", "Eq", "pending"),
                ])
                .build()
            )
            ```
        """
        return {
            "type": "Condition",
            "content": {
                "field": field,
                "operator": operator,
                "value": value,
            },
        }
