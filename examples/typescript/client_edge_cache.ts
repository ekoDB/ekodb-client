import { EkoDBClient, Stage } from "@ekodb/ekodb-client";
import * as dotenv from "dotenv";

dotenv.config();

const BASE_URL = process.env.API_BASE_URL || "http://localhost:8080";
const API_KEY = process.env.API_BASE_KEY || "a-test-api-key-from-ekodb";

function isNotFoundError(error: unknown): boolean {
  const message = error instanceof Error ? error.message : String(error);
  return message.includes("status 404") || /not found/i.test(message);
}

async function edgeCacheExample() {
  const client = new EkoDBClient(BASE_URL, API_KEY);
  const runSuffix = `${process.pid}_${Date.now()}`;
  const functionLabel = `cache_api_call_ts_${runSuffix}`;
  const cacheKey = `edge_cache_ts_${runSuffix}`;
  let primaryError: unknown;

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

    let scriptId: string;
    try {
      scriptId = await client.saveFunction(cacheScript);
      console.log(`✓ Edge cache script created: ${scriptId}\n`);
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      if (
        !message.includes("status 409") &&
        !message.includes("already exists")
      ) {
        throw error;
      }
      // Function already exists from a previous run — update it in place so the
      // example is idempotent and proves both the create and update paths.
      await client.updateFunction(cacheScript.label, cacheScript);
      const existing = await client.getFunction(cacheScript.label);
      scriptId = existing.id ?? cacheScript.label;
      console.log(
        `ℹ️  Function '${cacheScript.label}' already existed — updated instead`,
      );
      console.log(`✓ Edge cache script updated: ${scriptId}\n`);
    }

    // Test it - First call hits API
    console.log("Call 1: Cache miss (fetches from API)");
    const start1 = Date.now();
    const result1 = await client.callFunction(scriptId, {
      cache_key: cacheKey,
      api_url: "https://jsonplaceholder.typicode.com/users/1",
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
      api_url: "https://jsonplaceholder.typicode.com/users/1",
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
    const cleanupErrors: unknown[] = [];
    try {
      const saved = await client.getFunction(functionLabel);
      await client.deleteFunction(saved.id ?? functionLabel);
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
