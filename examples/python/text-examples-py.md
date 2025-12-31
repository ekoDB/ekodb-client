make test-examples-python
🧪 [36mRunning Python examples (direct HTTP/WebSocket)...[0m
✓ Authentication successful

=== Insert Document ===
Inserted: {'id': '1AtHKcn9CuPYRp4FSYPwRI7WDgRuuoB81NCIw1e622yAKrZEv3yPJ-oBVkq7BBVlJkALHMNKivZW9fH6hjlDdw'}

=== Find by ID ===
Found: {'id': '1AtHKcn9CuPYRp4FSYPwRI7WDgRuuoB81NCIw1e622yAKrZEv3yPJ-oBVkq7BBVlJkALHMNKivZW9fH6hjlDdw', 'value': {'type': 'Integer', 'value': 42}, 'name': {'value': 'Test Record', 'type': 'String'}, 'active': {'value': True, 'type': 'Boolean'}}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'value': {'value': 100, 'type': 'Integer'}, 'active': {'value': True, 'type': 'Boolean'}, 'name': {'type': 'String', 'value': 'Updated Record'}, 'id': '1AtHKcn9CuPYRp4FSYPwRI7WDgRuuoB81NCIw1e622yAKrZEv3yPJ-oBVkq7BBVlJkALHMNKivZW9fH6hjlDdw'}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: OzuHNu3HBCQJv4o_AkTX0qjzwIKg-qfalYHGh1k66UfW3VutE3O01Y0n_pg4otwK3O-pyXBVTeGvX6uut0kfyg

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
        "id": "1keYPJvW6FB7hGaXsqhyCZ7CwoMvaiXk_VaFjue1b3Rx9dZwlmdq3_LXzLW6-GYcNI_dkQ0efAoScYSbPFuB4g",
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
        "id": "OzuHNu3HBCQJv4o_AkTX0qjzwIKg-qfalYHGh1k66UfW3VutE3O01Y0n_pg4otwK3O-pyXBVTeGvX6uut0kfyg",
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
cache:product:1: {'price': 29.99, 'name': 'Product 1'}
cache:product:2: {'price': 39.989999999999995, 'name': 'Product 2'}
cache:product:3: {'name': 'Product 3', 'price': 49.989999999999995}

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: wHuYll1Qa-v1w_yKPEpdWyfiAn-K-O6TaO5pmVE6uTWwRQZNQHicAaILtbQHToz260AYSdF0Kw86gwrJRwe2Nw

=== List Collections ===
Total collections: 11
Sample collections: ['test_collection', 'chat_messages__ek0_testing', 'test_accounts', 'demo_collection', 'batch_users']

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: False

✓ All collection management operations completed successfully
✓ Authentication successful

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: 2_6dnLeIxG60BAdyGDdG27djj8yPsEGVMw9KRBUNBK9Aq-Sm9mCLmje4pDHqwhpABv5hpsupov3d1VJEQaAG7w

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
✓ Inserted document with TTL: zmnKfe3N63yRzSso1TMQRIj_X57MTvaVW7vlFXyoa-zNr5hwOfaqF7LcqniL82EnIITRcOQ04lI-vyyxmpW9tw

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 2 record(s) via WebSocket

✓ WebSocket TTL example completed successfully
🚀 ekoDB Scripts Example (Python/HTTP)

✓ Authentication successful

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: c5GgU_cXToYurBJ8A-JxoiynN8OP63Lb411S0MUCnrklE60Oaj1IeTZs_UzDwDO89h2os3CTn5Q295PAiqvtfQ
📊 Found 30 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: bZZIjs65RVK28Q8-h3Ucvm0SiWQu2C6VQ7HoeHrcPqeSGJjnUNXh6_fIUkDVIlwTNbUtwh22TozUn_liYzP_KQ
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: xJAvr2GoM6mWIeIcSq8rZNbEB_dMQwf7i1leGx_sUZVC-koJZlQu-tGCqKKPIZIkmu8hUd5z-pCZ8vis9XW-5Q
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {'status': 'inactive', 'count': 15, 'max_score': 90, 'avg_score': 50.0}
   {'max_score': 100, 'status': 'active', 'avg_score': 60.0, 'count': 15}

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
Created Alice: $1000 - ID: KnH07QJMEGkVQXjZaf3HOFtPG48D4vPF6iwR0faGKyQoWLycphuANuV42KRhHnTwXZh9hMSwStKvbqyCeFggNQ
Created Bob: $500 - ID: M5LAm3USWgzTaxxcz2PfiA4U0jZLAm08T4h9TJTu3uXJJrnFT7sTASNM_760pjfZlEFUNrnOw9JZBGeymo1RQA

=== Example 1: Begin Transaction ===
Transaction ID: f5acdf8e-496a-4872-ab9a-a8f2a3e60336

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
New transaction: b38a414f-607e-4275-bb7b-51d215d518e6
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: qAi0kT5o-9tm6ETRB2re68RqXMq4-6MAIoCJLQyxZSzuFJYiNKa_cn_LFvs-3JSPyIO0eSLLzcp9G3-ersEL-g
Created Bob: $500 - ID: 1xSn9z9Tc_UhELrsSEoNdVK0ke6yp-vxnCv8nck_UAq_q3eiOVogHiz82FB8vjucQFfDy7y9ym9Bjfe4SDLNIA

=== Example 1: Begin Transaction ===
Transaction ID: 28f7bba9-1994-4c74-9397-8a9489d99c63

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
Created Alice: $1000 - ID: y3Pb93skT45r7UkPjQNm_F5BCrtgPD2pig1drMvUKTw2ACbUy76446tZDKqu2UmAKM8idHMQvCx7wu_U_Hdfcg
Created Bob: $500 - ID: PhMHpDOb8OAuEAVDrjjoVUwusRQ1MHvAPgbXrv2vdge4Eg_c8_TOeho7EL7bKIs6Px0RL20WlkoMfZWuTgbPiQ

=== Example 1: Begin Transaction ===
Transaction ID: 523b39db-d669-4ec6-a7c9-cba98717fd7e

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
New transaction: cccdafab-7028-4660-bea1-2b75798e3b21
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: s4lSmGDXkTDxkjclMnLe21pe0yFvRzBnaOLvfXtDo6W4FCJDIXu4_k0KtNyF6FhJAZbIDkgTITvAjJqzLHctXg
Created Bob: $500 - ID: A9EGmP2spLLdjbBeHbpHtttZXkY95upTqEWSEYWEt6Aw8Kq1hfGxE6KjOAYH25uYdATVM76z86pYNvRkM_PURA

=== Example 1: Begin Transaction ===
Transaction ID: 9c17f7cc-87a7-43f0-9c28-de404f65009d

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
Created Alice: $1000 - ID: g-CPKnyqiIHYG-piHYV2c6dWMMjQrD0j36FH_xgiPSEqNEcP_ZmgBWcIj0rcZXVK7TG8kdNYkhaEHbQqZ7Qrlw
Created Bob: $500 - ID: V5jfjfriXk2Mcx9h4nIq1bjlZZA_Nx_uR71xXxR9IuraWQqcHzNzKnbImI3Fmm_GSjzmKj7Ju0lPcVuaj34WVg

=== Example 1: Begin Transaction ===
Transaction ID: 42d6e1a0-a277-4db0-8f87-130a5d6a9106

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
New transaction: 95231bdf-2041-45cf-9adf-8f0974ae8afb
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: NHYFhZqlMXjOE0Z_E_BRQSGUAplA6NH_YK6U1-uM-q3yZKpjfkPL4ERii4EaTLDuuf4Tohh0491D2_UDnPXVVw
Created Bob: $500 - ID: KHLxXlu4wNORXruVJCdLLG8MPMaAOHrFTR7lMQPcy2eGjs9WQ01fAMN6P8aW11eOX7v0A-oQmVEAFVMociGoBw

=== Example 1: Begin Transaction ===
Transaction ID: 71063f42-2aa0-4e02-8945-d078607bfedf

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
   ✅ Script saved: aVED9GRJX4uSydlNVujB-Fsmiv5Jxy5KVAIwxChkEypzn5x4h_t9zTnfwDUbTvzUWLv2vEiJSZ7FrEveHWVQ7w

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: LmrZ8sgUzOwIE_0qF8o_-Cz3YQLbgLK6vsCqDzC8kFqmpQM3PdXxW0gUibQKYlh8vvWI4e1Ljvdnw_O_n99Cow
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}
   📋 Email: {'value': 'alice@example.com', 'type': 'String'}
   📋 Status: {'type': 'String', 'value': 'pending'}
   📋 Credits: {'value': 0, 'type': 'Integer'}

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: eOrlt_hLMpxav0uJfMn46ByiqfoIcrIPmf4eCt6b0N0pN-D5gRAoSCAxzG-W9cG7rd0344rvTzSXQtQz-oFT8g

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
   ✅ Script saved: PxoFY16zFc70AIaL2LOQEm2qydRvgdXHlK1rfAY7wk2kbi8kTcCRKafZfgRw2yH_YSuhASaPfX8nh604Kpg5Vw

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
   ✅ Script saved: 48quKA2NdX1WiDvB02OK4HZgSOYLFeczDLQQ1OGALCAz6XnUzdnSYgTbcFTO8hM_Ps16u7SxsRBb1tA2vh7TuA

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: aVED9GRJX4uSydlNVujB...
   ✅ Deleted script: eOrlt_hLMpxav0uJfMn4...
   ✅ Deleted script: PxoFY16zFc70AIaL2LOQ...
   ✅ Deleted script: 48quKA2NdX1WiDvB02OK...
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
    Finished `release` profile [optimized] target(s) in 0.09s
📦 Built wheel for abi3 Python ≥ 3.8 to /Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.5.0-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel for current platform...[0m
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.5.0-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.5.0
    Uninstalling ekodb_client-0.5.0:
      Successfully uninstalled ekodb_client-0.5.0
Successfully installed ekodb-client-0.5.0
✅ [32mPython client package built and installed![0m
🧪 [36mRunning Python client library examples...[0m
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {'id': 'MV5DJqLR1GLIGEsTNth4caUFZ18x_gxTXBzQoZ55GxOUR1qd63QTH92C7BcWqWtV0N5L066AInykgiwzJRsH4w'}

=== Find by ID ===
Found: {'id': 'MV5DJqLR1GLIGEsTNth4caUFZ18x_gxTXBzQoZ55GxOUR1qd63QTH92C7BcWqWtV0N5L066AInykgiwzJRsH4w', 'name': {'type': 'String', 'value': 'Test Record'}, 'value': {'value': 42, 'type': 'Integer'}, 'active': {'value': True, 'type': 'Boolean'}}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'value': {'type': 'Integer', 'value': 100}, 'active': {'value': True, 'type': 'Boolean'}, 'id': 'MV5DJqLR1GLIGEsTNth4caUFZ18x_gxTXBzQoZ55GxOUR1qd63QTH92C7BcWqWtV0N5L066AInykgiwzJRsH4w', 'name': {'value': 'Updated Record', 'type': 'String'}}

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
Collection created with first record: "_GWmq746FRcETwQ_Qh1SllxAdZtPDQHYe-OdLQCEiyivtLhhQLLx4PB6dynlTyhBA5VKng2eBiV79fqqzO5LoA"

=== List Collections ===
Total collections: 10
Sample collections: ['test_collection', 'chat_messages__ek0_testing', 'test_accounts', 'batch_users', 'scripts__ek0_testing']

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: False

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: Yo5NWLsw45fUB4EyAylYaEVj_RzmQCyzCkmsErafPPgZ853cDJXJ4s0hz9jaFUA66yk3Y2JQ4ZeXTQEzLDujxQ

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: 6tkHzGu1jZ_V7VI9ItSLIUdDPuh5e_dNZR8jwvHUa1fmOzKc9xUdWBY-zRB9BpcMzmWZmvc-0ACmKeIGKwbv9A

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
✓ Inserted test record: rd95n4vyBYMJ6FGzJ3-iqtYzRGoTkUZ7beZg0U0cQfwgIsauBZKA7tN6k7DSYyhdZzEv2zAlQTjFRmRB8FOh5g

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
✓ Inserted document with TTL: x6RbkuyF1Z0TQyoErHkRu-we4yXXrQ9ba8fXnfS_rgry3B6fdGRVk8_eJ7014hqfantu6vpKeCruRCtO4bJxuA

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
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title.value, bio, title, bio.value
  2. Score: 26.400, Matched: title.value, title, bio.value, bio
  3. Score: 26.400, Matched: bio, title, bio.value, title.value
  4. Score: 26.400, Matched: title, bio, title.value, bio.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio, bio.value, title, title.value
  2. Score: 39.600, Matched: bio, title.value, bio.value, title
  3. Score: 39.600, Matched: title, bio, bio.value, title.value
  4. Score: 39.600, Matched: title, bio, bio.value, title.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 2 results (matches: run, running, runs)
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.768
  2. Score: 0.766
  3. Score: 0.710

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.737, Matched: title.value, content.value, content, title
  2. Score: 1.136, Matched: title.value, content, title, content.value
  3. Score: 0.497, Matched: 

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
✓ Created session: aoe6MEuL3E1EmuO5qnfz0zmj9UCxpmR5TGic-qgUuC5Tr0b6ynRYl1hobYCeqXxu4wWURBibPcRgbY8-SCF3RA

=== Sending Chat Message ===
Message ID: cZeWIg4LhqkBTahhFsFOtG90b95gP6D3-53YfV7ocwpaNu8YkTegL6JWiREC5rISOGf-PDI-syMAyEA6aahdbA

=== AI Response ===
We have three products available:

1. ekoDB Pro: This is an Enterprise edition product with advanced features. The price for this product is $299.

2. ekoDB Cloud: This is a fully managed cloud database service product. The cost of this product is $499.

3. ekoDB: This is a high-performance database product with AI capabilities. This product is priced at $99.

Execution Time: 3520ms

=== Token Usage ===
Prompt tokens: 609
Completion tokens: 83
Total tokens: 692

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: sQ2RiwmEIWTw9tgE9J9WJf_jbrC58oReuQS0RVCqqb_p9HQ2a4Yij2Fkray27UFWgq4S7lx9X_gnmIC8ySgDiw

=== Sending Messages ===
✓ Message 1 sent
  Response: Answer: One available product is the ekoDB, a high-performance database product. It is priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 5DJRPU683OG1Fl0L2A2OF6whPsszJjmAV327yfLnemyz_lPLyyin_k3-YzCAdaCqqxxDUCY5lTvHKWaDnPMEWQ
  Parent: sQ2RiwmEIWTw9tgE9J9WJf_jbrC58oReuQS0RVCqqb_p9HQ2a4Yij2Fkray27UFWgq4S7lx9X_gnmIC8ySgDiw

=== Listing Sessions ===
✓ Found 6 sessions
  Session 1: 5DJRPU683OG1Fl0L2A2OF6whPsszJjmAV327yfLnemyz_lPLyyin_k3-YzCAdaCqqxxDUCY5lTvHKWaDnPMEWQ (Untitled)
  Session 2: sQ2RiwmEIWTw9tgE9J9WJf_jbrC58oReuQS0RVCqqb_p9HQ2a4Yij2Fkray27UFWgq4S7lx9X_gnmIC8ySgDiw (Untitled)
  Session 3: aoe6MEuL3E1EmuO5qnfz0zmj9UCxpmR5TGic-qgUuC5Tr0b6ynRYl1hobYCeqXxu4wWURBibPcRgbY8-SCF3RA (Untitled)
  Session 4: Y9IlcOV2FYlrt19uHBT3nw_RiNqIdVs6MtG_7rOFYFLhEOfn0b8eJ3leVDMrFWKw2U_9gaJhiyPp1eiGoMGI4w (Untitled)
  Session 5: qBqsUD_jL1fpoveeqUPs3_yDNveMsqdq1TamMm0Xps-3pScyHXiEnh5jYsC0QQlaPWGE1YT6iUAx3PH-0JEQng (Untitled)
  Session 6: 9msyvD3h8zah5g8WdK2SPkPlQqqs_rbgazEBNOjmdoMUNl04wtkPKyhQLv3aRA1BGzDbOwuBnOMc5aT88H9h0A (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: 5DJRPU683OG1Fl0L2A2OF6whPsszJjmAV327yfLnemyz_lPLyyin_k3-YzCAdaCqqxxDUCY5lTvHKWaDnPMEWQ

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: yL7pXRBlCbxDQCriqCUaKTKT-jTMr1BMK4YHE2Afug7_AzMEtOwEKl94HVItuITmj7Xf6LqIi_NOKAxvsWmDoQ

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the context provided, we have a high-performance database product named "ekoDB" available.

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['context_snippets', 'token_usage', 'created_at', 'chat_id', 'updated_at', 'id', 'content', 'role'])
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
✓ Created second session: rPOuQWVT2njcPDiEPMECJJCqn_dUz-waRLpPt-MeiwbHxylvroUkBcOh8x-E9Voy0mCLjYHyGVIQIQGvw2Fp1g
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

✅ Script saved: 9D8UuEx1wq72JlHL1UJlYJ_rR4a77dVblYQnWMNOlz-Sx1TxPyL6-2ieXniwVpoNA-S395laN8m0ecOrTXUSQA
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved: KSVffDnO5fn36sfXVZzaXjbZ-mSOlZAESCurpQPg2ShrX8u4qpjMseaufd-Axgi4fM7Jo5tsa4s-6KcmvOQctA
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: bonS1xRyFuAU1UHibWRCzUx55fdMvop83iNgEbp6G6YcSdNyta7F8YduJ__xL6H_NH9Q-O5RlrYL_t1Q3sd44w
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

✅ Script saved: gKOf1xf9Jnnz_WmcdkUcJLWXnLRnY_2m_-6tv-AX29PcW7lj14aYvG5lhFDb8b9Q45wwd_cVjx-3MTme3CGVtg
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
