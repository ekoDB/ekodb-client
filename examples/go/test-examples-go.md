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
Inserted: map[id:69V6yq5bGe_6HFnbT1aZUaqHC1dzAz0xw-31lKJta1kpuwI2d2ZJQ-oXhp63n9Brbqk4j0C8zzQSVozrOhU7pg]

=== Find by ID ===
Found: map[active:true id:69V6yq5bGe_6HFnbT1aZUaqHC1dzAz0xw-31lKJta1kpuwI2d2ZJQ-oXhp63n9Brbqk4j0C8zzQSVozrOhU7pg name:Test Record value:42]

=== Find with Query ===
Found documents: map[]

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] id:69V6yq5bGe_6HFnbT1aZUaqHC1dzAz0xw-31lKJta1kpuwI2d2ZJQ-oXhp63n9Brbqk4j0C8zzQSVozrOhU7pg name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
[32m✓ simple_crud.go completed successfully[0m
[34m
=== Running simple_websocket.go ===[0m
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: A4qF-pqdwkFsJuqfXLHpKbs6oAE_YPGEhnmgThECK-CXukWAgzfuHETj4unrK6t9mwdM3JK25MnDvj8OHX8BYw

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
        "id": "XynUeQSPaTfp_nzYBxA4ynDonvZIjV5Ok9f_VEPpIkitQWuzrXR_nHJ9cd9NPM7dOh-40i2EZzq0MpmA7LiPeQ",
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
        "id": "xqlWFz-xcGkzGrnXOkLAPRhrmrsYJH2KINL4W-8E2QvjvbU_O7jY19A6D8om-LLjx6BB85QBiBvAz0MPk3ORjg",
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
        "id": "A4qF-pqdwkFsJuqfXLHpKbs6oAE_YPGEhnmgThECK-CXukWAgzfuHETj4unrK6t9mwdM3JK25MnDvj8OHX8BYw",
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

✅ Script saved: H-pncSqeFifxzF2x9lAnqeoubXyAt5k9YsM9a2CE5HGN1TwEZMIM1-Pijg1CE4o-n6yfhPgIkOduvArXRpMqVg
📊 Found 10 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: e7XqNIWQkitlagqHPP4VY26nqQaFu3U1O2hRm6y475KLbTSUyggoDz2NYqfOu3XfhhI8NZ2tVd3lFhhkJZ7lSw
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: NNdmO0hWHPnnjSCB1sPgRLcJYSjis9wxMJprx_CJ806jWOSO_5zS_x_1lQcYH0Cs-meOWjkcZueZSFO8_9H7kA
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
Collection created with first record: eqqXHWFuLdiJWmLOlwOU4uXiinklvcpAxE-VlSIUzvU-TULdnlTOd4TfGSigf8TeRShqkF00Nbt8yu7BGktyUw

=== List Collections ===
Total collections: 16
Sample collections: [chat_configurations__ek0_testing chat_messages__ek0_testing demo_collection scripts__ek0_testing batch_users]

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
Created Alice: $1000 - ID: sYR3daRPfT1X3py5SBTKy8_lTktOmGQovUwv3h68tjdgnHOnYiS4pw8GnSsdsEzn6xnxcPGtvZ990Hz0nxkjYA
Created Bob: $500 - ID: 4I7lx6jY5svWuGeD9n7mnb3CAg0xlgKDq8Yd4ZXWHXB9D7HViReWrGp5SETLYNo9Js6REIt19J1ExlyIpXkZpA

=== Example 1: Begin Transaction ===
Transaction ID: 89217c6c-d943-4249-9279-7e06433f2042

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
New transaction: f4cad4af-e39b-4f6b-8f70-90dd4336186a
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
   ✅ Script saved: UiT8dpZiW3NJHE7wn1rQsDuY3YKjAtiwKutQUlKyV-usgdqcu8oBJzBocYIKdy37GohJYSB_Og-4uHnlHtd64w

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: dC6OVOdidTuRcOyLN0oBkXkBeeEpFejWx0N1KJXC-dhabmWuSlNylmc5cM21JdMZIlC_dnYIkwHWyznSL1HSKw
   📋 Name: map[type:String value:Alice Smith]
   📋 Email: map[type:String value:alice@example.com]
   📋 Status: map[type:String value:pending]
   📋 Credits: map[type:Integer value:0]

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: CCx6z4LNs6a9bw0tkmdTYrCLOwZki_1uk5njfVRANPaLDxHT_V1BZt0N4pGyQluVQba2AOy_2Ak5anHQlTLAHA

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
   ✅ Script saved: awGidDOzQEhACQnDkhX9cYz6L2dpb__eRvrkl5r1a9eU6XfjEemb2bOGkoDJRN0eddRjyeYHtvgYcIiU306T2Q

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
   ✅ Script saved: wRLrmjzYQ7IsGCjtryNoqu7W7Su1GfL0-WQHPuG1ni2aVg_Umz0JEnQKcm6_1xUrj_aNdwoP7V97n8smI_2Opg

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: UiT8dpZiW3NJHE7wn1rQ...
   ✅ Deleted script: CCx6z4LNs6a9bw0tkmdT...
   ✅ Deleted script: awGidDOzQEhACQnDkhX9...
   ✅ Deleted script: wRLrmjzYQ7IsGCjtryNo...
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

2026/01/26 00:57:52 No .env file found, using defaults
✓ Client connected

═══════════════════════════════════════════════════════════
TEST 1: Document TTL Expiration
═══════════════════════════════════════════════════════════

[Step 1] Insert document with 3 second TTL
  Input: {name: 'TTL Test', value: 'should expire'}
  TTL: 3s
  Output: Document ID = bcEDAysuy7ybf18LRmRMQPAhCrjqPEU_SEnlAl1SjaefnE75aiQyfJs8t8tCAappwKIPAIhGMnODjysMg7_wHQ
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: FindByID(bcEDAysuy7ybf18LRmRMQPAhCrjqPEU_SEnlAl1SjaefnE75aiQyfJs8t8tCAappwKIPAIhGMnODjysMg7_wHQ)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: FindByID(bcEDAysuy7ybf18LRmRMQPAhCrjqPEU_SEnlAl1SjaefnE75aiQyfJs8t8tCAappwKIPAIhGMnODjysMg7_wHQ)
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

2026/01/26 00:58:00 No .env file found, using defaults
✓ Client connected

═══════════════════════════════════════════════════════════
TEST: WebSocket TTL Expiration
═══════════════════════════════════════════════════════════

[Step 1] Insert document with 3 second TTL
  Input: {name: 'WS TTL Test', value: 'should expire'}
  TTL: 3s
  Output: Document ID = X2BEoxnHjlyoxxueLGSjpvlfa1JTI06ca_jb3riWrlPtsXa_y6hoypJIgZvzSo46bPwhWSU4NUDSpZ3UVn3Egw
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: FindByID(X2BEoxnHjlyoxxueLGSjpvlfa1JTI06ca_jb3riWrlPtsXa_y6hoypJIgZvzSo46bPwhWSU4NUDSpZ3UVn3Egw)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: FindByID(X2BEoxnHjlyoxxueLGSjpvlfa1JTI06ca_jb3riWrlPtsXa_y6hoypJIgZvzSo46bPwhWSU4NUDSpZ3UVn3Egw)
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
Inserted: map[id:M9roSnfUtNDjzpGYTp-k2WaNHs7rZ7o79a2EpfW-yBc44CO8HQtLkJrq-X1lpFYiE_o60wTsaBzAnq9Sd3zClg]

=== Find by ID ===
Found: map[active:true categories:[electronics computers] created_at:2026-01-26T00:58:07-05:00 data:aGVsbG8gd29ybGQ= embedding:[0.1 0.2 0.3 0.4 0.5] id:M9roSnfUtNDjzpGYTp-k2WaNHs7rZ7o79a2EpfW-yBc44CO8HQtLkJrq-X1lpFYiE_o60wTsaBzAnq9Sd3zClg metadata:map[key:value nested:map[deep:true]] name:Test Record price:99.99 tags:[tag1 tag2 tag3] user_id:550e8400-e29b-41d4-a716-446655440000 value:42]

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 0
  active (Boolean): true
  price (Decimal): 99.990000
  created_at (DateTime): 2026-01-26 00:58:07 -0500 EST
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1 tag2 tag3]
  metadata (Object): map[key:value nested:map[deep:true]]
  embedding (Vector): [0.1 0.2 0.3 0.4 0.5]
  categories (Set): [electronics computers]
  data (Bytes): 11 bytes
Plain record: map[active:true categories:[electronics computers] created_at:2026-01-26T00:58:07-05:00 data:aGVsbG8gd29ybGQ= embedding:[0.1 0.2 0.3 0.4 0.5] id:M9roSnfUtNDjzpGYTp-k2WaNHs7rZ7o79a2EpfW-yBc44CO8HQtLkJrq-X1lpFYiE_o60wTsaBzAnq9Sd3zClg metadata:map[key:value nested:map[deep:true]] name:Test Record price:99.99 tags:[tag1 tag2 tag3] user_id:550e8400-e29b-41d4-a716-446655440000 value:42]

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-01-26T05:58:07Z] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:M9roSnfUtNDjzpGYTp-k2WaNHs7rZ7o79a2EpfW-yBc44CO8HQtLkJrq-X1lpFYiE_o60wTsaBzAnq9Sd3zClg metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Updated Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: Qth_mTKGsUcvFslTiAfKUa38l-UPokyLE2nC9Q9Nd7rAIA-SB8bP4GaQFzUy9SgW_2ylcYa1k9EhyJ7GyEnIvw

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
Collection created with first record: 89rmEPfljTomOs7dsfFYAws3RiKbmRFHvyylrSOPrkmHftl-dkRqCiB-lfx3smJl4TuJrefLzMk85O67A2p91g

=== List Collections ===
Total collections: 15
Sample collections: [client_collection_management_go chat_configurations__ek0_testing chat_messages__ek0_testing scripts__ek0_testing batch_users]

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
Created Alice: $1000 - ID: -iPY6igk_L62Zufssk-b_Fiyow9AB0Bt6wGXKkL5L6YGpN-JoyGkkxWaTZWBqK7uR56kUnKLlEUa66KAfS2y4A
Created Bob: $500 - ID: XLF5w8HGq0tnLdJOiYHVoSg-g5iaizSjt60sQ7l3oHbgI2Ua9dAIFWAtUaz3Q_SQ4vh2kQFCzhpRU_NvXpg6nQ

=== Example 1: Begin Transaction ===
Transaction ID: 9d05a29a-83bf-4412-9e36-9b7cae36aa42

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: b408289d-68d6-4d8d-b6d8-595c00fb1fe7
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
  1. Score: 0.756
  2. Score: 0.747
  3. Score: 0.724

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.690
  2. Score: 1.499
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
✓ Inserted document: ch9sSYxvjeefumK7Nt_vl-VtT-CTUKxfxodAJ8e6YEfktYAZMMU_B529rJUb6j6N8eegyVORbEXat_40G2XpLg

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: gk-sXu4y4JpYFTCWUHzZ7faTngAN3TbohuItmyL3Ral1-k-FBdIWIarfJa4aa1R5MdtuencE3NU4m7QPTPlmVw

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
✓ Inserted document with TTL: HdOVOSg16Mw9jOiPyGzS3G2Q2FQnlkiRxidThF9-1kN2ud9wKQDqI9zyLNeriKWkQQJKqR90NnpZINd8abBTFw

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
✓ Edge cache script created: zeVv2IaBttuB4TufXrKgJnGxOxZvdn-260RFJkpgiu7ePC_10l3XUu5viM_f5wm5554mpQ5OIoccc-GLajEffA

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

✅ Script saved: 7tr-OuGiLXA3oW9uvDbj0uJg6nDbIGpYn-6AOqmqqjH91c1D3dxN8Da75HEwfTDyjbJknpGFc2ihUapKNU9khw
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

✅ Saved reusable function: fetch_and_store_user
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 86.753417ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 75.477958ms
   📊 Records: 1
   🚀 Cache speedup: 1.1x faster!

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

✅ Script saved: fA9F7tfAWX6ko00-5Mju3ZLdBV8atoc9aSILKj0hvkXc3RUvTELwgxOnGxxvl2EETu2QhxvRNiR9VhIEoPFv5A
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

✅ Inserted order: ryIP9njGyYbzySuqkrCx9d1MO0oq-3wrawF0-Pvr2DKRk636pa6GiDbbCRaZzZF7eP_CpIcyLUU3eCPyy64lGw
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: OLJNLs7OLaetxsHuFROj2T-FMTfGjmCI6LdCABA5D4hl7IxQFp_ZfelWCou9aTzLjLEj1bHeV9_WtZMEv5-PqQ
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: map[role:admin userId:user_abc]
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session
📝 Example 4: KV Operations in Scripts

✅ Script saved: pKRZvHH_MObA93RpF2_Io1gkCVumvI9AkwEP3d3DWuWCMb0fRtjzDz9E0gRbEccMBjKhgLaXnLvP755ZPlmbJg
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Script

✅ Script saved: 3vHdxwShnboVmQQxPLhyXUJMWOrFaVWgbDMo2TsBc__4unM9c2AvscxlJP24WPguzMjtUuIYCdMVTfO8h7V8Rw
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
✓ Created SWR script: fetch_api_user_go (Vokbp_FJYn0E8yWxF2JAn5s0v4XVovqkXJPFDBZ3ztauC1VlPGIlTFERWhq4Jr75JkdCwWwiC3Nzt1_QSKlPkw)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "cached_at": "2026-01-26T00:58:11-05:00",
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
      "id": "L2XsKpQT56eVRRoLqwtdrrzutAjGNSzGLDfLnWwRvMCOSpoXWC514msKC-YjyRane3L25XNuMqyPN21ueFy-0w"
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
Response time: 79ms (served from cache)
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
   1. map[type:String value:Getting Started with ekoDB] (map[type:String value:Database])
   2. map[type:String value:Database Design Principles] (map[type:String value:Database])
   3. map[type:String value:Natural Language Processing] (map[type:String value:AI])
   4. map[type:String value:Introduction to Machine Learning] (map[type:String value:AI])
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
✓ Created session: iOoS8YtheulgTtAeIuuE8D3bhrehIrH7-gMf9kcjqZ1WhiK4u5IFgblmUK0mnvBgQq4b_DIusk1CVO0wAc81Ew

=== Sending Chat Message ===
Message ID: 8LKM6t00JKIl8h5S9896vlSFUQrFguNMXxv_S4NYYUiALswkmHoZ0tMwyzb_0Xpv-jfqkRNpNdIqmC7tqK-1fQ

=== AI Response ===
There are three products available:

1. ekoDB: This is a high-performance database product with AI capabilities. The price for this product is $99.

2. ekoDB Cloud: This is a fully managed cloud database service product. The price for this product is $499.

3. ekoDB Pro: This is an enterprise edition product with advanced features. The price for this product is $299.

=== Context Used (3 snippets) ===
  Snippet 1: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:A high-performance database product with AI capabilities id:GhCgogDBsMY9HbreV6oZpxr8JmYT4eqM5QRjvxZ-OoKPbYVjtDIqwYhkX16TWg0HSjikb2ozxYnkTOH6pVKahQ name:ekoDB price:99] score:0.1111111111111111]
  Snippet 2: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Fully managed cloud database service product id:XUXJCmMhLyiZf5nW6-Xn6ti8D0-f-Aqd45sdRhISp2kcEO9AW_7MxEMREy_X6ZTp26GDIUtzYeeHL8YIVcAtoQ name:ekoDB Cloud price:499] score:0.1111111111111111]
  Snippet 3: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Enterprise edition product with advanced features id:9KhDQRlu-yPcjXYCl3yQDUIUfmPHtLAViGz2Y0_Hw4MXo0cZKE-4KJiMBpYEoCjdGlvyKeNBuumTHYf4kK7fCw name:ekoDB Pro price:299] score:0.1111111111111111]

Execution Time: 2977ms

=== Token Usage ===
Prompt tokens: 450
Completion tokens: 84
Total tokens: 534

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: LmSQtJ0PSHNRi8IZpwtyZ_5VPFUn8o2s06I2qNHLI_Jo_kdPx1ckb2Vw1nqJKyjiDVnU-ceYaPESBSp_ulhR0A

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the provided context, there is a high-performance database product available named "ekoDB". It is priced at $99.

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
✓ Created second session: ltoCE9Mp-YHRJtsl5hQz6NbEq3eQLlHVQko1hD7fAv4kWIRFPEQ8lYFYcwbEKxKjiqcO4ZH5mj5uSn5vyDHDEw
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
✓ Created session: IrOVWEMKky4KQ6TrgSdxgT6MJoIpcGShelNCnE6DM-UxuLeKBPvkAUMsX67mVo-ZZj0hUujbe4-8KQ2yrzR3AQ

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product based on the provided context is "ekoDB". It is a high-performance database product priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 2XJhpFO91hH0Wfu7q2gnBsw5ktT8aWSSesecGcTv1dFofBm5_JsPFFszfeIPphjbgk_ic8n3rcI8qEhW7yzUQA
  Parent: IrOVWEMKky4KQ6TrgSdxgT6MJoIpcGShelNCnE6DM-UxuLeKBPvkAUMsX67mVo-ZZj0hUujbe4-8KQ2yrzR3AQ

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: 2XJhpFO91hH0Wfu7q2gnBsw5ktT8aWSSesecGcTv1dFofBm5_JsPFFszfeIPphjbgk_ic8n3rcI8qEhW7yzUQA (Untitled)
  Session 2: IrOVWEMKky4KQ6TrgSdxgT6MJoIpcGShelNCnE6DM-UxuLeKBPvkAUMsX67mVo-ZZj0hUujbe4-8KQ2yrzR3AQ (Untitled)
  Session 3: ltoCE9Mp-YHRJtsl5hQz6NbEq3eQLlHVQko1hD7fAv4kWIRFPEQ8lYFYcwbEKxKjiqcO4ZH5mj5uSn5vyDHDEw (Untitled)
  Session 4: iOoS8YtheulgTtAeIuuE8D3bhrehIrH7-gMf9kcjqZ1WhiK4u5IFgblmUK0mnvBgQq4b_DIusk1CVO0wAc81Ew (Untitled)
  Session 5: x0OceMRUKlVJqgmvrBOeSwKgLI_pZrbtCn1_hHQAaS0T-T4dt2vZEFP0c6XtqVpd5F4QprgHp2CCrwyIkLI1Dg (Untitled)
  Session 6: hfFbxSDUNbfSKyxC79AJeZd7GCrQRPZm1gkmWSev-oxoy7vEStTatUY4EFqw212ot2-7w95yye9aE4-_D1Ioaw (Untitled)
  Session 7: qO-Z-fYTrhxTnTF_RGGmi6IGq6Cw3n5Wtt4IzSVHy-EGRp8ZpGeXPf6yW9lO2L5vAbnhl3E9xRUrLPDM6uHP9g (Untitled)
  Session 8: wNWC_Xk3eqKojZcNLJI7ju-2qUVlYBXfFDTXA8MF4cqBWoe0Vh8TkGCxiYsb0gwbZeQUPFasl6EsXuhSfp3kzQ (Untitled)
  Session 9: gnRXJNHkYunqfd9H8WAnDUhdM_bv_6oBpprg16bG5k95zOLdYvzkww8npnALyOvXgr2HoKc7v1m9f-T9raeLGQ (Untitled)
  Session 10: twO1BB1nRCiwMXP9x5q1aLkLr3v9Hx_eQLeKCnTTvf2LQClZQ3jyYvmcuFPlzPfVdBYPZw-czz__aLo_WgAbBg (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: 2XJhpFO91hH0Wfu7q2gnBsw5ktT8aWSSesecGcTv1dFofBm5_JsPFFszfeIPphjbgk_ic8n3rcI8qEhW7yzUQA

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Map Creation ===
✓ Created record with native map: map[id:7ElMQh_pFXhuKT0ikDOvs0DIPn_ZphvsT8tMSmwChuMbIZffIB-k9HPIkXY2smjJbVf7IoZVMUOIGr5i629RDQ]

=== Upsert Operation ===
✓ First upsert (update): map[active:map[type:Boolean value:true] age:map[type:Integer value:29] email:map[type:String value:alice.j@newdomain.com] id:7ElMQh_pFXhuKT0ikDOvs0DIPn_ZphvsT8tMSmwChuMbIZffIB-k9HPIkXY2smjJbVf7IoZVMUOIGr5i629RDQ name:map[type:String value:Alice Johnson]]
✓ Second upsert (insert): map[id:fIqYCnfz15-XuI2rJM6Ssa6C5BAzKYPYV7juGERzIm3nJk3K-mRYIXgIkhZEGbgQHbYy0VBa42CUqgM2FZypXw]

=== Find One Operation ===
✓ Found user by email: map[active:map[type:Boolean value:true] age:map[type:Integer value:29] email:map[type:String value:alice.j@newdomain.com] id:7ElMQh_pFXhuKT0ikDOvs0DIPn_ZphvsT8tMSmwChuMbIZffIB-k9HPIkXY2smjJbVf7IoZVMUOIGr5i629RDQ name:map[type:String value:Alice Johnson]]
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
Inserted with ripple: map[id:Xp8Htgw-P8qITHymxU-iUiHpwrkK1ud7M7AMLH2UdWid-MmLJKcqnFVGE2eD30YooJSD5UDFhvjGXCjNYyrMbw]
Inserted with bypass_ripple: map[id:M78OU5s1Xb9ZdaFzx6olFeR2OwHCwbzj8SlBqIZzslhFLLbLN7o71sAgsEqvA_S40ypP0ymKGSUrD6rG-veMGA]
Inserted with TTL and bypass_ripple: map[id:4r7Tz_IJq9mNLWJVAY562vzdXj1qy_kUmToL_Xd4_N83dWB6gZJZRUrvjgz8DtaeArAIB-oip85Etr5HXv09Tg]
Updated with bypass_ripple: map[id:Xp8Htgw-P8qITHymxU-iUiHpwrkK1ud7M7AMLH2UdWid-MmLJKcqnFVGE2eD30YooJSD5UDFhvjGXCjNYyrMbw name:map[type:String value:Product 1] price:map[type:Integer value:150]]
Deleted with bypass_ripple
Batch inserted with bypass_ripple: 2 records
Upserted with bypass_ripple: map[id:ek8U5vWD0Q218oC4mOIas4tiy8zIQyzO8CCPEEvsjvH-5BtzOBUOzwdRxkOnOZeeMr_NyHqwdUx9qp2FbV8q9w]
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
