make test-examples-javascript
🧪 [36mRunning JavaScript examples (direct HTTP/WebSocket)...[0m

added 1 package, removed 1 package, and audited 9 packages in 972ms

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
  id: 'dPUeDW-bCj8KE22oPvEkQFjjDIiiV40FNm-gBxV180OHI1o71zcyouaflOs8m7WdSoeW1wY_ldKiNGM9p1-LTg'
}

=== Find by ID ===
Found: {
  active: { type: 'Boolean', value: true },
  name: { value: 'Test Record', type: 'String' },
  id: 'dPUeDW-bCj8KE22oPvEkQFjjDIiiV40FNm-gBxV180OHI1o71zcyouaflOs8m7WdSoeW1wY_ldKiNGM9p1-LTg',
  value: { type: 'Integer', value: 42 }
}

=== Find with Query ===
Found documents: [
  {
    name: { value: 'Test Record', type: 'String' },
    active: { type: 'Boolean', value: true },
    value: { value: 42, type: 'Integer' },
    id: 'dPUeDW-bCj8KE22oPvEkQFjjDIiiV40FNm-gBxV180OHI1o71zcyouaflOs8m7WdSoeW1wY_ldKiNGM9p1-LTg'
  }
]

=== Update Document ===
Updated: {
  name: { type: 'String', value: 'Updated Record' },
  active: { type: 'Boolean', value: true },
  id: 'dPUeDW-bCj8KE22oPvEkQFjjDIiiV40FNm-gBxV180OHI1o71zcyouaflOs8m7WdSoeW1wY_ldKiNGM9p1-LTg',
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
✓ Inserted test record: pZmZikEFe1HyKF2Z_3Jna-EXgx1hTAVjD-eJPxekG-GPSKM2-F7V9T2nXwj4eUSwcVzvRtsAQlrSK55w1q1WtQ

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "messageId": "1775486162457",
  "payload": {
    "data": [
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "sl9qJa63YqamwdnLwuQ-IPi4si3m3r7hOpnW-Kztm_8W-Z8o6YzIroIr50a990xpHKfaqhdG798B5mV6dzTnUA",
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
        "id": "pZmZikEFe1HyKF2Z_3Jna-EXgx1hTAVjD-eJPxekG-GPSKM2-F7V9T2nXwj4eUSwcVzvRtsAQlrSK55w1q1WtQ",
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
        "id": "-f0h1copZhZ1-1UgYMQ-hXlAMY2TRqZRvIjv-flcOdAOifuyhypOS0w1NG7mqVGDd1BEUcdA_hwXHEBHJ__3og",
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
        "id": "uKxu__8Gtntf8Uu4Bkg9zfXLTk0OCOqXCpjPT6hkcr8ie6npnHB7qp7j7GO7igeW_fk9AaX1UlZMGCmzt1uiOQ",
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
🚀 ekoDB Functions Example (JavaScript/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Function saved: LL8uT8LZqtZcgYv-wZnrn52stCugkA5QIC96wzfJZP5Nu1dai7hsVp5rxcSfQg15PuthRtGto08RWU-An6psUA
📊 Found 10 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Function saved: 17zV-haYJZIcZffkxbDA30gZYnU7V2XnduNvUju4HLSltNn-bDfVBYoPPXg_r8C7DYPMDXcOhERiGTzUjGz0OQ
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: 13uKqLRihdXases5aOtwUPtKFpheIe8OWJCkwaIbYPDIpH8IpvtdqYRQupukrn2oIEHYv0C7iKF2-l-VWvczmA
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"count":10,"status":"inactive","avg_score":50,"max_score":90}
   {"count":10,"status":"active","avg_score":60,"max_score":100}

📝 Example 4: Function Management

📋 Total functions: 33
🔍 Retrieved function: Get Active Users
✏️  Function updated
🗑️  Function deleted

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
Collection created with first record: 6L3nGEwsDHyR-snLshFdjMM5uyDl8t7NlNePPFGnY40LNXGkjeGCYQ3JYXC8cOaRDPn6_6qa7BNtw3Z3j0T_rg

=== List Collections ===
Total collections: 25
Sample collections: [
  'schema_users_client_py',
  'test_collection',
  'schema_documents_client_go',
  'schema_products_client_ts',
  'functions__ek0_testing'
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
Created Alice: $1000 - ID: _ZfluWHxeGtE6GvQo5wjQ4MDmsqsQkz2esdG5ckM9qiK2t0sRAuhcG6E-82r4bo3KD9XHKs-innAJcX1PsfOTQ
Created Bob: $500 - ID: PMy9XtzvSw38r8Z3gYlCRVQbGMOo3wEZxjKaGFe5J5JWHVC9gOWMZE6kCZ43FAo9woOnAZDa7DcoADdljB-N1g

=== Example 1: Begin Transaction ===
Transaction ID: bfc5e97b-9478-4fec-94e0-9f87b3d7f438

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
New transaction: 3fd2d941-d11a-499c-b3ab-a4df3d6fc443
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: {"value":700,"type":"Integer"}

=== Cleanup ===
✓ Deleted test accounts

✓ All transaction examples completed
[32m✓ transactions.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/crud_functions.js ===[0m
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
   ✅ Function saved: 9NYZF9eZ6Rj5OL8OKUSiLt2pCH3Cb-CmYVFUEu9_CRmy9OhSiv_Nc4e6YUn3Erq93zo3aSyvzSbjP2lxh6GUgw

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
   ✅ Function saved: YYu62ek6crsuHSxRohMyUBmWPWZl3WmA9qkM_IV5jNeYlU3V4C3E53fLy0Ia9WXre1XIYAuUrB4mWh5lNpnLBA

2️⃣ Calling Function (Query + Update + Verify)...
   ✅ Function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Status updated to: {"value":"active","type":"String"}
   📋 Name: {"value":"Alice Smith","type":"String"}

============================================================
📝 Function 3: Query + Update Credits + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: z8pXsY-utDiO0Z_tI4_wArVwLl6wvSqnET2dHDsFybzdG2rZ209oj309u2Am8mR30eoYSMBBeSRYP0V8pHu3Ww

2️⃣ Calling Function (Query + Update Credits + Verify)...
   ✅ Function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {"type":"Integer","value":0}
   📋 Status: {"value":"active","type":"String"}
   📋 Name: {"value":"Alice Smith","type":"String"}

============================================================
📝 Function 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: PYxHzD0m-S51fwbZIuirp-JCbR8YS3HD1qdWFXDjKHpJh2Hpz7lUI2WJ5RSIv2Xv1XJtQ6aFCNJelRzDsCnrtw

2️⃣ Calling Function (Query + Delete + Verify)...
   ✅ Function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted function: 9NYZF9eZ6Rj5OL8OKUSi...
   ✅ Deleted function: YYu62ek6crsuHSxRohMy...
   ✅ Deleted function: z8pXsY-utDiO0Z_tI4_w...
   ✅ Deleted function: PYxHzD0m-S51fwbZIuir...
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
  Output: Document ID = 8YW5rRk0WNYGiETihAIzPkWXq4laBSFfjX-vO9Iy8SxdnoPvEfL97jTsJqp6Oiuc2oLUWpxZtJ-vy8RmOL1PYg
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: findById(8YW5rRk0WNYGiETihAIzPkWXq4laBSFfjX-vO9Iy8SxdnoPvEfL97jTsJqp6Oiuc2oLUWpxZtJ-vy8RmOL1PYg)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: findById(8YW5rRk0WNYGiETihAIzPkWXq4laBSFfjX-vO9Iy8SxdnoPvEfL97jTsJqp6Oiuc2oLUWpxZtJ-vy8RmOL1PYg)
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
  Output: Document ID = bUkYNieDWmb96XiF7C5vrt9Fhp24le57JDy9ys7zEZjGBNzaZFigBpVe5lv_44X4WhiEuYr09cR8a9TOXWmdgA
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: findById(bUkYNieDWmb96XiF7C5vrt9Fhp24le57JDy9ys7zEZjGBNzaZFigBpVe5lv_44X4WhiEuYr09cR8a9TOXWmdgA)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: findById(bUkYNieDWmb96XiF7C5vrt9Fhp24le57JDy9ys7zEZjGBNzaZFigBpVe5lv_44X4WhiEuYr09cR8a9TOXWmdgA)
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

> @ekodb/ekodb-client@0.15.2 prepare
> npm run build


> @ekodb/ekodb-client@0.15.2 build
> tsc


up to date, audited 52 packages in 1s

17 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.15.2 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning JavaScript client library examples...[0m

added 1 package, removed 1 package, and audited 9 packages in 981ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'Sb-yLiTQH9Fvl4dSMYv1LwQ6M0dCq5WE8ezbFPvxWxu8QhEb-atu8LxNHGKrtZ6wl5U_fhByqCR_dT_YiAgN4w'
}

=== Find by ID ===
Found: {
  name: { value: 'Test Record', type: 'String' },
  id: 'Sb-yLiTQH9Fvl4dSMYv1LwQ6M0dCq5WE8ezbFPvxWxu8QhEb-atu8LxNHGKrtZ6wl5U_fhByqCR_dT_YiAgN4w',
  value: { type: 'Integer', value: 42 },
  active: { value: true, type: 'Boolean' }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  value: { type: 'Integer', value: 100 },
  name: { type: 'String', value: 'Updated Record' },
  active: { value: true, type: 'Boolean' },
  id: 'Sb-yLiTQH9Fvl4dSMYv1LwQ6M0dCq5WE8ezbFPvxWxu8QhEb-atu8LxNHGKrtZ6wl5U_fhByqCR_dT_YiAgN4w'
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: StZilo06shSpYwrRn8ItVB9D_xUbVMxRLtDzlBmHvMK73azkumfjHrN2QFHUcQkKe0_yMLdMJ2z05161-N7Gvw

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
Collection created with first record: sXvHU4reazGjA3OdltbIukYic2T8Lkdp-SLV_zQ2mDUKSXhOLLJq6mMqE0iyR2YzacW9lSG8XVPONcb70Lmpbg

=== List Collections ===
Total collections: 24
Sample collections: schema_users_client_py,test_collection,schema_documents_client_go,schema_products_client_ts,functions__ek0_testing

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
cache:product:2: { name: 'Product 2', price: 39.989999999999995 }
cache:product:3: { name: 'Product 3', price: 49.989999999999995 }

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 0 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 15

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: false

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: o3Rva6RfNGUU2PZwQIgkhIkdyRLLyjoypa9hZpjD79rLSwxnQcvzShheMXZBYoTkD7Y7JpdY1Oao5aNWazbfXQ
Created Bob: $500 - ID: 1jt5CBBTx-bVuLUBVwFZQnRSo6pY7paYnv35-owrced_Eo4Rhe_Yg0Wqd_JJup-TIBejKCYJ3ICyYoaV4mHvoQ

=== Example 1: Begin Transaction ===
Transaction ID: ac2fc339-b548-45df-aa4a-cb5234cb9989

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 7dcd2b08-84ab-4e20-b092-a3124a6bc29b
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
  1. Score: 25.740, Matched: name, email.value, email, name.value
  2. Score: 12.540, Matched: name.value, name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title, bio.value, bio, title.value
  2. Score: 26.400, Matched: title.value, bio, title, bio.value
  3. Score: 26.400, Matched: bio.value, title, title.value, bio
  4. Score: 26.400, Matched: bio.value, title, bio, title.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title.value, bio.value, title, bio
  2. Score: 39.600, Matched: title.value, bio, bio.value, title
  3. Score: 39.600, Matched: bio, title.value, title, bio.value
  4. Score: 39.600, Matched: bio.value, bio, title.value, title

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio.value, bio

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.752, Matched: 
  2. Score: 0.742, Matched: 
  3. Score: 0.735, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.694, Matched: content.value, content, title, title.value
  2. Score: 1.497, Matched: content.value, title.value, title, content
  3. Score: 0.301, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: bio, skills.value, skills, bio.value

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
✓ Inserted document: TfKoFx4V4I2IEP3Umd4tdTzi9jYVEgoOg3Qor__KpaAF1mXdTXnvkiuZu9JL6Mkxlx17ocRbyyluAt5Uoy6QqA

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: -EYGk45eYee4URujMQDYTgiXAbnIR1FDtsZFfnHbb7ielHeEu8I7eQdFP1QjA8I9VrBMrJ7Qucs2_6yQGQR_3g

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
✓ Inserted document with TTL: 8ylnErTJa_SBXMbRvd1EQz6CGO3xH-Ze9l2qR4G6m7-OOyUTv5jUWYj9UK5ooYVBhh9wjSkb1ph0oY33T2dgsA

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
✓ Edge cache script created: p8yiLT_ccdnttc1eBdPDmpV95pi3thM5vqY9CO77CTv7t_JeoRP_dOHgc9Al28ObwRrr1hmCVZyxERq33rXDTg

Call 1: Cache miss (fetches from API)
Response time: 42ms
Result: {
  "records": [
    {
      "value": {
        "longitude": -73.99308,
        "elevation": 32,
        "current_units": {
          "time": "iso8601",
          "temperature_2m": "°C",
          "interval": "seconds"
        },
        "current": {
          "interval": 900,
          "temperature_2m": 7,
          "time": "2026-04-06T14:30"
        },
        "latitude": 40.710335,
        "utc_offset_seconds": 0,
        "timezone": "GMT",
        "generationtime_ms": 0.02312660217285156,
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
Response time: 42ms (1x faster!)
Result: {
  "records": [
    {
      "value": {
        "longitude": -73.99308,
        "elevation": 32,
        "current_units": {
          "time": "iso8601",
          "temperature_2m": "°C",
          "interval": "seconds"
        },
        "current": {
          "interval": 900,
          "temperature_2m": 7,
          "time": "2026-04-06T14:30"
        },
        "latitude": 40.710335,
        "utc_offset_seconds": 0,
        "timezone": "GMT",
        "generationtime_ms": 0.02312660217285156,
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

🚀 ekoDB Functions Example (JavaScript Client)

✅ Client initialized (token exchange automatic)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved: z1nBS2rbw79e5gQoBtV3THnCnqPvo2RcHcDzPb6p518rebr-xG5CAEA6pu5S3vs6Aox9o8lI8T-ymLj-rmlOAA
📊 Found 10 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Function

✅ Function saved
📊 Found 10 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Function

✅ Function saved
📊 Statistics: 2 groups
   {"status":"inactive","avg_score":50,"count":5}
   {"count":5,"status":"active","avg_score":60}
⏱️  Execution time: 0ms

📝 Example 4: Function Management

📋 Total functions: 36
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

✅ Saved reusable function: fetch_user
✅ Saved composed function: get_user_wrapper (calls fetch_user + projects fields)

📊 Result from composed function:
   Records: 1
   Name: {"value":"User 1","type":"String"}
   Department: {"type":"String","value":"engineering"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 156ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "email": "Sincere@april.biz",
    "website": "hildegard.org",
    "address": {
      "zipcode": "92998-3874",
      "street": "Kulas Light",
      "suite": "Apt. 556",
      "city":...

Second call (cache hit - from cache):
   ⏱️  Duration: 39ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "email": "Sincere@april.biz",
    "website": "hildegard.org",
    "address": {
      "zipcode": "92998-3874",
      "street": "Kulas Light",
      "suite": "Apt. 556",
      "city":...
   🚀 Cache speedup: 4.0x faster!

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
🚀 ekoDB JavaScript Complete Functions Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Function saved: TUp3V-Dw-pxxIz-KA7Bk5KIQphRWqBo2RsqpRfMDZFgBNOqrQZrcatmPH4OmzoqiVSLvFHHEwFwZ8E_2kK7Sgg
📊 Found 2 product groups
   {"category":"Furniture","avg_price":474,"count":2}
   {"count":3,"avg_price":575.6666666666666,"category":"Electronics"}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   {"count":3,"category":"Electronics"}
   {"count":2,"category":"Furniture"}
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

✅ Inserted order: WMFgdSTdwshKq21jxEmRr1ADJ8aYJkKMgc3HRM4PFQGmlwOA6RdzMGmXlxlSjjNTCRRve_aDQcOe0EpUqeAXFw
✅ Inserted 2 products with wrapped types

📝 Example 2: Function with Wrapped Type Parameters

✅ Function saved: dedTbrZzSt8-bCoWj-uegtOF_kAUk-xA4J95eRsUYec38LRrhghN2KVHCxIn8WH0tplsxE4AjHFAJL98UHzC6A
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"role":"admin","userId":"user_abc"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: OMY3wVzFc1n0O5O5cAPU2jpc2EnhEVAMXszwxciQUZUx9WtMq5ZtgNvSjGTpjy5AJHHZKZQk2OEPmMS0J3S_2g
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Function

✅ Function saved: W07mxhOsQaABEsc-RyeAAg3OcKQa6x6i6OTHaQZzlC6zePMIfGzyHPQfa9-ydJ9GBU009SoHD_G36_LXJWC8Kw
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
   1. User 7 - Score: 70
   2. User 2 - Score: 20
   3. User 3 - Score: 30
   4. User 9 - Score: 90
   5. User 4 - Score: 40
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
   1. Vector Databases Explained (Database)
   2. Introduction to Machine Learning (AI)
   3. Database Design Principles (Database)
   4. Getting Started with ekoDB (Database)
   5. Natural Language Processing (AI)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   AI: 2 documents
   Database: 3 documents
⏱️  Execution time: 0ms

📝 Example 3: Select Specific Fields

✅ Function saved
📊 Document titles (5 docs):
   1. Vector Databases Explained
   2. Introduction to Machine Learning
   3. Database Design Principles
   4. Getting Started with ekoDB
   5. Natural Language Processing
⏱️  Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Function saved
📊 Projected documents (showing first 3):
   1. Vector Databases Explained
   2. Introduction to Machine Learning
   3. Database Design Principles
⏱️  Execution time: 0ms

📝 Example 5: All Document Fields

✅ Function saved
📊 All documents (5 total, showing first 2):
   1. Vector Databases Explained (Database)
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
   1. Monitor 27" - $399 (⭐4.6)
   2. USB-C Cable - $19 (⭐4.3)
   3. Office Lamp - $79 (⭐4.05)
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
   1. Precision: Vector data is very precise in terms of depicting geographical features. It can accurately represent points, lines, and polygons.

2. Scalability: Vector data can be easily scaled-up or down without losing the quality of the data.

3. Less Storage: They require less storage space compared to raster databases because only the coordinates of the vertices are stored.

4. Data Integrity: Features in a vector database maintain their explicit shapes, positions, and connections to other features.

5. Flexibility: They allow complex geographic depictions like network analysis, proximity operations, and topology analysis.

6. High Quality Visualization: Vector databases provide high-quality visualization making them perfect for designing maps and similar outcomes.

7. Attribute Storage: They allow easier linkage of attributes or additional data to vector features.
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embedding function saved
📊 Generated embeddings for 3 articles
   1. "Draft Article" - 1536D vector
   2. "Getting Started with ekoDB" - 1536D vector
   3. "Advanced Query Patterns" - 1536D vector
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
✓ Created SWR script: fetch_api_user_js (BbHZIe5f356UtiPGi9GCxuvHphqHHZc-hPG8zaFa2eZ38FA2RCY8trBYxxq0vT9CW8ur5urB9rY2HvnGBSFZ0w)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "name": "Leanne Graham",
        "phone": "1-770-736-8031 x56442",
        "username": "Bret",
        "id": 1,
        "company": {
          "catchPhrase": "Multi-layered client-server neural-net",
          "bs": "harness real-time e-markets",
          "name": "Romaguera-Crona"
        },
        "website": "hildegard.org",
        "email": "Sincere@april.biz",
        "address": {
          "street": "Kulas Light",
          "city": "Gwenborough",
          "suite": "Apt. 556",
          "zipcode": "92998-3874",
          "geo": {
            "lat": "-37.3159",
            "lng": "81.1496"
          }
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
Response time: 46ms (served from cache)
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
✓ Created native SWR script: github_user_native (MzznJ8vvr86a2XOts_thdcyNjSgcbwXWkT5mqDSlO-QIjGO3USoq9jzrErdEyrLxUr1ilFMb-cQtmIdeoYZI-Q)

First call (cache miss - will fetch from GitHub API):
  Response time: 16ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 21ms
  Speedup: 0.8x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (j0vNDZZr36CMO2XPqOysrQcRheXiTpOZcpmRzxz39HnxTKTxFupWnqxRehpaxNOsWNRHgWxOYQId-o_EjqCO3A)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (Ueqx4W7hP84M1gpbgWurDo2CIcGQsEKOwpJNc2qf3S1aGy-LIGoJ0aiOiIU0qIURRfzpMR5lzXS8ZRWI7QhoLw)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (kjphfP8-gVfDcNLsB77LRfdWMHH_AvAyOP443m_lIgGMMpaiq_78nbrTayGsbDCtPFB95JYN1n7DMM7pZpEYiQ)
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
✓ Created session: xhJnPEzYQY19hfGqiq9CezuD3XkNm8DjD29LaXaQgPAAXr-oMsmWm1KavWdw6s8_oIgXbBBE7fa-7KI0cLL41w

=== Sending Chat Message ===
Message ID: nfjaVf4QFSmHSPx2LWqCxH8ayYe5RlrFkfjbb9artQ_Gt9q8XeDJY3soFOQw-IAMOqIGLxyEun2S6siMuJAWGg

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
    id: 'Hceqwl2v-JW2eZUjVGhgorhcz5eNm4WiQXAR-pqcxQQ2AXM5vMI7yP0V271yu0a88b-8rh9M9wy14wLomLQWSg',
    description: 'Fully managed cloud database service product',
    name: 'ekoDB Cloud',
    price: 499
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_js',
  record: {
    name: 'ekoDB',
    description: 'A high-performance database product with AI capabilities',
    id: '4rw40SLO0A5s4pZQmOLKazCYe_Hn5BmdZs2tAR0R2BMAmiNg19ODuZAkDBdzyjFOozdmiHpRh7KbqtTUH2aB-Q',
    price: 99
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_js',
  record: {
    name: 'ekoDB Pro',
    price: 299,
    id: 'MoX7pxi7GUs3Bu53gcWss9VkGGrZiDNTwVbbKWi7Kq2bCDMNFxcwc6pH1-a8tbJA7Qs2XwQqrEBVaHoEiJzSIw',
    description: 'Enterprise edition product with advanced features'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 5227ms

=== Token Usage ===
Prompt tokens: 1298
Completion tokens: 97
Total tokens: 1395

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: mp1VRoI-yQEQp3rqDaf5l9u_h4lt26FQQarpMckRvLv66BktEXTW0JLESZtGOMZvdZvtyxY6qk3dyNHtLvwDuw

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Product Name**: ekoDB
- **Description**: High-performance database product
- **Price**: $99

If you need more information or want to see additional products, feel free to ask!

✓ Second message sent

=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of ekoDB is $99. If you have any other questions or need further information, feel free to ask!

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: GutR2Kop14miQ1gINNV3dyWQoCz_wIRTt5bhqdUsTMcZVv-VPuqpw2_kywNb-47bqvPCJxdHembS8c2OVpdtAw
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
✓ Created session: E-uDmg_IPKtQyLt2hOq6vbFqAqgrkvLyqLVeF79CwKN_oq4l04hnrQ7NOd9SNVq5xmmeFaUX5OAVLjJBkHCkHA

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name**: ekoDB
- **Price**: $99

If you need more details or additional products, let me know!

✓ Message 2 sent
  Response: The price of ekoDB is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 4Bl5wz2Y8rdc4sruWuk7N2JCwbcXVWTliA2RshlHx00AP15oQaGQIoHDzoe8-tV-5fdqaHNGezS5Ef6gUXHxEQ
  Parent: E-uDmg_IPKtQyLt2hOq6vbFqAqgrkvLyqLVeF79CwKN_oq4l04hnrQ7NOd9SNVq5xmmeFaUX5OAVLjJBkHCkHA

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: 4Bl5wz2Y8rdc4sruWuk7N2JCwbcXVWTliA2RshlHx00AP15oQaGQIoHDzoe8-tV-5fdqaHNGezS5Ef6gUXHxEQ (Untitled)
  Session 2: E-uDmg_IPKtQyLt2hOq6vbFqAqgrkvLyqLVeF79CwKN_oq4l04hnrQ7NOd9SNVq5xmmeFaUX5OAVLjJBkHCkHA (Untitled)
  Session 3: GutR2Kop14miQ1gINNV3dyWQoCz_wIRTt5bhqdUsTMcZVv-VPuqpw2_kywNb-47bqvPCJxdHembS8c2OVpdtAw (Untitled)
  Session 4: xhJnPEzYQY19hfGqiq9CezuD3XkNm8DjD29LaXaQgPAAXr-oMsmWm1KavWdw6s8_oIgXbBBE7fa-7KI0cLL41w (Untitled)
  Session 5: SHA6ohgHqTHINn3lsYulYkHqq5UYoCvwan6Om54JoOIfumwS6awtoGHyTgq66pFGaFZpWumk8QBP02E8uD-Qpg (Untitled)
  Session 6: x0yNZbn8j0NxZISxXnlRTttymR6nJU92zziWR2D3CLJj2ksR5jg13bT4V9rSFLPLcuTXp4WJlFhiL03sFbH-yA (Untitled)
  Session 7: SD51LcfJKzg1prQWmIzwvZqJPSWingS4wnfYLoIr_f8y5VUpaqtuq0BtpzrdL3OncQ6qf0-EULyh9rmxHgkk0w (Untitled)
  Session 8: FXw_vmV_nz9mVLs7l1N0z1q4JHtj6mAR-EmfQenVjVDP7i3OZG0awa6LYOI7gY4iGNQC8zFW5wlNVzQeue3zFQ (Untitled)
  Session 9: qaZlnrvpXgOmnRT4X_GoobUXY1d80jAl02ds3ItS73nTHw3COFVhQhh-TwtolkoeD_sQZQxcJOVGAibxq0V9Kg (Untitled)
  Session 10: kDm_Y1zUiVUO4TkG5AcPEUyMvCyXVR63nkP-M4pGiSTJsb_QfxT5bANALSqmw9CQW7oq5Wja2Yfkac6BGW-rew (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: 4Bl5wz2Y8rdc4sruWuk7N2JCwbcXVWTliA2RshlHx00AP15oQaGQIoHDzoe8-tV-5fdqaHNGezS5Ef6gUXHxEQ

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
✅ [32mJavaScript client examples complete![0m
