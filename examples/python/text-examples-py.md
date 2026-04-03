make test-examples-python
🧪 [36mRunning Python examples (direct HTTP/WebSocket)...[0m
=== Simple CRUD Operations (Direct HTTP) ===

✓ Authentication successful

=== Insert Document ===
Inserted: {'id': 'DJo69H27J0nCROhUDHl_2cJ8NTFyuWubZ9GF7tKSgEIEVYs7F-vJR4_Tia-yV2i97jNtdbBB4Hcy4eJaWsn24A'}

=== Find by ID ===
Found: {'name': {'value': 'Test Record', 'type': 'String'}, 'value': {'value': 42, 'type': 'Integer'}, 'id': 'DJo69H27J0nCROhUDHl_2cJ8NTFyuWubZ9GF7tKSgEIEVYs7F-vJR4_Tia-yV2i97jNtdbBB4Hcy4eJaWsn24A', 'active': {'value': True, 'type': 'Boolean'}}

=== Find with Query ===
Found documents: [{'value': {'value': 42, 'type': 'Integer'}, 'name': {'type': 'String', 'value': 'Test Record'}, 'active': {'value': True, 'type': 'Boolean'}, 'id': 'DJo69H27J0nCROhUDHl_2cJ8NTFyuWubZ9GF7tKSgEIEVYs7F-vJR4_Tia-yV2i97jNtdbBB4Hcy4eJaWsn24A'}]

=== Update Document ===
Updated: {'name': {'value': 'Updated Record', 'type': 'String'}, 'value': {'type': 'Integer', 'value': 100}, 'id': 'DJo69H27J0nCROhUDHl_2cJ8NTFyuWubZ9GF7tKSgEIEVYs7F-vJR4_Tia-yV2i97jNtdbBB4Hcy4eJaWsn24A', 'active': {'value': True, 'type': 'Boolean'}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: zQECmSh2ns8ZLfEQScqatfFFhkooJ39oZ1dU-amJnKZTKXj0Mm24Mpp9ICJ0TTqm9HAmZnkQWnmC_cfXHj99Lg

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "messageId": "512558460",
  "payload": {
    "data": [
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "f1ep3MgsAlnio7R9S6GnmVSrYAorrXrI27q9RdGsoyd30671cQvxiWd5TDUy6NRUMVNF5-2ErJVEeT9xTtvSOw",
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
        "id": "zQECmSh2ns8ZLfEQScqatfFFhkooJ39oZ1dU-amJnKZTKXj0Mm24Mpp9ICJ0TTqm9HAmZnkQWnmC_cfXHj99Lg",
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
🚀 ekoDB Scripts Example (Python/HTTP)

✓ Authentication successful

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: bQkeChV2KmM7BVZtZEzhj0YkRNfT75S1w9shI39cfF8NJPsTC2W9HtQOMNuHw1d5NCQ7X-2XUqYH0-QoGTQhmw
📊 Found 15 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: hQaKk8ebAwFvg7OtN5dZt0pIojg7UjNv8XiWrcGFo7tP7TVfsW9UhSmGj-buUdGmJa-U7XbUv9F5eGYWi0AFsA
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: mPqrvot8T-VmheDzgRwWYGqeETDTGmWmLMhOznS6aZBLeG-sP4p5CQDowqF78AdtOr2usMNBBjIzBBS5PtuI5w
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {'status': 'inactive', 'avg_score': 50.0, 'count': 15, 'max_score': 90}
   {'max_score': 100, 'status': 'active', 'count': 15, 'avg_score': 60.0}

📝 Example 4: Function Management

📋 Total scripts: 15
🔍 Retrieved script: Get Active Users
✏️  Script updated
🗑️  Script deleted

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
cache:product:1: {'name': 'Product 1', 'price': 29.99}
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
Collection created with first record: MAG3gThmGtXRZUKspk-MaeMVI_wLI-AOrNLkjxhWU39blxMH1dxv5mN_CgmMwI5sXhXDWJykqUEcbqor-tTSLQ

=== List Collections ===
Total collections: 13
Sample collections: ['demo_collection', 'chat_messages__ek0_testing', 'products', 'ttl_cache', 'users']

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
Created Alice: $1000 - ID: zwzXOx3JfKhjxAnIiVKPTjr4vIwWa5LfABvMt0aOmWc83h9anpDGr7hZMck_36TAbcUc3b7opnbNpmAZ3Mqayw
Created Bob: $500 - ID: HvTa16Azy7ncxaSlSb-hBQAIENSKgo5M6Hx7nEk1hq8jRdVEviahpW78gLSncdj190cVm5uXRI6LRVTp9j5WzA

=== Example 1: Begin Transaction ===
Transaction ID: 364a81bb-65c6-4239-9f6d-9883648e3452

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
New transaction: cb4c99c3-b035-46e3-813d-b194d1d254a8
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'value': 700, 'type': 'Integer'}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: KfsvHZURoE1E11CRgZBfLMEi__jOuLsT9kRUAkTlHzS7ViAFiR6HNRaa7ZmWbVTph-7o_vLv8t3eAdhSBeFYeg
Created Bob: $500 - ID: 5FwV21mfn0DjHuUn8TzlKTqAlKfIxImdVfNp7AaCFfrNrg6OGNx58EwH7COMT3VlkphXasKHjyytwZ_-LMUVtg

=== Example 1: Begin Transaction ===
Transaction ID: 9986d504-b80f-4b35-a7e0-602ff6688c35

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
Created Alice: $1000 - ID: VKqSLgeNoA77WkCdOYzqwGYMKxIsaZXpD2U6pIR1ZVD529U8yZOM6K5QQIJMBeuR45tRvW1Hj5J8DAboMDkFyw
Created Bob: $500 - ID: e1LjdLsyoQ_dA9qj6Ye1IjOgXw4k5qEMNdTSbtYXO7Erj8Og90_qzen3Wx3jJ3Oz-3hlqTbD1dAap6VPptaECg

=== Example 1: Begin Transaction ===
Transaction ID: 79ac30b8-491b-4091-b835-d34c15a70d7f

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
New transaction: f5e2f320-53fb-4ace-996e-fefa6fe39d1a
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'type': 'Integer', 'value': 700}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 64l19L7oDDncoQbYt28AYtrSguR8frhr-Dlflc3JPt6UCrI8flW-9zw9uLxC88UHHG0-GAkxy-p-v9MG12NWZg
Created Bob: $500 - ID: E2ieWt6XWM1DPnyPYu6Zj_LvdP2cbZg22JwxpMUwlTMkdh742X_6A3q0qprMd86wap-SQGmzG4DDCcdAIa-mTw

=== Example 1: Begin Transaction ===
Transaction ID: 756e1a1f-2a76-48e2-b3c1-336c1586d7b6

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


🔄 Switching to DURABLE mode...
   ✓ Config updated: durable_operations=true


============================================================
🔥 TEST 3: Durable Mode (durable=true)
============================================================

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: QH_yKI60FSh-vnaeB3PPwMIe0Wq2WF9CkECHqjDV5bOMtYXWkR51D6PduEKI53Y91SoXWyxywmdKThGPdEkAnA
Created Bob: $500 - ID: jPk-MpURkHZpdnZuRPC36T25L9IHuAallS2MENAnC21_EaHV1C0QSvosEdd9lj30KqMmEVCFKZ9XUKpJ8SLPbw

=== Example 1: Begin Transaction ===
Transaction ID: 79dad448-342b-414b-8891-0defa8454d5a

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
New transaction: 94433bb0-6247-4c9d-b5be-3198f4ef41e4
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'type': 'Integer', 'value': 700}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: xA3NqiXZJvnvkdZVjLRB9MLPKRmsQLK0XGPbm2keEIU7a6XTfGc57ofk_4PubjQ34pveMHFbNGpNH4Kyb-5aqw
Created Bob: $500 - ID: HQPfuFJVSR4B4zfUHCRqFWzZEhFWoLnEB59KlhX4KBN-nLz2Yyqw0U6Jhn9yFIT6t76iCtJdVj_aoY2QrgNGVA

=== Example 1: Begin Transaction ===
Transaction ID: 8b2aff04-5f3c-4b1b-9eef-07ba5fd45b70

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: ${'value': 800, 'type': 'Integer'}
Bob: ${'value': 700, 'type': 'Integer'}

=== Cleanup ===
✓ Deleted test accounts


🔄 Restoring original configuration...
   ✓ Config restored: durable_operations=True


============================================================
✅ ALL TESTS PASSED - Transactions successful
============================================================

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
   ✅ Script saved: KBnKQF2-Pz_XMRPbqOJGHJ9Q1HrOgBR2ouqUSxnixn11dm9VMSVZkASLPRabss83FymDo5i5-puZgWA1D8GdwQ

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: 51pRYTP7jtqUYO7sy2VoMFrns9QTtGEZUsv9r5Gx9ZapT-TIdGt01GSdFbdgt6moJft-uXgfTlegR6kVLfekTw
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}
   📋 Email: {'type': 'String', 'value': 'alice@example.com'}
   📋 Status: {'type': 'String', 'value': 'pending'}
   📋 Credits: {'value': 0, 'type': 'Integer'}

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: Hf_U24PP9TwbhGZQsNBVrevLLWOWar08PcaNU3rgkbZcZgfl5K6e24eVVq0lQ6AXR7yLFWFLOotJtgDepWIcig

2️⃣ Calling Script (Query + Update + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Status updated to: {'value': 'active', 'type': 'String'}
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}

============================================================
📝 Script 3: Query + Update Credits + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: QQn8CUKspO-TgSefyp7YlwvEXqEZvtBQ5vFp9fvJK7GC2KcuI0E6ktRcr_rzx0HbZuWUqmxtTh22Uy3xk-XU7Q

2️⃣ Calling Script (Query + Update Credits + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {'type': 'Integer', 'value': 0}
   📋 Status: {'type': 'String', 'value': 'active'}
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}

============================================================
📝 Script 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: 9A-3hooQsi7wB20YLvXBwXNkjtcXFgE27-BoUj0vd8KtwKbFr_LAWFSzBB0pPQad28O6fuE7rN9yG0f6Rs0bjA

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: KBnKQF2-Pz_XMRPbqOJG...
   ✅ Deleted script: Hf_U24PP9TwbhGZQsNBV...
   ✅ Deleted script: QQn8CUKspO-TgSefyp7Y...
   ✅ Deleted script: 9A-3hooQsi7wB20YLvXB...
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
  Output: Document ID = f97PTyqThT12FdOhNujb3Tt_n2wJSFKopxxNNIW1UCw8JxxZNDuApYhIuWdBGFZP-btTeWVTdropgqKjakdxeg
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: find_by_id(f97PTyqThT12FdOhNujb3Tt_n2wJSFKopxxNNIW1UCw8JxxZNDuApYhIuWdBGFZP-btTeWVTdropgqKjakdxeg)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: find_by_id(f97PTyqThT12FdOhNujb3Tt_n2wJSFKopxxNNIW1UCw8JxxZNDuApYhIuWdBGFZP-btTeWVTdropgqKjakdxeg)
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
  Output: Document ID = n2WV5xO3qHP2qHYNsTUejiJqL_ry-GWtfQLpfYuKODQQSnwdrqBs3jQRtGcNQDtfHpd-Rw9lRLENKWGX0pCmgw
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: find_by_id(n2WV5xO3qHP2qHYNsTUejiJqL_ry-GWtfQLpfYuKODQQSnwdrqBs3jQRtGcNQDtfHpd-Rw9lRLENKWGX0pCmgw)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: find_by_id(n2WV5xO3qHP2qHYNsTUejiJqL_ry-GWtfQLpfYuKODQQSnwdrqBs3jQRtGcNQDtfHpd-Rw9lRLENKWGX0pCmgw)
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
=== Running crud_scripts.py ===[0m
[32m✓ crud_scripts.py completed successfully[0m
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
    Finished `release` profile [optimized] target(s) in 0.14s
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
Inserted: {'id': 'Bf6_NGIUbjxPQv8r9nQ7eKUHP112Fu3MVLjLMR9VSB__d4hnKdikiH88BBHKTQEeQPz7JOnVecBLRBWi_9srLQ'}

=== Find by ID ===
Found: {'data': {'type': 'String', 'value': 'aGVsbG8gd29ybGQ='}, 'id': 'Bf6_NGIUbjxPQv8r9nQ7eKUHP112Fu3MVLjLMR9VSB__d4hnKdikiH88BBHKTQEeQPz7JOnVecBLRBWi_9srLQ', 'categories': {'value': ['electronics', 'computers'], 'type': 'Array'}, 'user_id': {'type': 'String', 'value': '550e8400-e29b-41d4-a716-446655440000'}, 'metadata': {'value': {'key': 'value', 'nested': {'deep': True}}, 'type': 'Object'}, 'value': {'value': 42, 'type': 'Integer'}, 'tags': {'value': ['tag1', 'tag2', 'tag3'], 'type': 'Array'}, 'name': {'type': 'String', 'value': 'Test Record'}, 'active': {'value': True, 'type': 'Boolean'}, 'embedding': {'type': 'Array', 'value': [0.1, 0.2, 0.3, 0.4, 0.5]}, 'created_at': {'value': '2026-04-03T10:30:57.291827', 'type': 'String'}, 'price': {'value': 99.99, 'type': 'Float'}}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): True
  price (Decimal): 99.99
  created_at (DateTime): 2026-04-03 10:30:57.291827
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): ['tag1', 'tag2', 'tag3']
  metadata (Object): {'key': 'value', 'nested': {'deep': True}}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): ['electronics', 'computers']
  data (Bytes): 11 bytes
Plain record: {'data': 'aGVsbG8gd29ybGQ=', 'id': 'Bf6_NGIUbjxPQv8r9nQ7eKUHP112Fu3MVLjLMR9VSB__d4hnKdikiH88BBHKTQEeQPz7JOnVecBLRBWi_9srLQ', 'categories': ['electronics', 'computers'], 'user_id': '550e8400-e29b-41d4-a716-446655440000', 'metadata': {'key': 'value', 'nested': {'deep': True}}, 'value': 42, 'tags': ['tag1', 'tag2', 'tag3'], 'name': 'Test Record', 'active': True, 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5], 'created_at': '2026-04-03T10:30:57.291827', 'price': 99.99}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'active': {'value': True, 'type': 'Boolean'}, 'id': 'Bf6_NGIUbjxPQv8r9nQ7eKUHP112Fu3MVLjLMR9VSB__d4hnKdikiH88BBHKTQEeQPz7JOnVecBLRBWi_9srLQ', 'data': {'value': 'aGVsbG8gd29ybGQ=', 'type': 'String'}, 'categories': {'value': ['electronics', 'computers'], 'type': 'Array'}, 'value': {'value': 100, 'type': 'Integer'}, 'name': {'value': 'Updated Record', 'type': 'String'}, 'price': {'type': 'Float', 'value': 99.99}, 'user_id': {'value': '550e8400-e29b-41d4-a716-446655440000', 'type': 'String'}, 'metadata': {'type': 'Object', 'value': {'key': 'value', 'nested': {'deep': True}}}, 'tags': {'value': ['tag1', 'tag2', 'tag3'], 'type': 'Array'}, 'embedding': {'type': 'Array', 'value': [0.1, 0.2, 0.3, 0.4, 0.5]}, 'created_at': {'type': 'String', 'value': '2026-04-03T10:30:57.291827'}}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: o9NFKSzUpZMafkbR0AR_l0EJ6i_DDk8riUBwLHqPN7VXznTCQa2kYxlwNiL_-SHUew1MrpAfr-73y0ZDbyJC2Q

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
Collection created with first record: "CoRVONY9ms8FOEnk-i7DlsU45pT0-rqXQnJYelkZkJ84WkmzohANFuTbdGEva9zA29h2An-MIOwClcA6EAIurg"

=== List Collections ===
Total collections: 12
Sample collections: ['chat_messages__ek0_testing', 'products', 'ttl_cache', 'test_collection', 'scripts__ek0_testing']

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
  cache:product:2: {'name': 'Product 2', 'price': 39.99}
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
Created Alice: $1000 - ID: dVh3mMW8FDaH41A6_SUhVOQ8LB0hQep7Am181sAsuBub3UjxZmmliJHgh63PdgJtJpmwDPjN4Ze5yALlAQWd3g
Created Bob: $500 - ID: G-WlMdrMjU5IBs8j_cAfLDnHV4heEwmg6WZS4blw6vKT6mhfg5_bhfr6IddB96FX9aylalGlCWm7appAiremJA

=== Example 1: Begin Transaction ===
Transaction ID: 91f62744-08d2-48cb-9c04-2bb5a1f04e07

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 6ac908dd-6839-44cd-8461-c8d8a17bc43d
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
  1. Score: 25.740, Matched: name.value, name, email, email.value
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title, bio, title.value, bio.value
  2. Score: 26.400, Matched: title, bio.value, bio, title.value
  3. Score: 26.400, Matched: bio, bio.value, title, title.value
  4. Score: 26.400, Matched: title.value, title, bio.value, bio

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio, bio.value, title.value, title
  2. Score: 39.600, Matched: title, bio.value, bio, title.value
  3. Score: 39.600, Matched: bio.value, title, title.value, bio
  4. Score: 39.600, Matched: title, bio, bio.value, title.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 2 results (matches: run, running, runs)
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio.value, bio

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.759
  2. Score: 0.740
  3. Score: 0.727

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.380, Matched: title, content.value, content, title.value
  2. Score: 1.363, Matched: content.value, content, title.value, title
  3. Score: 0.370, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 13.200, Matched: title.value, title

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
✓ Inserted document: 7mySnTFOyXLhKW2D-6jBf19IcGKuaQYglSz2j-hRf4E3lgyOTdDactWpvNTPKlU9jy28qfjSOUMrIbBIFFtTTA

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: Vra_qlDlmMd3kYz24_N8P3Ng9JMtiImJkZSRYj5kSFljcuj-oBSaDTTomx5E18DFWlZKw5U_e2KcHh38534Zeg

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
✓ Inserted document with TTL: AM0EoC8I8xyGcHvQ93yMKLAYx_z8jm3NY9p1EYn1CU1ialzQeZVLkfc2T_THo-jSJG3W3yR0_H_Yv78ZQ3DLoQ

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
✓ Edge cache script created: qNj1rHoDJiiX_Fk1SgIAF_3nTZrE1kP6QBIY_hG6jDujl7ImEMWicJqh1ROOWOJDRqjfCULHwF0vrWHzd0Cf0g

Call 1: Cache miss (fetches from API)
Response time: 559ms
Result: {
  "records": [
    {
      "value": {
        "current": {
          "interval": 900,
          "temperature_2m": 7.1,
          "time": "2026-04-03T14:30"
        },
        "current_units": {
          "interval": "seconds",
          "temperature_2m": "\u00b0C",
          "time": "iso8601"
        },
        "elevation": 32.0,
        "generationtime_ms": 0.02014636993408203,
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
Response time: 41ms (13.5x faster!)
Result: {
  "records": [
    {
      "value": {
        "current": {
          "interval": 900,
          "temperature_2m": 7.1,
          "time": "2026-04-03T14:30"
        },
        "current_units": {
          "interval": "seconds",
          "temperature_2m": "\u00b0C",
          "time": "iso8601"
        },
        "elevation": 32.0,
        "generationtime_ms": 0.02014636993408203,
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

🚀 ekoDB Scripts Example (Python)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: OZZasZEhqQsZj-6k8whg6QY_hlrZNg9y5NSx4Uop3qxlqYxXS7R8dyAHbj6wneMXNWUtOlRo07qk_23QtnH_1g
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved: f9H9zbhAkmKHxbedvR2g9MeaKcxnHZGjTGuY1U0Kngil0l7P9VD0ICYnknPt_FzKUyHn2xcR4qQ7OeOAzgz5-g
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: oB-Siola58suAR7qP2L-3TFLrqmMjjdDAITbnHcgSIPQ7Evc2F6wnJTJ-8cbHBeX83gH9D7ld20L5Zs5JHmqiw
📊 Statistics: 2 groups
   {'avg_score': 60.0, 'count': 5, 'status': 'active'}

   {'avg_score': 50.0, 'count': 5, 'status': 'inactive'}

📝 Example 4: Script Management

📋 Total scripts: 18
🔍 Retrieved script: Get Active Users
✏️  Script updated
🗑️  Script deleted

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
   ⏱️  Duration: 38.4ms
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
🚀 ekoDB Python Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved: Ov6JkDqfjIm13GYT5Eok2SmujLwBVkHCLDyzrE_vnuZ9exzRXXE9nwJ1yDF63c26oCTJYfsXUkNUQHUnhGwTaw
📊 Found 2 product groups
   {'avg_price': 575.6666666666666, 'category': 'Electronics', 'count': 3}
   {'avg_price': 474.0, 'category': 'Furniture', 'count': 2}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved
📊 Found 2 categories
   {'category': 'Electronics', 'count': 3}
   {'category': 'Furniture', 'count': 2}
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
🚀 ekoDB Python KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: Q4qInHL0IK0JVu9pdOA-occQCdN72iJqZ_3Gmf9B7oMVtDeW-5ujnAn1GlJ5a0Lumpnzz9CYeBymYRfCcWXaaw
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

✅ Inserted order: 8N_Tpz0oQAcb-qI4uU5VFml3Ge-nxE8S_jsi7BIObORRXREQEzsSiUey5bNYquEvaAsRJuDZV53mvUZiyCffyQ
✅ Cached order status
📊 Quick status lookup: {'value': '{"status":"processing","updated_at":"2026-04-03T14:31:00.087192+00:00"}'}

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
✓ Created SWR script: fetch_api_user_py (hHe0leeHjgNuGH6g7RjN4toLhngzBRsNCRvcV8rNJ1hN7Y7QGEWdxFmIlYMtlp87qQies5yj9OlfojSyMtWT4A)

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
Response time: 40ms (served from cache)
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
✓ Created native SWR script: github_user_native (KpuwV5hR-CgYrsXyD3EEECumMcNwmNfNYXuQRgqwDx4Uda-KZPL9aSiEsnTDWFatO-d7z2U1BczgflBfcqE1MQ)

First call (cache miss - will fetch from GitHub API):
  Response time: 129ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 21ms
  Speedup: 6.1x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (NpHFAN9JBtfShbClaAX-c4nOc6n4Lr_4z-PQiXV3cWNovLF4o2NcxFvcjhxh2TRwwwNY2OEgw5mxcqDvPw-QZw)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (VPzkSap43eQbxmbwvd3093BJ4VzisMj05KNusjzM76qkhNCdei1_2CEswTTuluNlAelf0Kr9OtgS7WdFSlmR-A)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (CUC-4dnFKhy7EVSdyUqxLhF6EQomdL_ERxZXS0_RnYufQo8wsZ61PwO24A4Byx7pXWpLLHCa__fcATDBGJS9rA)
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
🚀 ekoDB Python Advanced Scripts Example

📋 Setting up test data...
✅ Created 8 products

📝 Example 1: List All Products

✅ Script saved
📊 Found 8 products
⏱️  Execution time: 0ms

📝 Example 2: Group Products by Category

✅ Script saved
📊 Category breakdown:
   {'avg_price': 365.6666666666667, 'category': 'Furniture', 'count': 3}
   {'avg_price': 367.0, 'category': 'Electronics', 'count': 5}
⏱️  Execution time: 0ms

📝 Example 3: Count Total Products

✅ Script saved
📊 Total products: [{'total': 8}]
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced script examples finished!
🚀 ekoDB Python AI Scripts Example

📋 Setting up test data...
✅ Created 2 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
🤖 AI Response:
   Vector databases offer several benefits:

1. **Efficient Similarity Search**: They excel at finding similar items in high-dimensional spaces, making them ideal for tasks like image, video, and text retrieval.

2. **Scalability**: Designed to handle large datasets efficiently, allowing for rapid querying even as data grows.

3. **Support for ML Models**: They are optimized for use with machine learning models, especially for embedding and feature extraction applications.

4. **Real-Time Performance**: Vector databases provide real-time capabilities for applications requiring immediate results, such as recommendation systems.

5. **Flexible Data Types**: They accommodate various data types, including text, images, and audio, in a unified framework.

6. **Advanced Indexing Techniques**: Utilizing algorithms like Approximate Nearest Neighbor (ANN) for faster search times compared to traditional databases.

7. **Ease of Integration**: Often come with APIs that integrate seamlessly with existing applications and workflows.

These features make vector databases appealing for AI-driven applications and data-intensive operations.
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
🚀 ekoDB Python CRUD Scripts Example

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
📊 Average score by role:
   {'avg_score': 20.0, 'count': 3, 'role': 'admin'}
   {'avg_score': 70.0, 'count': 7, 'role': 'user'}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD script examples finished!
🚀 ekoDB Python Search Scripts Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Script saved
📊 Found 5 documents
   1. Database Design Principles (Database)
   2. Getting Started with ekoDB (Database)
   3. Vector Databases Explained (Database)
   4. Introduction to Machine Learning (AI)
   5. Natural Language Processing (AI)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Script saved
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
✓ Created session: wfABG2pu6JA9StMRL4p3vfEt3VSkQjhx8ZQqiXaHzazDalnKBMVsf8_-a8xEJLSi3PpwZt3sVMzie1vAinUA6A

=== Sending Chat Message ===
Message ID: gNx2PuEPCLnhaSJrsoYAcOUOYthcUMp0xlvwGu1fbeczrEC-TVE4m0FNWdS8grzxA8Z6dyov-UPsf4PEKFUssg

=== AI Response ===
Here are the available products and their prices:

1. **ekoDB Pro**
   - **Price:** $299
   - **Description:** Enterprise edition product with advanced features

2. **ekoDB**
   - **Price:** $99
   - **Description:** A high-performance database product with AI capabilities

3. **ekoDB Cloud**
   - **Price:** $499
   - **Description:** Fully managed cloud database service product

Execution Time: 2568ms

=== Token Usage ===
Prompt tokens: 1298
Completion tokens: 95
Total tokens: 1393

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: tGGNmnGiFX4b2bdkljRfH8oLAQdJO1_kC23JLHr1mZuaW1udbqfsijepvd_dngWajcZIRd_TnbDKECWxO1fZ2Q

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: High-performance database product
- **Price**: $99

If you have any more questions or need further information, feel free to ask!

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['context_snippets', 'updated_at', 'llm_provider', 'role', 'content', 'created_at', 'llm_model', 'chat_id', 'token_usage', 'id'])
Debug: First message role: {'value': 'assistant', 'type': 'String'}
=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of ekoDB is $99. If you have any more questions or need further assistance, feel free to ask!

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: rhjy7d4dLnGAML7eqyrN7O3KOusPh8Rb_cdUHeNgVg6xcNp3TDeQHkXFKp8b5D4t42mv7Shjn4pZGlw4z1pLjQ
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
✓ Created session: Q-XooHMM3eZ6M7nOW9eTQ8OuZ5cEvNVKiDwYB5jtRvATiEsC5RuJRw63dhCpsiiXqZo4ZevBfgAltd11elm_aw

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: A high-performance database product
- **Price**: $99

If you need more details or additional products, feel free to ask!

✓ Message 2 sent
  Response: The price of ekoDB is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: sb4FesmdIcXm1L6eaGXh48HiNTc9M9dsOvH_vOUmSv6ipRvcLhYU6otEtCXueHfw_F3F0J6FLwEFPONPpKgzlg
  Parent: Q-XooHMM3eZ6M7nOW9eTQ8OuZ5cEvNVKiDwYB5jtRvATiEsC5RuJRw63dhCpsiiXqZo4ZevBfgAltd11elm_aw

=== Listing Sessions ===
✓ Found 7 sessions
  Session 1: sb4FesmdIcXm1L6eaGXh48HiNTc9M9dsOvH_vOUmSv6ipRvcLhYU6otEtCXueHfw_F3F0J6FLwEFPONPpKgzlg (Untitled)
  Session 2: Q-XooHMM3eZ6M7nOW9eTQ8OuZ5cEvNVKiDwYB5jtRvATiEsC5RuJRw63dhCpsiiXqZo4ZevBfgAltd11elm_aw (Untitled)
  Session 3: rhjy7d4dLnGAML7eqyrN7O3KOusPh8Rb_cdUHeNgVg6xcNp3TDeQHkXFKp8b5D4t42mv7Shjn4pZGlw4z1pLjQ (Untitled)
  Session 4: wfABG2pu6JA9StMRL4p3vfEt3VSkQjhx8ZQqiXaHzazDalnKBMVsf8_-a8xEJLSi3PpwZt3sVMzie1vAinUA6A (Untitled)
  Session 5: vCYe82Jg-pOeZZIHObPhq_7GmYPvpDBTrTVkCnR0TIAzSQ1PijqVblarLs6OvWmTuSDzO8eyi8HYbAwcLiYcyw (Untitled)
  Session 6: iaBlsAKJ7QB_nFVEf5jdEb-WhsDQYfsVz6ukmMZS1btgaQmVPlZzEBLnbPkBTg0U0ehNKYADpoXccf-fxuPFKg (Untitled)
  Session 7: 07fRg7thu1zdanTV9Y1iGdPQKtd_ydNh6iIWtBx7NdEteHa5RSoQr6F8ws67b1vQXslB27kZ7iebo7IfcIQn5Q (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: sb4FesmdIcXm1L6eaGXh48HiNTc9M9dsOvH_vOUmSv6ipRvcLhYU6otEtCXueHfw_F3F0J6FLwEFPONPpKgzlg

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Dict Creation ===
✓ Created record with plain dict: {'id': 'R9f7XW050HYXhz-76_Iu9-PJVVhCiW0YS2z226qolb_aiU4H0dYK3KxahU9AKaO46y0SkR8BIUaeYMeDb7GZ7Q'}

=== Upsert Operation ===
✓ Upsert (update existing record): R9f7XW050HYXhz-76_Iu9-PJVVhCiW0YS2z226qolb_aiU4H0dYK3KxahU9AKaO46y0SkR8BIUaeYMeDb7GZ7Q
✓ Inserted second record: 9wys_aF7g3Z-rUxP_1Lsrn955qZhaqeuTFoOMcAh7HkVZklHDHxSt9lnyOzpN9_v2hzD4O98s3U7At7UGzxVhg
✓ Upsert (update second record): 9wys_aF7g3Z-rUxP_1Lsrn955qZhaqeuTFoOMcAh7HkVZklHDHxSt9lnyOzpN9_v2hzD4O98s3U7At7UGzxVhg

=== Find One Operation ===
✓ Found user by email: {'id': 'R9f7XW050HYXhz-76_Iu9-PJVVhCiW0YS2z226qolb_aiU4H0dYK3KxahU9AKaO46y0SkR8BIUaeYMeDb7GZ7Q', 'name': {'value': 'Alice Johnson', 'type': 'String'}, 'email': {'type': 'String', 'value': 'alice.j@newdomain.com'}, 'age': {'value': 29, 'type': 'Integer'}, 'active': {'value': True, 'type': 'Boolean'}}
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
   Inserted with ripple: {'id': 'ovn910AKsIMIUkaCG3c_Qkz74sjGwD4eK71C9ZHs6p3cWR4_Gs0Ul0CPSoYfsAOmFReROfsm4RmgNSy4FnWaQw'}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {'id': 'W9_YSlY8P9hhm_RA6PzGCNE1GRmbZXfBmdmCVsIZSGPNBbotWvb4BjUJNrVMBDbO-8_vPoMPsEMBgTyIx29dMg'}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {'id': 'ovn910AKsIMIUkaCG3c_Qkz74sjGwD4eK71C9ZHs6p3cWR4_Gs0Ul0CPSoYfsAOmFReROfsm4RmgNSy4FnWaQw', 'price': {'type': 'Integer', 'value': 150}, 'name': {'value': 'Product 1', 'type': 'String'}}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {'name': {'type': 'String', 'value': 'Upsert Product'}, 'price': {'type': 'Integer', 'value': 500}, 'id': 'ovn910AKsIMIUkaCG3c_Qkz74sjGwD4eK71C9ZHs6p3cWR4_Gs0Ul0CPSoYfsAOmFReROfsm4RmgNSy4FnWaQw'}

✅ All bypass_ripple operations completed successfully!
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields (id, name, email only)
  Found 3 active users
  Fields returned: ['email', 'name', 'id']
  First user: {'value': 'Bob Smith', 'type': 'String'} <{'type': 'String', 'value': 'bob@example.com'}>

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ['id', 'age', 'bio', 'email', 'created_at', 'avatar_url', 'status', 'name', 'user_role']

Example 3: Complex query with projection (active users, ages 18-65)
  Found 3 active users (ages 18-65)
    - {'value': 'Dave Brown', 'type': 'String'} (age {'type': 'Integer', 'value': 45})
    - {'value': 'Alice Johnson', 'type': 'String'} (age {'type': 'Integer', 'value': 30})
    - {'type': 'String', 'value': 'Bob Smith'} (age {'type': 'Integer', 'value': 25})

Example 4: Query inactive users with profile fields
  Found 1 inactive users
    - {'value': 'Carol White', 'type': 'String'}: {'type': 'String', 'value': 'Manager'}

Example 5: Compare full vs projected data
  Full query:
    - 12 fields per record
    - Fields: ['avatar_url', 'name', 'id', 'user_role', 'age', 'secret_token', 'status', 'email', 'api_key', 'created_at', 'password', 'bio']
  Projected query:
    - 3 fields per record
    - Fields: ['id', 'name', 'email']
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
Created user function with ID: m7BYKcRq3lof-yiA0DnJAtG1hI5VsGwtB6ukLi5hD01geFx5vFP1XjLrQ4ZgZsSzN1XaU_6aNMZS4lBl_qhDDg

=== Get User Function ===
Retrieved: get_active_users_py - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 25 user functions:
  - fetch_and_store_user: Fetch user from API and cache in KV
  - get_active_users: Get Active Users (Updated)
  - get_active_users_paginated: Get Active Users (Paginated)
  - get_active_users_updated: Get Active Users (Updated)
  - fetch_and_store_user: Fetch user from API and cache in KV
  - get_active_users_paginated: Get Active Users (Paginated)
  - fetch_slim_user: Validate and slim down user
  - swr_user: SWR pattern for user data (KV-based)
  - get_user_wrapper: Wrapper that calls fetch_user
  - validate_user: Check if user exists
  - cache_api_call_py: Cache External API Call
  - get_verified_user: Get verified and validated user
  - get_active_users: Get Active Users (Updated)
  - get_high_scoring_active_users: Get High Scoring Active Users
  - get_active_users_updated: Get Active Users (Updated)
  - get_active_users_py: Get Active Users
  - get_users_by_status: Get Users By Status
  - validate_user: Check if user exists
  - get_users_by_status: Get Users By Status
  - fetch_user: Fetch user by code
  - fetch_slim_user: Validate and slim down user
  - get_user_wrapper: Wrapper that calls fetch_user
  - fetch_user: Fetch user by code
  - swr_user: SWR pattern for user data (KV-based)
  - get_verified_user: Get verified and validated user

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
