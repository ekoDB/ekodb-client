/**
 * KV Store & Wrapped Types Example for ekoDB TypeScript Client
 *
 * Demonstrates: KV operations in scripts, wrapped type field builders
 */

import { EkoDBClient, Stage, Field, UserFunction } from "@ekodb/ekodb-client";
import * as dotenv from "dotenv";

dotenv.config();

const BASE_URL = process.env.API_BASE_URL || "http://localhost:8080";
const API_KEY = process.env.API_BASE_KEY || "a-test-api-key-from-ekodb";
const ORDERS_COLLECTION = "kv_wrapped_orders_ts";
const PRODUCTS_COLLECTION = "kv_wrapped_products_ts";
const SCRIPT_ORDERS_COLLECTION = "kv_wrapped_script_orders_ts";
const PROCESSED_ORDERS_COLLECTION = "kv_wrapped_processed_orders_ts";
const CREATE_ORDER_LABEL = "kv_wrapped_create_order_ts";
const CACHED_PRODUCT_LABEL = "kv_wrapped_cached_product_ts";
const PROCESS_ORDER_LABEL = "kv_wrapped_process_order_ts";
const SESSION_KEY = "kv_wrapped_ts:user:session:123";
const CACHE_KEY = "kv_wrapped_ts:cache:product:456";
const PRODUCT_KEY = "kv_wrapped_ts:product:cache:789";
const CONFIG_KEYS = [
  "kv_wrapped_ts:config:app:theme",
  "kv_wrapped_ts:config:app:language",
  "kv_wrapped_ts:config:app:notifications",
  "kv_wrapped_ts:config:user:preferences",
] as const;
const ORDER_ID = "c2d3e4f5-a1b2-c3d4-e5f6-a1b2c3d4e5f6";
const ORDER_STATUS_KEY = `kv_wrapped_ts:order:status:${ORDER_ID}`;
const FUNCTION_LABELS = [
  CREATE_ORDER_LABEL,
  CACHED_PRODUCT_LABEL,
  PROCESS_ORDER_LABEL,
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

// =============================================================================
// Wrapped Types Examples
// =============================================================================

async function wrappedTypesInsert(client: EkoDBClient): Promise<void> {
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

  const result = await client.insert(ORDERS_COLLECTION, order);
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
    await client.insert(PRODUCTS_COLLECTION, product);
  }
  console.log(`✅ Inserted ${products.length} products with wrapped types\n`);
}

async function wrappedTypesInScript(client: EkoDBClient): Promise<string> {
  console.log("📝 Example 2: UserFunction with Wrapped Type Parameters\n");

  // Create a script that inserts records with wrapped types
  const script = {
    label: CREATE_ORDER_LABEL,
    name: "Create Order with Wrapped Types",
    description: "Demonstrates wrapped types in script insert operations",
    version: "1.0",
    parameters: {
      order_total: {
        required: true,
      },
      order_id: {
        required: true,
      },
      timestamp: {
        required: true,
        description: "Current UTC timestamp (ISO 8601)",
      },
    },
    functions: [
      Stage.insert(SCRIPT_ORDERS_COLLECTION, {
        order_id: "{{order_id}}",
        total: { type: "Decimal", value: "{{order_total}}" },
        created_at: "{{timestamp}}",
        status: "pending",
      }),
    ],
    tags: ["orders", "wrapped-types"],
  };

  const id = await saveOrUpdate(client, script);
  console.log(`✅ Function saved: ${id}`);

  // Execute the script with a decimal parameter
  const result = await client.callFunction(id, {
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

async function kvBasicOperations(client: EkoDBClient): Promise<void> {
  console.log("📝 Example 3: Basic KV Store Operations\n");

  // Set a simple value
  await client.kvSet(SESSION_KEY, { userId: "user_abc", role: "admin" });
  console.log("✅ Set session data");

  // Get the value back
  const session = await client.kvGet(SESSION_KEY);
  console.log(`📊 Retrieved session: ${JSON.stringify(session)}`);

  // Check if key exists
  const exists = await client.kvExists(SESSION_KEY);
  console.log(`🔍 Key exists: ${exists}`);

  // Set with TTL (expires in 1 hour)
  await client.kvSet(CACHE_KEY, { name: "Cached Product", price: 99.99 }, 3600);
  console.log("✅ Set cached data with 1 hour TTL");

  // Delete a key
  await client.kvDelete(SESSION_KEY);
  console.log(`🗑️  Deleted session\n`);
}

async function kvScriptOperations(client: EkoDBClient): Promise<string> {
  console.log("📝 Example 4: KV Operations in Functions\n");

  // function that uses KV for caching query results
  const script = {
    label: CACHED_PRODUCT_LABEL,
    name: "Cached Product Lookup",
    description: "Uses KV store for caching within a script",
    version: "1.0",
    parameters: {
      product_key: {
        required: true,
      },
      product_data: {
        required: true,
      },
    },
    functions: [
      // Store product data in KV cache
      Stage.kvSet("{{product_key}}", "{{product_data}}", 3600),
      // Retrieve it back
      Stage.kvGet("{{product_key}}"),
    ],
    tags: ["kv", "caching"],
  };

  const id = await saveOrUpdate(client, script);
  console.log(`✅ Function saved: ${id}`);

  // Execute the caching script
  const result = await client.callFunction(id, {
    product_key: PRODUCT_KEY,
    product_data: { name: "Test Product", price: 49.99 },
  });
  console.log(`📊 Cached and retrieved product data`);
  console.log(`⏱️  Execution time: ${result.stats.execution_time_ms}ms\n`);

  return id;
}

async function kvPatternQuery(client: EkoDBClient): Promise<void> {
  console.log("📝 Example 5: KV Pattern Query\n");

  // Set up multiple KV entries with a pattern
  await client.kvSet(CONFIG_KEYS[0], { mode: "dark" });
  await client.kvSet(CONFIG_KEYS[1], { code: "en" });
  await client.kvSet(CONFIG_KEYS[2], { enabled: true });
  await client.kvSet(CONFIG_KEYS[3], { timezone: "UTC" });

  console.log("✅ Set 4 config entries");

  // Query all config:app:* keys
  const appConfigs = await client.kvQuery({
    pattern: "kv_wrapped_ts:config:app:*",
  });
  console.log(`📊 Found ${appConfigs.length} app config entries`);

  // Query all config:* keys
  const allConfigs = await client.kvQuery({
    pattern: "kv_wrapped_ts:config:*",
  });
  console.log(`📊 Found ${allConfigs.length} total config entries\n`);
}

// =============================================================================
// Combined Example: Wrapped Types + KV
// =============================================================================

async function combinedExample(client: EkoDBClient): Promise<string> {
  console.log("📝 Example 6: Combined Wrapped Types + KV Function\n");

  // A real-world script that:
  // 1. Stores order metadata in KV for quick access
  // 2. Inserts the full order with wrapped types
  const script = {
    label: PROCESS_ORDER_LABEL,
    name: "Process Order with Cache",
    description: "Demonstrates combined KV and wrapped type usage",
    version: "1.0",
    parameters: {
      order_id: {
        required: true,
      },
      total: {
        required: true,
      },
      timestamp: {
        required: true,
      },
    },
    functions: [
      // Cache order status in KV for quick lookups
      Stage.kvSet(
        "kv_wrapped_ts:order:status:{{order_id}}",
        {
          status: "processing",
          updated_at: "{{timestamp}}",
        },
        86400, // 24 hour TTL
      ),
      // Insert full order record with wrapped types
      Stage.insert(PROCESSED_ORDERS_COLLECTION, {
        order_id: "{{order_id}}",
        total: { type: "Decimal", value: "{{total}}" },
        created_at: "{{timestamp}}",
        status: "processing",
      }),
      // Verify the cache entry
      Stage.kvGet("kv_wrapped_ts:order:status:{{order_id}}"),
    ],
    tags: ["orders", "kv", "wrapped-types", "combined"],
  };

  const id = await saveOrUpdate(client, script);
  console.log(`✅ Function saved: ${id}`);

  // Execute the combined script
  const result = await client.callFunction(id, {
    order_id: ORDER_ID,
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
  for (const collection of [
    ORDERS_COLLECTION,
    PRODUCTS_COLLECTION,
    SCRIPT_ORDERS_COLLECTION,
    PROCESSED_ORDERS_COLLECTION,
  ]) {
    try {
      await client.deleteCollection(collection);
    } catch (error) {
      if (!isNotFoundError(error)) errors.push(error);
    }
  }
  for (const key of [
    SESSION_KEY,
    CACHE_KEY,
    PRODUCT_KEY,
    ...CONFIG_KEYS,
    ORDER_STATUS_KEY,
  ]) {
    try {
      await client.kvDelete(key);
    } catch (error) {
      if (!isNotFoundError(error)) errors.push(error);
    }
  }
  if (errors.length > 0) {
    console.log("⚠️  Cleanup had some errors (may be expected)\n");
    throw new AggregateError(errors, "KV/wrapped cleanup failed");
  }
  console.log("✅ Cleanup complete\n");
}

// =============================================================================
// Main
// =============================================================================

async function main() {
  console.log("🚀 ekoDB TypeScript KV Store & Wrapped Types Example\n");
  console.log("📋 Demonstrates:");
  console.log(
    "   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)",
  );
  console.log("   • KV store operations (get, set, delete, exists, query)");
  console.log("   • KV operations within scripts");
  console.log("   • Combined wrapped types + KV workflows\n");

  const client = new EkoDBClient(BASE_URL, API_KEY);
  await client.init();
  const scriptIds: string[] = [];

  let primaryError: unknown;

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
              "KV/wrapped example and cleanup failed",
            );
    }
  }
  if (primaryError !== undefined) throw primaryError;
  console.log("✅ All KV & Wrapped Types examples completed!");
  console.log("\n💡 Key takeaways:");
  console.log("   ✅ Use Field.* helpers for type-safe wrapped values");
  console.log(
    "   ✅ Field.decimal() preserves precision (no floating point errors)",
  );
  console.log("   ✅ KV store is great for caching and quick lookups");
  console.log("   ✅ Stage.kv*() functions work within scripts");
  console.log(
    "   ✅ Combine KV caching with collection inserts for real workflows",
  );
}

main().catch((error) => {
  console.error("❌ Error:", error);
  process.exit(1);
});
