make test-examples-javascript
🧪 [36mRunning JavaScript examples (direct HTTP/WebSocket)...[0m

added 1 package, removed 1 package, and audited 9 packages in 851ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
[34m
╔════════════════════════════════════════╗[0m
[34m║  ekoDB JavaScript Examples Test Suite ║[0m
[34m╚════════════════════════════════════════╝[0m
[34m
=== Checking Server Connection ===[0m
(node:71112) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
[32m✓ Server is ready[0m
[34m
=== Getting Authentication Token ===[0m
[32m✓ Authentication successful[0m
[33m
=== Running 10 Examples ===[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/simple_crud.js ===[0m
=== Simple CRUD Operations (Direct HTTP) ===

✓ Authentication successful

=== Insert Document ===
Inserted: {
  id: 'hbvG-g7y4P7IlJimbtfDN0-d3VSHuU-YRaL8YdE-I1aDlB6SfSfnaeeBjaTnSCW2D2GoxA7eJeMu1ksGSKJEBA'
}

=== Find by ID ===
Found: {
  name: { type: 'String', value: 'Test Record' },
  active: { value: true, type: 'Boolean' },
  id: 'hbvG-g7y4P7IlJimbtfDN0-d3VSHuU-YRaL8YdE-I1aDlB6SfSfnaeeBjaTnSCW2D2GoxA7eJeMu1ksGSKJEBA',
  value: { value: 42, type: 'Integer' }
}

=== Find with Query ===
Found documents: [
  {
    active: { value: true, type: 'Boolean' },
    value: { value: 42, type: 'Integer' },
    id: 'hbvG-g7y4P7IlJimbtfDN0-d3VSHuU-YRaL8YdE-I1aDlB6SfSfnaeeBjaTnSCW2D2GoxA7eJeMu1ksGSKJEBA',
    name: { type: 'String', value: 'Test Record' }
  }
]

=== Update Document ===
Updated: {
  value: { value: 100, type: 'Integer' },
  name: { type: 'String', value: 'Updated Record' },
  active: { value: true, type: 'Boolean' },
  id: 'hbvG-g7y4P7IlJimbtfDN0-d3VSHuU-YRaL8YdE-I1aDlB6SfSfnaeeBjaTnSCW2D2GoxA7eJeMu1ksGSKJEBA'
}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
[32m✓ simple_crud.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/simple_websocket.js ===[0m
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: _0Q3Oudgg4ouoqLCGbtJ7sEpLv6v9qtiZuhhettIAPlJl-X7iHBn3HEVGUzTbb5aZE9PGIWS6mOdCIrTTxJoDg

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "payload": {
    "data": [
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "d2VJG5iv4Y2GM8EF5v5Bvs8hZucupbWnljtU8V09E6kHsVSL76g8IaS5XdJ9hRAvPBx73kHkr6R-756nGUvxHg",
        "name": {
          "type": "String",
          "value": "WebSocket Test Record"
        },
        "value": {
          "type": "Integer",
          "value": 42
        }
      },
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "_0Q3Oudgg4ouoqLCGbtJ7sEpLv6v9qtiZuhhettIAPlJl-X7iHBn3HEVGUzTbb5aZE9PGIWS6mOdCIrTTxJoDg",
        "name": {
          "type": "String",
          "value": "WebSocket Test Record"
        },
        "value": {
          "type": "Integer",
          "value": 42
        }
      },
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "US1URz1oElP11pCefhxXASu8-cl5XivMknbWJiWq1kXVtN-5hRHQyja-29vCYjgUA0cRpJ_hJZcKdqayTZhPww",
        "name": {
          "type": "String",
          "value": "WebSocket Test Record"
        },
        "value": {
          "type": "Integer",
          "value": 42
        }
      },
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "ggUkVE8p-4-sgFujbNnfoo55fNFXQjXjuC4CFyYjUOz6nF6tI0-pZ_LBzcVVwNTxaLqn3-UX4dKYH11ySN0p4A",
        "name": {
          "type": "String",
          "value": "WebSocket Test Record"
        },
        "value": {
          "type": "Integer",
          "value": 42
        }
      }
    ]
  },
  "type": "Success"
}
✓ Retrieved 4 record(s) via WebSocket

✓ WebSocket example completed successfully
WebSocket closed
[32m✓ simple_websocket.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/http_functions.js ===[0m
🚀 ekoDB Scripts Example (JavaScript/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: MD0Tbf7C-tk_6PygovHIgEjpPDNm9226GEdF9iuHOMtkTLKbTYzZWYoGtoKmbK_7gnWo8kZv2-S7kfwgvDLY3A
📊 Found 20 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: Zr1bxN4XenwFs8FiZzZ_ZPiJToV_bHbYq6YczNkrpG3jP_q03OgN76Civmt1i-BXjscex6mS2fzXBxF1fmc1Gw
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: MzfwedvICb2bVXQ-Yy_iE1s0-uYrcSQYI3xlLcJ8LUQnQK1ST9u_z6QWzKHxDoQxc2pWRKdw3O1QNvgHtHUjEA
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"count":10,"avg_score":50,"status":"inactive","max_score":90}
   {"avg_score":60,"status":"active","count":10,"max_score":100}

📝 Example 4: Function Management

📋 Total scripts: 20
🔍 Retrieved script: Get Active Users
✏️  Script updated
🗑️  Script deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

✅ All examples completed!
[32m✓ http_functions.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/batch_operations.js ===[0m
=== Batch Operations (Direct HTTP) ===

✓ Authentication successful

=== Batch Insert ===
✓ Batch inserted 5 records
✓ Verified: Found 20 total records in collection

=== Creating test records for update/delete ===
Created 3 test records

=== Batch Update ===
✓ Batch updated 3 records
✓ Verified: Record updated with status="active"

=== Batch Delete ===
✓ Batch deleted 3 records
✓ Verified: Records successfully deleted (not found)

✓ All batch operations completed successfully
[32m✓ batch_operations.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/kv_operations.js ===[0m
=== Key-Value Operations (Direct HTTP) ===

✓ Authentication successful

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: { userId: 123, username: 'john_doe' }

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: { price: 29.99, name: 'Product 1' }
cache:product:2: { price: 39.989999999999995, name: 'Product 2' }
cache:product:3: { name: 'Product 3', price: 49.989999999999995 }

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
[32m✓ kv_operations.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/collection_management.js ===[0m
=== Collection Management (Direct HTTP) ===

✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: Zlfx3I0NGEQl4HDUy3kh_cNlNPFaNgvbuSFOZYaHNqWRZePIR2yTzvkA7ibwvXV_9qPQvJmU1EspQduVSdYFFw

=== List Collections ===
Total collections: 23
Sample collections: [
  'schema_documents_client_ts',
  'ttl_cache',
  'schema_users_client_ts',
  'users',
  'websocket_test'
]

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
[32m✓ collection_management.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/transactions.js ===[0m
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: wLYPadg5L7aP_bBYvLDXhmG0OA_cNO4gy2_5ocXz-W8-Po7TMstdrix1eRJKznkyRBLeNqTHqHl9iGRpOtnJDQ
Created Bob: $500 - ID: BdZi4K4Wej7FyuqyTq0A1Xxw8xP99iysOawPBuCe1XqSonsRx17rLTeGxE_eyF3iEfr6Y51Ty6JygOM_5QaOKg

=== Example 1: Begin Transaction ===
Transaction ID: f87d4f98-edae-4f03-8cec-b9c7b9c942c6

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: 800
Bob: 700

=== Example 5: Rollback ===
New transaction: 8a242188-f552-4a35-a5c2-8df155d36277
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: 700

=== Cleanup ===
✓ Deleted test accounts

✓ All transaction examples completed
[32m✓ transactions.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/crud_scripts.js ===[0m
🚀 ekoDB Complete CRUD Scripts Example
============================================================
Demonstrates:
  • Insert + Verify (using Query)
  • Query + Update Status + Verify
  • Query + Update Credits + Verify
  • Query Before Delete + Delete + Verify Gone

Each Script shows Functions chaining with proper verification
============================================================

============================================================
📝 Script 1: Insert + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: 3TfRNrHGUXDleqbnC5Xyzz3e2HcPPJ9iHTN-d9aLgXbMnz-UyL50zi68vSOYUsiYFN6uqeXRfw9vlrvwY5BlzA

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Name: [object Object]
   📋 Email: {"type":"String","value":"alice@example.com"}
   📋 Status: [object Object]
   📋 Credits: [object Object]

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: 85h2ropiMCiyf1M9MvWRo_ayMIfKz8TJlKF75Keadqq0ihlruBBUqnqHb84Bc4xsQ8oIO_VEMZbVJFZxLLhjQA

2️⃣ Calling Script (Query + Update + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Status updated to: {"type":"String","value":"active"}
   📋 Name: {"value":"Alice Smith","type":"String"}

============================================================
📝 Script 3: Query + Update Credits + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: m6O8hOzYnUMvck5uqFhRievI8Jz7YdPnXClYUAlQ9skHnrfENTEHtdUuJmlR6quAPZRbjb_7lI2UvGR5npi9JA

2️⃣ Calling Script (Query + Update Credits + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {"value":0,"type":"Integer"}
   📋 Status: {"type":"String","value":"active"}
   📋 Name: {"value":"Alice Smith","type":"String"}

============================================================
📝 Script 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: j79faR_3o86f5SgyQNPbNBBKYorrGOkwVyTfl3H8U3ug_9uYUTdeinFZVMSqk_9faXdmuSPApUyqQYgb4F6rmA

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: 3TfRNrHGUXDleqbnC5Xy...
   ✅ Deleted script: 85h2ropiMCiyf1M9MvWR...
   ✅ Deleted script: m6O8hOzYnUMvck5uqFhR...
   ✅ Deleted script: j79faR_3o86f5SgyQNPb...
   ✅ Deleted collection: users

============================================================
✅ Complete CRUD Scripts Example Finished!
============================================================

💡 Key Takeaways:
   ✅ Scripts chain Functions together
   ✅ Each Script demonstrates operation + verification
   ✅ Parameters make Scripts reusable
   ✅ Verification is built into the Script itself
   ✅ Complete CRUD lifecycle in 4 focused Scripts
[32m✓ crud_scripts.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl/document_ttl.js ===[0m
╔════════════════════════════════════════════════════════╗
║     TTL EXPIRATION VERIFICATION TEST                   ║
╚════════════════════════════════════════════════════════╝

This test verifies that document TTL expiration works correctly.
We will insert documents with short TTL and verify they expire.

✓ Client connected

═══════════════════════════════════════════════════════════
TEST 1: Document TTL Expiration
═══════════════════════════════════════════════════════════

[Step 1] Insert document with 3 second TTL
  Input: {name: 'TTL Test', value: 'should expire'}
  TTL: 3s
  Output: Document ID = LJ7tN0F7HYmFTRh2SZ_CT4d3-tvS6X39GdiIc9qWE8l6XbSW3OgjYukpUQX75McTd4aZsjzNe_b6T1x1p575KQ
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: findById(LJ7tN0F7HYmFTRh2SZ_CT4d3-tvS6X39GdiIc9qWE8l6XbSW3OgjYukpUQX75McTd4aZsjzNe_b6T1x1p575KQ)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: findById(LJ7tN0F7HYmFTRh2SZ_CT4d3-tvS6X39GdiIc9qWE8l6XbSW3OgjYukpUQX75McTd4aZsjzNe_b6T1x1p575KQ)
  Output: Error (expected) - Request failed with status 404: {"error":"Record not found (expired)"}
  ✓ PASS: Document expired (not found error)

═══════════════════════════════════════════════════════════
TEST 2: Multiple Documents with Different TTLs
═══════════════════════════════════════════════════════════

[Step 1] Insert 3 documents with different TTLs
  Inserted: 3s TTL=TRDlCq6n, 10s TTL=8D40T62j, Permanent=Dz4l_BIM
  ✓ PASS: All documents inserted

[Step 2] Wait 5 seconds (3s doc should expire)
  ✓ PASS: 3s TTL document expired
  ✓ PASS: 10s TTL document still exists
  ✓ PASS: Permanent document still exists

[Step 3] Wait 7 more seconds (10s doc should expire)
  ✓ PASS: 10s TTL document expired
  ✓ PASS: Permanent document still exists

═══════════════════════════════════════════════════════════
CLEANUP
═══════════════════════════════════════════════════════════
✓ Deleted test collection

╔════════════════════════════════════════════════════════╗
║              ALL TTL TESTS PASSED ✓                    ║
╚════════════════════════════════════════════════════════╝

TTL expiration is working correctly:
  • Documents with TTL expire after the specified time
  • Documents without TTL persist indefinitely
  • Different TTL durations are handled correctly
[32m✓ document_ttl.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl/websocket_ttl.js ===[0m
╔════════════════════════════════════════════════════════╗
║   WEBSOCKET TTL EXPIRATION VERIFICATION TEST           ║
╚════════════════════════════════════════════════════════╝

This test verifies TTL expiration works via WebSocket connections.
We will use WebSocket to insert, query, and verify TTL expiration.

✓ Client connected

═══════════════════════════════════════════════════════════
TEST: WebSocket TTL Expiration
═══════════════════════════════════════════════════════════

[Step 1] Insert document with 3 second TTL
  Input: {name: 'WS TTL Test', value: 'should expire'}
  TTL: 3s
  Output: Document ID = DbGP5QmjiPdu69nWGqb5ECPOApW1ZRM0c-OnivDsqwo4jQlE5hURHsk-gSJ_fXtt0ZLC9-ddxRAwj1tK6m1Y3Q
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: findById(DbGP5QmjiPdu69nWGqb5ECPOApW1ZRM0c-OnivDsqwo4jQlE5hURHsk-gSJ_fXtt0ZLC9-ddxRAwj1tK6m1Y3Q)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: findById(DbGP5QmjiPdu69nWGqb5ECPOApW1ZRM0c-OnivDsqwo4jQlE5hURHsk-gSJ_fXtt0ZLC9-ddxRAwj1tK6m1Y3Q)
  Output: Error (expected) - Request failed with status 404: {"error":"Record not found (expired)"}
  ✓ PASS: Document expired (not found error)

═══════════════════════════════════════════════════════════
CLEANUP
═══════════════════════════════════════════════════════════
✓ Deleted test collection

╔════════════════════════════════════════════════════════╗
║          WEBSOCKET TTL TEST PASSED ✓                   ║
╚════════════════════════════════════════════════════════╝

WebSocket TTL expiration is working correctly:
  • Documents with TTL inserted via client expire correctly
  • Queries correctly return null for expired documents
[32m✓ websocket_ttl.js completed successfully[0m
[34m
╔════════════════════════════════════════╗[0m
[34m║           Test Summary                 ║[0m
[34m╚════════════════════════════════════════╝[0m
[34mTotal: 10[0m
[32mPassed: 10[0m
[32mFailed: 0[0m
✅ [32mJavaScript direct examples complete![0m
📦 [36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.7.1 prepare
> npm run build


> @ekodb/ekodb-client@0.7.1 build
> tsc


up to date, audited 58 packages in 1s

15 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.7.1 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning JavaScript client library examples...[0m

added 1 package, removed 1 package, and audited 9 packages in 849ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'Mo-EPH1LnusDM7EbvrdHjwYvsa_RSuHOOol1hTQzRK0dngX6ttcbyR9rLiTCOfq5GBq4EVVkb9BbF9xFoPTC_Q'
}

=== Find by ID ===
Found: {
  active: { value: true, type: 'Boolean' },
  id: 'Mo-EPH1LnusDM7EbvrdHjwYvsa_RSuHOOol1hTQzRK0dngX6ttcbyR9rLiTCOfq5GBq4EVVkb9BbF9xFoPTC_Q',
  name: { type: 'String', value: 'Test Record' },
  value: { type: 'Integer', value: 42 }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  value: { type: 'Integer', value: 100 },
  name: { value: 'Updated Record', type: 'String' },
  active: { value: true, type: 'Boolean' },
  id: 'Mo-EPH1LnusDM7EbvrdHjwYvsa_RSuHOOol1hTQzRK0dngX6ttcbyR9rLiTCOfq5GBq4EVVkb9BbF9xFoPTC_Q'
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Batch Insert ===
✓ Batch inserted 5 records
✓ Verified: Found 5 total records in collection

=== Batch Update ===
✓ Batch updated 3 records

=== Batch Delete ===
✓ Batch deleted 3 records

=== Cleanup ===
✓ Deleted collection

✓ All batch operations completed successfully
✓ Client created

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: { username: 'john_doe', userId: 123 }

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: { price: 29.99, name: 'Product 1' }
cache:product:2: { price: 39.989999999999995, name: 'Product 2' }
cache:product:3: { name: 'Product 3', price: 49.989999999999995 }

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 0 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 4

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: false

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: KbgJw-xf-UfaAjAR31jsr3kugA6OSaxsD41eKirwifUDdDQa7WXlNAq2RskrniHrhOBDJj7VhqdlW5WyFpKZeQ

=== List Collections ===
Total collections: 22
Sample collections: schema_documents_client_ts,client_collection_management_js,ttl_cache,schema_users_client_ts,websocket_test

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: qzFC79isXWrs78jhqwAJvjOfQ8nO3yTxUlDrumEwVxS_qv887FWhuEgcCaGauGU6E7fQ2smdieTpz2_Ikibu0w

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: D4dXyHu_GXC-z-fHoLqlJFqOkIDdpEtyxTYRONDecesvmLRyBaSfZgy5D4owK-D78jA1FqDmk8Y2KHMQnULGkA

=== Query Documents ===
✓ Found 2 documents with TTL

=== Update Document ===
✓ Updated document

=== Delete Document ===
✓ Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All document TTL operations completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: fVZjJhrmmXzvShmSuJ9iLh6KLh7SxA9dv2tbRLHkyW0f0Ooa5r5LGvY3292j5hJ8sSnrAWGw6lnjexpRgk6OLw

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: l0GW1pEOpm_qw-irQpanbz6iOaAhegntSzcNggo2s-_Wd3g3ynOh0Z1ich0i3g8PosKEHQ3lY1SPxjTJlEYBLQ

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 5 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
🚀 ekoDB Scripts Example (JavaScript/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: o6bsEqY3XXhNkLKsgOhf0CkyNfO2VOre9DliZpIFQoqTNl2aoDiWBevDv6176wt8MVWam4Oh5j3Dis41J5V6Cw
📊 Found 10 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: ggXwzYp_z1eltafBZDiw4RO__qcgcQdif8_iJRCHpwP8uquP0gIJHp3AnnW4Tvcbvsyh7bdmHBCuD6elgbd-Xw
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: -9ZhE-87PccBT4opjjunVKk3lmgWZnSiai8kJdApQvDIOWA5mfTEI8sNq30K_aKnR9GEkeMmgosL6r04p356rA
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"count":5,"avg_score":60,"max_score":100,"status":"active"}
   {"count":5,"avg_score":50,"max_score":90,"status":"inactive"}

📝 Example 4: Function Management

📋 Total scripts: 22
🔍 Retrieved script: Get Active Users
✏️  Script updated
🗑️  Script deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

✅ All examples completed!
🚀 ekoDB Scripts Example (JavaScript Client)

✅ Client initialized (token exchange automatic)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: qb84O3Ubfb1Kt_GN9cK0j4gzkYdE4cOc-3kVK_Ch3HBSEi03BfeTJc9xRFaUZA7ONoC1oBPbj5T4Nyn_JH1mjA
📊 Found 20 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved
📊 Found 20 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved
📊 Statistics: 2 groups
   {"avg_score":50,"status":"inactive","count":10}
   {"count":10,"status":"active","avg_score":60}
⏱️  Execution time: 0ms

📝 Example 4: Script Management

📋 Total scripts: 24
🔍 Retrieved script: Get Active Users
✏️  Script updated
🗑️  Script deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

📝 Example 5: Multi-Stage Pipeline

✅ Multi-stage script saved
📊 Pipeline executed 2 stages
⏱️  Total execution time: 0ms
📈 Stage breakdown:

📝 Example 6: Count Users

✅ Count script saved
📊 Total user count: 20
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Deleted collection
✅ Deleted test scripts

✅ All examples completed successfully!

💡 Key Advantages of Using the Client:
   • Automatic token management
   • Type-safe Stage builders
   • ChatMessage helpers
   • Cleaner, more maintainable code
   • Built-in error handling
🚀 ekoDB Advanced Scripts Example

📋 Setting up test data...
✅ Created 10 products

📝 Example 1: List All Products

✅ Script saved
📊 Found 10 products
⏱️  Execution time: 0ms

📝 Example 2: Group Products by Category

✅ Script saved
📊 Found 2 categories
   Furniture: 4 items (avg $294.00)
   Electronics: 6 items (avg $325.67)
⏱️  Execution time: 0ms

📝 Example 3: Count All Products

✅ Script saved
📊 Total products: 10
⏱️  Execution time: 0ms

📝 Example 4: Multi-Stage Aggregation

✅ Script saved
📊 Category analysis (2 categories):
   Electronics:
      Products: 6 | Stock: 232 | Avg Rating: ⭐4.52
   Furniture:
      Products: 4 | Stock: 43 | Avg Rating: ⭐4.26

⏱️  Total execution time: 0ms
📈 Pipeline stages:

📝 Example 5: Project Specific Fields

✅ Script saved
📊 Product summaries (10 items, showing first 3):
   1. USB-C Cable - $19 (⭐4.3)
   2. Wireless Mouse - $29 (⭐4.5)
   3. Webcam HD - $119 (⭐4.5)
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced script examples completed!
🚀 ekoDB CRUD Scripts Example

📋 Setting up test data...
✅ Created 10 test users

📝 Example 1: List All Users

✅ Script saved
📊 Found 10 users
⏱️  Execution time: 0ms

📝 Example 2: Count Users by Status

✅ Script saved
📊 User counts by status:
   inactive: 3 users
   active: 7 users
⏱️  Execution time: 0ms

📝 Example 3: Average Score by Role

✅ Script saved
📊 Average scores by role:
   admin: 20.0 (3 users)
   user: 70.0 (7 users)
⏱️  Execution time: 0ms

📝 Example 4: Top Users by Score

✅ Script saved
📊 Users (showing first 5 of 10):
   1. User 2 (user2@example.com) - Score: 20
   2. User 9 (user9@example.com) - Score: 90
   3. User 10 (user10@example.com) - Score: 100
   4. User 6 (user6@example.com) - Score: 60
   5. User 1 (user1@example.com) - Score: 10
⏱️  Execution time: 0ms

📝 Example 5: User Summary Statistics

✅ Script saved
📊 User summary (2 groups):
   inactive users: 3 users, Total Score: 180
   active users: 7 users, Total Score: 370
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD script examples completed!
🚀 ekoDB Search Scripts Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Script saved
📊 Found 5 documents
   1. Database Design Principles (Database)
   2. Vector Databases Explained (Database)
   3. Natural Language Processing (AI)
   4. Introduction to Machine Learning (AI)
   5. Getting Started with ekoDB (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Script saved
📊 Documents by category:
   Database: 3 documents
   AI: 2 documents
⏱️  Execution time: 0ms

📝 Example 3: Select Specific Fields

✅ Script saved
📊 Document titles (5 docs):
   1. Database Design Principles
   2. Vector Databases Explained
   3. Natural Language Processing
   4. Introduction to Machine Learning
   5. Getting Started with ekoDB
⏱️  Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Script saved
📊 Projected documents (showing first 3):
   1. Database Design Principles
   2. Vector Databases Explained
   3. Natural Language Processing
⏱️  Execution time: 0ms

📝 Example 5: All Document Fields

✅ Script saved
📊 All documents (5 total, showing first 2):
   1. Database Design Principles (Database)
   2. Vector Databases Explained (Database)
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples completed!
🚀 ekoDB AI Scripts Example

⚠️  Note: These examples require AI API credentials (OpenAI, etc.)

📋 Setting up test data...
✅ Created 3 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
🤖 AI Response:
   1. Efficient Spatial Analysis: Vector databases allow for more precise calculations and spatial analyses compared to raster databases.
2. High Accuracy: They provide high geometric accuracy due to the use of points, lines, and polygons to represent spatial data.
3. Less Storage Space: They typically require less storage space compared to raster databases.
4. Scalability: Vector data can be easily scaled from small to large without losing any quality or detail.
5. Flexibility: They offer more flexibility in terms of data representation, as they can be easily edited and manipulated.
6. Better Visualization: Vector databases can provide better visual representation for features like points of interest, roads, and boundaries.
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embedding script saved
📊 Generated embeddings for 3 articles
   1. "Advanced Query Patterns" - 1536D vector
   2. "Getting Started with ekoDB" - 1536D vector
   3. "Draft Article" - 1536D vector
⏱️  Execution time: 0ms

📝 Example 3: List All Articles

✅ Script saved
📊 Found 3 articles
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All AI examples completed!
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 77P0Sed4T8RJbJdED1dBLh8R1Eo3ER6nmYWhvflV2NL0PGh-dpNqAIi3XEbkdIxQpZQYaG680sZrgOwe-82d3w
Created Bob: $500 - ID: zE-qmo5i3__5HiINDww0NhICYeVCcyh50qsGUnr7wAzHy9p5yUu6Uj-nmAdlZKZQkX08bKUOD-fUJ3l0GowSMQ

=== Example 1: Begin Transaction ===
Transaction ID: d9058756-f78d-4727-8477-e5a0e86d943b

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 7672cc52-8b7d-4a78-8474-0971dbb6466e
Status before rollback: Active
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
✅ [32mJavaScript client examples complete![0m
