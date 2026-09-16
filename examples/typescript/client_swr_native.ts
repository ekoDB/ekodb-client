/**
 * ekoDB TypeScript Client - Native SWR Function Example
 *
 * Demonstrates the native SWR (Stale-While-Revalidate) function that
 * encapsulates the entire cache-aside pattern in a single operation.
 */

import { EkoDBClient, Stage, UserFunction } from "@ekodb/ekodb-client";
import * as dotenv from "dotenv";

dotenv.config();

const BASE_URL = process.env.API_BASE_URL || "http://localhost:8080";
const API_KEY = process.env.API_BASE_KEY || "a-test-api-key-from-ekodb";
const FUNCTION_LABELS = {
  basic: "github_user_native_ts",
  audit: "product_swr_audit_ts",
  enrichment: "user_enrichment_pipeline_ts",
  dynamic: "flexible_cache_ts",
} as const;
const COLLECTIONS = {
  audit: "swr_audit_trail_ts",
  enrichment: "enriched_users_swr_ts",
} as const;
const KV_KEYS = {
  basicTemplate: "swr_native:ts:github:user:{{username}}",
  basic: "swr_native:ts:github:user:torvalds",
  auditTemplate: "swr_native:ts:product:{{product_id}}",
  audit: "swr_native:ts:product:1",
  enrichmentTemplate: "swr_native:ts:api:user:{{user_id}}",
  enrichment: "swr_native:ts:api:user:1",
  dynamicTemplate: "swr_native:ts:resource:{{resource_id}}",
  dynamic: "swr_native:ts:resource:test",
} as const;

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
    if (!existing.id) {
      throw new Error(
        `Updated function '${script.label}' did not return an id`,
      );
    }
    return existing.id;
  }
}

async function exampleBasicSWR(client: EkoDBClient): Promise<string> {
  console.log("\nExample 1: Basic Native SWR");
  console.log("─".repeat(80));
  console.log(
    "Single function replaces KvGet → If → HttpRequest → KvSet pipeline",
  );

  // Create script with native SWR function
  const basicSWRScript: UserFunction = {
    label: FUNCTION_LABELS.basic,
    name: "GitHub User Lookup (Native SWR)",
    description:
      "Fetches GitHub user data with automatic caching using native SWR",
    parameters: {
      username: {
        description: "GitHub username to fetch",
        required: true,
      },
    },
    functions: [
      Stage.swr(
        KV_KEYS.basicTemplate,
        "15m",
        "https://api.github.com/users/{{username}}",
        "GET",
        { "User-Agent": "ekoDB-Client" },
        undefined,
        undefined,
        "user_data",
      ),
    ],
    tags: ["github", "swr", "native"],
  };

  const scriptId = await saveOrUpdate(client, basicSWRScript);
  console.log(
    `✓ Created native SWR script: ${FUNCTION_LABELS.basic} (${scriptId})`,
  );

  // First call - cache miss
  console.log("\nFirst call (cache miss - will fetch from GitHub API):");
  const start1 = Date.now();
  const result1 = await client.callFunction(scriptId, {
    username: "torvalds",
  });
  const duration1 = Date.now() - start1;
  console.log(`  Response time: ${duration1}ms`);
  console.log(`  Records returned: ${result1.records?.length || 0}`);

  // Second call - cache hit
  console.log("\nSecond call (cache hit - instant from KV store):");
  const start2 = Date.now();
  const result2 = await client.callFunction(scriptId, {
    username: "torvalds",
  });
  const duration2 = Date.now() - start2;
  const speedup = duration2 > 0 ? duration1 / duration2 : 0;
  console.log(`  Response time: ${duration2}ms`);
  console.log(`  Speedup: ${speedup.toFixed(1)}x faster 🚀`);
  console.log(`  Records returned: ${result2.records?.length || 0}\n`);

  return scriptId;
}

async function exampleAuditTrail(client: EkoDBClient): Promise<string> {
  console.log("\nExample 2: SWR with Built-in Audit Trail");
  console.log("─".repeat(80));
  console.log("Optional collection parameter for automatic request logging");

  // Create script with audit trail
  const auditSWRScript: UserFunction = {
    label: FUNCTION_LABELS.audit,
    name: "Product API with Audit (Native SWR)",
    description: "Caches product data and logs all requests automatically",
    parameters: {
      product_id: {
        description: "Product ID to fetch",
        required: true,
      },
    },
    functions: [
      Stage.swr(
        KV_KEYS.auditTemplate,
        "1h",
        "https://fakestoreapi.com/products/{{product_id}}",
        "GET",
        undefined,
        undefined,
        undefined,
        "product",
        COLLECTIONS.audit,
      ),
    ],
    tags: ["products", "audit"],
  };

  const auditScriptId = await saveOrUpdate(client, auditSWRScript);
  console.log(
    `✓ Created SWR script with audit trail: ${FUNCTION_LABELS.audit} (${auditScriptId})`,
  );

  console.log("\nFetching product (will create audit trail entry):");
  const productResult = await client.callFunction(auditScriptId, {
    product_id: "1",
  });
  console.log("  ✓ Product fetched and cached");
  console.log(`  ✓ Audit record created in '${COLLECTIONS.audit}' collection`);
  console.log(`  Records: ${productResult.records?.length || 0}\n`);

  return auditScriptId;
}

async function examplePipelineEnrichment(client: EkoDBClient): Promise<string> {
  console.log("\nExample 3: SWR in Multi-Function Pipeline");
  console.log("─".repeat(80));
  console.log("Fetch external data → Process → Store in collection");

  // Create enrichment pipeline
  const pipelineScript: UserFunction = {
    label: FUNCTION_LABELS.enrichment,
    name: "User Data Enrichment Pipeline",
    description: "Fetches external API data and stores enriched results",
    parameters: {
      user_id: {
        description: "User ID to enrich",
        required: true,
      },
    },
    functions: [
      // Step 1: Fetch from external API with caching (30 min TTL)
      Stage.swr(
        KV_KEYS.enrichmentTemplate,
        "30m",
        "https://jsonplaceholder.typicode.com/users/{{user_id}}",
        "GET",
        undefined,
        undefined,
        undefined,
        "user_data",
      ),
      // Step 2: Store enriched data in collection (24 hour TTL)
      Stage.insert(
        COLLECTIONS.enrichment,
        {
          user_id: { type: "String", value: "{{user_id}}" },
          source_data: { type: "Object", value: "{{user_data}}" },
        },
        false,
        86400,
      ),
    ],
    tags: ["enrichment", "pipeline"],
  };

  const pipelineScriptId = await saveOrUpdate(client, pipelineScript);
  console.log(
    `✓ Created enrichment pipeline: ${FUNCTION_LABELS.enrichment} (${pipelineScriptId})`,
  );

  console.log("\nRunning pipeline:");
  const enrichResult = await client.callFunction(pipelineScriptId, {
    user_id: "1",
  });
  console.log("  ✓ Data fetched from API (cached 30m)");
  console.log(
    `  ✓ Enriched data stored in '${COLLECTIONS.enrichment}' (TTL 24h)`,
  );
  console.log(
    `  Pipeline returned ${enrichResult.records?.length || 0} records\n`,
  );

  return pipelineScriptId;
}

async function exampleDynamicTTL(client: EkoDBClient): Promise<string> {
  console.log("\nExample 4: Dynamic TTL Configuration");
  console.log("─".repeat(80));
  console.log(
    "TTL as parameter - supports duration strings, integers, ISO timestamps",
  );

  // Create script with dynamic TTL
  const dynamicTTLScript: UserFunction = {
    label: FUNCTION_LABELS.dynamic,
    name: "Flexible Cache TTL (Native SWR)",
    description: "Demonstrates parameterized TTL values",
    parameters: {
      resource_id: {
        description: "Resource to fetch",
        required: true,
      },
      ttl: {
        description: "Cache duration (e.g., '5m', '1h', '30s')",
        required: true,
      },
    },
    functions: [
      Stage.swr(
        KV_KEYS.dynamicTemplate,
        "{{ttl}}",
        "https://jsonplaceholder.typicode.com/posts/{{resource_id}}",
        "GET",
      ),
    ],
    tags: ["dynamic"],
  };

  const dynamicScriptId = await saveOrUpdate(client, dynamicTTLScript);
  console.log(
    `✓ Created dynamic TTL script: ${FUNCTION_LABELS.dynamic} (${dynamicScriptId})`,
  );

  // Test with different TTLs
  const ttlTests = [
    { value: "5m", description: "5 minutes" },
    { value: "1h", description: "1 hour" },
    { value: "30s", description: "30 seconds" },
  ];

  for (const test of ttlTests) {
    await client.callFunction(dynamicScriptId, {
      resource_id: "test",
      ttl: test.value,
    });
    console.log(`  ✓ Cached with TTL: ${test.value} (${test.description})`);
  }

  return dynamicScriptId;
}

async function cleanup(
  client: EkoDBClient,
  scriptIds: string[],
): Promise<void> {
  console.log("\n🧹 Cleaning up...");
  const failures: unknown[] = [];
  const cleanupIds = new Set(scriptIds);
  try {
    const functions = await client.listFunctions();
    for (const fn of functions) {
      if (
        Object.values(FUNCTION_LABELS).includes(
          fn.label as (typeof FUNCTION_LABELS)[keyof typeof FUNCTION_LABELS],
        ) &&
        fn.id
      ) {
        cleanupIds.add(fn.id);
      }
    }
  } catch (error) {
    failures.push(error);
  }
  for (const scriptId of cleanupIds) {
    try {
      await client.deleteFunction(scriptId);
    } catch (error) {
      failures.push(error);
    }
  }
  for (const key of [
    KV_KEYS.basic,
    KV_KEYS.audit,
    KV_KEYS.enrichment,
    KV_KEYS.dynamic,
  ]) {
    try {
      await client.kvDelete(key);
    } catch (error) {
      if (!isNotFoundError(error)) failures.push(error);
    }
  }
  for (const collection of Object.values(COLLECTIONS)) {
    try {
      await client.deleteCollection(collection);
    } catch (error) {
      if (!isNotFoundError(error)) failures.push(error);
    }
  }
  if (failures.length === 0) {
    console.log(
      `✓ Deleted ${cleanupIds.size} test scripts and owned SWR resources`,
    );
    return;
  }
  throw new AggregateError(failures, "Native SWR cleanup failed");
}

function isNotFoundError(error: unknown): boolean {
  const message = error instanceof Error ? error.message : String(error);
  return message.includes("404") || message.toLowerCase().includes("not found");
}

async function main() {
  console.log("🚀 ekoDB TypeScript Client - Native SWR Function Examples\n");
  console.log("📋 Demonstrates:");
  console.log("   • Single-function SWR pattern (replaces 4-step pipeline)");
  console.log(
    "   • Automatic cache checking, HTTP fetching, and cache setting",
  );
  console.log("   • Built-in audit trail support");
  console.log("   • Duration string TTLs ('15m', '1h', '30s')");
  console.log("   • Multi-function pipeline integration");
  console.log("   • Dynamic TTL configuration\n");

  const client = new EkoDBClient(BASE_URL, API_KEY);
  await client.init();

  const scriptIds: string[] = [];
  let primaryError: unknown;

  try {
    await cleanup(client, scriptIds);
    // Run examples
    scriptIds.push(await exampleBasicSWR(client));
    scriptIds.push(await exampleAuditTrail(client));
    scriptIds.push(await examplePipelineEnrichment(client));
    scriptIds.push(await exampleDynamicTTL(client));

    // Summary
    console.log("\n" + "=".repeat(80));
    console.log("✅ Key Benefits of Native SWR:");
    console.log("✅ Single function: Replaces 4-function cache-aside pattern");
    console.log(
      "✅ Duration strings: Use '15m', '1h', '2h' instead of calculating seconds",
    );
    console.log(
      "✅ Built-in audit: Optional collection parameter for automatic logging",
    );
    console.log(
      "✅ Auto-enrichment: output_field populates params for downstream functions",
    );
    console.log(
      "✅ Transactional: Works correctly in both transactional and non-transactional contexts",
    );
    console.log(
      "✅ KV-optimized: Uses native KV store with proper TTL handling",
    );

    console.log("\n=== Performance Comparison ===");
    console.log(
      "Legacy Pattern: KvGet → If → HttpRequest → KvSet → Insert (5 functions)",
    );
    console.log("Native SWR:     SWR → Insert (2 functions)");
    console.log(
      "Result:         60% fewer functions, cleaner code, same behavior 🎯",
    );
  } catch (error) {
    primaryError = error;
    console.error("❌ Error:", error);
    throw error;
  } finally {
    try {
      await cleanup(client, scriptIds);
    } catch (cleanupError) {
      if (primaryError !== undefined) {
        throw new AggregateError(
          [primaryError, cleanupError],
          "Native SWR example and cleanup failed",
        );
      }
      throw cleanupError;
    }
  }

  console.log("\n✅ All examples completed!");
}

main().catch((error) => {
  console.error(error);
  process.exit(1);
});
