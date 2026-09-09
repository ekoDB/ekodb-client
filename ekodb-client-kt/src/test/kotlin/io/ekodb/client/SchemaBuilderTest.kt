package io.ekodb.client

import kotlinx.serialization.json.*
import java.io.File
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFalse
import kotlin.test.assertTrue

class SchemaBuilderTest {
    @Test
    fun `intent match flat schema aliases match accepted server field types`() {
        val evidence = Json.parseToJsonElement(File("../test-fixtures/schema-probe-0.72.2.json").readText()).jsonObject
        val original = evidence.getValue("exactSchema").jsonObject.getValue("fields").jsonObject
        val accepted = evidence.getValue("attempts").jsonArray.map { it.jsonObject }
            .single { it["fieldType"] == JsonPrimitive("Vector") }
        val serverFields = Json.parseToJsonElement(accepted.getValue("schema").jsonObject.getValue("body").jsonPrimitive.content)
            .jsonObject.getValue("fields").jsonObject
        for ((name, value) in original) {
            val field = value.jsonObject
            val builder = FieldTypeSchemaBuilder(field.getValue("field_type").jsonPrimitive.content).required()
            if (name == "embedding") builder.vectorIndex("flat", "cosine", 16, 200)
            val built = builder.build()
            assertEquals(serverFields.getValue(name).jsonObject["field_type"], built["field_type"], name)
            assertEquals(field["required"], built["required"], name)
            assertEquals(field["index"], built["index"], name)
        }
    }

    // ========================================================================
    // FieldTypeSchemaBuilder Tests
    // ========================================================================

    @Test
    fun `basic field type schema`() {
        val schema = FieldTypeSchemaBuilder("string").build()
        assertEquals("String", schema["field_type"]?.jsonPrimitive?.content)
        assertFalse(schema["required"]?.jsonPrimitive?.boolean ?: true)
        assertFalse(schema["unique"]?.jsonPrimitive?.boolean ?: true)
    }

    @Test
    fun `required field`() {
        val schema = FieldTypeSchemaBuilder("string").required().build()
        assertTrue(schema["required"]?.jsonPrimitive?.boolean ?: false)
    }

    @Test
    fun `unique field`() {
        val schema = FieldTypeSchemaBuilder("string").unique().build()
        assertTrue(schema["unique"]?.jsonPrimitive?.boolean ?: false)
    }

    @Test
    fun `default value string`() {
        val schema = FieldTypeSchemaBuilder("string").defaultValue("hello").build()
        assertEquals("hello", schema["default"]?.jsonPrimitive?.content)
    }

    @Test
    fun `default value number`() {
        val schema = FieldTypeSchemaBuilder("number").defaultValue(42).build()
        assertEquals(42, schema["default"]?.jsonPrimitive?.int)
    }

    @Test
    fun `enum values`() {
        val schema = FieldTypeSchemaBuilder("string")
            .enumStrings("A", "B", "C")
            .build()
        val enums = schema["enums"]?.jsonArray
        assertEquals(3, enums?.size)
        assertEquals("A", enums?.get(0)?.jsonPrimitive?.content)
    }

    @Test
    fun `range constraints`() {
        val schema = FieldTypeSchemaBuilder("number").range(0, 100).build()
        assertEquals(0, schema["min"]?.jsonPrimitive?.int)
        assertEquals(100, schema["max"]?.jsonPrimitive?.int)
    }

    @Test
    fun `regex pattern`() {
        val schema = FieldTypeSchemaBuilder("string").pattern("^[a-z]+$").build()
        assertEquals("^[a-z]+$", schema["regex"]?.jsonPrimitive?.content)
    }

    @Test
    fun `text index defaults`() {
        val schema = FieldTypeSchemaBuilder("string").textIndex().build()
        val index = schema["index"]?.jsonObject
        assertEquals("text", index?.get("type")?.jsonPrimitive?.content)
        assertEquals("english", index?.get("language")?.jsonPrimitive?.content)
    }

    @Test
    fun `text index custom language`() {
        val schema = FieldTypeSchemaBuilder("string")
            .textIndex(language = "french", analyzer = "standard")
            .build()
        val index = schema["index"]?.jsonObject
        assertEquals("french", index?.get("language")?.jsonPrimitive?.content)
        assertEquals("standard", index?.get("analyzer")?.jsonPrimitive?.content)
    }

    @Test
    fun `vector index defaults`() {
        val schema = FieldTypeSchemaBuilder("vector").vectorIndex().build()
        assertEquals("Vector", schema["field_type"]?.jsonPrimitive?.content)
        val index = schema["index"]?.jsonObject
        assertEquals("vector", index?.get("type")?.jsonPrimitive?.content)
        assertEquals("flat", index?.get("algorithm")?.jsonPrimitive?.content)
        assertEquals("cosine", index?.get("metric")?.jsonPrimitive?.content)
        assertEquals(16, index?.get("m")?.jsonPrimitive?.int)
        assertEquals(200, index?.get("ef_construction")?.jsonPrimitive?.int)
    }

    @Test
    fun `vector index custom params`() {
        val schema = FieldTypeSchemaBuilder("vector")
            .vectorIndex(algorithm = "hnsw", metric = "euclidean", m = 32, efConstruction = 400)
            .build()
        assertEquals("Vector", schema["field_type"]?.jsonPrimitive?.content)
        val index = schema["index"]?.jsonObject
        assertEquals("hnsw", index?.get("algorithm")?.jsonPrimitive?.content)
        assertEquals("euclidean", index?.get("metric")?.jsonPrimitive?.content)
        assertEquals(32, index?.get("m")?.jsonPrimitive?.int)
        assertEquals(400, index?.get("ef_construction")?.jsonPrimitive?.int)
    }

    @Test
    fun `known field type aliases use server casing and unknown types pass through`() {
        for (canonical in listOf("String", "Integer", "Float", "Boolean", "DateTime", "UUID", "Array", "Object",
            "Decimal", "Bytes", "Null", "Number", "Set", "Vector", "Binary", "Duration", "FutureType")) {
            assertEquals(canonical, FieldTypeSchemaBuilder(canonical).build()["field_type"]?.jsonPrimitive?.content)
            if (canonical != "FutureType") {
                assertEquals(canonical, FieldTypeSchemaBuilder(canonical.lowercase()).build()["field_type"]?.jsonPrimitive?.content)
            }
        }
    }

    @Test
    fun `array and vector schemas retain distinct types with identical vector indexes`() {
        for (algorithm in listOf("flat", "hnsw")) {
            val array = FieldTypeSchemaBuilder("Array").required().vectorIndex(algorithm).build()
            val vector = FieldTypeSchemaBuilder("vector").required().vectorIndex(algorithm).build()
            assertEquals("Array", array["field_type"]?.jsonPrimitive?.content)
            assertEquals("Vector", vector["field_type"]?.jsonPrimitive?.content)
            assertEquals(array["index"], vector["index"])
            assertEquals(JsonPrimitive(true), vector["required"])
        }
    }

    @Test
    fun `btree index`() {
        val schema = FieldTypeSchemaBuilder("number").btreeIndex().build()
        val index = schema["index"]?.jsonObject
        assertEquals("btree", index?.get("type")?.jsonPrimitive?.content)
    }

    @Test
    fun `hash index`() {
        val schema = FieldTypeSchemaBuilder("string").hashIndex().build()
        val index = schema["index"]?.jsonObject
        assertEquals("hash", index?.get("type")?.jsonPrimitive?.content)
    }

    @Test
    fun `chained field schema`() {
        val schema = FieldTypeSchemaBuilder("string")
            .required()
            .unique()
            .pattern("^[a-z]+$")
            .textIndex()
            .build()
        assertTrue(schema["required"]?.jsonPrimitive?.boolean ?: false)
        assertTrue(schema["unique"]?.jsonPrimitive?.boolean ?: false)
        assertEquals("^[a-z]+$", schema["regex"]?.jsonPrimitive?.content)
        assertEquals("text", schema["index"]?.jsonObject?.get("type")?.jsonPrimitive?.content)
    }

    // ========================================================================
    // SchemaBuilder Tests
    // ========================================================================

    @Test
    fun `basic schema with one field`() {
        val schema = SchemaBuilder()
            .addField("name", FieldTypeSchemaBuilder("string").required())
            .build()
        val fields = schema["fields"]?.jsonObject
        assertEquals(1, fields?.size)
        assertTrue(fields?.containsKey("name") ?: false)
        assertEquals(1, schema["version"]?.jsonPrimitive?.int)
        assertTrue(schema["bypass_ripple"]?.jsonPrimitive?.boolean ?: false)
    }

    @Test
    fun `schema with multiple fields`() {
        val schema = SchemaBuilder()
            .addField("name", FieldTypeSchemaBuilder("string").required())
            .addField("email", FieldTypeSchemaBuilder("string").unique())
            .addField("age", FieldTypeSchemaBuilder("number").range(0, 150))
            .build()
        val fields = schema["fields"]?.jsonObject
        assertEquals(3, fields?.size)
    }

    @Test
    fun `schema with custom version`() {
        val schema = SchemaBuilder().version(2).build()
        assertEquals(2, schema["version"]?.jsonPrimitive?.int)
    }

    @Test
    fun `schema bypass ripple false`() {
        val schema = SchemaBuilder().bypassRipple(false).build()
        assertFalse(schema["bypass_ripple"]?.jsonPrimitive?.boolean ?: true)
    }

    @Test
    fun `full schema with indexes`() {
        val schema = SchemaBuilder()
            .addField("title", FieldTypeSchemaBuilder("string").required().textIndex())
            .addField("embedding", FieldTypeSchemaBuilder("vector").vectorIndex(algorithm = "hnsw"))
            .addField("category", FieldTypeSchemaBuilder("string").enumStrings("A", "B", "C"))
            .addField("score", FieldTypeSchemaBuilder("number").range(0, 100).btreeIndex())
            .version(2)
            .bypassRipple(false)
            .build()

        assertEquals(2, schema["version"]?.jsonPrimitive?.int)
        assertFalse(schema["bypass_ripple"]?.jsonPrimitive?.boolean ?: true)

        val fields = schema["fields"]?.jsonObject
        assertEquals(4, fields?.size)

        val titleIndex = fields?.get("title")?.jsonObject?.get("index")?.jsonObject
        assertEquals("text", titleIndex?.get("type")?.jsonPrimitive?.content)

        val embeddingIndex = fields?.get("embedding")?.jsonObject?.get("index")?.jsonObject
        assertEquals("vector", embeddingIndex?.get("type")?.jsonPrimitive?.content)

        val scoreIndex = fields?.get("score")?.jsonObject?.get("index")?.jsonObject
        assertEquals("btree", scoreIndex?.get("type")?.jsonPrimitive?.content)
    }
}
