// ekoDB TypeScript Client - KV Document Linking Integration Example
//
// Exercises: kvSet, kvLink, kvGetLinks, kvUnlink.
// KV links associate KV keys with documents in collections.

import { EkoDBClient } from "@ekodb/ekodb-client";

async function main() {
  console.log("=== ekoDB KV Document Linking Example (TypeScript) ===\n");

  const baseUrl = process.env.API_BASE_URL || "http://localhost:8080";
  const apiKey = process.env.API_BASE_KEY;
  if (!apiKey) throw new Error("API_BASE_KEY environment variable is required");

  const client = new EkoDBClient(baseUrl, apiKey);
  const suffix = `${process.pid}_${Date.now()}`;
  const collection = `kv_links_example_ts_${suffix}`;
  const key = `session:admin_ts:${suffix}`;
  let primaryError: unknown;

  try {
    // Setup: create a KV entry and some documents to link
    console.log("--- Setup: creating KV key and documents ---");
    await client.kvSet(key, { userId: "admin", role: "superuser" });
    console.log("Set KV key: session:admin");

    const doc1 = await client.insert(collection, {
      name: "Admin Profile",
      email: "admin@example.com",
    });
    const doc1Id = doc1.id as string;
    console.log(`Inserted document 1: ${doc1Id}`);

    const doc2 = await client.insert(collection, {
      name: "Admin Audit Log",
      action: "login",
      timestamp: new Date().toISOString(),
    });
    const doc2Id = doc2.id as string;
    console.log(`Inserted document 2: ${doc2Id}`);

    // 1. Link documents to the KV key
    console.log("\n--- Linking documents to KV key ---");
    const link1 = await client.kvLink(key, collection, doc1Id);
    console.log(`Linked doc ${doc1Id}: ${JSON.stringify(link1)}`);

    const link2 = await client.kvLink(key, collection, doc2Id);
    console.log(`Linked doc ${doc2Id}: ${JSON.stringify(link2)}`);

    // 2. Get all links for the KV key
    console.log("\n--- Getting links for KV key ---");
    const links = await client.kvGetLinks(key);
    console.log("Links:", JSON.stringify(links, null, 2));

    // 3. Unlink one document
    console.log("\n--- Unlinking document ---");
    const unlinked = await client.kvUnlink(key, collection, doc2Id);
    console.log(`Unlinked doc ${doc2Id}: ${JSON.stringify(unlinked)}`);

    // 4. Verify remaining links
    console.log("\n--- Verifying remaining links ---");
    const remainingLinks = await client.kvGetLinks(key);
    console.log("Remaining links:", JSON.stringify(remainingLinks, null, 2));

    console.log("\n=== All KV linking operations completed ===");
  } catch (error) {
    primaryError = error;
    throw error;
  } finally {
    console.log("\n--- Cleanup ---");
    const cleanupErrors: unknown[] = [];
    for (const cleanup of [
      () => client.kvDelete(key),
      () => client.deleteCollection(collection),
    ]) {
      try {
        await cleanup();
      } catch (error) {
        const message = error instanceof Error ? error.message : String(error);
        if (!message.includes("status 404") && !/not found/i.test(message)) {
          cleanupErrors.push(error);
        }
      }
    }
    if (cleanupErrors.length > 0) {
      throw new AggregateError(
        primaryError === undefined
          ? cleanupErrors
          : [primaryError, ...cleanupErrors],
        "KV linking example or cleanup failed",
      );
    }
    console.log("Cleanup complete");
  }
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
