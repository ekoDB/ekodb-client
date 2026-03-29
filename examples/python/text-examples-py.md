make test-examples-python
🧪 [36mRunning Python examples (direct HTTP/WebSocket)...[0m
=== Simple CRUD Operations (Direct HTTP) ===

✓ Authentication successful

=== Insert Document ===
Inserted: {'id': '5Shu-6q0hCwe3FKCrq9AdT1KmahrPhYEoT9t1HvRqcilnDJBuozN573dF0gnA14KASF2QyeXXGwB73CGkpAh1A'}

=== Find by ID ===
Found: {'value': {'value': 42, 'type': 'Integer'}, 'name': {'value': 'Test Record', 'type': 'String'}, 'active': {'type': 'Boolean', 'value': True}, 'id': '5Shu-6q0hCwe3FKCrq9AdT1KmahrPhYEoT9t1HvRqcilnDJBuozN573dF0gnA14KASF2QyeXXGwB73CGkpAh1A'}

=== Find with Query ===
Found documents: [{'active': {'value': True, 'type': 'Boolean'}, 'value': {'value': 42, 'type': 'Integer'}, 'id': '5Shu-6q0hCwe3FKCrq9AdT1KmahrPhYEoT9t1HvRqcilnDJBuozN573dF0gnA14KASF2QyeXXGwB73CGkpAh1A', 'name': {'type': 'String', 'value': 'Test Record'}}]

=== Update Document ===
Updated: {'active': {'type': 'Boolean', 'value': True}, 'value': {'type': 'Integer', 'value': 100}, 'name': {'value': 'Updated Record', 'type': 'String'}, 'id': '5Shu-6q0hCwe3FKCrq9AdT1KmahrPhYEoT9t1HvRqcilnDJBuozN573dF0gnA14KASF2QyeXXGwB73CGkpAh1A'}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: _S5yYpl3xBCh_YyPvDcmKVdwN76GYR1Oecn1I3cEbjoR2IwpNSv7WkqMKf8ONNyBKeTkGJG7Gfwp6qJ1Ag1tbg

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "messageId": "27657813",
  "payload": {
    "data": [
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "_S5yYpl3xBCh_YyPvDcmKVdwN76GYR1Oecn1I3cEbjoR2IwpNSv7WkqMKf8ONNyBKeTkGJG7Gfwp6qJ1Ag1tbg",
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
        "id": "uECQwVBLmwZzmL6GZrW53lL84vBuwZLnKCf76m2k1zWzCWK2lhqx5kfniy4ncQB0Ggm7bX1kOBzBkTnjpJKOHg",
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

✅ Script saved: 7lj4T79UX9YvVXEzL3ySifTkjZYCfr5dpfscn1LZ366AtB0YVtcSVR_OWZQUffidoe4LEQJrv4OtOnQvgQ5SsA
📊 Found 15 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: ZvoL9zhMYNSe3XARt-m3qNc2Qq7NPSb2P7zBeQgmI-fZ7jwoFbSs5I46yF_vPGiJBwljYGZppN1nV1SHmjpMlg
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: l0aYjdkLXGquL6xWi2TX57r8nOW1mwxe6ndrG_nEmGFd_l9nq8vObca52FTu7OYRWXXR3emp6bdIpw2NCTMKbg
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {'max_score': 90, 'count': 15, 'status': 'inactive', 'avg_score': 50.0}
   {'avg_score': 60.0, 'count': 15, 'max_score': 100, 'status': 'active'}

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
cache:product:1: {'price': 29.99, 'name': 'Product 1'}
cache:product:2: {'name': 'Product 2', 'price': 39.989999999999995}
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
Collection created with first record: RU6Vno86YtlLs1UIROH1StPagwX_7oZQJYso09JPOxh8V7HvngOvbo6oq4TyskThFrRuuiHKCAlhQURGJlLpvA

=== List Collections ===
Total collections: 14
Sample collections: ['users', 'chat_configurations__ek0_testing', 'demo_collection', 'ws_ttl_test', 'scripts__ek0_testing']

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
Created Alice: $1000 - ID: Fe86tL-w2j0h0eWaSwWIGuIXFDJT9F2Aqr7qGRh-SRwO1MpbAgoMpxaJWbW1fOImzPHShkUqj77sY2kRTA5v_w
Created Bob: $500 - ID: BHHV1plBnlwlSf2xC4_80-xQVhdUlBTaewHrLzGuAAciUH90RXTtNBZlqSUme9Z646wJ-iMFvnttjw9Ht4ve3w

=== Example 1: Begin Transaction ===
Transaction ID: f6fbfcc6-db34-40d8-96ef-5285542ed11d

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
New transaction: 00a2c50c-8260-4afd-be4b-9b1156615a19
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'value': 700, 'type': 'Integer'}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: tv6tAZsT4dw9miU6g0acOp_u2RSVjj8fQODGmHIdZ_mUEzD0Oc743dIApEYj-g-brTI1DBF3oLhfs2aSy5zLUQ
Created Bob: $500 - ID: GD7irzWb7TJOJSXJQD_YkPo71F1wioJgD2F6P_I3FKLCdkXQxzgemN_4XZcOxN2xoXFTegVST6A8jRBqxmwMqQ

=== Example 1: Begin Transaction ===
Transaction ID: 81e3651c-319b-4719-9b9e-fd8867f248fe

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
Created Alice: $1000 - ID: fW2ZSwgb3F0GWc9zU1muB82SRjx0lef_w_VMo0l1v5yEv3NFSQqMygqR4IHixlWE0GiLzAS38u_sdfCd-zwj_A
Created Bob: $500 - ID: XqikHCspggLjRQBm8RpEQBFGIpUiil2AGTatEhXiwyI_eRWaIiFs7Zb3hkArIV2qcS1llo4c8hONCYTU1E1sWA

=== Example 1: Begin Transaction ===
Transaction ID: e6045b93-fae7-4f05-8c09-8359a3715e07

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
New transaction: 48e7162d-b1c4-497b-afec-452aa7e6537b
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'type': 'Integer', 'value': 700}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: Jh4WHQkFWNI5cbPd0ZQ9mWiYfJdO7_DdFPB7xubreFKOKBhUrK4uIApjL0zcU6kGRyIQubUh_9aEmhZxOs8o3Q
Created Bob: $500 - ID: BCRjvdUHa71OzCtExyKfVJmcL31IkdAofzX3fJkAz41ydEFj1hORKzzTyLADOLrWmdF68BX42gj1AQ1R27I7Dg

=== Example 1: Begin Transaction ===
Transaction ID: 301ba09a-ae2f-4952-bfc7-c7174c7cc0e7

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
Created Alice: $1000 - ID: DZ4PijrrFjN7azx774N3hzz1loWk5tDWHAc7Vwo0TzoxXYqZN8jOCL4AR1ygUkmi6woi5O6-U89boz1ATfEsuQ
Created Bob: $500 - ID: bcHVQPL9v-yEnjR82NJFkEKJ7rngjdMpoID2n84hW8yvmleshmDMWSteueEWFlg6L4l6Of_AvJWnRHh5mf-q6w

=== Example 1: Begin Transaction ===
Transaction ID: 93bd2422-5897-4190-a94a-5f0f51312cf9

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
New transaction: a97d6bd9-7527-4075-9a0d-85b25a1acf86
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'type': 'Integer', 'value': 700}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 6E2jb_Jt8eteq1wD2gksTRgZI9kjwq1cArozwy3hWmO_PnvdkE0yxu0aDOJ5mJcAxVc6wUYPFU0AqX3PPJbkOA
Created Bob: $500 - ID: 7HAuw9ditPHv0lSUWIzmhehNdSG-YbZRbEuuCzOcNguDU2cI-mHgRKArqurpKNOVklR3K-2Xh-o-c5fPZj5WFA

=== Example 1: Begin Transaction ===
Transaction ID: 5290d51d-f0dc-45fd-9107-b163915e44c6

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
   ✅ Script saved: dE6VVoUrp3nZschORKFb_TH4OZd4OnmbjCPS_mpF02dT-e7tHwylZuoay57ApZMN3p-ClJw6BMmYrFbqIPOxBw

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: KCRBE4yGWdImsrAzBRMZfgAuOk1lfKfx7oxeak8Bl2r7gHi_0tOVOGIQ-ODpCoM5SR_K1KcLEUkuxq_XbvKTQQ
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}
   📋 Email: {'value': 'alice@example.com', 'type': 'String'}
   📋 Status: {'type': 'String', 'value': 'pending'}
   📋 Credits: {'type': 'Integer', 'value': 0}

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: gkPn4X2mmjI5NHS7atgsxyAvOJ89rieF3Yu-jLMUC8KJhHKkA4xHG9oLcxDcy6pbB97yeyAmlKFYFPOw7n9F4g

2️⃣ Calling Script (Query + Update + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Status updated to: {'type': 'String', 'value': 'active'}
   📋 Name: {'value': 'Alice Smith', 'type': 'String'}

============================================================
📝 Script 3: Query + Update Credits + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: mQH2wCPrNUD76lbIQ0JlLEjfLpq3eNqd9ONdMDRX5HMTn7H1SAD41ltAQFo_ZHYcUQjNVtMFTU6cxgvKEw0whQ

2️⃣ Calling Script (Query + Update Credits + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {'value': 0, 'type': 'Integer'}
   📋 Status: {'value': 'active', 'type': 'String'}
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}

============================================================
📝 Script 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: gyeXjIUlmTzNgKG-GA4cYfGz0iQ7jyZUwU-Q28CmvAVd9O-wZine9TuTaD-MvA6UZMGagUrXcMLWEtJVEBtr8w

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: dE6VVoUrp3nZschORKFb...
   ✅ Deleted script: gkPn4X2mmjI5NHS7atgs...
   ✅ Deleted script: mQH2wCPrNUD76lbIQ0Jl...
   ✅ Deleted script: gyeXjIUlmTzNgKG-GA4c...
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
  Output: Document ID = Nrt4c-WDl1QtquMfl5dRNZn0qOofDHY79sk_cc2xnXXKxv9CuHmDBfSqsy7a-c5WrDdI9rM3UTJnY-qe492yfQ
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: find_by_id(Nrt4c-WDl1QtquMfl5dRNZn0qOofDHY79sk_cc2xnXXKxv9CuHmDBfSqsy7a-c5WrDdI9rM3UTJnY-qe492yfQ)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: find_by_id(Nrt4c-WDl1QtquMfl5dRNZn0qOofDHY79sk_cc2xnXXKxv9CuHmDBfSqsy7a-c5WrDdI9rM3UTJnY-qe492yfQ)
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
  Output: Document ID = CWmdMMNLBtQtc3NYWZiuUP91CIlEVUYcjIaPlB5AAZOQ_91pX2sOEbE4_WE1RJ9374FZbA0WSO3dohO-GdRzig
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: find_by_id(CWmdMMNLBtQtc3NYWZiuUP91CIlEVUYcjIaPlB5AAZOQ_91pX2sOEbE4_WE1RJ9374FZbA0WSO3dohO-GdRzig)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: find_by_id(CWmdMMNLBtQtc3NYWZiuUP91CIlEVUYcjIaPlB5AAZOQ_91pX2sOEbE4_WE1RJ9374FZbA0WSO3dohO-GdRzig)
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
🐍 Found CPython 3.11 at /Library/Frameworks/Python.framework/Versions/3.11/bin/python3
💻 Using `MACOSX_DEPLOYMENT_TARGET=11.0` for aarch64-apple-darwin by default
    Finished `release` profile [optimized] target(s) in 0.09s
📦 Built wheel for abi3 Python ≥ 3.8 to /Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.15.1-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel for current platform...[0m
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.15.1-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.15.1
    Uninstalling ekodb_client-0.15.1:
      Successfully uninstalled ekodb_client-0.15.1
Successfully installed ekodb-client-0.15.1
✅ [32mPython client package built and installed![0m
🧪 [36mRunning Python client library examples...[0m
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {'id': 'tMJ1mv6nSokhC3LnDF3tZJzLUMN7MMa8ZlIV-FV15FAvMdIb6p-X9t5zg3_p2-mwmtH_cZ242SnMwdbRZzqz3w'}

=== Find by ID ===
Found: {'embedding': {'type': 'Array', 'value': [0.1, 0.2, 0.3, 0.4, 0.5]}, 'data': {'type': 'String', 'value': 'aGVsbG8gd29ybGQ='}, 'price': {'type': 'Float', 'value': 99.99}, 'user_id': {'type': 'String', 'value': '550e8400-e29b-41d4-a716-446655440000'}, 'tags': {'type': 'Array', 'value': ['tag1', 'tag2', 'tag3']}, 'categories': {'value': ['electronics', 'computers'], 'type': 'Array'}, 'id': 'tMJ1mv6nSokhC3LnDF3tZJzLUMN7MMa8ZlIV-FV15FAvMdIb6p-X9t5zg3_p2-mwmtH_cZ242SnMwdbRZzqz3w', 'metadata': {'value': {'nested': {'deep': True}, 'key': 'value'}, 'type': 'Object'}, 'active': {'type': 'Boolean', 'value': True}, 'created_at': {'value': '2026-03-28T19:49:16.642779', 'type': 'String'}, 'name': {'value': 'Test Record', 'type': 'String'}, 'value': {'value': 42, 'type': 'Integer'}}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): True
  price (Decimal): 99.99
  created_at (DateTime): 2026-03-28 19:49:16.642779
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): ['tag1', 'tag2', 'tag3']
  metadata (Object): {'nested': {'deep': True}, 'key': 'value'}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): ['electronics', 'computers']
  data (Bytes): 11 bytes
Plain record: {'embedding': [0.1, 0.2, 0.3, 0.4, 0.5], 'data': 'aGVsbG8gd29ybGQ=', 'price': 99.99, 'user_id': '550e8400-e29b-41d4-a716-446655440000', 'tags': ['tag1', 'tag2', 'tag3'], 'categories': ['electronics', 'computers'], 'id': 'tMJ1mv6nSokhC3LnDF3tZJzLUMN7MMa8ZlIV-FV15FAvMdIb6p-X9t5zg3_p2-mwmtH_cZ242SnMwdbRZzqz3w', 'metadata': {'nested': {'deep': True}, 'key': 'value'}, 'active': True, 'created_at': '2026-03-28T19:49:16.642779', 'name': 'Test Record', 'value': 42}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'name': {'value': 'Updated Record', 'type': 'String'}, 'data': {'type': 'String', 'value': 'aGVsbG8gd29ybGQ='}, 'active': {'value': True, 'type': 'Boolean'}, 'price': {'type': 'Float', 'value': 99.99}, 'value': {'type': 'Integer', 'value': 100}, 'id': 'tMJ1mv6nSokhC3LnDF3tZJzLUMN7MMa8ZlIV-FV15FAvMdIb6p-X9t5zg3_p2-mwmtH_cZ242SnMwdbRZzqz3w', 'categories': {'value': ['electronics', 'computers'], 'type': 'Array'}, 'created_at': {'value': '2026-03-28T19:49:16.642779', 'type': 'String'}, 'user_id': {'value': '550e8400-e29b-41d4-a716-446655440000', 'type': 'String'}, 'tags': {'type': 'Array', 'value': ['tag1', 'tag2', 'tag3']}, 'metadata': {'value': {'nested': {'deep': True}, 'key': 'value'}, 'type': 'Object'}, 'embedding': {'type': 'Array', 'value': [0.1, 0.2, 0.3, 0.4, 0.5]}}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: zMk-8BdxPC6py4gWyrqmrpsJkRrrqxa8r2-kkxiIhqraapDZ1eibKonzvqzJEMggxpPrqrR4codHReSJWaRd-Q

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
Collection created with first record: "jYAS4nBxTyf-6WUEli1_diAt1g0-7iEkpWaI5bQH1ru2ikG7FVhhvjmzdDGn7Bi3ZebSAhWz1xMlONJ78znHgA"

=== List Collections ===
Total collections: 13
Sample collections: ['chat_configurations__ek0_testing', 'ws_ttl_test', 'scripts__ek0_testing', 'chat_messages__ek0_testing', 'ttl_cache']

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
Created Alice: $1000 - ID: nhoA-Bj3vCoTmOgn9FR6Bv4WqLbdKFz_WRmmkhxgYFaEbj96rLw3pogq4GIdHf2RmZ1b5rb8JsS-_9feui60tA
Created Bob: $500 - ID: x8i3nyQmVyMZ4gA-o9_mqxhPF2qHMSvrORz-Vd-s6c6JIU23G3VhDMPnfnITL82PXNFVZyl9GMN1gtDXEYt44Q

=== Example 1: Begin Transaction ===
Transaction ID: 7873b47f-d32d-4c29-bdea-859291288ae1

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 27a31446-baab-4a28-a21f-74c0814e48b5
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
  1. Score: 25.740, Matched: email.value, name, name.value, email
  2. Score: 12.540, Matched: name.value, name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title, title.value, bio, bio.value
  2. Score: 26.400, Matched: title.value, bio, title, bio.value
  3. Score: 26.400, Matched: bio.value, title.value, title, bio
  4. Score: 26.400, Matched: title.value, title, bio, bio.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title, title.value, bio.value, bio
  2. Score: 39.600, Matched: bio, bio.value, title, title.value
  3. Score: 39.600, Matched: title.value, title, bio.value, bio
  4. Score: 39.600, Matched: bio, bio.value, title.value, title

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio.value, bio

5. Search with stemming and exact match boosting:
Found 2 results (matches: run, running, runs)
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.794
  2. Score: 0.761
  3. Score: 0.748

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.397, Matched: title, content.value, content, title.value
  2. Score: 1.374, Matched: content.value, title.value, content, title
  3. Score: 0.380, Matched: 

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
✓ Inserted document: knRt6NQxICH1R1lrpVK0Zc6sxj2OMMhqI1cJC-AraJKKXbElJ29YBdhhJtYD-GscGt67CtCE7lyseJuRGjQysQ

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: kJAwzMy-Zr_HuLSgg9vuq7pyrpQpVxfiYnRgj8kZdxxF8o07QVSXL6Tqjr0louRBeENPpeGLPxwJSHQ8hCJOuA

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
✓ Inserted document with TTL: pBsd06CYP45sc5LZoWxSnnCOXnUQ6w1MLwgxeBKzQg4AdxUPn7l7m-5wAwXZ-5DT9n_7yCTsGaRF5lku-bgRzA

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
✓ Edge cache script created: _Im6JtjzTeiVqXIBxZqQMWZKZC51Vu3kG5060HaoXhvVJIjTxoeCKzLIMiB7nLEunNxF9WNg5FJ-Yfwrnvi89A

Call 1: Cache miss (fetches from API)
Response time: 569ms
Result: {
  "records": [
    {
      "value": {
        "current": {
          "interval": 900,
          "temperature_2m": 3.5,
          "time": "2026-03-28T23:45"
        },
        "current_units": {
          "interval": "seconds",
          "temperature_2m": "\u00b0C",
          "time": "iso8601"
        },
        "elevation": 32.0,
        "generationtime_ms": 0.016570091247558594,
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
Response time: 40ms (14.2x faster!)
Result: {
  "records": [
    {
      "value": {
        "current": {
          "interval": 900,
          "temperature_2m": 3.5,
          "time": "2026-03-28T23:45"
        },
        "current_units": {
          "interval": "seconds",
          "temperature_2m": "\u00b0C",
          "time": "iso8601"
        },
        "elevation": 32.0,
        "generationtime_ms": 0.016570091247558594,
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

✅ Script saved: xbMTwRjnj8mh217Ty7_mvmWvZcCCtiTF7Za3veYidiiizhOj8zR6PZgKZIBl5Fe4QmqMrZCbBM_JR_duJiWVoQ
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved: jg2H1MChcknNNV_4E_4Au6n_zLseFsx7HICl83aJUjaHPH_mRgFuwGAiBnUw4oeFxo87Kv1EB7YBGntnQmtz8A
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: rJnywIfQOBx4jjQGpaKKoyJGaNuMqDsW2UgYIXncZgrg_i2syuMER93gX_CqeKV22tclmEyDaZ21VgSNbD2UOQ
📊 Statistics: 2 groups
   {'avg_score': 50.0, 'count': 5, 'status': 'inactive'}

   {'avg_score': 60.0, 'count': 5, 'status': 'active'}

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
   ⏱️  Duration: 38.2ms
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
   ⏱️  Duration: 38.6ms
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

✅ Script saved: 9CMjG2PbupNCEUgbGrg1qCK1DHZf1stwKMobwzgH8aZv8_Nfgda29o6RZjuActsNCHQES4ABDz9bXM44Hdd1MQ
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

✅ Inserted order: 3RQXOJXiSJh6rH1XBrIWxhNWMRbQFpTMcdhvmqmFTnJM-zHBcEJTTSYzHfE3P3Mmi_uHGZ8dwEaRqKprrjAyAw
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

✅ Inserted order: zjgq6EVKFNSgbFgHt5TVwwISiMaVF917sTNU32G1x1uHUMUyq9wumyP0KHKnVo78G5bdUAtArgOF2fnnNQrNtg
✅ Cached order status
📊 Quick status lookup: {'value': '{"status":"processing","updated_at":"2026-03-28T23:49:19.365590+00:00"}'}

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
✓ Created SWR script: fetch_api_user_py (VwQXgal6qYtgijtfG_j0aoBl7UboNQsB5hMddl-vgGSYj-tCkGzkDm6t0kNIMfsOZJaXd-2EvWn_tH4X-KhIzw)

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
Response time: 39ms (served from cache)
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
✓ Created native SWR script: github_user_native (AUceEZNiKeKDPGiIPnPaThvet78mlAFLrawx9VwdF2Oz0gTmz69YaTMlQuh_vF9PWgyxfiKcl3LczOeefzx3RQ)

First call (cache miss - will fetch from GitHub API):
  Response time: 133ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 24ms
  Speedup: 5.6x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (ykp3RxfkGqPy-aV4IgwlHFPGcmB8kzUFCLUy96qWcH8SFN32mln-9NqM3dIrPvVbfqOnf2FNEETLRM_8ot4BYg)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (zrCjqHvlf4xWTpkX6LZ5dJM49Cz3L5KlH13QOZuvG6FCyFiZLAwz-vfQ9tAxKJcAs4i4UUhZYSsJjnwozc-4SQ)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (zvIZ2kFZcCw5BvnNGx-og0lzmapxOwxfuHfLTTqyaUjpUfCknVJji2MmoTd_JEtI33HkFo9o03QZQ3pnb705uQ)
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
   Vector databases offer several benefits, including:

1. **Efficient Similarity Search**: They enable fast retrieval of similar items based on vector representations, ideal for tasks like recommendation systems.

2. **Handling High-Dimensional Data**: They manage complex, high-dimensional data effectively, such as images, text embeddings, and more.

3. **Scalability**: Many vector databases are designed to scale horizontally, accommodating large datasets without performance degradation.

4. **Real-time Querying**: They support real-time similarity searches, which is crucial for applications like chatbots and dynamic user experiences.

5. **Integration with Machine Learning**: They facilitate seamless integration with machine learning workflows, optimizing embedding retrievals.

6. **Flexible Data Models**: Vector databases can handle various data types, including structured and unstructured data.

7. **Enhanced Accuracy**: By using advanced algorithms for vector search, they improve the accuracy of results in finding relevant items.

8. **Support for Multi-modal Data**: They can manage and query across different data modalities, such as text, images, and audio.

These features make vector databases particularly valuable in AI and machine learning applications.
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
   inactive: 3 users
   active: 7 users
⏱️  Execution time: 0ms

📝 Example 3: Average Score by Role

✅ Script saved
📊 Average score by role:
   {'avg_score': 70.0, 'count': 7, 'role': 'user'}
   {'avg_score': 20.0, 'count': 3, 'role': 'admin'}
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
   1. Getting Started with ekoDB (Database)
   2. Natural Language Processing (AI)
   3. Introduction to Machine Learning (AI)
   4. Vector Databases Explained (Database)
   5. Database Design Principles (Database)
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
✓ Created session: pkmE2E5xs52sdcOVdKgIqWPVAlHqOnXmvl-MXprvOEhMLgAYzuH2XSnmx3V8T12WZMq9xqrfTcTglJQ1Zx4k5A

=== Sending Chat Message ===
Message ID: P2ay3EJ2P9-dmlXVuUwc83RCu5C_sfyk_ZjnYUI90MFfUrnUiyEMzW1xyLqWa87I0Cgc39VaBcV3ufEQ-Mn1Jg

=== AI Response ===
Here are the available products along with their prices:

1. **ekoDB Cloud**
   - Description: Fully managed cloud database service product
   - Price: $499

2. **ekoDB**
   - Description: A high-performance database product with AI capabilities
   - Price: $99

3. **ekoDB Pro**
   - Description: Enterprise edition product with advanced features
   - Price: $299

Execution Time: 1817ms

=== Token Usage ===
Prompt tokens: 1298
Completion tokens: 91
Total tokens: 1389

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: AY7Fr3fvV83ax7wkv6K-tBliPSBbCwABlJgLLa8xv0qXUyKrYjk0g4f4XrhneDxvRyDR_ToCd7Ei05TvoqVSFQ

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** High-performance database product
- **Price:** $99

If you need more information or have other questions, feel free to ask!

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['role', 'token_usage', 'id', 'updated_at', 'llm_provider', 'context_snippets', 'content', 'llm_model', 'chat_id', 'created_at'])
Debug: First message role: {'type': 'String', 'value': 'assistant'}
=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of ekoDB is $99. If you have any further questions or need more information, feel free to ask!

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: OUGtIBJszHl4Zgo0ENTpTwyDMiuwPxQJxonyTQx-Reb-PqdBsA46sl1sJvJjUxkaNkDjoy2bL1pg1hFCFOcazQ
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
✓ Created session: QXOdFl0w4yhb1lF1zrevRN2vRu-5DRX9LZ6plyVt-UCxEnXgZTHp7c87IvQFBE69ZvueD4h8qjQvtxzoUsiKJw

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name**: ekoDB
- **Price**: $99

If you need more information or want to explore additional products, please let me know!

✓ Message 2 sent
  Response: The price of ekoDB is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 35PIdX_nDYoy19Xk8ygPd2IR_WTFhCg-SrMIlyFoakU8KIoz672BTnNDxqtQCtLNJRoucE2r9elM9u5_m-Ovsw
  Parent: QXOdFl0w4yhb1lF1zrevRN2vRu-5DRX9LZ6plyVt-UCxEnXgZTHp7c87IvQFBE69ZvueD4h8qjQvtxzoUsiKJw

=== Listing Sessions ===
✓ Found 7 sessions
  Session 1: 35PIdX_nDYoy19Xk8ygPd2IR_WTFhCg-SrMIlyFoakU8KIoz672BTnNDxqtQCtLNJRoucE2r9elM9u5_m-Ovsw (Untitled)
  Session 2: QXOdFl0w4yhb1lF1zrevRN2vRu-5DRX9LZ6plyVt-UCxEnXgZTHp7c87IvQFBE69ZvueD4h8qjQvtxzoUsiKJw (Untitled)
  Session 3: OUGtIBJszHl4Zgo0ENTpTwyDMiuwPxQJxonyTQx-Reb-PqdBsA46sl1sJvJjUxkaNkDjoy2bL1pg1hFCFOcazQ (Untitled)
  Session 4: pkmE2E5xs52sdcOVdKgIqWPVAlHqOnXmvl-MXprvOEhMLgAYzuH2XSnmx3V8T12WZMq9xqrfTcTglJQ1Zx4k5A (Untitled)
  Session 5: 9Jr3Qe-CxxNhxTsCRmrM9rNvW0r0x2fzl446Rb9ns4tzaWZvzDxApSQxUtkA8cJvKf_fROyiqNfWYwWM4ahQmw (Untitled)
  Session 6: pFPnqAtp08oCL4UUqkRYoHA97h5__QHkMVXMvvIXdGr734rhuDDvY0KBEmhOVYCwLeX-bA_SNR8bLQydscbghA (Untitled)
  Session 7: XYf4K4p27uG-Khw_d6gCgyk8l5FE5HJ_sBKkrVPC_B8yIv1SlstejAemAqE_4HfUcKJn6PifBgFYFG6Fc0qMxQ (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: 35PIdX_nDYoy19Xk8ygPd2IR_WTFhCg-SrMIlyFoakU8KIoz672BTnNDxqtQCtLNJRoucE2r9elM9u5_m-Ovsw

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Dict Creation ===
✓ Created record with plain dict: {'id': 'nbAYgGyRU1PwX5rapxTzJV43fZEOAKBcuBjNYY7zQ5K7t5UWswtu-jN9DU6oKylGeb0GJRa5cdfnjsEgf_-KnA'}

=== Upsert Operation ===
✓ Upsert (update existing record): nbAYgGyRU1PwX5rapxTzJV43fZEOAKBcuBjNYY7zQ5K7t5UWswtu-jN9DU6oKylGeb0GJRa5cdfnjsEgf_-KnA
✓ Inserted second record: JSk0vnWmRyItWs5t3MWOeFcescWJ3WqbBqmC_oCwDfZqE6Ml4DvOxM1VFva-FTlNMSSPkLaJxF4KP7k48NapfA
✓ Upsert (update second record): JSk0vnWmRyItWs5t3MWOeFcescWJ3WqbBqmC_oCwDfZqE6Ml4DvOxM1VFva-FTlNMSSPkLaJxF4KP7k48NapfA

=== Find One Operation ===
✓ Found user by email: {'age': {'value': 29, 'type': 'Integer'}, 'name': {'value': 'Alice Johnson', 'type': 'String'}, 'id': 'nbAYgGyRU1PwX5rapxTzJV43fZEOAKBcuBjNYY7zQ5K7t5UWswtu-jN9DU6oKylGeb0GJRa5cdfnjsEgf_-KnA', 'email': {'value': 'alice.j@newdomain.com', 'type': 'String'}, 'active': {'type': 'Boolean', 'value': True}}
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
   Inserted with ripple: {'id': 'jOImT9h5ZzgrsxQQthtybU9m73YcTFptT4UPckHAkclm0BwbLTpaSq5gm948rdWHs-F3wPgLjcUBYR8-UOd80A'}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {'id': 'lLacRyJOYyiIjz115l6hH-BCrWaLVr2baXWRFDjQ8E8ILWjTtAZLm_WZBYwjitOz9qh9doG6oHU6KQF0VaimwA'}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {'name': {'type': 'String', 'value': 'Product 1'}, 'id': 'jOImT9h5ZzgrsxQQthtybU9m73YcTFptT4UPckHAkclm0BwbLTpaSq5gm948rdWHs-F3wPgLjcUBYR8-UOd80A', 'price': {'type': 'Integer', 'value': 150}}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {'name': {'value': 'Upsert Product', 'type': 'String'}, 'id': 'jOImT9h5ZzgrsxQQthtybU9m73YcTFptT4UPckHAkclm0BwbLTpaSq5gm948rdWHs-F3wPgLjcUBYR8-UOd80A', 'price': {'value': 500, 'type': 'Integer'}}

✅ All bypass_ripple operations completed successfully!
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields (id, name, email only)
  Found 3 active users
  Fields returned: ['name', 'id', 'email']
  First user: {'type': 'String', 'value': 'Alice Johnson'} <{'value': 'alice@example.com', 'type': 'String'}>

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ['status', 'email', 'bio', 'id', 'user_role', 'created_at', 'name', 'age', 'avatar_url']

Example 3: Complex query with projection (active users, ages 18-65)
  Found 3 active users (ages 18-65)
    - {'value': 'Dave Brown', 'type': 'String'} (age {'type': 'Integer', 'value': 45})
    - {'type': 'String', 'value': 'Alice Johnson'} (age {'value': 30, 'type': 'Integer'})
    - {'type': 'String', 'value': 'Bob Smith'} (age {'type': 'Integer', 'value': 25})

Example 4: Query inactive users with profile fields
  Found 1 inactive users
    - {'value': 'Carol White', 'type': 'String'}: {'type': 'String', 'value': 'Manager'}

Example 5: Compare full vs projected data
  Full query:
    - 12 fields per record
    - Fields: ['secret_token', 'created_at', 'id', 'avatar_url', 'password', 'email', 'status', 'name', 'age', 'user_role', 'api_key', 'bio']
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
Created user function with ID: toHZ34wgW9Xjb5bETP210Q6Q47WAAJGTbP5XYsloy_hkIAPrxPw_6mr2NcIeGE9Gq2TShevbAMNvEDAFE9k4Pw

=== Get User Function ===
Retrieved: get_active_users_py - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 25 user functions:
  - validate_user: Check if user exists
  - get_active_users_updated: Get Active Users (Updated)
  - get_user_wrapper: Wrapper that calls fetch_user
  - cache_api_call_py: Cache External API Call
  - get_verified_user: Get verified and validated user
  - get_active_users_updated: Get Active Users (Updated)
  - get_active_users: Get Active Users (Updated)
  - swr_user: SWR pattern for user data (KV-based)
  - get_active_users_paginated: Get Active Users (Paginated)
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_user: Fetch user by code
  - get_active_users: Get Active Users (Updated)
  - fetch_slim_user: Validate and slim down user
  - fetch_slim_user: Validate and slim down user
  - get_verified_user: Get verified and validated user
  - fetch_and_store_user: Fetch user from API and cache in KV
  - swr_user: SWR pattern for user data (KV-based)
  - get_user_wrapper: Wrapper that calls fetch_user
  - get_users_by_status: Get Users By Status
  - get_users_by_status: Get Users By Status
  - get_active_users_paginated: Get Active Users (Paginated)
  - get_high_scoring_active_users: Get High Scoring Active Users
  - fetch_user: Fetch user by code
  - get_active_users_py: Get Active Users
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
✅ [32mPython client examples complete![0m
