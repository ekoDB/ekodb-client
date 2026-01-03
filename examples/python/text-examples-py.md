make test-examples-python
🧪 [36mRunning Python examples (direct HTTP/WebSocket)...[0m
✓ Authentication successful

=== Insert Document ===
Inserted: {'id': 'i0_IALTcjzMcBAmmjmVkcZbCtRGkVhoJA667xFh3-7IeDJ4_XgVb9LZ1VBfkyPSFf9sYzxU8ivwK6bhMdcWbew'}

=== Find by ID ===
Found: {'active': {'type': 'Boolean', 'value': True}, 'name': {'type': 'String', 'value': 'Test Record'}, 'value': {'value': 42, 'type': 'Integer'}, 'id': 'i0_IALTcjzMcBAmmjmVkcZbCtRGkVhoJA667xFh3-7IeDJ4_XgVb9LZ1VBfkyPSFf9sYzxU8ivwK6bhMdcWbew'}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'name': {'type': 'String', 'value': 'Updated Record'}, 'active': {'type': 'Boolean', 'value': True}, 'value': {'value': 100, 'type': 'Integer'}, 'id': 'i0_IALTcjzMcBAmmjmVkcZbCtRGkVhoJA667xFh3-7IeDJ4_XgVb9LZ1VBfkyPSFf9sYzxU8ivwK6bhMdcWbew'}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: NqBUHcYBhQsxpBJ6Wr445GhB5erdJyUXaXh6g2Af8sV1K5EPeEu9kVbQ_O36z37jxYySk8R6UZeDUiP1R0tk7w

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
        "id": "NqBUHcYBhQsxpBJ6Wr445GhB5erdJyUXaXh6g2Af8sV1K5EPeEu9kVbQ_O36z37jxYySk8R6UZeDUiP1R0tk7w",
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
        "id": "3REH3FFOJS88jlLDXdXST_xRlKUK1peqhumVvWGk4SnA-eoyIpwb9hCtPMEGaOqrhuTiQ8c5hactyKNl6qQWfg",
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
cache:product:1: {'price': 29.99, 'name': 'Product 1'}
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
Collection created with first record: eRiSIfP4ehikX1IzbeTMRhLTKCQbCeWaE5ZNsVjuxsuPMleXdrrLIb02G7yRYDkqNz3FLOZL4sN76aiSAZ2sEA

=== List Collections ===
Total collections: 11
Sample collections: ['batch_users', 'test_collection', 'users', 'demo_collection', 'websocket_test']

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: False

✓ All collection management operations completed successfully
✓ Authentication successful

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: Eqv5d-G91o1fdxNVec_o9YGo_lfOiOasHTKTm0bm6CFBcWDQD6vxr2nuAK-Pz6SLOr0lctdo_XVrxdQGsOg5mQ

=== Insert Document with TTL (5 minutes - integer) ===
✓ Inserted document: created

=== Insert Document with TTL (30 minutes - duration string) ===
✓ Inserted document with duration string TTL: jBxu-Uk9fcbWCGz6irZNYfQBxOwQ_fqNn5vLBMRmGX0H_lOBv_Sr-qG6_s87sJM50KDoQ9jMvU1PQcTSdpA2bw

=== Query Documents ===
✓ Found 5 documents with TTL

=== Update Document ===
✓ Updated document

=== Delete Document ===
✓ Deleted document

✓ All document TTL operations completed successfully
✓ Authentication successful

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: j2q-a5Noh4Au29tvpYb3P5o47kMfZ-I97VIJnlYykAsXq6so0MgbZEXooh-uQChzkxI9w9LShHrZT4Ql0fYKwA

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 2 record(s) via WebSocket

✓ WebSocket TTL example completed successfully
🚀 ekoDB Scripts Example (Python/HTTP)

✓ Authentication successful

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: _cFbbd6GMHda-NEK3Y71xCszYt0Z2X04qcZs7ryOyoWf0nJ6hhYgBNFprrr4yZlE0sy7gYtukV9dyunxDn1sTw
📊 Found 30 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: BJ3znt6FRbj1xFldGpPwsGobtALoSy81PIEWQGv1g7N0RfTHZXXbINf8SBNZ2XaarM-Lm_t8vaa50pqfwPv8hA
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: lW3e8PKp81InrU-4pOGnTr3qlkVjNBGpWyz1lajPiRlKysqGVD_6OADZ8HKZi_p5BBpnJnzFolMMC1IBU4UO7Q
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {'count': 15, 'status': 'active', 'max_score': 100, 'avg_score': 60.0}
   {'count': 15, 'status': 'inactive', 'avg_score': 50.0, 'max_score': 90}

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
Created Alice: $1000 - ID: mxTHrArSHMbtSU8mIX58FHTOPOnPs30xbQyAJhxhmhK2TOwXwryV7BqGphX5s4kCTt61bgAT9iap8MXRjQ4gYA
Created Bob: $500 - ID: RyMMLcvoZW2enpQPC3ZNWKaLsi2VP3-wskJLrDpdXkoZgexnLYUQdBFQBjLJiUn7JWgC-VDPeF5x-hvnQ1Ba3Q

=== Example 1: Begin Transaction ===
Transaction ID: ba1e14b2-9070-4997-91f6-9e63eb2218ef

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
New transaction: 08e371fc-844e-44a6-9316-f4513ba27e7a
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: W4Ed-7x-0vh39itJeJwFRvHJG-BCcytIDFBHbsnG8iUwUl3WA_8t_KDQ-60cuVMzcW_IC56sPcC-hdckoM8AHw
Created Bob: $500 - ID: EMUrNftPIHi_-Qxs-nSw1TQXRJWRtLC9Bqw5NmegzZkowI0JQZzJc_sNQEOVDA-cjfOfb5XUjGMwsqjdk8Q7sA

=== Example 1: Begin Transaction ===
Transaction ID: 9d0c88d3-a4ab-4b49-a521-2b3f84f988fb

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
Created Alice: $1000 - ID: -ZJKWKrlZy4zuFnYgMXw-0oIoiyGEe-9WVd85zxQc5IU95eFid8zX35LfgRKXFw2H_E_g7__WEVRHiE-VaTCug
Created Bob: $500 - ID: pj1bMBEQpXs-2f51aBhkQeAJnk17OiLZ0K_mryENT6c03ii9BKeNjxRc4Uh3GKuy7IH1SWR4_yFoFfNKDyWDwQ

=== Example 1: Begin Transaction ===
Transaction ID: 8e898186-649f-4bff-9e7b-bca7fc9bf984

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
New transaction: 2d2e35f4-44e3-4330-b225-0c7f6f27c4f2
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: cYa-pf5A4FqTmy5c4D03P6p6xfXl5Ugq1GL9BmyU8F2b9IcCdD20G4CfQK2IWVgXQAHVut3DMgtArXy9X3sXUQ
Created Bob: $500 - ID: ZXYTPypfH0tHIYuBRtB9gXQAIoSjV54u1ZLq5xoT1ZLXoGSnSsAhRAxPC9F6DmSCd7pcj-4-EyskIR_g6e-Zvg

=== Example 1: Begin Transaction ===
Transaction ID: 34ada4a1-e451-4618-bac0-054c92d84303

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
Created Alice: $1000 - ID: TFNHGsjIYrMkwiFwJYVT-BdMsS5uY0ty8ynCFM0X_h0Tp3wtKtUPEMqIBwI1PR0G5iDDRgOo0kAoGaSgpE4hkg
Created Bob: $500 - ID: 4MpTYd6jJcVG7lLT2nYH-L4QqzcWlhKip1soeyBApYRxzqrq8rMr1xySKMHZlvTnGDcMNfAr7Y1gIrvpymq_xg

=== Example 1: Begin Transaction ===
Transaction ID: ada1301b-e20d-4307-8e0f-2d0ff895f5c5

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
New transaction: cc818da0-a047-4f11-a282-065bf1174c82
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: -_xSoG3Y547b710_MTCMS43XB5bBNat0_dfNMvg9oszvmNJYmqvS2TrQPDxWvydjkoOCL4QerItFZfyFMsF6rw
Created Bob: $500 - ID: HczZ_CbW93yZh13BMAlK1lzHtdZAsnxdEz_7XZ-Ci17CdmGbg7rfj9uyMhcxaMG-981mV0Gv8wBVPmxfsyVo7w

=== Example 1: Begin Transaction ===
Transaction ID: 9e4d7b7e-2d56-4fa3-8718-823f23554e5e

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
   ✅ Script saved: yhzhZZSlXHZKB24v6myE6QdmnDcJG4c57Gcr9iyPNy9nnBdp8cPGnbfQeBrX3V-wNGTQouYtIS2VohGXorBOsg

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: wZKygAUzudm__zAXIFf1RVw3U-wbEkkRqY5HL5Ad2JyeWIC1uVpoZFyqaqUPJysv25v7tJ7BV1jIym02UMwRww
   📋 Name: {'value': 'Alice Smith', 'type': 'String'}
   📋 Email: {'value': 'alice@example.com', 'type': 'String'}
   📋 Status: {'type': 'String', 'value': 'pending'}
   📋 Credits: {'value': 0, 'type': 'Integer'}

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: fczbLEuFuAnISVYdiNo1rxGz6v-wHKirJ95lzT56gNQCPJFFsflgXiloftpcXK_hzG1KWqQEeLDez0si4Ch-lA

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
   ✅ Script saved: f5j3XmtUMDN1aK8n-HWAdqKVmsrPaPZdDLmWHsDWyg2bQt3zDMmsqpy-5agbFDVO2AsgNYYsCWcXVWZcSAT8Pw

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
   ✅ Script saved: 9X3xnbSLrda0IQt_92djhFIW-RoGwEPpif8Ee50PdHnXoJMc8ypGWT4Zm3KxPPVGt1JoT4FoYeSkCmxbRq_A-g

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: yhzhZZSlXHZKB24v6myE...
   ✅ Deleted script: fczbLEuFuAnISVYdiNo1...
   ✅ Deleted script: f5j3XmtUMDN1aK8n-HWA...
   ✅ Deleted script: 9X3xnbSLrda0IQt_92dj...
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
    Finished `release` profile [optimized] target(s) in 0.11s
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
Inserted: {'id': 'hBBuE3xXs2gv_vGk75eQqpBaQuP1UJ2AF6BkChWE8dDK9bhKncn-mExveC9_LwRSnFMX5BXjYrv9fYMf60t_LQ'}

=== Find by ID ===
Found: {'active': {'type': 'Boolean', 'value': True}, 'user_id': {'value': '550e8400-e29b-41d4-a716-446655440000', 'type': 'String'}, 'embedding': {'type': 'Array', 'value': [0.1, 0.2, 0.3, 0.4, 0.5]}, 'tags': {'type': 'Array', 'value': ['tag1', 'tag2', 'tag3']}, 'created_at': {'type': 'String', 'value': '2026-01-03T16:34:35.558917'}, 'categories': {'value': ['electronics', 'computers'], 'type': 'Array'}, 'price': {'type': 'Float', 'value': 99.99}, 'value': {'value': 42, 'type': 'Integer'}, 'data': {'value': 'aGVsbG8gd29ybGQ=', 'type': 'String'}, 'metadata': {'type': 'Object', 'value': {'key': 'value', 'nested': {'deep': True}}}, 'id': 'hBBuE3xXs2gv_vGk75eQqpBaQuP1UJ2AF6BkChWE8dDK9bhKncn-mExveC9_LwRSnFMX5BXjYrv9fYMf60t_LQ', 'name': {'value': 'Test Record', 'type': 'String'}}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): True
  price (Decimal): 99.99
  created_at (DateTime): 2026-01-03 16:34:35.558917
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): ['tag1', 'tag2', 'tag3']
  metadata (Object): {'key': 'value', 'nested': {'deep': True}}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): ['electronics', 'computers']
  data (Bytes): 11 bytes
Plain record: {'active': True, 'user_id': '550e8400-e29b-41d4-a716-446655440000', 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5], 'tags': ['tag1', 'tag2', 'tag3'], 'created_at': '2026-01-03T16:34:35.558917', 'categories': ['electronics', 'computers'], 'price': 99.99, 'value': 42, 'data': 'aGVsbG8gd29ybGQ=', 'metadata': {'key': 'value', 'nested': {'deep': True}}, 'id': 'hBBuE3xXs2gv_vGk75eQqpBaQuP1UJ2AF6BkChWE8dDK9bhKncn-mExveC9_LwRSnFMX5BXjYrv9fYMf60t_LQ', 'name': 'Test Record'}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'user_id': {'value': '550e8400-e29b-41d4-a716-446655440000', 'type': 'String'}, 'categories': {'type': 'Array', 'value': ['electronics', 'computers']}, 'price': {'type': 'Float', 'value': 99.99}, 'tags': {'value': ['tag1', 'tag2', 'tag3'], 'type': 'Array'}, 'data': {'type': 'String', 'value': 'aGVsbG8gd29ybGQ='}, 'embedding': {'value': [0.1, 0.2, 0.3, 0.4, 0.5], 'type': 'Array'}, 'value': {'type': 'Integer', 'value': 100}, 'name': {'value': 'Updated Record', 'type': 'String'}, 'metadata': {'type': 'Object', 'value': {'key': 'value', 'nested': {'deep': True}}}, 'id': 'hBBuE3xXs2gv_vGk75eQqpBaQuP1UJ2AF6BkChWE8dDK9bhKncn-mExveC9_LwRSnFMX5BXjYrv9fYMf60t_LQ', 'active': {'type': 'Boolean', 'value': True}, 'created_at': {'value': '2026-01-03T16:34:35.558917', 'type': 'String'}}

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
Collection created with first record: "0q0e6MYNry3t526nb05n_kWBweeSms0PUVdSA1WGM0IpjbQvHStrZZaIeAICUcD0SOeXXEIwo6kxVHeJCCHfhQ"

=== List Collections ===
Total collections: 10
Sample collections: ['batch_users', 'test_collection', 'websocket_test', 'scripts__ek0_testing', 'ws_ttl_test']

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: False

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: b55C6XorvgUtWxdTFGR2gB5fuyHukH_7EHs18Qva68cT96Q0a0P4VrKdggwMWvKzExMpxj-jvg3_Q6FVdOhoKg

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: PfXOS0NPnAkp4kw6H8LXE30X-bQyZgRon15sfbe5aSMwbUCdkNSpAY4umNSmyVVH0pfAREhYjRXQMQmH1PczIQ

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
✓ Inserted test record: nwUIttZpviquD5ldpbjyDZw_Bph5Hv10KsYTUIg1V0EZvLvJvqlzfuhee9EPbc1vhSYgO3deV4-7Ezek23Z3lw

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
✓ Inserted document with TTL: 7DxF0iaQDJzCEAX6AL6MVSy2NNVG9W7yDs5lxrCTu-6nURRbcxy5IuXwj59a-nT7K6SJY2h8Hta2D-f4880_Xw

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
  1. Score: 25.740, Matched: name, name.value, email, email.value
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: bio.value, title.value, title, bio
  2. Score: 26.400, Matched: bio, title.value, title, bio.value
  3. Score: 26.400, Matched: title, title.value, bio.value, bio
  4. Score: 26.400, Matched: bio, title.value, bio.value, title

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title, bio, title.value, bio.value
  2. Score: 39.600, Matched: bio, title, bio.value, title.value
  3. Score: 39.600, Matched: bio, title, title.value, bio.value
  4. Score: 39.600, Matched: bio, title, bio.value, title.value

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
  1. Score: 0.743
  2. Score: 0.733
  3. Score: 0.730

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.713, Matched: content, content.value, title.value, title
  2. Score: 1.111, Matched: content, title, title.value, content.value
  3. Score: 0.520, Matched: 

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
✓ Created session: suW32bDnrdg3dzucV75MS8O41JMRsxVPZR4KXhybpNPRgnT6cVCZdvuc9_7tH0H_Nvr4g0bW5094uRafHh7o-Q

=== Sending Chat Message ===
Message ID: XPbEvpbndbG58wmJVgEMmc6QuwXkTlPVyV_vdr1avdRasJF3UIWTqzLonIIBCfaFxdtCW_6zbPm_ezf4a7g43g

=== AI Response ===
We have three products available:

1. ekoDB: This is a high-performance database product with AI capabilities. The price of this product is $99.

2. ekoDB Cloud: This is a fully managed cloud database service product. The price of this product is $499.

3. ekoDB Pro: This is an Enterprise edition product with advanced features. The price of this product is $299.

Execution Time: 3406ms

=== Token Usage ===
Prompt tokens: 602
Completion tokens: 84
Total tokens: 686

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: wg3xdpQyALUaf3raEAK_aaMirLmzVKNiRvDmUlFEchZL29zEYSS5OQTeoRJ_SUgMMVXZ0tYKAanLUknaT9L6UA

=== Sending Messages ===
✓ Message 1 sent
  Response: Answer: The available product is ekoDB which is a high-performance database product. Its price is 99 units (the exact currency isn't specified in the context).

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: SPZoLHZwOu8EjL7mPjOk1hzMuci7dB_PManAEqE3z8sizapldEQ1jpxXuBemRcPnngiwK7gobK_vuQcvPccwrQ
  Parent: wg3xdpQyALUaf3raEAK_aaMirLmzVKNiRvDmUlFEchZL29zEYSS5OQTeoRJ_SUgMMVXZ0tYKAanLUknaT9L6UA

=== Listing Sessions ===
✓ Found 6 sessions
  Session 1: SPZoLHZwOu8EjL7mPjOk1hzMuci7dB_PManAEqE3z8sizapldEQ1jpxXuBemRcPnngiwK7gobK_vuQcvPccwrQ (Untitled)
  Session 2: wg3xdpQyALUaf3raEAK_aaMirLmzVKNiRvDmUlFEchZL29zEYSS5OQTeoRJ_SUgMMVXZ0tYKAanLUknaT9L6UA (Untitled)
  Session 3: suW32bDnrdg3dzucV75MS8O41JMRsxVPZR4KXhybpNPRgnT6cVCZdvuc9_7tH0H_Nvr4g0bW5094uRafHh7o-Q (Untitled)
  Session 4: 0dBcE6CuGPe38twSLr6KwkhV4MWkOcyUILsxWuw1XuDS6lcHD9gKIUGow5S05ZNUmWnef4DZqYv4Z5Wg4QZaeg (Untitled)
  Session 5: 4BJU8CUhE7wK9eBg_tXVfWzd-xoRSNQaJvE9j4aZcul7zYysl8DFpVIexM7wACzOVLGIjvWoSqD_gOxgxQu5mA (Untitled)
  Session 6: DcLiiFvuMZQ0jNmXFlxO-028DyjsLecXVNmcPmQuWrR-3Dwbx99wRtK3uS-9YGYwDiFqoD-Wa7sA1oEN4Sq7Hw (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: SPZoLHZwOu8EjL7mPjOk1hzMuci7dB_PManAEqE3z8sizapldEQ1jpxXuBemRcPnngiwK7gobK_vuQcvPccwrQ

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: YhaT0rg58UXwVvMbIF9lqhDkxybvuyekzsx8s6mWqXjgxXFsJRo5TFRZjEOhLc-49J53uyFkz-7VVkqNEks1fQ

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information provided, the available product is ekoDB, which is a high-performance database product. It is currently priced at $99.

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['context_snippets', 'token_usage', 'created_at', 'id', 'role', 'chat_id', 'updated_at', 'content'])
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
✓ Created second session: HvdlKRKII1VWid5cNQ0-HK17Qbk6_WXWU-tlxQ7Fjx7gnJwvMUCldZ21ikUg_fz2dfOn5hv90pnIG3uLW3Z3Rw
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

✅ Script saved: h4CNRITkenr7D_L7JTq4Q4S1gUV2OuHhzfUphaKDgPW00Ymg0_aNnAW2kOf2Af0DlIcNLbmxfLawctGqWz1Cuw
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved: lWWUQFnV4AXVvuHnxWrEaoYtgE8R-eltBUmTkgcdTALLggvO-QuqorH3aCDti5f_rWrzJTshcLlAhvDCbTu0Sg
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: O1By4DMX3ZMnjMD-3tMOjiZ64_xSLtFIWOieTiO02MijTRuDTZY2_GhEpeQL7JXehrUj9td6YNj1hrKrfNEVcQ
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

✅ Script saved: EdVNJbGJ5hVO40jLV87mKVkdvXH0lZe0wHZeEIOL2E66GdHu1zgiY1m9vuVMhH63g0oECHhexTkc5cbhg2Mv-Q
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
Created Alice: $1000 - ID: lQgmtNUB7eEKPofvKOEHEhONnfAYXGUMcRWvZ4y9RszkliIsVHUB_mUU1i1rKOaCPBHP_q1DskzNm63RJb633Q
Created Bob: $500 - ID: XraaUuN-JWSgYNSpsUNHMs4z5sV1NqOTNPZNNyb57GdXghsXBHtT45Cqk4-9l9i9PE4eHADxz1ngJTtZqV302Q

=== Example 1: Begin Transaction ===
Transaction ID: 851c69db-4e9f-42da-b8e5-1f3d5f7e38f2

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 9b3f3306-cd30-4826-90f7-cc6731796c10
Status before rollback: Active
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
✅ [32mPython client examples complete![0m
