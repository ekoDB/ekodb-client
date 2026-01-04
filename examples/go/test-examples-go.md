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
Inserted: map[id:vMZbsOa4wgdIL6bjVA95Je54RKOaHfSzqopzS3rX-NLBUm8xIaof4I9mHNLBTkpKaeXzp_ImSTX4-mEw0mtM6A]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] id:vMZbsOa4wgdIL6bjVA95Je54RKOaHfSzqopzS3rX-NLBUm8xIaof4I9mHNLBTkpKaeXzp_ImSTX4-mEw0mtM6A name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query ===
Found 1 documents

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] id:vMZbsOa4wgdIL6bjVA95Je54RKOaHfSzqopzS3rX-NLBUm8xIaof4I9mHNLBTkpKaeXzp_ImSTX4-mEw0mtM6A name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
[32m✓ simple_crud.go completed successfully[0m
[34m
=== Running simple_websocket.go ===[0m
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: y9NoyejrqnC0jG01aRCiN8LAwyxDTytOuFaianoVIj5ki1UHMIIB0_cmUCxZarYjkBfyk_34ya5TMs876CQIVw

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
        "id": "y9NoyejrqnC0jG01aRCiN8LAwyxDTytOuFaianoVIj5ki1UHMIIB0_cmUCxZarYjkBfyk_34ya5TMs876CQIVw",
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
        "id": "9jhiJD-vX2ICyYsmVzgVmtlyo9yTqcOWZTdsd5jfBvDaYIiuAKy4_bD3ASkcLX_E2gV2whXZrWIPWIfwLrXQAQ",
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
        "id": "LBF7Gequi0v04qbT1iANWTN-FRO0bYDKvukFbxdX9CO7wYKIchKcISe0P-UObFrpY2NNO2nXY-XcPKH9NW1CkA",
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
Batch insert response: map[failed:[] successful:[AOU4yto2RvqewJ2te7ppQ5POxNfmlmPA4F7RHP6pjIm3iDmzQy02r4xYgr6HG_dzKuSgKbzzTLIG5S90mbn3uQ VeC730SxMVTbVl_vGnwHqjSQsuyL7c8Mvm5KSQcqlrWtUflcn0OWWewrHyYXI7R9C2wuNQi_glCSrqkbvM5lQA 5UkLeHigdELGdOd7mmWWqd2w-UeD8fmZSzHWvtbQGXk99moszY0y8elImJmsau9I7W9zvrTlY8LUyWy0OpJlQQ 9ajghvCQcV0VZI0Vwnyex5yZ5F6Hp4JAGhM_wxD77ql72M-qzJ6xfFY9G5QyLm6o1mmKtJDwVrgSmH280kbY5A X6bIuFzKW3eV-MngaiNFgjLl3m9GeucYLHDrzQcTKzrLOWX0NfKI674DlrJaeKAlE62EuGkT7eNJOkRNp3fe_w]]
✓ Batch insert completed

=== Creating test records for update/delete ===
Created 3 test records

=== Batch Update ===
Batch update response: map[failed:[] successful:[E8NIP4KS9zqwo6zKIa0ScOYACJkR82vu47gmF8mq9mfaafgU2xbomxwg6ozhKddu2IKf4Xp-4S4GyNM4qOJk6g U9hbftrUMvvNGJgdmyv5E-d_JDOAed3T7L5TkIho0QVAK352cuT0NjXUI9g3kWvc_0FkERcgGvHcGtOP75XQHw lmciiJvtPyteIjZGd3ocA6Fb17hUJRVwrYeRYq40PQdY97-Oj95Sz6MHkI45F-qkkWUeMXdM2Nuix-4RRjgOng]]
✓ Batch update completed

=== Batch Delete ===
Batch delete response: map[failed:[] successful:[E8NIP4KS9zqwo6zKIa0ScOYACJkR82vu47gmF8mq9mfaafgU2xbomxwg6ozhKddu2IKf4Xp-4S4GyNM4qOJk6g U9hbftrUMvvNGJgdmyv5E-d_JDOAed3T7L5TkIho0QVAK352cuT0NjXUI9g3kWvc_0FkERcgGvHcGtOP75XQHw lmciiJvtPyteIjZGd3ocA6Fb17hUJRVwrYeRYq40PQdY97-Oj95Sz6MHkI45F-qkkWUeMXdM2Nuix-4RRjgOng]]
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
Collection created with first record: ptddHEPSQEaJB-fBdFuXup2nTQfYjiB43-9jWw-G8Fqat1_qSVwbkf0UJMdTDW7h5lp1LLU7v-XALrXsVANGTw

=== List Collections ===
Total collections: 15
Sample collections: [ttl_cache schema_products_client_py test_accounts ws_ttl_test websocket_test schema_documents_client_py chat_messages__ek0_testing test_collection batch_users scripts__ek0_testing schema_employees_client_py chat_configurations__ek0_testing demo_collection schema_users_client_py users]

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
✓ Inserted document: 93EUsAFTkEZid37cYhiq6DL-kf0RJ0bzLj3umKpYLumvn7FxYnjl3lwDL8IXmuulOtnOC9BjiL_iVRTW0Q7T2g

=== Insert Document with TTL (5 minutes - integer) ===
✓ Inserted document: <nil>

=== Insert Document with TTL (30 minutes - duration string) ===
✓ Inserted document with duration string TTL: IMuIAGkLu3H5-m5R0dm7z-zYd7SfeKswVNOK58B5GWl6v61rEfWIp3jgpfm4gUjl3x59p7esAWcLVpMgCWNKuQ

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
✓ Inserted document with TTL: mndcJS_9e9rw4DFBzHHar1STmp2Qjaxz0ZSJp5lf-tbAHoLD6PenjQZ_AdXUVbg3ZETlCm_QOcqkKurOTnJfKQ

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

✅ Script saved: voGIlYCtRq6Z-7Mlmt5OhVFcQthorYs5F61oQKcIvdZjinXFpuY_nQw82mL1RTTzSbXeNZwSlEXG-xhvvB2lyQ
📊 Found 20 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: yXoUypW4ag_ITjSTKrIoVXBr6QVUmBsv1AZqWK-edPRVnE3fbfK4jBxvAAxYJSw1m9_yrTAc77eMp0rv-NU0gw
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: 90f_d45fplSIo_ag-S-qknPZffuMbjbfNri7ns1ywx2jhi0mj2hY5XtM6EE7PlSC3ldxVlT5N9GthghX0F0hhA
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
=== Running transactions.go ===[0m
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: UCP6bkhJFRSHznVl9W7GIWz2wslTfRhMAVa-ZlFTRlOG-BzLWmwy_4Hb_p78axWOyz7mlJgVQwmWpMaYRHkKQQ
Created Bob: $500 - ID: HyL-iDekqT50foEfJ_gdmntwGMxaeOWhzCQlk_D8krG7xfigPFRALaFB83wxBrpWaUV5SXL6rV2aI27t3T58pw

=== Example 1: Begin Transaction ===
Transaction ID: 2a50b320-e77c-492f-9c3b-5ea900f18b89

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
New transaction: 1221b17c-d2a0-4208-a3c4-f4d22402593b
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
   ✅ Script saved: V1B5EkaDpzdZ6wfrI-2HvpK46n3OF_d6ftg2VGxYjg0qsddV4PbUmIcqa9_Qcl_n0y0cws5BeG9eXd22L5vlHg

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: mkA1fnH9tKXCpwLiOTS_BkgGz_u1hmQ2X4xKRzKxnlexyZyUEhMdgM_jCisDeGsDvBBwfdUp5gXZTBCkKnAcAQ
   📋 Name: map[type:String value:Alice Smith]
   📋 Email: map[type:String value:alice@example.com]
   📋 Status: map[type:String value:pending]
   📋 Credits: map[type:Integer value:0]

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: W98TGpVnde8q-GEvpVM60VXi0uu_RR5IC-vJCfX2fUE1a9vLhjqR1oLGSytJlyY9rojRzqTyVoH-aseYeXnU1w

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
   ✅ Script saved: 8zG0KGg18VAEU2CeY3bTdtjIknQfKN2JxEGd0ZmLMBslbvxESNPGmpoyyqJYkNQqUGkkNBeUfKADpMCMyCTsvQ

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
   ✅ Script saved: E3mpUECOwCaCQ1grRgDDyoKyDjpAaj78iQwVCl3xTqmbjLf9xvIuDeJBk2Time_VQFP5bYaHilDGnzU0ERdCgA

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: V1B5EkaDpzdZ6wfrI-2H...
   ✅ Deleted script: W98TGpVnde8q-GEvpVM6...
   ✅ Deleted script: 8zG0KGg18VAEU2CeY3bT...
   ✅ Deleted script: E3mpUECOwCaCQ1grRgDD...
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
Inserted: map[id:mXgclJsIydEPhyyDAAzGgpBCPQfaJpmOLi82-0fszHDLPcW3nBlMK0HGOqOPH0KAgu-xBNhg-WER-oc6sHp8Wg]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-01-04T19:37:46Z] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:mXgclJsIydEPhyyDAAzGgpBCPQfaJpmOLi82-0fszHDLPcW3nBlMK0HGOqOPH0KAgu-xBNhg-WER-oc6sHp8Wg metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Test Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:42]]

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 0
  active (Boolean): true
  price (Decimal): 99.990000
  created_at (DateTime): 2026-01-04 19:37:46 +0000 UTC
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1 tag2 tag3]
  metadata (Object): map[key:value nested:map[deep:true]]
  embedding (Vector): [0.1 0.2 0.3 0.4 0.5]
  categories (Set): [electronics computers]
  data (Bytes): 11 bytes
Plain record: map[active:true categories:[electronics computers] created_at:2026-01-04T19:37:46Z data:aGVsbG8gd29ybGQ= embedding:[0.1 0.2 0.3 0.4 0.5] id:mXgclJsIydEPhyyDAAzGgpBCPQfaJpmOLi82-0fszHDLPcW3nBlMK0HGOqOPH0KAgu-xBNhg-WER-oc6sHp8Wg metadata:map[key:value nested:map[deep:true]] name:Test Record price:99.99 tags:[tag1 tag2 tag3] user_id:550e8400-e29b-41d4-a716-446655440000 value:42]

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-01-04T19:37:46Z] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:mXgclJsIydEPhyyDAAzGgpBCPQfaJpmOLi82-0fszHDLPcW3nBlMK0HGOqOPH0KAgu-xBNhg-WER-oc6sHp8Wg metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Updated Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:100]]

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
Collection created with first record: LgzxqPtmMxmTRLSCEt6S_O4Vyvd6Uu3zhwMuYhO0cVRKJVBy7r9Lm2cfMUlnS8HFEUs51pv1LBi-gdFcX2s_jQ

=== List Collections ===
Total collections: 14
Sample collections: [ttl_cache schema_products_client_py test_accounts ws_ttl_test websocket_test]

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: I5NlmwPKjQq8OJrR-_220eBY5R2MrvSjjzkeYfjHEhetvR4tgUAZEhdWED6QK83NOmQM3RiOKUTKJopgdSVaLw

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: Kzn4B_TE1OyIplWfuG7i5rWPfJfQVaH-4fyjPCJOcTov2_6C9u9_yeE9DB77-c8oQNALuXuhV-WCd2Ch1eZixw

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
✓ Inserted test record: 9EBEsdCevxiIXptvK30YVkTmMV__gILck8LTvxwF543VUubQtYlAYyXcL3jEYjhbdV8DRXsp18QiQIf6Hp49VQ

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
✓ Inserted document with TTL: rkl1K6tMPJssGR-Tt6j03YxA3GnjFE-5Fq3GQ3wvei2MJkaX3qcdr_UVdYof3JU9Wq2zaUYb55b3t5-t8vZpjw

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
✓ Created session: 6c6I4tbKn4s5HXGqxA8u3X9GUS68Q-_arUUTv8flyLDZAgHVoQH7Il1ZjmGO5l1Wcc0-GL1_hEctqKojwhRU-A

=== Sending Chat Message ===
Message ID: g5FOe2S68Akz6LAimmdpkipAA4KsjzhULm86_ttidQ8LgxwbbOd7w1S_lhHqAoDvFkfrZrbRjGz1T6N30isG_A

=== AI Response ===
Based on the provided information, the available products and their prices are as follows:

1. ekoDB: This is a high-performance database product with AI capabilities. The price for this product is $99.
2. ekoDB Cloud: This is a fully managed cloud database service product. The price for this product is $499.
3. ekoDB Pro: This is an enterprise edition product with advanced features. The price for this product is $299.

=== Context Used (3 snippets) ===
  Snippet 1: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:A high-performance database product with AI capabilities id:DQ0JWQzNPOoktqQTBnfh9YmhdY6AhMn8JLP-Supfi1GNn8E-63kJ70BibIv63VGM3n03nEl_gyqWrEXw8l0qsQ name:ekoDB price:99] score:0.1111111111111111]
  Snippet 2: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Fully managed cloud database service product id:7SM-XcP2TbhQQmNA41a2NFoP2fQLSBLfUvybp6tiOqXJY_GkHz3JcFC6Dk7HY3mmNNJ5lVGrJd2y5KuV2VuMdA name:ekoDB Cloud price:499] score:0.1111111111111111]
  Snippet 3: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Enterprise edition product with advanced features id:L-RrDH3t_0mBJ_Y2u2m3f_hBPUq1vpyHpC8Cb-w7S3dIFHcyX04ukW0wwiSNaWJzEeds6iquYjf2WhbKqbi94A name:ekoDB Pro price:299] score:0.1111111111111111]

Execution Time: 2576ms

=== Token Usage ===
Prompt tokens: 612
Completion tokens: 94
Total tokens: 706

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: Us_Q6dEoGIzoWbZSxPo30f5vvCQWxznglrknYiDn6iY2PXbYFDs2NCf4h1Mm5xImmYJBDFwmEMjlqVTkY0-A_A

=== Sending Messages ===
✓ Message 1 sent
  Response: Answer: One product available is the ekoDB, which is a high-performance database product. It is priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: V5Qa3qamU75cR45Qkp4oec3EqnSXNUcSYqhl366Tl2t9DpM5EToFhEaAtpRhnX4YwQUzW7ih3TIAEX3x9nz8KQ
  Parent: Us_Q6dEoGIzoWbZSxPo30f5vvCQWxznglrknYiDn6iY2PXbYFDs2NCf4h1Mm5xImmYJBDFwmEMjlqVTkY0-A_A

=== Listing Sessions ===
✓ Found 9 sessions
  Session 1: V5Qa3qamU75cR45Qkp4oec3EqnSXNUcSYqhl366Tl2t9DpM5EToFhEaAtpRhnX4YwQUzW7ih3TIAEX3x9nz8KQ (Untitled)
  Session 2: Us_Q6dEoGIzoWbZSxPo30f5vvCQWxznglrknYiDn6iY2PXbYFDs2NCf4h1Mm5xImmYJBDFwmEMjlqVTkY0-A_A (Untitled)
  Session 3: 6c6I4tbKn4s5HXGqxA8u3X9GUS68Q-_arUUTv8flyLDZAgHVoQH7Il1ZjmGO5l1Wcc0-GL1_hEctqKojwhRU-A (Untitled)
  Session 4: jmpddhHFZ8sTVNF70g7Qmqj7SExneQLGhcthObJr-pu8acdXHAXCRIuxUGP_rkBZ9_M_cZt0hyqCSWJLO55HBw (Untitled)
  Session 5: rViBhRDXhRbwLJ7q7HgIWAR-WzJpWkLXUnD6trKYXIrKNfa6L9gDBL9gBxUojcl3yMP0zwN4IH5XVgtxHLKaXw (Untitled)
  Session 6: 75BwxYE7jLzRVr74SLD1UtuvPgDxlgpFIhcnXtSRADrscb60rK4_7_n7mx6Ip0Q5uiS2ZECQcBR6Z8uuZumVcQ (Untitled)
  Session 7: QRhmpERBtntusi1QZ2IH-CYZz8YqvGsUxP5rSLCZFUJV6k9Fh9IzpoKcyUkay5qYqXNuTe4_QsJe4LM3bnFRfg (Untitled)
  Session 8: Z5AKjIPI2yAcnUe0vBJQ876BWLJQ6XJWcYAMhqakwaXAEkhgRaePkXsgJTBf8TLtVPGSn7RWgBvBAq_zwKXWwA (Untitled)
  Session 9: o1-Gof096ND-RPEft85EWug_1lfr_pJ9PK5P-0h1_mparol8lKHJxwGPXtTCJTuOEkqpagS5eun4RH9azwSU_A (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: V5Qa3qamU75cR45Qkp4oec3EqnSXNUcSYqhl366Tl2t9DpM5EToFhEaAtpRhnX4YwQUzW7ih3TIAEX3x9nz8KQ

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: m0MPTKcmRmXi-6apnKaIbw5TD0tgD-Od21R_cvK-FXQQaNtlQ8Tv3-_A2d-wDBzFUjEcCyqRoSUe1JRrw70Ewg

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the context provided, the available product is a high-performance database product named "ekoDB". It is priced at $99.

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
✓ Created second session: _NL6OYBqQLoEupcizbL9l5texJ70hYuEDsy6eTjOgiLN5oGcWPOY8SXyDAtM8qpA8loeptso8XXiGosWevuMEQ
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
  1. Score: 0.739
  2. Score: 0.736
  3. Score: 0.716

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.695
  2. Score: 1.486
  3. Score: 0.296

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

✅ Script saved: Q6zYUKQNbe9lKb7me9Zlp8tYSx6OfQ7NyI7caJRcPv4rsXBtE9_HzJ898A_tFPvwyisv0SOHM3lseZbgG-113Q
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
Created Alice: $1000 - ID: sjtcLjCz7JKna6vBlXKEzkZMAD6__1nsxpGQyyAWL3H2HdXkA4k0JypBR4nr0mFW9_c5kBT9SzTz2kPNCGj-gA
Created Bob: $500 - ID: ZOfPc_vnUFZgjWT0yBg7-tAvzwNfsqlBWbz-u9lbbgaxxidX6QUhSp3TfhMB6WJFyuvHEhuGM5mMGafWmaVZAA

=== Example 1: Begin Transaction ===
Transaction ID: af9c9b9f-3bed-452a-bada-ca16a5e71dc6

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 307b2735-e02f-4c15-9c87-95ffc36f7d01
Status before rollback: Active
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
✅ [32mGo client examples complete![0m
