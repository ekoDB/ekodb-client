make test-examples-python
🧪 [36mRunning Python examples (direct HTTP/WebSocket)...[0m
=== Simple CRUD Operations (Direct HTTP) ===

✓ Authentication successful

=== Insert Document ===
Inserted: {'id': 'L9cpBD0Eljx2Ncf21z-_jzCQ6vf2MRIHkp4lvfrqdFGz-up_gg7uugVcVhFK3J9n6JJagH1DJjGG_rFo-ojnaw'}

=== Find by ID ===
Found: {'id': 'L9cpBD0Eljx2Ncf21z-_jzCQ6vf2MRIHkp4lvfrqdFGz-up_gg7uugVcVhFK3J9n6JJagH1DJjGG_rFo-ojnaw', 'name': {'value': 'Test Record', 'type': 'String'}, 'value': {'value': 42, 'type': 'Integer'}, 'active': {'value': True, 'type': 'Boolean'}}

=== Find with Query ===
Found documents: [{'active': {'type': 'Boolean', 'value': True}, 'value': {'type': 'Integer', 'value': 42}, 'name': {'value': 'Test Record', 'type': 'String'}, 'id': 'L9cpBD0Eljx2Ncf21z-_jzCQ6vf2MRIHkp4lvfrqdFGz-up_gg7uugVcVhFK3J9n6JJagH1DJjGG_rFo-ojnaw'}]

=== Update Document ===
Updated: {'value': {'value': 100, 'type': 'Integer'}, 'active': {'type': 'Boolean', 'value': True}, 'name': {'type': 'String', 'value': 'Updated Record'}, 'id': 'L9cpBD0Eljx2Ncf21z-_jzCQ6vf2MRIHkp4lvfrqdFGz-up_gg7uugVcVhFK3J9n6JJagH1DJjGG_rFo-ojnaw'}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: d2VJG5iv4Y2GM8EF5v5Bvs8hZucupbWnljtU8V09E6kHsVSL76g8IaS5XdJ9hRAvPBx73kHkr6R-756nGUvxHg

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "payload": {
    "data": [
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "ggUkVE8p-4-sgFujbNnfoo55fNFXQjXjuC4CFyYjUOz6nF6tI0-pZ_LBzcVVwNTxaLqn3-UX4dKYH11ySN0p4A",
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
        "id": "d2VJG5iv4Y2GM8EF5v5Bvs8hZucupbWnljtU8V09E6kHsVSL76g8IaS5XdJ9hRAvPBx73kHkr6R-756nGUvxHg",
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

✅ Script saved: KPBv7JXT5qmEhVw3uFi45QSVAnE3axQWH4Tb4J7Jeu5I-qJwlk3waHXfzEiuTJWb6C1ikfNOTnoFgA8M2h0E1A
📊 Found 30 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: A3xs7UqCZfx7yUIZy4HdfuS-IHE-fy1E2DnhwZMJ-zcXKyit-BnYHwawjXDMKDtpg1tQ6Ux6swPvk3GtiseEDg
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: 9z-ifzRfhMLtn5iwJdnvH1b22E5zbCFrDwJkSYFkl-VC57Peq1_L_5eJaEmrXKHLm6m0bbFof4yx-xO3UCShxQ
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {'status': 'active', 'count': 15, 'avg_score': 60.0, 'max_score': 100}
   {'max_score': 90, 'count': 15, 'avg_score': 50.0, 'status': 'inactive'}

📝 Example 4: Function Management

📋 Total scripts: 8
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
cache:product:1: {'name': 'Product 1', 'price': 29.99}
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
Collection created with first record: Qfzh6EF258UEJGL2tDDTXY5tOdF2-JWmgiQPfG3OHV42Qd0Tzy-o1ZRZcjS_e2kGjp_p0NclMkdbNIbpRyh_pA

=== List Collections ===
Total collections: 11
Sample collections: ['ttl_cache', 'users', 'websocket_test', 'chat_messages__ek0_testing', 'test_collection']

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: False

✓ All collection management operations completed successfully
✓ Authentication successful

📋 Getting original configuration...
   Original durable_operations: False


============================================================
🔥 TEST 1: Original Config (durable=False)
============================================================

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: e_CzE76gy-iHzi4_-y-ZqtW5FgWLQwTU3yJuI81oXbZ1ZA2mVcnNakkTBELCst31I3dsW2KhJ9r0gUjCgTQfRQ
Created Bob: $500 - ID: nddoH0ndf-3OB--MGWeDzJgphSL9yCmvxQRvZKU2T-uXQrokxz9DOFMMtUAylhG7B1TOqv3WvQUW6md7RAzYpw

=== Example 1: Begin Transaction ===
Transaction ID: 832d3e33-7c8c-4bb3-b51d-fbb605341bb9

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: $800
Bob: $700

=== Example 5: Rollback ===
New transaction: 20b61822-5769-405d-bc83-74ac4bde0513
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: sfS-nBmWTm_Wi80-QC5sdaWYoo9Os3UpSPPyb9xcXZ-TGLonMHzjXRHpT9M52TgqXcOHwRVwjbsx-ouXE-J8yg
Created Bob: $500 - ID: BXX2wsVqKVwrD5BWgBF5IwtYwNmqupSEN-A_ggDzCHeJMyx-VVYTBK9YsINBGp0T1s5lHm9hrG-lG4-dbWlwKQ

=== Example 1: Begin Transaction ===
Transaction ID: c8dfe23b-d7c9-4d76-9911-1a5b7d54ce36

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: $800
Bob: $700

=== Cleanup ===
✓ Deleted test accounts


🔄 Switching to NON-DURABLE mode...
   ✓ Config updated: durable_operations=false


============================================================
🔥 TEST 2: Non-Durable Mode (durable=false)
============================================================

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: rjgZnM-wNOLAnITFU0oaveeTYbgl9lVJ_i_OsejkiZDGefUl3rXQkTUnmE3RktsN1pil3FiIy1U80MeUAzpyNA
Created Bob: $500 - ID: cd83vRnzkeSyIBa9vQINnzJXb6kDeQrfyzXbbr9cm_oDMz_iMhaCnXObCwLFTjCw9wr5max4QLrFWQ5UZWIaiw

=== Example 1: Begin Transaction ===
Transaction ID: b7e58552-f391-4979-99ad-cd8eee8020d9

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: $800
Bob: $700

=== Example 5: Rollback ===
New transaction: 1adf9671-3492-49bc-a9cb-b5b9e84b85b5
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: hMnsp-cKHmhXAHi96W6B2Nm64YxgBln4dw_OO0GU9ZkmZe9Oh579y_WPfenP59FY_fgutN_p5F2yc4PUNLtQ-A
Created Bob: $500 - ID: uXRshnYu77TKhjPYuaNAk8HCzsjxHEsUJU_PNIey6agKDEGW7SK7jHcQ5TVIdNivJmImbxPFankGMya0g7rv3w

=== Example 1: Begin Transaction ===
Transaction ID: 8c85499c-dd1c-47f4-9407-ee564cff109e

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: $800
Bob: $700

=== Cleanup ===
✓ Deleted test accounts


🔄 Switching to DURABLE mode...
   ✓ Config updated: durable_operations=true


============================================================
🔥 TEST 3: Durable Mode (durable=true)
============================================================

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: v5Km4S-cfp8pb-dnCQ6oPAmkqLnXZcWk3xoa6SkdwrNfCLJQynFz53bFzr8keJzW7bxBONz8kMIR4QA0tr1OlQ
Created Bob: $500 - ID: qTAg_sgYqRtTNHyhYAWWz49Z5jQPEntRhkDBNn3eiT49n5YY6-dNULzklpEGQL_zPZ4_V8F2Pptc6aLwC2q_Sg

=== Example 1: Begin Transaction ===
Transaction ID: e2bd48ff-3d34-4c32-8269-633f63790d42

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: $800
Bob: $700

=== Example 5: Rollback ===
New transaction: c53f9483-8cf0-42a6-bf07-148ea8c10800
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: q-le2yU1IFfFKzEcyDwAQ8qF-GuKSbZDM1rrBjCL-U6EwqGYZ8GZudyeSbBNTM2XWaVrg786KuGlSLY-_MJPXw
Created Bob: $500 - ID: u8Qz67d1BlaBXeDyBEk9rZdr5bq9Xfecc5FDE_YbzAAKuqSEGPd5VMC33Ou-M-WPf1nLaHycT-2zZ1TBmkAFbA

=== Example 1: Begin Transaction ===
Transaction ID: c7ddea9d-268b-4df1-8b10-453ee480b1c8

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: $800
Bob: $700

=== Cleanup ===
✓ Deleted test accounts


🔄 Restoring original configuration...
   ✓ Config restored: durable_operations=False


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
   ✅ Script saved: T9tSacurfr9Ifa0nyRRBIMMoOc4UE7p8dOPgI-fLY7mhq0Qnf2FunVSsjZPJfFk8oHC-QnhJ_OPeLDdXoTUlEA

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: NVBaoRYSGbZ6UPndQjmtFQ9LwN7nGcWHJaDDc58eqDCYW283TWam65FO3VJeyni9u-LP7_eBy7nuCIBZ5YPIhA
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}
   📋 Email: {'value': 'alice@example.com', 'type': 'String'}
   📋 Status: {'type': 'String', 'value': 'pending'}
   📋 Credits: {'type': 'Integer', 'value': 0}

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: vKkiZCHouPMY6lB835UoYI1oELsOM_RQaLQl0rn-6b2uyyfWvj7rbEvO9M5jKXVwWaOGsjKZd_rprqaI3jjKZA

2️⃣ Calling Script (Query + Update + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Status updated to: {'type': 'String', 'value': 'active'}
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}

============================================================
📝 Script 3: Query + Update Credits + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: 4aDZAZuAZCLYMbuitVrVFpZAU_e8u1C0ECQZrxBs9RUzSzSiReO2Cb5zeZgL7mG4JFFLPKJmhWB0dqKJVVeo-Q

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
   ✅ Script saved: bGJBLRkFUgqCtnMe1Or2LlJoHwg98W5sFrQ73OJHP1xKcRqDofs-fohJ86p0191xIpGrjQ8W-1ODO3v8QqnvNA

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: T9tSacurfr9Ifa0nyRRB...
   ✅ Deleted script: vKkiZCHouPMY6lB835Uo...
   ✅ Deleted script: 4aDZAZuAZCLYMbuitVrV...
   ✅ Deleted script: bGJBLRkFUgqCtnMe1Or2...
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
  Output: Document ID = MeB5hdZHxsKnsFHpjxeyb_Zel7T1nyGF9A3E5kak7-CWdhymBaTwMvOfvddVnBgBxm0EMYQVu5qQJEixEeN90Q
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: find_by_id(MeB5hdZHxsKnsFHpjxeyb_Zel7T1nyGF9A3E5kak7-CWdhymBaTwMvOfvddVnBgBxm0EMYQVu5qQJEixEeN90Q)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: find_by_id(MeB5hdZHxsKnsFHpjxeyb_Zel7T1nyGF9A3E5kak7-CWdhymBaTwMvOfvddVnBgBxm0EMYQVu5qQJEixEeN90Q)
  Output: Error (expected) - Find failed: Record not found
  ✓ PASS: Document expired (not found error)

═══════════════════════════════════════════════════════════
TEST 2: Multiple Documents with Different TTLs
═══════════════════════════════════════════════════════════

[Step 1] Insert 3 documents with different TTLs
  Inserted: 3s TTL=wpnOJF9l, 10s TTL=q8ds6yD7, Permanent=GNHZft8k
  ✓ PASS: All documents inserted

[Step 2] Wait 5 seconds (3s doc should expire)
  ✓ PASS: 3s TTL document expired
  ✓ PASS: 10s TTL document still exists
  ✓ PASS: Permanent document still exists

[Step 3] Wait 7 more seconds (10s doc should expire)
  ✓ PASS: 10s TTL document expired
  ✓ PASS: Permanent document still exists

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
  Output: Document ID = oSsSVC9TYHx15JyD0OAN2TTq8_66tICTFSR9Md9MsHup9NTwvxFh_TjoutSkCPUjj8dnDTWP0LkjBlOynY_qkw
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: find_by_id(oSsSVC9TYHx15JyD0OAN2TTq8_66tICTFSR9Md9MsHup9NTwvxFh_TjoutSkCPUjj8dnDTWP0LkjBlOynY_qkw)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: find_by_id(oSsSVC9TYHx15JyD0OAN2TTq8_66tICTFSR9Md9MsHup9NTwvxFh_TjoutSkCPUjj8dnDTWP0LkjBlOynY_qkw)
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
🐍 Not using a specific python interpreter
💻 Using `MACOSX_DEPLOYMENT_TARGET=11.0` for aarch64-apple-darwin by default
    Finished `release` profile [optimized] target(s) in 0.07s
📦 Built wheel for abi3 Python ≥ 3.8 to /Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.7.1-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel for current platform...[0m
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.7.1-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.7.1
    Uninstalling ekodb_client-0.7.1:
      Successfully uninstalled ekodb_client-0.7.1
Successfully installed ekodb-client-0.7.1
✅ [32mPython client package built and installed![0m
🧪 [36mRunning Python client library examples...[0m
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {'id': '-o6rjOx0bvwBc1yxgk4ZESk6sizVlFWxpmst1LpEd5w4h3yRnoM1b-MWcPg8S7X7cVg5rvaHj6kpWmX31TA2gw'}

=== Find by ID ===
Found: {'metadata': {'type': 'Object', 'value': {'nested': {'deep': True}, 'key': 'value'}}, 'price': {'type': 'Float', 'value': 99.99}, 'active': {'type': 'Boolean', 'value': True}, 'data': {'type': 'String', 'value': 'aGVsbG8gd29ybGQ='}, 'value': {'value': 42, 'type': 'Integer'}, 'tags': {'value': ['tag1', 'tag2', 'tag3'], 'type': 'Array'}, 'embedding': {'type': 'Array', 'value': [0.1, 0.2, 0.3, 0.4, 0.5]}, 'id': '-o6rjOx0bvwBc1yxgk4ZESk6sizVlFWxpmst1LpEd5w4h3yRnoM1b-MWcPg8S7X7cVg5rvaHj6kpWmX31TA2gw', 'name': {'value': 'Test Record', 'type': 'String'}, 'categories': {'type': 'Array', 'value': ['electronics', 'computers']}, 'user_id': {'type': 'String', 'value': '550e8400-e29b-41d4-a716-446655440000'}, 'created_at': {'value': '2026-01-04T17:29:58.710058', 'type': 'String'}}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): True
  price (Decimal): 99.99
  created_at (DateTime): 2026-01-04 17:29:58.710058
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): ['tag1', 'tag2', 'tag3']
  metadata (Object): {'nested': {'deep': True}, 'key': 'value'}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): ['electronics', 'computers']
  data (Bytes): 11 bytes
Plain record: {'metadata': {'nested': {'deep': True}, 'key': 'value'}, 'price': 99.99, 'active': True, 'data': 'aGVsbG8gd29ybGQ=', 'value': 42, 'tags': ['tag1', 'tag2', 'tag3'], 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5], 'id': '-o6rjOx0bvwBc1yxgk4ZESk6sizVlFWxpmst1LpEd5w4h3yRnoM1b-MWcPg8S7X7cVg5rvaHj6kpWmX31TA2gw', 'name': 'Test Record', 'categories': ['electronics', 'computers'], 'user_id': '550e8400-e29b-41d4-a716-446655440000', 'created_at': '2026-01-04T17:29:58.710058'}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'created_at': {'value': '2026-01-04T17:29:58.710058', 'type': 'String'}, 'value': {'value': 100, 'type': 'Integer'}, 'metadata': {'value': {'nested': {'deep': True}, 'key': 'value'}, 'type': 'Object'}, 'price': {'value': 99.99, 'type': 'Float'}, 'tags': {'value': ['tag1', 'tag2', 'tag3'], 'type': 'Array'}, 'name': {'type': 'String', 'value': 'Updated Record'}, 'categories': {'value': ['electronics', 'computers'], 'type': 'Array'}, 'user_id': {'type': 'String', 'value': '550e8400-e29b-41d4-a716-446655440000'}, 'data': {'value': 'aGVsbG8gd29ybGQ=', 'type': 'String'}, 'embedding': {'type': 'Array', 'value': [0.1, 0.2, 0.3, 0.4, 0.5]}, 'active': {'value': True, 'type': 'Boolean'}, 'id': '-o6rjOx0bvwBc1yxgk4ZESk6sizVlFWxpmst1LpEd5w4h3yRnoM1b-MWcPg8S7X7cVg5rvaHj6kpWmX31TA2gw'}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
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

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: {'value': '{"userId":123,"username":"john_doe"}'}

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: {'value': '{"name":"Product 1","price":29.99}'}
cache:product:2: {'value': '{"name":"Product 2","price":39.989999999999995}'}
cache:product:3: {'value': '{"name":"Product 3","price":49.989999999999995}'}

=== KV Exists ===
Key exists: True

=== KV Find (Pattern Query) ===
Found 3 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 4

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: False

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: "Ima8WlHg8cnB7eTw92BBag5QsNO16LuOBUOek_b7b_n8pzGW6N5DsOegP4BiBB_ypAe9wlVCaBsSJI0T8b2WMA"

=== List Collections ===
Total collections: 10
Sample collections: ['ttl_cache', 'websocket_test', 'chat_messages__ek0_testing', 'test_collection', 'scripts__ek0_testing']

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: False

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: A4Z7R535QDG6fZi26kOWDtToIo_Np80jxl9wommmpSmf7br9VRtGQaAShTlf3JhbCVIg39hmZP8MReV6KS0WfQ

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: XkP9XmlOZIcFwvQnNAWDQRGPFATdVl64s4Pg1F-WQ5LASYdoX_8GIsNWIdla2TebfJpXG-2Zg_4Dw0q9u_h_Xw

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

=== Inserting Test Data ===
✓ Inserted test record: DsLAlFAk8qvjYXpi0BGSzqX51ul77RA5mR_QqNvnFIMwD2dpWeOwPt_org1WZCMQl5zZtCG1s1FfmicxaE88Zw

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: 4qOBs_c7Xtd4aScXUA-9PAYgVswNoNPLx4rL9xlzhThC7VtUzRt1OjhuyCOwhqJRsg6oivOOzL7OBmD1Bl1XuA

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 5 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
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
  1. Score: 25.740, Matched: email.value, name, email, name.value
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: bio, title, title.value, bio.value
  2. Score: 26.400, Matched: title, bio, title.value, bio.value
  3. Score: 26.400, Matched: title, bio, title.value, bio.value
  4. Score: 26.400, Matched: bio, title.value, bio.value, title

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title.value, bio, title, bio.value
  2. Score: 39.600, Matched: bio.value, title, title.value, bio
  3. Score: 39.600, Matched: title, title.value, bio, bio.value
  4. Score: 39.600, Matched: bio, bio.value, title.value, title

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 2 results (matches: run, running, runs)
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.778
  2. Score: 0.774
  3. Score: 0.769

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.738, Matched: title.value, title, content, content.value
  2. Score: 1.142, Matched: title, content, title.value, content.value
  3. Score: 0.544, Matched: 

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
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: LJ5cZyqHoXs52cGWUxESsBowN9WD_4okFMLRbpD8-ggFUhYqKZPFaRlUDCmTo8J2LkXgnMPzPinMwwzCxLmR8w

=== Sending Chat Message ===
Message ID: tCIVlD1ch28TaM-M3mXJDWXk_jHVGbeXwHGt19m14KQFVJq2YpcliZRGMweffhmsS3bnROh8Dzqu15MhCIpHXw

=== AI Response ===
Based on the information available, there are three products available:

1. ekoDB: This is a high-performance database product with AI capabilities. It is priced at $99.

2. ekoDB Pro: This is the enterprise edition of the product with advanced features. It is priced at $299.

3. ekoDB Cloud: This is a fully managed cloud database service product. It is priced at $499.

Execution Time: 3025ms

=== Token Usage ===
Prompt tokens: 609
Completion tokens: 86
Total tokens: 695

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: j6jNCKZTN-N6CxuK9wi6SxzdbbBMKWEGWrM8AniwBQpg_SWyD86sGW2NZ5rI63Z7R7RlpXjn8nsPwHQcT3VslQ

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information available, the product that is available is "ekoDB". It is a high-performance database product and is priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: aaKLSKj40uqNyixJJzdzSUcjBDOspmlzLEWTt8fF9wYPvXhpdXARdyITA0AwQ2XhgzDVChN_UxJiool54iA60g
  Parent: j6jNCKZTN-N6CxuK9wi6SxzdbbBMKWEGWrM8AniwBQpg_SWyD86sGW2NZ5rI63Z7R7RlpXjn8nsPwHQcT3VslQ

=== Listing Sessions ===
✓ Found 6 sessions
  Session 1: aaKLSKj40uqNyixJJzdzSUcjBDOspmlzLEWTt8fF9wYPvXhpdXARdyITA0AwQ2XhgzDVChN_UxJiool54iA60g (Untitled)
  Session 2: j6jNCKZTN-N6CxuK9wi6SxzdbbBMKWEGWrM8AniwBQpg_SWyD86sGW2NZ5rI63Z7R7RlpXjn8nsPwHQcT3VslQ (Untitled)
  Session 3: LJ5cZyqHoXs52cGWUxESsBowN9WD_4okFMLRbpD8-ggFUhYqKZPFaRlUDCmTo8J2LkXgnMPzPinMwwzCxLmR8w (Untitled)
  Session 4: zgZmlmVFLWGNLEdEB26uHagZbr2ZF7802mz_wjqfZqon0azZ3gofLs6k7o8B_odODTl89yzN_di8oFEaFf0y8g (Untitled)
  Session 5: mA61Yn2joaX-S0mdSdsaajZiffk0LtVAs6i9T-bDcik-YewWD2js2py8GGeFemTIxR6pDVrFEWI6WsaCqFGwUA (Untitled)
  Session 6: DhIh4JKtzyz-P_VwbAkLSXZCjZtY0TlcbaSniFvYPnmyL6GKGRUO4aUqqhc3KeIZqzMADTi3NFtcGZeLK9UFTQ (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: aaKLSKj40uqNyixJJzdzSUcjBDOspmlzLEWTt8fF9wYPvXhpdXARdyITA0AwQ2XhgzDVChN_UxJiool54iA60g

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: nyVWDeAHuPqxSgnotyLNk9_AC-2-G8KC8UyJ7dJfngs1xbENkT2WtIqa_PuOAO9xM4mnGCwX2T9STGBfGMnC4w

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information available, the product that is available is ekoDB, which is a high-performance database product. It is priced at $99.

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['context_snippets', 'token_usage', 'chat_id', 'role', 'id', 'updated_at', 'content', 'created_at'])
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
✓ Created second session: Io01vXhsW8BkPYUCUUTLdDWRPT21aGynTNwl8dKLlJ6X24xSeN2mBCOkKES2IJnJ9F6h51vrnyjWWQLYmuyOMQ
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 4

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 3

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
🚀 ekoDB Scripts Example (Python)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: FkH0MpldIfEtEx7Guu6vQ1mFUMBfTffZ0AOIH4zMqJRSkH1ydwWe5vYONzt2M1if_NFXiyfOzdlCUhtzxEHHew
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved: MpxPQFJtzpSLL3U77ZaITLFQ-qaU98WswRgfI6IDNRPJZEpfJB8H-Pn13Eel9SuwHPoS1N2LWOtfrucFM4Gu9A
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: 5MVqI6WnUKgbTY1Fersp0gQ-HCIl9UfMH8J9zkCxE18SChZE_nBoD6v_Jyte1_9b56StYQsoz61qMTiiwhgj9g
📊 Statistics: 2 groups
   {'avg_score': 60.0, 'count': 5, 'status': 'active'}

   {'avg_score': 50.0, 'count': 5, 'status': 'inactive'}

📝 Example 4: Script Management

📋 Total scripts: 10
🔍 Retrieved script: Get Active Users
✏️  Script updated
🗑️  Script deleted

ℹ️  Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.

✅ All examples completed!
🚀 ekoDB Python Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved: CDDSlFpkoRk3ZnPqmGrHrJo3Ack6gDl_sfOVHXXU1q3cU4h_mFZk93KIMuS-RuAcM4UT9GK_BKcYTGJhzOFr2g
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
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: QylQN2h49AIkA4l8TLreKE3a_g1sJstt1X8t4LmnDNsHaYdaJLEsKR_RSM5IEIaWx7ja_NpkrHj4r3w41SRmXg
Created Bob: $500 - ID: OB1N_ViWmVN9Ej4PicD0BE-71SDG4NK5WVc7qj7CW2KL0oao7jgyTsZ5Bukac7qugxDYou7PoIBTxWC3_ydowg

=== Example 1: Begin Transaction ===
Transaction ID: 10d1cc1e-69fb-4c88-99a2-1f25c422e76e

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 5665db11-1db9-47b8-a43f-8d6d6697fbe2
Status before rollback: Active
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
✅ [32mPython client examples complete![0m
