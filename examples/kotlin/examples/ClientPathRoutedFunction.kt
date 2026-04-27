package io.ekodb.client.examples

import io.ekodb.client.EkoDBClient
import io.github.cdimascio.dotenv.dotenv
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.add
import kotlinx.serialization.json.buildJsonArray
import kotlinx.serialization.json.buildJsonObject
import kotlinx.serialization.json.put
import kotlinx.serialization.json.putJsonArray
import kotlinx.serialization.json.putJsonObject

/**
 * Path-routed function — expose a stored function under a REST URL.
 *
 * Sets `http_method` + `http_path` on a UserFunction so ekoDB's
 * path-routed dispatcher (`/api/route/{path}`) routes incoming HTTP
 * requests to it.
 *
 * Routing rules:
 *   - method match required (GET/POST/PUT/PATCH/DELETE all dispatch)
 *   - segment count must match the pattern
 *   - lowest "specificity score" (fewest `:placeholder`) wins on ties
 *   - body params merge OVER path params on collision
 *
 * Requires ekoDB >= 0.42.0.
 */
fun main() = runBlocking {
    val dotenv = dotenv { ignoreIfMissing = true }
    val baseUrl = dotenv["API_BASE_URL"] ?: "http://localhost:8080"
    val apiKey = dotenv["API_BASE_KEY"] ?: "a-test-api-key-from-ekodb"

    val client = EkoDBClient.builder().baseUrl(baseUrl).apiKey(apiKey).build()
    println("✓ Client created")

    // Most-specific literal — `/users/admin` answers admin lookups
    // separately from the `:id` route.
    val admin = buildJsonObject {
        put("label", "kt_route_admin")
        put("name", "Get admin user")
        put("http_method", "GET")
        put("http_path", "/users/admin")
        putJsonObject("parameters") {}
        putJsonArray("functions") {
            add(buildJsonObject {
                put("type", "Return")
                putJsonObject("fields") { put("role", "admin") }
                put("status_code", 200)
            })
        }
    }
    saveRouteFn(client, admin, "kt_route_admin")

    // Single-placeholder route.
    val byId = buildJsonObject {
        put("label", "kt_route_user_by_id")
        put("name", "Get user by id")
        put("http_method", "GET")
        put("http_path", "/users/:id")
        putJsonObject("parameters") {
            putJsonObject("id") { put("required", true) }
        }
        putJsonArray("functions") {
            add(buildJsonObject {
                put("type", "Return")
                putJsonObject("fields") { put("requested_id", "{{id}}") }
                put("status_code", 200)
            })
        }
    }
    saveRouteFn(client, byId, "kt_route_user_by_id")

    // Two-placeholder nested route.
    val posts = buildJsonObject {
        put("label", "kt_route_user_posts")
        put("name", "Get user's post")
        put("http_method", "GET")
        put("http_path", "/users/:id/posts/:post_id")
        putJsonObject("parameters") {
            putJsonObject("id") { put("required", true) }
            putJsonObject("post_id") { put("required", true) }
        }
        putJsonArray("functions") {
            add(buildJsonObject {
                put("type", "Return")
                putJsonObject("fields") {
                    put("user_id", "{{id}}")
                    put("post_id", "{{post_id}}")
                }
                put("status_code", 200)
            })
        }
    }
    saveRouteFn(client, posts, "kt_route_user_posts")

    // POST with body merge: path provides `:org`, body provides `name`.
    val create = buildJsonObject {
        put("label", "kt_route_org_create_member")
        put("name", "Add member to org")
        put("http_method", "POST")
        put("http_path", "/orgs/:org/members")
        putJsonObject("parameters") {
            putJsonObject("org") { put("required", true) }
            putJsonObject("name") { put("required", true) }
        }
        putJsonArray("functions") {
            add(buildJsonObject {
                put("type", "Return")
                putJsonObject("fields") {
                    put("org", "{{org}}")
                    put("name", "{{name}}")
                    put("created", true)
                }
                put("status_code", 201)
            })
        }
    }
    saveRouteFn(client, create, "kt_route_org_create_member")

    println("\nTry them with curl:")
    println("  curl http://localhost:8080/api/route/users/admin")
    println("  curl http://localhost:8080/api/route/users/42")
    println("  curl http://localhost:8080/api/route/users/42/posts/7")
    println("  curl -X POST http://localhost:8080/api/route/orgs/acme/members \\")
    println("       -H 'Content-Type: application/json' -d '{\"name\":\"alice\"}'")

    for (label in listOf(
        "kt_route_admin", "kt_route_user_by_id",
        "kt_route_user_posts", "kt_route_org_create_member",
    )) {
        try { client.deleteUserFunction(label) } catch (_: Exception) {}
    }
    println("\n✓ Cleaned up demo functions")
}

private suspend fun saveRouteFn(
    client: EkoDBClient,
    fn: kotlinx.serialization.json.JsonObject,
    label: String,
) {
    try {
        client.saveUserFunction(fn)
        println("✓ $label saved")
    } catch (e: Exception) {
        println("SaveUserFunction($label) error: ${e.message}")
    }
}
