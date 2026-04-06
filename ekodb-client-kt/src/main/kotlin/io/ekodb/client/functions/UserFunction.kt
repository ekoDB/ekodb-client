package io.ekodb.client.functions

import kotlinx.datetime.Instant
import kotlinx.serialization.Serializable

/**
 * A reusable sequence of Functions stored in ekoDB.
 * Called by label via the call_function chat tool or REST API.
 */
@Serializable
data class UserFunction(
    val label: String,
    val name: String,
    val description: String? = null,
    val version: String = "1.0",
    val parameters: Map<String, ParameterDefinition> = emptyMap(),
    val functions: List<FunctionStageConfig>,
    val tags: List<String> = emptyList(),
    val id: String? = null,
    val created_at: String? = null,
    val updated_at: String? = null
)


/**
 * Parameter definition for a function
 */
@Serializable
data class ParameterDefinition(
    val required: Boolean = false,
    val default: kotlinx.serialization.json.JsonElement? = null,
    val description: String? = null,
    @kotlinx.serialization.SerialName("param_type")
    val paramType: String? = null
)

// ParameterValue removed - use direct values or string interpolation "{{param}}" instead

/**
 * Function execution result
 */
@Serializable
data class FunctionResult(
    val records: List<kotlinx.serialization.json.JsonObject>,
    val stats: FunctionStats
)

/**
 * Function execution statistics
 */
@Serializable
data class FunctionStats(
    val input_count: Int,
    val output_count: Int,
    val execution_time_ms: Long,
    val stages_executed: Int,
    val stage_stats: List<StageStats>
)

/**
 * Statistics for a single stage
 */
@Serializable
data class StageStats(
    val stage: String,
    val input_count: Int,
    val output_count: Int,
    val execution_time_ms: Long
)
