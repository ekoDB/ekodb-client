#!/usr/bin/env node
/**
 * Transaction Examples - Direct HTTP API calls
 */

require("dotenv").config({ path: require("path").join(__dirname, "..", ".env") });

const BASE_URL = process.env.API_BASE_URL || "http://localhost:8080";
const API_KEY = process.env.API_BASE_KEY || "a-test-api-key-from-ekodb";

async function getAuthToken() {
  const response = await fetch(`${BASE_URL}/api/auth/token`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ api_key: API_KEY }),
  });
  const data = await response.json();
  return data.token;
}

async function makeRequest(method, path, token, data = null) {
  const options = {
    method,
    headers: {
      Authorization: `Bearer ${token}`,
      "Content-Type": "application/json",
    },
  };

  if (data) {
    options.body = JSON.stringify(data);
  }

  const response = await fetch(`${BASE_URL}${path}`, options);
  return response.json();
}

async function main() {
  const token = await getAuthToken();
  console.log("✓ Authentication successful\n");

  // Setup test data
  console.log("=== Setup: Creating Test Accounts ===");
  const alice = await makeRequest("POST", "/api/insert/test_accounts", token, {
    account_id: "ACC001",
    name: "Alice",
    balance: 1000,
  });
  const aliceId = alice.id;
  console.log(`Created Alice: $1000 - ID: ${aliceId}`);

  const bob = await makeRequest("POST", "/api/insert/test_accounts", token, {
    account_id: "ACC002",
    name: "Bob",
    balance: 500,
  });
  const bobId = bob.id;
  console.log(`Created Bob: $500 - ID: ${bobId}\n`);

  // Example 1: Begin Transaction
  console.log("=== Example 1: Begin Transaction ===");
  const txData = await makeRequest("POST", "/api/transactions", token, {
    isolation_level: "ReadCommitted",
  });
  const txId = txData.transaction_id;
  console.log(`Transaction ID: ${txId}\n`);

  // Example 2: Operations with transaction_id
  console.log("=== Example 2: Operations with transaction_id ===");
  await makeRequest(
    "PUT",
    `/api/batch/update/test_accounts?transaction_id=${encodeURIComponent(
      txId
    )}`,
    token,
    {
      updates: [
        { id: aliceId, data: { balance: 800 } },
        { id: bobId, data: { balance: 700 } },
      ],
    }
  );
  console.log("Updated Alice: $1000 → $800");
  console.log("Updated Bob: $500 → $700\n");

  // Example 3: Transaction Status
  console.log("=== Example 3: Transaction Status ===");
  const status = await makeRequest("GET", `/api/transactions/${txId}`, token);
  console.log(`Status: ${status.state}`);
  console.log(`Operations: ${status.operations_count}\n`);

  // Example 4: Commit Transaction
  console.log("=== Example 4: Commit Transaction ===");
  await makeRequest("POST", `/api/transactions/${txId}/commit`, token);
  console.log("✓ Transaction committed\n");

  // Verification
  console.log("=== Verification ===");
  const aliceFinal = await makeRequest(
    "GET",
    `/api/find/test_accounts/${aliceId}`,
    token
  );
  console.log(`Alice: ${JSON.stringify(aliceFinal.balance)}`);

  const bobFinal = await makeRequest(
    "GET",
    `/api/find/test_accounts/${bobId}`,
    token
  );
  console.log(`Bob: ${JSON.stringify(bobFinal.balance)}\n`);

  // Example 5: Rollback
  console.log("=== Example 5: Rollback ===");
  const txData2 = await makeRequest("POST", "/api/transactions", token, {
    isolation_level: "ReadCommitted",
  });
  const txId2 = txData2.transaction_id;
  console.log(`New transaction: ${txId2}`);

  await makeRequest(
    "PUT",
    `/api/batch/update/test_accounts?transaction_id=${encodeURIComponent(
      txId2
    )}`,
    token,
    {
      updates: [{ id: bobId, data: { balance: 600 } }],
    }
  );
  console.log("Updated Bob: $700 → $600 (in transaction)");

  await makeRequest("POST", `/api/transactions/${txId2}/rollback`, token);
  console.log("✓ Transaction rolled back");

  const bobAfter = await makeRequest(
    "GET",
    `/api/find/test_accounts/${bobId}`,
    token
  );
  console.log(`Bob after rollback: ${JSON.stringify(bobAfter.balance)}\n`);

  // Cleanup
  console.log("=== Cleanup ===");
  await makeRequest("DELETE", `/api/delete/test_accounts/${aliceId}`, token);
  await makeRequest("DELETE", `/api/delete/test_accounts/${bobId}`, token);
  console.log("✓ Deleted test accounts\n");

  console.log("✓ All transaction examples completed");
}

main().catch(console.error);
