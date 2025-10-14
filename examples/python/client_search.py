"""
Search Example

Demonstrates full-text search, vector search, and hybrid search:
- Full-text search with fuzzy matching
- Vector/semantic search
- Hybrid search combining text and vectors
- Field weighting and boosting
"""

import asyncio
import os
import random
from pathlib import Path
from dotenv import load_dotenv
from ekodb_client import Client

# Load environment variables
env_path = Path(__file__).parent.parent / ".env"
load_dotenv(env_path)


async def main():
    # Create client
    base_url = os.getenv("API_BASE_URL", "http://localhost:8080")
    api_key = os.getenv("API_BASE_KEY", "a-test-api-key-from-ekodb")
    client = Client.new(base_url, api_key)

    print("=== Search Examples ===\n")

    # Use unique collection names
    users_collection = "search_users_client_py"
    documents_collection = "search_documents_client_py"

    # Cleanup any existing test collections
    try:
        await client.delete_collection(users_collection)
        await client.delete_collection(documents_collection)
    except:
        pass  # Ignore errors if collections don't exist

    # Setup: Insert test data
    print("Setting up test data...")

    # Insert users for searching
    await client.insert(
        users_collection,
        {
            "name": "John Doe",
            "email": "john@example.com",
            "bio": "Software engineer specializing in JavaScript and TypeScript",
            "title": "Senior Developer Engineer",
        },
    )

    await client.insert(
        users_collection,
        {
            "name": "Jane Smith",
            "email": "jane@example.com",
            "bio": "Product manager with experience in running agile teams",
            "title": "Product Manager",
        },
    )

    await client.insert(
        users_collection,
        {
            "name": "Bob Johnson",
            "email": "bob@example.com",
            "bio": "DevOps engineer running cloud infrastructure",
            "title": "DevOps Engineer",
        },
    )

    await client.insert(
        users_collection,
        {
            "name": "Alice Williams",
            "email": "alice@example.com",
            "bio": "Senior software engineer with expertise in distributed systems",
            "title": "Staff Engineer",
        },
    )

    await client.insert(
        users_collection,
        {
            "name": "Charlie Davis",
            "email": "charlie@example.com",
            "bio": "Machine learning engineer working on AI models",
            "title": "ML Engineer",
        },
    )

    # Insert documents with vector embeddings for vector/hybrid search
    await client.insert(
        documents_collection,
        {
            "title": "Introduction to Machine Learning",
            "content": "Machine learning is a subset of artificial intelligence...",
            "embedding": [random.random() for _ in range(384)],
        },
    )

    await client.insert(
        documents_collection,
        {
            "title": "Deep Learning Fundamentals",
            "content": "Deep learning uses neural networks with multiple layers...",
            "embedding": [random.random() for _ in range(384)],
        },
    )

    await client.insert(
        documents_collection,
        {
            "title": "Natural Language Processing",
            "content": "NLP enables computers to understand human language...",
            "embedding": [random.random() for _ in range(384)],
        },
    )

    print("✅ Test data created\n")

    # Example 1: Basic full-text search
    print("1. Basic full-text search:")
    search1 = {"query": "john", "fields": "name,email,bio", "limit": 10}

    results1 = await client.search(users_collection, search1)
    print(f"Found {len(results1.get('results', []))} results")
    for i, result in enumerate(results1.get("results", []), 1):
        score = result.get("score", 0)
        matched = result.get("matched_fields", [])
        print(f"  {i}. Score: {score:.3f}, Matched: {', '.join(matched)}")
    print()

    # Example 2: Fuzzy search with typo tolerance
    print("2. Fuzzy search (typo tolerance):")
    search2 = {
        "query": "enginer",  # Typo: should match "engineer"
        "fields": "title,bio",
        "fuzzy": True,
        "max_edit_distance": 2,
        "limit": 5,
    }

    results2 = await client.search(users_collection, search2)
    print(f"Found {len(results2.get('results', []))} results with fuzzy matching")
    for i, result in enumerate(results2.get("results", []), 1):
        score = result.get("score", 0)
        matched = result.get("matched_fields", [])
        print(f"  {i}. Score: {score:.3f}, Matched: {', '.join(matched)}")
    print()

    # Example 3: Search with field weights
    print("3. Search with field weights:")
    search3 = {
        "query": "engineer",
        "fields": "title,bio",
        "weights": "title:3.0,bio:1.0",
        "limit": 10,
    }

    results3 = await client.search(users_collection, search3)
    print(f"Found {len(results3.get('results', []))} results with weighted fields")
    for i, result in enumerate(results3.get("results", []), 1):
        score = result.get("score", 0)
        matched = result.get("matched_fields", [])
        print(f"  {i}. Score: {score:.3f}, Matched: {', '.join(matched)}")
    print()

    # Example 4: Search with minimum score threshold
    print("4. Search with minimum score threshold:")
    search4 = {
        "query": "software",
        "fields": "title,bio",
        "min_score": 0.3,
        "limit": 10,
    }

    results4 = await client.search(users_collection, search4)
    print(f"Found {len(results4.get('results', []))} results with score >= 0.3")
    for i, result in enumerate(results4.get("results", []), 1):
        score = result.get("score", 0)
        matched = result.get("matched_fields", [])
        print(f"  {i}. Score: {score:.3f}, Matched: {', '.join(matched)}")
    print()

    # Example 5: Search with stemming and exact match boosting
    print("5. Search with stemming and exact match boosting:")
    search5 = {
        "query": "running",
        "fields": "bio",
        "enable_stemming": True,
        "boost_exact": True,
        "limit": 10,
    }

    results5 = await client.search(users_collection, search5)
    print(
        f"Found {len(results5.get('results', []))} results (matches: run, running, runs)"
    )
    for i, result in enumerate(results5.get("results", []), 1):
        score = result.get("score", 0)
        matched = result.get("matched_fields", [])
        print(f"  {i}. Score: {score:.3f}, Matched: {', '.join(matched)}")
    print()

    # Example 6: Vector search (semantic search)
    print("6. Vector search (semantic search):")
    query_vector = [random.random() for _ in range(384)]
    search6 = {
        "query": "",  # Empty query for vector-only search
        "vector": query_vector,
        "vector_field": "embedding",
        "limit": 3,
    }

    results6 = await client.search(documents_collection, search6)
    print(f"Found {len(results6.get('results', []))} semantically similar documents")
    for i, result in enumerate(results6.get("results", []), 1):
        score = result.get("score", 0)
        print(f"  {i}. Score: {score:.3f}")
    print()

    # Example 7: Hybrid search (text + vector)
    print("7. Hybrid search (text + vector):")
    search7 = {
        "query": "machine learning",
        "fields": "title,content",
        "vector": query_vector,
        "vector_field": "embedding",
        "hybrid_alpha": 0.5,  # 50% text, 50% vector
        "limit": 3,
    }

    results7 = await client.search(documents_collection, search7)
    print(
        f"Found {len(results7.get('results', []))} results using hybrid search (text + vector)"
    )
    for i, result in enumerate(results7.get("results", []), 1):
        score = result.get("score", 0)
        matched = result.get("matched_fields", [])
        print(f"  {i}. Score: {score:.3f}, Matched: {', '.join(matched)}")
    print()

    # Example 8: Case-sensitive search
    print("8. Case-sensitive search:")
    search8 = {
        "query": "Senior",
        "fields": "title",
        "case_sensitive": True,
        "limit": 10,
    }

    results8 = await client.search(users_collection, search8)
    print(f"Found {len(results8.get('results', []))} results (case-sensitive)")
    for i, result in enumerate(results8.get("results", []), 1):
        score = result.get("score", 0)
        matched = result.get("matched_fields", [])
        print(f"  {i}. Score: {score:.3f}, Matched: {', '.join(matched)}")
    print()

    # Cleanup
    print("=== Cleanup ===")
    await client.delete_collection(users_collection)
    await client.delete_collection(documents_collection)
    print("✅ Deleted test collections\n")

    print("✅ Search examples completed!")


if __name__ == "__main__":
    asyncio.run(main())
