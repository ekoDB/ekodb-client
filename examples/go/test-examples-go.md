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
Inserted: map[id:hoDT7JQ2XpK1kyzMHhtxjFobaIMKbwCpyX8KXxV3CKGFGvZxG3dOwZYpY0teV64fqJAmXvB1VRqQ1_0sULtOqA]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] id:hoDT7JQ2XpK1kyzMHhtxjFobaIMKbwCpyX8KXxV3CKGFGvZxG3dOwZYpY0teV64fqJAmXvB1VRqQ1_0sULtOqA name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query ===
Found documents: map[]

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] id:hoDT7JQ2XpK1kyzMHhtxjFobaIMKbwCpyX8KXxV3CKGFGvZxG3dOwZYpY0teV64fqJAmXvB1VRqQ1_0sULtOqA name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
[32m✓ simple_crud.go completed successfully[0m
[34m
=== Running simple_websocket.go ===[0m
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: US1URz1oElP11pCefhxXASu8-cl5XivMknbWJiWq1kXVtN-5hRHQyja-29vCYjgUA0cRpJ_hJZcKdqayTZhPww

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
        "id": "ggUkVE8p-4-sgFujbNnfoo55fNFXQjXjuC4CFyYjUOz6nF6tI0-pZ_LBzcVVwNTxaLqn3-UX4dKYH11ySN0p4A",
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
        "id": "d2VJG5iv4Y2GM8EF5v5Bvs8hZucupbWnljtU8V09E6kHsVSL76g8IaS5XdJ9hRAvPBx73kHkr6R-756nGUvxHg",
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
        "id": "US1URz1oElP11pCefhxXASu8-cl5XivMknbWJiWq1kXVtN-5hRHQyja-29vCYjgUA0cRpJ_hJZcKdqayTZhPww",
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

✅ Script saved: Jrr9ewxBb2ZA2zGTfgDHMm4q3wYSvZ0uznztg7bGC--MbEP8tfxLSeb-dp8DOt4zIvWI96qXwi-5Vca24q5cfA
📊 Found 20 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: IiefJwTNsPNiSZujejg8A7aWd1ra818A6sYjYGzQ_llkvL-xcdpG3mYKXT5gX3agtdvo6flu9-nnyFJ-QPQiJg
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: F8Q7zB6I0PNLbfMh8J9fDCj5YmwXmHA-KrIDDDpOKMFrF_HbIZeOuglm65zV6xKSfa7immA8FKJGlyeNw6niMQ
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"avg_score":50,"count":10,"max_score":90,"status":"inactive"}
   {"avg_score":60,"count":10,"max_score":100,"status":"active"}

📝 Example 4: Function Management

📋 Total scripts: 12
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
Collection created with first record: za2YBWtPRoNpEXlk32vYnPp7IOPRyk6vxVeH2BB_m184IhlnX9U7KgCgORYs06VeORU7ui9j546L_u8sFM5OKg

=== List Collections ===
Total collections: 15
Sample collections: [ttl_cache users websocket_test chat_messages__ek0_testing test_collection]

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
Created Alice: $1000 - ID: 39s8XrkvJsBubPF_7hjUQet04MLLnED_eNiKlTGGMGDYvHOFAEMorSI_Iq75qus1gBtBNFnwzrWHtSRr7qFmqQ
Created Bob: $500 - ID: TYcUJPSBhmiM9aa5fmSfaBk4Aii2ImAzq1olE_GAfWRRMe2H2hPEcYmmm2jzBBllhouODuzywUy96BAxzw9-jg

=== Example 1: Begin Transaction ===
Transaction ID: 9ebeb764-60e6-4554-8d73-35fc042a2364

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
New transaction: 7cf8bc22-7e25-481e-a367-558adc7e1d22
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
   ✅ Script saved: 4ENwO71Wwx9C-KrQT7mbMRDQg4jjT-UxxW5w5jnKW6zesK9HDG3xlL4P-QJ8oEEuJgWmAEVv90RFIpFNtHhwOw

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: n8CHzyd_fhTYM_08DmRW6jKdFmg8OYyy3rSKnAhk2KRL80lPc21kQdDIMp809MX761YgXVUXQPVnTRtxXCWmSA
   📋 Name: map[type:String value:Alice Smith]
   📋 Email: map[type:String value:alice@example.com]
   📋 Status: map[type:String value:pending]
   📋 Credits: map[type:Integer value:0]

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: FtE0581ce19U7QUKeJz0v-DRltdZH5NEkKpFCOH6y5qdMwruAEd_JibfGwZPw4iVreGTWGaknKiL_dABCK5LHA

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
   ✅ Script saved: i-gY6g4xGQ0pW3yc7H0AH9B1eHGGP7Md2ikfAzfUktTZAHfftt53U4SIk0OIzEpta9ezX_QKXcA1jOHBCxBJzg

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
   ✅ Script saved: JWOtdrh611kMyL5UVOtfNC3nM63RYK5tMmXKLfb1bw1vDKczkFCiN8Zt3u0AMPh3R17eWdbC4thx90KzEp8lOw

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: 4ENwO71Wwx9C-KrQT7mb...
   ✅ Deleted script: FtE0581ce19U7QUKeJz0...
   ✅ Deleted script: i-gY6g4xGQ0pW3yc7H0A...
   ✅ Deleted script: JWOtdrh611kMyL5UVOtf...
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

2026/01/04 17:30:28 No .env file found, using defaults
✓ Client connected

═══════════════════════════════════════════════════════════
TEST 1: Document TTL Expiration
═══════════════════════════════════════════════════════════

[Step 1] Insert document with 3 second TTL
  Input: {name: 'TTL Test', value: 'should expire'}
  TTL: 3s
  Output: Document ID = HxBqnZ-4W4RO6YoHeSFy5CE5ziXshc78UZ6qktf0T6UDeX19zs-nisE86xW0PSBR1ds-91ZUE_BSiWWjiWxONA
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: FindByID(HxBqnZ-4W4RO6YoHeSFy5CE5ziXshc78UZ6qktf0T6UDeX19zs-nisE86xW0PSBR1ds-91ZUE_BSiWWjiWxONA)
  Output: Found document with name = map[type:String value:TTL Test]
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: FindByID(HxBqnZ-4W4RO6YoHeSFy5CE5ziXshc78UZ6qktf0T6UDeX19zs-nisE86xW0PSBR1ds-91ZUE_BSiWWjiWxONA)
  Output: Error (expected) - request failed with status 404: ��Record has been deleted
  ✓ PASS: Document expired (not found error)

═══════════════════════════════════════════════════════════
TEST 2: Multiple Documents with Different TTLs
═══════════════════════════════════════════════════════════

[Step 1] Insert 3 documents with different TTLs
  Inserted: 3s TTL=04r2kqzI, 10s TTL=pIWo5_iY, Permanent=_idygP-o
  ✓ PASS: All documents inserted

[Step 2] Wait 5 seconds (3s doc should expire)
  ✓ PASS: 3s TTL document expired
  ✓ PASS: 10s TTL document still exists
  ✓ PASS: Permanent document still exists

[Step 3] Wait 7 more seconds (10s doc should expire)
  ✓ PASS: 10s TTL document expired
  ✓ PASS: Permanent document still exists

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

2026/01/04 17:30:47 No .env file found, using defaults
✓ Client connected

═══════════════════════════════════════════════════════════
TEST: WebSocket TTL Expiration
═══════════════════════════════════════════════════════════

[Step 1] Insert document with 3 second TTL
  Input: {name: 'WS TTL Test', value: 'should expire'}
  TTL: 3s
  Output: Document ID = vQhAnH3IZOth5rxV1ecfIx1FM_GS1WCKz51BzRkroHx1LkzOXTI6_yhjMO8yf5JEhQeDMhXaiBoQnImBcb6leA
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: FindByID(vQhAnH3IZOth5rxV1ecfIx1FM_GS1WCKz51BzRkroHx1LkzOXTI6_yhjMO8yf5JEhQeDMhXaiBoQnImBcb6leA)
  Output: Found document with name = map[type:String value:WS TTL Test]
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: FindByID(vQhAnH3IZOth5rxV1ecfIx1FM_GS1WCKz51BzRkroHx1LkzOXTI6_yhjMO8yf5JEhQeDMhXaiBoQnImBcb6leA)
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
Inserted: map[id:yGn8nPXGLgsRv2r1D1-GqYqV_mu-bQH1xWi-_UAH416YGvx1hnfRvKRepIb7hKNiNOtcr5-XS5t8nec6I28b3A]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-01-04T22:30:55Z] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:yGn8nPXGLgsRv2r1D1-GqYqV_mu-bQH1xWi-_UAH416YGvx1hnfRvKRepIb7hKNiNOtcr5-XS5t8nec6I28b3A metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Test Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:42]]

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 0
  active (Boolean): true
  price (Decimal): 99.990000
  created_at (DateTime): 2026-01-04 22:30:55 +0000 UTC
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1 tag2 tag3]
  metadata (Object): map[key:value nested:map[deep:true]]
  embedding (Vector): [0.1 0.2 0.3 0.4 0.5]
  categories (Set): [electronics computers]
  data (Bytes): 11 bytes
Plain record: map[active:true categories:[electronics computers] created_at:2026-01-04T22:30:55Z data:aGVsbG8gd29ybGQ= embedding:[0.1 0.2 0.3 0.4 0.5] id:yGn8nPXGLgsRv2r1D1-GqYqV_mu-bQH1xWi-_UAH416YGvx1hnfRvKRepIb7hKNiNOtcr5-XS5t8nec6I28b3A metadata:map[key:value nested:map[deep:true]] name:Test Record price:99.99 tags:[tag1 tag2 tag3] user_id:550e8400-e29b-41d4-a716-446655440000 value:42]

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-01-04T22:30:55Z] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:yGn8nPXGLgsRv2r1D1-GqYqV_mu-bQH1xWi-_UAH416YGvx1hnfRvKRepIb7hKNiNOtcr5-XS5t8nec6I28b3A metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Updated Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
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

=== Create Collection (via insert) ===
Collection created with first record: D0VfzeWpEyAg4mdrr3mB1qGxIzuSrTfl4OKV_xQGSduJzzz7W_FYIHfMvUKJoBOTmFaIJ8XILgUAVTYrI0QYkg

=== List Collections ===
Total collections: 14
Sample collections: [client_collection_management_go ttl_cache websocket_test chat_messages__ek0_testing test_collection]

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: I59o_ug3zNuxP1qQyG-6XuBmr142W7oR5Uw7IuxM9dosKq56_52JY6SUQmXxaOyg0CjBvJEYsSb7BQATIEeTQQ

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: ob9vx1TbMsr1Jgh4POhpJvSeDB2H9w-yJ5fVkqrqkkFZUDhqNRy16UCfqd5c3SuN1qodh8lz2TLNn2-DQCflTw

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

=== Inserting Test Data ===
✓ Inserted test record: twkefirzEY4ou6HbEmlDW6Vh0DbyWw6RF0HEKKiXK-8JP-sKK8XX8jwTcm5bRxomqGisLEqD-ASLO5mu1sWi5g

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: Pv7Y3NV2Kzks3BfFE8rmyI9C478HP8b_5VeG95_ftbEfwpEd2KdtG5VooSkW21N9szSNna1qfwp1QN49PwG8Fw

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 5 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: Qyvsy0PzVbiBeghb9hUiKkm9jkqwn0zqoSrh-sUl0FxvsU7IdcSFGYFMzXIP0bqVZJLQrKxGOYxDiL50740Fuw

=== Sending Chat Message ===
Message ID: G6qd-a-7T9j5SK2SuPxwqnQmfk6lflQtCZT53MrEbmCPq4MSb8fi_GRMsd-EF2yyHM8p3DG5oqiWyTOX46IWtA

=== AI Response ===
The available products and their prices are as follows:

1. ekoDB: This is a high-performance database product with AI capabilities. The price of this product is $99.

2. ekoDB Pro: This is an Enterprise edition product with advanced features. It is priced at $299.

3. ekoDB Cloud: This is a fully managed cloud database service product. The cost of this product is $499.

=== Context Used (3 snippets) ===
  Snippet 1: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:A high-performance database product with AI capabilities id:QHE4Fzf6BzLZ2_7o10QaIxQyowUWJ9B3W0U7sThM_dcz1SP3mLxZYAUBOkMUQAf78lPQ1ClY4GPNrcML4jXlaA name:ekoDB price:99] score:0.1111111111111111]
  Snippet 2: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Enterprise edition product with advanced features id:CS0jE23tMSkjekiT6j_VbzyPGE2jFfk_Oof7nwIwTLYkj272DQVk7UgxRTqJEJO26jFJN-jGx3I_32IU8zNq2Q name:ekoDB Pro price:299] score:0.1111111111111111]
  Snippet 3: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Fully managed cloud database service product id:QzsmVSm2UEUX5x7--spGpYE1TOst9Hg-OrCb9ZNrOXFCSjslJo_tWCD4KjF7r8QSqxQIyYOk-17YA6t6qpRVlQ name:ekoDB Cloud price:499] score:0.1111111111111111]

Execution Time: 2775ms

=== Token Usage ===
Prompt tokens: 610
Completion tokens: 86
Total tokens: 696

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: Gu_xIGD5MefdoNTeJDkJ0I4d3RSsP9ER4PGpxn03S00E8P1XDAGDW9F6VOqwiNH5nVOvIOO25KjqjHwrAQDw4w

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information provided, the available product is ekoDB. It is a high-performance database product that costs $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: Ss7H6_sdaz85IWQS28SqRuXgfrQ8rlcG5DCeeNeR35czVlG7EMZTAOUG3ln83VYvXAZ5EHObpVmCVb_dLwUJLw
  Parent: Gu_xIGD5MefdoNTeJDkJ0I4d3RSsP9ER4PGpxn03S00E8P1XDAGDW9F6VOqwiNH5nVOvIOO25KjqjHwrAQDw4w

=== Listing Sessions ===
✓ Found 9 sessions
  Session 1: Ss7H6_sdaz85IWQS28SqRuXgfrQ8rlcG5DCeeNeR35czVlG7EMZTAOUG3ln83VYvXAZ5EHObpVmCVb_dLwUJLw (Untitled)
  Session 2: Gu_xIGD5MefdoNTeJDkJ0I4d3RSsP9ER4PGpxn03S00E8P1XDAGDW9F6VOqwiNH5nVOvIOO25KjqjHwrAQDw4w (Untitled)
  Session 3: Qyvsy0PzVbiBeghb9hUiKkm9jkqwn0zqoSrh-sUl0FxvsU7IdcSFGYFMzXIP0bqVZJLQrKxGOYxDiL50740Fuw (Untitled)
  Session 4: Io01vXhsW8BkPYUCUUTLdDWRPT21aGynTNwl8dKLlJ6X24xSeN2mBCOkKES2IJnJ9F6h51vrnyjWWQLYmuyOMQ (Untitled)
  Session 5: j6jNCKZTN-N6CxuK9wi6SxzdbbBMKWEGWrM8AniwBQpg_SWyD86sGW2NZ5rI63Z7R7RlpXjn8nsPwHQcT3VslQ (Untitled)
  Session 6: LJ5cZyqHoXs52cGWUxESsBowN9WD_4okFMLRbpD8-ggFUhYqKZPFaRlUDCmTo8J2LkXgnMPzPinMwwzCxLmR8w (Untitled)
  Session 7: zgZmlmVFLWGNLEdEB26uHagZbr2ZF7802mz_wjqfZqon0azZ3gofLs6k7o8B_odODTl89yzN_di8oFEaFf0y8g (Untitled)
  Session 8: mA61Yn2joaX-S0mdSdsaajZiffk0LtVAs6i9T-bDcik-YewWD2js2py8GGeFemTIxR6pDVrFEWI6WsaCqFGwUA (Untitled)
  Session 9: DhIh4JKtzyz-P_VwbAkLSXZCjZtY0TlcbaSniFvYPnmyL6GKGRUO4aUqqhc3KeIZqzMADTi3NFtcGZeLK9UFTQ (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: Ss7H6_sdaz85IWQS28SqRuXgfrQ8rlcG5DCeeNeR35czVlG7EMZTAOUG3ln83VYvXAZ5EHObpVmCVb_dLwUJLw

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: m9jHcYUkhcwOfx9DOjIggjk8gqd0AAOZDufKhLBbQoPqEgPjHX5D1b3NfdrQ7qUD3qxdWrC-J8eUVHIeX3-PKA

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information I have, the available product is "ekoDB", which is a high-performance database product. It is priced at $99.

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
✓ Created second session: NZ3y677q6jvHnfJfB7cTEZQef4i2DBvbBXefm61pJxn-HBf9p7So7p4zJUjILngY864c_I2FcDV6_j51na4qAA
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
  1. Score: 0.763
  2. Score: 0.761
  3. Score: 0.748

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.704
  2. Score: 1.505
  3. Score: 0.299

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
🚀 ekoDB Scripts Example (Go Client)

✅ Client initialized

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: EfV84tbVU9_UwKIXGOiHykxXJOWEUmkkZ9VPbixL8sNzaz2AV-DhFbBayoUl7bY061IFCLUCIZej7bm1Qd0GTQ
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

📋 Total scripts: 14
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
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: uAz0RZXUo-aFO2RBl-wyeRH8GHkVVweyPifV6I08F9z46tG6um2kT1csHhc_XYwgjkdbKfbG9glfEzwZYTCxOQ
Created Bob: $500 - ID: 0INQG-q2NOfpiXhaXmhUfTVQazzJNAyPFFNI1FFplY8iJWRWhoNd2oAdQjTnSguz72ZtnPlW7ZCviaRjCFFAhA

=== Example 1: Begin Transaction ===
Transaction ID: f9c60b28-d29c-44cf-a739-251c05e091d9

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 6046a809-275e-420e-b6b0-9b14e85827a8
Status before rollback: Active
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
✅ [32mGo client examples complete![0m
