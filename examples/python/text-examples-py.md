make test-examples-python
📦 Ensuring Python example dependencies in .venv...
🧪 Running Python examples (direct HTTP/WebSocket)...
=== Simple CRUD Operations (Direct HTTP) ===

✓ Authentication successful

=== Insert Document ===
Inserted: {'id': 'OELepit3Lgzj45HpBv_tsDerl9T_URMlDk63UPr-XxgHZTWqwqvRratrA1xXb88xlDO__UNoJrRMgqBk7Enzkg'}

=== Find by ID ===
Found: {'name': {'value': 'Test Record', 'type': 'String'}, 'active': {'type': 'Boolean', 'value': True}, 'id': 'OELepit3Lgzj45HpBv_tsDerl9T_URMlDk63UPr-XxgHZTWqwqvRratrA1xXb88xlDO__UNoJrRMgqBk7Enzkg', 'value': {'value': 42, 'type': 'Integer'}}

=== Find with Query ===
Found documents: [{'value': {'type': 'Integer', 'value': 42}, 'name': {'type': 'String', 'value': 'Test Record'}, 'id': 'OELepit3Lgzj45HpBv_tsDerl9T_URMlDk63UPr-XxgHZTWqwqvRratrA1xXb88xlDO__UNoJrRMgqBk7Enzkg', 'active': {'value': True, 'type': 'Boolean'}}]

=== Update Document ===
Updated: {'active': {'type': 'Boolean', 'value': True}, 'value': {'type': 'Integer', 'value': 100}, 'id': 'OELepit3Lgzj45HpBv_tsDerl9T_URMlDk63UPr-XxgHZTWqwqvRratrA1xXb88xlDO__UNoJrRMgqBk7Enzkg', 'name': {'type': 'String', 'value': 'Updated Record'}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: PMwDVoTXyzsbBfUOXD9YkJgWAPJ_GYDGEZaMtm2SX0kS94io2_aAx8opoI-GkqCD_Uf8Hm6fVWGkGn858T3shw

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "type": "Success",
  "payload": {
    "data": [
      {
        "value": {
          "type": "Integer",
          "value": 42
        },
        "name": {
          "value": "WebSocket Test Record",
          "type": "String"
        },
        "id": "PMwDVoTXyzsbBfUOXD9YkJgWAPJ_GYDGEZaMtm2SX0kS94io2_aAx8opoI-GkqCD_Uf8Hm6fVWGkGn858T3shw",
        "active": {
          "value": true,
          "type": "Boolean"
        }
      }
    ]
  },
  "messageId": "229620708"
}
✓ Retrieved 1 record via WebSocket

✓ WebSocket example completed successfully
🚀 ekoDB Functions Example (Python/HTTP)

✓ Authentication successful

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Function saved: LBoNX32cBSmZjwRwpbr4sr2F0lY6RAJVr_TIVAI8qMU7YyEan4hyU2GswQRLvyO1SCxLxrMENN_iwZtaxxFraQ
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Function saved: LMek5zyg4OfSrCrU-2ylqTDE8stq5rZ7pFJ6kZ9pA-gWGfvOeLjnXNcvMiygjN0AvgkjZII97UQDikRKhDT0iA
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: AcUsoNfUcXIdBmsPgubcHCuq9gvqJPrXi1byDDjxExvvzm6bskGvgzZs27r3Q6SJBqqYJ4Gea9TcR-lUf7DgfQ
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {'avg_score': {'value': 50.0, 'type': 'Float'}, 'count': {'value': 5, 'type': 'Integer'}, 'status': {'value': 'inactive', 'type': 'String'}, 'max_score': {'type': 'Integer', 'value': 90}}
   {'max_score': {'type': 'Integer', 'value': 100}, 'avg_score': {'type': 'Float', 'value': 60.0}, 'count': {'type': 'Integer', 'value': 5}, 'status': {'type': 'String', 'value': 'active'}}

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
Retrieved value: {'value': {'username': 'john_doe', 'userId': 123}, 'type': 'Object'}

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
kv_operations:direct:py:cache:product:1: {'value': {'price': 29.99, 'name': 'Product 1'}, 'type': 'Object'}
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
Collection created with first record: JiKSKO-sKIa_Pga8xPIcc5RmsN9IVl6uzY4bVRJKZoR3c-03SOSrBmnbrEXS5d30TqKAlBlG-SJ9KhjO4HnDIQ

=== List Collections ===
Total collections: 13
Sample collections: ['chat_raw_completions__ek0_testing', 'chat_agent_configs__ek0_testing', 'schedules__ek0_testing', 'functions__ek0_testing', 'audit__ek0_testing']

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
Created Alice: $1000 - ID: R7XdyMgIGe3iVeZZIz_1XGolkHSYSZ1pfNPT3_ZaulN9JjsK9Hm-nNvjHlQdCc2KMAiTbwJsEDNHvA0LZbm_vg
Created Bob: $500 - ID: vinnfCdvnx4YNYj02pKondcXRwzsAv1W6RA214d05UlzueumotgHydJXeexdvFU0aWwjuZDtbbRH7_WLqoLj0A

=== Example 1: Begin Transaction ===
Transaction ID: 2e723d36-195e-4ccb-8076-9918e047de94

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
New transaction: 34596cfa-947b-4733-a9f2-512200e803cf
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'value': 700, 'type': 'Integer'}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: FV8UhBRQMiD5O5DhuDTuOEULg4Gjo8HSi0K3Zn0MJ3kCk94-_bYebA8P_gqm1vHQXmim9EWlO4fOOZzUcQQLEA
Created Bob: $500 - ID: iNgrLOLYsEMWd8wpSbKC7dpJloDZxHdg7ke6CQROG-n8gpZD_ELNfx2ul6e1Gr2_R0PhH1C6lZB2GESOo5dbcg

=== Example 1: Begin Transaction ===
Transaction ID: cc800e30-caa7-4070-a447-0c87f28f456d

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
Created Alice: $1000 - ID: Or44EmU_6FYJr0E8yVWfHhCXZZFh1vlItbDk2vpAt13Pyr9t_Iec6YETZvIBU3HZ14iVwPdour6msdwmOLeaCg
Created Bob: $500 - ID: rAskWtvq-Fd7k0sN6hJxY-zmvprDcMCLirZiGC5v6cyQ-h1uDkNYM_UECOoFVHWWd2qzLUZ6VLHyHft7pR3PtQ

=== Example 1: Begin Transaction ===
Transaction ID: 850a9aab-0834-4538-86c4-e64e1dee9fa7

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
New transaction: ebf95359-3f13-4515-98ed-79a8cfc4514d
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'value': 700, 'type': 'Integer'}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: iU76r7oQ_2sa2FrfOrlC4f_EXQbvU4riYIkKzlAcXFhcdUcqXvTiI-OmVXjrzEK6fQob_l1T2h4_iGSN0I5CeQ
Created Bob: $500 - ID: xoiMvw_2SrPuPOXarUXg47i3ijtE9-gbLsfgDLIHl7ruiC9jDi-wsxOEtwPEVzj0G3MQnpA3ZsPsdr_2r1P2Zg

=== Example 1: Begin Transaction ===
Transaction ID: 273c19fc-2f27-4a9b-8121-f7499a732ccb

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
Created Alice: $1000 - ID: j27Fy7egSEpVak7n9xJ3ZDjCpU1G4GaE-SllOk_FIJghjsZePFOhm_2bSuOIC1Few2PMj4JroiytIPHrVJQArg
Created Bob: $500 - ID: o8BKwxdK2FubKnv2J0TXUOqV4NHJztKYk0QKqo6g_YBYQsIZmAneT91vKMDK9XsCNMhRUef6E6fMkbpqzS43WA

=== Example 1: Begin Transaction ===
Transaction ID: e0785a45-89e5-4846-816b-509ca0d29403

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
New transaction: ed3c7e9e-5d6f-4841-bd59-03b1f95723fb
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'type': 'Integer', 'value': 700}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: MCL2bAgU23RdS9Ke2-e9f2lMcfeGqWyFN5RYdq7xyevRAjH4GGzDGIR8rRl1Dm7NIWKXrqSmubP-5CD8v49pLw
Created Bob: $500 - ID: E2vFLrK5J-imsTJzy3j9b-TuKA8EFEqlkXF7eZMv5ugSxfqICdEr21tnS33uQW1uryPAMcqlHl7qj2QSBQz3hA

=== Example 1: Begin Transaction ===
Transaction ID: 05d570d9-6cbc-48ce-b620-ffc9ab00907b

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
   ✅ Function saved: SC4dHLlFHj-lScnCbqApNvRCvr74FAXaDi4nJKvDieX1r11bzp9xyUHvS9hBqBckw0ZOISqsUpujdjzpoPwwww

2️⃣ Calling function (Insert + Verify)...
   ✅ function executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: 8AxfXhMTQgNfX3d42Oa5k6og7z8IRU_fuJMYpFavwjZOl9XjWBjFuxLXiWvFjj_-Ov8nMVNug1m3dVTVXesJSg
   📋 Name: {'value': 'Alice Smith', 'type': 'String'}
   📋 Email: {'type': 'String', 'value': 'alice@example.com'}
   📋 Status: {'type': 'String', 'value': 'pending'}
   📋 Credits: {'type': 'Integer', 'value': 0}

============================================================
📝 function 2: Query + Update + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: 9FT-bkhPNQ3naWwNRw2Zimmm9ljJXdzzHWnkP-B9kNUhE7QqFeUbsYGVcJx5-rR22sCriUZONbqBv58DH_oANg

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
   ✅ Function saved: HhIAcaH9bH-IbuioRrFy9NonswwqAuNiLMHHvynzLfCK39LwIK3W5gfKZsJG4BgtTHFuej-bu_anva1U9xa4Pw

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
   ✅ Function saved: 8QvM3vAkaY4XCjEzGnB_Woz37rhtdeUA5Cziu84k3Y1b6HecsgZ_wJ4r_Q-kQvrHMy3kno3XTzFMPk1ZdHoy3Q

2️⃣ Calling function (Query + Delete + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: SC4dHLlFHj-lScnCbqAp...
   ✅ Deleted script: 9FT-bkhPNQ3naWwNRw2Z...
   ✅ Deleted script: HhIAcaH9bH-IbuioRrFy...
   ✅ Deleted script: 8QvM3vAkaY4XCjEzGnB_...
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
  Output: Document ID = wxsAi_MqhN_-GTEgvrbD8EKZuxSIkpZahcrAZInFrFCWhLd6mPsFIU33XIIwNuksfgh_s0Q8YJGlV9F6Q6hFKg
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: find_by_id(wxsAi_MqhN_-GTEgvrbD8EKZuxSIkpZahcrAZInFrFCWhLd6mPsFIU33XIIwNuksfgh_s0Q8YJGlV9F6Q6hFKg)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: find_by_id(wxsAi_MqhN_-GTEgvrbD8EKZuxSIkpZahcrAZInFrFCWhLd6mPsFIU33XIIwNuksfgh_s0Q8YJGlV9F6Q6hFKg)
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
  Output: Document ID = T8Dnoux2iwX8ljnnh4dvWo3UZwV3-d3RGJApNngR7zHxlWw98LMy5AXUYFRgjGo6F17tCz37FKfiIPHq55zqlg
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: find_by_id(T8Dnoux2iwX8ljnnh4dvWo3UZwV3-d3RGJApNngR7zHxlWw98LMy5AXUYFRgjGo6F17tCz37FKfiIPHq55zqlg)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: find_by_id(T8Dnoux2iwX8ljnnh4dvWo3UZwV3-d3RGJApNngR7zHxlWw98LMy5AXUYFRgjGo6F17tCz37FKfiIPHq55zqlg)
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
    Finished `release` profile [optimized] target(s) in 0.16s
📦 Built wheel for abi3 Python ≥ 3.8 to ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.32.0-cp38-abi3-macosx_11_0_arm64.whl
📦 Installing Python wheel into .venv...
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.32.0-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.32.0
    Uninstalling ekodb_client-0.32.0:
      Successfully uninstalled ekodb_client-0.32.0
Successfully installed ekodb-client-0.32.0
🧪 Ensuring test dependencies (pytest) in .venv...
✅ Python client package built and installed!
Admin permissions read successfully
=== ekoDB Advanced CRUD Example (Python) ===

--- insert ---
Inserted: r0YMs22Jh-dPVO2g08wOTfexG_qxWAF5krbVQ6tBTRW3tqYdPmIQVS6PJ6XA3aTWyj1RGo4YE2MYUNQtzZ6-NA

--- update_with_action (increment) ---
After increment count by 5: {'score': {'value': 50.0, 'type': 'Float'}, 'count': {'type': 'Integer', 'value': 15}, 'id': 'r0YMs22Jh-dPVO2g08wOTfexG_qxWAF5krbVQ6tBTRW3tqYdPmIQVS6PJ6XA3aTWyj1RGo4YE2MYUNQtzZ6-NA', 'tags': {'value': ['initial'], 'type': 'Array'}, 'name': {'value': 'Counter Record', 'type': 'String'}}

--- update_with_action (decrement) ---
After decrement score by 10: {'name': {'type': 'String', 'value': 'Counter Record'}, 'count': {'value': 15, 'type': 'Integer'}, 'id': 'r0YMs22Jh-dPVO2g08wOTfexG_qxWAF5krbVQ6tBTRW3tqYdPmIQVS6PJ6XA3aTWyj1RGo4YE2MYUNQtzZ6-NA', 'tags': {'value': ['initial'], 'type': 'Array'}, 'score': {'type': 'Float', 'value': 40.0}}

--- update_with_action_sequence ---
After action sequence: {'name': {'value': 'Counter Record', 'type': 'String'}, 'count': {'type': 'Integer', 'value': 115}, 'tags': {'value': ['initial', 'sequenced'], 'type': 'Array'}, 'score': {'type': 'Float', 'value': 40.0}, 'id': 'r0YMs22Jh-dPVO2g08wOTfexG_qxWAF5krbVQ6tBTRW3tqYdPmIQVS6PJ6XA3aTWyj1RGo4YE2MYUNQtzZ6-NA'}

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
✓ Created session: QtPLxQJYy2M0cRiArnUGNoM8j9h9bIpfLXdSvHAJe8OqK2L3sSY_zMASmkHZpdicAC4vop8W-pcL-OV9sWoJfg

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is **ekoDB**, a high-performance database product priced at **$99**.

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['llm_provider', 'content', 'chat_id', 'llm_model', 'context_snippets', 'updated_at', 'token_usage', 'created_at', 'role', 'id', 'author'])
Debug: First message role: {'type': 'String', 'value': 'assistant'}
=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: ekoDB costs **$99**.

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: 1W_q_bIMG38iwwd-q4M57r7mI3f4eo2HTpmc1vZmTIAfAkIAJDbflEocLOoVmRP3wjkez2LMngNpBRonrC9MmQ
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 7

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 6

=== Cleanup ===
✓ Deleted chat session: 1W_q_bIMG38iwwd-q4M57r7mI3f4eo2HTpmc1vZmTIAfAkIAJDbflEocLOoVmRP3wjkez2LMngNpBRonrC9MmQ
✓ Deleted chat session: QtPLxQJYy2M0cRiArnUGNoM8j9h9bIpfLXdSvHAJe8OqK2L3sSY_zMASmkHZpdicAC4vop8W-pcL-OV9sWoJfg
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: Nuggvdpkd_XPSgCOKunOMmAUj8Z5eFf1mlCEA6o8tlzKIB2bRPZOIDhpJLPo1d2U9cxNQEA6_aHhugdBfOnNyw

=== Sending Chat Message ===
Message ID: _ECIIFUCuaaP02ne_A-As0_KEhj-jlbUWHZWH4l63X394qIw1IPl0e78lgXttTPCrR5HnFbSsRNMW5EB86JIrw

=== AI Response ===
Available products and prices:

- **ekoDB** — 99
- **ekoDB Pro** — 299
- **ekoDB Cloud** — 499

Execution Time: 6161ms

=== Token Usage ===
Prompt tokens: 55699
Completion tokens: 253
Total tokens: 55952

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Message Stream (SSE) Example (Python) ===

Created session: Sl_qhnJmRYr-xxeRIVU_tLaF-A-byMgkPleiRGT5ZjQGGQUbFDeB123ejhwR6DqyLXJ1g3iJ9zzvjAwDP66XlA

Streaming response for: 'What is ekoDB?'

{"__progress":"received"}{"__progress":"generating","model":"gpt-6-luna","provider":"openai"}**ekoDB** is a database platform for building applications. The name can refer to different projects, so its exact features depend on which one you mean. If you share a link or where you saw it, I can give you a more precise explanation.

--- Stream complete ---
Message ID: 21F_EQv5Xkba86e8nF_ZtPYW2tV8oSz6VlLRHAggt_jzsNKVjMZM_6xfvRHb0LHTWHqsjK4FOYAvwzstuuORDQ
Execution time: 6338ms
Context window: 128000 tokens

✓ Chat message stream example completed
✓ Client created

=== Get All Chat Models ===
OpenAI models: ['tts-1', 'gpt-5-mini', 'gpt-5-mini-2025-08-07', 'gpt-5.4-2026-03-05', 'gpt-5-nano', 'gpt-5.4-mini', 'omni-moderation-2024-09-26', 'gpt-live-transcribe', 'gpt-5.6-luna', 'gpt-4o-2024-11-20', 'gpt-5.1', 'whisper-1', 'gpt-5-pro', 'gpt-realtime-whisper', 'omni-moderation-latest', 'gpt-realtime-2', 'gpt-5-search-api', 'gpt-5.5-2026-04-23', 'gpt-5.5-pro', 'gpt-audio-2025-08-28', 'gpt-4o-mini-transcribe-2025-03-20', 'chat-latest', 'gpt-realtime-translate', 'gpt-5.4-pro-2026-03-05', 'gpt-4o-mini-transcribe-2025-12-15', 'gpt-5-pro-2025-10-06', 'gpt-5.6-sol', 'gpt-5.4-mini-2026-03-17', 'text-embedding-3-large', 'gpt-4o-2024-08-06', 'o3', 'gpt-5.4-pro', 'gpt-4o-mini-tts', 'gpt-realtime-mini-2025-12-15', 'gpt-realtime-2.1-mini', 'gpt-realtime-2025-08-28', 'tts-1-hd-1106', 'tts-1-hd', 'gpt-realtime-1.5', 'gpt-realtime', 'gpt-5.2-pro', 'gpt-4o-transcribe-diarize', 'gpt-5.1-2025-11-13', 'gpt-5.4', 'text-embedding-3-small', 'gpt-5-2025-08-07', 'o3-2025-04-16', 'gpt-5.2-2025-12-11', 'chatgpt-image-latest', 'tts-1-1106', 'gpt-5.5-pro-2026-04-23', 'gpt-5.2-pro-2025-12-11', 'gpt-5.4-nano', 'gpt-audio-mini-2025-12-15', 'gpt-audio-mini', 'gpt-4.1-mini-2025-04-14', 'gpt-5', 'gpt-4o-mini-transcribe', 'gpt-image-2-2026-04-21', 'gpt-transcribe', 'gpt-image-2', 'text-embedding-ada-002', 'gpt-realtime-2.1', 'gpt-4.1', 'gpt-5.5', 'gpt-audio-1.5', 'gpt-image-1.5', 'gpt-6.1-sol', 'gpt-6-sol', 'gpt-realtime-mini', 'gpt-image-1-mini', 'gpt-5.6-terra', 'gpt-5-nano-2025-08-07', 'gpt-4o-transcribe', 'gpt-4.1-mini', 'gpt-4o-mini', 'gpt-6.1-terra', 'gpt-6-luna', 'gpt-4o-mini-tts-2025-12-15', 'gpt-audio', 'gpt-5.3-codex', 'gpt-5-search-api-2025-10-14', 'gpt-4o', 'gpt-image-2.6-flare', 'gpt-4o-mini-2024-07-18', 'gpt-4.1-2025-04-14', 'gpt-image-2.5-flare', 'gpt-4o-mini-tts-2025-03-20', 'gpt-5.2', 'gpt-5.4-nano-2026-03-17', 'gpt-6-astra', 'gpt-image-2.5-sunburst-2026-09-08', 'gpt-image-2.5-sunburst', 'gpt-image-2.5-flare-2026-09-08', 'gpt-image-2.6-sunburst', 'gpt-live-1']
Anthropic models: ['claude-haiku-5-5', 'claude-sonnet-5-5', 'claude-opus-5-5', 'claude-fable-5-1', 'claude-opus-5', 'claude-sonnet-5', 'claude-fable-5', 'claude-opus-4-8', 'claude-opus-4-7', 'claude-sonnet-4-6', 'claude-opus-4-6', 'claude-opus-4-5-20251101', 'claude-haiku-4-5-20251001', 'claude-sonnet-4-5-20250929']
Perplexity models: []
Gemini models: []

=== Get OpenAI Models ===
OpenAI models: ['tts-1', 'gpt-5-mini', 'gpt-5-mini-2025-08-07', 'gpt-5.4-2026-03-05', 'gpt-5-nano', 'gpt-5.4-mini', 'omni-moderation-2024-09-26', 'gpt-live-transcribe', 'gpt-5.6-luna', 'gpt-4o-2024-11-20', 'gpt-5.1', 'whisper-1', 'gpt-5-pro', 'gpt-realtime-whisper', 'omni-moderation-latest', 'gpt-realtime-2', 'gpt-5-search-api', 'gpt-5.5-2026-04-23', 'gpt-5.5-pro', 'gpt-audio-2025-08-28', 'gpt-4o-mini-transcribe-2025-03-20', 'chat-latest', 'gpt-realtime-translate', 'gpt-5.4-pro-2026-03-05', 'gpt-4o-mini-transcribe-2025-12-15', 'gpt-5-pro-2025-10-06', 'gpt-5.6-sol', 'gpt-5.4-mini-2026-03-17', 'text-embedding-3-large', 'gpt-4o-2024-08-06', 'o3', 'gpt-5.4-pro', 'gpt-4o-mini-tts', 'gpt-realtime-mini-2025-12-15', 'gpt-realtime-2.1-mini', 'gpt-realtime-2025-08-28', 'tts-1-hd-1106', 'tts-1-hd', 'gpt-realtime-1.5', 'gpt-realtime', 'gpt-5.2-pro', 'gpt-4o-transcribe-diarize', 'gpt-5.1-2025-11-13', 'gpt-5.4', 'text-embedding-3-small', 'gpt-5-2025-08-07', 'o3-2025-04-16', 'gpt-5.2-2025-12-11', 'chatgpt-image-latest', 'tts-1-1106', 'gpt-5.5-pro-2026-04-23', 'gpt-5.2-pro-2025-12-11', 'gpt-5.4-nano', 'gpt-audio-mini-2025-12-15', 'gpt-audio-mini', 'gpt-4.1-mini-2025-04-14', 'gpt-5', 'gpt-4o-mini-transcribe', 'gpt-image-2-2026-04-21', 'gpt-transcribe', 'gpt-image-2', 'text-embedding-ada-002', 'gpt-realtime-2.1', 'gpt-4.1', 'gpt-5.5', 'gpt-audio-1.5', 'gpt-image-1.5', 'gpt-6.1-sol', 'gpt-6-sol', 'gpt-realtime-mini', 'gpt-image-1-mini', 'gpt-5.6-terra', 'gpt-5-nano-2025-08-07', 'gpt-4o-transcribe', 'gpt-4.1-mini', 'gpt-4o-mini', 'gpt-6.1-terra', 'gpt-6-luna', 'gpt-4o-mini-tts-2025-12-15', 'gpt-audio', 'gpt-5.3-codex', 'gpt-5-search-api-2025-10-14', 'gpt-4o', 'gpt-image-2.6-flare', 'gpt-4o-mini-2024-07-18', 'gpt-4.1-2025-04-14', 'gpt-image-2.5-flare', 'gpt-4o-mini-tts-2025-03-20', 'gpt-5.2', 'gpt-5.4-nano-2026-03-17', 'gpt-6-astra', 'gpt-image-2.5-sunburst-2026-09-08', 'gpt-image-2.5-sunburst', 'gpt-image-2.5-flare-2026-09-08', 'gpt-image-2.6-sunburst', 'gpt-live-1']

=== Get Anthropic Models ===
Anthropic models: ['claude-haiku-5-5', 'claude-sonnet-5-5', 'claude-opus-5-5', 'claude-fable-5-1', 'claude-opus-5', 'claude-sonnet-5', 'claude-fable-5', 'claude-opus-4-8', 'claude-opus-4-7', 'claude-sonnet-4-6', 'claude-opus-4-6', 'claude-opus-4-5-20251101', 'claude-haiku-4-5-20251001', 'claude-sonnet-4-5-20250929']

✓ Chat Models API example complete
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: XYLVXt4wvzFPOWkgsymxmrWYASThgu0hDFXVmAwgT-AUQJRJsHI-Fhdp_RSoY-Yepw4YMeRK4UAYCN_EUACwtg

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is **ekoDB**, a high-performance database product priced at **$99**.

✓ Message 2 sent
  Response: ekoDB costs **$99**.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: XGcalUqGkvWMGHrAq1pTP4NQNQZJbx0kjRgrfhMrKtXv_cLiaI847rJf4mliS5Df7y0DmygttogL_Tq53GoenQ
  Parent: XYLVXt4wvzFPOWkgsymxmrWYASThgu0hDFXVmAwgT-AUQJRJsHI-Fhdp_RSoY-Yepw4YMeRK4UAYCN_EUACwtg

=== Listing Sessions ===
✓ Found 2 sessions
  Session 1: XGcalUqGkvWMGHrAq1pTP4NQNQZJbx0kjRgrfhMrKtXv_cLiaI847rJf4mliS5Df7y0DmygttogL_Tq53GoenQ (Untitled)
  Session 2: XYLVXt4wvzFPOWkgsymxmrWYASThgu0hDFXVmAwgT-AUQJRJsHI-Fhdp_RSoY-Yepw4YMeRK4UAYCN_EUACwtg (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: XGcalUqGkvWMGHrAq1pTP4NQNQZJbx0kjRgrfhMrKtXv_cLiaI847rJf4mliS5Df7y0DmygttogL_Tq53GoenQ

=== Cleanup ===
✓ Deleted sessions and collection

✓ All session management operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: "3aEP3fEASx2H9RjqdOnRfcVSQU2iQmwKo1CzvxqWO3agUVVW6hM1wBevgpzi2o973R-yhXBisu0g9y6wZZ1t1w"

=== List Collections ===
Total collections: 13
Sample collections: ['chat_raw_completions__ek0_testing', 'chat_agent_configs__ek0_testing', 'schedules__ek0_testing', 'functions__ek0_testing', 'audit__ek0_testing']

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
✓ Created record with plain dict: {'id': 'VGmvuet4gkfu_YKwp9qsdwy1n2qI2LjQjQmxgjPQUJO5leppTG0hDP5r2zh2kYpkq6MQxz5FR7D6x1eiAkEOeQ'}

=== Upsert Operation ===
✓ Upsert (update existing record): VGmvuet4gkfu_YKwp9qsdwy1n2qI2LjQjQmxgjPQUJO5leppTG0hDP5r2zh2kYpkq6MQxz5FR7D6x1eiAkEOeQ
✓ Inserted second record: ZtZzvpRdfPK6c61p4GVo5EKjtwdhKwxHRFF1cqx6iaC7aVvDkBQjdb605w2UEIQchDwW6muMZGC9xfRXBqihyA
✓ Upsert (update second record): ZtZzvpRdfPK6c61p4GVo5EKjtwdhKwxHRFF1cqx6iaC7aVvDkBQjdb605w2UEIQchDwW6muMZGC9xfRXBqihyA

=== Find One Operation ===
✓ Found user by email: {'id': 'VGmvuet4gkfu_YKwp9qsdwy1n2qI2LjQjQmxgjPQUJO5leppTG0hDP5r2zh2kYpkq6MQxz5FR7D6x1eiAkEOeQ', 'active': {'type': 'Boolean', 'value': True}, 'age': {'value': 29, 'type': 'Integer'}, 'name': {'value': 'Alice Johnson', 'type': 'String'}, 'email': {'type': 'String', 'value': 'alice.j@newdomain.com'}}
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
✓ Inserted document: Edbzx8vQHtnWxXZlnoQwjxw6YEyS0-tlZvVTBFRPzUwQDhdE4s_8i5ysM1vwo8Rf1bVJkUIOoF4vTnmuFYUI9Q

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: myL3fI6ikcANwGWBo7BeA4lYy2hDwKnIP6lLp2wgspytty1ybS1CSMVXaKWmKY5whlmc6P-_AoHNc1ahDPdDKw

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
✓ Edge cache script created: w-OqoF_52h-kSLe8bBNZiOi8jtnScdnwnKw0L-RGL7c5Agl9Usk1yxrlcuXB97Rs_tXZR92jZ9e2kVNtnzp9fQ

Call 1: Cache miss (fetches from API)
Response time: 157ms
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "current": {
            "interval": 900,
            "temperature_2m": 14.1,
            "time": "2026-11-08T16:45"
          },
          "current_units": {
            "interval": "seconds",
            "temperature_2m": "\u00b0C",
            "time": "iso8601"
          },
          "elevation": 32.0,
          "generationtime_ms": 0.02658367156982422,
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
Response time: 1ms (198.1x faster!)
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "current": {
            "interval": 900,
            "temperature_2m": 14.1,
            "time": "2026-11-08T16:45"
          },
          "current_units": {
            "interval": "seconds",
            "temperature_2m": "\u00b0C",
            "time": "iso8601"
          },
          "elevation": 32.0,
          "generationtime_ms": 0.02658367156982422,
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
   ⏱️  Duration: 68.6ms
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
   ⏱️  Duration: 0.7ms
   📊 Records: 1
   📦 Data: {
      "value": {
            "type": "Object",
            "value": {
                  "address": {
                        "city": "Gwenborough",
                        "geo": {
                 ...
   🚀 Cache speedup: 95.2x faster!

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
   Vector databases store embeddings—numeric representations of text, images, audio, and other data—and make it efficient to find items that are *similar in meaning or content*, even when they don’t share exact keywords.

Key benefits include:

- **Semantic search:** Finds relevant results by meaning, not just keyword matches.
- **Fast similarity search at scale:** Uses specialized indexes to search large collections of vectors efficiently.
- **Better AI applications:** Helps retrieval-augmented generation (RAG) systems find relevant context for language models.
- **Support for multiple data types:** Can power search across text, images, audio, and other content represented as vectors.
- **Filtering and hybrid search:** Many systems combine vector similarity with metadata filters or traditional keyword search.

They’re usually best used alongside a traditional database: vectors help find similar content, while relational or document databases remain useful for structured records and transactions.
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

✅ Function saved: SwGXuG_FRhl99Y8i-kX1qRZvQZXgW_6wHD24okoWtU5t9R7sIV4POlyiAPL3V3h_UXPohvmayomPKOF12xvdJA
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
   {'avg_score': {'type': 'Float', 'value': 20.0}, 'count': {'type': 'Integer', 'value': 3}, 'role': {'type': 'String', 'value': 'admin'}}
   {'avg_score': {'type': 'Float', 'value': 70.0}, 'count': {'type': 'Integer', 'value': 7}, 'role': {'type': 'String', 'value': 'user'}}
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

✅ Inserted order: qfWIEhqCaNmtqAYlgVzeXPUcQnP3UaeKzAt2Ow9GoEDs5Njv7Yw90x9Zg384NU8Qgig6ChmgJlNiIfWVOiR5hw
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

✅ Inserted order: oeQJMpXxsloaK5bewoa16WcRZXll1qVaNtIX6lFfVsRGG8w2_HYEMHbuzRJOw1zqmN9c4bHZ-XDbLAJIIc_B2g
✅ Cached order status
📊 Quick status lookup: {'value': '{"type":"Object","value":{"status":"processing","updated_at":"2026-11-08T16:52:08.584701+00:00"}}'}

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
   1. Introduction to Machine Learning (AI)
   2. Natural Language Processing (AI)
   3. Getting Started with ekoDB (Database)
   4. Database Design Principles (Database)
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
🚀 ekoDB Functions Example (Python)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved: rvX6NZLg5dKHqoDRuG3eburGh9YvMN5Cdw7fHJo3_CkZWgAViSwStb_kTorICnEaOvNIzpoKdyOwVfTEFk3E2w
📊 Found 5 active users

📝 Example 2: Parameterized Function

✅ Function saved: 9bdZUjxHsN9Rt4kNagzZnFsA_LhNLPcBy2QCVxthzNEQtmOPiIgJmJxjBTfGhZ9AYQ1ps2a5CZG-YDwu9GPsaA
📊 Found 3 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: JITdgoQIrsF30ONVESVcN0DNKbtBeckWSUbuz8GpQjOF6dYAH1H3tpKrhpv5EXHfEXKaMwrqClVaNKE4XbmgxA
📊 Statistics: 2 groups
   {'avg_score': {'type': 'Float', 'value': 60.0}, 'count': {'type': 'Integer', 'value': 5}, 'status': {'type': 'String', 'value': 'active'}}

   {'avg_score': {'type': 'Float', 'value': 50.0}, 'count': {'type': 'Integer', 'value': 5}, 'status': {'type': 'String', 'value': 'inactive'}}

📝 Example 4: Function Management

📋 Total functions: 3
🔍 Retrieved function: Get Active Users
✏️  Function updated
🗑️  Function deleted

ℹ️  Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.

✅ All examples completed!
=== ekoDB Goal Template CRUD Example (Python) ===

--- Creating goal template ---
Created template: Data Migration (id: tqyAr_mLTBT5tcVfY82p3_Ivl_uLJHkGerre7zhAIPeOxfH-X-IAYzai3lnn_dE4cduIme6yxd9gty_CeL6lZg)

--- Listing templates ---
Templates: {'count': 1, 'items': [{'description': {'type': 'String', 'value': 'Template for migrating data between schemas'}, 'id': 'tqyAr_mLTBT5tcVfY82p3_Ivl_uLJHkGerre7zhAIPeOxfH-X-IAYzai3lnn_dE4cduIme6yxd9gty_CeL6lZg', 'owner': {'type': 'Object', 'value': {'key_id': '3a0cdba0da7c060610042783cc758f1f', 'key_label': 'default_admin'}}, 'steps': {'type': 'Array', 'value': [{'description': 'Analyze source schema'}, {'description': 'Create target schema'}, {'description': 'Migrate records'}, {'description': 'Validate results'}]}, 'title': {'type': 'String', 'value': 'Data Migration'}}], 'total': 1}

--- Getting template ---
Fetched: {'type': 'String', 'value': 'Data Migration'}

--- Updating template ---
Updated description: {'type': 'String', 'value': 'Updated: comprehensive data migration workflow'}

--- Deleting template ---
Template deleted successfully

✓ Goal template CRUD example completed
=== ekoDB Goals / Tasks / Agents Integration Example (Python) ===

--- goal_create ---
Created goal: Deploy v3 (id: Oyj4iOD8ANUnHqd8Q7Opy0e06iplzJL9d8GAGm15KIgkToIBvpyyLeWWXn-JfbUzfJwOK-2cFslKaHHLsATLiw)

--- goal_list ---
Goals: {'count': 1, 'goals': [{'created_at': {'type': 'DateTime', 'value': '2026-11-08T16:52:09.201391+00:00'}, 'description': {'type': 'String', 'value': 'Deploy version 3 to production'}, 'id': 'Oyj4iOD8ANUnHqd8Q7Opy0e06iplzJL9d8GAGm15KIgkToIBvpyyLeWWXn-JfbUzfJwOK-2cFslKaHHLsATLiw', 'owner': {'type': 'Object', 'value': {'key_id': '3a0cdba0da7c060610042783cc758f1f', 'key_label': 'default_admin'}}, 'status': {'type': 'String', 'value': 'pending'}, 'steps': {'type': 'String', 'value': '[{"description":"Build Docker image","status":"pending"},{"description":"Run migrations","status":"pending"},{"description":"Smoke test","status":"pending"}]'}, 'title': {'type': 'String', 'value': 'Deploy v3'}, 'updated_at': {'type': 'DateTime', 'value': '2026-11-08T16:52:09.201391+00:00'}}], 'total': 1}

--- goal_get ---
Fetched: {'type': 'String', 'value': 'Deploy v3'}

--- goal_update ---
Updated description: {'type': 'String', 'value': 'Deploy version 3 to production (updated)'}

--- goal_search ---
Search results: {'count': 1, 'items': [{'_score': 13.2, 'created_at': {'type': 'DateTime', 'value': '2026-11-08T16:52:09.201391+00:00'}, 'description': {'type': 'String', 'value': 'Deploy version 3 to production (updated)'}, 'id': 'Oyj4iOD8ANUnHqd8Q7Opy0e06iplzJL9d8GAGm15KIgkToIBvpyyLeWWXn-JfbUzfJwOK-2cFslKaHHLsATLiw', 'owner': {'type': 'Object', 'value': {'key_id': '3a0cdba0da7c060610042783cc758f1f', 'key_label': 'default_admin'}}, 'status': {'type': 'String', 'value': 'pending'}, 'steps': {'type': 'String', 'value': '[{"description":"Build Docker image","status":"pending"},{"description":"Run migrations","status":"pending"},{"description":"Smoke test","status":"pending"}]'}, 'title': {'type': 'String', 'value': 'Deploy v3'}, 'updated_at': {'type': 'DateTime', 'value': '2026-11-08T16:52:09.205389+00:00'}}]}

--- goal_step_start (step 0) ---
Step 0 started: {'created_at': {'type': 'DateTime', 'value': '2026-11-08T16:52:09.201391+00:00'}, 'description': {'type': 'String', 'value': 'Deploy version 3 to production (updated)'}, 'id': 'Oyj4iOD8ANUnHqd8Q7Opy0e06iplzJL9d8GAGm15KIgkToIBvpyyLeWWXn-JfbUzfJwOK-2cFslKaHHLsATLiw', 'owner': {'type': 'Object', 'value': {'key_id': '3a0cdba0da7c060610042783cc758f1f', 'key_label': 'default_admin'}}, 'status': {'type': 'String', 'value': 'in_progress'}, 'steps': {'type': 'String', 'value': '[{"description":"Build Docker image","status":"InProgress"},{"description":"Run migrations","status":"pending"},{"description":"Smoke test","status":"pending"}]'}, 'title': {'type': 'String', 'value': 'Deploy v3'}, 'updated_at': {'type': 'DateTime', 'value': '2026-11-08T16:52:09.208900+00:00'}}

--- goal_step_complete (step 0) ---
Step 0 completed: {'created_at': {'type': 'DateTime', 'value': '2026-11-08T16:52:09.201391+00:00'}, 'description': {'type': 'String', 'value': 'Deploy version 3 to production (updated)'}, 'id': 'Oyj4iOD8ANUnHqd8Q7Opy0e06iplzJL9d8GAGm15KIgkToIBvpyyLeWWXn-JfbUzfJwOK-2cFslKaHHLsATLiw', 'owner': {'type': 'Object', 'value': {'key_id': '3a0cdba0da7c060610042783cc758f1f', 'key_label': 'default_admin'}}, 'status': {'type': 'String', 'value': 'in_progress'}, 'steps': {'type': 'String', 'value': '[{"description":"Build Docker image","result":"Docker image built: sha256:abc123","status":"Completed"},{"description":"Run migrations","status":"pending"},{"description":"Smoke test","status":"pending"}]'}, 'title': {'type': 'String', 'value': 'Deploy v3'}, 'updated_at': {'type': 'DateTime', 'value': '2026-11-08T16:52:09.213226+00:00'}}

--- goal_step_fail (step 1) ---
Step 1 failed: {'created_at': {'type': 'DateTime', 'value': '2026-11-08T16:52:09.201391+00:00'}, 'description': {'type': 'String', 'value': 'Deploy version 3 to production (updated)'}, 'id': 'Oyj4iOD8ANUnHqd8Q7Opy0e06iplzJL9d8GAGm15KIgkToIBvpyyLeWWXn-JfbUzfJwOK-2cFslKaHHLsATLiw', 'owner': {'type': 'Object', 'value': {'key_id': '3a0cdba0da7c060610042783cc758f1f', 'key_label': 'default_admin'}}, 'status': {'type': 'String', 'value': 'in_progress'}, 'steps': {'type': 'String', 'value': '[{"description":"Build Docker image","result":"Docker image built: sha256:abc123","status":"Completed"},{"description":"Run migrations","error":"Migration failed: column already exists","status":"Failed"},{"description":"Smoke test","status":"pending"}]'}, 'title': {'type': 'String', 'value': 'Deploy v3'}, 'updated_at': {'type': 'DateTime', 'value': '2026-11-08T16:52:09.217408+00:00'}}

--- goal_complete ---
Goal completed: {'type': 'String', 'value': 'pending_review'}

--- goal_approve ---
Goal approved: {'type': 'String', 'value': 'in_progress'}

--- goal_reject ---
Goal rejected: {'type': 'String', 'value': 'failed'}

--- task_create ---
Created task: Nightly Backup (id: CA8F6MpxlvgIm8m1LtaMyVM9lqmWTXlzIAAfh7fZtXS_lLJQHR8K3bNdndxBYfNuprumEpNvy2su1QKkt18ObQ)

--- task_list ---
Tasks: {'count': 1, 'items': [{'cron': {'type': 'String', 'value': '0 2 * * *'}, 'description': {'type': 'String', 'value': 'Backup all databases nightly'}, 'id': 'CA8F6MpxlvgIm8m1LtaMyVM9lqmWTXlzIAAfh7fZtXS_lLJQHR8K3bNdndxBYfNuprumEpNvy2su1QKkt18ObQ', 'max_consecutive_failures': {'type': 'Integer', 'value': 3}, 'name': {'type': 'String', 'value': 'Nightly Backup'}, 'owner': {'type': 'Object', 'value': {'key_id': '3a0cdba0da7c060610042783cc758f1f', 'key_label': 'default_admin'}}}], 'total': 1}

--- task_get ---
Fetched: {'type': 'String', 'value': 'Nightly Backup'}

--- task_start ---
Task started: {'type': 'String', 'value': 'running'}

--- task_succeed ---
Task succeeded: {'consecutive_failures': {'type': 'Integer', 'value': 0}, 'cron': {'type': 'String', 'value': '0 2 * * *'}, 'description': {'type': 'String', 'value': 'Backup all databases nightly'}, 'id': 'CA8F6MpxlvgIm8m1LtaMyVM9lqmWTXlzIAAfh7fZtXS_lLJQHR8K3bNdndxBYfNuprumEpNvy2su1QKkt18ObQ', 'last_error': {'type': 'Null', 'value': None}, 'last_run': {'type': 'DateTime', 'value': '2026-11-08T16:52:09.260646+00:00'}, 'max_consecutive_failures': {'type': 'Integer', 'value': 3}, 'name': {'type': 'String', 'value': 'Nightly Backup'}, 'owner': {'type': 'Object', 'value': {'key_id': '3a0cdba0da7c060610042783cc758f1f', 'key_label': 'default_admin'}}, 'run_count': {'type': 'Integer', 'value': 1}, 'status': {'type': 'String', 'value': 'active'}, 'updated_at': {'type': 'DateTime', 'value': '2026-11-08T16:52:09.260646+00:00'}}

--- task_pause ---
Task paused: {'type': 'String', 'value': 'paused'}

--- task_resume ---
Task resumed: {'type': 'String', 'value': 'active'}

--- task_fail ---
Task failed: {'consecutive_failures': {'type': 'Integer', 'value': 1}, 'cron': {'type': 'String', 'value': '0 2 * * *'}, 'description': {'type': 'String', 'value': 'Backup all databases nightly'}, 'id': 'CA8F6MpxlvgIm8m1LtaMyVM9lqmWTXlzIAAfh7fZtXS_lLJQHR8K3bNdndxBYfNuprumEpNvy2su1QKkt18ObQ', 'last_error': {'type': 'String', 'value': 'Disk full'}, 'last_run': {'type': 'DateTime', 'value': '2026-11-08T16:52:09.282687+00:00'}, 'max_consecutive_failures': {'type': 'Integer', 'value': 3}, 'name': {'type': 'String', 'value': 'Nightly Backup'}, 'next_run': {'type': 'DateTime', 'value': '2026-03-22T02:00:00+00:00'}, 'owner': {'type': 'Object', 'value': {'key_id': '3a0cdba0da7c060610042783cc758f1f', 'key_label': 'default_admin'}}, 'run_count': {'type': 'Integer', 'value': 1}, 'status': {'type': 'String', 'value': 'active'}, 'updated_at': {'type': 'DateTime', 'value': '2026-11-08T16:52:09.282687+00:00'}}

--- task_due ---
Due tasks: {'count': 0, 'items': []}

--- task_delete ---
Task deleted

--- agent_create ---
Created agent: CodeReviewer (id: QDMvjCRQU2qqfdW5SXu1hg_9HgcBEiNXmzZ7ChwQJUm8nQx3VESW6xruka____6XqsxcbRYvlV1K4izA2n2oWw)

--- agent_list ---
Agents: {'count': 1, 'items': [{'deployment_id': {'type': 'String', 'value': 'deploy_test'}, 'id': 'QDMvjCRQU2qqfdW5SXu1hg_9HgcBEiNXmzZ7ChwQJUm8nQx3VESW6xruka____6XqsxcbRYvlV1K4izA2n2oWw', 'llm_model': {'type': 'String', 'value': 'gpt-6-luna'}, 'name': {'type': 'String', 'value': 'CodeReviewer'}, 'owner': {'type': 'Object', 'value': {'key_id': '3a0cdba0da7c060610042783cc758f1f', 'key_label': 'default_admin'}}, 'run_policy': {'type': 'String', 'value': '{"trigger":{"allow_write_operations":false,"write_tools":[]},"eval":{"allow_write_operations":false,"write_tools":[]},"customer":{"allow_write_operations":false,"write_tools":[]}}'}, 'system_prompt': {'type': 'String', 'value': 'You review code for correctness and style.'}, 'tools': {'type': 'Array', 'value': ['web_search', 'file_read']}}], 'total': 1}

--- agent_get ---
Fetched: {'type': 'String', 'value': 'CodeReviewer'}

--- agent_get_by_name ---
By name: {'type': 'String', 'value': 'CodeReviewer'}

--- agent_update ---
Updated agent prompt: {'type': 'String', 'value': 'You review code. Be concise.'}

--- agents_by_deployment ---
Agents in deploy_test: {'count': 0, 'items': []}
WARNING: agents_by_deployment omitted created agent QDMvjCRQU2qqfdW5SXu1hg_9HgcBEiNXmzZ7ChwQJUm8nQx3VESW6xruka____6XqsxcbRYvlV1K4izA2n2oWw; TODO: check/fix the server-side deployment lookup

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
Inserted document: VWVSn-K7zvEbKwf-xmlRWCUotzD1Be2BTqZ1c0iXCqY_M5ErFDPktUnEB5FifiibIDiQt1wzXD9tZpS8MyZZZA

--- kv_link ---
Linked: None

--- kv_get_links ---
Links for app:config: [{'collection': 'kv_links_example_py', 'created_at': '2026-11-08T16:52:09.871995Z', 'document_id': 'VWVSn-K7zvEbKwf-xmlRWCUotzD1Be2BTqZ1c0iXCqY_M5ErFDPktUnEB5FifiibIDiQt1wzXD9tZpS8MyZZZA', 'field_path': None, 'last_accessed': '2026-11-08T16:52:09.872348Z', 'metadata': {}}]

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
Response: The traditional primary colors are red, yellow, and blue. For light and screens, they’re red, green, and blue (RGB).

--- Structured Output via SSE ---
JSON response: [
  {"name": "Earth", "diameter_km": 12742},
  {"name": "Mars", "diameter_km": 6779},
  {"name": "Jupiter", "diameter_km": 139820}
]

--- Blocking HTTP (for comparison) ---
Blocking response: Hello!

=== Done ===
=== ekoDB Schedule Management Example (Python) ===

--- create_schedule ---
Created schedule: Nightly Report (id: 4677e629-13e4-4134-82b4-a0cedec81bf6)

--- list_schedules ---
Schedules: {'count': 1, 'schedules': [{'created_at': '2026-11-08T16:52:14.514613Z', 'created_by': {'key_id': '3a0cdba0da7c060610042783cc758f1f', 'key_label': 'default_admin'}, 'cron_expression': '0 0 0 * * *', 'description': 'Generate and email nightly analytics report', 'enabled': True, 'function_label': 'schedule_noop_python_70851', 'id': '4677e629-13e4-4134-82b4-a0cedec81bf6', 'last_execution': None, 'name': 'Nightly Report', 'next_execution': '2026-11-09T00:00:00Z', 'parameters': {}, 'stats': {'avg_execution_time_ms': 0.0, 'failed_executions': 0, 'last_error': None, 'successful_executions': 0, 'total_executions': 0}, 'timezone': 'UTC', 'updated_at': '2026-11-08T16:52:14.514613Z'}]}

--- get_schedule ---
Fetched: Nightly Report cron=0 0 0 * * *

--- update_schedule ---
Updated cron: 0 30 1 * * *

--- trigger_schedule ---
Triggered: {'schedule_id': '4677e629-13e4-4134-82b4-a0cedec81bf6', 'status': 'triggered'}

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
  1. Score: 13.200, Matched: title, bio
  2. Score: 13.200, Matched: title, bio
  3. Score: 13.200, Matched: bio, title
  4. Score: 13.200, Matched: bio, title

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 26.400, Matched: bio, title
  2. Score: 26.400, Matched: title, bio
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
  1. Score: 0.750
  2. Score: 0.747
  3. Score: 0.726

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.375, Matched: content, title
  2. Score: 0.863, Matched: content, title
  3. Score: 0.373, Matched:

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 6.600, Matched: title

9. Vector search with a metadata pre-filter (category = ml):
Found 2 documents in category "ml" (NLP excluded)
  1. Introduction to Machine Learning (category: ml)
  2. Deep Learning Fundamentals (category: ml)

Paired vector search: document=Rust Programming Answer, query=Rust Programming Question
=== Cleanup ===
✅ Deleted test collections

✅ Search examples completed!
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {'id': '71D2VJXPu4o9zPW9E5fsuJnzdC73ENiC0HhgqgmnG8fxffnu4rMLvSv56HExStAy25rSS133_mDriwwcD8iz5g'}

=== Find by ID ===
Found: {'data': {'value': 'aGVsbG8gd29ybGQ=', 'type': 'String'}, 'tags': {'value': ['tag1', 'tag2', 'tag3'], 'type': 'Array'}, 'name': {'value': 'Test Record', 'type': 'String'}, 'value': {'type': 'Integer', 'value': 42}, 'metadata': {'type': 'Object', 'value': {'key': 'value', 'nested': {'deep': True}}}, 'user_id': {'value': '550e8400-e29b-41d4-a716-446655440000', 'type': 'String'}, 'created_at': {'type': 'String', 'value': '2026-11-08T11:52:15.057201'}, 'price': {'value': 99.99, 'type': 'Float'}, 'active': {'type': 'Boolean', 'value': True}, 'id': '71D2VJXPu4o9zPW9E5fsuJnzdC73ENiC0HhgqgmnG8fxffnu4rMLvSv56HExStAy25rSS133_mDriwwcD8iz5g', 'categories': {'value': ['electronics', 'computers'], 'type': 'Array'}, 'embedding': {'type': 'Array', 'value': [0.1, 0.2, 0.3, 0.4, 0.5]}}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): True
  price (Decimal): 99.99
  created_at (DateTime): 2026-11-08 11:52:15.057201
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): ['tag1', 'tag2', 'tag3']
  metadata (Object): {'key': 'value', 'nested': {'deep': True}}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): ['electronics', 'computers']
  data (Bytes): 11 bytes
Plain record: {'data': 'aGVsbG8gd29ybGQ=', 'tags': ['tag1', 'tag2', 'tag3'], 'name': 'Test Record', 'value': 42, 'metadata': {'key': 'value', 'nested': {'deep': True}}, 'user_id': '550e8400-e29b-41d4-a716-446655440000', 'created_at': '2026-11-08T11:52:15.057201', 'price': 99.99, 'active': True, 'id': '71D2VJXPu4o9zPW9E5fsuJnzdC73ENiC0HhgqgmnG8fxffnu4rMLvSv56HExStAy25rSS133_mDriwwcD8iz5g', 'categories': ['electronics', 'computers'], 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5]}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'price': {'type': 'Float', 'value': 99.99}, 'active': {'value': True, 'type': 'Boolean'}, 'user_id': {'type': 'String', 'value': '550e8400-e29b-41d4-a716-446655440000'}, 'data': {'value': 'aGVsbG8gd29ybGQ=', 'type': 'String'}, 'metadata': {'type': 'Object', 'value': {'key': 'value', 'nested': {'deep': True}}}, 'id': '71D2VJXPu4o9zPW9E5fsuJnzdC73ENiC0HhgqgmnG8fxffnu4rMLvSv56HExStAy25rSS133_mDriwwcD8iz5g', 'created_at': {'type': 'String', 'value': '2026-11-08T11:52:15.057201'}, 'categories': {'type': 'Array', 'value': ['electronics', 'computers']}, 'tags': {'type': 'Array', 'value': ['tag1', 'tag2', 'tag3']}, 'name': {'value': 'Updated Record', 'type': 'String'}, 'embedding': {'value': [0.1, 0.2, 0.3, 0.4, 0.5], 'type': 'Array'}, 'value': {'type': 'Integer', 'value': 100}}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: neQyneCRJYDN2VjWFQmZ7nZt1ABlPNVTiLW6M_OdYoLh2yQmjSDfLktsiYDdjsFzuUJvvPiOLwcGPwcGmf3alA

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
✓ Created native SWR script: github_user_native_py (i_sfk6wLDtBRPJV4qkhYaWsyz2UK3-H7R-Q_dw0BkZnrwbezOer1c-SO61S_eCmBDVW_5heXgVL6VFkbs2lN_A)

First call (cache miss - will fetch from GitHub API):
  Response time: 125ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 1ms
  Speedup: 130.2x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit_py (VnoMZ088Ea0IZqYZV67io7gkceSgyz5H_JVwFsPR-iotxq0O9bMX_-QrgNPqD4ueDd7tva6_ioB9Bwu5ldy4rQ)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail_py' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline_py (NGgSmZmk5ofi6aBlsTQWL9YjVagr0FZbU5NJhuh4eyw_gdGYARktTJwxk2_XkL2yIbj1-r7GjGB-sM1vdxZlGQ)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users_swr_py' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache_py (L0TS23vHPJCHiIGxpQ7oJqYK2d1j2yiAn9mf_ytS4JhXrbVTBV6jJgPqr31WTVp0UNAlqV5Xjhg4nu9vfCt-Lg)
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
✓ Created SWR script: fetch_api_user_py (rV2yQQObZrmF-w6aJu3m4DJl2epXb2eUZXOUwwNySiSX1ca9MNLStZxqFy7PeyoaswxMrB70sOXPslD1ZNWiUQ)

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
Response time: 1ms (served from cache)
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
Created Alice: $1000 - ID: ihNgMMgrXBz1t5_4p49L-vu0pkdPYFO-TKKWUFVRvTdmnOqkw2WzewqAkk22fIa_yKS2nzOIAUzIUBvhVxm-TQ
Created Bob: $500 - ID: 8HtFCRVhup_ObZ921dAVHRRUNh49R2XHm2ai6KRKPx5o2alHJWv01gi4ip0VDamXNpST5ndsXdSNU2BYgdVEvw

=== Example 1: Begin Transaction ===
Transaction ID (server-default isolation): 418bc902-a563-4931-aec7-cf5e78a8571c

=== Example 2: Operations within Transaction ===
Alice in transaction: {'id': 'ihNgMMgrXBz1t5_4p49L-vu0pkdPYFO-TKKWUFVRvTdmnOqkw2WzewqAkk22fIa_yKS2nzOIAUzIUBvhVxm-TQ', 'balance': {'value': 800, 'type': 'Integer'}, 'name': {'type': 'String', 'value': 'Alice'}, 'account_id': {'value': 'ACC001', 'type': 'String'}}
Bob in transaction: {'id': '8HtFCRVhup_ObZ921dAVHRRUNh49R2XHm2ai6KRKPx5o2alHJWv01gi4ip0VDamXNpST5ndsXdSNU2BYgdVEvw', 'balance': {'type': 'Integer', 'value': 700}, 'account_id': {'value': 'ACC002', 'type': 'String'}, 'name': {'type': 'String', 'value': 'Bob'}}
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

✓ Verified committed balances: Alice=$800, Bob=$700

=== Example 5: Rollback Demo ===
New transaction: 3269fccd-5b21-4a43-9c4c-cfc739023f08
Updated Bob: $700 → $600 (in transaction)
Status before rollback: Active
✓ Transaction rolled back

✓ Verified Bob remains $700 after rollback

=== Cleanup ===
✓ Deleted test account collection

✓ All client transaction examples completed
✓ Client created

=== Create User Function ===
Created user function with ID: xKd8whtwSTHLr7qLUYqAoQ9kzUbBk3FzrcSe_azwwQYzylt2dA_J0fFCQfd5h0BlNLZhDc4RJSrlBEDvR9om2Q

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

Created chat session: 8cAiwHDvss_xnMjUTdzMihaLFl5gKjU72wDW5AiEUVON2VYYI_JTTF_rikSb08AnkgprBzQtAsPJ08h6lpzHEQ

Sending message: 'What is the capital of France?'
{"__progress":"received"}{"__progress":"generating","model":"gpt-5.6-luna","provider":"openai"}The capital of France is **Paris**.

--- Stream ended ---
Message ID: G3ZcST0mNCuEANG94p-RIAVcJpbyMrYdMhkncraENxObRFmNmbWC38RR-Oghifpt5mEPZHuquIbdy46dNuYTSQ
Execution time: 678ms
Token usage: {'cached_input_tokens': 0, 'completion_tokens': 13, 'prompt_tokens': 23, 'thinking_tokens': 0, 'total_tokens': 36}

Full response: {"__progress":"received"}{"__progress":"generating","model":"gpt-5.6-luna","provider":"openai"}The capital of France is **Paris**....
=== WebSocket Subscription Example (Python) ===

✓ Authentication successful
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_py' ===
✓ Subscribed (subscription_id: sub_7a4d9ce268c64cd5b9659d7bb9c7d275)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: 3Jt6E-wg7mJa_2LIr_-MpWaevdHXnK3gps8nFkPJSyXXn3ZMfdD1huLNK8vOGF7PxKO3HkKHW5lkNswOSNFCMQ

  📡 Notification received:
     Event:      insert
     Collection: ws_subscribe_example_py
     Record IDs: 3Jt6E-wg7mJa_2LIr_-MpWaevdHXnK3gps8nFkPJSyXXn3ZMfdD1huLNK8vOGF7PxKO3HkKHW5lkNswOSNFCMQ
     Timestamp:  2026-11-08T16:52:17.631117+00:00

Inserting record 2...
✓ Inserted: CdoH6jw5WIHeRooSEladDEG3Rr-CX0J84bVy1SwVF-Q7qQ1SMO_Cn7ClzP0_JdGYJQYUcBFERCBNwgpmZjvukg

  📡 Notification received:
     Event:      insert
     Record IDs: CdoH6jw5WIHeRooSEladDEG3Rr-CX0J84bVy1SwVF-Q7qQ1SMO_Cn7ClzP0_JdGYJQYUcBFERCBNwgpmZjvukg

=== Unsubscribing ===
✓ Unsubscribed: {'collection': 'ws_subscribe_example_py', 'found': True, 'unsubscribed': True}

=== Cleanup ===
✓ Deleted collection 'ws_subscribe_example_py'

✓ WebSocket subscription example completed successfully
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: nSJ8IIUWOPA9ntogFHN613BLnwgpeX_wPDoz9yXy9Ph2DA1i6ErVdatjs5WuquUJIpQtdfkSC5m9KV1MnDMvYQ

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
   Inserted with ripple: {'id': 'IgvBAtAUSt7cM9KAuXrvjoWCoI6iO1j3tT_vsmGQCQ3QNMDrIjrfyMXQBDwDLxbgx0FppwuPRaO6R6YHekPB0g'}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {'id': 'Gqd3ROH6Zp__RJYnP_xIiGwGOQBMeQw2UM9WSjVnAU-jD8lrq-MsDuFPtJ5V2LQ86J7PyLWy8fkbUPzKj71eyw'}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {'price': {'value': 150, 'type': 'Integer'}, 'id': 'IgvBAtAUSt7cM9KAuXrvjoWCoI6iO1j3tT_vsmGQCQ3QNMDrIjrfyMXQBDwDLxbgx0FppwuPRaO6R6YHekPB0g', 'name': {'type': 'String', 'value': 'Product 1'}}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {'name': {'value': 'Upsert Product', 'type': 'String'}, 'price': {'value': 500, 'type': 'Integer'}, 'id': 'IgvBAtAUSt7cM9KAuXrvjoWCoI6iO1j3tT_vsmGQCQ3QNMDrIjrfyMXQBDwDLxbgx0FppwuPRaO6R6YHekPB0g'}

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
  Fields returned: ['user_role', 'created_at', 'bio', 'id', 'avatar_url', 'age', 'name', 'status', 'email']

Example 3: Complex query with projection (active users, ages 18-65)
  Found 3 active users (ages 18-65)
    - {'type': 'String', 'value': 'Dave Brown'} (age {'type': 'Integer', 'value': 45})
    - {'type': 'String', 'value': 'Alice Johnson'} (age {'type': 'Integer', 'value': 30})
    - {'type': 'String', 'value': 'Bob Smith'} (age {'value': 25, 'type': 'Integer'})

Example 4: Query inactive users with profile fields
  Found 1 inactive users
    - {'value': 'Carol White', 'type': 'String'}: {'type': 'String', 'value': 'Manager'}

Example 5: Compare full vs projected data
  Full query:
    - 12 fields per record
    - Fields: ['api_key', 'created_at', 'password', 'email', 'bio', 'user_role', 'id', 'name', 'age', 'avatar_url', 'status', 'secret_token']
  Projected query:
    - 3 fields per record
    - Fields: ['email', 'name', 'id']
  Bandwidth savings: ~75% fewer fields

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
