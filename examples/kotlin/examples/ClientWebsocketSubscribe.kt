package io.ekodb.client.examples

import io.github.cdimascio.dotenv.dotenv
import io.ktor.client.*
import io.ktor.client.engine.cio.*
import io.ktor.client.plugins.contentnegotiation.*
import io.ktor.client.plugins.websocket.*
import io.ktor.client.request.*
import io.ktor.client.statement.*
import io.ktor.http.*
import io.ktor.serialization.kotlinx.json.*
import io.ktor.websocket.*
import kotlinx.coroutines.*
import kotlinx.coroutines.channels.Channel
import kotlinx.serialization.json.*

/**
 * WebSocket Subscription Example - Real-time mutation notifications
 *
 * Demonstrates subscribing to collection changes via WebSocket and
 * receiving push notifications when records are inserted, updated, or deleted.
 *
 * Requires: ekoDB server running on localhost:8080
 */
fun main() = runBlocking {
    val dotenv = dotenv()
    val baseUrl = dotenv["API_BASE_URL"] ?: "http://localhost:8080"
    val wsUrl = dotenv["WS_BASE_URL"] ?: "ws://localhost:8080"
    val apiKey = dotenv["API_BASE_KEY"] ?: "a-test-api-key-from-ekodb"

    val collection = "ws_subscribe_example_kt"

    println("=== WebSocket Subscription Example (Kotlin) ===\n")

    val httpClient = HttpClient(CIO) {
        install(ContentNegotiation) {
            json(Json { ignoreUnknownKeys = true })
        }
        install(WebSockets)
    }

    try {
        // Step 1: Authenticate
        val tokenResponse = httpClient.post("$baseUrl/api/auth/token") {
            contentType(ContentType.Application.Json)
            setBody(buildJsonObject { put("api_key", apiKey) }.toString())
        }
        val tokenJson = Json.parseToJsonElement(tokenResponse.bodyAsText()).jsonObject
        val token = tokenJson["token"]!!.jsonPrimitive.content
        println("✓ Authentication successful")

        // Step 2: Connect to WebSocket
        println("\n=== Connecting to WebSocket ===")

        val wsUrlParsed = Url(wsUrl)
        val notifications = Channel<JsonObject>(10)

        httpClient.webSocket(
            method = HttpMethod.Get,
            host = wsUrlParsed.host,
            port = wsUrlParsed.port,
            path = "/api/ws",
            request = {
                header("Authorization", "Bearer $token")
            }
        ) {
            println("✓ WebSocket connected")

            // Step 3: Subscribe to collection
            println("\n=== Subscribing to '$collection' ===")

            val subscribeMsg = buildJsonObject {
                put("type", "Subscribe")
                put("payload", buildJsonObject {
                    put("collection", collection)
                })
            }
            send(Frame.Text(subscribeMsg.toString()))

            // Read subscription confirmation
            val confirmFrame = incoming.receive()
            if (confirmFrame is Frame.Text) {
                val response = Json.parseToJsonElement(confirmFrame.readText()).jsonObject
                val subId = response["payload"]
                    ?.jsonObject?.get("data")
                    ?.jsonObject?.get("subscription_id")
                    ?.jsonPrimitive?.content
                println("✓ Subscribed (subscription_id: $subId)")
            }

            // Step 4: Launch a coroutine to listen for notifications
            val readJob = launch {
                for (frame in incoming) {
                    if (frame is Frame.Text) {
                        val parsed = Json.parseToJsonElement(frame.readText()).jsonObject
                        if (parsed["type"]?.jsonPrimitive?.content == "MutationNotification") {
                            notifications.send(parsed)
                        }
                    }
                }
            }

            // Step 5: Insert records via REST API to trigger notifications
            println("\n=== Performing mutations to trigger notifications ===")

            println("Inserting record 1...")
            val insertRes1 = httpClient.post("$baseUrl/api/insert/$collection") {
                header("Authorization", "Bearer $token")
                contentType(ContentType.Application.Json)
                setBody(buildJsonObject {
                    put("name", "Alice")
                    put("role", "engineer")
                    put("active", JsonPrimitive(true))
                }.toString())
            }
            val insertData1 = Json.parseToJsonElement(insertRes1.bodyAsText()).jsonObject
            println("✓ Inserted: ${insertData1["id"]}")

            // Wait for notification
            try {
                val notification = withTimeout(5000) { notifications.receive() }
                val payload = notification["payload"]!!.jsonObject
                println("\n  📡 Notification received:")
                println("     Event:      ${payload["event"]}")
                println("     Collection: ${payload["collection"]}")
                println("     Record IDs: ${payload["record_ids"]}")
                println("     Timestamp:  ${payload["timestamp"]}")
            } catch (e: TimeoutCancellationException) {
                println("  ⏳ No notification within timeout")
            }

            println("\nInserting record 2...")
            val insertRes2 = httpClient.post("$baseUrl/api/insert/$collection") {
                header("Authorization", "Bearer $token")
                contentType(ContentType.Application.Json)
                setBody(buildJsonObject {
                    put("name", "Bob")
                    put("role", "designer")
                    put("active", JsonPrimitive(true))
                }.toString())
            }
            val insertData2 = Json.parseToJsonElement(insertRes2.bodyAsText()).jsonObject
            println("✓ Inserted: ${insertData2["id"]}")

            try {
                val notification = withTimeout(5000) { notifications.receive() }
                val payload = notification["payload"]!!.jsonObject
                println("\n  📡 Notification received:")
                println("     Event:      ${payload["event"]}")
                println("     Record IDs: ${payload["record_ids"]}")
            } catch (e: TimeoutCancellationException) {
                println("  ⏳ No notification within timeout")
            }

            // Step 6: Unsubscribe
            println("\n=== Unsubscribing ===")
            val unsubMsg = buildJsonObject {
                put("type", "Unsubscribe")
                put("payload", buildJsonObject {
                    put("collection", collection)
                })
            }
            send(Frame.Text(unsubMsg.toString()))
            println("✓ Unsubscribed")

            // Cleanup
            readJob.cancelAndJoin()
            println("\n✓ WebSocket subscription example completed successfully")
        }
    } finally {
        httpClient.close()
    }
}
