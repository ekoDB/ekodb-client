make test-examples-python
📦 Ensuring Python example dependencies in .venv...
🧪 Running Python examples (direct HTTP/WebSocket)...
=== Simple CRUD Operations (Direct HTTP) ===

✓ Authentication successful

=== Insert Document ===
Inserted: {'id': 'TM-QdUR_6x6IslYUQbtugSFgLeifarLw6N7SHSUWkUQCVjEURCmVJjcLHIDAknO-ciup0ntYpzj50LRl9v9Mxw'}

=== Find by ID ===
Found: {'name': {'type': 'String', 'value': 'Test Record'}, 'active': {'type': 'Boolean', 'value': True}, 'id': 'TM-QdUR_6x6IslYUQbtugSFgLeifarLw6N7SHSUWkUQCVjEURCmVJjcLHIDAknO-ciup0ntYpzj50LRl9v9Mxw', 'value': {'value': 42, 'type': 'Integer'}}

=== Find with Query ===
Found documents: [{'name': {'type': 'String', 'value': 'Test Record'}, 'value': {'value': 42, 'type': 'Integer'}, 'active': {'type': 'Boolean', 'value': True}, 'id': 'TM-QdUR_6x6IslYUQbtugSFgLeifarLw6N7SHSUWkUQCVjEURCmVJjcLHIDAknO-ciup0ntYpzj50LRl9v9Mxw'}]

=== Update Document ===
Updated: {'name': {'type': 'String', 'value': 'Updated Record'}, 'active': {'value': True, 'type': 'Boolean'}, 'id': 'TM-QdUR_6x6IslYUQbtugSFgLeifarLw6N7SHSUWkUQCVjEURCmVJjcLHIDAknO-ciup0ntYpzj50LRl9v9Mxw', 'value': {'type': 'Integer', 'value': 100}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: vnEyQd5QIipWYXaesy5-e3jkSI5lF5Nntzy0yNsk8QDvZJ2Out94Lb4Kkh2lJWzCu3oJXVOvVhdHrebBeMgJ7A

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "type": "Success",
  "payload": {
    "data": [
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "name": {
          "type": "String",
          "value": "WebSocket Test Record"
        },
        "value": {
          "value": 42,
          "type": "Integer"
        },
        "id": "vnEyQd5QIipWYXaesy5-e3jkSI5lF5Nntzy0yNsk8QDvZJ2Out94Lb4Kkh2lJWzCu3oJXVOvVhdHrebBeMgJ7A"
      }
    ]
  },
  "messageId": "372394558"
}
✓ Retrieved 1 record via WebSocket

✓ WebSocket example completed successfully
🚀 ekoDB Functions Example (Python/HTTP)

✓ Authentication successful

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Function saved: sM68v2lTjXweuS5N0EaQyBcF1223pit5d-J-kFSzAsf9EQYOHOEGj5MJIfXAQPhh5bawnYwTSfm4qKdkkWiigA
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Function saved: RuE08wdmkE9MkXv9EMiJSxyM_YfPmSbKG3koydGlUfkBGtPm4Kl7-Cwzue59fLWMrXXdBIY80KYMHQ05z1j_ig
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: U1c1tCezuP_dW6DxqPJolTkcKFbvP2nRie6l9P8R66I-hdkQ47u8ZwHwRhaUC4DOV2XrWU-MC8XS62LpC9qttw
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {'count': {'type': 'Integer', 'value': 5}, 'max_score': {'type': 'Integer', 'value': 100}, 'status': {'value': 'active', 'type': 'String'}, 'avg_score': {'value': 60.0, 'type': 'Float'}}
   {'max_score': {'value': 90, 'type': 'Integer'}, 'avg_score': {'value': 50.0, 'type': 'Float'}, 'status': {'type': 'String', 'value': 'inactive'}, 'count': {'value': 5, 'type': 'Integer'}}

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
Retrieved value: {'type': 'Object', 'value': {'userId': 123, 'username': 'john_doe'}}

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
kv_operations:direct:py:cache:product:1: {'type': 'Object', 'value': {'name': 'Product 1', 'price': 29.99}}
kv_operations:direct:py:cache:product:2: {'type': 'Object', 'value': {'name': 'Product 2', 'price': 39.989999999999995}}
kv_operations:direct:py:cache:product:3: {'type': 'Object', 'value': {'price': 49.989999999999995, 'name': 'Product 3'}}

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
=== Collection Management (Direct HTTP) ===

✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: 8WIytDUW7n2w13lz4Qb1Dr99fiP2CzYXfQLFavlVyyd9178oo_TnFnj064ZvCDJUcW1mSc4bFRzg4TwWXI_Rvg

=== List Collections ===
Total collections: 13
Sample collections: ['chat_agent_configs__ek0_testing', 'agent_function_versions__ek0_testing', 'schedules__ek0_testing', 'functions__ek0_testing', 'chat_configurations__ek0_testing']

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
Created Alice: $1000 - ID: VLImFh2NTxDJx6Aip5SHAQFUg1IOTO6ELQyQidOaazkGZ6rTYzDuxBGLI5dPk291XEKfBWPeTBQmzH_8RxkfPw
Created Bob: $500 - ID: mZZbicGtnRYDnYqSgv_oCKC0GTVWGyNCc3irZzTCcK8FjZ3RhF6dDb6jU2YuVgP2FnxLcIXg-VndRBMJtQgswA

=== Example 1: Begin Transaction ===
Transaction ID: ff04e3d1-50e3-4a55-9f93-b28754427a11

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
New transaction: c7e8547f-738c-4246-a7e5-aa5e5ebde828
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'type': 'Integer', 'value': 700}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: RDmu5MWZdrTO-PFMdZdANPVSCeU5XtMYEjQJOUTwoX0d_mmHKuOI9hFW6polhu93HitdIUJ8hFqfyFByZtx7Ew
Created Bob: $500 - ID: rKnVOtTCZv8cur7YIPwI63yDKHnCLRLRpAKhuzYdCme3jbqWf_xyJO4-3jLz2h0tl415Jv7ceI97DeNRgjPwRA

=== Example 1: Begin Transaction ===
Transaction ID: ce4bf37f-aecf-4d19-9f78-06d69ce67523

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
Created Alice: $1000 - ID: 2YgIbBifibp8lDGVf0hzL8sgn7KaXtm_d4512WIBb0f6hL-dLvA9yCW7oAlwI_faYrbb12YA4prcqnBo4cHeyA
Created Bob: $500 - ID: sKrbpVGUav4XAXXpiyaSvRknnJCFnf84qKZOIiWSZs_IKCuEsj1xuoyLk_RRsPuhNlX2LUxUtfsyIqqQ306S0w

=== Example 1: Begin Transaction ===
Transaction ID: 7600bc9c-180d-4aec-ba1c-00fd9eb7e916

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
New transaction: 30eb8f0d-dc08-46cc-832c-3b5c7b6c7a4e
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'type': 'Integer', 'value': 700}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: IvhYLSywsp88rDwhpMKXGNo_OqB8YHbP1U0ODUfhjUJ64tQGH1iCY024VudY3aGzUauYRcj_rFwDAidepyTE4A
Created Bob: $500 - ID: 6LEfITW0a04fCDUF5H7EgLwGrIbkUXiWGqpD3kDs-4_g8sf_6XtLE1McnqHJyEChuPAMkEnFOgwTmazj1cudhA

=== Example 1: Begin Transaction ===
Transaction ID: a35c94d5-a3bf-4a31-be1d-a3931946c1ef

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
Created Alice: $1000 - ID: L4FLEcQPSjzLkTk6xZr6ZcJLc1fbFB34INXVcD7D0y4YcfVzd43ZSGKS4vW-A712_AJakReNBaKqMvbekUFFQQ
Created Bob: $500 - ID: ZGPJeClHYKpMCi3kRZ4rlbSsyDRTpM-Q-BCQTCPQTAzheRP4hUJC5s_xiDLXD1bdS9KRMK6onSrisnRkzL0Oqg

=== Example 1: Begin Transaction ===
Transaction ID: 4def801c-e469-4c61-bd91-ac0d314a28de

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
New transaction: 5ab1d015-ab97-4838-80a8-11054ca1c48a
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'value': 700, 'type': 'Integer'}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: mqw3yKMtTb-KfuFe3jglw_ERh-3JkObr9L7lpNFeWe8fYN_qU2Zt6mzz4B8_qZk2qtwFLo9-g8nr95dwrE1GJA
Created Bob: $500 - ID: hxGdhy7SIFjUovyrJp_Eev92UkYm5qGuwaseFdzl9oCIXc6CUBcWd71k8eoknvGNocBVKCAhsrVe8DfaeLnI3w

=== Example 1: Begin Transaction ===
Transaction ID: 37aab40f-f612-48eb-9d1b-648e0962c746

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
   ✅ Function saved: cEvB-vC2XGDlxOtAxl7_C-CEjJtG1vOT_v-I6ie23hrAZVnbevpnO2UaJ854spKW6b4wHtAVuyD_-vozZB2b8A

2️⃣ Calling function (Insert + Verify)...
   ✅ function executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: EP55Hdi3uQCblEzQCISkkYD5u7z1aU3kD_DxDEJZtwskUn2Rcv72yxIDRQnS9dG5rEX7rj2YGn_JTGtVaUvFJA
   📋 Name: {'value': 'Alice Smith', 'type': 'String'}
   📋 Email: {'type': 'String', 'value': 'alice@example.com'}
   📋 Status: {'value': 'pending', 'type': 'String'}
   📋 Credits: {'value': 0, 'type': 'Integer'}

============================================================
📝 function 2: Query + Update + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: 1gNXWpQ-JcyL9mHbaqM-YxR9pT4c0AVWLm-wmvpuKaElJq6AGPqrgCmUhK118aIdQKOFpjrhEyc8y3czmOCxSw

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
   ✅ Function saved: _ixl6NC74KZeIQ9mtt9s30unMakTLGhhRr4sByQ-S2eaP5ZqJQRYURT_URBYSALRSwRRty4C_mBVs_dK99eAow

2️⃣ Calling function (Query + Update Credits + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {'value': 100, 'type': 'Integer'}
   📋 Status: {'type': 'String', 'value': 'active'}
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}

============================================================
📝 function 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: hBoRVgjU2Wu7jIjO8VbUiJ9ljWZe9XMcOXOdgYNFEpQc6NwZRddoUsyp_EqD04VcstYp_qDmGHPDYMs7zB8fYQ

2️⃣ Calling function (Query + Delete + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: cEvB-vC2XGDlxOtAxl7_...
   ✅ Deleted script: 1gNXWpQ-JcyL9mHbaqM-...
   ✅ Deleted script: _ixl6NC74KZeIQ9mtt9s...
   ✅ Deleted script: hBoRVgjU2Wu7jIjO8VbU...
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
  Output: Document ID = vNkdU5uF5FZ0y-VqoEUtKMowZL0CrFWQsvY1ALkNwMwsjaj_eWvLwgC_vwDbVuuh4MGXSbmLp4Gdx9jenSu5Zg
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: find_by_id(vNkdU5uF5FZ0y-VqoEUtKMowZL0CrFWQsvY1ALkNwMwsjaj_eWvLwgC_vwDbVuuh4MGXSbmLp4Gdx9jenSu5Zg)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: find_by_id(vNkdU5uF5FZ0y-VqoEUtKMowZL0CrFWQsvY1ALkNwMwsjaj_eWvLwgC_vwDbVuuh4MGXSbmLp4Gdx9jenSu5Zg)
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
  Output: Document ID = 5PzrkBB84GZ3Rw-oEBp9uUbX8nby1cuihKBfnrzHA-ms9gw_oQTBu-jlIFJSSgDkoyhWLERdkB3DocJTaD9Jpw
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: find_by_id(5PzrkBB84GZ3Rw-oEBp9uUbX8nby1cuihKBfnrzHA-ms9gw_oQTBu-jlIFJSSgDkoyhWLERdkB3DocJTaD9Jpw)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: find_by_id(5PzrkBB84GZ3Rw-oEBp9uUbX8nby1cuihKBfnrzHA-ms9gw_oQTBu-jlIFJSSgDkoyhWLERdkB3DocJTaD9Jpw)
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
    Finished `release` profile [optimized] target(s) in 0.19s
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
bash: warning: setlocale: LC_ALL: cannot change locale (C.UTF-8): No such file or directory
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== ekoDB Advanced CRUD Example (Python) ===

--- insert ---
Inserted: aqM86JhjkPW7sw4LwSnTbLZeBgdIJRUQlFE6Aj74WW40KoIESgVk9aVwU3NWjP5F7G8Vf2jAHwjspAjPcv5ndw

--- update_with_action (increment) ---
After increment count by 5: {'id': 'aqM86JhjkPW7sw4LwSnTbLZeBgdIJRUQlFE6Aj74WW40KoIESgVk9aVwU3NWjP5F7G8Vf2jAHwjspAjPcv5ndw', 'name': {'type': 'String', 'value': 'Counter Record'}, 'tags': {'type': 'Array', 'value': ['initial']}, 'count': {'type': 'Integer', 'value': 15}, 'score': {'value': 50.0, 'type': 'Float'}}

--- update_with_action (decrement) ---
After decrement score by 10: {'name': {'type': 'String', 'value': 'Counter Record'}, 'id': 'aqM86JhjkPW7sw4LwSnTbLZeBgdIJRUQlFE6Aj74WW40KoIESgVk9aVwU3NWjP5F7G8Vf2jAHwjspAjPcv5ndw', 'score': {'value': 40.0, 'type': 'Float'}, 'tags': {'type': 'Array', 'value': ['initial']}, 'count': {'type': 'Integer', 'value': 15}}

--- update_with_action_sequence ---
After action sequence: {'id': 'aqM86JhjkPW7sw4LwSnTbLZeBgdIJRUQlFE6Aj74WW40KoIESgVk9aVwU3NWjP5F7G8Vf2jAHwjspAjPcv5ndw', 'name': {'type': 'String', 'value': 'Counter Record'}, 'tags': {'value': ['initial', 'sequenced'], 'type': 'Array'}, 'score': {'value': 40.0, 'type': 'Float'}, 'count': {'type': 'Integer', 'value': 115}}

--- cleanup ---
Cleaned up collection

=== Advanced CRUD example completed ===
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: LnBuzLJPwASGqnCQlUmhO-1c-qo1rcC7OuzGVFe0vmhLkxk_HNpdR-SGcLd6nLT15m11-fTUmBM2R0N8KXOWqQ

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** High-performance database product
- **Price:** $99

If you need more information or have any other questions, feel free to ask!

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['llm_provider', 'token_usage', 'chat_id', 'id', 'updated_at', 'content', 'created_at', 'context_snippets', 'llm_model', 'role'])
Debug: First message role: {'value': 'assistant', 'type': 'String'}
=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of ekoDB is $99. If you have any other questions or need further assistance, feel free to ask!

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: PblUNwUt_wNcVUJhMbyomvEWuBtNCEiXbReOivh-2e6abH23PqdXpOJNTJg6RIJOZTcjgq9Z3CTFDkPoORg7kQ
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 7

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 6

=== Cleanup ===
✓ Deleted chat session: PblUNwUt_wNcVUJhMbyomvEWuBtNCEiXbReOivh-2e6abH23PqdXpOJNTJg6RIJOZTcjgq9Z3CTFDkPoORg7kQ
✓ Deleted chat session: LnBuzLJPwASGqnCQlUmhO-1c-qo1rcC7OuzGVFe0vmhLkxk_HNpdR-SGcLd6nLT15m11-fTUmBM2R0N8KXOWqQ
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: d8LtJHpYC_tm702s4o4sgae4mXMsGk5ATuVD8XQxHZV3cprhe4-YQYp61TL-c-AopjE2vl8RqAjOO3r1n20l8g

=== Sending Chat Message ===
Message ID: 9IjPNKJ5vZnKlE2rpTcWxrPpTO8udDzuThHvRDt4KQrovnYYluhDlay2I5ejh644jlH9PeknDfvSieDBX5uvdA

=== AI Response ===
The available products and their prices are as follows:

1. **ekoDB Cloud**: $499
   - Description: Fully managed cloud database service product

2. **ekoDB**: $99
   - Description: A high-performance database product with AI capabilities

3. **ekoDB Pro**: $299
   - Description: Enterprise edition product with advanced features

Execution Time: 2239ms

=== Token Usage ===
Prompt tokens: 3413
Completion tokens: 82
Total tokens: 3495

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ Chat completed successfully
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== ekoDB Chat Message Stream (SSE) Example (Python) ===

Created session: d2jbEsqufBGFUSSPpH8BB2fd4CXpZNhqpCEmA8abcXgB3Jzg8z05ZsjBxJVgwmVlaHmqYy8ifqrTSWa0RYTX8A

Streaming response for: 'What is ekoDB?'

{"__progress":"received"}{"__progress":"generating","model":"gpt-4o-mini","provider":"openai"}EkoDB is a high-performance, embedded database specifically designed for use with the Elixir programming language. It focuses on providing fast access to data while being lightweight and easy to integrate into Elixir applications. EkoDB supports transactional capabilities and is optimized for concurrent access, making it suitable for applications that require efficient data storage and retrieval.

EkoDB is particularly suitable for scenarios where you want a database that can be embedded within an application without the need for a separate database server, thus reducing the overhead of managing a standalone database system. Its design leverages the strengths of the BEAM virtual machine, which Elixir and Erlang run on, providing excellent performance under concurrent workloads.

If you need more specific details or have particular questions about EkoDB, feel free to ask!

--- Stream complete ---
Message ID: 7oTL2uYA-skM95Dt4TndZ-L42OhbZucqA1W3GC2dFVnRbBVfVe5aMGgqKGI598Iz6xztDo7GfzTMv5JVIZSmtg
Execution time: 2307ms
Context window: 128000 tokens

✓ Chat message stream example completed
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: oU2biI16LkKeN5i_IyBuQxxuujQp6RtWRlBHUnF17Bkgv3dcM20VRUwVz578P3nj7KmMhfGg9ilOR_iq5jI-UA

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: A high-performance database product
- **Price**: $99

If you need more details or have other questions, feel free to ask!

✓ Message 2 sent
  Response: The price of ekoDB is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: D5-jZ63H9q4tt_Rv8EoZUi7qV_LxdHn0Fthwil8JBmSVd30GcPAM4b2zer9Q6nOP-7mTnrkumglcFawMr8nb3g
  Parent: oU2biI16LkKeN5i_IyBuQxxuujQp6RtWRlBHUnF17Bkgv3dcM20VRUwVz578P3nj7KmMhfGg9ilOR_iq5jI-UA

=== Listing Sessions ===
✓ Found 2 sessions
  Session 1: D5-jZ63H9q4tt_Rv8EoZUi7qV_LxdHn0Fthwil8JBmSVd30GcPAM4b2zer9Q6nOP-7mTnrkumglcFawMr8nb3g (Untitled)
  Session 2: oU2biI16LkKeN5i_IyBuQxxuujQp6RtWRlBHUnF17Bkgv3dcM20VRUwVz578P3nj7KmMhfGg9ilOR_iq5jI-UA (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: D5-jZ63H9q4tt_Rv8EoZUi7qV_LxdHn0Fthwil8JBmSVd30GcPAM4b2zer9Q6nOP-7mTnrkumglcFawMr8nb3g

=== Cleanup ===
✓ Deleted sessions and collection

✓ All session management operations completed successfully
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: "_880851ZpRaOlHOpj6IL3C_3AeBEaD0VYnKnMDK_iL5Eq85tK_Cci6eO90jToLPKMWcCLVofcjJIDc-RAjufVw"

=== List Collections ===
Total collections: 13
Sample collections: ['chat_agent_configs__ek0_testing', 'agent_function_versions__ek0_testing', 'client_collection_management_python', 'schedules__ek0_testing', 'functions__ek0_testing']

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: False

✓ All collection management operations completed successfully
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== ekoDB Convenience Methods Example ===

=== Native Dict Creation ===
✓ Created record with plain dict: {'id': 'UuZqbQLFX9209NKfyFPQRaMjGyC82kaMangHnRkiat83hVKIZFihD5Z04YWjpLJARCTS59-I7f7soG3ffincAA'}

=== Upsert Operation ===
✓ Upsert (update existing record): UuZqbQLFX9209NKfyFPQRaMjGyC82kaMangHnRkiat83hVKIZFihD5Z04YWjpLJARCTS59-I7f7soG3ffincAA
✓ Inserted second record: 4imi8hLOz-1OJRE-OsPEy4f_rSuAC_MWElFRrZW6jfE--Ct3gZosPXykkkk5zd_MZR2OrZci2GSJFegMHG9sdw
✓ Upsert (update second record): 4imi8hLOz-1OJRE-OsPEy4f_rSuAC_MWElFRrZW6jfE--Ct3gZosPXykkkk5zd_MZR2OrZci2GSJFegMHG9sdw

=== Find One Operation ===
✓ Found user by email: {'active': {'type': 'Boolean', 'value': True}, 'age': {'type': 'Integer', 'value': 29}, 'id': 'UuZqbQLFX9209NKfyFPQRaMjGyC82kaMangHnRkiat83hVKIZFihD5Z04YWjpLJARCTS59-I7f7soG3ffincAA', 'email': {'type': 'String', 'value': 'alice.j@newdomain.com'}, 'name': {'type': 'String', 'value': 'Alice Johnson'}}
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: ama0yiTT93xI7mRlE5SzKYmmEPx7Xf-58ooldS9drtPa9pgW216FbV4uELq1W6kWydtVg-kBopFzaDAWNECWHg

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: lpPdHomV0qnCX-N1NFzEyAluq6LDjxupYRSpT-L75-oPy36pCX67ItRTk9Bcfi6621iPrO6frzZi2gzNsbz79Q

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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== ekoDB as Edge Cache - Simple Example ===

Creating edge cache function...
✓ Edge cache script created: 04t0Hil8MnDnr6wDBmJFm3wfgbGFPDjBWZ4qlR618H9sqSGVJQMWEpZFE9dnuDUtUOnkswEs49DbT4yzRSzf-w

Call 1: Cache miss (fetches from API)
Response time: 439ms
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "current": {
            "interval": 900,
            "temperature_2m": 20.0,
            "time": "2026-09-16T13:45"
          },
          "current_units": {
            "interval": "seconds",
            "temperature_2m": "\u00b0C",
            "time": "iso8601"
          },
          "elevation": 32.0,
          "generationtime_ms": 0.03254413604736328,
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
Response time: 12ms (37.0x faster!)
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "current": {
            "interval": 900,
            "temperature_2m": 20.0,
            "time": "2026-09-16T13:45"
          },
          "current_units": {
            "interval": "seconds",
            "temperature_2m": "\u00b0C",
            "time": "iso8601"
          },
          "elevation": 32.0,
          "generationtime_ms": 0.03254413604736328,
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

perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
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
   ⏱️  Duration: 102.7ms
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
   ⏱️  Duration: 10.2ms
   📊 Records: 1
   📦 Data: {
      "value": {
            "type": "Object",
            "value": {
                  "address": {
                        "city": "Gwenborough",
                        "geo": {
                 ...
   🚀 Cache speedup: 10.0x faster!

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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
client_function_contract: ok
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
🚀 ekoDB Functions Example (Python)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved: uBTrqc9g8xgrY3yfXCvwj0RjoBpGV8eABjKx6Pt9NN8qPNBN2argQTmPhaUdABFHNzn6vPr08kI-8lGYKZin9g
📊 Found 5 active users

📝 Example 2: Parameterized Function

✅ Function saved: Q7vNTMfUB0R7p18d8r-3LzRJQmv5__c5jaK3Vm9WSM_IBXeS88RDw9aoxnIiCNngKSF_sctZhxTtqTy0wwoXWA
📊 Found 3 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: K-SpJpHmEcyordqYtWLwxiadH5RjiQBBQvZzhU-ejDxtwb17Q_WfYEAEPLeBQVLHp7SUPwb81flo-VJONZesgg
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
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
   {'avg_price': {'type': 'Float', 'value': 367.0}, 'category': {'type': 'String', 'value': 'Electronics'}, 'count': {'type': 'Integer', 'value': 5}}
   {'avg_price': {'type': 'Float', 'value': 365.6666666666667}, 'category': {'type': 'String', 'value': 'Furniture'}, 'count': {'type': 'Integer', 'value': 3}}
⏱️  Execution time: 0ms

📝 Example 3: Count Total Products

✅ Function saved
📊 Total products: [{'total': {'type': 'Integer', 'value': 8}}]
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced script examples finished!
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
🚀 ekoDB Python AI Functions Example

📋 Setting up test data...
✅ Created 2 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
🤖 AI Response:
   Vector databases offer several benefits:

1. **Efficient Similarity Search**: They enable fast retrieval of similar items using vector embeddings, which is essential for applications like image and text similarity.

2. **High Dimensional Data Handling**: They can efficiently manage and query high-dimensional data, making them suitable for machine learning and AI applications.

3. **Scalability**: Vector databases are designed to scale with large datasets, providing robust performance as data grows.

4. **Real-Time Processing**: They support real-time data processing and querying, which is critical for dynamic applications.

5. **Advanced Indexing Techniques**: Use of specialized indexing techniques (e.g., Annoy, HNSW) to improve search speed and accuracy.

6. **Integration with ML Models**: They easily integrate with machine learning frameworks, enabling seamless workflows from model training to deployment.

7. **Diverse Data Types**: Ability to store and query various data types (text, images, audio) in a unified manner.

8. **Enhanced Recommendation Systems**: Improve recommendation systems through better user-item similarity calculations.

These advantages make vector databases particularly valuable in fields like natural language processing, computer vision, and recommendation systems.
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
🚀 ekoDB Python Complete Functions Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Function saved: 1Az68NmOcpikvGyF8TGvTr5s4JQQGJ5eMkg_7NjHKI7EGJwjjijYgpSjCG2m8OtzogGgfqLHxrNle1YUA0KIng
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
   {'category': {'type': 'String', 'value': 'Electronics'}, 'count': {'type': 'Integer', 'value': 3}}
   {'category': {'type': 'String', 'value': 'Furniture'}, 'count': {'type': 'Integer', 'value': 2}}
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

perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
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
   {'avg_score': {'type': 'Float', 'value': 20.0}, 'count': {'type': 'Integer', 'value': 3}, 'role': {'type': 'String', 'value': 'admin'}}
   {'avg_score': {'type': 'Float', 'value': 70.0}, 'count': {'type': 'Integer', 'value': 7}, 'role': {'type': 'String', 'value': 'user'}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD script examples finished!
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
🚀 ekoDB Python KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: cO6ej5ddtcabksB-r6i6VrV3j_CQKfb6hjSXIg_28tMnD6knMeUHFRd2AZ08Em7QQgMuZHRqnipjf0zAJNQuaA
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

✅ Inserted order: _Nc7sXcocnckJf_QLGbzlvmbjs9LxTIAVeFHg6cXTKZaCk8qqSsP2PSY4ExzDJet_5QLL1zKCyc2SZMLhO1q2w
✅ Cached order status
📊 Quick status lookup: {'value': '{"type":"Object","value":{"status":"processing","updated_at":"2026-09-16T13:46:38.975619+00:00"}}'}

🧹 Cleaning up...
✅ Cleanup complete

✅ All KV & Wrapped Types examples completed!

💡 Key takeaways:
   ✅ Use field_* helpers for type-safe wrapped values
   ✅ field_decimal() preserves precision (no floating point errors)
   ✅ KV store is great for caching and quick lookups
   ✅ Combine KV caching with collection inserts for real workflows
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
🚀 ekoDB Python Search Functions Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Function saved
📊 Found 5 documents
   1. Vector Databases Explained (Database)
   2. Getting Started with ekoDB (Database)
   3. Natural Language Processing (AI)
   4. Introduction to Machine Learning (AI)
   5. Database Design Principles (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   {'category': {'type': 'String', 'value': 'Database'}, 'count': {'type': 'Integer', 'value': 3}}
   {'category': {'type': 'String', 'value': 'AI'}, 'count': {'type': 'Integer', 'value': 2}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== ekoDB Goal Template CRUD Example (Python) ===

--- Creating goal template ---
Created template: Data Migration (id: AuHv6TdHjDt1-RbjlghEBYgJHaYo3sDUs2iwKOUsn5ZIBT6rYedHCuJvhQAM6JIGkx9uEcWhiGnuHLKdCCKEJg)

--- Listing templates ---
Templates: {'count': 1, 'items': [{'description': {'type': 'String', 'value': 'Template for migrating data between schemas'}, 'id': 'AuHv6TdHjDt1-RbjlghEBYgJHaYo3sDUs2iwKOUsn5ZIBT6rYedHCuJvhQAM6JIGkx9uEcWhiGnuHLKdCCKEJg', 'steps': {'type': 'Array', 'value': [{'description': 'Analyze source schema'}, {'description': 'Create target schema'}, {'description': 'Migrate records'}, {'description': 'Validate results'}]}, 'title': {'type': 'String', 'value': 'Data Migration'}}]}

--- Getting template ---
Fetched: {'type': 'String', 'value': 'Data Migration'}

--- Updating template ---
Updated description: {'type': 'String', 'value': 'Updated: comprehensive data migration workflow'}

--- Deleting template ---
Template deleted successfully

✓ Goal template CRUD example completed
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== ekoDB Goals / Tasks / Agents Integration Example (Python) ===

--- goal_create ---
Created goal: Deploy v3 (id: bDL5pHK0PGrg0tSGOgbzgYc4LEyEdawMZEjYIagxXGNxm2lsEEwePDGcjH1GA6mfDguiQxzmCfnV-95Bzk8Neg)

--- goal_list ---
Goals: {'count': 1, 'goals': [{'created_at': '2026-09-16T13:46:39.430861+00:00', 'description': 'Deploy version 3 to production', 'id': 'bDL5pHK0PGrg0tSGOgbzgYc4LEyEdawMZEjYIagxXGNxm2lsEEwePDGcjH1GA6mfDguiQxzmCfnV-95Bzk8Neg', 'status': 'pending', 'steps': '[{"description":"Build Docker image","status":"pending"},{"description":"Run migrations","status":"pending"},{"description":"Smoke test","status":"pending"}]', 'title': 'Deploy v3', 'updated_at': '2026-09-16T13:46:39.430861+00:00'}]}

--- goal_get ---
Fetched: {'type': 'String', 'value': 'Deploy v3'}

--- goal_update ---
Updated description: {'type': 'String', 'value': 'Deploy version 3 to production (updated)'}

--- goal_search ---
Search results: {'count': 1, 'items': [{'_score': 13.2, 'created_at': {'type': 'DateTime', 'value': '2026-09-16T13:46:39.430861+00:00'}, 'description': {'type': 'String', 'value': 'Deploy version 3 to production (updated)'}, 'id': 'bDL5pHK0PGrg0tSGOgbzgYc4LEyEdawMZEjYIagxXGNxm2lsEEwePDGcjH1GA6mfDguiQxzmCfnV-95Bzk8Neg', 'status': {'type': 'String', 'value': 'pending'}, 'steps': {'type': 'String', 'value': '[{"description":"Build Docker image","status":"pending"},{"description":"Run migrations","status":"pending"},{"description":"Smoke test","status":"pending"}]'}, 'title': {'type': 'String', 'value': 'Deploy v3'}, 'updated_at': {'type': 'DateTime', 'value': '2026-09-16T13:46:39.451770+00:00'}}]}

--- goal_step_start (step 0) ---
Step 0 started: {'created_at': {'type': 'DateTime', 'value': '2026-09-16T13:46:39.430861+00:00'}, 'description': {'type': 'String', 'value': 'Deploy version 3 to production (updated)'}, 'id': 'bDL5pHK0PGrg0tSGOgbzgYc4LEyEdawMZEjYIagxXGNxm2lsEEwePDGcjH1GA6mfDguiQxzmCfnV-95Bzk8Neg', 'status': {'type': 'String', 'value': 'in_progress'}, 'steps': {'type': 'String', 'value': '[{"description":"Build Docker image","status":"InProgress"},{"description":"Run migrations","status":"pending"},{"description":"Smoke test","status":"pending"}]'}, 'title': {'type': 'String', 'value': 'Deploy v3'}, 'updated_at': {'type': 'DateTime', 'value': '2026-09-16T13:46:39.462377+00:00'}}

--- goal_step_complete (step 0) ---
Step 0 completed: {'created_at': {'type': 'DateTime', 'value': '2026-09-16T13:46:39.430861+00:00'}, 'description': {'type': 'String', 'value': 'Deploy version 3 to production (updated)'}, 'id': 'bDL5pHK0PGrg0tSGOgbzgYc4LEyEdawMZEjYIagxXGNxm2lsEEwePDGcjH1GA6mfDguiQxzmCfnV-95Bzk8Neg', 'status': {'type': 'String', 'value': 'in_progress'}, 'steps': {'type': 'String', 'value': '[{"description":"Build Docker image","result":"Docker image built: sha256:abc123","status":"Completed"},{"description":"Run migrations","status":"pending"},{"description":"Smoke test","status":"pending"}]'}, 'title': {'type': 'String', 'value': 'Deploy v3'}, 'updated_at': {'type': 'DateTime', 'value': '2026-09-16T13:46:39.468647+00:00'}}

--- goal_step_fail (step 1) ---
Step 1 failed: {'created_at': {'type': 'DateTime', 'value': '2026-09-16T13:46:39.430861+00:00'}, 'description': {'type': 'String', 'value': 'Deploy version 3 to production (updated)'}, 'id': 'bDL5pHK0PGrg0tSGOgbzgYc4LEyEdawMZEjYIagxXGNxm2lsEEwePDGcjH1GA6mfDguiQxzmCfnV-95Bzk8Neg', 'status': {'type': 'String', 'value': 'in_progress'}, 'steps': {'type': 'String', 'value': '[{"description":"Build Docker image","result":"Docker image built: sha256:abc123","status":"Completed"},{"description":"Run migrations","error":"Migration failed: column already exists","status":"Failed"},{"description":"Smoke test","status":"pending"}]'}, 'title': {'type': 'String', 'value': 'Deploy v3'}, 'updated_at': {'type': 'DateTime', 'value': '2026-09-16T13:46:39.475090+00:00'}}

--- goal_complete ---
Goal completed: {'type': 'String', 'value': 'pending_review'}

--- goal_approve ---
Goal approved: {'type': 'String', 'value': 'in_progress'}

--- goal_reject ---
Goal rejected: {'type': 'String', 'value': 'failed'}

--- task_create ---
Created task: Nightly Backup (id: STMTtV3OhmZfvVi1jECkojYI8Poh8Y0RKDuGLpjKsOX7ga0WiwMGYmSnk0Ybz7pQ7XNoaBUxUQtldeLQKndG1w)

--- task_list ---
Tasks: {'count': 1, 'items': [{'cron': {'type': 'String', 'value': '0 2 * * *'}, 'description': {'type': 'String', 'value': 'Backup all databases nightly'}, 'id': 'STMTtV3OhmZfvVi1jECkojYI8Poh8Y0RKDuGLpjKsOX7ga0WiwMGYmSnk0Ybz7pQ7XNoaBUxUQtldeLQKndG1w', 'max_consecutive_failures': {'type': 'Integer', 'value': 3}, 'name': {'type': 'String', 'value': 'Nightly Backup'}}]}

--- task_get ---
Fetched: {'type': 'String', 'value': 'Nightly Backup'}

--- task_start ---
Task started: {'type': 'String', 'value': 'running'}

--- task_succeed ---
Task succeeded: {'consecutive_failures': {'type': 'Integer', 'value': 0}, 'cron': {'type': 'String', 'value': '0 2 * * *'}, 'description': {'type': 'String', 'value': 'Backup all databases nightly'}, 'id': 'STMTtV3OhmZfvVi1jECkojYI8Poh8Y0RKDuGLpjKsOX7ga0WiwMGYmSnk0Ybz7pQ7XNoaBUxUQtldeLQKndG1w', 'last_error': {'type': 'Null', 'value': None}, 'last_run': {'type': 'DateTime', 'value': '2026-09-16T13:46:39.546563+00:00'}, 'max_consecutive_failures': {'type': 'Integer', 'value': 3}, 'name': {'type': 'String', 'value': 'Nightly Backup'}, 'run_count': {'type': 'Integer', 'value': 1}, 'status': {'type': 'String', 'value': 'active'}, 'updated_at': {'type': 'DateTime', 'value': '2026-09-16T13:46:39.546563+00:00'}}

--- task_pause ---
Task paused: {'type': 'String', 'value': 'paused'}

--- task_resume ---
Task resumed: {'type': 'String', 'value': 'active'}

--- task_fail ---
Task failed: {'consecutive_failures': {'type': 'Integer', 'value': 1}, 'cron': {'type': 'String', 'value': '0 2 * * *'}, 'description': {'type': 'String', 'value': 'Backup all databases nightly'}, 'id': 'STMTtV3OhmZfvVi1jECkojYI8Poh8Y0RKDuGLpjKsOX7ga0WiwMGYmSnk0Ybz7pQ7XNoaBUxUQtldeLQKndG1w', 'last_error': {'type': 'String', 'value': 'Disk full'}, 'last_run': {'type': 'DateTime', 'value': '2026-09-16T13:46:39.580430+00:00'}, 'max_consecutive_failures': {'type': 'Integer', 'value': 3}, 'name': {'type': 'String', 'value': 'Nightly Backup'}, 'next_run': {'type': 'DateTime', 'value': '2026-03-22T02:00:00+00:00'}, 'run_count': {'type': 'Integer', 'value': 1}, 'status': {'type': 'String', 'value': 'active'}, 'updated_at': {'type': 'DateTime', 'value': '2026-09-16T13:46:39.580430+00:00'}}

--- task_due ---
Due tasks: {'count': 0, 'items': []}

--- task_delete ---
Task deleted

--- agent_create ---
Created agent: CodeReviewer (id: XIDwOsl1zc59N9RYvoqX_7GkCvQ2Fs7LqAmJHrTh7MmEeXtB4E94coLPiTS9mJ57mBiviiyxAzEpehW_L8PYIA)

--- agent_list ---
Agents: {'count': 1, 'items': [{'deployment_id': {'type': 'String', 'value': 'deploy_test'}, 'id': 'XIDwOsl1zc59N9RYvoqX_7GkCvQ2Fs7LqAmJHrTh7MmEeXtB4E94coLPiTS9mJ57mBiviiyxAzEpehW_L8PYIA', 'llm_model': {'type': 'String', 'value': 'gpt-4o'}, 'name': {'type': 'String', 'value': 'CodeReviewer'}, 'system_prompt': {'type': 'String', 'value': 'You review code for correctness and style.'}, 'tools': {'type': 'Array', 'value': ['web_search', 'file_read']}}]}

--- agent_get ---
Fetched: {'type': 'String', 'value': 'CodeReviewer'}

--- agent_get_by_name ---
By name: {'type': 'String', 'value': 'CodeReviewer'}

--- agent_update ---
Updated agent prompt: {'type': 'String', 'value': 'You review code. Be concise.'}

--- agents_by_deployment ---
Agents in deploy_test: {'count': 0, 'items': []}
WARNING: agents_by_deployment omitted created agent XIDwOsl1zc59N9RYvoqX_7GkCvQ2Fs7LqAmJHrTh7MmEeXtB4E94coLPiTS9mJ57mBiviiyxAzEpehW_L8PYIA; TODO: check/fix the server-side deployment lookup

--- agent_delete ---
Agent deleted

--- cleanup ---
Goals cleaned up

=== All goals/tasks/agents operations completed successfully ===
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== ekoDB KV Links Example (Python) ===

--- kv_set ---
Set KV key: app:config

--- insert document ---
Inserted document: U4z5TpCZJQfXc3du_-UM0WXrbXOUya1mezFtSPlfIN5tJx3hrTcRKHMeyENao2L_hgkjuwN9LXJzKbECH2Xwig

--- kv_link ---
Linked: None

--- kv_get_links ---
Links for app:config: [{'collection': 'kv_links_example_py', 'created_at': '2026-09-16T13:46:40.281713Z', 'document_id': 'U4z5TpCZJQfXc3du_-UM0WXrbXOUya1mezFtSPlfIN5tJx3hrTcRKHMeyENao2L_hgkjuwN9LXJzKbECH2Xwig', 'field_path': None, 'last_accessed': '2026-09-16T13:46:40.282939Z', 'metadata': {}}]

--- kv_unlink ---
Unlinked: None

--- kv_get_links (verify empty) ---
Links after unlink: []

--- cleanup ---
Cleaned up KV key and collection

=== KV links example completed ===
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
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
  kv_operations:py:cache:product:2: {'name': 'Product 2', 'price': 39.99}
  kv_operations:py:cache:product:3: {'price': 49.99, 'name': 'Product 3'}

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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== ekoDB Raw Completion Stream (SSE) Example ===

--- Basic SSE Raw Completion ---
Response: The three primary colors are red, blue, and yellow.

--- Structured Output via SSE ---
JSON response: [
  {
    "name": "Earth",
    "diameter_km": 12742
  },
  {
    "name": "Mars",
    "diameter_km": 6779
  },
  {
    "name": "Jupiter",
    "diameter_km": 139820
  }
]

--- Blocking HTTP (for comparison) ---
Blocking response: Hello! I hope you're having a wonderful day.

=== Done ===
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== ekoDB Schedule Management Example (Python) ===

--- create_schedule ---
Created schedule: Nightly Report (id: cf536bf3-55eb-48f8-9fcd-eb9f3e5ee824)

--- list_schedules ---
Schedules: {'count': 1, 'schedules': [{'created_at': '2026-09-16T13:46:43.864278Z', 'cron_expression': '0 0 0 * * *', 'description': 'Generate and email nightly analytics report', 'enabled': True, 'function_label': 'schedule_noop_python_15331', 'id': 'cf536bf3-55eb-48f8-9fcd-eb9f3e5ee824', 'last_execution': None, 'name': 'Nightly Report', 'next_execution': '2026-09-17T00:00:00Z', 'parameters': {}, 'stats': {'avg_execution_time_ms': 0.0, 'failed_executions': 0, 'last_error': None, 'successful_executions': 0, 'total_executions': 0}, 'timezone': 'UTC', 'updated_at': '2026-09-16T13:46:43.864278Z'}]}

--- get_schedule ---
Fetched: Nightly Report cron=0 0 0 * * *

--- update_schedule ---
Updated cron: 0 30 1 * * *

--- trigger_schedule ---
Triggered: {'schedule_id': 'cf536bf3-55eb-48f8-9fcd-eb9f3e5ee824', 'status': 'triggered'}

--- pause_schedule ---
Paused: enabled=False

--- resume_schedule ---
Resumed: enabled=True
Schedule deleted successfully

=== Schedule management example completed ===
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== Search Examples ===

Setting up test data...
✅ Test data created

1. Basic full-text search:
Found 2 results
  1. Score: 12.870, Matched: email, name
  2. Score: 6.270, Matched: name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 13.200, Matched: bio, title
  2. Score: 13.200, Matched: title, bio
  3. Score: 13.200, Matched: title, bio
  4. Score: 13.200, Matched: title, bio

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 26.400, Matched: title, bio
  2. Score: 26.400, Matched: bio, title
  3. Score: 26.400, Matched: title, bio
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
  1. Score: 0.767
  2. Score: 0.762
  3. Score: 0.739

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.381, Matched: title, content
  2. Score: 0.884, Matched: title, content
  3. Score: 0.370, Matched:

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 6.600, Matched: title

9. Vector search with a metadata pre-filter (category = ml):
Found 2 documents in category "ml" (NLP excluded)
  1. Deep Learning Fundamentals (category: ml)
  2. Introduction to Machine Learning (category: ml)

=== Cleanup ===
✅ Deleted test collections

✅ Search examples completed!
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {'id': 'N-jyqJKt4chumKeEwGLoDr_Mt1BzZ3w98K7EC8gbnD7fQMu9DWBrtly37pGLR-l8474E1rE_4Ij6midLbfqj6A'}

=== Find by ID ===
Found: {'tags': {'value': ['tag1', 'tag2', 'tag3'], 'type': 'Array'}, 'name': {'type': 'String', 'value': 'Test Record'}, 'data': {'value': 'aGVsbG8gd29ybGQ=', 'type': 'String'}, 'categories': {'type': 'Array', 'value': ['electronics', 'computers']}, 'user_id': {'type': 'String', 'value': '550e8400-e29b-41d4-a716-446655440000'}, 'embedding': {'value': [0.1, 0.2, 0.3, 0.4, 0.5], 'type': 'Array'}, 'created_at': {'type': 'String', 'value': '2026-09-16T09:46:44.552501'}, 'metadata': {'type': 'Object', 'value': {'nested': {'deep': True}, 'key': 'value'}}, 'price': {'type': 'Float', 'value': 99.99}, 'value': {'value': 42, 'type': 'Integer'}, 'id': 'N-jyqJKt4chumKeEwGLoDr_Mt1BzZ3w98K7EC8gbnD7fQMu9DWBrtly37pGLR-l8474E1rE_4Ij6midLbfqj6A', 'active': {'value': True, 'type': 'Boolean'}}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): True
  price (Decimal): 99.99
  created_at (DateTime): 2026-09-16 09:46:44.552501
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): ['tag1', 'tag2', 'tag3']
  metadata (Object): {'nested': {'deep': True}, 'key': 'value'}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): ['electronics', 'computers']
  data (Bytes): 11 bytes
Plain record: {'tags': ['tag1', 'tag2', 'tag3'], 'name': 'Test Record', 'data': 'aGVsbG8gd29ybGQ=', 'categories': ['electronics', 'computers'], 'user_id': '550e8400-e29b-41d4-a716-446655440000', 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5], 'created_at': '2026-09-16T09:46:44.552501', 'metadata': {'nested': {'deep': True}, 'key': 'value'}, 'price': 99.99, 'value': 42, 'id': 'N-jyqJKt4chumKeEwGLoDr_Mt1BzZ3w98K7EC8gbnD7fQMu9DWBrtly37pGLR-l8474E1rE_4Ij6midLbfqj6A', 'active': True}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'embedding': {'value': [0.1, 0.2, 0.3, 0.4, 0.5], 'type': 'Array'}, 'user_id': {'value': '550e8400-e29b-41d4-a716-446655440000', 'type': 'String'}, 'created_at': {'type': 'String', 'value': '2026-09-16T09:46:44.552501'}, 'tags': {'type': 'Array', 'value': ['tag1', 'tag2', 'tag3']}, 'metadata': {'value': {'key': 'value', 'nested': {'deep': True}}, 'type': 'Object'}, 'categories': {'value': ['electronics', 'computers'], 'type': 'Array'}, 'name': {'value': 'Updated Record', 'type': 'String'}, 'id': 'N-jyqJKt4chumKeEwGLoDr_Mt1BzZ3w98K7EC8gbnD7fQMu9DWBrtly37pGLR-l8474E1rE_4Ij6midLbfqj6A', 'value': {'type': 'Integer', 'value': 100}, 'price': {'type': 'Float', 'value': 99.99}, 'data': {'type': 'String', 'value': 'aGVsbG8gd29ybGQ='}, 'active': {'type': 'Boolean', 'value': True}}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: d60DCX9GnxIKQsDpagOUzX2xmIKZtyU89-K6jIyshEpRUocVoMj1xBKKVYKM3c-v21kOSTSV7VpsAFvJDTEBpA

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
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
✓ Created native SWR script: github_user_native_py (xxZRSrRhnZaSTLnQLruEdF-V8vOpgMgyn1PIEGKNjV86T6ht_OICPgXbtae5MSo1XfNeJQkUzeaV0PrWbT09-w)

First call (cache miss - will fetch from GitHub API):
  Response time: 160ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 4ms
  Speedup: 38.9x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit_py (BbAIc7ghYyGfeNUFbYU5NIDFyHSQwe-LvKn5MKfTpPm6fl6FhpjykrRq1tybVbbDw_geCee3myS11qfGpkr_eg)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail_py' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline_py (XHbikHblkYdNoShLSTDacMgtjXxgoxnSmKTjL0FAB7Ebb13KjeK3Q3AUVuRxdkWHjn0HDNnaGbcBUCeFdXX-wQ)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users_swr_py' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache_py (Zq_uMGdi__HW7v131_fyoYI4PXaKs9RpAhjNZQsCNdSwpCLn9Mh9cr1hJYTp4uyCmgJrKMPQ-WsraWM6LlinuA)
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_api_user_py (4xgBQBp-RoILrO5Ipdr09WufQuaQbTVoJmelROSED7wAtpL783yPsSUQiEzkduVVmlH0yFoSfjQ5IAE5nupzJA)

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
Response time: 6ms (served from cache)
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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: qzE2Ohr-Pn04Lm5t2PLqd1TaF06MQf_SSrUY94v1hf1p2qK0T4JSBa9wxUJhZjgqgtzIl12HL8FxHaOp0KO5Ww
Created Bob: $500 - ID: a2-ixUxm5rJsDPFl8jimqo7DSN5Oc-yBvKr1GNTZ1W2Cfo1t01goYIOJd3AuPaH4dI400ADV0KAWnnXov2OJNg

=== Example 1: Begin Transaction ===
Transaction ID (server-default isolation): 255cf96b-c6cf-4b94-b8bc-2b2541f1f723

=== Example 2: Operations within Transaction ===
Alice in transaction: {'id': 'qzE2Ohr-Pn04Lm5t2PLqd1TaF06MQf_SSrUY94v1hf1p2qK0T4JSBa9wxUJhZjgqgtzIl12HL8FxHaOp0KO5Ww', 'name': {'type': 'String', 'value': 'Alice'}, 'balance': {'value': 800, 'type': 'Integer'}, 'account_id': {'type': 'String', 'value': 'ACC001'}}
Bob in transaction: {'id': 'a2-ixUxm5rJsDPFl8jimqo7DSN5Oc-yBvKr1GNTZ1W2Cfo1t01goYIOJd3AuPaH4dI400ADV0KAWnnXov2OJNg', 'balance': {'type': 'Integer', 'value': 700}, 'name': {'type': 'String', 'value': 'Bob'}, 'account_id': {'value': 'ACC002', 'type': 'String'}}
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

✓ Verified committed balances: Alice=$800, Bob=$700

=== Example 5: Rollback Demo ===
New transaction: 38cbd076-acca-4a4e-9140-90fd8c3c7b49
Updated Bob: $700 → $600 (in transaction)
Status before rollback: Active
✓ Transaction rolled back

✓ Verified Bob remains $700 after rollback

=== Cleanup ===
✓ Deleted test account collection

✓ All client transaction examples completed
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
✓ Client created

=== Create User Function ===
Created user function with ID: cikQHqX29yz_g8VdcDYS8SZs3o8EhOOxEmzCyC7yIRKdfVcdHpmVoajnngCj3gUDievScrqsrp3Fq32ZJAdXYA

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
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== WebSocket Chat Streaming Example (Python) ===

Created chat session: 8X7E13QPW5EJp8uVTxi94mr0Whg69J-rK-gEXlsvfga6DNhMNarxtcG4t1DlPUR8ynq-PO4xqfzvts9WN85UrA

Sending message: 'What is the capital of France?'
{"__progress":"received"}{"__progress":"generating","model":"gpt-4o-mini","provider":"openai"}The capital of France is Paris.

--- Stream ended ---
Message ID: tosx-JmwfYh2ujqkvdKMxTo9TDgcnYODkw708jvG1CXe0fOqg5TbJXmYmRESh0OvaNgI_OYvoccXHCZi_z4vyA
Execution time: 870ms
Token usage: {'completion_tokens': 8, 'prompt_tokens': 15, 'total_tokens': 23}

Full response: {"__progress":"received"}{"__progress":"generating","model":"gpt-4o-mini","provider":"openai"}The capital of France is Paris....
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== WebSocket Subscription Example (Python) ===

✓ Authentication successful
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_py' ===
✓ Subscribed (subscription_id: sub_f2670ef209344356a9e118212bb4ef18)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: JWGtopGtQlVlfAwgxYqbMrD7d-mspY4Jf-lAINnj6MujzSb__AkyjCq9wn9X7h4H-QUHOXAMDSDNQrdCiXcYlg

  📡 Notification received:
     Event:      insert
     Collection: ws_subscribe_example_py
     Record IDs: JWGtopGtQlVlfAwgxYqbMrD7d-mspY4Jf-lAINnj6MujzSb__AkyjCq9wn9X7h4H-QUHOXAMDSDNQrdCiXcYlg
     Timestamp:  2026-09-16T13:46:47.501309+00:00

Inserting record 2...
✓ Inserted: Q2IixamaWeZb8UZTF3wllCBzQHEXUelak2R0Ok35JmVxhQvsUveUAbyslsTwfsozjaSSDBiuBTKjgAh4kLZnuw

  📡 Notification received:
     Event:      insert
     Record IDs: Q2IixamaWeZb8UZTF3wllCBzQHEXUelak2R0Ok35JmVxhQvsUveUAbyslsTwfsozjaSSDBiuBTKjgAh4kLZnuw

=== Unsubscribing ===
✓ Unsubscribed: {'collection': 'ws_subscribe_example_py', 'found': True, 'unsubscribed': True}

=== Cleanup ===
✓ Deleted collection 'ws_subscribe_example_py'

✓ WebSocket subscription example completed successfully
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: 09XHycUGX8_ITYQH0fmkijqqoXnPs_jtI6ULyUdu5XvHF1uvQSrLc_q0bSFAC8id4PLD75UwwqSAr0Dnf2Tc-Q

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 5 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
=== Bypass Ripple Example ===

1. Basic insert (ripple enabled):
   Inserted with ripple: {'id': 'SzVyw2VOoq18eU2wQBBXOlNN8JbFY3EPnUsEw7oi0mznOWDr7MTNl2K9k9R2F6IvqIqHfTc3a3n6DuSOB268XQ'}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {'id': 'z4fZDtrI9ZHsdgRuWjXeuv4zW5PeJDVLnlyc1VVV_z6bbcQBiEThc6DWF9PpYCYN3hHBbUqKa3IHBCsDDqSTFA'}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {'name': {'value': 'Product 1', 'type': 'String'}, 'price': {'value': 150, 'type': 'Integer'}, 'id': 'SzVyw2VOoq18eU2wQBBXOlNN8JbFY3EPnUsEw7oi0mznOWDr7MTNl2K9k9R2F6IvqIqHfTc3a3n6DuSOB268XQ'}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {'price': {'type': 'Integer', 'value': 500}, 'id': 'SzVyw2VOoq18eU2wQBBXOlNN8JbFY3EPnUsEw7oi0mznOWDr7MTNl2K9k9R2F6IvqIqHfTc3a3n6DuSOB268XQ', 'name': {'type': 'String', 'value': 'Upsert Product'}}

✅ All bypass_ripple operations completed successfully!
perl: warning: Setting locale failed.
perl: warning: Please check that your locale settings:
	LC_ALL = "C.UTF-8",
	LC_CTYPE = "C.UTF-8",
	LANG = "C.UTF-8"
    are supported and installed on your system.
perl: warning: Falling back to the standard locale ("C").
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
  Fields returned: ['user_role', 'bio', 'created_at', 'name', 'avatar_url', 'email', 'id', 'status', 'age']

Example 3: Complex query with projection (active users, ages 18-65)
  Found 3 active users (ages 18-65)
    - {'value': 'Dave Brown', 'type': 'String'} (age {'type': 'Integer', 'value': 45})
    - {'type': 'String', 'value': 'Alice Johnson'} (age {'type': 'Integer', 'value': 30})
    - {'type': 'String', 'value': 'Bob Smith'} (age {'value': 25, 'type': 'Integer'})

Example 4: Query inactive users with profile fields
  Found 1 inactive users
    - {'value': 'Carol White', 'type': 'String'}: {'value': 'Manager', 'type': 'String'}

Example 5: Compare full vs projected data
  Full query:
    - 12 fields per record
    - Fields: ['id', 'status', 'avatar_url', 'api_key', 'password', 'email', 'bio', 'name', 'age', 'user_role', 'secret_token', 'created_at']
  Projected query:
    - 3 fields per record
    - Fields: ['name', 'email', 'id']
  Bandwidth savings: ~75% fewer fields

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
