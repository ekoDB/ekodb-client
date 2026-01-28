make test-examples-javascript
🧪 [36mRunning JavaScript examples (direct HTTP/WebSocket)...[0m

added 1 package, removed 1 package, and audited 9 packages in 807ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
[34m
╔════════════════════════════════════════╗[0m
[34m║  ekoDB JavaScript Examples Test Suite ║[0m
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
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/simple_crud.js ===[0m
=== Simple CRUD Operations (Direct HTTP) ===

✓ Authentication successful

=== Insert Document ===
Inserted: {
  id: 'CVXcyK96Uy1dlKSX6eiYZOk-g2jhX4FFapv06Z_nRUf0UevaQ0mJNLH79OEiSMXHsocyCYEQT52c7GdczhRf_w'
}

=== Find by ID ===
Found: {
  value: 42,
  id: 'CVXcyK96Uy1dlKSX6eiYZOk-g2jhX4FFapv06Z_nRUf0UevaQ0mJNLH79OEiSMXHsocyCYEQT52c7GdczhRf_w',
  active: true,
  name: 'Test Record'
}

=== Find with Query ===
Found documents: [
  {
    active: { value: true, type: 'Boolean' },
    value: { type: 'Integer', value: 42 },
    name: { type: 'String', value: 'Test Record' },
    id: 'CVXcyK96Uy1dlKSX6eiYZOk-g2jhX4FFapv06Z_nRUf0UevaQ0mJNLH79OEiSMXHsocyCYEQT52c7GdczhRf_w'
  }
]

=== Update Document ===
Updated: {
  value: { type: 'Integer', value: 100 },
  id: 'CVXcyK96Uy1dlKSX6eiYZOk-g2jhX4FFapv06Z_nRUf0UevaQ0mJNLH79OEiSMXHsocyCYEQT52c7GdczhRf_w',
  name: { type: 'String', value: 'Updated Record' },
  active: { value: true, type: 'Boolean' }
}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
[32m✓ simple_crud.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/simple_websocket.js ===[0m
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: 87SuZzUx0mmvYVW5f56pKwHPRp50DUJzfIsoSDPNroVRvpnc-drDiu03yh9E6y9kUvUCEBgS0o_1q6wU51HwmQ

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
        "id": "_hwl46HALMS28RzJD93jwOtMeWUYANFIwzG5gSnz1rqwWezvbHOgeMS3LMVgSh3lKcXnZdihlCqDmX0udb80oA",
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
        "id": "0LJADB61uaAeguYwN2eepp9qhGcqs5GjvarSJQCE3epOh5jnZ1-_6iMblhDP7snnYDeYGoTNJuOLfjFj3G1CFA",
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
        "id": "87SuZzUx0mmvYVW5f56pKwHPRp50DUJzfIsoSDPNroVRvpnc-drDiu03yh9E6y9kUvUCEBgS0o_1q6wU51HwmQ",
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
        "id": "oJm5UDTCfxwFeKzYk0FPOeqI3I76oqgMzD9mKIGAVecxCXdowuT5fEPyBstw_M5Ym1RAJ-tWx-mzm-V2sens6A",
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
✓ Retrieved 4 record(s) via WebSocket

✓ WebSocket example completed successfully
WebSocket closed
[32m✓ simple_websocket.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/http_functions.js ===[0m
🚀 ekoDB Scripts Example (JavaScript/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: 3XBg-_bISV85ngewfsvbeAgTyLQeHYbE_5edPKWopcwk28pq8NrfF93OUDRQOvo3r6eqa-G1_k8U1FjJSvJNqg
📊 Found 30 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: YX-KkPH3e5IbTp-AYJJcboO6wW1monieZIggqncLYRDbFBtXHW5a_tFzFLvM_pUMWfM1eZfnpBOyfce61x8ucg
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: ojU41Op_InvfZj3einEN-tWYFqpwmB-BWb_dUOfYKq5mTslHRTPKgxlrf8d2KfPG9_l0prfk1z9ZNUYF3blCDw
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"count":10,"avg_score":60,"max_score":100,"status":"active"}
   {"max_score":90,"count":10,"status":"inactive","avg_score":50}

📝 Example 4: Function Management

📋 Total scripts: 52
🔍 Retrieved script: Get Active Users
✏️  Script updated
🗑️  Script deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

✅ All examples completed!
[32m✓ http_functions.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/batch_operations.js ===[0m
=== Batch Operations (Direct HTTP) ===

✓ Authentication successful

=== Batch Insert ===
✓ Batch inserted 5 records
✓ Verified: Found 20 total records in collection

=== Creating test records for update/delete ===
Created 3 test records

=== Batch Update ===
✓ Batch updated 3 records
✓ Verified: Record updated with status="active"

=== Batch Delete ===
✓ Batch deleted 3 records
✓ Verified: Records successfully deleted (not found)

✓ All batch operations completed successfully
[32m✓ batch_operations.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/kv_operations.js ===[0m
=== Key-Value Operations (Direct HTTP) ===

✓ Authentication successful

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: { userId: 123, username: 'john_doe' }

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: { price: 29.99, name: 'Product 1' }
cache:product:2: { price: 39.989999999999995, name: 'Product 2' }
cache:product:3: { price: 49.989999999999995, name: 'Product 3' }

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
[32m✓ kv_operations.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/collection_management.js ===[0m
=== Collection Management (Direct HTTP) ===

✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: 8P9CX6X9bvHmGHoV_KtK6ruS_9JE6_ukjWZom7UYnS4zjxNdEMX-hUQGwwxcL7_fNPV7kVceK8bhrb8Tcfg53w

=== List Collections ===
Total collections: 24
Sample collections: [
  'schema_employees_client_go',
  'test_accounts',
  'schema_documents_client_py',
  'schema_documents_client_ts',
  'schema_products_client_py'
]

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
[32m✓ collection_management.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/transactions.js ===[0m
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: dqSfG8NEalTbsopLB6BRC5iKC5UXrFiTd4mz38cJd4j4V-2o08N2IND17bCDsTtum40GQ2fS_MsEy8R_M1JM-A
Created Bob: $500 - ID: 2V49wEYIyuoEaa12nK3QPz9Ye6ndZfP9D9GRWXYKi4MNxodY9QNeObwVqIw1bSE0kzGnUJlhSfKh2ovBu73n8g

=== Example 1: Begin Transaction ===
Transaction ID: 8ef6f594-34fa-4220-a6d8-951764cee306

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: 800
Bob: 700

=== Example 5: Rollback ===
New transaction: bdca5d48-fa5a-4961-b78f-0d1fe1d6c876
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: 700

=== Cleanup ===
✓ Deleted test accounts

✓ All transaction examples completed
[32m✓ transactions.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/crud_scripts.js ===[0m
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
   ✅ Script saved: h8hNC4m8O8hT6S4zn_uHPFCrRBOfyLx6mFtkVDj0UFN3-ReSMKToJnuEPVrntdHBTxQPBR96pUDpKTywokmdqg

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Name: [object Object]
   📋 Email: {"type":"String","value":"alice@example.com"}
   📋 Status: [object Object]
   📋 Credits: [object Object]

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: E1JTqnEM5-L3F7GYu299u7X7MbpMf0zA-23kWuqaLLb-L0dAxMhOZfhXn7honOOv_B6DeqJuyRIozQhA-57HIg

2️⃣ Calling Script (Query + Update + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Status updated to: {"type":"String","value":"active"}
   📋 Name: {"value":"Alice Smith","type":"String"}

============================================================
📝 Script 3: Query + Update Credits + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: 4A52pyxqCABr_2rWC_0kHGDGp2H7olSESgqhuZptC8lmzYnjHKp8Mja6cGP90xSnzDYkAlU_LJIpsnoTDu3hJg

2️⃣ Calling Script (Query + Update Credits + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {"type":"Integer","value":0}
   📋 Status: {"type":"String","value":"active"}
   📋 Name: {"value":"Alice Smith","type":"String"}

============================================================
📝 Script 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: wc_ti7EQqQGrtrRqRWUurFNU-USkMpEydSSe89Yg_PRF8UUFzqfNlgMcYzWmIvucmr_l9p4VADvX7Hr5AR0LLg

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: h8hNC4m8O8hT6S4zn_uH...
   ✅ Deleted script: E1JTqnEM5-L3F7GYu299...
   ✅ Deleted script: 4A52pyxqCABr_2rWC_0k...
   ✅ Deleted script: wc_ti7EQqQGrtrRqRWUu...
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
[32m✓ crud_scripts.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/document_ttl.js ===[0m
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
  Output: Document ID = i3wgLW2VB1QKcjg5Zg106CUq0HVurjPqL-hjXRKi39_GlCdBBiSqyCmQEgeLNRjV0pAHN_Q2oESfGrWvFIiBNA
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: findById(i3wgLW2VB1QKcjg5Zg106CUq0HVurjPqL-hjXRKi39_GlCdBBiSqyCmQEgeLNRjV0pAHN_Q2oESfGrWvFIiBNA)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: findById(i3wgLW2VB1QKcjg5Zg106CUq0HVurjPqL-hjXRKi39_GlCdBBiSqyCmQEgeLNRjV0pAHN_Q2oESfGrWvFIiBNA)
  Output: Error (expected) - Request failed with status 404: {"error":"Record not found (expired)"}
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
[32m✓ document_ttl.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/websocket_ttl.js ===[0m
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
  Output: Document ID = B-_kBofFuc8sRYg1yK-Bd8ngKvjakwky1HZRoNw7tb8vBE1LXXN7P9kER565oPmFUgjKtuQPV-Qxlhcxh-7aIw
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: findById(B-_kBofFuc8sRYg1yK-Bd8ngKvjakwky1HZRoNw7tb8vBE1LXXN7P9kER565oPmFUgjKtuQPV-Qxlhcxh-7aIw)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: findById(B-_kBofFuc8sRYg1yK-Bd8ngKvjakwky1HZRoNw7tb8vBE1LXXN7P9kER565oPmFUgjKtuQPV-Qxlhcxh-7aIw)
  Output: Error (expected) - Request failed with status 404: {"error":"Record has been deleted"}
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
  • Queries correctly return null for expired documents
[32m✓ websocket_ttl.js completed successfully[0m
[34m
╔════════════════════════════════════════╗[0m
[34m║           Test Summary                 ║[0m
[34m╚════════════════════════════════════════╝[0m
[34mTotal: 10[0m
[32mPassed: 10[0m
[32mFailed: 0[0m
✅ [32mJavaScript direct examples complete![0m
📦 [36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.9.0 prepare
> npm run build


> @ekodb/ekodb-client@0.9.0 build
> tsc


up to date, audited 58 packages in 1s

15 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.9.0 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning JavaScript client library examples...[0m

added 1 package, removed 1 package, and audited 9 packages in 821ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'dUwD7_wotwjVMWyeQPVK4x8dSuXe_yN5zFM7RR7V4LNH6QF93lEOnA1aBaeSNQm9vv9_AXLY30lrcE_BD8zQcQ'
}

=== Find by ID ===
Found: {
  active: true,
  name: 'Test Record',
  value: 42,
  id: 'dUwD7_wotwjVMWyeQPVK4x8dSuXe_yN5zFM7RR7V4LNH6QF93lEOnA1aBaeSNQm9vv9_AXLY30lrcE_BD8zQcQ'
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  active: { type: 'Boolean', value: true },
  name: { value: 'Updated Record', type: 'String' },
  value: { value: 100, type: 'Integer' },
  id: 'dUwD7_wotwjVMWyeQPVK4x8dSuXe_yN5zFM7RR7V4LNH6QF93lEOnA1aBaeSNQm9vv9_AXLY30lrcE_BD8zQcQ'
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: zFPmiASRULSLPofs0xGfF8fOU8QEwfOdTZVSx9IDxmAt8RsNfbWDWxeHjMMV3tzUqX56yLp-zf8dkOzA5Gv92g

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
Collection created with first record: svo0tNYyBgZivZ35UXbuWWIuI7xi8ikSuq8B-jowxgD3wXPubU3RWSvl_Y6e_VHm5_qaic9oGKmExEVzhvCC3Q

=== List Collections ===
Total collections: 23
Sample collections: schema_employees_client_go,test_accounts,schema_documents_client_py,schema_documents_client_ts,schema_products_client_py

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: { username: 'john_doe', userId: 123 }

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: { price: 29.99, name: 'Product 1' }
cache:product:2: { price: 39.989999999999995, name: 'Product 2' }
cache:product:3: { name: 'Product 3', price: 49.989999999999995 }

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 0 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 17

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: false

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: Be1pFt441bXENn9DrktTeW6Brlp4MhvS-1Gcg9eN_UvkQJbfJtDWYTTieRdTdUpyG_x5CJq0ddGYv8-2tOktag
Created Bob: $500 - ID: g7E-BaPVBdINMOe3ytzWBgwwo3oQo153u4QKc_WLbbKuvzWY16CYEPB0cD8BW_vV-diwO7-ce3hPY9by9U5jHA

=== Example 1: Begin Transaction ===
Transaction ID: fcb4cced-d8fb-40b2-8ff7-163d3f6fd4d4

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: d029268a-633b-41b8-8552-75828e4a4bd6
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
Found 3 valid users

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
  2. Score: 12.540, Matched: name.value, name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title, bio.value, title.value, bio
  2. Score: 26.400, Matched: title.value, bio, bio.value, title
  3. Score: 26.400, Matched: title, bio.value, bio, title.value
  4. Score: 26.400, Matched: title, bio, bio.value, title.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio, title, title.value, bio.value
  2. Score: 39.600, Matched: title, title.value, bio.value, bio
  3. Score: 39.600, Matched: title, title.value, bio.value, bio
  4. Score: 39.600, Matched: bio.value, title, title.value, bio

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio.value, bio

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.792, Matched: 
  2. Score: 0.751, Matched: 
  3. Score: 0.749, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.700, Matched: content.value, title.value, title, content
  2. Score: 1.500, Matched: title.value, content, title, content.value
  3. Score: 0.317, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: skills.value, bio.value, skills, bio

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
Schema fields: [ 'age', 'email', 'name', 'status' ]
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
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: DO8l-aBB0mXBwxsM0H2G4smR9ZD_mJ1MveFI2zIN19FyjmT1GpthzBlUBBWo_Zx5a6HVzK9xvcSSiojKbQdS0A

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: _ZXrUXa51BNxg3hdoYpl7BsKBpxaNIFBiJ0DYzcwe1bo3SH9iYm11jOM7Et-C5T3A-YTIxiB2qBetxAzVo8Q5w

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
✓ Inserted document with TTL: AnbWjhLLAenJkRBPzXTWykx5X42vnp3WpaivTRhCgR_xvfDdmW9FPVGuKLIxuOvyaiN1FoiECZWS-zi8Ptr0wA

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
=== ekoDB as Edge Cache - Simple Example ===

Creating edge cache function...
✓ Edge cache script created: 6ZOcTZNh-FCM1eu2tfuKwmxc6XNQsMtf9JjFV4lujzv1r-nhz15I0rF2RyJbkfpa0jGPJrUrEBCPnwjyKZWA2A

Call 1: Cache miss (fetches from API)
Response time: 4ms
Result: {
  "records": [
    {
      "value": {
        "generationtime_ms": 0.027298927307128903,
        "current_units": {
          "time": "iso8601",
          "interval": "seconds",
          "temperature_2m": "°C"
        },
        "latitude": 40.710335,
        "timezone": "GMT",
        "elevation": 32,
        "longitude": -73.99308,
        "utc_offset_seconds": 0,
        "timezone_abbreviation": "GMT",
        "current": {
          "interval": 900,
          "temperature_2m": -10.7,
          "time": "2026-01-28T00:00"
        }
      }
    }
  ],
  "stats": {
    "input_count": 0,
    "output_count": 1,
    "execution_time_ms": 0,
    "stages_executed": 2,
    "stage_stats": []
  }
}

Call 2: Cache hit (served from ekoDB)
Response time: 4ms (1x faster!)
Result: {
  "records": [
    {
      "value": {
        "generationtime_ms": 0.027298927307128903,
        "current_units": {
          "time": "iso8601",
          "interval": "seconds",
          "temperature_2m": "°C"
        },
        "latitude": 40.710335,
        "timezone": "GMT",
        "elevation": 32,
        "longitude": -73.99308,
        "utc_offset_seconds": 0,
        "timezone_abbreviation": "GMT",
        "current": {
          "interval": 900,
          "temperature_2m": -10.7,
          "time": "2026-01-28T00:00"
        }
      }
    }
  ],
  "stats": {
    "input_count": 0,
    "output_count": 1,
    "execution_time_ms": 0,
    "stages_executed": 2,
    "stage_stats": []
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

🚀 ekoDB Scripts Example (JavaScript Client)

✅ Client initialized (token exchange automatic)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: 9sJm3P-G5jfnr1dO-fjFtGcR9wougk_eaREDQ8LPeSjPDotZxla0vlcHoWmYTrD9_R-j0YjC9EeXTw6qU7BVBw
📊 Found 10 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved
📊 Found 10 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved
📊 Statistics: 2 groups
   {"count":5,"avg_score":50,"status":"inactive"}
   {"avg_score":60,"status":"active","count":5}
⏱️  Execution time: 0ms

📝 Example 4: Script Management

📋 Total scripts: 55
🔍 Retrieved script: Get Active Users
✏️  Script updated
🗑️  Script deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

📝 Example 5: Multi-Stage Pipeline

✅ Multi-stage script saved
📊 Pipeline executed 2 stages
⏱️  Total execution time: 0ms
📈 Stage breakdown:

📝 Example 6: Count Users

✅ Count script saved
📊 Total user count: 10
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Deleted collection
✅ Deleted test scripts

✅ All examples completed successfully!

💡 Key Advantages of Using the Client:
   • Automatic token management
   • Type-safe Stage builders
   • ChatMessage helpers
   • Cleaner, more maintainable code
   • Built-in error handling
=== ekoDB Function Composition Examples ===

📋 Setting up test data...

✅ Test data ready

📝 Example 1: Basic Function Composition

Building reusable functions that call each other...

✅ Saved reusable function: fetch_user
✅ Saved composed function: get_user_wrapper (calls fetch_user + projects fields)

📊 Result from composed function:
   Records: 1
   Name: {"type":"String","value":"User 1"}
   Department: {"value":"engineering","type":"String"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 4ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "name": "Leanne Graham",
    "username": "Bret",
    "company": {
      "catchPhrase": "Multi-layered client-server neural-net",
      "name": "Romaguera-Crona",
      "bs": "harnes...

Second call (cache hit - from cache):
   ⏱️  Duration: 4ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "name": "Leanne Graham",
    "username": "Bret",
    "company": {
      "catchPhrase": "Multi-layered client-server neural-net",
      "name": "Romaguera-Crona",
      "bs": "harnes...
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
🚀 ekoDB JavaScript Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved: WZPYRylVludeQU2q9L-n-WAOy2n-_t2bD06me4RNncQHRPlLN2efphoDJoClNaugSzSyWcUcoKuYm1yhsE4qpA
📊 Found 2 product groups
   {"category":"Furniture","count":2,"avg_price":474}
   {"avg_price":575.6666666666666,"count":3,"category":"Electronics"}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved
📊 Found 2 categories
   {"count":3,"category":"Electronics"}
   {"category":"Furniture","count":2}
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
🚀 ekoDB JavaScript KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: RxmnzE6MQVmiEyJLrwof0pA7G106TckOtA6HWnjA995PqRNpy0yatEIyoZ2jpWFqP5SwBEg3qjjXMBBOU0BuoQ
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: H0SoGATFyymFNcWzEpQgsN2uTcQbPzE9eDf93hTbFRue6eZoxwIOW_6wwyLv9R4KjurTMJa40F_3uRPIW8hgiQ
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"role":"admin","userId":"user_abc"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: 1MN7Ugeg3aT5NB9Znov4oop8NmRziCe3XLv57ucHTd9Q3TKWxGvvn0_7j8A-9EREON-2PPJZ8cdZrpyTM-P4Eg
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Script

✅ Script saved: 5G4DIa64n_SmGjUtaE2YCg4wR1ZEbLJYlMbc-yIOOUU5do_kqR-wMj7HQL34xnTAo2Yjr-HILH2tO55xm9_Pqg
📊 Processed order with caching
⏱️  Stages executed: 3
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All KV & Wrapped Types examples completed!

💡 Key takeaways:
   ✅ Use Field.* helpers for type-safe wrapped values
   ✅ Field.decimal() preserves precision (no floating point errors)
   ✅ KV store is great for caching and quick lookups
   ✅ Stage.kv*() functions work within scripts
   ✅ Combine KV caching with collection inserts for real workflows
🚀 ekoDB CRUD Scripts Example

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
📊 Average scores by role:
   admin: 20.0 (3 users)
   user: 70.0 (7 users)
⏱️  Execution time: 0ms

📝 Example 4: Top Users by Score

✅ Script saved
📊 Users (showing first 5 of 5):
   1. User 3 - Score: 60
   2. User 5 - Score: 100
   3. User 1 - Score: 20
   4. User 2 - Score: 40
   5. User 4 - Score: 80
⏱️  Execution time: 0ms

📝 Example 5: User Summary Statistics

✅ Script saved
📊 User summary (2 groups):
   active users: 7 users, Total Score: 370
   inactive users: 3 users, Total Score: 180
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD script examples completed!
🚀 ekoDB Search Scripts Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Script saved
📊 Found 5 documents
   1. Introduction to Machine Learning (AI)
   2. Natural Language Processing (AI)
   3. Database Design Principles (Database)
   4. Getting Started with ekoDB (Database)
   5. Vector Databases Explained (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Script saved
📊 Documents by category:
   AI: 2 documents
   Database: 3 documents
⏱️  Execution time: 0ms

📝 Example 3: Select Specific Fields

✅ Script saved
📊 Document titles (5 docs):
   1. Introduction to Machine Learning
   2. Natural Language Processing
   3. Database Design Principles
   4. Getting Started with ekoDB
   5. Vector Databases Explained
⏱️  Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Script saved
📊 Projected documents (showing first 3):
   1. Introduction to Machine Learning
   2. Natural Language Processing
   3. Database Design Principles
⏱️  Execution time: 0ms

📝 Example 5: All Document Fields

✅ Script saved
📊 All documents (5 total, showing first 2):
   1. Introduction to Machine Learning (AI)
   2. Natural Language Processing (AI)
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples completed!
🚀 ekoDB Advanced Scripts Example

📋 Setting up test data...
✅ Created 10 products

📝 Example 1: List All Products

✅ Script saved
📊 Found 10 products
⏱️  Execution time: 0ms

📝 Example 2: Group Products by Category

✅ Script saved
📊 Found 2 categories
   Electronics: 6 items (avg $325.67)
   Furniture: 4 items (avg $294.00)
⏱️  Execution time: 0ms

📝 Example 3: Count All Products

✅ Script saved
📊 Total products: 10
⏱️  Execution time: 0ms

📝 Example 4: Multi-Stage Aggregation

✅ Script saved
📊 Category analysis (2 categories):
   Furniture:
      Products: 4 | Stock: 43 | Avg Rating: ⭐4.26
   Electronics:
      Products: 6 | Stock: 232 | Avg Rating: ⭐4.52

⏱️  Total execution time: 0ms
📈 Pipeline stages:

📝 Example 5: Project Specific Fields

✅ Script saved
📊 Product summaries (10 items, showing first 3):
   1. Webcam HD - $119 (⭐4.5)
   2. USB-C Cable - $19 (⭐4.3)
   3. Laptop Pro - $1299 (⭐4.8)
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced script examples completed!
🚀 ekoDB AI Scripts Example

⚠️  Note: These examples require AI API credentials (OpenAI, etc.)

📋 Setting up test data...
✅ Created 3 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
🤖 AI Response:
   Vector databases offer several benefits including:

1. Precision: They maintain the accuracy of data as they use points, lines, and polygons to represent real-world features.

2. Scalability: Objects within a vector database can be scaled up or down without losing their original shape or details.

3. Advanced analysis: They allow for complex spatial and topological analysis, such as determining proximity and routing.

4. Less storage: They usually require less storage space compared to raster databases.

5. Flexibility: The data within vector databases can be easily manipulated and customized according to user requirements.
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embedding script saved
📊 Generated embeddings for 3 articles
   1. "Getting Started with ekoDB" - 1536D vector
   2. "Advanced Query Patterns" - 1536D vector
   3. "Draft Article" - 1536D vector
⏱️  Execution time: 0ms

📝 Example 3: List All Articles

✅ Script saved
📊 Found 3 articles
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All AI examples completed!
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_api_user_js (GY74usH-qGGQbZ4AgJ3GXa_7g1__mrA3u3XHNRfUMqefRXln8gsFZ0cBlj6CcWY1WxrwY8aJqpgAaL0D1HhuTg)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "address": {
          "suite": "Apt. 556",
          "zipcode": "92998-3874",
          "geo": {
            "lat": "-37.3159",
            "lng": "81.1496"
          },
          "city": "Gwenborough",
          "street": "Kulas Light"
        },
        "phone": "1-770-736-8031 x56442",
        "email": "Sincere@april.biz",
        "website": "hildegard.org",
        "id": 1,
        "name": "Leanne Graham",
        "company": {
          "bs": "harness real-time e-markets",
          "catchPhrase": "Multi-layered client-server neural-net",
          "name": "Romaguera-Crona"
        },
        "username": "Bret"
      }
    }
  ],
  "stats": {
    "input_count": 0,
    "output_count": 1,
    "execution_time_ms": 0,
    "stages_executed": 2,
    "stage_stats": []
  }
}
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 4ms (served from cache)
✓ Lightning fast cache hit

🧹 Cleaning up...
✓ Cleanup complete

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation
🚀 ekoDB JavaScript Client - Native SWR Function Examples

📋 Demonstrates:
   • Single-function SWR pattern (replaces 4-step pipeline)
   • Automatic cache checking, HTTP fetching, and cache setting
   • Built-in audit trail support
   • Duration string TTLs ('15m', '1h', '30s')
   • Multi-function pipeline integration
   • Dynamic TTL configuration


Example 1: Basic Native SWR
────────────────────────────────────────────────────────────────────────────────
Single function replaces KvGet → If → HttpRequest → KvSet pipeline
✓ Created native SWR script: github_user_native (m1hCn9_mBKOwNfEqaROUqK542XWmA2xpVCiB7FzNXaAU0ebm3egdUD63J9OJG8GepGL-XiFdvs-UinczC0ou7Q)

First call (cache miss - will fetch from GitHub API):
  Response time: 4ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 5ms
  Speedup: 0.8x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (rD5t7MGmR0Nxo45dLXlA2qRkhTR1_L8ETeeQ74mbsoTAh4krA7oK4_CPHfllngSkkAYu_sKetWSsFXKpzn3D6g)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (BiKSOmSivTpheuLRrWuHfroDtjd8nH0_5earI-_lDAjF4YO2vUIzl_nEM_AW_unY3dDuz8WvYOReKLs7XuCM7A)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (rEy02tLwrM30DOHgUY1Z_uPtJg47hPNHun0SA3GL3Qe3d6rPH4p1VSX-f0yrBfdHHtTMhK3F6W5bhl2A4WUpMg)
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
✓ Deleted 4 test scripts

✅ All examples completed!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: Sfa8qiiHqWQ5I_597vIVrvZ53lxKXKQdspVTljCbOZDckdQUv3wnhkEBzzDfyjltpi5kEtkkCChH6B7POPlrHQ

=== Sending Chat Message ===
Message ID: c8ebWZ_3L4sXMkwCuXxI1JhTXRuw9O9klrHpVxY2RxGropYE4XX3U2cQebMirDz2Qv6mv--gfyNRXE-GA_I7XA

=== AI Response ===
There are three products available:

1. ekoDB Pro: This is an Enterprise edition product with advanced features. The price for this product is $299.
2. ekoDB Cloud: This is a fully managed cloud database service product. The price for this product is $499.
3. ekoDB: This is a high-performance database product with AI capabilities. The price for this product is $99.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_js',
  record: {
    name: 'ekoDB Pro',
    price: 299,
    id: 'FwavYG1wepVF3QCgCUY7RLuyGApKGBP7W5qvW_43sdLMLzukOHz_bDm6X5ISYsPuZBpufcvbcGaMri0bWfZLow',
    description: 'Enterprise edition product with advanced features'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_js',
  record: {
    price: 499,
    id: 'f7zNmFtt9Hl7ujy8hK1GLEpIfrpikXHobpaZDjnsemZ_be7ax0FDpZUDQteevjGcgVKOFGuETHhU3NIFVQbT3A',
    description: 'Fully managed cloud database service product',
    name: 'ekoDB Cloud'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_js',
  record: {
    name: 'ekoDB',
    description: 'A high-performance database product with AI capabilities',
    id: 'Xg09D4EW14ZiwluZ6PxPcXx2ZvwBtjtt82hWmQSf7evHD5DWBz7A-WEuu3X1F7G9ST-D9o_U-M4g1FTSDZj1wA',
    price: 99
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 3348ms

=== Token Usage ===
Prompt tokens: 451
Completion tokens: 84
Total tokens: 535

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: hnfDosynXBJ2ZhAJT2zFpt4c3pjfA_v7tJjg0_H0pejbC2wh9MaRoid3GZKX3pQZY9j7TSR3DuPuIT3LpiUDOA

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information provided, there is a high-performance database product available named "ekoDB". It is priced at $99.

✓ Second message sent

=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of ekoDB is $99.

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: HWrg3vD-bIQs0W4DGV-xdnZOsk809jzsT6kvCjXg8w82_kOFPwt50-RAdyyR3Gp0-p4xswXn5m9tX702IgHTnA
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
✓ Created session: EKx3pP9paNBuhc3b8Izq2TA_sTP_nHWkivxY3JQEW-h6FnSgXRoIxRJAk-_EgG6YkRPISSxV-5ZB7YMAVyjbzw

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information available, one product is ekoDB. This is a high-performance database product available for the price of $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: QA2oIri0a-I8cFtK-mZ4lA__zO3Uj_wXgNYhab9lK54342gr6SKEkO0W63AdPR0d928rzM0FCvCMKcL7gYL4yw
  Parent: EKx3pP9paNBuhc3b8Izq2TA_sTP_nHWkivxY3JQEW-h6FnSgXRoIxRJAk-_EgG6YkRPISSxV-5ZB7YMAVyjbzw

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: QA2oIri0a-I8cFtK-mZ4lA__zO3Uj_wXgNYhab9lK54342gr6SKEkO0W63AdPR0d928rzM0FCvCMKcL7gYL4yw (Untitled)
  Session 2: EKx3pP9paNBuhc3b8Izq2TA_sTP_nHWkivxY3JQEW-h6FnSgXRoIxRJAk-_EgG6YkRPISSxV-5ZB7YMAVyjbzw (Untitled)
  Session 3: HWrg3vD-bIQs0W4DGV-xdnZOsk809jzsT6kvCjXg8w82_kOFPwt50-RAdyyR3Gp0-p4xswXn5m9tX702IgHTnA (Untitled)
  Session 4: Sfa8qiiHqWQ5I_597vIVrvZ53lxKXKQdspVTljCbOZDckdQUv3wnhkEBzzDfyjltpi5kEtkkCChH6B7POPlrHQ (Untitled)
  Session 5: P_aaJN68XhRIbhqfgQ_jSrjyRJVoyUUykicgdR0eXORz9SYRtDEcPdyMkgNNqEGMPJ6Z-2-jUFnlI589d2aHmQ (Untitled)
  Session 6: PrDZapYRBDXb0krYAwkAKnXfZUOSgqcgU7tqRVf9dNiJcoo_7VaT41GdHKL3je6SgvKQEitPVy93IY0fiY03cg (Untitled)
  Session 7: 2WR0InFC-qeXX9VK4maTnsaT0VQ9MZMJjG-GYITX1ZZy6xBtyDrI785FLoDXPUOMUL872PUCs5UDnbBZ0Z3uNQ (Untitled)
  Session 8: GWN7n_LCJc5sI8y74tHllA5-oBLAoo9mdo1YYk47xcv89lSXIaEmVmie9ZzHja53uGONrfdAsiQZICuwn5wOhw (Untitled)
  Session 9: _XwItNsgyui8eFwQdJ6vlZ3UIvBQDyuKd_-KbvxG5BgvgtF-S9jyqIDFa40sGXoYzxX4EjtHa9ev55NghF-r5g (Untitled)
  Session 10: 5HlYZzepTIGvK8nKfiHDDimKGkZe_gVOvDe1xEdj-yE_eDBZBijc4ne1Q4SstWoYaWF52WMpcUodRD87Ef9tLw (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: QA2oIri0a-I8cFtK-mZ4lA__zO3Uj_wXgNYhab9lK54342gr6SKEkO0W63AdPR0d928rzM0FCvCMKcL7gYL4yw

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
✅ [32mJavaScript client examples complete![0m
