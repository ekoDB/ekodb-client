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
Inserted: map[id:GtvC4o3Bc5mJnTAMgN9xCC0JrjRr--K1RCJgqrqWPNIAx7pQ3_U_KAOsD4Y73NopJbldHQdHnNluXEktQYgHJg]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] id:GtvC4o3Bc5mJnTAMgN9xCC0JrjRr--K1RCJgqrqWPNIAx7pQ3_U_KAOsD4Y73NopJbldHQdHnNluXEktQYgHJg name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query ===
Found documents: map[]

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] id:GtvC4o3Bc5mJnTAMgN9xCC0JrjRr--K1RCJgqrqWPNIAx7pQ3_U_KAOsD4Y73NopJbldHQdHnNluXEktQYgHJg name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
[32m✓ simple_crud.go completed successfully[0m
[34m
=== Running simple_websocket.go ===[0m
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: wOhw36APY7lqulIUv8F8sgCT5KKowLEVn1StWe5ITSbhiGFihQjS0yKngcE0-d6Pqs0oe1bokPr8hlOsg3kr1w

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
        "id": "5yKdWQgJ0eshje0NLVcqSLxJGHZEoxIMmSlenatXgi9XL08hgn5_57TrAMXSKiD0F8B_VHoOI3uK8BOuAr4dVQ",
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
        "id": "NijRrca-9YBQBeXU6j280I4rPcUwOpFnrXCbwAWzKK-LMaIattQNOpF3zXfRLzEbyzyKyf4XMcJ6kC1iBfqmHw",
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
        "id": "wOhw36APY7lqulIUv8F8sgCT5KKowLEVn1StWe5ITSbhiGFihQjS0yKngcE0-d6Pqs0oe1bokPr8hlOsg3kr1w",
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

✅ Script saved: WfrHxBOLEk6l9FCCGEpqMyCpkDGJbf3Wt7Bgt4jaL3TGXi4NfP6g20mI-RUwewHQXD937EzlMYqjnbCceu61dQ
📊 Found 10 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: M1cWUj1uxy8xTE-mGatv1xPNRL9QhHtR1W8pj3nsjFrHB7UxvrmuLo2BSyfzWEpAbTpMCxhvDMFGdTNlX14-3Q
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: vUmiTU4O5NyergoE0FKzK38guyBvff0kqBGmDOuf-AbVRw5UrgXtF6DsJyR5BFNs-JCC9TlG3lH1rQzAE6qcPg
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
Collection created with first record: -M1C0L_gKDgCKxu7SYtfOrTTEh2rK1z1lw36SJSe9K9-U6tldqm30O5rUztaHun75V-k0kBFDv3qC67IteUd2A

=== List Collections ===
Total collections: 17
Sample collections: [schema_products_client_py batch_users ws_ttl_test scripts__ek0_testing github_cache]

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
Created Alice: $1000 - ID: YfnT1BKDk6F0Piu0TmhZEklRyO6JZGeb7HK7EEviqSgbPU0QhkGqV6JbHi1WwqQ83SuwmZQZ3oBu4VnExqMRNw
Created Bob: $500 - ID: S-SGjflhx8vINIG0ELSTVtvtYbWMBJzZgBPqyMp8t9XCka236mPcG-MgJLn_dOYFsgddFSbrZuXUDrTb28TGEg

=== Example 1: Begin Transaction ===
Transaction ID: c063f661-2246-44c2-8bbd-f55d438137a5

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
New transaction: 10cdd397-eebc-49b5-9ca4-988365ee9324
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
   ✅ Script saved: DXACOIbS7F5bJcv527cxaRp4sFNJOLncanEwYWihpczEs8hv4Xcdcx_UsN4SXoe5EShloFgde0umCQO9e8nb1Q

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: a_SLe39ycIQtKONzpi31IbaXudH8ENMjySEAVe-QmTvj6UnIdxq_uH0GiJ6Kxx9a8l_M9c3PHnIbwuS2XmjAXg
   📋 Name: map[type:String value:Alice Smith]
   📋 Email: map[type:String value:alice@example.com]
   📋 Status: map[type:String value:pending]
   📋 Credits: map[type:Integer value:0]

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: KU3pQjjM3CRfUr2GrNa83S2O2nsEGJwhiAW3I9pu0QS2zAXGd4fGy12zX7LpPPPCXRPPiVr-m2zWKuzYISM9pQ

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
   ✅ Script saved: nXVwp-xC9X5hrBkJk5YEJg1bqD3KeOUqXn6XDiukN94_5t8hK6T-XDuLEapRNWtVAXRa8bpeRBRdWf4UDbGxog

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
   ✅ Script saved: FmN3GhptZAeBewb4gJHLZxhh6WVr7lHqN2QVsnuwJS8FqeunVTCce8LKeQ1KMHR6RlKGDNONBKKY7-FvOXgRyA

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: DXACOIbS7F5bJcv527cx...
   ✅ Deleted script: KU3pQjjM3CRfUr2GrNa8...
   ✅ Deleted script: nXVwp-xC9X5hrBkJk5YE...
   ✅ Deleted script: FmN3GhptZAeBewb4gJHL...
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

2026/01/06 11:11:09 No .env file found, using defaults
✓ Client connected

═══════════════════════════════════════════════════════════
TEST 1: Document TTL Expiration
═══════════════════════════════════════════════════════════

[Step 1] Insert document with 3 second TTL
  Input: {name: 'TTL Test', value: 'should expire'}
  TTL: 3s
  Output: Document ID = j0vcTiKIMl145jHOvy9Zh_CzTvgKOjgy0PqtcVEr9_qtlcmKEJvnrX_mlaC7Bgz7qIO_wSq_TrMw2YJjr8AiFg
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: FindByID(j0vcTiKIMl145jHOvy9Zh_CzTvgKOjgy0PqtcVEr9_qtlcmKEJvnrX_mlaC7Bgz7qIO_wSq_TrMw2YJjr8AiFg)
  Output: Found document with name = map[type:String value:TTL Test]
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: FindByID(j0vcTiKIMl145jHOvy9Zh_CzTvgKOjgy0PqtcVEr9_qtlcmKEJvnrX_mlaC7Bgz7qIO_wSq_TrMw2YJjr8AiFg)
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

2026/01/06 11:11:16 No .env file found, using defaults
✓ Client connected

═══════════════════════════════════════════════════════════
TEST: WebSocket TTL Expiration
═══════════════════════════════════════════════════════════

[Step 1] Insert document with 3 second TTL
  Input: {name: 'WS TTL Test', value: 'should expire'}
  TTL: 3s
  Output: Document ID = 0b9zite4PD1qms7TM_TQkxzN796qCKFJOKStOwrP-WFbmjC8ui8Mq2J3Vu-mhv5XYgKMtYWiIF5iHvtLHD2vug
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: FindByID(0b9zite4PD1qms7TM_TQkxzN796qCKFJOKStOwrP-WFbmjC8ui8Mq2J3Vu-mhv5XYgKMtYWiIF5iHvtLHD2vug)
  Output: Found document with name = map[type:String value:WS TTL Test]
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: FindByID(0b9zite4PD1qms7TM_TQkxzN796qCKFJOKStOwrP-WFbmjC8ui8Mq2J3Vu-mhv5XYgKMtYWiIF5iHvtLHD2vug)
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
Inserted: map[id:oVViHmI8H8ncZ2xskStkzLGfqiZ2-j0sU_YsNmbXVrqEW_NAjTgpcorVC21E9wQ0uPShFIk2D0Fk4EVOtf5AQg]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-01-06T16:11:23Z] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:oVViHmI8H8ncZ2xskStkzLGfqiZ2-j0sU_YsNmbXVrqEW_NAjTgpcorVC21E9wQ0uPShFIk2D0Fk4EVOtf5AQg metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Test Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:42]]

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 0
  active (Boolean): true
  price (Decimal): 99.990000
  created_at (DateTime): 2026-01-06 16:11:23 +0000 UTC
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1 tag2 tag3]
  metadata (Object): map[key:value nested:map[deep:true]]
  embedding (Vector): [0.1 0.2 0.3 0.4 0.5]
  categories (Set): [electronics computers]
  data (Bytes): 11 bytes
Plain record: map[active:true categories:[electronics computers] created_at:2026-01-06T16:11:23Z data:aGVsbG8gd29ybGQ= embedding:[0.1 0.2 0.3 0.4 0.5] id:oVViHmI8H8ncZ2xskStkzLGfqiZ2-j0sU_YsNmbXVrqEW_NAjTgpcorVC21E9wQ0uPShFIk2D0Fk4EVOtf5AQg metadata:map[key:value nested:map[deep:true]] name:Test Record price:99.99 tags:[tag1 tag2 tag3] user_id:550e8400-e29b-41d4-a716-446655440000 value:42]

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-01-06T16:11:23Z] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:oVViHmI8H8ncZ2xskStkzLGfqiZ2-j0sU_YsNmbXVrqEW_NAjTgpcorVC21E9wQ0uPShFIk2D0Fk4EVOtf5AQg metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Updated Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: eeuT0NsnhfNX0O6W3_bH-25pa_Ea8Umx6qdYBNqJUnSznkaykfUyFlEO1nA7t_13z5nidJJgqNgtQYnxDO7Mbg

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
Collection created with first record: af2BT3SbLVWqtk1U5TSB_BkytuJHO0MjWeO7Dv4SpG1-B7jpl_e_d_oj-gIst6vlsLrcOKtmxBTynymOx1HZqA

=== List Collections ===
Total collections: 16
Sample collections: [schema_products_client_py batch_users ws_ttl_test scripts__ek0_testing github_cache]

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

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: map[name:Product 1 price:29.99]
cache:product:2: map[name:Product 2 price:39.989999999999995]
cache:product:3: map[name:Product 3 price:49.989999999999995]

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 3 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 4

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: false

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: liYne5FCqips4jUB1rVW2IUPOOs7rQ48NatyEhTQxvNTZUji8O6DQNQGd23txgKMvX7gW3QVKV_MwEo-nGbE1g
Created Bob: $500 - ID: 0giMybQdb9faSGNV-z1eU1hr-1imxdDLCJbcSrUJIYTyLzJdzkv58Zbh-FBnj6kFunmG8wcjPYDK7UJ-1Ea2sw

=== Example 1: Begin Transaction ===
Transaction ID: 76c44efb-9996-48f1-bd42-a0b9618683a1

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: bdfbbd94-6a2b-46c8-a4d8-bfb3eccf78d6
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
  2. Score: 0.762
  3. Score: 0.730

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.692
  2. Score: 1.511
  3. Score: 0.305

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
✓ Inserted document: oBeM1aNMsiHxrJLdwxGTKTBKEAKsjt-ZYlFmxN-ayUCInB8_Ib8rqf7J8CKoUH4EP4wDcSq-TgHeM4LPttrKXg

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: fzaLSga9Wsl1gqKNgMBABjl2GV0DQOXQc8KyaPiS0AeM764MWvPkzOEA4yaeWXNLeABiNEeI1KdNFRoqRaIfAA

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
✓ Inserted document with TTL: rQaWiGqxdaEIYLY9521ihhBDBuxGcZ9kDXsvLZtWkeBBuxyJBA8LPAwGJZBNRvfS5uPo7Z5q-pmceFNzW7Z7VA

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
✓ Edge cache script created: QU_fGh9Wl0NekHYRi1VKJDmIZ1BKAQZnFX_1vWTLmAh8owDyZ7y3sI0-u5_KIZWDzVCVLz96g77bXqCozUTG_Q

Call 1: Cache lookup
Response time: 1ms
Found 1 cached entries

Call 2: Cache lookup (connection warm)
Response time: 1ms
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

✅ Script saved: 1cLf-HMj8QqaESy4YplqtUAOo9ibGH_sED_52K2YtyZPNdNAdSAsIW2uskB9wUvEgKz22TYgXCAFr-6fWKhWkw
📊 Found 10 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved
📊 Found 10 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved
📊 Statistics: 2 groups
   map[avg_score:60 count:5 status:active]
   map[avg_score:50 count:5 status:inactive]
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
   ⏱️  Duration: 127.805417ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 84.312084ms
   📊 Records: 1
   🚀 Cache speedup: 1.5x faster!

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

✅ Script saved: 1lsr8xJttUkCkSlifxNwQCg64iWmZqT4GWbvULM9Aw654BqLHbCfObDeIHrPTICgS0nA-AzBSF7hAqJA9LdJVw
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

✅ Inserted order: Am1V7jZt0Rj2GSGQfU_e7AQ5z5oOSez2MY3JnA_EmZ8apmATJSg-PzCgRqUl1445AWD2U9PNo-RTsIXUlwij-w
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: BvFoFdMUeY-alGxirGL1lVx2hXTBPn1BDoQRfobjjULiMOPncyATb7xzTXfOxKT7IIp-67MpuWTKfdWQHguGoQ
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: map[role:admin userId:user_abc]
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session
📝 Example 4: KV Operations in Scripts

✅ Script saved: 5WmFdvBTlYH_slC66OUQFn4A1lYZLicFyVmYZksVffjB9BaF7TC8l9-XHc_kR5pvnnzmN5ZSeJAC9tCfSjTUgg
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Script

✅ Script saved: MLepfCjB1LX_YaMpFKQD-jPHwiUoyTlGJG7q9MvdxTYoqviCGPegER0x_XfH2KZi4cbsC05tltGpC0BHnNlxwg
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
✓ Created SWR script: fetch_github_user_go (uf1ZzsAV_DvEP4fkFB1uHN2Yt0lMQQEiLw_1l6TyMnqsXeiOBl1FEvRPU6DVMywVRsQ5_P8Ya22qR4_w3OREBQ)

Step 2: First call - Cache miss, fetches from GitHub API
Result: {
  "records": [
    {
      "cached_at": "2026-01-06T11:11:26-05:00",
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
          "followers": 270299,
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
          "public_repos": 9,
          "received_events_url": "https://api.github.com/users/torvalds/received_events",
          "repos_url": "https://api.github.com/users/torvalds/repos",
          "site_admin": false,
          "starred_url": "https://api.github.com/users/torvalds/starred{/owner}{/repo}",
          "subscriptions_url": "https://api.github.com/users/torvalds/subscriptions",
          "twitter_username": null,
          "type": "User",
          "updated_at": "2025-11-24T04:16:14Z",
          "url": "https://api.github.com/users/torvalds",
          "user_view_type": "public"
        }
      },
      "id": "IAMfC6bghidublLByrRDNUi6O1FZtDvyuMvwGnMDcdNd01vbzRWi_Hk8kUd_Yhx7suzLP3j3pj2i-ZrDBRzdIA"
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
Response time: 71ms (served from cache)
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
   1. map[type:String value:Introduction to Machine Learning] (map[type:String value:AI])
   2. map[type:String value:Natural Language Processing] (map[type:String value:AI])
   3. map[type:String value:Getting Started with ekoDB] (map[type:String value:Database])
   4. map[type:String value:Database Design Principles] (map[type:String value:Database])
   5. map[type:String value:Vector Databases Explained] (map[type:String value:Database])
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Script saved
📊 Documents by category:
   map[category:AI count:2]
   map[category:Database count:3]
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: X9t5cttsehSZsRMNaQJaFNuP86HLbljgFO9jHItG45p3NzgjeeiOKF4tY97d975r8UT0y_xwUsC38ihAeBwYgg

=== Sending Chat Message ===
Message ID: DzS7bCJTXxuAEkfr5a4Wb_TCSHTtsvIxkPKu9O8FHilnx6IJbzN9lkrivr7WY9LNdgKy_fl6Y0aGeJFJ01b3tQ

=== AI Response ===
The available products are:

1. ekoDB: This is a high-performance database product with AI capabilities. It is priced at $99.

2. ekoDB Pro: This is the enterprise edition product with advanced features. It costs $299.

3. ekoDB Cloud: This is a fully managed cloud database service product. The price for this product is $499.

=== Context Used (3 snippets) ===
  Snippet 1: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:A high-performance database product with AI capabilities id:a5X_bVCevT_5kT-9kc08WXQqzGPQTl3xGb6Qik-ZXHFmU7EiROindGn3EHFP-MzWqUnn_VlNUMYdcTpEbYJNzw name:ekoDB price:99] score:0.1111111111111111]
  Snippet 2: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Enterprise edition product with advanced features id:cvmwX0VwJ2UpzpFd4wvVrqnNGNHZdBRzhvlSrCZUArJ9w_C-_BkQA4EI9z3IEb3Bt9eDlTMQnFxrm0IMI5dSiw name:ekoDB Pro price:299] score:0.1111111111111111]
  Snippet 3: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Fully managed cloud database service product id:1cfsKrkp0eVU8KS8jraNI6YnwDkmpUPHj8GQ9_JF-TDn0P0FzdD-ZGEnFQ6NZyG7vJ6njeKsoowrcnmrc_D7vg name:ekoDB Cloud price:499] score:0.1111111111111111]

Execution Time: 3123ms

=== Token Usage ===
Prompt tokens: 455
Completion tokens: 77
Total tokens: 532

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: sK6VuJhhK34wgmqXdtlIjrsmMj_aX7Or3SxLW2e3t2WDM5u6mujVgEdEbB52Q5WCCcP-61rjaxlEbK2IKB38og

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the context provided, the available product is "ekoDB", which is a high-performance database product. It is priced at $99.

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
✓ Created second session: EB3jEuzG-LxkynD65O1rXeJuvg8QIZkvVwPEHVz2k56v6PiRdnSL7oN61GeamNEYGSrhJJXdcEuHNcMobxBfPg
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
✓ Created session: hdaG0aO5VXgNNA9ja6-NDcERT20eKc8L-AlFO9_tsCHN8TU2CPV530YAWqRxBzziO26QIQc7jX6oPB2c88mMmg

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the provided context, one product available is ekoDB. It's a high-performance database product priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: GICodxKr0jQ8jniyEi_Rz5fHY22XdSnRqdLCOU0PGdm2XU_7JZ_X3rw9ZcpIQGuukUSd1m2_vAN57IrTnMrmbA
  Parent: hdaG0aO5VXgNNA9ja6-NDcERT20eKc8L-AlFO9_tsCHN8TU2CPV530YAWqRxBzziO26QIQc7jX6oPB2c88mMmg

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: GICodxKr0jQ8jniyEi_Rz5fHY22XdSnRqdLCOU0PGdm2XU_7JZ_X3rw9ZcpIQGuukUSd1m2_vAN57IrTnMrmbA (Untitled)
  Session 2: hdaG0aO5VXgNNA9ja6-NDcERT20eKc8L-AlFO9_tsCHN8TU2CPV530YAWqRxBzziO26QIQc7jX6oPB2c88mMmg (Untitled)
  Session 3: EB3jEuzG-LxkynD65O1rXeJuvg8QIZkvVwPEHVz2k56v6PiRdnSL7oN61GeamNEYGSrhJJXdcEuHNcMobxBfPg (Untitled)
  Session 4: X9t5cttsehSZsRMNaQJaFNuP86HLbljgFO9jHItG45p3NzgjeeiOKF4tY97d975r8UT0y_xwUsC38ihAeBwYgg (Untitled)
  Session 5: UPpvdwRwbg5gporVh95VhM_j6mgk-bEi5bO1tWemG0l-v_l3EIujQD_Te-tr_i5kQcURMoyMQDaUMQSMrH_2aw (Untitled)
  Session 6: Vwv_dS-QtANBp_pTXuP31-g0Z56KtYJkKqzGlcNeqgBv68RCtjOiUv2gnGQKhO03ZeTUkAZZVX49FONaJsaS-w (Untitled)
  Session 7: ld9rVKp4aH4JOkUnFVYGKj4j8r2wHCUzMhFDp6pPQ_uKiRHIPwF3xlBjj_jGZnNHXzBvJCFoL7eFsx1FGxpNYw (Untitled)
  Session 8: jSz7-Rn7Gm2dKTCBQP-Na7ElcHYIhA-KdHJAC4xfo_HHUe9692grvVeO8tsjgtikoNoDpkcoo-Y6Pr3IWt5sxQ (Untitled)
  Session 9: q06liewa9ga2qYakyZ96N2GGsnUAwOqtrCRMLPHiWGhF1EO3wrHr_bbAT4z9mta5NnhSvf-MPBdyCFSYGNIzqw (Untitled)
  Session 10: FbI9y0Ydck_SMHJ_B-YqCwwxEsLnxwrSKcGyg2Kr74MXEWlYYCdWZ_1x1xSwLF5OunKuwVUWMmSzsLvZo_3JOw (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: GICodxKr0jQ8jniyEi_Rz5fHY22XdSnRqdLCOU0PGdm2XU_7JZ_X3rw9ZcpIQGuukUSd1m2_vAN57IrTnMrmbA

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Map Creation ===
✓ Created record with native map: map[id:xeUcSaOVntITF4gxRTEQSjhro_k2YnOKnB2BRTNU80FvghweSGzbxc0hsZ_Hd_YP2-LHdjKeTmjBKURwbENYDQ]

=== Upsert Operation ===
✓ First upsert (update): map[active:map[type:Boolean value:true] age:map[type:Integer value:29] email:map[type:String value:alice.j@newdomain.com] id:xeUcSaOVntITF4gxRTEQSjhro_k2YnOKnB2BRTNU80FvghweSGzbxc0hsZ_Hd_YP2-LHdjKeTmjBKURwbENYDQ name:map[type:String value:Alice Johnson]]
✓ Second upsert (insert): map[id:tGQRoAnUZV1PNoeGZapmE3zL5YAqEKaephnZf5UqsCAwjgwsUBeBGqQHNUxKIzTQjWSab5LACg9NSBkqroOqmQ]

=== Find One Operation ===
✓ Found user by email: map[active:map[type:Boolean value:true] age:map[type:Integer value:29] email:map[type:String value:alice.j@newdomain.com] id:xeUcSaOVntITF4gxRTEQSjhro_k2YnOKnB2BRTNU80FvghweSGzbxc0hsZ_Hd_YP2-LHdjKeTmjBKURwbENYDQ name:map[type:String value:Alice Johnson]]
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
Inserted with ripple: map[id:bG903Jycy5xUgCtu0-u1SbpBE52R2h0fAyO7x0XRwetOHI107Cteh4bNG3hnUpsgz85gEgzqiuaLlM68eojD3w]
Inserted with bypass_ripple: map[id:PR8v2Q175HDtiRuxwvq-1VdBafAf16hV9P8uSsU2dpWgA_TAVwReDo-R9jcEUrZ8ZdQCingxh4U4u-GniQspQg]
Inserted with TTL and bypass_ripple: map[id:7oyKwdNqr1Wx-I8GMa8J-BRax6v8x29CwfJbBpPTAo7o-17ppxGg0TQlnfpRaDUoBHNtrlZRueyPx7hT4XMdow]
Updated with bypass_ripple: map[id:bG903Jycy5xUgCtu0-u1SbpBE52R2h0fAyO7x0XRwetOHI107Cteh4bNG3hnUpsgz85gEgzqiuaLlM68eojD3w name:map[type:String value:Product 1] price:map[type:Integer value:150]]
Deleted with bypass_ripple
Batch inserted with bypass_ripple: 2 records
Upserted with bypass_ripple: map[id:j2rmCXDLAcarVN974h-8E7X92yH6NlJcH9syeC0WnE_aYVFEzlZ8P7BVVazYCD3IIHqJtPMOCBlZwk0N1is3rQ]
✅ [32mGo client examples complete![0m
