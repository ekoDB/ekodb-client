const { EkoDBClient } = require("@ekodb/ekodb-client");
const dotenv = require("dotenv");

dotenv.config();

const BASE_URL = process.env.API_BASE_URL || "http://localhost:8080";
const API_KEY = process.env.API_BASE_KEY || "a-test-api-key-from-ekodb";
const COLLECTION = "client_transactions_accounts_js";

function fieldValue(field) {
  return field && typeof field === "object" && "value" in field
    ? field.value
    : field;
}

/**
 * Transaction Example using ekoDB JavaScript Client
 *
 * Demonstrates:
 * - Begin transaction with the server default or an explicit isolation level
 * - Operations within a transaction
 * - Transaction status checking
 * - Commit transaction
 * - Rollback transaction
 */
async function main() {
  const client = new EkoDBClient(BASE_URL, API_KEY);
  await client.init();
  console.log("✓ Client created\n");
  const activeTransactions = [];
  let operationError;

  try {
    console.log("=== Setup: Creating Test Accounts ===");
    const alice = await client.insert(COLLECTION, {
      account_id: "ACC001",
      name: "Alice",
      balance: 1000,
    });
    const aliceId = fieldValue(alice.id);
    if (!aliceId) throw new Error("No ID returned for Alice");
    console.log(`Created Alice: $1000 - ID: ${aliceId}`);

    const bob = await client.insert(COLLECTION, {
      account_id: "ACC002",
      name: "Bob",
      balance: 500,
    });
    const bobId = fieldValue(bob.id);
    if (!bobId) throw new Error("No ID returned for Bob");
    console.log(`Created Bob: $500 - ID: ${bobId}\n`);

    // Example 1: Begin Transaction
    console.log("=== Example 1: Begin Transaction ===");
    const txId = await client.beginTransaction();
    activeTransactions.push(txId);
    console.log(`Transaction ID (server-default isolation): ${txId}\n`);

    // Example 2: Operations within Transaction
    console.log("=== Example 2: Operations within Transaction ===");
    await client.update(
      COLLECTION,
      aliceId,
      { balance: 800 },
      { transactionId: txId },
    );
    await client.update(
      COLLECTION,
      bobId,
      { balance: 700 },
      { transactionId: txId },
    );
    const aliceInTx = await client.findById(COLLECTION, aliceId, {
      transactionId: txId,
    });
    const bobInTx = await client.findById(COLLECTION, bobId, {
      transactionId: txId,
    });
    if (
      fieldValue(aliceInTx.balance) !== 800 ||
      fieldValue(bobInTx.balance) !== 700
    ) {
      throw new Error("Transactional reads did not return balances 800/700");
    }
    console.log("Updated Alice: $1000 → $800");
    console.log("Updated Bob: $500 → $700\n");

    // Example 3: Transaction Status
    console.log("=== Example 3: Transaction Status ===");
    const status = await client.getTransactionStatus(txId);
    console.log(`Status: ${status.state}`);
    console.log(`Operations: ${status.operations_count}\n`);
    if (status.state !== "Active" || status.operations_count !== 2) {
      throw new Error(
        `Unexpected transaction status: ${JSON.stringify(status)}`,
      );
    }

    // Example 4: Commit Transaction
    console.log("=== Example 4: Commit Transaction ===");
    await client.commitTransaction(txId);
    activeTransactions.splice(activeTransactions.indexOf(txId), 1);
    console.log("✓ Transaction committed\n");
    const committedAlice = await client.findById(COLLECTION, aliceId);
    const committedBob = await client.findById(COLLECTION, bobId);
    if (
      fieldValue(committedAlice.balance) !== 800 ||
      fieldValue(committedBob.balance) !== 700
    ) {
      throw new Error("Committed balances were not 800/700");
    }
    console.log("✓ Verified committed balances: Alice=$800, Bob=$700\n");

    // Example 5: Rollback Demo
    console.log("=== Example 5: Rollback Demo ===");
    const txId2 = await client.beginTransaction("ReadCommitted");
    activeTransactions.push(txId2);
    console.log(`New transaction: ${txId2}`);

    await client.update(
      COLLECTION,
      bobId,
      { balance: 600 },
      { transactionId: txId2 },
    );
    const bobInRollbackTx = await client.findById(COLLECTION, bobId, {
      transactionId: txId2,
    });
    if (fieldValue(bobInRollbackTx.balance) !== 600) {
      throw new Error(
        "Rollback transaction did not expose Bob's staged balance of 600",
      );
    }
    console.log("Updated Bob: $700 → $600 (in transaction)");

    const status2 = await client.getTransactionStatus(txId2);
    console.log(`Status before rollback: ${status2.state}`);
    if (status2.state !== "Active" || status2.operations_count !== 1) {
      throw new Error(
        `Unexpected rollback transaction status: ${JSON.stringify(status2)}`,
      );
    }

    await client.rollbackTransaction(txId2);
    activeTransactions.splice(activeTransactions.indexOf(txId2), 1);
    console.log("✓ Transaction rolled back\n");
    const bobAfterRollback = await client.findById(COLLECTION, bobId);
    if (fieldValue(bobAfterRollback.balance) !== 700) {
      throw new Error("Bob's rolled-back balance did not remain 700");
    }
    console.log("✓ Verified Bob remains $700 after rollback\n");
  } catch (error) {
    operationError = error;
  }

  console.log("=== Cleanup ===");
  const cleanupErrors = [];
  for (const transactionId of [...activeTransactions].reverse()) {
    try {
      await client.rollbackTransaction(transactionId);
    } catch (error) {
      cleanupErrors.push(error);
    }
  }
  try {
    await client.deleteCollection(COLLECTION);
    console.log("✓ Deleted test account collection\n");
  } catch (error) {
    cleanupErrors.push(error);
  }
  if (cleanupErrors.length > 0) {
    throw new AggregateError(
      operationError === undefined
        ? cleanupErrors
        : [operationError, ...cleanupErrors],
      "Transaction example cleanup failed",
    );
  }
  if (operationError !== undefined) throw operationError;

  console.log("✓ All client transaction examples completed");
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
