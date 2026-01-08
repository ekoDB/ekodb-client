make test-examples-python
🧪 [36mRunning Python examples (direct HTTP/WebSocket)...[0m
=== Simple CRUD Operations (Direct HTTP) ===

✓ Authentication successful

=== Insert Document ===
Inserted: {'id': 'KJZbk8NzsoBv2RArbGcLUApIi2FnFJSc6hHld5rJTyUZ016wHrzvgMKp328tbVA8IBq3LG8YwVTbejdo3__opw'}

=== Find by ID ===
Found: {'value': {'type': 'Integer', 'value': 42}, 'id': 'KJZbk8NzsoBv2RArbGcLUApIi2FnFJSc6hHld5rJTyUZ016wHrzvgMKp328tbVA8IBq3LG8YwVTbejdo3__opw', 'active': {'type': 'Boolean', 'value': True}, 'name': {'value': 'Test Record', 'type': 'String'}}

=== Find with Query ===
Found documents: [{'id': 'KJZbk8NzsoBv2RArbGcLUApIi2FnFJSc6hHld5rJTyUZ016wHrzvgMKp328tbVA8IBq3LG8YwVTbejdo3__opw', 'name': {'type': 'String', 'value': 'Test Record'}, 'active': {'type': 'Boolean', 'value': True}, 'value': {'type': 'Integer', 'value': 42}}]

=== Update Document ===
Updated: {'value': {'value': 100, 'type': 'Integer'}, 'active': {'type': 'Boolean', 'value': True}, 'name': {'value': 'Updated Record', 'type': 'String'}, 'id': 'KJZbk8NzsoBv2RArbGcLUApIi2FnFJSc6hHld5rJTyUZ016wHrzvgMKp328tbVA8IBq3LG8YwVTbejdo3__opw'}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: Zs3qWTavKG00RXxhyD2QgfYzarxwKWLRD0l-Ntq7pJzuemEz8rt7R6vn-qovKfdkYl6WscoRzN-P-H9kczDSYw

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
        "id": "Zs3qWTavKG00RXxhyD2QgfYzarxwKWLRD0l-Ntq7pJzuemEz8rt7R6vn-qovKfdkYl6WscoRzN-P-H9kczDSYw",
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
        "id": "SVlLPEcHJ7pHpasjs8WN-AIaQWFzZ5XxZP_Uy6uluLftzJgli8t6DcQqg8gKNJyLE6qs6wgoPTDQAZkiqgc4_w",
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

✅ Script saved: fhE3dVaVdYMwolmN_7dKbq_WTUNbUZgWEp7DP37CMhZD7j_O7dtup5wCV012RHneLyNgeAkAaRKuZev_57z2lw
📊 Found 15 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: k9kT2DCBrwjvNmKe7rFYKnYiMiUdJ5UtgQYVeZH4EhU8tfX_LAnkF7AE0GEs1-cNCv9xxX4OOu6mToavTB-krw
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: G0RAVHOljxzHAHBZu2Qnk6OGLvEoKhUTktq75iR5wQadQMq9ANc5HNFCgJcxxdx9GO10T1qYMrZZHPtbZwRoTA
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {'count': 15, 'status': 'inactive', 'max_score': 90, 'avg_score': 50.0}
   {'avg_score': 60.0, 'status': 'active', 'max_score': 100, 'count': 15}

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
Collection created with first record: NeNvdD7-hw_mp3KpnsvkULaFZnsEwQBeDCodqZgt5dRDNLKCfpoNV_Xg7ES9fkAA78IuN4FkCEE817oY7kc9gA

=== List Collections ===
Total collections: 13
Sample collections: ['demo_collection', 'github_cache', 'websocket_test', 'test_accounts', 'scripts__ek0_testing']

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
Created Alice: $1000 - ID: zcbGmm78wPpEKz3pYbqniNf120JYID62exSQ1bPfJUdCWIBCvH1xSn9vmyj6e_7DHBHOLqbbvNKKJon1wM1gAw
Created Bob: $500 - ID: BWo88_pFOP_iRAt4_vdfZTBOxUgabixKNWEF5jWedHHhEaUPymnkNS1Rm9Zeb0Gg7trH2L6L8ucatVr94SltIg

=== Example 1: Begin Transaction ===
Transaction ID: 3466235d-16f1-490d-9e86-b62fd4d84efd

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
New transaction: 73df9a66-8366-4f51-8d7a-b4b8da8bf0b3
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: YHNk0kWZ1ZZVRNjHTpk-iLoymWmNzCHwri8Ta73kbYYtNFUB_fOmz7qIFf8Ww0jT-pHhyucevYxtfzq6OSi4Kw
Created Bob: $500 - ID: KCANR7vVG2xuG6X44-impwsWyex2IndR3nyU75XOnq15mBxqMYG0cVfFN7YX3EyDX7gkYiFcImD_eQlGrMLqlQ

=== Example 1: Begin Transaction ===
Transaction ID: ab3c261a-1068-4dae-ba33-76d895ae7517

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
Created Alice: $1000 - ID: oNbDQRx33lOEAbN_vMJj-lydNP9J24zGDky4HeyoTgm4CdJvdJFAr8aBo5GzUZm65mLEdrDvnyPCOuwG6rrxgA
Created Bob: $500 - ID: CFu2yN7tAxu13FSuXD-kWgPDdDin5TEb6lFgd6cVpYh7IWjoX0romA9YmA1oAPSL_-_E7eHxGFCAThuCaQ0MCA

=== Example 1: Begin Transaction ===
Transaction ID: 3b606eb4-2abb-4687-a77e-e5921df0715b

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
New transaction: 4d6bc897-79dc-488a-b54b-57f9ddc84576
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: EbEMlZc6j5Q-TXd1HYw359vfNWUMR90G7Sa2LUSW7AQOe4ehVJOETeZnvrhae_aLnd5WV7rBld5UV37L8AMgIw
Created Bob: $500 - ID: Myx7Oxvc5G8yN3FDs8HkDyP8Bit1bxwo7YNvn99UGgX6MAdgeo6IL3-WAquUqHt3FCKdM81E3h3qaQmgzanv6w

=== Example 1: Begin Transaction ===
Transaction ID: 092f8be4-9ed2-4b23-aafb-fbbde5de12c1

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
Created Alice: $1000 - ID: DJpd0Ad4NFlp9TaMNcWyF2di_QCbXiQp8ACM0-7JjNbP3vbvm7cUkCcqKnW0mstAbtsnpY92vUkukxxrDSGbjw
Created Bob: $500 - ID: N5nJBvaKfQvR7s1lFiI140qx3AOjPAnjLYxH-q9DYgV3Ij23r-448SFPKoFxfiHd8jGUfQvNZ-spyQrwCW36HA

=== Example 1: Begin Transaction ===
Transaction ID: 3421f9b8-e885-4e6b-a05e-2841b17b88df

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
New transaction: 471e25bf-39a5-43dd-ae52-9295ddd6a24c
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: lRYGwztUg3PHScK8B20FnaWXzt6ygiT2kzoP6UUswa464smK-n-b8532YsP8Pe9UwX-LqX0VAhAEHuwlx4dJBA
Created Bob: $500 - ID: e3w1jhxrMrg_7HH8tqanbSEWiPIyhz4sxNBS7R_mUg15satMibNKnxR3FUaQGEoJpbDvfyUROsH2HQtig_IHAg

=== Example 1: Begin Transaction ===
Transaction ID: d4ef262d-eb17-468b-a4ba-7ec01e29059f

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
   ✅ Script saved: qxyz9c9lrOrwSwqA9Z5lShY8MX6dNp8EHJbNJYz2bb1inH9cbBeVQ7OnxCss9y1mlJWoIIYHKBP8j5wNQPjYsQ

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: Nv5oE1OD3iRkIgMxuswY3RFTRtfqBcFG1S3HDF5xFKbLFw7Oi3TqRrPCRxPYS8aINKr4XoTOneF1wGDqK6YIOg
   📋 Name: {'value': 'Alice Smith', 'type': 'String'}
   📋 Email: {'type': 'String', 'value': 'alice@example.com'}
   📋 Status: {'type': 'String', 'value': 'pending'}
   📋 Credits: {'value': 0, 'type': 'Integer'}

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: LN1S8Z0CZ8QUjVNLgp30HqTyfNzI0kkkf6bYtQ_RVe-o8F_WcZDvZyG7X_IAc6cDLA5io1SYD3pusHVTJJtG5Q

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
   ✅ Script saved: bE6Pcf_04TX7KNY4Qnxlism60t91jX9P7hVfuMMkbJU2vfpboeEAyissOmuq8Z4ypUrjsONfufMV-pva6Nrh2A

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
   ✅ Script saved: 9nHhZdhsKXbr3CBCN8qftiJeidpbrI-Ayg76NI18jqzWSMSt9f0K34wjVa60U5NiYHC9HxJLlx6JmiWOfYYCqQ

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: qxyz9c9lrOrwSwqA9Z5l...
   ✅ Deleted script: LN1S8Z0CZ8QUjVNLgp30...
   ✅ Deleted script: bE6Pcf_04TX7KNY4Qnxl...
   ✅ Deleted script: 9nHhZdhsKXbr3CBCN8qf...
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
  Output: Document ID = wVGAR3dkYePtlXrwsqtgTVXtv2NXT3NR8F5uzIQgmIIdFge0EVpH2Vxy8AR_5pveszPrhg6a97B7bX32BAEN3Q
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: find_by_id(wVGAR3dkYePtlXrwsqtgTVXtv2NXT3NR8F5uzIQgmIIdFge0EVpH2Vxy8AR_5pveszPrhg6a97B7bX32BAEN3Q)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: find_by_id(wVGAR3dkYePtlXrwsqtgTVXtv2NXT3NR8F5uzIQgmIIdFge0EVpH2Vxy8AR_5pveszPrhg6a97B7bX32BAEN3Q)
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
  Output: Document ID = BFTc4Lj_qiY0WE_3bUc8Fx59UUnpRFvk6_p4KvVq5h6q2HBVJ3iHT-e4aktVzODYOYlQ5n4eudAsflqRUNSh2w
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: find_by_id(BFTc4Lj_qiY0WE_3bUc8Fx59UUnpRFvk6_p4KvVq5h6q2HBVJ3iHT-e4aktVzODYOYlQ5n4eudAsflqRUNSh2w)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: find_by_id(BFTc4Lj_qiY0WE_3bUc8Fx59UUnpRFvk6_p4KvVq5h6q2HBVJ3iHT-e4aktVzODYOYlQ5n4eudAsflqRUNSh2w)
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
    Finished `release` profile [optimized] target(s) in 0.07s
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
Inserted: {'id': 'wW3jaE1OxDgbK5haI47_2tW7dtpE5S6-HuMM2wg5A970CeOA9alNG4YVeArHFO_28SO0NTYADmLbiPBNiZfyRQ'}

=== Find by ID ===
Found: {'categories': {'value': ['electronics', 'computers'], 'type': 'Array'}, 'created_at': {'type': 'String', 'value': '2026-01-07T21:40:18.664721'}, 'metadata': {'type': 'Object', 'value': {'key': 'value', 'nested': {'deep': True}}}, 'name': {'type': 'String', 'value': 'Test Record'}, 'data': {'type': 'String', 'value': 'aGVsbG8gd29ybGQ='}, 'active': {'type': 'Boolean', 'value': True}, 'value': {'value': 42, 'type': 'Integer'}, 'id': 'wW3jaE1OxDgbK5haI47_2tW7dtpE5S6-HuMM2wg5A970CeOA9alNG4YVeArHFO_28SO0NTYADmLbiPBNiZfyRQ', 'tags': {'type': 'Array', 'value': ['tag1', 'tag2', 'tag3']}, 'embedding': {'type': 'Array', 'value': [0.1, 0.2, 0.3, 0.4, 0.5]}, 'price': {'type': 'Float', 'value': 99.99}, 'user_id': {'value': '550e8400-e29b-41d4-a716-446655440000', 'type': 'String'}}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): True
  price (Decimal): 99.99
  created_at (DateTime): 2026-01-07 21:40:18.664721
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): ['tag1', 'tag2', 'tag3']
  metadata (Object): {'key': 'value', 'nested': {'deep': True}}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): ['electronics', 'computers']
  data (Bytes): 11 bytes
Plain record: {'categories': ['electronics', 'computers'], 'created_at': '2026-01-07T21:40:18.664721', 'metadata': {'key': 'value', 'nested': {'deep': True}}, 'name': 'Test Record', 'data': 'aGVsbG8gd29ybGQ=', 'active': True, 'value': 42, 'id': 'wW3jaE1OxDgbK5haI47_2tW7dtpE5S6-HuMM2wg5A970CeOA9alNG4YVeArHFO_28SO0NTYADmLbiPBNiZfyRQ', 'tags': ['tag1', 'tag2', 'tag3'], 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5], 'price': 99.99, 'user_id': '550e8400-e29b-41d4-a716-446655440000'}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'embedding': {'type': 'Array', 'value': [0.1, 0.2, 0.3, 0.4, 0.5]}, 'created_at': {'type': 'String', 'value': '2026-01-07T21:40:18.664721'}, 'active': {'type': 'Boolean', 'value': True}, 'data': {'value': 'aGVsbG8gd29ybGQ=', 'type': 'String'}, 'metadata': {'value': {'key': 'value', 'nested': {'deep': True}}, 'type': 'Object'}, 'categories': {'type': 'Array', 'value': ['electronics', 'computers']}, 'id': 'wW3jaE1OxDgbK5haI47_2tW7dtpE5S6-HuMM2wg5A970CeOA9alNG4YVeArHFO_28SO0NTYADmLbiPBNiZfyRQ', 'name': {'type': 'String', 'value': 'Updated Record'}, 'price': {'value': 99.99, 'type': 'Float'}, 'tags': {'value': ['tag1', 'tag2', 'tag3'], 'type': 'Array'}, 'value': {'type': 'Integer', 'value': 100}, 'user_id': {'type': 'String', 'value': '550e8400-e29b-41d4-a716-446655440000'}}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: do0jEfMdM800iUqI-D_fnmlb846HGwjS9ZM7kZ4v9qcs3BBzdsbeIc87F2CSq3gEaYGApzGLgzLs4fPaQRTpOQ

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
Collection created with first record: "YZy-9FeExX4ayNn0tKBczgOTuSV4KfwSOedDVYpwLhV36iNDmehAWRxYPVPy4DuTSghqCQb_LuryHu72MaLQbQ"

=== List Collections ===
Total collections: 12
Sample collections: ['github_cache', 'client_collection_management_python', 'websocket_test', 'test_accounts', 'scripts__ek0_testing']

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
Total keys in store: 8

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: False

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: NWdlioFIhUy3v1L_XZkRHQKrWTO3AYxt66TxFoqtb3GZybjPTs-CHl-keqjt1to8Zt4tqybx_dMoCVKHVSVyfA
Created Bob: $500 - ID: mTVsRWp8TDfouGflQ1JhcP366QCJBVnIeJ_6vMqcOtA16-rABYIwR3YipZPhfNagZ_AI3H_kSdTkxNwvx7EXow

=== Example 1: Begin Transaction ===
Transaction ID: 8bad3d4e-fb4f-401a-b440-2e657223cb18

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 8c236d20-e49f-47bb-93dd-0e42a81f4d68
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
  1. Score: 25.740, Matched: name, name.value, email, email.value
  2. Score: 12.540, Matched: name.value, name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: bio.value, title, title.value, bio
  2. Score: 26.400, Matched: bio, title, title.value, bio.value
  3. Score: 26.400, Matched: bio.value, title.value, title, bio
  4. Score: 26.400, Matched: bio, title.value, title, bio.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio, title, title.value, bio.value
  2. Score: 39.600, Matched: title.value, title, bio.value, bio
  3. Score: 39.600, Matched: title, bio, title.value, bio.value
  4. Score: 39.600, Matched: bio, bio.value, title, title.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio.value, bio

5. Search with stemming and exact match boosting:
Found 2 results (matches: run, running, runs)
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio.value, bio

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.740
  2. Score: 0.733
  3. Score: 0.721

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.367, Matched: title.value, content, title, content.value
  2. Score: 1.360, Matched: title, content.value, title.value, content
  3. Score: 0.370, Matched: 

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
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: _5uSikvKI2dezAE9-DV9jhTmFLrW7ejMoY0rTxa86XRGkkuizdiPd7hI3XB8nvv5J4R65cYlfXMZL6YevwEQ4w

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: XahX_NK1SeQkR1ifHn7VO7k7tbaFoXdu7WYLu4MV8_OXZ2trdanqQhDX7bknJiDiI0hQLpu41wyR37tIFwZljA

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
✓ Inserted document with TTL: QK2kqjFKikNLy3U9M4GIeV4pWFgyy1unuwbYbcJCzZdaNFuFFac71we3oSPGxomZg7t3MFIcDItJid2LfqCMHg

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
✓ Edge cache script created: kuU0nedB_MATQ9Zl7ijtpbtDTiQv5OOocnLc4w2j--2f9bKxrNNf3OVe7XML1OpSwbOxbwtT6jrRTPYtc9lrDg

Call 1: Cache miss (fetches from API)
Response time: 2ms
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

✅ Script saved: KrXH3zX5h_bEepKW9i74-oe6tekwlssRTgBIADFs0Rc3a76Hk7Xg0mw--NcV6z1Fvq0YTGSg_xbCP5snJB89rg
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved: KeT6IXatx9W_sX21PE-iYc87P2O6xnkDOjCSPJFnoczi4HeLjAnTx_Ry9ROVuVpEbMagN_BPQ4Vxm0VE8-f0Ow
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: BXmfXuFHB0VCTQs5TlVvOCj6Hs6CJLN2dEZjbrK4P-6WVI5ZtT9AS0Aezx7BTFJYxh3wePGag_q6iwLPuz7A_w
📊 Statistics: 2 groups
   {'avg_score': 60.0, 'count': 5, 'status': 'active'}

   {'avg_score': 50.0, 'count': 5, 'status': 'inactive'}

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
   ⏱️  Duration: 131.4ms
   📊 Records: 0

Second call (cache hit - from cache):
   ⏱️  Duration: 84.9ms
   📊 Records: 0
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

✅ Script saved: WN1f_lj_idf4TIEggUiFq-FVsfBDJB7wPV0s8oLyikqGgg48Uojkb0T6FP4fSASYD8rr2jGdS9N4vcl5K5AoWg
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

✅ Inserted order: JnOUeNnOIAXvVXCHyF_4n69dq666hDVEWyZGDMywmm3-y_adpZER9NmDhm3Rq7pHxKAyr3MMsbOVPoNBaAwEXQ
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

✅ Inserted order: 5x92ejttZ5IGWEFFoq6et-8SisPYw-estXbdEyNKqn_pUnXmQYE7gCTRtWlRsBnsbNlLChaFFEn4xGS0totaCQ
✅ Cached order status
📊 Quick status lookup: {'value': '{"status":"processing","updated_at":"2026-01-08T02:40:21.286854+00:00"}'}

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
✓ Created SWR script: fetch_github_user_py (ojwSZVAhOZGNfFEW6CqQR2_3dU1Ct2lH4lIDLBaSSDJPFBsUEmwbA2C7811QxqKk_tRlIrhaWpmTZ1H6VdFkmQ)

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
   Vector databases offer several benefits including:

1. High precision: They store data as points, lines, and polygons, providing accuracy in representation.
2. Scalability: Vector data can be easily scaled up or down without losing quality.
3. Attribute storage: They allow for the storage of additional attributes or information about each vector.
4. Flexibility: They are capable of representing features of all sizes and shapes, including complex structures. 
5. Less storage space: Compared to raster databases, vector databases often require less storage space.
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
   2. Introduction to Machine Learning (AI)
   3. Natural Language Processing (AI)
   4. Getting Started with ekoDB (Database)
   5. Vector Databases Explained (Database)
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
✓ Created session: 3NxXAggJhkJnSgTudToXG9vYml6UMoNakmstzAWOv8utXP8AT5SIXcBWeApEuFalEyudulfn0gvvqKudKjty-Q

=== Sending Chat Message ===
Message ID: pdN1Ta0RnsQteIzD1plT6CHKH5kGx5cVoDLXuYq5T2a4zXSfe19FcLHnKhqxgcLVYLnhUwNGcH49COCsPaMRdw

=== AI Response ===
There are three products available:

1. ekoDB: This is a high-performance database product with AI capabilities. It is priced at $99.

2. ekoDB Pro: This is an enterprise edition product with advanced features. It is priced at $299.

3. ekoDB Cloud: This is a fully managed cloud database service product. It is priced at $499.

Execution Time: 4059ms

=== Token Usage ===
Prompt tokens: 446
Completion tokens: 78
Total tokens: 524

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: hfa65lFNg_UWeuHGKS71pD-mFEp5XWG4ND2fGsvWmtJOJszU-m1lwURWrEF0_FcP8ZRT6goOlQm67gs4B_8Yfw

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information provided, one of the available products is called "ekoDB". It's a high-performance database product and its price is $99.

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['created_at', 'chat_id', 'role', 'token_usage', 'content', 'updated_at', 'context_snippets', 'id'])
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
✓ Created second session: 57jBIahNPlAPPXE6bVy_VvwYy7ki92yCKM39SaZ_pcZ7IcxH-IWyi9f2UGzrJF2gEDNqLRqCP8q4hFV7rznaHw
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
✓ Created session: DIGMAxv66exWItthxiQVC4SoPGLxkvXlUzFvHP20kylx7_FzOqzocFxd1lYW5DLF9u1HNjN0wz0M7g7gd6A7Yg

=== Sending Messages ===
✓ Message 1 sent
  Response: The product available is ekoDB, which is a high-performance database product. It is priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: c6g9Wc6xNQG4t0vw3-KHk9to_MrWWn-hmQltezQ5HO-Fs1G3J1lQ0w0scQUxawfC9Y8yyEwWmOHSjhA7qzHGaA
  Parent: DIGMAxv66exWItthxiQVC4SoPGLxkvXlUzFvHP20kylx7_FzOqzocFxd1lYW5DLF9u1HNjN0wz0M7g7gd6A7Yg

=== Listing Sessions ===
✓ Found 7 sessions
  Session 1: c6g9Wc6xNQG4t0vw3-KHk9to_MrWWn-hmQltezQ5HO-Fs1G3J1lQ0w0scQUxawfC9Y8yyEwWmOHSjhA7qzHGaA (Untitled)
  Session 2: DIGMAxv66exWItthxiQVC4SoPGLxkvXlUzFvHP20kylx7_FzOqzocFxd1lYW5DLF9u1HNjN0wz0M7g7gd6A7Yg (Untitled)
  Session 3: 57jBIahNPlAPPXE6bVy_VvwYy7ki92yCKM39SaZ_pcZ7IcxH-IWyi9f2UGzrJF2gEDNqLRqCP8q4hFV7rznaHw (Untitled)
  Session 4: 3NxXAggJhkJnSgTudToXG9vYml6UMoNakmstzAWOv8utXP8AT5SIXcBWeApEuFalEyudulfn0gvvqKudKjty-Q (Untitled)
  Session 5: Jk-YojHkNfcOm8Q15rRclp2BzZa0XdD07qEzY3nVgRQWph8Uuq0BnBsLmXrz1T7xbwUX1_ewyFCbqFGL0NkQYQ (Untitled)
  Session 6: QLU0cQni__dtpqONfLrpNwkpCbh0lXQkXhy4tDfBr--nFZod9oDRyC1ckBY928M1HanWB4aVtasj8prZk0ZuAA (Untitled)
  Session 7: L5QBbwDEz8WpRNuEKFEiI8T-mpT3DDw4hNzdoOiLFNwH6X0E8ATKIiHt0_cChSkZGu6pDPy6f05Vu97oVFWNJg (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: c6g9Wc6xNQG4t0vw3-KHk9to_MrWWn-hmQltezQ5HO-Fs1G3J1lQ0w0scQUxawfC9Y8yyEwWmOHSjhA7qzHGaA

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Dict Creation ===
✓ Created record with plain dict: {'id': 'lXdfZ2ybhmH0erSfcQ0n06FoRxgnxWIKHO6lGw_hH-erg0XKJAlExmqHKN3aaBvoGIbi5G6LDheb8hVjUoEjgA'}

=== Upsert Operation ===
✓ Upsert (update existing record): lXdfZ2ybhmH0erSfcQ0n06FoRxgnxWIKHO6lGw_hH-erg0XKJAlExmqHKN3aaBvoGIbi5G6LDheb8hVjUoEjgA
✓ Inserted second record: 9f1R7JBgpZFgoO0aGIGUla_ERs5vSKH3IU9D5BTscKBzZ5saHX1N4xW_b29mXs_xS4mZR4tKtsEfptIhL5uLeA
✓ Upsert (update second record): 9f1R7JBgpZFgoO0aGIGUla_ERs5vSKH3IU9D5BTscKBzZ5saHX1N4xW_b29mXs_xS4mZR4tKtsEfptIhL5uLeA

=== Find One Operation ===
✓ Found user by email: {'age': {'type': 'Integer', 'value': 29}, 'email': {'value': 'alice.j@newdomain.com', 'type': 'String'}, 'id': 'lXdfZ2ybhmH0erSfcQ0n06FoRxgnxWIKHO6lGw_hH-erg0XKJAlExmqHKN3aaBvoGIbi5G6LDheb8hVjUoEjgA', 'active': {'value': True, 'type': 'Boolean'}, 'name': {'value': 'Alice Johnson', 'type': 'String'}}
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
   Inserted with ripple: {'id': 'zFc5tlxU9sIXG0wcSyV2Rprc4Wi4QBAoT-pj39n68S7I8h0F-bHutHU0N-H5qerTWCeMaey6wDWiQqB094SUCg'}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {'id': 'Al5u8eSv1Qdx41a1pm8pdZy_bJ62ZjbTbA2mK29tbkD3xlBme2CbAcX-6eexhhqZ8afkhMjuRT6ogTmpjUVlRg'}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {'price': {'type': 'Integer', 'value': 150}, 'name': {'type': 'String', 'value': 'Product 1'}, 'id': 'zFc5tlxU9sIXG0wcSyV2Rprc4Wi4QBAoT-pj39n68S7I8h0F-bHutHU0N-H5qerTWCeMaey6wDWiQqB094SUCg'}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {'name': {'type': 'String', 'value': 'Upsert Product'}, 'price': {'value': 500, 'type': 'Integer'}, 'id': 'zFc5tlxU9sIXG0wcSyV2Rprc4Wi4QBAoT-pj39n68S7I8h0F-bHutHU0N-H5qerTWCeMaey6wDWiQqB094SUCg'}

✅ All bypass_ripple operations completed successfully!
✅ [32mPython client examples complete![0m
