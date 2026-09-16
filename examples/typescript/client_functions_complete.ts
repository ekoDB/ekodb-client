/**
 * Complete Functions Example for ekoDB TypeScript Client
 *
 * Demonstrates: FindAll, Group, Count, Multi-stage Pipelines
 */

import { EkoDBClient, Stage, UserFunction } from "@ekodb/ekodb-client";
import * as dotenv from "dotenv";

dotenv.config();

const BASE_URL = process.env.API_BASE_URL || "http://localhost:8080";
const API_KEY = process.env.API_BASE_KEY || "a-test-api-key-from-ekodb";
const COLLECTION = "complete_products_ts";
const FUNCTION_LABELS = [
  "complete_product_stats_ts",
  "complete_list_products_ts",
  "complete_count_category_ts",
  "complete_top_rated_ts",
  "complete_list_limit_ts",
  "complete_product_summary_ts",
] as const;

function isNotFoundError(error: unknown): boolean {
  const message = error instanceof Error ? error.message : String(error);
  return message.includes("status 404") || /not found/i.test(message);
}

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
    await client.insert(COLLECTION, product);
  }

  console.log(`✅ Created ${products.length} products\n`);
}

async function productStatsScript(client: EkoDBClient): Promise<string> {
  console.log("📝 Example 1: FindAll + Group (Simple Aggregation)\n");

  const script = {
    label: FUNCTION_LABELS[0],
    name: "Product Statistics",
    version: "1.0",
    parameters: {},
    functions: [
      Stage.findAll(COLLECTION),
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

  const id = await saveOrUpdate(client, script);
  console.log(`✅ Function saved: ${id}`);

  const result = await client.callFunction(id);

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
    label: FUNCTION_LABELS[1],
    name: "List All Products",
    version: "1.0",
    parameters: {},
    functions: [Stage.findAll(COLLECTION)],
    tags: ["products", "list"],
  };

  const id = await saveOrUpdate(client, script);
  console.log("✅ Function saved");

  const result = await client.callFunction(id);

  console.log(`📊 Found ${result.records.length} products`);
  console.log(`⏱️  Execution time: ${result.stats.execution_time_ms}ms\n`);

  return id;
}

async function categoryCountScript(client: EkoDBClient): Promise<string> {
  console.log("📝 Example 3: Count by Category\n");

  const script = {
    label: FUNCTION_LABELS[2],
    name: "Count Products by Category",
    version: "1.0",
    parameters: {},
    functions: [
      Stage.findAll(COLLECTION),
      Stage.group(
        ["category"],
        [{ output_field: "count", operation: "Count" as const }],
      ),
    ],
    tags: ["products", "analytics"],
  };

  const id = await saveOrUpdate(client, script);
  console.log("✅ Function saved");

  const result = await client.callFunction(id);

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
    label: FUNCTION_LABELS[3],
    name: "Top Rated Products",
    version: "1.0",
    parameters: {},
    functions: [Stage.findAll(COLLECTION)],
    tags: ["products", "quality"],
  };

  const id = await saveOrUpdate(client, script);
  console.log("✅ Function saved");

  const result = await client.callFunction(id);

  console.log(`📊 Found ${result.records.length} products`);
  console.log(`⏱️  Execution time: ${result.stats.execution_time_ms}ms\n`);

  return id;
}

async function scriptWithParameter(client: EkoDBClient): Promise<string> {
  console.log("📝 Example 5: UserFunction with Parameter Definition\n");

  const script = {
    label: FUNCTION_LABELS[4],
    name: "List Products with Limit",
    version: "1.0",
    parameters: {
      max_items: {
        required: false,
        default: 5,
      },
    },
    functions: [Stage.findAll(COLLECTION)],
    tags: ["products", "list"],
  };

  const id = await saveOrUpdate(client, script);
  console.log("✅ Function saved");

  const result = await client.callFunction(id, {
    max_items: 3,
  });

  console.log(`📊 Found ${result.records.length} products`);
  console.log(`⏱️  Execution time: ${result.stats.execution_time_ms}ms\n`);

  return id;
}

async function multiStagePipeline(client: EkoDBClient): Promise<string> {
  console.log("📝 Example 6: Multi-Stage Pipeline (FindAll → Group → Count)\n");

  const script = {
    label: FUNCTION_LABELS[5],
    name: "Product Summary Report",
    version: "1.0",
    parameters: {},
    functions: [
      Stage.findAll(COLLECTION),
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

  const id = await saveOrUpdate(client, script);
  console.log("✅ Function saved");

  const result = await client.callFunction(id);

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

  const errors: unknown[] = [];
  for (const id of scriptIds) {
    try {
      await client.deleteFunction(id);
    } catch (error) {
      if (!isNotFoundError(error)) errors.push(error);
    }
  }
  try {
    await client.deleteCollection(COLLECTION);
  } catch (error) {
    if (!isNotFoundError(error)) errors.push(error);
  }
  if (errors.length > 0) {
    console.log("⚠️  Cleanup had some errors\n");
    throw new AggregateError(errors, "Complete functions cleanup failed");
  }
  console.log("✅ Cleanup complete\n");
}

async function main() {
  console.log("🚀 ekoDB TypeScript Complete Functions Example\n");
  console.log(
    "📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines\n",
  );

  const client = new EkoDBClient(BASE_URL, API_KEY);
  const scriptIds: string[] = [];

  try {
    await client.init();
  } catch (error) {
    console.error("⚠️  Auth initialization failed, retrying...");
    // Wait a moment and retry once
    await new Promise((resolve) => setTimeout(resolve, 1000));
    await client.init();
  }

  let primaryError: unknown;
  try {
    await setupTestData(client);
    scriptIds.push(await productStatsScript(client));
    scriptIds.push(await listProductsScript(client));
    scriptIds.push(await categoryCountScript(client));
    scriptIds.push(await topRatedScript(client));
    scriptIds.push(await scriptWithParameter(client));
    scriptIds.push(await multiStagePipeline(client));
  } catch (error) {
    console.error("❌ Error:", error);
    primaryError = error;
  } finally {
    try {
      await cleanup(client, scriptIds);
    } catch (cleanupError) {
      primaryError =
        primaryError === undefined
          ? cleanupError
          : new AggregateError(
              [primaryError, cleanupError],
              "Complete functions example and cleanup failed",
            );
    }
  }
  if (primaryError !== undefined) throw primaryError;
  console.log("✅ All complete script examples finished!");
  console.log("\n💡 This example demonstrates ekoDB's function system:");
  console.log("   ✅ FindAll operations");
  console.log("   ✅ Group aggregations (Count, Average)");
  console.log("   ✅ Multi-stage pipelines (FindAll → Group → Count)");
  console.log("   ✅ Parameter definitions");
  console.log("   ✅ Function management (save, call, delete)");
}

main().catch((error) => {
  console.error("❌ Error:", error);
  process.exit(1);
});
