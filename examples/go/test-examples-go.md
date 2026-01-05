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
Inserted: map[id:spQrscJWjGnFhXpz6QrJnEsgOqmx1goTLQSE6OlGCYi57SOE7FJly53b3KwlqMCLRie-9JI8aHOBsCX2TmiudQ]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] id:spQrscJWjGnFhXpz6QrJnEsgOqmx1goTLQSE6OlGCYi57SOE7FJly53b3KwlqMCLRie-9JI8aHOBsCX2TmiudQ name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query ===
Found documents: map[]

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] id:spQrscJWjGnFhXpz6QrJnEsgOqmx1goTLQSE6OlGCYi57SOE7FJly53b3KwlqMCLRie-9JI8aHOBsCX2TmiudQ name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
[32m✓ simple_crud.go completed successfully[0m
[34m
=== Running simple_websocket.go ===[0m
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: gNJIM5t-Z90GX2S83uajBb8Cf_25wpkB7Fp-GaV-ZpiQWf0plIOqYPCcPhkA5i0lNm-RyuZvqPAbCHUrfvb1PQ

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
        "id": "jCqqfNX1CH8oAOspCeoYtlibdQJHNY0vgR73fxLb98lsm2A2lhAlkEWs2K4JJ3fG0iHF4joH55M1tlEJ2xM6hw",
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
        "id": "t72bpt21jeSg4T5Hfo6Fk9U-NA8EZPcpKDWKpNfxWbDNAhHHFhjk2RkrVZVqsb_YoUfIXZy-UOQ7xUd1LDrPFQ",
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
        "id": "gNJIM5t-Z90GX2S83uajBb8Cf_25wpkB7Fp-GaV-ZpiQWf0plIOqYPCcPhkA5i0lNm-RyuZvqPAbCHUrfvb1PQ",
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

✅ Script saved: Ki31zNLjoj2T_O1HFSR6AU622LSqsSbPN9IoUrSQfZYpeURp8RzL9tw6OSApFm3SmAGj1pmLATFpCBc1qf0mjQ
📊 Found 25 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: ekOS-A0e8fiPtvmsz1VQAy1r-RLWaAQcvhB__24OHOvqfa745vI1SetszGyOLVAftsZwymqHp8o9uvLOb3EmWQ
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: AdcuWyR6fFmHk9idsycvwGktajYbLqBjA6VRIjP-nR6k_chgNdgLSDO_fPQw5433N9PbQRRQXleElQp2_aSdlA
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"avg_score":60,"count":10,"max_score":100,"status":"active"}
   {"avg_score":50,"count":10,"max_score":90,"status":"inactive"}

📝 Example 4: Function Management

📋 Total scripts: 28
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
Collection created with first record: BqOBnmW9cU0vqOSKuX2ZJ6gx_FRA4aJNoBm2Ew4z2UDBSHqJuPJLcBmZjNgnuQPz_U_zy5774C73S9AsyJ5eew

=== List Collections ===
Total collections: 17
Sample collections: [chat_configurations__ek0_testing schema_documents_client_py schema_users_client_py edge_cache_py ws_ttl_test]

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
Created Alice: $1000 - ID: lAfC7G-H6WJfgVQElX7JdBZNYlyQfF5JIdcBUYtjzf0TBV3AuC4suAdzLyXvHbX_qB4T5eoBJ5UM7Wnk2ubM6w
Created Bob: $500 - ID: 5hCyEcydmSqkAWTlFwjK2OhRmZzlVl-MJX9PZqPof_4lcOUdpuKL9A6U4vGbqV1GkPXE2feOmWL4dHcHprQSaQ

=== Example 1: Begin Transaction ===
Transaction ID: e7b7e7f2-d8c3-4053-b813-a3e6161bdf8c

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
New transaction: a749d827-cf87-4b22-bdc7-22c95fd087ff
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
   ✅ Script saved: 8kZRETBtWbblJMm1CjA2D8ZIOCdTSAJF5NIXnf7oOsdNWWXfgLrtxHZTphPLTGY4PigJIzt_an8qVc2c7cBuhw

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: ozLRZOc-4EBKn2sa1eiFBIG13zA78kMsmJ5-uxcETNZI7kSNaJgW3ZSjvMg20C1EKlhPOUyK7oZ3gje8AhNigg
   📋 Name: map[type:String value:Alice Smith]
   📋 Email: map[type:String value:alice@example.com]
   📋 Status: map[type:String value:pending]
   📋 Credits: map[type:Integer value:0]

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: 45zm8yN5LomC17JOiuk_ETgsHQ157ywnWSqRl_uOY0S3g6oWsLfxTh4LdrNzrZTakmNqzNd47OoNetig6UHD4g

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
   ✅ Script saved: Vydq20VY9CUSZZhSSdss9nZWGsC3dTWnCmVVJaDC-czinvyaoLPktVv-bj55eusm0-OzSNQRrv6dRa2j2t8vPg

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
   ✅ Script saved: OFvYV08606AQuQ2Hy_-h8miKLOcgaxzLpW8D0u-I-PjELf3bPUhHDCYe7IgMmPXNssMIw0oMyDxtwOqM6tMBXA

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: 8kZRETBtWbblJMm1CjA2...
   ✅ Deleted script: 45zm8yN5LomC17JOiuk_...
   ✅ Deleted script: Vydq20VY9CUSZZhSSdss...
   ✅ Deleted script: OFvYV08606AQuQ2Hy_-h...
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

2026/01/05 10:33:51 No .env file found, using defaults
✓ Client connected

═══════════════════════════════════════════════════════════
TEST 1: Document TTL Expiration
═══════════════════════════════════════════════════════════

[Step 1] Insert document with 3 second TTL
  Input: {name: 'TTL Test', value: 'should expire'}
  TTL: 3s
  Output: Document ID = JlOpdZPi5tS8rwbPJlEWb9eyj5J4NyhGrJo66aohE-TQWTgkGA-sR9YLK6-ZD9ukcxtrNV8eC1I5aNCxDJs-dA
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: FindByID(JlOpdZPi5tS8rwbPJlEWb9eyj5J4NyhGrJo66aohE-TQWTgkGA-sR9YLK6-ZD9ukcxtrNV8eC1I5aNCxDJs-dA)
  Output: Found document with name = map[type:String value:TTL Test]
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: FindByID(JlOpdZPi5tS8rwbPJlEWb9eyj5J4NyhGrJo66aohE-TQWTgkGA-sR9YLK6-ZD9ukcxtrNV8eC1I5aNCxDJs-dA)
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

2026/01/05 10:33:58 No .env file found, using defaults
✓ Client connected

═══════════════════════════════════════════════════════════
TEST: WebSocket TTL Expiration
═══════════════════════════════════════════════════════════

[Step 1] Insert document with 3 second TTL
  Input: {name: 'WS TTL Test', value: 'should expire'}
  TTL: 3s
  Output: Document ID = cGCe6TmfAU371vQxkOhkZGA6RrdoHRoHQHI7QiaCtBFA7t_aP36EFlsJKiZTjqlD0TCxN8W0xua6l_G5jK3H5Q
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: FindByID(cGCe6TmfAU371vQxkOhkZGA6RrdoHRoHQHI7QiaCtBFA7t_aP36EFlsJKiZTjqlD0TCxN8W0xua6l_G5jK3H5Q)
  Output: Found document with name = map[type:String value:WS TTL Test]
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: FindByID(cGCe6TmfAU371vQxkOhkZGA6RrdoHRoHQHI7QiaCtBFA7t_aP36EFlsJKiZTjqlD0TCxN8W0xua6l_G5jK3H5Q)
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
Inserted: map[id:koxA7K_BX4H-m5WNFiZhLUKAH_X2mP-BDva780FU0_mOyPyif0jSSuy09dqGWue2bd2xYxvn_ZNpPagFGnFavw]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-01-05T15:34:05Z] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:koxA7K_BX4H-m5WNFiZhLUKAH_X2mP-BDva780FU0_mOyPyif0jSSuy09dqGWue2bd2xYxvn_ZNpPagFGnFavw metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Test Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:42]]

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 0
  active (Boolean): true
  price (Decimal): 99.990000
  created_at (DateTime): 2026-01-05 15:34:05 +0000 UTC
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1 tag2 tag3]
  metadata (Object): map[key:value nested:map[deep:true]]
  embedding (Vector): [0.1 0.2 0.3 0.4 0.5]
  categories (Set): [electronics computers]
  data (Bytes): 11 bytes
Plain record: map[active:true categories:[electronics computers] created_at:2026-01-05T15:34:05Z data:aGVsbG8gd29ybGQ= embedding:[0.1 0.2 0.3 0.4 0.5] id:koxA7K_BX4H-m5WNFiZhLUKAH_X2mP-BDva780FU0_mOyPyif0jSSuy09dqGWue2bd2xYxvn_ZNpPagFGnFavw metadata:map[key:value nested:map[deep:true]] name:Test Record price:99.99 tags:[tag1 tag2 tag3] user_id:550e8400-e29b-41d4-a716-446655440000 value:42]

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-01-05T15:34:05Z] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:koxA7K_BX4H-m5WNFiZhLUKAH_X2mP-BDva780FU0_mOyPyif0jSSuy09dqGWue2bd2xYxvn_ZNpPagFGnFavw metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Updated Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: WbEEkYkgJFjG-TDr2voP_TO_R_dqu7BntklZUDSf47sj1Mnw6MaW52ZqgCbuqoATb9og1DfOqDoh5TGcLFn5CA

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
Collection created with first record: og2n3VqHt6uHiKdCsD1j2ImR2_MwTAeCwWSDtp8d9j7yPpUEqK2dg6fD4tFH-eglHkWWy52Ku702_mMT7RN7EA

=== List Collections ===
Total collections: 16
Sample collections: [chat_configurations__ek0_testing schema_documents_client_py schema_users_client_py edge_cache_py ws_ttl_test]

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
Created Alice: $1000 - ID: un1Dbfw_WKN6PBoKUDMc7yNSLOVbeGj7ivrIbDnEBIRM8BjO4FjJokjaPmOIZDgvP19SiLCn_muMsm6mXmMTjg
Created Bob: $500 - ID: -Mn5pQEMcM4-4AtaG0Y0oKpHjMN_cDsuojNQb7Eexxlegr7R3MjQLjQ_Ur3J1NlE0b661lux_-i0ubyi3gSulw

=== Example 1: Begin Transaction ===
Transaction ID: 0d4d3623-d9a8-4da5-b5af-9aeb6dd9a05b

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 10e152ba-fec8-4429-86c0-9cb97b0f40ed
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
  1. Score: 0.781
  2. Score: 0.774
  3. Score: 0.729

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.710
  2. Score: 1.513
  3. Score: 0.291

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
✓ Inserted document: Q9V4C9mz1GdMK2u8tz_MxM4bsqFP0Ro9pUxl6ivK1T1QzLi_-nxxzwNRKmJVhftBkJkJO7_W0o_FpeoiXPR0yQ

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: ylT5LWc7Lj-gIPQUPTiyBXCw0w49KugxXsrnejlp2vzejjjNcXW0njLHtvSSFerHqmL-9FxtNidFpH9IxJ1-jQ

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
✓ Inserted document with TTL: i6yw9nD8lXyKp1pFxOeoJ3eEGHB9A4jkM-ErcFvvO4gjnF3t0TdxFjl4_ImIrpxXyo0vsJwTm76Onfbpjk1ccg

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
✓ Edge cache script created: x3zhVyGeYEAhpU2QITQ0h_IR9rD2mTF-etXbIL6pD2J4sk9AL9-ufalUyZiRiOwkcOaVZk2msnI4t0snzHHq2Q

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

✅ Script saved: gzOny4Jfb7cx9r8-_pTV9Vh-wK7punwhrBbFuTBeTRu4y2hc4PB6ALPPSplBNMaxqcGlKBA4VvBqWpNXBNy8Xg
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

📋 Total scripts: 30
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
   ⏱️  Duration: 136.955041ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 85.388417ms
   📊 Records: 1
   🚀 Cache speedup: 1.6x faster!

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

✅ Script saved: fOBGaBu_dtxIQRzXkgWhzdYoq8nfwQayh4s03GvhJCo1lCWI3VRW0vadmaYTytgBOFS41sVicgK4ympkHXyCzA
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

✅ Inserted order: Mgc-urQoG1S9LnX3h2rv0udNVVKYvl8YmyS1dyex2QLQrQKsXu04hJMttlAxisiL0zJi5OCLie4hETJi5hcdLg
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: DTroZ1qsT7jgtq6XrhQh_EcchS4-hZs4W_cooVpsDtrisP-VI_5oREgYzEoGdXub0nN1mkJFmZqlcYR_VeIDfw
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: map[role:admin userId:user_abc]
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session
📝 Example 4: KV Operations in Scripts

✅ Script saved: vPV38SXyes3q4LhkE9kp8_EUKMJ0Ta4f2ygRc5MaXl8DKgnd5trXFdh28pxheSawk5VePoEvTEK6x38r6m6Elw
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Script

✅ Script saved: JhRVo_k9g0mENkRMfSMeY3si_k924a4LK1pcCMP6jmp_Wgb8M5ABUK4OPFlY4yMOcoUFHUbMWb6Kj0QRrvZwpA
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
2026/01/05 10:34:08 Save script error: request failed with status 500: Unhandled rejection: ErrorResponse { error: "JSON deserialization error: missing field `then_functions` at line 1 column 844" }
# command-line-arguments
./client_functions_advanced.go:73:36: cannot use script1 (variable of type map[string]interface{}) as ekodb.Script value in argument to client.SaveScript
./client_functions_advanced.go:78:27: invalid operation: cannot index result1 (variable of type *ekodb.FunctionResult)
./client_functions_advanced.go:81:25: invalid operation: cannot index result1 (variable of type *ekodb.FunctionResult)
./client_functions_advanced.go:107:36: cannot use script2 (variable of type map[string]interface{}) as ekodb.Script value in argument to client.SaveScript
./client_functions_advanced.go:113:27: invalid operation: cannot index result2 (variable of type *ekodb.FunctionResult)
./client_functions_advanced.go:118:25: invalid operation: cannot index result2 (variable of type *ekodb.FunctionResult)
make[2]: *** [test-examples-go-client] Error 1
