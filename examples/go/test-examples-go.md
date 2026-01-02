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
Inserted: map[id:3xxI1lRQYw3aBWF26WSVYuenpuuRQ7-H0BHVeTaSWmA0FYqN2NbzMXX_fLSlZPLHivCvwkKVTLmnIzjr3xEo7w]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] id:3xxI1lRQYw3aBWF26WSVYuenpuuRQ7-H0BHVeTaSWmA0FYqN2NbzMXX_fLSlZPLHivCvwkKVTLmnIzjr3xEo7w name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query ===
Found 1 documents

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] id:3xxI1lRQYw3aBWF26WSVYuenpuuRQ7-H0BHVeTaSWmA0FYqN2NbzMXX_fLSlZPLHivCvwkKVTLmnIzjr3xEo7w name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
[32m✓ simple_crud.go completed successfully[0m
[34m
=== Running simple_websocket.go ===[0m
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: k9tk-YmXFH0UUqtz8QuYuwFD-TN6jz2k_5yl05LN9nc9CedFYB0vLe9HoCDg5OSk63FIcPzsj8lzTRppYmT79w

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
        "id": "k9tk-YmXFH0UUqtz8QuYuwFD-TN6jz2k_5yl05LN9nc9CedFYB0vLe9HoCDg5OSk63FIcPzsj8lzTRppYmT79w",
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
        "id": "DWtfhXvA14K_gV543gEE-ww8d0AntNrR6aknFBPOXwp_d3kvxo0T58fFXFS68odu9b2TN8gPAqPB1d44FLyLyg",
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
        "id": "j7cabfp3zf1fqbuWvsaWueFgmiV_kbeJ8-iQm7irQ6aYmr3XJKzZBVeOS5sytzHo_wrxas5BYrKHWjfI6lwWXQ",
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
Batch insert response: map[failed:[] successful:[k3hHu6WE7CAlgEpKpuAfibxR50CMbzn-w81My3K20cVCV40obxy8KW8x4Zg5h2D8lM3s2YvR9q5XXlQbd-8_cw FlCvMZMZaDnEH9YNhZJKiWoQailX_CQhXunmcHTULSGOk--or5ZLieOCv6MI7tXAfpGPmhJkIXh-Cm_pr2g1Og 6yo0uA_mBa1dfegGsxY3gADkXr3rLk7Ds54iPPgB7mLTKlmSFZw0rP2Ugwn_c0IhusqiHhsgWfh12vbMsZ4tWw zQ_qee-jgTYvvr2sw3V3gjjO38sGVs7V2sSmCk-V8OV7m9U_gDlrUo-sJKPRlsOYp-5KOSkQg15LvUfNaex96Q NDUrcSGQB4TXasevzx9DLBnWVWhC2_26ptIWSa8BUjO81_-GY7FMkV3TU2u28Yeo_xa1Fs9qCy2KWT1Qby9_-Q]]
✓ Batch insert completed

=== Creating test records for update/delete ===
Created 3 test records

=== Batch Update ===
Batch update response: map[failed:[] successful:[6rDq7WshBnqmol1szm_hZquIiHp1CCjgfo8tPLiCq1TuY1mQ7m1EeI8f0RD5I39LxA_5WyApe-C9b0K0R0kqWQ v7q0XAw4nk-P77qz3kVAO0Nyy_z8exr6DrnaxGIv8DMyoIC8WDmo_bGT8FFx10_gC1f05yW4VtlJqCIChFGN9Q D3RT7alBGyBwoTeiZ7v9n6L71h_liFhvm1rz_jzyM05kPlX2mrIFfpTLbIDnFqOI0JhBuj0_ZRsjzybXnfjwdQ]]
✓ Batch update completed

=== Batch Delete ===
Batch delete response: map[failed:[] successful:[6rDq7WshBnqmol1szm_hZquIiHp1CCjgfo8tPLiCq1TuY1mQ7m1EeI8f0RD5I39LxA_5WyApe-C9b0K0R0kqWQ v7q0XAw4nk-P77qz3kVAO0Nyy_z8exr6DrnaxGIv8DMyoIC8WDmo_bGT8FFx10_gC1f05yW4VtlJqCIChFGN9Q D3RT7alBGyBwoTeiZ7v9n6L71h_liFhvm1rz_jzyM05kPlX2mrIFfpTLbIDnFqOI0JhBuj0_ZRsjzybXnfjwdQ]]
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
Collection created with first record: u99tevp9y8hggtiAuKHDjZcDHR01_1vbnNPodoldEHE1JtaAnwIEq1SgSn12RYT_UI89r6DdiLs-8V5EKOv3lA

=== List Collections ===
Total collections: 15
Sample collections: [chat_configurations__ek0_testing test_accounts schema_employees_client_py schema_products_client_py test_collection chat_messages__ek0_testing users demo_collection batch_users scripts__ek0_testing ttl_cache websocket_test ws_ttl_test schema_users_client_py schema_documents_client_py]

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
✓ Inserted document: RhJTNgcNa8gwCS8S4i7JmDr2APJm0sfnxuo-fRO6HCHVlyMOsfGhYqR-R3l2HVjud8rUAZF6JeZ_dMqygbpiUQ

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
✓ Inserted document with TTL: f3wmThe3CgmFnT1LeeoSL12AGZ0ALrRX8pUz6zRUc2kwpvV9_8gjJ_OaPZkPYlKyUaWaua_rnt4Br59Nx-qCMA

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

✅ Script saved: nBMXm3UMAhR3TiMlEJydP7c2ltWmXBIIRR4IY4alOWs0-EQoAbKvOiovBrtpnjHxkiKcBpzT3u99MYhigj0xsA
📊 Found 20 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: UpwMHCSakhqhr_Hju-x24qqRIB4AGh80yK9V1yu7QsZ-Qb4RB5QhyBYR68VF8XtBc8kKqJP9vi7WW_GYYqmksA
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: GZTHSrkgdaPgC6Qdv9ZT9JFLqcf0JkZxSYjq-QtnVdafEnpK8up2PhFknJxHlrmIfrhOwacoRYOIpUBWtKOTeg
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
Created Alice: $1000 - ID: kIep5DgB8ZzCEoPl8uVCQ7WJ8uf1c9A-2eFyQGJv-on2G2a6TkdEP_DYinlmhInjATJPomj3jf1FeACnp8_Xmw
Created Bob: $500 - ID: c4Xpe-rIyo4hXMGwBN9BUFXKvNKopRV4PjkENlAOfJ5A3lGCuNzXS3A3Grecv40rtMKIscQk5973WfBvwGmTXw

=== Example 1: Begin Transaction ===
Transaction ID: eafbe0b1-d945-45e4-9f1d-72047a457023

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
New transaction: f68df0d0-866d-4927-9111-8be36aae07b5
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
   ✅ Script saved: EAKf1DN05ZVkTNF-8ZNI_6dURkUoZVXIjXPRTsGOk9sEAJMwZWcdz2wMkRiYZiy-6v-Xvm-EFdXhEPD8-RUzvQ

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: QXkZDMVyiMyk5V6sd60MEbmWpt3W7U5ZhgOyj-Q88PR3niCk6ldXqApDiMttrNRw97A1mHLt1BaV28xkCwvKpg
   📋 Name: map[type:String value:Alice Smith]
   📋 Email: map[type:String value:alice@example.com]
   📋 Status: map[type:String value:pending]
   📋 Credits: map[type:Integer value:0]

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: 4Y41eNvLghmWWoKQBWfmeDMcFS6YOJKLm3nGCa2LTlv8PIV3XiQe3aTBOXVd6fS31qlNI4lc3tz015sp2csmMw

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
   ✅ Script saved: ZjUbdV-INiaWabAzj1AZiOXH6RbRPE8Ne4Xys8JU4qmeRPxIoVred057yIy5upyl2v9zam1gHDtWOX0aY39aRA

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
   ✅ Script saved: AXKXfG92ZvMuhRNPOFLAbfGeJglEo-mZd2wdyx6AdKTbvUJ_PnVREamSHoXnfxn-df-Xp6iCmxNu0Eulz6XPFQ

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: EAKf1DN05ZVkTNF-8ZNI...
   ✅ Deleted script: 4Y41eNvLghmWWoKQBWfm...
   ✅ Deleted script: ZjUbdV-INiaWabAzj1AZ...
   ✅ Deleted script: AXKXfG92ZvMuhRNPOFLA...
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
Inserted: map[id:AvO1TtRjBbkVGOG0eJWe68_BqUMY5yIPwBOyq2-4hDcFZUNKX6tYndybcKkpEHZ_ola8HxaOjCa-aDo1OpPMEg]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-01-02T02:17:13Z] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:AvO1TtRjBbkVGOG0eJWe68_BqUMY5yIPwBOyq2-4hDcFZUNKX6tYndybcKkpEHZ_ola8HxaOjCa-aDo1OpPMEg metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Test Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:42]]

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 0
  active (Boolean): true
  price (Decimal): 99.990000
  created_at (DateTime): 2026-01-02 02:17:13 +0000 UTC
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1 tag2 tag3]
  metadata (Object): map[key:value nested:map[deep:true]]
  embedding (Vector): [0.1 0.2 0.3 0.4 0.5]
  categories (Set): [electronics computers]
  data (Bytes): 11 bytes
Plain record: map[active:true categories:[electronics computers] created_at:2026-01-02T02:17:13Z data:aGVsbG8gd29ybGQ= embedding:[0.1 0.2 0.3 0.4 0.5] id:AvO1TtRjBbkVGOG0eJWe68_BqUMY5yIPwBOyq2-4hDcFZUNKX6tYndybcKkpEHZ_ola8HxaOjCa-aDo1OpPMEg metadata:map[key:value nested:map[deep:true]] name:Test Record price:99.99 tags:[tag1 tag2 tag3] user_id:550e8400-e29b-41d4-a716-446655440000 value:42]

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-01-02T02:17:13Z] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:AvO1TtRjBbkVGOG0eJWe68_BqUMY5yIPwBOyq2-4hDcFZUNKX6tYndybcKkpEHZ_ola8HxaOjCa-aDo1OpPMEg metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Updated Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:100]]

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
Collection created with first record: FkmoxbzCM07N0I3p0rnXUHcvpxqCPaXwxejbnJ4f8ytnOsXwfn9-xI339R8v3CCTs8mKOX1hElGUGusy_DKfCQ

=== List Collections ===
Total collections: 14
Sample collections: [chat_configurations__ek0_testing test_accounts schema_employees_client_py schema_products_client_py test_collection]

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: 4RRCBVcdIY_HUVb_BCFM4EybFYP3vWErRXJcJZ1yXGU7L6tzLFOmjoY-gZ7oRuKA9dfUW957b71oNB8LZHXUTg

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: XqHIvOrIfe1EEmhFajDpCRq7qmognoFhPhi2gaPHFgFxi67WQkl_yrcqD43ZZTPIc_MFloF5KgqD70nx18Jl9A

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
✓ Inserted test record: Z-4Ch03_2rIBipGRZ0Xqo77SM167a9tFZnsX0GpzNugWsHFSLXThxhjfZwNQ5NjZ8OC0ibb4qbw_3TMg64jhqQ

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
✓ Inserted document with TTL: WUXhq1Kuhd0EPz5OukEsIeDLhuiDiqpZKP5r07s38fYd3S9Pm5N8QkvPbtQWx7ZnxpWCOd7tOHjXs6xWDz6OFA

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
✓ Created session: qoDX-6X-EBfv0Do0vMORjFhGHVQSswgRqwdUUCMAN2x5yVQgmQOYNWyMtCGoHUqEe4PlLZ-SEg9I12QvmH0d6Q

=== Sending Chat Message ===
Message ID: 6Upt9_PuI-9cQU8jRkLHuQ-DYH-IqQLSlLYxAx3QeOZSjCGmaXTmbX_2ZOrJ7B9pLNN4wTGrAPK2mV-yhS4z1Q

=== AI Response ===
There are three products available:

1. ekoDB: This is a high-performance database product with AI capabilities. The price for this product is $99.

2. ekoDB Pro: This is an enterprise edition product with advanced features. The price for this product is $299.

3. ekoDB Cloud: This is a fully managed cloud database service product. The price for this product is $499.

=== Context Used (3 snippets) ===
  Snippet 1: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:A high-performance database product with AI capabilities id:t9MvccvRyEY-nfBYy6leRQfcwXJFLHl8LN3NmbUiLMu5XxPL9ZhZ8Zs3QrfkMgt7prbczPfQDoHr-wzQkDoO1A name:ekoDB price:99] score:0.1111111111111111]
  Snippet 2: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Enterprise edition product with advanced features id:wP4GqzvLL9uIGv1PBc7njHnWtY__VIWmYbntisHAor-By_0SGl4M3WEuCMSyBBHWBGMXm1w78oGk9i4Q7BPuzw name:ekoDB Pro price:299] score:0.1111111111111111]
  Snippet 3: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Fully managed cloud database service product id:JLAMuFDGoNIy_osI4O-8Ur_lkXb_ZqN4WrQDMm2B87DF80zCgiuoYcNc_hFdswEBry8B_baIINdcvVkQWyJo2g name:ekoDB Cloud price:499] score:0.1111111111111111]

Execution Time: 2431ms

=== Token Usage ===
Prompt tokens: 614
Completion tokens: 84
Total tokens: 698

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: c0bJRkvhyl0UhgpqjcpLeUXm6CMmPku6setuwlybdrediS41ssgqp78SPrtXuwizD3T-K6F7HG9nP2HqRrjGag

=== Sending Messages ===
✓ Message 1 sent
  Response: The product available, according to the context provided, is a high-performance database product named "ekoDB". It is priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB product, a high-performance database, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 7Doh9XfismB5StD5za6S2p2ry_pU55iLEntT0CBWKNRL0_Hr1ODlVgQ7uHimsgVjwUv5ddxnD412RV_s0GVf-A
  Parent: c0bJRkvhyl0UhgpqjcpLeUXm6CMmPku6setuwlybdrediS41ssgqp78SPrtXuwizD3T-K6F7HG9nP2HqRrjGag

=== Listing Sessions ===
✓ Found 9 sessions
  Session 1: 7Doh9XfismB5StD5za6S2p2ry_pU55iLEntT0CBWKNRL0_Hr1ODlVgQ7uHimsgVjwUv5ddxnD412RV_s0GVf-A (Untitled)
  Session 2: c0bJRkvhyl0UhgpqjcpLeUXm6CMmPku6setuwlybdrediS41ssgqp78SPrtXuwizD3T-K6F7HG9nP2HqRrjGag (Untitled)
  Session 3: qoDX-6X-EBfv0Do0vMORjFhGHVQSswgRqwdUUCMAN2x5yVQgmQOYNWyMtCGoHUqEe4PlLZ-SEg9I12QvmH0d6Q (Untitled)
  Session 4: IrEDW5RVJSGcKFA6HYlzC3Jo45l1VQTq3iAf0nUXBShA7QxYasnCKR91wfvsZWK5sehGUT0fSMyCS8_ck9yeIQ (Untitled)
  Session 5: Qbq7z6JqiL85ZJejuPI55MMKxTTGgr7gDxB3N_FcD6e2OvrOabfKIMAFkSLjuM60ApEL40ENOiyaThC9cTF_uA (Untitled)
  Session 6: rtWqQ91ZdgT7AU06vsKV7rcjVfvC8s_aekKGnhDsMuSXt3hPn3DbR2FlL6POaqP5km1zzY-bcFb7-qL6A2CfNw (Untitled)
  Session 7: M5jfgFMLXAGf8KQ-Kc3oeMIk1LG1uVKrEIKZP_wh993kb2mGcllzluKQ5mV-o6qZU1qofEi4eQOSEA6BACYP8g (Untitled)
  Session 8: ipy1SsZHmJi1lwpkk_PUStFrqpVFSIw6gysmgKGyw0jZRdO3MHb5ubGGqXzRzjxFLn5IOhTi_xUNuuO_97A0hg (Untitled)
  Session 9: EIki62qqB5lU5m6nt01y8UfVL_LgF66otheWB_KKRJMELVCaNA2lRWOQuszEK7iX-FhyXi-qmmcIhK7m9S_YMw (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: 7Doh9XfismB5StD5za6S2p2ry_pU55iLEntT0CBWKNRL0_Hr1ODlVgQ7uHimsgVjwUv5ddxnD412RV_s0GVf-A

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: qKe6bPLCeEfXXjlitjYZXvYK54L3CRp6irfOfSoBrvOqv2aAgfxTX4IPhnKzN5Ed0XWwb4MUwbQR-99bCyYD8A

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information given, the product available is the ekoDB, which is a high-performance database product. It is priced at $99.

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
✓ Created second session: 5vwqgmmAnv3Y-yS838JrSTCyWsphUC6GJk5-CZGtvDfoJJi465Tv8LranAB4M2-uz91CF6McbzYnfEJnizDGDA
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
  1. Score: 0.777
  2. Score: 0.766
  3. Score: 0.755

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.711
  2. Score: 1.507
  3. Score: 0.302

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

✅ Script saved: _6O7fltQb3S6AeCL5v8Ds06mjcccvKpl7y4M9ddmdDRCJLVjW797hDkhVg3nGsfGsOavAzmLYEIxuYlRRWH5Zg
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
✅ [32mGo client examples complete![0m
