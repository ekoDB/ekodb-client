make test-examples-python
🧪 [36mRunning Python examples (direct HTTP/WebSocket)...[0m
=== Simple CRUD Operations (Direct HTTP) ===

✓ Authentication successful

=== Insert Document ===
Inserted: {'id': 'GLHNbL06-wc0VcxLvk0YfBIsL98u2-qKGtKgcX81fMrnQvCczX9P6wW7km4LfwnfR1b5QdqmrABN5logqEjrLw'}

=== Find by ID ===
Found: {'id': 'GLHNbL06-wc0VcxLvk0YfBIsL98u2-qKGtKgcX81fMrnQvCczX9P6wW7km4LfwnfR1b5QdqmrABN5logqEjrLw', 'value': 42, 'active': True, 'name': 'Test Record'}

=== Find with Query ===
Found documents: [{'id': 'GLHNbL06-wc0VcxLvk0YfBIsL98u2-qKGtKgcX81fMrnQvCczX9P6wW7km4LfwnfR1b5QdqmrABN5logqEjrLw', 'active': {'type': 'Boolean', 'value': True}, 'name': {'type': 'String', 'value': 'Test Record'}, 'value': {'type': 'Integer', 'value': 42}}]

=== Update Document ===
Updated: {'name': {'value': 'Updated Record', 'type': 'String'}, 'id': 'GLHNbL06-wc0VcxLvk0YfBIsL98u2-qKGtKgcX81fMrnQvCczX9P6wW7km4LfwnfR1b5QdqmrABN5logqEjrLw', 'value': {'value': 100, 'type': 'Integer'}, 'active': {'type': 'Boolean', 'value': True}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: V7V0v4mKQGqSaPv_0ST4Gl51DVujlFQPWBPeIOOJJtJ2hekrrygil20odbMkN3rEhNuAxv79PUhjp6aJPJgDMg

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
        "id": "UJ8sqjygQJh0xmSoS1YV0zvVaZmZyqqdFu1OVFWvpebBTmIknmxii-J_gDQXS07nfbIq8eBNXTvj3CxPf1LaXw",
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
        "id": "V7V0v4mKQGqSaPv_0ST4Gl51DVujlFQPWBPeIOOJJtJ2hekrrygil20odbMkN3rEhNuAxv79PUhjp6aJPJgDMg",
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

✅ Script saved: UHycwCXiXTZTSAlLHk2P5VU_DOoO7pue2_TX-GVezDLZlLXHNDcpmLC6pGqZP6ylkszVNDXaBliA9J0S83zucw
📊 Found 35 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: v8V5Ff3Dpjn-7jxAG8uXouqpukvJPpBM6qge7Iqlu5rgXroLtcMjTAmcS97J6xgvK8zi40pU5Q9RcfKDFQ-7gg
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: ncGHVTehkwxR7H6VGbS2aQme7d-UhRI_ReUjXvbE4Tsp5txCaxFGDiqTqq8rAtnkj8XBbUIUkjbqARyzgRqMaA
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {'status': 'active', 'count': 15, 'avg_score': 60.0, 'max_score': 100}
   {'count': 15, 'avg_score': 50.0, 'max_score': 90, 'status': 'inactive'}

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
Retrieved value: {'userId': 123, 'username': 'john_doe'}

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
Collection created with first record: 9vw9jn2NPxR8X35YQO21fbwmsZD-ENn7teBnh1Zz8Ek967gNatK8xp2u6JS_zWHkoqKabEuPoyI6ALpzIsw4Ig

=== List Collections ===
Total collections: 12
Sample collections: ['test_collection', 'demo_collection', 'chat_messages__ek0_testing', 'test_accounts', 'products']

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
Created Alice: $1000 - ID: Lid1chlABAmhFC9wplSTm0zkYwd4rO43QcID_lwGnuvt_pouSPVmdAE_fvsAUG8pQha9TPUw1OC9bHElcBcehg
Created Bob: $500 - ID: ivg_N-KiAfinzZcblXXUm6DhrISTbhVyNXuepW9QjFuI84y-OSU0kY2An32y3bnyLGmo70wU8PEg_NemvFWTrA

=== Example 1: Begin Transaction ===
Transaction ID: d300f51d-c4a2-4c94-b55d-1e2c903799d1

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
New transaction: 567691a4-2345-477e-a8a2-585656048ca7
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: o4vo46HCKWiIPzA5MNtVjeO2WHjAo4J11JdTf9FxrWDGmMa8VBKaGFN29VCDdPPSOXnxljFTpPlePwTLuVrQ1Q
Created Bob: $500 - ID: 4a8ChNeJDCp3Zssr0L5oPufPu4RK40yaeBd8b-lajfIzyRLqVeH1BZ7wZhvDbYAJXld45ZJIBJAZrli_5FjW1w

=== Example 1: Begin Transaction ===
Transaction ID: 68de5484-7adc-4721-bd0b-17f20b5efe21

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
Created Alice: $1000 - ID: K1omGcn_lZ1Zz_bSnYzWJQ3XvW_NS1pIksUrldkjGXGkd7Uhl4dkHYpnQpcohisJqehwrHB-3Nrx9v0T_umRaQ
Created Bob: $500 - ID: LEWlha2lQ1PTFwn2t0-25Cw0ACKxKq83_cILXHbGjkdWQHdEZBnBq3jRNcKoSCN7Y0KkLViw11Z_KFNvVOnNyQ

=== Example 1: Begin Transaction ===
Transaction ID: 18712884-0517-4c7a-81a4-25c090b35934

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
New transaction: 6224006b-b035-4249-b6e4-54d65c3344de
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 6PFSro698puvYoZHCqd_DlPMv0Zz0Us1P6Xt85MrLiw-sPVsX_WfsCmytDlRtpQouVa4ytu1ApUESHPT2NQs8Q
Created Bob: $500 - ID: TF1nPpGRKm6DOkCDMd0ku-3YJKaRL6pkusMZ142nwLL9W8QaNWOLBI2tT8aFO6KcBo7D9fSrNtPSCZI53XvKog

=== Example 1: Begin Transaction ===
Transaction ID: 88605e0d-1a06-4b93-97ea-64ca82a5c717

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
Created Alice: $1000 - ID: BjkH7kl4yN5CbTVp2XEXbiyDrR6QlML5g9xziup7Vs8VCnk5EHNpJqCODrsK_1OVoYYXfaVp4oU3h-N9ess-4g
Created Bob: $500 - ID: da0uZGHvKm9bJGY2H2lXaZ5Fw4u4bhkYLxzkpsWwI0MUlJkeY2OBkNbFwhULZ5V5nbHr_t1R1OdHtngaCCWDUw

=== Example 1: Begin Transaction ===
Transaction ID: 19f7786d-010a-4aca-a11e-f2dffe6c6e37

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
New transaction: 085ec67a-c5d3-4e70-86c3-7b1623cbb020
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: vxQJHaVhuufTQZxYADUCTNNonThm7dzWOYApGC0arynT5AKBNdZVxXj3C5DGn1lnY0AYqjBM6OryOK0rFKRvAw
Created Bob: $500 - ID: uCFL5UskUun1XW-akF4l6kz6otosYCGfOWob_aGjvgqa2-Vb7kIpECrtgh4GAB6H0RdyCTk0V-wh15c0x9WPXg

=== Example 1: Begin Transaction ===
Transaction ID: 9ffc9947-a58c-47dd-beb0-5b83335b0651

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
   ✅ Script saved: ItKcAhqG2zLuxThf5DwBTfc3hAnTAeUNe91n539qDaNCINxXPQSKfPc2db1CFtddyUKVBkuiDy8Uq-CaVA5ilA

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: Nsvold7ewlIN1Zp4uiRUbolmjHDyMgY34Bt6FbtIE_if65NyP5unjcOyphllxltZP8JTW1u7acaGh0fWYZ1Mng
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}
   📋 Email: {'type': 'String', 'value': 'alice@example.com'}
   📋 Status: {'type': 'String', 'value': 'pending'}
   📋 Credits: {'type': 'Integer', 'value': 0}

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: Kv3ctu3TtQOVI8ozCZCBn-SZUnRISFO-TXRHdacgrSJrbKWDKRNvF8jkCJ3Ow8dlLXF9bYFAu9Nq-gOBsDTY-Q

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
   ✅ Script saved: -Np9VPS4maensPFsBmh-WZt4DNQY5k3LiebVnkWHrPXranN_SmhKQPafiZsocKWa1Pn5QH1n8CLZKs3yIEA3BQ

2️⃣ Calling Script (Query + Update Credits + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {'value': 0, 'type': 'Integer'}
   📋 Status: {'type': 'String', 'value': 'active'}
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}

============================================================
📝 Script 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: rEEJvKqNDgN2yl17JwJDlVjlf5qPCa7ew17kT4lbsgsALciUSyVlfAX7eE4rdhX3Tzsk9-za8VE_CvsKfYNt-A

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: ItKcAhqG2zLuxThf5DwB...
   ✅ Deleted script: Kv3ctu3TtQOVI8ozCZCB...
   ✅ Deleted script: -Np9VPS4maensPFsBmh-...
   ✅ Deleted script: rEEJvKqNDgN2yl17JwJD...
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
  Output: Document ID = T0B75fr4wlTrI-ziAD-ab2OAK7y5cmUYk2lU3tDYSykz2JKjBThkzfc4BvPgyMPYYFJD8GYT16fUnzBebjd9lg
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: find_by_id(T0B75fr4wlTrI-ziAD-ab2OAK7y5cmUYk2lU3tDYSykz2JKjBThkzfc4BvPgyMPYYFJD8GYT16fUnzBebjd9lg)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: find_by_id(T0B75fr4wlTrI-ziAD-ab2OAK7y5cmUYk2lU3tDYSykz2JKjBThkzfc4BvPgyMPYYFJD8GYT16fUnzBebjd9lg)
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
  Output: Document ID = ook_gXnYJEZu1AoQjCKiHAbVbNyjtRzzeegYOVvanpQP9bJo5rpJFb5PxFO-wOv-Dkp_JKvoAh02hozC-ZaYFw
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: find_by_id(ook_gXnYJEZu1AoQjCKiHAbVbNyjtRzzeegYOVvanpQP9bJo5rpJFb5PxFO-wOv-Dkp_JKvoAh02hozC-ZaYFw)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: find_by_id(ook_gXnYJEZu1AoQjCKiHAbVbNyjtRzzeegYOVvanpQP9bJo5rpJFb5PxFO-wOv-Dkp_JKvoAh02hozC-ZaYFw)
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
    Finished `release` profile [optimized] target(s) in 0.12s
📦 Built wheel for abi3 Python ≥ 3.8 to /Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.10.0-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel for current platform...[0m
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.10.0-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.10.0
    Uninstalling ekodb_client-0.10.0:
      Successfully uninstalled ekodb_client-0.10.0
Successfully installed ekodb-client-0.10.0
✅ [32mPython client package built and installed![0m
🧪 [36mRunning Python client library examples...[0m
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {'id': 'c_npJnq4LynwMyM7VyJkHT_ojyAtnkdZuwg7Z4HEMl8XjSEJTbAjbW6o6a2zO9EKhEQua3IqDCS1FRn20VPRkg'}

=== Find by ID ===
Found: {'metadata': {'key': 'value', 'nested': {'deep': True}}, 'price': 99.99, 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5], 'data': 'aGVsbG8gd29ybGQ=', 'id': 'c_npJnq4LynwMyM7VyJkHT_ojyAtnkdZuwg7Z4HEMl8XjSEJTbAjbW6o6a2zO9EKhEQua3IqDCS1FRn20VPRkg', 'name': 'Test Record', 'user_id': '550e8400-e29b-41d4-a716-446655440000', 'tags': ['tag1', 'tag2', 'tag3'], 'categories': ['electronics', 'computers'], 'active': True, 'created_at': '2026-01-28T09:42:46.218668', 'value': 42}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): True
  price (Decimal): 99.99
  created_at (DateTime): 2026-01-28 09:42:46.218668
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): ['tag1', 'tag2', 'tag3']
  metadata (Object): {'key': 'value', 'nested': {'deep': True}}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): ['electronics', 'computers']
  data (Bytes): 11 bytes
Plain record: {'metadata': {'key': 'value', 'nested': {'deep': True}}, 'price': 99.99, 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5], 'data': 'aGVsbG8gd29ybGQ=', 'id': 'c_npJnq4LynwMyM7VyJkHT_ojyAtnkdZuwg7Z4HEMl8XjSEJTbAjbW6o6a2zO9EKhEQua3IqDCS1FRn20VPRkg', 'name': 'Test Record', 'user_id': '550e8400-e29b-41d4-a716-446655440000', 'tags': ['tag1', 'tag2', 'tag3'], 'categories': ['electronics', 'computers'], 'active': True, 'created_at': '2026-01-28T09:42:46.218668', 'value': 42}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'name': {'value': 'Updated Record', 'type': 'String'}, 'data': {'type': 'String', 'value': 'aGVsbG8gd29ybGQ='}, 'price': {'type': 'Float', 'value': 99.99}, 'created_at': {'value': '2026-01-28T09:42:46.218668', 'type': 'String'}, 'tags': {'value': ['tag1', 'tag2', 'tag3'], 'type': 'Array'}, 'embedding': {'value': [0.1, 0.2, 0.3, 0.4, 0.5], 'type': 'Array'}, 'user_id': {'type': 'String', 'value': '550e8400-e29b-41d4-a716-446655440000'}, 'value': {'type': 'Integer', 'value': 100}, 'active': {'type': 'Boolean', 'value': True}, 'metadata': {'value': {'nested': {'deep': True}, 'key': 'value'}, 'type': 'Object'}, 'categories': {'value': ['electronics', 'computers'], 'type': 'Array'}, 'id': 'c_npJnq4LynwMyM7VyJkHT_ojyAtnkdZuwg7Z4HEMl8XjSEJTbAjbW6o6a2zO9EKhEQua3IqDCS1FRn20VPRkg'}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: WxYD0HVP6pAAAbeWtzplrsuBST82PZlld7m1lvGjbanUk4ix8in9YC-wm6OHCta2VIBjZwsRt5UlQsXO_3iBMw

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
Collection created with first record: "_i27H8U4j-E0i6rT7WEAru_j7BQlmamuMSSSogI0vYp7LjdLPin7Grjfe6F27L01FKHS6K3pxtk4xTPOpFYiGw"

=== List Collections ===
Total collections: 11
Sample collections: ['test_collection', 'chat_messages__ek0_testing', 'test_accounts', 'products', 'batch_users']

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
Created Alice: $1000 - ID: _pbBYbErkVSVpt65Vq-5811zs2kwnmQEIfChu77ifO1lOQM1JvNpgqj4KLyY9GNKT60hgoS3yZKsXiYMi-0amQ
Created Bob: $500 - ID: a7X_A2IPVlNSufK1i_W5R8zsPySLD4O0B8f_n_lY2Vib6e5yExYZnx7dboGlJne1UGMYTUOeh6MxNPl010D1ew

=== Example 1: Begin Transaction ===
Transaction ID: 4744b0d5-3edd-4439-b0f3-03fd54b5dbc5

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 20cd7507-09ea-47a3-b711-d1405c3308f2
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
  1. Score: 25.740, Matched: name.value, email, email.value, name
  2. Score: 12.540, Matched: name.value, name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title, bio.value, bio, title.value
  2. Score: 26.400, Matched: title, title.value, bio.value, bio
  3. Score: 26.400, Matched: bio.value, title.value, bio, title
  4. Score: 26.400, Matched: bio, title, bio.value, title.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title.value, title, bio.value, bio
  2. Score: 39.600, Matched: title, title.value, bio.value, bio
  3. Score: 39.600, Matched: title, bio, title.value, bio.value
  4. Score: 39.600, Matched: title, title.value, bio, bio.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio.value, bio

5. Search with stemming and exact match boosting:
Found 2 results (matches: run, running, runs)
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.735
  2. Score: 0.728
  3. Score: 0.725

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.364, Matched: content.value, content, title, title.value
  2. Score: 1.367, Matched: title, content, content.value, title.value
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
✓ Inserted document: 4HKlzQDkpfhsW8DnzET8GYk4k7sTc8Z8mPTJCOxHJulRS2z3YoTnfgxVqP1CYvwCRCE3oXCu1k_Stg5p39YoSQ

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: 8zId9tV6MOW86DOBuWZeaHJZ5lyXxDFxNY3vgsdf_ATZzNH6PdJFDuIW7USFhJi_HHpTZUs4MByHOSvlV49uLw

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
✓ Inserted document with TTL: iI2C-f8Y7f_jvp4_Xe1wpcxtnI-9dDBZpv12NnzHauVvx1G8NjWBU2Pcb4wnpp2ZXuP3zZzS_G7FvI69wTzgzA

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
✓ Edge cache script created: tHcAY8zUFDvsnZWZ19MbOUkCaFvNSroiPZn4OZGfqZI4P1u3JZwUyDV2q1mI0b6pghD297JHn14wIP0XB69RvA

Call 1: Cache miss (fetches from API)
Response time: 453ms
Result: {
  "records": [
    {
      "value": {
        "current": {
          "interval": 900,
          "temperature_2m": -8.1,
          "time": "2026-01-28T14:30"
        },
        "current_units": {
          "interval": "seconds",
          "temperature_2m": "\u00b0C",
          "time": "iso8601"
        },
        "elevation": 32.0,
        "generationtime_ms": 0.02181529998779297,
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
Response time: 7ms (68.6x faster!)
Result: {
  "records": [
    {
      "value": {
        "current": {
          "interval": 900,
          "temperature_2m": -8.1,
          "time": "2026-01-28T14:30"
        },
        "current_units": {
          "interval": "seconds",
          "temperature_2m": "\u00b0C",
          "time": "iso8601"
        },
        "elevation": 32.0,
        "generationtime_ms": 0.02181529998779297,
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

✅ Script saved: 8iwQZqTNwQikWHQ7P9oIhImlUu0qTQXIMgpr1pWcgPc7GahIk0mTn1H2YuNP9c-tW_-kQPVqgKJ3itZO0tP6XQ
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved: WR9P01CCBn1woJWwJM8C1gjuCPuisp4V7_LFNj7WXmya-jF7AZoq0wLdqun4hiRqhY0gPnWZMW5lrQApAr_C3Q
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: 72pu7efKa_5s3MzHw0vlOtOJ_jPBOFT8v4k2jRP8rI93h-iDgNYA_rciEdkKxdl7lIHyaeXWZ00YM5hPDmrgbA
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
   ⏱️  Duration: 1.5ms
   📊 Records: 1
   📦 Data: {
      "value": {
            "address": {
                  "city": "Gwenborough",
                  "geo": {
                        "lat": "-37.3159",
                        "lng": "81.1496"
    ...
   🚀 Cache speedup: 1.1x faster!

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

✅ Script saved: dhWCWZREBwXeq_C44e8iK2-hx2yV-10XNKcqciW-3kHupChEq683nHQZQa2ec9XmBn_W-u9Oy-yPcFjJO8Z4RQ
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

✅ Inserted order: xFhn8aODEtCgs-9w8nW7K777zK9CmZyAn-41aKwI16J6nybkUdzKnzFfp2aMs6bmLrVmCbN2qY_8eA6Vtb_TXA
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

✅ Inserted order: 9gsy67biJmZf7jo2huDFDTJrneO6j4887s8woGO4BGNYdBN5D-xeZx1--djnc_9gmquVdlT4qBCr1L8ilM_CnQ
✅ Cached order status
📊 Quick status lookup: {'value': '{"status":"processing","updated_at":"2026-01-28T14:42:48.525663+00:00"}'}

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
✓ Created SWR script: fetch_api_user_py (-cZb8YujnzDsCiVCt-n0rPGNsit3pJxIWlqp1vC5TjWntilzd4WQBSa2S67RRI-MgwdysXCexAhbCsdPNwCkYw)

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
Response time: 3ms (served from cache)
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
✓ Created native SWR script: github_user_native (h6oXeYwwG3FtMFNHCX-HS6OJK1XudERlzVtMRVc5lEmEcb_oXcphq-VEVgcqssItnwJpeFB1kQ0f9kmiet6Lig)

First call (cache miss - will fetch from GitHub API):
  Response time: 120ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 3ms
  Speedup: 38.8x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (pvGHJihX3-qt5P08ZEXe8t7BE2iTj14GeiPV7zwKLuiIkvVqp0c-Ui_ECaJpFcKcGeySgdCrkQUbk2OqIqcnvQ)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (makqtGQUL8yAhCskVAtT_0TWVJEqXFS5bcMhCuPik7DxRsxuNrD3AlmGnPNhERwzKFllnE21CGgcMbXM_-CFzQ)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (fxas1GecvGL_YbF4uvDi-VJ2zokSXrsrPMwZJitNXQdcGQ39XFFVCu6BEiubAZBF95FjWiu64jrTX9lW0X8MOQ)
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
   Vector databases offer several benefits such as high scalability, precision and flexibility. They allow efficient storage and access of data, especially spatial data. They support complex data structures like polygons and lines, making them suitable for GIS applications. They also allow for easy editing and manipulation of data.
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
   2. Vector Databases Explained (Database)
   3. Database Design Principles (Database)
   4. Natural Language Processing (AI)
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
✓ Created session: sahPJix1CZjQI-8zJFD156k6RFBRsC-UV_3q3KGgtuWemXtIe1cBz22bzC8zpB9-49pi4S9dDYXF-Xl7wTTVWg

=== Sending Chat Message ===
Message ID: PfcfvxsNP25OhP2ofUfhgb2Ga-4tNXNIh3hx14-VZCHK8TQEs2doFoK6B6L-yRCx8df0-R6S4lkNjYau5sZaLA

=== AI Response ===
We have three products available:

1. ekoDB: This is a high-performance database product with AI capabilities. The price for this product is $99.

2. ekoDB Pro: This is our enterprise edition product with advanced features. The price for this product is $299.

3. ekoDB Cloud: This is a fully managed cloud database service product. The price for this product is $499.

Execution Time: 4227ms

=== Token Usage ===
Prompt tokens: 454
Completion tokens: 84
Total tokens: 538

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: BDCqBaaTE4bJtfYTqkJHquXlLGUCLuCm0w_hKiR4FmAa8Cc1ZrU9TFqrUOYbRbxX0bo5wq1xKBBOpW9NKo2gbQ

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information provided, the product available is ekoDB. It is a high-performance database product priced at $99.

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['content', 'chat_id', 'token_usage', 'updated_at', 'context_snippets', 'created_at', 'role', 'id'])
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
✓ Created second session: ZH-TQGGOgKc2eAuwfdVl88A3nXpv_Tl1Nr5J8T1NOkKYel4TOeJPIYe6gKx9IFSkDGJC33fac-ZKKorWwnc4lg
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
✓ Created session: sPGFz1pxKMHqtytbF2S3VC_9uARlLrXyILW09b_AhA0I0x5Y8cEGwrMkL4N2OrUftkAzOXJDGpEE9toFKkvkaw

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information provided, one product available is "ekoDB". It is a high-performance database product priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: jXs8TA01wXEA9zGC5YDHKRTUgbRHwEbO5ngYBaIW9wIhElKh56Ku7XgTXRIg1UMi7RyTS6mPaHind-4kI74bfQ
  Parent: sPGFz1pxKMHqtytbF2S3VC_9uARlLrXyILW09b_AhA0I0x5Y8cEGwrMkL4N2OrUftkAzOXJDGpEE9toFKkvkaw

=== Listing Sessions ===
✓ Found 7 sessions
  Session 1: jXs8TA01wXEA9zGC5YDHKRTUgbRHwEbO5ngYBaIW9wIhElKh56Ku7XgTXRIg1UMi7RyTS6mPaHind-4kI74bfQ (Untitled)
  Session 2: sPGFz1pxKMHqtytbF2S3VC_9uARlLrXyILW09b_AhA0I0x5Y8cEGwrMkL4N2OrUftkAzOXJDGpEE9toFKkvkaw (Untitled)
  Session 3: ZH-TQGGOgKc2eAuwfdVl88A3nXpv_Tl1Nr5J8T1NOkKYel4TOeJPIYe6gKx9IFSkDGJC33fac-ZKKorWwnc4lg (Untitled)
  Session 4: sahPJix1CZjQI-8zJFD156k6RFBRsC-UV_3q3KGgtuWemXtIe1cBz22bzC8zpB9-49pi4S9dDYXF-Xl7wTTVWg (Untitled)
  Session 5: xyr5atcONAmY5b1CaBCT-SREXJ7oTLuhABQcN-smysHOm58aSEPqEZNkqsGJDwxG-Fkv9YCQvgkFZE4YWE5amw (Untitled)
  Session 6: rMNvVHF92_i7TijhVx4WnS4dvCmtzBovgjZTRYvcIN0B7CvdN7DWvV5EaABdJY8yCI4jBkpW6NQ8ZgVIrEc6Ig (Untitled)
  Session 7: 3SpDq19pY2Ui9N9-tOtcvr4WAkrxDsWBFE3DCYNzdL6SrvNVZClJpKRk7pXCOHa5JXLXKWb2vS9fd-CNweAnjw (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: jXs8TA01wXEA9zGC5YDHKRTUgbRHwEbO5ngYBaIW9wIhElKh56Ku7XgTXRIg1UMi7RyTS6mPaHind-4kI74bfQ

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Dict Creation ===
✓ Created record with plain dict: {'id': 'KXX_0cn9w2T6QxhpHXSHY9rhDpfjc6xqthI-jv5Lq69IqgydN3wqxcBXx3SJ7N68IGBp2HGAj5bph4pH-xYT8Q'}

=== Upsert Operation ===
✓ Upsert (update existing record): KXX_0cn9w2T6QxhpHXSHY9rhDpfjc6xqthI-jv5Lq69IqgydN3wqxcBXx3SJ7N68IGBp2HGAj5bph4pH-xYT8Q
✓ Inserted second record: cFEcxfIDib6Sk8sFEWkPSl-6DcHbCWesrXi9W3wBDPNX-vDoEnPRhUCQCz2Q1llWyOqiyNomR_IduoTd3QyoJQ
✓ Upsert (update second record): cFEcxfIDib6Sk8sFEWkPSl-6DcHbCWesrXi9W3wBDPNX-vDoEnPRhUCQCz2Q1llWyOqiyNomR_IduoTd3QyoJQ

=== Find One Operation ===
✓ Found user by email: {'email': {'value': 'alice.j@newdomain.com', 'type': 'String'}, 'name': {'type': 'String', 'value': 'Alice Johnson'}, 'active': {'type': 'Boolean', 'value': True}, 'age': {'type': 'Integer', 'value': 29}, 'id': 'KXX_0cn9w2T6QxhpHXSHY9rhDpfjc6xqthI-jv5Lq69IqgydN3wqxcBXx3SJ7N68IGBp2HGAj5bph4pH-xYT8Q'}
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
   Inserted with ripple: {'id': 'PcZKLB7Gw5pZzDkuorIgDIq_32bp01i1wFZgUk2vDMNEu8i5KKDKk1TXSlg0pqr5A6ZFQ7hw8BCi8bacF4y2ng'}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {'id': 'K_uXKF2JfshwBqu3qR89CyX0mZOyOpaGAe7wpdJ4UWYKloyFVQGzDXihfrlYHcuT0Pxk_zhMGghNf9hqisYtTA'}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {'name': {'type': 'String', 'value': 'Product 1'}, 'id': 'PcZKLB7Gw5pZzDkuorIgDIq_32bp01i1wFZgUk2vDMNEu8i5KKDKk1TXSlg0pqr5A6ZFQ7hw8BCi8bacF4y2ng', 'price': {'type': 'Integer', 'value': 150}}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {'id': 'PcZKLB7Gw5pZzDkuorIgDIq_32bp01i1wFZgUk2vDMNEu8i5KKDKk1TXSlg0pqr5A6ZFQ7hw8BCi8bacF4y2ng', 'name': {'type': 'String', 'value': 'Upsert Product'}, 'price': {'type': 'Integer', 'value': 500}}

✅ All bypass_ripple operations completed successfully!
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields (id, name, email only)
  Found 3 active users
  Fields returned: ['id', 'name', 'email']
  First user: {'type': 'String', 'value': 'Alice Johnson'} <{'type': 'String', 'value': 'alice@example.com'}>

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ['age', 'user_role', 'bio', 'status', 'id', 'avatar_url', 'name', 'created_at', 'email']

Example 3: Complex query with projection (active users, ages 18-65)
  Found 3 active users (ages 18-65)
    - {'type': 'String', 'value': 'Dave Brown'} (age {'value': 45, 'type': 'Integer'})
    - {'type': 'String', 'value': 'Alice Johnson'} (age {'type': 'Integer', 'value': 30})
    - {'type': 'String', 'value': 'Bob Smith'} (age {'value': 25, 'type': 'Integer'})

Example 4: Query inactive users with profile fields
  Found 1 inactive users
    - {'type': 'String', 'value': 'Carol White'}: {'value': 'Manager', 'type': 'String'}

Example 5: Compare full vs projected data
  Full query:
    - 12 fields per record
    - Fields: ['secret_token', 'password', 'user_role', 'email', 'avatar_url', 'name', 'created_at', 'api_key', 'age', 'bio', 'status', 'id']
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
OpenAI models: ['gpt-4-0613', 'gpt-4', 'gpt-3.5-turbo', 'gpt-5.2-codex', 'gpt-4o-mini-tts-2025-12-15', 'gpt-realtime-mini-2025-12-15', 'gpt-audio-mini-2025-12-15', 'chatgpt-image-latest', 'davinci-002', 'babbage-002', 'gpt-3.5-turbo-instruct', 'gpt-3.5-turbo-instruct-0914', 'dall-e-3', 'dall-e-2', 'gpt-4-1106-preview', 'gpt-3.5-turbo-1106', 'tts-1-hd', 'tts-1-1106', 'tts-1-hd-1106', 'text-embedding-3-small', 'text-embedding-3-large', 'gpt-4-0125-preview', 'gpt-4-turbo-preview', 'gpt-3.5-turbo-0125', 'gpt-4-turbo', 'gpt-4-turbo-2024-04-09', 'gpt-4o', 'gpt-4o-2024-05-13', 'gpt-4o-mini-2024-07-18', 'gpt-4o-mini', 'gpt-4o-2024-08-06', 'chatgpt-4o-latest', 'gpt-4o-audio-preview', 'gpt-4o-realtime-preview', 'omni-moderation-latest', 'omni-moderation-2024-09-26', 'gpt-4o-realtime-preview-2024-12-17', 'gpt-4o-audio-preview-2024-12-17', 'gpt-4o-mini-realtime-preview-2024-12-17', 'gpt-4o-mini-audio-preview-2024-12-17', 'o1-2024-12-17', 'o1', 'gpt-4o-mini-realtime-preview', 'gpt-4o-mini-audio-preview', 'o3-mini', 'o3-mini-2025-01-31', 'gpt-4o-2024-11-20', 'gpt-4o-search-preview-2025-03-11', 'gpt-4o-search-preview', 'gpt-4o-mini-search-preview-2025-03-11', 'gpt-4o-mini-search-preview', 'gpt-4o-transcribe', 'gpt-4o-mini-transcribe', 'o1-pro-2025-03-19', 'o1-pro', 'gpt-4o-mini-tts', 'o3-2025-04-16', 'o4-mini-2025-04-16', 'o3', 'o4-mini', 'gpt-4.1-2025-04-14', 'gpt-4.1', 'gpt-4.1-mini-2025-04-14', 'gpt-4.1-mini', 'gpt-4.1-nano-2025-04-14', 'gpt-4.1-nano', 'gpt-image-1', 'codex-mini-latest', 'gpt-4o-realtime-preview-2025-06-03', 'gpt-4o-audio-preview-2025-06-03', 'o4-mini-deep-research', 'gpt-4o-transcribe-diarize', 'o4-mini-deep-research-2025-06-26', 'gpt-5-chat-latest', 'gpt-5-2025-08-07', 'gpt-5', 'gpt-5-mini-2025-08-07', 'gpt-5-mini', 'gpt-5-nano-2025-08-07', 'gpt-5-nano', 'gpt-audio-2025-08-28', 'gpt-realtime', 'gpt-realtime-2025-08-28', 'gpt-audio', 'gpt-5-codex', 'gpt-image-1-mini', 'gpt-5-pro-2025-10-06', 'gpt-5-pro', 'gpt-audio-mini', 'gpt-audio-mini-2025-10-06', 'gpt-5-search-api', 'gpt-realtime-mini', 'gpt-realtime-mini-2025-10-06', 'sora-2', 'sora-2-pro', 'gpt-5-search-api-2025-10-14', 'gpt-5.1-chat-latest', 'gpt-5.1-2025-11-13', 'gpt-5.1', 'gpt-5.1-codex', 'gpt-5.1-codex-mini', 'gpt-5.1-codex-max', 'gpt-image-1.5', 'gpt-5.2-2025-12-11', 'gpt-5.2', 'gpt-5.2-pro-2025-12-11', 'gpt-5.2-pro', 'gpt-5.2-chat-latest', 'gpt-4o-mini-transcribe-2025-12-15', 'gpt-4o-mini-transcribe-2025-03-20', 'gpt-4o-mini-tts-2025-03-20', 'gpt-3.5-turbo-16k', 'tts-1', 'whisper-1', 'text-embedding-ada-002']
Anthropic models: ['claude-opus-4-5-20251101', 'claude-haiku-4-5-20251001', 'claude-sonnet-4-5-20250929', 'claude-opus-4-1-20250805', 'claude-opus-4-20250514', 'claude-sonnet-4-20250514', 'claude-3-7-sonnet-20250219', 'claude-3-5-haiku-20241022', 'claude-3-haiku-20240307']
Perplexity models: ['sonar', 'sonar-pro', 'sonar-deep-research', 'sonar-reasoning', 'sonar-reasoning-pro']

=== Get OpenAI Models ===
OpenAI models: ['gpt-4-0613', 'gpt-4', 'gpt-3.5-turbo', 'gpt-5.2-codex', 'gpt-4o-mini-tts-2025-12-15', 'gpt-realtime-mini-2025-12-15', 'gpt-audio-mini-2025-12-15', 'chatgpt-image-latest', 'davinci-002', 'babbage-002', 'gpt-3.5-turbo-instruct', 'gpt-3.5-turbo-instruct-0914', 'dall-e-3', 'dall-e-2', 'gpt-4-1106-preview', 'gpt-3.5-turbo-1106', 'tts-1-hd', 'tts-1-1106', 'tts-1-hd-1106', 'text-embedding-3-small', 'text-embedding-3-large', 'gpt-4-0125-preview', 'gpt-4-turbo-preview', 'gpt-3.5-turbo-0125', 'gpt-4-turbo', 'gpt-4-turbo-2024-04-09', 'gpt-4o', 'gpt-4o-2024-05-13', 'gpt-4o-mini-2024-07-18', 'gpt-4o-mini', 'gpt-4o-2024-08-06', 'chatgpt-4o-latest', 'gpt-4o-audio-preview', 'gpt-4o-realtime-preview', 'omni-moderation-latest', 'omni-moderation-2024-09-26', 'gpt-4o-realtime-preview-2024-12-17', 'gpt-4o-audio-preview-2024-12-17', 'gpt-4o-mini-realtime-preview-2024-12-17', 'gpt-4o-mini-audio-preview-2024-12-17', 'o1-2024-12-17', 'o1', 'gpt-4o-mini-realtime-preview', 'gpt-4o-mini-audio-preview', 'o3-mini', 'o3-mini-2025-01-31', 'gpt-4o-2024-11-20', 'gpt-4o-search-preview-2025-03-11', 'gpt-4o-search-preview', 'gpt-4o-mini-search-preview-2025-03-11', 'gpt-4o-mini-search-preview', 'gpt-4o-transcribe', 'gpt-4o-mini-transcribe', 'o1-pro-2025-03-19', 'o1-pro', 'gpt-4o-mini-tts', 'o3-2025-04-16', 'o4-mini-2025-04-16', 'o3', 'o4-mini', 'gpt-4.1-2025-04-14', 'gpt-4.1', 'gpt-4.1-mini-2025-04-14', 'gpt-4.1-mini', 'gpt-4.1-nano-2025-04-14', 'gpt-4.1-nano', 'gpt-image-1', 'codex-mini-latest', 'gpt-4o-realtime-preview-2025-06-03', 'gpt-4o-audio-preview-2025-06-03', 'o4-mini-deep-research', 'gpt-4o-transcribe-diarize', 'o4-mini-deep-research-2025-06-26', 'gpt-5-chat-latest', 'gpt-5-2025-08-07', 'gpt-5', 'gpt-5-mini-2025-08-07', 'gpt-5-mini', 'gpt-5-nano-2025-08-07', 'gpt-5-nano', 'gpt-audio-2025-08-28', 'gpt-realtime', 'gpt-realtime-2025-08-28', 'gpt-audio', 'gpt-5-codex', 'gpt-image-1-mini', 'gpt-5-pro-2025-10-06', 'gpt-5-pro', 'gpt-audio-mini', 'gpt-audio-mini-2025-10-06', 'gpt-5-search-api', 'gpt-realtime-mini', 'gpt-realtime-mini-2025-10-06', 'sora-2', 'sora-2-pro', 'gpt-5-search-api-2025-10-14', 'gpt-5.1-chat-latest', 'gpt-5.1-2025-11-13', 'gpt-5.1', 'gpt-5.1-codex', 'gpt-5.1-codex-mini', 'gpt-5.1-codex-max', 'gpt-image-1.5', 'gpt-5.2-2025-12-11', 'gpt-5.2', 'gpt-5.2-pro-2025-12-11', 'gpt-5.2-pro', 'gpt-5.2-chat-latest', 'gpt-4o-mini-transcribe-2025-12-15', 'gpt-4o-mini-transcribe-2025-03-20', 'gpt-4o-mini-tts-2025-03-20', 'gpt-3.5-turbo-16k', 'tts-1', 'whisper-1', 'text-embedding-ada-002']

=== Get Anthropic Models ===
Anthropic models: ['claude-opus-4-5-20251101', 'claude-haiku-4-5-20251001', 'claude-sonnet-4-5-20250929', 'claude-opus-4-1-20250805', 'claude-opus-4-20250514', 'claude-sonnet-4-20250514', 'claude-3-7-sonnet-20250219', 'claude-3-5-haiku-20241022', 'claude-3-haiku-20240307']

✓ Chat Models API example complete
✓ Client created

=== Create User Function ===
Created user function with ID: knw0sK_fnis5chJ_10EXYVLuK_qPTwhFCeVdeccEZBIuzNBSOkETxTGLGfZXmgTVASb8seD9h6gszhfrMtlAvA

=== Get User Function ===
Retrieved: get_active_users_py - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 25 user functions:
  - fetch_user: Fetch user by code
  - get_active_users_paginated: Get Active Users (Paginated)
  - swr_user: SWR pattern for user data (KV-based)
  - get_user_wrapper: Wrapper that calls fetch_user
  - get_active_users_paginated: Get Active Users (Paginated)
  - validate_user: Check if user exists
  - get_active_users: Get Active Users (Updated)
  - get_user_wrapper: Wrapper that calls fetch_user
  - fetch_user: Fetch user by code
  - get_active_users_updated: Get Active Users (Updated)
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_and_store_user: Fetch user from API and cache in KV
  - get_verified_user: Get verified and validated user
  - fetch_slim_user: Validate and slim down user
  - get_high_scoring_active_users: Get High Scoring Active Users
  - get_verified_user: Get verified and validated user
  - swr_user: SWR pattern for user data (KV-based)
  - get_active_users: Get Active Users (Updated)
  - get_active_users_py: Get Active Users
  - validate_user: Check if user exists
  - fetch_slim_user: Validate and slim down user
  - get_active_users_updated: Get Active Users (Updated)
  - get_users_by_status: Get Users By Status
  - get_users_by_status: Get Users By Status
  - cache_api_call_py: Cache External API Call

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
