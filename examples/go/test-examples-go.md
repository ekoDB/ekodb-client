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
Inserted: map[id:Ub49L1WXGY9wW0hHHOXwTXWviUMlUkIMqf6wNgy3M2TNUN66jEQl0QnEOMYXOQynzN0z4-QiBHlgAMxZ8-U6CQ]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] id:Ub49L1WXGY9wW0hHHOXwTXWviUMlUkIMqf6wNgy3M2TNUN66jEQl0QnEOMYXOQynzN0z4-QiBHlgAMxZ8-U6CQ name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query ===
Found 1 documents

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] id:Ub49L1WXGY9wW0hHHOXwTXWviUMlUkIMqf6wNgy3M2TNUN66jEQl0QnEOMYXOQynzN0z4-QiBHlgAMxZ8-U6CQ name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
[32m✓ simple_crud.go completed successfully[0m
[34m
=== Running simple_websocket.go ===[0m
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: 1JNp5xbnQN5NRBa8fEEUHRYA9jRzodnChyV6YjAfYfef-s0OYAwTz4IUNZObWzDqbBNaYEZYexQ6TF4zhalGVQ

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
        "id": "1JNp5xbnQN5NRBa8fEEUHRYA9jRzodnChyV6YjAfYfef-s0OYAwTz4IUNZObWzDqbBNaYEZYexQ6TF4zhalGVQ",
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
        "id": "kL8DVSUh7_LbJlGQ8Jp8rpmf3HJUvw0sR1K61EIbOfhPQNbCdjciPwd8xTHlb6AY0AiPX1Gmo0kplRygdd5vow",
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
        "id": "IjdISjptT6bysMqTDvn82myude-F1EHII_2b_ly8-sMxVL2QcMVlBbQPhso4ZGmgB5bQBHKla7FbAnTqScFsJw",
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
Batch insert response: map[failed:[] successful:[QdPO4lQCTPr6atcSdpS3eRexi7HhG4CJ-LhmsGw0ECy2opCm34DS-V1kMsF_15LPmr2aP6le73pZPPAz0IgpUQ JYydxlDZVx8DAizKancU7PgXqzjmrlywZ_OQa-mYtc3dZClh8EwZEqB8fWJBZ5cVDsNmaH6conM4kqfogwlZoQ JyZoEdUYOSBAzYtvihRO2Qls4dDcj_pwWouzxEquN_ePvXyidXs6hpHNgfxfFHUIOULZDYcxSwflisOsoyRu8Q bjw1QwUxwru5beaXcrJCWuBrVQ3Sx_2H_jOzAYkZ-aFYYiYobuRcQG94QdpZ2tQTRywX3-BmUwihZ1HqEXeapg W6Ez00HJKblWr5oemKFhfPsaQw3RmX_krWHdv9nPecQV1AF2WtrVcejSHA-MGcPhY8Y6Bth-KToZsr134c_PYw]]
✓ Batch insert completed

=== Creating test records for update/delete ===
Created 3 test records

=== Batch Update ===
Batch update response: map[failed:[] successful:[oYvh5loGgYXfhzHUGCDaqCOLGvKccDNi1yPEG23xXySHRwHFfntsz2VsDs3eiRHJiUhElBoWncZXEjg9d_0hmg tRz99h9CkeomIZUPbUx5HomBqdVoiG8l-aDjM-KD6Ut5JaCDAwlurgnshSmGRlArEYfePxegxozFHyUZ0DOmcA RqCJMyLRHvjJwdIN6KA0MWG2x65mNOK7-9CrvdV9a5RwthkKbpP9uVsqBJlrI5rpZpxIdR2xtUHVW08HErWRMQ]]
✓ Batch update completed

=== Batch Delete ===
Batch delete response: map[failed:[] successful:[oYvh5loGgYXfhzHUGCDaqCOLGvKccDNi1yPEG23xXySHRwHFfntsz2VsDs3eiRHJiUhElBoWncZXEjg9d_0hmg tRz99h9CkeomIZUPbUx5HomBqdVoiG8l-aDjM-KD6Ut5JaCDAwlurgnshSmGRlArEYfePxegxozFHyUZ0DOmcA RqCJMyLRHvjJwdIN6KA0MWG2x65mNOK7-9CrvdV9a5RwthkKbpP9uVsqBJlrI5rpZpxIdR2xtUHVW08HErWRMQ]]
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
Collection created with first record: mlXk5Mn0fFZUgpA8--mMsBQ_ULHh8kgSoahpdPAOa1538au4INApfGc98tkIwfYbR7cG__t23e_Z4DEb45hYWA

=== List Collections ===
Total collections: 15
Sample collections: [test_accounts users ttl_cache ws_ttl_test chat_configurations__ek0_testing websocket_test test_collection schema_users_client_py chat_messages__ek0_testing demo_collection schema_products_client_py scripts__ek0_testing batch_users schema_employees_client_py schema_documents_client_py]

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
✓ Inserted document: KD5gTt_zcZrwhDq_kUzTLO5oyFAvgCiTwMn68lNT2RAcm__7UrNuY-mcDgbIgBuwyjAF1l4w7BJttx9AeN9mZA

=== Insert Document with TTL (5 minutes - integer) ===
✓ Inserted document: <nil>

=== Insert Document with TTL (30 minutes - duration string) ===
✓ Inserted document with duration string TTL: FGWqrCgrPZN6ZYp4rMd9Khm-_aCsEPnWjuOu-f3-Gc6i13pe6ELVGc7eH_qkN8_zJr8nY601JJiG9fJ4wggWUg

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
✓ Inserted document with TTL: O3lTGA8EyNtSwFtDQ6zvCaah22VF9giWRtCf4_WFOIQ-T1dpPnuklrAMyP0zWgEIzWE5RZXkttV0ZS5YsbCFOw

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

✅ Script saved: _gHKtBlzlBEb2DpdtQ-ir5cJdNJi7KE7VY7GPRE7vCNc1L_2JwbQ6TRRH_-VJaRE04JDqJQiEc8TwmWFvQ8GWA
📊 Found 20 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: 2V4jhN34bH7fCVR80TcWlU1csE9s6cdDWQksqfp_x2Tr60yg3V6UW1O9mzDHycFy2XuM00tnUGk9llMpY4tSSQ
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: Oe8ArE_Cw458SI1BP51KFKft1Q9phiKUDnAXChyRQIsq8Ya7dNbOgOclwNUG5beE4qpW30JQe_3ErmFN1RYRiQ
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
Created Alice: $1000 - ID: k5QhYPbJPTIxR5t7KOL5T_-QNZVejOjr3S8SMt_JCo1FYC0xwnH43UT3hxO1B81khraRrrssIYwaY4ib2Zp_Tw
Created Bob: $500 - ID: G9RllK69GTVz4uZTRqpiUob1VQueWNacvhQBp_w5Xe06cSVJj1i9uYFPy6umJapfRxeKam-iMl4inF4XPiE-SA

=== Example 1: Begin Transaction ===
Transaction ID: 5ddaa8b7-40c3-4d2d-92e5-d0fe8c488bac

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
New transaction: 5943ea02-ebdb-4e74-8935-bd49f6970c06
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
   ✅ Script saved: ZvGvhoJRgnSMyNx-qPvQFnccaIwpl9ZmBDAff9mIw5DwCquxiBq1C381SWkq4vJAJIFpG3vq6nzkvadP120tuA

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: pifxT9tZabIKcSUk4vqAdCT_6EuBzKVz7fx5jnh_u_ZFNG9yPNgKp9wYz4fQ5-EMEP0KfopdJ16qy8cClLB1pg
   📋 Name: map[type:String value:Alice Smith]
   📋 Email: map[type:String value:alice@example.com]
   📋 Status: map[type:String value:pending]
   📋 Credits: map[type:Integer value:0]

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: Wet2uJxYv234-Rb-e2nK9a1OcIhA54xEymNCYh3NR9rf-GFUynW4qllJJf27qUTatvaOzmxPUEwr8KW1gclmdw

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
   ✅ Script saved: aUdV7ZdHQliI8ck17kGHiYys11ujOf73SVzhes3ou2ljCR7a_wMa7qbJ-EAS4dSTCPmTCnv2N8u5if_uYKztzQ

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
   ✅ Script saved: RQhEzNGjS5x4M8uf0LG6VoSCGGGcMg71o4YhYgalFbDIOy4PGEagA7IAyvOF9TIwCleWnwMa652nUTeANJmXnA

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: ZvGvhoJRgnSMyNx-qPvQ...
   ✅ Deleted script: Wet2uJxYv234-Rb-e2nK...
   ✅ Deleted script: aUdV7ZdHQliI8ck17kGH...
   ✅ Deleted script: RQhEzNGjS5x4M8uf0LG6...
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
Inserted: map[id:hDErmXATtxWzYNabBUPsW3vhrDPqyy_GN6vsAJLacuv2WweJWGq2wkn5c_yXu3gowQTTgh3FnWN4wjkDTOtBiw]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-01-03T09:51:08Z] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:hDErmXATtxWzYNabBUPsW3vhrDPqyy_GN6vsAJLacuv2WweJWGq2wkn5c_yXu3gowQTTgh3FnWN4wjkDTOtBiw metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Test Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:42]]

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 0
  active (Boolean): true
  price (Decimal): 99.990000
  created_at (DateTime): 2026-01-03 09:51:08 +0000 UTC
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1 tag2 tag3]
  metadata (Object): map[key:value nested:map[deep:true]]
  embedding (Vector): [0.1 0.2 0.3 0.4 0.5]
  categories (Set): [electronics computers]
  data (Bytes): 11 bytes
Plain record: map[active:true categories:[electronics computers] created_at:2026-01-03T09:51:08Z data:aGVsbG8gd29ybGQ= embedding:[0.1 0.2 0.3 0.4 0.5] id:hDErmXATtxWzYNabBUPsW3vhrDPqyy_GN6vsAJLacuv2WweJWGq2wkn5c_yXu3gowQTTgh3FnWN4wjkDTOtBiw metadata:map[key:value nested:map[deep:true]] name:Test Record price:99.99 tags:[tag1 tag2 tag3] user_id:550e8400-e29b-41d4-a716-446655440000 value:42]

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-01-03T09:51:08Z] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:hDErmXATtxWzYNabBUPsW3vhrDPqyy_GN6vsAJLacuv2WweJWGq2wkn5c_yXu3gowQTTgh3FnWN4wjkDTOtBiw metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Updated Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:100]]

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
Collection created with first record: Hi_a3G4zCN8te5bH0m47pVvOT0q404ibIipxGLjePjNL5QBOxzUgAiy2eREmkxpyzY8kF5gONq3W7UmOHNrZfw

=== List Collections ===
Total collections: 14
Sample collections: [client_collection_management_go test_accounts ttl_cache ws_ttl_test chat_configurations__ek0_testing]

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: Whd0jZiDFISxdkp9WH5hUB6ItgcLNQYpNGPcn5d93-DhATUZ5nC25tL-JayOzSVYaybKcr6pjIqGqzsRaKCdRA

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: xLitIGPsHCjNpzlBhZ7qo_HcAwC4HYTuBLdlsRiBC6DA_x6ENx_Ovjzb77p84ir4y1IpvNS3F0p6aDAQt4Hxbw

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
✓ Inserted test record: wDXf6aWUmBWm4zSUF_I5rSkoerTHFLrmORGxlGJaYFK4vpkHKOHkFdQhl6s6da4osy42li8BFGXipQ2EeH2MEA

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
✓ Inserted document with TTL: W9K2zRgJ8p4_oWAcUfixp1F8DTyi8EIXC42M47uXPqNg-oV-Llr1mPovph4xonA7oll5mH8Fm8r5LyV52D2DqQ

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
✓ Created session: IA5-rWYt2lejYRO9aNTZg4W-5TeMAvxB42a9TSCPeQw1CwR_yBAuwn2TANGMOssgAlVBKIqvHCr9wPW1BHcn5g

=== Sending Chat Message ===
Message ID: Ex8TPEExV9UDcaDpootf-ivJXauHIuA0dHMlosdUERW9E6SfUfkABYo2cfGETgGYiKN6tyMA0-orncHpwFLtiQ

=== AI Response ===
There are three products available:

1. ekoDB Pro: This is an enterprise edition product with advanced features. The price is $299.

2. ekoDB: This is a high-performance database product with AI capabilities. The price is $99.

3. ekoDB Cloud: This is a fully managed cloud database service product. The price is $499.

=== Context Used (3 snippets) ===
  Snippet 1: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Enterprise edition product with advanced features id:j1dwIwow-6bPVUjI11s0MJZh6d-fRLE1Y4PEyUmOaeoZ52GzfV3k31fc6ZadsxGq3L5L5-0-_BvPLCv0Bbksyw name:ekoDB Pro price:299] score:0.1111111111111111]
  Snippet 2: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:A high-performance database product with AI capabilities id:CLhHlMULpkCCK8OxlWZ1YpmMzH93WO9T4qI7qH5d9Kz3r5modKFuZCYGMsdVEa7aY_COQb4DUtT89eha41-Shg name:ekoDB price:99] score:0.1111111111111111]
  Snippet 3: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Fully managed cloud database service product id:b0eSG5BHs4XZB3YFa6gol2RLRXbZNYdqi7OdHJrhfBxHmV_rG3U-usd64G7HvlnfJ6QgGcyUbvBG2VqgTBj_xA name:ekoDB Cloud price:499] score:0.1111111111111111]

Execution Time: 2922ms

=== Token Usage ===
Prompt tokens: 616
Completion tokens: 75
Total tokens: 691

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: cW1cPq-jltIWG_-9ktoHU7rmTVOUdCHtyHfIf3qKNv2xf70zgTMgqEs5j74CEdrtwEPO8J8FdZPBC7ZzrPe3EA

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information provided, the available product is a high-performance database product named "ekoDB". It is priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 3vmzfLB-WmvB8idZJB7jLZqj0YQQ49rOv6FirnUVrQeZrdJ1DLq76y_li-PXJCbZtnxU8LGX_0daiNa5PI3Z_A
  Parent: cW1cPq-jltIWG_-9ktoHU7rmTVOUdCHtyHfIf3qKNv2xf70zgTMgqEs5j74CEdrtwEPO8J8FdZPBC7ZzrPe3EA

=== Listing Sessions ===
✓ Found 9 sessions
  Session 1: 3vmzfLB-WmvB8idZJB7jLZqj0YQQ49rOv6FirnUVrQeZrdJ1DLq76y_li-PXJCbZtnxU8LGX_0daiNa5PI3Z_A (Untitled)
  Session 2: cW1cPq-jltIWG_-9ktoHU7rmTVOUdCHtyHfIf3qKNv2xf70zgTMgqEs5j74CEdrtwEPO8J8FdZPBC7ZzrPe3EA (Untitled)
  Session 3: IA5-rWYt2lejYRO9aNTZg4W-5TeMAvxB42a9TSCPeQw1CwR_yBAuwn2TANGMOssgAlVBKIqvHCr9wPW1BHcn5g (Untitled)
  Session 4: kjK1pDTNLOKHPH9OvZuvvQLFzTMCSQt-byLD2bA78E2U9VUsJHSz_I5zfuQo1BXHoSW53CpQnsFBhK4Q530HVw (Untitled)
  Session 5: r8-q1NPgr8SOnlJxjWfntm6zuChhix2h6xtYe5ervcSBCOE-h8CFI_i7m9pJxLVzK_gop-dpdEHNtpNLVEH2Zg (Untitled)
  Session 6: YQzzCfZjPdw5kP1aBN6THyY1bRvLMtCvolkzyPoRuQRuLMHMWeb6xYWJbJXPi-iZTzqBLMxeJJFJzPXJlfwLbQ (Untitled)
  Session 7: kjC5gFDXqUMkJtTG9oQxoJ4GZsqn4-JnYCiIcGGKMR0243BGdCUoT_VC4DQP5SHwUY2nER_UmhkyajKM6mJdfQ (Untitled)
  Session 8: SI0IVjSitRK4WC0Rq1i2_ftWMw6wc_a-F_5jxxLXdDFcbDYkAVkRLYe-vu7rx-cnpa6CbU66anXhCjolvyc_ZQ (Untitled)
  Session 9: _7tHWwvnTeyUz_55D47bdvKv-LR17o5N-3frkweqyiY75TE85p882gSX8NaWWrOIRzliqYJXFtiK6qpW5i5ydQ (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: 3vmzfLB-WmvB8idZJB7jLZqj0YQQ49rOv6FirnUVrQeZrdJ1DLq76y_li-PXJCbZtnxU8LGX_0daiNa5PI3Z_A

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: PV6OHVsfD57CjKcOBKZcXPrKUhYEB3cGU7rr2xylkI6kd1J4LaZeg2937gfgLya0Yy9tWQOhJSNSGAk9kniZjA

=== Sending Initial Message ===
✓ Message sent
  Response: The available product from the context is ekoDB, which is a high-performance database product.

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
✓ Created second session: aRl2LkGcRd7T25p5PRat3KIdAs16ofTpaDxFNR29iOaKKEM6fI71deiXeF0clC7DGdm3GL9KoHsZbKVfRlwKSg
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
  1. Score: 0.785
  2. Score: 0.776
  3. Score: 0.759

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.714
  2. Score: 1.503
  3. Score: 0.311

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

✅ Script saved: CtMvQnI37q0ZxQT1Rtzvs3k2sDeqMHJ9lstW-mNMjzyWO_aa_TC5-Od7PX7d6DERHg41fifpbp5Nu6tk0QGV0A
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
Created Alice: $1000 - ID: Vmrf6ef8h3aeibUHq1V_30fzK3qTW2HMeIiGYO1f4l-U-N6YoxHPlw4KA-keeNiGsQ3iMQcthltyNTX6Q-KzVA
Created Bob: $500 - ID: RTWIiJf7IqqIoH6-JDgColpjsMfAe70bYo_TSr8WB2ravow0TZzTo7hYuUmetSPwYn3Qx1Q1HdCTEWAU2zbsnA

=== Example 1: Begin Transaction ===
2026/01/03 04:51:26 invalid isolation level: ReadCommitted (must be one of: READ_UNCOMMITTED, READ_COMMITTED, REPEATABLE_READ, SERIALIZABLE)
exit status 1
make[2]: *** [test-examples-go-client] Error 1
