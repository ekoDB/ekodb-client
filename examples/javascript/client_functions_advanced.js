/**
 * Advanced Functions Example - Query, Sort, Limit, Group
 *
 * Demonstrates advanced query and aggregation operations using simple patterns
 */

const { EkoDBClient, Stage } = require("@ekodb/ekodb-client");
const dotenv = require("dotenv");

dotenv.config();

const BASE_URL = process.env.API_BASE_URL || "http://localhost:8080";
const API_KEY = process.env.API_BASE_KEY || "a-test-api-key-from-ekodb";
const RUN_SUFFIX = `${process.pid}_${Date.now()}`;
const COLLECTION = `advanced_products_js_${RUN_SUFFIX}`;
const LABELS = {
  list: `list_all_products_js_${RUN_SUFFIX}`,
  category: `products_by_category_js_${RUN_SUFFIX}`,
  count: `count_products_js_${RUN_SUFFIX}`,
  analysis: `category_analysis_js_${RUN_SUFFIX}`,
  summary: `product_summary_js_${RUN_SUFFIX}`,
};

function scalar(value) {
  return value && typeof value === "object" && "value" in value
    ? value.value
    : value;
}

function requireRecordCount(result, expected, label) {
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

/**
 * Save a function idempotently.
 *
 * The server returns HTTP 409 ("A function with label 'X' already exists.")
 * when a function with the same fixed label already exists. On that error we
 * UPDATE the existing function via PUT /api/functions/{label} (the server's
 * GET/PUT/DELETE routes accept either the encrypted ID or the label), then
 * resolve and return its encrypted ID so the rest of the example keeps working.
 * Any other error is propagated.
 */
async function saveOrUpdate(client, script) {
  try {
    return await client.saveFunction(script);
  } catch (error) {
    if (error.message && error.message.includes("already exists")) {
      await client.updateFunction(script.label, script);
      console.log(
        `ℹ️  Function '${script.label}' already existed — updated instead`,
      );
      const existing = await client.getFunction(script.label);
      return existing.id;
    }
    throw error;
  }
}

async function setupTestData(client) {
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
    {
      name: "Office Lamp",
      category: "Furniture",
      price: 79,
      stock: 18,
      rating: 4.05,
    },
    {
      name: "Webcam HD",
      category: "Electronics",
      price: 119,
      stock: 22,
      rating: 4.5,
    },
  ];

  for (const product of products) {
    await client.insert(COLLECTION, product);
  }

  console.log(`✅ Created ${products.length} products\n`);
}

async function listAllScript(client) {
  console.log("📝 Example 1: List All Products\n");

  const script = {
    label: LABELS.list,
    name: "List All Products",
    version: "1.0",
    parameters: {},
    functions: [Stage.findAll(COLLECTION)],
    tags: ["products", "list"],
  };

  const scriptId = await saveOrUpdate(client, script);
  console.log("✅ Function saved");

  const result = await client.callFunction(scriptId);
  const records = requireRecordCount(result, 10, "List products");
  console.log(`📊 Found ${records.length} products`);
  console.log(`⏱️  Execution time: ${result.stats.execution_time_ms}ms\n`);

  return scriptId;
}

async function groupByCategoryScript(client) {
  console.log("📝 Example 2: Group Products by Category\n");

  const script = {
    label: LABELS.category,
    name: "Products by Category",
    version: "1.0",
    parameters: {},
    functions: [
      Stage.findAll(COLLECTION),
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

  const scriptId = await saveOrUpdate(client, script);
  console.log("✅ Function saved");

  const result = await client.callFunction(scriptId);
  const records = requireRecordCount(result, 2, "Group products");
  const counts = Object.fromEntries(
    records.map((cat) => [
      String(scalar(cat.category)),
      Number(scalar(cat.count)),
    ]),
  );
  if (counts.Electronics !== 6 || counts.Furniture !== 4) {
    throw new Error(
      `Category counts were ${JSON.stringify(counts)}; expected Electronics=6 and Furniture=4`,
    );
  }
  console.log(`📊 Found ${records.length} categories`);
  records.forEach((cat) => {
    console.log(
      `   ${cat.category?.value || cat.category}: ${cat.count?.value || cat.count} items (avg $${(cat.avg_price?.value || cat.avg_price).toFixed(2)})`,
    );
  });
  console.log(`⏱️  Execution time: ${result.stats.execution_time_ms}ms\n`);

  return scriptId;
}

async function countProductsScript(client) {
  console.log("📝 Example 3: Count All Products\n");

  const script = {
    label: LABELS.count,
    name: "Count All Products",
    version: "1.0",
    parameters: {},
    functions: [Stage.findAll(COLLECTION), Stage.count()],
    tags: ["products", "count"],
  };

  const scriptId = await saveOrUpdate(client, script);
  console.log("✅ Function saved");

  const result = await client.callFunction(scriptId);
  const records = requireRecordCount(result, 1, "Count products");
  const count = Number(scalar(records[0].count));
  if (count !== 10)
    throw new Error(`Count products returned ${count}; expected 10`);
  console.log(`📊 Total products: ${count}`);
  console.log(`⏱️  Execution time: ${result.stats.execution_time_ms}ms\n`);

  return scriptId;
}

async function multiStageAggregationScript(client) {
  console.log("📝 Example 4: Multi-Stage Aggregation\n");

  const script = {
    label: LABELS.analysis,
    name: "Category Analysis",
    version: "1.0",
    parameters: {},
    functions: [
      Stage.findAll(COLLECTION),
      Stage.group(
        ["category"],
        [
          { output_field: "count", operation: "Count" },
          {
            output_field: "total_stock",
            operation: "Sum",
            input_field: "stock",
          },
          {
            output_field: "avg_rating",
            operation: "Average",
            input_field: "rating",
          },
        ],
      ),
    ],
    tags: ["products", "analytics", "multi-stage"],
  };

  const scriptId = await saveOrUpdate(client, script);
  console.log("✅ Function saved");

  const result = await client.callFunction(scriptId);
  const records = requireRecordCount(result, 2, "Category analysis");
  const aggregates = Object.fromEntries(
    records.map((category) => [
      String(scalar(category.category)),
      {
        count: Number(scalar(category.count)),
        stock: Number(scalar(category.total_stock)),
      },
    ]),
  );
  if (
    aggregates.Electronics?.count !== 6 ||
    aggregates.Electronics?.stock !== 232 ||
    aggregates.Furniture?.count !== 4 ||
    aggregates.Furniture?.stock !== 43
  ) {
    throw new Error(
      `Category analysis was ${JSON.stringify(aggregates)}; expected Electronics=6/232 and Furniture=4/43`,
    );
  }
  console.log(`📊 Category analysis (${records.length} categories):`);
  records.forEach((cat) => {
    console.log(`   ${cat.category?.value || cat.category}:`);
    console.log(
      `      Products: ${cat.count?.value || cat.count} | Stock: ${cat.total_stock?.value || cat.total_stock} | Avg Rating: ⭐${(cat.avg_rating?.value || cat.avg_rating).toFixed(2)}`,
    );
  });
  console.log(
    `\n⏱️  Total execution time: ${result.stats.execution_time_ms}ms`,
  );
  console.log("📈 Pipeline stages:");
  result.stats.stage_stats.forEach((stage, index) => {
    console.log(
      `   ${index + 1}. ${stage.stage}: ${stage.input_count} → ${stage.output_count} records (${stage.execution_time_ms}ms)`,
    );
  });
  console.log();

  return scriptId;
}

async function projectFieldsScript(client) {
  console.log("📝 Example 5: Project Specific Fields\n");

  const script = {
    label: LABELS.summary,
    name: "Product Summary",
    version: "1.0",
    parameters: {},
    functions: [
      Stage.findAll(COLLECTION),
      Stage.project(["name", "price", "rating"], false),
    ],
    tags: ["products", "projection"],
  };

  const scriptId = await saveOrUpdate(client, script);
  console.log("✅ Function saved");

  const result = await client.callFunction(scriptId);
  const records = requireRecordCount(result, 10, "Product summary");
  console.log(
    `📊 Product summaries (${records.length} items, showing first 3):`,
  );
  records.slice(0, 3).forEach((p, i) => {
    console.log(
      `   ${i + 1}. ${p.name?.value || p.name} - $${p.price?.value || p.price} (⭐${p.rating?.value || p.rating})`,
    );
  });
  console.log(`⏱️  Execution time: ${result.stats.execution_time_ms}ms\n`);

  return scriptId;
}

async function cleanup(client, scriptIds, primaryError) {
  console.log("🧹 Cleaning up...");

  const cleanupErrors = [];
  for (const id of scriptIds) {
    try {
      await client.deleteFunction(id);
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      if (!message.includes("status 404") && !/not found/i.test(message)) {
        cleanupErrors.push(error);
      }
    }
  }
  try {
    await client.deleteCollection(COLLECTION);
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    if (!message.includes("status 404") && !/not found/i.test(message)) {
      cleanupErrors.push(error);
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

async function main() {
  console.log("🚀 ekoDB Advanced Functions Example\n");

  const client = new EkoDBClient(BASE_URL, API_KEY);
  await client.init();
  const scriptIds = [];
  let primaryError;
  try {
    await setupTestData(client);
    scriptIds.push(await listAllScript(client));
    scriptIds.push(await groupByCategoryScript(client));
    scriptIds.push(await countProductsScript(client));
    scriptIds.push(await multiStageAggregationScript(client));
    scriptIds.push(await projectFieldsScript(client));
  } catch (error) {
    primaryError = error;
    throw error;
  } finally {
    await cleanup(client, scriptIds, primaryError);
  }

  console.log("✅ All advanced function examples completed!");
}

main().catch((error) => {
  console.error("❌ Error:", error);
  process.exitCode = 1;
});
