make test-examples-javascript
🧪 [36mRunning JavaScript examples (direct HTTP/WebSocket)...[0m

added 1 package, removed 1 package, and audited 9 packages in 954ms

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
  id: 'G0jxqQepD_lv5xfLhu6nVjEd4dO-jHFHPoDAeynywslxh-2KtetwO_7cBhrGIuyHWsJMypImEraqf8XU8AYLLw'
}

=== Find by ID ===
Found: {
  name: { type: 'String', value: 'Test Record' },
  id: 'G0jxqQepD_lv5xfLhu6nVjEd4dO-jHFHPoDAeynywslxh-2KtetwO_7cBhrGIuyHWsJMypImEraqf8XU8AYLLw',
  active: { type: 'Boolean', value: true },
  value: { value: 42, type: 'Integer' }
}

=== Find with Query ===
Found documents: [
  {
    value: { value: 42, type: 'Integer' },
    name: { value: 'Test Record', type: 'String' },
    id: 'G0jxqQepD_lv5xfLhu6nVjEd4dO-jHFHPoDAeynywslxh-2KtetwO_7cBhrGIuyHWsJMypImEraqf8XU8AYLLw',
    active: { value: true, type: 'Boolean' }
  }
]

=== Update Document ===
Updated: {
  value: { type: 'Integer', value: 100 },
  name: { type: 'String', value: 'Updated Record' },
  id: 'G0jxqQepD_lv5xfLhu6nVjEd4dO-jHFHPoDAeynywslxh-2KtetwO_7cBhrGIuyHWsJMypImEraqf8XU8AYLLw',
  active: { value: true, type: 'Boolean' }
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
✓ Inserted test record: 2gCiK1gMfgUB6nocfeRqWpNCqZhSvD7khDx4uZs91BkRA9ju9_4q3AkFAKoC1hXqE1tQW4MJReSZbDfbX8-plg

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "messageId": "1774734496185",
  "payload": {
    "data": [
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "fQW6SBGgSvKuNS5AKtPgbx9f8znVSScfJfJFeZectmHoGisu5TIZw-ftWvtZkUwKd3podcIrg1_b_HMlYHimDA",
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
        "id": "2gCiK1gMfgUB6nocfeRqWpNCqZhSvD7khDx4uZs91BkRA9ju9_4q3AkFAKoC1hXqE1tQW4MJReSZbDfbX8-plg",
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
        "id": "ZODatZcbUzL0VY5XwPCFMuuff3xb33htMyj5AetliQ0GoBfkKJNfjwrL9qqem_ISoiT2b-KFCkj1wZV2c4g9yg",
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
        "id": "gFEqTXq3N-9Rjq25gBAgck5lECyiNA4m7S70bsLetAO7GjvryzHjPOF5SCbfMLKA7rf5fjVQkU9GUpQFU5H-cA",
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

✅ Script saved: Pr0ajsDlrQ0ieNevElMRXOykVN3zn9qPg2yPVHRKcLCnYa521LGxoWtdgFv77g14PxYzf8hzpfLD0FlY5PeWbg
📊 Found 10 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: rVyWof_Gu4QNQEnv4xW-51kt3u9tOoChcTyoARqMlBfoSicBeE9czxkBBE7HwU-mHWWbvZRQPdFHWb0ofpPYSA
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: Amja3rAo718P2Eo_e420WuZ_y9NF1tKF_TGa9yy-M8esU6nGMvVGBG-e6X_nfi9GQVj6p0Aq2IVSUu9tezNUcA
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"avg_score":60,"max_score":100,"count":10,"status":"active"}
   {"count":10,"max_score":90,"avg_score":50,"status":"inactive"}

📝 Example 4: Function Management

📋 Total scripts: 33
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
Retrieved value: { username: 'john_doe', userId: 123 }

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
Collection created with first record: gBn8bIqDQ1mszVLYY_Ikn6_1Y_jBKtdDLicpbmWHfHoKmMNeIkZclpEsoIun2VXF-xwNBq8Z77CB3i5FxQgxqw

=== List Collections ===
Total collections: 25
Sample collections: [
  'schema_products_client_go',
  'schema_employees_client_ts',
  'ws_ttl_test',
  'schema_products_client_ts',
  'audit__ek0_testing'
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
Created Alice: $1000 - ID: 991ivDZcPS4MOkiike8T2xtKfi01pn26CwKGmC_xDIvtWC0TrJkMROX05xy7fgN26rB8F93Ty7Me-DKUCnUI5Q
Created Bob: $500 - ID: _74D5uE04Etz8EUxGMRcFg2cWqi8Nrf8lL6xOKTZFkO1TKhl1B0RdnUVjjqS3kwAtZxzW-9DUSh9pwRM9oRXZg

=== Example 1: Begin Transaction ===
Transaction ID: af844926-9c4a-4011-b81f-5c201faf820b

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: {"type":"Integer","value":800}
Bob: {"type":"Integer","value":700}

=== Example 5: Rollback ===
New transaction: 99ba88e2-30db-42c3-97a3-c3f579f670ef
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: {"type":"Integer","value":700}

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
   ✅ Script saved: rLSa_agY78qTRn3dN6O6ou8b2SjmaiTXW_lgnkggd-mI8MznnLo-wqr-FnuJdnBv2fS7RKv-un504V41An39QA

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
   ✅ Script saved: QGjlNaQc1eb7wJFuwuDh17cy61EG1u9O89HD1KJmAJv-cCOV2fFm2QOqpm4NxIOPdnqbNYefYgJjlV5y71oSaw

2️⃣ Calling Script (Query + Update + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Status updated to: {"value":"active","type":"String"}
   📋 Name: {"value":"Alice Smith","type":"String"}

============================================================
📝 Script 3: Query + Update Credits + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: PyNaRTaJLJxvSKCT0YUIg39kv8JnIKz14_4oGtiFwECqENu3KQlXwT_JBy1fFkZijVivZAbqum66JfRtlCw4fg

2️⃣ Calling Script (Query + Update Credits + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {"type":"Integer","value":0}
   📋 Status: {"type":"String","value":"active"}
   📋 Name: {"type":"String","value":"Alice Smith"}

============================================================
📝 Script 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: bGnbH0o_Biylienaez-0wJpNb8guQgWzYZcc0sxSoKUEhDbX0vnEKXmqCMXV7bQ24Og4KY2hzaDTe9lSHjD70g

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: rLSa_agY78qTRn3dN6O6...
   ✅ Deleted script: QGjlNaQc1eb7wJFuwuDh...
   ✅ Deleted script: PyNaRTaJLJxvSKCT0YUI...
   ✅ Deleted script: bGnbH0o_Biylienaez-0...
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
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/document_ttl.js ===[0m
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
  Output: Document ID = fr2G_8jcbzXHhm_E-P_UqOh5m-KZ_d1lKFDpyYFKrjkCs4ga-2QxSWbJS4P4o-WgDZ6yEicGjTOoMetsevKfdQ
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: findById(fr2G_8jcbzXHhm_E-P_UqOh5m-KZ_d1lKFDpyYFKrjkCs4ga-2QxSWbJS4P4o-WgDZ6yEicGjTOoMetsevKfdQ)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: findById(fr2G_8jcbzXHhm_E-P_UqOh5m-KZ_d1lKFDpyYFKrjkCs4ga-2QxSWbJS4P4o-WgDZ6yEicGjTOoMetsevKfdQ)
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
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/websocket_ttl.js ===[0m
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
  Output: Document ID = mkku4AtYfbwmnXjEU9fqAWMRZQcBUR3_Z8flb6xBFNrs233U0DX6Rd9s3T3m1enmOLWYS-DabHxOLrRi8WfrZA
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: findById(mkku4AtYfbwmnXjEU9fqAWMRZQcBUR3_Z8flb6xBFNrs233U0DX6Rd9s3T3m1enmOLWYS-DabHxOLrRi8WfrZA)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: findById(mkku4AtYfbwmnXjEU9fqAWMRZQcBUR3_Z8flb6xBFNrs233U0DX6Rd9s3T3m1enmOLWYS-DabHxOLrRi8WfrZA)
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

> @ekodb/ekodb-client@0.15.1 prepare
> npm run build


> @ekodb/ekodb-client@0.15.1 build
> tsc


up to date, audited 52 packages in 1s

17 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.15.1 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning JavaScript client library examples...[0m

added 1 package, removed 1 package, and audited 9 packages in 954ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'Xgo-MULamZmRVjQmHb_QFErpRnymeRuikg4nasr6BB86DvfHXyoPsT9fZD-zRVz0G0uU1eqO_br1pYdjPhFACw'
}

=== Find by ID ===
Found: {
  name: { type: 'String', value: 'Test Record' },
  value: { value: 42, type: 'Integer' },
  active: { type: 'Boolean', value: true },
  id: 'Xgo-MULamZmRVjQmHb_QFErpRnymeRuikg4nasr6BB86DvfHXyoPsT9fZD-zRVz0G0uU1eqO_br1pYdjPhFACw'
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  value: { type: 'Integer', value: 100 },
  name: { type: 'String', value: 'Updated Record' },
  id: 'Xgo-MULamZmRVjQmHb_QFErpRnymeRuikg4nasr6BB86DvfHXyoPsT9fZD-zRVz0G0uU1eqO_br1pYdjPhFACw',
  active: { type: 'Boolean', value: true }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: 1ErhYsCvBAOD9MLiDTLbVE--vhsstiLQuc0qEIxTqhdZJSlgLf5H9HvchfvAmM88s68aCm_NadWeXs4gJAoPjA

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
Collection created with first record: Cz2w7S4QQs5Q3bxtrTvpafCdgfOfb6D_01sjH5y4-x5YvfU4l5JEmP16wX2pGStUJQ2h4UrF4ei2_HwsNMSdxQ

=== List Collections ===
Total collections: 24
Sample collections: schema_products_client_go,schema_employees_client_ts,ws_ttl_test,schema_products_client_ts,audit__ek0_testing

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
Retrieved value: { username: 'john_doe', userId: 123 }

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: { price: 29.99, name: 'Product 1' }
cache:product:2: { name: 'Product 2', price: 39.989999999999995 }
cache:product:3: { name: 'Product 3', price: 49.989999999999995 }

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 0 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 17

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: false

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: G9wA4CRUiCL5OkPoGCHANAXW_zKfJk16gg10Jg4VS-hc5H_EdHy888btOwkdiRHIMe7wG6K0NrnL9RBxqhGH7Q
Created Bob: $500 - ID: I2myDs8sm10KHNCNIT65_ylb35gB_Xqbp2rYC607dQ7-3D2u6IiTfNrwdhyGvZ-kyAVwWoQ9XEUqdlUja2kg2Q

=== Example 1: Begin Transaction ===
Transaction ID: 60c3f931-9716-49a9-9e76-ff009e1ec701

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 2bc76593-c59d-4d13-a912-b7cd23442168
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
  1. Score: 25.740, Matched: name.value, name, email.value, email
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title, bio, bio.value, title.value
  2. Score: 26.400, Matched: bio.value, title, bio, title.value
  3. Score: 26.400, Matched: title, bio, title.value, bio.value
  4. Score: 26.400, Matched: title.value, bio.value, title, bio

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio.value, title, bio, title.value
  2. Score: 39.600, Matched: bio.value, bio, title, title.value
  3. Score: 39.600, Matched: title.value, title, bio.value, bio
  4. Score: 39.600, Matched: title, bio.value, title.value, bio

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio.value, bio

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio.value, bio

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.737, Matched: 
  2. Score: 0.734, Matched: 
  3. Score: 0.725, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.694, Matched: title, title.value, content.value, content
  2. Score: 1.495, Matched: title.value, content, content.value, title
  3. Score: 0.290, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: skills.value, bio.value, skills, bio

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
✓ Inserted document: Yj6nruk5ZWWn5STGKYv7_IDYS2Stj1ttKlqa_mdM9uT8MtxWp5IEv4qOLVOVmCKP1fzCSkqjJIF2aPrcv-8WdQ

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: AuYDTTdnb7u-05nplayLc9Z8vH_M-Wl6zC2vCmGh2YSi-bdDF1CZVt9a0XkqMGGogcai1x6KJGbem3IGvTgicw

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
✓ Inserted document with TTL: V7bVGzRnIelDM95FId3aAzxyzuSKziWEB8v_fSGTYe2S8yP99OkEdxL8ubfiZSyppWPCx2h3znRw7z_W7PZPgA

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
✓ Edge cache script created: U62GOKAehmq2W1obs3dw87DZrBnu_tDLSuY7ZQlUORoZ83ULpspnihDg-u2JdUz-ZY6ph0H8DYxvPTIWmvGYiQ

Call 1: Cache miss (fetches from API)
Response time: 41ms
Result: {
  "records": [
    {
      "value": {
        "longitude": -73.99308,
        "timezone": "GMT",
        "current": {
          "interval": 900,
          "time": "2026-03-28T21:45",
          "temperature_2m": 5.5
        },
        "elevation": 32,
        "latitude": 40.710335,
        "current_units": {
          "interval": "seconds",
          "temperature_2m": "°C",
          "time": "iso8601"
        },
        "utc_offset_seconds": 0,
        "generationtime_ms": 0.022292137145996097,
        "timezone_abbreviation": "GMT"
      }
    }
  ],
  "stats": {
    "input_count": 0,
    "output_count": 1,
    "execution_time_ms": 0,
    "stages_executed": 2,
    "stage_stats": []
  }
}

Call 2: Cache hit (served from ekoDB)
Response time: 41ms (1x faster!)
Result: {
  "records": [
    {
      "value": {
        "longitude": -73.99308,
        "timezone": "GMT",
        "current": {
          "interval": 900,
          "time": "2026-03-28T21:45",
          "temperature_2m": 5.5
        },
        "elevation": 32,
        "latitude": 40.710335,
        "current_units": {
          "interval": "seconds",
          "temperature_2m": "°C",
          "time": "iso8601"
        },
        "utc_offset_seconds": 0,
        "generationtime_ms": 0.022292137145996097,
        "timezone_abbreviation": "GMT"
      }
    }
  ],
  "stats": {
    "input_count": 0,
    "output_count": 1,
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

✅ Script saved: GLECVK5jZJAl-Km2P1Gt_ld6Y9CMpMjHs2Ax1sGIhDns6em2-JFHP-U57Ph4Hug1DRN0Ff7aJh9rKi-0R4gTbg
📊 Found 10 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved
📊 Found 10 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved
📊 Statistics: 2 groups
   {"status":"active","avg_score":60,"count":5}
   {"status":"inactive","avg_score":50,"count":5}
⏱️  Execution time: 0ms

📝 Example 4: Script Management

📋 Total scripts: 36
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
   Name: {"type":"String","value":"User 1"}
   Department: {"type":"String","value":"engineering"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 40ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "id": 1,
    "address": {
      "zipcode": "92998-3874",
      "geo": {
        "lng": "81.1496",
        "lat": "-37.3159"
      },
      "city": "Gwenborough",
      "street": "Ku...

Second call (cache hit - from cache):
   ⏱️  Duration: 39ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "id": 1,
    "address": {
      "zipcode": "92998-3874",
      "geo": {
        "lng": "81.1496",
        "lat": "-37.3159"
      },
      "city": "Gwenborough",
      "street": "Ku...
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

✅ Script saved: plgJcDlgjw2MDn1pCZxfONUGtVo8gRqIi7Bo9lgWCbKlmZWP_M6zU4r7Tw55Jr6rHk9fTeYeR-VaJK8JJiMEkg
📊 Found 2 product groups
   {"count":2,"avg_price":474,"category":"Furniture"}
   {"category":"Electronics","avg_price":575.6666666666666,"count":3}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved
📊 Found 2 categories
   {"category":"Furniture","count":2}
   {"category":"Electronics","count":3}
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

✅ Inserted order: xKcvvo6BICBPbe7mBsNqKZjcxi9XibnIeRC1n7uDDAPnhu7TSrtTRl9pUD634CGKW4jQR8rvp6LdFRzo_q3haQ
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: R0RxRLFXFMgmmW47E99d-FyRZMwfT8i4aQcLqrb97Njry07lvwQ-9eClBtmDT4Rd593sfFyz7uNH5OEw9HXdew
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"role":"admin","userId":"user_abc"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: ZTwj-PZCmLWyHoxDW9rEZ2OwCHH2h1az0hHFsQ7CprLqsqzuUXTPToLpHeAminSDep7hJ0IyDrZFmDBGygM95w
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Script

✅ Script saved: 0KfhwlSEIhhUQgH4gfODp0fIevoSuPGESoC0uvi_K0gVJF7m9hNjOdF_FNcFuvCNLRyO5TQaPBbDuN2air0qgQ
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
   inactive: 3 users
   active: 7 users
⏱️  Execution time: 0ms

📝 Example 3: Average Score by Role

✅ Script saved
📊 Average scores by role:
   user: 70.0 (7 users)
   admin: 20.0 (3 users)
⏱️  Execution time: 0ms

📝 Example 4: Top Users by Score

✅ Script saved
📊 Users (showing first 5 of 10):
   1. User 5 - Score: 50
   2. User 10 - Score: 100
   3. User 2 - Score: 20
   4. User 3 - Score: 30
   5. User 8 - Score: 80
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
   2. Getting Started with ekoDB (Database)
   3. Vector Databases Explained (Database)
   4. Introduction to Machine Learning (AI)
   5. Natural Language Processing (AI)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Script saved
📊 Documents by category:
   AI: 2 documents
   Database: 3 documents
⏱️  Execution time: 0ms

📝 Example 3: Select Specific Fields

✅ Script saved
📊 Document titles (5 docs):
   1. Database Design Principles
   2. Getting Started with ekoDB
   3. Vector Databases Explained
   4. Introduction to Machine Learning
   5. Natural Language Processing
⏱️  Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Script saved
📊 Projected documents (showing first 3):
   1. Database Design Principles
   2. Getting Started with ekoDB
   3. Vector Databases Explained
⏱️  Execution time: 0ms

📝 Example 5: All Document Fields

✅ Script saved
📊 All documents (5 total, showing first 2):
   1. Database Design Principles (Database)
   2. Getting Started with ekoDB (Database)
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
   2. Keyboard - $89 (⭐4.4)
   3. Monitor 27" - $399 (⭐4.6)
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
   1. Precise Geometry: Vector databases provide more accurate and detailed presentation of geographic features.

2. Scalability: In the vector model, data remains accurate and sharp when zooming in or out.

3. Less Disk Space: Vector databases can take up less disk space compared to raster databases.

4. Data Integration: Vector databases can facilitate the integration of different datasets.

5. Data Analysis: They allow for robust spatial analysis and mathematical modelling.

6. Maintenance and Update: It is possible to update vector databases without affecting the entire dataset.

7. Attribute Assignment: In vector databases, attributes can be assigned to individual vectors, providing more detailed and comprehensive information for analysis.
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embedding script saved
📊 Generated embeddings for 3 articles
   1. "Draft Article" - 1536D vector
   2. "Getting Started with ekoDB" - 1536D vector
   3. "Advanced Query Patterns" - 1536D vector
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
✓ Created SWR script: fetch_api_user_js (ZkUmbtQ8pVwjkN7sb09ErSqSGQog-OniVDnwzGLWC0otcJK3krnwaOY66tMvlOAWzYqwCKGcGjG1kaX6zEfFQA)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "name": "Leanne Graham",
        "website": "hildegard.org",
        "address": {
          "suite": "Apt. 556",
          "street": "Kulas Light",
          "city": "Gwenborough",
          "geo": {
            "lat": "-37.3159",
            "lng": "81.1496"
          },
          "zipcode": "92998-3874"
        },
        "phone": "1-770-736-8031 x56442",
        "id": 1,
        "company": {
          "catchPhrase": "Multi-layered client-server neural-net",
          "bs": "harness real-time e-markets",
          "name": "Romaguera-Crona"
        },
        "username": "Bret",
        "email": "Sincere@april.biz"
      }
    }
  ],
  "stats": {
    "input_count": 0,
    "output_count": 1,
    "execution_time_ms": 0,
    "stages_executed": 2,
    "stage_stats": []
  }
}
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 42ms (served from cache)
✓ Lightning fast cache hit

🧹 Cleaning up...
✓ Cleanup complete

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation
🚀 ekoDB JavaScript Client - Native SWR Function Examples

📋 Demonstrates:
   • Single-function SWR pattern (replaces 4-step pipeline)
   • Automatic cache checking, HTTP fetching, and cache setting
   • Built-in audit trail support
   • Duration string TTLs ('15m', '1h', '30s')
   • Multi-function pipeline integration
   • Dynamic TTL configuration


Example 1: Basic Native SWR
────────────────────────────────────────────────────────────────────────────────
Single function replaces KvGet → If → HttpRequest → KvSet pipeline
✓ Created native SWR script: github_user_native (Wmzo1ZuJ4zFFQwun1FFkCYG41odeM8jh5MVDayO58VBFotLOAsbDHLWcgISSi907PrXoDfFX7QkdLPjgNfgf3g)

First call (cache miss - will fetch from GitHub API):
  Response time: 16ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 17ms
  Speedup: 0.9x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (ph3Indp8U1Ke98YgWF8s6GXVlDT12luB_cAuURT7ms1tGfntHLARj0MCKILN3YUO8NOrM2rQN6txIT0gFrMxvg)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (Uj8TF9ylFJpjLIv_rMfvBJsXphlC2QTYdxkLnC-gp6pI-AB3FFN-G9ZyACwuaYEdqYmhbKu5VAJffw9IdUasfg)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (PtE8YUG8yOqqxfZXjMOlaCt4OVbtz0_FCR22K9kGTfM74T8h5DmjpXNPvcacqBaBulIWt-VVd6t31qpO3om5XQ)
  ✓ Cached with TTL: 5m (5 minutes)
  ✓ Cached with TTL: 1h (1 hour)
  ✓ Cached with TTL: 30s (30 seconds)

================================================================================
✅ Key Benefits of Native SWR:
✅ Single function: Replaces 4-function cache-aside pattern
✅ Duration strings: Use '15m', '1h', '2h' instead of calculating seconds
✅ Built-in audit: Optional collection parameter for automatic logging
✅ Auto-enrichment: output_field populates params for downstream functions
✅ Transactional: Works correctly in both transactional and non-transactional contexts
✅ KV-optimized: Uses native KV store with proper TTL handling

=== Performance Comparison ===
Legacy Pattern: KvGet → If → HttpRequest → KvSet → Insert (5 functions)
Native SWR:     SWR → Insert (2 functions)
Result:         60% fewer functions, cleaner code, same behavior 🎯

🧹 Cleaning up...
✓ Deleted 4 test scripts

✅ All examples completed!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: Qnx-in5M8OsQ2S-X3KzbbkhGC2xu1cmZiW-isnnLRf6kmdWOTKNu6XGWXNsXT4SrTga-bt2G-7WOvSboQPYgwg

=== Sending Chat Message ===
Message ID: FGfkj-373eFhugb7Wruh3GeT33-96-KEVNkorwYDZ-VQ3AmIR5SSwbf7Q-ZH4qeUbLTNs_jiXpAuTthE_acznQ

=== AI Response ===
The available products and their prices are as follows:

1. **ekoDB Pro**
   - Price: $299
   - Description: Enterprise edition product with advanced features.

2. **ekoDB Cloud**
   - Price: $499
   - Description: Fully managed cloud database service product.

3. **ekoDB**
   - Price: $99
   - Description: A high-performance database product with AI capabilities.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_js',
  record: {
    id: '-quARG-K8B5yvYIQMFlj4uCSAFFIOH3EV5lOPZvxABzb72J3nFTHdyQ248VubtD8Mi4i6iGdgjS7Zn7Xbh9Hqw',
    name: 'ekoDB Pro',
    description: 'Enterprise edition product with advanced features',
    price: 299
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_js',
  record: {
    name: 'ekoDB Cloud',
    price: 499,
    id: '22hyEuuG7S-binvZNERtF21NyGARh-eAOPelAUlof4mh-4JsN105Bj7bDZVW4o54qJ4Pc14Eua17HNsa5ecf-Q',
    description: 'Fully managed cloud database service product'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_js',
  record: {
    price: 99,
    description: 'A high-performance database product with AI capabilities',
    id: 'OJKe8Om0jH6Z-nZmyaymn2Q49LINXaCZvDfDCmk2FaFoMAK4aHhwu9WlKkbOnSMzuxmKxQyzdbmeysbKdUaX1g',
    name: 'ekoDB'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 2383ms

=== Token Usage ===
Prompt tokens: 1298
Completion tokens: 91
Total tokens: 1389

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: X8hX5_OmADq8xs09n7MdwlePVqCWMErem75cGjK4ccKVaRGYsTkH7YmNnVkLen1ksIDkUhGLEvB5w_QRASsQqw

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** High-performance database product
- **Price:** $99

If you need more details or have further questions, feel free to ask!

✓ Second message sent

=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of ekoDB is $99. If you have any more questions or need further information, feel free to ask!

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: tVnKKouZwLf5Vf205Vr9fmvExnDZvs6FYdmxEH1EhIMbDEL1UUhWr_UUT0yGw06rkAw5fvw0ZNwux7DkthW8Aw
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 6

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 5

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: aH41tlisMQ2Mbdy1X_P5pp1QvjOINzzB2yyR9shYSLUJCgT2sASSfkiSFdoUOCzVswcUDXDD8rKsXOrRXvWZ7w

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** A high-performance database product
- **Price:** $99

If you would like more details or have other questions, feel free to ask!

✓ Message 2 sent
  Response: The price of the product **ekoDB** is **$99**.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: REMeZEWkRUIvKGpkV9X5w0nPhoGqWMZDvEoyzbi1nI7xaJTpZTYTa6HbuqGRK-T7nGJBMyDtYmpMB_EdYKhbMQ
  Parent: aH41tlisMQ2Mbdy1X_P5pp1QvjOINzzB2yyR9shYSLUJCgT2sASSfkiSFdoUOCzVswcUDXDD8rKsXOrRXvWZ7w

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: REMeZEWkRUIvKGpkV9X5w0nPhoGqWMZDvEoyzbi1nI7xaJTpZTYTa6HbuqGRK-T7nGJBMyDtYmpMB_EdYKhbMQ (Untitled)
  Session 2: aH41tlisMQ2Mbdy1X_P5pp1QvjOINzzB2yyR9shYSLUJCgT2sASSfkiSFdoUOCzVswcUDXDD8rKsXOrRXvWZ7w (Untitled)
  Session 3: tVnKKouZwLf5Vf205Vr9fmvExnDZvs6FYdmxEH1EhIMbDEL1UUhWr_UUT0yGw06rkAw5fvw0ZNwux7DkthW8Aw (Untitled)
  Session 4: Qnx-in5M8OsQ2S-X3KzbbkhGC2xu1cmZiW-isnnLRf6kmdWOTKNu6XGWXNsXT4SrTga-bt2G-7WOvSboQPYgwg (Untitled)
  Session 5: sx_rMnfqErh-mtyjHAyXFsabMqi00LrFPcyYpdKVjsVkC2qDvI96XR7o-5mkJ-n1g--B4q9yh2ZvVSS9th69DA (Untitled)
  Session 6: cEDZEwnAiPqPjGQvqTYxwxrjJZyEqAy9ETnuVnZGbmvmU11zpr2pTDfnhAhbxgZn-OUc3iEiklbm49mxRRazVQ (Untitled)
  Session 7: i-BUzL5C2HrCbpnbd0AFuVafXbkVVZTFFUmBhONDsxEu6V3nsJBaxZIsxWxd1we4-zkOhR_yG4V4GWuPhJdl3Q (Untitled)
  Session 8: k1E8eIVqGqE-lO_7PkwnwtlQtLl2BUNR-B2feKtveL_8ak_ZAuzJeBKfLFkf_zI-KDxeX90IC5wEWVusJSscMA (Untitled)
  Session 9: 1gjmtoMHROg0KCa8IbrFbAQEV4A-em9sQzlFUQeQgdL-rCg0DPsL2p7KS9KY8zml-OlqB47tmELFAaSLOkddqw (Untitled)
  Session 10: pzOmm3GrdXAkvNrQvEDpLh7CvHMf-1VaM0kvPSika5LbfNwnJcNf5IQaEpCvYaancEPji9cden_C4jNJTfDDcQ (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: REMeZEWkRUIvKGpkV9X5w0nPhoGqWMZDvEoyzbi1nI7xaJTpZTYTa6HbuqGRK-T7nGJBMyDtYmpMB_EdYKhbMQ

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
✅ [32mJavaScript client examples complete![0m
