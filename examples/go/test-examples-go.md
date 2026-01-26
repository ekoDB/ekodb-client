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
Inserted: map[id:gmUuTTcFZNq-5zDauOvuXZuRW6bcLhzyg4OHkYAjmkxJersD6c52UP-z_I6BOive3df7OrJGY_LAwQFFGOQQMQ]

=== Find by ID ===
Found: map[active:true id:gmUuTTcFZNq-5zDauOvuXZuRW6bcLhzyg4OHkYAjmkxJersD6c52UP-z_I6BOive3df7OrJGY_LAwQFFGOQQMQ name:Test Record value:42]

=== Find with Query ===
Found documents: map[]

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] id:gmUuTTcFZNq-5zDauOvuXZuRW6bcLhzyg4OHkYAjmkxJersD6c52UP-z_I6BOive3df7OrJGY_LAwQFFGOQQMQ name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
[32m✓ simple_crud.go completed successfully[0m
[34m
=== Running simple_websocket.go ===[0m
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: HcXTmrgLhE2Sv7O9HhkC6tf7c-SDSMXvqOTzKbm5zA0faWPThx9PS56pVP4iX3g7tWPKZeBg9R5v8p_06zrFIg

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
        "id": "CUYleyzUq5ERksJhs2_ex0xZMacukDb7Fahybmg6_kYNBoaiKUftv-iDrdcvmQeuPACaYe-iqENgAa27grrarg",
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
        "id": "BRWJqswv00Jc4ldtHNBYkzPv1XIMTVaTFLjDm0FPXH0wMIOwGp6SL1oZIOUqlv30gPRzSi7wdyjSsdR6-gH8Tg",
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
        "id": "HcXTmrgLhE2Sv7O9HhkC6tf7c-SDSMXvqOTzKbm5zA0faWPThx9PS56pVP4iX3g7tWPKZeBg9R5v8p_06zrFIg",
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

✅ Script saved: upCvXRCK6gfIJK_zD0Mf9d5crTs1oeldFD2O9Y7FM_kTsi8I5SRgAOGh7YNBq61rll2qEUJLCU6_fjOJHcY0Ow
📊 Found 25 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: hR8woPxLICq0MeAcBYuHhTaTCuCEIkrC0AZNUCsAK9mVapoAyzNIPeryvccXdgsTLhJnGI0ob-6Dszi5qDLOVQ
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: 1TheM3q1iVbbSPB5aJDtETp3Lc_r-bDH-5TXFsiQOJhc6pcgj9dSUWL6ZO_A2oZu0_2hmo8nagIGvnjvH9dm8g
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
Collection created with first record: NqXppQAkn9qc4axz4G1gv8UImBv92RZr3CgX4Eobs0mhAKr6imM6OyX4zxEQM67Ew5GNo5_XmV5aLHoZWUTUIw

=== List Collections ===
Total collections: 15
Sample collections: [ws_ttl_test test_collection chat_configurations__ek0_testing demo_collection scripts__ek0_testing]

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
Created Alice: $1000 - ID: r-23f9Q1LpWjjVBAAAVej2xwGqk7i2UEc2KNPbazjV5NVae5Upi0Aqbsa_1NxptJEtMWaRsqOGO5xDTsG4Gt7w
Created Bob: $500 - ID: ufMUz98_3oF35DNMN2ju_it_S2fxJnVI2ZjqLAvdOnpGXPJMhYQBfoZ-MxtxnLRREl_oNuBFZTblDg47g3xbqg

=== Example 1: Begin Transaction ===
Transaction ID: 4af6fb76-a5f1-4fa1-8f38-11e75a0a4646

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
New transaction: 828999fa-be34-4994-bbd1-25b0a6647fae
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
   ✅ Script saved: gPkKTto2rnHnCh1rv7JuPSfQb_Rpefj8dOS2AKKe4n8H-AP6JDddsZuM0PiDAmxFjsn3wedTzBxAqY0evgDcfw

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: cDSE4GXpjsRj0ophTNrxhg7zS4rPMEftOegmbTeulQt5AWUVtM4UyCp6vl31nO-B_DIQCxKICAeJg4e5Xbm21Q
   📋 Name: map[type:String value:Alice Smith]
   📋 Email: map[type:String value:alice@example.com]
   📋 Status: map[type:String value:pending]
   📋 Credits: map[type:Integer value:0]

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: wlA9W_M7bwckQv_YgUqdLs9N_Fh5wOp0-OBQZJ1dM2mxMp7uUFajGEHTByQTfLLxG6IvXasZzl8HTDFWnx-qHA

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
   ✅ Script saved: hzc4lZDaS3NxhXivb5uoN8VN7sMEmimNuVRHEKkriOFaSPvunH_zvpCCUwkbqY7w7yPrny_kUxAGbY8uoHtv7g

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
   ✅ Script saved: g9anC-el_dzGwkAJ0ZafdCPIs05kTGu5I5vLEVe_Xrftt-CP2xDcl1mPLbqnJpKjkzXLh2UctxmJo7oYH4j1FQ

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: gPkKTto2rnHnCh1rv7Ju...
   ✅ Deleted script: wlA9W_M7bwckQv_YgUqd...
   ✅ Deleted script: hzc4lZDaS3NxhXivb5uo...
   ✅ Deleted script: g9anC-el_dzGwkAJ0Zaf...
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

2026/01/26 18:25:06 No .env file found, using defaults
✓ Client connected

═══════════════════════════════════════════════════════════
TEST 1: Document TTL Expiration
═══════════════════════════════════════════════════════════

[Step 1] Insert document with 3 second TTL
  Input: {name: 'TTL Test', value: 'should expire'}
  TTL: 3s
  Output: Document ID = VrLsvY9SxtZJl-ecQeEhDj93wXEBsbM1mrkb-cfR3IypyucJr87JnCwyqP0ZbkxOaLOOknL2kJlofwYHY5tNIA
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: FindByID(VrLsvY9SxtZJl-ecQeEhDj93wXEBsbM1mrkb-cfR3IypyucJr87JnCwyqP0ZbkxOaLOOknL2kJlofwYHY5tNIA)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: FindByID(VrLsvY9SxtZJl-ecQeEhDj93wXEBsbM1mrkb-cfR3IypyucJr87JnCwyqP0ZbkxOaLOOknL2kJlofwYHY5tNIA)
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

2026/01/26 18:25:13 No .env file found, using defaults
✓ Client connected

═══════════════════════════════════════════════════════════
TEST: WebSocket TTL Expiration
═══════════════════════════════════════════════════════════

[Step 1] Insert document with 3 second TTL
  Input: {name: 'WS TTL Test', value: 'should expire'}
  TTL: 3s
  Output: Document ID = Xkk68ityd0u2IvlElPGPcHrMET1paIESyMgZtl5sOM-mvQUGrsUlUEJAqbAZb9znMyET-2HfnbRn275sTzG3SQ
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: FindByID(Xkk68ityd0u2IvlElPGPcHrMET1paIESyMgZtl5sOM-mvQUGrsUlUEJAqbAZb9znMyET-2HfnbRn275sTzG3SQ)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: FindByID(Xkk68ityd0u2IvlElPGPcHrMET1paIESyMgZtl5sOM-mvQUGrsUlUEJAqbAZb9znMyET-2HfnbRn275sTzG3SQ)
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
Inserted: map[id:wqGC9GD0wCT18CYVpXuFnKqOVB_J0ezIahWLEQs90Rgj7--1450XudXBLxu1z3xxF3301Fr4zuUyg3X2jbGviA]

=== Find by ID ===
Found: map[active:true categories:[electronics computers] created_at:2026-01-26T18:25:20-05:00 data:aGVsbG8gd29ybGQ= embedding:[0.1 0.2 0.3 0.4 0.5] id:wqGC9GD0wCT18CYVpXuFnKqOVB_J0ezIahWLEQs90Rgj7--1450XudXBLxu1z3xxF3301Fr4zuUyg3X2jbGviA metadata:map[key:value nested:map[deep:true]] name:Test Record price:99.99 tags:[tag1 tag2 tag3] user_id:550e8400-e29b-41d4-a716-446655440000 value:42]

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 0
  active (Boolean): true
  price (Decimal): 99.990000
  created_at (DateTime): 2026-01-26 18:25:20 -0500 EST
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1 tag2 tag3]
  metadata (Object): map[key:value nested:map[deep:true]]
  embedding (Vector): [0.1 0.2 0.3 0.4 0.5]
  categories (Set): [electronics computers]
  data (Bytes): 11 bytes
Plain record: map[active:true categories:[electronics computers] created_at:2026-01-26T18:25:20-05:00 data:aGVsbG8gd29ybGQ= embedding:[0.1 0.2 0.3 0.4 0.5] id:wqGC9GD0wCT18CYVpXuFnKqOVB_J0ezIahWLEQs90Rgj7--1450XudXBLxu1z3xxF3301Fr4zuUyg3X2jbGviA metadata:map[key:value nested:map[deep:true]] name:Test Record price:99.99 tags:[tag1 tag2 tag3] user_id:550e8400-e29b-41d4-a716-446655440000 value:42]

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-01-26T23:25:20Z] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:wqGC9GD0wCT18CYVpXuFnKqOVB_J0ezIahWLEQs90Rgj7--1450XudXBLxu1z3xxF3301Fr4zuUyg3X2jbGviA metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Updated Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: lEVXrTcVw8rIbhXluRAQNN5-fgyoWQmWHX-MJJ4wAVfJrQlWWPLcxcyI_XYE5diHS4FCTDsflWRbDm65CuoTWA

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
Collection created with first record: 77tIxqm6-Guc4QPG8pVYwG4ea87ZXDMTigwKZ2szmHNd_N0kjRx7G_3AGdphWW6iNXnvrq5LTN5xdLfDE3Q5dQ

=== List Collections ===
Total collections: 14
Sample collections: [ws_ttl_test test_collection chat_configurations__ek0_testing scripts__ek0_testing websocket_test]

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
Total keys in store: 10

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
Created Alice: $1000 - ID: Bs6siC6fSpdL0bNOIrNY0HqVzvt-0LOU9cdAMlcJ7tDWV73zskf-KvnhOmoITfdRS8GLryLeP9iNSBsKlvIZDw
Created Bob: $500 - ID: jfNtkceUr4S6e4I-qGodgwTI_y6yLCaD0PwKKGNsy10UuZF-WleLGL9UFMIc66o-gY2ZN_AT5maTYuLYTW-HUQ

=== Example 1: Begin Transaction ===
Transaction ID: c36171d0-6c24-479d-a876-a4bea92f1d78

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: ba342858-51c8-442a-a42a-bf7721389418
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
  1. Score: 0.787
  2. Score: 0.754
  3. Score: 0.751

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.715
  2. Score: 1.500
  3. Score: 0.302

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
  - Mouse ($0) by Alice Johnson
  - Laptop ($0) by Alice Johnson

5. Complex join with multiple conditions:
Found 2 users with example.com emails
  - Alice Johnson (alice@example.com): Building A
  - Bob Smith (bob@example.com): Building B

=== Cleanup ===
✅ Deleted test collections

✅ Join operations examples completed!
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: zYu7G9IqDzzolkrYuVHexDdw2kYtBt-h6uFvwmSXjMim8R3uGmDNYfVfcrmqVoahbcHMQnrLwjAnFUrbUcXOBw

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: nUz4j2zlnfBF7bwfCcPJGtqXDOiWXjusfolG4x3vha7BNVHujwgTPlddQqFQrUJVPGQKKzHDHke1XzqKoLVF3Q

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
✓ Inserted document with TTL: 2dGWAd3ThzPlGjixpHs4AGCeiKVod4vxXpvcWNgwk8BlOFPqTNP-acXDTtTvujoqBQ3tMN4No1SiNHyww3YMeA

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
✓ Edge cache script created: 7cBpvn0fBRxS3QGQsxP2_Qn8pM7DBbsKo8AbRsT-nSPgwCH1Yj375lrIyUCdbactYHp8emVKTHJmUX5WaGHyMw

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

✅ Script saved: qnwrz5aUuqmA72i89X37Ney7eWvxQCC5QR3F769mH4PntPUVXYN5X8Q3W9SYVKJt8-TUuElHNqoOAg2vz0eHGw
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
   ⏱️  Duration: 2.194291ms
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
   ⏱️  Duration: 2.188542ms
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

✅ Script saved: _rZiVoN83cVgNbdk-FSJf3Ieq35cZUAYRIJ2H-sbwDxZ8q8gCPXpO8xmBpIpmQF55MmC-A38vgrQXQ0rNG0qpQ
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

✅ Inserted order: 9kPXmAVEGMKHFUutKGEAkrrmAcuAi-L_u-tTCi8ilEC2_3s2TvrEWDDslgWLjfjqR1b1giygut60xTqe5h04mw
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: otrkrP0TlIMGBsOuC8zez-y01QMBVVvLhJC8-J9DXzEYkmmzvqMJhdowzwUyOm3cOCi__TIA8Sm3VXrhQFFXqA
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: map[role:admin userId:user_abc]
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session
📝 Example 4: KV Operations in Scripts

✅ Script saved: 9T274v4F6QIvu3nl8KBvR76W2jrXAU3IhsnAPm3aO6wf__DpVrMFqEy-hNk6n3GQpxukkn3nIQOgwSBkUGrfgA
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Script

✅ Script saved: EdlIowSeSMGm_bbWn7RVuVuBck0kOwC-XMejZ8HL5Ufp4AvB602ULijIGtDzDlOBoQwTRHVisXqyJI-tWiYRSw
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
✓ Created SWR script: fetch_api_user_go (eRqn7e-Dt22NyZkezQFchmObzglCLb6a3szGpU64PekvyM9MU7d1Llplk_WMe7op4HxfGpxK1WFPVnYoL-goYw)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "cached_at": "2026-01-26T18:25:23-05:00",
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
      "id": "DBzzIbLeQf2_wYnWqkojDy2TBWpYJv2RjpVNDdPRJWCO3tM4qr3wUdaUfYzOeRmfr08QSddeag0vmjQdwWz-rQ"
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
Response time: 64ms (served from cache)
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
   1. map[type:String value:Database Design Principles] (map[type:String value:Database])
   2. map[type:String value:Getting Started with ekoDB] (map[type:String value:Database])
   3. map[type:String value:Vector Databases Explained] (map[type:String value:Database])
   4. map[type:String value:Introduction to Machine Learning] (map[type:String value:AI])
   5. map[type:String value:Natural Language Processing] (map[type:String value:AI])
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
✓ Created session: 5SXXYkac4oI8ZD6t96ysIzlR4gmK3V7-R1Iwtf2MJGfi98pnTNB6zu2F6x0ZDv_w28kkWW9OAnBPVRKv9l6Mqg

=== Sending Chat Message ===
Message ID: 98Ns11LWnW7GwKa1aAXT653XnqkbSwmsC5WKGTzNLMwGG3ABOsgDVBBcrSzgVR3DxAwO7IIcRPis8UrTSWIKCA

=== AI Response ===
There are three products available:

1. ekoDB Cloud: This is a fully managed cloud database service product. The price for this product is $499.

2. ekoDB Pro: This is an enterprise edition product with advanced features. The price for this product is $299.

3. ekoDB: This is a high-performance database product with AI capabilities. The price for this product is $99.

=== Context Used (3 snippets) ===
  Snippet 1: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Fully managed cloud database service product id:D160WSNY3WSozvNUELHpudTFpRheNjY00hGadoxG4kevbmF9Dw9mfbGwAOVqJbqgxiH3E2XhYRMbQ608M5RSlg name:ekoDB Cloud price:499] score:0.1111111111111111]
  Snippet 2: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Enterprise edition product with advanced features id:H7443kVPegg5NPpQ1heaRYaAE2nfzqJgn_Fx6VVQuSgtq_X8CUhK1UYNs1-FZIu71-JLdfpjMorXLry-Wg2IOA name:ekoDB Pro price:299] score:0.1111111111111111]
  Snippet 3: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:A high-performance database product with AI capabilities id:Y6JE-ub8mMZSdw-Np646eNoVGAefVn9999w91jA1RjfHB1g0Uh3jxU9Yu_HZ2IlSKIvvk4jQ86mwW77AdGsyIQ name:ekoDB price:99] score:0.1111111111111111]

Execution Time: 2785ms

=== Token Usage ===
Prompt tokens: 443
Completion tokens: 84
Total tokens: 527

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 5aP5andDapvfEVPaWZevPjxsstyZ2apNpvwl9jtGL1D4IcfYoOsjIbaYY7FTxqfpgEGl2C5Uq6HR9QuB4TEgyA

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the available context, the product available is "ekoDB". This is a high-performance database product. The price for ekoDB is $99.

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
✓ Created second session: LfxcAjSnqvQbHtrXI6GQFVFJ4iItWlvXOyC_AE-bc4lY_Jq9FZVUs9XeBwmZxf6v5e8mRkJiFUlwD8aaU50iqA
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
✓ Created session: M9ADsELP71bK8H9eVTTm70HNQsJjIwaOy8a8JNwQ3F-33q26Vd95gmZZRp1X6oaoJpT1dfguwe9mVlq8wIPF3w

=== Sending Messages ===
✓ Message 1 sent
  Response: Currently, we have a high-performance database product available named "ekoDB". It is priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 5MrhekMHwHd3QTFa6pna3b6p3ehokT05gquzqTCuDs_HxzoLhWY2JmkRKXd_R-IAtHwEZFTUidJihYFK0NbFFA
  Parent: M9ADsELP71bK8H9eVTTm70HNQsJjIwaOy8a8JNwQ3F-33q26Vd95gmZZRp1X6oaoJpT1dfguwe9mVlq8wIPF3w

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: 5MrhekMHwHd3QTFa6pna3b6p3ehokT05gquzqTCuDs_HxzoLhWY2JmkRKXd_R-IAtHwEZFTUidJihYFK0NbFFA (Untitled)
  Session 2: M9ADsELP71bK8H9eVTTm70HNQsJjIwaOy8a8JNwQ3F-33q26Vd95gmZZRp1X6oaoJpT1dfguwe9mVlq8wIPF3w (Untitled)
  Session 3: LfxcAjSnqvQbHtrXI6GQFVFJ4iItWlvXOyC_AE-bc4lY_Jq9FZVUs9XeBwmZxf6v5e8mRkJiFUlwD8aaU50iqA (Untitled)
  Session 4: 5SXXYkac4oI8ZD6t96ysIzlR4gmK3V7-R1Iwtf2MJGfi98pnTNB6zu2F6x0ZDv_w28kkWW9OAnBPVRKv9l6Mqg (Untitled)
  Session 5: Vjg7gbLn2PTz6aASV1V2HqucyY9_0-mbJorwf-OYI1bz4n56cRj5AiDkY4LOfEIdWts1LdYVp-ZI7Fs0X7TGCg (Untitled)
  Session 6: MjMzthfgQkK_RCZ5U_aYC-J-lAwwz_GwyqBHLHRntDFGFW8elnqNQCAvkbPZ32JJA4sn7fvaUDNmK7HDcAzdAw (Untitled)
  Session 7: 4-V04W3imEKGCRZjDetm--x5I8nlgyhbglc0O0ne_DpXqOAoTm4N6TjYOBYebjqZSDug4o_S4hpEvTDgsa8NVg (Untitled)
  Session 8: 5QxYKUFTMmEPqanYhCgr6qW5GS16WR1B8OBf27ZhsDvNl5nyCnRYlQrSxnhpECUbLp6GSLR16YjBt-2IJ9PsGw (Untitled)
  Session 9: Jl4mTFpz00jWsGudEumneFR__XNqOT5dDRYYYMoqAxV6dp57xjbgUb5ihLSePZv2Ja1dspdRuGNIayfv4oEYiw (Untitled)
  Session 10: Dg8xiBnwBaVnvN7d4DIqKabLRHXxd9EOqXcGWdc16Tpz1RhUKGt1zSSxCy1IsQFvAn6SFcZ1iR5Ci5LGyCQvyQ (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: 5MrhekMHwHd3QTFa6pna3b6p3ehokT05gquzqTCuDs_HxzoLhWY2JmkRKXd_R-IAtHwEZFTUidJihYFK0NbFFA

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Map Creation ===
✓ Created record with native map: map[id:h_i5BtPjvUBhrMowg29hL6zL3M3cTZVE3T5zcwP8WbpgYSd27slkXWwUBrBvq5KhJognUjp1NnIRa3nVw9vcKw]

=== Upsert Operation ===
✓ First upsert (update): map[active:map[type:Boolean value:true] age:map[type:Integer value:29] email:map[type:String value:alice.j@newdomain.com] id:h_i5BtPjvUBhrMowg29hL6zL3M3cTZVE3T5zcwP8WbpgYSd27slkXWwUBrBvq5KhJognUjp1NnIRa3nVw9vcKw name:map[type:String value:Alice Johnson]]
✓ Second upsert (insert): map[id:sswP2P4WXgTW3hUgnlRigxfEYrJq4-QrwcxD_hsy1UKDxtBup36J_4D5xb1PRRIB7UEN6HmygthEn1yM8M0sIA]

=== Find One Operation ===
✓ Found user by email: map[active:map[type:Boolean value:true] age:map[type:Integer value:29] email:map[type:String value:alice.j@newdomain.com] id:h_i5BtPjvUBhrMowg29hL6zL3M3cTZVE3T5zcwP8WbpgYSd27slkXWwUBrBvq5KhJognUjp1NnIRa3nVw9vcKw name:map[type:String value:Alice Johnson]]
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
Inserted with ripple: map[id:BOZJDGtOZQxVO9L5-4TeGTyM-_LrSBLAdAR97bQ7T4J7z8zgR16vkzVA0Tuz86sV6VPIZNX6OIOWgkUAKtCtWw]
Inserted with bypass_ripple: map[id:MsXgxQkMDcYaIgHf6iamYt9cjGfRJc1Xk28Po0_PTPyyGZYyA_6_USqrdn_5c327vXguYHiFHCurgP2SdzypoA]
Inserted with TTL and bypass_ripple: map[id:weluE2UAO0zkjH9Ancpnn83AVYAIYQVRYqPs8lcfrADJltayLZjZbDgmkYRbcnlc7EiEwItqP7hAss-1kjDhVw]
Updated with bypass_ripple: map[id:BOZJDGtOZQxVO9L5-4TeGTyM-_LrSBLAdAR97bQ7T4J7z8zgR16vkzVA0Tuz86sV6VPIZNX6OIOWgkUAKtCtWw name:map[type:String value:Product 1] price:map[type:Integer value:150]]
Deleted with bypass_ripple
Batch inserted with bypass_ripple: 2 records
Upserted with bypass_ripple: map[id:BZrZVEh1EDV_j6jhbvmNvqQYmGqraJJxqFfubbKUoQwz7PeYb6ym_9eurP7GIL2pvr7fpvhSeRk8RLDIORz82g]
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
