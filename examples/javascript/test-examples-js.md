make test-examples-javascript
🧪 [36mRunning JavaScript examples (direct HTTP/WebSocket)...[0m

added 1 package, removed 1 package, and audited 9 packages in 785ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
[34m
╔════════════════════════════════════════╗[0m
[34m║  ekoDB JavaScript Examples Test Suite ║[0m
[34m╚════════════════════════════════════════╝[0m
[34m
=== Checking Server Connection ===[0m
(node:63719) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
[32m✓ Server is ready[0m
[34m
=== Getting Authentication Token ===[0m
[32m✓ Authentication successful[0m
[33m
=== Running 10 Examples ===[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/simple_crud.js ===[0m
✓ Authentication successful

=== Insert Document ===
(node:63722) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
Inserted: {
  id: 'SYZ1qTh0Sw4yQLPMQSsB7XikiIzYeYf6a8ZK40BZQvc_xD8Mn_Il-TrIwhuOihtnF3-NXKBemUTnM2eE5Tf0gQ'
}

=== Find by ID ===
Found: {
  value: { value: 42, type: 'Integer' },
  active: { type: 'Boolean', value: true },
  id: 'SYZ1qTh0Sw4yQLPMQSsB7XikiIzYeYf6a8ZK40BZQvc_xD8Mn_Il-TrIwhuOihtnF3-NXKBemUTnM2eE5Tf0gQ',
  name: { type: 'String', value: 'Test Record' }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  value: { value: 100, type: 'Integer' },
  id: 'SYZ1qTh0Sw4yQLPMQSsB7XikiIzYeYf6a8ZK40BZQvc_xD8Mn_Il-TrIwhuOihtnF3-NXKBemUTnM2eE5Tf0gQ',
  active: { value: true, type: 'Boolean' },
  name: { type: 'String', value: 'Updated Record' }
}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
[32m✓ simple_crud.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/simple_websocket.js ===[0m
✓ Authentication successful

=== Inserting Test Data ===
(node:63750) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
✓ Inserted test record: Pa7R3PiGt3p4YlYpqhrD2eixvMIcEmIj35VogIHxpACw8ZSuFt-YKnbU3UVubX2Ul2LmPIVfDBCsvsdmtwvm4g

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
        "id": "FTsekewxgB5micEApFUENESf1T4V1PoJc7ytPFR4kyIC6AB3mCv18L8t7JYVDT6tkCGtqwz8jXgnOOsmcpkPkw",
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
        "id": "Pa7R3PiGt3p4YlYpqhrD2eixvMIcEmIj35VogIHxpACw8ZSuFt-YKnbU3UVubX2Ul2LmPIVfDBCsvsdmtwvm4g",
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
        "id": "v_o_VMTYzOkploU1BbN3TCNs9Kbc8H7BASVbe-rp30wX4gyOWAjC8jUwlFCsg2pqbeSVbXBiD-nK4oMNdpJJEg",
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
        "id": "um21JmrqMHTOaWiaJQsXKEO81RaRGKGHACyQBVWU8yNrbkOgM0K0tZKnikMWm0v1nnEdc0jxGdcOI5tZuN4HRA",
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
[32m✓ simple_websocket.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/batch_operations.js ===[0m
✓ Authentication successful

=== Batch Insert ===
(node:63753) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
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
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/kv_operations.js ===[0m
✓ Authentication successful

=== KV Set ===
(node:63771) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
✓ Set key: session:user123

=== KV Get ===
Retrieved value: {
  loginTime: '2026-01-03T23:55:55.634Z',
  userId: 123,
  username: 'john_doe'
}

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: { name: 'Product 1', price: 29.99 }
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
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/collection_management.js ===[0m
✓ Authentication successful

=== Create Collection (via insert) ===
(node:63774) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
Collection created with first record: N90CNCySk2MviTVtaHQStwtewhrTXm3OL6LCOWM1v67kWoX6vT1wJ00ryA1UYYS-T_CYiJkyWxs0bVYzLCKDKA

=== List Collections ===
Total collections: 23
Sample collections: [
  'chat_configurations__ek0_testing',
  'ttl_cache',
  'schema_documents_client_py',
  'schema_products_client_go',
  'schema_users_client_go'
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
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/document_ttl.js ===[0m
✓ Authentication successful

=== Insert Document with TTL (1 hour) ===
(node:63781) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
✓ Inserted document: TbyXfjzonHNqpJSuKgdjN_8_0-QA6mTWppnrFBdAQoMJhlzqIclckVckgiKh7k7Rp3hR9bV5Z27Wf41TNth3vQ

=== Insert Document with TTL (5 minutes - integer) ===
✓ Inserted document: created

=== Insert Document with TTL (30 minutes - duration string) ===
✓ Inserted document with duration string TTL: TBRjQKeo82t7U4XdDkspbtYrO7YqNxC2yN6Wa07golFN1vP3Gk1P11YRpe5r-lrlDa2hXwCOM4IomKLsjBxAjA

=== Query Documents ===
✓ Found 7 documents with TTL

=== Update Document ===
✓ Updated document

=== Delete Document ===
✓ Deleted document

✓ All document TTL operations completed successfully
[32m✓ document_ttl.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/websocket_ttl.js ===[0m
✓ Authentication successful

=== Insert Test Data with TTL ===
(node:63794) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
✓ Inserted document with TTL: yUlYb72KZdbfjdjp7Rs7ATc5SFuQVDCU7hAq8D2I4WMsZps1D3IDV-XS9DFNb5vdzWtsgZN0EZsmp3OsgiBO-A

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 4 record(s) via WebSocket

✓ WebSocket TTL example completed successfully
[32m✓ websocket_ttl.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/http_functions.js ===[0m
🚀 ekoDB Scripts Example (JavaScript/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: 9a2aJf8FbjxrN5s1rqPIfCa7NOpMG6DKCyVmUvDGChL9iSi9P6JpXb3IHmWGG6lMgjL_LHV05WycBzDHYwGL3w
📊 Found 20 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: muMcH8W4Z1VEbEKAlortntWmyk_WMsxUzAoT2DFnBq2SRWJDvQ90Sq1d2SKzhaRuSItU2begWOQIkbRGbR8BYw
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: L6ng3kPi6HgMTjRuSvBinF_rnVavucBXQgOLWoGsOrlF20XvPDOTNycuO8gEIiJaKOKoBIMXQ4wd9De90MUh_Q
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"status":"inactive","count":10,"avg_score":50,"max_score":90}
   {"count":10,"avg_score":60,"status":"active","max_score":100}

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
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/transactions.js ===[0m
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: O1zTjj8jE1m4qYXCGY8k2JRHs8dvw20GAnmAdeanlZUdhh4ycasXeNBph9l9-gvXCautzZ6SbY_cA4UvP3ybNg
Created Bob: $500 - ID: rK7g8fMB2xYMhKJQCwUwv38VdWH8EcRKk33EJgG5JYk3Hd0RknBuiVaozUfSuUFnygY473JvpjSDFdTTCz_fLQ

=== Example 1: Begin Transaction ===
Transaction ID: 09b38310-9548-4e45-aca8-8067c5d9eaec

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
New transaction: a1c3c9d2-537e-452a-afda-068bf39570c5
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
   ✅ Script saved: tVdexwN51KiAsI9PODtABctJrxz4jPR8-6HL2sdLw1-_Bovhe-YZAGPd5Ol413GIJ5z8BsYSyHi0iUrawqYurg

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
   ✅ Script saved: tO8cqTKmx5y8CvhFevzRiSJX7BBzrOL74vve6fd0PF5IENVyqbZAFptFzVEHudgVwsW9roFokKuTCXsIP3IMoQ

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
   ✅ Script saved: vS77pK3n_MjnZToV9jtJ1SCL1yYTmJoZ65H_vOMMe0YAn1fXAghWQPqxSQk0FzOLVsWb9izdCYPWRcpETj2-7g

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
   ✅ Script saved: 7ZsLnO6AjMHOhKl9fFGHFjIkEuKdN1YI5MscygY7monRdZKu8DXcGjqyMyg8inv7Qyfym_sjh7RErSFCW34EQQ

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: tVdexwN51KiAsI9PODtA...
   ✅ Deleted script: tO8cqTKmx5y8CvhFevzR...
   ✅ Deleted script: vS77pK3n_MjnZToV9jtJ...
   ✅ Deleted script: 7ZsLnO6AjMHOhKl9fFGH...
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
╔════════════════════════════════════════╗[0m
[34m║           Test Summary                 ║[0m
[34m╚════════════════════════════════════════╝[0m
[34mTotal: 10[0m
[32mPassed: 10[0m
[32mFailed: 0[0m
✅ [32mJavaScript direct examples complete![0m
📦 [36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.7.0 prepare
> npm run build


> @ekodb/ekodb-client@0.7.0 build
> tsc


up to date, audited 58 packages in 1s

15 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.7.0 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning JavaScript client library examples...[0m

added 1 package, removed 1 package, and audited 9 packages in 818ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'bW5kJIQriWHsPlRmQh32eRMVAsgq7SeyGCkC5VK2mOf3aEGhCB7lGnTm1pUxFHe2gXbCFBpP2z714fdiPrc8LQ'
}

=== Find by ID ===
Found: {
  value: { value: 42, type: 'Integer' },
  id: 'bW5kJIQriWHsPlRmQh32eRMVAsgq7SeyGCkC5VK2mOf3aEGhCB7lGnTm1pUxFHe2gXbCFBpP2z714fdiPrc8LQ',
  name: { value: 'Test Record', type: 'String' },
  active: { value: true, type: 'Boolean' }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  value: { type: 'Integer', value: 100 },
  id: 'bW5kJIQriWHsPlRmQh32eRMVAsgq7SeyGCkC5VK2mOf3aEGhCB7lGnTm1pUxFHe2gXbCFBpP2z714fdiPrc8LQ',
  active: { type: 'Boolean', value: true },
  name: { value: 'Updated Record', type: 'String' }
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
Retrieved value: { userId: 123, username: 'john_doe' }

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
Total keys in store: 4

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: false

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: _YenXnTXAsZ4EKQdAP-iCK5EA6Ijpqik5waHkS-RfkqQf4gDdiIiOhIBSMwvnRjhgemJIvuhJSJani1Td_1sMQ

=== List Collections ===
Total collections: 22
Sample collections: chat_configurations__ek0_testing,ttl_cache,schema_documents_client_py,schema_users_client_go,schema_products_client_go

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: RjBpRSQRVYXh4hmPfyc1bekbLPBXswv_-FPT_7qH-VtW4FCiJ2yr5BJwMQgjlNahQfcDOLjlYAXZ2NSWjpZ0iQ

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: Ca2sdieU5CQuoDopdENeFPJKrXb_jh80ikNbaGfT24xEk7kxfRcBrwd0Utsram_WuSrXrtfUN4G9l9tPRgxx4g

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
✓ Inserted test record: kJvaulA8lBQmtH1JXM5fCyTQg5ub6wFPVFdkOR7tw0dklxh-BK-Dken14Q_-GchLbchIsklp0mA2F8fwjWiYyA

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
✓ Inserted document with TTL: 7ukUmLOxAUeo7EMFuWFWiM8Ugj5OmHuQSnPveC0p4JZEMND-2JUgz6kDRq0h3rNnQUR3anTlT-SYGvdS8kavag

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

✅ Script saved: vGRqsGmi1VPmRx2_Ta8VEL4YCdxlmXDcNa7rHgxlfQirO1PDoSWla3zM1fFN_CGL8YKOeCYh16RVTyhioKkUJw
📊 Found 10 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: X8Q0THXGLI1GED_Q4r4C9O6XV6iU-bwbJPlijvo4gY_ep3SEblCGiE9qhooT0znkdO2HEDOoWw9Cbm_NCYNpNQ
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: LVVWpO--8b5hTmE2EOtmOtAr-i3xnkrMMZu2bptAmP7doE-PWmCF9243uKJzzNncWJ-BcKhV0FBVWn2xiVmqXg
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"status":"active","count":5,"avg_score":60,"max_score":100}
   {"max_score":90,"status":"inactive","count":5,"avg_score":50}

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

✅ Script saved: A94XyKzBhWTC5J3mo4_UFgSdusCeL7zi2_RIn7e1a65TFFfyPMbh9K6TMHy8EV6PU5-tR4xdSzTf3MIynmN_tA
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
   {"status":"active","count":10,"avg_score":60}
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
   1. Laptop Pro - $1299 (⭐4.8)
   2. Monitor 27" - $399 (⭐4.6)
   3. Bookshelf - $149 (⭐4.1)
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
   user: 70.0 (7 users)
   admin: 20.0 (3 users)
⏱️  Execution time: 0ms

📝 Example 4: Top Users by Score

✅ Script saved
📊 Users (showing first 5 of 0):
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
   1. Introduction to Machine Learning (AI)
   2. Database Design Principles (Database)
   3. Natural Language Processing (AI)
   4. Getting Started with ekoDB (Database)
   5. Vector Databases Explained (Database)
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
   1. Introduction to Machine Learning
   2. Database Design Principles
   3. Natural Language Processing
   4. Getting Started with ekoDB
   5. Vector Databases Explained
⏱️  Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Script saved
📊 Projected documents (showing first 3):
   1. Introduction to Machine Learning
   2. Database Design Principles
   3. Natural Language Processing
⏱️  Execution time: 0ms

📝 Example 5: All Document Fields

✅ Script saved
📊 All documents (5 total, showing first 2):
   1. Introduction to Machine Learning (AI)
   2. Database Design Principles (Database)
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
   1. Scalability: Vector databases can handle large amounts of data and scale as needed.
2. Speed: They provide fast data retrieval and processing capabilities.
3. Flexibility: They can handle both structured and unstructured data.
4. Accuracy: Vector databases allow for precise computations on spatial data.
5. Real-Time Analysis: They can perform real-time spatial analyses and operations.
6. Storage Efficiency: Vector data requires less storage space compared to raster data.
7. Detail Preservation: They preserve details even when zoomed in, unlike raster databases.
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embedding script saved
📊 Generated embeddings for 3 articles
   1. "Advanced Query Patterns" - 1536D vector
   2. "Draft Article" - 1536D vector
   3. "Getting Started with ekoDB" - 1536D vector
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
Created Alice: $1000 - ID: cUFPmpy2FkbzwNgpTQw21bm7Vr1OKboLc7qcH_VqDLBxGp8sxB2JVfhdFfVcMuZJLEw7bOX4k7uSM3eXCaw02Q
Created Bob: $500 - ID: 1fUT3FhQ8kSJlJBhXoBNIgtIOn46zwlfaPGlh27x4zTCW_Yf_oPu7hzPchGMDpk2FtEWX-cJrnznyNigIs-FyQ

=== Example 1: Begin Transaction ===
Transaction ID: 73366db5-7624-49c1-a52d-eb295ed7e0fe

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 9edcc8ef-0258-4573-9dde-43170feefe0e
Status before rollback: Active
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
✅ [32mJavaScript client examples complete![0m
