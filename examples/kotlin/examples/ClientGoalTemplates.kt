// ekoDB Kotlin Client - Goal Template CRUD Example
//
// Demonstrates creating, listing, getting, updating, and deleting goal templates.

package io.ekodb.client.examples

import io.ekodb.client.EkoDBClient
import io.ekodb.client.getStringValue
import kotlinx.serialization.json.jsonPrimitive
import kotlinx.serialization.json.buildJsonObject
import kotlinx.serialization.json.put
import kotlinx.serialization.json.putJsonArray
import kotlinx.coroutines.runBlocking

fun main() = runBlocking {
    println("=== ekoDB Goal Template CRUD Example (Kotlin) ===\n")

    val baseUrl = System.getenv("API_BASE_URL") ?: "http://localhost:8080"
    val apiKey = System.getenv("API_BASE_KEY")
        ?: throw IllegalStateException("API_BASE_KEY environment variable is required")

    val client = EkoDBClient.builder()
        .baseUrl(baseUrl)
        .apiKey(apiKey)
        .build()

    var templateId: String? = null
    var deleted = false
    try {
        // 1. Create a goal template
        println("--- Creating goal template ---")
        val template = client.goalTemplateCreate(buildJsonObject {
            put("title", "Data Migration")
            put("description", "Template for migrating data between schemas")
            putJsonArray("steps") {
                add(buildJsonObject { put("description", "Analyze source schema") })
                add(buildJsonObject { put("description", "Create target schema") })
                add(buildJsonObject { put("description", "Migrate records") })
                add(buildJsonObject { put("description", "Validate results") })
            }
        })
        val createdId = requireNotNull(template["id"]).jsonPrimitive.content
        templateId = createdId
        val created = client.goalTemplateGet(createdId)
        val templateTitle = requireNotNull(getStringValue(created["title"]))
        require(templateTitle == "Data Migration") {
            "Unexpected created template title: $templateTitle"
        }
        println("Created template: $templateTitle (id: $templateId)")

        // 2. List all templates
        println("\n--- Listing templates ---")
        val list = client.goalTemplateList()
        println("Templates: $list")

        // 3. Get template by ID
        println("\n--- Getting template ---")
        val fetched = client.goalTemplateGet(createdId)
        println("Fetched: ${fetched["title"]}")

        // 4. Update template
        println("\n--- Updating template ---")
        val updated = client.goalTemplateUpdate(createdId, buildJsonObject {
            put("description", "Updated: comprehensive data migration workflow")
        })
        println("Updated description: ${updated["description"]}")

        // 5. Delete template
        println("\n--- Deleting template ---")
        client.goalTemplateDelete(createdId)
        deleted = true
        println("Template deleted successfully")

        println("\n✓ Goal template CRUD example completed")
    } finally {
        if (!deleted) {
            templateId?.let { id ->
                runCatching { client.goalTemplateDelete(id) }
                    .onFailure { println("Cleanup error deleting template $id: ${it.message}") }
            }
        }
        client.close()
    }
}
