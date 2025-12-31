make test-examples-python
🧪 [36mRunning Python examples (direct HTTP/WebSocket)...[0m
✓ Authentication successful

=== Insert Document ===
Inserted: {'id': 'aQBIurWwMbk5_-em2W34NM1zX3UAwcIfhlvIMUBoKG-01yuEqmBiR6xzyCsgg8h65v6tA7kBzPDxsoEftpj_nQ'}

=== Find by ID ===
Found: {'active': {'type': 'Boolean', 'value': True}, 'id': 'aQBIurWwMbk5_-em2W34NM1zX3UAwcIfhlvIMUBoKG-01yuEqmBiR6xzyCsgg8h65v6tA7kBzPDxsoEftpj_nQ', 'value': {'type': 'Integer', 'value': 42}, 'name': {'type': 'String', 'value': 'Test Record'}}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'active': {'type': 'Boolean', 'value': True}, 'value': {'value': 100, 'type': 'Integer'}, 'id': 'aQBIurWwMbk5_-em2W34NM1zX3UAwcIfhlvIMUBoKG-01yuEqmBiR6xzyCsgg8h65v6tA7kBzPDxsoEftpj_nQ', 'name': {'value': 'Updated Record', 'type': 'String'}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: BExdULEoFaYZPfAMdPX2qmL8ebqnPB_ZMAWtSsyK8bboegtIiDf1HsJ6a_WgAl-vfT93UVHT98iu3psuczswtA

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
        "id": "BExdULEoFaYZPfAMdPX2qmL8ebqnPB_ZMAWtSsyK8bboegtIiDf1HsJ6a_WgAl-vfT93UVHT98iu3psuczswtA",
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
        "id": "OIcUeukTb5AoykKpQDNuJeW36cIo-7vd-q8gp7gPtlnHoXGHToZKYM_zNufISLbfCfwaSx6cT6aRhWPJ5ALqJA",
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
Retrieved value: {'username': 'john_doe', 'userId': 123}

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: {'price': 29.99, 'name': 'Product 1'}
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
Collection created with first record: bC72-gix1d-Io2kltIyDA4GMAJ3E3FQqbS5HuvglrSVa2k9-8k14WKCA0QkAs5eVQOR4XTdMlunD_jElIgds5w

=== List Collections ===
Total collections: 11
Sample collections: ['test_accounts', 'ttl_cache', 'demo_collection', 'chat_messages__ek0_testing', 'chat_configurations__ek0_testing']

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: False

✓ All collection management operations completed successfully
✓ Authentication successful

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: e_DemtjBytm1aUekviehfI2WaUZV4mncm5Vlp-T1EHvHIplOkJ0AUmJq6DP389iFJBKAT-yT9p6nxO7OM3X6Ew

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
✓ Inserted document with TTL: ezCDdpPpTlcSadsdXX35nYTMBD9sPEaLaEVJQeCvaDh0myHJhXISv_EO1hSWihQYZ22hYoyvKk8XDle0EaBS5A

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 2 record(s) via WebSocket

✓ WebSocket TTL example completed successfully
🚀 ekoDB Scripts Example (Python/HTTP)

✓ Authentication successful

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: YCuFvKl9PmIdiZf3a3lmqFhEAVXhhwBACQWQKcu645K_CYfLWH_vdhVN0HBFqjAFO5a2G8Zxe0eJ6D8bjMxTgg
📊 Found 15 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: qzIhx2z8KPU_uq7GlYShJpMq-01OWhqS8IJzqgdzExU3j6jOTPjANM-ZHLGZizzy5LgdEpxeF5R2GGvoxth2Ew
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: htqCVXTl6FZDRKGsSk3HPdnBMKTEIml-cGsFJNHHe1xPRgy4lhkXq46SJj_o6HUI4NNVQEBkQb16L-QuIA33TA
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {'avg_score': 60.0, 'status': 'active', 'count': 15, 'max_score': 100}
   {'max_score': 90, 'status': 'inactive', 'avg_score': 50.0, 'count': 15}

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
Created Alice: $1000 - ID: Kbtr4EYl4lcCayIae4-FXpzUFbqQM7cVcsKNZsnUKcCJu6ovah8an7RFmFy06xZ4fv_uTGhkNWCHQSZeyKwMsg
Created Bob: $500 - ID: ADcHR4foCP4GbEM6eM3HP9F806V1wtd7URZIXVPvYMY_fkHtHuPBA0IJ35_vYVZcaZztnbFi8saooJOwRY4-3g

=== Example 1: Begin Transaction ===
Transaction ID: f30ef923-5d2c-45f3-b2b3-67dc0e4d4044

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
New transaction: 93e73f4a-60a9-46fe-98e5-e2f208f46d50
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: jOWPTh5PucjFpDFPQOrlIGpoVPCZpjrgoRSGpXDxobdc0ZV4ZpVlxzD5unDzuA413svLAXJ9pOEcpayLNna81g
Created Bob: $500 - ID: oL1Hj6WOsX1NgWvHxtQY2OJRG1wh8jbtZrlElzB-izS4kupK1bvjYpLdL1wZ83kGThJKSmP08X-mLI4wPhHCVw

=== Example 1: Begin Transaction ===
Transaction ID: a6a1ace1-c5ae-4b8d-8512-6d52071d8353

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
Created Alice: $1000 - ID: oWAtZuddUh6uv5EeKWWDek9pvGCNsP7QnEBYn0d6ERwFXPzM8MeqJeOblLrha5IP59OBAje8YFHLj0s0qrSkaA
Created Bob: $500 - ID: qeFypJ-ixPBSXrrzQD4iGua8VyElFeWSUTMRQ8EaL3TliIBZYwvRhWvYYYux5C8YaQ2sdEd08aNUVw6ndsPjEw

=== Example 1: Begin Transaction ===
Transaction ID: d43c27eb-b14f-408a-bb8a-acfbd7c5f835

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
New transaction: e69b5523-de3a-40ff-a232-e98ea587b90a
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: jlbvhzMb77zAOgMC44nL1Vkeh94IFMtHjAcSfR3uGnhcxLN3Y2Acm8qkkB1JVjYsfHmE5QjTOrf0sGQ-i5q0-A
Created Bob: $500 - ID: gU0mEZ881UzHk7vNtLgp4-4ZdcSM5VTVuGw6uNiWbda9AUELTyKuJQJUJTEnsxqV56n1KVARhLWT2tkasRFyew

=== Example 1: Begin Transaction ===
Transaction ID: 27fff5be-2fc3-4ee9-bec7-d576071dedd3

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
Created Alice: $1000 - ID: sCGhSbWCd2I9GffgU0IS9i_Xc_XmAd4nYiXXVvXKCfTz3S9NAU4utDrH0Z-8-cEu82VLl2JgwIo1JPOmq012iA
Created Bob: $500 - ID: sVv3Q3MrEexMvOGQFbO_h3C9BS2NyShLTBYci_x1frdBYsQ2loZSwv4p0Eu3JY0w6eL8YdYnT5F0xRl421-IJw

=== Example 1: Begin Transaction ===
Transaction ID: 1956429e-0121-448e-8fbd-3aaa5069cb9b

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
New transaction: a9b95735-4a15-4487-ba65-ea8e5e6c1e5c
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: a2--e1cPo045ipm_s2gkx2mq2NIsZgmVBpaq8Knk7a1r48dwDBZE0lBkxB5mEjGwXe698yalAWQKZId6Cg9kFA
Created Bob: $500 - ID: JKFfToxnofCKcz7TCGdRmpZLbJ-fnBbBuLv8mY_Hm8NCY85I2Qw-YefxTg9FJrnbkIobI5Ari55okFxi1HAL4Q

=== Example 1: Begin Transaction ===
Transaction ID: 8404657b-b9b1-4ca2-9136-879df5b125e9

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
   ✅ Script saved: nkHMxbdLYxWNv-Guui9dLv9iH0k3A3r1LmfvSDVsBehYmnEMHKWHbjNix8FIvV6dUW3ycydfImkxRMKbnqNeEA

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: O4Pmk5bxfDj5q0u4EFAX3iOQrrZubv1LdEBkcUbXzAxZvjwrFaEbkQIHODzv7hR9R4XViPKP5MV4uyeSl9Rh-Q
   📋 Name: {'value': 'Alice Smith', 'type': 'String'}
   📋 Email: {'value': 'alice@example.com', 'type': 'String'}
   📋 Status: {'type': 'String', 'value': 'pending'}
   📋 Credits: {'type': 'Integer', 'value': 0}

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: v5oA58hLMT5ouEBJ-dcvdfp3EsumZTtQYb88MGymCoNSHicULnz9GOn5nIxdHAPa0BTPeyWCz6RCN9_hPHt-CA

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
   ✅ Script saved: pDu6Yx5PIweUO4t-wguCTwP2l2n7IW_KyGFtIZd2vmfuhbF_NZxnQwaYGQa29zWFtzMKc7_QlrOLrGBKAWJObA

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
   ✅ Script saved: 0mwtaymOSw84xX2FgVf5-HmCS2z1fIlFr6zmce0R0P-vYCXlzMLBB3bOcd4NPm6iRiloYLuqQKFTY6lp-09gPA

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: nkHMxbdLYxWNv-Guui9d...
   ✅ Deleted script: v5oA58hLMT5ouEBJ-dcv...
   ✅ Deleted script: pDu6Yx5PIweUO4t-wguC...
   ✅ Deleted script: 0mwtaymOSw84xX2FgVf5...
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
    Finished `release` profile [optimized] target(s) in 0.14s
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
Inserted: {'id': '9QeJnHEQTxTCqm5d6XaNaNMmzxcSbHYbLx5Wy0XNltHT39cmq4i2oVWzTpcQp7f8ZrOv5A786kkaskYre03Gaw'}

=== Find by ID ===
Found: {'active': {'value': True, 'type': 'Boolean'}, 'id': '9QeJnHEQTxTCqm5d6XaNaNMmzxcSbHYbLx5Wy0XNltHT39cmq4i2oVWzTpcQp7f8ZrOv5A786kkaskYre03Gaw', 'name': {'type': 'String', 'value': 'Test Record'}, 'value': {'value': 42, 'type': 'Integer'}}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'active': {'value': True, 'type': 'Boolean'}, 'value': {'type': 'Integer', 'value': 100}, 'name': {'type': 'String', 'value': 'Updated Record'}, 'id': '9QeJnHEQTxTCqm5d6XaNaNMmzxcSbHYbLx5Wy0XNltHT39cmq4i2oVWzTpcQp7f8ZrOv5A786kkaskYre03Gaw'}

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
Collection created with first record: "Bb11WR-gahHVOMU4-S4-VcpwCo77LeT-OrKp_bCpC6guOASu3Vg_9-WAFvy_DQl2j0zc_kZDvZ5PVFZWKR2krw"

=== List Collections ===
Total collections: 10
Sample collections: ['test_accounts', 'ttl_cache', 'chat_messages__ek0_testing', 'chat_configurations__ek0_testing', 'test_collection']

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: False

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: xt4qHAdhRhc0KVZDcFPsx_5ELMMbMxrXnnnrvaiRLMI5-3ao4zTJm_4mAVYzDY6g62ALltmTklmgK_RzuHrtsQ

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: G90mnhv72kP6RzbjOQbSwqn5MGFBVxAQyVKxv8iyAtNaFhaN4fgJ5WvRqLJmNIrnrX4UHRmShUMmnh8lkIuBdQ

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
✓ Inserted test record: E7QUkCbvo57MkAY1LGgvwbE_5sG8KUJ3hBFV3X1ihWQkMxAg715vKEECaDjpV4iF1lIkWcIBYzhMdDVoJiI93A

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
✓ Inserted document with TTL: -KNC8nAb7oIdSVT13U7nLq6XIaqaNcmss2zUDO3RxqYXUafQ2sAM2kapWMmw96D_DNyJ43IXTekZHYc6Yqk8Jg

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
  1. Score: 25.740, Matched: email, email.value, name.value, name
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: bio.value, bio, title, title.value
  2. Score: 26.400, Matched: bio.value, title.value, title, bio
  3. Score: 26.400, Matched: bio.value, title.value, title, bio
  4. Score: 26.400, Matched: title.value, bio, bio.value, title

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title, bio.value, bio, title.value
  2. Score: 39.600, Matched: title.value, bio, bio.value, title
  3. Score: 39.600, Matched: bio, bio.value, title.value, title
  4. Score: 39.600, Matched: title.value, bio.value, bio, title

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 2 results (matches: run, running, runs)
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio.value, bio

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.754
  2. Score: 0.752
  3. Score: 0.734

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.726, Matched: title.value, content.value, title, content
  2. Score: 1.128, Matched: content, title, content.value, title.value
  3. Score: 0.514, Matched: 

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
✓ Created session: escAHAP-FcRf2PLZuTgFmENQqrE6LEvMLOpY5U5tFaxBmw5IVW1Sydk9FL3pfilwUfCqQCyiCuC8Wv5vC3eE0Q

=== Sending Chat Message ===
Message ID: NsgHsVabDX_SbUCdOt9lOVE1fWm5mHbBQp4GWrU7vfloipjkLtpyiw3wA0owZmrNsLuNRWNux2vynMPQB8M-1Q

=== AI Response ===
The available products and their prices are as follows:

1. ekoDB Pro: This is an enterprise edition product with advanced features. The price is $299.
2. ekoDB Cloud: This is a fully managed cloud database service product. The price is $499.
3. ekoDB: This is a high-performance database product with AI capabilities. The price is $99.

Execution Time: 2716ms

=== Token Usage ===
Prompt tokens: 603
Completion tokens: 79
Total tokens: 682

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: cCXRhshO2K1u4oEgzhUn5CeenXcQlZ06uX7cjsO_4W1GoVAHNY_mU7IYMPGxynYUv3_wNHDa1sL0B9uJ7os78A

=== Sending Messages ===
✓ Message 1 sent
  Response: Answer: One product available is the ekoDB, a high-performance database product. It is priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: vI6cMWwoe1g0lBK1MZ3w3o3xtQR5hLWGlrw_GRHfokc5wvsowF2pHj_avRbQAw-Q1gehqIWmT2PCXcANa1RbEw
  Parent: cCXRhshO2K1u4oEgzhUn5CeenXcQlZ06uX7cjsO_4W1GoVAHNY_mU7IYMPGxynYUv3_wNHDa1sL0B9uJ7os78A

=== Listing Sessions ===
✓ Found 6 sessions
  Session 1: vI6cMWwoe1g0lBK1MZ3w3o3xtQR5hLWGlrw_GRHfokc5wvsowF2pHj_avRbQAw-Q1gehqIWmT2PCXcANa1RbEw (Untitled)
  Session 2: cCXRhshO2K1u4oEgzhUn5CeenXcQlZ06uX7cjsO_4W1GoVAHNY_mU7IYMPGxynYUv3_wNHDa1sL0B9uJ7os78A (Untitled)
  Session 3: escAHAP-FcRf2PLZuTgFmENQqrE6LEvMLOpY5U5tFaxBmw5IVW1Sydk9FL3pfilwUfCqQCyiCuC8Wv5vC3eE0Q (Untitled)
  Session 4: hyGmrj0Z3bTf09rh4aobr5bED81Zs2p4vAC3-HmLu1uJjUvxMG6N94KSZw2qnG7vBqdW60KHJcX7K5l_eD_Yag (Untitled)
  Session 5: LDJLO-5YuyhN72uoGjH8oZkLQWc_B7ARiZYwyG-8rDhRttuosxeeXgu_2eRIr0XKRfQH1xKZk8_QkxAiUoAEeA (Untitled)
  Session 6: OYjJR7pVbKoYxcBDhajVQoErv88QB3r08vuGOXPIKr3sG79g5agf2DVqZivkEABQRz95uBQ_hm8b9krr1yc3mw (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: vI6cMWwoe1g0lBK1MZ3w3o3xtQR5hLWGlrw_GRHfokc5wvsowF2pHj_avRbQAw-Q1gehqIWmT2PCXcANa1RbEw

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 7QzB6Q7jJbrFLVgZVX_G7lIWYdAJdkd0dgz9WkS4QjL2hgAifO1p4ah6mX1MXQdpSMLs54MECLalXe0iAIRxiQ

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information provided, the product available is the ekoDB. It is a high-performance database product priced at $99.

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['created_at', 'updated_at', 'token_usage', 'chat_id', 'role', 'content', 'id', 'context_snippets'])
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
✓ Created second session: MeoMxNtgzw5KEIQyZX_n66uvkZPV8Fq5Dj2SVWOUflZxg7wgntyjQc2cHezCDrKXgAXeh3-ZzT4XJZ1weC8g0g
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

✅ Script saved: luTE9VNeb_P9NbDXel4pV75yy2D527GiK9c55vOcG0N5CitUnYoXTxzsbBGIjiarYjp-OeMF5871WUd84G5jKw
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved: vOxiZvodE12oVANmlS8bv1HecEeLsU0e_PpPaIGlJUIS5XwVfYoXg0nEVD6loCVFxgzaIkImo151_mqw3OmCtQ
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: EhQAsMUZFgVqlzyju24Mzhr_EgzE2BevJRW3DtbsBb_dNlhKeY-_ICHKvqw_KmqKnMN5FYklEeYpkFvOUraisg
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

✅ Script saved: M5RevVC_xggyEBoewrwlmaw40xYJdoPnfbD1egT089J43jk4R7926S3njR6Lh4phFyViyJ_Jxpo8UUwE9_407Q
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
