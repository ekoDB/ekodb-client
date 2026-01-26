make test-examples-python
🧪 [36mRunning Python examples (direct HTTP/WebSocket)...[0m
=== Simple CRUD Operations (Direct HTTP) ===

✓ Authentication successful

=== Insert Document ===
Inserted: {'id': 'hHYw6zgmzw0xqo6lFBRfGFDfMrhAUBoTzTQfDzVhmpZcrNOA4LTCYRMaTuGbxR5rXvLztXf0gUQ2ep9q8OaU8Q'}

=== Find by ID ===
Found: {'name': 'Test Record', 'value': 42, 'id': 'hHYw6zgmzw0xqo6lFBRfGFDfMrhAUBoTzTQfDzVhmpZcrNOA4LTCYRMaTuGbxR5rXvLztXf0gUQ2ep9q8OaU8Q', 'active': True}

=== Find with Query ===
Found documents: [{'active': {'value': True, 'type': 'Boolean'}, 'name': {'type': 'String', 'value': 'Test Record'}, 'value': {'type': 'Integer', 'value': 42}, 'id': 'hHYw6zgmzw0xqo6lFBRfGFDfMrhAUBoTzTQfDzVhmpZcrNOA4LTCYRMaTuGbxR5rXvLztXf0gUQ2ep9q8OaU8Q'}]

=== Update Document ===
Updated: {'name': {'type': 'String', 'value': 'Updated Record'}, 'id': 'hHYw6zgmzw0xqo6lFBRfGFDfMrhAUBoTzTQfDzVhmpZcrNOA4LTCYRMaTuGbxR5rXvLztXf0gUQ2ep9q8OaU8Q', 'value': {'type': 'Integer', 'value': 100}, 'active': {'value': True, 'type': 'Boolean'}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: CUYleyzUq5ERksJhs2_ex0xZMacukDb7Fahybmg6_kYNBoaiKUftv-iDrdcvmQeuPACaYe-iqENgAa27grrarg

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
        "id": "CUYleyzUq5ERksJhs2_ex0xZMacukDb7Fahybmg6_kYNBoaiKUftv-iDrdcvmQeuPACaYe-iqENgAa27grrarg",
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
        "id": "BRWJqswv00Jc4ldtHNBYkzPv1XIMTVaTFLjDm0FPXH0wMIOwGp6SL1oZIOUqlv30gPRzSi7wdyjSsdR6-gH8Tg",
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

✅ Script saved: W-y0jvUaXDCVZAkwHuZG9Uk9yaU-jnDIaNNN8Y7rPN2v44VpHNdoVH2xUbDHhskFGZdIDKOZYQvAwUN0Bv_jHA
📊 Found 35 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: eFzAj__SnLF-W8fNt0jrDkw0NRNCDh60J3kP8bd4-xUZfaf6xEuT53w5cqyZw04OtpL-fCn0qkXwt0eZb3zDCg
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: 1nNal6n3RxMF9kc9RuXubbUUsVy4VWMIooT-gCALJuyX5lE0CjcV1cREj46RP04ZY1A52ZAsHaPO3VbyCjJkRQ
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {'avg_score': 50.0, 'max_score': 90, 'status': 'inactive', 'count': 15}
   {'status': 'active', 'max_score': 100, 'avg_score': 60.0, 'count': 15}

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
Collection created with first record: m5VHlZw85SQoUH54Kas7GScbGnmxiVieW0Rm6q0T0WlSiDy_JxthT6XYxC5kd9aftnq8frKOevnGNw1DXK4a_Q

=== List Collections ===
Total collections: 12
Sample collections: ['ws_ttl_test', 'test_collection', 'chat_configurations__ek0_testing', 'demo_collection', 'scripts__ek0_testing']

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: False

✓ All collection management operations completed successfully
✓ Authentication successful

📋 Getting original configuration...
   Original durable_operations: True


============================================================
🔥 TEST 1: Original Config (durable=True)
============================================================

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: Fsb0Wuihh43JcEqlNi5ka-p4MBKDVDx5XBN_sLLf7DcVbTQCzwdUpJzLHXBnMHFOvao_l3Stk97C7CtiLQvFUw
Created Bob: $500 - ID: 9SXSYd1Die3zShpNDP7vGsI9zfOuzOvLsknOVSycDabXijvIo1HIbBICtmRhStZwmFlineBc3Yxv1QwCNfy4Vw

=== Example 1: Begin Transaction ===
Transaction ID: 356afa86-4c8c-4684-8e44-02b27528c6bd

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
New transaction: 882317b1-b6ab-44f7-a540-1486a79aa8d2
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: aFtySlNxY7BiNalf_y67v-0olAn92L3zQzQYpQX9SiDEeVgrb_ljq6Ss-EMxgyRFxxmznpV1G9TDsFiOwA7myQ
Created Bob: $500 - ID: -gTvzaQ6NTeXBf7i9WsgvWHmmVL73mgqOMFagcXzJaVgmFCbhNIbjErvZPLdsG0FYb-PmHITTxbK5wUuqXOCCA

=== Example 1: Begin Transaction ===
Transaction ID: e014de72-da36-4b1f-9101-89f0c04df6f0

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
Created Alice: $1000 - ID: zsHKG6278DBM7_sPvXjJhPSQJlkESqSam_reFBM6ilX81ZWqpqGYJWUDrg6QHaS3isrgWt73G27bWNXlXj0yiA
Created Bob: $500 - ID: DOAFCY3DWhL9eot9jRKMOmw0-OjgMRALPJ2RwgYTOYnMXbk8TtUlY5S0LlQ2PwAkuGBvkfJafyDRe8NNA6yZKQ

=== Example 1: Begin Transaction ===
Transaction ID: ae25215b-3eb8-4155-9dcc-e8bf5c60fdf5

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
New transaction: 1bdcce8b-df20-4c8d-98bf-8757d5b259d4
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 3F4HByHUSdPPpTLykRABQWFGg65v-rHf4gmySa6qGf8p8q1Q5WNtV_NExHtFM2cMHXxOQXQLtYyoTCcUS7qCuA
Created Bob: $500 - ID: lCCw03dcMjKPQIYp_BWcO0xaUqp3SzrTQZjD1apSnpLP9XPNLIRQ4Dx0AoXk29jnSxCAstaSqOuy86ViUyUvCg

=== Example 1: Begin Transaction ===
Transaction ID: 12b8235f-ed89-4b8b-84ec-85545737aead

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
Created Alice: $1000 - ID: GRF55rUfrYBPi7QBVguS-CW8OgY70DLOFaH-69ZpmwyuQMRpppW26YchdL-jVyPwxFXv5Omc322b3H0ZQxf6fg
Created Bob: $500 - ID: y1oXflHa0yeoqqekjrE70nqaOpxHlPc_-yvle4c6FAttfGzYQzVzIp5k6oEHT6m7b1b89Yr5P2AfY4OyyvPJog

=== Example 1: Begin Transaction ===
Transaction ID: 8e7232fa-44f5-4112-9518-2db080797a69

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
New transaction: 20acaf75-5ed3-4842-9cee-96ca167d061e
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: Ly5lvU4Xe0oOxoGUoITBinAPGs_xM3p4palZ8t44ARcUK5RRSW5Z8f2HFwkTpsQ_8UcFsqx5C7VC4d1-tT0rIw
Created Bob: $500 - ID: 0ltb3ksCHCR6tZWiON-mPKof91F60nu3mrzaZGON8fvhcPF-tCSWyyhg2Aiu0WrxENUdhiRJQN5yucdqXsxnmg

=== Example 1: Begin Transaction ===
Transaction ID: e5817ded-49f9-4e69-b410-e418a2ab35fe

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
   ✓ Config restored: durable_operations=True


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
   ✅ Script saved: Xe-SC6WFFsUk13AaywXNQTuKahbc_bk3_Tx1AFIaMZ7ysSVGK7bZMWmFLfgfqkfGdpMpaV6HlNo4r-FPGWM9xA

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: vO-EVJtdSwwg_E4N_vDCtad-v3VfJDJs8r3f9r_Q6aHi_YO6OfOxeNz-sJvf0ASJQ9jgz2fXU3drnoh9fnsdjw
   📋 Name: {'value': 'Alice Smith', 'type': 'String'}
   📋 Email: {'type': 'String', 'value': 'alice@example.com'}
   📋 Status: {'value': 'pending', 'type': 'String'}
   📋 Credits: {'value': 0, 'type': 'Integer'}

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: seCSnYgoK5cQ2jwZhVlEphhPZXPqJ3aqq_Zrcn_UzfoYGY0fY84vBlBa4z4wjt1LsPaTijibzTt5nTdMGp4mcw

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
   ✅ Script saved: M1gVGqndOxklWMGVTOdJNn-qHSovyQOpK8HFHyJeKAKao_1Kkoemt0WBOcxIuyRPxOuH4_gWf_W_pUrOomuCKw

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
   ✅ Script saved: lLOlpWPISwBN1b36xvrsEhH7Y3_NhBTe7qNZT2cijZKOuoGanBIFPAxyxRuXx0SkEkYaTY0ZzsvY72FokrJhmA

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: Xe-SC6WFFsUk13AaywXN...
   ✅ Deleted script: seCSnYgoK5cQ2jwZhVlE...
   ✅ Deleted script: M1gVGqndOxklWMGVTOdJ...
   ✅ Deleted script: lLOlpWPISwBN1b36xvrs...
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
  Output: Document ID = TXbr4lqe3sP8bUkIs1pvYkxiSGatDa7JT-g6wD28HSTpPPLmqDBDY8sOX3pmYPdtaPtZGLY1gAWV0COk0ltENw
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: find_by_id(TXbr4lqe3sP8bUkIs1pvYkxiSGatDa7JT-g6wD28HSTpPPLmqDBDY8sOX3pmYPdtaPtZGLY1gAWV0COk0ltENw)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: find_by_id(TXbr4lqe3sP8bUkIs1pvYkxiSGatDa7JT-g6wD28HSTpPPLmqDBDY8sOX3pmYPdtaPtZGLY1gAWV0COk0ltENw)
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
  Output: Document ID = JVF3gYmIZhd5LI2AmtrLn80BkyRTFkJp5f3N-hdJSjWZMakukWG4W-lo8TfJUDleKyTchWdMswlx01NApQsCgA
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: find_by_id(JVF3gYmIZhd5LI2AmtrLn80BkyRTFkJp5f3N-hdJSjWZMakukWG4W-lo8TfJUDleKyTchWdMswlx01NApQsCgA)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: find_by_id(JVF3gYmIZhd5LI2AmtrLn80BkyRTFkJp5f3N-hdJSjWZMakukWG4W-lo8TfJUDleKyTchWdMswlx01NApQsCgA)
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
    Finished `release` profile [optimized] target(s) in 0.08s
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
Inserted: {'id': 'PfTZMeURj3GGEafVnIA_Yaut-h6gFYQvIWOYwAg0CNQECk57mWRAzdhGUfZGoFvl_gDtkrMXmQ-XKj7lDZn0rQ'}

=== Find by ID ===
Found: {'data': 'aGVsbG8gd29ybGQ=', 'metadata': {'key': 'value', 'nested': {'deep': True}}, 'price': 99.99, 'created_at': '2026-01-26T18:24:34.982586', 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5], 'active': True, 'value': 42, 'user_id': '550e8400-e29b-41d4-a716-446655440000', 'categories': ['electronics', 'computers'], 'id': 'PfTZMeURj3GGEafVnIA_Yaut-h6gFYQvIWOYwAg0CNQECk57mWRAzdhGUfZGoFvl_gDtkrMXmQ-XKj7lDZn0rQ', 'tags': ['tag1', 'tag2', 'tag3'], 'name': 'Test Record'}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): True
  price (Decimal): 99.99
  created_at (DateTime): 2026-01-26 18:24:34.982586
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): ['tag1', 'tag2', 'tag3']
  metadata (Object): {'key': 'value', 'nested': {'deep': True}}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): ['electronics', 'computers']
  data (Bytes): 11 bytes
Plain record: {'data': 'aGVsbG8gd29ybGQ=', 'metadata': {'key': 'value', 'nested': {'deep': True}}, 'price': 99.99, 'created_at': '2026-01-26T18:24:34.982586', 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5], 'active': True, 'value': 42, 'user_id': '550e8400-e29b-41d4-a716-446655440000', 'categories': ['electronics', 'computers'], 'id': 'PfTZMeURj3GGEafVnIA_Yaut-h6gFYQvIWOYwAg0CNQECk57mWRAzdhGUfZGoFvl_gDtkrMXmQ-XKj7lDZn0rQ', 'tags': ['tag1', 'tag2', 'tag3'], 'name': 'Test Record'}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'data': {'type': 'String', 'value': 'aGVsbG8gd29ybGQ='}, 'id': 'PfTZMeURj3GGEafVnIA_Yaut-h6gFYQvIWOYwAg0CNQECk57mWRAzdhGUfZGoFvl_gDtkrMXmQ-XKj7lDZn0rQ', 'price': {'value': 99.99, 'type': 'Float'}, 'name': {'type': 'String', 'value': 'Updated Record'}, 'value': {'type': 'Integer', 'value': 100}, 'created_at': {'value': '2026-01-26T18:24:34.982586', 'type': 'String'}, 'categories': {'type': 'Array', 'value': ['electronics', 'computers']}, 'active': {'type': 'Boolean', 'value': True}, 'embedding': {'value': [0.1, 0.2, 0.3, 0.4, 0.5], 'type': 'Array'}, 'tags': {'type': 'Array', 'value': ['tag1', 'tag2', 'tag3']}, 'user_id': {'value': '550e8400-e29b-41d4-a716-446655440000', 'type': 'String'}, 'metadata': {'type': 'Object', 'value': {'key': 'value', 'nested': {'deep': True}}}}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: O7KhC_M3zaVzx4a482CT9TnY4uIFjR9OVyDYcsmwn52LL5cfOveE4F40-tSjdaZrR4sL04RXEg-1Z3jQjda-Lw

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
Collection created with first record: "C6qma96Llk01iZt_PeFbNWG8CJc04GcX1yR80IIqf_zlm4i6yvY6LoA00cxADEXWHCUqUHly_Pvz0UOUg-ZcHw"

=== List Collections ===
Total collections: 11
Sample collections: ['ws_ttl_test', 'test_collection', 'chat_configurations__ek0_testing', 'scripts__ek0_testing', 'client_collection_management_python']

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

=== KV Batch Set ===
✓ Batch set 3 keys
  cache:product:1: success
  cache:product:2: success
  cache:product:3: success

=== KV Batch Get ===
✓ Batch retrieved 3 values
  cache:product:1: {'price': 29.99, 'name': 'Product 1'}
  cache:product:2: {'price': 39.99, 'name': 'Product 2'}
  cache:product:3: {'price': 49.99, 'name': 'Product 3'}

=== KV Exists ===
Key exists: True

=== KV Find (Pattern Query) ===
Found 3 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 9

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: False

=== KV Batch Delete ===
✓ Batch deleted 3 keys
  cache:product:1: deleted
  cache:product:2: deleted
  cache:product:3: deleted

✓ All KV operations completed successfully
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: PcHBNET5A9gWF3_8D0VrkwRCQTFd1ryhB4AgENVTeetg9fa_LiFTAYEKOsDfRf0b8WjPlfz1WYnjOnCg8FkztA
Created Bob: $500 - ID: 4aipvtRQIhGO2XXAPOA9cRePqKHRbqaNpKtYN_LgNRreug1sacpnaXY-MHvQkIi-YX6WCU1KDZbfHLzZlGchgg

=== Example 1: Begin Transaction ===
Transaction ID: 42f57bfc-31d4-47c4-b410-676c5f93556f

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 9f0c86ee-52b6-4b49-b16c-b55a86552e8b
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
  1. Score: 25.740, Matched: name, name.value, email.value, email
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: bio.value, bio, title.value, title
  2. Score: 26.400, Matched: title, title.value, bio.value, bio
  3. Score: 26.400, Matched: title, title.value, bio, bio.value
  4. Score: 26.400, Matched: bio.value, title.value, bio, title

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title, bio.value, title.value, bio
  2. Score: 39.600, Matched: bio, title.value, bio.value, title
  3. Score: 39.600, Matched: title, bio, bio.value, title.value
  4. Score: 39.600, Matched: bio.value, bio, title, title.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 2 results (matches: run, running, runs)
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio.value, bio

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.766
  2. Score: 0.755
  3. Score: 0.741

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.370, Matched: content.value, content, title.value, title
  2. Score: 1.377, Matched: title.value, title, content, content.value
  3. Score: 0.383, Matched: 

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
✓ Inserted document: alxAbBvnFUE3buj-LlCvSTaYAzL2I4-wqI_noELLJ_VeU_Gze3HRzx6U0cHiGWlvwo2WcEdipWiTmUgpNUNl2Q

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: fcxv0EdRJuWCy_UgwRBi1nSIB-9Ya1LNspnGXYrSgFfK5zufRsaWh23NZWEBGZCECDLh89Ea7yTp7etMqfxUnw

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
✓ Inserted document with TTL: jFc4mThjbuYBRvuS6Wz6ysFQqb5sHYqnV_We-TZ4UX9xAmm38EZvigVA_CrJn1yqg3MPgx-R6NUE3VXiFI8cMA

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
✓ Edge cache script created: OiA_oJZG2HkaZVSymKqcZ2aUup0YzB71KO-d4d8wbvMmmtpkI2z2TiPa7FGjI-Lqnuz65AB0Bs2IQ_rBGg3yRQ

Call 1: Cache miss (fetches from API)
Response time: 2ms
Result: {
  "records": [
    {}
  ],
  "stats": {
    "execution_time_ms": 0,
    "input_count": 0,
    "output_count": 1,
    "stage_stats": [],
    "stages_executed": 2
  }
}

Call 2: Cache hit (served from ekoDB)
Response time: 2ms (1.1x faster!)
Result: {
  "records": [
    {}
  ],
  "stats": {
    "execution_time_ms": 0,
    "input_count": 0,
    "output_count": 1,
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

✅ Script saved: 7OepqkX1Y_BU4AJXjeH7zd-sUqNbdFcPWTGk81vRYIgU3GmToxFNte3Hmm2mv9HGNixYVOMlDid7BzQbWOlItw
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved: JTSJOwGzsouCHEpa9WmJkluGXwXCmkadtOPD9CIIfOazmPM_aarttroNlXkOQcwI5Uta_8n2MmA3pNTeB3wtxg
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: i15V8j0cfTq8FKJJm-jhHGFeC3FajJLHnIcPprfkXKmw-n00ozUIGv80VRQ63P4ey82LNTyvui8xOufI8upiYQ
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

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 1.6ms
   📊 Records: 1
   📦 Data: {
      "value": {
            "address": {
                  "city": "Gwenborough",
                  "geo": {
                        "lat": "-37.3159",
                        "lng": "81.1496"
    ...

Second call (cache hit - from cache):
   ⏱️  Duration: 1.6ms
   📊 Records: 1
   📦 Data: {
      "value": {
            "address": {
                  "city": "Gwenborough",
                  "geo": {
                        "lat": "-37.3159",
                        "lng": "81.1496"
    ...
   🚀 Cache speedup: 1.0x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

✅ Level 1 function: validate_user
✅ Level 2 function: fetch_slim_user (calls validate_user)
✅ Level 3 function: get_verified_user (calls fetch_slim_user)

📊 Result from 3-level nested composition:
   Records: 1
   Name: User 1
   Department: engineering

🎯 Key Benefit: Each function is independently testable and reusable!
   - validate_user: Used in 100 different workflows
   - fetch_slim_user: Used in 50 workflows
   - get_verified_user: Specific workflow


✅ All composition examples completed!
🚀 ekoDB Python Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved: zP1YlROqOpWHoVt7C9E9fZe6hFDzNrwzybfxXkbD_9xxUY-1VxEFIWD6pjJoVRSfi9R1f1ZFw1QPbTEOFP5xAg
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
🚀 ekoDB Python KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: Ds3Rf9mQPaqt3wQLqOoJ7Tl0B4S2jshXEnYs2nKtL2GscJIqfKT-ydIqjzpitmFaqyEj3D96qLVSN3963c02_Q
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

✅ Inserted order: jarSxRthkMb94u51pRsJ3TkWrFJmSzbEMcCAcm3pK3f0LuBAa5Xy2T8qtq1mXR5iwINV04A2hr4IjEtwFNVl4w
✅ Cached order status
📊 Quick status lookup: {'value': '{"status":"processing","updated_at":"2026-01-26T23:24:36.846346+00:00"}'}

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
✓ Created SWR script: fetch_api_user_py (LGRC-pwueab0MZK_hTvc4EiXmo6iq7jwdHzxV82kjzYnV21q67rXHjAmB03DmX3j_U1c8shX2CfCxlTlmXyC4Q)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "address": {
          "city": "Gwenborough",
          "geo": {
            "lat": "-37.3159",
            "lng": "81.1496"
          },
          "street": "Kulas Light",
          "suite": "Apt. 556",
          "zipcode": "92998-3874"
        },
        "company": {
          "bs": "harness real-time e-markets",
          "catchPhrase": "Multi-layered client-server neural-net",
          "name": "Romaguera-Crona"
        },
        "email": "Sincere@april.biz",
        "id": 1,
        "name": "Leanne Graham",
        "phone": "1-770-736-8031 x56442",
        "username": "Bret",
        "website": "hildegard.org"
      }
    }
  ],
  "stats": {
    "execution_time_ms": 0,
    "input_count": 0,
    "output_count": 1,
    "stage_stats": [],
    "stages_executed": 2
  }
}
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 3ms (served from cache)
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
   {'avg_price': 365.6666666666667, 'category': 'Furniture', 'count': 3}
   {'avg_price': 367.0, 'category': 'Electronics', 'count': 5}
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
   Vector databases offer several benefits such as high precision in data representation, capability to store complex data structures, easy scalability and manipulation of data, suitability for geographic and spatial data, and efficient storage by eliminating redundancies.
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
   2. Database Design Principles (Database)
   3. Introduction to Machine Learning (AI)
   4. Getting Started with ekoDB (Database)
   5. Vector Databases Explained (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Script saved
📊 Documents by category:
   {'category': 'Database', 'count': 3}
   {'category': 'AI', 'count': 2}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: 4-V04W3imEKGCRZjDetm--x5I8nlgyhbglc0O0ne_DpXqOAoTm4N6TjYOBYebjqZSDug4o_S4hpEvTDgsa8NVg

=== Sending Chat Message ===
Message ID: g2unJwOGKEeuYMVCAkMalsGFESGJiDjYHR1DbPShOvZmbZFkMmqpzJ2EXTltiQsykl6SdwV0ghATzvAG5wibzw

=== AI Response ===
We have three products available:

1. ekoDB Cloud - This is a fully managed cloud database service. The price for this product is $499.

2. ekoDB - This is a high-performance database product with AI capabilities. The price for this product is $99.

3. ekoDB Pro - This is an enterprise edition product with advanced features. The price for this product is $299.

Execution Time: 2584ms

=== Token Usage ===
Prompt tokens: 448
Completion tokens: 83
Total tokens: 531

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: aLWyLazou-2Zn1lvSQjQtGqqWPJ2xbxA3JdcFWV9UpfnEAAHKL6WIL6mPRhkBMG46x7twdIGDncgnrDVatFEDg

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information available, we have the ekoDB product, which is a high-performance database product. It is priced at $99.

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['context_snippets', 'created_at', 'token_usage', 'chat_id', 'updated_at', 'content', 'id', 'role'])
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
✓ Created second session: MjMzthfgQkK_RCZ5U_aYC-J-lAwwz_GwyqBHLHRntDFGFW8elnqNQCAvkbPZ32JJA4sn7fvaUDNmK7HDcAzdAw
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
✓ Created session: Vjg7gbLn2PTz6aASV1V2HqucyY9_0-mbJorwf-OYI1bz4n56cRj5AiDkY4LOfEIdWts1LdYVp-ZI7Fs0X7TGCg

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product based on the context provided is "ekoDB". It is a high-performance database product. The price of this product is $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: LVlBmUcvz-vLSHi2HTWLn2VOyjrcqwNKPRAfODUXZ3PjAp5o7D9YKbeukuFYASnM8sZtvF0PIlCXcmrxbx4KtA
  Parent: Vjg7gbLn2PTz6aASV1V2HqucyY9_0-mbJorwf-OYI1bz4n56cRj5AiDkY4LOfEIdWts1LdYVp-ZI7Fs0X7TGCg

=== Listing Sessions ===
✓ Found 7 sessions
  Session 1: LVlBmUcvz-vLSHi2HTWLn2VOyjrcqwNKPRAfODUXZ3PjAp5o7D9YKbeukuFYASnM8sZtvF0PIlCXcmrxbx4KtA (Untitled)
  Session 2: Vjg7gbLn2PTz6aASV1V2HqucyY9_0-mbJorwf-OYI1bz4n56cRj5AiDkY4LOfEIdWts1LdYVp-ZI7Fs0X7TGCg (Untitled)
  Session 3: MjMzthfgQkK_RCZ5U_aYC-J-lAwwz_GwyqBHLHRntDFGFW8elnqNQCAvkbPZ32JJA4sn7fvaUDNmK7HDcAzdAw (Untitled)
  Session 4: 4-V04W3imEKGCRZjDetm--x5I8nlgyhbglc0O0ne_DpXqOAoTm4N6TjYOBYebjqZSDug4o_S4hpEvTDgsa8NVg (Untitled)
  Session 5: 5QxYKUFTMmEPqanYhCgr6qW5GS16WR1B8OBf27ZhsDvNl5nyCnRYlQrSxnhpECUbLp6GSLR16YjBt-2IJ9PsGw (Untitled)
  Session 6: Jl4mTFpz00jWsGudEumneFR__XNqOT5dDRYYYMoqAxV6dp57xjbgUb5ihLSePZv2Ja1dspdRuGNIayfv4oEYiw (Untitled)
  Session 7: Dg8xiBnwBaVnvN7d4DIqKabLRHXxd9EOqXcGWdc16Tpz1RhUKGt1zSSxCy1IsQFvAn6SFcZ1iR5Ci5LGyCQvyQ (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: LVlBmUcvz-vLSHi2HTWLn2VOyjrcqwNKPRAfODUXZ3PjAp5o7D9YKbeukuFYASnM8sZtvF0PIlCXcmrxbx4KtA

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Dict Creation ===
✓ Created record with plain dict: {'id': 'JVvr2kvZfwAS7HoAs_K4iHQkpidI6IE8yBR3UOPtRoWdRs6Mp-KuhZj_t7L3t17R4ad5s6dZqGo1jQ3sP86QrQ'}

=== Upsert Operation ===
✓ Upsert (update existing record): JVvr2kvZfwAS7HoAs_K4iHQkpidI6IE8yBR3UOPtRoWdRs6Mp-KuhZj_t7L3t17R4ad5s6dZqGo1jQ3sP86QrQ
✓ Inserted second record: Gn0nkxvtqKxJEES_oTioDumbuI8-qq2QH9im_fjrum_Rog_F7Rt2PWErCQ3cSIF6uxaKmLQcnlgtbWiuX5r0pw
✓ Upsert (update second record): Gn0nkxvtqKxJEES_oTioDumbuI8-qq2QH9im_fjrum_Rog_F7Rt2PWErCQ3cSIF6uxaKmLQcnlgtbWiuX5r0pw

=== Find One Operation ===
✓ Found user by email: {'active': {'type': 'Boolean', 'value': True}, 'age': {'type': 'Integer', 'value': 29}, 'email': {'type': 'String', 'value': 'alice.j@newdomain.com'}, 'id': 'JVvr2kvZfwAS7HoAs_K4iHQkpidI6IE8yBR3UOPtRoWdRs6Mp-KuhZj_t7L3t17R4ad5s6dZqGo1jQ3sP86QrQ', 'name': {'type': 'String', 'value': 'Alice Johnson'}}
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
   Inserted with ripple: {'id': 'TDAFdiMbsQVKiYRD7a7ycpJqiVsShJo2AqK7cU-fD0e6GydDUnATLKScMjbXNNWPZvSq3bkHe16OMcBhM9OpBw'}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {'id': 'D3w420hJ1SXSjOsMcHOUYBP-cwy3au94uEEvlv9IiL8O7lEjg-uZ8j4U4YyV184A_xwcigFi1rLNInovO3i8VA'}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {'name': {'value': 'Product 1', 'type': 'String'}, 'price': {'value': 150, 'type': 'Integer'}, 'id': 'TDAFdiMbsQVKiYRD7a7ycpJqiVsShJo2AqK7cU-fD0e6GydDUnATLKScMjbXNNWPZvSq3bkHe16OMcBhM9OpBw'}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {'name': {'value': 'Upsert Product', 'type': 'String'}, 'price': {'value': 500, 'type': 'Integer'}, 'id': 'TDAFdiMbsQVKiYRD7a7ycpJqiVsShJo2AqK7cU-fD0e6GydDUnATLKScMjbXNNWPZvSq3bkHe16OMcBhM9OpBw'}

✅ All bypass_ripple operations completed successfully!
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields
Fetched 3 users with only 3 fields each

Example 2: Exclude sensitive fields
Fetched 2 admins without sensitive data
  Password field excluded: False

Example 3: Complex query with projection
Fetched 3 active users with profile fields

Example 4: Find by ID with projection
Fetched user profile: {'value': 'Alice Johnson', 'type': 'String'}

Example 5: Compare full vs projected data
Full query returned 12 fields per user
Projected query returned 12 fields per user

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
✅ [32mPython client examples complete![0m
