make test-examples-python
🧪 [36mRunning Python examples (direct HTTP/WebSocket)...[0m
✓ Authentication successful

=== Insert Document ===
Inserted: {'id': 'Z5RB5lTCfs5daBDLiJdAwpjZMql8f-w1aAUqkKw2cgtP1akKV-UOp_jxPRIShOGMwMpMl8mNXrfR33pXSOXkIQ'}

=== Find by ID ===
Found: {'active': {'type': 'Boolean', 'value': True}, 'value': {'value': 42, 'type': 'Integer'}, 'id': 'Z5RB5lTCfs5daBDLiJdAwpjZMql8f-w1aAUqkKw2cgtP1akKV-UOp_jxPRIShOGMwMpMl8mNXrfR33pXSOXkIQ', 'name': {'value': 'Test Record', 'type': 'String'}}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'id': 'Z5RB5lTCfs5daBDLiJdAwpjZMql8f-w1aAUqkKw2cgtP1akKV-UOp_jxPRIShOGMwMpMl8mNXrfR33pXSOXkIQ', 'active': {'value': True, 'type': 'Boolean'}, 'name': {'value': 'Updated Record', 'type': 'String'}, 'value': {'type': 'Integer', 'value': 100}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: DWtfhXvA14K_gV543gEE-ww8d0AntNrR6aknFBPOXwp_d3kvxo0T58fFXFS68odu9b2TN8gPAqPB1d44FLyLyg

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
        "id": "DWtfhXvA14K_gV543gEE-ww8d0AntNrR6aknFBPOXwp_d3kvxo0T58fFXFS68odu9b2TN8gPAqPB1d44FLyLyg",
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
        "id": "j7cabfp3zf1fqbuWvsaWueFgmiV_kbeJ8-iQm7irQ6aYmr3XJKzZBVeOS5sytzHo_wrxas5BYrKHWjfI6lwWXQ",
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
✓ Authentication successful

=== Batch Insert ===
✓ Batch inserted 5 records
✓ Verified: Found 10 total records in collection

=== Creating test records for update/delete ===
Created 3 test records

=== Batch Update ===
✓ Batch updated 3 records
✓ Verified: Record updated with status="{'value': 'active', 'type': 'String'}"

=== Batch Delete ===
✓ Batch deleted 3 records
✓ Verified: Records successfully deleted (not found)

✓ All batch operations completed successfully
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
✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: smTn3nJ8Wv3FKyEn0c1W3rg3wdWgu5pCAqjmiGmtDvBrD1hh5fvpMh3mc5uawHI2D_rHC8s3Ei6aLWjPNkrSLQ

=== List Collections ===
Total collections: 11
Sample collections: ['chat_configurations__ek0_testing', 'test_accounts', 'test_collection', 'chat_messages__ek0_testing', 'users']

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: False

✓ All collection management operations completed successfully
✓ Authentication successful

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: nSI5yOcHux82_QbAp0gw9OcWcqOJnx_bpvdeDUp1ayeajdaSHK8JzVQ7wqU6uhcwRqsQk7K6eaJQDpDqb1c8SA

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: created

=== Query Documents ===
✓ Found 2 documents with TTL

=== Update Document ===
✓ Updated document

=== Delete Document ===
✓ Skipped delete (no ID returned)

✓ All document TTL operations completed successfully
✓ Authentication successful

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: KMP8u55Eidw7c1mnW4Uq7GgN16Jgjfvq6H_AiSLJey6l_SfUb5JBy7v09NkYvmatvAJiR-0x9RFEojugAkJtMg

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 2 record(s) via WebSocket

✓ WebSocket TTL example completed successfully
🚀 ekoDB Scripts Example (Python/HTTP)

✓ Authentication successful

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: LmnufFjIL0gBt7e4cXj9A_HZrEXVDk4xVH9qsOoMV46tPCZZE5YYNZDinSNvvs479_8TxGJVeMC7zg6WB1uj7g
📊 Found 30 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: 8EHSBJR9uQCSY6JZ7LZaJbRBR-34bz0BhvFeh24FvyaKTGGWe9QhjnVi_GJAnp3qp7k829NzId_5rwuZSNRhSA
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: 3uma-Y7E9ZrYzumiquSod7KqxHDI96zcDDfnThfXF25xeElN4_mPgXbcpiWJ0P34mGcWyz6umKbEQcPTI4gGew
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {'status': 'inactive', 'avg_score': 50.0, 'max_score': 90, 'count': 15}
   {'avg_score': 60.0, 'max_score': 100, 'status': 'active', 'count': 15}

📝 Example 4: Function Management

📋 Total scripts: 8
🔍 Retrieved script: Get Active Users
✏️  Script updated
🗑️  Script deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

✅ All examples completed!
✓ Authentication successful

📋 Getting original configuration...
   Original durable_operations: False


============================================================
🔥 TEST 1: Original Config (durable=False)
============================================================

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: k6UhufSHqFX7xaq2Wv_g7ea4mMaCENG6hL67Kfb-BrmUU6II_Pggy7hO6BX36LsBvRuwwoaWvpd2Q7ZbQceT7A
Created Bob: $500 - ID: p2Nnw1Dgx2OqMPAtU80j3G59MkXGZyp1wTZMWCkWg0-9HKSL8xij2rNxQoUjPKwb3nwIIIpPqeMwSIqqmfvaLw

=== Example 1: Begin Transaction ===
Transaction ID: 6a7647b6-e660-46cf-bce5-190f3573670a

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
New transaction: 8d728fd7-8b77-4b63-93ba-e0c8cce0e3a2
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: k2kczb7PGCwRGZAlwQcknQaRdsCa4H1L0vU1SHqGhfULcGQ-owsDRsVzKGcehU3dD8SffaY9hdSHhxMUXcWgrw
Created Bob: $500 - ID: MPV9uOwyTA9pMUUpc29usrZdWNcrXVusHoqcWt1WKGkZs_eA9FRcsyseHoLWev4QMTCgNrlEkisMDWAhaVhmPA

=== Example 1: Begin Transaction ===
Transaction ID: 7c341b35-e32a-4f5d-a6af-a830d2f76cb4

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
Created Alice: $1000 - ID: aDJfP5gZYly71Iut0IraYT1dazSfaol_KMOQ_egcF0nKtmM_X7r65oOYpglU_k06et5l3ZSgjnaapoMwopuCLQ
Created Bob: $500 - ID: b6M1aD6QzNx9GeqBrXSIOHdNrptSXbih0ta5iPoYJcNG73aM2PJy5dIK1JnWPtwcspCgG_vei2rVubQB2OPXzg

=== Example 1: Begin Transaction ===
Transaction ID: 1784e9b3-acaf-4867-8ae0-3e15084fb72a

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
New transaction: 223d27cd-9f81-42bd-8428-2a55fd6a0aa1
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: hQd9cQczUBDtFG4SgJSEWlqPftCeRD7Rokj1ThAAW06TL8vfeGHx4u7qBiflWggtrdmh2NdHCNH06zFTYvEHFw
Created Bob: $500 - ID: yz9H-B4iITHYiWJC7pNfD5z1watjXM4W03asfmZnr0JE4pHRezy-cEKgCOSxHi2Rl1G_SGxKKRtEtFk7a7dQtg

=== Example 1: Begin Transaction ===
Transaction ID: 1cfe2606-f628-4c94-94dc-7f9f0c700416

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
Created Alice: $1000 - ID: 3RZKYEkvlSQna0p5NPx25LF96gDah95PK9H9WiHZVVnM745eK1pM1H0BeeF-46IixehyjjRxBsNScMpkzuQ9yg
Created Bob: $500 - ID: b3uLIEnuKIFzBJAzbOCKITfGCbuy5BhLURDBg2L-UXF9V3OJxX16MWKifYOAii6_Spq5rQ8_rZLCBOMWf3sZlw

=== Example 1: Begin Transaction ===
Transaction ID: f59718b0-d668-493a-abd1-33ed93daf9f2

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
New transaction: fafb3cbf-fae9-4bd9-b760-960978979537
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: RvDqonMNhnvR-HAu2QB8Y3XNzoUl_33h6NbjLWpMufIe7a79GAK5VeWLwmD-nO0PHrPUOvO8PvIGIfmnsoHvug
Created Bob: $500 - ID: Ag0n5iMPO4gNfmXx5PUkzf0Uw0Y5hROGOgkjVmcxvrNMM4X-j_LBh1P8m-LusUMmojAPLKHzTX4AZqSSFAMCuA

=== Example 1: Begin Transaction ===
Transaction ID: 10e4e6af-7d4b-434e-a8f8-38e95d855b36

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
   ✅ Script saved: UnFydaNC3g_MHmw0uiwnDGu6D4FISjmQQuDYANDl_XKSW0eKT3tmgQWtncSEURfNjYr8bDiQX_CmjG-P7IBvEQ

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: butEN3CyRpkWTm8hft5zCvml_KT2hnQxBraMkgNuT0WnalYSxQnjFdgb1d7AcLUnqTx4DmKl9DVg-SmL8Hdq_Q
   📋 Name: {'value': 'Alice Smith', 'type': 'String'}
   📋 Email: {'type': 'String', 'value': 'alice@example.com'}
   📋 Status: {'type': 'String', 'value': 'pending'}
   📋 Credits: {'type': 'Integer', 'value': 0}

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: evkMRvmClHbIr63y-TF0aQDw2XabTxDNR6Z9QwD_XsNCk01dMWbznM624N-X_WnXP2lHLyih1wcNWuBPw56wzg

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
   ✅ Script saved: U-WPwNEnMCuOTVXEa3TiKyB0vSlHFQ4RgR0Vq-kVdHph1eVNolIZHG3dv-X6i09CqMxCgliyGySUkUKSKOH_Jg

2️⃣ Calling Script (Query + Update Credits + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {'type': 'Integer', 'value': 0}
   📋 Status: {'type': 'String', 'value': 'active'}
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}

============================================================
📝 Script 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: cqjoKjSnsNoO_6H39N7utS7rwO2KvBHxv7Lp3fqvXUO5EEFDzRY9qBLbT41tZheinTdpEHQsibpLItktsvX5XQ

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: UnFydaNC3g_MHmw0uiwn...
   ✅ Deleted script: evkMRvmClHbIr63y-TF0...
   ✅ Deleted script: U-WPwNEnMCuOTVXEa3Ti...
   ✅ Deleted script: cqjoKjSnsNoO_6H39N7u...
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
=== Running batch_operations.py ===[0m
[32m✓ batch_operations.py completed successfully[0m
[34m
=== Running kv_operations.py ===[0m
[32m✓ kv_operations.py completed successfully[0m
[34m
=== Running collection_management.py ===[0m
[32m✓ collection_management.py completed successfully[0m
[34m
=== Running document_ttl.py ===[0m
[32m✓ document_ttl.py completed successfully[0m
[34m
=== Running websocket_ttl.py ===[0m
[32m✓ websocket_ttl.py completed successfully[0m
[34m
=== Running http_functions.py ===[0m
[32m✓ http_functions.py completed successfully[0m
[34m
=== Running transactions.py ===[0m
[32m✓ transactions.py completed successfully[0m
[34m
=== Running crud_scripts.py ===[0m
[32m✓ crud_scripts.py completed successfully[0m
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
📦 Built wheel for abi3 Python ≥ 3.8 to /Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.6.0-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel for current platform...[0m
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.6.0-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.6.0
    Uninstalling ekodb_client-0.6.0:
      Successfully uninstalled ekodb_client-0.6.0
Successfully installed ekodb-client-0.6.0
✅ [32mPython client package built and installed![0m
🧪 [36mRunning Python client library examples...[0m
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {'id': 'n6l2uiuwyfDjbdUENLzso8o2zVJ-4QzJYzJSrVl4izTGc1-VBbXfOaf65NtO4rO6eA6obSDJOs_gT7WCsOymOQ'}

=== Find by ID ===
Found: {'created_at': {'value': '2026-01-01T21:16:40.823012', 'type': 'String'}, 'categories': {'type': 'Array', 'value': ['electronics', 'computers']}, 'embedding': {'type': 'Array', 'value': [0.1, 0.2, 0.3, 0.4, 0.5]}, 'metadata': {'type': 'Object', 'value': {'nested': {'deep': True}, 'key': 'value'}}, 'tags': {'type': 'Array', 'value': ['tag1', 'tag2', 'tag3']}, 'price': {'value': 99.99, 'type': 'Float'}, 'active': {'type': 'Boolean', 'value': True}, 'data': {'type': 'String', 'value': 'aGVsbG8gd29ybGQ='}, 'id': 'n6l2uiuwyfDjbdUENLzso8o2zVJ-4QzJYzJSrVl4izTGc1-VBbXfOaf65NtO4rO6eA6obSDJOs_gT7WCsOymOQ', 'user_id': {'type': 'String', 'value': '550e8400-e29b-41d4-a716-446655440000'}, 'value': {'type': 'Integer', 'value': 42}, 'name': {'type': 'String', 'value': 'Test Record'}}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): True
  price (Decimal): 99.99
  created_at (DateTime): 2026-01-01 21:16:40.823012
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): ['tag1', 'tag2', 'tag3']
  metadata (Object): {'nested': {'deep': True}, 'key': 'value'}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): ['electronics', 'computers']
  data (Bytes): 11 bytes
Plain record: {'created_at': '2026-01-01T21:16:40.823012', 'categories': ['electronics', 'computers'], 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5], 'metadata': {'nested': {'deep': True}, 'key': 'value'}, 'tags': ['tag1', 'tag2', 'tag3'], 'price': 99.99, 'active': True, 'data': 'aGVsbG8gd29ybGQ=', 'id': 'n6l2uiuwyfDjbdUENLzso8o2zVJ-4QzJYzJSrVl4izTGc1-VBbXfOaf65NtO4rO6eA6obSDJOs_gT7WCsOymOQ', 'user_id': '550e8400-e29b-41d4-a716-446655440000', 'value': 42, 'name': 'Test Record'}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'embedding': {'type': 'Array', 'value': [0.1, 0.2, 0.3, 0.4, 0.5]}, 'value': {'value': 100, 'type': 'Integer'}, 'categories': {'type': 'Array', 'value': ['electronics', 'computers']}, 'data': {'type': 'String', 'value': 'aGVsbG8gd29ybGQ='}, 'name': {'value': 'Updated Record', 'type': 'String'}, 'tags': {'type': 'Array', 'value': ['tag1', 'tag2', 'tag3']}, 'active': {'type': 'Boolean', 'value': True}, 'user_id': {'type': 'String', 'value': '550e8400-e29b-41d4-a716-446655440000'}, 'created_at': {'type': 'String', 'value': '2026-01-01T21:16:40.823012'}, 'id': 'n6l2uiuwyfDjbdUENLzso8o2zVJ-4QzJYzJSrVl4izTGc1-VBbXfOaf65NtO4rO6eA6obSDJOs_gT7WCsOymOQ', 'metadata': {'value': {'key': 'value', 'nested': {'deep': True}}, 'type': 'Object'}, 'price': {'value': 99.99, 'type': 'Float'}}

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

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: "BWSwBx8pieGGKCA1EDwMbWk0L5E2kuY91_p0nwH6ri-_ybsrNYDUcsxpFwYT747I0_4w2oj1dirxq9iHQ-xMyw"

=== List Collections ===
Total collections: 10
Sample collections: ['chat_configurations__ek0_testing', 'test_accounts', 'test_collection', 'chat_messages__ek0_testing', 'batch_users']

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: False

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: XTJeJb8ujSsRuYE-ORKJwKBppjcqitiD7jq1GdsiCeI9OJnwU16-UeMfQAKz9s_83xZTHobGALnCEfLuene_XA

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: 2WXjWJeV9YYFFGtmfRuu-o3XcpQ6VgGKxv-70dAeRwNrAUoWeMXSJJmfomYSEKx2Mqn0lSpx6wbVcfhg9UQb5g

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
✓ Inserted test record: fSyUmHGAjw2vGXLD4WW6d6xcKueZwAfHflFzBwSptIoc1JHzWUGeGN0gx-6HBKxRbiAhEn6rbZo48mAXsIvx9g

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
✓ Inserted document with TTL: js65sebgUEspAmT5Fwb7qWZ64k_cYdp4PTeD37PJTspFg8VkcXV9lwKrVpTOWNvVa3WS0tQcMuh8T0T7r7IfJA

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
  1. Score: 25.740, Matched: name, email.value, name.value, email
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title.value, bio, title, bio.value
  2. Score: 26.400, Matched: title.value, bio.value, title, bio
  3. Score: 26.400, Matched: title, title.value, bio, bio.value
  4. Score: 26.400, Matched: title, bio.value, title.value, bio

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title, bio, title.value, bio.value
  2. Score: 39.600, Matched: title.value, title, bio.value, bio
  3. Score: 39.600, Matched: bio.value, title, title.value, bio
  4. Score: 39.600, Matched: bio.value, title.value, title, bio

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
  1. Score: 0.786
  2. Score: 0.771
  3. Score: 0.752

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.751, Matched: title, title.value, content.value, content
  2. Score: 1.139, Matched: title, content, content.value, title.value
  3. Score: 0.527, Matched: 

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
✓ Created session: rtWqQ91ZdgT7AU06vsKV7rcjVfvC8s_aekKGnhDsMuSXt3hPn3DbR2FlL6POaqP5km1zzY-bcFb7-qL6A2CfNw

=== Sending Chat Message ===
Message ID: Qzmf_-K1oiZ9FwE47feilUgJreUBD9cok7D5X3Ej8LGwCx_oc6VJ370WntFxVucguu70R56m_cgF4klRK83akw

=== AI Response ===
There are three products available:

1. ekoDB: This is a high-performance database product with AI capabilities. The price for this product is $99.

2. ekoDB Cloud: This is a fully managed cloud database service product. The price for this product is $499.

3. ekoDB Pro: This is an enterprise edition product with advanced features. The price for this product is $299.

Execution Time: 2190ms

=== Token Usage ===
Prompt tokens: 608
Completion tokens: 84
Total tokens: 692

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: Qbq7z6JqiL85ZJejuPI55MMKxTTGgr7gDxB3N_FcD6e2OvrOabfKIMAFkSLjuM60ApEL40ENOiyaThC9cTF_uA

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the context provided, the available product is ekoDB. It is a high-performance database product and is priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: BGMBN5FnLMG7fmvQingbiIOc1-1d4OlqgIkgCgkKikBgzBz64iqyX-kbAhrTf3V42AxL7TmtWYrCANGNbCvlnQ
  Parent: Qbq7z6JqiL85ZJejuPI55MMKxTTGgr7gDxB3N_FcD6e2OvrOabfKIMAFkSLjuM60ApEL40ENOiyaThC9cTF_uA

=== Listing Sessions ===
✓ Found 6 sessions
  Session 1: BGMBN5FnLMG7fmvQingbiIOc1-1d4OlqgIkgCgkKikBgzBz64iqyX-kbAhrTf3V42AxL7TmtWYrCANGNbCvlnQ (Untitled)
  Session 2: Qbq7z6JqiL85ZJejuPI55MMKxTTGgr7gDxB3N_FcD6e2OvrOabfKIMAFkSLjuM60ApEL40ENOiyaThC9cTF_uA (Untitled)
  Session 3: rtWqQ91ZdgT7AU06vsKV7rcjVfvC8s_aekKGnhDsMuSXt3hPn3DbR2FlL6POaqP5km1zzY-bcFb7-qL6A2CfNw (Untitled)
  Session 4: M5jfgFMLXAGf8KQ-Kc3oeMIk1LG1uVKrEIKZP_wh993kb2mGcllzluKQ5mV-o6qZU1qofEi4eQOSEA6BACYP8g (Untitled)
  Session 5: ipy1SsZHmJi1lwpkk_PUStFrqpVFSIw6gysmgKGyw0jZRdO3MHb5ubGGqXzRzjxFLn5IOhTi_xUNuuO_97A0hg (Untitled)
  Session 6: EIki62qqB5lU5m6nt01y8UfVL_LgF66otheWB_KKRJMELVCaNA2lRWOQuszEK7iX-FhyXi-qmmcIhK7m9S_YMw (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: BGMBN5FnLMG7fmvQingbiIOc1-1d4OlqgIkgCgkKikBgzBz64iqyX-kbAhrTf3V42AxL7TmtWYrCANGNbCvlnQ

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: FLzKUYJMBI5-zHXig94uMv_cy328YdSpQb-5NLRySAVU8DRpTq5kHxuCHd1Afw5LKu6kxAhP9uHmRVK28UOtRw

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information available, the product "ekoDB" is available. It is a high-performance database product and is priced at $99.

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['token_usage', 'context_snippets', 'id', 'updated_at', 'role', 'created_at', 'chat_id', 'content'])
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
✓ Created second session: IrEDW5RVJSGcKFA6HYlzC3Jo45l1VQTq3iAf0nUXBShA7QxYasnCKR91wfvsZWK5sehGUT0fSMyCS8_ck9yeIQ
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

✅ Script saved: bCSzccu52FZxnM2hTkwvXDU8DUdtY9AeBQU8rvrVhLUCx-_lg2e8faVhBfNNwFm7b5AKYbvLhTELo_NSCz-Wkg
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved: _WLMDNpm1VVUnPwaXz82VVdRjpHeSyHa5OsaqJjpj9HFInl5p9kWK3DGkX4Swf-JC195vv2581J57uiIOyjk7Q
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: z2Ie0ZfUb112mAQCqWPvajKR1trpKpQdGfVNyv4ef6a5w1iDnHChB85gZtTfdo3uARKtQ_XpkCO1xNIc1dq9Uw
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

✅ Script saved: 8jNxhf_5luqZDgT56yq8VGXwTLJe1QIzCwxYB2f4tjtGiwBUbVgOzmptRjnJQ_vBtd1YRwGEM186buxC-of7Ow
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
✅ [32mPython client examples complete![0m
