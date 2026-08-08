"""Tests for SchemaBuilder and FieldTypeSchemaBuilder."""

from ekodb_client.schema_builder import SchemaBuilder, FieldTypeSchemaBuilder


class TestFieldTypeSchemaBuilder:
    def test_basic(self):
        schema = FieldTypeSchemaBuilder("string").build()
        assert schema["field_type"] == "string"
        assert schema["required"] is False
        assert schema["unique"] is False

    def test_required(self):
        schema = FieldTypeSchemaBuilder("string").required().build()
        assert schema["required"] is True

    def test_unique(self):
        schema = FieldTypeSchemaBuilder("string").unique().build()
        assert schema["unique"] is True

    def test_default_value(self):
        schema = FieldTypeSchemaBuilder("string").default_value("hello").build()
        assert schema["default"] == "hello"

    def test_enums(self):
        schema = FieldTypeSchemaBuilder("string").enums(["a", "b", "c"]).build()
        assert schema["enums"] == ["a", "b", "c"]

    def test_range(self):
        schema = FieldTypeSchemaBuilder("number").range(0, 100).build()
        assert schema["min"] == 0
        assert schema["max"] == 100

    def test_range_partial(self):
        schema = FieldTypeSchemaBuilder("number").range(min_val=0).build()
        assert schema["min"] == 0
        assert "max" not in schema

    def test_pattern(self):
        schema = FieldTypeSchemaBuilder("string").pattern("^[a-z]+$").build()
        assert schema["regex"] == "^[a-z]+$"

    def test_text_index(self):
        schema = FieldTypeSchemaBuilder("string").text_index().build()
        assert schema["index"] == {"type": "text", "language": "english"}

    def test_text_index_custom(self):
        schema = (
            FieldTypeSchemaBuilder("string")
            .text_index(language="french", analyzer="standard")
            .build()
        )
        assert schema["index"]["language"] == "french"
        assert schema["index"]["analyzer"] == "standard"

    def test_vector_index(self):
        schema = FieldTypeSchemaBuilder("vector").vector_index().build()
        assert schema["index"]["type"] == "vector"
        assert schema["index"]["algorithm"] == "flat"
        assert schema["index"]["metric"] == "cosine"
        assert schema["index"]["m"] == 16
        assert schema["index"]["ef_construction"] == 200

    def test_vector_index_custom(self):
        schema = (
            FieldTypeSchemaBuilder("vector")
            .vector_index(
                algorithm="hnsw", metric="euclidean", m=32, ef_construction=400
            )
            .build()
        )
        assert schema["index"]["algorithm"] == "hnsw"
        assert schema["index"]["metric"] == "euclidean"
        assert schema["index"]["m"] == 32
        assert schema["index"]["ef_construction"] == 400

    def test_btree_index(self):
        schema = FieldTypeSchemaBuilder("number").btree_index().build()
        assert schema["index"] == {"type": "btree"}

    def test_hash_index(self):
        schema = FieldTypeSchemaBuilder("string").hash_index().build()
        assert schema["index"] == {"type": "hash"}

    def test_chaining(self):
        schema = (
            FieldTypeSchemaBuilder("string")
            .required()
            .unique()
            .pattern("^[a-z]+$")
            .text_index()
            .build()
        )
        assert schema["required"] is True
        assert schema["unique"] is True
        assert schema["regex"] == "^[a-z]+$"
        assert schema["index"]["type"] == "text"


class TestSchemaBuilder:
    def test_basic(self):
        schema = (
            SchemaBuilder()
            .add_field("name", FieldTypeSchemaBuilder("string").required().build())
            .build()
        )
        assert "name" in schema["fields"]
        assert schema["fields"]["name"]["required"] is True
        assert schema["version"] == 1
        assert schema["bypass_ripple"] is True

    def test_multiple_fields(self):
        schema = (
            SchemaBuilder()
            .add_field("name", FieldTypeSchemaBuilder("string").required().build())
            .add_field("email", FieldTypeSchemaBuilder("string").unique().build())
            .add_field("age", FieldTypeSchemaBuilder("number").range(0, 150).build())
            .build()
        )
        assert len(schema["fields"]) == 3
        assert schema["fields"]["email"]["unique"] is True
        assert schema["fields"]["age"]["min"] == 0

    def test_auto_build_field(self):
        schema = (
            SchemaBuilder()
            .add_field("name", FieldTypeSchemaBuilder("string").required())
            .build()
        )
        assert schema["fields"]["name"]["required"] is True

    def test_bypass_ripple(self):
        schema = SchemaBuilder().bypass_ripple(False).build()
        assert schema["bypass_ripple"] is False

    def test_version(self):
        schema = SchemaBuilder().version(2).build()
        assert schema["version"] == 2

    def test_full_schema(self):
        schema = (
            SchemaBuilder()
            .add_field(
                "title",
                FieldTypeSchemaBuilder("string").required().text_index().build(),
            )
            .add_field(
                "embedding",
                FieldTypeSchemaBuilder("vector")
                .vector_index(algorithm="hnsw", metric="cosine")
                .build(),
            )
            .add_field(
                "category",
                FieldTypeSchemaBuilder("string").enums(["A", "B", "C"]).build(),
            )
            .add_field(
                "score",
                FieldTypeSchemaBuilder("number").range(0, 100).btree_index().build(),
            )
            .version(2)
            .bypass_ripple(False)
            .build()
        )

        assert schema["version"] == 2
        assert schema["bypass_ripple"] is False
        assert len(schema["fields"]) == 4
        assert schema["fields"]["title"]["index"]["type"] == "text"
        assert schema["fields"]["embedding"]["index"]["type"] == "vector"
        assert schema["fields"]["category"]["enums"] == ["A", "B", "C"]
        assert schema["fields"]["score"]["index"]["type"] == "btree"
