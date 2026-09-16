make test-examples-javascript
🧪 Running JavaScript examples (direct HTTP/WebSocket)...

added 1 package, removed 1 package, and audited 9 packages in 3s

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
  id: 'F8fDcJ4m7sZHXQmlukN3Y6trYGCjr8kj5-zd1jsO5davj6UmHT1cyay0sTDApPXrIBg-XSK_3tCySK549PS3vw'
}

=== Find by ID ===
Found: {
  name: { type: 'String', value: 'Test Record' },
  active: { type: 'Boolean', value: true },
  id: 'F8fDcJ4m7sZHXQmlukN3Y6trYGCjr8kj5-zd1jsO5davj6UmHT1cyay0sTDApPXrIBg-XSK_3tCySK549PS3vw',
  value: { value: 42, type: 'Integer' }
}

=== Find with Query ===
Found documents: [
  {
    active: { value: true, type: 'Boolean' },
    name: { type: 'String', value: 'Test Record' },
    id: 'F8fDcJ4m7sZHXQmlukN3Y6trYGCjr8kj5-zd1jsO5davj6UmHT1cyay0sTDApPXrIBg-XSK_3tCySK549PS3vw',
    value: { value: 42, type: 'Integer' }
  }
]

=== Update Document ===
Updated: {
  value: { type: 'Integer', value: 100 },
  id: 'F8fDcJ4m7sZHXQmlukN3Y6trYGCjr8kj5-zd1jsO5davj6UmHT1cyay0sTDApPXrIBg-XSK_3tCySK549PS3vw',
  name: { type: 'String', value: 'Updated Record' },
  active: { value: true, type: 'Boolean' }
}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
✓ simple_crud.js completed successfully

=== Running ekoDB/ekodb-client/examples/javascript/simple_websocket.js ===
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: tuqqPo65s9NE1l4L0kSO8amhdoidyx2Fk_IEq-_EyOMswmorSeE9YZZrvwb4IIhcmvprhg5ypVrxZSnCdUYkdg

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "type": "Success",
  "payload": {
    "data": [
      {
        "id": "tuqqPo65s9NE1l4L0kSO8amhdoidyx2Fk_IEq-_EyOMswmorSeE9YZZrvwb4IIhcmvprhg5ypVrxZSnCdUYkdg",
        "active": {
          "value": true,
          "type": "Boolean"
        },
        "value": {
          "value": 42,
          "type": "Integer"
        },
        "name": {
          "value": "WebSocket Test Record",
          "type": "String"
        }
      }
    ]
  },
  "messageId": "1789495144348"
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

✅ Function saved: 8WvIDGgAtJWxUpiDWtSh-pVCGanOohyLVCHP94_jGQwVBklGMVlO0pYGt0U6QzqZ5_RiKX4AVbVoJbV-IlHArA
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Function saved: eU4Vxjhng9slQoKuArxowH6bM3t1299gyCsfF6aRd3midRztkjqWUrNcae7zKx637cX7whDJtmpXlUL_h9Q17A
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: OGllAFnMS3I4LOCc8XA579bDAYajW3vG3GJPu65CkpZ7UNAsZ0L8D-4bRcXfEIurso_p6EX_NyGobRULdZ7Dsw
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"status":{"value":"inactive","type":"String"},"avg_score":{"type":"Float","value":50},"max_score":{"type":"Integer","value":90},"count":{"value":5,"type":"Integer"}}
   {"count":{"type":"Integer","value":5},"avg_score":{"type":"Float","value":60},"status":{"type":"String","value":"active"},"max_score":{"type":"Integer","value":100}}

📝 Example 4: Function Management

📋 Total functions: 14
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
Retrieved value: { type: 'Object', value: { userId: 123, username: 'john_doe' } }

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: { value: { name: 'Product 1', price: 29.99 }, type: 'Object' }
cache:product:2: {
  type: 'Object',
  value: { name: 'Product 2', price: 39.989999999999995 }
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
Collection created with first record: Rnfa4g453_cS0g9ZqFa67BEVsYG-XKlFPj7SnCEbrZacCNmH7WvUZwoJbgdSa_PQcDvV-iUUg0AftHqwa_QXFA

=== List Collections ===
Total collections: 33
Sample collections: [
  'schema_employees_client_ts',
  'schema_products_client_go',
  'agent_function_versions__ek0_testing',
  'chat_agent_configs__ek0_testing',
  'chat_goal_templates__ek0_testing'
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
Created Alice: $1000 - ID: 954rkVxajrQ5KMhClBx2c-hQgH-misT_OnJMU1CLivB6zmFjz1grHlIchxTgd-JK0VIw35Yjlw1CV_TyYTZziQ
Created Bob: $500 - ID: PQd6SZUNfeHHyoHwnu_2f1hy9NiCYmXC2RCdwgHQYGAOMX4jBn8ykOfnlBL6rHtD4lca3jUTQ6tR5STud5lbHw

=== Example 1: Begin Transaction ===
Transaction ID: 2ce6b13e-de85-46bc-9e33-6651f29b35bd

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: {"value":800,"type":"Integer"}
Bob: {"value":700,"type":"Integer"}

=== Example 5: Rollback ===
New transaction: 846e0741-efd5-4614-8d1b-5b49f67fdba1
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
   ✅ Function saved: vRhdtTd7Oo38hyjV6gZvLj70OU4wi5brcnaifzm2hF17--vspedeC2SfjfeF4FVcB2GxTAKM2j3PEfPw_-tJPQ

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
   ✅ Function saved: 9c-4ayaieVe_9pFO3hek4gTBuMEBoFy0cpQVoJnfafEyTYG_2BSto9OK9WyRCJB6ypvFImajElh0uD12qZMKEw

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
   ✅ Function saved: 0Thm3ncO1j_IGYGtopU1f52gKJJpynW8bXNceXQam2rMnnl3B81wZDaVTLlPjYnvzL4tlGmI13eNk4xRzMazzw

2️⃣ Calling Function (Query + Update Credits + Verify)...
   ✅ Function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {"value":100,"type":"Integer"}
   📋 Status: {"value":"active","type":"String"}
   📋 Name: {"type":"String","value":"Alice Smith"}

============================================================
📝 Function 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: gRiNk7jdsd11SFuqWL0TRz1segjWtAiSdldg3ajhX8deKZIKojRtxjfxQt1tJzj1evWT4EjW7vqeDO-BSu0YDg

2️⃣ Calling Function (Query + Delete + Verify)...
   ✅ Function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted function: vRhdtTd7Oo38hyjV6gZv...
   ✅ Deleted function: 9c-4ayaieVe_9pFO3hek...
   ✅ Deleted function: 0Thm3ncO1j_IGYGtopU1...
   ✅ Deleted function: gRiNk7jdsd11SFuqWL0T...
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
  Output: Document ID = sygB9G5o9JyKciMLRHKcSbo_UYajm4R6XkeitKOs08EXOQGGbV-U1ew1DBqpRMpD2NS4yggRnK2QvcMGT60O0g
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: findById(sygB9G5o9JyKciMLRHKcSbo_UYajm4R6XkeitKOs08EXOQGGbV-U1ew1DBqpRMpD2NS4yggRnK2QvcMGT60O0g)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: findById(sygB9G5o9JyKciMLRHKcSbo_UYajm4R6XkeitKOs08EXOQGGbV-U1ew1DBqpRMpD2NS4yggRnK2QvcMGT60O0g)
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
  Output: Document ID = UnGbqnoSPZ-1akQnWs16JFPPxKVV1IbI1ZiSX8B2Bs76YwYmYyVymKscS8IQdp1ZqR6La6y8EQTN7ASgAT0KwA
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: findById(UnGbqnoSPZ-1akQnWs16JFPPxKVV1IbI1ZiSX8B2Bs76YwYmYyVymKscS8IQdp1ZqR6La6y8EQTN7ASgAT0KwA)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: findById(UnGbqnoSPZ-1akQnWs16JFPPxKVV1IbI1ZiSX8B2Bs76YwYmYyVymKscS8IQdp1ZqR6La6y8EQTN7ASgAT0KwA)
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

> @ekodb/ekodb-client@0.26.4 prepare
> npm run build


> @ekodb/ekodb-client@0.26.4 build
> tsc


up to date, audited 46 packages in 4s

12 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.26.4 build
> tsc

✅ TypeScript client built!

added 1 package, removed 1 package, and audited 9 packages in 3s

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities

added 1 package, removed 1 package, and audited 13 packages in 3s

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> ekodb-typescript-examples@1.0.0 build
> tsc

bash: warning: setlocale: LC_ALL: cannot change locale (C.UTF-8): No such file or directory
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
✓ Created session: 7C-N9dULIr9A-vLIwmI4GH2gp9TftHWmSOYLC1noqeuc1u4tZ3HhpUZR8ZjFFHzMaORwHDR7DEQ9fetcJ_Yptg

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** High-performance database product
- **Price:** $99

If you have any more questions or need further information, feel free to ask!

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
✓ Created second session: hCkoSQHcG22ihQqUV0v7VbGtTJXM0VbX73mN9kqlbZez8zmP9iT7VahYxJLfzOXBuJdqIYQzWS6H6PJTvyU3kg
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 7

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 6

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: z-5DbLxhrjnqZOcmcj8j_0Wdsj-ytze8jY0rqjCVibULNicquu8OHgdWxkuvTeSidmVl4exznBdI0l-AgvhXHQ

=== Sending Chat Message ===
Message ID: RwxVWoGms3JdiSa77MncQNBQBG59DkVBrcx44sH6MYWK40NwA-br2UCw3tmRZ27EmmOWeQkFArZf1p7mKvJ46Q

=== AI Response ===
Here are the available products along with their prices:

1. **ekoDB Cloud**
   - Price: $499
   - Description: Fully managed cloud database service product

2. **ekoDB**
   - Price: $99
   - Description: A high-performance database product with AI capabilities

3. **ekoDB Pro**
   - Price: $299
   - Description: Enterprise edition product with advanced features

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_js',
  record: {
    price: 499,
    name: 'ekoDB Cloud',
    description: 'Fully managed cloud database service product',
    id: 'eWArQCVvbmm-PHKBCjpXnrfqDV2KScsHeaacktY3lCS3KciyIWFJ3uR5OtGIchqyybMFsM7lavsWMwmJ1z1yUw'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_js',
  record: {
    price: 99,
    id: '0FbZA3OMJjK0bsViUXHNNOZfB_ZQ4QPN_VCjd1AKAAL1YiI5UEwfVCpNHO0AQCtap4Ev_cFWq8kQoJYR3f6Eow',
    name: 'ekoDB',
    description: 'A high-performance database product with AI capabilities'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_js',
  record: {
    name: 'ekoDB Pro',
    description: 'Enterprise edition product with advanced features',
    id: 'Aez4fHwJYOknfEStP3fvVzbTBvFZzPWalHBIfWa6KPC5mFfZnPFQcgEhVXEIeevRFchglDu3McmB91OkvVCDmg',
    price: 299
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 6681ms

=== Token Usage ===
Prompt tokens: 3413
Completion tokens: 91
Total tokens: 3504

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ Chat completed successfully
✓ Client created

=== List Chat Models ===
Available chat models by provider:
  openai:
    - text-embedding-ada-002
    - whisper-1
    - gpt-3.5-turbo
    - tts-1
    - gpt-3.5-turbo-16k
    - gpt-4-0613
    - gpt-4
    - davinci-002
    - babbage-002
    - gpt-3.5-turbo-instruct
    - gpt-3.5-turbo-instruct-0914
    - gpt-3.5-turbo-1106
    - tts-1-hd
    - tts-1-1106
    - tts-1-hd-1106
    - text-embedding-3-small
    - text-embedding-3-large
    - gpt-3.5-turbo-0125
    - gpt-4-turbo
    - gpt-4-turbo-2024-04-09
    - gpt-4o
    - gpt-4o-2024-05-13
    - gpt-4o-mini-2024-07-18
    - gpt-4o-mini
    - gpt-4o-2024-08-06
    - omni-moderation-latest
    - omni-moderation-2024-09-26
    - o1-2024-12-17
    - o1
    - o3-mini
    - o3-mini-2025-01-31
    - gpt-4o-2024-11-20
    - gpt-4o-mini-search-preview-2025-03-11
    - gpt-4o-mini-search-preview
    - gpt-4o-transcribe
    - gpt-4o-mini-transcribe
    - o1-pro-2025-03-19
    - o1-pro
    - gpt-4o-mini-tts
    - o3-2025-04-16
    - o4-mini-2025-04-16
    - o3
    - o4-mini
    - gpt-4.1-2025-04-14
    - gpt-4.1
    - gpt-4.1-mini-2025-04-14
    - gpt-4.1-mini
    - gpt-4.1-nano-2025-04-14
    - gpt-4.1-nano
    - gpt-image-1
    - o4-mini-deep-research
    - gpt-4o-transcribe-diarize
    - o4-mini-deep-research-2025-06-26
    - gpt-5-chat-latest
    - gpt-5-2025-08-07
    - gpt-5
    - gpt-5-mini-2025-08-07
    - gpt-5-mini
    - gpt-5-nano-2025-08-07
    - gpt-5-nano
    - gpt-audio-2025-08-28
    - gpt-realtime
    - gpt-realtime-2025-08-28
    - gpt-audio
    - gpt-5-codex
    - gpt-image-1-mini
    - gpt-5-pro-2025-10-06
    - gpt-5-pro
    - gpt-audio-mini
    - gpt-audio-mini-2025-10-06
    - gpt-5-search-api
    - gpt-realtime-mini
    - sora-2
    - sora-2-pro
    - gpt-5-search-api-2025-10-14
    - gpt-5.1-chat-latest
    - gpt-5.1-2025-11-13
    - gpt-5.1
    - gpt-5.1-codex
    - gpt-5.1-codex-mini
    - gpt-5.1-codex-max
    - gpt-image-1.5
    - gpt-5.2-2025-12-11
    - gpt-5.2
    - gpt-5.2-pro-2025-12-11
    - gpt-5.2-pro
    - gpt-5.2-chat-latest
    - gpt-4o-mini-transcribe-2025-12-15
    - gpt-4o-mini-transcribe-2025-03-20
    - gpt-4o-mini-tts-2025-03-20
    - gpt-4o-mini-tts-2025-12-15
    - gpt-realtime-mini-2025-12-15
    - gpt-audio-mini-2025-12-15
    - chatgpt-image-latest
    - gpt-5.2-codex
    - gpt-5.3-codex
    - gpt-realtime-1.5
    - gpt-audio-1.5
    - gpt-4o-search-preview
    - gpt-4o-search-preview-2025-03-11
    - gpt-5.3-chat-latest
    - gpt-5.4-2026-03-05
    - gpt-5.4-pro
    - gpt-5.4-pro-2026-03-05
    - gpt-5.4
    - gpt-5.4-nano-2026-03-17
    - gpt-5.4-nano
    - gpt-5.4-mini-2026-03-17
    - gpt-5.4-mini
    - gpt-image-2
    - gpt-image-2-2026-04-21
    - gpt-5.5
    - gpt-5.5-2026-04-23
    - gpt-5.5-pro
    - gpt-5.5-pro-2026-04-23
    - chat-latest
    - gpt-realtime-translate
    - gpt-realtime-2
    - gpt-realtime-whisper
    - gpt-5.6-sol
    - gpt-5.6-terra
    - gpt-5.6-luna
    - gpt-realtime-2.1
    - gpt-realtime-2.1-mini
    - gpt-transcribe
    - gpt-live-transcribe
    - gpt-6-astra
    - gpt-image-2.5-flare
    - gpt-image-2.5-sunburst
    - gpt-image-2.5-flare-2026-09-08
    - gpt-image-2.5-sunburst-2026-09-08
    - gpt-live-1
  anthropic:
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
Provider status:
  anthropic: ok 11 models
  gemini: not_configured (unverified) No Gemini API Key
  openai: ok 132 models
  perplexity: not_configured (unverified) No Perplexity API Key

=== Get Specific Provider Models ===
OpenAI models: text-embedding-ada-002, whisper-1, gpt-3.5-turbo, tts-1, gpt-3.5-turbo-16k, gpt-4-0613, gpt-4, davinci-002, babbage-002, gpt-3.5-turbo-instruct, gpt-3.5-turbo-instruct-0914, gpt-3.5-turbo-1106, tts-1-hd, tts-1-1106, tts-1-hd-1106, text-embedding-3-small, text-embedding-3-large, gpt-3.5-turbo-0125, gpt-4-turbo, gpt-4-turbo-2024-04-09, gpt-4o, gpt-4o-2024-05-13, gpt-4o-mini-2024-07-18, gpt-4o-mini, gpt-4o-2024-08-06, omni-moderation-latest, omni-moderation-2024-09-26, o1-2024-12-17, o1, o3-mini, o3-mini-2025-01-31, gpt-4o-2024-11-20, gpt-4o-mini-search-preview-2025-03-11, gpt-4o-mini-search-preview, gpt-4o-transcribe, gpt-4o-mini-transcribe, o1-pro-2025-03-19, o1-pro, gpt-4o-mini-tts, o3-2025-04-16, o4-mini-2025-04-16, o3, o4-mini, gpt-4.1-2025-04-14, gpt-4.1, gpt-4.1-mini-2025-04-14, gpt-4.1-mini, gpt-4.1-nano-2025-04-14, gpt-4.1-nano, gpt-image-1, o4-mini-deep-research, gpt-4o-transcribe-diarize, o4-mini-deep-research-2025-06-26, gpt-5-chat-latest, gpt-5-2025-08-07, gpt-5, gpt-5-mini-2025-08-07, gpt-5-mini, gpt-5-nano-2025-08-07, gpt-5-nano, gpt-audio-2025-08-28, gpt-realtime, gpt-realtime-2025-08-28, gpt-audio, gpt-5-codex, gpt-image-1-mini, gpt-5-pro-2025-10-06, gpt-5-pro, gpt-audio-mini, gpt-audio-mini-2025-10-06, gpt-5-search-api, gpt-realtime-mini, sora-2, sora-2-pro, gpt-5-search-api-2025-10-14, gpt-5.1-chat-latest, gpt-5.1-2025-11-13, gpt-5.1, gpt-5.1-codex, gpt-5.1-codex-mini, gpt-5.1-codex-max, gpt-image-1.5, gpt-5.2-2025-12-11, gpt-5.2, gpt-5.2-pro-2025-12-11, gpt-5.2-pro, gpt-5.2-chat-latest, gpt-4o-mini-transcribe-2025-12-15, gpt-4o-mini-transcribe-2025-03-20, gpt-4o-mini-tts-2025-03-20, gpt-4o-mini-tts-2025-12-15, gpt-realtime-mini-2025-12-15, gpt-audio-mini-2025-12-15, chatgpt-image-latest, gpt-5.2-codex, gpt-5.3-codex, gpt-realtime-1.5, gpt-audio-1.5, gpt-4o-search-preview, gpt-4o-search-preview-2025-03-11, gpt-5.3-chat-latest, gpt-5.4-2026-03-05, gpt-5.4-pro, gpt-5.4-pro-2026-03-05, gpt-5.4, gpt-5.4-nano-2026-03-17, gpt-5.4-nano, gpt-5.4-mini-2026-03-17, gpt-5.4-mini, gpt-image-2, gpt-image-2-2026-04-21, gpt-5.5, gpt-5.5-2026-04-23, gpt-5.5-pro, gpt-5.5-pro-2026-04-23, chat-latest, gpt-realtime-translate, gpt-realtime-2, gpt-realtime-whisper, gpt-5.6-sol, gpt-5.6-terra, gpt-5.6-luna, gpt-realtime-2.1, gpt-realtime-2.1-mini, gpt-transcribe, gpt-live-transcribe, gpt-6-astra, gpt-image-2.5-flare, gpt-image-2.5-sunburst, gpt-image-2.5-flare-2026-09-08, gpt-image-2.5-sunburst-2026-09-08, gpt-live-1

=== Get Anthropic Models ===
Anthropic models: claude-fable-5-1, claude-opus-5, claude-sonnet-5, claude-fable-5, claude-opus-4-8, claude-opus-4-7, claude-sonnet-4-6, claude-opus-4-6, claude-opus-4-5-20251101, claude-haiku-4-5-20251001, claude-sonnet-4-5-20250929

=== Get Non-Existent Provider ===
Expected error for non-existent provider: Error: Request failed with status 404: {"error":"Unknown provider 'nonexistent_provider_xyz'","error_kind":"unknown_provider"}

✓ Chat Models example complete
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: lXOYCcqcKWvHw5gX-4Njcz1Z7aDg_4hAtIeYtj6yqWejyAuCJsCYqeFiBc7y8H5S--XeDs3I4sDPe8otKzh8SA

=== Sending Messages ===
✓ Message 1 sent
  Response: Currently, the available product is:

- **Name:** ekoDB
- **Description:** A high-performance database product
- **Price:** $99

If you need more information or details about other products, please let me know!

✓ Message 2 sent
  Response: The price of ekoDB is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: bK3Jeh_ye6jDStQmcUt2VHZB2DbEV-oUUnX-50nF1Gs7qWngtlK-DT57QTvJZLCZen2D9IQI5RUVGV63x-s2pA
  Parent: lXOYCcqcKWvHw5gX-4Njcz1Z7aDg_4hAtIeYtj6yqWejyAuCJsCYqeFiBc7y8H5S--XeDs3I4sDPe8otKzh8SA

=== Listing Sessions ===
✓ Found 7 sessions
  Session 1: bK3Jeh_ye6jDStQmcUt2VHZB2DbEV-oUUnX-50nF1Gs7qWngtlK-DT57QTvJZLCZen2D9IQI5RUVGV63x-s2pA (Untitled)
  Session 2: lXOYCcqcKWvHw5gX-4Njcz1Z7aDg_4hAtIeYtj6yqWejyAuCJsCYqeFiBc7y8H5S--XeDs3I4sDPe8otKzh8SA (Untitled)
  Session 3: hCkoSQHcG22ihQqUV0v7VbGtTJXM0VbX73mN9kqlbZez8zmP9iT7VahYxJLfzOXBuJdqIYQzWS6H6PJTvyU3kg (Untitled)
  Session 4: vuKbaGHsQC94yLa7_2ro5Vdy1xg9A2RA5KvKSyzx1oJLcCAi4D6DZ8eKRvWcmz0zxkBszT3iMxPubOmpZYLlXw (Untitled)
  Session 5: YUYboJLJZ3yqwxN1Cb5yCHopx8VzkWngmGBfDuMzwk5ocLrAA3onhZxGs9kFbV_RmSc9V1V2HhdynIh_f9G3Ww (Untitled)
  Session 6: MAxsP25SeTa3zmDskKituh1NEf7CdZJC9f2ywIhfXYYV748h-w5M7KwY6MKOrpZK8nSLu8xnE7EilU_pKoIjUg (Untitled)
  Session 7: SflIr8vmbb7jS_W_FfaYgsH9mYpw2-RnHYV0wV1SwX1f-j6HhxF5WJTlBhQmO8kpxpJki7WWPq2VM9MJBc5OQg (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: bK3Jeh_ye6jDStQmcUt2VHZB2DbEV-oUUnX-50nF1Gs7qWngtlK-DT57QTvJZLCZen2D9IQI5RUVGV63x-s2pA

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ All session management operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: 1sMzGLznO_2yNqKHYMYewkCuEKNbD78UNdOTeQ3obu1y5esYPWAitTUjVhaiRjRYpd_bcSvbFKNnNeoxplygYA

=== List Collections ===
Total collections: 33
Sample collections: schema_employees_client_ts,schema_products_client_go,agent_function_versions__ek0_testing,chat_agent_configs__ek0_testing,chat_goal_templates__ek0_testing

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: QmzH-QI5oYOfghWmXH0pi14rdqVeZFDGIm4DVaJoSKUWmv6NkAT7Y9IDcWckL1B57us9PaY3z87KwosMUtbxEQ

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: PCfTFvc3d3NeJ9hqgUdP_rBgTDAuh4R8KUqhF2iFee34zOmt7Fbiqd1-FOq2ybyo9xPLIKICnjLTFATs8uus5g

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
✓ Edge cache script created: wV_6d37BMQkiluixPD6uxsnVvChxDV3u0w2I0eYsPIzPQmZn3KzuhHBzp-o8ovDDl-5V4dQ0m5BTO59qVvB0oQ

Call 1: Cache miss (fetches from API)
Response time: 454ms
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "timezone_abbreviation": "GMT",
          "current_units": {
            "interval": "seconds",
            "time": "iso8601",
            "temperature_2m": "°C"
          },
          "utc_offset_seconds": 0,
          "generationtime_ms": 0.035762786865234375,
          "longitude": -73.99308,
          "latitude": 40.710335,
          "elevation": 32,
          "timezone": "GMT",
          "current": {
            "temperature_2m": 22.3,
            "interval": 900,
            "time": "2026-09-15T18:00"
          }
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

Call 2: Cache hit (served from ekoDB)
Response time: 14ms (32.4x faster!)
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "timezone_abbreviation": "GMT",
          "current_units": {
            "interval": "seconds",
            "time": "iso8601",
            "temperature_2m": "°C"
          },
          "utc_offset_seconds": 0,
          "generationtime_ms": 0.035762786865234375,
          "longitude": -73.99308,
          "latitude": 40.710335,
          "elevation": 32,
          "timezone": "GMT",
          "current": {
            "temperature_2m": 22.3,
            "interval": 900,
            "time": "2026-09-15T18:00"
          }
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

ℹ️  Function 'fetch_user' already existed — updated instead
✅ Saved reusable function: fetch_user
ℹ️  Function 'get_user_wrapper' already existed — updated instead
✅ Saved composed function: get_user_wrapper (calls fetch_user + projects fields)

📊 Result from composed function:
   Records: 1
   Name: {"type":"String","value":"User 1"}
   Department: {"type":"String","value":"engineering"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

ℹ️  Function 'fetch_and_store_user' already existed — updated instead
✅ Saved reusable function: fetch_and_store_user (uses KV)
ℹ️  Function 'swr_user' already existed — updated instead
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 163ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "company": {
        "name": "Romaguera-Crona",
        "bs": "harness real-time e-markets",
        "catchPhrase": "Multi-layered client-serv...

Second call (cache hit - from cache):
   ⏱️  Duration: 64ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "company": {
        "name": "Romaguera-Crona",
        "bs": "harness real-time e-markets",
        "catchPhrase": "Multi-layered client-serv...
   🚀 Cache speedup: 2.5x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

ℹ️  Function 'validate_user' already existed — updated instead
✅ Level 1 function: validate_user
ℹ️  Function 'fetch_slim_user' already existed — updated instead
✅ Level 2 function: fetch_slim_user (calls validate_user)
ℹ️  Function 'get_verified_user' already existed — updated instead
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

✅ Function saved: __zCG9k_e6iC2dlKBVqyj6gAKFms1hp5fEr7JV0vnhZ8r6cBdqJVBHlusXY6kU7Gk2KWXsE-ralYbMI9pv-hAA
📊 Found 5 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Function

✅ Function saved
📊 Found 3 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Function

✅ Function saved
📊 Statistics: 2 groups
   {"avg_score":{"value":60,"type":"Float"},"count":{"value":5,"type":"Integer"},"status":{"type":"String","value":"active"}}
   {"count":{"type":"Integer","value":5},"avg_score":{"type":"Float","value":50},"status":{"value":"inactive","type":"String"}}
⏱️  Execution time: 0ms

📝 Example 4: Function Management

📋 Total functions: 15
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
   1. Laptop Pro - $1299 (⭐4.8)
   2. Webcam HD - $119 (⭐4.5)
   3. Desk Chair - $349 (⭐4.2)
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
   1. Higher precision: Vector databases use mathematical points, lines, and polygons to represent spatial data, hence provide draw precision compared to raster databases.

2. Scalability: The data can be scaled up or down without losing quality, making it suitable for mapping operations on various scales.

3. Less storage: Vector databases tend to use less storage space compared to raster databases because they only need to store information about the vertices and paths of the geometric figures.

4. Detailed attributes: They allow the attachment of extensive attributes to individual objects, which provides comprehensive information for each spatial feature.

5. Better Analysis: Vector data support a wide spectrum of analysis like topological and network analysis which might not be supported in a raster database.

6. Easier editing: It is generally easier and more versatile to edit vector objects compared to raster objects.

⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embedding function saved
📊 Generated embeddings for 3 articles
   1. "Draft Article" - 0D vector
   2. "Getting Started with ekoDB" - 0D vector
   3. "Advanced Query Patterns" - 0D vector
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

✅ Function saved: -kWyfj0OJJq-oqirBP7d_0noSgjU4hzfEj9MPEgVbvrTzglQGvQHt0qpJB9UJgy0H33LLhtMbnbtZ3J5tggZhg
📊 Found 2 product groups
   {"category":{"value":"Electronics","type":"String"},"avg_price":{"type":"Float","value":575.6666666666666},"count":{"type":"Integer","value":3}}
   {"avg_price":{"value":474,"type":"Float"},"category":{"type":"String","value":"Furniture"},"count":{"type":"Integer","value":2}}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   {"category":{"value":"Electronics","type":"String"},"count":{"type":"Integer","value":3}}
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
   1. User 7 - Score: 70
   2. User 6 - Score: 60
   3. User 5 - Score: 50
   4. User 4 - Score: 40
   5. User 8 - Score: 80
⏱️  Execution time: 0ms

📝 Example 5: User Summary Statistics

✅ Function saved
📊 User summary (2 groups):
   active users: 7 users, Total Score: 370
   inactive users: 3 users, Total Score: 180
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

✅ Inserted order: mRy1cjtAKIz-00hEiZb_3UC1w-tI9Q-viqDxPQUPmzmpeBSGruINI3XkrFjMEw-ZUxzsiyGsXlnokWr9CjvrnA
✅ Inserted 2 products with wrapped types

📝 Example 2: Function with Wrapped Type Parameters

✅ Function saved: ICaVe7UPSxr45L0n6JW0eAYmnBlbYviaevg5HfBn12ieLEHuR4Da9rqCcu9bcTyrUMujFgA--gBcNKRi_rhF2g
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"value":{"role":"admin","userId":"user_abc"},"type":"Object"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: ZW6XSLEWzNWFj2O3AXv-CDadGsa1MfE4w02LT4ZC22oZdmVwX7MkmXJpYsP6jwzGFLY-aXEXZ5R-SSxJntNf9g
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 3 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Function

✅ Function saved: FpLJ_ZAPzhgO64eVTNnf9naIwRFt7mdRLU8-N4bJ_X7UHPvoZYoJWGZVvlZg6-jOaQjau-ccnDLgUsCQcmSqew
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
   3. Database Design Principles (Database)
   4. Natural Language Processing (AI)
   5. Vector Databases Explained (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   Database: 3 documents
   AI: 2 documents
⏱️  Execution time: 0ms

📝 Example 3: Select Specific Fields

✅ Function saved
📊 Document titles (5 docs):
   1. Introduction to Machine Learning
   2. Getting Started with ekoDB
   3. Database Design Principles
   4. Natural Language Processing
   5. Vector Databases Explained
⏱️  Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Function saved
📊 Projected documents (showing first 3):
   1. Introduction to Machine Learning
   2. Getting Started with ekoDB
   3. Database Design Principles
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

=== Cleanup ===
✅ Deleted test collections

✅ Join operations examples completed!
✓ Client created

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: { type: 'Object', value: { userId: 123, username: 'john_doe' } }

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: { value: { name: 'Product 1', price: 29.99 }, type: 'Object' }
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
Total keys in store: 9

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
  1. Score: 12.870, Matched: name, email
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
  2. Score: 26.400, Matched: bio, title
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
  1. Score: 0.774, Matched:
  2. Score: 0.752, Matched:
  3. Score: 0.742, Matched:

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.501, Matched: content, title
  2. Score: 0.897, Matched: title, content
  3. Score: 0.310, Matched:

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 13.200, Matched: skills, bio

=== Cleanup ===
✅ Deleted test collections

✅ Search examples completed!
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: '4-MurzqSw7MnkPrGv4sdvEF9RQMHE8AGa2bU7o3ywT-ix5adnxaY7tLHCaQWObIjxsHIjREUP-WbTfOUxgVGnA'
}

=== Find by ID ===
Found: {
  active: { value: true, type: 'Boolean' },
  value: { type: 'Integer', value: 42 },
  name: { type: 'String', value: 'Test Record' },
  id: '4-MurzqSw7MnkPrGv4sdvEF9RQMHE8AGa2bU7o3ywT-ix5adnxaY7tLHCaQWObIjxsHIjREUP-WbTfOUxgVGnA'
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  id: '4-MurzqSw7MnkPrGv4sdvEF9RQMHE8AGa2bU7o3ywT-ix5adnxaY7tLHCaQWObIjxsHIjREUP-WbTfOUxgVGnA',
  active: { type: 'Boolean', value: true },
  name: { type: 'String', value: 'Updated Record' },
  value: { type: 'Integer', value: 100 }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: t4BsuG2ZgFq1W93NHcepGe_a-DIwpFNonCkRKf3Se7omjY5EEJxj5WkqBgEV5QLfR9RCtbHrMQ5Pr1Ohi4hsRQ

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
✓ Created native SWR script: github_user_native_js (Q5DHB75F-D-dmS97FXTNy7nDq6y793GZ6BmM1qDKKqtScyu-HaQSkbwqVbpWHAwR9A-kGXwGJFHdcslkd2nmHw)

First call (cache miss - will fetch from GitHub API):
  Response time: 207ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 14ms
  Speedup: 14.8x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit_js (dERZX2aScueMliv68ihQNW-wKmWvI_qh8PiGm25ay4l9OvuLn13wYYmla6zo22IUJ6YCiTwQYDyP0oF_Hksahg)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail_js' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline_js (2Wp-Mafbmyp4yf48_I9ryO8E3bH296Jqy3v06z1qYjhSF-f2LNhGLHUY5VFv-yE0z6xP7Rx-lu8ESgO4hjMiAQ)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users_swr_js' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache_js (z3ATGw-bdEdN4GbJe_kW6hEdz1SJ0gqXKHgdaohQCWvi_pvR8b8GS9h0GNVwQxSBMMMiWUrM3AtFe3w092bfDA)
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
✓ Created SWR script: fetch_api_user_js (Pebyn8mG_MOexJFUp93R8ePk3POogPYF_dWqtq7afbJ0oNqWLU7QOJnOlTDlKgZSyVmYOa5E0yueXSnPxnlUvg)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "value": {
          "website": "hildegard.org",
          "address": {
            "street": "Kulas Light",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "suite": "Apt. 556",
            "zipcode": "92998-3874",
            "city": "Gwenborough"
          },
          "name": "Leanne Graham",
          "company": {
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net"
          },
          "email": "Sincere@april.biz",
          "username": "Bret",
          "id": 1,
          "phone": "1-770-736-8031 x56442"
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
Response time: 6ms (served from cache)
✓ Lightning fast cache hit

🧹 Cleaning up...
✓ Cleanup complete

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: e1OV4akIF_yUMFU02CS48N4xX_kkRsWC8OKzGfL_qnylfH8G89wGQJjlcETQms7IPsRA-mBI6QSwQxqKWW-uMg
Created Bob: $500 - ID: 1IZQUMZVOjYVS9omoyBHBdsx-QxVbelov81SZwXbUxUOcqzzya0ybRB3t3FdrOt-6KW08N4sryJca5TfocfjKA

=== Example 1: Begin Transaction ===
Transaction ID (server-default isolation): 4768cd4e-f3a3-4d5a-a8f0-98d577547dfc

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 59731f6c-5c40-4d69-85dc-d185cd7faec3
Status before rollback: Active
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: IZ6ashqF2X4Ed5cepsuBmGy66ghWSsXpicZdnBNqLASpgDdU1coorWWGYMGkjDlbf0pFt_XeJwRRWP1KnpGDlA

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
Inserted: Vdm3UXvLqusN1ECKcbfpY3P2nwSSDxkIMAQNA9LbAXbmnHn8F9Z5k2-Y7O7PVJPYOJd1jioGIc_F9JwZoFT5hA

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
✓ Created session: -NKOl2jyh_bSKHzclmHE1qHD2mW39pGpxIs6pzc42UpG8jdNk0A0f1k77831PyACssQnI0BH0KmYDRLNTYYGOQ

=== Sending Initial Message ===
✓ Message sent
  Response: It seems there are currently no active products available in the database. If you were looking for specific details about a product or if there's anything else you need, please let me know!

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
✓ Created second session: 8nmIAEuBvl3MZ2ga6cs9wDiH9-Lt73UPAMuab3OlVB_feqbh_jI7KTOo_62LHb2pxuALcwtL4F1iUdrJPtOD8Q
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 7

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 6

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: kYP1pDtSGKqcDF8jaFLNfsgyfWfH8fc0yJc2055X-6e81m7HhcFoDyIew54XwDeAVIHTf58psnhQg481Q3XucA

=== Sending Chat Message ===
Message ID: tSArvan9t5COeYs-pWQ4T3DzV5-yrUHoDx4wuy8VPeEOq4-yYyYqvMesCZz9YySmNsElMhlop4qYH32yopw6EQ

=== AI Response ===
Here are the products available along with their prices:

1. **ekoDB Pro**
   - Price: $299
   - Description: Enterprise edition product with advanced features

2. **ekoDB**
   - Price: $99
   - Description: A high-performance database product with AI capabilities

3. **ekoDB Cloud**
   - Price: $499
   - Description: Fully managed cloud database service product

If you need more information about any specific product, feel free to ask!

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    description: 'Enterprise edition product with advanced features',
    id: '8i2l7aBZY01JlLfYlqInxz--ab5mjEHbHPyTBao-Fvmntq6CvNWVGBNjfaeRuPV6yfT0052tjKyNA4v1J4g9qw',
    price: 299,
    name: 'ekoDB Pro'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    id: 'NJcru8_gOXbWSvdvWnAG1eez7ZnZqQfhZDVwWJs5XxD1nyqitFoLWIseRm_tr_A0qz8aT_W50RygUUvK8CGtFQ',
    name: 'ekoDB',
    price: 99,
    description: 'A high-performance database product with AI capabilities'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    description: 'Fully managed cloud database service product',
    price: 499,
    id: 'MjJ3QT7wxmv0LSCoj03ChR51Ykj_ZhquJ1ZRx5wUW6xngr266I7EImb2BwqPiX95_Shhxln0vJ2o0ziG2ICA1A',
    name: 'ekoDB Cloud'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 6024ms

=== Token Usage ===
Prompt tokens: 3413
Completion tokens: 110
Total tokens: 3523

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Message Stream (SSE) Example (TypeScript) ===

Created session: _W8uLBnKKl4FSapuSEKXVEjQtNlv7chSv9Geup_Imi-5S1jeviukqbx8o6fjmjA3L7KzTP85Z18-H2RIJqGgCQ

Streaming response for: 'What is ekoDB?'

{"__progress":"received"}{"__progress":"generating","model":"default","provider":"openai"}**ekoDB** is a curated knowledgebase dedicated to the field of **cancer ecosystems**. It integrates and organizes large-scale multi-omic datasets, computational tools, and literature, focusing on the complex cellular and molecular interactions within tumor microenvironments (TMEs). The goal of ekoDB is to facilitate cancer research by providing researchers with user-friendly access to annotated data and analytical resources relating to:

- Immune landscapes and immune cell infiltration in tumors
- Cell-cell communication within the TME
- Spatial organization and phenotypic diversity of tumor and immune cells
- Tumor heterogeneity and ecosystem dynamics

**Key features of ekoDB include**:
- Aggregated and harmonized single-cell, spatial transcriptomics, and bulk sequencing datasets from various cancer studies.
- Visualization tools for exploring the composition and dynamics of cancer ecosystems.
- Downloadable data and analysis results for further research.

ekoDB is designed to support the research community's efforts in unraveling the complexity of tumor ecology, improving biomarker discovery, and developing new therapeutic strategies.

**References:**
- [ekoDB official website](https://ekodb.dp.tech)
- Zhu, X. et al., *ekoDB: a comprehensive molecular ecology knowledgebase for cancer ecosystem research*, Nucleic Acids Research, 2024. [Link](https://academic.oup.com/nar/article/52/D1/D952/7374524)

--- Stream complete ---
Message ID: iYjHofZ1YX8NtxH3R0-ymEQHLb4d568EX-i-XExKOqcV1ZsiKCjB4XWUkFvuSIs-JeUiBy6unXFrpQfmDu5R7g
Execution time: 4465ms
Context window: 1000000 tokens

✓ Chat message stream example completed
✓ Client created

=== List Chat Models ===
Available chat models by provider:
  openai:
    - text-embedding-ada-002
    - whisper-1
    - gpt-3.5-turbo
    - tts-1
    - gpt-3.5-turbo-16k
    - gpt-4-0613
    - gpt-4
    - davinci-002
    - babbage-002
    - gpt-3.5-turbo-instruct
    - gpt-3.5-turbo-instruct-0914
    - gpt-3.5-turbo-1106
    - tts-1-hd
    - tts-1-1106
    - tts-1-hd-1106
    - text-embedding-3-small
    - text-embedding-3-large
    - gpt-3.5-turbo-0125
    - gpt-4-turbo
    - gpt-4-turbo-2024-04-09
    - gpt-4o
    - gpt-4o-2024-05-13
    - gpt-4o-mini-2024-07-18
    - gpt-4o-mini
    - gpt-4o-2024-08-06
    - omni-moderation-latest
    - omni-moderation-2024-09-26
    - o1-2024-12-17
    - o1
    - o3-mini
    - o3-mini-2025-01-31
    - gpt-4o-2024-11-20
    - gpt-4o-mini-search-preview-2025-03-11
    - gpt-4o-mini-search-preview
    - gpt-4o-transcribe
    - gpt-4o-mini-transcribe
    - o1-pro-2025-03-19
    - o1-pro
    - gpt-4o-mini-tts
    - o3-2025-04-16
    - o4-mini-2025-04-16
    - o3
    - o4-mini
    - gpt-4.1-2025-04-14
    - gpt-4.1
    - gpt-4.1-mini-2025-04-14
    - gpt-4.1-mini
    - gpt-4.1-nano-2025-04-14
    - gpt-4.1-nano
    - gpt-image-1
    - o4-mini-deep-research
    - gpt-4o-transcribe-diarize
    - o4-mini-deep-research-2025-06-26
    - gpt-5-chat-latest
    - gpt-5-2025-08-07
    - gpt-5
    - gpt-5-mini-2025-08-07
    - gpt-5-mini
    - gpt-5-nano-2025-08-07
    - gpt-5-nano
    - gpt-audio-2025-08-28
    - gpt-realtime
    - gpt-realtime-2025-08-28
    - gpt-audio
    - gpt-5-codex
    - gpt-image-1-mini
    - gpt-5-pro-2025-10-06
    - gpt-5-pro
    - gpt-audio-mini
    - gpt-audio-mini-2025-10-06
    - gpt-5-search-api
    - gpt-realtime-mini
    - sora-2
    - sora-2-pro
    - gpt-5-search-api-2025-10-14
    - gpt-5.1-chat-latest
    - gpt-5.1-2025-11-13
    - gpt-5.1
    - gpt-5.1-codex
    - gpt-5.1-codex-mini
    - gpt-5.1-codex-max
    - gpt-image-1.5
    - gpt-5.2-2025-12-11
    - gpt-5.2
    - gpt-5.2-pro-2025-12-11
    - gpt-5.2-pro
    - gpt-5.2-chat-latest
    - gpt-4o-mini-transcribe-2025-12-15
    - gpt-4o-mini-transcribe-2025-03-20
    - gpt-4o-mini-tts-2025-03-20
    - gpt-4o-mini-tts-2025-12-15
    - gpt-realtime-mini-2025-12-15
    - gpt-audio-mini-2025-12-15
    - chatgpt-image-latest
    - gpt-5.2-codex
    - gpt-5.3-codex
    - gpt-realtime-1.5
    - gpt-audio-1.5
    - gpt-4o-search-preview
    - gpt-4o-search-preview-2025-03-11
    - gpt-5.3-chat-latest
    - gpt-5.4-2026-03-05
    - gpt-5.4-pro
    - gpt-5.4-pro-2026-03-05
    - gpt-5.4
    - gpt-5.4-nano-2026-03-17
    - gpt-5.4-nano
    - gpt-5.4-mini-2026-03-17
    - gpt-5.4-mini
    - gpt-image-2
    - gpt-image-2-2026-04-21
    - gpt-5.5
    - gpt-5.5-2026-04-23
    - gpt-5.5-pro
    - gpt-5.5-pro-2026-04-23
    - chat-latest
    - gpt-realtime-translate
    - gpt-realtime-2
    - gpt-realtime-whisper
    - gpt-5.6-sol
    - gpt-5.6-terra
    - gpt-5.6-luna
    - gpt-realtime-2.1
    - gpt-realtime-2.1-mini
    - gpt-transcribe
    - gpt-live-transcribe
    - gpt-6-astra
    - gpt-image-2.5-flare
    - gpt-image-2.5-sunburst
    - gpt-image-2.5-flare-2026-09-08
    - gpt-image-2.5-sunburst-2026-09-08
    - gpt-live-1
  anthropic:
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
Provider status:
  anthropic: ok 11 models
  gemini: not_configured (unverified) No Gemini API Key
  openai: ok 132 models
  perplexity: not_configured (unverified) No Perplexity API Key

=== Get Specific Provider Models ===
OpenAI models: text-embedding-ada-002, whisper-1, gpt-3.5-turbo, tts-1, gpt-3.5-turbo-16k, gpt-4-0613, gpt-4, davinci-002, babbage-002, gpt-3.5-turbo-instruct, gpt-3.5-turbo-instruct-0914, gpt-3.5-turbo-1106, tts-1-hd, tts-1-1106, tts-1-hd-1106, text-embedding-3-small, text-embedding-3-large, gpt-3.5-turbo-0125, gpt-4-turbo, gpt-4-turbo-2024-04-09, gpt-4o, gpt-4o-2024-05-13, gpt-4o-mini-2024-07-18, gpt-4o-mini, gpt-4o-2024-08-06, omni-moderation-latest, omni-moderation-2024-09-26, o1-2024-12-17, o1, o3-mini, o3-mini-2025-01-31, gpt-4o-2024-11-20, gpt-4o-mini-search-preview-2025-03-11, gpt-4o-mini-search-preview, gpt-4o-transcribe, gpt-4o-mini-transcribe, o1-pro-2025-03-19, o1-pro, gpt-4o-mini-tts, o3-2025-04-16, o4-mini-2025-04-16, o3, o4-mini, gpt-4.1-2025-04-14, gpt-4.1, gpt-4.1-mini-2025-04-14, gpt-4.1-mini, gpt-4.1-nano-2025-04-14, gpt-4.1-nano, gpt-image-1, o4-mini-deep-research, gpt-4o-transcribe-diarize, o4-mini-deep-research-2025-06-26, gpt-5-chat-latest, gpt-5-2025-08-07, gpt-5, gpt-5-mini-2025-08-07, gpt-5-mini, gpt-5-nano-2025-08-07, gpt-5-nano, gpt-audio-2025-08-28, gpt-realtime, gpt-realtime-2025-08-28, gpt-audio, gpt-5-codex, gpt-image-1-mini, gpt-5-pro-2025-10-06, gpt-5-pro, gpt-audio-mini, gpt-audio-mini-2025-10-06, gpt-5-search-api, gpt-realtime-mini, sora-2, sora-2-pro, gpt-5-search-api-2025-10-14, gpt-5.1-chat-latest, gpt-5.1-2025-11-13, gpt-5.1, gpt-5.1-codex, gpt-5.1-codex-mini, gpt-5.1-codex-max, gpt-image-1.5, gpt-5.2-2025-12-11, gpt-5.2, gpt-5.2-pro-2025-12-11, gpt-5.2-pro, gpt-5.2-chat-latest, gpt-4o-mini-transcribe-2025-12-15, gpt-4o-mini-transcribe-2025-03-20, gpt-4o-mini-tts-2025-03-20, gpt-4o-mini-tts-2025-12-15, gpt-realtime-mini-2025-12-15, gpt-audio-mini-2025-12-15, chatgpt-image-latest, gpt-5.2-codex, gpt-5.3-codex, gpt-realtime-1.5, gpt-audio-1.5, gpt-4o-search-preview, gpt-4o-search-preview-2025-03-11, gpt-5.3-chat-latest, gpt-5.4-2026-03-05, gpt-5.4-pro, gpt-5.4-pro-2026-03-05, gpt-5.4, gpt-5.4-nano-2026-03-17, gpt-5.4-nano, gpt-5.4-mini-2026-03-17, gpt-5.4-mini, gpt-image-2, gpt-image-2-2026-04-21, gpt-5.5, gpt-5.5-2026-04-23, gpt-5.5-pro, gpt-5.5-pro-2026-04-23, chat-latest, gpt-realtime-translate, gpt-realtime-2, gpt-realtime-whisper, gpt-5.6-sol, gpt-5.6-terra, gpt-5.6-luna, gpt-realtime-2.1, gpt-realtime-2.1-mini, gpt-transcribe, gpt-live-transcribe, gpt-6-astra, gpt-image-2.5-flare, gpt-image-2.5-sunburst, gpt-image-2.5-flare-2026-09-08, gpt-image-2.5-sunburst-2026-09-08, gpt-live-1

=== Get Anthropic Models ===
Anthropic models: claude-fable-5-1, claude-opus-5, claude-sonnet-5, claude-fable-5, claude-opus-4-8, claude-opus-4-7, claude-sonnet-4-6, claude-opus-4-6, claude-opus-4-5-20251101, claude-haiku-4-5-20251001, claude-sonnet-4-5-20250929

=== Get Non-Existent Provider ===
Expected error for non-existent provider: Error: Request failed with status 404: {"error":"Unknown provider 'nonexistent_provider_xyz'","error_kind":"unknown_provider"}

✓ Chat Models example complete
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: hm5aw9utC9F2v7PS_NrYOqDbK2eZGIjDigYqh5qDKz6u1j-upqHAEMw59MVOxT2Q9Upj04O-zFJ9imVa1GK1wA

=== Sending Messages ===
✓ Message 1 sent
  Response: It appears that there are no products available in the queried database. However, I can confirm that we have a product called **ekoDB** with the following details:

- **Description:** A high-performance database product
- **Price:** $99

If you need further assistance or want more information, feel free to ask!

✓ Message 2 sent
  Response: The price of **ekoDB** is **$99**. If you have any more questions or need further information, feel free to ask!

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 1egLKgFpPARPm4C5xVrm7Z3hQlI4eaDhmRhovvZZciai8VxNT5wPR4aSQqssZDzj2hdT5kWQSdfxdJKHk4jaIA
  Parent: hm5aw9utC9F2v7PS_NrYOqDbK2eZGIjDigYqh5qDKz6u1j-upqHAEMw59MVOxT2Q9Upj04O-zFJ9imVa1GK1wA

=== Listing Sessions ===
✓ Found 8 sessions
  Session 1: 1egLKgFpPARPm4C5xVrm7Z3hQlI4eaDhmRhovvZZciai8VxNT5wPR4aSQqssZDzj2hdT5kWQSdfxdJKHk4jaIA (Untitled)
  Session 2: hm5aw9utC9F2v7PS_NrYOqDbK2eZGIjDigYqh5qDKz6u1j-upqHAEMw59MVOxT2Q9Upj04O-zFJ9imVa1GK1wA (Untitled)
  Session 3: 8nmIAEuBvl3MZ2ga6cs9wDiH9-Lt73UPAMuab3OlVB_feqbh_jI7KTOo_62LHb2pxuALcwtL4F1iUdrJPtOD8Q (Untitled)
  Session 4: hCkoSQHcG22ihQqUV0v7VbGtTJXM0VbX73mN9kqlbZez8zmP9iT7VahYxJLfzOXBuJdqIYQzWS6H6PJTvyU3kg (Untitled)
  Session 5: vuKbaGHsQC94yLa7_2ro5Vdy1xg9A2RA5KvKSyzx1oJLcCAi4D6DZ8eKRvWcmz0zxkBszT3iMxPubOmpZYLlXw (Untitled)
  Session 6: YUYboJLJZ3yqwxN1Cb5yCHopx8VzkWngmGBfDuMzwk5ocLrAA3onhZxGs9kFbV_RmSc9V1V2HhdynIh_f9G3Ww (Untitled)
  Session 7: MAxsP25SeTa3zmDskKituh1NEf7CdZJC9f2ywIhfXYYV748h-w5M7KwY6MKOrpZK8nSLu8xnE7EilU_pKoIjUg (Untitled)
  Session 8: SflIr8vmbb7jS_W_FfaYgsH9mYpw2-RnHYV0wV1SwX1f-j6HhxF5WJTlBhQmO8kpxpJki7WWPq2VM9MJBc5OQg (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: 1egLKgFpPARPm4C5xVrm7Z3hQlI4eaDhmRhovvZZciai8VxNT5wPR4aSQqssZDzj2hdT5kWQSdfxdJKHk4jaIA

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ All session management operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: nOlF3SdXSfWt1Xn4qLCnzQNINRhwtcPYNpeC7JvOW1o6D54d2Sih45k9bnW9GRiPlI21gzG5t0no3A60cOOCag

=== List Collections ===
Total collections: 37
Sample collections: schema_employees_client_ts,schema_products_client_go,agent_function_versions__ek0_testing,chat_agent_configs__ek0_testing,chat_goal_templates__ek0_testing

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
  POST /api/functions/conc_demo_pay        { "idempotency_key": "...", "amount": 100 }
  POST /api/functions/conc_demo_rl_fail    { "user_id": 42 }
  POST /api/functions/conc_demo_rl_skip    { "user_id": 42 }
  POST /api/functions/conc_demo_lock       { "resource": "queue:drain" }

✓ Cleaned up demo functions
=== ekoDB Convenience Methods Example ===

=== Native Object Creation ===
✓ Created record with plain object: {
  id: 'XzXPnN9tq7E4RmyD3jGTraCLVRBvzsNFGOJucKgX0PJ4nMI-1Xfx4pDr39oJFydDni1boHXr3YQTkonY0T-hnA'
}

=== Upsert Operation ===
✓ First upsert (update): {
  id: 'XzXPnN9tq7E4RmyD3jGTraCLVRBvzsNFGOJucKgX0PJ4nMI-1Xfx4pDr39oJFydDni1boHXr3YQTkonY0T-hnA',
  active: { type: 'Boolean', value: true },
  email: { type: 'String', value: 'alice.j@newdomain.com' },
  age: { type: 'Integer', value: 29 },
  name: { type: 'String', value: 'Alice Johnson' }
}
✓ Second upsert (insert): {
  id: 'cpTn0L_dLmX_aj7LKXTMUXlsZ2s_knHGCHhQTBKrmLFGz7QCgPyhtOSOOoD1jz_IOHEyylWOBLtZNq5kVfhgOQ'
}

=== Find One Operation ===
✓ Found user by email: {
  id: 'XzXPnN9tq7E4RmyD3jGTraCLVRBvzsNFGOJucKgX0PJ4nMI-1Xfx4pDr39oJFydDni1boHXr3YQTkonY0T-hnA',
  name: { type: 'String', value: 'Alice Johnson' },
  age: { type: 'Integer', value: 29 },
  active: { type: 'Boolean', value: true },
  email: { value: 'alice.j@newdomain.com', type: 'String' }
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
✓ crypto_demo_hmac saved
✓ crypto_demo_aes saved
✓ crypto_demo_uuid saved
✓ crypto_demo_totp saved
✓ crypto_demo_encoding saved

Invoke them with:
  POST /api/functions/crypto_demo_hmac     { "payload": "hi" }
  POST /api/functions/crypto_demo_aes      { "plaintext": "secret" }
  POST /api/functions/crypto_demo_uuid
  POST /api/functions/crypto_demo_totp
  POST /api/functions/crypto_demo_encoding { "title": "Héllo World" }

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
✓ Inserted document: FYkvoG21Ko59MH_SuncSj068k1FBIPf4bEWQbvV7YowV32OhGrUCeF4MIgzsATQgwzZ-hnAa1CELeTkPl75T0w

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: diOffO3UE6A0lwMZtDzMCVY3T1pzUGrDTrCtzVy3QG2y2KB_zpiR7CTwAGiIImIZ6eBE9s3Zay_KbvOz67nrjg

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
ℹ️  Function 'cache_api_call' already existed — updated instead
✓ Edge cache script updated: KGn5p2Nthgt1SmGcncaj0NQoxs8yMjFgT0pLDSua-8IZSkDxxui2PSux4X3CojN7N9BjFUyKA5dA92LRD6n_zA

Call 1: Cache miss (fetches from API)
Response time: 6ms
Result: {
  "records": [
    {
      "value": {
        "value": {
          "phone": "1-770-736-8031 x56442",
          "website": "hildegard.org",
          "company": {
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona"
          },
          "id": 1,
          "name": "Leanne Graham",
          "username": "Bret",
          "address": {
            "suite": "Apt. 556",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "zipcode": "92998-3874",
            "city": "Gwenborough",
            "street": "Kulas Light"
          },
          "email": "Sincere@april.biz"
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
Response time: 4ms (1.5x faster!)
Result: {
  "records": [
    {
      "value": {
        "value": {
          "phone": "1-770-736-8031 x56442",
          "website": "hildegard.org",
          "company": {
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona"
          },
          "id": 1,
          "name": "Leanne Graham",
          "username": "Bret",
          "address": {
            "suite": "Apt. 556",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "zipcode": "92998-3874",
            "city": "Gwenborough",
            "street": "Kulas Light"
          },
          "email": "Sincere@april.biz"
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

Function 'fetch_user' already existed — updated instead
✅ Saved reusable function: fetch_user
Function 'get_user_wrapper' already existed — updated instead
✅ Saved composed function: get_user_wrapper (calls fetch_user + projects fields)

📊 Result from composed function:
   Records: 1
   Name: {"type":"String","value":"User 1"}
   Department: {"type":"String","value":"engineering"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

Function 'fetch_and_store_user' already existed — updated instead
✅ Saved reusable function: fetch_and_store_user (uses KV)
Function 'swr_user' already existed — updated instead
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 6ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "company": {
        "name": "Romaguera-Crona",
        "bs": "harness real-time e-markets",
        "catchPhrase": "Multi-layered client-serv...

Second call (cache hit - from cache):
   ⏱️  Duration: 4ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "company": {
        "name": "Romaguera-Crona",
        "bs": "harness real-time e-markets",
        "catchPhrase": "Multi-layered client-server neural-net"
      }...
   🚀 Cache speedup: 1.5x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

Function 'validate_user' already existed — updated instead
✅ Level 1 function: validate_user
Function 'fetch_slim_user' already existed — updated instead
✅ Level 2 function: fetch_slim_user (calls validate_user)
Function 'get_verified_user' already existed — updated instead
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

✅ Function saved: uow8mTDdXdxqpArVGPwwCawr1omf79ydyKb3pofRcfJhX7iPDRA-TRUu4SJVvG73X520OKslCIHY6W9VszIC_Q
📊 Found 5 active users

📝 Example 2: Parameterized Function

✅ Function saved: mE4YtxMMeYXm8IbKB-epC7m3VOSY6OHNkJ9Fi1j5K7xl4Q4Lh85vuEC-3K-X5mnSnU_-Ofe9Is_FDoC7Mm8CEw
📊 Found 3 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: 58rzK48Xrb_ee7HXHc44RGUtV4-DjhyzHLaIz7wP7UuHGj5dPzBaUzi6gwCv04lQV7vtFlX_vWxGkqlYSVDMKw
📊 Statistics: 2 groups
   {"status":{"value":"inactive","type":"String"},"avg_score":{"value":50,"type":"Float"},"count":{"value":5,"type":"Integer"}}
   {"status":{"type":"String","value":"active"},"count":{"value":5,"type":"Integer"},"avg_score":{"value":60,"type":"Float"}}

📝 Example 4: UserFunction Management

📋 Total scripts: 15
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
   {"avg_price":{"type":"Float","value":367},"count":{"value":5,"type":"Integer"},"category":{"value":"Electronics","type":"String"}}
   {"avg_price":{"type":"Float","value":365.6666666666667},"count":{"value":3,"type":"Integer"},"category":{"type":"String","value":"Furniture"}}
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

1. **High-Dimensional Data Handling**: Efficiently manage and search vast amounts of high-dimensional data, such as embeddings from machine learning models.

2. **Fast Similarity Search**: Enable rapid nearest neighbor search, which is crucial for applications like recommendation systems and image/video retrieval.

3. **Scalability**: Designed to handle large-scale datasets with horizontal scalability, allowing for growth without performance degradation.

4. **Flexibility in Data Types**: Support various data types, including images, text, and audio, by representing them as vectors.

5. **Real-Time Processing**: Facilitate near-real-time analytics, which is valuable for dynamic applications that require immediate results.

6. **Integration with AI/ML**: Seamless integration with machine learning workflows for tasks like clustering, classification, and anomaly detection.

7. **Enhanced Performance**: Optimized for performance, reducing latency in queries compared to traditional databases when dealing with unstructured data.

8. **Support for Advanced Algorithms**: Enable the implementation of complex algorithms for tasks such as semantic search and natural language processing.
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embed script saved
📊 Embedding generated
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

✅ Function saved: Uli0Kih7kT2kCQhwmKNEWrENEkLXl4DcNnN29LVwWoA1Zces7YYEH3n_6Ox0tCQilHMwtzxI4RqTxhxY3IO9sw
📊 Found 2 product groups
   {"category":{"type":"String","value":"Electronics"},"count":{"type":"Integer","value":3},"avg_price":{"type":"Float","value":575.6666666666666}}
   {"count":{"type":"Integer","value":2},"avg_price":{"type":"Float","value":474},"category":{"value":"Furniture","type":"String"}}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   {"count":{"type":"Integer","value":3},"category":{"type":"String","value":"Electronics"}}
   {"category":{"type":"String","value":"Furniture"},"count":{"type":"Integer","value":2}}
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
   {"role":{"value":"user","type":"String"},"count":{"type":"Integer","value":7},"avg_score":{"type":"Float","value":70}}
   {"role":{"value":"admin","type":"String"},"count":{"type":"Integer","value":3},"avg_score":{"type":"Float","value":20}}
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

✅ Inserted order: 1RQT20Gfm8tVO9Smx-sR0h335fFhnFkY29PH_X-VPb4729z7FVUSpFSmTkNeqK07D7QnLPq9ycG_n2H1lwDagw
✅ Inserted 2 products with wrapped types

📝 Example 2: UserFunction with Wrapped Type Parameters

✅ Function saved: q-rBmMyzrwQJdiUD0WmBuZolg5eRJ7Or8D91vzoZMRz9LdjHJBvld9gvwTHisfhgfkgXEGDlC16o_6xpepeFoQ
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"type":"Object","value":{"role":"admin","userId":"user_abc"}}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: LLaoMfSW-UdNREUsRHs-b8yiBZW-y2SlmzRXwNIg7cMTxxXiXGORGq3c5CWFdeIbGrtDgYMVmfaw73YGZL1HcQ
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 3 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Function

✅ Function saved: RKfuuzYpeptF4btu-6d_vsFJHigB_f_-ksqB1qxKA50eRyWjrFNRcLhrL3ewCH8UMLkIMToXPrZSBtzp0XjihQ
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
   1. Vector Databases Explained (Database)
   2. Natural Language Processing (AI)
   3. Database Design Principles (Database)
   4. Introduction to Machine Learning (AI)
   5. Getting Started with ekoDB (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   {"count":{"type":"Integer","value":3},"category":{"value":"Database","type":"String"}}
   {"count":{"value":2,"type":"Integer"},"category":{"type":"String","value":"AI"}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
=== ekoDB Goal Template CRUD Example (TypeScript) ===

--- Creating goal template ---
Created template: Data Migration (id: 0z6N03TZl0aBV61eKixki01R-OZ66ZVwc2GSNOLjmOA5uqKN7L_q7TH2_enT-YNNSFlL2TsV8S1Zv8xiL2AWgg)

--- Listing templates ---
Templates: {
  count: 1,
  items: [
    {
      description: [Object],
      id: '0z6N03TZl0aBV61eKixki01R-OZ66ZVwc2GSNOLjmOA5uqKN7L_q7TH2_enT-YNNSFlL2TsV8S1Zv8xiL2AWgg',
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
Created goal: Deploy v2.0 (id: P4a5f0CnS_RKc4TyqfkljHHjMt5Ij2muAfrDNKRvRGH8L2nLNjcpPJxcJH9NQL2TPIZ9Dp0Twr3BVRwGRKlLcw)

--- Listing goals ---
Goals: {
  "count": 1,
  "goals": [
    {
      "created_at": "2026-09-15T18:02:27.754712+00:00",
      "description": "Ship version 2.0 to production",
      "id": "P4a5f0CnS_RKc4TyqfkljHHjMt5Ij2muAfrDNKRvRGH8L2nLNjcpPJxcJH9NQL2TPIZ9Dp0Twr3BVRwGRKlLcw",
      "status": "pending",
      "steps": "[{\"description\":\"Run test suite\"},{\"description\":\"Build release artifacts\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]",
      "title": "Deploy v2.0",
      "updated_at": "2026-09-15T18:02:27.754712+00:00"
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
        "value": "2026-09-15T18:02:27.754712+00:00"
      },
      "description": {
        "type": "String",
        "value": "Ship version 2.0 with hot-fix patches"
      },
      "id": "P4a5f0CnS_RKc4TyqfkljHHjMt5Ij2muAfrDNKRvRGH8L2nLNjcpPJxcJH9NQL2TPIZ9Dp0Twr3BVRwGRKlLcw",
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
        "value": "2026-09-15T18:02:27.793574+00:00"
      }
    }
  ]
}

--- Goal step: start step 0 ---
Step 0 started on goal P4a5f0CnS_RKc4TyqfkljHHjMt5Ij2muAfrDNKRvRGH8L2nLNjcpPJxcJH9NQL2TPIZ9Dp0Twr3BVRwGRKlLcw
--- Goal step: complete step 0 ---
Step 0 completed on goal P4a5f0CnS_RKc4TyqfkljHHjMt5Ij2muAfrDNKRvRGH8L2nLNjcpPJxcJH9NQL2TPIZ9Dp0Twr3BVRwGRKlLcw
--- Goal step: fail step 1 ---
Step 1 failed on goal P4a5f0CnS_RKc4TyqfkljHHjMt5Ij2muAfrDNKRvRGH8L2nLNjcpPJxcJH9NQL2TPIZ9Dp0Twr3BVRwGRKlLcw

--- Completing goal ---
Goal status: pending_review
--- Approving goal ---
Goal status after approve: in_progress

--- Creating goal to reject ---
--- Rejecting goal ---
Goal status after reject: failed


--- Creating task ---
Created task: Hourly Health Check (id: 2nXjDzbkhuqALahCrpOM2MUJgmMOvTfvhKky6nrCRKN4dkFnq9gB3lbryZu_8ltt8wucBX8MSmSdkIK4z_FDVg)

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
      "id": "2nXjDzbkhuqALahCrpOM2MUJgmMOvTfvhKky6nrCRKN4dkFnq9gB3lbryZu_8ltt8wucBX8MSmSdkIK4z_FDVg",
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
Created agent: SupportBot (id: cEUR6saBaw1-7czwBdmZk_TPbivSgnx6UfPRPZK-nNmbkY2hWd6pRQ6JGDtXr3EKh5DEZ3LRU0hzddCpUEgDew)

--- Listing agents ---
Agents: {
  "count": 1,
  "items": [
    {
      "deployment_id": {
        "type": "String",
        "value": "deploy_prod_1"
      },
      "id": "cEUR6saBaw1-7czwBdmZk_TPbivSgnx6UfPRPZK-nNmbkY2hWd6pRQ6JGDtXr3EKh5DEZ3LRU0hzddCpUEgDew",
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
By name: SupportBot (id: cEUR6saBaw1-7czwBdmZk_TPbivSgnx6UfPRPZK-nNmbkY2hWd6pRQ6JGDtXr3EKh5DEZ3LRU0hzddCpUEgDew)

--- Updating agent ---
Updated agent: SupportBot

--- Getting agents by deployment ---
Agents in deployment: {
  "count": 0,
  "items": []
}

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
✓ ts_users_register saved
✓ ts_users_login saved
✓ ts_users_verify_token saved

=== Auth flow defined as pure stored functions ===
Call them like:
  POST /api/functions/ts_users_register { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/ts_users_login    { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/ts_users_verify_token { "token": "<jwt>" }

Set JWT_SECRET in ekoDB's environment_vars whitelist before invoking.

✓ Cleaned up demo functions
=== ekoDB KV Document Linking Example (TypeScript) ===

--- Setup: creating KV key and documents ---
Set KV key: session:admin
Inserted document 1: -Hxy_4IWvmjoEZIZPoZ4ZA8DksKby--3yyQydrD7YkVOT_2SdN93coIbOdjqUsppuiPDgcg8o29oSiqNtbMEZQ
Inserted document 2: doHlyt5UcRbAirtrkgnXleT-HFrepHFmWOpyYMxGHN2F_0nb3V4SgL7Rof6Wuc7BslFrU9RbLqQOLQxghK31lw

--- Linking documents to KV key ---
Linked doc -Hxy_4IWvmjoEZIZPoZ4ZA8DksKby--3yyQydrD7YkVOT_2SdN93coIbOdjqUsppuiPDgcg8o29oSiqNtbMEZQ: null
Linked doc doHlyt5UcRbAirtrkgnXleT-HFrepHFmWOpyYMxGHN2F_0nb3V4SgL7Rof6Wuc7BslFrU9RbLqQOLQxghK31lw: null

--- Getting links for KV key ---
Links: [
  {
    "collection": "kv_links_example_ts",
    "document_id": "-Hxy_4IWvmjoEZIZPoZ4ZA8DksKby--3yyQydrD7YkVOT_2SdN93coIbOdjqUsppuiPDgcg8o29oSiqNtbMEZQ",
    "field_path": null,
    "created_at": "2026-09-15T18:02:30.575586Z",
    "last_accessed": "2026-09-15T18:02:30.594565Z",
    "metadata": {}
  },
  {
    "collection": "kv_links_example_ts",
    "document_id": "doHlyt5UcRbAirtrkgnXleT-HFrepHFmWOpyYMxGHN2F_0nb3V4SgL7Rof6Wuc7BslFrU9RbLqQOLQxghK31lw",
    "field_path": null,
    "created_at": "2026-09-15T18:02:30.590204Z",
    "last_accessed": "2026-09-15T18:02:30.594565Z",
    "metadata": {}
  }
]

--- Unlinking document ---
Unlinked doc doHlyt5UcRbAirtrkgnXleT-HFrepHFmWOpyYMxGHN2F_0nb3V4SgL7Rof6Wuc7BslFrU9RbLqQOLQxghK31lw: null

--- Verifying remaining links ---
Remaining links: [
  {
    "collection": "kv_links_example_ts",
    "document_id": "-Hxy_4IWvmjoEZIZPoZ4ZA8DksKby--3yyQydrD7YkVOT_2SdN93coIbOdjqUsppuiPDgcg8o29oSiqNtbMEZQ",
    "field_path": null,
    "created_at": "2026-09-15T18:02:30.575586Z",
    "last_accessed": "2026-09-15T18:02:30.605193Z",
    "metadata": {}
  }
]

--- Cleanup ---
Cleanup complete

=== All KV linking operations completed ===
✓ Client created

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: { value: { username: 'john_doe', userId: 123 }, type: 'Object' }

=== KV Batch Set ===
✓ Batch set 3 keys
  cache:product:1: success
  cache:product:2: success
  cache:product:3: success

=== KV Batch Get ===
✓ Batch retrieved 3 values
  cache:product:1: { name: 'Product 1', price: 29.99 }
  cache:product:2: { price: 39.99, name: 'Product 2' }
  cache:product:3: { price: 49.99, name: 'Product 3' }

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 3 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 9

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: false

=== KV Batch Delete ===
✓ Batch deleted 3 keys
  cache:product:1: deleted
  cache:product:2: deleted
  cache:product:3: deleted

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
✓ Cleaned up test keys

=== Summary ===
✅ Use Field.decimal() for monetary values, percentages, and
   any case where floating-point errors are unacceptable.
✅ Field.decimal() stores values as strings internally,
   preserving exact precision across all operations.
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
    "name": "Earth",
    "diameter_km": 12742
  },
  {
    "name": "Mars",
    "diameter_km": 6779
  },
  {
    "name": "Jupiter",
    "diameter_km": 139820
  }
]

--- Blocking HTTP (for comparison) ---
Blocking response: Hello! I hope you're having a great day.

=== Done ===
=== ekoDB Schedule Management Example (TypeScript) ===

--- Creating schedule ---
Created schedule: Nightly Database Backup (id: 23a55926-be7e-4fde-aea4-e3fd9ef9d7d7, cron: 0 0 2 * * *)

--- Listing schedules ---
Schedules: {
  "count": 1,
  "schedules": [
    {
      "created_at": "2026-09-15T18:02:42.477409Z",
      "cron_expression": "0 0 2 * * *",
      "description": null,
      "enabled": true,
      "function_label": "schedule_noop_typescript_24355",
      "id": "23a55926-be7e-4fde-aea4-e3fd9ef9d7d7",
      "last_execution": null,
      "name": "Nightly Database Backup",
      "next_execution": "2026-09-16T02:00:00Z",
      "parameters": {},
      "stats": {
        "avg_execution_time_ms": 0,
        "failed_executions": 0,
        "last_error": null,
        "successful_executions": 0,
        "total_executions": 0
      },
      "timezone": "UTC",
      "updated_at": "2026-09-15T18:02:42.477409Z"
    }
  ]
}

--- Getting schedule ---
Fetched: Nightly Database Backup (cron: 0 0 2 * * *)

--- Updating schedule ---
Updated: Nightly Full Backup (new cron: 0 0 3 * * *)

--- Triggering schedule ---
Trigger response: {
  "schedule_id": "23a55926-be7e-4fde-aea4-e3fd9ef9d7d7",
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
  1. Score: 12.870, Matched: email, name
  2. Score: 6.270, Matched: name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 13.200, Matched: title, bio
  2. Score: 13.200, Matched: bio, title
  3. Score: 13.200, Matched: title, bio
  4. Score: 13.200, Matched: title, bio

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 26.400, Matched: bio, title
  2. Score: 26.400, Matched: title, bio
  3. Score: 26.400, Matched: title, bio
  4. Score: 26.400, Matched: bio, title

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 6.600, Matched: bio
  2. Score: 6.600, Matched: bio

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 6.600, Matched: bio

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.774, Matched:
  2. Score: 0.772, Matched:
  3. Score: 0.754, Matched:

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.501, Matched: content, title
  2. Score: 0.910, Matched: content, title
  3. Score: 0.309, Matched:

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 13.200, Matched: bio, skills

9. Vector search with a metadata pre-filter (category = ml):
Found 2 documents in category "ml" (NLP excluded)
  1. Deep Learning Fundamentals (category: ml)
  2. Introduction to Machine Learning (category: ml)

=== Cleanup ===
✅ Deleted test collections

✅ Search examples completed!
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'PNb5NiQaihw5YEsEQvgXEp66q5EnkRNIjIBe61RuRJQOhI-CM08uNJ8BipBQMCv1N7u9eCvjIlSCDd0lz5IfyA'
}

=== Find by ID ===
Found: {
  categories: { type: 'Array', value: [ 'electronics', 'computers' ] },
  created_at: { value: '2026-09-15T18:02:44.118+00:00', type: 'DateTime' },
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] },
  value: { type: 'Integer', value: 42 },
  user_id: { value: '550e8400-e29b-41d4-a716-446655440000', type: 'String' },
  name: { type: 'String', value: 'Test Record' },
  metadata: { value: { key: 'value', nested: [Object] }, type: 'Object' },
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  price: { value: 99.99, type: 'Float' },
  active: { value: true, type: 'Boolean' },
  id: 'PNb5NiQaihw5YEsEQvgXEp66q5EnkRNIjIBe61RuRJQOhI-CM08uNJ8BipBQMCv1N7u9eCvjIlSCDd0lz5IfyA',
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] }
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-09-15T18:02:44.118Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { key: 'value', nested: { deep: true } }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  categories: [ 'electronics', 'computers' ],
  created_at: '2026-09-15T18:02:44.118+00:00',
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  value: 42,
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  name: 'Test Record',
  metadata: { key: 'value', nested: { deep: true } },
  data: 'aGVsbG8gd29ybGQ=',
  price: 99.99,
  active: true,
  id: 'PNb5NiQaihw5YEsEQvgXEp66q5EnkRNIjIBe61RuRJQOhI-CM08uNJ8BipBQMCv1N7u9eCvjIlSCDd0lz5IfyA',
  tags: [ 'tag1', 'tag2', 'tag3' ]
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  value: { value: 100, type: 'Integer' },
  categories: { value: [ 'electronics', 'computers' ], type: 'Array' },
  price: { value: 99.99, type: 'Float' },
  id: 'PNb5NiQaihw5YEsEQvgXEp66q5EnkRNIjIBe61RuRJQOhI-CM08uNJ8BipBQMCv1N7u9eCvjIlSCDd0lz5IfyA',
  tags: { value: [ 'tag1', 'tag2', 'tag3' ], type: 'Array' },
  created_at: { value: '2026-09-15T18:02:44.118+00:00', type: 'DateTime' },
  name: { type: 'String', value: 'Updated Record' },
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] },
  active: { value: true, type: 'Boolean' },
  metadata: { type: 'Object', value: { key: 'value', nested: [Object] } },
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: MaN-CtgMKxFWHZBpOJMCd9LxkfYIR-YVoLHya-ovllpr4aPKYTJy88i2o4rL6aKlZsO_kgILkiQcN8RGTlOLhA

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
✓ Created native SWR script: github_user_native_ts (LslJm_UHmfQOtd0JrTFARkPC43uX6x0DMcHlxewviYcv2bwAGBPtslCQ1jJyOTU5rLarJqed9PLDOaaHGi-eQw)

First call (cache miss - will fetch from GitHub API):
  Response time: 164ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 9ms
  Speedup: 18.2x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit_ts (TDNOxApZx_oKRuTckyVGOjz1cz-zApEED7VSoaObHzK4F7ZlEkjBz7cPqJWhn-T1B7GzqLvWGzF867kxma1v9g)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail_ts' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline_ts (Ws6-Ub0UvvsWH_I6iVTBFc-7hEu5NxPkXWRzb8j1t99zwDvgIE5dc_Jc3LcryDONbHDF3MC3payQGztZBJcwmg)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users_swr_ts' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache_ts (HLQUphEszTPSgZceyyqEFQLTdoR35BudMELYJOqKtPCvJmWSCuO8iMO-OP0C0mSmCepMEHyk61UfqWx4JSgu5g)
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
Function 'fetch_api_user' already existed — updated instead
✓ Created SWR script: fetch_api_user (YpdpS_Zhn0VlrPNEiJiImqRS1dZGpJK573hvdEVDVwpHHnxDd0VDq4wKUZm3fy6_xdxP_5VCzkfLiKyBgDCATw)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "website": "hildegard.org",
          "address": {
            "street": "Kulas Light",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "suite": "Apt. 556",
            "zipcode": "92998-3874",
            "city": "Gwenborough"
          },
          "name": "Leanne Graham",
          "company": {
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net"
          },
          "email": "Sincere@april.biz",
          "username": "Bret",
          "id": 1,
          "phone": "1-770-736-8031 x56442"
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
Response time: 11ms (served from cache)
Result (cached): {
  "records": [
    {
      "value": {
        "value": {
          "website": "hildegard.org",
          "address": {
            "street": "Kulas Light",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "suite": "Apt. 556",
            "zipcode": "92998-3874",
            "city": "Gwenborough"
          },
          "name": "Leanne Graham",
          "company": {
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net"
          },
          "email": "Sincere@april.biz",
          "username": "Bret",
          "id": 1,
          "phone": "1-770-736-8031 x56442"
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
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
Function 'fetch_product_with_reviews' already existed — updated instead
✓ Created enrichment script: fetch_product_with_reviews (nTIKJdn9imY6WSIgQJDxYhjBSivzUkUw-X262i-BTncmU_MuenQg9QpB_aGTjo36ThoHhSz3ttSCjvbhOge_FQ)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "weight": 4,
          "discountPercentage": 10.48,
          "dimensions": {
            "height": 13.08,
            "width": 15.14,
            "depth": 22.99
          },
          "meta": {
            "updatedAt": "2026-05-23T11:27:41.868Z",
            "barcode": "5784719087687",
            "qrCode": "https://cdn.dummyjson.com/public/qr-code.png",
            "createdAt": "2025-10-09T14:47:01.588Z"
          },
          "minimumOrderQuantity": 48,
          "shippingInformation": "Ships in 3-5 business days",
          "stock": 99,
          "id": 1,
          "brand": "Essence",
          "tags": [
            "beauty",
            "mascara"
          ],
          "title": "Essence Mascara Lash Princess",
          "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
          "rating": 2.56,
          "availabilityStatus": "In Stock",
          "images": [
            "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
          ],
          "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
          "price": 9.99,
          "category": "beauty",
          "reviews": [
            {
              "comment": "Would not recommend!",
              "date": "2025-04-30T09:41:02.053Z",
              "reviewerName": "Eleanor Collins",
              "reviewerEmail": "eleanor.collins@x.dummyjson.com",
              "rating": 3
            },
            {
              "reviewerEmail": "lucas.gordon@x.dummyjson.com",
              "date": "2025-04-30T09:41:02.053Z",
              "comment": "Very satisfied!",
              "reviewerName": "Lucas Gordon",
              "rating": 4
            },
            {
              "reviewerEmail": "eleanor.collins@x.dummyjson.com",
              "comment": "Highly impressed!",
              "rating": 5,
              "reviewerName": "Eleanor Collins",
              "date": "2025-04-30T09:41:02.053Z"
            }
          ],
          "sku": "BEA-ESS-ESS-001",
          "returnPolicy": "No return policy",
          "warrantyInformation": "1 week warranty"
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
Created Alice: $1000 - ID: XoF3jmpJn3IIqK1TcDEFtFWQNd6UWSeCZ3OxwSpz8lohn5IL-E0e2NEQiHa5SPrbafI8wi2NsRuHxO15nE9pxQ
Created Bob: $500 - ID: jvfuWwW-ZzapctLHXEombm_Z_F7AAADGWqsU4Ms29mnBNxLQE4h-ys6g28RbqOD0DUhKVroku6GL1xxMwPHoow

=== Example 1: Begin Transaction ===
Transaction ID (server-default isolation): a8ddf2b4-16d6-4ba3-805f-ed51b423b496

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: ba3b29a2-5ec4-4e44-bff6-097b7764ca4b
Status before rollback: Active
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
✓ Client created

=== Create User Function ===
Created user function with ID: gb4JooUhfekWqzW4h92Is8WqTM3Kz5Pk12CUE19cRczyZkHrLmg3XEiYtBE60LnTU6CFMOdpArxye3m_LqksIA

=== Get User Function ===
Retrieved: get_active_users_ts - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 13 user functions:
  - fetch_slim_user: Validate and slim down user
  - cache_api_call_js: Cache External API Call
  - get_verified_user: Get verified and validated user
  - cache_api_call: Cache External API Call
  - cache_api_call_py: Cache External API Call
  - fetch_api_user: Fetch User with Cache
  - fetch_user: Fetch user by code
  - get_user_wrapper: Wrapper that calls fetch_user
  - fetch_product_with_reviews: Fetch Product with Reviews (Multi-API)
  - swr_user: SWR pattern for user data (KV-based)
  - fetch_and_store_user: Fetch user from API and cache in KV
  - get_active_users_ts: Get Active Users
  - validate_user: Check if user exists

=== Update User Function ===
User function updated successfully

=== Delete User Function ===
User function deleted successfully

✓ User Functions API example complete
=== WebSocket Chat Streaming Example (TypeScript) ===

Created chat session: rglSiBmBMU_HxuLekpGiQo_xtbNA5ZVNpgx6wSVeVgdVb25tlvZWsXfVvP6NpUKnhG5Wrce1-lKcjAKKzza7bg

Sending message: 'What is the capital of France?'
{"__progress":"received"}{"__progress":"generating","model":"default","provider":"openai"}The capital of France is Paris.

--- Stream ended ---
Message ID: 0PIvbkMF-1R1wy6G0RhYD76xF8KySpLKQAONIaTaFz0B5thQPRYsZ6KD0Cgi2KwgaMtPIH4qABJBLVcAp3kO3Q
Execution time: 2337ms
Token usage: {"completion_tokens":8,"prompt_tokens":15,"total_tokens":23}

Full response: {"__progress":"received"}{"__progress":"generating","model":"default","provider":"openai"}The capital of France is Paris....
=== WebSocket Subscription Example ===

✓ Authentication successful
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_ts' ===
✓ Subscribed (subscription_id: sub_5b24034195d94f40a653ce3d0f541012)

=== Listening for mutation notifications ===

Inserting a record...

  📡 Notification received:
     Event:      insert
     Collection: ws_subscribe_example_ts
     Record IDs: 4J6iUOIBJ7i9BTR0V0Rd_BNqxkXk_E3g5Yx_sLhLFsAZPW9vuhz_YEK7lRm3msey3FwPWjb8lrYH6_3yrL2McQ
     Timestamp:  2026-09-15T18:02:50.167327+00:00
     Records:    {"active":{"type":"Boolean","value":true},"id":"4J6iUOIBJ7i9BTR0V0Rd_BNqxkXk_E3g5Yx_sLhLFsAZPW9vuhz_...
✓ Inserted record: 4J6iUOIBJ7i9BTR0V0Rd_BNqxkXk_E3g5Yx_sLhLFsAZPW9vuhz_YEK7lRm3msey3FwPWjb8lrYH6_3yrL2McQ

Inserting another record...

  📡 Notification received:
     Event:      insert
     Collection: ws_subscribe_example_ts
     Record IDs: 7AEK2CX9m2Tix_br0R97krh3UgB0gGPMIQSPGxdEkqTysuyZ2T8AQkjtxbpCxTbuQHQQ-bjTApBxmXTGwi1aOA
     Timestamp:  2026-09-15T18:02:51.182832+00:00
     Records:    {"active":{"type":"Boolean","value":true},"id":"7AEK2CX9m2Tix_br0R97krh3UgB0gGPMIQSPGxdEkqTysuyZ2T8A...

=== Unsubscribing ===
✓ Unsubscribed: {"collection":"ws_subscribe_example_ts","found":true,"unsubscribed":true}

✓ WebSocket subscription example completed successfully
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: 9AXlTOQRmHl75WM0EUZbMmk8i6SDBp7ZJGtgISRjnP6Kh4GcAyh0EiaJtIHcD-z-D4k2L1kFksZjNUg_lZlh7g

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
   Inserted with ripple: {"id":"W0-pSo3f2l8efNozPwQtsrvWpHYvrnWt5lCWXsEP-clHb5dDVMVk6BDJZgCMXifs3yIFOwxanLSnWXVFl7bkLw"}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {"id":"pMljFpQuuXqXhBdzcavkFJntVtrNuFOIxnx2jVEpDB_bHgHdHHecoNXWWrSId3zQg6s0lk71-2XyyFRLqgTd_Q"}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {"id":"W0-pSo3f2l8efNozPwQtsrvWpHYvrnWt5lCWXsEP-clHb5dDVMVk6BDJZgCMXifs3yIFOwxanLSnWXVFl7bkLw","price":{"value":150,"type":"Integer"},"name":{"type":"String","value":"Product 1"}}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {"name":{"type":"String","value":"Upsert Product"},"id":"custom-id","price":{"type":"Integer","value":500}}

✅ All bypass_ripple operations completed successfully!
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields (id, name, email only)
  Found 3 active users
  Fields returned: ["name","id","email"]
  First user: Bob Smith <bob@example.com>

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ["age","created_at","user_role","status","bio","avatar_url","email","name","id"]

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
    - Fields: ["id","avatar_url","secret_token","age","name","user_role","status","api_key","email","bio","password","created_at"]
  Projected query:
    - 3 fields per record
    - Fields: ["name","id","email"]
  Bandwidth savings: ~75% fewer fields

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
