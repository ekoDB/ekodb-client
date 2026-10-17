make test-examples-javascript
🧪 Running JavaScript examples (direct HTTP/WebSocket)...

added 1 package, removed 1 package, and audited 9 packages in 399ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities

╔════════════════════════════════════════╗
║  ekoDB JavaScript Examples Test Suite ║
╚════════════════════════════════════════╝

=== Checking Server Connection ===
✓ Server is ready

=== Getting Authentication Token ===
✓ Authentication successful

=== Running 10 Examples ===

=== Running ekoDB/ekodb-client/examples/javascript/simple_crud.js ===
=== Simple CRUD Operations (Direct HTTP) ===

✓ Authentication successful

=== Insert Document ===
Inserted: {
  id: '2KXR9L4e3MGW2LSKTScyzN7fKGC6ETzaXfzdBBQNBgwODUrd20_4X7NbJ9-2imog3CjeNOUcGZUc9y6aNSGa0w'
}

=== Find by ID ===
Found: {
  name: { type: 'String', value: 'Test Record' },
  value: { value: 42, type: 'Integer' },
  id: '2KXR9L4e3MGW2LSKTScyzN7fKGC6ETzaXfzdBBQNBgwODUrd20_4X7NbJ9-2imog3CjeNOUcGZUc9y6aNSGa0w',
  active: { value: true, type: 'Boolean' }
}

=== Find with Query ===
Found documents: [
  {
    active: { value: true, type: 'Boolean' },
    name: { value: 'Test Record', type: 'String' },
    value: { value: 42, type: 'Integer' },
    id: '2KXR9L4e3MGW2LSKTScyzN7fKGC6ETzaXfzdBBQNBgwODUrd20_4X7NbJ9-2imog3CjeNOUcGZUc9y6aNSGa0w'
  }
]

=== Update Document ===
Updated: {
  active: { type: 'Boolean', value: true },
  value: { value: 100, type: 'Integer' },
  name: { type: 'String', value: 'Updated Record' },
  id: '2KXR9L4e3MGW2LSKTScyzN7fKGC6ETzaXfzdBBQNBgwODUrd20_4X7NbJ9-2imog3CjeNOUcGZUc9y6aNSGa0w'
}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
✓ simple_crud.js completed successfully

=== Running ekoDB/ekodb-client/examples/javascript/simple_websocket.js ===
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: QoFraBADew3StK1_yKjjOrxb8VvF1iTMsCkOwuiBnjLfS2mydY-lF5It8Ciw7jSt_yt4Q2gsZv7P6kFjkPoT9w

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "type": "Success",
  "payload": {
    "data": [
      {
        "value": {
          "type": "Integer",
          "value": 42
        },
        "active": {
          "value": true,
          "type": "Boolean"
        },
        "id": "QoFraBADew3StK1_yKjjOrxb8VvF1iTMsCkOwuiBnjLfS2mydY-lF5It8Ciw7jSt_yt4Q2gsZv7P6kFjkPoT9w",
        "name": {
          "value": "WebSocket Test Record",
          "type": "String"
        }
      }
    ]
  },
  "messageId": "1792225957658"
}
✓ Retrieved 1 record via WebSocket

✓ WebSocket example completed successfully
WebSocket closed
✓ simple_websocket.js completed successfully

=== Running ekoDB/ekodb-client/examples/javascript/http_functions.js ===
🚀 ekoDB Functions Example (JavaScript/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Function saved: 3gLdiSDW9KYdRRKhejGhKUSJyQglanQnI9C0iJVnuW97HAer4yLBdKtc7bSgB8PLzBfGH8XTHB7XMRvHXMvPrQ
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Function saved: OqJTxdgo5b_b5iB2TXzZM1L1ZbPRMaztoh6_1HqEX0ht9cRSZf2kbl6-JnWJpx6aU6hPnI7QDsYBnYPYKQ_nqA
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: 6Why51EzmoAqhBUZV8GmH_t85Y-ohV1BEVKmIYUbt7zaU3tDHsSBfV1yhRC6RqaarLiz7m3fgUJOlO1imzcKcw
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"avg_score":{"value":60,"type":"Float"},"count":{"type":"Integer","value":5},"status":{"type":"String","value":"active"},"max_score":{"type":"Integer","value":100}}
   {"status":{"value":"inactive","type":"String"},"avg_score":{"type":"Float","value":50},"count":{"value":5,"type":"Integer"},"max_score":{"type":"Integer","value":90}}

📝 Example 4: Function Management

📋 Total functions: 7
🔍 Retrieved function: Get Active Users
✏️  Function updated
🗑️  Function deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

✅ All examples completed!
✓ http_functions.js completed successfully

=== Running ekoDB/ekodb-client/examples/javascript/batch_operations.js ===
=== Batch Operations (Direct HTTP) ===

✓ Authentication successful

=== Batch Insert ===
✓ Batch inserted 5 records
✓ Verified: Found 5 total records in collection

=== Creating test records for update/delete ===
Created 3 test records

=== Batch Update ===
✓ Batch updated 3 records
✓ Verified: Record updated with status="active"

=== Batch Delete ===
✓ Batch deleted 3 records
✓ Verified: Records successfully deleted (not found)

✓ All batch operations completed successfully
✓ batch_operations.js completed successfully

=== Running ekoDB/ekodb-client/examples/javascript/kv_operations.js ===
=== Key-Value Operations (Direct HTTP) ===

✓ Authentication successful

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: { value: { userId: 123, username: 'john_doe' }, type: 'Object' }

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: { type: 'Object', value: { price: 29.99, name: 'Product 1' } }
cache:product:2: {
  value: { name: 'Product 2', price: 39.989999999999995 },
  type: 'Object'
}
cache:product:3: {
  type: 'Object',
  value: { price: 49.989999999999995, name: 'Product 3' }
}

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ kv_operations.js completed successfully

=== Running ekoDB/ekodb-client/examples/javascript/collection_management.js ===
=== Collection Management (Direct HTTP) ===

✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: x07LJVXhVef1w23l_hBFU6bRxXWp7QqNirfLSG1j-NOqKphxVGHeKlIvt0Iyu2fv2Au4ukMqKC3DGgKbeFq4YQ

=== List Collections ===
Total collections: 22
Sample collections: [
  'chat_goals__ek0_testing',
  'chat_messages__ek0_testing',
  'schedules__ek0_testing',
  'schema_documents_client_go',
  'schema_users_client_ts'
]

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ collection_management.js completed successfully

=== Running ekoDB/ekodb-client/examples/javascript/transactions.js ===
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: vTzvW_tkn7YBBKfStua7IxbuD19ybsM1vUFXuVWqEJDAQyXlJuZOWwoALxS_cKMvMOAowPvqb5Dhat7WKL7IaQ
Created Bob: $500 - ID: cM85IH0Zq4a5j2mh9iK_suVqC0NIlO1g-0PgfWYl2-G8ou328z-FVoGjFjrtbHHcc4aYdjIxG1j52m2hQbaDOg

=== Example 1: Begin Transaction ===
Transaction ID: 2f06c7dc-5ba9-46bb-b5f5-801245704a23

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: {"type":"Integer","value":800}
Bob: {"value":700,"type":"Integer"}

=== Example 5: Rollback ===
New transaction: a3c9b5d3-140c-41dc-a662-1be51e20d870
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: {"type":"Integer","value":700}

=== Cleanup ===
✓ Deleted test accounts

✓ All transaction examples completed
✓ transactions.js completed successfully

=== Running ekoDB/ekodb-client/examples/javascript/crud_functions.js ===
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
   ✅ Deleted collection: crud_functions_users_js

============================================================
📝 Function 1: Insert + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: _ZTAkPcDp4jJ7CVhoHwtXYJZRCR29D7QOixunDzY_M-qA5tQ8tXwdBk5Ee1NlMMDYwX8psjKHRPHE1t6yBs4OA

2️⃣ Calling Function (Insert + Verify)...
   ✅ Function executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Name: Alice Smith
   📋 Email: alice@example.com
   📋 Status: pending
   📋 Credits: 0

============================================================
📝 Function 2: Query + Update + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: p7-L7FNDHbhCY9UPB0ubl_tkG0ljjcOccLafVZsflpd_YAwcPz5CaFnB3g12k6SE-9Q8QLh6euRXBumiAUugHw

2️⃣ Calling Function (Query + Update + Verify)...
   ✅ Function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Status updated to: {"type":"String","value":"active"}
   📋 Name: {"type":"String","value":"Alice Smith"}

============================================================
📝 Function 3: Query + Update Credits + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: G4ElS-VCwB7dKPGhdw0I_XFFwTVzkeVxQ57isOtAdW9NKxTc_ZUokqOgOjkLX5dE5Y3Gn030MzhPYSBVKCvIZA

2️⃣ Calling Function (Query + Update Credits + Verify)...
   ✅ Function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {"type":"Integer","value":100}
   📋 Status: {"type":"String","value":"active"}
   📋 Name: {"value":"Alice Smith","type":"String"}

============================================================
📝 Function 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: h1Y6GcRppQdT6sKIYEq-CFpoOhdFEFCRFiHhRAdMGJmQdoU0DEjGVxediXG767BG8vLnBYl-S3XsqqILCYx5BQ

2️⃣ Calling Function (Query + Delete + Verify)...
   ✅ Function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted function: _ZTAkPcDp4jJ7CVhoHwt...
   ✅ Deleted function: p7-L7FNDHbhCY9UPB0ub...
   ✅ Deleted function: G4ElS-VCwB7dKPGhdw0I...
   ✅ Deleted function: h1Y6GcRppQdT6sKIYEq-...
   ✅ Deleted collection: crud_functions_users_js

============================================================
✅ Complete CRUD Functions Example Finished!
============================================================

💡 Key Takeaways:
   ✅ Functions chain Functions together
   ✅ Each function demonstrates operation + verification
   ✅ Parameters make functions reusable
   ✅ Verification is built into the function itself
   ✅ Complete CRUD lifecycle in 4 focused functions
✓ crud_functions.js completed successfully

=== Running ekoDB/ekodb-client/examples/javascript/document_ttl.js ===
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
  Output: Document ID = w5XLz9KUKzppsf7h6I55Sp4TRBIIuaNbBTJH4FI9qHU_zNPGhQqD5J0UVjMvA3ltDUt9RVqHZasmt64l1sUjqg
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: findById(w5XLz9KUKzppsf7h6I55Sp4TRBIIuaNbBTJH4FI9qHU_zNPGhQqD5J0UVjMvA3ltDUt9RVqHZasmt64l1sUjqg)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: findById(w5XLz9KUKzppsf7h6I55Sp4TRBIIuaNbBTJH4FI9qHU_zNPGhQqD5J0UVjMvA3ltDUt9RVqHZasmt64l1sUjqg)
  Output: Error (expected) - Request failed with status 404: {"error":"Record not found (expired)"}
  ✓ PASS: Document expired (not found error)

═══════════════════════════════════════════════════════════
CLEANUP
═══════════════════════════════════════════════════════════
✓ Deleted test collection

╔════════════════════════════════════════════════════════╗
║              ALL TTL TESTS PASSED ✓                    ║
╚════════════════════════════════════════════════════════╝

TTL expiration is working correctly:
  • Documents with TTL expire after the specified time
  • Documents without TTL persist indefinitely
  • Different TTL durations are handled correctly
✓ document_ttl.js completed successfully

=== Running ekoDB/ekodb-client/examples/javascript/websocket_ttl.js ===
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
  Output: Document ID = UYpXad-QyOwOgh242Fd_UBE3FrNURCVIJvPGsOupWk7xfPkmXL5fu3HmORYYb8Jh4mStmzs5qjwmcnEUBf82lA
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: findById(UYpXad-QyOwOgh242Fd_UBE3FrNURCVIJvPGsOupWk7xfPkmXL5fu3HmORYYb8Jh4mStmzs5qjwmcnEUBf82lA)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: findById(UYpXad-QyOwOgh242Fd_UBE3FrNURCVIJvPGsOupWk7xfPkmXL5fu3HmORYYb8Jh4mStmzs5qjwmcnEUBf82lA)
  Output: Error (expected) - Request failed with status 404: {"error":"Record not found (expired)"}
  ✓ PASS: Document expired (not found error)

═══════════════════════════════════════════════════════════
CLEANUP
═══════════════════════════════════════════════════════════
✓ Deleted test collection

╔════════════════════════════════════════════════════════╗
║          WEBSOCKET TTL TEST PASSED ✓                   ║
╚════════════════════════════════════════════════════════╝

WebSocket TTL expiration is working correctly:
  • Documents with TTL inserted via client expire correctly
  • Queries correctly return null for expired documents
✓ websocket_ttl.js completed successfully

╔════════════════════════════════════════╗
║           Test Summary                 ║
╚════════════════════════════════════════╝
Total: 10
Passed: 10
Failed: 0
✅ JavaScript direct examples complete!
📦 Building TypeScript client library...

> @ekodb/ekodb-client@0.30.0 prepare
> npm run build


> @ekodb/ekodb-client@0.30.0 build
> tsc


up to date, audited 44 packages in 542ms

12 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.30.0 build
> tsc

✅ TypeScript client built!

added 1 package, removed 1 package, and audited 9 packages in 431ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities

added 1 package, removed 1 package, and audited 13 packages in 406ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> ekodb-typescript-examples@1.0.0 build
> tsc

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
✓ Created session: jtvZnpbTN4ybjXOVWSnRpPnbi3jvDGL_Ros721XYqeGezgV1c3CTnNridYkk_NPjnTodwc0hQmUuhUu3ftg52A

=== Sending Initial Message ===
✓ Message sent
  Response: There are currently no products available in the database based on the specified criteria. If you need further assistance or wish to check other filters, please let me know!

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
✓ Created second session: DrJ4ZIDt2xwZa45L_IpU6J29uYaaPEDgofrxfxcYcTzKKKQ7kCsg4KoEoqfzugdqy-NFAFTTJQ93eQqyx1i1mw
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 7

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 6

=== Cleanup ===
✓ Deleted second session
✓ Deleted session
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: 2G8zI5Qec947apAxSMahTPvNROQmLlmErC14jKw5yoMJlmvqFfsWEUEE_XVochXDxKO_mS_QS7Dy1GzcR02rCw

=== Sending Chat Message ===
Message ID: FY2wP7Ew9QO9G5D63FbhN__ZrSvdtHJVWiLCsXrp6Kfb61Kqvpoc7Byu1VolgCmhlfue5w9G98bZ5TYf_t_5Gw

=== AI Response ===
Here are the products available along with their prices:

1. **ekoDB Cloud**: $499
   - Description: Fully managed cloud database service product

2. **ekoDB**: $99
   - Description: A high-performance database product with AI capabilities

3. **ekoDB Pro**: $299
   - Description: Enterprise edition product with advanced features

If you would like to know more about any specific product, feel free to ask!

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_js',
  record: {
    price: 499,
    description: 'Fully managed cloud database service product',
    id: 'Ug9dxlk51XkuJqAyFvcVcqN9QXes-_u2VNjl5RGhmbCUiPB_VpEnhap9aCSTpLROT0lCpeTahOqwFAAQ7Vls4w',
    name: 'ekoDB Cloud'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_js',
  record: {
    id: 'aH7L1YGNo82RA4AXBgcwQmQVVlzwo2hycageaRCOndnl43DZ21DNVVYG1nSopYXQELY7OAh9oJjxztKSzCrnaQ',
    description: 'A high-performance database product with AI capabilities',
    name: 'ekoDB',
    price: 99
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_js',
  record: {
    description: 'Enterprise edition product with advanced features',
    name: 'ekoDB Pro',
    price: 299,
    id: 'c0Ta9InZGFo6NV-Df8y8D0Y64GvckkGU34WbXlDpOTeXnM9zfvqPESFkusP5KPZJg-h68-0b8_fF7dJPxyXdRw'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 2382ms

=== Token Usage ===
Prompt tokens: 3413
Completion tokens: 102
Total tokens: 3515

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ Chat completed successfully
✓ Client created

=== List Chat Models ===
Available chat models by provider:
  openai:
    - gpt-3.5-turbo-0125
    - gpt-5.4-2026-03-05
    - gpt-5-nano
    - gpt-5.4-mini
    - o1-pro
    - gpt-5.1-codex
    - tts-1
    - gpt-5-mini
    - gpt-5-mini-2025-08-07
    - gpt-4-turbo-2024-04-09
    - gpt-realtime-2.1-mini
    - gpt-realtime-2025-08-28
    - gpt-4
    - tts-1-hd
    - gpt-realtime-1.5
    - gpt-realtime
    - gpt-5.2-pro
    - gpt-5-chat-latest
    - gpt-5.5-pro
    - gpt-audio-2025-08-28
    - gpt-4o-mini-transcribe-2025-03-20
    - omni-moderation-2024-09-26
    - gpt-live-transcribe
    - gpt-5.6-luna
    - omni-moderation-latest
    - o4-mini-2025-04-16
    - gpt-4o-2024-11-20
    - gpt-5.1
    - whisper-1
    - gpt-5-pro
    - gpt-realtime-whisper
    - gpt-5-2025-08-07
    - gpt-5.4-mini-2026-03-17
    - babbage-002
    - gpt-4o-2024-05-13
    - text-embedding-3-large
    - chat-latest
    - gpt-realtime-translate
    - gpt-5.4-pro-2026-03-05
    - gpt-realtime-2
    - gpt-5-search-api
    - gpt-5.5-2026-04-23
    - tts-1-1106
    - gpt-5.5-pro-2026-04-23
    - gpt-audio-mini-2025-10-06
    - gpt-5.2-pro-2025-12-11
    - o3-mini-2025-01-31
    - gpt-audio-1.5
    - gpt-image-1.5
    - gpt-4-turbo
    - gpt-4o-2024-08-06
    - o3
    - gpt-4o-mini-search-preview-2025-03-11
    - gpt-4o-mini-transcribe-2025-12-15
    - gpt-4.1-mini
    - gpt-4o-mini
    - gpt-4.1-nano
    - gpt-image-2
    - gpt-3.5-turbo-16k
    - text-embedding-ada-002
    - gpt-realtime-2.1
    - gpt-4.1
    - gpt-5.5
    - gpt-4o-transcribe-diarize
    - gpt-5-codex
    - gpt-4-0613
    - gpt-5.1-2025-11-13
    - gpt-4o-search-preview
    - o1-2024-12-17
    - gpt-5-pro-2025-10-06
    - gpt-5.6-sol
    - gpt-4o-mini-tts-2025-12-15
    - gpt-image-1
    - gpt-audio
    - gpt-5.3-codex
    - gpt-5-search-api-2025-10-14
    - gpt-realtime-mini
    - gpt-image-1-mini
    - gpt-5.6-terra
    - gpt-3.5-turbo-instruct-0914
    - sora-2
    - gpt-5.4-pro
    - gpt-4o-mini-tts
    - gpt-realtime-mini-2025-12-15
    - gpt-5.1-codex-max
    - gpt-4o-mini-2024-07-18
    - gpt-4.1-2025-04-14
    - gpt-5
    - gpt-4o-mini-transcribe
    - gpt-image-2-2026-04-21
    - gpt-5.2-chat-latest
    - sora-2-pro
    - gpt-3.5-turbo-instruct
    - gpt-transcribe
    - gpt-4.1-nano-2025-04-14
    - gpt-5.1-chat-latest
    - tts-1-hd-1106
    - gpt-5.2-codex
    - gpt-4o
    - o4-mini-deep-research
    - gpt-5.4
    - gpt-4o-search-preview-2025-03-11
    - text-embedding-3-small
    - gpt-5-nano-2025-08-07
    - gpt-4o-transcribe
    - o4-mini
    - gpt-5.1-codex-mini
    - gpt-image-2.5-sunburst-2026-09-08
    - gpt-5.3-chat-latest
    - o3-2025-04-16
    - gpt-5.2-2025-12-11
    - chatgpt-image-latest
    - gpt-3.5-turbo
    - gpt-3.5-turbo-1106
    - gpt-4o-mini-tts-2025-03-20
    - gpt-5.2
    - gpt-5.4-nano-2026-03-17
    - o1
    - davinci-002
    - gpt-5.4-nano
    - o3-mini
    - o4-mini-deep-research-2025-06-26
    - gpt-audio-mini-2025-12-15
    - o1-pro-2025-03-19
    - gpt-6-luna
    - gpt-image-2.5-flare
    - gpt-6-astra
    - gpt-audio-mini
    - gpt-4.1-mini-2025-04-14
    - gpt-image-2.5-sunburst
    - gpt-image-2.5-flare-2026-09-08
    - gpt-4o-mini-search-preview
    - gpt-6-sol
    - gpt-live-1
    - gpt-6.1-sol
  anthropic:
    - claude-haiku-5-5
    - claude-sonnet-5-5
    - claude-opus-5-5
    - claude-fable-5-1
    - claude-opus-5
    - claude-sonnet-5
    - claude-fable-5
    - claude-opus-4-8
    - claude-opus-4-7
    - claude-sonnet-4-6
    - claude-opus-4-6
    - claude-opus-4-5-20251101
    - claude-haiku-4-5-20251001
    - claude-sonnet-4-5-20250929
  gemini:
    - gemini-2.5-flash
    - gemini-2.5-pro
    - gemini-2.5-flash-preview-tts
    - gemini-2.5-pro-preview-tts
    - gemma-4-26b-a4b-it
    - gemma-4-31b-it
    - gemini-flash-latest
    - gemini-flash-lite-latest
    - gemini-pro-latest
    - gemini-2.5-flash-lite
    - gemini-2.5-flash-image
    - gemini-3-flash-preview
    - gemini-3.1-pro-preview
    - gemini-3.1-pro-preview-customtools
    - gemini-3.1-flash-lite-preview
    - gemini-3.1-flash-lite
    - gemini-3-pro-image-preview
    - gemini-3-pro-image
    - nano-banana-pro-preview
    - gemini-3.1-flash-image-preview
    - gemini-3.1-flash-image
    - gemini-3.1-flash-lite-image
    - gemini-nano-banana-2.1
    - gemini-3.5-flash
    - gemini-3.5-flash-lite
    - gemini-omni-flash-preview
    - gemini-omni-1.1-flash
    - gemini-3.5-transcribe
    - gemini-3.6-flash
    - gemini-3.7-flash
    - gemini-3.8-flash
    - gemini-4-argon
    - lyria-3-clip-preview
    - lyria-3-pro-preview
    - lyria-3.5
    - gemini-3.1-flash-tts-preview
    - gemini-3.8-flash-tts
    - gemini-3.8-flash-lite-tts
    - gemini-robotics-er-2-preview
    - gemini-2.5-computer-use-preview-10-2025
    - antigravity-preview-05-2026
    - antigravity-preview-09-2026
    - antigravity-preview-latest
    - deep-research-max-preview-04-2026
    - deep-research-preview-04-2026
    - deep-research-pro-preview-12-2025
    - gemini-embedding-001
    - gemini-embedding-2-preview
    - gemini-embedding-2
    - aqa
    - veo-3.1-generate-preview
    - veo-3.1-fast-generate-preview
    - veo-3.1-lite-generate-preview
    - gemini-3.5-transcribe-live
    - gemini-2.5-flash-native-audio-latest
    - gemini-2.5-flash-native-audio-preview-09-2025
    - gemini-2.5-flash-native-audio-preview-12-2025
    - gemini-3.1-flash-live-preview
    - gemini-3.8-live
    - gemini-3.8-live-extended-thinking
    - gemini-robotics-er-2-streaming-preview
    - gemini-3.5-live-translate-preview
    - lyria-realtime-exp
Provider status:
  anthropic: ok 14 models
  gemini: ok 63 models
  openai: ok 135 models
  perplexity: not_configured (unverified) No Perplexity API Key

=== Get Specific Provider Models ===
OpenAI models: gpt-3.5-turbo-0125, gpt-5.4-2026-03-05, gpt-5-nano, gpt-5.4-mini, o1-pro, gpt-5.1-codex, tts-1, gpt-5-mini, gpt-5-mini-2025-08-07, gpt-4-turbo-2024-04-09, gpt-realtime-2.1-mini, gpt-realtime-2025-08-28, gpt-4, tts-1-hd, gpt-realtime-1.5, gpt-realtime, gpt-5.2-pro, gpt-5-chat-latest, gpt-5.5-pro, gpt-audio-2025-08-28, gpt-4o-mini-transcribe-2025-03-20, omni-moderation-2024-09-26, gpt-live-transcribe, gpt-5.6-luna, omni-moderation-latest, o4-mini-2025-04-16, gpt-4o-2024-11-20, gpt-5.1, whisper-1, gpt-5-pro, gpt-realtime-whisper, gpt-5-2025-08-07, gpt-5.4-mini-2026-03-17, babbage-002, gpt-4o-2024-05-13, text-embedding-3-large, chat-latest, gpt-realtime-translate, gpt-5.4-pro-2026-03-05, gpt-realtime-2, gpt-5-search-api, gpt-5.5-2026-04-23, tts-1-1106, gpt-5.5-pro-2026-04-23, gpt-audio-mini-2025-10-06, gpt-5.2-pro-2025-12-11, o3-mini-2025-01-31, gpt-audio-1.5, gpt-image-1.5, gpt-4-turbo, gpt-4o-2024-08-06, o3, gpt-4o-mini-search-preview-2025-03-11, gpt-4o-mini-transcribe-2025-12-15, gpt-4.1-mini, gpt-4o-mini, gpt-4.1-nano, gpt-image-2, gpt-3.5-turbo-16k, text-embedding-ada-002, gpt-realtime-2.1, gpt-4.1, gpt-5.5, gpt-4o-transcribe-diarize, gpt-5-codex, gpt-4-0613, gpt-5.1-2025-11-13, gpt-4o-search-preview, o1-2024-12-17, gpt-5-pro-2025-10-06, gpt-5.6-sol, gpt-4o-mini-tts-2025-12-15, gpt-image-1, gpt-audio, gpt-5.3-codex, gpt-5-search-api-2025-10-14, gpt-realtime-mini, gpt-image-1-mini, gpt-5.6-terra, gpt-3.5-turbo-instruct-0914, sora-2, gpt-5.4-pro, gpt-4o-mini-tts, gpt-realtime-mini-2025-12-15, gpt-5.1-codex-max, gpt-4o-mini-2024-07-18, gpt-4.1-2025-04-14, gpt-5, gpt-4o-mini-transcribe, gpt-image-2-2026-04-21, gpt-5.2-chat-latest, sora-2-pro, gpt-3.5-turbo-instruct, gpt-transcribe, gpt-4.1-nano-2025-04-14, gpt-5.1-chat-latest, tts-1-hd-1106, gpt-5.2-codex, gpt-4o, o4-mini-deep-research, gpt-5.4, gpt-4o-search-preview-2025-03-11, text-embedding-3-small, gpt-5-nano-2025-08-07, gpt-4o-transcribe, o4-mini, gpt-5.1-codex-mini, gpt-image-2.5-sunburst-2026-09-08, gpt-5.3-chat-latest, o3-2025-04-16, gpt-5.2-2025-12-11, chatgpt-image-latest, gpt-3.5-turbo, gpt-3.5-turbo-1106, gpt-4o-mini-tts-2025-03-20, gpt-5.2, gpt-5.4-nano-2026-03-17, o1, davinci-002, gpt-5.4-nano, o3-mini, o4-mini-deep-research-2025-06-26, gpt-audio-mini-2025-12-15, o1-pro-2025-03-19, gpt-6-luna, gpt-image-2.5-flare, gpt-6-astra, gpt-audio-mini, gpt-4.1-mini-2025-04-14, gpt-image-2.5-sunburst, gpt-image-2.5-flare-2026-09-08, gpt-4o-mini-search-preview, gpt-6-sol, gpt-live-1, gpt-6.1-sol

=== Get Anthropic Models ===
Anthropic models: claude-haiku-5-5, claude-sonnet-5-5, claude-opus-5-5, claude-fable-5-1, claude-opus-5, claude-sonnet-5, claude-fable-5, claude-opus-4-8, claude-opus-4-7, claude-sonnet-4-6, claude-opus-4-6, claude-opus-4-5-20251101, claude-haiku-4-5-20251001, claude-sonnet-4-5-20250929

=== Get Non-Existent Provider ===
Expected error for non-existent provider: Error: Request failed with status 404: {"error":"Unknown provider 'nonexistent_provider_xyz'","error_kind":"unknown_provider"}

✓ Chat Models example complete
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: rrB0uUthp4hO7dVeXzsOPcPY_U0K6pXmVnRPhC6mYOGpB8Bm366Ho3W760pdAOG1G-45MjIyP9MuC4igoooWnw

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** A high-performance database product
- **Price:** $99

If you need more information or have any specific questions about ekoDB, feel free to ask!

✓ Message 2 sent
  Response: The price of ekoDB is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: w81Pm8v8IdOsPM-gtTxpIiaEjZ_kwihC4k5PDYxasPjM_wlK8k7XgRjTI786aRgtDKzYyeRG10OshbEvjrjGBA
  Parent: rrB0uUthp4hO7dVeXzsOPcPY_U0K6pXmVnRPhC6mYOGpB8Bm366Ho3W760pdAOG1G-45MjIyP9MuC4igoooWnw

=== Listing Sessions ===
✓ Found 2 sessions
  Session 1: w81Pm8v8IdOsPM-gtTxpIiaEjZ_kwihC4k5PDYxasPjM_wlK8k7XgRjTI786aRgtDKzYyeRG10OshbEvjrjGBA (Untitled)
  Session 2: rrB0uUthp4hO7dVeXzsOPcPY_U0K6pXmVnRPhC6mYOGpB8Bm366Ho3W760pdAOG1G-45MjIyP9MuC4igoooWnw (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: w81Pm8v8IdOsPM-gtTxpIiaEjZ_kwihC4k5PDYxasPjM_wlK8k7XgRjTI786aRgtDKzYyeRG10OshbEvjrjGBA

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ All session management operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: aMooFFIlmZ2H5sMALY6GSkgwMPT_SmE5HLk7OpXZ8eGdP-jRflWt9hQSrSu1iUqLICzgBhaoiKkFNnVlvfy30w

=== List Collections ===
Total collections: 23
Sample collections: chat_goals__ek0_testing,test_accounts,chat_messages__ek0_testing,schedules__ek0_testing,schema_documents_client_go

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: sg6Pc1r_bcAbXFwPHOxRjF3ICSgcfJJGcnpaq5GHVrqgEmJCtdoOwbPgy8XaMLaWYmkZU8tbyldT-13ZnXtJow

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: 4z_edSERniBKfmsfPJzw0apOsp6t6ZZgFCSq9D4cM9zimB51iMPSnAnLkPBza5WxQdIl0MKvMbACfDcf_-rrNw

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

Creating edge cache function...
✓ Edge cache script created: qZFKzs_o-iVoqwGH95fQxZ0YEufAClANojKKDWe5eCeCA6FLv3oLTm7ycHa_fQzj184_uoD51g8SbuhRnQxqSA

Call 1: Cache miss (fetches from API)
Response time: 463ms
Result: {
  "records": [
    {
      "value": {
        "value": {
          "current_units": {
            "interval": "seconds",
            "temperature_2m": "°C",
            "time": "iso8601"
          },
          "latitude": 40.710335,
          "utc_offset_seconds": 0,
          "elevation": 32,
          "generationtime_ms": 0.02646446228027344,
          "timezone_abbreviation": "GMT",
          "longitude": -73.99308,
          "current": {
            "temperature_2m": 11.4,
            "time": "2026-10-17T08:30",
            "interval": 900
          },
          "timezone": "GMT"
        },
        "type": "Object"
      }
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

Call 2: Cache hit (served from ekoDB)
Response time: 12ms (38.6x faster!)
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "current_units": {
            "interval": "seconds",
            "temperature_2m": "°C",
            "time": "iso8601"
          },
          "latitude": 40.710335,
          "utc_offset_seconds": 0,
          "elevation": 32,
          "generationtime_ms": 0.02646446228027344,
          "timezone_abbreviation": "GMT",
          "longitude": -73.99308,
          "current": {
            "temperature_2m": 11.4,
            "time": "2026-10-17T08:30",
            "interval": 900
          },
          "timezone": "GMT"
        }
      }
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

=== The Magic ===
- Your DATABASE is your EDGE
- No Redis needed
- No CDN needed
- No cache invalidation logic needed (TTL handles it)
- With ripples: All nodes auto-sync cache
- One service: Database + Cache + Edge Functions

✓ Example complete!

=== ekoDB Function Composition Examples ===

📋 Setting up test data...

✅ Test data ready

📝 Example 1: Basic Function Composition

Building reusable functions that call each other...

✅ Saved reusable function: fetch_user
✅ Saved composed function: get_user_wrapper (calls fetch_user + projects fields)

📊 Result from composed function:
   Records: 1
   Name: {"value":"User 1","type":"String"}
   Department: {"type":"String","value":"engineering"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 59ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "id": 1,
      "company": {
        "catchPhrase": "Multi-layered client-server neural-net",
        "name": "Romaguera-Crona",
        "bs": ...

Second call (cache hit - from cache):
   ⏱️  Duration: 2ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "id": 1,
      "company": {
        "catchPhrase": "Multi-layered client-server neural-net",
        "name": "Romaguera-Crona",
        "bs": "harness real-time e-m...
   🚀 Cache speedup: 29.5x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

✅ Level 1 function: validate_user
✅ Level 2 function: fetch_slim_user (calls validate_user)
✅ Level 3 function: get_verified_user (calls fetch_slim_user)

📊 Result from 3-level nested composition:
   Records: 1
   Name: User 1
   Department: engineering

🎯 Key Benefit: Each function is independently testable and reusable!
   - validate_user: Used in 100 different workflows
   - fetch_slim_user: Used in 50 workflows
   - get_verified_user: Specific workflow


✅ All composition examples completed!
🚀 ekoDB Functions Example (JavaScript Client)

✅ Client initialized (token exchange automatic)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved: iLIhjtdM3L3cVo2HGbx8fWS4K9co887nScKM-t-3ZYTJZ2o5biTQxwk_uGM3TZcqqoR3FszPWv2BWqzy7gH9Bg
📊 Found 5 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Function

✅ Function saved
📊 Found 3 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Function

✅ Function saved
📊 Statistics: 2 groups
   {"avg_score":{"value":50,"type":"Float"},"status":{"type":"String","value":"inactive"},"count":{"value":5,"type":"Integer"}}
   {"avg_score":{"value":60,"type":"Float"},"status":{"type":"String","value":"active"},"count":{"type":"Integer","value":5}}
⏱️  Execution time: 0ms

📝 Example 4: Function Management

📋 Total functions: 7
🔍 Retrieved function: Get Active Users
✏️  Function updated
🗑️  Function deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

📝 Example 5: Multi-Stage Pipeline

✅ Multi-stage function saved
📊 Pipeline executed 2 stages
⏱️  Total execution time: 0ms
📈 Stage breakdown:

📝 Example 6: Count Users

✅ Count function saved
📊 Total user count: 10
⏱️  Execution time: 0ms

✅ All examples completed successfully!

💡 Key Advantages of Using the Client:
   • Automatic token management
   • Type-safe Stage builders
   • ChatMessage helpers
   • Cleaner, more maintainable code
   • Built-in error handling
🧹 Cleaning up...
✅ Deleted collection
✅ Deleted test functions

🚀 ekoDB Advanced Functions Example

📋 Setting up test data...
✅ Created 10 products

📝 Example 1: List All Products

✅ Function saved
📊 Found 10 products
⏱️  Execution time: 0ms

📝 Example 2: Group Products by Category

✅ Function saved
📊 Found 2 categories
   Electronics: 6 items (avg $325.67)
   Furniture: 4 items (avg $294.00)
⏱️  Execution time: 0ms

📝 Example 3: Count All Products

✅ Function saved
📊 Total products: 10
⏱️  Execution time: 0ms

📝 Example 4: Multi-Stage Aggregation

✅ Function saved
📊 Category analysis (2 categories):
   Electronics:
      Products: 6 | Stock: 232 | Avg Rating: ⭐4.52
   Furniture:
      Products: 4 | Stock: 43 | Avg Rating: ⭐4.26

⏱️  Total execution time: 0ms
📈 Pipeline stages:

📝 Example 5: Project Specific Fields

✅ Function saved
📊 Product summaries (10 items, showing first 3):
   1. Wireless Mouse - $29 (⭐4.5)
   2. Standing Desk - $599 (⭐4.7)
   3. Keyboard - $89 (⭐4.4)
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced function examples completed!
🚀 ekoDB AI Functions Example

⚠️  Note: These examples require AI API credentials (OpenAI, etc.)

📋 Setting up test data...
✅ Created 3 articles

📝 Example 1: Simple Chat Completion

✅ Chat function saved
🤖 AI Response:
   1. Precise: Vector data is more precise since it uses specific coordinates for representing locations.

2. Scalability: Vector data can be easily scaled without losing its quality which promotes clear visibility at all zoom levels.

3. Less Disk Space: Vector data typically requires less disk space as compared to raster data.

4. Accurate Geographic Data: They provide an accurate representation of geographic features such as roads, buildings, etc.

5. Advanced Analysis: They facilitate complex spatial and network analysis including shortest path, buffer generation, etc.

6. Rich Attributes: Vector data can hold a large number of attribute data such as name, category, status, etc. for each individual feature.

7. Data Limit: There is no limit on the level of detail that can be represented in vector data.

8. Flexible: It's versatile for creating different types of maps as vector data can be segregated by attribute to create specific views.
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embedding function saved
📊 Generated embeddings for 3 articles
   1. "Advanced Query Patterns" - 1536D vector
   2. "Getting Started with ekoDB" - 1536D vector
   3. "Draft Article" - 1536D vector
⏱️  Execution time: 0ms

📝 Example 3: List All Articles

✅ Function saved
📊 Found 3 articles
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All AI examples completed!
🚀 ekoDB JavaScript Complete Functions Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Function saved: uMqgbx91IlAP4X-oc-Vfg0jik4E_Z_DPg-3GV5jYhiAfTPMvv2_7Ut2iBHe5OCOGzrABKZSV_gC3ok_cfRTovw
📊 Found 2 product groups
   {"count":{"value":3,"type":"Integer"},"avg_price":{"value":575.6666666666666,"type":"Float"},"category":{"type":"String","value":"Electronics"}}
   {"category":{"value":"Furniture","type":"String"},"avg_price":{"value":474,"type":"Float"},"count":{"value":2,"type":"Integer"}}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   {"category":{"type":"String","value":"Electronics"},"count":{"type":"Integer","value":3}}
   {"category":{"type":"String","value":"Furniture"},"count":{"type":"Integer","value":2}}
⏱️  Execution time: 0ms

📝 Example 4: High Rating Products

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 5: Function with Parameter Definition

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 6: Multi-Stage Pipeline (FindAll → Group → Count)

✅ Function saved
📊 Pipeline executed 3 stages
⏱️  Total execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All complete function examples finished!

💡 This example demonstrates ekoDB's Function system:
   ✅ FindAll operations
   ✅ Group aggregations (Count, Average)
   ✅ Multi-stage pipelines (FindAll → Group → Count)
   ✅ Parameter definitions
   ✅ Function management (save, call, delete)
🚀 ekoDB CRUD Functions Example

📋 Setting up test data...
✅ Created 10 test users

📝 Example 1: List All Users

✅ Function saved
📊 Found 10 users
⏱️  Execution time: 0ms

📝 Example 2: Count Users by Status

✅ Function saved
📊 User counts by status:
   active: 7 users
   inactive: 3 users
⏱️  Execution time: 0ms

📝 Example 3: Average Score by Role

✅ Function saved
📊 Average scores by role:
   user: 70.0 (7 users)
   admin: 20.0 (3 users)
⏱️  Execution time: 0ms

📝 Example 4: Top Users by Score

✅ Function saved
📊 Users (showing first 5 of 10):
   1. User 4 - Score: 40
   2. User 9 - Score: 90
   3. User 7 - Score: 70
   4. User 10 - Score: 100
   5. User 5 - Score: 50
⏱️  Execution time: 0ms

📝 Example 5: User Summary Statistics

✅ Function saved
📊 User summary (2 groups):
   active: 7 users, Total Score: 370
   inactive: 3 users, Total Score: 180
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD function examples completed!
🚀 ekoDB JavaScript KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: BVh8IDdGVosticU4VBtZRQFnuu_d26WpHPCC-MnN5DFlYioMbeiI0K-gCVklVZc_uEP_ZOvvhe6RVAn-NTLwbQ
✅ Inserted 2 products with wrapped types

📝 Example 2: Function with Wrapped Type Parameters

✅ Function saved: l2du5C3E8XHWr-tqmJJ3okJY4SC7N0IHn6TegZ5z5k4smG5SqSY6TGzEjaQUml4GbQKlJCSQO1SINCulWxpmvA
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"value":{"userId":"user_abc","role":"admin"},"type":"Object"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: xngoGzePQwLt9-o1oi3-k7FHwSNvJljg5rORzvDqvS_-_JBVaUTPk25D8H4MxFPoTABwoZE4D7-8-L3mtPyLpw
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 3 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Function

✅ Function saved: 1ICBQGfagWtYmQQ2tqC4qi80RykdNrSqy34DK-6WUl8bjQpDHScWvfhfYrY64CB-c_SaAgScU2t86HSeOcEfrA
📊 Processed order with caching
⏱️  Stages executed: 3
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All KV & Wrapped Types examples completed!

💡 Key takeaways:
   ✅ Use Field.* helpers for type-safe wrapped values
   ✅ Field.decimal() preserves precision (no floating point errors)
   ✅ KV store is great for caching and quick lookups
   ✅ Stage.kv*() functions work within scripts
   ✅ Combine KV caching with collection inserts for real workflows
🚀 ekoDB Search Functions Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Function saved
📊 Found 5 documents
   1. Introduction to Machine Learning (AI)
   2. Getting Started with ekoDB (Database)
   3. Vector Databases Explained (Database)
   4. Database Design Principles (Database)
   5. Natural Language Processing (AI)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   AI: 2 documents
   Database: 3 documents
⏱️  Execution time: 0ms

📝 Example 3: Select Specific Fields

✅ Function saved
📊 Document titles (5 docs):
   1. Introduction to Machine Learning
   2. Getting Started with ekoDB
   3. Vector Databases Explained
   4. Database Design Principles
   5. Natural Language Processing
⏱️  Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Function saved
📊 Projected documents (showing first 3):
   1. Introduction to Machine Learning
   2. Getting Started with ekoDB
   3. Vector Databases Explained
⏱️  Execution time: 0ms

📝 Example 5: All Document Fields

✅ Function saved
📊 All documents (5 total, showing first 2):
   1. Introduction to Machine Learning (AI)
   2. Getting Started with ekoDB (Database)
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search function examples completed!
=== Join Operations Examples ===

Setting up sample data...
✅ Sample data created

1. Single collection join (users with departments):
Found 2 users with department data:
  - Bob Smith: Sales
  - Alice Johnson: Engineering

2. Join with filtering:
Found 1 users in Engineering:
  - Alice Johnson: Building A

3. Join with user profiles:
Found 2 users with profile data:
  - Bob Smith: Sales Manager
  - Alice Johnson: Senior Software Engineer

4. Join orders with user data:
Found 2 completed orders:
  - Laptop ($1200) by Alice Johnson
  - Mouse ($25) by Alice Johnson

5. Complex join with multiple conditions:
Found 2 users with example.com emails:
  - Alice Johnson (alice@example.com): Building A
  - Bob Smith (bob@example.com): Building B

=== Cleanup ===
✅ Deleted test collections

✅ Join operations examples completed!
✓ Client created

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: { value: { username: 'john_doe', userId: 123 }, type: 'Object' }

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: { type: 'Object', value: { price: 29.99, name: 'Product 1' } }
cache:product:2: {
  type: 'Object',
  value: { name: 'Product 2', price: 39.989999999999995 }
}
cache:product:3: {
  type: 'Object',
  value: { name: 'Product 3', price: 49.989999999999995 }
}

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
=== Schema Management Examples ===

1. Creating user schema with basic fields:
✅ User schema created

2. Creating product schema with text index:
✅ Product schema with indexes created

3. Creating document schema with vector index:
✅ Document schema with vector index created

4. Retrieving collection schema:
Schema fields: [ 'age', 'email', 'name', 'status' ]
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
  1. Score: 12.870, Matched: email, name
  2. Score: 6.270, Matched: name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 13.200, Matched: title, bio
  2. Score: 13.200, Matched: title, bio
  3. Score: 13.200, Matched: bio, title
  4. Score: 13.200, Matched: bio, title

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 26.400, Matched: bio, title
  2. Score: 26.400, Matched: title, bio
  3. Score: 26.400, Matched: title, bio
  4. Score: 26.400, Matched: title, bio

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 6.600, Matched: bio
  2. Score: 6.600, Matched: bio

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 6.600, Matched: bio

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.744, Matched:
  2. Score: 0.736, Matched:
  3. Score: 0.725, Matched:

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.494, Matched: title, content
  2. Score: 0.890, Matched: content, title
  3. Score: 0.298, Matched:

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 13.200, Matched: bio, skills

=== Cleanup ===
✅ Deleted test collections

✅ Search examples completed!
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'YVXKqWxig6q1Cm9IWFxh1C9k27qe2MAxLc56MpjahbbIUg4vGRogPBJgaH5d12X5Az8qhaD2mCjP-wUy9PoTow'
}

=== Find by ID ===
Found: {
  active: { value: true, type: 'Boolean' },
  id: 'YVXKqWxig6q1Cm9IWFxh1C9k27qe2MAxLc56MpjahbbIUg4vGRogPBJgaH5d12X5Az8qhaD2mCjP-wUy9PoTow',
  name: { type: 'String', value: 'Test Record' },
  value: { type: 'Integer', value: 42 }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  active: { value: true, type: 'Boolean' },
  id: 'YVXKqWxig6q1Cm9IWFxh1C9k27qe2MAxLc56MpjahbbIUg4vGRogPBJgaH5d12X5Az8qhaD2mCjP-wUy9PoTow',
  value: { type: 'Integer', value: 100 },
  name: { value: 'Updated Record', type: 'String' }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: oS7WOaieT1HfpcYapy1d9i7VNMQ0kzL2Clmx7EQ04A5F1pAMqPzQJ8SPECWHDJSGkuOV2QFDd4iHJBTzug5LFA

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
🚀 ekoDB JavaScript Client - Native SWR Function Examples

📋 Demonstrates:
   • Single-function SWR pattern (replaces 4-step pipeline)
   • Automatic cache checking, HTTP fetching, and cache setting
   • Built-in audit trail support
   • Duration string TTLs ('15m', '1h', '30s')
   • Multi-function pipeline integration
   • Dynamic TTL configuration


🧹 Cleaning up...
✓ Deleted 0 test functions and owned SWR resources

Example 1: Basic Native SWR
────────────────────────────────────────────────────────────────────────────────
Single function replaces KvGet → If → HttpRequest → KvSet pipeline
✓ Created native SWR script: github_user_native_js (KCKbifRVERglpmVHkIF24hnNW-ZIKHgwj5u2E3vI3MSoGmG0MkhALoHeIVYvDYj2jOzu24Hr9vwKr05L9H9fyQ)

First call (cache miss - will fetch from GitHub API):
  Response time: 126ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 3ms
  Speedup: 42.0x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit_js (OATU45jcHVumYNAfvgq1LBNYMfi1tivnI12LnBXn3qo4Y3M1QBKeYtGGbtagH3aqxD9kwcScdX_sL_rL7cEaig)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail_js' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline_js (KTJn4sSpTAhKIGdsmpFfe2a-g4I6hLTB4g91P3ni4LHDKfZHHEvYBpycHV3BGMtvfYDlazI4mAAhq95pK1cjkQ)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users_swr_js' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache_js (9UTi_liZsUhP4dF_9ztcqWaftTvzDFa5QW-tHLs4I88hAWg0ZSIE8VPQSYd8NF_9gDrjlcw90lWDz7z71FdmPw)
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
✓ Deleted 4 test functions and owned SWR resources

✅ All examples completed!
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_api_user_js_1706_1792226030269 (eTSmwGYcM9yrs_z2abxga3mObpqSgbbQuFYal7YldgfmsTTm9L2OCCk_QafX4lN5UHDWVFqUkXpqSc97C6iluw)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "value": {
          "company": {
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona"
          },
          "name": "Leanne Graham",
          "address": {
            "suite": "Apt. 556",
            "street": "Kulas Light",
            "city": "Gwenborough",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "zipcode": "92998-3874"
          },
          "website": "hildegard.org",
          "id": 1,
          "email": "Sincere@april.biz",
          "phone": "1-770-736-8031 x56442",
          "username": "Bret"
        },
        "type": "Object"
      }
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
Response time: 3ms (served from cache)
✓ Lightning fast cache hit

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation
🧹 Cleaning up...
✓ Cleanup complete

✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: e8uDn_1uAwDINS4nc8gF6doZc_Lbko2rDhqKmKmpJYrOoE4DJifjpFzv4AHfQzy0sfTfuSMnoIbvawChXvy8ZA
Created Bob: $500 - ID: J-KxIL_STMGj5cIKLPp3FLhK51CmcP6MGAV8Ff4xnZv2dvEUvo28ZPdfW07hydVxADFxg5Z48OQG4IUaC7N7WA

=== Example 1: Begin Transaction ===
Transaction ID (server-default isolation): d0d1ada4-549c-42ec-a48f-4ced958385be

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
New transaction: aaeeed08-df14-4f19-9f8b-e6b622439cda
Updated Bob: $700 → $600 (in transaction)
Status before rollback: Active
✓ Transaction rolled back

✓ Verified Bob remains $700 after rollback

=== Cleanup ===
✓ Deleted test account collection

✓ All client transaction examples completed
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: KTv4e7RFtOIrq4uI6B7aaERIJGKRWK8nm29m47nyKaIsCf9q7ezvA8Vzs3b7uJ31eCEmjJku8tcFKFUWer9syA

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
=== ekoDB Advanced CRUD Example (TypeScript) ===

--- Inserting base record ---
Inserted: YgwLK3JT0q0fCMb1yst3zOTe__nCm0w4cpbYGLgKB8z9LcMQlER4RWVn7YEPNw2hrM51Ix14-nbPIBQXVrekTQ

--- updateWithAction: increment ---
Score after increment: 150

--- updateWithAction: push ---
Tags after push: ["beginner","pro"]

--- updateWithAction: clear ---
temp_data after clear: null

--- updateWithActionSequence: multiple atomic actions ---
After sequence - score: 160, lives: 2, tags: ["beginner","pro","veteran"]

--- Cleanup ---
Deleted collection

=== All advanced CRUD operations completed ===
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
✓ Created session: ypDQNxKMgFe0DF-vKR6w0LbhOruirR_Mkw7A5z6Tqt8TSez9cMjKjD_-hrEAxcVnqMp0KA5cPjPq_3nc8I_8Eg

=== Sending Initial Message ===
✓ Message sent
  Response: It seems that there are currently no available products listed in the database. The only entry present regarding a product is about ekoDB, but it didn't return any records when queried. If there's anything else you'd like to explore or if you have specific products in mind, feel free to let me know!

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
✓ Created second session: J3LFByvpp_bNBZVUmtnkWs7f-wu-qaC79zNig60bnzrd_bIYZVDPUY2rZp0JduAeGXxdAZf5xWu4vDa202tXoQ
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 7

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 6

=== Cleanup ===
✓ Deleted chat session: J3LFByvpp_bNBZVUmtnkWs7f-wu-qaC79zNig60bnzrd_bIYZVDPUY2rZp0JduAeGXxdAZf5xWu4vDa202tXoQ
✓ Deleted chat session: ypDQNxKMgFe0DF-vKR6w0LbhOruirR_Mkw7A5z6Tqt8TSez9cMjKjD_-hrEAxcVnqMp0KA5cPjPq_3nc8I_8Eg
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: vYD2wDa11PCpOtNLxHPSR4FXNiv2HqVR3l0qyyb_2i7UPtyDfILw_rLNgWKNnu7okod3GRNca1WlGHRDvYcU1A

=== Sending Chat Message ===
Message ID: xeaDWxaE-CL0vLIO7JI-PmKHDfDONpMyL1vrjgjYglmx9UVFF_qwGa-us5duF9-ev35IdNl1u_zxNsvPaPU_iA

=== AI Response ===
The available products and their prices are as follows:

1. **ekoDB Pro**
   - Description: Enterprise edition product with advanced features
   - Price: $299

2. **ekoDB Cloud**
   - Description: Fully managed cloud database service product
   - Price: $499

3. **ekoDB**
   - Description: A high-performance database product with AI capabilities
   - Price: $99

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    id: 'EQMfCwMLFMyrjaJq2nFypOSl32T2QYETMXVKSw4a1xOq-5ZDBJcoM0RROf_SCv2t7Pvvpp1t3MP10cdpFYvk1w',
    price: 299,
    name: 'ekoDB Pro',
    description: 'Enterprise edition product with advanced features'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    price: 499,
    name: 'ekoDB Cloud',
    id: 'WQh1ApAtpmwHP4uv6PnON8WlQg4Wr3lUpzXaI3cEW2dlBHFLGnZst0rY0G96e93yLsfOFIBuBUY8JR8t7vmjWg',
    description: 'Fully managed cloud database service product'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    price: 99,
    name: 'ekoDB',
    description: 'A high-performance database product with AI capabilities',
    id: 'xwX7G1-HnsI-iYl67JdqvA4uLuoVFwGGVyGKjj0xURwE_fVUsUAmuB3QX0EbYICzGhpKvrQvn5SXUCr8PXZc_Q'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 1802ms

=== Token Usage ===
Prompt tokens: 3413
Completion tokens: 90
Total tokens: 3503

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Message Stream (SSE) Example (TypeScript) ===

Created session: I1080J8ap9GX988MpSyadW5z4yY2RusDx5iLn9JuBq1PYuiVA1AdFgwC4QexhmBSbzFzDIcsLlY3ll-Y-P8thg

Streaming response for: 'What is ekoDB?'

{"__progress":"received"}{"__progress":"generating","model":"default","provider":"openai"}**ekoDB** is a comprehensive, cloud-based knowledge platform focused on environmental, conservation, and biodiversity data. Designed to facilitate research, collaboration, and decision-making in ecology and related fields, ekoDB offers centralized access to a wide range of datasets—including species observations, habitat information, geospatial data, and environmental variables.

**Key features of ekoDB include:**
- **Data Aggregation:** Gathers and organizes biodiversity, environmental, and ecological data from various sources (e.g., scientific studies, public databases, citizen science initiatives).
- **Search and Visualization:** Provides user-friendly tools for searching datasets and visualizing complex ecological relationships.
- **Collaboration Tools:** Supports data sharing, project collaboration, and community engagement among researchers, conservation practitioners, and policymakers.
- **APIs and Integration:** Allows integration with other platforms and tools, facilitating advanced analysis and reporting.

**Applications:**
- Academic research in ecology, conservation, and environmental sciences
- Conservation policy and habitat management decisions
- Citizen science initiatives and data sharing

**Note:** If you’re referring to a specific project, platform, or database with a similar name but different focus, please provide more context, as there may be multiple uses of the term "ekoDB" in various regions or sectors.

--- Stream complete ---
Message ID: CpNNEYa23tZyGg-MG11qbFf4heReCFQTgd4ySPOEl3BxWRGMOhJeCSGXi63ejjvK2MVfh_VLmU9uIOPq-tgqGg
Execution time: 1814ms
Context window: 1000000 tokens

✓ Chat message stream example completed
✓ Client created

=== List Chat Models ===
Available chat models by provider:
  openai:
    - gpt-3.5-turbo-0125
    - gpt-5.4-2026-03-05
    - gpt-5-nano
    - gpt-5.4-mini
    - o1-pro
    - gpt-5.1-codex
    - tts-1
    - gpt-5-mini
    - gpt-5-mini-2025-08-07
    - gpt-4-turbo-2024-04-09
    - gpt-realtime-2.1-mini
    - gpt-realtime-2025-08-28
    - gpt-4
    - tts-1-hd
    - gpt-realtime-1.5
    - gpt-realtime
    - gpt-5.2-pro
    - gpt-5-chat-latest
    - gpt-5.5-pro
    - gpt-audio-2025-08-28
    - gpt-4o-mini-transcribe-2025-03-20
    - omni-moderation-2024-09-26
    - gpt-live-transcribe
    - gpt-5.6-luna
    - omni-moderation-latest
    - o4-mini-2025-04-16
    - gpt-4o-2024-11-20
    - gpt-5.1
    - whisper-1
    - gpt-5-pro
    - gpt-realtime-whisper
    - gpt-5-2025-08-07
    - gpt-5.4-mini-2026-03-17
    - babbage-002
    - gpt-4o-2024-05-13
    - text-embedding-3-large
    - chat-latest
    - gpt-realtime-translate
    - gpt-5.4-pro-2026-03-05
    - gpt-realtime-2
    - gpt-5-search-api
    - gpt-5.5-2026-04-23
    - tts-1-1106
    - gpt-5.5-pro-2026-04-23
    - gpt-audio-mini-2025-10-06
    - gpt-5.2-pro-2025-12-11
    - o3-mini-2025-01-31
    - gpt-audio-1.5
    - gpt-image-1.5
    - gpt-4-turbo
    - gpt-4o-2024-08-06
    - o3
    - gpt-4o-mini-search-preview-2025-03-11
    - gpt-4o-mini-transcribe-2025-12-15
    - gpt-4.1-mini
    - gpt-4o-mini
    - gpt-4.1-nano
    - gpt-image-2
    - gpt-3.5-turbo-16k
    - text-embedding-ada-002
    - gpt-realtime-2.1
    - gpt-4.1
    - gpt-5.5
    - gpt-4o-transcribe-diarize
    - gpt-5-codex
    - gpt-4-0613
    - gpt-5.1-2025-11-13
    - gpt-4o-search-preview
    - o1-2024-12-17
    - gpt-5-pro-2025-10-06
    - gpt-5.6-sol
    - gpt-4o-mini-tts-2025-12-15
    - gpt-image-1
    - gpt-audio
    - gpt-5.3-codex
    - gpt-5-search-api-2025-10-14
    - gpt-realtime-mini
    - gpt-image-1-mini
    - gpt-5.6-terra
    - gpt-3.5-turbo-instruct-0914
    - sora-2
    - gpt-5.4-pro
    - gpt-4o-mini-tts
    - gpt-realtime-mini-2025-12-15
    - gpt-5.1-codex-max
    - gpt-4o-mini-2024-07-18
    - gpt-4.1-2025-04-14
    - gpt-5
    - gpt-4o-mini-transcribe
    - gpt-image-2-2026-04-21
    - gpt-5.2-chat-latest
    - sora-2-pro
    - gpt-3.5-turbo-instruct
    - gpt-transcribe
    - gpt-4.1-nano-2025-04-14
    - gpt-5.1-chat-latest
    - tts-1-hd-1106
    - gpt-5.2-codex
    - gpt-4o
    - o4-mini-deep-research
    - gpt-5.4
    - gpt-4o-search-preview-2025-03-11
    - text-embedding-3-small
    - gpt-5-nano-2025-08-07
    - gpt-4o-transcribe
    - o4-mini
    - gpt-5.1-codex-mini
    - gpt-image-2.5-sunburst-2026-09-08
    - gpt-5.3-chat-latest
    - o3-2025-04-16
    - gpt-5.2-2025-12-11
    - chatgpt-image-latest
    - gpt-3.5-turbo
    - gpt-3.5-turbo-1106
    - gpt-4o-mini-tts-2025-03-20
    - gpt-5.2
    - gpt-5.4-nano-2026-03-17
    - o1
    - davinci-002
    - gpt-5.4-nano
    - o3-mini
    - o4-mini-deep-research-2025-06-26
    - gpt-audio-mini-2025-12-15
    - o1-pro-2025-03-19
    - gpt-6-luna
    - gpt-image-2.5-flare
    - gpt-6-astra
    - gpt-audio-mini
    - gpt-4.1-mini-2025-04-14
    - gpt-image-2.5-sunburst
    - gpt-image-2.5-flare-2026-09-08
    - gpt-4o-mini-search-preview
    - gpt-6-sol
    - gpt-live-1
    - gpt-6.1-sol
  anthropic:
    - claude-haiku-5-5
    - claude-sonnet-5-5
    - claude-opus-5-5
    - claude-fable-5-1
    - claude-opus-5
    - claude-sonnet-5
    - claude-fable-5
    - claude-opus-4-8
    - claude-opus-4-7
    - claude-sonnet-4-6
    - claude-opus-4-6
    - claude-opus-4-5-20251101
    - claude-haiku-4-5-20251001
    - claude-sonnet-4-5-20250929
  gemini:
    - gemini-2.5-flash
    - gemini-2.5-pro
    - gemini-2.5-flash-preview-tts
    - gemini-2.5-pro-preview-tts
    - gemma-4-26b-a4b-it
    - gemma-4-31b-it
    - gemini-flash-latest
    - gemini-flash-lite-latest
    - gemini-pro-latest
    - gemini-2.5-flash-lite
    - gemini-2.5-flash-image
    - gemini-3-flash-preview
    - gemini-3.1-pro-preview
    - gemini-3.1-pro-preview-customtools
    - gemini-3.1-flash-lite-preview
    - gemini-3.1-flash-lite
    - gemini-3-pro-image-preview
    - gemini-3-pro-image
    - nano-banana-pro-preview
    - gemini-3.1-flash-image-preview
    - gemini-3.1-flash-image
    - gemini-3.1-flash-lite-image
    - gemini-nano-banana-2.1
    - gemini-3.5-flash
    - gemini-3.5-flash-lite
    - gemini-omni-flash-preview
    - gemini-omni-1.1-flash
    - gemini-3.5-transcribe
    - gemini-3.6-flash
    - gemini-3.7-flash
    - gemini-3.8-flash
    - gemini-4-argon
    - lyria-3-clip-preview
    - lyria-3-pro-preview
    - lyria-3.5
    - gemini-3.1-flash-tts-preview
    - gemini-3.8-flash-tts
    - gemini-3.8-flash-lite-tts
    - gemini-robotics-er-2-preview
    - gemini-2.5-computer-use-preview-10-2025
    - antigravity-preview-05-2026
    - antigravity-preview-09-2026
    - antigravity-preview-latest
    - deep-research-max-preview-04-2026
    - deep-research-preview-04-2026
    - deep-research-pro-preview-12-2025
    - gemini-embedding-001
    - gemini-embedding-2-preview
    - gemini-embedding-2
    - aqa
    - veo-3.1-generate-preview
    - veo-3.1-fast-generate-preview
    - veo-3.1-lite-generate-preview
    - gemini-3.5-transcribe-live
    - gemini-2.5-flash-native-audio-latest
    - gemini-2.5-flash-native-audio-preview-09-2025
    - gemini-2.5-flash-native-audio-preview-12-2025
    - gemini-3.1-flash-live-preview
    - gemini-3.8-live
    - gemini-3.8-live-extended-thinking
    - gemini-robotics-er-2-streaming-preview
    - gemini-3.5-live-translate-preview
    - lyria-realtime-exp
Provider status:
  anthropic: ok 14 models
  gemini: ok 63 models
  openai: ok 135 models
  perplexity: not_configured (unverified) No Perplexity API Key

=== Get Specific Provider Models ===
OpenAI models: gpt-3.5-turbo-0125, gpt-5.4-2026-03-05, gpt-5-nano, gpt-5.4-mini, o1-pro, gpt-5.1-codex, tts-1, gpt-5-mini, gpt-5-mini-2025-08-07, gpt-4-turbo-2024-04-09, gpt-realtime-2.1-mini, gpt-realtime-2025-08-28, gpt-4, tts-1-hd, gpt-realtime-1.5, gpt-realtime, gpt-5.2-pro, gpt-5-chat-latest, gpt-5.5-pro, gpt-audio-2025-08-28, gpt-4o-mini-transcribe-2025-03-20, omni-moderation-2024-09-26, gpt-live-transcribe, gpt-5.6-luna, omni-moderation-latest, o4-mini-2025-04-16, gpt-4o-2024-11-20, gpt-5.1, whisper-1, gpt-5-pro, gpt-realtime-whisper, gpt-5-2025-08-07, gpt-5.4-mini-2026-03-17, babbage-002, gpt-4o-2024-05-13, text-embedding-3-large, chat-latest, gpt-realtime-translate, gpt-5.4-pro-2026-03-05, gpt-realtime-2, gpt-5-search-api, gpt-5.5-2026-04-23, tts-1-1106, gpt-5.5-pro-2026-04-23, gpt-audio-mini-2025-10-06, gpt-5.2-pro-2025-12-11, o3-mini-2025-01-31, gpt-audio-1.5, gpt-image-1.5, gpt-4-turbo, gpt-4o-2024-08-06, o3, gpt-4o-mini-search-preview-2025-03-11, gpt-4o-mini-transcribe-2025-12-15, gpt-4.1-mini, gpt-4o-mini, gpt-4.1-nano, gpt-image-2, gpt-3.5-turbo-16k, text-embedding-ada-002, gpt-realtime-2.1, gpt-4.1, gpt-5.5, gpt-4o-transcribe-diarize, gpt-5-codex, gpt-4-0613, gpt-5.1-2025-11-13, gpt-4o-search-preview, o1-2024-12-17, gpt-5-pro-2025-10-06, gpt-5.6-sol, gpt-4o-mini-tts-2025-12-15, gpt-image-1, gpt-audio, gpt-5.3-codex, gpt-5-search-api-2025-10-14, gpt-realtime-mini, gpt-image-1-mini, gpt-5.6-terra, gpt-3.5-turbo-instruct-0914, sora-2, gpt-5.4-pro, gpt-4o-mini-tts, gpt-realtime-mini-2025-12-15, gpt-5.1-codex-max, gpt-4o-mini-2024-07-18, gpt-4.1-2025-04-14, gpt-5, gpt-4o-mini-transcribe, gpt-image-2-2026-04-21, gpt-5.2-chat-latest, sora-2-pro, gpt-3.5-turbo-instruct, gpt-transcribe, gpt-4.1-nano-2025-04-14, gpt-5.1-chat-latest, tts-1-hd-1106, gpt-5.2-codex, gpt-4o, o4-mini-deep-research, gpt-5.4, gpt-4o-search-preview-2025-03-11, text-embedding-3-small, gpt-5-nano-2025-08-07, gpt-4o-transcribe, o4-mini, gpt-5.1-codex-mini, gpt-image-2.5-sunburst-2026-09-08, gpt-5.3-chat-latest, o3-2025-04-16, gpt-5.2-2025-12-11, chatgpt-image-latest, gpt-3.5-turbo, gpt-3.5-turbo-1106, gpt-4o-mini-tts-2025-03-20, gpt-5.2, gpt-5.4-nano-2026-03-17, o1, davinci-002, gpt-5.4-nano, o3-mini, o4-mini-deep-research-2025-06-26, gpt-audio-mini-2025-12-15, o1-pro-2025-03-19, gpt-6-luna, gpt-image-2.5-flare, gpt-6-astra, gpt-audio-mini, gpt-4.1-mini-2025-04-14, gpt-image-2.5-sunburst, gpt-image-2.5-flare-2026-09-08, gpt-4o-mini-search-preview, gpt-6-sol, gpt-live-1, gpt-6.1-sol

=== Get Anthropic Models ===
Anthropic models: claude-haiku-5-5, claude-sonnet-5-5, claude-opus-5-5, claude-fable-5-1, claude-opus-5, claude-sonnet-5, claude-fable-5, claude-opus-4-8, claude-opus-4-7, claude-sonnet-4-6, claude-opus-4-6, claude-opus-4-5-20251101, claude-haiku-4-5-20251001, claude-sonnet-4-5-20250929

=== Get Non-Existent Provider ===
Expected error for non-existent provider: Error: Request failed with status 404: {"error":"Unknown provider 'nonexistent_provider_xyz'","error_kind":"unknown_provider"}

✓ Chat Models example complete
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: OO1PnQRPIj4lYeywr1qYN2lmtNH4FHmPMwMUQkJKPDURVHIvd1zYHaSRAJQKoAfyVX2meuK3FoVjKk1SjQF39g

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: A high-performance database product
- **Price**: $99

If you need more information or have further questions, let me know!

✓ Message 2 sent
  Response: The price of ekoDB is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: LUe4T1KcGJH_sAfc9vnAmvuE8LG3KcuFQ8vNclTmEtr5mUOMRYCBQDlPwfhhgsoPH8EDZk7zl1-VbXr_7iFqng
  Parent: OO1PnQRPIj4lYeywr1qYN2lmtNH4FHmPMwMUQkJKPDURVHIvd1zYHaSRAJQKoAfyVX2meuK3FoVjKk1SjQF39g

=== Listing Sessions ===
✓ Found 2 sessions
  Session 1: LUe4T1KcGJH_sAfc9vnAmvuE8LG3KcuFQ8vNclTmEtr5mUOMRYCBQDlPwfhhgsoPH8EDZk7zl1-VbXr_7iFqng (Untitled)
  Session 2: OO1PnQRPIj4lYeywr1qYN2lmtNH4FHmPMwMUQkJKPDURVHIvd1zYHaSRAJQKoAfyVX2meuK3FoVjKk1SjQF39g (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: LUe4T1KcGJH_sAfc9vnAmvuE8LG3KcuFQ8vNclTmEtr5mUOMRYCBQDlPwfhhgsoPH8EDZk7zl1-VbXr_7iFqng

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ All session management operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: ju_iBYolqE8sSq6gBm0Nmv-kg9qZqo5Pi75m_dGAJlV-LJVeK6qWG0TgmrAprwe81HV01DIkZTs-d5iDylwQkQ

=== List Collections ===
Total collections: 27
Sample collections: chat_goals__ek0_testing,test_accounts,chat_messages__ek0_testing,schedules__ek0_testing,schema_documents_client_go

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Check Collection Exists (Before Creation) ===
Collection 'collection_utils_test_ts' exists: false

=== Creating Test Documents ===
Created 5 test documents

=== Check Collection Exists (After Creation) ===
Collection 'collection_utils_test_ts' exists: true

=== Count Documents ===
Document count in 'collection_utils_test_ts': 5

=== Check Non-Existent Collection ===
Collection 'nonexistent_collection_xyz' exists: false

=== Cleanup ===
Deleted collection 'collection_utils_test_ts'

✓ Collection Utilities example complete
✓ Client created
✓ conc_demo_pay saved
✓ conc_demo_rl_fail saved
✓ conc_demo_rl_skip saved
✓ conc_demo_lock saved

Invoke them like:
  POST /api/functions/conc_demo_pay_ts_3150_1792226049044 { "idempotency_key": "...", "amount": 100 }
  POST /api/functions/conc_demo_rl_fail_ts_3150_1792226049044 { "user_id": 42 }
  POST /api/functions/conc_demo_rl_skip_ts_3150_1792226049044 { "user_id": 42 }
  POST /api/functions/conc_demo_lock_ts_3150_1792226049044 { "resource": "queue:drain" }

✓ Cleaned up demo functions
=== ekoDB Convenience Methods Example ===

=== Native Object Creation ===
✓ Created record with plain object: {
  id: 'wmIMeLnlY_1bdMP2om7WlXBiEaLlmaeHo9Z72TIPDyoBU4bhlSYF3S1zsRCvsoXzrlgwhe7kKwXZSwYoLlaD1g'
}

=== Upsert Operation ===
✓ First upsert (update): {
  age: { type: 'Integer', value: 29 },
  email: { type: 'String', value: 'alice.j@newdomain.com' },
  name: { type: 'String', value: 'Alice Johnson' },
  id: 'wmIMeLnlY_1bdMP2om7WlXBiEaLlmaeHo9Z72TIPDyoBU4bhlSYF3S1zsRCvsoXzrlgwhe7kKwXZSwYoLlaD1g',
  active: { value: true, type: 'Boolean' }
}
✓ Second upsert (insert): { id: 'new-user-id' }

=== Find One Operation ===
✓ Found user by email: {
  id: 'wmIMeLnlY_1bdMP2om7WlXBiEaLlmaeHo9Z72TIPDyoBU4bhlSYF3S1zsRCvsoXzrlgwhe7kKwXZSwYoLlaD1g',
  active: { type: 'Boolean', value: true },
  name: { type: 'String', value: 'Alice Johnson' },
  age: { value: 29, type: 'Integer' },
  email: { type: 'String', value: 'alice.j@newdomain.com' }
}
✓ User not found (as expected)

=== Exists Check ===
✓ Record exists: true
✓ Fake record exists: false (should be false)

=== Pagination ===
✓ Inserted 25 records for pagination
✓ Page 1: 10 records (expected 10)
✓ Page 2: 10 records (expected 10)
✓ Page 3: 7 records (expected ~7)

=== Cleanup ===
✓ Deleted collection

✅ All convenience methods demonstrated successfully!
✓ Client created
✓ crypto_demo_hmac_ts saved
✓ crypto_demo_aes_ts saved
✓ crypto_demo_uuid_ts saved
✓ crypto_demo_totp_ts saved
✓ crypto_demo_encoding_ts saved

Invoke them with:
  POST /api/functions/crypto_demo_hmac_ts { "payload": "hi" }
  POST /api/functions/crypto_demo_aes_ts { "plaintext": "secret" }
  POST /api/functions/crypto_demo_uuid_ts
  POST /api/functions/crypto_demo_totp_ts
  POST /api/functions/crypto_demo_encoding_ts { "title": "Héllo World" }

✓ Cleaned up demo functions
=== Distinct Values Example ===

Inserting sample products...
Inserted 8 products

=== Distinct Categories (all products) ===
Found 3 distinct categories:
  - books
  - clothing
  - electronics

=== Distinct Statuses (all products) ===
Found 3 distinct statuses:
  - active
  - archived
  - discontinued

=== Distinct Statuses in Electronics ===
Found 2 distinct statuses for electronics:
  - active
  - discontinued

Cleanup done.
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: eWihjHvWri4uUmvCptDIurdKZZ0QS_zwTJ4nJDhIJU8df1QvzJsn2n_czNW6xpt-ZYEGMWmVEpk-D1zRPaa1HQ

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: blgQjefTM9FzAkH0XY5CtOVK-FerCCDenZlwPE0I5FS3UYRGaSNc1z9jhrgT3Dv-UF5wgGU1M1LfUjuAOyyLiQ

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

Creating edge cache function...
✓ Edge cache script created: PH9hXh7UJUUcf6hjSGymUqos9mMWe34ljChfIIj3dmqopywJkxzfUVceFJbA0dwbULhWv8r3fDzVc3T6stM6gA

Call 1: Cache miss (fetches from API)
Response time: 53ms
Result: {
  "records": [
    {
      "value": {
        "value": {
          "address": {
            "zipcode": "92998-3874",
            "city": "Gwenborough",
            "street": "Kulas Light",
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            },
            "suite": "Apt. 556"
          },
          "email": "Sincere@april.biz",
          "phone": "1-770-736-8031 x56442",
          "id": 1,
          "company": {
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets"
          },
          "website": "hildegard.org",
          "username": "Bret",
          "name": "Leanne Graham"
        },
        "type": "Object"
      }
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

Call 2: Cache hit (served from ekoDB)
Response time: 3ms (17.7x faster!)
Result: {
  "records": [
    {
      "value": {
        "value": {
          "address": {
            "zipcode": "92998-3874",
            "city": "Gwenborough",
            "street": "Kulas Light",
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            },
            "suite": "Apt. 556"
          },
          "email": "Sincere@april.biz",
          "phone": "1-770-736-8031 x56442",
          "id": 1,
          "company": {
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets"
          },
          "website": "hildegard.org",
          "username": "Bret",
          "name": "Leanne Graham"
        },
        "type": "Object"
      }
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

=== The Magic ===
- Your DATABASE is your EDGE
- No Redis needed
- No CDN needed
- No cache invalidation logic needed (TTL handles it)
- With ripples: All nodes auto-sync cache
- One service: Database + Cache + Edge Functions

✓ Example complete!

=== ekoDB Function Composition Examples ===

📋 Setting up test data...

✅ Test data ready

📝 Example 1: Basic Function Composition

Building reusable functions that call each other...

✅ Saved reusable function: fetch_user
✅ Saved composed function: get_user_wrapper (calls fetch_user + projects fields)

📊 Result from composed function:
   Records: 1
   Name: {"value":"User 1","type":"String"}
   Department: {"type":"String","value":"engineering"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 59ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "phone": "1-770-736-8031 x56442",
      "website": "hildegard.org",
      "email": "Sincere@april.biz",
      "address": {
        "street": "...

Second call (cache hit - from cache):
   ⏱️  Duration: 3ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "phone": "1-770-736-8031 x56442",
      "website": "hildegard.org",
      "email": "Sincere@april.biz",
      "address": {
        "street": "...
   🚀 Cache speedup: 19.7x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

✅ Level 1 function: validate_user
✅ Level 2 function: fetch_slim_user (calls validate_user)
✅ Level 3 function: get_verified_user (calls fetch_slim_user)

📊 Result from 3-level nested composition:
   Records: 1
   Name: User 1
   Department: engineering

🎯 Key Benefit: Each function is independently testable and reusable!
   - validate_user: Used in 100 different workflows
   - fetch_slim_user: Used in 50 workflows
   - get_verified_user: Specific workflow


✅ All composition examples completed!
client_function_contract: ok
🚀 ekoDB Functions Example (TypeScript)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved: 6ZCtiUfLcow72XRDQbLGKsaJ6xnM6noLxnh9Gmty2PU5bgSoLW_B2ONWdBSelWbAVYhbzofc2of9P89eC4XRiQ
📊 Found 5 active users

📝 Example 2: Parameterized Function

✅ Function saved: BqTuFrfWlVWXhPgfZnNXYpKllO_OBIi7nprBog8oi1dwCqnMyZzwoFhv-vz9hhovljxPp6ocsfA2zsizLlHCig
📊 Found 3 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: jgatsWHPAXKj-l0XogMoKTypXEtAWfXSfsGydsU8HSO6Cqt6qZPWdv1-8SeRmsAKyLcf54hXVRVv7KgCZJcsDA
📊 Statistics: 2 groups
   {"count":{"type":"Integer","value":5},"status":{"value":"inactive","type":"String"},"avg_score":{"value":50,"type":"Float"}}
   {"status":{"type":"String","value":"active"},"avg_score":{"value":60,"type":"Float"},"count":{"value":5,"type":"Integer"}}

📝 Example 4: UserFunction Management

📋 Total scripts: 8
🔍 Retrieved script: Get Active Users
✏️  function updated
🗑️  function deleted

ℹ️  Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.

✅ All examples completed!
🚀 ekoDB TypeScript Advanced Functions Example

📋 Setting up test data...
✅ Created 8 products

📝 Example 1: List All Products

✅ Function saved
📊 Found 8 products
⏱️  Execution time: 0ms

📝 Example 2: Group Products by Category

✅ Function saved
📊 Category breakdown:
   {"category":{"value":"Furniture","type":"String"},"avg_price":{"type":"Float","value":365.6666666666667},"count":{"type":"Integer","value":3}}
   {"category":{"value":"Electronics","type":"String"},"avg_price":{"value":367,"type":"Float"},"count":{"type":"Integer","value":5}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced script examples finished!
🚀 ekoDB TypeScript AI Functions Example

📋 Setting up test data...
✅ Created 2 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
🤖 AI Response:
   Vector databases offer several benefits:

1. **Efficient Similarity Search**: They enable rapid retrieval of similar items based on vector representations, ideal for applications like recommendations, search, and image recognition.

2. **High-dimensional Data Handling**: Vector databases can manage complex data types, such as text embeddings and image features, effectively.

3. **Scalability**: They can scale to handle large datasets, maintaining performance even with millions or billions of vectors.

4. **Machine Learning Integration**: Vector databases facilitate integrating with ML models, allowing for seamless operations on embeddings and prediction tasks.

5. **Real-time Processing**: Many vector databases support real-time querying and updates, making them suitable for dynamic applications.

6. **Flexible Indexing**: They often provide advanced indexing techniques (like approximate nearest neighbor search) for faster query responses.

7. **Enhanced Analytics**: Vector databases can support advanced analytics and insights generation, leveraging the semantic relationships within vector data.

8. **Multi-modal Capabilities**: They can handle various data types simultaneously, such as text, images, and audio, enabling comprehensive analysis across different modalities.
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embed script saved
📊 Generated 2 embeddings
   Dimensions: 1536
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All AI script examples finished!

💡 This example demonstrates:
   ✅ Chat completions with system/user messages
   ✅ Embedding generation for text
🚀 ekoDB TypeScript Complete Functions Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Function saved: A4gNZoddbD9Gh-HVE63HWTK6eFLAHERuTNMnASdL5xSiKRI7Bk977Qzq8zDYJ6CqbSMEBubZ_WmrOwJq487a6Q
📊 Found 2 product groups
   {"count":{"value":2,"type":"Integer"},"avg_price":{"value":474,"type":"Float"},"category":{"type":"String","value":"Furniture"}}
   {"avg_price":{"type":"Float","value":575.6666666666666},"count":{"value":3,"type":"Integer"},"category":{"type":"String","value":"Electronics"}}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   {"count":{"type":"Integer","value":3},"category":{"value":"Electronics","type":"String"}}
   {"count":{"value":2,"type":"Integer"},"category":{"type":"String","value":"Furniture"}}
⏱️  Execution time: 0ms

📝 Example 4: High Rating Products

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 5: UserFunction with Parameter Definition

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 6: Multi-Stage Pipeline (FindAll → Group → Count)

✅ Function saved
📊 Pipeline executed 3 stages
⏱️  Total execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All complete script examples finished!

💡 This example demonstrates ekoDB's function system:
   ✅ FindAll operations
   ✅ Group aggregations (Count, Average)
   ✅ Multi-stage pipelines (FindAll → Group → Count)
   ✅ Parameter definitions
   ✅ Function management (save, call, delete)
🚀 ekoDB TypeScript CRUD Functions Example

📋 Setting up test data...
✅ Created 10 test users

📝 Example 1: List All Users

✅ Function saved
📊 Found 10 users
⏱️  Execution time: 0ms

📝 Example 2: Count Users by Status

✅ Function saved
📊 User counts by status:
   active: 7 users
   inactive: 3 users
⏱️  Execution time: 0ms

📝 Example 3: Average Score by Role

✅ Function saved
📊 Average score by role:
   {"role":{"value":"user","type":"String"},"avg_score":{"type":"Float","value":70},"count":{"type":"Integer","value":7}}
   {"role":{"value":"admin","type":"String"},"avg_score":{"type":"Float","value":20},"count":{"value":3,"type":"Integer"}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD script examples finished!
🚀 ekoDB TypeScript KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: hI1QBVZz9hjbbNO8iHMPVifmqDCy03MmBBi9mEFczL3RP7Qo5IWJUxV7PRFk7zGZ3Ok9Gfu3BnfollMRucAIDA
✅ Inserted 2 products with wrapped types

📝 Example 2: UserFunction with Wrapped Type Parameters

✅ Function saved: s7PLI-srLW1zszgYvsDPEE1AA1-cMLF_Q4E2-0Damj-VCUfIm1l5lQQ5ZDWWzN4yzNydJl18abezgU8nqh3Ymg
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"value":{"role":"admin","userId":"user_abc"},"type":"Object"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: tO_CEGkYVwOK2xN1WQhnOzOWpDRVnxbqrgH22uP7cX-KNGxaKWs7Ynxy7xU3xWVXJQYbXqP4n4PIKIJHrGHTtg
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 3 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Function

✅ Function saved: zmtpQuexZWmunW7VdRD8DIXMwDF4UkpUSLSOkwYS2aUt6rnBCqTUiPiH8IQ2sALYB6fpQqDWvBovXqXxe5GOyA
📊 Processed order with caching
⏱️  Stages executed: 3
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All KV & Wrapped Types examples completed!

💡 Key takeaways:
   ✅ Use Field.* helpers for type-safe wrapped values
   ✅ Field.decimal() preserves precision (no floating point errors)
   ✅ KV store is great for caching and quick lookups
   ✅ Stage.kv*() functions work within scripts
   ✅ Combine KV caching with collection inserts for real workflows
🚀 ekoDB TypeScript Search Functions Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Function saved
📊 Found 5 documents
   1. Introduction to Machine Learning (AI)
   2. Natural Language Processing (AI)
   3. Vector Databases Explained (Database)
   4. Database Design Principles (Database)
   5. Getting Started with ekoDB (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   {"count":{"value":2,"type":"Integer"},"category":{"type":"String","value":"AI"}}
   {"category":{"type":"String","value":"Database"},"count":{"value":3,"type":"Integer"}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
=== ekoDB Goal Template CRUD Example (TypeScript) ===

--- Creating goal template ---
Created template: Data Migration (id: E_ooDu5Old1mMPDH5FiUcKsA0-ViGSrGVuSAdIlmcQgIFxR1sov7Akk6vOuEYU64RnIi6s6dVjKyvDvXcGDUNw)

--- Listing templates ---
Templates: {
  count: 1,
  items: [
    {
      description: [Object],
      id: 'E_ooDu5Old1mMPDH5FiUcKsA0-ViGSrGVuSAdIlmcQgIFxR1sov7Akk6vOuEYU64RnIi6s6dVjKyvDvXcGDUNw',
      steps: [Object],
      title: [Object]
    }
  ]
}

--- Getting template ---
Fetched: Data Migration

--- Updating template ---
Updated description: Updated: comprehensive data migration workflow

--- Deleting template ---
Template deleted successfully

✓ Goal template CRUD example completed
=== ekoDB Goals, Tasks & Agents Example (TypeScript) ===

--- Creating goal ---
Created goal: Deploy v2.0 (id: Z0Yp7GcLBnMFm0MxNKZQO1FybHCIVrYAYrJP8RZLbFYbn63z-G7OIoNVFxDzYM_D7zU4xvpF0cqEatRm5UN-Nw)

--- Listing goals ---
Goals: {
  "count": 1,
  "goals": [
    {
      "created_at": "2026-10-17T08:34:14.186395+00:00",
      "description": "Ship version 2.0 to production",
      "id": "Z0Yp7GcLBnMFm0MxNKZQO1FybHCIVrYAYrJP8RZLbFYbn63z-G7OIoNVFxDzYM_D7zU4xvpF0cqEatRm5UN-Nw",
      "status": "pending",
      "steps": "[{\"description\":\"Run test suite\"},{\"description\":\"Build release artifacts\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]",
      "title": "Deploy v2.0",
      "updated_at": "2026-10-17T08:34:14.186395+00:00"
    }
  ]
}

--- Getting goal ---
Fetched: Deploy v2.0

--- Updating goal ---
Updated description: Ship version 2.0 with hot-fix patches

--- Searching goals ---
Search results: {
  "count": 1,
  "items": [
    {
      "_score": 12.870000000000001,
      "created_at": {
        "type": "DateTime",
        "value": "2026-10-17T08:34:14.186395+00:00"
      },
      "description": {
        "type": "String",
        "value": "Ship version 2.0 with hot-fix patches"
      },
      "id": "Z0Yp7GcLBnMFm0MxNKZQO1FybHCIVrYAYrJP8RZLbFYbn63z-G7OIoNVFxDzYM_D7zU4xvpF0cqEatRm5UN-Nw",
      "status": {
        "type": "String",
        "value": "pending"
      },
      "steps": {
        "type": "String",
        "value": "[{\"description\":\"Run test suite\"},{\"description\":\"Build release artifacts\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]"
      },
      "title": {
        "type": "String",
        "value": "Deploy v2.0"
      },
      "updated_at": {
        "type": "DateTime",
        "value": "2026-10-17T08:34:14.200361+00:00"
      }
    }
  ]
}

--- Goal step: start step 0 ---
Step 0 started on goal Z0Yp7GcLBnMFm0MxNKZQO1FybHCIVrYAYrJP8RZLbFYbn63z-G7OIoNVFxDzYM_D7zU4xvpF0cqEatRm5UN-Nw
--- Goal step: complete step 0 ---
Step 0 completed on goal Z0Yp7GcLBnMFm0MxNKZQO1FybHCIVrYAYrJP8RZLbFYbn63z-G7OIoNVFxDzYM_D7zU4xvpF0cqEatRm5UN-Nw
--- Goal step: fail step 1 ---
Step 1 failed on goal Z0Yp7GcLBnMFm0MxNKZQO1FybHCIVrYAYrJP8RZLbFYbn63z-G7OIoNVFxDzYM_D7zU4xvpF0cqEatRm5UN-Nw

--- Completing goal ---
Goal status: pending_review
--- Approving goal ---
Goal status after approve: in_progress

--- Creating goal to reject ---
--- Rejecting goal ---
Goal status after reject: failed


--- Creating task ---
Created task: Hourly Health Check (id: -Zf1wiDq4rHjxP0Kf19X1cY6oxNroEO0YzTvtmqY7pEHmJeQpdyT2fcjHlWHAMpWVZDSQ8_10LlryBYQ9Ow_kg)

--- Listing tasks ---
Tasks: {
  "count": 1,
  "items": [
    {
      "action": {
        "type": "String",
        "value": "health_check"
      },
      "config": {
        "type": "Object",
        "value": {
          "endpoint": "/health"
        }
      },
      "cron": {
        "type": "String",
        "value": "0 * * * *"
      },
      "id": "-Zf1wiDq4rHjxP0Kf19X1cY6oxNroEO0YzTvtmqY7pEHmJeQpdyT2fcjHlWHAMpWVZDSQ8_10LlryBYQ9Ow_kg",
      "name": {
        "type": "String",
        "value": "Hourly Health Check"
      }
    }
  ]
}

--- Getting task ---
Fetched: Hourly Health Check

--- Starting task ---
Task status: running

--- Succeeding task ---
Task status after succeed: active

--- Pausing task ---
Task status after pause: paused

--- Resuming task ---
Task status after resume: active

--- Failing task ---
Task status after fail: active

--- Getting due tasks ---
Due tasks: {
  "count": 0,
  "items": []
}

--- Deleting task ---
Task deleted successfully


--- Creating agent ---
Created agent: SupportBot (id: NNIO-thhBbd4dzsYcahza3Kx5MoXVnipomxvWLZlB92En-DGxAuK7dE1WmAiFiTUR6-RmtPFpjtWyMTO-wg_uA)

--- Listing agents ---
Agents: {
  "count": 1,
  "items": [
    {
      "deployment_id": {
        "type": "String",
        "value": "deploy_prod_1"
      },
      "id": "NNIO-thhBbd4dzsYcahza3Kx5MoXVnipomxvWLZlB92En-DGxAuK7dE1WmAiFiTUR6-RmtPFpjtWyMTO-wg_uA",
      "llm_model": {
        "type": "String",
        "value": "gpt-4"
      },
      "name": {
        "type": "String",
        "value": "SupportBot"
      },
      "system_prompt": {
        "type": "String",
        "value": "You are a helpful customer support agent."
      }
    }
  ]
}

--- Getting agent by ID ---
Fetched: SupportBot

--- Getting agent by name ---
By name: SupportBot (id: NNIO-thhBbd4dzsYcahza3Kx5MoXVnipomxvWLZlB92En-DGxAuK7dE1WmAiFiTUR6-RmtPFpjtWyMTO-wg_uA)

--- Updating agent ---
Updated agent: SupportBot

--- Getting agents by deployment ---
Agents in deployment: {
  "count": 0,
  "items": []
}
WARNING: agents-by-deployment omitted created agent NNIO-thhBbd4dzsYcahza3Kx5MoXVnipomxvWLZlB92En-DGxAuK7dE1WmAiFiTUR6-RmtPFpjtWyMTO-wg_uA; TODO: check/fix the server-side deployment lookup

--- Deleting agent ---
Agent deleted successfully

--- Cleanup: deleting goals ---
Goals deleted successfully

=== All goals, tasks & agents operations completed ===
=== Join Operations Examples ===

Setting up sample data...
✅ Sample data created

1. Single collection join (users with departments):
Found 2 users with department data:
  - Alice Johnson: Engineering
  - Bob Smith: Sales

2. Join with filtering:
Found 1 users in Engineering:
  - Alice Johnson: Building A

3. Join with user profiles:
Found 2 users with profile data:
  - Alice Johnson: Senior Software Engineer
  - Bob Smith: Sales Manager

4. Join orders with user data:
Found 2 completed orders:
  - Laptop ($1200) by Alice Johnson
  - Mouse ($25) by Alice Johnson

5. Complex join with multiple conditions:
Found 2 users with example.com emails:
  - Alice Johnson (alice@example.com): Building A
  - Bob Smith (bob@example.com): Building B


✅ Join operations examples completed!
=== Cleanup ===
✅ Deleted test collections
✓ Client created
✓ ts_users_register saved
✓ ts_users_login saved
✓ ts_users_verify_token saved

=== Auth flow defined as pure stored functions ===
Call them like:
  POST /api/functions/jwt_register_ts_3990_1792226054705 { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/jwt_login_ts_3990_1792226054705 { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/jwt_verify_ts_3990_1792226054705 { "token": "<jwt>" }

Set JWT_SECRET in ekoDB's environment_vars whitelist before invoking.

✓ Cleaned up demo functions
=== ekoDB KV Document Linking Example (TypeScript) ===

--- Setup: creating KV key and documents ---
Set KV key: session:admin
Inserted document 1: 0089KlSNHg0qegVrc2pZzoNFCwDFva3EVNz82nzy_mhCLGP6ttQATWSDEN0-UQgZs8uTT7L8NCnXsF9Oe266yw
Inserted document 2: ZlpMHC06c9l4ZKBh8YndaWyqIFvU8Cu6Taktq0NFUoK2PXkaBeuihNbTE4f8_WGpgqf_1kaCB-NCQn98j8C0fg

--- Linking documents to KV key ---
Linked doc 0089KlSNHg0qegVrc2pZzoNFCwDFva3EVNz82nzy_mhCLGP6ttQATWSDEN0-UQgZs8uTT7L8NCnXsF9Oe266yw: null
Linked doc ZlpMHC06c9l4ZKBh8YndaWyqIFvU8Cu6Taktq0NFUoK2PXkaBeuihNbTE4f8_WGpgqf_1kaCB-NCQn98j8C0fg: null

--- Getting links for KV key ---
Links: [
  {
    "collection": "kv_links_example_ts_3992_1792226054787",
    "document_id": "ZlpMHC06c9l4ZKBh8YndaWyqIFvU8Cu6Taktq0NFUoK2PXkaBeuihNbTE4f8_WGpgqf_1kaCB-NCQn98j8C0fg",
    "field_path": null,
    "created_at": "2026-10-17T08:34:14.867922Z",
    "last_accessed": "2026-10-17T08:34:14.869300Z",
    "metadata": {}
  },
  {
    "collection": "kv_links_example_ts_3992_1792226054787",
    "document_id": "0089KlSNHg0qegVrc2pZzoNFCwDFva3EVNz82nzy_mhCLGP6ttQATWSDEN0-UQgZs8uTT7L8NCnXsF9Oe266yw",
    "field_path": null,
    "created_at": "2026-10-17T08:34:14.866526Z",
    "last_accessed": "2026-10-17T08:34:14.869300Z",
    "metadata": {}
  }
]

--- Unlinking document ---
Unlinked doc ZlpMHC06c9l4ZKBh8YndaWyqIFvU8Cu6Taktq0NFUoK2PXkaBeuihNbTE4f8_WGpgqf_1kaCB-NCQn98j8C0fg: null

--- Verifying remaining links ---
Remaining links: [
  {
    "collection": "kv_links_example_ts_3992_1792226054787",
    "document_id": "0089KlSNHg0qegVrc2pZzoNFCwDFva3EVNz82nzy_mhCLGP6ttQATWSDEN0-UQgZs8uTT7L8NCnXsF9Oe266yw",
    "field_path": null,
    "created_at": "2026-10-17T08:34:14.866526Z",
    "last_accessed": "2026-10-17T08:34:14.871293Z",
    "metadata": {}
  }
]

=== All KV linking operations completed ===

--- Cleanup ---
Cleanup complete
✓ Client created

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: { value: { username: 'john_doe', userId: 123 }, type: 'Object' }

=== KV Batch Set ===
✓ Batch set 3 keys
  kv_ops_ts_3994_1792226054957:cache:product:1: success
  kv_ops_ts_3994_1792226054957:cache:product:2: success
  kv_ops_ts_3994_1792226054957:cache:product:3: success

=== KV Batch Get ===
✓ Batch retrieved 3 values
  kv_ops_ts_3994_1792226054957:cache:product:1: { price: 29.99, name: 'Product 1' }
  kv_ops_ts_3994_1792226054957:cache:product:2: { price: 39.99, name: 'Product 2' }
  kv_ops_ts_3994_1792226054957:cache:product:3: { price: 49.99, name: 'Product 3' }

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 3 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 4

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: false

=== KV Batch Delete ===
✓ Batch deleted 3 keys
  kv_ops_ts_3994_1792226054957:cache:product:1: deleted
  kv_ops_ts_3994_1792226054957:cache:product:2: deleted
  kv_ops_ts_3994_1792226054957:cache:product:3: deleted

✓ All KV operations completed successfully
=== KV Precision: Float vs Decimal ===

=== Test 1: Using JavaScript Floats (LOSES PRECISION) ===
✓ Stored products with float prices

Retrieved float prices:
  Widget A: $29.99 (expected $29.99) ✓
  Widget B: $39.99 (expected $39.99) ✓
  Widget C: $49.99 (expected $49.99) ✓

=== Test 2: Using Field.decimal() (PRESERVES PRECISION) ===
✓ Stored products with decimal prices

Retrieved decimal prices:
  Widget A: $29.99 (expected $29.99) ✓
  Widget B: $39.99 (expected $39.99) ✓
  Widget C: $49.99 (expected $49.99) ✓

=== Test 3: Sum Calculation Comparison ===
  Float sum: $119.97 (expected $119.97)
  Decimal sum: $119.97 (expected $119.97)

=== Test 4: Extreme Precision Example ===
  Float 0.1 + 0.2 = 0.30000000000000004 (should be 0.3)
  Decimal "0.30" = 0.30 (exact!)

=== Cleanup ===

=== Summary ===
✅ Use Field.decimal() for monetary values, percentages, and
   any case where floating-point errors are unacceptable.
✅ Field.decimal() stores values as strings internally,
   preserving exact precision across all operations.

=== Cleanup ===
✓ Cleaned up test keys
✓ Client created
✓ ts_route_admin → GET /api/route/users/admin
✓ ts_route_user_by_id → GET /api/route/users/:id
✓ ts_route_user_posts → GET /api/route/users/:id/posts/:post_id
✓ ts_route_org_create_member → POST /api/route/orgs/:org/members

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
    "name": "Mercury",
    "diameter_km": 4879
  },
  {
    "name": "Jupiter",
    "diameter_km": 139820
  },
  {
    "name": "Neptune",
    "diameter_km": 49244
  }
]

--- Blocking HTTP (for comparison) ---
Blocking response: Hello! I hope you're having a wonderful day.

=== Done ===
=== ekoDB Schedule Management Example (TypeScript) ===

--- Creating schedule ---
Created schedule: Nightly Database Backup (id: e62be03b-1cc3-4c6a-8e9d-798d72a43e4c, cron: 0 0 2 * * *)

--- Listing schedules ---
Schedules: {
  "count": 1,
  "schedules": [
    {
      "created_at": "2026-10-17T08:34:17.434286Z",
      "cron_expression": "0 0 2 * * *",
      "description": null,
      "enabled": true,
      "function_label": "schedule_noop_typescript_4443_1792226057378",
      "id": "e62be03b-1cc3-4c6a-8e9d-798d72a43e4c",
      "last_execution": null,
      "name": "Nightly Database Backup",
      "next_execution": "2026-10-18T02:00:00Z",
      "parameters": {},
      "stats": {
        "avg_execution_time_ms": 0,
        "failed_executions": 0,
        "last_error": null,
        "successful_executions": 0,
        "total_executions": 0
      },
      "timezone": "UTC",
      "updated_at": "2026-10-17T08:34:17.434286Z"
    }
  ]
}

--- Getting schedule ---
Fetched: Nightly Database Backup (cron: 0 0 2 * * *)

--- Updating schedule ---
Updated: Nightly Full Backup (new cron: 0 0 3 * * *)

--- Triggering schedule ---
Trigger response: {
  "schedule_id": "e62be03b-1cc3-4c6a-8e9d-798d72a43e4c",
  "status": "triggered"
}

--- Pausing schedule ---
Schedule enabled after pause: false

--- Resuming schedule ---
Schedule enabled after resume: true

--- Deleting schedule ---
Schedule deleted successfully

=== All schedule operations completed ===
=== Schema Management Examples ===

1. Creating user schema with basic fields:
✅ User schema created

2. Creating product schema with text index:
✅ Product schema with indexes created

3. Creating document schema with vector index:
✅ Document schema with vector index created

4. Retrieving collection schema:
Schema fields: [ 'age', 'email', 'name', 'status' ]
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
  1. Score: 12.870, Matched: name, email
  2. Score: 6.270, Matched: name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 13.200, Matched: bio, title
  2. Score: 13.200, Matched: title, bio
  3. Score: 13.200, Matched: bio, title
  4. Score: 13.200, Matched: bio, title

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 26.400, Matched: title, bio
  2. Score: 26.400, Matched: title, bio
  3. Score: 26.400, Matched: bio, title
  4. Score: 26.400, Matched: title, bio

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 6.600, Matched: bio
  2. Score: 6.600, Matched: bio

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 6.600, Matched: bio

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.758, Matched:
  2. Score: 0.748, Matched:
  3. Score: 0.747, Matched:

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.499, Matched: content, title
  2. Score: 0.899, Matched: content, title
  3. Score: 0.303, Matched:

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 13.200, Matched: bio, skills

9. Vector search with a metadata pre-filter (category = ml):
Found 2 documents in category "ml" (NLP excluded)
  1. Introduction to Machine Learning (category: ml)
  2. Deep Learning Fundamentals (category: ml)

Paired vector search: document=Rust Programming Answer, query=Rust Programming Question

✅ Search examples completed!
=== Cleanup ===
✅ Deleted test collections
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'oWcCINsGqyc0QW36MfO6zD0mX0BUvfpAlx970uECMp5VgeEHbb5UUO0RYfa6NmaiLwz7HX2RhhKweJaJcws0NA'
}

=== Find by ID ===
Found: {
  name: { value: 'Test Record', type: 'String' },
  metadata: { type: 'Object', value: { key: 'value', nested: [Object] } },
  categories: { value: [ 'electronics', 'computers' ], type: 'Array' },
  data: { value: 'aGVsbG8gd29ybGQ=', type: 'String' },
  price: { value: 99.99, type: 'Float' },
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] },
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' },
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  value: { type: 'Integer', value: 42 },
  active: { type: 'Boolean', value: true },
  created_at: { type: 'DateTime', value: '2026-10-17T08:34:18.034+00:00' },
  id: 'oWcCINsGqyc0QW36MfO6zD0mX0BUvfpAlx970uECMp5VgeEHbb5UUO0RYfa6NmaiLwz7HX2RhhKweJaJcws0NA'
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-10-17T08:34:18.034Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { key: 'value', nested: { deep: true } }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  name: 'Test Record',
  metadata: { key: 'value', nested: { deep: true } },
  categories: [ 'electronics', 'computers' ],
  data: 'aGVsbG8gd29ybGQ=',
  price: 99.99,
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  tags: [ 'tag1', 'tag2', 'tag3' ],
  value: 42,
  active: true,
  created_at: '2026-10-17T08:34:18.034+00:00',
  id: 'oWcCINsGqyc0QW36MfO6zD0mX0BUvfpAlx970uECMp5VgeEHbb5UUO0RYfa6NmaiLwz7HX2RhhKweJaJcws0NA'
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  metadata: { type: 'Object', value: { key: 'value', nested: [Object] } },
  price: { type: 'Float', value: 99.99 },
  user_id: { value: '550e8400-e29b-41d4-a716-446655440000', type: 'String' },
  data: { value: 'aGVsbG8gd29ybGQ=', type: 'String' },
  active: { value: true, type: 'Boolean' },
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  id: 'oWcCINsGqyc0QW36MfO6zD0mX0BUvfpAlx970uECMp5VgeEHbb5UUO0RYfa6NmaiLwz7HX2RhhKweJaJcws0NA',
  name: { value: 'Updated Record', type: 'String' },
  value: { type: 'Integer', value: 100 },
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] },
  created_at: { type: 'DateTime', value: '2026-10-17T08:34:18.034+00:00' },
  categories: { value: [ 'electronics', 'computers' ], type: 'Array' }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: 6V1rekcgpiLYz7zF2VMs7RlacqdG1y_uRYZcPulbsqq-oxzOpPNkVlUoI6goNQF_ebbb6Gk6U_nlYOEyAP7FIg

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
🚀 ekoDB TypeScript Client - Native SWR Function Examples

📋 Demonstrates:
   • Single-function SWR pattern (replaces 4-step pipeline)
   • Automatic cache checking, HTTP fetching, and cache setting
   • Built-in audit trail support
   • Duration string TTLs ('15m', '1h', '30s')
   • Multi-function pipeline integration
   • Dynamic TTL configuration


🧹 Cleaning up...
✓ Deleted 0 test scripts and owned SWR resources

Example 1: Basic Native SWR
────────────────────────────────────────────────────────────────────────────────
Single function replaces KvGet → If → HttpRequest → KvSet pipeline
✓ Created native SWR script: github_user_native_ts (_eE6WP9XHARDd5RCxQsOq-en6ehNZ9yMxx6iXilzJo3LKeEQFwcCVtO9lyYY1DWh-6oHVSCs1ZjRci59Sfj6Cg)

First call (cache miss - will fetch from GitHub API):
  Response time: 54ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 2ms
  Speedup: 27.0x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit_ts (yNuv7nl6HKUnBBcCrLXuvRbH949LcsANQHqhLsYB1gQzP5mWl3Do4MGE4OA99-AVdn-GidV5dXmWPuQVrR0dCg)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail_ts' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline_ts (j3E4xGMtg03cWzr3EYL2tXdrBVDFRLes2euzsS8RZYW0hHn9nOJpjDvX8dKWbZPYru54QPQoBd5le0KV5wtJyA)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users_swr_ts' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache_ts (rIhJaYQcLz2nBw4jUYHjr2GodQnhbmDraVrSE5ak-6eSFLOLM34tlRD7EGnbf4LPdygzIR3POaFSDYBvwtT7Mg)
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
✓ Deleted 4 test scripts and owned SWR resources

✅ All examples completed!
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_api_user_ts_4706_1792226059130 (96PcNn5f-14QV0LnHhdIFTqN5KBxZ5s5DtH68sASFv84oYU5E4OBciuJ_RLPTeuOlbPa-Ip2usc_2D6p9GqSuw)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "id": 1,
          "address": {
            "street": "Kulas Light",
            "city": "Gwenborough",
            "suite": "Apt. 556",
            "zipcode": "92998-3874",
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            }
          },
          "name": "Leanne Graham",
          "email": "Sincere@april.biz",
          "username": "Bret",
          "company": {
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona"
          },
          "phone": "1-770-736-8031 x56442",
          "website": "hildegard.org"
        }
      }
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
Response time: 3ms (served from cache)
Result (cached): {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "id": 1,
          "address": {
            "street": "Kulas Light",
            "city": "Gwenborough",
            "suite": "Apt. 556",
            "zipcode": "92998-3874",
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            }
          },
          "name": "Leanne Graham",
          "email": "Sincere@april.biz",
          "username": "Bret",
          "company": {
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona"
          },
          "phone": "1-770-736-8031 x56442",
          "website": "hildegard.org"
        }
      }
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
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_reviews_ts_4706_1792226059130 (Xqwv-VEkr_1eNbTay76b9DjOYfzuCUR4IwK6EGQB5Z6WWedaC__Jyhv6dQ5A_TcSK-p1QeOAinl3BuYJhfrMvQ)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {
      "value": {
        "value": {
          "shippingInformation": "Ships in 3-5 business days",
          "id": 1,
          "title": "Essence Mascara Lash Princess",
          "meta": {
            "updatedAt": "2026-05-23T11:27:41.868Z",
            "barcode": "5784719087687",
            "createdAt": "2025-10-09T14:47:01.588Z",
            "qrCode": "https://cdn.dummyjson.com/public/qr-code.png"
          },
          "reviews": [
            {
              "date": "2025-04-30T09:41:02.053Z",
              "comment": "Would not recommend!",
              "reviewerEmail": "eleanor.collins@x.dummyjson.com",
              "reviewerName": "Eleanor Collins",
              "rating": 3
            },
            {
              "reviewerName": "Lucas Gordon",
              "rating": 4,
              "date": "2025-04-30T09:41:02.053Z",
              "comment": "Very satisfied!",
              "reviewerEmail": "lucas.gordon@x.dummyjson.com"
            },
            {
              "date": "2025-04-30T09:41:02.053Z",
              "comment": "Highly impressed!",
              "rating": 5,
              "reviewerName": "Eleanor Collins",
              "reviewerEmail": "eleanor.collins@x.dummyjson.com"
            }
          ],
          "sku": "BEA-ESS-ESS-001",
          "dimensions": {
            "depth": 22.99,
            "height": 13.08,
            "width": 15.14
          },
          "stock": 99,
          "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
          "rating": 2.56,
          "brand": "Essence",
          "minimumOrderQuantity": 48,
          "availabilityStatus": "In Stock",
          "price": 9.99,
          "discountPercentage": 10.48,
          "images": [
            "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
          ],
          "returnPolicy": "No return policy",
          "tags": [
            "beauty",
            "mascara"
          ],
          "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
          "warrantyInformation": "1 week warranty",
          "weight": 4,
          "category": "beauty"
        },
        "type": "Object"
      }
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
✓ Multi-API data fetched, merged, and cached atomically

=== Why This Is Powerful ===

✓ No separate cache layer (Redis, Memcached) needed
✓ No manual cache invalidation (TTL handles it)
✓ No separate edge infrastructure (ekoDB IS the edge)
✓ Atomic operations (function executes as transaction)
✓ With multi-node + ripples: Auto-sync across all nodes
✓ Sub-millisecond cache hits from internal storage
✓ One service instead of many (cache + API gateway + database)

=== Real-World Use Cases ===

1. API Gateway Pattern:
   - Client → ekoDB Function → Check cache → Call microservices → Merge → Cache

2. Database Federation:
   - Query multiple DBs (Postgres, MongoDB) + external APIs
   - Merge results in one function call

3. IoT Data Enrichment:
   - Sensor data + weather API + location API
   - Enrich and cache in one atomic operation

4. E-commerce Product Pages:
   - Product info + reviews + inventory + pricing
   - All from different sources, cached together

✓ Example complete - Your database IS your edge!

✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: M0N-8QWIg1Fy3ag6Y3pyI86aqwcjTYiMk9B227VmalA66rK0b4jbd1-HSi83UVJ4j4P0Z7qJhNhcijVI72BCJw
Created Bob: $500 - ID: EQbkJ5PUu5OAClPtP22a9Gf5ZltEF7N-o7JSLpOyQbmu7jF0C9k33QbNjaOUXUkBckr_dYhY_WdmygGyyeWOeA

=== Example 1: Begin Transaction ===
Transaction ID (server-default isolation): dc622af5-411b-4ad9-9b7a-1a1cd7212267

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
New transaction: bcfbc3b1-ff7d-4590-8fd4-efa36d49683e
Updated Bob: $700 → $600 (in transaction)
Status before rollback: Active
✓ Transaction rolled back

✓ Verified Bob remains $700 after rollback

=== Cleanup ===
✓ Deleted test account collection

✓ All client transaction examples completed
✓ Client created

=== Create User Function ===
Created user function with ID: 7k_V6JEACKUxt1_gRrXGFi1B5f5qIdDM1wPP2vsrJDOOARC9jK1RHDni3NdHeGUEUYl_3rWE-7k6FTgLftO9vA

=== Get User Function ===
Retrieved: get_active_users_ts - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 8 user functions:
  - get_active_users_ts: Get Active Users
  - get_active_users_client_js: Get Active Users (Updated)
  - get_active_users_client_ts_updated: Get Active Users (Updated)
  - get_active_users_client_ts_updated: Get Active Users (Updated)
  - fetch_product_reviews_ts_4706_1792226059130: Fetch Product with Reviews (Multi-API)
  - fetch_product_reviews_ts_97810_1792225951079: Fetch Product with Reviews (Multi-API)
  - conc_demo_rl_fail_ts_95985_1792225931695: Rate-limit (fail mode)
  - conc_demo_rl_skip_ts_95985_1792225931695: Rate-limit (skip mode)

=== Update User Function ===
User function updated successfully

=== Delete User Function ===
User function deleted successfully

✓ User Functions API example complete
=== WebSocket Chat Streaming Example (TypeScript) ===

Created chat session: X0-ZBVpVDEsV2Zs-mn1kt-IX8DFNhNlbu6sVU2byJYGaQcpWTq-jDEthsIJ6UO125RHBw2ZPd-GWQeGNAG_aKg

Sending message: 'What is the capital of France?'
{"__progress":"received"}{"__progress":"generating","model":"default","provider":"openai"}The capital of France is Paris.

--- Stream ended ---
Message ID: ttWd2cZWDj2SRdxYaNnc1C51UFySrFIQKSoj5GXsH_CS8wxIuDzDClvUDkx2vYDKrHqKtwvzTg9EqsVIuj6Z5w
Execution time: 619ms
Token usage: {"completion_tokens":8,"prompt_tokens":15,"total_tokens":23}

Full response: {"__progress":"received"}{"__progress":"generating","model":"default","provider":"openai"}The capital of France is Paris....
=== WebSocket Subscription Example ===

✓ Authentication successful
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_ts' ===
✓ Subscribed (subscription_id: sub_39cf5aae5d7f4c2fb53fcf1ab6068d2a)

=== Performing mutations to trigger notifications ===
Inserting a record...
✓ Inserted record: Q1YZXtU4k8XPlX1QuYjTsPgi35IP70UkYXsTtF_9HUSoG6ktJP6EMb0jNjhl9BWV5C6OYf5m1gz5gNtKp4wYUw
  📡 Notification received for Q1YZXtU4k8XPlX1QuYjTsPgi35IP70UkYXsTtF_9HUSoG6ktJP6EMb0jNjhl9BWV5C6OYf5m1gz5gNtKp4wYUw

Inserting another record...
✓ Inserted record: HW0_bb_SivpoCpNvRC_lZ9QiQ3a5FXcWNFJpPC-KOHHrKBbiFfENUD0mV5O2EajWfpyqrTBr5Vajn3XVGBLIjw
  📡 Notification received for HW0_bb_SivpoCpNvRC_lZ9QiQ3a5FXcWNFJpPC-KOHHrKBbiFfENUD0mV5O2EajWfpyqrTBr5Vajn3XVGBLIjw

=== Unsubscribing ===
✓ Unsubscribed: {"collection":"ws_subscribe_example_ts","found":true,"unsubscribed":true}

✓ WebSocket subscription example completed successfully
✓ Deleted collection 'ws_subscribe_example_ts'
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: 8IRn0SpXNUpXXk480UnlzV-0tb5rTDvhNF_Ibz-oKBshW2EdLa3uTP4PMqSVHEuGAHiRdauvquxtXE8PquxBjg

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 5 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
=== Bypass Ripple Example ===

1. Basic insert (ripple enabled):
   Inserted with ripple: {"id":"vbjI9AaqG4L18dp5uDqqtg-03b90_JnSJqOr-oePq4C0vesbYLR8H3euDyE6IcXh1imjYJktWeq9U6stAy9Ozg"}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {"id":"gMuPt2mEqfdyXH4jMP2HFr8a4dsuZnmaNfO0ls1Bk001fJFj18oLpgndGR8GF0hi05gyzlNXzFhMwMm6Ge2LIA"}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {"price":{"value":150,"type":"Integer"},"id":"vbjI9AaqG4L18dp5uDqqtg-03b90_JnSJqOr-oePq4C0vesbYLR8H3euDyE6IcXh1imjYJktWeq9U6stAy9Ozg","name":{"type":"String","value":"Product 1"}}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {"id":"custom-id"}

✅ All bypass_ripple operations completed successfully!
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields (id, name, email only)
  Found 3 active users
  Fields returned: ["id","email","name"]
  First user: Dave Brown <dave@example.com>

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ["created_at","id","email","user_role","bio","status","age","name","avatar_url"]

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
    - Fields: ["name","api_key","age","created_at","bio","status","email","avatar_url","user_role","id","secret_token","password"]
  Projected query:
    - 3 fields per record
    - Fields: ["email","name","id"]
  Bandwidth savings: ~75% fewer fields

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
