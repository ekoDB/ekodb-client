make test-examples-python
📦 Ensuring Python example dependencies in .venv...
🧪 Running Python examples (direct HTTP/WebSocket)...
=== Simple CRUD Operations (Direct HTTP) ===

✓ Authentication successful

=== Insert Document ===
Inserted: {'id': '8EGz8v-ddpOaYxF8ZdakbiSDPKpoCqvHUdnewMkcN-5UPM0KOqIPc1LGBOtH5GqQyFUkIv3j5Pc3Vbe1rnZNhg'}

=== Find by ID ===
Found: {'value': {'type': 'Integer', 'value': 42}, 'active': {'type': 'Boolean', 'value': True}, 'name': {'type': 'String', 'value': 'Test Record'}, 'id': '8EGz8v-ddpOaYxF8ZdakbiSDPKpoCqvHUdnewMkcN-5UPM0KOqIPc1LGBOtH5GqQyFUkIv3j5Pc3Vbe1rnZNhg'}

=== Find with Query ===
Found documents: [{'id': '8EGz8v-ddpOaYxF8ZdakbiSDPKpoCqvHUdnewMkcN-5UPM0KOqIPc1LGBOtH5GqQyFUkIv3j5Pc3Vbe1rnZNhg', 'value': {'type': 'Integer', 'value': 42}, 'name': {'type': 'String', 'value': 'Test Record'}, 'active': {'value': True, 'type': 'Boolean'}}]

=== Update Document ===
Updated: {'name': {'type': 'String', 'value': 'Updated Record'}, 'active': {'type': 'Boolean', 'value': True}, 'id': '8EGz8v-ddpOaYxF8ZdakbiSDPKpoCqvHUdnewMkcN-5UPM0KOqIPc1LGBOtH5GqQyFUkIv3j5Pc3Vbe1rnZNhg', 'value': {'type': 'Integer', 'value': 100}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: DIRmPQUEeMeb2kbxKaB88YHsNYei1rVFefF8R9TDm71jPF-AkNIkhk3Jl5mSyVpkmvnem8A4CWRq6d-xvU_Z3g

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "type": "Success",
  "payload": {
    "data": [
      {
        "id": "DIRmPQUEeMeb2kbxKaB88YHsNYei1rVFefF8R9TDm71jPF-AkNIkhk3Jl5mSyVpkmvnem8A4CWRq6d-xvU_Z3g",
        "active": {
          "value": true,
          "type": "Boolean"
        },
        "value": {
          "value": 42,
          "type": "Integer"
        },
        "name": {
          "value": "WebSocket Test Record",
          "type": "String"
        }
      }
    ]
  },
  "messageId": "397463568"
}
✓ Retrieved 1 record via WebSocket

✓ WebSocket example completed successfully
🚀 ekoDB Functions Example (Python/HTTP)

✓ Authentication successful

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Function saved: Wo8V2erlCgrE0cepizpNmHzXrAIr0kIwEzZgyW6zsEEKoLoJcfnGKbP4k5DgXh8RGIxYPY8aHbHNTUGo1jVvGw
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Function saved: 1zz-V-BpE86bQA8B2PfSgXyM3oRmN4BnEbfvcHLMMZ1WQpu0n2HnjZE3792M3YQJ6_SPqOTXnY0czhoFi-sv6A
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: 1JHCJxojeSXAi71Vy8ZDth8cJHyozheUon2dVJGN5A9luayzm4o41ovQnJnrgHU2wEzPMmhGVNdizU8-5IrkaQ
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {'max_score': {'type': 'Integer', 'value': 90}, 'status': {'value': 'inactive', 'type': 'String'}, 'count': {'type': 'Integer', 'value': 5}, 'avg_score': {'value': 50.0, 'type': 'Float'}}
   {'max_score': {'type': 'Integer', 'value': 100}, 'count': {'value': 5, 'type': 'Integer'}, 'avg_score': {'type': 'Float', 'value': 60.0}, 'status': {'value': 'active', 'type': 'String'}}

📝 Example 4: Function Management

📋 Total scripts: 3
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
✓ Verified: Found 5 total records in collection

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
Retrieved value: {'type': 'Object', 'value': {'username': 'john_doe', 'userId': 123}}

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
kv_operations:direct:py:cache:product:1: {'type': 'Object', 'value': {'price': 29.99, 'name': 'Product 1'}}
kv_operations:direct:py:cache:product:2: {'value': {'price': 39.989999999999995, 'name': 'Product 2'}, 'type': 'Object'}
kv_operations:direct:py:cache:product:3: {'value': {'name': 'Product 3', 'price': 49.989999999999995}, 'type': 'Object'}

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
=== Collection Management (Direct HTTP) ===

✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: Q05G8ajtHOuBFC8MGbHWYRVLFK96O-kOcIEefD_pZ39VyCqnCKgDd4Yx8hs8DWG_7g841pSseS1s6_DXsrNnbQ

=== List Collections ===
Total collections: 13
Sample collections: ['demo_collection', 'chat_goals__ek0_testing', 'schedules__ek0_testing', 'agent_function_versions__ek0_testing', 'chat_raw_completions__ek0_testing']

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
Created Alice: $1000 - ID: D-u6Jjw4ClqySZaSJrgQex4O-87sk6XjLfRrhnBLJt87Ae-C-u1EloiTH5k6yErSX3HeXLX3nPJ_v8XuAmRVdQ
Created Bob: $500 - ID: MwZlbkDLfy9RpTrvgBFBqt7EoCb7N5zn_nuQXGuRToY4oHyUyFZrdMB58HLiPiq18nzRvh3J5NMP2m-fr7t2vw

=== Example 1: Begin Transaction ===
Transaction ID: 6e51ef90-48c0-4d90-b535-f0c985d7179d

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
New transaction: c27e915f-184e-4b10-a44f-7ff85341da22
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'value': 700, 'type': 'Integer'}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: hl-1U5Unaavs0TR8BC8LRHdJkWWcse3W1Sx1_VQI_wWdxmIZExNS-VLrZ-Y5twWEguXvFm9Ln4x7k3IVLJJB3g
Created Bob: $500 - ID: L0nLFmFOOa8XJkMs8pifsQtzgS_h4EWxhHxWlXZSgfwDJ_-XAKgplaqSx4UNYi8DozqTa0NslLaiLY4Uh_NZlg

=== Example 1: Begin Transaction ===
Transaction ID: 9671cee5-f29f-4b7b-9b25-c0fe55979ca6

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
Created Alice: $1000 - ID: bVGw4mOXov2OtCr6_8TWT1VombO5HGHzZaiY86pTirfcAhBMWR0k0IMojlRo0-HntMP37j7CuvonD4cWRM-5lw
Created Bob: $500 - ID: 2BJqe8S2SJssrT59_-WK1s3Y8wXuwXzmtxjBqrhLK8-LcgdJAIpmCUCSVXaZYC4c1P-JuTHD-HO-pRZUw7mclg

=== Example 1: Begin Transaction ===
Transaction ID: 91a52fc2-3ceb-4ac8-9575-51b3f1cc9afb

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
New transaction: 2a0f4a0c-dde9-4eb9-8088-963c76d1dc17
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'value': 700, 'type': 'Integer'}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 0M6mtt_Ui0n-dI2BoO0zhXNB63X1Qdw7700PRFK5BmBkBGPQqo0-1UC4GPXiavh6_omNL6ltIfUPF3rvcuRSXQ
Created Bob: $500 - ID: J-NOi1BtGqcULtNRpXdQHh5qhBxinFAlYWHd9zdKBery1VGGINSnKj6KVBv3RBdloo7Ik1YYpbChKc2hwFsWpA

=== Example 1: Begin Transaction ===
Transaction ID: d5ced8cc-bbc2-4ce2-9dc0-916d34c19c94

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
Created Alice: $1000 - ID: _vH8qWx8DWBarstLYYIy1ICL2rfUdi9TRoAIHfVnTfhWKLSgOffGZRTaIF8KdlGl-6hlZixccHFFQI8FNqCaCw
Created Bob: $500 - ID: tI4V9vINipfyXnT9zXK_G0GtThvilYEW4_ujT8GuyWRxksLy8WV7VMDbhoqdlxbUeBGBYGYh6M4zQ5kqSWNPuA

=== Example 1: Begin Transaction ===
Transaction ID: 412c58ff-0eaa-4536-92ae-6af57d725e4a

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
New transaction: 72d9d94d-e74a-4d1d-90d3-56050818d9d1
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'value': 700, 'type': 'Integer'}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: sjHa8IY6IWy1Ybq30Bn6WJKgdkUNtGQwyZYS8INWyMx0Zhds1PvG78mNIPzevomQEbnxeZ2uxv4l-pYvuIELzg
Created Bob: $500 - ID: slUOZ4b5uMhyENbDYlT9wX9qIhGqh5vN18Hp2pAV7TaZDDqEjHZkBTv3IipGKke3W29refi1irHxRlcv_NYo7g

=== Example 1: Begin Transaction ===
Transaction ID: 449bbc73-4e38-4f41-9fcc-18a352161210

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
🧹 Cleanup
============================================================
   ✅ Deleted collection: crud_functions_users_py
============================================================
📝 function 1: Insert + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: VyInv1b29beS8EmtbU7NzW1Tk0KUK__5lwzf6ckSib8X6GkKFNoZQWKXbS6kCbtBNV2GrcOOHQmk529RjLMlGA

2️⃣ Calling function (Insert + Verify)...
   ✅ function executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: a2SqBgk8dXdcvB7CWbLtCjjgAOOrXtUjYi5sTkdVPRmNIo176Mh_xFxKbkaOnnH6-MdUkBFeNkLJ4KBAD5S51w
   📋 Name: {'value': 'Alice Smith', 'type': 'String'}
   📋 Email: {'type': 'String', 'value': 'alice@example.com'}
   📋 Status: {'value': 'pending', 'type': 'String'}
   📋 Credits: {'type': 'Integer', 'value': 0}

============================================================
📝 function 2: Query + Update + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: Fw22rDs6SkdRP_s7OoKjblNl9Hs-QNedIL1U7vtPKwDOPRSpOP3_vjE3vEoQVUAikLH-bcmtkfvlMiZQsIO-zg

2️⃣ Calling function (Query + Update + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Status updated to: {'type': 'String', 'value': 'active'}
   📋 Name: {'value': 'Alice Smith', 'type': 'String'}

============================================================
📝 function 3: Query + Update Credits + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: r5zoPdXE6F1JBVgzae_OOGMjSHcAAF7f23n3ykKGMWfnk67mmDpTFaieFEseMtAPZnT1prM_tksrXGa_yS8PhQ

2️⃣ Calling function (Query + Update Credits + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {'type': 'Integer', 'value': 100}
   📋 Status: {'type': 'String', 'value': 'active'}
   📋 Name: {'value': 'Alice Smith', 'type': 'String'}

============================================================
📝 function 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: lbH463u7czNm0nVxJJ_4YHDCNr5nAgg1VCD9fYrG9NUsq_6Yx2DqqrKb0jMW_5mVUv1L15PuybBn5tD1TbdEQQ

2️⃣ Calling function (Query + Delete + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: VyInv1b29beS8EmtbU7N...
   ✅ Deleted script: Fw22rDs6SkdRP_s7OoKj...
   ✅ Deleted script: r5zoPdXE6F1JBVgzae_O...
   ✅ Deleted script: lbH463u7czNm0nVxJJ_4...
   ✅ Deleted collection: crud_functions_users_py

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
  Output: Document ID = hQBdLMUlnbgomudlH7dLU3FTWvAggwjLVUbuWzV_xlTe3-LpQ0sfobW_lkmmsNDQtErok6er0YA14-yef_SCDQ
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: find_by_id(hQBdLMUlnbgomudlH7dLU3FTWvAggwjLVUbuWzV_xlTe3-LpQ0sfobW_lkmmsNDQtErok6er0YA14-yef_SCDQ)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: find_by_id(hQBdLMUlnbgomudlH7dLU3FTWvAggwjLVUbuWzV_xlTe3-LpQ0sfobW_lkmmsNDQtErok6er0YA14-yef_SCDQ)
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
  Output: Document ID = bCUojgoZie_LFe4bXesT0wb4OyJymhSwEdYoSvBmSf0ZFnIu-e5aEVH2Dg8eUliPLA5kB7VbRiRzBGfq5SyG5g
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: find_by_id(bCUojgoZie_LFe4bXesT0wb4OyJymhSwEdYoSvBmSf0ZFnIu-e5aEVH2Dg8eUliPLA5kB7VbRiRzBGfq5SyG5g)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: find_by_id(bCUojgoZie_LFe4bXesT0wb4OyJymhSwEdYoSvBmSf0ZFnIu-e5aEVH2Dg8eUliPLA5kB7VbRiRzBGfq5SyG5g)
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

╔════════════════════════════════════════╗
║   ekoDB Python Examples Test Suite    ║
╚════════════════════════════════════════╝

=== Checking Server Connection ===
✓ Server is ready

=== Getting Authentication Token ===
✓ Authentication successful

=== Running 10 Examples ===

=== Running simple_crud.py ===
✓ simple_crud.py completed successfully

=== Running simple_websocket.py ===
✓ simple_websocket.py completed successfully

=== Running http_functions.py ===
✓ http_functions.py completed successfully

=== Running batch_operations.py ===
✓ batch_operations.py completed successfully

=== Running kv_operations.py ===
✓ kv_operations.py completed successfully

=== Running collection_management.py ===
✓ collection_management.py completed successfully

=== Running transactions.py ===
✓ transactions.py completed successfully

=== Running crud_functions.py ===
✓ crud_functions.py completed successfully

=== Running document_ttl.py ===
✓ document_ttl.py completed successfully

=== Running websocket_ttl.py ===
✓ websocket_ttl.py completed successfully

╔════════════════════════════════════════╗
║           Test Summary                 ║
╚════════════════════════════════════════╝
Total: 10
Passed: 10
Failed: 0
✅ Python direct examples complete!
🐍 Building Python client package...
🔧 Ensuring maturin is available in .venv...
🔨 Building wheel...
🍹 Building a mixed python/rust project
🐍 Found CPython 3.11 at /Library/Frameworks/Python.framework/Versions/3.11/bin/python3
🔗 Found pyo3 bindings with abi3-py3.8 support
💻 Using `MACOSX_DEPLOYMENT_TARGET=11.0` for aarch64-apple-darwin by default
    Finished `release` profile [optimized] target(s) in 0.15s
📦 Built wheel for abi3 Python ≥ 3.8 to ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.26.4-cp38-abi3-macosx_11_0_arm64.whl
📦 Installing Python wheel into .venv...
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.26.4-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.26.4
    Uninstalling ekodb_client-0.26.4:
      Successfully uninstalled ekodb_client-0.26.4
Successfully installed ekodb-client-0.26.4
🧪 Ensuring test dependencies (pytest) in .venv...
✅ Python client package built and installed!
=== ekoDB Advanced CRUD Example (Python) ===

--- insert ---
Inserted: XSNVof4dIF8mfadOhhnxfGm6uPb32g2RE2zi_TuBMvd3IMLSg_G0S429Z--MnN2qal7ksGMj-D6ztP25vPATmA

--- update_with_action (increment) ---
After increment count by 5: {'tags': {'value': ['initial'], 'type': 'Array'}, 'count': {'type': 'Integer', 'value': 15}, 'score': {'value': 50.0, 'type': 'Float'}, 'name': {'type': 'String', 'value': 'Counter Record'}, 'id': 'XSNVof4dIF8mfadOhhnxfGm6uPb32g2RE2zi_TuBMvd3IMLSg_G0S429Z--MnN2qal7ksGMj-D6ztP25vPATmA'}

--- update_with_action (decrement) ---
After decrement score by 10: {'tags': {'type': 'Array', 'value': ['initial']}, 'score': {'type': 'Float', 'value': 40.0}, 'count': {'type': 'Integer', 'value': 15}, 'id': 'XSNVof4dIF8mfadOhhnxfGm6uPb32g2RE2zi_TuBMvd3IMLSg_G0S429Z--MnN2qal7ksGMj-D6ztP25vPATmA', 'name': {'type': 'String', 'value': 'Counter Record'}}

--- update_with_action_sequence ---
After action sequence: {'score': {'type': 'Float', 'value': 40.0}, 'tags': {'value': ['initial', 'sequenced'], 'type': 'Array'}, 'name': {'value': 'Counter Record', 'type': 'String'}, 'id': 'XSNVof4dIF8mfadOhhnxfGm6uPb32g2RE2zi_TuBMvd3IMLSg_G0S429Z--MnN2qal7ksGMj-D6ztP25vPATmA', 'count': {'type': 'Integer', 'value': 115}}

--- cleanup ---
Cleaned up collection

=== Advanced CRUD example completed ===
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
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: Hm2Eeh8aeynKvmjuspRfPXJRGEoO3sYn6bIrSSQTsAYoSSzy8xK4ugyVKJprgX1nyJLmAwsrzTgTpbl5r0LM4g

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** High-performance database product
- **Price:** $99

Let me know if you need more information or if there's anything else I can assist you with!

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['llm_model', 'id', 'updated_at', 'created_at', 'role', 'chat_id', 'context_snippets', 'llm_provider', 'content', 'token_usage'])
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
✓ Created second session: E1FmdS2zK3Of4cUipn1aul2vaLMOzFk3hXgr60ytJVWEdnbxRtNZL-WiyMFYt0nQZWWmCuc9HjyVKbAzSFrSIg
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 7

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 6

=== Cleanup ===
✓ Deleted chat session: E1FmdS2zK3Of4cUipn1aul2vaLMOzFk3hXgr60ytJVWEdnbxRtNZL-WiyMFYt0nQZWWmCuc9HjyVKbAzSFrSIg
✓ Deleted chat session: Hm2Eeh8aeynKvmjuspRfPXJRGEoO3sYn6bIrSSQTsAYoSSzy8xK4ugyVKJprgX1nyJLmAwsrzTgTpbl5r0LM4g
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: KCFHWYOhkgxNUDgJ3d5Z7S7yothTQvKtwv4el8gT4cZzwynyPJbqXTI8rZjr5-InX_7uvxlZlUxDCjvl7_2g_Q

=== Sending Chat Message ===
Message ID: 0GvOKUEneWRweftcFDabFPl6UtSN0FJUmCO2TvMszhTpnlahqqQMI4D4Xfy1bccWLK0KySttkybnBBimP8ETow

=== AI Response ===
Here are the available products along with their prices:

1. **ekoDB Cloud**
   - Price: $499
   - Description: Fully managed cloud database service product

2. **ekoDB Pro**
   - Price: $299
   - Description: Enterprise edition product with advanced features

3. **ekoDB**
   - Price: $99
   - Description: A high-performance database product with AI capabilities

Execution Time: 3188ms

=== Token Usage ===
Prompt tokens: 3413
Completion tokens: 91
Total tokens: 3504

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Message Stream (SSE) Example (Python) ===

Created session: 4bhc0pDjJChDjq4ggBq_ZetC7LO9UpWc4aad1dA7Lvw-DJntaGWLpVx8bTmH_znjPrg_wMzRrc3HwigcELMF3g

Streaming response for: 'What is ekoDB?'

{"__progress":"received"}{"__progress":"generating","model":"gpt-4o-mini","provider":"openai"}ekoDB is a database tool designed as a lightweight solution for efficient persistent data storage. It is particularly suitable for scenarios that require easy integration with applications, especially in embedded systems or small-scale applications. ekoDB aims to simplify the development process by providing a straightforward API for developers while supporting functionalities like data indexing and querying.

The primary focus of ekoDB is on performance and ease of use, making it an attractive option for developers looking for a simple yet effective way to manage data in their applications without the overhead of more complex database systems.

If you have specific use cases or features of ekoDB that you're interested in, feel free to ask!

--- Stream complete ---
Message ID: 8dwlv0bQ6QXbAp0jzu1FcHjznPKlFBXkMvq_i4nT83zax1F8aTPDcuXFaUsvGauvmm8_ncGCnaaae3WNrk8UFA
Execution time: 1466ms
Context window: 128000 tokens

✓ Chat message stream example completed
✓ Client created

=== Get All Chat Models ===
OpenAI models: ['text-embedding-ada-002', 'whisper-1', 'gpt-3.5-turbo', 'tts-1', 'gpt-3.5-turbo-16k', 'gpt-4-0613', 'gpt-4', 'davinci-002', 'babbage-002', 'gpt-3.5-turbo-instruct', 'gpt-3.5-turbo-instruct-0914', 'gpt-3.5-turbo-1106', 'tts-1-hd', 'tts-1-1106', 'tts-1-hd-1106', 'text-embedding-3-small', 'text-embedding-3-large', 'gpt-3.5-turbo-0125', 'gpt-4-turbo', 'gpt-4-turbo-2024-04-09', 'gpt-4o', 'gpt-4o-2024-05-13', 'gpt-4o-mini-2024-07-18', 'gpt-4o-mini', 'gpt-4o-2024-08-06', 'omni-moderation-latest', 'omni-moderation-2024-09-26', 'o1-2024-12-17', 'o1', 'o3-mini', 'o3-mini-2025-01-31', 'gpt-4o-2024-11-20', 'gpt-4o-mini-search-preview-2025-03-11', 'gpt-4o-mini-search-preview', 'gpt-4o-transcribe', 'gpt-4o-mini-transcribe', 'o1-pro-2025-03-19', 'o1-pro', 'gpt-4o-mini-tts', 'o3-2025-04-16', 'o4-mini-2025-04-16', 'o3', 'o4-mini', 'gpt-4.1-2025-04-14', 'gpt-4.1', 'gpt-4.1-mini-2025-04-14', 'gpt-4.1-mini', 'gpt-4.1-nano-2025-04-14', 'gpt-4.1-nano', 'gpt-image-1', 'o4-mini-deep-research', 'gpt-4o-transcribe-diarize', 'o4-mini-deep-research-2025-06-26', 'gpt-5-chat-latest', 'gpt-5-2025-08-07', 'gpt-5', 'gpt-5-mini-2025-08-07', 'gpt-5-mini', 'gpt-5-nano-2025-08-07', 'gpt-5-nano', 'gpt-audio-2025-08-28', 'gpt-realtime', 'gpt-realtime-2025-08-28', 'gpt-audio', 'gpt-5-codex', 'gpt-image-1-mini', 'gpt-5-pro-2025-10-06', 'gpt-5-pro', 'gpt-audio-mini', 'gpt-audio-mini-2025-10-06', 'gpt-5-search-api', 'gpt-realtime-mini', 'sora-2', 'sora-2-pro', 'gpt-5-search-api-2025-10-14', 'gpt-5.1-chat-latest', 'gpt-5.1-2025-11-13', 'gpt-5.1', 'gpt-5.1-codex', 'gpt-5.1-codex-mini', 'gpt-5.1-codex-max', 'gpt-image-1.5', 'gpt-5.2-2025-12-11', 'gpt-5.2', 'gpt-5.2-pro-2025-12-11', 'gpt-5.2-pro', 'gpt-5.2-chat-latest', 'gpt-4o-mini-transcribe-2025-12-15', 'gpt-4o-mini-transcribe-2025-03-20', 'gpt-4o-mini-tts-2025-03-20', 'gpt-4o-mini-tts-2025-12-15', 'gpt-realtime-mini-2025-12-15', 'gpt-audio-mini-2025-12-15', 'chatgpt-image-latest', 'gpt-5.2-codex', 'gpt-5.3-codex', 'gpt-realtime-1.5', 'gpt-audio-1.5', 'gpt-4o-search-preview', 'gpt-4o-search-preview-2025-03-11', 'gpt-5.3-chat-latest', 'gpt-5.4-2026-03-05', 'gpt-5.4-pro', 'gpt-5.4-pro-2026-03-05', 'gpt-5.4', 'gpt-5.4-nano-2026-03-17', 'gpt-5.4-nano', 'gpt-5.4-mini-2026-03-17', 'gpt-5.4-mini', 'gpt-image-2', 'gpt-image-2-2026-04-21', 'gpt-5.5', 'gpt-5.5-2026-04-23', 'gpt-5.5-pro', 'gpt-5.5-pro-2026-04-23', 'chat-latest', 'gpt-realtime-translate', 'gpt-realtime-2', 'gpt-realtime-whisper', 'gpt-5.6-sol', 'gpt-5.6-terra', 'gpt-5.6-luna', 'gpt-realtime-2.1', 'gpt-realtime-2.1-mini', 'gpt-transcribe', 'gpt-live-transcribe', 'gpt-6-astra', 'gpt-image-2.5-flare', 'gpt-image-2.5-sunburst', 'gpt-image-2.5-flare-2026-09-08', 'gpt-image-2.5-sunburst-2026-09-08', 'gpt-live-1']
Anthropic models: ['claude-fable-5-1', 'claude-opus-5', 'claude-sonnet-5', 'claude-fable-5', 'claude-opus-4-8', 'claude-opus-4-7', 'claude-sonnet-4-6', 'claude-opus-4-6', 'claude-opus-4-5-20251101', 'claude-haiku-4-5-20251001', 'claude-sonnet-4-5-20250929']
Perplexity models: []
Gemini models: []

=== Get OpenAI Models ===
OpenAI models: ['text-embedding-ada-002', 'whisper-1', 'gpt-3.5-turbo', 'tts-1', 'gpt-3.5-turbo-16k', 'gpt-4-0613', 'gpt-4', 'davinci-002', 'babbage-002', 'gpt-3.5-turbo-instruct', 'gpt-3.5-turbo-instruct-0914', 'gpt-3.5-turbo-1106', 'tts-1-hd', 'tts-1-1106', 'tts-1-hd-1106', 'text-embedding-3-small', 'text-embedding-3-large', 'gpt-3.5-turbo-0125', 'gpt-4-turbo', 'gpt-4-turbo-2024-04-09', 'gpt-4o', 'gpt-4o-2024-05-13', 'gpt-4o-mini-2024-07-18', 'gpt-4o-mini', 'gpt-4o-2024-08-06', 'omni-moderation-latest', 'omni-moderation-2024-09-26', 'o1-2024-12-17', 'o1', 'o3-mini', 'o3-mini-2025-01-31', 'gpt-4o-2024-11-20', 'gpt-4o-mini-search-preview-2025-03-11', 'gpt-4o-mini-search-preview', 'gpt-4o-transcribe', 'gpt-4o-mini-transcribe', 'o1-pro-2025-03-19', 'o1-pro', 'gpt-4o-mini-tts', 'o3-2025-04-16', 'o4-mini-2025-04-16', 'o3', 'o4-mini', 'gpt-4.1-2025-04-14', 'gpt-4.1', 'gpt-4.1-mini-2025-04-14', 'gpt-4.1-mini', 'gpt-4.1-nano-2025-04-14', 'gpt-4.1-nano', 'gpt-image-1', 'o4-mini-deep-research', 'gpt-4o-transcribe-diarize', 'o4-mini-deep-research-2025-06-26', 'gpt-5-chat-latest', 'gpt-5-2025-08-07', 'gpt-5', 'gpt-5-mini-2025-08-07', 'gpt-5-mini', 'gpt-5-nano-2025-08-07', 'gpt-5-nano', 'gpt-audio-2025-08-28', 'gpt-realtime', 'gpt-realtime-2025-08-28', 'gpt-audio', 'gpt-5-codex', 'gpt-image-1-mini', 'gpt-5-pro-2025-10-06', 'gpt-5-pro', 'gpt-audio-mini', 'gpt-audio-mini-2025-10-06', 'gpt-5-search-api', 'gpt-realtime-mini', 'sora-2', 'sora-2-pro', 'gpt-5-search-api-2025-10-14', 'gpt-5.1-chat-latest', 'gpt-5.1-2025-11-13', 'gpt-5.1', 'gpt-5.1-codex', 'gpt-5.1-codex-mini', 'gpt-5.1-codex-max', 'gpt-image-1.5', 'gpt-5.2-2025-12-11', 'gpt-5.2', 'gpt-5.2-pro-2025-12-11', 'gpt-5.2-pro', 'gpt-5.2-chat-latest', 'gpt-4o-mini-transcribe-2025-12-15', 'gpt-4o-mini-transcribe-2025-03-20', 'gpt-4o-mini-tts-2025-03-20', 'gpt-4o-mini-tts-2025-12-15', 'gpt-realtime-mini-2025-12-15', 'gpt-audio-mini-2025-12-15', 'chatgpt-image-latest', 'gpt-5.2-codex', 'gpt-5.3-codex', 'gpt-realtime-1.5', 'gpt-audio-1.5', 'gpt-4o-search-preview', 'gpt-4o-search-preview-2025-03-11', 'gpt-5.3-chat-latest', 'gpt-5.4-2026-03-05', 'gpt-5.4-pro', 'gpt-5.4-pro-2026-03-05', 'gpt-5.4', 'gpt-5.4-nano-2026-03-17', 'gpt-5.4-nano', 'gpt-5.4-mini-2026-03-17', 'gpt-5.4-mini', 'gpt-image-2', 'gpt-image-2-2026-04-21', 'gpt-5.5', 'gpt-5.5-2026-04-23', 'gpt-5.5-pro', 'gpt-5.5-pro-2026-04-23', 'chat-latest', 'gpt-realtime-translate', 'gpt-realtime-2', 'gpt-realtime-whisper', 'gpt-5.6-sol', 'gpt-5.6-terra', 'gpt-5.6-luna', 'gpt-realtime-2.1', 'gpt-realtime-2.1-mini', 'gpt-transcribe', 'gpt-live-transcribe', 'gpt-6-astra', 'gpt-image-2.5-flare', 'gpt-image-2.5-sunburst', 'gpt-image-2.5-flare-2026-09-08', 'gpt-image-2.5-sunburst-2026-09-08', 'gpt-live-1']

=== Get Anthropic Models ===
Anthropic models: ['claude-fable-5-1', 'claude-opus-5', 'claude-sonnet-5', 'claude-fable-5', 'claude-opus-4-8', 'claude-opus-4-7', 'claude-sonnet-4-6', 'claude-opus-4-6', 'claude-opus-4-5-20251101', 'claude-haiku-4-5-20251001', 'claude-sonnet-4-5-20250929']

✓ Chat Models API example complete
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 1tMkYFLr8U8IvV6J1NlFcbbNMdh9ObPzpP6RFI4dlhZUR17gKbqzYO-_Yx7XXH8JLi32ASvHZsEsAIAGK_KduQ

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name:** ekoDB
  - **Description:** A high-performance database product
  - **Price:** $99

If you need further details or have any specific questions about this product, feel free to ask!

✓ Message 2 sent
  Response: The price of the product ekoDB is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: PWbKuWCBCejFnSBHvBQpfTJ3s-PU-bJfWJscVeq7I71EhYtxXbcW9ZM-Fycfo7tdxqCwaTjt7QxJn5XGaH8n4A
  Parent: 1tMkYFLr8U8IvV6J1NlFcbbNMdh9ObPzpP6RFI4dlhZUR17gKbqzYO-_Yx7XXH8JLi32ASvHZsEsAIAGK_KduQ

=== Listing Sessions ===
✓ Found 2 sessions
  Session 1: PWbKuWCBCejFnSBHvBQpfTJ3s-PU-bJfWJscVeq7I71EhYtxXbcW9ZM-Fycfo7tdxqCwaTjt7QxJn5XGaH8n4A (Untitled)
  Session 2: 1tMkYFLr8U8IvV6J1NlFcbbNMdh9ObPzpP6RFI4dlhZUR17gKbqzYO-_Yx7XXH8JLi32ASvHZsEsAIAGK_KduQ (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: PWbKuWCBCejFnSBHvBQpfTJ3s-PU-bJfWJscVeq7I71EhYtxXbcW9ZM-Fycfo7tdxqCwaTjt7QxJn5XGaH8n4A

=== Cleanup ===
✓ Deleted sessions and collection

✓ All session management operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: "XRapYBrHSWwSmLKOjufkNY88CszH1HQ1ea500dSl7wpviE5HM16F6maQxoZhe0vRFC-UzqQSqBUd5zYqZV9k_w"

=== List Collections ===
Total collections: 13
Sample collections: ['chat_goals__ek0_testing', 'schedules__ek0_testing', 'agent_function_versions__ek0_testing', 'client_collection_management_python', 'chat_raw_completions__ek0_testing']

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: False

✓ All collection management operations completed successfully
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
✓ Client created
✓ conc_demo_pay saved
✓ conc_demo_rl_fail saved
✓ conc_demo_rl_skip saved
✓ conc_demo_lock saved

Invoke them like:
  POST /api/functions/conc_demo_pay        { "idempotency_key": "...", "amount": 100 }
  POST /api/functions/conc_demo_rl_fail    { "user_id": 42 }
  POST /api/functions/conc_demo_rl_skip    { "user_id": 42 }
  POST /api/functions/conc_demo_lock       { "resource": "queue:drain" }

✓ Cleaned up demo functions
=== ekoDB Convenience Methods Example ===

=== Native Dict Creation ===
✓ Created record with plain dict: {'id': '1aBn7qJkjrCbAcnxd03iW8AtdxPrWM4_dVdh-pmOsCzRBYr4yHwe0f3R_44IUsQDffDgJCLdMYfjWi7CMRK6zA'}

=== Upsert Operation ===
✓ Upsert (update existing record): 1aBn7qJkjrCbAcnxd03iW8AtdxPrWM4_dVdh-pmOsCzRBYr4yHwe0f3R_44IUsQDffDgJCLdMYfjWi7CMRK6zA
✓ Inserted second record: CYON3TRcmWHGrmrAgmcurV_59LdCDUfIkBBPYIjWT4e511Jlm03TcSsIaD07WR88dcyLCKRC0CXBNO9V7pDSuw
✓ Upsert (update second record): CYON3TRcmWHGrmrAgmcurV_59LdCDUfIkBBPYIjWT4e511Jlm03TcSsIaD07WR88dcyLCKRC0CXBNO9V7pDSuw

=== Find One Operation ===
✓ Found user by email: {'age': {'value': 29, 'type': 'Integer'}, 'id': '1aBn7qJkjrCbAcnxd03iW8AtdxPrWM4_dVdh-pmOsCzRBYr4yHwe0f3R_44IUsQDffDgJCLdMYfjWi7CMRK6zA', 'name': {'type': 'String', 'value': 'Alice Johnson'}, 'active': {'type': 'Boolean', 'value': True}, 'email': {'value': 'alice.j@newdomain.com', 'type': 'String'}}
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
✓ Client created
✓ crypto_demo_hmac_py saved
✓ crypto_demo_aes_py saved
✓ crypto_demo_uuid_py saved
✓ crypto_demo_totp_py saved
✓ crypto_demo_encoding_py saved

Invoke them with:
  POST /api/functions/crypto_demo_hmac_py { "payload": "hi" }
  POST /api/functions/crypto_demo_aes_py { "plaintext": "secret" }
  POST /api/functions/crypto_demo_uuid_py
  POST /api/functions/crypto_demo_totp_py
  POST /api/functions/crypto_demo_encoding_py { "title": "Héllo World" }

✓ Cleaned up demo functions
=== Distinct Values Example ===

Inserting sample products...
Inserted 8 products

=== Distinct Categories (all products) ===
Found 3 distinct categories:
  - {'type': 'String', 'value': 'books'}
  - {'type': 'String', 'value': 'clothing'}
  - {'type': 'String', 'value': 'electronics'}

=== Distinct Statuses (all products) ===
Found 3 distinct statuses:
  - {'type': 'String', 'value': 'active'}
  - {'type': 'String', 'value': 'archived'}
  - {'type': 'String', 'value': 'discontinued'}

=== Distinct Statuses in Electronics ===
Found 2 distinct statuses for electronics:
  - {'type': 'String', 'value': 'active'}
  - {'type': 'String', 'value': 'discontinued'}

Cleanup done.
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: Hny2Ww1mQtVeDbXufEDTRHE-tgWZ2xjapXYg8SakhjD1n9z-RWayfCVwdPtyHcO_SQTWZRC6q6OBaWjAmoCtnA

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: r6vZ_uVgjp9QmhGHpoa0tM6jFiE8nQB9clCjsyPcvn0hvFUjyGGlWn3dMOKvbJUFSvBPEVQQk7nFBjDyoU4p7A

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
=== ekoDB as Edge Cache - Simple Example ===

Creating edge cache function...
✓ Edge cache script created: a9omHquMWxP0Y31_8dN_Fw5CSiZfd23QYKsVcsn45VANlHtzdnJWlXd9hNPJGt2eVtXWtT8wwpa6YhxXkQwwzQ

Call 1: Cache miss (fetches from API)
Response time: 465ms
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "current": {
            "interval": 900,
            "temperature_2m": 26.1,
            "time": "2026-09-16T20:30"
          },
          "current_units": {
            "interval": "seconds",
            "temperature_2m": "\u00b0C",
            "time": "iso8601"
          },
          "elevation": 32.0,
          "generationtime_ms": 0.05114078521728515,
          "latitude": 40.710335,
          "longitude": -73.99308,
          "timezone": "GMT",
          "timezone_abbreviation": "GMT",
          "utc_offset_seconds": 0
        }
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
Response time: 2ms (207.6x faster!)
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "current": {
            "interval": 900,
            "temperature_2m": 26.1,
            "time": "2026-09-16T20:30"
          },
          "current_units": {
            "interval": "seconds",
            "temperature_2m": "\u00b0C",
            "time": "iso8601"
          },
          "elevation": 32.0,
          "generationtime_ms": 0.05114078521728515,
          "latitude": 40.710335,
          "longitude": -73.99308,
          "timezone": "GMT",
          "timezone_abbreviation": "GMT",
          "utc_offset_seconds": 0
        }
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

=== ekoDB Function Composition Examples ===

📋 Setting up test data...

✅ Test data ready

📝 Example 1: Basic Function Composition

Building reusable functions that call each other...

✅ Saved reusable function: fetch_user_py
✅ Saved composed function: get_user_wrapper_py (calls fetch_user_py + projects fields)

📊 Result from composed function:
   Records: 1
   Name: {"type": "String", "value": "User 1"}
   Department: {"type": "String", "value": "engineering"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user_py (uses KV)
✅ Saved SWR function using composition: swr_user_py

First call (cache miss - will fetch from API):
   ⏱️  Duration: 87.6ms
   📊 Records: 1
   📦 Data: {
      "value": {
            "type": "Object",
            "value": {
                  "address": {
                        "city": "Gwenborough",
                        "geo": {
                 ...

Second call (cache hit - from cache):
   ⏱️  Duration: 2.3ms
   📊 Records: 1
   📦 Data: {
      "value": {
            "type": "Object",
            "value": {
                  "address": {
                        "city": "Gwenborough",
                        "geo": {
                 ...
   🚀 Cache speedup: 38.6x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

✅ Level 1 function: validate_user_py
✅ Level 2 function: fetch_slim_user_py (calls validate_user_py)
✅ Level 3 function: get_verified_user_py (calls fetch_slim_user_py)

📊 Result from 3-level nested composition:
   Records: 1
   Name: User 1
   Department: engineering

🎯 Key Benefit: Each function is independently testable and reusable!
   - validate_user: Used in 100 different workflows
   - fetch_slim_user: Used in 50 workflows
   - get_verified_user: Specific workflow


✅ Cleanup complete
✅ All composition examples completed!
client_function_contract: ok
🚀 ekoDB Functions Example (Python)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved: ATlZmhsJXMauA_WvX8twJztiZwO8lA3vG0diZTUQgujqDXNX_Zg9b0LNMiPHqe7qze5aRdgCImhDE-BsAw5TVw
📊 Found 5 active users

📝 Example 2: Parameterized Function

✅ Function saved: jkFA9qPHLiBcP0IVhQtFv0hd1aifNMKxdBz3Hz2JrYpP0a7EIJiEPImU6cYmIAevN_Y-25zzzhoxVLlfgu6x1A
📊 Found 3 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: g8EgrcNJ24ajtq6pfYHCLlEyOfu-Z5tDr3IGlrThJqY2_ucIS3uZYk0IVkxEJJlKOSGleu9T4wA_TOka104c-g
📊 Statistics: 2 groups
   {'avg_score': {'type': 'Float', 'value': 50.0}, 'count': {'type': 'Integer', 'value': 5}, 'status': {'type': 'String', 'value': 'inactive'}}

   {'avg_score': {'type': 'Float', 'value': 60.0}, 'count': {'type': 'Integer', 'value': 5}, 'status': {'type': 'String', 'value': 'active'}}

📝 Example 4: Function Management

📋 Total functions: 3
🔍 Retrieved function: Get Active Users
✏️  Function updated
🗑️  Function deleted

ℹ️  Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.

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
   {'avg_price': {'type': 'Float', 'value': 365.6666666666667}, 'category': {'type': 'String', 'value': 'Furniture'}, 'count': {'type': 'Integer', 'value': 3}}
   {'avg_price': {'type': 'Float', 'value': 367.0}, 'category': {'type': 'String', 'value': 'Electronics'}, 'count': {'type': 'Integer', 'value': 5}}
⏱️  Execution time: 0ms

📝 Example 3: Count Total Products

✅ Function saved
📊 Total products: [{'total': {'type': 'Integer', 'value': 8}}]
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

1. **Efficient Similarity Search**: They allow for fast and accurate similarity searches for high-dimensional data, making them ideal for applications like image and text retrieval.

2. **Scalability**: Vector databases can handle large datasets efficiently, enabling scaling as data grows.

3. **Handling Complex Data**: They support complex data types (e.g., embeddings from machine learning models), making them suitable for AI applications.

4. **Real-time Processing**: Many vector databases provide capabilities for real-time data processing and querying.

5. **Flexible Indexing**: Advanced indexing techniques, such as HNSW (Hierarchical Navigable Small World) and PQ (Product Quantization), enhance performance.

6. **Integration with ML Workflows**: They easily integrate with machine learning and deep learning frameworks for seamless data workflows.

7. **Multi-Modal Data**: Support for various data formats (text, images, audio) in a unified manner.

Overall, vector databases enhance the performance and capabilities of applications that require handling unstructured or semi-structured data effectively.
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embed script saved
📊 Generated 2 embeddings
   Dimensions: 1536
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All AI script examples finished!

💡 This example demonstrates:
   ✅ Chat completions with system/user messages
   ✅ Embedding generation for text
🚀 ekoDB Python Complete Functions Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Function saved: VjuXVrh23PbMvThdoVLcAhYwQEQMvnLFRxLHXqyMKuhcV9H6zRMCEgepxiZefMwvVTYh6yhS0AIbg4RwH7Y1Ow
📊 Found 2 product groups
   {'avg_price': {'type': 'Float', 'value': 575.6666666666666}, 'category': {'type': 'String', 'value': 'Electronics'}, 'count': {'type': 'Integer', 'value': 3}}
   {'avg_price': {'type': 'Float', 'value': 474.0}, 'category': {'type': 'String', 'value': 'Furniture'}, 'count': {'type': 'Integer', 'value': 2}}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   {'category': {'type': 'String', 'value': 'Furniture'}, 'count': {'type': 'Integer', 'value': 2}}
   {'category': {'type': 'String', 'value': 'Electronics'}, 'count': {'type': 'Integer', 'value': 3}}
⏱️  Execution time: 0ms

📝 Example 4: High Rating Products

✅ Function saved
📊 Found 3 products
⏱️  Execution time: 0ms

📝 Example 5: function with Parameter Definition

✅ Function saved
📊 Found 3 products
⏱️  Execution time: 0ms

📝 Example 6: Multi-Stage Pipeline (FindAll → Group → Count)

✅ Function saved
📊 Pipeline executed 3 stages
⏱️  Total execution time: 0ms

✅ All complete script examples finished!

💡 This example demonstrates ekoDB's function system:
   ✅ FindAll operations
   ✅ Group aggregations (Count, Average)
   ✅ Multi-stage pipelines (FindAll → Group → Count)
   ✅ Parameter definitions
   ✅ Function management (save, call, delete)
🧹 Cleaning up...
✅ Cleanup complete

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
   {'avg_score': {'type': 'Float', 'value': 70.0}, 'count': {'type': 'Integer', 'value': 7}, 'role': {'type': 'String', 'value': 'user'}}
   {'avg_score': {'type': 'Float', 'value': 20.0}, 'count': {'type': 'Integer', 'value': 3}, 'role': {'type': 'String', 'value': 'admin'}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD script examples finished!
🚀 ekoDB Python KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: 90zfLE2Vvz8Oi4kAZ44xEAhyIfrLS58j9JFyaYO_LYi3ETZfiHnuEKCSZH0aogylmo57GJiZDbqZlJKGgTMB7g
✅ Inserted 2 products with wrapped types

📝 Example 2: Querying and Extracting Wrapped Types

📊 Found 2 products
   • Wireless Mouse
   • Laptop Pro

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {'value': '{"type":"Object","value":{"role":"admin","userId":"user_abc"}}'}
🔍 Key exists: True
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Pattern Query

✅ Set 4 config entries
📊 Found 3 app config entries
📊 Found 4 total config entries

📝 Example 5: Combined Wrapped Types + KV Usage

✅ Inserted order: 6Aazmvc5XKKZoln4aNYuOeevDKt_EJdlF2Dlkgceocag2CHECEuhCqGSj_Y06H3_jPvOB8A9VltTLnVb2LtnNw
✅ Cached order status
📊 Quick status lookup: {'value': '{"type":"Object","value":{"status":"processing","updated_at":"2026-09-16T20:44:27.122108+00:00"}}'}

🧹 Cleaning up...
✅ Cleanup complete

✅ All KV & Wrapped Types examples completed!

💡 Key takeaways:
   ✅ Use field_* helpers for type-safe wrapped values
   ✅ field_decimal() preserves precision (no floating point errors)
   ✅ KV store is great for caching and quick lookups
   ✅ Combine KV caching with collection inserts for real workflows
🚀 ekoDB Python Search Functions Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Function saved
📊 Found 5 documents
   1. Getting Started with ekoDB (Database)
   2. Introduction to Machine Learning (AI)
   3. Database Design Principles (Database)
   4. Natural Language Processing (AI)
   5. Vector Databases Explained (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   {'category': {'type': 'String', 'value': 'AI'}, 'count': {'type': 'Integer', 'value': 2}}
   {'category': {'type': 'String', 'value': 'Database'}, 'count': {'type': 'Integer', 'value': 3}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
=== ekoDB Goal Template CRUD Example (Python) ===

--- Creating goal template ---
Created template: Data Migration (id: Pe4e_p9dLBra_VoZ2rwrgkNMGysaVY-Y10pmP8YRYzYt-cFO-7ATVkHnXCDqzFO-F5UOnVH--NmYAbJ_N601nQ)

--- Listing templates ---
Templates: {'count': 1, 'items': [{'description': {'type': 'String', 'value': 'Template for migrating data between schemas'}, 'id': 'Pe4e_p9dLBra_VoZ2rwrgkNMGysaVY-Y10pmP8YRYzYt-cFO-7ATVkHnXCDqzFO-F5UOnVH--NmYAbJ_N601nQ', 'steps': {'type': 'Array', 'value': [{'description': 'Analyze source schema'}, {'description': 'Create target schema'}, {'description': 'Migrate records'}, {'description': 'Validate results'}]}, 'title': {'type': 'String', 'value': 'Data Migration'}}]}

--- Getting template ---
Fetched: {'type': 'String', 'value': 'Data Migration'}

--- Updating template ---
Updated description: {'type': 'String', 'value': 'Updated: comprehensive data migration workflow'}

--- Deleting template ---
Template deleted successfully

✓ Goal template CRUD example completed
=== ekoDB Goals / Tasks / Agents Integration Example (Python) ===

--- goal_create ---
Created goal: Deploy v3 (id: ipL7zGst6uXqEiC3Gca81o_g57fX2klZUmLaCCtlQEC6uLk5nbF6OyxHgZBaw7nksPTJBWzrpnAQtNKlJxlUjw)

--- goal_list ---
Goals: {'count': 1, 'goals': [{'created_at': '2026-09-16T20:44:27.478983+00:00', 'description': 'Deploy version 3 to production', 'id': 'ipL7zGst6uXqEiC3Gca81o_g57fX2klZUmLaCCtlQEC6uLk5nbF6OyxHgZBaw7nksPTJBWzrpnAQtNKlJxlUjw', 'status': 'pending', 'steps': '[{"description":"Build Docker image","status":"pending"},{"description":"Run migrations","status":"pending"},{"description":"Smoke test","status":"pending"}]', 'title': 'Deploy v3', 'updated_at': '2026-09-16T20:44:27.478983+00:00'}]}

--- goal_get ---
Fetched: {'type': 'String', 'value': 'Deploy v3'}

--- goal_update ---
Updated description: {'type': 'String', 'value': 'Deploy version 3 to production (updated)'}

--- goal_search ---
Search results: {'count': 1, 'items': [{'_score': 13.2, 'created_at': {'type': 'DateTime', 'value': '2026-09-16T20:44:27.478983+00:00'}, 'description': {'type': 'String', 'value': 'Deploy version 3 to production (updated)'}, 'id': 'ipL7zGst6uXqEiC3Gca81o_g57fX2klZUmLaCCtlQEC6uLk5nbF6OyxHgZBaw7nksPTJBWzrpnAQtNKlJxlUjw', 'status': {'type': 'String', 'value': 'pending'}, 'steps': {'type': 'String', 'value': '[{"description":"Build Docker image","status":"pending"},{"description":"Run migrations","status":"pending"},{"description":"Smoke test","status":"pending"}]'}, 'title': {'type': 'String', 'value': 'Deploy v3'}, 'updated_at': {'type': 'DateTime', 'value': '2026-09-16T20:44:27.489158+00:00'}}]}

--- goal_step_start (step 0) ---
Step 0 started: {'created_at': {'type': 'DateTime', 'value': '2026-09-16T20:44:27.478983+00:00'}, 'description': {'type': 'String', 'value': 'Deploy version 3 to production (updated)'}, 'id': 'ipL7zGst6uXqEiC3Gca81o_g57fX2klZUmLaCCtlQEC6uLk5nbF6OyxHgZBaw7nksPTJBWzrpnAQtNKlJxlUjw', 'status': {'type': 'String', 'value': 'in_progress'}, 'steps': {'type': 'String', 'value': '[{"description":"Build Docker image","status":"InProgress"},{"description":"Run migrations","status":"pending"},{"description":"Smoke test","status":"pending"}]'}, 'title': {'type': 'String', 'value': 'Deploy v3'}, 'updated_at': {'type': 'DateTime', 'value': '2026-09-16T20:44:27.497773+00:00'}}

--- goal_step_complete (step 0) ---
Step 0 completed: {'created_at': {'type': 'DateTime', 'value': '2026-09-16T20:44:27.478983+00:00'}, 'description': {'type': 'String', 'value': 'Deploy version 3 to production (updated)'}, 'id': 'ipL7zGst6uXqEiC3Gca81o_g57fX2klZUmLaCCtlQEC6uLk5nbF6OyxHgZBaw7nksPTJBWzrpnAQtNKlJxlUjw', 'status': {'type': 'String', 'value': 'in_progress'}, 'steps': {'type': 'String', 'value': '[{"description":"Build Docker image","result":"Docker image built: sha256:abc123","status":"Completed"},{"description":"Run migrations","status":"pending"},{"description":"Smoke test","status":"pending"}]'}, 'title': {'type': 'String', 'value': 'Deploy v3'}, 'updated_at': {'type': 'DateTime', 'value': '2026-09-16T20:44:27.504164+00:00'}}

--- goal_step_fail (step 1) ---
Step 1 failed: {'created_at': {'type': 'DateTime', 'value': '2026-09-16T20:44:27.478983+00:00'}, 'description': {'type': 'String', 'value': 'Deploy version 3 to production (updated)'}, 'id': 'ipL7zGst6uXqEiC3Gca81o_g57fX2klZUmLaCCtlQEC6uLk5nbF6OyxHgZBaw7nksPTJBWzrpnAQtNKlJxlUjw', 'status': {'type': 'String', 'value': 'in_progress'}, 'steps': {'type': 'String', 'value': '[{"description":"Build Docker image","result":"Docker image built: sha256:abc123","status":"Completed"},{"description":"Run migrations","error":"Migration failed: column already exists","status":"Failed"},{"description":"Smoke test","status":"pending"}]'}, 'title': {'type': 'String', 'value': 'Deploy v3'}, 'updated_at': {'type': 'DateTime', 'value': '2026-09-16T20:44:27.510322+00:00'}}

--- goal_complete ---
Goal completed: {'type': 'String', 'value': 'pending_review'}

--- goal_approve ---
Goal approved: {'type': 'String', 'value': 'in_progress'}

--- goal_reject ---
Goal rejected: {'type': 'String', 'value': 'failed'}

--- task_create ---
Created task: Nightly Backup (id: VCFJgWUgGhIwRmIeGMvR22xmyHgCf0yNnWtjZMxBwk4wlKQYxzhHlADEjCXA17LqUE3V-GJckTBcHgMxHKFh-g)

--- task_list ---
Tasks: {'count': 1, 'items': [{'cron': {'type': 'String', 'value': '0 2 * * *'}, 'description': {'type': 'String', 'value': 'Backup all databases nightly'}, 'id': 'VCFJgWUgGhIwRmIeGMvR22xmyHgCf0yNnWtjZMxBwk4wlKQYxzhHlADEjCXA17LqUE3V-GJckTBcHgMxHKFh-g', 'max_consecutive_failures': {'type': 'Integer', 'value': 3}, 'name': {'type': 'String', 'value': 'Nightly Backup'}}]}

--- task_get ---
Fetched: {'type': 'String', 'value': 'Nightly Backup'}

--- task_start ---
Task started: {'type': 'String', 'value': 'running'}

--- task_succeed ---
Task succeeded: {'consecutive_failures': {'type': 'Integer', 'value': 0}, 'cron': {'type': 'String', 'value': '0 2 * * *'}, 'description': {'type': 'String', 'value': 'Backup all databases nightly'}, 'id': 'VCFJgWUgGhIwRmIeGMvR22xmyHgCf0yNnWtjZMxBwk4wlKQYxzhHlADEjCXA17LqUE3V-GJckTBcHgMxHKFh-g', 'last_error': {'type': 'Null', 'value': None}, 'last_run': {'type': 'DateTime', 'value': '2026-09-16T20:44:27.569037+00:00'}, 'max_consecutive_failures': {'type': 'Integer', 'value': 3}, 'name': {'type': 'String', 'value': 'Nightly Backup'}, 'run_count': {'type': 'Integer', 'value': 1}, 'status': {'type': 'String', 'value': 'active'}, 'updated_at': {'type': 'DateTime', 'value': '2026-09-16T20:44:27.569037+00:00'}}

--- task_pause ---
Task paused: {'type': 'String', 'value': 'paused'}

--- task_resume ---
Task resumed: {'type': 'String', 'value': 'active'}

--- task_fail ---
Task failed: {'consecutive_failures': {'type': 'Integer', 'value': 1}, 'cron': {'type': 'String', 'value': '0 2 * * *'}, 'description': {'type': 'String', 'value': 'Backup all databases nightly'}, 'id': 'VCFJgWUgGhIwRmIeGMvR22xmyHgCf0yNnWtjZMxBwk4wlKQYxzhHlADEjCXA17LqUE3V-GJckTBcHgMxHKFh-g', 'last_error': {'type': 'String', 'value': 'Disk full'}, 'last_run': {'type': 'DateTime', 'value': '2026-09-16T20:44:27.597841+00:00'}, 'max_consecutive_failures': {'type': 'Integer', 'value': 3}, 'name': {'type': 'String', 'value': 'Nightly Backup'}, 'next_run': {'type': 'DateTime', 'value': '2026-03-22T02:00:00+00:00'}, 'run_count': {'type': 'Integer', 'value': 1}, 'status': {'type': 'String', 'value': 'active'}, 'updated_at': {'type': 'DateTime', 'value': '2026-09-16T20:44:27.597841+00:00'}}

--- task_due ---
Due tasks: {'count': 0, 'items': []}

--- task_delete ---
Task deleted

--- agent_create ---
Created agent: CodeReviewer (id: DWZ--MTh5EEjZUQpW_pB3AdN6_ZkAUxLsKRgbc4VJzu6tAQkV-76s8gpylzTThQD5m-gxoSGvKx_SK3Tg06c2w)

--- agent_list ---
Agents: {'count': 1, 'items': [{'deployment_id': {'type': 'String', 'value': 'deploy_test'}, 'id': 'DWZ--MTh5EEjZUQpW_pB3AdN6_ZkAUxLsKRgbc4VJzu6tAQkV-76s8gpylzTThQD5m-gxoSGvKx_SK3Tg06c2w', 'llm_model': {'type': 'String', 'value': 'gpt-4o'}, 'name': {'type': 'String', 'value': 'CodeReviewer'}, 'system_prompt': {'type': 'String', 'value': 'You review code for correctness and style.'}, 'tools': {'type': 'Array', 'value': ['web_search', 'file_read']}}]}

--- agent_get ---
Fetched: {'type': 'String', 'value': 'CodeReviewer'}

--- agent_get_by_name ---
By name: {'type': 'String', 'value': 'CodeReviewer'}

--- agent_update ---
Updated agent prompt: {'type': 'String', 'value': 'You review code. Be concise.'}

--- agents_by_deployment ---
Agents in deploy_test: {'count': 0, 'items': []}
WARNING: agents_by_deployment omitted created agent DWZ--MTh5EEjZUQpW_pB3AdN6_ZkAUxLsKRgbc4VJzu6tAQkV-76s8gpylzTThQD5m-gxoSGvKx_SK3Tg06c2w; TODO: check/fix the server-side deployment lookup

--- agent_delete ---
Agent deleted

--- cleanup ---
Goals cleaned up

=== All goals/tasks/agents operations completed successfully ===
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
✓ py_users_register saved
✓ py_users_login saved
✓ py_users_verify_token saved

=== Auth flow defined as pure stored functions ===
Call them like:
  POST /api/functions/py_users_register { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/py_users_login    { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/py_users_verify_token { "token": "<jwt>" }

✓ Cleaned up demo functions
=== ekoDB KV Links Example (Python) ===

--- kv_set ---
Set KV key: app:config

--- insert document ---
Inserted document: zS4EmHK2bg6aYfg7ItOuM7xDWyLp9o_KJ28IcnO5UPXInlEbwe2I0PsMonYHmkAeMmZ3zlfGKO7YYh9vVlyf1A

--- kv_link ---
Linked: None

--- kv_get_links ---
Links for app:config: [{'collection': 'kv_links_example_py', 'created_at': '2026-09-16T20:44:28.121612Z', 'document_id': 'zS4EmHK2bg6aYfg7ItOuM7xDWyLp9o_KJ28IcnO5UPXInlEbwe2I0PsMonYHmkAeMmZ3zlfGKO7YYh9vVlyf1A', 'field_path': None, 'last_accessed': '2026-09-16T20:44:28.122095Z', 'metadata': {}}]

--- kv_unlink ---
Unlinked: None

--- kv_get_links (verify empty) ---
Links after unlink: []

--- cleanup ---
Cleaned up KV key and collection

=== KV links example completed ===
✓ Client created

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: {'value': '{"type":"Object","value":{"userId":123,"username":"john_doe"}}'}

=== KV Batch Set ===
✓ Batch set 3 keys
  kv_operations:py:cache:product:1: success
  kv_operations:py:cache:product:2: success
  kv_operations:py:cache:product:3: success

=== KV Batch Get ===
✓ Batch retrieved 3 values
  kv_operations:py:cache:product:1: {'name': 'Product 1', 'price': 29.99}
  kv_operations:py:cache:product:2: {'price': 39.99, 'name': 'Product 2'}
  kv_operations:py:cache:product:3: {'name': 'Product 3', 'price': 49.99}

=== KV Exists ===
Key exists: True

=== KV Find (Pattern Query) ===
Found 3 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 4

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: False

=== KV Batch Delete ===
✓ Batch deleted 3 keys
  kv_operations:py:cache:product:1: deleted
  kv_operations:py:cache:product:2: deleted
  kv_operations:py:cache:product:3: deleted

✓ All KV operations completed successfully
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
✓ py_route_admin saved
✓ py_route_user_by_id saved
✓ py_route_user_posts saved
✓ py_route_org_create_member saved

Try them with curl:
  curl http://localhost:8080/api/route/users/admin
  curl http://localhost:8080/api/route/users/42
  curl http://localhost:8080/api/route/users/42/posts/7
  curl -X POST http://localhost:8080/api/route/orgs/acme/members \
       -H 'Content-Type: application/json' -d '{"name":"alice"}'

✓ Cleaned up demo functions
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
=== ekoDB Raw Completion Stream (SSE) Example ===

--- Basic SSE Raw Completion ---
Response: The three primary colors are red, blue, and yellow.

--- Structured Output via SSE ---
JSON response: [
  {
    "name": "Mercury",
    "diameter_km": 4879
  },
  {
    "name": "Venus",
    "diameter_km": 12104
  },
  {
    "name": "Earth",
    "diameter_km": 12742
  }
]

--- Blocking HTTP (for comparison) ---
Blocking response: Hello! I hope you’re having a wonderful day.

=== Done ===
=== ekoDB Schedule Management Example (Python) ===

--- create_schedule ---
Created schedule: Nightly Report (id: 5d608bc4-a14f-49a5-8a61-51d7d3bf946e)

--- list_schedules ---
Schedules: {'count': 1, 'schedules': [{'created_at': '2026-09-16T20:44:31.921304Z', 'cron_expression': '0 0 0 * * *', 'description': 'Generate and email nightly analytics report', 'enabled': True, 'function_label': 'schedule_noop_python_64877', 'id': '5d608bc4-a14f-49a5-8a61-51d7d3bf946e', 'last_execution': None, 'name': 'Nightly Report', 'next_execution': '2026-09-17T00:00:00Z', 'parameters': {}, 'stats': {'avg_execution_time_ms': 0.0, 'failed_executions': 0, 'last_error': None, 'successful_executions': 0, 'total_executions': 0}, 'timezone': 'UTC', 'updated_at': '2026-09-16T20:44:31.921304Z'}]}

--- get_schedule ---
Fetched: Nightly Report cron=0 0 0 * * *

--- update_schedule ---
Updated cron: 0 30 1 * * *

--- trigger_schedule ---
Triggered: {'schedule_id': '5d608bc4-a14f-49a5-8a61-51d7d3bf946e', 'status': 'triggered'}

--- pause_schedule ---
Paused: enabled=False

--- resume_schedule ---
Resumed: enabled=True
Schedule deleted successfully

=== Schedule management example completed ===
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
=== Search Examples ===

Setting up test data...
✅ Test data created

1. Basic full-text search:
Found 2 results
  1. Score: 12.870, Matched: name, email
  2. Score: 6.270, Matched: name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 13.200, Matched: bio, title
  2. Score: 13.200, Matched: bio, title
  3. Score: 13.200, Matched: title, bio
  4. Score: 13.200, Matched: title, bio

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 26.400, Matched: title, bio
  2. Score: 26.400, Matched: bio, title
  3. Score: 26.400, Matched: bio, title
  4. Score: 26.400, Matched: bio, title

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 6.600, Matched: bio
  2. Score: 6.600, Matched: bio

5. Search with stemming and exact match boosting:
Found 2 results (matches: run, running, runs)
  1. Score: 6.600, Matched: bio
  2. Score: 6.600, Matched: bio

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.775
  2. Score: 0.759
  3. Score: 0.746

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.387, Matched: content, title
  2. Score: 0.873, Matched: content, title
  3. Score: 0.380, Matched:

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 6.600, Matched: title

9. Vector search with a metadata pre-filter (category = ml):
Found 2 documents in category "ml" (NLP excluded)
  1. Introduction to Machine Learning (category: ml)
  2. Deep Learning Fundamentals (category: ml)

=== Cleanup ===
✅ Deleted test collections

✅ Search examples completed!
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {'id': 'PMGeryE9Yp6jmr0AXeFsGNXDwTNm4GPgpBRoqvoakp5HNF9KvemumFOcKyTbugRyGcoK-0Zrz2L8KdAx-0iyQg'}

=== Find by ID ===
Found: {'name': {'type': 'String', 'value': 'Test Record'}, 'price': {'value': 99.99, 'type': 'Float'}, 'user_id': {'type': 'String', 'value': '550e8400-e29b-41d4-a716-446655440000'}, 'categories': {'type': 'Array', 'value': ['electronics', 'computers']}, 'metadata': {'value': {'key': 'value', 'nested': {'deep': True}}, 'type': 'Object'}, 'value': {'value': 42, 'type': 'Integer'}, 'embedding': {'type': 'Array', 'value': [0.1, 0.2, 0.3, 0.4, 0.5]}, 'active': {'value': True, 'type': 'Boolean'}, 'tags': {'value': ['tag1', 'tag2', 'tag3'], 'type': 'Array'}, 'created_at': {'value': '2026-09-16T16:44:32.397447', 'type': 'String'}, 'id': 'PMGeryE9Yp6jmr0AXeFsGNXDwTNm4GPgpBRoqvoakp5HNF9KvemumFOcKyTbugRyGcoK-0Zrz2L8KdAx-0iyQg', 'data': {'type': 'String', 'value': 'aGVsbG8gd29ybGQ='}}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): True
  price (Decimal): 99.99
  created_at (DateTime): 2026-09-16 16:44:32.397447
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): ['tag1', 'tag2', 'tag3']
  metadata (Object): {'key': 'value', 'nested': {'deep': True}}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): ['electronics', 'computers']
  data (Bytes): 11 bytes
Plain record: {'name': 'Test Record', 'price': 99.99, 'user_id': '550e8400-e29b-41d4-a716-446655440000', 'categories': ['electronics', 'computers'], 'metadata': {'key': 'value', 'nested': {'deep': True}}, 'value': 42, 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5], 'active': True, 'tags': ['tag1', 'tag2', 'tag3'], 'created_at': '2026-09-16T16:44:32.397447', 'id': 'PMGeryE9Yp6jmr0AXeFsGNXDwTNm4GPgpBRoqvoakp5HNF9KvemumFOcKyTbugRyGcoK-0Zrz2L8KdAx-0iyQg', 'data': 'aGVsbG8gd29ybGQ='}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'tags': {'type': 'Array', 'value': ['tag1', 'tag2', 'tag3']}, 'price': {'type': 'Float', 'value': 99.99}, 'id': 'PMGeryE9Yp6jmr0AXeFsGNXDwTNm4GPgpBRoqvoakp5HNF9KvemumFOcKyTbugRyGcoK-0Zrz2L8KdAx-0iyQg', 'created_at': {'type': 'String', 'value': '2026-09-16T16:44:32.397447'}, 'data': {'type': 'String', 'value': 'aGVsbG8gd29ybGQ='}, 'embedding': {'value': [0.1, 0.2, 0.3, 0.4, 0.5], 'type': 'Array'}, 'user_id': {'type': 'String', 'value': '550e8400-e29b-41d4-a716-446655440000'}, 'metadata': {'value': {'key': 'value', 'nested': {'deep': True}}, 'type': 'Object'}, 'categories': {'value': ['electronics', 'computers'], 'type': 'Array'}, 'active': {'value': True, 'type': 'Boolean'}, 'value': {'value': 100, 'type': 'Integer'}, 'name': {'value': 'Updated Record', 'type': 'String'}}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: WC0GA6qDW_YJKoye_nVytd4zY2grWeZApo8j2Y61ZNWJqBTOHJJJMzb5WuS7XCHR8EoQOUy6gl8_pw53zdHI_Q

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
🚀 ekoDB Python Client - Native SWR Function Examples

📋 Demonstrates:
   • Single-function SWR pattern (replaces 4-step pipeline)
   • Automatic cache checking, HTTP fetching, and cache setting
   • Built-in audit trail support
   • Duration string TTLs ('15m', '1h', '30s')
   • Multi-function pipeline integration
   • Dynamic TTL configuration


🧹 Cleaning up...
✓ Deleted 0 test scripts and owned SWR resources

Example 1: Basic Native SWR
────────────────────────────────────────────────────────────────────────────────
Single function replaces KvGet → If → HttpRequest → KvSet pipeline
✓ Created native SWR script: github_user_native_py (ykHljVlslX8vtbrMepBmFRWjG4Jz0jhOPl6s-VJZunHmFH6PE8v-VdFNHhR5q9K8045qSMx6lJrnTPF8t2nxtg)

First call (cache miss - will fetch from GitHub API):
  Response time: 241ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 2ms
  Speedup: 101.1x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit_py (qsNjgiNf9G_Hkw4dKiFM785b8e0S5ZkzwQqAG0TSLl0FHA5X7G1c0SMehNP6R2CBtvyNbHypbCRXHvpJiGBW0g)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail_py' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline_py (S6o6toMrSkeWhIege8aaRWxLgDLhWn3wYwQWbjuvEF3Bn3_Xw1y5r5EFvN-ob5xXcrJM8w8H2oBgP0hQWp-0uw)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users_swr_py' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache_py (kmViC8kNFLBQGSmWPUhxdR4u2dOgGBcGZmNEEUxVe88HmNQnc3v13TE4Q9j0-pCl32MNuwQTPj5L9eq3gZaJKw)
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
✓ Deleted 4 test scripts and owned SWR resources

✅ All examples completed!
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_api_user_py (XkG-QeXi7AhsImmZCGVocW9iaREmcxAL518lW7bBtthHFZMNxZLgawz3hTawvadqExfD7Hgt148o8xpqQJ1Law)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
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
Response time: 2ms (served from cache)
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
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
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: DcUPoLZbmvZQ3wB3he57DhUG9441bodlQe-nY_JfRli0RGC24jggm-RN4_09pmdZwgRSoO3jRL_cSRqQrcX3zQ
Created Bob: $500 - ID: K9rxO6g7MaXN57LNnKksOFiW9i-Qt-rSjXfkzNh4fGuL0Sh2RgMhgAgVde1dANU8SE9f8_uBNQsCAt2fNEmx3Q

=== Example 1: Begin Transaction ===
Transaction ID (server-default isolation): 566fb109-c545-4f5a-8ec5-7d7e59cf572c

=== Example 2: Operations within Transaction ===
Alice in transaction: {'name': {'type': 'String', 'value': 'Alice'}, 'balance': {'value': 800, 'type': 'Integer'}, 'id': 'DcUPoLZbmvZQ3wB3he57DhUG9441bodlQe-nY_JfRli0RGC24jggm-RN4_09pmdZwgRSoO3jRL_cSRqQrcX3zQ', 'account_id': {'value': 'ACC001', 'type': 'String'}}
Bob in transaction: {'account_id': {'type': 'String', 'value': 'ACC002'}, 'name': {'type': 'String', 'value': 'Bob'}, 'balance': {'type': 'Integer', 'value': 700}, 'id': 'K9rxO6g7MaXN57LNnKksOFiW9i-Qt-rSjXfkzNh4fGuL0Sh2RgMhgAgVde1dANU8SE9f8_uBNQsCAt2fNEmx3Q'}
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

✓ Verified committed balances: Alice=$800, Bob=$700

=== Example 5: Rollback Demo ===
New transaction: 3c2f8506-f264-44ea-a905-1bffec5fb963
Updated Bob: $700 → $600 (in transaction)
Status before rollback: Active
✓ Transaction rolled back

✓ Verified Bob remains $700 after rollback

=== Cleanup ===
✓ Deleted test account collection

✓ All client transaction examples completed
✓ Client created

=== Create User Function ===
Created user function with ID: Z0Xd-eUyoi8F8c_bVtwy0c6j4iZ8IqizqtvQ_NL-8bA0cG52JiWegf7IQy_0U_cTJZtjU76uDhUW8pxMeK8xAQ

=== Get User Function ===
Retrieved: get_active_users_py - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 1 user functions:
  - get_active_users_py: Get Active Users

=== Update User Function ===
User function updated successfully

=== Delete User Function ===
User function deleted successfully

✓ User Functions API example complete
=== WebSocket Chat Streaming Example (Python) ===

Created chat session: hRMzEHjUbl5zraLNYrsNjRomdYWoBRCQXh3DD-6vh_-fSHKNgtyeMgHzNcp04gumPCiybECK_Bdxym3JCSbVZw

Sending message: 'What is the capital of France?'
{"__progress":"received"}{"__progress":"generating","model":"gpt-4o-mini","provider":"openai"}The capital of France is Paris.

--- Stream ended ---
Message ID: vaxSMNpvrQ-GqRyeT8ujFV1JJCWSS_hci0OtejXmJaYAw-BRhyu0UlVh_OGrpGgo4Mhw9cj4Hfx-kFj_bVTdLw
Execution time: 670ms
Token usage: {'completion_tokens': 8, 'prompt_tokens': 15, 'total_tokens': 23}

Full response: {"__progress":"received"}{"__progress":"generating","model":"gpt-4o-mini","provider":"openai"}The capital of France is Paris....
=== WebSocket Subscription Example (Python) ===

✓ Authentication successful
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_py' ===
✓ Subscribed (subscription_id: sub_b46b751735cc4553a276f99ebe298f9f)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: BVB9EoWq7vQdKQRdS8TvHv9_VFZa1viI-yklNiikbBpNPl8g177Q5raqG371BCKxFuq-2z0569cMmVr_EAyL6g

  📡 Notification received:
     Event:      insert
     Collection: ws_subscribe_example_py
     Record IDs: BVB9EoWq7vQdKQRdS8TvHv9_VFZa1viI-yklNiikbBpNPl8g177Q5raqG371BCKxFuq-2z0569cMmVr_EAyL6g
     Timestamp:  2026-09-16T20:44:34.693795+00:00

Inserting record 2...
✓ Inserted: 9LkkI1RD-btBtXShWOWr1sJKcau98NZ2CbWvclaUIzr9nFOY1s17EqcKIrzhVXRbm3iC2biAQ9GpyPh2yA0yGw

  📡 Notification received:
     Event:      insert
     Record IDs: 9LkkI1RD-btBtXShWOWr1sJKcau98NZ2CbWvclaUIzr9nFOY1s17EqcKIrzhVXRbm3iC2biAQ9GpyPh2yA0yGw

=== Unsubscribing ===
✓ Unsubscribed: {'collection': 'ws_subscribe_example_py', 'found': True, 'unsubscribed': True}

=== Cleanup ===
✓ Deleted collection 'ws_subscribe_example_py'

✓ WebSocket subscription example completed successfully
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: 5mJnFjInQh99pcs0bt_IMIBkj-mC_x_tOwCOFzQHIOUEwq29mjn0pbo6wKRf1N1ScEkTMkk1gsTlelCOghXsnQ

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 5 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
=== Bypass Ripple Example ===

1. Basic insert (ripple enabled):
   Inserted with ripple: {'id': 'LEFFeBW-XODAerA3fIXmsCFiH1jJeCGRrRQNC6azD-fiKLjZLprpje4OHEvDGGDK3RxPIE9jeMh-SEO68lZiqg'}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {'id': 'fmFIzMAcQE3lel84xXLwIZTh3i2scwypYro5xNC52QkVHX0IZ3Aw04ynOwAeVk8W4ZNy82nmFMMlyA-VABdwMw'}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {'price': {'value': 150, 'type': 'Integer'}, 'id': 'LEFFeBW-XODAerA3fIXmsCFiH1jJeCGRrRQNC6azD-fiKLjZLprpje4OHEvDGGDK3RxPIE9jeMh-SEO68lZiqg', 'name': {'type': 'String', 'value': 'Product 1'}}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {'price': {'type': 'Integer', 'value': 500}, 'id': 'LEFFeBW-XODAerA3fIXmsCFiH1jJeCGRrRQNC6azD-fiKLjZLprpje4OHEvDGGDK3RxPIE9jeMh-SEO68lZiqg', 'name': {'value': 'Upsert Product', 'type': 'String'}}

✅ All bypass_ripple operations completed successfully!
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields (id, name, email only)
  Found 3 active users
  Fields returned: ['id', 'email', 'name']
  First user: {'type': 'String', 'value': 'Dave Brown'} <{'value': 'dave@example.com', 'type': 'String'}>

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ['avatar_url', 'bio', 'email', 'user_role', 'status', 'age', 'id', 'created_at', 'name']

Example 3: Complex query with projection (active users, ages 18-65)
  Found 3 active users (ages 18-65)
    - {'type': 'String', 'value': 'Dave Brown'} (age {'value': 45, 'type': 'Integer'})
    - {'value': 'Alice Johnson', 'type': 'String'} (age {'type': 'Integer', 'value': 30})
    - {'type': 'String', 'value': 'Bob Smith'} (age {'value': 25, 'type': 'Integer'})

Example 4: Query inactive users with profile fields
  Found 1 inactive users
    - {'value': 'Carol White', 'type': 'String'}: {'type': 'String', 'value': 'Manager'}

Example 5: Compare full vs projected data
  Full query:
    - 12 fields per record
    - Fields: ['name', 'age', 'secret_token', 'bio', 'created_at', 'avatar_url', 'email', 'api_key', 'user_role', 'password', 'status', 'id']
  Projected query:
    - 3 fields per record
    - Fields: ['email', 'name', 'id']
  Bandwidth savings: ~75% fewer fields

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
