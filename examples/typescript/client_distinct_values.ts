/**
 * Distinct Values Example
 *
 * Demonstrates how to retrieve all unique values for a field in a collection,
 * with and without filters.
 *
 * Run with: npx ts-node client_distinct_values.ts
 */

import * as dotenv from "dotenv";
import * as path from "path";
import { EkoDBClient } from "@ekodb/ekodb-client";

dotenv.config({ path: path.resolve(__dirname, "../.env") });

async function main() {
  const baseURL = process.env.API_BASE_URL ?? "http://localhost:8080";
  const apiKey = process.env.API_BASE_KEY ?? "a-test-api-key-from-ekodb";

  const client = new EkoDBClient({ baseURL, apiKey });

  console.log("=== Distinct Values Example ===\n");

  const collection = "distinct_values_example_ts";

  // Cleanup from previous runs
  try {
    await client.deleteCollection(collection);
  } catch {}

  // ---------------------------------------------------------------------------
  // Step 1: Insert sample data
  // ---------------------------------------------------------------------------
  console.log("Inserting sample products...");

  const products = [
    { name: "Widget A", category: "electronics", status: "active" },
    { name: "Widget B", category: "electronics", status: "active" },
    { name: "Gadget X", category: "electronics", status: "discontinued" },
    { name: "Book 1", category: "books", status: "active" },
    { name: "Book 2", category: "books", status: "active" },
    { name: "Book 3", category: "books", status: "archived" },
    { name: "Shirt M", category: "clothing", status: "active" },
    { name: "Shirt L", category: "clothing", status: "active" },
  ];

  for (const product of products) {
    await client.insert(collection, product);
  }
  console.log(`Inserted ${products.length} products\n`);

  // ---------------------------------------------------------------------------
  // Step 2: All distinct categories (no filter)
  // ---------------------------------------------------------------------------
  console.log("=== Distinct Categories (all products) ===");
  const categories = await client.distinctValues(collection, "category");
  console.log(`Found ${categories.count} distinct categories:`);
  for (const v of categories.values) {
    console.log(`  - ${v}`);
  }
  console.log();

  // ---------------------------------------------------------------------------
  // Step 3: All distinct statuses (no filter)
  // ---------------------------------------------------------------------------
  console.log("=== Distinct Statuses (all products) ===");
  const statuses = await client.distinctValues(collection, "status");
  console.log(`Found ${statuses.count} distinct statuses:`);
  for (const v of statuses.values) {
    console.log(`  - ${v}`);
  }
  console.log();

  // ---------------------------------------------------------------------------
  // Step 4: Distinct statuses within electronics only
  // ---------------------------------------------------------------------------
  console.log("=== Distinct Statuses in Electronics ===");
  const electronicsStatuses = await client.distinctValues(
    collection,
    "status",
    {
      filter: {
        type: "Condition",
        content: { field: "category", operator: "Eq", value: "electronics" },
      },
    },
  );
  console.log(
    `Found ${electronicsStatuses.count} distinct statuses for electronics:`,
  );
  for (const v of electronicsStatuses.values) {
    console.log(`  - ${v}`);
  }
  console.log();

  // ---------------------------------------------------------------------------
  // Cleanup
  // ---------------------------------------------------------------------------
  await client.deleteCollection(collection);
  console.log("Cleanup done.");
}

main().catch(console.error);
