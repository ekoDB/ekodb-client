make test-examples-python
🧪 [36mRunning Python examples (direct HTTP/WebSocket)...[0m
=== Simple CRUD Operations (Direct HTTP) ===

✓ Authentication successful

=== Insert Document ===
Inserted: {'id': 'twoxWMb2mwijONFhaYC8hfML5_pOKwq-HB5KKfmv6WZEMh8s3IrsVA44mzLu05_EVF7bbAGHwebjhXaJLP8Eyg'}

=== Find by ID ===
Found: {'value': 42, 'active': True, 'id': 'twoxWMb2mwijONFhaYC8hfML5_pOKwq-HB5KKfmv6WZEMh8s3IrsVA44mzLu05_EVF7bbAGHwebjhXaJLP8Eyg', 'name': 'Test Record'}

=== Find with Query ===
Found documents: [{'value': {'type': 'Integer', 'value': 42}, 'id': 'twoxWMb2mwijONFhaYC8hfML5_pOKwq-HB5KKfmv6WZEMh8s3IrsVA44mzLu05_EVF7bbAGHwebjhXaJLP8Eyg', 'active': {'type': 'Boolean', 'value': True}, 'name': {'type': 'String', 'value': 'Test Record'}}]

=== Update Document ===
Updated: {'name': {'type': 'String', 'value': 'Updated Record'}, 'id': 'twoxWMb2mwijONFhaYC8hfML5_pOKwq-HB5KKfmv6WZEMh8s3IrsVA44mzLu05_EVF7bbAGHwebjhXaJLP8Eyg', 'value': {'type': 'Integer', 'value': 100}, 'active': {'value': True, 'type': 'Boolean'}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: GVRDwJ1ctChqDW-nHlur23wm5ZxClykHrnNvvUfdGMGCmcMMLu6HmAOOGOX7dcCRGFO11nhRc7UzzQMSIvLOHw

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
        "id": "GVRDwJ1ctChqDW-nHlur23wm5ZxClykHrnNvvUfdGMGCmcMMLu6HmAOOGOX7dcCRGFO11nhRc7UzzQMSIvLOHw",
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
        "id": "ajOM9grEuxTgPap_lS26dHJIjqMNVtCi0kY26-XoI-_V6fIVLgzgWGF1AbVnsyVjJ7f6teDdL69dUR6se7jbXA",
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

✅ Script saved: kEWSqn-dbSmjmrZ77xxu9MR_lb7H8H_r6jFV1jim64jR4XwitOsM2T0bTFxttnbg_J8icj6FfJudiyv8nNXczg
📊 Found 15 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: AkYBmBoprcsmrzi-e34ToNIGvc0NOiky_WaXcjvtl2DnMhLIaFzDBpkulih9U4c6AMC9jj0lZ6rFL-lvTmQ3yA
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: 36lqyvsQSlgzEOD2C1dyp-uBYZw29_g-9YlHpbYvH5TXpEwHwlSgoiOdH5HD4k2VqCKNtQWEeXx37m4RblbRSA
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {'count': 15, 'status': 'inactive', 'max_score': 90, 'avg_score': 50.0}
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
Collection created with first record: Ele5YcDMXK6UicQadaaVJKMd-2evpfelNJw7d_vQdlTXykxKlen9ji30DlRoFN3IlGGHHmQDUv7ii523v-BMAA

=== List Collections ===
Total collections: 13
Sample collections: ['chat_messages__ek0_testing', 'github_cache', 'ws_ttl_test', 'test_collection', 'products']

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
Created Alice: $1000 - ID: DCHCq-BrpDqSnxYiJ2GWaqG6b6xMvOqPmgxsU4OY0K2riR6_jjKrX5dra7NL0tgHGLO9_-D1tur04-OGAJTtIw
Created Bob: $500 - ID: bPdQIcoRYKQUIdKhyqA9qPosWlNNyycS8ESpy-ASlJbHpRo3HsT44k62lrIaOXMXu5ZuG9gGcMafoF1UZ2fGbg

=== Example 1: Begin Transaction ===
Transaction ID: 153aa5b8-8035-45bd-93de-449fa6751d61

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
New transaction: 7f8418e7-c2a7-4e52-9e34-78b3f0964571
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: tA-_YZLK0WRvaEJ7V9HzMNWhZa0gI7iy4KV122amfNeZMjbVGmObJc_a0EDfT8siAojswgDOPvH85l3fF2BIbw
Created Bob: $500 - ID: uZHNJDL5Ulqs4sbI57DlVp0CPxGwKATcnpzcds403Twm9ZnNnVqXKLOY-ta26acUztu5Lp5OOVd4ro94Uj7HBg

=== Example 1: Begin Transaction ===
Transaction ID: 39044c63-4817-4b8c-9028-832ac7172c64

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
Created Alice: $1000 - ID: -WhlIQs8rYl9JVlDqP7skCxR0esvEqcd2yI9ua-NFQgtt52lnH2-aVPFRbOYSYUcwkjo-91vQCYgrpVQUIdgyQ
Created Bob: $500 - ID: a2VSJEBLMNHc2EbjVX5f6XK2CVl9bCCu6cqvv-OoqgaGq1rYtmaMZEghkSAm3Q2DY8Rd8uPSN5ir2rAbMcuCrA

=== Example 1: Begin Transaction ===
Transaction ID: 2b04206f-3839-4787-aeb3-a7b43297a638

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
New transaction: 7ffca086-2727-499f-b968-0e4819022588
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: WTqmkypvFBTKymW7bdWtASgXkFYgh7PzDcqgGFnVO1LzkL3yNhIfqQzJtdZpo917U8sPc85uIPik0FM1Cw4LVA
Created Bob: $500 - ID: d8G0bINpGE-EXdlK3t-htxr2sue6tlR098j2GDjp_dlEQ9OhJzC1SReXcckcWM35mpxj1vnE7MPqYoZTd2W5Rg

=== Example 1: Begin Transaction ===
Transaction ID: 03be6cfc-6b8b-4b6c-aac5-7c369a17d542

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
Created Alice: $1000 - ID: 6a8HzzuI9crTxjD4XjnY7VOmYwDQuhhV8jCp8N7TSnxp6kHh0lIkA5k2jqAYu_BltWIr8GwKK3_H8L3ZwqR9AQ
Created Bob: $500 - ID: 9H4YaSih8zjEuasrWV56wGvs3JPZkG-mN2YpVxp6-tVPf3SCjCCodlB4SOBqb-TzfUm2HiBXJaDTwfdL9QeG8w

=== Example 1: Begin Transaction ===
Transaction ID: a95d6223-ed51-4128-9cff-f5f447aa01d6

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
New transaction: 10bda475-29c1-4f1b-ada6-448f1ba941b0
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: a5TMmdDa7301t4lUN3FYpDVFiDUgurHFHP3bryisPE2pGzP92Fu63VUf2ykEwlBxKDOHIvUokpNSCIOmw_dEbw
Created Bob: $500 - ID: Sf5V9VvHGwLxUxaqNBmE8h3R8vvYvQox97qXC-hxczjO_JzhEY4WvrMbegDpCdax_g_HiL978BuAxx9C9_7lbA

=== Example 1: Begin Transaction ===
Transaction ID: b3b06dda-c697-4e4d-8b0a-627588b18ad1

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
   ✅ Script saved: PpkyRCS_ZU96Wgv67lWP1f1ZfFXWJMlLf1knmpfYQhKC_E_O6pDchqV67dZkE2Vmc-O_QM0hfbezKLyxcZvOJQ

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: ekK2AGha6Da5znGukJccmy1HytBM2oXk_BR1GKEofrJiH1NYdo4qW_-1dIj2jM6ODOsaV5TmcD3Aw8U8z1xFHA
   📋 Name: {'value': 'Alice Smith', 'type': 'String'}
   📋 Email: {'type': 'String', 'value': 'alice@example.com'}
   📋 Status: {'value': 'pending', 'type': 'String'}
   📋 Credits: {'value': 0, 'type': 'Integer'}

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: k-DWLI6nINYvDxRx0E5QfAziRosEy-emXyeLZsg7t-RGM_xbHdsHm--AVsbZ3orzeOhoR1rKGcfKFkxqF4_B9A

2️⃣ Calling Script (Query + Update + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Status updated to: {'value': 'active', 'type': 'String'}
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}

============================================================
📝 Script 3: Query + Update Credits + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: vymFuSaUnShVR59kiGXeZRC8lTP3UuHynKIWm3OI7sWa3gifiNDsNqmen-2ye5Xq8LTbaNcMF8VOR6K1FKjyEA

2️⃣ Calling Script (Query + Update Credits + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {'type': 'Integer', 'value': 0}
   📋 Status: {'type': 'String', 'value': 'active'}
   📋 Name: {'value': 'Alice Smith', 'type': 'String'}

============================================================
📝 Script 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: s9Re-jxm5cfAmrunTGDMTGz6-dBWAWwiUC7ckadwf5reyOrI3Rwjkns1YAGD5Q36XcWh4uxV0tL5DqxKBihXpw

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: PpkyRCS_ZU96Wgv67lWP...
   ✅ Deleted script: k-DWLI6nINYvDxRx0E5Q...
   ✅ Deleted script: vymFuSaUnShVR59kiGXe...
   ✅ Deleted script: s9Re-jxm5cfAmrunTGDM...
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
  Output: Document ID = yEARhX_Z2hCRsrOit6NtEVGSwUCqPa6Ph5DjLOeuhsyJ2CaVzupXvKdF2kOSIaRIViq3XLRtoae8rr4nT4PyBQ
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: find_by_id(yEARhX_Z2hCRsrOit6NtEVGSwUCqPa6Ph5DjLOeuhsyJ2CaVzupXvKdF2kOSIaRIViq3XLRtoae8rr4nT4PyBQ)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: find_by_id(yEARhX_Z2hCRsrOit6NtEVGSwUCqPa6Ph5DjLOeuhsyJ2CaVzupXvKdF2kOSIaRIViq3XLRtoae8rr4nT4PyBQ)
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
  Output: Document ID = 1MZVRDELHveTjb3GkhoZ_s71ct0YBAW1Dg4MSgPGjAv9OQ6hRe1liR5KMFTDOuRBgaCxqYRIDdi9G7_FRGcwXA
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: find_by_id(1MZVRDELHveTjb3GkhoZ_s71ct0YBAW1Dg4MSgPGjAv9OQ6hRe1liR5KMFTDOuRBgaCxqYRIDdi9G7_FRGcwXA)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: find_by_id(1MZVRDELHveTjb3GkhoZ_s71ct0YBAW1Dg4MSgPGjAv9OQ6hRe1liR5KMFTDOuRBgaCxqYRIDdi9G7_FRGcwXA)
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
Inserted: {'id': 'a38iXBgDCa7pOIl7rQyPVYsgazGCUrYDxiQS-IYkY34aC1ITFdMHa0vPMg-UGkfRJFDD_-GnrJR-XQv9_0uyKQ'}

=== Find by ID ===
Found: {'categories': ['electronics', 'computers'], 'metadata': {'key': 'value', 'nested': {'deep': True}}, 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5], 'created_at': '2026-01-23T12:27:02.630193', 'data': 'aGVsbG8gd29ybGQ=', 'id': 'a38iXBgDCa7pOIl7rQyPVYsgazGCUrYDxiQS-IYkY34aC1ITFdMHa0vPMg-UGkfRJFDD_-GnrJR-XQv9_0uyKQ', 'user_id': '550e8400-e29b-41d4-a716-446655440000', 'name': 'Test Record', 'value': 42, 'tags': ['tag1', 'tag2', 'tag3'], 'active': True, 'price': 99.99}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): True
  price (Decimal): 99.99
  created_at (DateTime): 2026-01-23 12:27:02.630193
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): ['tag1', 'tag2', 'tag3']
  metadata (Object): {'key': 'value', 'nested': {'deep': True}}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): ['electronics', 'computers']
  data (Bytes): 11 bytes
Plain record: {'categories': ['electronics', 'computers'], 'metadata': {'key': 'value', 'nested': {'deep': True}}, 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5], 'created_at': '2026-01-23T12:27:02.630193', 'data': 'aGVsbG8gd29ybGQ=', 'id': 'a38iXBgDCa7pOIl7rQyPVYsgazGCUrYDxiQS-IYkY34aC1ITFdMHa0vPMg-UGkfRJFDD_-GnrJR-XQv9_0uyKQ', 'user_id': '550e8400-e29b-41d4-a716-446655440000', 'name': 'Test Record', 'value': 42, 'tags': ['tag1', 'tag2', 'tag3'], 'active': True, 'price': 99.99}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'name': {'type': 'String', 'value': 'Updated Record'}, 'price': {'type': 'Float', 'value': 99.99}, 'id': 'a38iXBgDCa7pOIl7rQyPVYsgazGCUrYDxiQS-IYkY34aC1ITFdMHa0vPMg-UGkfRJFDD_-GnrJR-XQv9_0uyKQ', 'value': {'type': 'Integer', 'value': 100}, 'tags': {'type': 'Array', 'value': ['tag1', 'tag2', 'tag3']}, 'metadata': {'value': {'key': 'value', 'nested': {'deep': True}}, 'type': 'Object'}, 'data': {'value': 'aGVsbG8gd29ybGQ=', 'type': 'String'}, 'embedding': {'type': 'Array', 'value': [0.1, 0.2, 0.3, 0.4, 0.5]}, 'categories': {'value': ['electronics', 'computers'], 'type': 'Array'}, 'active': {'value': True, 'type': 'Boolean'}, 'user_id': {'value': '550e8400-e29b-41d4-a716-446655440000', 'type': 'String'}, 'created_at': {'value': '2026-01-23T12:27:02.630193', 'type': 'String'}}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: NEaehovgojfrlz7In_B2jUYLJJPoJ6D-_QsbqwyZKll7rn833nKWBwYaQR2ztTWD3sWQjA-A2lZ9jKow4TzTvQ

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
Collection created with first record: "Lf2_MMhN9FmWqTrKRlRkXxRQvnctUm9smpi3UlPdDIyY4C5D42scQNHPujfM0KyiPVCeSWcRCianEzKg4hF5-Q"

=== List Collections ===
Total collections: 12
Sample collections: ['chat_messages__ek0_testing', 'github_cache', 'ws_ttl_test', 'test_collection', 'products']

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
  cache:product:3: {'price': 49.99, 'name': 'Product 3'}

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
Created Alice: $1000 - ID: dBbHXAndgzlgIrO7edWNQIjg8hV-Q9AE9QH0K3EtCihKeOpzGMDMwaRT76NmQUdIdRTIjy6a6nVOkDUyAidLZA
Created Bob: $500 - ID: 1fhz3MZxkzj6RU98jQ-qx56IRi0I0-44K4kCLjTn_mEe2nuUxzoaMAcjRF1UdbPc3VyaGLD7QwhRA95haYRjHg

=== Example 1: Begin Transaction ===
Transaction ID: b3ad17bb-1915-44e8-941a-61c4555a4a29

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 21b04eba-5562-44f9-bbf5-691c6b69ef3a
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
  1. Score: 25.740, Matched: name, email, name.value, email.value
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: bio, title, bio.value, title.value
  2. Score: 26.400, Matched: title, title.value, bio.value, bio
  3. Score: 26.400, Matched: title, bio, bio.value, title.value
  4. Score: 26.400, Matched: title.value, bio.value, title, bio

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title.value, title, bio, bio.value
  2. Score: 39.600, Matched: bio.value, title, title.value, bio
  3. Score: 39.600, Matched: title, title.value, bio.value, bio
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
  1. Score: 0.765
  2. Score: 0.763
  3. Score: 0.757

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.382, Matched: content, title, title.value, content.value
  2. Score: 1.379, Matched: content, title.value, title, content.value
  3. Score: 0.382, Matched: 

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
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: 2EAXOumjbXqi645KAYRfNtMoPmOzbE9cxP-4RC2W8uob_Hr_IL1WWhIwt3jDIxHSFLitZreRCYB_yJEZoZcuRw

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: eJ39od71GyTldxsHQ3j6PZ_Qqmv2gv-P2aPDhchFtNsMJVJ9nBk8VJCmV8It59N00m_W1uLnPQzvKVuR1tlMoQ

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
✓ Inserted document with TTL: 87ES_SeNEwZcKWoMlAqSBaRZaxjcjy5RlyVfIUaqKok9sYrBwHEqK5JERFk1waTgK8dIYo0cZ9sd7e90dGURmg

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
✓ Edge cache script created: 2uawugjJDGm6J-2lGGKvubnCB2xKYOMDBGCesqRu8-XpOWQGe49VC7yW4A0qgEb7rWuuLPTAmaQalLdiCE17nA

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
Response time: 2ms (1.1x faster!)
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

✅ Script saved: ymSqbX4jFUO2TpCuPldFACl5hwuIZ9Cq-Ru7eKh_LgWZWzK43ss9tw_o6Nh7MTsZpYMr0qTbbbe9Lpx75UCCmg
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved: e3y9LhfQLc1dw4NzuyaaIPFE55I4UGGk97A_tJNPRr7qSGl2J6WqP-kenvBL03ikUUS9pkGGYDwkUcTASTIVZA
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: LgmStIBxqM_W9J4z5a1EFY4sIeQFVomWcVY9BAkuj0EflXgKv7aqJmrIqx_lSyj9O3RAGEQjdpJiURuQHpj3zQ
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
   ⏱️  Duration: 127.2ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 78.9ms
   📊 Records: 1
   🚀 Cache speedup: 1.6x faster!

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

✅ Script saved: xja5OEnP-hhxH6355k7vBJu-FgwT57xJLmuVSUDFRsG4kdQBld8Xcw1HbkYecFd_dHQwJONCwYsKacnqaqiKbA
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

✅ Inserted order: W3l6PGA8gje4C-R6qpmKoCXFArD4gUalH8PeSNkDPwjkH_Khxud9EbknS19u3qXHjNVIs5pYelyPz0IJhRlDfg
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

✅ Inserted order: nxxZY15qP_MLaqGgDMybZIGCljFyP0P70zBlajO1KUB-ujJ17NHQSltp6dUIrT3Z9Wa5v2rd7iojykGBrSuffg
✅ Cached order status
📊 Quick status lookup: {'value': '{"status":"processing","updated_at":"2026-01-23T17:27:04.374352+00:00"}'}

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
✓ Created SWR script: fetch_github_user_py (8vCY1cigdt97nr3AgHEA1ZFHJMjVZOpfo_lBOTQyp1xO1YeXsJkSUh1SIalcRXKNW1jayQr0EBTo39_diQSt0Q)

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
   Vector databases offer several benefits including high precision, support for complex structures like points, lines, polygons, and scalability for large datasets. They also facilitate accurate geographic analysis and spatial querying.
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
✓ Created session: deBMeaW6ZyOXZgSVS4I7CjcdHEUe5mNs3ujERxw6PbgqG52yubBVJplrqnvQgRwQHBfnGxO5nWYgAy5o7IQrMw

=== Sending Chat Message ===
Message ID: vTbbdrtwkc8mWSvmbHhowTXOFa0K1dHQs2wiBNWTDBV8D7vKVdEhwqUt9kuIMXiwgWhfppuICPVpbAJngolN6Q

=== AI Response ===
We have three products available:

1. ekoDB Cloud: This is a fully managed cloud database service product. It is priced at $499.

2. ekoDB: This is a high-performance database product with AI capabilities. It is priced at $99.

3. ekoDB Pro: This is our enterprise edition product with advanced features. It is priced at $299.

Execution Time: 2579ms

=== Token Usage ===
Prompt tokens: 454
Completion tokens: 78
Total tokens: 532

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: olRDG0h-RBsVX_CRaDRRgLSQON-Btl8bZ_b-YyDDUVjxDBSHt_-tUEbk14GDCutEM9n8QJRpGvWmTRp6vq3rHw

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information provided, the available product is ekoDB. It's a high-performance database product and it's priced at $99.

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['id', 'context_snippets', 'chat_id', 'token_usage', 'updated_at', 'created_at', 'role', 'content'])
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
✓ Created second session: zOR29wOGIutIB8YS2Abnb1ahQ69bxObd1qN0vAehTVcUeLY3FKlJVFqey_muiTzCTeh6hkk8hdQVGYFPMuvPoA
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
✓ Created session: ebQGyGWgh0zo6NC6G7leQ3mI6uFt96rW_Ik654cLY62ub1HGUMF-izl-bt8R8MtfugVaDXf6ZJfAuqsIn92MAg

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the context provided, the available product is "ekoDB". It is a high-performance database product. The price for this product is $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: Bywkt8qlKN7sL69jUopYLVOJg9z_wfiWrl-TOdRF_q2o-7bXti07QS3ic8VuA_fqtQP5xZ5d9NSU4MfxVGG7Fw
  Parent: ebQGyGWgh0zo6NC6G7leQ3mI6uFt96rW_Ik654cLY62ub1HGUMF-izl-bt8R8MtfugVaDXf6ZJfAuqsIn92MAg

=== Listing Sessions ===
✓ Found 7 sessions
  Session 1: Bywkt8qlKN7sL69jUopYLVOJg9z_wfiWrl-TOdRF_q2o-7bXti07QS3ic8VuA_fqtQP5xZ5d9NSU4MfxVGG7Fw (Untitled)
  Session 2: ebQGyGWgh0zo6NC6G7leQ3mI6uFt96rW_Ik654cLY62ub1HGUMF-izl-bt8R8MtfugVaDXf6ZJfAuqsIn92MAg (Untitled)
  Session 3: zOR29wOGIutIB8YS2Abnb1ahQ69bxObd1qN0vAehTVcUeLY3FKlJVFqey_muiTzCTeh6hkk8hdQVGYFPMuvPoA (Untitled)
  Session 4: deBMeaW6ZyOXZgSVS4I7CjcdHEUe5mNs3ujERxw6PbgqG52yubBVJplrqnvQgRwQHBfnGxO5nWYgAy5o7IQrMw (Untitled)
  Session 5: DC2g7WWPHsV5bOOghEpM4TZLVy4ViAFTeIHUeBiROZLxtVeH0tzTN51YS3Q-ke8HI91cEfTmVyI_oXSOkPD65A (Untitled)
  Session 6: -4ud5ChnACwEjh1N7E3gYEQ33gMpz4YAMtz_zYGQwtZblZRjNJc7BSsYHmrBEoM4Mx_Z58rUDmdMickMiP7dzg (Untitled)
  Session 7: crBexqJwKDpcsGuB3rJQOwiUxiEf2h11eXAFX6UNxZ6Ubw7DoeufXGKOj9ak8CZ38Y6jh3U1t1xtDGRI_nGmsw (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: Bywkt8qlKN7sL69jUopYLVOJg9z_wfiWrl-TOdRF_q2o-7bXti07QS3ic8VuA_fqtQP5xZ5d9NSU4MfxVGG7Fw

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Dict Creation ===
✓ Created record with plain dict: {'id': 'HXB1N9QYZ_zC7Bf0Bva0ScDUkKOuDgHxcyeQlVIWI6613ebcGawMNN4Amh_0QzV2hExT8F91qvnpla2Vzhp0-Q'}

=== Upsert Operation ===
✓ Upsert (update existing record): HXB1N9QYZ_zC7Bf0Bva0ScDUkKOuDgHxcyeQlVIWI6613ebcGawMNN4Amh_0QzV2hExT8F91qvnpla2Vzhp0-Q
✓ Inserted second record: OmkhFthSylaQZtKOqr4HDC9xyHNZfoGaY3tNWR72SeaKEiZGL-qooyrFYsmmgFtAPKG4PdzFYmJIbAPoBf6OZA
✓ Upsert (update second record): OmkhFthSylaQZtKOqr4HDC9xyHNZfoGaY3tNWR72SeaKEiZGL-qooyrFYsmmgFtAPKG4PdzFYmJIbAPoBf6OZA

=== Find One Operation ===
✓ Found user by email: {'name': {'value': 'Alice Johnson', 'type': 'String'}, 'age': {'value': 29, 'type': 'Integer'}, 'active': {'type': 'Boolean', 'value': True}, 'email': {'type': 'String', 'value': 'alice.j@newdomain.com'}, 'id': 'HXB1N9QYZ_zC7Bf0Bva0ScDUkKOuDgHxcyeQlVIWI6613ebcGawMNN4Amh_0QzV2hExT8F91qvnpla2Vzhp0-Q'}
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
   Inserted with ripple: {'id': 'iQLERUaAlzjuMaDGhvZSZ2Rn7JXIWD7Cwko15cdVc-1Ueb5wHUp0cn0HX6ikGe89sR1-N2CI6p2bpShnzjWANg'}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {'id': 'OfoK_Os8lozEIJT0POKCor0pp4xOGm6HreYWiHmx3ySUz2AnxpHyiJLt3JP6gcrHuP46TYx_HmoLEdAdaFqdzQ'}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {'name': {'value': 'Product 1', 'type': 'String'}, 'id': 'iQLERUaAlzjuMaDGhvZSZ2Rn7JXIWD7Cwko15cdVc-1Ueb5wHUp0cn0HX6ikGe89sR1-N2CI6p2bpShnzjWANg', 'price': {'value': 150, 'type': 'Integer'}}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {'name': {'value': 'Upsert Product', 'type': 'String'}, 'id': 'iQLERUaAlzjuMaDGhvZSZ2Rn7JXIWD7Cwko15cdVc-1Ueb5wHUp0cn0HX6ikGe89sR1-N2CI6p2bpShnzjWANg', 'price': {'type': 'Integer', 'value': 500}}

✅ All bypass_ripple operations completed successfully!
✅ [32mPython client examples complete![0m
