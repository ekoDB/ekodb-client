package io.ekodb.client.examples

import io.ekodb.client.EkoDBClient
import io.github.cdimascio.dotenv.dotenv
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.JsonPrimitive
import kotlinx.serialization.json.buildJsonArray
import kotlinx.serialization.json.buildJsonObject
import kotlinx.serialization.json.put
import kotlinx.serialization.json.putJsonArray
import kotlinx.serialization.json.putJsonObject

/**
 * User Functions API Example - Using ekoDB Kotlin client library
 *
 * This example demonstrates CRUD operations for User Functions.
 * User Functions are reusable sequences of Functions that can be called by Scripts.
 */
fun main() = runBlocking {
    val dotenv = dotenv()
    val baseUrl = dotenv["API_BASE_URL"] ?: "http://localhost:8080"
    val apiKey = dotenv["API_BASE_KEY"] ?: "a-test-api-key-from-ekodb"

    val client = EkoDBClient.builder()
        .baseUrl(baseUrl)
        .apiKey(apiKey)
        .build()

    println("=== ekoDB Kotlin Client - User Functions Example ===\n")

    try {
        // Example 1: Create a User Function
        println("=== Create User Function ===")
        val userFunction = buildJsonObject {
            put("label", "get_active_users_kt")
            put("name", "Get Active Users (Kotlin)")
            put("description", "Fetches all users and filters by active status")
            put("version", "1.0.0")
            putJsonObject("parameters") {
                putJsonObject("collection") {
                    put("required", true)
                    put("description", "Collection to query")
                }
            }
            putJsonArray("functions") {
                add(buildJsonObject {
                    put("type", "FindAll")
                    put("collection", "{{collection}}")
                })
            }
            putJsonArray("tags") {
                add(JsonPrimitive("users"))
                add(JsonPrimitive("query"))
                add(JsonPrimitive("kotlin"))
            }
        }

        try {
            val funcId = client.saveUserFunction(userFunction)
            println("Created user function with ID: $funcId")
        } catch (e: Exception) {
            println("SaveUserFunction error: ${e.message}")
        }
        println()

        // Example 2: Get User Function by label
        println("=== Get User Function ===")
        try {
            val retrieved = client.getUserFunction("get_active_users_kt")
            println("Retrieved: ${retrieved["label"]} - ${retrieved["name"]}")
            retrieved["description"]?.let { println("Description: $it") }
        } catch (e: Exception) {
            println("GetUserFunction error: ${e.message}")
        }
        println()

        // Example 3: List all User Functions
        println("=== List All User Functions ===")
        try {
            val allFuncs = client.listUserFunctions()
            println("Found ${allFuncs.size} user functions:")
            for (func in allFuncs.take(5)) {
                println("  - ${func["label"]}: ${func["name"]}")
            }
        } catch (e: Exception) {
            println("ListUserFunctions error: ${e.message}")
        }
        println()

        // Example 4: List User Functions by tag
        println("=== List User Functions by Tag ===")
        try {
            val taggedFuncs = client.listUserFunctions(listOf("kotlin"))
            println("Found ${taggedFuncs.size} user functions with 'kotlin' tag:")
            for (func in taggedFuncs) {
                println("  - ${func["label"]}")
            }
        } catch (e: Exception) {
            println("ListUserFunctions (with tags) error: ${e.message}")
        }
        println()

        // Example 5: Update User Function
        println("=== Update User Function ===")
        val updatedFunction = buildJsonObject {
            put("label", "get_active_users_kt")
            put("name", "Get Active Users (Kotlin - Updated)")
            put("description", "Fetches all users - updated description")
            put("version", "1.1.0")
            putJsonObject("parameters") {
                putJsonObject("collection") {
                    put("required", true)
                    put("description", "Collection to query")
                }
            }
            putJsonArray("functions") {
                add(buildJsonObject {
                    put("type", "FindAll")
                    put("collection", "{{collection}}")
                })
            }
            putJsonArray("tags") {
                add(JsonPrimitive("users"))
                add(JsonPrimitive("query"))
                add(JsonPrimitive("kotlin"))
            }
        }

        try {
            client.updateUserFunction("get_active_users_kt", updatedFunction)
            println("User function updated successfully")
        } catch (e: Exception) {
            println("UpdateUserFunction error: ${e.message}")
        }
        println()

        // Example 6: Delete User Function
        println("=== Delete User Function ===")
        try {
            client.deleteUserFunction("get_active_users_kt")
            println("User function deleted successfully")
        } catch (e: Exception) {
            println("DeleteUserFunction error: ${e.message}")
        }

    } finally {
        client.close()
        println("\n=== User Functions Example Complete ===")
    }
}
