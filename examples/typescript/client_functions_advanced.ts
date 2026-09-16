/**
 * Advanced Functions Example - Query, Sort, Limit, Group
 *
 * Demonstrates advanced query and aggregation operations using simple patterns
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

function requireGroupCounts(
  records: any[],
  field: string,
  expected: Record<string, number>,
): void {
  if (records.length !== Object.keys(expected).length) {
    throw new Error(
      `Group returned ${records.length} rows; expected ${Object.keys(expected).length}`,
    );
  }
  for (const record of records) {
    const group = String(scalar(record[field]));
    const count = Number(scalar(record.count));
    if (expected[group] !== count) {
      throw new Error(
        `Group ${group} returned count ${count}; expected ${expected[group]}`,
      );
    }
  }
}

async function main() {
  const client = new EkoDBClient(BASE_URL, API_KEY);
  const suffix = `${process.pid}_${Date.now()}`;
  const collection = `advanced_products_ts_${suffix}`;
  const listLabel = `list_all_products_adv_ts_${suffix}`;
  const groupLabel = `products_by_category_ts_${suffix}`;
  const scriptIds: string[] = [];
  let ownsCollection = false;
  let primaryError: unknown;

  console.log("🚀 ekoDB TypeScript Advanced Functions Example\n");

  try {
    // Setup test data
    console.log("📋 Setting up test data...");
    const products = [
      {
        name: "Laptop Pro",
        category: "Electronics",
        price: 1299,
        stock: 15,
        rating: 4.8,
      },
      {
        name: "Wireless Mouse",
        category: "Electronics",
        price: 29,
        stock: 45,
        rating: 4.5,
      },
      {
        name: "Desk Chair",
        category: "Furniture",
        price: 349,
        stock: 8,
        rating: 4.2,
      },
      {
        name: "Standing Desk",
        category: "Furniture",
        price: 599,
        stock: 12,
        rating: 4.7,
      },
      {
        name: "USB-C Cable",
        category: "Electronics",
        price: 19,
        stock: 100,
        rating: 4.3,
      },
      {
        name: 'Monitor 27"',
        category: "Electronics",
        price: 399,
        stock: 20,
        rating: 4.6,
      },
      {
        name: "Bookshelf",
        category: "Furniture",
        price: 149,
        stock: 5,
        rating: 4.1,
      },
      {
        name: "Keyboard",
        category: "Electronics",
        price: 89,
        stock: 30,
        rating: 4.4,
      },
    ];

    ownsCollection = true;
    for (const product of products) {
      await client.insert(collection, product);
    }
    console.log(`✅ Created ${products.length} products\n`);

    // Example 1: List All Products
    console.log("📝 Example 1: List All Products\n");
    const script1 = {
      label: listLabel,
      name: "List All Products",
      version: "1.0",
      parameters: {},
      functions: [Stage.findAll(collection)],
      tags: ["products", "list"],
    };
    const scriptId1 = await saveOrUpdate(client, script1);
    scriptIds.push(scriptId1);
    console.log("✅ Function saved");

    const result1 = await client.callFunction(scriptId1);
    const listRecords = requireRecordCount(
      result1,
      products.length,
      "List products",
    );
    console.log(`📊 Found ${listRecords.length} products`);
    console.log(`⏱️  Execution time: ${result1.stats?.execution_time_ms}ms\n`);

    // Example 2: Group Products by Category
    console.log("📝 Example 2: Group Products by Category\n");
    const script2 = {
      label: groupLabel,
      name: "Products by Category",
      version: "1.0",
      parameters: {},
      functions: [
        Stage.findAll(collection),
        Stage.group(
          ["category"],
          [
            { output_field: "count", operation: "Count" },
            {
              output_field: "avg_price",
              operation: "Average",
              input_field: "price",
            },
          ],
        ),
      ],
      tags: ["products", "analytics"],
    };
    const scriptId2 = await saveOrUpdate(client, script2);
    scriptIds.push(scriptId2);
    console.log("✅ Function saved");

    const result2 = await client.callFunction(scriptId2);
    const groupRecords = requireRecordCount(result2, 2, "Group products");
    requireGroupCounts(groupRecords, "category", {
      Electronics: 5,
      Furniture: 3,
    });
    console.log("📊 Category breakdown:");
    groupRecords.forEach((record: any) =>
      console.log(`   ${JSON.stringify(record)}`),
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
          "Advanced functions cleanup failed",
        );
    } else {
      console.log("✅ Cleanup complete\n");
    }
  }

  console.log("✅ All advanced script examples finished!");
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
