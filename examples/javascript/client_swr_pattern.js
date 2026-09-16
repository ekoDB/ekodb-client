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
      return existing.id;
    }
    throw error;
  }
}

async function main() {
  const client = new EkoDBClient(BASE_URL, API_KEY);
  await client.init();
  const runSuffix = `${process.pid}_${Date.now()}`;
  const functionLabel = `fetch_api_user_js_${runSuffix}`;
  const cacheKey = `swr:user_js:${runSuffix}:1`;
  let primaryError;

  try {
    console.log("=== ekoDB SWR (Stale-While-Revalidate) Pattern ===\n");

    console.log("Step 1: Create SWR function that acts as edge cache");

    // Using jsonplaceholder.typicode.com - a reliable free API for testing
    const swrScript = {
      label: functionLabel,
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
        Stage.kvGet(`swr:user_js:${runSuffix}:{{user_id}}`),
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
          // Cache miss - fetch from API and cache
          [
            Stage.httpRequest(
              "https://jsonplaceholder.typicode.com/users/{{user_id}}",
              "GET",
              { Accept: "application/json" },
            ),
            // Store in KV with 5 minute TTL
            Stage.kvSet(
              `swr:user_js:${runSuffix}:{{user_id}}`,
              "{{http_response}}",
              300,
            ),
            // Retrieve the cached data to return
            Stage.kvGet(`swr:user_js:${runSuffix}:{{user_id}}`),
            Stage.project(["value"], false),
          ],
        ),
      ],
    };

    const scriptId = await saveOrUpdate(client, swrScript);
    console.log(`✓ Created SWR script: ${swrScript.label} (${scriptId})\n`);

    console.log("Step 2: First call - Cache miss, fetches from API");
    const result1 = await client.callFunction(scriptId, {
      user_id: "1",
      ttl: 300,
    });
    console.log("Result:", JSON.stringify(result1, null, 2));
    console.log("✓ Data fetched from external API and cached\n");

    console.log("Step 3: Second call - Cache hit, instant response from ekoDB");
    const start = Date.now();
    await client.callFunction(scriptId, { user_id: "1" });
    const duration = Date.now() - start;
    console.log(`Response time: ${duration}ms (served from cache)`);
    console.log("✓ Lightning fast cache hit\n");

    console.log("=== SWR Pattern Summary ===");
    console.log("✅ Cache miss → Fetch from API → Store in ekoDB");
    console.log("✅ Cache hit → Instant response from ekoDB");
    console.log("✅ TTL handles automatic cache invalidation");
  } catch (error) {
    primaryError = error;
    throw error;
  } finally {
    console.log("🧹 Cleaning up...");
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
        "SWR example or cleanup failed",
      );
    }
    console.log("✓ Cleanup complete\n");
  }
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
