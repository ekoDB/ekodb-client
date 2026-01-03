make test-examples-python
🧪 [36mRunning Python examples (direct HTTP/WebSocket)...[0m
✓ Authentication successful

=== Insert Document ===
Inserted: {'id': 'srp2jVhnwqfjXydVN6usmsKiebh3rtZGM_IMPSFTDH1LEiHMfBqj2rfBsq_dKlj-83D1rFeigvcfw6myyTr3NQ'}

=== Find by ID ===
Found: {'id': 'srp2jVhnwqfjXydVN6usmsKiebh3rtZGM_IMPSFTDH1LEiHMfBqj2rfBsq_dKlj-83D1rFeigvcfw6myyTr3NQ', 'value': {'type': 'Integer', 'value': 42}, 'name': {'value': 'Test Record', 'type': 'String'}, 'active': {'type': 'Boolean', 'value': True}}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'name': {'type': 'String', 'value': 'Updated Record'}, 'id': 'srp2jVhnwqfjXydVN6usmsKiebh3rtZGM_IMPSFTDH1LEiHMfBqj2rfBsq_dKlj-83D1rFeigvcfw6myyTr3NQ', 'value': {'type': 'Integer', 'value': 100}, 'active': {'type': 'Boolean', 'value': True}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: pgX3tN0CrrfdwZdfEP7IfhBBi2_OMTlOZKaFUVkClPD4NDbdz0ze2s_DBBIMQ5LTxd-lIMtq_vWFdt_XE62GnQ

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
        "id": "hVJrJrqbHjCEisTnEM8A-FztQmjbfEL92UsvIUHFCyH4j45VgMzWEtQyv_b1grPXz9_tC-9hntazuuCZdw86tg",
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
        "id": "pgX3tN0CrrfdwZdfEP7IfhBBi2_OMTlOZKaFUVkClPD4NDbdz0ze2s_DBBIMQ5LTxd-lIMtq_vWFdt_XE62GnQ",
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
Collection created with first record: C-Knh7qvWpKtP898TWo5DCz4Tji3kZTqUSezR3EVUeKlGA88lxgfcO2qA0h1AU1ic1l6pjrYQRA7O1JWPgkXbg

=== List Collections ===
Total collections: 11
Sample collections: ['ws_ttl_test', 'test_accounts', 'scripts__ek0_testing', 'test_collection', 'users']

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: False

✓ All collection management operations completed successfully
✓ Authentication successful

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: ImxmN2hPkQlZUF2D4wotJZH8rmTz8Rhocnks4BHxn3OKjK53qH3qxjC1J8KeG9HMu94BqfVeo0Ry2E4yoVSjRQ

=== Insert Document with TTL (5 minutes - integer) ===
✓ Inserted document: created

=== Insert Document with TTL (30 minutes - duration string) ===
✓ Inserted document with duration string TTL: -_X9YAbX_hjIO7vZk1YxHo_RU553sPNWBbUHRHi8stFVLVB7Y8Rs0eEAy-Ntoak-Xj5zTeTYt3KTleDaHbgCTg

=== Query Documents ===
✓ Found 5 documents with TTL

=== Update Document ===
✓ Updated document

=== Delete Document ===
✓ Deleted document

✓ All document TTL operations completed successfully
✓ Authentication successful

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: GvbN3QMJ-nIAwyz_bnQy1y9vphKyw3ojyXpUHbnf8j0NNZCnRdy6t8X_-tv5UgRj2zxb8sjOtBLVIP7PWDSP8g

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 2 record(s) via WebSocket

✓ WebSocket TTL example completed successfully
🚀 ekoDB Scripts Example (Python/HTTP)

✓ Authentication successful

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: lb_nGCGP3nSMANGUW_ZDifPRJ-xOmzxHovFzSCGUo7Ika_07EjhTgn84UT176lib0FhlopK390RJAzjmkV016w
📊 Found 30 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: hYC5ETvWHPERsjJzB9ISHfxZI8LuCAtGQvrTcjwKkN8vpv9SxLSSLLNOJVcrlaD7kKnFLgg6n9EJXbb4Y_aZug
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: EhappmHeMV8qHHfitpEIOX760Ql4mPhB8fwP0Zvpm9Nd0uapOemOrTceg8J8u5DSuBMH6RL6DzCN3uisFgTFTA
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {'count': 15, 'max_score': 100, 'avg_score': 60.0, 'status': 'active'}
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
Created Alice: $1000 - ID: OMCC9HawJggZKfY08o-c3pIBR7S6dzArjHRtSQbnIKVdQT8XiAyefVaGcsM95pDF5nmRcWlK7lRr2J215-POGQ
Created Bob: $500 - ID: AdFjJtlIvrucF2ONkb4_F1s3plk2e8enLPp2OspmhPhjCdSogTaNgFUC78tYFcdnZaPEW4I0o1gS8JcbVN43sQ

=== Example 1: Begin Transaction ===
Transaction ID: 1e532b63-a4c8-42d9-9331-4a4bf2e6ec40

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
New transaction: 078a0e76-9f39-45f2-bb10-ae7e1b978494
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: nVUxO58G-Y21SMTcT1qrSWvvTDR5OSfC1Upe-UjaytlXzluMhMZ7QavHEyscDAz81eI7wZNrFx7XNrEwsasIVg
Created Bob: $500 - ID: _UrPyZW1uAf2z50I-d4wqHy993VSeBchVWbWSYHmv1KDITkES_oYvvqXsIC96ivo86oXtsg4x_AU-X2fioochg

=== Example 1: Begin Transaction ===
Transaction ID: d38fa636-8f2b-4e6c-a894-0177403d7181

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
Created Alice: $1000 - ID: Z1cRvI40bfPE5XgROqCmQf52yrAwpWzr_Ed4MwnMDraxRIPfnakb3XiGF6Nj67BLKWWjL3dj7LIq9XTF6JyMrw
Created Bob: $500 - ID: VDXBPCLnKVfUiGCM7v_sYrB3sI1YBgYvOBdhpmZxdemEYLPVuHh7lTKcjNao7EbllJuATjbC-SYpE26anuSDhQ

=== Example 1: Begin Transaction ===
Transaction ID: cd1bec71-c7f7-4dd0-9f5f-7b8a5ce59f5a

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
New transaction: 52413ae6-2fb8-4693-9419-d5ee030ff5b4
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: bZ_9lLpg-Svj-jYE7PAcNp9VFQYIWDbkdr0n6Q3AmyRXL3c6v1IJI7A_3o7cRMyTNclcJg3Lr9GLj5O974sOTw
Created Bob: $500 - ID: rkRezE83F-RlZB73CrucTTlruIVyTt4l5cc7bDJBu8pmxOqI2h6jp-4p0ylL2j6iMrifE6-s3JyJ6Zi-FkqCKA

=== Example 1: Begin Transaction ===
Transaction ID: 13853489-b1bb-4cb3-8bc2-5bf806197c3f

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
Created Alice: $1000 - ID: Dz1AGiTY0sMJzCaZP8hzZAtUzObALe1ZZFme-5QtTCaDdXkkcbAvhEyEDh0ErIcvafHCKB4DghvgvJ42mIUFyQ
Created Bob: $500 - ID: xbvZ5U2PzWBJCFKJQdMkQYhHAOj-mKHr8twiQII3uJ9B9aOIO4fEufk4CUde40Ayer_zu-2F0AaTntGdJ3i8qg

=== Example 1: Begin Transaction ===
Transaction ID: d63e81fe-b379-4651-9e91-01aa7fbd1da9

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
New transaction: ff119684-c05d-4650-a5b4-4f19829e1014
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: wHPCQp-P82CR_Pi-H-xl47RZateFI8unueTbZfJYD31HQw56VudWTHkcBXXE09Cm4fSAJotHVPCSLx0u8jI68g
Created Bob: $500 - ID: 9UP3bEAGeW4ny3FKefvIor-cWQvCVGdRegXgoycBdgwzRRnJLD9dWGZiNNlflrsgMv57rNkTkee7Qj4CJaEPOA

=== Example 1: Begin Transaction ===
Transaction ID: 125f761a-4de9-45df-89bc-06fb85e81b45

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
   ✅ Script saved: _bjMZgw3HgakutDxZXS3RaxpzhnhimeAfw8RFtCnfi4PVcdRqzqdUJpJxzazvst8c55HE7TqufVV519rrDWH6w

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: EfymqsGklObH4QW4dC8Hg4QaJMibg0wg4CMRqcLEdivk0y4NEdwDIHHRlObHExJf8WpCefwaYYxjZ6Bk9DjeZg
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}
   📋 Email: {'value': 'alice@example.com', 'type': 'String'}
   📋 Status: {'value': 'pending', 'type': 'String'}
   📋 Credits: {'type': 'Integer', 'value': 0}

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: FlBv5x0gE1Q2lqNYoQ0F2WfW8nFChzDfvKj7iJ57th6O5apE2oUZkVLVfqrDp0bQQFqEf66_vKl_7qBlV9SK0w

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
   ✅ Script saved: vhPZs36QwJd-5WESWr5pHcIcDC35Bhhjzr8jATlsqRJsdOR3cWLyxgky2s5pya-fjLlTjtmoe85NYMRKREVzZg

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
   ✅ Script saved: 8x5XuMrMMB6x6Eu9fhXf9l5VJSxO8vWigCc9ja4X6dIYC378dkI25IdbyS6TdADEiu7HLuqMMt2aItgz0DpKtw

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: _bjMZgw3HgakutDxZXS3...
   ✅ Deleted script: FlBv5x0gE1Q2lqNYoQ0F...
   ✅ Deleted script: vhPZs36QwJd-5WESWr5p...
   ✅ Deleted script: 8x5XuMrMMB6x6Eu9fhXf...
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
📦 Built wheel for abi3 Python ≥ 3.8 to /Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.6.1-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel for current platform...[0m
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.6.1-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.6.1
    Uninstalling ekodb_client-0.6.1:
      Successfully uninstalled ekodb_client-0.6.1
Successfully installed ekodb-client-0.6.1
✅ [32mPython client package built and installed![0m
🧪 [36mRunning Python client library examples...[0m
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {'id': 'iPEEe7_k8MvltVX2mtjQEP4knaJdiMfY8om88f7FMQJz1YGMWtAVROkUURF8mNaEtu3vyN_3EDd6H-TgaWVaaQ'}

=== Find by ID ===
Found: {'active': {'value': True, 'type': 'Boolean'}, 'name': {'value': 'Test Record', 'type': 'String'}, 'embedding': {'type': 'Array', 'value': [0.1, 0.2, 0.3, 0.4, 0.5]}, 'user_id': {'value': '550e8400-e29b-41d4-a716-446655440000', 'type': 'String'}, 'data': {'type': 'String', 'value': 'aGVsbG8gd29ybGQ='}, 'tags': {'value': ['tag1', 'tag2', 'tag3'], 'type': 'Array'}, 'categories': {'type': 'Array', 'value': ['electronics', 'computers']}, 'price': {'type': 'Float', 'value': 99.99}, 'value': {'type': 'Integer', 'value': 42}, 'created_at': {'value': '2026-01-03T00:26:43.177847', 'type': 'String'}, 'metadata': {'type': 'Object', 'value': {'nested': {'deep': True}, 'key': 'value'}}, 'id': 'iPEEe7_k8MvltVX2mtjQEP4knaJdiMfY8om88f7FMQJz1YGMWtAVROkUURF8mNaEtu3vyN_3EDd6H-TgaWVaaQ'}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): True
  price (Decimal): 99.99
  created_at (DateTime): 2026-01-03 00:26:43.177847
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): ['tag1', 'tag2', 'tag3']
  metadata (Object): {'nested': {'deep': True}, 'key': 'value'}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): ['electronics', 'computers']
  data (Bytes): 11 bytes
Plain record: {'active': True, 'name': 'Test Record', 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5], 'user_id': '550e8400-e29b-41d4-a716-446655440000', 'data': 'aGVsbG8gd29ybGQ=', 'tags': ['tag1', 'tag2', 'tag3'], 'categories': ['electronics', 'computers'], 'price': 99.99, 'value': 42, 'created_at': '2026-01-03T00:26:43.177847', 'metadata': {'nested': {'deep': True}, 'key': 'value'}, 'id': 'iPEEe7_k8MvltVX2mtjQEP4knaJdiMfY8om88f7FMQJz1YGMWtAVROkUURF8mNaEtu3vyN_3EDd6H-TgaWVaaQ'}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'metadata': {'type': 'Object', 'value': {'nested': {'deep': True}, 'key': 'value'}}, 'price': {'value': 99.99, 'type': 'Float'}, 'tags': {'value': ['tag1', 'tag2', 'tag3'], 'type': 'Array'}, 'user_id': {'value': '550e8400-e29b-41d4-a716-446655440000', 'type': 'String'}, 'created_at': {'type': 'String', 'value': '2026-01-03T00:26:43.177847'}, 'name': {'type': 'String', 'value': 'Updated Record'}, 'active': {'value': True, 'type': 'Boolean'}, 'id': 'iPEEe7_k8MvltVX2mtjQEP4knaJdiMfY8om88f7FMQJz1YGMWtAVROkUURF8mNaEtu3vyN_3EDd6H-TgaWVaaQ', 'embedding': {'value': [0.1, 0.2, 0.3, 0.4, 0.5], 'type': 'Array'}, 'data': {'type': 'String', 'value': 'aGVsbG8gd29ybGQ='}, 'categories': {'type': 'Array', 'value': ['electronics', 'computers']}, 'value': {'type': 'Integer', 'value': 100}}

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
Collection created with first record: "kiVTAFXPjgEMx2h-kemDQzl6dh6VFeWayIcSBZAmdeZP6HBHYbHonxl4XkPQ3QYOvKbGmYe4OvnL7esm3QV7Kw"

=== List Collections ===
Total collections: 10
Sample collections: ['ws_ttl_test', 'test_accounts', 'scripts__ek0_testing', 'test_collection', 'websocket_test']

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: False

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: o0t4Ogtcx344b_kF5Yd8MA47oF001mLnT9U_EH-LgywkmlE22VMOtjGPmoyZBF0kh6s_8Yn1MgEaJ3P_8vBHCw

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: W4J--01h8oJhi6YGr2I_OqFLm9fW2OtAjqLNP3--z6bn2Fb4ofpOG2AxufcanYcmBGVJdFJ0eJWSHFPbjwpEdQ

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
✓ Inserted test record: tXT_J49TYn3DdeDrDl9lxmRajVYMl6aHfB4At3XeCThzqhvnJQJElqT_O7QHGTr2D8QsASHJOxCCZPsFNsu9FA

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
✓ Inserted document with TTL: pd6E4qh64nVlX12W0W5DXgy6NNPdWowarRMO4ZeqyWmdvGHa_oKDlu05J5yszfeghPa2vyP3ogVBtH137smS2w

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
  1. Score: 25.740, Matched: email, name, email.value, name.value
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: bio, title, bio.value, title.value
  2. Score: 26.400, Matched: title.value, bio.value, bio, title
  3. Score: 26.400, Matched: title, bio, bio.value, title.value
  4. Score: 26.400, Matched: title, title.value, bio, bio.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title, title.value, bio.value, bio
  2. Score: 39.600, Matched: bio, title.value, title, bio.value
  3. Score: 39.600, Matched: bio, bio.value, title.value, title
  4. Score: 39.600, Matched: bio, bio.value, title.value, title

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
  1. Score: 0.776
  2. Score: 0.764
  3. Score: 0.764

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.735, Matched: title.value, content, content.value, title
  2. Score: 1.143, Matched: title.value, title, content, content.value
  3. Score: 0.535, Matched: 

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
✓ Created session: zzMnYpnbOdEiHo60NW7v27gF22aW9C9vI17DJPbTOj_dVJ8W5PrS1iXVv3DLxXuW_gp_SHv8N7h3_9yZz7UGpg

=== Sending Chat Message ===
Message ID: u86ztLXecbAIBWAZGOkniLpulTzqJciFFdIcobTwnnbDEUsyAOJfpiNc92DByC3UMnyZMDjkaTzrP0fbvgdrFQ

=== AI Response ===
Based on the information available, there are three products:

1. "ekoDB Pro" is an enterprise edition product with advanced features. Its price is $299.
2. "ekoDB Cloud" is a fully managed cloud database service product. Its price is $499.
3. "ekoDB" is a high-performance database product with AI capabilities. Its price is $99.

Execution Time: 3195ms

=== Token Usage ===
Prompt tokens: 616
Completion tokens: 77
Total tokens: 693

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: qZMycTvr9bUB7YzLX4ZcWbXjNc0DhwM1cTVcywNKzMxIiU7HUYO1RD5mktQvJD1w7qRXgiu87_BhTzLj1_HUKQ

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information available, the product that is available is "ekoDB", which is a high-performance database product. It is priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 5n465zdaC6KlllJUCSAbZ3C7OUyZ9z8OWjuLEKnkn1GuzNHzVTc1HIdYDOwc6uoGRTZ9zcGQvuJYzVPhtiPxcA
  Parent: qZMycTvr9bUB7YzLX4ZcWbXjNc0DhwM1cTVcywNKzMxIiU7HUYO1RD5mktQvJD1w7qRXgiu87_BhTzLj1_HUKQ

=== Listing Sessions ===
✓ Found 6 sessions
  Session 1: 5n465zdaC6KlllJUCSAbZ3C7OUyZ9z8OWjuLEKnkn1GuzNHzVTc1HIdYDOwc6uoGRTZ9zcGQvuJYzVPhtiPxcA (Untitled)
  Session 2: qZMycTvr9bUB7YzLX4ZcWbXjNc0DhwM1cTVcywNKzMxIiU7HUYO1RD5mktQvJD1w7qRXgiu87_BhTzLj1_HUKQ (Untitled)
  Session 3: zzMnYpnbOdEiHo60NW7v27gF22aW9C9vI17DJPbTOj_dVJ8W5PrS1iXVv3DLxXuW_gp_SHv8N7h3_9yZz7UGpg (Untitled)
  Session 4: ufrTyD4PrTBEQptwzw4r9KilyotTBDWqwmk4QImGnlZfBf6Qx-Xc8uVTAkyswCKMpDmpE7KDxy8Iez2OfoOILQ (Untitled)
  Session 5: PNbBYjnZytJfK4lVrRLNRsiBxwy7a78bqkqjn6t5ALLHU37liYPxCrve7K80XGdiFzBiAyyIZnLhiCk0RpcR3Q (Untitled)
  Session 6: Qq23Wydaedyu53YVx8GKmxuBLGgTrRMMJJrULpbZ9S1uB9m90eEgQQja0jX0_QNEgwLr3vzJYz91qxxD6Dh-tQ (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: 5n465zdaC6KlllJUCSAbZ3C7OUyZ9z8OWjuLEKnkn1GuzNHzVTc1HIdYDOwc6uoGRTZ9zcGQvuJYzVPhtiPxcA

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 4Y6v4MUApBJKT2bv1pUeGfDEENiQ8e3OfWdsi73KUmsh1rhNECkJvadXd0TCeZESatNR8-7DFrMYZ0jDCVguBw

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information available, we have the ekoDB product. It's a high-performance database product.

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['chat_id', 'context_snippets', 'content', 'token_usage', 'role', 'id', 'created_at', 'updated_at'])
Debug: First message role: {'type': 'String', 'value': 'assistant'}
=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of ekoDB is 99.

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: GcI_-JS9SW66hdDUtOx8VFC7p1LwmVDKuARk01Ave2Iv5cQ4tA5LKv4tnVsbWwxppCW-Vc4RbAEteKlXSvJ3aQ
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

✅ Script saved: v9VWm-1xZu2VYtjZurZR4p6_kglTUN3_hq6uUMQSXoQn5P07THjujwGzSYH9zZzs74l-xmcSdOFLt9fGkmkRLg
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved: INJ3Spv9K_eDrWVl3XXXyi-EjT63mc4IRDsG143BGQg-ha4OS5DpqwPUIlG9hWc8GuLuf1XDEQYnhdCjGdB12g
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: NVU9rQhsAbe9wLSPc2-8eq-JK-W7CVw5rcP7jzehTRUjfSyN7zvvAqkLakg6PVTeOJJ-LTY8MXUyEKa4qpQhYQ
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

✅ Script saved: KbLUpn4yvbpM4BR5sk6-xm_MC4kWh5KaqJ-ORfFwsWnrXSlSzvGQcrnELHbDzYMdarhRjY1j4cMeJIXh4M-4NQ
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
Created Alice: $1000 - ID: ZYgX5cS6yJo1GUdAzqHyW9SkCQFU2HRXDQKMNSL33V_gbrF1i_CSDUqMOsO4lUJYjS61os9aQPevngmfbrjXDQ
Created Bob: $500 - ID: eUcYQ65l91nId1xIGzm-AcLgf3HbnHAG5z4qGzM0famk54KTyqO2oaXvCovAvDVd1j9rB7yOkyaZWCfoZ71cOg

=== Example 1: Begin Transaction ===
Transaction ID: ae8e2a42-dd65-4eea-b889-5506cf4ba342

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 3e3f4e69-e14d-4e28-81bb-285a135c79d7
Status before rollback: Active
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
✅ [32mPython client examples complete![0m
