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
 * Run with: npm run rag_conversation_system
 */

import { EkoDBClient } from "@ekodb/ekodb-client";
import * as dotenv from "dotenv";

dotenv.config();

interface Message {
  conversation_id: string;
  role: string;
  content: string;
  embedding: number[];
  tags: string;
  timestamp: string;
}

function extractStringField(record: any, field: string): string {
  const value = record[field];
  if (typeof value === "object" && value?.value) {
    return String(value.value);
  }
  return value ? String(value) : "N/A";
}

async function createConversation(
  client: EkoDBClient,
  collection: string,
  convId: string,
  title: string,
): Promise<void> {
  const conv = {
    id: convId,
    title,
    created_at: new Date().toISOString(),
    search_config: {
      collections: ["rag_messages"],
      search_type: "hybrid",
      limit: 10,
    },
  };
  await client.insert(collection, conv);
}

async function generateEmbedding(
  client: EkoDBClient,
  text: string,
): Promise<number[]> {
  console.log("  → Calling ekoDB embed() helper...");
  console.log("    • Using model: text-embedding-3-small");
  console.log(`    • Text length: ${text.length} characters`);
  console.log(
    "    • Behind the scenes: Creating temp Function with Embed operation",
  );

  const start = Date.now();
  const embedding = await client.embed(text, "text-embedding-3-small");
  const duration = (Date.now() - start) / 1000;

  console.log(
    `    ✓ Generated embedding: ${embedding.length} dimensions in ${duration.toFixed(3)}s`,
  );
  console.log("    • Function auto-cleaned up by client");

  return embedding;
}

async function storeMessageWithEmbedding(
  client: EkoDBClient,
  collection: string,
  conversationId: string,
  role: string,
  content: string,
  tags: string[],
): Promise<void> {
  const embedding = await generateEmbedding(client, content);

  const msg: Message = {
    conversation_id: conversationId,
    role,
    content,
    embedding,
    tags: tags.join(","),
    timestamp: new Date().toISOString(),
  };

  await client.insert(collection, msg);
}

async function main() {
  console.log("=== ekoDB RAG Conversation System ===\n");
  console.log(
    "This example shows how ekoDB can power a self-improving AI system",
  );
  console.log("that learns from its own conversation history.\n");

  // Create client
  const client = new EkoDBClient({
    baseURL: process.env.API_BASE_URL || "http://localhost:8080",
    apiKey: process.env.API_BASE_KEY || "a-test-api-key-from-ekodb",
  });

  const messagesCollection = "rag_messages";
  const conversationsCollection = "rag_conversations";

  // Cleanup any existing data
  try {
    await client.deleteCollection(messagesCollection);
    await client.deleteCollection(conversationsCollection);
  } catch {
    // Ignore if collections don't exist
  }

  // ========================================
  // STEP 1: Simulate Historical Conversations
  // ========================================
  console.log("=== Step 1: Building Conversation History ===");
  console.log("Storing previous conversations with embeddings...\n");

  // Conversation 1: Rust Programming Discussion
  const conv1Id = "conv_rust_programming";
  await createConversation(
    client,
    conversationsCollection,
    conv1Id,
    "Rust Programming",
  );

  const rustMessages: Array<[string, string]> = [
    ["user", "What are the key features of Rust?"],
    [
      "assistant",
      "Rust's key features include: memory safety without garbage collection, " +
        "zero-cost abstractions, ownership system, powerful type system, and excellent " +
        "concurrency support.",
    ],
    ["user", "How does the borrow checker work?"],
    [
      "assistant",
      "The borrow checker enforces Rust's ownership rules at compile time. It ensures " +
        "that references don't outlive the data they point to and prevents data races by " +
        "allowing either multiple immutable references or one mutable reference.",
    ],
  ];

  for (const [role, content] of rustMessages) {
    await storeMessageWithEmbedding(
      client,
      messagesCollection,
      conv1Id,
      role,
      content,
      ["rust", "programming"],
    );
  }
  console.log(
    `✓ Stored Rust programming conversation (${rustMessages.length} messages)`,
  );

  // Conversation 2: Database Design Discussion
  const conv2Id = "conv_database_design";
  await createConversation(
    client,
    conversationsCollection,
    conv2Id,
    "Database Design",
  );

  const dbMessages: Array<[string, string]> = [
    ["user", "What is database normalization?"],
    [
      "assistant",
      "Database normalization is the process of organizing data to reduce redundancy " +
        "and improve data integrity. It involves dividing large tables into smaller ones " +
        "and defining relationships between them using foreign keys.",
    ],
    ["user", "When should I use NoSQL over SQL?"],
    [
      "assistant",
      "Use NoSQL when you need: flexible schemas, horizontal scaling, high write " +
        "throughput, or when working with unstructured data. SQL is better for complex " +
        "queries, ACID transactions, and structured data with well-defined relationships.",
    ],
  ];

  for (const [role, content] of dbMessages) {
    await storeMessageWithEmbedding(
      client,
      messagesCollection,
      conv2Id,
      role,
      content,
      ["database", "design"],
    );
  }
  console.log(
    `✓ Stored database design conversation (${dbMessages.length} messages)`,
  );

  // Conversation 3: Performance Optimization
  const conv3Id = "conv_performance";
  await createConversation(
    client,
    conversationsCollection,
    conv3Id,
    "Performance Optimization",
  );

  const perfMessages: Array<[string, string]> = [
    ["user", "How can I optimize database queries?"],
    [
      "assistant",
      "Key database optimization techniques: use indexes wisely, avoid SELECT *, " +
        "optimize JOIN operations, use query caching, denormalize when needed, and " +
        "analyze query execution plans.",
    ],
    ["user", "What about memory management in Rust?"],
    [
      "assistant",
      "Rust's ownership system provides zero-cost memory management. Use Box for " +
        "heap allocation, Rc/Arc for shared ownership, and avoid cloning large data " +
        "structures. The compiler optimizes away unnecessary allocations.",
    ],
  ];

  for (const [role, content] of perfMessages) {
    await storeMessageWithEmbedding(
      client,
      messagesCollection,
      conv3Id,
      role,
      content,
      ["performance", "optimization"],
    );
  }
  console.log(
    `✓ Stored performance optimization conversation (${perfMessages.length} messages)\n`,
  );

  // ========================================
  // STEP 2: New User Question - RAG in Action
  // ========================================
  console.log("=== Step 2: New User Question with Context Retrieval ===");
  const userQuestion =
    "How do I write memory-safe high-performance database code?";
  console.log(`User asks: "${userQuestion}"\n`);

  // ========================================
  // STEP 3: Search Across ALL Previous Conversations
  // ========================================
  console.log("=== Step 3: Searching Related Context ===");
  console.log(
    "Using hybrid search to find relevant messages from all conversations...\n",
  );

  // Generate embedding for the question
  console.log("\n→ Generating embedding for user question...");
  const questionEmbedding = await generateEmbedding(client, userQuestion);

  console.log("\n→ Executing hybridSearch()...");
  console.log(`  • Collection: ${messagesCollection}`);
  console.log(`  • Query text: "${userQuestion}"`);
  console.log(`  • Vector dimensions: ${questionEmbedding.length}`);
  console.log("  • Limit: 5 results");
  console.log("  • Search type: Semantic (vector) + Keyword (text)");
  console.log("  • Server combines both scores for relevance ranking");

  const searchStart = Date.now();
  const relatedMessages = await client.hybridSearch(
    messagesCollection,
    userQuestion,
    questionEmbedding,
    5,
  );
  const searchDuration = (Date.now() - searchStart) / 1000;
  console.log(`  ✓ Search completed in ${searchDuration.toFixed(3)}s`);

  console.log(
    `✓ Found ${relatedMessages.length} related messages across all conversations:`,
  );

  const contextMessages: string[] = [];
  for (let i = 0; i < relatedMessages.length; i++) {
    const msg = relatedMessages[i];
    const content = extractStringField(msg, "content");
    const convId = extractStringField(msg, "conversation_id");
    const score = msg._score || 0;

    console.log(`  ${i + 1}. [Score: ${score.toFixed(3)}] From ${convId}`);
    console.log(`     ${content}\n`);

    contextMessages.push(content);
  }

  // ========================================
  // STEP 4: Build Context-Aware AI Response
  // ========================================
  console.log("=== Step 4: Generating Context-Aware Response ===");

  // Prepare context from search results
  const context =
    "Here is relevant information from previous conversations:\n\n" +
    contextMessages.map((msg, i) => `Context ${i + 1}: ${msg}`).join("\n\n");

  // Create chat session for the new question
  const chatSession = await client.createChatSession({
    collections: [],
    llm_provider: "openai",
    llm_model: "gpt-4",
    system_prompt:
      "You are a helpful programming assistant. Use the provided context " +
      "to give comprehensive answers that combine knowledge from multiple " +
      `topics. Context:\n\n${context}`,
  });

  // Send the question
  const response = await client.chatMessage(chatSession.chat_id, {
    message: userQuestion,
  });

  console.log("✓ AI Response (with context from 3 conversations):\n");
  if (response.responses?.length > 0) {
    console.log(`${response.responses[0]}\n`);
  }

  // ========================================
  // STEP 5: Store New Conversation
  // ========================================
  console.log("=== Step 5: Storing New Conversation ===");

  const newConvId = "conv_new_question";
  await createConversation(
    client,
    conversationsCollection,
    newConvId,
    "Memory-Safe Database Code",
  );

  // Store user question
  await storeMessageWithEmbedding(
    client,
    messagesCollection,
    newConvId,
    "user",
    userQuestion,
    ["rust", "database", "performance"],
  );

  // Store AI response
  if (response.responses?.length > 0) {
    await storeMessageWithEmbedding(
      client,
      messagesCollection,
      newConvId,
      "assistant",
      response.responses[0],
      ["rust", "database", "performance"],
    );
  }

  console.log("✓ New conversation stored and indexed for future retrieval\n");

  // ========================================
  // STEP 6: Demonstrate Cross-Conversation Search
  // ========================================
  console.log("=== Step 6: Cross-Conversation Search ===");
  console.log(
    "Searching for messages about 'ownership' across ALL conversations...\n",
  );

  console.log("\n→ Executing textSearch()...");
  console.log(`  • Collection: ${messagesCollection}`);
  console.log(`  • Query: "ownership system"`);
  console.log("  • Limit: 3 results");
  console.log("  • Search method: Full-text with fuzzy matching & stemming");
  console.log("  • No vector embeddings needed - pure keyword search");

  const textStart = Date.now();
  const ownershipResults = await client.textSearch(
    messagesCollection,
    "ownership system",
    { limit: 3 },
  );
  const textDuration = (Date.now() - textStart) / 1000;
  console.log(`  ✓ Text search completed in ${textDuration.toFixed(3)}s`);

  console.log(
    `✓ Found ${ownershipResults.results.length} messages mentioning ownership:`,
  );
  for (let i = 0; i < ownershipResults.results.length; i++) {
    const msg = ownershipResults.results[i].record;
    const content = extractStringField(msg, "content");
    const convId = extractStringField(msg, "conversation_id");
    console.log(`  ${i + 1}. From ${convId}: ${content}\n`);
  }

  // ========================================
  // STEP 7: Show System Statistics
  // ========================================
  console.log("=== System Statistics ===\n");

  console.log("→ Querying database statistics...");
  console.log("  • Using findAllWithLimit() helper - simplified query API\n");

  const totalMessages = await client.findAllWithLimit(messagesCollection, 1000);
  const totalConvs = await client.findAllWithLimit(
    conversationsCollection,
    100,
  );

  console.log("📊 Database Statistics:");

  console.log(`  • Total conversations: ${totalConvs.length}`);
  console.log(`  • Total messages stored: ${totalMessages.length}`);
  console.log("  • All messages indexed for vector search ✓");
  console.log("  • All messages indexed for text search ✓");
  console.log("  • All messages queryable by metadata ✓\n");

  // ========================================
  // STEP 8: Demonstrate Dynamic Configuration
  // ========================================
  console.log("=== Step 8: Dynamic Search Configuration ===");
  console.log("Each conversation can have its own search config...\n");

  console.log("💡 Conversations can store custom search configurations:");
  console.log("  • Search type: hybrid, text, or vector");
  console.log("  • Relevance thresholds");
  console.log("  • Filter by tags or metadata");
  console.log("  • Collection-specific settings");
  console.log("  • Per-conversation AI behavior");
  console.log(
    "\nThis enables context-aware search tuned to each conversation's needs!\n",
  );

  // ========================================
  // Cleanup
  // ========================================
  console.log("=== Cleanup ===");
  await client.deleteChatSession(chatSession.chat_id);
  await client.deleteCollection(messagesCollection);
  await client.deleteCollection(conversationsCollection);
  console.log("✓ Cleanup complete\n");

  // ========================================
  // Summary
  // ========================================
  console.log("\n=== 📚 Summary: What This Example Showed ===\n");
  console.log("🔧 ekoDB Native Capabilities Used:");
  console.log("  ✓ Functions with Embed operation (AI integration)");
  console.log("  ✓ Hybrid Search (text + vector combined)");
  console.log("  ✓ Text Search (full-text with stemming)");
  console.log("  ✓ Automatic embedding generation");
  console.log("  ✓ Cross-collection queries\n");
  console.log("🚀 New Client Helper Methods:");
  console.log("  • client.embed(text, model) - Generate embeddings");
  console.log("  • client.hybridSearch() - Semantic + keyword search");
  console.log("  • client.textSearch() - Full-text search");
  console.log("  • client.findAllWithLimit() - Query all documents\n");
  console.log("💡 Key Takeaways:");
  console.log(
    "  1. ekoDB handles AI Functions natively - no external services needed",
  );
  console.log("  2. One-line embedding generation with auto-cleanup");
  console.log(
    "  3. Hybrid search combines semantic understanding + keyword matching",
  );
  console.log("  4. Perfect for RAG: store, search, and retrieve context");
  console.log(
    "  5. All AI capabilities accessible through simple client methods\n",
  );
  console.log("🎯 Build production RAG systems with ekoDB!");
  console.log("   → Set OPENAI_API_KEY in your ekoDB server environment");
  console.log("   → Use these client helpers to make AI integration simple");
  console.log("   → Scale to millions of documents with native indexing\n");
}

main().catch(console.error);
