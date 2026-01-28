make test-examples-python
🧪 [36mRunning Python examples (direct HTTP/WebSocket)...[0m
=== Simple CRUD Operations (Direct HTTP) ===

✓ Authentication successful

=== Insert Document ===
Inserted: {'id': 'sHEeahabnWfUy7eYmNbav3ixMYPtcQVtCXmHfQ_wsQrcbrhitS_7lWjzNbzW3QlKv_zY37lf98t2b3qvrCmrbA'}

=== Find by ID ===
Found: {'active': True, 'value': 42, 'id': 'sHEeahabnWfUy7eYmNbav3ixMYPtcQVtCXmHfQ_wsQrcbrhitS_7lWjzNbzW3QlKv_zY37lf98t2b3qvrCmrbA', 'name': 'Test Record'}

=== Find with Query ===
Found documents: [{'active': {'type': 'Boolean', 'value': True}, 'name': {'type': 'String', 'value': 'Test Record'}, 'value': {'type': 'Integer', 'value': 42}, 'id': 'sHEeahabnWfUy7eYmNbav3ixMYPtcQVtCXmHfQ_wsQrcbrhitS_7lWjzNbzW3QlKv_zY37lf98t2b3qvrCmrbA'}]

=== Update Document ===
Updated: {'value': {'type': 'Integer', 'value': 100}, 'name': {'type': 'String', 'value': 'Updated Record'}, 'active': {'type': 'Boolean', 'value': True}, 'id': 'sHEeahabnWfUy7eYmNbav3ixMYPtcQVtCXmHfQ_wsQrcbrhitS_7lWjzNbzW3QlKv_zY37lf98t2b3qvrCmrbA'}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: _hwl46HALMS28RzJD93jwOtMeWUYANFIwzG5gSnz1rqwWezvbHOgeMS3LMVgSh3lKcXnZdihlCqDmX0udb80oA

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
        "id": "_hwl46HALMS28RzJD93jwOtMeWUYANFIwzG5gSnz1rqwWezvbHOgeMS3LMVgSh3lKcXnZdihlCqDmX0udb80oA",
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
        "id": "oJm5UDTCfxwFeKzYk0FPOeqI3I76oqgMzD9mKIGAVecxCXdowuT5fEPyBstw_M5Ym1RAJ-tWx-mzm-V2sens6A",
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

✅ Script saved: EiFa-E_f9vBoscRFvYp3nY-4r97lLKv21s-58dn0JXQgX7VwMNJtgXP3mpSr3IwoyRkYGfr78ea21qqb4mJYog
📊 Found 35 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: uwKkryUFx2abmIJgreADLWDljFb_0Z4yj4xMQPwpFwteDegHMXkDGnzPX8dsuSQZtYObJSeyFTM-_LHTDVm1XA
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: guLUnW4PePOnvGNPa4d79VR0XF0x1yhaAo_f6uHqbPS-t8i6iv8DLl5zTVS1j0o31FJ0aIwZWZbQjffnsNt_Tw
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {'count': 15, 'avg_score': 60.0, 'status': 'active', 'max_score': 100}
   {'max_score': 90, 'count': 15, 'status': 'inactive', 'avg_score': 50.0}

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
Retrieved value: {'username': 'john_doe', 'userId': 123}

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
Collection created with first record: AcJ8gZ7BIQ4Ktnz2lVPQA-ew8KQyeS27sdJoY9xvih3olVJhmEv-LifYgPE3E1WyOTRDlqEHpSzfPdm7IjjHjg

=== List Collections ===
Total collections: 12
Sample collections: ['test_accounts', 'products', 'ttl_cache', 'batch_users', 'users']

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
Created Alice: $1000 - ID: O7uGtT8SvCHIlCG_OdiNy--rHCoARqF7yuisKnpOeMxq03zEK4Z3JpC0oTmOjf_nuRogd4k0KlQHtI3gRwAvVQ
Created Bob: $500 - ID: CBrDdFvUuzBq_6RZeYdAYoKBkhw88u-N7MfnJuqd_3qYgAbKfVQjtcJsC3g8tYrjN9iJNr-90C-EduFLeLAekg

=== Example 1: Begin Transaction ===
Transaction ID: 8e27d6e2-74bc-4955-8e28-fc7d101c930c

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
New transaction: ce50a3c1-beb3-4ac7-b81d-97fa3914b557
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: RsIcTUv5UijcVuku2OUdqdQ9TnxWrjigh-ukmdOEIt4YCduHHqsRYktdO5PV8e9te671gDVC7mbVSoMzlYHNuA
Created Bob: $500 - ID: WOOD6_CR5DROJHKIpxdM8_i3IidAvUJZpccWTMhKn6jFm9Ezackwff36nGmk9Ic8f-8aiCRJoYuWf67JApyPQg

=== Example 1: Begin Transaction ===
Transaction ID: 80aac5d7-6b5e-41fb-98cc-79cf95a7c02c

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
Created Alice: $1000 - ID: blOWyhV8zFzUGn3ZMRuVLgOPZ5UTFQ8X6Uv59c_CWluEHJcF-ACi_c6hsdQqvI-LCs811nDEpikfdKh9wvDEeQ
Created Bob: $500 - ID: xJt3unnUrAjWfgL_eVsQRQG28y_Op7woDTTeg8wyUtlin5SI2DgcrYOVEQ9KRjWcWuF2pIcX5AUcTHZta9724A

=== Example 1: Begin Transaction ===
Transaction ID: 4e2a184e-bc15-4962-8ad7-4f7acc2f6a29

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
New transaction: b84d5c8e-06e2-400e-9bc6-dfad3bc5b5fa
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 8bfxatYriK6xaCOHk-lsthybkAoWVotPvy8ZDYn1CVB9u6kLrnDcZexALle6bs1sLmIuuiOOmWXV_svm9vYDyA
Created Bob: $500 - ID: XzRXgVu3_4Y5gSNqQDjDGg6EnouozpwqY1Uxi9QBrImd9ojm2-VsoBzpC5chO3SKGu9XiNl0coIt0hX2DzbX4g

=== Example 1: Begin Transaction ===
Transaction ID: 7d3e07bf-9613-468b-9b03-d43b181839e8

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
Created Alice: $1000 - ID: HCNwQONamPxJ_rIIV_FEId4Kt_sb42BmkoMfyo-dLIs9CPaKUHa6-ZMHVRGq5HBZTafs4x2tu1xMohTefjHYnA
Created Bob: $500 - ID: ctQBGy8GJpqfW9797_tG0j0Y8AYlVE35_ef7h4sp0Nz-hOD2tNJcuWxxC6noryfFcxWxIpz04X67KS46XM0QxQ

=== Example 1: Begin Transaction ===
Transaction ID: 941bddba-10b5-43db-9cea-dff337fa1643

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
New transaction: ba36f0b7-c439-4003-89f8-f79358adcd05
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 5H1FffHfijYYjP1hgJzMG0NNsRpmtqVhEuVtewMFDPiYNXjRshd_Iusfb7_iRSg_U92g1pv6X0oUH1PdyeYT8Q
Created Bob: $500 - ID: qI6IUwIB2CftZQuddFgxWsqsjnI3HtiTvX7DetZKcDpFpHXjUjh4CeqX80eCF8yzWdUgKdlYjEvCjPGthXGtDQ

=== Example 1: Begin Transaction ===
Transaction ID: 73d5cb68-de46-431f-ab4c-22aff4abf462

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
   ✅ Script saved: x5dywLH37Cs01Tqtje4EaYLvLtu3GO47RtgvApeeoqiiO84ADHNxRBb3kIFISvvg4PvGYUcCcmACAveV0N_fLQ

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: GL2b5sXFyQCNSTdPzS2coQLGodHzg9pwI77SPv6_IJX2rdnD71NceH1MloKaEhWimogNVCqNgAL8lx3KRkSCbQ
   📋 Name: {'value': 'Alice Smith', 'type': 'String'}
   📋 Email: {'type': 'String', 'value': 'alice@example.com'}
   📋 Status: {'value': 'pending', 'type': 'String'}
   📋 Credits: {'type': 'Integer', 'value': 0}

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: Yqefs6edoyRo36R_uWvEjt2uR1w8E2hkswA4pzc2xl1q_bsGXy9VizINkyZEpP3VUte9MmAYt2ukJmq7HYkxaw

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
   ✅ Script saved: JHt09pKFcM_KNUIrEwOUiUCx0Vj1yzjN_icWqPlMrylEOiPqHJw4YD60Eqlz7d8ePOaVKAh81YFbnZHgOsgjoQ

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
   ✅ Script saved: duhh6QPxx40ow8J3LJL23muqIvmrfZO9At4zQ_xI0DCVZGTNpEcUw5Dt0OztRTphooB7cBTX0ZoMsHbpX1lNkg

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: x5dywLH37Cs01Tqtje4E...
   ✅ Deleted script: Yqefs6edoyRo36R_uWvE...
   ✅ Deleted script: JHt09pKFcM_KNUIrEwOU...
   ✅ Deleted script: duhh6QPxx40ow8J3LJL2...
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
  Output: Document ID = WNidZBEscxe5fMREpFdp0Y6wjmi9ZuvLLgLfN_GjHhA14gJZAIcY3UD6vvpSTUBm962u155GX9Rwtpk9MY9ZLQ
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: find_by_id(WNidZBEscxe5fMREpFdp0Y6wjmi9ZuvLLgLfN_GjHhA14gJZAIcY3UD6vvpSTUBm962u155GX9Rwtpk9MY9ZLQ)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: find_by_id(WNidZBEscxe5fMREpFdp0Y6wjmi9ZuvLLgLfN_GjHhA14gJZAIcY3UD6vvpSTUBm962u155GX9Rwtpk9MY9ZLQ)
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
  Output: Document ID = jkeQR7pO-2bJQHSE7KuPo1sOB_n5wD-buB5176bCl2JQAnuGEfZlSS5vMr3ARk_uWZxYQg_PKtN7vJl8mE2uxg
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: find_by_id(jkeQR7pO-2bJQHSE7KuPo1sOB_n5wD-buB5176bCl2JQAnuGEfZlSS5vMr3ARk_uWZxYQg_PKtN7vJl8mE2uxg)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: find_by_id(jkeQR7pO-2bJQHSE7KuPo1sOB_n5wD-buB5176bCl2JQAnuGEfZlSS5vMr3ARk_uWZxYQg_PKtN7vJl8mE2uxg)
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
    Finished `release` profile [optimized] target(s) in 0.08s
📦 Built wheel for abi3 Python ≥ 3.8 to /Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.9.0-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel for current platform...[0m
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.9.0-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.9.0
    Uninstalling ekodb_client-0.9.0:
      Successfully uninstalled ekodb_client-0.9.0
Successfully installed ekodb-client-0.9.0
✅ [32mPython client package built and installed![0m
🧪 [36mRunning Python client library examples...[0m
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {'id': '2YLTnRwJMd6N1bHZauHG-9dYnhW3AGlzUg4NvQgxSxuTKeP8ALg3EVMKNh8zX8MJruLMvdscHgn_98jwggC-vA'}

=== Find by ID ===
Found: {'metadata': {'nested': {'deep': True}, 'key': 'value'}, 'active': True, 'value': 42, 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5], 'name': 'Test Record', 'price': 99.99, 'id': '2YLTnRwJMd6N1bHZauHG-9dYnhW3AGlzUg4NvQgxSxuTKeP8ALg3EVMKNh8zX8MJruLMvdscHgn_98jwggC-vA', 'tags': ['tag1', 'tag2', 'tag3'], 'user_id': '550e8400-e29b-41d4-a716-446655440000', 'data': 'aGVsbG8gd29ybGQ=', 'categories': ['electronics', 'computers'], 'created_at': '2026-01-27T19:11:50.064913'}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): True
  price (Decimal): 99.99
  created_at (DateTime): 2026-01-27 19:11:50.064913
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): ['tag1', 'tag2', 'tag3']
  metadata (Object): {'nested': {'deep': True}, 'key': 'value'}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): ['electronics', 'computers']
  data (Bytes): 11 bytes
Plain record: {'metadata': {'nested': {'deep': True}, 'key': 'value'}, 'active': True, 'value': 42, 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5], 'name': 'Test Record', 'price': 99.99, 'id': '2YLTnRwJMd6N1bHZauHG-9dYnhW3AGlzUg4NvQgxSxuTKeP8ALg3EVMKNh8zX8MJruLMvdscHgn_98jwggC-vA', 'tags': ['tag1', 'tag2', 'tag3'], 'user_id': '550e8400-e29b-41d4-a716-446655440000', 'data': 'aGVsbG8gd29ybGQ=', 'categories': ['electronics', 'computers'], 'created_at': '2026-01-27T19:11:50.064913'}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'tags': {'type': 'Array', 'value': ['tag1', 'tag2', 'tag3']}, 'created_at': {'type': 'String', 'value': '2026-01-27T19:11:50.064913'}, 'name': {'type': 'String', 'value': 'Updated Record'}, 'embedding': {'value': [0.1, 0.2, 0.3, 0.4, 0.5], 'type': 'Array'}, 'active': {'type': 'Boolean', 'value': True}, 'value': {'type': 'Integer', 'value': 100}, 'price': {'value': 99.99, 'type': 'Float'}, 'id': '2YLTnRwJMd6N1bHZauHG-9dYnhW3AGlzUg4NvQgxSxuTKeP8ALg3EVMKNh8zX8MJruLMvdscHgn_98jwggC-vA', 'data': {'type': 'String', 'value': 'aGVsbG8gd29ybGQ='}, 'user_id': {'value': '550e8400-e29b-41d4-a716-446655440000', 'type': 'String'}, 'categories': {'value': ['electronics', 'computers'], 'type': 'Array'}, 'metadata': {'type': 'Object', 'value': {'nested': {'deep': True}, 'key': 'value'}}}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: dCYOHo6RbL2T5YhR3Jy8rgFhav-LFExhRPDYL6Z2rumh6cF4qOl0Jmw1f0J3QjiwYdyyAesushbC25MWMvF5dw

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
Collection created with first record: "1HFzjJxufoM1Ix3_nR7dkBWa4itbkTNUInXXeZa0PC32aFR4nt72pj3wJIPuh95Zd5hvGQKz3pMjXpLynVTasQ"

=== List Collections ===
Total collections: 11
Sample collections: ['test_accounts', 'client_collection_management_python', 'products', 'ttl_cache', 'batch_users']

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
Created Alice: $1000 - ID: fcAxozKQdIeGGKsee3laTGd31q9uV_oJw6rmHkbVNmWZG-3EXCuunLK82bFko37jrkzyFpBXCAAsF90lw3tBjQ
Created Bob: $500 - ID: XVnMYL5fR8zBgskxnv7s7bnzoQQl4RHdZZ5d3OJ1OXMOgzhNRfEFbTUshbnKnwu5FgOkvXqv27BBCMGGrdBImg

=== Example 1: Begin Transaction ===
Transaction ID: 86cfab0e-690b-4589-8f87-c2da8d235330

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: a2244811-293e-4a54-883b-d004f53bab41
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
  1. Score: 25.740, Matched: name.value, name, email.value, email
  2. Score: 12.540, Matched: name.value, name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: bio.value, bio, title, title.value
  2. Score: 26.400, Matched: title, title.value, bio.value, bio
  3. Score: 26.400, Matched: bio, title.value, title, bio.value
  4. Score: 26.400, Matched: title.value, bio, title, bio.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title.value, bio, title, bio.value
  2. Score: 39.600, Matched: bio.value, title.value, title, bio
  3. Score: 39.600, Matched: title.value, title, bio, bio.value
  4. Score: 39.600, Matched: bio.value, bio, title, title.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio.value, bio

5. Search with stemming and exact match boosting:
Found 2 results (matches: run, running, runs)
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio.value, bio

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.749
  2. Score: 0.749
  3. Score: 0.747

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.373, Matched: title, content.value, title.value, content
  2. Score: 1.374, Matched: title, title.value, content, content.value
  3. Score: 0.375, Matched: 

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
✓ Inserted document: gylFzkZztAfjnpoRVGMWCmT0Gx-v3FNYxmEp10zez1GKXnkVGehsUZsATOcCjeR6e-DVvWh8jkyxLivxHwJuNA

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: InCNfwcUo7KZzXDiLX0X3lzMPQ-CCZWctsLG822as-rAtnKxIOJYg8xw3rn2Qz34a763cJWSxCQ843yLwe4JuA

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
✓ Inserted document with TTL: 5owEqYVM9n1iLGu_rsZDYDABXP9NUwDJIla3h0W2Po6J_WvS0qTyo_ibFT-HeW9rI8ngifziTHrdYbuMUwFXYw

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
✓ Edge cache script created: BWjpDRnYaOGQD75a1JsmXYJFj0P1zU9gqcRorTBuQPcaI4sK5Ezrhl-5YoXj3pv_7rJZVze5wj3WJdRthsrP6g

Call 1: Cache miss (fetches from API)
Response time: 1825ms
Result: {
  "records": [
    {
      "value": {
        "current": {
          "interval": 900,
          "temperature_2m": -10.7,
          "time": "2026-01-28T00:00"
        },
        "current_units": {
          "interval": "seconds",
          "temperature_2m": "\u00b0C",
          "time": "iso8601"
        },
        "elevation": 32.0,
        "generationtime_ms": 0.027298927307128903,
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
Response time: 5ms (346.6x faster!)
Result: {
  "records": [
    {
      "value": {
        "current": {
          "interval": 900,
          "temperature_2m": -10.7,
          "time": "2026-01-28T00:00"
        },
        "current_units": {
          "interval": "seconds",
          "temperature_2m": "\u00b0C",
          "time": "iso8601"
        },
        "elevation": 32.0,
        "generationtime_ms": 0.027298927307128903,
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

✅ Script saved: PQKKOeW9383xaWtx7egNjEK-PXAoV8lbKGikg7o-LKPEVAGcvUCf-pPh0sXaYpSI6XZ_6KzwysuhJkeRXHPVKQ
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved: y8ZRWQuMqVhRqIWhd898roUQoKPxOTOS4irWi0qFTuTe9T1aG0UfXOFUkMmnr_-rXFTve2s6UjJ4wetdV1SwsA
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: bD1eHa4eBkOO50H0YKCFc_ItPv7eKN0s7OQYjxFxrQpMYtNxAsJuewTelTK3_UOxJP4MOtBV-CDWco37navcAA
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
   ⏱️  Duration: 1.7ms
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
   ⏱️  Duration: 1.8ms
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

✅ Script saved: Fp6kRYeelc4eckkzFqF2y3CBXUqdpxTwkE_GXXvDJttH99uK2Av0nrtWy9JuYY1Dd3p667GrnRBXBqeZgAivAg
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

✅ Inserted order: HWfNW3bNfyTjDe6GKqp-QP88aTkFHcjy4fcnnwW2Zfvg87igIG49PMTve33WWUmMcL4m2OVmMwTNptpzvumWtw
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
🗑️  Deleted session: None

📝 Example 4: KV Pattern Query

✅ Set 4 config entries
📊 Found 3 app config entries
📊 Found 4 total config entries

📝 Example 5: Combined Wrapped Types + KV Usage

✅ Inserted order: LtGOKOC7nRZi8F2VxVUzs4dU1ByvZeK0NaB6nM-ZeXrBqyyuB_oDGJf6NR6u6ZF-s6M2d5dpz6IXfjadKmTQ5g
✅ Cached order status
📊 Quick status lookup: {'value': '{"status":"processing","updated_at":"2026-01-28T00:11:53.692135+00:00"}'}

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
✓ Created SWR script: fetch_api_user_py (C2Kzc3f4YkheHRtged8X7Cu5MRhFn0b14j6Xr2xBQkv4v9EemEeep3uK71oizrdxUQhDj52CKbnfg28OhFaD-A)

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
Response time: 5ms (served from cache)
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
✓ Created native SWR script: github_user_native (XlXkEVCea7ubMqK0tz09fQL4njJ-XcisnLuX7k80zezwFr3KLny7AydmyAIwIseGrDGOq-otLrreatq2qrvuow)

First call (cache miss - will fetch from GitHub API):
  Response time: 136ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 4ms
  Speedup: 31.8x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (XMOs0Bc6oQ4AabgGI1UPs_h4-C3akfLONY0f5hC0rb0ZNZqWQwcoVepdDS9zJSWJToJ1DvGT6JbB5Rbnow7BAA)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (DINywdRalStE3-BTzDEOfIVsZCMz-Zehik0KIAvA0ZUfAFX7aqWgYuro1TFPKCbNSxH1syLrFKeYoFffQMNz6A)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (y43DVsF0VXcn5DwhreW7Ed8uCNwH_plqJhYBZyioRN1G2K6KvLMZpFV_MtgiisVGhyYNyUGDoJ9_bMfkiJiH_A)
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
   Vector databases offer several benefits such as high precision, scalability, and the ability to represent complex geographical features like points, lines, and polygons. They support advanced spatial analysis and mapping due to their ability to accurately depict size, shape, and location. Additionally, data in vector form is typically easier to edit and manipulate.
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
   1. Natural Language Processing (AI)
   2. Getting Started with ekoDB (Database)
   3. Database Design Principles (Database)
   4. Introduction to Machine Learning (AI)
   5. Vector Databases Explained (Database)
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
✓ Created session: VofpQp9wru7iR1LwyfXa3iLZcXzr9oQBFOB34zHu_xa9TrylGOYBgMtqMcdLtGSAcC0VP6elo5_67K8BE79w7g

=== Sending Chat Message ===
Message ID: PZTXUp9oOKE4DskbVKSklbfj-f3B5lgMcOwQ7SRoRE3Akn0HLTKRhIZ78Szk16UUA0JpAAKf1xNTq5dSq6EaaA

=== AI Response ===
There are three products available:

1. ekoDB Pro: This is an enterprise edition product with advanced features. The price is $299.

2. ekoDB: This is a high-performance database product with AI capabilities. The price is $99.

3. ekoDB Cloud: This is a fully managed cloud database service product. The price is $499.

Execution Time: 3045ms

=== Token Usage ===
Prompt tokens: 449
Completion tokens: 75
Total tokens: 524

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: z2mgCFGtqX0NhwE6GRqnMBuYvLj7Y3xIoSYkqPay4QbPMX98qx0wYEBX9UaG0GUWAwKTvMWRN-De_RIQSgt59Q

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the context provided, the available product is named "ekoDB". It's a high-performance database product and its price is $99.

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['role', 'content', 'updated_at', 'created_at', 'context_snippets', 'token_usage', 'chat_id', 'id'])
Debug: First message role: {'value': 'assistant', 'type': 'String'}
=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of ekoDB is $99.

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: vBYgCKA7tCIFVCp4q8I18qr8arv5t1VHkvH8epnh6z8K-qtc4ACIAQ8D9tijYnjG5uegaQ52hu-xo1IcjJHWTw
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
✓ Created session: 6AhoM-pLbXz0EkVBNngDDFBYloDHpeJWLOckv2klHQDr0Vo2qoDXVPNFCm0hoI-6JepnpIg60RTqwWMYpwc9dg

=== Sending Messages ===
✓ Message 1 sent
  Response: The product available is the ekoDB, a high-performance database product. It is priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: sTUHv9_mQNGtEOC-98q7pTy9-Lzturs-RHetrmLIJCyyB2PoOx4u5NOK2BJcsP6WgpTwPsCXHTk-XA23VBKUQQ
  Parent: 6AhoM-pLbXz0EkVBNngDDFBYloDHpeJWLOckv2klHQDr0Vo2qoDXVPNFCm0hoI-6JepnpIg60RTqwWMYpwc9dg

=== Listing Sessions ===
✓ Found 7 sessions
  Session 1: sTUHv9_mQNGtEOC-98q7pTy9-Lzturs-RHetrmLIJCyyB2PoOx4u5NOK2BJcsP6WgpTwPsCXHTk-XA23VBKUQQ (Untitled)
  Session 2: 6AhoM-pLbXz0EkVBNngDDFBYloDHpeJWLOckv2klHQDr0Vo2qoDXVPNFCm0hoI-6JepnpIg60RTqwWMYpwc9dg (Untitled)
  Session 3: vBYgCKA7tCIFVCp4q8I18qr8arv5t1VHkvH8epnh6z8K-qtc4ACIAQ8D9tijYnjG5uegaQ52hu-xo1IcjJHWTw (Untitled)
  Session 4: VofpQp9wru7iR1LwyfXa3iLZcXzr9oQBFOB34zHu_xa9TrylGOYBgMtqMcdLtGSAcC0VP6elo5_67K8BE79w7g (Untitled)
  Session 5: vd5EO4q7AGtzgy6rMLWE_cVehblRhF5qYGcg6b_LzAk5HEuCM4Py_U3E7SJQDW_euOhnS4KcItAtKBXvLKkibg (Untitled)
  Session 6: p-ZCLCGlwHgVx28M8OWhlqK2IUXGNo3BEjAUu1504UE3P7KKhpdpHzzfIE8pn0ivZxDV81GWHmCLZzzHlEGMmg (Untitled)
  Session 7: 1C-L1tqnYk1eKUb3g3_TcFE6VIqGbd8bFPOUYLln1Uu4Fcu5rTWtTyI4UqGIYCE5ksDdntSQVJfrdS4sBBSpNg (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: sTUHv9_mQNGtEOC-98q7pTy9-Lzturs-RHetrmLIJCyyB2PoOx4u5NOK2BJcsP6WgpTwPsCXHTk-XA23VBKUQQ

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Dict Creation ===
✓ Created record with plain dict: {'id': 'ggoZ3jvYcQ3ZugD0ZOQ6OM79At7DP4N6iG8ifQpU634yFLnlYacAsdd0uKFWsCnYDpxGQo0zRpYAlPRuhrHOvw'}

=== Upsert Operation ===
✓ Upsert (update existing record): ggoZ3jvYcQ3ZugD0ZOQ6OM79At7DP4N6iG8ifQpU634yFLnlYacAsdd0uKFWsCnYDpxGQo0zRpYAlPRuhrHOvw
✓ Inserted second record: WMqWSbZ_TU3C32efSUGRiolZOM85jeJMsaizqjk-WhGGWA_FzlC5_Nkn7tMA9QoF6H3wuuJGbmdnL3f0gR5lOQ
✓ Upsert (update second record): WMqWSbZ_TU3C32efSUGRiolZOM85jeJMsaizqjk-WhGGWA_FzlC5_Nkn7tMA9QoF6H3wuuJGbmdnL3f0gR5lOQ

=== Find One Operation ===
✓ Found user by email: {'age': {'type': 'Integer', 'value': 29}, 'name': {'type': 'String', 'value': 'Alice Johnson'}, 'id': 'ggoZ3jvYcQ3ZugD0ZOQ6OM79At7DP4N6iG8ifQpU634yFLnlYacAsdd0uKFWsCnYDpxGQo0zRpYAlPRuhrHOvw', 'email': {'type': 'String', 'value': 'alice.j@newdomain.com'}, 'active': {'type': 'Boolean', 'value': True}}
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
   Inserted with ripple: {'id': 'm4rcWYnml2Q9sG5umtVaGs4gHUY4YU2dBUaS4hjtj-u2EC9Si-c2in-4We3Yb_5hWpSws2BnT02zC1LgcLrf5g'}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {'id': 'zGVeLIo8sAd48iAmUI5qwSCh1V0LYuJYx5MrO_gRsoTQtre526AafWugv53G4s7-DBN3r9NZVzUIEC12BThczA'}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {'price': {'type': 'Integer', 'value': 150}, 'id': 'm4rcWYnml2Q9sG5umtVaGs4gHUY4YU2dBUaS4hjtj-u2EC9Si-c2in-4We3Yb_5hWpSws2BnT02zC1LgcLrf5g', 'name': {'value': 'Product 1', 'type': 'String'}}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {'name': {'type': 'String', 'value': 'Upsert Product'}, 'price': {'value': 500, 'type': 'Integer'}, 'id': 'm4rcWYnml2Q9sG5umtVaGs4gHUY4YU2dBUaS4hjtj-u2EC9Si-c2in-4We3Yb_5hWpSws2BnT02zC1LgcLrf5g'}

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
Fetched user profile: {'type': 'String', 'value': 'Alice Johnson'}

Example 5: Compare full vs projected data
Full query returned 12 fields per user
Projected query returned 12 fields per user

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
  Float 0.1 + 0.2 = None (should be 0.3)
  Decimal "0.30" = None (exact!)

=== Cleanup ===
✓ Cleaned up test keys

=== Summary ===
✅ Use field_decimal() for monetary values, percentages, and
   any case where floating-point errors are unacceptable.
✅ field_decimal() stores values as strings internally,
   preserving exact precision across all operations.
✅ [32mPython client examples complete![0m
