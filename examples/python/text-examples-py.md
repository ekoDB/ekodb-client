make test-examples-python
🧪 [36mRunning Python examples (direct HTTP/WebSocket)...[0m
✓ Authentication successful

=== Insert Document ===
Inserted: {'id': 'dYkRme2ONpAMZ0V4rqmQWSiK1shYjM7UIfJZA3E5ZaAE6slxCyJFbaGQkaKXfhKYRT69VgBF3Z6UmwKp7MQfFw'}

=== Find by ID ===
Found: {'id': 'dYkRme2ONpAMZ0V4rqmQWSiK1shYjM7UIfJZA3E5ZaAE6slxCyJFbaGQkaKXfhKYRT69VgBF3Z6UmwKp7MQfFw', 'active': {'type': 'Boolean', 'value': True}, 'value': {'value': 42, 'type': 'Integer'}, 'name': {'value': 'Test Record', 'type': 'String'}}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'name': {'type': 'String', 'value': 'Updated Record'}, 'id': 'dYkRme2ONpAMZ0V4rqmQWSiK1shYjM7UIfJZA3E5ZaAE6slxCyJFbaGQkaKXfhKYRT69VgBF3Z6UmwKp7MQfFw', 'active': {'type': 'Boolean', 'value': True}, 'value': {'type': 'Integer', 'value': 100}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: IclmydnNxR1P-E0R7bBIuBKTF1Ka65Q1s32FLDl28hTOjoHAqNCeuCWCZsfDHQ4J5KOp9vNZEg0ayYtaSPXtQA

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
        "id": "IclmydnNxR1P-E0R7bBIuBKTF1Ka65Q1s32FLDl28hTOjoHAqNCeuCWCZsfDHQ4J5KOp9vNZEg0ayYtaSPXtQA",
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
        "id": "fGqA_U1JCaXbYlEiFASIABYGZ-Ckhe9tGhJXqSIoaSMrfQnRLfvqHABUQDbp2Vb2nk6WYTT1jQOIKE8UhS3SGQ",
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
cache:product:1: {'name': 'Product 1', 'price': 29.99}
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
Collection created with first record: kdUsjAoHTxnMFqefKTxIZTNeQQQ-87MQ6i1qbFF-n7_oNPlzJO2LLuAjrXTIHMOzqfl6bw-Qj7olG0zv2yUzog

=== List Collections ===
Total collections: 11
Sample collections: ['websocket_test', 'ttl_cache', 'scripts__ek0_testing', 'users', 'test_accounts']

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: False

✓ All collection management operations completed successfully
✓ Authentication successful

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: P0jcOruyuqVvOIrYqsdZxTGw79msBR0MPtKr0uu7j9TOKndWLWWFYi8e0c63g9UXhaiY1RxvREpt4i4VFHNsfg

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
✓ Inserted document with TTL: E9U_dgrjTub1oBfND5qI_tgUM0hfYcg_6Sf-bAFEOCdateAknF7_0g6E2fsk79pIrDawY1YeNXYMfVotle7NmA

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 2 record(s) via WebSocket

✓ WebSocket TTL example completed successfully
🚀 ekoDB Scripts Example (Python/HTTP)

✓ Authentication successful

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: keNtsEK2NmdyZD5JIq5L_V5xKnAAkiYjk7IQpqnJT6t_w8M7nyvTrBH0ZFKvtJvum61FvfAyHJaca5NjIhOIEg
📊 Found 30 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: Dag48tk9SacyZAMlo-jPspCCV7hjeIrf0kfr75KmqKCYyk2uxhPAgYuOqs0xiZYPThA6TMUbYt0_VL1tXuuUwg
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: PJc7HNHLc8_3E87VNbziiA7NZmZBFRxlr8Y8rpc85J1uLauUTIyIG1JH89-DmqMbXiZsZGUEwjVLZtO6ook4Og
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {'max_score': 90, 'count': 15, 'avg_score': 50.0, 'status': 'inactive'}
   {'avg_score': 60.0, 'status': 'active', 'count': 15, 'max_score': 100}

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
Created Alice: $1000 - ID: Ql-l5GqNqbAdQPFsMFSmrHXTQ7xrD-EMRyzIAu-VOR3A3Oepa4BkOIZ04Yi3QAmZbzrDPi_4lmSi2gDKFYJLMw
Created Bob: $500 - ID: SRd1UIDt1ZoR3ByH_5C2o1NWoR_Y_4LiGZC9pi_HupevAN4aPRUmtB2ujUtaQitdeOOBfUG3wUPao52LW2D3nw

=== Example 1: Begin Transaction ===
Transaction ID: b4ead9d8-a4b7-4a59-8edc-6a6dee920fe7

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
New transaction: 16ecbb3a-2658-4617-8371-815b05309215
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: ogpYjocuGnFhAj-VFLk6tQ_JcoVcRTl5M4QNEDyyO3JBWzA2f50xGUsrfscfZXJYWJcIdOCPMPsY673SHQaJ8w
Created Bob: $500 - ID: t21AE_svF9ADqTdIcQoq54WmzKYT5877WNqrYg08RtN05Ku2hMxWeXjJGRES88MsOzFD_OIh6m0a03sP27UDGg

=== Example 1: Begin Transaction ===
Transaction ID: 25ed54c6-730d-4951-a321-e764d5e4284e

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
Created Alice: $1000 - ID: hw53OjTENCv8YBSqKxLxo_Thh9bMe-Y5DbN7UEwuaIsm103S6w5z0UsO69Vx_ifoUGgPIvzkz_3oNif5h8sb5Q
Created Bob: $500 - ID: DpPKB2jwWpg8pHtooQ0EpYeLLGm-APN9VW-ixTvFDX0Iw4088kLmbgP724GZkgySTiXR62c9QZovxdyxF3B40Q

=== Example 1: Begin Transaction ===
Transaction ID: 677b95c9-0c01-4df8-8017-4d18b15bf191

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
New transaction: 6cacc338-4039-4bfd-a426-2d7fc8245e5d
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: CW1YK8w6bZt7fYVOCIDbjps9iMI8gXY9hFet-c94__WAAV3Uw_4Qjj53lY0DpEoysqQTqUt3Pxs60NUD15RC9g
Created Bob: $500 - ID: p6sBE4TGPArnxOa-RgGTTXxrmnkJ1VgKyIL8oP6GNcHRCQC3y66OkFUoVHVT7mTyjv4tZY59hshmTZVl68qm7Q

=== Example 1: Begin Transaction ===
Transaction ID: b3d462b7-1833-439a-8077-7d169a0f77c2

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
Created Alice: $1000 - ID: Ptd_cSzzcCk4YJOgjO05pK5IJceiXSPf9S_AMVZ7LGMhrrXqmV1TMdSlX1cE7_PAYp0luW6OmcrvMM3dcdX3UA
Created Bob: $500 - ID: HROi5yFep4rCONGqrpCkjzGe1JU9tc-UD_RkiOkpcOdifRE7Jno92AzIZbS6Wk22yR34e4M5-G4Ag1PZKfTIwQ

=== Example 1: Begin Transaction ===
Transaction ID: 96b2f3cc-cc22-4a4e-b8ed-3adc54a5da32

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
New transaction: 36beb015-5609-4350-9d15-b682cb8d08cc
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: Q1sSOKcI9si96e_WfhUsH33jV-seHZxpAqNiCdXmTfKBtlN2kELmwfFXPvbJWl_8YiKqm8C50UkVvVwOWo9Wcw
Created Bob: $500 - ID: h4qreTRaLMlBGG2FceVu5prblRHVPIvIlXZQAQ97Ma2X9oJ7xbG3JJKl0hp3R_ko66LdGrmzpGkicYu6KtVKsQ

=== Example 1: Begin Transaction ===
Transaction ID: 0b3c3103-6a44-4bdf-a68b-04208e871cdb

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
   ✅ Script saved: vW5o2ivvOBx6SEsxXcs-qXWCX-LHzm609O-fCKr8Qj-NuormlDuB_FJ8e4Rq1FlXmXHBANb2wejeD_XeyGx9hA

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: CU4eIM24HsufiWuQE6C3FVwhSHKveAUOm0XgA732Zs0Cdy1QtIIiReF_LfCRj2t-iFuy_o7i84OYLoTcCSwUDA
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}
   📋 Email: {'type': 'String', 'value': 'alice@example.com'}
   📋 Status: {'type': 'String', 'value': 'pending'}
   📋 Credits: {'value': 0, 'type': 'Integer'}

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: hSFFUBZAoQetK4TZWpLyidvcXN0jIMY6oOLrP2Qq85-MYAuxNmfC_Xc_khA9OhK3dHDs9sXO1hFfE4EU5v33Vg

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
   ✅ Script saved: dn87ZG9sZxDbV689tQFlPVNplHSAzCgEH-gwCuzNBfDguVdGEPX4CIGPfzhxIH0jmOya6viJx1DLoGexDzjXzA

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
   ✅ Script saved: YEO9mdYIDlsE1cjoOZTtEOJ8oHm8ZHzsOt8g0n3Mkgpjh3TpoSUqozplWUEPtosnXK6W7yTrN6urV2434zU7dw

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: vW5o2ivvOBx6SEsxXcs-...
   ✅ Deleted script: hSFFUBZAoQetK4TZWpLy...
   ✅ Deleted script: dn87ZG9sZxDbV689tQFl...
   ✅ Deleted script: YEO9mdYIDlsE1cjoOZTt...
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
    Finished `release` profile [optimized] target(s) in 0.10s
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
Inserted: {'id': 'WtiKbYriuwdqnryTPXlPxHIRnymN_f4E4n2RcmAGtno9RMRQ2WAKa_KdmrSS4dqfPg4c0hNVI20x5KqRVJDRfQ'}

=== Find by ID ===
Found: {'id': 'WtiKbYriuwdqnryTPXlPxHIRnymN_f4E4n2RcmAGtno9RMRQ2WAKa_KdmrSS4dqfPg4c0hNVI20x5KqRVJDRfQ', 'active': {'type': 'Boolean', 'value': True}, 'value': {'type': 'Integer', 'value': 42}, 'name': {'value': 'Test Record', 'type': 'String'}}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'name': {'type': 'String', 'value': 'Updated Record'}, 'id': 'WtiKbYriuwdqnryTPXlPxHIRnymN_f4E4n2RcmAGtno9RMRQ2WAKa_KdmrSS4dqfPg4c0hNVI20x5KqRVJDRfQ', 'active': {'value': True, 'type': 'Boolean'}, 'value': {'value': 100, 'type': 'Integer'}}

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
Collection created with first record: "CTx4--hpL99g3d6QG-LMq71MMmz33USLMcstKlmB__Szg3wfEUr9FpqEVrk6DEYM6CFP7an2IgjeeK9B0PX5Dw"

=== List Collections ===
Total collections: 10
Sample collections: ['websocket_test', 'ttl_cache', 'scripts__ek0_testing', 'test_accounts', 'test_collection']

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: False

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: AGxkCAS-cowEh6Bwbe3WI_GIyAQpwbn7s3AgRvIFSqxRZeFKo9BRitCFEYrwdAy_3y7mJ9Rl4O9qXvz49IEzbg

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: BRqcem6UFhssRxhstGZ2bYAoXhf31LBN0ENr8KAn_3lk9DjEr-Zh5AT-2-O_Z4vj2425-SCHFMJ0BbB9h89eHg

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
✓ Inserted test record: 2nUxZa88wwhd92pccaRwW7aEkHn8mfOVmUXD3wr8Bjc9L4U0cEgMbnEalW7gfO4G2hmHAJQPxEpvxoB1RHEOqw

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
✓ Inserted document with TTL: S8hgmmy1GH_ilpbv5MLEeZazP9tXmebliOQCGY6Tarz5pZOeBp8mJbFi9KZ2CDpucGdHDUVmSkrq8N_fo0wEfA

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
  1. Score: 25.740, Matched: email.value, name.value, name, email
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: bio.value, title, title.value, bio
  2. Score: 26.400, Matched: title.value, title, bio, bio.value
  3. Score: 26.400, Matched: bio, title, title.value, bio.value
  4. Score: 26.400, Matched: title, bio.value, title.value, bio

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title.value, bio, bio.value, title
  2. Score: 39.600, Matched: bio.value, bio, title, title.value
  3. Score: 39.600, Matched: bio, bio.value, title, title.value
  4. Score: 39.600, Matched: bio.value, title.value, bio, title

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
  1. Score: 0.732
  2. Score: 0.728
  3. Score: 0.723

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.706, Matched: content.value, title, title.value, content
  2. Score: 1.110, Matched: content, title.value, content.value, title
  3. Score: 0.513, Matched: 

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
  - Laptop ($1200) by Alice Johnson
  - Mouse ($25) by Alice Johnson

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
✓ Created session: JehJVzy0fGzjb5QdDQEKy5wozk9uYujxIkjq0V5YTjSRI0ad3j65tSRgHdY1djrSCKaYCyqxoZ0QuX97tozN6Q

=== Sending Chat Message ===
Message ID: F_v60-_9ZxrmR9YOoenAE0TVyklJByAkcuFne3BpAf5kLS_-yGBHHReqrfq88LkQXZkIrGeJOAXf6BQ83oeSAw

=== AI Response ===
We have three products available:

1. ekoDB Pro: This is an enterprise edition product with advanced features. The price for this product is $299.

2. ekoDB: This is a high-performance database product with AI capabilities. The price for this product is $99.

3. ekoDB Cloud: This is a fully managed cloud database service product. The price for this product is $499.

Execution Time: 2876ms

=== Token Usage ===
Prompt tokens: 608
Completion tokens: 84
Total tokens: 692

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: fluLxb7w4wPDUOdSIOtQjVhdiR09WlaNCjyUoqemlxqDIa8uzcpRQBBHqtePk3jKPnZbmg_r0mkJp5Gqo6XIbw

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information I have, one product available is ekoDB, a high-performance database product. It's priced at 99 units.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: VavaZWaynMzXrFwMAuN0JYWgFdRWfVT769hoCsK6xWG5F5-VM7Cv9BkJ_Bw6_1ZTv69O5gcWR2EVjuy4Im-h7g
  Parent: fluLxb7w4wPDUOdSIOtQjVhdiR09WlaNCjyUoqemlxqDIa8uzcpRQBBHqtePk3jKPnZbmg_r0mkJp5Gqo6XIbw

=== Listing Sessions ===
✓ Found 6 sessions
  Session 1: VavaZWaynMzXrFwMAuN0JYWgFdRWfVT769hoCsK6xWG5F5-VM7Cv9BkJ_Bw6_1ZTv69O5gcWR2EVjuy4Im-h7g (Untitled)
  Session 2: fluLxb7w4wPDUOdSIOtQjVhdiR09WlaNCjyUoqemlxqDIa8uzcpRQBBHqtePk3jKPnZbmg_r0mkJp5Gqo6XIbw (Untitled)
  Session 3: JehJVzy0fGzjb5QdDQEKy5wozk9uYujxIkjq0V5YTjSRI0ad3j65tSRgHdY1djrSCKaYCyqxoZ0QuX97tozN6Q (Untitled)
  Session 4: FCexZoTUlOX6KTMmjH8vgzqs3v-JG_bq367FxD8kJTuYbPFS0SQ--AfP1uiO3tCjYL3IFjfW2XJv984i2JMbig (Untitled)
  Session 5: JI9ALeP9q7XIpzqiQkD8s_hsGsDcCR2eUhCkRH_5XNItOPrTyc0CeMrkI485fLp7sANlcLMTZeGqjhUlhuyHUw (Untitled)
  Session 6: HWO_Ih0OA303BqPhCgWr6vgXp3n0ydpNFOkFouLtHWO6i67yp0lhJOAGkd3BR00nWokmyL7toIpPhMwWY0WFsA (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: VavaZWaynMzXrFwMAuN0JYWgFdRWfVT769hoCsK6xWG5F5-VM7Cv9BkJ_Bw6_1ZTv69O5gcWR2EVjuy4Im-h7g

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: dPNKEv_dfqJL8x3sqR3r2tpGbSp43UYC7alf9KFGbpMnykDYjj3QbyVu8N4qZRHMEeI_2znmmGZjwj7e8oQcyg

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information available, the product "ekoDB" is available. It is a high-performance database product and it is priced at $99.

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['updated_at', 'created_at', 'chat_id', 'context_snippets', 'token_usage', 'id', 'role', 'content'])
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
✓ Created second session: w3uwW8-60AB64JP2kQHBpGnrWGOXfXp289TQ8dj3GhYDE9QI6-wPz9Ctj-OMc9v_ttISNovA-FnkX8mqAqCyCA
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

✅ Script saved: RO4ee1NjBSkvQ9zm4b4RmQTKkTukNqwk7bF1K_Sn17dCfBbBaPEzwizIUCCMNSI1SjTPRUOO9YthRYQvi9z4iQ
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved: 97iyx4o_mg9KNrI-_D9vAQR5cGE_Bhi-j0g9yBlVerx_ZbBQ6Qs2wqKhmjgCWPU5HQNFKllPD2AXRFZ4JG9ZLQ
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: 2Q0sGYYFqUHJ_JAwK3klk-JwvT3_hGkBu7WzufwEYbRnf23fdH2C4slINQGae1p9zIvClxBNGI_l4DXw8MKbTQ
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

✅ Script saved: ilqfGx2BoJsjcxFfmC4BcuEaQgYfxdeXHpPSPuojRZOrWuwM8E2EALe_bELPytqsjAq5-OQspWL8EUaSDStMxQ
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
✅ [32mPython client examples complete![0m
