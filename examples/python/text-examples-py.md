make test-examples-python
🧪 [36mRunning Python examples (direct HTTP/WebSocket)...[0m
=== Simple CRUD Operations (Direct HTTP) ===

✓ Authentication successful

=== Insert Document ===
Inserted: {'id': 'b2a9PAJf4xLLn7FR4bMCTYOulr7F7ZjqPchHIUrlPaQU68z-s8J6PHHqBldg1QWVzkV1f-q8Kx_L6XyTGRfO-w'}

=== Find by ID ===
Found: {'id': 'b2a9PAJf4xLLn7FR4bMCTYOulr7F7ZjqPchHIUrlPaQU68z-s8J6PHHqBldg1QWVzkV1f-q8Kx_L6XyTGRfO-w', 'active': True, 'value': 42, 'name': 'Test Record'}

=== Find with Query ===
Found documents: [{'value': {'type': 'Integer', 'value': 42}, 'active': {'type': 'Boolean', 'value': True}, 'name': {'type': 'String', 'value': 'Test Record'}, 'id': 'b2a9PAJf4xLLn7FR4bMCTYOulr7F7ZjqPchHIUrlPaQU68z-s8J6PHHqBldg1QWVzkV1f-q8Kx_L6XyTGRfO-w'}]

=== Update Document ===
Updated: {'name': {'value': 'Updated Record', 'type': 'String'}, 'id': 'b2a9PAJf4xLLn7FR4bMCTYOulr7F7ZjqPchHIUrlPaQU68z-s8J6PHHqBldg1QWVzkV1f-q8Kx_L6XyTGRfO-w', 'value': {'value': 100, 'type': 'Integer'}, 'active': {'type': 'Boolean', 'value': True}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: xqlWFz-xcGkzGrnXOkLAPRhrmrsYJH2KINL4W-8E2QvjvbU_O7jY19A6D8om-LLjx6BB85QBiBvAz0MPk3ORjg

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
        "id": "XynUeQSPaTfp_nzYBxA4ynDonvZIjV5Ok9f_VEPpIkitQWuzrXR_nHJ9cd9NPM7dOh-40i2EZzq0MpmA7LiPeQ",
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
        "id": "xqlWFz-xcGkzGrnXOkLAPRhrmrsYJH2KINL4W-8E2QvjvbU_O7jY19A6D8om-LLjx6BB85QBiBvAz0MPk3ORjg",
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

✅ Script saved: AJFRfWZmwnzeY37Slt7VbeklV1rt1KnDPSPl_lx8MxbM2_kt1bHnswmcUoTio0eP1PumEZViyLX7KjkMZTnKPQ
📊 Found 35 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: UgBdcEnHAO3MiRrRjIFG2_HCHnGpb1tkEVHt4sMKSexnEI-g4jtybRy7V3mZDc60N2RFvdtQeDlgE3GgNZZxwA
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: DkP1rlWoRL1Ul1y-3m5IkUGGLm66-k1fR2DujLuNLAKSbJU3SQ4cLtu0CS1nX0GxpaO1ZHjH6sGtC0pL--HLBA
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {'avg_score': 50.0, 'count': 15, 'status': 'inactive', 'max_score': 90}
   {'status': 'active', 'count': 15, 'avg_score': 60.0, 'max_score': 100}

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
cache:product:1: {'name': 'Product 1', 'price': 29.99}
cache:product:2: {'price': 39.989999999999995, 'name': 'Product 2'}
cache:product:3: {'name': 'Product 3', 'price': 49.989999999999995}

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
=== Collection Management (Direct HTTP) ===

✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: Qt4XcDYpJQRr9ZuM3VgENn--bkjoILfw47PLcERuW5Xk7uhDZcujE9YDWWY34YgVMd2LaXSXFk1yZR2u4h85ow

=== List Collections ===
Total collections: 13
Sample collections: ['chat_configurations__ek0_testing', 'chat_messages__ek0_testing', 'demo_collection', 'scripts__ek0_testing', 'batch_users']

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
Created Alice: $1000 - ID: DKAODFSqVWzfuFMFeQNqF_BgzYaPQkMiSHnvMBYQosiM67e6HJ0638fKiaVot5DiV1V-HhvyX1FjbtOZn54cjQ
Created Bob: $500 - ID: SsGfPdpe1wgvsANrpMVcbNibaLZaU4bBXswiE1gvW-MoeMmEFrPE_K05XCsH9ug0nNboT08NitWn-gT1wUOW-Q

=== Example 1: Begin Transaction ===
Transaction ID: 2214f0c4-c815-40c1-a0c0-d19cdcd9255b

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
New transaction: 38cd3186-8002-4875-bda0-94a0067f103d
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: NRe8z46Q7OK7K-vAlP7YOO15NXCwwQI0zteCSD4DTr-6Z5z3o_c05IdYYAKu3Sd2B3QAUt1k9KhJ77i2EORJJw
Created Bob: $500 - ID: zeb6xV5H3EMUhEDcv0kRxJGjrC-5Fv4LpLwDVnLIJksoj0PKDLMvS-iiKZKUZZR8xaFo8CkF8c4JBDEjF-3zLQ

=== Example 1: Begin Transaction ===
Transaction ID: d1fd19d4-2455-455d-9e66-38d58fe7cfc0

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
Created Alice: $1000 - ID: IsmVagbSmD0EUt7zntcOr934pSULojp-7qTzSTIAoNUntHa302-TN5WcQT3t6dF7i4LXHfYk7L-4ejEHImFpqg
Created Bob: $500 - ID: ZsJypxq8YxB3EgC6ggb2TfGqmybCax1ad7_WW_nIUbk5pjaRwKwDftXA5mhrtFm_NmEDlB6mbIIrUFLrOHeDbw

=== Example 1: Begin Transaction ===
Transaction ID: a5077aa2-c28f-4651-8f44-e78fb7d2ba86

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
New transaction: 51432369-79ca-4b28-8eab-a6f074dfb76a
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: zHaLkWX2zXrBDhPpdtHqLEMfssPQ81EqfMDLzPNYIUn_-7A5H400YcPR_2giV1OU-13vzszmZDBtZmU7SMXDxw
Created Bob: $500 - ID: 2D2LewRgiYEZJwkVbKo8SA-r7O6aQaiRTxZCtwKDpf4otuJxhYpr7iEO8tbhs8x0Huc2yWCWveREElvlsXF97g

=== Example 1: Begin Transaction ===
Transaction ID: 39407dcc-c96b-4cfe-9613-d6af9a74f3d6

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
Created Alice: $1000 - ID: BB27UmnUpB2RhH8rILZr89KwhJ2np86JXryJOdwSwr3m-ZjmRT-GRVgL_I3nXppLz55NTWA6BP4bRIdRJx8lCw
Created Bob: $500 - ID: Xv3uc1Y8jfL77Mn9BZ6lzJVyX6S88Z9tZDJkIG44trFlHJNJCOFfIxPtDA0OJA8SCm4tGZ945QBTuK7KOrtTdw

=== Example 1: Begin Transaction ===
Transaction ID: f277e804-7c86-403a-8a26-2c45423c1009

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
New transaction: 55bc3766-659f-4543-8217-610bae52dfbe
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: diOidDIBcVd2gwS3en-bsBbktZwyJG7n_cVGU3vrgwyifbPiy-iFARZRpc3uR6JoALKypz_uVXDFgZv6fJBLKQ
Created Bob: $500 - ID: DXRK2UzjJMs2LPAO5t6x79JP6eJ6UsOCHJYEFE3gZMaypZN84-IEHOc22ZGxmcDgc4rY0p8BrLwKk1qyLnutbA

=== Example 1: Begin Transaction ===
Transaction ID: 80a79e82-7319-470b-94c8-1051314bb5d7

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
   ✅ Script saved: SJvvWkF9SUvkTEe1MBo1MXbjkri_-4qP0sNFi2zAq1q0ieyWrfZcmM9nvRA2474XJ08YFYB9V4wzE8azzamj1g

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: xhZtvlL4StLCtvE3HqHZ-yF7AxqWSnBsVM_HSypuQWI55iBrRAQttjeDo6H4D1FEdX8q5vpVfLpxOtkj_TmJcw
   📋 Name: {'value': 'Alice Smith', 'type': 'String'}
   📋 Email: {'type': 'String', 'value': 'alice@example.com'}
   📋 Status: {'value': 'pending', 'type': 'String'}
   📋 Credits: {'type': 'Integer', 'value': 0}

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: epWuBeWMtVqo_FivTVy3_meRdq1MOastSVdy3KXNhIAdTJiur-4GICR1jl-t8I5HWmbP-PobMYraPqeTaPuIFw

2️⃣ Calling Script (Query + Update + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Status updated to: {'type': 'String', 'value': 'active'}
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}

============================================================
📝 Script 3: Query + Update Credits + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: heN2pt9qLfc9AwUP3MGW1PMOhhLkUxZc-XTz6_G4Rjl-h-pIoQ-StgD8QN-wR41XDARAMu9wWFky-dDZw7-O2Q

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
   ✅ Script saved: lsJlHLg83ORYecl4lTnVKa4KyexfHWLrxRZ2228_e_5c25UxU62b5E1rFJqJVBYDYdcB0Ll2u8fWAC79D_cy8w

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: SJvvWkF9SUvkTEe1MBo1...
   ✅ Deleted script: epWuBeWMtVqo_FivTVy3...
   ✅ Deleted script: heN2pt9qLfc9AwUP3MGW...
   ✅ Deleted script: lsJlHLg83ORYecl4lTnV...
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
  Output: Document ID = WLvu5PLwz7ZA-tFS1eOjBafhUOy7gBCCL7ip0_1DFr-vUrqGknP8MEUrlY2Ai8wyKsRAZGRr5-1l6ElAlu53RA
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: find_by_id(WLvu5PLwz7ZA-tFS1eOjBafhUOy7gBCCL7ip0_1DFr-vUrqGknP8MEUrlY2Ai8wyKsRAZGRr5-1l6ElAlu53RA)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: find_by_id(WLvu5PLwz7ZA-tFS1eOjBafhUOy7gBCCL7ip0_1DFr-vUrqGknP8MEUrlY2Ai8wyKsRAZGRr5-1l6ElAlu53RA)
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
  Output: Document ID = LR5U5LBObbGp84EN1mHwLlXkkzNRyfD2r9cIeoLY2jsB2oRXp6FMYNqCJS5X8FqW4bEVj-qbmgh1KIAe0E-NEw
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: find_by_id(LR5U5LBObbGp84EN1mHwLlXkkzNRyfD2r9cIeoLY2jsB2oRXp6FMYNqCJS5X8FqW4bEVj-qbmgh1KIAe0E-NEw)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: find_by_id(LR5U5LBObbGp84EN1mHwLlXkkzNRyfD2r9cIeoLY2jsB2oRXp6FMYNqCJS5X8FqW4bEVj-qbmgh1KIAe0E-NEw)
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
Inserted: {'id': 'sJruZHeF8YWLEFdoBrhkmBu5d2K_sq2yARMn9gUmNlrMdNmpEARqLceWDQUOvdLW5-E55iY9kGXC3V6-A-9zZQ'}

=== Find by ID ===
Found: {'tags': ['tag1', 'tag2', 'tag3'], 'value': 42, 'categories': ['electronics', 'computers'], 'name': 'Test Record', 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5], 'user_id': '550e8400-e29b-41d4-a716-446655440000', 'metadata': {'nested': {'deep': True}, 'key': 'value'}, 'active': True, 'data': 'aGVsbG8gd29ybGQ=', 'id': 'sJruZHeF8YWLEFdoBrhkmBu5d2K_sq2yARMn9gUmNlrMdNmpEARqLceWDQUOvdLW5-E55iY9kGXC3V6-A-9zZQ', 'created_at': '2026-01-26T00:57:21.704239', 'price': 99.99}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): True
  price (Decimal): 99.99
  created_at (DateTime): 2026-01-26 00:57:21.704239
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): ['tag1', 'tag2', 'tag3']
  metadata (Object): {'nested': {'deep': True}, 'key': 'value'}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): ['electronics', 'computers']
  data (Bytes): 11 bytes
Plain record: {'tags': ['tag1', 'tag2', 'tag3'], 'value': 42, 'categories': ['electronics', 'computers'], 'name': 'Test Record', 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5], 'user_id': '550e8400-e29b-41d4-a716-446655440000', 'metadata': {'nested': {'deep': True}, 'key': 'value'}, 'active': True, 'data': 'aGVsbG8gd29ybGQ=', 'id': 'sJruZHeF8YWLEFdoBrhkmBu5d2K_sq2yARMn9gUmNlrMdNmpEARqLceWDQUOvdLW5-E55iY9kGXC3V6-A-9zZQ', 'created_at': '2026-01-26T00:57:21.704239', 'price': 99.99}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'price': {'value': 99.99, 'type': 'Float'}, 'metadata': {'type': 'Object', 'value': {'nested': {'deep': True}, 'key': 'value'}}, 'tags': {'type': 'Array', 'value': ['tag1', 'tag2', 'tag3']}, 'embedding': {'value': [0.1, 0.2, 0.3, 0.4, 0.5], 'type': 'Array'}, 'categories': {'type': 'Array', 'value': ['electronics', 'computers']}, 'created_at': {'value': '2026-01-26T00:57:21.704239', 'type': 'String'}, 'data': {'value': 'aGVsbG8gd29ybGQ=', 'type': 'String'}, 'active': {'type': 'Boolean', 'value': True}, 'value': {'type': 'Integer', 'value': 100}, 'id': 'sJruZHeF8YWLEFdoBrhkmBu5d2K_sq2yARMn9gUmNlrMdNmpEARqLceWDQUOvdLW5-E55iY9kGXC3V6-A-9zZQ', 'name': {'type': 'String', 'value': 'Updated Record'}, 'user_id': {'type': 'String', 'value': '550e8400-e29b-41d4-a716-446655440000'}}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: VSesnppyjS1mppFXklqJixW2CBJK24668cbRFg_X_RYLPy-ENZo92lUTNV5ZVGxP6AIlPICxwYUJHlBGIbayrw

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
Collection created with first record: "QpoeNHFr7ad0pnPoWE8mB4U4MWmAbUiptmWPyvd6SXL2KvuWz8w-BuhrYxQeSVFbqwxev5fSwBpfxn2KV4qSxQ"

=== List Collections ===
Total collections: 12
Sample collections: ['client_collection_management_python', 'chat_configurations__ek0_testing', 'chat_messages__ek0_testing', 'scripts__ek0_testing', 'batch_users']

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
  cache:product:2: {'name': 'Product 2', 'price': 39.99}
  cache:product:3: {'name': 'Product 3', 'price': 49.99}

=== KV Exists ===
Key exists: True

=== KV Find (Pattern Query) ===
Found 3 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 8

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
Created Alice: $1000 - ID: 05Rk6Pa-o3wTA4BufUa7KBgCp0aKVB4xU74x52KAmVTixyc-umlkYEQh_J9HKHmCKmJPKKsy-iYyHP3sty8OKw
Created Bob: $500 - ID: N8MeMLVWS65SisZTmXsOjQtuPW8pNEz_3KJjy0YJGsQAr7GBvdcVZl12-pz9jQmvM_zOmyF8BgWYrFB2xZ1Clg

=== Example 1: Begin Transaction ===
Transaction ID: 929c208e-35e6-4b42-aff4-b6306dc0189b

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: e3651b1f-3029-44bf-96ea-0b20a61db80e
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
  1. Score: 25.740, Matched: email.value, email, name, name.value
  2. Score: 12.540, Matched: name.value, name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title.value, bio.value, title, bio
  2. Score: 26.400, Matched: title, bio, title.value, bio.value
  3. Score: 26.400, Matched: title, bio.value, title.value, bio
  4. Score: 26.400, Matched: bio, bio.value, title, title.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio, title.value, title, bio.value
  2. Score: 39.600, Matched: title, title.value, bio, bio.value
  3. Score: 39.600, Matched: bio.value, title.value, title, bio
  4. Score: 39.600, Matched: title.value, title, bio, bio.value

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
  1. Score: 0.773
  2. Score: 0.749
  3. Score: 0.735

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.374, Matched: title, content.value, title.value, content
  2. Score: 1.368, Matched: content.value, title.value, content, title
  3. Score: 0.387, Matched: 

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
✓ Inserted document: M3QeiogMzpAHl-Z1Sfu8WsN6QUdrJZjdWmjCza9j8mndHU-rj8JK_LJRfrDF2Ryvtzv39DFWCIxMnXYBgliGbg

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: NpHiFixvaOJK7wyM4Suh3t6ymTUoCS7Yp1ex18nBqyHZkjC6SwYijEtVVND6bTM9d06pXA0HOqy73eGi83o7cg

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
✓ Inserted document with TTL: Jwcms98lUTnzZPMChqcSzNWLXP-XnsWsePNg6nf2v4h7KBHgXGnCp2ePTuss_5HFyBJqzHyCK_BvhCfHF07T8A

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
✓ Edge cache script created: t02gQvZ02RxairmreyJ92cwdY5ArJjBSCqM2EM9f5N5rEjUe-VQM0GzlFxu-waOpt-006P03AUXMHUMgnUykYw

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

✅ Script saved: ge2AzVH6tdNpOXMuiMb_GEf8ax7m08-xA7zXkPy5p3VInrryVHv_1Zo98GWaxeuFh1K8TgekaOoxRomrHXRPGw
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved: qjq9-yE8hHfegCyejlJ10uZNi5MnV-99aIOu62cuEAPzXDmFRllUT4qbB4mpgPr1yqXf1O0hucbj2vY5gaX7vw
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: JaO0Bqk9Xho_waAsCucNMlVFn9cBtsfbkPzN-R_YWEMu8IKI3fWetKRrExKyg5yD-IBPqPdK6uZn-Sea-_0bog
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

✅ Saved reusable function: fetch_and_store_user
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 67.6ms
   📊 Records: 0

Second call (cache hit - from cache):
   ⏱️  Duration: 62.2ms
   📊 Records: 0
   🚀 Cache speedup: 1.1x faster!

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

✅ Script saved: O2TPSTA5NskUp_0tgoUkjZraM5TQfU4DlXj40HR_JV3RqyVP_FLc499Ioprl9WiaXKABL69QrX5u6Vh1Z_3eVw
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

✅ Inserted order: LUMGr9PTXV3y621Navbvqjk14n6AIG2G343KeYKTmlgbA73KIa-tiTk2_GYUejkTcFOMWOzwS33ZsXNKwClNFg
✅ Inserted 2 products with wrapped types

📝 Example 2: Querying and Extracting Wrapped Types

📊 Found 2 products
   • Laptop Pro
   • Wireless Mouse

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

✅ Inserted order: wKNnS6A9pRc6TaQ6ImyOhlryMk3fXSDaTlyQDA05a772Pl9Ot6ukBWDrX16lxk2gPiLP68nVG6t-Eq0efEfDdA
✅ Cached order status
📊 Quick status lookup: {'value': '{"status":"processing","updated_at":"2026-01-26T05:57:23.675841+00:00"}'}

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
✓ Created SWR script: fetch_api_user_py (-bCxmxQaOB6n9sBtG9sEczBUQWoANrKfHu6GbE2UIFrh0NUXcOSfbjNSeKlFjoBNd8EvwaImyhG7u9B1GIeJdw)

Step 2: First call - Cache miss, fetches from API
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
   Vector databases offer several benefits such as high precision, scalability, and flexibility. They allow complex geometric operations, support multi-user editing, and enable topological analysis. Furthermore, vector databases are good for representing discrete features and maintaining spatial details.
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
   {'avg_score': 20.0, 'count': 3, 'role': 'admin'}
   {'avg_score': 70.0, 'count': 7, 'role': 'user'}
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
   1. Vector Databases Explained (Database)
   2. Natural Language Processing (AI)
   3. Introduction to Machine Learning (AI)
   4. Getting Started with ekoDB (Database)
   5. Database Design Principles (Database)
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
✓ Created session: qO-Z-fYTrhxTnTF_RGGmi6IGq6Cw3n5Wtt4IzSVHy-EGRp8ZpGeXPf6yW9lO2L5vAbnhl3E9xRUrLPDM6uHP9g

=== Sending Chat Message ===
Message ID: E4s8ActqRx0yvNKTC7QacIu8XLyk7RgmHeKXb_OIv56nht4lb_2AeN-pDxl-E3HxzcFBkvwjcSC_WGQ6QmF3Cw

=== AI Response ===
We have three products available:

1. ekoDB Pro: This is an enterprise edition product with advanced features. The price for ekoDB Pro is $299.

2. ekoDB Cloud: This is a fully managed cloud database service product. The price for ekoDB Cloud is $499.

3. ekoDB: This is a high-performance database product with AI capabilities. The price for ekoDB is $99.

Execution Time: 2583ms

=== Token Usage ===
Prompt tokens: 449
Completion tokens: 89
Total tokens: 538

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: C72dvLKIhLq4pet_FQ_mAPmEt9wwoTG9o6XU2p-DavmqZB_W40LqP9t2-Ec0DdyL7yPYYMKw29gd1ik1r918Bw

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information provided, the available product is "ekoDB". It is a high-performance database product priced at $99.

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['chat_id', 'content', 'context_snippets', 'role', 'token_usage', 'id', 'created_at', 'updated_at'])
Debug: First message role: {'type': 'String', 'value': 'assistant'}
=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of the ekoDB is $99.

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: hfFbxSDUNbfSKyxC79AJeZd7GCrQRPZm1gkmWSev-oxoy7vEStTatUY4EFqw212ot2-7w95yye9aE4-_D1Ioaw
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
✓ Created session: x0OceMRUKlVJqgmvrBOeSwKgLI_pZrbtCn1_hHQAaS0T-T4dt2vZEFP0c6XtqVpd5F4QprgHp2CCrwyIkLI1Dg

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information provided, there is a high-performance database product available named ekoDB. It costs $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: Et5UKK7wuG-JvSWMb2ZfwSt2YUMfwNTA4xZ2uOBY1Wmm6DSfzZy3eSpZ8eb4KM9kIStgpH9JKq0CmIuo5phhVQ
  Parent: x0OceMRUKlVJqgmvrBOeSwKgLI_pZrbtCn1_hHQAaS0T-T4dt2vZEFP0c6XtqVpd5F4QprgHp2CCrwyIkLI1Dg

=== Listing Sessions ===
✓ Found 7 sessions
  Session 1: Et5UKK7wuG-JvSWMb2ZfwSt2YUMfwNTA4xZ2uOBY1Wmm6DSfzZy3eSpZ8eb4KM9kIStgpH9JKq0CmIuo5phhVQ (Untitled)
  Session 2: x0OceMRUKlVJqgmvrBOeSwKgLI_pZrbtCn1_hHQAaS0T-T4dt2vZEFP0c6XtqVpd5F4QprgHp2CCrwyIkLI1Dg (Untitled)
  Session 3: hfFbxSDUNbfSKyxC79AJeZd7GCrQRPZm1gkmWSev-oxoy7vEStTatUY4EFqw212ot2-7w95yye9aE4-_D1Ioaw (Untitled)
  Session 4: qO-Z-fYTrhxTnTF_RGGmi6IGq6Cw3n5Wtt4IzSVHy-EGRp8ZpGeXPf6yW9lO2L5vAbnhl3E9xRUrLPDM6uHP9g (Untitled)
  Session 5: wNWC_Xk3eqKojZcNLJI7ju-2qUVlYBXfFDTXA8MF4cqBWoe0Vh8TkGCxiYsb0gwbZeQUPFasl6EsXuhSfp3kzQ (Untitled)
  Session 6: gnRXJNHkYunqfd9H8WAnDUhdM_bv_6oBpprg16bG5k95zOLdYvzkww8npnALyOvXgr2HoKc7v1m9f-T9raeLGQ (Untitled)
  Session 7: twO1BB1nRCiwMXP9x5q1aLkLr3v9Hx_eQLeKCnTTvf2LQClZQ3jyYvmcuFPlzPfVdBYPZw-czz__aLo_WgAbBg (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: Et5UKK7wuG-JvSWMb2ZfwSt2YUMfwNTA4xZ2uOBY1Wmm6DSfzZy3eSpZ8eb4KM9kIStgpH9JKq0CmIuo5phhVQ

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Dict Creation ===
✓ Created record with plain dict: {'id': 'xsFxoJzBLluuOz-1ONWAXA3_xurXViXyNedk79W4Ya-JG9y_-PngEA4nwExgnoCdsB1RDmqLyBKsvO5pIIg0Mw'}

=== Upsert Operation ===
✓ Upsert (update existing record): xsFxoJzBLluuOz-1ONWAXA3_xurXViXyNedk79W4Ya-JG9y_-PngEA4nwExgnoCdsB1RDmqLyBKsvO5pIIg0Mw
✓ Inserted second record: 44u5e1bin8VTGeyuWQBrNjDWKl86jQVBriM9UTshGqxlrvl-8RSj41ySykeTPOLFCXddl64jvK5UrElW2wvPNQ
✓ Upsert (update second record): 44u5e1bin8VTGeyuWQBrNjDWKl86jQVBriM9UTshGqxlrvl-8RSj41ySykeTPOLFCXddl64jvK5UrElW2wvPNQ

=== Find One Operation ===
✓ Found user by email: {'active': {'type': 'Boolean', 'value': True}, 'id': 'xsFxoJzBLluuOz-1ONWAXA3_xurXViXyNedk79W4Ya-JG9y_-PngEA4nwExgnoCdsB1RDmqLyBKsvO5pIIg0Mw', 'name': {'type': 'String', 'value': 'Alice Johnson'}, 'email': {'value': 'alice.j@newdomain.com', 'type': 'String'}, 'age': {'value': 29, 'type': 'Integer'}}
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
   Inserted with ripple: {'id': 'CdezZUXFEb_6PkuHpvWvsIx2REAYJnyeC1h6ptOVdUzJCGKuKMIzDbD-b7jH7MgtlhsxoSEAEO9dZAnsjCcZvA'}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {'id': 'iQOMjLx74K_g15LzDwj4pbkxDXfNMr44-4eHahx8gqbGenl1iJauKw45th30Ax-7LUvYGLsAFfsM5xPndErp0A'}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {'price': {'value': 150, 'type': 'Integer'}, 'id': 'CdezZUXFEb_6PkuHpvWvsIx2REAYJnyeC1h6ptOVdUzJCGKuKMIzDbD-b7jH7MgtlhsxoSEAEO9dZAnsjCcZvA', 'name': {'type': 'String', 'value': 'Product 1'}}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {'price': {'type': 'Integer', 'value': 500}, 'id': 'CdezZUXFEb_6PkuHpvWvsIx2REAYJnyeC1h6ptOVdUzJCGKuKMIzDbD-b7jH7MgtlhsxoSEAEO9dZAnsjCcZvA', 'name': {'value': 'Upsert Product', 'type': 'String'}}

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
Fetched user profile: {'type': 'String', 'value': 'Alice Johnson'}

Example 5: Compare full vs projected data
Full query returned 12 fields per user
Projected query returned 12 fields per user

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
✅ [32mPython client examples complete![0m
