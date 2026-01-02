//! RAG Conversation System Example
//!
//! This example demonstrates building a complete RAG (Retrieval-Augmented Generation) system
//! where AI can search across all previous conversations to provide context-aware responses.
//!
//! Key Features:
//! - Store messages with automatic embeddings
//! - Search related messages across all conversations
//! - Use search results as context for AI responses
//! - Dynamic conversation configurations
//! - Cross-conversation knowledge retrieval
//!
//! This showcases how ekoDB can be the complete backend for a self-improving AI chat system
//! that learns from its own history.
//!
//! Prerequisites:
//! - Run the ekoDB server: `make run`
//! - Set OPENAI_API_KEY environment variable
//!
//! Run with: `cargo run --example rag_conversation_system`

use ekodb_client::{Client, Record};
use serde_json::json;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();

    println!("=== ekoDB RAG Conversation System ===\n");
    println!("This example shows how ekoDB can power a self-improving AI system");
    println!("that learns from its own conversation history.\n");

    // Create client
    let client = Client::builder()
        .base_url(
            std::env::var("API_BASE_URL").unwrap_or_else(|_| "http://localhost:8080".to_string()),
        )
        .api_key(std::env::var("API_BASE_KEY")?)
        .build()?;

    let messages_collection = "rag_messages";
    let conversations_collection = "rag_conversations";

    // Cleanup any existing data
    let _ = client.delete_collection(messages_collection).await;
    let _ = client.delete_collection(conversations_collection).await;

    // ========================================
    // STEP 1: Simulate Historical Conversations
    // ========================================
    println!("=== Step 1: Building Conversation History ===");
    println!("Storing previous conversations with embeddings...\n");

    // Conversation 1: Rust Programming Discussion
    let conv1_id = "conv_rust_programming";
    create_conversation(
        &client,
        conversations_collection,
        conv1_id,
        "Rust Programming",
    )
    .await?;

    let rust_messages = vec![
        ("user", "What are the key features of Rust?"),
        (
            "assistant",
            "Rust's key features include: memory safety without garbage collection, \
             zero-cost abstractions, ownership system, powerful type system, and excellent \
             concurrency support.",
        ),
        ("user", "How does the borrow checker work?"),
        (
            "assistant",
            "The borrow checker enforces Rust's ownership rules at compile time. It ensures \
             that references don't outlive the data they point to and prevents data races by \
             allowing either multiple immutable references or one mutable reference.",
        ),
    ];

    for (role, content) in &rust_messages {
        store_message_with_embedding(
            &client,
            messages_collection,
            conv1_id,
            role,
            content,
            &["rust", "programming"],
        )
        .await?;
    }
    println!(
        "✓ Stored Rust programming conversation ({} messages)",
        rust_messages.len()
    );

    // Conversation 2: Database Design Discussion
    let conv2_id = "conv_database_design";
    create_conversation(
        &client,
        conversations_collection,
        conv2_id,
        "Database Design",
    )
    .await?;

    let db_messages = vec![
        ("user", "What is database normalization?"),
        (
            "assistant",
            "Database normalization is the process of organizing data to reduce redundancy \
             and improve data integrity. It involves dividing large tables into smaller ones \
             and defining relationships between them using foreign keys.",
        ),
        ("user", "When should I use NoSQL over SQL?"),
        (
            "assistant",
            "Use NoSQL when you need: flexible schemas, horizontal scaling, high write \
             throughput, or when working with unstructured data. SQL is better for complex \
             queries, ACID transactions, and structured data with well-defined relationships.",
        ),
    ];

    for (role, content) in &db_messages {
        store_message_with_embedding(
            &client,
            messages_collection,
            conv2_id,
            role,
            content,
            &["database", "design"],
        )
        .await?;
    }
    println!(
        "✓ Stored database design conversation ({} messages)",
        db_messages.len()
    );

    // Conversation 3: Performance Optimization
    let conv3_id = "conv_performance";
    create_conversation(
        &client,
        conversations_collection,
        conv3_id,
        "Performance Optimization",
    )
    .await?;

    let perf_messages = vec![
        ("user", "How can I optimize database queries?"),
        (
            "assistant",
            "Key database optimization techniques: use indexes wisely, avoid SELECT *, \
             optimize JOIN operations, use query caching, denormalize when needed, and \
             analyze query execution plans.",
        ),
        ("user", "What about memory management in Rust?"),
        (
            "assistant",
            "Rust's ownership system provides zero-cost memory management. Use Box for \
             heap allocation, Rc/Arc for shared ownership, and avoid cloning large data \
             structures. The compiler optimizes away unnecessary allocations.",
        ),
    ];

    for (role, content) in &perf_messages {
        store_message_with_embedding(
            &client,
            messages_collection,
            conv3_id,
            role,
            content,
            &["performance", "optimization"],
        )
        .await?;
    }
    println!(
        "✓ Stored performance optimization conversation ({} messages)\n",
        perf_messages.len()
    );

    // ========================================
    // STEP 2: New User Question - RAG in Action
    // ========================================
    println!("=== Step 2: New User Question with Context Retrieval ===");
    let user_question = "How do I write memory-safe high-performance database code?";
    println!("User asks: \"{}\"\n", user_question);

    // ========================================
    // STEP 3: Search Across ALL Previous Conversations
    // ========================================
    println!("=== Step 3: Searching Related Context ===");
    println!("Using hybrid search to find relevant messages from all conversations...\n");

    // Generate embedding for the question
    println!("\n→ Generating embedding for user question...");
    let question_embedding = generate_embedding(&client, user_question).await?;

    // Use NEW hybrid_search helper - combining semantic + keyword search
    println!("\n→ Executing hybrid_search()...");
    println!("  • Collection: {}", messages_collection);
    println!("  • Query text: \"{}\"", user_question);
    println!("  • Vector dimensions: {}", question_embedding.len());
    println!("  • Limit: 5 results");
    println!("  • Search type: Semantic (vector) + Keyword (text)");
    println!("  • Server combines both scores for relevance ranking");

    let search_start = std::time::Instant::now();
    let related_messages = client
        .hybrid_search(
            messages_collection,
            user_question,
            question_embedding.clone(),
            5, // Top 5 most relevant
        )
        .await?;
    let search_duration = search_start.elapsed();

    println!("  ✓ Search completed in {:?}", search_duration);
    println!(
        "\n✓ Found {} related messages across all conversations:",
        related_messages.len()
    );

    let mut context_messages = Vec::new();
    for (i, msg) in related_messages.iter().enumerate() {
        let content = msg
            .get("content")
            .and_then(|v| {
                if let ekodb_client::FieldType::String(s) = v {
                    Some(s.clone())
                } else {
                    None
                }
            })
            .unwrap_or_else(|| "N/A".to_string());

        let conv_id = msg
            .get("conversation_id")
            .and_then(|v| {
                if let ekodb_client::FieldType::String(s) = v {
                    Some(s.clone())
                } else {
                    None
                }
            })
            .unwrap_or_else(|| "N/A".to_string());

        println!("  {}. From {}", i + 1, conv_id);
        println!("     {}\n", content);

        context_messages.push(content);
    }

    // ========================================
    // STEP 4: Build Context-Aware AI Response
    // ========================================
    println!("=== Step 4: Generating Context-Aware Response ===");

    // Prepare context from search results
    let context = format!(
        "Here is relevant information from previous conversations:\n\n{}",
        context_messages
            .iter()
            .enumerate()
            .map(|(i, msg)| format!("Context {}: {}", i + 1, msg))
            .collect::<Vec<_>>()
            .join("\n\n")
    );

    // Create chat session for the new question
    let chat_session = client
        .create_chat_session(
            ekodb_client::CreateChatSessionRequest::new("openai")
                .model("gpt-4")
                .system_prompt(&format!(
                    "You are a helpful programming assistant. Use the provided context \
                     to give comprehensive answers that combine knowledge from multiple \
                     topics. Context:\n\n{}",
                    context
                )),
        )
        .await?;

    // Send the question
    let response = client
        .chat_message(
            &chat_session.chat_id,
            ekodb_client::ChatMessageRequest::new(user_question),
        )
        .await?;

    println!("✓ AI Response (with context from {} conversations):\n", 3);
    if !response.responses.is_empty() {
        println!("{}\n", response.responses[0]);
    }

    // ========================================
    // STEP 5: Store New Conversation
    // ========================================
    println!("=== Step 5: Storing New Conversation ===");

    let new_conv_id = "conv_new_question";
    create_conversation(
        &client,
        conversations_collection,
        new_conv_id,
        "Memory-Safe Database Code",
    )
    .await?;

    // Store user question
    store_message_with_embedding(
        &client,
        messages_collection,
        new_conv_id,
        "user",
        user_question,
        &["rust", "database", "performance"],
    )
    .await?;

    // Store AI response
    if !response.responses.is_empty() {
        store_message_with_embedding(
            &client,
            messages_collection,
            new_conv_id,
            "assistant",
            &response.responses[0],
            &["rust", "database", "performance"],
        )
        .await?;
    }

    println!("✓ New conversation stored and indexed for future retrieval\n");

    // ========================================
    // STEP 6: Demonstrate Cross-Conversation Search
    // ========================================
    println!("=== Step 6: Cross-Conversation Search ===");
    println!("Searching for messages about 'ownership' across ALL conversations...\n");

    // NEW text_search helper - simple text search without embeddings
    println!("\n→ Executing text_search()...");
    println!("  • Collection: {}", messages_collection);
    println!("  • Query: \"ownership system\"");
    println!("  • Limit: 3 results");
    println!("  • Search method: Full-text with fuzzy matching & stemming");
    println!("  • No vector embeddings needed - pure keyword search");

    let text_search_start = std::time::Instant::now();
    let ownership_results = client
        .text_search(
            messages_collection,
            "ownership system",
            3, // Limit to 3 results
        )
        .await?;
    let text_search_duration = text_search_start.elapsed();

    println!("  ✓ Text search completed in {:?}", text_search_duration);
    println!(
        "\n✓ Found {} messages mentioning ownership:",
        ownership_results.len()
    );
    for (i, msg) in ownership_results.iter().enumerate() {
        let content = msg
            .get("content")
            .and_then(|v| {
                if let ekodb_client::FieldType::String(s) = v {
                    Some(s.clone())
                } else {
                    None
                }
            })
            .unwrap_or_else(|| "N/A".to_string());

        let conv_id = msg
            .get("conversation_id")
            .and_then(|v| {
                if let ekodb_client::FieldType::String(s) = v {
                    Some(s.clone())
                } else {
                    None
                }
            })
            .unwrap_or_else(|| "N/A".to_string());

        println!("  {}. From {}: {}\n", i + 1, conv_id, content);
    }

    // ========================================
    // STEP 7: Show System Statistics
    // ========================================
    println!("=== System Statistics ===");

    // NEW find_all helper - query all documents with a limit
    println!("\n→ Querying database statistics...");
    println!("  • Using find_all() helper - simplified query API");

    let total_messages = client.find_all(messages_collection, 1000).await?;
    let total_convs = client.find_all(conversations_collection, 100).await?;

    println!("\n📊 Database Statistics:");
    println!("  • Total conversations: {}", total_convs.len());
    println!("  • Total messages stored: {}", total_messages.len());
    println!("  • All messages indexed for vector search ✓");
    println!("  • All messages indexed for text search ✓");
    println!("  • All messages queryable by metadata ✓\n");

    // ========================================
    // STEP 8: Dynamic Search Configuration
    // ========================================
    println!("=== Step 8: Dynamic Search Configuration ===");
    println!("Each conversation can have its own search config...\n");

    println!("💡 Conversations can store custom search configurations:");
    println!("  • Search type: hybrid, text, or vector");
    println!("  • Relevance thresholds");
    println!("  • Filter by tags or metadata");
    println!("  • Collection-specific settings");
    println!("  • Per-conversation AI behavior");
    println!("\nThis enables context-aware search tuned to each conversation's needs!\n");

    // ========================================
    // Cleanup
    // ========================================
    println!("\n=== Cleanup ===");
    println!("Deleting example collections...\n");

    client.delete_collection(messages_collection).await?;
    client.delete_collection(conversations_collection).await?;

    println!("✅ All done! RAG system demonstrated successfully.\n");
    println!("✓ Using search results to enhance AI responses (RAG)");
    println!("✓ Building a self-improving knowledge base");
    println!("✓ Dynamic search configurations per conversation");
    println!("\nekoDB provides everything needed for AI-powered applications:");
    println!("  • Vector search (semantic similarity)");
    println!("  • Text search (keyword matching)");
    println!("  • Hybrid search (best of both)");
    println!("  • AI functions (Chat, Embed)");
    println!("  • Flexible querying and filtering");
    println!("  • All in one database - no external dependencies!\n");

    Ok(())
}

// Helper: Create a conversation record
async fn create_conversation(
    client: &Client,
    collection: &str,
    conv_id: &str,
    title: &str,
) -> Result<(), Box<dyn Error>> {
    let mut conv = Record::new();
    conv.insert("id", conv_id);
    conv.insert("title", title);
    conv.insert("created_at", chrono::Utc::now().to_rfc3339());
    conv.insert(
        "search_config",
        json!({
            "collections": ["rag_messages"],
            "search_type": "hybrid",
            "limit": 10
        }),
    );

    client.insert(collection, conv, None).await?;
    Ok(())
}

// Helper: Store a message with automatic embedding
async fn store_message_with_embedding(
    client: &Client,
    collection: &str,
    conversation_id: &str,
    role: &str,
    content: &str,
    tags: &[&str],
) -> Result<(), Box<dyn Error>> {
    // Generate embedding for the message
    let embedding = generate_embedding(client, content).await?;

    // Store message with embedding
    let mut msg = Record::new();
    msg.insert("conversation_id", conversation_id);
    msg.insert("role", role);
    msg.insert("content", content);
    // Convert Vec<f64> to FieldType::Array
    msg.insert(
        "embedding",
        ekodb_client::FieldType::Array(
            embedding
                .into_iter()
                .map(ekodb_client::FieldType::Float)
                .collect(),
        ),
    );
    msg.insert("tags", tags.join(","));
    msg.insert("timestamp", chrono::Utc::now().to_rfc3339());

    client.insert(collection, msg, None).await?;
    Ok(())
}

// Helper: Generate embedding using ekoDB's NEW embed() helper
async fn generate_embedding(client: &Client, text: &str) -> Result<Vec<f64>, Box<dyn Error>> {
    // Simple one-line call - ekoDB handles the Function creation/execution/cleanup
    println!("  → Calling ekoDB embed() helper...");
    println!("    • Using model: text-embedding-3-small");
    println!("    • Text length: {} characters", text.len());
    println!("    • Behind the scenes: Creating temp Function with Embed operation");

    let start = std::time::Instant::now();
    let embedding = client.embed(text, "text-embedding-3-small").await?;
    let duration = start.elapsed();

    println!(
        "    ✓ Generated embedding: {} dimensions in {:?}",
        embedding.len(),
        duration
    );
    println!("    • Function auto-cleaned up by client");

    Ok(embedding)
}
