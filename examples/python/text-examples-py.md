make test-examples-python
🧪 [36mRunning Python examples (direct HTTP/WebSocket)...[0m
=== Simple CRUD Operations (Direct HTTP) ===

✓ Authentication successful

=== Insert Document ===
Inserted: {'id': '7i9Jk6yZG84HV_2irWYpFgoR2_5qVVMUHRAYhKJfQjidxQRLu4NUeF1j7bGcBijWmBlH54gWZofQ4GVetBgZ-Q'}

=== Find by ID ===
Found: {'active': True, 'value': 42, 'id': '7i9Jk6yZG84HV_2irWYpFgoR2_5qVVMUHRAYhKJfQjidxQRLu4NUeF1j7bGcBijWmBlH54gWZofQ4GVetBgZ-Q', 'name': 'Test Record'}

=== Find with Query ===
Found documents: [{'active': {'type': 'Boolean', 'value': True}, 'name': {'value': 'Test Record', 'type': 'String'}, 'value': {'value': 42, 'type': 'Integer'}, 'id': '7i9Jk6yZG84HV_2irWYpFgoR2_5qVVMUHRAYhKJfQjidxQRLu4NUeF1j7bGcBijWmBlH54gWZofQ4GVetBgZ-Q'}]

=== Update Document ===
Updated: {'name': {'value': 'Updated Record', 'type': 'String'}, 'id': '7i9Jk6yZG84HV_2irWYpFgoR2_5qVVMUHRAYhKJfQjidxQRLu4NUeF1j7bGcBijWmBlH54gWZofQ4GVetBgZ-Q', 'active': {'type': 'Boolean', 'value': True}, 'value': {'type': 'Integer', 'value': 100}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: 1qobg-2Rq-h1oteGUGivHlWqJXt932IRY3a7Nc5Ctt0dKs5wfdgbqRO66EOezNHHx2k8Yqy0eH7nrKd18WIjPQ

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
        "id": "1qobg-2Rq-h1oteGUGivHlWqJXt932IRY3a7Nc5Ctt0dKs5wfdgbqRO66EOezNHHx2k8Yqy0eH7nrKd18WIjPQ",
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
        "id": "OxC0Iqm6dKscDmJ4lvOfrE5ULerFFSPFfDhxbdGMxPG7DjUSbfjfBCkLZHOrBWGwBrzGmKldyS-4fge4dR1o9Q",
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

✅ Script saved: 6SY73sBjHot4CFsV3-7gX0DO5YyDBujUdUwCfZr2va-2AoEoVPmDYm-Zum4FRl31b_7IONLINRyX6slgkl72PQ
📊 Found 35 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: NgJJmSP0VOpQaQWlHjV0LahNbJCVQ-hvlP1cKuJC2OhdJeN6M7z5hDw6SwAD47T2EFaTRbMRJI6nm00avzP9TQ
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: 8L-98YoT6kACG0CJRXSAoqGBnE8avz9qlfQI_-TosdK-AK_DUaOpcDTnIE7QozvQK_aAvSCawfPGpAsejjPw3g
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {'count': 15, 'status': 'active', 'avg_score': 60.0, 'max_score': 100}
   {'avg_score': 50.0, 'count': 15, 'status': 'inactive', 'max_score': 90}

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
cache:product:1: {'name': 'Product 1', 'price': 29.99}
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
Collection created with first record: Nh8Zjdzb4dypwUPRmNnoPrPQ9znuW0h92E4XetRG5NpcbRFI19AGdye7wj_Uk-SHu-ytYKZK6Th-03Itw41jIg

=== List Collections ===
Total collections: 12
Sample collections: ['products', 'batch_users', 'ttl_cache', 'chat_configurations__ek0_testing', 'test_accounts']

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
Created Alice: $1000 - ID: Z9vpXc-ME92nbckBsUtDOgsCFBUIjTp1CnEjIM4MJj85eobVzsjRqvtna-bSo7P1TrC5UUuMr4UOIogQrIHSug
Created Bob: $500 - ID: or78fZ3PIHU6UxjqVX3aT6ZmSwxwu1woB4PvITvJ2GUDiRvYgOEr5l_ohSkwBNrBH4HDBGFa3O-WkRVU9OJW8g

=== Example 1: Begin Transaction ===
Transaction ID: 8b90624d-53b3-407d-bf71-84b8d31a7820

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
New transaction: b7569ed9-6f0a-4c64-97d4-10c31efa856e
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: hNgNgl1jDFCXB8musGn8E0tqwdawyluOvmZOoR9hKrwdO8owVzoVbDka3oc7_yco_qL2RsB_UaXVSboqW7pmJg
Created Bob: $500 - ID: 75iVvt0qvzoK9kIfczHOctHbFVozwxFEMEDiSD4IhgRkVdo7VU5FZge0fLR23fKbxLzHJcYpyTfgP5DdQ_Sv3g

=== Example 1: Begin Transaction ===
Transaction ID: 8680f6c3-6769-463d-9e0a-e6fc52cb6fec

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
Created Alice: $1000 - ID: IjPfNotPCX1dFXuJ6A-zVOzHt8uBCfq3dh5mvkC8r4KCqOUnVhtYq8CVtlm2A5bup07NFFIrZD3fJuvp7rvbVg
Created Bob: $500 - ID: 8LZ_bUFSya79Iefbr6_a22oSnoOE-sr0eJ1Chp9m4iAf0TuYodyeSOpRVvP1_aOaIhJagSyb-Afxi9DgRGgh4w

=== Example 1: Begin Transaction ===
Transaction ID: 9d0a50e6-5df6-41ef-8840-9d8c6cc7fbe8

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
New transaction: b2268d71-2d1d-40cd-8f8c-b60095289f59
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: QTXLJhZp4suaDCVO2YHJX4_TxmXlKktVNsI4IaL92BFbmhgCk2ZDpBSxveaTQu8ny3qYikfRjFB7hz-OPH76Vg
Created Bob: $500 - ID: AHQFOtzlonh0I7XDVbflHPLLLAlmTn5d9N-YV4vVY-zxPwiwML7MCdgqDBfsjSFFNvRHN8eyQGW6GNoja_Ab2Q

=== Example 1: Begin Transaction ===
Transaction ID: 34c3df0f-4981-4451-8f86-0616fb3e40d8

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
Created Alice: $1000 - ID: GTE1LMVv_1czpwREu24MzBPzJksOkwbeh4_hVwb81u5fWx3Pw6LKLeLF0WaSnH1kaxnjRP_8wyPiYJ1chbALAQ
Created Bob: $500 - ID: JugJM1R444xjs7q7t3dfNt8C_tXL7lWkLVgmzu689BDZpSSTSoSfF0TFeCiNvUdZmDqowlEzVRHFwqJFmLqvJw

=== Example 1: Begin Transaction ===
Transaction ID: a53d1764-3aaa-4844-b2ba-a2a356b4bb8d

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
New transaction: bd0b69ba-48b4-4d5e-979f-2c708477bd32
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: -gN9WErBaj9dgEaVqsLKLzlZcQHfHV8ICqLTS8GMVkJQMB9iGm3G-WMCa8eOJxZcGf4q9xaTlw2mL5PE5SKqvA
Created Bob: $500 - ID: BOQYZvPdIzNR7B4PJP0yujUAHFTDkdQRiSMQ08MEjA2PosbHhFvUxaPdGCHV26DHBQ2U0xcNPRMeblxZ_MOosw

=== Example 1: Begin Transaction ===
Transaction ID: 58ac525a-90d0-45b3-ad92-bd340750d1e5

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
   ✅ Script saved: wc480O-xalDeDCBSJ7ftNuaO63SPu6jgiNIFDTAKSA13hEnK9lVkmzN4YV6gW1tzM0lyfRHmGuwPqkXfczxWCw

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: 8XLS79HeNAqXAzlkWI-wKIHc0UB5g58Bec_D68_o5JOs_WiG9VOcneUb6tPJiUHFIOW0giuC4rSg6VQUHp6jww
   📋 Name: {'value': 'Alice Smith', 'type': 'String'}
   📋 Email: {'value': 'alice@example.com', 'type': 'String'}
   📋 Status: {'value': 'pending', 'type': 'String'}
   📋 Credits: {'type': 'Integer', 'value': 0}

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: trq4f1arqIOm8itUYGBu1n7Op8DFLylMV4GQlW5-Tsk0K8MpYPLrIS0p9Lob84TgJVrZV4UPEmKLxYAZr0emJg

2️⃣ Calling Script (Query + Update + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Status updated to: {'value': 'active', 'type': 'String'}
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}

============================================================
📝 Script 3: Query + Update Credits + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: 8mFHMqMSwyfo4F--JPhTpEJHcl_weKZpUKWfn17uZIfbh4_jsS4n4YJPREnO2UoIoId2OoS7bKjkN1X3krtV6w

2️⃣ Calling Script (Query + Update Credits + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {'type': 'Integer', 'value': 0}
   📋 Status: {'value': 'active', 'type': 'String'}
   📋 Name: {'value': 'Alice Smith', 'type': 'String'}

============================================================
📝 Script 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: cdtX03RJdaETKVeEWbFe1Qdvjh3kZ082OVmwcOD_TVNCkCYF6dYJtJhIBRPuIyK4C_xeDJ9SsGrjQa2n_kvHIA

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: wc480O-xalDeDCBSJ7ft...
   ✅ Deleted script: trq4f1arqIOm8itUYGBu...
   ✅ Deleted script: 8mFHMqMSwyfo4F--JPhT...
   ✅ Deleted script: cdtX03RJdaETKVeEWbFe...
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
  Output: Document ID = -bUm0aKRSGVaA9mthdAe6pc8oUuLjGPAB_zELkrXmlSwEOZfOD49t5YNLS2s0LLzQu4tDFE3sQhMcqnW-NhkVg
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: find_by_id(-bUm0aKRSGVaA9mthdAe6pc8oUuLjGPAB_zELkrXmlSwEOZfOD49t5YNLS2s0LLzQu4tDFE3sQhMcqnW-NhkVg)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: find_by_id(-bUm0aKRSGVaA9mthdAe6pc8oUuLjGPAB_zELkrXmlSwEOZfOD49t5YNLS2s0LLzQu4tDFE3sQhMcqnW-NhkVg)
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
  Output: Document ID = 1YV3_wC664yFQvYJqHdQ9fdeMy_ND0eYEPcghn3fNfDy3eh2PX_CxPTv5vYBkwz0McCKDy8svCzpDBKrZyxG5A
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: find_by_id(1YV3_wC664yFQvYJqHdQ9fdeMy_ND0eYEPcghn3fNfDy3eh2PX_CxPTv5vYBkwz0McCKDy8svCzpDBKrZyxG5A)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: find_by_id(1YV3_wC664yFQvYJqHdQ9fdeMy_ND0eYEPcghn3fNfDy3eh2PX_CxPTv5vYBkwz0McCKDy8svCzpDBKrZyxG5A)
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
    Finished `release` profile [optimized] target(s) in 0.09s
📦 Built wheel for abi3 Python ≥ 3.8 to /Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.8.0-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel for current platform...[0m
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.8.0-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.8.0
    Uninstalling ekodb_client-0.8.0:
      Successfully uninstalled ekodb_client-0.8.0
Successfully installed ekodb-client-0.8.0
✅ [32mPython client package built and installed![0m
🧪 [36mRunning Python client library examples...[0m
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {'id': 'xci5zJ7pzc2kxuAoTCut_hbwPCEk67o7ahplTNDqUrS-Iz_Z1idNr9SfXgMci_o_1C5OUdftO29mUtR2tuYS7w'}

=== Find by ID ===
Found: {'data': 'aGVsbG8gd29ybGQ=', 'user_id': '550e8400-e29b-41d4-a716-446655440000', 'metadata': {'key': 'value', 'nested': {'deep': True}}, 'value': 42, 'created_at': '2026-01-26T17:54:46.953650', 'name': 'Test Record', 'id': 'xci5zJ7pzc2kxuAoTCut_hbwPCEk67o7ahplTNDqUrS-Iz_Z1idNr9SfXgMci_o_1C5OUdftO29mUtR2tuYS7w', 'price': 99.99, 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5], 'categories': ['electronics', 'computers'], 'tags': ['tag1', 'tag2', 'tag3'], 'active': True}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): True
  price (Decimal): 99.99
  created_at (DateTime): 2026-01-26 17:54:46.953650
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): ['tag1', 'tag2', 'tag3']
  metadata (Object): {'key': 'value', 'nested': {'deep': True}}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): ['electronics', 'computers']
  data (Bytes): 11 bytes
Plain record: {'data': 'aGVsbG8gd29ybGQ=', 'user_id': '550e8400-e29b-41d4-a716-446655440000', 'metadata': {'key': 'value', 'nested': {'deep': True}}, 'value': 42, 'created_at': '2026-01-26T17:54:46.953650', 'name': 'Test Record', 'id': 'xci5zJ7pzc2kxuAoTCut_hbwPCEk67o7ahplTNDqUrS-Iz_Z1idNr9SfXgMci_o_1C5OUdftO29mUtR2tuYS7w', 'price': 99.99, 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5], 'categories': ['electronics', 'computers'], 'tags': ['tag1', 'tag2', 'tag3'], 'active': True}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'name': {'value': 'Updated Record', 'type': 'String'}, 'categories': {'value': ['electronics', 'computers'], 'type': 'Array'}, 'tags': {'value': ['tag1', 'tag2', 'tag3'], 'type': 'Array'}, 'metadata': {'type': 'Object', 'value': {'key': 'value', 'nested': {'deep': True}}}, 'id': 'xci5zJ7pzc2kxuAoTCut_hbwPCEk67o7ahplTNDqUrS-Iz_Z1idNr9SfXgMci_o_1C5OUdftO29mUtR2tuYS7w', 'embedding': {'value': [0.1, 0.2, 0.3, 0.4, 0.5], 'type': 'Array'}, 'user_id': {'type': 'String', 'value': '550e8400-e29b-41d4-a716-446655440000'}, 'created_at': {'type': 'String', 'value': '2026-01-26T17:54:46.953650'}, 'price': {'type': 'Float', 'value': 99.99}, 'data': {'type': 'String', 'value': 'aGVsbG8gd29ybGQ='}, 'value': {'type': 'Integer', 'value': 100}, 'active': {'type': 'Boolean', 'value': True}}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: MAwLfTTE-sf1bljfDlfdBQJfJch3a9zWlCaR3R5eYdiOK9NOOWLl7sAAV37dKxtXAJU4elohGQdw8OlQv5Yw2w

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
Collection created with first record: "5pCF4LfO-touKM5wNA5QFl08wZd1257aP5-7MYQp8GtpV8A55CfZIrOZTQj11znPyhag7viELs2-puSu7Qbjgw"

=== List Collections ===
Total collections: 11
Sample collections: ['products', 'batch_users', 'ttl_cache', 'client_collection_management_python', 'chat_configurations__ek0_testing']

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
Created Alice: $1000 - ID: 0SRxR10HRUsgba7y6oeluM1GmXBvCnm60p84pItcOkZi2ndl8l82e85PS09aoGZ4wNreIpPkQv9-2L7zalrhwg
Created Bob: $500 - ID: gIzdpL4iISdoF8M-JoQngQFeLFJGjAx6oWftkJGsgJiIlGtCzq3SX6nolwAfnKFdwH-ZKgIvXBF-FhpXgByWgw

=== Example 1: Begin Transaction ===
Transaction ID: 6f1c7f0b-d89b-41cf-b752-026a6eca563b

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 3ec8ed06-0582-4350-b70f-4e9a10e9c545
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
  1. Score: 25.740, Matched: email.value, email, name, name.value
  2. Score: 12.540, Matched: name.value, name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: bio.value, bio, title, title.value
  2. Score: 26.400, Matched: title, bio.value, title.value, bio
  3. Score: 26.400, Matched: title.value, title, bio.value, bio
  4. Score: 26.400, Matched: bio.value, title, title.value, bio

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title, bio.value, title.value, bio
  2. Score: 39.600, Matched: title, bio.value, title.value, bio
  3. Score: 39.600, Matched: title.value, title, bio.value, bio
  4. Score: 39.600, Matched: title, bio.value, title.value, bio

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
  1. Score: 0.757
  2. Score: 0.755
  3. Score: 0.751

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.376, Matched: content.value, title, content, title.value
  2. Score: 1.378, Matched: title, content, content.value, title.value
  3. Score: 0.378, Matched: 

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
✓ Inserted document: BY1HJ30tC4Ed6AkOWlzDoxYT6zGdIJTXYaSwkkJVmyQ_uiqPRjyReGC8FYdax2uoBJgpCuEm_qU4-0YiBMNuYw

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: myfyMcI61wiYpY9Uoj3NCrQ9OecX6J6b0dSrTaQU4pHcwG89mPascCLCMEkJIDugz2Bp-n4Pen_pK_SSLYKgCg

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
✓ Inserted document with TTL: ai_ms-Db0zjG4xImp_CuKkuLnHP5ho1OLLmTldtfJIShUEmn2H8DLgzbPIgzi7_de8H9CXaMxHi69cpA8e9Rkw

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
✓ Edge cache script created: 8AzHlxo6xJTJQJnLI_jnDgcBcwCoQ9Vxmk_sgYaE0ptmpvUlPMiPEsyQiNwJ-HxKSrO9sV9ZEqX1KjPpY5THSQ

Call 1: Cache miss (fetches from API)
Response time: 2ms
Result: {
  "records": [
    {}
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
Response time: 2ms (1.2x faster!)
Result: {
  "records": [
    {}
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

✅ Script saved: ugfGJ9A-7uO5BhKCug--Ak1aPaB2m4fM2SvGPRnql1_1FRVUHaUJrgUHQQ7ahH4U1SwL9jK-WK9RQG9btCYnNA
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved: PGgsxm2I1jX0DohnkAOQMMn2g7SLCANGGqVFctvx7fgg1_Radm5cs4iKilCwBhYsdK1jqjl576e1lCDyHaL2Og
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: YkXrU375QGDCVn1xCLihyUx0xWlBy7L4sjpVwZISj08dqwD228hHUGuSAuTQoiX7UlEhulu8xKT2qkWHXaHb5g
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
   ⏱️  Duration: 1.6ms
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
   ⏱️  Duration: 1.6ms
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

✅ Script saved: TYppOdJplXsIielvXcA8K1TOCRfX52D7rkVmpMB3oQ4wEIUW334NErdcy6mrpJQ4cx4hlIB1WnA0eP6vS3r9gQ
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

✅ Inserted order: ergflUKzaJNN_N4TRK_nbuWV2SU2k69iHXzMbCXwLDkdVJXF-H71Mu0xqqH-s1O8klMkl7wVamMgkFyOXNos3g
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
🗑️  Deleted session: None

📝 Example 4: KV Pattern Query

✅ Set 4 config entries
📊 Found 3 app config entries
📊 Found 4 total config entries

📝 Example 5: Combined Wrapped Types + KV Usage

✅ Inserted order: KP2Jr-ZNlZVuJltV85qdkW5d4KAQcq-HLFRXufjuNtCjbUxxjsfaEUzDVQ3H9sVBqASocS79kUGYuFzxkgkjLA
✅ Cached order status
📊 Quick status lookup: {'value': '{"status":"processing","updated_at":"2026-01-26T22:54:48.831744+00:00"}'}

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
✓ Created SWR script: fetch_api_user_py (dMG7qLDZEV_7oj97i9_d1ijvrrjydKCaCv20SH6JYKp5OBR6fSvUiEiEzB8FHxvNkZo5Ewkpppi4pzAq1z8m2Q)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {}
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
✓ Lightning fast cache hit

🧹 Cleaning up...
✓ Cleanup complete

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation
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
   {'avg_price': 365.6666666666667, 'category': 'Furniture', 'count': 3}
   {'avg_price': 367.0, 'category': 'Electronics', 'count': 5}
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
   Vector databases offer several advantages including high precision, scalability, and topological information preservation. They can represent complex structures like point, line, and polygon features accurately. It's easy to perform operations like network analysis and topology building. Additionally, they allow direct access to individual features and attributes, making data management more efficient. Lastly, vector data requires less disk space compared to raster data.
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
   {'avg_score': 20.0, 'count': 3, 'role': 'admin'}
   {'avg_score': 70.0, 'count': 7, 'role': 'user'}
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
   1. Introduction to Machine Learning (AI)
   2. Vector Databases Explained (Database)
   3. Natural Language Processing (AI)
   4. Database Design Principles (Database)
   5. Getting Started with ekoDB (Database)
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
✓ Created session: uFJ8rSkbpXH0q3zdp1u6zQl-s1cb-aPG7sUj-qT6fhtiUDubtV-rfrCcwCAbv0AHO4McRBQPiLBRyERPvdYQ5A

=== Sending Chat Message ===
Message ID: JyXPveSxynel5QxLZtBncjmg6qhvrYRYNN6Fhm6yBgdPyk6HKuH7A5t3fAJiKkslBd0Tz9zH36sSCOXkOXjKiA

=== AI Response ===
We have three products available:

1. ekoDB Pro: This is our enterprise edition product with advanced features. It is priced at $299.
2. ekoDB Cloud: This is a fully managed cloud database service product. It is priced at $499.
3. ekoDB: This is a high-performance database product with AI capabilities. It is priced at $99.

Execution Time: 6283ms

=== Token Usage ===
Prompt tokens: 456
Completion tokens: 78
Total tokens: 534

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: BTgflDt4wFT3Sd5h5rv_6_c7uvW-TXZ-NdIGkHV_mn2tDinmB8aeWJTzXDfXyYme0FI1iiR7zGUSfVU_rwcRrA

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information available, there is a high-performance database product named "ekoDB". It costs $99.

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['content', 'token_usage', 'context_snippets', 'updated_at', 'chat_id', 'id', 'created_at', 'role'])
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
✓ Created second session: abgQ5GG0yWu3q7SuaGVLCcUVlpN9Lzr4K_Ny5-l2OTbo_TIKL8_i_0l6B09fmXoNwSeA7l-_BUQDVow0CLWaqQ
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
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: kyC0Ud3IJFGfhFk2Y2madFyLgiMvCvPmsvGcY1ih02i2fp016-68MsK1BxLwHr8CiBTZH7_6rwrr4c85w1mskw

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information provided, a product available is ekoDB. It is a high-performance database product priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 8QUAb80WqnGbsp2Fj-jsiHNOgc8RJXVgjkTyC-2fl1zQv7frbWRnnAwVF53YMfn6SBSjT0bowCY6V4HIpVAuJQ
  Parent: kyC0Ud3IJFGfhFk2Y2madFyLgiMvCvPmsvGcY1ih02i2fp016-68MsK1BxLwHr8CiBTZH7_6rwrr4c85w1mskw

=== Listing Sessions ===
✓ Found 7 sessions
  Session 1: 8QUAb80WqnGbsp2Fj-jsiHNOgc8RJXVgjkTyC-2fl1zQv7frbWRnnAwVF53YMfn6SBSjT0bowCY6V4HIpVAuJQ (Untitled)
  Session 2: kyC0Ud3IJFGfhFk2Y2madFyLgiMvCvPmsvGcY1ih02i2fp016-68MsK1BxLwHr8CiBTZH7_6rwrr4c85w1mskw (Untitled)
  Session 3: abgQ5GG0yWu3q7SuaGVLCcUVlpN9Lzr4K_Ny5-l2OTbo_TIKL8_i_0l6B09fmXoNwSeA7l-_BUQDVow0CLWaqQ (Untitled)
  Session 4: uFJ8rSkbpXH0q3zdp1u6zQl-s1cb-aPG7sUj-qT6fhtiUDubtV-rfrCcwCAbv0AHO4McRBQPiLBRyERPvdYQ5A (Untitled)
  Session 5: BA_ndoTnr5r03-371eTaiV8HcAFSw0mV27F1OLGme-UgwiGVwrPuPA1_Kp9gyHW2wlVLEZqBv0vfCO4udTlanA (Untitled)
  Session 6: pcvxPqW253IbKRKd-rHxq-mW7fHJhDl4VlInFqDEDbDIVNBA5RUJBXB2SDuGmSYhgC8rUos6lwrMWjZ1mnEJVg (Untitled)
  Session 7: JJp8VIsGR4wyMvu17JEv-OLpzuXsTUFvjS_alTXh2kLaX0urem-vKy4enOAO1M-uRfBK8cMBjeUSkp6euPePLw (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: 8QUAb80WqnGbsp2Fj-jsiHNOgc8RJXVgjkTyC-2fl1zQv7frbWRnnAwVF53YMfn6SBSjT0bowCY6V4HIpVAuJQ

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Dict Creation ===
✓ Created record with plain dict: {'id': 'XaoQAM3BL6K1Nr4HyRtD0_5bhvJY5HwUmAlReGjRuLVPClO4t2Fu9j8g34a4xQbS2yM8OPxRzpI2D9caYyjSog'}

=== Upsert Operation ===
✓ Upsert (update existing record): XaoQAM3BL6K1Nr4HyRtD0_5bhvJY5HwUmAlReGjRuLVPClO4t2Fu9j8g34a4xQbS2yM8OPxRzpI2D9caYyjSog
✓ Inserted second record: YWvifG8ehmZO1gu65RPghnle_Umou7TVUAfDsFla_kSqpXyF4St_OaSTtN4PkG16KoGhkwLaiy5BG3GTDrEgFw
✓ Upsert (update second record): YWvifG8ehmZO1gu65RPghnle_Umou7TVUAfDsFla_kSqpXyF4St_OaSTtN4PkG16KoGhkwLaiy5BG3GTDrEgFw

=== Find One Operation ===
✓ Found user by email: {'name': {'value': 'Alice Johnson', 'type': 'String'}, 'age': {'type': 'Integer', 'value': 29}, 'active': {'value': True, 'type': 'Boolean'}, 'id': 'XaoQAM3BL6K1Nr4HyRtD0_5bhvJY5HwUmAlReGjRuLVPClO4t2Fu9j8g34a4xQbS2yM8OPxRzpI2D9caYyjSog', 'email': {'type': 'String', 'value': 'alice.j@newdomain.com'}}
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
   Inserted with ripple: {'id': '7yUKDK-MOqZ-H8EasLbnk5oNKRN2BrfMRn3LwgK1L1ovH5JdZkGwWoLUGuIYhcas1kNVm84nM3YIjZH8n91vnw'}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {'id': 'W5F5yOEEDfygJiH1TYHMWi8reg8EEFOIcm6C5LU_fqf5ganhxS12BNzHZdmbvcnwZfeSdY4m38x80mBtq4hVDA'}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {'price': {'type': 'Integer', 'value': 150}, 'name': {'type': 'String', 'value': 'Product 1'}, 'id': '7yUKDK-MOqZ-H8EasLbnk5oNKRN2BrfMRn3LwgK1L1ovH5JdZkGwWoLUGuIYhcas1kNVm84nM3YIjZH8n91vnw'}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {'price': {'type': 'Integer', 'value': 500}, 'name': {'type': 'String', 'value': 'Upsert Product'}, 'id': '7yUKDK-MOqZ-H8EasLbnk5oNKRN2BrfMRn3LwgK1L1ovH5JdZkGwWoLUGuIYhcas1kNVm84nM3YIjZH8n91vnw'}

✅ All bypass_ripple operations completed successfully!
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields
Fetched 3 users with only 3 fields each

Example 2: Exclude sensitive fields
Fetched 2 admins without sensitive data
  Password field excluded: False

Example 3: Complex query with projection
Fetched 3 active users with profile fields

Example 4: Find by ID with projection
Fetched user profile: {'value': 'Alice Johnson', 'type': 'String'}

Example 5: Compare full vs projected data
Full query returned 12 fields per user
Projected query returned 12 fields per user

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
✅ [32mPython client examples complete![0m
