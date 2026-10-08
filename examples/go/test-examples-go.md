make test-examples-go
🧪 Running Go examples (direct HTTP/WebSocket)...

╔════════════════════════════════════════╗
║     ekoDB Go Examples Test Suite      ║
╚════════════════════════════════════════╝

=== Checking Server Connection ===
✓ Server is ready

=== Running 10 Examples ===

=== Running simple_crud.go ===
=== Simple CRUD Operations (Direct HTTP) ===

✓ Authentication successful

=== Insert Document ===
Inserted: map[id:yFPY0vOQFkyupnZwSMBkPp-c3q-PI7Xuj-e_mevTplsU3CLH1sHpWCG0ELvPQFPla4X7IAf-jf6oiTZJv6UlBw]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] id:yFPY0vOQFkyupnZwSMBkPp-c3q-PI7Xuj-e_mevTplsU3CLH1sHpWCG0ELvPQFPla4X7IAf-jf6oiTZJv6UlBw name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query ===
Found documents: [map[active:map[type:Boolean value:true] id:yFPY0vOQFkyupnZwSMBkPp-c3q-PI7Xuj-e_mevTplsU3CLH1sHpWCG0ELvPQFPla4X7IAf-jf6oiTZJv6UlBw name:map[type:String value:Test Record] value:map[type:Integer value:42]]]

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] id:yFPY0vOQFkyupnZwSMBkPp-c3q-PI7Xuj-e_mevTplsU3CLH1sHpWCG0ELvPQFPla4X7IAf-jf6oiTZJv6UlBw name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
✓ simple_crud.go completed successfully

=== Running simple_websocket.go ===
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: F9sbSTwl_OlQzEcgOF30OUhCSVfi3xaFlD24EG-yE2DlYVlWoC4RTUmOJAMM4J2z3WtyNOYz5nV7Q8ZLzEuusQ

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "messageId": "1791438381815597000",
  "payload": {
    "data": [
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "F9sbSTwl_OlQzEcgOF30OUhCSVfi3xaFlD24EG-yE2DlYVlWoC4RTUmOJAMM4J2z3WtyNOYz5nV7Q8ZLzEuusQ",
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
✓ Retrieved 1 record via WebSocket

✓ WebSocket example completed successfully
✓ simple_websocket.go completed successfully

=== Running http_functions.go ===
🚀 ekoDB Functions Example (Go/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Function saved: qAb5yGT_F44-3Ib22hiH8MXJMpbTMLAgHk58jZoFslDOXCAzTTvrUFTKEjbFYnKgF_1aEU2oklWwCrBXlHHpiw
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Function saved: ALO1qivam5l-QDxQEK7IJ4IrEwCZ5pxag5m06HD0Q4FMsGtEuQ7RuyjSXpJNAIPVdTcL1fTpb7UhNXhCxa0QwQ
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: aLils6lZkWCBvxXDoizQwEA9_lqiutbmB7yRQdfEHR3GsdnUDq4Y3vvhys9LpfBL-9FbOcvN9hYLB6oIyzl0Cg
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"avg_score":{"type":"Float","value":50},"count":{"type":"Integer","value":5},"max_score":{"type":"Integer","value":90},"status":{"type":"String","value":"inactive"}}
   {"avg_score":{"type":"Float","value":60},"count":{"type":"Integer","value":5},"max_score":{"type":"Integer","value":100},"status":{"type":"String","value":"active"}}

📝 Example 4: Function Management

📋 Total functions: 3
🔍 Retrieved function: Get Active Users
✏️  Function updated
🗑️  Function deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

✅ All examples completed!
✓ http_functions.go completed successfully

=== Running batch_operations.go ===
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
✓ batch_operations.go completed successfully

=== Running kv_operations.go ===
=== Key-Value Operations (Direct HTTP) ===

✓ Authentication successful

=== KV Set ===
✓ Set key: session:user123:go

=== KV Get ===
Retrieved value: map[type:Object value:map[userId:123 username:john_doe]]

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1:go: map[type:Object value:map[name:Product 1 price:29.99]]
cache:product:2:go: map[type:Object value:map[name:Product 2 price:39.989999999999995]]
cache:product:3:go: map[type:Object value:map[name:Product 3 price:49.989999999999995]]

=== KV Delete ===
✓ Deleted key: session:user123:go
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ kv_operations.go completed successfully

=== Running collection_management.go ===
=== Collection Management (Direct HTTP) ===

✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: 3vqlZlGbV7Mn4CpN-gVghOktcIp64I0R_0dvaLJu8RUYFRzZmQ__FE3vwC2jApEWQ3hBLjkg-xm0Hyfqw3in0w

=== List Collections ===
Total collections: 13
Sample collections: [chat_agent_configs__ek0_testing chat_goal_templates__ek0_testing audit__ek0_testing chat_tasks__ek0_testing agent_function_versions__ek0_testing]

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ collection_management.go completed successfully

=== Running transactions.go ===
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: om_rKRJBuTlZGJT7iTIUllVYGJ11F5JBn7VHXGpxYuCIVJ67uL5DUDca6rksfrpE4xSn5lxPrh-vXK4m0WsVMg
Created Bob: $500 - ID: qYdGP3NtCaJVjtat1TbZ7FkVc4Dbc09G3W1ZofWpqfVczn-015fm1KOyJTPzngfO0dC7LcpSwhUdY7AODZjKLA

=== Example 1: Begin Transaction ===
Transaction ID: 2f0fc5c5-7e21-4971-a188-33583c504930

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: map[type:Integer value:800]
Bob: map[type:Integer value:700]

=== Example 5: Rollback ===
New transaction: eb085050-5bfe-4ca6-b93a-5f31e5e71939
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: map[type:Integer value:700]

=== Cleanup ===
✓ Deleted test accounts

✓ All transaction examples completed
✓ transactions.go completed successfully

=== Running crud_functions.go ===
🚀 ekoDB Complete CRUD Functions Example
============================================================
Demonstrates:
  • Insert + Verify (using Query)
  • Query + Update Status + Verify
  • Query + Update Credits + Verify
  • Query Before Delete + Delete + Verify Gone

Each function shows Functions chaining with proper verification
============================================================

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted collection: crud_functions_users_go

============================================================
📝 function 1: Insert + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: TxT_u06c6rGmCM55CxyhtMveIP7lh6PlnTyPMPrj5EPTn3OyIKQqpeNWN_Syfe_l5Vb930rPIPHnZp3Tu3maZw

2️⃣ Calling function (Insert + Verify)...
   ✅ function executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: fdMXVZlXyFC_BZgPMVWZnko0DUXbWOQMkPA4h_JWGOqN_4Pe36TUSHmwZW4mrv6-USouHHOsFW-vVvPzuNEEAg
   📋 Name: map[type:String value:Alice Smith]
   📋 Email: map[type:String value:alice@example.com]
   📋 Status: map[type:String value:pending]
   📋 Credits: map[type:Integer value:0]

============================================================
📝 function 2: Query + Update + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: HpWIQBjvPdGyrfCMGzcxFt4uvFtu15ckr9gI8XTf2MhJqHpWsIFLw25BfX9frdkL_oGoEBOUFF4LwAx_km8H9w

2️⃣ Calling function (Query + Update + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Status updated to: map[type:String value:active]
   📋 Name: map[type:String value:Alice Smith]

============================================================
📝 function 3: Query + Update Credits + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: 2R7yTuS09ARUESnMmhAlY0IzpQ970NbfAZSm_xYLVjfs7yb88LmKk36M1BCJnY8prPn33BiyjrKeNoNSlSXrqQ

2️⃣ Calling function (Query + Update Credits + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: map[type:Integer value:100]
   📋 Status: map[type:String value:active]
   📋 Name: map[type:String value:Alice Smith]

============================================================
📝 function 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: jyYy5Y1tE3GPLd1Lm5xJYL-7l4-Netp0FYE0D9pD928U2Du4SkCl-j-Ll28LN4UbORj35_NTX_gS4tq6qP2jTQ

2️⃣ Calling function (Query + Delete + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: TxT_u06c6rGmCM55Cxyh...
   ✅ Deleted script: HpWIQBjvPdGyrfCMGzcx...
   ✅ Deleted script: 2R7yTuS09ARUESnMmhAl...
   ✅ Deleted script: jyYy5Y1tE3GPLd1Lm5xJ...
   ✅ Deleted collection: crud_functions_users_go

============================================================
✅ Complete CRUD Functions Example Finished!
============================================================

💡 Key Takeaways:
   ✅ Functions chain steps together
   ✅ Each function demonstrates operation + verification
   ✅ Parameters make functions reusable
   ✅ Verification is built into the function itself
   ✅ Complete CRUD lifecycle in 4 focused functions
✓ crud_functions.go completed successfully

=== Running document_ttl.go ===
╔════════════════════════════════════════════════════════╗
║     TTL EXPIRATION VERIFICATION TEST                   ║
╚════════════════════════════════════════════════════════╝

This test verifies that document TTL expiration works correctly.
We will insert documents with short TTL and verify they expire.

✓ Client connected

═══════════════════════════════════════════════════════════
TEST 1: Document TTL Expiration
═══════════════════════════════════════════════════════════

[Step 1] Insert document with 3 second TTL
  Input: {name: 'TTL Test', value: 'should expire'}
  TTL: 3s
  Output: Document ID = sahZeaLMFExioV4GF4bB80xCH3JUK1sesEhg2eE1iRdYoMQGkrEuqtvdVDasjOfDwCQxYrZLrMxdGAb9LHYN4g
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: FindByID(sahZeaLMFExioV4GF4bB80xCH3JUK1sesEhg2eE1iRdYoMQGkrEuqtvdVDasjOfDwCQxYrZLrMxdGAb9LHYN4g)
  Output: Found document with name = map[type:String value:TTL Test]
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: FindByID(sahZeaLMFExioV4GF4bB80xCH3JUK1sesEhg2eE1iRdYoMQGkrEuqtvdVDasjOfDwCQxYrZLrMxdGAb9LHYN4g)
  Output: Error (expected) - request failed with status 404: ��error�Record not found (expired)
  ✓ PASS: Document expired (not found error)

═══════════════════════════════════════════════════════════
CLEANUP
═══════════════════════════════════════════════════════════

╔════════════════════════════════════════════════════════╗
║              ALL TTL TESTS PASSED ✓                    ║
╚════════════════════════════════════════════════════════╝

TTL expiration is working correctly:
  • Documents with TTL expire after the specified time
  • Documents without TTL persist indefinitely
  • Different TTL durations are handled correctly
✓ Deleted test collection
✓ document_ttl.go completed successfully

=== Running websocket_ttl.go ===
╔════════════════════════════════════════════════════════╗
║   WEBSOCKET TTL EXPIRATION VERIFICATION TEST           ║
╚════════════════════════════════════════════════════════╝

This test verifies TTL expiration works via WebSocket connections.
We will use WebSocket to insert, query, and verify TTL expiration.

✓ Client connected

═══════════════════════════════════════════════════════════
TEST: WebSocket TTL Expiration
═══════════════════════════════════════════════════════════

[Step 1] Insert document with 3 second TTL
  Input: {name: 'WS TTL Test', value: 'should expire'}
  TTL: 3s
  Output: Document ID = WJJzrpPeCtqLaNITbJBeO3FcRrqM2bt65a0JKKqGXIoXyC5Lqu65__luJfxqWWLHrFGNT5uHSA-5_P6xWXlueA
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: FindByID(WJJzrpPeCtqLaNITbJBeO3FcRrqM2bt65a0JKKqGXIoXyC5Lqu65__luJfxqWWLHrFGNT5uHSA-5_P6xWXlueA)
  Output: Found document with name = map[type:String value:WS TTL Test]
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: FindByID(WJJzrpPeCtqLaNITbJBeO3FcRrqM2bt65a0JKKqGXIoXyC5Lqu65__luJfxqWWLHrFGNT5uHSA-5_P6xWXlueA)
  Output: Error (expected) - request failed with status 404: ��error�Record not found (expired)
  ✓ PASS: Document expired (not found error)

═══════════════════════════════════════════════════════════
CLEANUP
═══════════════════════════════════════════════════════════

╔════════════════════════════════════════════════════════╗
║          WEBSOCKET TTL TEST PASSED ✓                   ║
╚════════════════════════════════════════════════════════╝

WebSocket TTL expiration is working correctly:
  • Documents with TTL inserted via client expire correctly
  • Queries correctly return nil for expired documents
✓ Deleted test collection
✓ websocket_ttl.go completed successfully

╔════════════════════════════════════════╗
║           Test Summary                 ║
╚════════════════════════════════════════╝
Total: 10
Passed: 10
Failed: 0
✅ Go direct examples complete!
=== ekoDB Advanced CRUD Integration Example (Go) ===

--- Inserting base record ---
Inserted record: IAedDqipupmIIU8NZe9CoMi4CuTwKYFxV-HNmLMIjEUzbWoNyr5q1-uuOsFRdW-kXflvN2ckVQYCQ2l4QmS_jA

--- UpdateWithAction: increment ---
After increment by 5: count = map[type:Integer value:5]
After increment by 3: count = map[type:Integer value:8]

--- UpdateWithAction: push ---
After push 'new-tag': tags = map[type:Array value:[initial new-tag]]
After push 30.0: scores = map[type:Array value:[10 20 30]]

--- UpdateWithAction: decrement ---
After decrement by 2: count = map[type:Integer value:6]

--- UpdateWithActionSequence ---
After sequence: count=map[type:Integer value:16], tags=map[type:Array value:[initial new-tag batch-added]], scores=map[type:Array value:[10 20 30 99.9]]

--- Cleanup ---
Cleaned up all resources

=== All advanced CRUD operations completed successfully ===
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
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: lyrSuE2NYGvjR5S8luiVtbFioXC_9FG5gRotBZacTbU7VDlo1DUwceDOSPkhh5nuf2LVSki4akkLWhqiY6kVOw

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: High-performance database product
- **Price**: $99

If you have any more questions or need further assistance, feel free to ask!

✓ Second message sent

=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of ekoDB is $99. If you have any other questions or need more information, feel free to ask!

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: wKaLpXE3DB_BBRGwI2MVSJFwR5PRHc5NShcFZEYGdDdVsKVqYsLFwITSBSXNYCV6b8sjJaX5ghSZTBfqSWUYdw
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 7

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 6

=== Cleanup ===
✓ Deleted chat session: wKaLpXE3DB_BBRGwI2MVSJFwR5PRHc5NShcFZEYGdDdVsKVqYsLFwITSBSXNYCV6b8sjJaX5ghSZTBfqSWUYdw
✓ Deleted chat session: lyrSuE2NYGvjR5S8luiVtbFioXC_9FG5gRotBZacTbU7VDlo1DUwceDOSPkhh5nuf2LVSki4akkLWhqiY6kVOw
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: YbbbU20q9hKKDzH0ZKUfZ3-mpi242httbchCPS1qiGlWmW9Meo4tAskU4Wm4vvQYrA9I7O6wAmDroyMD4hlZ0Q

=== Sending Chat Message ===
Message ID: 0X3ICNvhEL5sWONh9jTHqZaWyhsBZTX-oEjMGQzSgeUUUQFAjtPnQ-OMQG89LOe_eF0-dC16JxHcuBXCsLSUqA

=== AI Response ===
The available products and their prices are as follows:

1. **ekoDB**
   - Description: A high-performance database product with AI capabilities
   - Price: $99

2. **ekoDB Pro**
   - Description: Enterprise edition product with advanced features
   - Price: $299

3. **ekoDB Cloud**
   - Description: Fully managed cloud database service product
   - Price: $499

=== Context Used (3 snippets) ===
  Snippet 1: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:A high-performance database product with AI capabilities id:L_FAgAp2JdDreLg9O7-TJaNrSlkxrg1wZ0viVRJqpyygws8PbehkRDyVA1MjEI3fiLQC95QdRAVzhUefNZC-Jg name:ekoDB price:99] score:0.1111111111111111]
  Snippet 2: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Enterprise edition product with advanced features id:izEFrrwpB2kChhF6gUZ9kAcegp25IWU15UD0EVFq8fFFU9q-QHn3FaTPCFI1Orr9Y56gCLfIQzUdmZDt0Tq78A name:ekoDB Pro price:299] score:0.1111111111111111]
  Snippet 3: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Fully managed cloud database service product id:tBQnY6kAcS6vBX3lt4U5T-N5bKHlj-eq_Dx0Pre5Z0yn9-xMUQopW6Oz_ugMJGKRRSo8_goS241BBvBDX_sgUw name:ekoDB Cloud price:499] score:0.1111111111111111]

Execution Time: 2890ms

=== Token Usage ===
Prompt tokens: 3413
Completion tokens: 90
Total tokens: 3503

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Message Stream (SSE) Example (Go) ===

Created session: tHmYnbuxsoCJpzfbuSTJ22gXNu-Z2MdEWR05_Ob1kooGDC8nUWtMJwApLRxOST0tb77ZPKu-K6nTdVzkx2gjJQ

Streaming response for: 'What is ekoDB?'

{"__progress":"received"}{"__progress":"generating","model":"gpt-4o-mini","provider":"openai"}EkoDB is a high-performance, embedded database designed for use in various applications where efficient data storage and retrieval are crucial. It's characterized by its lightweight nature, making it suitable for use in environments with limited resources, such as mobile devices, Internet of Things (IoT) devices, and other embedded systems.

EkoDB typically features:

1. **Embedded Design**: It runs within the application rather than as a separate server, allowing for faster access to data since there's no network overhead.

2. **High Performance**: EkoDB is optimized for low-latency operations and can handle a significant amount of transactions per second, making it ideal for real-time applications.

3. **ACID Compliance**: The database likely adheres to the principles of ACID (Atomicity, Consistency, Isolation, Durability), ensuring reliable transactions even in the event of failures.

4. **Schema Flexibility**: It may support various data models, such as key-value pairs, which allows developers to choose the most suitable model for their particular use case.

5. **Concurrency Control**: The database is designed to allow multiple users or processes to read and write to the database simultaneously while maintaining data integrity.

EkoDB is utilized in applications where traditional databases might be too bulky or where resource constraints are a concern. Examples include mobile applications, automotive systems, and various embedded platforms.

For specific features, performance benchmarks, and configurations, it would be beneficial to refer to the official documentation or website of EkoDB, as implementations and capabilities can vary.

--- Stream complete ---
Message ID: MSLMr_k2IcyN-hY1AOKEBhTbjQZUTPOE8oO5pb3bPhJwvPwqOnrZnSJ8iWtScu2GnIa49y6UZS3A2S64W9LvBA
Execution time: 4511ms
Context window: 128000 tokens

✓ Chat message stream example completed
✓ Client created

=== Get All Chat Models ===
OpenAI models: [o3-2025-04-16 gpt-5.1-chat-latest gpt-5.3-chat-latest gpt-5.2-2025-12-11 sora-2 chatgpt-image-latest gpt-4.1-mini gpt-3.5-turbo gpt-4o-mini gpt-image-2.5-sunburst gpt-image-2.5-flare-2026-09-08 gpt-6-luna gpt-5.4-mini-2026-03-17 gpt-4-turbo gpt-4.1-nano gpt-image-2 gpt-5.4-pro gpt-realtime-mini-2025-12-15 gpt-3.5-turbo-16k babbage-002 text-embedding-ada-002 gpt-4o-mini-tts gpt-4o-2024-08-06 o4-mini-2025-04-16 gpt-4o-2024-11-20 omni-moderation-latest gpt-realtime tts-1-hd gpt-realtime-1.5 gpt-realtime-2 gpt-5-search-api gpt-5.5-2026-04-23 gpt-5.1 gpt-5.1-codex gpt-4o o1-2024-12-17 gpt-5.2-chat-latest gpt-4o-mini-tts-2025-12-15 gpt-image-1 davinci-002 gpt-image-2-2026-04-21 gpt-4o-mini-transcribe o3-mini gpt-audio gpt-5.4-nano o3 gpt-5.1-codex-max gpt-realtime-2.1 gpt-4.1 gpt-5.5 gpt-4o-mini-search-preview-2025-03-11 text-embedding-3-large gpt-4o-mini-search-preview gpt-4o-2024-05-13 gpt-6-sol gpt-5.3-codex gpt-3.5-turbo-instruct sora-2-pro gpt-5 gpt-audio-mini-2025-12-15 gpt-transcribe o4-mini-deep-research-2025-06-26 o1-pro-2025-03-19 gpt-4.1-nano-2025-04-14 gpt-5-search-api-2025-10-14 gpt-4-turbo-2024-04-09 gpt-realtime-2.1-mini tts-1 gpt-5-mini omni-moderation-2024-09-26 gpt-5-mini-2025-08-07 gpt-realtime-2025-08-28 gpt-5.4-2026-03-05 gpt-live-transcribe gpt-3.5-turbo-0125 gpt-5-nano gpt-5.6-luna gpt-4 gpt-image-2.5-sunburst-2026-09-08 whisper-1 gpt-5.4-mini gpt-5-pro o1-pro gpt-realtime-whisper gpt-5.2-pro gpt-5-chat-latest gpt-live-1 gpt-4o-mini-transcribe-2025-12-15 gpt-5-2025-08-07 tts-1-1106 gpt-5.5-pro gpt-5-pro-2025-10-06 chat-latest gpt-5.2-pro-2025-12-11 gpt-4o-mini-transcribe-2025-03-20 gpt-audio-2025-08-28 gpt-5.4-pro-2026-03-05 gpt-realtime-translate gpt-5.6-sol gpt-audio-mini-2025-10-06 gpt-5.5-pro-2026-04-23 tts-1-hd-1106 gpt-realtime-mini gpt-5.4 gpt-4o-transcribe-diarize gpt-5-codex gpt-image-1-mini gpt-image-1.5 gpt-5.1-2025-11-13 o4-mini-deep-research gpt-4o-search-preview-2025-03-11 o3-mini-2025-01-31 gpt-4-0613 gpt-4o-search-preview gpt-5.6-terra text-embedding-3-small gpt-audio-1.5 gpt-3.5-turbo-instruct-0914 gpt-6-astra gpt-5.2-codex gpt-3.5-turbo-1106 gpt-5-nano-2025-08-07 gpt-audio-mini gpt-4o-mini-tts-2025-03-20 gpt-5.2 gpt-4.1-mini-2025-04-14 gpt-4.1-2025-04-14 gpt-4o-mini-2024-07-18 o4-mini o1 gpt-5.4-nano-2026-03-17 gpt-5.1-codex-mini gpt-4o-transcribe gpt-image-2.5-flare gpt-6.1-sol]
Anthropic models: [claude-haiku-5-5 claude-sonnet-5-5 claude-opus-5-5 claude-fable-5-1 claude-opus-5 claude-sonnet-5 claude-fable-5 claude-opus-4-8 claude-opus-4-7 claude-sonnet-4-6 claude-opus-4-6 claude-opus-4-5-20251101 claude-haiku-4-5-20251001 claude-sonnet-4-5-20250929]
Perplexity models: []
Gemini models: []
  anthropic: ok 14 models
  gemini: not_configured (unverified) No Gemini API Key
  openai: ok 135 models
  perplexity: not_configured (unverified) No Perplexity API Key

=== Get OpenAI Models ===
OpenAI models: [o3-2025-04-16 gpt-5.1-chat-latest gpt-5.3-chat-latest gpt-5.2-2025-12-11 sora-2 chatgpt-image-latest gpt-4.1-mini gpt-3.5-turbo gpt-4o-mini gpt-image-2.5-sunburst gpt-image-2.5-flare-2026-09-08 gpt-6-luna gpt-5.4-mini-2026-03-17 gpt-4-turbo gpt-4.1-nano gpt-image-2 gpt-5.4-pro gpt-realtime-mini-2025-12-15 gpt-3.5-turbo-16k babbage-002 text-embedding-ada-002 gpt-4o-mini-tts gpt-4o-2024-08-06 o4-mini-2025-04-16 gpt-4o-2024-11-20 omni-moderation-latest gpt-realtime tts-1-hd gpt-realtime-1.5 gpt-realtime-2 gpt-5-search-api gpt-5.5-2026-04-23 gpt-5.1 gpt-5.1-codex gpt-4o o1-2024-12-17 gpt-5.2-chat-latest gpt-4o-mini-tts-2025-12-15 gpt-image-1 davinci-002 gpt-image-2-2026-04-21 gpt-4o-mini-transcribe o3-mini gpt-audio gpt-5.4-nano o3 gpt-5.1-codex-max gpt-realtime-2.1 gpt-4.1 gpt-5.5 gpt-4o-mini-search-preview-2025-03-11 text-embedding-3-large gpt-4o-mini-search-preview gpt-4o-2024-05-13 gpt-6-sol gpt-5.3-codex gpt-3.5-turbo-instruct sora-2-pro gpt-5 gpt-audio-mini-2025-12-15 gpt-transcribe o4-mini-deep-research-2025-06-26 o1-pro-2025-03-19 gpt-4.1-nano-2025-04-14 gpt-5-search-api-2025-10-14 gpt-4-turbo-2024-04-09 gpt-realtime-2.1-mini tts-1 gpt-5-mini omni-moderation-2024-09-26 gpt-5-mini-2025-08-07 gpt-realtime-2025-08-28 gpt-5.4-2026-03-05 gpt-live-transcribe gpt-3.5-turbo-0125 gpt-5-nano gpt-5.6-luna gpt-4 gpt-image-2.5-sunburst-2026-09-08 whisper-1 gpt-5.4-mini gpt-5-pro o1-pro gpt-realtime-whisper gpt-5.2-pro gpt-5-chat-latest gpt-live-1 gpt-4o-mini-transcribe-2025-12-15 gpt-5-2025-08-07 tts-1-1106 gpt-5.5-pro gpt-5-pro-2025-10-06 chat-latest gpt-5.2-pro-2025-12-11 gpt-4o-mini-transcribe-2025-03-20 gpt-audio-2025-08-28 gpt-5.4-pro-2026-03-05 gpt-realtime-translate gpt-5.6-sol gpt-audio-mini-2025-10-06 gpt-5.5-pro-2026-04-23 tts-1-hd-1106 gpt-realtime-mini gpt-5.4 gpt-4o-transcribe-diarize gpt-5-codex gpt-image-1-mini gpt-image-1.5 gpt-5.1-2025-11-13 o4-mini-deep-research gpt-4o-search-preview-2025-03-11 o3-mini-2025-01-31 gpt-4-0613 gpt-4o-search-preview gpt-5.6-terra text-embedding-3-small gpt-audio-1.5 gpt-3.5-turbo-instruct-0914 gpt-6-astra gpt-5.2-codex gpt-3.5-turbo-1106 gpt-5-nano-2025-08-07 gpt-audio-mini gpt-4o-mini-tts-2025-03-20 gpt-5.2 gpt-4.1-mini-2025-04-14 gpt-4.1-2025-04-14 gpt-4o-mini-2024-07-18 o4-mini o1 gpt-5.4-nano-2026-03-17 gpt-5.1-codex-mini gpt-4o-transcribe gpt-image-2.5-flare gpt-6.1-sol]

=== Get Anthropic Models ===
Anthropic models: [claude-haiku-5-5 claude-sonnet-5-5 claude-opus-5-5 claude-fable-5-1 claude-opus-5 claude-sonnet-5 claude-fable-5 claude-opus-4-8 claude-opus-4-7 claude-sonnet-4-6 claude-opus-4-6 claude-opus-4-5-20251101 claude-haiku-4-5-20251001 claude-sonnet-4-5-20250929]

✓ Chat Models API example complete
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: ARfT2xTQMOLJIeLMCqenq39dHxNg4otXFEZQKWD7pHZfxBQARiFHwFH_qkTaK7Y0cFDe4yCr6NABkPisthTQVQ

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** A high-performance database product
- **Price:** $99

If you need more information or have any other questions, feel free to ask!

✓ Message 2 sent
  Response: The price of ekoDB is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 6mHGmHiBbH2Z5DJpIJgc8za_swyFYMaWNxEyghUjugT1yuHu5DJmRs_5zHdGLcaBnl1HhW9OUovoJYtPMRlkAw
  Parent: ARfT2xTQMOLJIeLMCqenq39dHxNg4otXFEZQKWD7pHZfxBQARiFHwFH_qkTaK7Y0cFDe4yCr6NABkPisthTQVQ

=== Listing Sessions ===
✓ Found 2 sessions
  Session 1: 6mHGmHiBbH2Z5DJpIJgc8za_swyFYMaWNxEyghUjugT1yuHu5DJmRs_5zHdGLcaBnl1HhW9OUovoJYtPMRlkAw (Untitled)
  Session 2: ARfT2xTQMOLJIeLMCqenq39dHxNg4otXFEZQKWD7pHZfxBQARiFHwFH_qkTaK7Y0cFDe4yCr6NABkPisthTQVQ (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: 6mHGmHiBbH2Z5DJpIJgc8za_swyFYMaWNxEyghUjugT1yuHu5DJmRs_5zHdGLcaBnl1HhW9OUovoJYtPMRlkAw

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ All session management operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: RrXWRk6cmqnKkBOLDirYusLCMAk30KRrzZEch7eBbHUFEUzQqv2SBonOSPV-IStnCoY90DBn6zliFdDiE_MwRQ

=== List Collections ===
Total collections: 13
Sample collections: [chat_agent_configs__ek0_testing chat_goal_templates__ek0_testing audit__ek0_testing chat_tasks__ek0_testing agent_function_versions__ek0_testing]

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Check Collection Exists (Before Creation) ===
Collection 'collection_utils_test_go' exists: false

=== Creating Test Documents ===
Created 5 test documents

=== Check Collection Exists (After Creation) ===
Collection 'collection_utils_test_go' exists: true

=== Count Documents ===
Document count in 'collection_utils_test_go': 5

=== Check Non-Existent Collection ===
Collection 'nonexistent_collection_xyz' exists: false

=== Cleanup ===
Deleted collection 'collection_utils_test_go'

✓ Collection Utilities example complete
✓ Client created
✓ conc_demo_pay_go saved
✓ conc_demo_rl_fail_go saved
✓ conc_demo_rl_skip_go saved
✓ conc_demo_lock_go saved

Invoke them like:
  POST /api/functions/conc_demo_pay_go        { "idempotency_key": "...", "amount": 100 }
  POST /api/functions/conc_demo_rl_fail_go    { "user_id": 42 }
  POST /api/functions/conc_demo_rl_skip_go    { "user_id": 42 }
  POST /api/functions/conc_demo_lock_go       { "resource": "queue:drain" }

✓ Cleaned up demo functions
=== ekoDB Convenience Methods Example ===

=== Native Map Creation ===
✓ Created record with native map: map[id:f61T59RtfVKCqtSc8wYNPgacz_sM4RHnqsbLPlmIiNDB8l0Ifphks_CbkXvVrv79ke3q6m-AdRy2zgn5B57meg]

=== Upsert Operation ===
✓ First upsert (update): map[active:map[type:Boolean value:true] age:map[type:Integer value:29] email:map[type:String value:alice.j@newdomain.com] id:f61T59RtfVKCqtSc8wYNPgacz_sM4RHnqsbLPlmIiNDB8l0Ifphks_CbkXvVrv79ke3q6m-AdRy2zgn5B57meg name:map[type:String value:Alice Johnson]]
✓ Second upsert (insert): map[id:new-user-1791438447792625000]

=== Find One Operation ===
✓ Found user by email: map[active:map[type:Boolean value:true] age:map[type:Integer value:29] email:map[type:String value:alice.j@newdomain.com] id:f61T59RtfVKCqtSc8wYNPgacz_sM4RHnqsbLPlmIiNDB8l0Ifphks_CbkXvVrv79ke3q6m-AdRy2zgn5B57meg name:map[type:String value:Alice Johnson]]
✓ User not found (as expected)

=== Exists Check ===
✓ Record exists: true
✓ Fake record exists: false (should be false)

=== Pagination ===
✓ Inserted 25 records for pagination
✓ Page 1: 10 records (expected 10)
✓ Page 2: 10 records (expected 10)
✓ Page 3: 7 records (expected 7)

=== Cleanup ===
✓ Deleted collection

✅ All convenience methods demonstrated successfully!
✓ Client created
✓ crypto_demo_hmac_go saved
✓ crypto_demo_aes_go saved
✓ crypto_demo_uuid_go saved
✓ crypto_demo_totp_go saved
✓ crypto_demo_encoding_go saved

Invoke them with:
  POST /api/functions/crypto_demo_hmac_go     { "payload": "hi" }
  POST /api/functions/crypto_demo_aes_go      { "plaintext": "secret" }
  POST /api/functions/crypto_demo_uuid_go
  POST /api/functions/crypto_demo_totp_go
  POST /api/functions/crypto_demo_encoding_go { "title": "Héllo World" }

✓ Cleaned up demo functions
=== Distinct Values Example ===

Inserting sample products...
Inserted 8 products

=== Distinct Categories (all products) ===
Found 3 distinct categories:
  - map[type:String value:books]
  - map[type:String value:clothing]
  - map[type:String value:electronics]

=== Distinct Statuses (all products) ===
Found 3 distinct statuses:
  - map[type:String value:active]
  - map[type:String value:archived]
  - map[type:String value:discontinued]

=== Distinct Statuses in Electronics ===
Found 2 distinct statuses for electronics:
  - map[type:String value:active]
  - map[type:String value:discontinued]

Cleanup done.
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: PR27CW6jDGaWMvOGX791wR18XtsoLuXs8wBofNUPkfwYWOETZXoE2U-nSMuFkS0q1CtA_QoeQ9peX3Is9ren5g

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: 2xr2-5wWCmig1nwihwa_cA1UrCXDg59cqb5yZNbzxUjJetzcEHR1p_dKTMD7is9V-PKuy1WnurBJCvmAumf53w

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
=== ekoDB as Edge Cache - Simple Example ===

Setting up edge cache collection...
✓ Cache entry created

Creating edge cache lookup script...
✓ Edge cache script created: oDyQzbZeh7jpbQM7jXZnP-m4ADoTNsYPreQffbwLr67IufF0hA6z-Y_wS7YTPq66Q1DsBq71RojaB-f0A017Vg

Call 1: Cache lookup
Response time: 2ms
Found 1 cached entries

Call 2: Cache lookup (connection warm)
Response time: 2ms
Found 1 cached entries

=== The Magic ===
- Your DATABASE is your EDGE
- No Redis needed
- No CDN needed
- No cache invalidation logic needed (TTL handles it)
- With ripples: All nodes auto-sync cache
- One service: Database + Cache + Edge Functions

✓ Example complete!

🧹 Cleaning up...
✓ Cleanup complete
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

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 63.571292ms
   📊 Records: 1
   📦 Data: {
        "value": {
          "type": "Object",
          "value": {
            "address": {
              "city": "Gwenborough",
              "geo": {
                "lat": "-37.3159",
          ...

Second call (cache hit - from cache):
   ⏱️  Duration: 4.047083ms
   📊 Records: 1
   📦 Data: {
        "value": {
          "type": "Object",
          "value": {
            "address": {
              "city": "Gwenborough",
              "geo": {
                "lat": "-37.3159",
          ...
   🚀 Cache speedup: 15.8x faster!

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
client_function_contract: ok
🚀 ekoDB Functions Example (Go Client)

✅ Client initialized

🧹 Cleaning up...
✅ Deleted collection
✅ Deleted test scripts

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved: 0dNJTYcKykhGRdY-XaD5zbzhLdiM-mU8K9K_q1v6aYVUYCRl9mLApU2TDYOlByMU6P7hKHsEcXMqKUCkAoB5qw
📊 Found 5 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Function

✅ Function saved
📊 Found 3 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Function

✅ Function saved
📊 Statistics: 2 groups
   map[avg_score:map[type:Float value:50] count:map[type:Integer value:5] status:map[type:String value:inactive]]
   map[avg_score:map[type:Float value:60] count:map[type:Integer value:5] status:map[type:String value:active]]
⏱️  Execution time: 0ms

📝 Example 4: Function Management

📋 Total functions: 3
🔍 Retrieved function: Get Active Users
✏️  Function updated
🗑️  Function deleted

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
🚀 ekoDB Go Advanced Functions Example

📋 Setting up test data...
✅ Created 8 products

📝 Example 1: List All Products

✅ Function saved
📊 Found 8 products
⏱️  Execution time: 0ms

📝 Example 2: Group Products by Category

✅ Function saved
📊 Category breakdown:
   map[avg_price:map[type:Float value:365.6666666666667] category:map[type:String value:Furniture] count:map[type:Integer value:3]]
   map[avg_price:map[type:Float value:367] category:map[type:String value:Electronics] count:map[type:Integer value:5]]
⏱️  Execution time: 0ms

✅ All advanced script examples finished!
🧹 Cleaning up...
✅ Cleanup complete
🚀 ekoDB Go AI Functions Example

📋 Setting up test data...
✅ Created 2 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
📊 AI Response generated
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embed script saved
📊 Embedding generated
⏱️  Execution time: 0ms

✅ All AI script examples finished!

💡 This example demonstrates:
   ✅ Chat completions with system/user messages
   ✅ Embedding generation for text
🧹 Cleaning up...
✅ Cleanup complete
🚀 ekoDB Go Complete Functions Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Function saved: N9tVb-e9VL46HKpcGLwbHnaxQp-d2y6Yf0VTJD6YVDCJ0ySnRbDpVx9eQL6q-UF1g2gNCPuRsj69PmVGqsMrsQ
📊 Found 2 category stats

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories

📝 Example 4: Multi-Stage Pipeline (FindAll → Group → Count)

✅ Function saved
📊 Pipeline executed 3 stages


✅ All complete script examples finished!

💡 This example demonstrates ekoDB's function system:
   ✅ FindAll operations
   ✅ Group aggregations (Count, Average)
   ✅ Multi-stage pipelines (FindAll → Group → Count)
   ✅ Function management (save, call, delete)
🧹 Cleaning up...
✅ Cleanup complete
🚀 ekoDB Go CRUD Functions Example

📋 Setting up test data...
✅ Created 10 test users

📝 Example 1: List All Users

✅ Function saved
📊 Found 10 users
⏱️  Execution time: 0ms

📝 Example 2: Count Users by Status

✅ Function saved
📊 User counts by status:
   map[count:map[type:Integer value:7] status:map[type:String value:active]]
   map[count:map[type:Integer value:3] status:map[type:String value:inactive]]
⏱️  Execution time: 0ms

✅ All CRUD script examples finished!
🧹 Cleaning up...
✅ Cleanup complete
🚀 ekoDB Go KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: IzEK4UdSB9ZTtZu0ABgqWABNjyVSAzt3OVGudCC12FXfPpzGHTqEx1T_wC6iBdvdgfcS86YLCiKNyR7Zcq6H1w
✅ Inserted 2 products with wrapped types

📝 Example 2: function with Wrapped Type Parameters

✅ Function saved: vj63LT1EN6INhuLeOTeVLlv0pC_yqJs9nFeJ0gUATRroTD3zkHkAcs-UA5chHi7wScScmbPr1Y6ryaarppQ61A
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: map[type:Object value:map[role:admin userId:user_abc]]
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session
📝 Example 4: KV Operations in Functions

✅ Function saved: f-Bf1Q8s5x_CtDN8YYjsdGwbFQc7gnhSGNPeeJxeyAOCEF7zO3dVeN7rc04HmKuS6A7n7fkN4Tqre41Xo8-58A
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Function

✅ Function saved: yEGOzLQIvlkGw8BO-Qd0Qs82Ob60kFHH_2yZDGTGrZrMH-2WUb7AFVen6vQLJ5wdWAiXCb-Ochihe3ZVXnF9-w
📊 Processed order with caching
⏱️  Stages executed: 3
⏱️  Execution time: 0ms

✅ All KV & Wrapped Types examples completed!

💡 Key takeaways:
   ✅ Use Field* helpers for type-safe wrapped values
   ✅ FieldDecimal() preserves precision (no floating point errors)
   ✅ KV store is great for caching and quick lookups
   ✅ StageKv*() functions work within scripts
🧹 Cleaning up...
✅ Cleanup complete

🚀 ekoDB Go Search Functions Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Function saved
📊 Found 5 documents
   1. map[type:String value:Introduction to Machine Learning] (map[type:String value:AI])
   2. map[type:String value:Database Design Principles] (map[type:String value:Database])
   3. map[type:String value:Vector Databases Explained] (map[type:String value:Database])
   4. map[type:String value:Getting Started with ekoDB] (map[type:String value:Database])
   5. map[type:String value:Natural Language Processing] (map[type:String value:AI])
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   map[category:map[type:String value:AI] count:map[type:Integer value:2]]
   map[category:map[type:String value:Database] count:map[type:Integer value:3]]
⏱️  Execution time: 0ms

✅ All search script examples finished!
🧹 Cleaning up...
✅ Cleanup complete
=== ekoDB Goal Template CRUD Example (Go) ===

--- Creating goal template ---
Created template: Data Migration (id: O2R5wfu1cPCpj3ZGaO08vwNoBqwYzAlWnaIre0V8-eqxkoTJRDH8Q09XzxQCZJaYeH3J3XaBqD_irsLhpYYKOg)

--- Listing templates ---
Templates: map[count:1 items:[map[description:map[type:String value:Template for migrating data between schemas] id:O2R5wfu1cPCpj3ZGaO08vwNoBqwYzAlWnaIre0V8-eqxkoTJRDH8Q09XzxQCZJaYeH3J3XaBqD_irsLhpYYKOg steps:map[type:Array value:[map[description:Analyze source schema] map[description:Create target schema] map[description:Migrate records] map[description:Validate results]]] title:map[type:String value:Data Migration]]]]

--- Getting template ---
Fetched: Data Migration

--- Updating template ---
Updated description: Updated: comprehensive data migration workflow

--- Deleting template ---
Template deleted successfully

✓ Goal template CRUD example completed
=== ekoDB Goals, Tasks & Agents Integration Example (Go) ===

--- Creating goal ---
Created goal: Deploy v2.0 (id: 5d_OVx3uNrDI5onvqbCxIqoH2nEgAGhsUp44wI1nE4M1HPk2kKjI8zvBoRFjdBeMFNvtobun0-06GQ6Ysi-X_A)

--- Listing goals ---
Goals response: map[count:1 goals:[map[created_at:2026-10-08T05:47:39.951713+00:00 description:Ship the v2.0 release to production id:5d_OVx3uNrDI5onvqbCxIqoH2nEgAGhsUp44wI1nE4M1HPk2kKjI8zvBoRFjdBeMFNvtobun0-06GQ6Ysi-X_A status:pending steps:[{"description":"Run integration tests"},{"description":"Build release artifacts"},{"description":"Deploy to staging"}] title:Deploy v2.0 updated_at:2026-10-08T05:47:39.951713+00:00]]]

--- Getting goal ---
Fetched goal: Deploy v2.0

--- Updating goal ---
Updated description: Ship the v2.0 release to production (with hotfix)

--- Searching goals ---
Search result: map[count:1 items:[map[_score:12.870000000000001 created_at:map[type:DateTime value:2026-10-08T05:47:39.951713+00:00] description:map[type:String value:Ship the v2.0 release to production (with hotfix)] id:5d_OVx3uNrDI5onvqbCxIqoH2nEgAGhsUp44wI1nE4M1HPk2kKjI8zvBoRFjdBeMFNvtobun0-06GQ6Ysi-X_A status:map[type:String value:pending] steps:map[type:String value:[{"description":"Run integration tests"},{"description":"Build release artifacts"},{"description":"Deploy to staging"}]] title:map[type:String value:Deploy v2.0] updated_at:map[type:DateTime value:2026-10-08T05:47:39.964591+00:00]]]]

--- Goal step lifecycle ---
Step 0 started
Step 0 completed
Step 1 started
Step 1 failed

--- Completing and approving goal ---
Goal status after complete: pending_review
Goal status after approve: in_progress

--- Testing GoalReject ---
Goal status after reject: failed

--- Creating task ---
Created task: Run benchmarks (id: z8c9Xn9gF2JuofE8MzhGLxP1ULfawGy7wohIjIRRTLd5UjEoH2lpOsongce8aKQv3SH1ypTIwJE9ysSTQo1VCA)

--- Listing tasks ---
Tasks: map[count:1 items:[map[description:map[type:String value:Execute YCSB benchmarks against staging] due_at:map[type:DateTime value:2026-10-09T05:47:40+00:00] id:z8c9Xn9gF2JuofE8MzhGLxP1ULfawGy7wohIjIRRTLd5UjEoH2lpOsongce8aKQv3SH1ypTIwJE9ysSTQo1VCA name:map[type:String value:Run benchmarks]]]]

--- Getting task ---
Fetched task: Run benchmarks

--- Task lifecycle: start -> succeed ---
Task status: running
Task status: active

--- Task lifecycle: pause -> resume -> fail ---
Task status after pause: paused
Task status after resume: active
Task failed successfully

--- Due tasks ---
Due tasks: map[count:0 items:[]]

--- Deleting tasks ---
Tasks deleted

--- Creating agent ---
Created agent: benchmark-runner (id: cs3SN1y2XHBqw9AM8ACqr5JbAPYv0H7hoP1tTz67Y8uzTdKEUUnmvjvRfsBFJOIEYzTRSnfXLPscj5tFHFscrg)

--- Listing agents ---
Agents: map[count:1 items:[map[deployment_id:map[type:String value:deploy_prod_1] description:map[type:String value:Runs periodic benchmarks] id:cs3SN1y2XHBqw9AM8ACqr5JbAPYv0H7hoP1tTz67Y8uzTdKEUUnmvjvRfsBFJOIEYzTRSnfXLPscj5tFHFscrg llm_model:map[type:String value:gpt-4.1] name:map[type:String value:benchmark-runner]]]]

--- Getting agent by ID ---
Fetched agent: benchmark-runner

--- Getting agent by name ---
Agent by name: benchmark-runner

--- Updating agent ---
Updated agent description: Runs hourly YCSB benchmarks

--- Agents by deployment ---
Agents in deploy_prod_1: map[count:0 items:[]]

--- Deleting agent ---
Agent deleted

--- Deleting goal ---
Goal deleted

=== All goals, tasks & agents operations completed successfully ===
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
  - Laptop ($1200) by Alice Johnson
  - Mouse ($25) by Alice Johnson

5. Complex join with multiple conditions:
Found 2 users with example.com emails
  - Alice Johnson (alice@example.com): Building A
  - Bob Smith (bob@example.com): Building B

=== Cleanup ===
✅ Deleted test collections

✅ Join operations examples completed!
✓ Client created
✓ go_users_register saved
✓ go_users_login saved
✓ go_users_verify_token saved

=== Auth flow defined as pure stored functions ===
Call them like:
  POST /api/functions/go_users_register { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/go_users_login    { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/go_users_verify_token { "token": "<jwt>" }

✓ Cleaned up demo functions
=== ekoDB KV Links Integration Example (Go) ===

--- Inserting documents ---
Inserted doc1: ZDJEafpUfXCVIJoFlIRm9hhaE5tWpjpMINF829fcvzHsYC7gMQxgXAv2RMUOAeXmOQX-fHfCFbVPQeyl5PwOqg
Inserted doc2: Aqo5GbfIbLJHGZDaqAAc3OEblXi7OZY5reaWOvd3CjkEhU9US5PK-l5y9L6YAWy2PHsSHdQNCs3uWEkcNq6ABA

--- Setting KV entry ---
KV entry set: team:engineering:go

--- Linking KV key to documents ---
Linked doc1: map[]
Linked doc2: map[]

--- Getting links ---
Links for team:engineering:go: [map[collection:kv_links_example_go created_at:2026-10-08T05:47:41.990531Z document_id:Aqo5GbfIbLJHGZDaqAAc3OEblXi7OZY5reaWOvd3CjkEhU9US5PK-l5y9L6YAWy2PHsSHdQNCs3uWEkcNq6ABA field_path:<nil> last_accessed:2026-10-08T05:47:41.991649Z metadata:map[]] map[collection:kv_links_example_go created_at:2026-10-08T05:47:41.989059Z document_id:ZDJEafpUfXCVIJoFlIRm9hhaE5tWpjpMINF829fcvzHsYC7gMQxgXAv2RMUOAeXmOQX-fHfCFbVPQeyl5PwOqg field_path:<nil> last_accessed:2026-10-08T05:47:41.991649Z metadata:map[]]]

--- Unlinking doc2 ---
Unlink result: map[]

--- Verifying links after unlink ---
Links after unlink: [map[collection:kv_links_example_go created_at:2026-10-08T05:47:41.989059Z document_id:ZDJEafpUfXCVIJoFlIRm9hhaE5tWpjpMINF829fcvzHsYC7gMQxgXAv2RMUOAeXmOQX-fHfCFbVPQeyl5PwOqg field_path:<nil> last_accessed:2026-10-08T05:47:41.993693Z metadata:map[]]]

--- Cleanup ---
Cleaned up all resources

=== All KV link operations completed successfully ===
✓ Client created

=== KV Set ===
✓ Set key: session:user123:go

=== KV Get ===
Retrieved value: map[type:Object value:map[userId:123 username:john_doe]]

=== KV Batch Set ===
✓ Batch set 3 keys
  cache:product:1:go: success
  cache:product:2:go: success
  cache:product:3:go: success

=== KV Batch Get ===
✓ Batch retrieved 3 values
  cache:product:1:go: map[name:Product 1 price:29.99]
  cache:product:2:go: map[name:Product 2 price:39.99]
  cache:product:3:go: map[name:Product 3 price:49.99]

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 3 keys matching 'cache:product:.*:go'

=== KV Query (Alias for Find) ===
Total keys in store: 4

=== KV Delete ===
✓ Deleted key: session:user123:go
✓ Verified: Key exists after delete: false

=== KV Batch Delete ===
✓ Batch deleted 3 keys
  cache:product:1:go: deleted
  cache:product:2:go: deleted
  cache:product:3:go: deleted

✓ All KV operations completed successfully
=== KV Precision: Float vs Decimal ===

=== Test 1: Using Go Floats (LOSES PRECISION) ===
✓ Stored products with float prices

Retrieved float prices:
  Widget A: $29.99 (expected $29.99) ✓
  Widget B: $39.99 (expected $39.99) ✓
  Widget C: $49.99 (expected $49.99) ✓

=== Test 2: Using FieldDecimal() (PRESERVES PRECISION) ===
✓ Stored products with decimal prices

Retrieved decimal prices:
  Widget A: $29.99 (expected $29.99) ✓
  Widget B: $39.99 (expected $39.99) ✓
  Widget C: $49.99 (expected $49.99) ✓

=== Test 3: Sum Calculation Comparison ===
  Float sum: $119.97 (expected $119.97)
  Decimal sum: $119.97 (expected $119.97)

=== Test 4: Extreme Precision Example ===
  Float 0.1 + 0.2 = 0.29999999999999999 (should be 0.3)
  Decimal "0.30" = 0.30 (exact!)

=== Summary ===
✅ Use FieldDecimal() for monetary values, percentages, and
   any case where floating-point errors are unacceptable.
✅ FieldDecimal() stores values as strings internally,
   preserving exact precision across all operations.

=== Cleanup ===
✓ Cleaned up test keys
✓ Client created
✓ go_route_admin saved
✓ go_route_user_by_id saved
✓ go_route_user_posts saved
✓ go_route_org_create_member saved

Try them with curl:
  curl http://localhost:8080/api/route/users/admin
  curl http://localhost:8080/api/route/users/42
  curl http://localhost:8080/api/route/users/42/posts/7
  curl -X POST http://localhost:8080/api/route/orgs/acme/members \
       -H 'Content-Type: application/json' -d '{"name":"alice"}'

✓ Cleaned up demo functions
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
=== ekoDB Raw Completion Stream (SSE) Example ===

--- Basic SSE Raw Completion ---
Response: The three primary colors are red, blue, and yellow.

--- Structured Output via SSE ---
JSON response: [
  {
    "name": "Earth",
    "diameter_km": 12742
  },
  {
    "name": "Jupiter",
    "diameter_km": 139820
  },
  {
    "name": "Mars",
    "diameter_km": 6779
  }
]

--- Blocking HTTP (for comparison) ---
Blocking response: Hello! I hope you're having a great day.

=== Done ===
=== ekoDB Schedules Integration Example (Go) ===

--- Creating schedule ---
Created schedule: nightly-backup (id: 05b2ea64-e92b-404b-9d76-f801a6b68477)

--- Listing schedules ---
Schedules: map[count:1 schedules:[map[created_at:2026-10-08T05:47:47.319236Z cron_expression:0 0 0 * * * description:Runs a database backup every night at midnight enabled:true function_label:schedule_noop_go_74638 id:05b2ea64-e92b-404b-9d76-f801a6b68477 last_execution:<nil> name:nightly-backup next_execution:2026-10-09T00:00:00Z parameters:map[] stats:map[avg_execution_time_ms:0 failed_executions:0 last_error:<nil> successful_executions:0 total_executions:0] timezone:UTC updated_at:2026-10-08T05:47:47.319236Z]]]

--- Getting schedule ---
Fetched schedule: nightly-backup (cron: 0 0 0 * * *)

--- Updating schedule ---
Updated cron: 0 0 2 * * *

--- Triggering schedule ---
Trigger response: map[schedule_id:05b2ea64-e92b-404b-9d76-f801a6b68477 status:triggered]

--- Pausing schedule ---
Schedule enabled: false

--- Resuming schedule ---
Schedule enabled: true

--- Deleting schedule ---
Schedule deleted

=== All schedule operations completed successfully ===
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
=== Search Examples ===

Setting up test data...
✅ Test data created

1. Basic full-text search:
Found 2 results
  1. Score: 12.870
  2. Score: 6.270

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 13.200
  2. Score: 13.200
  3. Score: 13.200
  4. Score: 13.200

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 26.400
  2. Score: 26.400
  3. Score: 26.400
  4. Score: 26.400

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 6.600
  2. Score: 6.600

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 6.600

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.767
  2. Score: 0.764
  3. Score: 0.760

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.507
  2. Score: 0.906
  3. Score: 0.304

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 13.200

9. Vector search with a metadata pre-filter (category = ml):
Found 2 documents in category "ml" (NLP excluded)
  1. Introduction to Machine Learning (category: ml)
  2. Deep Learning Fundamentals (category: ml)


✅ Search examples completed!
=== Cleanup ===
✅ Deleted test collections
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: map[id:tNtd0kp8OE6WvZQZ0fzL4q1-QPLSWS34gkEVjFP9hnCOP2i0Y0a8BE5W9pFKuZuxPq5mxdpkogB3v4k6ldpu4g]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-10-08T05:47:49+00:00] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:tNtd0kp8OE6WvZQZ0fzL4q1-QPLSWS34gkEVjFP9hnCOP2i0Y0a8BE5W9pFKuZuxPq5mxdpkogB3v4k6ldpu4g metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Test Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:42]]

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.990000
  created_at (DateTime): 2026-10-08 05:47:49 +0000 +0000
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1 tag2 tag3]
  metadata (Object): map[key:value nested:map[deep:true]]
  embedding (Vector): [0.1 0.2 0.3 0.4 0.5]
  categories (Set): [electronics computers]
  data (Bytes): 11 bytes
Plain record: map[active:true categories:[electronics computers] created_at:2026-10-08T05:47:49+00:00 data:aGVsbG8gd29ybGQ= embedding:[0.1 0.2 0.3 0.4 0.5] id:tNtd0kp8OE6WvZQZ0fzL4q1-QPLSWS34gkEVjFP9hnCOP2i0Y0a8BE5W9pFKuZuxPq5mxdpkogB3v4k6ldpu4g metadata:map[key:value nested:map[deep:true]] name:Test Record price:99.99 tags:[tag1 tag2 tag3] user_id:550e8400-e29b-41d4-a716-446655440000 value:42]

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-10-08T05:47:49+00:00] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:tNtd0kp8OE6WvZQZ0fzL4q1-QPLSWS34gkEVjFP9hnCOP2i0Y0a8BE5W9pFKuZuxPq5mxdpkogB3v4k6ldpu4g metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Updated Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: DD1OK1MCVlp7b3-1FnX9S3teCx7LWew03DBiULHumvK6YY9nsS3lr4k1VLW2AGTHonnxGVuX8UJnKeC5zvq7OQ

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
🚀 ekoDB Go Client - Native SWR Function Examples

📋 Demonstrates:
   • Single-function SWR pattern (replaces 4-step pipeline)
   • Automatic cache checking, HTTP fetching, and cache setting
   • Built-in audit trail support
   • Duration string TTLs ('15m', '1h', '30s')
   • Multi-function pipeline integration
   • Dynamic TTL configuration


Example 1: Basic Native SWR
────────────────────────────────────────────────────────────────────────────────
Single function replaces KvGet → If → HttpRequest → KvSet pipeline
✓ Created native SWR script: github_user_native_go (5BXfj1UsaPAMa7s0o2b-2d_EuMYBcDJ8nwHL3iuzvGK5wA2QfkPC54lgMbwk4gSZaUeBPcljXM0F4ekoAEnzfQ)

First call (cache miss - will fetch from GitHub API):
  Response time: 138ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 11ms
  Speedup: 12.5x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit_go (RBRP0IazdnAe3VJXqJKLoLacJchA0y-XmC9ZPZd4rxEWCgj9Ys7ZHaRZeUJhyPrckdOC_9-125I50T-9Ssewog)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail_go' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline_go (CiaVQptbmhuw2cm63pwdfr_fOrBDRVVGuyoPDmfAqQErdzjRiJBURhjzorjE1OD6VvEBaM0P5tk1DADIuJyNgQ)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users_go' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache_go (qwNfOWAgpLYRuc08-t4NrrYV7tPcwk5EEp2zK9nfuDkogrC4aFXgPiHx0q9lCL2wjyo9VUIVOFtEHjeXYlcFVg)
  ✓ Cached with TTL: 5m (5 minutes)
  ✓ Cached with TTL: 1h (1 hour)
  ✓ Cached with TTL: 30s (30 seconds)

================================================================================
✅ Key Benefits of Native SWR:
✅ Single function: Replaces 4-function cache-aside pattern
✅ Duration strings: Use '15m', '1h', '2h' instead of calculating seconds
✅ Built-in audit: Optional collection parameter for automatic logging
✅ Auto-enrichment: output_field populates params for downstream functions
✅ Transactional: Works correctly in both transactional and non-transactional contexts
✅ KV-optimized: Uses native KV store with proper TTL handling

=== Performance Comparison ===
Legacy Pattern: KvGet → If → HttpRequest → KvSet → Insert (5 functions)
Native SWR:     SWR → Insert (2 functions)
Result:         60% fewer functions, cleaner code, same behavior 🎯

🧹 Cleaning up...
✓ Deleted 4 test scripts

✅ All examples completed!
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_api_user_go (ulx3zQKS9YzVC2lh0zElTgXLKXCsc0ExS6R4fZ6z1eaaQ8l7--Ae_zJP4yQtt4BGAC0I4JWtHkZe6BgMBrCyWA)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "cached_at": {
        "type": "DateTime",
        "value": "2026-10-08T05:47:51+00:00"
      },
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
      "id": "1"
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
Response time: 11ms (served from cache)
✓ Lightning fast cache hit

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation
🧹 Cleaning up...
✓ Cleanup complete

✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 3q7aAWtSiAc4CV5qG6MOJdkmNFWJl8dj4NdgGootqvgillcxIgrxkz4_cbn-W5RnjrOR2TgM94nX-LvbRTBfSg
Created Bob: $500 - ID: w1Jxhrfogb0zS29mc0o5IIh7lp3LrbNmlxuqsKMUK9rOxWafGqApomJUfUpCrDfAoyDTHUwDEKcNNzV4ucNjQQ

=== Example 1: Begin Transaction ===
Transaction ID (server-default isolation): a1f5bfc9-4b8c-4fa6-88c5-170efef534b1

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

✓ Verified committed balances: Alice=$800, Bob=$700

=== Example 5: Rollback Demo ===
New transaction: 7d17f41e-acf4-4463-8305-94f5dde90dc2
Updated Bob: $700 → $600 (in transaction)
Status before rollback: Active
✓ Transaction rolled back

✓ Verified Bob remains $700 after rollback

=== Cleanup ===
✓ Deleted test account collection

✓ All client transaction examples completed
✓ Client created

=== Create User Function ===
Created user function with ID: LkfYvkal_ZzfPppCd1nMl_U2vyk5NAD83-u9a0zAmvtUpEldjoD1WV8casayC7ff1-NgHDQgX6xEWbR5Te1tMA

=== Get User Function ===
Retrieved: get_active_users_go - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 1 user functions:
  - get_active_users_go: Get Active Users

=== List User Functions by Tag ===
Found 1 user functions with 'users' tag:
  - get_active_users_go

=== Update User Function ===
User function updated successfully

=== Delete User Function ===
User function deleted successfully

✓ User Functions API example complete
=== WebSocket Chat Streaming Example (Go) ===

Created chat session: JL8QjUu3Yldzd4qorz05ZPGdHI9oujkR8d5LOp14_lROFh89uUcPBdg4oBQ3BliqYmYOuIvPU1X32k28sa8Vxw

Sending message: 'What is the capital of France?'
{"__progress":"received"}{"__progress":"generating","model":"gpt-4o-mini","provider":"openai"}The capital of France is Paris.

--- Stream ended ---
Message ID: HNgTVqONETkkcsVfbxCrVYX3YU-qeTB9rBjIF04GPyPseZd0ETdgV1K-eKpOzT1mvz6BzSkGTZASd-phCfppKQ
Execution time: 686ms
Token usage: {"completion_tokens":8,"prompt_tokens":15,"total_tokens":23}

Full response: {"__progress":"received"}{"__progress":"generating","model":"gpt-4o-mini","provider":"openai"}The capital of France is Paris....
=== WebSocket Subscription Example (Go) ===

✓ Authentication successful
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_go' ===
✓ Subscribed (subscription_id: sub_bb04575f2add44cb9474f0b5675e0074)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: UHhse2YSc0B7F0dlcFUhTQPsq05K9KSXZO-GE92tqi0j6HC3YI_L4jtNumAIwh8PpfEIqTam0Pmk2-esXznxnQ

  📡 Notification received:
     Event:      insert
     Collection: ws_subscribe_example_go
     Record IDs: [UHhse2YSc0B7F0dlcFUhTQPsq05K9KSXZO-GE92tqi0j6HC3YI_L4jtNumAIwh8PpfEIqTam0Pmk2-esXznxnQ]
     Timestamp:  2026-10-08T05:47:54.492204+00:00

Inserting record 2...
✓ Inserted: ibbrEPmoeL-stZ_pPk9I-6PqHmF5CQyGdNpuGg5dxg6QOZFx88qdjTy6AD1KzJo90ntsv190yxT0E9FVPx1kIQ

  📡 Notification received:
     Event:      insert
     Record IDs: [ibbrEPmoeL-stZ_pPk9I-6PqHmF5CQyGdNpuGg5dxg6QOZFx88qdjTy6AD1KzJo90ntsv190yxT0E9FVPx1kIQ]

=== Unsubscribing ===
✓ Unsubscribed

✓ WebSocket subscription example completed successfully
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: 2IHkYP3VIYeaPhazEONnS9TRv6AZ0ivMBrxxpzLoyud8bjzCnJ_C_y_rqaAySxo92ETwnR6OvA4-yC9y8z9ePg

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 5 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
Inserted with ripple: map[id:SH4aCv3j4GQYGRLZzrZfKlksj8oMf_0-f3WI_xBytu7bXZlqpveGkeVLT8pAvf9STwTW0I45BYtsng47I_gZHg]
Inserted with bypass_ripple: map[id:PenfClmVMUSgJd1G2PG7nsqPySCxLv5LLmSHv_TNlo6_YRgbtHFQNpq4n3zk9ZMzCtijc2oOgS2qy5gxafKg_A]
Inserted with TTL and bypass_ripple: map[id:QFdWBwvUR9B37Nwp4ouGKCMMmnb2sSzK2hZlCvFZJW2LlwMxeEv0HhL0PmDbw8ilFUbuqQ9wnXeYzDnNitVlLQ]
Updated with bypass_ripple: map[id:SH4aCv3j4GQYGRLZzrZfKlksj8oMf_0-f3WI_xBytu7bXZlqpveGkeVLT8pAvf9STwTW0I45BYtsng47I_gZHg name:map[type:String value:Product 1] price:map[type:Integer value:150]]
Deleted with bypass_ripple
Batch inserted with bypass_ripple: 2 records
Upserted with bypass_ripple: map[id:custom-id]
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields (id, name, email only)
  Found 3 active users
  Fields returned: [email id name]
  First user: Bob Smith <bob@example.com>

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: [age avatar_url bio created_at email id name status user_role]

Example 3: Complex query with projection (active users, ages 18-65)
  Found 3 active users (ages 18-65)
    - Dave Brown (age 45)
    - Alice Johnson (age 30)
    - Bob Smith (age 25)

Example 4: Query inactive users with profile fields
  Found 1 inactive users
    - Carol White: Manager

Example 5: Compare full vs projected data
  Full query:
    - 12 fields per record
    - Fields: [age api_key avatar_url bio created_at email id name password secret_token status user_role]
  Projected query:
    - 3 fields per record
    - Fields: [email id name]
  Bandwidth savings: ~75% fewer fields

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
