make test-examples-python
🧪 [36mRunning Python examples (direct HTTP/WebSocket)...[0m
✓ Authentication successful

=== Insert Document ===
Inserted: {'id': 'fGZzLK6DlHlYw_vuW5BdF8YQXrma1EQ4ocsUMij_eg6Oxl8iprLKJ2foPQMZseIXJcAQJgniME-aUHRmuWuP7g'}

=== Find by ID ===
Found: {'id': 'fGZzLK6DlHlYw_vuW5BdF8YQXrma1EQ4ocsUMij_eg6Oxl8iprLKJ2foPQMZseIXJcAQJgniME-aUHRmuWuP7g', 'active': {'type': 'Boolean', 'value': True}, 'value': {'value': 42, 'type': 'Integer'}, 'name': {'value': 'Test Record', 'type': 'String'}}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'value': {'value': 100, 'type': 'Integer'}, 'name': {'value': 'Updated Record', 'type': 'String'}, 'id': 'fGZzLK6DlHlYw_vuW5BdF8YQXrma1EQ4ocsUMij_eg6Oxl8iprLKJ2foPQMZseIXJcAQJgniME-aUHRmuWuP7g', 'active': {'value': True, 'type': 'Boolean'}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: a3HSAeDJ4dkj4hErQeiyqFAv339ot0i90jIBizcekkcFWpdStzLfwrQA-ejOxfbPFxFuF9w977m4x7DpOIeeUA

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
        "id": "nM-4Yeiy_NNKrQ8nYlvtKtF7PcI8tNE5I1l7Gsolj9YOSEoI5ZHPa-lL6fSZAOlMYl6LvJkAmUO3NNZv-wV4Aw",
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
        "id": "a3HSAeDJ4dkj4hErQeiyqFAv339ot0i90jIBizcekkcFWpdStzLfwrQA-ejOxfbPFxFuF9w977m4x7DpOIeeUA",
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
✓ Verified: Record updated with status="{'type': 'String', 'value': 'active'}"

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
cache:product:2: {'name': 'Product 2', 'price': 39.989999999999995}
cache:product:3: {'name': 'Product 3', 'price': 49.989999999999995}

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: RmN4VZwuFS0ghNDKfu4LGk_fnkXJ__TNbuAJ_22wzYmLLtLizi_anQQYY91_Y0kTy-7SBqC7t278T9KKBvaelA

=== List Collections ===
Total collections: 11
Sample collections: ['batch_users', 'chat_messages__ek0_testing', 'test_collection', 'users', 'scripts__ek0_testing']

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: False

✓ All collection management operations completed successfully
✓ Authentication successful

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: ddnT9nDPDw_cAbQ8UN6H0xcisg5xZ7ueY_nnwhAz9cLsrKHbxZEbPJoK3q-pw4dyLaQwW9j-_Bci0IXENxkPWg

=== Insert Document with TTL (5 minutes - integer) ===
✓ Inserted document: created

=== Insert Document with TTL (30 minutes - duration string) ===
✓ Inserted document with duration string TTL: n74H60Jy-8ItOmBWLaOqrdF3DjQPBFZKBCSM1WYW6TBwkjYoZv1tOPrwuAYpF1I2Ym1m_FhZWGyKC2X9Dc-8ig

=== Query Documents ===
✓ Found 5 documents with TTL

=== Update Document ===
✓ Updated document

=== Delete Document ===
✓ Deleted document

✓ All document TTL operations completed successfully
✓ Authentication successful

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: DRlQMjPPYPXnMTos_Z9TYq34I-7tUR5s0E6su4FeO3TkjVLsnZTY5NSBix66RSE8ma-tcUh0aIURYmfHRGMBJg

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 2 record(s) via WebSocket

✓ WebSocket TTL example completed successfully
🚀 ekoDB Scripts Example (Python/HTTP)

✓ Authentication successful

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: YEpdKoHVrexVtGpvAz_TG8ypFA16j7U5NObeihfZKGfxxlOVBRJyqe1El3jNjKzOGsmIGT6MASPqPw6Z7-wnGw
📊 Found 30 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: vyeA1eo-_tDqYgBFgaRahqRWOtTF6GorJJX5tJQBO0CY5DaA5VOOn58rYRRHK873j-Ru1GrxMN3ZqQKtQx15LA
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: -tN6QXzrPJF_KZn108nKqjXmS4O0E9crBCKgxrZhH1QlsNXGAv3gcN3ECk3_w0MbQWUh1qbXcT916fMbdhCK8Q
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {'count': 15, 'avg_score': 50.0, 'status': 'inactive', 'max_score': 90}
   {'avg_score': 60.0, 'count': 15, 'status': 'active', 'max_score': 100}

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
Created Alice: $1000 - ID: dauWCYmH0KIOJpRRAowt2PWcshgqB3RzQcz_n_4BGxO8HTuBW1jLuGLHKJZk4BAMd_VEd7uW4MKw-18PFxULyw
Created Bob: $500 - ID: iX01rzu7JSSgVTg_q6OjEe3a8POrLLzo7CocLuz-Sm8z61frReeOkmgaLALfpblvF0R92mi4r_NFGj2ih_CrNQ

=== Example 1: Begin Transaction ===
Transaction ID: 3f01a7a0-1a09-41f5-b6cf-6e13ff6bed3d

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
New transaction: 5529e99f-17df-4453-a247-ba64128ead6a
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: ObA6f-Cn1FRqOqlgia9kkTXfH9dCiM5GGyQ41fZR6DRVw27MKGodyiPtcsJZiDdEKKWDeL5vCmtQPYB6zRgjqQ
Created Bob: $500 - ID: 8GOCsHbtquT7xBuge5Bnvg_5C5IEX6JY0KOK5lLTWXXq1dDjoJ5zB3euGGaSBpFzvO4RjJJ58yxjBzHU0guEgg

=== Example 1: Begin Transaction ===
Transaction ID: 5167ef1a-dfda-4fd3-9c50-f323cce76cd7

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
Created Alice: $1000 - ID: S5t2oSUdGFc1DtGKonhjVheGA6OVDvaX0D4V8VbVszsF1y7NhNd5O6gcD6RbX85R38cfXBzP3vy4VcyoK08Gzw
Created Bob: $500 - ID: k93wbIUSKsSNDOiIJYR_xIn2sdrYVKLtIVUJDyC4x7NMVefmRhmYJOl1nALbfEpu3VgqUUWmtvP-1t4Fhs-Wbg

=== Example 1: Begin Transaction ===
Transaction ID: 0ab85737-5b2a-49a4-8134-e0ce5e3339bd

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
New transaction: c7f509a2-0a21-43fc-aefa-347587f45f48
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: Wi_TWJVL9j3_X3dScIrAJNtBuyaNxkhgT8u3WCV2Xy_dM9cTUB3qB8nQJZugMhZnS_xHTuAEyCTO-USt3-r_LQ
Created Bob: $500 - ID: dlhrEcuskjlasNag4KeZjxvGhM1k7sLoiqJdz4p70Jtk4HaVpeEvb31HACHSK6Une8PuJPzwnweveXucZ3vThg

=== Example 1: Begin Transaction ===
Transaction ID: 185c89d1-2ab3-49df-abd2-681951b23fe7

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
Created Alice: $1000 - ID: QEA_lRRjpyqHu2289FE76Ckes6aMWWH4GN7QXWc9UgCVRaDJSy7YCjkdzp0jIqUl0mw2piR9zhHz2OjjB_SVtg
Created Bob: $500 - ID: GujXfca1YzbYJXXUcKUv1Uo-R5tLcwfvsB9mdjDCwFHOR7r7pIGxWeJmw5em5_TTHDLrAjF18DWRs9CkEb4vBQ

=== Example 1: Begin Transaction ===
Transaction ID: df101bc6-c2f6-4c6d-b9de-39392ed8e7e3

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
New transaction: 67a9a696-576d-42b3-a408-abb955d4dac8
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 6tpxBeAKTWt7fn_SPXuXpN_GFY2TDf867PNjPvVuqpu45tvxuSD7uBfrSYRF0TUlVmN-HKn8wJ0v31CYF5oPyQ
Created Bob: $500 - ID: pmb1py6dC9-9sKYZIy_6YrVtX3DoWXFAu96SAdXsMRailMpEakGqcob2OzHG7mRdBfr2kNQSgWDysaUud9QvWw

=== Example 1: Begin Transaction ===
Transaction ID: 29c71e98-b41c-4806-a12f-c29dd26605aa

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
   ✅ Script saved: wn_6rC1cGRj2rLCUAO8HWv_fUJd5dkf37rY5bihHtqZSbfaZiMdGzOvGwflbJPVWJtxcd6CfaylnP9wS1XYexA

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: tVVzOeW_eADjf6DlOnm8tJJzSuLWCmQIMDVgC7rHxjqDMH4K9LdcXn6myyu-ryFSGL8sR5kUx2ui1heZgytiyA
   📋 Name: {'value': 'Alice Smith', 'type': 'String'}
   📋 Email: {'type': 'String', 'value': 'alice@example.com'}
   📋 Status: {'value': 'pending', 'type': 'String'}
   📋 Credits: {'value': 0, 'type': 'Integer'}

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: gBNmcbWvZots_ZvRxLD3BeaIACdHvMTDlDdNE8e6Jwgsoi7kUE2DrIK7hYgX3VgbfogKjdh5xY6iJZ3GCv2v8Q

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
   ✅ Script saved: TwPl6J0gSYUAPOtUzF5MPPrzpAYHCsj7eR1lml41hDMlB_NUtzWpnyzfUlbnqMguA1VZjMOqIlbKHhCFsRcSzQ

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
   ✅ Script saved: gcfz4TNpRfKBS0CFU1Xx3yfOqP9rBTzORLPQG-R0_PzfIKhAV_YMmUWPmodbLYctNxZ1YS-YRP9oyAH04PRmSA

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: wn_6rC1cGRj2rLCUAO8H...
   ✅ Deleted script: gBNmcbWvZots_ZvRxLD3...
   ✅ Deleted script: TwPl6J0gSYUAPOtUzF5M...
   ✅ Deleted script: gcfz4TNpRfKBS0CFU1Xx...
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
Inserted: {'id': 'M2bXYIA1JLzj_XQOkVarsxpU1w3mrEuZKTBy7jt6p2E-bJE1dGX2MdrqTFuaZM0PSdohpAGhcnshHPa01eA6uQ'}

=== Find by ID ===
Found: {'data': {'type': 'String', 'value': 'aGVsbG8gd29ybGQ='}, 'price': {'type': 'Float', 'value': 99.99}, 'categories': {'type': 'Array', 'value': ['electronics', 'computers']}, 'embedding': {'value': [0.1, 0.2, 0.3, 0.4, 0.5], 'type': 'Array'}, 'id': 'M2bXYIA1JLzj_XQOkVarsxpU1w3mrEuZKTBy7jt6p2E-bJE1dGX2MdrqTFuaZM0PSdohpAGhcnshHPa01eA6uQ', 'tags': {'value': ['tag1', 'tag2', 'tag3'], 'type': 'Array'}, 'active': {'value': True, 'type': 'Boolean'}, 'created_at': {'type': 'String', 'value': '2026-01-04T01:32:12.725936'}, 'value': {'value': 42, 'type': 'Integer'}, 'name': {'type': 'String', 'value': 'Test Record'}, 'metadata': {'value': {'nested': {'deep': True}, 'key': 'value'}, 'type': 'Object'}, 'user_id': {'value': '550e8400-e29b-41d4-a716-446655440000', 'type': 'String'}}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): True
  price (Decimal): 99.99
  created_at (DateTime): 2026-01-04 01:32:12.725936
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): ['tag1', 'tag2', 'tag3']
  metadata (Object): {'nested': {'deep': True}, 'key': 'value'}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): ['electronics', 'computers']
  data (Bytes): 11 bytes
Plain record: {'data': 'aGVsbG8gd29ybGQ=', 'price': 99.99, 'categories': ['electronics', 'computers'], 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5], 'id': 'M2bXYIA1JLzj_XQOkVarsxpU1w3mrEuZKTBy7jt6p2E-bJE1dGX2MdrqTFuaZM0PSdohpAGhcnshHPa01eA6uQ', 'tags': ['tag1', 'tag2', 'tag3'], 'active': True, 'created_at': '2026-01-04T01:32:12.725936', 'value': 42, 'name': 'Test Record', 'metadata': {'nested': {'deep': True}, 'key': 'value'}, 'user_id': '550e8400-e29b-41d4-a716-446655440000'}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'created_at': {'type': 'String', 'value': '2026-01-04T01:32:12.725936'}, 'name': {'value': 'Updated Record', 'type': 'String'}, 'value': {'value': 100, 'type': 'Integer'}, 'price': {'type': 'Float', 'value': 99.99}, 'active': {'value': True, 'type': 'Boolean'}, 'data': {'value': 'aGVsbG8gd29ybGQ=', 'type': 'String'}, 'metadata': {'type': 'Object', 'value': {'key': 'value', 'nested': {'deep': True}}}, 'id': 'M2bXYIA1JLzj_XQOkVarsxpU1w3mrEuZKTBy7jt6p2E-bJE1dGX2MdrqTFuaZM0PSdohpAGhcnshHPa01eA6uQ', 'categories': {'value': ['electronics', 'computers'], 'type': 'Array'}, 'embedding': {'type': 'Array', 'value': [0.1, 0.2, 0.3, 0.4, 0.5]}, 'tags': {'value': ['tag1', 'tag2', 'tag3'], 'type': 'Array'}, 'user_id': {'type': 'String', 'value': '550e8400-e29b-41d4-a716-446655440000'}}

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
Collection created with first record: "5FQm46vMFAm8OKrHXHOACQOBP28JcH6GLIcI6Vwzgn0FtLR-ouX0_AGxz6fsZv3Erj_zQQj1VAsVavQDNLhxRw"

=== List Collections ===
Total collections: 10
Sample collections: ['batch_users', 'chat_messages__ek0_testing', 'test_collection', 'scripts__ek0_testing', 'websocket_test']

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: False

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: J3U4wAFw2CDWAc5Y7fC5kuVURyMgXMLfXZYcMtfMo313P29yJ_dlYvLHIcGybpwjM3rcXlLiT4G43BT89LrmCA

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: bPQ_VHXK6P3X_Cif9Zg8ub0l-kbvdFF9F9eJhUPn0rAHxNgvahWQsYcHnzhesPmTno14ulpiaxDIKjcJR_-g_w

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
✓ Inserted test record: PUjOqbvpsM_ZBfQqmPdYQRdu4tNCBDXVlYzYIDA2STJMN_QMpE-yBiHnah6FQ61V-_H8q7NUy18gYUQhBiwo2w

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
✓ Inserted document with TTL: CJkeoPPnffAQulPgK2HOos_cZF4yagC1Drpz2dFomVzEZDEy72Ezsuyo23-XjbnlGTJq6uf_chWSQZMfOkXSjQ

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
  1. Score: 25.740, Matched: email, email.value, name.value, name
  2. Score: 12.540, Matched: name.value, name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: bio, title, bio.value, title.value
  2. Score: 26.400, Matched: bio.value, title.value, title, bio
  3. Score: 26.400, Matched: bio.value, title, bio, title.value
  4. Score: 26.400, Matched: title, bio, title.value, bio.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio, title, bio.value, title.value
  2. Score: 39.600, Matched: bio.value, title, bio, title.value
  3. Score: 39.600, Matched: title, title.value, bio, bio.value
  4. Score: 39.600, Matched: title, bio, title.value, bio.value

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
  1. Score: 0.781
  2. Score: 0.753
  3. Score: 0.737

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.716, Matched: title, content.value, content, title.value
  2. Score: 1.147, Matched: content, content.value, title, title.value
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
✓ Created session: rWDdGI5jTi6Tm3bG4At0VPgu8m0mVWOpMz-31zOK6dkYgequ9swcbxoeAfq68y86Op-7mF-ee8-n4vDcw5fGCA

=== Sending Chat Message ===
Message ID: zFE6iw3UwvD7X50oxScmIKjGYsW-rxunC3iTGcYshk7jN-JG2Zw_LIU5AylL6wUy6qcv5I99s0eGnG1JJKhT9w

=== AI Response ===
The products available are:

1. ekoDB Cloud: This is a fully managed cloud database service product. The price of this product is $499.

2. ekoDB: This is a high-performance database product with AI capabilities. The price of this product is $99.

3. ekoDB Pro: This is an enterprise edition product with advanced features. The price of this product is $299.

Execution Time: 2700ms

=== Token Usage ===
Prompt tokens: 606
Completion tokens: 83
Total tokens: 689

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: BGziR8-8gkMr75-xFjh9ze6J3YD333YrHqF2Rj1S2EGv9Nmw4wg9zAVCZqH3ftp7Snp1WZ2xdzgRPwCwswUE_g

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the available data, one product option is the "ekoDB", which is a high-performance database product. The price for it is $99.

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: XnsJ8gn5nhWAWWMWu7OE_aXjbkDw0Sr6kY8AJa_vYGdsoj5GCHMuAmvgsMaDabHCVO0re21WZ-E0aVouZnnEjg
  Parent: BGziR8-8gkMr75-xFjh9ze6J3YD333YrHqF2Rj1S2EGv9Nmw4wg9zAVCZqH3ftp7Snp1WZ2xdzgRPwCwswUE_g

=== Listing Sessions ===
✓ Found 6 sessions
  Session 1: XnsJ8gn5nhWAWWMWu7OE_aXjbkDw0Sr6kY8AJa_vYGdsoj5GCHMuAmvgsMaDabHCVO0re21WZ-E0aVouZnnEjg (Untitled)
  Session 2: BGziR8-8gkMr75-xFjh9ze6J3YD333YrHqF2Rj1S2EGv9Nmw4wg9zAVCZqH3ftp7Snp1WZ2xdzgRPwCwswUE_g (Untitled)
  Session 3: rWDdGI5jTi6Tm3bG4At0VPgu8m0mVWOpMz-31zOK6dkYgequ9swcbxoeAfq68y86Op-7mF-ee8-n4vDcw5fGCA (Untitled)
  Session 4: ckT-vsSlrRFV67BvKFS30PxVBECc1qDvSkEeAZIHWGmSDdCBnk96BxjiLq-EJhaQMWUZQ-4ePmY9X4BckHHBMw (Untitled)
  Session 5: xi8NGvCFxkY8run1C7czBm8Zusrp3ulWQll5bARwCYBvRo77ERcVKgiJZg_PaNTblnAnCPp4cNn3XbX42Fahfw (Untitled)
  Session 6: AtwBzakJFSuDzeoN5C86fY2ZlXZBQj1kjD1Tjxl0iHObhCP8EL4aRdenEGal4OwDAUBIP79dvRi523bjUx4vTA (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: XnsJ8gn5nhWAWWMWu7OE_aXjbkDw0Sr6kY8AJa_vYGdsoj5GCHMuAmvgsMaDabHCVO0re21WZ-E0aVouZnnEjg

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: r1RhDCNcLRqZ7Lvp5PBLdubZHGV_ARKALNwmG6kJKvkjmCWXaL-Qp_uKM_gSewu22Y1N2C2lMqc9pHASvkJ7wQ

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information available, one product that is available is ekoDB. It's a high-performance database product and it's priced at $99.

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['created_at', 'updated_at', 'content', 'token_usage', 'chat_id', 'context_snippets', 'role', 'id'])
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
✓ Created second session: hIhtRI8S4wACiDzJQuUxJAcNLdKhXI3vCzy7Zx2uvCBYmr-lHtb8oxO_AQ4pU8uXdwKlEqOb0Gt7PX7MWaCmww
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

✅ Script saved: F4TwKiI3ezE0EHHY1IN4qkedxWsUThEceQWQwZuCEGIdZoxV5ENy455wt5dRBUPvAmEDCbirGJtDBX2Tl6Rhjw
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved: WF4OQ1VbNz8TLgnyVRrwFHsyRrcTQfxkvXEU9jPSxPXhGV6EQpHbslLwtvNN381uj0Hxk6cVsvfEqe0Kemuw5A
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: 2QCEPbK_BBBq-c3gUAL1MUurejHnLial8CiK-0RME8VSbVJhk7JiDSplj1-I7q77rrn2FAI9-TXlcELyLOFZ5Q
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

✅ Script saved: 0wpgM1aqiN03KerbfiKBXthAEPb5NyZ76fkL0gO90Y70fgUUzJbNvEgPQVykzdLHy-cN0my9x2Q8jtvyaXjhtg
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
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: EeuBPIbSRgPp_HbJ4jFqiBMtPLtcO3HM63RX_b9-pkrHFim-bCXTvqH5z6GvIRwrI-uGNi7IoFFog4iEEvuX0A
Created Bob: $500 - ID: UWst16YOTDJniCVKmllwWUOyhkL9GoGwlfY8daf8E6X-wGnrGna0-Zk-tISIOEIwydL9mMS4U_x2HZqmR8zgmQ

=== Example 1: Begin Transaction ===
Transaction ID: 7aecde4e-7ebb-413c-ae0f-609fa112e676

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: a868a556-086d-405e-89e6-ec60edb44d4a
Status before rollback: Active
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
✅ [32mPython client examples complete![0m
