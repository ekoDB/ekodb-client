make test-examples-go
🧪 [36mRunning Go examples (direct HTTP/WebSocket)...[0m
[34m
╔════════════════════════════════════════╗[0m
[34m║     ekoDB Go Examples Test Suite      ║[0m
[34m╚════════════════════════════════════════╝[0m
[34m
=== Checking Server Connection ===[0m
[32m✓ Server is ready[0m
[33m
=== Running 10 Examples ===[0m
[34m
=== Running simple_crud.go ===[0m
=== Simple CRUD Operations (Direct HTTP) ===

✓ Authentication successful

=== Insert Document ===
Inserted: map[id:P7VgiWRlk-2Gx9bVuZG1HwBMcRDnrzgOOqmfQDI0vZEmWwS4pVKdIkgt2xZLtLbyvJdMm-wSk_Nn0svrwKYJiA]

=== Find by ID ===
Found: map[active:true id:P7VgiWRlk-2Gx9bVuZG1HwBMcRDnrzgOOqmfQDI0vZEmWwS4pVKdIkgt2xZLtLbyvJdMm-wSk_Nn0svrwKYJiA name:Test Record value:42]

=== Find with Query ===
Found documents: map[]

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] id:P7VgiWRlk-2Gx9bVuZG1HwBMcRDnrzgOOqmfQDI0vZEmWwS4pVKdIkgt2xZLtLbyvJdMm-wSk_Nn0svrwKYJiA name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
[32m✓ simple_crud.go completed successfully[0m
[34m
=== Running simple_websocket.go ===[0m
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: GEv26dPSRPt4XxnEv67qCooubqCkgLUZqEpx5QsGzWhwRUF_PZGZzp29lz7Zn0D3Vf0Doykedto4X2Ei1hZe2w

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
        "id": "GEv26dPSRPt4XxnEv67qCooubqCkgLUZqEpx5QsGzWhwRUF_PZGZzp29lz7Zn0D3Vf0Doykedto4X2Ei1hZe2w",
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
✓ Retrieved 3 record(s) via WebSocket

✓ WebSocket example completed successfully
[32m✓ simple_websocket.go completed successfully[0m
[34m
=== Running http_functions.go ===[0m
🚀 ekoDB Scripts Example (Go/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: ejjPIOkBEKs-WTxpwPfFm7_C9A6dz3GctYuNNkA8NGkXr2uobFtTVsYuYlcBXhFMqFCa8emtBYBClRFxkZiMxA
📊 Found 25 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: X29w4ehZCT_KALrIXJ96ideHA-8VYh6WATwxidYPLdNEsVirFEbiIBXm19xL2bJkdiUQHI9T893n-ICxBhg3BA
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: 4lpoE1AWSsJ9wIRLmC8KP-WAbDXXJ8nype0yzycfGqifu6tyrVpWLuloedceJ3FqGScXZ3PaeS7qQccZy4sImA
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"avg_score":60,"count":10,"max_score":100,"status":"active"}
   {"avg_score":50,"count":10,"max_score":90,"status":"inactive"}

📝 Example 4: Function Management

📋 Total scripts: 27
🔍 Retrieved script: Get Active Users
✏️  Script updated
🗑️  Script deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

✅ All examples completed!
[32m✓ http_functions.go completed successfully[0m
[34m
=== Running batch_operations.go ===[0m
=== Batch Operations (Direct HTTP) ===

✓ Authentication successful

=== Batch Insert ===
✓ Batch inserted 5 records

=== Creating test records for update/delete ===
Created 3 test records

=== Batch Update ===
✓ Batch updated 3 records

=== Batch Delete ===
✓ Batch deleted 3 records
✓ Verified: Records successfully deleted (not found)

✓ All batch operations completed successfully
[32m✓ batch_operations.go completed successfully[0m
[34m
=== Running kv_operations.go ===[0m
=== Key-Value Operations (Direct HTTP) ===

✓ Authentication successful

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: map[userId:123 username:john_doe]

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: map[name:Product 1 price:29.99]
cache:product:2: map[name:Product 2 price:39.989999999999995]
cache:product:3: map[name:Product 3 price:49.989999999999995]

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
[32m✓ kv_operations.go completed successfully[0m
[34m
=== Running collection_management.go ===[0m
=== Collection Management (Direct HTTP) ===

✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: rwrliQXQAqaEt0JEF59YNhHQBUf64RFTR7G-eeU0IPBmHa4hrJqN_qQ3aShgPjL17O92WMfTdAR5oAj6zw9iHw

=== List Collections ===
Total collections: 16
Sample collections: [schema_products_client_py chat_messages__ek0_testing github_cache ws_ttl_test test_collection]

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
[32m✓ collection_management.go completed successfully[0m
[34m
=== Running transactions.go ===[0m
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: UGXoVSK5zbKzQf-56qSfwjnVGf0-Y5AaBDU61d_gc6qxwdGh7JmZUBa0FVmQ3S6DxPavmCUHvJkKDcxg5eF4IQ
Created Bob: $500 - ID: aQV2kVDNFIoaSHpE33NmnAcZFclNA60KnZhuBRRhC1lu3VWGGyDuByaA5EZeiL_MwfKqRdKcg-5pAWOyb-ONGA

=== Example 1: Begin Transaction ===
Transaction ID: d10e17a2-53c4-4812-9ed7-c663a7863d89

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
New transaction: dd091a9e-556f-4bd7-991d-4bd0d3537cb8
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: 700

=== Cleanup ===
✓ Deleted test accounts

✓ All transaction examples completed
[32m✓ transactions.go completed successfully[0m
[34m
=== Running crud_scripts.go ===[0m
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
   ✅ Script saved: IcLdGUWzJVbVgcWxR9NEFY7pkpdR6FqIXonfejxbbf0zXj-Dtb9Tpt5mCD-rqqR6fv8P0gd-Zb1JPYQYu_AO0Q

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: j4U8TSieG5mihJF5GKoUe5hSqck2uGDVayo1hJGdEtQXmjMoKaY2PGRCDUGCS60fOXceA1eg30iML1VQDow-LQ
   📋 Name: map[type:String value:Alice Smith]
   📋 Email: map[type:String value:alice@example.com]
   📋 Status: map[type:String value:pending]
   📋 Credits: map[type:Integer value:0]

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: ANt1llVfi21JG_9ytSbYjmF4gqZTfTgdRljalCFfzf6DiNjI9W47bnE7DjkRbMzTFXbsGOvPSjggGv3UvXU73A

2️⃣ Calling Script (Query + Update + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Status updated to: map[type:String value:active]
   📋 Name: map[type:String value:Alice Smith]

============================================================
📝 Script 3: Query + Update Credits + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: jETWc17CmVoEeEjgOZEYfwaa_aQ-apGk3UD3m3V0bx-E0phR6DkGYk76VgiDqajfzgVDa5o4hqRJOSgTQ1WrPQ

2️⃣ Calling Script (Query + Update Credits + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: map[type:Integer value:0]
   📋 Status: map[type:String value:active]
   📋 Name: map[type:String value:Alice Smith]

============================================================
📝 Script 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: 7eXra-z0E1pc8sy8OFgNsYMmUN2SmZ8dIQHTMUJmZMvoCthRLbm9SjaBLzrAwLxe3CyqsEXHsHeDplnS-JDKVg

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: IcLdGUWzJVbVgcWxR9NE...
   ✅ Deleted script: ANt1llVfi21JG_9ytSbY...
   ✅ Deleted script: jETWc17CmVoEeEjgOZEY...
   ✅ Deleted script: 7eXra-z0E1pc8sy8OFgN...
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
[32m✓ crud_scripts.go completed successfully[0m
[34m
=== Running document_ttl.go ===[0m
╔════════════════════════════════════════════════════════╗
║     TTL EXPIRATION VERIFICATION TEST                   ║
╚════════════════════════════════════════════════════════╝

This test verifies that document TTL expiration works correctly.
We will insert documents with short TTL and verify they expire.

2026/01/23 12:27:39 No .env file found, using defaults
✓ Client connected

═══════════════════════════════════════════════════════════
TEST 1: Document TTL Expiration
═══════════════════════════════════════════════════════════

[Step 1] Insert document with 3 second TTL
  Input: {name: 'TTL Test', value: 'should expire'}
  TTL: 3s
  Output: Document ID = PfM4Q7Eot1IaF8_NCMaISEYkNer7HKvuWM5RL7EqVhcO_oNM_5gNgmv8cLYZjDI_e1Osk6CHZMNuYX8Kk2Ub5w
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: FindByID(PfM4Q7Eot1IaF8_NCMaISEYkNer7HKvuWM5RL7EqVhcO_oNM_5gNgmv8cLYZjDI_e1Osk6CHZMNuYX8Kk2Ub5w)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: FindByID(PfM4Q7Eot1IaF8_NCMaISEYkNer7HKvuWM5RL7EqVhcO_oNM_5gNgmv8cLYZjDI_e1Osk6CHZMNuYX8Kk2Ub5w)
  Output: Error (expected) - request failed with status 404: ��Record not found (expired)
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
[32m✓ document_ttl.go completed successfully[0m
[34m
=== Running websocket_ttl.go ===[0m
╔════════════════════════════════════════════════════════╗
║   WEBSOCKET TTL EXPIRATION VERIFICATION TEST           ║
╚════════════════════════════════════════════════════════╝

This test verifies TTL expiration works via WebSocket connections.
We will use WebSocket to insert, query, and verify TTL expiration.

2026/01/23 12:27:47 No .env file found, using defaults
✓ Client connected

═══════════════════════════════════════════════════════════
TEST: WebSocket TTL Expiration
═══════════════════════════════════════════════════════════

[Step 1] Insert document with 3 second TTL
  Input: {name: 'WS TTL Test', value: 'should expire'}
  TTL: 3s
  Output: Document ID = WddD7oGdJ4JcLmQci4xGozEaMzvOVmszZmIZLwu-BqkA0E24RtYV027oTLvkymTNMKe_6O_e4hwfxq9DDg4rcQ
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: FindByID(WddD7oGdJ4JcLmQci4xGozEaMzvOVmszZmIZLwu-BqkA0E24RtYV027oTLvkymTNMKe_6O_e4hwfxq9DDg4rcQ)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: FindByID(WddD7oGdJ4JcLmQci4xGozEaMzvOVmszZmIZLwu-BqkA0E24RtYV027oTLvkymTNMKe_6O_e4hwfxq9DDg4rcQ)
  Output: Error (expected) - request failed with status 404: ��Record has been deleted
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
  • Queries correctly return nil for expired documents
[32m✓ websocket_ttl.go completed successfully[0m
[34m
╔════════════════════════════════════════╗[0m
[34m║           Test Summary                 ║[0m
[34m╚════════════════════════════════════════╝[0m
[34mTotal: 10[0m
[32mPassed: 10[0m
[32mFailed: 0[0m
✅ [32mGo direct examples complete![0m
🧪 [36mRunning Go client library examples...[0m
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: map[id:retr1TCNXs8OxHItWt0_abieIdnwSsAjxDah7836KvplXnYc-ILCh3wszOVwRhW59lGyGTL-E5v6jOQQLvoM9w]

=== Find by ID ===
Found: map[active:true categories:[electronics computers] created_at:2026-01-23T12:27:54-05:00 data:aGVsbG8gd29ybGQ= embedding:[0.1 0.2 0.3 0.4 0.5] id:retr1TCNXs8OxHItWt0_abieIdnwSsAjxDah7836KvplXnYc-ILCh3wszOVwRhW59lGyGTL-E5v6jOQQLvoM9w metadata:map[key:value nested:map[deep:true]] name:Test Record price:99.99 tags:[tag1 tag2 tag3] user_id:550e8400-e29b-41d4-a716-446655440000 value:42]

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 0
  active (Boolean): true
  price (Decimal): 99.990000
  created_at (DateTime): 2026-01-23 12:27:54 -0500 EST
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1 tag2 tag3]
  metadata (Object): map[key:value nested:map[deep:true]]
  embedding (Vector): [0.1 0.2 0.3 0.4 0.5]
  categories (Set): [electronics computers]
  data (Bytes): 11 bytes
Plain record: map[active:true categories:[electronics computers] created_at:2026-01-23T12:27:54-05:00 data:aGVsbG8gd29ybGQ= embedding:[0.1 0.2 0.3 0.4 0.5] id:retr1TCNXs8OxHItWt0_abieIdnwSsAjxDah7836KvplXnYc-ILCh3wszOVwRhW59lGyGTL-E5v6jOQQLvoM9w metadata:map[key:value nested:map[deep:true]] name:Test Record price:99.99 tags:[tag1 tag2 tag3] user_id:550e8400-e29b-41d4-a716-446655440000 value:42]

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-01-23T17:27:54Z] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:retr1TCNXs8OxHItWt0_abieIdnwSsAjxDah7836KvplXnYc-ILCh3wszOVwRhW59lGyGTL-E5v6jOQQLvoM9w metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Updated Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: hTiC0Ej3hTpQ4tGjuiMTCyV-tPnrp8O3EyXWwKPOyjbZOqnxdg4SU_u1b8FWD7KIE7QgnTwcIGcenyLwkrx7BQ

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
Collection created with first record: kNPaBdU4pEe-BHS-AeK3z0reVS6K4SPtSAi4jeBEDO4vwOP5QGCd3xQ_mcjDOdV-eXA7TIazWzB76LsrIfF4BA

=== List Collections ===
Total collections: 15
Sample collections: [schema_products_client_py chat_messages__ek0_testing github_cache ws_ttl_test test_collection]

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
Retrieved value: map[userId:123 username:john_doe]

=== KV Batch Set ===
✓ Batch set 3 keys
  cache:product:1: success
  cache:product:2: success
  cache:product:3: success

=== KV Batch Get ===
✓ Batch retrieved 3 values
  cache:product:1: map[value:map[name:Product 1 price:29.99]]
  cache:product:2: map[value:map[name:Product 2 price:39.99]]
  cache:product:3: map[value:map[name:Product 3 price:49.99]]

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 3 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 8

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: false

=== KV Batch Delete ===
✓ Batch deleted 3 keys
  cache:product:1: deleted
  cache:product:2: deleted
  cache:product:3: deleted

✓ All KV operations completed successfully
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 6kRsHr3VXGqL0QM-XULMoxrIsFHsAP2DO610SUMe7xS1aaDkqiWOHMIVitIAYXxBgWh9CufChMVREhMfEOLZaA
Created Bob: $500 - ID: nF8pp_FSqfzfBV37VcRoEqGxcMT1LxfVlT9KqZwPcG8sT29HZiOKtYk1_nSTk8VJymeMERdhkef6rVyGjDg7RA

=== Example 1: Begin Transaction ===
Transaction ID: 6ec7b52c-9f9e-40db-9dc2-63f362cb387b

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: d4d0bef9-a0a4-454d-b508-1d61b6aacce4
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
  1. Score: 25.740
  2. Score: 12.540

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400
  2. Score: 26.400
  3. Score: 26.400
  4. Score: 26.400

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600
  2. Score: 39.600
  3. Score: 39.600
  4. Score: 39.600

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200
  2. Score: 13.200

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.777
  2. Score: 0.751
  3. Score: 0.749

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.700
  2. Score: 1.511
  3. Score: 0.300

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400

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
Schema fields: 4 fields
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
Found 2 users with department data
  - Bob Smith: Sales
  - Alice Johnson: Engineering

2. Join with filtering:
Found 1 users in Engineering
  - Alice Johnson: Building A

3. Join with user profiles:
Found 2 users with profile data
  - Bob Smith: Sales Manager
  - Alice Johnson: Senior Software Engineer

4. Join orders with user data:
Found 2 completed orders
  - Laptop ($0) by Alice Johnson
  - Mouse ($0) by Alice Johnson

5. Complex join with multiple conditions:
Found 2 users with example.com emails
  - Alice Johnson (alice@example.com): Building A
  - Bob Smith (bob@example.com): Building B

=== Cleanup ===
✅ Deleted test collections

✅ Join operations examples completed!
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: u7jR6e-22jznIBZeyT60XF7mxABrgE27Du7LijmxYj2cZsGWmTjgvrlApZn7mfZ4hy8SQ7iTAAYrt3NhmZV_3w

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: R_pyJUX9Zki6JfZUo7ovOx2mKjoR6K0pmyDewvSRFT0BtG54HUlVM2W2-eUm1vlkRqW3eLLHQn0unzXQnhgK3Q

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
✓ Inserted document with TTL: iK78lLppZXhWIZwn5yDcbbeYR416r3oJZt0-Gofl7htyNaGqycRMJGzD1w7JjUlLAFnEFUtQlUxNEXSmS5O2-w

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 5 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
=== ekoDB as Edge Cache - Simple Example ===

Setting up edge cache collection...
✓ Cache entry created

Creating edge cache lookup script...
✓ Edge cache script created: YYfJeXm9Qa_IE8_08nxMslGDSWItH-1qef46Tmn4f74abye6bM8wAAUOedE5F9Ar1lV74eK_scPbYBXcxnGHOw

Call 1: Cache lookup
Response time: 2ms
Found 1 cached entries

Call 2: Cache lookup (connection warm)
Response time: 2ms
Found 1 cached entries

🧹 Cleaning up...
✓ Cleanup complete

=== The Magic ===
- Your DATABASE is your EDGE
- No Redis needed
- No CDN needed
- No cache invalidation logic needed (TTL handles it)
- With ripples: All nodes auto-sync cache
- One service: Database + Cache + Edge Functions

✓ Example complete!
🚀 ekoDB Scripts Example (Go Client)

✅ Client initialized

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: 56BuegBS2Hbz-mppiQ-1YdHkl5x_vU4R9GDgMCrSRISFrRH8BDwgz0Ig_6VQmZtPXLeG_eTsGDcbvalqScaozw
📊 Found 10 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved
📊 Found 10 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved
📊 Statistics: 2 groups
   map[avg_score:50 count:5 status:inactive]
   map[avg_score:60 count:5 status:active]
⏱️  Execution time: 0ms

📝 Example 4: Script Management

📋 Total scripts: 29
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
   Department: {"type":"String","value":"engineering"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using CallFunction to replace inline logic in SWR pattern...

✅ Saved reusable function: fetch_and_store_github
✅ Saved SWR function using composition: swr_github_user

First call (cache miss - will fetch from GitHub):
   ⏱️  Duration: 165.409667ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 82.577167ms
   📊 Records: 1
   🚀 Cache speedup: 2.0x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

✅ Level 1 function: validate_user
✅ Level 2 function: fetch_slim_user (calls validate_user)
✅ Level 3 function: count_validated_user (calls fetch_slim_user)

📊 Result from 3-level nested composition:
   Records: 1
   Name: null
   Department: null
   Record count: 1

🎯 Key Benefit: Each function is independently testable and reusable!
   - validate_user: Used in 100 different workflows
   - fetch_slim_user: Used in 50 workflows
   - count_validated_user: Specific workflow


✅ All composition examples completed!
🚀 ekoDB Go Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved: 6MCoP74yguQZOgCEWOoRHpcOT9BKjPQ6bv_c3nJ_NASAPUH03yHkhzE75-UL5_GK54N5yPjWpI71wR_ETaQrGQ
📊 Found 2 category stats

📝 Example 2: Simple Product Listing

✅ Script saved
📊 Found 5 products

📝 Example 3: Count by Category

✅ Script saved
📊 Found 2 categories

📝 Example 4: Multi-Stage Pipeline (FindAll → Group → Count)

✅ Script saved
📊 Pipeline executed 3 stages

🧹 Cleaning up...
✅ Cleanup complete

✅ All complete script examples finished!

💡 This example demonstrates ekoDB's Script system:
   ✅ FindAll operations
   ✅ Group aggregations (Count, Average)
   ✅ Multi-stage pipelines (FindAll → Group → Count)
   ✅ Script management (save, call, delete)
🚀 ekoDB Go KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: gL4QWdO_WLlNe-r8lJe_HqXOHfWhNvhfeW9XUyriLak3-t1MdAL2ri6js4iWEVbnW7yc_QwRmgwzL8TH3lbLAQ
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: TTkh0N2EIIr4zDoKqA_Tmb6b4IN_1dpfOz7bb9oH9ZI2WUJ1qHRDO75X4uwswqLl5VRkkhjcq4sWlXI-eeimCg
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: map[role:admin userId:user_abc]
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session
📝 Example 4: KV Operations in Scripts

✅ Script saved: QdRRbxEuJa8YEwDlxTjR3NtFGi3P1ivYFba0witj3ECNQQhCwEC2K3ct4KT_XvHwjbuGYcfJrB35xzEbgF_dRg
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Script

✅ Script saved: 1EmzfOA6-fnvepnLJpV9s-Nr9WhIxKteHNFmXzeshWBIzBd8kMvRYYQ8ExVvuEJnkOzlFkzefR8hu0wID0qrSA
📊 Processed order with caching
⏱️  Stages executed: 3
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All KV & Wrapped Types examples completed!

💡 Key takeaways:
   ✅ Use Field* helpers for type-safe wrapped values
   ✅ FieldDecimal() preserves precision (no floating point errors)
   ✅ KV store is great for caching and quick lookups
   ✅ StageKv*() functions work within scripts
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_github_user_go (kT2HqowY323u4Y45Y5XiNEtK5zZ70ZWUwzKwIStuKTGi3qosXhLP8NOTPpuiT0JYKrRS3UOkQ1spMVmc2dxnmg)

Step 2: First call - Cache miss, fetches from GitHub API
Result: {
  "records": [
    {
      "cached_at": "2026-01-23T12:28:03-05:00",
      "data": {
        "type": "Object",
        "value": {
          "avatar_url": "https://avatars.githubusercontent.com/u/1024025?v=4",
          "bio": null,
          "blog": "",
          "company": "Linux Foundation",
          "created_at": "2011-09-03T15:26:22Z",
          "email": null,
          "events_url": "https://api.github.com/users/torvalds/events{/privacy}",
          "followers": 279915,
          "followers_url": "https://api.github.com/users/torvalds/followers",
          "following": 0,
          "following_url": "https://api.github.com/users/torvalds/following{/other_user}",
          "gists_url": "https://api.github.com/users/torvalds/gists{/gist_id}",
          "gravatar_id": "",
          "hireable": null,
          "html_url": "https://github.com/torvalds",
          "id": 1024025,
          "location": "Portland, OR",
          "login": "torvalds",
          "name": "Linus Torvalds",
          "node_id": "MDQ6VXNlcjEwMjQwMjU=",
          "organizations_url": "https://api.github.com/users/torvalds/orgs",
          "public_gists": 1,
          "public_repos": 11,
          "received_events_url": "https://api.github.com/users/torvalds/received_events",
          "repos_url": "https://api.github.com/users/torvalds/repos",
          "site_admin": false,
          "starred_url": "https://api.github.com/users/torvalds/starred{/owner}{/repo}",
          "subscriptions_url": "https://api.github.com/users/torvalds/subscriptions",
          "twitter_username": null,
          "type": "User",
          "updated_at": "2026-01-13T07:10:05Z",
          "url": "https://api.github.com/users/torvalds",
          "user_view_type": "public"
        }
      },
      "id": "ObakaR3uMqws5oF8fMNFtT6X0Fw4nrmyqY9BvxKscH6LcZX_NuMYOtLOB2Bn7dm8QioKjmOuNuJ64in6PcA4Rw"
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
Response time: 70ms (served from cache)
✓ Lightning fast cache hit

🧹 Cleaning up...
✓ Cleanup complete

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation
🚀 ekoDB Go Advanced Scripts Example

📋 Setting up test data...
✅ Created 8 products

📝 Example 1: List All Products

✅ Script saved
📊 Found 8 products
⏱️  Execution time: 0ms

📝 Example 2: Group Products by Category

✅ Script saved
📊 Category breakdown:
   map[avg_price:367 category:Electronics count:5]
   map[avg_price:365.6666666666667 category:Furniture count:3]
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced script examples finished!
🚀 ekoDB Go AI Scripts Example

📋 Setting up test data...
✅ Created 2 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
📊 AI Response generated
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embed script saved
🧹 Cleaning up...
✅ Cleanup complete

✅ All AI script examples finished!

💡 This example demonstrates:
   ✅ Chat completions with system/user messages
   ✅ Embedding generation for text
🚀 ekoDB Go CRUD Scripts Example

📋 Setting up test data...
✅ Created 10 test users

📝 Example 1: List All Users

✅ Script saved
📊 Found 10 users
⏱️  Execution time: 0ms

📝 Example 2: Count Users by Status

✅ Script saved
📊 User counts by status:
   map[count:3 status:inactive]
   map[count:7 status:active]
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD script examples finished!
🚀 ekoDB Go Search Scripts Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Script saved
📊 Found 5 documents
   1. map[type:String value:Introduction to Machine Learning] (map[type:String value:AI])
   2. map[type:String value:Database Design Principles] (map[type:String value:Database])
   3. map[type:String value:Vector Databases Explained] (map[type:String value:Database])
   4. map[type:String value:Natural Language Processing] (map[type:String value:AI])
   5. map[type:String value:Getting Started with ekoDB] (map[type:String value:Database])
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Script saved
📊 Documents by category:
   map[category:Database count:3]
   map[category:AI count:2]
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: GFi9nITIlmmDiWZfKTsqe6vfG12LC7fx6h3DUO-A8GnXij598gQSbfMBnLLFF371mGI3CoYy-ndoWqrvEIJSvA

=== Sending Chat Message ===
Message ID: lzAKtS-AefggWhd5afaX2V0Jn-ZJAMLVy-PTBFpXNhd9zozCw7CovIyzrbZfhyZMoKS_Sd1jT7rqtC8ZMdqlRw

=== AI Response ===
The available products are:

1. ekoDB Pro: This is an enterprise edition product with advanced features. It is priced at $299.

2. ekoDB Cloud: This is a fully managed cloud database service product. It is priced at $499.

3. ekoDB: This is a high-performance database product with AI capabilities. It is priced at $99.

=== Context Used (3 snippets) ===
  Snippet 1: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Enterprise edition product with advanced features id:8Vaz2-2O2Cp0F35gGXQNgBV6ws21cPheVwIKEHuk9WV2t8wBP7O1ycjSri_Ka3DI2YzSGz7wzACLvNpEbDy4Gw name:ekoDB Pro price:299] score:0.1111111111111111]
  Snippet 2: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Fully managed cloud database service product id:cxIpm_KDIyl010jRQp2mWzNBHmuGDkZLvEM5g0so65w3IRZjnSeBUQr1-55ISKUuxTFQ7V0XQKeMpZHA51-Zlw name:ekoDB Cloud price:499] score:0.1111111111111111]
  Snippet 3: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:A high-performance database product with AI capabilities id:36MmNCrVmYyk43S4jnTT6Q7fSr34z7MlPcPW2rlJOCD4f1jWz4zVhVJO-I79DUOFGqS1PnKtI8eOlPLaEjREgg name:ekoDB price:99] score:0.1111111111111111]

Execution Time: 3324ms

=== Token Usage ===
Prompt tokens: 454
Completion tokens: 77
Total tokens: 531

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: vyFOQdM_-mtCbS08tXqiTw5oRiyWm5ddmT2M0arG49vrwddPbRyLue-FfPxBqQjcz5Iy0y3okLk6eopP_oUiIA

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information provided, the available product is the ekoDB. This is a high-performance database product priced at $99.

✓ Second message sent

=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of the ekoDB, a high-performance database product, is $99.

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: 8djIhno3iGEPxBoj-qxsGY1Zpx_yFB-AEO3e3JulWsFnGJE73TCbQ-BN95kEtrpkrz4vmqG1UpPQSqVXz7qukA
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
✓ Created session: FbZ0jfAMEmukOnFz-TIuFApCVXAa_o4foAsdfdf0x8Okg3OrSYwzqJjwjXFpqe2WJZD-Ap7dEfDxqFEgyAEjOQ

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information available, there is a product named "ekoDB". It is a high-performance database product. The price for ekoDB is $99.

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: ryewnfyMy-d70KkjeqbmqMaWpXyFiZzpAW9lOoHCtjmxKQ84gVi3lmSezoEl0U33Oea3A-nRyrlK1SweydmOPA
  Parent: FbZ0jfAMEmukOnFz-TIuFApCVXAa_o4foAsdfdf0x8Okg3OrSYwzqJjwjXFpqe2WJZD-Ap7dEfDxqFEgyAEjOQ

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: ryewnfyMy-d70KkjeqbmqMaWpXyFiZzpAW9lOoHCtjmxKQ84gVi3lmSezoEl0U33Oea3A-nRyrlK1SweydmOPA (Untitled)
  Session 2: FbZ0jfAMEmukOnFz-TIuFApCVXAa_o4foAsdfdf0x8Okg3OrSYwzqJjwjXFpqe2WJZD-Ap7dEfDxqFEgyAEjOQ (Untitled)
  Session 3: 8djIhno3iGEPxBoj-qxsGY1Zpx_yFB-AEO3e3JulWsFnGJE73TCbQ-BN95kEtrpkrz4vmqG1UpPQSqVXz7qukA (Untitled)
  Session 4: GFi9nITIlmmDiWZfKTsqe6vfG12LC7fx6h3DUO-A8GnXij598gQSbfMBnLLFF371mGI3CoYy-ndoWqrvEIJSvA (Untitled)
  Session 5: ebQGyGWgh0zo6NC6G7leQ3mI6uFt96rW_Ik654cLY62ub1HGUMF-izl-bt8R8MtfugVaDXf6ZJfAuqsIn92MAg (Untitled)
  Session 6: zOR29wOGIutIB8YS2Abnb1ahQ69bxObd1qN0vAehTVcUeLY3FKlJVFqey_muiTzCTeh6hkk8hdQVGYFPMuvPoA (Untitled)
  Session 7: deBMeaW6ZyOXZgSVS4I7CjcdHEUe5mNs3ujERxw6PbgqG52yubBVJplrqnvQgRwQHBfnGxO5nWYgAy5o7IQrMw (Untitled)
  Session 8: DC2g7WWPHsV5bOOghEpM4TZLVy4ViAFTeIHUeBiROZLxtVeH0tzTN51YS3Q-ke8HI91cEfTmVyI_oXSOkPD65A (Untitled)
  Session 9: -4ud5ChnACwEjh1N7E3gYEQ33gMpz4YAMtz_zYGQwtZblZRjNJc7BSsYHmrBEoM4Mx_Z58rUDmdMickMiP7dzg (Untitled)
  Session 10: crBexqJwKDpcsGuB3rJQOwiUxiEf2h11eXAFX6UNxZ6Ubw7DoeufXGKOj9ak8CZ38Y6jh3U1t1xtDGRI_nGmsw (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: ryewnfyMy-d70KkjeqbmqMaWpXyFiZzpAW9lOoHCtjmxKQ84gVi3lmSezoEl0U33Oea3A-nRyrlK1SweydmOPA

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Map Creation ===
✓ Created record with native map: map[id:aC5BVegqTvO6SHCW0S3_kCTSVzlky9LNEdFBZbndoJlEoChxO33bFx5EP8H6Aa2swmM1FHoMZVSjLbq6ZGFDkg]

=== Upsert Operation ===
✓ First upsert (update): map[active:map[type:Boolean value:true] age:map[type:Integer value:29] email:map[type:String value:alice.j@newdomain.com] id:aC5BVegqTvO6SHCW0S3_kCTSVzlky9LNEdFBZbndoJlEoChxO33bFx5EP8H6Aa2swmM1FHoMZVSjLbq6ZGFDkg name:map[type:String value:Alice Johnson]]
✓ Second upsert (insert): map[id:G9rJXkFK86EhDYOPvU5IuNwc8POMc-_G5ip0LjpJQw6w1ux1PsW_4PYZqNyz9ogSIAXfoFscSuWnWR4Tsa8WYw]

=== Find One Operation ===
✓ Found user by email: map[active:map[type:Boolean value:true] age:map[type:Integer value:29] email:map[type:String value:alice.j@newdomain.com] id:aC5BVegqTvO6SHCW0S3_kCTSVzlky9LNEdFBZbndoJlEoChxO33bFx5EP8H6Aa2swmM1FHoMZVSjLbq6ZGFDkg name:map[type:String value:Alice Johnson]]
✓ User not found (as expected)

=== Exists Check ===
✓ Record exists: true
✓ Fake record exists: false (should be false)

=== Pagination ===
✓ Inserted 25 records for pagination
✓ Page 1: 10 records (expected 10)
✓ Page 2: 10 records (expected 10)
✓ Page 3: 7 records (expected ~7)

=== Cleanup ===
✓ Deleted collection

✅ All convenience methods demonstrated successfully!
Inserted with ripple: map[id:mJ8ksz9vRSydj1UZpplgm3u0MIGLGFNKAzZBsar2YDnZKZS8Zb5M3yNvCdi6EKJHIsqK-7gaf-650-9FlMtGfw]
Inserted with bypass_ripple: map[id:fJH4dqsxiUn30yrI0ETAeLhoYjf_Jv6YVnARvMF_tEoo8u3UPnOo43kwSMr582lcW0-24tyWKkr6paCpwtvLQg]
Inserted with TTL and bypass_ripple: map[id:mLCPjUDcyCoVvvoQSCXd9C53pTpNHf-VuJY_LN7EAOUb4_D_BBjQUiswnCbl5d4Rq-oaj1pBDV2Gcr4qM2WfaQ]
Updated with bypass_ripple: map[id:mJ8ksz9vRSydj1UZpplgm3u0MIGLGFNKAzZBsar2YDnZKZS8Zb5M3yNvCdi6EKJHIsqK-7gaf-650-9FlMtGfw name:map[type:String value:Product 1] price:map[type:Integer value:150]]
Deleted with bypass_ripple
Batch inserted with bypass_ripple: 2 records
Upserted with bypass_ripple: map[id:KDCS95jFtz5bMiRo_tFLeXLzGGu9OQImIymApwfI_u6f2wWwf1Z1VgUdtNPk_-EK74qpFrQ1ka-AU6cBjc11Sw]
✅ [32mGo client examples complete![0m
