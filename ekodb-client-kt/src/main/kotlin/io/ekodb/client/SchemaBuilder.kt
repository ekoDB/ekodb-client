package io.ekodb.client

import kotlinx.serialization.json.*

/**
 * Fluent builder for field type schemas with constraints and indexes.
 *
 * Example:
 * ```kotlin
 * val field = FieldTypeSchemaBuilder("string")
 *     .required()
 *     .unique()
 *     .textIndex()
 *     .build()
 * ```
 */
class FieldTypeSchemaBuilder(private val fieldType: String) {
    private var required = false
    private var unique = false
    private var default: JsonElement? = null
    private var enums: List<JsonElement>? = null
    private var min: JsonElement? = null
    private var max: JsonElement? = null
    private var regex: String? = null
    private var index: JsonObject? = null

    /** Mark the field as required. */
    fun required() = apply { this.required = true }

    /** Enforce uniqueness on this field. */
    fun unique() = apply { this.unique = true }

    /** Set a default value for the field. */
    fun defaultValue(value: JsonElement) = apply { this.default = value }

    /** Set a default string value. */
    fun defaultValue(value: String) = apply { this.default = JsonPrimitive(value) }

    /** Set a default numeric value. */
    fun defaultValue(value: Number) = apply { this.default = JsonPrimitive(value) }

    /** Set a default boolean value. */
    fun defaultValue(value: Boolean) = apply { this.default = JsonPrimitive(value) }

    /** Restrict field to a set of allowed values. */
    fun enums(values: List<JsonElement>) = apply { this.enums = values }

    /** Restrict field to a set of allowed string values. */
    fun enumStrings(vararg values: String) = apply {
        this.enums = values.map { JsonPrimitive(it) }
    }

    /** Set min/max value constraints. */
    fun range(min: Number? = null, max: Number? = null) = apply {
        min?.let { this.min = JsonPrimitive(it) }
        max?.let { this.max = JsonPrimitive(it) }
    }

    /** Set a regex validation pattern. */
    fun pattern(regex: String) = apply { this.regex = regex }

    /** Add a full-text search index. */
    fun textIndex(language: String = "english", analyzer: String? = null) = apply {
        this.index = buildJsonObject {
            put("type", "text")
            put("language", language)
            analyzer?.let { put("analyzer", it) }
        }
    }

    /**
     * Add a vector similarity search index.
     *
     * @param algorithm Index algorithm ("flat", "hnsw", "ivf")
     * @param metric Distance metric ("cosine", "euclidean", "dotproduct")
     * @param m HNSW parameter for max connections per node
     * @param efConstruction HNSW parameter for construction-time search width
     */
    fun vectorIndex(
        algorithm: String = "flat",
        metric: String = "cosine",
        m: Int = 16,
        efConstruction: Int = 200
    ) = apply {
        this.index = buildJsonObject {
            put("type", "vector")
            put("algorithm", algorithm)
            put("metric", metric)
            put("m", m)
            put("ef_construction", efConstruction)
        }
    }

    /** Add a B-tree index for range queries and exact matches. */
    fun btreeIndex() = apply {
        this.index = buildJsonObject { put("type", "btree") }
    }

    /** Add a hash index for exact matches (faster than B-tree). */
    fun hashIndex() = apply {
        this.index = buildJsonObject { put("type", "hash") }
    }

    /** Build the field type schema as a JsonObject. */
    fun build(): JsonObject = buildJsonObject {
        put("field_type", fieldType)
        put("required", required)
        put("unique", unique)
        default?.let { put("default", it) }
        enums?.let { put("enums", JsonArray(it)) }
        min?.let { put("min", it) }
        max?.let { put("max", it) }
        regex?.let { put("regex", it) }
        index?.let { put("index", it) }
    }
}

/**
 * Fluent builder for collection schemas.
 *
 * Example:
 * ```kotlin
 * val schema = SchemaBuilder()
 *     .addField("name", FieldTypeSchemaBuilder("string").required().build())
 *     .addField("email", FieldTypeSchemaBuilder("string").unique().build())
 *     .build()
 *
 * client.createCollection("users", schema)
 * ```
 */
class SchemaBuilder {
    private val fields = mutableMapOf<String, JsonObject>()
    private var bypassRipple = true
    private var version = 1

    /** Add a field definition from a pre-built JsonObject. */
    fun addField(name: String, fieldSchema: JsonObject) = apply {
        fields[name] = fieldSchema
    }

    /** Add a field definition from a builder (auto-built). */
    fun addField(name: String, builder: FieldTypeSchemaBuilder) = apply {
        fields[name] = builder.build()
    }

    /** Set whether to bypass ripple on collection creation. */
    fun bypassRipple(bypass: Boolean = true) = apply { this.bypassRipple = bypass }

    /** Set the schema version. */
    fun version(version: Int) = apply { this.version = version }

    /** Build the final schema as a JsonObject. */
    fun build(): JsonObject = buildJsonObject {
        put("fields", buildJsonObject {
            fields.forEach { (name, schema) -> put(name, schema) }
        })
        put("version", version)
        put("bypass_ripple", bypassRipple)
    }
}
