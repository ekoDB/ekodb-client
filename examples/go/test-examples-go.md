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
Inserted: map[id:aCuFTML4DJklvoCsrnDBl-ZjxoTJUCQXB2rtFAH2QrEmnb0gWNu1Pqn7MOwvzhTpl2xqQ9cY89mKC-XuKdcfhw]

=== Find by ID ===
Found: map[active:true id:aCuFTML4DJklvoCsrnDBl-ZjxoTJUCQXB2rtFAH2QrEmnb0gWNu1Pqn7MOwvzhTpl2xqQ9cY89mKC-XuKdcfhw name:Test Record value:42]

=== Find with Query ===
Found documents: map[]

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] id:aCuFTML4DJklvoCsrnDBl-ZjxoTJUCQXB2rtFAH2QrEmnb0gWNu1Pqn7MOwvzhTpl2xqQ9cY89mKC-XuKdcfhw name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
[32m✓ simple_crud.go completed successfully[0m
[34m
=== Running simple_websocket.go ===[0m
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: 9EeVbFUN6VQJg44lIJj9v3V6ea92aTPNLFIVKep_EE20y-vFENwg8lMAnyr-UA0MPFZS97bncRRXhrpvyMEw2Q

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
        "id": "2ohhdIKZs8t7QdU7ZWUXCc4EPm8T-Iy-RVYfPio_EofPFq65P9o3SaJhKtwl8h8xuajQE9DK3IsJAV0tEM2-dw",
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
        "id": "V-GwO5brmP15gkZCWnGtp4o8ngZYVpyshElKjClatQ0lIDPngkQSMEFFFJNE6mPPBwj0_pYxOs-mnpoye8qKTw",
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
        "id": "9EeVbFUN6VQJg44lIJj9v3V6ea92aTPNLFIVKep_EE20y-vFENwg8lMAnyr-UA0MPFZS97bncRRXhrpvyMEw2Q",
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

✅ Script saved: BJj74Sjec-JWwmhiHy_8zfnVoNHFFyPsrGuS7atHIgZUkMhHxCLx8FdnbjcyMD3uGp4sAw-VrWkP_XF_yUgWbg
📊 Found 10 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: r2TCqM7fJlOJmLba08ZwhNarLnl44tDa-r_eo8q3WUh04K4GL0Hywqm4UCdGsP1CBlI9Puwdg3cQ-B2cZmo4fw
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: 8y8iro2WEBDslI4GlOoXFmT3qtjlMSRsIB2CGRJ3BWpjh_OneHngnMmTd06-4uAT93XtgVbUCExDDbbwLr0o-A
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
Collection created with first record: Lswnum6kiBaY4xoJ1rRYt4HHTOkeBlu60zwYYWAS87a6jQj2-VbhyQ5NyDfdMWS-5VT8Owe6OWAgFLCxJpIgRg

=== List Collections ===
Total collections: 16
Sample collections: [users ttl_cache test_collection user_cache scripts__ek0_testing]

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
Created Alice: $1000 - ID: p6KHWFGQRRr0zjIfzsUTacR8MGBlShsYQn76pSgQtvNvlf_yYgTMdDSTBEtjgbsgYIcyNh9ErJXTFc-NQ1dyow
Created Bob: $500 - ID: Yv6JfeH6rYy67j2SP9IgONf5gcQf9bDrd1PIjvMe77kkFKmeZSGLKgNbzBQUPmcSCZl6sKmasbSblWKp40YqTg

=== Example 1: Begin Transaction ===
Transaction ID: 2a8a32b5-aaae-4372-ba60-2590cdd384b1

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
New transaction: 758b1e6c-ca39-4264-ae5a-7847d5034b07
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
   ✅ Script saved: rtvoM6o3MaZMOujvWakODzW2Qm-oduKF-9yN2bVxnk50rHG24hkXzlQYb2xstHFK93F_AnI6YbgOLLq1NMIzoA

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: ufqIfy6oqesPzrO69VaVBB-7w9Q9yCC6Tkg1RxwaX9GAtVgy4TYg9xiY1RQT7F_EYkbDXXXBSt0hNcE6KdUVpA
   📋 Name: map[type:String value:Alice Smith]
   📋 Email: map[type:String value:alice@example.com]
   📋 Status: map[type:String value:pending]
   📋 Credits: map[type:Integer value:0]

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: KaTtdUBZ4UhorxOPcTSw1uJjR558nCKFqmN6Ci1PbkKBksJ567-OGnqnG2jTlRI_ItN3Vxdtuql9iPByo5vEdA

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
   ✅ Script saved: 4pmxLRWHCCSRtoxzGKYZ-ow1XAFNwJBtyswCWC7V9DXc6FFVK55CqoZS5AHIU7nrROGIVPEIDQg7xky_Ls8QOA

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
   ✅ Script saved: d2QF86XmRti8bBf5JxgKUhBWTwYlOBrHT_rxHxjOhnbUwQSiG5hnzfmC-o2IlTaCXKAmYg8glHfWJQjfViRt8g

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: rtvoM6o3MaZMOujvWakO...
   ✅ Deleted script: KaTtdUBZ4UhorxOPcTSw...
   ✅ Deleted script: 4pmxLRWHCCSRtoxzGKYZ...
   ✅ Deleted script: d2QF86XmRti8bBf5JxgK...
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

2026/01/26 03:01:22 No .env file found, using defaults
✓ Client connected

═══════════════════════════════════════════════════════════
TEST 1: Document TTL Expiration
═══════════════════════════════════════════════════════════

[Step 1] Insert document with 3 second TTL
  Input: {name: 'TTL Test', value: 'should expire'}
  TTL: 3s
  Output: Document ID = jKs1bFohFiI9rEvUo2GoXoV3OlgsiUKHVX8zGB8cTWL14t-LGYg3UQC_bRGtDLzXJQqfUE2afzpThfPmgXpaFA
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: FindByID(jKs1bFohFiI9rEvUo2GoXoV3OlgsiUKHVX8zGB8cTWL14t-LGYg3UQC_bRGtDLzXJQqfUE2afzpThfPmgXpaFA)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: FindByID(jKs1bFohFiI9rEvUo2GoXoV3OlgsiUKHVX8zGB8cTWL14t-LGYg3UQC_bRGtDLzXJQqfUE2afzpThfPmgXpaFA)
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

2026/01/26 03:01:29 No .env file found, using defaults
✓ Client connected

═══════════════════════════════════════════════════════════
TEST: WebSocket TTL Expiration
═══════════════════════════════════════════════════════════

[Step 1] Insert document with 3 second TTL
  Input: {name: 'WS TTL Test', value: 'should expire'}
  TTL: 3s
  Output: Document ID = yldhRYv8WXQTWRX6XdcJTl4SNakOVV5JNQrI_75EebGyda7u6UyFuiam3QMoxpZMC4nziHmgpcYcy30GhfHlZQ
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: FindByID(yldhRYv8WXQTWRX6XdcJTl4SNakOVV5JNQrI_75EebGyda7u6UyFuiam3QMoxpZMC4nziHmgpcYcy30GhfHlZQ)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: FindByID(yldhRYv8WXQTWRX6XdcJTl4SNakOVV5JNQrI_75EebGyda7u6UyFuiam3QMoxpZMC4nziHmgpcYcy30GhfHlZQ)
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
Inserted: map[id:o5PqPwidS7c7MMFR0np-JGjXYMhymD3mu3FSLyrYg0WsQJpDV5lQFmFgFxw8uLvPTwEV0HQKTfwtCrfdbNHUHg]

=== Find by ID ===
Found: map[active:true categories:[electronics computers] created_at:2026-01-26T03:01:36-05:00 data:aGVsbG8gd29ybGQ= embedding:[0.1 0.2 0.3 0.4 0.5] id:o5PqPwidS7c7MMFR0np-JGjXYMhymD3mu3FSLyrYg0WsQJpDV5lQFmFgFxw8uLvPTwEV0HQKTfwtCrfdbNHUHg metadata:map[key:value nested:map[deep:true]] name:Test Record price:99.99 tags:[tag1 tag2 tag3] user_id:550e8400-e29b-41d4-a716-446655440000 value:42]

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 0
  active (Boolean): true
  price (Decimal): 99.990000
  created_at (DateTime): 2026-01-26 03:01:36 -0500 EST
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1 tag2 tag3]
  metadata (Object): map[key:value nested:map[deep:true]]
  embedding (Vector): [0.1 0.2 0.3 0.4 0.5]
  categories (Set): [electronics computers]
  data (Bytes): 11 bytes
Plain record: map[active:true categories:[electronics computers] created_at:2026-01-26T03:01:36-05:00 data:aGVsbG8gd29ybGQ= embedding:[0.1 0.2 0.3 0.4 0.5] id:o5PqPwidS7c7MMFR0np-JGjXYMhymD3mu3FSLyrYg0WsQJpDV5lQFmFgFxw8uLvPTwEV0HQKTfwtCrfdbNHUHg metadata:map[key:value nested:map[deep:true]] name:Test Record price:99.99 tags:[tag1 tag2 tag3] user_id:550e8400-e29b-41d4-a716-446655440000 value:42]

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-01-26T08:01:36Z] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:o5PqPwidS7c7MMFR0np-JGjXYMhymD3mu3FSLyrYg0WsQJpDV5lQFmFgFxw8uLvPTwEV0HQKTfwtCrfdbNHUHg metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Updated Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: dnwhRGVd_NaNcPqeW8Ws3_Bw1CSBWoY7z3eM_SpG2sY0tM0OBXekjiFPNvh0_BuSWPFiHFdssFybjFHKI1Ur8Q

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
Collection created with first record: dMjXVhdfp2QkeQg8HkhS7Z3YL3PK-H0lDqcEY5sVO2EpmISdMEHDkxGyWn-wnLHnDXI1ev7dradu_wZKeJ90dg

=== List Collections ===
Total collections: 15
Sample collections: [ttl_cache test_collection user_cache scripts__ek0_testing schema_users_client_py]

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
Created Alice: $1000 - ID: tLdSRk2tzQDHLJXjJWcM1VO454mvV8B0QmKifXOlERChdEhS3dZd8saWDSot6cF880TsyuP7AaBuqKyvQAL62Q
Created Bob: $500 - ID: 3cRjCBDbYBdxeH_Qzq1hdflfjY1Eb51VQTstyg21ShBDxflbr2thjWE5VrJIklapXy6CZUSDHhPHPd5UDzEy-A

=== Example 1: Begin Transaction ===
Transaction ID: f0eaddde-abf1-43de-a09a-17cb49c0786e

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: a6fb394f-3710-44ff-99cc-6ce975fed120
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
  1. Score: 0.764
  2. Score: 0.748
  3. Score: 0.734

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.699
  2. Score: 1.506
  3. Score: 0.293

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
✓ Inserted document: f4xQf5-xO59HATnwqKKpH1rYa8CDAu1QvOrXxiGu6T_lc9aScMU3GuVWqAC9f09v1XmI_5clm1qt2UutKYHejQ

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: IWe9HBnwY2XMxE2UmA7GzI4NJ-aSeiZ_5R7I7EmugOG9FTCbEcaU4kZeLlEbV3pNRiOMjOJuJiSgyNrf4hYjmw

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
✓ Inserted document with TTL: W45qOG1fqytIDKmJ9vFURM7iMUWgZhc3Fi6Vdn0VCJCGMIswPsMbNLN7KYkVY8K6ifvLJqhNL-Ri4ybKK2wS6Q

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
✓ Edge cache script created: gPZ2MiCiDn6wyuDu5_2bA4m_BND6LLd5Gukw9T8WqmBYNexIP426PQtk97bgxGQRMXEzLvDmux6Fa7FGrDLtaQ

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

✅ Script saved: ReU1o1l5VaddkPoi5Wu_erfuqMB5Hz9INrjUiPJLXQxr353HiEQ_kNODGtyu_1a300uZrUtMPENJvUBbwJgP4Q
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

✅ Saved reusable function: fetch_and_store_user
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 66.125292ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 68.192708ms
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

✅ Script saved: wQ-AfeuooNQxVjuMnmgwmNhHTbPpoxqGj_ZHkp41kg9afUBub9FjOPzfXyvTByM0-HmLznSCcQI19Dqa83C6rQ
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

✅ Inserted order: 4uR0I7iAtsgB9XWdIIF2QfSQLa7tHRPRdn20ZW73Kr0UHLuzMRF4NhrBcWHTigp-iGf9Ol1pUmvc3lmAXP-EgA
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: xWiW2gsbCyfEsYiKyQpnDb0yOvd8M5fGosN58BNCgg1FJ9TII87_Pzk0Ue2PYKqdlgtEM9cjohy2zaPOttvP-A
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: map[role:admin userId:user_abc]
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session
📝 Example 4: KV Operations in Scripts

✅ Script saved: _y79BKkPfQLldQBRVG3397J8bmkQ9aXNH18Rdr4bHEbkD21CxYv5TYjN6yf7RxZo7W9wAFNd1RI0GnhfrQVq-g
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Script

✅ Script saved: Jq_FEV1mFiQDDwISDegRwImuPnhvGEbGwE_1JXK3IZYVxsXHy9Gz3tRPm71sTVgZ6ArK6_EV9QD09G_4QbHKTw
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
✓ Created SWR script: fetch_api_user_go (gKE6vtvUsGTw_WTxQ72q7PHYTx15Lnjx2KDl9Ffesdnrzyhf0i2MaDDtLrjP5qRLfBZAVvTw3Qqhepp5zaj7zA)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "cached_at": "2026-01-26T03:01:39-05:00",
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
      "id": "OgwX53vIi1aMEr8wbzQnVW11vUYHEGb_7TQ8JSmwMV9CGJW51ObanIUoIFbFLI6eBHmx10Es1nA-RVHKoixYJw"
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
Response time: 67ms (served from cache)
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
   1. map[type:String value:Natural Language Processing] (map[type:String value:AI])
   2. map[type:String value:Getting Started with ekoDB] (map[type:String value:Database])
   3. map[type:String value:Database Design Principles] (map[type:String value:Database])
   4. map[type:String value:Introduction to Machine Learning] (map[type:String value:AI])
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
✓ Created session: cpHlk-ADD4R88a2saVoQSXnT-rPHLY-3TLG0Q-4D1KmO5jf_Yq15-g-qFDRzbar4P_XZYuWWMmUL3QOqQXwdeQ

=== Sending Chat Message ===
Message ID: IWCIRjwZ-ug1-h3nsSo0dvhLXbR0uLUH8fTOc3YjlqiRQFFe3W6mhkg2ky1BXzY77385N-8R7GYFI49gnkKwvw

=== AI Response ===
We have three products available:

1. ekoDB: This is a high-performance database product with AI capabilities. The price of ekoDB is $99.

2. ekoDB Pro: This is our enterprise edition product with advanced features. The price of ekoDB Pro is $299.

3. ekoDB Cloud: This is a fully managed cloud database service product. The price of ekoDB Cloud is $499.

=== Context Used (3 snippets) ===
  Snippet 1: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:A high-performance database product with AI capabilities id:mL1hDM5pQkl8JJCEwgdN6EaggUg_vpVYEodE9Xi1mbaxjc9C2Eg_Bf_XP5E-7b929IG_dw_G_nKhDNlY5v9muA name:ekoDB price:99] score:0.1111111111111111]
  Snippet 2: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Enterprise edition product with advanced features id:mIOx_g-ReeR8uZWvN8fU8AE2jNTdCKmUIHeTN0I-hxlH2IRxJu_Sx9ASbIQmvM6EsDmaoIWSFQP-AXx-QxRXtg name:ekoDB Pro price:299] score:0.1111111111111111]
  Snippet 3: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Fully managed cloud database service product id:IAC-3J8d4OwuowDA1VnVAQfuFlWbMq3gtMYBactLUKvwpHbDslDRJgIWqBzGqPVZBVma4Uyud-6N-9Y4ftnswg name:ekoDB Cloud price:499] score:0.1111111111111111]

Execution Time: 3123ms

=== Token Usage ===
Prompt tokens: 449
Completion tokens: 89
Total tokens: 538

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 6opKRrWhlX015gbHC8zT50GwgPqw9UR1RFJhw0CWOR0D85glL8ra5x2YYgy28M8JgBjCLO8gR5834EXLnwKagQ

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information available, we have the "ekoDB" available. It's a high-performance database product that costs $99.

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
✓ Created second session: L2n3N8nWclhIqgCg_HvoXpytdb71VhqTt_bw1_KjLcMzxTom1LQ9TYXYlaiJBahUKc_Q6LPU6nAMWPHPP4g4Zw
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
✓ Created session: XEMmWa39EXymaiyPjWK2XGee5O1GdzWDrVv-XIqEBYq4pg3FFS9TKL-u_VHtaFuYOaC-fjydSlX8mx-hHZg8uw

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information provided, the available product is "ekoDB". It is a high-performance database product. The price of this product is $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: jRazOc5hAjklSd2FlL-I7bHARy_SNZ7LIcKA8vJRDn2Qr8KQ6ZpxkamS4k5asXXP7naLHx-KvT_tsHEG86IItA
  Parent: XEMmWa39EXymaiyPjWK2XGee5O1GdzWDrVv-XIqEBYq4pg3FFS9TKL-u_VHtaFuYOaC-fjydSlX8mx-hHZg8uw

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: jRazOc5hAjklSd2FlL-I7bHARy_SNZ7LIcKA8vJRDn2Qr8KQ6ZpxkamS4k5asXXP7naLHx-KvT_tsHEG86IItA (Untitled)
  Session 2: XEMmWa39EXymaiyPjWK2XGee5O1GdzWDrVv-XIqEBYq4pg3FFS9TKL-u_VHtaFuYOaC-fjydSlX8mx-hHZg8uw (Untitled)
  Session 3: L2n3N8nWclhIqgCg_HvoXpytdb71VhqTt_bw1_KjLcMzxTom1LQ9TYXYlaiJBahUKc_Q6LPU6nAMWPHPP4g4Zw (Untitled)
  Session 4: cpHlk-ADD4R88a2saVoQSXnT-rPHLY-3TLG0Q-4D1KmO5jf_Yq15-g-qFDRzbar4P_XZYuWWMmUL3QOqQXwdeQ (Untitled)
  Session 5: YZTKwgcM0LUFIISSEphG7T2aRGMZCoX5bTTZbWuYOXK7-hZ_QKRkBxC3Qe3kv1VD2_eD7VMd_bywYTEFyyu6zw (Untitled)
  Session 6: GtfMq8eRFBL5hDUmOLtkGd4bjUEosRy4nkpiK5_-XmqIIiKyyeNwaHW-_s-pEBkz5CEYEcuVDBgdrN47637TsQ (Untitled)
  Session 7: r3WEzajhiOln7bNynUKzsDgIZFkUdrh2pPsREnAIE12Hs9SDxUUNYqdLerZ05GZScJZq9i4hw9rGdStaEqZt-g (Untitled)
  Session 8: txXTU787YDCmI_8Mu2xbUdXH1CuPDrTSreIwj10JqQxeQfx-c4ezBzvpIRsHlLBxNzJMQCuJkifsxvald9nfiw (Untitled)
  Session 9: FgQNl1wmCTOX8XWaikjWKnJh5jz1vLej2PnipxoS35AnUh8n2sqqRoULUiv_bYmC1vk9OP-5AowIoQaNZhkzmw (Untitled)
  Session 10: fHsXg70v83-LYbtuzLXIlj7wO9HrxMk8LNNXdJIlO0x3bzdjAUXMoATyoWAIe7npLkz5Docj-wiZlS0gODtFgw (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: jRazOc5hAjklSd2FlL-I7bHARy_SNZ7LIcKA8vJRDn2Qr8KQ6ZpxkamS4k5asXXP7naLHx-KvT_tsHEG86IItA

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Map Creation ===
✓ Created record with native map: map[id:Bv38Xf8KAaQDidMxnQm6vioGMl8opfINWvfptlMzW70rRU-uaRjUPlytp3FkNITe49-q-jFuEBUbJZP9ItD2aA]

=== Upsert Operation ===
✓ First upsert (update): map[active:map[type:Boolean value:true] age:map[type:Integer value:29] email:map[type:String value:alice.j@newdomain.com] id:Bv38Xf8KAaQDidMxnQm6vioGMl8opfINWvfptlMzW70rRU-uaRjUPlytp3FkNITe49-q-jFuEBUbJZP9ItD2aA name:map[type:String value:Alice Johnson]]
✓ Second upsert (insert): map[id:27ZIVECn1FtOymc68zsT1K1ARcQh1gK8Jv3Qx5nl8RmX-wN4-zWExHed_2ukTLMDr6nB8Q0BNDVCP09M-TVZnw]

=== Find One Operation ===
✓ Found user by email: map[active:map[type:Boolean value:true] age:map[type:Integer value:29] email:map[type:String value:alice.j@newdomain.com] id:Bv38Xf8KAaQDidMxnQm6vioGMl8opfINWvfptlMzW70rRU-uaRjUPlytp3FkNITe49-q-jFuEBUbJZP9ItD2aA name:map[type:String value:Alice Johnson]]
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
Inserted with ripple: map[id:YClLxG1PicW4gO-vrAsv-mcTyheYyDxubrwgFOvi5EV4NIOWYCV5cGcYx42jp7qqm_6azKbcchBdSwoGEOYyWg]
Inserted with bypass_ripple: map[id:JKl26vKqHQBGBZIWu_7d7s3yJSzNHe7ZB198szRj8VYqpFCsf2Hf3Km1n8MBR-cAX5lO1zNx7SHiDsSMdQl4zQ]
Inserted with TTL and bypass_ripple: map[id:62VSauD3Vi1cqfo2G8wDQfaFBlFEoJGEuaEIneKVMdUm4a-RE8cvwYXSTetYDXtqbBOBvSzDijjafODtN38bMQ]
Updated with bypass_ripple: map[id:YClLxG1PicW4gO-vrAsv-mcTyheYyDxubrwgFOvi5EV4NIOWYCV5cGcYx42jp7qqm_6azKbcchBdSwoGEOYyWg name:map[type:String value:Product 1] price:map[type:Integer value:150]]
Deleted with bypass_ripple
Batch inserted with bypass_ripple: 2 records
Upserted with bypass_ripple: map[id:uKugc0XK7bkKnMD6pPJgdg6WK-GVCd0zG9Gme4NyHAG5akLRqNWJSoIocj7_mYti1RKngO9z23USIr9WupKk5w]
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
