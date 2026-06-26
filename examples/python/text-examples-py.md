make test-examples-python
📦 [36mEnsuring Python example dependencies in .venv...[0m
🧪 [36mRunning Python examples (direct HTTP/WebSocket)...[0m
=== Simple CRUD Operations (Direct HTTP) ===

✓ Authentication successful

=== Insert Document ===
Inserted: {'id': 'Xah1Rbe6OAMilzKoLhrgu-SDg9RgFjwHpwMKG9a_Xpi3pd0Fw3PrO8xlwLTEOoqQA8uN233BawHkhaTLbiGzVQ'}

=== Find by ID ===
Found: {'active': {'type': 'Boolean', 'value': True}, 'value': {'type': 'Integer', 'value': 42}, 'id': 'Xah1Rbe6OAMilzKoLhrgu-SDg9RgFjwHpwMKG9a_Xpi3pd0Fw3PrO8xlwLTEOoqQA8uN233BawHkhaTLbiGzVQ', 'name': {'value': 'Test Record', 'type': 'String'}}

=== Find with Query ===
Found documents: [{'active': {'type': 'Boolean', 'value': True}, 'id': 'Xah1Rbe6OAMilzKoLhrgu-SDg9RgFjwHpwMKG9a_Xpi3pd0Fw3PrO8xlwLTEOoqQA8uN233BawHkhaTLbiGzVQ', 'value': {'type': 'Integer', 'value': 42}, 'name': {'type': 'String', 'value': 'Test Record'}}]

=== Update Document ===
Updated: {'value': {'value': 100, 'type': 'Integer'}, 'active': {'type': 'Boolean', 'value': True}, 'name': {'type': 'String', 'value': 'Updated Record'}, 'id': 'Xah1Rbe6OAMilzKoLhrgu-SDg9RgFjwHpwMKG9a_Xpi3pd0Fw3PrO8xlwLTEOoqQA8uN233BawHkhaTLbiGzVQ'}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: WPqvrkImxJdJQGknbhfZE_zv7HPfMd2R_AWgTwv8FDfhndwQgMgaizHru0D74aM7v4u_IW-2BVaM0HqFXLcpUg

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
          "value": true,
          "type": "Boolean"
        },
        "id": "WPqvrkImxJdJQGknbhfZE_zv7HPfMd2R_AWgTwv8FDfhndwQgMgaizHru0D74aM7v4u_IW-2BVaM0HqFXLcpUg",
        "value": {
          "value": 42,
          "type": "Integer"
        }
      },
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "OZdVF92oGdfsvvOSD4yyE45G1-Ue61ATHNGkDBRj_10Z-cgCfY2PomLVC0mMdOXbhG8OuJzrg8y4cBsbOzDxyw",
        "value": {
          "type": "Integer",
          "value": 42
        },
        "name": {
          "type": "String",
          "value": "WebSocket Test Record"
        }
      }
    ]
  },
  "messageId": "1779559684"
}
✓ Retrieved 2 record(s) via WebSocket

✓ WebSocket example completed successfully
🚀 ekoDB Functions Example (Python/HTTP)

✓ Authentication successful

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Function saved: oZOlBzsfRFsH-3nCZAt5FFAjUjh_9wsQQNckRRBPYAcKr4kPlWpekb3DwjKe83_CO9-bl-SDfzgpk2-mDhw9NA
📊 Found 15 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

ℹ️  Function 'get_active_users_paginated' already existed — updated instead
✅ Function saved: get_active_users_paginated
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: H4PHLxzCH3yP6_oZ4M7yPaCkvqqdu7eyag-SehZpTYfW5rMvK3FEHkAGrg0ZSGeb7JxWOy-JNbl9AIKMVNcTlQ
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {'max_score': {'value': 90, 'type': 'Integer'}, 'count': {'value': 15, 'type': 'Integer'}, 'status': {'value': 'inactive', 'type': 'String'}, 'avg_score': {'value': 50.0, 'type': 'Float'}}
   {'count': {'value': 15, 'type': 'Integer'}, 'max_score': {'type': 'Integer', 'value': 100}, 'avg_score': {'value': 60.0, 'type': 'Float'}, 'status': {'type': 'String', 'value': 'active'}}

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
Collection created with first record: Y3Es3n-9NzZ3pC3V6NkoNg7QWk8IBGclUbBzRcpHOVr3mbEtq3Fs0spugeEPFkYewz8c_WRgGXiM4VcpiIrHlg

=== List Collections ===
Total collections: 14
Sample collections: ['test_collection', 'functions__ek0_testing', 'demo_collection', 'ttl_cache', 'chat_messages__ek0_testing']

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
Created Alice: $1000 - ID: 33iKPKjE_SRhXWIKNlSb6DvUw4zMMyjV9kZl6JIq2ZjAz8Z4xIh008XWZBsQ2I89qPRxMu8CCxdIrTRYMHeXAw
Created Bob: $500 - ID: 8LoQDvfBoGmtA3ecQoocnayNJQrNKgzCYclvjIARIyHNPTOOm01vEMGkHGPND3UukLx5AJZVKMMIoaUuezYemg

=== Example 1: Begin Transaction ===
Transaction ID: 57ea0f39-b81b-4468-b189-1fa75c13f15c

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
New transaction: ebba06ec-594e-489f-a4f5-d14ea43e81bd
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'type': 'Integer', 'value': 700}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: j8522t_c3zGV0wkPjjBkTKzKruVkP4TCZzw2Mn45Vg0v3ZvjZF3pMCXHY2DHZtAw_hnM0mYKW8nzkwihwmGolw
Created Bob: $500 - ID: rx5t2ADsAsAn0GO9eIXy87be2nBQb7VtmS1xfBSNzvy3CHyxjp-AexX4IloG5xcxJPGWbqq1aT55u5Mz-xjYYg

=== Example 1: Begin Transaction ===
Transaction ID: 79dcf7ca-441e-4426-8cf6-892f3128fe09

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
Created Alice: $1000 - ID: TXqjz44JJ-2JFes1bIep6ctRyF3iTxzu1nZFbQ8YROX9L0XeP4sA9xrZjlGu2XPjm1fGv5E0RvRO_0LBex3fQA
Created Bob: $500 - ID: Q7nY2DGv5xjDbOIRTAtbQO07mQ3bPwlK65jDsWlxDWhmBiSjBseDEsiTgVjGqv3LCVYsiZZinfb9lYvlRR6r-w

=== Example 1: Begin Transaction ===
Transaction ID: b5682218-3438-40c6-8a49-00156e7cc85c

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
New transaction: 71237a39-baad-4d8a-9a11-14ca5bd3db4e
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'value': 700, 'type': 'Integer'}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: TmH1dK6reIBglvDWbHudbIESK839VKNfTgUme_Y2xQUjP0tFkwkxrm7nUBfIo9D11-8Hh9JVTvvDreKBRPQCtA
Created Bob: $500 - ID: 077Z6t-bJeW4-DDdPU084NcGiMOm5514lnoDPqv66d57d8SyzUTJ57MLQjUyYQ05Nk8xMEdPEd-Ip2CpaACACg

=== Example 1: Begin Transaction ===
Transaction ID: 73126efc-8888-4227-a1b7-f58b3f8d5595

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
Created Alice: $1000 - ID: z2hjIVqfX_44iI3eulRUM-FrMemLzc4Rw9G-UAHLcp7cnnGcVZcvCTp7XumAg7a0czVeGwBatJ3r4YpMfbRJ1A
Created Bob: $500 - ID: Gh990eS12EM_jPsf2HbBc1q_95zhsdgfcbxRuMWIgoJSy7Y6GMzhuF2oc7Cq5Gce7JbJiippslwFLGaMar2LZg

=== Example 1: Begin Transaction ===
Transaction ID: 36ffc907-d843-4e6e-bb1f-7f77e644dc69

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
New transaction: 99ae94c8-1011-41d9-8ecb-da4496022da5
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'value': 700, 'type': 'Integer'}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: gyTktnnINiB41eCV_aAkMBnN8cBegDjslkJmShUSSmEIsZgr7tD0ts7sywxkGEP8fRTsudiFk3bk3OaHVeUEIA
Created Bob: $500 - ID: E-9tfOIgARELqmywfyoJ3-H7GsfOAIqfMDcJgzs8OAfw_IJhz3dkC826Qa37Xj5jSqS2AV5GgAGhARzS8x84EA

=== Example 1: Begin Transaction ===
Transaction ID: 0052185b-0d60-4005-9cdf-0d936b705875

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
   ✅ Function saved: bOy3FQIPoZSbp16wgwx-tA3LJ-lDGu80xrO-hmyJH2Cns5TlE-_FF9OktwcwVNHczvnDnAUqzxWboxigMh5pHA

2️⃣ Calling function (Insert + Verify)...
   ✅ function executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: 3uwCKf8J6V8ggCUMgZvA-sNZ8EKRtB70HRic3brpLJht7R5lRpuHahZRRms6PkifeNSNwi9t5nddKHJA07743g
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}
   📋 Email: {'value': 'alice@example.com', 'type': 'String'}
   📋 Status: {'value': 'pending', 'type': 'String'}
   📋 Credits: {'type': 'Integer', 'value': 0}

============================================================
📝 function 2: Query + Update + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: YBzOGsEOExfrwgdFJomszigxkb0tWArwGnfbbtIS1Fk6Cpw8tEhoFVD51nugLXK4wyHBmKanS2vTAEih70WgmA

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
   ✅ Function saved: zOY-rBaL10PJybdiQ7x6GC_QlIlX3F5-nu2sGreGJDEAKBXuOgZrWaweFcwg4cPbnyovLp-OE3YQaABnYH9pfw

2️⃣ Calling function (Query + Update Credits + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {'type': 'Integer', 'value': 0}
   📋 Status: {'value': 'active', 'type': 'String'}
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}

============================================================
📝 function 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: 5a9xWSLy8xPFusCWwUG-RKfVEkug5c3A_mWp2J_RoPdsi8NhJCYgrslV3qaSnhkW2nnkCYZIPqGShu0udSr9Vg

2️⃣ Calling function (Query + Delete + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: bOy3FQIPoZSbp16wgwx-...
   ✅ Deleted script: YBzOGsEOExfrwgdFJoms...
   ✅ Deleted script: zOY-rBaL10PJybdiQ7x6...
   ✅ Deleted script: 5a9xWSLy8xPFusCWwUG-...
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
  Output: Document ID = tAXA3_d8JKcbeYtwcAYbOpInHD4v_CI8ZtYn1zMf_dQfx6vAI3FHZOpO1UpjUME6tZhmDwpswVYmJMejO6MtWg
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: find_by_id(tAXA3_d8JKcbeYtwcAYbOpInHD4v_CI8ZtYn1zMf_dQfx6vAI3FHZOpO1UpjUME6tZhmDwpswVYmJMejO6MtWg)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: find_by_id(tAXA3_d8JKcbeYtwcAYbOpInHD4v_CI8ZtYn1zMf_dQfx6vAI3FHZOpO1UpjUME6tZhmDwpswVYmJMejO6MtWg)
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
  Output: Document ID = e1YQyrd0lNEt8nyeKxFwCt9HB9s-n-nprNIfl3tgvyy6iaNtoSITr8KXwmE4MnHPtYb3vxnujkJlXOdgxq564w
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: find_by_id(e1YQyrd0lNEt8nyeKxFwCt9HB9s-n-nprNIfl3tgvyy6iaNtoSITr8KXwmE4MnHPtYb3vxnujkJlXOdgxq564w)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: find_by_id(e1YQyrd0lNEt8nyeKxFwCt9HB9s-n-nprNIfl3tgvyy6iaNtoSITr8KXwmE4MnHPtYb3vxnujkJlXOdgxq564w)
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
🐍 Found CPython 3.14 at /opt/homebrew/opt/python@3.14/bin/python3.14
🔗 Found pyo3 bindings with abi3-py3.8 support
💻 Using `MACOSX_DEPLOYMENT_TARGET=11.0` for aarch64-apple-darwin by default
    Finished `release` profile [optimized] target(s) in 0.17s
📦 Built wheel for abi3 Python ≥ 3.8 to ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.22.0-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel into .venv...[0m
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.22.0-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.22.0
    Uninstalling ekodb_client-0.22.0:
      Successfully uninstalled ekodb_client-0.22.0
Successfully installed ekodb-client-0.22.0
✅ [32mPython client package built and installed![0m
🧪 [36mRunning Python client library examples...[0m
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {'id': 'y7f16jLcg8TSm7H44oLVGkMA0hs2LLXGtU0RoNrk0LFvxnshvAIC-SLWkFy6sg3hkH91SFs7vL6IrSAAum_nyA'}

=== Find by ID ===
Found: {'value': {'type': 'Integer', 'value': 42}, 'active': {'type': 'Boolean', 'value': True}, 'categories': {'value': ['electronics', 'computers'], 'type': 'Array'}, 'metadata': {'value': {'key': 'value', 'nested': {'deep': True}}, 'type': 'Object'}, 'tags': {'type': 'Array', 'value': ['tag1', 'tag2', 'tag3']}, 'embedding': {'type': 'Array', 'value': [0.1, 0.2, 0.3, 0.4, 0.5]}, 'name': {'value': 'Test Record', 'type': 'String'}, 'user_id': {'type': 'String', 'value': '550e8400-e29b-41d4-a716-446655440000'}, 'price': {'value': 99.99, 'type': 'Float'}, 'id': 'y7f16jLcg8TSm7H44oLVGkMA0hs2LLXGtU0RoNrk0LFvxnshvAIC-SLWkFy6sg3hkH91SFs7vL6IrSAAum_nyA', 'data': {'value': 'aGVsbG8gd29ybGQ=', 'type': 'String'}, 'created_at': {'type': 'String', 'value': '2026-06-26T12:52:30.515842'}}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): True
  price (Decimal): 99.99
  created_at (DateTime): 2026-06-26 12:52:30.515842
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): ['tag1', 'tag2', 'tag3']
  metadata (Object): {'key': 'value', 'nested': {'deep': True}}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): ['electronics', 'computers']
  data (Bytes): 11 bytes
Plain record: {'value': 42, 'active': True, 'categories': ['electronics', 'computers'], 'metadata': {'key': 'value', 'nested': {'deep': True}}, 'tags': ['tag1', 'tag2', 'tag3'], 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5], 'name': 'Test Record', 'user_id': '550e8400-e29b-41d4-a716-446655440000', 'price': 99.99, 'id': 'y7f16jLcg8TSm7H44oLVGkMA0hs2LLXGtU0RoNrk0LFvxnshvAIC-SLWkFy6sg3hkH91SFs7vL6IrSAAum_nyA', 'data': 'aGVsbG8gd29ybGQ=', 'created_at': '2026-06-26T12:52:30.515842'}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'categories': {'type': 'Array', 'value': ['electronics', 'computers']}, 'data': {'value': 'aGVsbG8gd29ybGQ=', 'type': 'String'}, 'name': {'type': 'String', 'value': 'Updated Record'}, 'user_id': {'type': 'String', 'value': '550e8400-e29b-41d4-a716-446655440000'}, 'price': {'value': 99.99, 'type': 'Float'}, 'tags': {'type': 'Array', 'value': ['tag1', 'tag2', 'tag3']}, 'id': 'y7f16jLcg8TSm7H44oLVGkMA0hs2LLXGtU0RoNrk0LFvxnshvAIC-SLWkFy6sg3hkH91SFs7vL6IrSAAum_nyA', 'active': {'value': True, 'type': 'Boolean'}, 'created_at': {'type': 'String', 'value': '2026-06-26T12:52:30.515842'}, 'metadata': {'value': {'key': 'value', 'nested': {'deep': True}}, 'type': 'Object'}, 'embedding': {'type': 'Array', 'value': [0.1, 0.2, 0.3, 0.4, 0.5]}, 'value': {'value': 100, 'type': 'Integer'}}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: m07itEkmkNnBB4tguIeH0CTtBpVJXOQD3zwPRp09zYpfN_T3XVLVNnqVOAutgSfjToXXcbTbX3BYPLCWlwLHXA

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
Collection created with first record: "Ao70H8q0jXfAtIhwMsNYbaINv8rHqcn4KTNugs4CJcXNpeA5DK5uaVMbm84gMvrdBQS1pXF33i04PRG7MaH2sg"

=== List Collections ===
Total collections: 13
Sample collections: ['test_collection', 'functions__ek0_testing', 'ttl_cache', 'chat_messages__ek0_testing', 'audit__ek0_testing']

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
Created Alice: $1000 - ID: hrWWv2f6FiEjMw3wvntrlY-PhzlkYKyTFZ9SfhY2-SlYCGVKzbyz2djRnbtJKBERbdKxQoYrlTnBwq7w74pJUQ
Created Bob: $500 - ID: EgyR8CzdPHQNUzhWLdiebQoiUb5L3V7O6jtikdH1uajVVVWPAes3YjG9vrG4NOmyqqXGM13IRH8XJCjEDvRRpw

=== Example 1: Begin Transaction ===
Transaction ID: fb556514-5238-4465-810c-2f1e962567b1

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 18d46038-e320-4656-b2db-686073226f6d
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
  4. Score: 13.200, Matched: bio, title

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 26.400, Matched: title, bio
  2. Score: 26.400, Matched: bio, title
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
  1. Score: 0.768
  2. Score: 0.761
  3. Score: 0.740

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.370, Matched: content, title
  2. Score: 0.880, Matched: title, content
  3. Score: 0.384, Matched: 

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
✓ Inserted document: zUs_JCSaJgiUlRnd4OpTjy44RC4VOKRQ29JkbZYcwoK1HmXYa0WVavHJRFcI3l2lBcD1us3TACO26UIuM-LB3g

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: UwdrHn9j80SEpxRlfidfYDWJo3gRaKLKga4hiq91o_Ids-QF38Dqy6jtBVgW9zFZSg4EX5YGQVLzZ4P2oq6Iow

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
✓ Inserted document with TTL: 6SJq7alfnYLwYhlJjhnLi-9Xu1JSViuBUzcNW3ch-_GDoaEHBKW4_wE3Em8y3QvB1n-I_K-OQ6eU3DBke2hz5A

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
✓ Edge cache script created: ES64RkWj0JW7E5FHprrhWmsdOjlud0k1honmbzsy4j4He7k4nfHMOZ73om7DrJVkKDbSH_C-gWQLU2097DNwkQ

Call 1: Cache miss (fetches from API)
Response time: 431ms
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "current": {
            "interval": 900,
            "temperature_2m": 25.9,
            "time": "2026-06-26T16:45"
          },
          "current_units": {
            "interval": "seconds",
            "temperature_2m": "\u00b0C",
            "time": "iso8601"
          },
          "elevation": 32.0,
          "generationtime_ms": 0.011205673217773438,
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
Response time: 11ms (40.9x faster!)
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "current": {
            "interval": 900,
            "temperature_2m": 25.9,
            "time": "2026-06-26T16:45"
          },
          "current_units": {
            "interval": "seconds",
            "temperature_2m": "\u00b0C",
            "time": "iso8601"
          },
          "elevation": 32.0,
          "generationtime_ms": 0.011205673217773438,
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

✅ Function saved: 74zXRKvj-lgyQn-NqHQtun4iC_V-BSnq_TlbkEJKxNPtbbJnjPFBhjRVQ3C2jDkoFaUe5faIvLa21Dr6wtKtMw
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
   ⏱️  Duration: 1.5ms
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
   ⏱️  Duration: 1.6ms
   📊 Records: 1
   📦 Data: {
      "value": {
            "type": "Object",
            "value": {
                  "address": {
                        "city": "Gwenborough",
                        "geo": {
                 ...
   🚀 Cache speedup: 0.9x faster!

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

✅ Function saved: iX7EOEvntwRssdwwv1h1IbqLlHWvPPbo4fSdoylGOQfIcGZDXWRVcWUPrAsMfDOPnwTXwd4tppuRl7bo_ApOmQ
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

✅ Inserted order: Nmj4NOH6LjHsdLRJVyyWwYYwxsl_SDN74M5VHQJGUZonmsT4aKzO57_k2UetCtAkq5l8HySXbBaQpV-Y0SogDQ
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

✅ Inserted order: Oa6XY7e-pDd20255jv4YnvDndvr3DHvfQhik_UnmJOiGTyo2Zmmo4MenmhWPI_cmz7q4NcrBRKEBVaFdL14L-A
✅ Cached order status
📊 Quick status lookup: {'value': '{"status":"processing","updated_at":"2026-06-26T16:52:33.418802+00:00"}'}

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
✓ Created SWR script: fetch_api_user_py (fowGnGeclSjnKtbaZ46yghCPUxm-npvKSDL0nHOuIc0lDctuuwefLgJOBrqEep35-e8q1Ew8C0FK3c1-jCUlug)

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
✓ Created native SWR script: github_user_native (BiSbYurO0DVDar49P4PNltdD61z-FpUIDBWBRbZa-lRvcGVTeGCItEI5Pc7I7LW0-VxhkgQmijdWbPS1vswc5A)

First call (cache miss - will fetch from GitHub API):
  Response time: 120ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 3ms
  Speedup: 45.2x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (lHxRbhfK0jkjR3niuNvn9OQuRMiuXb4hhzCxeNAq_Sp01VHI_Nx-2ATVQbspieemCURVLhOmSpdg637FgwM2Og)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (a2Sc3WvhhApUgEj9mKWcxQQ41JV1-tg-st0oHmSoIhLXdSYaIqEx4lliri-9emF3pTMHI2Fvh1D2Bo0tQez6uQ)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (a04f2U7hOxMCofTGT1GwM1qFGVNnyJBOtUO1jKbkqkJ_bXWRObX7jPJHCyBO1EKTWPcVtXHa_44jauXjA88Emg)
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
   Vector databases offer several benefits:

1. **Efficient Similarity Search**: They excel in retrieving similar items using vector embeddings, which is ideal for applications like image recognition and recommendation systems.

2. **High Dimensional Data Handling**: Vector databases effectively manage and query high-dimensional data, enabling complex queries that traditional databases struggle with.

3. **Scalability**: They are designed to scale with large datasets, accommodating growth in data and queries without significant performance loss.

4. **Real-time Performance**: Many vector databases provide low-latency responses, making them suitable for real-time applications.

5. **Integration with Machine Learning**: They work seamlessly with machine learning models to store and retrieve embeddings, enhancing AI-driven applications.

6. **Flexible Querying**: Users can execute various types of queries, such as k-nearest neighbor searches, to derive insights from data.

7. **Semantic Understanding**: Vector representations capture semantic relationships, improving the interpretation of data in NLP and other contexts.

8. **Enhanced Search Capabilities**: They support advanced search functionalities, such as faceted search and interactive querying based on similarity.

Overall, vector databases are particularly beneficial for applications focused on managing and querying complex, high-dimensional data effectively.
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
   {'avg_score': {'type': 'Float', 'value': 70.0}, 'count': {'type': 'Integer', 'value': 7}, 'role': {'type': 'String', 'value': 'user'}}
   {'avg_score': {'type': 'Float', 'value': 20.0}, 'count': {'type': 'Integer', 'value': 3}, 'role': {'type': 'String', 'value': 'admin'}}
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
   1. Vector Databases Explained (Database)
   2. Introduction to Machine Learning (AI)
   3. Natural Language Processing (AI)
   4. Getting Started with ekoDB (Database)
   5. Database Design Principles (Database)
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
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: dNGb9c_qlepdYXRMjHPAGJy-MDa4ulQLp2WDlIqDZw6P0oMFTVsKtL3Kbm7QmxX2GWwRZn8COanmCIfYs3kCBA

=== Sending Chat Message ===
Message ID: XRT7RlUJ9nwJA4R7BS66A2_2cfQ78jOhxogmGJRZwZoIY33ib_uEs7k1f_HtrcUF1zhjNxRX23DyaiOeqjHbOQ

=== AI Response ===
The available products and their prices are as follows:

1. **ekoDB Cloud**
   - Price: $499
   - Description: Fully managed cloud database service product.

2. **ekoDB**
   - Price: $99
   - Description: A high-performance database product with AI capabilities.

3. **ekoDB Pro**
   - Price: $299
   - Description: Enterprise edition product with advanced features.

Execution Time: 11565ms

=== Token Usage ===
Prompt tokens: 3413
Completion tokens: 91
Total tokens: 3504

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: l8Irj6iHxXLJiDvqHYzsGL-I9z-X23UGygGkme7y6lNCp4PjTfQsSFnBrTU5U4O3bu92dGpAt_ZaNpgnAlksvg

=== Sending Initial Message ===
✓ Message sent
  Response: It seems there are currently no available products in the database. However, based on the context provided, I found the following product:

- **Name**: ekoDB
- **Description**: High-performance database product
- **Price**: $99

If you have any other questions or if there's anything specific you're looking for, feel free to ask!

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['context_snippets', 'chat_id', 'created_at', 'role', 'llm_model', 'token_usage', 'llm_provider', 'content', 'id', 'updated_at'])
Debug: First message role: {'type': 'String', 'value': 'assistant'}
=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of ekoDB is $99. If you need more information or have any other questions, feel free to ask!

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: ukiK-AXE26Q3Zjg_6vlCTPlIKs6tPt_cZ1FJZmS2k3GLm7E9jvnL_X7O14_DOGfmoVXa1X1BaT06zCM8dy9weQ
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
✓ Created session: umL4YpWC2US-LpHHuvmBs8BYLWUZPtf4fhOS7CgLFregK-JnW1WTBFmwvTUbIcEbYV_DKsiFtMz2vlbMKkqh7w

=== Sending Messages ===
✓ Message 1 sent
  Response: It seems there are no records available in the database regarding products. However, from the context provided, I found information about a product:

- **Name:** ekoDB
- **Description:** A high-performance database product
- **Price:** $99

If you are looking for more specific details or different products, please provide additional context or clarify your request!

✓ Message 2 sent
  Response: The price of ekoDB is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 1xlbu1PejFWAh14E_CAWeWalbzvJ_Ejjv3mCBa043nrR5ntGIQCemyV9Y85MWjMyt7baxRjchkieocZugla49A
  Parent: umL4YpWC2US-LpHHuvmBs8BYLWUZPtf4fhOS7CgLFregK-JnW1WTBFmwvTUbIcEbYV_DKsiFtMz2vlbMKkqh7w

=== Listing Sessions ===
✓ Found 7 sessions
  Session 1: 1xlbu1PejFWAh14E_CAWeWalbzvJ_Ejjv3mCBa043nrR5ntGIQCemyV9Y85MWjMyt7baxRjchkieocZugla49A (Untitled)
  Session 2: umL4YpWC2US-LpHHuvmBs8BYLWUZPtf4fhOS7CgLFregK-JnW1WTBFmwvTUbIcEbYV_DKsiFtMz2vlbMKkqh7w (Untitled)
  Session 3: ukiK-AXE26Q3Zjg_6vlCTPlIKs6tPt_cZ1FJZmS2k3GLm7E9jvnL_X7O14_DOGfmoVXa1X1BaT06zCM8dy9weQ (Untitled)
  Session 4: dNGb9c_qlepdYXRMjHPAGJy-MDa4ulQLp2WDlIqDZw6P0oMFTVsKtL3Kbm7QmxX2GWwRZn8COanmCIfYs3kCBA (Untitled)
  Session 5: 6iCjiw0uTvLDoU1C2PDmYU20G9zcAOYlEnu-WLoDMnO2zotdLwhU4slQ0dBk3ZXelzYuE0Suy_FNS2QhZ3KUPw (Untitled)
  Session 6: FjhdyrsoiSxi3Wvhn_BjeTxxYLupEHEXgBlIAm75KSmSVRD5jV6FYxG0fBI4F5AEpPH2IIcsnreDVjMJLcTrcQ (Untitled)
  Session 7: ieawk8LiucOs5nKI_C_wHeCLjw92fUJnpMz3PFAce2AWqQtREuB4lsOSg9wo8gn3QNFF03_aSZGLtGlGMSasFw (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: 1xlbu1PejFWAh14E_CAWeWalbzvJ_Ejjv3mCBa043nrR5ntGIQCemyV9Y85MWjMyt7baxRjchkieocZugla49A

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Dict Creation ===
✓ Created record with plain dict: {'id': 'LsO2qxhoaheVJwHDUXGQNxQRVB61VnHw9lnDaNv91w0ua9uPO-NRaFMZjVeoUQynwHQaGWQgQOWaV_yoVThg7A'}

=== Upsert Operation ===
✓ Upsert (update existing record): LsO2qxhoaheVJwHDUXGQNxQRVB61VnHw9lnDaNv91w0ua9uPO-NRaFMZjVeoUQynwHQaGWQgQOWaV_yoVThg7A
✓ Inserted second record: xf4rglBbkwz6_e7Xt7Cc5QIA0ztrokM4kudXq0tZen20-1qo87Y1unNQXo7KqGnTsxJgjwmi4Sl_2BoFOaQadw
✓ Upsert (update second record): xf4rglBbkwz6_e7Xt7Cc5QIA0ztrokM4kudXq0tZen20-1qo87Y1unNQXo7KqGnTsxJgjwmi4Sl_2BoFOaQadw

=== Find One Operation ===
✓ Found user by email: {'age': {'value': 29, 'type': 'Integer'}, 'name': {'type': 'String', 'value': 'Alice Johnson'}, 'id': 'LsO2qxhoaheVJwHDUXGQNxQRVB61VnHw9lnDaNv91w0ua9uPO-NRaFMZjVeoUQynwHQaGWQgQOWaV_yoVThg7A', 'email': {'value': 'alice.j@newdomain.com', 'type': 'String'}, 'active': {'value': True, 'type': 'Boolean'}}
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
   Inserted with ripple: {'id': 'FnrNplAoxFME4VahPjCBgghjFLGQw0FlkGUFYIu_Kco2eU8CCbiRCojp7X-IEiMS770GHe1tJPIGQXYQMpwwLQ'}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {'id': 'EKsm-3Njx9VFfHhN-SfdxIJbrpWDgyMifH3rgf8Y14d33uxVULd33mmf7rr5Htl6GXTRLSf8OU65-dbu6TX0Yg'}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {'id': 'FnrNplAoxFME4VahPjCBgghjFLGQw0FlkGUFYIu_Kco2eU8CCbiRCojp7X-IEiMS770GHe1tJPIGQXYQMpwwLQ', 'price': {'value': 150, 'type': 'Integer'}, 'name': {'value': 'Product 1', 'type': 'String'}}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {'name': {'type': 'String', 'value': 'Upsert Product'}, 'id': 'FnrNplAoxFME4VahPjCBgghjFLGQw0FlkGUFYIu_Kco2eU8CCbiRCojp7X-IEiMS770GHe1tJPIGQXYQMpwwLQ', 'price': {'type': 'Integer', 'value': 500}}

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
  Fields returned: ['created_at', 'id', 'email', 'status', 'age', 'avatar_url', 'name', 'bio', 'user_role']

Example 3: Complex query with projection (active users, ages 18-65)
  Found 3 active users (ages 18-65)
    - {'value': 'Dave Brown', 'type': 'String'} (age {'type': 'Integer', 'value': 45})
    - {'value': 'Alice Johnson', 'type': 'String'} (age {'value': 30, 'type': 'Integer'})
    - {'type': 'String', 'value': 'Bob Smith'} (age {'type': 'Integer', 'value': 25})

Example 4: Query inactive users with profile fields
  Found 1 inactive users
    - {'type': 'String', 'value': 'Carol White'}: {'value': 'Manager', 'type': 'String'}

Example 5: Compare full vs projected data
  Full query:
    - 12 fields per record
    - Fields: ['created_at', 'id', 'password', 'status', 'email', 'avatar_url', 'user_role', 'api_key', 'age', 'secret_token', 'bio', 'name']
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
OpenAI models: ['text-embedding-ada-002', 'whisper-1', 'gpt-3.5-turbo', 'tts-1', 'gpt-3.5-turbo-16k', 'gpt-4-0613', 'gpt-4', 'davinci-002', 'babbage-002', 'gpt-3.5-turbo-instruct', 'gpt-3.5-turbo-instruct-0914', 'gpt-3.5-turbo-1106', 'tts-1-hd', 'tts-1-1106', 'tts-1-hd-1106', 'text-embedding-3-small', 'text-embedding-3-large', 'gpt-3.5-turbo-0125', 'gpt-4-turbo', 'gpt-4-turbo-2024-04-09', 'gpt-4o', 'gpt-4o-2024-05-13', 'gpt-4o-mini-2024-07-18', 'gpt-4o-mini', 'gpt-4o-2024-08-06', 'omni-moderation-latest', 'omni-moderation-2024-09-26', 'o1-2024-12-17', 'o1', 'o3-mini', 'o3-mini-2025-01-31', 'gpt-4o-2024-11-20', 'gpt-4o-mini-search-preview-2025-03-11', 'gpt-4o-mini-search-preview', 'gpt-4o-transcribe', 'gpt-4o-mini-transcribe', 'o1-pro-2025-03-19', 'o1-pro', 'gpt-4o-mini-tts', 'o3-2025-04-16', 'o4-mini-2025-04-16', 'o3', 'o4-mini', 'gpt-4.1-2025-04-14', 'gpt-4.1', 'gpt-4.1-mini-2025-04-14', 'gpt-4.1-mini', 'gpt-4.1-nano-2025-04-14', 'gpt-4.1-nano', 'gpt-image-1', 'o4-mini-deep-research', 'gpt-4o-transcribe-diarize', 'o4-mini-deep-research-2025-06-26', 'gpt-5-chat-latest', 'gpt-5-2025-08-07', 'gpt-5', 'gpt-5-mini-2025-08-07', 'gpt-5-mini', 'gpt-5-nano-2025-08-07', 'gpt-5-nano', 'gpt-audio-2025-08-28', 'gpt-realtime', 'gpt-realtime-2025-08-28', 'gpt-audio', 'gpt-5-codex', 'gpt-image-1-mini', 'gpt-5-pro-2025-10-06', 'gpt-5-pro', 'gpt-audio-mini', 'gpt-audio-mini-2025-10-06', 'gpt-5-search-api', 'gpt-realtime-mini', 'gpt-realtime-mini-2025-10-06', 'sora-2', 'sora-2-pro', 'gpt-5-search-api-2025-10-14', 'gpt-5.1-chat-latest', 'gpt-5.1-2025-11-13', 'gpt-5.1', 'gpt-5.1-codex', 'gpt-5.1-codex-mini', 'gpt-5.1-codex-max', 'gpt-image-1.5', 'gpt-5.2-2025-12-11', 'gpt-5.2', 'gpt-5.2-pro-2025-12-11', 'gpt-5.2-pro', 'gpt-5.2-chat-latest', 'gpt-4o-mini-transcribe-2025-12-15', 'gpt-4o-mini-transcribe-2025-03-20', 'gpt-4o-mini-tts-2025-03-20', 'gpt-4o-mini-tts-2025-12-15', 'gpt-realtime-mini-2025-12-15', 'gpt-audio-mini-2025-12-15', 'chatgpt-image-latest', 'gpt-5.2-codex', 'gpt-5.3-codex', 'gpt-realtime-1.5', 'gpt-audio-1.5', 'gpt-4o-search-preview', 'gpt-4o-search-preview-2025-03-11', 'gpt-5.3-chat-latest', 'gpt-5.4-2026-03-05', 'gpt-5.4-pro', 'gpt-5.4-pro-2026-03-05', 'gpt-5.4', 'gpt-5.4-nano-2026-03-17', 'gpt-5.4-nano', 'gpt-5.4-mini-2026-03-17', 'gpt-5.4-mini', 'gpt-image-2', 'gpt-image-2-2026-04-21', 'gpt-5.5', 'gpt-5.5-2026-04-23', 'gpt-5.5-pro', 'gpt-5.5-pro-2026-04-23', 'chat-latest', 'gpt-realtime-translate', 'gpt-realtime-2', 'gpt-realtime-whisper']
Anthropic models: ['claude-fable-5', 'claude-opus-4-8', 'claude-opus-4-7', 'claude-sonnet-4-6', 'claude-opus-4-6', 'claude-opus-4-5-20251101', 'claude-haiku-4-5-20251001', 'claude-sonnet-4-5-20250929', 'claude-opus-4-1-20250805']
Perplexity models: ['sonar', 'sonar-pro', 'sonar-deep-research', 'sonar-reasoning', 'sonar-reasoning-pro']

=== Get OpenAI Models ===
OpenAI models: ['text-embedding-ada-002', 'whisper-1', 'gpt-3.5-turbo', 'tts-1', 'gpt-3.5-turbo-16k', 'gpt-4-0613', 'gpt-4', 'davinci-002', 'babbage-002', 'gpt-3.5-turbo-instruct', 'gpt-3.5-turbo-instruct-0914', 'gpt-3.5-turbo-1106', 'tts-1-hd', 'tts-1-1106', 'tts-1-hd-1106', 'text-embedding-3-small', 'text-embedding-3-large', 'gpt-3.5-turbo-0125', 'gpt-4-turbo', 'gpt-4-turbo-2024-04-09', 'gpt-4o', 'gpt-4o-2024-05-13', 'gpt-4o-mini-2024-07-18', 'gpt-4o-mini', 'gpt-4o-2024-08-06', 'omni-moderation-latest', 'omni-moderation-2024-09-26', 'o1-2024-12-17', 'o1', 'o3-mini', 'o3-mini-2025-01-31', 'gpt-4o-2024-11-20', 'gpt-4o-mini-search-preview-2025-03-11', 'gpt-4o-mini-search-preview', 'gpt-4o-transcribe', 'gpt-4o-mini-transcribe', 'o1-pro-2025-03-19', 'o1-pro', 'gpt-4o-mini-tts', 'o3-2025-04-16', 'o4-mini-2025-04-16', 'o3', 'o4-mini', 'gpt-4.1-2025-04-14', 'gpt-4.1', 'gpt-4.1-mini-2025-04-14', 'gpt-4.1-mini', 'gpt-4.1-nano-2025-04-14', 'gpt-4.1-nano', 'gpt-image-1', 'o4-mini-deep-research', 'gpt-4o-transcribe-diarize', 'o4-mini-deep-research-2025-06-26', 'gpt-5-chat-latest', 'gpt-5-2025-08-07', 'gpt-5', 'gpt-5-mini-2025-08-07', 'gpt-5-mini', 'gpt-5-nano-2025-08-07', 'gpt-5-nano', 'gpt-audio-2025-08-28', 'gpt-realtime', 'gpt-realtime-2025-08-28', 'gpt-audio', 'gpt-5-codex', 'gpt-image-1-mini', 'gpt-5-pro-2025-10-06', 'gpt-5-pro', 'gpt-audio-mini', 'gpt-audio-mini-2025-10-06', 'gpt-5-search-api', 'gpt-realtime-mini', 'gpt-realtime-mini-2025-10-06', 'sora-2', 'sora-2-pro', 'gpt-5-search-api-2025-10-14', 'gpt-5.1-chat-latest', 'gpt-5.1-2025-11-13', 'gpt-5.1', 'gpt-5.1-codex', 'gpt-5.1-codex-mini', 'gpt-5.1-codex-max', 'gpt-image-1.5', 'gpt-5.2-2025-12-11', 'gpt-5.2', 'gpt-5.2-pro-2025-12-11', 'gpt-5.2-pro', 'gpt-5.2-chat-latest', 'gpt-4o-mini-transcribe-2025-12-15', 'gpt-4o-mini-transcribe-2025-03-20', 'gpt-4o-mini-tts-2025-03-20', 'gpt-4o-mini-tts-2025-12-15', 'gpt-realtime-mini-2025-12-15', 'gpt-audio-mini-2025-12-15', 'chatgpt-image-latest', 'gpt-5.2-codex', 'gpt-5.3-codex', 'gpt-realtime-1.5', 'gpt-audio-1.5', 'gpt-4o-search-preview', 'gpt-4o-search-preview-2025-03-11', 'gpt-5.3-chat-latest', 'gpt-5.4-2026-03-05', 'gpt-5.4-pro', 'gpt-5.4-pro-2026-03-05', 'gpt-5.4', 'gpt-5.4-nano-2026-03-17', 'gpt-5.4-nano', 'gpt-5.4-mini-2026-03-17', 'gpt-5.4-mini', 'gpt-image-2', 'gpt-image-2-2026-04-21', 'gpt-5.5', 'gpt-5.5-2026-04-23', 'gpt-5.5-pro', 'gpt-5.5-pro-2026-04-23', 'chat-latest', 'gpt-realtime-translate', 'gpt-realtime-2', 'gpt-realtime-whisper']

=== Get Anthropic Models ===
Anthropic models: ['claude-fable-5', 'claude-opus-4-8', 'claude-opus-4-7', 'claude-sonnet-4-6', 'claude-opus-4-6', 'claude-opus-4-5-20251101', 'claude-haiku-4-5-20251001', 'claude-sonnet-4-5-20250929', 'claude-opus-4-1-20250805']

✓ Chat Models API example complete
✓ Client created

=== Create User Function ===
Created user function with ID: LJTxWUMlbYnNlAiDSrPvLcaH7GsyztWtafldhqMr_XAyz8Sg6SsJA6R7UUKuVHRUOOtXwWbEBswxMs9CdNmCIA

=== Get User Function ===
Retrieved: get_active_users_py - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 14 user functions:
  - get_high_scoring_active_users: Get High Scoring Active Users
  - get_verified_user: Get verified and validated user
  - swr_user: SWR pattern for user data (KV-based)
  - get_active_users_updated: Get Active Users (Updated)
  - get_active_users_py: Get Active Users
  - get_users_by_status: Get Users By Status
  - get_user_wrapper: Wrapper that calls fetch_user
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_user: Fetch user by code
  - fetch_slim_user: Validate and slim down user
  - validate_user: Check if user exists
  - get_active_users_updated: Get Active Users (Updated)
  - cache_api_call_py: Cache External API Call
  - get_active_users_paginated: Get Active Users (Paginated)

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
