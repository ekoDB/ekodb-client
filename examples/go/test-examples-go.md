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
Inserted: map[id:JoqsWbHMDAHLn7iVPRzBGHnZ3dkXlRMoY5iH1uDRTQiXVNRrL-d4-Kj1VcdjPeX7eJcj8RBvIw4Bi0hTLOVJIg]

=== Find by ID ===
Found: map[active:true id:JoqsWbHMDAHLn7iVPRzBGHnZ3dkXlRMoY5iH1uDRTQiXVNRrL-d4-Kj1VcdjPeX7eJcj8RBvIw4Bi0hTLOVJIg name:Test Record value:42]

=== Find with Query ===
Found documents: map[]

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] id:JoqsWbHMDAHLn7iVPRzBGHnZ3dkXlRMoY5iH1uDRTQiXVNRrL-d4-Kj1VcdjPeX7eJcj8RBvIw4Bi0hTLOVJIg name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
[32m✓ simple_crud.go completed successfully[0m
[34m
=== Running simple_websocket.go ===[0m
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: 5hJY8xNIE_yBjOKap9rIhD9xSjQwmq22-i2n8dejL3ukAO1L14NWPI-e_ItHJxYuPlZccDwnp8KqS_oNQYYPVg

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
        "id": "pcH7DNujG1CstAOdqZ9y9CWA6UOITf7bMp3t8WIPEoFoCrnix0MgigTh0pgJ8vQZKPG4SajrkXJxvH3_GZD-MA",
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
        "id": "hqv3ZeE80SGWiiGHjNPjmodyzWaTYy5FkLK3O98nFYXHTzeIusk1MyTtfMT4roJOF9NhJp4yMqsxgGZOgbaAiA",
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
        "id": "5hJY8xNIE_yBjOKap9rIhD9xSjQwmq22-i2n8dejL3ukAO1L14NWPI-e_ItHJxYuPlZccDwnp8KqS_oNQYYPVg",
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

✅ Script saved: 6bg37w2E0UaP5EG_8TlHrOWHAVEpKHOLZplPD1k8ZYwqIzjZmpV1F2hsqTlp0NIG35S50OhrjKWPCF33qgSmwg
📊 Found 10 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: 1F6e-5sY7SSL6dueMEYHnPerrx3tk38Ji--C73IbuKrB_mYy20taMgh5fwDhUyS-eksh_ltCpvOcQGQzuBPtvQ
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: FfkApau2dX_-pOvy4iKsHj-jYuvU1756ANHQjVA_iOLQvHvNAZBWWcxlKwp2Xjs0HfJzeAD7R0gLCgX5-P3SRw
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
Collection created with first record: A17tjXZscK2QvqbMAeAk1OQ9B5M-jUciO-n4N2I6GDZFU4xVlEbGwl2_OP5UYyas0ZopW4cjM3aUnpc2Z4qZMQ

=== List Collections ===
Total collections: 16
Sample collections: [batch_users schema_employees_client_py ttl_cache chat_configurations__ek0_testing websocket_test]

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
Created Alice: $1000 - ID: 9waahSIMWhvoXeDsXSH4qw4GNNQ64oj3ZW4JExiJVXK2YVoU4y62mu2u61_vtZAGRLyow7uPLTfpLE84HLtT0Q
Created Bob: $500 - ID: sqqFKUHN8-wLoGh671Zlk-qYlJfOM2bP5pkqp10nVNN7Ub2esu3ZC6JzLLVq3vYVjmWOMqFs021G3QeQCTNShQ

=== Example 1: Begin Transaction ===
Transaction ID: 28f06bbe-e5d1-441c-ba5f-8ec7934d834d

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
New transaction: 46386a5d-6d84-4b04-ba73-b0b0940e24a6
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
   ✅ Script saved: ou07_CpjGuOjeVBvhkr2jtAZqCahdPTeu8nX8OOGHI6-VTxXRtXRYxhgrt2I9B3mtKyRzw5HDL-hpA_LyywnaA

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: R4CVvXpi6csBaIU2oUu2cELZ3_OvCigMxL7qun9JKY9FHCkG32bBqO9pETGptsf4dRYafiSznb7B-DcKklPHww
   📋 Name: map[type:String value:Alice Smith]
   📋 Email: map[type:String value:alice@example.com]
   📋 Status: map[type:String value:pending]
   📋 Credits: map[type:Integer value:0]

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: Ni3OFV7jpMXGw_-5XqY4bOdWpDldQx8kxBd0YAga2PydWcpwWdchTZaNeEDxu2tr-bVm2SbzUuqs_L7oMuWzcw

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
   ✅ Script saved: l-eNi13IEN1GeFNtFEQgWY_U5QWrbOu7RaxNL9PINq090lQsBE1p2tHpx_ML1YnhWPl0bL4pj9BkRm03-dUvjw

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
   ✅ Script saved: O6i4gfUlJRAlwMCpojbb3KN7et2cbCM0vslJfNQ5ByxiGN0dOxTSeKtmHcRLeaKW7J0Ibu9mmk09I0mnmmpGDg

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: ou07_CpjGuOjeVBvhkr2...
   ✅ Deleted script: Ni3OFV7jpMXGw_-5XqY4...
   ✅ Deleted script: l-eNi13IEN1GeFNtFEQg...
   ✅ Deleted script: O6i4gfUlJRAlwMCpojbb...
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

2026/01/26 10:06:02 No .env file found, using defaults
✓ Client connected

═══════════════════════════════════════════════════════════
TEST 1: Document TTL Expiration
═══════════════════════════════════════════════════════════

[Step 1] Insert document with 3 second TTL
  Input: {name: 'TTL Test', value: 'should expire'}
  TTL: 3s
  Output: Document ID = CkZU1aOhb_AtTIcYt_33vI2Aot64UDBZnvVuYQlOb1F1CiVtLUhCgYSKCPek0XQFubREWZ88knAs5q01Y-q0sw
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: FindByID(CkZU1aOhb_AtTIcYt_33vI2Aot64UDBZnvVuYQlOb1F1CiVtLUhCgYSKCPek0XQFubREWZ88knAs5q01Y-q0sw)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: FindByID(CkZU1aOhb_AtTIcYt_33vI2Aot64UDBZnvVuYQlOb1F1CiVtLUhCgYSKCPek0XQFubREWZ88knAs5q01Y-q0sw)
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

2026/01/26 10:06:09 No .env file found, using defaults
✓ Client connected

═══════════════════════════════════════════════════════════
TEST: WebSocket TTL Expiration
═══════════════════════════════════════════════════════════

[Step 1] Insert document with 3 second TTL
  Input: {name: 'WS TTL Test', value: 'should expire'}
  TTL: 3s
  Output: Document ID = C5KkvrMBEgYE_7ylOLSTCQEhaJX2m0CT58SP-JU9UnDnXR2VQEVzk3NhiawD8rGjhRnfSb1bv1NBx1W-by8MTQ
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: FindByID(C5KkvrMBEgYE_7ylOLSTCQEhaJX2m0CT58SP-JU9UnDnXR2VQEVzk3NhiawD8rGjhRnfSb1bv1NBx1W-by8MTQ)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: FindByID(C5KkvrMBEgYE_7ylOLSTCQEhaJX2m0CT58SP-JU9UnDnXR2VQEVzk3NhiawD8rGjhRnfSb1bv1NBx1W-by8MTQ)
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
Inserted: map[id:QG5baRjztrCNCwC7mFCcic6XlwrZDgnfJ8zMRzRxCRnr_NNFkkL24qlz43AUT5Ha3nWIrHMwSUIbfJz2Fu0WWA]

=== Find by ID ===
Found: map[active:true categories:[electronics computers] created_at:2026-01-26T10:06:17-05:00 data:aGVsbG8gd29ybGQ= embedding:[0.1 0.2 0.3 0.4 0.5] id:QG5baRjztrCNCwC7mFCcic6XlwrZDgnfJ8zMRzRxCRnr_NNFkkL24qlz43AUT5Ha3nWIrHMwSUIbfJz2Fu0WWA metadata:map[key:value nested:map[deep:true]] name:Test Record price:99.99 tags:[tag1 tag2 tag3] user_id:550e8400-e29b-41d4-a716-446655440000 value:42]

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 0
  active (Boolean): true
  price (Decimal): 99.990000
  created_at (DateTime): 2026-01-26 10:06:17 -0500 EST
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1 tag2 tag3]
  metadata (Object): map[key:value nested:map[deep:true]]
  embedding (Vector): [0.1 0.2 0.3 0.4 0.5]
  categories (Set): [electronics computers]
  data (Bytes): 11 bytes
Plain record: map[active:true categories:[electronics computers] created_at:2026-01-26T10:06:17-05:00 data:aGVsbG8gd29ybGQ= embedding:[0.1 0.2 0.3 0.4 0.5] id:QG5baRjztrCNCwC7mFCcic6XlwrZDgnfJ8zMRzRxCRnr_NNFkkL24qlz43AUT5Ha3nWIrHMwSUIbfJz2Fu0WWA metadata:map[key:value nested:map[deep:true]] name:Test Record price:99.99 tags:[tag1 tag2 tag3] user_id:550e8400-e29b-41d4-a716-446655440000 value:42]

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-01-26T15:06:17Z] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:QG5baRjztrCNCwC7mFCcic6XlwrZDgnfJ8zMRzRxCRnr_NNFkkL24qlz43AUT5Ha3nWIrHMwSUIbfJz2Fu0WWA metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Updated Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: EnoN86Ly3d0g00SNrQO6iyn4NDXKp7fF3Cr9MyEnWR6xQtavaapkK-MMSdzCnMl8AvND6mZVtpGSFO4If7HDLA

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
Collection created with first record: HOLCFVBup6r1GiDdeu8y_0to9Xe0-iHU6r5__UHZPiANK6gT_PrNjELfmDtHZrQwiDJAxgYoA8xvD3_Fm0vd2w

=== List Collections ===
Total collections: 15
Sample collections: [batch_users client_collection_management_go schema_employees_client_py ttl_cache chat_configurations__ek0_testing]

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
Created Alice: $1000 - ID: j9P3TcfXnIqgVX1eU-LRgyx2b7tOXc0sP4yFcpjZaCyC2pIlXc00PObKORYMXpy7fqjDJ7dqVMCUWTSp4C7lkg
Created Bob: $500 - ID: rJFiQ3_SCC8wVPnrWAMT5LmBuqJaSUOijWxmQ3wTJNYE192fp8p9SQuoHUoDS2DX9U_uc3YwVZcvcqbvPVGpFQ

=== Example 1: Begin Transaction ===
Transaction ID: 2185196d-d82f-4c90-a34f-1d764801a887

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 94ad4c0a-ad3f-4a21-90a6-02638d88003c
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
  1. Score: 0.766
  2. Score: 0.745
  3. Score: 0.736

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.706
  2. Score: 1.494
  3. Score: 0.298

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
✓ Inserted document: UwoinCCDu8yalWrWM634SWGwZ-FBIakFrSDlUwOmA1bqC1g8Bubr-Tdk7oiZz6XpUarwp6i6X_ky77p_-Cr9bw

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: iT1li29JCFa-og9I_H2s1VF2d63fdvlvGXe-RM0H1spRQxKQkRANnQyC2GtgGUXwXYmYFfi4d2zKBba1uN2Stw

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
✓ Inserted document with TTL: TM9lG1_jLs45wLmdCnUYuTr9WBRInO37AUasR8fvEqeWVa39m65a8cTkar7lF4YS5aaORhF9IeKUI1XI1FZ__A

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
✓ Edge cache script created: xidufuZ2QjjTNDssnYgUtUCTz9Z-Ug2DFQ9ua2glWWVKTJ7x4XGavsXTTci-B3-flr26MeqzYcnE4TKg-Q-ZIA

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

✅ Script saved: -Q8XwOCQ1CPeHwHOrkxSP54YYcUlwiyYpZfkuNxOCmc93f_JgEAC3MQqsvdJk09JaP17QkS5qUzziRCeSgxpzQ
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
   ⏱️  Duration: 70.787084ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 69.793958ms
   📊 Records: 1
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

✅ Script saved: Rg2fMqjb_olxT75mjWjt2KYwSvFuINaCch1eb63Kp61Jw_xYn3LHNpyx0PKactvFuPrPtoMWDBB-WrkLBjMUFA
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

✅ Inserted order: jnUZ10cbjzmpthiGJjLPbE1WgBkQs5u5w-vl8F3-A_vHuqsK71h4w1R31t_rBNLv2GOc3q8ApONmmG13__03gw
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: GmVhI2zfdAiglKBCiha7-NTJDf3BdYwyUjxQER1ijruOJKIy2i4gdlESVQdqWtREfCDT15CyDAP2eGp5rt5VWg
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: map[role:admin userId:user_abc]
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session
📝 Example 4: KV Operations in Scripts

✅ Script saved: TSHajlMjAEuGfMo2nAVS0fExyDVqGb4Kz8VYMRfERDmUi8vaSTf9dXA9CsMZ-SsIhB0OHN7LMgLaZw64jaKWCA
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Script

✅ Script saved: LzRJeZEQeJUDnsGgNJab9YOvr5XZvUWV_DLt7wjMoTXp3AMQkmwpLdz53Txd_7jrFB3IUe5K4ujho4sX-OMYqw
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
✓ Created SWR script: fetch_api_user_go (210lsOVvg2f3Q8P7bHJqBhnlI2cpC5ZxoJPE5zzqmkRJ8aU49D-cg6bAw4JCwTZJcbCKwFrsIxDKJs5o65FWYg)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "cached_at": "2026-01-26T10:06:25-05:00",
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
      "id": "1sPyw-c3AEk_m0qgiZ2W4l6IWhtB-PPUBXEiGhef3Ln5WlYTONSIIaNcw57uTdmKuajmgW9l1mBxsCaT4a96NQ"
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
   1. map[type:String value:Getting Started with ekoDB] (map[type:String value:Database])
   2. map[type:String value:Vector Databases Explained] (map[type:String value:Database])
   3. map[type:String value:Introduction to Machine Learning] (map[type:String value:AI])
   4. map[type:String value:Database Design Principles] (map[type:String value:Database])
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
✓ Created session: pk7aA3dAX2d3eAynAmikTPNZOCiz1gvd-t23welEmVxmgiKenDhkabrvAxJJCY4a187SCvG2PnwjZv7g2Sk8Ow

=== Sending Chat Message ===
Message ID: hyK4Q6l8s3qX_4UNAH6lxkDibXk5j-uGwDmpjMhvpQqT-SGNUnaxUWZ_Mk8wzXxPhiRVpFfTvWqwUqQgihI0yg

=== AI Response ===
There are three products available:

1. ekoDB Cloud: This is a fully managed cloud database service product. The price for this service is $499.

2. ekoDB Pro: This is the enterprise edition product with advanced features. It is priced at $299.

3. ekoDB: This is a high-performance database product with AI capabilities. The cost for this product is $99.

=== Context Used (3 snippets) ===
  Snippet 1: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Fully managed cloud database service product id:eykdUWtuLbsW4lHIPE0ytUfyX1R6rVv1nyKH4D-9xfKjUUuiLbdjklpIGaFSFr_dBfKiU1_6hgiHudTomznX7Q name:ekoDB Cloud price:499] score:0.1111111111111111]
  Snippet 2: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Enterprise edition product with advanced features id:ScV_RBjxN-c47-oKsM0hBr63UaJefVG03IwDTp4DQ2M6fGH4xtjRL2ghuImUDtZZxYgjf7CQcufuWWVzlg4Qvg name:ekoDB Pro price:299] score:0.1111111111111111]
  Snippet 3: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:A high-performance database product with AI capabilities id:hH287t2HiG_bhwImH3_Q1FjPvSIY1EX2jQHBYBUlgkH7uc-GNicqkjx7FzE1Kk2GnQiSszTITfkitCkgv5XBZw name:ekoDB price:99] score:0.1111111111111111]

Execution Time: 3422ms

=== Token Usage ===
Prompt tokens: 452
Completion tokens: 82
Total tokens: 534

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: fMwi9B7TrTfKJBGmU5eQKXMYLDrKPVK3v63ShN1-_o2DHl8iwAqu-VLXdgFzGlaZP1pfv5D0-76Ity3syizbzQ

=== Sending Initial Message ===
✓ Message sent
  Response: Currently, we have the ekoDB available. It's a high-performance database product, priced at $99.

✓ Second message sent

=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of the ekoDB is $99.

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: D4Z_YfTWI7OhM6qRxOYYxzbG94AXkSb6FBwL_qCLZ2a2Xy4gOwGLjEmD0XswFmozj2pgAXGv_Tmm9VUd1b--hQ
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
✓ Created session: XVLAxRIZ2pkRmHp9G4_7Fxu3Q3sWMtIfMsNr_xL7N20PJT9DHMvXAk9fn_d3yknl2b_WHniFz3mwSyofDnGjVQ

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the context provided, the available product is ekoDB. It's a high-performance database product and it's priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 4WYDp0TVacNY2KT8sELKiYWkfyKThAvdCJiWnlHmR1uGcFtE5syBJMA7CSHW_2uFBbXTSocRQL7BUkavp1qiQA
  Parent: XVLAxRIZ2pkRmHp9G4_7Fxu3Q3sWMtIfMsNr_xL7N20PJT9DHMvXAk9fn_d3yknl2b_WHniFz3mwSyofDnGjVQ

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: 4WYDp0TVacNY2KT8sELKiYWkfyKThAvdCJiWnlHmR1uGcFtE5syBJMA7CSHW_2uFBbXTSocRQL7BUkavp1qiQA (Untitled)
  Session 2: XVLAxRIZ2pkRmHp9G4_7Fxu3Q3sWMtIfMsNr_xL7N20PJT9DHMvXAk9fn_d3yknl2b_WHniFz3mwSyofDnGjVQ (Untitled)
  Session 3: D4Z_YfTWI7OhM6qRxOYYxzbG94AXkSb6FBwL_qCLZ2a2Xy4gOwGLjEmD0XswFmozj2pgAXGv_Tmm9VUd1b--hQ (Untitled)
  Session 4: pk7aA3dAX2d3eAynAmikTPNZOCiz1gvd-t23welEmVxmgiKenDhkabrvAxJJCY4a187SCvG2PnwjZv7g2Sk8Ow (Untitled)
  Session 5: HFG9kxwFQSLLrPWzsffCsQoAqtrWGP867ZAGED-kiFKNCjBAroYOXSMr96SDrNcSy6IB22lwZ6pUe59bwrjPuA (Untitled)
  Session 6: O9hPELmjeoF5xcC-AXO5BYlAw51P13QCevQPKOms7urZi09TugPjOlCyJwNaWCAPQHtBPKSG5G77SG5izjXBdQ (Untitled)
  Session 7: WEGypk7qoA_kChE-4sfyjDdjSAsXDRjBIfhNexFlxd_H5EtOlhbH93hvkk0xguAi-36XXCmnH-riuzJDgvxy9A (Untitled)
  Session 8: b3lg9Yy6htKsJGVHXEZH9XVx52hF9v35CMNfcM9ODLsCTUXxJBBw7oPw2uhR0kv2BowTcwuqcUIJOd2LaNzNyQ (Untitled)
  Session 9: We-cGv7b94MoxJd3A7vLwsVvnPb2qnW2A2SPcwXauV2x_6VwPpPz28nGCnhz1NGsCtHfNXyOfLwtFENoEJ2KHg (Untitled)
  Session 10: bZ-7B3w-TbSQt3kqjj27W2ZHe4J10z8GsnkV781WYxzhZK0OIgmiWjzyKuoXWNnvtWFIDo8zcO4S6xPvVHkDRA (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: 4WYDp0TVacNY2KT8sELKiYWkfyKThAvdCJiWnlHmR1uGcFtE5syBJMA7CSHW_2uFBbXTSocRQL7BUkavp1qiQA

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Map Creation ===
✓ Created record with native map: map[id:RLOpospXgPfVErJzQEuw6ufC1HoeOpleVvvjNMgJpSAIHbsR-CRYg8EXC78yGM_JxZF5U7KFuJvjNOroxoLNjQ]

=== Upsert Operation ===
✓ First upsert (update): map[active:map[type:Boolean value:true] age:map[type:Integer value:29] email:map[type:String value:alice.j@newdomain.com] id:RLOpospXgPfVErJzQEuw6ufC1HoeOpleVvvjNMgJpSAIHbsR-CRYg8EXC78yGM_JxZF5U7KFuJvjNOroxoLNjQ name:map[type:String value:Alice Johnson]]
✓ Second upsert (insert): map[id:RFM71nfYowPvWcRdXil93PZL5RSPSsG09goCQ4Be4WK-wfBlwBBqPQn9EDdsV4QtEeIJxyZZzE64KUUKLzUq2g]

=== Find One Operation ===
✓ Found user by email: map[active:map[type:Boolean value:true] age:map[type:Integer value:29] email:map[type:String value:alice.j@newdomain.com] id:RLOpospXgPfVErJzQEuw6ufC1HoeOpleVvvjNMgJpSAIHbsR-CRYg8EXC78yGM_JxZF5U7KFuJvjNOroxoLNjQ name:map[type:String value:Alice Johnson]]
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
Inserted with ripple: map[id:h8nEPQydvlLQvVw434qw3VQi_UrrHq2BMCSoP918rHTJR7IUi6gcpysQlDVHShcG34v-ir_qlTi-EqA9OAaI4w]
Inserted with bypass_ripple: map[id:RzEnKd5OvqKH5yt6k7q3yLMsUkpA2oLXK14rPIBjUgXpUhQw9wJxWUFAahUCOAo4EJJmlO5Cz0omuUdHaYa_-A]
Inserted with TTL and bypass_ripple: map[id:f-lao7y4NRWCZf0KP5pBTueeAE-txg4iHdmN9cjzkRtFugxIm_iAt4_TVrAPebyJbZNPEjV_Pn4VXpIwhh6ptQ]
Updated with bypass_ripple: map[id:h8nEPQydvlLQvVw434qw3VQi_UrrHq2BMCSoP918rHTJR7IUi6gcpysQlDVHShcG34v-ir_qlTi-EqA9OAaI4w name:map[type:String value:Product 1] price:map[type:Integer value:150]]
Deleted with bypass_ripple
Batch inserted with bypass_ripple: 2 records
Upserted with bypass_ripple: map[id:d4gvKKSfT-0kcx85BVLsdkIJQCJlLtyrZe6_Bdrp1P_gjOB5CkDHXkMmZWwef2ETjW_a0LgmuMTITZCyE-q6JQ]
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
