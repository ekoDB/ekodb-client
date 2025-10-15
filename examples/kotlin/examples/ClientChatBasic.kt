package io.ekodb.client.examples

import io.ekodb.client.EkoDBClient
import io.ekodb.client.types.Record
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.buildJsonObject
import kotlinx.serialization.json.jsonPrimitive
import kotlinx.serialization.json.put
import kotlinx.serialization.json.putJsonArray
import kotlinx.serialization.json.putJsonObject

/**
 * Basic Chat example - Simple AI chat with context search
 */
fun main() = runBlocking {
    val baseUrl = System.getenv("API_BASE_URL") ?: "http://localhost:8080"
    val apiKey = System.getenv("API_BASE_KEY") ?: error("API_BASE_KEY environment variable not set")
    
    val client = EkoDBClient.builder()
        .baseUrl(baseUrl)
        .apiKey(apiKey)
        .build()
    
    val collection = "kotlin_chat_basic_example"
    
    println("=== ekoDB Kotlin Client - Basic Chat Example ===\n")
    
    try {
        // Step 1: Insert sample data for context
        println("=== Inserting Sample Data ===")
        
        val doc1 = Record.new()
            .insert("title", "Introduction to ekoDB")
            .insert("content", "ekoDB is a high-performance database with intelligent caching, real-time capabilities, and AI integration.")
            .insert("category", "documentation")
        client.insert(collection, doc1)
        
        val doc2 = Record.new()
            .insert("title", "Search Features")
            .insert("content", "ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval.")
            .insert("category", "features")
        client.insert(collection, doc2)
        
        val doc3 = Record.new()
            .insert("title", "AI Chat Integration")
            .insert("content", "The chat feature allows you to query your database using natural language and get AI-powered responses with relevant context.")
            .insert("category", "features")
        client.insert(collection, doc3)
        
        println("✓ Inserted 3 sample documents\n")
        
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
            put("system_prompt", "You are a helpful assistant for ekoDB documentation.")
            put("max_context_messages", 10)
            put("bypass_ripple", false)
        }
        
        val session = client.createChatSession(sessionRequest)
        val chatId = session["chat_id"]?.jsonPrimitive?.content ?: error("No chat_id in response")
        println("✓ Created session: $chatId\n")
        
        // Step 3: Send a chat message
        println("=== Sending Chat Message ===")
        
        val messageRequest = buildJsonObject {
            put("message", "What is ekoDB and what features does it have?")
        }
        
        val response = client.chatMessage(chatId, messageRequest)
        println("✓ Chat response:")
        println("  Message ID: ${response["message_id"]}")
        println("  Responses: ${response["responses"]}\n")
        
        // Step 4: Cleanup
        println("=== Cleanup ===")
        client.deleteChatSession(chatId)
        println("✓ Deleted chat session")
        
    } finally {
        try {
            client.deleteCollection(collection)
            println("✓ Deleted collection: $collection")
        } catch (e: Exception) {
            println("⚠ Could not delete collection: ${e.message}")
        }
        
        client.close()
        println("\n✓ Basic chat example completed successfully")
    }
}
