make test-examples-javascript
🧪 [36mRunning JavaScript examples (direct HTTP/WebSocket)...[0m

added 1 package, removed 1 package, and audited 9 packages in 825ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
[34m
╔════════════════════════════════════════╗[0m
[34m║  ekoDB JavaScript Examples Test Suite ║[0m
[34m╚════════════════════════════════════════╝[0m
[34m
=== Checking Server Connection ===[0m
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
  id: 'ISzee-FEEREEXNQaxUTTwQwTksXEeSeJZrDxWS_uweUbJl9ox73YRrg35tGZEuIOQOW5SnGwiIQzSB7uFhkMdw'
}

=== Find by ID ===
Found: {
  name: 'Test Record',
  value: 42,
  active: true,
  id: 'ISzee-FEEREEXNQaxUTTwQwTksXEeSeJZrDxWS_uweUbJl9ox73YRrg35tGZEuIOQOW5SnGwiIQzSB7uFhkMdw'
}

=== Find with Query ===
Found documents: [
  {
    value: { type: 'Integer', value: 42 },
    active: { type: 'Boolean', value: true },
    id: 'ISzee-FEEREEXNQaxUTTwQwTksXEeSeJZrDxWS_uweUbJl9ox73YRrg35tGZEuIOQOW5SnGwiIQzSB7uFhkMdw',
    name: { value: 'Test Record', type: 'String' }
  }
]

=== Update Document ===
Updated: {
  name: { type: 'String', value: 'Updated Record' },
  active: { value: true, type: 'Boolean' },
  id: 'ISzee-FEEREEXNQaxUTTwQwTksXEeSeJZrDxWS_uweUbJl9ox73YRrg35tGZEuIOQOW5SnGwiIQzSB7uFhkMdw',
  value: { type: 'Integer', value: 100 }
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
✓ Inserted test record: pAed3XzykALKu2H_RZtGGgHWD-bpVXZLcLZxJGJpsb93YUB8UNz3ACCCfeXVQVzH5g9c6HiCULpql6XqxYV7Gg

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
        "id": "V-GwO5brmP15gkZCWnGtp4o8ngZYVpyshElKjClatQ0lIDPngkQSMEFFFJNE6mPPBwj0_pYxOs-mnpoye8qKTw",
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
        "id": "9EeVbFUN6VQJg44lIJj9v3V6ea92aTPNLFIVKep_EE20y-vFENwg8lMAnyr-UA0MPFZS97bncRRXhrpvyMEw2Q",
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
        "id": "pAed3XzykALKu2H_RZtGGgHWD-bpVXZLcLZxJGJpsb93YUB8UNz3ACCCfeXVQVzH5g9c6HiCULpql6XqxYV7Gg",
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
        "id": "2ohhdIKZs8t7QdU7ZWUXCc4EPm8T-Iy-RVYfPio_EofPFq65P9o3SaJhKtwl8h8xuajQE9DK3IsJAV0tEM2-dw",
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

✅ Script saved: RpEJTqp-WLuYB8u2InNl6LaxCaaJ3VaAlm1GF5QFvo-th4P8qN5pWelMprqOW2lYZKD4FtoI8h0w1lb5H0r84g
📊 Found 30 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: aWeSLlwiC4KKZx8MrU34XxtTpaGerQm8YtRtEPZriTjtuG5gG5yBzlzMk9PK3HpAm-qZmABRuXcUWU1EzOK8zg
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: vtWb99YwZI-nMiHgQJ5DnVezswnsvYWwh80QScwbn5oNx0MMSqsXuZCsg2uptXz2lU7ZoGwv79PmDCB-YTGiuA
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"count":10,"status":"active","avg_score":60,"max_score":100}
   {"count":10,"status":"inactive","max_score":90,"avg_score":50}

📝 Example 4: Function Management

📋 Total scripts: 52
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
Collection created with first record: RgSYgzmhcKvckPObs_5VCzF_puACzUO0Z1Fazmlt5swT6sfqh-Mg2ykj1MwPxyPkWDkSo26cBVgZc1f7k9XpZg

=== List Collections ===
Total collections: 24
Sample collections: [
  'users',
  'schema_users_client_go',
  'ttl_cache',
  'test_collection',
  'schema_documents_client_go'
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
Created Alice: $1000 - ID: 4F2irs6s2JLgJEGrgNK_x9TXJ2ZRS4gQNOaZA-4q9-Y30V9vsFg5AUe7m1rs2xGzLHTsGgUQKHHDZyoS-IczHw
Created Bob: $500 - ID: XYEptgKpegt81ET44b9xgE7UHQQoagQsfe8MtO3kQx7c5LJUATyhLfQaDEXoHU8l30gGsw2mk0IMqIdX1v27GQ

=== Example 1: Begin Transaction ===
Transaction ID: 6e93353a-ac0b-4dbd-8c61-5bf54016dc97

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
New transaction: 2487875a-c0bc-43f3-ba6a-d27032b0955b
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
   ✅ Script saved: xfujGgLFUVrAYktJ_lc43poyAaD42EfmT-6fGw1acRFkK2-D7CCO9_z78zgWtSPfvvWmAnfxS0cmgCfkUXL5UQ

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Name: [object Object]
   📋 Email: {"value":"alice@example.com","type":"String"}
   📋 Status: [object Object]
   📋 Credits: [object Object]

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: tZ37Ez2CFjEoUUZowXGNDF7IgDXAoqfOpigfJextgCSasvPixT3_gPoklaPCe_il-sd3MDcZCJSuk4ypSfubkg

2️⃣ Calling Script (Query + Update + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Status updated to: {"value":"active","type":"String"}
   📋 Name: {"type":"String","value":"Alice Smith"}

============================================================
📝 Script 3: Query + Update Credits + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: -wVqivgz5ApapYZ6YLfMQ7mxZmif6yLp0NalzfRllLwA2uQr7CiMjsaVfE9R_wZRfq7Z9CI6GMXoNDN_D2-1xg

2️⃣ Calling Script (Query + Update Credits + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {"value":0,"type":"Integer"}
   📋 Status: {"type":"String","value":"active"}
   📋 Name: {"type":"String","value":"Alice Smith"}

============================================================
📝 Script 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: 8JH57yUyosor5FTTUwUCwTyVx7Sslw5NMpNUFDRlYMrsoeLxophPMEQSHIyiplna6-UUJbqmQK2lpi7oSe1rkw

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: xfujGgLFUVrAYktJ_lc4...
   ✅ Deleted script: tZ37Ez2CFjEoUUZowXGN...
   ✅ Deleted script: -wVqivgz5ApapYZ6YLfM...
   ✅ Deleted script: 8JH57yUyosor5FTTUwUC...
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
  Output: Document ID = wfOJDtIiRJVKN_HbulI7B5-QoiuWuXXu8keYH2dXFrCpYA3izHGoEUK_ZCBCmdMBO9Us-0kP3-1o5bCh95ug_A
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: findById(wfOJDtIiRJVKN_HbulI7B5-QoiuWuXXu8keYH2dXFrCpYA3izHGoEUK_ZCBCmdMBO9Us-0kP3-1o5bCh95ug_A)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: findById(wfOJDtIiRJVKN_HbulI7B5-QoiuWuXXu8keYH2dXFrCpYA3izHGoEUK_ZCBCmdMBO9Us-0kP3-1o5bCh95ug_A)
  Output: Error (expected) - Request failed with status 404: {"error":"Record not found (expired)"}
  ✓ PASS: Document expired (not found error)

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
  Output: Document ID = aJUIgHfCNqa50oCrnRjZjFBF9WFpOz_Lu53gygVBFC5heoVO3hCVp3C2L9czI59F6J4dL0E-lsDNzUrQUOytDw
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: findById(aJUIgHfCNqa50oCrnRjZjFBF9WFpOz_Lu53gygVBFC5heoVO3hCVp3C2L9czI59F6J4dL0E-lsDNzUrQUOytDw)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: findById(aJUIgHfCNqa50oCrnRjZjFBF9WFpOz_Lu53gygVBFC5heoVO3hCVp3C2L9czI59F6J4dL0E-lsDNzUrQUOytDw)
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

> @ekodb/ekodb-client@0.8.0 prepare
> npm run build


> @ekodb/ekodb-client@0.8.0 build
> tsc


up to date, audited 58 packages in 1s

15 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.8.0 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning JavaScript client library examples...[0m

added 1 package, removed 1 package, and audited 9 packages in 822ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'c-J7FjA45T_28eBWz3d1MsJOhyi3t2ZuRHP_TxqBNTrt5pKI3IXcL9dFEGmcVq4XNNEyKBgUY5IzLplRrehpeQ'
}

=== Find by ID ===
Found: {
  name: 'Test Record',
  active: true,
  value: 42,
  id: 'c-J7FjA45T_28eBWz3d1MsJOhyi3t2ZuRHP_TxqBNTrt5pKI3IXcL9dFEGmcVq4XNNEyKBgUY5IzLplRrehpeQ'
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  name: { type: 'String', value: 'Updated Record' },
  value: { type: 'Integer', value: 100 },
  id: 'c-J7FjA45T_28eBWz3d1MsJOhyi3t2ZuRHP_TxqBNTrt5pKI3IXcL9dFEGmcVq4XNNEyKBgUY5IzLplRrehpeQ',
  active: { type: 'Boolean', value: true }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: 1kSPwiBAnchWWTAr0NR_C79j4FE0s5_V6ryt5vgbiJGYU5n5FP-uH7ck-xW0a49BqkmyhacK_4rat_QTy267gw

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
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

=== Create Collection (via insert) ===
Collection created with first record: qWaaWgb2rqv_zUJ_u7YoJFivxmWDTX2BiWmsju3vXNT0Sqys5AGFPrNC8PoltFo9NBVI_2BlQqIWHsP1pdYMew

=== List Collections ===
Total collections: 23
Sample collections: schema_users_client_go,ttl_cache,client_collection_management_js,test_collection,schema_documents_client_go

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: { userId: 123, username: 'john_doe' }

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: { name: 'Product 1', price: 29.99 }
cache:product:2: { price: 39.989999999999995, name: 'Product 2' }
cache:product:3: { price: 49.989999999999995, name: 'Product 3' }

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 0 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 10

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: false

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: aJpn6jnSpJFfkmd27PZfaAmFvFSYIqSQaCcgqs3BmNr7Ze0pX2c8h_U2rRESjqhXlhhcS24yPQSxJACGDpcZag
Created Bob: $500 - ID: UvFeQEP1jn70j9pz2NJb0A7ZZGXS1xxT8wSq2Qea106Qc_soYBm-k_ngOmMzO7l6PBQRXYzMN5iecq83bDUNcw

=== Example 1: Begin Transaction ===
Transaction ID: 94259ab8-9359-4015-96ae-3f811a774e85

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: ba3da1fb-02a9-40fe-acb7-d9c4c557423d
Status before rollback: Active
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
=== Query Builder Examples ===

Setting up test data...
✅ Test data created

1. Simple equality query:
Found 2 active users

2. Range query with sorting:
Found 3 users aged 18-65

3. String operations:
Found 2 users with @example.com emails

4. IN operator:
Found 2 privileged users

5. Complex query with multiple conditions:
Found 1 active US users over 21

6. Pagination:
Page 1: 2 users

7. NOT IN operator:
Found 3 valid users

8. Using bypass flags:
Found 2 users (bypassed cache)

=== Cleanup ===
✅ Deleted test collection

✅ Query Builder examples completed!
=== Search Examples ===

Setting up test data...
✅ Test data created

1. Basic full-text search:
Found 2 results
  1. Score: 25.740, Matched: name.value, email, email.value, name
  2. Score: 12.540, Matched: name.value, name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: bio.value, bio, title.value, title
  2. Score: 26.400, Matched: bio.value, bio, title, title.value
  3. Score: 26.400, Matched: title, title.value, bio, bio.value
  4. Score: 26.400, Matched: title.value, title, bio, bio.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio, title.value, title, bio.value
  2. Score: 39.600, Matched: title, bio.value, title.value, bio
  3. Score: 39.600, Matched: title, bio, bio.value, title.value
  4. Score: 39.600, Matched: title.value, bio.value, bio, title

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.777, Matched: 
  2. Score: 0.759, Matched: 
  3. Score: 0.733, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.693, Matched: title.value, title, content.value, content
  2. Score: 1.511, Matched: content, title, title.value, content.value
  3. Score: 0.304, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: skills.value, bio.value, bio, skills

=== Cleanup ===
✅ Deleted test collections

✅ Search examples completed!
=== Schema Management Examples ===

1. Creating user schema with basic fields:
✅ User schema created

2. Creating product schema with text index:
✅ Product schema with indexes created

3. Creating document schema with vector index:
✅ Document schema with vector index created

4. Retrieving collection schema:
Schema fields: [ 'age', 'email', 'name', 'status' ]
Schema version: 1

5. Retrieving collection metadata:
Collection has 4 fields

6. Creating employee schema with all constraint types:
✅ Employee schema with all constraints created

✅ Schema management examples completed!
=== Join Operations Examples ===

Setting up sample data...
✅ Sample data created

1. Single collection join (users with departments):
Found 2 users with department data:
  - Alice Johnson: Engineering
  - Bob Smith: Sales

2. Join with filtering:
Found 1 users in Engineering:
  - Alice Johnson: Building A

3. Join with user profiles:
Found 2 users with profile data:
  - Alice Johnson: Senior Software Engineer
  - Bob Smith: Sales Manager

4. Join orders with user data:
Found 2 completed orders:
  - Laptop ($1200) by Alice Johnson
  - Mouse ($25) by Alice Johnson

5. Complex join with multiple conditions:
Found 2 users with example.com emails:
  - Alice Johnson (alice@example.com): Building A
  - Bob Smith (bob@example.com): Building B

=== Cleanup ===
✅ Deleted test collections

✅ Join operations examples completed!
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: S9fu35Ghe_vJ9Y7se1fPe7OO7g83FLsrDgx_OFFr51bkN1Hlz3lgAIhph0URqHYfvcHN--R2T5_YOBWg2y7qhg

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: UHVnPXecXb8X7FLu6vSMcTEx1S7Y8r24B6x6j0XLbbXMXFLz1MwYFHOJCEsVnarMCeZvkAfSH997F6iXDey5kQ

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

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: yMYODEYHc2fTpFBTno_9tXpgs3SuuNG_j7JEEcRZdkaGijyZJsdOHxWm4vQPkXCdIW9iDjWxJlMjrRTp4Kq7RA

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
=== ekoDB as Edge Cache - Simple Example ===

Creating edge cache function...
✓ Edge cache script created: LAL2bE-ApV3eFbz0bQY5h6BCoHh0mgiOUJP6heILjo26nH8LM1cC2YSN3L6JzC3wehymyTbywYf40vuzCegQYA

Call 1: Cache miss (fetches from API)
Response time: 4ms
Result: {
  "records": [],
  "stats": {
    "input_count": 0,
    "output_count": 0,
    "execution_time_ms": 0,
    "stages_executed": 2,
    "stage_stats": []
  }
}

Call 2: Cache hit (served from ekoDB)
Response time: 5ms (0.8x faster!)
Result: {
  "records": [],
  "stats": {
    "input_count": 0,
    "output_count": 0,
    "execution_time_ms": 0,
    "stages_executed": 2,
    "stage_stats": []
  }
}

=== The Magic ===
- Your DATABASE is your EDGE
- No Redis needed
- No CDN needed
- No cache invalidation logic needed (TTL handles it)
- With ripples: All nodes auto-sync cache
- One service: Database + Cache + Edge Functions

✓ Example complete!

🚀 ekoDB Scripts Example (JavaScript Client)

✅ Client initialized (token exchange automatic)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: XBTPRj29EzpStEiourK7bTJBJAm5Z0xYqU4vIbVgij1cqXD7EwrKBiFyd2F86x-Lq5yWTH5GoXCswaGGHYmx9Q
📊 Found 10 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved
📊 Found 10 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved
📊 Statistics: 2 groups
   {"status":"inactive","avg_score":50,"count":5}
   {"status":"active","count":5,"avg_score":60}
⏱️  Execution time: 0ms

📝 Example 4: Script Management

📋 Total scripts: 55
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
📊 Total user count: 10
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
=== ekoDB Function Composition Examples ===

📋 Setting up test data...

✅ Test data ready

📝 Example 1: Basic Function Composition

Building reusable functions that call each other...

✅ Saved reusable function: fetch_user
✅ Saved composed function: get_user_wrapper (calls fetch_user + projects fields)

📊 Result from composed function:
   Records: 1
   Name: {"value":"User 1","type":"String"}
   Department: {"type":"String","value":"engineering"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using CallFunction to replace inline logic in SWR pattern...

✅ Saved reusable function: fetch_and_store_user
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 78ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 77ms
   📊 Records: 1
   🚀 Cache speedup: 1.0x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

✅ Level 1 function: validate_user
✅ Level 2 function: fetch_slim_user (calls validate_user)
✅ Level 3 function: get_verified_user (calls fetch_slim_user)

📊 Result from 3-level nested composition:
   Records: 1
   Name: User 1
   Department: engineering

🎯 Key Benefit: Each function is independently testable and reusable!
   - validate_user: Used in 100 different workflows
   - fetch_slim_user: Used in 50 workflows
   - get_verified_user: Specific workflow


✅ All composition examples completed!
🚀 ekoDB JavaScript Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved: cnz94JF-bi569KXoSmWyxq03bsC13rBvfM7wKaGJTz0izboiuAj5guDivfXAT27ufrKgW3J5C2v09i844pBdLQ
📊 Found 2 product groups
   {"category":"Electronics","avg_price":575.6666666666666,"count":3}
   {"count":2,"category":"Furniture","avg_price":474}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved
📊 Found 2 categories
   {"count":3,"category":"Electronics"}
   {"category":"Furniture","count":2}
⏱️  Execution time: 0ms

📝 Example 4: High Rating Products

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 5: Script with Parameter Definition

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 6: Multi-Stage Pipeline (FindAll → Group → Count)

✅ Script saved
📊 Pipeline executed 3 stages
⏱️  Total execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All complete script examples finished!

💡 This example demonstrates ekoDB's Script system:
   ✅ FindAll operations
   ✅ Group aggregations (Count, Average)
   ✅ Multi-stage pipelines (FindAll → Group → Count)
   ✅ Parameter definitions
   ✅ Script management (save, call, delete)
🚀 ekoDB JavaScript KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: fSpdhmtzotcbZe5dqLmXeOebk8cZqqpdC64XcA7_CjW_DFAh8zePYOLzhOD4zJJUhnnFQxz9oS36M1VCDPg26Q
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: A_kdW2GLnrWI1PEst4hLAolbUJ1sRR6uQKvXgy36OPcEDbwHWLRk4wgNW1rZqAgUIVRL6ypNYO5OONIO7G8wCA
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"userId":"user_abc","role":"admin"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: QboE5q5KvlvPLRNl0NiG_JxsRKzHmBxap-lDunnGk93BLzMDMutDC66nicB0_0sU-sWDpU7blkxD8q5XnldhzQ
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Script

✅ Script saved: NVVA6C5ZBV_fKbQfEPAdYdp-ZlIz_8LxMUjfdjXEaMilG7w_T1WmL2MDOwEEXYt9AhbSeubUee9EcRV_XUEGWA
📊 Processed order with caching
⏱️  Stages executed: 3
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All KV & Wrapped Types examples completed!

💡 Key takeaways:
   ✅ Use Field.* helpers for type-safe wrapped values
   ✅ Field.decimal() preserves precision (no floating point errors)
   ✅ KV store is great for caching and quick lookups
   ✅ Stage.kv*() functions work within scripts
   ✅ Combine KV caching with collection inserts for real workflows
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
   active: 7 users
   inactive: 3 users
⏱️  Execution time: 0ms

📝 Example 3: Average Score by Role

✅ Script saved
📊 Average scores by role:
   user: 70.0 (7 users)
   admin: 20.0 (3 users)
⏱️  Execution time: 0ms

📝 Example 4: Top Users by Score

✅ Script saved
📊 Users (showing first 5 of 5):
   1. User 1 - Score: 20
   2. User 2 - Score: 40
   3. User 4 - Score: 80
   4. User 3 - Score: 60
   5. User 5 - Score: 100
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
   1. Introduction to Machine Learning (AI)
   2. Vector Databases Explained (Database)
   3. Getting Started with ekoDB (Database)
   4. Database Design Principles (Database)
   5. Natural Language Processing (AI)
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
   1. Introduction to Machine Learning
   2. Vector Databases Explained
   3. Getting Started with ekoDB
   4. Database Design Principles
   5. Natural Language Processing
⏱️  Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Script saved
📊 Projected documents (showing first 3):
   1. Introduction to Machine Learning
   2. Vector Databases Explained
   3. Getting Started with ekoDB
⏱️  Execution time: 0ms

📝 Example 5: All Document Fields

✅ Script saved
📊 All documents (5 total, showing first 2):
   1. Introduction to Machine Learning (AI)
   2. Vector Databases Explained (Database)
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples completed!
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
   Electronics: 6 items (avg $325.67)
   Furniture: 4 items (avg $294.00)
⏱️  Execution time: 0ms

📝 Example 3: Count All Products

✅ Script saved
📊 Total products: 10
⏱️  Execution time: 0ms

📝 Example 4: Multi-Stage Aggregation

✅ Script saved
📊 Category analysis (2 categories):
   Furniture:
      Products: 4 | Stock: 43 | Avg Rating: ⭐4.26
   Electronics:
      Products: 6 | Stock: 232 | Avg Rating: ⭐4.52

⏱️  Total execution time: 0ms
📈 Pipeline stages:

📝 Example 5: Project Specific Fields

✅ Script saved
📊 Product summaries (10 items, showing first 3):
   1. USB-C Cable - $19 (⭐4.3)
   2. Desk Chair - $349 (⭐4.2)
   3. Keyboard - $89 (⭐4.4)
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced script examples completed!
🚀 ekoDB AI Scripts Example

⚠️  Note: These examples require AI API credentials (OpenAI, etc.)

📋 Setting up test data...
✅ Created 3 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
🤖 AI Response:
   Vector databases offer several benefits including:

1. High Precision: They store data in points, lines, and polygons, providing detailed and accurate spatial data representation.

2. Scalability: Vector data remains consistent in quality even when scaling up or down, unlike raster data that can lose quality when zoomed in.

3. Less Storage: Vector data typically requires less storage space compared to raster data for the same geographical area.

4. Attribute Storage: Vector databases can store attribute data directly, allowing for more complex analyses.

5. Flexibility: They allow for easy modification and manipulation of data such as changing the shape, size, and location of map features.
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embedding script saved
📊 Generated embeddings for 3 articles
   1. "Draft Article" - 1536D vector
   2. "Advanced Query Patterns" - 1536D vector
   3. "Getting Started with ekoDB" - 1536D vector
⏱️  Execution time: 0ms

📝 Example 3: List All Articles

✅ Script saved
📊 Found 3 articles
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All AI examples completed!
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_api_user_js (Wq1gwl5Z5Uzf1w4c_NMMNhCpGXMH_VmSlJy6U5Ui6tBONdta0CfDWnRmDfPXwAKSQZTjkO9sjHRtD7AM4OTlmQ)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [],
  "stats": {
    "input_count": 0,
    "output_count": 0,
    "execution_time_ms": 0,
    "stages_executed": 2,
    "stage_stats": []
  }
}
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 5ms (served from cache)
✓ Lightning fast cache hit

🧹 Cleaning up...
✓ Cleanup complete

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: iX4A4qD9JgfBih1Vxc054xhEkAmXKkEOWhaoZIGsirD_1UyyTINJTonb8AKDaBr1PKP9wm9BMQn6QlVCEt2PzA

=== Sending Chat Message ===
Message ID: eYmcuV9ntiiAhiSUeXJf3z18Fax0hXrrCmyKhQxDbAQM_3GQAZLZSBDay3rOhbKEZwdpuEAy31-OVO2Jm47g6Q

=== AI Response ===
There are three products available:

1. ekoDB is a high-performance database product with AI capabilities. It's priced at $99.

2. ekoDB Pro is the enterprise edition product with advanced features. It's priced at $299.

3. ekoDB Cloud is a fully managed cloud database service product. It's priced at $499.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_js',
  record: {
    id: '-JXHF5h8NhEbr8vEGb8HDhynonygnQS46rBoCyHJ4uUSrRXYpM3iVjEwfaD__QcBTMxKa4QlTcF3DAe-H8zxDQ',
    name: 'ekoDB',
    price: 99,
    description: 'A high-performance database product with AI capabilities'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_js',
  record: {
    price: 299,
    name: 'ekoDB Pro',
    description: 'Enterprise edition product with advanced features',
    id: 'cCYlB5gQ7-2QIhhTsp3RHjTb5R3rEo2_fRCHKni2xF2yPOVpQGc4nVkdW3AZz-0esPkMbFZ15yMa1F1NcxsQDA'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_js',
  record: {
    name: 'ekoDB Cloud',
    price: 499,
    id: 'MPL2qnoaPthYmEyB8wXqYOTjVUiCihIo6ioammpE0RdN3Ubkh8LUPMN1-3yog72T_OVcO4eCR7ZQ_iWi0QOMxw',
    description: 'Fully managed cloud database service product'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 2628ms

=== Token Usage ===
Prompt tokens: 461
Completion tokens: 72
Total tokens: 533

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: n9-pVMckb91RgK7JEeUbBtXlaJAde7ZCcB6XKKRvWMZa8dlxAO8mHZQfp9ztERdDk6b98v0wCh5q5943_na9pw

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information provided, the available product is ekoDB. It is a high-performance database product and it is priced at $99.

✓ Second message sent

=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of ekoDB is $99.

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: jUuIgQX4hgXbXDkD2fLLZVkHRXJV5K_MCRjq0VG5muzbwfwsDEbPJ1OjXEzhcNLV6-XNA9STIWTXfWhLh6taLA
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 4

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 3

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: cje7M5VYe_B4Zahi741PxUf0cipLy1QyBHl_Djx3ExP8edV8OsSXencho0zBtJPhWX-ocFHTwxwCbpNGJuNsWQ

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information given, there is a product available named "ekoDB". It is a high-performance database product priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: zbBlLGXCJ1p1p3njqVgtwaIDQ42JNj3sqg2nuxkRBbfA0Z6u-EYAsA3TLTPB0g23ZoEApMDcJigzxq28dlALtQ
  Parent: cje7M5VYe_B4Zahi741PxUf0cipLy1QyBHl_Djx3ExP8edV8OsSXencho0zBtJPhWX-ocFHTwxwCbpNGJuNsWQ

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: zbBlLGXCJ1p1p3njqVgtwaIDQ42JNj3sqg2nuxkRBbfA0Z6u-EYAsA3TLTPB0g23ZoEApMDcJigzxq28dlALtQ (Untitled)
  Session 2: cje7M5VYe_B4Zahi741PxUf0cipLy1QyBHl_Djx3ExP8edV8OsSXencho0zBtJPhWX-ocFHTwxwCbpNGJuNsWQ (Untitled)
  Session 3: jUuIgQX4hgXbXDkD2fLLZVkHRXJV5K_MCRjq0VG5muzbwfwsDEbPJ1OjXEzhcNLV6-XNA9STIWTXfWhLh6taLA (Untitled)
  Session 4: iX4A4qD9JgfBih1Vxc054xhEkAmXKkEOWhaoZIGsirD_1UyyTINJTonb8AKDaBr1PKP9wm9BMQn6QlVCEt2PzA (Untitled)
  Session 5: adrkqaUlLO292DEXjF9g-V8TStYqHCmoKU2zeyvk7CFpO9XTqFR3qss8ZtZSXRoYXns5MrvgmZt6YarMyvxcIA (Untitled)
  Session 6: fwouIl2cr-G6N6vNT0z14MolEjcyX2FSbskE7sSUhtpFrghvIfYLMZYeoj-7-HKa8zhavOf0bf0XN3kitgzqkQ (Untitled)
  Session 7: doaPGJAF7e3z4i0nEIk9aJm3ASjNJ51v1ObUWXku0_s4SqDqnEBotSJ12_4wIanyzkSEcPSqTXEm0jVd_xM1Uw (Untitled)
  Session 8: XEMmWa39EXymaiyPjWK2XGee5O1GdzWDrVv-XIqEBYq4pg3FFS9TKL-u_VHtaFuYOaC-fjydSlX8mx-hHZg8uw (Untitled)
  Session 9: L2n3N8nWclhIqgCg_HvoXpytdb71VhqTt_bw1_KjLcMzxTom1LQ9TYXYlaiJBahUKc_Q6LPU6nAMWPHPP4g4Zw (Untitled)
  Session 10: cpHlk-ADD4R88a2saVoQSXnT-rPHLY-3TLG0Q-4D1KmO5jf_Yq15-g-qFDRzbar4P_XZYuWWMmUL3QOqQXwdeQ (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: zbBlLGXCJ1p1p3njqVgtwaIDQ42JNj3sqg2nuxkRBbfA0Z6u-EYAsA3TLTPB0g23ZoEApMDcJigzxq28dlALtQ

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
✅ [32mJavaScript client examples complete![0m
