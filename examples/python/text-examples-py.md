make test-examples-python
📦 Ensuring Python example dependencies in .venv...
🧪 Running Python examples (direct HTTP/WebSocket)...
=== Simple CRUD Operations (Direct HTTP) ===

✓ Authentication successful

=== Insert Document ===
Inserted: {'id': 'nhp0Gkid2WiE8eUo5RBE9Id-rDfsOUtTlSGhJIeLDMeKh013lnN0myQlPNHDDDW9vX4RBHuBjOe8NEKfQ3HXrw'}

=== Find by ID ===
Found: {'name': {'type': 'String', 'value': 'Test Record'}, 'active': {'type': 'Boolean', 'value': True}, 'value': {'value': 42, 'type': 'Integer'}, 'id': 'nhp0Gkid2WiE8eUo5RBE9Id-rDfsOUtTlSGhJIeLDMeKh013lnN0myQlPNHDDDW9vX4RBHuBjOe8NEKfQ3HXrw'}

=== Find with Query ===
Found documents: [{'id': 'nhp0Gkid2WiE8eUo5RBE9Id-rDfsOUtTlSGhJIeLDMeKh013lnN0myQlPNHDDDW9vX4RBHuBjOe8NEKfQ3HXrw', 'name': {'type': 'String', 'value': 'Test Record'}, 'value': {'value': 42, 'type': 'Integer'}, 'active': {'value': True, 'type': 'Boolean'}}]

=== Update Document ===
Updated: {'value': {'type': 'Integer', 'value': 100}, 'id': 'nhp0Gkid2WiE8eUo5RBE9Id-rDfsOUtTlSGhJIeLDMeKh013lnN0myQlPNHDDDW9vX4RBHuBjOe8NEKfQ3HXrw', 'active': {'value': True, 'type': 'Boolean'}, 'name': {'value': 'Updated Record', 'type': 'String'}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: lSfJX0_6nppbwdFi5dS5H8c0uBWMcsD9PiPCBFZWEUlpRsH9FnNbGZCOPvCM_ZUlGosN6klFTTQiglHNkTKJiA

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
          "type": "String",
          "value": "WebSocket Test Record"
        },
        "id": "lSfJX0_6nppbwdFi5dS5H8c0uBWMcsD9PiPCBFZWEUlpRsH9FnNbGZCOPvCM_ZUlGosN6klFTTQiglHNkTKJiA",
        "active": {
          "type": "Boolean",
          "value": true
        }
      }
    ]
  },
  "messageId": "1061086415"
}
✓ Retrieved 1 record via WebSocket

✓ WebSocket example completed successfully
🚀 ekoDB Functions Example (Python/HTTP)

✓ Authentication successful

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Function saved: pVnRL31QzPd7zsvJzN7erZ2fhrDFa2X3pYB6AaxvxcR1udwI9WxseJBV5bVJwMNX8B6uOMiqgucZBGR6PMT6bw
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Function saved: dvA1ubouDW8Ld2Ic3-d7tgOXUtB9wdCYVUs9ExvHwkD56dGqXPvS7rMykCUJtlyaSPfGZlADWXyejGwMIw5oJQ
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: tj9tLhi38FwTL-Z8So_RD0AfUSXeW6bYJP1h1P71jIztHYllRn1JB-2mwSOBHb_JwdqJhEp9bbx9J66OyB665w
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {'max_score': {'type': 'Integer', 'value': 90}, 'count': {'type': 'Integer', 'value': 5}, 'avg_score': {'value': 50.0, 'type': 'Float'}, 'status': {'value': 'inactive', 'type': 'String'}}
   {'max_score': {'value': 100, 'type': 'Integer'}, 'avg_score': {'type': 'Float', 'value': 60.0}, 'count': {'value': 5, 'type': 'Integer'}, 'status': {'value': 'active', 'type': 'String'}}

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
kv_operations:direct:py:cache:product:1: {'value': {'price': 29.99, 'name': 'Product 1'}, 'type': 'Object'}
kv_operations:direct:py:cache:product:2: {'value': {'name': 'Product 2', 'price': 39.989999999999995}, 'type': 'Object'}
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
Collection created with first record: 9y74vt7USrWa5ALTrOJzH2DiPEeVyTI7wbV3ELnG1FmqKR62NrKAINFtptTwxU6kRnsWBuPmh2xY4ugTOWUwfg

=== List Collections ===
Total collections: 13
Sample collections: ['demo_collection', 'chat_agent_configs__ek0_testing', 'chat_goal_templates__ek0_testing', 'audit__ek0_testing', 'chat_tasks__ek0_testing']

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
Created Alice: $1000 - ID: Bgij-8gqXrm0-WoS500pxQkKYMSYU-Wpqst48srlM7R5udMgfJKbYaiBgrRqh1mA9iTu62NEInTwOwLJKCSLfQ
Created Bob: $500 - ID: -4aEiqpSYHZwS4hX0n8ySv17E15_Gu1rgy2RENcO5VVQDc-ZSOxDzCGd-u-S118mqwLkauxPL66D8OPuYbYGoQ

=== Example 1: Begin Transaction ===
Transaction ID: a8bcf720-20df-41c9-befc-b16cbd7d10e6

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
New transaction: a35a8a77-ae48-4088-ae39-7642f007dbbb
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'type': 'Integer', 'value': 700}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: rMuDGt9ozEnSlUDrnNTA-NgDZjZznG2hfjS4nYEtO08PeORYC2DQWE8qkaKHcbO8_R4jk7SI21Ly0WCVHc6p1Q
Created Bob: $500 - ID: 1NHE_USjiJ1HUd5U7SxxEbmOss96I_YNbHb-C1VCYPWYkI95vZZieLQ75dNOXPArh6Ifwcru6CX3KBIxhEwSHQ

=== Example 1: Begin Transaction ===
Transaction ID: 4f90c0fb-ace2-4731-a4e4-cff4333bbe7b

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
Created Alice: $1000 - ID: hruEE7vyJA89iYAPPV6Nl-3WSkiud9FLCLsTdCkhpPDhm4uVhKfiRvyXSyKLr-p_XMKHLGJvIDXzakl5kfHL1A
Created Bob: $500 - ID: Xuk6uEItn0YBWW9Nz4TJV5a99SdsB4m1jEnmpAxg3XDQmoDCqVy2Oy0Tna6c34u7e4M-lOS6FBEmvTn8-75NGQ

=== Example 1: Begin Transaction ===
Transaction ID: da0e223e-b408-4d63-bc30-440bb59d5be7

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
New transaction: c0dfe708-7e23-418a-9d24-9c398b26a237
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'value': 700, 'type': 'Integer'}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: -EKWcWUIgjW-VL2r7jQEpvWCJBtXCdL8hehN4lD96Dk987Lf6AtT3rhCR8DEpP3Cb6-_kMFF079mUAfcA6nR2Q
Created Bob: $500 - ID: fhca_h537Gvmiihr7zqKXOqgymbLH7Xf0vXlC5VjINiM5yekxfk1z9QNewLxCW-REUH3QcUgcuI60H3PwZjjqg

=== Example 1: Begin Transaction ===
Transaction ID: 5a7ad27e-de07-4749-b0b2-ef3ce105e3c8

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
Created Alice: $1000 - ID: tWfQe_KfAMIOSTdA5wetxZGheEXj1iKeu6Mse7y4OeSwupkX0eFZNfwAKcAEaHsyZ-SdUiA9x2Jpwdey8pQv5w
Created Bob: $500 - ID: F1LhkxWjdkFOjXesiIyMDpsfR8qcH4Umc0ZE-eRsT5eX2Ve2KnFQbo8BKG7gafdy0SkRH1zKs6R1ddKFss76rg

=== Example 1: Begin Transaction ===
Transaction ID: c89c99fb-340c-495c-a080-5dbf26ecca10

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
New transaction: 35c7439c-7344-4e36-bb73-15f7701e34b5
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'type': 'Integer', 'value': 700}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: y_ry2u5D4AUqIdsoVoQnyS3bj5y5bkdBEXz78hNndUWfR0ie3xwM8XoZ-k1meKGOax_6loOnPN8XRKmkdXbErQ
Created Bob: $500 - ID: wg5oz6eYBxxYukK0zNjPeU58lmmRkm66t1ThQcrJGAmm5q2467P9lPxJS9PvL1OT_wxyrZ5x1PtHOdxCsYLJ0A

=== Example 1: Begin Transaction ===
Transaction ID: 25b6b494-f7b3-404a-b726-292d8de7222e

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
   ✅ Function saved: MmTAAfTxGZvWyDnB0yyfw-iHVtpjEoUYI9arXj4xHtvsX8pCPJE2_3_KPxk7TqYNn8MZA8P5jfnjp89g751YEg

2️⃣ Calling function (Insert + Verify)...
   ✅ function executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: QxDkLGDaXd4fSMlblHndqFIo5Q0irDMkJacCC9Kz9zjisjaM9_AO4onJnl74enHsk99fjWZz9qRGn0tzY8csPg
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}
   📋 Email: {'type': 'String', 'value': 'alice@example.com'}
   📋 Status: {'value': 'pending', 'type': 'String'}
   📋 Credits: {'value': 0, 'type': 'Integer'}

============================================================
📝 function 2: Query + Update + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: jBA0D-SSzeBUVlNtSHiwrmdHE8SQCqGo1Un3ijS1sOTSdRGUKHeOWjuqfEu46vEpkGAOmpwKpBpRvwHVbbRDYw

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
   ✅ Function saved: hu1T_pcX4WD8VJgUo7B57yeRSs4khrRabRgrLMB_QS8YwaSPO4x0CkPOKpYxXsclUAaxkXmtIJOPaH5-fbDh0A

2️⃣ Calling function (Query + Update Credits + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {'value': 100, 'type': 'Integer'}
   📋 Status: {'type': 'String', 'value': 'active'}
   📋 Name: {'value': 'Alice Smith', 'type': 'String'}

============================================================
📝 function 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: QKwGoOr4ZJdzDW-WXpjKExmSCzj2lD_4x2WO3m2OSC8HhnX9SVo-9xrgiM4OCldutqebo9QGMUJ2zXiaFmv_dw

2️⃣ Calling function (Query + Delete + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: MmTAAfTxGZvWyDnB0yyf...
   ✅ Deleted script: jBA0D-SSzeBUVlNtSHiw...
   ✅ Deleted script: hu1T_pcX4WD8VJgUo7B5...
   ✅ Deleted script: QKwGoOr4ZJdzDW-WXpjK...
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
  Output: Document ID = ECY3CGWexpBkXz_EVkW_Kor_nNEaZsKXwF863TxJV-pL8EZU9aftVO7ULl1aHdbxMOymcnFTnbzHOBLDxBNLQw
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: find_by_id(ECY3CGWexpBkXz_EVkW_Kor_nNEaZsKXwF863TxJV-pL8EZU9aftVO7ULl1aHdbxMOymcnFTnbzHOBLDxBNLQw)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: find_by_id(ECY3CGWexpBkXz_EVkW_Kor_nNEaZsKXwF863TxJV-pL8EZU9aftVO7ULl1aHdbxMOymcnFTnbzHOBLDxBNLQw)
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
  Output: Document ID = mAFne1i_4KVaC4ntIfdmfO1LXzRfugfT8izkLCT8TqtjmgPvme-vZSq2eiywpuB0aY8FtEejVTxuP5ePfhU0mA
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: find_by_id(mAFne1i_4KVaC4ntIfdmfO1LXzRfugfT8izkLCT8TqtjmgPvme-vZSq2eiywpuB0aY8FtEejVTxuP5ePfhU0mA)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: find_by_id(mAFne1i_4KVaC4ntIfdmfO1LXzRfugfT8izkLCT8TqtjmgPvme-vZSq2eiywpuB0aY8FtEejVTxuP5ePfhU0mA)
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
    Finished `release` profile [optimized] target(s) in 0.17s
📦 Built wheel for abi3 Python ≥ 3.8 to ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.27.0-cp38-abi3-macosx_11_0_arm64.whl
📦 Installing Python wheel into .venv...
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.27.0-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.27.0
    Uninstalling ekodb_client-0.27.0:
      Successfully uninstalled ekodb_client-0.27.0
Successfully installed ekodb-client-0.27.0
🧪 Ensuring test dependencies (pytest) in .venv...
✅ Python client package built and installed!
=== ekoDB Advanced CRUD Example (Python) ===

--- insert ---
Inserted: fMNib1n0TJVuvDbjxL81MQfdGJX-AroycDs0uhhQqTnmBfJ3jXrsQ_sgR4-SXXoKSlhf6hgBA2j730iK1nfxlw

--- update_with_action (increment) ---
After increment count by 5: {'id': 'fMNib1n0TJVuvDbjxL81MQfdGJX-AroycDs0uhhQqTnmBfJ3jXrsQ_sgR4-SXXoKSlhf6hgBA2j730iK1nfxlw', 'count': {'type': 'Integer', 'value': 15}, 'tags': {'value': ['initial'], 'type': 'Array'}, 'score': {'type': 'Float', 'value': 50.0}, 'name': {'type': 'String', 'value': 'Counter Record'}}

--- update_with_action (decrement) ---
After decrement score by 10: {'id': 'fMNib1n0TJVuvDbjxL81MQfdGJX-AroycDs0uhhQqTnmBfJ3jXrsQ_sgR4-SXXoKSlhf6hgBA2j730iK1nfxlw', 'name': {'type': 'String', 'value': 'Counter Record'}, 'score': {'type': 'Float', 'value': 40.0}, 'tags': {'type': 'Array', 'value': ['initial']}, 'count': {'type': 'Integer', 'value': 15}}

--- update_with_action_sequence ---
After action sequence: {'name': {'type': 'String', 'value': 'Counter Record'}, 'count': {'type': 'Integer', 'value': 115}, 'score': {'type': 'Float', 'value': 40.0}, 'id': 'fMNib1n0TJVuvDbjxL81MQfdGJX-AroycDs0uhhQqTnmBfJ3jXrsQ_sgR4-SXXoKSlhf6hgBA2j730iK1nfxlw', 'tags': {'type': 'Array', 'value': ['initial', 'sequenced']}}

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
✓ Created session: sWTRtLnlMDIIeAR8ZhJP4C6nIAEFyrshZRb-AlYv8DG7ZExyh4y9yy7btb-jlFlRgUPTxoyJ17hO5t_p4qNK7g

=== Sending Initial Message ===
✓ Message sent
  Response: It seems there are currently no products available in the database, based on the query for relevant records. Would you like to add products or check something else?

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['context_snippets', 'llm_model', 'id', 'llm_provider', 'chat_id', 'content', 'token_usage', 'created_at', 'role', 'updated_at'])
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
✓ Created second session: kQ5BBk5EsbJjs3EBFfQXm3A0sy5kdAbvfIuYn6xJHE82ZwcuKKfdF_X5LsxU1ZqsVd29il0LNTqj1v-Q8slF3w
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 7

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 6

=== Cleanup ===
✓ Deleted chat session: kQ5BBk5EsbJjs3EBFfQXm3A0sy5kdAbvfIuYn6xJHE82ZwcuKKfdF_X5LsxU1ZqsVd29il0LNTqj1v-Q8slF3w
✓ Deleted chat session: sWTRtLnlMDIIeAR8ZhJP4C6nIAEFyrshZRb-AlYv8DG7ZExyh4y9yy7btb-jlFlRgUPTxoyJ17hO5t_p4qNK7g
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: 9j2uxJBCNy_OrB2g6d2mK8zwIWIy-_z3XA7mIF6JGTQyA8aJpeO6jLOC5qegAaoJ7FaGdkHSn85JY2jr3WddZA

=== Sending Chat Message ===
Message ID: ZlYzlm4SBZoBysox2ef2VZK-D3h14z1p_7sibo1cf2rQP6o3LqH0bINUxAzgFHTT6l6LL0q-AOX-N4_HF8BWkQ

=== AI Response ===
Here are the available products and their prices:

1. **ekoDB Pro**
   - Price: $299
   - Description: Enterprise edition product with advanced features.

2. **ekoDB Cloud**
   - Price: $499
   - Description: Fully managed cloud database service product.

3. **ekoDB**
   - Price: $99
   - Description: A high-performance database product with AI capabilities.

Execution Time: 3044ms

=== Token Usage ===
Prompt tokens: 3413
Completion tokens: 90
Total tokens: 3503

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Message Stream (SSE) Example (Python) ===

Created session: uP3Etkc42s1kGvmSHFk6GJFiP4UMqhYZX26GHpVYa0ttaaakaNdRQOQRO5nsHs5vf1PdVy0dlO3gBepS8tsm8Q

Streaming response for: 'What is ekoDB?'

{"__progress":"received"}{"__progress":"generating","model":"gpt-4o-mini","provider":"openai"}ekoDB is a high-performance, scalable NoSQL database designed for use in distributed systems. It is built to provide efficient storage and retrieval of large volumes of unstructured or semi-structured data, making it suitable for applications that require quick access to data across multiple nodes.

Key features of ekoDB typically include:

1. **Scalability**: It allows for easy scaling horizontally by adding more servers to accommodate growing amounts of data and higher request loads.

2. **High Availability**: ekoDB often includes mechanisms for replication and fault tolerance, ensuring data remains accessible even in the event of server failures.

3. **Schema Flexibility**: As a NoSQL database, ekoDB enables developers to store data without a fixed schema, making it easier to adapt to changing application requirements.

4. **Performance**: It is optimized for fast data access and can handle high throughput of read and write operations.

5. **Distributed Architecture**: Designed to operate in a distributed environment, ekoDB can efficiently manage data across various geographical locations.

Unlike traditional relational databases, which use tables and structured query languages, ekoDB allows for more flexible data models, catering to applications that need to work with diverse datasets.

It's worth noting that specific features and functionalities can vary depending on the implementation and the version of ekoDB, as well as the context in which it is used, such as specific industries or use cases.

--- Stream complete ---
Message ID: 3-kBZB4SexPkFsgmmNBchdPD555LY79_cEztG9ZC4zm-6EuCeBjTCCOuzWbtkzdxwuehbFYuOEWnHYK8tQujPA
Execution time: 3498ms
Context window: 128000 tokens

✓ Chat message stream example completed
✓ Client created

=== Get All Chat Models ===
OpenAI models: ['o3-2025-04-16', 'gpt-5.1-chat-latest', 'gpt-5.3-chat-latest', 'gpt-5.2-2025-12-11', 'sora-2', 'chatgpt-image-latest', 'gpt-4.1-mini', 'gpt-3.5-turbo', 'gpt-4o-mini', 'gpt-image-2.5-sunburst', 'gpt-image-2.5-flare-2026-09-08', 'gpt-6-luna', 'gpt-5.4-mini-2026-03-17', 'gpt-4-turbo', 'gpt-4.1-nano', 'gpt-image-2', 'gpt-5.4-pro', 'gpt-realtime-mini-2025-12-15', 'gpt-3.5-turbo-16k', 'babbage-002', 'text-embedding-ada-002', 'gpt-4o-mini-tts', 'gpt-4o-2024-08-06', 'o4-mini-2025-04-16', 'gpt-4o-2024-11-20', 'omni-moderation-latest', 'gpt-realtime', 'tts-1-hd', 'gpt-realtime-1.5', 'gpt-realtime-2', 'gpt-5-search-api', 'gpt-5.5-2026-04-23', 'gpt-5.1', 'gpt-5.1-codex', 'gpt-4o', 'o1-2024-12-17', 'gpt-5.2-chat-latest', 'gpt-4o-mini-tts-2025-12-15', 'gpt-image-1', 'davinci-002', 'gpt-image-2-2026-04-21', 'gpt-4o-mini-transcribe', 'o3-mini', 'gpt-audio', 'gpt-5.4-nano', 'o3', 'gpt-5.1-codex-max', 'gpt-realtime-2.1', 'gpt-4.1', 'gpt-5.5', 'gpt-4o-mini-search-preview-2025-03-11', 'text-embedding-3-large', 'gpt-4o-mini-search-preview', 'gpt-4o-2024-05-13', 'gpt-6-sol', 'gpt-5.3-codex', 'gpt-3.5-turbo-instruct', 'sora-2-pro', 'gpt-5', 'gpt-audio-mini-2025-12-15', 'gpt-transcribe', 'o4-mini-deep-research-2025-06-26', 'o1-pro-2025-03-19', 'gpt-4.1-nano-2025-04-14', 'gpt-5-search-api-2025-10-14', 'gpt-4-turbo-2024-04-09', 'gpt-realtime-2.1-mini', 'tts-1', 'gpt-5-mini', 'omni-moderation-2024-09-26', 'gpt-5-mini-2025-08-07', 'gpt-realtime-2025-08-28', 'gpt-5.4-2026-03-05', 'gpt-live-transcribe', 'gpt-3.5-turbo-0125', 'gpt-5-nano', 'gpt-5.6-luna', 'gpt-4', 'gpt-image-2.5-sunburst-2026-09-08', 'whisper-1', 'gpt-5.4-mini', 'gpt-5-pro', 'o1-pro', 'gpt-realtime-whisper', 'gpt-5.2-pro', 'gpt-5-chat-latest', 'gpt-live-1', 'gpt-4o-mini-transcribe-2025-12-15', 'gpt-5-2025-08-07', 'tts-1-1106', 'gpt-5.5-pro', 'gpt-5-pro-2025-10-06', 'chat-latest', 'gpt-5.2-pro-2025-12-11', 'gpt-4o-mini-transcribe-2025-03-20', 'gpt-audio-2025-08-28', 'gpt-5.4-pro-2026-03-05', 'gpt-realtime-translate', 'gpt-5.6-sol', 'gpt-audio-mini-2025-10-06', 'gpt-5.5-pro-2026-04-23', 'tts-1-hd-1106', 'gpt-realtime-mini', 'gpt-5.4', 'gpt-4o-transcribe-diarize', 'gpt-5-codex', 'gpt-image-1-mini', 'gpt-image-1.5', 'gpt-5.1-2025-11-13', 'o4-mini-deep-research', 'gpt-4o-search-preview-2025-03-11', 'o3-mini-2025-01-31', 'gpt-4-0613', 'gpt-4o-search-preview', 'gpt-5.6-terra', 'text-embedding-3-small', 'gpt-audio-1.5', 'gpt-3.5-turbo-instruct-0914', 'gpt-6-astra', 'gpt-5.2-codex', 'gpt-3.5-turbo-1106', 'gpt-5-nano-2025-08-07', 'gpt-audio-mini', 'gpt-4o-mini-tts-2025-03-20', 'gpt-5.2', 'gpt-4.1-mini-2025-04-14', 'gpt-4.1-2025-04-14', 'gpt-4o-mini-2024-07-18', 'o4-mini', 'o1', 'gpt-5.4-nano-2026-03-17', 'gpt-5.1-codex-mini', 'gpt-4o-transcribe', 'gpt-image-2.5-flare', 'gpt-6.1-sol']
Anthropic models: ['claude-haiku-5-5', 'claude-sonnet-5-5', 'claude-opus-5-5', 'claude-fable-5-1', 'claude-opus-5', 'claude-sonnet-5', 'claude-fable-5', 'claude-opus-4-8', 'claude-opus-4-7', 'claude-sonnet-4-6', 'claude-opus-4-6', 'claude-opus-4-5-20251101', 'claude-haiku-4-5-20251001', 'claude-sonnet-4-5-20250929']
Perplexity models: []
Gemini models: []

=== Get OpenAI Models ===
OpenAI models: ['o3-2025-04-16', 'gpt-5.1-chat-latest', 'gpt-5.3-chat-latest', 'gpt-5.2-2025-12-11', 'sora-2', 'chatgpt-image-latest', 'gpt-4.1-mini', 'gpt-3.5-turbo', 'gpt-4o-mini', 'gpt-image-2.5-sunburst', 'gpt-image-2.5-flare-2026-09-08', 'gpt-6-luna', 'gpt-5.4-mini-2026-03-17', 'gpt-4-turbo', 'gpt-4.1-nano', 'gpt-image-2', 'gpt-5.4-pro', 'gpt-realtime-mini-2025-12-15', 'gpt-3.5-turbo-16k', 'babbage-002', 'text-embedding-ada-002', 'gpt-4o-mini-tts', 'gpt-4o-2024-08-06', 'o4-mini-2025-04-16', 'gpt-4o-2024-11-20', 'omni-moderation-latest', 'gpt-realtime', 'tts-1-hd', 'gpt-realtime-1.5', 'gpt-realtime-2', 'gpt-5-search-api', 'gpt-5.5-2026-04-23', 'gpt-5.1', 'gpt-5.1-codex', 'gpt-4o', 'o1-2024-12-17', 'gpt-5.2-chat-latest', 'gpt-4o-mini-tts-2025-12-15', 'gpt-image-1', 'davinci-002', 'gpt-image-2-2026-04-21', 'gpt-4o-mini-transcribe', 'o3-mini', 'gpt-audio', 'gpt-5.4-nano', 'o3', 'gpt-5.1-codex-max', 'gpt-realtime-2.1', 'gpt-4.1', 'gpt-5.5', 'gpt-4o-mini-search-preview-2025-03-11', 'text-embedding-3-large', 'gpt-4o-mini-search-preview', 'gpt-4o-2024-05-13', 'gpt-6-sol', 'gpt-5.3-codex', 'gpt-3.5-turbo-instruct', 'sora-2-pro', 'gpt-5', 'gpt-audio-mini-2025-12-15', 'gpt-transcribe', 'o4-mini-deep-research-2025-06-26', 'o1-pro-2025-03-19', 'gpt-4.1-nano-2025-04-14', 'gpt-5-search-api-2025-10-14', 'gpt-4-turbo-2024-04-09', 'gpt-realtime-2.1-mini', 'tts-1', 'gpt-5-mini', 'omni-moderation-2024-09-26', 'gpt-5-mini-2025-08-07', 'gpt-realtime-2025-08-28', 'gpt-5.4-2026-03-05', 'gpt-live-transcribe', 'gpt-3.5-turbo-0125', 'gpt-5-nano', 'gpt-5.6-luna', 'gpt-4', 'gpt-image-2.5-sunburst-2026-09-08', 'whisper-1', 'gpt-5.4-mini', 'gpt-5-pro', 'o1-pro', 'gpt-realtime-whisper', 'gpt-5.2-pro', 'gpt-5-chat-latest', 'gpt-live-1', 'gpt-4o-mini-transcribe-2025-12-15', 'gpt-5-2025-08-07', 'tts-1-1106', 'gpt-5.5-pro', 'gpt-5-pro-2025-10-06', 'chat-latest', 'gpt-5.2-pro-2025-12-11', 'gpt-4o-mini-transcribe-2025-03-20', 'gpt-audio-2025-08-28', 'gpt-5.4-pro-2026-03-05', 'gpt-realtime-translate', 'gpt-5.6-sol', 'gpt-audio-mini-2025-10-06', 'gpt-5.5-pro-2026-04-23', 'tts-1-hd-1106', 'gpt-realtime-mini', 'gpt-5.4', 'gpt-4o-transcribe-diarize', 'gpt-5-codex', 'gpt-image-1-mini', 'gpt-image-1.5', 'gpt-5.1-2025-11-13', 'o4-mini-deep-research', 'gpt-4o-search-preview-2025-03-11', 'o3-mini-2025-01-31', 'gpt-4-0613', 'gpt-4o-search-preview', 'gpt-5.6-terra', 'text-embedding-3-small', 'gpt-audio-1.5', 'gpt-3.5-turbo-instruct-0914', 'gpt-6-astra', 'gpt-5.2-codex', 'gpt-3.5-turbo-1106', 'gpt-5-nano-2025-08-07', 'gpt-audio-mini', 'gpt-4o-mini-tts-2025-03-20', 'gpt-5.2', 'gpt-4.1-mini-2025-04-14', 'gpt-4.1-2025-04-14', 'gpt-4o-mini-2024-07-18', 'o4-mini', 'o1', 'gpt-5.4-nano-2026-03-17', 'gpt-5.1-codex-mini', 'gpt-4o-transcribe', 'gpt-image-2.5-flare', 'gpt-6.1-sol']

=== Get Anthropic Models ===
Anthropic models: ['claude-haiku-5-5', 'claude-sonnet-5-5', 'claude-opus-5-5', 'claude-fable-5-1', 'claude-opus-5', 'claude-sonnet-5', 'claude-fable-5', 'claude-opus-4-8', 'claude-opus-4-7', 'claude-sonnet-4-6', 'claude-opus-4-6', 'claude-opus-4-5-20251101', 'claude-haiku-4-5-20251001', 'claude-sonnet-4-5-20250929']

✓ Chat Models API example complete
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: fJVCAPvSlSUqLopkLDrLuNhSDSg8c0-kMEn0_YN5RuTbfr8Hy7lCYlyUjfxCmlbVJ1w0k8iNvynluyac-NOCiA

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name**: ekoDB
  - **Description**: A high-performance database product
  - **Price**: $99

If you need more information or assistance, feel free to ask!

✓ Message 2 sent
  Response: The price of the product **ekoDB** is **$99**.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: Z5yx30TLt4ELsrQidkM4tS2c2QNyB8NAms0XAyJJsm8aJH1hDhV_H4f_tD-x1Iy3RcnOoXpzqULH3gkIzbcd4w
  Parent: fJVCAPvSlSUqLopkLDrLuNhSDSg8c0-kMEn0_YN5RuTbfr8Hy7lCYlyUjfxCmlbVJ1w0k8iNvynluyac-NOCiA

=== Listing Sessions ===
✓ Found 2 sessions
  Session 1: Z5yx30TLt4ELsrQidkM4tS2c2QNyB8NAms0XAyJJsm8aJH1hDhV_H4f_tD-x1Iy3RcnOoXpzqULH3gkIzbcd4w (Untitled)
  Session 2: fJVCAPvSlSUqLopkLDrLuNhSDSg8c0-kMEn0_YN5RuTbfr8Hy7lCYlyUjfxCmlbVJ1w0k8iNvynluyac-NOCiA (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: Z5yx30TLt4ELsrQidkM4tS2c2QNyB8NAms0XAyJJsm8aJH1hDhV_H4f_tD-x1Iy3RcnOoXpzqULH3gkIzbcd4w

=== Cleanup ===
✓ Deleted sessions and collection

✓ All session management operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: "Sn-2UnIpDKBXonfCDIW0A6kuY1WdCio0g1em0gBrFZ16-dTnpD3ADg52-F5BNBClSmSMjJ3vCDnDJ1_sLDRiIA"

=== List Collections ===
Total collections: 13
Sample collections: ['chat_agent_configs__ek0_testing', 'chat_goal_templates__ek0_testing', 'audit__ek0_testing', 'chat_tasks__ek0_testing', 'agent_function_versions__ek0_testing']

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
✓ Created record with plain dict: {'id': 'ge8QCZbjvZA8ounlIMfITOpIT-R8SGO393aqU7oVPLMnRTRQb9Eai6rxlXo1n4vBwmiZ0m_LUc6ApeotD20gHQ'}

=== Upsert Operation ===
✓ Upsert (update existing record): ge8QCZbjvZA8ounlIMfITOpIT-R8SGO393aqU7oVPLMnRTRQb9Eai6rxlXo1n4vBwmiZ0m_LUc6ApeotD20gHQ
✓ Inserted second record: AGAuT5-tZFTcEBKgskowqCEP17EskZr5UQ-cj8TFEwQy8hmfWp5M90rYAiM2py6YY3RVJ0HoypY3ncM8uYFQBQ
✓ Upsert (update second record): AGAuT5-tZFTcEBKgskowqCEP17EskZr5UQ-cj8TFEwQy8hmfWp5M90rYAiM2py6YY3RVJ0HoypY3ncM8uYFQBQ

=== Find One Operation ===
✓ Found user by email: {'email': {'value': 'alice.j@newdomain.com', 'type': 'String'}, 'age': {'type': 'Integer', 'value': 29}, 'name': {'type': 'String', 'value': 'Alice Johnson'}, 'active': {'type': 'Boolean', 'value': True}, 'id': 'ge8QCZbjvZA8ounlIMfITOpIT-R8SGO393aqU7oVPLMnRTRQb9Eai6rxlXo1n4vBwmiZ0m_LUc6ApeotD20gHQ'}
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
✓ Inserted document: TRzRH7DJTKskfBsZNkkXTCgTuwLu1n2Ce4VzWMwYmsgCXYF1WO1mpDnP4Db-M0Q7zAp8EXjgx_7ZBuv7R3F8uA

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: QFi93IXgLCbGYN6IAwfgnxFy5PartSs23GF1ZzUirpkR-sTZAyH69cW_B8lQ1DpJFmtbVdnn0trb_fPTI8xiMw

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
✓ Edge cache script created: nGQtFXhG1XLa_F6YZ5U9ZBsICYij3m5H3HMdiGosAshikMWQcB572jzdAmaaGUn7Lqhqm09mlSWknsC70AtU7A

Call 1: Cache miss (fetches from API)
Response time: 172ms
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "current": {
            "interval": 900,
            "temperature_2m": 13.6,
            "time": "2026-10-08T05:45"
          },
          "current_units": {
            "interval": "seconds",
            "temperature_2m": "\u00b0C",
            "time": "iso8601"
          },
          "elevation": 32.0,
          "generationtime_ms": 0.045180320739746094,
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
Response time: 4ms (38.9x faster!)
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "current": {
            "interval": 900,
            "temperature_2m": 13.6,
            "time": "2026-10-08T05:45"
          },
          "current_units": {
            "interval": "seconds",
            "temperature_2m": "\u00b0C",
            "time": "iso8601"
          },
          "elevation": 32.0,
          "generationtime_ms": 0.045180320739746094,
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
   ⏱️  Duration: 54.1ms
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
   ⏱️  Duration: 1.8ms
   📊 Records: 1
   📦 Data: {
      "value": {
            "type": "Object",
            "value": {
                  "address": {
                        "city": "Gwenborough",
                        "geo": {
                 ...
   🚀 Cache speedup: 29.4x faster!

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

✅ Function saved: J85xQ8UfBWqprHXFjVzopFuoEa0N5dH1_FEtGUq2wFIu4Z2RatWwyoBFSd_WO2RSok8TSnxvtHI9a8VSM1zR8Q
📊 Found 5 active users

📝 Example 2: Parameterized Function

✅ Function saved: duivbzKf_h_skxfJmGMBBnFvXvRedgB0d4uy1_bbGGhKEgMYkCxsAZ36iFXyW8B-v6DnNxdojLEQhTXQbKBLYA
📊 Found 3 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: -rSR_-tGHSL_HnYaSEQvnA06HemyE1W8ICyk_CZ0rO9ARKZby0rqJmOaSCwTgndEUgORoCoV8zW4FrqI-yeFiQ
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

1. **Efficient Similarity Search**: They excel at searching for similar items based on high-dimensional vector representations, making them ideal for applications like image and text retrieval.

2. **High Performance**: Optimized for handling large-scale data, vector databases provide fast query response times even with extensive datasets.

3. **Scalability**: Designed to scale horizontally, they can manage growing amounts of data without significant drops in performance.

4. **Flexible Data Representation**: Support for various data types (text, images, audio) can be converted into vectors, making them versatile for different applications.

5. **Natural Language Processing**: They are particularly useful in NLP tasks where word embeddings and contextual embeddings are utilized for semantic searches.

6. **Integration with Machine Learning**: Seamless integration with AI and ML pipelines for real-time inference and analytics.

7. **Support for Complex Queries**: Ability to perform complex queries beyond simple key-value lookups, incorporating relevance and ranking.

8. **Real-time Data Handling**: Capable of handling real-time data updates, making them suitable for dynamic applications.

9. **Enhanced User Experience**: By providing more accurate and context-aware search results, they improve user engagement and satisfaction.

10. **Open Source Options**: Many vector databases are open source, allowing for customization and community support.

These benefits make vector databases particularly advantageous for machine learning, AI, and data retrieval tasks.
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

✅ Function saved: nDfhaENKPJQwx0yc0AJMY2w6V6_rgXERemQbQhD0pJMMx9STyLPyVznXZMZTpfqk6P9MEg-PsQBCNwiILoArjA
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

✅ Inserted order: _A6zA4GorLSrMzeX0_C12PJuDvz19o0QkYv9GoDSACFwdoZJDZ-pL5stYRlcqGqAvgDWtD0YAypcmiqskht1Eg
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

✅ Inserted order: cuKrV-cb5Gdab8pNGe2ZD9z5Edqtj-g2r3xNJvehMxVAwAQj7Zl3QW2l7cHvUxamQE9qPDH5zdwfQv79BD9rwg
✅ Cached order status
📊 Quick status lookup: {'value': '{"type":"Object","value":{"status":"processing","updated_at":"2026-10-08T05:45:58.957885+00:00"}}'}

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
   2. Database Design Principles (Database)
   3. Natural Language Processing (AI)
   4. Vector Databases Explained (Database)
   5. Getting Started with ekoDB (Database)
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
Created template: Data Migration (id: 4s5kE-fk8rmYsVPOvu5a_QIPhOwqgzTM8nqIN1kY6gGzp0PfmkGCso1xUsDKTFxpeOdJNEBd82rXGlFNIR6ccw)

--- Listing templates ---
Templates: {'count': 1, 'items': [{'description': {'type': 'String', 'value': 'Template for migrating data between schemas'}, 'id': '4s5kE-fk8rmYsVPOvu5a_QIPhOwqgzTM8nqIN1kY6gGzp0PfmkGCso1xUsDKTFxpeOdJNEBd82rXGlFNIR6ccw', 'steps': {'type': 'Array', 'value': [{'description': 'Analyze source schema'}, {'description': 'Create target schema'}, {'description': 'Migrate records'}, {'description': 'Validate results'}]}, 'title': {'type': 'String', 'value': 'Data Migration'}}]}

--- Getting template ---
Fetched: {'type': 'String', 'value': 'Data Migration'}

--- Updating template ---
Updated description: {'type': 'String', 'value': 'Updated: comprehensive data migration workflow'}

--- Deleting template ---
Template deleted successfully

✓ Goal template CRUD example completed
=== ekoDB Goals / Tasks / Agents Integration Example (Python) ===

--- goal_create ---
Created goal: Deploy v3 (id: HOv0x2RqRWGKMyr68wEBQZtG_zAqloHY5-eJUKjEgkelSoziSF1u-YqmlysNSUAPtlPTdqoFf6w-qYcCVEwsAg)

--- goal_list ---
Goals: {'count': 1, 'goals': [{'created_at': '2026-10-08T05:45:59.301219+00:00', 'description': 'Deploy version 3 to production', 'id': 'HOv0x2RqRWGKMyr68wEBQZtG_zAqloHY5-eJUKjEgkelSoziSF1u-YqmlysNSUAPtlPTdqoFf6w-qYcCVEwsAg', 'status': 'pending', 'steps': '[{"description":"Build Docker image","status":"pending"},{"description":"Run migrations","status":"pending"},{"description":"Smoke test","status":"pending"}]', 'title': 'Deploy v3', 'updated_at': '2026-10-08T05:45:59.301219+00:00'}]}

--- goal_get ---
Fetched: {'type': 'String', 'value': 'Deploy v3'}

--- goal_update ---
Updated description: {'type': 'String', 'value': 'Deploy version 3 to production (updated)'}

--- goal_search ---
Search results: {'count': 1, 'items': [{'_score': 13.2, 'created_at': {'type': 'DateTime', 'value': '2026-10-08T05:45:59.301219+00:00'}, 'description': {'type': 'String', 'value': 'Deploy version 3 to production (updated)'}, 'id': 'HOv0x2RqRWGKMyr68wEBQZtG_zAqloHY5-eJUKjEgkelSoziSF1u-YqmlysNSUAPtlPTdqoFf6w-qYcCVEwsAg', 'status': {'type': 'String', 'value': 'pending'}, 'steps': {'type': 'String', 'value': '[{"description":"Build Docker image","status":"pending"},{"description":"Run migrations","status":"pending"},{"description":"Smoke test","status":"pending"}]'}, 'title': {'type': 'String', 'value': 'Deploy v3'}, 'updated_at': {'type': 'DateTime', 'value': '2026-10-08T05:45:59.310952+00:00'}}]}

--- goal_step_start (step 0) ---
Step 0 started: {'created_at': {'type': 'DateTime', 'value': '2026-10-08T05:45:59.301219+00:00'}, 'description': {'type': 'String', 'value': 'Deploy version 3 to production (updated)'}, 'id': 'HOv0x2RqRWGKMyr68wEBQZtG_zAqloHY5-eJUKjEgkelSoziSF1u-YqmlysNSUAPtlPTdqoFf6w-qYcCVEwsAg', 'status': {'type': 'String', 'value': 'in_progress'}, 'steps': {'type': 'String', 'value': '[{"description":"Build Docker image","status":"InProgress"},{"description":"Run migrations","status":"pending"},{"description":"Smoke test","status":"pending"}]'}, 'title': {'type': 'String', 'value': 'Deploy v3'}, 'updated_at': {'type': 'DateTime', 'value': '2026-10-08T05:45:59.317924+00:00'}}

--- goal_step_complete (step 0) ---
Step 0 completed: {'created_at': {'type': 'DateTime', 'value': '2026-10-08T05:45:59.301219+00:00'}, 'description': {'type': 'String', 'value': 'Deploy version 3 to production (updated)'}, 'id': 'HOv0x2RqRWGKMyr68wEBQZtG_zAqloHY5-eJUKjEgkelSoziSF1u-YqmlysNSUAPtlPTdqoFf6w-qYcCVEwsAg', 'status': {'type': 'String', 'value': 'in_progress'}, 'steps': {'type': 'String', 'value': '[{"description":"Build Docker image","result":"Docker image built: sha256:abc123","status":"Completed"},{"description":"Run migrations","status":"pending"},{"description":"Smoke test","status":"pending"}]'}, 'title': {'type': 'String', 'value': 'Deploy v3'}, 'updated_at': {'type': 'DateTime', 'value': '2026-10-08T05:45:59.324254+00:00'}}

--- goal_step_fail (step 1) ---
Step 1 failed: {'created_at': {'type': 'DateTime', 'value': '2026-10-08T05:45:59.301219+00:00'}, 'description': {'type': 'String', 'value': 'Deploy version 3 to production (updated)'}, 'id': 'HOv0x2RqRWGKMyr68wEBQZtG_zAqloHY5-eJUKjEgkelSoziSF1u-YqmlysNSUAPtlPTdqoFf6w-qYcCVEwsAg', 'status': {'type': 'String', 'value': 'in_progress'}, 'steps': {'type': 'String', 'value': '[{"description":"Build Docker image","result":"Docker image built: sha256:abc123","status":"Completed"},{"description":"Run migrations","error":"Migration failed: column already exists","status":"Failed"},{"description":"Smoke test","status":"pending"}]'}, 'title': {'type': 'String', 'value': 'Deploy v3'}, 'updated_at': {'type': 'DateTime', 'value': '2026-10-08T05:45:59.329281+00:00'}}

--- goal_complete ---
Goal completed: {'type': 'String', 'value': 'pending_review'}

--- goal_approve ---
Goal approved: {'type': 'String', 'value': 'in_progress'}

--- goal_reject ---
Goal rejected: {'type': 'String', 'value': 'failed'}

--- task_create ---
Created task: Nightly Backup (id: LOzG2_LunU6so0RZiXIFpD6E6xj0g34ApwiICBUi0ma2ZnONkDsUT7pR1ylCRC24_jJsFCjg6EZbYVZDp17ZOA)

--- task_list ---
Tasks: {'count': 1, 'items': [{'cron': {'type': 'String', 'value': '0 2 * * *'}, 'description': {'type': 'String', 'value': 'Backup all databases nightly'}, 'id': 'LOzG2_LunU6so0RZiXIFpD6E6xj0g34ApwiICBUi0ma2ZnONkDsUT7pR1ylCRC24_jJsFCjg6EZbYVZDp17ZOA', 'max_consecutive_failures': {'type': 'Integer', 'value': 3}, 'name': {'type': 'String', 'value': 'Nightly Backup'}}]}

--- task_get ---
Fetched: {'type': 'String', 'value': 'Nightly Backup'}

--- task_start ---
Task started: {'type': 'String', 'value': 'running'}

--- task_succeed ---
Task succeeded: {'consecutive_failures': {'type': 'Integer', 'value': 0}, 'cron': {'type': 'String', 'value': '0 2 * * *'}, 'description': {'type': 'String', 'value': 'Backup all databases nightly'}, 'id': 'LOzG2_LunU6so0RZiXIFpD6E6xj0g34ApwiICBUi0ma2ZnONkDsUT7pR1ylCRC24_jJsFCjg6EZbYVZDp17ZOA', 'last_error': {'type': 'Null', 'value': None}, 'last_run': {'type': 'DateTime', 'value': '2026-10-08T05:45:59.386354+00:00'}, 'max_consecutive_failures': {'type': 'Integer', 'value': 3}, 'name': {'type': 'String', 'value': 'Nightly Backup'}, 'run_count': {'type': 'Integer', 'value': 1}, 'status': {'type': 'String', 'value': 'active'}, 'updated_at': {'type': 'DateTime', 'value': '2026-10-08T05:45:59.386354+00:00'}}

--- task_pause ---
Task paused: {'type': 'String', 'value': 'paused'}

--- task_resume ---
Task resumed: {'type': 'String', 'value': 'active'}

--- task_fail ---
Task failed: {'consecutive_failures': {'type': 'Integer', 'value': 1}, 'cron': {'type': 'String', 'value': '0 2 * * *'}, 'description': {'type': 'String', 'value': 'Backup all databases nightly'}, 'id': 'LOzG2_LunU6so0RZiXIFpD6E6xj0g34ApwiICBUi0ma2ZnONkDsUT7pR1ylCRC24_jJsFCjg6EZbYVZDp17ZOA', 'last_error': {'type': 'String', 'value': 'Disk full'}, 'last_run': {'type': 'DateTime', 'value': '2026-10-08T05:45:59.412627+00:00'}, 'max_consecutive_failures': {'type': 'Integer', 'value': 3}, 'name': {'type': 'String', 'value': 'Nightly Backup'}, 'next_run': {'type': 'DateTime', 'value': '2026-03-22T02:00:00+00:00'}, 'run_count': {'type': 'Integer', 'value': 1}, 'status': {'type': 'String', 'value': 'active'}, 'updated_at': {'type': 'DateTime', 'value': '2026-10-08T05:45:59.412627+00:00'}}

--- task_due ---
Due tasks: {'count': 0, 'items': []}

--- task_delete ---
Task deleted

--- agent_create ---
Created agent: CodeReviewer (id: orjAWGtByDTrFU8Q4S3F0DPb4IlLntIR2KVpG_8FnIsJ5oQcW2BY10AujhI0SXfjOSbzI7i3TQyEWOzCYKP-dA)

--- agent_list ---
Agents: {'count': 1, 'items': [{'deployment_id': {'type': 'String', 'value': 'deploy_test'}, 'id': 'orjAWGtByDTrFU8Q4S3F0DPb4IlLntIR2KVpG_8FnIsJ5oQcW2BY10AujhI0SXfjOSbzI7i3TQyEWOzCYKP-dA', 'llm_model': {'type': 'String', 'value': 'gpt-4o'}, 'name': {'type': 'String', 'value': 'CodeReviewer'}, 'system_prompt': {'type': 'String', 'value': 'You review code for correctness and style.'}, 'tools': {'type': 'Array', 'value': ['web_search', 'file_read']}}]}

--- agent_get ---
Fetched: {'type': 'String', 'value': 'CodeReviewer'}

--- agent_get_by_name ---
By name: {'type': 'String', 'value': 'CodeReviewer'}

--- agent_update ---
Updated agent prompt: {'type': 'String', 'value': 'You review code. Be concise.'}

--- agents_by_deployment ---
Agents in deploy_test: {'count': 0, 'items': []}
WARNING: agents_by_deployment omitted created agent orjAWGtByDTrFU8Q4S3F0DPb4IlLntIR2KVpG_8FnIsJ5oQcW2BY10AujhI0SXfjOSbzI7i3TQyEWOzCYKP-dA; TODO: check/fix the server-side deployment lookup

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
Inserted document: QWzuHyUdAA3IkZNliYROjfmAhnPUhRmi8dTX_MvQuPKsxtnJWod6MwqaTtBsGWY4xzblItzhWjD8wPwnBv5_5Q

--- kv_link ---
Linked: None

--- kv_get_links ---
Links for app:config: [{'collection': 'kv_links_example_py', 'created_at': '2026-10-08T05:45:59.925372Z', 'document_id': 'QWzuHyUdAA3IkZNliYROjfmAhnPUhRmi8dTX_MvQuPKsxtnJWod6MwqaTtBsGWY4xzblItzhWjD8wPwnBv5_5Q', 'field_path': None, 'last_accessed': '2026-10-08T05:45:59.925998Z', 'metadata': {}}]

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
  kv_operations:py:cache:product:2: {'name': 'Product 2', 'price': 39.99}
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
    "name": "Mars",
    "diameter_km": 6779
  }
]

--- Blocking HTTP (for comparison) ---
Blocking response: Hello! I hope you're having a wonderful day.

=== Done ===
=== ekoDB Schedule Management Example (Python) ===

--- create_schedule ---
Created schedule: Nightly Report (id: 368dff75-4ff9-4a77-9278-b329c018676d)

--- list_schedules ---
Schedules: {'count': 1, 'schedules': [{'created_at': '2026-10-08T05:46:02.453192Z', 'cron_expression': '0 0 0 * * *', 'description': 'Generate and email nightly analytics report', 'enabled': True, 'function_label': 'schedule_noop_python_73526', 'id': '368dff75-4ff9-4a77-9278-b329c018676d', 'last_execution': None, 'name': 'Nightly Report', 'next_execution': '2026-10-09T00:00:00Z', 'parameters': {}, 'stats': {'avg_execution_time_ms': 0.0, 'failed_executions': 0, 'last_error': None, 'successful_executions': 0, 'total_executions': 0}, 'timezone': 'UTC', 'updated_at': '2026-10-08T05:46:02.453192Z'}]}

--- get_schedule ---
Fetched: Nightly Report cron=0 0 0 * * *

--- update_schedule ---
Updated cron: 0 30 1 * * *

--- trigger_schedule ---
Triggered: {'schedule_id': '368dff75-4ff9-4a77-9278-b329c018676d', 'status': 'triggered'}

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
  2. Score: 13.200, Matched: title, bio
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
  1. Score: 0.741
  2. Score: 0.726
  3. Score: 0.725

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.371, Matched: content, title
  2. Score: 0.863, Matched: content, title
  3. Score: 0.363, Matched:

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
Inserted: {'id': 'QCAKHolDXEdl5ttxMdtgTn_9VNohbHVua1vtzs-1QOsa39dPVNu6Q4QmFLTmeJ_wfGI2Lvirtvu92edfIUB_xw'}

=== Find by ID ===
Found: {'price': {'type': 'Float', 'value': 99.99}, 'created_at': {'type': 'String', 'value': '2026-10-08T01:46:02.912572'}, 'id': 'QCAKHolDXEdl5ttxMdtgTn_9VNohbHVua1vtzs-1QOsa39dPVNu6Q4QmFLTmeJ_wfGI2Lvirtvu92edfIUB_xw', 'data': {'type': 'String', 'value': 'aGVsbG8gd29ybGQ='}, 'metadata': {'value': {'key': 'value', 'nested': {'deep': True}}, 'type': 'Object'}, 'name': {'value': 'Test Record', 'type': 'String'}, 'active': {'value': True, 'type': 'Boolean'}, 'tags': {'type': 'Array', 'value': ['tag1', 'tag2', 'tag3']}, 'embedding': {'type': 'Array', 'value': [0.1, 0.2, 0.3, 0.4, 0.5]}, 'value': {'type': 'Integer', 'value': 42}, 'categories': {'value': ['electronics', 'computers'], 'type': 'Array'}, 'user_id': {'type': 'String', 'value': '550e8400-e29b-41d4-a716-446655440000'}}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): True
  price (Decimal): 99.99
  created_at (DateTime): 2026-10-08 01:46:02.912572
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): ['tag1', 'tag2', 'tag3']
  metadata (Object): {'key': 'value', 'nested': {'deep': True}}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): ['electronics', 'computers']
  data (Bytes): 11 bytes
Plain record: {'price': 99.99, 'created_at': '2026-10-08T01:46:02.912572', 'id': 'QCAKHolDXEdl5ttxMdtgTn_9VNohbHVua1vtzs-1QOsa39dPVNu6Q4QmFLTmeJ_wfGI2Lvirtvu92edfIUB_xw', 'data': 'aGVsbG8gd29ybGQ=', 'metadata': {'key': 'value', 'nested': {'deep': True}}, 'name': 'Test Record', 'active': True, 'tags': ['tag1', 'tag2', 'tag3'], 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5], 'value': 42, 'categories': ['electronics', 'computers'], 'user_id': '550e8400-e29b-41d4-a716-446655440000'}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'categories': {'value': ['electronics', 'computers'], 'type': 'Array'}, 'name': {'type': 'String', 'value': 'Updated Record'}, 'data': {'value': 'aGVsbG8gd29ybGQ=', 'type': 'String'}, 'id': 'QCAKHolDXEdl5ttxMdtgTn_9VNohbHVua1vtzs-1QOsa39dPVNu6Q4QmFLTmeJ_wfGI2Lvirtvu92edfIUB_xw', 'user_id': {'value': '550e8400-e29b-41d4-a716-446655440000', 'type': 'String'}, 'value': {'type': 'Integer', 'value': 100}, 'tags': {'type': 'Array', 'value': ['tag1', 'tag2', 'tag3']}, 'metadata': {'type': 'Object', 'value': {'key': 'value', 'nested': {'deep': True}}}, 'active': {'value': True, 'type': 'Boolean'}, 'created_at': {'type': 'String', 'value': '2026-10-08T01:46:02.912572'}, 'price': {'value': 99.99, 'type': 'Float'}, 'embedding': {'value': [0.1, 0.2, 0.3, 0.4, 0.5], 'type': 'Array'}}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: itAuQ4PJmD3ApiSv2gwkI0Bk7wPkeL9UI8ZP0kQPBKzsDNg8ft0RswBrmMXi3fSEKAOGJ5kxkyI5fGZTUuZzcA

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
✓ Created native SWR script: github_user_native_py (ekoudmiNn6HiDbOVjaOi95WVmdpJuf8LVr03pQb_tJkjwjxkbne9khZj9TCLfpThadoU43FCUTQakL-Rg3vtGQ)

First call (cache miss - will fetch from GitHub API):
  Response time: 128ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 2ms
  Speedup: 72.6x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit_py (gmtmgHCiMIgHatUPqljuEYkBvmkDrOQJoQCkXtxHWql_vYpOxf_3Bu5REt5uoirDllg0HckykbLevFeY3522GA)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail_py' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline_py (ORp9UryUxzIVksWta4_DGAn5QEsuXYcQzCMDd7unEwj2Oh-IyxJ54r7tjoG5XzH0hKk_T6VgIq9Q7GT4jCB6Fg)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users_swr_py' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache_py (K9dZXlZt2f9Y5iAyvDQSeHFehtHHKe9_-BnY9gBd-_7radjoegmr0yIvGHLWiUzCDpADGNQKvCBG_EMSSIM2LQ)
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
✓ Created SWR script: fetch_api_user_py (fN4R3TDU_K5wQwdKQvcEhis0T4Td6-0nQfduy_ccVmpvZjiTIhm8D_kM5B_EpyGwymYbredxRsirxkha6ufkpA)

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
Created Alice: $1000 - ID: UMe2dTk3y2Rl7EBFw8jA2le0bqWkTNF2-WxhVs8ZwyeVrI50UoXnLm0D9rky8bxupl-AJFN1dKRKPl4CR1hiSA
Created Bob: $500 - ID: sreO7wYjYljKmBTOysoT1HHY4snO3k6nMFNPdbtfTxug-Md6QVMwcfhrmlDhyWWM_yNpkOLGbPMH3g17ztu5AA

=== Example 1: Begin Transaction ===
Transaction ID (server-default isolation): 43ee772a-e5c1-44f6-8ace-8e046dd07c2c

=== Example 2: Operations within Transaction ===
Alice in transaction: {'balance': {'type': 'Integer', 'value': 800}, 'id': 'UMe2dTk3y2Rl7EBFw8jA2le0bqWkTNF2-WxhVs8ZwyeVrI50UoXnLm0D9rky8bxupl-AJFN1dKRKPl4CR1hiSA', 'account_id': {'type': 'String', 'value': 'ACC001'}, 'name': {'type': 'String', 'value': 'Alice'}}
Bob in transaction: {'name': {'type': 'String', 'value': 'Bob'}, 'account_id': {'value': 'ACC002', 'type': 'String'}, 'balance': {'value': 700, 'type': 'Integer'}, 'id': 'sreO7wYjYljKmBTOysoT1HHY4snO3k6nMFNPdbtfTxug-Md6QVMwcfhrmlDhyWWM_yNpkOLGbPMH3g17ztu5AA'}
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

✓ Verified committed balances: Alice=$800, Bob=$700

=== Example 5: Rollback Demo ===
New transaction: fca3200a-8f41-46a2-b713-a836579b0291
Updated Bob: $700 → $600 (in transaction)
Status before rollback: Active
✓ Transaction rolled back

✓ Verified Bob remains $700 after rollback

=== Cleanup ===
✓ Deleted test account collection

✓ All client transaction examples completed
✓ Client created

=== Create User Function ===
Created user function with ID: lfe3Nvbakg8E2l0LmDnVfnGY5CxIDqxyPAWctOE_1NM8DSxuNRG61dzwjCWtDuXPl2xF9h6fwf5esx57urmzbg

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

Created chat session: pYFzo_1eGiuE3BLAWR58Vl8ETAeqPrNX2lscWrnFJ3XHQmJOL_i_0WW3hWF0-kRUkdLrVr2PUUo7Ej-ySaT2wA

Sending message: 'What is the capital of France?'
{"__progress":"received"}{"__progress":"generating","model":"gpt-4o-mini","provider":"openai"}The capital of France is Paris.

--- Stream ended ---
Message ID: HWvw-1hiTKIb3fbqDMOFrYlE2qIeoxt81bueUb1i4JXRhyv7HTLm0BnL53eaOOJZF62A7qUH8f6oayqfjCpv0g
Execution time: 647ms
Token usage: {'completion_tokens': 8, 'prompt_tokens': 15, 'total_tokens': 23}

Full response: {"__progress":"received"}{"__progress":"generating","model":"gpt-4o-mini","provider":"openai"}The capital of France is Paris....
=== WebSocket Subscription Example (Python) ===

✓ Authentication successful
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_py' ===
✓ Subscribed (subscription_id: sub_85eb612cb8874e79b9f6e26011e3764a)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: tRgyu7VudRgeLRQ8Bg_tN7YUR3DNyzurDrREusPbdKRDfslFWTDWibpI5EfDY5md3CJZlscLOYPvDVfCgIObyg

  📡 Notification received:
     Event:      insert
     Collection: ws_subscribe_example_py
     Record IDs: tRgyu7VudRgeLRQ8Bg_tN7YUR3DNyzurDrREusPbdKRDfslFWTDWibpI5EfDY5md3CJZlscLOYPvDVfCgIObyg
     Timestamp:  2026-10-08T05:46:05.312015+00:00

Inserting record 2...
✓ Inserted: zT_DVQceB0h-MkEYIL-xkdKxaLRx09fF6TfIZfKA3HKv0D5tnXrZFa-oG-ybAeCz8EnG-t27kIoayYSd-HDfVA

  📡 Notification received:
     Event:      insert
     Record IDs: zT_DVQceB0h-MkEYIL-xkdKxaLRx09fF6TfIZfKA3HKv0D5tnXrZFa-oG-ybAeCz8EnG-t27kIoayYSd-HDfVA

=== Unsubscribing ===
✓ Unsubscribed: {'collection': 'ws_subscribe_example_py', 'found': True, 'unsubscribed': True}

=== Cleanup ===
✓ Deleted collection 'ws_subscribe_example_py'

✓ WebSocket subscription example completed successfully
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: SK3Id7oreuZBLreU_KTis4jd18gRXjhTyyTjqs6fLc1w3VFimHBA9Kxcwg_NWaTc_hPLoXxEsoN6FgDj6qMnQA

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
   Inserted with ripple: {'id': 'sqzoXavluRWSCXOE2mRBh3GI4DNEZFm0TJYHsCAO5hvp0YlQEy8Ax7UGcACTe9ctTR6qc8CpkV563PJzRKXZmA'}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {'id': 'hmQRhGLGz0bUl30hiBxcC1BwNl4lCzLzfROQFB32blngjUXtAfhdnUqvXl4Ryg6RCZCOW8Y_SHM9LEbAV73OSQ'}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {'price': {'value': 150, 'type': 'Integer'}, 'name': {'type': 'String', 'value': 'Product 1'}, 'id': 'sqzoXavluRWSCXOE2mRBh3GI4DNEZFm0TJYHsCAO5hvp0YlQEy8Ax7UGcACTe9ctTR6qc8CpkV563PJzRKXZmA'}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {'id': 'sqzoXavluRWSCXOE2mRBh3GI4DNEZFm0TJYHsCAO5hvp0YlQEy8Ax7UGcACTe9ctTR6qc8CpkV563PJzRKXZmA', 'price': {'value': 500, 'type': 'Integer'}, 'name': {'value': 'Upsert Product', 'type': 'String'}}

✅ All bypass_ripple operations completed successfully!
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields (id, name, email only)
  Found 3 active users
  Fields returned: ['id', 'name', 'email']
  First user: {'value': 'Dave Brown', 'type': 'String'} <{'type': 'String', 'value': 'dave@example.com'}>

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ['name', 'created_at', 'bio', 'age', 'status', 'avatar_url', 'user_role', 'id', 'email']

Example 3: Complex query with projection (active users, ages 18-65)
  Found 3 active users (ages 18-65)
    - {'type': 'String', 'value': 'Dave Brown'} (age {'value': 45, 'type': 'Integer'})
    - {'type': 'String', 'value': 'Alice Johnson'} (age {'type': 'Integer', 'value': 30})
    - {'type': 'String', 'value': 'Bob Smith'} (age {'value': 25, 'type': 'Integer'})

Example 4: Query inactive users with profile fields
  Found 1 inactive users
    - {'type': 'String', 'value': 'Carol White'}: {'type': 'String', 'value': 'Manager'}

Example 5: Compare full vs projected data
  Full query:
    - 12 fields per record
    - Fields: ['secret_token', 'email', 'id', 'api_key', 'avatar_url', 'user_role', 'status', 'name', 'created_at', 'password', 'bio', 'age']
  Projected query:
    - 3 fields per record
    - Fields: ['email', 'name', 'id']
  Bandwidth savings: ~75% fewer fields

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
