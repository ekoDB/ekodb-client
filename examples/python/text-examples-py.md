make test-examples-python
🧪 [36mRunning Python examples (direct HTTP/WebSocket)...[0m
✓ Authentication successful

=== Insert Document ===
Inserted: {'id': 'Qw7V8C3puicAhL859VODekhDfuGDe4iYaagz5690hIgi35MHrGkC_sIT4slUbkCreI4a3UZMSbedxxj1oj5HhQ'}

=== Find by ID ===
Found: {'id': 'Qw7V8C3puicAhL859VODekhDfuGDe4iYaagz5690hIgi35MHrGkC_sIT4slUbkCreI4a3UZMSbedxxj1oj5HhQ', 'value': {'type': 'Integer', 'value': 42}, 'name': {'type': 'String', 'value': 'Test Record'}, 'active': {'type': 'Boolean', 'value': True}}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'id': 'Qw7V8C3puicAhL859VODekhDfuGDe4iYaagz5690hIgi35MHrGkC_sIT4slUbkCreI4a3UZMSbedxxj1oj5HhQ', 'value': {'type': 'Integer', 'value': 100}, 'name': {'value': 'Updated Record', 'type': 'String'}, 'active': {'type': 'Boolean', 'value': True}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: 9jhiJD-vX2ICyYsmVzgVmtlyo9yTqcOWZTdsd5jfBvDaYIiuAKy4_bD3ASkcLX_E2gV2whXZrWIPWIfwLrXQAQ

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
        "id": "9jhiJD-vX2ICyYsmVzgVmtlyo9yTqcOWZTdsd5jfBvDaYIiuAKy4_bD3ASkcLX_E2gV2whXZrWIPWIfwLrXQAQ",
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
        "id": "LBF7Gequi0v04qbT1iANWTN-FRO0bYDKvukFbxdX9CO7wYKIchKcISe0P-UObFrpY2NNO2nXY-XcPKH9NW1CkA",
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
Collection created with first record: jlpU3PK9S-LrSSpzG2p70P9NipbkyxQCQnWg0UtNnVP5DqmSC_h4htCyHQxkvxecdEeLMmAB8Y9yj1UlCOjOlg

=== List Collections ===
Total collections: 11
Sample collections: ['ttl_cache', 'test_accounts', 'ws_ttl_test', 'websocket_test', 'chat_messages__ek0_testing']

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: False

✓ All collection management operations completed successfully
✓ Authentication successful

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: yRYLo9XNwSTAsFA-NASApGBGuTjfZw9wpFFE_KaVHwC7PO3UCVq6h-iVVcF7l7Kr_PQ8Gy7R2k-JvyWEwB1TaQ

=== Insert Document with TTL (5 minutes - integer) ===
✓ Inserted document: created

=== Insert Document with TTL (30 minutes - duration string) ===
✓ Inserted document with duration string TTL: 2istEHazK4dUehzRj8Gq-VOsA65e9U2xmuzuGAZPkPnAoic0RheKhjByKmktu77c1eOw749-aj19zekLPPRohQ

=== Query Documents ===
✓ Found 5 documents with TTL

=== Update Document ===
✓ Updated document

=== Delete Document ===
✓ Deleted document

✓ All document TTL operations completed successfully
✓ Authentication successful

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: kWbmlTMV66UXAgSqP3-Danmit2RBVG_b_XGVEceodiqAq5SI5CjUmMJhaM9XZmadtiIXf_xfEon7DkMjVObXCg

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 2 record(s) via WebSocket

✓ WebSocket TTL example completed successfully
🚀 ekoDB Scripts Example (Python/HTTP)

✓ Authentication successful

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: -c23XrEgV1mhlw-xDD95pagbMK6zqKaj0xgsLrV__eNjPiEhX_c_9Gnn0E0m0K59mKGH12cXTny6NKWqLgI9fg
📊 Found 30 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: s50egmjXQFhPOHbdSV4Myk0uwGc7WiSUF3h7FA7-2UOoDNC6-86MVVJWR1V7DicFa42WaznAX8VmiOnNklHjzg
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: MYnYh925i_zsVDMPHHoAP0SBdyXt1-Oc1UAQNKcFz7f7pwwn9hEOqmIhT7lZpIkpnUVnAaPVYy9cEODLtk8MMA
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {'count': 15, 'status': 'active', 'max_score': 100, 'avg_score': 60.0}
   {'avg_score': 50.0, 'status': 'inactive', 'count': 15, 'max_score': 90}

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
Created Alice: $1000 - ID: kK1rT2OFs_wzJf2jvZbzD7C1HO3hPPHD3FsVm5aR8spqyA_gYPvx8KhWmRVJJxzo-kO4bcoBJrgLva68WzCcWA
Created Bob: $500 - ID: Xjkz4YfczIq9ZtjlstzcX6ArRXJpfqZZQfzhSU-UrmS2shKnC-h9QNGBaqfI_NQB7LPVi3SoaQ8vz3SPkuAzHg

=== Example 1: Begin Transaction ===
Transaction ID: b9f91497-34bd-49de-a067-61a3a6e7aeae

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
New transaction: 4381001d-6d82-4609-bb4b-d5766696b41d
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: HOT1LKb-cl6cSbVU2d5ABR6FPGJwKC_MXcDsxdnpwgschi9DTP08-2c_LKOn_oZsEWOkLZvxNRcCWo6WBhDeww
Created Bob: $500 - ID: 68tfarcjTK2QfBwc9sPISxRFCL6mZ5yivQ22yMzhsMW7T9o71ee39rngBSRTkIAtYGTX4VK6WIUQFhMrf3F6zg

=== Example 1: Begin Transaction ===
Transaction ID: 903f9431-01f5-4e37-832e-2755dccaba5b

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
Created Alice: $1000 - ID: _7bOTSXf5BJSlSvCNvJSVm_YtljFgZa6VQqfpCzJct65eb5ZcZvAJpU98EZl9D3g1EjS9Jtc8yWGxWXiBu4nWA
Created Bob: $500 - ID: vowbgXCWfqpMg77sBFuHl8qUB37SaO6mwyP_9gdoe8cp-WKfuHcHIL3sEw6ICCeOtvk29BtnQAEDaNWhLGIiWQ

=== Example 1: Begin Transaction ===
Transaction ID: 726339ca-0687-4e77-9ca2-99dc1710a898

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
New transaction: 8d1ef96e-5b13-4018-9588-3cf4eb1fad8b
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: g0YDEHGG-c5xeCfeykui502rTfolqjjmVZcvTlE3wWtuo5DuCQB0QPTzSpa3HvfRvrne7MCCxd_cJYfl_S9YLw
Created Bob: $500 - ID: tD3l4mglYCsfYENhkfuigVlhnP4bKiWTIo_qpncWc8MpuvTMuMPlhEowr_GJxtDZWfgyKZ-kC2yhRaG4NqtLqQ

=== Example 1: Begin Transaction ===
Transaction ID: cb1bcce2-fd6d-4759-8a4f-c03aa22d3a0c

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
Created Alice: $1000 - ID: MsXTP7EQWW9QpGmRGaRJi-B5wAJbjVyaXkuWpzSu7SYtAN7qiqEHaTRJN_07vCl-a89y2dmt8RVC6b8TilOw8g
Created Bob: $500 - ID: 2KgL3heHxXZc6xBR-A-GGDKc18gMaLmrycDJTl31dCT1ZXd1KL67oyt5tYeWJx9BmBdBODVQBgGDM7wNf3m1xQ

=== Example 1: Begin Transaction ===
Transaction ID: 14a7d149-2b36-4e1f-ab1e-cc1877fdb6ac

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
New transaction: e1d5a47d-8ebe-462c-b91e-ed428ec10dc9
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: RmXhO8z-yeqd6uNeoA8jVj1LhrenA69HZrSibuvlasV8Sog0aVDvTtfgfhVhcEzrEjN0n_BZfCBEp53SgI7Dng
Created Bob: $500 - ID: cOOAsE11NWlelZFMOJ7B6vJPckMTrd6UFUGO1UyfA1fFXJQQ53TrDStb1bvoojWoGCc0g2J7SoClm-Coz60iVw

=== Example 1: Begin Transaction ===
Transaction ID: 3c863c5d-1173-459c-865f-63ac90282dd7

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
   ✅ Script saved: RtMk-LB4lTL-dBnaPdVNXWGH5z4av1ufyMfzuQ06ccS4PlIDqN_8bk5MxEC5_PpzA82WNLSAEDPXcKfIuc2bWg

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: vsQkKmEe4csJWmmcZgV2SkUmm8c7UGOEaVlLnyI0RLB8fHvf5LUzR48dz13XiP7s2YE_cRr_3jZf8DeW5o8ocQ
   📋 Name: {'value': 'Alice Smith', 'type': 'String'}
   📋 Email: {'type': 'String', 'value': 'alice@example.com'}
   📋 Status: {'type': 'String', 'value': 'pending'}
   📋 Credits: {'value': 0, 'type': 'Integer'}

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: PLr_jj23SzhriG_t4194s0YyBaNE9V8oNtwGSaVJjPcxaRFWkKrQ-TllaT7rq3JIKMWXjlJNLLGL7AbaYA6InQ

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
   ✅ Script saved: Z4kICiKSWZk1keJNdfJfOCHQFgyZZ5B_fjily-_7X-GjVC0wPoAlwkNGHZhmOF3JuKReK9YDjWemx_kxS7xxbw

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
   ✅ Script saved: kzaA9ouBUpXp1faMi3vvuZpctajwuhbteUMiGkcXWSsruXI7ly1JVQrhhn6nITsulmFrctxL8u1RB0VYCBxxcw

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: RtMk-LB4lTL-dBnaPdVN...
   ✅ Deleted script: PLr_jj23SzhriG_t4194...
   ✅ Deleted script: Z4kICiKSWZk1keJNdfJf...
   ✅ Deleted script: kzaA9ouBUpXp1faMi3vv...
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
Inserted: {'id': '22dYwAeXMG1qIyY_qBOXiyTtDUHLKWzdQntaORsamupMAZYxcpG_2z9WWk0SXp8ERJctaLFOY2BYqGH7SNcewA'}

=== Find by ID ===
Found: {'id': '22dYwAeXMG1qIyY_qBOXiyTtDUHLKWzdQntaORsamupMAZYxcpG_2z9WWk0SXp8ERJctaLFOY2BYqGH7SNcewA', 'data': {'type': 'String', 'value': 'aGVsbG8gd29ybGQ='}, 'tags': {'type': 'Array', 'value': ['tag1', 'tag2', 'tag3']}, 'created_at': {'value': '2026-01-04T14:37:11.642256', 'type': 'String'}, 'name': {'value': 'Test Record', 'type': 'String'}, 'embedding': {'value': [0.1, 0.2, 0.3, 0.4, 0.5], 'type': 'Array'}, 'active': {'value': True, 'type': 'Boolean'}, 'categories': {'type': 'Array', 'value': ['electronics', 'computers']}, 'value': {'type': 'Integer', 'value': 42}, 'metadata': {'type': 'Object', 'value': {'nested': {'deep': True}, 'key': 'value'}}, 'user_id': {'type': 'String', 'value': '550e8400-e29b-41d4-a716-446655440000'}, 'price': {'type': 'Float', 'value': 99.99}}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): True
  price (Decimal): 99.99
  created_at (DateTime): 2026-01-04 14:37:11.642256
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): ['tag1', 'tag2', 'tag3']
  metadata (Object): {'nested': {'deep': True}, 'key': 'value'}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): ['electronics', 'computers']
  data (Bytes): 11 bytes
Plain record: {'id': '22dYwAeXMG1qIyY_qBOXiyTtDUHLKWzdQntaORsamupMAZYxcpG_2z9WWk0SXp8ERJctaLFOY2BYqGH7SNcewA', 'data': 'aGVsbG8gd29ybGQ=', 'tags': ['tag1', 'tag2', 'tag3'], 'created_at': '2026-01-04T14:37:11.642256', 'name': 'Test Record', 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5], 'active': True, 'categories': ['electronics', 'computers'], 'value': 42, 'metadata': {'nested': {'deep': True}, 'key': 'value'}, 'user_id': '550e8400-e29b-41d4-a716-446655440000', 'price': 99.99}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'price': {'type': 'Float', 'value': 99.99}, 'value': {'type': 'Integer', 'value': 100}, 'name': {'value': 'Updated Record', 'type': 'String'}, 'tags': {'value': ['tag1', 'tag2', 'tag3'], 'type': 'Array'}, 'embedding': {'type': 'Array', 'value': [0.1, 0.2, 0.3, 0.4, 0.5]}, 'active': {'value': True, 'type': 'Boolean'}, 'id': '22dYwAeXMG1qIyY_qBOXiyTtDUHLKWzdQntaORsamupMAZYxcpG_2z9WWk0SXp8ERJctaLFOY2BYqGH7SNcewA', 'categories': {'type': 'Array', 'value': ['electronics', 'computers']}, 'created_at': {'type': 'String', 'value': '2026-01-04T14:37:11.642256'}, 'user_id': {'value': '550e8400-e29b-41d4-a716-446655440000', 'type': 'String'}, 'metadata': {'type': 'Object', 'value': {'nested': {'deep': True}, 'key': 'value'}}, 'data': {'value': 'aGVsbG8gd29ybGQ=', 'type': 'String'}}

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
Collection created with first record: "9ow_P5LGIOhszGkNPbsHi3kIKdSFitAJTgu5X9rhV0gy69of6WuVY_AJUIl5KZX_dzsSW9qtWGvtXvq0Es2uCA"

=== List Collections ===
Total collections: 10
Sample collections: ['ttl_cache', 'test_accounts', 'ws_ttl_test', 'websocket_test', 'chat_messages__ek0_testing']

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: False

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: raY9gHm5AROuIiSV6fjREopQASc35wH5Tc4-gN4tVUqJybKfHjd_4bq-eYp4vVfL_70h0ZijxuWk_95Ylw3OJw

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: CzDYbtmkV6cfnSuSt4o4W9Ekd9Y77XV_YC06fsFzN6tfIswPXtvs3AvBtGEJKi0Q4xFgJSfSezg6OuAHXEXFxw

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
✓ Inserted test record: UOE0vMi-b_2ZhktT_IyDRMUqcra5M_YHDmTDVrY5YyFa_-k9V5sMs4dMeeYOZQv6KYhlwDwqqGVqgr2V4UjLIw

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
✓ Inserted document with TTL: FSngzkMdaKufGQyF3owRO2xB5aadZI84s52Flh9t_UWZgAwgBO0AAWrqDe-2kmK5iO_gA_GUJLCkIpwqnlRKwA

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
  1. Score: 25.740, Matched: email.value, name.value, email, name
  2. Score: 12.540, Matched: name.value, name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: bio, title, title.value, bio.value
  2. Score: 26.400, Matched: title.value, title, bio, bio.value
  3. Score: 26.400, Matched: bio.value, title, title.value, bio
  4. Score: 26.400, Matched: title, title.value, bio, bio.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio, title, title.value, bio.value
  2. Score: 39.600, Matched: title.value, bio, title, bio.value
  3. Score: 39.600, Matched: title, bio, bio.value, title.value
  4. Score: 39.600, Matched: title, bio, bio.value, title.value

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
  1. Score: 0.755
  2. Score: 0.751
  3. Score: 0.737

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.726, Matched: content.value, content, title, title.value
  2. Score: 1.116, Matched: content, title, content.value, title.value
  3. Score: 0.529, Matched: 

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
✓ Created session: 75BwxYE7jLzRVr74SLD1UtuvPgDxlgpFIhcnXtSRADrscb60rK4_7_n7mx6Ip0Q5uiS2ZECQcBR6Z8uuZumVcQ

=== Sending Chat Message ===
Message ID: EZFyBh5_S9qnyecVa-LSmKpQXnVSXnEjDTJc_BFtWGSZHQHOT7IebgSsBOg9Wq08DUG2G8rLwvMUKzwXyWlEgQ

=== AI Response ===
We have three products available:

1. ekoDB Pro: This is an enterprise edition product with advanced features. The price for ekoDB Pro is $299.

2. ekoDB: This is a high-performance database product with AI capabilities. The price for ekoDB is $99.

3. ekoDB Cloud: This is a fully managed cloud database service product. The price for ekoDB Cloud is $499.

Execution Time: 2883ms

=== Token Usage ===
Prompt tokens: 611
Completion tokens: 89
Total tokens: 700

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: rViBhRDXhRbwLJ7q7HgIWAR-WzJpWkLXUnD6trKYXIrKNfa6L9gDBL9gBxUojcl3yMP0zwN4IH5XVgtxHLKaXw

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is "ekoDB", which is a high-performance database product. It is priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB database product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: ENZUc6J-ov9VDsYSxvtvbEWPu8mGL6CGnt8kN7HHlPpfaEyCLyttskpA-G9opIYiEBF5s_0NpuOKOCnQH7ZyhQ
  Parent: rViBhRDXhRbwLJ7q7HgIWAR-WzJpWkLXUnD6trKYXIrKNfa6L9gDBL9gBxUojcl3yMP0zwN4IH5XVgtxHLKaXw

=== Listing Sessions ===
✓ Found 6 sessions
  Session 1: ENZUc6J-ov9VDsYSxvtvbEWPu8mGL6CGnt8kN7HHlPpfaEyCLyttskpA-G9opIYiEBF5s_0NpuOKOCnQH7ZyhQ (Untitled)
  Session 2: rViBhRDXhRbwLJ7q7HgIWAR-WzJpWkLXUnD6trKYXIrKNfa6L9gDBL9gBxUojcl3yMP0zwN4IH5XVgtxHLKaXw (Untitled)
  Session 3: 75BwxYE7jLzRVr74SLD1UtuvPgDxlgpFIhcnXtSRADrscb60rK4_7_n7mx6Ip0Q5uiS2ZECQcBR6Z8uuZumVcQ (Untitled)
  Session 4: QRhmpERBtntusi1QZ2IH-CYZz8YqvGsUxP5rSLCZFUJV6k9Fh9IzpoKcyUkay5qYqXNuTe4_QsJe4LM3bnFRfg (Untitled)
  Session 5: Z5AKjIPI2yAcnUe0vBJQ876BWLJQ6XJWcYAMhqakwaXAEkhgRaePkXsgJTBf8TLtVPGSn7RWgBvBAq_zwKXWwA (Untitled)
  Session 6: o1-Gof096ND-RPEft85EWug_1lfr_pJ9PK5P-0h1_mparol8lKHJxwGPXtTCJTuOEkqpagS5eun4RH9azwSU_A (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: ENZUc6J-ov9VDsYSxvtvbEWPu8mGL6CGnt8kN7HHlPpfaEyCLyttskpA-G9opIYiEBF5s_0NpuOKOCnQH7ZyhQ

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: C-m2kF2980TOCKFVBOFH0lgC6QY68KRQGCi1VYZ23KvaDWkMHcHaZetsZi9MdLek2pWYdzoi4VCeytz7yw8diA

=== Sending Initial Message ===
✓ Message sent
  Response: The product available based on the provided context is ekoDB. It is a high-performance database product and is priced at $99.

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['id', 'role', 'content', 'created_at', 'chat_id', 'token_usage', 'context_snippets', 'updated_at'])
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
✓ Created second session: jmpddhHFZ8sTVNF70g7Qmqj7SExneQLGhcthObJr-pu8acdXHAXCRIuxUGP_rkBZ9_M_cZt0hyqCSWJLO55HBw
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

✅ Script saved: lh5BUEM6DGKHZMuhANyoiGjQ76kmvqeQNQsvQInBY_Tk-G_V5iQtYG5rMIbzSg9Qx0SIXl2ODOPziwCdKBD5Kw
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved: W4GWgp3yKxkGoy1ZeIjUbHuhnRz5coy7NhTFh0R3YCNL3X6cqXuhRhPCxEeAyHqkuaNyY_LXvBOzIklwUfg1CA
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: Ma2AR1vDOW3NHQ4S6SR_LDyKzH89PfV_hrlj_QwG4A-uEsJ-eO5Lxp5FPVhHKvYE65Hbrj9lJrVS7ekDZpVGsQ
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

✅ Script saved: YvJ2FI5_Fz083TiPQvhQaYaltm-h084hQI9jRQSS8z4BW4ZzNeXwhxV2Zi5bZN8IP1W9MEtsWPHqPpQfqR2vZw
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
Created Alice: $1000 - ID: PaJGCKsyYxTEfpwA-52JJO3HTzC8Hu9My78wVjUeZQv2wedrQZvMsgc_V1kv858RzxULf7HLhVMaMywHG_CtEw
Created Bob: $500 - ID: lwZFsAQvm9mHqGB41fAjLPi8gCHIT1M4w-VcN7kMjGsfCWfsoiMi9Q5w6aMsRsSC0NgtWGmw6dFxVSOVH2NC-Q

=== Example 1: Begin Transaction ===
Transaction ID: 84ad6284-2080-44aa-bc2d-23d9bef43abf

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: a5007ba8-7fb6-4b16-8c70-188fe95c2e19
Status before rollback: Active
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
✅ [32mPython client examples complete![0m
