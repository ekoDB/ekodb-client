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

  const collection = "kv_links_example_ts";

  // Setup: create a KV entry and some documents to link
  console.log("--- Setup: creating KV key and documents ---");
  await client.kvSet("session:admin", { userId: "admin", role: "superuser" });
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
  const link1 = await client.kvLink("session:admin", collection, doc1Id);
  console.log(`Linked doc ${doc1Id}: ${JSON.stringify(link1)}`);

  const link2 = await client.kvLink("session:admin", collection, doc2Id);
  console.log(`Linked doc ${doc2Id}: ${JSON.stringify(link2)}`);

  // 2. Get all links for the KV key
  console.log("\n--- Getting links for KV key ---");
  const links = await client.kvGetLinks("session:admin");
  console.log("Links:", JSON.stringify(links, null, 2));

  // 3. Unlink one document
  console.log("\n--- Unlinking document ---");
  const unlinked = await client.kvUnlink("session:admin", collection, doc2Id);
  console.log(`Unlinked doc ${doc2Id}: ${JSON.stringify(unlinked)}`);

  // 4. Verify remaining links
  console.log("\n--- Verifying remaining links ---");
  const remainingLinks = await client.kvGetLinks("session:admin");
  console.log("Remaining links:", JSON.stringify(remainingLinks, null, 2));

  // Cleanup
  console.log("\n--- Cleanup ---");
  await client.kvUnlink("session:admin", collection, doc1Id);
  await client.kvDelete("session:admin");
  await client.delete(collection, doc1Id);
  await client.delete(collection, doc2Id);
  await client.deleteCollection(collection);
  console.log("Cleanup complete");

  console.log("\n=== All KV linking operations completed ===");
}

main().catch(console.error);
