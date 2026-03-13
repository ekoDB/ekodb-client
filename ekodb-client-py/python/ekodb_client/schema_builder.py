"""
SchemaBuilder for defining ekoDB collection schemas with a fluent API.

Example:
    ```python
    from ekodb_client import SchemaBuilder, FieldTypeSchemaBuilder

    schema = (
        SchemaBuilder()
        .add_field("name", FieldTypeSchemaBuilder("string").required().build())
        .add_field("email", FieldTypeSchemaBuilder("string").required().unique().build())
        .add_field("age", FieldTypeSchemaBuilder("number").range(0, 150).build())
        .add_field("bio", FieldTypeSchemaBuilder("string").text_index().build())
        .add_field("embedding", FieldTypeSchemaBuilder("vector").vector_index().build())
        .build()
    )

    await client.create_collection("users", schema)
    ```
"""

from typing import Any, Dict, List, Optional, Union


class FieldTypeSchemaBuilder:
    """Fluent builder for field type schemas with constraints and indexes."""

    def __init__(self, field_type: str):
        """Create a field type schema builder.

        Args:
            field_type: The field type (e.g., "string", "number", "boolean",
                        "vector", "datetime", "uuid", "decimal", "array",
                        "set", "object", "binary", "bytes", "duration")
        """
        self._field_type = field_type
        self._required = False
        self._unique = False
        self._default: Optional[Any] = None
        self._enums: List[Any] = []
        self._min: Optional[Any] = None
        self._max: Optional[Any] = None
        self._regex: Optional[str] = None
        self._index: Optional[Dict[str, Any]] = None

    def required(self) -> "FieldTypeSchemaBuilder":
        """Mark the field as required."""
        self._required = True
        return self

    def unique(self) -> "FieldTypeSchemaBuilder":
        """Enforce uniqueness on this field."""
        self._unique = True
        return self

    def default_value(self, value: Any) -> "FieldTypeSchemaBuilder":
        """Set a default value for the field."""
        self._default = value
        return self

    def enums(self, values: List[Any]) -> "FieldTypeSchemaBuilder":
        """Restrict field to a set of allowed values."""
        self._enums = values
        return self

    def range(
        self, min_val: Optional[Any] = None, max_val: Optional[Any] = None
    ) -> "FieldTypeSchemaBuilder":
        """Set min/max value constraints."""
        self._min = min_val
        self._max = max_val
        return self

    def pattern(self, regex: str) -> "FieldTypeSchemaBuilder":
        """Set a regex validation pattern."""
        self._regex = regex
        return self

    def text_index(
        self, language: str = "english", analyzer: Optional[str] = None
    ) -> "FieldTypeSchemaBuilder":
        """Add a full-text search index."""
        index: Dict[str, Any] = {"type": "text", "language": language}
        if analyzer is not None:
            index["analyzer"] = analyzer
        self._index = index
        return self

    def vector_index(
        self,
        algorithm: str = "flat",
        metric: str = "cosine",
        m: int = 16,
        ef_construction: int = 200,
    ) -> "FieldTypeSchemaBuilder":
        """Add a vector similarity search index.

        Args:
            algorithm: Index algorithm ("flat", "hnsw", "ivf")
            metric: Distance metric ("cosine", "euclidean", "dotproduct")
            m: HNSW parameter for max connections per node
            ef_construction: HNSW parameter for construction-time search width
        """
        self._index = {
            "type": "vector",
            "algorithm": algorithm,
            "metric": metric,
            "m": m,
            "ef_construction": ef_construction,
        }
        return self

    def btree_index(self) -> "FieldTypeSchemaBuilder":
        """Add a B-tree index for range queries and exact matches."""
        self._index = {"type": "btree"}
        return self

    def hash_index(self) -> "FieldTypeSchemaBuilder":
        """Add a hash index for exact matches (faster than B-tree)."""
        self._index = {"type": "hash"}
        return self

    def build(self) -> Dict[str, Any]:
        """Build the field type schema dict."""
        schema: Dict[str, Any] = {
            "field_type": self._field_type,
            "required": self._required,
            "unique": self._unique,
        }

        if self._default is not None:
            schema["default"] = self._default
        if self._enums:
            schema["enums"] = self._enums
        if self._min is not None:
            schema["min"] = self._min
        if self._max is not None:
            schema["max"] = self._max
        if self._regex is not None:
            schema["regex"] = self._regex
        if self._index is not None:
            schema["index"] = self._index

        return schema


class SchemaBuilder:
    """Fluent builder for collection schemas."""

    def __init__(self):
        self._fields: Dict[str, Dict[str, Any]] = {}
        self._bypass_ripple: bool = True
        self._version: int = 1

    def add_field(
        self, name: str, field_schema: Union[Dict[str, Any], "FieldTypeSchemaBuilder"]
    ) -> "SchemaBuilder":
        """Add a field definition to the schema.

        Args:
            name: The field name
            field_schema: Either a dict from FieldTypeSchemaBuilder.build()
                          or a FieldTypeSchemaBuilder instance (auto-built)
        """
        if isinstance(field_schema, FieldTypeSchemaBuilder):
            self._fields[name] = field_schema.build()
        else:
            self._fields[name] = field_schema
        return self

    def bypass_ripple(self, bypass: bool = True) -> "SchemaBuilder":
        """Set whether to bypass ripple on collection creation."""
        self._bypass_ripple = bypass
        return self

    def version(self, version: int) -> "SchemaBuilder":
        """Set the schema version."""
        self._version = version
        return self

    def build(self) -> Dict[str, Any]:
        """Build the final schema dict."""
        return {
            "fields": self._fields,
            "version": self._version,
            "bypass_ripple": self._bypass_ripple,
        }
