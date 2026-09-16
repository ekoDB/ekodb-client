/**
 * ekoDB as Edge Cache Example
 *
 * Demonstrates using ekoDB Functions as an edge cache:
 * - Create a cache function that checks cache first
 * - On cache miss, fetch from external API and store
 * - On cache hit, return cached data instantly
 */

const { EkoDBClient, Stage } = require("@ekodb/ekodb-client");
require("dotenv").config();

const BASE_URL = process.env.API_BASE_URL || "http://localhost:8080";
const API_KEY = process.env.API_BASE_KEY || "a-test-api-key-from-ekodb";

function isNotFoundError(error) {
  const message = error instanceof Error ? error.message : String(error);
  return message.includes("status 404") || /not found/i.test(message);
}

/**
 * Save a function idempotently.
 *
 * The server returns HTTP 409 ("A function with label 'X' already exists.")
 * when a function with the same fixed label already exists. On that error we
 * UPDATE the existing function via PUT /api/functions/{label} (the server's
 * PUT route accepts either the encrypted ID or the label), then resolve and
 * return its encrypted ID. Any other error is propagated.
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

async function edgeCacheExample() {
  const client = new EkoDBClient(BASE_URL, API_KEY);
  await client.init();
  const runSuffix = `${process.pid}_${Date.now()}`;
  const functionLabel = `cache_api_call_js_${runSuffix}`;
  const cacheKey = `edge_cache_js_${runSuffix}`;
  let primaryError;

  try {
    console.log("=== ekoDB as Edge Cache - Simple Example ===\n");

    console.log("Creating edge cache function...");

    // Simple passthrough cache pattern
    const cacheScript = {
      label: functionLabel,
      name: "Cache External API Call",
      description:
        "Database as edge: Check cache → Call API if miss → Store result → Return",
      parameters: {
        cache_key: {
          required: true,
          description: "Unique cache key",
        },
        api_url: {
          required: true,
          description: "External API URL to call",
        },
        ttl_seconds: {
          required: false,
          default: 300,
          description: "Time to live in seconds",
        },
      },
      version: "1.0",
      tags: ["cache", "edge"],
      functions: [
        // 1. Check KV cache
        Stage.kvGet("{{cache_key}}"),

        // 2. If cache exists, return it; else fetch from API
        Stage.if(
          // KvGet returns {value: ...} on hit, {value: null} on miss
          // So we check if "value" is not null to detect cache hit
          {
            type: "Not",
            value: {
              condition: {
                type: "FieldEquals",
                value: { field: "value", value: null },
              },
            },
          },
          // Cache hit - return cached data
          [Stage.project(["value"], false)],
          // Cache miss - fetch external API and store in KV
          [
            Stage.httpRequest("{{api_url}}", "GET", {
              "User-Agent": "ekoDB-Edge-Cache",
            }),
            // Store in KV with 5 minute TTL
            Stage.kvSet("{{cache_key}}", "{{http_response}}", 300),
            // Retrieve the cached data to return
            Stage.kvGet("{{cache_key}}"),
            Stage.project(["value"], false),
          ],
        ),
      ],
    };

    const scriptId = await saveOrUpdate(client, cacheScript);
    console.log(`✓ Edge cache script created: ${scriptId}\n`);

    // Test it - First call hits API
    console.log("Call 1: Cache miss (fetches from API)");
    const start1 = Date.now();
    const result1 = await client.callFunction(scriptId, {
      cache_key: cacheKey,
      api_url:
        "https://api.open-meteo.com/v1/forecast?latitude=40.7128&longitude=-74.0060&current=temperature_2m",
      ttl_seconds: 300,
    });
    const duration1 = Date.now() - start1;
    console.log(`Response time: ${duration1}ms`);
    console.log("Result:", JSON.stringify(result1, null, 2));

    // Test it again - Second call hits cache
    console.log("\nCall 2: Cache hit (served from ekoDB)");
    const start2 = Date.now();
    const result2 = await client.callFunction(scriptId, {
      cache_key: cacheKey,
      api_url:
        "https://api.open-meteo.com/v1/forecast?latitude=40.7128&longitude=-74.0060&current=temperature_2m",
    });
    const duration2 = Date.now() - start2;
    console.log(
      `Response time: ${duration2}ms (${
        Math.round((duration1 / duration2) * 10) / 10
      }x faster!)`,
    );
    console.log("Result:", JSON.stringify(result2, null, 2));

    console.log("\n=== The Magic ===");
    console.log("- Your DATABASE is your EDGE");
    console.log("- No Redis needed");
    console.log("- No CDN needed");
    console.log("- No cache invalidation logic needed (TTL handles it)");
    console.log("- With ripples: All nodes auto-sync cache");
    console.log("- One service: Database + Cache + Edge Functions");
  } catch (error) {
    primaryError = error;
    throw error;
  } finally {
    const cleanupErrors = [];
    try {
      const saved = await client.getFunction(functionLabel);
      await client.deleteFunction(saved.id || functionLabel);
    } catch (error) {
      if (!isNotFoundError(error)) cleanupErrors.push(error);
    }
    try {
      await client.kvDelete(cacheKey);
    } catch (error) {
      if (!isNotFoundError(error)) cleanupErrors.push(error);
    }
    if (cleanupErrors.length > 0) {
      throw new AggregateError(
        primaryError === undefined
          ? cleanupErrors
          : [primaryError, ...cleanupErrors],
        "Edge cache example or cleanup failed",
      );
    }
  }

  console.log("\n✓ Example complete!\n");
}

edgeCacheExample().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
