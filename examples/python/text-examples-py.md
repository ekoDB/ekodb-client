make test-examples-python
🧪 [36mRunning Python examples (direct HTTP/WebSocket)...[0m
=== Simple CRUD Operations (Direct HTTP) ===

✓ Authentication successful

=== Insert Document ===
Inserted: {'id': 'vvctkCIpd-JQPPt9qcCfzmfk9Y5-8GxwJJADvEMJBOUUdHjee_tJbjKVcqbWJEE9BQIIObVv-c8ka-z2xo1FNQ'}

=== Find by ID ===
Found: {'name': {'type': 'String', 'value': 'Test Record'}, 'value': {'value': 42, 'type': 'Integer'}, 'active': {'value': True, 'type': 'Boolean'}, 'id': 'vvctkCIpd-JQPPt9qcCfzmfk9Y5-8GxwJJADvEMJBOUUdHjee_tJbjKVcqbWJEE9BQIIObVv-c8ka-z2xo1FNQ'}

=== Find with Query ===
Found documents: [{'id': 'vvctkCIpd-JQPPt9qcCfzmfk9Y5-8GxwJJADvEMJBOUUdHjee_tJbjKVcqbWJEE9BQIIObVv-c8ka-z2xo1FNQ', 'active': {'type': 'Boolean', 'value': True}, 'name': {'value': 'Test Record', 'type': 'String'}, 'value': {'value': 42, 'type': 'Integer'}}]

=== Update Document ===
Updated: {'name': {'type': 'String', 'value': 'Updated Record'}, 'active': {'value': True, 'type': 'Boolean'}, 'value': {'type': 'Integer', 'value': 100}, 'id': 'vvctkCIpd-JQPPt9qcCfzmfk9Y5-8GxwJJADvEMJBOUUdHjee_tJbjKVcqbWJEE9BQIIObVv-c8ka-z2xo1FNQ'}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: NijRrca-9YBQBeXU6j280I4rPcUwOpFnrXCbwAWzKK-LMaIattQNOpF3zXfRLzEbyzyKyf4XMcJ6kC1iBfqmHw

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
        "id": "5yKdWQgJ0eshje0NLVcqSLxJGHZEoxIMmSlenatXgi9XL08hgn5_57TrAMXSKiD0F8B_VHoOI3uK8BOuAr4dVQ",
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
        "id": "NijRrca-9YBQBeXU6j280I4rPcUwOpFnrXCbwAWzKK-LMaIattQNOpF3zXfRLzEbyzyKyf4XMcJ6kC1iBfqmHw",
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

✅ Script saved: UxEiwmXd_VYy-fIfeY893eVU0LxvqNzVnhSsPM547mFfRwF-h_ig4uHx0sEoEPco5hj9OZYhPZIr7iiPmkrA3Q
📊 Found 35 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: pb6_jWm4IZ88lb5by0W0jhMai9zs1eWWSwQ_NJLcSKqtIxax8zM3Yw6lOAy9wsPpi1WE4OaDZ3jC9r0ML5AXUA
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: 2Kk0HiiN9MWf4EmSh7DHWUsuGYg4P19JgwlfA4cLmS7rBVBAsjCjxP-KENEbNFN7nWjOEuDZy6uFRIm6x_AycQ
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {'avg_score': 60.0, 'status': 'active', 'count': 15, 'max_score': 100}
   {'count': 15, 'avg_score': 50.0, 'status': 'inactive', 'max_score': 90}

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
Collection created with first record: Pi08dMDlitOqNEz8K_6XIVRDwB7u5fiy0yl4mWTfN2MoMqySj5RUfxyA3Hibp3DbOVL47moBPHM2ieKfx6m5Sg

=== List Collections ===
Total collections: 13
Sample collections: ['batch_users', 'ws_ttl_test', 'products', 'scripts__ek0_testing', 'github_cache']

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
Created Alice: $1000 - ID: gxmtaUSKEAHKMJn8VaaKA5_SpagLajigkcRWJUy29aJXpWaYc9fxcWGD-KsEJX7xwvQ2VOLPBG-UvgbICtGzPQ
Created Bob: $500 - ID: l1qqBPLN4p5JqqYyaCpE6UJpFNlU0gxzM07Y4y2FwmBmbi_zTengJS922Y1D_kMGN69lhHVpItafQ5jrTrgFlA

=== Example 1: Begin Transaction ===
Transaction ID: 36cd4141-fcaf-45cf-b23e-e8ab75037810

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
New transaction: 61e29575-7375-4c0b-9c07-b434d8f5f0b9
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: uwwf94q0_nUy1LC3DG2BMhxUYkUOzQFwauspnmrvubE7DdDG7PcjO1rXI0-0D6df-mgL-tgPbjwkNmIWmdQM7g
Created Bob: $500 - ID: y_ouGPOSq2fLgFVaIbu7DFwUfJfe9AF7zR5qDJfn3ntp9MIYphnKhItVtEmKH9SXASdAwmX5L73JH8eOzQfa0g

=== Example 1: Begin Transaction ===
Transaction ID: 8288d17f-06ff-40e4-8acf-cef8f9f089a5

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
Created Alice: $1000 - ID: iGegGZkjsy1UHlbkJJKCI871KhQRXGhmfH4HfKxFmA8_0v5pvXUuka9bUySlSyChChkyiaKHc_G6fV9biMNMBA
Created Bob: $500 - ID: zzZaGGh37Gq8wUOEKxBma8yx_OeLwZdriGC04uqRqU8o561HY-N0xvaxJyobTvYficby9E4EcBMrDAq4VIsquQ

=== Example 1: Begin Transaction ===
Transaction ID: 33e802ed-ba14-4e28-8b70-7bf5ef1be2a9

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
New transaction: 315b8a1e-4ab8-437c-a82f-0f98d1b43ce7
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: HU_KI9HL-vN_iPYFf_CmU6h31xyKAqTHnz-yvlV_eC1pWJZxmKAXvgrBAi62bO8tfggOWQLxVeAKJ_B2RhV9UQ
Created Bob: $500 - ID: 6Bn7VtruwYJUrTv-RLxsIIH6G1YwxSNdOFwRPohKTpncC_Ylc-DHqW0tUOPnkazDoQyXD34V0VK6sNd4RS02gQ

=== Example 1: Begin Transaction ===
Transaction ID: dde6d0bf-1dd3-441a-b239-560a99750b53

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
Created Alice: $1000 - ID: ei7XuiyllwmCFGCL7tsLRJcGtvonyfiledr14w5-HBHG1VIjM5HJoOrIZ2Na9EwPYAf1HuCUYKUU10y8RoAcag
Created Bob: $500 - ID: twD5BxMX2IvrssM69hYrHVfwT5WXyVUKRwGDYQSveVLE7HEdHd8cV78hK2hUwHF9GVqri9c30Fomc8QPub7jdg

=== Example 1: Begin Transaction ===
Transaction ID: 317d5be7-e0bd-4240-8f02-9716fecab985

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
New transaction: 7a452e88-1eea-4ea2-b15f-c4c8dba3989e
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: dK18BtxlEd2mq3vCA4y2GAr2oP6Rd63CVQJbljzJuCl6wt74W4xqWorVsFRCnB7YHtjYDy8om4lBNcJBDvlhBw
Created Bob: $500 - ID: NOq-ke2SLSjYRcMLJCSuzHr_2ms2pQEhNjwJG6rpcLsaAtgoirQNyAQnl9gY1J5dSOf6zSkjSwfQ5HxWMedm5g

=== Example 1: Begin Transaction ===
Transaction ID: 074266b7-b900-47e1-ae96-1c1571deb557

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
   ✅ Script saved: 7wUzRP5EFAO4qbzUrjWKqpgju45S3EN_D0bgzlb5gphYqzPmn3JdkS3FdT3hL9Y2nQD-AhRmpLJcpkrn2Rjx_g

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: DiS29mQgPVpccZlFYlq5zbDIHLmnO_eh_DtRmSXNSkidPAL3ZQUw1VImmZHGtM9H4UEmaavvn_7DUpdysLooGQ
   📋 Name: {'value': 'Alice Smith', 'type': 'String'}
   📋 Email: {'type': 'String', 'value': 'alice@example.com'}
   📋 Status: {'type': 'String', 'value': 'pending'}
   📋 Credits: {'value': 0, 'type': 'Integer'}

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: Cr2cflX01rrt4EZFK_EuF0ukwO8adg9pfR9n6gWvrFjT47xenlMUWLFrsk7A2MEdaN6qf_xYxeZkGvq1InMaaw

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
   ✅ Script saved: s_AwOdEyEL1dxRanL2Z6uncVWg0nuEkvIBL6H-ioH8gARynkJGr8t5k-WLgXI0T5ucB0A3lZv-iLS0ml3akfEw

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
   ✅ Script saved: 2WgctZ9wF1_h8Luj7tOhgvVE9WrnjpmjSk2H4bEQi6S88XDECz9zOwYfB5FgJ6zQGA40NXAAfKQyijWZSzeCdA

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: 7wUzRP5EFAO4qbzUrjWK...
   ✅ Deleted script: Cr2cflX01rrt4EZFK_Eu...
   ✅ Deleted script: s_AwOdEyEL1dxRanL2Z6...
   ✅ Deleted script: 2WgctZ9wF1_h8Luj7tOh...
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
  Output: Document ID = pOAJg1ng7NqI1O88_oOFrDwcTRCXkOyC-kX3vNmy_A3t1uchguSHb4C_mnlsjHBNo9HMkzaar3nPiMhOgNurFg
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: find_by_id(pOAJg1ng7NqI1O88_oOFrDwcTRCXkOyC-kX3vNmy_A3t1uchguSHb4C_mnlsjHBNo9HMkzaar3nPiMhOgNurFg)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: find_by_id(pOAJg1ng7NqI1O88_oOFrDwcTRCXkOyC-kX3vNmy_A3t1uchguSHb4C_mnlsjHBNo9HMkzaar3nPiMhOgNurFg)
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
  Output: Document ID = x-XDiqhar8f0jaJSRDJGuz29_xFMs-ZdFtrFOABCzrKYKSAGCj0NyM4Nj3b8lAU3v2v7k0y16LwjrUCvM_LkUg
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: find_by_id(x-XDiqhar8f0jaJSRDJGuz29_xFMs-ZdFtrFOABCzrKYKSAGCj0NyM4Nj3b8lAU3v2v7k0y16LwjrUCvM_LkUg)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: find_by_id(x-XDiqhar8f0jaJSRDJGuz29_xFMs-ZdFtrFOABCzrKYKSAGCj0NyM4Nj3b8lAU3v2v7k0y16LwjrUCvM_LkUg)
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
    Finished `release` profile [optimized] target(s) in 0.13s
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
Inserted: {'id': 'nDJQBL-DCMhpYvoxnDQz4E4mNoa6uGiTYqfp0AtbwFyqkLxMRu7_RlBDCVdDhv39X4AFlNLG1m5NHRXmjUEY5w'}

=== Find by ID ===
Found: {'embedding': {'value': [0.1, 0.2, 0.3, 0.4, 0.5], 'type': 'Array'}, 'user_id': {'type': 'String', 'value': '550e8400-e29b-41d4-a716-446655440000'}, 'id': 'nDJQBL-DCMhpYvoxnDQz4E4mNoa6uGiTYqfp0AtbwFyqkLxMRu7_RlBDCVdDhv39X4AFlNLG1m5NHRXmjUEY5w', 'categories': {'value': ['electronics', 'computers'], 'type': 'Array'}, 'active': {'value': True, 'type': 'Boolean'}, 'created_at': {'value': '2026-01-06T11:10:31.001245', 'type': 'String'}, 'value': {'value': 42, 'type': 'Integer'}, 'price': {'value': 99.99, 'type': 'Float'}, 'tags': {'type': 'Array', 'value': ['tag1', 'tag2', 'tag3']}, 'metadata': {'type': 'Object', 'value': {'key': 'value', 'nested': {'deep': True}}}, 'data': {'type': 'String', 'value': 'aGVsbG8gd29ybGQ='}, 'name': {'type': 'String', 'value': 'Test Record'}}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): True
  price (Decimal): 99.99
  created_at (DateTime): 2026-01-06 11:10:31.001245
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): ['tag1', 'tag2', 'tag3']
  metadata (Object): {'key': 'value', 'nested': {'deep': True}}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): ['electronics', 'computers']
  data (Bytes): 11 bytes
Plain record: {'embedding': [0.1, 0.2, 0.3, 0.4, 0.5], 'user_id': '550e8400-e29b-41d4-a716-446655440000', 'id': 'nDJQBL-DCMhpYvoxnDQz4E4mNoa6uGiTYqfp0AtbwFyqkLxMRu7_RlBDCVdDhv39X4AFlNLG1m5NHRXmjUEY5w', 'categories': ['electronics', 'computers'], 'active': True, 'created_at': '2026-01-06T11:10:31.001245', 'value': 42, 'price': 99.99, 'tags': ['tag1', 'tag2', 'tag3'], 'metadata': {'key': 'value', 'nested': {'deep': True}}, 'data': 'aGVsbG8gd29ybGQ=', 'name': 'Test Record'}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'data': {'type': 'String', 'value': 'aGVsbG8gd29ybGQ='}, 'id': 'nDJQBL-DCMhpYvoxnDQz4E4mNoa6uGiTYqfp0AtbwFyqkLxMRu7_RlBDCVdDhv39X4AFlNLG1m5NHRXmjUEY5w', 'categories': {'value': ['electronics', 'computers'], 'type': 'Array'}, 'metadata': {'value': {'key': 'value', 'nested': {'deep': True}}, 'type': 'Object'}, 'user_id': {'type': 'String', 'value': '550e8400-e29b-41d4-a716-446655440000'}, 'name': {'value': 'Updated Record', 'type': 'String'}, 'active': {'value': True, 'type': 'Boolean'}, 'value': {'value': 100, 'type': 'Integer'}, 'tags': {'type': 'Array', 'value': ['tag1', 'tag2', 'tag3']}, 'created_at': {'type': 'String', 'value': '2026-01-06T11:10:31.001245'}, 'price': {'value': 99.99, 'type': 'Float'}, 'embedding': {'type': 'Array', 'value': [0.1, 0.2, 0.3, 0.4, 0.5]}}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: J-af5-hidEzKWmfZHMHNq5D2azRl6aROuiiNMMH7FobjNJuQsAAiKdrv2hLvij97rzzaFJeOTFI2SBMxKcw4vw

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
Collection created with first record: "M3H281WCEcaUcfNcX3WOwd8O46BhSpyLJj7bUFjxLHYAXK0zi6tJkEgDQn74-UpGh-C4zYdidVhdikBy-8YdFg"

=== List Collections ===
Total collections: 12
Sample collections: ['batch_users', 'ws_ttl_test', 'client_collection_management_python', 'products', 'scripts__ek0_testing']

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
Created Alice: $1000 - ID: Qa543jqYtUcPrKgY6aetfcYLoITy9opHiSSeDkVh1ahOR5L-oDuoR2P_EiVC6jBZbuhy6R9y2ORoSrKpxgLB3g
Created Bob: $500 - ID: cgethdgQFuRI5UYcLYozJJ053gCfSkH6-kX8yveBlcoHfu6AHv6J0DneunA1olUA_leyx9iQj5hwnrgQciVVfA

=== Example 1: Begin Transaction ===
Transaction ID: fdb2be7a-9e3c-4208-a8d2-e057152cbeb7

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 9d689106-622f-4d5e-bc31-0d439aa96ea7
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
  1. Score: 25.740, Matched: name, email, email.value, name.value
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: bio.value, bio, title, title.value
  2. Score: 26.400, Matched: title, title.value, bio, bio.value
  3. Score: 26.400, Matched: bio, title, bio.value, title.value
  4. Score: 26.400, Matched: title, title.value, bio.value, bio

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title.value, bio.value, title, bio
  2. Score: 39.600, Matched: title.value, bio.value, title, bio
  3. Score: 39.600, Matched: bio.value, title.value, title, bio
  4. Score: 39.600, Matched: title.value, title, bio, bio.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 2 results (matches: run, running, runs)
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio.value, bio

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.770
  2. Score: 0.760
  3. Score: 0.758

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.380, Matched: title, content.value, content, title.value
  2. Score: 1.379, Matched: content, content.value, title, title.value
  3. Score: 0.385, Matched: 

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
✓ Inserted document: 8k4nGTsuebOPMHd_hihFI8Bwcevwo7TugwgkfwGGqQLxNWSExui7bczQFLXOMU8zIRKX9xCUh0_irSweepLHOQ

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: DrDNtvaEZjAXS-mNd9blGJ4jOWLjuYpb06xXoOi5FfpZm6_oiLe5l4X9Au3-Ei3p24XQ1dOTN0MJbl4YzPc3cA

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
✓ Inserted document with TTL: UC0GbSSdHensWgotZCmzTW9BDYoAw6x-9zbdH-qETOIP3IJiZCm_FIMK4M3dq8mBR6JfJZjqm-GkrFnpUIjz8A

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
✓ Edge cache script created: KVIDZ59JkiFDLFPil9MP3omAY5VtZZGQl0d01HdMo20OPQ0Q8IN4Ls14Wf2kQfugA9DnQCEIqkvjEXMb3ADuow

Call 1: Cache miss (fetches from API)
Response time: 573ms
Result: {
  "records": [
    {
      "cached_at": "2026-01-06T11:10:32.406726",
      "data": {
        "type": "Object",
        "value": {
          "current": {
            "interval": 900,
            "temperature_2m": 5.1,
            "time": "2026-01-06T16:00"
          },
          "current_units": {
            "interval": "seconds",
            "temperature_2m": "\u00b0C",
            "time": "iso8601"
          },
          "elevation": 32.0,
          "generationtime_ms": 0.026106834411621097,
          "latitude": 40.710335,
          "longitude": -73.99309,
          "timezone": "GMT",
          "timezone_abbreviation": "GMT",
          "utc_offset_seconds": 0
        }
      },
      "id": "WiHktZ0s7L6qMPppBt4iNCvUR_e07RU72wbX82uPtlXOCBLq1XNgsNceBMw9dSv3HRAAmRPgHk1tFJCaKZmNeA"
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
Response time: 539ms (1.1x faster!)
Result: {
  "records": [
    {
      "cached_at": "2026-01-06T11:10:32.406726",
      "data": {
        "type": "Object",
        "value": {
          "current": {
            "interval": 900,
            "temperature_2m": 5.1,
            "time": "2026-01-06T16:00"
          },
          "current_units": {
            "interval": "seconds",
            "temperature_2m": "\u00b0C",
            "time": "iso8601"
          },
          "elevation": 32.0,
          "generationtime_ms": 0.01728534698486328,
          "latitude": 40.710335,
          "longitude": -73.99309,
          "timezone": "GMT",
          "timezone_abbreviation": "GMT",
          "utc_offset_seconds": 0
        }
      },
      "id": "bB9hNj3KGYOrnKAW0oS7nK9DhrrW-3YFAi5ymvRIj6aOh0peFibNI0kDjhRK7Xh_u6xw2_nke4IO8Ue5y6sLyw"
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

✅ Script saved: KB3HSBfoL8VM3BxFKsWKyMwq3aJjaTa3aMhMqiA4Cmfelvcqk_-JGC4AgKHwfLfhA227iJKU04ASSDGs7dyZIg
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved: MtRX8iuWpw8IqBLBlHVHIfQI48T1i2enrHdfGCjeVI06jbutoBX1kjnotk36arPYnvB9Ouaa93wtJ786ju0Ikw
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: H6sNBrNrnTh8zCenOnqmOjC73UKQMUp659-4bcv8my-DNnPhej34zGcy5uBFrlc6ComMRSUa2zjyQnxqifZW7A
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
   ⏱️  Duration: 135.5ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 81.0ms
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

✅ Script saved: ULs8m6JRUJAo76Zyzi8TdlVKkz5D1gVfk9T_m8hJIlfKBaljIcOySiapmM2-cgaWZ49Bv7Y0fT4wVUGsxtaUlg
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

✅ Inserted order: 6IiKKt1GUcAFozYCEqWHWvUi4FwIz-q5MY6CY1TucNrJLBwdYIcxkI_o81-1PdMzZ0B06J0WsBTuoI1qsMkzaA
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

✅ Inserted order: KCW8I6nQIp0VsR4V4HdKL8q24gfGx7T8Fc-5lmoEcOg6sckDswtYidUJoyBTw24k4xo625mJ2vXSjSsnYYByeQ
✅ Cached order status
📊 Quick status lookup: {'value': '{"status":"processing","updated_at":"2026-01-06T16:10:34.247979+00:00"}'}

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
✓ Created SWR script: fetch_github_user_py (B2odpP768id-fn1uQ6q342HE0OrboNc1HOrwJ_YqguoEqF0b3uHZMvIwdh4hB3gOOCns7eIXM8O4N7StGj7riw)

Step 2: First call - Cache miss, fetches from GitHub API
Result: {
  "records": [
    {
      "cached_at": "2026-01-06T11:10:34.415692",
      "data": {
        "type": "Object",
        "value": {
          "avatar_url": "https://avatars.githubusercontent.com/u/1024025?v=4",
          "bio": null,
          "blog": "",
          "company": "Linux Foundation",
          "created_at": "2011-09-03T15:26:22Z",
          "email": null,
          "events_url": "https://api.github.com/users/torvalds/events{/privacy}",
          "followers": 270299,
          "followers_url": "https://api.github.com/users/torvalds/followers",
          "following": 0,
          "following_url": "https://api.github.com/users/torvalds/following{/other_user}",
          "gists_url": "https://api.github.com/users/torvalds/gists{/gist_id}",
          "gravatar_id": "",
          "hireable": null,
          "html_url": "https://github.com/torvalds",
          "id": 1024025,
          "location": "Portland, OR",
          "login": "torvalds",
          "name": "Linus Torvalds",
          "node_id": "MDQ6VXNlcjEwMjQwMjU=",
          "organizations_url": "https://api.github.com/users/torvalds/orgs",
          "public_gists": 1,
          "public_repos": 9,
          "received_events_url": "https://api.github.com/users/torvalds/received_events",
          "repos_url": "https://api.github.com/users/torvalds/repos",
          "site_admin": false,
          "starred_url": "https://api.github.com/users/torvalds/starred{/owner}{/repo}",
          "subscriptions_url": "https://api.github.com/users/torvalds/subscriptions",
          "twitter_username": null,
          "type": "User",
          "updated_at": "2025-11-24T04:16:14Z",
          "url": "https://api.github.com/users/torvalds",
          "user_view_type": "public"
        }
      },
      "id": "8kYielVMNhuowHFsDrNPQMIli5gl4sRTwxE4i3KYhctinSKGL4nh1zom2mVVuk5o4dLbCQgY6032BZs1NOyEpw"
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
Response time: 84ms (served from cache)
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
   Vector databases have several benefits including:

1. Precision: They provide more accurate and detailed data representation.
2. Scalability: The quality of vector data is not affected by scaling or zooming.
3. Flexibility: It's easier to modify and manipulate data.
4. Less Space: They typically require less storage space compared to raster databases.
5. Topology: Vector databases have the ability to maintain topological information like adjacency, connectivity and relationships between features.
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
   4. Introduction to Machine Learning (AI)
   5. Vector Databases Explained (Database)
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
✓ Created session: ld9rVKp4aH4JOkUnFVYGKj4j8r2wHCUzMhFDp6pPQ_uKiRHIPwF3xlBjj_jGZnNHXzBvJCFoL7eFsx1FGxpNYw

=== Sending Chat Message ===
Message ID: uZgJ0NvEWH-hJHdl3KNIB70EMLykYRt2nhr3gwHGLlLliH0wX_20Qqlp5jXzz0Q11MHTm7FJkb6pncDeQPsbgg

=== AI Response ===
There are three products available:

1. ekoDB Cloud: This is a fully managed cloud database service product. The price for this product is $499.

2. ekoDB Pro: This is an enterprise edition product with advanced features. The price for this product is $299.

3. ekoDB: This is a high-performance database product with AI capabilities. The price for this product is $99.

Execution Time: 4377ms

=== Token Usage ===
Prompt tokens: 450
Completion tokens: 84
Total tokens: 534

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 1gzm5zfXlhBQ621uv3rfxJkKBEv77lqME2WaMW3ZVk8B-zc5MMMhFLfMeiarWBIANE1U9qucF-nET8F9eZxKCg

=== Sending Initial Message ===
✓ Message sent
  Response: Currently, we have the "ekoDB" available. It's a high-performance database product priced at $99.

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['id', 'role', 'updated_at', 'created_at', 'context_snippets', 'token_usage', 'content', 'chat_id'])
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
✓ Created second session: Vwv_dS-QtANBp_pTXuP31-g0Z56KtYJkKqzGlcNeqgBv68RCtjOiUv2gnGQKhO03ZeTUkAZZVX49FONaJsaS-w
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
✓ Created session: UPpvdwRwbg5gporVh95VhM_j6mgk-bEi5bO1tWemG0l-v_l3EIujQD_Te-tr_i5kQcURMoyMQDaUMQSMrH_2aw

=== Sending Messages ===
✓ Message 1 sent
  Response: The product available is called "ekoDB". It is a high-performance database product. The price for this product is $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: CmkXUGl8N39PNfVRr_wfp_s9gXhLtpKaLd7K8JE8JEizguq4G_Ih6TWm_fI7mh0etUGyvOC3ZG1uYG-OnM22RQ
  Parent: UPpvdwRwbg5gporVh95VhM_j6mgk-bEi5bO1tWemG0l-v_l3EIujQD_Te-tr_i5kQcURMoyMQDaUMQSMrH_2aw

=== Listing Sessions ===
✓ Found 7 sessions
  Session 1: CmkXUGl8N39PNfVRr_wfp_s9gXhLtpKaLd7K8JE8JEizguq4G_Ih6TWm_fI7mh0etUGyvOC3ZG1uYG-OnM22RQ (Untitled)
  Session 2: UPpvdwRwbg5gporVh95VhM_j6mgk-bEi5bO1tWemG0l-v_l3EIujQD_Te-tr_i5kQcURMoyMQDaUMQSMrH_2aw (Untitled)
  Session 3: Vwv_dS-QtANBp_pTXuP31-g0Z56KtYJkKqzGlcNeqgBv68RCtjOiUv2gnGQKhO03ZeTUkAZZVX49FONaJsaS-w (Untitled)
  Session 4: ld9rVKp4aH4JOkUnFVYGKj4j8r2wHCUzMhFDp6pPQ_uKiRHIPwF3xlBjj_jGZnNHXzBvJCFoL7eFsx1FGxpNYw (Untitled)
  Session 5: jSz7-Rn7Gm2dKTCBQP-Na7ElcHYIhA-KdHJAC4xfo_HHUe9692grvVeO8tsjgtikoNoDpkcoo-Y6Pr3IWt5sxQ (Untitled)
  Session 6: q06liewa9ga2qYakyZ96N2GGsnUAwOqtrCRMLPHiWGhF1EO3wrHr_bbAT4z9mta5NnhSvf-MPBdyCFSYGNIzqw (Untitled)
  Session 7: FbI9y0Ydck_SMHJ_B-YqCwwxEsLnxwrSKcGyg2Kr74MXEWlYYCdWZ_1x1xSwLF5OunKuwVUWMmSzsLvZo_3JOw (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: CmkXUGl8N39PNfVRr_wfp_s9gXhLtpKaLd7K8JE8JEizguq4G_Ih6TWm_fI7mh0etUGyvOC3ZG1uYG-OnM22RQ

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Dict Creation ===
✓ Created record with plain dict: {'id': 'Xws4uumYwVnn9YKf6874w_eAs3owy-V133AuFim4SKoZJeaJCMc6PXv8xmrQ8xXO53WP1Jplj41hzMVnvZKX1w'}

=== Upsert Operation ===
✓ Upsert (update existing record): Xws4uumYwVnn9YKf6874w_eAs3owy-V133AuFim4SKoZJeaJCMc6PXv8xmrQ8xXO53WP1Jplj41hzMVnvZKX1w
✓ Inserted second record: 24oBPzZQiB-LQQuTdTxpXyTvHEwjSBfQU4yb_cMt7aMbGbXE6GFbmUKaTJAP_Fw2uTa5rioSu1gMxb4GGa8qXA
✓ Upsert (update second record): 24oBPzZQiB-LQQuTdTxpXyTvHEwjSBfQU4yb_cMt7aMbGbXE6GFbmUKaTJAP_Fw2uTa5rioSu1gMxb4GGa8qXA

=== Find One Operation ===
✓ Found user by email: {'name': {'type': 'String', 'value': 'Alice Johnson'}, 'active': {'type': 'Boolean', 'value': True}, 'email': {'type': 'String', 'value': 'alice.j@newdomain.com'}, 'id': 'Xws4uumYwVnn9YKf6874w_eAs3owy-V133AuFim4SKoZJeaJCMc6PXv8xmrQ8xXO53WP1Jplj41hzMVnvZKX1w', 'age': {'type': 'Integer', 'value': 29}}
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
   Inserted with ripple: {'id': 'Iq4y-QTSX0nfbyMV9UvBQ9KhZbeaTVi9w8o8LH_bDrLtfBlhTVVOuRM6ZRLZ9FmjS0VnVlVFXal1CHs84JArJw'}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {'id': 'WI5ZRbnGyvEuQL-PRrOnXBFTeEvO-jY01CRoZV8yPEPMgpodRYF6Wx0Tz5wRgDaO4Mw4uFzJYpj_2X7xYeJU-g'}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {'name': {'type': 'String', 'value': 'Product 1'}, 'id': 'Iq4y-QTSX0nfbyMV9UvBQ9KhZbeaTVi9w8o8LH_bDrLtfBlhTVVOuRM6ZRLZ9FmjS0VnVlVFXal1CHs84JArJw', 'price': {'type': 'Integer', 'value': 150}}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {'name': {'value': 'Upsert Product', 'type': 'String'}, 'price': {'value': 500, 'type': 'Integer'}, 'id': 'Iq4y-QTSX0nfbyMV9UvBQ9KhZbeaTVi9w8o8LH_bDrLtfBlhTVVOuRM6ZRLZ9FmjS0VnVlVFXal1CHs84JArJw'}

✅ All bypass_ripple operations completed successfully!
✅ [32mPython client examples complete![0m
