make test-examples-python
📦 Ensuring Python example dependencies in .venv...
🧪 Running Python examples (direct HTTP/WebSocket)...
=== Simple CRUD Operations (Direct HTTP) ===

✓ Authentication successful

=== Insert Document ===
Inserted: {'id': 'M8b_EQ6FX89176ofS_mxbUMnHKr7xk6cq3ysDYwaGxDz-m-jZib8wW77I7W7j5-OVSu-pOJbpkqUi3lpvIX9Sg'}

=== Find by ID ===
Found: {'name': {'type': 'String', 'value': 'Test Record'}, 'value': {'type': 'Integer', 'value': 42}, 'active': {'value': True, 'type': 'Boolean'}, 'id': 'M8b_EQ6FX89176ofS_mxbUMnHKr7xk6cq3ysDYwaGxDz-m-jZib8wW77I7W7j5-OVSu-pOJbpkqUi3lpvIX9Sg'}

=== Find with Query ===
Found documents: [{'value': {'type': 'Integer', 'value': 42}, 'name': {'value': 'Test Record', 'type': 'String'}, 'active': {'type': 'Boolean', 'value': True}, 'id': 'M8b_EQ6FX89176ofS_mxbUMnHKr7xk6cq3ysDYwaGxDz-m-jZib8wW77I7W7j5-OVSu-pOJbpkqUi3lpvIX9Sg'}]

=== Update Document ===
Updated: {'id': 'M8b_EQ6FX89176ofS_mxbUMnHKr7xk6cq3ysDYwaGxDz-m-jZib8wW77I7W7j5-OVSu-pOJbpkqUi3lpvIX9Sg', 'value': {'value': 100, 'type': 'Integer'}, 'name': {'type': 'String', 'value': 'Updated Record'}, 'active': {'value': True, 'type': 'Boolean'}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: i-BoHLxUCpfQhQWQxwsyD402xlYPS6LoD6X_oYCMV_fkoL_K-fB3R6hX9PVSkXNS9gZ3sIGGNLchn2jZnUkZ7A

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "type": "Success",
  "payload": {
    "data": [
      {
        "value": {
          "value": 42,
          "type": "Integer"
        },
        "name": {
          "type": "String",
          "value": "WebSocket Test Record"
        },
        "id": "i-BoHLxUCpfQhQWQxwsyD402xlYPS6LoD6X_oYCMV_fkoL_K-fB3R6hX9PVSkXNS9gZ3sIGGNLchn2jZnUkZ7A",
        "active": {
          "type": "Boolean",
          "value": true
        }
      }
    ]
  },
  "messageId": "2148508262"
}
✓ Retrieved 1 record via WebSocket

✓ WebSocket example completed successfully
🚀 ekoDB Functions Example (Python/HTTP)

✓ Authentication successful

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Function saved: g4nhW7FBtFM4pg-a3xN6gKdooKICB428mD0DqMGT2pyNQiuQHjECybkaN8tUhyx8ckc4YZuHjZIlQIXlserdbQ
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Function saved: 6T-DDGypM0BT7MnDl192wuPGADbMbRUy1YkoGRPUUisZRIO1Cc_mLhhU_-UUIrJ6pUR1SJGPZFv-8SOGUydeTw
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: uQYYJgx5LwITResE59P7zlFowoQb0zupmIza0iRpOnMPUf70G6tGppq04-ABOJES9qo2M7YBgXcS52w9gT9xHA
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {'max_score': {'type': 'Integer', 'value': 90}, 'status': {'value': 'inactive', 'type': 'String'}, 'avg_score': {'value': 50.0, 'type': 'Float'}, 'count': {'type': 'Integer', 'value': 5}}
   {'count': {'type': 'Integer', 'value': 5}, 'avg_score': {'value': 60.0, 'type': 'Float'}, 'max_score': {'value': 100, 'type': 'Integer'}, 'status': {'type': 'String', 'value': 'active'}}

📝 Example 4: Function Management

📋 Total scripts: 3
🔍 Retrieved script: Get Active Users
✏️  Function updated
🗑️  Function deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

✅ All examples completed!
=== Batch Operations (Direct HTTP) ===

✓ Authentication successful

=== Batch Insert ===
✓ Batch inserted 5 records
✓ Verified: Found 5 total records in collection

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
Retrieved value: {'type': 'Object', 'value': {'userId': 123, 'username': 'john_doe'}}

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
kv_operations:direct:py:cache:product:1: {'type': 'Object', 'value': {'price': 29.99, 'name': 'Product 1'}}
kv_operations:direct:py:cache:product:2: {'value': {'name': 'Product 2', 'price': 39.989999999999995}, 'type': 'Object'}
kv_operations:direct:py:cache:product:3: {'type': 'Object', 'value': {'price': 49.989999999999995, 'name': 'Product 3'}}

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
=== Collection Management (Direct HTTP) ===

✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: cas8-8_Gj5EDfZrpmX0BsMPCZg2ZDuh_2YCg5S4SySyEr9qebg6znlmLpQ9opHPIsKW2XCnoDlSLPSc2I6Gxag

=== List Collections ===
Total collections: 13
Sample collections: ['chat_goals__ek0_testing', 'chat_agent_configs__ek0_testing', 'chat_tasks__ek0_testing', 'chat_goal_templates__ek0_testing', 'chat_configurations__ek0_testing']

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
Created Alice: $1000 - ID: hFLG3ML0dVZ5sQUM76xnA_cT6akSrIy1SpmXTvcvjpej68OOhBfCx1-mmR2txyCjGhMV4Lhd6snkXUswobC6jg
Created Bob: $500 - ID: WcRJm_3eNUlcMXZstHGfRzXnehfwaCqn2nuGwZAU6nxOXMGG7qW-H6DEh_gvkupViW-hzKYu9nJ754aITjCeyw

=== Example 1: Begin Transaction ===
Transaction ID: 55e00123-fcbf-4a39-9478-66f66b408569

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: ${'type': 'Integer', 'value': 800}
Bob: ${'type': 'Integer', 'value': 700}

=== Example 5: Rollback ===
New transaction: 2db4ca1d-df85-4be8-ad47-c4c23e838fc8
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'type': 'Integer', 'value': 700}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: e6vTqGKEXR9j2chZ1ykCa-8aYD1UZx00QhBAlzxZqSlqUMacQVz6Oj61zqPGwWud5qadPoltvaiWhJl1HjoXfg
Created Bob: $500 - ID: eSOSlyssNoOtJUWn2v6s9KQmO1Xvqd5c5uVzyUSjN4cr9K2U4oPxTs67KpbJ_YtnzvQ3ZiGrjlkg-Src0aUQtQ

=== Example 1: Begin Transaction ===
Transaction ID: 14771edf-017c-4666-9388-db6c4129faf4

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: ${'type': 'Integer', 'value': 800}
Bob: ${'value': 700, 'type': 'Integer'}

=== Cleanup ===
✓ Deleted test accounts


🔄 Switching to NON-DURABLE mode...
   ✓ Config updated: durable_operations=false


============================================================
🔥 TEST 2: Non-Durable Mode (durable=false)
============================================================

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: L8p8h7Qs2Z7YSWUGYewY-1Frgi_FWLxNhls1-U_GEPMHZwmM6a9NElfuFqp7wcQITxwgQ2Q80aFI0S4x__HTeA
Created Bob: $500 - ID: SdG5SJgBxA03QV-eeI3ZhbYdFGbc-x9vdQvryDoY_4zpbA-Zdn87AKH_UANPkrufQeEw0_aNYNQBdXWzvAqqfA

=== Example 1: Begin Transaction ===
Transaction ID: 34d464a6-7f3d-467b-9e40-ce1adc7f574f

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: ${'type': 'Integer', 'value': 800}
Bob: ${'type': 'Integer', 'value': 700}

=== Example 5: Rollback ===
New transaction: 7238079c-dbe4-4990-9d30-4a18bf7b7bac
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'type': 'Integer', 'value': 700}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: MzuQ0URf50htUnk_KQZ3ZvgoSaYOl5Dac15XjQYIIkJwSbI4AFCv8VEjM5CsSZxQb7SwK4EIEIZa_XCFgqLDTw
Created Bob: $500 - ID: 3nIhP_DJ2KYPorTHNPMPbc7bfbCDYW3AV2l-bURPr9LIVE2SgBAKd_Z7M3pOMqPdbkNVre8VKadc39xDhSy5nw

=== Example 1: Begin Transaction ===
Transaction ID: 65785a0d-5d1a-4fd3-9b55-ec5d628b0cae

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: ${'type': 'Integer', 'value': 800}
Bob: ${'type': 'Integer', 'value': 700}

=== Cleanup ===
✓ Deleted test accounts


🔄 Switching to DURABLE mode...
   ✓ Config updated: durable_operations=true


============================================================
🔥 TEST 3: Durable Mode (durable=true)
============================================================

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 1KkLRtKP3oAsM-z69dvP-qz2Y8Tl0AnFvVpO1UqdCyceO97Qi9pyUSOyQ8o0h7BeXVnAvIQgPpNcDbyR4FWNQQ
Created Bob: $500 - ID: lYPtjV8PyGMIjUOrZZguI8o_GFmBNrtd304LYJ04C5_qUL7mxJQlYCmVuCDGfJKjgPdozBCST3hsYYGdIUWwQw

=== Example 1: Begin Transaction ===
Transaction ID: 9d08783c-e9de-477d-b301-dac3ef77db38

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: ${'type': 'Integer', 'value': 800}
Bob: ${'value': 700, 'type': 'Integer'}

=== Example 5: Rollback ===
New transaction: 19318aef-6516-4eb3-9cd1-aa6f7aa0f128
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'type': 'Integer', 'value': 700}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: K4XeyRBVS8LjZSmlQlO4TEtu8I5vMvexSklqkXoGytjX-uJB9tPtM5a3dQJ5vvy9fpFU_S7rz66kT1DP_k4Uug
Created Bob: $500 - ID: vqkS3gPegHP9z66_y1GPxMbPRXVotWKEeEsWsj--bgbnrjNyvXE0mkNHzZlMZrb7MbXclX41ZLTPGBrzHkzZLw

=== Example 1: Begin Transaction ===
Transaction ID: 5ccf2939-7ede-48c2-804b-8cc418f04d77

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: ${'value': 800, 'type': 'Integer'}
Bob: ${'value': 700, 'type': 'Integer'}

=== Cleanup ===
✓ Deleted test accounts


🔄 Restoring original configuration...
   ✓ Config restored: durable_operations=True


============================================================
✅ ALL TESTS PASSED - Transactions successful
============================================================

🚀 ekoDB Complete CRUD Functions Example
============================================================
Demonstrates:
  • Insert + Verify (using Query)
  • Query + Update Status + Verify
  • Query + Update Credits + Verify
  • Query Before Delete + Delete + Verify Gone

Each function shows Functions chaining with proper verification
============================================================

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted collection: crud_functions_users_py
============================================================
📝 function 1: Insert + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: oBMDdBfuJqICiV3vxfk1fvTqpmgX_fbHLuc6udqpihAb8ZemEfTPK_IlhfxMWNNbiDrqISlniEZkkBLW7KKZaA

2️⃣ Calling function (Insert + Verify)...
   ✅ function executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: d_SYEFDmJbIf9MuPPHKkXOzZxbcb8fIL722Z_dgUPPfmFQiSJG9rSFJwaqw9POQWhYcssfbOHHpAEVgqGVE8Jw
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}
   📋 Email: {'type': 'String', 'value': 'alice@example.com'}
   📋 Status: {'type': 'String', 'value': 'pending'}
   📋 Credits: {'value': 0, 'type': 'Integer'}

============================================================
📝 function 2: Query + Update + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: jPJsox36HpdJeLvEXqCcf_SH9f9nenE_EeAk6nvJ-RmdCRGbVxgnnSZ7ZG_LhVkYWLswO9Ae2NYhyzJjOSWHgQ

2️⃣ Calling function (Query + Update + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Status updated to: {'value': 'active', 'type': 'String'}
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}

============================================================
📝 function 3: Query + Update Credits + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: KuioFKuRoua0RRyaWzW2DXSlmt_6pWHLOAfHECZUv2LjMgx5nDqvHtFftWdQkyVmINAiRCC_cSWZj6dvbBqjlw

2️⃣ Calling function (Query + Update Credits + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {'type': 'Integer', 'value': 100}
   📋 Status: {'type': 'String', 'value': 'active'}
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}

============================================================
📝 function 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: sAX4_6JcBgt0b7F_PkXboABqyHKhVHgEgofAM86pcnepOTmBlJA2Rj6kOsN-se_W7jnujF1QTUjwkjct79_kvA

2️⃣ Calling function (Query + Delete + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: oBMDdBfuJqICiV3vxfk1...
   ✅ Deleted script: jPJsox36HpdJeLvEXqCc...
   ✅ Deleted script: KuioFKuRoua0RRyaWzW2...
   ✅ Deleted script: sAX4_6JcBgt0b7F_PkXb...
   ✅ Deleted collection: crud_functions_users_py

============================================================
✅ Complete CRUD Functions Example Finished!
============================================================

💡 Key Takeaways:
   ✅ Functions chain Functions together
   ✅ Each function demonstrates operation + verification
   ✅ Parameters make functions reusable
   ✅ Verification is built into the function itself
   ✅ Complete CRUD lifecycle in 4 focused functions
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
  Output: Document ID = 5p3RNcUXW3VA7MX17buz_cB45WKfe4anu4yf7TDsIHwpCjRMou-dZ2s8-1PLavZmgNcL1f_0p34wrczTbko02g
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: find_by_id(5p3RNcUXW3VA7MX17buz_cB45WKfe4anu4yf7TDsIHwpCjRMou-dZ2s8-1PLavZmgNcL1f_0p34wrczTbko02g)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: find_by_id(5p3RNcUXW3VA7MX17buz_cB45WKfe4anu4yf7TDsIHwpCjRMou-dZ2s8-1PLavZmgNcL1f_0p34wrczTbko02g)
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
  Output: Document ID = SH3kuHCzszyUZLcFeKVGf7RuxfHPVnEiGhM4N4z952LWKjuGh_ZkwAFrWdBBl10bN6RQU8N1RXX4bakE4hP1Kg
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: find_by_id(SH3kuHCzszyUZLcFeKVGf7RuxfHPVnEiGhM4N4z952LWKjuGh_ZkwAFrWdBBl10bN6RQU8N1RXX4bakE4hP1Kg)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: find_by_id(SH3kuHCzszyUZLcFeKVGf7RuxfHPVnEiGhM4N4z952LWKjuGh_ZkwAFrWdBBl10bN6RQU8N1RXX4bakE4hP1Kg)
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

╔════════════════════════════════════════╗
║   ekoDB Python Examples Test Suite    ║
╚════════════════════════════════════════╝

=== Checking Server Connection ===
✓ Server is ready

=== Getting Authentication Token ===
✓ Authentication successful

=== Running 10 Examples ===

=== Running simple_crud.py ===
✓ simple_crud.py completed successfully

=== Running simple_websocket.py ===
✓ simple_websocket.py completed successfully

=== Running http_functions.py ===
✓ http_functions.py completed successfully

=== Running batch_operations.py ===
✓ batch_operations.py completed successfully

=== Running kv_operations.py ===
✓ kv_operations.py completed successfully

=== Running collection_management.py ===
✓ collection_management.py completed successfully

=== Running transactions.py ===
✓ transactions.py completed successfully

=== Running crud_functions.py ===
✓ crud_functions.py completed successfully

=== Running document_ttl.py ===
✓ document_ttl.py completed successfully

=== Running websocket_ttl.py ===
✓ websocket_ttl.py completed successfully

╔════════════════════════════════════════╗
║           Test Summary                 ║
╚════════════════════════════════════════╝
Total: 10
Passed: 10
Failed: 0
✅ Python direct examples complete!
🐍 Building Python client package...
🔧 Ensuring maturin is available in .venv...
🔨 Building wheel...
🍹 Building a mixed python/rust project
🐍 Found CPython 3.11 at /Library/Frameworks/Python.framework/Versions/3.11/bin/python3
🔗 Found pyo3 bindings with abi3-py3.8 support
💻 Using `MACOSX_DEPLOYMENT_TARGET=11.0` for aarch64-apple-darwin by default
    Finished `release` profile [optimized] target(s) in 0.16s
📦 Built wheel for abi3 Python ≥ 3.8 to ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.30.1-cp38-abi3-macosx_11_0_arm64.whl
📦 Installing Python wheel into .venv...
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.30.1-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.30.1
    Uninstalling ekodb_client-0.30.1:
      Successfully uninstalled ekodb_client-0.30.1
Successfully installed ekodb-client-0.30.1
🧪 Ensuring test dependencies (pytest) in .venv...
✅ Python client package built and installed!
Admin permissions read successfully
=== ekoDB Advanced CRUD Example (Python) ===

--- insert ---
Inserted: E83KP_UpmvMNP6dxSvfCsOuvqjHIyOfB25SY6J_YRpBv0aW6QecVna1DTdFMWokuDgbDgJK_0nJ7wpHLiqSijw

--- update_with_action (increment) ---
After increment count by 5: {'id': 'E83KP_UpmvMNP6dxSvfCsOuvqjHIyOfB25SY6J_YRpBv0aW6QecVna1DTdFMWokuDgbDgJK_0nJ7wpHLiqSijw', 'tags': {'type': 'Array', 'value': ['initial']}, 'score': {'type': 'Float', 'value': 50.0}, 'name': {'type': 'String', 'value': 'Counter Record'}, 'count': {'type': 'Integer', 'value': 15}}

--- update_with_action (decrement) ---
After decrement score by 10: {'tags': {'value': ['initial'], 'type': 'Array'}, 'name': {'type': 'String', 'value': 'Counter Record'}, 'count': {'value': 15, 'type': 'Integer'}, 'score': {'value': 40.0, 'type': 'Float'}, 'id': 'E83KP_UpmvMNP6dxSvfCsOuvqjHIyOfB25SY6J_YRpBv0aW6QecVna1DTdFMWokuDgbDgJK_0nJ7wpHLiqSijw'}

--- update_with_action_sequence ---
After action sequence: {'count': {'type': 'Integer', 'value': 115}, 'id': 'E83KP_UpmvMNP6dxSvfCsOuvqjHIyOfB25SY6J_YRpBv0aW6QecVna1DTdFMWokuDgbDgJK_0nJ7wpHLiqSijw', 'tags': {'value': ['initial', 'sequenced'], 'type': 'Array'}, 'name': {'type': 'String', 'value': 'Counter Record'}, 'score': {'value': 40.0, 'type': 'Float'}}

--- cleanup ---
Cleaned up collection

=== Advanced CRUD example completed ===
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
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: DaaaUYUQrr_Dmp7pAgKk-Cnn-B8SPjZ7FGcAx7iEPZeVTmXOLwT9QPLK_eSML8LTMQwZbkBRkFRiorzGZX4g8A

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** High-performance database product
- **Price:** $99

If you need more information or have further questions, feel free to ask!

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['token_usage', 'llm_model', 'id', 'chat_id', 'role', 'content', 'context_snippets', 'created_at', 'updated_at', 'llm_provider'])
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
✓ Created second session: YWlPPRDDWQFRTx1WvcBGGCO9ReTO7xHtJbIitEUiufYJzXNjp7TK1IsHYGKF55Nx5UD8VWii10_Yve0kSIQoKQ
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 7

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 6

=== Cleanup ===
✓ Deleted chat session: YWlPPRDDWQFRTx1WvcBGGCO9ReTO7xHtJbIitEUiufYJzXNjp7TK1IsHYGKF55Nx5UD8VWii10_Yve0kSIQoKQ
✓ Deleted chat session: DaaaUYUQrr_Dmp7pAgKk-Cnn-B8SPjZ7FGcAx7iEPZeVTmXOLwT9QPLK_eSML8LTMQwZbkBRkFRiorzGZX4g8A
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: A82B4kyPNQVbF7gR1OouhZAUOMRA6mYGLFwNMTvQjXetJcm_gcEdFGa4-xZmXei1iTgJ88Ut7O4mQjqOAnSPjQ

=== Sending Chat Message ===
Message ID: Wzo084xLVg4PsAo8OFWd45LYnjgFfIMiYKVd0TMRFI2DY7YJO_NHhiO-UaN-ad6xB85cyk2Qy-TovfbP8xz8VQ

=== AI Response ===
Here are the products available along with their prices:

1. **ekoDB Pro**
   - Description: Enterprise edition product with advanced features
   - Price: $299

2. **ekoDB Cloud**
   - Description: Fully managed cloud database service product
   - Price: $499

3. **ekoDB**
   - Description: A high-performance database product with AI capabilities
   - Price: $99

Execution Time: 7597ms

=== Token Usage ===
Prompt tokens: 3413
Completion tokens: 91
Total tokens: 3504

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Message Stream (SSE) Example (Python) ===

Created session: MprTxuUL0YCp_S9E-upT826FetfIWhXJuK00zBvHxWYPT0qFW0acOkDLB1zYKuVk7nIHFOoH3-J7a2z9x1NG3w

Streaming response for: 'What is ekoDB?'

{"__progress":"received"}{"__progress":"generating","model":"gpt-4o-mini","provider":"openai"}EkoDB is a high-performance, open-source database management system designed for handling large amounts of data and providing efficient querying capabilities. It is particularly focused on supporting key-value and document data models, making it suitable for a variety of applications that require fast access to data.

Key features of ekoDB may include:

1. **Scalability**: EkoDB is designed to scale horizontally, allowing it to handle increased loads by distributing data across multiple nodes.

2. **Performance**: The system emphasizes high performance for both read and write operations, often incorporating advanced indexing and caching strategies.

3. **Flexibility**: As it supports different data models, developers have the flexibility to work with data in ways that are most suitable for their applications.

4. **Open Source**: Being open source means that users can freely access, modify, and use the software, which can facilitate community contributions and enhancements.

5. **Data Durability and Reliability**: EkoDB likely incorporates mechanisms for data integrity and durability, ensuring that data is safely stored and can be recovered in case of failure.

6. **Ease of Use**: The system is designed with usability in mind, often providing libraries or interfaces that make it easier for developers to integrate ekoDB into their applications.

It's important to check the official documentation or the project’s GitHub repository for the most up-to-date information, features, and specific use cases for ekoDB, as the database landscape can evolve rapidly.

--- Stream complete ---
Message ID: EwRdwT-Hq-UyjH1ZnijLRf-Nh9M4RIIyL1hYp9AspnUrYvZpTN4dSel8OX-2RV2FUDLWcdUXyp72lOYLOKiwlA
Execution time: 3000ms
Context window: 128000 tokens

✓ Chat message stream example completed
✓ Client created

=== Get All Chat Models ===
OpenAI models: ['gpt-4-turbo-2024-04-09', 'gpt-realtime-2.1-mini', 'gpt-realtime-2025-08-28', 'gpt-4', 'tts-1-hd', 'gpt-realtime-1.5', 'gpt-realtime', 'gpt-5.2-pro', 'gpt-5-chat-latest', 'gpt-3.5-turbo-0125', 'gpt-5.4-2026-03-05', 'gpt-5-nano', 'gpt-5.4-mini', 'o1-pro', 'gpt-5.1-codex', 'tts-1', 'gpt-5-mini', 'gpt-5-mini-2025-08-07', 'omni-moderation-2024-09-26', 'gpt-live-transcribe', 'gpt-5.6-luna', 'omni-moderation-latest', 'gpt-5-2025-08-07', 'o4-mini-2025-04-16', 'gpt-4o-2024-11-20', 'gpt-5.1', 'whisper-1', 'gpt-5-pro', 'gpt-realtime-whisper', 'gpt-5.5-pro', 'gpt-audio-2025-08-28', 'gpt-4o-mini-transcribe-2025-03-20', 'gpt-realtime-2', 'gpt-5-search-api', 'gpt-5.5-2026-04-23', 'chat-latest', 'gpt-realtime-translate', 'gpt-5.4-pro-2026-03-05', 'gpt-5.4-mini-2026-03-17', 'babbage-002', 'gpt-4o-2024-05-13', 'text-embedding-3-large', 'gpt-4o-mini-transcribe-2025-12-15', 'tts-1-1106', 'gpt-5.5-pro-2026-04-23', 'gpt-audio-mini-2025-10-06', 'gpt-5.2-pro-2025-12-11', 'gpt-4-turbo', 'gpt-4o-2024-08-06', 'o3', 'gpt-4o-mini-search-preview-2025-03-11', 'o3-mini-2025-01-31', 'gpt-audio-1.5', 'gpt-image-1.5', 'gpt-5-pro-2025-10-06', 'gpt-5.6-sol', 'gpt-4.1-nano', 'gpt-image-2', 'gpt-3.5-turbo-16k', 'text-embedding-ada-002', 'gpt-realtime-2.1', 'gpt-4.1', 'gpt-5.5', 'gpt-4o-transcribe-diarize', 'gpt-5-codex', 'gpt-4-0613', 'gpt-5.1-2025-11-13', 'gpt-4o-search-preview', 'gpt-4.1-mini', 'gpt-4o-mini', 'gpt-5.4-pro', 'gpt-4o-mini-tts', 'gpt-realtime-mini-2025-12-15', 'gpt-5.1-codex-max', 'gpt-realtime-mini', 'gpt-image-1-mini', 'gpt-5.6-terra', 'gpt-3.5-turbo-instruct-0914', 'o1-2024-12-17', 'sora-2', 'tts-1-hd-1106', 'gpt-4o-mini-tts-2025-12-15', 'gpt-image-1', 'gpt-audio', 'gpt-5.3-codex', 'gpt-5-search-api-2025-10-14', 'gpt-5', 'gpt-4o-mini-transcribe', 'gpt-image-2-2026-04-21', 'gpt-5.2-chat-latest', 'sora-2-pro', 'gpt-3.5-turbo-instruct', 'gpt-transcribe', 'gpt-4.1-nano-2025-04-14', 'o4-mini-deep-research', 'gpt-5.4', 'gpt-4o-search-preview-2025-03-11', 'text-embedding-3-small', 'gpt-5.1-chat-latest', 'gpt-4o-mini-2024-07-18', 'gpt-4.1-2025-04-14', 'gpt-5.3-chat-latest', 'o3-2025-04-16', 'gpt-5.2-2025-12-11', 'chatgpt-image-latest', 'gpt-3.5-turbo', 'gpt-5.2-codex', 'gpt-4o', 'davinci-002', 'gpt-5.4-nano', 'o3-mini', 'o4-mini-deep-research-2025-06-26', 'gpt-audio-mini-2025-12-15', 'o1-pro-2025-03-19', 'gpt-5-nano-2025-08-07', 'gpt-4o-transcribe', 'o4-mini', 'gpt-5.1-codex-mini', 'gpt-3.5-turbo-1106', 'gpt-4o-mini-tts-2025-03-20', 'gpt-5.2', 'gpt-5.4-nano-2026-03-17', 'o1', 'gpt-image-2.5-sunburst-2026-09-08', 'gpt-6-luna', 'gpt-audio-mini', 'gpt-4.1-mini-2025-04-14', 'gpt-image-2.5-flare', 'gpt-4o-mini-search-preview', 'gpt-6-sol', 'gpt-6-astra', 'gpt-6.1-sol', 'gpt-live-1', 'gpt-image-2.5-sunburst', 'gpt-image-2.5-flare-2026-09-08']
Anthropic models: ['claude-haiku-5-5', 'claude-sonnet-5-5', 'claude-opus-5-5', 'claude-fable-5-1', 'claude-opus-5', 'claude-sonnet-5', 'claude-fable-5', 'claude-opus-4-8', 'claude-opus-4-7', 'claude-sonnet-4-6', 'claude-opus-4-6', 'claude-opus-4-5-20251101', 'claude-haiku-4-5-20251001', 'claude-sonnet-4-5-20250929']
Perplexity models: []
Gemini models: []

=== Get OpenAI Models ===
OpenAI models: ['gpt-4-turbo-2024-04-09', 'gpt-realtime-2.1-mini', 'gpt-realtime-2025-08-28', 'gpt-4', 'tts-1-hd', 'gpt-realtime-1.5', 'gpt-realtime', 'gpt-5.2-pro', 'gpt-5-chat-latest', 'gpt-3.5-turbo-0125', 'gpt-5.4-2026-03-05', 'gpt-5-nano', 'gpt-5.4-mini', 'o1-pro', 'gpt-5.1-codex', 'tts-1', 'gpt-5-mini', 'gpt-5-mini-2025-08-07', 'omni-moderation-2024-09-26', 'gpt-live-transcribe', 'gpt-5.6-luna', 'omni-moderation-latest', 'gpt-5-2025-08-07', 'o4-mini-2025-04-16', 'gpt-4o-2024-11-20', 'gpt-5.1', 'whisper-1', 'gpt-5-pro', 'gpt-realtime-whisper', 'gpt-5.5-pro', 'gpt-audio-2025-08-28', 'gpt-4o-mini-transcribe-2025-03-20', 'gpt-realtime-2', 'gpt-5-search-api', 'gpt-5.5-2026-04-23', 'chat-latest', 'gpt-realtime-translate', 'gpt-5.4-pro-2026-03-05', 'gpt-5.4-mini-2026-03-17', 'babbage-002', 'gpt-4o-2024-05-13', 'text-embedding-3-large', 'gpt-4o-mini-transcribe-2025-12-15', 'tts-1-1106', 'gpt-5.5-pro-2026-04-23', 'gpt-audio-mini-2025-10-06', 'gpt-5.2-pro-2025-12-11', 'gpt-4-turbo', 'gpt-4o-2024-08-06', 'o3', 'gpt-4o-mini-search-preview-2025-03-11', 'o3-mini-2025-01-31', 'gpt-audio-1.5', 'gpt-image-1.5', 'gpt-5-pro-2025-10-06', 'gpt-5.6-sol', 'gpt-4.1-nano', 'gpt-image-2', 'gpt-3.5-turbo-16k', 'text-embedding-ada-002', 'gpt-realtime-2.1', 'gpt-4.1', 'gpt-5.5', 'gpt-4o-transcribe-diarize', 'gpt-5-codex', 'gpt-4-0613', 'gpt-5.1-2025-11-13', 'gpt-4o-search-preview', 'gpt-4.1-mini', 'gpt-4o-mini', 'gpt-5.4-pro', 'gpt-4o-mini-tts', 'gpt-realtime-mini-2025-12-15', 'gpt-5.1-codex-max', 'gpt-realtime-mini', 'gpt-image-1-mini', 'gpt-5.6-terra', 'gpt-3.5-turbo-instruct-0914', 'o1-2024-12-17', 'sora-2', 'tts-1-hd-1106', 'gpt-4o-mini-tts-2025-12-15', 'gpt-image-1', 'gpt-audio', 'gpt-5.3-codex', 'gpt-5-search-api-2025-10-14', 'gpt-5', 'gpt-4o-mini-transcribe', 'gpt-image-2-2026-04-21', 'gpt-5.2-chat-latest', 'sora-2-pro', 'gpt-3.5-turbo-instruct', 'gpt-transcribe', 'gpt-4.1-nano-2025-04-14', 'o4-mini-deep-research', 'gpt-5.4', 'gpt-4o-search-preview-2025-03-11', 'text-embedding-3-small', 'gpt-5.1-chat-latest', 'gpt-4o-mini-2024-07-18', 'gpt-4.1-2025-04-14', 'gpt-5.3-chat-latest', 'o3-2025-04-16', 'gpt-5.2-2025-12-11', 'chatgpt-image-latest', 'gpt-3.5-turbo', 'gpt-5.2-codex', 'gpt-4o', 'davinci-002', 'gpt-5.4-nano', 'o3-mini', 'o4-mini-deep-research-2025-06-26', 'gpt-audio-mini-2025-12-15', 'o1-pro-2025-03-19', 'gpt-5-nano-2025-08-07', 'gpt-4o-transcribe', 'o4-mini', 'gpt-5.1-codex-mini', 'gpt-3.5-turbo-1106', 'gpt-4o-mini-tts-2025-03-20', 'gpt-5.2', 'gpt-5.4-nano-2026-03-17', 'o1', 'gpt-image-2.5-sunburst-2026-09-08', 'gpt-6-luna', 'gpt-audio-mini', 'gpt-4.1-mini-2025-04-14', 'gpt-image-2.5-flare', 'gpt-4o-mini-search-preview', 'gpt-6-sol', 'gpt-6-astra', 'gpt-6.1-sol', 'gpt-live-1', 'gpt-image-2.5-sunburst', 'gpt-image-2.5-flare-2026-09-08']

=== Get Anthropic Models ===
Anthropic models: ['claude-haiku-5-5', 'claude-sonnet-5-5', 'claude-opus-5-5', 'claude-fable-5-1', 'claude-opus-5', 'claude-sonnet-5', 'claude-fable-5', 'claude-opus-4-8', 'claude-opus-4-7', 'claude-sonnet-4-6', 'claude-opus-4-6', 'claude-opus-4-5-20251101', 'claude-haiku-4-5-20251001', 'claude-sonnet-4-5-20250929']

✓ Chat Models API example complete
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: nCNPuKJv8GPtyXLCRvTkHGJYE9kPCha3eUtQo623t7zdHWswiQOEo7XnnWNhZ59T-5xdw9CJY-cGK5ErOAiRDw

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** A high-performance database product
- **Price:** $99

If you need more information about this product or any others, feel free to ask!

✓ Message 2 sent
  Response: The price of the product **ekoDB** is **$99**.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: CMWF5zMcVhO_-4tSMpu503Z1S_g-vfQLUBh9er7OqszyExy9vadH6Od-X0MTEQsjRI0ClP0xzGk1LNrULn6Opg
  Parent: nCNPuKJv8GPtyXLCRvTkHGJYE9kPCha3eUtQo623t7zdHWswiQOEo7XnnWNhZ59T-5xdw9CJY-cGK5ErOAiRDw

=== Listing Sessions ===
✓ Found 2 sessions
  Session 1: CMWF5zMcVhO_-4tSMpu503Z1S_g-vfQLUBh9er7OqszyExy9vadH6Od-X0MTEQsjRI0ClP0xzGk1LNrULn6Opg (Untitled)
  Session 2: nCNPuKJv8GPtyXLCRvTkHGJYE9kPCha3eUtQo623t7zdHWswiQOEo7XnnWNhZ59T-5xdw9CJY-cGK5ErOAiRDw (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: CMWF5zMcVhO_-4tSMpu503Z1S_g-vfQLUBh9er7OqszyExy9vadH6Od-X0MTEQsjRI0ClP0xzGk1LNrULn6Opg

=== Cleanup ===
✓ Deleted sessions and collection

✓ All session management operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: "Ii8jCWNX2g1G9DZoeWnA3l3BcT_8B_1OqP474LxAPk_uG0VIXzOXLbteI4VHxCFaA3P0aBHMxjoTU3m27p5lLA"

=== List Collections ===
Total collections: 13
Sample collections: ['chat_goals__ek0_testing', 'chat_agent_configs__ek0_testing', 'chat_tasks__ek0_testing', 'client_collection_management_python', 'chat_goal_templates__ek0_testing']

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: False

✓ All collection management operations completed successfully
✓ Client created

=== Check Collection Exists (Before Creation) ===
Collection 'collection_utils_test_py' exists: False

=== Creating Test Documents ===
Created 5 test documents

=== Check Collection Exists (After Creation) ===
Collection 'collection_utils_test_py' exists: True

=== Count Documents ===
Document count in 'collection_utils_test_py': 5

=== Check Non-Existent Collection ===
Collection 'nonexistent_collection_xyz' exists: False

=== Cleanup ===
Deleted collection 'collection_utils_test_py'

✓ Collection Utilities example complete
✓ Client created
✓ conc_demo_pay saved
✓ conc_demo_rl_fail saved
✓ conc_demo_rl_skip saved
✓ conc_demo_lock saved

Invoke them like:
  POST /api/functions/conc_demo_pay        { "idempotency_key": "...", "amount": 100 }
  POST /api/functions/conc_demo_rl_fail    { "user_id": 42 }
  POST /api/functions/conc_demo_rl_skip    { "user_id": 42 }
  POST /api/functions/conc_demo_lock       { "resource": "queue:drain" }

✓ Cleaned up demo functions
=== ekoDB Convenience Methods Example ===

=== Native Dict Creation ===
✓ Created record with plain dict: {'id': 'wA4zWJ24zL0FCo2fk4hHRMvdfgJPAc8ReEHc8VgMts8ZyyM42jAm3xFNbKmGjjMA-RwMOi-MfdX6IOBskCTFFA'}

=== Upsert Operation ===
✓ Upsert (update existing record): wA4zWJ24zL0FCo2fk4hHRMvdfgJPAc8ReEHc8VgMts8ZyyM42jAm3xFNbKmGjjMA-RwMOi-MfdX6IOBskCTFFA
✓ Inserted second record: naY99mZWVdXnuNbnF24mXyB3arbH4SK7iDsD-8HbaMHVBmkq6QHxlodk9iSy3X-Ay2oqzS15vLWvmx90oA2CGg
✓ Upsert (update second record): naY99mZWVdXnuNbnF24mXyB3arbH4SK7iDsD-8HbaMHVBmkq6QHxlodk9iSy3X-Ay2oqzS15vLWvmx90oA2CGg

=== Find One Operation ===
✓ Found user by email: {'id': 'wA4zWJ24zL0FCo2fk4hHRMvdfgJPAc8ReEHc8VgMts8ZyyM42jAm3xFNbKmGjjMA-RwMOi-MfdX6IOBskCTFFA', 'name': {'value': 'Alice Johnson', 'type': 'String'}, 'age': {'type': 'Integer', 'value': 29}, 'email': {'type': 'String', 'value': 'alice.j@newdomain.com'}, 'active': {'value': True, 'type': 'Boolean'}}
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
✓ Client created
✓ crypto_demo_hmac_py saved
✓ crypto_demo_aes_py saved
✓ crypto_demo_uuid_py saved
✓ crypto_demo_totp_py saved
✓ crypto_demo_encoding_py saved

Invoke them with:
  POST /api/functions/crypto_demo_hmac_py { "payload": "hi" }
  POST /api/functions/crypto_demo_aes_py { "plaintext": "secret" }
  POST /api/functions/crypto_demo_uuid_py
  POST /api/functions/crypto_demo_totp_py
  POST /api/functions/crypto_demo_encoding_py { "title": "Héllo World" }

✓ Cleaned up demo functions
=== Distinct Values Example ===

Inserting sample products...
Inserted 8 products

=== Distinct Categories (all products) ===
Found 3 distinct categories:
  - {'type': 'String', 'value': 'books'}
  - {'type': 'String', 'value': 'clothing'}
  - {'type': 'String', 'value': 'electronics'}

=== Distinct Statuses (all products) ===
Found 3 distinct statuses:
  - {'type': 'String', 'value': 'active'}
  - {'type': 'String', 'value': 'archived'}
  - {'type': 'String', 'value': 'discontinued'}

=== Distinct Statuses in Electronics ===
Found 2 distinct statuses for electronics:
  - {'type': 'String', 'value': 'active'}
  - {'type': 'String', 'value': 'discontinued'}

Cleanup done.
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: vOSp6OEkK6Pus6meC3P9NDFyqoCBSNGmmMhR45dJ_9R86n-BXayRqIV5XFyn5KDZp27FUg2nzLK52E9-HVEjJQ

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: a8H4fJG6CIOj6iyI5sSTmfud5W517WahyZLNWc8qCYpjdLcrDYUl_UIsHtUXYdvm70ulY0X9mumDEW_tqGhssQ

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
=== ekoDB as Edge Cache - Simple Example ===

Creating edge cache function...
✓ Edge cache script created: 2Ix1QX585skJmHnmF2F73_9IYx9zL8_OgXCSr_w6JghZls2C8somSSjTUvH2Xj9OwffOPEt0891lrcdo54HIeQ

Call 1: Cache miss (fetches from API)
Response time: 435ms
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "current": {
            "interval": 900,
            "temperature_2m": 20.6,
            "time": "2026-10-20T20:00"
          },
          "current_units": {
            "interval": "seconds",
            "temperature_2m": "\u00b0C",
            "time": "iso8601"
          },
          "elevation": 32.0,
          "generationtime_ms": 0.017642974853515625,
          "latitude": 40.710335,
          "longitude": -73.99308,
          "timezone": "GMT",
          "timezone_abbreviation": "GMT",
          "utc_offset_seconds": 0
        }
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

Call 2: Cache hit (served from ekoDB)
Response time: 2ms (236.9x faster!)
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "current": {
            "interval": 900,
            "temperature_2m": 20.6,
            "time": "2026-10-20T20:00"
          },
          "current_units": {
            "interval": "seconds",
            "temperature_2m": "\u00b0C",
            "time": "iso8601"
          },
          "elevation": 32.0,
          "generationtime_ms": 0.017642974853515625,
          "latitude": 40.710335,
          "longitude": -73.99308,
          "timezone": "GMT",
          "timezone_abbreviation": "GMT",
          "utc_offset_seconds": 0
        }
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

=== The Magic ===
- Your DATABASE is your EDGE
- No Redis needed
- No CDN needed
- No cache invalidation logic needed (TTL handles it)
- With ripples: All nodes auto-sync cache
- One service: Database + Cache + Edge Functions

✓ Example complete!

=== ekoDB Function Composition Examples ===

📋 Setting up test data...

✅ Test data ready

📝 Example 1: Basic Function Composition

Building reusable functions that call each other...

✅ Saved reusable function: fetch_user_py
✅ Saved composed function: get_user_wrapper_py (calls fetch_user_py + projects fields)

📊 Result from composed function:
   Records: 1
   Name: {"type": "String", "value": "User 1"}
   Department: {"type": "String", "value": "engineering"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user_py (uses KV)
✅ Saved SWR function using composition: swr_user_py

First call (cache miss - will fetch from API):
   ⏱️  Duration: 65.0ms
   📊 Records: 1
   📦 Data: {
      "value": {
            "type": "Object",
            "value": {
                  "address": {
                        "city": "Gwenborough",
                        "geo": {
                 ...

Second call (cache hit - from cache):
   ⏱️  Duration: 0.5ms
   📊 Records: 1
   📦 Data: {
      "value": {
            "type": "Object",
            "value": {
                  "address": {
                        "city": "Gwenborough",
                        "geo": {
                 ...
   🚀 Cache speedup: 130.0x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

✅ Level 1 function: validate_user_py
✅ Level 2 function: fetch_slim_user_py (calls validate_user_py)
✅ Level 3 function: get_verified_user_py (calls fetch_slim_user_py)

📊 Result from 3-level nested composition:
   Records: 1
   Name: User 1
   Department: engineering

🎯 Key Benefit: Each function is independently testable and reusable!
   - validate_user: Used in 100 different workflows
   - fetch_slim_user: Used in 50 workflows
   - get_verified_user: Specific workflow


✅ Cleanup complete
✅ All composition examples completed!
client_function_contract: ok
🚀 ekoDB Functions Example (Python)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved: Qt0v6OtplKmVd0KTqlDXhlp1YhS0BuuRFLbL6n6azdUUvcMSm9GgbK3ywbzNDAMCKPIqzwyCsU0reguTxrC1sA
📊 Found 5 active users

📝 Example 2: Parameterized Function

✅ Function saved: 6pAHV_0v4TFGv7Oli0-s8Jb8DaqJk7PksBCQcIGpVpvMNpMWGVmulyRUyFwit7CGugcJqpfQ0MzWb93DO1J-KA
📊 Found 3 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: SbWsywZBw4SqJLZnS8707RjFEgcqMnKOB_JggzjODzttRadnHTOth7gDUrPAl3ZEvzABr8AoyZ49ZIyplfcJgA
📊 Statistics: 2 groups
   {'avg_score': {'type': 'Float', 'value': 60.0}, 'count': {'type': 'Integer', 'value': 5}, 'status': {'type': 'String', 'value': 'active'}}

   {'avg_score': {'type': 'Float', 'value': 50.0}, 'count': {'type': 'Integer', 'value': 5}, 'status': {'type': 'String', 'value': 'inactive'}}

📝 Example 4: Function Management

📋 Total functions: 3
🔍 Retrieved function: Get Active Users
✏️  Function updated
🗑️  Function deleted

ℹ️  Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.

✅ All examples completed!
🚀 ekoDB Python Advanced Functions Example

📋 Setting up test data...
✅ Created 8 products

📝 Example 1: List All Products

✅ Function saved
📊 Found 8 products
⏱️  Execution time: 0ms

📝 Example 2: Group Products by Category

✅ Function saved
📊 Category breakdown:
   {'avg_price': {'type': 'Float', 'value': 367.0}, 'category': {'type': 'String', 'value': 'Electronics'}, 'count': {'type': 'Integer', 'value': 5}}
   {'avg_price': {'type': 'Float', 'value': 365.6666666666667}, 'category': {'type': 'String', 'value': 'Furniture'}, 'count': {'type': 'Integer', 'value': 3}}
⏱️  Execution time: 0ms

📝 Example 3: Count Total Products

✅ Function saved
📊 Total products: [{'total': {'type': 'Integer', 'value': 8}}]
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced script examples finished!
🚀 ekoDB Python AI Functions Example

📋 Setting up test data...
✅ Created 2 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
🤖 AI Response:
   Vector databases offer several benefits:

1. **Efficient Similarity Searches**: They enable fast searching and retrieval of data based on vector similarity, which is ideal for tasks like recommendation systems and image retrieval.

2. **High-Dimensional Data Handling**: They are optimized for storing and processing high-dimensional vectors, which are common in machine learning applications.

3. **Scalability**: Vector databases can handle large volumes of data efficiently, often leveraging distributed architectures.

4. **Integration with AI and ML**: They support advanced analytics and machine learning model outputs, making it easier to manage and query embeddings.

5. **Real-Time Processing**: Many vector databases are designed for quick data ingestion and real-time querying, suitable for dynamic applications.

6. **Flexibility**: They can accommodate various types of data and use cases, including text, images, and audio embeddings.

7. **Support for Advanced Indexing**: Techniques like HNSW or ANNOY provide optimized indexing methods for faster retrieval times compared to traditional databases.

8. **Rich Query Capabilities**: Users can perform complex queries based on vector distance metrics, enabling nuanced data exploration.

These features make vector databases particularly powerful for applications in AI, natural language processing, and other fields requiring high-performance data analysis.
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embed script saved
📊 Generated 2 embeddings
   Dimensions: 1536
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All AI script examples finished!

💡 This example demonstrates:
   ✅ Chat completions with system/user messages
   ✅ Embedding generation for text
🚀 ekoDB Python Complete Functions Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Function saved: o5CYRYm-5NxpOJHxaxNacHsr9D5JEbkwyPD6wh5lVEZiNo_6VaAwGYEgp43PwAlZRAm3C2cpzQ7K4vbfPMYsLQ
📊 Found 2 product groups
   {'avg_price': {'type': 'Float', 'value': 575.6666666666666}, 'category': {'type': 'String', 'value': 'Electronics'}, 'count': {'type': 'Integer', 'value': 3}}
   {'avg_price': {'type': 'Float', 'value': 474.0}, 'category': {'type': 'String', 'value': 'Furniture'}, 'count': {'type': 'Integer', 'value': 2}}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   {'category': {'type': 'String', 'value': 'Electronics'}, 'count': {'type': 'Integer', 'value': 3}}
   {'category': {'type': 'String', 'value': 'Furniture'}, 'count': {'type': 'Integer', 'value': 2}}
⏱️  Execution time: 0ms

📝 Example 4: High Rating Products

✅ Function saved
📊 Found 3 products
⏱️  Execution time: 0ms

📝 Example 5: function with Parameter Definition

✅ Function saved
📊 Found 3 products
⏱️  Execution time: 0ms

📝 Example 6: Multi-Stage Pipeline (FindAll → Group → Count)

✅ Function saved
📊 Pipeline executed 3 stages
⏱️  Total execution time: 0ms

✅ All complete script examples finished!

💡 This example demonstrates ekoDB's function system:
   ✅ FindAll operations
   ✅ Group aggregations (Count, Average)
   ✅ Multi-stage pipelines (FindAll → Group → Count)
   ✅ Parameter definitions
   ✅ Function management (save, call, delete)
🧹 Cleaning up...
✅ Cleanup complete

🚀 ekoDB Python CRUD Functions Example

📋 Setting up test data...
✅ Created 10 test users

📝 Example 1: List All Users

✅ Function saved
📊 Found 10 users
⏱️  Execution time: 0ms

📝 Example 2: Count Users by Status

✅ Function saved
📊 User counts by status:
   inactive: 3 users
   active: 7 users
⏱️  Execution time: 0ms

📝 Example 3: Average Score by Role

✅ Function saved
📊 Average score by role:
   {'avg_score': {'type': 'Float', 'value': 70.0}, 'count': {'type': 'Integer', 'value': 7}, 'role': {'type': 'String', 'value': 'user'}}
   {'avg_score': {'type': 'Float', 'value': 20.0}, 'count': {'type': 'Integer', 'value': 3}, 'role': {'type': 'String', 'value': 'admin'}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD script examples finished!
🚀 ekoDB Python KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: hnAApYsOT4sVKzuvt3cjQ6G9htiQBalQOf1xT_E7EkQfWaSjUOCXPT7_olkpgBfRGlHae2rxtQ9169IWYdu_Vw
✅ Inserted 2 products with wrapped types

📝 Example 2: Querying and Extracting Wrapped Types

📊 Found 2 products
   • Laptop Pro
   • Wireless Mouse

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {'value': '{"type":"Object","value":{"role":"admin","userId":"user_abc"}}'}
🔍 Key exists: True
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Pattern Query

✅ Set 4 config entries
📊 Found 3 app config entries
📊 Found 4 total config entries

📝 Example 5: Combined Wrapped Types + KV Usage

✅ Inserted order: 3M1ZQAN5VbYP6fy4FnDcG7V28UNTQs7uJNOCNiQ1qKJK0Xbp-ZDwdUyw5OMfXqYGoycyQFOcHPx_7zqhIP4h2g
✅ Cached order status
📊 Quick status lookup: {'value': '{"type":"Object","value":{"status":"processing","updated_at":"2026-10-20T20:02:49.923939+00:00"}}'}

🧹 Cleaning up...
✅ Cleanup complete

✅ All KV & Wrapped Types examples completed!

💡 Key takeaways:
   ✅ Use field_* helpers for type-safe wrapped values
   ✅ field_decimal() preserves precision (no floating point errors)
   ✅ KV store is great for caching and quick lookups
   ✅ Combine KV caching with collection inserts for real workflows
🚀 ekoDB Python Search Functions Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Function saved
📊 Found 5 documents
   1. Getting Started with ekoDB (Database)
   2. Introduction to Machine Learning (AI)
   3. Vector Databases Explained (Database)
   4. Database Design Principles (Database)
   5. Natural Language Processing (AI)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   {'category': {'type': 'String', 'value': 'AI'}, 'count': {'type': 'Integer', 'value': 2}}
   {'category': {'type': 'String', 'value': 'Database'}, 'count': {'type': 'Integer', 'value': 3}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
=== ekoDB Goal Template CRUD Example (Python) ===

--- Creating goal template ---
Created template: Data Migration (id: jm-Rb3OxkcOD5i10hYYCZV2JAHf3TXb-Gn0xhgW-JfrbihCv6DFPiGiwB1b7q8oJaKzWmLO__wNMgn3YaF0BeA)

--- Listing templates ---
Templates: {'count': 1, 'items': [{'description': {'type': 'String', 'value': 'Template for migrating data between schemas'}, 'id': 'jm-Rb3OxkcOD5i10hYYCZV2JAHf3TXb-Gn0xhgW-JfrbihCv6DFPiGiwB1b7q8oJaKzWmLO__wNMgn3YaF0BeA', 'steps': {'type': 'Array', 'value': [{'description': 'Analyze source schema'}, {'description': 'Create target schema'}, {'description': 'Migrate records'}, {'description': 'Validate results'}]}, 'title': {'type': 'String', 'value': 'Data Migration'}}]}

--- Getting template ---
Fetched: {'type': 'String', 'value': 'Data Migration'}

--- Updating template ---
Updated description: {'type': 'String', 'value': 'Updated: comprehensive data migration workflow'}

--- Deleting template ---
Template deleted successfully

✓ Goal template CRUD example completed
=== ekoDB Goals / Tasks / Agents Integration Example (Python) ===

--- goal_create ---
Created goal: Deploy v3 (id: gPzgQkh0gXEtE0JGjkFf7JPGWDIzDxmc_4dkkgZKqSTtrZTKjRR8Wtm-tzfVd5k0Yr5uJf4zlSvR5c2MEZI3wg)

--- goal_list ---
Goals: {'count': 1, 'goals': [{'created_at': '2026-10-20T20:02:50.272828+00:00', 'description': 'Deploy version 3 to production', 'id': 'gPzgQkh0gXEtE0JGjkFf7JPGWDIzDxmc_4dkkgZKqSTtrZTKjRR8Wtm-tzfVd5k0Yr5uJf4zlSvR5c2MEZI3wg', 'status': 'pending', 'steps': '[{"description":"Build Docker image","status":"pending"},{"description":"Run migrations","status":"pending"},{"description":"Smoke test","status":"pending"}]', 'title': 'Deploy v3', 'updated_at': '2026-10-20T20:02:50.272828+00:00'}]}

--- goal_get ---
Fetched: {'type': 'String', 'value': 'Deploy v3'}

--- goal_update ---
Updated description: {'type': 'String', 'value': 'Deploy version 3 to production (updated)'}

--- goal_search ---
Search results: {'count': 1, 'items': [{'_score': 13.2, 'created_at': {'type': 'DateTime', 'value': '2026-10-20T20:02:50.272828+00:00'}, 'description': {'type': 'String', 'value': 'Deploy version 3 to production (updated)'}, 'id': 'gPzgQkh0gXEtE0JGjkFf7JPGWDIzDxmc_4dkkgZKqSTtrZTKjRR8Wtm-tzfVd5k0Yr5uJf4zlSvR5c2MEZI3wg', 'status': {'type': 'String', 'value': 'pending'}, 'steps': {'type': 'String', 'value': '[{"description":"Build Docker image","status":"pending"},{"description":"Run migrations","status":"pending"},{"description":"Smoke test","status":"pending"}]'}, 'title': {'type': 'String', 'value': 'Deploy v3'}, 'updated_at': {'type': 'DateTime', 'value': '2026-10-20T20:02:50.277832+00:00'}}]}

--- goal_step_start (step 0) ---
Step 0 started: {'created_at': {'type': 'DateTime', 'value': '2026-10-20T20:02:50.272828+00:00'}, 'description': {'type': 'String', 'value': 'Deploy version 3 to production (updated)'}, 'id': 'gPzgQkh0gXEtE0JGjkFf7JPGWDIzDxmc_4dkkgZKqSTtrZTKjRR8Wtm-tzfVd5k0Yr5uJf4zlSvR5c2MEZI3wg', 'status': {'type': 'String', 'value': 'in_progress'}, 'steps': {'type': 'String', 'value': '[{"description":"Build Docker image","status":"InProgress"},{"description":"Run migrations","status":"pending"},{"description":"Smoke test","status":"pending"}]'}, 'title': {'type': 'String', 'value': 'Deploy v3'}, 'updated_at': {'type': 'DateTime', 'value': '2026-10-20T20:02:50.283185+00:00'}}

--- goal_step_complete (step 0) ---
Step 0 completed: {'created_at': {'type': 'DateTime', 'value': '2026-10-20T20:02:50.272828+00:00'}, 'description': {'type': 'String', 'value': 'Deploy version 3 to production (updated)'}, 'id': 'gPzgQkh0gXEtE0JGjkFf7JPGWDIzDxmc_4dkkgZKqSTtrZTKjRR8Wtm-tzfVd5k0Yr5uJf4zlSvR5c2MEZI3wg', 'status': {'type': 'String', 'value': 'in_progress'}, 'steps': {'type': 'String', 'value': '[{"description":"Build Docker image","result":"Docker image built: sha256:abc123","status":"Completed"},{"description":"Run migrations","status":"pending"},{"description":"Smoke test","status":"pending"}]'}, 'title': {'type': 'String', 'value': 'Deploy v3'}, 'updated_at': {'type': 'DateTime', 'value': '2026-10-20T20:02:50.287431+00:00'}}

--- goal_step_fail (step 1) ---
Step 1 failed: {'created_at': {'type': 'DateTime', 'value': '2026-10-20T20:02:50.272828+00:00'}, 'description': {'type': 'String', 'value': 'Deploy version 3 to production (updated)'}, 'id': 'gPzgQkh0gXEtE0JGjkFf7JPGWDIzDxmc_4dkkgZKqSTtrZTKjRR8Wtm-tzfVd5k0Yr5uJf4zlSvR5c2MEZI3wg', 'status': {'type': 'String', 'value': 'in_progress'}, 'steps': {'type': 'String', 'value': '[{"description":"Build Docker image","result":"Docker image built: sha256:abc123","status":"Completed"},{"description":"Run migrations","error":"Migration failed: column already exists","status":"Failed"},{"description":"Smoke test","status":"pending"}]'}, 'title': {'type': 'String', 'value': 'Deploy v3'}, 'updated_at': {'type': 'DateTime', 'value': '2026-10-20T20:02:50.291686+00:00'}}

--- goal_complete ---
Goal completed: {'type': 'String', 'value': 'pending_review'}

--- goal_approve ---
Goal approved: {'type': 'String', 'value': 'in_progress'}

--- goal_reject ---
Goal rejected: {'type': 'String', 'value': 'failed'}

--- task_create ---
Created task: Nightly Backup (id: ldFRnZA6vWvv2hCeeRLqHXkn5JJvUEyy7pedkdovHe5WWv8uyvgg1AglyjiEXhts3QfeooyGNgPGsVbjN7xHdw)

--- task_list ---
Tasks: {'count': 1, 'items': [{'cron': {'type': 'String', 'value': '0 2 * * *'}, 'description': {'type': 'String', 'value': 'Backup all databases nightly'}, 'id': 'ldFRnZA6vWvv2hCeeRLqHXkn5JJvUEyy7pedkdovHe5WWv8uyvgg1AglyjiEXhts3QfeooyGNgPGsVbjN7xHdw', 'max_consecutive_failures': {'type': 'Integer', 'value': 3}, 'name': {'type': 'String', 'value': 'Nightly Backup'}}]}

--- task_get ---
Fetched: {'type': 'String', 'value': 'Nightly Backup'}

--- task_start ---
Task started: {'type': 'String', 'value': 'running'}

--- task_succeed ---
Task succeeded: {'consecutive_failures': {'type': 'Integer', 'value': 0}, 'cron': {'type': 'String', 'value': '0 2 * * *'}, 'description': {'type': 'String', 'value': 'Backup all databases nightly'}, 'id': 'ldFRnZA6vWvv2hCeeRLqHXkn5JJvUEyy7pedkdovHe5WWv8uyvgg1AglyjiEXhts3QfeooyGNgPGsVbjN7xHdw', 'last_error': {'type': 'Null', 'value': None}, 'last_run': {'type': 'DateTime', 'value': '2026-10-20T20:02:50.331956+00:00'}, 'max_consecutive_failures': {'type': 'Integer', 'value': 3}, 'name': {'type': 'String', 'value': 'Nightly Backup'}, 'run_count': {'type': 'Integer', 'value': 1}, 'status': {'type': 'String', 'value': 'active'}, 'updated_at': {'type': 'DateTime', 'value': '2026-10-20T20:02:50.331956+00:00'}}

--- task_pause ---
Task paused: {'type': 'String', 'value': 'paused'}

--- task_resume ---
Task resumed: {'type': 'String', 'value': 'active'}

--- task_fail ---
Task failed: {'consecutive_failures': {'type': 'Integer', 'value': 1}, 'cron': {'type': 'String', 'value': '0 2 * * *'}, 'description': {'type': 'String', 'value': 'Backup all databases nightly'}, 'id': 'ldFRnZA6vWvv2hCeeRLqHXkn5JJvUEyy7pedkdovHe5WWv8uyvgg1AglyjiEXhts3QfeooyGNgPGsVbjN7xHdw', 'last_error': {'type': 'String', 'value': 'Disk full'}, 'last_run': {'type': 'DateTime', 'value': '2026-10-20T20:02:50.351113+00:00'}, 'max_consecutive_failures': {'type': 'Integer', 'value': 3}, 'name': {'type': 'String', 'value': 'Nightly Backup'}, 'next_run': {'type': 'DateTime', 'value': '2026-03-22T02:00:00+00:00'}, 'run_count': {'type': 'Integer', 'value': 1}, 'status': {'type': 'String', 'value': 'active'}, 'updated_at': {'type': 'DateTime', 'value': '2026-10-20T20:02:50.351113+00:00'}}

--- task_due ---
Due tasks: {'count': 0, 'items': []}

--- task_delete ---
Task deleted

--- agent_create ---
Created agent: CodeReviewer (id: ptj5TV5yYQGFVjgjHu5LXXL1_2CDLQq5Z8eJJQy1ba0Kj9MLywmVyiLbadJclySJX9ScD8jckyKcSrC1ciwAAQ)

--- agent_list ---
Agents: {'count': 1, 'items': [{'deployment_id': {'type': 'String', 'value': 'deploy_test'}, 'id': 'ptj5TV5yYQGFVjgjHu5LXXL1_2CDLQq5Z8eJJQy1ba0Kj9MLywmVyiLbadJclySJX9ScD8jckyKcSrC1ciwAAQ', 'llm_model': {'type': 'String', 'value': 'gpt-4o'}, 'name': {'type': 'String', 'value': 'CodeReviewer'}, 'system_prompt': {'type': 'String', 'value': 'You review code for correctness and style.'}, 'tools': {'type': 'Array', 'value': ['web_search', 'file_read']}}]}

--- agent_get ---
Fetched: {'type': 'String', 'value': 'CodeReviewer'}

--- agent_get_by_name ---
By name: {'type': 'String', 'value': 'CodeReviewer'}

--- agent_update ---
Updated agent prompt: {'type': 'String', 'value': 'You review code. Be concise.'}

--- agents_by_deployment ---
Agents in deploy_test: {'count': 0, 'items': []}
WARNING: agents_by_deployment omitted created agent ptj5TV5yYQGFVjgjHu5LXXL1_2CDLQq5Z8eJJQy1ba0Kj9MLywmVyiLbadJclySJX9ScD8jckyKcSrC1ciwAAQ; TODO: check/fix the server-side deployment lookup

--- agent_delete ---
Agent deleted

--- cleanup ---
Goals cleaned up

=== All goals/tasks/agents operations completed successfully ===
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
✓ py_users_register saved
✓ py_users_login saved
✓ py_users_verify_token saved

=== Auth flow defined as pure stored functions ===
Call them like:
  POST /api/functions/py_users_register { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/py_users_login    { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/py_users_verify_token { "token": "<jwt>" }

✓ Cleaned up demo functions
=== ekoDB KV Links Example (Python) ===

--- kv_set ---
Set KV key: app:config

--- insert document ---
Inserted document: t6s0ojNhq3qM7GjPh_OC8AZJ_jaAvJKebs4-2eBDWmEJWIxiSowSIRDRD7kP4OhxuYiv5y4RxMLOR93pi2cXrw

--- kv_link ---
Linked: None

--- kv_get_links ---
Links for app:config: [{'collection': 'kv_links_example_py', 'created_at': '2026-10-20T20:02:50.847959Z', 'document_id': 't6s0ojNhq3qM7GjPh_OC8AZJ_jaAvJKebs4-2eBDWmEJWIxiSowSIRDRD7kP4OhxuYiv5y4RxMLOR93pi2cXrw', 'field_path': None, 'last_accessed': '2026-10-20T20:02:50.848215Z', 'metadata': {}}]

--- kv_unlink ---
Unlinked: None

--- kv_get_links (verify empty) ---
Links after unlink: []

--- cleanup ---
Cleaned up KV key and collection

=== KV links example completed ===
✓ Client created

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: {'value': '{"type":"Object","value":{"userId":123,"username":"john_doe"}}'}

=== KV Batch Set ===
✓ Batch set 3 keys
  kv_operations:py:cache:product:1: success
  kv_operations:py:cache:product:2: success
  kv_operations:py:cache:product:3: success

=== KV Batch Get ===
✓ Batch retrieved 3 values
  kv_operations:py:cache:product:1: {'name': 'Product 1', 'price': 29.99}
  kv_operations:py:cache:product:2: {'price': 39.99, 'name': 'Product 2'}
  kv_operations:py:cache:product:3: {'price': 49.99, 'name': 'Product 3'}

=== KV Exists ===
Key exists: True

=== KV Find (Pattern Query) ===
Found 3 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 4

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: False

=== KV Batch Delete ===
✓ Batch deleted 3 keys
  kv_operations:py:cache:product:1: deleted
  kv_operations:py:cache:product:2: deleted
  kv_operations:py:cache:product:3: deleted

✓ All KV operations completed successfully
=== KV Precision: Float vs Decimal ===

=== Test 1: Using Python Floats (LOSES PRECISION) ===
✓ Stored products with float prices

Retrieved float prices:
  Widget A: $29.99 (expected $29.99) ✓
  Widget B: $39.99 (expected $39.99) ✓
  Widget C: $49.99 (expected $49.99) ✓

=== Test 2: Using field_decimal() (PRESERVES PRECISION) ===
✓ Stored products with decimal prices

Retrieved decimal prices:
  Widget A: $29.99 (expected $29.99) ✓
  Widget B: $39.99 (expected $39.99) ✓
  Widget C: $49.99 (expected $49.99) ✓

=== Test 3: Sum Calculation Comparison ===
  Float sum: $119.97 (expected $119.97)
  Decimal sum: $119.97 (expected $119.97)

=== Test 4: Extreme Precision Example ===
  Float 0.1 + 0.2 = 0.30000000000000004 (should be 0.3)
  Decimal "0.30" = 0.30 (exact!)

=== Cleanup ===
✓ Cleaned up test keys

=== Summary ===
✅ Use field_decimal() for monetary values, percentages, and
   any case where floating-point errors are unacceptable.
✅ field_decimal() stores values as strings internally,
   preserving exact precision across all operations.
✓ Client created
✓ py_route_admin saved
✓ py_route_user_by_id saved
✓ py_route_user_posts saved
✓ py_route_org_create_member saved

Try them with curl:
  curl http://localhost:8080/api/route/users/admin
  curl http://localhost:8080/api/route/users/42
  curl http://localhost:8080/api/route/users/42/posts/7
  curl -X POST http://localhost:8080/api/route/orgs/acme/members \
       -H 'Content-Type: application/json' -d '{"name":"alice"}'

✓ Cleaned up demo functions
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
=== ekoDB Raw Completion Stream (SSE) Example ===

--- Basic SSE Raw Completion ---
Response: The three primary colors are red, blue, and yellow.

--- Structured Output via SSE ---
JSON response: [
  {
    "name": "Earth",
    "diameter_km": 12742
  },
  {
    "name": "Mars",
    "diameter_km": 6779
  },
  {
    "name": "Jupiter",
    "diameter_km": 139820
  }
]

--- Blocking HTTP (for comparison) ---
Blocking response: Hello! I hope you're having a wonderful day.

=== Done ===
=== ekoDB Schedule Management Example (Python) ===

--- create_schedule ---
Created schedule: Nightly Report (id: 83e8a7a6-f172-4393-83d3-aaa111f50563)

--- list_schedules ---
Schedules: {'count': 1, 'schedules': [{'created_at': '2026-10-20T20:02:54.062803Z', 'cron_expression': '0 0 0 * * *', 'description': 'Generate and email nightly analytics report', 'enabled': True, 'function_label': 'schedule_noop_python_56896', 'id': '83e8a7a6-f172-4393-83d3-aaa111f50563', 'last_execution': None, 'name': 'Nightly Report', 'next_execution': '2026-10-21T00:00:00Z', 'parameters': {}, 'stats': {'avg_execution_time_ms': 0.0, 'failed_executions': 0, 'last_error': None, 'successful_executions': 0, 'total_executions': 0}, 'timezone': 'UTC', 'updated_at': '2026-10-20T20:02:54.062803Z'}]}

--- get_schedule ---
Fetched: Nightly Report cron=0 0 0 * * *

--- update_schedule ---
Updated cron: 0 30 1 * * *

--- trigger_schedule ---
Triggered: {'schedule_id': '83e8a7a6-f172-4393-83d3-aaa111f50563', 'status': 'triggered'}

--- pause_schedule ---
Paused: enabled=False

--- resume_schedule ---
Resumed: enabled=True
Schedule deleted successfully

=== Schedule management example completed ===
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
=== Search Examples ===

Setting up test data...
✅ Test data created

1. Basic full-text search:
Found 2 results
  1. Score: 12.870, Matched: email, name
  2. Score: 6.270, Matched: name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 13.200, Matched: bio, title
  2. Score: 13.200, Matched: bio, title
  3. Score: 13.200, Matched: bio, title
  4. Score: 13.200, Matched: bio, title

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 26.400, Matched: bio, title
  2. Score: 26.400, Matched: bio, title
  3. Score: 26.400, Matched: title, bio
  4. Score: 26.400, Matched: bio, title

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 6.600, Matched: bio
  2. Score: 6.600, Matched: bio

5. Search with stemming and exact match boosting:
Found 2 results (matches: run, running, runs)
  1. Score: 6.600, Matched: bio
  2. Score: 6.600, Matched: bio

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.773
  2. Score: 0.761
  3. Score: 0.754

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.377, Matched: content, title
  2. Score: 0.887, Matched: content, title
  3. Score: 0.381, Matched:

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 6.600, Matched: title

9. Vector search with a metadata pre-filter (category = ml):
Found 2 documents in category "ml" (NLP excluded)
  1. Deep Learning Fundamentals (category: ml)
  2. Introduction to Machine Learning (category: ml)

Paired vector search: document=Rust Programming Answer, query=Rust Programming Question
=== Cleanup ===
✅ Deleted test collections

✅ Search examples completed!
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {'id': '2D1YzT2utjCqvVcz32kfaS5YaDcrZV2XemqmfEGLSr6QyOCoy55q7veg2jQlxmubElk6B81yc6y4SmlYkmHo1g'}

=== Find by ID ===
Found: {'id': '2D1YzT2utjCqvVcz32kfaS5YaDcrZV2XemqmfEGLSr6QyOCoy55q7veg2jQlxmubElk6B81yc6y4SmlYkmHo1g', 'data': {'value': 'aGVsbG8gd29ybGQ=', 'type': 'String'}, 'active': {'type': 'Boolean', 'value': True}, 'tags': {'type': 'Array', 'value': ['tag1', 'tag2', 'tag3']}, 'name': {'type': 'String', 'value': 'Test Record'}, 'categories': {'type': 'Array', 'value': ['electronics', 'computers']}, 'price': {'value': 99.99, 'type': 'Float'}, 'value': {'type': 'Integer', 'value': 42}, 'created_at': {'type': 'String', 'value': '2026-10-20T16:02:54.514135'}, 'embedding': {'value': [0.1, 0.2, 0.3, 0.4, 0.5], 'type': 'Array'}, 'user_id': {'type': 'String', 'value': '550e8400-e29b-41d4-a716-446655440000'}, 'metadata': {'type': 'Object', 'value': {'key': 'value', 'nested': {'deep': True}}}}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): True
  price (Decimal): 99.99
  created_at (DateTime): 2026-10-20 16:02:54.514135
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): ['tag1', 'tag2', 'tag3']
  metadata (Object): {'key': 'value', 'nested': {'deep': True}}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): ['electronics', 'computers']
  data (Bytes): 11 bytes
Plain record: {'id': '2D1YzT2utjCqvVcz32kfaS5YaDcrZV2XemqmfEGLSr6QyOCoy55q7veg2jQlxmubElk6B81yc6y4SmlYkmHo1g', 'data': 'aGVsbG8gd29ybGQ=', 'active': True, 'tags': ['tag1', 'tag2', 'tag3'], 'name': 'Test Record', 'categories': ['electronics', 'computers'], 'price': 99.99, 'value': 42, 'created_at': '2026-10-20T16:02:54.514135', 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5], 'user_id': '550e8400-e29b-41d4-a716-446655440000', 'metadata': {'key': 'value', 'nested': {'deep': True}}}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'metadata': {'type': 'Object', 'value': {'key': 'value', 'nested': {'deep': True}}}, 'data': {'type': 'String', 'value': 'aGVsbG8gd29ybGQ='}, 'id': '2D1YzT2utjCqvVcz32kfaS5YaDcrZV2XemqmfEGLSr6QyOCoy55q7veg2jQlxmubElk6B81yc6y4SmlYkmHo1g', 'user_id': {'value': '550e8400-e29b-41d4-a716-446655440000', 'type': 'String'}, 'price': {'type': 'Float', 'value': 99.99}, 'created_at': {'type': 'String', 'value': '2026-10-20T16:02:54.514135'}, 'name': {'type': 'String', 'value': 'Updated Record'}, 'embedding': {'type': 'Array', 'value': [0.1, 0.2, 0.3, 0.4, 0.5]}, 'categories': {'value': ['electronics', 'computers'], 'type': 'Array'}, 'value': {'type': 'Integer', 'value': 100}, 'tags': {'type': 'Array', 'value': ['tag1', 'tag2', 'tag3']}, 'active': {'value': True, 'type': 'Boolean'}}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: N0OrindsygZ1R8MovOYIC98CU7BzfjXC0x-kunTE9GL4Onz5liPVmEYoMdd6bDxq2RhIyppDdJW3UjjyCTQ6cA

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
🚀 ekoDB Python Client - Native SWR Function Examples

📋 Demonstrates:
   • Single-function SWR pattern (replaces 4-step pipeline)
   • Automatic cache checking, HTTP fetching, and cache setting
   • Built-in audit trail support
   • Duration string TTLs ('15m', '1h', '30s')
   • Multi-function pipeline integration
   • Dynamic TTL configuration


🧹 Cleaning up...
✓ Deleted 0 test scripts and owned SWR resources

Example 1: Basic Native SWR
────────────────────────────────────────────────────────────────────────────────
Single function replaces KvGet → If → HttpRequest → KvSet pipeline
✓ Created native SWR script: github_user_native_py (fjz23WIwTvWGD-xFbbVzN-DvRzoRrownM9rOBdqZb2pDOM2VR3oh_BU6RusOToZn7jcX6IlZ8P3UZEe1T20ctg)

First call (cache miss - will fetch from GitHub API):
  Response time: 133ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 1ms
  Speedup: 212.6x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit_py (k3_6ruOBq8_DlUXssBZrdsN00zazRAMpNoOdEs5kKPU2fi8OHraedyHxh-BSx9LwYuAKUXt5HEA9E7vha9tZyg)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail_py' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline_py (vyicxqomYCBV8uES_oRLSM-SKx42nsqt5AI9l65-jIyQjF6pxJRQJ2JIILWn8byoLbcesF1zoPFW8dChSadU3Q)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users_swr_py' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache_py (tMEUbhFVE3Z4h_o54-Gu6rmXMOOsJ57a-GX8vX3tvCernGqnpgEFVvU1ExnNjlq8Fc_7WJidf9N9-7rljrDgsQ)
  ✓ Cached with TTL: 5m (5 minutes)
  ✓ Cached with TTL: 1h (1 hour)
  ✓ Cached with TTL: 30s (30 seconds)

================================================================================
✅ Key Benefits of Native SWR:
✅ Single function: Replaces 4-function cache-aside pattern
✅ Duration strings: Use '15m', '1h', '2h' instead of calculating seconds
✅ Built-in audit: Optional collection parameter for automatic logging
✅ Auto-enrichment: output_field populates params for downstream functions
✅ Transactional: Works correctly in both transactional and non-transactional contexts
✅ KV-optimized: Uses native KV store with proper TTL handling

=== Performance Comparison ===
Legacy Pattern: KvGet → If → HttpRequest → KvSet → Insert (5 functions)
Native SWR:     SWR → Insert (2 functions)
Result:         60% fewer functions, cleaner code, same behavior 🎯

🧹 Cleaning up...
✓ Deleted 4 test scripts and owned SWR resources

✅ All examples completed!
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_api_user_py (qf8xW49kbWWXkQegqXnUgVim6DcSActbptW6hajQMlXIBPIjsNGgrk8MtCi6Ww3tSFmoiLy5iFOcXAAqpvlGww)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
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
Response time: 1ms (served from cache)
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
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
✓ Lightning fast cache hit

🧹 Cleaning up...
✓ Cleanup complete

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: HDkdJOLpa0zxrq7W0u98HcUEQFzKscXF4tDJLwhQmb53CCRfeShMfJlmoKcS9eapcovolTDpNvdS0vabCfSsIA
Created Bob: $500 - ID: FBDWzwcZfB_-bFgXsbx9Ir5pcyRtB2fZTwOSxWSqizqFdmkx9OiIvaQqQqfmmB6YToM2N-lbbAdm6p3A-Bd-Vw

=== Example 1: Begin Transaction ===
Transaction ID (server-default isolation): 925c49fe-3113-456a-9edd-49973fa2371d

=== Example 2: Operations within Transaction ===
Alice in transaction: {'account_id': {'type': 'String', 'value': 'ACC001'}, 'balance': {'type': 'Integer', 'value': 800}, 'name': {'type': 'String', 'value': 'Alice'}, 'id': 'HDkdJOLpa0zxrq7W0u98HcUEQFzKscXF4tDJLwhQmb53CCRfeShMfJlmoKcS9eapcovolTDpNvdS0vabCfSsIA'}
Bob in transaction: {'name': {'type': 'String', 'value': 'Bob'}, 'balance': {'value': 700, 'type': 'Integer'}, 'account_id': {'type': 'String', 'value': 'ACC002'}, 'id': 'FBDWzwcZfB_-bFgXsbx9Ir5pcyRtB2fZTwOSxWSqizqFdmkx9OiIvaQqQqfmmB6YToM2N-lbbAdm6p3A-Bd-Vw'}
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

✓ Verified committed balances: Alice=$800, Bob=$700

=== Example 5: Rollback Demo ===
New transaction: de2fd121-d09c-4824-b283-bd3d7272051c
Updated Bob: $700 → $600 (in transaction)
Status before rollback: Active
✓ Transaction rolled back

✓ Verified Bob remains $700 after rollback

=== Cleanup ===
✓ Deleted test account collection

✓ All client transaction examples completed
✓ Client created

=== Create User Function ===
Created user function with ID: YkGNNRC6qplXPSuXvQrpBjUh10x1JFcuJ1bnZO8mYByK4UnPH5aPZZ8dl5cgcHBeW1l4OemqlBbTQRykGvetHw

=== Get User Function ===
Retrieved: get_active_users_py - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 1 user functions:
  - get_active_users_py: Get Active Users

=== Update User Function ===
User function updated successfully

=== Delete User Function ===
User function deleted successfully

✓ User Functions API example complete
=== WebSocket Chat Streaming Example (Python) ===

Created chat session: mFBBvl_69b0CodzumAFRnJ2rjNf5vC1grEGpgZRQMgxkcsx1Y4TnirSPECUsaxN5xccj02U6DyXR-Iok6gwrGA

Sending message: 'What is the capital of France?'
{"__progress":"received"}{"__progress":"generating","model":"gpt-4o-mini","provider":"openai"}The capital of France is Paris.

--- Stream ended ---
Message ID: CuCrr1gsAensHYrNeShU7_E2DQoYhZki-sx51-Kzd8pqaOFbFc6G3pcOv_AHCkrsiEn5TYCw4mt3gK2GiPKXqg
Execution time: 516ms
Token usage: {'completion_tokens': 8, 'prompt_tokens': 15, 'total_tokens': 23}

Full response: {"__progress":"received"}{"__progress":"generating","model":"gpt-4o-mini","provider":"openai"}The capital of France is Paris....
=== WebSocket Subscription Example (Python) ===

✓ Authentication successful
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_py' ===
✓ Subscribed (subscription_id: sub_5bf5503d1b43466ea2427529bd9faa77)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: CFCb7jAhU78eKhfIePi7Bh5UItdUvJAfBJ1aF-zWOZISw9P643zYokD428_XPR9E4-Dv2MBGb4-2WTNN5z0WJg

  📡 Notification received:
     Event:      insert
     Collection: ws_subscribe_example_py
     Record IDs: CFCb7jAhU78eKhfIePi7Bh5UItdUvJAfBJ1aF-zWOZISw9P643zYokD428_XPR9E4-Dv2MBGb4-2WTNN5z0WJg
     Timestamp:  2026-10-20T20:02:56.813309+00:00

Inserting record 2...
✓ Inserted: q4FQsn_zUW2QEnG2ldwu8U9GeJmxUqgZQOmy-USIFesPopj4Fs6fxeyHTl47n8NVjZE4kYMDIpyoSrAik_NMIg

  📡 Notification received:
     Event:      insert
     Record IDs: q4FQsn_zUW2QEnG2ldwu8U9GeJmxUqgZQOmy-USIFesPopj4Fs6fxeyHTl47n8NVjZE4kYMDIpyoSrAik_NMIg

=== Unsubscribing ===
✓ Unsubscribed: {'collection': 'ws_subscribe_example_py', 'found': True, 'unsubscribed': True}

=== Cleanup ===
✓ Deleted collection 'ws_subscribe_example_py'

✓ WebSocket subscription example completed successfully
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: gAvAV_7yX5lBcsELlC9-s3NyG3VanpU8sf7ZiSNFJg48EK9QK8j3cxcf2pj8D2VALQ-0tC7rVd9chVieS0sM1A

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 5 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
=== Bypass Ripple Example ===

1. Basic insert (ripple enabled):
   Inserted with ripple: {'id': '9lDrgF3M7zBwrUW1z7YG5JVDvrmg_AknFVt29vdPRvbqjaTyMkOG0i6id4AIEW9KdOCxIkAaOPXAJApRNjERzw'}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {'id': 'oMRUsHmQuiTRyo-ceG4plIW2kPy0wfXOZ8tI4tbgWL9gU7DpYY4iRe1-FlfWXv_0hnr7e7g33UyXGwLj5LW44g'}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {'name': {'value': 'Product 1', 'type': 'String'}, 'id': '9lDrgF3M7zBwrUW1z7YG5JVDvrmg_AknFVt29vdPRvbqjaTyMkOG0i6id4AIEW9KdOCxIkAaOPXAJApRNjERzw', 'price': {'type': 'Integer', 'value': 150}}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {'price': {'value': 500, 'type': 'Integer'}, 'id': '9lDrgF3M7zBwrUW1z7YG5JVDvrmg_AknFVt29vdPRvbqjaTyMkOG0i6id4AIEW9KdOCxIkAaOPXAJApRNjERzw', 'name': {'type': 'String', 'value': 'Upsert Product'}}

✅ All bypass_ripple operations completed successfully!
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields (id, name, email only)
  Found 3 active users
  Fields returned: ['id', 'name', 'email']
  First user: {'type': 'String', 'value': 'Dave Brown'} <{'value': 'dave@example.com', 'type': 'String'}>

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ['id', 'bio', 'age', 'user_role', 'avatar_url', 'name', 'status', 'created_at', 'email']

Example 3: Complex query with projection (active users, ages 18-65)
  Found 3 active users (ages 18-65)
    - {'type': 'String', 'value': 'Dave Brown'} (age {'type': 'Integer', 'value': 45})
    - {'type': 'String', 'value': 'Alice Johnson'} (age {'value': 30, 'type': 'Integer'})
    - {'type': 'String', 'value': 'Bob Smith'} (age {'value': 25, 'type': 'Integer'})

Example 4: Query inactive users with profile fields
  Found 1 inactive users
    - {'value': 'Carol White', 'type': 'String'}: {'value': 'Manager', 'type': 'String'}

Example 5: Compare full vs projected data
  Full query:
    - 12 fields per record
    - Fields: ['id', 'created_at', 'user_role', 'api_key', 'bio', 'email', 'name', 'password', 'age', 'status', 'avatar_url', 'secret_token']
  Projected query:
    - 3 fields per record
    - Fields: ['id', 'email', 'name']
  Bandwidth savings: ~75% fewer fields

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
