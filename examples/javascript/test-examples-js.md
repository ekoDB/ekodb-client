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
  id: '1yGC8ls4Jm_2YtjV_S4E-Zl38Ke3cEs8FCwzleB1AQ6L2hTl_Ik8Ii-oIJMfWQoOH3ldVW82KeCDqGkDW-CcEw'
}

=== Find by ID ===
Found: {
  name: { type: 'String', value: 'Test Record' },
  value: { type: 'Integer', value: 42 },
  id: '1yGC8ls4Jm_2YtjV_S4E-Zl38Ke3cEs8FCwzleB1AQ6L2hTl_Ik8Ii-oIJMfWQoOH3ldVW82KeCDqGkDW-CcEw',
  active: { type: 'Boolean', value: true }
}

=== Find with Query ===
Found documents: [
  {
    name: { type: 'String', value: 'Test Record' },
    id: '1yGC8ls4Jm_2YtjV_S4E-Zl38Ke3cEs8FCwzleB1AQ6L2hTl_Ik8Ii-oIJMfWQoOH3ldVW82KeCDqGkDW-CcEw',
    active: { type: 'Boolean', value: true },
    value: { type: 'Integer', value: 42 }
  }
]

=== Update Document ===
Updated: {
  id: '1yGC8ls4Jm_2YtjV_S4E-Zl38Ke3cEs8FCwzleB1AQ6L2hTl_Ik8Ii-oIJMfWQoOH3ldVW82KeCDqGkDW-CcEw',
  active: { type: 'Boolean', value: true },
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
✓ Inserted test record: QJCNFcYllGV1u5PgqeAkjV1AKnsr8BxfVFAqifAwSFRwqlTBzqLqLqj6ayKsH1tfb4khvTj-5tzxUv_wTB2XOg

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
        "id": "I3WYB2cMD26_B0-wQWqrKlff0C5PZt29GeDA2MvMqo4WcLeCF83k2bC-8qtWwz80wWbycPo0nlebTCTsI7XEbA",
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
        "id": "nrFky7TUpLwkxOlhaCDSUe0PCao8kfz0iZYaDndFzDJ96Kl38Z2qDWBwYQYTAPkaH5E4IT8NKcJugA_gXfF4vA",
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
        "id": "dXQAu1HuEjtk6dl1FYNhoMg00e2HKQHKOuJjOqzonYjW7sA1xwZw30KQegVqqknaBIdW4IaU1YGZtxyXHMxyxg",
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
        "id": "QJCNFcYllGV1u5PgqeAkjV1AKnsr8BxfVFAqifAwSFRwqlTBzqLqLqj6ayKsH1tfb4khvTj-5tzxUv_wTB2XOg",
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

✅ Script saved: gAA-Zwy9CA2kN4Phv-mVhhxwpN7GqS2b9akOoOdDDWa5OyaCGUKr4vf5CINt6MZ2hAGJcfSg61aiiYP3GW_YlA
📊 Found 10 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: KtI6Jqm-lQjC5hEgpwwnTfzcvXnnoiKRuZa8OsjUcyd-vBrAMZEydKk0X-z-d-fR0q7SqrN9QisviNzHe08-Ng
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: cvzOR6F3pppGO25ncrIsWqAWgZTPkR8RqUrieqxlt_4LgXG1pd704xzv8jo7Nn5kDoRo6WaqrIPfPpzhHi0yDA
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"avg_score":60,"count":10,"max_score":100,"status":"active"}
   {"count":10,"status":"inactive","max_score":90,"avg_score":50}

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
Collection created with first record: aBMZ_8V6S8sblXYwJj55BJ7GZDtU6RazmLoowtOc7hJ-Eh8npEQYQqwYmlignjAppPobeSqd47B-c-p0O9ONZg

=== List Collections ===
Total collections: 25
Sample collections: [
  'demo_collection',
  'batch_users',
  'schema_documents_client_py',
  'users',
  'schema_employees_client_py'
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
Created Alice: $1000 - ID: 2GrSOw6WBzYX8JyhRajAQd4PvHhOIHMhvfYpvIqQ-MiHJ2I6s1kntLi7wr0vV7ZyO1qEY9Brk6vTTdBvfyVcPQ
Created Bob: $500 - ID: C-cgojHTzmOu2uXNMRZp0uMWN60zeRFsgMO47_OLOfKr93B9kA5pRxOQZ3xKLqpgrjThCK34q9xTNP7gMwbAaQ

=== Example 1: Begin Transaction ===
Transaction ID: 7e03a0ff-1dbc-41a0-b16c-69e374ec1478

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
New transaction: a3b9a7f2-2baa-4c3d-87cc-18c4ccb77f61
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: {"value":700,"type":"Integer"}

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
   ✅ Script saved: XNTDb95pmihjKG9bfhZ-hHF-vFcZM-xdY2HjDN0mPFfWGXmHPq-moPolNHrAzZoc42Gao_XsfT5bLWqMqcIoAg

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
   ✅ Script saved: NEq3va2v7-0Lop4VKEKWC3t_sev_pn2ydxglxNDX2qrVF8m-1RP25rp1POwPFEBhVgEzwt8JJpCbioRqMXmarA

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
   ✅ Script saved: f60mA-Wv5JqFqaV5r7bcPXKNCWhwW1-EUyX4RY4QgFttlHdp-0GhvYcKKaPEKW9bH6EOT4Osh_3umwQ_VJGlig

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
   ✅ Script saved: 5psMBQEdKPnhgKXsYYdQGDoLOX6NvbCZ3kQArh7liQMzG3-iPD79AgpyYpeGzkTd7kzLTgUBEYE9WpEzP_ipTQ

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: XNTDb95pmihjKG9bfhZ-...
   ✅ Deleted script: NEq3va2v7-0Lop4VKEKW...
   ✅ Deleted script: f60mA-Wv5JqFqaV5r7bc...
   ✅ Deleted script: 5psMBQEdKPnhgKXsYYdQ...
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
  Output: Document ID = gWHKXFcKJlRQQB1R2MaLwPJcfjZ7sFAhCIHeg8jaW8sB7o7bUJQaAkLl_XYtNbTR8PKATg9W_v4dt4nf0VJ0Fw
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: findById(gWHKXFcKJlRQQB1R2MaLwPJcfjZ7sFAhCIHeg8jaW8sB7o7bUJQaAkLl_XYtNbTR8PKATg9W_v4dt4nf0VJ0Fw)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: findById(gWHKXFcKJlRQQB1R2MaLwPJcfjZ7sFAhCIHeg8jaW8sB7o7bUJQaAkLl_XYtNbTR8PKATg9W_v4dt4nf0VJ0Fw)
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
  Output: Document ID = hC8ZB3qhP8_TrHBjiZbALPw--GYTbd8cEFfe6TCiJjdo4bmA2D4QhdfjOJ_KS_TC23MS4cf0Kc_EAKEGVR5WWw
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: findById(hC8ZB3qhP8_TrHBjiZbALPw--GYTbd8cEFfe6TCiJjdo4bmA2D4QhdfjOJ_KS_TC23MS4cf0Kc_EAKEGVR5WWw)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: findById(hC8ZB3qhP8_TrHBjiZbALPw--GYTbd8cEFfe6TCiJjdo4bmA2D4QhdfjOJ_KS_TC23MS4cf0Kc_EAKEGVR5WWw)
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

> @ekodb/ekodb-client@0.13.0 prepare
> npm run build


> @ekodb/ekodb-client@0.13.0 build
> tsc


up to date, audited 53 packages in 1s

17 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.13.0 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning JavaScript client library examples...[0m

added 1 package, removed 1 package, and audited 9 packages in 973ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'LfDMNLtFlpJ7Gb6Cl9copfzcQSwlJrz3oaIGItUwHu_2FxMzHorRnzDhNk2M7dkLK2z5n04MIGn3d-tCluVtsw'
}

=== Find by ID ===
Found: {
  value: { type: 'Integer', value: 42 },
  id: 'LfDMNLtFlpJ7Gb6Cl9copfzcQSwlJrz3oaIGItUwHu_2FxMzHorRnzDhNk2M7dkLK2z5n04MIGn3d-tCluVtsw',
  active: { type: 'Boolean', value: true },
  name: { value: 'Test Record', type: 'String' }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  name: { value: 'Updated Record', type: 'String' },
  value: { value: 100, type: 'Integer' },
  active: { type: 'Boolean', value: true },
  id: 'LfDMNLtFlpJ7Gb6Cl9copfzcQSwlJrz3oaIGItUwHu_2FxMzHorRnzDhNk2M7dkLK2z5n04MIGn3d-tCluVtsw'
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: 6-Rw6pOPj6MbGt1Laa3qkkJcUjIZt08-nQzsqcT-pBKZQwjhWrmb7eozrfRx5vkdcHtY0SAGIEvelLLvUQz7Mg

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
Collection created with first record: JFquGB6ANqsJ2zNE0r2yXDa4ElHj2CnlInmpDe0b4K4hSizyKFD4Z_2-UQEx_zyFcSbXI0zc4_BLPaTHmUosyQ

=== List Collections ===
Total collections: 24
Sample collections: batch_users,schema_documents_client_py,schema_employees_client_py,schema_employees_client_go,chat_configurations__ek0_testing

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
Total keys in store: 17

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: false

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: N6aLp9G-S54YTxI96fFAiMhzFtMMQN2TnST2jomlZ8f26HgU1b2uXyYyRMSbf0-Aznd1SmAiMfc4f98VtGGSrg
Created Bob: $500 - ID: mbIp-rTfuOqaP-Lc-7RukVLdiNi99eqCHRvuePlNZ5kfM300L2dG3pLevjMckdwbWFk3kocp0xw3uH0D-qM99w

=== Example 1: Begin Transaction ===
Transaction ID: 0258a0a7-0d54-4910-b74b-59e40fc1348b

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 7e05186e-d426-45fb-85a9-374b11378832
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
  1. Score: 25.740, Matched: name, email, email.value, name.value
  2. Score: 12.540, Matched: name.value, name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title, title.value, bio, bio.value
  2. Score: 26.400, Matched: bio.value, title, bio, title.value
  3. Score: 26.400, Matched: title.value, bio, title, bio.value
  4. Score: 26.400, Matched: bio.value, title.value, title, bio

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title, bio.value, bio, title.value
  2. Score: 39.600, Matched: title.value, bio, title, bio.value
  3. Score: 39.600, Matched: bio, title.value, title, bio.value
  4. Score: 39.600, Matched: bio, title, title.value, bio.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio.value, bio

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio.value, bio

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.795, Matched: 
  2. Score: 0.770, Matched: 
  3. Score: 0.748, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.718, Matched: content.value, content, title.value, title
  2. Score: 1.508, Matched: title, content.value, title.value, content
  3. Score: 0.299, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: bio.value, skills, skills.value, bio

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
✓ Inserted document: pXp4am1Qvaza9I4EaPbvOvCJWoTYifrxPS8vbxvJs5pWSZ9kjrtUgjBgR7aSPQgEvfhSbLN65kxELcHgigLE7Q

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: IPWOUkH7AUQgvhBUMW5humis4T2WTGYvuhgnMwh_l56U88z7O1L2UYa2lyVTz1ssa61KsUZ4rkpGDP4c472Wnw

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
✓ Inserted document with TTL: CSA9LL9UF2oyaCwp9pHvB_aE0cqNJ_3CXrqbCID_SfeNBRHAjcQr9mS2dJISnIju6kNGhYo01QAFVnwMvxIrSA

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
✓ Edge cache script created: 310v-3z5LmBNWQ-tpQh3_RW_Gf1MEALJerRdKWRD8JYkmdwb29lqzXVf23C1KfAd1qFlksJNGTbTzMG58e6U0g

Call 1: Cache miss (fetches from API)
Response time: 3ms
Result: {
  "records": [
    {
      "value": {
        "latitude": 40.710335,
        "elevation": 32,
        "current": {
          "time": "2026-03-22T16:45",
          "interval": 900,
          "temperature_2m": 10
        },
        "utc_offset_seconds": 0,
        "current_units": {
          "temperature_2m": "°C",
          "time": "iso8601",
          "interval": "seconds"
        },
        "timezone_abbreviation": "GMT",
        "generationtime_ms": 0.02181529998779297,
        "longitude": -73.99308,
        "timezone": "GMT"
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
Response time: 4ms (0.8x faster!)
Result: {
  "records": [
    {
      "value": {
        "latitude": 40.710335,
        "elevation": 32,
        "current": {
          "time": "2026-03-22T16:45",
          "interval": 900,
          "temperature_2m": 10
        },
        "utc_offset_seconds": 0,
        "current_units": {
          "temperature_2m": "°C",
          "time": "iso8601",
          "interval": "seconds"
        },
        "timezone_abbreviation": "GMT",
        "generationtime_ms": 0.02181529998779297,
        "longitude": -73.99308,
        "timezone": "GMT"
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

✅ Script saved: HYZKwS71J9dzxXnxOwpUkhhP6Zl69Ki9A5tjNfsqFmxcfMmofORf-WeuqC581C63ND-5_ZqGmnpUDpv1RxYwmA
📊 Found 10 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved
📊 Found 10 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved
📊 Statistics: 2 groups
   {"avg_score":60,"status":"active","count":5}
   {"avg_score":50,"status":"inactive","count":5}
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
    "name": "Leanne Graham",
    "username": "Bret",
    "phone": "1-770-736-8031 x56442",
    "address": {
      "zipcode": "92998-3874",
      "city": "Gwenborough",
      "geo": {
  ...

Second call (cache hit - from cache):
   ⏱️  Duration: 3ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "name": "Leanne Graham",
    "username": "Bret",
    "phone": "1-770-736-8031 x56442",
    "address": {
      "zipcode": "92998-3874",
      "city": "Gwenborough",
      "geo": {
  ...
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

✅ Script saved: 6YJH0LRXwmgekFVu0R8kw9QK8SPyhf6ZNnhfxZuHV6VvE4oWCvWaXz4XuBlnhANU97_iX0GdzVxp5NRzcynTvA
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
   {"count":2,"category":"Furniture"}
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

✅ Inserted order: 7YSPbpUcoDlHZCXJLS9Pegl6aDKahQAMVqTkLh_OJHLBH_cI-GDVWMzcVGWTC92Uc-c_hi_A31Sci0AGk6Yq8Q
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: wttWOSZ5qa4dQv6N0knqJYxrylJ_gjq8oxBByz6O9pN7B8veqJnJLeSWjOE-W8lanCufl_Z0kIrUlgAT1szAXw
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"role":"admin","userId":"user_abc"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: b8Ci0QtOC5OTP9IlhPvosiApmx9eDegwF92sk0m_jcv8Rpd3Q_tuVvPlwlld4Pw0N2hnIm9lmeK45Jjlgl8Jgw
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Script

✅ Script saved: DHx6Me_iF2GnhbsvJ-kILNqc2Po1_YtTDJu9mIKa9B4sP5HKYVg77ed5oYdMJmfVOQdACtxfbNr9rvaAuU1zQg
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
📊 Users (showing first 5 of 10):
   1. User 10 - Score: 100
   2. User 8 - Score: 80
   3. User 1 - Score: 10
   4. User 2 - Score: 20
   5. User 4 - Score: 40
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
   1. Natural Language Processing (AI)
   2. Vector Databases Explained (Database)
   3. Getting Started with ekoDB (Database)
   4. Database Design Principles (Database)
   5. Introduction to Machine Learning (AI)
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
   2. Vector Databases Explained
   3. Getting Started with ekoDB
   4. Database Design Principles
   5. Introduction to Machine Learning
⏱️  Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Script saved
📊 Projected documents (showing first 3):
   1. Natural Language Processing
   2. Vector Databases Explained
   3. Getting Started with ekoDB
⏱️  Execution time: 0ms

📝 Example 5: All Document Fields

✅ Script saved
📊 All documents (5 total, showing first 2):
   1. Natural Language Processing (AI)
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
   Furniture:
      Products: 4 | Stock: 43 | Avg Rating: ⭐4.26
   Electronics:
      Products: 6 | Stock: 232 | Avg Rating: ⭐4.52

⏱️  Total execution time: 0ms
📈 Pipeline stages:

📝 Example 5: Project Specific Fields

✅ Script saved
📊 Product summaries (10 items, showing first 3):
   1. Webcam HD - $119 (⭐4.5)
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
   1. High Precision: Vector databases store data as points, lines, and polygons, allowing for precise representations of features.

2. High Quality Visualization: They provide a clear and realistic display of geographic data.

3. Attribute Storage: They store attribute and spatial data seamlessly.

4. Scalability: Vector data maintains its quality when scaling up or down in size.

5. Advanced Analysis: They allow for complex spatial and network analysis such as finding nearest neighbors, shortest path, etc.

6. Less Storage Space: They generally require less storage space than raster databases for the same level of detail.

7. Topology: Vector model accurately represents topological relationships between different geographic features.
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
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_api_user_js (1tBlucZea6dFURpcIGHu8cDSfiL7BX_9sw1M1L7QEYUQaTgOTZWZM1hpmJVuunjX5rB3IZrX4zWrKAARW08Oxw)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "website": "hildegard.org",
        "phone": "1-770-736-8031 x56442",
        "company": {
          "bs": "harness real-time e-markets",
          "name": "Romaguera-Crona",
          "catchPhrase": "Multi-layered client-server neural-net"
        },
        "email": "Sincere@april.biz",
        "username": "Bret",
        "id": 1,
        "address": {
          "suite": "Apt. 556",
          "city": "Gwenborough",
          "street": "Kulas Light",
          "zipcode": "92998-3874",
          "geo": {
            "lat": "-37.3159",
            "lng": "81.1496"
          }
        },
        "name": "Leanne Graham"
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
Response time: 9ms (served from cache)
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
✓ Created native SWR script: github_user_native (3-56-U1l0r8vqCh9pEaLQOq8qWkV2hFdBMM8s0ipkRZ4uBeHklUjtkVzV9uplwElV-JEeJQJ7SpYZ2pStnIrOA)

First call (cache miss - will fetch from GitHub API):
  Response time: 4ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 3ms
  Speedup: 1.3x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (T3eDWMTXgW6nKGiT7hqq9Swh3yQr1VcKUktbm-YRb3oNXI4nBZ9hKd4xrEkmKfgLTe91Kf1RFQXiRaIi42dgng)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (b4N-c6dQUocc3VZj1kZfa2MJwtj1caIAs7cKUwv5gy1LRZYnFddwGpvDxmFymgFDBQ518i5V1ec-XCnr4icWmA)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (NlSOJP6rSQWxN2IwH_5cauDX4X01RuKfVlWOSa7jxQdPRMBO0YctQpsp0WipPbyA_vkXPww7Uyhz7okqFiY1-w)
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
✓ Created session: APL51N5kWo96oL7tll-9jE9l6X9YzkPwISnKNXzzQtjT-qR_7kmmhxRMg1I-KqoDOE6oIAr7TIMrYYikvJUUxA

=== Sending Chat Message ===
Message ID: l_BDcZZ5Xa-YMoVeXTuHCrrnjRMdDK0bt5io258-B6MqPslS2kmO6VUN2OPM01TfPwSkZQrGYXsj3HaPmIITUA

=== AI Response ===
Here are the available products along with their prices:

1. **ekoDB Cloud**
   - **Description:** Fully managed cloud database service product
   - **Price:** $499

2. **ekoDB Pro**
   - **Description:** Enterprise edition product with advanced features
   - **Price:** $299

3. **ekoDB**
   - **Description:** A high-performance database product with AI capabilities
   - **Price:** $99

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_js',
  record: {
    description: 'Fully managed cloud database service product',
    name: 'ekoDB Cloud',
    price: 499,
    id: 'BJfLSDQ3MIxmGM2xgz-XnNxkmDOpqWioiqQQoO0hq2aJp557Af1J4b5aQtfUBhj7QH6uUGHQZ9KXOXlHinHoPw'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_js',
  record: {
    name: 'ekoDB Pro',
    id: 'pYuxcBprxND4Y51yFQ-G4DjNbboLBqktyuGyWP_l77yrQ_rj-mdrZIyIqmcvd7vzhAqNuTgPx1hos2A3KmYwhw',
    description: 'Enterprise edition product with advanced features',
    price: 299
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_js',
  record: {
    price: 99,
    description: 'A high-performance database product with AI capabilities',
    name: 'ekoDB',
    id: 'rZ1ElA4R0w7Fr9CZACHnKPQuY7moMvSowvgoAltemreqNLHjd5h4PgIgmq3jMX3Npvteza_6jUbS_MOTPcrKrQ'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 5574ms

=== Token Usage ===
Prompt tokens: 840
Completion tokens: 97
Total tokens: 937

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: j5b2z_uWFXAzYCXGbNPxGhPrEZuVFTPZ8iQR1BErE9VVNQNlC6oGR_i5K_w3D8LBnAHQ9Fq9RoE87wyfEHcW6w

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the database information, there is one product available:

- **Name**: ekoDB
- **Description**: High-performance database product
- **Price**: $99

If you need more details or information about other products, please let me know!

✓ Second message sent

=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of ekoDB is $99. If you have any other questions or need more information, feel free to ask!

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: FHjnii8nvcX8jObiC19Oy01DnQ0LUOSZKwkdsNBt7W0PIknYVv5qy3CE8TXHBzl4VfR93XL3W2zmYdW32M5QUw
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
✓ Created session: IddqMYNhfq8HS6_ZWjKwTVW0KmcBemTv_48U8ggqzIHHD386CQ3cbVDdjeAEr9-zK3jE4y6PYhgkt88pEH-o6g

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name**: ekoDB
  - **Description**: A high-performance database product
  - **Price**: $99

✓ Message 2 sent
  Response: The price of ekoDB is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: kT38dFiy2-GEhtdw6uuX6nIBOh_RT_3wJ2VgYyHzC7MibnCpt1bwUzbtWn-4hs5c26MAtahpfNv94z6bSSrAHg
  Parent: IddqMYNhfq8HS6_ZWjKwTVW0KmcBemTv_48U8ggqzIHHD386CQ3cbVDdjeAEr9-zK3jE4y6PYhgkt88pEH-o6g

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: kT38dFiy2-GEhtdw6uuX6nIBOh_RT_3wJ2VgYyHzC7MibnCpt1bwUzbtWn-4hs5c26MAtahpfNv94z6bSSrAHg (Untitled)
  Session 2: IddqMYNhfq8HS6_ZWjKwTVW0KmcBemTv_48U8ggqzIHHD386CQ3cbVDdjeAEr9-zK3jE4y6PYhgkt88pEH-o6g (Untitled)
  Session 3: FHjnii8nvcX8jObiC19Oy01DnQ0LUOSZKwkdsNBt7W0PIknYVv5qy3CE8TXHBzl4VfR93XL3W2zmYdW32M5QUw (Untitled)
  Session 4: APL51N5kWo96oL7tll-9jE9l6X9YzkPwISnKNXzzQtjT-qR_7kmmhxRMg1I-KqoDOE6oIAr7TIMrYYikvJUUxA (Untitled)
  Session 5: iyFUX2cxcYpZjLC-aO8fe1MesTruIsYcWvaJcqe7grtxkIr7zf1qZv7m91QK4wHUelLrwZPQr1_A0N3fWKGVPw (Untitled)
  Session 6: bKuI4y_8m5uyFAaEfVS3PaOVxD7GjZ2RfSyAwirMSjpAY2aSjEvsK8V8bDcXP8XOEc-UXGCrUMSc1WAvIL7CKg (Untitled)
  Session 7: p5RD3hp1iBkXTpUnJyvIRLM9ya3BgxrJnXngS3I3gnsqYIyhzLN8_CG0naNZ1ltBiQgDkP7DoFR6gLYqFQWxdg (Untitled)
  Session 8: Yc_a7W4O9604Zk9lNVFl_NqbkG7vUB5Osr6abi8__5cRWYDqxRX6jYXgRIWOT8ISTdXTjI9IBKasVihl5pY8Hg (Untitled)
  Session 9: JWnsfRU1jdDeaYWO7MdIDo9H4YkYxvm1Z00Cjh_jbt69CPNJ8TRyt8zYTcitiNV3T7gtZp6rlOi9D-Rbny5VXw (Untitled)
  Session 10: OeeHcsFKYGiGZUjmxkrM2k2ObJVz-EvfRoRycmed4KJjbmLsZhBcBd-yUwujh-jicf84Ez3OdkehS9FFkbuYCw (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: kT38dFiy2-GEhtdw6uuX6nIBOh_RT_3wJ2VgYyHzC7MibnCpt1bwUzbtWn-4hs5c26MAtahpfNv94z6bSSrAHg

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
✅ [32mJavaScript client examples complete![0m
