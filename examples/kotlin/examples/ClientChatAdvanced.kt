package io.ekodb.client.examples

import io.ekodb.client.EkoDBClient
import io.ekodb.client.types.Record
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.buildJsonObject
import kotlinx.serialization.json.jsonArray
import kotlinx.serialization.json.jsonObject
import kotlinx.serialization.json.jsonPrimitive
import kotlinx.serialization.json.put
import kotlinx.serialization.json.putJsonArray
import kotlinx.serialization.json.putJsonObject

/**
 * Advanced Chat example - Message regeneration, editing, and advanced features
 */
fun main() = runBlocking {
    val baseUrl = System.getenv("API_BASE_URL") ?: "http://localhost:8080"
    val apiKey = System.getenv("API_BASE_KEY") ?: error("API_BASE_KEY environment variable not set")
    
    val client = EkoDBClient.builder()
        .baseUrl(baseUrl)
        .apiKey(apiKey)
        .build()
    
    val collection = "kotlin_chat_advanced_example"
    
    println("=== ekoDB Kotlin Client - Advanced Chat Features Example ===\n")
    
    try {
        // Insert sample data
        println("=== Inserting Sample Data ===")
        
        val record = Record.new()
            .insert("name", "ekoDB")
            .insert("description", "High-performance database product")
            .insert("price", 99)
        client.insert(collection, record)
        println("✓ Inserted sample product\n")
        
        // Create a chat session
        println("=== Creating Chat Session ===")
        
        val sessionRequest = buildJsonObject {
            putJsonArray("collections") {
                add(buildJsonObject {
                    put("collection_name", collection)
                    putJsonArray("fields") {}
                })
            }
            put("llm_provider", "openai")
            put("llm_model", "gpt-4")
            put("system_prompt", "You are a helpful product assistant.")
        }
        
        val session = client.createChatSession(sessionRequest)
        val chatId = session["chat_id"]?.jsonPrimitive?.content ?: error("No chat_id in response")
        println("✓ Created session: $chatId\n")
        
        // Send initial message
        println("=== Sending Initial Message ===")
        
        val msg1Request = buildJsonObject {
            put("message", "What products are available?")
        }
        val msg1 = client.chatMessage(chatId, msg1Request)
        println("✓ Message sent")
        println("  Responses: ${msg1["responses"]}\n")
        
        // Send second message
        val msg2Request = buildJsonObject {
            put("message", "What is the price of ekoDB?")
        }
        client.chatMessage(chatId, msg2Request)
        println("✓ Second message sent\n")
        
        // Get message history to find assistant and user message IDs
        val messagesResponse = client.getChatSessionMessages(chatId, buildJsonObject {
            put("limit", 10)
            put("sort", "desc")
        })
        val messages = messagesResponse["messages"]?.jsonArray ?: error("No messages in response")
        
        // Find the most recent assistant message ID
        val assistantMessageId = messages.firstOrNull { msg ->
            val role = msg.jsonObject["role"]?.jsonObject?.get("value")?.jsonPrimitive?.content
            role == "assistant"
        }?.jsonObject?.get("id")?.jsonPrimitive?.content ?: error("No assistant message found")
        
        // Find the most recent user message ID
        val userMessageId = messages.firstOrNull { msg ->
            val role = msg.jsonObject["role"]?.jsonObject?.get("value")?.jsonPrimitive?.content
            role == "user"
        }?.jsonObject?.get("id")?.jsonPrimitive?.content ?: error("No user message found")
        
        // Regenerate the assistant message (AI response)
        println("=== Regenerating AI Response ===")
        
        val regenerated = client.regenerateChatMessage(chatId, assistantMessageId)
        println("✓ AI response regenerated")
        println("  New responses: ${regenerated["responses"]}\n")
        
        // Update a message
        println("=== Updating Message ===")
        
        val updateRequest = buildJsonObject {
            put("content", "Can you tell me more about ekoDB features?")
        }
        client.updateChatMessage(chatId, userMessageId, updateRequest)
        println("✓ Updated message content\n")
        
        // Toggle forgotten status
        println("=== Toggling Forgotten Status ===")
        
        try {
            val forgottenRequest = buildJsonObject {
                put("forgotten", true)
            }
            client.toggleForgottenMessage(chatId, userMessageId, forgottenRequest)
            println("✓ Marked message as forgotten (excluded from context)\n")
        } catch (e: Exception) {
            println("⚠ Toggle forgotten failed (connection error): ${e.message}\n")
        }
        
        // Create second session for merging
        println("=== Creating Second Session for Merge ===")
        
        val session2 = client.createChatSession(sessionRequest)
        val chatId2 = session2["chat_id"]?.jsonPrimitive?.content ?: error("No chat_id in response")
        println("✓ Created second session: $chatId2\n")
        
        // Merge sessions
        println("=== Merging Sessions ===")
        
        val mergeRequest = buildJsonObject {
            putJsonArray("source_chat_ids") {
                add(kotlinx.serialization.json.JsonPrimitive(chatId2))
            }
            put("target_chat_id", chatId)
            put("merge_strategy", "Chronological")
        }
        
        try {
            val merged = client.mergeChatSessions(mergeRequest)
            println("✓ Merged sessions")
            println("  Merged session ID: ${merged["chat_id"]}\n")
        } catch (e: Exception) {
            println("⚠ Merge failed (may not be supported): ${e.message}\n")
        }
        
        // Delete a specific message
        println("=== Deleting Message ===")
        
        client.deleteChatMessage(chatId, userMessageId)
        println("✓ Deleted message\n")
        
        // Cleanup
        println("=== Cleanup ===")
        client.deleteChatSession(chatId)
        client.deleteChatSession(chatId2)
        println("✓ Deleted chat sessions")
        
    } finally {
        try {
            client.deleteCollection(collection)
            println("✓ Deleted collection: $collection")
        } catch (e: Exception) {
            println("⚠ Could not delete collection: ${e.message}")
        }
        
        client.close()
        println("\n✓ Advanced chat features example completed successfully")
    }
}
