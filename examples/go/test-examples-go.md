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
Inserted: map[id:EQYuMW-fPjk12aS7b71aqGSFGLm3w6yNrnUq3FUdzRfb6hGM8rxr2vqFHwr6Z-L8YGW1Ntk1TdqO6TQOQTyFyA]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] id:EQYuMW-fPjk12aS7b71aqGSFGLm3w6yNrnUq3FUdzRfb6hGM8rxr2vqFHwr6Z-L8YGW1Ntk1TdqO6TQOQTyFyA name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query ===
Found documents: [map[active:map[type:Boolean value:true] id:EQYuMW-fPjk12aS7b71aqGSFGLm3w6yNrnUq3FUdzRfb6hGM8rxr2vqFHwr6Z-L8YGW1Ntk1TdqO6TQOQTyFyA name:map[type:String value:Test Record] value:map[type:Integer value:42]]]

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] id:EQYuMW-fPjk12aS7b71aqGSFGLm3w6yNrnUq3FUdzRfb6hGM8rxr2vqFHwr6Z-L8YGW1Ntk1TdqO6TQOQTyFyA name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
✓ simple_crud.go completed successfully

=== Running simple_websocket.go ===
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: 2607uzqLiNgA3EHl9LyxK5NVUvt89nwvEaMhDMaFrKA8Xtv_FEmk9dHI0YXFL8zCkfVGau0Imseo6D8hCx8Wpw

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "messageId": "1793594576329357000",
  "payload": {
    "data": [
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "2607uzqLiNgA3EHl9LyxK5NVUvt89nwvEaMhDMaFrKA8Xtv_FEmk9dHI0YXFL8zCkfVGau0Imseo6D8hCx8Wpw",
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

✅ Function saved: xrMp58xkPuYGtRIKVF1nQ0TnWneUvYaxP_VhgeXqf3JlC_5eJtwK8RrMBqYZhNvN3iFBAXDN4iHKpDClCGHfkQ
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Function saved: 9O02ILb4zwDad4TWwGP9HpiOURQClrQ_Z_um8UVstrZeZ3Jr8QfQ9SrGHUKMDdcNu7oGwh0f4g44aOn8pY-CiQ
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: XMxsUfkmeYeXFhgtKLd2-sYnVu0bF54w_HIH9c3qEEvMXeITzuqH9OWqJ7VSPZUnbAGhgAaztWEKU2WbMSRfnA
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
Collection created with first record: 5vqr15uQm8FVlnRO6PzS9cVszU8MaMmPLuPXR-non5UE9gjzlbdNA2yRQB_1CipLfZmlECwMZc0dVAPja91EtA

=== List Collections ===
Total collections: 13
Sample collections: [audit__ek0_testing chat_raw_completions__ek0_testing functions__ek0_testing chat_turns__ek0_testing chat_goal_templates__ek0_testing]

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ collection_management.go completed successfully

=== Running transactions.go ===
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: hxVdl8WzUV2NLrXarhnIkuA8Dzf9V8YNG_jLmm2mLrCcIrGaa9uKzJgIG0LgXUUFw7iu0yP0X0HbM48UW459ZA
Created Bob: $500 - ID: O3cXHlSrkIf2httdrytWuI7iNu17Afj1FCAXoNlzeJbjBAaPf9PP5ZA0tRRX7Vjpyeu5DiqKWQZRgok_hYJFlg

=== Example 1: Begin Transaction ===
Transaction ID: e0526af9-441e-48ae-a5ae-6a64533238e7

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
New transaction: 153f81ba-66d3-437f-acbc-f1a40c5fe0fc
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
   ✅ Function saved: haB-gObnH_13iOXZn3XPC-XEwhDz28HhMK0kJehVX9QBjevLPHD7YAlC4nAxbxAEdxyekfNRJwnaLJavUpfMDw

2️⃣ Calling function (Insert + Verify)...
   ✅ function executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: MKt3g9vW2V2eqmaNle_4huRTBCYLtCyg-t_VaE1oJZgCrkkAkNSeEhARolKPvI_hW0Mp3Y-g9HO1S96ePAR-sQ
   📋 Name: map[type:String value:Alice Smith]
   📋 Email: map[type:String value:alice@example.com]
   📋 Status: map[type:String value:pending]
   📋 Credits: map[type:Integer value:0]

============================================================
📝 function 2: Query + Update + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: fHLd6Zi7XFmPFZY3YTrjU3Rc_raM1usPaB-HQGlPJC9AOs4Z8ICpBBXUdY5oRPVdzE4wz5mByr1bLdIcTs3mlA

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
   ✅ Function saved: Y9rAfB4CrLX41gVoEFsMLKALbpt6hl5Z2U30fFj3vtKgDaTqW79ya1hzP__pDKTW7YwgUtIVsLqBghK5573B-Q

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
   ✅ Function saved: GaVNV5d6a9EpN6Jf0ryug_2IooHMnG4LHHmsjs3KXn3kmSGV1yo8YsOoOw7_X3V7mIWPJm2QP1xqKE0dMi5L2w

2️⃣ Calling function (Query + Delete + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: haB-gObnH_13iOXZn3XP...
   ✅ Deleted script: fHLd6Zi7XFmPFZY3YTrj...
   ✅ Deleted script: Y9rAfB4CrLX41gVoEFsM...
   ✅ Deleted script: GaVNV5d6a9EpN6Jf0ryu...
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
  Output: Document ID = HTxC5RSAgimk2vK8Kvz37w0ptZUKaLk0wjAIuHchS9pFbKo8mORgCKycdir-59uCYSw0lVFCyZzEFWeB2FbT2g
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: FindByID(HTxC5RSAgimk2vK8Kvz37w0ptZUKaLk0wjAIuHchS9pFbKo8mORgCKycdir-59uCYSw0lVFCyZzEFWeB2FbT2g)
  Output: Found document with name = map[type:String value:TTL Test]
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: FindByID(HTxC5RSAgimk2vK8Kvz37w0ptZUKaLk0wjAIuHchS9pFbKo8mORgCKycdir-59uCYSw0lVFCyZzEFWeB2FbT2g)
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
  Output: Document ID = y8vM3kLRBsasloFrz2h9BQq4xJ7hS2IAOHExrGEZzaXF8QeYFbwosHHxgQRPiD-bjlxduntVhxoigqCJxXJ13g
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: FindByID(y8vM3kLRBsasloFrz2h9BQq4xJ7hS2IAOHExrGEZzaXF8QeYFbwosHHxgQRPiD-bjlxduntVhxoigqCJxXJ13g)
  Output: Found document with name = map[type:String value:WS TTL Test]
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: FindByID(y8vM3kLRBsasloFrz2h9BQq4xJ7hS2IAOHExrGEZzaXF8QeYFbwosHHxgQRPiD-bjlxduntVhxoigqCJxXJ13g)
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
Admin permissions read successfully
=== ekoDB Advanced CRUD Integration Example (Go) ===

--- Inserting base record ---
Inserted record: YiJWtzPQou0LIfr9j6UIySHSGWQo2gWt8w8DVIY7sYTkaOBxcc3o570A8kXdylSWuyLs4Sh42R5BhyyLlgwIng

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
✓ Created session: Z6u0tGu1Ax9dBIXOvok_rFTWBT02qDG8nGJyAYg7c7v33FtClqf9QZszZFCShutBuhwwGRh_A1vdXZD6jCKJmA

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is **ekoDB** — a high-performance database product priced at **$99**.

✓ Second message sent

=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: ekoDB is priced at **99**. The context doesn’t specify a currency.

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: VYZ46HVLPGdZjMlU197xC8ac7OxjlLA4B3Qg3pFWEZ8kI2Ib5rf23u0R9m8lUnWYmXFUbCrhVGTFijIhjZkDdA
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 7

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 6

=== Cleanup ===
✓ Deleted chat session: VYZ46HVLPGdZjMlU197xC8ac7OxjlLA4B3Qg3pFWEZ8kI2Ib5rf23u0R9m8lUnWYmXFUbCrhVGTFijIhjZkDdA
✓ Deleted chat session: Z6u0tGu1Ax9dBIXOvok_rFTWBT02qDG8nGJyAYg7c7v33FtClqf9QZszZFCShutBuhwwGRh_A1vdXZD6jCKJmA
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: 3GyADvXNysEYxB-JzwbdOApYnpgdYRm0IQtMRtIvoSMTYUq6I5TX_yjbkQPOOzv2V7RxroTBP9ZXhuX8Kf_6tw

=== Sending Chat Message ===
Message ID: n8AlslRnAxSwVymeBXHE5l_S0zABtaTegX6IFfO7BoVXTF12yk1BJgudTi4sfU68IuVrwn6XnKVH75IGi0UWoA

=== AI Response ===
- **ekoDB** — $99
- **ekoDB Pro** — $299
- **ekoDB Cloud** — $499

=== Context Used (3 snippets) ===
  Snippet 1: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Fully managed cloud database service product id:PW9LwXnAlYZILP3Qyr5tMX0Su3XoFsc_pXgAlvwkVa1FHRtBoLbVbkIIVS5CSwKfyXuZQ8JDOuVNhloche6vVg name:ekoDB Cloud price:499] score:0.1111111111111111]
  Snippet 2: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Enterprise edition product with advanced features id:fmrZ4NUI7vwqTjgejmSwsL7uNmhKPdjlW3ZGTQsX1GBW1p-dDkpE6JlcXApYLFkA5xi_0QOzKEkAIEaIjuUi2A name:ekoDB Pro price:299] score:0.1111111111111111]
  Snippet 3: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:A high-performance database product with AI capabilities id:hnlE2x-69uIOT42DGBQGQgHO-OO5pTr-ZkR-2VOAiBTNGvDBJY2Z-E3Kf50nmTZXY1TUzzQZ-DzaqkuxRajl-A name:ekoDB price:99] score:0.1111111111111111]

Execution Time: 1438ms

=== Token Usage ===
Prompt tokens: 13717
Completion tokens: 32
Total tokens: 13749

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Message Stream (SSE) Example (Go) ===

Created session: dpxJ2khdMAzimL8fQ3xFSDuj2AKlX3OGX6Tq4IuVA-ON8ri7I6FgMiWoF2A3xJpawxsvior8Go-5fRiaB9MdtA

Streaming response for: 'What is ekoDB?'

{"__progress":"received"}{"__progress":"generating","model":"gpt-6-luna","provider":"openai"}**ekoDB** is a database project for storing application data. The name can refer to different products or projects, so the exact features depend on which one you mean. If you share a link or where you encountered it, I can identify that specific ekoDB.

--- Stream complete ---
Message ID: VZkzfRvmJwndyWQ4fKao11Y1p3MYIKULaHXst1tYykSg6kUXBarDddpWiNBDJEJp9CbY24kFX07PPB7R2rSuyw
Execution time: 9273ms
Context window: 128000 tokens

✓ Chat message stream example completed
✓ Client created

=== Get All Chat Models ===
OpenAI models: [gpt-3.5-turbo-0125 gpt-5.4-2026-03-05 gpt-5-nano gpt-5.5-pro gpt-audio-2025-08-28 gpt-4o-mini-transcribe-2025-03-20 gpt-4-turbo-2024-04-09 gpt-realtime-2.1-mini gpt-realtime-2025-08-28 gpt-4 tts-1-hd gpt-realtime-1.5 gpt-realtime gpt-5.2-pro gpt-5-chat-latest gpt-5.4-mini-2026-03-17 babbage-002 gpt-4o-2024-05-13 text-embedding-3-large tts-1 omni-moderation-2024-09-26 gpt-live-transcribe gpt-5.6-luna gpt-5-2025-08-07 gpt-5-mini gpt-5-mini-2025-08-07 omni-moderation-latest o3-mini-2025-01-31 gpt-audio-1.5 gpt-image-1.5 tts-1-1106 gpt-5.5-pro-2026-04-23 gpt-audio-mini-2025-10-06 gpt-5.2-pro-2025-12-11 o4-mini-2025-04-16 gpt-4o-2024-11-20 gpt-5.1 whisper-1 gpt-5-pro gpt-realtime-whisper gpt-realtime-2 gpt-5-search-api gpt-5.5-2026-04-23 gpt-4.1-nano gpt-image-2 gpt-3.5-turbo-16k text-embedding-ada-002 gpt-realtime-2.1 gpt-4.1 gpt-5.5 chat-latest gpt-realtime-translate gpt-5.4-pro-2026-03-05 gpt-4.1-mini gpt-4o-mini gpt-4o-mini-transcribe-2025-12-15 gpt-realtime-mini gpt-image-1-mini gpt-5.6-terra gpt-3.5-turbo-instruct-0914 gpt-4-turbo gpt-4o-2024-08-06 o3 gpt-4o-mini-search-preview-2025-03-11 o1-2024-12-17 gpt-5.1-chat-latest gpt-5-pro-2025-10-06 gpt-5.6-sol gpt-4o-transcribe-diarize gpt-5-codex gpt-4-0613 gpt-5.1-2025-11-13 gpt-4o-search-preview gpt-4o-mini-tts-2025-12-15 gpt-image-1 gpt-audio gpt-5.3-codex gpt-5-search-api-2025-10-14 gpt-5.4-pro gpt-4o-mini-tts gpt-realtime-mini-2025-12-15 gpt-5.1-codex-max sora-2 gpt-4o gpt-4o-mini-2024-07-18 gpt-4.1-2025-04-14 gpt-5 gpt-4o-mini-transcribe gpt-image-2-2026-04-21 gpt-5.2-chat-latest sora-2-pro gpt-3.5-turbo-instruct gpt-transcribe gpt-4.1-nano-2025-04-14 gpt-3.5-turbo-1106 gpt-4o-mini-tts-2025-03-20 gpt-5.2 gpt-5.4-nano-2026-03-17 o1 tts-1-hd-1106 gpt-5.2-codex gpt-5.4-mini o1-pro gpt-5.1-codex o4-mini-deep-research gpt-5.4 gpt-4o-search-preview-2025-03-11 text-embedding-3-small gpt-5-nano-2025-08-07 gpt-4o-transcribe o4-mini gpt-5.1-codex-mini gpt-6-astra gpt-5.3-chat-latest o3-2025-04-16 gpt-5.2-2025-12-11 chatgpt-image-latest gpt-3.5-turbo gpt-image-2.5-sunburst-2026-09-08 gpt-image-2.5-flare davinci-002 gpt-5.4-nano o3-mini o4-mini-deep-research-2025-06-26 gpt-audio-mini-2025-12-15 o1-pro-2025-03-19 gpt-live-1 gpt-image-2.5-sunburst gpt-image-2.5-flare-2026-09-08 gpt-6.1-terra gpt-audio-mini gpt-4.1-mini-2025-04-14 gpt-6-luna gpt-4o-mini-search-preview gpt-6-sol gpt-6.1-sol]
Anthropic models: [claude-haiku-5-5 claude-sonnet-5-5 claude-opus-5-5 claude-fable-5-1 claude-opus-5 claude-sonnet-5 claude-fable-5 claude-opus-4-8 claude-opus-4-7 claude-sonnet-4-6 claude-opus-4-6 claude-opus-4-5-20251101 claude-haiku-4-5-20251001 claude-sonnet-4-5-20250929]
Perplexity models: []
Gemini models: [gemini-2.5-flash gemini-2.5-pro gemini-2.5-flash-preview-tts gemini-2.5-pro-preview-tts gemma-4-26b-a4b-it gemma-4-31b-it gemini-flash-latest gemini-flash-lite-latest gemini-pro-latest gemini-2.5-flash-lite gemini-2.5-flash-image gemini-3-flash-preview gemini-3.1-pro-preview gemini-3.1-pro-preview-customtools gemini-3.1-flash-lite-preview gemini-3.1-flash-lite gemini-3-pro-image-preview gemini-3-pro-image nano-banana-pro-preview gemini-3.1-flash-image-preview gemini-3.1-flash-image gemini-3.1-flash-lite-image gemini-nano-banana-2.1 gemini-3.5-flash gemini-3.5-flash-lite gemini-omni-flash-preview gemini-omni-1.1-flash gemini-3.5-transcribe gemini-3.6-flash gemini-3.7-flash gemini-3.8-flash gemini-4-argon lyria-3-clip-preview lyria-3-pro-preview lyria-3.5 gemini-3.1-flash-tts-preview gemini-3.8-flash-tts gemini-3.8-flash-lite-tts gemini-robotics-er-2-preview gemini-2.5-computer-use-preview-10-2025 antigravity-preview-05-2026 antigravity-preview-09-2026 antigravity-preview-latest deep-research-max-preview-04-2026 deep-research-preview-04-2026 deep-research-pro-preview-12-2025 gemini-embedding-001 gemini-embedding-2-preview gemini-embedding-2 aqa gemini-3.5-transcribe-live gemini-2.5-flash-native-audio-latest gemini-2.5-flash-native-audio-preview-09-2025 gemini-2.5-flash-native-audio-preview-12-2025 gemini-3.1-flash-live-preview gemini-3.8-live gemini-3.8-live-extended-thinking gemini-robotics-er-2-streaming-preview gemini-3.5-live-translate-preview lyria-realtime-exp]
  anthropic: ok 14 models
  gemini: ok 60 models
  openai: ok 136 models
  perplexity: not_configured (unverified) No Perplexity API Key

=== Get OpenAI Models ===
OpenAI models: [gpt-3.5-turbo-0125 gpt-5.4-2026-03-05 gpt-5-nano gpt-5.5-pro gpt-audio-2025-08-28 gpt-4o-mini-transcribe-2025-03-20 gpt-4-turbo-2024-04-09 gpt-realtime-2.1-mini gpt-realtime-2025-08-28 gpt-4 tts-1-hd gpt-realtime-1.5 gpt-realtime gpt-5.2-pro gpt-5-chat-latest gpt-5.4-mini-2026-03-17 babbage-002 gpt-4o-2024-05-13 text-embedding-3-large tts-1 omni-moderation-2024-09-26 gpt-live-transcribe gpt-5.6-luna gpt-5-2025-08-07 gpt-5-mini gpt-5-mini-2025-08-07 omni-moderation-latest o3-mini-2025-01-31 gpt-audio-1.5 gpt-image-1.5 tts-1-1106 gpt-5.5-pro-2026-04-23 gpt-audio-mini-2025-10-06 gpt-5.2-pro-2025-12-11 o4-mini-2025-04-16 gpt-4o-2024-11-20 gpt-5.1 whisper-1 gpt-5-pro gpt-realtime-whisper gpt-realtime-2 gpt-5-search-api gpt-5.5-2026-04-23 gpt-4.1-nano gpt-image-2 gpt-3.5-turbo-16k text-embedding-ada-002 gpt-realtime-2.1 gpt-4.1 gpt-5.5 chat-latest gpt-realtime-translate gpt-5.4-pro-2026-03-05 gpt-4.1-mini gpt-4o-mini gpt-4o-mini-transcribe-2025-12-15 gpt-realtime-mini gpt-image-1-mini gpt-5.6-terra gpt-3.5-turbo-instruct-0914 gpt-4-turbo gpt-4o-2024-08-06 o3 gpt-4o-mini-search-preview-2025-03-11 o1-2024-12-17 gpt-5.1-chat-latest gpt-5-pro-2025-10-06 gpt-5.6-sol gpt-4o-transcribe-diarize gpt-5-codex gpt-4-0613 gpt-5.1-2025-11-13 gpt-4o-search-preview gpt-4o-mini-tts-2025-12-15 gpt-image-1 gpt-audio gpt-5.3-codex gpt-5-search-api-2025-10-14 gpt-5.4-pro gpt-4o-mini-tts gpt-realtime-mini-2025-12-15 gpt-5.1-codex-max sora-2 gpt-4o gpt-4o-mini-2024-07-18 gpt-4.1-2025-04-14 gpt-5 gpt-4o-mini-transcribe gpt-image-2-2026-04-21 gpt-5.2-chat-latest sora-2-pro gpt-3.5-turbo-instruct gpt-transcribe gpt-4.1-nano-2025-04-14 gpt-3.5-turbo-1106 gpt-4o-mini-tts-2025-03-20 gpt-5.2 gpt-5.4-nano-2026-03-17 o1 tts-1-hd-1106 gpt-5.2-codex gpt-5.4-mini o1-pro gpt-5.1-codex o4-mini-deep-research gpt-5.4 gpt-4o-search-preview-2025-03-11 text-embedding-3-small gpt-5-nano-2025-08-07 gpt-4o-transcribe o4-mini gpt-5.1-codex-mini gpt-6-astra gpt-5.3-chat-latest o3-2025-04-16 gpt-5.2-2025-12-11 chatgpt-image-latest gpt-3.5-turbo gpt-image-2.5-sunburst-2026-09-08 gpt-image-2.5-flare davinci-002 gpt-5.4-nano o3-mini o4-mini-deep-research-2025-06-26 gpt-audio-mini-2025-12-15 o1-pro-2025-03-19 gpt-live-1 gpt-image-2.5-sunburst gpt-image-2.5-flare-2026-09-08 gpt-6.1-terra gpt-audio-mini gpt-4.1-mini-2025-04-14 gpt-6-luna gpt-4o-mini-search-preview gpt-6-sol gpt-6.1-sol]

=== Get Anthropic Models ===
Anthropic models: [claude-haiku-5-5 claude-sonnet-5-5 claude-opus-5-5 claude-fable-5-1 claude-opus-5 claude-sonnet-5 claude-fable-5 claude-opus-4-8 claude-opus-4-7 claude-sonnet-4-6 claude-opus-4-6 claude-opus-4-5-20251101 claude-haiku-4-5-20251001 claude-sonnet-4-5-20250929]

✓ Chat Models API example complete
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: S0D0VTWzlmBKodithumXLurHAO5ffrDKcPn554xzZ23M4dP6ZRjwcil2a61okyxoQJgxer6VQv6WnC97_9m8lA

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is **ekoDB**, described as a high-performance database product. It’s priced at **$99**.

✓ Message 2 sent
  Response: ekoDB is priced at **$99**.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: I8MAdV0cMudO3W91DiFsfT-HB2yXrPoOiQJvk-DVJhAlIJB_ckVwvK-SLIXSbJBXbjO65i9pMJ3rPwrnOde_pA
  Parent: S0D0VTWzlmBKodithumXLurHAO5ffrDKcPn554xzZ23M4dP6ZRjwcil2a61okyxoQJgxer6VQv6WnC97_9m8lA

=== Listing Sessions ===
✓ Found 2 sessions
  Session 1: I8MAdV0cMudO3W91DiFsfT-HB2yXrPoOiQJvk-DVJhAlIJB_ckVwvK-SLIXSbJBXbjO65i9pMJ3rPwrnOde_pA (Untitled)
  Session 2: S0D0VTWzlmBKodithumXLurHAO5ffrDKcPn554xzZ23M4dP6ZRjwcil2a61okyxoQJgxer6VQv6WnC97_9m8lA (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: I8MAdV0cMudO3W91DiFsfT-HB2yXrPoOiQJvk-DVJhAlIJB_ckVwvK-SLIXSbJBXbjO65i9pMJ3rPwrnOde_pA

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ All session management operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: lR-UwRVouVsYSr_AQJyVuDdkY7-zuYR4bRaA4VMMiw7GWfKhdZfIS5fIeQF-LlU_77oXi3YXHrhbPmqKCuosjA

=== List Collections ===
Total collections: 13
Sample collections: [audit__ek0_testing chat_raw_completions__ek0_testing functions__ek0_testing chat_turns__ek0_testing chat_goal_templates__ek0_testing]

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
✓ Created record with native map: map[id:U1FYw5EZjKIkCCngtW4JVKkM4ToUO_PCXbrpaOm0Og8IZ-q4omZqyMuvjEUclZB-cOyV2f7vUiLAG9IG6fqp3A]

=== Upsert Operation ===
✓ First upsert (update): map[active:map[type:Boolean value:true] age:map[type:Integer value:29] email:map[type:String value:alice.j@newdomain.com] id:U1FYw5EZjKIkCCngtW4JVKkM4ToUO_PCXbrpaOm0Og8IZ-q4omZqyMuvjEUclZB-cOyV2f7vUiLAG9IG6fqp3A name:map[type:String value:Alice Johnson]]
✓ Second upsert (insert): map[id:new-user-1793594636951667000]

=== Find One Operation ===
✓ Found user by email: map[active:map[type:Boolean value:true] age:map[type:Integer value:29] email:map[type:String value:alice.j@newdomain.com] id:U1FYw5EZjKIkCCngtW4JVKkM4ToUO_PCXbrpaOm0Og8IZ-q4omZqyMuvjEUclZB-cOyV2f7vUiLAG9IG6fqp3A name:map[type:String value:Alice Johnson]]
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
✓ Inserted document: Sgkk_Euqpr7RvVbvxqSRQc5rR7MTWHU_kq0lvNFWE8vghzvZPdx0agd57DSGJfPXAOMlT5PkxshBn22TIyYSgw

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: kqE5Mz5c0EshGCepK-_P97WFI1-eK_scj_XiI76TKsHQWk4BKbPRycuxFysIZpT2Ni5C7UuTi4ArgOZU0DLqnA

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
✓ Edge cache script created: vOzh4GaVFzxvBSHKBlLItEEZWklHwLdLA4R2wj_KkALyUQ_3jHHfw5DJHAK7GBTXjHKvZAHUgj8tbKHG5k516g

Call 1: Cache lookup
Response time: 1ms
Found 1 cached entries

Call 2: Cache lookup (connection warm)
Response time: 1ms
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
   ⏱️  Duration: 59.932333ms
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
   ⏱️  Duration: 2.30475ms
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
   🚀 Cache speedup: 29.5x faster!

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

✅ Function saved: FZVLIQaugVCnpDnyHCZHkyS8Cuu5KhkqzFMhNu1z1vpXENw085Y8S2kmG43MrcIc5RZYnmikXr0WAcFAkIuXtw
📊 Found 5 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Function

✅ Function saved
📊 Found 3 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Function

✅ Function saved
📊 Statistics: 2 groups
   map[avg_score:map[type:Float value:60] count:map[type:Integer value:5] status:map[type:String value:active]]
   map[avg_score:map[type:Float value:50] count:map[type:Integer value:5] status:map[type:String value:inactive]]
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
   map[avg_price:map[type:Float value:367] category:map[type:String value:Electronics] count:map[type:Integer value:5]]
   map[avg_price:map[type:Float value:365.6666666666667] category:map[type:String value:Furniture] count:map[type:Integer value:3]]
⏱️  Execution time: 0ms

✅ All advanced script examples finished!
🧹 Cleaning up...
✅ Cleanup complete
🚀 ekoDB Go AI Functions Example

📋 Setting up test data...
✅ Created 2 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
🤖 AI Response:
   Vector databases are useful for searching data by **meaning or similarity**, rather than only by exact keywords or matching fields. Benefits include:

- **Semantic search:** Finds relevant text, images, audio, or other content even when it uses different wording.
- **Fast similarity lookup at scale:** Indexes and approximate-nearest-neighbor algorithms make searches over large embedding collections practical.
- **Metadata filtering:** Combines similarity search with filters such as date, category, or access permissions.
- **AI application support:** Helps retrieve relevant context for retrieval-augmented generation (RAG), recommendations, and personalization.
- **Multimodal search:** Can search across different data types when they’re represented as vectors.

They’re most useful when similarity-based retrieval is central; they don’t replace relational databases for transactional queries or structured data management.
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embed script saved
📊 Generated 2 embeddings
   Dimensions: 1536
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

✅ Function saved: RzDGgMaPNJIIb4faNv9qXPnyJpuT1tqJD3z0DRcVT-sCq2s-zZGOThu9ruOP_37t6czwUyCnfRrcdMs5lJg1vw
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

✅ Inserted order: 0MteaWOLkeNSc_ZZpslYXzCMorKxNTr5NXHWenza7-bJOIWCSaLdh4d3VDx4xAhHkpIKA79QSalAhxpyTnJd4g
✅ Inserted 2 products with wrapped types

📝 Example 2: function with Wrapped Type Parameters

✅ Function saved: NydKlN7Ht6Q9YFa91Lo-BnNQuw3wGxwNLzevumLw-tQVx3Osk8a1SnwkPduioXAGcSdaBRlqzn85MFt6NuP4iw
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: map[type:Object value:map[role:admin userId:user_abc]]
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session
📝 Example 4: KV Operations in Functions

✅ Function saved: iFpjyhUG1Ve59GL9AVgwS5WpAYihp5CfadIUO1f4hzRD6gdQCcgQ9z_WyyL-yY0rM1IrserASE_ERGwDt1FQwA
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Function

✅ Function saved: 5mqYQA-o4hrJxalGOB2EDfgRqGSyc0nerqn5jwDC7Z7lrUBLUbmp_PvlHAL_70JSzhR7bSYjmLJktxAQcubKYg
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
   1. map[type:String value:Vector Databases Explained] (map[type:String value:Database])
   2. map[type:String value:Natural Language Processing] (map[type:String value:AI])
   3. map[type:String value:Database Design Principles] (map[type:String value:Database])
   4. map[type:String value:Getting Started with ekoDB] (map[type:String value:Database])
   5. map[type:String value:Introduction to Machine Learning] (map[type:String value:AI])
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
Created template: Data Migration (id: OfGQSeWDFmFvgjznnQxqd6pzsRZrG7fwXxtVwjZ5p0swKsHoB5SmZCPxRgHAO4VKsGalIaEc4bB2HSU9Lp61fA)

--- Listing templates ---
Templates: map[count:1 items:[map[description:map[type:String value:Template for migrating data between schemas] id:OfGQSeWDFmFvgjznnQxqd6pzsRZrG7fwXxtVwjZ5p0swKsHoB5SmZCPxRgHAO4VKsGalIaEc4bB2HSU9Lp61fA owner:map[type:Object value:map[key_id:3a0cdba0da7c060610042783cc758f1f key_label:default_admin]] steps:map[type:Array value:[map[description:Analyze source schema] map[description:Create target schema] map[description:Migrate records] map[description:Validate results]]] title:map[type:String value:Data Migration]]] total:1]

--- Getting template ---
Fetched: Data Migration

--- Updating template ---
Updated description: Updated: comprehensive data migration workflow

--- Deleting template ---
Template deleted successfully

✓ Goal template CRUD example completed
=== ekoDB Goals, Tasks & Agents Integration Example (Go) ===

--- Creating goal ---
Created goal: Deploy v2.0 (id: 3a24Hs4sj2uVhws7n7Q3tJIArDnDhudpqjJD549jtQSvwjwlkpv1EJ6zWhkO4c7g4mTmBqb0wqcmdHzrTjxSFg)

--- Listing goals ---
Goals response: map[count:1 goals:[map[created_at:map[type:DateTime value:2026-11-02T04:44:05.340516+00:00] description:map[type:String value:Ship the v2.0 release to production] id:3a24Hs4sj2uVhws7n7Q3tJIArDnDhudpqjJD549jtQSvwjwlkpv1EJ6zWhkO4c7g4mTmBqb0wqcmdHzrTjxSFg owner:map[type:Object value:map[key_id:3a0cdba0da7c060610042783cc758f1f key_label:default_admin]] status:map[type:String value:pending] steps:map[type:String value:[{"description":"Run integration tests"},{"description":"Build release artifacts"},{"description":"Deploy to staging"}]] title:map[type:String value:Deploy v2.0] updated_at:map[type:DateTime value:2026-11-02T04:44:05.340516+00:00]]] total:1]

--- Getting goal ---
Fetched goal: Deploy v2.0

--- Updating goal ---
Updated description: Ship the v2.0 release to production (with hotfix)

--- Searching goals ---
Search result: map[count:1 items:[map[_score:12.870000000000001 created_at:map[type:DateTime value:2026-11-02T04:44:05.340516+00:00] description:map[type:String value:Ship the v2.0 release to production (with hotfix)] id:3a24Hs4sj2uVhws7n7Q3tJIArDnDhudpqjJD549jtQSvwjwlkpv1EJ6zWhkO4c7g4mTmBqb0wqcmdHzrTjxSFg owner:map[type:Object value:map[key_id:3a0cdba0da7c060610042783cc758f1f key_label:default_admin]] status:map[type:String value:pending] steps:map[type:String value:[{"description":"Run integration tests"},{"description":"Build release artifacts"},{"description":"Deploy to staging"}]] title:map[type:String value:Deploy v2.0] updated_at:map[type:DateTime value:2026-11-02T04:44:05.350904+00:00]]]]

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
Created task: Run benchmarks (id: ZR734MorV7nDFuKbeapDHNKnJzecpjDSkaxqNNf2RQ8bTN0ZXzTZgaCQDY0jvrTP0NBqDr_3NmxFTQ0jqsxJ7g)

--- Listing tasks ---
Tasks: map[count:1 items:[map[description:map[type:String value:Execute YCSB benchmarks against staging] due_at:map[type:DateTime value:2026-11-03T04:44:05+00:00] id:ZR734MorV7nDFuKbeapDHNKnJzecpjDSkaxqNNf2RQ8bTN0ZXzTZgaCQDY0jvrTP0NBqDr_3NmxFTQ0jqsxJ7g name:map[type:String value:Run benchmarks] owner:map[type:Object value:map[key_id:3a0cdba0da7c060610042783cc758f1f key_label:default_admin]]]] total:1]

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
Created agent: benchmark-runner (id: isSsFdf3IlQUQou3k6zOfe9ITJbECnClFkJnPckWBRCrYMconXznmOlb6u045nXd-Vcq3ITpjZ_GoSQAuz2Fjw)

--- Listing agents ---
Agents: map[count:1 items:[map[deployment_id:map[type:String value:deploy_prod_1] description:map[type:String value:Runs periodic benchmarks] id:isSsFdf3IlQUQou3k6zOfe9ITJbECnClFkJnPckWBRCrYMconXznmOlb6u045nXd-Vcq3ITpjZ_GoSQAuz2Fjw llm_model:map[type:String value:gpt-6-luna] name:map[type:String value:benchmark-runner] owner:map[type:Object value:map[key_id:3a0cdba0da7c060610042783cc758f1f key_label:default_admin]] run_policy:map[type:String value:{"trigger":{"allow_write_operations":false,"write_tools":[]},"eval":{"allow_write_operations":false,"write_tools":[]},"customer":{"allow_write_operations":false,"write_tools":[]}}]]] total:1]

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
Inserted doc1: HUgR484VcS7aSlpYBG3b8BijCx9QL4p7lP1MluSyPffLf9Tey5yaogkHLQ_b03lN3Ltves3T91z1E2yFYb8IDw
Inserted doc2: 0A40WuFJPoYjCv6w2epcSWBE33Yg1OC_cHPQK6WbidvNLJ79670auG-SCzHbooTFUfdh4hbZ9ZajjvUUDXlgSQ

--- Setting KV entry ---
KV entry set: team:engineering:go

--- Linking KV key to documents ---
Linked doc1: map[]
Linked doc2: map[]

--- Getting links ---
Links for team:engineering:go: [map[collection:kv_links_example_go created_at:2026-11-02T04:44:06.156809Z document_id:HUgR484VcS7aSlpYBG3b8BijCx9QL4p7lP1MluSyPffLf9Tey5yaogkHLQ_b03lN3Ltves3T91z1E2yFYb8IDw field_path:<nil> last_accessed:2026-11-02T04:44:06.158108Z metadata:map[]] map[collection:kv_links_example_go created_at:2026-11-02T04:44:06.157540Z document_id:0A40WuFJPoYjCv6w2epcSWBE33Yg1OC_cHPQK6WbidvNLJ79670auG-SCzHbooTFUfdh4hbZ9ZajjvUUDXlgSQ field_path:<nil> last_accessed:2026-11-02T04:44:06.158108Z metadata:map[]]]

--- Unlinking doc2 ---
Unlink result: map[]

--- Verifying links after unlink ---
Links after unlink: [map[collection:kv_links_example_go created_at:2026-11-02T04:44:06.156809Z document_id:HUgR484VcS7aSlpYBG3b8BijCx9QL4p7lP1MluSyPffLf9Tey5yaogkHLQ_b03lN3Ltves3T91z1E2yFYb8IDw field_path:<nil> last_accessed:2026-11-02T04:44:06.159392Z metadata:map[]]]

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
Response: For traditional painting, the primary colors are **red, yellow, and blue**. For light and screens, they’re **red, green, and blue**.

--- Structured Output via SSE ---
JSON response: [
  {"name": "Earth", "diameter_km": 12742},
  {"name": "Mars", "diameter_km": 6779},
  {"name": "Venus", "diameter_km": 12104}
]

--- Blocking HTTP (for comparison) ---
Blocking response: Hello!

=== Done ===
=== ekoDB Schedules Integration Example (Go) ===

--- Creating schedule ---
Created schedule: nightly-backup (id: 81ac37ec-63c9-41f9-b479-2ab614e8951c)

--- Listing schedules ---
Schedules: map[count:1 schedules:[map[created_at:2026-11-02T04:44:11.674728Z cron_expression:0 0 0 * * * description:Runs a database backup every night at midnight enabled:true function_label:schedule_noop_go_41786 id:81ac37ec-63c9-41f9-b479-2ab614e8951c last_execution:<nil> name:nightly-backup next_execution:2026-11-03T00:00:00Z parameters:map[] stats:map[avg_execution_time_ms:0 failed_executions:0 last_error:<nil> successful_executions:0 total_executions:0] timezone:UTC updated_at:2026-11-02T04:44:11.674728Z]]]

--- Getting schedule ---
Fetched schedule: nightly-backup (cron: 0 0 0 * * *)

--- Updating schedule ---
Updated cron: 0 0 2 * * *

--- Triggering schedule ---
Trigger response: map[schedule_id:81ac37ec-63c9-41f9-b479-2ab614e8951c status:triggered]

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
  1. Score: 0.750
  2. Score: 0.749
  3. Score: 0.745

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.500
  2. Score: 0.900
  3. Score: 0.298

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 13.200

9. Vector search with a metadata pre-filter (category = ml):
Found 2 documents in category "ml" (NLP excluded)
  1. Introduction to Machine Learning (category: ml)
  2. Deep Learning Fundamentals (category: ml)

Paired vector search: document=Rust Programming Answer, query=Rust Programming Question
Document search execution path: index
Query search execution path: index

✅ Search examples completed!
=== Cleanup ===
✅ Deleted test collections
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: map[id:8OMhIHYOBgKbId93b9vSWAnVmsiFp2kw4dKF32bT0Bf1AcdeCEEVUF2JDVwgxKycZ9wfn19_RSvFIjfYjX066Q]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-11-02T04:44:12+00:00] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:8OMhIHYOBgKbId93b9vSWAnVmsiFp2kw4dKF32bT0Bf1AcdeCEEVUF2JDVwgxKycZ9wfn19_RSvFIjfYjX066Q metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Test Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:42]]

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.990000
  created_at (DateTime): 2026-11-02 04:44:12 +0000 +0000
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1 tag2 tag3]
  metadata (Object): map[key:value nested:map[deep:true]]
  embedding (Vector): [0.1 0.2 0.3 0.4 0.5]
  categories (Set): [electronics computers]
  data (Bytes): 11 bytes
Plain record: map[active:true categories:[electronics computers] created_at:2026-11-02T04:44:12+00:00 data:aGVsbG8gd29ybGQ= embedding:[0.1 0.2 0.3 0.4 0.5] id:8OMhIHYOBgKbId93b9vSWAnVmsiFp2kw4dKF32bT0Bf1AcdeCEEVUF2JDVwgxKycZ9wfn19_RSvFIjfYjX066Q metadata:map[key:value nested:map[deep:true]] name:Test Record price:99.99 tags:[tag1 tag2 tag3] user_id:550e8400-e29b-41d4-a716-446655440000 value:42]

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-11-02T04:44:12+00:00] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:8OMhIHYOBgKbId93b9vSWAnVmsiFp2kw4dKF32bT0Bf1AcdeCEEVUF2JDVwgxKycZ9wfn19_RSvFIjfYjX066Q metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Updated Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: pLvVNJfXuCqClt-nLlfgXq1Aa3BtHTErXjSNJJ9zX8kV9OcIXu70qti1hkbTMDbQxCENsdW4-5lpCcOUrN6mbQ

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
✓ Created native SWR script: github_user_native_go (2JpYVYFX5PACspq21Saq4XP2C8voxBYsEzsPK7BDKtz-BG6-rtVGyAN03g0zuM70xcyuaI6BOnJr4FJv9DwXHg)

First call (cache miss - will fetch from GitHub API):
  Response time: 149ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 2ms
  Speedup: 74.5x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit_go (oz0UFUTYL2_ltdDB_K9EgcQQ8OZgyqdTeST6y94tcwKl0vZxABItYyhMAILL1lHF_xS_YuKTVKB1lmQ-YtQJbw)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail_go' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline_go (_TVP5kmP6ha1sJJ74ZBIJlQXeP1PyJJJ1R6WoaPV3eNM3RC4dSojcrhyXMHfHt-i2ha4sPz-G_EAARq4vFi3HQ)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users_go' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache_go (p6BwTW2lP1QBIu210KBURF1wCBGrM3oV6F3ScbT2CnJ-wOfhCJ3Sgh6pUuX4k8lL82CCx4qODHRBj60tvr2RtQ)
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
✓ Created SWR script: fetch_api_user_go (EteSWU9UkvgQc9Vd39diI4CT6rPdDJCt_H39Kcl3Sm9xG9TZn_56-VceYhXz2Ym8QnEaKbgOqaxj9KRjifjScA)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "cached_at": {
        "type": "DateTime",
        "value": "2026-11-02T04:44:13+00:00"
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
Response time: 2ms (served from cache)
✓ Lightning fast cache hit

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation
🧹 Cleaning up...
✓ Cleanup complete

✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: _LSmNIPn8F9rp1JMeXqfqcoIqscFcRxqbJAmygmW6bdxsdAuE3WrqGeuNByMowzKyjLWwvps1ls2Dg0JAQRTVA
Created Bob: $500 - ID: J8F4Lq17uUYGMX0DJdMEJxmmztITbnGQqP3CFEpY-QdCtm5eXUkWWOVXcEoe49jCg1qLTDdw5n_M_rMXvDskgw

=== Example 1: Begin Transaction ===
Transaction ID (server-default isolation): ecce0d48-fa38-4bac-9f38-dd788ca9bd99

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
New transaction: 8ebcdc4e-22a0-4fc1-93ce-38ba8fbb0af7
Updated Bob: $700 → $600 (in transaction)
Status before rollback: Active
✓ Transaction rolled back

✓ Verified Bob remains $700 after rollback

=== Cleanup ===
✓ Deleted test account collection

✓ All client transaction examples completed
✓ Client created

=== Create User Function ===
Created user function with ID: UzFNtN-USr8z5VQ0g9Qmpk7EoLARZzfjPnakgO2OO7xYrliLjmWKcC-gIWfvm7oExeNcowPtLgTjeMnYxLHlkQ

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

Created chat session: mFA_KK8USO_mudkJ4uhDwedS0T0eTOQI2jXqL1jmTD72r5cPn-5D2g_k5xY2Zx6R8bnbgJ5Pz5qZMxY3nJQkgQ

Sending message: 'What is the capital of France?'
{"__progress":"received"}{"__progress":"generating","model":"gpt-5.6-luna","provider":"openai"}The capital of France is **Paris**.

--- Stream ended ---
Message ID: O_IZRCia9mllJoaNsjIjq6imEpf5nR81L3glqTDe3TUNO6O8pfe_5lrncIxtg_cIHeVXDnw_TG3L6Tv03O2Dqw
Execution time: 905ms
Token usage: {"cached_input_tokens":0,"completion_tokens":13,"prompt_tokens":23,"thinking_tokens":0,"total_tokens":36}

Full response: {"__progress":"received"}{"__progress":"generating","model":"gpt-5.6-luna","provider":"openai"}The capital of France is **Paris**....
=== WebSocket Subscription Example (Go) ===

✓ Authentication successful
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_go' ===
✓ Subscribed (subscription_id: sub_3363d3230a2b47ea9067b8f520d38ca9)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: 58J8ThlZkftLA2CXMV1vHa0yP-oPSJCDZXxx4Wu6ChDlQjH0JKBgRf5IWkxrVSJA9lhm-xeJ8f9C5Iaefv4UUA

  📡 Notification received:
     Event:      insert
     Collection: ws_subscribe_example_go
     Record IDs: [58J8ThlZkftLA2CXMV1vHa0yP-oPSJCDZXxx4Wu6ChDlQjH0JKBgRf5IWkxrVSJA9lhm-xeJ8f9C5Iaefv4UUA]
     Timestamp:  2026-11-02T04:44:15.596856+00:00

Inserting record 2...
✓ Inserted: WPUmFo3xP8k4WJ51chy_6SF4r3b42lehQGufwvL5epDkgVCDDiz-Eqgegi3aGi9SpIeHFnGpavxpC0Hvb1G_XA

  📡 Notification received:
     Event:      insert
     Record IDs: [WPUmFo3xP8k4WJ51chy_6SF4r3b42lehQGufwvL5epDkgVCDDiz-Eqgegi3aGi9SpIeHFnGpavxpC0Hvb1G_XA]

=== Unsubscribing ===
✓ Unsubscribed

✓ WebSocket subscription example completed successfully
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: WgcLd5dLo2U73i96z5tnhBVEEc15-8FTSmmIjIvftLlZAHSWDSDupgpi-Bn8czZ-r0pZHpGRlAzni8bjE9maAg

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 5 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
Inserted with ripple: map[id:bjFLMpRjvbIFP2qXQ8s5nuXruqr1KXxeP0fqShBH8Ry5p8jrajDrAEiRtNXhAWlGTMJG1LGP6OEBaTt59UIbGQ]
Inserted with bypass_ripple: map[id:bFKbfdKN4xIKH0J53G4dKQtoGmdDTjDEPpUlcl1ZCQGYaT-sIm8qnKtG9ntX-v5ocsAepG5a1UkJIMg-fKRhJw]
Inserted with TTL and bypass_ripple: map[id:cRioqVuvZiFBlGlKv_QRPpn2HJaVNrhwtiYttMaawkSF0WigqH8XT_DxZNUrz24dN5O373LNkIRtiWAAqcaC1A]
Updated with bypass_ripple: map[id:bjFLMpRjvbIFP2qXQ8s5nuXruqr1KXxeP0fqShBH8Ry5p8jrajDrAEiRtNXhAWlGTMJG1LGP6OEBaTt59UIbGQ name:map[type:String value:Product 1] price:map[type:Integer value:150]]
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
