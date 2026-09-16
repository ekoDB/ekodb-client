// ekoDB TypeScript Client - Advanced CRUD Integration Example
//
// Exercises: updateWithAction (increment, push, clear),
// updateWithActionSequence, restoreRecord, restoreCollection.

import { EkoDBClient, getValue } from "@ekodb/ekodb-client";

async function main() {
  console.log("=== ekoDB Advanced CRUD Example (TypeScript) ===\n");

  const baseUrl = process.env.API_BASE_URL || "http://localhost:8080";
  const apiKey = process.env.API_BASE_KEY;
  if (!apiKey) throw new Error("API_BASE_KEY environment variable is required");

  const client = new EkoDBClient(baseUrl, apiKey);

  const collection = "advanced_crud_example_ts";

  // ========================================================================
  // updateWithAction: increment, push, clear
  // ========================================================================

  // Setup: insert a record to work with
  console.log("--- Inserting base record ---");
  const record = await client.insert(collection, {
    name: "Player One",
    score: 100,
    lives: 3,
    tags: ["beginner"],
    temp_data: "will be cleared",
  });
  const recordId = record.id as string;
  console.log(`Inserted: ${recordId}`);

  // 1. Increment a field
  console.log("\n--- updateWithAction: increment ---");
  const incremented = await client.updateWithAction(
    collection,
    recordId,
    "increment",
    "score",
    50,
  );
  const incrementedScore = getValue<number>(incremented.score);
  if (incrementedScore !== 150)
    throw new Error(`Unexpected score: ${incrementedScore}`);
  console.log(`Score after increment: ${incrementedScore}`);

  // 2. Push to an array field
  console.log("\n--- updateWithAction: push ---");
  const pushed = await client.updateWithAction(
    collection,
    recordId,
    "push",
    "tags",
    "pro",
  );
  console.log(`Tags after push: ${JSON.stringify(getValue(pushed.tags))}`);

  // 3. Clear a field
  console.log("\n--- updateWithAction: clear ---");
  const cleared = await client.updateWithAction(
    collection,
    recordId,
    "clear",
    "temp_data",
  );
  const clearedTempData = getValue(cleared.temp_data);
  if (clearedTempData != null) throw new Error("temp_data was not cleared");
  console.log(`temp_data after clear: ${clearedTempData}`);

  // ========================================================================
  // updateWithActionSequence: multiple atomic actions
  // ========================================================================

  console.log("\n--- updateWithActionSequence: multiple atomic actions ---");
  const sequenced = await client.updateWithActionSequence(
    collection,
    recordId,
    [
      ["increment", "score", 10],
      ["decrement", "lives", 1],
      ["push", "tags", "veteran"],
    ],
  );
  const sequenceScore = getValue<number>(sequenced.score);
  const sequenceLives = getValue<number>(sequenced.lives);
  const sequenceTags = getValue<string[]>(sequenced.tags);
  if (sequenceScore !== 160 || sequenceLives !== 2) {
    throw new Error(
      `Unexpected sequence result: score=${sequenceScore}, lives=${sequenceLives}`,
    );
  }
  console.log(
    `After sequence - score: ${sequenceScore}, lives: ${sequenceLives}, tags: ${JSON.stringify(sequenceTags)}`,
  );

  // ========================================================================
  // restoreRecord
  // ========================================================================

  /*
   * TODO(ekoDB dev team): Re-enable the record and collection restore examples
   * after the server can recover deleted record data. The live API currently
   * returns status="success", restored=false and removes the tombstone, leaving
   * the record unavailable afterward.
   *
   * // Insert and delete a record, then restore it
   * console.log("\n--- restoreRecord ---");
   * const toDelete = await client.insert(collection, {
   *   name: "Temporary Record",
   *   value: 42,
   * });
   * const deleteId = toDelete.id as string;
   * console.log(`Inserted record to delete: ${deleteId}`);
   *
   * await client.delete(collection, deleteId);
   * console.log(`Deleted record: ${deleteId}`);
   *
   * const restored = await client.restoreRecord(collection, deleteId);
   * console.log(`Restore result: ${restored}`);
   *
   * // Verify the record is back
   * const restoredRecord = await client.findById(collection, deleteId);
   * console.log(
   *   `Restored record: ${getValue(restoredRecord.name)} (value: ${getValue(restoredRecord.value)})`,
   * );
   *
   * // ======================================================================
   * // restoreCollection
   * // ======================================================================
   *
   * console.log("\n--- restoreCollection ---");
   * const extra1 = await client.insert(collection, { name: "Extra 1" });
   * const extra2 = await client.insert(collection, { name: "Extra 2" });
   * await client.delete(collection, extra1.id as string);
   * await client.delete(collection, extra2.id as string);
   * console.log("Deleted 2 extra records");
   *
   * const restoreResult = await client.restoreCollection(collection);
   * console.log(
   *   `restoreCollection result: ${restoreResult.recordsRestored} records restored`,
   * );
   */

  // ========================================================================
  // Cleanup
  // ========================================================================

  console.log("\n--- Cleanup ---");
  await client.deleteCollection(collection);
  console.log("Deleted collection");

  console.log("\n=== All advanced CRUD operations completed ===");
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
