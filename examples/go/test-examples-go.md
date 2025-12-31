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
Inserted: map[id:95hmCDBSwQLfm124cDJu8JVU9nYonr3aJZBdfm3W5M9yfY6ehtJozWEcZ6PRRdhpyuvgodFdF8S_eAgEONeqPQ]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] id:95hmCDBSwQLfm124cDJu8JVU9nYonr3aJZBdfm3W5M9yfY6ehtJozWEcZ6PRRdhpyuvgodFdF8S_eAgEONeqPQ name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query ===
Found 1 documents

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] id:95hmCDBSwQLfm124cDJu8JVU9nYonr3aJZBdfm3W5M9yfY6ehtJozWEcZ6PRRdhpyuvgodFdF8S_eAgEONeqPQ name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
[32m✓ simple_crud.go completed successfully[0m
[34m
=== Running simple_websocket.go ===[0m
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: rncnCNXtn9TifyjIRBuSS9NwEa4LkEujohxC3uUuEvKmgfRNVOk0eFL9shR9g3HdyKsv0S0RKI11q-cTGLTC6Q

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
        "id": "1keYPJvW6FB7hGaXsqhyCZ7CwoMvaiXk_VaFjue1b3Rx9dZwlmdq3_LXzLW6-GYcNI_dkQ0efAoScYSbPFuB4g",
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
        "id": "rncnCNXtn9TifyjIRBuSS9NwEa4LkEujohxC3uUuEvKmgfRNVOk0eFL9shR9g3HdyKsv0S0RKI11q-cTGLTC6Q",
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
        "id": "OzuHNu3HBCQJv4o_AkTX0qjzwIKg-qfalYHGh1k66UfW3VutE3O01Y0n_pg4otwK3O-pyXBVTeGvX6uut0kfyg",
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
Batch insert response: map[failed:[] successful:[psl2OvehUK7aA8F0QodILbJAieiVRFd0DzNUlNeEvVEey8OdkR_W1RnA8ZzaM1eWHYiJKWPI8kbdDbpRx7GoyA jdnm6CRuy7NNkfbbLZLTFhewDtvOgzZ1zjBOnPh2QYhPR8t5BFRIl5NTi9K_FpKjW5EdX0Fm3jsXMVNs98iSBg BWw6UcDH8TcpLf2ryIJXbvXbEX3YKr64E8LUfjIYt5qFQMisg39sby1AfI_NqJO5TTEL9TcvsIl47N_0--0hmQ pzPWFEGPHlZX-i16yT2Lo1RL9QMNtSeemiku9JslyU9RBhOktLHvNdow9eAzM9lkRCfePZun-w22jJ85yrZhGg dTM4Ba_aaKVVbbX5I0vUig2Rx6eAe0m-7UNOlE19tnXbKMNdbH_lTtfHV0BzIj5QeO1Zdb6_S_-5hnJH_q_1cg]]
✓ Batch insert completed

=== Creating test records for update/delete ===
Created 3 test records

=== Batch Update ===
Batch update response: map[failed:[] successful:[OZ_su3v-zBxR8FoYQWgF7CYD9VcIipeYahZzTKgg3nHzlIymMfFrNhkUTLcBbPz4csvQB88kbbyag8_4E2jHBQ _1C7jfbuaRRpoQ41s1y3rOBVt-44iHQBijAGp2TpwfedqysNK4N38jLX46K26tb3eWuBkNVNPxz9C2sLKXR__w VMFDa8gJaSv97kiFGG8id5xS1x4AklcC1ER9yWi-LVM-JNWvPDuD1tPnAhY48iww6q0QtpckUkDN1H0x3zAZVA]]
✓ Batch update completed

=== Batch Delete ===
Batch delete response: map[failed:[] successful:[OZ_su3v-zBxR8FoYQWgF7CYD9VcIipeYahZzTKgg3nHzlIymMfFrNhkUTLcBbPz4csvQB88kbbyag8_4E2jHBQ _1C7jfbuaRRpoQ41s1y3rOBVt-44iHQBijAGp2TpwfedqysNK4N38jLX46K26tb3eWuBkNVNPxz9C2sLKXR__w VMFDa8gJaSv97kiFGG8id5xS1x4AklcC1ER9yWi-LVM-JNWvPDuD1tPnAhY48iww6q0QtpckUkDN1H0x3zAZVA]]
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
Collection created with first record: v4SekgwbP6c2Fh-wnIUb21kBF_slzsQESIRzxGFZpedmBYwUIZ9FIPgZLk3VJ6IpkDKVrNOwQ4utjK1zcDJfOw

=== List Collections ===
Total collections: 15
Sample collections: [test_collection chat_messages__ek0_testing test_accounts schema_users_client_py demo_collection batch_users scripts__ek0_testing schema_employees_client_py ttl_cache schema_products_client_py ws_ttl_test users websocket_test chat_configurations__ek0_testing schema_documents_client_py]

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
✓ Inserted document: FjVvf0tV8MLadImR-t9leHQGDejGgz4IfoyldkNAUaBEVWYc6kTBOJPQ1lMtOQX6lUfs4ZeGkFeJCDwRQIPYJA

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: created

=== Query Documents ===
✓ Found 3 documents with TTL

=== Update Document ===
✓ Updated document

=== Delete Document ===
✓ Skipped delete (no ID returned)

✓ All document TTL operations completed successfully
[32m✓ document_ttl.go completed successfully[0m
[34m
=== Running websocket_ttl.go ===[0m
✓ Authentication successful

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: 5cDVDf4j1cEKomnuXpEvFC2IL6zpeBkusr1N8DPUCRhv2CPovhT8xdjM4UuFoAaTdOd85vHywfg0he-PMEENVg

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

✅ Script saved: K_NTH8GamaOMsKqK85fI0x1uuI9LrfRE7i2IpyUxhJ-pm9-IhRZRwIuZWL1SPwAHgCAmh3SzzxxVh6Mqa2YZ2g
📊 Found 20 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: K3dVs7AwV2ss4OiMbQAsgCVGKg5PgsRVCW9ZFXybqKm-Vnn8miSnGwa16d9wO9iDSqfNSSzS9Tdg3gjb26A7rg
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: INekY8BDC6KU05_1mnxX1I_ndowSNwINp2jacq7qjeD4hDEoLYd0NQq5GuFTCtV8RIUpKboh2fWGSuySwYoUgg
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
Created Alice: $1000 - ID: GoPktwtTzydm-aAaQo8b5DOqlFORBLNXn12uKF7nlj_MUzkNfl_J9WtLLFlCUZrK4x8_Uz68lJT7ZTweY8CeAQ
Created Bob: $500 - ID: r9Ud2dexDLqUlqjNJYaazv4cApZTo3EpUKThVbo0YnrAvbXVvSkaq5jV2VwGMvs7HxPhFHQstXbaPxwSQtFJvQ

=== Example 1: Begin Transaction ===
Transaction ID: 0c10a4d7-a00b-42ba-bcb1-127bc4426b18

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
New transaction: 7cb17e63-8848-4ecd-8457-5be3fd264bd5
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
   ✅ Script saved: skBPSt1LhTHkjzxy4xUQ-d__tslBRaTuv5ju1PRugSPFVx5gPY6du0KkvkKceoljwv6ycnLt-l5z8CVqi3E_qg

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: TYDAAOIqApG5_5O-ZX30XC8iBJ-fU3knRqGSpdosiJbcwQf7F43g2eWS7y_V2mZfuX_ptPwX3fzS-UiVaPWvOw
   📋 Name: map[type:String value:Alice Smith]
   📋 Email: map[type:String value:alice@example.com]
   📋 Status: map[type:String value:pending]
   📋 Credits: map[type:Integer value:0]

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: t9RDaRfycR71lNxTsm3eS2JVDUTPsDLGkg94qf3lzek_ITzpiysm47WaLyhgn7II6Unhq2TfY4TzswkMtSZs9g

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
   ✅ Script saved: tKnuP-gKD0T5uYvfwiRP4wBBFABNSgQkslKFMCtXE54aSUwFZJQMTMj1jPhh46cKtpf-_5NpifNo9plPhfLpOA

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
   ✅ Script saved: mOUZYPYNWEZpnqvLl0KVtnkvfOkuXl7UpS0AKkybLou5QaJne3wmcdlKO3VPx3VceS-t7SOlQcQw3uOA2TAGVQ

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: skBPSt1LhTHkjzxy4xUQ...
   ✅ Deleted script: t9RDaRfycR71lNxTsm3e...
   ✅ Deleted script: tKnuP-gKD0T5uYvfwiRP...
   ✅ Deleted script: mOUZYPYNWEZpnqvLl0KV...
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
Inserted: map[id:slrxuYPm_Qm67wR-vbxI6cIOMLMHMTyacDlZPkCE6gQ6snuxixNJPlgnYQZi7mik5TZaPdHcL1G3hheo6t3u9w]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] id:slrxuYPm_Qm67wR-vbxI6cIOMLMHMTyacDlZPkCE6gQ6snuxixNJPlgnYQZi7mik5TZaPdHcL1G3hheo6t3u9w name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] id:slrxuYPm_Qm67wR-vbxI6cIOMLMHMTyacDlZPkCE6gQ6snuxixNJPlgnYQZi7mik5TZaPdHcL1G3hheo6t3u9w name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

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

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: ptq7x_8Its9FgLFMmNN5NiKPqqEMcmXnmphNoie_5f2V8Sysdby14hE6DL76jMMqrEDKKyr9ky8wYsUhWK_NFw

=== List Collections ===
Total collections: 14
Sample collections: [test_collection chat_messages__ek0_testing test_accounts schema_users_client_py batch_users]

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: Wt3EBFdJtj848cNl36j5KMadp8GdMx_uX1a8YBXDkwy2OoeRs8nuT5lvbX_9Xxr2Oiglv-UfojiX9byYptgoLA

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: rl0McGcpha2Rxutc4E_3b9G3XSnnmMIj_6oDVT09HYCa7Z7iSdOd706pwc9PpjXpATx9fMkFU8XQ94PM_lO2mA

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
✓ Inserted test record: qDX7006IaMXpT9VeGt-WhYEAD4m8R6-l5kUEVILJ0rbXGZgu0P3zunMoubh7JsAyk-Ch2YQo9Qlpe--FiN5zoA

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
✓ Inserted document with TTL: 9gedxrLJaVItPs1XzPm1TQOvZs6FoqFfjO12PIGEuRrKa8YBZEO5u44TxFtccCvchWZ_OEAzyoNvPa7fjxlnfQ

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
✓ Created session: 821H3iSPIbH7vew7_PwAtAC9ISMwvMRCdEty66oC-5tcpXoP4JTJmI0I9vxK_eBCrT28f9HnLWB3bWMIGxHKrg

=== Sending Chat Message ===
Message ID: zhA4msUnMaxIUGA9kPK7ii1BErznR8KcUkIrDBT4_suFPAqaGrgxFaeD6ZW2IKs3daiPeBs5sDCJMS4AbzU8Mg

=== AI Response ===
Based on the provided information, the available products and their prices are:

1. ekoDB Pro: This is an Enterprise edition product with advanced features. The price is $299.

2. ekoDB Cloud: This is a fully managed cloud database service product. The price is $499.

3. ekoDB: This is a high-performance database product with AI capabilities. The price is $99.

=== Context Used (3 snippets) ===
  Snippet 1: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Enterprise edition product with advanced features id:zASVYNXNvSrAkMIcEPU-wYat1P6ek7HPyI9xS6xTm9D9EX0_cLGspFNqJ3Wuiu592lOKB_SjyLdUQa5qUztpaw name:ekoDB Pro price:299] score:0.1111111111111111]
  Snippet 2: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Fully managed cloud database service product id:z85OoP9kh8wY2Knfj01MxnZ8DLSR3PbXbZXlalq0q32qdVsh-VkpjGhf8KBSblDORNDQAgxchpt0yn051RB2Tg name:ekoDB Cloud price:499] score:0.1111111111111111]
  Snippet 3: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:A high-performance database product with AI capabilities id:m2oM6rxhHgF3I4lbBLgzwpTJoOYG6o453XegVAXIwP9stpJPW8jwYxvYMSZZDTXQvHrqASGBqOzfA2NUM5ac3Q name:ekoDB price:99] score:0.1111111111111111]

Execution Time: 2519ms

=== Token Usage ===
Prompt tokens: 610
Completion tokens: 83
Total tokens: 693

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: odv6MLCuEtIj3NUV-uSW6TTFYvnBemkIBtIuafULJVlO8vS_Zg-LszGuoGQcik32p3_p1cfQ2b2uCpQwXpfVIQ

=== Sending Messages ===
✓ Message 1 sent
  Response: Answer: The product available is ekoDB, a high-performance database product. It is priced at 99 units.

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: fRgne_XfPZ6R8iKg83YNhw3oq1CIqp2VtYwyqs4OU78-BMDtN82sGKcMeeKlSbNCtaqJIcoFa9GBasdBua1qfg
  Parent: odv6MLCuEtIj3NUV-uSW6TTFYvnBemkIBtIuafULJVlO8vS_Zg-LszGuoGQcik32p3_p1cfQ2b2uCpQwXpfVIQ

=== Listing Sessions ===
✓ Found 9 sessions
  Session 1: fRgne_XfPZ6R8iKg83YNhw3oq1CIqp2VtYwyqs4OU78-BMDtN82sGKcMeeKlSbNCtaqJIcoFa9GBasdBua1qfg (Untitled)
  Session 2: odv6MLCuEtIj3NUV-uSW6TTFYvnBemkIBtIuafULJVlO8vS_Zg-LszGuoGQcik32p3_p1cfQ2b2uCpQwXpfVIQ (Untitled)
  Session 3: 821H3iSPIbH7vew7_PwAtAC9ISMwvMRCdEty66oC-5tcpXoP4JTJmI0I9vxK_eBCrT28f9HnLWB3bWMIGxHKrg (Untitled)
  Session 4: rPOuQWVT2njcPDiEPMECJJCqn_dUz-waRLpPt-MeiwbHxylvroUkBcOh8x-E9Voy0mCLjYHyGVIQIQGvw2Fp1g (Untitled)
  Session 5: sQ2RiwmEIWTw9tgE9J9WJf_jbrC58oReuQS0RVCqqb_p9HQ2a4Yij2Fkray27UFWgq4S7lx9X_gnmIC8ySgDiw (Untitled)
  Session 6: aoe6MEuL3E1EmuO5qnfz0zmj9UCxpmR5TGic-qgUuC5Tr0b6ynRYl1hobYCeqXxu4wWURBibPcRgbY8-SCF3RA (Untitled)
  Session 7: Y9IlcOV2FYlrt19uHBT3nw_RiNqIdVs6MtG_7rOFYFLhEOfn0b8eJ3leVDMrFWKw2U_9gaJhiyPp1eiGoMGI4w (Untitled)
  Session 8: qBqsUD_jL1fpoveeqUPs3_yDNveMsqdq1TamMm0Xps-3pScyHXiEnh5jYsC0QQlaPWGE1YT6iUAx3PH-0JEQng (Untitled)
  Session 9: 9msyvD3h8zah5g8WdK2SPkPlQqqs_rbgazEBNOjmdoMUNl04wtkPKyhQLv3aRA1BGzDbOwuBnOMc5aT88H9h0A (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: fRgne_XfPZ6R8iKg83YNhw3oq1CIqp2VtYwyqs4OU78-BMDtN82sGKcMeeKlSbNCtaqJIcoFa9GBasdBua1qfg

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: wpgk9XzUe66icqRb5w-9MVGuzBHnJ1IoL_Ei7ot4YIN6P1PCvM0OvARhJtb7AteUIf2y8MNn7dcWflCckLaJqw

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information available, the product "ekoDB" is available. It's a high-performance database product.

✓ Second message sent

=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of ekoDB is 99.

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: EB4Z9QpAgykYsvdkcb07eqv0-Q3ZRYPpC_ElC-i-1fKANvdGOuiJjA7D77TFhWuN5tRHAGcJcQiYD8Bv8xSudg
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
  1. Score: 0.765
  2. Score: 0.757
  3. Score: 0.744

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.703
  2. Score: 1.506
  3. Score: 0.297

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
🚀 ekoDB Scripts Example (Go Client)

✅ Client initialized

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: edcIIR4w_Vd0dNefJnmRNF_H9SxJcNk-JYoWAosieM3nBr3fRbgNgGb4_KX4JRMtHV-XnNs-YaDl-tfdm_xuTA
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
✅ [32mGo client examples complete![0m
