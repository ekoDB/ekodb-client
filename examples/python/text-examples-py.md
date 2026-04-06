make test-examples-python
🧪 [36mRunning Python examples (direct HTTP/WebSocket)...[0m
=== Simple CRUD Operations (Direct HTTP) ===

✓ Authentication successful

=== Insert Document ===
Inserted: {'id': 'hyfXdGLoUvMtmi1WmHgw9TIEqaLzp121-lIe2JlP2sYe6pgFuWTkMMicKxTNmbLypKGIDyfMY14ExX9sS8IFRQ'}

=== Find by ID ===
Found: {'id': 'hyfXdGLoUvMtmi1WmHgw9TIEqaLzp121-lIe2JlP2sYe6pgFuWTkMMicKxTNmbLypKGIDyfMY14ExX9sS8IFRQ', 'name': {'type': 'String', 'value': 'Test Record'}, 'active': {'value': True, 'type': 'Boolean'}, 'value': {'type': 'Integer', 'value': 42}}

=== Find with Query ===
Found documents: [{'value': {'value': 42, 'type': 'Integer'}, 'name': {'value': 'Test Record', 'type': 'String'}, 'active': {'value': True, 'type': 'Boolean'}, 'id': 'hyfXdGLoUvMtmi1WmHgw9TIEqaLzp121-lIe2JlP2sYe6pgFuWTkMMicKxTNmbLypKGIDyfMY14ExX9sS8IFRQ'}]

=== Update Document ===
Updated: {'id': 'hyfXdGLoUvMtmi1WmHgw9TIEqaLzp121-lIe2JlP2sYe6pgFuWTkMMicKxTNmbLypKGIDyfMY14ExX9sS8IFRQ', 'value': {'value': 100, 'type': 'Integer'}, 'active': {'type': 'Boolean', 'value': True}, 'name': {'value': 'Updated Record', 'type': 'String'}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: aB1SJ5GLXF3qgUKQhvdMwwsI18IxsbaRPTksSu-l_ozaRxe6uI_9DWWk3CCk09ZGNdzrFpPkZlWMZO-6q6bwmQ

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "messageId": "740956908",
  "payload": {
    "data": [
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "aB1SJ5GLXF3qgUKQhvdMwwsI18IxsbaRPTksSu-l_ozaRxe6uI_9DWWk3CCk09ZGNdzrFpPkZlWMZO-6q6bwmQ",
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
        "id": "-Y1QkUwIUNwv7K_DDT9HDqnUZnG4p2GiYfp32sra_Et5SNn0w6eMDDWj5odCNCHBH6pvyV46cJHNCiTKCU7Qxw",
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

✅ Function saved: IPNaHOMd80Y2teJZGXvnS7bSPfdhrjcUu223gbiq1RbTMR02GmVfj52QbXlf74rPk5hF-fTV-dvKBsfTu8dm4w
📊 Found 35 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Function saved: ZWewrgCwnfNLh3loUQNC1muYssBMnfNyqUMfiBLDiUQ4AjOsMrH6yvP2tkgfWp8VhEEzcf7mcJ0uJEzS6IHhqA
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: 9U7mCbLGhTZuLCLwnGECiI0sSc24mo1jZiC8OraLuI9DsuBrMYTGmLzbbuCsYCHiWYjF7BbhJ_X_lIfTFHVPOQ
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {'status': 'inactive', 'count': 15, 'avg_score': 50.0, 'max_score': 90}
   {'max_score': 100, 'status': 'active', 'avg_score': 60.0, 'count': 15}

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
Collection created with first record: vTsq1opYhpCM_vLJQktxowEn1ZISZDOHgu4muY3raKskfAkajKTl45FQvolJty3OHh_GWd0KrH5lcVmJ2uNjgQ

=== List Collections ===
Total collections: 13
Sample collections: ['test_collection', 'products', 'websocket_test', 'ttl_cache', 'functions__ek0_testing']

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
Created Alice: $1000 - ID: s-i-8fgle54iYrczIYbqxQjjUFGQC1QuajwS1R7yPhnesT8p89SF7YpxGTWbryB3uUF8UsiFYoMO3_91tilqGA
Created Bob: $500 - ID: MdCO49TbxUZQOScY700Sd-9bGPSmtduIG7tIOQUoIUiqf5jCpe-mbwjGv9GwDDIV6p5uteuqOZWv2WKTYeMKOg

=== Example 1: Begin Transaction ===
Transaction ID: 18b62751-6600-4f1e-a3ed-3f9c9e24780b

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
New transaction: b8ad4b4a-e91a-47ce-a350-25a680d64505
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'type': 'Integer', 'value': 700}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: d6M8BPMpmaDSSN3MxH2GRi4UUi6EfXwMJ4xKmY6rdFQKtP_C-BmSpD0Pf5PZWVu__SrYhYQlkJxYmt1iAXGQUA
Created Bob: $500 - ID: J5n7kVlL47dA2MOSzfJMqN8IbPCgQ9Ma6ziH2NiG6w4OWZCFrVTGNOOcjP8ucR7Ws-rHyuImUU7p4KCf3e-tyA

=== Example 1: Begin Transaction ===
Transaction ID: 4c64d817-5471-4556-a1e1-9d04f07a9827

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


🔄 Switching to NON-DURABLE mode...
   ✓ Config updated: durable_operations=false


============================================================
🔥 TEST 2: Non-Durable Mode (durable=false)
============================================================

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: LG8Pddx2rIizhMz8LbUrQdwIiGQHSaQjWczOSad38ZhKG5a2xi36Qx6Cu1dhezlbIQGqZawyljdE9KdBmzk43A
Created Bob: $500 - ID: 2dwj_EWG5rev05WLzV70uy4AJdgjcebZuM2OwXIPWOZwATSdmhtS0P_jXmVTqFRYokZuzU2carNBlmnWcAHkVQ

=== Example 1: Begin Transaction ===
Transaction ID: e65913f6-6661-44eb-813d-fdc500cb93d5

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
New transaction: 0a171c2b-4456-4d86-8bcf-3639530057a7
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'type': 'Integer', 'value': 700}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: kFK3gdKzmSlAHiLTm_pMykOsZnSN1iwUNbs0rPjIzPReM9pdbAu30qHVf4tjoENUcPEhxnRNpZAOxv18ifQQaw
Created Bob: $500 - ID: M-XcSdD5ycUdV-5ChuaUUycBP8ocE_4hJCVAk2q69PczEXXit433Re1Mr9LzgCE8SeIKwrfysYrLKY5mvlU3mw

=== Example 1: Begin Transaction ===
Transaction ID: 64941edb-5593-4a01-9b8d-92bcab13e40d

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
Created Alice: $1000 - ID: h9RbIqyMjQtAyeTgIDpbvbEvtIBfOT_8DH6tj3MN_cVF44GLL6F03nigzCI15c79IS-07IS7ZlU0h33sh2n5pw
Created Bob: $500 - ID: p9WqE0C-DEG4gFKL5A1MM-s6lkGMBSZhqr1J0PG7ix0QTGyFg4jZBHSWRtmHXZzeiqo9X35NQOhcHwQGVyTZqg

=== Example 1: Begin Transaction ===
Transaction ID: e6a878f8-85b3-40e5-91e0-172429a97fa2

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
New transaction: 6c372cc8-d23c-49f3-a4de-6259ddf81b65
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'type': 'Integer', 'value': 700}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: oDJ7bi-H3DnNG3iiCH8IZo6eZLB1BVBFbQ1cIQXtVqEe4ZQi6BVfV5U6VYdm7C5fP9p6IMbIald2QholjoVAug
Created Bob: $500 - ID: D5jM_5hhnlQgrQ0qo8x97aUnVXpeYPTlYMlTBYl0mTKQA-6_FLPRGOCsxwLMyFid0Nd8mYdvSupRDc24ECuCUw

=== Example 1: Begin Transaction ===
Transaction ID: 8f6ee223-8b94-4e4a-8721-8795404fd54b

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
📝 function 1: Insert + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: CI39idzSk0rVEHc1qjU2iNwXhZOJlh1hZA6SxS1P0GzTEYi6hT6UXdOmOEW3HCotvQBBtjtHZGdo7k6cXiNtsg

2️⃣ Calling function (Insert + Verify)...
   ✅ function executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: kJXPFtCTTct65n8xw8_HP7aFHabLPv6V9eYlpy_0CSzu4OZHMRd2-msHkUkTXP6ek6xo6uI_X5sYLy0X5LFijw
   📋 Name: {'value': 'Alice Smith', 'type': 'String'}
   📋 Email: {'value': 'alice@example.com', 'type': 'String'}
   📋 Status: {'type': 'String', 'value': 'pending'}
   📋 Credits: {'value': 0, 'type': 'Integer'}

============================================================
📝 function 2: Query + Update + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: XV96BUk81RuOKG5xTBzpanp9VNk9pzD22cHSjXNG-XCUeFz79bSms3cInI0gyYxLu2cMLdLmCZO-dXdYgTmtog

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
   ✅ Function saved: wWvhthPKy5_Qd8Lpl2-3h1Pa3sxPI5k5Fxg4Mw3QDELoGS35PdrCuPYSQdQlsots8Q7S0UJiv8okGkenF3p3pQ

2️⃣ Calling function (Query + Update Credits + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {'value': 0, 'type': 'Integer'}
   📋 Status: {'type': 'String', 'value': 'active'}
   📋 Name: {'value': 'Alice Smith', 'type': 'String'}

============================================================
📝 function 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: o46DU8j4K8_WhwGZwpw-26qVGvNysQ17ipB77Zaqc6-LgEi36dmgg66mKpg6hpZY3IMipHEZTRw3JB3HWQn9Pw

2️⃣ Calling function (Query + Delete + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: CI39idzSk0rVEHc1qjU2...
   ✅ Deleted script: XV96BUk81RuOKG5xTBzp...
   ✅ Deleted script: wWvhthPKy5_Qd8Lpl2-3...
   ✅ Deleted script: o46DU8j4K8_WhwGZwpw-...
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
  Output: Document ID = 5TgZ_voqG6XuN8nlwGcqSFuwHesfgfqn_UzzMr_ZUjaBd0dGofsjp-P1amofO4HYYGUswqM917HunJ1wEUDr7A
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: find_by_id(5TgZ_voqG6XuN8nlwGcqSFuwHesfgfqn_UzzMr_ZUjaBd0dGofsjp-P1amofO4HYYGUswqM917HunJ1wEUDr7A)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: find_by_id(5TgZ_voqG6XuN8nlwGcqSFuwHesfgfqn_UzzMr_ZUjaBd0dGofsjp-P1amofO4HYYGUswqM917HunJ1wEUDr7A)
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
  Output: Document ID = N8t8Tn3WT4EzJHxh3sB5svZpicmy7Y1WGcYDWfZZmAoHmGR91dET1-tKIi3BMGdrWEi3QmhP1s87jIAS12VcQw
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: find_by_id(N8t8Tn3WT4EzJHxh3sB5svZpicmy7Y1WGcYDWfZZmAoHmGR91dET1-tKIi3BMGdrWEi3QmhP1s87jIAS12VcQw)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: find_by_id(N8t8Tn3WT4EzJHxh3sB5svZpicmy7Y1WGcYDWfZZmAoHmGR91dET1-tKIi3BMGdrWEi3QmhP1s87jIAS12VcQw)
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
Inserted: {'id': 'oGdfKjvBL5_b6k1NGs7MJhdTYYgt1PW3fwalEGeVMHDVpPxsMhxVqpIBaj8P7f9WMmcblA5a7qKIC3DGVamUaA'}

=== Find by ID ===
Found: {'name': {'type': 'String', 'value': 'Test Record'}, 'embedding': {'value': [0.1, 0.2, 0.3, 0.4, 0.5], 'type': 'Array'}, 'id': 'oGdfKjvBL5_b6k1NGs7MJhdTYYgt1PW3fwalEGeVMHDVpPxsMhxVqpIBaj8P7f9WMmcblA5a7qKIC3DGVamUaA', 'price': {'value': 99.99, 'type': 'Float'}, 'data': {'type': 'String', 'value': 'aGVsbG8gd29ybGQ='}, 'active': {'value': True, 'type': 'Boolean'}, 'tags': {'type': 'Array', 'value': ['tag1', 'tag2', 'tag3']}, 'metadata': {'type': 'Object', 'value': {'nested': {'deep': True}, 'key': 'value'}}, 'user_id': {'value': '550e8400-e29b-41d4-a716-446655440000', 'type': 'String'}, 'categories': {'type': 'Array', 'value': ['electronics', 'computers']}, 'value': {'value': 42, 'type': 'Integer'}, 'created_at': {'type': 'String', 'value': '2026-04-06T01:57:35.829688'}}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): True
  price (Decimal): 99.99
  created_at (DateTime): 2026-04-06 01:57:35.829688
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): ['tag1', 'tag2', 'tag3']
  metadata (Object): {'nested': {'deep': True}, 'key': 'value'}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): ['electronics', 'computers']
  data (Bytes): 11 bytes
Plain record: {'name': 'Test Record', 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5], 'id': 'oGdfKjvBL5_b6k1NGs7MJhdTYYgt1PW3fwalEGeVMHDVpPxsMhxVqpIBaj8P7f9WMmcblA5a7qKIC3DGVamUaA', 'price': 99.99, 'data': 'aGVsbG8gd29ybGQ=', 'active': True, 'tags': ['tag1', 'tag2', 'tag3'], 'metadata': {'nested': {'deep': True}, 'key': 'value'}, 'user_id': '550e8400-e29b-41d4-a716-446655440000', 'categories': ['electronics', 'computers'], 'value': 42, 'created_at': '2026-04-06T01:57:35.829688'}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'user_id': {'type': 'String', 'value': '550e8400-e29b-41d4-a716-446655440000'}, 'categories': {'value': ['electronics', 'computers'], 'type': 'Array'}, 'data': {'type': 'String', 'value': 'aGVsbG8gd29ybGQ='}, 'value': {'value': 100, 'type': 'Integer'}, 'embedding': {'value': [0.1, 0.2, 0.3, 0.4, 0.5], 'type': 'Array'}, 'metadata': {'type': 'Object', 'value': {'key': 'value', 'nested': {'deep': True}}}, 'active': {'type': 'Boolean', 'value': True}, 'price': {'value': 99.99, 'type': 'Float'}, 'id': 'oGdfKjvBL5_b6k1NGs7MJhdTYYgt1PW3fwalEGeVMHDVpPxsMhxVqpIBaj8P7f9WMmcblA5a7qKIC3DGVamUaA', 'tags': {'type': 'Array', 'value': ['tag1', 'tag2', 'tag3']}, 'created_at': {'type': 'String', 'value': '2026-04-06T01:57:35.829688'}, 'name': {'type': 'String', 'value': 'Updated Record'}}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: kwRFOPwddFvYdejM6KGpHst2GAdkSm2A4RnwbJCXlUsItp_ld7Tht83txxJTp0Crp4AJd9Vvh92J_8LOv_q3Dg

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
Collection created with first record: "k9sdGA8PmpW2xuhbzWcY67_-o6wC05S9cSyRp4ACDvebiRREZJ3sKRFvaQs04ZOqQgMROLfLwUBZQ22hl9CKgw"

=== List Collections ===
Total collections: 12
Sample collections: ['test_collection', 'products', 'websocket_test', 'ttl_cache', 'functions__ek0_testing']

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
Created Alice: $1000 - ID: 2dtBRuxq5bz79QDPS7RrsRRa5musAPowUxajlrpRwnnaC-aBbektQmxSANJ4AQ1zd9uRlN9GGscMnX02MbFYLg
Created Bob: $500 - ID: kUtlTfR30Hef4WC_WbpaSs9CdJWHmq5NYZ48FLV9p7gQoGv6xMUEURBHKc5TSs7V7K0I8Oz4lk4FPfKA1qhEcA

=== Example 1: Begin Transaction ===
Transaction ID: f8e3fbbd-89c9-422a-babe-581754bddcd6

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: bbd57124-32d5-47e6-a041-87e4395e1256
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
  1. Score: 26.400, Matched: bio.value, title, bio, title.value
  2. Score: 26.400, Matched: bio.value, bio, title.value, title
  3. Score: 26.400, Matched: bio.value, title.value, bio, title
  4. Score: 26.400, Matched: bio.value, bio, title, title.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio, title.value, title, bio.value
  2. Score: 39.600, Matched: bio.value, title.value, title, bio
  3. Score: 39.600, Matched: title.value, bio, title, bio.value
  4. Score: 39.600, Matched: bio.value, title, title.value, bio

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
  1. Score: 0.792
  2. Score: 0.762
  3. Score: 0.755

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.378, Matched: content, title, title.value, content.value
  2. Score: 1.396, Matched: title, content, content.value, title.value
  3. Score: 0.381, Matched: 

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
✓ Inserted document: B_FTmbDQuyRJNw1lohjvRejZl4FZqh-ikqub8IQE8cfkOsxRr64IES9_hiUHz9_ztQbdPnDNpFU2LScqz3IZ-A

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: SDaTUKOQvZ4wTVF3X5tUvOqV0rne8izkCE7jTlUIUqk44KnEGe1cx9-oHw3C2ctWcaQz7B8BUqQm0QAPioq5Bg

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
✓ Inserted document with TTL: RpN0jIZwm3IpszZAeiyK9JqVtp3538zlY7d4hgyUN7pbU6z3_Zff6JKkvAnA3DO3fUVhOJ-14pv0-eu5bQ8ijQ

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
✓ Edge cache script created: 46l7uP7GwSf69z2-lCjlR_ryoNw5DIY5LQ0YpHVpETu6NKOVzjcaXkWB1SV0gMiLaZqa0j0omOg6VKZcr0YdAQ

Call 1: Cache miss (fetches from API)
Response time: 523ms
Result: {
  "records": [
    {
      "value": {
        "current": {
          "interval": 900,
          "temperature_2m": 6.0,
          "time": "2026-04-06T05:45"
        },
        "current_units": {
          "interval": "seconds",
          "temperature_2m": "\u00b0C",
          "time": "iso8601"
        },
        "elevation": 32.0,
        "generationtime_ms": 0.017523765563964844,
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
Response time: 42ms (12.3x faster!)
Result: {
  "records": [
    {
      "value": {
        "current": {
          "interval": 900,
          "temperature_2m": 6.0,
          "time": "2026-04-06T05:45"
        },
        "current_units": {
          "interval": "seconds",
          "temperature_2m": "\u00b0C",
          "time": "iso8601"
        },
        "elevation": 32.0,
        "generationtime_ms": 0.017523765563964844,
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

✅ Function saved: HQhISy_fl3YVfmHtbJyJM80iHRQ_AN-0GrqOEaa_k3AJiBGuw3DPsgH9xCt7viojZR0UG9JXhAhiNoMiCH4BFQ
📊 Found 10 active users

📝 Example 2: Parameterized Function

✅ Function saved: rxEpl8Qy4-KX7E-92KQAJdxnbtJVHNT4CiixD9bIzkEGmj8CW1Vyzi0SmHeuhCiaIwSpbnffPDnGcgmcqMxzyA
📊 Found 10 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: uYoaoCHHuj4gTxsRHblsS9sijkRaK8l6z7_iD5fUET6cNPt89UbQsAtP1h9mxGGs9kNYCh5tIB2uyoUyLEAjcA
📊 Statistics: 2 groups
   {'avg_score': 60.0, 'count': 5, 'status': 'active'}

   {'avg_score': 50.0, 'count': 5, 'status': 'inactive'}

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
   ⏱️  Duration: 38.0ms
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
   ⏱️  Duration: 38.3ms
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

✅ Function saved: bazY6TXZPLH0fYcSNLZouNKzi2Mm-aTq9sF8Ryyd0tklp4Ad19ykpVB4CqlbGLiaBFbsVOLspQ3jZc8AOx9BKw
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

✅ Inserted order: mrrf-d8Ln6mLDEIe3mFvyvZ29RdYT37Nkc9fpfqq19LX2y5Y2H0Og7d6TFTf-YQucTP14HUOcCDCofXLlb4gtQ
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

✅ Inserted order: in74U7hwB8fiuMzOoRFVAjunbUgFWdHqeHC3rznk46pta1SoM4QaDyY4emGdvlGHDDwf6Ln1TwyW1vWANVJzMw
✅ Cached order status
📊 Quick status lookup: {'value': '{"status":"processing","updated_at":"2026-04-06T05:57:38.604992+00:00"}'}

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
✓ Created SWR script: fetch_api_user_py (7n-UZ-YK-5rGrUiRbQavkr05dR9aprdq143WI9VdyrUQsuyx9yVYB7pAgv_xcAvWzMH6S2s2ly0rDmhXsAzTrw)

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
✓ Created native SWR script: github_user_native (W1xT0PDWpBL-Ynq_OU0LsfhOorF8PSGbm9zU8vzNA1JNdRnR0vPYJGtAbpHlc1ABB2W4Fz8ppoX1q7xyFB2qUA)

First call (cache miss - will fetch from GitHub API):
  Response time: 123ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 18ms
  Speedup: 7.0x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (KEDYwwR7l1B7Xin99OAGasH074oi4JFqTL6wS4nOqRBRD5L9Hug-kchjU0dJAWhRWgclOUfnVa2njlFHUM5aiQ)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (7O9QWpmyTBwn_my1NW-023evJqSroxZV1lfOsJsK7FzFj0q83czFPQkvn1CtSfV1WT2vTNcQvdy-GOKpL6iQ6g)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (CRphllXpqBQ6Z6pmVwqshx4gUosIapSKbFOWUpHdwJrcW-tLE-LFE2s2JKJn8kdYfElp2tUxO66H_wjb-UbcUA)
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
   Vector databases offer several benefits, including:

1. **Efficient Similarity Search**: They excel at handling high-dimensional vector data, enabling rapid nearest neighbor searches and similarity comparisons.

2. **Scalability**: Designed to scale with large datasets, allowing for efficient storage and retrieval of vast amounts of vector data.

3. **Flexibility**: Support various types of data, including text, images, and audio, making them versatile for different applications.

4. **Enhanced AI Capabilities**: Improve performance for machine learning and AI applications by supporting embeddings generated from models.

5. **Real-time Querying**: Enable real-time retrieval of relevant data points, crucial for applications like recommendation systems and natural language processing.

6. **High Performance**: Optimized for speed and efficiency, often utilizing specialized data structures and indexing techniques.

7. **Integration with AI Frameworks**: Frequently compatible with popular AI and machine learning frameworks for seamless workflow integration.

8. **Rich Querying**: Facilitate complex querying capabilities, including filtering and ranking based on multiple criteria. 

These advantages make vector databases particularly useful in fields like machine learning, natural language processing, and recommendation systems.
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
   1. Getting Started with ekoDB (Database)
   2. Database Design Principles (Database)
   3. Introduction to Machine Learning (AI)
   4. Natural Language Processing (AI)
   5. Vector Databases Explained (Database)
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
✓ Created session: reBocC0dkKKQT30ZXAe3Ad-0U_GUTjJE4CZZSEk9uDEvdg0lMMRdMGYBBHiUelD0Ms3X_RiwtAF15H2GzFu9Bg

=== Sending Chat Message ===
Message ID: JjnD0i5FwE_bJtzCSzFICszNCByVVdM0efeK5_lN1Vz5U7pr2MKzRWy6G3HvdceCalFbTqweNKjZPZkEG_qugA

=== AI Response ===
The available products and their prices are as follows:

1. **ekoDB Pro** - $299
2. **ekoDB** - $99
3. **ekoDB Cloud** - $499

Execution Time: 7331ms

=== Token Usage ===
Prompt tokens: 2772
Completion tokens: 49
Total tokens: 2821

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: DVdkESQJeCRghoZH2aeu-38T4X0GAuZoBreM6ENnKC5E34ZkkyHNB3sXb9_dLuXa31wbmv8rc14iNbJFWKl5AA

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: High-performance database product
- **Price**: $99

If you need more information or additional products, please let me know!

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['updated_at', 'llm_provider', 'token_usage', 'content', 'llm_model', 'role', 'context_snippets', 'created_at', 'id', 'chat_id'])
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
✓ Created second session: _TqQUDru657FYIO91SiEMHfnut6HjfeL9twRb2M98HEWZSZRdWO8h9xS489XTUW1T74A59gtqftDW8GRxjPhWQ
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
✓ Created session: mTCGfeh03CagCcNbWtbZuJ06U9VhaEvJ4cKQBYUiCjwZA5fTRD9jvGrreLB4SazgeKhUkofiI7cVALOqdWqq-A

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name:** ekoDB
- **Price:** $99

If you need more details or additional products, let me know!

✓ Message 2 sent
  Response: The price of ekoDB is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: P1kYpSuygsv2UqR4YKeaBtwGd3iEtYspORczEdIbBfCqaxIszKwgmkaem_-dinIr09KpFOxxTRpZI2S8G4ITiQ
  Parent: mTCGfeh03CagCcNbWtbZuJ06U9VhaEvJ4cKQBYUiCjwZA5fTRD9jvGrreLB4SazgeKhUkofiI7cVALOqdWqq-A

=== Listing Sessions ===
✓ Found 7 sessions
  Session 1: P1kYpSuygsv2UqR4YKeaBtwGd3iEtYspORczEdIbBfCqaxIszKwgmkaem_-dinIr09KpFOxxTRpZI2S8G4ITiQ (Untitled)
  Session 2: mTCGfeh03CagCcNbWtbZuJ06U9VhaEvJ4cKQBYUiCjwZA5fTRD9jvGrreLB4SazgeKhUkofiI7cVALOqdWqq-A (Untitled)
  Session 3: _TqQUDru657FYIO91SiEMHfnut6HjfeL9twRb2M98HEWZSZRdWO8h9xS489XTUW1T74A59gtqftDW8GRxjPhWQ (Untitled)
  Session 4: reBocC0dkKKQT30ZXAe3Ad-0U_GUTjJE4CZZSEk9uDEvdg0lMMRdMGYBBHiUelD0Ms3X_RiwtAF15H2GzFu9Bg (Untitled)
  Session 5: 3YM7IzSN6M0w3JrCh-00l84pGH06JmyfF15U__UKiWFuRd_e89HQL65ebo7Fr65ncJMNcpjpys4lQp84BIe85w (Untitled)
  Session 6: QJ1tRdmF2ChzbOxluHr79WKe9w7hPGSMmFlnfNS8_JUdgNjjWGNUx9eOei4nxDZR7AJOMdwOKkju_CtWylJYow (Untitled)
  Session 7: nx9B8dMhSoBFDc7TMj9VcUBs_3AYQiGlUruZhrVe7vbfO7ifnTm513v7gMsDU51k9hKAmiBWs2frRI8IlGklFA (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: P1kYpSuygsv2UqR4YKeaBtwGd3iEtYspORczEdIbBfCqaxIszKwgmkaem_-dinIr09KpFOxxTRpZI2S8G4ITiQ

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Dict Creation ===
✓ Created record with plain dict: {'id': 'P8KXGYR-waAYMMEWzMPShaK81ate7qY8Qmw7hWPI_99By5KJ0_cIWj9C8ZiPNiYEXRmsjDrdKqPxkj-16jZXew'}

=== Upsert Operation ===
✓ Upsert (update existing record): P8KXGYR-waAYMMEWzMPShaK81ate7qY8Qmw7hWPI_99By5KJ0_cIWj9C8ZiPNiYEXRmsjDrdKqPxkj-16jZXew
✓ Inserted second record: Bv6ZQyVax6kXkgfqNbIomnRHnACRbYd3MD1iYpSE7xWvntgUXG3OaD1m8qi0KqbpBkK-dKAp3jVRKw0l9qIGkA
✓ Upsert (update second record): Bv6ZQyVax6kXkgfqNbIomnRHnACRbYd3MD1iYpSE7xWvntgUXG3OaD1m8qi0KqbpBkK-dKAp3jVRKw0l9qIGkA

=== Find One Operation ===
✓ Found user by email: {'name': {'value': 'Alice Johnson', 'type': 'String'}, 'age': {'type': 'Integer', 'value': 29}, 'id': 'P8KXGYR-waAYMMEWzMPShaK81ate7qY8Qmw7hWPI_99By5KJ0_cIWj9C8ZiPNiYEXRmsjDrdKqPxkj-16jZXew', 'active': {'type': 'Boolean', 'value': True}, 'email': {'type': 'String', 'value': 'alice.j@newdomain.com'}}
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
   Inserted with ripple: {'id': '8c3W1avViwQ0PEggG4D85lRG7XwYXaBXpn1pQt-Kl5MCiT-4jHd-4T7IFK9vpRUPhFp-3rvehlYVJQ7Yz_zCRw'}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {'id': 'jBrm-DhHySl9Np2GGdbjG_50qoqOZOEjL7RVwngFchXp6EgdMqVFJE-vzyo4k-THUBQZV900nYCBu4dsETjQeQ'}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {'id': '8c3W1avViwQ0PEggG4D85lRG7XwYXaBXpn1pQt-Kl5MCiT-4jHd-4T7IFK9vpRUPhFp-3rvehlYVJQ7Yz_zCRw', 'name': {'type': 'String', 'value': 'Product 1'}, 'price': {'type': 'Integer', 'value': 150}}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {'price': {'value': 500, 'type': 'Integer'}, 'id': '8c3W1avViwQ0PEggG4D85lRG7XwYXaBXpn1pQt-Kl5MCiT-4jHd-4T7IFK9vpRUPhFp-3rvehlYVJQ7Yz_zCRw', 'name': {'type': 'String', 'value': 'Upsert Product'}}

✅ All bypass_ripple operations completed successfully!
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields (id, name, email only)
  Found 3 active users
  Fields returned: ['id', 'email', 'name']
  First user: {'type': 'String', 'value': 'Dave Brown'} <{'type': 'String', 'value': 'dave@example.com'}>

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ['email', 'created_at', 'avatar_url', 'user_role', 'status', 'age', 'id', 'bio', 'name']

Example 3: Complex query with projection (active users, ages 18-65)
  Found 3 active users (ages 18-65)
    - {'value': 'Dave Brown', 'type': 'String'} (age {'value': 45, 'type': 'Integer'})
    - {'value': 'Alice Johnson', 'type': 'String'} (age {'type': 'Integer', 'value': 30})
    - {'value': 'Bob Smith', 'type': 'String'} (age {'value': 25, 'type': 'Integer'})

Example 4: Query inactive users with profile fields
  Found 1 inactive users
    - {'value': 'Carol White', 'type': 'String'}: {'value': 'Manager', 'type': 'String'}

Example 5: Compare full vs projected data
  Full query:
    - 12 fields per record
    - Fields: ['password', 'status', 'bio', 'avatar_url', 'age', 'api_key', 'created_at', 'name', 'secret_token', 'id', 'email', 'user_role']
  Projected query:
    - 3 fields per record
    - Fields: ['email', 'name', 'id']
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
Created user function with ID: kz_2gu7wj7mvFi60JxHKaEz1PJ4LRZO-MgajqMVW8WYFd26GUtMbBcuADs_FKI-8o5mCn07qHUAl8ZBjoNUxFQ

=== Get User Function ===
Retrieved: get_active_users_py - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 25 user functions:
  - get_active_users: Get Active Users (Updated)
  - validate_user: Check if user exists
  - get_users_by_status: Get Users By Status
  - get_active_users_paginated: Get Active Users (Paginated)
  - get_verified_user: Get verified and validated user
  - get_active_users_paginated: Get Active Users (Paginated)
  - get_user_wrapper: Wrapper that calls fetch_user
  - get_active_users_updated: Get Active Users (Updated)
  - fetch_user: Fetch user by code
  - validate_user: Check if user exists
  - fetch_slim_user: Validate and slim down user
  - get_active_users: Get Active Users (Updated)
  - get_user_wrapper: Wrapper that calls fetch_user
  - cache_api_call_py: Cache External API Call
  - get_high_scoring_active_users: Get High Scoring Active Users
  - fetch_and_store_user: Fetch user from API and cache in KV
  - swr_user: SWR pattern for user data (KV-based)
  - get_active_users_updated: Get Active Users (Updated)
  - swr_user: SWR pattern for user data (KV-based)
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_user: Fetch user by code
  - get_verified_user: Get verified and validated user
  - get_users_by_status: Get Users By Status
  - fetch_slim_user: Validate and slim down user
  - get_active_users_py: Get Active Users

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
