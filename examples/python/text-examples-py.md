make test-examples-python
🧪 [36mRunning Python examples (direct HTTP/WebSocket)...[0m
=== Simple CRUD Operations (Direct HTTP) ===

✓ Authentication successful

=== Insert Document ===
Inserted: {'id': 'vrEzxNQ9JQIVtivsyK6K9aAroX-JDabRolwesfcnXYqarEft051_EeMIB8bBS2j8LoMo-riQ3i4q5TnhVXLncA'}

=== Find by ID ===
Found: {'value': 42, 'active': True, 'id': 'vrEzxNQ9JQIVtivsyK6K9aAroX-JDabRolwesfcnXYqarEft051_EeMIB8bBS2j8LoMo-riQ3i4q5TnhVXLncA', 'name': 'Test Record'}

=== Find with Query ===
Found documents: [{'active': {'type': 'Boolean', 'value': True}, 'id': 'vrEzxNQ9JQIVtivsyK6K9aAroX-JDabRolwesfcnXYqarEft051_EeMIB8bBS2j8LoMo-riQ3i4q5TnhVXLncA', 'name': {'value': 'Test Record', 'type': 'String'}, 'value': {'value': 42, 'type': 'Integer'}}]

=== Update Document ===
Updated: {'active': {'value': True, 'type': 'Boolean'}, 'id': 'vrEzxNQ9JQIVtivsyK6K9aAroX-JDabRolwesfcnXYqarEft051_EeMIB8bBS2j8LoMo-riQ3i4q5TnhVXLncA', 'value': {'type': 'Integer', 'value': 100}, 'name': {'type': 'String', 'value': 'Updated Record'}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: pcH7DNujG1CstAOdqZ9y9CWA6UOITf7bMp3t8WIPEoFoCrnix0MgigTh0pgJ8vQZKPG4SajrkXJxvH3_GZD-MA

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
        "id": "pcH7DNujG1CstAOdqZ9y9CWA6UOITf7bMp3t8WIPEoFoCrnix0MgigTh0pgJ8vQZKPG4SajrkXJxvH3_GZD-MA",
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
        "id": "hqv3ZeE80SGWiiGHjNPjmodyzWaTYy5FkLK3O98nFYXHTzeIusk1MyTtfMT4roJOF9NhJp4yMqsxgGZOgbaAiA",
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

✅ Script saved: eHj7TwLpHsULSr_Z28vUX6G8HdzgmG3mWJMzA8wcNlqbNZWUuXt4XdMN_b1JC__Jd5DhvhwDI1LuqdQj8h1sTg
📊 Found 35 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: ZJ9OKdcpF9dSlSdz-dcO9ab8crPeTIUVAtHI9psNA_IBQXyCHUvCFnQQGMfrJ93RHg8u_CZehoFmenkQ5BoA9Q
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: GfPe8lR70RMBjnX_LD4qEp2_w_dZBC2ipEddcx2z40S8YLU_zOyvLkK10LRPJPju_VkNT9JV1orYTTfpyWCc4Q
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {'avg_score': 50.0, 'status': 'inactive', 'count': 15, 'max_score': 90}
   {'max_score': 100, 'status': 'active', 'count': 15, 'avg_score': 60.0}

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
Collection created with first record: CF_LcEq1hUauv4TzhaWMtOS6PvINDSidfSSI5Eu5QG8PsWvGaFJL-X00ufQJbVjXrk6gk4GqeczqzhigqTHbuQ

=== List Collections ===
Total collections: 13
Sample collections: ['batch_users', 'ttl_cache', 'chat_configurations__ek0_testing', 'websocket_test', 'chat_messages__ek0_testing']

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
Created Alice: $1000 - ID: gS7B6vjlDXeph0CnTjFowBM4iKqVfWj219rKOme4JBl1dDGVuik05IIOJWS1_Bpk7u8Rab9pGO8uZt5sgYQGMQ
Created Bob: $500 - ID: MW6yNyZ855wIgbownX-JpopjtZ-ZBy1q9MRX70Px3i67UhZ6mU1R8BQEeW8BmFcyNGtB19Hrcoq9ZYTpEeY3Ew

=== Example 1: Begin Transaction ===
Transaction ID: 0bbabe2b-1788-4b98-8349-c610ff7d642b

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
New transaction: ff1cecfc-037a-4ded-96da-525e6056772e
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: UU8fgwdjjGPDmIA8CU7XRbkEXdraLFs8_87ANliCTWXMn_C96tBliqUBeaKSQKpJ59YDeRndUXTb_IXycY7WDA
Created Bob: $500 - ID: ljQayUzdy5gf2aA7NeoyAWn4FF2d3N1xz5ep5p78H4mnSVf5RXJmsiLinSpFPZEUpkhRqkESsPcHrJQXSJoxCQ

=== Example 1: Begin Transaction ===
Transaction ID: 6a1adc2a-f535-4353-ba06-966326c1358b

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
Created Alice: $1000 - ID: pfz2_kBzipTK24FdyOgZ9RaHPzzUsHltotVXt3DqIR-YflcV6JT3mEF1-dABBBqDKEJMi-nORjLPVnPBjJUZvQ
Created Bob: $500 - ID: pge6r2-Zn9VS7XYuqiSpjFybtzZQTozkHYRVDs0m82ef3Dl0FxzVcO6lV0QEjuCaTwZ6tAau5GTyaZS4xfIvOA

=== Example 1: Begin Transaction ===
Transaction ID: 810e1652-3a91-43cc-87ee-4e0141b38496

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
New transaction: 0e70b1b6-3b3f-4a38-814c-b2bdcb9edf29
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: Zkd_Y3PAovP1xbuJVClQBXs9zl8wF83PnoIAuZm6iAgCjarsHz0rLqluHmXTV6BeKgxxg2R87IWbut_wMHy3bA
Created Bob: $500 - ID: WHTJrn96vtuGlF_DTEWEf1Thf2QQZPCCtziFmzDBDMZGBf34SxNmw8GRk4Ob8s_wQ9Meo-qfWc4IofCe2d7Aew

=== Example 1: Begin Transaction ===
Transaction ID: dd8bd348-d64d-41ca-8656-71d78d8c0a09

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
Created Alice: $1000 - ID: HF3lihnVo5mY2qngquPshIuuSBldo6oZV-iL19wjymmWddqMadl4jE6BGNnpJSaURjOrd5oZjtJasCZdE6uYDQ
Created Bob: $500 - ID: YFHSxezww6VvGUoh523wC3qDwHjn_23GEn37lwLrDptM1k3M7wbGmALpUP7DbCw_tD00y-_z0ii3kV0hLTvzlw

=== Example 1: Begin Transaction ===
Transaction ID: 79fd3a10-6896-4dae-8fd7-a9c38e22106d

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
New transaction: 37be743f-3b3a-496a-89a1-7d24fe5a8251
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: OTQr20MyzLmO4wwt8S6ZN0JzWIld3MAG0XfyLbTyPcB9OlJgNyjy-OuBuvwzktNP3gv67kVm4zTinsTzRci1qw
Created Bob: $500 - ID: c3O7urB9sqz2oLtMt1NYM5XuPXhOFpgF3Y6AW1YzIdTGFDWyJl5wrRqtD-pkMwuJKa4RUdHXZuavldPCKGUk-Q

=== Example 1: Begin Transaction ===
Transaction ID: c20082ad-b6b8-40ef-a445-8e798b20d6e3

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
   ✅ Script saved: FUTOGegkMvDj2HW9gpuHZvblqjNEMFY4N3b9fm54UfJGKhPnwqN9C3eKMVN1qzYY3UCsgIEVSgINLJH4NXWp3w

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: VF0juGCa5fF4cZHsE7NwjkKd3hTA8J5fB32ON1DP01YAobDlyMGn54lHo6jm79PL3B5-BRaknwf5YOoN3NF8zQ
   📋 Name: {'value': 'Alice Smith', 'type': 'String'}
   📋 Email: {'type': 'String', 'value': 'alice@example.com'}
   📋 Status: {'type': 'String', 'value': 'pending'}
   📋 Credits: {'type': 'Integer', 'value': 0}

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: YNgs8UXbs9o70qGNxxg9_9mUEL7V4eESgkxfRuuM6ewja9IGHiE3NjVCx1JbC2ztCbsvWXNudNuTcNSr9eAXyg

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
   ✅ Script saved: zgHhsBN3Aon2Jru28Pdm_yK1Pn0SUku2ZgF-NXGszU3nZPiDYjLLy28BjX9PS3E6rATACXWCCNTxvrsWkLeozA

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
   ✅ Script saved: XHZeiBQd0U4APiikKYo0n0vLXb73Ntp4tyH9u9AMYjxRql7aqBdez7GeudGAm6Ax-Q_-byNwQX6U8rm5-2MQFw

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: FUTOGegkMvDj2HW9gpuH...
   ✅ Deleted script: YNgs8UXbs9o70qGNxxg9...
   ✅ Deleted script: zgHhsBN3Aon2Jru28Pdm...
   ✅ Deleted script: XHZeiBQd0U4APiikKYo0...
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
  Output: Document ID = 4co_BM9LtALdxeO6NNGcdq4KcO5pRGHIGeFPj9aj5pGXRVXEnnL-hzDqI_zSlyV4bUmSwqItInm7-eb6_zDgXw
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: find_by_id(4co_BM9LtALdxeO6NNGcdq4KcO5pRGHIGeFPj9aj5pGXRVXEnnL-hzDqI_zSlyV4bUmSwqItInm7-eb6_zDgXw)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: find_by_id(4co_BM9LtALdxeO6NNGcdq4KcO5pRGHIGeFPj9aj5pGXRVXEnnL-hzDqI_zSlyV4bUmSwqItInm7-eb6_zDgXw)
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
  Output: Document ID = GlcsqcLGq05uIgnXWQcETlrOj-lDW25nUZNlqMNAhQskZsju-ixBsOtK6s1DgPiUQ5AOxXFNyJW_zUca0_fs1w
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: find_by_id(GlcsqcLGq05uIgnXWQcETlrOj-lDW25nUZNlqMNAhQskZsju-ixBsOtK6s1DgPiUQ5AOxXFNyJW_zUca0_fs1w)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: find_by_id(GlcsqcLGq05uIgnXWQcETlrOj-lDW25nUZNlqMNAhQskZsju-ixBsOtK6s1DgPiUQ5AOxXFNyJW_zUca0_fs1w)
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
Inserted: {'id': 'b7wRm3dVMDUt-HprVTJmF_9d2Pb_CwdqqNhLi__CjgjxdhzVkvoV5ZW_RyNJIUar8vcpacToK9uXD3nUXJZjdw'}

=== Find by ID ===
Found: {'active': True, 'price': 99.99, 'metadata': {'nested': {'deep': True}, 'key': 'value'}, 'name': 'Test Record', 'value': 42, 'created_at': '2026-01-26T10:05:26.236163', 'data': 'aGVsbG8gd29ybGQ=', 'id': 'b7wRm3dVMDUt-HprVTJmF_9d2Pb_CwdqqNhLi__CjgjxdhzVkvoV5ZW_RyNJIUar8vcpacToK9uXD3nUXJZjdw', 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5], 'user_id': '550e8400-e29b-41d4-a716-446655440000', 'categories': ['electronics', 'computers'], 'tags': ['tag1', 'tag2', 'tag3']}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): True
  price (Decimal): 99.99
  created_at (DateTime): 2026-01-26 10:05:26.236163
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): ['tag1', 'tag2', 'tag3']
  metadata (Object): {'nested': {'deep': True}, 'key': 'value'}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): ['electronics', 'computers']
  data (Bytes): 11 bytes
Plain record: {'active': True, 'price': 99.99, 'metadata': {'nested': {'deep': True}, 'key': 'value'}, 'name': 'Test Record', 'value': 42, 'created_at': '2026-01-26T10:05:26.236163', 'data': 'aGVsbG8gd29ybGQ=', 'id': 'b7wRm3dVMDUt-HprVTJmF_9d2Pb_CwdqqNhLi__CjgjxdhzVkvoV5ZW_RyNJIUar8vcpacToK9uXD3nUXJZjdw', 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5], 'user_id': '550e8400-e29b-41d4-a716-446655440000', 'categories': ['electronics', 'computers'], 'tags': ['tag1', 'tag2', 'tag3']}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'tags': {'type': 'Array', 'value': ['tag1', 'tag2', 'tag3']}, 'active': {'type': 'Boolean', 'value': True}, 'id': 'b7wRm3dVMDUt-HprVTJmF_9d2Pb_CwdqqNhLi__CjgjxdhzVkvoV5ZW_RyNJIUar8vcpacToK9uXD3nUXJZjdw', 'metadata': {'value': {'key': 'value', 'nested': {'deep': True}}, 'type': 'Object'}, 'created_at': {'type': 'String', 'value': '2026-01-26T10:05:26.236163'}, 'embedding': {'type': 'Array', 'value': [0.1, 0.2, 0.3, 0.4, 0.5]}, 'value': {'value': 100, 'type': 'Integer'}, 'data': {'value': 'aGVsbG8gd29ybGQ=', 'type': 'String'}, 'user_id': {'value': '550e8400-e29b-41d4-a716-446655440000', 'type': 'String'}, 'categories': {'value': ['electronics', 'computers'], 'type': 'Array'}, 'price': {'value': 99.99, 'type': 'Float'}, 'name': {'type': 'String', 'value': 'Updated Record'}}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: VtqkhNI9pBaC9Xnq_VE1KOAywavpUWBiniAkKzYKuVGJIhy5-Y7-Mfp8Uqh9tXh6IVBOZWyt56z2-eZUgYTDpw

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
Collection created with first record: "OgK6NySqIaknDkrilU_rIqrMfQhm1htT4hexpU-ygxbYIu0nChgwQYeVx5UW9x6JhuFQ2OHaXyRrS45SYkWkqg"

=== List Collections ===
Total collections: 12
Sample collections: ['batch_users', 'ttl_cache', 'chat_configurations__ek0_testing', 'websocket_test', 'chat_messages__ek0_testing']

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
Created Alice: $1000 - ID: ByHk7a8LrtMNKiTKVVCsPR_ivUIHOIqBKkHn4W3kk1lrb6fnU8DRjf9WxLwkvLhKQbarwmCNui8nXm6usWRfxA
Created Bob: $500 - ID: vUF2ySnZC6giPEIxixngTgQTKpEc50MKisXpqQItDaO8rfuG1Eg9nIzzeNU8ll60au4tArsHZ0pyFYg8I8KhCg

=== Example 1: Begin Transaction ===
Transaction ID: af849815-f06d-4187-9b5c-7c26d466f177

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 178272d1-3a17-4a16-92de-0ce76dac1dce
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
  1. Score: 25.740, Matched: name.value, name, email.value, email
  2. Score: 12.540, Matched: name.value, name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title.value, title, bio, bio.value
  2. Score: 26.400, Matched: bio, title.value, bio.value, title
  3. Score: 26.400, Matched: title, bio.value, title.value, bio
  4. Score: 26.400, Matched: bio.value, title.value, title, bio

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title, bio, title.value, bio.value
  2. Score: 39.600, Matched: title.value, bio, title, bio.value
  3. Score: 39.600, Matched: title.value, bio.value, title, bio
  4. Score: 39.600, Matched: bio, bio.value, title.value, title

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio.value, bio

5. Search with stemming and exact match boosting:
Found 2 results (matches: run, running, runs)
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.767
  2. Score: 0.753
  3. Score: 0.730

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.365, Matched: title.value, content, content.value, title
  2. Score: 1.377, Matched: content, title.value, title, content.value
  3. Score: 0.384, Matched: 

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
✓ Inserted document: 3GVkQm-Og-e5qmJcKjkabJiMDE-Gqn4a2Qf_ojM9iFUjNb8xnryTpV88H5erOxrOUPaubL4ujE_ZQnjJnqK_1A

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: -x7p5T5aalEZQXden9KxWo8qN5r5fO6cKMGFrpd6rfEv4lO-kdwY0p3FXmfb6G9GF266RbI08pP376vZ9JFVrA

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
✓ Inserted document with TTL: npumN7zX9umAuWLHhiA90F1kGAhWZKiKNtkjJgD5uAeK7xqkg1ZOeBc_VUBYIyP73d-NsRa-XUMA0nl6MH9QQA

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
✓ Edge cache script created: QwKyDW-ykfjzxRdpGQ0KGcDOpXVGuOFllgpR4IAkleAXzlcsNwgrlPpYFTMNJ0DbfQlMRK7TbvA0PUcMa0w05Q

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

✅ Script saved: 63FDIpNsHEI0RIjsj2rd5WUUrNRJ72ncY2oZMk55OANhxjppBOO928ei-iaqPXZEBLR4G3taHK1_HD38rSNvbw
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved: y4v9XdFkJRdsx-oqzY793yy8AbvWz98XCYuP7E6FHcgaXBzAJV6xcsEQfFfNECXV-aDs6OY6t03MTKdR8ViaXw
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: AMez3ebxiK6HFUBqpspcnC9y9gbrCLSA2Ct4Qe8kbSWdUYDikBkwhaU36k-deTtlxcw5ThybUWVRvJSUyNsHLg
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

✅ Saved reusable function: fetch_and_store_user
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 68.2ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 68.2ms
   📊 Records: 1
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

✅ Script saved: TCS4EFN0hUoz5_M5ZZxuQjyRq_khFsiXja3tSdmhH4Mq-ColiamJB678wCD0ZVw3ehCABZkX0GE-6diROaZcOw
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
🚀 ekoDB Python KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: r6z9phb_9M6u8qhGBdLFRnqwWlGS8CSkKJIMYpG1ldCOPaIPDurVfGFj_7aiu-NCzZB19IY24_ybPT9vtzuTeQ
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

✅ Inserted order: gzOfWkM2sqM_StvsDOd0sUiD_jkObm1eIv6kilw62dIxVB3_P7plbNUNpUk9K-1NzTsogVbB15FrYQrF-jUKyw
✅ Cached order status
📊 Quick status lookup: {'value': '{"status":"processing","updated_at":"2026-01-26T15:05:28.210923+00:00"}'}

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
✓ Created SWR script: fetch_api_user_py (y4YJKf8vuyW8webFL7VjTugDGscirNEidv9THE7qwqy5E8C-PHESaEjaC-xJnk9lH6gs6KpOtzh0Xv2eB_gOMg)

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
   Vector databases offer advantages like efficient storage, precise representation of data, easy updating of data, and support for complex geospatial operations. They can represent data at its original resolution and form without generalization, allowing for accurate analysis and visualization.
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
   1. Database Design Principles (Database)
   2. Vector Databases Explained (Database)
   3. Natural Language Processing (AI)
   4. Introduction to Machine Learning (AI)
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
✓ Created session: WEGypk7qoA_kChE-4sfyjDdjSAsXDRjBIfhNexFlxd_H5EtOlhbH93hvkk0xguAi-36XXCmnH-riuzJDgvxy9A

=== Sending Chat Message ===
Message ID: kksSujEhPno9dpvU_hrXz40hWjXtFl3yU7fJDcFcZLDn-IeBDQvh5TH2Xis_sq88lFnidsplvMFhL1lzQ5mYfg

=== AI Response ===
There are three products available:

1. "ekoDB Cloud" is a fully managed cloud database service product. It's priced at $499.
2. "ekoDB Pro" is an Enterprise edition product with advanced features. It's priced at $299.
3. "ekoDB" is a high-performance database product with AI capabilities. It's priced at $99.

Execution Time: 3313ms

=== Token Usage ===
Prompt tokens: 451
Completion tokens: 75
Total tokens: 526

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: OFFrX1H5oI5KpzWJdHqVRQiYBihREPth34qRCv4_-MMnU15k3L7lDOWxfcqTaOhRVjyuVfo9IRuJvBgdpOlOag

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information provided, there is a product available named "ekoDB". This is a high-performance database product. It is priced at $99.

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['chat_id', 'content', 'token_usage', 'context_snippets', 'created_at', 'id', 'role', 'updated_at'])
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
✓ Created second session: O9hPELmjeoF5xcC-AXO5BYlAw51P13QCevQPKOms7urZi09TugPjOlCyJwNaWCAPQHtBPKSG5G77SG5izjXBdQ
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
✓ Created session: HFG9kxwFQSLLrPWzsffCsQoAqtrWGP867ZAGED-kiFKNCjBAroYOXSMr96SDrNcSy6IB22lwZ6pUe59bwrjPuA

=== Sending Messages ===
✓ Message 1 sent
  Response: The product available is called "ekoDB". This is a high-performance database product. It is priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: Uqa-e6jauAFF1XHxkVHvgWYmr-aI6IvH7brvBD-qfgGlMTLZ963fV4Ssdi_3wppjiKNqDqJRCJMRUE7-aRyBBQ
  Parent: HFG9kxwFQSLLrPWzsffCsQoAqtrWGP867ZAGED-kiFKNCjBAroYOXSMr96SDrNcSy6IB22lwZ6pUe59bwrjPuA

=== Listing Sessions ===
✓ Found 7 sessions
  Session 1: Uqa-e6jauAFF1XHxkVHvgWYmr-aI6IvH7brvBD-qfgGlMTLZ963fV4Ssdi_3wppjiKNqDqJRCJMRUE7-aRyBBQ (Untitled)
  Session 2: HFG9kxwFQSLLrPWzsffCsQoAqtrWGP867ZAGED-kiFKNCjBAroYOXSMr96SDrNcSy6IB22lwZ6pUe59bwrjPuA (Untitled)
  Session 3: O9hPELmjeoF5xcC-AXO5BYlAw51P13QCevQPKOms7urZi09TugPjOlCyJwNaWCAPQHtBPKSG5G77SG5izjXBdQ (Untitled)
  Session 4: WEGypk7qoA_kChE-4sfyjDdjSAsXDRjBIfhNexFlxd_H5EtOlhbH93hvkk0xguAi-36XXCmnH-riuzJDgvxy9A (Untitled)
  Session 5: b3lg9Yy6htKsJGVHXEZH9XVx52hF9v35CMNfcM9ODLsCTUXxJBBw7oPw2uhR0kv2BowTcwuqcUIJOd2LaNzNyQ (Untitled)
  Session 6: We-cGv7b94MoxJd3A7vLwsVvnPb2qnW2A2SPcwXauV2x_6VwPpPz28nGCnhz1NGsCtHfNXyOfLwtFENoEJ2KHg (Untitled)
  Session 7: bZ-7B3w-TbSQt3kqjj27W2ZHe4J10z8GsnkV781WYxzhZK0OIgmiWjzyKuoXWNnvtWFIDo8zcO4S6xPvVHkDRA (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: Uqa-e6jauAFF1XHxkVHvgWYmr-aI6IvH7brvBD-qfgGlMTLZ963fV4Ssdi_3wppjiKNqDqJRCJMRUE7-aRyBBQ

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Dict Creation ===
✓ Created record with plain dict: {'id': 'lUCLvUrsy9R2tZOS8np_gipsymE9w5bY5G0398PmnWWEGF6eTyhwmX880hK0PzGbfqFOFTFetjPXwl0YSwJRrg'}

=== Upsert Operation ===
✓ Upsert (update existing record): lUCLvUrsy9R2tZOS8np_gipsymE9w5bY5G0398PmnWWEGF6eTyhwmX880hK0PzGbfqFOFTFetjPXwl0YSwJRrg
✓ Inserted second record: Mk3HkFeCyDYWxYddD1HchfRXH9jnDO1-5XqqOuk4cA07dswarM2MZ-M9xTvU-wAhmb-cu4ckfb6DZ5al1-atlw
✓ Upsert (update second record): Mk3HkFeCyDYWxYddD1HchfRXH9jnDO1-5XqqOuk4cA07dswarM2MZ-M9xTvU-wAhmb-cu4ckfb6DZ5al1-atlw

=== Find One Operation ===
✓ Found user by email: {'age': {'type': 'Integer', 'value': 29}, 'name': {'value': 'Alice Johnson', 'type': 'String'}, 'active': {'type': 'Boolean', 'value': True}, 'email': {'type': 'String', 'value': 'alice.j@newdomain.com'}, 'id': 'lUCLvUrsy9R2tZOS8np_gipsymE9w5bY5G0398PmnWWEGF6eTyhwmX880hK0PzGbfqFOFTFetjPXwl0YSwJRrg'}
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
   Inserted with ripple: {'id': 'vz5iQDMX2PduLnQNHs15AxwpIioIkMp5QqYtQ0OQFjxNWZ2jhpk-OUk7eGsjR2l1PCJI0kRsLxQYG5Oh9tM-zg'}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {'id': 'k0zi4P5vKUF-440YdxwR0EzMvCjezHBCFDVM4hiLet2BkwPHqdfmn5rTXjoPl1pV-4lXxJQz1fgXKxUR7Hdsgg'}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {'price': {'value': 150, 'type': 'Integer'}, 'id': 'vz5iQDMX2PduLnQNHs15AxwpIioIkMp5QqYtQ0OQFjxNWZ2jhpk-OUk7eGsjR2l1PCJI0kRsLxQYG5Oh9tM-zg', 'name': {'value': 'Product 1', 'type': 'String'}}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {'name': {'type': 'String', 'value': 'Upsert Product'}, 'id': 'vz5iQDMX2PduLnQNHs15AxwpIioIkMp5QqYtQ0OQFjxNWZ2jhpk-OUk7eGsjR2l1PCJI0kRsLxQYG5Oh9tM-zg', 'price': {'type': 'Integer', 'value': 500}}

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
