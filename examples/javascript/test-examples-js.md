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
  id: 'dibfDoAuiUZhbntNaOSIhPq_fGgvYI1UWgr5Ut7E177nLxntuvqK7UyL1xvFilORWSx9S-kyX3g_pmraeTgXrQ'
}

=== Find by ID ===
Found: {
  id: 'dibfDoAuiUZhbntNaOSIhPq_fGgvYI1UWgr5Ut7E177nLxntuvqK7UyL1xvFilORWSx9S-kyX3g_pmraeTgXrQ',
  value: { value: 42, type: 'Integer' },
  active: { type: 'Boolean', value: true },
  name: { type: 'String', value: 'Test Record' }
}

=== Find with Query ===
Found documents: [
  {
    name: { value: 'Test Record', type: 'String' },
    id: 'dibfDoAuiUZhbntNaOSIhPq_fGgvYI1UWgr5Ut7E177nLxntuvqK7UyL1xvFilORWSx9S-kyX3g_pmraeTgXrQ',
    active: { type: 'Boolean', value: true },
    value: { value: 42, type: 'Integer' }
  }
]

=== Update Document ===
Updated: {
  value: { type: 'Integer', value: 100 },
  active: { value: true, type: 'Boolean' },
  name: { type: 'String', value: 'Updated Record' },
  id: 'dibfDoAuiUZhbntNaOSIhPq_fGgvYI1UWgr5Ut7E177nLxntuvqK7UyL1xvFilORWSx9S-kyX3g_pmraeTgXrQ'
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
✓ Inserted test record: 7jZICJEcyznxXiHsS2cgUcEa2zKrPF2G0x8p5vLpn7V12z73WWoKNYfKGCsJxQR6gCzxg0WxxAXhWgyDVk0BUA

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
        "id": "p29wgNhv15wQFnFRwZVmU75-w5TLn3Qycq3wU-vZCs3UtBruVucpOyhaY-ay6drGKU6soyPlHQmUkOmFaAZr-w",
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
        "id": "7jZICJEcyznxXiHsS2cgUcEa2zKrPF2G0x8p5vLpn7V12z73WWoKNYfKGCsJxQR6gCzxg0WxxAXhWgyDVk0BUA",
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
        "id": "ywoj2dz8naGzZbLS0WxHYqaEy-sh5CkOfNidCqhHh37j031rIlggOHXPDzDJ8MMMI9i10dtBBT64461bhKvHJQ",
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
        "id": "TnN49tN_ZzlfQSCWsfi8rPbOGzCj4lD5Zvwj6pnI8u9EN8AVbYmQkLJpacV1UBNbpC9zb_cZrI4jdYFU5oyg0A",
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

✅ Script saved: WQQ0j2YLLaKfUbs78OjPHh4IE08u69B5d3HVMqVWaMJLzvQ_mCQ6Nd17CxfrF8tPJbKJSEfO9uyOCHFtrfZuQw
📊 Found 10 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: 8xzq3Q0nqYKW9AmIDXideKlmanYeLoPemXDsRVHEXFGCxIwIVnWg5hvoUA5SLwi30e0qg9T1BlMgOFDjOMMCHQ
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: K6fFawIahRErAWcZ-bYaTE4v_LuJJt7vK7iUORJTanojso80hm-WQuVsIhGXE0Lz0WZm8P2WUUJOoqkRSo7DDw
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"count":10,"avg_score":50,"status":"inactive","max_score":90}
   {"max_score":100,"count":10,"status":"active","avg_score":60}

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
Collection created with first record: h9u3yYuJZAvSMxsMh2d7sIkPDVXHt1NIT7peODEWa2eAIiajPNNC-8lQna8AQIaz9DCN1U6J4khjRyt44Ho-GA

=== List Collections ===
Total collections: 25
Sample collections: [
  'schema_products_client_go',
  'audit__ek0_testing',
  'test_collection',
  'scripts__ek0_testing',
  'schema_employees_client_go'
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
Created Alice: $1000 - ID: lNG0wWeOASJU0iDwcE8puApweCEqOb65AoWz28kMW-Fv2mGRF3HWnxVaTR1oLlXZUU2ceuqMHLNY3uRSKMwOiA
Created Bob: $500 - ID: xHUOF7lpZEgvVywMiPnUj6y98iLkO-XL_N5evE9t_GsxHtgm-U34TOulXFYXbu-0GaRYH8s8i80w8Gqjfeg5tA

=== Example 1: Begin Transaction ===
Transaction ID: 646002c5-c5ab-4cce-95fe-f5e409f6b666

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
Bob: {"type":"Integer","value":700}

=== Example 5: Rollback ===
New transaction: d05f0b9d-e8c5-40fa-be24-cbd5ce60d57f
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
   ✅ Script saved: WKBostS2XQHq9vDgwBqnIh7AyyJiEr8hhq1MxwNIxzWAyeu-xzZiQ6zlT_If65jMykKGzYsRrUsyfO5__hRx7A

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
   ✅ Script saved: M6eBgjGpL8V3-5jgqM6Oxabagab3FWHrWf1SWPH1V53EYZjzIJigd5O5Db_icL-6fX-XWpXFERoB6vmLPJyrJA

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
   ✅ Script saved: JQ9BP7hDjsrKUWdLD_-0dgQQJY7bdGP0LNqXfYAeJXCehAKKXQCq_gydiIcQ9EzyimvNvEEJhZ5Jku_1suZmFA

2️⃣ Calling Script (Query + Update Credits + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {"value":0,"type":"Integer"}
   📋 Status: {"value":"active","type":"String"}
   📋 Name: {"type":"String","value":"Alice Smith"}

============================================================
📝 Script 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: FNpT9Um2lubTsmTEsr38mCMlGSbzhuT6JfMH3IbianslG-Q-O6NNGf9wIYvQA_QrpK2Osrg5om4wsTyOuj3IQA

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: WKBostS2XQHq9vDgwBqn...
   ✅ Deleted script: M6eBgjGpL8V3-5jgqM6O...
   ✅ Deleted script: JQ9BP7hDjsrKUWdLD_-0...
   ✅ Deleted script: FNpT9Um2lubTsmTEsr38...
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
  Output: Document ID = 41zT0SQ43HeZ86vb-hQRhRVXokwJWq-46O-pOYRmvFS91kJk4AqXn6rmuIB9pspXzCZWaYA1eexozrh__IH81g
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: findById(41zT0SQ43HeZ86vb-hQRhRVXokwJWq-46O-pOYRmvFS91kJk4AqXn6rmuIB9pspXzCZWaYA1eexozrh__IH81g)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: findById(41zT0SQ43HeZ86vb-hQRhRVXokwJWq-46O-pOYRmvFS91kJk4AqXn6rmuIB9pspXzCZWaYA1eexozrh__IH81g)
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
  Output: Document ID = VDU_wuvdjPMnMwvex8nGJFF0BjI-w4nRAawkGSNhPPlj6P7nNW3vwEdJWw45cWCO47agu4pJDVdEmQD1abfVnw
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: findById(VDU_wuvdjPMnMwvex8nGJFF0BjI-w4nRAawkGSNhPPlj6P7nNW3vwEdJWw45cWCO47agu4pJDVdEmQD1abfVnw)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: findById(VDU_wuvdjPMnMwvex8nGJFF0BjI-w4nRAawkGSNhPPlj6P7nNW3vwEdJWw45cWCO47agu4pJDVdEmQD1abfVnw)
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

> @ekodb/ekodb-client@0.14.0 prepare
> npm run build


> @ekodb/ekodb-client@0.14.0 build
> tsc


up to date, audited 53 packages in 1s

17 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.14.0 build
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
  id: 'WOM0qBExl5NaYVxHSkcVgz550v1-tSemvKhZSvLSSp8dr_A2YePtK-xkQF_1npY5SnU49yxvRaAPcSeh-ltvZA'
}

=== Find by ID ===
Found: {
  value: { type: 'Integer', value: 42 },
  name: { value: 'Test Record', type: 'String' },
  active: { type: 'Boolean', value: true },
  id: 'WOM0qBExl5NaYVxHSkcVgz550v1-tSemvKhZSvLSSp8dr_A2YePtK-xkQF_1npY5SnU49yxvRaAPcSeh-ltvZA'
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  active: { value: true, type: 'Boolean' },
  id: 'WOM0qBExl5NaYVxHSkcVgz550v1-tSemvKhZSvLSSp8dr_A2YePtK-xkQF_1npY5SnU49yxvRaAPcSeh-ltvZA',
  name: { value: 'Updated Record', type: 'String' },
  value: { type: 'Integer', value: 100 }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: bRKP8n0AQgqHfcyXrHekCOrnLma_S7Bt1sTmpukEU6n1kDsQT6IvXPiS1mLmTgi3xJRTD4MRHYt1WqjKWF4j9w

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
Collection created with first record: WNVSncZ8LzmcNSznTWwcBN_bKbGHRRoUHDTQCVbmDIAr8ad3moFfeOFylMdkZG0IgVbhjqPn-uZOk8apLSDRBA

=== List Collections ===
Total collections: 24
Sample collections: schema_products_client_go,audit__ek0_testing,test_collection,scripts__ek0_testing,schema_employees_client_go

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
Total keys in store: 17

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: false

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: p6uBqXFublHIlKp8NkNcf9-34T48TyWTpZPyb_zWxhldrUvh_prjfGebgyTG1jX-qA8F1stmbY4pRBx3FOTF5A
Created Bob: $500 - ID: Oo-eHUgAovUptVXB-rrxPqfjSgbePCHFhG1BGLAReiRC0wdQt4CDNWo1DkYy1cQAr0lBgyAmBR2nWM9wmuf9pA

=== Example 1: Begin Transaction ===
Transaction ID: 91d7870d-793b-45cf-be4f-c1b0270be397

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: c1f520eb-1060-415f-b01e-2bd965acc062
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
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: bio.value, title.value, bio, title
  2. Score: 26.400, Matched: bio.value, title, bio, title.value
  3. Score: 26.400, Matched: title, title.value, bio.value, bio
  4. Score: 26.400, Matched: title.value, bio.value, title, bio

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title, bio.value, bio, title.value
  2. Score: 39.600, Matched: title, title.value, bio, bio.value
  3. Score: 39.600, Matched: title, bio, bio.value, title.value
  4. Score: 39.600, Matched: title, bio, title.value, bio.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio.value, bio

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.776, Matched: 
  2. Score: 0.749, Matched: 
  3. Score: 0.749, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.699, Matched: title.value, content.value, title, content
  2. Score: 1.499, Matched: title.value, content.value, title, content
  3. Score: 0.310, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: skills, bio.value, skills.value, bio

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
✓ Inserted document: cvDYW8DLsNl1lfUWUFw-LaOVK28ckhVa1wlS_zdDZfZq7wmyxKUAXK2vc0ZHqaUhnFXH-xwxl1XvocNs576bVQ

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: I4CWlZ7Ke-cqFVO_3H7_tE_3ZRv_qkIpC6-tWibWMXkU8ECiZvYLNavaS_UKcOw6ERG0uKA3mLKUANTi3nXloQ

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
✓ Inserted document with TTL: n9q4owW7OrqzE6WAwzUrmGMU_G-1nldZVBIufMUlUBrMNUan5ea_pngsvEgwS2ReKv4m8WgTlFLXxQiW97FxYw

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
✓ Edge cache script created: ziKs6rXDeE5Fe1cKrKaTaMzzG5OfHB2AXxfMIO9Jsc4UcDS0qoI5yJfRdS0tCAQweyuIHfE_iTvMZM1jwcmnAA

Call 1: Cache miss (fetches from API)
Response time: 4ms
Result: {
  "records": [
    {
      "value": {
        "current_units": {
          "temperature_2m": "°C",
          "time": "iso8601",
          "interval": "seconds"
        },
        "generationtime_ms": 0.32401084899902344,
        "longitude": -73.99308,
        "timezone": "GMT",
        "timezone_abbreviation": "GMT",
        "utc_offset_seconds": 0,
        "elevation": 32,
        "current": {
          "time": "2026-03-24T21:00",
          "interval": 900,
          "temperature_2m": 8.5
        },
        "latitude": 40.710335
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
Response time: 3ms (1.3x faster!)
Result: {
  "records": [
    {
      "value": {
        "current_units": {
          "temperature_2m": "°C",
          "time": "iso8601",
          "interval": "seconds"
        },
        "generationtime_ms": 0.32401084899902344,
        "longitude": -73.99308,
        "timezone": "GMT",
        "timezone_abbreviation": "GMT",
        "utc_offset_seconds": 0,
        "elevation": 32,
        "current": {
          "time": "2026-03-24T21:00",
          "interval": 900,
          "temperature_2m": 8.5
        },
        "latitude": 40.710335
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

✅ Script saved: t20Ej3tgmx431mi0wOZHZshglJmoXg4BDCYOIKTm1SrmEICVG3dctme6X5Yxl0zzHzs1KqHoPB6flsyXFgE1tA
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
   {"status":"active","avg_score":60,"count":5}
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
   Department: {"value":"engineering","type":"String"}

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
    "website": "hildegard.org",
    "company": {
      "catchPhrase": "Multi-layered client-server neural-net",
      "bs": "harness real-time e-markets",
      "name": "Romaguera-Crona...

Second call (cache hit - from cache):
   ⏱️  Duration: 5ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "website": "hildegard.org",
    "company": {
      "catchPhrase": "Multi-layered client-server neural-net",
      "bs": "harness real-time e-markets",
      "name": "Romaguera-Crona...
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

✅ Script saved: Kl-NcZC4kugBf6dgK9sWNfAplgEheOJ5pATlBE3sjRk7-BEMw3op-wiNwI1ut1646MhRTxVpGi1qzVdYCvoEkg
📊 Found 2 product groups
   {"count":2,"avg_price":474,"category":"Furniture"}
   {"count":3,"category":"Electronics","avg_price":575.6666666666666}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved
📊 Found 2 categories
   {"count":3,"category":"Electronics"}
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

✅ Inserted order: yddWY1S1Qiq1QS41RTA0_Bzz-542J28eAAZHeduYow3E7GQEv03JTZXoEv0Lhfnw_GA0b5VIvNi33eyTxRQnow
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: nYAMBGkys_PqRqRk2dKTECIN_uRL0KPRX79ik-Kz5q1GUX8fL7YB-_g3CvNy_EB6cBDk8y6bsjm6Od9KKNBVEg
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"userId":"user_abc","role":"admin"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: ftLs09GPv90nJOswwoueIaF486zPfWK-wNX10o--v3GCsDdesXs7nfRCGM92yo79zObcE4KkoEDYwOS86BXaew
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Script

✅ Script saved: xo9Ro67yV7tUovzzAKXEmIif1zwsq6W6ojO_1_HtWWcyj3Oq40orlaxPmIGAiBKJVUpxW49RZtyieGSBy558hw
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
   1. User 6 - Score: 60
   2. User 9 - Score: 90
   3. User 10 - Score: 100
   4. User 7 - Score: 70
   5. User 1 - Score: 10
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
   2. Introduction to Machine Learning (AI)
   3. Natural Language Processing (AI)
   4. Getting Started with ekoDB (Database)
   5. Database Design Principles (Database)
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
   2. Introduction to Machine Learning
   3. Natural Language Processing
   4. Getting Started with ekoDB
   5. Database Design Principles
⏱️  Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Script saved
📊 Projected documents (showing first 3):
   1. Vector Databases Explained
   2. Introduction to Machine Learning
   3. Natural Language Processing
⏱️  Execution time: 0ms

📝 Example 5: All Document Fields

✅ Script saved
📊 All documents (5 total, showing first 2):
   1. Vector Databases Explained (Database)
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
   1. Standing Desk - $599 (⭐4.7)
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
   1. Precision: Vector data is more accurate as it uses coordinates for representation. 

2. Scale Independence: Vector data remains undistorted when zoomed in or out, maintaining its original shape.

3. Representation of Data: It can better represent discrete objects and complex structures like road networks, contour lines.

4. Attributes Association: Vector data allows for easier attaching of attributes to the geometric elements.

5. Less Storage: Vector data typically requires less storage, depending on the complexity and size of the data.  

6. Data Manipulation: Geometric transformations, such as translation, rotation, and scaling, are easier with vector data.

7. Topological Analysis: Vector data can effectively store topological information, which is crucial for spatial analysis and network analysis.
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
✓ Created SWR script: fetch_api_user_js (48_lOOWXIjzlLAeMPkKA85zIecFZ3tcvuccYLKHl03YiOC4Yp2pkAVP8HTKzS8aXOFD4P8zD18hXExa8TKr3Ow)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "address": {
          "city": "Gwenborough",
          "geo": {
            "lng": "81.1496",
            "lat": "-37.3159"
          },
          "suite": "Apt. 556",
          "zipcode": "92998-3874",
          "street": "Kulas Light"
        },
        "name": "Leanne Graham",
        "id": 1,
        "website": "hildegard.org",
        "company": {
          "bs": "harness real-time e-markets",
          "catchPhrase": "Multi-layered client-server neural-net",
          "name": "Romaguera-Crona"
        },
        "phone": "1-770-736-8031 x56442",
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
✓ Created native SWR script: github_user_native (RrsWNWpQrUJgPZGi45ezuPdFlHMo6fI22cp6x8q3uP3g0yfrIYPGwXc5dx2gxU5AYAZvpYCI5SPeQq9VDFOBdA)

First call (cache miss - will fetch from GitHub API):
  Response time: 3ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 11ms
  Speedup: 0.3x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (q595QxXBYDlp11vprMnYaQ_D36i1H4xY4taKzgTTV7hTnuBETwDlT5PKLOS9yo5Gav_HNADYHxMUC9fxsphRmA)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (6FEL2jO6XsOliQE6srDLGEMd1d-ErUH5Ww9Ih3HzMGb6jW6Q687NDrnPwaiBC25alsvVnsix03gg-SuXReuvNw)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (SU56XG7WQrgEPCrMnQQYc63a1rhCJWMReCQQR_3q50eAIaWQ_gPYv7j_acSl7O2ygECGlGG7wSYEeF4OljxVbA)
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
✓ Created session: NX84j8S1dCup5TCwM1EURcGz35HL2zQlWSPO8uQKhde8GVMLZIwTH3ikLuziG2EWfSu_HcMreHA8lFD-BQj0YA

=== Sending Chat Message ===
Message ID: VER0NIR8UnitD9BUSNdNjHCfiUNK3a6kVsU-qpBhzoQ7c2JuTbp3TlhM6jFcQ0mS8ky_WRHWR59Mef8nj2oF3Q

=== AI Response ===
Here are the available products and their prices:

1. **ekoDB**
   - Price: $99
   
2. **ekoDB Pro**
   - Price: $299

3. **ekoDB Cloud**
   - Price: $499

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_js',
  record: {
    name: 'ekoDB',
    description: 'A high-performance database product with AI capabilities',
    id: 'nUF0FcQdI_D6dR7pnwJ9MosE0U5oZYVHRi1f79DJkH68lE2a6rfggRITkqv9osWzDeLBw7R1l8yBQ7a187kQNA',
    price: 99
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
    id: 'NNLCAgzZJkstT6s7GjqibGwffIi7quyfmXTYHe8ql7d_dNBaZkRe8nteJLpJHvqoDlhQ39x7zEwO_4f4vcGyDA'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_js',
  record: {
    price: 499,
    name: 'ekoDB Cloud',
    description: 'Fully managed cloud database service product',
    id: 'yj8jpcJ3rhEeGxdMdRdlZy8E3Ov1dHzBC6msvXQ6jsb_lgaCmRM20zhVc8yBonbKGr09ersroIjOQDou9pKroA'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 2318ms

=== Token Usage ===
Prompt tokens: 716
Completion tokens: 38
Total tokens: 754

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 24B8_XNJqPpL2rbEOILWok2N8Uf1LtBdKXZtNJJduagzdt6WS20xeDGaxJZaeHzSHnBzo_nhUtcLbaxVWUhRmA

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
  New response: The price of ekoDB is $99. If you have any further questions or need more information, feel free to ask!

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: xSRugGMPV-jKUnDyDXbSpD7ac7_gIFWtKzWAZni8RV-0JrfYAz9Gq9nVeelnxqDugaaQ6juIjVjQGarZ6-20dw
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
✓ Created session: 7GtPg8bU8KXpZAZZbpNQ4427pr4xcVIj0XznZeLM5vZPu1T4hIpcDJXrQjMDQgI0WF_Fs8DljNSb6ePFTF8RYQ

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: A high-performance database product
- **Price**: $99

If you need more information or have further questions, feel free to ask!

✓ Message 2 sent
  Response: The price of ekoDB is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: N7bEkCgLiD9HSAGYnCX1_VybhYBi7uKqpgdS_REuW0opqEqp6npwd5IyGsiU612XdX6tjjUN5VublsuWteY7lQ
  Parent: 7GtPg8bU8KXpZAZZbpNQ4427pr4xcVIj0XznZeLM5vZPu1T4hIpcDJXrQjMDQgI0WF_Fs8DljNSb6ePFTF8RYQ

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: N7bEkCgLiD9HSAGYnCX1_VybhYBi7uKqpgdS_REuW0opqEqp6npwd5IyGsiU612XdX6tjjUN5VublsuWteY7lQ (Untitled)
  Session 2: 7GtPg8bU8KXpZAZZbpNQ4427pr4xcVIj0XznZeLM5vZPu1T4hIpcDJXrQjMDQgI0WF_Fs8DljNSb6ePFTF8RYQ (Untitled)
  Session 3: xSRugGMPV-jKUnDyDXbSpD7ac7_gIFWtKzWAZni8RV-0JrfYAz9Gq9nVeelnxqDugaaQ6juIjVjQGarZ6-20dw (Untitled)
  Session 4: NX84j8S1dCup5TCwM1EURcGz35HL2zQlWSPO8uQKhde8GVMLZIwTH3ikLuziG2EWfSu_HcMreHA8lFD-BQj0YA (Untitled)
  Session 5: O8uPYCmT2bs6eElGfQIFq1bzDYTzukhYDp6g7mAC7HwIhpk_UMryxNuCQvaDCb7xbVIkiRkn1s6GEBlUlD0-jQ (Untitled)
  Session 6: lefcny3jZshMG5_Q9EfoiHHaKZbo3hH7LV-icODkkwOSf_jOXUIuVBWNwBLqE9XOeRV3EbWlOIMqSckIOz24TA (Untitled)
  Session 7: pnxe7aMbQjH_peBLMpm81qXVZK0UiG3R1-i60T-WuYnpmL8DlcagI3v-QQ_XmhsqTNncD7c0tbHotMkjZN7dgA (Untitled)
  Session 8: VJjosVy2bhucs6R1c8L-uGMZbpxPNmhfNIQerlEoqtJxXHkKQgjkkV1lkOE74eG0TxxhW9DlubqMAZPzitnAOA (Untitled)
  Session 9: rGkk2u51QaaFHFlvz4hu54e9j4c-KjMEIiQNqbRkT7pOSgHeL86q4UXxB_VTQ448nKIx8RaLhdeQD2FIBtQYqw (Untitled)
  Session 10: Xy_exLllEMByQPU328zPF3r8Ba9AMuyW7pJXisWrxwqgkWtisN5UK9988eGp2nPuhojnw0WJ3HIjaIVYC9p_cA (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: N7bEkCgLiD9HSAGYnCX1_VybhYBi7uKqpgdS_REuW0opqEqp6npwd5IyGsiU612XdX6tjjUN5VublsuWteY7lQ

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
✅ [32mJavaScript client examples complete![0m
