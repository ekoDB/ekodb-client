make test-examples-python
🧪 [36mRunning Python examples (direct HTTP/WebSocket)...[0m
✓ Authentication successful

=== Insert Document ===
Inserted: {'id': '_bDuL-hndeYv8fRUpB_c2oFYSgvRqqqEiXb-EotSTW_CWoQ1R9vHRNDd-Se-oEpORCd2fJKXwAVLPMlYcc0t1w'}

=== Find by ID ===
Found: {'id': '_bDuL-hndeYv8fRUpB_c2oFYSgvRqqqEiXb-EotSTW_CWoQ1R9vHRNDd-Se-oEpORCd2fJKXwAVLPMlYcc0t1w', 'active': {'value': True, 'type': 'Boolean'}, 'name': {'type': 'String', 'value': 'Test Record'}, 'value': {'type': 'Integer', 'value': 42}}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'name': {'value': 'Updated Record', 'type': 'String'}, 'active': {'value': True, 'type': 'Boolean'}, 'id': '_bDuL-hndeYv8fRUpB_c2oFYSgvRqqqEiXb-EotSTW_CWoQ1R9vHRNDd-Se-oEpORCd2fJKXwAVLPMlYcc0t1w', 'value': {'type': 'Integer', 'value': 100}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: EHEYTwN4dvGdPiFjeN68OBYQo7RXh3x50hrjyfNlRu14LvgPwHcWDqQ-lzEqpzxKv9OGmvlLZReOEVa3sk9Szw

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
        "id": "EHEYTwN4dvGdPiFjeN68OBYQo7RXh3x50hrjyfNlRu14LvgPwHcWDqQ-lzEqpzxKv9OGmvlLZReOEVa3sk9Szw",
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
        "id": "WaBLKHHaUr5EzU2ATdkHLvXtzFbO8-_txXCCsCgLbh3hqPL8p-jLVoUzdHAgJhCQlO1uSOqvXl_I5gzOCfGMxA",
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
cache:product:2: {'name': 'Product 2', 'price': 39.989999999999995}
cache:product:3: {'price': 49.989999999999995, 'name': 'Product 3'}

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: iJkY7hlsEDpY0r3ieE9t1vQtwT7CAglH86ERBMes1DcSXj_1lAzJqeTpiV8BMGNcL6nfpiTP8eowaWvMUd-f9w

=== List Collections ===
Total collections: 11
Sample collections: ['chat_messages__ek0_testing', 'test_accounts', 'websocket_test', 'users', 'demo_collection']

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: False

✓ All collection management operations completed successfully
✓ Authentication successful

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: vUcI3WxcKRs1AJZvYFQg23obmEMIffDi-WEZG1Xw9o9YJ1CpdqQhzfDV3Jv7ar2yzMO06fEpjET5-rJofKQ37g

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: created

=== Query Documents ===
✓ Found 2 documents with TTL

=== Update Document ===
✓ Updated document

=== Delete Document ===
✓ Skipped delete (no ID returned)

✓ All document TTL operations completed successfully
✓ Authentication successful

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: SlPs0GwVOEFPm1uHaZVLCzmCprw5SHi-tSV8iDd5Cx43dbUzvxtFl8sP5AP0pIBPU3zD0-Lx5ct73YZeqvxoqA

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 2 record(s) via WebSocket

✓ WebSocket TTL example completed successfully
🚀 ekoDB Scripts Example (Python/HTTP)

✓ Authentication successful

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: bpZb3HsuOUDk0_ay2bI2DKGz6LmoCVkqF4sU-1PER3e7b4atIBMMDFqkCFJGlinrXkkpu7p5V0TOll44Dn8LMA
📊 Found 15 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: 3IPoKS4GC5C-peIGm09hBaXjL7az0HCT_3uLgxMvP_Hd-lGauybB3GmPGdilk_nzfpVl3kPy27NNQVnaaNgP-Q
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: y3gjl959mLjhBpM92IPhlf3_KKN9jHcuzESIdwsLCwclLgFebW4fePSVN44E4ooolXaWChOaRS5v8pXRuUeLhQ
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {'status': 'inactive', 'max_score': 90, 'count': 15, 'avg_score': 50.0}
   {'max_score': 100, 'status': 'active', 'count': 15, 'avg_score': 60.0}

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
Created Alice: $1000 - ID: Xx9htuCOGqj6XY8xuCPVhLbxnVoMjEEmnNAqUcit2D3vD0nTdCA1n33itUywUSyzmmB4cwFJwAVGYm7meSs_5g
Created Bob: $500 - ID: mp5X0xb2AvVlFBrUKBImEkECBfASJTdovRuCduEsH02mWCn17tXsd9ym-zldPhyJKulfZpxgUiKAP2c0cl7dyw

=== Example 1: Begin Transaction ===
Transaction ID: 837f4b0f-b4a7-4d95-a8ee-1a156d73c5dc

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
New transaction: 0e93b5e9-88a2-453e-b093-caa9c1588ebd
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: wouASXHcRyLmvx-IDENvcWHxYsMp6H-dSJBoBcyrGTLH5ic7Y7rwfwckIxYfBHJV-_9Pt8Mg7P_YUuhTwGXXBg
Created Bob: $500 - ID: 292A7bfrG0ghVaQaLSh-BbRgvsn092Rxs2JaTS80cwYf8ERY7Mml3duTzp5QmYvWe0nQc-TJoMBsDy9cOpb_MQ

=== Example 1: Begin Transaction ===
Transaction ID: 07618cff-71b2-4065-b5c8-f826212d5d75

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
Created Alice: $1000 - ID: mTnFQdeO7NSlBSqpf2lBNoblDWhHOUee7Sg0YUJOLSUGSghfXck8SvMCTKJrwQdg72SBa9Cvvqf27qwji4DXNw
Created Bob: $500 - ID: F-JRWElmKN04sCfSuiaL_Iw2YTRmZf8N-QSZBknt__LS-jgZfxBN9xXhBe7aJbFYvR0dqWSs_vzVQPM4j-plYw

=== Example 1: Begin Transaction ===
Transaction ID: 047ed0dd-a4bd-42d2-8ec5-61c0c7b53061

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
New transaction: b777559c-f74c-43c7-bf9b-6ba83bb5f48c
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: yzEqp9egYFUpHWVQAr3MiA2yZ12-2anJYUU7GIBJuzk76resQF_itqhuaZkNCQzR7O_lJe_anf0w8NFHvjh9Kg
Created Bob: $500 - ID: qe8g6s5w0Oujfl4kDCl4kiUmQh7YnLcMg3zn3wjH0PcXCzvlVf9mduOpITS9bywO5T4lCTUR2mGLNN3JSPhOGg

=== Example 1: Begin Transaction ===
Transaction ID: 8d34b705-3745-4d6a-a5b3-807e585c0716

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
Created Alice: $1000 - ID: 2v1aCHWK0MDWVSi7AK2vBSs4PJaAKtq5Mf15xmAtKXAvZxwmxEYnT4YhT8L6wIsh8Pejkrtz4V1tdHPoel65mg
Created Bob: $500 - ID: awKX2OPgtzmq2D4YZX-rziDuJODkRaLdxPIKTd3nCev2OQxDTGOCfFlAQb0o0voFd3NxL3WAQOcjV0lTa-HqYA

=== Example 1: Begin Transaction ===
Transaction ID: 77e6dc93-03b5-4342-90ca-7ac3ff7fcf20

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
New transaction: 56795fc2-cdfc-40d4-8804-7133b1918cd9
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 6FGj1FgNa-iMqwiW9xhRw9BNi1cUhPCR7W_Q3MP77SiFNxUTqWy7T_ElcMsNw4DWugIE1hH9fmtRYfLhenDoDQ
Created Bob: $500 - ID: waBa8vKrm8RrnoXFX9Ox5aVJMF-pTFQVT_w8Bd6xdEUKECWh7crWvRmOes1z9gnBfNcKCmarB2H3jdXVJqm_Xw

=== Example 1: Begin Transaction ===
Transaction ID: fac0ac69-3dfb-46c9-b772-f961f0644836

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
   ✅ Script saved: mXffO6t28NQb868CBLnOwYvGP8BYxuZT6d8KoXXLXtJEQ09BPq6BHMpi5bj_4zy3TX4pcjnBQZhnfB8khMIHXQ

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: p2knr01uiJu1H5qZ63Crmdq2Hr66v0BwZBoT5Bu0mrVZ_sVlExDNOl6WA-Aw1uBdGTCIXl7u9NtqAwudVrkj2g
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}
   📋 Email: {'value': 'alice@example.com', 'type': 'String'}
   📋 Status: {'type': 'String', 'value': 'pending'}
   📋 Credits: {'value': 0, 'type': 'Integer'}

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: y0Xm6B-ekNqdakdJ4LlowK9RRWX2YbVL2hQUuaXWmK-aWL3aWO_gaIOwuRi9i-5_QAYVmInpeIXQIxoYFpX75Q

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
   ✅ Script saved: tMl3EtJv6NV5svTXYLclpYrdGH_UmxjiVu4gbXQQZNZuoDjIvRv1QMgmB3TnKZs1b6ynTbwamDXvMxduUQp2CQ

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
   ✅ Script saved: eYpBAPOP22LNGJVUHT78vEnUG9k_tl-StIZiPGCi8RpXjo1iv3D5dFCpIMMCd4YVldwLuIASqFIkcA02qKBf3w

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: mXffO6t28NQb868CBLnO...
   ✅ Deleted script: y0Xm6B-ekNqdakdJ4Llo...
   ✅ Deleted script: tMl3EtJv6NV5svTXYLcl...
   ✅ Deleted script: eYpBAPOP22LNGJVUHT78...
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
📦 Built wheel for abi3 Python ≥ 3.8 to /Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.6.0-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel for current platform...[0m
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.6.0-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.6.0
    Uninstalling ekodb_client-0.6.0:
      Successfully uninstalled ekodb_client-0.6.0
Successfully installed ekodb-client-0.6.0
✅ [32mPython client package built and installed![0m
🧪 [36mRunning Python client library examples...[0m
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {'id': 'fVsT2QFTJak8_IVXXGszDJaXykigeMToHH1QslqG2AWtcqj6u7UztJLrzI3PdVBW5rwRYQdnL2XetQbU0mD2Qw'}

=== Find by ID ===
Found: {'value': {'value': 42, 'type': 'Integer'}, 'name': {'type': 'String', 'value': 'Test Record'}, 'id': 'fVsT2QFTJak8_IVXXGszDJaXykigeMToHH1QslqG2AWtcqj6u7UztJLrzI3PdVBW5rwRYQdnL2XetQbU0mD2Qw', 'active': {'type': 'Boolean', 'value': True}}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'active': {'type': 'Boolean', 'value': True}, 'value': {'type': 'Integer', 'value': 100}, 'id': 'fVsT2QFTJak8_IVXXGszDJaXykigeMToHH1QslqG2AWtcqj6u7UztJLrzI3PdVBW5rwRYQdnL2XetQbU0mD2Qw', 'name': {'value': 'Updated Record', 'type': 'String'}}

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

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: "3iDvykMhNCrl2VzWg1ExjByB1Qn5zw58-ksHh1ZEbD6zZIhgXYrQQDb-S2Zt4uXiZaVKzI8fwPORdYVpeTTSlw"

=== List Collections ===
Total collections: 10
Sample collections: ['chat_messages__ek0_testing', 'client_collection_management_python', 'test_accounts', 'websocket_test', 'ttl_cache']

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: False

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: I2lBYGb3A00NOTKTXmK-0VJjnofHzQHPmIDlWwWr7AhNK-I2xeOv4jh10wAyBuKGqQQ5tPLVtqMbfSpk2q0nmw

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: DWft8VoFogdZOOJFhv1FvjfabzXEa-UX9jKYef9dKSsYjBTRrcjblk2IJUoP6bVFE4Xp4RW0qzzw5izHx5DABw

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
✓ Inserted test record: 2ZRdkkdqHM0fzEgtj8_t3pXg0mVinohGMPx8GsWnyQ2z61S3lz5Sn9G-Bh16uGocgNViNwb8hsZUzEhbsvR8TQ

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
✓ Inserted document with TTL: qk9hFhy6gN-MT31zFCsAtU9ILapKkchHWwFwSS8P0WUU3u_DkO4EZC1yV_5A5OA7SQ3uIvHz8FH5Ob8FYuHv1g

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
  1. Score: 26.400, Matched: bio.value, bio, title, title.value
  2. Score: 26.400, Matched: title.value, bio.value, bio, title
  3. Score: 26.400, Matched: title.value, bio.value, title, bio
  4. Score: 26.400, Matched: bio, title, title.value, bio.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title.value, bio.value, title, bio
  2. Score: 39.600, Matched: title.value, bio.value, title, bio
  3. Score: 39.600, Matched: title.value, bio, bio.value, title
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
  1. Score: 0.755
  2. Score: 0.736
  3. Score: 0.728

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.729, Matched: content, title.value, title, content.value
  2. Score: 1.110, Matched: title, content.value, content, title.value
  3. Score: 0.515, Matched: 

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
✓ Created session: KQOalY4bPJPqNB915OmVLg_bRHONknf6mt2pN34QtbgXg1de6yTko9Mp822in1Pivy0H5-NxZjwyBR7wcoMXTA

=== Sending Chat Message ===
Message ID: FjQ6L8BUyxgC6kKqB4NYF6ATzJC5UIZWFO6tpvh52M8Atp8Ha0TCNbWlC8l1g5mUJjmc3h2h7KsuBE1hUXQucw

=== AI Response ===
There are three products available:

1. "ekoDB Cloud" is a fully managed cloud database service product. Its price is $499.
2. "ekoDB Pro" is an enterprise edition product with advanced features. It is priced at $299.
3. "ekoDB" is a high-performance database product with AI capabilities. It costs $99.

Execution Time: 2689ms

=== Token Usage ===
Prompt tokens: 609
Completion tokens: 72
Total tokens: 681

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 098hQxXsSWrfeco-Rh28tx65AW2CPFqfirZRpCKkdrdI-eH8o4gZ9XROWCgCRUEiFNQphpO4aCi05xZJ_pgJMQ

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information available, there is a product named "ekoDB". It is a high-performance database product. The price for this product is $99.

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: oG7gkN7o7INnE2MbLkPPo5UGsO0U_G-9MVJIbdjtYBjuBeWQ3BTfnCdaiZjCUJINr3cTELHlQHlYu6I0YR1EDA
  Parent: 098hQxXsSWrfeco-Rh28tx65AW2CPFqfirZRpCKkdrdI-eH8o4gZ9XROWCgCRUEiFNQphpO4aCi05xZJ_pgJMQ

=== Listing Sessions ===
✓ Found 6 sessions
  Session 1: oG7gkN7o7INnE2MbLkPPo5UGsO0U_G-9MVJIbdjtYBjuBeWQ3BTfnCdaiZjCUJINr3cTELHlQHlYu6I0YR1EDA (Untitled)
  Session 2: 098hQxXsSWrfeco-Rh28tx65AW2CPFqfirZRpCKkdrdI-eH8o4gZ9XROWCgCRUEiFNQphpO4aCi05xZJ_pgJMQ (Untitled)
  Session 3: KQOalY4bPJPqNB915OmVLg_bRHONknf6mt2pN34QtbgXg1de6yTko9Mp822in1Pivy0H5-NxZjwyBR7wcoMXTA (Untitled)
  Session 4: OUSEtgQ_-7kKWGMt4AmVYrdXPPGik3263KCnNt9ueJAKP1k4kK_PUO1YW4JlGYF-TG2Tmcwoh-qhMRAInCKHkA (Untitled)
  Session 5: G4DhRHz9-8gw8a7-nQBcBLd29RZmU5B9ERthrec45Dhw07EDfqLMUnzuBtpz7-EnRrin2N2AEJOh2lhgCH-HZA (Untitled)
  Session 6: LsrAmQyiFnof-6HfYut53SABCHrDvt2bS-ATTomrdt03ouabmWG3DHX-iGnYDsyVIYmH3pxEdrdeqf0cTWaG8A (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: oG7gkN7o7INnE2MbLkPPo5UGsO0U_G-9MVJIbdjtYBjuBeWQ3BTfnCdaiZjCUJINr3cTELHlQHlYu6I0YR1EDA

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: wN7NSjooXs-J7nzhl5nCO0938qabFU2bU8ictghNl0Gcafsqdm1xkvXnAH-8NXoDn9w6i-C2Pxq3Z1Ms1EwM7Q

=== Sending Initial Message ===
✓ Message sent
  Response: The product available based on the context is "ekoDB". It's a high-performance database product priced at $99.

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['chat_id', 'role', 'id', 'created_at', 'content', 'token_usage', 'context_snippets', 'updated_at'])
Debug: First message role: {'value': 'assistant', 'type': 'String'}
=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of ekoDB is 99.

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: IldEnBWbEmtuzluqWP2jIEkU5fEl-tkgvYm_Ws_4Txx-zvPQ4p6OrzVY6M-ejEo1hk1UOA5yZ1ceJxqIT73qgw
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

✅ Script saved: rPi461iJQoLodRrqiy0R_nIrIlEGfXsLcpYQythRbR7A7VzhQeJ02XhnBsyujadF-yCoNmmF3iAHC-xDh_-gKw
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved: 5ZDIuPWacmHsRuQCTYl9AhnvY5GvC38ao8XNf9ivubyciiEhl2NJsCA_-rdokcC1fcSu_uscHIUs9cPZobNVpQ
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: 7ydNywRyfeW4DpxISuKQ8tFRhRD_8jIvNQVYzDqXpYceoGE6ZIExTGXFg19DIqkq5k06EyUC41skAAiTZUqvAw
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

✅ Script saved: MLUHIkkc-BFGRhPPxFoY5rBkEyv4H18WDhAokqYLu_VZThY_UxxWguhresfpdGm-ECRy7OfmPF_lR1Y3sChoGw
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
✅ [32mPython client examples complete![0m
