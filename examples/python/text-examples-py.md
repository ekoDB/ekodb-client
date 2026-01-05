make test-examples-python
🧪 [36mRunning Python examples (direct HTTP/WebSocket)...[0m
=== Simple CRUD Operations (Direct HTTP) ===

✓ Authentication successful

=== Insert Document ===
Inserted: {'id': 'dcdbm6OGxfX65V3h70XvCB0PWPuuFkzEO7yIkKgO41kPT5nqshzEGWlX9uQmrdfiRnOkBQ62f5h5ZbiB-f9HWg'}

=== Find by ID ===
Found: {'name': {'type': 'String', 'value': 'Test Record'}, 'id': 'dcdbm6OGxfX65V3h70XvCB0PWPuuFkzEO7yIkKgO41kPT5nqshzEGWlX9uQmrdfiRnOkBQ62f5h5ZbiB-f9HWg', 'value': {'type': 'Integer', 'value': 42}, 'active': {'type': 'Boolean', 'value': True}}

=== Find with Query ===
Found documents: [{'active': {'type': 'Boolean', 'value': True}, 'name': {'type': 'String', 'value': 'Test Record'}, 'id': 'dcdbm6OGxfX65V3h70XvCB0PWPuuFkzEO7yIkKgO41kPT5nqshzEGWlX9uQmrdfiRnOkBQ62f5h5ZbiB-f9HWg', 'value': {'type': 'Integer', 'value': 42}}]

=== Update Document ===
Updated: {'active': {'type': 'Boolean', 'value': True}, 'name': {'type': 'String', 'value': 'Updated Record'}, 'value': {'type': 'Integer', 'value': 100}, 'id': 'dcdbm6OGxfX65V3h70XvCB0PWPuuFkzEO7yIkKgO41kPT5nqshzEGWlX9uQmrdfiRnOkBQ62f5h5ZbiB-f9HWg'}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: t72bpt21jeSg4T5Hfo6Fk9U-NA8EZPcpKDWKpNfxWbDNAhHHFhjk2RkrVZVqsb_YoUfIXZy-UOQ7xUd1LDrPFQ

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
        "id": "jCqqfNX1CH8oAOspCeoYtlibdQJHNY0vgR73fxLb98lsm2A2lhAlkEWs2K4JJ3fG0iHF4joH55M1tlEJ2xM6hw",
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
        "id": "t72bpt21jeSg4T5Hfo6Fk9U-NA8EZPcpKDWKpNfxWbDNAhHHFhjk2RkrVZVqsb_YoUfIXZy-UOQ7xUd1LDrPFQ",
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

✅ Script saved: Q2Vco7QdhKQMxGOB3-Q6MjTnVKS2lTjly87kesssWoDQRnh9K4-Q2fX3iapG_Z-5hitDem3e8DnKqX0wSCYNKg
📊 Found 35 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: 0rl6a-uFCVIQsiqjdwaan_A1OOzlNG_62JA4yqE8oRuTX_-C22wYw9avv0oR-MhAHZlJHrUQTyPWSBYirx7pSw
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: wipRXZPeml8OWQTNQTzd7csW_Xc8zTlzZf2zTHeZDw0N8i5v5Rz74DtFKl2seTqrIGydnULmTgm49HHFh7QEsQ
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {'count': 15, 'avg_score': 60.0, 'max_score': 100, 'status': 'active'}
   {'avg_score': 50.0, 'max_score': 90, 'status': 'inactive', 'count': 15}

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
Collection created with first record: FBX8YSKbb0Sm4CvTeuQmAleFSMDRTaDQBT3HU2FRG7Yp27DtfRE31ynYn0AcNG_1D5q_1c5rMIkTibmxQCCcnw

=== List Collections ===
Total collections: 12
Sample collections: ['chat_configurations__ek0_testing', 'ws_ttl_test', 'websocket_test', 'test_collection', 'users']

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
Created Alice: $1000 - ID: 6y4nPqaatxZfQjJZTOzQ5kx3-4NDWVSDLfCJeFSAO25kB52DKUSeiFQ6oZCB2gZahzFhB9cdi0LCjEj9QvCHvA
Created Bob: $500 - ID: Qzb4j74yLSABtffUVwfn1yERhwW4_eWJEmqRQ-RmqZMi9ts2rJtwUgPGvSgC6hOeMCMne-LXZzrsQnpMFkxFGw

=== Example 1: Begin Transaction ===
Transaction ID: 8ac7e18d-0b18-4e37-88b2-24a1142a4497

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
New transaction: f3afd2c1-fc79-49fe-93f7-f0ea7aef532f
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: zkFlqY_FMF88AZu49LnwLvukRmLMt4YI990NZmtBKEaPya4ynrdVcUkuozpJ5uRXzapN7qA40VGxPqVZmX20ZQ
Created Bob: $500 - ID: YNUZuueY3h4sR8kmm2CIWDM1gWZHJtyc1V6oI7mUScpwyi7LwMNp-rDTVH6fZqPWAkCbRInOA4XnHmI411NjEQ

=== Example 1: Begin Transaction ===
Transaction ID: 95f55141-586b-4eae-8f84-e259e2367d2e

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
Created Alice: $1000 - ID: afZRBAth1NBO4td9xDug8DjF3sKsAb8QNM_a1DhoHDsginHDSd4JKlTZ0esvPEd_AljaTKdE7FuTQN3G3I1sfQ
Created Bob: $500 - ID: wSO6QDEutzlaCGxOS2tMvUMf4duLQn-4w8MjjzqvTMFS2d6JWY0TCyN805_foVISleCBUvg57ggh4SvLFbjkvQ

=== Example 1: Begin Transaction ===
Transaction ID: dec01d2a-4615-4991-897b-1a6103b9dca9

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
New transaction: d28ab0b3-272b-471b-8f34-2982fa313769
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: r7WNqCTcTU6gCDYIOoHgvt7ilU7g7fjBEOuN-ygifnzxUGYysMbc6GWUyn0K3JtEi2iB9INGFFAVGw5TiiFiDg
Created Bob: $500 - ID: FQ8QlNo2o7bQHhmlOrvlTZbL3znt-MpZUPZCdNGQ-E4NGXVy670M1l4Rgfhmz2Q0RqvGNITeZVUVBcEWea_mFw

=== Example 1: Begin Transaction ===
Transaction ID: 97f0731c-4323-4490-851e-6ed6d95d6d36

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
Created Alice: $1000 - ID: fiLhWYRc1ni7Il9qm6r0l2FIZOD70dvtURgg9OlQxVI7gwYFCOb_l3ozDRftmAR659aDD3-NYs20vX-n5YpSkg
Created Bob: $500 - ID: aINnbCMF5wW0aMug71tMfUnU-TzRTazTjt-e2mHmP2922JGaOW9Y-4XXBK3GmiZJ2daadu11eMOUntvvM_w0Dw

=== Example 1: Begin Transaction ===
Transaction ID: e9170096-8a55-4b06-b8ff-934f0f20a267

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
New transaction: 152b9038-17ec-483a-908d-b62526de7736
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: Cj6xXstUV4VO5eLrWrGDrZ96ZwUrkjOMRg4JVXeS1RDwkiEECaCsc5ZMj9SqZxQBBYaywiwM2tbh_7dOr_kfFw
Created Bob: $500 - ID: rBFlalGSlOBufnitvyLX2TDLBuIpLeza2eV1HmvJMccfsTOXYYppYZqzOTIK400-D0hPQ6TAkq69hE-Z4vIpcQ

=== Example 1: Begin Transaction ===
Transaction ID: 2e0e0c67-a63d-4166-9b07-9cd1f017fa95

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
   ✅ Script saved: keMLbr92zDKK2Nw2Th8wGhkeY9IkvhJT-7X8DwDQKCfbiLwSAZAoZJW6p75n_m0dos2-9BfhfvfPtMYp1UBaMQ

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: TgfN3LnXOv3ZlAcC_HPl8q0qygUrS9l6dkxmjXB1TqK8f700kVv-L7CHclhv1zB8a3dr5eENbmHn2t5K-FwWdA
   📋 Name: {'value': 'Alice Smith', 'type': 'String'}
   📋 Email: {'value': 'alice@example.com', 'type': 'String'}
   📋 Status: {'value': 'pending', 'type': 'String'}
   📋 Credits: {'type': 'Integer', 'value': 0}

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: k9FV8UqRoVCdZZW0PzH7Lm7ksZuMoiJRah24VKtXQZsWLGwp4c-64fm_Kz_NFFAYweHMK58fppxXv5eyvvyYVQ

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
   ✅ Script saved: V1Y6cFs7znaq7KRNTQKSJIeLX2VoJZEXCwxOou-wIpZkkeJbbMS-QDi3iLUhegaM3IwTywxJ9aayCAbWWqTCxw

2️⃣ Calling Script (Query + Update Credits + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {'value': 0, 'type': 'Integer'}
   📋 Status: {'value': 'active', 'type': 'String'}
   📋 Name: {'value': 'Alice Smith', 'type': 'String'}

============================================================
📝 Script 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: d66ar0Hkhuw5mKPWMGiR1ChwOMNSdIPkwV1VDRLpWh9m3LZ2iVZVf6ZwjWioaD9y0NSNthtSCPP5OAjJnYfQGQ

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: keMLbr92zDKK2Nw2Th8w...
   ✅ Deleted script: k9FV8UqRoVCdZZW0PzH7...
   ✅ Deleted script: V1Y6cFs7znaq7KRNTQKS...
   ✅ Deleted script: d66ar0Hkhuw5mKPWMGiR...
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
  Output: Document ID = lOTU-P0G7MvXdlXCreSatBG-_AcBNIVmjuo-IEw7couZfPXL787FHNqFFCY05Q6_mEpQQKqdF1vGo-Ipr1WVEA
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: find_by_id(lOTU-P0G7MvXdlXCreSatBG-_AcBNIVmjuo-IEw7couZfPXL787FHNqFFCY05Q6_mEpQQKqdF1vGo-Ipr1WVEA)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: find_by_id(lOTU-P0G7MvXdlXCreSatBG-_AcBNIVmjuo-IEw7couZfPXL787FHNqFFCY05Q6_mEpQQKqdF1vGo-Ipr1WVEA)
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
  Output: Document ID = RBih_U6JrqNY0lguSkPYmseTOZUJihMCpGKe_gN3kZNIk-92d1AWZ44V9932M-P5JI-DuWh14jyB55LPyQTLVg
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: find_by_id(RBih_U6JrqNY0lguSkPYmseTOZUJihMCpGKe_gN3kZNIk-92d1AWZ44V9932M-P5JI-DuWh14jyB55LPyQTLVg)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: find_by_id(RBih_U6JrqNY0lguSkPYmseTOZUJihMCpGKe_gN3kZNIk-92d1AWZ44V9932M-P5JI-DuWh14jyB55LPyQTLVg)
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
Inserted: {'id': 'i66cHF1pildawzYt0dOSoVGHBypWVt6Dur1iqlEW9C01hf10Xqs6RfSornAvZxL5772pbmHs5wC2UIfs-F3o-w'}

=== Find by ID ===
Found: {'data': {'value': 'aGVsbG8gd29ybGQ=', 'type': 'String'}, 'name': {'value': 'Test Record', 'type': 'String'}, 'id': 'i66cHF1pildawzYt0dOSoVGHBypWVt6Dur1iqlEW9C01hf10Xqs6RfSornAvZxL5772pbmHs5wC2UIfs-F3o-w', 'active': {'type': 'Boolean', 'value': True}, 'embedding': {'value': [0.1, 0.2, 0.3, 0.4, 0.5], 'type': 'Array'}, 'categories': {'value': ['electronics', 'computers'], 'type': 'Array'}, 'metadata': {'type': 'Object', 'value': {'key': 'value', 'nested': {'deep': True}}}, 'created_at': {'value': '2026-01-05T10:33:30.516539', 'type': 'String'}, 'user_id': {'value': '550e8400-e29b-41d4-a716-446655440000', 'type': 'String'}, 'tags': {'value': ['tag1', 'tag2', 'tag3'], 'type': 'Array'}, 'price': {'type': 'Float', 'value': 99.99}, 'value': {'type': 'Integer', 'value': 42}}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): True
  price (Decimal): 99.99
  created_at (DateTime): 2026-01-05 10:33:30.516539
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): ['tag1', 'tag2', 'tag3']
  metadata (Object): {'key': 'value', 'nested': {'deep': True}}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): ['electronics', 'computers']
  data (Bytes): 11 bytes
Plain record: {'data': 'aGVsbG8gd29ybGQ=', 'name': 'Test Record', 'id': 'i66cHF1pildawzYt0dOSoVGHBypWVt6Dur1iqlEW9C01hf10Xqs6RfSornAvZxL5772pbmHs5wC2UIfs-F3o-w', 'active': True, 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5], 'categories': ['electronics', 'computers'], 'metadata': {'key': 'value', 'nested': {'deep': True}}, 'created_at': '2026-01-05T10:33:30.516539', 'user_id': '550e8400-e29b-41d4-a716-446655440000', 'tags': ['tag1', 'tag2', 'tag3'], 'price': 99.99, 'value': 42}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'embedding': {'value': [0.1, 0.2, 0.3, 0.4, 0.5], 'type': 'Array'}, 'price': {'type': 'Float', 'value': 99.99}, 'categories': {'value': ['electronics', 'computers'], 'type': 'Array'}, 'name': {'type': 'String', 'value': 'Updated Record'}, 'id': 'i66cHF1pildawzYt0dOSoVGHBypWVt6Dur1iqlEW9C01hf10Xqs6RfSornAvZxL5772pbmHs5wC2UIfs-F3o-w', 'created_at': {'type': 'String', 'value': '2026-01-05T10:33:30.516539'}, 'value': {'value': 100, 'type': 'Integer'}, 'tags': {'value': ['tag1', 'tag2', 'tag3'], 'type': 'Array'}, 'active': {'type': 'Boolean', 'value': True}, 'user_id': {'value': '550e8400-e29b-41d4-a716-446655440000', 'type': 'String'}, 'metadata': {'value': {'key': 'value', 'nested': {'deep': True}}, 'type': 'Object'}, 'data': {'type': 'String', 'value': 'aGVsbG8gd29ybGQ='}}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: 4vULh0fY5gMzGff1GiFMpX8U1L2ytm8AWTFNob60XtEoBXDzQ2FQXGxeL-uo6UkwRl8-ENb7AcktA6ab1bn8nQ

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
Collection created with first record: "0IKKcA-OYDRnGKLolMibYWQIlhMflvDr3BAF20s3mV2fpnkohHOjD9Udk5kTMmRumJgGYQq1Tl3rtxhhPIAyEw"

=== List Collections ===
Total collections: 11
Sample collections: ['chat_configurations__ek0_testing', 'ws_ttl_test', 'websocket_test', 'test_collection', 'test_accounts']

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

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: OdZu5yrTk3HGSXHg9R92xKQOE4FiLve4oC9fcA8Ui5BnFcslKW7NH8ZL4LJjrz4k6yeIYU_8gXcZ_MQ6JfsLVA
Created Bob: $500 - ID: kzBEt6qVp67B-mfZHJ_EN7DA76srq1vniNRgh5fl7Fwz_EfVIVNTYzNZ6Go1GnhlamrYsOu8xKj66svujqk6jQ

=== Example 1: Begin Transaction ===
Transaction ID: f235cd11-fdad-4bb2-a7a2-0abe7d1cc5fe

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: ec6d1065-9b96-4714-a2ea-8f095fa72272
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
  1. Score: 25.740, Matched: name, name.value, email.value, email
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title, bio, bio.value, title.value
  2. Score: 26.400, Matched: bio.value, title, bio, title.value
  3. Score: 26.400, Matched: title.value, bio, title, bio.value
  4. Score: 26.400, Matched: title, title.value, bio, bio.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title.value, bio.value, bio, title
  2. Score: 39.600, Matched: title, title.value, bio.value, bio
  3. Score: 39.600, Matched: title, bio, title.value, bio.value
  4. Score: 39.600, Matched: title, bio, title.value, bio.value

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
  1. Score: 0.758
  2. Score: 0.744
  3. Score: 0.727

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.721, Matched: content.value, title, title.value, content
  2. Score: 1.109, Matched: title.value, title, content.value, content
  3. Score: 0.530, Matched: 

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
✓ Inserted document: Yg7QNePsevlsvS8wgve5u-baj0XfW5q9t17UdUO_VM3WR5LvnwtZpb-vjpb__sAEdZszLYRagRCx3n9gD6zKrw

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: vdpFNvY0OiFJPVsrPGFcimx0IeF3yeXrmySt9f-KKeHqd4Ihyv7kyRDtx2r_eUG2khLr_WmfTDBVf791MR2NBA

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
✓ Inserted document with TTL: -Xgskh6sP9I92FgDEDWf_fl2g9QDjyRDib4mpn-p1f1eTS4BhJc6kkoEUEBSS6VI0IV_gSF9Ojti5xdgBZYqwg

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
✓ Edge cache script created: jvnpLMc_W9g8sLmigZ9SwgSy1-zB0lK9MDrWrlTuPenelTjdjDIsZj-m7FTHUtqIVG4qP-pTycn4kkrEovremA

Call 1: Cache miss (fetches from API)
Response time: 557ms
Result: {
  "records": [
    {
      "cached_at": "2026-01-05T10:33:31.925240",
      "data": {
        "type": "Object",
        "value": {
          "current": {
            "interval": 900,
            "temperature_2m": 0.1,
            "time": "2026-01-05T15:30"
          },
          "current_units": {
            "interval": "seconds",
            "temperature_2m": "\u00b0C",
            "time": "iso8601"
          },
          "elevation": 32.0,
          "generationtime_ms": 0.0209808349609375,
          "latitude": 40.710335,
          "longitude": -73.99309,
          "timezone": "GMT",
          "timezone_abbreviation": "GMT",
          "utc_offset_seconds": 0
        }
      },
      "id": "aq-2vljpZcwEeUooSEqOCl_dGuAhGVQv-FrybPdMjiVM4Oi_8sMIKb2ODhr-JQjfNyUIQZFNjtuWQbbWX0tIoA"
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
Response time: 544ms (1.0x faster!)
Result: {
  "records": [
    {
      "cached_at": "2026-01-05T10:33:31.925240",
      "data": {
        "type": "Object",
        "value": {
          "current": {
            "interval": 900,
            "temperature_2m": 0.1,
            "time": "2026-01-05T15:30"
          },
          "current_units": {
            "interval": "seconds",
            "temperature_2m": "\u00b0C",
            "time": "iso8601"
          },
          "elevation": 32.0,
          "generationtime_ms": 0.02205371856689453,
          "latitude": 40.710335,
          "longitude": -73.99309,
          "timezone": "GMT",
          "timezone_abbreviation": "GMT",
          "utc_offset_seconds": 0
        }
      },
      "id": "mtZcjswxSn8EE7BOSrIDvyBSzRm3ADoYnruFOtQecvEy2kdmuSK4tvArE67kcqGO693PbZPFDZUwmpuOoviqlQ"
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

✅ Script saved: hvwWiBrNqTXw-FTvSes_gWUtdDIBOTCdTznZcfJlKmy8eCGuqqfZWFuzUcadtAhk-Wyy_lgfS-hOUy7E9oP5cw
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved: 8V4ZZCtfnVJJzOFWC-glIun5st0Ie-2ZBzome7_CFdlEiw0dcYn4Kq8HyvBjSRaU69I7iPujDubJoq5_-yle0g
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: wBjxw4H2Hy93vYiWDrJTWgls6ELN9Yr6a_zYvUC0L5FYMbQS6GZYhAZq-X1HLzyvVlaGF8vG1KXdqU8Rse-6QQ
📊 Statistics: 2 groups
   {'avg_score': 60.0, 'count': 5, 'status': 'active'}

   {'avg_score': 50.0, 'count': 5, 'status': 'inactive'}

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

Using CallFunction to replace inline logic in SWR pattern...

✅ Saved reusable function: fetch_and_store_github
✅ Saved SWR function using composition: swr_github_user

First call (cache miss - will fetch from GitHub):
   ⏱️  Duration: 144.1ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 83.3ms
   📊 Records: 1
   🚀 Cache speedup: 1.7x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

✅ Level 1 function: validate_user
✅ Level 2 function: fetch_slim_user (calls validate_user)
✅ Level 3 function: count_validated_user (calls fetch_slim_user)

📊 Result from 3-level nested composition:
   Records: 1
   Name: None
   Department: None
   Record count: 1

🎯 Key Benefit: Each function is independently testable and reusable!
   - validate_user: Used in 100 different workflows
   - fetch_slim_user: Used in 50 workflows
   - count_validated_user: Specific workflow


✅ All composition examples completed!
🚀 ekoDB Python Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved: Co3qC82KMFMPRAJrh8Q22G5Y8oM2AxxCnoIPT55B0blZ1PlintTGWryWEpijrzFCx9oTGaXHQgHux76_4e30OQ
📊 Found 2 product groups
   {'avg_price': 474.0, 'category': 'Furniture', 'count': 2}
   {'avg_price': 575.6666666666666, 'category': 'Electronics', 'count': 3}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved
📊 Found 2 categories
   {'category': 'Furniture', 'count': 2}
   {'category': 'Electronics', 'count': 3}
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

✅ Inserted order: BuTxU6yuX_ptwfr3yycatNZAnuKh1eVIWpVdt4YaDZQ4QeRGp2YbGM2njTxIQZy1k69bDNI958BoTwwos2vidg
✅ Inserted 2 products with wrapped types

📝 Example 2: Querying and Extracting Wrapped Types

📊 Found 2 products
   • Wireless Mouse
   • Laptop Pro

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

✅ Inserted order: p6bcTfGHAiWS_Uck_V8IXU94T_gwVffjdeTDh95aq7mYg-UYAp_ba-vSaNF3a9bcRlfCm-Goirckyvn9Pc5jQQ
✅ Cached order status
📊 Quick status lookup: {'value': '{"status":"processing","updated_at":"2026-01-05T15:33:33.748053+00:00"}'}

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
✓ Created SWR script: fetch_github_user_py (FHO7MD0fDJL8eZAtvg-UH-6Suqb-fGKpykZ4WPlAa_c3_g9Yx7OIRfky0t9FB7JitX4_r-LCLuTX7MSVvYGjDA)

Step 2: First call - Cache miss, fetches from GitHub API
Traceback (most recent call last):
  File "/Users/tek/Development/ekoDB/ekodb-client/examples/python/client_swr_pattern.py", line 119, in <module>
    asyncio.run(main())
  File "/Library/Frameworks/Python.framework/Versions/3.11/lib/python3.11/asyncio/runners.py", line 190, in run
    return runner.run(main)
           ^^^^^^^^^^^^^^^^
  File "/Library/Frameworks/Python.framework/Versions/3.11/lib/python3.11/asyncio/runners.py", line 118, in run
    return self._loop.run_until_complete(task)
           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Library/Frameworks/Python.framework/Versions/3.11/lib/python3.11/asyncio/base_events.py", line 653, in run_until_complete
    return future.result()
           ^^^^^^^^^^^^^^^
  File "/Users/tek/Development/ekoDB/ekodb-client/examples/python/client_swr_pattern.py", line 82, in main
    result1 = await client.call_script(
              ^^^^^^^^^^^^^^^^^^^^^^^^^
RuntimeError: Call script failed: API error (500): Server error: Unhandled rejection: ErrorResponse { error: "Field 'data' has incomplete field definition - must have both 'type' and 'value'" }
make[2]: *** [test-examples-python-client] Error 1
