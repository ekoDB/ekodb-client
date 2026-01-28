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
Inserted: map[id:1SPzmvmEzruTUag-Gz4QadtVcm89Mz_9RbpjtIFnF7ckTuCT8P_J4FYwvpa8nCecVDqAxYJ5tNyjRYApNtx0uQ]

=== Find by ID ===
Found: map[active:true id:1SPzmvmEzruTUag-Gz4QadtVcm89Mz_9RbpjtIFnF7ckTuCT8P_J4FYwvpa8nCecVDqAxYJ5tNyjRYApNtx0uQ name:Test Record value:42]

=== Find with Query ===
Found documents: [map[active:map[type:Boolean value:true] id:1SPzmvmEzruTUag-Gz4QadtVcm89Mz_9RbpjtIFnF7ckTuCT8P_J4FYwvpa8nCecVDqAxYJ5tNyjRYApNtx0uQ name:map[type:String value:Test Record] value:map[type:Integer value:42]]]

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] id:1SPzmvmEzruTUag-Gz4QadtVcm89Mz_9RbpjtIFnF7ckTuCT8P_J4FYwvpa8nCecVDqAxYJ5tNyjRYApNtx0uQ name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
[32m✓ simple_crud.go completed successfully[0m
[34m
=== Running simple_websocket.go ===[0m
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: nNbqNp0PZJeu64PMuqIrmHqPklGfWntWmBXjK_lKGx_W3ZKFCFjRngAoKH6Qz1MavLhGdZhaGsW2vaME1-AoBw

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
        "id": "S_U13Mss9h6XuHFJAuiD6zeBJ1UT7ZOk0_SZxhcjzzoyIThIysh_qUPR8HijAPJf_mlmpEgauyRwdt0ZfUuDxA",
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
        "id": "nNbqNp0PZJeu64PMuqIrmHqPklGfWntWmBXjK_lKGx_W3ZKFCFjRngAoKH6Qz1MavLhGdZhaGsW2vaME1-AoBw",
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
        "id": "S-g_KsEmXxeRR7SufzogTElDqP-F9m9IRs05FO8UL78xYZaP_aAkSbHUzqAbaHHS4eeylQKCp90CHoPUwFFtkQ",
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

✅ Script saved: 3DOZvAM75Pa_3YwOElUlx-kGyKGlSatJxSXTC8liqu76uk5hmNxufuNoHrenAVKERKCx6KS97RCP-VYt6AArzw
📊 Found 10 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: p3JiUY7IAmp1SyGvC1WVL54nxQkhpluyZoFdf8CpYAsurW1OrUkD8c8ESXZKyeeH2UcF438ih6ZruFpIEj4QHQ
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: ktZwDUB59VLQWb4fnIuU8OUgGOnqDsOTPu7Ft_A6BNKKNEfhLKdhyriIZlFNj2nihsJPcDb7AgUypw94TKzbpQ
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"avg_score":50,"count":10,"max_score":90,"status":"inactive"}
   {"avg_score":60,"count":10,"max_score":100,"status":"active"}

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
Collection created with first record: 5i638lJIFPx6oxWrkGdJcXxeghVSxD1qoBDxu4HPkBvgle9p4vYcsa81u3bP-533iEt2i-z2HEbwoqM8CkKhFw

=== List Collections ===
Total collections: 15
Sample collections: [chat_configurations__ek0_testing test_accounts batch_users demo_collection schema_users_client_py]

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
Created Alice: $1000 - ID: 6LlGDD4lonhjCHTYoPg3oeXE6FvhGVzTie_GsuYRTqV_kt2G6A4lCx2xd4CZTEFSxcj53Gz17AumwCPxwwn_hw
Created Bob: $500 - ID: gHOBa6YaFy1Yj1bdEE8UMWoT6E6dzxYg6j2h6u1B09-ciYZdSHobO9ZTBAesvN8uakKv9_htNwsKycQ3NHbAng

=== Example 1: Begin Transaction ===
Transaction ID: 545ccd18-0e81-41d8-b350-08f16a709f77

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
New transaction: 4a0166a8-522f-4d08-a666-5ebec4ab1564
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
   ✅ Script saved: CfLaYYX3Zt-1LpjtNROcvlmHtHGjkxE2kcjAa604nzGI2Uzsl4pluyY2nt2nt18oPNjyLhadayMEHuvQuwZjsg

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: jjMQer-jE_v8khn9SXS6ymsggAtm9Sr4eH_VTO3K1JcKzd1XH8rblDbHxhRA6jAjN7hQUhO8fDJJIn5RoeNaiQ
   📋 Name: map[type:String value:Alice Smith]
   📋 Email: map[type:String value:alice@example.com]
   📋 Status: map[type:String value:pending]
   📋 Credits: map[type:Integer value:0]

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: NLXyAYinipuOHcei0mXsMpmUrhsUTvoiEh7BACixFeFFh4rgn5V_Ua_Rpuc-D1F02N71hCT8EQb-Nh_DYiG-Zg

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
   ✅ Script saved: XQqDlWB1WuSMM9W9aXU8r9xFqc6rVTJPseBi1LQQdyNSHPXLZIi1_d4sbzKpYjY6SrnurTmrPehMmf3yi70ErA

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
   ✅ Script saved: wirTNAoPOMVpI_peeL4HUe6kn5UValIe2jDbzJTUZRwMBwklFleZWPqPhuY6aUd2u7H0K4PeexmTU8e90_UTuA

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: CfLaYYX3Zt-1LpjtNROc...
   ✅ Deleted script: NLXyAYinipuOHcei0mXs...
   ✅ Deleted script: XQqDlWB1WuSMM9W9aXU8...
   ✅ Deleted script: wirTNAoPOMVpI_peeL4H...
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

2026/01/27 12:13:46 No .env file found, using defaults
✓ Client connected

═══════════════════════════════════════════════════════════
TEST 1: Document TTL Expiration
═══════════════════════════════════════════════════════════

[Step 1] Insert document with 3 second TTL
  Input: {name: 'TTL Test', value: 'should expire'}
  TTL: 3s
  Output: Document ID = epOzbUXtrEAfT1dxBNNWFDN-R_yYYqXpPi_zQaT_I-QpXa5kW9j4rPZfpue1CP3RNAqIuPHI6nCiwCp3gG0RsA
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: FindByID(epOzbUXtrEAfT1dxBNNWFDN-R_yYYqXpPi_zQaT_I-QpXa5kW9j4rPZfpue1CP3RNAqIuPHI6nCiwCp3gG0RsA)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: FindByID(epOzbUXtrEAfT1dxBNNWFDN-R_yYYqXpPi_zQaT_I-QpXa5kW9j4rPZfpue1CP3RNAqIuPHI6nCiwCp3gG0RsA)
  Output: Error (expected) - request failed with status 404: ��Record has been deleted
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

2026/01/27 12:13:53 No .env file found, using defaults
✓ Client connected

═══════════════════════════════════════════════════════════
TEST: WebSocket TTL Expiration
═══════════════════════════════════════════════════════════

[Step 1] Insert document with 3 second TTL
  Input: {name: 'WS TTL Test', value: 'should expire'}
  TTL: 3s
  Output: Document ID = SwYk2yP4BIAY4yLurdrS8I5oOmDIVfMGLtKx44CNyW7Ww5okiT0a0l3Z-t9ifRdK2qjIBKU-l2d3YOp9zcZDpw
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: FindByID(SwYk2yP4BIAY4yLurdrS8I5oOmDIVfMGLtKx44CNyW7Ww5okiT0a0l3Z-t9ifRdK2qjIBKU-l2d3YOp9zcZDpw)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: FindByID(SwYk2yP4BIAY4yLurdrS8I5oOmDIVfMGLtKx44CNyW7Ww5okiT0a0l3Z-t9ifRdK2qjIBKU-l2d3YOp9zcZDpw)
  Output: Error (expected) - request failed with status 404: ��Record not found (expired)
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
Inserted: map[id:ummU5LKCDgMgpl7w4IYOFSqYVfZ8Enz92CFPZRvFWhO3tvXhdSQ8KiX63UBXTQJK2hIa0yefuEl44raeHaQxyQ]

=== Find by ID ===
Found: map[active:true categories:[electronics computers] created_at:2026-01-27T12:14:00-05:00 data:aGVsbG8gd29ybGQ= embedding:[0.1 0.2 0.3 0.4 0.5] id:ummU5LKCDgMgpl7w4IYOFSqYVfZ8Enz92CFPZRvFWhO3tvXhdSQ8KiX63UBXTQJK2hIa0yefuEl44raeHaQxyQ metadata:map[key:value nested:map[deep:true]] name:Test Record price:99.99 tags:[tag1 tag2 tag3] user_id:550e8400-e29b-41d4-a716-446655440000 value:42]

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 0
  active (Boolean): true
  price (Decimal): 99.990000
  created_at (DateTime): 2026-01-27 12:14:00 -0500 EST
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1 tag2 tag3]
  metadata (Object): map[key:value nested:map[deep:true]]
  embedding (Vector): [0.1 0.2 0.3 0.4 0.5]
  categories (Set): [electronics computers]
  data (Bytes): 11 bytes
Plain record: map[active:true categories:[electronics computers] created_at:2026-01-27T12:14:00-05:00 data:aGVsbG8gd29ybGQ= embedding:[0.1 0.2 0.3 0.4 0.5] id:ummU5LKCDgMgpl7w4IYOFSqYVfZ8Enz92CFPZRvFWhO3tvXhdSQ8KiX63UBXTQJK2hIa0yefuEl44raeHaQxyQ metadata:map[key:value nested:map[deep:true]] name:Test Record price:99.99 tags:[tag1 tag2 tag3] user_id:550e8400-e29b-41d4-a716-446655440000 value:42]

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-01-27T17:14:00Z] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:ummU5LKCDgMgpl7w4IYOFSqYVfZ8Enz92CFPZRvFWhO3tvXhdSQ8KiX63UBXTQJK2hIa0yefuEl44raeHaQxyQ metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Updated Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: kVovMcG-p-oppR2L6w6IZ8XMSUm-NwUVfFOYfexnQwnrhbeCRWCDG8UVYa-M3LKg_kcqddGm3PT9gu9YtCSBLA

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
Collection created with first record: 98WkOmhnrgKXtNiJJ4oZOj5KzJOslIfxkmkUpOk3iaqUsqxN_-QgdvidH79s2j4s3ICPGnshfJnLfS9UGbzKww

=== List Collections ===
Total collections: 14
Sample collections: [chat_configurations__ek0_testing test_accounts client_collection_management_go batch_users schema_users_client_py]

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
Created Alice: $1000 - ID: igChHyeFFVaRYwug9R18axCNQ4WbID7gfZMcNxf6HfyOoZopu__DSE3rePE9_pTp3_jqzlpqAFhMsDS0zjl9sg
Created Bob: $500 - ID: kyFsy2wvNL-hxy30Wfhe7VaVSJn5zUv_jppZ_qC_NrNNZp7kWpAAqQUzVU55QWBhhkqmdTZA9tx2RJnIaECQmQ

=== Example 1: Begin Transaction ===
Transaction ID: 83c3c198-e961-4071-8350-d8d32881544b

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 20f5fbc7-9287-47c0-87a5-86b2fa9215da
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
  1. Score: 0.740
  2. Score: 0.735
  3. Score: 0.734

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.694
  2. Score: 1.494
  3. Score: 0.296

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
✓ Inserted document: bdYPai_7UWmRTK2w5Y6Oc0M9A0DdVAIzJeAeWzZSvWBImVIe8HpFQ4Vh8mLIY8AiebsDvNFl72lLoUfF_zOm0w

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: aFF-w6FXHPzErMMEKoHdzTiciCxFxSuM5qRT1r7rJ-I93GYkYSIYbwCUFMg_cpA57BFkhLh1Ol88Lb1xBSzhoA

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
✓ Inserted document with TTL: oiwaL16RXa_QPEOnsBzKctPZ1JzXc_cl6-C16XzpOi4F58lAW8HLq-g0mVnbvD9Nsrt6UxYb1HroWijYDAX1uw

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
✓ Edge cache script created: Xc4_knI33YfIKGAMdy7v9MFpSkepviVCR0cOwdtV1LlTmNBbNeAj0qHd1T3Af-Vm9Khv5E1jr3UhsHaAQlTf3g

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

✅ Script saved: 9dOcO0mGfws1ZzPL7oEhDFVdnALbgd2wYgEn55U9vvIsNSUv8WUiBywOE6RXRwHGFjYxAdbJV8DJ6x32dJwlsA
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

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 2.529583ms
   📊 Records: 1
   📦 Data: {
        "value": {
          "address": {
            "city": "Gwenborough",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "street":...

Second call (cache hit - from cache):
   ⏱️  Duration: 2.56275ms
   📊 Records: 1
   📦 Data: {
        "value": {
          "address": {
            "city": "Gwenborough",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "street":...
   🚀 Cache speedup: 1.0x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

✅ Level 1 function: validate_user
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
🚀 ekoDB Go Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved: u90wEdYcBiZalZJ_3GEOlQ9hKFWGcDp45eaqcqWVhwh0U5aEcOnxDYlA_Orr_OCPOKtc3hUHKDLn2QzkeRCHGg
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

✅ Inserted order: gGID_rT4HSkta9GIAILksWDl4x_AaUXxVT_t5aPGxbsG1J-c0ZaIGfCc-YiHALBlO23XveaKnnwh5ZFum5AVGA
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: L6DIV0cPSmijCjVTgilz_ymw56_yGruHB583OAk8R9t8OM1OBHpT7AAr1IKp4iiKnwBmzYeDuODMlGGyZi_eKg
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: map[role:admin userId:user_abc]
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session
📝 Example 4: KV Operations in Scripts

✅ Script saved: yO7v6afzHhU5Y2Gy18rqguh0VTm02lxFBAZ_wQDQrWzGNIwOqm7xaO4J2w_0tiseh0RohpyoRUqbauJZaQqg4A
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Script

✅ Script saved: MDrPfBKAWzwroU5HzPxae3kuWCNVX83UqMW5J78KXkcmPYZpKwLzv4b_G7RkNxwJlvb35LEema1UXRuBYV53lg
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
✓ Created SWR script: fetch_api_user_go (nzLdpPBvyHXOOYgWqb94WT1HnjEkNuWvKJHTP6aDucNxCjhquuGxg4pPQLJt6oZ8VmN7AXGoYKr11jqN0nW66A)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "cached_at": "2026-01-27T12:14:03-05:00",
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
      "id": "vpLlOtsMmZVwh6VZ0lE7IDfRwE-gIRdnyPYCZ64R4DYEdKCz-6yECIaERHAue100B5XOV78zSvQXfOEyVOcXFQ"
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
Response time: 69ms (served from cache)
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
   map[avg_price:365.6666666666667 category:Furniture count:3]
   map[avg_price:367 category:Electronics count:5]
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
   map[count:7 status:active]
   map[count:3 status:inactive]
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
   1. map[type:String value:Vector Databases Explained] (map[type:String value:Database])
   2. map[type:String value:Database Design Principles] (map[type:String value:Database])
   3. map[type:String value:Introduction to Machine Learning] (map[type:String value:AI])
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
✓ Created session: H5TOVzzNxakRARI8q0N8aI6MXf9khSLO60NUFeIYlRi4rNsHQf9Ft3xqdUkG0HrNylE4YlWGD3w9dHAkqNo0Ig

=== Sending Chat Message ===
Message ID: XqC4fLEgOVhXr6NU-O9h31p41jdZchHhAL5f1K6ZomvpD8-7YrzHta4gNK57Lxgq2nMuuvsUyPQA3tU6a7Glsg

=== AI Response ===
There are three products available:

1. ekoDB Pro: This is an Enterprise edition product with advanced features. It is priced at $299.

2. ekoDB: This is a high-performance database product with AI capabilities. It is priced at $99.

3. ekoDB Cloud: This is a fully managed cloud database service product. It is priced at $499.

=== Context Used (3 snippets) ===
  Snippet 1: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Enterprise edition product with advanced features id:B7WzqLgE1h3OhaBOIV3FkGGV88aGicI1UPr4TDCOYmi-KVvfK3LKI0-JhoBVaRzXOjHmJ9D-zMCWnBDH1ky7BA name:ekoDB Pro price:299] score:0.1111111111111111]
  Snippet 2: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:A high-performance database product with AI capabilities id:kXOSGaVnZTkSDoB9tqr-jCNQq9qU9C0q8uGGWcHIdh1NpO_RSc6aXq44wLkm9E5p6BOdbrc0uQi27e4DBLQtsg name:ekoDB price:99] score:0.1111111111111111]
  Snippet 3: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Fully managed cloud database service product id:fJx5_OzOqFzqWoRoz6mOboyK1BGoDEOJlubWsONjlGa_AJL0gOQmGxPscT2WlUolTzsFta6dEfl6JRGujN4oyw name:ekoDB Cloud price:499] score:0.1111111111111111]

Execution Time: 3734ms

=== Token Usage ===
Prompt tokens: 463
Completion tokens: 78
Total tokens: 541

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: cGIBO-Oliy1lNywx4nmtO2uGn8-kIrRosNIF4_SjY_uJZIbTqhQT6013xQ_r4a8IPooUFX_IKv_UaYDT0NBbAA

=== Sending Initial Message ===
✓ Message sent
  Response: The available product based on the context provided is "ekoDB". It's a high-performance database product priced at $99.

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
✓ Created second session: vCj_jR9APFVECAEupGE-eShauY1CU4zi6rs881YufAEuruBTM5UiMLTrGofiOuPSc5t7QN0jtSJMOoTnG5FXbw
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
✓ Created session: -x18cYo6AhFsWkJX4FyQFZUmxRdlu3Y5twqrLatXf6lKNHYH1n0a78OTtK_NtEP8P6K8SOBqyWoId_5FT-fAJQ

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information available, the product that is available is "ekoDB". It is a high-performance database product priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB high-performance database product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: UnIEw6fAAanw8m8sPwlTwfU-ribmd8CXmFiD8n5-KEJMRTZ4UHgk2SSHdVFgFxaDjJuzfQuAq_-Ub9suoX9XBw
  Parent: -x18cYo6AhFsWkJX4FyQFZUmxRdlu3Y5twqrLatXf6lKNHYH1n0a78OTtK_NtEP8P6K8SOBqyWoId_5FT-fAJQ

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: UnIEw6fAAanw8m8sPwlTwfU-ribmd8CXmFiD8n5-KEJMRTZ4UHgk2SSHdVFgFxaDjJuzfQuAq_-Ub9suoX9XBw (Untitled)
  Session 2: -x18cYo6AhFsWkJX4FyQFZUmxRdlu3Y5twqrLatXf6lKNHYH1n0a78OTtK_NtEP8P6K8SOBqyWoId_5FT-fAJQ (Untitled)
  Session 3: vCj_jR9APFVECAEupGE-eShauY1CU4zi6rs881YufAEuruBTM5UiMLTrGofiOuPSc5t7QN0jtSJMOoTnG5FXbw (Untitled)
  Session 4: H5TOVzzNxakRARI8q0N8aI6MXf9khSLO60NUFeIYlRi4rNsHQf9Ft3xqdUkG0HrNylE4YlWGD3w9dHAkqNo0Ig (Untitled)
  Session 5: p4qIdxyzvVRIUsqZEFi6SQma6mUQhvAEuyfhx3AGbTcVcV_R_ZrS7yko8TuJm_H1dQAdckXP1b4GW6NMpUte0A (Untitled)
  Session 6: sAzMtmJ-PQw2AMEuPiO1cVavd-TPGz-NdoWzNaWrQwQC5YnvBxHARHZQzeL1qVMGkgyKyhSGl-4SFyQ4Yp8K6w (Untitled)
  Session 7: wI1DCXeEsJdeUO_kt02FRPdNDwDBsstg9B7FWw2CKRTfOPdsGlg_28kcDF5_qt7kKB0d1QzG3ChFwb0qYkflbw (Untitled)
  Session 8: -X3r6ftFEexwrKAvlzkZrkxeHh-JiNF87XeKMozCByA1f4epxPc0neizTsZUeZo--ymhrhF980pm_eMdeb0n3w (Untitled)
  Session 9: MFbKEwGka_F5zUqzfAjBoCmSLbdAGQnNUFluQGZQBVOpiWcYKxRFe4Oy3bwrvytTuJMcL4AkRMra-kseAeNFWA (Untitled)
  Session 10: O5WSPWnxYUn00jIQyOvwuAX84ZXyGYnF_HOnwe0EBVt-TTf9oMr7iziaWfFbk57WXUb3Xd3_RSNC0h1XSgmiSg (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: UnIEw6fAAanw8m8sPwlTwfU-ribmd8CXmFiD8n5-KEJMRTZ4UHgk2SSHdVFgFxaDjJuzfQuAq_-Ub9suoX9XBw

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Map Creation ===
✓ Created record with native map: map[id:aNjMJ1beQwQ7m1Y3GuYe6rKhLblDIU7ff__NkIRbtPdppUyuarqfeF1DZtyBEOy5Rr2T4-ze8Wa2R3PeJgFcWg]

=== Upsert Operation ===
✓ First upsert (update): map[active:map[type:Boolean value:true] age:map[type:Integer value:29] email:map[type:String value:alice.j@newdomain.com] id:aNjMJ1beQwQ7m1Y3GuYe6rKhLblDIU7ff__NkIRbtPdppUyuarqfeF1DZtyBEOy5Rr2T4-ze8Wa2R3PeJgFcWg name:map[type:String value:Alice Johnson]]
✓ Second upsert (insert): map[id:zjP4jpEI9qi0V8dUNitVqeCkd_oZ3nzPbcyn5tXflIS3qLOv0JEDpeoyo4kngdsp_Aa-TtkhNQJG36slzPO3yA]

=== Find One Operation ===
✓ Found user by email: map[active:map[type:Boolean value:true] age:map[type:Integer value:29] email:map[type:String value:alice.j@newdomain.com] id:aNjMJ1beQwQ7m1Y3GuYe6rKhLblDIU7ff__NkIRbtPdppUyuarqfeF1DZtyBEOy5Rr2T4-ze8Wa2R3PeJgFcWg name:map[type:String value:Alice Johnson]]
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
Inserted with ripple: map[id:kaBrezMLia5UfsuGuT3AO8og_KkYqaivzfOl7WV0hJ3KFhrDAZQLHOotzTlsHYWcwsXQTQKyzA93ybMjzD8cIQ]
Inserted with bypass_ripple: map[id:NDQtdw8dFb67SyXs_3zQHMy8AEOrnZASZUlkD9hAjXxqY2Bfhln2BLSINs_JgSfYu2VfAlzENEYwuzRzZag_eg]
Inserted with TTL and bypass_ripple: map[id:gm-FmWhl8ocvzFM6ltYNAMIeh8CSbAopI4nZ4hUbmF0MAHrILtg-XXPnEp94qJJ1S07uH9gh2Al1je9tQp3u4A]
Updated with bypass_ripple: map[id:kaBrezMLia5UfsuGuT3AO8og_KkYqaivzfOl7WV0hJ3KFhrDAZQLHOotzTlsHYWcwsXQTQKyzA93ybMjzD8cIQ name:map[type:String value:Product 1] price:map[type:Integer value:150]]
Deleted with bypass_ripple
Batch inserted with bypass_ripple: 2 records
Upserted with bypass_ripple: map[id:39xtAUWA_RvrbYFbJ0_g9DGE-0WuzE9hfQjYguLlNoUkHDAUzkrQBQ9nWL0LyOxGeO5R8yK8mYVUJqoF5qd4Hw]
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields
Fetched 3 users with only 3 fields each

Example 2: Exclude sensitive fields
Fetched 2 admins without sensitive data
  Password field excluded: false

Example 3: Complex query with projection
Fetched 3 active users with profile fields

Example 4: Find by ID with projection
Fetched user profile: Alice Johnson

Example 5: Compare full vs projected data
Full query returned 12 fields per user
Projected query returned 12 fields per user

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
✅ [32mGo client examples complete![0m
