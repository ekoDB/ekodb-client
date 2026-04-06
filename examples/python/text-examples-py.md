make test-examples-python
🧪 [36mRunning Python examples (direct HTTP/WebSocket)...[0m
=== Simple CRUD Operations (Direct HTTP) ===

✓ Authentication successful

=== Insert Document ===
Inserted: {'id': 'DU0XsPJjcLC--cl34H__TX7u_0Yvtolf6U7gBq_jdMj40XonT_g1Rl9EckP4XzmXMRMFAUWbH9oBENv4_PMwhQ'}

=== Find by ID ===
Found: {'id': 'DU0XsPJjcLC--cl34H__TX7u_0Yvtolf6U7gBq_jdMj40XonT_g1Rl9EckP4XzmXMRMFAUWbH9oBENv4_PMwhQ', 'active': {'type': 'Boolean', 'value': True}, 'value': {'type': 'Integer', 'value': 42}, 'name': {'type': 'String', 'value': 'Test Record'}}

=== Find with Query ===
Found documents: [{'id': 'DU0XsPJjcLC--cl34H__TX7u_0Yvtolf6U7gBq_jdMj40XonT_g1Rl9EckP4XzmXMRMFAUWbH9oBENv4_PMwhQ', 'name': {'value': 'Test Record', 'type': 'String'}, 'active': {'value': True, 'type': 'Boolean'}, 'value': {'type': 'Integer', 'value': 42}}]

=== Update Document ===
Updated: {'id': 'DU0XsPJjcLC--cl34H__TX7u_0Yvtolf6U7gBq_jdMj40XonT_g1Rl9EckP4XzmXMRMFAUWbH9oBENv4_PMwhQ', 'name': {'type': 'String', 'value': 'Updated Record'}, 'value': {'value': 100, 'type': 'Integer'}, 'active': {'type': 'Boolean', 'value': True}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: c1i-VSVWIrdWR6V_fqAt9gYeFb4NEK79wRxhE2e-rS1NrAVhfIwV131gxPfckEaWJEf3ZhSJFn1SDg5j6md7Fw

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "messageId": "652217827",
  "payload": {
    "data": [
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "M3mSBAp10I75QixK6vT-brdSo_q7MSHafLfCr4Q0cOmppQVBlQCXrDgmbtfsD9RdbXFqtDeOqvbEC0gjfRifNQ",
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
        "id": "c1i-VSVWIrdWR6V_fqAt9gYeFb4NEK79wRxhE2e-rS1NrAVhfIwV131gxPfckEaWJEf3ZhSJFn1SDg5j6md7Fw",
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
✓ Retrieved 2 record(s) via WebSocket

✓ WebSocket example completed successfully
🚀 ekoDB Functions Example (Python/HTTP)

✓ Authentication successful

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Function saved: i-rZFngF_a-atWucHekcIQgQTMtyRKs-vhBDib-mIs3W8gmutFtbLemfjE_rKV2BQ1N5QmjW5XRr-XLR7Ybuig
📊 Found 15 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Function saved: 42Z6cXlrMU49NFmTal03DvMtUXiiprPqn87ZAiHiu3PTW-oDe8F4TWTn7xoGbvRq2kFZvx_t-E99ebk5ore_xg
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: Rp8tFbHXpzpfBER4hv7H_TFwk9Fk1YkNG-gpre91ps5-GoZf_fv-K3tbOeB5UnyWOxL2P442h_g8nX_H2Il0fA
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {'status': 'inactive', 'avg_score': 50.0, 'max_score': 90, 'count': 15}
   {'max_score': 100, 'avg_score': 60.0, 'count': 15, 'status': 'active'}

📝 Example 4: Function Management

📋 Total scripts: 15
🔍 Retrieved script: Get Active Users
✏️  function updated
🗑️  function deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

✅ All examples completed!
=== Batch Operations (Direct HTTP) ===

✓ Authentication successful

=== Batch Insert ===
✓ Batch inserted 5 records
✓ Verified: Found 10 total records in collection

=== Creating test records for update/delete ===
Created 3 test records

=== Batch Update ===
✓ Batch updated 3 records
✓ Verified: Record updated with status="active"

=== Batch Delete ===
✓ Batch deleted 3 records
✓ Verified: Records successfully deleted (not found)

✓ All batch operations completed successfully
=== Key-Value Operations (Direct HTTP) ===

✓ Authentication successful

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: {'userId': 123, 'username': 'john_doe'}

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: {'price': 29.99, 'name': 'Product 1'}
cache:product:2: {'name': 'Product 2', 'price': 39.989999999999995}
cache:product:3: {'price': 49.989999999999995, 'name': 'Product 3'}

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
=== Collection Management (Direct HTTP) ===

✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: 3rTvVUlXQWMJoVh2cU-U-fO2ZlVEch7auA1YKP08cFjs4z55coI68qVuUH3frCQTLMORTs8a1HX24fh854AUBg

=== List Collections ===
Total collections: 13
Sample collections: ['ttl_cache', 'chat_messages__ek0_testing', 'demo_collection', 'functions__ek0_testing', 'products']

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: False

✓ All collection management operations completed successfully
✓ Authentication successful

📋 Getting original configuration...
   Original durable_operations: True


============================================================
🔥 TEST 1: Original Config (durable=True)
============================================================

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: dcL0PYZpnQr_aujiL0_UaacAVuyhyypkS17BMQRfuP7U0a520-uUoRNNJ5oBeeKpWP3Www2fL758M7pbZqCqzw
Created Bob: $500 - ID: KyBMaX6zPTat9YVrpdVIiGycJRSWbTBFuRY8acpE5MB_QmzL8VHt0egtRJ3JUC3_944WojGV13wcdGmzWr0Gnw

=== Example 1: Begin Transaction ===
Transaction ID: 571f6304-0313-40f8-982b-41fe5e6e7982

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: ${'type': 'Integer', 'value': 800}
Bob: ${'type': 'Integer', 'value': 700}

=== Example 5: Rollback ===
New transaction: d0fa0c4c-650b-4564-b01a-be542a3cfd37
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'type': 'Integer', 'value': 700}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: R_pABTyG6cMQ-UkKI-cNnkDVH_sJzieQNw9HlBcmYiSh1z8H0buZixSIZvefSqlGGbKHYdDb1pY8WBgdU_oCjw
Created Bob: $500 - ID: NyAhLmaoDeQYK89gqmOoNWQIweDBqguJ58KQTjFUzL8bZsLdYcx-03Ix4hLhZYuCAXTFk6ogWgmEOk3AwGTcxQ

=== Example 1: Begin Transaction ===
Transaction ID: 027b2cb8-ce52-4d6f-9579-c8fbd9a5393d

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: ${'type': 'Integer', 'value': 800}
Bob: ${'value': 700, 'type': 'Integer'}

=== Cleanup ===
✓ Deleted test accounts


🔄 Switching to NON-DURABLE mode...
   ✓ Config updated: durable_operations=false


============================================================
🔥 TEST 2: Non-Durable Mode (durable=false)
============================================================

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: TtT2sP9kQycNl-rNGDlQJ3oa3zZh2m9z3irzf9FiRpzboKa7rcnM5mdoE9aR__1NWd6zTJCSV5BBhmS6l0G_qg
Created Bob: $500 - ID: 7b_n1WLEw8j0nxPAP52EV9DUWrvenoNaMh1wcuyyFyuirl3h6EnSuoPRoxh-gOqq7naRWCsQPA_WGTVcQSW5PA

=== Example 1: Begin Transaction ===
Transaction ID: fd7eacc4-3307-48b7-b953-a8b02123a0db

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: ${'type': 'Integer', 'value': 800}
Bob: ${'value': 700, 'type': 'Integer'}

=== Example 5: Rollback ===
New transaction: 8f4fc86c-143b-4a7c-9f0a-473b3fa74231
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'type': 'Integer', 'value': 700}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: TLRYNQpMdAeSWPPVpVtY8DmfUpRJBHGlPkYQYa3QrGpP1izjhHJ_rFSu84R62HEo7WjPQ5lT9eZJ8coAIoT3Lg
Created Bob: $500 - ID: yVl1XBg7X1bDJg6aYrxQLz0wdFJ9MJUrdqaoensh7DNy1DHd88PPI0Zrv3qNzycHkqklWTyjskPComaUhke82g

=== Example 1: Begin Transaction ===
Transaction ID: 772e76c2-ed40-454d-aa03-af16145cc650

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: ${'type': 'Integer', 'value': 800}
Bob: ${'value': 700, 'type': 'Integer'}

=== Cleanup ===
✓ Deleted test accounts


🔄 Switching to DURABLE mode...
   ✓ Config updated: durable_operations=true


============================================================
🔥 TEST 3: Durable Mode (durable=true)
============================================================

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: CZ0T-d1x8Ton8gtVG1MgyR8LyRrzz5mi-aeXGEJo8lPWJWAKj4H5LDGOeA2icJSri13EabQ_XXHRQzVBftnFnw
Created Bob: $500 - ID: 8DJsYk2O5sW3FMY7X4AkmqviPEF5DanzbHe-qJRiJLjoFDx1D5X9UbrozcZRTxvgS-K_mGZqbvBXiGAhHS6mxw

=== Example 1: Begin Transaction ===
Transaction ID: ce4b260c-3da2-49f7-b53d-a17f45537344

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: ${'type': 'Integer', 'value': 800}
Bob: ${'value': 700, 'type': 'Integer'}

=== Example 5: Rollback ===
New transaction: 320f78da-a57f-433a-b341-0d07cd41a033
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'type': 'Integer', 'value': 700}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: UAKTRVkFYwPugfNTbRJF9pHdTwC4nhix3XUTeH06kAokq4v6kRzujBH5lkiat73B_7oRfh5Qq-NSmmRiRSSABg
Created Bob: $500 - ID: OMZxgUPZOC8K5lTJoiXaPZK_-cYPw6NDsXcLdQCGt9-cVFdRHn1UTCOI-nqsD86ZILmR1rDehht5WJnqVfNjZw

=== Example 1: Begin Transaction ===
Transaction ID: d588da2d-4b96-478c-bc51-9558cff14d44

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: ${'type': 'Integer', 'value': 800}
Bob: ${'type': 'Integer', 'value': 700}

=== Cleanup ===
✓ Deleted test accounts


🔄 Restoring original configuration...
   ✓ Config restored: durable_operations=True


============================================================
✅ ALL TESTS PASSED - Transactions successful
============================================================

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
📝 function 1: Insert + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: 3U_dCDqjO9I_pUGNywNST1POTAt4PtDPZE5jLKkVbnW9D4KcIWz-uxp-GiibqeQYYpZsPptMNVYMjtuAABcspw

2️⃣ Calling function (Insert + Verify)...
   ✅ function executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: hr9SqG5UVkqEg_-Q0yc9UK6yh25fw_YaxCosQTPCxb6d3hkcuPhSpbo3nO6s8vPRJ4uUlYuufyF1eKmBQPqo6Q
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}
   📋 Email: {'value': 'alice@example.com', 'type': 'String'}
   📋 Status: {'value': 'pending', 'type': 'String'}
   📋 Credits: {'type': 'Integer', 'value': 0}

============================================================
📝 function 2: Query + Update + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: uEyI7gY-jzYeFqBDdYiFb2uYykBhARaPyUwHx9qC9yp4s4nG5rDzTizv8Ew2s_-amR6Nic5nC0TbsSxNUvSF6A

2️⃣ Calling function (Query + Update + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Status updated to: {'type': 'String', 'value': 'active'}
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}

============================================================
📝 function 3: Query + Update Credits + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: 3EHFhPIlhQSlFq3m80kRTFxN_J6BpTem2oZ1LQU-RPNSg5zl8MdgzZKS--4FWgkXYdpc0MyhEU4PwxNcVlNCAQ

2️⃣ Calling function (Query + Update Credits + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {'type': 'Integer', 'value': 0}
   📋 Status: {'type': 'String', 'value': 'active'}
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}

============================================================
📝 function 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: LK9cZwf-uJ6itK2_4S35kybmJ1zOqkqKIetuhnMxk-vs68uFgnfrjU_Cq9QmOOL-LFHE-jlrDiVaFHtg9U3iMA

2️⃣ Calling function (Query + Delete + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: 3U_dCDqjO9I_pUGNywNS...
   ✅ Deleted script: uEyI7gY-jzYeFqBDdYiF...
   ✅ Deleted script: 3EHFhPIlhQSlFq3m80kR...
   ✅ Deleted script: LK9cZwf-uJ6itK2_4S35...
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
  Output: Document ID = Sfbs1br8FdhCa61F2Y1D3NK5L4adXQZ8NV6uLhKlWDv3NIk6OwgXRu9z30y0y5-4-1O9sJ5oqZeinI2zMNWx_g
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: find_by_id(Sfbs1br8FdhCa61F2Y1D3NK5L4adXQZ8NV6uLhKlWDv3NIk6OwgXRu9z30y0y5-4-1O9sJ5oqZeinI2zMNWx_g)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: find_by_id(Sfbs1br8FdhCa61F2Y1D3NK5L4adXQZ8NV6uLhKlWDv3NIk6OwgXRu9z30y0y5-4-1O9sJ5oqZeinI2zMNWx_g)
  Output: Error (expected) - Find failed: Record not found
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
  Output: Document ID = uH91F6o_apADBpls4BDsuVaJzge6pSrfRtQIo5HsBJ7hv6bwWVz6jKSh1UYso-T3i7tOSDPgIt0Z6J9Ow2uraA
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: find_by_id(uH91F6o_apADBpls4BDsuVaJzge6pSrfRtQIo5HsBJ7hv6bwWVz6jKSh1UYso-T3i7tOSDPgIt0Z6J9Ow2uraA)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: find_by_id(uH91F6o_apADBpls4BDsuVaJzge6pSrfRtQIo5HsBJ7hv6bwWVz6jKSh1UYso-T3i7tOSDPgIt0Z6J9Ow2uraA)
  Output: Error (expected) - Find failed: Record not found
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
  • Queries correctly return None for expired documents
[34m
╔════════════════════════════════════════╗[0m
[34m║   ekoDB Python Examples Test Suite    ║[0m
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
=== Running simple_crud.py ===[0m
[32m✓ simple_crud.py completed successfully[0m
[34m
=== Running simple_websocket.py ===[0m
[32m✓ simple_websocket.py completed successfully[0m
[34m
=== Running http_functions.py ===[0m
[32m✓ http_functions.py completed successfully[0m
[34m
=== Running batch_operations.py ===[0m
[32m✓ batch_operations.py completed successfully[0m
[34m
=== Running kv_operations.py ===[0m
[32m✓ kv_operations.py completed successfully[0m
[34m
=== Running collection_management.py ===[0m
[32m✓ collection_management.py completed successfully[0m
[34m
=== Running transactions.py ===[0m
[32m✓ transactions.py completed successfully[0m
[34m
=== Running crud_functions.py ===[0m
[32m✓ crud_functions.py completed successfully[0m
[34m
=== Running document_ttl.py ===[0m
[32m✓ document_ttl.py completed successfully[0m
[34m
=== Running websocket_ttl.py ===[0m
[32m✓ websocket_ttl.py completed successfully[0m
[34m
╔════════════════════════════════════════╗[0m
[34m║           Test Summary                 ║[0m
[34m╚════════════════════════════════════════╝[0m
[34mTotal: 10[0m
[32mPassed: 10[0m
[32mFailed: 0[0m
✅ [32mPython direct examples complete![0m
🐍 [36mBuilding Python client package...[0m
🍹 Building a mixed python/rust project
🔗 Found pyo3 bindings with abi3 support
🐍 Found CPython 3.11 at /Library/Frameworks/Python.framework/Versions/3.11/bin/python3
💻 Using `MACOSX_DEPLOYMENT_TARGET=11.0` for aarch64-apple-darwin by default
    Finished `release` profile [optimized] target(s) in 0.16s
📦 Built wheel for abi3 Python ≥ 3.8 to /Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.15.2-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel for current platform...[0m
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.15.2-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.15.2
    Uninstalling ekodb_client-0.15.2:
      Successfully uninstalled ekodb_client-0.15.2
Successfully installed ekodb-client-0.15.2
✅ [32mPython client package built and installed![0m
🧪 [36mRunning Python client library examples...[0m
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {'id': 'nuP5oMt7nFnlrggNtLjngsPA_AX89xiSfdTIWAz-qjbDpGdsQnZPLwrh0T7ikf6r8inWiwrHLjsHJFEkiNLitg'}

=== Find by ID ===
Found: {'embedding': {'value': [0.1, 0.2, 0.3, 0.4, 0.5], 'type': 'Array'}, 'active': {'type': 'Boolean', 'value': True}, 'created_at': {'value': '2026-04-05T01:18:36.571965', 'type': 'String'}, 'name': {'value': 'Test Record', 'type': 'String'}, 'metadata': {'type': 'Object', 'value': {'key': 'value', 'nested': {'deep': True}}}, 'price': {'type': 'Float', 'value': 99.99}, 'tags': {'type': 'Array', 'value': ['tag1', 'tag2', 'tag3']}, 'user_id': {'type': 'String', 'value': '550e8400-e29b-41d4-a716-446655440000'}, 'id': 'nuP5oMt7nFnlrggNtLjngsPA_AX89xiSfdTIWAz-qjbDpGdsQnZPLwrh0T7ikf6r8inWiwrHLjsHJFEkiNLitg', 'data': {'type': 'String', 'value': 'aGVsbG8gd29ybGQ='}, 'categories': {'type': 'Array', 'value': ['electronics', 'computers']}, 'value': {'type': 'Integer', 'value': 42}}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): True
  price (Decimal): 99.99
  created_at (DateTime): 2026-04-05 01:18:36.571965
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): ['tag1', 'tag2', 'tag3']
  metadata (Object): {'key': 'value', 'nested': {'deep': True}}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): ['electronics', 'computers']
  data (Bytes): 11 bytes
Plain record: {'embedding': [0.1, 0.2, 0.3, 0.4, 0.5], 'active': True, 'created_at': '2026-04-05T01:18:36.571965', 'name': 'Test Record', 'metadata': {'key': 'value', 'nested': {'deep': True}}, 'price': 99.99, 'tags': ['tag1', 'tag2', 'tag3'], 'user_id': '550e8400-e29b-41d4-a716-446655440000', 'id': 'nuP5oMt7nFnlrggNtLjngsPA_AX89xiSfdTIWAz-qjbDpGdsQnZPLwrh0T7ikf6r8inWiwrHLjsHJFEkiNLitg', 'data': 'aGVsbG8gd29ybGQ=', 'categories': ['electronics', 'computers'], 'value': 42}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'price': {'value': 99.99, 'type': 'Float'}, 'value': {'value': 100, 'type': 'Integer'}, 'user_id': {'value': '550e8400-e29b-41d4-a716-446655440000', 'type': 'String'}, 'tags': {'value': ['tag1', 'tag2', 'tag3'], 'type': 'Array'}, 'id': 'nuP5oMt7nFnlrggNtLjngsPA_AX89xiSfdTIWAz-qjbDpGdsQnZPLwrh0T7ikf6r8inWiwrHLjsHJFEkiNLitg', 'embedding': {'type': 'Array', 'value': [0.1, 0.2, 0.3, 0.4, 0.5]}, 'created_at': {'value': '2026-04-05T01:18:36.571965', 'type': 'String'}, 'name': {'type': 'String', 'value': 'Updated Record'}, 'active': {'type': 'Boolean', 'value': True}, 'data': {'value': 'aGVsbG8gd29ybGQ=', 'type': 'String'}, 'categories': {'value': ['electronics', 'computers'], 'type': 'Array'}, 'metadata': {'type': 'Object', 'value': {'nested': {'deep': True}, 'key': 'value'}}}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: 4ECKHR5jsg-LXj2Ga5VBiruGSrehqxVW7xDnD_lGMUv-YqKW94XZekYTms1hfcYmOonb7JhI1U6VXX11cFpc-A

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
Collection created with first record: "rQzGpvMBqJdTVIM8yh0_4EQY-8TwDGELzve8tPQkQ7TtgsdzqXpDVipckTsTTqUjoLQiCBcwqbpAlgMOCxpXxQ"

=== List Collections ===
Total collections: 12
Sample collections: ['ttl_cache', 'chat_messages__ek0_testing', 'functions__ek0_testing', 'products', 'audit__ek0_testing']

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: False

✓ All collection management operations completed successfully
✓ Client created

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: {'value': '{"userId":123,"username":"john_doe"}'}

=== KV Batch Set ===
✓ Batch set 3 keys
  cache:product:1: success
  cache:product:2: success
  cache:product:3: success

=== KV Batch Get ===
✓ Batch retrieved 3 values
  cache:product:1: {'price': 29.99, 'name': 'Product 1'}
  cache:product:2: {'price': 39.99, 'name': 'Product 2'}
  cache:product:3: {'price': 49.99, 'name': 'Product 3'}

=== KV Exists ===
Key exists: True

=== KV Find (Pattern Query) ===
Found 3 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 9

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: False

=== KV Batch Delete ===
✓ Batch deleted 3 keys
  cache:product:1: deleted
  cache:product:2: deleted
  cache:product:3: deleted

✓ All KV operations completed successfully
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: wPCIJidVplYDKutWuyrthnrzi_uZXqMWxcNQM6IUFBuQt7jGwV54zMkEJIplZakE-dFtwPXUO1_YE5cYlGsv3w
Created Bob: $500 - ID: 9c5daqrZ0UG4TLhkwzPvrrsVShN3IbVSV6PrA-8-OyghIc-xgegIoBmHZNHa060HJP9ogeoKxvAhyqUPM1XCTw

=== Example 1: Begin Transaction ===
Transaction ID: af13d542-ea43-4506-81e7-a4b0fd7469c8

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 9c110ee6-165f-4ce8-94ba-40bb85053905
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
Found 2 valid users

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
  1. Score: 25.740, Matched: name.value, email.value, name, email
  2. Score: 12.540, Matched: name.value, name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title.value, bio, title, bio.value
  2. Score: 26.400, Matched: bio.value, title.value, bio, title
  3. Score: 26.400, Matched: bio, bio.value, title.value, title
  4. Score: 26.400, Matched: title, bio, bio.value, title.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title.value, bio, bio.value, title
  2. Score: 39.600, Matched: title, title.value, bio.value, bio
  3. Score: 39.600, Matched: title, bio.value, title.value, bio
  4. Score: 39.600, Matched: title, bio.value, bio, title.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio.value, bio

5. Search with stemming and exact match boosting:
Found 2 results (matches: run, running, runs)
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.744
  2. Score: 0.742
  3. Score: 0.733

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.371, Matched: content.value, content, title.value, title
  2. Score: 1.372, Matched: title, content.value, title.value, content
  3. Score: 0.367, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 13.200, Matched: title, title.value

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
Schema fields: ['age', 'email', 'name', 'status']
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
  - Mouse ($25) by Alice Johnson
  - Laptop ($1200) by Alice Johnson

5. Complex join with multiple conditions:
Found 2 users with example.com emails:
  - Alice Johnson (alice@example.com): Building A
  - Bob Smith (bob@example.com): Building B

=== Cleanup ===
✅ Deleted test collections

✅ Join operations examples completed!
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: U7j8uNxicJgC5iC5gn3SbWsNuACX7z4Uw8c-VvoiYKlQPYxcIX2lxz6wHSdAy0_9gYn5rtg0tZ56vf_1tDd0FQ

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: kmO9-_TIR1mTzTz9ILCsct6OM1-M-892takk5WTIUbjZurs28muCr8WlmHvY454ThGS8vUbPhjSdjz9NdsXchw

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
✓ Inserted document with TTL: P61lB3iykKBvxtlg7ndiCM2FZw6aFY3XSQeDtwsXEhxsUeXWxJpElmEOsscHe10mHL2twXEJFUlVqZrpVv-Zvg

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 5 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
=== ekoDB as Edge Cache - Simple Example ===

Creating edge cache function...
✓ Edge cache script created: c9KUbTk92JyVTWexLg9aONmzoJYupVTsSPU8PMK-TdTwZy_C6H4LWKT7zL0ySZDjs9evx7SlX2a8KRWD58Zaow

Call 1: Cache miss (fetches from API)
Response time: 542ms
Result: {
  "records": [
    {
      "value": {
        "current": {
          "interval": 900,
          "temperature_2m": 5.6,
          "time": "2026-04-05T05:15"
        },
        "current_units": {
          "interval": "seconds",
          "temperature_2m": "\u00b0C",
          "time": "iso8601"
        },
        "elevation": 32.0,
        "generationtime_ms": 0.01704692840576172,
        "latitude": 40.710335,
        "longitude": -73.99308,
        "timezone": "GMT",
        "timezone_abbreviation": "GMT",
        "utc_offset_seconds": 0
      }
    }
  ],
  "stats": {
    "execution_time_ms": 0,
    "input_count": 0,
    "output_count": 1,
    "stage_stats": [],
    "stages_executed": 2
  }
}

Call 2: Cache hit (served from ekoDB)
Response time: 41ms (13.4x faster!)
Result: {
  "records": [
    {
      "value": {
        "current": {
          "interval": 900,
          "temperature_2m": 5.6,
          "time": "2026-04-05T05:15"
        },
        "current_units": {
          "interval": "seconds",
          "temperature_2m": "\u00b0C",
          "time": "iso8601"
        },
        "elevation": 32.0,
        "generationtime_ms": 0.01704692840576172,
        "latitude": 40.710335,
        "longitude": -73.99308,
        "timezone": "GMT",
        "timezone_abbreviation": "GMT",
        "utc_offset_seconds": 0
      }
    }
  ],
  "stats": {
    "execution_time_ms": 0,
    "input_count": 0,
    "output_count": 1,
    "stage_stats": [],
    "stages_executed": 2
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

🚀 ekoDB Functions Example (Python)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved: l8iqU8K4saq70FL7AViu-OGXH-q1_0lBK1us-Aj__aqOkAWINZe6a2lma8UCIyb8x1jUo1u-uUwUXNw8IOtweQ
📊 Found 10 active users

📝 Example 2: Parameterized Function

✅ Function saved: SFWnGwKcQtn_FwcuYfAH1QcNnz3dR8yHYq11Ehx8OphtpCytcYJP2PAH9nZc2DUPQA2JWGta2FFErWUlqPXY5w
📊 Found 10 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: zPGf8jdDAFIK4L3eOfpAAYOQr-umxrIwTgm8rzENWkt1v2JJyqRNhALGZbdTrqtZPDU5tz2D0GDInDl3cVW40w
📊 Statistics: 2 groups
   {'avg_score': 50.0, 'count': 5, 'status': 'inactive'}

   {'avg_score': 60.0, 'count': 5, 'status': 'active'}

📝 Example 4: function Management

📋 Total scripts: 18
🔍 Retrieved script: Get Active Users
✏️  function updated
🗑️  function deleted

ℹ️  Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.

✅ All examples completed!
=== ekoDB Function Composition Examples ===

📋 Setting up test data...

✅ Test data ready

📝 Example 1: Basic Function Composition

Building reusable functions that call each other...

✅ Saved reusable function: fetch_user
✅ Saved composed function: get_user_wrapper (calls fetch_user + projects fields)

📊 Result from composed function:
   Records: 1
   Name: {"type": "String", "value": "User 1"}
   Department: {"type": "String", "value": "engineering"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 39.0ms
   📊 Records: 1
   📦 Data: {
      "value": {
            "address": {
                  "city": "Gwenborough",
                  "geo": {
                        "lat": "-37.3159",
                        "lng": "81.1496"
    ...

Second call (cache hit - from cache):
   ⏱️  Duration: 38.7ms
   📊 Records: 1
   📦 Data: {
      "value": {
            "address": {
                  "city": "Gwenborough",
                  "geo": {
                        "lat": "-37.3159",
                        "lng": "81.1496"
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
🚀 ekoDB Python Complete Functions Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Function saved: rss52-8ucJVVKOxhxyyE02VZaBztBuQje4yu22NCuhPAsat_hE78vBZNTswRXQ2D3keX9qpmmUtpRBSHwtYfQw
📊 Found 2 product groups
   {'avg_price': 474.0, 'category': 'Furniture', 'count': 2}
   {'avg_price': 575.6666666666666, 'category': 'Electronics', 'count': 3}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   {'category': 'Electronics', 'count': 3}
   {'category': 'Furniture', 'count': 2}
⏱️  Execution time: 0ms

📝 Example 4: High Rating Products

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 5: function with Parameter Definition

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 6: Multi-Stage Pipeline (FindAll → Group → Count)

✅ Function saved
📊 Pipeline executed 3 stages
⏱️  Total execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All complete script examples finished!

💡 This example demonstrates ekoDB's function system:
   ✅ FindAll operations
   ✅ Group aggregations (Count, Average)
   ✅ Multi-stage pipelines (FindAll → Group → Count)
   ✅ Parameter definitions
   ✅ Function management (save, call, delete)
🚀 ekoDB Python KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: I1UsjM6P9PasvsXvn2T0gS6DRQplHzd5znkl6TlaAzdC6uUXPnwrdUyal3PHdfqCc7OD_xAVDsAQA74trsFdiA
✅ Inserted 2 products with wrapped types

📝 Example 2: Querying and Extracting Wrapped Types

📊 Found 2 products
   • Wireless Mouse
   • Laptop Pro

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {'value': '{"role":"admin","userId":"user_abc"}'}
🔍 Key exists: True
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Pattern Query

✅ Set 4 config entries
📊 Found 3 app config entries
📊 Found 4 total config entries

📝 Example 5: Combined Wrapped Types + KV Usage

✅ Inserted order: 46cLLnwl_0B6rIrUDKW2DUwn8__o8Vd5Hm0QhxIMJb-L7_5Yy2gwv7bKl1o9QWAHTP8C6XAbZ61_mmJggdgiKQ
✅ Cached order status
📊 Quick status lookup: {'value': '{"status":"processing","updated_at":"2026-04-05T05:18:39.345010+00:00"}'}

🧹 Cleaning up...
✅ Cleanup complete

✅ All KV & Wrapped Types examples completed!

💡 Key takeaways:
   ✅ Use field_* helpers for type-safe wrapped values
   ✅ field_decimal() preserves precision (no floating point errors)
   ✅ KV store is great for caching and quick lookups
   ✅ Combine KV caching with collection inserts for real workflows
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_api_user_py (acN3r_9cSq-Iaec8JCPzFN7y4N15Kc2qZf7TppbWmEX2ZiNDj0UEyVKtRdaHG2_OAT4UqvlFOvTVy7F-iwvU4Q)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
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
        "company": {
          "bs": "harness real-time e-markets",
          "catchPhrase": "Multi-layered client-server neural-net",
          "name": "Romaguera-Crona"
        },
        "email": "Sincere@april.biz",
        "id": 1,
        "name": "Leanne Graham",
        "phone": "1-770-736-8031 x56442",
        "username": "Bret",
        "website": "hildegard.org"
      }
    }
  ],
  "stats": {
    "execution_time_ms": 0,
    "input_count": 0,
    "output_count": 1,
    "stage_stats": [],
    "stages_executed": 2
  }
}
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 41ms (served from cache)
✓ Lightning fast cache hit

🧹 Cleaning up...
✓ Cleanup complete

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation
🚀 ekoDB Python Client - Native SWR Function Examples

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
✓ Created native SWR script: github_user_native (N5u-wXZrUpJeV2nBkFc44nlCY3pDQXySIARUsRlC5JVOboJzMhAgelsXzY-UsGW2cApiWOsUqBT7t-NUbIBbwg)

First call (cache miss - will fetch from GitHub API):
  Response time: 133ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 43ms
  Speedup: 3.1x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (eQ6p6zvqCwy8UBDoGGGjdCiGEh2G-waOJmAMSkT6P1WioMb9xqTX48L3lmGETq8TD-Drjw8gB8TUorOqcfy0HQ)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (UhsUBzUChmdisfgZsEgRPvcYnCDJMfT44Fhae6aZtVGoKGVVXjZWLV_E4tkzFg70tDrQACt-gksvMxlzpLRjAw)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (1Zf9wCQWJUBNaMN6pS_cj597f95esEeeKTmPaX8DoAhbTwztTQvQPECzjvAiViOmGXDRan-mxtsGYwYOUGIHiw)
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
🚀 ekoDB Python Advanced Functions Example

📋 Setting up test data...
✅ Created 8 products

📝 Example 1: List All Products

✅ Function saved
📊 Found 8 products
⏱️  Execution time: 0ms

📝 Example 2: Group Products by Category

✅ Function saved
📊 Category breakdown:
   {'avg_price': 365.6666666666667, 'category': 'Furniture', 'count': 3}
   {'avg_price': 367.0, 'category': 'Electronics', 'count': 5}
⏱️  Execution time: 0ms

📝 Example 3: Count Total Products

✅ Function saved
📊 Total products: [{'total': 8}]
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced script examples finished!
🚀 ekoDB Python AI Functions Example

📋 Setting up test data...
✅ Created 2 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
🤖 AI Response:
   Vector databases offer several benefits, including:

1. **Efficient Similarity Search**: They enable fast nearest neighbor searches based on high-dimensional vector embeddings, ideal for applications like recommendation systems and image retrieval.

2. **Scalability**: Designed to handle large volumes of data, vector databases can efficiently scale as the dataset grows.

3. **Flexibility**: They support various data types and can integrate with different machine learning frameworks, accommodating diverse applications.

4. **Enhanced Performance**: Optimized for parallel processing and indexing techniques, vector databases provide quicker query responses.

5. **Semantic Understanding**: They can capture the semantic relationships between data points, improving the accuracy of information retrieval and analytics.

6. **Real-time Processing**: Many vector databases support real-time data ingestion and querying, making them suitable for dynamic applications.

7. **Integration with AI/ML Workflows**: They can seamlessly integrate with AI and machine learning pipelines, facilitating model deployment and inference. 

These features make vector databases particularly valuable for applications in natural language processing, image recognition, and other AI-related tasks.
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embed script saved
📊 Embedding generated
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All AI script examples finished!

💡 This example demonstrates:
   ✅ Chat completions with system/user messages
   ✅ Embedding generation for text
🚀 ekoDB Python CRUD Functions Example

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
📊 Average score by role:
   {'avg_score': 20.0, 'count': 3, 'role': 'admin'}
   {'avg_score': 70.0, 'count': 7, 'role': 'user'}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD script examples finished!
🚀 ekoDB Python Search Functions Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Function saved
📊 Found 5 documents
   1. Introduction to Machine Learning (AI)
   2. Vector Databases Explained (Database)
   3. Database Design Principles (Database)
   4. Getting Started with ekoDB (Database)
   5. Natural Language Processing (AI)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   {'category': 'AI', 'count': 2}
   {'category': 'Database', 'count': 3}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: PpGkNdYNC-xnB_qboKh3_HG8uDOU5O227iZB5HnWpVP9lC2mN4foelxzmM9bDGDk0IcglQHjgCOjNpX_nKvFZg

=== Sending Chat Message ===
Message ID: qZ860dLhm6UJ9-oEcB2UZQqUL4nlZdIx1z0UOtS8ZtVe3jU3fy6d7xA_ayb7Y0kvG21JIpWIuew59qlrlSDMSA

=== AI Response ===
Here are the available products along with their prices:

1. **ekoDB**
   - Description: A high-performance database product with AI capabilities
   - Price: $99

2. **ekoDB Pro**
   - Description: Enterprise edition product with advanced features
   - Price: $299

3. **ekoDB Cloud**
   - Description: Fully managed cloud database service product
   - Price: $499

Execution Time: 2795ms

=== Token Usage ===
Prompt tokens: 1298
Completion tokens: 91
Total tokens: 1389

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 2mCLrDp7wKWQ3lwworciQjkAo1bVIo2hENAvQ7SOYbBYWnzcG43bz5jXGaTD7ZW98Fs9bH1C11Q16i8v7mHpFg

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: High-performance database product
- **Price**: $99

If you need more information or additional products, just let me know!

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['created_at', 'context_snippets', 'content', 'chat_id', 'id', 'llm_model', 'llm_provider', 'token_usage', 'updated_at', 'role'])
Debug: First message role: {'type': 'String', 'value': 'assistant'}
=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of ekoDB is $99. If you have any other questions or need further information, feel free to ask!

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: JqAsGGQYR6E_XIdp4OqFeNK0LW8NTvIgVoq90S1HsyDnDZ_-GRMk0qW9UGan26odpyK7yfl78jmp0zKdEurGYw
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
✓ Created session: QHGbuY3c63URk_CEyxJlf9FFov1UFbUvfYRMXtPTYRJFyfp53Dl5QMnJZ0EbeO5j3-Kac3ZkyiwL41v6rl1q1Q

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** A high-performance database product
- **Price:** $99

If you need more information or details about other products, please let me know!

✓ Message 2 sent
  Response: The price of ekoDB is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: ErwjQmD6Nwl4RqBdMwyWUlM-2BNTuSMsTYcvanyfhkYrcL60htUR-4-iBKCpkc4udtIFmD95Afuy8dW2HxbRsg
  Parent: QHGbuY3c63URk_CEyxJlf9FFov1UFbUvfYRMXtPTYRJFyfp53Dl5QMnJZ0EbeO5j3-Kac3ZkyiwL41v6rl1q1Q

=== Listing Sessions ===
✓ Found 7 sessions
  Session 1: ErwjQmD6Nwl4RqBdMwyWUlM-2BNTuSMsTYcvanyfhkYrcL60htUR-4-iBKCpkc4udtIFmD95Afuy8dW2HxbRsg (Untitled)
  Session 2: QHGbuY3c63URk_CEyxJlf9FFov1UFbUvfYRMXtPTYRJFyfp53Dl5QMnJZ0EbeO5j3-Kac3ZkyiwL41v6rl1q1Q (Untitled)
  Session 3: JqAsGGQYR6E_XIdp4OqFeNK0LW8NTvIgVoq90S1HsyDnDZ_-GRMk0qW9UGan26odpyK7yfl78jmp0zKdEurGYw (Untitled)
  Session 4: PpGkNdYNC-xnB_qboKh3_HG8uDOU5O227iZB5HnWpVP9lC2mN4foelxzmM9bDGDk0IcglQHjgCOjNpX_nKvFZg (Untitled)
  Session 5: LvpKy5X7XyHYUFkFEbAwarTOb5exXjVWY50-t-BRvzR-jFmRAQxIToU785iVfC5rsjABggQXAa4bgNmTTTLfJQ (Untitled)
  Session 6: gCtghcCKk2NkoWeR3aULAGempVykwxF17RqUM8RXu14KPh2n23DD1tU8su1vK6OPrKJsLR5VJP80NPx5mpanBg (Untitled)
  Session 7: YtHj__UZG0cxt1hEZH9f-8CiAk5kGd22qzWWvRrs2lhWGFArJZ-mKe_HKoGMfVpW1qVT5jRV0nfGbAsbK64T9w (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: ErwjQmD6Nwl4RqBdMwyWUlM-2BNTuSMsTYcvanyfhkYrcL60htUR-4-iBKCpkc4udtIFmD95Afuy8dW2HxbRsg

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Dict Creation ===
✓ Created record with plain dict: {'id': 'Ma-bfB5QtOs7owoO5RO708nPe3NVoZdlw9lUO3e1J3itiHDJM1sbpHUb9AWj3SogAR6zTBVnuas5c8p88Je9PQ'}

=== Upsert Operation ===
✓ Upsert (update existing record): Ma-bfB5QtOs7owoO5RO708nPe3NVoZdlw9lUO3e1J3itiHDJM1sbpHUb9AWj3SogAR6zTBVnuas5c8p88Je9PQ
✓ Inserted second record: 9BRNFCPIfpSuzWQJkMpjCtDuKtn1ZN_ugg5b3XWrkCCj5A9zN6ZajepBEZcc8ydZDAW50CaezuodrsIpPH6fiA
✓ Upsert (update second record): 9BRNFCPIfpSuzWQJkMpjCtDuKtn1ZN_ugg5b3XWrkCCj5A9zN6ZajepBEZcc8ydZDAW50CaezuodrsIpPH6fiA

=== Find One Operation ===
✓ Found user by email: {'age': {'type': 'Integer', 'value': 29}, 'active': {'type': 'Boolean', 'value': True}, 'id': 'Ma-bfB5QtOs7owoO5RO708nPe3NVoZdlw9lUO3e1J3itiHDJM1sbpHUb9AWj3SogAR6zTBVnuas5c8p88Je9PQ', 'name': {'value': 'Alice Johnson', 'type': 'String'}, 'email': {'type': 'String', 'value': 'alice.j@newdomain.com'}}
✓ User not found (as expected)

=== Exists Check ===
✓ Record exists: True
✓ Fake record exists: False (should be False)

=== Pagination ===
✓ Inserted 25 records for pagination
✓ Page 1: 10 records (expected 10)
✓ Page 2: 10 records (expected 10)
✓ Page 3: 7 records (expected ~7)

=== Cleanup ===
✓ Deleted collection

✅ All convenience methods demonstrated successfully!
=== Bypass Ripple Example ===

1. Basic insert (ripple enabled):
   Inserted with ripple: {'id': 'WikiGXs9CBc8Lw3oPc32cqs26lQAP0fScHPVobkz-Js403HqGlWJtLpkvzQR8gx7e6wc3P8mx0ssDBiQwj7oQw'}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {'id': 'wj7uD9Dvbi4dZwAawj9z6n28o433wiGwUZFeXvDimEoRS3tATWIHvWaaTyjEaknY4PHWmHt-_3Gv3Mms5RpiMg'}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {'name': {'value': 'Product 1', 'type': 'String'}, 'id': 'WikiGXs9CBc8Lw3oPc32cqs26lQAP0fScHPVobkz-Js403HqGlWJtLpkvzQR8gx7e6wc3P8mx0ssDBiQwj7oQw', 'price': {'value': 150, 'type': 'Integer'}}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {'price': {'value': 500, 'type': 'Integer'}, 'id': 'WikiGXs9CBc8Lw3oPc32cqs26lQAP0fScHPVobkz-Js403HqGlWJtLpkvzQR8gx7e6wc3P8mx0ssDBiQwj7oQw', 'name': {'type': 'String', 'value': 'Upsert Product'}}

✅ All bypass_ripple operations completed successfully!
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields (id, name, email only)
  Found 3 active users
  Fields returned: ['name', 'id', 'email']
  First user: {'type': 'String', 'value': 'Bob Smith'} <{'value': 'bob@example.com', 'type': 'String'}>

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ['status', 'name', 'age', 'bio', 'user_role', 'email', 'created_at', 'id', 'avatar_url']

Example 3: Complex query with projection (active users, ages 18-65)
  Found 3 active users (ages 18-65)
    - {'value': 'Dave Brown', 'type': 'String'} (age {'type': 'Integer', 'value': 45})
    - {'type': 'String', 'value': 'Alice Johnson'} (age {'type': 'Integer', 'value': 30})
    - {'value': 'Bob Smith', 'type': 'String'} (age {'type': 'Integer', 'value': 25})

Example 4: Query inactive users with profile fields
  Found 1 inactive users
    - {'value': 'Carol White', 'type': 'String'}: {'type': 'String', 'value': 'Manager'}

Example 5: Compare full vs projected data
  Full query:
    - 12 fields per record
    - Fields: ['id', 'name', 'created_at', 'status', 'age', 'email', 'secret_token', 'api_key', 'avatar_url', 'bio', 'user_role', 'password']
  Projected query:
    - 3 fields per record
    - Fields: ['id', 'email', 'name']
  Bandwidth savings: ~75% fewer fields

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
=== KV Precision: Float vs Decimal ===

=== Test 1: Using Python Floats (LOSES PRECISION) ===
✓ Stored products with float prices

Retrieved float prices:
  Widget A: $29.99 (expected $29.99) ✓
  Widget B: $39.99 (expected $39.99) ✓
  Widget C: $49.99 (expected $49.99) ✓

=== Test 2: Using field_decimal() (PRESERVES PRECISION) ===
✓ Stored products with decimal prices

Retrieved decimal prices:
  Widget A: $29.99 (expected $29.99) ✓
  Widget B: $39.99 (expected $39.99) ✓
  Widget C: $49.99 (expected $49.99) ✓

=== Test 3: Sum Calculation Comparison ===
  Float sum: $119.97 (expected $119.97)
  Decimal sum: $119.97 (expected $119.97)

=== Test 4: Extreme Precision Example ===
  Float 0.1 + 0.2 = 0.30000000000000004 (should be 0.3)
  Decimal "0.30" = 0.30 (exact!)

=== Cleanup ===
✓ Cleaned up test keys

=== Summary ===
✅ Use field_decimal() for monetary values, percentages, and
   any case where floating-point errors are unacceptable.
✅ field_decimal() stores values as strings internally,
   preserving exact precision across all operations.
✓ Client created

=== Get All Chat Models ===
OpenAI models: ['gpt-4-0613', 'gpt-4', 'gpt-3.5-turbo', 'gpt-5.4-mini', 'gpt-5.4', 'gpt-5.4-nano-2026-03-17', 'gpt-5.4-nano', 'gpt-5.4-mini-2026-03-17', 'davinci-002', 'babbage-002', 'gpt-3.5-turbo-instruct', 'gpt-3.5-turbo-instruct-0914', 'dall-e-3', 'dall-e-2', 'gpt-3.5-turbo-1106', 'tts-1-hd', 'tts-1-1106', 'tts-1-hd-1106', 'text-embedding-3-small', 'text-embedding-3-large', 'gpt-3.5-turbo-0125', 'gpt-4-turbo', 'gpt-4-turbo-2024-04-09', 'gpt-4o', 'gpt-4o-2024-05-13', 'gpt-4o-mini-2024-07-18', 'gpt-4o-mini', 'gpt-4o-2024-08-06', 'gpt-4o-audio-preview', 'gpt-4o-realtime-preview', 'omni-moderation-latest', 'omni-moderation-2024-09-26', 'gpt-4o-realtime-preview-2024-12-17', 'gpt-4o-audio-preview-2024-12-17', 'gpt-4o-mini-realtime-preview-2024-12-17', 'gpt-4o-mini-audio-preview-2024-12-17', 'o1-2024-12-17', 'o1', 'gpt-4o-mini-realtime-preview', 'gpt-4o-mini-audio-preview', 'o3-mini', 'o3-mini-2025-01-31', 'gpt-4o-2024-11-20', 'gpt-4o-mini-search-preview-2025-03-11', 'gpt-4o-mini-search-preview', 'gpt-4o-transcribe', 'gpt-4o-mini-transcribe', 'o1-pro-2025-03-19', 'o1-pro', 'gpt-4o-mini-tts', 'o3-2025-04-16', 'o4-mini-2025-04-16', 'o3', 'o4-mini', 'gpt-4.1-2025-04-14', 'gpt-4.1', 'gpt-4.1-mini-2025-04-14', 'gpt-4.1-mini', 'gpt-4.1-nano-2025-04-14', 'gpt-4.1-nano', 'gpt-image-1', 'gpt-4o-realtime-preview-2025-06-03', 'gpt-4o-audio-preview-2025-06-03', 'o4-mini-deep-research', 'gpt-4o-transcribe-diarize', 'o4-mini-deep-research-2025-06-26', 'gpt-5-chat-latest', 'gpt-5-2025-08-07', 'gpt-5', 'gpt-5-mini-2025-08-07', 'gpt-5-mini', 'gpt-5-nano-2025-08-07', 'gpt-5-nano', 'gpt-audio-2025-08-28', 'gpt-realtime', 'gpt-realtime-2025-08-28', 'gpt-audio', 'gpt-5-codex', 'gpt-image-1-mini', 'gpt-5-pro-2025-10-06', 'gpt-5-pro', 'gpt-audio-mini', 'gpt-audio-mini-2025-10-06', 'gpt-5-search-api', 'gpt-realtime-mini', 'gpt-realtime-mini-2025-10-06', 'sora-2', 'sora-2-pro', 'gpt-5-search-api-2025-10-14', 'gpt-5.1-chat-latest', 'gpt-5.1-2025-11-13', 'gpt-5.1', 'gpt-5.1-codex', 'gpt-5.1-codex-mini', 'gpt-5.1-codex-max', 'gpt-image-1.5', 'gpt-5.2-2025-12-11', 'gpt-5.2', 'gpt-5.2-pro-2025-12-11', 'gpt-5.2-pro', 'gpt-5.2-chat-latest', 'gpt-4o-mini-transcribe-2025-12-15', 'gpt-4o-mini-transcribe-2025-03-20', 'gpt-4o-mini-tts-2025-03-20', 'gpt-4o-mini-tts-2025-12-15', 'gpt-realtime-mini-2025-12-15', 'gpt-audio-mini-2025-12-15', 'chatgpt-image-latest', 'gpt-5.2-codex', 'gpt-5.3-codex', 'gpt-realtime-1.5', 'gpt-audio-1.5', 'gpt-4o-search-preview', 'gpt-4o-search-preview-2025-03-11', 'gpt-5.3-chat-latest', 'gpt-5.4-2026-03-05', 'gpt-5.4-pro', 'gpt-5.4-pro-2026-03-05', 'gpt-3.5-turbo-16k', 'tts-1', 'whisper-1', 'text-embedding-ada-002']
Anthropic models: ['claude-sonnet-4-6', 'claude-opus-4-6', 'claude-opus-4-5-20251101', 'claude-haiku-4-5-20251001', 'claude-sonnet-4-5-20250929', 'claude-opus-4-1-20250805', 'claude-opus-4-20250514', 'claude-sonnet-4-20250514', 'claude-3-haiku-20240307']
Perplexity models: ['sonar', 'sonar-pro', 'sonar-deep-research', 'sonar-reasoning', 'sonar-reasoning-pro']

=== Get OpenAI Models ===
OpenAI models: ['gpt-4-0613', 'gpt-4', 'gpt-3.5-turbo', 'gpt-5.4-mini', 'gpt-5.4', 'gpt-5.4-nano-2026-03-17', 'gpt-5.4-nano', 'gpt-5.4-mini-2026-03-17', 'davinci-002', 'babbage-002', 'gpt-3.5-turbo-instruct', 'gpt-3.5-turbo-instruct-0914', 'dall-e-3', 'dall-e-2', 'gpt-3.5-turbo-1106', 'tts-1-hd', 'tts-1-1106', 'tts-1-hd-1106', 'text-embedding-3-small', 'text-embedding-3-large', 'gpt-3.5-turbo-0125', 'gpt-4-turbo', 'gpt-4-turbo-2024-04-09', 'gpt-4o', 'gpt-4o-2024-05-13', 'gpt-4o-mini-2024-07-18', 'gpt-4o-mini', 'gpt-4o-2024-08-06', 'gpt-4o-audio-preview', 'gpt-4o-realtime-preview', 'omni-moderation-latest', 'omni-moderation-2024-09-26', 'gpt-4o-realtime-preview-2024-12-17', 'gpt-4o-audio-preview-2024-12-17', 'gpt-4o-mini-realtime-preview-2024-12-17', 'gpt-4o-mini-audio-preview-2024-12-17', 'o1-2024-12-17', 'o1', 'gpt-4o-mini-realtime-preview', 'gpt-4o-mini-audio-preview', 'o3-mini', 'o3-mini-2025-01-31', 'gpt-4o-2024-11-20', 'gpt-4o-mini-search-preview-2025-03-11', 'gpt-4o-mini-search-preview', 'gpt-4o-transcribe', 'gpt-4o-mini-transcribe', 'o1-pro-2025-03-19', 'o1-pro', 'gpt-4o-mini-tts', 'o3-2025-04-16', 'o4-mini-2025-04-16', 'o3', 'o4-mini', 'gpt-4.1-2025-04-14', 'gpt-4.1', 'gpt-4.1-mini-2025-04-14', 'gpt-4.1-mini', 'gpt-4.1-nano-2025-04-14', 'gpt-4.1-nano', 'gpt-image-1', 'gpt-4o-realtime-preview-2025-06-03', 'gpt-4o-audio-preview-2025-06-03', 'o4-mini-deep-research', 'gpt-4o-transcribe-diarize', 'o4-mini-deep-research-2025-06-26', 'gpt-5-chat-latest', 'gpt-5-2025-08-07', 'gpt-5', 'gpt-5-mini-2025-08-07', 'gpt-5-mini', 'gpt-5-nano-2025-08-07', 'gpt-5-nano', 'gpt-audio-2025-08-28', 'gpt-realtime', 'gpt-realtime-2025-08-28', 'gpt-audio', 'gpt-5-codex', 'gpt-image-1-mini', 'gpt-5-pro-2025-10-06', 'gpt-5-pro', 'gpt-audio-mini', 'gpt-audio-mini-2025-10-06', 'gpt-5-search-api', 'gpt-realtime-mini', 'gpt-realtime-mini-2025-10-06', 'sora-2', 'sora-2-pro', 'gpt-5-search-api-2025-10-14', 'gpt-5.1-chat-latest', 'gpt-5.1-2025-11-13', 'gpt-5.1', 'gpt-5.1-codex', 'gpt-5.1-codex-mini', 'gpt-5.1-codex-max', 'gpt-image-1.5', 'gpt-5.2-2025-12-11', 'gpt-5.2', 'gpt-5.2-pro-2025-12-11', 'gpt-5.2-pro', 'gpt-5.2-chat-latest', 'gpt-4o-mini-transcribe-2025-12-15', 'gpt-4o-mini-transcribe-2025-03-20', 'gpt-4o-mini-tts-2025-03-20', 'gpt-4o-mini-tts-2025-12-15', 'gpt-realtime-mini-2025-12-15', 'gpt-audio-mini-2025-12-15', 'chatgpt-image-latest', 'gpt-5.2-codex', 'gpt-5.3-codex', 'gpt-realtime-1.5', 'gpt-audio-1.5', 'gpt-4o-search-preview', 'gpt-4o-search-preview-2025-03-11', 'gpt-5.3-chat-latest', 'gpt-5.4-2026-03-05', 'gpt-5.4-pro', 'gpt-5.4-pro-2026-03-05', 'gpt-3.5-turbo-16k', 'tts-1', 'whisper-1', 'text-embedding-ada-002']

=== Get Anthropic Models ===
Anthropic models: ['claude-sonnet-4-6', 'claude-opus-4-6', 'claude-opus-4-5-20251101', 'claude-haiku-4-5-20251001', 'claude-sonnet-4-5-20250929', 'claude-opus-4-1-20250805', 'claude-opus-4-20250514', 'claude-sonnet-4-20250514', 'claude-3-haiku-20240307']

✓ Chat Models API example complete
✓ Client created

=== Create User Function ===
Created user function with ID: 46sRu3j9C3X8ScS7wr6M9NNkUYjUkjET1ntcUtQjn6VfHDou861cnKHjl-a-7PzOJxLIODrTS0hy8JUmXfKDyg

=== Get User Function ===
Retrieved: get_active_users_py - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 25 user functions:
  - swr_user: SWR pattern for user data (KV-based)
  - get_verified_user: Get verified and validated user
  - get_user_wrapper: Wrapper that calls fetch_user
  - cache_api_call_py: Cache External API Call
  - get_active_users: Get Active Users (Updated)
  - fetch_and_store_user: Fetch user from API and cache in KV
  - swr_user: SWR pattern for user data (KV-based)
  - get_high_scoring_active_users: Get High Scoring Active Users
  - get_active_users_paginated: Get Active Users (Paginated)
  - get_users_by_status: Get Users By Status
  - get_active_users_paginated: Get Active Users (Paginated)
  - fetch_user: Fetch user by code
  - validate_user: Check if user exists
  - get_active_users_updated: Get Active Users (Updated)
  - get_users_by_status: Get Users By Status
  - fetch_user: Fetch user by code
  - validate_user: Check if user exists
  - fetch_slim_user: Validate and slim down user
  - get_active_users_updated: Get Active Users (Updated)
  - get_verified_user: Get verified and validated user
  - get_active_users_py: Get Active Users
  - fetch_slim_user: Validate and slim down user
  - fetch_and_store_user: Fetch user from API and cache in KV
  - get_user_wrapper: Wrapper that calls fetch_user
  - get_active_users: Get Active Users (Updated)

=== Update User Function ===
User function updated successfully

=== Delete User Function ===
User function deleted successfully

✓ User Functions API example complete
✓ Client created

=== Check Collection Exists (Before Creation) ===
Collection 'collection_utils_test_py' exists: False

=== Creating Test Documents ===
Created 5 test documents

=== Check Collection Exists (After Creation) ===
Collection 'collection_utils_test_py' exists: True

=== Count Documents ===
Document count in 'collection_utils_test_py': 5

=== Check Non-Existent Collection ===
Collection 'nonexistent_collection_xyz' exists: False

=== Cleanup ===
Deleted collection 'collection_utils_test_py'

✓ Collection Utilities example complete
✅ [32mPython client examples complete![0m
