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
 * Concurrency stages — IdempotencyClaim, RateLimit, LockAcquire/Release.
 *
 * These three primitives wrap ekoDB's atomic KV layer with the right
 * semantics for common server-side patterns. Requires ekoDB >= 0.42.0.
 */
fun main() = runBlocking {
    val dotenv = dotenv { ignoreIfMissing = true }
    val baseUrl = dotenv["API_BASE_URL"] ?: "http://localhost:8080"
    val apiKey = dotenv["API_BASE_KEY"] ?: "a-test-api-key-from-ekodb"

    val client = EkoDBClient.builder().baseUrl(baseUrl).apiKey(apiKey).build()
    println("✓ Client created")

    // 1. Idempotent payment — claim → branch on replay → do work.
    val pay = buildJsonObject {
        put("label", "conc_demo_pay")
        put("name", "Idempotent payment")
        putJsonObject("parameters") {
            putJsonObject("idempotency_key") { put("required", true) }
            putJsonObject("amount") { put("required", true) }
        }
        putJsonArray("functions") {
            add(buildJsonObject {
                put("type", "IdempotencyClaim")
                put("key", "{{idempotency_key}}")
                put("ttl_secs", 86400)
                put("output_field", "claim")
            })
            add(buildJsonObject {
                put("type", "If")
                putJsonObject("condition") {
                    put("type", "FieldEquals")
                    putJsonObject("value") {
                        put("field", "claim.claimed")
                        put("value", false)
                    }
                }
                putJsonArray("then_functions") {
                    add(buildJsonObject {
                        put("type", "Return")
                        putJsonObject("fields") {
                            put("status", "replay")
                            put("idempotency_key", "{{idempotency_key}}")
                        }
                        put("status_code", 200)
                    })
                }
                putJsonArray("else_functions") {
                    add(buildJsonObject {
                        put("type", "Insert")
                        put("collection", "charges")
                        putJsonObject("record") {
                            put("amount", "{{amount}}")
                            put("idempotency_key", "{{idempotency_key}}")
                        }
                        put("bypass_ripple", false)
                    })
                    add(buildJsonObject {
                        put("type", "Return")
                        putJsonObject("fields") { put("status", "charged") }
                        put("status_code", 201)
                    })
                }
            })
        }
    }
    saveConcFn(client, pay, "conc_demo_pay")

    // 2. Rate-limited endpoint (fail mode).
    val rlFail = buildJsonObject {
        put("label", "conc_demo_rl_fail")
        put("name", "Rate-limit (fail mode)")
        putJsonObject("parameters") {
            putJsonObject("user_id") { put("required", true) }
        }
        putJsonArray("functions") {
            add(buildJsonObject {
                put("type", "RateLimit")
                put("key", "user-{{user_id}}")
                put("limit", 10)
                put("window_secs", 60)
                put("output_field", "rl")
            })
            add(buildJsonObject {
                put("type", "Return")
                putJsonObject("fields") { put("ok", true) }
                put("status_code", 200)
            })
        }
    }
    saveConcFn(client, rlFail, "conc_demo_rl_fail")

    // 3. Rate-limited endpoint (skip mode).
    val rlSkip = buildJsonObject {
        put("label", "conc_demo_rl_skip")
        put("name", "Rate-limit (skip mode)")
        putJsonObject("parameters") {
            putJsonObject("user_id") { put("required", true) }
        }
        putJsonArray("functions") {
            add(buildJsonObject {
                put("type", "RateLimit")
                put("key", "user-{{user_id}}")
                put("limit", 10)
                put("window_secs", 60)
                put("on_exceed", "skip")
                put("output_field", "rl")
            })
            add(buildJsonObject {
                put("type", "If")
                putJsonObject("condition") {
                    put("type", "FieldEquals")
                    putJsonObject("value") {
                        put("field", "rl.allowed")
                        put("value", false)
                    }
                }
                putJsonArray("then_functions") {
                    add(buildJsonObject {
                        put("type", "Return")
                        putJsonObject("fields") { put("status", "rate_limited") }
                        put("status_code", 429)
                    })
                }
                putJsonArray("else_functions") {
                    add(buildJsonObject {
                        put("type", "Return")
                        putJsonObject("fields") { put("status", "ok") }
                        put("status_code", 200)
                    })
                }
            })
        }
    }
    saveConcFn(client, rlSkip, "conc_demo_rl_skip")

    // 4. Distributed lock — acquire + critical section + release (token-fenced).
    val lock = buildJsonObject {
        put("label", "conc_demo_lock")
        put("name", "Critical section under lock")
        putJsonObject("parameters") {
            putJsonObject("resource") { put("required", true) }
        }
        putJsonArray("functions") {
            add(buildJsonObject {
                put("type", "LockAcquire")
                put("key", "{{resource}}")
                put("ttl_secs", 30)
                put("output_field", "lock")
            })
            add(buildJsonObject {
                put("type", "If")
                putJsonObject("condition") {
                    put("type", "FieldEquals")
                    putJsonObject("value") {
                        put("field", "lock.acquired")
                        put("value", false)
                    }
                }
                putJsonArray("then_functions") {
                    add(buildJsonObject {
                        put("type", "Return")
                        putJsonObject("fields") { put("status", "busy") }
                        put("status_code", 409)
                    })
                }
                putJsonArray("else_functions") {
                    add(buildJsonObject {
                        put("type", "Insert")
                        put("collection", "lock_demo_audit")
                        putJsonObject("record") { put("resource", "{{resource}}") }
                        put("bypass_ripple", false)
                        put("ttl", 60)
                    })
                    add(buildJsonObject {
                        put("type", "LockRelease")
                        put("key", "{{resource}}")
                        put("token", "{{lock.token}}")
                        put("output_field", "release")
                    })
                    add(buildJsonObject {
                        put("type", "Return")
                        putJsonObject("fields") { put("status", "done") }
                        put("status_code", 200)
                    })
                }
            })
        }
    }
    saveConcFn(client, lock, "conc_demo_lock")

    println("\nInvoke them like:")
    println("  POST /api/functions/conc_demo_pay        { \"idempotency_key\": \"...\", \"amount\": 100 }")
    println("  POST /api/functions/conc_demo_rl_fail    { \"user_id\": 42 }")
    println("  POST /api/functions/conc_demo_rl_skip    { \"user_id\": 42 }")
    println("  POST /api/functions/conc_demo_lock       { \"resource\": \"queue:drain\" }")

    for (label in listOf(
        "conc_demo_pay", "conc_demo_rl_fail",
        "conc_demo_rl_skip", "conc_demo_lock",
    )) {
        try { client.deleteUserFunction(label) } catch (_: Exception) {}
    }
    println("\n✓ Cleaned up demo functions")
}

private suspend fun saveConcFn(
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
