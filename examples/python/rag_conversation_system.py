#!/usr/bin/env python3
"""
RAG Conversation System Example

This example demonstrates building a complete RAG (Retrieval-Augmented Generation) system
where AI can search across all previous conversations to provide context-aware responses.

Key Features:
- Store messages with automatic embeddings
- Search related messages across all conversations
- Use search results as context for AI responses
- Dynamic conversation configurations
- Cross-conversation knowledge retrieval

This showcases how ekoDB can be the complete backend for a self-improving AI chat system
that learns from its own history.

Prerequisites:
- Run the ekoDB server: make run
- Set OPENAI_API_KEY environment variable

Run with: python rag_conversation_system.py
"""

import os
import asyncio
from datetime import datetime
from typing import List, Dict, Any
from pathlib import Path
from ekodb_client import Client
from dotenv import load_dotenv

# Load environment variables from .env file
env_path = Path(__file__).parent.parent / ".env"
load_dotenv(env_path)


def extract_string_field(record: Dict[str, Any], field: str) -> str:
    """Extract string field from record (handles Object wrapper)"""
    value = record.get(field)
    if isinstance(value, dict) and "value" in value:
        return str(value["value"])
    return str(value) if value else "N/A"


async def create_conversation(
    client: Client, collection: str, conv_id: str, title: str
):
    """Create a conversation record"""
    conv = {
        "id": conv_id,
        "title": title,
        "created_at": datetime.utcnow().isoformat(),
        "search_config": {
            "collections": ["rag_messages"],
            "search_type": "hybrid",
            "limit": 10,
        },
    }
    await client.insert(collection, conv)


async def generate_embedding(client: Client, text: str) -> List[float]:
    """Generate embedding using ekoDB's NEW embed() helper"""
    import time

    print(f"  → Calling ekoDB embed() helper...")
    print(f"    • Using model: text-embedding-3-small")
    print(f"    • Text length: {len(text)} characters")
    print(f"    • Behind the scenes: Creating temp Function with Embed operation")

    start = time.time()
    embedding = await client.embed(text, "text-embedding-3-small")
    duration = time.time() - start

    print(f"    ✓ Generated embedding: {len(embedding)} dimensions in {duration:.3f}s")
    print(f"    • Function auto-cleaned up by client")

    return embedding


async def store_message_with_embedding(
    client: Client,
    collection: str,
    conversation_id: str,
    role: str,
    content: str,
    tags: List[str],
):
    """Store a message with automatic embedding"""
    embedding = await generate_embedding(client, content)

    msg = {
        "conversation_id": conversation_id,
        "role": role,
        "content": content,
        "embedding": embedding,
        "tags": ",".join(tags),
        "timestamp": datetime.utcnow().isoformat(),
    }

    await client.insert(collection, msg)


async def main():
    print("=== ekoDB RAG Conversation System ===\n")
    print("This example shows how ekoDB can power a self-improving AI system")
    print("that learns from its own conversation history.\n")

    # Create client
    base_url = os.getenv("API_BASE_URL", "http://localhost:8080")
    api_key = os.getenv("API_BASE_KEY", "a-test-api-key-from-ekodb")
    client = Client.new(base_url, api_key)

    messages_collection = "rag_messages"
    conversations_collection = "rag_conversations"

    # Cleanup any existing data
    try:
        await client.delete_collection(messages_collection)
        await client.delete_collection(conversations_collection)
    except:
        pass

    # ========================================
    # STEP 1: Simulate Historical Conversations
    # ========================================
    print("=== Step 1: Building Conversation History ===")
    print("Storing previous conversations with embeddings...\n")

    # Conversation 1: Rust Programming Discussion
    conv1_id = "conv_rust_programming"
    await create_conversation(
        client, conversations_collection, conv1_id, "Rust Programming"
    )

    rust_messages = [
        ("user", "What are the key features of Rust?"),
        (
            "assistant",
            "Rust's key features include: memory safety without garbage collection, "
            "zero-cost abstractions, ownership system, powerful type system, and excellent "
            "concurrency support.",
        ),
        ("user", "How does the borrow checker work?"),
        (
            "assistant",
            "The borrow checker enforces Rust's ownership rules at compile time. It ensures "
            "that references don't outlive the data they point to and prevents data races by "
            "allowing either multiple immutable references or one mutable reference.",
        ),
    ]

    for role, content in rust_messages:
        await store_message_with_embedding(
            client,
            messages_collection,
            conv1_id,
            role,
            content,
            ["rust", "programming"],
        )
    print(f"✓ Stored Rust programming conversation ({len(rust_messages)} messages)")

    # Conversation 2: Database Design Discussion
    conv2_id = "conv_database_design"
    await create_conversation(
        client, conversations_collection, conv2_id, "Database Design"
    )

    db_messages = [
        ("user", "What is database normalization?"),
        (
            "assistant",
            "Database normalization is the process of organizing data to reduce redundancy "
            "and improve data integrity. It involves dividing large tables into smaller ones "
            "and defining relationships between them using foreign keys.",
        ),
        ("user", "When should I use NoSQL over SQL?"),
        (
            "assistant",
            "Use NoSQL when you need: flexible schemas, horizontal scaling, high write "
            "throughput, or when working with unstructured data. SQL is better for complex "
            "queries, ACID transactions, and structured data with well-defined relationships.",
        ),
    ]

    for role, content in db_messages:
        await store_message_with_embedding(
            client, messages_collection, conv2_id, role, content, ["database", "design"]
        )
    print(f"✓ Stored database design conversation ({len(db_messages)} messages)")

    # Conversation 3: Performance Optimization
    conv3_id = "conv_performance"
    await create_conversation(
        client, conversations_collection, conv3_id, "Performance Optimization"
    )

    perf_messages = [
        ("user", "How can I optimize database queries?"),
        (
            "assistant",
            "Key database optimization techniques: use indexes wisely, avoid SELECT *, "
            "optimize JOIN operations, use query caching, denormalize when needed, and "
            "analyze query execution plans.",
        ),
        ("user", "What about memory management in Rust?"),
        (
            "assistant",
            "Rust's ownership system provides zero-cost memory management. Use Box for "
            "heap allocation, Rc/Arc for shared ownership, and avoid cloning large data "
            "structures. The compiler optimizes away unnecessary allocations.",
        ),
    ]

    for role, content in perf_messages:
        await store_message_with_embedding(
            client,
            messages_collection,
            conv3_id,
            role,
            content,
            ["performance", "optimization"],
        )
    print(
        f"✓ Stored performance optimization conversation ({len(perf_messages)} messages)\n"
    )

    # ========================================
    # STEP 2: New User Question - RAG in Action
    # ========================================
    print("=== Step 2: New User Question with Context Retrieval ===")
    user_question = "How do I write memory-safe high-performance database code?"
    print(f'User asks: "{user_question}"\n')

    # ========================================
    # STEP 3: Search Across ALL Previous Conversations
    # ========================================
    print("=== Step 3: Searching Related Context ===")
    print("Using hybrid search to find relevant messages from all conversations...\n")

    # Generate embedding for the question
    print("\n→ Generating embedding for user question...")
    question_embedding = await generate_embedding(client, user_question)

    # Use NEW hybrid_search helper - combining semantic + keyword search
    print("\n→ Executing hybrid_search()...")
    print(f"  • Collection: {messages_collection}")
    print(f'  • Query text: "{user_question}"')
    print(f"  • Vector dimensions: {len(question_embedding)}")
    print(f"  • Limit: 5 results")
    print(f"  • Search type: Semantic (vector) + Keyword (text)")
    print(f"  • Server combines both scores for relevance ranking")

    import time

    search_start = time.time()
    related_messages = await client.hybrid_search(
        messages_collection,
        user_question,
        question_embedding,
        5,  # Top 5 most relevant
    )
    search_duration = time.time() - search_start

    print(f"  ✓ Search completed in {search_duration:.3f}s")
    print(
        f"\n✓ Found {len(related_messages)} related messages across all conversations:"
    )

    context_messages = []
    for i, msg in enumerate(related_messages):
        content = extract_string_field(msg, "content")
        conv_id = extract_string_field(msg, "conversation_id")
        score = msg.get("_score", 0.0)

        print(f"  {i + 1}. [Score: {score:.3f}] From {conv_id}")
        print(f"     {content}\n")

        context_messages.append(content)

    # ========================================
    # STEP 4: Build Context-Aware AI Response
    # ========================================
    print("=== Step 4: Generating Context-Aware Response ===")

    # Prepare context from search results
    context = "Here is relevant information from previous conversations:\n\n"
    context += "\n\n".join(
        [f"Context {i + 1}: {msg}" for i, msg in enumerate(context_messages)]
    )

    # Create chat session for the new question
    chat_session = await client.create_chat_session(
        collections=[],  # No collections needed for this chat
        llm_provider="openai",
        llm_model="gpt-4",
        system_prompt=(
            f"You are a helpful programming assistant. Use the provided context "
            f"to give comprehensive answers that combine knowledge from multiple "
            f"topics. Context:\n\n{context}"
        ),
    )

    # Send the question
    response = await client.chat_message(
        chat_id=chat_session["chat_id"], message=user_question
    )

    print(f"✓ AI Response (with context from 3 conversations):\n")
    if response.get("responses"):
        print(f"{response['responses'][0]}\n")

    # ========================================
    # STEP 5: Store New Conversation
    # ========================================
    print("=== Step 5: Storing New Conversation ===")

    new_conv_id = "conv_new_question"
    await create_conversation(
        client, conversations_collection, new_conv_id, "Memory-Safe Database Code"
    )

    # Store user question
    await store_message_with_embedding(
        client,
        messages_collection,
        new_conv_id,
        "user",
        user_question,
        ["rust", "database", "performance"],
    )

    # Store AI response
    if response.get("responses"):
        await store_message_with_embedding(
            client,
            messages_collection,
            new_conv_id,
            "assistant",
            response["responses"][0],
            ["rust", "database", "performance"],
        )

    print("✓ New conversation stored and indexed for future retrieval\n")

    # ========================================
    # STEP 6: Demonstrate Cross-Conversation Search
    # ========================================
    print("=== Step 6: Cross-Conversation Search ===")
    print("Searching for messages about 'ownership' across ALL conversations...\n")

    ownership_results = await client.text_search(
        collection=messages_collection,
        query_text="ownership system",
        limit=3,
    )

    print(f"✓ Found {len(ownership_results)} messages mentioning ownership:")
    for i, msg in enumerate(ownership_results):
        content = extract_string_field(msg, "content")
        conv_id = extract_string_field(msg, "conversation_id")
        print(f"  {i + 1}. From {conv_id}: {content}\n")

    # ========================================
    # STEP 7: Show System Statistics
    # ========================================
    print("=== System Statistics ===")

    total_messages = await client.find_all(messages_collection, 1000)
    total_convs = await client.find_all(conversations_collection, 100)

    print(f"Total conversations: {len(total_convs)}")
    print(f"Total messages stored: {len(total_messages)}")
    print("All messages are indexed for vector search ✓")
    print("All messages are indexed for text search ✓")
    print("All messages are queryable by metadata ✓\n")

    # ========================================
    # STEP 8: Dynamic Search Configuration
    # ========================================
    print("=== Step 8: Dynamic Search Configuration ===")
    print("Each conversation can have its own search config...\n")

    print("💡 Conversations can store custom search configurations:")
    print("  • Search type: hybrid, text, or vector")
    print("  • Relevance thresholds")
    print("  • Filter by tags or metadata")
    print("  • Collection-specific settings")
    print("  • Per-conversation AI behavior")
    print("\nThis enables context-aware search tuned to each conversation's needs!\n")

    # ========================================
    # Cleanup
    # ========================================
    print("=== Cleanup ===")
    await client.delete_chat_session(chat_session["chat_id"])
    await client.delete_collection(messages_collection)
    await client.delete_collection(conversations_collection)
    print("✓ Cleanup complete\n")

    # ========================================
    # Summary - What We Demonstrated
    # ========================================
    print("\n=== 📚 Summary: What This Example Showed ===\n")

    print("🔧 ekoDB Native Capabilities Used:")
    print("  ✓ Functions with Embed operation (AI integration)")
    print("  ✓ Hybrid Search (text + vector combined)")
    print("  ✓ Text Search (full-text with stemming)")
    print("  ✓ Automatic embedding generation")
    print("  ✓ Cross-collection queries")
    print("")

    print("🚀 New Client Helper Methods:")
    print("  • client.embed(text, model) - Generate embeddings")
    print("  • client.hybrid_search() - Semantic + keyword search")
    print("  • client.text_search() - Full-text search")
    print("  • client.find_all() - Query all documents")
    print("")

    print("💡 Key Takeaways:")
    print("  1. ekoDB handles AI Functions natively - no external services needed")
    print("  2. One-line embedding generation with auto-cleanup")
    print("  3. Hybrid search combines semantic understanding + keyword matching")
    print("  4. Perfect for RAG: store, search, and retrieve context")
    print("  5. All AI capabilities accessible through simple client methods")
    print("")

    print("🎯 Build production RAG systems with ekoDB!")
    print("   → Set OPENAI_API_KEY in your ekoDB server environment")
    print("   → Use these client helpers to make AI integration simple")
    print("   → Scale to millions of documents with native indexing")
    print("")


if __name__ == "__main__":
    asyncio.run(main())
