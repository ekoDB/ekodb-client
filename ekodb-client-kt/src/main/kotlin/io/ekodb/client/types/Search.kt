package io.ekodb.client.types

import kotlinx.serialization.EncodeDefault
import kotlinx.serialization.ExperimentalSerializationApi
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.JsonElement
import kotlinx.serialization.json.JsonObject

/** The distance metrics supported by the search protocol. */
@Serializable
enum class DistanceMetric {
    @SerialName("cosine")
    COSINE,

    @SerialName("euclidean")
    EUCLIDEAN,

    @SerialName("dotproduct")
    DOT_PRODUCT
}

/**
 * Text, vector, or hybrid search request. Unset options are omitted by the client;
 * no server ranking defaults are imposed. An empty [query] supports vector-only search.
 *
 * [fields] is comma-separated and [weights] uses `field:weight,field2:weight2`,
 * matching Rust and the TypeScript builder. SearchQueryBuilder accepts lists/maps.
 * [filters] reuses [Query.filter], the canonical QueryBuilder expression, not a
 * whole find query. Language and field names are open strings.
 */
@OptIn(ExperimentalSerializationApi::class)
@Serializable
data class SearchQuery(
    /** Search text; use an empty string for a pure vector request. */
    @EncodeDefault
    val query: String = "",
    /** Stemming language, e.g. "english"; intentionally an open string. */
    val language: String? = null,
    /** Whether matching is case-sensitive. */
    @SerialName("case_sensitive")
    val caseSensitive: Boolean? = null,
    /** Whether fuzzy matching is enabled. */
    val fuzzy: Boolean? = null,
    /** Minimum result score requested from the server. */
    @SerialName("min_score")
    val minScore: Double? = null,
    /** Comma-separated searchable field names; the builder also accepts a list. */
    val fields: String? = null,
    /** Field boosts in `field:weight` format; the builder also accepts a numeric map. */
    val weights: String? = null,
    /** Whether stemming is enabled. */
    @SerialName("enable_stemming")
    val enableStemming: Boolean? = null,
    /** Whether exact matches receive a boost. */
    @SerialName("boost_exact")
    val boostExact: Boolean? = null,
    /** Maximum fuzzy edit distance (sibling documentation specifies 0–5). */
    @SerialName("max_edit_distance")
    val maxEditDistance: Int? = null,
    /** Request ripple-cache bypass. */
    @SerialName("bypass_ripple")
    val bypassRipple: Boolean? = null,
    /** Request cache bypass. */
    @SerialName("bypass_cache")
    val bypassCache: Boolean? = null,
    /** Maximum number of returned results. */
    val limit: Int? = null,
    /** Caller-supplied query embedding; this client does not generate it automatically. */
    val vector: List<Double>? = null,
    /** Vector field to search; omitted by default (siblings document "embedding"). */
    @SerialName("vector_field")
    val vectorField: String? = null,
    /** Requested distance metric, omitted unless selected. */
    @SerialName("vector_metric")
    val vectorMetric: DistanceMetric? = null,
    /** Number of nearest vector candidates requested. */
    @SerialName("vector_k")
    val vectorK: Int? = null,
    /** Vector similarity threshold; interpretation belongs to the server. */
    @SerialName("vector_threshold")
    val vectorThreshold: Double? = null,
    /** Text contribution to hybrid ranking; no helper default is imposed. */
    @SerialName("text_weight")
    val textWeight: Double? = null,
    /** Vector contribution to hybrid ranking; no helper default is imposed. */
    @SerialName("vector_weight")
    val vectorWeight: Double? = null,
    /** Requested included document fields. */
    @SerialName("select_fields")
    val selectFields: List<String>? = null,
    /** Requested excluded document fields. */
    @SerialName("exclude_fields")
    val excludeFields: List<String>? = null,
    /** Canonical prefilter expression from QueryBuilder, shared with Query.filter. */
    val filters: JsonElement? = null,
)

/** A ranked hit. The dynamic record retains all document fields, including `_score`. */
@Serializable
data class SearchResult(
    val record: JsonObject,
    val score: Double,
    @SerialName("matched_fields")
    val matchedFields: List<String>,
)

/** Search metadata; timing is optional, as in TypeScript, for older responses. */
@Serializable
data class SearchResponse(
    val results: List<SearchResult>,
    val total: Long,
    @SerialName("execution_time_ms")
    val executionTimeMs: Long? = null,
)
