make test-examples-python
🧪 [36mRunning Python examples (direct HTTP/WebSocket)...[0m
=== Simple CRUD Operations (Direct HTTP) ===

✓ Authentication successful

=== Insert Document ===
Inserted: {'id': 'ACFxHhedVNMCt4mje0-aUM359sNnaBz-T4jUHccwkkxmLgcwlzBk3HEAjG6FuQ2bJhIF2eYFGyVRc2lwu6ZmKA'}

=== Find by ID ===
Found: {'name': 'Test Record', 'active': True, 'id': 'ACFxHhedVNMCt4mje0-aUM359sNnaBz-T4jUHccwkkxmLgcwlzBk3HEAjG6FuQ2bJhIF2eYFGyVRc2lwu6ZmKA', 'value': 42}

=== Find with Query ===
Found documents: [{'name': {'type': 'String', 'value': 'Test Record'}, 'id': 'ACFxHhedVNMCt4mje0-aUM359sNnaBz-T4jUHccwkkxmLgcwlzBk3HEAjG6FuQ2bJhIF2eYFGyVRc2lwu6ZmKA', 'active': {'value': True, 'type': 'Boolean'}, 'value': {'value': 42, 'type': 'Integer'}}]

=== Update Document ===
Updated: {'name': {'value': 'Updated Record', 'type': 'String'}, 'id': 'ACFxHhedVNMCt4mje0-aUM359sNnaBz-T4jUHccwkkxmLgcwlzBk3HEAjG6FuQ2bJhIF2eYFGyVRc2lwu6ZmKA', 'value': {'type': 'Integer', 'value': 100}, 'active': {'value': True, 'type': 'Boolean'}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: V-GwO5brmP15gkZCWnGtp4o8ngZYVpyshElKjClatQ0lIDPngkQSMEFFFJNE6mPPBwj0_pYxOs-mnpoye8qKTw

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
        "id": "2ohhdIKZs8t7QdU7ZWUXCc4EPm8T-Iy-RVYfPio_EofPFq65P9o3SaJhKtwl8h8xuajQE9DK3IsJAV0tEM2-dw",
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
        "id": "V-GwO5brmP15gkZCWnGtp4o8ngZYVpyshElKjClatQ0lIDPngkQSMEFFFJNE6mPPBwj0_pYxOs-mnpoye8qKTw",
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

✅ Script saved: R2vVTgh-4iqBZ7I1zuxVWXCZJ5sfiKUrDJ-hN7cNVUfKrRDwCYdia8X1D24BZUPyYvf5SwTxOq6sN1c9xRBv5w
📊 Found 15 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: G5qzMg0dp-hby7b2lqUPR3Qp3rKxctMeTcC1iaeV6G0I5vfDHzHN_kMzYTiStHvbzQJ2M6ZB9TmaJ0_O9wywAQ
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: J5mzNE9JNtAo5ZA9TJX5XKBQ3egfqhGSE_hVDPw4DDyL2gYzEV1ieySMLuUZF2O8ORDKfTE-r1pAYO-lmGY3KA
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {'max_score': 100, 'status': 'active', 'count': 15, 'avg_score': 60.0}
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
cache:product:1: {'price': 29.99, 'name': 'Product 1'}
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
Collection created with first record: Qyg7jJrMMqEgEkT0AXLs_Ji2qVkGJY2I9kO3KG37kAWrESmNsOSYFWkE-Geurrm2vvJBgB7CTu8RKkuo27CKvg

=== List Collections ===
Total collections: 13
Sample collections: ['users', 'ttl_cache', 'test_collection', 'user_cache', 'scripts__ek0_testing']

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
Created Alice: $1000 - ID: jS-GeqSaA5GSvlR73GW3COKu7cpG2vPH6uFm0bztnyr8YurwbY8isqlJcC9AO_Hw6P5GSMcZ-zJH_D305roEVQ
Created Bob: $500 - ID: rCJK5JKvW15r_w20diEuKaOuu92GdSqcylM5iaU-2kfGFtHv6i1PQVkqsJx5lQYTgU_1pq9f2O-GbxYxT_KdUA

=== Example 1: Begin Transaction ===
Transaction ID: 430e776d-6d6f-4d3e-bf16-abb870f1ac5a

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
New transaction: 4aae93b0-b359-47ba-a04f-d0bd217074a9
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: t1Vc6ibuzi0uoLBupd92jxJVdYPO-OZfHweNe8B9D6UkOZE2NtdM_zk7fq_eonK56eWR1QV2zg29KMvVOrfTcw
Created Bob: $500 - ID: n9H61Xkz4uxwxTccXDY4ulgk4nhXkOGOhk_l0XWU13-8vmmRWm_uN41Jer0eTMBhbs3kzJiV_3MGea4fWYRh1Q

=== Example 1: Begin Transaction ===
Transaction ID: 58bf7f99-8bc1-486d-96af-61378cf016aa

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
Created Alice: $1000 - ID: 1sZXMfWNmxXbCj_c5uY5xKsiEsy9isnoPwNq2qN9lEkmccYUPwvzOEI3BEZq6Kfln1qhIg_VjU0l52tLQQbaiQ
Created Bob: $500 - ID: 1CFefpXIPuvvpjhJX7bIsVCzA9MpP7d7wEJC7igA7GFs12IMo6IjbNFlvLP-ZlVrquV844H_pf9dAMzvbqCSnA

=== Example 1: Begin Transaction ===
Transaction ID: f22404e2-012e-4a8f-b320-abd9d3c497a1

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
New transaction: 3739fb95-dd12-4359-8f4d-d11d41f19808
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: puD6-ddeWqtwfeZh_yoWMBfFDH2DyiSU_dxxUv0QoBIjeZmTboNusS7hjWSKUljg1eA7K29BNZY_-LYCmUJtmQ
Created Bob: $500 - ID: lSbaIVulybUhfX2Whm1w23uO_-8GYBb06mr0woGmeLSwONUYAxfWKAAGzkaIcp_sTeqzhjEyEsIAPSuf59_sTw

=== Example 1: Begin Transaction ===
Transaction ID: 5d4a3902-d249-403d-9375-c6659fe5a14e

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
Created Alice: $1000 - ID: O55mer6AjOMIZUNDfznjC6b-msIvPoRKI6u9-PuXDIfo7v1NAWFZ-BFtLDMy5M6t0_Guke96o5t17zum5dCtPA
Created Bob: $500 - ID: GjT96ds9GVpXLbbuQyZwc40nIXLum8dhxXTtBvJ8aFiCyy0P_MXaFoHmee9OGa8kBS4n3ME4jwCEtPivXwVeSw

=== Example 1: Begin Transaction ===
Transaction ID: 173d70fc-7e2e-473e-a676-c38afde904d1

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
New transaction: 2f28daa5-92b5-40a1-b0ed-5f66785f0d27
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: JushUXofzvpPoPxDYG7sP1M1ZZJHPg5WWICe-5_y4s8t06wy-r-xoXAG0znz63zx9FO2gcW3tqJJCThbIK3EGg
Created Bob: $500 - ID: j6hhV_WEni5EQPQ9GM2B1MRDDOyQ-HyD9iMCJAFiTiLgEAngcUUEAF7V_qFToevbQKdY0jKshRfFEKuRfR9GhQ

=== Example 1: Begin Transaction ===
Transaction ID: a708b7f4-ded1-4588-92f1-23020ba5021e

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
   ✅ Script saved: xI5rIpR_Py1pNLaBTS6rJe_qWzUv3xwx6A_oe_HPXinAQpOTjpkaA_0SXUbooIn6tSaGQKAPiLi9NgO-Rep10A

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: rkyx_dQZr8gF1rs2LTvZuNLKQ0x-DeGNfsEfGztjogTlLowzjjwD-i6ec4Ekli3QU1q8ZcaggpafGj49D5st9A
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}
   📋 Email: {'value': 'alice@example.com', 'type': 'String'}
   📋 Status: {'type': 'String', 'value': 'pending'}
   📋 Credits: {'value': 0, 'type': 'Integer'}

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: j9roLMKNsSxJd5JqqsKie5voxq4fSH-sNBGuJKw4PLVCZf4QZZ2KvjvqYOgtWtFl-JLRjK5Ht60Sq_DsShAgkA

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
   ✅ Script saved: ec9zQizGQlLUfFrJPSpqgUyZl3SMtgC_La3qx9O9woNvVKKIkvPhJ8E4_TNdZnQ0O7WdLb-korYdhR3BHiGwQQ

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
   ✅ Script saved: Qe094OUrC9gPXYKkJx4Lt2aeGqrkfK05oG6Y_r7_wNapGDQBV7E393CwQ4eCiz5D9ZwikZx16bR-X4rpTxHXhQ

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: xI5rIpR_Py1pNLaBTS6r...
   ✅ Deleted script: j9roLMKNsSxJd5JqqsKi...
   ✅ Deleted script: ec9zQizGQlLUfFrJPSpq...
   ✅ Deleted script: Qe094OUrC9gPXYKkJx4L...
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
  Output: Document ID = Rq-u73tglFO3Y0oxqq6-NZCfqvNe2nSFRGG4eM-dUfJjdtJB5a5OUFac6VHgVUtfe6oCH4tFz7dK19IroqPG_Q
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: find_by_id(Rq-u73tglFO3Y0oxqq6-NZCfqvNe2nSFRGG4eM-dUfJjdtJB5a5OUFac6VHgVUtfe6oCH4tFz7dK19IroqPG_Q)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: find_by_id(Rq-u73tglFO3Y0oxqq6-NZCfqvNe2nSFRGG4eM-dUfJjdtJB5a5OUFac6VHgVUtfe6oCH4tFz7dK19IroqPG_Q)
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
  Output: Document ID = COR3bjv5PSiw-z-v1hvHN3RWR0N98x0EuwNyHgTmkHNfW9Kr-Tqr6-amaoS7skx8M-UnldzaolP8NbQ7YpXqJw
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: find_by_id(COR3bjv5PSiw-z-v1hvHN3RWR0N98x0EuwNyHgTmkHNfW9Kr-Tqr6-amaoS7skx8M-UnldzaolP8NbQ7YpXqJw)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: find_by_id(COR3bjv5PSiw-z-v1hvHN3RWR0N98x0EuwNyHgTmkHNfW9Kr-Tqr6-amaoS7skx8M-UnldzaolP8NbQ7YpXqJw)
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
    Finished `release` profile [optimized] target(s) in 0.08s
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
Inserted: {'id': 'wv-zawRXOT_z9nw5Ekrf9hhjDn1Y9sc1Dq-vs3O_VgcS9BzMp2mkpZYTwkHaGqFXc1_-73o5wjG0lSspGEem5w'}

=== Find by ID ===
Found: {'categories': ['electronics', 'computers'], 'value': 42, 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5], 'tags': ['tag1', 'tag2', 'tag3'], 'active': True, 'created_at': '2026-01-26T03:00:50.465258', 'data': 'aGVsbG8gd29ybGQ=', 'price': 99.99, 'metadata': {'nested': {'deep': True}, 'key': 'value'}, 'name': 'Test Record', 'user_id': '550e8400-e29b-41d4-a716-446655440000', 'id': 'wv-zawRXOT_z9nw5Ekrf9hhjDn1Y9sc1Dq-vs3O_VgcS9BzMp2mkpZYTwkHaGqFXc1_-73o5wjG0lSspGEem5w'}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): True
  price (Decimal): 99.99
  created_at (DateTime): 2026-01-26 03:00:50.465258
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): ['tag1', 'tag2', 'tag3']
  metadata (Object): {'nested': {'deep': True}, 'key': 'value'}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): ['electronics', 'computers']
  data (Bytes): 11 bytes
Plain record: {'categories': ['electronics', 'computers'], 'value': 42, 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5], 'tags': ['tag1', 'tag2', 'tag3'], 'active': True, 'created_at': '2026-01-26T03:00:50.465258', 'data': 'aGVsbG8gd29ybGQ=', 'price': 99.99, 'metadata': {'nested': {'deep': True}, 'key': 'value'}, 'name': 'Test Record', 'user_id': '550e8400-e29b-41d4-a716-446655440000', 'id': 'wv-zawRXOT_z9nw5Ekrf9hhjDn1Y9sc1Dq-vs3O_VgcS9BzMp2mkpZYTwkHaGqFXc1_-73o5wjG0lSspGEem5w'}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'id': 'wv-zawRXOT_z9nw5Ekrf9hhjDn1Y9sc1Dq-vs3O_VgcS9BzMp2mkpZYTwkHaGqFXc1_-73o5wjG0lSspGEem5w', 'tags': {'value': ['tag1', 'tag2', 'tag3'], 'type': 'Array'}, 'metadata': {'value': {'nested': {'deep': True}, 'key': 'value'}, 'type': 'Object'}, 'active': {'value': True, 'type': 'Boolean'}, 'data': {'value': 'aGVsbG8gd29ybGQ=', 'type': 'String'}, 'created_at': {'value': '2026-01-26T03:00:50.465258', 'type': 'String'}, 'embedding': {'value': [0.1, 0.2, 0.3, 0.4, 0.5], 'type': 'Array'}, 'value': {'type': 'Integer', 'value': 100}, 'categories': {'value': ['electronics', 'computers'], 'type': 'Array'}, 'name': {'value': 'Updated Record', 'type': 'String'}, 'price': {'value': 99.99, 'type': 'Float'}, 'user_id': {'value': '550e8400-e29b-41d4-a716-446655440000', 'type': 'String'}}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: rZAETHIc04XZeEiZfyEvawbxrqJV5tjtZOGa58_vvVq1xMK6MeEctnr-BUXtQ_Hcp6lNGCXjtilyYoK1IpQseA

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
Collection created with first record: "ZzlXi5FhVDJ_9_8WaU7bSPvYLVjw5Uvj2CPUexklafGkWaeQECo-ZEoAggX26Ueo9Be7mi6Hpg_7ihu_aEZI3w"

=== List Collections ===
Total collections: 12
Sample collections: ['ttl_cache', 'test_collection', 'user_cache', 'scripts__ek0_testing', 'products']

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
  cache:product:3: {'name': 'Product 3', 'price': 49.99}

=== KV Exists ===
Key exists: True

=== KV Find (Pattern Query) ===
Found 3 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 8

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
Created Alice: $1000 - ID: op-7B3o3oKe0Yh4pn8ZksXD_ZtuqBzQ5wpegVx24yn8Fd3LJuIqn3PKW5yX12uwqneGqOVMofr9twNLP3ppu9w
Created Bob: $500 - ID: QIbcXs8cC78ZfZ2Wq3KPV6Xyh-qz9FuulQA3LLm28Zz5spgOLEhqoLMT920aonw8zDZ-wSeV8uIHpzwYQdt86g

=== Example 1: Begin Transaction ===
Transaction ID: d732af89-71aa-44a2-b271-973a535f9ee7

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: f9b51cc0-f137-41e5-a304-ee285dcf9046
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
  1. Score: 25.740, Matched: email.value, name, email, name.value
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: bio.value, title.value, title, bio
  2. Score: 26.400, Matched: title.value, bio, bio.value, title
  3. Score: 26.400, Matched: bio.value, title.value, title, bio
  4. Score: 26.400, Matched: bio, bio.value, title.value, title

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio, title, bio.value, title.value
  2. Score: 39.600, Matched: bio.value, title, bio, title.value
  3. Score: 39.600, Matched: bio.value, bio, title, title.value
  4. Score: 39.600, Matched: bio, title.value, bio.value, title

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 2 results (matches: run, running, runs)
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.763
  2. Score: 0.737
  3. Score: 0.727

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.381, Matched: title.value, title, content.value, content
  2. Score: 1.368, Matched: title, content, content.value, title.value
  3. Score: 0.363, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 13.200, Matched: title, title.value

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
✓ Inserted document: urCX6UNZOnO9fslJQo0DLCN1u1YK304vMDtlf8pXGmiw-pMVGDwula0xvr1XuZQ01pJUV3W-8BDYn7RNy7zW0A

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: 0lkuyAJ1LDDoVqjPvKt-_C_SBMuFSJhlVRttpU_zVPiSOFsgkQwo9kLhC6QRHhPup5Oi2mktT2zGfmOiNIt8ag

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
✓ Inserted document with TTL: mg8I_6KtP214xoq684T4epkD5IbkrJY6UoIOKHXIvmfXWXwBi9G5M1pzQux0CjJF_S3xbzoWO1CzLVkvemD0oQ

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
✓ Edge cache script created: MOOOrt7qSspbykyKQCHN-w-R0AQySWgYHoXZV-512HYO6hDJvrIpAKAWSZnIHmL7X350c0kscJ4BBo49ihGhVA

Call 1: Cache miss (fetches from API)
Response time: 2ms
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

✅ Script saved: PC0cJpiDKiH7F2lWoIChHMVs1KZy8fT40gObmOjDETlV349U1bOG6afcTOCssCwQ-Gs0eaGrM9n421b8f3POKg
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved: XdPZpzqwekFPb-WPIY5WplON2lBk8DqiCWgxJ7vdkH2Dh9uRBaNXRtEnNxhwNAx3F2EuRPKAS-eSrTJO1KvjUQ
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: 5E06eKPnbRfPQ3l2DMZ7fbFgJZevJuqgdLo9I254WDH6Ggk8N3tJ14BOGXgTJixliR1DjANELqlLLxDI1SUdZw
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

✅ Saved reusable function: fetch_and_store_user
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 75.9ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 64.1ms
   📊 Records: 1
   🚀 Cache speedup: 1.2x faster!

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

✅ Script saved: tjKDOG7MYGRZqwBrQwGSqgY1Bo2ThEEjT0mtwfav2zmhxXVWslGaqixWvW2X0ZoRlBmzOXn_yw5SxJXjoJJh1g
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

✅ Inserted order: gJ1ewrCAGGxshNc3_7Z-fFpk_pbhcukNe_TrYfXEWTTkTdZPE1b83RI_jZLxMISQH5sO61sha5hYjiUVrCM1JA
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

✅ Inserted order: 5a5E17jTyaQ_PExNa_GNK2442GDAq8zd3mXNlTA1LW8RxPYSySCR8Z2Z2kS7yGYLxwswpMXsp4FvwiZms1ECeA
✅ Cached order status
📊 Quick status lookup: {'value': '{"status":"processing","updated_at":"2026-01-26T08:00:52.368533+00:00"}'}

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
✓ Created SWR script: fetch_api_user_py (moVKimK3MttY3Kb-oIBZ88mDrUSWAtX4T9pH9xO04Ho_6g4iMpIVe62MQlNYlR45yogFTkjdjsjkvpFWeVdjaw)

Step 2: First call - Cache miss, fetches from API
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
   Vector databases offer benefits such as efficient storage, precise representation of data, high scalability, and the ability to handle complex, multi-dimensional data. These databases are ideal for geographical data management and spatial analysis.
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
   1. Database Design Principles (Database)
   2. Natural Language Processing (AI)
   3. Getting Started with ekoDB (Database)
   4. Vector Databases Explained (Database)
   5. Introduction to Machine Learning (AI)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Script saved
📊 Documents by category:
   {'category': 'Database', 'count': 3}
   {'category': 'AI', 'count': 2}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: r3WEzajhiOln7bNynUKzsDgIZFkUdrh2pPsREnAIE12Hs9SDxUUNYqdLerZ05GZScJZq9i4hw9rGdStaEqZt-g

=== Sending Chat Message ===
Message ID: wKltwjXxULjzJHX3qSOGrQXv3Gp7VFQbO0s8vwLBxot-jTVxavSTakmJRwO4WKaPlLmoVh9fi3noRsVQOljrrg

=== AI Response ===
We have three products available:

1. ekoDB Pro: This is an enterprise edition product with advanced features, priced at $299.
2. ekoDB Cloud: This is a fully managed cloud database service product, priced at $499.
3. ekoDB: This is a high-performance database product with AI capabilities, priced at $99.

Execution Time: 2893ms

=== Token Usage ===
Prompt tokens: 450
Completion tokens: 72
Total tokens: 522

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: -gPuox-DNU-wzKh1HuCs130iH34S32C3i_902ZfS4nb09qVe8vl6cmwl1WZY5Maql7B-ZCXJKOn0dxuXldQtVA

=== Sending Initial Message ===
✓ Message sent
  Response: Currently, we have the "ekoDB" available. It is a high-performance database product priced at $99.

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['updated_at', 'content', 'token_usage', 'created_at', 'context_snippets', 'chat_id', 'id', 'role'])
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
✓ Created second session: GtfMq8eRFBL5hDUmOLtkGd4bjUEosRy4nkpiK5_-XmqIIiKyyeNwaHW-_s-pEBkz5CEYEcuVDBgdrN47637TsQ
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
✓ Created session: YZTKwgcM0LUFIISSEphG7T2aRGMZCoX5bTTZbWuYOXK7-hZ_QKRkBxC3Qe3kv1VD2_eD7VMd_bywYTEFyyu6zw

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information provided, the available product is "ekoDB". This is a high-performance database product. It is priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: HbtOyCjlu64LLquUo_Igx9_Fuz3JVUVHr2jJpVQU5WbqRkt2Yg1V6Si2noxeuSGu9lQXoxc201I0PaTN45hyZg
  Parent: YZTKwgcM0LUFIISSEphG7T2aRGMZCoX5bTTZbWuYOXK7-hZ_QKRkBxC3Qe3kv1VD2_eD7VMd_bywYTEFyyu6zw

=== Listing Sessions ===
✓ Found 7 sessions
  Session 1: HbtOyCjlu64LLquUo_Igx9_Fuz3JVUVHr2jJpVQU5WbqRkt2Yg1V6Si2noxeuSGu9lQXoxc201I0PaTN45hyZg (Untitled)
  Session 2: YZTKwgcM0LUFIISSEphG7T2aRGMZCoX5bTTZbWuYOXK7-hZ_QKRkBxC3Qe3kv1VD2_eD7VMd_bywYTEFyyu6zw (Untitled)
  Session 3: GtfMq8eRFBL5hDUmOLtkGd4bjUEosRy4nkpiK5_-XmqIIiKyyeNwaHW-_s-pEBkz5CEYEcuVDBgdrN47637TsQ (Untitled)
  Session 4: r3WEzajhiOln7bNynUKzsDgIZFkUdrh2pPsREnAIE12Hs9SDxUUNYqdLerZ05GZScJZq9i4hw9rGdStaEqZt-g (Untitled)
  Session 5: txXTU787YDCmI_8Mu2xbUdXH1CuPDrTSreIwj10JqQxeQfx-c4ezBzvpIRsHlLBxNzJMQCuJkifsxvald9nfiw (Untitled)
  Session 6: FgQNl1wmCTOX8XWaikjWKnJh5jz1vLej2PnipxoS35AnUh8n2sqqRoULUiv_bYmC1vk9OP-5AowIoQaNZhkzmw (Untitled)
  Session 7: fHsXg70v83-LYbtuzLXIlj7wO9HrxMk8LNNXdJIlO0x3bzdjAUXMoATyoWAIe7npLkz5Docj-wiZlS0gODtFgw (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: HbtOyCjlu64LLquUo_Igx9_Fuz3JVUVHr2jJpVQU5WbqRkt2Yg1V6Si2noxeuSGu9lQXoxc201I0PaTN45hyZg

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Dict Creation ===
✓ Created record with plain dict: {'id': 'z5K5oAd_HxOFzxeqWN4Vl4eHZtVizCSz63P87RW8sVmqePhi0Q1Uz1hSUUe77FuU06wo9LNALQj-CI32F_1ZDA'}

=== Upsert Operation ===
✓ Upsert (update existing record): z5K5oAd_HxOFzxeqWN4Vl4eHZtVizCSz63P87RW8sVmqePhi0Q1Uz1hSUUe77FuU06wo9LNALQj-CI32F_1ZDA
✓ Inserted second record: kLDj6CCkbD7YB8qQE7jWVhiInq771vfGUeXsNcqpqWxmbYgfyoxza_Jk_ptzl160shhiLnR_iWtHMK8uKJebSA
✓ Upsert (update second record): kLDj6CCkbD7YB8qQE7jWVhiInq771vfGUeXsNcqpqWxmbYgfyoxza_Jk_ptzl160shhiLnR_iWtHMK8uKJebSA

=== Find One Operation ===
✓ Found user by email: {'active': {'type': 'Boolean', 'value': True}, 'age': {'type': 'Integer', 'value': 29}, 'id': 'z5K5oAd_HxOFzxeqWN4Vl4eHZtVizCSz63P87RW8sVmqePhi0Q1Uz1hSUUe77FuU06wo9LNALQj-CI32F_1ZDA', 'email': {'value': 'alice.j@newdomain.com', 'type': 'String'}, 'name': {'value': 'Alice Johnson', 'type': 'String'}}
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
   Inserted with ripple: {'id': 'LLYSuADeemyq8UT4eM5b8C698JKUxpBT5dcHam9Lk786fN92di17FGuooSSzz7SHYYe1TqXPeHuOWASTMGQxcA'}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {'id': 'A_DYAfaG8cFug0hF89gLVkkv-aXqbEZTey0Mz7etHhAHtyxBNYiwuDaVFYj5oyd-nwnpH9-pGSG1fugQsuv6bA'}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {'name': {'value': 'Product 1', 'type': 'String'}, 'id': 'LLYSuADeemyq8UT4eM5b8C698JKUxpBT5dcHam9Lk786fN92di17FGuooSSzz7SHYYe1TqXPeHuOWASTMGQxcA', 'price': {'type': 'Integer', 'value': 150}}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {'price': {'type': 'Integer', 'value': 500}, 'id': 'LLYSuADeemyq8UT4eM5b8C698JKUxpBT5dcHam9Lk786fN92di17FGuooSSzz7SHYYe1TqXPeHuOWASTMGQxcA', 'name': {'type': 'String', 'value': 'Upsert Product'}}

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
