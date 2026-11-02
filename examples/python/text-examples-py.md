make test-examples-python
📦 Ensuring Python example dependencies in .venv...
🧪 Running Python examples (direct HTTP/WebSocket)...
=== Simple CRUD Operations (Direct HTTP) ===

✓ Authentication successful

=== Insert Document ===
Inserted: {'id': 'alitHW5T4m5rSpmPyFtBkQKbUrNwft04Cn05LSlntPi8gtFbkBDV0DLMHuIgfXUa_QBex2f2yz5MWfqgrKRpdg'}

=== Find by ID ===
Found: {'value': {'value': 42, 'type': 'Integer'}, 'id': 'alitHW5T4m5rSpmPyFtBkQKbUrNwft04Cn05LSlntPi8gtFbkBDV0DLMHuIgfXUa_QBex2f2yz5MWfqgrKRpdg', 'active': {'type': 'Boolean', 'value': True}, 'name': {'type': 'String', 'value': 'Test Record'}}

=== Find with Query ===
Found documents: [{'name': {'type': 'String', 'value': 'Test Record'}, 'id': 'alitHW5T4m5rSpmPyFtBkQKbUrNwft04Cn05LSlntPi8gtFbkBDV0DLMHuIgfXUa_QBex2f2yz5MWfqgrKRpdg', 'active': {'type': 'Boolean', 'value': True}, 'value': {'value': 42, 'type': 'Integer'}}]

=== Update Document ===
Updated: {'name': {'value': 'Updated Record', 'type': 'String'}, 'value': {'type': 'Integer', 'value': 100}, 'active': {'value': True, 'type': 'Boolean'}, 'id': 'alitHW5T4m5rSpmPyFtBkQKbUrNwft04Cn05LSlntPi8gtFbkBDV0DLMHuIgfXUa_QBex2f2yz5MWfqgrKRpdg'}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: YuQn7swrJIJ8phzRH8cR5R1NHtJN1KOYmyaQE97VBiBiQAJi1xcIvEFPXa4EvqU8CQw2W3Y54gU1tMTk-rA22g

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "type": "Success",
  "payload": {
    "data": [
      {
        "value": {
          "value": 42,
          "type": "Integer"
        },
        "active": {
          "value": true,
          "type": "Boolean"
        },
        "name": {
          "type": "String",
          "value": "WebSocket Test Record"
        },
        "id": "YuQn7swrJIJ8phzRH8cR5R1NHtJN1KOYmyaQE97VBiBiQAJi1xcIvEFPXa4EvqU8CQw2W3Y54gU1tMTk-rA22g"
      }
    ]
  },
  "messageId": "876269990"
}
✓ Retrieved 1 record via WebSocket

✓ WebSocket example completed successfully
🚀 ekoDB Functions Example (Python/HTTP)

✓ Authentication successful

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Function saved: zFxDo1KFDsfripBEMInd_1uGJpp6ADKBcNdY_3rIWukU0AIC2Wbx4IWXIkdhUIBChrXh6iUMq2xDVLJxTzrJuA
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Function saved: _prJzrxLQKYuB5nVBD_ab-kmFacLvkC8h6XY6tzMuRt0PjB7RQLIF5mPTm4tGrX5ZIt4nlWcdnzPxjJt6jrarw
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: oc_iNXoGnZee2v4tX-McUB94kVWxM9j0VlSunPLcczkXHnxpuzNI3hyzoGRRXnvPT4QR8xhzD7RhxLUOgS6e7w
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {'avg_score': {'type': 'Float', 'value': 50.0}, 'max_score': {'value': 90, 'type': 'Integer'}, 'count': {'type': 'Integer', 'value': 5}, 'status': {'value': 'inactive', 'type': 'String'}}
   {'count': {'type': 'Integer', 'value': 5}, 'avg_score': {'type': 'Float', 'value': 60.0}, 'max_score': {'value': 100, 'type': 'Integer'}, 'status': {'value': 'active', 'type': 'String'}}

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
kv_operations:direct:py:cache:product:1: {'value': {'name': 'Product 1', 'price': 29.99}, 'type': 'Object'}
kv_operations:direct:py:cache:product:2: {'type': 'Object', 'value': {'name': 'Product 2', 'price': 39.989999999999995}}
kv_operations:direct:py:cache:product:3: {'value': {'price': 49.989999999999995, 'name': 'Product 3'}, 'type': 'Object'}

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
=== Collection Management (Direct HTTP) ===

✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: xSNne4Hju3jmPKjilFRqC5v8WWYgDo1ZoUKTlZiDTAn5ZRtCx1Evsjt0z4m4pFiy6z98_3lS8obqbqbWqCCR8g

=== List Collections ===
Total collections: 13
Sample collections: ['audit__ek0_testing', 'chat_raw_completions__ek0_testing', 'functions__ek0_testing', 'chat_turns__ek0_testing', 'chat_goal_templates__ek0_testing']

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
Created Alice: $1000 - ID: TcGhtZ3njo2LhblWSC1KWWSeqpyqpclHu19VtUdEmy_ahTUhRrFhs0NRt87GvoQtL1aDyzcTEyaqj1rVA2EG6g
Created Bob: $500 - ID: 3wNq_2pTCIe45D_Kd9Pq917I3hNcGppXiYghUq0-pbZ0GfPUMunCGkZqocis01sOeV6mOoauG_fID0KOZc465Q

=== Example 1: Begin Transaction ===
Transaction ID: facaae35-4804-4899-9e24-e3d7a563187f

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
New transaction: fa608ce4-fe06-4b95-9a2d-649f670abd88
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'value': 700, 'type': 'Integer'}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 1QEKWKeenM8i069n6RhogQHsGmneL7uvg3IFanivG33V5_e9lGz8RgPrHK0i0OZKVXrcmvwekBxhB6E2V5S4GA
Created Bob: $500 - ID: A8gRn3EZem-UlQVNakbPM5ZDV2dYXtWFGKfMyoj-8FEgkZ-OROlF5gYSgEfFTJNFWiMn6mE6AknayGi2NCDx9A

=== Example 1: Begin Transaction ===
Transaction ID: a3693e52-0e8d-4614-b84d-21006176ce52

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
Created Alice: $1000 - ID: ii_dYdINTWWoFB_uERuCa0QDOOkQNZXqYa7qkeCrZ0Y2crVn6Us0Ir1xrN0c9ZbqfwbKP4dcJGXmZxetkz3iSg
Created Bob: $500 - ID: pn6cUCDCeYTGNJLOmc_iYIS1UH5KEzW2M2NBQgUtzRQ1bdZKbvrvVCj-sNKlA5KBmI0CUprXW3NpRPe-M1ribQ

=== Example 1: Begin Transaction ===
Transaction ID: 81ac7ab3-8554-40aa-aef1-98625d18df34

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
New transaction: 3d2fc8bd-bff8-42d0-beeb-fb112700c6a8
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'value': 700, 'type': 'Integer'}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: Hvq782Y8biqOfdV9do3YqtegFOS5QGTq87Fnknj5pcIi3lmCXXEWUcT2cEDt2blz2CUjLzUv5q8sZPYuSDx8yw
Created Bob: $500 - ID: 9snKyXGPxb2kQpugQweNpSx_ePdOOl-KKFr9vH-Inma9nDhAAuxEfQ-38cQrFyf3XDEM7BmunmusqmMWiRfaOQ

=== Example 1: Begin Transaction ===
Transaction ID: 688de700-df68-4ae3-9abc-a60c861d0a61

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
Created Alice: $1000 - ID: PkJ1J4zYg01hQ99IO0y1YZ6ieDfaB3jXe7_A8DiA63mp34jrbozpcRaR1HdEltmnCcdNwupXP_mbOuTTlhu0aw
Created Bob: $500 - ID: Tq3KB8KJyaPVClvXLaFWEpxqL9h5LpWEHJALxQP1opfMkIbOIBxVVNteJvt-nH6GEFcjjAPmd2tJPhPgqmlFEA

=== Example 1: Begin Transaction ===
Transaction ID: 0ba85aae-2b5f-4560-a48f-ba442dfab519

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
New transaction: c495f19c-becf-445a-9878-b4d3d935d9ef
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'type': 'Integer', 'value': 700}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: hT8Z4oJZwE35aCxm0RX1AXxe6oFu8ZhnU4StagS44JQGv-ObYG_6XUcD5qSb7BCbs7BWVVLpp4onEyc3w1Nf4Q
Created Bob: $500 - ID: OhV6N-_uxH5kz2mA8XM8WMQEhMoLtuJruqQunkX_9v6c4eFTpP4O2ymoRdIvj-YG8C0lCD5RHsAkwrAuyOT72g

=== Example 1: Begin Transaction ===
Transaction ID: dd867d43-1344-45b8-818b-d0c7b6d33a40

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
   ✅ Function saved: j2z7ZZ42CMfJOK3JuJtsH9fjHOulyX1qOqH02xanDlVOx_PzHLS11ZAVK4dkA0l7MgTNzuEXh609NfBq28Bjqg

2️⃣ Calling function (Insert + Verify)...
   ✅ function executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: cKwwOgtORilZkiQWRpyu344RnPIX0bc4OyX5rH3zY56Pi0ZEt5e3A38AiB8B154tdrgG3-_ZD3Abun76avB2yQ
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}
   📋 Email: {'type': 'String', 'value': 'alice@example.com'}
   📋 Status: {'type': 'String', 'value': 'pending'}
   📋 Credits: {'value': 0, 'type': 'Integer'}

============================================================
📝 function 2: Query + Update + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: N0-g1eAAfBJ5_Heq_PSj1J_cKCppXoR9T8YraS1Sg5X6Jbtoosun6Qad6Z4PXzL90v-yaVb4yTf11r8TN0jokA

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
   ✅ Function saved: 5pMYvEbe62ZzkXRrq2WunDfSvO4d2VaI73HtfDNENxlyxhaK_WInvlv514hvrLWJ--uyTM-bTc5SnhHYzYd9pg

2️⃣ Calling function (Query + Update Credits + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {'type': 'Integer', 'value': 100}
   📋 Status: {'value': 'active', 'type': 'String'}
   📋 Name: {'value': 'Alice Smith', 'type': 'String'}

============================================================
📝 function 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: JmbI4CTua70IE20lkZWMv3J3Wt9sPfDafsO-JoshVlWzO3OEIxJ3YV7HYrBgBC1bBCVtPK9lidcPG4Z_ip3EXA

2️⃣ Calling function (Query + Delete + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: j2z7ZZ42CMfJOK3JuJts...
   ✅ Deleted script: N0-g1eAAfBJ5_Heq_PSj...
   ✅ Deleted script: 5pMYvEbe62ZzkXRrq2Wu...
   ✅ Deleted script: JmbI4CTua70IE20lkZWM...
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
  Output: Document ID = QN8nNb6EseCCv4XdU9mtTjSkLuOG0n-LWO-WNvRVdapCCg64U4SMgJveeezt83NWzGk_WJaI-ABPJHBjLjbiLQ
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: find_by_id(QN8nNb6EseCCv4XdU9mtTjSkLuOG0n-LWO-WNvRVdapCCg64U4SMgJveeezt83NWzGk_WJaI-ABPJHBjLjbiLQ)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: find_by_id(QN8nNb6EseCCv4XdU9mtTjSkLuOG0n-LWO-WNvRVdapCCg64U4SMgJveeezt83NWzGk_WJaI-ABPJHBjLjbiLQ)
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
  Output: Document ID = 36_X2JZi_AoPFb5dLI-yVK8tL6g4HDbJ2UshDwr2vM9OrRgN349wRTvoHsblcQuJEMmBSIo6-sda_TbUJYLr0g
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: find_by_id(36_X2JZi_AoPFb5dLI-yVK8tL6g4HDbJ2UshDwr2vM9OrRgN349wRTvoHsblcQuJEMmBSIo6-sda_TbUJYLr0g)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: find_by_id(36_X2JZi_AoPFb5dLI-yVK8tL6g4HDbJ2UshDwr2vM9OrRgN349wRTvoHsblcQuJEMmBSIo6-sda_TbUJYLr0g)
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
🐍 Found CPython 3.15 at /Users/tek/.local/bin/python3
🔗 Found pyo3 bindings with abi3-py3.8 support
💻 Using `MACOSX_DEPLOYMENT_TARGET=11.0` for aarch64-apple-darwin by default
    Finished `release` profile [optimized] target(s) in 0.20s
📦 Built wheel for abi3 Python ≥ 3.8 to ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.31.0-cp38-abi3-macosx_11_0_arm64.whl
📦 Installing Python wheel into .venv...
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.31.0-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.31.0
    Uninstalling ekodb_client-0.31.0:
      Successfully uninstalled ekodb_client-0.31.0
Successfully installed ekodb-client-0.31.0
🧪 Ensuring test dependencies (pytest) in .venv...
✅ Python client package built and installed!
Admin permissions read successfully
=== ekoDB Advanced CRUD Example (Python) ===

--- insert ---
Inserted: zj6WmhSGlc-QjAyzZUekj65bygXafZdV8UpcdlzzuVNyty-BXh1Z6NMiCKxIZBEIVJrQtcqhhODPU48LE1R8Vg

--- update_with_action (increment) ---
After increment count by 5: {'name': {'type': 'String', 'value': 'Counter Record'}, 'count': {'value': 15, 'type': 'Integer'}, 'id': 'zj6WmhSGlc-QjAyzZUekj65bygXafZdV8UpcdlzzuVNyty-BXh1Z6NMiCKxIZBEIVJrQtcqhhODPU48LE1R8Vg', 'tags': {'value': ['initial'], 'type': 'Array'}, 'score': {'value': 50.0, 'type': 'Float'}}

--- update_with_action (decrement) ---
After decrement score by 10: {'count': {'value': 15, 'type': 'Integer'}, 'id': 'zj6WmhSGlc-QjAyzZUekj65bygXafZdV8UpcdlzzuVNyty-BXh1Z6NMiCKxIZBEIVJrQtcqhhODPU48LE1R8Vg', 'tags': {'type': 'Array', 'value': ['initial']}, 'score': {'type': 'Float', 'value': 40.0}, 'name': {'value': 'Counter Record', 'type': 'String'}}

--- update_with_action_sequence ---
After action sequence: {'tags': {'type': 'Array', 'value': ['initial', 'sequenced']}, 'id': 'zj6WmhSGlc-QjAyzZUekj65bygXafZdV8UpcdlzzuVNyty-BXh1Z6NMiCKxIZBEIVJrQtcqhhODPU48LE1R8Vg', 'name': {'value': 'Counter Record', 'type': 'String'}, 'count': {'type': 'Integer', 'value': 115}, 'score': {'value': 40.0, 'type': 'Float'}}

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
✓ Created session: zUwx0vKmIh84zFj9lAtm01XgJSZW1muwLwjvy-jjQVOMNIATtwTFPovnZ3E9rnsdgdCmKa9-EoF90247bC0hLQ

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is **ekoDB**, described as a high-performance database product, priced at **99**.

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['created_at', 'author', 'content', 'llm_provider', 'chat_id', 'llm_model', 'role', 'token_usage', 'updated_at', 'context_snippets', 'id'])
Debug: First message role: {'type': 'String', 'value': 'assistant'}
=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: ekoDB costs **99**. The currency isn’t specified.

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: LKpuiQnTchck8TFLIkaX-kzGdOIYlc2as4mWtUrUANGa78wseWx8ZmeLLsL7YPoTyvPDMdLs6uRpME-9X2lHOg
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 7

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 6

=== Cleanup ===
✓ Deleted chat session: LKpuiQnTchck8TFLIkaX-kzGdOIYlc2as4mWtUrUANGa78wseWx8ZmeLLsL7YPoTyvPDMdLs6uRpME-9X2lHOg
✓ Deleted chat session: zUwx0vKmIh84zFj9lAtm01XgJSZW1muwLwjvy-jjQVOMNIATtwTFPovnZ3E9rnsdgdCmKa9-EoF90247bC0hLQ
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: r6z7B8cRIjiwcF8guMFZa1buacDA4pQlGrSjuff22lDfxDDVnOpucURKLnBT5hh4T3tW3JMNNkUGPirmTBbxqQ

=== Sending Chat Message ===
Message ID: ZTvVBwp27GL1rlFFVxGZKMozdrh2S9gD-oKoMxQVAECS-RlQbgAXeK20tXVB_Z_-sXtQm665uf2_QtvwR0AmAg

=== AI Response ===
Available products and listed prices:

- **ekoDB** — 99
- **ekoDB Pro** — 299
- **ekoDB Cloud** — 499

The currency isn’t specified in the data.

Execution Time: 2101ms

=== Token Usage ===
Prompt tokens: 13716
Completion tokens: 80
Total tokens: 13796

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Message Stream (SSE) Example (Python) ===

Created session: 1ulK0_bmI-Lt0Wjz9u-MsLCKMVChyI5ajbT0KB6HX5Frh6cyWljtn7YVRn_N7_6xPfLOPmmvvHoMBvmGa8A8hA

Streaming response for: 'What is ekoDB?'

{"__progress":"received"}{"__progress":"generating","model":"gpt-6-luna","provider":"openai"}ekoDB is a lightweight database project for storing and querying application data. It’s generally described as an embedded database, meaning it can run alongside an app rather than requiring a separate database server. There may be multiple projects with that name, so a link would help if you mean a specific one.

--- Stream complete ---
Message ID: m1xsZPpflmKuXOs_8o9Bx12dRRR7CblNjxu8ozjtA1m0ImHs8rP9eY6PeeRMvEKJc7OaReiPQneAlS87OeSknA
Execution time: 10120ms
Context window: 128000 tokens

✓ Chat message stream example completed
✓ Client created

=== Get All Chat Models ===
OpenAI models: ['gpt-3.5-turbo-0125', 'gpt-5.4-2026-03-05', 'gpt-5-nano', 'gpt-5.5-pro', 'gpt-audio-2025-08-28', 'gpt-4o-mini-transcribe-2025-03-20', 'gpt-4-turbo-2024-04-09', 'gpt-realtime-2.1-mini', 'gpt-realtime-2025-08-28', 'gpt-4', 'tts-1-hd', 'gpt-realtime-1.5', 'gpt-realtime', 'gpt-5.2-pro', 'gpt-5-chat-latest', 'gpt-5.4-mini-2026-03-17', 'babbage-002', 'gpt-4o-2024-05-13', 'text-embedding-3-large', 'tts-1', 'omni-moderation-2024-09-26', 'gpt-live-transcribe', 'gpt-5.6-luna', 'gpt-5-2025-08-07', 'gpt-5-mini', 'gpt-5-mini-2025-08-07', 'omni-moderation-latest', 'o3-mini-2025-01-31', 'gpt-audio-1.5', 'gpt-image-1.5', 'tts-1-1106', 'gpt-5.5-pro-2026-04-23', 'gpt-audio-mini-2025-10-06', 'gpt-5.2-pro-2025-12-11', 'o4-mini-2025-04-16', 'gpt-4o-2024-11-20', 'gpt-5.1', 'whisper-1', 'gpt-5-pro', 'gpt-realtime-whisper', 'gpt-realtime-2', 'gpt-5-search-api', 'gpt-5.5-2026-04-23', 'gpt-4.1-nano', 'gpt-image-2', 'gpt-3.5-turbo-16k', 'text-embedding-ada-002', 'gpt-realtime-2.1', 'gpt-4.1', 'gpt-5.5', 'chat-latest', 'gpt-realtime-translate', 'gpt-5.4-pro-2026-03-05', 'gpt-4.1-mini', 'gpt-4o-mini', 'gpt-4o-mini-transcribe-2025-12-15', 'gpt-realtime-mini', 'gpt-image-1-mini', 'gpt-5.6-terra', 'gpt-3.5-turbo-instruct-0914', 'gpt-4-turbo', 'gpt-4o-2024-08-06', 'o3', 'gpt-4o-mini-search-preview-2025-03-11', 'o1-2024-12-17', 'gpt-5.1-chat-latest', 'gpt-5-pro-2025-10-06', 'gpt-5.6-sol', 'gpt-4o-transcribe-diarize', 'gpt-5-codex', 'gpt-4-0613', 'gpt-5.1-2025-11-13', 'gpt-4o-search-preview', 'gpt-4o-mini-tts-2025-12-15', 'gpt-image-1', 'gpt-audio', 'gpt-5.3-codex', 'gpt-5-search-api-2025-10-14', 'gpt-5.4-pro', 'gpt-4o-mini-tts', 'gpt-realtime-mini-2025-12-15', 'gpt-5.1-codex-max', 'sora-2', 'gpt-4o', 'gpt-4o-mini-2024-07-18', 'gpt-4.1-2025-04-14', 'gpt-5', 'gpt-4o-mini-transcribe', 'gpt-image-2-2026-04-21', 'gpt-5.2-chat-latest', 'sora-2-pro', 'gpt-3.5-turbo-instruct', 'gpt-transcribe', 'gpt-4.1-nano-2025-04-14', 'gpt-3.5-turbo-1106', 'gpt-4o-mini-tts-2025-03-20', 'gpt-5.2', 'gpt-5.4-nano-2026-03-17', 'o1', 'tts-1-hd-1106', 'gpt-5.2-codex', 'gpt-5.4-mini', 'o1-pro', 'gpt-5.1-codex', 'o4-mini-deep-research', 'gpt-5.4', 'gpt-4o-search-preview-2025-03-11', 'text-embedding-3-small', 'gpt-5-nano-2025-08-07', 'gpt-4o-transcribe', 'o4-mini', 'gpt-5.1-codex-mini', 'gpt-6-astra', 'gpt-5.3-chat-latest', 'o3-2025-04-16', 'gpt-5.2-2025-12-11', 'chatgpt-image-latest', 'gpt-3.5-turbo', 'gpt-image-2.5-sunburst-2026-09-08', 'gpt-image-2.5-flare', 'davinci-002', 'gpt-5.4-nano', 'o3-mini', 'o4-mini-deep-research-2025-06-26', 'gpt-audio-mini-2025-12-15', 'o1-pro-2025-03-19', 'gpt-live-1', 'gpt-image-2.5-sunburst', 'gpt-image-2.5-flare-2026-09-08', 'gpt-6.1-terra', 'gpt-audio-mini', 'gpt-4.1-mini-2025-04-14', 'gpt-6-luna', 'gpt-4o-mini-search-preview', 'gpt-6-sol', 'gpt-6.1-sol']
Anthropic models: ['claude-haiku-5-5', 'claude-sonnet-5-5', 'claude-opus-5-5', 'claude-fable-5-1', 'claude-opus-5', 'claude-sonnet-5', 'claude-fable-5', 'claude-opus-4-8', 'claude-opus-4-7', 'claude-sonnet-4-6', 'claude-opus-4-6', 'claude-opus-4-5-20251101', 'claude-haiku-4-5-20251001', 'claude-sonnet-4-5-20250929']
Perplexity models: []
Gemini models: []

=== Get OpenAI Models ===
OpenAI models: ['gpt-3.5-turbo-0125', 'gpt-5.4-2026-03-05', 'gpt-5-nano', 'gpt-5.5-pro', 'gpt-audio-2025-08-28', 'gpt-4o-mini-transcribe-2025-03-20', 'gpt-4-turbo-2024-04-09', 'gpt-realtime-2.1-mini', 'gpt-realtime-2025-08-28', 'gpt-4', 'tts-1-hd', 'gpt-realtime-1.5', 'gpt-realtime', 'gpt-5.2-pro', 'gpt-5-chat-latest', 'gpt-5.4-mini-2026-03-17', 'babbage-002', 'gpt-4o-2024-05-13', 'text-embedding-3-large', 'tts-1', 'omni-moderation-2024-09-26', 'gpt-live-transcribe', 'gpt-5.6-luna', 'gpt-5-2025-08-07', 'gpt-5-mini', 'gpt-5-mini-2025-08-07', 'omni-moderation-latest', 'o3-mini-2025-01-31', 'gpt-audio-1.5', 'gpt-image-1.5', 'tts-1-1106', 'gpt-5.5-pro-2026-04-23', 'gpt-audio-mini-2025-10-06', 'gpt-5.2-pro-2025-12-11', 'o4-mini-2025-04-16', 'gpt-4o-2024-11-20', 'gpt-5.1', 'whisper-1', 'gpt-5-pro', 'gpt-realtime-whisper', 'gpt-realtime-2', 'gpt-5-search-api', 'gpt-5.5-2026-04-23', 'gpt-4.1-nano', 'gpt-image-2', 'gpt-3.5-turbo-16k', 'text-embedding-ada-002', 'gpt-realtime-2.1', 'gpt-4.1', 'gpt-5.5', 'chat-latest', 'gpt-realtime-translate', 'gpt-5.4-pro-2026-03-05', 'gpt-4.1-mini', 'gpt-4o-mini', 'gpt-4o-mini-transcribe-2025-12-15', 'gpt-realtime-mini', 'gpt-image-1-mini', 'gpt-5.6-terra', 'gpt-3.5-turbo-instruct-0914', 'gpt-4-turbo', 'gpt-4o-2024-08-06', 'o3', 'gpt-4o-mini-search-preview-2025-03-11', 'o1-2024-12-17', 'gpt-5.1-chat-latest', 'gpt-5-pro-2025-10-06', 'gpt-5.6-sol', 'gpt-4o-transcribe-diarize', 'gpt-5-codex', 'gpt-4-0613', 'gpt-5.1-2025-11-13', 'gpt-4o-search-preview', 'gpt-4o-mini-tts-2025-12-15', 'gpt-image-1', 'gpt-audio', 'gpt-5.3-codex', 'gpt-5-search-api-2025-10-14', 'gpt-5.4-pro', 'gpt-4o-mini-tts', 'gpt-realtime-mini-2025-12-15', 'gpt-5.1-codex-max', 'sora-2', 'gpt-4o', 'gpt-4o-mini-2024-07-18', 'gpt-4.1-2025-04-14', 'gpt-5', 'gpt-4o-mini-transcribe', 'gpt-image-2-2026-04-21', 'gpt-5.2-chat-latest', 'sora-2-pro', 'gpt-3.5-turbo-instruct', 'gpt-transcribe', 'gpt-4.1-nano-2025-04-14', 'gpt-3.5-turbo-1106', 'gpt-4o-mini-tts-2025-03-20', 'gpt-5.2', 'gpt-5.4-nano-2026-03-17', 'o1', 'tts-1-hd-1106', 'gpt-5.2-codex', 'gpt-5.4-mini', 'o1-pro', 'gpt-5.1-codex', 'o4-mini-deep-research', 'gpt-5.4', 'gpt-4o-search-preview-2025-03-11', 'text-embedding-3-small', 'gpt-5-nano-2025-08-07', 'gpt-4o-transcribe', 'o4-mini', 'gpt-5.1-codex-mini', 'gpt-6-astra', 'gpt-5.3-chat-latest', 'o3-2025-04-16', 'gpt-5.2-2025-12-11', 'chatgpt-image-latest', 'gpt-3.5-turbo', 'gpt-image-2.5-sunburst-2026-09-08', 'gpt-image-2.5-flare', 'davinci-002', 'gpt-5.4-nano', 'o3-mini', 'o4-mini-deep-research-2025-06-26', 'gpt-audio-mini-2025-12-15', 'o1-pro-2025-03-19', 'gpt-live-1', 'gpt-image-2.5-sunburst', 'gpt-image-2.5-flare-2026-09-08', 'gpt-6.1-terra', 'gpt-audio-mini', 'gpt-4.1-mini-2025-04-14', 'gpt-6-luna', 'gpt-4o-mini-search-preview', 'gpt-6-sol', 'gpt-6.1-sol']

=== Get Anthropic Models ===
Anthropic models: ['claude-haiku-5-5', 'claude-sonnet-5-5', 'claude-opus-5-5', 'claude-fable-5-1', 'claude-opus-5', 'claude-sonnet-5', 'claude-fable-5', 'claude-opus-4-8', 'claude-opus-4-7', 'claude-sonnet-4-6', 'claude-opus-4-6', 'claude-opus-4-5-20251101', 'claude-haiku-4-5-20251001', 'claude-sonnet-4-5-20250929']

✓ Chat Models API example complete
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: aLgm6JdePd7jVZ6OWB1z1BKhbLNwam0_dNn6jliIEPC_TfzyGNgOOOJLIAx5p4oGlSkqWQK2zOabzozRSPF-hg

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is **ekoDB**, a high-performance database product, priced at **$99**.

✓ Message 2 sent
  Response: ekoDB is priced at **$99**.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: d11JMIlSIdub24NV5MZBNAVWX07siRK7CwB_ANhukfpyLZ6Em6x87gWqqJEZYSMWTaEhXP_j_l5TAoPyNllRlA
  Parent: aLgm6JdePd7jVZ6OWB1z1BKhbLNwam0_dNn6jliIEPC_TfzyGNgOOOJLIAx5p4oGlSkqWQK2zOabzozRSPF-hg

=== Listing Sessions ===
✓ Found 2 sessions
  Session 1: d11JMIlSIdub24NV5MZBNAVWX07siRK7CwB_ANhukfpyLZ6Em6x87gWqqJEZYSMWTaEhXP_j_l5TAoPyNllRlA (Untitled)
  Session 2: aLgm6JdePd7jVZ6OWB1z1BKhbLNwam0_dNn6jliIEPC_TfzyGNgOOOJLIAx5p4oGlSkqWQK2zOabzozRSPF-hg (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: d11JMIlSIdub24NV5MZBNAVWX07siRK7CwB_ANhukfpyLZ6Em6x87gWqqJEZYSMWTaEhXP_j_l5TAoPyNllRlA

=== Cleanup ===
✓ Deleted sessions and collection

✓ All session management operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: "am27GBve294XMevPVYYaM-Bf8B44XJXqyThATX3LORxcRt_FdUa-E4swbYUIeoG6bCDmDgphqIjvbVaS1VE_DA"

=== List Collections ===
Total collections: 13
Sample collections: ['audit__ek0_testing', 'client_collection_management_python', 'chat_raw_completions__ek0_testing', 'functions__ek0_testing', 'chat_turns__ek0_testing']

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
✓ Created record with plain dict: {'id': 'aDI-pKNn-onni_hYaX8diXs1MCzm1G-KXZqqc2TJu5GmPr8G6b89iktIY7oVp2fVAs64MdSUnY3h7eF9ytKSWw'}

=== Upsert Operation ===
✓ Upsert (update existing record): aDI-pKNn-onni_hYaX8diXs1MCzm1G-KXZqqc2TJu5GmPr8G6b89iktIY7oVp2fVAs64MdSUnY3h7eF9ytKSWw
✓ Inserted second record: LZhHmFVvHT5W4mXEoxV5lr6_ob6jv9SU8E4-U2lNbRwpXITx30HnMXOOrDmTdkrF2sIr0uN_n1T32BVpbMPH6A
✓ Upsert (update second record): LZhHmFVvHT5W4mXEoxV5lr6_ob6jv9SU8E4-U2lNbRwpXITx30HnMXOOrDmTdkrF2sIr0uN_n1T32BVpbMPH6A

=== Find One Operation ===
✓ Found user by email: {'id': 'aDI-pKNn-onni_hYaX8diXs1MCzm1G-KXZqqc2TJu5GmPr8G6b89iktIY7oVp2fVAs64MdSUnY3h7eF9ytKSWw', 'active': {'type': 'Boolean', 'value': True}, 'name': {'type': 'String', 'value': 'Alice Johnson'}, 'age': {'type': 'Integer', 'value': 29}, 'email': {'type': 'String', 'value': 'alice.j@newdomain.com'}}
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
✓ Inserted document: st32LXg_kNS-IElmlq2Kwxua9Lk4S2S6RaP0FNpVadhEadK-ItznR0ufT8KCgiIqdgIf9avHP8-V0_LhCBks0Q

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: KcEYJqR19ZFsTGJuCU6mqXJ_51j0iRxfHfTRwRLDgakRgftJP35o8zoMqfFJN0e5HRJK2dOig4bU89TNlYt-kw

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
✓ Edge cache script created: u7YqYRio7EdaTJYHgmQBKBjTzhyvYeOkPXgLVE03nZO1XD4Rnr-lzQfQ3pZm9ZLhZVEFym817N_lpQD1z2MGLw

Call 1: Cache miss (fetches from API)
Response time: 161ms
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "current": {
            "interval": 900,
            "temperature_2m": 8.5,
            "time": "2026-11-02T04:30"
          },
          "current_units": {
            "interval": "seconds",
            "temperature_2m": "\u00b0C",
            "time": "iso8601"
          },
          "elevation": 32.0,
          "generationtime_ms": 0.030875205993652344,
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
Response time: 2ms (69.7x faster!)
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "current": {
            "interval": 900,
            "temperature_2m": 8.5,
            "time": "2026-11-02T04:30"
          },
          "current_units": {
            "interval": "seconds",
            "temperature_2m": "\u00b0C",
            "time": "iso8601"
          },
          "elevation": 32.0,
          "generationtime_ms": 0.030875205993652344,
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
   ⏱️  Duration: 77.3ms
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
   ⏱️  Duration: 2.1ms
   📊 Records: 1
   📦 Data: {
      "value": {
            "type": "Object",
            "value": {
                  "address": {
                        "city": "Gwenborough",
                        "geo": {
                 ...
   🚀 Cache speedup: 37.5x faster!

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

✅ Function saved: VKbrBoKSnaJ-SojTXxZh3VjsfrEI57DmzTNhhdYqXhM5EnqPXqvNQzh9XoZfgBWHap1wvLcZtnnKTHGdD6vZ3g
📊 Found 5 active users

📝 Example 2: Parameterized Function

✅ Function saved: GWUMeJSjdoFtbO1nWlHI3kvNRtjUN657mEiWlg098P7GqBl2oc_S2vdTggrgqcWNgczWXaSQE4m74U7eIPOf5g
📊 Found 3 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: -IjBHxboF7flAyQhHiqQXnONf3gKmvQ9x1ICEr0gvQXP79dYPEmOKzoUJPebgEUL7bJr5bg8CqWMpAeILys8lQ
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
🚀 ekoDB Python AI Functions Example

📋 Setting up test data...
✅ Created 2 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
🤖 AI Response:
   Vector databases are designed to store and search **embeddings**—numeric representations of text, images, audio, or other data. Their main benefits include:

- **Semantic search:** Find items by meaning or similarity, not just exact keywords.
- **Fast similarity search at scale:** Use specialized indexes to find close matches across large collections.
- **RAG and AI applications:** Retrieve relevant documents to provide context to language models.
- **Recommendations and matching:** Suggest similar products, content, images, or users.
- **Metadata filtering:** Combine similarity search with conditions such as category, date, or access permissions.
- **Less custom infrastructure:** Manage vector storage, indexing, and retrieval without building those systems from scratch.

For small datasets or simple keyword search, a traditional database may be sufficient; vector databases are most useful when similarity search is central to the application.
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

✅ Function saved: cUaK47jNEocnnSen9quJMcg0-rNWqLF8OnAk4Oa_VE7dQNRX2T5rCAGnNUzugVj9wY71L3LNQ-E4PsGgozvleA
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

✅ Inserted order: YSFKFz8wDSOJz3h6w9KMiXT9jZemollBnmGXUAo-chRIiWAHX5qSudQpfC0yJBQIzs1665s7lPSDpwra3RQ7GA
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

✅ Inserted order: OvS1zUaIObd_i1-YN8NIq2RB0pouAjqEeevrB2VNwKLV9Ak2BaIofjw2qfjhDWCW4SKPyXRHagShlRbgM1A2Mw
✅ Cached order status
📊 Quick status lookup: {'value': '{"type":"Object","value":{"status":"processing","updated_at":"2026-11-02T04:42:37.821584+00:00"}}'}

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
   2. Vector Databases Explained (Database)
   3. Database Design Principles (Database)
   4. Introduction to Machine Learning (AI)
   5. Natural Language Processing (AI)
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
=== ekoDB Goal Template CRUD Example (Python) ===

--- Creating goal template ---
Created template: Data Migration (id: vD_Z68XCHynfkZdhQeLPoisa2uiave5hRJUW9fNCrXLNjWzyrV5sidMKt_M1WDG-coR9Hk6_LJke0SgQKRH9_Q)

--- Listing templates ---
Templates: {'count': 1, 'items': [{'description': {'type': 'String', 'value': 'Template for migrating data between schemas'}, 'id': 'vD_Z68XCHynfkZdhQeLPoisa2uiave5hRJUW9fNCrXLNjWzyrV5sidMKt_M1WDG-coR9Hk6_LJke0SgQKRH9_Q', 'owner': {'type': 'Object', 'value': {'key_id': '3a0cdba0da7c060610042783cc758f1f', 'key_label': 'default_admin'}}, 'steps': {'type': 'Array', 'value': [{'description': 'Analyze source schema'}, {'description': 'Create target schema'}, {'description': 'Migrate records'}, {'description': 'Validate results'}]}, 'title': {'type': 'String', 'value': 'Data Migration'}}], 'total': 1}

--- Getting template ---
Fetched: {'type': 'String', 'value': 'Data Migration'}

--- Updating template ---
Updated description: {'type': 'String', 'value': 'Updated: comprehensive data migration workflow'}

--- Deleting template ---
Template deleted successfully

✓ Goal template CRUD example completed
=== ekoDB Goals / Tasks / Agents Integration Example (Python) ===

--- goal_create ---
Created goal: Deploy v3 (id: yhDXbv1--zSbUnde0ywEQTXiWtvmCeql1tIanySIgOa9hYqSwpKJ905tBlmQ0x4pawUJzgszqlbKpcxkjOAYcg)

--- goal_list ---
Goals: {'count': 1, 'goals': [{'created_at': {'type': 'DateTime', 'value': '2026-11-02T04:42:38.271359+00:00'}, 'description': {'type': 'String', 'value': 'Deploy version 3 to production'}, 'id': 'yhDXbv1--zSbUnde0ywEQTXiWtvmCeql1tIanySIgOa9hYqSwpKJ905tBlmQ0x4pawUJzgszqlbKpcxkjOAYcg', 'owner': {'type': 'Object', 'value': {'key_id': '3a0cdba0da7c060610042783cc758f1f', 'key_label': 'default_admin'}}, 'status': {'type': 'String', 'value': 'pending'}, 'steps': {'type': 'String', 'value': '[{"description":"Build Docker image","status":"pending"},{"description":"Run migrations","status":"pending"},{"description":"Smoke test","status":"pending"}]'}, 'title': {'type': 'String', 'value': 'Deploy v3'}, 'updated_at': {'type': 'DateTime', 'value': '2026-11-02T04:42:38.271359+00:00'}}], 'total': 1}

--- goal_get ---
Fetched: {'type': 'String', 'value': 'Deploy v3'}

--- goal_update ---
Updated description: {'type': 'String', 'value': 'Deploy version 3 to production (updated)'}

--- goal_search ---
Search results: {'count': 1, 'items': [{'_score': 13.2, 'created_at': {'type': 'DateTime', 'value': '2026-11-02T04:42:38.271359+00:00'}, 'description': {'type': 'String', 'value': 'Deploy version 3 to production (updated)'}, 'id': 'yhDXbv1--zSbUnde0ywEQTXiWtvmCeql1tIanySIgOa9hYqSwpKJ905tBlmQ0x4pawUJzgszqlbKpcxkjOAYcg', 'owner': {'type': 'Object', 'value': {'key_id': '3a0cdba0da7c060610042783cc758f1f', 'key_label': 'default_admin'}}, 'status': {'type': 'String', 'value': 'pending'}, 'steps': {'type': 'String', 'value': '[{"description":"Build Docker image","status":"pending"},{"description":"Run migrations","status":"pending"},{"description":"Smoke test","status":"pending"}]'}, 'title': {'type': 'String', 'value': 'Deploy v3'}, 'updated_at': {'type': 'DateTime', 'value': '2026-11-02T04:42:38.282001+00:00'}}]}

--- goal_step_start (step 0) ---
Step 0 started: {'created_at': {'type': 'DateTime', 'value': '2026-11-02T04:42:38.271359+00:00'}, 'description': {'type': 'String', 'value': 'Deploy version 3 to production (updated)'}, 'id': 'yhDXbv1--zSbUnde0ywEQTXiWtvmCeql1tIanySIgOa9hYqSwpKJ905tBlmQ0x4pawUJzgszqlbKpcxkjOAYcg', 'owner': {'type': 'Object', 'value': {'key_id': '3a0cdba0da7c060610042783cc758f1f', 'key_label': 'default_admin'}}, 'status': {'type': 'String', 'value': 'in_progress'}, 'steps': {'type': 'String', 'value': '[{"description":"Build Docker image","status":"InProgress"},{"description":"Run migrations","status":"pending"},{"description":"Smoke test","status":"pending"}]'}, 'title': {'type': 'String', 'value': 'Deploy v3'}, 'updated_at': {'type': 'DateTime', 'value': '2026-11-02T04:42:38.291192+00:00'}}

--- goal_step_complete (step 0) ---
Step 0 completed: {'created_at': {'type': 'DateTime', 'value': '2026-11-02T04:42:38.271359+00:00'}, 'description': {'type': 'String', 'value': 'Deploy version 3 to production (updated)'}, 'id': 'yhDXbv1--zSbUnde0ywEQTXiWtvmCeql1tIanySIgOa9hYqSwpKJ905tBlmQ0x4pawUJzgszqlbKpcxkjOAYcg', 'owner': {'type': 'Object', 'value': {'key_id': '3a0cdba0da7c060610042783cc758f1f', 'key_label': 'default_admin'}}, 'status': {'type': 'String', 'value': 'in_progress'}, 'steps': {'type': 'String', 'value': '[{"description":"Build Docker image","result":"Docker image built: sha256:abc123","status":"Completed"},{"description":"Run migrations","status":"pending"},{"description":"Smoke test","status":"pending"}]'}, 'title': {'type': 'String', 'value': 'Deploy v3'}, 'updated_at': {'type': 'DateTime', 'value': '2026-11-02T04:42:38.297834+00:00'}}

--- goal_step_fail (step 1) ---
Step 1 failed: {'created_at': {'type': 'DateTime', 'value': '2026-11-02T04:42:38.271359+00:00'}, 'description': {'type': 'String', 'value': 'Deploy version 3 to production (updated)'}, 'id': 'yhDXbv1--zSbUnde0ywEQTXiWtvmCeql1tIanySIgOa9hYqSwpKJ905tBlmQ0x4pawUJzgszqlbKpcxkjOAYcg', 'owner': {'type': 'Object', 'value': {'key_id': '3a0cdba0da7c060610042783cc758f1f', 'key_label': 'default_admin'}}, 'status': {'type': 'String', 'value': 'in_progress'}, 'steps': {'type': 'String', 'value': '[{"description":"Build Docker image","result":"Docker image built: sha256:abc123","status":"Completed"},{"description":"Run migrations","error":"Migration failed: column already exists","status":"Failed"},{"description":"Smoke test","status":"pending"}]'}, 'title': {'type': 'String', 'value': 'Deploy v3'}, 'updated_at': {'type': 'DateTime', 'value': '2026-11-02T04:42:38.304249+00:00'}}

--- goal_complete ---
Goal completed: {'type': 'String', 'value': 'pending_review'}

--- goal_approve ---
Goal approved: {'type': 'String', 'value': 'in_progress'}

--- goal_reject ---
Goal rejected: {'type': 'String', 'value': 'failed'}

--- task_create ---
Created task: Nightly Backup (id: wQ9omT9JOet6Duhtkd8t_kwQLSPO6CkEKlC9oNsdpS2fLYvp6CwiHQNJaHGvxpPU-ol5FtmMChOF6W5fk5Lb-Q)

--- task_list ---
Tasks: {'count': 1, 'items': [{'cron': {'type': 'String', 'value': '0 2 * * *'}, 'description': {'type': 'String', 'value': 'Backup all databases nightly'}, 'id': 'wQ9omT9JOet6Duhtkd8t_kwQLSPO6CkEKlC9oNsdpS2fLYvp6CwiHQNJaHGvxpPU-ol5FtmMChOF6W5fk5Lb-Q', 'max_consecutive_failures': {'type': 'Integer', 'value': 3}, 'name': {'type': 'String', 'value': 'Nightly Backup'}, 'owner': {'type': 'Object', 'value': {'key_id': '3a0cdba0da7c060610042783cc758f1f', 'key_label': 'default_admin'}}}], 'total': 1}

--- task_get ---
Fetched: {'type': 'String', 'value': 'Nightly Backup'}

--- task_start ---
Task started: {'type': 'String', 'value': 'running'}

--- task_succeed ---
Task succeeded: {'consecutive_failures': {'type': 'Integer', 'value': 0}, 'cron': {'type': 'String', 'value': '0 2 * * *'}, 'description': {'type': 'String', 'value': 'Backup all databases nightly'}, 'id': 'wQ9omT9JOet6Duhtkd8t_kwQLSPO6CkEKlC9oNsdpS2fLYvp6CwiHQNJaHGvxpPU-ol5FtmMChOF6W5fk5Lb-Q', 'last_error': {'type': 'Null', 'value': None}, 'last_run': {'type': 'DateTime', 'value': '2026-11-02T04:42:38.365499+00:00'}, 'max_consecutive_failures': {'type': 'Integer', 'value': 3}, 'name': {'type': 'String', 'value': 'Nightly Backup'}, 'owner': {'type': 'Object', 'value': {'key_id': '3a0cdba0da7c060610042783cc758f1f', 'key_label': 'default_admin'}}, 'run_count': {'type': 'Integer', 'value': 1}, 'status': {'type': 'String', 'value': 'active'}, 'updated_at': {'type': 'DateTime', 'value': '2026-11-02T04:42:38.365499+00:00'}}

--- task_pause ---
Task paused: {'type': 'String', 'value': 'paused'}

--- task_resume ---
Task resumed: {'type': 'String', 'value': 'active'}

--- task_fail ---
Task failed: {'consecutive_failures': {'type': 'Integer', 'value': 1}, 'cron': {'type': 'String', 'value': '0 2 * * *'}, 'description': {'type': 'String', 'value': 'Backup all databases nightly'}, 'id': 'wQ9omT9JOet6Duhtkd8t_kwQLSPO6CkEKlC9oNsdpS2fLYvp6CwiHQNJaHGvxpPU-ol5FtmMChOF6W5fk5Lb-Q', 'last_error': {'type': 'String', 'value': 'Disk full'}, 'last_run': {'type': 'DateTime', 'value': '2026-11-02T04:42:38.394121+00:00'}, 'max_consecutive_failures': {'type': 'Integer', 'value': 3}, 'name': {'type': 'String', 'value': 'Nightly Backup'}, 'next_run': {'type': 'DateTime', 'value': '2026-03-22T02:00:00+00:00'}, 'owner': {'type': 'Object', 'value': {'key_id': '3a0cdba0da7c060610042783cc758f1f', 'key_label': 'default_admin'}}, 'run_count': {'type': 'Integer', 'value': 1}, 'status': {'type': 'String', 'value': 'active'}, 'updated_at': {'type': 'DateTime', 'value': '2026-11-02T04:42:38.394121+00:00'}}

--- task_due ---
Due tasks: {'count': 0, 'items': []}

--- task_delete ---
Task deleted

--- agent_create ---
Created agent: CodeReviewer (id: 7GLW_UXJ7GOv1ypDcueMGj_BEkLBRt8oIQCfWwYCrFZwuQsjFw8DfLaXScxVKrwEcCY-gU_1VdJr5Z9cvvAuew)

--- agent_list ---
Agents: {'count': 1, 'items': [{'deployment_id': {'type': 'String', 'value': 'deploy_test'}, 'id': '7GLW_UXJ7GOv1ypDcueMGj_BEkLBRt8oIQCfWwYCrFZwuQsjFw8DfLaXScxVKrwEcCY-gU_1VdJr5Z9cvvAuew', 'llm_model': {'type': 'String', 'value': 'gpt-6-luna'}, 'name': {'type': 'String', 'value': 'CodeReviewer'}, 'owner': {'type': 'Object', 'value': {'key_id': '3a0cdba0da7c060610042783cc758f1f', 'key_label': 'default_admin'}}, 'run_policy': {'type': 'String', 'value': '{"trigger":{"allow_write_operations":false,"write_tools":[]},"eval":{"allow_write_operations":false,"write_tools":[]},"customer":{"allow_write_operations":false,"write_tools":[]}}'}, 'system_prompt': {'type': 'String', 'value': 'You review code for correctness and style.'}, 'tools': {'type': 'Array', 'value': ['web_search', 'file_read']}}], 'total': 1}

--- agent_get ---
Fetched: {'type': 'String', 'value': 'CodeReviewer'}

--- agent_get_by_name ---
By name: {'type': 'String', 'value': 'CodeReviewer'}

--- agent_update ---
Updated agent prompt: {'type': 'String', 'value': 'You review code. Be concise.'}

--- agents_by_deployment ---
Agents in deploy_test: {'count': 0, 'items': []}
WARNING: agents_by_deployment omitted created agent 7GLW_UXJ7GOv1ypDcueMGj_BEkLBRt8oIQCfWwYCrFZwuQsjFw8DfLaXScxVKrwEcCY-gU_1VdJr5Z9cvvAuew; TODO: check/fix the server-side deployment lookup

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
Inserted document: xfPO_MBWecWeMm_Rf58f1fmdQ_QM1I1A3ADgG56deYOz5mF9mtsqr7IMyvUJmkDQc7Xy8RqKT2ZFzeQqjFVauw

--- kv_link ---
Linked: None

--- kv_get_links ---
Links for app:config: [{'collection': 'kv_links_example_py', 'created_at': '2026-11-02T04:42:39.057Z', 'document_id': 'xfPO_MBWecWeMm_Rf58f1fmdQ_QM1I1A3ADgG56deYOz5mF9mtsqr7IMyvUJmkDQc7Xy8RqKT2ZFzeQqjFVauw', 'field_path': None, 'last_accessed': '2026-11-02T04:42:39.057939Z', 'metadata': {}}]

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
  kv_operations:py:cache:product:1: {'price': 29.99, 'name': 'Product 1'}
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
Response: For traditional painting, the primary colors are **red, yellow, and blue**. For light and screens, they’re **red, green, and blue**.

--- Structured Output via SSE ---
JSON response: [
  {"name": "Earth", "diameter_km": 12742},
  {"name": "Mars", "diameter_km": 6779},
  {"name": "Jupiter", "diameter_km": 139820}
]

--- Blocking HTTP (for comparison) ---
Blocking response: Hello! 👋

=== Done ===
=== ekoDB Schedule Management Example (Python) ===

--- create_schedule ---
Created schedule: Nightly Report (id: 212f40aa-c7da-43ca-aacb-bfa41ee024e5)

--- list_schedules ---
Schedules: {'count': 1, 'schedules': [{'created_at': '2026-11-02T04:42:44.441846Z', 'cron_expression': '0 0 0 * * *', 'description': 'Generate and email nightly analytics report', 'enabled': True, 'function_label': 'schedule_noop_python_25170', 'id': '212f40aa-c7da-43ca-aacb-bfa41ee024e5', 'last_execution': None, 'name': 'Nightly Report', 'next_execution': '2026-11-03T00:00:00Z', 'parameters': {}, 'stats': {'avg_execution_time_ms': 0.0, 'failed_executions': 0, 'last_error': None, 'successful_executions': 0, 'total_executions': 0}, 'timezone': 'UTC', 'updated_at': '2026-11-02T04:42:44.441846Z'}]}

--- get_schedule ---
Fetched: Nightly Report cron=0 0 0 * * *

--- update_schedule ---
Updated cron: 0 30 1 * * *

--- trigger_schedule ---
Triggered: {'schedule_id': '212f40aa-c7da-43ca-aacb-bfa41ee024e5', 'status': 'triggered'}

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
  1. Score: 26.400, Matched: bio, title
  2. Score: 26.400, Matched: title, bio
  3. Score: 26.400, Matched: bio, title
  4. Score: 26.400, Matched: title, bio

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
  1. Score: 0.760
  2. Score: 0.759
  3. Score: 0.733

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.367, Matched: title, content
  2. Score: 0.879, Matched: content, title
  3. Score: 0.380, Matched:

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 6.600, Matched: title

9. Vector search with a metadata pre-filter (category = ml):
Found 2 documents in category "ml" (NLP excluded)
  1. Deep Learning Fundamentals (category: ml)
  2. Introduction to Machine Learning (category: ml)

Paired vector search: document=Rust Programming Answer, query=Rust Programming Question
=== Cleanup ===
✅ Deleted test collections

✅ Search examples completed!
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {'id': '7vlRWqin3s-e6aTi3ZquRAunfZuARkmphnGSkwFfmanV2xIupP47MQLwGAGyt1tE3PUZlEnZ6KqkuNhWr46gIA'}

=== Find by ID ===
Found: {'active': {'value': True, 'type': 'Boolean'}, 'created_at': {'type': 'String', 'value': '2026-11-01T23:42:45.054465'}, 'embedding': {'type': 'Array', 'value': [0.1, 0.2, 0.3, 0.4, 0.5]}, 'user_id': {'type': 'String', 'value': '550e8400-e29b-41d4-a716-446655440000'}, 'id': '7vlRWqin3s-e6aTi3ZquRAunfZuARkmphnGSkwFfmanV2xIupP47MQLwGAGyt1tE3PUZlEnZ6KqkuNhWr46gIA', 'categories': {'type': 'Array', 'value': ['electronics', 'computers']}, 'data': {'type': 'String', 'value': 'aGVsbG8gd29ybGQ='}, 'metadata': {'value': {'key': 'value', 'nested': {'deep': True}}, 'type': 'Object'}, 'name': {'type': 'String', 'value': 'Test Record'}, 'value': {'type': 'Integer', 'value': 42}, 'price': {'value': 99.99, 'type': 'Float'}, 'tags': {'value': ['tag1', 'tag2', 'tag3'], 'type': 'Array'}}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): True
  price (Decimal): 99.99
  created_at (DateTime): 2026-11-01 23:42:45.054465
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): ['tag1', 'tag2', 'tag3']
  metadata (Object): {'key': 'value', 'nested': {'deep': True}}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): ['electronics', 'computers']
  data (Bytes): 11 bytes
Plain record: {'active': True, 'created_at': '2026-11-01T23:42:45.054465', 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5], 'user_id': '550e8400-e29b-41d4-a716-446655440000', 'id': '7vlRWqin3s-e6aTi3ZquRAunfZuARkmphnGSkwFfmanV2xIupP47MQLwGAGyt1tE3PUZlEnZ6KqkuNhWr46gIA', 'categories': ['electronics', 'computers'], 'data': 'aGVsbG8gd29ybGQ=', 'metadata': {'key': 'value', 'nested': {'deep': True}}, 'name': 'Test Record', 'value': 42, 'price': 99.99, 'tags': ['tag1', 'tag2', 'tag3']}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'active': {'value': True, 'type': 'Boolean'}, 'data': {'value': 'aGVsbG8gd29ybGQ=', 'type': 'String'}, 'value': {'value': 100, 'type': 'Integer'}, 'price': {'value': 99.99, 'type': 'Float'}, 'created_at': {'value': '2026-11-01T23:42:45.054465', 'type': 'String'}, 'metadata': {'value': {'key': 'value', 'nested': {'deep': True}}, 'type': 'Object'}, 'user_id': {'type': 'String', 'value': '550e8400-e29b-41d4-a716-446655440000'}, 'embedding': {'type': 'Array', 'value': [0.1, 0.2, 0.3, 0.4, 0.5]}, 'categories': {'type': 'Array', 'value': ['electronics', 'computers']}, 'tags': {'type': 'Array', 'value': ['tag1', 'tag2', 'tag3']}, 'name': {'type': 'String', 'value': 'Updated Record'}, 'id': '7vlRWqin3s-e6aTi3ZquRAunfZuARkmphnGSkwFfmanV2xIupP47MQLwGAGyt1tE3PUZlEnZ6KqkuNhWr46gIA'}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: qi5a5d-BbhywNourD6NwXp3PW1n8mEX4iFlXIf5xP7vsQPz9w4yAjkfTX4ooQPcsxNrUak3IUE6-BNW7Ujmqlw

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
✓ Created native SWR script: github_user_native_py (cmk44uzSvkc160_NW4--yN-BO3pPghfWHywAt0u5j8MGjqJp9a9qyvGratV9sNJzSlq5MNl4eL6gu1GMwM9cKg)

First call (cache miss - will fetch from GitHub API):
  Response time: 311ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 3ms
  Speedup: 97.7x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit_py (X3IIbSJpgGxOeJ48o_LrskZ3lsS2qqCj5ZdeQ_xaXFkLyfBAvN0z5flc_to3o1HUVvg4JELfbqRoiikF_0nf1w)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail_py' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline_py (yfR2YSKzR3qDV4wGo8JIn_yarlaO8ICq6nk1_-Bz3dtqszqXbZjOO_9oYvnLFoApv3hoTmf7uOBi-Mx-7alNyg)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users_swr_py' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache_py (dWPE3_-b5XjvCCgTQB3U6ASxqIFijYhBk_4RZYk9SlEhtWSP5Pz7ZTebJHq8syhXbLZGoSfwSAIYLz7GiCEPaA)
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
✓ Created SWR script: fetch_api_user_py (6l901tmyq0vSyDKn4UUFHKxUIAfxLydeP6Q8WPLA8mo-lOp07QfAKhS82nrL0RdgpdV8-lD7S2O2e6e6nRcL3g)

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
Created Alice: $1000 - ID: r-XXWvHX5kQQ0apB3qpYKlEOmZrycT_B1Yv7ABl-LNf9y1ZM37wQQF2ICJvTVl7qBVgdkEyhzl8xB9uMrJef4Q
Created Bob: $500 - ID: RxPRWoEaLOITwkGbMS7jiZHsRvM_3Thdwr5ibV_QM1LAhyW6Nig_L3ZC2UswaHOJ-SrLUwivucnDycltz27fvA

=== Example 1: Begin Transaction ===
Transaction ID (server-default isolation): 9935eff9-a2eb-49b3-936f-1b849b29d457

=== Example 2: Operations within Transaction ===
Alice in transaction: {'id': 'r-XXWvHX5kQQ0apB3qpYKlEOmZrycT_B1Yv7ABl-LNf9y1ZM37wQQF2ICJvTVl7qBVgdkEyhzl8xB9uMrJef4Q', 'name': {'type': 'String', 'value': 'Alice'}, 'account_id': {'type': 'String', 'value': 'ACC001'}, 'balance': {'value': 800, 'type': 'Integer'}}
Bob in transaction: {'name': {'type': 'String', 'value': 'Bob'}, 'id': 'RxPRWoEaLOITwkGbMS7jiZHsRvM_3Thdwr5ibV_QM1LAhyW6Nig_L3ZC2UswaHOJ-SrLUwivucnDycltz27fvA', 'balance': {'type': 'Integer', 'value': 700}, 'account_id': {'value': 'ACC002', 'type': 'String'}}
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

✓ Verified committed balances: Alice=$800, Bob=$700

=== Example 5: Rollback Demo ===
New transaction: f957fef2-6827-43ce-be54-f2964c536361
Updated Bob: $700 → $600 (in transaction)
Status before rollback: Active
✓ Transaction rolled back

✓ Verified Bob remains $700 after rollback

=== Cleanup ===
✓ Deleted test account collection

✓ All client transaction examples completed
✓ Client created

=== Create User Function ===
Created user function with ID: ADheaPBRNzZx5DWV8tTAuUO-oIHQ5kXw3YKJ9Fax7wWeBgmuIV-USkrxtCp7y8_kihpZOd3pkirdl9zZOLhtbA

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

Created chat session: 30JSPiqVQUQWoIC12fnlUB4aQ8NCewHNOvyYFF0gCvt83o0kahizcCqp-GLjC2cWf8mocxpTkPc14ABZPr6UYw

Sending message: 'What is the capital of France?'
{"__progress":"received"}{"__progress":"generating","model":"gpt-5.6-luna","provider":"openai"}The capital of France is Paris.

--- Stream ended ---
Message ID: Cuqz-SLr845ALQppRnMtSSTsFGo5HBvVyYfk8g-DjlTorXEiVYUUgorIjxkEDkxpW8895mtfvvKaYXotHpO_-A
Execution time: 2734ms
Token usage: {'cached_input_tokens': 0, 'completion_tokens': 11, 'prompt_tokens': 23, 'thinking_tokens': 0, 'total_tokens': 34}

Full response: {"__progress":"received"}{"__progress":"generating","model":"gpt-5.6-luna","provider":"openai"}The capital of France is Paris....
=== WebSocket Subscription Example (Python) ===

✓ Authentication successful
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_py' ===
✓ Subscribed (subscription_id: sub_6beb1c5a47004aff98957934b9106327)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: KSDRvZI9BX-X25KfZugWEUlh-C-3Ri4GhuKkCkV3cs0LbhAcc_gXD3MJVX4XgaNqcsIo_Pz4uA47VaCYm5PJ6Q

  📡 Notification received:
     Event:      insert
     Collection: ws_subscribe_example_py
     Record IDs: KSDRvZI9BX-X25KfZugWEUlh-C-3Ri4GhuKkCkV3cs0LbhAcc_gXD3MJVX4XgaNqcsIo_Pz4uA47VaCYm5PJ6Q
     Timestamp:  2026-11-02T04:42:53.099716+00:00

Inserting record 2...
✓ Inserted: bnz0mQifvzArzt5AzIqNTklrePfrvvDGt-vyd0j-lKApaU8QpgX5FsrAwO7tuUJcIi5bak9Qmcrsv7wa7EPnjA

  📡 Notification received:
     Event:      insert
     Record IDs: bnz0mQifvzArzt5AzIqNTklrePfrvvDGt-vyd0j-lKApaU8QpgX5FsrAwO7tuUJcIi5bak9Qmcrsv7wa7EPnjA

=== Unsubscribing ===
✓ Unsubscribed: {'collection': 'ws_subscribe_example_py', 'found': True, 'unsubscribed': True}

=== Cleanup ===
✓ Deleted collection 'ws_subscribe_example_py'

✓ WebSocket subscription example completed successfully
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: TpUcvz9_LjvXBWOxzanWr4Xb_DJGWg8cPavErWZXokamYtLSp84WtTx0p8FydTpyd4eb9Y3jSFtA62HJIGl7_Q

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
   Inserted with ripple: {'id': 'Dppt1xz-VSgZHiW820j83FD9yDKox7mpXLn_KPZBm_DNffLcrTMU4UqznFRBpUzrk_dTFOejnbh5KDxPe0OASQ'}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {'id': 'XxEpgDGA9LjyurhAZl8uLj7W5J2OVVfi7NR8p9AwVSVpFTQajeElk2iNhIZEJxdM-Kjmwyn3r6SLsfAKuCJrLg'}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {'price': {'type': 'Integer', 'value': 150}, 'id': 'Dppt1xz-VSgZHiW820j83FD9yDKox7mpXLn_KPZBm_DNffLcrTMU4UqznFRBpUzrk_dTFOejnbh5KDxPe0OASQ', 'name': {'type': 'String', 'value': 'Product 1'}}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {'name': {'type': 'String', 'value': 'Upsert Product'}, 'id': 'Dppt1xz-VSgZHiW820j83FD9yDKox7mpXLn_KPZBm_DNffLcrTMU4UqznFRBpUzrk_dTFOejnbh5KDxPe0OASQ', 'price': {'type': 'Integer', 'value': 500}}

✅ All bypass_ripple operations completed successfully!
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields (id, name, email only)
  Found 3 active users
  Fields returned: ['id', 'name', 'email']
  First user: {'type': 'String', 'value': 'Alice Johnson'} <{'type': 'String', 'value': 'alice@example.com'}>

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ['age', 'status', 'avatar_url', 'id', 'created_at', 'user_role', 'name', 'bio', 'email']

Example 3: Complex query with projection (active users, ages 18-65)
  Found 3 active users (ages 18-65)
    - {'value': 'Dave Brown', 'type': 'String'} (age {'type': 'Integer', 'value': 45})
    - {'value': 'Alice Johnson', 'type': 'String'} (age {'type': 'Integer', 'value': 30})
    - {'value': 'Bob Smith', 'type': 'String'} (age {'value': 25, 'type': 'Integer'})

Example 4: Query inactive users with profile fields
  Found 1 inactive users
    - {'type': 'String', 'value': 'Carol White'}: {'type': 'String', 'value': 'Manager'}

Example 5: Compare full vs projected data
  Full query:
    - 12 fields per record
    - Fields: ['avatar_url', 'created_at', 'name', 'bio', 'password', 'email', 'id', 'user_role', 'age', 'secret_token', 'status', 'api_key']
  Projected query:
    - 3 fields per record
    - Fields: ['id', 'name', 'email']
  Bandwidth savings: ~75% fewer fields

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
