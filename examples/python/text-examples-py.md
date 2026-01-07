make test-examples-python
🧪 [36mRunning Python examples (direct HTTP/WebSocket)...[0m
=== Simple CRUD Operations (Direct HTTP) ===

✓ Authentication successful

=== Insert Document ===
Inserted: {'id': 'f0hCACX-m86BOjWCuwMFke6Qy3OspyJZx1Uq1d33A0pN1-mB1rjgb9HgWSTaV9LEw6BY87J5iIVFePuuVpkvag'}

=== Find by ID ===
Found: {'id': 'f0hCACX-m86BOjWCuwMFke6Qy3OspyJZx1Uq1d33A0pN1-mB1rjgb9HgWSTaV9LEw6BY87J5iIVFePuuVpkvag', 'value': {'value': 42, 'type': 'Integer'}, 'name': {'type': 'String', 'value': 'Test Record'}, 'active': {'value': True, 'type': 'Boolean'}}

=== Find with Query ===
Found documents: [{'name': {'value': 'Test Record', 'type': 'String'}, 'id': 'f0hCACX-m86BOjWCuwMFke6Qy3OspyJZx1Uq1d33A0pN1-mB1rjgb9HgWSTaV9LEw6BY87J5iIVFePuuVpkvag', 'value': {'type': 'Integer', 'value': 42}, 'active': {'value': True, 'type': 'Boolean'}}]

=== Update Document ===
Updated: {'active': {'value': True, 'type': 'Boolean'}, 'id': 'f0hCACX-m86BOjWCuwMFke6Qy3OspyJZx1Uq1d33A0pN1-mB1rjgb9HgWSTaV9LEw6BY87J5iIVFePuuVpkvag', 'name': {'value': 'Updated Record', 'type': 'String'}, 'value': {'type': 'Integer', 'value': 100}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: CQXM2art_zD-gjpFOHTRP740F6rffXlCn_MtjzsbpBXE-KdDU_Mp2pvMJQugfLRs2BTAgvS9oEJOdCOBue75KQ

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
        "id": "psWb4szhCOYR74wQQoV1UoXtekxHu4T-M77Ilvd9C8hiC7_DrSCj_I3Eh_y1yRQ_T5-wi4HjHQHATu_Z5OBMXg",
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
        "id": "CQXM2art_zD-gjpFOHTRP740F6rffXlCn_MtjzsbpBXE-KdDU_Mp2pvMJQugfLRs2BTAgvS9oEJOdCOBue75KQ",
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

✅ Script saved: DgK-efDZdsTqWZkXpkesR0UOUXRLNdue6k2mGm6evVFbTJjka_H_io9_yzg450fTavyzgqVm7LPOVixM3uutWQ
📊 Found 15 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: MJyIVRYWGPbgYupeMorNECOXB37yWOz2pJArF3uKPY9yNrOmgH-6aB2qYuSPxk6j9jvpD0UrBc6YEm7WWXMrOA
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: vF9JodIx1eO45DE48wRgrlC0EdD1Hvl_jAHa8qb2nFeNg4f9oHhIAnIHL5VbkMUlARGhXTAnBl8jQE9KBdbmtQ
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {'status': 'active', 'max_score': 100, 'avg_score': 60.0, 'count': 15}
   {'count': 15, 'status': 'inactive', 'max_score': 90, 'avg_score': 50.0}

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
cache:product:2: {'price': 39.989999999999995, 'name': 'Product 2'}
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
Collection created with first record: DCZg5zNXO3r_ORr1lyC_zdW-Sra2JDtz2CCPOLHuiyzE0mbKFweDDlxEZ6nTqvj8jcfqsfimr_L07IJSXL8PWQ

=== List Collections ===
Total collections: 13
Sample collections: ['test_collection', 'github_cache', 'websocket_test', 'demo_collection', 'products']

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
Created Alice: $1000 - ID: mPdF3gifbsve0f8iWT3hIhk-461IuTZ5iQu1zDgoPfyraq0leNg331WgiWZS-HE-j7jlMi5zJ6StcrlGNnJ54A
Created Bob: $500 - ID: asVTRWuy_98QZEzo5nRW2sgW8_u5OrqhHyqOIEj6AYzfyWFxazY-DysLcIPnvSMT9QMSJubLP5KvOd7Yz8CSSQ

=== Example 1: Begin Transaction ===
Transaction ID: 5c659078-e9d1-4786-ba40-66971f7227d3

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
New transaction: 6ab43e40-31e7-4b21-81bd-ec04df9024ba
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: BjA-G6p4hazayjlXq-ND9fdkudGDW58LK9phX1E5_t4SJnLpzTXNR3F3LOZuOGGLOvhAOHxeR-AQRSuWgxKWPQ
Created Bob: $500 - ID: 5vApVcbfzG0FVI62ejC6WKWMfv1kJm8T_sv2mjRXc45bw0qjcupMrHDXW-JUdDoIzn3-MYrqrNczNqagadIwBQ

=== Example 1: Begin Transaction ===
Transaction ID: d8debc15-6b25-4152-af23-192342376a53

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
Created Alice: $1000 - ID: pVNx7BO4bHOWVpt24uYqA7CB7g9PNHlZqofxiTDcuclP86UCPfPmt-nEI7FQ8XDMxTjqZAhm38YDZkhVoblBiw
Created Bob: $500 - ID: q6B5H4vCkYinBn8LR5iFNutL--TeB5KONwXmRZnKhqEzgDjp9oqU_SxaVxbyz8ezUf5mHCozeQM0D4y3CDFk6g

=== Example 1: Begin Transaction ===
Transaction ID: 8c37e628-efff-4ee6-a315-538c38832970

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
New transaction: 75f45189-763a-4e09-b02c-8d1cfcabaf2c
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: CfZAIaDYtjy2oGQesot4YVihWOYDuhPRRYfak7NlXo9njsxyv6EiCQjnFw2V_M2yJ2n5aLw03UISop9eJt7Ltg
Created Bob: $500 - ID: iOfINq8GEXPhQlzdO9RtaNTIomFu9RDOjZ_KlY98fZE_y92-nrJo9s7640CwcIZMBVBLMRcYdMH9DamPJq1BCg

=== Example 1: Begin Transaction ===
Transaction ID: 208d379e-66f6-4528-be52-83c7d1771c03

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
Created Alice: $1000 - ID: bf95z_KQKh0gUze99AP0bhexWmY59mHz3SVbLK45jW8TTpV9i66Qdndr5iOnY95XsJuM0PoBIZ-4lfilyJfiqw
Created Bob: $500 - ID: kAjmlC1dyiEGpVB1MelFyZvTF_wCwZHlGkbJhvJCQiLqJyZU--Qc8XEnZmfR7_JjYPLhbxOyjnEQ9HwukqpSWA

=== Example 1: Begin Transaction ===
Transaction ID: 8905e2e2-7895-40ce-b2c8-31ed6e9022b1

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
New transaction: d88c7d69-ce1b-42b4-8bba-bc63352b748c
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 12OVcLfypNpJ8RXr3tUGW4_g7u--Cxe-BBdw5g3TNSd_vFh1olZjZ28wSpWAlwoDLm5z-rjDC54JCf2PkEXPow
Created Bob: $500 - ID: X436LjXk_O80I03z34PevKZC2Ck-1Q0azuFXTCBAJrQ4dIHHwKEYkIKHubFKqd5b-x-NLFEkoaqrIzMBHSDVXA

=== Example 1: Begin Transaction ===
Transaction ID: 64c227dd-9ba5-48f3-9d0b-dbad2a67fa83

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
   ✅ Script saved: N6OHMXKII5BH0G_necfPP2yw-MR64F5DLp642yOVa1WaQ7FdnM2gxJeaARRJEtWdWRfJvP4nvmXNJSQgMV6vjQ

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: Wws7gkOzaMTtfrncGUNsDv4a_Cy73ZE0SI7heLoNldN_VGYx-0ZMET9olHsUbN7_JbXOkOWNFQh6hpla3Xj0jg
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}
   📋 Email: {'value': 'alice@example.com', 'type': 'String'}
   📋 Status: {'value': 'pending', 'type': 'String'}
   📋 Credits: {'type': 'Integer', 'value': 0}

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: SgylmhzYZpfMzmsrygciS_75pyeMka5tja91g9U8IG4F89Yu1JFdodjY1ZHEM7fLd-SnSlSNYrsuOYYhBiv6Pw

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
   ✅ Script saved: vF04r06Hxw4hrfy2rns2Se8Uc0A73zX93vh6jHVQpE7E0RUZ5lJgbSPPMBaKsqTBPYlcucXL60t4DsKm5v3K9Q

2️⃣ Calling Script (Query + Update Credits + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {'type': 'Integer', 'value': 0}
   📋 Status: {'value': 'active', 'type': 'String'}
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}

============================================================
📝 Script 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: y9_et167kFrcJxfOZ1PS3XFGnDGHb1-eN-ZBQnoWTaFGp4afzb9TXkdzcDvcXK-D3WBOijSk-YLym1rEFdAHTg

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: N6OHMXKII5BH0G_necfP...
   ✅ Deleted script: SgylmhzYZpfMzmsrygci...
   ✅ Deleted script: vF04r06Hxw4hrfy2rns2...
   ✅ Deleted script: y9_et167kFrcJxfOZ1PS...
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
  Output: Document ID = B571FJJxTDXesxlyagbrOykXUIw994pCLJJDsyusr9GOsKua5uRQnIypgnAE8-6525J6ak8KpdKBWYbN7at8WA
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: find_by_id(B571FJJxTDXesxlyagbrOykXUIw994pCLJJDsyusr9GOsKua5uRQnIypgnAE8-6525J6ak8KpdKBWYbN7at8WA)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: find_by_id(B571FJJxTDXesxlyagbrOykXUIw994pCLJJDsyusr9GOsKua5uRQnIypgnAE8-6525J6ak8KpdKBWYbN7at8WA)
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
  Output: Document ID = 6XLJEAWn5jSnb3LVYTvMgDKt-loNO1i1cSSZKJjw0-F5xOejfE9wzz2G1EqHPi5a9Xh06fgQJktzpzemMk1Mvg
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: find_by_id(6XLJEAWn5jSnb3LVYTvMgDKt-loNO1i1cSSZKJjw0-F5xOejfE9wzz2G1EqHPi5a9Xh06fgQJktzpzemMk1Mvg)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: find_by_id(6XLJEAWn5jSnb3LVYTvMgDKt-loNO1i1cSSZKJjw0-F5xOejfE9wzz2G1EqHPi5a9Xh06fgQJktzpzemMk1Mvg)
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
    Finished `release` profile [optimized] target(s) in 0.15s
📦 Built wheel for abi3 Python ≥ 3.8 to /Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.8.0-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel for current platform...[0m
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.8.0-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.8.0
    Uninstalling ekodb_client-0.8.0:
      Successfully uninstalled ekodb_client-0.8.0
Successfully installed ekodb-client-0.8.0
✅ [32mPython client package built and installed![0m
🧪 [36mRunning Python client library examples...[0m
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {'id': 'LeZCXG2crROYZZrxhy3XB13dsuDuBqfqc0DY9hY6jgD0pzFCQDIOm9W7fWPSOtHUBOp1xhPt-KB2quvEGVD4iQ'}

=== Find by ID ===
Found: {'created_at': {'value': '2026-01-07T11:49:27.655700', 'type': 'String'}, 'user_id': {'type': 'String', 'value': '550e8400-e29b-41d4-a716-446655440000'}, 'metadata': {'value': {'key': 'value', 'nested': {'deep': True}}, 'type': 'Object'}, 'id': 'LeZCXG2crROYZZrxhy3XB13dsuDuBqfqc0DY9hY6jgD0pzFCQDIOm9W7fWPSOtHUBOp1xhPt-KB2quvEGVD4iQ', 'data': {'type': 'String', 'value': 'aGVsbG8gd29ybGQ='}, 'name': {'type': 'String', 'value': 'Test Record'}, 'tags': {'value': ['tag1', 'tag2', 'tag3'], 'type': 'Array'}, 'price': {'value': 99.99, 'type': 'Float'}, 'value': {'type': 'Integer', 'value': 42}, 'active': {'type': 'Boolean', 'value': True}, 'categories': {'type': 'Array', 'value': ['electronics', 'computers']}, 'embedding': {'type': 'Array', 'value': [0.1, 0.2, 0.3, 0.4, 0.5]}}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): True
  price (Decimal): 99.99
  created_at (DateTime): 2026-01-07 11:49:27.655700
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): ['tag1', 'tag2', 'tag3']
  metadata (Object): {'key': 'value', 'nested': {'deep': True}}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): ['electronics', 'computers']
  data (Bytes): 11 bytes
Plain record: {'created_at': '2026-01-07T11:49:27.655700', 'user_id': '550e8400-e29b-41d4-a716-446655440000', 'metadata': {'key': 'value', 'nested': {'deep': True}}, 'id': 'LeZCXG2crROYZZrxhy3XB13dsuDuBqfqc0DY9hY6jgD0pzFCQDIOm9W7fWPSOtHUBOp1xhPt-KB2quvEGVD4iQ', 'data': 'aGVsbG8gd29ybGQ=', 'name': 'Test Record', 'tags': ['tag1', 'tag2', 'tag3'], 'price': 99.99, 'value': 42, 'active': True, 'categories': ['electronics', 'computers'], 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5]}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'name': {'value': 'Updated Record', 'type': 'String'}, 'categories': {'value': ['electronics', 'computers'], 'type': 'Array'}, 'value': {'value': 100, 'type': 'Integer'}, 'metadata': {'type': 'Object', 'value': {'key': 'value', 'nested': {'deep': True}}}, 'tags': {'type': 'Array', 'value': ['tag1', 'tag2', 'tag3']}, 'price': {'type': 'Float', 'value': 99.99}, 'user_id': {'type': 'String', 'value': '550e8400-e29b-41d4-a716-446655440000'}, 'data': {'type': 'String', 'value': 'aGVsbG8gd29ybGQ='}, 'created_at': {'value': '2026-01-07T11:49:27.655700', 'type': 'String'}, 'embedding': {'value': [0.1, 0.2, 0.3, 0.4, 0.5], 'type': 'Array'}, 'active': {'value': True, 'type': 'Boolean'}, 'id': 'LeZCXG2crROYZZrxhy3XB13dsuDuBqfqc0DY9hY6jgD0pzFCQDIOm9W7fWPSOtHUBOp1xhPt-KB2quvEGVD4iQ'}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: CTqBqotojIvfPOFnLEIr_CPCUQ6K2OBOZ-ispKtYb1ZuKffrIWvyrzHq9dU5EZe3HMyCoD7B6LHR59ypl4sXmQ

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
Collection created with first record: "r8JGv4ilyMWnfmDNQ_ITJrksAfc5pXR1pGG78Ley7h1lXfoDfSokPjqkGESqKK_38GhtUHe2BtyNdDIpTLYJ7Q"

=== List Collections ===
Total collections: 12
Sample collections: ['test_collection', 'github_cache', 'websocket_test', 'products', 'batch_users']

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
Created Alice: $1000 - ID: VqYAWoHq-IsXgBjahMq51YtyZsSumvgosBIBHaGiC0F2ArqAXGTfjhTZIJ3vuPe97gJqs-WcnHKPYTeyMB5gVg
Created Bob: $500 - ID: rRyaH-U_4smabVMb7Zf1xtPJteM3i4vml1Kpiwu_2tWJEqiGXNGJnUiAmIU-mTv9F4IDpg_9Mxm4ooCE0_oKJw

=== Example 1: Begin Transaction ===
Transaction ID: 488919e7-933f-4343-8fc3-f26e32f6b759

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 2c7608d0-4249-4840-bc56-8ce91e28f0f0
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
  1. Score: 25.740, Matched: name.value, email.value, name, email
  2. Score: 12.540, Matched: name.value, name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: bio, title.value, bio.value, title
  2. Score: 26.400, Matched: title, bio, title.value, bio.value
  3. Score: 26.400, Matched: bio, title.value, bio.value, title
  4. Score: 26.400, Matched: bio.value, bio, title, title.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio, title, bio.value, title.value
  2. Score: 39.600, Matched: bio.value, title, title.value, bio
  3. Score: 39.600, Matched: bio, bio.value, title, title.value
  4. Score: 39.600, Matched: title.value, bio.value, title, bio

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
  1. Score: 0.745
  2. Score: 0.745
  3. Score: 0.734

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.367, Matched: title, title.value, content, content.value
  2. Score: 1.372, Matched: title.value, title, content, content.value
  3. Score: 0.373, Matched: 

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
✓ Inserted document: VPMpnHRuuVCBN-2P_7WyD8NhrWnldqT35Y1EDFTMSfdo6c-6Z72YDhS5oxeIdvMO640yd1ItSB7clucHU19adA

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: c6xESOSIdW3GvsXuxx00mq0Rg8xfP5aj-GfPwpIOiNA-muZnFjQc8wogLAJ2lwhi295PWrRmVhB5fLhTb31nng

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
✓ Inserted document with TTL: eEnFVxMjF6_k-qKUsa1fSWliBXU_y1-4XKy3OVjmRJjst7zRJs9ZFfEE362UJczNr2HRCCosESfda7MpdGefGQ

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
✓ Edge cache script created: NNV95DWwSKcWMVrUy66cVjRCt1XJ9BpmV5ZtW1i3k8okHCk_IhKk38YXyYmZr3zKNH43DwKR0bYQCi5j3EjtHg

Call 1: Cache miss (fetches from API)
Response time: 1ms
Result: {
  "records": [],
  "stats": {
    "execution_time_ms": 0,
    "input_count": 0,
    "output_count": 0,
    "stage_stats": [],
    "stages_executed": 2
  }
}

Call 2: Cache hit (served from ekoDB)
Response time: 1ms (1.2x faster!)
Result: {
  "records": [],
  "stats": {
    "execution_time_ms": 0,
    "input_count": 0,
    "output_count": 0,
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

✅ Script saved: uSvvJ7nCyeHZ6_j_RU6Ps_EPNOY1s54vzp6USvY71kfU8VkEz5OootM4suQvpSblk0WCQbO9numJkoKIDu_9jw
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved: SpAsypcH8tm56m2uO8wEg_cV-QSgnfZTjlLGPu6IxlWQ2_l9o-SB-Sm8j_C2pf_s2HqRqXdzaw47_F4i5HjLVA
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: YY1NebOZw6XBPmfKeyyrsAD8QRE4-ZX4bohESOa5n60BSt-QAKbV0tMKyJMjf18rR6ZxFGiCw2skNpuYcEH0aw
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
   ⏱️  Duration: 132.2ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 87.3ms
   📊 Records: 1
   🚀 Cache speedup: 1.5x faster!

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

✅ Script saved: GryIyHmi7eSAYYbgM2mrCRqa4IVt7OdEPlWoqw0e_dapLgNI5RlSQ02GZLuF94rczzku9NaV2L3cl_NWS7b27g
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

✅ Inserted order: mnf7KTnOKrHvVEFjkwB6FPnLcyFRk9T3Eur_WCR0-Vhg00ZhbnGfQcUovePwWVuc361Enc0X7AcY_lUia37WHQ
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

✅ Inserted order: dwaWoJ40AWBkADi0Qxt7uoYKZqBwVXa5Uz7WGIOy7z4QMlwjkQFrHJeWdUYv3q6HCUpaN2qXixFJGPFxtlzW1A
✅ Cached order status
📊 Quick status lookup: {'value': '{"status":"processing","updated_at":"2026-01-07T16:49:30.240891+00:00"}'}

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
✓ Created SWR script: fetch_github_user_py (2IFg_GF4S9lAVXKtY6l3a4-VFLrvcmauuB5q_CudvdBb0vKGnYjssWANZioJe7qf5vHK5_LjzTWg9jx9WKEF1g)

Step 2: First call - Cache miss, fetches from GitHub API
Result: {
  "records": [],
  "stats": {
    "execution_time_ms": 0,
    "input_count": 0,
    "output_count": 0,
    "stage_stats": [],
    "stages_executed": 2
  }
}
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 2ms (served from cache)
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
   Vector databases provide benefits such as accurate geographical representation, better data consistency, the ability to handle complex spatial relationships, and efficient storage of data. They also allow for precise editing and analysis.
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
   1. Natural Language Processing (AI)
   2. Vector Databases Explained (Database)
   3. Introduction to Machine Learning (AI)
   4. Database Design Principles (Database)
   5. Getting Started with ekoDB (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Script saved
📊 Documents by category:
   {'category': 'AI', 'count': 2}
   {'category': 'Database', 'count': 3}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: -xcLjFXFHh-R-Bgx66ye6uW9RTmOYwXWVrLNxRA_zWqoqCYc3HDdw43VqJuBFw7ls_RJHgAgn19-gp-OvXZT5g

=== Sending Chat Message ===
Message ID: _yWNupX5jB-ifRctWC8et77RT38AaKpYSeEdR31HFaqql3cXpwSU1KPRoPD36Y59EWrQfrhzwabZH5V9f9AeFw

=== AI Response ===
There are three products available:

1. ekoDB Pro: This is an Enterprise edition product with advanced features. The price of this product is $299.

2. ekoDB: This is a high-performance database product with AI capabilities. The price of this product is $99.

3. ekoDB Cloud: This is a fully managed cloud database service product. The price of this product is $499.

Execution Time: 4144ms

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
✓ Created session: edcQjtwhPScN-c0ZNH7Y-mp6bt_rjhcrxZIyaIe_8c4twTATRCeo_-xsiFa1te1WzPB9YqQbL_NyorifjVT3Lg

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information available, there is a high-performance database product named ekoDB. It's priced at $99.

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['token_usage', 'updated_at', 'chat_id', 'created_at', 'id', 'role', 'content', 'context_snippets'])
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
✓ Created second session: 2z0WtTj1U5IXwDuns-OLvc06ilIGp6zB7yZaf7wad7Ru6Sw29Paelo2Gjm5rlKDSxNiWGZEGHkxIYyjb80OtkA
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
✓ Created session: Tz0-kccR_F1ZJIXKHGuZUUh-OSQHxZvgNILU04xBnpRdus8Nv00sSZTJBvkAZREjP8N4x2Lw5aDIblS6G4eZ2A

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the context provided, the available product is "ekoDB". This is a high-performance database product priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: eTuF7twW4mO5Qq4Ycnf2eW7SWkv_PO6xjF-g4fBmEFm41UyDhh41WUrEURUDICYJwj7NxonNqSq1C26i3uG75w
  Parent: Tz0-kccR_F1ZJIXKHGuZUUh-OSQHxZvgNILU04xBnpRdus8Nv00sSZTJBvkAZREjP8N4x2Lw5aDIblS6G4eZ2A

=== Listing Sessions ===
✓ Found 7 sessions
  Session 1: eTuF7twW4mO5Qq4Ycnf2eW7SWkv_PO6xjF-g4fBmEFm41UyDhh41WUrEURUDICYJwj7NxonNqSq1C26i3uG75w (Untitled)
  Session 2: Tz0-kccR_F1ZJIXKHGuZUUh-OSQHxZvgNILU04xBnpRdus8Nv00sSZTJBvkAZREjP8N4x2Lw5aDIblS6G4eZ2A (Untitled)
  Session 3: 2z0WtTj1U5IXwDuns-OLvc06ilIGp6zB7yZaf7wad7Ru6Sw29Paelo2Gjm5rlKDSxNiWGZEGHkxIYyjb80OtkA (Untitled)
  Session 4: -xcLjFXFHh-R-Bgx66ye6uW9RTmOYwXWVrLNxRA_zWqoqCYc3HDdw43VqJuBFw7ls_RJHgAgn19-gp-OvXZT5g (Untitled)
  Session 5: KMwr3q9cYv9nTfjb72AWsbsObQ3cuvqfQbNusT2wPLICyzWzP1IfQF5XB5ldG0Hem5yDKb4J7sEPGEmAgiAtbQ (Untitled)
  Session 6: aQvWLRkM9xqrR1nW3QvfDY3m3S1phEtCVuLD_Uxz2NqHPcpjhqb924KHYMIzlA8Kih-Z9M5GUZopILk-dUgIpQ (Untitled)
  Session 7: qd_TGU8qeuze5yU4lqFmn79h5KzryiY3ccxd7zedBP-7_u26iP2GQZ185jMWyQ-b27s3MgvwD7-B33CxNh4Gsw (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: eTuF7twW4mO5Qq4Ycnf2eW7SWkv_PO6xjF-g4fBmEFm41UyDhh41WUrEURUDICYJwj7NxonNqSq1C26i3uG75w

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Dict Creation ===
✓ Created record with plain dict: {'id': 'IqjChloiOn-a6z_2F3LLOgiagoPz7bgI-owzF3tI5jA54GNz-OBe6_WX0uS5-WTqk0FOuCkKnHArHWV9fJoCEg'}

=== Upsert Operation ===
✓ Upsert (update existing record): IqjChloiOn-a6z_2F3LLOgiagoPz7bgI-owzF3tI5jA54GNz-OBe6_WX0uS5-WTqk0FOuCkKnHArHWV9fJoCEg
✓ Inserted second record: 1qoDQBwFGGu1j0NjLMSuCalDGhZIUTa0ILcXS1KzRvGgPNMYfXBAfx30-XGeyQzCeNJUObD0sJ6iM26rjn9ZnA
✓ Upsert (update second record): 1qoDQBwFGGu1j0NjLMSuCalDGhZIUTa0ILcXS1KzRvGgPNMYfXBAfx30-XGeyQzCeNJUObD0sJ6iM26rjn9ZnA

=== Find One Operation ===
✓ Found user by email: {'name': {'value': 'Alice Johnson', 'type': 'String'}, 'id': 'IqjChloiOn-a6z_2F3LLOgiagoPz7bgI-owzF3tI5jA54GNz-OBe6_WX0uS5-WTqk0FOuCkKnHArHWV9fJoCEg', 'email': {'type': 'String', 'value': 'alice.j@newdomain.com'}, 'age': {'type': 'Integer', 'value': 29}, 'active': {'type': 'Boolean', 'value': True}}
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
   Inserted with ripple: {'id': 'VOyiHC4xgyTWR8VshvPy8s-hrlpeKqWbrowDrTXPfFQhn79Nex-IVQCzdGNmNAoG6Dyrg32pmrw9metsuM8fnA'}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {'id': 'FiMWqjVNzLiGi9g2BnE8zE6UMovCds9LZoghUCCOoin5P821INHrG8mC3TkU5elhPfo907P7LoKP4onMhe9CEQ'}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {'name': {'value': 'Product 1', 'type': 'String'}, 'price': {'type': 'Integer', 'value': 150}, 'id': 'VOyiHC4xgyTWR8VshvPy8s-hrlpeKqWbrowDrTXPfFQhn79Nex-IVQCzdGNmNAoG6Dyrg32pmrw9metsuM8fnA'}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {'price': {'type': 'Integer', 'value': 500}, 'id': 'VOyiHC4xgyTWR8VshvPy8s-hrlpeKqWbrowDrTXPfFQhn79Nex-IVQCzdGNmNAoG6Dyrg32pmrw9metsuM8fnA', 'name': {'type': 'String', 'value': 'Upsert Product'}}

✅ All bypass_ripple operations completed successfully!
✅ [32mPython client examples complete![0m
