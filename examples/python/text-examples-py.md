make test-examples-python
🧪 [36mRunning Python examples (direct HTTP/WebSocket)...[0m
=== Simple CRUD Operations (Direct HTTP) ===

✓ Authentication successful

=== Insert Document ===
Inserted: {'id': 'WdJ8uJ9wsbKEI8e6qVpy57wrEQEhSP2lacYKecdlVIwwWwBfpOpg1ZlKgGLyPVxaNEbpwWqCGJYN8Xki47vavA'}

=== Find by ID ===
Found: {'name': {'value': 'Test Record', 'type': 'String'}, 'id': 'WdJ8uJ9wsbKEI8e6qVpy57wrEQEhSP2lacYKecdlVIwwWwBfpOpg1ZlKgGLyPVxaNEbpwWqCGJYN8Xki47vavA', 'active': {'type': 'Boolean', 'value': True}, 'value': {'type': 'Integer', 'value': 42}}

=== Find with Query ===
Found documents: [{'value': {'type': 'Integer', 'value': 42}, 'id': 'WdJ8uJ9wsbKEI8e6qVpy57wrEQEhSP2lacYKecdlVIwwWwBfpOpg1ZlKgGLyPVxaNEbpwWqCGJYN8Xki47vavA', 'active': {'value': True, 'type': 'Boolean'}, 'name': {'type': 'String', 'value': 'Test Record'}}]

=== Update Document ===
Updated: {'id': 'WdJ8uJ9wsbKEI8e6qVpy57wrEQEhSP2lacYKecdlVIwwWwBfpOpg1ZlKgGLyPVxaNEbpwWqCGJYN8Xki47vavA', 'name': {'value': 'Updated Record', 'type': 'String'}, 'active': {'value': True, 'type': 'Boolean'}, 'value': {'value': 100, 'type': 'Integer'}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: MtXpLar30tru4Y354yBpOXUjpyCzRaHSnY8__zDq3z-SdZu4fg-jkmuyQAxukeHRhr6q1tC5bKDJeWdu3oDqHQ

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "messageId": "1298024898",
  "payload": {
    "data": [
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "FeqA3vaSCqzaM09W3ES2j5OE8ThY6bgfDXBhYB9paIu6aM8tKHwgA8kvLgSA_dvnIjkt2zrbSb_ciW8KsFJ-KA",
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
        "id": "MtXpLar30tru4Y354yBpOXUjpyCzRaHSnY8__zDq3z-SdZu4fg-jkmuyQAxukeHRhr6q1tC5bKDJeWdu3oDqHQ",
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

✅ Function saved: 30Rcfh0S1_tDA0REL_8TLAVAt7oSfmeZVVdJ1nfohg40imC_jkYgJhT8QWMYQIOOniiDI4zPsBUxArLpdK9qIA
📊 Found 15 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Function saved: XWbR9w_jYppWcRvXU6946Myj_78qIQVTxR-B9T-h7pRR8lVJuaPxc8GfX1nAcOIQmcXL0o3ePwDaW5riyNSUJg
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: 7IDPEuRq9zsjmbcxHq8cWH8S5s_fSYlB1d9Zh9xgcmW0tb4AJVWtpgrklJupWnq729DGJ6mdI7Y7Bfpz1Dq9hA
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {'max_score': 90, 'count': 15, 'status': 'inactive', 'avg_score': 50.0}
   {'count': 15, 'avg_score': 60.0, 'max_score': 100, 'status': 'active'}

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
cache:product:1: {'name': 'Product 1', 'price': 29.99}
cache:product:2: {'price': 39.989999999999995, 'name': 'Product 2'}
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
Collection created with first record: RNBRz3unpC8JLl6bZ9iPFN5RfyBEKH0RSlDnGu9itEs1tj0Dq_Y-JypIVj_G3fHtNoGrUorBHnlKxqvtLP1frg

=== List Collections ===
Total collections: 13
Sample collections: ['test_collection', 'users', 'demo_collection', 'chat_messages__ek0_testing', 'test_accounts']

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
Created Alice: $1000 - ID: RikEQ1EPWT-w_ci_koQ9NgOg6PvyA0H0U3_pqm24s3rBCv1AXb2I-QV84Rs-j92u0IYrDtcG2tRp7yyhuYzVOg
Created Bob: $500 - ID: pULGJIiTdSocS4mtO8ieqofOVAclZs7qMJj3riTBGQy1y5StiPWz8N0IkviXE4UfddyzrBgeNAUWUOcheEOg6g

=== Example 1: Begin Transaction ===
Transaction ID: 573ce337-76d3-4b81-9909-eb0710ae248d

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
New transaction: bfd48f1d-3d2d-4ae8-b5b1-a9f44573e28f
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'type': 'Integer', 'value': 700}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: io6bd4_UtScng2ZDToJ0j8LatcA9IwztCBZrj6Xz9q9NTMGmxM9UKB5VPBNppvKXIMX7vosNx_VHWEmIGz0XTw
Created Bob: $500 - ID: Ag8SajFYgBLlKy1eyj6wxy34pTu10Z5O7MAY2_SeI_IBGAnnVak_pM55Fi4pBHc6G_KtbM5KoqLskp8MGMeKpw

=== Example 1: Begin Transaction ===
Transaction ID: 6baafb9e-0659-40e8-87d2-13e086e2e1d5

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
Created Alice: $1000 - ID: gqfq4CrqlEjK1AmCxbqdAFAKbw3PV7Iagx1ZtyKxqBf8g8L7ey-oPLLLKWiLSRfby4xT-kgAdXZwAl_oOUwNMg
Created Bob: $500 - ID: ODri0-rXExlsv_dPhDU-gb1QAT8seh072nnaBGLdxzteN8_yd_u2I9aP79Rzqq5ilbjP2yF-kVXUvtiBJafRRg

=== Example 1: Begin Transaction ===
Transaction ID: 9d1875a7-1554-4f20-8b2b-cc1b62e77bd8

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
Bob: ${'value': 700, 'type': 'Integer'}

=== Example 5: Rollback ===
New transaction: bd2e684d-b05b-44a1-8ac3-c7b15d457205
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'type': 'Integer', 'value': 700}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: oawEfYiNoO5-8xGSnBkj5IT4FUQTeTMEXP7AlRYSIALmNTItMKS5FpOTbxGiWCp5Hwc6yxTgvNtI6eIOenN5wg
Created Bob: $500 - ID: R4Ita1trYuP6HVMLIV2jsiXJXZVjb0fe5GrnC1Hch0RWgMuL0PGgyQr5b39XZOApzi4CWrwRl6-M0kLmogy6fg

=== Example 1: Begin Transaction ===
Transaction ID: cfba7079-cc81-4795-883a-58d2ace46b72

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
Created Alice: $1000 - ID: TRIDZuUYexPqCevz68nATwBd9LpSYQ_7SjxnLuywubks2IehG8BhZotN0EjUT98QyVizMPRX90bHupMzezvECQ
Created Bob: $500 - ID: rg0JAeaD0LAjSlxQ-WEPJ-F29DYCpOrZNsSNr_LHcsQw3JBF0RmUtwCFQrmBC_XiM0x_ElAuoXpbAvz1lw8IhA

=== Example 1: Begin Transaction ===
Transaction ID: bf6ba4a5-d194-4a07-b8e9-9b730ed061fb

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
New transaction: 98231c42-b840-4644-973b-8355d5f47d06
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'value': 700, 'type': 'Integer'}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 24bvdZX94ItbiY-ommS-jQvIj9oC6h2IlU-aulfSaggBCBnpcqmDbjQyk3RZ9ZkBNYG_R2QITlUhlA_HMi_R_A
Created Bob: $500 - ID: pOJY-u0pGasNGIFWFcWYpKePaAYfUwsKyf78a2j6Y5WiDxSPbOXYYfl5I-fteqVIs-GvUc_tB_NGTdpNNR6lNA

=== Example 1: Begin Transaction ===
Transaction ID: bdbf7d12-b0f3-467c-a0d8-e3c40f5d6203

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
   ✅ Function saved: I9TMs7eKhQHBDO-Wwzti7ms7EF1zZfEpHOMnIq5mBPy4XRr73gUzXb1Xf2-lMp300t_jU-xijs_K3HOp6IBUxA

2️⃣ Calling function (Insert + Verify)...
   ✅ function executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: 44plh4LIt-WV_bc5uOw-l52qdAp4me4ZP9kPREdQ7GeBIt4WeMfM7aojc95C5ShE8QHHAhZpSuUPAakvPyl2WA
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}
   📋 Email: {'value': 'alice@example.com', 'type': 'String'}
   📋 Status: {'type': 'String', 'value': 'pending'}
   📋 Credits: {'value': 0, 'type': 'Integer'}

============================================================
📝 function 2: Query + Update + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: LmjF0RUfyhWd2xf3sNHt8rwtnCvXdvTMkiWL171oUEngoA9-is51ZGTml6jDIaFHN4NFu3K8Ccg42tzbVbl0rg

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
   ✅ Function saved: 17N2lnC2BGd3J3pIPdDqOC-LbdFLxhbKi9BNuR202ZgElNrcvCWB4DNBfXNSWeY2AYLIYJU_q86z2Iob6Eka_w

2️⃣ Calling function (Query + Update Credits + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {'type': 'Integer', 'value': 0}
   📋 Status: {'type': 'String', 'value': 'active'}
   📋 Name: {'value': 'Alice Smith', 'type': 'String'}

============================================================
📝 function 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: QBwk1YCBFk5rfSpOxxIxemkW3okxpeo77ZczbrMBa_TVjWDnBarDr1OwEmUQoZecTjlHai0tNWwBP6gqBVkQOQ

2️⃣ Calling function (Query + Delete + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: I9TMs7eKhQHBDO-Wwzti...
   ✅ Deleted script: LmjF0RUfyhWd2xf3sNHt...
   ✅ Deleted script: 17N2lnC2BGd3J3pIPdDq...
   ✅ Deleted script: QBwk1YCBFk5rfSpOxxIx...
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
  Output: Document ID = NZZgZgQZvGHBWtjWF4dVZ7fD82iu00czC8oHCOW_f-aTG_CVDXcGHV7oG6ZjCZfOFGI1AAVCRuw4pNh9y4gadA
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: find_by_id(NZZgZgQZvGHBWtjWF4dVZ7fD82iu00czC8oHCOW_f-aTG_CVDXcGHV7oG6ZjCZfOFGI1AAVCRuw4pNh9y4gadA)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: find_by_id(NZZgZgQZvGHBWtjWF4dVZ7fD82iu00czC8oHCOW_f-aTG_CVDXcGHV7oG6ZjCZfOFGI1AAVCRuw4pNh9y4gadA)
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
  Output: Document ID = erTs4ibLtrEo8Fg-Gblr_yjOZCzVHVPcMXefX-DX9YPrchQHdNgXoTUa8RQwGapV6JUNoeXGogFSpzYhJbeTWg
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: find_by_id(erTs4ibLtrEo8Fg-Gblr_yjOZCzVHVPcMXefX-DX9YPrchQHdNgXoTUa8RQwGapV6JUNoeXGogFSpzYhJbeTWg)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: find_by_id(erTs4ibLtrEo8Fg-Gblr_yjOZCzVHVPcMXefX-DX9YPrchQHdNgXoTUa8RQwGapV6JUNoeXGogFSpzYhJbeTWg)
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
    Finished `release` profile [optimized] target(s) in 0.14s
📦 Built wheel for abi3 Python ≥ 3.8 to /Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.16.0-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel for current platform...[0m
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.16.0-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.16.0
    Uninstalling ekodb_client-0.16.0:
      Successfully uninstalled ekodb_client-0.16.0
Successfully installed ekodb-client-0.16.0
✅ [32mPython client package built and installed![0m
🧪 [36mRunning Python client library examples...[0m
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {'id': 'UA_NbBobOS5BHsfC2sbG-ipxoqesxiB60dLE6iPdxONRdOZ9tpjJn8Ga1Z7hVVwQUC2M1PVUHRnLWJYutw5e9g'}

=== Find by ID ===
Found: {'price': {'type': 'Float', 'value': 99.99}, 'created_at': {'value': '2026-04-12T12:42:04.163113', 'type': 'String'}, 'value': {'type': 'Integer', 'value': 42}, 'metadata': {'type': 'Object', 'value': {'key': 'value', 'nested': {'deep': True}}}, 'id': 'UA_NbBobOS5BHsfC2sbG-ipxoqesxiB60dLE6iPdxONRdOZ9tpjJn8Ga1Z7hVVwQUC2M1PVUHRnLWJYutw5e9g', 'active': {'type': 'Boolean', 'value': True}, 'data': {'value': 'aGVsbG8gd29ybGQ=', 'type': 'String'}, 'name': {'type': 'String', 'value': 'Test Record'}, 'categories': {'type': 'Array', 'value': ['electronics', 'computers']}, 'tags': {'type': 'Array', 'value': ['tag1', 'tag2', 'tag3']}, 'embedding': {'type': 'Array', 'value': [0.1, 0.2, 0.3, 0.4, 0.5]}, 'user_id': {'value': '550e8400-e29b-41d4-a716-446655440000', 'type': 'String'}}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): True
  price (Decimal): 99.99
  created_at (DateTime): 2026-04-12 12:42:04.163113
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): ['tag1', 'tag2', 'tag3']
  metadata (Object): {'key': 'value', 'nested': {'deep': True}}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): ['electronics', 'computers']
  data (Bytes): 11 bytes
Plain record: {'price': 99.99, 'created_at': '2026-04-12T12:42:04.163113', 'value': 42, 'metadata': {'key': 'value', 'nested': {'deep': True}}, 'id': 'UA_NbBobOS5BHsfC2sbG-ipxoqesxiB60dLE6iPdxONRdOZ9tpjJn8Ga1Z7hVVwQUC2M1PVUHRnLWJYutw5e9g', 'active': True, 'data': 'aGVsbG8gd29ybGQ=', 'name': 'Test Record', 'categories': ['electronics', 'computers'], 'tags': ['tag1', 'tag2', 'tag3'], 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5], 'user_id': '550e8400-e29b-41d4-a716-446655440000'}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'user_id': {'type': 'String', 'value': '550e8400-e29b-41d4-a716-446655440000'}, 'active': {'type': 'Boolean', 'value': True}, 'metadata': {'type': 'Object', 'value': {'nested': {'deep': True}, 'key': 'value'}}, 'price': {'type': 'Float', 'value': 99.99}, 'id': 'UA_NbBobOS5BHsfC2sbG-ipxoqesxiB60dLE6iPdxONRdOZ9tpjJn8Ga1Z7hVVwQUC2M1PVUHRnLWJYutw5e9g', 'embedding': {'value': [0.1, 0.2, 0.3, 0.4, 0.5], 'type': 'Array'}, 'created_at': {'value': '2026-04-12T12:42:04.163113', 'type': 'String'}, 'data': {'type': 'String', 'value': 'aGVsbG8gd29ybGQ='}, 'name': {'value': 'Updated Record', 'type': 'String'}, 'categories': {'type': 'Array', 'value': ['electronics', 'computers']}, 'tags': {'value': ['tag1', 'tag2', 'tag3'], 'type': 'Array'}, 'value': {'type': 'Integer', 'value': 100}}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: lpQbH6HueYa2A-Mu6jXgj9_h71jPDsq69AlGT3LIyCk3_m7-pknUf86zr60NmetENtxMddPyWzqDF8X6fbcavA

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
Collection created with first record: "H8MQ2ad7oR-tq0gyIC_l0hLmL2YlwvAUlcTycO-lljjULbhTJdKpKaPF3pMvrAoJKOJ0VH501njlMomNBsRNZg"

=== List Collections ===
Total collections: 12
Sample collections: ['test_collection', 'chat_messages__ek0_testing', 'test_accounts', 'ws_ttl_test', 'ttl_cache']

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
Created Alice: $1000 - ID: J1WVBnMGF8AZKfh88xYTjoIWdZxml0_6mloMQsOAOss7MwlHdAjP7ZIJ85NNDkThBLA-GXEzGDDHaKN5TXRz7Q
Created Bob: $500 - ID: IOESA3wrBngUFyVFNjkq5eCKXXGHc5ind2okvx5rc_mvsqTwUg_6Zq2Rq108BK1VvhgdSE6aOhrXq4ViIJ6j7g

=== Example 1: Begin Transaction ===
Transaction ID: cdd27df7-9b82-49b9-8757-4fe1e84b4f0e

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: f2a84bf5-92c5-409b-becc-812e38a7de4b
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
  1. Score: 25.740, Matched: name, email, name.value, email.value
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: bio.value, title, bio, title.value
  2. Score: 26.400, Matched: title, title.value, bio, bio.value
  3. Score: 26.400, Matched: title.value, title, bio.value, bio
  4. Score: 26.400, Matched: title, title.value, bio, bio.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio.value, title.value, title, bio
  2. Score: 39.600, Matched: title, title.value, bio, bio.value
  3. Score: 39.600, Matched: bio.value, title, title.value, bio
  4. Score: 39.600, Matched: title.value, bio.value, title, bio

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio.value, bio

5. Search with stemming and exact match boosting:
Found 2 results (matches: run, running, runs)
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.755
  2. Score: 0.743
  3. Score: 0.740

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.378, Matched: title, title.value, content, content.value
  2. Score: 1.370, Matched: content.value, content, title.value, title
  3. Score: 0.372, Matched: 

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
✓ Inserted document: CtiMtUuArxr7MB2je5QcbmcuIl1OnSNYJbiYFuwExJp0fHKPMvrBuLNZO0CQAah-lsW3XxX-IyoK2hQoKrHPoA

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: 1nkuxhbE0Vih9w2kpHSTDIJWaXzjeSy5SpRpK7ufofp2mguP9a4-kie2g8OI21T2Co_XydQjFZVsriaTN7u8RQ

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
✓ Inserted document with TTL: 0MIvzroraxeiqB42D27y3ecEExA7UmfNY6H4AFsgQTgkjH4qul9Ji49I_u5rZZCb0vxqF3xaM_0VMfOWHU7OXA

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
✓ Edge cache script created: 8IPVpceoHAT1dHqwvn7wA4zu02h63opz4R1HPSGOT5xNG4oiDtk_ww5RvfeXOgpC5OxqJ-Gc3bunV4cCwOeB-g

Call 1: Cache miss (fetches from API)
Response time: 560ms
Result: {
  "records": [
    {
      "value": {
        "current": {
          "interval": 900,
          "temperature_2m": 12.3,
          "time": "2026-04-12T16:30"
        },
        "current_units": {
          "interval": "seconds",
          "temperature_2m": "\u00b0C",
          "time": "iso8601"
        },
        "elevation": 32.0,
        "generationtime_ms": 0.025153160095214844,
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
Response time: 40ms (14.0x faster!)
Result: {
  "records": [
    {
      "value": {
        "current": {
          "interval": 900,
          "temperature_2m": 12.3,
          "time": "2026-04-12T16:30"
        },
        "current_units": {
          "interval": "seconds",
          "temperature_2m": "\u00b0C",
          "time": "iso8601"
        },
        "elevation": 32.0,
        "generationtime_ms": 0.025153160095214844,
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

✅ Function saved: T1TcFzTOyzlWdvuZv30urBQfWeSrlpvz8fmzwVgL5Q5XTsig1qbCS9ug_-jds_AF8PJRo6KdD2qIomSEj0wYbA
📊 Found 10 active users

📝 Example 2: Parameterized Function

✅ Function saved: tAdW0sFeDwMIdF84pfRNBgqFcRWdvfUGWsNVvk8SsbYfosP1YELS8IbIInSNX02XmwpqGWOfUkZCDOvs4hcvsw
📊 Found 10 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: eVMMs_KOY_clv8BlCDpXnYsTMidI9v44w8CwnNm57Z74imIwuII4KyqlzOjz2sH83whcmjwVYgjmn92CkbhiHg
📊 Statistics: 2 groups
   {'avg_score': 50.0, 'count': 5, 'status': 'inactive'}

   {'avg_score': 60.0, 'count': 5, 'status': 'active'}

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

Second call (cache hit - from cache):
   ⏱️  Duration: 38.6ms
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

✅ Function saved: 5I92eiN5y_1sRKHl0Qk_cZQlNuZbXbz45kSNoUFUQpqlcjobtFRqvqpu-Ise0KgUJxE5Rt-XUTDoLm9ol6EVGg
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

✅ Inserted order: chJcpoVxHIzGAkaBXIitO1RLU_EjCD1XXL3uNTMmbV7k2D21LHGQvwg_Zyk3VTTVpL3x7J964EQF82l_2X-vaQ
✅ Inserted 2 products with wrapped types

📝 Example 2: Querying and Extracting Wrapped Types

📊 Found 2 products
   • Laptop Pro
   • Wireless Mouse

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

✅ Inserted order: paFyhS-TF4wFfzm8Bzv0TX79UtzR3lR1oAJwAg_gt-jTMh10pzm9knOdaedE7FxAtqXyXI6s_zxLh5b0Z4bj_Q
✅ Cached order status
📊 Quick status lookup: {'value': '{"status":"processing","updated_at":"2026-04-12T16:42:06.935069+00:00"}'}

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
✓ Created SWR script: fetch_api_user_py (eTQdgAbFbazgJxhaYfjHzpUzsubfqXeD8IRjr4mj-dK4dDZORnKjFQX7ytk1_oBSo7_f0BDlsmk9iAKhGB-7_A)

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
Response time: 39ms (served from cache)
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
✓ Created native SWR script: github_user_native (bdgx8hGjrX5XJdLf3IPZKq980fg9aqOZjUrYzMaqUf5We_4LrZj1t7rYEk-M0r12dkBwKR3TShGAuW0GQzRP2w)

First call (cache miss - will fetch from GitHub API):
  Response time: 128ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 23ms
  Speedup: 5.6x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (PRNTcy75K0XbcLZqjRRA-isE68E8_PkpIvz1gH1uWW_tlvgeXwOCqteQQ3Et5D1WHYNAqvbsxJch6-Y7WGJ_Kg)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (_p_Rw41NzK3XE1Dbl_9eC4QlUe6_f2BwY6jCI9XJZUvPr83PgD7OA-2qX8waCQeLV9h9xflI5djZ_bgN0ReDdA)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (OdapOh2Z0nWXxF5WaY5g9Z7NvJrIkHD5al5GreylrAGd58hWb3a-ZflUU5t7eZUtsSHR7oBw9sp4CsIdXs138g)
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
   {'avg_price': 367.0, 'category': 'Electronics', 'count': 5}
   {'avg_price': 365.6666666666667, 'category': 'Furniture', 'count': 3}
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
   The benefits of using vector databases include:

1. **High-dimensional data handling**: They efficiently store and manage high-dimensional vectors, ideal for complex data types like images, text, and audio.

2. **Fast similarity search**: Vector databases support rapid nearest-neighbor search capabilities, enabling quick retrieval of similar items based on vector representations.

3. **Scalability**: They are designed to scale with large datasets, making them suitable for applications with extensive data growth.

4. **Support for machine learning**: They facilitate the integration of machine learning models, allowing for real-time inference and analysis.

5. **Multi-modal data support**: Vector databases can handle diverse data types, enabling applications in various domains such as natural language processing, computer vision, and recommendation systems.

6. **Enhanced performance**: Optimized algorithms improve performance in search and retrieval tasks, reducing latency and bandwidth usage.

7. **Similarity-based recommendations**: They enable personalized content recommendations based on user behavior and preferences measured through vector representations. 

8. **Flexibility**: Users can easily implement advanced analytics and personalized applications using the stored vector data.
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
   1. Natural Language Processing (AI)
   2. Introduction to Machine Learning (AI)
   3. Getting Started with ekoDB (Database)
   4. Vector Databases Explained (Database)
   5. Database Design Principles (Database)
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
✓ Created session: hqgZ7u1n4z1PjYSwN3mGKkYDjbJgw-JhqI7U6wQhwAaSmQMFswL4loRxhLaW_4sKaj0SpDoJPx-rNtOdQ8yMWw

=== Sending Chat Message ===
Message ID: GXpyUv0anDvvXrmvJJxOeiv7Ctkm2el2AFSvSKZxKbH2oBb2HvUvaSmHW-YeO5hhC8Q3yWOiaYC5TUZI7Z6LOA

=== AI Response ===
Here are the available products and their prices:

1. **ekoDB**
   - Price: $99
   - Description: A high-performance database product with AI capabilities.

2. **ekoDB Cloud**
   - Price: $499
   - Description: Fully managed cloud database service product.

3. **ekoDB Pro**
   - Price: $299
   - Description: Enterprise edition product with advanced features.

Execution Time: 3368ms

=== Token Usage ===
Prompt tokens: 1298
Completion tokens: 90
Total tokens: 1388

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 0njM6U28ucl9FaZZebz_LXUVIxYIyRjXIwOyo-O31JcDu8LHpawYqBQJWY4ae5PRGPglZqsaTOP5tMDJd0qxvQ

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: High-performance database product
- **Price**: $99

If you need more information or want to see additional products, please let me know!

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['llm_provider', 'created_at', 'role', 'content', 'chat_id', 'id', 'token_usage', 'context_snippets', 'updated_at', 'llm_model'])
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
✓ Created second session: 5qaltAlAg_c0xW2xSTM4Las-hGRQUIRoLBisgrFpRUkeQd8WdOmlaAueAj-04BaV2RHQ5oroV5f16673NJTdjQ
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
✓ Created session: yjDtt3Myn4J9VCsJl1q64BwCSIek6DgKpLdOUgwWzwGL8ht9XgjOAZSpGsomKCMrt42gOaiH59G-lArj1Zj4Mg

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: A high-performance database product
- **Price**: $99

If you need more information or details on additional products, just let me know!

✓ Message 2 sent
  Response: The price of ekoDB is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: zOxm83y9Z8HSQLgfcOuKLxLm7gNlfzIxpK26xdJZVA2rd8Up6nBp68tJ47tZXQXbxwp2rsuzzik5dK3MtW1NZg
  Parent: yjDtt3Myn4J9VCsJl1q64BwCSIek6DgKpLdOUgwWzwGL8ht9XgjOAZSpGsomKCMrt42gOaiH59G-lArj1Zj4Mg

=== Listing Sessions ===
✓ Found 7 sessions
  Session 1: zOxm83y9Z8HSQLgfcOuKLxLm7gNlfzIxpK26xdJZVA2rd8Up6nBp68tJ47tZXQXbxwp2rsuzzik5dK3MtW1NZg (Untitled)
  Session 2: yjDtt3Myn4J9VCsJl1q64BwCSIek6DgKpLdOUgwWzwGL8ht9XgjOAZSpGsomKCMrt42gOaiH59G-lArj1Zj4Mg (Untitled)
  Session 3: 5qaltAlAg_c0xW2xSTM4Las-hGRQUIRoLBisgrFpRUkeQd8WdOmlaAueAj-04BaV2RHQ5oroV5f16673NJTdjQ (Untitled)
  Session 4: hqgZ7u1n4z1PjYSwN3mGKkYDjbJgw-JhqI7U6wQhwAaSmQMFswL4loRxhLaW_4sKaj0SpDoJPx-rNtOdQ8yMWw (Untitled)
  Session 5: 0ZZzmUTPDZOFp4razygtycQFjXKop4-aPaMKK1bzpnKQ_UXhdMTBB1So4reoVYfYqSZXHnBjvWsejC2HlwDJNw (Untitled)
  Session 6: l5DeQ5mR60sriqk2G-P3hkq7CA5vp5JG9onoxl5Z3eTWo12j2q-ACHbJp8bD_xx8JeWcaeb6ILOzK82IBltWmA (Untitled)
  Session 7: W_XoN7N1gKoGeMnrYxB84031ZTTwjzTNPGO90Hga4096w6VlR5nj8oHRtfCyQNWugwRiHGvLiRlRrPYKATXZoA (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: zOxm83y9Z8HSQLgfcOuKLxLm7gNlfzIxpK26xdJZVA2rd8Up6nBp68tJ47tZXQXbxwp2rsuzzik5dK3MtW1NZg

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Dict Creation ===
✓ Created record with plain dict: {'id': '0jye6ovJKPeRJjD98BzB_OwDzUS3zlg0rJoHPKXXMQCje9UcBcxKYHsxebdrjAIwJWktmTTH24EFTMImOD_cMQ'}

=== Upsert Operation ===
✓ Upsert (update existing record): 0jye6ovJKPeRJjD98BzB_OwDzUS3zlg0rJoHPKXXMQCje9UcBcxKYHsxebdrjAIwJWktmTTH24EFTMImOD_cMQ
✓ Inserted second record: tMSszyKbDxp0X2_gX3qALdU3dWfCAipOa25jG8n097tYYByirbSoY5uufsqCrldR8fBkh9qVU_Lp1-C48aLEPQ
✓ Upsert (update second record): tMSszyKbDxp0X2_gX3qALdU3dWfCAipOa25jG8n097tYYByirbSoY5uufsqCrldR8fBkh9qVU_Lp1-C48aLEPQ

=== Find One Operation ===
✓ Found user by email: {'active': {'value': True, 'type': 'Boolean'}, 'email': {'type': 'String', 'value': 'alice.j@newdomain.com'}, 'age': {'value': 29, 'type': 'Integer'}, 'name': {'type': 'String', 'value': 'Alice Johnson'}, 'id': '0jye6ovJKPeRJjD98BzB_OwDzUS3zlg0rJoHPKXXMQCje9UcBcxKYHsxebdrjAIwJWktmTTH24EFTMImOD_cMQ'}
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
   Inserted with ripple: {'id': 'rYJGFmQu3HVs_bI_44AElfp4L9Fbx5aEnQVnJlXKL5v4b9xxjy-yBDOzwzRwME9SHBxys278Y7XIXaQc2SR-Ig'}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {'id': 'jHdG183ZBzRsPyw019oM6STyq21yLGvOIm7XSvjJKNx4Tifmo5rSRR2Q6VrqIAHUYS6r7ZlgMdL4mFOpv-a4Sg'}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {'price': {'type': 'Integer', 'value': 150}, 'id': 'rYJGFmQu3HVs_bI_44AElfp4L9Fbx5aEnQVnJlXKL5v4b9xxjy-yBDOzwzRwME9SHBxys278Y7XIXaQc2SR-Ig', 'name': {'type': 'String', 'value': 'Product 1'}}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {'name': {'value': 'Upsert Product', 'type': 'String'}, 'price': {'type': 'Integer', 'value': 500}, 'id': 'rYJGFmQu3HVs_bI_44AElfp4L9Fbx5aEnQVnJlXKL5v4b9xxjy-yBDOzwzRwME9SHBxys278Y7XIXaQc2SR-Ig'}

✅ All bypass_ripple operations completed successfully!
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields (id, name, email only)
  Found 3 active users
  Fields returned: ['id', 'email', 'name']
  First user: {'type': 'String', 'value': 'Alice Johnson'} <{'type': 'String', 'value': 'alice@example.com'}>

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ['name', 'email', 'bio', 'avatar_url', 'status', 'id', 'created_at', 'user_role', 'age']

Example 3: Complex query with projection (active users, ages 18-65)
  Found 3 active users (ages 18-65)
    - {'type': 'String', 'value': 'Dave Brown'} (age {'value': 45, 'type': 'Integer'})
    - {'value': 'Alice Johnson', 'type': 'String'} (age {'type': 'Integer', 'value': 30})
    - {'value': 'Bob Smith', 'type': 'String'} (age {'value': 25, 'type': 'Integer'})

Example 4: Query inactive users with profile fields
  Found 1 inactive users
    - {'value': 'Carol White', 'type': 'String'}: {'value': 'Manager', 'type': 'String'}

Example 5: Compare full vs projected data
  Full query:
    - 12 fields per record
    - Fields: ['user_role', 'name', 'secret_token', 'created_at', 'email', 'api_key', 'id', 'status', 'age', 'bio', 'avatar_url', 'password']
  Projected query:
    - 3 fields per record
    - Fields: ['name', 'id', 'email']
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
Created user function with ID: rUkdJP97IT9jACNcXv7KeZrC0OqkgROGWUO5oMQ7uezjz-nZH0Tb9QaQb8D3HCzGz6Xum0qAmix5wg3-0dxadA

=== Get User Function ===
Retrieved: get_active_users_py - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 25 user functions:
  - get_active_users_updated: Get Active Users (Updated)
  - fetch_and_store_user: Fetch user from API and cache in KV
  - get_users_by_status: Get Users By Status
  - get_verified_user: Get verified and validated user
  - get_verified_user: Get verified and validated user
  - get_users_by_status: Get Users By Status
  - get_active_users_py: Get Active Users
  - fetch_slim_user: Validate and slim down user
  - get_user_wrapper: Wrapper that calls fetch_user
  - cache_api_call_py: Cache External API Call
  - get_active_users_paginated: Get Active Users (Paginated)
  - get_active_users_updated: Get Active Users (Updated)
  - get_high_scoring_active_users: Get High Scoring Active Users
  - fetch_slim_user: Validate and slim down user
  - validate_user: Check if user exists
  - get_active_users_paginated: Get Active Users (Paginated)
  - get_user_wrapper: Wrapper that calls fetch_user
  - swr_user: SWR pattern for user data (KV-based)
  - validate_user: Check if user exists
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_user: Fetch user by code
  - swr_user: SWR pattern for user data (KV-based)
  - get_active_users: Get Active Users (Updated)
  - fetch_user: Fetch user by code
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
