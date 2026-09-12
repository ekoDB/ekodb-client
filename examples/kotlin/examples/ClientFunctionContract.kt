/** Offline contract smoke test for stored-function stage serialization. */
package io.ekodb.client.examples

import io.ekodb.client.functions.FunctionCondition
import io.ekodb.client.functions.FunctionStageConfig
import io.ekodb.client.functions.GroupFunctionConfig
import io.ekodb.client.functions.GroupFunctionOp
import io.ekodb.client.functions.QueryConditionOperator
import io.ekodb.client.functions.TransactionConfig
import io.ekodb.client.functions.UserFunction
import io.ekodb.client.functions.queryCondition
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.JsonObject
import kotlinx.serialization.json.JsonPrimitive
import kotlinx.serialization.json.buildJsonObject
import kotlinx.serialization.json.jsonArray
import kotlinx.serialization.json.jsonObject
import kotlinx.serialization.json.jsonPrimitive
import kotlinx.serialization.json.put

private fun requireStage(stages: List<JsonObject>, type: String): JsonObject =
    stages.firstOrNull { it["type"]?.jsonPrimitive?.content == type }
        ?: error("missing $type stage")

fun main() {
    val filter = queryCondition("status", QueryConditionOperator.Eq, JsonPrimitive("active"))
    val function = UserFunction(
        label = "client_function_contract",
        name = "Client function contract",
        functions = listOf(
            FunctionStageConfig.Query("items", filter = filter, limit = 5),
            FunctionStageConfig.Update(
                "items",
                filter,
                buildJsonObject { put("status", "ready") },
            ),
            FunctionStageConfig.Delete("items", filter),
            FunctionStageConfig.Group(
                by_fields = listOf("category"),
                functions = listOf(
                    GroupFunctionConfig("labels", GroupFunctionOp.AddToSet, "label"),
                    GroupFunctionConfig("score_stddev", GroupFunctionOp.StandardDeviation, "score"),
                    GroupFunctionConfig("unique_users", GroupFunctionOp.ApproxDistinct, "user_id"),
                ),
            ),
            FunctionStageConfig.If(
                FunctionCondition.FieldGreaterThanOrEqual("stock", JsonPrimitive(1)),
                listOf(FunctionStageConfig.SetField("available", JsonPrimitive(true))),
            ),
            FunctionStageConfig.Upsert(
                "items",
                "sku",
                JsonPrimitive("sku-1"),
                buildJsonObject { put("name", "Widget") },
            ),
            FunctionStageConfig.Increment("items", "item-1", "views", JsonPrimitive(2)),
            FunctionStageConfig.Push("items", "item-1", "tags", JsonPrimitive("new")),
            FunctionStageConfig.SetField("source", JsonPrimitive("contract")),
            FunctionStageConfig.AddFields(
                listOf(
                    buildJsonObject {
                        put("field_name", "total")
                        put(
                            "expression",
                            buildJsonObject {
                                put("type", "Field")
                                put("name", "price")
                            },
                        )
                    },
                ),
            ),
            FunctionStageConfig.CurrentDatetime("processed_at"),
            FunctionStageConfig.VectorSearch("items", listOf(0.1, 0.2), 3, 0.8),
            FunctionStageConfig.HybridSearch("items", "widget", listOf(0.1, 0.2), 3),
            FunctionStageConfig.FindOneAndUpdate(
                "items",
                "item-1",
                buildJsonObject { put("status", "claimed") },
                bypass_ripple = true,
                ttl = 60,
            ),
            FunctionStageConfig.UpdateWithAction(
                "items",
                "item-1",
                "increment",
                "views",
                JsonPrimitive(1),
                bypass_ripple = true,
            ),
            FunctionStageConfig.BatchDelete("items", listOf("item-1", "item-2"), true),
            FunctionStageConfig.Embed("body", "embedding", JsonPrimitive("text-embedding-3-small")),
            FunctionStageConfig.HttpRequest(
                "https://example.com/items",
                timeout_seconds = 10,
                output_field = "response",
            ),
        ),
        transaction_config = TransactionConfig(
            enabled = true,
            auto_rollback = true,
            isolation_level = "Serializable",
        ),
    )

    val encoded = Json.encodeToJsonElement(UserFunction.serializer(), function).jsonObject
    val stages = encoded.getValue("functions").jsonArray.map { it.jsonObject }
    check(
        encoded.getValue("transaction_config").jsonObject["isolation_level"]
            ?.jsonPrimitive?.content == "Serializable",
    )
    listOf("Query", "Update", "Delete").forEach { type ->
        check(requireStage(stages, type).getValue("filter").jsonObject["type"]?.jsonPrimitive?.content == "Condition")
    }
    check(
        requireStage(stages, "Group").getValue("functions").jsonArray[2]
            .jsonObject["operation"]?.jsonPrimitive?.content == "ApproxDistinct",
    )
    check(
        requireStage(stages, "If").getValue("condition").jsonObject["type"]
            ?.jsonPrimitive?.content == "FieldGreaterThanOrEqual",
    )
    check(requireStage(stages, "BatchDelete").getValue("record_ids").jsonArray[1].jsonPrimitive.content == "item-2")
    check(requireStage(stages, "Embed")["input_field"]?.jsonPrimitive?.content == "body")
    check(requireStage(stages, "HttpRequest")["timeout_seconds"]?.jsonPrimitive?.content == "10")
    check(requireStage(stages, "HttpRequest")["output_field"]?.jsonPrimitive?.content == "response")
    listOf(
        "Upsert",
        "Increment",
        "Push",
        "SetField",
        "AddFields",
        "CurrentDatetime",
        "VectorSearch",
        "HybridSearch",
        "FindOneAndUpdate",
        "UpdateWithAction",
    ).forEach { requireStage(stages, it) }

    println("client_function_contract: ok")
}
