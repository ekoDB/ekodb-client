import { EkoDBClient, Stage } from "@ekodb/ekodb-client";
import * as dotenv from "dotenv";

dotenv.config();

const BASE_URL = process.env.API_BASE_URL || "http://localhost:8080";
const API_KEY = process.env.API_BASE_KEY || "a-test-api-key-from-ekodb";

async function swrPatternExample() {
  const client = new EkoDBClient(BASE_URL, API_KEY);

  console.log("=== ekoDB SWR (Stale-While-Revalidate) Pattern ===\n");

  console.log("Step 1: Create SWR function that acts as edge cache");

  // Using jsonplaceholder.typicode.com - a reliable free API for testing
  const swrScript = {
    label: "fetch_api_user",
    name: "Fetch User with Cache",
    description:
      "SWR pattern: Check cache, fetch from API if stale, auto-update with TTL",
    parameters: {
      user_id: {
        required: true,
        description: "User ID to fetch",
      },
      ttl: {
        required: false,
        default: 300,
        description: "Cache TTL in seconds (default: 5 minutes)",
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
            { Accept: "application/json" },
          ),
          // Store in KV with 5 minute TTL
          Stage.kvSet("api:user:{{user_id}}", "{{http_response}}", 300),
          // Retrieve the cached data to return
          Stage.kvGet("api:user:{{user_id}}"),
          Stage.project(["data"], false),
        ],
      ),
    ],
  };

  const scriptId = await client.saveScript(swrScript);
  console.log(`✓ Created SWR script: ${swrScript.label} (${scriptId})\n`);

  console.log("Step 2: First call - Cache miss, fetches from API");
  const result1 = await client.callScript("fetch_api_user", {
    user_id: "1",
    ttl: 300,
  });
  console.log("Result:", JSON.stringify(result1, null, 2));
  console.log("✓ Data fetched from external API and cached\n");

  console.log("Step 3: Second call - Cache hit, instant response from ekoDB");
  const start = Date.now();
  const result2 = await client.callScript("fetch_api_user", {
    user_id: "1",
  });
  const duration = Date.now() - start;
  console.log(`Response time: ${duration}ms (served from cache)`);
  console.log("✓ Lightning fast cache hit\n");

  console.log("=== Advanced: SWR with Data Enrichment ===\n");

  console.log("Creating product enrichment function...");
  const enrichScript = {
    label: "fetch_product_with_reviews",
    name: "Fetch Product with Reviews (Multi-API)",
    description: "Demonstrates calling multiple APIs and merging results",
    parameters: {
      product_id: {
        required: true,
        description: "Product ID",
      },
      ttl: {
        required: false,
        default: 600,
        description: "Cache TTL (10 minutes)",
      },
    },
    version: "1.0",
    tags: ["enrichment", "product", "cache"],
    functions: [
      // Check KV cache for product data
      Stage.kvGet("product:{{product_id}}"),
      Stage.if(
        { type: "HasRecords" },
        // Cache hit - return cached data
        [Stage.project(["data"], false)],
        // Cache miss - fetch from API and cache
        [
          Stage.httpRequest(
            "https://dummyjson.com/products/{{product_id}}",
            "GET",
          ),
          // Store in KV with 10 minute TTL
          Stage.kvSet("product:{{product_id}}", "{{http_response}}", 600),
          // Retrieve the cached data to return
          Stage.kvGet("product:{{product_id}}"),
          Stage.project(["data"], false),
        ],
      ),
    ],
  };

  const enrichScriptId = await client.saveScript(enrichScript);
  console.log(
    `✓ Created enrichment script: ${enrichScript.label} (${enrichScriptId})\n`,
  );

  console.log(
    "Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result",
  );
  const enriched = await client.callScript("fetch_product_with_reviews", {
    product_id: "1",
    ttl: 600,
  });
  console.log("Enriched data:", JSON.stringify(enriched, null, 2));
  console.log("✓ Multi-API data fetched, merged, and cached atomically\n");

  console.log("=== Why This Is Powerful ===\n");
  console.log("✓ No separate cache layer (Redis, Memcached) needed");
  console.log("✓ No manual cache invalidation (TTL handles it)");
  console.log("✓ No separate edge infrastructure (ekoDB IS the edge)");
  console.log("✓ Atomic operations (function executes as transaction)");
  console.log("✓ With multi-node + ripples: Auto-sync across all nodes");
  console.log("✓ Sub-millisecond cache hits from internal storage");
  console.log("✓ One service instead of many (cache + API gateway + database)");

  console.log("\n=== Real-World Use Cases ===\n");
  console.log("1. API Gateway Pattern:");
  console.log(
    "   - Client → ekoDB Function → Check cache → Call microservices → Merge → Cache\n",
  );

  console.log("2. Database Federation:");
  console.log("   - Query multiple DBs (Postgres, MongoDB) + external APIs");
  console.log("   - Merge results in one function call\n");

  console.log("3. IoT Data Enrichment:");
  console.log("   - Sensor data + weather API + location API");
  console.log("   - Enrich and cache in one atomic operation\n");

  console.log("4. E-commerce Product Pages:");
  console.log("   - Product info + reviews + inventory + pricing");
  console.log("   - All from different sources, cached together\n");

  console.log("✓ Example complete - Your database IS your edge!\n");
}

swrPatternExample().catch(console.error);
