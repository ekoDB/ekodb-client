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

async function exampleBasicSWR(client) {
  console.log("\nExample 1: Basic Native SWR");
  console.log("─".repeat(80));
  console.log("Single function replaces KvGet → If → HttpRequest → KvSet pipeline");

  // Create script with native SWR function
  const basicSWRScript = {
    label: "github_user_native",
    name: "GitHub User Lookup (Native SWR)",
    description: "Fetches GitHub user data with automatic caching using native SWR",
    parameters: {
      username: {
        type: "String",
        description: "GitHub username to fetch",
        required: true,
      },
    },
    functions: [
      Stage.swr(
        "github:user:{{username}}",
        "15m",
        "https://api.github.com/users/{{username}}",
        "GET",
        { "User-Agent": "ekoDB-Client" },
        undefined,
        undefined,
        "user_data"
      ),
    ],
    tags: ["github", "swr", "native"],
  };

  const scriptId = await client.saveScript(basicSWRScript);
  console.log(`✓ Created native SWR script: github_user_native (${scriptId})`);

  // First call - cache miss
  console.log("\nFirst call (cache miss - will fetch from GitHub API):");
  const start1 = Date.now();
  const result1 = await client.callScript("github_user_native", {
    username: "torvalds",
  });
  const duration1 = Date.now() - start1;
  console.log(`  Response time: ${duration1}ms`);
  console.log(`  Records returned: ${result1.records?.length || 0}`);

  // Second call - cache hit
  console.log("\nSecond call (cache hit - instant from KV store):");
  const start2 = Date.now();
  const result2 = await client.callScript("github_user_native", {
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
    label: "product_swr_audit",
    name: "Product API with Audit (Native SWR)",
    description: "Caches product data and logs all requests automatically",
    parameters: {
      product_id: {
        type: "String",
        description: "Product ID to fetch",
        required: true,
      },
    },
    functions: [
      Stage.swr(
        "product:{{product_id}}",
        "1h",
        "https://fakestoreapi.com/products/{{product_id}}",
        "GET",
        undefined,
        undefined,
        undefined,
        "product",
        "swr_audit_trail"
      ),
    ],
    tags: ["products", "audit"],
  };

  const auditScriptId = await client.saveScript(auditSWRScript);
  console.log(
    `✓ Created SWR script with audit trail: product_swr_audit (${auditScriptId})`
  );

  console.log("\nFetching product (will create audit trail entry):");
  const productResult = await client.callScript("product_swr_audit", {
    product_id: "1",
  });
  console.log("  ✓ Product fetched and cached");
  console.log("  ✓ Audit record created in 'swr_audit_trail' collection");
  console.log(`  Records: ${productResult.records?.length || 0}\n`);

  return auditScriptId;
}

async function examplePipelineEnrichment(client) {
  console.log("\nExample 3: SWR in Multi-Function Pipeline");
  console.log("─".repeat(80));
  console.log("Fetch external data → Process → Store in collection");

  // Create enrichment pipeline
  const pipelineScript = {
    label: "user_enrichment_pipeline",
    name: "User Data Enrichment Pipeline",
    description: "Fetches external API data and stores enriched results",
    parameters: {
      user_id: {
        type: "String",
        description: "User ID to enrich",
        required: true,
      },
    },
    functions: [
      // Step 1: Fetch from external API with caching (30 min TTL)
      Stage.swr(
        "api:user:{{user_id}}",
        "30m",
        "https://jsonplaceholder.typicode.com/users/{{user_id}}",
        "GET",
        undefined,
        undefined,
        undefined,
        "user_data"
      ),
      // Step 2: Store enriched data in collection (24 hour TTL)
      Stage.insert(
        "enriched_users",
        {
          user_id: { type: "String", value: "{{user_id}}" },
          source_data: { type: "Object", value: "{{user_data}}" },
        },
        false,
        "24h"
      ),
    ],
    tags: ["enrichment", "pipeline"],
  };

  const pipelineScriptId = await client.saveScript(pipelineScript);
  console.log(
    `✓ Created enrichment pipeline: user_enrichment_pipeline (${pipelineScriptId})`
  );

  console.log("\nRunning pipeline:");
  const enrichResult = await client.callScript("user_enrichment_pipeline", {
    user_id: "1",
  });
  console.log("  ✓ Data fetched from API (cached 30m)");
  console.log("  ✓ Enriched data stored in 'enriched_users' (TTL 24h)");
  console.log(`  Pipeline returned ${enrichResult.records?.length || 0} records\n`);

  return pipelineScriptId;
}

async function exampleDynamicTTL(client) {
  console.log("\nExample 4: Dynamic TTL Configuration");
  console.log("─".repeat(80));
  console.log("TTL as parameter - supports duration strings, integers, ISO timestamps");

  // Create script with dynamic TTL
  const dynamicTTLScript = {
    label: "flexible_cache",
    name: "Flexible Cache TTL (Native SWR)",
    description: "Demonstrates parameterized TTL values",
    parameters: {
      resource_id: {
        type: "String",
        description: "Resource to fetch",
        required: true,
      },
      ttl: {
        type: "String",
        description: "Cache duration (e.g., '5m', '1h', '30s')",
        required: true,
      },
    },
    functions: [
      Stage.swr(
        "resource:{{resource_id}}",
        "{{ttl}}",
        "https://jsonplaceholder.typicode.com/posts/{{resource_id}}",
        "GET"
      ),
    ],
    tags: ["dynamic"],
  };

  const dynamicScriptId = await client.saveScript(dynamicTTLScript);
  console.log(`✓ Created dynamic TTL script: flexible_cache (${dynamicScriptId})`);

  // Test with different TTLs
  const ttlTests = [
    { value: "5m", description: "5 minutes" },
    { value: "1h", description: "1 hour" },
    { value: "30s", description: "30 seconds" },
  ];

  for (const test of ttlTests) {
    await client.callScript("flexible_cache", {
      resource_id: "test",
      ttl: test.value,
    });
    console.log(`  ✓ Cached with TTL: ${test.value} (${test.description})`);
  }

  return dynamicScriptId;
}

async function cleanup(client, scriptIds) {
  console.log("\n🧹 Cleaning up...");
  try {
    for (const scriptId of scriptIds) {
      await client.deleteScript(scriptId);
    }
    console.log(`✓ Deleted ${scriptIds.length} test scripts`);
  } catch (error) {
    console.log(`⚠ Cleanup error (non-critical): ${error}`);
  }
}

async function main() {
  console.log("🚀 ekoDB JavaScript Client - Native SWR Function Examples\n");
  console.log("📋 Demonstrates:");
  console.log("   • Single-function SWR pattern (replaces 4-step pipeline)");
  console.log("   • Automatic cache checking, HTTP fetching, and cache setting");
  console.log("   • Built-in audit trail support");
  console.log("   • Duration string TTLs ('15m', '1h', '30s')");
  console.log("   • Multi-function pipeline integration");
  console.log("   • Dynamic TTL configuration\n");

  const client = new EkoDBClient(BASE_URL, API_KEY);
  await client.init();

  const scriptIds = [];

  try {
    // Run examples
    scriptIds.push(await exampleBasicSWR(client));
    scriptIds.push(await exampleAuditTrail(client));
    scriptIds.push(await examplePipelineEnrichment(client));
    scriptIds.push(await exampleDynamicTTL(client));

    // Summary
    console.log("\n" + "=".repeat(80));
    console.log("✅ Key Benefits of Native SWR:");
    console.log("✅ Single function: Replaces 4-function cache-aside pattern");
    console.log("✅ Duration strings: Use '15m', '1h', '2h' instead of calculating seconds");
    console.log("✅ Built-in audit: Optional collection parameter for automatic logging");
    console.log("✅ Auto-enrichment: output_field populates params for downstream functions");
    console.log("✅ Transactional: Works correctly in both transactional and non-transactional contexts");
    console.log("✅ KV-optimized: Uses native KV store with proper TTL handling");

    console.log("\n=== Performance Comparison ===");
    console.log("Legacy Pattern: KvGet → If → HttpRequest → KvSet → Insert (5 functions)");
    console.log("Native SWR:     SWR → Insert (2 functions)");
    console.log("Result:         60% fewer functions, cleaner code, same behavior 🎯");
  } catch (error) {
    console.error("❌ Error:", error);
    throw error;
  } finally {
    await cleanup(client, scriptIds);
  }

  console.log("\n✅ All examples completed!");
}

main().catch(console.error);
