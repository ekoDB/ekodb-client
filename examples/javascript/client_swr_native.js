/**
 * ekoDB JavaScript Client - Native SWR Function Example
 *
 * Demonstrates the native SWR (Stale-While-Revalidate) function that
 * encapsulates the entire cache-aside pattern in a single operation.
 */

const { EkoDBClient, Stage } = require("@ekodb/ekodb-client");
require("dotenv").config();

const BASE_URL = process.env.API_BASE_URL || "http://localhost:8080";
const API_KEY = process.env.API_BASE_KEY || "a-test-api-key-from-ekodb";
const FUNCTION_LABELS = {
  basic: "github_user_native_js",
  audit: "product_swr_audit_js",
  enrichment: "user_enrichment_pipeline_js",
  dynamic: "flexible_cache_js",
};
const COLLECTIONS = {
  audit: "swr_audit_trail_js",
  enrichment: "enriched_users_swr_js",
};
const KV_KEYS = {
  basicTemplate: "swr_native:js:github:user:{{username}}",
  basic: "swr_native:js:github:user:torvalds",
  auditTemplate: "swr_native:js:product:{{product_id}}",
  audit: "swr_native:js:product:1",
  enrichmentTemplate: "swr_native:js:api:user:{{user_id}}",
  enrichment: "swr_native:js:api:user:1",
  dynamicTemplate: "swr_native:js:resource:{{resource_id}}",
  dynamic: "swr_native:js:resource:test",
};

function isNotFoundError(error) {
  const message = String(error);
  return message.includes("404") || message.toLowerCase().includes("not found");
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
      if (!existing.id) {
        throw new Error(
          `Updated function '${script.label}' did not return an id`,
        );
      }
      return existing.id;
    }
    throw error;
  }
}

async function exampleBasicSWR(client) {
  console.log("\nExample 1: Basic Native SWR");
  console.log("─".repeat(80));
  console.log(
    "Single function replaces KvGet → If → HttpRequest → KvSet pipeline",
  );

  // Create script with native SWR function
  const basicSWRScript = {
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

async function exampleAuditTrail(client) {
  console.log("\nExample 2: SWR with Built-in Audit Trail");
  console.log("─".repeat(80));
  console.log("Optional collection parameter for automatic request logging");

  // Create script with audit trail
  const auditSWRScript = {
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

async function examplePipelineEnrichment(client) {
  console.log("\nExample 3: SWR in Multi-Function Pipeline");
  console.log("─".repeat(80));
  console.log("Fetch external data → Process → Store in collection");

  // Create enrichment pipeline
  const pipelineScript = {
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
        "24h",
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

async function exampleDynamicTTL(client) {
  console.log("\nExample 4: Dynamic TTL Configuration");
  console.log("─".repeat(80));
  console.log(
    "TTL as parameter - supports duration strings, integers, ISO timestamps",
  );

  // Create script with dynamic TTL
  const dynamicTTLScript = {
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

async function cleanup(client, scriptIds) {
  console.log("\n🧹 Cleaning up...");
  const errors = [];
  const cleanupIds = new Set(scriptIds);
  try {
    const functions = await client.listFunctions();
    for (const fn of functions) {
      if (Object.values(FUNCTION_LABELS).includes(fn.label) && fn.id) {
        cleanupIds.add(fn.id);
      }
    }
  } catch (error) {
    errors.push(["discover functions", error]);
  }
  let deleted = 0;
  for (const scriptId of cleanupIds) {
    try {
      await client.deleteFunction(scriptId);
      deleted += 1;
    } catch (error) {
      errors.push([`function ${scriptId}`, error]);
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
      if (!isNotFoundError(error)) errors.push([`KV key ${key}`, error]);
    }
  }
  for (const collection of Object.values(COLLECTIONS)) {
    try {
      await client.deleteCollection(collection);
    } catch (error) {
      if (!isNotFoundError(error))
        errors.push([`collection ${collection}`, error]);
    }
  }
  if (errors.length > 0) {
    const details = errors
      .map(([resource, error]) => `${resource}: ${error}`)
      .join("; ");
    console.log(`⚠ Cleanup failed: ${details}`);
    throw new Error(
      `Failed to clean up ${errors.length} owned resource(s): ${details}`,
    );
  }
  console.log(`✓ Deleted ${deleted} test functions and owned SWR resources`);
}

async function main() {
  console.log("🚀 ekoDB JavaScript Client - Native SWR Function Examples\n");
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

  const scriptIds = [];

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
    console.error("❌ Error:", error);
    throw error;
  } finally {
    await cleanup(client, scriptIds);
  }

  console.log("\n✅ All examples completed!");
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
