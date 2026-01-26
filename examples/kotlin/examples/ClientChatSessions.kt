package io.ekodb.client.examples

import io.ekodb.client.EkoDBClient
import io.ekodb.client.types.Record
import io.github.cdimascio.dotenv.dotenv
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.buildJsonObject
import kotlinx.serialization.json.jsonPrimitive
import kotlinx.serialization.json.put
import kotlinx.serialization.json.putJsonArray
import kotlinx.serialization.json.putJsonObject

/**
 * Chat Sessions example - Session management and history
 */
fun main() = runBlocking {
    val dotenv = dotenv()
    val baseUrl = dotenv["API_BASE_URL"] ?: "http://localhost:8080"
    val apiKey = dotenv["API_BASE_KEY"] ?: "a-test-api-key-from-ekodb"
    
    val client = EkoDBClient.builder()
        .baseUrl(baseUrl)
        .apiKey(apiKey)
        .build()
    
    val collection = "kotlin_chat_sessions_example"
    
    println("=== ekoDB Kotlin Client - Chat Session Management Example ===\n")
    
    try {
        // Step 1: Insert sample data
        println("=== Inserting Sample Data ===")
        
        val doc = Record.new()
            .insert("product", "ekoDB")
            .insert("description", "A high-performance database product with AI capabilities")
            .insert("price", 99)
        client.insert(collection, doc)
        println("✓ Inserted sample product\n")
        
        // Step 2: Create a chat session
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
            put("system_prompt", "You are a helpful assistant for product information.")
        }
        
        val session = client.createChatSession(sessionRequest)
        val chatId = session["chat_id"]?.jsonPrimitive?.content ?: error("No chat_id in response")
        println("✓ Created session: $chatId\n")
        
        // Step 3: Send messages
        println("=== Sending Messages ===")
        
        val msg1Request = buildJsonObject {
            put("message", "What products are available?")
        }
        val msg1 = client.chatMessage(chatId, msg1Request)
        println("✓ Message 1 sent")
        println("  Responses: ${msg1["responses"]}\n")
        
        val msg2Request = buildJsonObject {
            put("message", "What is the price?")
        }
        val msg2 = client.chatMessage(chatId, msg2Request)
        println("✓ Message 2 sent")
        println("  Responses: ${msg2["responses"]}\n")
        
        // Step 4: Get message history
        println("=== Getting Message History ===")
        
        val messages = client.getChatSessionMessages(chatId)
        println("✓ Retrieved message history")
        println("  Total messages: ${messages["messages"]}\n")
        
        // Step 5: Update session
        println("=== Updating Session ===")
        
        val updateRequest = buildJsonObject {
            put("system_prompt", "You are an expert product consultant.")
        }
        client.updateChatSession(chatId, updateRequest)
        println("✓ Updated session system prompt\n")
        
        // Step 6: List all sessions
        println("=== Listing Sessions ===")
        
        val sessions = client.listChatSessions()
        println("✓ Total sessions: ${sessions["total"]}\n")
        
        // Step 7: Branch session
        println("=== Branching Session ===")
        
        val branchRequest = buildJsonObject {
            putJsonArray("collections") {
                add(buildJsonObject {
                    put("collection_name", collection)
                    putJsonArray("fields") {}
                })
            }
            put("llm_provider", "openai")
            put("llm_model", "gpt-4")
            put("parent_id", chatId)
        }
        
        val branchedSession = client.branchChatSession(branchRequest)
        val branchedChatId = branchedSession["chat_id"]?.jsonPrimitive?.content ?: error("No chat_id in response")
        println("✓ Created branched session: $branchedChatId\n")
        
        // Cleanup
        println("=== Cleanup ===")
        client.deleteChatSession(chatId)
        client.deleteChatSession(branchedChatId)
        println("✓ Deleted chat sessions")
        
    } finally {
        try {
            client.deleteCollection(collection)
            println("✓ Deleted collection: $collection")
        } catch (e: Exception) {
            println("⚠ Could not delete collection: ${e.message}")
        }
        
        client.close()
        println("\n✓ Chat session management example completed successfully")
    }
}
