make test-examples-python
🧪 [36mRunning Python examples (direct HTTP/WebSocket)...[0m
✓ Authentication successful

=== Insert Document ===
Inserted: {'id': 'VpZPIX6jbB1QS4jgfD8XwH1b9IDluid5qQ-I6o7RZaws4I6JO-NnATr42CLGuQBEAjbWa25C3fwygTO89qU5GQ'}

=== Find by ID ===
Found: {'name': {'value': 'Test Record', 'type': 'String'}, 'id': 'VpZPIX6jbB1QS4jgfD8XwH1b9IDluid5qQ-I6o7RZaws4I6JO-NnATr42CLGuQBEAjbWa25C3fwygTO89qU5GQ', 'value': {'value': 42, 'type': 'Integer'}, 'active': {'type': 'Boolean', 'value': True}}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'name': {'type': 'String', 'value': 'Updated Record'}, 'active': {'value': True, 'type': 'Boolean'}, 'id': 'VpZPIX6jbB1QS4jgfD8XwH1b9IDluid5qQ-I6o7RZaws4I6JO-NnATr42CLGuQBEAjbWa25C3fwygTO89qU5GQ', 'value': {'value': 100, 'type': 'Integer'}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: um21JmrqMHTOaWiaJQsXKEO81RaRGKGHACyQBVWU8yNrbkOgM0K0tZKnikMWm0v1nnEdc0jxGdcOI5tZuN4HRA

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
        "id": "v_o_VMTYzOkploU1BbN3TCNs9Kbc8H7BASVbe-rp30wX4gyOWAjC8jUwlFCsg2pqbeSVbXBiD-nK4oMNdpJJEg",
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
        "id": "um21JmrqMHTOaWiaJQsXKEO81RaRGKGHACyQBVWU8yNrbkOgM0K0tZKnikMWm0v1nnEdc0jxGdcOI5tZuN4HRA",
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
Collection created with first record: rff2Hcwwa55gH61SeoUj_KIawT2FOjInRezxIeZBCMP9XUO11KTHk28HMxWK-Fz5MlG1ykiqYtkq-zjxbBmjpQ

=== List Collections ===
Total collections: 11
Sample collections: ['chat_configurations__ek0_testing', 'ttl_cache', 'scripts__ek0_testing', 'demo_collection', 'test_collection']

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: False

✓ All collection management operations completed successfully
✓ Authentication successful

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: zQtCv-l0T0mBlyA4VA9cWX7cE421ZjhP7xT7LST6eFUs5myQtJduro7cUIVE0V1Hjh8M97Mf6lXAvhEd8Cj3YQ

=== Insert Document with TTL (5 minutes - integer) ===
✓ Inserted document: created

=== Insert Document with TTL (30 minutes - duration string) ===
✓ Inserted document with duration string TTL: l1vrLKJ55IDg4TwPzdGCSx2PijAZrc9LtappUeVQHsGDlDwKkO1733B-O5gq6yNjEZUIlX0HPe-wi7WKnW1AjA

=== Query Documents ===
✓ Found 5 documents with TTL

=== Update Document ===
✓ Updated document

=== Delete Document ===
✓ Deleted document

✓ All document TTL operations completed successfully
✓ Authentication successful

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: gPJ1Lyh94ujA-CV2TrEnCc0_qr1FCukeLLyyoJePYFapsQgmpQJOIdZICP3zkHwbdK4UDKrUA7NdmlFcW3Z1oQ

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 2 record(s) via WebSocket

✓ WebSocket TTL example completed successfully
🚀 ekoDB Scripts Example (Python/HTTP)

✓ Authentication successful

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: DhEdyznYZEAAdVFLJIXIgGk3DqCQSJ7CJFaXFR7GzfWCIXFdHNfzqp_VtBwWHzungPBTlTCquuM8Dp6rUopPzw
📊 Found 30 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: r90VyPsFrUYpwgcVaB1hDCu1dwoQDYRt8kwf5arBI2rPmMTUOB2SBqDPY5TTQrUHOqoAlLFNG3Ynv85fBWSGMA
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: AB72k8P3SwQxBBDSC_LkZsbtU4hwqL4u5leefU52Jc47sMBTnAefWQMIELKMwf0qBQjUr9csRhEBNL4RPPIuSA
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {'avg_score': 50.0, 'status': 'inactive', 'count': 15, 'max_score': 90}
   {'count': 15, 'avg_score': 60.0, 'status': 'active', 'max_score': 100}

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
Created Alice: $1000 - ID: buifY4YUnxMDABtCcWqRhBy-cXFVXZQqHECogebTV2tcxCo6SPgJFuHUoGWYurZrK0rl6tomhnSrIZwEIdzbSQ
Created Bob: $500 - ID: P83D0AVNs1CTtNKtR1fo2IRtTcQbzMkbe7EhQybqxqlkH8N-MwussL7tI7VfbTJgx4dQSeAAovhHxXrqUHI4Xg

=== Example 1: Begin Transaction ===
Transaction ID: 5e0b3d6e-48a8-42d9-9b0f-e3e425f30fa7

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
New transaction: be77e5c6-d507-4e06-937a-45d99e7e308d
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: YX4tBCbvsOtyUXc6HvMHDlHrGcaRMWgtDqTVIl6ADF01QwsyyGcjPHAehCkPLXbtW0mYGoEd0fvrkDyxT_Gp2A
Created Bob: $500 - ID: rD5qvnXGwyGK1rrHu04JFWpRPxOuMUC9Qcoi7I7sQctOW6sZ-wocFslX0ijlmPPKkZCcZU0obBgTMsqP7oKGJg

=== Example 1: Begin Transaction ===
Transaction ID: 64940ec8-ddce-4892-b6f5-570bb9446834

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
Created Alice: $1000 - ID: yRYG0Y0AwET0XuD7rDHN7jq88vEYBscSDshkvNV2BiArc8ACbCRFCGtpirgVSZewkzVdYGdcmC-GN8X3pb1-Mw
Created Bob: $500 - ID: -iVaI4iXnf4YdpOJ_JthqV0VA8X_Hn-e64gjKfNYb51NIYnfiiOqTP_CnCVC0TR1Yn9_zvmbxvY02tjHMv3mpA

=== Example 1: Begin Transaction ===
Transaction ID: 3c3718b6-ea54-4b8d-adac-4105c7baf2ca

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
New transaction: 5e64f15f-5015-4e00-b87b-71aeca22b16c
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: mxoBoZLf8AB2thF1VmtpJZq7k78V4MEKdaB1Q9gzm90YSw1o8aV0igEmiO_EaEIWvt03nGPrqo-G_3fsGaUObQ
Created Bob: $500 - ID: Pc4cKkDrgEuuvqtYtQkqdCO4hUp_C3SbhNyVEQbxspjseH12krP07-qEl3EJCnruEv-E4GUspb-QWfg5VG0--A

=== Example 1: Begin Transaction ===
Transaction ID: 0ed33c1d-a271-493e-b984-5fc41e970dd3

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
Created Alice: $1000 - ID: Luc9V6R9vQTyOWpNbR3O6vsaDBxKFC7muf2iW-6yAFQ-4dCAHGT8VIUGI7ExYHRYQCy5jwPAbM6MDICNjGeioA
Created Bob: $500 - ID: iRoirefKcSSCr9V9qBaGTj_z-JQ8T8bFEA1CEmhx7vXyDa_XIBSCUlJfAoy_Z05zM7PKJ7FI_1OdXv4MUikpEQ

=== Example 1: Begin Transaction ===
Transaction ID: 4615dafa-078f-414e-8725-98f1f993f8be

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
New transaction: f26b8c2a-574a-48f8-93bf-fe6249c9a1e9
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: QyHR9Drp8xQeea3QKR6_RsJ9dzmkmAhEf8d2KeGM5b-8_9Sg0ihApV4Rl9xGB9URJgSp_JyrNJ_rSaX0QusUqg
Created Bob: $500 - ID: ibqblKItBnN77gT761IX06geMdzbZOcaCtMC0Akm9J5UwcE--zNvBSMN6Se4O9y8-U1VbZLe3tKWW27O_uW1Ew

=== Example 1: Begin Transaction ===
Transaction ID: 9e55f12a-811d-4e45-8ecf-7da50bcdeba4

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
   ✅ Script saved: UJDeobuujAng8eUUAqa3erd-sbAAuxsCCmyh4QS0AmhQqkrKhFrbeoElm3lwjEmEPiRMpGVMY-l95gz6znRBCg

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: FcNv0eXVUg1wEVbLyeYX39N2rANfgdTewfnzTvIRrXOVrmvLxQ-b94fVEWsAe659ke_sm9yFJV2bbtBeCnp45Q
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}
   📋 Email: {'type': 'String', 'value': 'alice@example.com'}
   📋 Status: {'type': 'String', 'value': 'pending'}
   📋 Credits: {'value': 0, 'type': 'Integer'}

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: KC53LsabOSy6s91QCclaXn8z1CSw3OG0ZNM0y_87vLaCijMxVa6Jo-HVjYSqkX01wEYEyG278xwoakZfWh1B9A

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
   ✅ Script saved: 2n3od9xM92TjEXtHJrMd_U5bEqkozC1KphnbFwhmM9Y6yUKi2ankqScxCxUbzk_bF1pm1c35vt0yMUBECB_sAg

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
   ✅ Script saved: kNcuNqaQB07KxIi9TxmD0DcnzV177b6APtJlgt9rNexptuntCO7GHYvc1xFwGEyeN2P1T0h1RKOfcqG2mOrtpw

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: UJDeobuujAng8eUUAqa3...
   ✅ Deleted script: KC53LsabOSy6s91QCcla...
   ✅ Deleted script: 2n3od9xM92TjEXtHJrMd...
   ✅ Deleted script: kNcuNqaQB07KxIi9TxmD...
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
Inserted: {'id': 'sxNtMS0bqKj42LbupIMYM0R3up9fcfjcC0EE8s9j2w65EWgZLnM9tb5ykEoyCJ5nYWp8_tkCGS8jxXD-pG7y6w'}

=== Find by ID ===
Found: {'name': {'type': 'String', 'value': 'Test Record'}, 'id': 'sxNtMS0bqKj42LbupIMYM0R3up9fcfjcC0EE8s9j2w65EWgZLnM9tb5ykEoyCJ5nYWp8_tkCGS8jxXD-pG7y6w', 'value': {'type': 'Integer', 'value': 42}, 'tags': {'type': 'Array', 'value': ['tag1', 'tag2', 'tag3']}, 'active': {'value': True, 'type': 'Boolean'}, 'embedding': {'value': [0.1, 0.2, 0.3, 0.4, 0.5], 'type': 'Array'}, 'categories': {'type': 'Array', 'value': ['electronics', 'computers']}, 'user_id': {'type': 'String', 'value': '550e8400-e29b-41d4-a716-446655440000'}, 'price': {'value': 99.99, 'type': 'Float'}, 'created_at': {'value': '2026-01-03T18:54:38.697986', 'type': 'String'}, 'data': {'value': 'aGVsbG8gd29ybGQ=', 'type': 'String'}, 'metadata': {'type': 'Object', 'value': {'key': 'value', 'nested': {'deep': True}}}}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): True
  price (Decimal): 99.99
  created_at (DateTime): 2026-01-03 18:54:38.697986
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): ['tag1', 'tag2', 'tag3']
  metadata (Object): {'key': 'value', 'nested': {'deep': True}}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): ['electronics', 'computers']
  data (Bytes): 11 bytes
Plain record: {'name': 'Test Record', 'id': 'sxNtMS0bqKj42LbupIMYM0R3up9fcfjcC0EE8s9j2w65EWgZLnM9tb5ykEoyCJ5nYWp8_tkCGS8jxXD-pG7y6w', 'value': 42, 'tags': ['tag1', 'tag2', 'tag3'], 'active': True, 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5], 'categories': ['electronics', 'computers'], 'user_id': '550e8400-e29b-41d4-a716-446655440000', 'price': 99.99, 'created_at': '2026-01-03T18:54:38.697986', 'data': 'aGVsbG8gd29ybGQ=', 'metadata': {'key': 'value', 'nested': {'deep': True}}}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'active': {'type': 'Boolean', 'value': True}, 'created_at': {'type': 'String', 'value': '2026-01-03T18:54:38.697986'}, 'metadata': {'value': {'nested': {'deep': True}, 'key': 'value'}, 'type': 'Object'}, 'data': {'value': 'aGVsbG8gd29ybGQ=', 'type': 'String'}, 'tags': {'type': 'Array', 'value': ['tag1', 'tag2', 'tag3']}, 'id': 'sxNtMS0bqKj42LbupIMYM0R3up9fcfjcC0EE8s9j2w65EWgZLnM9tb5ykEoyCJ5nYWp8_tkCGS8jxXD-pG7y6w', 'value': {'value': 100, 'type': 'Integer'}, 'embedding': {'type': 'Array', 'value': [0.1, 0.2, 0.3, 0.4, 0.5]}, 'price': {'value': 99.99, 'type': 'Float'}, 'name': {'value': 'Updated Record', 'type': 'String'}, 'categories': {'type': 'Array', 'value': ['electronics', 'computers']}, 'user_id': {'value': '550e8400-e29b-41d4-a716-446655440000', 'type': 'String'}}

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
Collection created with first record: "5LO-OpOmyIf0SfzonohO8ssyIecRBSvJapPHm26ufUfYECi3YNvRX1qqzK5XcmrpjZSgUXaB2cb6Q3nt4hy4Lw"

=== List Collections ===
Total collections: 10
Sample collections: ['chat_configurations__ek0_testing', 'ttl_cache', 'scripts__ek0_testing', 'test_collection', 'batch_users']

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: False

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: ln3kMnT9OgnaDWbYg2VfEucW0qgSD0m70a28JqAJ_azE3s22Wkypg_Y2DLHtac3YV6PMHtkSAf8DxbaZcWYwkQ

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: XnwvfC4RqXCV_Y-Uwka87sj-aCTTcvVq2nyyAdMl8xBOy-yfZbgVg5V60MwJiM8ODhr7UGdeNNg5h0HWOx7A0Q

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
✓ Inserted test record: 6fnoBkzr3PkB2IYmWnzocDcCtkcpm652OO4vzD75D_QQvYpEQesF-fAOUKpTF7k_G6hDssP0pzZcKJD4xeeqCg

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
✓ Inserted document with TTL: 2pNI-VlnuR9GF4VdunKk6Wzagxq-WKCmAa_VLNNw0TgzLg88rGjUYkUotdVMbesY1lcyQlsqMymsXtQujckARg

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
  1. Score: 26.400, Matched: title.value, title, bio.value, bio
  2. Score: 26.400, Matched: title.value, title, bio.value, bio
  3. Score: 26.400, Matched: bio.value, bio, title.value, title
  4. Score: 26.400, Matched: bio.value, title.value, bio, title

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio, title, title.value, bio.value
  2. Score: 39.600, Matched: title, title.value, bio, bio.value
  3. Score: 39.600, Matched: bio, bio.value, title, title.value
  4. Score: 39.600, Matched: bio, title.value, bio.value, title

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 2 results (matches: run, running, runs)
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.764
  2. Score: 0.762
  3. Score: 0.743

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.734, Matched: title.value, content, content.value, title
  2. Score: 1.120, Matched: content.value, content, title, title.value
  3. Score: 0.535, Matched: 

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
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: l4g7zNZrVeaCo5zuQPZks0Yqia4jcciEDUMpkyHfX0jkRBqSmIH-phsZn7hXgLEJV4kU7DCyyt3ahLVBx7oCHQ

=== Sending Chat Message ===
Message ID: 9W28QzNDqEVlz0dU7czZwcVy1K4LA6Q-pLscxZxIPdgKK0VF8xpU34GEsc_wjdbK2LnlTvxl07QKbqWoNVUhLQ

=== AI Response ===
There are three products available:

1. ekoDB: This is a high-performance database product with AI capabilities. The price for this product is $99.

2. ekoDB Cloud: This product is a fully managed cloud database service. The price for this product is $499.

3. ekoDB Pro: This is the enterprise edition product with advanced features. The price for this product is $299.

Execution Time: 2491ms

=== Token Usage ===
Prompt tokens: 603
Completion tokens: 84
Total tokens: 687

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 9_4ief_zaxiUf23V_Kj6v5j3TWlQvXldFGtpK6XzyLDFL-gyHZEzZq_eHbrAMtOeIsKM1wIK4kdD5k1-aFqqQA

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information available, the product "ekoDB" is available. It's a high-performance database product and it's priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: ES97t9jsi5usrA6BVEgzOW9AT_eGyIm7_9lLzcklPxeMOErA5gccTnUy2v8Q0_7r8WU8QBaH0ldy-2i3hEP_Jg
  Parent: 9_4ief_zaxiUf23V_Kj6v5j3TWlQvXldFGtpK6XzyLDFL-gyHZEzZq_eHbrAMtOeIsKM1wIK4kdD5k1-aFqqQA

=== Listing Sessions ===
✓ Found 6 sessions
  Session 1: ES97t9jsi5usrA6BVEgzOW9AT_eGyIm7_9lLzcklPxeMOErA5gccTnUy2v8Q0_7r8WU8QBaH0ldy-2i3hEP_Jg (Untitled)
  Session 2: 9_4ief_zaxiUf23V_Kj6v5j3TWlQvXldFGtpK6XzyLDFL-gyHZEzZq_eHbrAMtOeIsKM1wIK4kdD5k1-aFqqQA (Untitled)
  Session 3: l4g7zNZrVeaCo5zuQPZks0Yqia4jcciEDUMpkyHfX0jkRBqSmIH-phsZn7hXgLEJV4kU7DCyyt3ahLVBx7oCHQ (Untitled)
  Session 4: Z2pocQ1N977hiiEZksfO2WyAC5l-VC1h9TJzhg0S1xbXfB2ONM-Qk_Zqb6aWaumPAg1PXd6Wrerc4mYNkdmKdw (Untitled)
  Session 5: CG2PChDTWEdZekN5sWiZVrf_WLAtDRjp3PJxpTJjFYSqQCVmjeLDQWziHPt0aaTvTUT6V0Yt986-P4qwCZzNRw (Untitled)
  Session 6: peLCg_SRnDYQAjqcOCWo_IJvvhygPTvZtRKGBFOslnP8qxJdAALGQnFev7fga-KnBOZNPjrLWG6uIi8ibXIUuw (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: ES97t9jsi5usrA6BVEgzOW9AT_eGyIm7_9lLzcklPxeMOErA5gccTnUy2v8Q0_7r8WU8QBaH0ldy-2i3hEP_Jg

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: XdbNvcAnUcEO6MDPRDXMuLU5Lu-nWKFCsZb5LtERJvPYctWEiLW0578Uz8p49EbI4Y9y4owTDcbuSIzh6tToCg

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the context provided, one available product is "ekoDB". It is a high-performance database product. The price for this product is 99.

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['id', 'role', 'content', 'context_snippets', 'chat_id', 'created_at', 'updated_at', 'token_usage'])
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
✓ Created second session: pJAkPJ50ceg34xRRKItswW6PzC6DxtbaJCb0cxXJ-vky-Pz_nkfgehDNTp6ms__wEKw2RaPUOav2-ltIrOmPqg
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

✅ Script saved: 4skv9xRiuGH9r41MelfcTs15CpXREdqL8vM6wNBdiCStqkqXV2uaQLZKXQzthJ7761dfldNfrH8b9aZJTA6Ymw
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved: UMGtfvd048Uom5mGg1ZmK0IcD-F4lmcfpQljtCr92_edtJpzOvBVRVlh4HvbIVZz-7qX8XeDOrXJw4kXdR2F1Q
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: bMa5TrbyIPyYpU7_l4XjIzBuwQcAqI5LbLHDfJeBwUSc6lkZQBZgZhBvxf7ycWI6Qn3o-v5DW9ZQQIq7xOr0Yw
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

✅ Script saved: k5oJZfVlQxQcRpZZtKEooqqTghxivyGrARP0MkWYeGbFj-gH9GwFAXZ-5qAm5XyojnagJ-jrMakoE6BydZ4-zg
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
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: TLMHcMwYWJ9cvwyuYqOHIG5xN0BdcXwsvxAbhh5m82ExzpHArYu20PbOlUwRaO_Q-CwpVfv97u8wbPsdsluOmA
Created Bob: $500 - ID: DV_ZVQYtWl25PvwtPBFGrHMFq46HnBu1uDm_pIYaNWtFlfYwfOT6zB68sMxj0st7VEKf6_oQYVCr1ZLLVpCvFw

=== Example 1: Begin Transaction ===
Transaction ID: a0278f3e-d6e6-4892-a80d-f1ecd150886d

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: f0b72de0-b554-4338-846f-92a9732146a5
Status before rollback: Active
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
✅ [32mPython client examples complete![0m
