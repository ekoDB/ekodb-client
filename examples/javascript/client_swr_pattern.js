/**
 * SWR (Stale-While-Revalidate) Pattern Example
 *
 * Demonstrates using ekoDB Functions as an SWR cache:
 * - Check cache first, return if exists
 * - On cache miss, fetch from external API
 * - Store result with TTL for auto-expiration
 */

const { EkoDBClient, Stage } = require("@ekodb/ekodb-client");
require("dotenv").config();

const BASE_URL = process.env.API_BASE_URL || "http://localhost:8080";
const API_KEY = process.env.API_BASE_KEY || "a-test-api-key-from-ekodb";

async function main() {
  const client = new EkoDBClient(BASE_URL, API_KEY);
  await client.init();

  console.log("=== ekoDB SWR (Stale-While-Revalidate) Pattern ===\n");

  console.log("Step 1: Create SWR function that acts as edge cache");

  // Using jsonplaceholder.typicode.com - a reliable free API for testing
  const swrScript = {
    label: "fetch_api_user_js",
    name: "Fetch User with Cache",
    description: "SWR pattern: Check cache, fetch from API if stale",
    parameters: {
      user_id: { required: true, description: "User ID to fetch" },
      ttl: {
        required: false,
        default: 300,
        description: "Cache TTL in seconds",
      },
    },
    version: "1.0",
    tags: ["swr", "user", "cache"],
    functions: [
      // Check KV cache for user data
      Stage.kvGet("api:user:{{user_id}}"),
      Stage.if(
        { type: "HasRecords" },
        // Cache hit - return cached data
        [Stage.project(["data"], false)],
        // Cache miss - fetch from API and cache
        [
          Stage.httpRequest(
            "https://jsonplaceholder.typicode.com/users/{{user_id}}",
            "GET",
            { "Accept": "application/json" }
          ),
          // Store in KV with 5 minute TTL
          Stage.kvSet(
            "api:user:{{user_id}}",
            "{{http_response}}",
            300,
          ),
        ]
      ),
    ],
  };

  const scriptId = await client.saveScript(swrScript);
  console.log(`✓ Created SWR script: ${swrScript.label} (${scriptId})\n`);

  console.log("Step 2: First call - Cache miss, fetches from API");
  const result1 = await client.callScript("fetch_api_user_js", {
    user_id: "1",
    ttl: 300,
  });
  console.log("Result:", JSON.stringify(result1, null, 2));
  console.log("✓ Data fetched from external API and cached\n");

  console.log("Step 3: Second call - Cache hit, instant response from ekoDB");
  const start = Date.now();
  await client.callScript("fetch_api_user_js", { user_id: "1" });
  const duration = Date.now() - start;
  console.log(`Response time: ${duration}ms (served from cache)`);
  console.log("✓ Lightning fast cache hit\n");

  // Cleanup
  console.log("🧹 Cleaning up...");
  try {
    await client.deleteScript(scriptId);
    await client.deleteCollection("user_cache_js");
  } catch (e) {
    // Ignore cleanup errors
  }
  console.log("✓ Cleanup complete\n");

  console.log("=== SWR Pattern Summary ===");
  console.log("✅ Cache miss → Fetch from API → Store in ekoDB");
  console.log("✅ Cache hit → Instant response from ekoDB");
  console.log("✅ TTL handles automatic cache invalidation");
}

main().catch(console.error);
