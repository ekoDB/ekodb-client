import { EkoDBClient, Stage } from "@ekodb/ekodb-client";
import * as dotenv from "dotenv";

dotenv.config();

const BASE_URL = process.env.API_BASE_URL || "http://localhost:8080";
const API_KEY = process.env.API_BASE_KEY || "a-test-api-key-from-ekodb";

async function edgeCacheExample() {
  const client = new EkoDBClient(BASE_URL, API_KEY);

  console.log("=== ekoDB as Edge Cache - Simple Example ===\n");

  console.log("Creating edge cache function...");

  // Simple passthrough cache pattern
  const cacheScript = {
    label: "cache_api_call",
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
        // KvGet returns {value: ...} on hit, {kv_value: null} on miss
        { type: "FieldExists", value: { field: "value" } },
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

  const scriptId = await client.saveScript(cacheScript);
  console.log(`✓ Edge cache script created: ${scriptId}\n`);

  // Test it - First call hits API
  console.log("Call 1: Cache miss (fetches from API)");
  const start1 = Date.now();
  const result1 = await client.callScript("cache_api_call", {
    cache_key: "user_data_1",
    api_url: "https://jsonplaceholder.typicode.com/users/1",
    ttl_seconds: 300,
  });
  const duration1 = Date.now() - start1;
  console.log(`Response time: ${duration1}ms`);
  console.log("Result:", JSON.stringify(result1, null, 2));

  // Test it again - Second call hits cache
  console.log("\nCall 2: Cache hit (served from ekoDB)");
  const start2 = Date.now();
  const result2 = await client.callScript("cache_api_call", {
    cache_key: "user_data_1",
    api_url: "https://jsonplaceholder.typicode.com/users/1",
  });
  const duration2 = Date.now() - start2;
  console.log(
    `Response time: ${duration2}ms (${Math.round((duration1 / duration2) * 10) / 10}x faster!)`,
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
