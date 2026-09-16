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

    var token: String? = null
    var runFailure: Throwable? = null

    try {
        // Step 1: Authenticate
        val tokenResponse = httpClient.post("$baseUrl/api/auth/token") {
            contentType(ContentType.Application.Json)
            setBody(buildJsonObject { put("api_key", apiKey) }.toString())
        }
        val tokenJson = Json.parseToJsonElement(tokenResponse.bodyAsText()).jsonObject
        token = tokenJson["token"]!!.jsonPrimitive.content
        println("✓ Authentication successful")

        val staleCleanup = httpClient.delete("$baseUrl/api/collections/$collection") {
            header("Authorization", "Bearer $token")
        }
        if (!staleCleanup.status.isSuccess() && staleCleanup.status != HttpStatusCode.NotFound) {
            error("Stale collection cleanup failed: ${staleCleanup.status} ${staleCleanup.bodyAsText()}")
        }

        // Step 2: Connect to WebSocket
        println("\n=== Connecting to WebSocket ===")

        val wsUrlParsed = Url(wsUrl)
        val notifications = Channel<JsonObject>(10)
        val responses = Channel<JsonObject>(10)

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
            val confirmFrame = withTimeout(5000) { incoming.receive() }
            require(confirmFrame is Frame.Text) { "Subscription confirmation was not text" }
            val response = Json.parseToJsonElement(confirmFrame.readText()).jsonObject
            require(response["type"]?.jsonPrimitive?.content == "Success") {
                "Subscription failed: $response"
            }
            val subId = response["payload"]
                ?.jsonObject?.get("data")
                ?.jsonObject?.get("subscription_id")
                ?.jsonPrimitive?.content
                ?: error("Subscription confirmation omitted subscription_id: $response")
            println("✓ Subscribed (subscription_id: $subId)")

            // Step 4: Launch a coroutine to listen for notifications
            val readJob = launch {
                for (frame in incoming) {
                    if (frame is Frame.Text) {
                        val parsed = Json.parseToJsonElement(frame.readText()).jsonObject
                        if (parsed["type"]?.jsonPrimitive?.content == "MutationNotification") {
                            notifications.send(parsed)
                        } else {
                            responses.send(parsed)
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
            require(insertRes1.status.isSuccess()) {
                "First insert failed: ${insertRes1.status} ${insertRes1.bodyAsText()}"
            }
            val insertData1 = Json.parseToJsonElement(insertRes1.bodyAsText()).jsonObject
            println("✓ Inserted: ${insertData1["id"]}")

            // Wait for notification
            val notification1 = withTimeout(5000) { notifications.receive() }
            val payload1 = notification1["payload"]!!.jsonObject
            require(payload1["event"]?.jsonPrimitive?.content == "insert") { "Unexpected event: $payload1" }
            require(payload1["collection"]?.jsonPrimitive?.content == collection) { "Unexpected collection: $payload1" }
            require(payload1["record_ids"]?.jsonArray?.any { it.jsonPrimitive.content == insertData1["id"]?.jsonPrimitive?.content } == true) {
                "First notification omitted inserted record: $payload1"
            }
            println("\n  📡 Notification received:")
            println("     Event:      ${payload1["event"]}")
            println("     Collection: ${payload1["collection"]}")
            println("     Record IDs: ${payload1["record_ids"]}")
            println("     Timestamp:  ${payload1["timestamp"]}")

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
            require(insertRes2.status.isSuccess()) {
                "Second insert failed: ${insertRes2.status} ${insertRes2.bodyAsText()}"
            }
            val insertData2 = Json.parseToJsonElement(insertRes2.bodyAsText()).jsonObject
            println("✓ Inserted: ${insertData2["id"]}")

            val notification2 = withTimeout(5000) { notifications.receive() }
            val payload2 = notification2["payload"]!!.jsonObject
            require(payload2["event"]?.jsonPrimitive?.content == "insert") { "Unexpected event: $payload2" }
            require(payload2["collection"]?.jsonPrimitive?.content == collection) { "Unexpected collection: $payload2" }
            require(payload2["record_ids"]?.jsonArray?.any { it.jsonPrimitive.content == insertData2["id"]?.jsonPrimitive?.content } == true) {
                "Second notification omitted inserted record: $payload2"
            }
            println("\n  📡 Notification received:")
            println("     Event:      ${payload2["event"]}")
            println("     Record IDs: ${payload2["record_ids"]}")

            // Step 6: Unsubscribe
            println("\n=== Unsubscribing ===")
            val unsubMsg = buildJsonObject {
                put("type", "Unsubscribe")
                put("payload", buildJsonObject {
                    put("collection", collection)
                })
            }
            send(Frame.Text(unsubMsg.toString()))
            val unsubResponse = withTimeout(5000) { responses.receive() }
            require(unsubResponse["type"]?.jsonPrimitive?.content == "Success") {
                "Unsubscribe failed: $unsubResponse"
            }
            println("✓ Unsubscribed: $unsubResponse")

            // Cleanup
            readJob.cancelAndJoin()
            println("\n✓ WebSocket subscription example completed successfully")
        }
    } catch (error: Throwable) {
        runFailure = error
        throw error
    } finally {
        try {
            token?.let {
                val cleanup = httpClient.delete("$baseUrl/api/collections/$collection") {
                    header("Authorization", "Bearer $it")
                }
                if (!cleanup.status.isSuccess() && cleanup.status != HttpStatusCode.NotFound) {
                    error("Collection cleanup failed: ${cleanup.status} ${cleanup.bodyAsText()}")
                }
                println("✓ Deleted collection '$collection'")
            }
        } catch (cleanupError: Throwable) {
            if (runFailure != null) runFailure.addSuppressed(cleanupError)
            else throw cleanupError
        } finally {
            httpClient.close()
        }
    }
}
