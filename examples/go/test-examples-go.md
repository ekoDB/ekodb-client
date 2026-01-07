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
Inserted: map[id:jUda7yzIxgJLMFQ7M_Zva4CI5URXqVUHwfcimHZTcGMFnu22Yxq7BAIXkSQjO8xVltFDOXo4HFhOgTk4DLwYNQ]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] id:jUda7yzIxgJLMFQ7M_Zva4CI5URXqVUHwfcimHZTcGMFnu22Yxq7BAIXkSQjO8xVltFDOXo4HFhOgTk4DLwYNQ name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query ===
Found documents: map[]

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] id:jUda7yzIxgJLMFQ7M_Zva4CI5URXqVUHwfcimHZTcGMFnu22Yxq7BAIXkSQjO8xVltFDOXo4HFhOgTk4DLwYNQ name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
[32m✓ simple_crud.go completed successfully[0m
[34m
=== Running simple_websocket.go ===[0m
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: 16S9-ioNkoo3kgUUG_aqhUl2hpsuWer-r39lEDJojbJcayOVpbHl9ceplM-3Ec8RJI5RKZeFc2ZaJ1zSmmjVzw

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
        "id": "psWb4szhCOYR74wQQoV1UoXtekxHu4T-M77Ilvd9C8hiC7_DrSCj_I3Eh_y1yRQ_T5-wi4HjHQHATu_Z5OBMXg",
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
        "id": "CQXM2art_zD-gjpFOHTRP740F6rffXlCn_MtjzsbpBXE-KdDU_Mp2pvMJQugfLRs2BTAgvS9oEJOdCOBue75KQ",
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
        "id": "16S9-ioNkoo3kgUUG_aqhUl2hpsuWer-r39lEDJojbJcayOVpbHl9ceplM-3Ec8RJI5RKZeFc2ZaJ1zSmmjVzw",
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

✅ Script saved: I6vyGbhoQQX2cMJegqLwkESTsNx8pwxU49xdVmQnEbnMoQEndu7fbhpeuubpjTkUrhhkROt73cHXx3STtm4j8g
📊 Found 25 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: GLn4mQRoaulGTcqmPqisxpFEPjyiBFt1zwWXi_UcPRVeSQ11yWF5D-cbhE0k0usOmqLSiwks7DsgfhS2FnNcbQ
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: D_DoKAzgPuCSK_qJ_B5V_NQZ7nG9TQX0Hphhw-thvaTg2QsofmBqcDeGbThmQQDfg3-hompG6NzYk_bXdzBQAw
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
Collection created with first record: vcqv-uW_8vWY2e0rDG38VeOZAuHFnflXn2OfOVt_LS3CPks32p5XPqSAQTq7T0PoOXdYxeroacV6UipcvOhkrg

=== List Collections ===
Total collections: 16
Sample collections: [schema_documents_client_py test_collection schema_users_client_py github_cache websocket_test]

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
Created Alice: $1000 - ID: fYZXR7dtockFQdDBnp1Jyy4POFwVZDzIEme5DsdgK5Ioa1YRwX4DcZT6Dn7D2KeXmuxfvZJXf3APdyB_juWhjw
Created Bob: $500 - ID: W0iTI83cLjVCSr77Wae1DUzMVALdNelZA2AMDCA4S0YWJvIWTHYoA9oWRw9QcgyTWcAjMxxk5oux1hIFvGunUA

=== Example 1: Begin Transaction ===
Transaction ID: 3bcac1b8-be07-4b0f-aa11-fe67ddb3b1eb

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
New transaction: cfb3508f-c2f0-4b5f-988e-4ed9b88dbc67
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
   ✅ Script saved: ebwvJ6BRlx1ErR15tEgXtMui7zdw8-BND39lQSjzzDbqYkzMsfi_TRWZEtJO31yzH74r9eZkWbUKHHfwnEqhuA

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: O8--IDG8XFh_WruM0ttZt8TgW6QZlrzpV88Bud2-SASK68bJqZqDYYGvNXwj-RVg3czaGOzpAJANP4OQcYOG4Q
   📋 Name: map[type:String value:Alice Smith]
   📋 Email: map[type:String value:alice@example.com]
   📋 Status: map[type:String value:pending]
   📋 Credits: map[type:Integer value:0]

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: EjfGxK0ahcKJqimLJavVx4qbm6AZ-LMKn4cVqzpaEEZQqOFEHU_P2dQ_2K2E3HTopMOs6uckrRLEVJfg7fsOuA

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
   ✅ Script saved: lBS05ebIAc5sQeirR3Xk6nC2j5dl0fPmd1G_ox2eHuSwexBSu-aHhXBw19SD6-9FkxBcRH--1luSJbUkVHMZRw

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
   ✅ Script saved: S7og45R_DaLEezbBcrTDDeKK_MqbmWG7OIHw8z1Hr83QxFnwJvjhLaY0JZ5W3SYBHXeCKez_xYEyP8HBC53NSQ

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: ebwvJ6BRlx1ErR15tEgX...
   ✅ Deleted script: EjfGxK0ahcKJqimLJavV...
   ✅ Deleted script: lBS05ebIAc5sQeirR3Xk...
   ✅ Deleted script: S7og45R_DaLEezbBcrTD...
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

2026/01/07 11:50:03 No .env file found, using defaults
✓ Client connected

═══════════════════════════════════════════════════════════
TEST 1: Document TTL Expiration
═══════════════════════════════════════════════════════════

[Step 1] Insert document with 3 second TTL
  Input: {name: 'TTL Test', value: 'should expire'}
  TTL: 3s
  Output: Document ID = MdijfFOyZs1cD5qsKx8uiLSersLABoar3dZzHUSWvCPeZEv0RELbV9Qp4TjR-CCaX9RfNk3-bxb5FjtKnB8oww
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: FindByID(MdijfFOyZs1cD5qsKx8uiLSersLABoar3dZzHUSWvCPeZEv0RELbV9Qp4TjR-CCaX9RfNk3-bxb5FjtKnB8oww)
  Output: Found document with name = map[type:String value:TTL Test]
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: FindByID(MdijfFOyZs1cD5qsKx8uiLSersLABoar3dZzHUSWvCPeZEv0RELbV9Qp4TjR-CCaX9RfNk3-bxb5FjtKnB8oww)
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

2026/01/07 11:50:11 No .env file found, using defaults
✓ Client connected

═══════════════════════════════════════════════════════════
TEST: WebSocket TTL Expiration
═══════════════════════════════════════════════════════════

[Step 1] Insert document with 3 second TTL
  Input: {name: 'WS TTL Test', value: 'should expire'}
  TTL: 3s
  Output: Document ID = _iZ5glGaAeLLDke_yBEC71sgPqiDFjmY0jqvzmxAvfMyF9tzRCwrce6TecfIa98jy_2Q3PJ4GG8KX-BEQWqieQ
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: FindByID(_iZ5glGaAeLLDke_yBEC71sgPqiDFjmY0jqvzmxAvfMyF9tzRCwrce6TecfIa98jy_2Q3PJ4GG8KX-BEQWqieQ)
  Output: Found document with name = map[type:String value:WS TTL Test]
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: FindByID(_iZ5glGaAeLLDke_yBEC71sgPqiDFjmY0jqvzmxAvfMyF9tzRCwrce6TecfIa98jy_2Q3PJ4GG8KX-BEQWqieQ)
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
Inserted: map[id:nT2LCzpAzSWNZ6nAQyaW6w6RvQJHcxAPMjjvaGQyce0FmWzgKa1i-OlXWWFpIamUg5HSyXZ57ZoZ4y5x9XaA-g]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-01-07T16:50:19Z] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:nT2LCzpAzSWNZ6nAQyaW6w6RvQJHcxAPMjjvaGQyce0FmWzgKa1i-OlXWWFpIamUg5HSyXZ57ZoZ4y5x9XaA-g metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Test Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:42]]

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 0
  active (Boolean): true
  price (Decimal): 99.990000
  created_at (DateTime): 2026-01-07 16:50:19 +0000 UTC
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1 tag2 tag3]
  metadata (Object): map[key:value nested:map[deep:true]]
  embedding (Vector): [0.1 0.2 0.3 0.4 0.5]
  categories (Set): [electronics computers]
  data (Bytes): 11 bytes
Plain record: map[active:true categories:[electronics computers] created_at:2026-01-07T16:50:19Z data:aGVsbG8gd29ybGQ= embedding:[0.1 0.2 0.3 0.4 0.5] id:nT2LCzpAzSWNZ6nAQyaW6w6RvQJHcxAPMjjvaGQyce0FmWzgKa1i-OlXWWFpIamUg5HSyXZ57ZoZ4y5x9XaA-g metadata:map[key:value nested:map[deep:true]] name:Test Record price:99.99 tags:[tag1 tag2 tag3] user_id:550e8400-e29b-41d4-a716-446655440000 value:42]

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-01-07T16:50:19Z] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:nT2LCzpAzSWNZ6nAQyaW6w6RvQJHcxAPMjjvaGQyce0FmWzgKa1i-OlXWWFpIamUg5HSyXZ57ZoZ4y5x9XaA-g metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Updated Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: wfIMaeSuIhiELGr2OBAxRLJZ_X7ejKoyNohE6_9rhPXlb8lhHu4GFYvoqyEmbf7n9YZhk3F8yPxiAB10Ko2W-Q

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
Collection created with first record: 7PYYoYegbnKnEGrakmB6H-fio-YaXapSa_JlqCCOC5bVjsy3Gf6_TUeYqekZbECrTeAGnOMA4-IR6Z_scyaCNw

=== List Collections ===
Total collections: 15
Sample collections: [schema_documents_client_py test_collection schema_users_client_py github_cache websocket_test]

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
Created Alice: $1000 - ID: Iww-H4E_nfASut73SG1H92HASTNdmBapJ5v6v-7Mj_hSQDIz1msIUU8NVwbavmNM8l0fURCvFzIv-l_zLWdZXQ
Created Bob: $500 - ID: 1pKIUnlezBQWd3dL8Kx0SSCi2Crr7ftApQrYeLt44a3ckPUKF74Y4BJRP-8SHv3Ih3iaABpuANWfGFqg9mzLkQ

=== Example 1: Begin Transaction ===
Transaction ID: 361721fc-5789-402e-98a5-88b6f7500f6c

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 0011b3c1-60b0-471e-80ee-c362b41083c5
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
  1. Score: 0.767
  2. Score: 0.760
  3. Score: 0.757

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.703
  2. Score: 1.507
  3. Score: 0.304

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
✓ Inserted document: 0_blmJ3dLztzLg_TEBumr8eo3BdhuZD0GMdF9Ne6Tn7cr86sIzsB6t8gLZwMRPJexIHttkzkjK-7BIt4eg5dlQ

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: GhY5htg6kdEWBZUiIz5vj-R2wuyzgG4ZDFVjKzw3-eEe_vPdX7q8_ihgGf464mR7ihP8aJr73n70e5m4SWUsNA

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
✓ Inserted document with TTL: Kr5DyA7WcGot-Bnr56EtURPlVpy2ZOnr7hfdkfSBEC7uZiwx5gQTSknkWJ4IStUHFdjD13lEV5ddb473b5vNMg

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
✓ Edge cache script created: ywNNrCBgSjYF3YTWZMY37ExUvmu-v8Qmvbte8I2hnj7KGZRbjfvRHakPXCnI4dVWcM7jop3RrSjHuF6XmNIf8w

Call 1: Cache lookup
Response time: 2ms
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

✅ Script saved: UAN59tJzFwP2d92zEDAZTWHORHhptUNxUSsNTx9oS8NUSMfumA8VkRaknmMjG6TRMUFwexRC6DLa50W7Rn4wIw
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
   ⏱️  Duration: 141.766333ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 85.36925ms
   📊 Records: 1
   🚀 Cache speedup: 1.7x faster!

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

✅ Script saved: 5tlIOco2xp11drL76u5bo-S6kdVhYfPlvfaa2uVmf5GPMLPomIGpZRq5vRBnDnCHa6lX9J1uWztbb1PDwL9Dmw
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

✅ Inserted order: kxvLdD9S6EOUuyZCmG1Tvuoj2WparXQrip7kimsy1Z_hX_86pxvyoLDtCexStn9gdL4yQjkeS6V1QpIqJj7BAw
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: DGlAdDeyzMMSnKsXbZ0M9mq_tnFpw3aovORTQaTYfm-CCCnyKfTpG2hswpDEnf4M5EekU6qhb54EFdLp9BGXiQ
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: map[role:admin userId:user_abc]
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session
📝 Example 4: KV Operations in Scripts

✅ Script saved: NqLy4Pl3-J3Ahsmq3Ud84FMxxnm2NJlGuyO7NQN6xsk9V9pON6zygy6mkET9q-3rlrE4O-gimahHj2eumow-YA
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Script

✅ Script saved: T2_QzMVJWrTnPl5NFrvCLkILj4a8tLb_yvh_A3BJwFjj9iwF9taCRhNYpUz6efjV_5Q7wMd3U1yK2JLkskMSMQ
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
✓ Created SWR script: fetch_github_user_go (MuvXy7BMPBONbQaju_gUpTaDeXjnaDaj5rCCCeEAGGVk8i54EBqT5GEQycaucLvTSYSw0eDFIoin8D49IEk0kw)

Step 2: First call - Cache miss, fetches from GitHub API
Result: {
  "records": [
    {
      "cached_at": "2026-01-07T11:50:29-05:00",
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
          "followers": 270495,
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
      "id": "z9LgDgX8DmlH_VCf16aE1507YchrxD7z7Fr-pdt6NbiAeP7IKe6xC-6LbKZyjkOINvSkg9ccuPiEwHMOGW5y0w"
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
Response time: 85ms (served from cache)
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
   1. map[type:String value:Natural Language Processing] (map[type:String value:AI])
   2. map[type:String value:Introduction to Machine Learning] (map[type:String value:AI])
   3. map[type:String value:Getting Started with ekoDB] (map[type:String value:Database])
   4. map[type:String value:Database Design Principles] (map[type:String value:Database])
   5. map[type:String value:Vector Databases Explained] (map[type:String value:Database])
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
✓ Created session: VpNXcfJoGqkGtMBJo6_-992FNbN4k1fkjof5BM0K4YnCRY-6eIkZOJG7iE2pXivA2enUbqp-3-B0fo0rUMp1-A

=== Sending Chat Message ===
Message ID: OwyAS5v_-aKk-i4Xg-xBUjSC5hyaCEWitUZTdRXvJxk1zGeJFRqFaoIKBm_yRz-3L4c0Oigj2D_Lw_D-3kZDBw

=== AI Response ===
There are three products available:

1. The "ekoDB Pro" is an enterprise edition product with advanced features. It is priced at $299.
2. The "ekoDB Cloud" is a fully managed cloud database service product. It is priced at $499.
3. Lastly, the "ekoDB" is a high-performance database product with AI capabilities. It is priced at $99.

=== Context Used (3 snippets) ===
  Snippet 1: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Enterprise edition product with advanced features id:gEivysZ3jcG0dYJ_MjCfdUkdOHh4VSiCi8NSt_3XK3LnpKwyzM9E1jJViigv7CMiZJcbkAeX0-WKZY8PiEWawA name:ekoDB Pro price:299] score:0.1111111111111111]
  Snippet 2: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Fully managed cloud database service product id:M4CzwiCHOZxuWIjNI19EUjT2Cas2LO_LMxtX3fZ7gj0I-cDLLgONfqlELi9-xFMJplmuH9JwzMtwZtVM7zdjHw name:ekoDB Cloud price:499] score:0.1111111111111111]
  Snippet 3: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:A high-performance database product with AI capabilities id:wMa1RrsjKccyVAGDGNvZRM9CwXSdSfcaBXfvmRRaFQ9gRJqdedjzqAGN-9tHvpuG20NZJUBhWW5victwj9Grsg name:ekoDB price:99] score:0.1111111111111111]

Execution Time: 4599ms

=== Token Usage ===
Prompt tokens: 454
Completion tokens: 80
Total tokens: 534

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: kTuvQsmxuzosj6SCoNq3OqrvJS_xlHT4CEbj3eMGsZH5lvPn5eZW9o000HYpbxy5x6zKSXKN78Ne0P-8nXrZSw

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the context provided, the available product is ekoDB. It is a high-performance database product and is priced at $99.

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
✓ Created second session: 4ro2nsz3attRd93ZIQmS8CMbFnxwZnLU0AiIXFCx3wI71DKItAu8iu19Pv_gWkDSAkbXv578OyaCRNZlyKJN0w
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
✓ Created session: QgNaTKew4FukyFQpKPf4CDYmQLZpij-ZvQK1smny-h9u8n2GkvD5sxOPWGL5qBoaw1gpgODHHc8M8etF0zyo-A

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the provided context, the product available is called "ekoDB". It is a high-performance database product priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: QqFSyVVkEsi7Eio_GfWTeFJVP3wjUHwhD31rPnjrb5A2IX7Rx3uHM3t3wQLCaEH2wHykzLZi50E1rn3N011hnQ
  Parent: QgNaTKew4FukyFQpKPf4CDYmQLZpij-ZvQK1smny-h9u8n2GkvD5sxOPWGL5qBoaw1gpgODHHc8M8etF0zyo-A

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: QqFSyVVkEsi7Eio_GfWTeFJVP3wjUHwhD31rPnjrb5A2IX7Rx3uHM3t3wQLCaEH2wHykzLZi50E1rn3N011hnQ (Untitled)
  Session 2: QgNaTKew4FukyFQpKPf4CDYmQLZpij-ZvQK1smny-h9u8n2GkvD5sxOPWGL5qBoaw1gpgODHHc8M8etF0zyo-A (Untitled)
  Session 3: 4ro2nsz3attRd93ZIQmS8CMbFnxwZnLU0AiIXFCx3wI71DKItAu8iu19Pv_gWkDSAkbXv578OyaCRNZlyKJN0w (Untitled)
  Session 4: VpNXcfJoGqkGtMBJo6_-992FNbN4k1fkjof5BM0K4YnCRY-6eIkZOJG7iE2pXivA2enUbqp-3-B0fo0rUMp1-A (Untitled)
  Session 5: Tz0-kccR_F1ZJIXKHGuZUUh-OSQHxZvgNILU04xBnpRdus8Nv00sSZTJBvkAZREjP8N4x2Lw5aDIblS6G4eZ2A (Untitled)
  Session 6: 2z0WtTj1U5IXwDuns-OLvc06ilIGp6zB7yZaf7wad7Ru6Sw29Paelo2Gjm5rlKDSxNiWGZEGHkxIYyjb80OtkA (Untitled)
  Session 7: -xcLjFXFHh-R-Bgx66ye6uW9RTmOYwXWVrLNxRA_zWqoqCYc3HDdw43VqJuBFw7ls_RJHgAgn19-gp-OvXZT5g (Untitled)
  Session 8: KMwr3q9cYv9nTfjb72AWsbsObQ3cuvqfQbNusT2wPLICyzWzP1IfQF5XB5ldG0Hem5yDKb4J7sEPGEmAgiAtbQ (Untitled)
  Session 9: aQvWLRkM9xqrR1nW3QvfDY3m3S1phEtCVuLD_Uxz2NqHPcpjhqb924KHYMIzlA8Kih-Z9M5GUZopILk-dUgIpQ (Untitled)
  Session 10: qd_TGU8qeuze5yU4lqFmn79h5KzryiY3ccxd7zedBP-7_u26iP2GQZ185jMWyQ-b27s3MgvwD7-B33CxNh4Gsw (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: QqFSyVVkEsi7Eio_GfWTeFJVP3wjUHwhD31rPnjrb5A2IX7Rx3uHM3t3wQLCaEH2wHykzLZi50E1rn3N011hnQ

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Map Creation ===
✓ Created record with native map: map[id:1dnT7khrLccqJqsbKafocEdQ0NddC8lkHdBlethi0t3EX7DIl6L4ZjhA0ouU2BckcTl8XcGWUhA3ScwQ-ocKmA]

=== Upsert Operation ===
✓ First upsert (update): map[active:map[type:Boolean value:true] age:map[type:Integer value:29] email:map[type:String value:alice.j@newdomain.com] id:1dnT7khrLccqJqsbKafocEdQ0NddC8lkHdBlethi0t3EX7DIl6L4ZjhA0ouU2BckcTl8XcGWUhA3ScwQ-ocKmA name:map[type:String value:Alice Johnson]]
✓ Second upsert (insert): map[id:urlhufdqzvcSri6vFgNeLBSKWoVSpGkfwwlJpZVp1BKOQBhyOEAKpiAXvygYTyKdFdRoyZlSIxMrAToIFPYQWA]

=== Find One Operation ===
✓ Found user by email: map[active:map[type:Boolean value:true] age:map[type:Integer value:29] email:map[type:String value:alice.j@newdomain.com] id:1dnT7khrLccqJqsbKafocEdQ0NddC8lkHdBlethi0t3EX7DIl6L4ZjhA0ouU2BckcTl8XcGWUhA3ScwQ-ocKmA name:map[type:String value:Alice Johnson]]
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
Inserted with ripple: map[id:qIkUovowTiA8c-e_roI2xzo7nNjdmIUcMs7cNuUVU20mKcvbxSuD9q9cJEP87zDIdC16vCBc7GNONKYEYtOXzw]
Inserted with bypass_ripple: map[id:TIQdW1izNB5YHrswFtt_QtiVr0Z8GB-8_bPYD93coazHujh0fUJq3F15P4XmdCYobY2TTZvwqMRWQrDmIOaq_A]
Inserted with TTL and bypass_ripple: map[id:D-nqSF7wNj0d8t-33cJRyV2HrCSsctku3ZcErR48S6s0vWI33jNtbosUVQSgwsI5TOr5-OED5mFlI5FHWW2nRQ]
Updated with bypass_ripple: map[id:qIkUovowTiA8c-e_roI2xzo7nNjdmIUcMs7cNuUVU20mKcvbxSuD9q9cJEP87zDIdC16vCBc7GNONKYEYtOXzw name:map[type:String value:Product 1] price:map[type:Integer value:150]]
Deleted with bypass_ripple
Batch inserted with bypass_ripple: 2 records
Upserted with bypass_ripple: map[id:DpWGqrGSDw2QKjewkBQoiZAH4plUqAuYktAYFo4uC3xCXbVWvcFstFqSE-_b5ny3rwy3HSde_VrY2mVNEXG-8g]
✅ [32mGo client examples complete![0m
