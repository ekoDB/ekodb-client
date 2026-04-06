make test-examples-python
🧪 [36mRunning Python examples (direct HTTP/WebSocket)...[0m
=== Simple CRUD Operations (Direct HTTP) ===

✓ Authentication successful

=== Insert Document ===
Inserted: {'id': 'SD-ce07b5sn2RYTZj-Fy15zdMRVgcyd3yq2cDqHUAjt18b7ol25H0G8aB1iwvPC4VPyFC-N8_HhB52Ua37jP6w'}

=== Find by ID ===
Found: {'id': 'SD-ce07b5sn2RYTZj-Fy15zdMRVgcyd3yq2cDqHUAjt18b7ol25H0G8aB1iwvPC4VPyFC-N8_HhB52Ua37jP6w', 'active': {'type': 'Boolean', 'value': True}, 'value': {'value': 42, 'type': 'Integer'}, 'name': {'type': 'String', 'value': 'Test Record'}}

=== Find with Query ===
Found documents: [{'name': {'value': 'Test Record', 'type': 'String'}, 'value': {'type': 'Integer', 'value': 42}, 'id': 'SD-ce07b5sn2RYTZj-Fy15zdMRVgcyd3yq2cDqHUAjt18b7ol25H0G8aB1iwvPC4VPyFC-N8_HhB52Ua37jP6w', 'active': {'type': 'Boolean', 'value': True}}]

=== Update Document ===
Updated: {'id': 'SD-ce07b5sn2RYTZj-Fy15zdMRVgcyd3yq2cDqHUAjt18b7ol25H0G8aB1iwvPC4VPyFC-N8_HhB52Ua37jP6w', 'value': {'value': 100, 'type': 'Integer'}, 'name': {'type': 'String', 'value': 'Updated Record'}, 'active': {'value': True, 'type': 'Boolean'}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: sl9qJa63YqamwdnLwuQ-IPi4si3m3r7hOpnW-Kztm_8W-Z8o6YzIroIr50a990xpHKfaqhdG798B5mV6dzTnUA

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "messageId": "771884874",
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
        "id": "-f0h1copZhZ1-1UgYMQ-hXlAMY2TRqZRvIjv-flcOdAOifuyhypOS0w1NG7mqVGDd1BEUcdA_hwXHEBHJ__3og",
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

✅ Function saved: Z22LQkADbK-QXYw-KbtL0tA2fS-04hA4VyNd9qqn05l0hVxXOBQkknATW5oH7x2VemFmt2kQxGKkkNtKFNfdbA
📊 Found 15 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Function saved: MT7Sp4KJGdw6EUKaeusfneimrFEeeq6BBZmc1aqjCvXDR2sxJChhvHVNwRJuKEpeq19pwOwhHMnoalSDeQ4f2g
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: edNkuQSDp066YusRSZvXG4pfJC4C7paqoPLSGlJf82F9eShrDJJaBB1UFgpvI6RzQroqiicVid42SXRMA_WNgA
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {'count': 15, 'max_score': 90, 'status': 'inactive', 'avg_score': 50.0}
   {'count': 15, 'status': 'active', 'avg_score': 60.0, 'max_score': 100}

📝 Example 4: Function Management

📋 Total scripts: 15
🔍 Retrieved script: Get Active Users
✏️  Function updated
🗑️  Function deleted

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
cache:product:3: {'name': 'Product 3', 'price': 49.989999999999995}

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
=== Collection Management (Direct HTTP) ===

✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: J-EOid4N7ZBk2518bFUlbD5lfRzs2DJxW9nuYon0fKcGMoVMZBLJ9dh1V-yIfNBHdr9hMnDdH5stCd_NFz8nhA

=== List Collections ===
Total collections: 13
Sample collections: ['test_collection', 'functions__ek0_testing', 'demo_collection', 'chat_configurations__ek0_testing', 'websocket_test']

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
Created Alice: $1000 - ID: c_sjV1CVr7dcy9m6TB_poizNOZ0CJaaIaj1kg4xrfrWUK0Yv0fA5Nki1qketx7Z3iS36DoWj4Tf97eE_BuzjMQ
Created Bob: $500 - ID: jRgeywyFbJ6h5khVWFvWeRRf8fekF3fyed4IBQbhPn-rsCbonOrwjKszLNL6Hy-aUZ_jZG0rjyPqCbL2Dil23Q

=== Example 1: Begin Transaction ===
Transaction ID: e042f89e-1ba2-4aca-bf1c-acad22314219

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
New transaction: 3180d699-3b92-4cc3-ad9c-a9760133e91d
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'type': 'Integer', 'value': 700}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 2Gn1y5DEGHRgl4xnZ7R3wcexN6e4ktRuJ4TYzeTMMrNu2ymUkpH0PSBG6RWwbrbqt6tvg3lyGsZymXOmq4iQkA
Created Bob: $500 - ID: _9MPARqyVFc3Z_DIerAOUOaRdGja0iihIAq5AwrcZwcNTmtN3jYb2YpTVJX2SwZE0xUg5r3GYbWBNkhfL2E9UA

=== Example 1: Begin Transaction ===
Transaction ID: 274f8e6c-4cac-4006-b2da-ea3c720a7e2c

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


🔄 Switching to NON-DURABLE mode...
   ✓ Config updated: durable_operations=false


============================================================
🔥 TEST 2: Non-Durable Mode (durable=false)
============================================================

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: T-OJddCwWQAxVnKcRJfNfiwS-t67_bKd8N0u8y7az0rK1ZdmWE11Um_xIiOBAE8qHs1hVg82nJHEkRzuqq29RQ
Created Bob: $500 - ID: 3jTVuQxgw0Ig7c-RJoipx8WOiNEchB1aYF7Mfh3ulkQWmmxgGzXlszX2EIuOCB4GpzpLD5sRztyK2PdaEHBwmg

=== Example 1: Begin Transaction ===
Transaction ID: 9f4f27fc-0526-4088-971b-8d548844e9f6

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: ${'value': 800, 'type': 'Integer'}
Bob: ${'type': 'Integer', 'value': 700}

=== Example 5: Rollback ===
New transaction: 9b29c248-68b4-4ed1-a57f-c319f830d0d2
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'type': 'Integer', 'value': 700}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 12NpG4fa6y6MullTuu4tgFkZtVPHn8_y7gPqm7SOSsYU6SVGgkxpW37DpIrxJkL8ubqBq9t-v3aAqUEhlYwVLQ
Created Bob: $500 - ID: tQn5UP-Kpkr1V-GcQpgnda3wDweV9K_FQX87gKfvk5-PT15UrsGfoDuZmgxG544b3gNcWBDpSoGBwzIrrL22pw

=== Example 1: Begin Transaction ===
Transaction ID: edceebc5-705c-4146-a03c-bdb7001f8b73

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


🔄 Switching to DURABLE mode...
   ✓ Config updated: durable_operations=true


============================================================
🔥 TEST 3: Durable Mode (durable=true)
============================================================

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 6lRGK_rDIKYAf3rIjJGl7h2ffz6ggmxDory9hOuKjUt1Q4Ui29UuaFSRUmwG2VPhLsystpK3W53500Wvwnr0iA
Created Bob: $500 - ID: 9bfG_ZaE-xGDy2D-FjdZOXuoNWJ5wsdhfOg-IzTE8oZq6TsSlRE6eFsVx42i_Zfx1snHDnfk90dD_5KBCmXMLQ

=== Example 1: Begin Transaction ===
Transaction ID: 80a0f08a-b498-4368-9b13-0319f1a429db

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: ${'value': 800, 'type': 'Integer'}
Bob: ${'type': 'Integer', 'value': 700}

=== Example 5: Rollback ===
New transaction: 4b7d2026-6399-4588-946f-13c62cf75e38
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'type': 'Integer', 'value': 700}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: wdFe75O2Vmlr27suzjqpsY6Cqf859vEL2JKvDQZWHVwdrpNxyN1MTV4mswOJb8y8nGsSfcKavWMvivafJnY-bg
Created Bob: $500 - ID: k4mdBPzQ2CTj61_i7IRIalRm88Uyd5EZZhyyhAFdpTRi6wGZMYf8IGobDjaNcgv4pwodwzaCN45v68wI3ktZrQ

=== Example 1: Begin Transaction ===
Transaction ID: 07487448-8ab1-476c-be91-951b73e2307a

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: ${'value': 800, 'type': 'Integer'}
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
   ✅ Function saved: PXrLiJCZq6KG7xk9_9b4SoI_xn2XBmGWoQSaOboyLvHgdaFytd0HqpB7D7TEYGtQoj5fxgG9CcApu6MxMXoboQ

2️⃣ Calling function (Insert + Verify)...
   ✅ function executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: GqY5auGLxrGUlMpIfEspAZy0padoNb5ZZKgNjvY7aJ9Rl9LCPXcdRicK3RdS0vutf0u6QrB5xrZOntR-YUB-VA
   📋 Name: {'value': 'Alice Smith', 'type': 'String'}
   📋 Email: {'value': 'alice@example.com', 'type': 'String'}
   📋 Status: {'value': 'pending', 'type': 'String'}
   📋 Credits: {'type': 'Integer', 'value': 0}

============================================================
📝 function 2: Query + Update + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: uewU68lJm9lL0V876FMOYTkOwWl-NGuzaid4-HRe1vJ9p6aD4T1OO88kVbADYpZPTdKI_ouzTlj3lrMJg4f0Fg

2️⃣ Calling function (Query + Update + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Status updated to: {'value': 'active', 'type': 'String'}
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}

============================================================
📝 function 3: Query + Update Credits + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: SPBZSTOHUsJxwxQ8kjZ1HT7S4yQ5qj0NJ3cGKvuveBi6K5tJNZ7oCN8eWJt_diCsKi0UIFO0vpJlkaNjupqaGQ

2️⃣ Calling function (Query + Update Credits + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {'value': 0, 'type': 'Integer'}
   📋 Status: {'value': 'active', 'type': 'String'}
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}

============================================================
📝 function 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: c8E_NzYrE764pVm9nKP_7ZDKL-RS-ZC96EKXm28yblljqGQvJh1bYoebU1dBivrVSNammM2cYYPTpS_cV_xLHw

2️⃣ Calling function (Query + Delete + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: PXrLiJCZq6KG7xk9_9b4...
   ✅ Deleted script: uewU68lJm9lL0V876FMO...
   ✅ Deleted script: SPBZSTOHUsJxwxQ8kjZ1...
   ✅ Deleted script: c8E_NzYrE764pVm9nKP_...
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
  Output: Document ID = PoXgFVMJ53oLMq_107A3ic6x4vPv6IpkcfXyv8npBBr-Dpii_cmAjG9Xdct6ifDaYD67M4y6_r0pBmIIASMckw
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: find_by_id(PoXgFVMJ53oLMq_107A3ic6x4vPv6IpkcfXyv8npBBr-Dpii_cmAjG9Xdct6ifDaYD67M4y6_r0pBmIIASMckw)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: find_by_id(PoXgFVMJ53oLMq_107A3ic6x4vPv6IpkcfXyv8npBBr-Dpii_cmAjG9Xdct6ifDaYD67M4y6_r0pBmIIASMckw)
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
  Output: Document ID = p121nN7hgj_toVIDaYWhOVp3yjlXIT_Lenfb-QNWNMxdDQk1S6ueg4fZolpRLU1KrclTSlySyCR9KvPKiLe-tA
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: find_by_id(p121nN7hgj_toVIDaYWhOVp3yjlXIT_Lenfb-QNWNMxdDQk1S6ueg4fZolpRLU1KrclTSlySyCR9KvPKiLe-tA)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: find_by_id(p121nN7hgj_toVIDaYWhOVp3yjlXIT_Lenfb-QNWNMxdDQk1S6ueg4fZolpRLU1KrclTSlySyCR9KvPKiLe-tA)
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
    Finished `release` profile [optimized] target(s) in 0.15s
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
Inserted: {'id': 'T_RozQgOFfP3r0T0MJSSAY3ZOUwixpe6mtQAFJSmvgYmC1KYlxKSrqw7-hxczdjfW5XpS1i_0f85EXC1LW3QvQ'}

=== Find by ID ===
Found: {'active': {'value': True, 'type': 'Boolean'}, 'created_at': {'value': '2026-04-06T10:33:03.673507', 'type': 'String'}, 'id': 'T_RozQgOFfP3r0T0MJSSAY3ZOUwixpe6mtQAFJSmvgYmC1KYlxKSrqw7-hxczdjfW5XpS1i_0f85EXC1LW3QvQ', 'data': {'value': 'aGVsbG8gd29ybGQ=', 'type': 'String'}, 'price': {'value': 99.99, 'type': 'Float'}, 'tags': {'value': ['tag1', 'tag2', 'tag3'], 'type': 'Array'}, 'categories': {'type': 'Array', 'value': ['electronics', 'computers']}, 'embedding': {'value': [0.1, 0.2, 0.3, 0.4, 0.5], 'type': 'Array'}, 'value': {'type': 'Integer', 'value': 42}, 'metadata': {'value': {'key': 'value', 'nested': {'deep': True}}, 'type': 'Object'}, 'name': {'value': 'Test Record', 'type': 'String'}, 'user_id': {'value': '550e8400-e29b-41d4-a716-446655440000', 'type': 'String'}}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): True
  price (Decimal): 99.99
  created_at (DateTime): 2026-04-06 10:33:03.673507
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): ['tag1', 'tag2', 'tag3']
  metadata (Object): {'key': 'value', 'nested': {'deep': True}}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): ['electronics', 'computers']
  data (Bytes): 11 bytes
Plain record: {'active': True, 'created_at': '2026-04-06T10:33:03.673507', 'id': 'T_RozQgOFfP3r0T0MJSSAY3ZOUwixpe6mtQAFJSmvgYmC1KYlxKSrqw7-hxczdjfW5XpS1i_0f85EXC1LW3QvQ', 'data': 'aGVsbG8gd29ybGQ=', 'price': 99.99, 'tags': ['tag1', 'tag2', 'tag3'], 'categories': ['electronics', 'computers'], 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5], 'value': 42, 'metadata': {'key': 'value', 'nested': {'deep': True}}, 'name': 'Test Record', 'user_id': '550e8400-e29b-41d4-a716-446655440000'}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'user_id': {'type': 'String', 'value': '550e8400-e29b-41d4-a716-446655440000'}, 'id': 'T_RozQgOFfP3r0T0MJSSAY3ZOUwixpe6mtQAFJSmvgYmC1KYlxKSrqw7-hxczdjfW5XpS1i_0f85EXC1LW3QvQ', 'tags': {'type': 'Array', 'value': ['tag1', 'tag2', 'tag3']}, 'categories': {'value': ['electronics', 'computers'], 'type': 'Array'}, 'name': {'type': 'String', 'value': 'Updated Record'}, 'metadata': {'value': {'nested': {'deep': True}, 'key': 'value'}, 'type': 'Object'}, 'value': {'value': 100, 'type': 'Integer'}, 'price': {'value': 99.99, 'type': 'Float'}, 'created_at': {'type': 'String', 'value': '2026-04-06T10:33:03.673507'}, 'active': {'type': 'Boolean', 'value': True}, 'data': {'type': 'String', 'value': 'aGVsbG8gd29ybGQ='}, 'embedding': {'value': [0.1, 0.2, 0.3, 0.4, 0.5], 'type': 'Array'}}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: 24hvDOZlKJwaGGb9I5VZAxqL3OebiTB00Evav_FOkHPY7rDJ4dsfcxQ1ceu55_Ja1_cCAeYC5NpUtmlYaq3c-g

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
Collection created with first record: "Ar1t-sD03ZxRGYAj9NxhZ8u3PmLnSgDqfXldNMZORTX1pTB_kmi2q_LqKaKjGPx-UUnoW8SUr77lMs5w1JtQPg"

=== List Collections ===
Total collections: 12
Sample collections: ['client_collection_management_python', 'test_collection', 'functions__ek0_testing', 'chat_configurations__ek0_testing', 'websocket_test']

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
  cache:product:1: {'name': 'Product 1', 'price': 29.99}
  cache:product:2: {'price': 39.99, 'name': 'Product 2'}
  cache:product:3: {'name': 'Product 3', 'price': 49.99}

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
Created Alice: $1000 - ID: 283GLkR2v2qKnxHX2K3AV02mXF4jh7v7jSkU-yXFySpgMAJvDVcQWkHdZQ9pjhlN3WGsNA06Qzs_X88DRVWPcQ
Created Bob: $500 - ID: qxPbdf-IE9Gzdtr_YtK_b84WTaJ4cFeAwHiQm3UMDx0Tfu2yM3AmoZkotmn4AaXTa99rXODJXNw4kHjKlos1nA

=== Example 1: Begin Transaction ===
Transaction ID: 12645d1a-da24-454b-ab11-ef1be89b601b

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 87dfeba7-901c-4608-8376-fc1387bce045
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
  1. Score: 25.740, Matched: email.value, name, name.value, email
  2. Score: 12.540, Matched: name.value, name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: bio, bio.value, title, title.value
  2. Score: 26.400, Matched: bio.value, title, bio, title.value
  3. Score: 26.400, Matched: bio.value, bio, title, title.value
  4. Score: 26.400, Matched: title, title.value, bio, bio.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio.value, bio, title.value, title
  2. Score: 39.600, Matched: bio.value, title.value, bio, title
  3. Score: 39.600, Matched: title, bio.value, title.value, bio
  4. Score: 39.600, Matched: title, title.value, bio.value, bio

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio.value, bio

5. Search with stemming and exact match boosting:
Found 2 results (matches: run, running, runs)
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.750
  2. Score: 0.740
  3. Score: 0.732

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.366, Matched: content, title, title.value, content.value
  2. Score: 1.370, Matched: title, content.value, content, title.value
  3. Score: 0.375, Matched: 

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
✓ Inserted document: RpPMCfYbyWzE_6GY1gCkEoZc4zTsjRhK5szN0et-BAsoFbi8Q9VH5e66YQ0WIvOc-ZV7L19j0wdDmOjLo_K-jQ

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: aeVoVVmGTyxM2MUkkZTcq2sDFnfmppP1EZqB-w9nOKqPpvhV0eGQZ5cG9XzRJtw88jTta0KdJ4lMC8dt5nio3Q

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
✓ Inserted document with TTL: uzP1n3XYTiIdROXWFVd99o-vQa4VtlERjD296VUQO7IVS_LeQdz47ZPEgtzB0VUMjxvEtJfK2C5DOYCiW9Fpfw

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
✓ Edge cache script created: ZxJGVc39tL0kDymq9q8g5zkoYd0EiBZxW7qsoFVh19PRtl1pdFI4I6YTG69pK_LCr-3-pxMBsWOURjTdXkPsrA

Call 1: Cache miss (fetches from API)
Response time: 565ms
Result: {
  "records": [
    {
      "value": {
        "current": {
          "interval": 900,
          "temperature_2m": 7.0,
          "time": "2026-04-06T14:30"
        },
        "current_units": {
          "interval": "seconds",
          "temperature_2m": "\u00b0C",
          "time": "iso8601"
        },
        "elevation": 32.0,
        "generationtime_ms": 0.02312660217285156,
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
Response time: 41ms (13.9x faster!)
Result: {
  "records": [
    {
      "value": {
        "current": {
          "interval": 900,
          "temperature_2m": 7.0,
          "time": "2026-04-06T14:30"
        },
        "current_units": {
          "interval": "seconds",
          "temperature_2m": "\u00b0C",
          "time": "iso8601"
        },
        "elevation": 32.0,
        "generationtime_ms": 0.02312660217285156,
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

✅ Function saved: hLjABwsnLYCO9_wDpN8PKqEUugcHXnKrkEZ22amG5VhV5HugYbaxLKbEbP1ZPho-2M1NoYkaSTNB7DEVBq59mw
📊 Found 10 active users

📝 Example 2: Parameterized Function

✅ Function saved: SdXXLezaXntTEuVjfD9WyxGINwVIX2BbYABtvxyYqHrtqYHtSRdbqd5J60itGJLpt6D9fCZer2D-_iGXMx6AuA
📊 Found 10 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: g50yh6R0AKkOLlBES1mrH--PqeHwRwNBo08TKfEqf9c2vvBaNkQO2VaBJkVeLU4lzCAy74QdXKCrjX4j8__6EQ
📊 Statistics: 2 groups
   {'avg_score': 60.0, 'count': 5, 'status': 'active'}

   {'avg_score': 50.0, 'count': 5, 'status': 'inactive'}

📝 Example 4: Function Management

📋 Total functions: 18
🔍 Retrieved function: Get Active Users
✏️  Function updated
🗑️  Function deleted

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
   ⏱️  Duration: 38.2ms
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
   ⏱️  Duration: 39.1ms
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

✅ Function saved: jmYgiNTVtnc20eyyJPtfsLwP-PUxTsrpHxR1wO-uSO0xjENtcCJZhqCMvwdGT7Y95Wt0lk8WUQRLZg3ZL9ZSBA
📊 Found 2 product groups
   {'avg_price': 575.6666666666666, 'category': 'Electronics', 'count': 3}
   {'avg_price': 474.0, 'category': 'Furniture', 'count': 2}
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

✅ Inserted order: aROKs9P-OpUjd1DTkseJps9YYOo1B-P4UFY6sX7mMeUsKqRrtkvq9qURi5cMWHZpZkUH9uULU8y-NJ-kxA1XEQ
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

✅ Inserted order: hdUjqsHxNIGsVchgfCAs8MwYmr2n2pbqyZp-_iY_3XrgRn05wsYb2UmZ97RqibXEYnbVwLICIK6oQPGY50D9CA
✅ Cached order status
📊 Quick status lookup: {'value': '{"status":"processing","updated_at":"2026-04-06T14:33:06.474058+00:00"}'}

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
✓ Created SWR script: fetch_api_user_py (COqz9vemxVw0sjPiX4_OgLbKLu09rmzgPJYrjcYVN3DvayF9ATMu4lhMUCbpWvbtrbOVpP2jStk-tsrmT_IkHw)

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
✓ Created native SWR script: github_user_native (d_SKIvZFL02VoiYOfAX5Vx-PX1SsGCyPGSaxdX3OUx-JKAgQ-ySYRg2b-gKLW7CGslgzbtyaiNYViNRbWulSDQ)

First call (cache miss - will fetch from GitHub API):
  Response time: 122ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 20ms
  Speedup: 6.0x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (SC3WboCpe_gwIPosL80fFqJm9pDVFbCCgfn9FSyZqM4aeP-amfHOAXG1K9P6wSssP4XpFAVcYEKExQ7rCtDYkg)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (SkOzqQJ8S4wBSmKRjhVvb7AT3F8aoN9bNMN5In3CIua2WYbSdAPcFMdIYwVdtDJOISn56yOLzCEBv9UnwlT4vw)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (7CyEI4OZn8CH174GahxdvSRGVDV4vfAkO_GDX2KJZOgRKbSsiDP9UsakdqYz2gbXNlVcWBJHsUup-pExs4zKoQ)
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

1. **Enhanced Search Capabilities**: They enable similarity searches using vector embeddings, allowing for more intuitive retrieval of related items.
  
2. **Handling Unstructured Data**: They are well-suited for managing unstructured data, such as text and images, by transforming them into vector representations.

3. **Scalability**: Designed to handle large datasets efficiently, vector databases can scale horizontally to support growing amounts of data.

4. **Speed**: They provide fast querying capabilities, especially for nearest neighbor searches, due to optimized indexing and search algorithms.

5. **Integration with Machine Learning**: They facilitate seamless integration with machine learning models, allowing for real-time inference and analysis.

6. **Multi-Modal Data Support**: Vector databases can efficiently store and retrieve multiple types of data (text, images, audio) within the same framework.

7. **Flexibility**: They allow for complex queries that can combine various attributes and similarity metrics, making them versatile for diverse applications.

These advantages make vector databases particularly useful for AI/ML applications, recommendation systems, and natural language processing tasks.
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
   inactive: 3 users
   active: 7 users
⏱️  Execution time: 0ms

📝 Example 3: Average Score by Role

✅ Function saved
📊 Average score by role:
   {'avg_score': 70.0, 'count': 7, 'role': 'user'}
   {'avg_score': 20.0, 'count': 3, 'role': 'admin'}
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
   2. Natural Language Processing (AI)
   3. Vector Databases Explained (Database)
   4. Getting Started with ekoDB (Database)
   5. Database Design Principles (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   {'category': 'Database', 'count': 3}
   {'category': 'AI', 'count': 2}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: 9EGzf9YdCOVmLIqQkXmfFJaLCXNknlVWDoquoRQbrNZIFrXWKdMJ1boRO3yWcLHwltviIQd-A77lGsZ0UhZgDA

=== Sending Chat Message ===
Message ID: 7iFEwq-Ppg8XJPfpxoeNMohQB_fYio4ssTDxKnT3hDClap44W4thaLNCmdJw2p9m3vJP7ZksyCD57vfwcFxB7w

=== AI Response ===
Here are the available products and their prices:

1. **ekoDB Cloud** - $499
2. **ekoDB** - $99
3. **ekoDB Pro** - $299

If you have any more questions or need further information, feel free to ask!

Execution Time: 5234ms

=== Token Usage ===
Prompt tokens: 2772
Completion tokens: 70
Total tokens: 2842

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: O5H1er-891wQFDJjvvNEm3Rm66dEVkumyTn6UJzAmLEKH0RT_QFB4QHfA8ELpWjj3-EGN2opoALHE6i2B5No9g

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: High-performance database product
- **Price**: $99

If you need more information or have any other questions, feel free to ask!

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['content', 'chat_id', 'llm_model', 'updated_at', 'role', 'context_snippets', 'id', 'token_usage', 'created_at', 'llm_provider'])
Debug: First message role: {'value': 'assistant', 'type': 'String'}
=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of ekoDB is $99. If you have any further questions or need additional information, feel free to ask!

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: 6OOgl9nxJyaYS1rHbVDOR2HIK7TrZtGQSqxhWZ-KFpqW1JzxHtRTh72JO9CiSez-OkFmx3LM_2YMd32ZXjUCew
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
✓ Created session: OzY5hnL7NVu5jjlx3ACQkO5_HU8a-LKxt3yMAMfr0X-8Skuqq7tFJ7NQnF1G64SfFIGfeeVp1-NlrlY25vSdPQ

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: A high-performance database product
- **Price**: $99

If you need more information or other products, let me know!

✓ Message 2 sent
  Response: The price of ekoDB is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: X9Ou8SUOkBuZxBIvNU-azDY4Af5d4nwxLs76f4Z7zS7qsMi8Q8OdYK6tLo5_7bnEedx3s_Yws4KWrqcKfDY8tw
  Parent: OzY5hnL7NVu5jjlx3ACQkO5_HU8a-LKxt3yMAMfr0X-8Skuqq7tFJ7NQnF1G64SfFIGfeeVp1-NlrlY25vSdPQ

=== Listing Sessions ===
✓ Found 7 sessions
  Session 1: X9Ou8SUOkBuZxBIvNU-azDY4Af5d4nwxLs76f4Z7zS7qsMi8Q8OdYK6tLo5_7bnEedx3s_Yws4KWrqcKfDY8tw (Untitled)
  Session 2: OzY5hnL7NVu5jjlx3ACQkO5_HU8a-LKxt3yMAMfr0X-8Skuqq7tFJ7NQnF1G64SfFIGfeeVp1-NlrlY25vSdPQ (Untitled)
  Session 3: 6OOgl9nxJyaYS1rHbVDOR2HIK7TrZtGQSqxhWZ-KFpqW1JzxHtRTh72JO9CiSez-OkFmx3LM_2YMd32ZXjUCew (Untitled)
  Session 4: 9EGzf9YdCOVmLIqQkXmfFJaLCXNknlVWDoquoRQbrNZIFrXWKdMJ1boRO3yWcLHwltviIQd-A77lGsZ0UhZgDA (Untitled)
  Session 5: 3KoXYwwCCITYp_flL5A35UMDsfWBl2GpU2AzgdzxK4Ywi5llJ7V_6jJ2Usemo-334IudDV6eeot_tyLO6j8Grg (Untitled)
  Session 6: cQJxP86LSAdtP34i1wOt-MFh0JOvbneSKhZKwa335Padx_WMjW2BPok1-OdJhBYDBps2uE6lNxJcKarhTtmgtQ (Untitled)
  Session 7: ykweq9aT3ctDaaqDgEjReoyIRQ1dhjQNHffFgYGiE3kvyd940dzT2H1H6vbzPBdbe6rREtQ0tSrgTG49AyryPw (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: X9Ou8SUOkBuZxBIvNU-azDY4Af5d4nwxLs76f4Z7zS7qsMi8Q8OdYK6tLo5_7bnEedx3s_Yws4KWrqcKfDY8tw

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Dict Creation ===
✓ Created record with plain dict: {'id': 'bexoJ2FgU4xIWD3i7C5zTdbfefWKB3b8tntc6FcEAiWN3FjDYc0V_fC4mj7VFpPqfy4nRoQhIvqX8C2MWxQ-lw'}

=== Upsert Operation ===
✓ Upsert (update existing record): bexoJ2FgU4xIWD3i7C5zTdbfefWKB3b8tntc6FcEAiWN3FjDYc0V_fC4mj7VFpPqfy4nRoQhIvqX8C2MWxQ-lw
✓ Inserted second record: XAg3N9kabZTAqwR0KIHWKglHMg8EvgxgZi1oQmcdjDHydSnU1c-1ejC82FDl2TzcIQ9Pd664Vm66FF3BDsMx5w
✓ Upsert (update second record): XAg3N9kabZTAqwR0KIHWKglHMg8EvgxgZi1oQmcdjDHydSnU1c-1ejC82FDl2TzcIQ9Pd664Vm66FF3BDsMx5w

=== Find One Operation ===
✓ Found user by email: {'name': {'value': 'Alice Johnson', 'type': 'String'}, 'email': {'type': 'String', 'value': 'alice.j@newdomain.com'}, 'id': 'bexoJ2FgU4xIWD3i7C5zTdbfefWKB3b8tntc6FcEAiWN3FjDYc0V_fC4mj7VFpPqfy4nRoQhIvqX8C2MWxQ-lw', 'age': {'value': 29, 'type': 'Integer'}, 'active': {'value': True, 'type': 'Boolean'}}
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
   Inserted with ripple: {'id': '2yQqGIWmAhk3cQvPC1OhQOnH1qhaaagR7IcsNurhbRxh5hPdznrCNZ1iFBlOu_fh8hXwl6ydYxf2McmkivFRoQ'}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {'id': 'IAto3mzymtncGj37Kfl9XmXN9xeR9oZw-gcWuycH-6y4PFiCmqHECdOKsY23siyCalvrpWZxCyOfeehpTyKIfw'}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {'id': '2yQqGIWmAhk3cQvPC1OhQOnH1qhaaagR7IcsNurhbRxh5hPdznrCNZ1iFBlOu_fh8hXwl6ydYxf2McmkivFRoQ', 'name': {'type': 'String', 'value': 'Product 1'}, 'price': {'type': 'Integer', 'value': 150}}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {'id': '2yQqGIWmAhk3cQvPC1OhQOnH1qhaaagR7IcsNurhbRxh5hPdznrCNZ1iFBlOu_fh8hXwl6ydYxf2McmkivFRoQ', 'price': {'type': 'Integer', 'value': 500}, 'name': {'value': 'Upsert Product', 'type': 'String'}}

✅ All bypass_ripple operations completed successfully!
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields (id, name, email only)
  Found 3 active users
  Fields returned: ['email', 'id', 'name']
  First user: {'value': 'Dave Brown', 'type': 'String'} <{'type': 'String', 'value': 'dave@example.com'}>

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ['id', 'created_at', 'avatar_url', 'user_role', 'status', 'age', 'name', 'email', 'bio']

Example 3: Complex query with projection (active users, ages 18-65)
  Found 3 active users (ages 18-65)
    - {'type': 'String', 'value': 'Dave Brown'} (age {'type': 'Integer', 'value': 45})
    - {'type': 'String', 'value': 'Alice Johnson'} (age {'type': 'Integer', 'value': 30})
    - {'value': 'Bob Smith', 'type': 'String'} (age {'value': 25, 'type': 'Integer'})

Example 4: Query inactive users with profile fields
  Found 1 inactive users
    - {'type': 'String', 'value': 'Carol White'}: {'type': 'String', 'value': 'Manager'}

Example 5: Compare full vs projected data
  Full query:
    - 12 fields per record
    - Fields: ['name', 'id', 'user_role', 'status', 'email', 'api_key', 'bio', 'avatar_url', 'password', 'created_at', 'age', 'secret_token']
  Projected query:
    - 3 fields per record
    - Fields: ['name', 'email', 'id']
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
Created user function with ID: sKRPnwBd58-mHBjpR3tYjenuouEKIPtEbFOvoXmGZ904b8WDlH3TpP1LQzKpuazdtOlXoQewq8patynhXYMKYg

=== Get User Function ===
Retrieved: get_active_users_py - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 25 user functions:
  - get_active_users_paginated: Get Active Users (Paginated)
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_user: Fetch user by code
  - get_high_scoring_active_users: Get High Scoring Active Users
  - validate_user: Check if user exists
  - get_active_users_py: Get Active Users
  - get_active_users_paginated: Get Active Users (Paginated)
  - get_verified_user: Get verified and validated user
  - get_active_users_updated: Get Active Users (Updated)
  - fetch_slim_user: Validate and slim down user
  - fetch_user: Fetch user by code
  - get_active_users_updated: Get Active Users (Updated)
  - validate_user: Check if user exists
  - fetch_slim_user: Validate and slim down user
  - get_active_users: Get Active Users (Updated)
  - get_users_by_status: Get Users By Status
  - get_users_by_status: Get Users By Status
  - swr_user: SWR pattern for user data (KV-based)
  - get_user_wrapper: Wrapper that calls fetch_user
  - get_active_users: Get Active Users (Updated)
  - swr_user: SWR pattern for user data (KV-based)
  - get_verified_user: Get verified and validated user
  - get_user_wrapper: Wrapper that calls fetch_user
  - cache_api_call_py: Cache External API Call

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
