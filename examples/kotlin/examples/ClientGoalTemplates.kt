// ekoDB Kotlin Client - Goal Template CRUD Example
//
// Demonstrates creating, listing, getting, updating, and deleting goal templates.

import io.ekodb.client.EkoDBClient
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
    val templateId = template["id"].toString().trim('"')
    println("Created template: ${template["title"]} (id: $templateId)")

    // 2. List all templates
    println("\n--- Listing templates ---")
    val list = client.goalTemplateList()
    println("Templates: $list")

    // 3. Get template by ID
    println("\n--- Getting template ---")
    val fetched = client.goalTemplateGet(templateId)
    println("Fetched: ${fetched["title"]}")

    // 4. Update template
    println("\n--- Updating template ---")
    val updated = client.goalTemplateUpdate(templateId, buildJsonObject {
        put("description", "Updated: comprehensive data migration workflow")
    })
    println("Updated description: ${updated["description"]}")

    // 5. Delete template
    println("\n--- Deleting template ---")
    client.goalTemplateDelete(templateId)
    println("Template deleted successfully")

    client.close()
    println("\n✓ Goal template CRUD example completed")
}
