make test-examples-python
🧪 [36mRunning Python examples (direct HTTP/WebSocket)...[0m
✓ Authentication successful

=== Insert Document ===
Inserted: {'id': 'W8vbx_g5wVRjgJx8djaqBkCT1N8m79fPUCkDfHLAKwsNB8kVHpC9524CqJOcdBmhzKD4DW8u3ln7TaI6qZ7KfQ'}

=== Find by ID ===
Found: {'value': {'value': 42, 'type': 'Integer'}, 'id': 'W8vbx_g5wVRjgJx8djaqBkCT1N8m79fPUCkDfHLAKwsNB8kVHpC9524CqJOcdBmhzKD4DW8u3ln7TaI6qZ7KfQ', 'active': {'type': 'Boolean', 'value': True}, 'name': {'type': 'String', 'value': 'Test Record'}}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'value': {'type': 'Integer', 'value': 100}, 'name': {'value': 'Updated Record', 'type': 'String'}, 'id': 'W8vbx_g5wVRjgJx8djaqBkCT1N8m79fPUCkDfHLAKwsNB8kVHpC9524CqJOcdBmhzKD4DW8u3ln7TaI6qZ7KfQ', 'active': {'value': True, 'type': 'Boolean'}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: kL8DVSUh7_LbJlGQ8Jp8rpmf3HJUvw0sR1K61EIbOfhPQNbCdjciPwd8xTHlb6AY0AiPX1Gmo0kplRygdd5vow

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
        "id": "kL8DVSUh7_LbJlGQ8Jp8rpmf3HJUvw0sR1K61EIbOfhPQNbCdjciPwd8xTHlb6AY0AiPX1Gmo0kplRygdd5vow",
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
        "id": "IjdISjptT6bysMqTDvn82myude-F1EHII_2b_ly8-sMxVL2QcMVlBbQPhso4ZGmgB5bQBHKla7FbAnTqScFsJw",
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
Retrieved value: {'userId': 123, 'username': 'john_doe'}

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: {'name': 'Product 1', 'price': 29.99}
cache:product:2: {'price': 39.989999999999995, 'name': 'Product 2'}
cache:product:3: {'price': 49.989999999999995, 'name': 'Product 3'}

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: dN6BoJNNKmFJiEaKrxxfr5zZL0TJBPtG7oyVdL5rZvI02KpJ4iCUdcVWo6ZRa6LWLsSon_eg_J6xPJHM6sBDpw

=== List Collections ===
Total collections: 11
Sample collections: ['test_accounts', 'users', 'ttl_cache', 'ws_ttl_test', 'chat_configurations__ek0_testing']

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: False

✓ All collection management operations completed successfully
✓ Authentication successful

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: 7YGPrcYM7oXi7jzJnIYz3Z737d484neLxCRWn_qtVMeivUGroBR5ZYYSEfQPN4pkQrjS7kBvwEjpRBQ_wt7Lig

=== Insert Document with TTL (5 minutes - integer) ===
✓ Inserted document: created

=== Insert Document with TTL (30 minutes - duration string) ===
✓ Inserted document with duration string TTL: cXHxuiuw2XmxmvY-_QoWpVsqMGKqudSHEP5TRUWR4q90KE5YkZ-SlgnOg-RpyHQin--1uGo9HKJEVXEdmMA2nQ

=== Query Documents ===
✓ Found 5 documents with TTL

=== Update Document ===
✓ Updated document

=== Delete Document ===
✓ Deleted document

✓ All document TTL operations completed successfully
✓ Authentication successful

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: _sP-bqSQBPb9ShFJ4kgu_mqiYqNlgXYkLx5lwL9nVhYM7pdPhguL4u7ElUnW7MRPd0K5q8bL3N0Q8XtCP4ARfw

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 2 record(s) via WebSocket

✓ WebSocket TTL example completed successfully
🚀 ekoDB Scripts Example (Python/HTTP)

✓ Authentication successful

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: p5Rgtd69AhJoxpmufXf_dE5frfxl6z-Q5zzhqg3WsyOYIk00rSTVDH1D7IAL4Dwp6iLR_a2fDCFxHE84R8i73w
📊 Found 30 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: dlG2Jut91eGvyk_MySi0M0YgKX3umNfysG8IysZhjST-_QO7CuPncRL_PNjUhWdtZs0v4ApTNGv-urdjiZONdg
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: FAgVzqrMuYLmozAMTjXbHpvOx4OtbiiWJ87AQPkCpAbgF4RHwAP-9_LrFW-dZwRakAkYX6Tz2buSCVZTSn05PA
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {'status': 'inactive', 'max_score': 90, 'count': 15, 'avg_score': 50.0}
   {'status': 'active', 'count': 15, 'max_score': 100, 'avg_score': 60.0}

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
Created Alice: $1000 - ID: rlST9Hj8yVQ8OS-SD__hgmdr5KqkF02_Yy_OvFSbBOSrIENmzNB37z7x41udjBEzS-Q9E4LtcQPZ_s6NevTVfA
Created Bob: $500 - ID: yAKGSr95gNbzumn-WfkSssf9ofqS0QHXc9Hdq3On5enGPgsXrFuvTuUST922z67WamUdFI8OPk7ykW-L0MY2uA

=== Example 1: Begin Transaction ===
Transaction ID: 7450175e-5356-4d76-a42e-b4c7296985ad

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
New transaction: e6e6e0d6-264b-45ae-87bd-dbf3b71bad80
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: E-LkehClOGh_Bl2e-4Tb03dgxLslJYYAa_-h1-ncSJTE_ibmzgBzBag34K7HSm1xACfnbrX3xGVy8WIBKLUqmA
Created Bob: $500 - ID: yz3wniFHT6F5soPaDXcThxzvxCkdWkIAxutcl89hNDa_Sf1OcNhjKcokyvbFgm-XZg2qgQPZSh1z6t1B6xApAg

=== Example 1: Begin Transaction ===
Transaction ID: 8ee12960-c888-4372-8cf1-2558995e70dd

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
Created Alice: $1000 - ID: ut3G523iKyowpbt_tGgcvAiF4x6k2Ly_Ly1vGKeU4USDp580TvCL5l72Hop7Bk-RxuqGwmKEALm6qxUSyzYScQ
Created Bob: $500 - ID: wrL8Xode5pWyFt19KJi7qNLo4ZYdMa7ezMPvXpjjrmU1SKluR6SZSGi2gXHuzGyXgGQdLlOG_41QJLfvxZ_yLg

=== Example 1: Begin Transaction ===
Transaction ID: 37c38f35-65d5-4174-b4bb-1a5829434afe

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
New transaction: f10c40e3-4f15-4460-8acc-4da11a2c06a7
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: gb9OZLdWF0p8yD0O8JXLHzOWGgAEG1HmPTUMN9lLxkeIWNDDvP6GOki5ed2cUOpDT65lMMFAkJS0xevD9w2yEA
Created Bob: $500 - ID: Pp5H5KsQsd6XwpcZ_7C97beTqSuWehaMfmzZPp1WIcUjvDfEm-UeDoMQK1hCMhMRnzSPLSiDCK9vMtBhyP1jzA

=== Example 1: Begin Transaction ===
Transaction ID: 0db1b769-3f6b-47a3-a69f-5990f70c3eea

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
Created Alice: $1000 - ID: s_F-ZG1Mkve5LzOvXCxpFFERcYYsLC_ECjeE6We7VA9kLqgrOuIHmcTf_r3cTLbmkgQM2xYmW6J5A8a6W4-e8w
Created Bob: $500 - ID: _Y4B5lK9Jb_7zMpCnX7eY-EByVNZG7biuvUWIrJLEBUwt_4CH8dad_7S2N-At7vb4HM4428mEZQdiG5c3O-TTA

=== Example 1: Begin Transaction ===
Transaction ID: f7d3378d-5119-44b8-a5c1-3e777e833994

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
New transaction: d81ec17a-0773-4d51-959c-1c3a271a6b89
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: e7pMfuW3hK9gaoWoMtXi5sDKhjKuXzu_JSswbUF0W1VNUVirIOdU-2rY6Y-ldSxNjAuHLpixbA-XodMbjG5lbA
Created Bob: $500 - ID: Ofa5QzfKT7OttEGjhwFdts6INWrOZLEcdtV-Ij2cHnPx-ap7CCmELIw5W9mpqt0XurA-3id0-6uhrRZvn_k3Gw

=== Example 1: Begin Transaction ===
Transaction ID: 548ba4b1-6a4d-4522-ab14-e79bc0b48def

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
   ✅ Script saved: 4_moP4qWwVujv8Ck-ZuXiZbPiVMIc9JIDFToQ2MvS_bOBYSGtmJldrsiqIX0U95f2k2TK3rBLBY8MNq-B3lOiw

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: UDbNRXy6WEAgGsFuFTrqZv8SuQ-92DkR4_ZWKcGEEUNEBkM3wBAkcn8BDGSEzqKKA2Mcnv2mz_lC3nIqlN61Rg
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}
   📋 Email: {'type': 'String', 'value': 'alice@example.com'}
   📋 Status: {'value': 'pending', 'type': 'String'}
   📋 Credits: {'type': 'Integer', 'value': 0}

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: bHVcPK7HO1xHXnjUw2doD2e6Ww7XiMvzCOFhja9LBWG_h8TPeeop9bJXgo22EChF4xGVSpoXDW8mOTgW4W13bQ

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
   ✅ Script saved: bsj-RCFOTzcecHrkXgpG8UBwQNlUsy7BC22vbXdL8TUEenNpGMMzoszobjynxKJDDL0itCGalTWLao8ttA2p8A

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
   ✅ Script saved: IqGSwJwAxKhHejdZtSHfuXhGJDKg4cdiiTloNqN3X5LmvPd5eJtAjtX3kC5_gcnnvRHPElCMoNLT5zvFFAEJVg

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: 4_moP4qWwVujv8Ck-ZuX...
   ✅ Deleted script: bHVcPK7HO1xHXnjUw2do...
   ✅ Deleted script: bsj-RCFOTzcecHrkXgpG...
   ✅ Deleted script: IqGSwJwAxKhHejdZtSHf...
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
    Finished `release` profile [optimized] target(s) in 0.08s
📦 Built wheel for abi3 Python ≥ 3.8 to /Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.7.0-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel for current platform...[0m
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.7.0-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.7.0
    Uninstalling ekodb_client-0.7.0:
      Successfully uninstalled ekodb_client-0.7.0
Successfully installed ekodb-client-0.7.0
✅ [32mPython client package built and installed![0m
🧪 [36mRunning Python client library examples...[0m
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {'id': 'YT15MOmk9O49hyHQ3MtgvbDIE9ySeJM2F5ExMLrhmFl7nqYSXKVBDmOxlwrP9K2Jqve0GkQ2g1ZcNb2Zi_x3Uw'}

=== Find by ID ===
Found: {'value': {'type': 'Integer', 'value': 42}, 'name': {'type': 'String', 'value': 'Test Record'}, 'price': {'type': 'Float', 'value': 99.99}, 'data': {'type': 'String', 'value': 'aGVsbG8gd29ybGQ='}, 'id': 'YT15MOmk9O49hyHQ3MtgvbDIE9ySeJM2F5ExMLrhmFl7nqYSXKVBDmOxlwrP9K2Jqve0GkQ2g1ZcNb2Zi_x3Uw', 'metadata': {'value': {'key': 'value', 'nested': {'deep': True}}, 'type': 'Object'}, 'active': {'type': 'Boolean', 'value': True}, 'created_at': {'value': '2026-01-03T04:50:35.017567', 'type': 'String'}, 'user_id': {'value': '550e8400-e29b-41d4-a716-446655440000', 'type': 'String'}, 'tags': {'type': 'Array', 'value': ['tag1', 'tag2', 'tag3']}, 'categories': {'value': ['electronics', 'computers'], 'type': 'Array'}, 'embedding': {'type': 'Array', 'value': [0.1, 0.2, 0.3, 0.4, 0.5]}}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): True
  price (Decimal): 99.99
  created_at (DateTime): 2026-01-03 04:50:35.017567
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): ['tag1', 'tag2', 'tag3']
  metadata (Object): {'key': 'value', 'nested': {'deep': True}}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): ['electronics', 'computers']
  data (Bytes): 11 bytes
Plain record: {'value': 42, 'name': 'Test Record', 'price': 99.99, 'data': 'aGVsbG8gd29ybGQ=', 'id': 'YT15MOmk9O49hyHQ3MtgvbDIE9ySeJM2F5ExMLrhmFl7nqYSXKVBDmOxlwrP9K2Jqve0GkQ2g1ZcNb2Zi_x3Uw', 'metadata': {'key': 'value', 'nested': {'deep': True}}, 'active': True, 'created_at': '2026-01-03T04:50:35.017567', 'user_id': '550e8400-e29b-41d4-a716-446655440000', 'tags': ['tag1', 'tag2', 'tag3'], 'categories': ['electronics', 'computers'], 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5]}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'id': 'YT15MOmk9O49hyHQ3MtgvbDIE9ySeJM2F5ExMLrhmFl7nqYSXKVBDmOxlwrP9K2Jqve0GkQ2g1ZcNb2Zi_x3Uw', 'categories': {'type': 'Array', 'value': ['electronics', 'computers']}, 'value': {'type': 'Integer', 'value': 100}, 'data': {'type': 'String', 'value': 'aGVsbG8gd29ybGQ='}, 'tags': {'type': 'Array', 'value': ['tag1', 'tag2', 'tag3']}, 'name': {'type': 'String', 'value': 'Updated Record'}, 'user_id': {'value': '550e8400-e29b-41d4-a716-446655440000', 'type': 'String'}, 'metadata': {'type': 'Object', 'value': {'nested': {'deep': True}, 'key': 'value'}}, 'created_at': {'value': '2026-01-03T04:50:35.017567', 'type': 'String'}, 'active': {'value': True, 'type': 'Boolean'}, 'price': {'type': 'Float', 'value': 99.99}, 'embedding': {'value': [0.1, 0.2, 0.3, 0.4, 0.5], 'type': 'Array'}}

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
Collection created with first record: "IWkbr53nt_eHtqqTjJFKzu9L_Fm0uAPeBHLL7XhdydGsX6vJD2SQ4e-VMxE19jb4cKqFey_PBCE_HtHriSeuMg"

=== List Collections ===
Total collections: 10
Sample collections: ['test_accounts', 'ttl_cache', 'ws_ttl_test', 'chat_configurations__ek0_testing', 'websocket_test']

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: False

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: O1IPNZZbGn0a1ng2A-MlrlsY1lrcDuy06aSSbJL2k2Q2yhQcZU3lZ2d8obK2dEom6RK1OpZgstij8t-jyzUIAA

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: iBIGulWCK6iCCbuKL5E2d4CxgXD84GkpqA1XvOpC1FxPNku6FQ_Tf4px_YxY_zN5xNHj0MntKlwBiZp_Cc1TWg

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
✓ Inserted test record: lD0I63jZPMBRJwXlxulK_7z6sKGvTvUixzEnHeT-ZNJIG_ZwRW-0JsiUChqOQsvSrAMs9hXM0NeyaxFJJHtJMg

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
✓ Inserted document with TTL: 7cxgxiFDCfE-1fmqfHBrYywg8g3KbZlA3_vPKp0geNUQE3fD7LdgLJAwQTIhd5kq8NAXshXTBehBMmz-CQV_cw

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
  1. Score: 25.740, Matched: name, email.value, email, name.value
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: bio.value, title, title.value, bio
  2. Score: 26.400, Matched: title.value, bio.value, title, bio
  3. Score: 26.400, Matched: bio.value, title, title.value, bio
  4. Score: 26.400, Matched: bio.value, title.value, title, bio

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio, bio.value, title.value, title
  2. Score: 39.600, Matched: title.value, bio.value, title, bio
  3. Score: 39.600, Matched: title, title.value, bio.value, bio
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
  1. Score: 0.783
  2. Score: 0.753
  3. Score: 0.751

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.727, Matched: title.value, content.value, title, content
  2. Score: 1.126, Matched: content, content.value, title, title.value
  3. Score: 0.548, Matched: 

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
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: YQzzCfZjPdw5kP1aBN6THyY1bRvLMtCvolkzyPoRuQRuLMHMWeb6xYWJbJXPi-iZTzqBLMxeJJFJzPXJlfwLbQ

=== Sending Chat Message ===
Message ID: zF3RAkXpeSLhWD16K_OeA79Q8AvaGZeXTh0YiwuUIBkEWpey0bDZj85ReifvLJPrwAGOqyl_1ZBTRIDFQ8TnUA

=== AI Response ===
The available products are:

1. ekoDB Cloud: This is a fully managed cloud database service product. It is priced at $499.

2. ekoDB: This is a high-performance database product with AI capabilities. The cost for this product is $99.

3. ekoDB Pro: This is an enterprise edition product with advanced features. The price for this product is $299.

Execution Time: 2720ms

=== Token Usage ===
Prompt tokens: 599
Completion tokens: 81
Total tokens: 680

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: r8-q1NPgr8SOnlJxjWfntm6zuChhix2h6xtYe5ervcSBCOE-h8CFI_i7m9pJxLVzK_gop-dpdEHNtpNLVEH2Zg

=== Sending Messages ===
✓ Message 1 sent
  Response: Answer: One of the available products is ekoDB, which is a high-performance database product. The price for this product is $99.

✓ Message 2 sent
  Response: The price of the product "ekoDB" is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: CilX15i-UC2kwOpaRifVfHnnduebob47Ypk9S3zki1WChBCYdbJRKxVfOVmLExlQHmO6gDbJxHYpnce1zT4cXw
  Parent: r8-q1NPgr8SOnlJxjWfntm6zuChhix2h6xtYe5ervcSBCOE-h8CFI_i7m9pJxLVzK_gop-dpdEHNtpNLVEH2Zg

=== Listing Sessions ===
✓ Found 6 sessions
  Session 1: CilX15i-UC2kwOpaRifVfHnnduebob47Ypk9S3zki1WChBCYdbJRKxVfOVmLExlQHmO6gDbJxHYpnce1zT4cXw (Untitled)
  Session 2: r8-q1NPgr8SOnlJxjWfntm6zuChhix2h6xtYe5ervcSBCOE-h8CFI_i7m9pJxLVzK_gop-dpdEHNtpNLVEH2Zg (Untitled)
  Session 3: YQzzCfZjPdw5kP1aBN6THyY1bRvLMtCvolkzyPoRuQRuLMHMWeb6xYWJbJXPi-iZTzqBLMxeJJFJzPXJlfwLbQ (Untitled)
  Session 4: kjC5gFDXqUMkJtTG9oQxoJ4GZsqn4-JnYCiIcGGKMR0243BGdCUoT_VC4DQP5SHwUY2nER_UmhkyajKM6mJdfQ (Untitled)
  Session 5: SI0IVjSitRK4WC0Rq1i2_ftWMw6wc_a-F_5jxxLXdDFcbDYkAVkRLYe-vu7rx-cnpa6CbU66anXhCjolvyc_ZQ (Untitled)
  Session 6: _7tHWwvnTeyUz_55D47bdvKv-LR17o5N-3frkweqyiY75TE85p882gSX8NaWWrOIRzliqYJXFtiK6qpW5i5ydQ (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: CilX15i-UC2kwOpaRifVfHnnduebob47Ypk9S3zki1WChBCYdbJRKxVfOVmLExlQHmO6gDbJxHYpnce1zT4cXw

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: jXNcdOZuMBn1XW-A-bOVBhQD--2S4HRWKD4P6jTz6krHXqoBRIoC3yXBLc8_6zRQTPvUCluI5FEhX6QMT5GBwQ

=== Sending Initial Message ===
✓ Message sent
  Response: The product available based on the provided context is ekoDB. It's a high-performance database product.

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['updated_at', 'role', 'chat_id', 'created_at', 'token_usage', 'content', 'context_snippets', 'id'])
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
✓ Created second session: kjK1pDTNLOKHPH9OvZuvvQLFzTMCSQt-byLD2bA78E2U9VUsJHSz_I5zfuQo1BXHoSW53CpQnsFBhK4Q530HVw
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

✅ Script saved: 9Idosq_OCUDgUk1SBFRPvHqJF0EmUnyqeZVqFJBRj8iCz533wMTvfQhAb1dCOLsZysSuTiS1laYhTfD5bxXdqQ
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved: uMHmWwya9IrBqa2Jro9yl8AER9Rn1D6t3FVHcl3NheXnS24ZYESGJj2sgUk4InX5KCNtzGOC8HALJA5YAYLh9g
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: 0iMPU3Zy9NHo2XLWz0uaNTiqq7WBADc3IGkGClEwuBpktvLFFvo1zHl7f7pXnWpJQWR6NtcA8E_84nmQ73eXKg
📊 Statistics: 2 groups
   {'avg_score': 50.0, 'count': 5, 'status': 'inactive'}

   {'avg_score': 60.0, 'count': 5, 'status': 'active'}

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

✅ Script saved: c3GaK-dz2fGVRUxgZD6-yXx-RYaq00ds-SQoJ0ESEwq1axJb3f9yeP6l9nzZwT03xxQ0LBSgx-PJ9uclY4v4eA
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
Created Alice: $1000 - ID: 7jc0fqV-Z3Yhe3Li7uylMaInByvhaWmWCbdv1wec8aBOev2pKDpVNNDQWR6TjYqL49zkwpzuVqy0kLFP3y1JkQ
Created Bob: $500 - ID: 7yfyWZn4M631frINfZT62K-cTTc_XFvpv6sI7jXCe_46GgwX7pk6uAbYKRbBtVh0e1J_TOz_mpldGXYVhye6mw

=== Example 1: Begin Transaction ===
Transaction ID: c2eae7c1-2ffe-4c22-9cc6-c11adb24e73d

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: c18f88a6-c56c-45e8-9cad-b7ecd9778cd3
Status before rollback: Active
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
✅ [32mPython client examples complete![0m
