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
Inserted: map[id:IAdTinjF5zvcvQbUt1DCvE9EQVsW46mBjaveZ4GCjE4vlsX1HTDNg-H3aNXWWdDXwPPUIsultVfVCjsQ4mcW2g]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] id:IAdTinjF5zvcvQbUt1DCvE9EQVsW46mBjaveZ4GCjE4vlsX1HTDNg-H3aNXWWdDXwPPUIsultVfVCjsQ4mcW2g name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query ===
Found 1 documents

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] id:IAdTinjF5zvcvQbUt1DCvE9EQVsW46mBjaveZ4GCjE4vlsX1HTDNg-H3aNXWWdDXwPPUIsultVfVCjsQ4mcW2g name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
[32m✓ simple_crud.go completed successfully[0m
[34m
=== Running simple_websocket.go ===[0m
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: rOrZMWqDh7WWVTLDxIikzWv8TgnmEIw4VFgPZB6WZt6mnTipX67B6IVw3iRtb_yijeZztQjufYQyn0AN2m3wDQ

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
        "id": "rOrZMWqDh7WWVTLDxIikzWv8TgnmEIw4VFgPZB6WZt6mnTipX67B6IVw3iRtb_yijeZztQjufYQyn0AN2m3wDQ",
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
        "id": "uNNfPR9zCQ_Wj1E21yvsz6eduv194YhIWwL8E2fBMxqSqmMVPCRQjSoBSn6ipWjaTp5S-QAoqOl_dwGFXKUdCw",
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
        "id": "Kd7MuNthn11mq4fPbeELKVfNy-FciWdlj1aoIA441hvPMlKU0ZCKc0mBikFpaf1fbEYf6jXXevgjFsdeggqXNQ",
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
Batch insert response: map[failed:[] successful:[382IwybyM0tvE0YZUIGa7nJEGoCx-wvQCrBJh41VwpsY2JNi6wq_upbRKjfjeYEEreUCKnwHhgBrRLEb1nKJGw 8B-PM6dxkmSXhPyRSW3Q2gRSmNTtpA44bZHqMogIE09IVfE20b4qkx6zHzOtgZbFhePxRe6KTp8y_jbhCe4DDQ L1cNlX9jycAiwPvw3Wo9arLWAj-yUlXXV1clUJko8B0gfoS3xXe_yW6gJYX5A3_wNY8PudMgDmjTJCcPBQOV9Q 2kLrHD-2FEXT3dRWgt8UJMkmcpdBE5tIvECQVphaoOJebhNgH1U8Z_hsLennWKRA-Ipo3L7fleRKwziJHbbrLw S_pR5qXm4i8mt1h-5DBzvk-wH7Ni675zFEinjC7NgMcmrjVWiZlS6aUHxiFMT0UVJO8pTl22X7s9OH9hdE23zQ]]
✓ Batch insert completed

=== Creating test records for update/delete ===
Created 3 test records

=== Batch Update ===
Batch update response: map[failed:[] successful:[xgDfSJG3zpm_HcvsF-BNjX5eN52_LEQ_9QTsD4zom6Wu_MrfS9kl4Bi-pYU1vi-L1IOwoH-iZbUeOq-78BhFeg 39Z1_xvwY1XVNdp3xU5-9_5ZetXcfhNODyJaJ059mR0yyGnxoifNK-cCyT3ZU4RGNxllhMhQKJkRljHCGtx4Zg 97keQXbfeIy1JKfvWHCNA2G79HEvbVGvFyOrZxZ4POkMn7P8_2Ua_qDdyjUAqCBJ1SqVMFjP6SXhOiNVIw48EA]]
✓ Batch update completed

=== Batch Delete ===
Batch delete response: map[failed:[] successful:[xgDfSJG3zpm_HcvsF-BNjX5eN52_LEQ_9QTsD4zom6Wu_MrfS9kl4Bi-pYU1vi-L1IOwoH-iZbUeOq-78BhFeg 39Z1_xvwY1XVNdp3xU5-9_5ZetXcfhNODyJaJ059mR0yyGnxoifNK-cCyT3ZU4RGNxllhMhQKJkRljHCGtx4Zg 97keQXbfeIy1JKfvWHCNA2G79HEvbVGvFyOrZxZ4POkMn7P8_2Ua_qDdyjUAqCBJ1SqVMFjP6SXhOiNVIw48EA]]
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
Collection created with first record: 6UfpxQiASc3Ow_UTkXqXIvEVt3_Y42fsaQIm2bEee5O9o63pzgoRg7MQUOaQA9G5cvc-nGersLYt_xYbWJM9bg

=== List Collections ===
Total collections: 15
Sample collections: [schema_employees_client_py batch_users schema_products_client_py test_collection schema_users_client_py chat_configurations__ek0_testing users chat_messages__ek0_testing scripts__ek0_testing ttl_cache ws_ttl_test demo_collection schema_documents_client_py test_accounts websocket_test]

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
✓ Inserted document: BBoPo9m7BA_iYXZ_TNO7WwlSOyqYR8yY047BDRbyoEXx1W8p8qQynBfcpURtp3HICNwsL4UQYSTyNrF962UCHQ

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
✓ Inserted document with TTL: 0FSOpIc4KNAZ9JZtKUCzvPLjyp97nmXTyecMiYyyWMOifZoKhZX5XWhpF4vGEds8V5dRnZZyEBZEdVKEIU1QyQ

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

✅ Script saved: RIzRGTCJyqlPup7GrteMGxpvuRsf5bZHrm09XcQZkUNcVi0Tv8FU6S7K2onT-t5v-fbWzZgpFbXRVUGHOlPSvA
📊 Found 20 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: DP8A1qSnaEunuP4U4Wn41TjFrxS_BII9r1WMQmehTa_z9mmIxMK0HBfCWFGmd6K-3zn24YGItdz72QDN4pQgIg
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: 8sft0dvqtlAoqs9x4aFHvimisLL6cjk8HappHS1YWqlG9zzllflP9ekxmnwLS667LL86v_QlA1ApjeBxaQX7yw
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
Created Alice: $1000 - ID: cb3s5hnZZbg7T93Pf99JYYnkFjZexoaABhkEiNvjKjEQPKf2jv1ACb98XmY5-g6EjqQ0AU8ZyyKdMrRWXUIIrQ
Created Bob: $500 - ID: zAu2NbeZmXTIkGI2RJBERuZ9iF0UyX6x4LKqj-PnzIfcFu_OSe1Z6QGpOAKIYt9li6jX4EOY-kdSDuiKEmMzFg

=== Example 1: Begin Transaction ===
Transaction ID: 1ed1cd29-55ba-424d-9e14-cc59ee9081f4

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
New transaction: c76867b5-b1d6-40df-a785-18ceba8806de
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
   ✅ Script saved: y69Tgj7y5GsPx1EJ07IDhjFYA90xsrQ-rNtMsSQzWgPSbA7-QoVc8oFOf2ZiNtoZGypXWLBstZSP-3zP_Rfkvg

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: MAc4tIxWK1kkaQXf800kOlwCei7VJR-GyqwCNqnPbnXvEepzTAT--M2uOr6HgHGYhIQGU7ftJVD27dQ1truXTQ
   📋 Name: map[type:String value:Alice Smith]
   📋 Email: map[type:String value:alice@example.com]
   📋 Status: map[type:String value:pending]
   📋 Credits: map[type:Integer value:0]

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: PuhJEte6F3K-rFRrmbqdm_jPURMB8-gPVICYMG2Adn_Tdg2eJ4cVdoMyvP4oHR65AgySDZqzxeyJkue9oYe7TA

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
   ✅ Script saved: z02mFxp5uw5rUyEdDgDcGOciwQ12pjU4ODlpww6MUdK3cwu8zUdtFagJFdt2ygCN50YH4Lrokb8UM-eVFrlaOQ

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
   ✅ Script saved: dEnGrokF-NjwG9LhIxq0ZdIlgkIabaQ9Hr8y3huht2GcZvVAsnAOPu8n_aMHE-JcLuKewIUJB2k17SHSyq7fMA

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: y69Tgj7y5GsPx1EJ07ID...
   ✅ Deleted script: PuhJEte6F3K-rFRrmbqd...
   ✅ Deleted script: z02mFxp5uw5rUyEdDgDc...
   ✅ Deleted script: dEnGrokF-NjwG9LhIxq0...
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
Inserted: map[id:wd_LDAm2W9a8YXPclgme-xLeyaXHZ2vuqxhhHML1gfZjK_KGwf3TgdDAKz2_p2YbASnc7Qu-18tpNGXaXI5RDw]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] id:wd_LDAm2W9a8YXPclgme-xLeyaXHZ2vuqxhhHML1gfZjK_KGwf3TgdDAKz2_p2YbASnc7Qu-18tpNGXaXI5RDw name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] id:wd_LDAm2W9a8YXPclgme-xLeyaXHZ2vuqxhhHML1gfZjK_KGwf3TgdDAKz2_p2YbASnc7Qu-18tpNGXaXI5RDw name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

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
Collection created with first record: WcYDDv-X-lGtUNcCC40QLNYn5Nd_M8wpTRJ-0lsu6FwM0kPby9Y7ntor90ord_QoRmqSpc9UvgMjnbWWOWGXpg

=== List Collections ===
Total collections: 14
Sample collections: [schema_employees_client_py batch_users schema_products_client_py test_collection schema_users_client_py]

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: gY6qel2SErD-lp8OwGcomm1xW-YoGJnhP1wApQqXXx55MNUcnpa2UOE0VjFGFqCqiwKnroChdMxFPmccPSNZDw

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: ZKxIlbRJOiUORXYGdXMsr5Q8xL7MGTNPFCEpL4u7FT0r5EfclSuFL00rgXZAS7fNUc1-g18_sZvmjmFzCq5x7w

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
✓ Inserted test record: RHHLBNb8Er9CwnXKCJyQwGMpFdlPuv2Ln_wWFKVdsKDyXkWdWDq5KUC7dd5pnJefZ8rOEwlR63RJSUo483IojQ

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
✓ Inserted document with TTL: 9yW6-C-d-p3QDRZK8JJyVzxkcMH4hkdohgRSIhyN0nW8d35UmVQWkM5JumyONfpO97PgHrCiJ7-KUsBHfzfkuA

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
✓ Created session: v2sHPzLi65qdQ6hf-Js9BfbjCdw237_ldwjv4O9gOAJak9wodMbphJdBNk3fkb5WCgpDQ8I3EbhOABOA8t5SbA

=== Sending Chat Message ===
Message ID: zHOdZh4Xf_TbI6h0OvjVFJ4lCasHmUypSRuYBY6cwFMLxqM2RnSCoftv03fWh5zIBmYOYMumdopw9EK9CeurMA

=== AI Response ===
We have three products available:

1. ekoDB Cloud: This is a fully managed cloud database service product. The price is $499.

2. ekoDB Pro: This is an enterprise edition product with advanced features. The price is $299.

3. ekoDB: This is a high-performance database product with AI capabilities. The price is $99.

=== Context Used (3 snippets) ===
  Snippet 1: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Fully managed cloud database service product id:diGrfUFKemLJCcvuCi8L6dO3DjkKLkUGqxrCcvjd3UXXfvxBxX4UkR4qbkApmNkTSDvyP1Lj8PcChti4sLuVZg name:ekoDB Cloud price:499] score:0.1111111111111111]
  Snippet 2: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Enterprise edition product with advanced features id:V2YPOAX9lRUVMlrf3ATGwBZNpGoQ0nTzlQoYrYjfofZ1xDOp2lmcw9CQV-9Z3CMDttTxIHQWj1bM-0o4Max5gQ name:ekoDB Pro price:299] score:0.1111111111111111]
  Snippet 3: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:A high-performance database product with AI capabilities id:FLK5ygejihitT6yvxzN37uPUR-jfuI6tNqkoxKH1oYjwgpio4brAf9ZlSBZ4e7UTjNPYIIbeH2xHVSEnJS2Gug name:ekoDB price:99] score:0.1111111111111111]

Execution Time: 2241ms

=== Token Usage ===
Prompt tokens: 613
Completion tokens: 75
Total tokens: 688

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: Nl1MXe_ym3CqGP0-zV4n4927SZunPuOrsjYNzqtKLYRSh1xVU96TXy7VOfn9nmXIpAN3UJcY80zW23qzl_fKhg

=== Sending Messages ===
✓ Message 1 sent
  Response: The product available according to the provided context is ekoDB, a high-performance database product. It is priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: H2WPCLfx3tnaOKw6qNE9eNJJdPtsHwxXCpYHoV-f3xt7el2rQFGELa6I9HJZThmNQ2UpfAh9ehJz8_ec0-iu3g
  Parent: Nl1MXe_ym3CqGP0-zV4n4927SZunPuOrsjYNzqtKLYRSh1xVU96TXy7VOfn9nmXIpAN3UJcY80zW23qzl_fKhg

=== Listing Sessions ===
✓ Found 9 sessions
  Session 1: H2WPCLfx3tnaOKw6qNE9eNJJdPtsHwxXCpYHoV-f3xt7el2rQFGELa6I9HJZThmNQ2UpfAh9ehJz8_ec0-iu3g (Untitled)
  Session 2: Nl1MXe_ym3CqGP0-zV4n4927SZunPuOrsjYNzqtKLYRSh1xVU96TXy7VOfn9nmXIpAN3UJcY80zW23qzl_fKhg (Untitled)
  Session 3: v2sHPzLi65qdQ6hf-Js9BfbjCdw237_ldwjv4O9gOAJak9wodMbphJdBNk3fkb5WCgpDQ8I3EbhOABOA8t5SbA (Untitled)
  Session 4: KVh-vuJkxdu82QEem9mEJqlsehZ-MC_e1SnOJ7Xgd_lrHTjQZhpjlMgkO11L2jZt3heiLfxZmTU0K21jAkDofA (Untitled)
  Session 5: wm2yusSL9We4iCWtVQ8OgiRhxOTYG68-HqstgTHWeEx7_p2tmw4Wp59ymCN4bMg5hHgv9tlwAeuZZYkvGMQJiQ (Untitled)
  Session 6: _1CPODs-8nj298P5KnJz-t7BEQVsQOo6EyCog69m8gTNz_pANx0bruuIZ7qnAoLYTGBamNY1ZO3vaCNcVPD0Wg (Untitled)
  Session 7: hy9LUO2nl-RZUbTZUgrqYDj1mubOHylpZk3swm78FojI3ne4KfoLyDAjFLBhNK3WVW37L9ChwSPfRWPCGo3Ezg (Untitled)
  Session 8: GfXPY1xW_iSnX8H5hW76Z1_5bILggPqR4oM7Ap3fqJKBs31el-N2GY98rtL2-lOX7PCZ27GZhqT72QRHcbW-Rw (Untitled)
  Session 9: GnurYMIVmn2xtI9pusFTuqQgQau7C3-kRSIfPGO4vyRy9X02Wz0z5Z9WVCW6GeXPqSHyE8Vz98kCZRoZPeD1xw (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: H2WPCLfx3tnaOKw6qNE9eNJJdPtsHwxXCpYHoV-f3xt7el2rQFGELa6I9HJZThmNQ2UpfAh9ehJz8_ec0-iu3g

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: tBRQWmMEPalc6ts1ua2Z57OBzn6PsKfCqXtA_0TiAOTiVF25VFYVSl5iUINgDOeG5rV6ols7k7sPqw9V28bGDQ

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information available, we have the "ekoDB" product. It's a high-performance database product. The price for this product is $99.

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
✓ Created second session: EN1u8WYJ-XYaHi9TyUf54f-TTSu0EMrkz__RCp9y29vsugdqMwNrgyEQPBPnox61oymGm6v6jMBP1LKSqwQpoA
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
  1. Score: 0.784
  2. Score: 0.745
  3. Score: 0.730

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.713
  2. Score: 1.498
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

✅ Script saved: JPLmgOeDGz7Fp4uPjLL7AQrqhTwCBbZc-1VajkgnnRWc1y_owbMji_UM_RWb3RYIrHFoFy2-wyA8465hq7WN0g
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
