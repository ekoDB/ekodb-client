package io.ekodb.client.examples

import io.ekodb.client.EkoDBClient
import io.github.cdimascio.dotenv.dotenv
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.buildJsonObject
import kotlinx.serialization.json.put

/**
 * Schedule management example.
 *
 * Exercises create -> list -> get -> update -> pause -> resume -> delete.
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

    try {
        // 1. Create a schedule
        println("--- Creating schedule ---")
        val schedule = client.createSchedule(buildJsonObject {
            put("name", "Hourly Health Check")
            put("cron", "0 * * * *")
            put("description", "Ping all services every hour")
        })
        val schedId = schedule["id"].toString().trim('"')
        println("Created schedule: $schedId — ${schedule["name"]}")

        // 2. List all schedules
        println("\n--- Listing schedules ---")
        val list = client.listSchedules()
        println("Schedules: $list")

        // 3. Get schedule by ID
        println("\n--- Getting schedule ---")
        val fetched = client.getSchedule(schedId)
        println("Fetched: ${fetched["name"]} (cron: ${fetched["cron"]})")

        // 4. Update schedule
        println("\n--- Updating schedule ---")
        val updated = client.updateSchedule(schedId, buildJsonObject {
            put("cron", "*/30 * * * *")
            put("description", "Ping all services every 30 minutes")
        })
        println("Updated cron: ${updated["cron"]}")

        // 5. Pause schedule
        println("\n--- Pausing schedule ---")
        val paused = client.pauseSchedule(schedId)
        println("Status after pause: ${paused["status"]}")

        // 6. Resume schedule
        println("\n--- Resuming schedule ---")
        val resumed = client.resumeSchedule(schedId)
        println("Status after resume: ${resumed["status"]}")

        // 7. Delete schedule
        println("\n--- Deleting schedule ---")
        client.deleteSchedule(schedId)
        println("Schedule deleted successfully")

    } finally {
        client.close()
        println("\n=== Example Complete ===")
    }
}
