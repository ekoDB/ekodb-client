package io.ekodb.client.functions

import kotlinx.datetime.Instant
import kotlinx.serialization.Serializable

/**
 * Saved function definition with pipeline stages and parameters
 */
@Serializable
data class SavedFunction(
    val label: String,
    val name: String,
    val description: String? = null,
    val version: String = "1.0",
    val parameters: Map<String, ParameterDefinition> = emptyMap(),
    val pipeline: List<FunctionStageConfig>,
    val tags: List<String> = emptyList(),
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
    val description: String? = null
)

/**
 * Parameter value - literal or parameter reference
 */
@Serializable
data class ParameterValue(
    val type: String, // "Literal" or "Parameter"
    val value: kotlinx.serialization.json.JsonElement
) {
    companion object {
        fun literal(value: kotlinx.serialization.json.JsonElement) = 
            ParameterValue("Literal", value)
        
        fun parameter(name: String) = 
            ParameterValue("Parameter", kotlinx.serialization.json.JsonPrimitive(name))
    }
}

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
