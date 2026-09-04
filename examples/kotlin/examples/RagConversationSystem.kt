/**
 * RAG Conversation System Example
 *
 * This example demonstrates building a complete RAG (Retrieval-Augmented Generation) system
 * where AI can search across all previous conversations to provide context-aware responses.
 *
 * Key Features:
 * - Store messages with automatic embeddings
 * - Search related messages across all conversations
 * - Use search results as context for AI responses
 * - Dynamic conversation configurations
 * - Cross-conversation knowledge retrieval
 *
 * This showcases how ekoDB can be the complete backend for a self-improving AI chat system
 * that learns from its own history.
 *
 * Prerequisites:
 * - Run the ekoDB server: make run
 * - Set OPENAI_API_KEY environment variable
 *
 * Run with: ./gradlew run --args="RagConversationSystem"
 */

package io.ekodb.client.examples

import io.ekodb.client.EkoDBClient
import io.github.cdimascio.dotenv.dotenv
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.*
import kotlin.system.measureTimeMillis

data class Message(
    val conversationId: String,
    val role: String,
    val content: String,
    val embedding: List<Double>,
    val tags: String,
    val timestamp: String
)

fun extractStringField(record: JsonObject, field: String): String {
    val value = record[field] ?: return "N/A"
    return when {
        value is JsonPrimitive && value.isString -> value.content
        value is JsonObject && value["value"] is JsonPrimitive -> value["value"]?.jsonPrimitive?.content ?: "N/A"
        else -> value.toString()
    }
}

suspend fun createConversation(client: EkoDBClient, collection: String, convId: String, title: String) {
    val record = io.ekodb.client.types.Record()
    record.insert("conversation_id", convId)
    record.insert("title", title)
    record.insert("created_at", System.currentTimeMillis().toString())
    
    val searchConfig = mapOf(
        "collections" to io.ekodb.client.types.FieldType.array(listOf(io.ekodb.client.types.FieldType.string("rag_messages"))),
        "search_type" to io.ekodb.client.types.FieldType.string("hybrid"),
        "limit" to io.ekodb.client.types.FieldType.integer(10)
    )
    record["search_config"] = io.ekodb.client.types.FieldType.obj(searchConfig)
    
    client.insert(collection, record)
}

suspend fun storeMessageWithEmbedding(
    client: EkoDBClient,
    collection: String,
    conversationId: String,
    role: String,
    content: String,
    tags: List<String>
) {
    println("  → Calling ekoDB embed() helper...")
    println("    • Using model: text-embedding-3-small")
    println("    • Text length: ${content.length} characters")
    println("    • Behind the scenes: Creating temp Function with Embed operation")

    val embedding: List<Double>
    val duration = measureTimeMillis {
        embedding = client.embed(content, "text-embedding-3-small")
    }
    
    println("    ✓ Generated embedding: ${embedding.size} dimensions in ${duration / 1000.0}s")
    println("    • Function auto-cleaned up by client")

    val msg = io.ekodb.client.types.Record()
    msg.insert("conversation_id", conversationId)
    msg.insert("role", role)
    msg.insert("content", content)
    msg["embedding"] = io.ekodb.client.types.FieldType.array(embedding.map { 
        io.ekodb.client.types.FieldType.float(it) 
    })
    msg.insert("tags", tags.joinToString(","))
    msg.insert("timestamp", System.currentTimeMillis().toString())

    client.insert(collection, msg)
}

fun main() = runBlocking {
    println("=== ekoDB RAG Conversation System ===\n")
    println("This example shows how ekoDB can power a self-improving AI system")
    println("that learns from its own conversation history.\n")

    // Create client
    val dotenv = dotenv()
    val baseUrl = dotenv["API_BASE_URL"] ?: "http://localhost:8080"
    val apiKey = dotenv["API_BASE_KEY"] ?: "a-test-api-key-from-ekodb"
    
    val client = EkoDBClient.builder()
        .baseUrl(baseUrl)
        .apiKey(apiKey)
        .build()

    val messagesCollection = "rag_messages"
    val conversationsCollection = "rag_conversations"

    // Cleanup any existing data
    try { client.deleteCollection(messagesCollection) } catch (_: Exception) {}
    try { client.deleteCollection(conversationsCollection) } catch (_: Exception) {}

    // ========================================
    // STEP 1: Simulate Historical Conversations
    // ========================================
    println("=== Step 1: Building Conversation History ===")
    println("Storing previous conversations with embeddings...\n")

    // Conversation 1: Rust Programming Discussion
    val conv1Id = "conv_rust_programming"
    createConversation(client, conversationsCollection, conv1Id, "Rust Programming")

    val rustMessages = listOf(
        "user" to "What are the key features of Rust?",
        "assistant" to "Rust's key features include: memory safety without garbage collection, zero-cost abstractions, ownership system, powerful type system, and excellent concurrency support.",
        "user" to "How does the borrow checker work?",
        "assistant" to "The borrow checker enforces Rust's ownership rules at compile time. It ensures that references don't outlive the data they point to and prevents data races by allowing either multiple immutable references or one mutable reference."
    )

    for ((role, content) in rustMessages) {
        storeMessageWithEmbedding(client, messagesCollection, conv1Id, role, content, listOf("rust", "programming"))
    }
    println("✓ Stored Rust programming conversation (${rustMessages.size} messages)")

    // Conversation 2: Database Design Discussion
    val conv2Id = "conv_database_design"
    createConversation(client, conversationsCollection, conv2Id, "Database Design")

    val dbMessages = listOf(
        "user" to "What is database normalization?",
        "assistant" to "Database normalization is the process of organizing data to reduce redundancy and improve data integrity. It involves dividing large tables into smaller ones and defining relationships between them using foreign keys.",
        "user" to "When should I use NoSQL over SQL?",
        "assistant" to "Use NoSQL when you need: flexible schemas, horizontal scaling, high write throughput, or when working with unstructured data. SQL is better for complex queries, ACID transactions, and structured data with well-defined relationships."
    )

    for ((role, content) in dbMessages) {
        storeMessageWithEmbedding(client, messagesCollection, conv2Id, role, content, listOf("database", "design"))
    }
    println("✓ Stored database design conversation (${dbMessages.size} messages)")

    // Conversation 3: Performance Optimization
    val conv3Id = "conv_performance"
    createConversation(client, conversationsCollection, conv3Id, "Performance Optimization")

    val perfMessages = listOf(
        "user" to "How can I optimize database queries?",
        "assistant" to "Key database optimization techniques: use indexes wisely, avoid SELECT *, optimize JOIN operations, use query caching, denormalize when needed, and analyze query execution plans.",
        "user" to "What about memory management in Rust?",
        "assistant" to "Rust's ownership system provides zero-cost memory management. Use Box for heap allocation, Rc/Arc for shared ownership, and avoid cloning large data structures. The compiler optimizes away unnecessary allocations."
    )

    for ((role, content) in perfMessages) {
        storeMessageWithEmbedding(client, messagesCollection, conv3Id, role, content, listOf("performance", "optimization"))
    }
    println("✓ Stored performance optimization conversation (${perfMessages.size} messages)\n")

    // ========================================
    // STEP 2: New User Question - RAG in Action
    // ========================================
    println("=== Step 2: New User Question with Context Retrieval ===")
    val userQuestion = "How do I write memory-safe high-performance database code?"
    println("User asks: \"$userQuestion\"\n")

    // ========================================
    // STEP 3: Search Across ALL Previous Conversations
    // ========================================
    println("=== Step 3: Searching Related Context ===")
    println("Using hybrid search to find relevant messages from all conversations...\n")

    // Generate embedding for the question
    println("\n→ Generating embedding for user question...")
    println("  → Calling ekoDB embed() helper...")
    println("    • Using model: text-embedding-3-small")
    println("    • Text length: ${userQuestion.length} characters")
    println("    • Behind the scenes: Creating temp Function with Embed operation")

    val questionEmbedding: List<Double>
    val embedDuration = measureTimeMillis {
        questionEmbedding = client.embed(userQuestion, "text-embedding-3-small")
    }
    println("    ✓ Generated embedding: ${questionEmbedding.size} dimensions in ${embedDuration / 1000.0}s")
    println("    • Function auto-cleaned up by client")

    println("\n→ Executing hybridSearch()...")
    println("  • Collection: $messagesCollection")
    println("  • Query text: \"$userQuestion\"")
    println("  • Vector dimensions: ${questionEmbedding.size}")
    println("  • Limit: 5 results")
    println("  • Search type: Semantic (vector) + Keyword (text)")
    println("  • Server combines both scores for relevance ranking")

    val relatedMessages: List<JsonObject>
    val searchDuration = measureTimeMillis {
        relatedMessages = client.hybridSearch(messagesCollection, userQuestion, questionEmbedding, 5)
    }
    
    println("  ✓ Search completed in ${searchDuration / 1000.0}s")
    println("\n✓ Found ${relatedMessages.size} related messages across all conversations:")

    val contextMessages = mutableListOf<String>()
    relatedMessages.forEachIndexed { index, msg ->
        val content = extractStringField(msg, "content")
        val convId = extractStringField(msg, "conversation_id")
        val score = msg["_score"]?.jsonPrimitive?.doubleOrNull ?: 0.0

        println("  ${index + 1}. [Score: ${"%.3f".format(score)}] From $convId")
        println("     $content\n")

        contextMessages.add(content)
    }

    // ========================================
    // STEP 4: Build Context-Aware AI Response
    // ========================================
    println("=== Step 4: Generating Context-Aware Response ===")

    val context = "Here is relevant information from previous conversations:\n\n" +
        contextMessages.mapIndexed { i, msg -> "Context ${i + 1}: $msg" }.joinToString("\n\n")

    println("✓ Context prepared from search results")
    println("✓ AI would use this context to generate comprehensive response\n")

    // ========================================
    // STEP 5: Store New Conversation
    // ========================================
    println("=== Step 5: Storing New Conversation ===")

    val newConvId = "conv_new_question"
    createConversation(client, conversationsCollection, newConvId, "Memory-Safe Database Code")

    storeMessageWithEmbedding(client, messagesCollection, newConvId, "user", userQuestion, 
        listOf("rust", "database", "performance"))

    println("✓ New conversation stored and indexed for future retrieval\n")

    // ========================================
    // STEP 6: Demonstrate Cross-Conversation Search
    // ========================================
    println("=== Step 6: Cross-Conversation Search ===")
    println("Searching for messages about 'ownership' across ALL conversations...\n")

    println("\n→ Executing textSearch()...")
    println("  • Collection: $messagesCollection")
    println("  • Query: \"ownership system\"")
    println("  • Limit: 3 results")
    println("  • Search method: Full-text with fuzzy matching & stemming")
    println("  • No vector embeddings needed - pure keyword search")

    val ownershipResults: List<JsonObject>
    val textDuration = measureTimeMillis {
        ownershipResults = client.textSearch(messagesCollection, "ownership system", 3)
    }
    println("  ✓ Text search completed in ${textDuration / 1000.0}s")

    println("\n✓ Found ${ownershipResults.size} messages mentioning ownership:")
    ownershipResults.forEachIndexed { index, msg ->
        val content = extractStringField(msg, "content")
        val convId = extractStringField(msg, "conversation_id")
        println("  ${index + 1}. From $convId: $content\n")
    }

    // ========================================
    // STEP 7: Show System Statistics
    // ========================================
    println("=== System Statistics ===\n")

    println("→ Querying database statistics...")
    println("  • Using findAllWithLimit() helper - simplified query API\n")

    val totalMessages = client.findAllWithLimit(messagesCollection, 1000)
    val totalConvs = client.findAllWithLimit(conversationsCollection, 100)

    println("📊 Database Statistics:")
    println("  • Total conversations: ${totalConvs.size}")
    println("  • Total messages stored: ${totalMessages.size}")
    println("  • All messages indexed for vector search ✓")
    println("  • All messages indexed for text search ✓")
    println("  • All messages queryable by metadata ✓\n")

    // ========================================
    // STEP 8: Demonstrate Dynamic Configuration
    // ========================================
    println("=== Step 8: Dynamic Search Configuration ===")
    println("Each conversation can have its own search config...\n")

    println("💡 Conversations can store custom search configurations:")
    println("  • Search type: hybrid, text, or vector")
    println("  • Relevance thresholds")
    println("  • Filter by tags or metadata")
    println("  • Collection-specific settings")
    println("  • Per-conversation AI behavior")
    println("\nThis enables context-aware search tuned to each conversation's needs!\n")

    // ========================================
    // Cleanup
    // ========================================
    println("=== Cleanup ===")
    client.deleteCollection(messagesCollection)
    client.deleteCollection(conversationsCollection)
    println("✓ Cleanup complete\n")

    // ========================================
    // Summary
    // ========================================
    println("\n=== 📚 Summary: What This Example Showed ===\n")
    println("🔧 ekoDB Native Capabilities Used:")
    println("  ✓ Functions with Embed operation (AI integration)")
    println("  ✓ Hybrid Search (text + vector combined)")
    println("  ✓ Text Search (full-text with stemming)")
    println("  ✓ Automatic embedding generation")
    println("  ✓ Cross-collection queries\n")
    println("🚀 New Client Helper Methods:")
    println("  • client.embed(text, model) - Generate embeddings")
    println("  • client.hybridSearch() - Semantic + keyword search")
    println("  • client.textSearch() - Full-text search")
    println("  • client.findAllWithLimit() - Query all documents\n")
    println("💡 Key Takeaways:")
    println("  1. ekoDB handles AI Functions natively - no external services needed")
    println("  2. One-line embedding generation with auto-cleanup")
    println("  3. Hybrid search combines semantic understanding + keyword matching")
    println("  4. Perfect for RAG: store, search, and retrieve context")
    println("  5. All AI capabilities accessible through simple client methods\n")
    println("🎯 Build production RAG systems with ekoDB!")
    println("   → Set OPENAI_API_KEY in your ekoDB server environment")
    println("   → Use these client helpers to make AI integration simple")
    println("   → Scale to millions of documents with native indexing\n")

    client.close()
}
