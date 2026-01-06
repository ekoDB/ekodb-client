make test-examples-javascript
🧪 [36mRunning JavaScript examples (direct HTTP/WebSocket)...[0m

added 1 package, removed 1 package, and audited 9 packages in 839ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
[34m
╔════════════════════════════════════════╗[0m
[34m║  ekoDB JavaScript Examples Test Suite ║[0m
[34m╚════════════════════════════════════════╝[0m
[34m
=== Checking Server Connection ===[0m
(node:46690) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
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
  id: '6O_JOopTOadlVOLXNjB8GEF4w74YyoEvihvUMf_z0cyuQ9eRcrScdaIlSZwiTHy_LOsBJU4e9kF22CCXv89u6g'
}

=== Find by ID ===
Found: {
  name: { value: 'Test Record', type: 'String' },
  active: { value: true, type: 'Boolean' },
  value: { value: 42, type: 'Integer' },
  id: '6O_JOopTOadlVOLXNjB8GEF4w74YyoEvihvUMf_z0cyuQ9eRcrScdaIlSZwiTHy_LOsBJU4e9kF22CCXv89u6g'
}

=== Find with Query ===
Found documents: [
  {
    active: { value: true, type: 'Boolean' },
    id: '6O_JOopTOadlVOLXNjB8GEF4w74YyoEvihvUMf_z0cyuQ9eRcrScdaIlSZwiTHy_LOsBJU4e9kF22CCXv89u6g',
    name: { type: 'String', value: 'Test Record' },
    value: { type: 'Integer', value: 42 }
  }
]

=== Update Document ===
Updated: {
  id: '6O_JOopTOadlVOLXNjB8GEF4w74YyoEvihvUMf_z0cyuQ9eRcrScdaIlSZwiTHy_LOsBJU4e9kF22CCXv89u6g',
  active: { value: true, type: 'Boolean' },
  name: { type: 'String', value: 'Updated Record' },
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
✓ Inserted test record: 0qSV5P8zfDGeaCJNHEoNmqH5U7p22lSpMlkdHQ1VVKOYpqerHRO6pQBLCQGkoRKe1c1csOal0o8GYy9EJdRHnQ

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
        "id": "5yKdWQgJ0eshje0NLVcqSLxJGHZEoxIMmSlenatXgi9XL08hgn5_57TrAMXSKiD0F8B_VHoOI3uK8BOuAr4dVQ",
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
        "id": "0qSV5P8zfDGeaCJNHEoNmqH5U7p22lSpMlkdHQ1VVKOYpqerHRO6pQBLCQGkoRKe1c1csOal0o8GYy9EJdRHnQ",
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
        "id": "NijRrca-9YBQBeXU6j280I4rPcUwOpFnrXCbwAWzKK-LMaIattQNOpF3zXfRLzEbyzyKyf4XMcJ6kC1iBfqmHw",
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
        "id": "wOhw36APY7lqulIUv8F8sgCT5KKowLEVn1StWe5ITSbhiGFihQjS0yKngcE0-d6Pqs0oe1bokPr8hlOsg3kr1w",
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

✅ Script saved: bXRmxtZ2u9BnqkiVJvHRTpu5L3EN6wnp4yf7zkaodiRJLaz8wpsul4pMOB0WstVN459BWHawqNdM73fzAwrqlg
📊 Found 30 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: 0vsEuLQ19I7h6lVDbrxOb5iMiCaDWnKBtF0LPn6bH3e5s7xFoT8iaT4Qx7WSyfnAPtKw4ohaudptsyVYQ-DYYw
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: ZJ0_IJDOpwGXksNMRctZ_avpC48byXxpUcdSFTZiQtAmvntk2AIXoc0PN0AeqZ3WufSEgvuDR52NW6UtzYmCNA
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"status":"active","count":10,"max_score":100,"avg_score":60}
   {"avg_score":50,"status":"inactive","max_score":90,"count":10}

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
Collection created with first record: pFVo_vBM-dldzNueqyii0Z6MS8CQCeF9BWBo4vw45KQHAwmtzJny8zdhtHo6-D-7DRtK4rt7YW-x8kWj-2Xdeg

=== List Collections ===
Total collections: 27
Sample collections: [
  'schema_products_client_py',
  'batch_users',
  'edge_cache',
  'ws_ttl_test',
  'schema_employees_client_ts'
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
Created Alice: $1000 - ID: EoP9mwolhGBSGAFj2GxzoShPtTwJABgoKqO5UJi3I1JSqN-1zS_sOnO26r9-y_kkgqvvs38ohzi-vwFVo0bm2g
Created Bob: $500 - ID: 2-CYZ6rYrrTjuRyoTo1mq-ubvgd8N8NwPQ-qwfHLUtyzHV_p_MB18MVhIGF0CqUaiSWjfF87_C6oT0KdISqirA

=== Example 1: Begin Transaction ===
Transaction ID: 32238d4f-13df-4081-b41b-91c3237812be

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
New transaction: 37b07649-9ffe-47ee-bede-9cf27a5da51d
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
   ✅ Script saved: ERIPGjwmPV_X2_5JzgCM0ipUB_90btFUH3Mfok9WxvzPLVnUzudeRzsjVwY_ip632aG8fMzjt5jmVETN1lm8SQ

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
   ✅ Script saved: yo0gPJshz_O1sVRhXg3_2ErCUclDcvQQZmLFTHrd4dGXjfw2svaXY7q6p9sOf-5bSbJFlmNmZpVa-CTfsXJvBQ

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
   ✅ Script saved: ZAm26s0szdeQKNx3XeumqvBIAbalpuS87nCWhSUtVktSPNRAEpEk8pylOK3iymcRYyOLfc-vA0lhErxSKVMoCQ

2️⃣ Calling Script (Query + Update Credits + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {"type":"Integer","value":0}
   📋 Status: {"value":"active","type":"String"}
   📋 Name: {"value":"Alice Smith","type":"String"}

============================================================
📝 Script 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: Nv11yEfLIHlzJh9nRibhSNQVS9rKhZFWpSrY8F3xwOLZTldcNLiMsbVQmMTuoa8AQ-TBO8EJEyYeqtvYw1OSzA

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: ERIPGjwmPV_X2_5JzgCM...
   ✅ Deleted script: yo0gPJshz_O1sVRhXg3_...
   ✅ Deleted script: ZAm26s0szdeQKNx3Xeum...
   ✅ Deleted script: Nv11yEfLIHlzJh9nRibh...
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
  Output: Document ID = Fyfl5CYAj6aDI-Cj_RBBy3HvVSRJhDoIiGRM-ioexKFLb0iCpkWGYup0Xx8euKd-OtsVsm7NxDzFb9pSA6c5_w
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: findById(Fyfl5CYAj6aDI-Cj_RBBy3HvVSRJhDoIiGRM-ioexKFLb0iCpkWGYup0Xx8euKd-OtsVsm7NxDzFb9pSA6c5_w)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: findById(Fyfl5CYAj6aDI-Cj_RBBy3HvVSRJhDoIiGRM-ioexKFLb0iCpkWGYup0Xx8euKd-OtsVsm7NxDzFb9pSA6c5_w)
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
  Output: Document ID = icddADD8h2x03_Y3oOXYIDrYaoreaCwlFDEuvvTXrFBbaQQyJ0zHne-j-LxRN9pK5JvoCiykoAVC0HIA18O3sQ
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: findById(icddADD8h2x03_Y3oOXYIDrYaoreaCwlFDEuvvTXrFBbaQQyJ0zHne-j-LxRN9pK5JvoCiykoAVC0HIA18O3sQ)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: findById(icddADD8h2x03_Y3oOXYIDrYaoreaCwlFDEuvvTXrFBbaQQyJ0zHne-j-LxRN9pK5JvoCiykoAVC0HIA18O3sQ)
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

added 1 package, removed 1 package, and audited 9 packages in 820ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: '-7KkEgCLh462UpHQk1QHVGptM52VLdvyT_5eawQWQGmxueHbjiZ_K_R7SCE8lGQZ9HTZ97x1XXTFX7rO_ahJnQ'
}

=== Find by ID ===
Found: {
  value: { type: 'Integer', value: 42 },
  name: { value: 'Test Record', type: 'String' },
  active: { value: true, type: 'Boolean' },
  id: '-7KkEgCLh462UpHQk1QHVGptM52VLdvyT_5eawQWQGmxueHbjiZ_K_R7SCE8lGQZ9HTZ97x1XXTFX7rO_ahJnQ'
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  value: { type: 'Integer', value: 100 },
  id: '-7KkEgCLh462UpHQk1QHVGptM52VLdvyT_5eawQWQGmxueHbjiZ_K_R7SCE8lGQZ9HTZ97x1XXTFX7rO_ahJnQ',
  name: { value: 'Updated Record', type: 'String' },
  active: { value: true, type: 'Boolean' }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: eGcuOIrfm984P1ZmOMIQiJ92gZRubLFFfuxAzwzzqi4Cs1l3j8gm4tKh6pzyTu-jnSZogJnbWmVkZ1k8Gx0nJA

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
Collection created with first record: MCsjLO24D5fhOjv7HEC-LnmEEquLozjq5QyRDHv-YCJWiyi0T8IO1tC_a9bhV1GGbDUCmALL8nZ2wqEVAv5NtQ

=== List Collections ===
Total collections: 26
Sample collections: schema_products_client_py,batch_users,edge_cache,client_collection_management_js,ws_ttl_test

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
cache:product:2: { price: 39.989999999999995, name: 'Product 2' }
cache:product:3: { price: 49.989999999999995, name: 'Product 3' }

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 0 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 6

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: false

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: bXA9so30w0NsfBoCentwz3cAXLY0B7CvFto4ZQQobg23hWMA4hz1e28f9k2juj_9_uKKwBlY5TKxnV4JAsp38Q
Created Bob: $500 - ID: GHSYB56vR6K_JbZqIh1mqQfWVExB0yIUzChcaLxo9KMTek90bvGOrc-6TfYmyDV21cNyvBryuspHR2jPLWpBbQ

=== Example 1: Begin Transaction ===
Transaction ID: ff66fcea-5ced-4892-82c0-a710c6306414

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: e3665c6c-9f72-4c77-8490-30fe151eba2e
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
  1. Score: 26.400, Matched: bio.value, title, title.value, bio
  2. Score: 26.400, Matched: title, bio, title.value, bio.value
  3. Score: 26.400, Matched: bio.value, bio, title.value, title
  4. Score: 26.400, Matched: bio, title, title.value, bio.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title, bio, bio.value, title.value
  2. Score: 39.600, Matched: title, title.value, bio, bio.value
  3. Score: 39.600, Matched: title, title.value, bio, bio.value
  4. Score: 39.600, Matched: title.value, bio, title, bio.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio.value, bio

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.785, Matched: 
  2. Score: 0.778, Matched: 
  3. Score: 0.743, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.714, Matched: title.value, content.value, title, content
  2. Score: 1.497, Matched: title.value, title, content, content.value
  3. Score: 0.311, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: skills, skills.value, bio.value, bio

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
✓ Inserted document: fN9XvDPTi-OudxlbdpqjmrYzTnX1l5VSguE-fAv28hy5MobiGPvMpUo1KmNA-4hzli6desTihqF7QMCpnwN8jA

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: 02A0dH7_VzwuX1Uq-YBJUQrSru4l_IiZvw7hxYvu04t0fL0-z01onYYVWiM6gsWVtcray2gcd-jQ2TgoFp7Z-w

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
✓ Inserted document with TTL: 7oRTnXdjRqd52329OolXkR4Aff23n5drVijpz85saEwk9ifaSmj58CtLxuNL-ewp6z3q8A4k7VlEHwOlnLXA9w

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
✓ Edge cache script created: igYtXlYIazbJ607SEuWPCHUNQZe_LihEEgaNM1HvHMs8rIRr30VX405gx2ybj1Nd1DvohKd0plKUsVnXShhv0Q

Call 1: Cache miss (fetches from API)
Response time: 549ms
Result: {
  "records": [
    {
      "data": {
        "type": "Object",
        "value": {
          "generationtime_ms": 0.028848648071289062,
          "timezone": "GMT",
          "utc_offset_seconds": 0,
          "longitude": -73.99309,
          "current_units": {
            "time": "iso8601",
            "interval": "seconds",
            "temperature_2m": "°C"
          },
          "elevation": 32,
          "timezone_abbreviation": "GMT",
          "current": {
            "temperature_2m": 5.1,
            "time": "2026-01-06T16:00",
            "interval": 900
          },
          "latitude": 40.710335
        }
      },
      "cached_at": "2026-01-06T16:13:03.744Z",
      "id": "eat5wDHaHufQMzEPx6CLGk0Na0OV12ovOGLhwDk2-zNe29PWTOuRVsjELbaYnzPyDB2KY3AHTWiG_GGSF5urgA"
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
Response time: 594ms (0.9x faster!)
Result: {
  "records": [
    {
      "id": "94LBJN1WBZsprioJKgy8JAKc8nxaHKXNtBnYfs9f3UhTqDE8agdTCbwzf0ez10X035JMUYM6IVT6km-wMBUlQQ",
      "data": {
        "type": "Object",
        "value": {
          "timezone": "GMT",
          "generationtime_ms": 0.024557113647460938,
          "current": {
            "temperature_2m": 5.1,
            "interval": 900,
            "time": "2026-01-06T16:00"
          },
          "current_units": {
            "temperature_2m": "°C",
            "time": "iso8601",
            "interval": "seconds"
          },
          "elevation": 32,
          "latitude": 40.710335,
          "longitude": -73.99309,
          "utc_offset_seconds": 0,
          "timezone_abbreviation": "GMT"
        }
      },
      "cached_at": "2026-01-06T16:13:03.744Z"
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

✅ Script saved: rrOqnW6-sSTGqNsMv3zZmDsjIIpH5tjQf4H1H9j4WJcnoJSfxL-DKh49cluvMkvVzSAQOXK8V8n1oiU3Oks8UQ
📊 Found 10 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved
📊 Found 10 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved
📊 Statistics: 2 groups
   {"count":5,"avg_score":50,"status":"inactive"}
   {"avg_score":60,"count":5,"status":"active"}
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
   Name: {"type":"String","value":"User 1"}
   Department: {"type":"String","value":"engineering"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using CallFunction to replace inline logic in SWR pattern...

✅ Saved reusable function: fetch_and_store_github
✅ Saved SWR function using composition: swr_github_user

First call (cache miss - will fetch from GitHub):
   ⏱️  Duration: 141ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 91ms
   📊 Records: 1
   🚀 Cache speedup: 1.5x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

✅ Level 1 function: validate_user
✅ Level 2 function: fetch_slim_user (calls validate_user)
✅ Level 3 function: count_validated_user (calls fetch_slim_user)

📊 Result from 3-level nested composition:
   Records: 1
   Name: N/A
   Department: N/A
   Record count: 1

🎯 Key Benefit: Each function is independently testable and reusable!
   - validate_user: Used in 100 different workflows
   - fetch_slim_user: Used in 50 workflows
   - count_validated_user: Specific workflow


✅ All composition examples completed!
🚀 ekoDB JavaScript Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved: iLhI-HDerNCzv62VdQkr6294UtFTjxNN-plrN_jjoJvDgvRV-MmQ6VGa5nH0qlk4HWS0McOZ2GMVpd4pmY4cug
📊 Found 2 product groups
   {"count":3,"avg_price":575.6666666666666,"category":"Electronics"}
   {"category":"Furniture","count":2,"avg_price":474}
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

✅ Inserted order: V6zQNuh_YmROvez5xErXBdry37eRH0bYJCBktQRgsEizG587YnsVr_CoJ3BJUDBfv4JVysLi-KdwhGyVlqdjqg
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: RBBTMepn2wT6XocKHtVCZ2aupsE9eCltpiXfeNJqyMJgecrkVUaYmZ7EDS5-wjvJHB9HU4UsLjL3_HidiEfAyQ
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"role":"admin","userId":"user_abc"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: mIy4R9LyMfw1o_qSiKjI0H5b0ytaHwZmmu39yUEXVGINkmG_N7fSYXm8jMQvjQURpMN8tzKbC263QKoo0zo1FA
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Script

✅ Script saved: 5QXnVx7FRjVgOVKtPnDD9RcCIIT1D_UZF0eLEp_eGY5sXMRIJpW2RaVqD3u-_7x3Ue1G3XXXV8xTdykwAxeNqQ
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
   1. User 2 - Score: 20
   2. User 5 - Score: 50
   3. User 9 - Score: 90
   4. User 8 - Score: 80
   5. User 3 - Score: 30
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
   1. Vector Databases Explained (Database)
   2. Getting Started with ekoDB (Database)
   3. Database Design Principles (Database)
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
   1. Vector Databases Explained
   2. Getting Started with ekoDB
   3. Database Design Principles
   4. Introduction to Machine Learning
   5. Natural Language Processing
⏱️  Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Script saved
📊 Projected documents (showing first 3):
   1. Vector Databases Explained
   2. Getting Started with ekoDB
   3. Database Design Principles
⏱️  Execution time: 0ms

📝 Example 5: All Document Fields

✅ Script saved
📊 All documents (5 total, showing first 2):
   1. Vector Databases Explained (Database)
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
   1. Webcam HD - $119 (⭐4.5)
   2. Laptop Pro - $1299 (⭐4.8)
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
   Vector databases offer several benefits:

1. High Precision: They can represent data more accurately, as they use points, lines, and polygons.

2. Scalability: The quality of the data doesn't degrade when zooming in or out.

3. Data Representation: They can store complex geographical data like road networks, drainage systems, etc.

4. Attribute Storage: They can store a large amount of attribute data.

5. Analysis Capabilities: They support complex geographic operations like network analysis and topology.
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embedding script saved
📊 Generated embeddings for 3 articles
   1. "Getting Started with ekoDB" - 1536D vector
   2. "Advanced Query Patterns" - 1536D vector
   3. "Draft Article" - 1536D vector
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
✓ Created SWR script: fetch_github_user_js (aERTbukYdmMpCP-l0DOkpmSszvwyAkNoZs3EABPMht3OkBXiXRDom_6DBJi6VSM2HlM-lzgV3meznnayRIpp_g)

Step 2: First call - Cache miss, fetches from GitHub API
Result: {
  "records": [
    {
      "cached_at": "2026-01-06T16:13:13.302Z",
      "id": "ZuBNuxlOIKr0cuQbpt3DLDptQJyKl_PKCXpex4ZMSxzVWbLFEQAt2qLUBWJBOCgwzB36J0nTP2Ehc-12CgR8Cw",
      "data": {
        "type": "Object",
        "value": {
          "email": null,
          "following_url": "https://api.github.com/users/torvalds/following{/other_user}",
          "blog": "",
          "updated_at": "2025-11-24T04:16:14Z",
          "gravatar_id": "",
          "public_repos": 9,
          "bio": null,
          "public_gists": 1,
          "login": "torvalds",
          "starred_url": "https://api.github.com/users/torvalds/starred{/owner}{/repo}",
          "node_id": "MDQ6VXNlcjEwMjQwMjU=",
          "location": "Portland, OR",
          "twitter_username": null,
          "user_view_type": "public",
          "created_at": "2011-09-03T15:26:22Z",
          "hireable": null,
          "following": 0,
          "organizations_url": "https://api.github.com/users/torvalds/orgs",
          "name": "Linus Torvalds",
          "repos_url": "https://api.github.com/users/torvalds/repos",
          "url": "https://api.github.com/users/torvalds",
          "events_url": "https://api.github.com/users/torvalds/events{/privacy}",
          "gists_url": "https://api.github.com/users/torvalds/gists{/gist_id}",
          "received_events_url": "https://api.github.com/users/torvalds/received_events",
          "type": "User",
          "company": "Linux Foundation",
          "html_url": "https://github.com/torvalds",
          "id": 1024025,
          "subscriptions_url": "https://api.github.com/users/torvalds/subscriptions",
          "site_admin": false,
          "avatar_url": "https://avatars.githubusercontent.com/u/1024025?v=4",
          "followers": 270299,
          "followers_url": "https://api.github.com/users/torvalds/followers"
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
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 87ms (served from cache)
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
✓ Created session: pRivlIJEaoy3S6kCtqxMOuO2GXmR7rkWT8BsDj8KY3ILfqZb4ZERjVfvP5U1tBMtMStIsKls1_xGx9WfDYSb0w

=== Sending Chat Message ===
Message ID: 0YH4HQU1w3kwb_KNvRnE6ytorBfi13XXSVysqwrnplEoFXGsOw5XogSPJky0pFWyR2q3kiw80YoAVz6pJWBBSQ

=== AI Response ===
We have three products available:

1. ekoDB: This is a high-performance database product with AI capabilities. It's priced at $99.
2. ekoDB Cloud: This is a fully managed cloud database service product. It's priced at $499.
3. ekoDB Pro: This is our enterprise edition product with advanced features. It's priced at $299.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_js',
  record: {
    description: 'A high-performance database product with AI capabilities',
    price: 99,
    name: 'ekoDB',
    id: 'M6zBKUETuYcu8QVkEJ6naLWv2Ef1MgLeOcNabHcdpkzvXR5BAong79Y-2c0UIVtU8Z7Jpgbj-R_3sYLAE4lHtw'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_js',
  record: {
    id: 'z__YtF3bmS-PLWyy1phbPOOh7JuFTUwp2NUmtcYDtxLmUeOs_LwYg33CBLC9UTC3iQRW093PdaIp-7sXks48ZA',
    price: 499,
    description: 'Fully managed cloud database service product',
    name: 'ekoDB Cloud'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_js',
  record: {
    description: 'Enterprise edition product with advanced features',
    price: 299,
    name: 'ekoDB Pro',
    id: 'SMJxCE7CT1AsmFjvLG8h36EOHR95JYhitBEpUojjNbVPSOIxbSNJfT3D7lg5h-q2y5OnUspZXPvs622gUcKttw'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 3884ms

=== Token Usage ===
Prompt tokens: 447
Completion tokens: 78
Total tokens: 525

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: lKLUqic3BfSkc5G4dFHedhyhXRpVfOmIC8ZQExHz9eeUWY8hN1KMwobk1eIaE2G19jUq2kl2Dn026O36o3bwSw

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information provided, the product available is "ekoDB". It is a high-performance database product and is priced at $99.

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
✓ Created second session: CgafTDcw3gdix761YQ59PLeo7Z6twbp_I546rAhBXkIS8MnWhkGATSFX77x-_9va_3iVoTIbJT3VgT3T7gzZ8g
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
✓ Created session: 4Fo0DeaY1SJIMXfsb6JXjPqtIeiJ93jI0KM-53GLBR0nJZDMhrcLO9Qg902il80eA9u0JNTLKdOxFTul72GfQg

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information provided, the available product is "ekoDB". This is a high-performance database product and it costs $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: tfEgPxtOUjCzVSjKQI8fmJZOeRLYk3iD9PsyNTWzdeOKro2fjnRFj2HcdZJddw3y5ZN2d2ID8lFPHSybOr8Uxg
  Parent: 4Fo0DeaY1SJIMXfsb6JXjPqtIeiJ93jI0KM-53GLBR0nJZDMhrcLO9Qg902il80eA9u0JNTLKdOxFTul72GfQg

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: tfEgPxtOUjCzVSjKQI8fmJZOeRLYk3iD9PsyNTWzdeOKro2fjnRFj2HcdZJddw3y5ZN2d2ID8lFPHSybOr8Uxg (Untitled)
  Session 2: 4Fo0DeaY1SJIMXfsb6JXjPqtIeiJ93jI0KM-53GLBR0nJZDMhrcLO9Qg902il80eA9u0JNTLKdOxFTul72GfQg (Untitled)
  Session 3: CgafTDcw3gdix761YQ59PLeo7Z6twbp_I546rAhBXkIS8MnWhkGATSFX77x-_9va_3iVoTIbJT3VgT3T7gzZ8g (Untitled)
  Session 4: pRivlIJEaoy3S6kCtqxMOuO2GXmR7rkWT8BsDj8KY3ILfqZb4ZERjVfvP5U1tBMtMStIsKls1_xGx9WfDYSb0w (Untitled)
  Session 5: mokgXfm_deB8_nvB7m2cxF9e2-pijcG8V-souy39OJNjzlaLN_fLpWv_EUrNc5TCBYqoJB7Gako8ntad_ThuOQ (Untitled)
  Session 6: 9lzCblLUBkxRFffds-OVpG9c64OgZjfh7QP7knhbj8J8Zy-tt4xi24dUxNlQJojSy0TRk1HH_cqAGuZ5sFlChQ (Untitled)
  Session 7: 0OAikv6tJLqRDgUuWi9Pnl56j2x8xE7bzmwiLTgU9RMca2eAvrP-r175vrCL8NMFU-sg7NLECYq90SvuxK--_Q (Untitled)
  Session 8: hdaG0aO5VXgNNA9ja6-NDcERT20eKc8L-AlFO9_tsCHN8TU2CPV530YAWqRxBzziO26QIQc7jX6oPB2c88mMmg (Untitled)
  Session 9: EB3jEuzG-LxkynD65O1rXeJuvg8QIZkvVwPEHVz2k56v6PiRdnSL7oN61GeamNEYGSrhJJXdcEuHNcMobxBfPg (Untitled)
  Session 10: X9t5cttsehSZsRMNaQJaFNuP86HLbljgFO9jHItG45p3NzgjeeiOKF4tY97d975r8UT0y_xwUsC38ihAeBwYgg (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: tfEgPxtOUjCzVSjKQI8fmJZOeRLYk3iD9PsyNTWzdeOKro2fjnRFj2HcdZJddw3y5ZN2d2ID8lFPHSybOr8Uxg

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
✅ [32mJavaScript client examples complete![0m
