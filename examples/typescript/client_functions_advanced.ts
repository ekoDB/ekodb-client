/**
 * Advanced Scripts Example - Query, Sort, Limit, Group
 *
 * Demonstrates advanced query and aggregation operations using simple patterns
 */

import { EkoDBClient, Stage } from "@ekodb/ekodb-client";
import * as dotenv from "dotenv";

dotenv.config();

const BASE_URL = process.env.API_BASE_URL || "http://localhost:8080";
const API_KEY = process.env.API_BASE_KEY || "a-test-api-key-from-ekodb";

async function main() {
  const client = new EkoDBClient(BASE_URL, API_KEY);

  console.log("🚀 ekoDB TypeScript Advanced Scripts Example\n");

  // Setup test data
  console.log("📋 Setting up test data...");
  try {
    await client.deleteCollection("advanced_products_ts");
  } catch (e) {
    // Collection doesn't exist
  }

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

  for (const product of products) {
    await client.insert("advanced_products_ts", product);
  }
  console.log(`✅ Created ${products.length} products\n`);

  const scriptIds: string[] = [];

  // Example 1: List All Products
  console.log("📝 Example 1: List All Products\n");
  const script1 = {
    label: "list_all_products_adv_ts",
    name: "List All Products",
    version: "1.0",
    parameters: {},
    functions: [Stage.findAll("advanced_products_ts")],
    tags: ["products", "list"],
  };
  const scriptId1 = await client.saveScript(script1);
  scriptIds.push(scriptId1);
  console.log("✅ Script saved");

  const result1 = await client.callScript("list_all_products_adv_ts");
  console.log(`📊 Found ${result1.records?.length || 0} products`);
  console.log(`⏱️  Execution time: ${result1.stats?.execution_time_ms}ms\n`);

  // Example 2: Group Products by Category
  console.log("📝 Example 2: Group Products by Category\n");
  const script2 = {
    label: "products_by_category_ts",
    name: "Products by Category",
    version: "1.0",
    parameters: {},
    functions: [
      Stage.findAll("advanced_products_ts"),
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
  const scriptId2 = await client.saveScript(script2);
  scriptIds.push(scriptId2);
  console.log("✅ Script saved");

  const result2 = await client.callScript("products_by_category_ts");
  console.log("📊 Category breakdown:");
  result2.records?.forEach((record: any) =>
    console.log(`   ${JSON.stringify(record)}`),
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
    await client.deleteCollection("advanced_products_ts");
  } catch (e) {}
  console.log("✅ Cleanup complete\n");

  console.log("✅ All advanced script examples finished!");
}

main().catch(console.error);
