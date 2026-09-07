make test-examples-python
📦 [36mEnsuring Python example dependencies in .venv...[0m
🧪 [36mRunning Python examples (direct HTTP/WebSocket)...[0m
=== Simple CRUD Operations (Direct HTTP) ===

✓ Authentication successful

=== Insert Document ===
Inserted: {'id': 'a1Q5wlEhvqySoP3kgNk40K5Ijz_ty6228uY7R3jdt1PWY_9AwRQWmzaOuEYcUY357OSO7cUHcKCPURGECnCL_g'}

=== Find by ID ===
Found: {'name': {'value': 'Test Record', 'type': 'String'}, 'value': {'type': 'Integer', 'value': 42}, 'id': 'a1Q5wlEhvqySoP3kgNk40K5Ijz_ty6228uY7R3jdt1PWY_9AwRQWmzaOuEYcUY357OSO7cUHcKCPURGECnCL_g', 'active': {'type': 'Boolean', 'value': True}}

=== Find with Query ===
Found documents: [{'id': 'a1Q5wlEhvqySoP3kgNk40K5Ijz_ty6228uY7R3jdt1PWY_9AwRQWmzaOuEYcUY357OSO7cUHcKCPURGECnCL_g', 'value': {'type': 'Integer', 'value': 42}, 'active': {'type': 'Boolean', 'value': True}, 'name': {'value': 'Test Record', 'type': 'String'}}]

=== Update Document ===
Updated: {'active': {'type': 'Boolean', 'value': True}, 'value': {'value': 100, 'type': 'Integer'}, 'id': 'a1Q5wlEhvqySoP3kgNk40K5Ijz_ty6228uY7R3jdt1PWY_9AwRQWmzaOuEYcUY357OSO7cUHcKCPURGECnCL_g', 'name': {'value': 'Updated Record', 'type': 'String'}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: eo4k2XKeeP4Jw-NIruzkp5BZ37laWem-xan_dukdvdrmZzoejTtyeo5p4ZQXOFLyMu6bG4acNriPRHzHTq0EoQ

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
        "id": "eo4k2XKeeP4Jw-NIruzkp5BZ37laWem-xan_dukdvdrmZzoejTtyeo5p4ZQXOFLyMu6bG4acNriPRHzHTq0EoQ",
        "value": {
          "type": "Integer",
          "value": 42
        }
      },
      {
        "value": {
          "value": 42,
          "type": "Integer"
        },
        "id": "WsbRCK94m4hP3hqb2DP9BPADED5Qb3sY8pyagkVQFOgdbEuIFn84mi1fBQkRrv_aJlCNvJp3vQhqcdHXQRFFCQ",
        "name": {
          "value": "WebSocket Test Record",
          "type": "String"
        },
        "active": {
          "value": true,
          "type": "Boolean"
        }
      }
    ]
  },
  "messageId": "1203295986"
}
✓ Retrieved 2 record(s) via WebSocket

✓ WebSocket example completed successfully
🚀 ekoDB Functions Example (Python/HTTP)

✓ Authentication successful

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Function saved: aQOhxtl8FqlCWAfLt2kwxkBBWENcuPzH6NvLErvi6igzH0oBBanpH3WNqr33v8qjesn0v97Aap20UK57dLbiXg
📊 Found 15 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

ℹ️  Function 'get_active_users_paginated' already existed — updated instead
✅ Function saved: get_active_users_paginated
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: Vdeac4D5AE1Yu0e01zhmaHhYZ9Nm9THgZSo8j-SRN9MPn0a91xHPNoeRkmeySi10cTclh4mUgBgwDOuI9FfrNA
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {'max_score': {'type': 'Integer', 'value': 100}, 'count': {'type': 'Integer', 'value': 15}, 'avg_score': {'value': 60.0, 'type': 'Float'}, 'status': {'value': 'active', 'type': 'String'}}
   {'count': {'type': 'Integer', 'value': 15}, 'max_score': {'type': 'Integer', 'value': 90}, 'avg_score': {'type': 'Float', 'value': 50.0}, 'status': {'value': 'inactive', 'type': 'String'}}

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
cache:product:1: {'value': {'name': 'Product 1', 'price': 29.99}, 'type': 'Object'}
cache:product:2: {'type': 'Object', 'value': {'price': 39.989999999999995, 'name': 'Product 2'}}
cache:product:3: {'value': {'name': 'Product 3', 'price': 49.989999999999995}, 'type': 'Object'}

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
=== Collection Management (Direct HTTP) ===

✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: IYUSaUbLfQ_PKDpAZKhyblsLz6GnsPXC-yLxOt0yHSTMEmRyHuWZJDTUonE0oZsI2oQYCDVTA56Lr-ZWsBoPwQ

=== List Collections ===
Total collections: 15
Sample collections: ['products', 'ttl_cache', 'test_collection', 'websocket_test', 'demo_collection']

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
Created Alice: $1000 - ID: 4zSyg9uMWXBIZp5pdG09vVVtv-9JKQZDx9Jy3_2nItvRFPlgsUU77I4c_z19B4GKtpXm168T0opDja99SlzFNQ
Created Bob: $500 - ID: zI9zMrj6Kzlk4V0jbY1gmKyJxx8D4xxhnAXuefWbGfxMt6E_-_eF35NFdo15OWOQ25541Td_NmEr8vfz9y65Dg

=== Example 1: Begin Transaction ===
Transaction ID: 04a42851-e07f-4666-873d-2d5cb7c9b68b

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
New transaction: 9f001080-8c8b-4a0f-9242-84e9804fc2f7
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'value': 700, 'type': 'Integer'}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: j6zd2pmeUKTOIWURRGc0TMgyKxlrn2hh1PHEwwf4KCOELgZetVE4kj5M13d-rNuO1cqvhEJWFd2m7fnr81E9qg
Created Bob: $500 - ID: XDzRzFlO-5foII8yZ6E3M_8g7u89ytQhJp_jUmnNBIXME0URjOy-2Mj05Epi7jEXkBuoxugqc8NxQx2FXYvGGA

=== Example 1: Begin Transaction ===
Transaction ID: 2c764a89-78a4-4913-9aff-0847db854e8b

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
Created Alice: $1000 - ID: O2e0z3Z7haFSsDNinnxuZI0l79hjKcsIChWQt0NHBJ_-YDk6aiNQHRx_G2oLgFo5hF0e5CnWegh8rhVcDe0Scw
Created Bob: $500 - ID: RQqc9MkpQtQqh0uHellOafhMUu0ch5X2OpY9iTVl8WmjMddflQPaE2bCVpNCi-tSvt7Zq8IJS_WWQok5JZQ8aQ

=== Example 1: Begin Transaction ===
Transaction ID: 8d2a2e82-fcad-48f8-b21a-100b3b07f7c3

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
New transaction: 5095ef61-af82-4604-b77f-9e0738bb15ac
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'type': 'Integer', 'value': 700}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 2Meq2Oq-bqtXEyOgzll3gkZb-q6_Q1fdYb6wb5vNf4KuMtAY_1uzWdNRuHyvRu9fU6spY2tmv5mIAZiOr5k20g
Created Bob: $500 - ID: D_IT-TIbTRv_DwyUjYrlz_-RUwJZ7pwC3QLuafUl1UuY0Q5VYb9JrMAJOaAro0Ox7iIMcRHi105wQ9U9ObwljA

=== Example 1: Begin Transaction ===
Transaction ID: a13f161a-634f-43f4-a6cd-18e608e32199

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
Created Alice: $1000 - ID: RbFJVZCdpm7b4YpE4q70AIWNQke1LXS5ijSm0lBOCiUnNwG5F9xeSis-aVlo_ACyaimnX46zchlM4GwaVySTmQ
Created Bob: $500 - ID: Bsef3UwZoq_OfybqwAFmMSW_IsZtAaT--GSFuxHE06pWj6GQPm5OW70B5b06bk8yTnp2anUMU5J_QCdvsKK28Q

=== Example 1: Begin Transaction ===
Transaction ID: f71f1ab5-04f9-4cfa-b415-a05bdd418300

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
New transaction: bde2f986-03b4-4b1e-9523-4c8e56b91c7e
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'type': 'Integer', 'value': 700}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: ZbNsO3K7Oya9NXlx-TqEKwRLpKB6rY1FVnb14SslwthsQT6ScpmY_ldxKdkCUjmKlyLxL5eI_sIrbJ8j5-uakw
Created Bob: $500 - ID: gp1DZNgs4Lzb0rfvDBND95uZzQgFV5feI-hT5WwXI3uIuE56U8njvy5n-FeHnECrsB-U7A2M7OL5ULC1NrNmOw

=== Example 1: Begin Transaction ===
Transaction ID: cdffb018-57be-4344-acca-ad0b472cf2eb

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
   ✅ Function saved: 7awGHMBG3oKYZRgo1KQ3iJmzODKVXIRWTOk4NaMzz94Z5D8JhvQWsdo6Eo9b4LmYi9gvbnzAQgpMmMEwCSOG6A

2️⃣ Calling function (Insert + Verify)...
   ✅ function executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: 3MzLRJwJJ6yNGJ_0olLOju--BUsYajj5V0dNR_NnFLJkQhTwdvFCskt1qDfvuuFvVDHvMuW6MkIV-zbil4DRJA
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}
   📋 Email: {'type': 'String', 'value': 'alice@example.com'}
   📋 Status: {'value': 'pending', 'type': 'String'}
   📋 Credits: {'type': 'Integer', 'value': 0}

============================================================
📝 function 2: Query + Update + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: 16rRnN2tEGU81ceTkwgfLp8GKbkECl47-2OfFs8gd98ZMj5pw5zhVAGG4TxNSYLmAMBiLG2FjPq7Ba9s7fylAg

2️⃣ Calling function (Query + Update + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Status updated to: {'value': 'active', 'type': 'String'}
   📋 Name: {'value': 'Alice Smith', 'type': 'String'}

============================================================
📝 function 3: Query + Update Credits + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: mKgNzr8opctVS7c3bNIp2tHfa2GONJHak1BuroblMaA2K-CiWr81FZhL36-EpTVvjlhSgkoHz5hE3-8skAgbFQ

2️⃣ Calling function (Query + Update Credits + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {'type': 'Integer', 'value': 0}
   📋 Status: {'type': 'String', 'value': 'active'}
   📋 Name: {'value': 'Alice Smith', 'type': 'String'}

============================================================
📝 function 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: B10xKdCtBK0r4c8P1doG-hDuh52tOyTmZdCz9UVcZiz5Eqb65ttmMvOP-w2RtVG3ZvkchxAzMOxabWX_ztNJLg

2️⃣ Calling function (Query + Delete + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: 7awGHMBG3oKYZRgo1KQ3...
   ✅ Deleted script: 16rRnN2tEGU81ceTkwgf...
   ✅ Deleted script: mKgNzr8opctVS7c3bNIp...
   ✅ Deleted script: B10xKdCtBK0r4c8P1doG...
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
  Output: Document ID = EXYBokqZpVvuW-34sPo0YXAmw4reCUI8ZaIZ9Az0DqUc6cwRHjhnAKVCBUYnjYpkS-EWaF-1fu9kxW_iIEY6Ew
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: find_by_id(EXYBokqZpVvuW-34sPo0YXAmw4reCUI8ZaIZ9Az0DqUc6cwRHjhnAKVCBUYnjYpkS-EWaF-1fu9kxW_iIEY6Ew)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: find_by_id(EXYBokqZpVvuW-34sPo0YXAmw4reCUI8ZaIZ9Az0DqUc6cwRHjhnAKVCBUYnjYpkS-EWaF-1fu9kxW_iIEY6Ew)
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
  Output: Document ID = OQ5OXwbGJmpAEsftgWLJgP3NMpuYwf2jARmim1WGlMJEjVclruKyTkpjJeTiYfMiy3ybFYqUUMoZe3yW2rAHyw
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: find_by_id(OQ5OXwbGJmpAEsftgWLJgP3NMpuYwf2jARmim1WGlMJEjVclruKyTkpjJeTiYfMiy3ybFYqUUMoZe3yW2rAHyw)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: find_by_id(OQ5OXwbGJmpAEsftgWLJgP3NMpuYwf2jARmim1WGlMJEjVclruKyTkpjJeTiYfMiy3ybFYqUUMoZe3yW2rAHyw)
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
    Finished `release` profile [optimized] target(s) in 1m 05s
📦 Built wheel for abi3 Python ≥ 3.8 to ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.26.1-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel into .venv...[0m
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.26.1-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.26.1
    Uninstalling ekodb_client-0.26.1:
      Successfully uninstalled ekodb_client-0.26.1
Successfully installed ekodb-client-0.26.1
🧪 [36mEnsuring test dependencies (pytest) in .venv...[0m
✅ [32mPython client package built and installed![0m
🧪 [36mRunning Python client library examples...[0m
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {'id': 'EvF-JhBbxnRr5xUGeSBQABQ4fyFXmJ9QIoMUG_xDZmd1GRZe-d4bDWi95EG0keEHVPK44nDaVp3_Rym3D-1AUw'}

=== Find by ID ===
Found: {'tags': {'type': 'Array', 'value': ['tag1', 'tag2', 'tag3']}, 'categories': {'value': ['electronics', 'computers'], 'type': 'Array'}, 'metadata': {'type': 'Object', 'value': {'nested': {'deep': True}, 'key': 'value'}}, 'id': 'EvF-JhBbxnRr5xUGeSBQABQ4fyFXmJ9QIoMUG_xDZmd1GRZe-d4bDWi95EG0keEHVPK44nDaVp3_Rym3D-1AUw', 'value': {'type': 'Integer', 'value': 42}, 'data': {'type': 'String', 'value': 'aGVsbG8gd29ybGQ='}, 'active': {'value': True, 'type': 'Boolean'}, 'embedding': {'type': 'Array', 'value': [0.1, 0.2, 0.3, 0.4, 0.5]}, 'price': {'type': 'Float', 'value': 99.99}, 'name': {'value': 'Test Record', 'type': 'String'}, 'user_id': {'value': '550e8400-e29b-41d4-a716-446655440000', 'type': 'String'}, 'created_at': {'type': 'String', 'value': '2026-09-07T17:45:51.582285'}}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): True
  price (Decimal): 99.99
  created_at (DateTime): 2026-09-07 17:45:51.582285
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): ['tag1', 'tag2', 'tag3']
  metadata (Object): {'nested': {'deep': True}, 'key': 'value'}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): ['electronics', 'computers']
  data (Bytes): 11 bytes
Plain record: {'tags': ['tag1', 'tag2', 'tag3'], 'categories': ['electronics', 'computers'], 'metadata': {'nested': {'deep': True}, 'key': 'value'}, 'id': 'EvF-JhBbxnRr5xUGeSBQABQ4fyFXmJ9QIoMUG_xDZmd1GRZe-d4bDWi95EG0keEHVPK44nDaVp3_Rym3D-1AUw', 'value': 42, 'data': 'aGVsbG8gd29ybGQ=', 'active': True, 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5], 'price': 99.99, 'name': 'Test Record', 'user_id': '550e8400-e29b-41d4-a716-446655440000', 'created_at': '2026-09-07T17:45:51.582285'}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'user_id': {'type': 'String', 'value': '550e8400-e29b-41d4-a716-446655440000'}, 'metadata': {'type': 'Object', 'value': {'key': 'value', 'nested': {'deep': True}}}, 'price': {'value': 99.99, 'type': 'Float'}, 'tags': {'value': ['tag1', 'tag2', 'tag3'], 'type': 'Array'}, 'value': {'type': 'Integer', 'value': 100}, 'data': {'value': 'aGVsbG8gd29ybGQ=', 'type': 'String'}, 'id': 'EvF-JhBbxnRr5xUGeSBQABQ4fyFXmJ9QIoMUG_xDZmd1GRZe-d4bDWi95EG0keEHVPK44nDaVp3_Rym3D-1AUw', 'created_at': {'value': '2026-09-07T17:45:51.582285', 'type': 'String'}, 'categories': {'type': 'Array', 'value': ['electronics', 'computers']}, 'embedding': {'type': 'Array', 'value': [0.1, 0.2, 0.3, 0.4, 0.5]}, 'active': {'type': 'Boolean', 'value': True}, 'name': {'value': 'Updated Record', 'type': 'String'}}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: YiKAR-m432k_Dbi7mQlK7DK8rQMEcoswMXGqWyhbiLZaVRJ9rVlQ3AuYpS2NW1BSlowl7J6LKSvuvfjxJkGK7w

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
Collection created with first record: "ZQ-lYIMJXpju2ZIVubirsLkSx_mqCR-W32kJHqxF6ZSXPoCIAWbX3OgRjUzSdp1DoG0bNmDvdMXQMVbUM6SqOg"

=== List Collections ===
Total collections: 14
Sample collections: ['products', 'ttl_cache', 'test_collection', 'websocket_test', 'chat_configurations__ek0_testing']

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
Total keys in store: 6

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
Created Alice: $1000 - ID: yhTEl0mjHcP2H6FwcMMYeoU3eW8cEDdgv25XNOCxsU2gbjR53ZQDKWQ1bsjCDAKcHe7adwDpUpSGiWDTHbCzAQ
Created Bob: $500 - ID: _3s4vR0IQGYe8d2ACxN379RK5YrHHIgHf6XzdAIW3u8qyOGEZoCqXcXaGuwGSDuEz8LqRenoWuJqxWrmT7aMIA

=== Example 1: Begin Transaction ===
Transaction ID: a9f53d07-b148-4254-839b-3c2ad91f4194

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 0502005d-e8d0-4a33-8284-26ddf2c501d7
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
  1. Score: 12.870, Matched: email, name
  2. Score: 6.270, Matched: name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 13.200, Matched: bio, title
  2. Score: 13.200, Matched: bio, title
  3. Score: 13.200, Matched: bio, title
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
  1. Score: 0.751
  2. Score: 0.732
  3. Score: 0.720

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.366, Matched: title, content
  2. Score: 0.860, Matched: content, title
  3. Score: 0.375, Matched: 

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
✓ Inserted document: IhIyxXzg8h0v-2lDMxyW9LKyybpO0CnW9Jg8ll0XkVZ32mKfQkMjm_qWumr__jglVRrKqafRpJGH7-BChcJdgQ

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: X0-6UWcWwTRAX2GNzyLOwRxloOQsMxa-rsHCMDIUFkLFtIpCen1Xd_K7RJwzpfaEjTMe4oUMP2zDnkbuKJLuvQ

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
✓ Inserted document with TTL: I52ollVSHC7QKSNotyzSJdi7rZclamQASY9cQ2QhAOouqEKRKbOAfNjKny21H_rDOC8ZODPiwTJdR4LPMviQ5A

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
✓ Edge cache script created: SabLIGLiEs9jEZhgP33XWXfHibOVcf260UhOo0gsTlpv2pV1XXZBnQM_COSVfoqjk4hOtcU2Yj1_fDfAgSn1NA

Call 1: Cache miss (fetches from API)
Response time: 463ms
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "current": {
            "interval": 900,
            "temperature_2m": 26.4,
            "time": "2026-09-07T21:45"
          },
          "current_units": {
            "interval": "seconds",
            "temperature_2m": "\u00b0C",
            "time": "iso8601"
          },
          "elevation": 32.0,
          "generationtime_ms": 0.020742416381835938,
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
Response time: 14ms (34.2x faster!)
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "current": {
            "interval": 900,
            "temperature_2m": 26.4,
            "time": "2026-09-07T21:45"
          },
          "current_units": {
            "interval": "seconds",
            "temperature_2m": "\u00b0C",
            "time": "iso8601"
          },
          "elevation": 32.0,
          "generationtime_ms": 0.020742416381835938,
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

✅ Function saved: Bej__DMx1TsPCHeDWAqjwQrK4yO280cLF349X8Qe6QWmaH11P6KCmowRlDCfcUUiBo_WGA9A62jgkrwu4W5O2Q
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
   ⏱️  Duration: 87.5ms
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
   ⏱️  Duration: 2.5ms
   📊 Records: 1
   📦 Data: {
      "value": {
            "type": "Object",
            "value": {
                  "address": {
                        "city": "Gwenborough",
                        "geo": {
                 ...
   🚀 Cache speedup: 35.3x faster!

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

✅ Function saved: j-FfwMe1hD5AEe8kef_rXOrbbxR7M3AT5sl3U2ebQYeLegwzCnQxXdCb1Ifr-X6AlLioQndyUiDyRC56_hPXOA
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

✅ Inserted order: zHL8H7SIXdHrqfKsHUkMUTVkOsRpogO3ba7f10hXBGERNXKXDafEtzbiHPdx6xLsxQ69UehQeBC_q1oCVtb5_A
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

✅ Inserted order: 5jLe2y4E2tQzz6qBGahbdXfd7Tf_3jje44r473TF58gyZ82W9NSUBvIWalDeT5MvuUyRaEFH3rN1K4pmh9zkiA
✅ Cached order status
📊 Quick status lookup: {'value': '{"type":"Object","value":{"status":"processing","updated_at":"2026-09-07T21:45:54.527404+00:00"}}'}

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
✓ Created SWR script: fetch_api_user_py (fxUFE9JIn1JbZJs0ic8LgAEi0yDJKckws4qwSUVUfvVC9zmFZRFYq6MTtUhF2mAJ2mt4AMFWtJKeRVZDsG7h5g)

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
✓ Created native SWR script: github_user_native (1z1z7w7vKOw2kY3ehSrm5WPK-sdgTEJMu_8Jor8kBMYDuWuYNO-jkDNxhBMZBPkROCIin7rTML77Q2--pXdQoA)

First call (cache miss - will fetch from GitHub API):
  Response time: 11201ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 8ms
  Speedup: 1482.4x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (xZnkPwtbTr5eJDOXe3c6u36rQ_oJKEDQPOXa_vH9fED2-XXII71CSt_958e3ik5u_QEs6ZyIgGZ-7lUGWyRsbQ)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (GNvfJ_xijkUBMoOlSp2l7sJZ0l0WEmMpZCFUosywpIYEFTZX5gs6MLAGbonvvQ8a9nI3sHxjdfc5CJ5o-WQ2Qg)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (SSCjxkh4Rj-0soIhyXJckd6xAvC3subZ95cQQbnyfPwa2LeZ1C7IGQG1ZE05HZLDB9hbtKHyNycxkeOMvfPPNw)
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
   Vector databases offer several benefits, including:

1. **Efficient Similarity Search**: They enable fast similarity searches using vector representations, making them ideal for applications like recommendation systems and image retrieval.

2. **Handling Unstructured Data**: They efficiently manage and query unstructured data, such as text and images, by converting them into vector embeddings.

3. **Scalability**: Vector databases can scale to handle large datasets and high-dimensional vectors, supporting growing data needs.

4. **Real-time Processing**: Many vector databases are optimized for real-time querying, allowing for quick response times in dynamic environments.

5. **Advanced Machine Learning Integration**: They support machine learning workloads, enabling seamless integration of AI models for tasks like clustering and classification.

6. **Dimensionality Reduction Support**: Vector databases often incorporate techniques for dimensionality reduction, which can improve search efficiency and reduce storage costs.

7. **Multi-modal Data Support**: They can work with different types of data (text, images, audio) by representing them in a unified vector space.

Overall, vector databases enhance performance and capabilities in managing and querying complex data types.
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
🚀 ekoDB Python Search Functions Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Function saved
📊 Found 5 documents
   1. Database Design Principles (Database)
   2. Natural Language Processing (AI)
   3. Introduction to Machine Learning (AI)
   4. Getting Started with ekoDB (Database)
   5. Vector Databases Explained (Database)
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
✓ Created session: daiZC2QZTBDSnnyM0mBh543J3Ee0r_G5DhBylJLWym6POyg7M3-dT4tow1FkEhyU8qc6hIfOLBx-FI_9u16NIA

=== Sending Chat Message ===
Message ID: Z1ovPwJ2kPbYsyqjsy1VExkeLPHg0k9Cln16ohUx0lVHw03Ab0K6ddqtKWLfaPdLkw9kh-HIsJJuQ4HllzZsKw

=== AI Response ===
The available products and their prices are as follows:

1. **ekoDB Pro**
   - Price: $299
   - Description: Enterprise edition product with advanced features

2. **ekoDB**
   - Price: $99
   - Description: A high-performance database product with AI capabilities

3. **ekoDB Cloud**
   - Price: $499
   - Description: Fully managed cloud database service product

Execution Time: 1761ms

=== Token Usage ===
Prompt tokens: 3413
Completion tokens: 90
Total tokens: 3503

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: F0HuY0ip7tQDKrvJ6MATDEXoovwHKTK1DWMZMCCrguajTcs9RqP-okILyItnUmQ7uVdyDtZzWfdv45vAVcPTfQ

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** High-performance database product
- **Price:** $99

If you need more details or other products, just let me know!

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['content', 'created_at', 'chat_id', 'llm_model', 'id', 'role', 'llm_provider', 'token_usage', 'updated_at', 'context_snippets'])
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
✓ Created second session: MsFgaGHuEMqowhdNh55laOo1tJMk2CoF8bjwtylNl90Ei5BOTrKz0kqajiENJs7al67mxOoZizgXgU1_a19jWg
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
✓ Created session: nkBJXWlWv5CQEGIuO_iZt14BLDnyv3qqWNQSWIMEdEG76GnvcBXraqm_M40BZLRGqU3AaCGxIVVZv7wwyClb2g

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** A high-performance database product
- **Price:** $99

If you need more information or have any other questions, feel free to ask!

✓ Message 2 sent
  Response: The price of ekoDB is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: JZyJ6uwvW4GaYyX1ux4VkCPcIR5NTaqX5wMo1jJepkZvZb6DDC2AuyrZ7fSG-wlPDxpVHRjHYAmkw1Jf_HlEVA
  Parent: nkBJXWlWv5CQEGIuO_iZt14BLDnyv3qqWNQSWIMEdEG76GnvcBXraqm_M40BZLRGqU3AaCGxIVVZv7wwyClb2g

=== Listing Sessions ===
✓ Found 7 sessions
  Session 1: JZyJ6uwvW4GaYyX1ux4VkCPcIR5NTaqX5wMo1jJepkZvZb6DDC2AuyrZ7fSG-wlPDxpVHRjHYAmkw1Jf_HlEVA (Untitled)
  Session 2: nkBJXWlWv5CQEGIuO_iZt14BLDnyv3qqWNQSWIMEdEG76GnvcBXraqm_M40BZLRGqU3AaCGxIVVZv7wwyClb2g (Untitled)
  Session 3: MsFgaGHuEMqowhdNh55laOo1tJMk2CoF8bjwtylNl90Ei5BOTrKz0kqajiENJs7al67mxOoZizgXgU1_a19jWg (Untitled)
  Session 4: daiZC2QZTBDSnnyM0mBh543J3Ee0r_G5DhBylJLWym6POyg7M3-dT4tow1FkEhyU8qc6hIfOLBx-FI_9u16NIA (Untitled)
  Session 5: 1XLUyFhbuCQozWHtbWvx5Jjk1LEyfQxrk2ovJjO7bwll-2gtz50K88C5Zqr1REVwWLTNjHiDTqQmG2I9hmEMLQ (Untitled)
  Session 6: L1UQzFyFR4ccuvwyDYzO0gicZTwcGTyE2eObq41Pxc0Af_hs8haT87q3nMhpH4rHu1SgAXeMe2rlCNCgMNRpWA (Untitled)
  Session 7: EpncQpgraSLlTwxuY9OQk1S-6BHw17FPHrGe1hQSC1k3Y-mKu82KFBbgejEmRy0C6de3NyaqW20Z830tZ6GwRA (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: JZyJ6uwvW4GaYyX1ux4VkCPcIR5NTaqX5wMo1jJepkZvZb6DDC2AuyrZ7fSG-wlPDxpVHRjHYAmkw1Jf_HlEVA

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Dict Creation ===
✓ Created record with plain dict: {'id': '-Hf0Shv71kf9kwGjhSzD0YYz-MXTAy-uDnErUXXFqBhOwWhMo74NcZqY89OAvZtevNj0neuV1QYUoD7hZEmXtA'}

=== Upsert Operation ===
✓ Upsert (update existing record): -Hf0Shv71kf9kwGjhSzD0YYz-MXTAy-uDnErUXXFqBhOwWhMo74NcZqY89OAvZtevNj0neuV1QYUoD7hZEmXtA
✓ Inserted second record: L4pf8IKDJi445rF3t_eGy3U8edgfOnQOR58VEkTkhrYEYwSZuPKaGQbMm8kMb40skKkCv32o5NGujYBfXWqFXA
✓ Upsert (update second record): L4pf8IKDJi445rF3t_eGy3U8edgfOnQOR58VEkTkhrYEYwSZuPKaGQbMm8kMb40skKkCv32o5NGujYBfXWqFXA

=== Find One Operation ===
✓ Found user by email: {'email': {'value': 'alice.j@newdomain.com', 'type': 'String'}, 'id': '-Hf0Shv71kf9kwGjhSzD0YYz-MXTAy-uDnErUXXFqBhOwWhMo74NcZqY89OAvZtevNj0neuV1QYUoD7hZEmXtA', 'name': {'type': 'String', 'value': 'Alice Johnson'}, 'age': {'type': 'Integer', 'value': 29}, 'active': {'value': True, 'type': 'Boolean'}}
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
   Inserted with ripple: {'id': 'rz0ZfcMp1ez7DnM4a0le60qXzN4tOw7dQnU909ZGElfgVFHXB10bE21ONzfGqYbEeLvX6u9zfOYLUHiytt8M8g'}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {'id': 'pCXreJ0K1b61UbXxxsYcuZwrwRuQgxjOHw0xEtEd9D4fuu_vAY6r41WJBJVTjp31Y-jfvw_IzaTDTedZkIb4QQ'}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {'id': 'rz0ZfcMp1ez7DnM4a0le60qXzN4tOw7dQnU909ZGElfgVFHXB10bE21ONzfGqYbEeLvX6u9zfOYLUHiytt8M8g', 'price': {'type': 'Integer', 'value': 150}, 'name': {'type': 'String', 'value': 'Product 1'}}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {'name': {'type': 'String', 'value': 'Upsert Product'}, 'price': {'value': 500, 'type': 'Integer'}, 'id': 'rz0ZfcMp1ez7DnM4a0le60qXzN4tOw7dQnU909ZGElfgVFHXB10bE21ONzfGqYbEeLvX6u9zfOYLUHiytt8M8g'}

✅ All bypass_ripple operations completed successfully!
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields (id, name, email only)
  Found 3 active users
  Fields returned: ['email', 'name', 'id']
  First user: {'value': 'Alice Johnson', 'type': 'String'} <{'value': 'alice@example.com', 'type': 'String'}>

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ['user_role', 'age', 'created_at', 'avatar_url', 'id', 'bio', 'email', 'name', 'status']

Example 3: Complex query with projection (active users, ages 18-65)
  Found 3 active users (ages 18-65)
    - {'value': 'Dave Brown', 'type': 'String'} (age {'type': 'Integer', 'value': 45})
    - {'type': 'String', 'value': 'Alice Johnson'} (age {'type': 'Integer', 'value': 30})
    - {'value': 'Bob Smith', 'type': 'String'} (age {'value': 25, 'type': 'Integer'})

Example 4: Query inactive users with profile fields
  Found 1 inactive users
    - {'type': 'String', 'value': 'Carol White'}: {'type': 'String', 'value': 'Manager'}

Example 5: Compare full vs projected data
  Full query:
    - 12 fields per record
    - Fields: ['created_at', 'secret_token', 'age', 'status', 'user_role', 'email', 'bio', 'id', 'name', 'avatar_url', 'password', 'api_key']
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
OpenAI models: ['text-embedding-ada-002', 'whisper-1', 'gpt-3.5-turbo', 'tts-1', 'gpt-3.5-turbo-16k', 'gpt-4-0613', 'gpt-4', 'davinci-002', 'babbage-002', 'gpt-3.5-turbo-instruct', 'gpt-3.5-turbo-instruct-0914', 'gpt-3.5-turbo-1106', 'tts-1-hd', 'tts-1-1106', 'tts-1-hd-1106', 'text-embedding-3-small', 'text-embedding-3-large', 'gpt-3.5-turbo-0125', 'gpt-4-turbo', 'gpt-4-turbo-2024-04-09', 'gpt-4o', 'gpt-4o-2024-05-13', 'gpt-4o-mini-2024-07-18', 'gpt-4o-mini', 'gpt-4o-2024-08-06', 'omni-moderation-latest', 'omni-moderation-2024-09-26', 'o1-2024-12-17', 'o1', 'o3-mini', 'o3-mini-2025-01-31', 'gpt-4o-2024-11-20', 'gpt-4o-mini-search-preview-2025-03-11', 'gpt-4o-mini-search-preview', 'gpt-4o-transcribe', 'gpt-4o-mini-transcribe', 'o1-pro-2025-03-19', 'o1-pro', 'gpt-4o-mini-tts', 'o3-2025-04-16', 'o4-mini-2025-04-16', 'o3', 'o4-mini', 'gpt-4.1-2025-04-14', 'gpt-4.1', 'gpt-4.1-mini-2025-04-14', 'gpt-4.1-mini', 'gpt-4.1-nano-2025-04-14', 'gpt-4.1-nano', 'gpt-image-1', 'o4-mini-deep-research', 'gpt-4o-transcribe-diarize', 'o4-mini-deep-research-2025-06-26', 'gpt-5-chat-latest', 'gpt-5-2025-08-07', 'gpt-5', 'gpt-5-mini-2025-08-07', 'gpt-5-mini', 'gpt-5-nano-2025-08-07', 'gpt-5-nano', 'gpt-audio-2025-08-28', 'gpt-realtime', 'gpt-realtime-2025-08-28', 'gpt-audio', 'gpt-5-codex', 'gpt-image-1-mini', 'gpt-5-pro-2025-10-06', 'gpt-5-pro', 'gpt-audio-mini', 'gpt-audio-mini-2025-10-06', 'gpt-5-search-api', 'gpt-realtime-mini', 'sora-2', 'sora-2-pro', 'gpt-5-search-api-2025-10-14', 'gpt-5.1-chat-latest', 'gpt-5.1-2025-11-13', 'gpt-5.1', 'gpt-5.1-codex', 'gpt-5.1-codex-mini', 'gpt-5.1-codex-max', 'gpt-image-1.5', 'gpt-5.2-2025-12-11', 'gpt-5.2', 'gpt-5.2-pro-2025-12-11', 'gpt-5.2-pro', 'gpt-5.2-chat-latest', 'gpt-4o-mini-transcribe-2025-12-15', 'gpt-4o-mini-transcribe-2025-03-20', 'gpt-4o-mini-tts-2025-03-20', 'gpt-4o-mini-tts-2025-12-15', 'gpt-realtime-mini-2025-12-15', 'gpt-audio-mini-2025-12-15', 'chatgpt-image-latest', 'gpt-5.2-codex', 'gpt-5.3-codex', 'gpt-realtime-1.5', 'gpt-audio-1.5', 'gpt-4o-search-preview', 'gpt-4o-search-preview-2025-03-11', 'gpt-5.3-chat-latest', 'gpt-5.4-2026-03-05', 'gpt-5.4-pro', 'gpt-5.4-pro-2026-03-05', 'gpt-5.4', 'gpt-5.4-nano-2026-03-17', 'gpt-5.4-nano', 'gpt-5.4-mini-2026-03-17', 'gpt-5.4-mini', 'gpt-image-2', 'gpt-image-2-2026-04-21', 'gpt-5.5', 'gpt-5.5-2026-04-23', 'gpt-5.5-pro', 'gpt-5.5-pro-2026-04-23', 'chat-latest', 'gpt-realtime-translate', 'gpt-realtime-2', 'gpt-realtime-whisper', 'gpt-5.6-sol', 'gpt-5.6-terra', 'gpt-5.6-luna', 'gpt-realtime-2.1', 'gpt-realtime-2.1-mini', 'gpt-transcribe', 'gpt-live-transcribe', 'gpt-6-astra']
Anthropic models: ['claude-fable-5-1', 'claude-opus-5', 'claude-sonnet-5', 'claude-fable-5', 'claude-opus-4-8', 'claude-opus-4-7', 'claude-sonnet-4-6', 'claude-opus-4-6', 'claude-opus-4-5-20251101', 'claude-haiku-4-5-20251001', 'claude-sonnet-4-5-20250929']
Perplexity models: []
Gemini models: []

=== Get OpenAI Models ===
OpenAI models: ['text-embedding-ada-002', 'whisper-1', 'gpt-3.5-turbo', 'tts-1', 'gpt-3.5-turbo-16k', 'gpt-4-0613', 'gpt-4', 'davinci-002', 'babbage-002', 'gpt-3.5-turbo-instruct', 'gpt-3.5-turbo-instruct-0914', 'gpt-3.5-turbo-1106', 'tts-1-hd', 'tts-1-1106', 'tts-1-hd-1106', 'text-embedding-3-small', 'text-embedding-3-large', 'gpt-3.5-turbo-0125', 'gpt-4-turbo', 'gpt-4-turbo-2024-04-09', 'gpt-4o', 'gpt-4o-2024-05-13', 'gpt-4o-mini-2024-07-18', 'gpt-4o-mini', 'gpt-4o-2024-08-06', 'omni-moderation-latest', 'omni-moderation-2024-09-26', 'o1-2024-12-17', 'o1', 'o3-mini', 'o3-mini-2025-01-31', 'gpt-4o-2024-11-20', 'gpt-4o-mini-search-preview-2025-03-11', 'gpt-4o-mini-search-preview', 'gpt-4o-transcribe', 'gpt-4o-mini-transcribe', 'o1-pro-2025-03-19', 'o1-pro', 'gpt-4o-mini-tts', 'o3-2025-04-16', 'o4-mini-2025-04-16', 'o3', 'o4-mini', 'gpt-4.1-2025-04-14', 'gpt-4.1', 'gpt-4.1-mini-2025-04-14', 'gpt-4.1-mini', 'gpt-4.1-nano-2025-04-14', 'gpt-4.1-nano', 'gpt-image-1', 'o4-mini-deep-research', 'gpt-4o-transcribe-diarize', 'o4-mini-deep-research-2025-06-26', 'gpt-5-chat-latest', 'gpt-5-2025-08-07', 'gpt-5', 'gpt-5-mini-2025-08-07', 'gpt-5-mini', 'gpt-5-nano-2025-08-07', 'gpt-5-nano', 'gpt-audio-2025-08-28', 'gpt-realtime', 'gpt-realtime-2025-08-28', 'gpt-audio', 'gpt-5-codex', 'gpt-image-1-mini', 'gpt-5-pro-2025-10-06', 'gpt-5-pro', 'gpt-audio-mini', 'gpt-audio-mini-2025-10-06', 'gpt-5-search-api', 'gpt-realtime-mini', 'sora-2', 'sora-2-pro', 'gpt-5-search-api-2025-10-14', 'gpt-5.1-chat-latest', 'gpt-5.1-2025-11-13', 'gpt-5.1', 'gpt-5.1-codex', 'gpt-5.1-codex-mini', 'gpt-5.1-codex-max', 'gpt-image-1.5', 'gpt-5.2-2025-12-11', 'gpt-5.2', 'gpt-5.2-pro-2025-12-11', 'gpt-5.2-pro', 'gpt-5.2-chat-latest', 'gpt-4o-mini-transcribe-2025-12-15', 'gpt-4o-mini-transcribe-2025-03-20', 'gpt-4o-mini-tts-2025-03-20', 'gpt-4o-mini-tts-2025-12-15', 'gpt-realtime-mini-2025-12-15', 'gpt-audio-mini-2025-12-15', 'chatgpt-image-latest', 'gpt-5.2-codex', 'gpt-5.3-codex', 'gpt-realtime-1.5', 'gpt-audio-1.5', 'gpt-4o-search-preview', 'gpt-4o-search-preview-2025-03-11', 'gpt-5.3-chat-latest', 'gpt-5.4-2026-03-05', 'gpt-5.4-pro', 'gpt-5.4-pro-2026-03-05', 'gpt-5.4', 'gpt-5.4-nano-2026-03-17', 'gpt-5.4-nano', 'gpt-5.4-mini-2026-03-17', 'gpt-5.4-mini', 'gpt-image-2', 'gpt-image-2-2026-04-21', 'gpt-5.5', 'gpt-5.5-2026-04-23', 'gpt-5.5-pro', 'gpt-5.5-pro-2026-04-23', 'chat-latest', 'gpt-realtime-translate', 'gpt-realtime-2', 'gpt-realtime-whisper', 'gpt-5.6-sol', 'gpt-5.6-terra', 'gpt-5.6-luna', 'gpt-realtime-2.1', 'gpt-realtime-2.1-mini', 'gpt-transcribe', 'gpt-live-transcribe', 'gpt-6-astra']

=== Get Anthropic Models ===
Anthropic models: ['claude-fable-5-1', 'claude-opus-5', 'claude-sonnet-5', 'claude-fable-5', 'claude-opus-4-8', 'claude-opus-4-7', 'claude-sonnet-4-6', 'claude-opus-4-6', 'claude-opus-4-5-20251101', 'claude-haiku-4-5-20251001', 'claude-sonnet-4-5-20250929']

✓ Chat Models API example complete
✓ Client created

=== Create User Function ===
Created user function with ID: re4FI3SolAfgZ3bliXAqK-R8mc-t2oJ7FsJN1kpz2iIeCrdH5P0R8tc90RDx0_5buUDH1A_dRTaO-6hyjjTbdA

=== Get User Function ===
Retrieved: get_active_users_py - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 14 user functions:
  - get_active_users_updated: Get Active Users (Updated)
  - validate_user: Check if user exists
  - get_users_by_status: Get Users By Status
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_slim_user: Validate and slim down user
  - cache_api_call_py: Cache External API Call
  - fetch_user: Fetch user by code
  - get_user_wrapper: Wrapper that calls fetch_user
  - get_high_scoring_active_users: Get High Scoring Active Users
  - get_verified_user: Get verified and validated user
  - get_active_users_py: Get Active Users
  - swr_user: SWR pattern for user data (KV-based)
  - get_active_users_updated: Get Active Users (Updated)
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
