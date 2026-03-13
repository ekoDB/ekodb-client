package io.ekodb.client

import kotlinx.serialization.json.*

/**
 * Configuration for joining data across collections.
 *
 * Example:
 * ```kotlin
 * val join = JoinConfig.single("users", "user_id", "id", "user")
 *
 * val query = QueryBuilder()
 *     .eq("status", "active")
 *     .join(join.toJsonObject())
 *     .build()
 * ```
 */
data class JoinConfig(
    val collections: List<String>,
    val localField: String,
    val foreignField: String,
    val asField: String
) {
    /** Convert to JsonObject for use in queries. */
    fun toJsonObject(): JsonObject = buildJsonObject {
        put("collections", JsonArray(collections.map { JsonPrimitive(it) }))
        put("local_field", localField)
        put("foreign_field", foreignField)
        put("as_field", asField)
    }

    /** Convert to a Map for use in queries. */
    fun toMap(): Map<String, Any> = mapOf(
        "collections" to collections,
        "local_field" to localField,
        "foreign_field" to foreignField,
        "as_field" to asField
    )

    companion object {
        /** Create a join with a single collection. */
        fun single(
            collection: String,
            localField: String,
            foreignField: String,
            asField: String
        ) = JoinConfig(listOf(collection), localField, foreignField, asField)

        /** Create a join with multiple collections. */
        fun multi(
            collections: List<String>,
            localField: String,
            foreignField: String,
            asField: String
        ) = JoinConfig(collections, localField, foreignField, asField)
    }
}
