make test-examples-python
🧪 [36mRunning Python examples (direct HTTP/WebSocket)...[0m
✓ Authentication successful

=== Insert Document ===
Inserted: {'id': 'P4wqUi_AsPzgEpyJewpExzED_QVWKoEJv0t6Tr5-VTeQ2c9S5TRAMXVSWBCD5pXd4EJhBILy2mikJ_3bjXXK1w'}

=== Find by ID ===
Found: {'value': {'type': 'Integer', 'value': 42}, 'id': 'P4wqUi_AsPzgEpyJewpExzED_QVWKoEJv0t6Tr5-VTeQ2c9S5TRAMXVSWBCD5pXd4EJhBILy2mikJ_3bjXXK1w', 'active': {'value': True, 'type': 'Boolean'}, 'name': {'value': 'Test Record', 'type': 'String'}}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'value': {'value': 100, 'type': 'Integer'}, 'name': {'value': 'Updated Record', 'type': 'String'}, 'id': 'P4wqUi_AsPzgEpyJewpExzED_QVWKoEJv0t6Tr5-VTeQ2c9S5TRAMXVSWBCD5pXd4EJhBILy2mikJ_3bjXXK1w', 'active': {'type': 'Boolean', 'value': True}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: nnVOAK1UCMqtLlx1RjzLLmk48bJGDlOuJzKQHSasOE0HvHVCpuW8oQjNREhaixWi6DELTIQRjGxUr40Mz-SVAQ

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
        "id": "ZZP8j_9ycdafEWgY6Gv4o5YVw02nFkt05huGnFkyCMirV19KgWpN2J4AepMNXW6mBJu2DEx-dtNYUnc9a7rKIw",
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
        "id": "nnVOAK1UCMqtLlx1RjzLLmk48bJGDlOuJzKQHSasOE0HvHVCpuW8oQjNREhaixWi6DELTIQRjGxUr40Mz-SVAQ",
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
cache:product:3: {'price': 49.989999999999995, 'name': 'Product 3'}

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: rhchePLEu0FDfSFYqMvSqUrWmch2tS_C5C6ePw1bCugBopSbz1JN3R4a_qZ1xiF-cqIztdt05A-JDkTCq2EXqA

=== List Collections ===
Total collections: 11
Sample collections: ['ttl_cache', 'users', 'batch_users', 'test_collection', 'ws_ttl_test']

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: False

✓ All collection management operations completed successfully
✓ Authentication successful

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: nwUFcHLEHvuA2s6fkTSq0tbraj0I_R8TUd_Uq-EtVRDJzGNP0ajjFhF48-6MHqPOt85rjYMdcmZC2hLj7VnZvw

=== Insert Document with TTL (5 minutes - integer) ===
✓ Inserted document: created

=== Insert Document with TTL (30 minutes - duration string) ===
✓ Inserted document with duration string TTL: AFfbBc2y7JZoUY_cP6MZUmqQ1llBABxXeQK3feJxjUJZVkSmbj6lwayEH6RcvzCEjyxNcoMhzmfcywsTqinmIQ

=== Query Documents ===
✓ Found 5 documents with TTL

=== Update Document ===
✓ Updated document

=== Delete Document ===
✓ Deleted document

✓ All document TTL operations completed successfully
✓ Authentication successful

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: 5NBPIYMii5phJIZMgnBd0slcsnTqAcCoQxuITEwYiWx3C4SEJQ-VjuhhU6fWtdDtwS9_nnUCLSNiws-_uTt09Q

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 2 record(s) via WebSocket

✓ WebSocket TTL example completed successfully
🚀 ekoDB Scripts Example (Python/HTTP)

✓ Authentication successful

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: FblFlc-X-T8TNEFA5_iq2sMHN1JUC9VU3q51_Or1QJi8dWlf-6WFRS45T0iShXXiH1M-pf-XmubTUVLyT1hlfg
📊 Found 30 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: 8_a4qMnaR_TfenHzc9_PZUUnEb1OF7tue0G__gTnoL-IXmtM2p2trh2WETZR3PZDReiHldXqbpWH3_aXwr5SlA
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: sWgEvv7DLjC4NYElcJxMiFTZdVTHGrUJ_vYWr53RzxGgXLbvaFzW0FnYggj6svqOJmhqjezmNESbfQRGgBSNtw
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {'count': 15, 'avg_score': 60.0, 'status': 'active', 'max_score': 100}
   {'status': 'inactive', 'max_score': 90, 'avg_score': 50.0, 'count': 15}

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
Created Alice: $1000 - ID: f6Vfe0BL8j_6Yxbr7HNHqbGD9ihjcZYiMSJKoV9zUNvwkXNUI5dv72wlLV4bvKoVV-vEN1dQ8H5szvuGfUPBSg
Created Bob: $500 - ID: YWvX_DPIwC5OB5Bwwkn4emitjah9LhmA3cmSevMTtR_pVN7O7_8wzPh-2BsjZz-aRU0G5yZ2ElL6kT_HmrKnPA

=== Example 1: Begin Transaction ===
Transaction ID: b90f42d4-417e-4c98-bbad-f636b8b587a6

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
New transaction: 1c4c03d9-a0c2-4203-ba7f-ff11ec198dad
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: P9HSKIhSmZzs8hJYYlQG9xQjogaP3KTqb_9BRNL8GHsJp-gc8LMMaFCTDmDtZrU6n0xnx1H5lzCq9FDPa6N_7A
Created Bob: $500 - ID: tdbZhtZx1DrXZ_S9D6ivbnLKD9bNxmmLmmTB6dOnRUKs1Cn2BneIzq5BflGXzDTGYdW-opTCKuX9hgCmiuQQhg

=== Example 1: Begin Transaction ===
Transaction ID: bc55a1f4-86ae-49d6-9044-935603c0b729

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
Created Alice: $1000 - ID: Ox3ad5RJ4YcF-ZfkeUenwOBfgZacyJoZX2JvyCwM11Y36A2IFIvZ5FiFutoAyGwFUQF0L1_yoUyzFfTXbj9mHA
Created Bob: $500 - ID: iVQKw7MU0URWoN0lwND9noNnLkcRvZP7TUoMNCjHsA64pDqM2lVFF7Ey4jKDFLJC7P6uNx5PZtuQpMfPtwde6g

=== Example 1: Begin Transaction ===
Transaction ID: 513ccac0-78ba-4d64-9029-3d91afcfc35e

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
New transaction: fed6399f-ba37-4d07-88c5-86aa03a563d8
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: j8hxfFPASxyJlhW9E4mhgL30Q90fPHxKP-m01178vo4WCTDK-8Cr6TleH1kEok44PFqeMsOR-aqrKzsS310vNQ
Created Bob: $500 - ID: nu34P_bvuhZC1L5TzB-GQ47Vkdq3TSPKClI2FcbLfjy6oM3eWCvXcUo4BFW_iwXIM2OiFoUviQ7poRqQQ7a6Aw

=== Example 1: Begin Transaction ===
Transaction ID: 5b5e4428-b483-433e-be7a-f26f5a90d7c5

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
Created Alice: $1000 - ID: WguqB1NObZNWcF7q9prJi2-UAeY1lZGCe1AFsX-30YFkvMrb8TL6VdRviiQUFtTtNBVGoy-amLeCE_p-Tzb1qg
Created Bob: $500 - ID: uT7zhke10sh_-T0Bt2WUbSBsW3VgiLvagVsXtzzrrErAUlmzcuZsvgbMEgDL0TPWuEyzoK6AHTfS3sUB6B4ICw

=== Example 1: Begin Transaction ===
Transaction ID: 9c9fc946-f778-4a36-9393-8b5d43c59c1f

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
New transaction: 97eaa6e1-1025-46be-ac26-d2e04edc12ac
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: VrAaJwpIVu0BYfCcun8ZKuwrAOTboRxGW7uPRo-AIGq1KQj-3jLRXirsG2ADBQFS0ZSdTGtIm-FdPe64cI9mWg
Created Bob: $500 - ID: w0kTQjohHCYFHL2XyRdOl-7gjOWkW9RI6sPCZ23vfsNkx_SPoX82XqoAPNkZB4HgFBlIAYPAd9TtX6wacKKmzA

=== Example 1: Begin Transaction ===
Transaction ID: f3f4a98c-e687-4d70-a2ff-c689687382d2

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
   ✅ Script saved: 2bTEEKQiAfUyyloi9HhBWGFLp6fBGfHUn075CTI8xsS3bFG3rtGNja6LmhtNqFZYPYZQTZ22YJ4Bue9tgiw5tw

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: WnxUv5_SdDKwn1VPHw7395eTQ2yZe0ooz-Jnrzw3E9qTKpXUgVEFSm7yzPyqoH-gwNLJEEjSL5VrHzAwNWJd4g
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}
   📋 Email: {'type': 'String', 'value': 'alice@example.com'}
   📋 Status: {'type': 'String', 'value': 'pending'}
   📋 Credits: {'value': 0, 'type': 'Integer'}

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: ljzH8EgOcjnJOEySKWevxp2SjQO_3p82dII6xX26Rlt-Ak6IOYJtS12u0E1i3pFo9bF4xrgfjqDxSLeT4N7zEw

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
   ✅ Script saved: 5nQ3MF3oz55Tb6PYCypYSPJ4vPTZVKauZMKi8x0ydfD307pwARu4zAZkvELUIaXWGBhr5nKjXd1NHHK2gfmSrw

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
   ✅ Script saved: 20Ied9lwGibJPY4mIEExxe65k4Q186D_ZdwBHmqW5LB2LV3fc7wtOAegTXj5MRHzHEooHE1KchIviylMlphA6A

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: 2bTEEKQiAfUyyloi9HhB...
   ✅ Deleted script: ljzH8EgOcjnJOEySKWev...
   ✅ Deleted script: 5nQ3MF3oz55Tb6PYCypY...
   ✅ Deleted script: 20Ied9lwGibJPY4mIEEx...
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
    Finished `release` profile [optimized] target(s) in 0.07s
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
Inserted: {'id': 'j76Mt64GiWIrjeDh7W8Le1oJH2RMTI4l_tn9jc8P6aQQkohEELcwAY50VcFpFXTIw2lzJ2ga3RqMK_NmfdsLUQ'}

=== Find by ID ===
Found: {'active': {'type': 'Boolean', 'value': True}, 'user_id': {'value': '550e8400-e29b-41d4-a716-446655440000', 'type': 'String'}, 'name': {'type': 'String', 'value': 'Test Record'}, 'price': {'value': 99.99, 'type': 'Float'}, 'tags': {'value': ['tag1', 'tag2', 'tag3'], 'type': 'Array'}, 'value': {'type': 'Integer', 'value': 42}, 'embedding': {'value': [0.1, 0.2, 0.3, 0.4, 0.5], 'type': 'Array'}, 'categories': {'value': ['electronics', 'computers'], 'type': 'Array'}, 'metadata': {'type': 'Object', 'value': {'nested': {'deep': True}, 'key': 'value'}}, 'created_at': {'value': '2026-01-03T19:30:17.621961', 'type': 'String'}, 'id': 'j76Mt64GiWIrjeDh7W8Le1oJH2RMTI4l_tn9jc8P6aQQkohEELcwAY50VcFpFXTIw2lzJ2ga3RqMK_NmfdsLUQ', 'data': {'type': 'String', 'value': 'aGVsbG8gd29ybGQ='}}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): True
  price (Decimal): 99.99
  created_at (DateTime): 2026-01-03 19:30:17.621961
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): ['tag1', 'tag2', 'tag3']
  metadata (Object): {'nested': {'deep': True}, 'key': 'value'}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): ['electronics', 'computers']
  data (Bytes): 11 bytes
Plain record: {'active': True, 'user_id': '550e8400-e29b-41d4-a716-446655440000', 'name': 'Test Record', 'price': 99.99, 'tags': ['tag1', 'tag2', 'tag3'], 'value': 42, 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5], 'categories': ['electronics', 'computers'], 'metadata': {'nested': {'deep': True}, 'key': 'value'}, 'created_at': '2026-01-03T19:30:17.621961', 'id': 'j76Mt64GiWIrjeDh7W8Le1oJH2RMTI4l_tn9jc8P6aQQkohEELcwAY50VcFpFXTIw2lzJ2ga3RqMK_NmfdsLUQ', 'data': 'aGVsbG8gd29ybGQ='}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'tags': {'type': 'Array', 'value': ['tag1', 'tag2', 'tag3']}, 'name': {'type': 'String', 'value': 'Updated Record'}, 'created_at': {'value': '2026-01-03T19:30:17.621961', 'type': 'String'}, 'embedding': {'value': [0.1, 0.2, 0.3, 0.4, 0.5], 'type': 'Array'}, 'price': {'value': 99.99, 'type': 'Float'}, 'categories': {'type': 'Array', 'value': ['electronics', 'computers']}, 'value': {'value': 100, 'type': 'Integer'}, 'metadata': {'value': {'nested': {'deep': True}, 'key': 'value'}, 'type': 'Object'}, 'user_id': {'type': 'String', 'value': '550e8400-e29b-41d4-a716-446655440000'}, 'data': {'type': 'String', 'value': 'aGVsbG8gd29ybGQ='}, 'id': 'j76Mt64GiWIrjeDh7W8Le1oJH2RMTI4l_tn9jc8P6aQQkohEELcwAY50VcFpFXTIw2lzJ2ga3RqMK_NmfdsLUQ', 'active': {'value': True, 'type': 'Boolean'}}

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
Collection created with first record: "eHwv6972BrK9qAGUlIL9iwlpFkeZqDYKu_LUHoC2d_H1QkrhO75J4FtxmCPRdq6m2Kmr6RAHVPvno2TzRlzz1A"

=== List Collections ===
Total collections: 10
Sample collections: ['ttl_cache', 'batch_users', 'test_collection', 'ws_ttl_test', 'websocket_test']

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: False

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: XoWrSyQCouANaePE1qW8jywoctEqPxIkAKz1w2b7EaPUWftgHsMPhe5j4m_fEBgaU9x_0GtAxXJy8JPBv4IyjA

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: IPeV8E9vf0mYH6l7R9oB7ljdwDEcxqWjFktzl_mYn-2BipDPWLwnRjCPHZRQlKxSFnb_WgTwHguNbk49BvEx3A

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
✓ Inserted test record: DxTCExu96QQpK0PyC7IGUCMZjj4CiWkRu9aM9TJ9jwBZcAc0Sj3pRhPrCDq4JDyasKNqDag7RATFpUopkN6nhg

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
✓ Inserted document with TTL: 4Hd3V6miOP-po-pgkQgC1MKjqBDSkd2bddgsqq474F8HtmYh2mu_VjEHm5-9q_hKz7udg0E4wMInW8d6VljLpw

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
  1. Score: 25.740, Matched: email, name, name.value, email.value
  2. Score: 12.540, Matched: name.value, name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title, bio.value, title.value, bio
  2. Score: 26.400, Matched: bio, title, bio.value, title.value
  3. Score: 26.400, Matched: title.value, bio.value, title, bio
  4. Score: 26.400, Matched: bio, title, title.value, bio.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title, title.value, bio, bio.value
  2. Score: 39.600, Matched: title, bio.value, bio, title.value
  3. Score: 39.600, Matched: title.value, bio.value, title, bio
  4. Score: 39.600, Matched: title, bio.value, title.value, bio

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 2 results (matches: run, running, runs)
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio.value, bio

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.734
  2. Score: 0.729
  3. Score: 0.711

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.714, Matched: title.value, content, content.value, title
  2. Score: 1.098, Matched: title.value, title, content, content.value
  3. Score: 0.510, Matched: 

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
✓ Created session: 9_Jjc959t0oHyhGBza3R3NQbjHYLRTpswpfQR5l9OKH7tCF_LGFPkB1rbqnKFjS45RdDuv67_0nCrE_yC0eGJw

=== Sending Chat Message ===
Message ID: jKqYZJke_Ed-TsKEp-zBKGqPhPenpoDhATwzexNPFT3R1mbs247moHctmHAOZngRgsbvO-KoHtmvYE5KKVVQWA

=== AI Response ===
The available products are:

1. ekoDB: This is a high-performance database product with AI capabilities. The price of ekoDB is $99.

2. ekoDB Cloud: This is a fully managed cloud database service product. The price of ekoDB Cloud is $499.

3. ekoDB Pro: This is an enterprise edition product with advanced features. The price of ekoDB Pro is $299.

Execution Time: 3103ms

=== Token Usage ===
Prompt tokens: 612
Completion tokens: 88
Total tokens: 700

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: otL5_twubZAn9rGrc4bEFnd5xkY4aS2V7VeuBxE3pivjxf5HDsQBPB-gbnqrtwHNe0x-K4V_bsQIlymnQFDwSw

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information provided, the available product is "ekoDB". It is a high-performance database product.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: Chl4OK33kleGsSflEJxmBI5_DLpelyZCBi33ztY2Wz08hoQ7ccBn_nvOwAWlADM9t2dH8m4v-Io-9zLJzQd77w
  Parent: otL5_twubZAn9rGrc4bEFnd5xkY4aS2V7VeuBxE3pivjxf5HDsQBPB-gbnqrtwHNe0x-K4V_bsQIlymnQFDwSw

=== Listing Sessions ===
✓ Found 6 sessions
  Session 1: Chl4OK33kleGsSflEJxmBI5_DLpelyZCBi33ztY2Wz08hoQ7ccBn_nvOwAWlADM9t2dH8m4v-Io-9zLJzQd77w (Untitled)
  Session 2: otL5_twubZAn9rGrc4bEFnd5xkY4aS2V7VeuBxE3pivjxf5HDsQBPB-gbnqrtwHNe0x-K4V_bsQIlymnQFDwSw (Untitled)
  Session 3: 9_Jjc959t0oHyhGBza3R3NQbjHYLRTpswpfQR5l9OKH7tCF_LGFPkB1rbqnKFjS45RdDuv67_0nCrE_yC0eGJw (Untitled)
  Session 4: nG-we3gQtp1gSDq6fq6xMPlJ-4gSKMIesqqHvUqGbz21aNYku8Q_W2jLmV6iL1vTODEep_b5rEryHUaH2yHB-g (Untitled)
  Session 5: fabuYeYu50GbJZH1jgr2QWukNk_sPLaArrv4BWAEY-PAj_WKkMnZMeNrYn43vbRJcrsUksRWuKDz013FF4LMuA (Untitled)
  Session 6: iqrOsUYHuTFlzfksowEe8ciD6bzA2e94cOOZ7GyKEsD15VCgG1hxJsYOpdB_3OLEg2L7-kw21Vk-ZKuf-Et5zg (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: Chl4OK33kleGsSflEJxmBI5_DLpelyZCBi33ztY2Wz08hoQ7ccBn_nvOwAWlADM9t2dH8m4v-Io-9zLJzQd77w

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: QW2HnsjPLqfa7zd0mJFPAsRFn27VGP_Dbx6lIufNQT4MLh8_HDF-504gu0s1a5iinXUIBRKNHwMAjgXdnWdeuw

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information available, one product that is available is ekoDB. This is a high-performance database product and is priced at $99.

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['created_at', 'chat_id', 'updated_at', 'id', 'content', 'role', 'context_snippets', 'token_usage'])
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
✓ Created second session: 6V4Pa6UrFZn93O2rj4Xl2oPWQBRhE5NdBj-mVQPTiEg9jLGGJBmAYi4aMsrNyEos-WdgVlcHz9orGRnSM0okww
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

✅ Script saved: mhm4JPU9RG0sKYn2u5MMXaSaYyMFWq7dEQHNfioGpqZ6BDZxbpuEJYGbYnz1-i3ItABnlRWtBNsKQ0epdgMVEQ
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved: ITm5Ba9u5AxDH5YSxPnK3Z5aGSPw-u-qln8fVff0h5np0jDMpqUR5KKfRJ6d0LHtJGNdZDt5yAoQG8KZsrfoiw
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: JpaIwWM7wpBCbCi7hevfKIZq14H-kJ3Eb6JkyFkzJnVxEA3lRvb0VGQhLfusu-FzwvALXkqIxilwIvak3F5X9w
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

✅ Script saved: gkVsYA_DyZnkncaQhx5RGsyrfHVgMAE43Qk4QAtecVPIhpz_YWUrb-cjllzUwqQaCpCvdI2dHrEy9rhDYblOcQ
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
Created Alice: $1000 - ID: ua_4w6YnsXmask4jfMqJQLkUyHvILTF9YjETb6_Hz8tJLC_iq5S96xq4qPEwoPtaBVyMXE2Kt0lVulXDnkiClQ
Created Bob: $500 - ID: rchCEwJcRBCE7n7A16ISUg_6QBGjOiBWFWN5pVjnHshddT7YJC62m8j-skY6fpqJCY5eq1id_FaTI3ht8l5kmg

=== Example 1: Begin Transaction ===
Transaction ID: 85fec671-5c86-4c90-a713-982c749fbd5e

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 21f52534-7b42-4891-b118-e0b4b41d57c7
Status before rollback: Active
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
✅ [32mPython client examples complete![0m
