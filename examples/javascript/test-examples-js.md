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
  id: 'zH_6umpXI5ZjuLMPEts7dKa7Hpx2aCaU7eV4SAoQKv48Tu7tA8qUS9MdBp1pnA5S7uRfq_HJJ2xpmreMFpEekA'
}

=== Find by ID ===
Found: {
  name: { type: 'String', value: 'Test Record' },
  active: { value: true, type: 'Boolean' },
  id: 'zH_6umpXI5ZjuLMPEts7dKa7Hpx2aCaU7eV4SAoQKv48Tu7tA8qUS9MdBp1pnA5S7uRfq_HJJ2xpmreMFpEekA',
  value: { value: 42, type: 'Integer' }
}

=== Find with Query ===
Found documents: [
  {
    active: { type: 'Boolean', value: true },
    name: { type: 'String', value: 'Test Record' },
    value: { value: 42, type: 'Integer' },
    id: 'zH_6umpXI5ZjuLMPEts7dKa7Hpx2aCaU7eV4SAoQKv48Tu7tA8qUS9MdBp1pnA5S7uRfq_HJJ2xpmreMFpEekA'
  }
]

=== Update Document ===
Updated: {
  active: { value: true, type: 'Boolean' },
  id: 'zH_6umpXI5ZjuLMPEts7dKa7Hpx2aCaU7eV4SAoQKv48Tu7tA8qUS9MdBp1pnA5S7uRfq_HJJ2xpmreMFpEekA',
  name: { type: 'String', value: 'Updated Record' },
  value: { type: 'Integer', value: 100 }
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
✓ Inserted test record: 2DOD8LoFM1YcP6TpYzByDNQGvsYlgWHo0ilW5RnPTJqXGI0dcv82AWI7r7_3BRdZkx7OAc_PIiAEtjWFUAZoKw

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "messageId": "1775447629564",
  "payload": {
    "data": [
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "XIRI1bUm4XF0vKdRdr_ajMzn25G7r7wEIh_A79ht4EHm1G0QAT709vSsKX2lA0f2my_sOrIuvmhdEhrSnCBiGA",
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
        "id": "bHP1ni5oaE0HgsNvnFzZnpbpJK6MKlPwmNaxhEk4ZOLvCXyckqpXu5c6bKDkDawxQ6Fp8hrQ-iqhLJcf8dZ1lA",
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
        "id": "2DOD8LoFM1YcP6TpYzByDNQGvsYlgWHo0ilW5RnPTJqXGI0dcv82AWI7r7_3BRdZkx7OAc_PIiAEtjWFUAZoKw",
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
        "id": "YqPd6d25_D4tl6fKjbje3AExz8arcSNbcmeUwMRvFm_j2tAzd2q0DMFW65DwWrroSYcvnXyFUI5AqnR8pXWDOg",
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

✅ Function saved: 7NuK91hGg7vyIlzUSurvMrl2SCBHRdynTVjfZlBdx-p4i2RTzxocoo8NAOVw_h5iZRqBr1C8BzdPqnsaCawbBg
📊 Found 10 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Function saved: 5jdEmB_OU0Nv0vnMFlVTKKOt-8GlDzRB6S6sMUmIJlgx-kVqzaFW5DeqavjStLSg8agbR2anlzY094vaQhmmbw
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: wajVvZXlzN7jFlVSxibiv1PX0QzZyEcrBK_KVk0t0msTa0Xj2pVomnWDUnO5mThAXQEUdZzpzmwZXmVqUffIXw
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"avg_score":50,"count":10,"max_score":90,"status":"inactive"}
   {"status":"active","max_score":100,"count":10,"avg_score":60}

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
Collection created with first record: iwUoV0UolZeJKnzbR-hRzcQjtOKiPqRWawwX3-pBdIoHfCtjbgHJDOLxaeeZ3BfSzQA-smMNAAyMxKP2p2We2g

=== List Collections ===
Total collections: 50
Sample collections: [
  'audit__ek0_testing',
  'cached_database_metrics',
  'schema_products_client_go',
  'enriched_users',
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
Created Alice: $1000 - ID: fZ2IaVvUGtqfi8JnrpNwaYFAqvOITCd6mXPmVy1d1gCLc5DM8aj5GFlhN95ZRNbsLqJmN8iW4pOfrWIgS56KLg
Created Bob: $500 - ID: jggeJ00B9M7gNt1ZZpHrpqHyXQz6pkM4s0YLzzjlYvZuqWnPwbjGZQjILAOfBsBA9XEnQH1AfspVlfV3XyebtA

=== Example 1: Begin Transaction ===
Transaction ID: 3fa76e62-45c3-437d-9176-7177de68df1c

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
New transaction: ef34157e-0ac1-44e3-8dc5-fb2784f8aa38
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
   ✅ Function saved: GryGx93TzJwgKvmb31_nUOwOFUe7rViK3VelyPfXYgPcbNVE52o_rjtifARqFydfm7HXGWQwOVVf09CMvYnGwA

2️⃣ Calling Function (Insert + Verify)...
   ✅ Function executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Name: [object Object]
   📋 Email: {"value":"alice@example.com","type":"String"}
   📋 Status: [object Object]
   📋 Credits: [object Object]

============================================================
📝 Function 2: Query + Update + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: k5bk6N0noOhaPIito0y6LZYfY3MEfFKHmNwJYf_jnjbL99eRnHwXeHt9kiKqnaNRSswnRr0m6GwF3uYqPmhLNg

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
   ✅ Function saved: l2SOKpPtNCqDwmfwQF_ilZTh3LNJTIMm7jnFjgqUN6S0Hi2oPRssN7ZIpddCfB51lC-LcwkXg-MZYZHPB_aivA

2️⃣ Calling Function (Query + Update Credits + Verify)...
   ✅ Function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {"type":"Integer","value":0}
   📋 Status: {"type":"String","value":"active"}
   📋 Name: {"value":"Alice Smith","type":"String"}

============================================================
📝 Function 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: thUJFOSq0Xq8kDuoppJTxQ3TqZ4hwWfvJmNxbg4zL24e-n7WF1KHMH_kAQci3lNHcbl4WLplT08D48kOpx7g6Q

2️⃣ Calling Function (Query + Delete + Verify)...
   ✅ Function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted function: GryGx93TzJwgKvmb31_n...
   ✅ Deleted function: k5bk6N0noOhaPIito0y6...
   ✅ Deleted function: l2SOKpPtNCqDwmfwQF_i...
   ✅ Deleted function: thUJFOSq0Xq8kDuoppJT...
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
  Output: Document ID = Y5IQRTg5ObEemXfcQ15jYxrXi4q7iLzerSozpv7LZbPGIB5dl8zl9kKmP0xjfcZ9_h0qhQPTUCcXCazcBokHDw
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: findById(Y5IQRTg5ObEemXfcQ15jYxrXi4q7iLzerSozpv7LZbPGIB5dl8zl9kKmP0xjfcZ9_h0qhQPTUCcXCazcBokHDw)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: findById(Y5IQRTg5ObEemXfcQ15jYxrXi4q7iLzerSozpv7LZbPGIB5dl8zl9kKmP0xjfcZ9_h0qhQPTUCcXCazcBokHDw)
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
  Output: Document ID = YXTBwBg1SU4dwVT0fl-3u-0N2GVlzPROFcdhVgKiqLkArraVPBt-kwkJlWvQitPMUIwm5Futit5EeMU2mQssgQ
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: findById(YXTBwBg1SU4dwVT0fl-3u-0N2GVlzPROFcdhVgKiqLkArraVPBt-kwkJlWvQitPMUIwm5Futit5EeMU2mQssgQ)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: findById(YXTBwBg1SU4dwVT0fl-3u-0N2GVlzPROFcdhVgKiqLkArraVPBt-kwkJlWvQitPMUIwm5Futit5EeMU2mQssgQ)
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

added 1 package, removed 1 package, and audited 9 packages in 1s

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'XoOOEPNYhpYs6sBODqAGXy4HAkZXT2ZFDo1Qrws54ega8FWVeJdAW2o3owgxhqE8G5WFr3pPjSOj0OnQ0dlNzg'
}

=== Find by ID ===
Found: {
  id: 'XoOOEPNYhpYs6sBODqAGXy4HAkZXT2ZFDo1Qrws54ega8FWVeJdAW2o3owgxhqE8G5WFr3pPjSOj0OnQ0dlNzg',
  active: { value: true, type: 'Boolean' },
  name: { value: 'Test Record', type: 'String' },
  value: { type: 'Integer', value: 42 }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  active: { type: 'Boolean', value: true },
  name: { value: 'Updated Record', type: 'String' },
  value: { type: 'Integer', value: 100 },
  id: 'XoOOEPNYhpYs6sBODqAGXy4HAkZXT2ZFDo1Qrws54ega8FWVeJdAW2o3owgxhqE8G5WFr3pPjSOj0OnQ0dlNzg'
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: Eoyjb5_9w5J7Z-STXWTWVff3P4_oqI7ZiBWpldWHomuLB9b2k6xmn__XEID68NV727b6kHJ2LGXI2rnxlIIqpA

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
Collection created with first record: QPJY4fvzk5L1pJ2vHWsS1WY8jB5r7GvufsLHFp2mydBogJMOHGi2OjLvFNB7ixCoDxqtduG_nzoBDO6fjqM0Mw

=== List Collections ===
Total collections: 49
Sample collections: audit__ek0_testing,cached_database_metrics,schema_products_client_go,enriched_users,schema_employees_client_py

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
cache:product:3: { name: 'Product 3', price: 49.989999999999995 }

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 0 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 19

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: false

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: LfDPZSQwQUNpnYRrhui2B6aeYkmkIoQI8zq9hmF77YA5KpXf_HxL2SNgNnWBY9jtbBzPOaqeRud1w7CtmSGBSA
Created Bob: $500 - ID: vSEB1uYY6uwziLMNvQNcsQ6xniFBTt16Blul3xjG6MeQr3XSB450YJfUgbUZp980r5uBodeEFq3l0DGp-cUAgA

=== Example 1: Begin Transaction ===
Transaction ID: eddda2b3-0ffa-4f89-a08b-f7d01aa93510

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: e73321b4-6af3-4aa2-ae2f-2edf0095928d
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
  1. Score: 25.740, Matched: email.value, name, name.value, email
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: bio, title.value, title, bio.value
  2. Score: 26.400, Matched: title, bio, title.value, bio.value
  3. Score: 26.400, Matched: bio.value, bio, title, title.value
  4. Score: 26.400, Matched: title.value, title, bio.value, bio

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title.value, bio.value, bio, title
  2. Score: 39.600, Matched: title, title.value, bio.value, bio
  3. Score: 39.600, Matched: title, bio, title.value, bio.value
  4. Score: 39.600, Matched: bio, bio.value, title, title.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio.value, bio

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.752, Matched: 
  2. Score: 0.746, Matched: 
  3. Score: 0.741, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.698, Matched: content.value, content, title, title.value
  2. Score: 1.496, Matched: title.value, content, content.value, title
  3. Score: 0.301, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: bio.value, skills.value, bio, skills

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
✓ Inserted document: RBKShIYF6oYbu2yjyZJ7j4prypz7K3dY4MGigxkbOajIxFg1c5X3Q1gqwyi0WTcJkTkYTU6tjlfSWmHCAb-FBg

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: OZ2inoxC8R-RzKkv6YFcDefmOUqWBv8XKxqEElMwXHNG-JAK056CGSBsgVSVMGB_TGhklv07c-fwT0GUy1LDnQ

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
✓ Inserted document with TTL: y_-oJ8eUVPPbD3t3JPbOabW5TqSok6POT1W4mnWJwUCVONVwWFtrqJK53i5qWq7OefA2Cmu04ew0sVPmEt9XsA

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
✓ Edge cache script created: DCet2_HUUfNNQGyYuWQUjNgHbUuBYTTt9twwlmhvuES1MCkSAN_qRSWW8Exjn-WTbsb4QW9-AmE9crZWjaMgeQ

Call 1: Cache miss (fetches from API)
Response time: 44ms
Result: {
  "records": [
    {
      "value": {
        "current": {
          "temperature_2m": 7.8,
          "time": "2026-04-06T03:45",
          "interval": 900
        },
        "generationtime_ms": 0.01347064971923828,
        "elevation": 32,
        "longitude": -73.99308,
        "timezone": "GMT",
        "current_units": {
          "interval": "seconds",
          "time": "iso8601",
          "temperature_2m": "°C"
        },
        "timezone_abbreviation": "GMT",
        "utc_offset_seconds": 0,
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
Response time: 43ms (1x faster!)
Result: {
  "records": [
    {
      "value": {
        "current": {
          "temperature_2m": 7.8,
          "time": "2026-04-06T03:45",
          "interval": 900
        },
        "generationtime_ms": 0.01347064971923828,
        "elevation": 32,
        "longitude": -73.99308,
        "timezone": "GMT",
        "current_units": {
          "interval": "seconds",
          "time": "iso8601",
          "temperature_2m": "°C"
        },
        "timezone_abbreviation": "GMT",
        "utc_offset_seconds": 0,
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

🚀 ekoDB Functions Example (JavaScript Client)

✅ Client initialized (token exchange automatic)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved: gMZnrXAnm1g12tl07w9BSj38-4DQcnyovOyuRw9vO5GMFPceIDKnDPPYZ_QjoHQwzGI7YfIth30wjwTekh7kTg
📊 Found 10 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Function

✅ Function saved
📊 Found 10 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Function

✅ Function saved
📊 Statistics: 2 groups
   {"avg_score":60,"status":"active","count":5}
   {"count":5,"avg_score":50,"status":"inactive"}
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
   ⏱️  Duration: 174ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "id": 1,
    "username": "Bret",
    "company": {
      "catchPhrase": "Multi-layered client-server neural-net",
      "name": "Romaguera-Crona",
      "bs": "harness real-time e-ma...

Second call (cache hit - from cache):
   ⏱️  Duration: 41ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "id": 1,
    "username": "Bret",
    "company": {
      "catchPhrase": "Multi-layered client-server neural-net",
      "name": "Romaguera-Crona",
      "bs": "harness real-time e-ma...
   🚀 Cache speedup: 4.2x faster!

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

✅ Function saved: jOkz8tB_GRtjoYiuaNXYm5ghTFztzooF8iwMYnI23D2Wubxg2VUwLfbSSYAiNRMNd_-y-p8zpb_SJEr7IWMW-A
📊 Found 2 product groups
   {"category":"Furniture","count":2,"avg_price":474}
   {"category":"Electronics","count":3,"avg_price":575.6666666666666}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   {"category":"Furniture","count":2}
   {"category":"Electronics","count":3}
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

✅ Inserted order: LsUdAv14JMQOcbMHVQCLZa_MdS72eaADJzQmASa6aNp-n08DcGI3EDlIFAmFqMgAoQ9UF7V6TMxfScrNUY3vbg
✅ Inserted 2 products with wrapped types

📝 Example 2: Function with Wrapped Type Parameters

✅ Function saved: qjih_hs2Ygh_zgNRdvmfmGgmsC3haqLs1ZB38uMx0TWoPcujxWBdKu2_CLhiGw5tQu8P5iklWbT509wnhvSvkQ
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"userId":"user_abc","role":"admin"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: g--WtMdIQHQz1g0QhLRIX-40nyxqYQBBmIOCVXxIJS4D57Xa5SJT9v2A3FBwWwK1uFeFaascuYvE1fcWTwQGpw
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Function

✅ Function saved: i1-tddoaHYRtFMMTqg5OHA2DdbOEzy0dsn82GW4rvqCt8JhwKhBPYN3XuaZqTz82QU5h8VOK-GteUIElueWG4A
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
   inactive: 3 users
   active: 7 users
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
   1. User 1 - Score: 10
   2. User 4 - Score: 40
   3. User 8 - Score: 80
   4. User 5 - Score: 50
   5. User 7 - Score: 70
⏱️  Execution time: 0ms

📝 Example 5: User Summary Statistics

✅ Function saved
📊 User summary (2 groups):
   inactive users: 3 users, Total Score: 180
   active users: 7 users, Total Score: 370
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
   2. Vector Databases Explained (Database)
   3. Natural Language Processing (AI)
   4. Database Design Principles (Database)
   5. Introduction to Machine Learning (AI)
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
   1. Getting Started with ekoDB
   2. Vector Databases Explained
   3. Natural Language Processing
   4. Database Design Principles
   5. Introduction to Machine Learning
⏱️  Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Function saved
📊 Projected documents (showing first 3):
   1. Getting Started with ekoDB
   2. Vector Databases Explained
   3. Natural Language Processing
⏱️  Execution time: 0ms

📝 Example 5: All Document Fields

✅ Function saved
📊 All documents (5 total, showing first 2):
   1. Getting Started with ekoDB (Database)
   2. Vector Databases Explained (Database)
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
   Furniture:
      Products: 4 | Stock: 43 | Avg Rating: ⭐4.26
   Electronics:
      Products: 6 | Stock: 232 | Avg Rating: ⭐4.52

⏱️  Total execution time: 0ms
📈 Pipeline stages:

📝 Example 5: Project Specific Fields

✅ Function saved
📊 Product summaries (10 items, showing first 3):
   1. Monitor 27" - $399 (⭐4.6)
   2. Standing Desk - $599 (⭐4.7)
   3. Wireless Mouse - $29 (⭐4.5)
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
   1. Precision: Vector databases are able to store data with high precision, making them suitable for applications such as CAD and GIS where accuracy is crucial.

2. Scalability: Vector data can be easily scaled without losing quality. This means that you can zoom in and out without the image becoming pixelated or blurred.

3. Less Storage: Vector data is usually less memory-intensive than raster data. This is because it only needs to store the coordinates of points and the attributes of lines and polygons, rather than individual pixel values.

4. Topology: Most vector data models incorporate topological information. This means they accurately store spatial relationships between features such as connectivity, adjacency, and containment.

5. Attribute Data: Vector databases allow each entity to have associated attribute data, which can be useful for analysis and decision-making processes.

6. Quality of Data: Vector databases typically have a higher quality of data due to less redundancy and better structure compared to other databases.
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embedding function saved
📊 Generated embeddings for 3 articles
   1. "Advanced Query Patterns" - 1536D vector
   2. "Getting Started with ekoDB" - 1536D vector
   3. "Draft Article" - 1536D vector
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
✓ Created SWR script: fetch_api_user_js (tZm7hh-cMg70LXI_-iwbt0fNKM7fEuJKZOobSevPt-ruBX-y-kWQuDC76q-1JM-cJ7_tjl-2exsXKExoQo0QHQ)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "name": "Leanne Graham",
        "website": "hildegard.org",
        "username": "Bret",
        "address": {
          "geo": {
            "lng": "81.1496",
            "lat": "-37.3159"
          },
          "street": "Kulas Light",
          "suite": "Apt. 556",
          "zipcode": "92998-3874",
          "city": "Gwenborough"
        },
        "id": 1,
        "phone": "1-770-736-8031 x56442",
        "email": "Sincere@april.biz",
        "company": {
          "catchPhrase": "Multi-layered client-server neural-net",
          "name": "Romaguera-Crona",
          "bs": "harness real-time e-markets"
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
Response time: 43ms (served from cache)
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
✓ Created native SWR script: github_user_native (rxDY1TokPG9Hh1_6Zrrn2NDlp4B8CLyLSPRE2H_ix_mjmbWDd8VfrAja6zCG2wgxOk4elRZ7fyQvYcmJ8Iykxw)

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
✓ Created SWR script with audit trail: product_swr_audit (sI9aZ-AmzuvK0Vr6F6l0qYoJtU63GLt3YRfTHbrjVNlnB2-9qWzLGuahIpqtxTOAKFR3guR09d7D5P6AbBJMPg)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (s012Wg-0-rzy2pwAnklURD6LLt4CEXs9cJTyXhzCewDIDN9mZOwi_yIaXQnH9yYGccBCTHlmp8acgyWb6ZoVKg)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (nNLfkBSHG3PsEdYNCN6DiaHqImkkb2SpD2KxfxHDyAoqq48dYF-R1_iSNldQNbMV7zZ2QDLfSiT_CfmB1UwgEQ)
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
✓ Created session: pC9ZgDag9UdMhCCFGdintrN2bnIO_gW46_Nttw0a_juTVmQF5CEaOuhfQEDArRFsaYmGhDipDc_iNMRZNF6FRg

=== Sending Chat Message ===
Message ID: -BU-3PYkvQPjGr45P4ImnVwEXhekygsqFC0k1leoA3EL650YazrDbwputwlGlBuc-jp9bWQR1L9oTjOae80Ngw

=== AI Response ===
Here are the available products along with their prices:

1. **ekoDB Pro**
   - Description: Enterprise edition product with advanced features
   - Price: $299

2. **ekoDB Cloud**
   - Description: Fully managed cloud database service product
   - Price: $499

3. **ekoDB**
   - Description: A high-performance database product with AI capabilities
   - Price: $99

If you need more information about any of these products, feel free to ask!

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_js',
  record: {
    id: 'T2_MeD6UEko5iBMRo57n3c4G_xuGPVdz4QEiR5IW4UuRw9mVRKx3z9-18DQDp-nit7TpRFi25BWuLTkN_j-lJQ',
    price: 299,
    description: 'Enterprise edition product with advanced features',
    name: 'ekoDB Pro'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_js',
  record: {
    id: 'HcYS8VP5jLZCNm4yhk3cMqskVhJPKJY06Q5Xyc_wHkc8cXYqz93GgDzP0iKg7oIh9-D0sX2EtUUDsyt_d3U7pA',
    description: 'Fully managed cloud database service product',
    name: 'ekoDB Cloud',
    price: 499
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_js',
  record: {
    description: 'A high-performance database product with AI capabilities',
    name: 'ekoDB',
    price: 99,
    id: 'MPBar4MqTvaetGr3O9NlCNRhcDobatUyFBFaZoGM7aXOWt2u5TfIcg-DjnJF38Mf26MpJM-8TTUStyqldkRrjA'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 3151ms

=== Token Usage ===
Prompt tokens: 1298
Completion tokens: 110
Total tokens: 1408

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: MOE12b2du5c0PP4iUi8oc9qvpApOjpzYQjvYjTKAZgyhHIKQe9y-QYXr_hNEAfKbOsC8wBIma92BjXM5ZAeN9w

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: High-performance database product
- **Price**: $99

If you need more information or would like to see additional products, let me know!

✓ Second message sent

=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of ekoDB is $99. If you have any more questions or need further assistance, feel free to ask!

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: aOI93L8pyMfccWvlebaCdtv4RQTjsTioblCS8i-LVCgjWDcpwAayoz6F0mgIsdKO2SrZrLOCpP1MwE-YyBR9IA
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
✓ Created session: 3eLXgFvR1U8w_gYtTTlwRI4XUpNX8EqKVXDhT6OEOgq4HZe3yy0uQ_gp4CiV9prGTHjY1HzXXIxB4CGaKHrGtQ

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name:** ekoDB
- **Price:** $99

If you'd like more information or to explore additional products, please let me know!

✓ Message 2 sent
  Response: The price of ekoDB is **$99**. If you need any more information, feel free to ask!

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: DpExGxMi3Nt31b6hHoGt6-6mP5Sgw_chdhsUuvJgOZdV3RZAJV2ZkNGJ2SWetdLLkVHBAxfBhOsxruVOFamCtg
  Parent: 3eLXgFvR1U8w_gYtTTlwRI4XUpNX8EqKVXDhT6OEOgq4HZe3yy0uQ_gp4CiV9prGTHjY1HzXXIxB4CGaKHrGtQ

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: DpExGxMi3Nt31b6hHoGt6-6mP5Sgw_chdhsUuvJgOZdV3RZAJV2ZkNGJ2SWetdLLkVHBAxfBhOsxruVOFamCtg (Untitled)
  Session 2: 3eLXgFvR1U8w_gYtTTlwRI4XUpNX8EqKVXDhT6OEOgq4HZe3yy0uQ_gp4CiV9prGTHjY1HzXXIxB4CGaKHrGtQ (Untitled)
  Session 3: aOI93L8pyMfccWvlebaCdtv4RQTjsTioblCS8i-LVCgjWDcpwAayoz6F0mgIsdKO2SrZrLOCpP1MwE-YyBR9IA (Untitled)
  Session 4: pC9ZgDag9UdMhCCFGdintrN2bnIO_gW46_Nttw0a_juTVmQF5CEaOuhfQEDArRFsaYmGhDipDc_iNMRZNF6FRg (Untitled)
  Session 5: FR9F_qSfuVyQ4TdxyeyUH62vek2rS32PNm4TEei_tYVSi2403Te85WeB2e8oMlCh0cZQe4SqPTeUlpiKvzJTvg (Untitled)
  Session 6: p8s1qPSKPwGUe4GBfg6clOWiJdpIODgQAG5XkF0F0iSmDTfY_Omh2ocmr_ZcLYDggYpFsqMAy6LmFs5Qi-Zv3w (Untitled)
  Session 7: NywLVYKnaHEA4WfOYsmM9dY1oavQ8hnEq8Dodp6wkPp3PUoHYtYQTghQjokz331UmUU3qkT2VrZ7j6Wtw6o1qg (Untitled)
  Session 8: NDeOBVKsMCqOpcQHsNmV96Nji7V4MJEHYbKVcf_YfpvNMO5mcJIO19Nb_3ljx4BrrIZG572g5PizSy-vllMSSA (Untitled)
  Session 9: _DgimDs70IFjtBMQiEuIgkb4BNyDwHu4ZzjUhOnlzAxAeCdSKVpiyLbaB8_hA-OsMxz_HPNaHeYBLDnawpQd2w (Untitled)
  Session 10: j35FSFi7T5MNfApTG_tS_0nktaY5bNDTaUm9_Oh1NZq0crz26C65MZINFB_tBRD96KcT1EvDsvfv9PqQglIKyg (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: DpExGxMi3Nt31b6hHoGt6-6mP5Sgw_chdhsUuvJgOZdV3RZAJV2ZkNGJ2SWetdLLkVHBAxfBhOsxruVOFamCtg

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
✅ [32mJavaScript client examples complete![0m
