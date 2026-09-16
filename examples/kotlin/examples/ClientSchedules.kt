package io.ekodb.client.examples

import io.ekodb.client.EkoDBClient
import io.ekodb.client.functions.FunctionStageConfig
import io.ekodb.client.functions.UserFunction
import io.github.cdimascio.dotenv.dotenv
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.buildJsonObject
import kotlinx.serialization.json.boolean
import kotlinx.serialization.json.jsonArray
import kotlinx.serialization.json.jsonObject
import kotlinx.serialization.json.jsonPrimitive
import kotlinx.serialization.json.put

/**
 * Schedule management example.
 *
 * Exercises create -> list -> get -> update -> trigger -> pause -> resume -> delete.
 * A temporary no-op function makes the trigger step self-contained.
 */
fun main() = runBlocking {
    val dotenv = dotenv()
    val baseUrl = dotenv["API_BASE_URL"] ?: "http://localhost:8080"
    val apiKey = dotenv["API_BASE_KEY"] ?: "a-test-api-key-from-ekodb"

    val client = EkoDBClient.builder()
        .baseUrl(baseUrl)
        .apiKey(apiKey)
        .build()

    println("=== ekoDB Kotlin Client - Schedules Example ===\n")

    var functionId: String? = null
    var schedId: String? = null
    var primaryError: Throwable? = null
    try {
        val functionLabel = "schedule_noop_kotlin_${ProcessHandle.current().pid()}_${System.currentTimeMillis()}"
        functionId = client.saveFunction(UserFunction(
            label = functionLabel,
            name = "Schedule example no-op",
            parameters = emptyMap(),
            functions = listOf(
                FunctionStageConfig.Return(
                    fields = mapOf("triggered" to kotlinx.serialization.json.JsonPrimitive(true)),
                    status_code = 200
                )
            )
        ))

        // 1. Create a schedule
        println("--- Creating schedule ---")
        val schedule = client.createSchedule(buildJsonObject {
            put("name", "Hourly Health Check")
            put("function_label", functionLabel)
            put("cron_expression", "0 0 * * * *")
            put("description", "Ping all services every hour")
            put("enabled", true)
        })
        schedId = requireNotNull(schedule["id"]) { "Created schedule omitted id" }.jsonPrimitive.content
        check(schedule["name"]?.jsonPrimitive?.content == "Hourly Health Check") {
            "Unexpected created schedule name: ${schedule["name"]}"
        }
        check(schedule["cron_expression"]?.jsonPrimitive?.content == "0 0 * * * *") {
            "Unexpected created schedule cron: ${schedule["cron_expression"]}"
        }
        check(schedule["enabled"]?.jsonPrimitive?.boolean == true) {
            "Created schedule was not enabled: ${schedule["enabled"]}"
        }
        println("Created schedule: $schedId — ${schedule["name"]}")

        // 2. List all schedules
        println("\n--- Listing schedules ---")
        val list = client.listSchedules()
        println("Schedules: $list")
        val listedIds = requireNotNull(list["schedules"]) {
            "Schedule list response omitted schedules"
        }.jsonArray.map { item ->
            requireNotNull(item.jsonObject["id"]) { "Listed schedule omitted id" }.jsonPrimitive.content
        }
        check(schedId in listedIds) { "Created schedule $schedId was absent from listSchedules" }

        // 3. Get schedule by ID
        println("\n--- Getting schedule ---")
        val fetched = client.getSchedule(schedId)
        println("Fetched: ${fetched["name"]} (cron: ${fetched["cron_expression"]})")
        check(fetched["id"]?.jsonPrimitive?.content == schedId) {
            "Fetched schedule had unexpected id: ${fetched["id"]}"
        }
        check(fetched["cron_expression"]?.jsonPrimitive?.content == "0 0 * * * *") {
            "Fetched schedule had unexpected cron: ${fetched["cron_expression"]}"
        }

        // 4. Update schedule
        println("\n--- Updating schedule ---")
        val updated = client.updateSchedule(schedId, buildJsonObject {
            put("cron_expression", "0 */30 * * * *")
            put("description", "Ping all services every 30 minutes")
        })
        println("Updated cron: ${updated["cron_expression"]}")
        check(updated["cron_expression"]?.jsonPrimitive?.content == "0 */30 * * * *") {
            "Schedule update did not persist cron: ${updated["cron_expression"]}"
        }

        // 5. Trigger immediately
        println("\n--- Triggering schedule ---")
        val triggered = client.triggerSchedule(schedId)
        println("Trigger response: $triggered")
        check(triggered["schedule_id"]?.jsonPrimitive?.content == schedId) {
            "Trigger response had unexpected schedule id: ${triggered["schedule_id"]}"
        }

        // 6. Pause schedule
        println("\n--- Pausing schedule ---")
        val paused = client.pauseSchedule(schedId)
        println("Enabled after pause: ${paused["enabled"]}")
        check(paused["enabled"]?.jsonPrimitive?.boolean == false) {
            "Pause left schedule enabled: ${paused["enabled"]}"
        }

        // 7. Resume schedule
        println("\n--- Resuming schedule ---")
        val resumed = client.resumeSchedule(schedId)
        println("Enabled after resume: ${resumed["enabled"]}")
        check(resumed["enabled"]?.jsonPrimitive?.boolean == true) {
            "Resume left schedule disabled: ${resumed["enabled"]}"
        }

        // 8. Delete schedule
        println("\n--- Deleting schedule ---")
    } catch (error: Throwable) {
        primaryError = error
        throw error
    } finally {
        val cleanupErrors = mutableListOf<Throwable>()
        schedId?.let { id ->
            try {
                client.deleteSchedule(id)
                println("Schedule deleted successfully")
            } catch (error: Throwable) {
                cleanupErrors.add(error)
            }
        }
        functionId?.let { id ->
            try {
                client.deleteFunction(id)
            } catch (error: Throwable) {
                cleanupErrors.add(error)
            }
        }
        try {
            client.close()
        } catch (error: Throwable) {
            cleanupErrors.add(error)
        }

        val failure = primaryError
        if (failure != null) {
            cleanupErrors.forEach(failure::addSuppressed)
        } else if (cleanupErrors.isNotEmpty()) {
            val cleanupError = cleanupErrors.first()
            cleanupErrors.drop(1).forEach(cleanupError::addSuppressed)
            throw cleanupError
        } else {
            println("\n=== Example Complete ===")
        }
    }
}
