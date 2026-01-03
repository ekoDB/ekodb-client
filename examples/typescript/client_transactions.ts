import { EkoDBClient } from "@ekodb/ekodb-client";
import * as dotenv from "dotenv";

dotenv.config();

const BASE_URL = process.env.API_BASE_URL || "http://localhost:8080";
const API_KEY = process.env.API_BASE_KEY || "a-test-api-key-from-ekodb";

/**
 * Transaction Example using ekoDB TypeScript Client
 *
 * Demonstrates:
 * - Begin transaction with isolation level
 * - Operations within a transaction
 * - Transaction status checking
 * - Commit transaction
 * - Rollback transaction
 */
async function main() {
  const client = new EkoDBClient(BASE_URL, API_KEY);
  await client.init();
  console.log("✓ Client created\n");

  // Setup test data
  console.log("=== Setup: Creating Test Accounts ===");
  const alice = await client.insert("test_accounts", {
    account_id: "ACC001",
    name: "Alice",
    balance: 1000,
  });
  const aliceId = alice.id as string;
  console.log(`Created Alice: $1000 - ID: ${aliceId}`);

  const bob = await client.insert("test_accounts", {
    account_id: "ACC002",
    name: "Bob",
    balance: 500,
  });
  const bobId = bob.id as string;
  console.log(`Created Bob: $500 - ID: ${bobId}\n`);

  // Example 1: Begin Transaction
  console.log("=== Example 1: Begin Transaction ===");
  const txId = await client.beginTransaction("ReadCommitted");
  console.log(`Transaction ID: ${txId}\n`);

  // Example 2: Operations with transaction_id (using query param)
  console.log("=== Example 2: Operations within Transaction ===");
  // Note: For transactional updates, use the direct API with ?transaction_id
  // The client methods don't yet support transaction_id parameter
  console.log("Updated Alice: $1000 → $800 (simulated)");
  console.log("Updated Bob: $500 → $700 (simulated)\n");

  // Example 3: Transaction Status
  console.log("=== Example 3: Transaction Status ===");
  const status = await client.getTransactionStatus(txId);
  console.log(`Status: ${status.state}`);
  console.log(`Operations: ${status.operations_count}\n`);

  // Example 4: Commit Transaction
  console.log("=== Example 4: Commit Transaction ===");
  await client.commitTransaction(txId);
  console.log("✓ Transaction committed\n");

  // Example 5: Rollback Demo
  console.log("=== Example 5: Rollback Demo ===");
  const txId2 = await client.beginTransaction("ReadCommitted");
  console.log(`New transaction: ${txId2}`);

  // Get status before rollback
  const status2 = await client.getTransactionStatus(txId2);
  console.log(`Status before rollback: ${status2.state}`);

  await client.rollbackTransaction(txId2);
  console.log("✓ Transaction rolled back\n");

  // Cleanup
  console.log("=== Cleanup ===");
  await client.delete("test_accounts", aliceId);
  await client.delete("test_accounts", bobId);
  console.log("✓ Deleted test accounts\n");

  console.log("✓ All client transaction examples completed");
}

main().catch(console.error);
