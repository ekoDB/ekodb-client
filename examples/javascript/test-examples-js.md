make test-examples-javascript
🧪 [36mRunning JavaScript examples (direct HTTP/WebSocket)...[0m

added 1 package, removed 1 package, and audited 9 packages in 1s

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
=== Running ekoDB/ekodb-client/examples/javascript/simple_crud.js ===[0m
=== Simple CRUD Operations (Direct HTTP) ===

✓ Authentication successful

=== Insert Document ===
Inserted: {
  id: 'bOGFlvx_LZHQxqmwPdKc8h6EN8pSxYPAItUYeG5L86Lwg6LBckPQLXgLTujh89iEECKPi1lphhaajpGzV8nC_g'
}

=== Find by ID ===
Found: {
  id: 'bOGFlvx_LZHQxqmwPdKc8h6EN8pSxYPAItUYeG5L86Lwg6LBckPQLXgLTujh89iEECKPi1lphhaajpGzV8nC_g',
  active: { type: 'Boolean', value: true },
  name: { type: 'String', value: 'Test Record' },
  value: { value: 42, type: 'Integer' }
}

=== Find with Query ===
Found documents: [
  {
    active: { value: true, type: 'Boolean' },
    id: 'bOGFlvx_LZHQxqmwPdKc8h6EN8pSxYPAItUYeG5L86Lwg6LBckPQLXgLTujh89iEECKPi1lphhaajpGzV8nC_g',
    name: { value: 'Test Record', type: 'String' },
    value: { type: 'Integer', value: 42 }
  }
]

=== Update Document ===
Updated: {
  id: 'bOGFlvx_LZHQxqmwPdKc8h6EN8pSxYPAItUYeG5L86Lwg6LBckPQLXgLTujh89iEECKPi1lphhaajpGzV8nC_g',
  active: { value: true, type: 'Boolean' },
  value: { type: 'Integer', value: 100 },
  name: { value: 'Updated Record', type: 'String' }
}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
[32m✓ simple_crud.js completed successfully[0m
[34m
=== Running ekoDB/ekodb-client/examples/javascript/simple_websocket.js ===[0m
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: 30SauUmfPHlTgmROWW2BDaUNvLyJ8PnKii8WxntAuOTQK1r1jd4TkA09L6ttBMdNREHVF3HXwasWOhW5-Rwx4A

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "type": "Success",
  "payload": {
    "data": [
      {
        "id": "myzh8ozWbo5uFwQiqnAkokhilfiGHiMexcrtOt5iGq6OK6Nf_x7KPR6M7rjnYzTUbCq1xZf2P8lSNWf4D7FC5Q",
        "active": {
          "type": "Boolean",
          "value": true
        },
        "value": {
          "type": "Integer",
          "value": 42
        },
        "name": {
          "type": "String",
          "value": "WebSocket Test Record"
        }
      },
      {
        "value": {
          "value": 42,
          "type": "Integer"
        },
        "id": "30SauUmfPHlTgmROWW2BDaUNvLyJ8PnKii8WxntAuOTQK1r1jd4TkA09L6ttBMdNREHVF3HXwasWOhW5-Rwx4A",
        "name": {
          "value": "WebSocket Test Record",
          "type": "String"
        },
        "active": {
          "type": "Boolean",
          "value": true
        }
      },
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "eo4k2XKeeP4Jw-NIruzkp5BZ37laWem-xan_dukdvdrmZzoejTtyeo5p4ZQXOFLyMu6bG4acNriPRHzHTq0EoQ",
        "name": {
          "type": "String",
          "value": "WebSocket Test Record"
        },
        "value": {
          "value": 42,
          "type": "Integer"
        }
      },
      {
        "id": "WsbRCK94m4hP3hqb2DP9BPADED5Qb3sY8pyagkVQFOgdbEuIFn84mi1fBQkRrv_aJlCNvJp3vQhqcdHXQRFFCQ",
        "name": {
          "value": "WebSocket Test Record",
          "type": "String"
        },
        "value": {
          "type": "Integer",
          "value": 42
        },
        "active": {
          "value": true,
          "type": "Boolean"
        }
      }
    ]
  },
  "messageId": "1788817699073"
}
✓ Retrieved 4 record(s) via WebSocket

✓ WebSocket example completed successfully
WebSocket closed
[32m✓ simple_websocket.js completed successfully[0m
[34m
=== Running ekoDB/ekodb-client/examples/javascript/http_functions.js ===[0m
🚀 ekoDB Functions Example (JavaScript/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Function saved: SaaBPM8pwKJbBJfM6OKoE2M-GbJfS2R7PSh0RKZhoMYYJRIwFebyCfJCzw6gM3jqnG-4ekRcoBLs-aC0Rd0AMg
📊 Found 10 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Function saved: WL5OEqVozpKxLCsiSXgVHr5Z2qTyf7FQhzT1f7RGGEr_rrqnZpbSoptmE16PrVzh-3x51ruGXZG_nwWkRmpzHg
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: jiqLMzqN44cJwOli6ucLpzLzZ6vNllgJ3gvdG20e4rVYjteqmwyjXLvCpd3_R-x_YqnYBGmeQ6y_fq2XiqYi-g
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"avg_score":{"value":50,"type":"Float"},"count":{"type":"Integer","value":10},"status":{"value":"inactive","type":"String"},"max_score":{"type":"Integer","value":90}}
   {"status":{"type":"String","value":"active"},"avg_score":{"value":60,"type":"Float"},"count":{"type":"Integer","value":10},"max_score":{"type":"Integer","value":100}}

📝 Example 4: Function Management

📋 Total functions: 16
🔍 Retrieved function: Get Active Users
✏️  Function updated
🗑️  Function deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

✅ All examples completed!
[32m✓ http_functions.js completed successfully[0m
[34m
=== Running ekoDB/ekodb-client/examples/javascript/batch_operations.js ===[0m
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
=== Running ekoDB/ekodb-client/examples/javascript/kv_operations.js ===[0m
=== Key-Value Operations (Direct HTTP) ===

✓ Authentication successful

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: { type: 'Object', value: { userId: 123, username: 'john_doe' } }

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: { value: { name: 'Product 1', price: 29.99 }, type: 'Object' }
cache:product:2: {
  value: { price: 39.989999999999995, name: 'Product 2' },
  type: 'Object'
}
cache:product:3: {
  type: 'Object',
  value: { name: 'Product 3', price: 49.989999999999995 }
}

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
[32m✓ kv_operations.js completed successfully[0m
[34m
=== Running ekoDB/ekodb-client/examples/javascript/collection_management.js ===[0m
=== Collection Management (Direct HTTP) ===

✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: WWijaCrYn5XZtAggBHZwVdq0K3_lHsJMTKD4zM90bYvfK88hhUqv5FkIBjRkXWM2QxQV39_bKXS6g68D3x8Z0w

=== List Collections ===
Total collections: 27
Sample collections: [
  'schema_documents_client_ts',
  'schema_employees_client_ts',
  'ttl_cache',
  'schema_employees_client_go',
  'test_collection'
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
=== Running ekoDB/ekodb-client/examples/javascript/transactions.js ===[0m
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: G5mVE4nypxLFIGcVWJZc9GU0rcOC03LHGttMtQFVgiS6SY8XnhoQz-ROq6NI7jZAz3BNn134Pbn-qh8BjVey0w
Created Bob: $500 - ID: Tbwoaobti5a44xtSdM-ejs2wNkzAM-a3LkTMqYFoBgWW7zDpPHtngKkgRz8WyYwxGXkXlc-fjGdgMjfelQaGGA

=== Example 1: Begin Transaction ===
Transaction ID: cce50fd3-d086-430a-b542-113df7cf178d

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: {"value":800,"type":"Integer"}
Bob: {"value":700,"type":"Integer"}

=== Example 5: Rollback ===
New transaction: f04faf3b-c019-4955-8641-4ec15133ad38
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: {"type":"Integer","value":700}

=== Cleanup ===
✓ Deleted test accounts

✓ All transaction examples completed
[32m✓ transactions.js completed successfully[0m
[34m
=== Running ekoDB/ekodb-client/examples/javascript/crud_functions.js ===[0m
🚀 ekoDB Complete CRUD Functions Example
============================================================
Demonstrates:
  • Insert + Verify (using Query)
  • Query + Update Status + Verify
  • Query + Update Credits + Verify
  • Query Before Delete + Delete + Verify Gone

Each function shows Functions chaining with proper verification
============================================================

============================================================
📝 Function 1: Insert + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: dKuZr4Zk5Oz7OLxx0qRprLm3JSfycWn-Pxj8EPmzWjbw6aXm6Z4niZJ8K_WNyjeyZ8jEXBh77FpBK1UUbeg96Q

2️⃣ Calling Function (Insert + Verify)...
   ✅ Function executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Name: [object Object]
   📋 Email: {"type":"String","value":"alice@example.com"}
   📋 Status: [object Object]
   📋 Credits: [object Object]

============================================================
📝 Function 2: Query + Update + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: JZzbWlNrZKde4fecuOtHnCgjUAFkKr70Y5CMpV2ajObIkj4x42kf2P-2y9N6fkmFWPh1AjqMhjLl6F8nZAl0xg

2️⃣ Calling Function (Query + Update + Verify)...
   ✅ Function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Status updated to: {"value":"active","type":"String"}
   📋 Name: {"type":"String","value":"Alice Smith"}

============================================================
📝 Function 3: Query + Update Credits + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: Ttk7h2n4tkWFpowOVp3Gk1XBca3MUdD445jBYxtjGGTQNUovMPuKSD9vk4YDu93Z8sIL6m9UfnQ9xfCb8YgPXg

2️⃣ Calling Function (Query + Update Credits + Verify)...
   ✅ Function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {"type":"Integer","value":0}
   📋 Status: {"type":"String","value":"active"}
   📋 Name: {"type":"String","value":"Alice Smith"}

============================================================
📝 Function 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: De48f8b30oUb78q5mxNzcwsJPKaO2kfqfZhs5QHHMldEmsr7hnnY6hr7pq5owdwmhhqLIVycjdG8Q3r5t58dIQ

2️⃣ Calling Function (Query + Delete + Verify)...
   ✅ Function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted function: dKuZr4Zk5Oz7OLxx0qRp...
   ✅ Deleted function: JZzbWlNrZKde4fecuOtH...
   ✅ Deleted function: Ttk7h2n4tkWFpowOVp3G...
   ✅ Deleted function: De48f8b30oUb78q5mxNz...
   ✅ Deleted collection: users

============================================================
✅ Complete CRUD Functions Example Finished!
============================================================

💡 Key Takeaways:
   ✅ Functions chain Functions together
   ✅ Each function demonstrates operation + verification
   ✅ Parameters make functions reusable
   ✅ Verification is built into the function itself
   ✅ Complete CRUD lifecycle in 4 focused functions
[32m✓ crud_functions.js completed successfully[0m
[34m
=== Running ekoDB/ekodb-client/examples/javascript/document_ttl.js ===[0m
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
  Output: Document ID = ufNj738-hhpGlOGtPpFC0kBbRIceW1bmxuaCUXJysjSuhzUlym3mUZp8LsbzchQn3oIJ_81y1PGJLrkfZUTRcA
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: findById(ufNj738-hhpGlOGtPpFC0kBbRIceW1bmxuaCUXJysjSuhzUlym3mUZp8LsbzchQn3oIJ_81y1PGJLrkfZUTRcA)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: findById(ufNj738-hhpGlOGtPpFC0kBbRIceW1bmxuaCUXJysjSuhzUlym3mUZp8LsbzchQn3oIJ_81y1PGJLrkfZUTRcA)
  Output: Error (expected) - Request failed with status 404: {"error":"Record has been deleted"}
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
=== Running ekoDB/ekodb-client/examples/javascript/websocket_ttl.js ===[0m
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
  Output: Document ID = XI8j-BkXaFUShmizLv5H7ydUQ4DUcdC6pOfLuJHo6pIP3sTez3fqT8cxW2zObRDH5r_RQ9AKDHnrFiGkdA52_w
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: findById(XI8j-BkXaFUShmizLv5H7ydUQ4DUcdC6pOfLuJHo6pIP3sTez3fqT8cxW2zObRDH5r_RQ9AKDHnrFiGkdA52_w)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: findById(XI8j-BkXaFUShmizLv5H7ydUQ4DUcdC6pOfLuJHo6pIP3sTez3fqT8cxW2zObRDH5r_RQ9AKDHnrFiGkdA52_w)
  Output: Error (expected) - Request failed with status 404: {"error":"Record has been deleted"}
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

> @ekodb/ekodb-client@0.26.1 prepare
> npm run build


> @ekodb/ekodb-client@0.26.1 build
> tsc


up to date, audited 52 packages in 1s

17 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.26.1 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning JavaScript client library examples...[0m

added 1 package, removed 1 package, and audited 9 packages in 1s

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'a-ph9PLR3yA6ROx2qUOTSRBETmWwMgGEM3-yPK2Hf9Z48we5Su49eunzPeZ-ZFNv1ssOSFnrMZqOzBVKD_BsrQ'
}

=== Find by ID ===
Found: {
  id: 'a-ph9PLR3yA6ROx2qUOTSRBETmWwMgGEM3-yPK2Hf9Z48we5Su49eunzPeZ-ZFNv1ssOSFnrMZqOzBVKD_BsrQ',
  value: { value: 42, type: 'Integer' },
  name: { value: 'Test Record', type: 'String' },
  active: { value: true, type: 'Boolean' }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  id: 'a-ph9PLR3yA6ROx2qUOTSRBETmWwMgGEM3-yPK2Hf9Z48we5Su49eunzPeZ-ZFNv1ssOSFnrMZqOzBVKD_BsrQ',
  value: { type: 'Integer', value: 100 },
  name: { type: 'String', value: 'Updated Record' },
  active: { value: true, type: 'Boolean' }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: Fd0uqTqPo3pRXAu-a0nrXg7y92xbeeyBIbFNtKtWYFK9NkGwnK-b1knON8REZBpHh8VU4oG9tGe1PVyRhMfByw

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
Collection created with first record: Mpk1doLgoJaKHTsy-oMF4jN0QWYHVSCFk0jJHtGDdm5GldPNmWcX3m9TWdUCJwJUhl_EppS_XmA_hwTx4ePwfg

=== List Collections ===
Total collections: 26
Sample collections: schema_documents_client_ts,schema_employees_client_ts,ttl_cache,schema_employees_client_go,test_collection

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
Retrieved value: { value: { username: 'john_doe', userId: 123 }, type: 'Object' }

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: { value: { name: 'Product 1', price: 29.99 }, type: 'Object' }
cache:product:2: {
  type: 'Object',
  value: { name: 'Product 2', price: 39.989999999999995 }
}
cache:product:3: {
  value: { price: 49.989999999999995, name: 'Product 3' },
  type: 'Object'
}

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 0 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 16

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: false

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 8388gaxQtK_sE3MDC0Y82OD8NnKLK305EuF_qNmtVSBbHpyuQPm2-fz0nDXQirGLcp5V8T0cH_aZEdBRq4by4w
Created Bob: $500 - ID: ZFmh5ReBlu2lE2L2xwjVCosVn0Ht6Y8U5DCgOKvtQ5jPvP-wSplLPMPkqqsw3NQrVBqN6yOWrezT3jWQ0Twh0g

=== Example 1: Begin Transaction ===
Transaction ID: 69d3b392-67b4-48a7-8f38-058899be1d98

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: b557f828-e18d-49ad-bcd8-632cab16c372
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
  1. Score: 12.870, Matched: email, name
  2. Score: 6.270, Matched: name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 13.200, Matched: title, bio
  2. Score: 13.200, Matched: title, bio
  3. Score: 13.200, Matched: bio, title
  4. Score: 13.200, Matched: bio, title

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 26.400, Matched: title, bio
  2. Score: 26.400, Matched: title, bio
  3. Score: 26.400, Matched: title, bio
  4. Score: 26.400, Matched: bio, title

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 6.600, Matched: bio
  2. Score: 6.600, Matched: bio

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 6.600, Matched: bio

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.765, Matched: 
  2. Score: 0.763, Matched: 
  3. Score: 0.762, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.506, Matched: title, content
  2. Score: 0.905, Matched: content, title
  3. Score: 0.305, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 13.200, Matched: skills, bio

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
  - Bob Smith: Sales
  - Alice Johnson: Engineering

2. Join with filtering:
Found 1 users in Engineering:
  - Alice Johnson: Building A

3. Join with user profiles:
Found 2 users with profile data:
  - Bob Smith: Sales Manager
  - Alice Johnson: Senior Software Engineer

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
✓ Inserted document: 5iUKB3vl_8r3an3YTwpM1n_l-D-s998qAwj2ijwIviMCDKmE12jyQsrNy9d0WduHHRh0umq3rWgSCU-1eWuVHg

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: KjTWGyCsy24EufCwYab8tk5Q6XcJny7WpO1utUTqPWdAvDNi_W4gyKkVM0ZqKSbBv0M4QFxSDKCANr00E7s73g

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
✓ Inserted document with TTL: esZ_-EMaD4oATYNf86HL5XgEg8qy_uJOLSId_X_sK8moGZ8eN5tLAMt_GkgQEEAxWEsd3G9Bz1GERmPHzGqO2g

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
✓ Edge cache script created: APLTP1otmMB7tvUYWXmAO2XWwz9CDd69M6BncQX3D14nxV_TcMKVUNx0dG2pne6nyzo4StclEmTl-isEjXnP0g

Call 1: Cache miss (fetches from API)
Response time: 2ms
Result: {
  "records": [
    {
      "value": {
        "value": {
          "latitude": 40.710335,
          "longitude": -73.99308,
          "timezone_abbreviation": "GMT",
          "utc_offset_seconds": 0,
          "timezone": "GMT",
          "current": {
            "interval": 900,
            "temperature_2m": 26.4,
            "time": "2026-09-07T21:45"
          },
          "current_units": {
            "interval": "seconds",
            "temperature_2m": "°C",
            "time": "iso8601"
          },
          "elevation": 32,
          "generationtime_ms": 0.020742416381835938
        },
        "type": "Object"
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
Response time: 2ms (1x faster!)
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "latitude": 40.710335,
          "longitude": -73.99308,
          "timezone_abbreviation": "GMT",
          "utc_offset_seconds": 0,
          "timezone": "GMT",
          "current": {
            "interval": 900,
            "temperature_2m": 26.4,
            "time": "2026-09-07T21:45"
          },
          "current_units": {
            "interval": "seconds",
            "temperature_2m": "°C",
            "time": "iso8601"
          },
          "elevation": 32,
          "generationtime_ms": 0.020742416381835938
        }
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

🚀 ekoDB Functions Example (JavaScript Client)

✅ Client initialized (token exchange automatic)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

ℹ️  Function 'get_active_users' already existed — updated instead
✅ Function saved: QeG6MJqTboXd0b7yT8sbqgZUo0e9TnLqva3MAIpsaOiIiP3ICySHL_be-JUV4dg86Qi8s9z4ZXZ_4ESi3Ed7ZA
📊 Found 10 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Function

ℹ️  Function 'get_users_by_status' already existed — updated instead
✅ Function saved
📊 Found 10 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Function

✅ Function saved
📊 Statistics: 2 groups
   {"status":{"type":"String","value":"inactive"},"avg_score":{"value":50,"type":"Float"},"count":{"value":5,"type":"Integer"}}
   {"avg_score":{"type":"Float","value":60},"status":{"type":"String","value":"active"},"count":{"type":"Integer","value":5}}
⏱️  Execution time: 0ms

📝 Example 4: Function Management

📋 Total functions: 17
🔍 Retrieved function: Get Active Users
✏️  Function updated
🗑️  Function deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

📝 Example 5: Multi-Stage Pipeline

✅ Multi-stage function saved
📊 Pipeline executed 2 stages
⏱️  Total execution time: 0ms
📈 Stage breakdown:

📝 Example 6: Count Users

✅ Count function saved
📊 Total user count: 10
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Deleted collection
✅ Deleted test functions

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

ℹ️  Function 'fetch_user' already existed — updated instead
✅ Saved reusable function: fetch_user
✅ Saved composed function: get_user_wrapper (calls fetch_user + projects fields)

📊 Result from composed function:
   Records: 1
   Name: {"value":"User 1","type":"String"}
   Department: {"value":"engineering","type":"String"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

ℹ️  Function 'fetch_and_store_user' already existed — updated instead
✅ Saved reusable function: fetch_and_store_user (uses KV)
ℹ️  Function 'swr_user' already existed — updated instead
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 5ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "address": {
        "street": "Kulas Light",
        "city": "Gwenborough",
        "geo": {
          "lat": "-37.3159",
          "lng": "8...

Second call (cache hit - from cache):
   ⏱️  Duration: 2ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "address": {
        "street": "Kulas Light",
        "city": "Gwenborough",
        "geo": {
          "lat": "-37.3159",
          "lng": "8...
   🚀 Cache speedup: 2.5x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

ℹ️  Function 'validate_user' already existed — updated instead
✅ Level 1 function: validate_user
ℹ️  Function 'fetch_slim_user' already existed — updated instead
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
🚀 ekoDB JavaScript Complete Functions Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Function saved: JBp18BiG-VWP6tQvyr9h5gvkYaoCuS0KgCQW1UExAXaimTodLuoeaf6fa38zDS8ZSphB9fPXrZO2jCNRJX-nRA
📊 Found 2 product groups
   {"category":{"value":"Furniture","type":"String"},"count":{"type":"Integer","value":2},"avg_price":{"value":474,"type":"Float"}}
   {"count":{"type":"Integer","value":3},"avg_price":{"type":"Float","value":575.6666666666666},"category":{"type":"String","value":"Electronics"}}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   {"count":{"type":"Integer","value":2},"category":{"type":"String","value":"Furniture"}}
   {"count":{"type":"Integer","value":3},"category":{"value":"Electronics","type":"String"}}
⏱️  Execution time: 0ms

📝 Example 4: High Rating Products

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 5: Function with Parameter Definition

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 6: Multi-Stage Pipeline (FindAll → Group → Count)

✅ Function saved
📊 Pipeline executed 3 stages
⏱️  Total execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All complete function examples finished!

💡 This example demonstrates ekoDB's Function system:
   ✅ FindAll operations
   ✅ Group aggregations (Count, Average)
   ✅ Multi-stage pipelines (FindAll → Group → Count)
   ✅ Parameter definitions
   ✅ Function management (save, call, delete)
🚀 ekoDB JavaScript KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: L0elCMymegWeTYe-AYmNMMP_NqibJBnYFdKUBL6b3t6T_9mC5paClgCEzRdszUdq-J9sfgC6qIS0WDl4M10DfA
✅ Inserted 2 products with wrapped types

📝 Example 2: Function with Wrapped Type Parameters

✅ Function saved: yuFJUMlqcpYZOUAcEE_bcuBsuUq79pQt6p0vIQ2l9sNC_2kT14cBM6bkKq_NodKfkn538uXTLV3iAZ9Ca3CdcQ
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"value":{"role":"admin","userId":"user_abc"},"type":"Object"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: 9BTYyNWvCNn97AiQ1hTrBDKd9347up-_twDtT47zM1jTwtDrPdtAUp_AaQZe_QZojEvEFb8fwqWqSlbJ5G0Urg
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Function

✅ Function saved: cpzB3nWBcBmcnS514_DBDA5cLT31xUg4RZRSyan3N4LbvV_kPMCNOljjh5e_mWP_Kyg1EAosZGXF3o0pBqlpMA
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
🚀 ekoDB CRUD Functions Example

📋 Setting up test data...
✅ Created 10 test users

📝 Example 1: List All Users

✅ Function saved
📊 Found 10 users
⏱️  Execution time: 0ms

📝 Example 2: Count Users by Status

✅ Function saved
📊 User counts by status:
   active: 7 users
   inactive: 3 users
⏱️  Execution time: 0ms

📝 Example 3: Average Score by Role

✅ Function saved
📊 Average scores by role:
   admin: 20.0 (3 users)
   user: 70.0 (7 users)
⏱️  Execution time: 0ms

📝 Example 4: Top Users by Score

✅ Function saved
📊 Users (showing first 5 of 10):
   1. User 2 - Score: 20
   2. User 7 - Score: 70
   3. User 10 - Score: 100
   4. User 6 - Score: 60
   5. User 3 - Score: 30
⏱️  Execution time: 0ms

📝 Example 5: User Summary Statistics

✅ Function saved
📊 User summary (2 groups):
   active users: 7 users, Total Score: 370
   inactive users: 3 users, Total Score: 180
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD function examples completed!
🚀 ekoDB Search Functions Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Function saved
📊 Found 5 documents
   1. Getting Started with ekoDB (Database)
   2. Introduction to Machine Learning (AI)
   3. Natural Language Processing (AI)
   4. Database Design Principles (Database)
   5. Vector Databases Explained (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   Database: 3 documents
   AI: 2 documents
⏱️  Execution time: 0ms

📝 Example 3: Select Specific Fields

✅ Function saved
📊 Document titles (5 docs):
   1. Getting Started with ekoDB
   2. Introduction to Machine Learning
   3. Natural Language Processing
   4. Database Design Principles
   5. Vector Databases Explained
⏱️  Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Function saved
📊 Projected documents (showing first 3):
   1. Getting Started with ekoDB
   2. Introduction to Machine Learning
   3. Natural Language Processing
⏱️  Execution time: 0ms

📝 Example 5: All Document Fields

✅ Function saved
📊 All documents (5 total, showing first 2):
   1. Getting Started with ekoDB (Database)
   2. Introduction to Machine Learning (AI)
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search function examples completed!
🚀 ekoDB Advanced Functions Example

📋 Setting up test data...
✅ Created 10 products

📝 Example 1: List All Products

✅ Function saved
📊 Found 10 products
⏱️  Execution time: 0ms

📝 Example 2: Group Products by Category

✅ Function saved
📊 Found 2 categories
   Electronics: 6 items (avg $325.67)
   Furniture: 4 items (avg $294.00)
⏱️  Execution time: 0ms

📝 Example 3: Count All Products

✅ Function saved
📊 Total products: 10
⏱️  Execution time: 0ms

📝 Example 4: Multi-Stage Aggregation

✅ Function saved
📊 Category analysis (2 categories):
   Electronics:
      Products: 6 | Stock: 232 | Avg Rating: ⭐4.52
   Furniture:
      Products: 4 | Stock: 43 | Avg Rating: ⭐4.26

⏱️  Total execution time: 0ms
📈 Pipeline stages:

📝 Example 5: Project Specific Fields

✅ Function saved
📊 Product summaries (10 items, showing first 3):
   1. Bookshelf - $149 (⭐4.1)
   2. Keyboard - $89 (⭐4.4)
   3. USB-C Cable - $19 (⭐4.3)
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced function examples completed!
🚀 ekoDB AI Functions Example

⚠️  Note: These examples require AI API credentials (OpenAI, etc.)

📋 Setting up test data...
✅ Created 3 articles

📝 Example 1: Simple Chat Completion

✅ Chat function saved
🤖 AI Response:
   1. Precision: Vector databases can represent data with high precision without compromising the data quality.

2. Scalability: The objects in vector databases can be easily scaled up or down without losing data integrity.

3. Compactness: Vector databases require less storage space compared to raster databases, as they only store the points, lines, or polygons that represent a feature.

4. Rich Data Representation: Vector databases can represent complex geographical features such as rivers and roads more effectively than raster equivalents. They can readily handle both discrete objects (i.e., buildings) and abstract entities (i.e., sales territories).

5. Flexibility: They allow for more advanced spatial analysis and operations, such as calculating the shortest path or nearest neighbor.

6. Interactivity: Vector databases can facilitate user interactivity such as location searching and routing due to their inherent spatial indexing capability.
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embedding function saved
📊 Generated embeddings for 3 articles
   1. "Draft Article" - 0D vector
   2. "Getting Started with ekoDB" - 0D vector
   3. "Advanced Query Patterns" - 0D vector
⏱️  Execution time: 0ms

📝 Example 3: List All Articles

✅ Function saved
📊 Found 3 articles
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All AI examples completed!
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_api_user_js (TiB8E1myjANZshxpExwgTgdGGlnhsaTeEg_BoL9Ee_XK7FIxdEtwoPejS4ZztWMyIL0I2hUc0Q4QA7bUFkh2tg)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "value": {
          "website": "hildegard.org",
          "address": {
            "street": "Kulas Light",
            "zipcode": "92998-3874",
            "city": "Gwenborough",
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            },
            "suite": "Apt. 556"
          },
          "phone": "1-770-736-8031 x56442",
          "name": "Leanne Graham",
          "username": "Bret",
          "company": {
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets"
          },
          "email": "Sincere@april.biz",
          "id": 1
        },
        "type": "Object"
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
Response time: 8ms (served from cache)
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
✓ Created native SWR script: github_user_native (6SGQ47exyVaGriCTedRevlcozaqNKbKuS9h7Ua6dzuqCviDLNpErrpN4lefKK5TQ5LPoaaGjN3coEbnMTT4uDA)

First call (cache miss - will fetch from GitHub API):
  Response time: 4ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 8ms
  Speedup: 0.5x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (BR04u-5WtWXqGA1NR0eEa17Qf18xOEJmn7Nz75CrHX-cpMj8McKaoqIF6qIePKklzfRW0jyI-kzB27GufM_RzA)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (mX0c_p_Rcc8VxzNYIOYkmFzE2ZhwCLlkLpRryh2s4eozb3R0JNDHxnbBc_a-Oho75pjH_qSsBmrZW6vos8mMlw)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (rXYUPTKeJHPjJq8mRG16jCaX2vs_RbuqeBfDvxhT_EzEgnsmTtMFZuVHcnmNzyjgNS8G3nBeU0nRQZQ257TepA)
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
✓ Deleted 4 test functions

✅ All examples completed!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: 6-dVb9zuuQ9An4rAh8d3f88LWYm3GJklkG29YY9JMmUuTSZWzdzXltvguM9QwNExUVtuImrHvQwYSJcIcLSd2A

=== Sending Chat Message ===
Message ID: GVSsHHT-LtVq1_Ctr4MsozS4GPgxf4Q7fqxxyeq_gf1FjDm-5VHZWckTGiHYmlOy0Q-Wy5dgSuFJXf6-T8Tp-Q

=== AI Response ===
Here are the available products along with their prices:

1. **ekoDB Pro**: $299
   - Description: Enterprise edition product with advanced features

2. **ekoDB Cloud**: $499
   - Description: Fully managed cloud database service product

3. **ekoDB**: $99
   - Description: A high-performance database product with AI capabilities

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_js',
  record: {
    description: 'Enterprise edition product with advanced features',
    price: 299,
    id: '37909zGNsk4Qz18-jIEFlPIMsCNUXcVR5H0FytsP8nAfmNX2eJ3oNzr_jlCNsDOyNEFE17NSglxQ6duJl7qlLQ',
    name: 'ekoDB Pro'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_js',
  record: {
    name: 'ekoDB Cloud',
    price: 499,
    id: 'QRz3zyzYXMYszC1Zwoz2JX9NxnysSDXGq88RPovqE8OEfB8sy3ZJFEFX2rHnZuh4WaElSgwzAs2y8e91_HHjEA',
    description: 'Fully managed cloud database service product'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_js',
  record: {
    price: 99,
    id: 'AmXTDGLhDS9oBl0Qln6Nnq9UB2V4sT_vsFHUZO5zymutWbWltZKW22tvgc4AwhXqIe9ViEX58Z5I7JL_btWwuw',
    description: 'A high-performance database product with AI capabilities',
    name: 'ekoDB'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 1769ms

=== Token Usage ===
Prompt tokens: 3413
Completion tokens: 82
Total tokens: 3495

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: zM8LWo9PCwP2YyZR9ZAn97VqqxCLFoUPRH91tDBOLA2eG_7PUmLX5joXEhd1aRQpfB5pEBVnexy6QLVTHoPHDQ

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: High-performance database product
- **Price**: $99

If you need more information or additional products, let me know!

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
✓ Created second session: X8oeB_fKki_eLybnqkRR3M9ttpz8lgBNDBx2IC0ebUlyieP-qFO-8ONN-k8lRtU8iR6xbrUZaxNj7IWW7QIzpw
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 7

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 6

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: cAej9UWto-P6q4z_OjBPkvUGvdu2twaxnl-O1VxY2Ec5yoxcDg9DgIiOO0FLNjVYSdYtZ-J7DcntBpV_QI0JMQ

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is **ekoDB**, which is described as a high-performance database product. The price is **$99**.

✓ Message 2 sent
  Response: The price of **ekoDB** is **$99**.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 27v6n0iQgheMxUCm29oL6aXQuC0bB0RCZOF1CNu-gpj5taywmpJySUlkh0HDRp7IW_69sbpRU_gpY2RiCyyXgg
  Parent: cAej9UWto-P6q4z_OjBPkvUGvdu2twaxnl-O1VxY2Ec5yoxcDg9DgIiOO0FLNjVYSdYtZ-J7DcntBpV_QI0JMQ

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: 27v6n0iQgheMxUCm29oL6aXQuC0bB0RCZOF1CNu-gpj5taywmpJySUlkh0HDRp7IW_69sbpRU_gpY2RiCyyXgg (Untitled)
  Session 2: cAej9UWto-P6q4z_OjBPkvUGvdu2twaxnl-O1VxY2Ec5yoxcDg9DgIiOO0FLNjVYSdYtZ-J7DcntBpV_QI0JMQ (Untitled)
  Session 3: X8oeB_fKki_eLybnqkRR3M9ttpz8lgBNDBx2IC0ebUlyieP-qFO-8ONN-k8lRtU8iR6xbrUZaxNj7IWW7QIzpw (Untitled)
  Session 4: 6-dVb9zuuQ9An4rAh8d3f88LWYm3GJklkG29YY9JMmUuTSZWzdzXltvguM9QwNExUVtuImrHvQwYSJcIcLSd2A (Untitled)
  Session 5: 6RYvcxkccIYvET9JFSOmMNzfvbnZm6kzWHLQdDBVw3gjLQJNDXABj0lTKF6J4fUup43qcFsICDKLK1PWnBP_ow (Untitled)
  Session 6: B6IIPrlCRAJncGdiIVYwrU8vPsM-QTsUq6VGbpioL3_NKWXM4N_d9zRb_kUQMQlstoTx_q60VDE_1C-Mr15vhQ (Untitled)
  Session 7: sjclm-aZIqRQEaVKRC2CXdhXbzklEgeGdfCdlSgrAS5TbqDCfzNpGiRXszCUHMQnO2zG2AmHf1FG_UwX9fqSyg (Untitled)
  Session 8: 5hi3bOxBbTb2WcHkXfr7bCB4mS9f44BEJ-DAXVsqrF96OST0diI2NxRenbSDb4MEKRC9FbQwtn6_hP_p7xTt4w (Untitled)
  Session 9: Kh_2y1eqgZka3uqUF1ti21UPDhrWr12DBbgPg3rsoS8AX9-4AVxYHe8VZ1pIwPbRRRaEA6LYfPvIteEXjW8_yQ (Untitled)
  Session 10: EGq4BpwScMdsQDiBfksclFl8625dagxm3gVyxIyXJKhHxHZyinKJiOelY97gqIKri4o4BnJj2GVPWG8XuixU0Q (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: 27v6n0iQgheMxUCm29oL6aXQuC0bB0RCZOF1CNu-gpj5taywmpJySUlkh0HDRp7IW_69sbpRU_gpY2RiCyyXgg

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
✅ [32mJavaScript client examples complete![0m
