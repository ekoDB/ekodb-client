"""
JoinConfig for cross-collection joins in ekoDB queries.

Example:
    ```python
    from ekodb_client import QueryBuilder, JoinConfig

    join = JoinConfig.single("users", "user_id", "id", "user")

    query = (
        QueryBuilder()
        .eq("status", "active")
        .join(join.to_dict())
        .build()
    )

    results = await client.find("orders", query)
    ```
"""

from typing import Any, Dict, List


class JoinConfig:
    """Configuration for joining data across collections."""

    def __init__(
        self,
        collections: List[str],
        local_field: str,
        foreign_field: str,
        as_field: str,
    ):
        """Create a join configuration.

        Args:
            collections: Target collections to join with
            local_field: Field in the source collection
            foreign_field: Field in the target collection(s)
            as_field: Name of the field to store joined data
        """
        self.collections = collections
        self.local_field = local_field
        self.foreign_field = foreign_field
        self.as_field = as_field

    @classmethod
    def single(
        cls, collection: str, local_field: str, foreign_field: str, as_field: str
    ) -> "JoinConfig":
        """Create a join with a single collection."""
        return cls([collection], local_field, foreign_field, as_field)

    @classmethod
    def multi(
        cls,
        collections: List[str],
        local_field: str,
        foreign_field: str,
        as_field: str,
    ) -> "JoinConfig":
        """Create a join with multiple collections."""
        return cls(collections, local_field, foreign_field, as_field)

    def to_dict(self) -> Dict[str, Any]:
        """Convert to dict for use in queries."""
        return {
            "collections": self.collections,
            "local_field": self.local_field,
            "foreign_field": self.foreign_field,
            "as_field": self.as_field,
        }
