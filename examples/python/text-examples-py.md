make test-examples-python
🧪 [36mRunning Python examples (direct HTTP/WebSocket)...[0m
=== Simple CRUD Operations (Direct HTTP) ===

✓ Authentication successful

=== Insert Document ===
Inserted: {'id': 'phfh2eSYUXriYJQyO7tr6ftR3xAkCznOGUXVLJVaFQP2D-At0OCFlisNDL9L5kww0TSnvE-IqJZ-MY5J9WBKMw'}

=== Find by ID ===
Found: {'name': {'type': 'String', 'value': 'Test Record'}, 'id': 'phfh2eSYUXriYJQyO7tr6ftR3xAkCznOGUXVLJVaFQP2D-At0OCFlisNDL9L5kww0TSnvE-IqJZ-MY5J9WBKMw', 'value': {'type': 'Integer', 'value': 42}, 'active': {'type': 'Boolean', 'value': True}}

=== Find with Query ===
Found documents: [{'active': {'type': 'Boolean', 'value': True}, 'id': 'phfh2eSYUXriYJQyO7tr6ftR3xAkCznOGUXVLJVaFQP2D-At0OCFlisNDL9L5kww0TSnvE-IqJZ-MY5J9WBKMw', 'value': {'type': 'Integer', 'value': 42}, 'name': {'type': 'String', 'value': 'Test Record'}}]

=== Update Document ===
Updated: {'name': {'type': 'String', 'value': 'Updated Record'}, 'active': {'type': 'Boolean', 'value': True}, 'value': {'value': 100, 'type': 'Integer'}, 'id': 'phfh2eSYUXriYJQyO7tr6ftR3xAkCznOGUXVLJVaFQP2D-At0OCFlisNDL9L5kww0TSnvE-IqJZ-MY5J9WBKMw'}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: YOrVGcy8f4jZ6600r2cgnHQI7bIj5GDda2kwQvKJfZl8UXJxG3jujZoY5ZGGQ9p_QWUhqIIav26PpAwBa1VTkg

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
        "id": "YOrVGcy8f4jZ6600r2cgnHQI7bIj5GDda2kwQvKJfZl8UXJxG3jujZoY5ZGGQ9p_QWUhqIIav26PpAwBa1VTkg",
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
        "id": "YGFk_EvfaQlmIM2mRRLCf9Zt4kvlMzFYdrJHI0wrqfeoEf-hsCHJTXKuQDPjUyQoTI5uunzX9I5kMuBuDRiqhw",
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

✅ Script saved: 9QTSDF-2Bel483ab-axjpyiV3ih8itxl0C-JPLSMuoRaP-1UERFZlkMJvW43IOlGfyUPHWK9Lp_CTrnBDHqViw
📊 Found 35 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: sfwDyWg9UotX7lyVs5VEyahN_NLRcntnwqSMYfwn8vof3rjQn0Jvs3IZGGvS7iIWCb5efMMazamNqX6hkwu4Tw
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: -uXirDtHpyKYNX3kztrvb5PQAhxccPTqpl6BMrZBAPbqkbsjKyCrviMjxZgXv73-kIUd18HyuTvEKmSHk_t2dg
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {'count': 15, 'avg_score': 60.0, 'status': 'active', 'max_score': 100}
   {'max_score': 90, 'avg_score': 50.0, 'count': 15, 'status': 'inactive'}

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
Collection created with first record: 6-kwDSKyjyWEL5nidccmap67SFAop2udD0kS8xnU9JqLGhT8xhGx-KRagWC_0nMCeEhpwdhtFjwk_0EOVBIpbA

=== List Collections ===
Total collections: 13
Sample collections: ['websocket_test', 'demo_collection', 'chat_configurations__ek0_testing', 'test_accounts', 'test_collection']

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
Created Alice: $1000 - ID: NF1AZMVvYrdjDB6taxfZ3hfkUX0kIZ-CI_D7ue_fzjNBjlcyeeSrGzZo9xrs_lS9DJBhFUAjrR84aIaU0YGflQ
Created Bob: $500 - ID: uTgnDy8ix4SBtbSBOAo7RfWJvpY-Oi3TU_lit-psPCXr8c-CceLbG8-X7rRvmTXG0CaKBtoTPbMG_XCEH7qBDQ

=== Example 1: Begin Transaction ===
Transaction ID: 27bd4a68-7cf4-4811-af02-fa3530779b35

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
New transaction: 4d524d88-0d8c-436c-9974-f7f36f31c7eb
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: msNnlLFjRyY1iPbC0mkicbWqlj0fa5BW6RNRuJzfKzB2aVfoGNEcNik1bIQKN2D0H38Co6i3PToaw4-gPqQwrw
Created Bob: $500 - ID: yBDy3-aoPkLZYPLhG21EveGk-1mII-O5TN0AB-bynVqNix5aJFpqR0NxHVNY8nSl1nlVGw3RLmH3NsAOsh-o2w

=== Example 1: Begin Transaction ===
Transaction ID: 4eadede3-97a8-4f0a-b336-013818900e4f

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
Created Alice: $1000 - ID: etZFwy13plBKY0CjpWEkHxi5c2LsHn3FcZpBf-J8n68h1VK_Ftv8oxxbFvStHxbZcHoYp5jEHFN4H-WHTiKg7w
Created Bob: $500 - ID: fja_tHcZzzRKTt6TbyZvN-CSzz2KOmCweh3pTEYcVw4wAFgSj_SgNFiwnn7cNjPy5Vb85-dLc1yzWiMFCPOZXA

=== Example 1: Begin Transaction ===
Transaction ID: 3f886405-c0ce-4fc0-ae30-f7148bb0c6c4

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
New transaction: 07001fc6-08d9-498a-9e41-6ff665fd6870
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 9I5y3infW9i6T8FWO0RPnNmtdddL6SyB2ErD1Qm1RA035sstI39yZUXzxpEeSbw6Y-eKmEU3g5bgIlK-2NHZIw
Created Bob: $500 - ID: Rgx-2tlcFZHEIvVi8y8H3T6oUSTwDN4PerrzSY5u_Pjqnbvo8jhjnksj1mCGbWfYgsgK31TRbW3T1ukVEOJtLQ

=== Example 1: Begin Transaction ===
Transaction ID: d3b5b049-e4f5-44a5-8edc-0638a823fe7e

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
Created Alice: $1000 - ID: Qlf52J51b1E9JJRe2XecY27QELFzjWKjyOFSyLXShJIOEle-XIB5RTcCJh6xpZFMXf0tsOmbL_2BNDqofUO67g
Created Bob: $500 - ID: AzBj2nk3rRi5MXGYx6WP7-hqMlq-XFonziua-K8U1QkiCKv4r8XPc8Kw9Yrynk_Hc18obzNfGJkau4z4WaQOFg

=== Example 1: Begin Transaction ===
Transaction ID: 837f12ab-1d8a-4ff8-a25c-efd1982e1ac6

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
New transaction: 9606cd28-6b45-4257-912c-14428db216d6
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 1HwD3A_iE7AT5Sb9-wZrV8XQiBoFiteDTi6xy9Yq-GoK2uhqRVB98L2bhQNMnzKlS9u1IEICwqTf9ZPt96G4Gw
Created Bob: $500 - ID: nW9jIpFjG9Fyx2sX23zI2BhtSg5rZc5x-qLIOUNwnM74NKp-2lsgwk3NLqzC92j6iyPpnPmFTgjIhbHlQtvvvQ

=== Example 1: Begin Transaction ===
Transaction ID: c3b8c945-c10a-428e-b374-6091c6ec0b38

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
   ✅ Script saved: x8FK42taeTty6liQ-b5qqAl6rijGNOHAFSmmDXD0QI_Why2u0FWS9zcJ107jQ-jPonfIqb5mno5d25CbTgbrpA

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: 8Og1McqFa8yUs7qj0cs8Eba8mJqj9I_oENclOkKaf5gHxJYJFYmaRR7uUSVChjK1ZjrNXvZUaF7SdFtnTq4wfQ
   📋 Name: {'value': 'Alice Smith', 'type': 'String'}
   📋 Email: {'value': 'alice@example.com', 'type': 'String'}
   📋 Status: {'value': 'pending', 'type': 'String'}
   📋 Credits: {'value': 0, 'type': 'Integer'}

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: 9-MTB-WnBxeS29UlyHRQZvej69RlYuNqVAHyywMe90ZiDSXCan5TwdWD9bDMFZbFyG4Ju1HZEjZ_vPHxc3jlOw

2️⃣ Calling Script (Query + Update + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Status updated to: {'value': 'active', 'type': 'String'}
   📋 Name: {'value': 'Alice Smith', 'type': 'String'}

============================================================
📝 Script 3: Query + Update Credits + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: DPIWLrgSueEDBjQqE7gqgGzZ8rxSQ-ojq23PUlPkJmDH6dgBI1d7FCWhur9z9ezM-XKRUxKFbo-XZCtt8qkUtw

2️⃣ Calling Script (Query + Update Credits + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {'type': 'Integer', 'value': 0}
   📋 Status: {'type': 'String', 'value': 'active'}
   📋 Name: {'value': 'Alice Smith', 'type': 'String'}

============================================================
📝 Script 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: jHc0Zpwe8ffjNzlrBTbHRlzATliKltdbqBBZo8GSZPcNlDWc4AvW96vuFndhR5ROrV_dJgdP-1a2TWhouTegag

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: x8FK42taeTty6liQ-b5q...
   ✅ Deleted script: 9-MTB-WnBxeS29UlyHRQ...
   ✅ Deleted script: DPIWLrgSueEDBjQqE7gq...
   ✅ Deleted script: jHc0Zpwe8ffjNzlrBTbH...
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
  Output: Document ID = 7BD16gxVpn20oaTUOPRJZelCLAxFoB3c5Cs8zDFYCpCpdWtq6bRe9krI67_P1MuSaBXhi_2Dhq7SSrsx3Dp1Wg
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: find_by_id(7BD16gxVpn20oaTUOPRJZelCLAxFoB3c5Cs8zDFYCpCpdWtq6bRe9krI67_P1MuSaBXhi_2Dhq7SSrsx3Dp1Wg)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: find_by_id(7BD16gxVpn20oaTUOPRJZelCLAxFoB3c5Cs8zDFYCpCpdWtq6bRe9krI67_P1MuSaBXhi_2Dhq7SSrsx3Dp1Wg)
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
  Output: Document ID = OWjpniwy5VgGlePvPlVRNq4dxKNGEAd9qlAltHfb3Oe1gl5RAco_WfmoFR9NAKC2UCuZsKjIfnyIhHfKXybTCw
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: find_by_id(OWjpniwy5VgGlePvPlVRNq4dxKNGEAd9qlAltHfb3Oe1gl5RAco_WfmoFR9NAKC2UCuZsKjIfnyIhHfKXybTCw)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: find_by_id(OWjpniwy5VgGlePvPlVRNq4dxKNGEAd9qlAltHfb3Oe1gl5RAco_WfmoFR9NAKC2UCuZsKjIfnyIhHfKXybTCw)
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
    Finished `release` profile [optimized] target(s) in 0.14s
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
Inserted: {'id': 'LuEg0X3IRNqZ111EVB9EVL-nxhCWjNOzlRybJ0u7kj3dbQbxp4lTRf_Mkkzju6soe4Vrj4J0KXUl_JeIHZjULg'}

=== Find by ID ===
Found: {'user_id': {'type': 'String', 'value': '550e8400-e29b-41d4-a716-446655440000'}, 'metadata': {'type': 'Object', 'value': {'key': 'value', 'nested': {'deep': True}}}, 'created_at': {'value': '2026-01-07T20:16:41.044178', 'type': 'String'}, 'value': {'type': 'Integer', 'value': 42}, 'tags': {'type': 'Array', 'value': ['tag1', 'tag2', 'tag3']}, 'data': {'type': 'String', 'value': 'aGVsbG8gd29ybGQ='}, 'categories': {'type': 'Array', 'value': ['electronics', 'computers']}, 'active': {'type': 'Boolean', 'value': True}, 'embedding': {'type': 'Array', 'value': [0.1, 0.2, 0.3, 0.4, 0.5]}, 'name': {'type': 'String', 'value': 'Test Record'}, 'price': {'type': 'Float', 'value': 99.99}, 'id': 'LuEg0X3IRNqZ111EVB9EVL-nxhCWjNOzlRybJ0u7kj3dbQbxp4lTRf_Mkkzju6soe4Vrj4J0KXUl_JeIHZjULg'}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): True
  price (Decimal): 99.99
  created_at (DateTime): 2026-01-07 20:16:41.044178
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): ['tag1', 'tag2', 'tag3']
  metadata (Object): {'key': 'value', 'nested': {'deep': True}}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): ['electronics', 'computers']
  data (Bytes): 11 bytes
Plain record: {'user_id': '550e8400-e29b-41d4-a716-446655440000', 'metadata': {'key': 'value', 'nested': {'deep': True}}, 'created_at': '2026-01-07T20:16:41.044178', 'value': 42, 'tags': ['tag1', 'tag2', 'tag3'], 'data': 'aGVsbG8gd29ybGQ=', 'categories': ['electronics', 'computers'], 'active': True, 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5], 'name': 'Test Record', 'price': 99.99, 'id': 'LuEg0X3IRNqZ111EVB9EVL-nxhCWjNOzlRybJ0u7kj3dbQbxp4lTRf_Mkkzju6soe4Vrj4J0KXUl_JeIHZjULg'}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'data': {'type': 'String', 'value': 'aGVsbG8gd29ybGQ='}, 'created_at': {'type': 'String', 'value': '2026-01-07T20:16:41.044178'}, 'embedding': {'type': 'Array', 'value': [0.1, 0.2, 0.3, 0.4, 0.5]}, 'price': {'value': 99.99, 'type': 'Float'}, 'value': {'type': 'Integer', 'value': 100}, 'categories': {'type': 'Array', 'value': ['electronics', 'computers']}, 'name': {'value': 'Updated Record', 'type': 'String'}, 'metadata': {'type': 'Object', 'value': {'nested': {'deep': True}, 'key': 'value'}}, 'user_id': {'type': 'String', 'value': '550e8400-e29b-41d4-a716-446655440000'}, 'active': {'value': True, 'type': 'Boolean'}, 'id': 'LuEg0X3IRNqZ111EVB9EVL-nxhCWjNOzlRybJ0u7kj3dbQbxp4lTRf_Mkkzju6soe4Vrj4J0KXUl_JeIHZjULg', 'tags': {'value': ['tag1', 'tag2', 'tag3'], 'type': 'Array'}}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: dYqRCNHYQ5OnI8a7J33yDLebOqojjK5TWwqOnbC7_zpuQUYpkVqVuE_yX8BT8QiLC6t_-NqOFbgS8wdzFWkGLw

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
Collection created with first record: "zwIv1lrrfqfm5BbWqU8guO2sdIdq8fkvC_1e7U_KYUNOKSlBrCx2BtN37vXJwgdXdalY7pu9cQlFOUSFsf_nGw"

=== List Collections ===
Total collections: 12
Sample collections: ['websocket_test', 'chat_configurations__ek0_testing', 'test_accounts', 'test_collection', 'chat_messages__ek0_testing']

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
Total keys in store: 8

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: False

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: ouQ1ghlh6vuuwqJ8LB3UmZYQYuIRqluNM_uw6WB9sex65wQvCFKIY56l4KXOtxBXWrLwc96mzmyarAvUo2x_QQ
Created Bob: $500 - ID: 2y2sq-rJZneraJnrsAjDF_Jf13MszN85L_WQkMj2emZEgkEiUUGIM2BIftlTkxGgzLzoPYKbuEpaiC-wx2vzkw

=== Example 1: Begin Transaction ===
Transaction ID: 626ee6a9-2ea0-4c4c-9279-7de266cdc8f3

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: ec1a2612-b023-485c-a302-1cefbfd306c1
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
  1. Score: 26.400, Matched: bio, title, bio.value, title.value
  2. Score: 26.400, Matched: bio.value, bio, title.value, title
  3. Score: 26.400, Matched: bio, title, title.value, bio.value
  4. Score: 26.400, Matched: title, title.value, bio, bio.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title, title.value, bio, bio.value
  2. Score: 39.600, Matched: title.value, bio, bio.value, title
  3. Score: 39.600, Matched: title, bio.value, title.value, bio
  4. Score: 39.600, Matched: bio.value, bio, title, title.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio.value, bio

5. Search with stemming and exact match boosting:
Found 2 results (matches: run, running, runs)
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.750
  2. Score: 0.742
  3. Score: 0.732

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.371, Matched: title, title.value, content, content.value
  2. Score: 1.366, Matched: content, title.value, title, content.value
  3. Score: 0.375, Matched: 

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
✓ Inserted document: QUflwO-nYXa4WkUl75R93Kq-VBRP20_E1CFLAhHApwd2zC--QAFk7LtZUmJZL1YqgpLmfEANTWTcdlBUyQxLsg

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: TLNd22slliOEQljHK2H06XN8ldb1FfIlYzGMQE0qh3OvNkPhUELw__rGa_mmJAJGor5tcSZK9JZH-RqVpJwBvg

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
✓ Inserted document with TTL: QEAyNaQpEKeciOUQ3KctlycUje7S-t-dB7Kaso_W229RaGtZii0gj3BXLP8ScrRFdMdZ1cllHBS8f8-bHetxjw

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
✓ Edge cache script created: zRoXF8QkziXIvnPj6rb4RSUPRBj5KH_HS2koNau-sqE17o1tGkdUbYOStlAIlyuMd06XRb7x0apkTvYE-ELwEA

Call 1: Cache miss (fetches from API)
Response time: 1ms
Result: {
  "records": [],
  "stats": {
    "execution_time_ms": 0,
    "input_count": 0,
    "output_count": 0,
    "stage_stats": [],
    "stages_executed": 2
  }
}

Call 2: Cache hit (served from ekoDB)
Response time: 1ms (1.2x faster!)
Result: {
  "records": [],
  "stats": {
    "execution_time_ms": 0,
    "input_count": 0,
    "output_count": 0,
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

✅ Script saved: E2McJ1sBTTO13yKVvALk9124J0A-GW9bFh_jm2Og_9Rjc1v9F1hxRdzBscStQEOFyYrkrR-y11038ba6f032XQ
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved: ixAmiYJqhQFJsC45fInfySEGkX03xFyjY27SdMJZnhuz_1Fd7x98H-5weYm3VIqVC_uqMLAUx3XK6cK2oLdLbQ
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: Ex5liw1LFdLA-SGYBuow0KVwde6DxJV93MWwFTr_yT1-BOkXr6Fdzo5q51ayGiFViHD1yoC2n_R6AtTXzI_Qrw
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

Using CallFunction to replace inline logic in SWR pattern...

✅ Saved reusable function: fetch_and_store_github
✅ Saved SWR function using composition: swr_github_user

First call (cache miss - will fetch from GitHub):
   ⏱️  Duration: 134.5ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 80.2ms
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

✅ Script saved: 9YN55EjUe0TgYXUy7lPvvy_EyvplZLaAxEF-LfCGWuftpfC1lTeFQCTpyz2q8zTfkjZ4bY2sxQqNM6SrN1Oq_Q
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

✅ Inserted order: 9yVN0EaZwBIeLzqQvMnoeutVWk_c6oI88TEnDBMm6MjhuM8vqj_cVYYJgXqYGwtFnhauhk99hvIFTZeX6LS03Q
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

✅ Inserted order: _Wbeyv_tI8fZ0vSKo6aT4Ak9bSd1HqXBYai2pAeD4Tl_P8cW9WKXj78cZGfDnVEgTTH7qVe_blom3PpGIKFoTQ
✅ Cached order status
📊 Quick status lookup: {'value': '{"status":"processing","updated_at":"2026-01-08T01:16:43.622081+00:00"}'}

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
✓ Created SWR script: fetch_github_user_py (tjHHfrCsJFgX7MRGIFct-GX3ARFMi5MsN6yB94X4Yps8CrhwZtsfSJGLPhU54NmN20Wjv85EBz3aX-1M82Wb0w)

Step 2: First call - Cache miss, fetches from GitHub API
Result: {
  "records": [],
  "stats": {
    "execution_time_ms": 0,
    "input_count": 0,
    "output_count": 0,
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
   Vector databases have several benefits including high precision, ability to store complex data types such as points, lines, and polygons, support for topological analysis, and efficient spatial operations. They also allow for better visual representation of data and can store attributes like direction and connectivity.
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
   active: 7 users
   inactive: 3 users
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
✓ Created session: oqMt7Ogs0ZHIcPwl2kJjGKykHiGfcrJ32htPbW4fiv8eN0ClrX1rcTKvGMK9f_I8n_GL_iZnOonVi7Vz15AGAg

=== Sending Chat Message ===
Message ID: oXZUepvDbsGLWn1jC_svKsGD72CXk_CQPBjIHnmDXa1NnQH9EIZQFIwvNMNbsPQQxDkMikhtfEkJVl7-TvZuHQ

=== AI Response ===
There are three products available:

1. ekoDB Pro: This is an Enterprise edition product with advanced features. The price for this product is $299.

2. ekoDB: This is a high-performance database product with AI capabilities. The price for this product is $99.

3. ekoDB Cloud: This is a fully managed cloud database service product. The price for this product is $499.

Execution Time: 2370ms

=== Token Usage ===
Prompt tokens: 447
Completion tokens: 84
Total tokens: 531

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: K7VuyQwAnmFvjIEo8cyhnSVZVW55EtxtSrVwXuMOCepnhd8vjYqIpBWGQUtQK6g2HIVKesJU7ptgmIvlN9mMUw

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information provided, there is a high-performance database product available named ekoDB. It costs $99.

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['chat_id', 'updated_at', 'id', 'content', 'created_at', 'context_snippets', 'role', 'token_usage'])
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
✓ Created second session: Bg740pblowaCtxTkdGKFhaJLWzCwIMey-ah00qOlMj3rx5TlfQzPpITnoquy86ak3fOozB_lSks_gxC3cQQiVQ
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
✓ Created session: aZdrcSeIjeyVGJsYPzGBaySnv7pp-AdcgCG4v4JHTyDaDani-qspk-Lg5xexSo6BRPtrryL-3UyGX1KowazhTg

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information provided, the available product is "ekoDB" which is a high-performance database product. It is priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: EbRVbnPmyXaQRdGp3lyMURJfi8i9bEsqpnmJRzcAMhdnRNNRmiWWzGNuN7IJqosNoFZedizxauYmfMB2P8YUNA
  Parent: aZdrcSeIjeyVGJsYPzGBaySnv7pp-AdcgCG4v4JHTyDaDani-qspk-Lg5xexSo6BRPtrryL-3UyGX1KowazhTg

=== Listing Sessions ===
✓ Found 7 sessions
  Session 1: EbRVbnPmyXaQRdGp3lyMURJfi8i9bEsqpnmJRzcAMhdnRNNRmiWWzGNuN7IJqosNoFZedizxauYmfMB2P8YUNA (Untitled)
  Session 2: aZdrcSeIjeyVGJsYPzGBaySnv7pp-AdcgCG4v4JHTyDaDani-qspk-Lg5xexSo6BRPtrryL-3UyGX1KowazhTg (Untitled)
  Session 3: Bg740pblowaCtxTkdGKFhaJLWzCwIMey-ah00qOlMj3rx5TlfQzPpITnoquy86ak3fOozB_lSks_gxC3cQQiVQ (Untitled)
  Session 4: oqMt7Ogs0ZHIcPwl2kJjGKykHiGfcrJ32htPbW4fiv8eN0ClrX1rcTKvGMK9f_I8n_GL_iZnOonVi7Vz15AGAg (Untitled)
  Session 5: VnNDJIzivJGO-d6oYiThOL4dCjnRG0KkSk1xl7ysw5bA_veOV24kg5TsFoB_K1zKD79cwdUy_OPHzihTyAm8oA (Untitled)
  Session 6: mjHaaemRw7nqTA5cYf1lagsjBRNjghVs1oZGG8L1ld3XeP4oI_uxUWidyh0HYMRhcA-oeBKWNZGVuv4HZXsZGg (Untitled)
  Session 7: hKKsKC1L8ctguKeCb1OEaET9cHM8sKVVkCA72aSXWARAxinotoGJWbR5nBpyZsuXQNZCbp-ojhXbDMQeU9MoKQ (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: EbRVbnPmyXaQRdGp3lyMURJfi8i9bEsqpnmJRzcAMhdnRNNRmiWWzGNuN7IJqosNoFZedizxauYmfMB2P8YUNA

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Dict Creation ===
✓ Created record with plain dict: {'id': 'vj8uyOOH9y9kXUow2nGTrcs_YtFoHFoCajeflFhDdImvW8xAIVmunOChNUTV7Nigb2uVqT6bXlNEXspuN8oe_g'}

=== Upsert Operation ===
✓ Upsert (update existing record): vj8uyOOH9y9kXUow2nGTrcs_YtFoHFoCajeflFhDdImvW8xAIVmunOChNUTV7Nigb2uVqT6bXlNEXspuN8oe_g
✓ Inserted second record: bOxyjgSD4MokOjvGVCzXJhW-7ZYV6YYtSKDkaO13TNwN0g5muJvNrC-_Dj10_WlUHGJ05wBc2sE8Oc7YTkgW0Q
✓ Upsert (update second record): bOxyjgSD4MokOjvGVCzXJhW-7ZYV6YYtSKDkaO13TNwN0g5muJvNrC-_Dj10_WlUHGJ05wBc2sE8Oc7YTkgW0Q

=== Find One Operation ===
✓ Found user by email: {'active': {'value': True, 'type': 'Boolean'}, 'id': 'vj8uyOOH9y9kXUow2nGTrcs_YtFoHFoCajeflFhDdImvW8xAIVmunOChNUTV7Nigb2uVqT6bXlNEXspuN8oe_g', 'name': {'type': 'String', 'value': 'Alice Johnson'}, 'age': {'value': 29, 'type': 'Integer'}, 'email': {'type': 'String', 'value': 'alice.j@newdomain.com'}}
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
   Inserted with ripple: {'id': 'ZBjchqGkaquifCatm02KYPLmKyz6AhqjcnZWHGwsBePpmQi532fcoNrdRIF9_ICrnkxzb2YaNEZ1q9K4jV77bA'}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {'id': '2-embVBZBHOKJC629BTWyv0Ir1kZnTcP3xW1MSR5G75MheFytl09g6xBnxeeyqZtsJMN5-6vk4IeFbF4zeyZ4A'}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {'price': {'type': 'Integer', 'value': 150}, 'id': 'ZBjchqGkaquifCatm02KYPLmKyz6AhqjcnZWHGwsBePpmQi532fcoNrdRIF9_ICrnkxzb2YaNEZ1q9K4jV77bA', 'name': {'value': 'Product 1', 'type': 'String'}}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {'name': {'type': 'String', 'value': 'Upsert Product'}, 'id': 'ZBjchqGkaquifCatm02KYPLmKyz6AhqjcnZWHGwsBePpmQi532fcoNrdRIF9_ICrnkxzb2YaNEZ1q9K4jV77bA', 'price': {'type': 'Integer', 'value': 500}}

✅ All bypass_ripple operations completed successfully!
✅ [32mPython client examples complete![0m
