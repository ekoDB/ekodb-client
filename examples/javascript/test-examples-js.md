make test-examples-javascript
🧪 [36mRunning JavaScript examples (direct HTTP/WebSocket)...[0m

added 1 package, removed 1 package, and audited 9 packages in 832ms

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
  id: 'DHOnYbTKBLtnzDKUYXfHaxXYeLifuCJqKwAXjiGSeNGNyfP2vC0Dh45u1Lrl497aYvjO_Gk7qi90xOcdiaW9UQ'
}

=== Find by ID ===
Found: {
  name: 'Test Record',
  active: true,
  id: 'DHOnYbTKBLtnzDKUYXfHaxXYeLifuCJqKwAXjiGSeNGNyfP2vC0Dh45u1Lrl497aYvjO_Gk7qi90xOcdiaW9UQ',
  value: 42
}

=== Find with Query ===
Found documents: [
  {
    name: { value: 'Test Record', type: 'String' },
    active: { type: 'Boolean', value: true },
    id: 'DHOnYbTKBLtnzDKUYXfHaxXYeLifuCJqKwAXjiGSeNGNyfP2vC0Dh45u1Lrl497aYvjO_Gk7qi90xOcdiaW9UQ',
    value: { value: 42, type: 'Integer' }
  }
]

=== Update Document ===
Updated: {
  value: { value: 100, type: 'Integer' },
  name: { value: 'Updated Record', type: 'String' },
  active: { type: 'Boolean', value: true },
  id: 'DHOnYbTKBLtnzDKUYXfHaxXYeLifuCJqKwAXjiGSeNGNyfP2vC0Dh45u1Lrl497aYvjO_Gk7qi90xOcdiaW9UQ'
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
✓ Inserted test record: olqTjqs8sFbl1pTVr8na7b1YtSu7DtLDrWrNJUQIe6_b7G81iucHulLDTIOz8ahMO81ruIGD_KgiZ1JvvTKh7w

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
        "id": "S_U13Mss9h6XuHFJAuiD6zeBJ1UT7ZOk0_SZxhcjzzoyIThIysh_qUPR8HijAPJf_mlmpEgauyRwdt0ZfUuDxA",
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
        "id": "olqTjqs8sFbl1pTVr8na7b1YtSu7DtLDrWrNJUQIe6_b7G81iucHulLDTIOz8ahMO81ruIGD_KgiZ1JvvTKh7w",
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
        "id": "nNbqNp0PZJeu64PMuqIrmHqPklGfWntWmBXjK_lKGx_W3ZKFCFjRngAoKH6Qz1MavLhGdZhaGsW2vaME1-AoBw",
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
        "id": "S-g_KsEmXxeRR7SufzogTElDqP-F9m9IRs05FO8UL78xYZaP_aAkSbHUzqAbaHHS4eeylQKCp90CHoPUwFFtkQ",
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

✅ Script saved: RpCd-mGVwRxlEETRpQMUBbepbuSh42N_HXYRD72aHMSiKh7IRVPJF7gecDfdusw9YyRK6Q72EjlkhJhb5ZByCA
📊 Found 30 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: C0g7am4I7Nb6R-7KMQyb2aggRfYFOM68F1C1f_5QPp4onmQstQ3crHNVvefv5dPC01bd5j2UYXLEQpzp4Du7-A
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: Lxn0Ywzq1xZuGrl334Iz1lu94dMsIMYt5S7KWDmgdXDGXCI1wgL9xUtVaBtAHoMLk-tPcgNscPa-Zl4swor6JQ
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"status":"inactive","avg_score":50,"max_score":90,"count":10}
   {"count":10,"avg_score":60,"status":"active","max_score":100}

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
cache:product:2: { price: 39.989999999999995, name: 'Product 2' }
cache:product:3: { price: 49.989999999999995, name: 'Product 3' }

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
Collection created with first record: jGl8aCLnimOTbLuPhbL8b6WTE29Ir2Y8NplappzHZci6b9Xa7wQzWethobkPlsLRnEUcgE_p2JnciagYetNYtw

=== List Collections ===
Total collections: 23
Sample collections: [
  'chat_configurations__ek0_testing',
  'test_accounts',
  'schema_users_client_go',
  'batch_users',
  'demo_collection'
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
Created Alice: $1000 - ID: EeVnQZ4KZUG9UeeOEsYQHlbYs77l2HmBmvj_Z4oBrDgu8O0mCRs4UOP5xzlP61CzHNg9i314xBUIjOlVx7ZojQ
Created Bob: $500 - ID: 0jfrw_fY7Ql0AlNpp_LNANR1mHtYtRnMNGtGTgWoXS_9JmJItKXwj6Jn1vHT1t7lvC6k-eCsYbH3fHNgxlfoGA

=== Example 1: Begin Transaction ===
Transaction ID: 85b2a842-26f6-46cd-b8cb-997703dc1dee

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
New transaction: c7bc1600-0947-471c-a61d-2adb89b9582c
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
   ✅ Script saved: C4ALLJuQrzAVs8XtzzWROO4ukEhHf1czI3kIEABKdYxteA_yp0OV3ir-KDFlF86E7zkoCTGrgx1FjC1-Vjlqow

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
   ✅ Script saved: cMCD5MUFcYNRzqCHYo11aeq0R9Dh2dNZNcuO0e5RGn6LTpwdvhT4d0tYu6qYbM4KNE5jHpaS1ohHUNpbXxsZJw

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
   ✅ Script saved: R5WFHTDGbBvghF-XAMq_zLxp-NTJUB5pJAnY9Uyy3vTAszH98Fd2jMrkHyThnEbSTyg_H-5YtXwnZkubPV53qw

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
   ✅ Script saved: pLuVBXkfG9-RjMb_bSJ6tgEimsPZlQ1qgUscThmhmIHTjnM2DndaVEfJ0Ju3xrTd40bdpQWk29KNXfauopS6_A

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: C4ALLJuQrzAVs8XtzzWR...
   ✅ Deleted script: cMCD5MUFcYNRzqCHYo11...
   ✅ Deleted script: R5WFHTDGbBvghF-XAMq_...
   ✅ Deleted script: pLuVBXkfG9-RjMb_bSJ6...
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
  Output: Document ID = vsZR1V-oj7rs3YvzGQ5kMI8A1n9Hg2kKc4pal-TGDJekeF_fZX-rwjOEVDgRv5ixJDLXWq_2MnfA4tV_Jga7iQ
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: findById(vsZR1V-oj7rs3YvzGQ5kMI8A1n9Hg2kKc4pal-TGDJekeF_fZX-rwjOEVDgRv5ixJDLXWq_2MnfA4tV_Jga7iQ)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: findById(vsZR1V-oj7rs3YvzGQ5kMI8A1n9Hg2kKc4pal-TGDJekeF_fZX-rwjOEVDgRv5ixJDLXWq_2MnfA4tV_Jga7iQ)
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
  Output: Document ID = 4CvReKKiYHnN_Riv1KouJLQwoeHF4rnPBPoUSP3G2kViOVJKLAIJfGZzEMaBCmOtPfEhP4wEahsuoRzrhusOGA
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: findById(4CvReKKiYHnN_Riv1KouJLQwoeHF4rnPBPoUSP3G2kViOVJKLAIJfGZzEMaBCmOtPfEhP4wEahsuoRzrhusOGA)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: findById(4CvReKKiYHnN_Riv1KouJLQwoeHF4rnPBPoUSP3G2kViOVJKLAIJfGZzEMaBCmOtPfEhP4wEahsuoRzrhusOGA)
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

> @ekodb/ekodb-client@0.9.0 prepare
> npm run build


> @ekodb/ekodb-client@0.9.0 build
> tsc


up to date, audited 58 packages in 1s

15 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.9.0 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning JavaScript client library examples...[0m

added 1 package, removed 1 package, and audited 9 packages in 831ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'TPCb9582fDTY8l2PKsj_vXJBYzlgZx0021RdNGRDT4sEmfYdoN56rkZXg2WbIXlUB61um-gzrF0Xiz_Uf2ggJQ'
}

=== Find by ID ===
Found: {
  active: true,
  name: 'Test Record',
  value: 42,
  id: 'TPCb9582fDTY8l2PKsj_vXJBYzlgZx0021RdNGRDT4sEmfYdoN56rkZXg2WbIXlUB61um-gzrF0Xiz_Uf2ggJQ'
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  id: 'TPCb9582fDTY8l2PKsj_vXJBYzlgZx0021RdNGRDT4sEmfYdoN56rkZXg2WbIXlUB61um-gzrF0Xiz_Uf2ggJQ',
  value: { type: 'Integer', value: 100 },
  active: { type: 'Boolean', value: true },
  name: { type: 'String', value: 'Updated Record' }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: wMX5zQ7xkpxhSpCj4xUxJGaT43LojJ2C_cy9mUYmYuL1fJNwuPQMc64gszraXXVuGo913-61sJW6yw091WNXCg

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
Collection created with first record: mjv6EfqlCCy-aBMvdxqwYO1VGQR_QlpT0N0rh6j-8EZVyVwunHKObI9wU9O7If108Fh-ngo57XCW80DhtL_Fww

=== List Collections ===
Total collections: 22
Sample collections: chat_configurations__ek0_testing,test_accounts,schema_users_client_go,batch_users,schema_users_client_py

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
cache:product:1: { price: 29.99, name: 'Product 1' }
cache:product:2: { name: 'Product 2', price: 39.989999999999995 }
cache:product:3: { price: 49.989999999999995, name: 'Product 3' }

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 0 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 14

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: false

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: Y0X2MdSmmqXnRg6yATHHC8XdOCVh3a2YSgNOVrZOepHLciIlz1xn8U87anhrPQ1HmMENbIrpC8D3p3azjf5d6w
Created Bob: $500 - ID: Jh8FwgeOKf7n5OTBImDnIKEFYqQEQgScqWA0gg2-Gv7df-nFRz6hbSQGQ8Q3tSlV8pjgblekorM44ofn4prk-Q

=== Example 1: Begin Transaction ===
Transaction ID: cb86b97d-89a1-47bf-a779-28a6137ef85e

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: b3f62f5a-d43b-4e67-87af-adea0abad796
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
  1. Score: 26.400, Matched: title, title.value, bio.value, bio
  2. Score: 26.400, Matched: bio, title, title.value, bio.value
  3. Score: 26.400, Matched: title, bio, bio.value, title.value
  4. Score: 26.400, Matched: bio, title, bio.value, title.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title.value, bio.value, title, bio
  2. Score: 39.600, Matched: title, title.value, bio.value, bio
  3. Score: 39.600, Matched: bio.value, title, bio, title.value
  4. Score: 39.600, Matched: title, bio.value, bio, title.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.763, Matched: 
  2. Score: 0.746, Matched: 
  3. Score: 0.728, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.699, Matched: content.value, content, title, title.value
  2. Score: 1.505, Matched: title, content, content.value, title.value
  3. Score: 0.291, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: bio, skills, bio.value, skills.value

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
✓ Inserted document: QFU-FDDLaU8TQ6wUO0uTnruZNQkFpVv5Jt4AMtjvOCX39cJuwa9ZIGSVuOJJgDswvJ0thU0Y3KNtTukgY4mwow

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: VsYFTEXqNvBvMDI16FnHeUdtZ2_z1QXrRRIlu9gHePZJUkdSigRPqfczDGufhZzCXFw7g5KqRIti0EPLkeL3tg

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
✓ Inserted document with TTL: R3VBFTGjwbGcEC81y7fRKO3Vm47kWmbKV-YfUnaZScuuBxrSHrVjIjBgsEWCvkHZgsbrVuZkO-6jbNlo79ntog

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
✓ Edge cache script created: 3JK6aQlXKiGv0xAJiBwDUmyq_DeEwLtLoE_r2u3ccompVnmlghZtM8Ykr0x5XYJnH2Ud9meIewfxaSQOn-v5SQ

Call 1: Cache miss (fetches from API)
Response time: 4ms
Result: {
  "records": [
    {
      "value": {
        "elevation": 32,
        "current": {
          "interval": 900,
          "time": "2026-01-27T17:00",
          "temperature_2m": -6.7
        },
        "timezone": "GMT",
        "utc_offset_seconds": 0,
        "longitude": -73.99308,
        "generationtime_ms": 0.02646446228027344,
        "timezone_abbreviation": "GMT",
        "latitude": 40.710335,
        "current_units": {
          "interval": "seconds",
          "time": "iso8601",
          "temperature_2m": "°C"
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
Response time: 5ms (0.8x faster!)
Result: {
  "records": [
    {
      "value": {
        "elevation": 32,
        "current": {
          "interval": 900,
          "time": "2026-01-27T17:00",
          "temperature_2m": -6.7
        },
        "timezone": "GMT",
        "utc_offset_seconds": 0,
        "longitude": -73.99308,
        "generationtime_ms": 0.02646446228027344,
        "timezone_abbreviation": "GMT",
        "latitude": 40.710335,
        "current_units": {
          "interval": "seconds",
          "time": "iso8601",
          "temperature_2m": "°C"
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

✅ Script saved: ssWyFgZDuNg9ZNcK0YfSoC-F8u9-FdRRK1B4jr-K2e6NOta0S-SnAvcFGJwAhVJWQ1UMjvDxwMCHcr3a8lFyNg
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
   {"count":5,"avg_score":60,"status":"active"}
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

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 4ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "phone": "1-770-736-8031 x56442",
    "username": "Bret",
    "website": "hildegard.org",
    "id": 1,
    "email": "Sincere@april.biz",
    "name": "Leanne Graham",
    "company": ...

Second call (cache hit - from cache):
   ⏱️  Duration: 3ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "phone": "1-770-736-8031 x56442",
    "username": "Bret",
    "website": "hildegard.org",
    "id": 1,
    "email": "Sincere@april.biz",
    "name": "Leanne Graham",
    "company": ...
   🚀 Cache speedup: 1.3x faster!

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

✅ Script saved: KJJ5GPr9tr_TmkUZbKJNTJk8NGS_ui4BiRv63MpiRtAwCFGR0ozYUAsKu9DuFyMzW0kcUa-_K_1aw5_vee6lmg
📊 Found 2 product groups
   {"avg_price":575.6666666666666,"category":"Electronics","count":3}
   {"count":2,"category":"Furniture","avg_price":474}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved
📊 Found 2 categories
   {"category":"Furniture","count":2}
   {"count":3,"category":"Electronics"}
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

✅ Inserted order: FZzEBx_Eu99ZKrLfF2u5GYMuagFPGgNtf8aSHGB44f-2enJjVYhRIFelJoFdpYsKSkqkyLX7FoaETjMQjxRwHg
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: fDj5KUFt4rcbMGmgxsmFr62AfrCYsCzb4dWx_lX0LU4-MtLdT72nnw5jG4T6xP-EtJKvSIoi2Wj7d1wFEMlPbg
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"role":"admin","userId":"user_abc"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: TZsu219POkA4m6flIx0z9aN7LLYmtlgTd1gS5cqJ0DrxbK-4vBF9cwYT464CLZ9wCSA3gDAMDyojxbskQaDz0g
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Script

✅ Script saved: Qz3aPGGOw00aXw1rmw4VMx-Xvk22_-2sDS5xt7nGXXZLRfLT-F3zw0-hUMdJpuQzgmgM5nFbwJmMWJuc7-Y2Fg
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
   1. User 2 - Score: 40
   2. User 5 - Score: 100
   3. User 1 - Score: 20
   4. User 3 - Score: 60
   5. User 4 - Score: 80
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
   1. Natural Language Processing (AI)
   2. Introduction to Machine Learning (AI)
   3. Vector Databases Explained (Database)
   4. Database Design Principles (Database)
   5. Getting Started with ekoDB (Database)
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
   1. Natural Language Processing
   2. Introduction to Machine Learning
   3. Vector Databases Explained
   4. Database Design Principles
   5. Getting Started with ekoDB
⏱️  Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Script saved
📊 Projected documents (showing first 3):
   1. Natural Language Processing
   2. Introduction to Machine Learning
   3. Vector Databases Explained
⏱️  Execution time: 0ms

📝 Example 5: All Document Fields

✅ Script saved
📊 All documents (5 total, showing first 2):
   1. Natural Language Processing (AI)
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
   2. USB-C Cable - $19 (⭐4.3)
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

1. High Precision: They maintain high accuracy of data as they use points, lines, and polygons to represent data.

2. Scalability: Vector data scales effectively without losing quality as they are not pixel-based.

3. Detailed Analysis: They are suitable for complex spatial analysis including proximity and network analysis.

4. Less Disk Space: They usually require less storage space compared to raster databases.

5. Better Representation: They are ideal for representing features that have discrete boundaries like buildings or roads.
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
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_api_user_js (3sHyn2tYqqTogBAmFbeyhLTnXj_Q-3gyq9Nu43SzZgql3U7DM8PfwJtdP7Du351WNZmSWGomCEbGcLM1_KAPrg)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "website": "hildegard.org",
        "phone": "1-770-736-8031 x56442",
        "name": "Leanne Graham",
        "address": {
          "city": "Gwenborough",
          "geo": {
            "lat": "-37.3159",
            "lng": "81.1496"
          },
          "street": "Kulas Light",
          "suite": "Apt. 556",
          "zipcode": "92998-3874"
        },
        "email": "Sincere@april.biz",
        "username": "Bret",
        "company": {
          "name": "Romaguera-Crona",
          "bs": "harness real-time e-markets",
          "catchPhrase": "Multi-layered client-server neural-net"
        },
        "id": 1
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
Response time: 6ms (served from cache)
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
✓ Created session: 5_2pt119763cuAEGndpFXUXO-G_caHq6R576pVmjxqGlGa2Iite-90L1dH1QqBL-4XPGkq96Hc1auCIXEn6akA

=== Sending Chat Message ===
Message ID: L3v-_CpIAhmrSwaoZPysRhFQVcMsEFtRjx-T7xOIDk5c-40-uaOPOak7OG7N9691o2Tf90i05bYdUMazkSaPhA

=== AI Response ===
There are three products available:

1. The ekoDB Cloud is a fully managed cloud database service product, priced at $499.
2. The ekoDB Pro is an enterprise edition product with advanced features, it is priced at $299.
3. The ekoDB is a high-performance database product with AI capabilities, and it is priced at $99.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_js',
  record: {
    description: 'Fully managed cloud database service product',
    price: 499,
    id: 'DKMFHGk_NsyTy35-c8yPKIw_YJYz8ghwaAzr-puXf7A67yLtDT6pTvwnf0SU4V_Bv-T8YxpskJ1dr9fdz5W5MA',
    name: 'ekoDB Cloud'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_js',
  record: {
    price: 299,
    id: 'MpAQDxHDLoUnHsD7rDiMnikWWKrCcU3-eJ0pGH62EmA7i15Ck0QJL9YCRHE0EaWZjPfFnTwpTBP9MhWBja2tdg',
    name: 'ekoDB Pro',
    description: 'Enterprise edition product with advanced features'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_js',
  record: {
    name: 'ekoDB',
    price: 99,
    id: 'AE-zBmxJrtX_gtdLUZTp2hMOAKfR5kiJZKcut5LX2QdDwVbXigKP3KoK6SG4tuppzAZfHUQNRvDciiQDMR5q_Q',
    description: 'A high-performance database product with AI capabilities'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 2668ms

=== Token Usage ===
Prompt tokens: 450
Completion tokens: 74
Total tokens: 524

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: hw6Lp89Be2xUM74GjA0jdb9fCJa9IHlkTcv0l812tG2uD3hH_VstkMMk5clUdpdeTt9F1VrxM68naLtFP5deZQ

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information provided, there is a product available named "ekoDB". This is a high-performance database product. The price for this product is $99.

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
✓ Created second session: G38RUkxNovijCF-8DGs1rQKzXbWxCvY-d73y3PZ_IbWl2T5m6-A2BCT6eHscs9iiXeTBsFeFjXNzmxsATtD1AA
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
✓ Created session: eMVgQibNVejyIZvXZ4r7pE2uJJPUMKTJdHepMPv6j4LFv64F15t3XEQA2J-5OldelTiPGkTIj7Q_y2Nl8_gVpw

=== Sending Messages ===
✓ Message 1 sent
  Response: The product available is called "ekoDB". It is a high-performance database product priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: PnvhTm232pozzgENm23t8mQhO7pWC81m-6Q8VTTcFHGXT_yI5nVRCqEuPtAlDUG39y0No8PKt3WpxLthM0naOw
  Parent: eMVgQibNVejyIZvXZ4r7pE2uJJPUMKTJdHepMPv6j4LFv64F15t3XEQA2J-5OldelTiPGkTIj7Q_y2Nl8_gVpw

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: PnvhTm232pozzgENm23t8mQhO7pWC81m-6Q8VTTcFHGXT_yI5nVRCqEuPtAlDUG39y0No8PKt3WpxLthM0naOw (Untitled)
  Session 2: eMVgQibNVejyIZvXZ4r7pE2uJJPUMKTJdHepMPv6j4LFv64F15t3XEQA2J-5OldelTiPGkTIj7Q_y2Nl8_gVpw (Untitled)
  Session 3: G38RUkxNovijCF-8DGs1rQKzXbWxCvY-d73y3PZ_IbWl2T5m6-A2BCT6eHscs9iiXeTBsFeFjXNzmxsATtD1AA (Untitled)
  Session 4: 5_2pt119763cuAEGndpFXUXO-G_caHq6R576pVmjxqGlGa2Iite-90L1dH1QqBL-4XPGkq96Hc1auCIXEn6akA (Untitled)
  Session 5: 1LfAHPb4XlXtdOzun-Pt8V8ShLVy3wFUaQFnCwK6m_wc_1ggMQ-sf8rmo3QKUKyvLkjp98CZH7KUdjax_4Pesg (Untitled)
  Session 6: viUGfP54D7GHkKPi8-y4oIu0sEsSzcUcy7IW8t0kM-CkKDtnbvxdblIVCqmWuoPT_hcFzoh6yh4VMXX97NNiTg (Untitled)
  Session 7: Rfcw3lOe2ZuFSzTrMCeubJP7IxjrSY6z_Dw9BiTfwgmTiOsSQNFOgu35bVOjFkpnyOWlF1TL_fX6LZLCFNyVbw (Untitled)
  Session 8: -x18cYo6AhFsWkJX4FyQFZUmxRdlu3Y5twqrLatXf6lKNHYH1n0a78OTtK_NtEP8P6K8SOBqyWoId_5FT-fAJQ (Untitled)
  Session 9: vCj_jR9APFVECAEupGE-eShauY1CU4zi6rs881YufAEuruBTM5UiMLTrGofiOuPSc5t7QN0jtSJMOoTnG5FXbw (Untitled)
  Session 10: H5TOVzzNxakRARI8q0N8aI6MXf9khSLO60NUFeIYlRi4rNsHQf9Ft3xqdUkG0HrNylE4YlWGD3w9dHAkqNo0Ig (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: PnvhTm232pozzgENm23t8mQhO7pWC81m-6Q8VTTcFHGXT_yI5nVRCqEuPtAlDUG39y0No8PKt3WpxLthM0naOw

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
✅ [32mJavaScript client examples complete![0m
