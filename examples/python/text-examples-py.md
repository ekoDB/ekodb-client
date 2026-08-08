make test-examples-python
📦 [36mEnsuring Python example dependencies in .venv...[0m
🧪 [36mRunning Python examples (direct HTTP/WebSocket)...[0m
=== Simple CRUD Operations (Direct HTTP) ===

✓ Authentication successful

=== Insert Document ===
Inserted: {'id': 'BgATvTajCI9n44J02Jew44u4HeOy6E2jjvRoKQkDpGlyGZ8WWbGW0pLzd12X-LCZt7A6bmcyChPNOc1u5IuBWg'}

=== Find by ID ===
Found: {'active': {'type': 'Boolean', 'value': True}, 'value': {'type': 'Integer', 'value': 42}, 'id': 'BgATvTajCI9n44J02Jew44u4HeOy6E2jjvRoKQkDpGlyGZ8WWbGW0pLzd12X-LCZt7A6bmcyChPNOc1u5IuBWg', 'name': {'value': 'Test Record', 'type': 'String'}}

=== Find with Query ===
Found documents: [{'active': {'value': True, 'type': 'Boolean'}, 'value': {'type': 'Integer', 'value': 42}, 'name': {'type': 'String', 'value': 'Test Record'}, 'id': 'BgATvTajCI9n44J02Jew44u4HeOy6E2jjvRoKQkDpGlyGZ8WWbGW0pLzd12X-LCZt7A6bmcyChPNOc1u5IuBWg'}]

=== Update Document ===
Updated: {'active': {'value': True, 'type': 'Boolean'}, 'value': {'type': 'Integer', 'value': 100}, 'name': {'value': 'Updated Record', 'type': 'String'}, 'id': 'BgATvTajCI9n44J02Jew44u4HeOy6E2jjvRoKQkDpGlyGZ8WWbGW0pLzd12X-LCZt7A6bmcyChPNOc1u5IuBWg'}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: atjnQ2_MbhcPgzST8zEGu9Jf2fHreO5rAF3g7PXqpGDatOl5ntdvSZjqLjtL45S73QikEaPqSgzPv4YED1uK8Q

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "type": "Success",
  "payload": {
    "data": [
      {
        "name": {
          "type": "String",
          "value": "WebSocket Test Record"
        },
        "active": {
          "type": "Boolean",
          "value": true
        },
        "value": {
          "type": "Integer",
          "value": 42
        },
        "id": "YzqB7vN0r4D-3zhsgSJ8CEDDI_9gRK0zRHJnV7qaDbIYbCjfXHmDa2JMO7MiFFVbRGFPw8PepqVJ69RV3PvSHQ"
      },
      {
        "active": {
          "value": true,
          "type": "Boolean"
        },
        "id": "atjnQ2_MbhcPgzST8zEGu9Jf2fHreO5rAF3g7PXqpGDatOl5ntdvSZjqLjtL45S73QikEaPqSgzPv4YED1uK8Q",
        "name": {
          "value": "WebSocket Test Record",
          "type": "String"
        },
        "value": {
          "type": "Integer",
          "value": 42
        }
      }
    ]
  },
  "messageId": "3286157336"
}
✓ Retrieved 2 record(s) via WebSocket

✓ WebSocket example completed successfully
🚀 ekoDB Functions Example (Python/HTTP)

✓ Authentication successful

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Function saved: vZ1t0XtsYrQii-dLoUm71gEXFWbJgy6INyAENh2KJyWTqzsqp2Edw0vUXljaDlb_jBQYa2gYGX8g5oGRH3BHsQ
📊 Found 15 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

ℹ️  Function 'get_active_users_paginated' already existed — updated instead
✅ Function saved: get_active_users_paginated
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: Y7h4q0M6_aH_TYW5Z5Y4KhKoDJHM6ys-iey5jZtdehmbTNO2GEH1sa4_auclqhOxlcoEXlREmQQB_GPk8alubw
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {'max_score': {'type': 'Integer', 'value': 90}, 'avg_score': {'type': 'Float', 'value': 50.0}, 'status': {'value': 'inactive', 'type': 'String'}, 'count': {'value': 15, 'type': 'Integer'}}
   {'avg_score': {'value': 60.0, 'type': 'Float'}, 'status': {'type': 'String', 'value': 'active'}, 'max_score': {'type': 'Integer', 'value': 100}, 'count': {'type': 'Integer', 'value': 15}}

📝 Example 4: Function Management

📋 Total scripts: 13
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
Retrieved value: {'value': {'username': 'john_doe', 'userId': 123}, 'type': 'Object'}

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: {'type': 'Object', 'value': {'price': 29.99, 'name': 'Product 1'}}
cache:product:2: {'value': {'name': 'Product 2', 'price': 39.989999999999995}, 'type': 'Object'}
cache:product:3: {'type': 'Object', 'value': {'price': 49.989999999999995, 'name': 'Product 3'}}

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
=== Collection Management (Direct HTTP) ===

✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: zKV0nWMZX9tF9-8_WOpuYdS1nQ_YTONvBj-5R1oYh0o53o2RMQoWeEPK8FyxkB6SGiO2bBiqx9R5KH2blJKPcg

=== List Collections ===
Total collections: 14
Sample collections: ['audit__ek0_testing', 'products', 'functions__ek0_testing', 'users', 'test_collection']

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
Created Alice: $1000 - ID: WHqV6jrTvTr2S-cPcZRNIWkgh1hJLmfKIrsbxvuoOAg0ujBI4yFHQpBmkUA-b7KuN091hy0pzsboEyNX9n25Zw
Created Bob: $500 - ID: GSgAEeJrj6I1pTCFIhmlJJYsaK_4EeKB8ksqbL-a12qQXf1oxGQCQTOqUGv67x87ndHj0cM4lXhkXRYDW_jpuA

=== Example 1: Begin Transaction ===
Transaction ID: 22f58be8-850c-450a-b6f8-b752a98bfcf7

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
New transaction: 1142e0b0-e72d-4a01-a457-6e8f0b93637f
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'value': 700, 'type': 'Integer'}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: jKdpzq6cdmoLik10IGcPe9dpBBpHI8BXwGUu8ebVFQ23VCWf_hB228dJ5FLa2kAfcfvKbd6LzqnHTvPBCTAW1w
Created Bob: $500 - ID: E1HlVslAW22BsanPrj2iqUV86vGu2oLuqW9kX2zeSGwxyPKk_JX0uyK9hj-JY8P37K3oNWekAfOD2y27PyQkCQ

=== Example 1: Begin Transaction ===
Transaction ID: a4f7b0d3-ac20-4cb0-833e-51fcf9ada5dc

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
Created Alice: $1000 - ID: pAh7XtT6EpdjOwJZFMBRIpAM95SfCrkw5WZSpqtf5Tu7yNvLrHnXR8pN9NolytZ4XJARzKdds4bJbo01TK5kGg
Created Bob: $500 - ID: HsT6HPh1kjBJD6NjhpuvXHtJYggbpxfFNhj-K2No4YT_Qa_yb0rMEXafqWqPQpmszze1KfBfo2w3Ka6ptO_Tyg

=== Example 1: Begin Transaction ===
Transaction ID: 46e89960-5728-4c1d-8b58-035f03385e6f

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
New transaction: b6e1adbf-49a7-4625-9177-f9f55baf572d
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'type': 'Integer', 'value': 700}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: COZwkgSM6s3MCxwQw06lUaDVLOeQxiP1Vf9yBjcApRp-sXfjnkYjPe9j665euN3_dHfUpittu8cWHZBMUHd84Q
Created Bob: $500 - ID: 6hOfSTfC6i5Q0Y-oe6vr6Qo3sW2N42BW7ox_NLAuWFLZgCdJafXK4KkkueKhtc8jxrgBeS8wwQXd6DWmQ9tRfw

=== Example 1: Begin Transaction ===
Transaction ID: 2b52fb50-1c15-454e-acc3-b2bed2b3f91e

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
Created Alice: $1000 - ID: e3ggMqCVp2Q2ZlQSO92N567udVD7LlHu4bgmlmQ78QSuUA_TaC8SUtfRULcX8t-UksucBbALeq1M63VXaBmCvg
Created Bob: $500 - ID: Jre7pGmKcS2_4PG0tsJfPX775nZlaoXOEzlrD-Tk0BsXgxGwAGmj_FJDR8ZQGTjKnK19V5WcOdnv0QJiuqsrKw

=== Example 1: Begin Transaction ===
Transaction ID: 29c58519-d6a6-4cae-804c-894c66b28fed

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
New transaction: 39d0d831-4489-43b9-9c84-e61e5397da1d
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'value': 700, 'type': 'Integer'}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 5XVmncjvkwlHAPLf9SYj5YizsJnflFcop2zldMPaneBXkUx3lI96cbY8gDUi1-PuO1wDhP3-QqiSWhOLRfh-Sg
Created Bob: $500 - ID: nKlZYM9YbPbzHLCk2ci8kOZlLivnrcf8BdaTYoU2__9OUb_9uOfQDBDQtIUDBoUAMWV8DEI4UrwHeAMu-ke0cw

=== Example 1: Begin Transaction ===
Transaction ID: bf3b624f-607c-4ca5-94ff-06c93f152dac

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
   ✅ Function saved: zpWg9Fu86uAACrTML3uoSK5rooSHVS_PGTh0_x-gA0TCfzaM0lNx589gl0Ez9ro7NcoDUp55vStH65dR9j9pxQ

2️⃣ Calling function (Insert + Verify)...
   ✅ function executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: TCXSdrd2rYjFLe6sBjYotLA21YT6YLi5ymfp5DBW5BiqjF3rijtU7-A8owsmpw9YrfpAp5LRk1BQAft3ZDR0TA
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}
   📋 Email: {'value': 'alice@example.com', 'type': 'String'}
   📋 Status: {'type': 'String', 'value': 'pending'}
   📋 Credits: {'type': 'Integer', 'value': 0}

============================================================
📝 function 2: Query + Update + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: wVVhabhjqRqFvHj9-DXGv2Blbu3jwO5sN3PIZb-LGD-spL28EeWZkNjA6ftDzhesYt6Nth8tU8TSFe1OKq399A

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
   ✅ Function saved: b5opJwpKGMGPl5ke3qgIOIkNKwclp_ayOJLZKqilpGHeHIUvINFVTe-JqDgqHCy8e_k_gbPUWEbXx9z6PY1sIw

2️⃣ Calling function (Query + Update Credits + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {'type': 'Integer', 'value': 0}
   📋 Status: {'type': 'String', 'value': 'active'}
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}

============================================================
📝 function 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: 9aCoGKDh5Oet_2MvVlZ38xVcsRXHgFzfgK2EiKYhmvA-qNqVJ1FEYRGYoD9lrHMuZRZYNUQahVAFd10PgrElqQ

2️⃣ Calling function (Query + Delete + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: zpWg9Fu86uAACrTML3uo...
   ✅ Deleted script: wVVhabhjqRqFvHj9-DXG...
   ✅ Deleted script: b5opJwpKGMGPl5ke3qgI...
   ✅ Deleted script: 9aCoGKDh5Oet_2MvVlZ3...
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
  Output: Document ID = 50lSrQxYT28i93Ogj3pW0oUrilqsGtFDO6rKRkcG72B7L-9c48a6ybng-j7662i0cWBZkJ9n7YkuQLrfV9_8_Q
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: find_by_id(50lSrQxYT28i93Ogj3pW0oUrilqsGtFDO6rKRkcG72B7L-9c48a6ybng-j7662i0cWBZkJ9n7YkuQLrfV9_8_Q)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: find_by_id(50lSrQxYT28i93Ogj3pW0oUrilqsGtFDO6rKRkcG72B7L-9c48a6ybng-j7662i0cWBZkJ9n7YkuQLrfV9_8_Q)
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
  Output: Document ID = wDmU9zF8_HBnTcEqRc9Q3WCLjoEa9zwZ6eE67rSVVF6KoaL8n7jKveenMalS5xymqlxqve5NIcgL7O4Sp2LvcA
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: find_by_id(wDmU9zF8_HBnTcEqRc9Q3WCLjoEa9zwZ6eE67rSVVF6KoaL8n7jKveenMalS5xymqlxqve5NIcgL7O4Sp2LvcA)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: find_by_id(wDmU9zF8_HBnTcEqRc9Q3WCLjoEa9zwZ6eE67rSVVF6KoaL8n7jKveenMalS5xymqlxqve5NIcgL7O4Sp2LvcA)
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
🔧 [36mEnsuring maturin is available in .venv...[0m
🔨 [36mBuilding wheel...[0m
🍹 Building a mixed python/rust project
🐍 Found CPython 3.11 at /Library/Frameworks/Python.framework/Versions/3.11/bin/python3
🔗 Found pyo3 bindings with abi3-py3.8 support
💻 Using `MACOSX_DEPLOYMENT_TARGET=11.0` for aarch64-apple-darwin by default
    Finished `release` profile [optimized] target(s) in 0.17s
📦 Built wheel for abi3 Python ≥ 3.8 to ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.25.0-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel into .venv...[0m
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.25.0-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.25.0
    Uninstalling ekodb_client-0.25.0:
      Successfully uninstalled ekodb_client-0.25.0
Successfully installed ekodb-client-0.25.0
🧪 [36mEnsuring test dependencies (pytest) in .venv...[0m
✅ [32mPython client package built and installed![0m
🧪 [36mRunning Python client library examples...[0m
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {'id': 'lFDzVJC2BbYhAyQr0nG5rjtZgtvV2dvP7HM7fqr4eD_FcryLwKmE-yXqog8UKRrvdJxGoElC4orZHMyoQKqa8g'}

=== Find by ID ===
Found: {'created_at': {'type': 'String', 'value': '2026-07-14T09:47:39.528737'}, 'embedding': {'type': 'Array', 'value': [0.1, 0.2, 0.3, 0.4, 0.5]}, 'user_id': {'value': '550e8400-e29b-41d4-a716-446655440000', 'type': 'String'}, 'data': {'type': 'String', 'value': 'aGVsbG8gd29ybGQ='}, 'categories': {'type': 'Array', 'value': ['electronics', 'computers']}, 'value': {'value': 42, 'type': 'Integer'}, 'name': {'type': 'String', 'value': 'Test Record'}, 'id': 'lFDzVJC2BbYhAyQr0nG5rjtZgtvV2dvP7HM7fqr4eD_FcryLwKmE-yXqog8UKRrvdJxGoElC4orZHMyoQKqa8g', 'metadata': {'value': {'key': 'value', 'nested': {'deep': True}}, 'type': 'Object'}, 'active': {'value': True, 'type': 'Boolean'}, 'price': {'value': 99.99, 'type': 'Float'}, 'tags': {'type': 'Array', 'value': ['tag1', 'tag2', 'tag3']}}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): True
  price (Decimal): 99.99
  created_at (DateTime): 2026-07-14 09:47:39.528737
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): ['tag1', 'tag2', 'tag3']
  metadata (Object): {'key': 'value', 'nested': {'deep': True}}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): ['electronics', 'computers']
  data (Bytes): 11 bytes
Plain record: {'created_at': '2026-07-14T09:47:39.528737', 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5], 'user_id': '550e8400-e29b-41d4-a716-446655440000', 'data': 'aGVsbG8gd29ybGQ=', 'categories': ['electronics', 'computers'], 'value': 42, 'name': 'Test Record', 'id': 'lFDzVJC2BbYhAyQr0nG5rjtZgtvV2dvP7HM7fqr4eD_FcryLwKmE-yXqog8UKRrvdJxGoElC4orZHMyoQKqa8g', 'metadata': {'key': 'value', 'nested': {'deep': True}}, 'active': True, 'price': 99.99, 'tags': ['tag1', 'tag2', 'tag3']}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'embedding': {'value': [0.1, 0.2, 0.3, 0.4, 0.5], 'type': 'Array'}, 'active': {'type': 'Boolean', 'value': True}, 'price': {'type': 'Float', 'value': 99.99}, 'metadata': {'type': 'Object', 'value': {'key': 'value', 'nested': {'deep': True}}}, 'categories': {'type': 'Array', 'value': ['electronics', 'computers']}, 'created_at': {'type': 'String', 'value': '2026-07-14T09:47:39.528737'}, 'data': {'value': 'aGVsbG8gd29ybGQ=', 'type': 'String'}, 'id': 'lFDzVJC2BbYhAyQr0nG5rjtZgtvV2dvP7HM7fqr4eD_FcryLwKmE-yXqog8UKRrvdJxGoElC4orZHMyoQKqa8g', 'tags': {'type': 'Array', 'value': ['tag1', 'tag2', 'tag3']}, 'user_id': {'type': 'String', 'value': '550e8400-e29b-41d4-a716-446655440000'}, 'name': {'value': 'Updated Record', 'type': 'String'}, 'value': {'type': 'Integer', 'value': 100}}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: BWpMGV838OJl9Pdz88wuHh6cLFVdhziJidoKK02hpa6T4YbrjMn78tC-e3MRrhQJMghIuPQx8-nQwSg0n2LIVA

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
Collection created with first record: "d1XNmJyH6LPaca3sFhsQg23iPKrQo2KfQT8D0WotLZnKhibQ2PLx2kbZDochZzrWtdtYRtUecd4CTOoe2LvoUA"

=== List Collections ===
Total collections: 13
Sample collections: ['client_collection_management_python', 'audit__ek0_testing', 'products', 'functions__ek0_testing', 'test_collection']

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
Retrieved value: {'value': '{"type":"Object","value":{"userId":123,"username":"john_doe"}}'}

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
Created Alice: $1000 - ID: VrLjWjMC2_56QZDIPkgpOvyEDXUGdbS4zGxIIj41Qg6ApOxWQYghBrmU9A88g5BVAWOPdPmIk4dUZTw-TvGNug
Created Bob: $500 - ID: H0Vqgcp0YlKLNVaArmDfPHeluSWJSC04sFv9nN97YjOadR2Z7GKxw3xG5sz-8AEOslDvltXOxP_-9vI_BNidHA

=== Example 1: Begin Transaction ===
Transaction ID: aded103f-3fa9-4b8d-b576-a2e2cbef0103

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 6605e671-e27d-4e13-8030-5c36b2c03fbe
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
  1. Score: 12.870, Matched: name, email
  2. Score: 6.270, Matched: name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 13.200, Matched: title, bio
  2. Score: 13.200, Matched: bio, title
  3. Score: 13.200, Matched: title, bio
  4. Score: 13.200, Matched: title, bio

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 26.400, Matched: title, bio
  2. Score: 26.400, Matched: title, bio
  3. Score: 26.400, Matched: title, bio
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
  1. Score: 0.764
  2. Score: 0.747
  3. Score: 0.740

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.373, Matched: content, title
  2. Score: 0.882, Matched: title, content
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
✓ Inserted document: ZJvQIWmDvdhseB_mE6dNPblIg5AxE3iELX5wY8OoYAz-pbBZR8ktAGFIyu0diNdp1jI5z1nlygdonfCz1j078A

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: uD_xxWZ1kd4jQ0_jKIzZwu2UF-KmR_nL9D5VxuAM7NMzGsX1ihg9AOe0KvC08WfHQ8UKep16AmVfZQlAX7LabA

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
✓ Inserted document with TTL: _SZ492cQl6T5OApEqkt4tYxeU2-cVjbaeHA3iSIVbWthmYidQqMIRvfHQjned0Ur6bLtUou2K5bZsAF6q52_6A

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
✓ Edge cache script created: 7KU-4Wm0YBMA320AZJJxPDNI26x3aMD8CL9_cbyMWA9ObIEikKTJCFoaoexFXkvUrKSKXL9zAOyJITGaGTB8Fw

Call 1: Cache miss (fetches from API)
Response time: 436ms
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "current": {
            "interval": 900,
            "temperature_2m": 27.0,
            "time": "2026-07-14T13:45"
          },
          "current_units": {
            "interval": "seconds",
            "temperature_2m": "\u00b0C",
            "time": "iso8601"
          },
          "elevation": 32.0,
          "generationtime_ms": 0.01919269561767578,
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
Response time: 9ms (49.1x faster!)
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "current": {
            "interval": 900,
            "temperature_2m": 27.0,
            "time": "2026-07-14T13:45"
          },
          "current_units": {
            "interval": "seconds",
            "temperature_2m": "\u00b0C",
            "time": "iso8601"
          },
          "elevation": 32.0,
          "generationtime_ms": 0.01919269561767578,
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

🚀 ekoDB Functions Example (Python)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

ℹ️  Function 'get_active_users' already existed — updated instead
✅ Function saved: get_active_users
📊 Found 10 active users

📝 Example 2: Parameterized Function

ℹ️  Function 'get_users_by_status' already existed — updated instead
✅ Function saved: get_users_by_status
📊 Found 10 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: WhamULYS9FKMpu2Kc4NIjETGT7MTYL7ojXar7bfyPzINTW7nT6kDqqBY8J0OdzVITXlH7pLwrm6fYoqcbD6TGQ
📊 Statistics: 2 groups
   {'avg_score': {'type': 'Float', 'value': 60.0}, 'count': {'type': 'Integer', 'value': 5}, 'status': {'type': 'String', 'value': 'active'}}

   {'avg_score': {'type': 'Float', 'value': 50.0}, 'count': {'type': 'Integer', 'value': 5}, 'status': {'type': 'String', 'value': 'inactive'}}

📝 Example 4: Function Management

📋 Total functions: 14
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

ℹ️  Function 'fetch_user' already existed — updated instead
✅ Saved reusable function: fetch_user
ℹ️  Function 'get_user_wrapper' already existed — updated instead
✅ Saved composed function: get_user_wrapper (calls fetch_user + projects fields)

📊 Result from composed function:
   Records: 1
   Name: {"type": "String", "value": "User 1"}
   Department: {"type": "String", "value": "engineering"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

ℹ️  Function 'fetch_and_store_user' already existed — updated instead
✅ Saved reusable function: fetch_and_store_user (uses KV)
ℹ️  Function 'swr_user' already existed — updated instead
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 1.7ms
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
   ⏱️  Duration: 1.7ms
   📊 Records: 1
   📦 Data: {
      "value": {
            "type": "Object",
            "value": {
                  "address": {
                        "city": "Gwenborough",
                        "geo": {
                 ...
   🚀 Cache speedup: 1.0x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

ℹ️  Function 'validate_user' already existed — updated instead
✅ Level 1 function: validate_user
ℹ️  Function 'fetch_slim_user' already existed — updated instead
✅ Level 2 function: fetch_slim_user (calls validate_user)
ℹ️  Function 'get_verified_user' already existed — updated instead
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

✅ Function saved: MM4Tnp7ZeP2BuXJJBZTQg95n_ujDo0qGQXfchlteGbRHLXJWuGoDX4bCGX1a0GidCkst8GXdm_PYQ8DF6YTbNg
📊 Found 2 product groups
   {'avg_price': {'type': 'Float', 'value': 474.0}, 'category': {'type': 'String', 'value': 'Furniture'}, 'count': {'type': 'Integer', 'value': 2}}
   {'avg_price': {'type': 'Float', 'value': 575.6666666666666}, 'category': {'type': 'String', 'value': 'Electronics'}, 'count': {'type': 'Integer', 'value': 3}}
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

✅ Inserted order: qKNpn1y8bXsFw0CKwNThoAZKIF3SvkhyT3-SI1bM42rJgkYV3YZ1MNZthI53GlPcKFznqhfSjlZRdcMr3R3DeA
✅ Inserted 2 products with wrapped types

📝 Example 2: Querying and Extracting Wrapped Types

📊 Found 2 products
   • Laptop Pro
   • Wireless Mouse

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

✅ Inserted order: Mp0IR_qCKHilj7yJAWKoGgO5co22RdKR4IsTUQqfLWF6tmTAVe8SpV8WpJmsUB6u75IT2YVDovri19Um1hnhAQ
✅ Cached order status
📊 Quick status lookup: {'value': '{"type":"Object","value":{"status":"processing","updated_at":"2026-07-14T13:47:42.421430+00:00"}}'}

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
✓ Created SWR script: fetch_api_user_py (_ydVpuF3f5ciaxK6aBfD6y_x0GeawvVGNv-i01L4cCPwDW48G7M8TA5ca8R9FNZNR-Aq4TfJPbUae5kb535QOQ)

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
✓ Created native SWR script: github_user_native (xgfwui32FIcQxTTbaUE7iRx8R-9mW_vzUvj83N6MREGzwGHmCwS41wfInxCeNWVgGT79bhegWCkCHqgmUvRIhA)

First call (cache miss - will fetch from GitHub API):
  Response time: 121ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 2ms
  Speedup: 55.6x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (BvdxrBKWsTO3BUucz6XWYcOzkkCb0l3NOmhE43tqteEa3z7UkXlDob6OdCk3f8L7naH0eDrP3e2kB2rTpnxlHw)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (BFFN8k-nae7TAQ46--WbZVNIwahadtUo9s0a50lv0zXJgWOYmYRoI3MbB53CZz-GwARusgGeYd5XKeE7JxKDhw)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (oml11fIcOpycu02cIEPS2n8nVv8UOn_UTCgM4k-C_DL12bvdaqbvSAoRTVfXD_dBPKfxoo-VNbvGQcUkoD6s9g)
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
   Vector databases offer several benefits:

1. **Efficient Similarity Search**: They facilitate fast similarity searches for high-dimensional data, making them ideal for applications like image recognition and natural language processing.

2. **Scalability**: Vector databases can handle large datasets and scale efficiently, accommodating growing amounts of data without significant performance degradation.

3. **Complex Querying**: They support advanced querying capabilities, allowing users to retrieve data based on intricate patterns and relationships.

4. **Integration with ML Models**: Vector databases work effectively with machine learning models and can store embeddings from various models for easy retrieval.

5. **Real-time Data Processing**: They are optimized for real-time data analytics, enabling immediate insights from data streams.

6. **Reduced Latency**: Vector databases typically achieve lower query latency compared to traditional databases, enhancing user experience.

7. **Multimodal Data Support**: They can manage diverse data types (text, images, etc.) in a unified manner, facilitating cross-modal data comparisons. 

8. **Enhanced Data Retrieval**: They leverage vector representations to improve the relevancy of returned results in response to user queries.

These benefits make vector databases particularly advantageous for AI-driven applications and big data use cases.
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
   {'avg_score': {'type': 'Float', 'value': 20.0}, 'count': {'type': 'Integer', 'value': 3}, 'role': {'type': 'String', 'value': 'admin'}}
   {'avg_score': {'type': 'Float', 'value': 70.0}, 'count': {'type': 'Integer', 'value': 7}, 'role': {'type': 'String', 'value': 'user'}}
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
   1. Introduction to Machine Learning (AI)
   2. Getting Started with ekoDB (Database)
   3. Database Design Principles (Database)
   4. Vector Databases Explained (Database)
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
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: r_wc02Y4Gj4Oe95sscpaoNiwXFY2xq5ICG5_mlCQyko4WnJsHw8mO5PhlM5zAuyeabpTT2-ZvI46ge7ETicnbQ

=== Sending Chat Message ===
Message ID: 6gMhyNhdjMIb-v34DkrMT3maCRb1quJ8dN6_UBUYMSUFyTEvWORTR_gYUQFoxuR6CEGThF8iODTICdp75JEr3g

=== AI Response ===
Here are the products available along with their prices:

1. **ekoDB Cloud**: $499
   - Description: Fully managed cloud database service product.

2. **ekoDB Pro**: $299
   - Description: Enterprise edition product with advanced features.

3. **ekoDB**: $99
   - Description: A high-performance database product with AI capabilities.

Execution Time: 6039ms

=== Token Usage ===
Prompt tokens: 3413
Completion tokens: 83
Total tokens: 3496

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: R7OAhi9SgAiKM4yxIZqUZ2n59tGKIhqDdOQwAaaPFNoOFVk719RoUd2ko3ZSX0avBV9hgbrCdGq7El-1KXn9vg

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: High-performance database product
- **Price**: $99

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['chat_id', 'context_snippets', 'updated_at', 'id', 'llm_model', 'created_at', 'role', 'token_usage', 'llm_provider', 'content'])
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
✓ Created second session: 6_YNwhItnPEY95NBWHOKtlC0SifEp_7tEs0ifMM9MU75wlOF12GsqvLKNp7d5jQh_QE9el7fET13yIaX301_4A
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 7

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 6

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: r7WPdqwlcHC_G8bUVExMEIQxl49agy-fuwLtSQFNbE_xvN04r96fw4vSL_t44KICXCfkJ1if-ziB7sa59TQDNQ

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: A high-performance database product
- **Price**: $99

If you need more information or have other questions, feel free to ask!

✓ Message 2 sent
  Response: The price of the ekoDB product is $99. If you have any other questions or need further information, feel free to ask!

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: mx_NWgnIA4hJLFPDy-PeuEPOmrZLOj08SaeooMqZgVrKhjoVsYAcon0VM--ssKzs3HowM8q2UuE4Likw2dhEmQ
  Parent: r7WPdqwlcHC_G8bUVExMEIQxl49agy-fuwLtSQFNbE_xvN04r96fw4vSL_t44KICXCfkJ1if-ziB7sa59TQDNQ

=== Listing Sessions ===
✓ Found 7 sessions
  Session 1: mx_NWgnIA4hJLFPDy-PeuEPOmrZLOj08SaeooMqZgVrKhjoVsYAcon0VM--ssKzs3HowM8q2UuE4Likw2dhEmQ (Untitled)
  Session 2: r7WPdqwlcHC_G8bUVExMEIQxl49agy-fuwLtSQFNbE_xvN04r96fw4vSL_t44KICXCfkJ1if-ziB7sa59TQDNQ (Untitled)
  Session 3: 6_YNwhItnPEY95NBWHOKtlC0SifEp_7tEs0ifMM9MU75wlOF12GsqvLKNp7d5jQh_QE9el7fET13yIaX301_4A (Untitled)
  Session 4: r_wc02Y4Gj4Oe95sscpaoNiwXFY2xq5ICG5_mlCQyko4WnJsHw8mO5PhlM5zAuyeabpTT2-ZvI46ge7ETicnbQ (Untitled)
  Session 5: lz_AbI9DujI6oJ2KcgwO3Kz7MU-2mDEK20c0uMa_BZgeqvjMxtVGhGkWlfgRN30ecMd8SZ2Q1f61D05626W-mA (Untitled)
  Session 6: 5XWDr5Je240IcJaiceVUokJtYGILqDmFRj1JzwUZ7wBcK6AeJfVfM1f8XzRsKua-1h0Ce7W-49WlHtk9TUv1Qw (Untitled)
  Session 7: 4cAFSBdJJg7EuCQq2Ao6n32foY6PGCN4T2aMFWJYwyex8-q_GdJf3sNqxjJkVUQeU7a3f_cFD2cPPseckCeFmA (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: mx_NWgnIA4hJLFPDy-PeuEPOmrZLOj08SaeooMqZgVrKhjoVsYAcon0VM--ssKzs3HowM8q2UuE4Likw2dhEmQ

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Dict Creation ===
✓ Created record with plain dict: {'id': 'Pi7N_RALxVRwqDYjd7mrOyxrDBgy9KpXGjo5M3bwnCA298k_dDYhUtsvNjSqyGSDic4E0dgx3FQPgZ5K0LZE_w'}

=== Upsert Operation ===
✓ Upsert (update existing record): Pi7N_RALxVRwqDYjd7mrOyxrDBgy9KpXGjo5M3bwnCA298k_dDYhUtsvNjSqyGSDic4E0dgx3FQPgZ5K0LZE_w
✓ Inserted second record: 7XSTejhJoHjkQ1IehaYQjamw2AHLqr8QXO2Wvh7xMKa_W5UhY9THPpe52uZGCeCxjqnSGboXUUaMlrdxTJA-ng
✓ Upsert (update second record): 7XSTejhJoHjkQ1IehaYQjamw2AHLqr8QXO2Wvh7xMKa_W5UhY9THPpe52uZGCeCxjqnSGboXUUaMlrdxTJA-ng

=== Find One Operation ===
✓ Found user by email: {'id': 'Pi7N_RALxVRwqDYjd7mrOyxrDBgy9KpXGjo5M3bwnCA298k_dDYhUtsvNjSqyGSDic4E0dgx3FQPgZ5K0LZE_w', 'name': {'type': 'String', 'value': 'Alice Johnson'}, 'email': {'value': 'alice.j@newdomain.com', 'type': 'String'}, 'age': {'type': 'Integer', 'value': 29}, 'active': {'type': 'Boolean', 'value': True}}
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
   Inserted with ripple: {'id': 'zx029H41vEUKKlHpoNhFFUCo6Bz_kqf8grfSf3gBGjRNXmsTxJAB0wKgxDiQtxFN9YJ3JjGoRA23G9dm63B-xw'}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {'id': '6ZMTWpRICH-9ff165IFutXmOamO6HWrvIyecKzVxmjMcJ1wXQmBWvevcpmGyC1Xl4oolGRJcH1hMC9Mm78PxjQ'}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {'id': 'zx029H41vEUKKlHpoNhFFUCo6Bz_kqf8grfSf3gBGjRNXmsTxJAB0wKgxDiQtxFN9YJ3JjGoRA23G9dm63B-xw', 'name': {'value': 'Product 1', 'type': 'String'}, 'price': {'type': 'Integer', 'value': 150}}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {'price': {'type': 'Integer', 'value': 500}, 'name': {'value': 'Upsert Product', 'type': 'String'}, 'id': 'zx029H41vEUKKlHpoNhFFUCo6Bz_kqf8grfSf3gBGjRNXmsTxJAB0wKgxDiQtxFN9YJ3JjGoRA23G9dm63B-xw'}

✅ All bypass_ripple operations completed successfully!
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields (id, name, email only)
  Found 3 active users
  Fields returned: ['name', 'email', 'id']
  First user: {'type': 'String', 'value': 'Alice Johnson'} <{'value': 'alice@example.com', 'type': 'String'}>

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ['created_at', 'status', 'avatar_url', 'id', 'age', 'bio', 'email', 'name', 'user_role']

Example 3: Complex query with projection (active users, ages 18-65)
  Found 3 active users (ages 18-65)
    - {'type': 'String', 'value': 'Dave Brown'} (age {'value': 45, 'type': 'Integer'})
    - {'type': 'String', 'value': 'Alice Johnson'} (age {'type': 'Integer', 'value': 30})
    - {'type': 'String', 'value': 'Bob Smith'} (age {'value': 25, 'type': 'Integer'})

Example 4: Query inactive users with profile fields
  Found 1 inactive users
    - {'value': 'Carol White', 'type': 'String'}: {'type': 'String', 'value': 'Manager'}

Example 5: Compare full vs projected data
  Full query:
    - 12 fields per record
    - Fields: ['id', 'api_key', 'age', 'password', 'status', 'user_role', 'name', 'created_at', 'secret_token', 'email', 'avatar_url', 'bio']
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
  Float 0.1 + 0.2 = None (should be 0.3)
  Decimal "0.30" = None (exact!)

=== Cleanup ===
✓ Cleaned up test keys

=== Summary ===
✅ Use field_decimal() for monetary values, percentages, and
   any case where floating-point errors are unacceptable.
✅ field_decimal() stores values as strings internally,
   preserving exact precision across all operations.
✓ Client created

=== Get All Chat Models ===
OpenAI models: ['text-embedding-ada-002', 'whisper-1', 'gpt-3.5-turbo', 'tts-1', 'gpt-3.5-turbo-16k', 'gpt-4-0613', 'gpt-4', 'davinci-002', 'babbage-002', 'gpt-3.5-turbo-instruct', 'gpt-3.5-turbo-instruct-0914', 'gpt-3.5-turbo-1106', 'tts-1-hd', 'tts-1-1106', 'tts-1-hd-1106', 'text-embedding-3-small', 'text-embedding-3-large', 'gpt-3.5-turbo-0125', 'gpt-4-turbo', 'gpt-4-turbo-2024-04-09', 'gpt-4o', 'gpt-4o-2024-05-13', 'gpt-4o-mini-2024-07-18', 'gpt-4o-mini', 'gpt-4o-2024-08-06', 'omni-moderation-latest', 'omni-moderation-2024-09-26', 'o1-2024-12-17', 'o1', 'o3-mini', 'o3-mini-2025-01-31', 'gpt-4o-2024-11-20', 'gpt-4o-mini-search-preview-2025-03-11', 'gpt-4o-mini-search-preview', 'gpt-4o-transcribe', 'gpt-4o-mini-transcribe', 'o1-pro-2025-03-19', 'o1-pro', 'gpt-4o-mini-tts', 'o3-2025-04-16', 'o4-mini-2025-04-16', 'o3', 'o4-mini', 'gpt-4.1-2025-04-14', 'gpt-4.1', 'gpt-4.1-mini-2025-04-14', 'gpt-4.1-mini', 'gpt-4.1-nano-2025-04-14', 'gpt-4.1-nano', 'gpt-image-1', 'o4-mini-deep-research', 'gpt-4o-transcribe-diarize', 'o4-mini-deep-research-2025-06-26', 'gpt-5-chat-latest', 'gpt-5-2025-08-07', 'gpt-5', 'gpt-5-mini-2025-08-07', 'gpt-5-mini', 'gpt-5-nano-2025-08-07', 'gpt-5-nano', 'gpt-audio-2025-08-28', 'gpt-realtime', 'gpt-realtime-2025-08-28', 'gpt-audio', 'gpt-5-codex', 'gpt-image-1-mini', 'gpt-5-pro-2025-10-06', 'gpt-5-pro', 'gpt-audio-mini', 'gpt-audio-mini-2025-10-06', 'gpt-5-search-api', 'gpt-realtime-mini', 'gpt-realtime-mini-2025-10-06', 'sora-2', 'sora-2-pro', 'gpt-5-search-api-2025-10-14', 'gpt-5.1-chat-latest', 'gpt-5.1-2025-11-13', 'gpt-5.1', 'gpt-5.1-codex', 'gpt-5.1-codex-mini', 'gpt-5.1-codex-max', 'gpt-image-1.5', 'gpt-5.2-2025-12-11', 'gpt-5.2', 'gpt-5.2-pro-2025-12-11', 'gpt-5.2-pro', 'gpt-5.2-chat-latest', 'gpt-4o-mini-transcribe-2025-12-15', 'gpt-4o-mini-transcribe-2025-03-20', 'gpt-4o-mini-tts-2025-03-20', 'gpt-4o-mini-tts-2025-12-15', 'gpt-realtime-mini-2025-12-15', 'gpt-audio-mini-2025-12-15', 'chatgpt-image-latest', 'gpt-5.2-codex', 'gpt-5.3-codex', 'gpt-realtime-1.5', 'gpt-audio-1.5', 'gpt-4o-search-preview', 'gpt-4o-search-preview-2025-03-11', 'gpt-5.3-chat-latest', 'gpt-5.4-2026-03-05', 'gpt-5.4-pro', 'gpt-5.4-pro-2026-03-05', 'gpt-5.4', 'gpt-5.4-nano-2026-03-17', 'gpt-5.4-nano', 'gpt-5.4-mini-2026-03-17', 'gpt-5.4-mini', 'gpt-image-2', 'gpt-image-2-2026-04-21', 'gpt-5.5', 'gpt-5.5-2026-04-23', 'gpt-5.5-pro', 'gpt-5.5-pro-2026-04-23', 'chat-latest', 'gpt-realtime-translate', 'gpt-realtime-2', 'gpt-realtime-whisper', 'gpt-5.6-sol', 'gpt-5.6-terra', 'gpt-5.6-luna', 'gpt-realtime-2.1', 'gpt-realtime-2.1-mini']
Anthropic models: ['claude-sonnet-5', 'claude-fable-5', 'claude-opus-4-8', 'claude-opus-4-7', 'claude-sonnet-4-6', 'claude-opus-4-6', 'claude-opus-4-5-20251101', 'claude-haiku-4-5-20251001', 'claude-sonnet-4-5-20250929', 'claude-opus-4-1-20250805']
Perplexity models: []

=== Get OpenAI Models ===
OpenAI models: ['text-embedding-ada-002', 'whisper-1', 'gpt-3.5-turbo', 'tts-1', 'gpt-3.5-turbo-16k', 'gpt-4-0613', 'gpt-4', 'davinci-002', 'babbage-002', 'gpt-3.5-turbo-instruct', 'gpt-3.5-turbo-instruct-0914', 'gpt-3.5-turbo-1106', 'tts-1-hd', 'tts-1-1106', 'tts-1-hd-1106', 'text-embedding-3-small', 'text-embedding-3-large', 'gpt-3.5-turbo-0125', 'gpt-4-turbo', 'gpt-4-turbo-2024-04-09', 'gpt-4o', 'gpt-4o-2024-05-13', 'gpt-4o-mini-2024-07-18', 'gpt-4o-mini', 'gpt-4o-2024-08-06', 'omni-moderation-latest', 'omni-moderation-2024-09-26', 'o1-2024-12-17', 'o1', 'o3-mini', 'o3-mini-2025-01-31', 'gpt-4o-2024-11-20', 'gpt-4o-mini-search-preview-2025-03-11', 'gpt-4o-mini-search-preview', 'gpt-4o-transcribe', 'gpt-4o-mini-transcribe', 'o1-pro-2025-03-19', 'o1-pro', 'gpt-4o-mini-tts', 'o3-2025-04-16', 'o4-mini-2025-04-16', 'o3', 'o4-mini', 'gpt-4.1-2025-04-14', 'gpt-4.1', 'gpt-4.1-mini-2025-04-14', 'gpt-4.1-mini', 'gpt-4.1-nano-2025-04-14', 'gpt-4.1-nano', 'gpt-image-1', 'o4-mini-deep-research', 'gpt-4o-transcribe-diarize', 'o4-mini-deep-research-2025-06-26', 'gpt-5-chat-latest', 'gpt-5-2025-08-07', 'gpt-5', 'gpt-5-mini-2025-08-07', 'gpt-5-mini', 'gpt-5-nano-2025-08-07', 'gpt-5-nano', 'gpt-audio-2025-08-28', 'gpt-realtime', 'gpt-realtime-2025-08-28', 'gpt-audio', 'gpt-5-codex', 'gpt-image-1-mini', 'gpt-5-pro-2025-10-06', 'gpt-5-pro', 'gpt-audio-mini', 'gpt-audio-mini-2025-10-06', 'gpt-5-search-api', 'gpt-realtime-mini', 'gpt-realtime-mini-2025-10-06', 'sora-2', 'sora-2-pro', 'gpt-5-search-api-2025-10-14', 'gpt-5.1-chat-latest', 'gpt-5.1-2025-11-13', 'gpt-5.1', 'gpt-5.1-codex', 'gpt-5.1-codex-mini', 'gpt-5.1-codex-max', 'gpt-image-1.5', 'gpt-5.2-2025-12-11', 'gpt-5.2', 'gpt-5.2-pro-2025-12-11', 'gpt-5.2-pro', 'gpt-5.2-chat-latest', 'gpt-4o-mini-transcribe-2025-12-15', 'gpt-4o-mini-transcribe-2025-03-20', 'gpt-4o-mini-tts-2025-03-20', 'gpt-4o-mini-tts-2025-12-15', 'gpt-realtime-mini-2025-12-15', 'gpt-audio-mini-2025-12-15', 'chatgpt-image-latest', 'gpt-5.2-codex', 'gpt-5.3-codex', 'gpt-realtime-1.5', 'gpt-audio-1.5', 'gpt-4o-search-preview', 'gpt-4o-search-preview-2025-03-11', 'gpt-5.3-chat-latest', 'gpt-5.4-2026-03-05', 'gpt-5.4-pro', 'gpt-5.4-pro-2026-03-05', 'gpt-5.4', 'gpt-5.4-nano-2026-03-17', 'gpt-5.4-nano', 'gpt-5.4-mini-2026-03-17', 'gpt-5.4-mini', 'gpt-image-2', 'gpt-image-2-2026-04-21', 'gpt-5.5', 'gpt-5.5-2026-04-23', 'gpt-5.5-pro', 'gpt-5.5-pro-2026-04-23', 'chat-latest', 'gpt-realtime-translate', 'gpt-realtime-2', 'gpt-realtime-whisper', 'gpt-5.6-sol', 'gpt-5.6-terra', 'gpt-5.6-luna', 'gpt-realtime-2.1', 'gpt-realtime-2.1-mini']

=== Get Anthropic Models ===
Anthropic models: ['claude-sonnet-5', 'claude-fable-5', 'claude-opus-4-8', 'claude-opus-4-7', 'claude-sonnet-4-6', 'claude-opus-4-6', 'claude-opus-4-5-20251101', 'claude-haiku-4-5-20251001', 'claude-sonnet-4-5-20250929', 'claude-opus-4-1-20250805']

✓ Chat Models API example complete
✓ Client created

=== Create User Function ===
Created user function with ID: pBb5lOW1fCwy6giLSKkvI5sMP8oNOrUmn96JWQbRqwMiczNyhBQDH9kd0xbncqc1QXEyupcikNQNQbXfDuQoQA

=== Get User Function ===
Retrieved: get_active_users_py - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 14 user functions:
  - fetch_and_store_user: Fetch user from API and cache in KV
  - get_high_scoring_active_users: Get High Scoring Active Users
  - get_users_by_status: Get Users By Status
  - get_active_users_py: Get Active Users
  - swr_user: SWR pattern for user data (KV-based)
  - get_active_users_updated: Get Active Users (Updated)
  - fetch_user: Fetch user by code
  - get_active_users_updated: Get Active Users (Updated)
  - get_active_users_paginated: Get Active Users (Paginated)
  - cache_api_call_py: Cache External API Call
  - fetch_slim_user: Validate and slim down user
  - get_verified_user: Get verified and validated user
  - get_user_wrapper: Wrapper that calls fetch_user
  - validate_user: Check if user exists

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
✓ Client created
✓ crypto_demo_hmac saved
✓ crypto_demo_aes saved
✓ crypto_demo_uuid saved
✓ crypto_demo_totp saved
✓ crypto_demo_encoding saved

Invoke them with:
  POST /api/functions/crypto_demo_hmac     { "payload": "hi" }
  POST /api/functions/crypto_demo_aes      { "plaintext": "secret" }
  POST /api/functions/crypto_demo_uuid
  POST /api/functions/crypto_demo_totp
  POST /api/functions/crypto_demo_encoding { "title": "Héllo World" }

✓ Cleaned up demo functions
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
✅ [32mPython client examples complete![0m
