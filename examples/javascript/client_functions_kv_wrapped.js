/**
 * KV Store & Wrapped Types Example for ekoDB JavaScript Client
 *
 * Demonstrates: KV operations in scripts, wrapped type field builders
 */

const { EkoDBClient, Stage, Field } = require("@ekodb/ekodb-client");
require("dotenv").config();

const BASE_URL = process.env.API_BASE_URL || "http://localhost:8080";
const API_KEY = process.env.API_BASE_KEY || "a-test-api-key-from-ekodb";

// =============================================================================
// Wrapped Types Examples
// =============================================================================

async function wrappedTypesInsert(client) {
  console.log("📝 Example 1: Inserting Records with Wrapped Types\n");

  // Insert an order with wrapped types for precise data handling
  const order = {
    order_id: Field.uuid("550e8400-e29b-41d4-a716-446655440000"),
    total: Field.decimal("1234.56"), // Precise decimal, not floating point
    created_at: Field.dateTime(new Date()),
    processing_time: Field.duration(3600000), // 1 hour in milliseconds
    quantity: Field.number(42),
    tags: Field.set(["priority", "express", "international"]),
    embedding: Field.vector([0.1, 0.2, 0.3, 0.4, 0.5]),
    metadata: Field.object({ source: "web", campaign: "summer2024" }),
  };

  const result = await client.insert("orders_example", order);
  console.log(`✅ Inserted order: ${result.id}`);

  // Insert multiple products with various wrapped types
  const products = [
    {
      sku: Field.uuid("a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11"),
      name: "Laptop Pro",
      price: Field.decimal("1299.99"),
      stock: Field.integer(15),
      rating: Field.float(4.8),
      available: Field.boolean(true),
      categories: Field.set(["electronics", "computers"]),
    },
    {
      sku: Field.uuid("b1ffcd00-0d1c-5fg9-cc7e-7cc0ce491b22"),
      name: "Wireless Mouse",
      price: Field.decimal("29.99"),
      stock: Field.integer(150),
      rating: Field.float(4.5),
      available: Field.boolean(true),
      categories: Field.set(["electronics", "accessories"]),
    },
  ];

  for (const product of products) {
    await client.insert("products_example", product);
  }
  console.log(`✅ Inserted ${products.length} products with wrapped types\n`);
}

async function wrappedTypesInScript(client) {
  console.log("📝 Example 2: Function with Wrapped Type Parameters\n");

  // Create a script that inserts records with wrapped types
  const script = {
    label: "create_order_with_types_js",
    name: "Create Order with Wrapped Types (JS)",
    description: "Demonstrates wrapped types in script insert operations",
    version: "1.0",
    parameters: {
      order_total: {
        param_type: "String",
        required: true,
      },
      order_id: {
        param_type: "String",
        required: true,
      },
      timestamp: {
        param_type: "DateTime",
        required: true,
        description: "Current UTC timestamp (ISO 8601)",
      },
    },
    functions: [
      Stage.insert("script_orders", {
        order_id: "{{order_id}}",
        total: { type: "Decimal", value: "{{order_total}}" },
        created_at: "{{timestamp}}",
        status: "pending",
      }),
    ],
    tags: ["orders", "wrapped-types"],
  };

  const id = await client.saveFunction(script);
  console.log(`✅ Function saved: ${id}`);

  const result = await client.callFunction("create_order_with_types_js", {
    order_total: "599.99",
    order_id: `order_${Date.now()}`,
    timestamp: new Date().toISOString(),
  });
  console.log(`📊 Created order via script`);
  console.log(`⏱️  Execution time: ${result.stats.execution_time_ms}ms\n`);

  return id;
}

// =============================================================================
// KV Store Examples
// =============================================================================

async function kvBasicOperations(client) {
  console.log("📝 Example 3: Basic KV Store Operations\n");

  // Set a simple value
  await client.kvSet("user:session:123", { userId: "user_abc", role: "admin" });
  console.log("✅ Set session data");

  // Get the value back
  const session = await client.kvGet("user:session:123");
  console.log(`📊 Retrieved session: ${JSON.stringify(session)}`);

  // Check if key exists
  const exists = await client.kvExists("user:session:123");
  console.log(`🔍 Key exists: ${exists}`);

  // Set with TTL (expires in 1 hour)
  await client.kvSet(
    "cache:product:456",
    { name: "Cached Product", price: 99.99 },
    3600
  );
  console.log("✅ Set cached data with 1 hour TTL");

  // Delete a key
  await client.kvDelete("user:session:123");
  console.log(`🗑️  Deleted session\n`);
}

async function kvScriptOperations(client) {
  console.log("📝 Example 4: KV Operations in Functions\n");

  const script = {
    label: "cached_product_lookup_js",
    name: "Cached Product Lookup (JS)",
    description: "Uses KV store for caching within a script",
    version: "1.0",
    parameters: {
      product_key: {
        param_type: "String",
        required: true,
      },
      product_data: {
        param_type: "Object",
        required: true,
      },
    },
    functions: [
      Stage.kvSet("{{product_key}}", "{{product_data}}", 3600),
      Stage.kvGet("{{product_key}}"),
    ],
    tags: ["kv", "caching"],
  };

  const id = await client.saveFunction(script);
  console.log(`✅ Function saved: ${id}`);

  const result = await client.callFunction("cached_product_lookup_js", {
    product_key: "product:cache:789",
    product_data: { name: "Test Product", price: 49.99 },
  });
  console.log(`📊 Cached and retrieved product data`);
  console.log(`⏱️  Execution time: ${result.stats.execution_time_ms}ms\n`);

  return id;
}

async function kvPatternQuery(client) {
  console.log("📝 Example 5: KV Pattern Query\n");

  // Set up multiple KV entries with a pattern
  await client.kvSet("config:app:theme", { mode: "dark" });
  await client.kvSet("config:app:language", { code: "en" });
  await client.kvSet("config:app:notifications", { enabled: true });
  await client.kvSet("config:user:preferences", { timezone: "UTC" });

  console.log("✅ Set 4 config entries");

  // Query all config:app:* keys
  const appConfigs = await client.kvQuery({ pattern: "config:app:*" });
  console.log(`📊 Found ${appConfigs.length} app config entries`);

  // Query all config:* keys
  const allConfigs = await client.kvQuery({ pattern: "config:*" });
  console.log(`📊 Found ${allConfigs.length} total config entries\n`);
}

// =============================================================================
// Combined Example
// =============================================================================

async function combinedExample(client) {
  console.log("📝 Example 6: Combined Wrapped Types + KV Function\n");

  const script = {
    label: "process_order_with_cache_js",
    name: "Process Order with Cache (JS)",
    description: "Demonstrates combined KV and wrapped type usage",
    version: "1.0",
    parameters: {
      order_id: {
        param_type: "String",
        required: true,
      },
      total: {
        param_type: "String",
        required: true,
      },
      timestamp: {
        param_type: "DateTime",
        required: true,
      },
    },
    functions: [
      Stage.kvSet(
        "order:status:{{order_id}}",
        {
          status: "processing",
          updated_at: "{{timestamp}}",
        },
        86400
      ),
      Stage.insert("processed_orders", {
        order_id: "{{order_id}}",
        total: { type: "Decimal", value: "{{total}}" },
        created_at: "{{timestamp}}",
        status: "processing",
      }),
      Stage.kvGet("order:status:{{order_id}}"),
    ],
    tags: ["orders", "kv", "wrapped-types", "combined"],
  };

  const id = await client.saveFunction(script);
  console.log(`✅ Function saved: ${id}`);

  const result = await client.callFunction("process_order_with_cache_js", {
    order_id: "c2d3e4f5-a1b2-c3d4-e5f6-a1b2c3d4e5f6",
    total: "299.99",
    timestamp: new Date().toISOString(),
  });
  console.log(`📊 Processed order with caching`);
  console.log(`⏱️  Stages executed: ${result.stats.stages_executed}`);
  console.log(`⏱️  Execution time: ${result.stats.execution_time_ms}ms\n`);

  return id;
}

// =============================================================================
// Cleanup
// =============================================================================

async function cleanup(client, scriptIds) {
  console.log("🧹 Cleaning up...");

  try {
    for (const id of scriptIds) {
      await client.deleteFunction(id);
    }

    await client.deleteCollection("orders_example");
    await client.deleteCollection("products_example");
    await client.deleteCollection("script_orders");
    await client.deleteCollection("processed_orders");

    await client.kvDelete("cache:product:456");
    await client.kvDelete("product:cache:789");
    await client.kvDelete("config:app:theme");
    await client.kvDelete("config:app:language");
    await client.kvDelete("config:app:notifications");
    await client.kvDelete("config:user:preferences");
    await client.kvDelete("order:status:c2d3e4f5-a1b2-c3d4-e5f6-a1b2c3d4e5f6");

    console.log("✅ Cleanup complete\n");
  } catch (error) {
    console.log("⚠️  Cleanup had some errors (may be expected)\n");
  }
}

// =============================================================================
// Main
// =============================================================================

async function main() {
  console.log("🚀 ekoDB JavaScript KV Store & Wrapped Types Example\n");
  console.log("📋 Demonstrates:");
  console.log("   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)");
  console.log("   • KV store operations (get, set, delete, exists, query)");
  console.log("   • KV operations within scripts");
  console.log("   • Combined wrapped types + KV workflows\n");

  const client = new EkoDBClient(BASE_URL, API_KEY);
  await client.init();

  const scriptIds = [];

  try {
    // Wrapped Types Examples
    await wrappedTypesInsert(client);
    scriptIds.push(await wrappedTypesInScript(client));

    // KV Store Examples
    await kvBasicOperations(client);
    scriptIds.push(await kvScriptOperations(client));
    await kvPatternQuery(client);

    // Combined Example
    scriptIds.push(await combinedExample(client));

    // Cleanup
    await cleanup(client, scriptIds);

    console.log("✅ All KV & Wrapped Types examples completed!");
    console.log("\n💡 Key takeaways:");
    console.log("   ✅ Use Field.* helpers for type-safe wrapped values");
    console.log("   ✅ Field.decimal() preserves precision (no floating point errors)");
    console.log("   ✅ KV store is great for caching and quick lookups");
    console.log("   ✅ Stage.kv*() functions work within scripts");
    console.log("   ✅ Combine KV caching with collection inserts for real workflows");
  } catch (error) {
    console.error("❌ Error:", error);
    process.exit(1);
  }
}

main().catch((error) => {
  console.error("❌ Error:", error);
  process.exit(1);
});
