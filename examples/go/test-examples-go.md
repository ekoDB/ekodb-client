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
Inserted: map[id:Srf8xU0B65x6pUuddxH29_2J5ppYHBSgNTZOMyhvs4hOClbR5ba7jlTh501uGnjA6OysunJviMuLpcS3zQq1Lg]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] id:Srf8xU0B65x6pUuddxH29_2J5ppYHBSgNTZOMyhvs4hOClbR5ba7jlTh501uGnjA6OysunJviMuLpcS3zQq1Lg name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query ===
Found 1 documents

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] id:Srf8xU0B65x6pUuddxH29_2J5ppYHBSgNTZOMyhvs4hOClbR5ba7jlTh501uGnjA6OysunJviMuLpcS3zQq1Lg name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
[32m✓ simple_crud.go completed successfully[0m
[34m
=== Running simple_websocket.go ===[0m
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: j7IBJs1nREsgu7DnEMCXSpYq6P2tC4E6brhLBeEkUZezwJw1xqAQ4FybRLrm8to7RsLAZdjORHzug55oAO38eA

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
        "id": "j7IBJs1nREsgu7DnEMCXSpYq6P2tC4E6brhLBeEkUZezwJw1xqAQ4FybRLrm8to7RsLAZdjORHzug55oAO38eA",
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
        "id": "EHEYTwN4dvGdPiFjeN68OBYQo7RXh3x50hrjyfNlRu14LvgPwHcWDqQ-lzEqpzxKv9OGmvlLZReOEVa3sk9Szw",
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
        "id": "WaBLKHHaUr5EzU2ATdkHLvXtzFbO8-_txXCCsCgLbh3hqPL8p-jLVoUzdHAgJhCQlO1uSOqvXl_I5gzOCfGMxA",
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
Batch insert response: map[failed:[] successful:[NMlMCZIZ9AyBHZIwJmGIN6s-6tsXrwdSsZctZLyvDvvqiLitKD6iB5_P7uDMBwVldeTXKxV1OaxgsyLDebSXHQ NqVPSjxR_elaApI8Y1Myz7EdpedoQEdzNqGSz-qYbGaFf-PMJUYy5VtcPzYX9L-BdJUVgzLQk1dFFe9dmhMGkA t9iZkXw6MEZQ436WY6obDh9hW5kaJCb3VS_AdS7T3CpiooI2XH7WAPuT5i-24nijxVDOtWGdFqEziwQnDOuiiA vCGEVzJqyLo_cetnxB-gczlah2xUmd6KLcp_knNlgVANOXpzeIw5aWy73kfrBIE3TJ-pzvX6SiaKPgh9a04hsQ T4fxUf4v3JISzmwKfeLlTA3NW8U_ysEyROGo5kokTnlS3_KavFaXUNAIGFn9Xbzi7oJYkoVpaK2mK-2d_0vk9g]]
✓ Batch insert completed

=== Creating test records for update/delete ===
Created 3 test records

=== Batch Update ===
Batch update response: map[failed:[] successful:[xbyqDWW9N5P61BFdxGCCJqHB-zNZUFSpjtkpjpsSyaUvxSO3qKBhUqBM1B-aG-fFuIFapwE3Iop67SyZpGxMIA A1cGkBc9wEp2ywcSqNiQDGAdFJgHPRBddJOodW0b1QMCIgUxXmgw7xT9dIlInlcpvXe15sCaOW0OH0cM4O2Gkw O5IR7BHR_ZEZlgJzbPFeBV4TVLApX7d2pwZfdkxVe6mCudoSkgMo3uPyZ7vdSLtTSfsXTfBeLgGUiSNIMuYMSA]]
✓ Batch update completed

=== Batch Delete ===
Batch delete response: map[failed:[] successful:[xbyqDWW9N5P61BFdxGCCJqHB-zNZUFSpjtkpjpsSyaUvxSO3qKBhUqBM1B-aG-fFuIFapwE3Iop67SyZpGxMIA A1cGkBc9wEp2ywcSqNiQDGAdFJgHPRBddJOodW0b1QMCIgUxXmgw7xT9dIlInlcpvXe15sCaOW0OH0cM4O2Gkw O5IR7BHR_ZEZlgJzbPFeBV4TVLApX7d2pwZfdkxVe6mCudoSkgMo3uPyZ7vdSLtTSfsXTfBeLgGUiSNIMuYMSA]]
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
Collection created with first record: YrlJPyd7kLnx-hTkOiK9TWe-V6pGZHTkBNvl_-6t4pVW1B0PHw2CNpNt86MAR7SOZ_FsvGcXBy_-2L9TT4lpqA

=== List Collections ===
Total collections: 15
Sample collections: [chat_messages__ek0_testing schema_employees_client_py test_accounts websocket_test users demo_collection ttl_cache batch_users schema_users_client_py scripts__ek0_testing ws_ttl_test schema_documents_client_py test_collection chat_configurations__ek0_testing schema_products_client_py]

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
✓ Inserted document: 27DLJw3wrAyuuYlCr_wWZaMGrZvITjktmoSKFKuDYxIUqjjaldbni3epJPF7WujXaCwIzCED7l58kTJik2afjw

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
✓ Inserted document with TTL: JCFeQ4bm_Z3eoBtmdneSqmNZu3Zl96TeJGV1xfmkI4RCfEtlI2gN_pb8yZWaXJWwfjoSi1u76fBIvusgHBkCpg

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

✅ Script saved: IPv88Tsz-FR7bl0276EH1uubbHMBRqC2bmkaG2ofo2fioC2uXF3dG4hxeWkMIh0UB-Ro14_8QN1spDF2QUAU2g
📊 Found 10 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: gO5YuNIlsWiLsoXUNk-z96ZLl7iiGKojy4-6mYRgHaDTt6d2-SKr3bz_wJOtGs5I4W08XKha2yQB6xaQWh9y2A
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: mEDRIqzTC53tgm_33Vdf5lGpPDi2xEhYsVkvT3BKmZD1AjF4icsTivMgc32uPayMYfJpTorrj0KNGCHmQSVmiQ
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
Created Alice: $1000 - ID: mD6GBI2l0uSkusnC3VmpxtSOIDUJjHnopiekXKBP17BjoAHHUswJmDFCyGRqMiPzZDmMXXleaWNKT_SrCHCHEw
Created Bob: $500 - ID: a2QGbqlxr1tdM0qH0RaIHBNrXiTowXfJeYjQd6bVh7q51_8jN4Ii6IvmbQq244dh4v5FkZC2dhokw4rfEgdfcA

=== Example 1: Begin Transaction ===
Transaction ID: 7ef777b8-02df-4700-933b-5ad7c53692ef

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
New transaction: 911d4c30-a9fb-42b6-b4e7-fc884b56024a
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
   ✅ Script saved: moeATHaEnR_jFUt1n-8NM_fcOxOvS0eHN5yEQpyULoD1wYimXBxXApgAYayF55-aq4jZdGJgZ48x_ZsTSgwvlQ

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: -frE_T8AjNM9udnplLa0uIBTVxCJp43X4MXdAirCg91W8jqYrogTqSK1OG2zglKD3I-AUSFFPU55I3CnbrYuSg
   📋 Name: map[type:String value:Alice Smith]
   📋 Email: map[type:String value:alice@example.com]
   📋 Status: map[type:String value:pending]
   📋 Credits: map[type:Integer value:0]

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: yFZR8EqU-of6SYIui1iA5ypnyTyZ-pGuStH_DRdlg8YEv4CmhG_FWx4qWICB_rrkDFwRdrj5Q3Ec6XLIAQeNSg

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
   ✅ Script saved: 0sy-u8eutDH7p8O91-rh9uI37nxzqkVPKOXfcyLDs_pBY1JVgVPh2sQ9K2aDMD_VkXVokEr-bP7fUq45z2Kbrg

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
   ✅ Script saved: Y6eDtY4i-6dNbBdQQMXY7GdlE5EkXc1RFkzIQACtHMSwNVuESIh_dNOJLn-09yG8eVmGQTwF66FxOQk-ScyffA

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: moeATHaEnR_jFUt1n-8N...
   ✅ Deleted script: yFZR8EqU-of6SYIui1iA...
   ✅ Deleted script: 0sy-u8eutDH7p8O91-rh...
   ✅ Deleted script: Y6eDtY4i-6dNbBdQQMXY...
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
Inserted: map[id:9EW1XYD_1zPSmuaQQ5dvab8Li8kaB660ewnigC1-Yq0THgwcsCVbXfCgAZSsaF8h0LevksukuMHEcqS3qld9Mg]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] id:9EW1XYD_1zPSmuaQQ5dvab8Li8kaB660ewnigC1-Yq0THgwcsCVbXfCgAZSsaF8h0LevksukuMHEcqS3qld9Mg name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] id:9EW1XYD_1zPSmuaQQ5dvab8Li8kaB660ewnigC1-Yq0THgwcsCVbXfCgAZSsaF8h0LevksukuMHEcqS3qld9Mg name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

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
Collection created with first record: RfGMLBx7DNi6V56wfHPH1BCSGS0VNuPODXSytpc_i1rhUWqKsw4PaHEuZNpSwWwryZlFg17XLMG6UGKgq6GDEg

=== List Collections ===
Total collections: 14
Sample collections: [chat_messages__ek0_testing schema_employees_client_py test_accounts websocket_test client_collection_management_go]

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: zWSceGGctrWBpmL6LiQ_TYbiaYc-o4xeZ9Vu1YdbIg8fyI8f-eQK38E_QGItlo8-ZTtroYV3_tfS0FiIkf7J5g

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: yMamte_3mXvAIZn74fnOYJwXyOLf6_atnm6fk2oyxrC0UGi_nIdoif74dFfxbAMLjyjKAHdob_wv506V_pfQwg

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
✓ Inserted test record: eezGFt8BgRvOnCNpbfKpGw-JJMaAzgCQQl2wEvq8OghOAVuABuneseG2G_igSheon3Y8P9I4WLL8TXE7dX-6bg

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
✓ Inserted document with TTL: bD3TqpEKNA29erVRG-83UgnnA6AsfQIlMhCtcmCG2lDcVlfJWMU9SmXkbWlNsnNh9ai1eVbW_6SZAnBa5xfwBA

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
✓ Created session: z_7GlQKFQxUqZgud8O5V6t2cg3Hy7zRwtLJScBJKIHkYz_5kO1_kr2g2VlrUXZxXY57NZ8lIMjiyZjlVUh6ViQ

=== Sending Chat Message ===
Message ID: jkGrFIkcHTSiOMt2w35xDC_SFlCx3fC3WUTRkjMtxWdyLfvFj1sK2GWDiQP57KC_nb4mORKG5AUbXkZ_-QEVNQ

=== AI Response ===
There are three products available:

1. The "ekoDB Pro" is an enterprise edition product with advanced features. It is priced at $299.
   
2. The "ekoDB" is a high-performance database product with AI capabilities. It is priced at $99.
   
3. The "ekoDB Cloud" is a fully managed cloud database service product. It is priced at $499.

=== Context Used (3 snippets) ===
  Snippet 1: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Enterprise edition product with advanced features id:Rxtm7ynYLLVbYkIvD3zAnsiEPCo9IN9nwYZz5xXS5QIItVz_CNucLJXSZe3uAOCTv8DqzRR2H7N5yCG0BGH88Q name:ekoDB Pro price:299] score:0.1111111111111111]
  Snippet 2: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:A high-performance database product with AI capabilities id:69z3imyqNkZ7FS8WXV3aKZw5Yyv6KxgQMdyfiOKR1KtIGa8PQiIPmgRVe26mpCdpBcnc93UQKy5jDNOhLaWLQQ name:ekoDB price:99] score:0.1111111111111111]
  Snippet 3: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Fully managed cloud database service product id:NIzt1rzKJjzUxYbcV4-0L4Jf5ccZzMOvRG6vZ6v7idBFrZWwNOTSQE57b48zCUDNuHVYABjlFMPtu_b-KRpmzQ name:ekoDB Cloud price:499] score:0.1111111111111111]

Execution Time: 2431ms

=== Token Usage ===
Prompt tokens: 611
Completion tokens: 80
Total tokens: 691

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: uG0aZNV03fdA8F-6QPZsbDlooeIPE9qSmbi36dJenCU-ZuXi8kB8TgCG-o2HqgNdifFgXJrkVQs1qHzfM8H01Q

=== Sending Messages ===
✓ Message 1 sent
  Response: The product available is ekoDB, which is a high-performance database product. It is priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is 99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: H5JADVSGiyqV1M5FiiMK6h0luD_XeAUjfi-2rG6Q_elis2Lp-kuc4QzgK7nyhBKw4_uYvGw0cv2d9fJtVEmy0g
  Parent: uG0aZNV03fdA8F-6QPZsbDlooeIPE9qSmbi36dJenCU-ZuXi8kB8TgCG-o2HqgNdifFgXJrkVQs1qHzfM8H01Q

=== Listing Sessions ===
✓ Found 9 sessions
  Session 1: H5JADVSGiyqV1M5FiiMK6h0luD_XeAUjfi-2rG6Q_elis2Lp-kuc4QzgK7nyhBKw4_uYvGw0cv2d9fJtVEmy0g (Untitled)
  Session 2: uG0aZNV03fdA8F-6QPZsbDlooeIPE9qSmbi36dJenCU-ZuXi8kB8TgCG-o2HqgNdifFgXJrkVQs1qHzfM8H01Q (Untitled)
  Session 3: z_7GlQKFQxUqZgud8O5V6t2cg3Hy7zRwtLJScBJKIHkYz_5kO1_kr2g2VlrUXZxXY57NZ8lIMjiyZjlVUh6ViQ (Untitled)
  Session 4: IldEnBWbEmtuzluqWP2jIEkU5fEl-tkgvYm_Ws_4Txx-zvPQ4p6OrzVY6M-ejEo1hk1UOA5yZ1ceJxqIT73qgw (Untitled)
  Session 5: 098hQxXsSWrfeco-Rh28tx65AW2CPFqfirZRpCKkdrdI-eH8o4gZ9XROWCgCRUEiFNQphpO4aCi05xZJ_pgJMQ (Untitled)
  Session 6: KQOalY4bPJPqNB915OmVLg_bRHONknf6mt2pN34QtbgXg1de6yTko9Mp822in1Pivy0H5-NxZjwyBR7wcoMXTA (Untitled)
  Session 7: OUSEtgQ_-7kKWGMt4AmVYrdXPPGik3263KCnNt9ueJAKP1k4kK_PUO1YW4JlGYF-TG2Tmcwoh-qhMRAInCKHkA (Untitled)
  Session 8: G4DhRHz9-8gw8a7-nQBcBLd29RZmU5B9ERthrec45Dhw07EDfqLMUnzuBtpz7-EnRrin2N2AEJOh2lhgCH-HZA (Untitled)
  Session 9: LsrAmQyiFnof-6HfYut53SABCHrDvt2bS-ATTomrdt03ouabmWG3DHX-iGnYDsyVIYmH3pxEdrdeqf0cTWaG8A (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: H5JADVSGiyqV1M5FiiMK6h0luD_XeAUjfi-2rG6Q_elis2Lp-kuc4QzgK7nyhBKw4_uYvGw0cv2d9fJtVEmy0g

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 6pU21WqjNlKykauPF9PAq6ddIYJmzETBxXcxAF6Vpv5psofFrYHQFcf0dcpuxo9Ro9NGKWCsOOmOvqbDkYA7Sg

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information available, one product is "ekoDB". This is a high-performance database product. The price is $99.

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
✓ Created second session: 8Smsmom8pjVYdUP71Gk8rl1718m-bz1nU6XQAhrym_9JEzS93wLb8F4WJj2lZNmy2ODc5AOevjWB7nsXfC1AzQ
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
  1. Score: 0.764
  2. Score: 0.759
  3. Score: 0.739

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.695
  2. Score: 1.504
  3. Score: 0.305

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

✅ Script saved: 54v4H1jlRuPVW7yUUPobXOYc1l7HnVW85u_DTqTOWQFsT4iEnZmMA2JeNY_TybW7w2dkaKD2dIOCUqquR3nRXQ
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
