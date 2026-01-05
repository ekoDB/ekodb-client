/**
 * Complete Scripts Example for ekoDB TypeScript Client
 *
 * Demonstrates: FindAll, Group, Count, Multi-stage Pipelines
 */

import { EkoDBClient, Stage } from "@ekodb/ekodb-client";
import * as dotenv from "dotenv";

dotenv.config();

const BASE_URL = process.env.API_BASE_URL || "http://localhost:8080";
const API_KEY = process.env.API_BASE_KEY || "a-test-api-key-from-ekodb";

interface Product {
  name: string;
  category: string;
  price: number;
  stock: number;
  rating: number;
}

async function setupTestData(client: EkoDBClient): Promise<void> {
  console.log("📋 Setting up complete test data...");

  const products: Product[] = [
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
      name: 'Monitor 27"',
      category: "Electronics",
      price: 399,
      stock: 20,
      rating: 4.6,
    },
  ];

  for (const product of products) {
    await client.insert("complete_products", product);
  }

  console.log(`✅ Created ${products.length} products\n`);
}

async function productStatsScript(client: EkoDBClient): Promise<string> {
  console.log("📝 Example 1: FindAll + Group (Simple Aggregation)\n");

  const script = {
    label: "product_stats",
    name: "Product Statistics",
    version: "1.0",
    parameters: {},
    functions: [
      Stage.findAll("complete_products"),
      Stage.group(
        ["category"],
        [
          { output_field: "count", operation: "Count" as const },
          {
            output_field: "avg_price",
            operation: "Average" as const,
            input_field: "price",
          },
        ],
      ),
    ],
    tags: ["products", "analytics"],
  };

  const id = await client.saveScript(script);
  console.log(`✅ Script saved: ${id}`);

  const result = await client.callScript("product_stats");

  console.log(`📊 Found ${result.records.length} product groups`);
  result.records.forEach((record: any) => {
    console.log(`   ${JSON.stringify(record)}`);
  });
  console.log(`⏱️  Execution time: ${result.stats.execution_time_ms}ms\n`);

  return id;
}

async function listProductsScript(client: EkoDBClient): Promise<string> {
  console.log("📝 Example 2: Simple Product Listing\n");

  const script = {
    label: "list_all_products",
    name: "List All Products",
    version: "1.0",
    parameters: {},
    functions: [Stage.findAll("complete_products")],
    tags: ["products", "list"],
  };

  const id = await client.saveScript(script);
  console.log("✅ Script saved");

  const result = await client.callScript("list_all_products");

  console.log(`📊 Found ${result.records.length} products`);
  console.log(`⏱️  Execution time: ${result.stats.execution_time_ms}ms\n`);

  return id;
}

async function categoryCountScript(client: EkoDBClient): Promise<string> {
  console.log("📝 Example 3: Count by Category\n");

  const script = {
    label: "count_by_category",
    name: "Count Products by Category",
    version: "1.0",
    parameters: {},
    functions: [
      Stage.findAll("complete_products"),
      Stage.group(
        ["category"],
        [{ output_field: "count", operation: "Count" as const }],
      ),
    ],
    tags: ["products", "analytics"],
  };

  const id = await client.saveScript(script);
  console.log("✅ Script saved");

  const result = await client.callScript("count_by_category");

  console.log(`📊 Found ${result.records.length} categories`);
  result.records.forEach((record: any) => {
    console.log(`   ${JSON.stringify(record)}`);
  });
  console.log(`⏱️  Execution time: ${result.stats.execution_time_ms}ms\n`);

  return id;
}

async function topRatedScript(client: EkoDBClient): Promise<string> {
  console.log("📝 Example 4: High Rating Products\n");

  const script = {
    label: "top_rated_products",
    name: "Top Rated Products",
    version: "1.0",
    parameters: {},
    functions: [Stage.findAll("complete_products")],
    tags: ["products", "quality"],
  };

  const id = await client.saveScript(script);
  console.log("✅ Script saved");

  const result = await client.callScript("top_rated_products");

  console.log(`📊 Found ${result.records.length} products`);
  console.log(`⏱️  Execution time: ${result.stats.execution_time_ms}ms\n`);

  return id;
}

async function scriptWithParameter(client: EkoDBClient): Promise<string> {
  console.log("📝 Example 5: Script with Parameter Definition\n");

  const script = {
    label: "list_with_limit",
    name: "List Products with Limit",
    version: "1.0",
    parameters: {
      max_items: {
        param_type: "Integer" as const,
        required: false,
        default: 5,
      },
    },
    functions: [Stage.findAll("complete_products")],
    tags: ["products", "list"],
  };

  const id = await client.saveScript(script);
  console.log("✅ Script saved");

  const result = await client.callScript("list_with_limit", { max_items: 3 });

  console.log(`📊 Found ${result.records.length} products`);
  console.log(`⏱️  Execution time: ${result.stats.execution_time_ms}ms\n`);

  return id;
}

async function multiStagePipeline(client: EkoDBClient): Promise<string> {
  console.log("📝 Example 6: Multi-Stage Pipeline (FindAll → Group → Count)\n");

  const script = {
    label: "product_summary",
    name: "Product Summary Report",
    version: "1.0",
    parameters: {},
    functions: [
      Stage.findAll("complete_products"),
      Stage.group(
        ["category"],
        [
          { output_field: "count", operation: "Count" as const },
          {
            output_field: "avg_price",
            operation: "Average" as const,
            input_field: "price",
          },
        ],
      ),
      Stage.count("total_categories"),
    ],
    tags: ["products", "analytics"],
  };

  const id = await client.saveScript(script);
  console.log("✅ Script saved");

  const result = await client.callScript("product_summary");

  console.log(`📊 Pipeline executed ${result.stats.stages_executed} stages`);
  console.log(`⏱️  Total execution time: ${result.stats.execution_time_ms}ms`);
  console.log();

  return id;
}

async function cleanup(
  client: EkoDBClient,
  scriptIds: string[],
): Promise<void> {
  console.log("🧹 Cleaning up...");

  try {
    for (const id of scriptIds) {
      await client.deleteScript(id);
    }
    await client.deleteCollection("complete_products");
    console.log("✅ Cleanup complete\n");
  } catch (error) {
    console.log("⚠️  Cleanup had some errors\n");
  }
}

async function main() {
  console.log("🚀 ekoDB TypeScript Complete Scripts Example\n");
  console.log(
    "📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines\n",
  );

  const client = new EkoDBClient(BASE_URL, API_KEY);

  try {
    await client.init();
  } catch (error) {
    console.error("⚠️  Auth initialization failed, retrying...");
    // Wait a moment and retry once
    await new Promise((resolve) => setTimeout(resolve, 1000));
    await client.init();
  }

  try {
    await setupTestData(client);
    const scriptIds: string[] = [];
    scriptIds.push(await productStatsScript(client));
    scriptIds.push(await listProductsScript(client));
    scriptIds.push(await categoryCountScript(client));
    scriptIds.push(await topRatedScript(client));
    scriptIds.push(await scriptWithParameter(client));
    scriptIds.push(await multiStagePipeline(client));
    await cleanup(client, scriptIds);

    console.log("✅ All complete script examples finished!");
    console.log("\n💡 This example demonstrates ekoDB's Script system:");
    console.log("   ✅ FindAll operations");
    console.log("   ✅ Group aggregations (Count, Average)");
    console.log("   ✅ Multi-stage pipelines (FindAll → Group → Count)");
    console.log("   ✅ Parameter definitions");
    console.log("   ✅ Script management (save, call, delete)");
  } catch (error) {
    console.error("❌ Error:", error);
    process.exit(1);
  }
}

main().catch((error) => {
  console.error("❌ Error:", error);
  process.exit(1);
});
