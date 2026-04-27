package io.ekodb.client.examples

import io.ekodb.client.EkoDBClient
import io.github.cdimascio.dotenv.dotenv
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.JsonPrimitive
import kotlinx.serialization.json.add
import kotlinx.serialization.json.buildJsonArray
import kotlinx.serialization.json.buildJsonObject
import kotlinx.serialization.json.put
import kotlinx.serialization.json.putJsonArray
import kotlinx.serialization.json.putJsonObject

/**
 * JWT auth flow — registration + login as pure stored functions.
 *
 * Demonstrates the canonical password-auth pattern using ekoDB stages:
 *
 *   register flow:  BcryptHash → Insert
 *   login flow:     FindOne  → BcryptVerify → If(ok) { JwtSign + Return }
 *   verify flow:    JwtVerify → If(claims != null) { 200 } else { 401 }
 *
 * Operator-owned secrets (the JWT signing key) flow through
 * `{{env.JWT_SECRET}}` so they never enter the function definition or
 * the LLM context window.
 *
 * Requires ekoDB >= 0.43.0.
 */
fun main() = runBlocking {
    val dotenv = dotenv { ignoreIfMissing = true }
    val baseUrl = dotenv["API_BASE_URL"] ?: "http://localhost:8080"
    val apiKey = dotenv["API_BASE_KEY"] ?: "a-test-api-key-from-ekodb"

    val client = EkoDBClient.builder()
        .baseUrl(baseUrl)
        .apiKey(apiKey)
        .build()

    println("✓ Client created")

    // 1. Register: bcrypt-hash, insert.
    val register = buildJsonObject {
        put("label", "kt_users_register")
        put("name", "Register user")
        put("description", "Validate, bcrypt-hash, insert.")
        putJsonObject("parameters") {
            putJsonObject("email") { put("required", true) }
            putJsonObject("password") { put("required", true) }
        }
        putJsonArray("functions") {
            add(buildJsonObject {
                put("type", "BcryptHash")
                put("plain", "{{password}}")
                put("cost", 12)
                put("output_field", "password_hash")
            })
            add(buildJsonObject {
                put("type", "Insert")
                put("collection", "kt_users")
                putJsonObject("record") {
                    put("email", "{{email}}")
                    put("password_hash", "{{password_hash}}")
                }
                put("bypass_ripple", false)
            })
        }
    }
    saveJwtAuthFn(client, register, "kt_users_register")

    // 2. Login: find user, verify bcrypt, sign JWT on success.
    val login = buildJsonObject {
        put("label", "kt_users_login")
        put("name", "Login user")
        put("description", "Verify password, mint JWT.")
        putJsonObject("parameters") {
            putJsonObject("email") { put("required", true) }
            putJsonObject("password") { put("required", true) }
        }
        putJsonArray("functions") {
            add(buildJsonObject {
                put("type", "FindOne")
                put("collection", "kt_users")
                put("key", "email")
                put("value", "{{email}}")
            })
            add(buildJsonObject {
                put("type", "BcryptVerify")
                put("plain", "{{password}}")
                put("hash_field", "password_hash")
                put("output_field", "password_ok")
            })
            add(buildJsonObject {
                put("type", "If")
                putJsonObject("condition") {
                    put("type", "FieldEquals")
                    putJsonObject("value") {
                        put("field", "password_ok")
                        put("value", true)
                    }
                }
                putJsonArray("then_functions") {
                    add(buildJsonObject {
                        put("type", "JwtSign")
                        putJsonObject("claims") { put("email", "{{email}}") }
                        put("secret", "{{env.JWT_SECRET}}")
                        put("algorithm", "HS256")
                        put("expires_in_secs", 3600)
                        put("output_field", "token")
                    })
                    add(buildJsonObject {
                        put("type", "Return")
                        putJsonObject("fields") {
                            put("ok", true)
                            put("token", "{{token}}")
                        }
                        put("status_code", 200)
                    })
                }
                putJsonArray("else_functions") {
                    add(buildJsonObject {
                        put("type", "Return")
                        putJsonObject("fields") { put("ok", false) }
                        put("status_code", 401)
                    })
                }
            })
        }
    }
    saveJwtAuthFn(client, login, "kt_users_login")

    // 3. Verify a JWT — fail-closed when claims is null.
    val verify = buildJsonObject {
        put("label", "kt_users_verify_token")
        put("name", "Verify JWT token")
        putJsonObject("parameters") {
            putJsonObject("token") { put("required", true) }
        }
        putJsonArray("functions") {
            // Synthetic record so JwtVerify has working_data[0] to read off.
            add(buildJsonObject {
                put("type", "Insert")
                put("collection", "_inflight_jwt_check")
                putJsonObject("record") { put("token", "{{token}}") }
                put("bypass_ripple", true)
                put("ttl", 60)
            })
            add(buildJsonObject {
                put("type", "JwtVerify")
                put("token_field", "token")
                put("secret", "{{env.JWT_SECRET}}")
                put("algorithm", "HS256")
                put("output_field", "claims")
            })
            add(buildJsonObject {
                put("type", "If")
                putJsonObject("condition") {
                    put("type", "FieldEquals")
                    putJsonObject("value") {
                        put("field", "claims")
                        // null literal in JsonObject form
                        put("value", JsonPrimitive(null as String?))
                    }
                }
                putJsonArray("then_functions") {
                    add(buildJsonObject {
                        put("type", "Return")
                        putJsonObject("fields") { put("ok", false) }
                        put("status_code", 401)
                    })
                }
                putJsonArray("else_functions") {
                    add(buildJsonObject {
                        put("type", "Return")
                        putJsonObject("fields") { put("ok", true) }
                        put("status_code", 200)
                    })
                }
            })
        }
    }
    saveJwtAuthFn(client, verify, "kt_users_verify_token")

    println("\n=== Auth flow defined as pure stored functions ===")
    println("Call them like:")
    println("  POST /api/functions/kt_users_register { \"email\": \"a@b.com\", \"password\": \"s3cret\" }")
    println("  POST /api/functions/kt_users_login    { \"email\": \"a@b.com\", \"password\": \"s3cret\" }")
    println("  POST /api/functions/kt_users_verify_token { \"token\": \"<jwt>\" }")

    for (label in listOf("kt_users_register", "kt_users_login", "kt_users_verify_token")) {
        try {
            client.deleteUserFunction(label)
        } catch (_: Exception) {
        }
    }
    println("\n✓ Cleaned up demo functions")
}

private suspend fun saveJwtAuthFn(
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
