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
Inserted: map[id:42hQBr-K1do5CWsdo3bzNuFxQyEV6rckCDBQpCDtAbfAXybUw7Z-TqN8aYeo9-E0FC1ii8hWg5GnCLwH9bqsww]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] id:42hQBr-K1do5CWsdo3bzNuFxQyEV6rckCDBQpCDtAbfAXybUw7Z-TqN8aYeo9-E0FC1ii8hWg5GnCLwH9bqsww name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query ===
Found 1 documents

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] id:42hQBr-K1do5CWsdo3bzNuFxQyEV6rckCDBQpCDtAbfAXybUw7Z-TqN8aYeo9-E0FC1ii8hWg5GnCLwH9bqsww name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
[32m✓ simple_crud.go completed successfully[0m
[34m
=== Running simple_websocket.go ===[0m
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: z9LcWWNnG9qb3sE9I9ROC59IDx8hjNWn2hIqE9g6JS3nfKdpi4aLfs8iMwbAqZy7BhMTRTR6uqt3iSx83kVaIw

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
        "id": "nM-4Yeiy_NNKrQ8nYlvtKtF7PcI8tNE5I1l7Gsolj9YOSEoI5ZHPa-lL6fSZAOlMYl6LvJkAmUO3NNZv-wV4Aw",
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
        "id": "a3HSAeDJ4dkj4hErQeiyqFAv339ot0i90jIBizcekkcFWpdStzLfwrQA-ejOxfbPFxFuF9w977m4x7DpOIeeUA",
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
        "id": "z9LcWWNnG9qb3sE9I9ROC59IDx8hjNWn2hIqE9g6JS3nfKdpi4aLfs8iMwbAqZy7BhMTRTR6uqt3iSx83kVaIw",
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
Batch insert response: map[failed:[] successful:[Jc8xk6_U68NfhaUnIKyd682tkICc0WraZylJiYQeLfxKzafwubi7mRO1Lnl6zMWupFxnIN6QES5_2CmGeg5vKQ IN3fThyBehxqX5C5TgkPsOXtYZ1zUfzgKGSQCK4rHdUigrZ-304i_WyUvap9JnTV7M6P4TFrvZaghjy_pnbXyw XDpegAgaQQssBdQ3ZnzbhQeT9-H9RW4LLkcJ34kkecXLcVs3hjKrIonGDwYVcWpZspXFrPIePAeCbQBODigFmg d_qwnswluZO_-W1wP7JQrzHLPUJlL46pglkUMoNelotS9TkB48vOOqMruVEPwa16QFOZX0YOzRedHwANTWwi9Q iJ-aGNbJ5xWUOOFm3BH9HLWamkPPbzBJ1qt6wANk7E-vVNt23oxpvbm9LSQm_ddjFOV86hTzbRXgyqBzqvZ1Pw]]
✓ Batch insert completed

=== Creating test records for update/delete ===
Created 3 test records

=== Batch Update ===
Batch update response: map[failed:[] successful:[usPZUuFjxtalMEItumQwQVVr6Y1bSJjK12vTDaGgisyD5_BNc8rcBSpVfoGSvtcJd1bKx1DDykSDo5CiHkrvuQ -4Q-Du7fq9Ytzxxm_7w1aHZQtsG8Ce03464kJ1NR7ktp_jf1f_mHxoYwWyWBVR9Mk89UnZHeT5k7AOH1WFO9KQ souSs-xEo9ohjKK9YrkBNoee3JX_uTGFScclC2w4ewS-4yHAyI07ztfGzc82A0R7viBTTGxBO_AXkiBCp_Ff1w]]
✓ Batch update completed

=== Batch Delete ===
Batch delete response: map[failed:[] successful:[usPZUuFjxtalMEItumQwQVVr6Y1bSJjK12vTDaGgisyD5_BNc8rcBSpVfoGSvtcJd1bKx1DDykSDo5CiHkrvuQ -4Q-Du7fq9Ytzxxm_7w1aHZQtsG8Ce03464kJ1NR7ktp_jf1f_mHxoYwWyWBVR9Mk89UnZHeT5k7AOH1WFO9KQ souSs-xEo9ohjKK9YrkBNoee3JX_uTGFScclC2w4ewS-4yHAyI07ztfGzc82A0R7viBTTGxBO_AXkiBCp_Ff1w]]
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
Collection created with first record: eiYb5o4lzXJwVo2Pg-tcUC9mMS9KLuiTYMIaushlS8Ai4fU-cQHoqjBmGGetTB53cqBOf-Fc8U0fBwmvgg8heg

=== List Collections ===
Total collections: 15
Sample collections: [schema_documents_client_py batch_users chat_messages__ek0_testing test_collection schema_users_client_py users scripts__ek0_testing websocket_test ttl_cache test_accounts chat_configurations__ek0_testing schema_employees_client_py demo_collection ws_ttl_test schema_products_client_py]

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
✓ Inserted document: Wf5KL0xVueg5myVivUP5GrtM0VED7Jk_SZEVb9HusRep58iIin1jhC5yxpAemSzLZ9-9ii8HGPUYxjBVPqc9ww

=== Insert Document with TTL (5 minutes - integer) ===
✓ Inserted document: <nil>

=== Insert Document with TTL (30 minutes - duration string) ===
✓ Inserted document with duration string TTL: pvuNPqBtvqDtUsKUZXQJ_EaVYzmWv9LAkXY-CoJ2R9TGJqg5F4pWgYCKxwTSqybuSPEhqUO7-anhjDlU_GxMgA

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
✓ Inserted document with TTL: tTCUtfcvOjaefWWxGT8r-uTjiuX4jBJuHraAhjYbBSOw4N5_zJ2tlWjcWmPdodphEogjylkUH409I1bXWAf3ZA

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

✅ Script saved: pWLLmv4AinBEnwggFzoRyFuM5M-sxkQ4tfsGe_R6KOU-Qh06I_QelaIs8K-xH1_HUi7MVxv1l_nn6y_HC_kmMw
📊 Found 20 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: kII-H96vZIex6YhSu12qcqQ4zO4x0aSVBE9x7sT3MNi9SY-seSNXNkK5lH-9X5wozEHKZ2fWLFjAEneziV5Ixg
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: KggbccaX8Mrt4IkXgsUTyAaJGU2W-iuqTNMTzA7ypFbn_OVRvcuN_b1jhNVwpSopGNo-zmkfDftWM-op9gASXQ
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
Created Alice: $1000 - ID: Y8dVaHLQ0_GrJfEvYNq-_jGcDTaCyPWg0ckVbyDiZ2vx70WDeoxC2c5vKwdxnwJSxwZeKSTQ1MsHWkqHwQ51hA
Created Bob: $500 - ID: X4jeL4iUIdAOk2AMLf-LZt3gPvT0Pmod3kb5RQ1iupeCN3QLhvLowWO4RUKL6A_YVgZWA08BxkTujYoCcGjRBA

=== Example 1: Begin Transaction ===
Transaction ID: 6bcbba6d-d19c-4517-a346-067bea458d3c

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
New transaction: d47d41cb-cd5f-4bbe-8173-143857b2dd4c
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
   ✅ Script saved: jdU2z_I6Xn0e_YY6nm3IQRCnabqllNQKDRjlrk7SSCFzkeFq-HA0Sm6DIoyy_2xQgJhK8Q3XwHp0dK8H1yTS1Q

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: hfo5FDbcaCOThwtHZUjdKTDMNAVqVCMSCS09GTyJD8qlMOe_Fax_3vzwpPwpYrix2EyujQh3lvuRfd-oXw_hwQ
   📋 Name: map[type:String value:Alice Smith]
   📋 Email: map[type:String value:alice@example.com]
   📋 Status: map[type:String value:pending]
   📋 Credits: map[type:Integer value:0]

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: kkSgiJ1XbYyu9nIfHjqE2cVw0NfbcUJFBIaDhctXy6IdrziXRttFOwtaBGxXLtaO97y-JWPIOrg4Gbr9WXtLDw

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
   ✅ Script saved: JEcFsI6jAAeAo23tpXopVx82EiDfpw7x62xeAMBr9UHhP_Pjoyr6niExjf5MJjf9lso_L8gUSHac3mecRXxTeg

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
   ✅ Script saved: GGHQx6wkSOaJyP9sr1vfRgJ9SCimWbUBmcydcmHw-u54ngjJY2xibu66vE2pR5Qea4AamoT12izaVoJgfhrYag

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: jdU2z_I6Xn0e_YY6nm3I...
   ✅ Deleted script: kkSgiJ1XbYyu9nIfHjqE...
   ✅ Deleted script: JEcFsI6jAAeAo23tpXop...
   ✅ Deleted script: GGHQx6wkSOaJyP9sr1vf...
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
Inserted: map[id:Cl4dcMupgQpOA9Dm7Cu3ial5XePtSz3UTm2x_5oqu7OeQ3YsKV_mylF8d6n6zwWd-crcaJxjFiVX7-SBofsI5w]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-01-04T06:32:45Z] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:Cl4dcMupgQpOA9Dm7Cu3ial5XePtSz3UTm2x_5oqu7OeQ3YsKV_mylF8d6n6zwWd-crcaJxjFiVX7-SBofsI5w metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Test Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:42]]

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 0
  active (Boolean): true
  price (Decimal): 99.990000
  created_at (DateTime): 2026-01-04 06:32:45 +0000 UTC
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1 tag2 tag3]
  metadata (Object): map[key:value nested:map[deep:true]]
  embedding (Vector): [0.1 0.2 0.3 0.4 0.5]
  categories (Set): [electronics computers]
  data (Bytes): 11 bytes
Plain record: map[active:true categories:[electronics computers] created_at:2026-01-04T06:32:45Z data:aGVsbG8gd29ybGQ= embedding:[0.1 0.2 0.3 0.4 0.5] id:Cl4dcMupgQpOA9Dm7Cu3ial5XePtSz3UTm2x_5oqu7OeQ3YsKV_mylF8d6n6zwWd-crcaJxjFiVX7-SBofsI5w metadata:map[key:value nested:map[deep:true]] name:Test Record price:99.99 tags:[tag1 tag2 tag3] user_id:550e8400-e29b-41d4-a716-446655440000 value:42]

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-01-04T06:32:45Z] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:Cl4dcMupgQpOA9Dm7Cu3ial5XePtSz3UTm2x_5oqu7OeQ3YsKV_mylF8d6n6zwWd-crcaJxjFiVX7-SBofsI5w metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Updated Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:100]]

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
Collection created with first record: bcAx-Gxmaf6CoKnIkqRqTBHaMG--aAdbpW2ok2pN9T5JQqQKHQIEc25T5QDqlgorR4fdYQIX5eUpWyxfJXL0lg

=== List Collections ===
Total collections: 14
Sample collections: [schema_documents_client_py batch_users chat_messages__ek0_testing test_collection schema_users_client_py]

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: PQ5PJuzxyCIGkE2KgcTucdpZUaS4Jsp6yKzH-6VjqGZAFDFjeeKSDWUFB69qPWuhztYNVw7hshuCbgbzr6b1YA

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: eZj1mjbwa0E8PsyPjloRpiyoFGYavXJZIMu7Lb56DdmgSu-yR_ZyZ_GZwUm-2awINs1YCa8lF6zLLqnjXwT1-A

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
✓ Inserted test record: sz_RlPQXr_xJBLbHnZoiUzLFkVZ4aCgnWskFn88dcSw6UAHWejDSFBxYCCFZzU-qyhOelYsbrheyU0Nv6h_YCA

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
✓ Inserted document with TTL: aA8zTBFzNabVQm01skFAsAF_PSonHckucPUzxlHJC8Lq_T0S97hJuA-eimUXO9L9mGrO7z3zaf9PEDBXvYflEg

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
✓ Created session: iFUlZ1cqvlI-L5mwayMbK6rRYZHtvP0C1ZkjPMznuqaHhq4fUa3SisMopOjLCPYjrpobzAeyVmHK1WkSnbFhvQ

=== Sending Chat Message ===
Message ID: t1q88S88iC1ugcng3ig0N9Jp3azRz6dvNTp3vm6qwSKbAttKRNi8fEEjj0tANcMuLZKDlmdN8hjZ4os3j4bx0w

=== AI Response ===
Three products are available:

1. ekoDB Cloud: This is a fully managed cloud database service product. The price for this product is $499.

2. ekoDB Pro: This is an enterprise edition product with advanced features. The price for this product is $299.

3. ekoDB: This is a high-performance database product with AI capabilities. The price for this product is $99.

=== Context Used (3 snippets) ===
  Snippet 1: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Fully managed cloud database service product id:MnvqaOLriV1YRDi8k7ck_Er1nrr5TZ2loo1ge1nDuc8o0MpJ_l7UbIQn7ZEx0Nvv-Pm61sZiNwrQyf0E04Df4Q name:ekoDB Cloud price:499] score:0.1111111111111111]
  Snippet 2: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Enterprise edition product with advanced features id:ojUjjAKYXiqxpGG4toP70NhiDeVoLYPvjyPad3tlvr7CD2bCqCa1Ux9pmRKPUKoqv1B-7YEeI___JwBoSAYmPg name:ekoDB Pro price:299] score:0.1111111111111111]
  Snippet 3: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:A high-performance database product with AI capabilities id:QFtzKenzRcIFKHrKi0VeSOGTV0b9pLz3smDWA55WTSP0H9TU-bV1iR7P-pI_2JJADlEl6g4W2OQZnnQm5gVAOQ name:ekoDB price:99] score:0.1111111111111111]

Execution Time: 3041ms

=== Token Usage ===
Prompt tokens: 607
Completion tokens: 83
Total tokens: 690

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: BAfXzFTQQ1fyRUxlY_HZ0T5wLMlnNo2y5ZrgIRBDG8qIaDl9HssTKUdCrinchUSF9kJ9ZLJIY4Vhom3zZVrw_A

=== Sending Messages ===
✓ Message 1 sent
  Response: Answer: The available product is a high-performance database product named ekoDB. It is priced at 99 units.

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: s2T0G4G1vr7a8KXYXB9LCWQ2IOuZVaYaY9pScz9sUoIk2rj2xAZDojVFN85Nl1ShrUeGBKQICfkWgNm8LpY0MQ
  Parent: BAfXzFTQQ1fyRUxlY_HZ0T5wLMlnNo2y5ZrgIRBDG8qIaDl9HssTKUdCrinchUSF9kJ9ZLJIY4Vhom3zZVrw_A

=== Listing Sessions ===
✓ Found 9 sessions
  Session 1: s2T0G4G1vr7a8KXYXB9LCWQ2IOuZVaYaY9pScz9sUoIk2rj2xAZDojVFN85Nl1ShrUeGBKQICfkWgNm8LpY0MQ (Untitled)
  Session 2: BAfXzFTQQ1fyRUxlY_HZ0T5wLMlnNo2y5ZrgIRBDG8qIaDl9HssTKUdCrinchUSF9kJ9ZLJIY4Vhom3zZVrw_A (Untitled)
  Session 3: iFUlZ1cqvlI-L5mwayMbK6rRYZHtvP0C1ZkjPMznuqaHhq4fUa3SisMopOjLCPYjrpobzAeyVmHK1WkSnbFhvQ (Untitled)
  Session 4: hIhtRI8S4wACiDzJQuUxJAcNLdKhXI3vCzy7Zx2uvCBYmr-lHtb8oxO_AQ4pU8uXdwKlEqOb0Gt7PX7MWaCmww (Untitled)
  Session 5: BGziR8-8gkMr75-xFjh9ze6J3YD333YrHqF2Rj1S2EGv9Nmw4wg9zAVCZqH3ftp7Snp1WZ2xdzgRPwCwswUE_g (Untitled)
  Session 6: rWDdGI5jTi6Tm3bG4At0VPgu8m0mVWOpMz-31zOK6dkYgequ9swcbxoeAfq68y86Op-7mF-ee8-n4vDcw5fGCA (Untitled)
  Session 7: ckT-vsSlrRFV67BvKFS30PxVBECc1qDvSkEeAZIHWGmSDdCBnk96BxjiLq-EJhaQMWUZQ-4ePmY9X4BckHHBMw (Untitled)
  Session 8: xi8NGvCFxkY8run1C7czBm8Zusrp3ulWQll5bARwCYBvRo77ERcVKgiJZg_PaNTblnAnCPp4cNn3XbX42Fahfw (Untitled)
  Session 9: AtwBzakJFSuDzeoN5C86fY2ZlXZBQj1kjD1Tjxl0iHObhCP8EL4aRdenEGal4OwDAUBIP79dvRi523bjUx4vTA (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: s2T0G4G1vr7a8KXYXB9LCWQ2IOuZVaYaY9pScz9sUoIk2rj2xAZDojVFN85Nl1ShrUeGBKQICfkWgNm8LpY0MQ

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: J1NBTYJRQfNfowVBsBXX_0R2WfMfz10Ia0gvrsp_-Tx0zxkLcnbPq50veF1LMuYq3dfgHSBarNdogD87CMaZkw

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information available, one product that is available is "ekoDB". It's a high-performance database product and is priced at $99.

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
✓ Created second session: VsNDCHqfYmt3Nf653dwidqprl7O2ITkbnM1QbK71O_rGwgwCv8SHLolCOC7Z0uRjJIeYv3eqkk8KCgzjJ_9_Iw
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
  2. Score: 0.744
  3. Score: 0.729

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.698
  2. Score: 1.507
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

✅ Script saved: jtapenMzS7tAqtKp1q8z2hE60WTTAFwx1d8eH9-WzmNTGDQC5YoqUYPgaaHLTBmPgFg8ZoLgbJTC6P9fZ25h8w
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
Created Alice: $1000 - ID: mMSuGtLYnW1WqJPBST6grYPw-iV3ns892QljgLck4yVLAdQ9Faqkp5LWKBpUV5UgqnWAvtIDs6t4g21ejDd5QA
Created Bob: $500 - ID: LF65xWQokWhfWOCo9Dp9fcHUOfam_EgOkpiUKe_tATlbLxBCy16Yy9nhDrZCxGVjDCu03450xsuDWZph_g2F4w

=== Example 1: Begin Transaction ===
Transaction ID: a68cbe10-6d5c-4ffe-8f3f-1a9f01e4960b

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 36f1eb15-d590-46af-b9b8-c5b975fe33cf
Status before rollback: Active
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
✅ [32mGo client examples complete![0m
