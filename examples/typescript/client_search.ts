/**
 * Search Example
 *
 * Demonstrates full-text search, vector search, and hybrid search:
 * - Full-text search with fuzzy matching
 * - Vector/semantic search
 * - Hybrid search combining text and vectors
 * - Field weighting and boosting
 */

import { EkoDBClient, SearchQueryBuilder } from "@ekodb/ekodb-client";
import * as dotenv from "dotenv";

dotenv.config();

async function main() {
  const client = new EkoDBClient(
    process.env.API_BASE_URL || "http://localhost:8080",
    process.env.API_BASE_KEY || "a-test-api-key-from-ekodb",
  );
  await client.init();

  console.log("=== Search Examples ===\n");

  // Use unique collection names
  const usersCollection = "search_users_client_ts";
  const documentsCollection = "search_documents_client_ts";

  // Cleanup any existing test collections
  try {
    await client.deleteCollection(usersCollection);
    await client.deleteCollection(documentsCollection);
  } catch (e) {
    // Ignore errors if collections don't exist
  }

  // Setup: Insert test data
  console.log("Setting up test data...");

  // Insert users for searching
  await client.insert(usersCollection, {
    name: "John Doe",
    email: "john@example.com",
    bio: "Software engineer specializing in JavaScript and TypeScript",
    title: "Senior Developer Engineer",
    skills: ["JavaScript", "TypeScript", "React", "Node.js"],
  });

  await client.insert(usersCollection, {
    name: "Jane Smith",
    email: "jane@example.com",
    bio: "Product manager with experience in running agile teams",
    title: "Product Manager",
    skills: ["Product Management", "Agile", "Scrum"],
  });

  await client.insert(usersCollection, {
    name: "Bob Johnson",
    email: "bob@example.com",
    bio: "DevOps engineer running cloud infrastructure",
    title: "DevOps Engineer",
    skills: ["AWS", "Docker", "Kubernetes", "Terraform"],
  });

  await client.insert(usersCollection, {
    name: "Alice Williams",
    email: "alice@example.com",
    bio: "Senior software engineer with expertise in distributed systems",
    title: "Staff Engineer",
    skills: ["Go", "Rust", "Distributed Systems", "Microservices"],
  });

  await client.insert(usersCollection, {
    name: "Charlie Davis",
    email: "charlie@example.com",
    bio: "Machine learning engineer working on AI models",
    title: "ML Engineer",
    skills: ["Python", "TensorFlow", "PyTorch", "Machine Learning"],
  });

  // Insert documents with vector embeddings for vector/hybrid search
  await client.insert(documentsCollection, {
    title: "Introduction to Machine Learning",
    content: "Machine learning is a subset of artificial intelligence...",
    embedding: {
      type: "Vector",
      value: Array.from({ length: 384 }, () => Math.random()),
    },
  });

  await client.insert(documentsCollection, {
    title: "Deep Learning Fundamentals",
    content: "Deep learning uses neural networks with multiple layers...",
    embedding: {
      type: "Vector",
      value: Array.from({ length: 384 }, () => Math.random()),
    },
  });

  await client.insert(documentsCollection, {
    title: "Natural Language Processing",
    content: "NLP enables computers to understand human language...",
    embedding: {
      type: "Vector",
      value: Array.from({ length: 384 }, () => Math.random()),
    },
  });

  console.log("✅ Test data created\n");

  // Example 1: Basic full-text search
  console.log("1. Basic full-text search:");
  const search1 = new SearchQueryBuilder("john")
    .fields(["name", "email", "bio"])
    .limit(10)
    .build();

  const results1 = await client.search(usersCollection, search1);
  console.log(`Found ${results1.total} results`);
  results1.results.forEach((result, i) => {
    console.log(
      `  ${i + 1}. Score: ${result.score.toFixed(3)}, Matched: ${result.matched_fields.join(", ")}`,
    );
  });
  console.log();

  // Example 2: Fuzzy search with typo tolerance
  console.log("2. Fuzzy search (typo tolerance):");
  const search2 = new SearchQueryBuilder("enginer") // Typo: should match "engineer"
    .fields(["title", "bio"])
    .fuzzy(true)
    .maxEditDistance(2)
    .limit(5)
    .build();

  const results2 = await client.search(usersCollection, search2);
  console.log(`Found ${results2.total} results with fuzzy matching`);
  results2.results.forEach((result, i) => {
    console.log(
      `  ${i + 1}. Score: ${result.score.toFixed(3)}, Matched: ${result.matched_fields.join(", ")}`,
    );
  });
  console.log();

  // Example 3: Search with field weights
  console.log("3. Search with field weights:");
  const search3 = new SearchQueryBuilder("engineer")
    .fields(["title", "bio"])
    .weights({ title: 3.0, bio: 1.0 })
    .limit(10)
    .build();

  const results3 = await client.search(usersCollection, search3);
  console.log(`Found ${results3.total} results with weighted fields`);
  results3.results.forEach((result, i) => {
    console.log(
      `  ${i + 1}. Score: ${result.score.toFixed(3)}, Matched: ${result.matched_fields.join(", ")}`,
    );
  });
  console.log();

  // Example 4: Search with minimum score threshold
  console.log("4. Search with minimum score threshold:");
  const search4 = new SearchQueryBuilder("software")
    .fields(["title", "bio"])
    .minScore(0.3)
    .limit(10)
    .build();

  const results4 = await client.search(usersCollection, search4);
  console.log(`Found ${results4.total} results with score >= 0.3`);
  results4.results.forEach((result, i) => {
    console.log(
      `  ${i + 1}. Score: ${result.score.toFixed(3)}, Matched: ${result.matched_fields.join(", ")}`,
    );
  });
  console.log();

  // Example 5: Search with stemming and exact match boosting
  console.log("5. Search with stemming and exact match boosting:");
  const search5 = new SearchQueryBuilder("work")
    .fields(["bio"])
    .enableStemming(true)
    .boostExact(true)
    .language("english")
    .limit(10)
    .build();

  const results5 = await client.search(usersCollection, search5);
  console.log(
    `Found ${results5.total} results (matches: work, working, worked)`,
  );
  results5.results.forEach((result, i) => {
    console.log(
      `  ${i + 1}. Score: ${result.score.toFixed(3)}, Matched: ${result.matched_fields.join(", ")}`,
    );
  });
  console.log();

  // Example 6: Vector search (semantic search)
  console.log("6. Vector search (semantic search):");
  // Query vector - using random vector like in benchmarks
  const queryVector = Array.from({ length: 384 }, () => Math.random());

  const search6 = new SearchQueryBuilder("")
    .vector(queryVector)
    .vectorField("embedding")
    .vectorMetric("cosine")
    .vectorK(10)
    .vectorThreshold(0.0)
    .build();

  const results6 = await client.search(documentsCollection, search6);
  console.log(`Found ${results6.total} semantically similar documents`);
  results6.results.forEach((result, i) => {
    console.log(
      `  ${i + 1}. Score: ${result.score.toFixed(3)}, Matched: ${result.matched_fields.join(", ")}`,
    );
  });
  console.log();

  // Example 7: Hybrid search (text + vector)
  console.log("7. Hybrid search (text + vector):");
  const search7 = new SearchQueryBuilder("machine learning")
    .fields(["title", "content"])
    .vector(queryVector)
    .vectorField("embedding")
    .textWeight(0.6)
    .vectorWeight(0.4)
    .limit(3)
    .build();

  const results7 = await client.search(documentsCollection, search7);
  console.log(
    `Found ${results7.total} results using hybrid search (text + vector)`,
  );
  results7.results.forEach((result, i) => {
    console.log(
      `  ${i + 1}. Score: ${result.score.toFixed(3)}, Matched: ${result.matched_fields.join(", ")}`,
    );
  });
  console.log();

  // Example 8: Case-sensitive search
  console.log("8. Case-sensitive search:");
  const search8 = new SearchQueryBuilder("JavaScript")
    .fields(["skills", "bio"])
    .caseSensitive(true)
    .limit(10)
    .build();

  const results8 = await client.search(usersCollection, search8);
  console.log(`Found ${results8.total} results (case-sensitive)`);
  results8.results.forEach((result, i) => {
    console.log(
      `  ${i + 1}. Score: ${result.score.toFixed(3)}, Matched: ${result.matched_fields.join(", ")}`,
    );
  });
  console.log();

  // Cleanup
  console.log("=== Cleanup ===");
  await client.deleteCollection(usersCollection);
  await client.deleteCollection(documentsCollection);
  console.log("✅ Deleted test collections");

  console.log("\n✅ Search examples completed!");
}

main().catch(console.error);
