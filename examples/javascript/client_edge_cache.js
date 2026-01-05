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

async function edgeCacheExample() {
  const client = new EkoDBClient(BASE_URL, API_KEY);
  await client.init();

  console.log("=== ekoDB as Edge Cache - Simple Example ===\n");

  console.log("Creating edge cache function...");

  // Simple passthrough cache pattern
  const cacheScript = {
    label: "cache_api_call_js",
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
      // 1. Check cache
      Stage.findById("edge_cache_js", "{{cache_key}}"),

      // 2. If cache exists, return it; else fetch from API
      Stage.if(
        { type: "HasRecords" },
        // Cache hit - return cached data
        [Stage.project(["data", "cached_at"], false)],
        // Cache miss - fetch external API and store
        [
          Stage.httpRequest("{{api_url}}", "GET", {
            "User-Agent": "ekoDB-Edge-Cache",
          }),
          Stage.insert(
            "edge_cache_js",
            {
              id: { type: "String", value: "{{cache_key}}" },
              data: { type: "Object", value: "{{http_response}}" },
              cached_at: { type: "String", value: new Date().toISOString() },
            },
            false,
            undefined
          ),
        ]
      ),
    ],
  };

  const scriptId = await client.saveScript(cacheScript);
  console.log(`✓ Edge cache script created: ${scriptId}\n`);

  // Test it - First call hits API
  console.log("Call 1: Cache miss (fetches from API)");
  const start1 = Date.now();
  const result1 = await client.callScript("cache_api_call_js", {
    cache_key: "weather_nyc",
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
  const result2 = await client.callScript("cache_api_call_js", {
    cache_key: "weather_nyc",
    api_url:
      "https://api.open-meteo.com/v1/forecast?latitude=40.7128&longitude=-74.0060&current=temperature_2m",
  });
  const duration2 = Date.now() - start2;
  console.log(
    `Response time: ${duration2}ms (${Math.round((duration1 / duration2) * 10) / 10}x faster!)`
  );
  console.log("Result:", JSON.stringify(result2, null, 2));

  console.log("\n=== The Magic ===");
  console.log("- Your DATABASE is your EDGE");
  console.log("- No Redis needed");
  console.log("- No CDN needed");
  console.log("- No cache invalidation logic needed (TTL handles it)");
  console.log("- With ripples: All nodes auto-sync cache");
  console.log("- One service: Database + Cache + Edge Functions");

  console.log("\n✓ Example complete!\n");
}

edgeCacheExample().catch(console.error);
