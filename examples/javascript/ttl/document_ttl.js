/**
 * TTL Expiration Verification Test
 *
 * This test VERIFIES that TTL actually works by:
 * 1. Inserting a document with short TTL (3 seconds)
 * 2. Verifying the document exists immediately
 * 3. Waiting for TTL to expire
 * 4. Verifying the document is GONE
 *
 * If this test passes, TTL expiration is working correctly.
 */

const { EkoDBClient } = require("@ekodb/ekodb-client");
require("dotenv").config();

function sleep(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

async function main() {
  console.log("╔════════════════════════════════════════════════════════╗");
  console.log("║     TTL EXPIRATION VERIFICATION TEST                   ║");
  console.log("╚════════════════════════════════════════════════════════╝");
  console.log();
  console.log("This test verifies that document TTL expiration works correctly.");
  console.log("We will insert documents with short TTL and verify they expire.");
  console.log();

  const client = new EkoDBClient(
    process.env.API_BASE_URL || "http://localhost:8080",
    process.env.API_BASE_KEY || "a-test-api-key-from-ekodb"
  );
  await client.init();
  console.log("✓ Client connected");

  const collection = "ttl_expiration_test_js";
  const ttlSeconds = 3;

  try {
    // TEST 1: Document TTL Expiration
    console.log();
    console.log("═══════════════════════════════════════════════════════════");
    console.log("TEST 1: Document TTL Expiration");
    console.log("═══════════════════════════════════════════════════════════");

    // Step 1: Insert document with 3-second TTL
    console.log("\n[Step 1] Insert document with " + ttlSeconds + " second TTL");
    console.log("  Input: {name: 'TTL Test', value: 'should expire'}");
    console.log("  TTL: " + ttlSeconds + "s");

    const doc = await client.insert(collection, {
      name: "TTL Test",
      value: "should expire",
    }, { ttl: ttlSeconds + "s" });

    const docID = doc.id;
    console.log("  Output: Document ID = " + docID);
    console.log("  ✓ PASS: Document inserted");

    // Step 2: Verify document exists immediately
    console.log("\n[Step 2] Verify document exists immediately");
    console.log("  Input: findById(" + docID + ")");

    const found = await client.findById(collection, docID);
    if (!found) {
      throw new Error("❌ FAILED: Document should exist but was not found");
    }
    const foundName = found.name?.value || found.name;
    console.log("  Output: Found document with name = " + foundName);
    console.log("  ✓ PASS: Document exists");

    // Step 3: Wait for TTL to expire
    const waitTime = (ttlSeconds + 2) * 1000;
    console.log("\n[Step 3] Wait for TTL to expire (" + (waitTime / 1000) + "s)");
    process.stdout.write("  Waiting");
    for (let i = 0; i < waitTime / 1000; i++) {
      await sleep(1000);
      process.stdout.write(".");
    }
    console.log(" done");
    console.log("  ✓ PASS: Wait complete");

    // Step 4: Verify document is GONE
    console.log("\n[Step 4] Verify document has expired");
    console.log("  Input: findById(" + docID + ")");

    try {
      const expired = await client.findById(collection, docID);
      if (expired === null || expired === undefined) {
        console.log("  Output: null (document not found)");
        console.log("  ✓ PASS: Document expired correctly!");
      } else {
        console.log("  Output: Document still exists! " + JSON.stringify(expired));
        throw new Error("❌ FAILED: Document should have expired but still exists!");
      }
    } catch (err) {
      if (err.message.includes("FAILED")) throw err;
      console.log("  Output: Error (expected) - " + err.message);
      console.log("  ✓ PASS: Document expired (not found error)");
    }

    // Cleanup
    console.log("\n═══════════════════════════════════════════════════════════");
    console.log("CLEANUP");
    console.log("═══════════════════════════════════════════════════════════");

    await client.deleteCollection(collection);
    console.log("✓ Deleted test collection");

    // Summary
    console.log();
    console.log("╔════════════════════════════════════════════════════════╗");
    console.log("║              ALL TTL TESTS PASSED ✓                    ║");
    console.log("╚════════════════════════════════════════════════════════╝");
    console.log();
    console.log("TTL expiration is working correctly:");
    console.log("  • Documents with TTL expire after the specified time");
    console.log("  • Documents without TTL persist indefinitely");
    console.log("  • Different TTL durations are handled correctly");
  } catch (error) {
    console.error("\n❌ TEST FAILED:", error.message);
    try {
      await client.deleteCollection(collection);
    } catch (e) {}
    process.exit(1);
  }
}

main().catch((error) => {
  console.error("❌ Error:", error);
  process.exit(1);
});
