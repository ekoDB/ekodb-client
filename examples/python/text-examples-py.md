make test-examples-python
🧪 [36mRunning Python examples (direct HTTP/WebSocket)...[0m
=== Simple CRUD Operations (Direct HTTP) ===

✓ Authentication successful

=== Insert Document ===
Inserted: {'id': '6S9N7IIuTqqLdeHFcjGKjtf2UTX5m10knQGujwhzKC_oXG949LojxqOa9S1Gu_q9fuRm-4gBiD1cBxg19ysETg'}

=== Find by ID ===
Found: {'value': {'type': 'Integer', 'value': 42}, 'active': {'type': 'Boolean', 'value': True}, 'name': {'type': 'String', 'value': 'Test Record'}, 'id': '6S9N7IIuTqqLdeHFcjGKjtf2UTX5m10knQGujwhzKC_oXG949LojxqOa9S1Gu_q9fuRm-4gBiD1cBxg19ysETg'}

=== Find with Query ===
Found documents: [{'value': {'value': 42, 'type': 'Integer'}, 'name': {'value': 'Test Record', 'type': 'String'}, 'active': {'type': 'Boolean', 'value': True}, 'id': '6S9N7IIuTqqLdeHFcjGKjtf2UTX5m10knQGujwhzKC_oXG949LojxqOa9S1Gu_q9fuRm-4gBiD1cBxg19ysETg'}]

=== Update Document ===
Updated: {'value': {'value': 100, 'type': 'Integer'}, 'active': {'value': True, 'type': 'Boolean'}, 'name': {'type': 'String', 'value': 'Updated Record'}, 'id': '6S9N7IIuTqqLdeHFcjGKjtf2UTX5m10knQGujwhzKC_oXG949LojxqOa9S1Gu_q9fuRm-4gBiD1cBxg19ysETg'}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: fQW6SBGgSvKuNS5AKtPgbx9f8znVSScfJfJFeZectmHoGisu5TIZw-ftWvtZkUwKd3podcIrg1_b_HMlYHimDA

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "messageId": "20246520",
  "payload": {
    "data": [
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "fQW6SBGgSvKuNS5AKtPgbx9f8znVSScfJfJFeZectmHoGisu5TIZw-ftWvtZkUwKd3podcIrg1_b_HMlYHimDA",
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
        "id": "gFEqTXq3N-9Rjq25gBAgck5lECyiNA4m7S70bsLetAO7GjvryzHjPOF5SCbfMLKA7rf5fjVQkU9GUpQFU5H-cA",
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

✅ Script saved: 2Iqh9SsdncTNBnb5iotzQfo6tg1FbJtDY_EEgtPN5X5wuEsuivqsDW7vbryXUYNQBM2D1uChm2kr0TTYtzJE5w
📊 Found 15 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: TREBBKcC6YcPe7UDtl1InssjcshYF_NGfKKnoirU_yozCvMo5F0DDZCJNEuSOoO67XzxaKWWCHeEB8oG8bXKCw
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: gy0OinNwg5E03Kgcs4vCQQ9qfNl_oryoXxqeAlbjK8hhu6YSkMp4XGCFZK7LKqOLldstI9P5cYInzxLGq1HaaQ
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {'max_score': 100, 'count': 15, 'avg_score': 60.0, 'status': 'active'}
   {'count': 15, 'max_score': 90, 'avg_score': 50.0, 'status': 'inactive'}

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
cache:product:2: {'price': 39.989999999999995, 'name': 'Product 2'}
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
Collection created with first record: sTI05o4W8wVVR-rEAd_0YNAHF6IHymkRlWG4CVq1PdT-FdzGjuCfWUtcmvJMozcT6KFoz9nRobGDjwvb494P0w

=== List Collections ===
Total collections: 13
Sample collections: ['ws_ttl_test', 'audit__ek0_testing', 'products', 'batch_users', 'ttl_cache']

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
Created Alice: $1000 - ID: jVVWBZLCMriVKKacyR_3nGNj20pFdbAe51WzwM5JrSxI5qQLW_LlKAAmkgUpQ4ZMcwigZgAixGJkIhZGC0ETKQ
Created Bob: $500 - ID: 6IFIeEPFcnpfqdD02dHCF0Xsy5XTPJVICu8xWyWlrb2HeoRKolG0zQkF6fzOSEXOex9fpCa31dHd3fMTKwxRnw

=== Example 1: Begin Transaction ===
Transaction ID: 961cda7b-5bb6-45a8-9445-7e9ca30ce875

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
New transaction: 609dd2ed-e1bc-4b6b-9e9b-6df261d2aa4c
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'type': 'Integer', 'value': 700}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 4A7_JNzYfiGrR8QXYjeS42A-WdWCaWVnqRvarfWKfC-QOL7Zv3aTnkXMWs4er8eXyize-g9_EoJU5e6PcmQ5kw
Created Bob: $500 - ID: LEeI7tJRunOpr58EDQfjTlCXMUlp-f4WaP6fb7fojyjisoW7JIqp8jwk1jiQtA44-NaI-yn5XtrI-7WsEFDyew

=== Example 1: Begin Transaction ===
Transaction ID: 3c961792-bd5d-4d48-baec-a1c1ebfc77c8

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


🔄 Switching to NON-DURABLE mode...
   ✓ Config updated: durable_operations=false


============================================================
🔥 TEST 2: Non-Durable Mode (durable=false)
============================================================

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: Xa1ctCz6MhBGppd_WsKuggaZV1usmKQes_R-bOJS_LCTt63t2XX0fVmqsvEJZIooFyae35rnYdqJoRPHYiP23g
Created Bob: $500 - ID: mbTQlOYHcmFygtzsmPsmZYWXPnvhaudPhKe25bIl6FxfiA6BnJMgRFLV5-08mnUaVl660jrqJV3gw6_UvhFEMg

=== Example 1: Begin Transaction ===
Transaction ID: ab8da08f-cfd0-44a0-9841-90160db309ce

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
New transaction: 346efe27-8ba8-4075-afd2-a18ca31dcecf
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'value': 700, 'type': 'Integer'}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: V71RoGUOU3q5R6quQ-eeFOigPJm-bMJpyl-Brss4DSgJ4eBe3cU9JIXOEqwdQwLiNaKu0U_IT22jR0KoZ13B6w
Created Bob: $500 - ID: ktPFxVDpE24E1_dJFZeby_1HPFtySSBz5glaYEIbTINq37EP7NRiHlIaS7j7VVdVY5XQ4MjIgOo0PHLAEWnSeQ

=== Example 1: Begin Transaction ===
Transaction ID: 83d021f5-8a26-4e49-8355-96677f3e5ace

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


🔄 Switching to DURABLE mode...
   ✓ Config updated: durable_operations=true


============================================================
🔥 TEST 3: Durable Mode (durable=true)
============================================================

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: HvZU5vEr49SXA_fx_HxmIte6pNv9yaRRRvMsaMDrCFNCy-rFCUui5tqdinJ0_p51fsa_naEVcfHJwoZNyy59Sg
Created Bob: $500 - ID: bOGCBjghiaLRZSU_tXB7PfFkPbNZ8G4R-jFCIOjkjseZyiPFVAI4F7Y_zNKdGl8WTw1tYlq6rzhkaHh_dfbylw

=== Example 1: Begin Transaction ===
Transaction ID: c81c1940-8b24-4458-9142-00edd3f6722a

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
New transaction: 5befadfc-34f3-44f8-9a99-fe51f7d585c5
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'value': 700, 'type': 'Integer'}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: IwgMEIvRHV8G65ZCnhVBMywZwWmIuixRbsnSIS1j7tHuD57po5s7YOJCc9k5czvLOgPkDuQ4JCllwv9_WRO3pg
Created Bob: $500 - ID: lJOleciyM-ol0rMbIwawnlFmVUhPKwN6Xxt8cpl761XDOiqDIm7iA8sbOvjYW3rWzOTBVdRI11f05fhKlmWo7w

=== Example 1: Begin Transaction ===
Transaction ID: 9488d70d-1e2b-4c7d-ba16-651b717c7819

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
   ✅ Script saved: nXbOVRT8lht-8eI-3opmAw9N-X26M8sui6PelJdskJVdDregEQ-qOzdFRwvRlAQlnFgshiMws-h_L1Xfa0fWeQ

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: 8hpsxUcGZJodOf-wx_2jInvYvobY2QJQi7lF7cc8xshVl5e-h7-8UoGnsFinZlEeCrulRyGDS60Qq-iiiqY76w
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}
   📋 Email: {'type': 'String', 'value': 'alice@example.com'}
   📋 Status: {'type': 'String', 'value': 'pending'}
   📋 Credits: {'type': 'Integer', 'value': 0}

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: wNd3YU9v4U0k_hOdbQIch4dwwEV6DzNoPu8UVbIs3G2rjSQaHEkv84bGRA10uFmE66cuWYots6DXFK2QZleZ2Q

2️⃣ Calling Script (Query + Update + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Status updated to: {'type': 'String', 'value': 'active'}
   📋 Name: {'value': 'Alice Smith', 'type': 'String'}

============================================================
📝 Script 3: Query + Update Credits + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: -5-UmGAMbVwxjDErlXukUsAjUpqvJ_-A2cTO7sPY1zqjV16hnzZXWMkSdnSRO1ssXHfUrkWPUFlo7LbeKNWQBw

2️⃣ Calling Script (Query + Update Credits + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {'value': 0, 'type': 'Integer'}
   📋 Status: {'type': 'String', 'value': 'active'}
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}

============================================================
📝 Script 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: 7OBTuCtfIVyhtj4CyKfc4EV0LAR8nRf1fSvZ7vd6JnGmKyc2dGNriTlXBMoW7CWR56lweMpNsJ3kz1XiPnqC9g

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: nXbOVRT8lht-8eI-3opm...
   ✅ Deleted script: wNd3YU9v4U0k_hOdbQIc...
   ✅ Deleted script: -5-UmGAMbVwxjDErlXuk...
   ✅ Deleted script: 7OBTuCtfIVyhtj4CyKfc...
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
  Output: Document ID = kEeFFLcFgwmSKvCNJu-42MnMrlOJgQWBY_DZdRdosfwvSvYKwPpxnBRiMPR-etoUMYich7JxoPbFuR85segr3Q
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: find_by_id(kEeFFLcFgwmSKvCNJu-42MnMrlOJgQWBY_DZdRdosfwvSvYKwPpxnBRiMPR-etoUMYich7JxoPbFuR85segr3Q)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: find_by_id(kEeFFLcFgwmSKvCNJu-42MnMrlOJgQWBY_DZdRdosfwvSvYKwPpxnBRiMPR-etoUMYich7JxoPbFuR85segr3Q)
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
  Output: Document ID = rzyLOvjJ5Mpv18hsDje9qbZl3ApZyYSHEeg7O_w_AttXlCHukYP_6ERFZ4TGtx018Y0xIIYaJbNsZ5R6qzyPSw
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: find_by_id(rzyLOvjJ5Mpv18hsDje9qbZl3ApZyYSHEeg7O_w_AttXlCHukYP_6ERFZ4TGtx018Y0xIIYaJbNsZ5R6qzyPSw)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: find_by_id(rzyLOvjJ5Mpv18hsDje9qbZl3ApZyYSHEeg7O_w_AttXlCHukYP_6ERFZ4TGtx018Y0xIIYaJbNsZ5R6qzyPSw)
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
    Finished `release` profile [optimized] target(s) in 0.09s
📦 Built wheel for abi3 Python ≥ 3.8 to /Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.15.1-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel for current platform...[0m
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.15.1-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.15.1
    Uninstalling ekodb_client-0.15.1:
      Successfully uninstalled ekodb_client-0.15.1
Successfully installed ekodb-client-0.15.1
✅ [32mPython client package built and installed![0m
🧪 [36mRunning Python client library examples...[0m
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {'id': 'Fwx0q4m7idTsv67GYT9Cw2D-VQ4T1Wi1TEuV6bEOAIz8Y-fuNdTMEYydJp8l_MqKxcTx7475kqdmlWrwNgvrHA'}

=== Find by ID ===
Found: {'value': {'type': 'Integer', 'value': 42}, 'name': {'value': 'Test Record', 'type': 'String'}, 'price': {'type': 'Float', 'value': 99.99}, 'id': 'Fwx0q4m7idTsv67GYT9Cw2D-VQ4T1Wi1TEuV6bEOAIz8Y-fuNdTMEYydJp8l_MqKxcTx7475kqdmlWrwNgvrHA', 'tags': {'type': 'Array', 'value': ['tag1', 'tag2', 'tag3']}, 'user_id': {'value': '550e8400-e29b-41d4-a716-446655440000', 'type': 'String'}, 'data': {'type': 'String', 'value': 'aGVsbG8gd29ybGQ='}, 'metadata': {'value': {'key': 'value', 'nested': {'deep': True}}, 'type': 'Object'}, 'created_at': {'value': '2026-03-28T17:45:45.242501', 'type': 'String'}, 'active': {'type': 'Boolean', 'value': True}, 'embedding': {'type': 'Array', 'value': [0.1, 0.2, 0.3, 0.4, 0.5]}, 'categories': {'type': 'Array', 'value': ['electronics', 'computers']}}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): True
  price (Decimal): 99.99
  created_at (DateTime): 2026-03-28 17:45:45.242501
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): ['tag1', 'tag2', 'tag3']
  metadata (Object): {'key': 'value', 'nested': {'deep': True}}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): ['electronics', 'computers']
  data (Bytes): 11 bytes
Plain record: {'value': 42, 'name': 'Test Record', 'price': 99.99, 'id': 'Fwx0q4m7idTsv67GYT9Cw2D-VQ4T1Wi1TEuV6bEOAIz8Y-fuNdTMEYydJp8l_MqKxcTx7475kqdmlWrwNgvrHA', 'tags': ['tag1', 'tag2', 'tag3'], 'user_id': '550e8400-e29b-41d4-a716-446655440000', 'data': 'aGVsbG8gd29ybGQ=', 'metadata': {'key': 'value', 'nested': {'deep': True}}, 'created_at': '2026-03-28T17:45:45.242501', 'active': True, 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5], 'categories': ['electronics', 'computers']}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'data': {'type': 'String', 'value': 'aGVsbG8gd29ybGQ='}, 'active': {'value': True, 'type': 'Boolean'}, 'metadata': {'type': 'Object', 'value': {'nested': {'deep': True}, 'key': 'value'}}, 'value': {'value': 100, 'type': 'Integer'}, 'user_id': {'type': 'String', 'value': '550e8400-e29b-41d4-a716-446655440000'}, 'embedding': {'value': [0.1, 0.2, 0.3, 0.4, 0.5], 'type': 'Array'}, 'price': {'value': 99.99, 'type': 'Float'}, 'created_at': {'value': '2026-03-28T17:45:45.242501', 'type': 'String'}, 'tags': {'type': 'Array', 'value': ['tag1', 'tag2', 'tag3']}, 'id': 'Fwx0q4m7idTsv67GYT9Cw2D-VQ4T1Wi1TEuV6bEOAIz8Y-fuNdTMEYydJp8l_MqKxcTx7475kqdmlWrwNgvrHA', 'categories': {'value': ['electronics', 'computers'], 'type': 'Array'}, 'name': {'value': 'Updated Record', 'type': 'String'}}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: 2uiRVSdq0YJiHR0nO_ARlSIaQbrxzBLcWyBxz2hYq2kHQ6PQyBiTG4hvPbFx1FxZPoUwIPZB7wHTc76hR1tsOQ

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
Collection created with first record: "rI0GJE__97QGC9056MmhHBXmY4lOmGkhmTCiAWbhUxhRmjO0mMrMK7P1vFWUbYdt7Sc2jo6ZQ6QuX6LQ1dPe5w"

=== List Collections ===
Total collections: 12
Sample collections: ['ws_ttl_test', 'audit__ek0_testing', 'products', 'client_collection_management_python', 'batch_users']

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
  cache:product:2: {'name': 'Product 2', 'price': 39.99}
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
Created Alice: $1000 - ID: o6zolw3BE4TxYYPQySupzilSKdemFZ6ulIa9vdkxTuD_YLcb6o3Y8amX0haWL5xDHecvQ0qDQvDgHZRlRpGcCA
Created Bob: $500 - ID: 85XtuZuw-QZ8Qo-4HoNtIBcordJgjI15vORjeA9F-UwGdgaxBuRLv0R9UqYTh8UU8y1MhTSrPU2SN6xe2IXS-Q

=== Example 1: Begin Transaction ===
Transaction ID: cd53eb68-653c-4344-9cef-7e18d8fd6af5

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: f9375919-9977-4a9a-b28e-0ff0f2e2fdbc
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
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title, bio.value, title.value, bio
  2. Score: 26.400, Matched: title, bio, title.value, bio.value
  3. Score: 26.400, Matched: title, bio, bio.value, title.value
  4. Score: 26.400, Matched: title.value, bio, bio.value, title

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio.value, title.value, bio, title
  2. Score: 39.600, Matched: bio, bio.value, title, title.value
  3. Score: 39.600, Matched: title, bio, title.value, bio.value
  4. Score: 39.600, Matched: title.value, title, bio.value, bio

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 2 results (matches: run, running, runs)
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio.value, bio

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.755
  2. Score: 0.747
  3. Score: 0.740

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.370, Matched: content.value, content, title, title.value
  2. Score: 1.377, Matched: content, content.value, title, title.value
  3. Score: 0.374, Matched: 

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
✓ Inserted document: httLk51BHC3Ea2WvLn6xuKRLa9VcAOvgIymVCaZ11jHBqgFPEWhYAC5W1j3gGMBCQohO7tvZLgpqcYOtLnc44A

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: CElVjWKUB7A7r3RXpuoVh27W8IVFrqGE_uqYDVhXC7cHmd3MiaJQDBOjjgeDz8Bhx9QmCWfnA6BwdhwfuneQ_g

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
✓ Inserted document with TTL: JjpnWFlqcCV6QohVX_E7LcO6WY65uHtrJkkPk9hzPTsP2YOchmJP-pdTmUQ2xNbidXon6hdyJCobArsznXpvOQ

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
✓ Edge cache script created: wjWH3dn6SGZkTaxDq7BcGVeBwma0WNy1d5DHDf_AZvMyOVGgY1BDwDMvbPw6uXT8xXBle4YgUOlw3pLLhiWhOg

Call 1: Cache miss (fetches from API)
Response time: 720ms
Result: {
  "records": [
    {
      "value": {
        "current": {
          "interval": 900,
          "temperature_2m": 5.5,
          "time": "2026-03-28T21:45"
        },
        "current_units": {
          "interval": "seconds",
          "temperature_2m": "\u00b0C",
          "time": "iso8601"
        },
        "elevation": 32.0,
        "generationtime_ms": 0.022292137145996097,
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
Response time: 41ms (17.6x faster!)
Result: {
  "records": [
    {
      "value": {
        "current": {
          "interval": 900,
          "temperature_2m": 5.5,
          "time": "2026-03-28T21:45"
        },
        "current_units": {
          "interval": "seconds",
          "temperature_2m": "\u00b0C",
          "time": "iso8601"
        },
        "elevation": 32.0,
        "generationtime_ms": 0.022292137145996097,
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

✅ Script saved: 9yV8VoV5QPp2xAtsMLjUq0vDNlRnLG0-Kjdw81PtHo_BXUpHdfLMtTD3QJjyLSCvO53S8EZXIV-GMU-Pnw8ePg
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved: oCbxCaUIc3zuKO_T4TV7_JdUG0r3dveGPN0d5A5oiMtBC6fjo1wX6Uh0QOSk1IPDn77XU5PltnHmv-meaTZTow
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: 5HYUlIdaz8K6iYFha37qaIIyJShQmBWE_vw5tx9C1KVUAGSdVpmLRDWlQu_fCRiJTvLTSMjimRX9EdSEFHibZA
📊 Statistics: 2 groups
   {'avg_score': 50.0, 'count': 5, 'status': 'inactive'}

   {'avg_score': 60.0, 'count': 5, 'status': 'active'}

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

✅ Script saved: mNMwvTSpr5uzzTLAD68iMSgQlmiX9CoP0H5CPFV-d_lNE_rY1nucx7CWzs7m8ovgUR8N5aRB0FIfgWQo5DVBKg
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
   {'category': 'Furniture', 'count': 2}
   {'category': 'Electronics', 'count': 3}
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

✅ Inserted order: BGwjya0qcitnfitDCCgVooL6TvKMn7wSB5cSwwfWfpq_hQHnyRpR-2XTMELp73hFA9XFFB4lVLEkEfCNrDiajQ
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

✅ Inserted order: ACZH3fpDknVU0Si8hm1zxiYGQtH_XdYToWlG19pD_s0XdbYmg26la9NUmNHtkuIZroaAWep1ovYWBWEkAV5xog
✅ Cached order status
📊 Quick status lookup: {'value': '{"status":"processing","updated_at":"2026-03-28T21:45:48.142210+00:00"}'}

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
✓ Created SWR script: fetch_api_user_py (9NJolS3KOISlUQbOFDeICvUKuHn7sn7cpEmv-OVSY8enIY4iN-J6MKO_GmOaoVJhyQZRG0dzCPlCHBo4pC8kOA)

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
✓ Created native SWR script: github_user_native (1rpDO0Lx37c1En7K8XlUgK_npX-GypvUvQnE2gRMtwX31AN2ivA82gURD1-JcuJ6mc3hajH_vK-62_wVXtF6tA)

First call (cache miss - will fetch from GitHub API):
  Response time: 123ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 21ms
  Speedup: 5.8x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (G0RaPFc6ZJdQOL1ZnEhRBlFa60Kb2rNjqwZRLohvSczau4po9Buw7v7j6QbQZYlkWGR7svuUWrYUvSOTK9rrQg)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (jji1guiyJ6EUBek4EVtE32rZjl5KWTEbWuRjjMPAw0FR_Dil4nEsLbo_tHUj-bclQWsx_sviESgNpgDp-YzeLA)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (KBe0nfjL3Zv77Mua0A_CiPts51PES6YLbjXKsPF_FlSiKjtiai7omgKWFQVqNPmGkJdiXUszS4LnDtxCheuVHQ)
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
   {'avg_price': 367.0, 'category': 'Electronics', 'count': 5}
   {'avg_price': 365.6666666666667, 'category': 'Furniture', 'count': 3}
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
   Vector databases offer several benefits, including:

1. **Efficient Similarity Search**: They excel at searching for similar items using vector representations, making them ideal for applications like recommendation systems and image retrieval.

2. **High-dimensional Data Handling**: They can efficiently store and query high-dimensional data, which is crucial for machine learning and natural language processing applications.

3. **Scalability**: Many vector databases are designed to scale horizontally, allowing them to handle large volumes of data.

4. **Real-time Search Capabilities**: They often provide fast response times for queries, suitable for real-time applications.

5. **Integration with ML Models**: They can easily integrate with machine learning workflows, facilitating the storage and retrieval of embeddings from models.

6. **Flexibility**: Vector databases can handle diverse data types, such as text, images, and audio, in a single database.

7. **Advanced Indexing Techniques**: They utilize specialized indexing structures like HNSW and Annoy to improve search speed and efficiency.

8. **Support for Vector Operations**: Many offer built-in support for operations such as vector arithmetic, making complex queries more straightforward.

These features make vector databases particularly valuable in AI and data-intensive applications.
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
   active: 7 users
   inactive: 3 users
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
   1. Vector Databases Explained (Database)
   2. Introduction to Machine Learning (AI)
   3. Database Design Principles (Database)
   4. Getting Started with ekoDB (Database)
   5. Natural Language Processing (AI)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Script saved
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
✓ Created session: oSTbHI3HIfQPb12V5a71f57bJiy6odGOwleAbeB35d0aLyVVKHQDLIJaJDqfBEi770Qav9ZcpgPyr9k7AywD8A

=== Sending Chat Message ===
Message ID: jd1CrAdIVspDwjvTtC0XS14vV4M2rfvCoy7VxkLg_FBEyW5vhc7AhsetFZHQEvo8hGGbG6zoCD0utiCMHeuRBg

=== AI Response ===
Here are the available products and their prices:

1. **ekoDB Cloud**
   - Description: Fully managed cloud database service product
   - Price: $499

2. **ekoDB**
   - Description: A high-performance database product with AI capabilities
   - Price: $99

3. **ekoDB Pro**
   - Description: Enterprise edition product with advanced features
   - Price: $299

Execution Time: 2245ms

=== Token Usage ===
Prompt tokens: 1298
Completion tokens: 89
Total tokens: 1387

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: dIbCSxMLcmrPoG3nbNPQW4gjqIZNxPtO6e2UED3r_Pf17GKPr_cvFft2SdYJy_y4l-GB13WWUnNKkoMHX7RLsA

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name**: ekoDB
- **Price**: $99

If you would like more information or additional products, please let me know!

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['token_usage', 'content', 'llm_model', 'updated_at', 'context_snippets', 'llm_provider', 'chat_id', 'created_at', 'id', 'role'])
Debug: First message role: {'value': 'assistant', 'type': 'String'}
=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of ekoDB is $99. If you have any other questions or need further information, feel free to ask!

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: jaaT3Vfrj3B45QAf8_r32GkVnIiUv6kshpVNj2mn7m7EavDP9Lsw_ae_nu16Y5DI3ZQ8BiHlV3Q4VbHiytX-NA
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
✓ Created session: wyQf1deFhiTVT7AKiYX2cuj-4smraxMHlwSLVsrzIw6WFtPiNoGXeanCmQWjcPo4vL7dkeGFNljM_OAd9RqdgQ

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: A high-performance database product
- **Price**: $99 

If you need more information or further assistance, feel free to ask!

✓ Message 2 sent
  Response: The price of ekoDB is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: l6R5rQcWaI_JzS252WqlnguXi418Kost6-_MeDwXsdZnkp9yzaNCGO2TFbR7tJoRL3koCveZNEyC8mpkGbKBOw
  Parent: wyQf1deFhiTVT7AKiYX2cuj-4smraxMHlwSLVsrzIw6WFtPiNoGXeanCmQWjcPo4vL7dkeGFNljM_OAd9RqdgQ

=== Listing Sessions ===
✓ Found 7 sessions
  Session 1: l6R5rQcWaI_JzS252WqlnguXi418Kost6-_MeDwXsdZnkp9yzaNCGO2TFbR7tJoRL3koCveZNEyC8mpkGbKBOw (Untitled)
  Session 2: wyQf1deFhiTVT7AKiYX2cuj-4smraxMHlwSLVsrzIw6WFtPiNoGXeanCmQWjcPo4vL7dkeGFNljM_OAd9RqdgQ (Untitled)
  Session 3: jaaT3Vfrj3B45QAf8_r32GkVnIiUv6kshpVNj2mn7m7EavDP9Lsw_ae_nu16Y5DI3ZQ8BiHlV3Q4VbHiytX-NA (Untitled)
  Session 4: oSTbHI3HIfQPb12V5a71f57bJiy6odGOwleAbeB35d0aLyVVKHQDLIJaJDqfBEi770Qav9ZcpgPyr9k7AywD8A (Untitled)
  Session 5: gyNEy24WI3QCrsiEhkrIDESRQGTfXjb8K19yYzbV1N5bBqzVbySN-E2I81QMuassG-3hZTEl13vLPX-FKSHxAQ (Untitled)
  Session 6: dtK53BYTkysAiNRMWldOmIovwo_wnvgyHbNFMIfz9_pCa33c3eD0iQORSq9S1c2pWDBc9IrosRYBY20zvb8qmg (Untitled)
  Session 7: Rfr0sJJyydcNjx_m5eGjQmvOoovWjppLeco4coWFC92v51wT_xTauDziN2zIy1rsoZA5Hm_iGIXay4z-Fzbjww (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: l6R5rQcWaI_JzS252WqlnguXi418Kost6-_MeDwXsdZnkp9yzaNCGO2TFbR7tJoRL3koCveZNEyC8mpkGbKBOw

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Dict Creation ===
✓ Created record with plain dict: {'id': 'OIsNQ4WzMpKYiUHJXY139bxFTIcNsNhqC1B8hv3e9i0zAWIR9CANmZBkHNymleZ74ies_i3m3p4Ows9Xcyi_Zg'}

=== Upsert Operation ===
✓ Upsert (update existing record): OIsNQ4WzMpKYiUHJXY139bxFTIcNsNhqC1B8hv3e9i0zAWIR9CANmZBkHNymleZ74ies_i3m3p4Ows9Xcyi_Zg
✓ Inserted second record: x3X_DbtCSJ22f83y_6PK9XSlA8Uwb9E0WrNL0m3mtkZQ6H5J1_gNE7zrXqII8BKHk_029-LM_iuD23cjssCNXg
✓ Upsert (update second record): x3X_DbtCSJ22f83y_6PK9XSlA8Uwb9E0WrNL0m3mtkZQ6H5J1_gNE7zrXqII8BKHk_029-LM_iuD23cjssCNXg

=== Find One Operation ===
✓ Found user by email: {'id': 'OIsNQ4WzMpKYiUHJXY139bxFTIcNsNhqC1B8hv3e9i0zAWIR9CANmZBkHNymleZ74ies_i3m3p4Ows9Xcyi_Zg', 'email': {'value': 'alice.j@newdomain.com', 'type': 'String'}, 'name': {'value': 'Alice Johnson', 'type': 'String'}, 'age': {'type': 'Integer', 'value': 29}, 'active': {'type': 'Boolean', 'value': True}}
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
   Inserted with ripple: {'id': 'VUZ3qg7qw4nMfyCgWMiZrxrr_y-PGPm0N_OIr-c2leqzPYNpaK1rknty7tVi9NPKmiiWyx6ILR2uPwETVRXf1Q'}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {'id': '8WKIJZZKmtJ6QaKv7beRcNMI4_rDZNiNStMPUfo8fIGijd8PqwZgIapw4VEJgcZK_wDV6h7-3iXdNqZHPe7NDQ'}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {'price': {'type': 'Integer', 'value': 150}, 'id': 'VUZ3qg7qw4nMfyCgWMiZrxrr_y-PGPm0N_OIr-c2leqzPYNpaK1rknty7tVi9NPKmiiWyx6ILR2uPwETVRXf1Q', 'name': {'value': 'Product 1', 'type': 'String'}}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {'id': 'VUZ3qg7qw4nMfyCgWMiZrxrr_y-PGPm0N_OIr-c2leqzPYNpaK1rknty7tVi9NPKmiiWyx6ILR2uPwETVRXf1Q', 'price': {'value': 500, 'type': 'Integer'}, 'name': {'type': 'String', 'value': 'Upsert Product'}}

✅ All bypass_ripple operations completed successfully!
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields (id, name, email only)
  Found 3 active users
  Fields returned: ['email', 'id', 'name']
  First user: {'type': 'String', 'value': 'Alice Johnson'} <{'type': 'String', 'value': 'alice@example.com'}>

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ['avatar_url', 'email', 'user_role', 'status', 'age', 'bio', 'id', 'created_at', 'name']

Example 3: Complex query with projection (active users, ages 18-65)
  Found 3 active users (ages 18-65)
    - {'type': 'String', 'value': 'Dave Brown'} (age {'type': 'Integer', 'value': 45})
    - {'value': 'Alice Johnson', 'type': 'String'} (age {'value': 30, 'type': 'Integer'})
    - {'type': 'String', 'value': 'Bob Smith'} (age {'type': 'Integer', 'value': 25})

Example 4: Query inactive users with profile fields
  Found 1 inactive users
    - {'value': 'Carol White', 'type': 'String'}: {'value': 'Manager', 'type': 'String'}

Example 5: Compare full vs projected data
  Full query:
    - 12 fields per record
    - Fields: ['password', 'created_at', 'age', 'id', 'name', 'email', 'avatar_url', 'user_role', 'api_key', 'bio', 'status', 'secret_token']
  Projected query:
    - 3 fields per record
    - Fields: ['email', 'id', 'name']
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
Created user function with ID: 6fuHOb3-Z2HNrrxGlaj_bBTtENotSBmia2tdBvBKEUvLQaZWp8b01kcgJuo9vUI13i7lilROqo-9QvyZBLeO9A

=== Get User Function ===
Retrieved: get_active_users_py - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 25 user functions:
  - get_active_users: Get Active Users (Updated)
  - get_high_scoring_active_users: Get High Scoring Active Users
  - cache_api_call_py: Cache External API Call
  - get_active_users_paginated: Get Active Users (Paginated)
  - get_user_wrapper: Wrapper that calls fetch_user
  - get_user_wrapper: Wrapper that calls fetch_user
  - get_active_users_updated: Get Active Users (Updated)
  - fetch_user: Fetch user by code
  - validate_user: Check if user exists
  - get_active_users_paginated: Get Active Users (Paginated)
  - get_users_by_status: Get Users By Status
  - get_verified_user: Get verified and validated user
  - get_verified_user: Get verified and validated user
  - get_active_users: Get Active Users (Updated)
  - fetch_slim_user: Validate and slim down user
  - fetch_and_store_user: Fetch user from API and cache in KV
  - swr_user: SWR pattern for user data (KV-based)
  - fetch_user: Fetch user by code
  - swr_user: SWR pattern for user data (KV-based)
  - validate_user: Check if user exists
  - get_active_users_updated: Get Active Users (Updated)
  - get_active_users_py: Get Active Users
  - fetch_and_store_user: Fetch user from API and cache in KV
  - get_users_by_status: Get Users By Status
  - fetch_slim_user: Validate and slim down user

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
