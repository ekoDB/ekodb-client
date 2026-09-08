package io.ekodb.client

import io.ekodb.client.types.DistanceMetric
import io.ekodb.client.types.SearchQuery
import kotlinx.serialization.json.JsonElement

/** Fluent search construction; later builder changes do not affect earlier [build] results. */
class SearchQueryBuilder(query: String = "") {
    private var value = SearchQuery(query)

    fun language(language: String) = apply { value = value.copy(language = language) }
    fun caseSensitive(caseSensitive: Boolean) = apply { value = value.copy(caseSensitive = caseSensitive) }
    fun fuzzy(fuzzy: Boolean) = apply { value = value.copy(fuzzy = fuzzy) }
    fun minScore(minScore: Double) = apply { value = value.copy(minScore = minScore) }
    fun fields(fields: String) = apply { value = value.copy(fields = fields) }
    fun weights(weights: String) = apply { value = value.copy(weights = weights) }
    fun enableStemming(enableStemming: Boolean) = apply { value = value.copy(enableStemming = enableStemming) }
    fun boostExact(boostExact: Boolean) = apply { value = value.copy(boostExact = boostExact) }
    fun maxEditDistance(maxEditDistance: Int) = apply { value = value.copy(maxEditDistance = maxEditDistance) }
    fun bypassRipple(bypassRipple: Boolean) = apply { value = value.copy(bypassRipple = bypassRipple) }
    fun bypassCache(bypassCache: Boolean) = apply { value = value.copy(bypassCache = bypassCache) }
    fun limit(limit: Int) = apply { value = value.copy(limit = limit) }
    fun vector(vector: List<Double>) = apply { value = value.copy(vector = vector.toList()) }
    fun vectorField(vectorField: String) = apply { value = value.copy(vectorField = vectorField) }
    fun vectorMetric(vectorMetric: DistanceMetric) = apply { value = value.copy(vectorMetric = vectorMetric) }
    fun vectorK(vectorK: Int) = apply { value = value.copy(vectorK = vectorK) }
    fun vectorThreshold(vectorThreshold: Double) = apply { value = value.copy(vectorThreshold = vectorThreshold) }
    fun textWeight(textWeight: Double) = apply { value = value.copy(textWeight = textWeight) }
    fun vectorWeight(vectorWeight: Double) = apply { value = value.copy(vectorWeight = vectorWeight) }
    fun selectFields(selectFields: List<String>) = apply { value = value.copy(selectFields = selectFields.toList()) }
    fun excludeFields(excludeFields: List<String>) = apply { value = value.copy(excludeFields = excludeFields.toList()) }
    fun filters(filters: JsonElement) = apply { value = value.copy(filters = filters) }

    /** Normalize field names to the canonical comma-separated wire representation. */
    fun fields(fields: List<String>) = fields(fields.joinToString(","))

    /** Normalize numeric field weights to the canonical Rust/TypeScript wire string. */
    fun weights(weights: Map<String, Double>) = weights(
        weights.entries.joinToString(",") { (field, weight) -> "$field:${weight.toString().removeSuffix(".0")}" }
    )

    /** Build only a prefilter using the same representation as ordinary find queries. */
    fun filters(block: QueryBuilder.() -> Unit) = apply {
        value = value.copy(filters = QueryBuilder().apply(block).build().filter)
    }

    fun build(): SearchQuery = value
}
