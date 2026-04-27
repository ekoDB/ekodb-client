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
 * Crypto stages — comprehensive demo of every primitive added in
 * ekoDB 0.43.0 (HMAC, AES-GCM, UUID, TOTP, Base64, Hex, Slugify).
 *
 *   crypto_demo_hmac      HmacSign + HmacVerify (round trip)
 *   crypto_demo_aes       AesEncrypt + AesDecrypt (round trip)
 *   crypto_demo_uuid      UuidGenerate
 *   crypto_demo_totp      TotpGenerate (RFC 6238)
 *   crypto_demo_encoding  Base64 + Hex + Slugify
 *
 * Operator-owned secrets flow through `{{env.NAME}}` so they never
 * land in the function definition stored on disk.
 *
 * Requires ekoDB >= 0.43.0.
 */
fun main() = runBlocking {
    val dotenv = dotenv { ignoreIfMissing = true }
    val baseUrl = dotenv["API_BASE_URL"] ?: "http://localhost:8080"
    val apiKey = dotenv["API_BASE_KEY"] ?: "a-test-api-key-from-ekodb"

    val client = EkoDBClient.builder().baseUrl(baseUrl).apiKey(apiKey).build()
    println("✓ Client created")

    // 1. HMAC sign + verify round trip.
    val hmac = buildJsonObject {
        put("label", "crypto_demo_hmac")
        put("name", "HMAC sign + verify")
        putJsonObject("parameters") {
            putJsonObject("payload") { put("required", true) }
        }
        putJsonArray("functions") {
            add(buildJsonObject {
                put("type", "HmacSign")
                put("input", "{{payload}}")
                put("secret", "{{env.HMAC_KEY}}")
                put("algorithm", "sha256")
                put("output_field", "mac")
                put("encoding", "hex")
            })
            add(buildJsonObject {
                put("type", "HmacVerify")
                put("input", "{{payload}}")
                put("provided_mac", "{{mac}}")
                put("secret", "{{env.HMAC_KEY}}")
                put("algorithm", "sha256")
                put("encoding", "hex")
                put("output_field", "verified")
            })
        }
    }
    saveCryptoFn(client, hmac, "crypto_demo_hmac")

    // 2. AES-256-GCM encrypt + decrypt round trip.
    val aes = buildJsonObject {
        put("label", "crypto_demo_aes")
        put("name", "AES encrypt + decrypt")
        putJsonObject("parameters") {
            putJsonObject("plaintext") { put("required", true) }
        }
        putJsonArray("functions") {
            add(buildJsonObject {
                put("type", "AesEncrypt")
                put("plaintext", "{{plaintext}}")
                put("key", "{{env.DATA_KEY}}")
                put("key_encoding", "hex")
                put("output_field", "envelope")
            })
            add(buildJsonObject {
                put("type", "AesDecrypt")
                put("ciphertext_field", "envelope")
                put("key", "{{env.DATA_KEY}}")
                put("key_encoding", "hex")
                put("output_field", "recovered")
            })
        }
    }
    saveCryptoFn(client, aes, "crypto_demo_aes")

    // 3. UuidGenerate.
    val uuidFn = buildJsonObject {
        put("label", "crypto_demo_uuid")
        put("name", "Generate v4 UUID")
        putJsonObject("parameters") {}
        putJsonArray("functions") {
            add(buildJsonObject {
                put("type", "UuidGenerate")
                put("output_field", "id")
            })
        }
    }
    saveCryptoFn(client, uuidFn, "crypto_demo_uuid")

    // 4. TotpGenerate (RFC 6238 with SHA1).
    val totp = buildJsonObject {
        put("label", "crypto_demo_totp")
        put("name", "Generate TOTP code")
        putJsonObject("parameters") {}
        putJsonArray("functions") {
            add(buildJsonObject {
                put("type", "TotpGenerate")
                put("secret", "{{env.TOTP_SECRET}}")
                put("digits", 6)
                put("period", 30)
                put("algorithm", "sha1")
                put("output_field", "code")
            })
        }
    }
    saveCryptoFn(client, totp, "crypto_demo_totp")

    // 5. Base64 + Hex + Slugify chained.
    val encoding = buildJsonObject {
        put("label", "crypto_demo_encoding")
        put("name", "Base64 / Hex / Slugify")
        putJsonObject("parameters") {
            putJsonObject("title") { put("required", true) }
        }
        putJsonArray("functions") {
            add(buildJsonObject {
                put("type", "Base64Encode")
                put("input", "{{title}}")
                put("output_field", "title_b64")
            })
            add(buildJsonObject {
                put("type", "HexEncode")
                put("input", "{{title}}")
                put("output_field", "title_hex")
            })
            add(buildJsonObject {
                put("type", "Slugify")
                put("input", "{{title}}")
                put("output_field", "title_slug")
            })
        }
    }
    saveCryptoFn(client, encoding, "crypto_demo_encoding")

    println("\nInvoke them with:")
    println("  POST /api/functions/crypto_demo_hmac     { \"payload\": \"hi\" }")
    println("  POST /api/functions/crypto_demo_aes      { \"plaintext\": \"secret\" }")
    println("  POST /api/functions/crypto_demo_uuid")
    println("  POST /api/functions/crypto_demo_totp")
    println("  POST /api/functions/crypto_demo_encoding { \"title\": \"Héllo World\" }")

    for (label in listOf(
        "crypto_demo_hmac", "crypto_demo_aes", "crypto_demo_uuid",
        "crypto_demo_totp", "crypto_demo_encoding",
    )) {
        try { client.deleteUserFunction(label) } catch (_: Exception) {}
    }
    println("\n✓ Cleaned up demo functions")
}

private suspend fun saveCryptoFn(
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
