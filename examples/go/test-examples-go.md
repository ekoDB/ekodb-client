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
✓ Authentication successful

=== Insert Document ===
Inserted: map[id:hIn_DzOmZ2Zakn3E8o_i2f-azmPWJGunBy9U-UqT42gt76-mP9xQJv2snT3farkwqiUYmj1fT9ZtTnLRCh48pg]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] id:hIn_DzOmZ2Zakn3E8o_i2f-azmPWJGunBy9U-UqT42gt76-mP9xQJv2snT3farkwqiUYmj1fT9ZtTnLRCh48pg name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query ===
Found 1 documents

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] id:hIn_DzOmZ2Zakn3E8o_i2f-azmPWJGunBy9U-UqT42gt76-mP9xQJv2snT3farkwqiUYmj1fT9ZtTnLRCh48pg name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
[32m✓ simple_crud.go completed successfully[0m
[34m
=== Running simple_websocket.go ===[0m
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: MC6awzEkQm2HxIFz8YHls2Ws7hmMZSn51NM_fz8xZSMVu3ZXvP8tNXq4OgMXARzOhP5i5qmyxItK6259PZlT4Q

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
        "id": "hVJrJrqbHjCEisTnEM8A-FztQmjbfEL92UsvIUHFCyH4j45VgMzWEtQyv_b1grPXz9_tC-9hntazuuCZdw86tg",
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
        "id": "pgX3tN0CrrfdwZdfEP7IfhBBi2_OMTlOZKaFUVkClPD4NDbdz0ze2s_DBBIMQ5LTxd-lIMtq_vWFdt_XE62GnQ",
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
        "id": "MC6awzEkQm2HxIFz8YHls2Ws7hmMZSn51NM_fz8xZSMVu3ZXvP8tNXq4OgMXARzOhP5i5qmyxItK6259PZlT4Q",
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
=== Running batch_operations.go ===[0m
✓ Authentication successful

=== Batch Insert ===
Batch insert response: map[failed:[] successful:[RMaOnh2NCqA5RIjpx_HYQHmwB4jPM0QDI7n003GXErIK-OHIh6Rv30A09GbZpO5wvaPCKzCacvip-wn8l3hOAw rExra22oLriRSCbK3Y5Ynbww1NAPCGbwBmnCJP9BU2Nmsr21UOhcA-dR9KrM4nFHw1fDDU8thNVdO0dTHK6QFA Wi-scVLNefOr8CZw2sc_HDQvpdFVeBSWLJwDbTPB-Br7li00NZ55hHOs_L_BGb5waqQjcBqoaFRxd1gjadaiAQ iPnc8Xs_JZHfUy4KG_X5tQTjacSQiyIMLSISO1mdUyl4AQXvknVSJDJ4wYpOUe5Ya3SqgN8nOvyM50vLNyEPFw rP_UDf9oRN_Q6jlrI0r0001E5MSJMxQCH-xQjou7ofrHi4fBdqGwdf8Z8WKlVurXYv1Mf2vIIY8PI2EScrUpqQ]]
✓ Batch insert completed

=== Creating test records for update/delete ===
Created 3 test records

=== Batch Update ===
Batch update response: map[failed:[] successful:[w3D1mJN8YOxfb1I-VXK7Ightymio1frNegPQsp61UhwL8PuGf2dDYVLKySFIWj3WxIaDsZNPtpQSeiS_8g9WFA MPybfMi1k3VoHYESexay6YKFwNOijIGKHfF2chYTovsIWTYnea1hjUa0nEFHMEvfsLjf8aQ0Yl8pZ8woyD9hsg R_zjmKCZrnHaZHnW1c_iI9CcHKf_xZgsRqBQ3BDZRbGfyGReAlhNMvcbYOualHSoOid3YSGppS-p7kDCIiN4xQ]]
✓ Batch update completed

=== Batch Delete ===
Batch delete response: map[failed:[] successful:[w3D1mJN8YOxfb1I-VXK7Ightymio1frNegPQsp61UhwL8PuGf2dDYVLKySFIWj3WxIaDsZNPtpQSeiS_8g9WFA MPybfMi1k3VoHYESexay6YKFwNOijIGKHfF2chYTovsIWTYnea1hjUa0nEFHMEvfsLjf8aQ0Yl8pZ8woyD9hsg R_zjmKCZrnHaZHnW1c_iI9CcHKf_xZgsRqBQ3BDZRbGfyGReAlhNMvcbYOualHSoOid3YSGppS-p7kDCIiN4xQ]]
✓ Batch delete completed

✓ All batch operations completed successfully
[32m✓ batch_operations.go completed successfully[0m
[34m
=== Running kv_operations.go ===[0m
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
✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: BsfVPISm1QzJTRvtrNij_ht8AAF2Sm2UkPSu8hGA8c5e0e3KBy0vu_0plVxoxt12WrHd9Fm8_qeRjDYLv-ZuEg

=== List Collections ===
Total collections: 15
Sample collections: [ws_ttl_test schema_employees_client_py test_accounts scripts__ek0_testing test_collection users schema_products_client_py demo_collection websocket_test schema_users_client_py chat_configurations__ek0_testing schema_documents_client_py batch_users ttl_cache chat_messages__ek0_testing]

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
[32m✓ collection_management.go completed successfully[0m
[34m
=== Running document_ttl.go ===[0m
✓ Authentication successful

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: hkP5NnZ-WfSVBEn3koVg5AqflHUqGwNYBItKd0KMi_fXHoXkKAsoN_CgDGlu97yntYE-grZGDZZlgD4MEXOc_w

=== Insert Document with TTL (5 minutes - integer) ===
✓ Inserted document: <nil>

=== Insert Document with TTL (30 minutes - duration string) ===
✓ Inserted document with duration string TTL: 2xn8f_Gr_7CuWfEXC-HPQOb7uO1aUXuCZdLaFx9Z558xYJ1a2og0MQ_IXmjYA9Nq5KRb8_Wo4FtyOmgh218h_Q

=== Query Documents ===
✓ Found 6 documents with TTL

=== Update Document ===
✓ Updated document

=== Delete Document ===
✓ Deleted document

✓ All document TTL operations completed successfully
[32m✓ document_ttl.go completed successfully[0m
[34m
=== Running websocket_ttl.go ===[0m
✓ Authentication successful

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: ws2akVxiv4pwUdHcBuBZy1JLjFj1Ip9bB4Hc5avOeB8FmAyinKt6A-4oW4OEE4JasQi9tNNLZLpTj4RzSSzX_w

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 3 record(s) via WebSocket

✓ WebSocket TTL example completed successfully
[32m✓ websocket_ttl.go completed successfully[0m
[34m
=== Running http_functions.go ===[0m
🚀 ekoDB Scripts Example (Go/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: GNcBEGoQdC0KH_j6MmgSRtqQgV7HzfWexkIrU_zrtA-mJXiSfx3Mqq3UiRqps7AkIqfS_YhVai-EZwhYNaEdCw
📊 Found 10 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: oJjzEHq_-aXKpeGNxGJkTBTQVxRwGGp3YPAdhLve_8tOEfGLU0RylKz2D-gyuYZdgY8yC_2GLIq0t3PBfYYIRQ
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: RN2U9BElh8hLVQitjAlwjz6Dm81PH5vY9CnzZkTdxpDdtdT-5WV0GjLmku3JaLCK1DhksvgoWzSa0I23gdWh9g
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"avg_score":60,"count":10,"max_score":100,"status":"active"}
   {"avg_score":50,"count":10,"max_score":90,"status":"inactive"}

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
=== Running transactions.go ===[0m
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: VvRLok4WkXnEtftpjVUAkXpHxJIGHfo7A8MgDxIX8sgRmGbGpWTElxIolVNJMJDzzH5VYIE_m61XbMRDYyXR-A
Created Bob: $500 - ID: W1ynuIOuJN4KyW4X2hEBgc3SsZUsK8nuy2NBQiGj0sxq1xvhvNHL5sCvfzjDteIKBANq38_ni2k8rCZ1fgOYHw

=== Example 1: Begin Transaction ===
Transaction ID: d70b6d04-7a88-462b-bcd3-7f1530d945ae

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
New transaction: ff9b22fd-48ea-4f0c-a2b4-e4b081d1b234
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
   ✅ Script saved: NeeDNLkJzwhh_BmimGEHTltWfcNCyRlgv2Yhp8ABCeZ99guBc_QLePZ2SdyfrdPbjh8qfZY_14ijNJZIzp4icQ

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: B9EM5-l4jiNoOxDPCwur-QulkF8iJGbqQydi1jNv-MoXomJVReu4S2ts5xuCu4IiMw6I85Btz6DsdV-gxnbtbA
   📋 Name: map[type:String value:Alice Smith]
   📋 Email: map[type:String value:alice@example.com]
   📋 Status: map[type:String value:pending]
   📋 Credits: map[type:Integer value:0]

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: QLFW1rWytedQifAdCVVniJpDGjlrLRG7SAk4v9O_xOj5C4n_G9bnOLf4TolKIsuYM967dnaW-BTWFs5Vi2Eqvg

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
   ✅ Script saved: N59xLX7StPhOzyjtEvlsUi8EKybCFSlL9qhErOASd_Un-3L2LfewNg2l_hl-HxkiPAJ8xyVUWinXI0Fkp8eR_w

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
   ✅ Script saved: 5T5iPAT4GqAplIsO6pcmCCw8GOq2Ra0kC733xwy5BzKfYLun-3aBHbdg3nSzr61cDmxgM-5BJdTHlDm5OZsGUg

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: NeeDNLkJzwhh_BmimGEH...
   ✅ Deleted script: QLFW1rWytedQifAdCVVn...
   ✅ Deleted script: N59xLX7StPhOzyjtEvls...
   ✅ Deleted script: 5T5iPAT4GqAplIsO6pcm...
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
Inserted: map[id:v3BTjctST3hoD7ReC_wrfFBGPphbNz3tyk9a4RwJTtdyDmuFyo8vlwgrDntrBL2uzVnkobkPt5NTsaM1BWgnJw]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-01-03T05:27:15Z] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:v3BTjctST3hoD7ReC_wrfFBGPphbNz3tyk9a4RwJTtdyDmuFyo8vlwgrDntrBL2uzVnkobkPt5NTsaM1BWgnJw metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Test Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:42]]

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 0
  active (Boolean): true
  price (Decimal): 99.990000
  created_at (DateTime): 2026-01-03 05:27:15 +0000 UTC
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1 tag2 tag3]
  metadata (Object): map[key:value nested:map[deep:true]]
  embedding (Vector): [0.1 0.2 0.3 0.4 0.5]
  categories (Set): [electronics computers]
  data (Bytes): 11 bytes
Plain record: map[active:true categories:[electronics computers] created_at:2026-01-03T05:27:15Z data:aGVsbG8gd29ybGQ= embedding:[0.1 0.2 0.3 0.4 0.5] id:v3BTjctST3hoD7ReC_wrfFBGPphbNz3tyk9a4RwJTtdyDmuFyo8vlwgrDntrBL2uzVnkobkPt5NTsaM1BWgnJw metadata:map[key:value nested:map[deep:true]] name:Test Record price:99.99 tags:[tag1 tag2 tag3] user_id:550e8400-e29b-41d4-a716-446655440000 value:42]

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-01-03T05:27:15Z] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:v3BTjctST3hoD7ReC_wrfFBGPphbNz3tyk9a4RwJTtdyDmuFyo8vlwgrDntrBL2uzVnkobkPt5NTsaM1BWgnJw metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Updated Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:100]]

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
Collection created with first record: knQchNNXuB1IJWVmfG6kVqfE88v6x8wO92ytsr91-9R4a6wsKl2-XDMfTQ9U5KhEPgymhHfk4kCPuD8064PvLQ

=== List Collections ===
Total collections: 14
Sample collections: [ws_ttl_test schema_employees_client_py test_accounts scripts__ek0_testing test_collection]

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: Y3-LqToVGOoe1BiXYRh3obgMaucO8_mKdX8ZLXY1dJuKa3MqjQZZOzBQ32ZaCY8wkkDJBluNyigdlco9Mt7jOA

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: MLVjW6oke4poOPaHsnH4Yf-b_DIHZjaZ4XR8LNAzQVqAiuDYyjHuV1oCjKSofdUgv1qfBPXdOY2uQdaRSRq5vw

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
✓ Inserted test record: DKepBHs6Vm04ln1sKI6DGWvDjPp6783fuld3w36BiNttZYfuFkc-mLtzKXb8WIgUSpGlf1hOIygmDLvqidW5ow

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
✓ Inserted document with TTL: NL4LoXjvVvdBnFps6fxcBMQg16xeV_k_92KmXFw3BuVIhd89nUSGJGG_chsYwJyyO1wCVT40k9MXUWl87kx2CA

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
✓ Created session: SBvNH3R38sKAL9Bc6P62j4zXLJyCdfX2OuFUr3FbjkmewR9MVcxDSE7U2FSqLYy7ramHAlvT9fpWecCMl0PwJQ

=== Sending Chat Message ===
Message ID: jhpk8ms_UG0Ov19UqMei8aO7I-tY6XmzJwhSI9E69dzrjmxQvRt9t75Q2nmBTjD_DS4D3PxjFSEgX9DNsgIUSQ

=== AI Response ===
The available products and their respective prices are as follows:

1. ekoDB Cloud: This is a fully managed cloud database service product. The price for this product is $499.

2. ekoDB: This is a high-performance database product with AI capabilities. The price for this product is $99.

3. ekoDB Pro: This is an enterprise edition product with advanced features. The price for this product is $299.

=== Context Used (3 snippets) ===
  Snippet 1: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Fully managed cloud database service product id:aATLxp8BDAfl52VGbhiLFi1HR7cxUGTD5l9rxRlM4ffWWmjTKyFbAxj-eVV4s_t2vd-hVEzG3nDPRRGHyFMo7w name:ekoDB Cloud price:499] score:0.1111111111111111]
  Snippet 2: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:A high-performance database product with AI capabilities id:5Gf2JllYD3F_tVy8gZ3yUs6TUkxbPZAaNtnh8yIrtq6ZWk4SF2eIPGpVfhmLKtvx2D2G050rIMJy-W8Frpy94w name:ekoDB price:99] score:0.1111111111111111]
  Snippet 3: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Enterprise edition product with advanced features id:bUyxvrIX6qFDtmwOAmG69zm-ze5YwmBsA7RsGb_D7haOMt_GPABXKuLJKaXsEqOeMRT503KgWYOkJYXhOWFhDQ name:ekoDB Pro price:299] score:0.1111111111111111]

Execution Time: 2620ms

=== Token Usage ===
Prompt tokens: 604
Completion tokens: 89
Total tokens: 693

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: eLr7gqxll0XkMo0fngGviRakNgisqU8DCS85DaBsZs8CS6vga5l2NFW3o8tiPi6Hqdv_I52Fb4aGQ7L50MET_A

=== Sending Messages ===
✓ Message 1 sent
  Response: Answer: One of the products available is "ekoDB". It is a high-performance database product and it costs $99.

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 0ZFpLmRckrnONUJw0gtusDvjrBosVj3-gOdkpdvKBmpRERqYyFS4PCpiwhXnLCj0tIMOyyWQNFIlRWNrjO4NBg
  Parent: eLr7gqxll0XkMo0fngGviRakNgisqU8DCS85DaBsZs8CS6vga5l2NFW3o8tiPi6Hqdv_I52Fb4aGQ7L50MET_A

=== Listing Sessions ===
✓ Found 9 sessions
  Session 1: 0ZFpLmRckrnONUJw0gtusDvjrBosVj3-gOdkpdvKBmpRERqYyFS4PCpiwhXnLCj0tIMOyyWQNFIlRWNrjO4NBg (Untitled)
  Session 2: eLr7gqxll0XkMo0fngGviRakNgisqU8DCS85DaBsZs8CS6vga5l2NFW3o8tiPi6Hqdv_I52Fb4aGQ7L50MET_A (Untitled)
  Session 3: SBvNH3R38sKAL9Bc6P62j4zXLJyCdfX2OuFUr3FbjkmewR9MVcxDSE7U2FSqLYy7ramHAlvT9fpWecCMl0PwJQ (Untitled)
  Session 4: GcI_-JS9SW66hdDUtOx8VFC7p1LwmVDKuARk01Ave2Iv5cQ4tA5LKv4tnVsbWwxppCW-Vc4RbAEteKlXSvJ3aQ (Untitled)
  Session 5: qZMycTvr9bUB7YzLX4ZcWbXjNc0DhwM1cTVcywNKzMxIiU7HUYO1RD5mktQvJD1w7qRXgiu87_BhTzLj1_HUKQ (Untitled)
  Session 6: zzMnYpnbOdEiHo60NW7v27gF22aW9C9vI17DJPbTOj_dVJ8W5PrS1iXVv3DLxXuW_gp_SHv8N7h3_9yZz7UGpg (Untitled)
  Session 7: ufrTyD4PrTBEQptwzw4r9KilyotTBDWqwmk4QImGnlZfBf6Qx-Xc8uVTAkyswCKMpDmpE7KDxy8Iez2OfoOILQ (Untitled)
  Session 8: PNbBYjnZytJfK4lVrRLNRsiBxwy7a78bqkqjn6t5ALLHU37liYPxCrve7K80XGdiFzBiAyyIZnLhiCk0RpcR3Q (Untitled)
  Session 9: Qq23Wydaedyu53YVx8GKmxuBLGgTrRMMJJrULpbZ9S1uB9m90eEgQQja0jX0_QNEgwLr3vzJYz91qxxD6Dh-tQ (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: 0ZFpLmRckrnONUJw0gtusDvjrBosVj3-gOdkpdvKBmpRERqYyFS4PCpiwhXnLCj0tIMOyyWQNFIlRWNrjO4NBg

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: K2RRbgESugzlMeHPJZqA2yHhb4Ju9ijLrzGbONA4ccJYma5uLt4P6aqrbYk2pXWoLb3FiUasvNI2EIUNZkMvUg

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information available, we have the "ekoDB". It is a high-performance database product. The price for this product is $99.

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
✓ Created second session: ElVzQrLNy8abDUqTzInc62FK1DQNQ5oWxkOaO6_uS9oZlfd78IvPRsDC01kg-5fKbgtzqgyw2gruwFjn6PhEBg
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
  1. Score: 0.759
  2. Score: 0.746
  3. Score: 0.730

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.698
  2. Score: 1.504
  3. Score: 0.292

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
🚀 ekoDB Scripts Example (Go Client)

✅ Client initialized

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: S2nP6NKLht6bYZ3oTfElIXNkAQCDLU2KSFj-9IPRsRSRCdsJvX2VUnCeHJdRVcX4go8ysrYC-5sqwwYAUGJ_zw
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
Created Alice: $1000 - ID: 4HsYS5rbCLK1Olexa_6yejzwGtM9Rg7vif5AVkOi9qmMCpaB_WyVz9TjnDR_P60c2MB4riysgag_B2809wslGA
Created Bob: $500 - ID: J23MG0KtGp2GkveWUDd9g4S2ftILt5X--GRDwAhnBwmdCwF0PRb_UH9u2xofYepJdaozkqFedYT1_IGEkmfwJA

=== Example 1: Begin Transaction ===
Transaction ID: 87a36560-ee60-4f6f-bd3a-1156a6e9f556

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 2c8e7adc-4c3f-48fd-9df6-c84e9f651b25
Status before rollback: Active
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
✅ [32mGo client examples complete![0m
