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
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/simple_crud.js ===[0m
=== Simple CRUD Operations (Direct HTTP) ===

✓ Authentication successful

=== Insert Document ===
Inserted: {
  id: '68RsFFg4PDZw61yRGhry78OmiWC15A4w0yj7b7tNDwEcWn7sqfrYvoQxvlEfZYVuNtC2Ub5bY6aOfyZ_LavOsg'
}

=== Find by ID ===
Found: {
  value: { value: 42, type: 'Integer' },
  active: { type: 'Boolean', value: true },
  id: '68RsFFg4PDZw61yRGhry78OmiWC15A4w0yj7b7tNDwEcWn7sqfrYvoQxvlEfZYVuNtC2Ub5bY6aOfyZ_LavOsg',
  name: { value: 'Test Record', type: 'String' }
}

=== Find with Query ===
Found documents: [
  {
    value: { value: 42, type: 'Integer' },
    active: { type: 'Boolean', value: true },
    name: { value: 'Test Record', type: 'String' },
    id: '68RsFFg4PDZw61yRGhry78OmiWC15A4w0yj7b7tNDwEcWn7sqfrYvoQxvlEfZYVuNtC2Ub5bY6aOfyZ_LavOsg'
  }
]

=== Update Document ===
Updated: {
  active: { type: 'Boolean', value: true },
  id: '68RsFFg4PDZw61yRGhry78OmiWC15A4w0yj7b7tNDwEcWn7sqfrYvoQxvlEfZYVuNtC2Ub5bY6aOfyZ_LavOsg',
  name: { value: 'Updated Record', type: 'String' },
  value: { value: 100, type: 'Integer' }
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
✓ Inserted test record: fM7U75HBUvz21G8xs5CQNOlfVLWS647YlVQPFcaFCiBH4DuyrnQ1wGTFv1bWNyUio14B-bukwpZ0kfnBrrZKkw

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
        "id": "fM7U75HBUvz21G8xs5CQNOlfVLWS647YlVQPFcaFCiBH4DuyrnQ1wGTFv1bWNyUio14B-bukwpZ0kfnBrrZKkw",
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
        "id": "TK5i5uaqNtko1YYe_kXla3BPxvg2gt_LIl0pxHi6cG19vVm3UATakK3fGFDLbgSnOOgirv_QUyEMfphQVHHmJQ",
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
        "id": "qki0j-AbPiqn-W9E9_Co8X9oaL0jTuJ8TS557tLD_PXUotvNSLVZyvEMADozG_Uoz8jqy69EW3PmfjChDGxv6Q",
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
        "id": "SDTaGPMYzo1KfRt5vvVBSBhcq6q8Cep6qW6Q6wcjVfBuNrUGM_pRYpZDs2OkMZtmDr0LARETfYf70575_ppGBQ",
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

✅ Script saved: h5KUT38iJXij1r8_B2yXy1GcDNsOnL9qpEN62HX58znr3BaNUx5jEmYAFp9c0eS6dUMW9tBS26paW68VELmWCg
📊 Found 10 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: I3rPOnSGVP3AMBVvCToNwvk88SSoD0uaiXwqAQfdvL9VOyF5dIsJScDCsvpw-eL8LLOrJ33DTZuQhCMT4BWdLw
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: v48BTrYA-bnpiX_SE4nzugKrp8j7dV3VJ49--vKy9MSVR_vfi0bXaEBHHdxu89HMQIFOtDdgpIUf9sabfQO8qw
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"avg_score":60,"count":10,"max_score":100,"status":"active"}
   {"max_score":90,"count":10,"avg_score":50,"status":"inactive"}

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
cache:product:1: { name: 'Product 1', price: 29.99 }
cache:product:2: { name: 'Product 2', price: 39.989999999999995 }
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
Collection created with first record: Ee4KzHYspd5lAwT7EqT1EiWSONsLTGohID2ClGrwippXkSYArClCUcwOypyIn6ImXtFeR3RchWIBj_wEL3_lOA

=== List Collections ===
Total collections: 25
Sample collections: [
  'schema_products_client_go',
  'schema_users_client_ts',
  'schema_employees_client_go',
  'schema_employees_client_ts',
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
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/transactions.js ===[0m
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: Ntx4gmP_E_p__KBorsEZH07_eMJUoSskmr1wcZyyEybQxF88O56urhBF_4DitMJHARPrnhMyBOCnrqZkVrujYw
Created Bob: $500 - ID: dWSYgtu_GoMz44u7AIjLsLCGpKW0ilwL3awH9hD0ZNKBYPziYHVoFGlEjI3ZgrmkfRazT8nU8l7HtC_MfMFmjA

=== Example 1: Begin Transaction ===
Transaction ID: 95b381dd-9a47-4f51-ae27-50f5dca10628

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
Bob: {"value":700,"type":"Integer"}

=== Example 5: Rollback ===
New transaction: ee65bb52-aead-4cc7-aefa-a1184039dcb9
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
   ✅ Script saved: UlXHU-OHJcVnqRAWml_SfHlbktN1JIVDqSp9WKNMnJhEzWWPKxyIQBX8DGEWESyHyrPKwVspV0_UVtbhifnusQ

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
   ✅ Script saved: TCr4w9hXRpu6Afa8C8EzNh9Vgc06UOZ0eQSGF1xnHuzemOqUZyEBKW61zZEY0JYbALgmHOfP-o_LUMLTQ_cUog

2️⃣ Calling Script (Query + Update + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Status updated to: {"type":"String","value":"active"}
   📋 Name: {"type":"String","value":"Alice Smith"}

============================================================
📝 Script 3: Query + Update Credits + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: oDTjnAZ0fxfmnVNhxSAqeUp8J2VflNK7aS9SsoIy_Ak7deaVmSD_GHSrLFwSi-cvMO1cxjXxnhRIez2TqH_-qQ

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
   ✅ Script saved: UivSFxZrgWWj922d5sykYjtLaFwi63IWSaGjd8pBOk578Z1cMTM4J9F4MiDkwq_IkT2dBm14V1tAlXrFS6utMA

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: UlXHU-OHJcVnqRAWml_S...
   ✅ Deleted script: TCr4w9hXRpu6Afa8C8Ez...
   ✅ Deleted script: oDTjnAZ0fxfmnVNhxSAq...
   ✅ Deleted script: UivSFxZrgWWj922d5syk...
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
  Output: Document ID = BNOjQf5wB1snHzFyqvPzO-vdHPN8evvUo2cd5FRxseqjwbk7gSd2sYzPxHPlMsJE6JVOVxHFbJZ2y-IO_v2AlA
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: findById(BNOjQf5wB1snHzFyqvPzO-vdHPN8evvUo2cd5FRxseqjwbk7gSd2sYzPxHPlMsJE6JVOVxHFbJZ2y-IO_v2AlA)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: findById(BNOjQf5wB1snHzFyqvPzO-vdHPN8evvUo2cd5FRxseqjwbk7gSd2sYzPxHPlMsJE6JVOVxHFbJZ2y-IO_v2AlA)
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
  Output: Document ID = 9poUJBIlF3F3xZx9a_AypeE4TsiBfx9N7JfJe3-WkDj5A4vKXg4ue68jJqKPdiewASHDVWJ7o3tVNKMRAOXXhA
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: findById(9poUJBIlF3F3xZx9a_AypeE4TsiBfx9N7JfJe3-WkDj5A4vKXg4ue68jJqKPdiewASHDVWJ7o3tVNKMRAOXXhA)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: findById(9poUJBIlF3F3xZx9a_AypeE4TsiBfx9N7JfJe3-WkDj5A4vKXg4ue68jJqKPdiewASHDVWJ7o3tVNKMRAOXXhA)
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

> @ekodb/ekodb-client@0.13.0 prepare
> npm run build


> @ekodb/ekodb-client@0.13.0 build
> tsc


up to date, audited 53 packages in 2s

17 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.13.0 build
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
  id: 'JUSXCYjS7H1SrZY7tAZLG-vRTNMBq0Dw3bwCtrhxKsUZUTmJWHTrFDAR6sCwV8jhOpx6UWXkMb1Rq8tvSicwtQ'
}

=== Find by ID ===
Found: {
  value: { type: 'Integer', value: 42 },
  name: { type: 'String', value: 'Test Record' },
  active: { type: 'Boolean', value: true },
  id: 'JUSXCYjS7H1SrZY7tAZLG-vRTNMBq0Dw3bwCtrhxKsUZUTmJWHTrFDAR6sCwV8jhOpx6UWXkMb1Rq8tvSicwtQ'
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  name: { type: 'String', value: 'Updated Record' },
  id: 'JUSXCYjS7H1SrZY7tAZLG-vRTNMBq0Dw3bwCtrhxKsUZUTmJWHTrFDAR6sCwV8jhOpx6UWXkMb1Rq8tvSicwtQ',
  active: { value: true, type: 'Boolean' },
  value: { value: 100, type: 'Integer' }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: Aay7oFzP49o00_SVllBYHlF69BOHirVzkwLkMj-lwVF7IBDNx4st47LvsjN4NRCrZsf1l6166VWySUHSzdMeQA

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
Collection created with first record: LfWEnOcGJSXh6FQBKM8aOkjWVNjawlxfuxW09m-VIv5MnCJGnj5A7V8N_bdnYt2d0ngGwK1v8yBO7uG_kSZbUQ

=== List Collections ===
Total collections: 24
Sample collections: schema_products_client_go,schema_users_client_ts,schema_employees_client_go,schema_employees_client_ts,test_collection

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
cache:product:1: { name: 'Product 1', price: 29.99 }
cache:product:2: { name: 'Product 2', price: 39.989999999999995 }
cache:product:3: { price: 49.989999999999995, name: 'Product 3' }

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
Created Alice: $1000 - ID: B541_BTyzxw9IXHfG4iCk5vPn9A9mG9kGY_M7FPl-DBrOZjeHmB4RH8y1BIDXt9mItBQFu2NcbWKJ1T9IRLqpA
Created Bob: $500 - ID: J2b4H2p5IhRl5C4LxoUUFLVX_OsfmeOQTVly7ADUAQYBBvuXRovWzDDexT5APwb8LxnsASqRD6RpKrAds6lkcg

=== Example 1: Begin Transaction ===
Transaction ID: f8bf9e70-1f3f-45f7-be25-6f8785142524

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 393c77ab-4869-4533-8683-f89cbe7d646d
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
  1. Score: 25.740, Matched: name.value, name, email, email.value
  2. Score: 12.540, Matched: name.value, name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title, bio.value, bio, title.value
  2. Score: 26.400, Matched: title, title.value, bio, bio.value
  3. Score: 26.400, Matched: bio.value, title.value, title, bio
  4. Score: 26.400, Matched: bio.value, bio, title, title.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title, bio.value, bio, title.value
  2. Score: 39.600, Matched: title.value, title, bio, bio.value
  3. Score: 39.600, Matched: title.value, title, bio.value, bio
  4. Score: 39.600, Matched: bio, title, bio.value, title.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio.value, bio

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.775, Matched: 
  2. Score: 0.770, Matched: 
  3. Score: 0.763, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.710, Matched: content, title, content.value, title.value
  2. Score: 1.508, Matched: content.value, title.value, title, content
  3. Score: 0.305, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: skills.value, skills, bio, bio.value

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
✓ Inserted document: HoGHB0-iD8NM6_XiclFnDC-WMcW0od-CZaV4GnmMfvOGIwhLYa7z2i1LYEASDDOyoxmwDZOjX-BmNrwtKMnLrQ

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: CbLkQBxNh04RqI17bHxV_MwpkvoVa2_EGmCJDxsvkKXcIJNT_EMeS1IeK2n-67uvuZrloEe51Ey27rX5Lt9rRQ

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
✓ Inserted document with TTL: 3xjEpb6ltZO76GM1e5u4fsh1qsy_qE-8D7ZWkd7KxBCCS-pXdu82oqM9J3JE3hDukEu0Trfbzs3yRhWCHUV7BQ

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
✓ Edge cache script created: YWVnCXNvSk8swbxUyU6maCfMw2qFbscGGJ9TD7DyrSogMa0x_a8SbNg6tqElxJ0H6fo8h3FYupEo3OhRUCZDkg

Call 1: Cache miss (fetches from API)
Response time: 4ms
Result: {
  "records": [
    {
      "value": {
        "current_units": {
          "temperature_2m": "°C",
          "interval": "seconds",
          "time": "iso8601"
        },
        "latitude": 40.710335,
        "timezone_abbreviation": "GMT",
        "utc_offset_seconds": 0,
        "timezone": "GMT",
        "elevation": 32,
        "longitude": -73.99308,
        "generationtime_ms": 0.020742416381835938,
        "current": {
          "time": "2026-03-20T05:00",
          "interval": 900,
          "temperature_2m": 2.2
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

Call 2: Cache hit (served from ekoDB)
Response time: 4ms (1x faster!)
Result: {
  "records": [
    {
      "value": {
        "current_units": {
          "temperature_2m": "°C",
          "interval": "seconds",
          "time": "iso8601"
        },
        "latitude": 40.710335,
        "timezone_abbreviation": "GMT",
        "utc_offset_seconds": 0,
        "timezone": "GMT",
        "elevation": 32,
        "longitude": -73.99308,
        "generationtime_ms": 0.020742416381835938,
        "current": {
          "time": "2026-03-20T05:00",
          "interval": 900,
          "temperature_2m": 2.2
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

🚀 ekoDB Scripts Example (JavaScript Client)

✅ Client initialized (token exchange automatic)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: RWZmU-h4XHwj7FUDzDpUcjazUlVdDC1mqXtOakc0-IsHpZq87N9y1tC0BoYlPhQqC-BlgMZ8OPDsxqUKpAFh6A
📊 Found 10 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved
📊 Found 10 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved
📊 Statistics: 2 groups
   {"count":5,"status":"inactive","avg_score":50}
   {"status":"active","count":5,"avg_score":60}
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
   ⏱️  Duration: 3ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "phone": "1-770-736-8031 x56442",
    "website": "hildegard.org",
    "email": "Sincere@april.biz",
    "company": {
      "bs": "harness real-time e-markets",
      "name": "Romagu...

Second call (cache hit - from cache):
   ⏱️  Duration: 5ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "phone": "1-770-736-8031 x56442",
    "website": "hildegard.org",
    "email": "Sincere@april.biz",
    "company": {
      "bs": "harness real-time e-markets",
      "name": "Romagu...
   🚀 Cache speedup: 0.6x faster!

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

✅ Script saved: UMU2FP0NvJatS_itKk0oySrEhLJ9en0cDwYGwzFRA05wD59PHzydxTibDjDOefR-O5q3LRolS5Z-zFYayrb8vA
📊 Found 2 product groups
   {"category":"Electronics","count":3,"avg_price":575.6666666666666}
   {"count":2,"avg_price":474,"category":"Furniture"}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved
📊 Found 2 categories
   {"category":"Electronics","count":3}
   {"count":2,"category":"Furniture"}
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

✅ Inserted order: gAoLPL5VdNQ2xWEc4kehZTFjekU3rClDXRVjmc6zWfP9xxtOY36yfDYFhpylZZvguuF17QS-HL27WCGSWM7L9w
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: vlo4Y2GWfzMXAIdMMBBUoczC4LALmu-uq34Cmsul3U_9TBOxfNnedr5EORcCalJJm6yDukLMorIwiRTfG9443A
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"role":"admin","userId":"user_abc"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: wbZEmgIvp8a00Z2g_XbZNE9WnLQJbNluCqWipwQAOfPuRRAAbcBwe5zPloXiJaEPgMuNcjhq7qBAsRuY-akNYQ
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Script

✅ Script saved: GKKVuZ0zOC8c5p_TnSEN99_6dQIjNOVFjxo-BUD1tTG9ZzHBM1RWmQCdMLNC4TYgujlqArtwD_8YgKNMvhYTOQ
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
   admin: 20.0 (3 users)
   user: 70.0 (7 users)
⏱️  Execution time: 0ms

📝 Example 4: Top Users by Score

✅ Script saved
📊 Users (showing first 5 of 10):
   1. User 5 - Score: 50
   2. User 10 - Score: 100
   3. User 1 - Score: 10
   4. User 4 - Score: 40
   5. User 7 - Score: 70
⏱️  Execution time: 0ms

📝 Example 5: User Summary Statistics

✅ Script saved
📊 User summary (2 groups):
   active users: 7 users, Total Score: 370
   inactive users: 3 users, Total Score: 180
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
   2. Introduction to Machine Learning (AI)
   3. Natural Language Processing (AI)
   4. Getting Started with ekoDB (Database)
   5. Vector Databases Explained (Database)
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
   2. Introduction to Machine Learning
   3. Natural Language Processing
   4. Getting Started with ekoDB
   5. Vector Databases Explained
⏱️  Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Script saved
📊 Projected documents (showing first 3):
   1. Database Design Principles
   2. Introduction to Machine Learning
   3. Natural Language Processing
⏱️  Execution time: 0ms

📝 Example 5: All Document Fields

✅ Script saved
📊 All documents (5 total, showing first 2):
   1. Database Design Principles (Database)
   2. Introduction to Machine Learning (AI)
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
   1. Bookshelf - $149 (⭐4.1)
   2. Wireless Mouse - $29 (⭐4.5)
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
   1. High Accuracy: Vector data provides a high level of accuracy because it precisely represents geographic features.

2. Scalability: Vector data can be easily scaled up or down without losing its quality. 

3. Less Space: As vector data is only stored as points, lines, and polygons, it often requires less storage space than raster data.

4. Flexibility: Vector data allows easy editing and manipulation of data like calculating distances and areas, overlaying intersections and connections, etc.

5. Detailed Analysis: Vector databases are well-suited for detailed analyses like network analysis or adjacency & connectivity studies. 

6. Better Representation: Vector model notably represents discrete data, allows representation of original data without generalization.
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
✓ Created SWR script: fetch_api_user_js (Yz3bTxGRdlK_khlCHb-aJvvZQCNfGknxe7tsThwoZlI7ME4fREe9AnnCiS5EqRsAyF5lsmPWiJjeMZar8P-HXg)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "phone": "1-770-736-8031 x56442",
        "address": {
          "street": "Kulas Light",
          "suite": "Apt. 556",
          "zipcode": "92998-3874",
          "city": "Gwenborough",
          "geo": {
            "lng": "81.1496",
            "lat": "-37.3159"
          }
        },
        "email": "Sincere@april.biz",
        "website": "hildegard.org",
        "id": 1,
        "company": {
          "catchPhrase": "Multi-layered client-server neural-net",
          "bs": "harness real-time e-markets",
          "name": "Romaguera-Crona"
        },
        "name": "Leanne Graham",
        "username": "Bret"
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
Response time: 4ms (served from cache)
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
✓ Created native SWR script: github_user_native (M1hOp1xH1_PLv0XvxmpXlXpgC_eoWmqB6F3omnqlwPbjAtL-yLNGL7lkzxawUEq9z1YtvlPbq3-Z2yLR8ENULg)

First call (cache miss - will fetch from GitHub API):
  Response time: 4ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 11ms
  Speedup: 0.4x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (IhvPk6fSvLfr4LrsGkbVZu1W2ksd80uGmVFF-fkx_VM1v70U74Isp5Z5hg9Ny-SSwPCM9wP3kvikB8ggxauoEA)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (8DP9B0Ka3uHqx2wx3QYFNI7uBXmLAi9CUUf8oLp5Vth_ZNeIvmyMrfO3ExkhdSoZpKvO5MQRCqf12eGeUWYSOA)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (F8-LNGrjKCfegm9SGlZ92OC1dx8o-yrkqS8TVR66CzGOMP_zWS8TEPyqgAds9UJArIfqskRZdBFD5iZpz7j5ww)
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
✓ Created session: RVrqoWVKWngWTFeo7VTFrFcseEqpKFNUbPJlvGuMPLAvEd797FSvJPUIMG9i3_F-WC4EdP2yU5qs6AHTWPcUdw

=== Sending Chat Message ===
Message ID: k8PdPeneGQo7QTVUfwoBSZNVTbweynW7waRzQnLhuQc7VY5CyTgt-377X--an824vd03W7gQiGqd5UNvO2HGpA

=== AI Response ===
Here are the available products along with their prices:

1. **ekoDB Cloud**
   - **Description:** Fully managed cloud database service product
   - **Price:** $499

2. **ekoDB**
   - **Description:** A high-performance database product with AI capabilities
   - **Price:** $99

3. **ekoDB Pro**
   - **Description:** Enterprise edition product with advanced features
   - **Price:** $299

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_js',
  record: {
    name: 'ekoDB Cloud',
    description: 'Fully managed cloud database service product',
    id: '6aeH1w5COGCbBAhSmGZhDas3mEG_coPkCPiE2nYLKawVgNbTSHpxmCcCq9OM_UUn9qgPJN1huQ-01Y_wEN4MEQ',
    price: 499
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_js',
  record: {
    name: 'ekoDB',
    price: 99,
    description: 'A high-performance database product with AI capabilities',
    id: '8017-f2qOj5AST3RyMyvvnhcCBIIoweT3eSQIDhrqTOGhJ13jTRS8w4MiOjSUCdG-kWoPVFR3jcttMR3Por19g'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_js',
  record: {
    price: 299,
    description: 'Enterprise edition product with advanced features',
    name: 'ekoDB Pro',
    id: '8V4Py_ZBFyKzjFsXFlT6GmOfU3UxUE4FlwAwfHcbQkhEz7cn6PW7dGf6wxb0Oket4clgxZJcnFrw22luaykE_g'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 4837ms

=== Token Usage ===
Prompt tokens: 695
Completion tokens: 97
Total tokens: 792

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: HvBVH-7UIyP-jKne33x1Je00O9XHUjmnyiHDnHX6KcR8a2yS-JEaE2J4NN5cuB7ieLslSjnR7NaD16e8cKCKzA

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: High-performance database product
- **Price**: $99

If you need more information or additional products, please let me know!

✓ Second message sent

=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of ekoDB is $99. If you have any other questions or need further assistance, feel free to ask!

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: vzDg0URnVquwsAqzWQ5gQRe0VyL9SkwyScDLK7ZxSEuU3TOpMvs8KL9WkSRSfynK0os2_mGQ5jWVhRP2luQ1HQ
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
✓ Created session: Xei0qOl24whOtiLvq1_t5KlI45nKe_kjBiNAYPLUUVDCZxthmSRAIdNQ27URB-UlPjm5JkaQFC2VEVqGgaXnjQ

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: A high-performance database product
- **Price**: $99

If you need more details or have any specific questions about the product, feel free to ask!

✓ Message 2 sent
  Response: The price of ekoDB is $99. If you have any more questions or need further information, feel free to ask!

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: lCPKNctnaYk3_EDKF4AStTWWbOtZlHPzxklz6XvduubncEdGcQAjco1V0js24ie_Ncvz_adCG32uUlheeBuLXQ
  Parent: Xei0qOl24whOtiLvq1_t5KlI45nKe_kjBiNAYPLUUVDCZxthmSRAIdNQ27URB-UlPjm5JkaQFC2VEVqGgaXnjQ

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: lCPKNctnaYk3_EDKF4AStTWWbOtZlHPzxklz6XvduubncEdGcQAjco1V0js24ie_Ncvz_adCG32uUlheeBuLXQ (Untitled)
  Session 2: Xei0qOl24whOtiLvq1_t5KlI45nKe_kjBiNAYPLUUVDCZxthmSRAIdNQ27URB-UlPjm5JkaQFC2VEVqGgaXnjQ (Untitled)
  Session 3: vzDg0URnVquwsAqzWQ5gQRe0VyL9SkwyScDLK7ZxSEuU3TOpMvs8KL9WkSRSfynK0os2_mGQ5jWVhRP2luQ1HQ (Untitled)
  Session 4: RVrqoWVKWngWTFeo7VTFrFcseEqpKFNUbPJlvGuMPLAvEd797FSvJPUIMG9i3_F-WC4EdP2yU5qs6AHTWPcUdw (Untitled)
  Session 5: Z6iT8PdRI7325tHWcZRmjKEEbOSXeIoG34CJu5wHNPpHa8dpRY5ybLoC77_ZkpVgZULKKZZ6eg3j4tGTNwtK7Q (Untitled)
  Session 6: Uz7UFkiZf7lX6l7txVycldEete_4ijtSoDBLZLSBlVy_E5BJQd_e1q23v8DhWe7dhkyGRgP-3vA9tBojVcUeSA (Untitled)
  Session 7: cJ7KFC3ReqCiB715pcWAmK8VGiiCAoL7HsEQBUB3lyq_E5ftp6sYb3PGz7-SsEqSQ2Xrmtbhgnsj7Mk_1w7fww (Untitled)
  Session 8: -5sh5e53RHkzuNWTMIzkIDupGIb2ADBO_bDvRcRqrnhJe10g_-L7XKkcFg2nbnllj3j0ZoKVE1PSQ5uwkOIx0Q (Untitled)
  Session 9: 7dAJgIqPXPqtMwr6bScgNj5of5zkTnymUSqX109dbdd8LjyBLlNym5b98ESExHEvFdD6dE081U2rDnPpdV3-NA (Untitled)
  Session 10: mEiOcs6Z4Ri0oBGSTbUwcnwZPY7MfDnxHYCfFunBxqZ48nxDLi1TXnVOOEzfv_VqgFWI_91PJjtoGLD_uDBlXQ (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: lCPKNctnaYk3_EDKF4AStTWWbOtZlHPzxklz6XvduubncEdGcQAjco1V0js24ie_Ncvz_adCG32uUlheeBuLXQ

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
✅ [32mJavaScript client examples complete![0m
