package io.ekodb.client.types

import kotlinx.serialization.Serializable
import kotlinx.serialization.json.JsonElement

/**
 * Query operators for filtering
 */
@Serializable
sealed class QueryOperator {
    @Serializable data class Eq(val value: FieldType) : QueryOperator()
    @Serializable data class Ne(val value: FieldType) : QueryOperator()
    @Serializable data class Gt(val value: FieldType) : QueryOperator()
    @Serializable data class Gte(val value: FieldType) : QueryOperator()
    @Serializable data class Lt(val value: FieldType) : QueryOperator()
    @Serializable data class Lte(val value: FieldType) : QueryOperator()
    @Serializable data class In(val values: List<FieldType>) : QueryOperator()
    @Serializable data class NotIn(val values: List<FieldType>) : QueryOperator()
    @Serializable data class Regex(val pattern: String) : QueryOperator()
    @Serializable data class Exists(val exists: Boolean) : QueryOperator()
}

/**
 * Query for finding records
 */
@Serializable
data class Query(
    val filter: JsonElement? = null,
    val sort: JsonElement? = null,
    val limit: Int? = null,
    val skip: Int? = null,
    val join: JsonElement? = null,
    val bypassCache: Boolean? = null,
    val bypassRipple: Boolean? = null
) {
    companion object {
        fun new() = Query()
    }
    
    fun withFilter(filter: JsonElement) = copy(filter = filter)
    fun withSort(sort: JsonElement) = copy(sort = sort)
    fun withLimit(limit: Int) = copy(limit = limit)
    fun withSkip(skip: Int) = copy(skip = skip)
    fun withJoin(join: JsonElement) = copy(join = join)
    fun withBypassCache(bypass: Boolean) = copy(bypassCache = bypass)
    fun withBypassRipple(bypass: Boolean) = copy(bypassRipple = bypass)
}
