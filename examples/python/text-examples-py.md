make test-examples-python
🧪 [36mRunning Python examples (direct HTTP/WebSocket)...[0m
=== Simple CRUD Operations (Direct HTTP) ===

✓ Authentication successful

=== Insert Document ===
Inserted: {'id': '0LeP6Wu_ZUtM4MWYfzKqH7_vFaKfmQs76e7r2jyxNnmZ6b5sSyl7KEuNbNLRYEjKjaZSsahzkFW4jpSp3OUS6A'}

=== Find by ID ===
Found: {'id': '0LeP6Wu_ZUtM4MWYfzKqH7_vFaKfmQs76e7r2jyxNnmZ6b5sSyl7KEuNbNLRYEjKjaZSsahzkFW4jpSp3OUS6A', 'name': 'Test Record', 'value': 42, 'active': True}

=== Find with Query ===
Found documents: [{'active': {'type': 'Boolean', 'value': True}, 'name': {'value': 'Test Record', 'type': 'String'}, 'value': {'type': 'Integer', 'value': 42}, 'id': '0LeP6Wu_ZUtM4MWYfzKqH7_vFaKfmQs76e7r2jyxNnmZ6b5sSyl7KEuNbNLRYEjKjaZSsahzkFW4jpSp3OUS6A'}]

=== Update Document ===
Updated: {'id': '0LeP6Wu_ZUtM4MWYfzKqH7_vFaKfmQs76e7r2jyxNnmZ6b5sSyl7KEuNbNLRYEjKjaZSsahzkFW4jpSp3OUS6A', 'name': {'value': 'Updated Record', 'type': 'String'}, 'active': {'value': True, 'type': 'Boolean'}, 'value': {'value': 100, 'type': 'Integer'}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: SNyEc7R-CZkxYBn4G05Ra_eW5cYKuJXomLyBtWiFJVcI8VEUsZ4LRygII3U0d05JOVnjULyCL6HURn3kXafJ2Q

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
        "id": "SNyEc7R-CZkxYBn4G05Ra_eW5cYKuJXomLyBtWiFJVcI8VEUsZ4LRygII3U0d05JOVnjULyCL6HURn3kXafJ2Q",
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
        "id": "UTmrXQDM_xk24AvjXafv75Ex54tPoRRcsv09e6t_5bW_U4BC3Vkjl5mrH9x0OXidPmHqBPVf2zcWdZvSCusrYw",
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

✅ Script saved: IfdbY0M3b9Z-5weIe2jJO0WQ8fig6gVzIssv7bRNOpQaj1cW2Ei3Va-KJ_j4RL7_7vIqFgPEgf5HqdvRgxrgDA
📊 Found 35 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: QFOxlRYeXd6FrtG_b0cB2LHia9fxbMMl6CraS5k2ForzuqYF5kjG1QyXm65CTDADY3GsiF44gIV-BpwTtS8WOA
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: tQk_sZ1o-GlEW0g345ZNCUagWTY5Tuh74sf4QHT5AaoIr-n9NMu-HiN-3LvR5CrG1W0LsY2SEojeAqLysKuHfQ
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {'count': 15, 'status': 'active', 'max_score': 100, 'avg_score': 60.0}
   {'avg_score': 50.0, 'count': 15, 'status': 'inactive', 'max_score': 90}

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
Collection created with first record: Dnd0EIjeO8SL0128mUym4wOIZLlaVwA_fZBczLJSNhQF5gX-wKDC7PoNA6-aqBjA2jdDzTzWy9l-kSNe_toEDg

=== List Collections ===
Total collections: 12
Sample collections: ['scripts__ek0_testing', 'test_accounts', 'ws_ttl_test', 'chat_messages__ek0_testing', 'products']

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
Created Alice: $1000 - ID: KOEyh1OmEYfz05HwTf1qBODbmNO57mGDIyRxdCwOS9AMWeRzGPNwe0BcT0XR6YFV6NdREGhWJ6bui8bQIs1A2w
Created Bob: $500 - ID: n4ztcC2hPVsrpEP8EMCpUs8jor8tb6YSd84S6gqTHBr0ztdTEfcuXMZusVUDbF1yK5hadWviMndrhcq5C8oOMQ

=== Example 1: Begin Transaction ===
Transaction ID: 0a847a71-e6d3-479f-a163-d5ffa89555db

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: $800
Bob: $700

=== Example 5: Rollback ===
New transaction: 7090b58f-8d79-4425-b796-57c841a8cd49
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: mD7pOxq6lSgtqL8tNh28dZbI9gYg1nywsv8u1ojFlZBKXHYBO5rmvxFx0qO7nDh6ZF7uO8cexr_-C78KDiNvQA
Created Bob: $500 - ID: 8sr8_VyatLqo_NBH4ndksUOi8hP6uI8OtzAA_Q9F6LiKrKvRzz6juDrCjGWXPq6bOYlnNJKD9K7NAf-0eXBMSA

=== Example 1: Begin Transaction ===
Transaction ID: 2d193eb0-30d5-4c79-a516-5a0cc492b03c

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: $800
Bob: $700

=== Cleanup ===
✓ Deleted test accounts


🔄 Switching to NON-DURABLE mode...
   ✓ Config updated: durable_operations=false


============================================================
🔥 TEST 2: Non-Durable Mode (durable=false)
============================================================

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: tvuzx2NQ4meSnUcWGRKW6MSPmQcbw3mUTqMn9Y1SQUyfkj2Tr71ssudYsI1Pb2nUimXJmkeDTrjAHVVoympcPw
Created Bob: $500 - ID: u-WAzCC8oDhP3LQ4o4bq40mi3DfuzBDtSNA8fPrdAFU7KWVdqEmBJeXm8kN7rLFSyozi3R3-0BFwywYn17NuDg

=== Example 1: Begin Transaction ===
Transaction ID: 937cd77c-6dd9-4097-853a-f80694ab76b7

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: $800
Bob: $700

=== Example 5: Rollback ===
New transaction: 12a7b4dd-691f-4af8-92a1-5770542a5aa6
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 2erqmO6EiAS8kfpOtE3mlCLb--3mx2rzQk-divVSbZJDF-bW_75Nd5g8pYT4q8_Se_4HUKpWydVb6vwUJVTl4g
Created Bob: $500 - ID: Rl_aNC1kN3cWNlpPwl7RP1YicYYSHwxpWwz9UsEUa-4kRvp-yc3zXoRy3HD1fpu0Vp25P3OTqArkbtbd7mp5Eg

=== Example 1: Begin Transaction ===
Transaction ID: 210051cc-4304-4051-b88a-22999159e93c

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: $800
Bob: $700

=== Cleanup ===
✓ Deleted test accounts


🔄 Switching to DURABLE mode...
   ✓ Config updated: durable_operations=true


============================================================
🔥 TEST 3: Durable Mode (durable=true)
============================================================

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: BiNq3ZdgSSu_XMKLeSShHIl0ioVEDBONdG6DF3GnyI4C63ynYp7ib3KKbfFWtPIPkGIsWSoYYU00fiirXUWRZQ
Created Bob: $500 - ID: KLuj3qBkmCh_uug_4Hxc4T6mwztELBC995gCJkapFYzzcpL9s_fuabDhAjsExzBVHyPx6DAtVxByucou97GsGw

=== Example 1: Begin Transaction ===
Transaction ID: 8c317434-41bf-4935-9dee-2bce91bad430

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: $800
Bob: $700

=== Example 5: Rollback ===
New transaction: 96b3a035-a7c2-42a0-88f0-1c49641afad3
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: R-RlgwG5Z7izuhCyMhaKi3HvyryGtUdYHppV6WAggof8tRKNnMXmQJMNFomUYMHinfzA7dDE63mZ-jb7YDu8ZA
Created Bob: $500 - ID: bQTR3qZYT5LFUKL9TsxXhFpCqWso5wRmICYo5ciXlXCouZqpUxKqmxxrj2zBvBWDXNpWXofmBYlduc5_4U93Fg

=== Example 1: Begin Transaction ===
Transaction ID: efe5f27e-c9d6-4265-9951-b0c014e3263c

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: $800
Bob: $700

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
   ✅ Script saved: 1iYQ5HG1HxuMOHrbAnOtjLgfVPlH5D1vCafdgDnO0FrlxxsouSuxcNWzNs9QOgA7IPYuUd1_wcAmB5wTCPkIXw

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: IJ6NbCVAmcKvGUWjcqnfMrkEUH70VNw6gspTPybEwHP3b9lUcWajraEUKNgmfISOFhMW3Itn3iGbvmKuObIMSA
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}
   📋 Email: {'type': 'String', 'value': 'alice@example.com'}
   📋 Status: {'value': 'pending', 'type': 'String'}
   📋 Credits: {'type': 'Integer', 'value': 0}

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: 6MiQ_qdSY1ZhLoAa9-LpOk-R6Rtm8d9Mh6NAQFMyzoKC45JkcWA2AThfjSlmfjHPhSnM-tvIDeRs_yjhRm8Pag

2️⃣ Calling Script (Query + Update + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Status updated to: {'value': 'active', 'type': 'String'}
   📋 Name: {'value': 'Alice Smith', 'type': 'String'}

============================================================
📝 Script 3: Query + Update Credits + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: cBIkwWmd_Tkxn_q2Qnhi8_GZs8eTitSckIYUdOk1AwSeUh1z9sLPVb6JAosnQgTs2MOt0kzDu7lYn9vNeubVEA

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
   ✅ Script saved: 8cbxatYV2odmoouCU6VSaR33DTnKGIGvBJJyN23l4yXcYs9MClBAyb5_P0cpiLRXOChTpaWLlBDFvoHB4KRbYQ

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: 1iYQ5HG1HxuMOHrbAnOt...
   ✅ Deleted script: 6MiQ_qdSY1ZhLoAa9-Lp...
   ✅ Deleted script: cBIkwWmd_Tkxn_q2Qnhi...
   ✅ Deleted script: 8cbxatYV2odmoouCU6VS...
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
  Output: Document ID = qh9gGe2iHdjAAHQR4KqPc1RdUzYwXKVsAt-dyH5g4dF8RKNlqlsC78xA-Spd64kf78cx39j50tYz2zMytUcwgQ
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: find_by_id(qh9gGe2iHdjAAHQR4KqPc1RdUzYwXKVsAt-dyH5g4dF8RKNlqlsC78xA-Spd64kf78cx39j50tYz2zMytUcwgQ)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: find_by_id(qh9gGe2iHdjAAHQR4KqPc1RdUzYwXKVsAt-dyH5g4dF8RKNlqlsC78xA-Spd64kf78cx39j50tYz2zMytUcwgQ)
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
  Output: Document ID = QxOeDurf03NtrP2VOr-ERTyf5Pq_slRNfEex83OvdYgRc6r9w1_SDNWrHx2ewcpKW5XcHa-HrXOypVO2SZSYhQ
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: find_by_id(QxOeDurf03NtrP2VOr-ERTyf5Pq_slRNfEex83OvdYgRc6r9w1_SDNWrHx2ewcpKW5XcHa-HrXOypVO2SZSYhQ)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: find_by_id(QxOeDurf03NtrP2VOr-ERTyf5Pq_slRNfEex83OvdYgRc6r9w1_SDNWrHx2ewcpKW5XcHa-HrXOypVO2SZSYhQ)
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
🐍 Not using a specific python interpreter
💻 Using `MACOSX_DEPLOYMENT_TARGET=11.0` for aarch64-apple-darwin by default
    Finished `release` profile [optimized] target(s) in 0.13s
📦 Built wheel for abi3 Python ≥ 3.8 to /Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.8.0-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel for current platform...[0m
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.8.0-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.8.0
    Uninstalling ekodb_client-0.8.0:
      Successfully uninstalled ekodb_client-0.8.0
Successfully installed ekodb-client-0.8.0
✅ [32mPython client package built and installed![0m
🧪 [36mRunning Python client library examples...[0m
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {'id': '9TcB67PPsLBVpsEgZHJUMoXo_kjZ1g0dVa8BTkYh9z-byEtvUKRp8zac6oErbVPrp3b8Qbh_VUqqcuv3U1NcHg'}

=== Find by ID ===
Found: {'metadata': {'key': 'value', 'nested': {'deep': True}}, 'user_id': '550e8400-e29b-41d4-a716-446655440000', 'value': 42, 'id': '9TcB67PPsLBVpsEgZHJUMoXo_kjZ1g0dVa8BTkYh9z-byEtvUKRp8zac6oErbVPrp3b8Qbh_VUqqcuv3U1NcHg', 'data': 'aGVsbG8gd29ybGQ=', 'created_at': '2026-01-26T19:55:27.023627', 'name': 'Test Record', 'price': 99.99, 'categories': ['electronics', 'computers'], 'tags': ['tag1', 'tag2', 'tag3'], 'active': True, 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5]}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): True
  price (Decimal): 99.99
  created_at (DateTime): 2026-01-26 19:55:27.023627
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): ['tag1', 'tag2', 'tag3']
  metadata (Object): {'key': 'value', 'nested': {'deep': True}}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): ['electronics', 'computers']
  data (Bytes): 11 bytes
Plain record: {'metadata': {'key': 'value', 'nested': {'deep': True}}, 'user_id': '550e8400-e29b-41d4-a716-446655440000', 'value': 42, 'id': '9TcB67PPsLBVpsEgZHJUMoXo_kjZ1g0dVa8BTkYh9z-byEtvUKRp8zac6oErbVPrp3b8Qbh_VUqqcuv3U1NcHg', 'data': 'aGVsbG8gd29ybGQ=', 'created_at': '2026-01-26T19:55:27.023627', 'name': 'Test Record', 'price': 99.99, 'categories': ['electronics', 'computers'], 'tags': ['tag1', 'tag2', 'tag3'], 'active': True, 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5]}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'user_id': {'type': 'String', 'value': '550e8400-e29b-41d4-a716-446655440000'}, 'value': {'type': 'Integer', 'value': 100}, 'tags': {'type': 'Array', 'value': ['tag1', 'tag2', 'tag3']}, 'created_at': {'type': 'String', 'value': '2026-01-26T19:55:27.023627'}, 'id': '9TcB67PPsLBVpsEgZHJUMoXo_kjZ1g0dVa8BTkYh9z-byEtvUKRp8zac6oErbVPrp3b8Qbh_VUqqcuv3U1NcHg', 'embedding': {'type': 'Array', 'value': [0.1, 0.2, 0.3, 0.4, 0.5]}, 'categories': {'type': 'Array', 'value': ['electronics', 'computers']}, 'metadata': {'type': 'Object', 'value': {'key': 'value', 'nested': {'deep': True}}}, 'data': {'value': 'aGVsbG8gd29ybGQ=', 'type': 'String'}, 'active': {'value': True, 'type': 'Boolean'}, 'name': {'value': 'Updated Record', 'type': 'String'}, 'price': {'value': 99.99, 'type': 'Float'}}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: aB6ACHAkpllG6X4MGi5AzfcQuG2at-VeXnERUpTkJKzLBqCqFIdQOweG4VX5XivuoJL92UL_KNFYIIvg5HWTIA

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
Collection created with first record: "7TIJjB-6fLQN28IPhC5Tit-5uJeWTg8oTs7m9ZGsK2MAk-JTaR0-KQY7qcmHBKj81sD1eVzGhWnXcwi-5WlqtQ"

=== List Collections ===
Total collections: 11
Sample collections: ['scripts__ek0_testing', 'test_accounts', 'ws_ttl_test', 'chat_messages__ek0_testing', 'products']

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
Created Alice: $1000 - ID: Lc0h2hSHULBy1Bg-y1aejojx3jVLQx82otG-SAwdMHRsvNAqSAyGVWJLTgXYf7sXuUhBpXP2q3U8-BcmHjIndg
Created Bob: $500 - ID: 5_XGH5QTmBziudBRFKhfqAjurwCzlFIt2UCEsofEqVRCR1oNxSTa6HTHO5fUrka1ZMzPDHko1w3RNSiJbt6ojA

=== Example 1: Begin Transaction ===
Transaction ID: 98ad393f-647b-46a3-a536-a2c176be6963

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: b3ed75ec-9993-45bd-9eff-8584bcb0cfd4
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
  1. Score: 25.740, Matched: email.value, name, email, name.value
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title.value, title, bio, bio.value
  2. Score: 26.400, Matched: bio, bio.value, title, title.value
  3. Score: 26.400, Matched: title, bio, bio.value, title.value
  4. Score: 26.400, Matched: title.value, bio.value, title, bio

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title.value, bio, title, bio.value
  2. Score: 39.600, Matched: bio, bio.value, title, title.value
  3. Score: 39.600, Matched: title.value, bio, title, bio.value
  4. Score: 39.600, Matched: title, title.value, bio.value, bio

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 2 results (matches: run, running, runs)
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio.value, bio

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.762
  2. Score: 0.756
  3. Score: 0.748

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.381, Matched: title, content.value, content, title.value
  2. Score: 1.374, Matched: title, title.value, content, content.value
  3. Score: 0.378, Matched: 

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
✓ Inserted document: Ea51zEIIyXck6u5-RH9AQuMnLuOBQHG8uX1rpbGZRPlV6a3hiRYhD7XIaNEB80Uk_X466OUA13GkElVbXC5Vlg

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: K8Xmwkf4UcqFkYxStIDvgB2lmwJwxqcPu9b0WLPtK72d_UNmEbO7sYosIEAqqtwrchNGuNDb9zb8oJhnOCgAMQ

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
✓ Inserted document with TTL: b9ATjfSLdnmm65wjyoPyGl3-CreV02jbPnKuYE8keRB9-R9p3hENWnEvTCKrd9dta4Mp6yafSulVbyLz2IlYsg

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
✓ Edge cache script created: XVpEvrPid1t6vGjAlpO_R_9Y57lXxVx3hcLePvjPkjmsa2AUW-BOwksbE8z0Onqz5PA-n3--W0J_vB__ndNV0A

Call 1: Cache miss (fetches from API)
Response time: 468ms
Result: {
  "records": [
    {
      "value": {
        "current": {
          "interval": 900,
          "temperature_2m": -9.3,
          "time": "2026-01-27T00:45"
        },
        "current_units": {
          "interval": "seconds",
          "temperature_2m": "\u00b0C",
          "time": "iso8601"
        },
        "elevation": 32.0,
        "generationtime_ms": 0.025033950805664062,
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
Response time: 21ms (22.5x faster!)
Result: {
  "records": [
    {
      "value": {
        "current": {
          "interval": 900,
          "temperature_2m": -9.3,
          "time": "2026-01-27T00:45"
        },
        "current_units": {
          "interval": "seconds",
          "temperature_2m": "\u00b0C",
          "time": "iso8601"
        },
        "elevation": 32.0,
        "generationtime_ms": 0.025033950805664062,
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

✅ Script saved: 4M_-ok1UagS6bSh8Y_gU_iAXX09T98Ifw694Xp4QoVq4LAdL-QlbaV8CI50i4j4CbFNqMim_uG67wV_TXZbaTQ
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved: N9PQcamLTa3si63KFbcT0fTmqwEy7CCkEy6Vtq992xzx2uFgLQ-j2lygW8LGuI0frA6fGyKvSx5iO4QJjOmLSQ
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: Z6XdGWUYOTrELbsKSmW3fqZvq7FggAQ6B9o4geBBtBR2f02PxAhB18JyH_nv0zsCozu7VgoJTZQwVPqio0fGzg
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
   ⏱️  Duration: 2.0ms
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
   ⏱️  Duration: 2.1ms
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

✅ Script saved: a1FOb56TLXK6J8Yl5YEIrEah8rhJWRmUQ-SANvlWwXb_v_owbZPcS8uLjI79NQuWyBE6WlKpMgbiarOBmlUp7Q
📊 Found 2 product groups
   {'avg_price': 474.0, 'category': 'Furniture', 'count': 2}
   {'avg_price': 575.6666666666666, 'category': 'Electronics', 'count': 3}
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

✅ Inserted order: HXHc2HCWvynsQj0ejOz2vhQ6XK_tT7ir5oDmrP5izvaj_6VsIpn9OHzu1XmpFk2cQUueOCs1l8qmkdA7W7gWfw
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
🗑️  Deleted session: None

📝 Example 4: KV Pattern Query

✅ Set 4 config entries
📊 Found 3 app config entries
📊 Found 4 total config entries

📝 Example 5: Combined Wrapped Types + KV Usage

✅ Inserted order: WRbDK7uDlE5nG7wt-49ZGJ8LlJGrt8wd3R8ZF6DCFkiFzIIpyHPQa3VYROcn1Qgk7YOSqhwXUOKP1ZXPpEmh2w
✅ Cached order status
📊 Quick status lookup: {'value': '{"status":"processing","updated_at":"2026-01-27T00:55:29.410274+00:00"}'}

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
✓ Created SWR script: fetch_api_user_py (9AKKnAN62req5fMBiZdgk-3OMdR7yTk5yOWvF22ZFEuiugXvBvA4ZkG_HeaPA3kppnWjyAiiR-OzE1eSIZb7tg)

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
Response time: 3ms (served from cache)
✓ Lightning fast cache hit

🧹 Cleaning up...
✓ Cleanup complete

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation
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
   Vector databases offer benefits such as efficient data storage, precise representation of data points, easy scalability and manipulability, and support for complex geospatial operations. They also provide superior quality in representing data like contours and boundaries.
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
   3. Natural Language Processing (AI)
   4. Vector Databases Explained (Database)
   5. Introduction to Machine Learning (AI)
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
✓ Created session: DIyovkFIAzMyJ-y-TF-R3WXH-CnHmRyC3kFLhWSqFybg7QqLg6KzmNnjWLg4V4xbYSnZ3PbuGBPFJToeMYMbug

=== Sending Chat Message ===
Message ID: TPagY37ugHBsbYiRdNE8XgEoMiLte2pc6GoLOJcOpmBO5cCEtn_B7-SkwKpC8wzReN-_oyDXk0ndRBwHJeUrCQ

=== AI Response ===
There are three products available:

1. ekoDB Cloud: This is a fully managed cloud database service product. The price for this product is $499.

2. ekoDB: This is a high-performance database product with AI capabilities. The price for this product is $99.

3. ekoDB Pro: This is an enterprise edition product with advanced features. The price for this product is $299.

Execution Time: 2958ms

=== Token Usage ===
Prompt tokens: 460
Completion tokens: 84
Total tokens: 544

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: nUd1egSTXF7dCm_gxiTVhTzeN4UeJHnt-sXTNpQS5IwQUbAm5Y0v2J8db-sa__Vq3abGbSCWK_XesrmZMRJ_dw

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information provided, there is a high-performance database product available named "ekoDB". It is priced at $99.

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['context_snippets', 'updated_at', 'role', 'created_at', 'chat_id', 'id', 'token_usage', 'content'])
Debug: First message role: {'type': 'String', 'value': 'assistant'}
=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of ekoDB is $99.

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: vEMkVSZyHWnylKAfzuZhIfSJHFddMGWFbGS5usQHj1mOghiTFrJwKrtlOnm4q7P9cQjYzNv780LUL5XkrObmOg
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
✓ Created session: SMtDTtUsPPGtex-Ojt2Tj_Ve8q93_4P8VE49G97Cqp1WuMODqN66buqX9WkHOJlHD56oTCxFfDjbDht_dAd4xw

=== Sending Messages ===
✓ Message 1 sent
  Response: The product available is "ekoDB". It's a high-performance database product. The price for this product is $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: kXjurymYNOS6aAX07-2NonKnOUi5LC4CiZs6XnW67r832r9HvKH1qIbN38nN-fU5KERag4YwQg-P6IefV8oCjQ
  Parent: SMtDTtUsPPGtex-Ojt2Tj_Ve8q93_4P8VE49G97Cqp1WuMODqN66buqX9WkHOJlHD56oTCxFfDjbDht_dAd4xw

=== Listing Sessions ===
✓ Found 7 sessions
  Session 1: kXjurymYNOS6aAX07-2NonKnOUi5LC4CiZs6XnW67r832r9HvKH1qIbN38nN-fU5KERag4YwQg-P6IefV8oCjQ (Untitled)
  Session 2: SMtDTtUsPPGtex-Ojt2Tj_Ve8q93_4P8VE49G97Cqp1WuMODqN66buqX9WkHOJlHD56oTCxFfDjbDht_dAd4xw (Untitled)
  Session 3: vEMkVSZyHWnylKAfzuZhIfSJHFddMGWFbGS5usQHj1mOghiTFrJwKrtlOnm4q7P9cQjYzNv780LUL5XkrObmOg (Untitled)
  Session 4: DIyovkFIAzMyJ-y-TF-R3WXH-CnHmRyC3kFLhWSqFybg7QqLg6KzmNnjWLg4V4xbYSnZ3PbuGBPFJToeMYMbug (Untitled)
  Session 5: rHeX6XiKBfV6mnPQJ8Q7XI4mrrFMtZxMsSV-HOnSvtoXcSLVkZo32kkj-TuzGpbf3C0XFZD2600RDT1NCFYDnw (Untitled)
  Session 6: r6b9Xtu3fZXerjXX8LMYCAZoqSIvdapmKeLiGUgzMeS_A8z_ox_9W3kWlPiqv9s45hsnouA1hcxjVuZHpEUlPA (Untitled)
  Session 7: iYdwekxmt8ZM7XtMQgOGtgN534O1WtHojnY5RMQQHqz_wUtx3ZoW-2e2CK0xu0VSfw7TaAREHx1sDl_i1nhvcg (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: kXjurymYNOS6aAX07-2NonKnOUi5LC4CiZs6XnW67r832r9HvKH1qIbN38nN-fU5KERag4YwQg-P6IefV8oCjQ

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Dict Creation ===
✓ Created record with plain dict: {'id': 'HeU9iNIzhPdn2md5K_fNb2tkVyB3ypcgLikopn49F5ZU-b6rLWLFWp57apMZrL9slwAIXcoaqLyaoXPYOY5acQ'}

=== Upsert Operation ===
✓ Upsert (update existing record): HeU9iNIzhPdn2md5K_fNb2tkVyB3ypcgLikopn49F5ZU-b6rLWLFWp57apMZrL9slwAIXcoaqLyaoXPYOY5acQ
✓ Inserted second record: DJHsErV-BsyvrReB9ePNCDcTG-1J8j_P9TvWRl6OvrFx3Djk0TJO4Qm-ADgszYyq8yOIRcW-dn4DMk4tATn2sQ
✓ Upsert (update second record): DJHsErV-BsyvrReB9ePNCDcTG-1J8j_P9TvWRl6OvrFx3Djk0TJO4Qm-ADgszYyq8yOIRcW-dn4DMk4tATn2sQ

=== Find One Operation ===
✓ Found user by email: {'name': {'type': 'String', 'value': 'Alice Johnson'}, 'age': {'type': 'Integer', 'value': 29}, 'active': {'value': True, 'type': 'Boolean'}, 'id': 'HeU9iNIzhPdn2md5K_fNb2tkVyB3ypcgLikopn49F5ZU-b6rLWLFWp57apMZrL9slwAIXcoaqLyaoXPYOY5acQ', 'email': {'value': 'alice.j@newdomain.com', 'type': 'String'}}
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
   Inserted with ripple: {'id': '_kxnth5aDMIk1zNtHwUH8V7NxUXASfAw6FJy6EJw6whlmpBAS8nABC-bRpuStrWb2mPf0-Nuaaabbpk90VtodQ'}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {'id': 'SeyJvuHypPubZ1-17zB7jEFzDnxB4zSYX66vfScSWf9dIusaF-gNutqIFNmUU93zcr0Gakt8EoIgtbCt4AiAEA'}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {'price': {'type': 'Integer', 'value': 150}, 'id': '_kxnth5aDMIk1zNtHwUH8V7NxUXASfAw6FJy6EJw6whlmpBAS8nABC-bRpuStrWb2mPf0-Nuaaabbpk90VtodQ', 'name': {'value': 'Product 1', 'type': 'String'}}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {'name': {'value': 'Upsert Product', 'type': 'String'}, 'price': {'type': 'Integer', 'value': 500}, 'id': '_kxnth5aDMIk1zNtHwUH8V7NxUXASfAw6FJy6EJw6whlmpBAS8nABC-bRpuStrWb2mPf0-Nuaaabbpk90VtodQ'}

✅ All bypass_ripple operations completed successfully!
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields
Fetched 3 users with only 3 fields each

Example 2: Exclude sensitive fields
Fetched 2 admins without sensitive data
  Password field excluded: False

Example 3: Complex query with projection
Fetched 3 active users with profile fields

Example 4: Find by ID with projection
Fetched user profile: {'value': 'Alice Johnson', 'type': 'String'}

Example 5: Compare full vs projected data
Full query returned 12 fields per user
Projected query returned 12 fields per user

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
✅ [32mPython client examples complete![0m
