package io.ekodb.client.functions

import kotlinx.serialization.SerialName
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
    val transaction_config: TransactionConfig? = null,
    val http_method: String? = null,
    val http_path: String? = null,
    val id: String? = null,
    val created_at: String? = null,
    val updated_at: String? = null,
    /**
     * The key (and the subject its token named) that saved the function.
     * Server-managed and read-only: the server sets it on create, keeps it
     * across updates and replaces any value sent. Null for a function saved
     * before the server recorded authors.
     */
    val created_by: Principal? = null,
    /**
     * Whose grants every step of the function runs with: [RunAs.Caller] (the
     * calling key's grants) or [RunAs.Author] (the recorded author's). Leave
     * unset to let the server apply its default; a server reports null for a
     * function saved before the field existed.
     */
    val run_as: RunAs? = null
)

/** Whose grants the steps of a stored function run with. */
@Serializable
enum class RunAs {
    @SerialName("caller")
    Caller,

    @SerialName("author")
    Author
}

/**
 * The key, and the subject its token named, that created a function or a
 * schedule. Read-only: the server records it.
 */
@Serializable
data class Principal(
    /** The key's non-secret id, never the key itself. */
    val key_id: String,
    /** The key's label, when it was registered with one. */
    val key_label: String? = null,
    /** The subject the token was minted for, when it named one. */
    val subject: String? = null
)

@Serializable
data class TransactionConfig(
    val enabled: Boolean,
    val auto_rollback: Boolean,
    val isolation_level: String? = null,
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
