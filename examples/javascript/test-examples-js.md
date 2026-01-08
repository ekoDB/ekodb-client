make test-examples-javascript
🧪 [36mRunning JavaScript examples (direct HTTP/WebSocket)...[0m

added 1 package, removed 1 package, and audited 9 packages in 886ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
[34m
╔════════════════════════════════════════╗[0m
[34m║  ekoDB JavaScript Examples Test Suite ║[0m
[34m╚════════════════════════════════════════╝[0m
[34m
=== Checking Server Connection ===[0m
(node:95548) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
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
  id: 'hor6GYgljKTLEjnWLok5U6fNiGaEmk6Qqeal4fVV81iJBNqibEDR0Bjeg6syaJtZR0vy1pIlGE1OVnyvuW8lBQ'
}

=== Find by ID ===
Found: {
  name: { type: 'String', value: 'Test Record' },
  active: { type: 'Boolean', value: true },
  id: 'hor6GYgljKTLEjnWLok5U6fNiGaEmk6Qqeal4fVV81iJBNqibEDR0Bjeg6syaJtZR0vy1pIlGE1OVnyvuW8lBQ',
  value: { type: 'Integer', value: 42 }
}

=== Find with Query ===
Found documents: [
  {
    name: { type: 'String', value: 'Test Record' },
    active: { value: true, type: 'Boolean' },
    value: { type: 'Integer', value: 42 },
    id: 'hor6GYgljKTLEjnWLok5U6fNiGaEmk6Qqeal4fVV81iJBNqibEDR0Bjeg6syaJtZR0vy1pIlGE1OVnyvuW8lBQ'
  }
]

=== Update Document ===
Updated: {
  name: { type: 'String', value: 'Updated Record' },
  value: { type: 'Integer', value: 100 },
  active: { type: 'Boolean', value: true },
  id: 'hor6GYgljKTLEjnWLok5U6fNiGaEmk6Qqeal4fVV81iJBNqibEDR0Bjeg6syaJtZR0vy1pIlGE1OVnyvuW8lBQ'
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
✓ Inserted test record: 5XeckePG3ifHk0BCWRZJY023Xih7e5eoSehVkJ5N_bCBca9hhy8QkZ8L50TTu0p2lwACbvn_eHs6mgB5kVZxWg

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
        "id": "Zs3qWTavKG00RXxhyD2QgfYzarxwKWLRD0l-Ntq7pJzuemEz8rt7R6vn-qovKfdkYl6WscoRzN-P-H9kczDSYw",
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
        "id": "5XeckePG3ifHk0BCWRZJY023Xih7e5eoSehVkJ5N_bCBca9hhy8QkZ8L50TTu0p2lwACbvn_eHs6mgB5kVZxWg",
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
        "id": "SVlLPEcHJ7pHpasjs8WN-AIaQWFzZ5XxZP_Uy6uluLftzJgli8t6DcQqg8gKNJyLE6qs6wgoPTDQAZkiqgc4_w",
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
        "id": "1AuNVfG2N9yPjiwxfp4nwt0tvQJziBZBa5mEwv6AMqoznYQK_Z-ZXH7H0a-OwU3cMlYwv1OhXQvYs6iodPigsA",
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

✅ Script saved: m-IFv4r28MtuRwOn-Q1PkwG3n5i34CJTaIrqpOdonuUJvQ_2n7MsZUcMsOowEJFaXxy8xrGR66TotViLKkJ1kw
📊 Found 10 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: KNVoqto7XYuvJw67NXj8wPJ79Njt1zV3rzGxA02DQq53tLO_2zLtktSF6aijePRGi5abs0aOf07sj2hdyUYxVg
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: EY-p2YURv7iMXP1sW2U2DKAz9kqoWANHezBwkAepGk3B0JDcXnGXsYzcTLXrjz42T2bcsRGGnUBcW5Pury7VCw
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"count":10,"max_score":90,"avg_score":50,"status":"inactive"}
   {"status":"active","avg_score":60,"count":10,"max_score":100}

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
Collection created with first record: i6ZXl0jRXJA-dekWJWxImNCNlkUW6iCqNPkM_osgQGz0V9g-hMbvO-tFEzBswo-_vLDlDj0gg7BFQO_rSNr2ew

=== List Collections ===
Total collections: 24
Sample collections: [
  'schema_users_client_ts',
  'schema_users_client_go',
  'demo_collection',
  'github_cache',
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
Created Alice: $1000 - ID: Eq80-ZSZhsYVxrciq-VxwzpIpWeQPqj5qPp7nqkVRDQV0xjK3GnA60J2_Ljfc5eqLNGn5aI0zSnrPYfVGRscXg
Created Bob: $500 - ID: Jg09YhqTEwWQur1M3u7LzNQ-waHnu7_Bd27W_7HRiAf9kedJuVz8j1zf3tU5MIbKm1kOTgD6c5Y3lDf6LMDpYw

=== Example 1: Begin Transaction ===
Transaction ID: b1b1ba03-9ec1-4f51-9e0b-ad6cb66adb2a

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
New transaction: 04116782-29f8-4a72-a08c-0c329e035853
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
   ✅ Script saved: 8ftd22oHR3IkC7gkF_ttocBmZpfb6sXtbirx_4HYu7O4k9KwI7qmK6eq6E-vWV0LGCTsGXSkL4G5MbHJMfpZZw

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
   ✅ Script saved: eDFi8vWJ4iSZmeJZfJWIpT6ldjr_25Nk_ffxCQmpf2COeX4V3MbYkWXZmMZB14rkcWlgtPWD_sWWZqbk3fWx3g

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
   ✅ Script saved: nT5SyxhnlHHW1UnLR3Tf_CzVb0k5N3mrwXgefp94gIQ1MAXmbzY_BqTUm1QE34UwDXq4PeoIdu62RWHxn5MpyA

2️⃣ Calling Script (Query + Update Credits + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {"type":"Integer","value":0}
   📋 Status: {"value":"active","type":"String"}
   📋 Name: {"type":"String","value":"Alice Smith"}

============================================================
📝 Script 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: K7ZSXx-PLxGe1EF6xb2TebmoQ1g_n8rpxQZSYDH0vvCdLmWrgv3RLJwH6RnczJVwPeYM-IkhnMLVhhO5F-ZSnQ

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: 8ftd22oHR3IkC7gkF_tt...
   ✅ Deleted script: eDFi8vWJ4iSZmeJZfJWI...
   ✅ Deleted script: nT5SyxhnlHHW1UnLR3Tf...
   ✅ Deleted script: K7ZSXx-PLxGe1EF6xb2T...
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
  Output: Document ID = noQWxxyllCeDjIhLmBtgBFwGPR3L3XwDcB8OxrO2mRStWRb-aqYf_CtZS0WgAzRX5QN2Jn5x-9QN4WGQ3mAG5w
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: findById(noQWxxyllCeDjIhLmBtgBFwGPR3L3XwDcB8OxrO2mRStWRb-aqYf_CtZS0WgAzRX5QN2Jn5x-9QN4WGQ3mAG5w)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: findById(noQWxxyllCeDjIhLmBtgBFwGPR3L3XwDcB8OxrO2mRStWRb-aqYf_CtZS0WgAzRX5QN2Jn5x-9QN4WGQ3mAG5w)
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
  Output: Document ID = GDaQcxTudsg6k1ANIqog0y7AXpDfX17mfiy3du4x1nUFNsFxZ2vkJa8emuLx_F1xXgyJOPNrI0qmIfBnV9JTQQ
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: findById(GDaQcxTudsg6k1ANIqog0y7AXpDfX17mfiy3du4x1nUFNsFxZ2vkJa8emuLx_F1xXgyJOPNrI0qmIfBnV9JTQQ)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: findById(GDaQcxTudsg6k1ANIqog0y7AXpDfX17mfiy3du4x1nUFNsFxZ2vkJa8emuLx_F1xXgyJOPNrI0qmIfBnV9JTQQ)
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

added 1 package, removed 1 package, and audited 9 packages in 830ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'HVfBXVNHkK5XaWi30hopcmHZ_vlYXwrgD94EUphN5ACYcXeGXxPHeh5QCRx0tvtU3b1gOxoNUp8o3XlfDhTPMg'
}

=== Find by ID ===
Found: {
  value: { type: 'Integer', value: 42 },
  name: { type: 'String', value: 'Test Record' },
  id: 'HVfBXVNHkK5XaWi30hopcmHZ_vlYXwrgD94EUphN5ACYcXeGXxPHeh5QCRx0tvtU3b1gOxoNUp8o3XlfDhTPMg',
  active: { type: 'Boolean', value: true }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  value: { type: 'Integer', value: 100 },
  name: { type: 'String', value: 'Updated Record' },
  id: 'HVfBXVNHkK5XaWi30hopcmHZ_vlYXwrgD94EUphN5ACYcXeGXxPHeh5QCRx0tvtU3b1gOxoNUp8o3XlfDhTPMg',
  active: { value: true, type: 'Boolean' }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: pKOZIrsi-XuC3plUY0ZuzrkoicnuuJ7OXNMK0F6QhNVNtjgcuKU4vCfsDuEl2X_p2EaOM3qy_STzsN2mBFp6Kg

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
Collection created with first record: -ifc8l32a1M8AL1JhuoZoe5V_VHD-pAxQf-7y443-67doJ0aUokK0rLpZjojjNGPEHv4oNemGTNAhntMb4hXHA

=== List Collections ===
Total collections: 23
Sample collections: schema_users_client_ts,schema_users_client_go,github_cache,websocket_test,schema_employees_client_ts

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
cache:product:3: { name: 'Product 3', price: 49.989999999999995 }

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
Created Alice: $1000 - ID: B26o3vtRZAZKVhQ9YQdSJSg54UDYE0pYzuCoGVS-a6vMuYw1jfyFWr84U7OukesMyM69ROkdcWhmP8xK3N6fLQ
Created Bob: $500 - ID: t32osPjLdkAjRhNvwRWSbIsoMnZshVbBBHjzuGMUGpu9WE3iDsSZvSq3ZSZbFrYswAA-boBNGpEF7ddSLebCBA

=== Example 1: Begin Transaction ===
Transaction ID: 146bcb25-f3d6-433a-8b6b-6e17877681e9

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 3f935bb6-4827-4623-9e72-883d52edadb9
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
  1. Score: 25.740, Matched: name, name.value, email.value, email
  2. Score: 12.540, Matched: name.value, name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title.value, bio, title, bio.value
  2. Score: 26.400, Matched: bio, title, bio.value, title.value
  3. Score: 26.400, Matched: bio.value, title, title.value, bio
  4. Score: 26.400, Matched: bio.value, title, title.value, bio

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio, title, title.value, bio.value
  2. Score: 39.600, Matched: bio.value, title, title.value, bio
  3. Score: 39.600, Matched: bio.value, title, bio, title.value
  4. Score: 39.600, Matched: title.value, bio.value, bio, title

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio.value, bio

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.755, Matched: 
  2. Score: 0.754, Matched: 
  3. Score: 0.748, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.702, Matched: title.value, content.value, content, title
  2. Score: 1.499, Matched: title, title.value, content.value, content
  3. Score: 0.302, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: bio, skills, skills.value, bio.value

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
✓ Inserted document: Z5OGJD9kI21PnnqQCXAQBqVbJXBUd5Da82DRek75I4vQC5h0f3L7Gw-erTxOXKb9Jt85Cg6qOYeiFCK2cYbP0Q

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: wPSGzTErzk9zNr1Am66AGZon02_HxSbo3h003DGnhWb6EmlnucAkBUwTfP5DDO30azc45cT0FVq7LZucEuxWDQ

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
✓ Inserted document with TTL: UMd_-sFV_WHzu5-grMa2iPXrEfqtQjrayXzklSj0BCL32W6GL6TcICSs6IB4XTXTirvvIjVW85UkjswgYyZPaQ

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
✓ Edge cache script created: UUxIEhSJ-nhv9Xb7H7QLdd3IA6O0DCwa2SUzR5qGXInEL2YhAdvU0RaMf0MOZdYAd4wQOiYF1rj5oI1AiPgxNQ

Call 1: Cache miss (fetches from API)
Response time: 5ms
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
Response time: 4ms (1.3x faster!)
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

✅ Script saved: bFWk3nAfLNKUkbkwibDKX-il44SHIGa2hmI8R-TrDQKwCXYsz-P-_MxhvICxcMqIGjUiWNexv8jtmRzQ_1QRow
📊 Found 10 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved
📊 Found 10 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved
📊 Statistics: 2 groups
   {"count":5,"avg_score":60,"status":"active"}
   {"status":"inactive","count":5,"avg_score":50}
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
   ⏱️  Duration: 149ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 82ms
   📊 Records: 1
   🚀 Cache speedup: 1.8x faster!

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

✅ Script saved: gmPfdwHmljFZjp_gZuwvby4HgSveQbPf7USMP9-RpBzXDv24n-yN7lZsUPn9fpFqOhzZMZAT2t_hhoJ4Xh8VXw
📊 Found 2 product groups
   {"category":"Electronics","avg_price":575.6666666666666,"count":3}
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

✅ Inserted order: vN4Y5QSr7QPu7PGaXypsdgwY7sU5ydxBkdxUJ3MGwLheP_kUfxKKU-Fubemc0cTnKkKUrp0FlYGXKD6Lei-P5A
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: Xvya85fe1UrYHNuQkkKJ47wmJuKrWx3t_mYsZHIw1fPd3GGFdsD11yqOC2uJ_Pmin4ds3caJOSNDDC8N7nI2Jw
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"role":"admin","userId":"user_abc"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: Cjjk-wS0eSmCne9qb4dlzwf5GlIc4v7FenLNLPcUWTMOJI_Y7wnji2hJde1i_sYyf8JDt25iSrKgB8S826xAJQ
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Script

✅ Script saved: 6LMcTdgKGVuxP_sxFZlQux0EtBOsZ4tk-W_aZz7BSjKXLVFzkaEf9emUAmOoggeaYGQ4GWeIBIOlYtoDgKNnvg
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
   admin: 20.0 (3 users)
   user: 70.0 (7 users)
⏱️  Execution time: 0ms

📝 Example 4: Top Users by Score

✅ Script saved
📊 Users (showing first 5 of 5):
   1. User 4 - Score: 80
   2. User 5 - Score: 100
   3. User 2 - Score: 40
   4. User 1 - Score: 20
   5. User 3 - Score: 60
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
   1. Vector Databases Explained (Database)
   2. Database Design Principles (Database)
   3. Introduction to Machine Learning (AI)
   4. Natural Language Processing (AI)
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
   1. Vector Databases Explained
   2. Database Design Principles
   3. Introduction to Machine Learning
   4. Natural Language Processing
   5. Getting Started with ekoDB
⏱️  Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Script saved
📊 Projected documents (showing first 3):
   1. Vector Databases Explained
   2. Database Design Principles
   3. Introduction to Machine Learning
⏱️  Execution time: 0ms

📝 Example 5: All Document Fields

✅ Script saved
📊 All documents (5 total, showing first 2):
   1. Vector Databases Explained (Database)
   2. Database Design Principles (Database)
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
   1. Monitor 27" - $399 (⭐4.6)
   2. Webcam HD - $119 (⭐4.5)
   3. Bookshelf - $149 (⭐4.1)
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

1. High Precision: They provide accurate representations of geographic features.

2. Scalability: Features in vector databases can be scaled up or down without losing quality or resolution.

3. Topological Information: Vector databases can store topological information, which allows for complex spatial queries and analysis.

4. Less Space: They typically require less storage space compared to raster databases.

5. Flexibility: They allow for different types of geometric data (like points, lines, and polygons), enabling more varied and detailed data representation.
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
✓ Created SWR script: fetch_github_user_js (PuqkWs4w1o4DSgi-IS_yxta6KKkzNcvKrL7ZIqTlq2C0mIFdQrrs7_GgHqOfXwlPwtTSQnN7alZs812zfVW9uQ)

Step 2: First call - Cache miss, fetches from GitHub API
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
Response time: 4ms (served from cache)
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
✓ Created session: 0qGAFDFj_mSh0zu18foxfiXIgCREe5CxDSBbQBrWjs-nv9AAg_YeAo_-_KSOf0bwmMNpTJ9uFAFFtl91UvAJ2g

=== Sending Chat Message ===
Message ID: jYoDxrfClJ64dVtBUnKqtSHRN2S_yrPFCiDRGAx7D-I5VXw3XL5dQ9D6XHvwGZpXgvW73Nw6TPitgLxJ2iALEQ

=== AI Response ===
We have three products available:

1. ekoDB: This is a high-performance database product with AI capabilities. It is priced at $99.

2. ekoDB Cloud: This is a fully managed cloud database service product. It carries a price tag of $499.

3. ekoDB Pro: This is our enterprise edition product with advanced features. You can purchase this for $299.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_js',
  record: {
    price: 99,
    id: 'V6SytzXi5dSHjYPIewtcM0mkpt1t5mtrHpxFPB15_eS4BTFSKoTy1o-zG83SBE9rajvkwY9o1_XWLRTNlDvs9Q',
    name: 'ekoDB',
    description: 'A high-performance database product with AI capabilities'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_js',
  record: {
    description: 'Fully managed cloud database service product',
    id: '7UGFgtZeZIJmldIYbaaCilHWcy_3OZiVxC_z7NN9yeFFaIwXxKobwK6qRgAgHOZM2MpVlv5N3A8Xv8Ksd-Z8EQ',
    name: 'ekoDB Cloud',
    price: 499
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_js',
  record: {
    id: 'ldmXq6knJFpVCy8bmrxsVwTOxAHOFKx2Kcm1Mp62nicfhMiBkrKec0c2OWxqukf5EhTPMZWLld5D-6rp2TuPLg',
    name: 'ekoDB Pro',
    price: 299,
    description: 'Enterprise edition product with advanced features'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 3443ms

=== Token Usage ===
Prompt tokens: 450
Completion tokens: 81
Total tokens: 531

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: Et21h4HEDVdzrnn-BfyOIxv0dr5kTpxxcIBsy8rxwGY2RvEhOpZ05jsWpnLPtzCugT26ZMWOa2Zpw-WU6hfhYA

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information provided, one available product is ekoDB. It is a high-performance database product priced at $99.

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
✓ Created second session: ha0tkIiMm6uchXk_8B67GmQvm0ZQ8yzyXjQ9zh69wbDHToc6QgLdh9gMVKiaiDjB2YcDnEm8jQW9yVLcLKjJfQ
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
✓ Created session: -DlWwq1deN6MqSmMxCYgexgrY7wp15Nfqgllano6jgu8HFGBWNa28Zf-FQOT8WayUlg2n0vtYMEdTp22oXY-5g

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the context provided, there is a high-performance database product available named "ekoDB". It is priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: iFCXdsR9KYcKw3lVG9afh0WVlKJyvRfi4dfzeAWBeyEaN8zdGliTxFKXvuD-EG0HMQ2pcYURVvG8u_Ce9D0E8w
  Parent: -DlWwq1deN6MqSmMxCYgexgrY7wp15Nfqgllano6jgu8HFGBWNa28Zf-FQOT8WayUlg2n0vtYMEdTp22oXY-5g

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: iFCXdsR9KYcKw3lVG9afh0WVlKJyvRfi4dfzeAWBeyEaN8zdGliTxFKXvuD-EG0HMQ2pcYURVvG8u_Ce9D0E8w (Untitled)
  Session 2: -DlWwq1deN6MqSmMxCYgexgrY7wp15Nfqgllano6jgu8HFGBWNa28Zf-FQOT8WayUlg2n0vtYMEdTp22oXY-5g (Untitled)
  Session 3: ha0tkIiMm6uchXk_8B67GmQvm0ZQ8yzyXjQ9zh69wbDHToc6QgLdh9gMVKiaiDjB2YcDnEm8jQW9yVLcLKjJfQ (Untitled)
  Session 4: 0qGAFDFj_mSh0zu18foxfiXIgCREe5CxDSBbQBrWjs-nv9AAg_YeAo_-_KSOf0bwmMNpTJ9uFAFFtl91UvAJ2g (Untitled)
  Session 5: zxC62bVDmMB87FzrbkrzM5apjFZd63cSCh_3PMkA3ZOEeiPLpnSlTz5zFu-QhvDb6yoOFtzYPpP1XF2hReSFnw (Untitled)
  Session 6: yfnh7ZzS61Cp1Gl83RKHeLmZDgPVjEfr2kzSw87kAygTXYLPuOwk-HaYoB7mwiKnjOGxfVlHVdO7z6KHNDfTFg (Untitled)
  Session 7: lz2iSHAk36CpdWaoAO7vTyjyhl-FfwkFjQZFzMQkZBi-zOZMQ4h7d9XOmO13VdPhlcHPs5GaedO4RiS3Ddsc3w (Untitled)
  Session 8: 3cCe1l-PZtrwagRkwcVNerAu-2d3BsEGkHWi6LMbv9kmRQBh6hRQz0eP-ez6i_TDyDacU9Cumo9k1IP-m0MoTQ (Untitled)
  Session 9: nY1nketylBmANam_9ptNwEUzoV2FPMqnGhEmiVsJ1azyUsLeDn9rZqVTtiKpCHEz087myGQhlRW1vJrSj7Gd_g (Untitled)
  Session 10: JWwv8AH1Ej25LutJrccUBNhSF4_0ynyuRbITNEUozPddqCs-d6VKpvdxHRveT5YEtzMASZkt21X8AaEkrkfkHQ (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: iFCXdsR9KYcKw3lVG9afh0WVlKJyvRfi4dfzeAWBeyEaN8zdGliTxFKXvuD-EG0HMQ2pcYURVvG8u_Ce9D0E8w

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
✅ [32mJavaScript client examples complete![0m
