package io.ekodb.client

import io.ekodb.client.types.FieldType
import io.ekodb.client.types.Query
import kotlinx.serialization.json.*

/**
 * Sort order for query results
 */
enum class SortOrder {
    ASC, DESC
}

/**
 * Fluent API for building complex queries
 */
class QueryBuilder {
    private val filters = mutableListOf<JsonElement>()
    private val sorts = mutableListOf<Pair<String, Boolean>>()
    private var limitValue: Int? = null
    private var skipValue: Int? = null
    private var joinValue: JsonElement? = null
    private var bypassCacheValue: Boolean? = null
    private var bypassRippleValue: Boolean? = null
    
    /**
     * Equal to
     */
    fun eq(field: String, value: Any) = apply {
        filters.add(buildJsonObject {
            put("type", "Condition")
            put("content", buildJsonObject {
                put("field", field)
                put("operator", "Eq")
                put("value", convertToJsonElement(value))
            })
        })
    }
    
    /**
     * Not equal to
     */
    fun ne(field: String, value: Any) = apply {
        filters.add(buildJsonObject {
            put("type", "Condition")
            put("content", buildJsonObject {
                put("field", field)
                put("operator", "Ne")
                put("value", convertToJsonElement(value))
            })
        })
    }
    
    /**
     * Greater than
     */
    fun gt(field: String, value: Any) = apply {
        filters.add(buildJsonObject {
            put("type", "Condition")
            put("content", buildJsonObject {
                put("field", field)
                put("operator", "Gt")
                put("value", convertToJsonElement(value))
            })
        })
    }
    
    /**
     * Greater than or equal
     */
    fun gte(field: String, value: Any) = apply {
        filters.add(buildJsonObject {
            put("type", "Condition")
            put("content", buildJsonObject {
                put("field", field)
                put("operator", "Gte")
                put("value", convertToJsonElement(value))
            })
        })
    }
    
    /**
     * Less than
     */
    fun lt(field: String, value: Any) = apply {
        filters.add(buildJsonObject {
            put("type", "Condition")
            put("content", buildJsonObject {
                put("field", field)
                put("operator", "Lt")
                put("value", convertToJsonElement(value))
            })
        })
    }
    
    /**
     * Less than or equal
     */
    fun lte(field: String, value: Any) = apply {
        filters.add(buildJsonObject {
            put("type", "Condition")
            put("content", buildJsonObject {
                put("field", field)
                put("operator", "Lte")
                put("value", convertToJsonElement(value))
            })
        })
    }
    
    /**
     * In array
     */
    fun inArray(field: String, values: List<Any>) = apply {
        filters.add(buildJsonObject {
            put("type", "Condition")
            put("content", buildJsonObject {
                put("field", field)
                put("operator", "In")
                put("value", JsonArray(values.map { convertToJsonElement(it) }))
            })
        })
    }
    
    /**
     * Not in array
     */
    fun nin(field: String, values: List<Any>) = apply {
        filters.add(buildJsonObject {
            put("type", "Condition")
            put("content", buildJsonObject {
                put("field", field)
                put("operator", "NotIn")
                put("value", JsonArray(values.map { convertToJsonElement(it) }))
            })
        })
    }
    
    /**
     * Regex match
     */
    fun regex(field: String, pattern: String) = apply {
        filters.add(buildJsonObject {
            put("type", "Condition")
            put("content", buildJsonObject {
                put("field", field)
                put("operator", "Regex")
                put("value", JsonPrimitive(pattern))
            })
        })
    }
    
    /**
     * Contains substring
     */
    fun contains(field: String, substring: String) = apply {
        filters.add(buildJsonObject {
            put("type", "Condition")
            put("content", buildJsonObject {
                put("field", field)
                put("operator", "Contains")
                put("value", JsonPrimitive(substring))
            })
        })
    }
    
    /**
     * Sort ascending
     */
    fun sortAsc(field: String) = apply {
        sorts.add(field to true)
    }
    
    /**
     * Sort ascending (alias)
     */
    fun sortAscending(field: String) = sortAsc(field)
    
    /**
     * Sort descending
     */
    fun sortDesc(field: String) = apply {
        sorts.add(field to false)
    }
    
    /**
     * Sort descending (alias)
     */
    fun sortDescending(field: String) = sortDesc(field)
    
    /**
     * Limit number of results
     */
    fun limit(limit: Int) = apply {
        limitValue = limit
    }
    
    /**
     * Skip number of results for pagination
     */
    fun skip(skip: Int) = apply {
        skipValue = skip
    }
    
    /**
     * Bypass cache
     */
    fun bypassCache(bypass: Boolean = true) = apply {
        bypassCacheValue = bypass
    }
    
    /**
     * Bypass ripple
     */
    fun bypassRipple(bypass: Boolean = true) = apply {
        bypassRippleValue = bypass
    }
    
    /**
     * Add join configuration
     * @param joinConfig Map containing join configuration with keys:
     *   - collection: String - target collection to join
     *   - local_field: String - field in current collection
     *   - foreign_field: String - field in target collection
     *   - as: String - alias for joined data
     */
    fun join(joinConfig: Map<String, Any>) = apply {
        joinValue = convertMapToJsonElement(joinConfig)
    }
    
    /**
     * Add a simple join with another collection
     * @param collection Target collection to join
     * @param localField Field in current collection
     * @param foreignField Field in target collection
     * @param alias Alias for joined data (default: collection name)
     */
    fun joinWith(
        collection: String,
        localField: String,
        foreignField: String,
        alias: String = collection
    ) = apply {
        joinValue = buildJsonObject {
            put("collection", collection)
            put("local_field", localField)
            put("foreign_field", foreignField)
            put("as", alias)
        }
    }
    
    /**
     * Build the query
     */
    fun build(): Query {
        // Combine filters with AND logic if multiple filters exist
        val filterJson = when {
            filters.isEmpty() -> null
            filters.size == 1 -> filters[0]
            else -> buildJsonObject {
                put("type", "Logical")
                put("content", buildJsonObject {
                    put("operator", "And")
                    put("expressions", JsonArray(filters))
                })
            }
        }
        
        // Build sort as array of {field, ascending} objects
        val sortJson = if (sorts.isNotEmpty()) {
            JsonArray(sorts.map { (field, ascending) ->
                buildJsonObject {
                    put("field", field)
                    put("ascending", ascending)
                }
            })
        } else null
        
        return Query(
            filter = filterJson,
            sort = sortJson,
            limit = limitValue,
            skip = skipValue,
            join = joinValue,
            bypassCache = bypassCacheValue,
            bypassRipple = bypassRippleValue
        )
    }
    
    private fun convertToJsonElement(value: Any): JsonElement = when (value) {
        is String -> JsonPrimitive(value)
        is Number -> JsonPrimitive(value)
        is Boolean -> JsonPrimitive(value)
        is JsonElement -> value
        else -> JsonPrimitive(value.toString())
    }
    
    private fun convertMapToJsonElement(map: Map<String, Any>): JsonElement {
        return buildJsonObject {
            map.forEach { (key, value) ->
                when (value) {
                    is String -> put(key, value)
                    is Number -> put(key, value)
                    is Boolean -> put(key, value)
                    is Map<*, *> -> put(key, convertMapToJsonElement(value as Map<String, Any>))
                    is List<*> -> put(key, JsonArray(value.map { 
                        when (it) {
                            is Map<*, *> -> convertMapToJsonElement(it as Map<String, Any>)
                            else -> convertToJsonElement(it!!)
                        }
                    }))
                    is JsonElement -> put(key, value)
                    else -> put(key, value.toString())
                }
            }
        }
    }
    
    companion object {
        fun new() = QueryBuilder()
    }
}
