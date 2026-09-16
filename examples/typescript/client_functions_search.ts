/**
 * Search Functions Example - Basic Search Operations
 *
 * Demonstrates simple search and query operations using scripts
 */

import { EkoDBClient, Stage, UserFunction } from "@ekodb/ekodb-client";
import * as dotenv from "dotenv";

dotenv.config();

const BASE_URL = process.env.API_BASE_URL || "http://localhost:8080";
const API_KEY = process.env.API_BASE_KEY || "a-test-api-key-from-ekodb";

/** True when a save failed because the function label already exists (HTTP 409). */
function isAlreadyExistsError(error: unknown): boolean {
  const message = error instanceof Error ? error.message : String(error);
  return message.includes("status 409") || message.includes("already exists");
}

/**
 * Idempotent save: create the function, or update it in place if a function
 * with the same label already exists. Returns the function's id so downstream
 * cleanup-by-id continues to work.
 */
async function saveOrUpdate(
  client: EkoDBClient,
  script: UserFunction,
): Promise<string> {
  try {
    return await client.saveFunction(script);
  } catch (error) {
    if (!isAlreadyExistsError(error)) throw error;
    await client.updateFunction(script.label, script);
    console.log(`Function '${script.label}' already existed — updated instead`);
    const existing = await client.getFunction(script.label);
    return existing.id ?? script.label;
  }
}

function scalar(value: any): any {
  return value && typeof value === "object" && "value" in value
    ? value.value
    : value;
}

function requireRecordCount(
  result: any,
  expected: number,
  label: string,
): any[] {
  if (!Array.isArray(result.records)) {
    throw new Error(`${label} response omitted records`);
  }
  if (result.records.length !== expected) {
    throw new Error(
      `${label} returned ${result.records.length} records; expected ${expected}`,
    );
  }
  return result.records;
}

async function main() {
  const client = new EkoDBClient(BASE_URL, API_KEY);
  const suffix = `${process.pid}_${Date.now()}`;
  const collection = `search_docs_ts_${suffix}`;
  const listLabel = `list_all_docs_ts_${suffix}`;
  const groupLabel = `docs_by_category_ts_${suffix}`;
  const scriptIds: string[] = [];
  let ownsCollection = false;
  let primaryError: unknown;

  console.log("🚀 ekoDB TypeScript Search Functions Example\n");

  try {
    // Setup test data
    console.log("📋 Setting up test data...");
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

    ownsCollection = true;
    for (const doc of documents) {
      await client.insert(collection, doc);
    }
    console.log(`✅ Inserted ${documents.length} documents\n`);

    // Example 1: List All Documents
    console.log("📝 Example 1: List All Documents\n");
    const script1 = {
      label: listLabel,
      name: "List All Documents",
      version: "1.0",
      parameters: {},
      functions: [Stage.findAll(collection)],
      tags: ["search", "list"],
    };
    const scriptId1 = await saveOrUpdate(client, script1);
    scriptIds.push(scriptId1);
    console.log("✅ Function saved");

    const result1 = await client.callFunction(scriptId1);
    const listRecords = requireRecordCount(
      result1,
      documents.length,
      "List documents",
    );
    console.log(`📊 Found ${listRecords.length} documents`);
    listRecords.forEach((doc: any, i: number) => {
      const title = doc.title?.value || doc.title;
      const category = doc.category?.value || doc.category;
      console.log(`   ${i + 1}. ${title} (${category})`);
    });
    console.log(`⏱️  Execution time: ${result1.stats?.execution_time_ms}ms\n`);

    // Example 2: Count Documents by Category
    console.log("📝 Example 2: Count Documents by Category\n");
    const script2 = {
      label: groupLabel,
      name: "Documents by Category",
      version: "1.0",
      parameters: {},
      functions: [
        Stage.findAll(collection),
        Stage.group(
          ["category"],
          [{ output_field: "count", operation: "Count" }],
        ),
      ],
      tags: ["search", "analytics"],
    };
    const scriptId2 = await saveOrUpdate(client, script2);
    scriptIds.push(scriptId2);
    console.log("✅ Function saved");

    const result2 = await client.callFunction(scriptId2);
    const groupRecords = requireRecordCount(result2, 2, "Group documents");
    const counts = Object.fromEntries(
      groupRecords.map((group: any) => [
        String(scalar(group.category)),
        Number(scalar(group.count)),
      ]),
    );
    if (counts.AI !== 2 || counts.Database !== 3) {
      throw new Error(
        `Category counts were ${JSON.stringify(counts)}; expected AI=2 and Database=3`,
      );
    }
    console.log("📊 Documents by category:");
    groupRecords.forEach((group: any) =>
      console.log(`   ${JSON.stringify(group)}`),
    );
    console.log(`⏱️  Execution time: ${result2.stats?.execution_time_ms}ms\n`);
  } catch (error) {
    primaryError = error;
    throw error;
  } finally {
    console.log("🧹 Cleaning up...");
    const cleanupErrors: unknown[] = [];
    for (const scriptId of scriptIds) {
      try {
        await client.deleteFunction(scriptId);
      } catch (error) {
        const message = error instanceof Error ? error.message : String(error);
        if (!message.includes("status 404") && !/not found/i.test(message)) {
          cleanupErrors.push(error);
        }
      }
    }
    if (ownsCollection) {
      try {
        await client.deleteCollection(collection);
      } catch (error) {
        const message = error instanceof Error ? error.message : String(error);
        if (!message.includes("status 404") && !/not found/i.test(message)) {
          cleanupErrors.push(error);
        }
      }
    }
    if (cleanupErrors.length > 0) {
      if (primaryError) console.error("Cleanup errors:", cleanupErrors);
      else
        throw new AggregateError(
          cleanupErrors,
          "Search functions cleanup failed",
        );
    } else {
      console.log("✅ Cleanup complete\n");
    }
  }

  console.log("✅ All search script examples finished!");
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
