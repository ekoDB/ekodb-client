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
Inserted: map[id:0GvgMc3lj9I55AgcZEghmUSNC3X1PnQLAOCXOwqDx2ybz4z6DQNRbMW2-A2q6DkliJVS7GQIeBO6zdS1IGR2aw]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] id:0GvgMc3lj9I55AgcZEghmUSNC3X1PnQLAOCXOwqDx2ybz4z6DQNRbMW2-A2q6DkliJVS7GQIeBO6zdS1IGR2aw name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query ===
Found documents: [map[active:map[type:Boolean value:true] id:0GvgMc3lj9I55AgcZEghmUSNC3X1PnQLAOCXOwqDx2ybz4z6DQNRbMW2-A2q6DkliJVS7GQIeBO6zdS1IGR2aw name:map[type:String value:Test Record] value:map[type:Integer value:42]]]

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] id:0GvgMc3lj9I55AgcZEghmUSNC3X1PnQLAOCXOwqDx2ybz4z6DQNRbMW2-A2q6DkliJVS7GQIeBO6zdS1IGR2aw name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
[32m✓ simple_crud.go completed successfully[0m
[34m
=== Running simple_websocket.go ===[0m
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: myzh8ozWbo5uFwQiqnAkokhilfiGHiMexcrtOt5iGq6OK6Nf_x7KPR6M7rjnYzTUbCq1xZf2P8lSNWf4D7FC5Q

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "messageId": "1788817591210923000",
  "payload": {
    "data": [
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "myzh8ozWbo5uFwQiqnAkokhilfiGHiMexcrtOt5iGq6OK6Nf_x7KPR6M7rjnYzTUbCq1xZf2P8lSNWf4D7FC5Q",
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
        "id": "eo4k2XKeeP4Jw-NIruzkp5BZ37laWem-xan_dukdvdrmZzoejTtyeo5p4ZQXOFLyMu6bG4acNriPRHzHTq0EoQ",
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
        "id": "WsbRCK94m4hP3hqb2DP9BPADED5Qb3sY8pyagkVQFOgdbEuIFn84mi1fBQkRrv_aJlCNvJp3vQhqcdHXQRFFCQ",
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
🚀 ekoDB Functions Example (Go/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Function saved: Ce-EnLPtzYY9Xiz72eI4a0AUMmjcvTeYjpiStghkKVmqqwuPIqkMJhBwjgq7ra9dtQYW3zXjDLefj57z33holQ
📊 Found 10 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

ℹ️  Function 'get_active_users_paginated' already existed — updated instead
✅ Function saved: MeQV3_i3yzwXweQIO_tVmxFSYhdfxQrE4co4qvoEmX73e3sZaQ_N8t00rdbORwOu1zfEPKgkJsDbQLOzusQhOg
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: s9rJfRt58WlwK7ytbJc9TNYLhsikOkLDlIVFMMiPEE3NPIsb2zA33Evxrm3ddee34AbZ09dzTo1GWYvzkRff1A
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"avg_score":{"type":"Float","value":50},"count":{"type":"Integer","value":10},"max_score":{"type":"Integer","value":90},"status":{"type":"String","value":"inactive"}}
   {"avg_score":{"type":"Float","value":60},"count":{"type":"Integer","value":10},"max_score":{"type":"Integer","value":100},"status":{"type":"String","value":"active"}}

📝 Example 4: Function Management

📋 Total functions: 15
🔍 Retrieved function: Get Active Users
✏️  Function updated
🗑️  Function deleted

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
Retrieved value: map[type:Object value:map[userId:123 username:john_doe]]

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: map[type:Object value:map[name:Product 1 price:29.99]]
cache:product:2: map[type:Object value:map[name:Product 2 price:39.989999999999995]]
cache:product:3: map[type:Object value:map[name:Product 3 price:49.989999999999995]]

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
Collection created with first record: BzVLve3AuyusluMmd9hgzAIFMTaUdFAaclJ7k9iW4JybHhDfrXjqkLRWpyD9ooAPv-9KRy6gpYxK9hF0zlOIHQ

=== List Collections ===
Total collections: 19
Sample collections: [ttl_cache test_collection websocket_test schema_products_client_py demo_collection]

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
Created Alice: $1000 - ID: IO0j5iLJfCSyM5DKyGbp-p_1zj5FjV8d6nl1kg8b7qHZBU2oAkrUXshqEJAJvxyJwDr7clPiNbbUGhdHIdkC2Q
Created Bob: $500 - ID: aQ1XvCLpx4UKPAR3pOpgjTU03GKn1f-n_629caibO5C_012zv3UXqTBjyafjLrEiXDluUld_7p7RzVaz-YRTtg

=== Example 1: Begin Transaction ===
Transaction ID: d4c3755e-783a-4393-9d2e-faa0c52eff2e

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: map[type:Integer value:800]
Bob: map[type:Integer value:700]

=== Example 5: Rollback ===
New transaction: 30c64ebc-09e5-4cc4-b993-e7deef958046
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: map[type:Integer value:700]

=== Cleanup ===
✓ Deleted test accounts

✓ All transaction examples completed
[32m✓ transactions.go completed successfully[0m
[34m
=== Running crud_functions.go ===[0m
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
📝 function 1: Insert + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: _EPvKgtlTNW_LPmuyytzB7P39Gh1oqBF_k3_R5T6UELEHly-fwPqIIB_A8DGknDCaQtqyiG0du2ru9PcB5eF2g

2️⃣ Calling function (Insert + Verify)...
   ✅ function executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: j9q-_YjDtbJtUXPGBKKMzE1oD5bmyV4OBD_7lw3qDFZZ0rBZOZ4u9PhDX2U2Si3C_f2VtpcbBSV1d13Gz7x_Ng
   📋 Name: map[type:String value:Alice Smith]
   📋 Email: map[type:String value:alice@example.com]
   📋 Status: map[type:String value:pending]
   📋 Credits: map[type:Integer value:0]

============================================================
📝 function 2: Query + Update + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: BqElK0WxN1ujp4ZTfGmL1OQjqjfOW3orjXFU-2BClQDVCQsfjDoaBX6LVJbIlP2GVa6VpyFK7xu5N01xYJi80w

2️⃣ Calling function (Query + Update + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Status updated to: map[type:String value:active]
   📋 Name: map[type:String value:Alice Smith]

============================================================
📝 function 3: Query + Update Credits + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: jenXRA1TQURHDZSkE8JRvO9bw2Yd4hwnez7vzjebrmik77XGvhyjQBtoTF1LDHvfdLcplpBYXNn7dathSVK4TA

2️⃣ Calling function (Query + Update Credits + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: map[type:Integer value:0]
   📋 Status: map[type:String value:active]
   📋 Name: map[type:String value:Alice Smith]

============================================================
📝 function 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: 1nVXthIXRtG_IV_6gdDRd4jJNKLgylXAUcklsuUnh8kuQMGSI_Nlk-7-ZRNcc_JGjF96PNvi0iIanBeHPvYtAQ

2️⃣ Calling function (Query + Delete + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: _EPvKgtlTNW_LPmuyytz...
   ✅ Deleted script: BqElK0WxN1ujp4ZTfGmL...
   ✅ Deleted script: jenXRA1TQURHDZSkE8JR...
   ✅ Deleted script: 1nVXthIXRtG_IV_6gdDR...
   ✅ Deleted collection: users

============================================================
✅ Complete CRUD Functions Example Finished!
============================================================

💡 Key Takeaways:
   ✅ Functions chain steps together
   ✅ Each function demonstrates operation + verification
   ✅ Parameters make functions reusable
   ✅ Verification is built into the function itself
   ✅ Complete CRUD lifecycle in 4 focused functions
[32m✓ crud_functions.go completed successfully[0m
[34m
=== Running document_ttl.go ===[0m
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
  Output: Document ID = nCPlmcJmY2Hm3czFt-BEIhN7nHjthx5XG4QkSj3_llQ9f18jLMMwXSWb9sdpGF8G-j0cfRICRO_ixO8PvBLMBw
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: FindByID(nCPlmcJmY2Hm3czFt-BEIhN7nHjthx5XG4QkSj3_llQ9f18jLMMwXSWb9sdpGF8G-j0cfRICRO_ixO8PvBLMBw)
  Output: Found document with name = map[type:String value:TTL Test]
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: FindByID(nCPlmcJmY2Hm3czFt-BEIhN7nHjthx5XG4QkSj3_llQ9f18jLMMwXSWb9sdpGF8G-j0cfRICRO_ixO8PvBLMBw)
  Output: Error (expected) - request failed with status 404: ��error�Record not found (expired)
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

✓ Client connected

═══════════════════════════════════════════════════════════
TEST: WebSocket TTL Expiration
═══════════════════════════════════════════════════════════

[Step 1] Insert document with 3 second TTL
  Input: {name: 'WS TTL Test', value: 'should expire'}
  TTL: 3s
  Output: Document ID = 4rp7CAj3xCg4OJyGcKZkLBEco6j2_zj56X4RNmSt6Vff_vkcua4aiQNvxyt8FY0pEZAYyGpuxn96hdKNlBOrgQ
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: FindByID(4rp7CAj3xCg4OJyGcKZkLBEco6j2_zj56X4RNmSt6Vff_vkcua4aiQNvxyt8FY0pEZAYyGpuxn96hdKNlBOrgQ)
  Output: Found document with name = map[type:String value:WS TTL Test]
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: FindByID(4rp7CAj3xCg4OJyGcKZkLBEco6j2_zj56X4RNmSt6Vff_vkcua4aiQNvxyt8FY0pEZAYyGpuxn96hdKNlBOrgQ)
  Output: Error (expected) - request failed with status 404: ��error�Record not found (expired)
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
Inserted: map[id:h-RJ0MUGYOxXC-5YXke8MZ-vEfzJl8nSXI0C1X0B8d1Rl3l1BC3BLb0RxCiZULvCcocfNGxctxsXS8xFPZGmSg]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-09-07T21:47:04+00:00] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:h-RJ0MUGYOxXC-5YXke8MZ-vEfzJl8nSXI0C1X0B8d1Rl3l1BC3BLb0RxCiZULvCcocfNGxctxsXS8xFPZGmSg metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Test Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:42]]

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.990000
  created_at (DateTime): 2026-09-07 21:47:04 +0000 +0000
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1 tag2 tag3]
  metadata (Object): map[key:value nested:map[deep:true]]
  embedding (Vector): [0.1 0.2 0.3 0.4 0.5]
  categories (Set): [electronics computers]
  data (Bytes): 11 bytes
Plain record: map[active:true categories:[electronics computers] created_at:2026-09-07T21:47:04+00:00 data:aGVsbG8gd29ybGQ= embedding:[0.1 0.2 0.3 0.4 0.5] id:h-RJ0MUGYOxXC-5YXke8MZ-vEfzJl8nSXI0C1X0B8d1Rl3l1BC3BLb0RxCiZULvCcocfNGxctxsXS8xFPZGmSg metadata:map[key:value nested:map[deep:true]] name:Test Record price:99.99 tags:[tag1 tag2 tag3] user_id:550e8400-e29b-41d4-a716-446655440000 value:42]

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-09-07T21:47:04+00:00] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:h-RJ0MUGYOxXC-5YXke8MZ-vEfzJl8nSXI0C1X0B8d1Rl3l1BC3BLb0RxCiZULvCcocfNGxctxsXS8xFPZGmSg metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Updated Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: 2HXV4IUZziUnOmr4hcNsFTRS1FppAA4N7-wSkY-DNpj8hcPF-QlCJV72Z0zMcbtYdnIeN68iIpeTrClcfc0rtA

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
Collection created with first record: TQKHHx0Pl9fysXDk0wCF4L4TeDnRcZ2k4b5Bx25LUsviUPkJN2eLApyvyWA8ZMKIHtCJRvFFrjhsh0hpg6MUaw

=== List Collections ===
Total collections: 18
Sample collections: [ttl_cache test_collection websocket_test schema_products_client_py chat_configurations__ek0_testing]

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
Retrieved value: map[type:Object value:map[userId:123 username:john_doe]]

=== KV Batch Set ===
✓ Batch set 3 keys
  cache:product:1: success
  cache:product:2: success
  cache:product:3: success

=== KV Batch Get ===
✓ Batch retrieved 3 values
  cache:product:1: map[name:Product 1 price:29.99]
  cache:product:2: map[name:Product 2 price:39.99]
  cache:product:3: map[name:Product 3 price:49.99]

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 3 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 11

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
Created Alice: $1000 - ID: lIuxYI6SOH6KsRIUi5ll30-qVonVHr9jIxPPpHrmfA4cBPqKVfwTHFvTr9HdZGsugvx6o2WSB86hmVuZTrWQ4w
Created Bob: $500 - ID: bV_WbgytEPFP3Sv_WU1FinrgESeLG0W4PJaGXWlW6vvHWFbULHMbo3L4h5b6XmZYtocMHH-PnGMs_8x--RqIVA

=== Example 1: Begin Transaction ===
Transaction ID: cc44823e-8ff4-412d-8955-e56f6115f9d5

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: afc35f98-f12b-4869-881e-696b7b37a96e
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
  1. Score: 12.870
  2. Score: 6.270

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 13.200
  2. Score: 13.200
  3. Score: 13.200
  4. Score: 13.200

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 26.400
  2. Score: 26.400
  3. Score: 26.400
  4. Score: 26.400

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 6.600
  2. Score: 6.600

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 6.600

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.754
  2. Score: 0.739
  3. Score: 0.719

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.501
  2. Score: 0.895
  3. Score: 0.288

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 13.200

9. Vector search with a metadata pre-filter (category = ml):
Found 2 documents in category "ml" (NLP excluded)
  1. Introduction to Machine Learning (category: ml)
  2. Deep Learning Fundamentals (category: ml)

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
  - Alice Johnson: Engineering
  - Bob Smith: Sales

2. Join with filtering:
Found 1 users in Engineering
  - Alice Johnson: Building A

3. Join with user profiles:
Found 2 users with profile data
  - Alice Johnson: Senior Software Engineer
  - Bob Smith: Sales Manager

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
✓ Inserted document: r5ZFhYbaeHmWEkNKk98fQoZuod0U90jlIDuo4B3BRQKc1MMMvJAPQbLlOTcnvgP8LIsjD6A1HN3BXSJwf8dH9Q

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: fE07bdVlPlZMr4Rl6W08_y_53OgpKm9otNRqNJpR-jfcmb3duqHYjdvQt8uGep8wcOBfnoJrZEvhH9TqbUTQcg

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
✓ Inserted document with TTL: 7Gt2xWuQqzkRpC7h6LRzY41aPQUmBWDc1hnGZGJoNu1Su0vZPva2EM-my9ysXP55yP0iI_pUfVj73ZtolhsrRA

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
✓ Edge cache script created: S0FAXVkJ3vJsJxKA75znUfxxpvMZ4BJ9doAiRCYnXV7Zr-WEbnk4wRsCPxAmvkbtYPkBiW65C4ZshtFL6xk-fw

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
🚀 ekoDB Functions Example (Go Client)

✅ Client initialized

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

ℹ️  Function 'get_active_users' already existed — updated instead
✅ Function saved: ozLy4wDDOaGm7D_Rkmp-JE_BigNaZbLbtHE_ibFqSCj_FJ6IbpJWCnmJwwIVM5HocFVE_6uAaj99xwfA35boDg
📊 Found 10 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Function

ℹ️  Function 'get_users_by_status' already existed — updated instead
✅ Function saved
📊 Found 10 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Function

✅ Function saved
📊 Statistics: 2 groups
   map[avg_score:map[type:Float value:50] count:map[type:Integer value:5] status:map[type:String value:inactive]]
   map[avg_score:map[type:Float value:60] count:map[type:Integer value:5] status:map[type:String value:active]]
⏱️  Execution time: 0ms

📝 Example 4: Function Management

📋 Total functions: 15
🔍 Retrieved function: Get Active Users
✏️  Function updated
🗑️  Function deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

📝 Example 5: Multi-Stage Pipeline

✅ Multi-stage script saved
📊 Pipeline executed 2 stages
⏱️  Total execution time: 0ms
📈 Stage breakdown:

📝 Example 6: Count Users

✅ Count script saved
📊 Total user count: 0
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

Function 'fetch_user' already existed — updated instead
✅ Saved reusable function: fetch_user
✅ Saved composed function: get_user_wrapper (calls fetch_user + projects fields)

📊 Result from composed function:
   Records: 1
   Name: {"type":"String","value":"User 1"}
   Department: {"type":"String","value":"engineering"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

Function 'fetch_and_store_user' already existed — updated instead
✅ Saved reusable function: fetch_and_store_user (uses KV)
Function 'swr_user' already existed — updated instead
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 2.072583ms
   📊 Records: 1
   📦 Data: {
        "value": {
          "type": "Object",
          "value": {
            "address": {
              "city": "Gwenborough",
              "geo": {
                "lat": "-37.3159",
          ...

Second call (cache hit - from cache):
   ⏱️  Duration: 1.984208ms
   📊 Records: 1
   📦 Data: {
        "value": {
          "type": "Object",
          "value": {
            "address": {
              "city": "Gwenborough",
              "geo": {
                "lat": "-37.3159",
          ...
   🚀 Cache speedup: 2.0x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

Function 'validate_user' already existed — updated instead
✅ Level 1 function: validate_user
Function 'fetch_slim_user' already existed — updated instead
✅ Level 2 function: fetch_slim_user (calls validate_user)
✅ Level 3 function: get_verified_user (calls fetch_slim_user)

📊 Result from 3-level nested composition:
   Records: 1
   Name: {"type":"String","value":"User 1"}
   Department: {"type":"String","value":"engineering"}

🎯 Key Benefit: Each function is independently testable and reusable!
   - validate_user: Used in 100 different workflows
   - fetch_slim_user: Used in 50 workflows
   - get_verified_user: Specific workflow


✅ All composition examples completed!
🚀 ekoDB Go Complete Functions Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Function saved: OKBZ_BA6SvytuLqaRDrd9IwUNVuAyVn-w5aiZnDr4L0Yo7yuG26KdvHWdzY6isSBf-IpmLmwS1ijb2UIPgklmQ
📊 Found 2 category stats

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories

📝 Example 4: Multi-Stage Pipeline (FindAll → Group → Count)

✅ Function saved
📊 Pipeline executed 3 stages

🧹 Cleaning up...
✅ Cleanup complete

✅ All complete script examples finished!

💡 This example demonstrates ekoDB's function system:
   ✅ FindAll operations
   ✅ Group aggregations (Count, Average)
   ✅ Multi-stage pipelines (FindAll → Group → Count)
   ✅ Function management (save, call, delete)
🚀 ekoDB Go KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: Z8AHZiYQTU-LE_6gEjEXEp17l2l5sKR1N4bFU9K7iMtX5APeA0_stpP8EkV5217i8pusJVnYvQ46YATFAmFKRg
✅ Inserted 2 products with wrapped types

📝 Example 2: function with Wrapped Type Parameters

✅ Function saved: _0hsfJhH3zhzujr-IKdifwBMveuISKCL31BM0ujYVMpWXM_PCSdrvC8EwENRg1yB3Ou4tTmdXty12Se_fWYO4A
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: map[type:Object value:map[role:admin userId:user_abc]]
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session
📝 Example 4: KV Operations in Functions

✅ Function saved: uN0_NYaRSTxy89ycWQrF1sED3M8e36M4NN4opwdzyILevwwZjubCaRZd_F1qzbT7hy_zxQ-rA3YrCXsIn83UsQ
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Function

✅ Function saved: G--GNISCc9juJd95XMp5xSA9DHhsiz5tFSVPwnoYZOUFjfZBvsdONgQJvFFErKtsNn2j5SwX-idLkXR63nDakA
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
✓ Created SWR script: fetch_api_user_go (_wdSfTdSIvqK3RKW8I53XIDo_fdhI6kE5GxI8A-bo3uzX6ZO0sH15l9nFVzpatLstJHEfBm4X1PuzqegAS8M1Q)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "cached_at": {
        "type": "DateTime",
        "value": "2026-09-07T21:47:14+00:00"
      },
      "data": {
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
      },
      "id": "1"
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
Response time: 2ms (served from cache)
✓ Lightning fast cache hit

🧹 Cleaning up...
✓ Cleanup complete

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation
🚀 ekoDB Go Client - Native SWR Function Examples

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
✓ Created native SWR script: github_user_native (g4vzbZi-Z1jh7Wf85sv9nrBXBZdrL4W3_1XCMf0t3vNVpkNoO2jB-26O_V_J3cokBP-gD5uBmhG0HGxdne6iWA)

First call (cache miss - will fetch from GitHub API):
  Response time: 5ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 5ms
  Speedup: 1.0x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (Xm58tGu9YLlIayjDhdhotU7SuO_v0MsfrLFQorNRxMMsSu8_fRdEHnHk9ONOtRcpUGiCz79uliRHohLH6s9VLA)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (C2r8cxfWnSRTNVQummLBO4znL-G2uSi_Po2qa9uByPnWksQwJ75oO-_AdXAjKbNV0A3tRJp2RQ0UAFvxCSNyxg)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (1j3rgJIqoKGW47QZogk_hPC0X3YgZ4Dn769Px482eIJjB8mP2gsdPmQ9-3aRJZS2u3dIAS1aOJSkaUqM8zKrLA)
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
🚀 ekoDB Go Advanced Functions Example

📋 Setting up test data...
✅ Created 8 products

📝 Example 1: List All Products

✅ Function saved
📊 Found 8 products
⏱️  Execution time: 0ms

📝 Example 2: Group Products by Category

✅ Function saved
📊 Category breakdown:
   map[avg_price:map[type:Float value:365.6666666666667] category:map[type:String value:Furniture] count:map[type:Integer value:3]]
   map[avg_price:map[type:Float value:367] category:map[type:String value:Electronics] count:map[type:Integer value:5]]
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced script examples finished!
🚀 ekoDB Go AI Functions Example

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
🚀 ekoDB Go CRUD Functions Example

📋 Setting up test data...
✅ Created 10 test users

📝 Example 1: List All Users

✅ Function saved
📊 Found 10 users
⏱️  Execution time: 0ms

📝 Example 2: Count Users by Status

✅ Function saved
📊 User counts by status:
   map[count:map[type:Integer value:7] status:map[type:String value:active]]
   map[count:map[type:Integer value:3] status:map[type:String value:inactive]]
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD script examples finished!
🚀 ekoDB Go Search Functions Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Function saved
📊 Found 5 documents
   1. map[type:String value:Vector Databases Explained] (map[type:String value:Database])
   2. map[type:String value:Introduction to Machine Learning] (map[type:String value:AI])
   3. map[type:String value:Getting Started with ekoDB] (map[type:String value:Database])
   4. map[type:String value:Natural Language Processing] (map[type:String value:AI])
   5. map[type:String value:Database Design Principles] (map[type:String value:Database])
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   map[category:map[type:String value:AI] count:map[type:Integer value:2]]
   map[category:map[type:String value:Database] count:map[type:Integer value:3]]
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: EGq4BpwScMdsQDiBfksclFl8625dagxm3gVyxIyXJKhHxHZyinKJiOelY97gqIKri4o4BnJj2GVPWG8XuixU0Q

=== Sending Chat Message ===
Message ID: mbZtWJSaQkAZtYIXT_cYMeGBT4fLpXu2z5YS_tXrRGvcp9DxMjybJWf-vCGtkbGbNdxIMhIINjjmK1vBfPiNwg

=== AI Response ===
Here are the available products and their prices:

1. **ekoDB Pro**
   - Description: Enterprise edition product with advanced features
   - Price: $299

2. **ekoDB Cloud**
   - Description: Fully managed cloud database service product
   - Price: $499

3. **ekoDB**
   - Description: A high-performance database product with AI capabilities
   - Price: $99

=== Context Used (3 snippets) ===
  Snippet 1: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Enterprise edition product with advanced features id:AXrK8NWL4g_oSpQjpiOqy1yF1NChT1W9jKmHuP4HRq4DqTmCvzxgRbpBiyuW1-KQP73BQJ92W0vXSolzIpFH5w name:ekoDB Pro price:299] score:0.1111111111111111]
  Snippet 2: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Fully managed cloud database service product id:QVp5UFRjgksMU81w0-_VgOz-SD0vlQ7dG95uBC6E5ALXZa96TuyY-YS-HHtErZRBLtgqKqUnDlgm1cgc5YsPww name:ekoDB Cloud price:499] score:0.1111111111111111]
  Snippet 3: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:A high-performance database product with AI capabilities id:96bHVMXS7wMfahxtbbUSOFQtRtnHfl7WrEX9ugrbPUYc_qqShYF6n19Cetl1TuS7EgQSd0EMJ__biu4RdB0uxA name:ekoDB price:99] score:0.1111111111111111]

Execution Time: 2083ms

=== Token Usage ===
Prompt tokens: 3413
Completion tokens: 89
Total tokens: 3502

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: _2H2yeGPCd7-YtnDyN6f0V20uICvJdTCwMuwNeYap0nJRTa5cLgNcAcdYaCldl1N50nVf-289J258QMDs7Y8wg

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: High-performance database product
- **Price**: $99

If you need more information or have further questions, feel free to ask!

✓ Second message sent

=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of ekoDB is $99. If you need any more information, feel free to ask!

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: Kh_2y1eqgZka3uqUF1ti21UPDhrWr12DBbgPg3rsoS8AX9-4AVxYHe8VZ1pIwPbRRRaEA6LYfPvIteEXjW8_yQ
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 7

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 6

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 5hi3bOxBbTb2WcHkXfr7bCB4mS9f44BEJ-DAXVsqrF96OST0diI2NxRenbSDb4MEKRC9FbQwtn6_hP_p7xTt4w

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** A high-performance database product
- **Price:** $99

If you need more information or have any other questions, feel free to ask!

✓ Message 2 sent
  Response: The price of the ekoDB product is **$99**.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 7zdpWsP6f4aUa10hNP87v9yWu5_opG6xiaCFryc82tWO-V2d3SYDZ-AVAwm6eMG42TeyqpRng19IRtAQe_OUmw
  Parent: 5hi3bOxBbTb2WcHkXfr7bCB4mS9f44BEJ-DAXVsqrF96OST0diI2NxRenbSDb4MEKRC9FbQwtn6_hP_p7xTt4w

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: 7zdpWsP6f4aUa10hNP87v9yWu5_opG6xiaCFryc82tWO-V2d3SYDZ-AVAwm6eMG42TeyqpRng19IRtAQe_OUmw (Untitled)
  Session 2: 5hi3bOxBbTb2WcHkXfr7bCB4mS9f44BEJ-DAXVsqrF96OST0diI2NxRenbSDb4MEKRC9FbQwtn6_hP_p7xTt4w (Untitled)
  Session 3: Kh_2y1eqgZka3uqUF1ti21UPDhrWr12DBbgPg3rsoS8AX9-4AVxYHe8VZ1pIwPbRRRaEA6LYfPvIteEXjW8_yQ (Untitled)
  Session 4: EGq4BpwScMdsQDiBfksclFl8625dagxm3gVyxIyXJKhHxHZyinKJiOelY97gqIKri4o4BnJj2GVPWG8XuixU0Q (Untitled)
  Session 5: nkBJXWlWv5CQEGIuO_iZt14BLDnyv3qqWNQSWIMEdEG76GnvcBXraqm_M40BZLRGqU3AaCGxIVVZv7wwyClb2g (Untitled)
  Session 6: MsFgaGHuEMqowhdNh55laOo1tJMk2CoF8bjwtylNl90Ei5BOTrKz0kqajiENJs7al67mxOoZizgXgU1_a19jWg (Untitled)
  Session 7: daiZC2QZTBDSnnyM0mBh543J3Ee0r_G5DhBylJLWym6POyg7M3-dT4tow1FkEhyU8qc6hIfOLBx-FI_9u16NIA (Untitled)
  Session 8: 1XLUyFhbuCQozWHtbWvx5Jjk1LEyfQxrk2ovJjO7bwll-2gtz50K88C5Zqr1REVwWLTNjHiDTqQmG2I9hmEMLQ (Untitled)
  Session 9: L1UQzFyFR4ccuvwyDYzO0gicZTwcGTyE2eObq41Pxc0Af_hs8haT87q3nMhpH4rHu1SgAXeMe2rlCNCgMNRpWA (Untitled)
  Session 10: EpncQpgraSLlTwxuY9OQk1S-6BHw17FPHrGe1hQSC1k3Y-mKu82KFBbgejEmRy0C6de3NyaqW20Z830tZ6GwRA (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: 7zdpWsP6f4aUa10hNP87v9yWu5_opG6xiaCFryc82tWO-V2d3SYDZ-AVAwm6eMG42TeyqpRng19IRtAQe_OUmw

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Map Creation ===
✓ Created record with native map: map[id:g2dYWtzchw0RezQWNayrytyMoNGAOj4TXSzD6U_FpSedmf7cuIBAp72DjhWopb2n-MoNhZyWDYhJrhvLd6vr8Q]

=== Upsert Operation ===
✓ First upsert (update): map[active:map[type:Boolean value:true] age:map[type:Integer value:29] email:map[type:String value:alice.j@newdomain.com] id:g2dYWtzchw0RezQWNayrytyMoNGAOj4TXSzD6U_FpSedmf7cuIBAp72DjhWopb2n-MoNhZyWDYhJrhvLd6vr8Q name:map[type:String value:Alice Johnson]]
✓ Second upsert (insert): map[id:new-user-id]

=== Find One Operation ===
✓ Found user by email: map[active:map[type:Boolean value:true] age:map[type:Integer value:29] email:map[type:String value:alice.j@newdomain.com] id:g2dYWtzchw0RezQWNayrytyMoNGAOj4TXSzD6U_FpSedmf7cuIBAp72DjhWopb2n-MoNhZyWDYhJrhvLd6vr8Q name:map[type:String value:Alice Johnson]]
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
Inserted with ripple: map[id:lmWdR3sjMfPyUaV-zYBN9FaBipFGC3-w2kbDNiZxS-Oht0e3Stx9__jkRBkuvcSUX0uhA11iB2q77pI0Onwoxw]
Inserted with bypass_ripple: map[id:fYohOiopLUQL75aoOZw-6UTpAE56TZLGq4z9Hm32N5APr8b1whVlmkWlT_y39OLcvgF80-Q4jbvrTShwE6Bp6A]
Inserted with TTL and bypass_ripple: map[id:k2qqMcsszy4AgA2eERwY-V-72cIsaDp-dU-M9LBxuwRHVrMD-kTxcxszWN0OWqVryrZUBIkfb6nyzt7DFPI4dg]
Updated with bypass_ripple: map[id:lmWdR3sjMfPyUaV-zYBN9FaBipFGC3-w2kbDNiZxS-Oht0e3Stx9__jkRBkuvcSUX0uhA11iB2q77pI0Onwoxw name:map[type:String value:Product 1] price:map[type:Integer value:150]]
Deleted with bypass_ripple
Batch inserted with bypass_ripple: 2 records
Upserted with bypass_ripple: map[id:custom-id]
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields (id, name, email only)
  Found 3 active users
  Fields returned: [email id name]
  First user: Bob Smith <bob@example.com>

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: [age avatar_url bio created_at email id name status user_role]

Example 3: Complex query with projection (active users, ages 18-65)
  Found 3 active users (ages 18-65)
    - Dave Brown (age 45)
    - Alice Johnson (age 30)
    - Bob Smith (age 25)

Example 4: Query inactive users with profile fields
  Found 1 inactive users
    - Carol White: Manager

Example 5: Compare full vs projected data
  Full query:
    - 12 fields per record
    - Fields: [age api_key avatar_url bio created_at email id name password secret_token status user_role]
  Projected query:
    - 3 fields per record
    - Fields: [email id name]
  Bandwidth savings: ~75% fewer fields

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
=== KV Precision: Float vs Decimal ===

=== Test 1: Using Go Floats (LOSES PRECISION) ===
✓ Stored products with float prices

Retrieved float prices:
  Widget A: $29.99 (expected $29.99) ✓
  Widget B: $39.99 (expected $39.99) ✓
  Widget C: $49.99 (expected $49.99) ✓

=== Test 2: Using FieldDecimal() (PRESERVES PRECISION) ===
✓ Stored products with decimal prices

Retrieved decimal prices:
  Widget A: $29.99 (expected $29.99) ✓
  Widget B: $39.99 (expected $39.99) ✓
  Widget C: $49.99 (expected $49.99) ✓

=== Test 3: Sum Calculation Comparison ===
  Float sum: $119.97 (expected $119.97)
  Decimal sum: $119.97 (expected $119.97)

=== Test 4: Extreme Precision Example ===
  Float 0.1 + 0.2 = 0.00000000000000000 (should be 0.3)
  Decimal "0.30" =  (exact!)

=== Cleanup ===
✓ Cleaned up test keys

=== Summary ===
✅ Use FieldDecimal() for monetary values, percentages, and
   any case where floating-point errors are unacceptable.
✅ FieldDecimal() stores values as strings internally,
   preserving exact precision across all operations.
✓ Client created

=== Get All Chat Models ===
OpenAI models: [text-embedding-ada-002 whisper-1 gpt-3.5-turbo tts-1 gpt-3.5-turbo-16k gpt-4-0613 gpt-4 davinci-002 babbage-002 gpt-3.5-turbo-instruct gpt-3.5-turbo-instruct-0914 gpt-3.5-turbo-1106 tts-1-hd tts-1-1106 tts-1-hd-1106 text-embedding-3-small text-embedding-3-large gpt-3.5-turbo-0125 gpt-4-turbo gpt-4-turbo-2024-04-09 gpt-4o gpt-4o-2024-05-13 gpt-4o-mini-2024-07-18 gpt-4o-mini gpt-4o-2024-08-06 omni-moderation-latest omni-moderation-2024-09-26 o1-2024-12-17 o1 o3-mini o3-mini-2025-01-31 gpt-4o-2024-11-20 gpt-4o-mini-search-preview-2025-03-11 gpt-4o-mini-search-preview gpt-4o-transcribe gpt-4o-mini-transcribe o1-pro-2025-03-19 o1-pro gpt-4o-mini-tts o3-2025-04-16 o4-mini-2025-04-16 o3 o4-mini gpt-4.1-2025-04-14 gpt-4.1 gpt-4.1-mini-2025-04-14 gpt-4.1-mini gpt-4.1-nano-2025-04-14 gpt-4.1-nano gpt-image-1 o4-mini-deep-research gpt-4o-transcribe-diarize o4-mini-deep-research-2025-06-26 gpt-5-chat-latest gpt-5-2025-08-07 gpt-5 gpt-5-mini-2025-08-07 gpt-5-mini gpt-5-nano-2025-08-07 gpt-5-nano gpt-audio-2025-08-28 gpt-realtime gpt-realtime-2025-08-28 gpt-audio gpt-5-codex gpt-image-1-mini gpt-5-pro-2025-10-06 gpt-5-pro gpt-audio-mini gpt-audio-mini-2025-10-06 gpt-5-search-api gpt-realtime-mini sora-2 sora-2-pro gpt-5-search-api-2025-10-14 gpt-5.1-chat-latest gpt-5.1-2025-11-13 gpt-5.1 gpt-5.1-codex gpt-5.1-codex-mini gpt-5.1-codex-max gpt-image-1.5 gpt-5.2-2025-12-11 gpt-5.2 gpt-5.2-pro-2025-12-11 gpt-5.2-pro gpt-5.2-chat-latest gpt-4o-mini-transcribe-2025-12-15 gpt-4o-mini-transcribe-2025-03-20 gpt-4o-mini-tts-2025-03-20 gpt-4o-mini-tts-2025-12-15 gpt-realtime-mini-2025-12-15 gpt-audio-mini-2025-12-15 chatgpt-image-latest gpt-5.2-codex gpt-5.3-codex gpt-realtime-1.5 gpt-audio-1.5 gpt-4o-search-preview gpt-4o-search-preview-2025-03-11 gpt-5.3-chat-latest gpt-5.4-2026-03-05 gpt-5.4-pro gpt-5.4-pro-2026-03-05 gpt-5.4 gpt-5.4-nano-2026-03-17 gpt-5.4-nano gpt-5.4-mini-2026-03-17 gpt-5.4-mini gpt-image-2 gpt-image-2-2026-04-21 gpt-5.5 gpt-5.5-2026-04-23 gpt-5.5-pro gpt-5.5-pro-2026-04-23 chat-latest gpt-realtime-translate gpt-realtime-2 gpt-realtime-whisper gpt-5.6-sol gpt-5.6-terra gpt-5.6-luna gpt-realtime-2.1 gpt-realtime-2.1-mini gpt-transcribe gpt-live-transcribe gpt-6-astra]
Anthropic models: [claude-fable-5-1 claude-opus-5 claude-sonnet-5 claude-fable-5 claude-opus-4-8 claude-opus-4-7 claude-sonnet-4-6 claude-opus-4-6 claude-opus-4-5-20251101 claude-haiku-4-5-20251001 claude-sonnet-4-5-20250929]
Perplexity models: []
Gemini models: []
  anthropic: ok 11 models
  gemini: not_configured (unverified) No Gemini API Key
  openai: ok 127 models
  perplexity: not_configured (unverified) No Perplexity API Key

=== Get OpenAI Models ===
OpenAI models: [text-embedding-ada-002 whisper-1 gpt-3.5-turbo tts-1 gpt-3.5-turbo-16k gpt-4-0613 gpt-4 davinci-002 babbage-002 gpt-3.5-turbo-instruct gpt-3.5-turbo-instruct-0914 gpt-3.5-turbo-1106 tts-1-hd tts-1-1106 tts-1-hd-1106 text-embedding-3-small text-embedding-3-large gpt-3.5-turbo-0125 gpt-4-turbo gpt-4-turbo-2024-04-09 gpt-4o gpt-4o-2024-05-13 gpt-4o-mini-2024-07-18 gpt-4o-mini gpt-4o-2024-08-06 omni-moderation-latest omni-moderation-2024-09-26 o1-2024-12-17 o1 o3-mini o3-mini-2025-01-31 gpt-4o-2024-11-20 gpt-4o-mini-search-preview-2025-03-11 gpt-4o-mini-search-preview gpt-4o-transcribe gpt-4o-mini-transcribe o1-pro-2025-03-19 o1-pro gpt-4o-mini-tts o3-2025-04-16 o4-mini-2025-04-16 o3 o4-mini gpt-4.1-2025-04-14 gpt-4.1 gpt-4.1-mini-2025-04-14 gpt-4.1-mini gpt-4.1-nano-2025-04-14 gpt-4.1-nano gpt-image-1 o4-mini-deep-research gpt-4o-transcribe-diarize o4-mini-deep-research-2025-06-26 gpt-5-chat-latest gpt-5-2025-08-07 gpt-5 gpt-5-mini-2025-08-07 gpt-5-mini gpt-5-nano-2025-08-07 gpt-5-nano gpt-audio-2025-08-28 gpt-realtime gpt-realtime-2025-08-28 gpt-audio gpt-5-codex gpt-image-1-mini gpt-5-pro-2025-10-06 gpt-5-pro gpt-audio-mini gpt-audio-mini-2025-10-06 gpt-5-search-api gpt-realtime-mini sora-2 sora-2-pro gpt-5-search-api-2025-10-14 gpt-5.1-chat-latest gpt-5.1-2025-11-13 gpt-5.1 gpt-5.1-codex gpt-5.1-codex-mini gpt-5.1-codex-max gpt-image-1.5 gpt-5.2-2025-12-11 gpt-5.2 gpt-5.2-pro-2025-12-11 gpt-5.2-pro gpt-5.2-chat-latest gpt-4o-mini-transcribe-2025-12-15 gpt-4o-mini-transcribe-2025-03-20 gpt-4o-mini-tts-2025-03-20 gpt-4o-mini-tts-2025-12-15 gpt-realtime-mini-2025-12-15 gpt-audio-mini-2025-12-15 chatgpt-image-latest gpt-5.2-codex gpt-5.3-codex gpt-realtime-1.5 gpt-audio-1.5 gpt-4o-search-preview gpt-4o-search-preview-2025-03-11 gpt-5.3-chat-latest gpt-5.4-2026-03-05 gpt-5.4-pro gpt-5.4-pro-2026-03-05 gpt-5.4 gpt-5.4-nano-2026-03-17 gpt-5.4-nano gpt-5.4-mini-2026-03-17 gpt-5.4-mini gpt-image-2 gpt-image-2-2026-04-21 gpt-5.5 gpt-5.5-2026-04-23 gpt-5.5-pro gpt-5.5-pro-2026-04-23 chat-latest gpt-realtime-translate gpt-realtime-2 gpt-realtime-whisper gpt-5.6-sol gpt-5.6-terra gpt-5.6-luna gpt-realtime-2.1 gpt-realtime-2.1-mini gpt-transcribe gpt-live-transcribe gpt-6-astra]

=== Get Anthropic Models ===
Anthropic models: [claude-fable-5-1 claude-opus-5 claude-sonnet-5 claude-fable-5 claude-opus-4-8 claude-opus-4-7 claude-sonnet-4-6 claude-opus-4-6 claude-opus-4-5-20251101 claude-haiku-4-5-20251001 claude-sonnet-4-5-20250929]

✓ Chat Models API example complete
✓ Client created

=== Create User Function ===
Created user function with ID: Res8YXGt04bVZM4VTqWmuDhGV1kUXWorgizDO0MnIkPaROwzupN4yUsFvZbxB9H-gyg7cxmuCZtuGQsPC7lPdA

=== Get User Function ===
Retrieved: get_active_users - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 9 user functions:
  - fetch_slim_user: Validate and slim down user
  - fetch_user: Fetch user by code
  - validate_user: Check if user exists
  - swr_user: SWR pattern for user data (KV-based)
  - cache_api_call_py: Cache External API Call
  - get_verified_user: Get verified and validated user
  - get_user_wrapper: Wrapper that calls fetch_user
  - fetch_and_store_user: Fetch user from API and cache in KV
  - get_active_users: Get Active Users

=== List User Functions by Tag ===
Found 9 user functions with 'users' tag:
  - fetch_slim_user
  - fetch_user
  - validate_user
  - swr_user
  - cache_api_call_py
  - get_verified_user
  - get_user_wrapper
  - fetch_and_store_user
  - get_active_users

=== Update User Function ===
User function updated successfully

=== Delete User Function ===
User function deleted successfully

✓ User Functions API example complete
✓ Client created

=== Check Collection Exists (Before Creation) ===
Collection 'collection_utils_test' exists: false

=== Creating Test Documents ===
Created 5 test documents

=== Check Collection Exists (After Creation) ===
Collection 'collection_utils_test' exists: true

=== Count Documents ===
Document count in 'collection_utils_test': 5

=== Check Non-Existent Collection ===
Collection 'nonexistent_collection_xyz' exists: false

=== Cleanup ===
Deleted collection 'collection_utils_test'

✓ Collection Utilities example complete
✓ Client created
✓ go_users_register saved
✓ go_users_login saved
✓ go_users_verify_token saved

=== Auth flow defined as pure stored functions ===
Call them like:
  POST /api/functions/go_users_register { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/go_users_login    { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/go_users_verify_token { "token": "<jwt>" }

✓ Cleaned up demo functions
✓ Client created
✓ crypto_demo_hmac saved
✓ crypto_demo_aes saved
✓ crypto_demo_uuid saved
✓ crypto_demo_totp saved
✓ crypto_demo_encoding saved

Invoke them with:
  POST /api/functions/crypto_demo_hmac     { "payload": "hi" }
  POST /api/functions/crypto_demo_aes      { "plaintext": "secret" }
  POST /api/functions/crypto_demo_uuid
  POST /api/functions/crypto_demo_totp
  POST /api/functions/crypto_demo_encoding { "title": "Héllo World" }

✓ Cleaned up demo functions
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
✓ Client created
✓ go_route_admin saved
✓ go_route_user_by_id saved
✓ go_route_user_posts saved
✓ go_route_org_create_member saved

Try them with curl:
  curl http://localhost:8080/api/route/users/admin
  curl http://localhost:8080/api/route/users/42
  curl http://localhost:8080/api/route/users/42/posts/7
  curl -X POST http://localhost:8080/api/route/orgs/acme/members \
       -H 'Content-Type: application/json' -d '{"name":"alice"}'

✓ Cleaned up demo functions
✅ [32mGo client examples complete![0m
