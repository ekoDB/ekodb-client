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
Inserted: map[id:rivp_cfjb9XdnFqiWhmzErX9cuzEvHzcK4Z7lEqj9ETr_XK2FML1aQOkLUiBDNhq-1fby9m5NocFjCxC2cDKcA]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] id:rivp_cfjb9XdnFqiWhmzErX9cuzEvHzcK4Z7lEqj9ETr_XK2FML1aQOkLUiBDNhq-1fby9m5NocFjCxC2cDKcA name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query ===
Found 1 documents

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] id:rivp_cfjb9XdnFqiWhmzErX9cuzEvHzcK4Z7lEqj9ETr_XK2FML1aQOkLUiBDNhq-1fby9m5NocFjCxC2cDKcA name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
[32m✓ simple_crud.go completed successfully[0m
[34m
=== Running simple_websocket.go ===[0m
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: FTsekewxgB5micEApFUENESf1T4V1PoJc7ytPFR4kyIC6AB3mCv18L8t7JYVDT6tkCGtqwz8jXgnOOsmcpkPkw

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
        "id": "FTsekewxgB5micEApFUENESf1T4V1PoJc7ytPFR4kyIC6AB3mCv18L8t7JYVDT6tkCGtqwz8jXgnOOsmcpkPkw",
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
        "id": "v_o_VMTYzOkploU1BbN3TCNs9Kbc8H7BASVbe-rp30wX4gyOWAjC8jUwlFCsg2pqbeSVbXBiD-nK4oMNdpJJEg",
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
        "id": "um21JmrqMHTOaWiaJQsXKEO81RaRGKGHACyQBVWU8yNrbkOgM0K0tZKnikMWm0v1nnEdc0jxGdcOI5tZuN4HRA",
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
Batch insert response: map[failed:[] successful:[_LBxl4142w9mkofhH5lo6AGbVb181xv2nYRMPCUW0nO-00aD2OROMXZtdqodCWrwKQSOjXqEGIUUnk2Dg4DEug 9XGEAXrAlz-O7rT0r_dPN5J_wFctqZNoboVcPoF4gog6SNkL_CQZr2y8ny12kri1byJkKEMSB3TgW_WrlZIz0Q p4uDRUuHALuX0_Im-b-ISzDdUFqrUdPyipLCxcgvSUt3__UYc16Ai83JZ7eko56jxeyVI9RgXeP2DAz6E1V40A ZDOsULdKSkZWVD4KMaRPlMvO3W-drXmWnuRTFJbSYoglZxeGlGVFFU5MV6S7iSNms-610luKTMaFNgDWGcJHLQ elbhM3sbtA-pqfkJ6fmgZEF5ATJUduAY47fcD1V_MojZcbNp2hJIzDlK0fbEjYakF-vQlrtcmshrFd_E6jaa7g]]
✓ Batch insert completed

=== Creating test records for update/delete ===
Created 3 test records

=== Batch Update ===
Batch update response: map[failed:[] successful:[O4AMWe6VAYE3camhpyenX8pHsZaq1y2nfoaNhh_HQ3Im6GYWcJz_q4mToq2rzMhwn4SJmSbByF8o73XsLSIeGg SwVa4BmarvjXX8gzwE8hv10OdLQTr5VrNyjQo-tNXVamD4QBnEA3zvBLxt4r8bfHoCZ8BvelCNpjqNhZ6BKz1w 8vo0NhPbRqvVjEkbyeyJarLcRU0VP3vS1GwxmnkPJgWdicKHgVvDDLGtf9zbB_jh6Iln9I3nnB9EP5Gb6PK_KA]]
✓ Batch update completed

=== Batch Delete ===
Batch delete response: map[failed:[] successful:[O4AMWe6VAYE3camhpyenX8pHsZaq1y2nfoaNhh_HQ3Im6GYWcJz_q4mToq2rzMhwn4SJmSbByF8o73XsLSIeGg SwVa4BmarvjXX8gzwE8hv10OdLQTr5VrNyjQo-tNXVamD4QBnEA3zvBLxt4r8bfHoCZ8BvelCNpjqNhZ6BKz1w 8vo0NhPbRqvVjEkbyeyJarLcRU0VP3vS1GwxmnkPJgWdicKHgVvDDLGtf9zbB_jh6Iln9I3nnB9EP5Gb6PK_KA]]
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
Collection created with first record: 3PGG0gfFDbHTu2d812Asn_7Rg7sVl4lg1t3W2WkOGjzPVzmDwEvpzZt5nCjWtdDKku4wjnlyExzhLxcn0PHVvQ

=== List Collections ===
Total collections: 15
Sample collections: [chat_configurations__ek0_testing ttl_cache schema_documents_client_py scripts__ek0_testing demo_collection test_collection schema_employees_client_py batch_users websocket_test schema_users_client_py ws_ttl_test users schema_products_client_py chat_messages__ek0_testing test_accounts]

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
✓ Inserted document: c-1Sl2P2mgxY9hZRdfI3OVmrOKM68cxU6hkCGzDGfhLCxQrYiw684eoE_j3vKatWsaSTAhloTw2kbDknEA4xmQ

=== Insert Document with TTL (5 minutes - integer) ===
✓ Inserted document: <nil>

=== Insert Document with TTL (30 minutes - duration string) ===
✓ Inserted document with duration string TTL: p06aBl87fjpRMr5kpvCSmh9V0aGcifikRrSOXMf4t2wHx9-Iqc6CI40wLhh2FpzDjNNpDU7nmNX9nIfxipDF_Q

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
✓ Inserted document with TTL: zcSJH14KnWyS_gKb4ZSnzAHQf_2BD7hv3x3XL-Wnu36W3sR7GfvjSv4bnoZgjoL9sGzaw9H-1Xo4TJDcsRG2ZQ

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

✅ Script saved: XIIXBUK_0SlbzLxX76dvsfM-kgqbpw-ItOyTjxYyEnnPK_fzExcG0N5MnF3RmjzDkQ2B2vbDfI3MszZQfSxMQA
📊 Found 20 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: soNJqPCr1yqm-ugzq0mQuAp75ibpydU5XYUygfHktk_YPRD0YKePt-UtR6FLF2h5kfocjmVl9ZaNzMw081NwqA
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: VdG0jPwxmakttVoQETr90NMRMHRdn0cYHHfmUFhaVvTar35RenSmvuDFplXVisdJcDwDaRnTqZtneyOLPfeJ-Q
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
Created Alice: $1000 - ID: LgHgiDUd1-r9T92h4K5TsUu33JwJkJ56Zntge7A8JKFddvx3qbcpuQU8HJxQ5YzspDiP2awID6cu-SZ8Ul04TA
Created Bob: $500 - ID: xLZ0pwVoFhVa1hQkxpM1Lw55S0NmqG93Ih-X4u8MudwPB6JbfM4uE4gT4Y6wtQ4RMgaqgZn5V9VmrVLjhPZzbw

=== Example 1: Begin Transaction ===
Transaction ID: 9ad7ba83-14f3-4463-834d-3a9d422310a9

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
New transaction: f11dcf2f-45bb-4202-aaf6-9c17ebfbe56e
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
   ✅ Script saved: 7GYcMVHox8X2VlmfwKStu9N465WuTvL0pWRhTPrW4jh3R8sXwISqj8KATFEz-aF_f4aWdnmZWfw5KGx7QE09_Q

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: WopDK0CyvngpqvRn0stF4KSX6CsBVRhwDZ4kWpMAN7jkYn28j9bLo45aCi-MzvNBQSzKAe1M-7WeOGpcoLD49g
   📋 Name: map[type:String value:Alice Smith]
   📋 Email: map[type:String value:alice@example.com]
   📋 Status: map[type:String value:pending]
   📋 Credits: map[type:Integer value:0]

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: vJgzEQmKxafgelpkVWAW80oIVMMERTc-P-FROqsxthAZzaK31m4wm2CF4G-RPPd1VO9YLRehxUYKXLu-dN4pww

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
   ✅ Script saved: LJvq_V1YJBU7ljCG5zvvi5hCqCbhHy20RikzDHVW0Pz2wbpCHmg-VTwgqlKTzNBHoDaRLE5HDhz8-1hoRZgcOA

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
   ✅ Script saved: gC05wL746QvX1zQNcxXHmxyJSCEoVrKoNgb250z_lUEEOlOwAW3AafDm9J00UL5ePSh5r6tXzxTrUFw9wLKg2g

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: 7GYcMVHox8X2VlmfwKSt...
   ✅ Deleted script: vJgzEQmKxafgelpkVWAW...
   ✅ Deleted script: LJvq_V1YJBU7ljCG5zvv...
   ✅ Deleted script: gC05wL746QvX1zQNcxXH...
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
Inserted: map[id:8Xjm9XIaBnfJi42ZPq3zTxyo9ZM6CjKJ228zhw2VlDgRoxfuboJhqDbhen7S8uCMaioZiCU_4IgRhLULGYVbLg]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-01-03T23:55:11Z] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:8Xjm9XIaBnfJi42ZPq3zTxyo9ZM6CjKJ228zhw2VlDgRoxfuboJhqDbhen7S8uCMaioZiCU_4IgRhLULGYVbLg metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Test Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:42]]

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 0
  active (Boolean): true
  price (Decimal): 99.990000
  created_at (DateTime): 2026-01-03 23:55:11 +0000 UTC
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1 tag2 tag3]
  metadata (Object): map[key:value nested:map[deep:true]]
  embedding (Vector): [0.1 0.2 0.3 0.4 0.5]
  categories (Set): [electronics computers]
  data (Bytes): 11 bytes
Plain record: map[active:true categories:[electronics computers] created_at:2026-01-03T23:55:11Z data:aGVsbG8gd29ybGQ= embedding:[0.1 0.2 0.3 0.4 0.5] id:8Xjm9XIaBnfJi42ZPq3zTxyo9ZM6CjKJ228zhw2VlDgRoxfuboJhqDbhen7S8uCMaioZiCU_4IgRhLULGYVbLg metadata:map[key:value nested:map[deep:true]] name:Test Record price:99.99 tags:[tag1 tag2 tag3] user_id:550e8400-e29b-41d4-a716-446655440000 value:42]

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-01-03T23:55:11Z] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:8Xjm9XIaBnfJi42ZPq3zTxyo9ZM6CjKJ228zhw2VlDgRoxfuboJhqDbhen7S8uCMaioZiCU_4IgRhLULGYVbLg metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Updated Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:100]]

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
Collection created with first record: QxRnPpdUrtS5UIJhxanRdSId0Y0pFaAr0xuuRkWIaEjXrR93D59P4LuAJT95rRUXA21V-y2yrFK-XaoYs2s8tA

=== List Collections ===
Total collections: 14
Sample collections: [chat_configurations__ek0_testing ttl_cache schema_documents_client_py scripts__ek0_testing test_collection]

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: l2f0a9zNgkizL-DvmJUflGhGp5idRClogdXEV2-fgRkR1N7cG8I4GodpFdjJ0B4baaYbf9OmpGW5XfFWVTVgbw

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: 6FFq1lvd2WJNHuyPbNZBqMdhlwzLz34Rmgt0l3G7RFqLtotbSKG-P2lv8SLqvCrxz0LG2qHilcHdgSSdfeNfCw

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
✓ Inserted test record: v_LF3I3j6JS1LmTRSLG6_Pr8rPvskd0_wXrriIp6VD9NTw78mNdn5cqNAbrDlUGRdsWEUrt4NVVqpQcCpojPGA

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
✓ Inserted document with TTL: bVpQU5MegOFfcyAm7_ueqFzvrtBQMyq-Gpi6nOna49kIpllavTAdMxXnadEtPcyFU1aL8xI4FVqT2BNBa1sCYA

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
✓ Created session: Dogxv1Bte8IojZZSfjozm5Q7Bh1PRfDRm7Kts0TLSDNhwVoTkZU9B7Zyryl4unpGYGP_brfhd1cvzWqBLSZoxw

=== Sending Chat Message ===
Message ID: 0xn0dFJD1VfCm64zI5IvKGMU6P6lyQdOL4dx-M2qG1paMKzMaDRKkLvigo1zTN99wpXHxAXDjUUGhmXZT917ow

=== AI Response ===
There are three products available:

1. "ekoDB Cloud" is a fully managed cloud database service product. The price is $499.

2. "ekoDB" is a high-performance database product with AI capabilities. The price is $99.

3. "ekoDB Pro" is an enterprise edition product with advanced features. The price is $299.

=== Context Used (3 snippets) ===
  Snippet 1: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Fully managed cloud database service product id:FEizPpFdbvIdbKCs2rhrOER_sKRXxNnS9uj6oWcJbxxh31d8MAERwrg51VtjKM3PiKIAHyxXuq1Kv1Uc1z8Xag name:ekoDB Cloud price:499] score:0.1111111111111111]
  Snippet 2: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:A high-performance database product with AI capabilities id:SgxOV0qcwe_YWcPUIaOFCkJp3ca9kOm9heY8lWI1eNerdy-r9Cb4sLt1BVJwE_LjAeRactV2MO2l6xnwfDUmLg name:ekoDB price:99] score:0.1111111111111111]
  Snippet 3: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Enterprise edition product with advanced features id:Ks42Fw2HcdyxMIXiE6Q2vcOtNYonBFfkm51ven6kxkNi_wP3DyWpZ0AJJCrNTIY0U0h7XPzrjMJ2Fx8dh0d2Wg name:ekoDB Pro price:299] score:0.1111111111111111]

Execution Time: 2312ms

=== Token Usage ===
Prompt tokens: 613
Completion tokens: 72
Total tokens: 685

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: KnySy_WEfsp_X7M9HFbWUQFUF0_e0-g83hQ4ruClauV_v3Hx3qsidknLRmcrBq9N9LjEYo3Qf3E-Zf9vp1BW0A

=== Sending Messages ===
✓ Message 1 sent
  Response: The product available is the ekoDB, which is a high-performance database product. It is priced at 99 units.

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: pXATh_AppcMEk36vCsV1Qc0Ulh_huI_7CZBq5JqUfdvSMC3YkbjN-l_VqVR3TzFIwofDcZofSCmvSMi-rZ0UKw
  Parent: KnySy_WEfsp_X7M9HFbWUQFUF0_e0-g83hQ4ruClauV_v3Hx3qsidknLRmcrBq9N9LjEYo3Qf3E-Zf9vp1BW0A

=== Listing Sessions ===
✓ Found 9 sessions
  Session 1: pXATh_AppcMEk36vCsV1Qc0Ulh_huI_7CZBq5JqUfdvSMC3YkbjN-l_VqVR3TzFIwofDcZofSCmvSMi-rZ0UKw (Untitled)
  Session 2: KnySy_WEfsp_X7M9HFbWUQFUF0_e0-g83hQ4ruClauV_v3Hx3qsidknLRmcrBq9N9LjEYo3Qf3E-Zf9vp1BW0A (Untitled)
  Session 3: Dogxv1Bte8IojZZSfjozm5Q7Bh1PRfDRm7Kts0TLSDNhwVoTkZU9B7Zyryl4unpGYGP_brfhd1cvzWqBLSZoxw (Untitled)
  Session 4: pJAkPJ50ceg34xRRKItswW6PzC6DxtbaJCb0cxXJ-vky-Pz_nkfgehDNTp6ms__wEKw2RaPUOav2-ltIrOmPqg (Untitled)
  Session 5: 9_4ief_zaxiUf23V_Kj6v5j3TWlQvXldFGtpK6XzyLDFL-gyHZEzZq_eHbrAMtOeIsKM1wIK4kdD5k1-aFqqQA (Untitled)
  Session 6: l4g7zNZrVeaCo5zuQPZks0Yqia4jcciEDUMpkyHfX0jkRBqSmIH-phsZn7hXgLEJV4kU7DCyyt3ahLVBx7oCHQ (Untitled)
  Session 7: Z2pocQ1N977hiiEZksfO2WyAC5l-VC1h9TJzhg0S1xbXfB2ONM-Qk_Zqb6aWaumPAg1PXd6Wrerc4mYNkdmKdw (Untitled)
  Session 8: CG2PChDTWEdZekN5sWiZVrf_WLAtDRjp3PJxpTJjFYSqQCVmjeLDQWziHPt0aaTvTUT6V0Yt986-P4qwCZzNRw (Untitled)
  Session 9: peLCg_SRnDYQAjqcOCWo_IJvvhygPTvZtRKGBFOslnP8qxJdAALGQnFev7fga-KnBOZNPjrLWG6uIi8ibXIUuw (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: pXATh_AppcMEk36vCsV1Qc0Ulh_huI_7CZBq5JqUfdvSMC3YkbjN-l_VqVR3TzFIwofDcZofSCmvSMi-rZ0UKw

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: BEqv94BGuoMeQYX3pPbBjU1uRADxKTwYXUobq_-J6uV5oV0zw0peMRacpEdA3llV_MT-14zp-6c_gwSMTogkUQ

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information available, one of the products we have is the "ekoDB". It is a high-performance database product. It is priced at 99 units (the specific currency isn't mentioned in the context).

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
✓ Created second session: o_BggN1qIUogt0S1WPe31EPA0f2Y0eNckWn74WiDxXaSXeoZx_KvkCI0z7b3iqRZc9GJPCmI5xLqZFA-U-BXkg
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
  1. Score: 0.769
  2. Score: 0.757
  3. Score: 0.744

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.697
  2. Score: 1.503
  3. Score: 0.308

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

✅ Script saved: jE714NoFoty0Gvf-0woxTzOwW9OO_DXrNBKjHfIvMjT9m8MJiyVHwKtomCvV9ZUsDg_nrsXTSJYhrQDgukODvA
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
Created Alice: $1000 - ID: cwU6n-UvHoFMAQFtAVXtAT2H2_CsZN1JQXHJdIHKQVIeUL8PgLfvjPxP5RqMHz1P22AyKViBsf9NqtM1N3RD1A
Created Bob: $500 - ID: 9xxK_saVdjc8rP2LxJFNTXbbA7aCR9bHrKmxmi6egYouCB19aTRGTm2X4rFWRM8RMVSs-3p9-fnwXjWgiBkwag

=== Example 1: Begin Transaction ===
Transaction ID: c471465e-bb82-4883-83bb-af9b0efec628

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 9b06fa74-4694-4eb4-96af-038eaa089e93
Status before rollback: Active
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
✅ [32mGo client examples complete![0m
