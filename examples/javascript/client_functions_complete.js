/**
 * Complete Scripts Example - ekoDB Scripts
 * Demonstrates: FindAll, Group, Count, Multi-stage Pipelines
 */

const { EkoDBClient } = require("@ekodb/ekodb-client");
require("dotenv").config();

const BASE_URL = process.env.API_BASE_URL || "http://localhost:8080";
const API_KEY = process.env.API_BASE_KEY || "a-test-api-key-from-ekodb";

async function setupTestData(client) {
  console.log("📋 Setting up complete test data...");

  const products = [
    { name: "Laptop Pro", category: "Electronics", price: 1299, stock: 15, rating: 4.8 },
    { name: "Wireless Mouse", category: "Electronics", price: 29, stock: 45, rating: 4.5 },
    { name: "Desk Chair", category: "Furniture", price: 349, stock: 8, rating: 4.2 },
    { name: "Standing Desk", category: "Furniture", price: 599, stock: 12, rating: 4.7 },
    { name: 'Monitor 27"', category: "Electronics", price: 399, stock: 20, rating: 4.6 },
  ];

  for (const product of products) {
    await client.insert("complete_products_js", product);
  }

  console.log(`✅ Created ${products.length} products\n`);
}

async function advancedQueryFunction(client) {
  console.log("📝 Example 1: FindAll + Group (Simple Aggregation)\n");

  const script = {
    label: "product_stats_js",
    name: "Product Statistics",
    version: "1.0",
    parameters: {},
    functions: [
      { type: "FindAll", collection: "complete_products_js" },
      {
        type: "Group",
        by_fields: ["category"],
        functions: [
          { output_field: "count", operation: "Count" },
          { output_field: "avg_price", operation: "Average", input_field: "price" },
        ],
      },
    ],
    tags: ["products", "analytics"],
  };

  const scriptId = await client.saveScript(script);
  console.log(`✅ Script saved: ${scriptId}`);

  const result = await client.callScript("product_stats_js", null);

  console.log(`📊 Found ${result.records?.length || 0} product groups`);
  if (result.records) {
    result.records.forEach((record) => console.log(`   ${JSON.stringify(record)}`));
  }
  console.log(`⏱️  Execution time: ${result.stats?.execution_time_ms}ms\n`);

  return scriptId;
}

async function listProductsScript(client) {
  console.log("📝 Example 2: Simple Product Listing\n");

  const script = {
    label: "list_all_products_js",
    name: "List All Products",
    version: "1.0",
    parameters: {},
    functions: [{ type: "FindAll", collection: "complete_products_js" }],
    tags: ["products", "list"],
  };

  const scriptId = await client.saveScript(script);
  console.log("✅ Script saved");

  const result = await client.callScript("list_all_products_js", null);

  console.log(`📊 Found ${result.records?.length || 0} products`);
  console.log(`⏱️  Execution time: ${result.stats?.execution_time_ms}ms\n`);

  return scriptId;
}

async function categoryCountScript(client) {
  console.log("📝 Example 3: Count by Category\n");

  const script = {
    label: "count_by_category_js",
    name: "Count Products by Category",
    version: "1.0",
    parameters: {},
    functions: [
      { type: "FindAll", collection: "complete_products_js" },
      {
        type: "Group",
        by_fields: ["category"],
        functions: [{ output_field: "count", operation: "Count" }],
      },
    ],
    tags: ["products", "analytics"],
  };

  const scriptId = await client.saveScript(script);
  console.log("✅ Script saved");

  const result = await client.callScript("count_by_category_js", null);

  console.log(`📊 Found ${result.records?.length || 0} categories`);
  if (result.records) {
    result.records.forEach((record) => console.log(`   ${JSON.stringify(record)}`));
  }
  console.log(`⏱️  Execution time: ${result.stats?.execution_time_ms}ms\n`);

  return scriptId;
}

async function topRatedScript(client) {
  console.log("📝 Example 4: High Rating Products\n");

  const script = {
    label: "top_rated_products_js",
    name: "Top Rated Products",
    version: "1.0",
    parameters: {},
    functions: [{ type: "FindAll", collection: "complete_products_js" }],
    tags: ["products", "quality"],
  };

  const scriptId = await client.saveScript(script);
  console.log("✅ Script saved");

  const result = await client.callScript("top_rated_products_js", null);

  console.log(`📊 Found ${result.records?.length || 0} products`);
  console.log(`⏱️  Execution time: ${result.stats?.execution_time_ms}ms\n`);

  return scriptId;
}

async function scriptWithParameter(client) {
  console.log("📝 Example 5: Script with Parameter Definition\n");

  const script = {
    label: "list_with_limit_js",
    name: "List Products with Limit",
    version: "1.0",
    parameters: {
      max_items: {
        param_type: "Integer",
        required: false,
        default: 5,
      },
    },
    functions: [{ type: "FindAll", collection: "complete_products_js" }],
    tags: ["products", "list"],
  };

  const scriptId = await client.saveScript(script);
  console.log("✅ Script saved");

  const result = await client.callScript("list_with_limit_js", { max_items: 3 });

  console.log(`📊 Found ${result.records?.length || 0} products`);
  console.log(`⏱️  Execution time: ${result.stats?.execution_time_ms}ms\n`);

  return scriptId;
}

async function multiStagePipeline(client) {
  console.log("📝 Example 6: Multi-Stage Pipeline (FindAll → Group → Count)\n");

  const script = {
    label: "product_summary_js",
    name: "Product Summary Report",
    version: "1.0",
    parameters: {},
    functions: [
      { type: "FindAll", collection: "complete_products_js" },
      {
        type: "Group",
        by_fields: ["category"],
        functions: [
          { output_field: "count", operation: "Count" },
          { output_field: "avg_price", operation: "Average", input_field: "price" },
        ],
      },
      { type: "Count", output_field: "total_categories" },
    ],
    tags: ["products", "analytics"],
  };

  const scriptId = await client.saveScript(script);
  console.log("✅ Script saved");

  const result = await client.callScript("product_summary_js", null);

  console.log(`📊 Pipeline executed ${result.stats?.stages_executed} stages`);
  console.log(`⏱️  Total execution time: ${result.stats?.execution_time_ms}ms\n`);

  return scriptId;
}

async function cleanup(client, scriptIds) {
  console.log("🧹 Cleaning up...");

  try {
    for (const scriptId of scriptIds) {
      await client.deleteScript(scriptId);
    }
    await client.deleteCollection("complete_products_js");
    console.log("✅ Cleanup complete\n");
  } catch (e) {
    console.log(`⚠️  Cleanup had some errors: ${e.message}\n`);
  }
}

async function main() {
  console.log("🚀 ekoDB JavaScript Complete Scripts Example\n");
  console.log("📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines\n");

  const client = new EkoDBClient(BASE_URL, API_KEY);
  await client.init();

  try {
    await setupTestData(client);

    const scriptIds = [];
    scriptIds.push(await advancedQueryFunction(client));
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
  } catch (e) {
    console.error(`❌ Error: ${e.message}`);
  }
}

main().catch(console.error);
