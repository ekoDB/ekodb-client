/**
 * Search Scripts Example - Basic Search Operations
 *
 * Demonstrates simple search and query operations using scripts
 */

import { EkoDBClient, Stage } from "@ekodb/ekodb-client";
import * as dotenv from "dotenv";

dotenv.config();

const BASE_URL = process.env.API_BASE_URL || "http://localhost:8080";
const API_KEY = process.env.API_BASE_KEY || "a-test-api-key-from-ekodb";

async function main() {
  const client = new EkoDBClient(BASE_URL, API_KEY);

  console.log("🚀 ekoDB TypeScript Search Scripts Example\n");

  // Setup test data
  console.log("📋 Setting up test data...");
  try {
    await client.deleteCollection("search_docs_ts");
  } catch (e) {}

  const documents = [
    {
      title: "Introduction to Machine Learning",
      content: "Machine learning is a subset of artificial intelligence.",
      category: "AI",
      tags: ["ml", "ai", "data science"],
    },
    {
      title: "Database Design Principles",
      content: "Good database design involves normalization and indexing.",
      category: "Database",
      tags: ["database", "design", "sql"],
    },
    {
      title: "Vector Databases Explained",
      content: "Vector databases store data based on semantic similarity.",
      category: "Database",
      tags: ["vector", "database", "embeddings"],
    },
    {
      title: "Natural Language Processing",
      content: "NLP enables computers to understand human language.",
      category: "AI",
      tags: ["nlp", "ai", "text"],
    },
    {
      title: "Getting Started with ekoDB",
      content: "ekoDB is a high-performance database with AI capabilities.",
      category: "Database",
      tags: ["ekodb", "database", "tutorial"],
    },
  ];

  for (const doc of documents) {
    await client.insert("search_docs_ts", doc);
  }
  console.log(`✅ Inserted ${documents.length} documents\n`);

  const scriptIds: string[] = [];

  // Example 1: List All Documents
  console.log("📝 Example 1: List All Documents\n");
  const script1 = {
    label: "list_all_docs_ts",
    name: "List All Documents",
    version: "1.0",
    parameters: {},
    functions: [Stage.findAll("search_docs_ts")],
    tags: ["search", "list"],
  };
  const scriptId1 = await client.saveScript(script1);
  scriptIds.push(scriptId1);
  console.log("✅ Script saved");

  const result1 = await client.callScript("list_all_docs_ts");
  console.log(`📊 Found ${result1.records?.length || 0} documents`);
  result1.records?.forEach((doc: any, i: number) => {
    const title = doc.title?.value || doc.title;
    const category = doc.category?.value || doc.category;
    console.log(`   ${i + 1}. ${title} (${category})`);
  });
  console.log(`⏱️  Execution time: ${result1.stats?.execution_time_ms}ms\n`);

  // Example 2: Count Documents by Category
  console.log("📝 Example 2: Count Documents by Category\n");
  const script2 = {
    label: "docs_by_category_ts",
    name: "Documents by Category",
    version: "1.0",
    parameters: {},
    functions: [
      Stage.findAll("search_docs_ts"),
      Stage.group(
        ["category"],
        [{ output_field: "count", operation: "Count" }],
      ),
    ],
    tags: ["search", "analytics"],
  };
  const scriptId2 = await client.saveScript(script2);
  scriptIds.push(scriptId2);
  console.log("✅ Script saved");

  const result2 = await client.callScript("docs_by_category_ts");
  console.log("📊 Documents by category:");
  result2.records?.forEach((group: any) =>
    console.log(`   ${JSON.stringify(group)}`),
  );
  console.log(`⏱️  Execution time: ${result2.stats?.execution_time_ms}ms\n`);

  // Cleanup
  console.log("🧹 Cleaning up...");
  for (const scriptId of scriptIds) {
    try {
      await client.deleteScript(scriptId);
    } catch (e) {}
  }
  try {
    await client.deleteCollection("search_docs_ts");
  } catch (e) {}
  console.log("✅ Cleanup complete\n");

  console.log("✅ All search script examples finished!");
}

main().catch(console.error);
