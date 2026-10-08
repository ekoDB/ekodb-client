make test-examples-javascript
🧪 Running JavaScript examples (direct HTTP/WebSocket)...

added 1 package, removed 1 package, and audited 9 packages in 378ms

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
  id: 'QERp5AlG6u9CuuhIsktFgPhhhKHdoBWwPypqPqdOfLPFVndPWLNEI2cJjS-LsSv6ywAZLm6cBeycAuElQZVPsw'
}

=== Find by ID ===
Found: {
  value: { value: 42, type: 'Integer' },
  id: 'QERp5AlG6u9CuuhIsktFgPhhhKHdoBWwPypqPqdOfLPFVndPWLNEI2cJjS-LsSv6ywAZLm6cBeycAuElQZVPsw',
  active: { type: 'Boolean', value: true },
  name: { type: 'String', value: 'Test Record' }
}

=== Find with Query ===
Found documents: [
  {
    active: { type: 'Boolean', value: true },
    name: { type: 'String', value: 'Test Record' },
    value: { value: 42, type: 'Integer' },
    id: 'QERp5AlG6u9CuuhIsktFgPhhhKHdoBWwPypqPqdOfLPFVndPWLNEI2cJjS-LsSv6ywAZLm6cBeycAuElQZVPsw'
  }
]

=== Update Document ===
Updated: {
  name: { type: 'String', value: 'Updated Record' },
  id: 'QERp5AlG6u9CuuhIsktFgPhhhKHdoBWwPypqPqdOfLPFVndPWLNEI2cJjS-LsSv6ywAZLm6cBeycAuElQZVPsw',
  value: { value: 100, type: 'Integer' },
  active: { type: 'Boolean', value: true }
}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
✓ simple_crud.js completed successfully

=== Running ekoDB/ekodb-client/examples/javascript/simple_websocket.js ===
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: u_c_S5-uQmyrr76QIMoc4DEI1tBV4sjXXUw_LERNJTrb6gege-7k2T_lVb0rYXZcqgi3cUeu5voECrLrwoX0qw

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "type": "Success",
  "payload": {
    "data": [
      {
        "active": {
          "value": true,
          "type": "Boolean"
        },
        "value": {
          "type": "Integer",
          "value": 42
        },
        "name": {
          "value": "WebSocket Test Record",
          "type": "String"
        },
        "id": "u_c_S5-uQmyrr76QIMoc4DEI1tBV4sjXXUw_LERNJTrb6gege-7k2T_lVb0rYXZcqgi3cUeu5voECrLrwoX0qw"
      }
    ]
  },
  "messageId": "1791438543805"
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

✅ Function saved: rj_UgGBfIiYiYdQflnyMweyvuM65qqwUsQgth9rdJk-CD3Cee9i7GnPcBE4UieVi1IYgmY5PKhcq7NoU20Nx-Q
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Function saved: enJlxb-RrP6-YzHdbC6S9rDf6Qyg-tX2kHLbtUiXptyl3o5uN8Q-d2Hwq999ETEnymYELaMefU-4iXJRq1GMqA
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: Qe3JzxvKOaO15coozrxrFXONzyfJUycO9TatGkRN3KcKIRHuCjxI8xVNBOIhUU1kzQISfcz2f9Jz5F82630rVQ
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"avg_score":{"type":"Float","value":50},"max_score":{"value":90,"type":"Integer"},"status":{"value":"inactive","type":"String"},"count":{"type":"Integer","value":5}}
   {"count":{"value":5,"type":"Integer"},"status":{"value":"active","type":"String"},"avg_score":{"value":60,"type":"Float"},"max_score":{"value":100,"type":"Integer"}}

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
Retrieved value: { type: 'Object', value: { username: 'john_doe', userId: 123 } }

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: { value: { price: 29.99, name: 'Product 1' }, type: 'Object' }
cache:product:2: {
  value: { name: 'Product 2', price: 39.989999999999995 },
  type: 'Object'
}
cache:product:3: {
  value: { name: 'Product 3', price: 49.989999999999995 },
  type: 'Object'
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
Collection created with first record: njVQi5O4co6BaTsvRXBc-IXzqniuRd4BWSK9oqHWtQncFfoIQOqGZR4LHqkysi3mBQ-EZyWHZDCGxgqf_0OhCA

=== List Collections ===
Total collections: 22
Sample collections: [
  'demo_collection',
  'chat_agent_configs__ek0_testing',
  'chat_goal_templates__ek0_testing',
  'schema_documents_client_go',
  'audit__ek0_testing'
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
Created Alice: $1000 - ID: u6butrdzZ6q7trQ4Y6kOOYNQFfHt5VIdZCSNQ0Lm0DnrGofWJUlF6lqUe-PwECtWnoU1kNeAvkWcX_cUiTYfZQ
Created Bob: $500 - ID: gvAOKQapJut8H10olHavnzoJPgKl3H8ssMQy64RqdDsC5UWonoyjb25ZZnyGnydBPGIwdRyrG76PWi8Jr0ZvFQ

=== Example 1: Begin Transaction ===
Transaction ID: fbcd82b7-a768-4fd7-af99-3f5ddfadb4e5

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
Bob: {"type":"Integer","value":700}

=== Example 5: Rollback ===
New transaction: 4b4debf1-973a-44c4-a04d-adea5fa4aa15
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: {"value":700,"type":"Integer"}

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
   ✅ Function saved: jxqteduOlx_75iRXfbD03iaIQB18_A3LX0udmh_dl6mOHgYooncgxWinhJkmSLotBwlKkdRU5Qxq6GFqnu0dag

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
   ✅ Function saved: 8Jl8gR0U7v6suR7rOkTgma5Wp_b7DDf8SCMMhUKHcRgPXotg0Ise0wA26-2WdT0jIZNKhDzo2Em3Kt9JWuQkdg

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
   ✅ Function saved: grsJVHUlT6iKrAFpEZDFfxHwwvW6ZjqDSZSEdwMHO-hbupedNwQChSCpCPb8v_7GgkfN41PVXfQjhZOJf9nzBw

2️⃣ Calling Function (Query + Update Credits + Verify)...
   ✅ Function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {"type":"Integer","value":100}
   📋 Status: {"type":"String","value":"active"}
   📋 Name: {"type":"String","value":"Alice Smith"}

============================================================
📝 Function 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: HErQ0NMPIgUvSwf61MAWRVYnNQQlc4pW0fMlnHCW9wpzS1jeeLLd8CEIseN00zp5fl10UTsmzanC2PnKt9KybQ

2️⃣ Calling Function (Query + Delete + Verify)...
   ✅ Function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted function: jxqteduOlx_75iRXfbD0...
   ✅ Deleted function: 8Jl8gR0U7v6suR7rOkTg...
   ✅ Deleted function: grsJVHUlT6iKrAFpEZDF...
   ✅ Deleted function: HErQ0NMPIgUvSwf61MAW...
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
  Output: Document ID = nAhefpGch-k-y46NeHbiR7bpHmUftq7JbCCna0Wjv9C1lQziL3OFCGefIcVXckL45CoTKe8tsk4LltPRhAffmw
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: findById(nAhefpGch-k-y46NeHbiR7bpHmUftq7JbCCna0Wjv9C1lQziL3OFCGefIcVXckL45CoTKe8tsk4LltPRhAffmw)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: findById(nAhefpGch-k-y46NeHbiR7bpHmUftq7JbCCna0Wjv9C1lQziL3OFCGefIcVXckL45CoTKe8tsk4LltPRhAffmw)
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
  Output: Document ID = rd9Vo3i-pIhHLFlY5m7VOSfkdwbBtJejVqAuGzQO4TMC3J4K0f8sLB9QZil09o1p9QuDObF7fAIP_Em28XPgKg
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: findById(rd9Vo3i-pIhHLFlY5m7VOSfkdwbBtJejVqAuGzQO4TMC3J4K0f8sLB9QZil09o1p9QuDObF7fAIP_Em28XPgKg)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: findById(rd9Vo3i-pIhHLFlY5m7VOSfkdwbBtJejVqAuGzQO4TMC3J4K0f8sLB9QZil09o1p9QuDObF7fAIP_Em28XPgKg)
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

> @ekodb/ekodb-client@0.27.0 prepare
> npm run build


> @ekodb/ekodb-client@0.27.0 build
> tsc


up to date, audited 44 packages in 564ms

12 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.27.0 build
> tsc

✅ TypeScript client built!

added 1 package, removed 1 package, and audited 9 packages in 360ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities

added 1 package, removed 1 package, and audited 13 packages in 377ms

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
✓ Created session: vh79aGx_pEbOhp-nnIqW-5IHWep0BtY6bVtG-sNzrrXwcSQSVaErEYa-MqrK2pkLoSqp8RBj0LOmFFZDbe-pZA

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** High-performance database product
- **Price:** $99

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
✓ Created second session: sGxbCWrELqpt8SV1wmzl5ama5T_wHJs81Cf6hKpy8qRVncVNGNE5AEwmmQ4ngfX16QxG1QWzLnbIjxEFOEnX5A
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
✓ Created session: dfe1xl2yCaZR5-09gE5adchEofIHL6MD-loDbtOnmX7wwsyl03_hS8ZZ-mCzRFNs5IXZ3z6S8-amPeXWmuvtEQ

=== Sending Chat Message ===
Message ID: ffzcBnvkFIbPH7gCva3uLxpCgnTMrkaRJ_1OlQIHg1UNBXEZHviq1DzHUkGnZcZG14odX-tjf5knMr_8y3DSkA

=== AI Response ===
The available products and their prices are as follows:

1. **ekoDB**
   - **Price:** $99
   - **Description:** A high-performance database product with AI capabilities.

2. **ekoDB Pro**
   - **Price:** $299
   - **Description:** Enterprise edition product with advanced features.

3. **ekoDB Cloud**
   - **Price:** $499
   - **Description:** Fully managed cloud database service product.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_js',
  record: {
    description: 'A high-performance database product with AI capabilities',
    id: '4b9C8axznDvbM6hMtbMG9TLtcPK2Rg1jZjSt2Ng6ub3FonpqxXdEOF53mNPcmMhQ40tyGNaUpbElaF9DShV3Zg',
    name: 'ekoDB',
    price: 99
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_js',
  record: {
    id: 'OiEwtEgQhluRfyTAtSFYVUwr4xYJRSAmm1eUl1w-eOjFtwlsjzlWH1K9Q9jM6XH6pef98VPYw4SwC7a8N0HXBA',
    price: 299,
    description: 'Enterprise edition product with advanced features',
    name: 'ekoDB Pro'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_js',
  record: {
    price: 499,
    id: 'OlBtKm9sUU9bCNn5AGFxX0HnP1yyI_lL-feQp8wsNUJvHygFkhyjjGQkQOAIHgTYbYo0DbfDAnn3sNvLksYy4Q',
    name: 'ekoDB Cloud',
    description: 'Fully managed cloud database service product'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 3598ms

=== Token Usage ===
Prompt tokens: 3413
Completion tokens: 97
Total tokens: 3510

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ Chat completed successfully
✓ Client created

=== List Chat Models ===
Available chat models by provider:
  openai:
    - o3-2025-04-16
    - gpt-5.1-chat-latest
    - gpt-5.3-chat-latest
    - gpt-5.2-2025-12-11
    - sora-2
    - chatgpt-image-latest
    - gpt-4.1-mini
    - gpt-3.5-turbo
    - gpt-4o-mini
    - gpt-image-2.5-sunburst
    - gpt-image-2.5-flare-2026-09-08
    - gpt-6-luna
    - gpt-5.4-mini-2026-03-17
    - gpt-4-turbo
    - gpt-4.1-nano
    - gpt-image-2
    - gpt-5.4-pro
    - gpt-realtime-mini-2025-12-15
    - gpt-3.5-turbo-16k
    - babbage-002
    - text-embedding-ada-002
    - gpt-4o-mini-tts
    - gpt-4o-2024-08-06
    - o4-mini-2025-04-16
    - gpt-4o-2024-11-20
    - omni-moderation-latest
    - gpt-realtime
    - tts-1-hd
    - gpt-realtime-1.5
    - gpt-realtime-2
    - gpt-5-search-api
    - gpt-5.5-2026-04-23
    - gpt-5.1
    - gpt-5.1-codex
    - gpt-4o
    - o1-2024-12-17
    - gpt-5.2-chat-latest
    - gpt-4o-mini-tts-2025-12-15
    - gpt-image-1
    - davinci-002
    - gpt-image-2-2026-04-21
    - gpt-4o-mini-transcribe
    - o3-mini
    - gpt-audio
    - gpt-5.4-nano
    - o3
    - gpt-5.1-codex-max
    - gpt-realtime-2.1
    - gpt-4.1
    - gpt-5.5
    - gpt-4o-mini-search-preview-2025-03-11
    - text-embedding-3-large
    - gpt-4o-mini-search-preview
    - gpt-4o-2024-05-13
    - gpt-6-sol
    - gpt-5.3-codex
    - gpt-3.5-turbo-instruct
    - sora-2-pro
    - gpt-5
    - gpt-audio-mini-2025-12-15
    - gpt-transcribe
    - o4-mini-deep-research-2025-06-26
    - o1-pro-2025-03-19
    - gpt-4.1-nano-2025-04-14
    - gpt-5-search-api-2025-10-14
    - gpt-4-turbo-2024-04-09
    - gpt-realtime-2.1-mini
    - tts-1
    - gpt-5-mini
    - omni-moderation-2024-09-26
    - gpt-5-mini-2025-08-07
    - gpt-realtime-2025-08-28
    - gpt-5.4-2026-03-05
    - gpt-live-transcribe
    - gpt-3.5-turbo-0125
    - gpt-5-nano
    - gpt-5.6-luna
    - gpt-4
    - gpt-image-2.5-sunburst-2026-09-08
    - whisper-1
    - gpt-5.4-mini
    - gpt-5-pro
    - o1-pro
    - gpt-realtime-whisper
    - gpt-5.2-pro
    - gpt-5-chat-latest
    - gpt-live-1
    - gpt-4o-mini-transcribe-2025-12-15
    - gpt-5-2025-08-07
    - tts-1-1106
    - gpt-5.5-pro
    - gpt-5-pro-2025-10-06
    - chat-latest
    - gpt-5.2-pro-2025-12-11
    - gpt-4o-mini-transcribe-2025-03-20
    - gpt-audio-2025-08-28
    - gpt-5.4-pro-2026-03-05
    - gpt-realtime-translate
    - gpt-5.6-sol
    - gpt-audio-mini-2025-10-06
    - gpt-5.5-pro-2026-04-23
    - tts-1-hd-1106
    - gpt-realtime-mini
    - gpt-5.4
    - gpt-4o-transcribe-diarize
    - gpt-5-codex
    - gpt-image-1-mini
    - gpt-image-1.5
    - gpt-5.1-2025-11-13
    - o4-mini-deep-research
    - gpt-4o-search-preview-2025-03-11
    - o3-mini-2025-01-31
    - gpt-4-0613
    - gpt-4o-search-preview
    - gpt-5.6-terra
    - text-embedding-3-small
    - gpt-audio-1.5
    - gpt-3.5-turbo-instruct-0914
    - gpt-6-astra
    - gpt-5.2-codex
    - gpt-3.5-turbo-1106
    - gpt-5-nano-2025-08-07
    - gpt-audio-mini
    - gpt-4o-mini-tts-2025-03-20
    - gpt-5.2
    - gpt-4.1-mini-2025-04-14
    - gpt-4.1-2025-04-14
    - gpt-4o-mini-2024-07-18
    - o4-mini
    - o1
    - gpt-5.4-nano-2026-03-17
    - gpt-5.1-codex-mini
    - gpt-4o-transcribe
    - gpt-image-2.5-flare
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
Provider status:
  anthropic: ok 14 models
  gemini: not_configured (unverified) No Gemini API Key
  openai: ok 135 models
  perplexity: not_configured (unverified) No Perplexity API Key

=== Get Specific Provider Models ===
OpenAI models: o3-2025-04-16, gpt-5.1-chat-latest, gpt-5.3-chat-latest, gpt-5.2-2025-12-11, sora-2, chatgpt-image-latest, gpt-4.1-mini, gpt-3.5-turbo, gpt-4o-mini, gpt-image-2.5-sunburst, gpt-image-2.5-flare-2026-09-08, gpt-6-luna, gpt-5.4-mini-2026-03-17, gpt-4-turbo, gpt-4.1-nano, gpt-image-2, gpt-5.4-pro, gpt-realtime-mini-2025-12-15, gpt-3.5-turbo-16k, babbage-002, text-embedding-ada-002, gpt-4o-mini-tts, gpt-4o-2024-08-06, o4-mini-2025-04-16, gpt-4o-2024-11-20, omni-moderation-latest, gpt-realtime, tts-1-hd, gpt-realtime-1.5, gpt-realtime-2, gpt-5-search-api, gpt-5.5-2026-04-23, gpt-5.1, gpt-5.1-codex, gpt-4o, o1-2024-12-17, gpt-5.2-chat-latest, gpt-4o-mini-tts-2025-12-15, gpt-image-1, davinci-002, gpt-image-2-2026-04-21, gpt-4o-mini-transcribe, o3-mini, gpt-audio, gpt-5.4-nano, o3, gpt-5.1-codex-max, gpt-realtime-2.1, gpt-4.1, gpt-5.5, gpt-4o-mini-search-preview-2025-03-11, text-embedding-3-large, gpt-4o-mini-search-preview, gpt-4o-2024-05-13, gpt-6-sol, gpt-5.3-codex, gpt-3.5-turbo-instruct, sora-2-pro, gpt-5, gpt-audio-mini-2025-12-15, gpt-transcribe, o4-mini-deep-research-2025-06-26, o1-pro-2025-03-19, gpt-4.1-nano-2025-04-14, gpt-5-search-api-2025-10-14, gpt-4-turbo-2024-04-09, gpt-realtime-2.1-mini, tts-1, gpt-5-mini, omni-moderation-2024-09-26, gpt-5-mini-2025-08-07, gpt-realtime-2025-08-28, gpt-5.4-2026-03-05, gpt-live-transcribe, gpt-3.5-turbo-0125, gpt-5-nano, gpt-5.6-luna, gpt-4, gpt-image-2.5-sunburst-2026-09-08, whisper-1, gpt-5.4-mini, gpt-5-pro, o1-pro, gpt-realtime-whisper, gpt-5.2-pro, gpt-5-chat-latest, gpt-live-1, gpt-4o-mini-transcribe-2025-12-15, gpt-5-2025-08-07, tts-1-1106, gpt-5.5-pro, gpt-5-pro-2025-10-06, chat-latest, gpt-5.2-pro-2025-12-11, gpt-4o-mini-transcribe-2025-03-20, gpt-audio-2025-08-28, gpt-5.4-pro-2026-03-05, gpt-realtime-translate, gpt-5.6-sol, gpt-audio-mini-2025-10-06, gpt-5.5-pro-2026-04-23, tts-1-hd-1106, gpt-realtime-mini, gpt-5.4, gpt-4o-transcribe-diarize, gpt-5-codex, gpt-image-1-mini, gpt-image-1.5, gpt-5.1-2025-11-13, o4-mini-deep-research, gpt-4o-search-preview-2025-03-11, o3-mini-2025-01-31, gpt-4-0613, gpt-4o-search-preview, gpt-5.6-terra, text-embedding-3-small, gpt-audio-1.5, gpt-3.5-turbo-instruct-0914, gpt-6-astra, gpt-5.2-codex, gpt-3.5-turbo-1106, gpt-5-nano-2025-08-07, gpt-audio-mini, gpt-4o-mini-tts-2025-03-20, gpt-5.2, gpt-4.1-mini-2025-04-14, gpt-4.1-2025-04-14, gpt-4o-mini-2024-07-18, o4-mini, o1, gpt-5.4-nano-2026-03-17, gpt-5.1-codex-mini, gpt-4o-transcribe, gpt-image-2.5-flare, gpt-6.1-sol

=== Get Anthropic Models ===
Anthropic models: claude-haiku-5-5, claude-sonnet-5-5, claude-opus-5-5, claude-fable-5-1, claude-opus-5, claude-sonnet-5, claude-fable-5, claude-opus-4-8, claude-opus-4-7, claude-sonnet-4-6, claude-opus-4-6, claude-opus-4-5-20251101, claude-haiku-4-5-20251001, claude-sonnet-4-5-20250929

=== Get Non-Existent Provider ===
Expected error for non-existent provider: Error: Request failed with status 404: {"error":"Unknown provider 'nonexistent_provider_xyz'","error_kind":"unknown_provider"}

✓ Chat Models example complete
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: Q9V8MzC9PXNNJDyURerFSrtbG68qUYa4z-W-XGPa_zEH91wM6311xYXP3l4h6pSqDmiPXNjCjQ7Ma_nGXWhgeg

=== Sending Messages ===
✓ Message 1 sent
  Response: Currently, the available product is:

- **Name**: ekoDB
- **Description**: A high-performance database product
- **Price**: $99

If you need more information or have further questions, feel free to ask!

✓ Message 2 sent
  Response: The price of the product ekoDB is **$99**. If you have any more questions or need further information, feel free to ask!

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 6lSripj6uVBfBH1_rFqr5AxTuOIXHDPfUo6OG5tuzWRSvP0LgoceMLiQX-rU_ZvSuKuIi_-1nc_eHcBvB0a20w
  Parent: Q9V8MzC9PXNNJDyURerFSrtbG68qUYa4z-W-XGPa_zEH91wM6311xYXP3l4h6pSqDmiPXNjCjQ7Ma_nGXWhgeg

=== Listing Sessions ===
✓ Found 2 sessions
  Session 1: 6lSripj6uVBfBH1_rFqr5AxTuOIXHDPfUo6OG5tuzWRSvP0LgoceMLiQX-rU_ZvSuKuIi_-1nc_eHcBvB0a20w (Untitled)
  Session 2: Q9V8MzC9PXNNJDyURerFSrtbG68qUYa4z-W-XGPa_zEH91wM6311xYXP3l4h6pSqDmiPXNjCjQ7Ma_nGXWhgeg (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: 6lSripj6uVBfBH1_rFqr5AxTuOIXHDPfUo6OG5tuzWRSvP0LgoceMLiQX-rU_ZvSuKuIi_-1nc_eHcBvB0a20w

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ All session management operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: 1bMqYxs-kdQ-RmtaNJ7HPjO79SmsGDt4V2fymXcD93gWmiF0LVpBhqikqJe-sYNi2qRKWhGa9L7_ko5FZW5GsA

=== List Collections ===
Total collections: 23
Sample collections: chat_agent_configs__ek0_testing,chat_goal_templates__ek0_testing,client_collection_management_js,schema_documents_client_go,test_accounts

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: yoNfHBB_WGHBwo3IVxgSNy01HUsP-NHmRYLF5lAt-0IBZNfluCgWX3LQOmMqQNNLr3Q2DdEQ8z19IQgfclpT6w

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: 8hnZLY8EB9kAb1GVhKrWOOmjLpmYpzaLeSfSa753UYYKr9BFv1UrEQMw_0hRg-XYfy-G8Vy3HiWlC50tCVTBbA

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
✓ Edge cache script created: _YPlpu30v9blaDJTD6gkzVAlBSJ5kivz_Ahg1yWWpcDlIApSqMQbwXHRWU9fM_ho4jeGovt1dPEheRpP2esYHQ

Call 1: Cache miss (fetches from API)
Response time: 164ms
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "timezone_abbreviation": "GMT",
          "elevation": 32,
          "longitude": -73.99308,
          "utc_offset_seconds": 0,
          "generationtime_ms": 0.042557716369628906,
          "current": {
            "interval": 900,
            "time": "2026-10-08T05:45",
            "temperature_2m": 13.6
          },
          "current_units": {
            "time": "iso8601",
            "temperature_2m": "°C",
            "interval": "seconds"
          },
          "latitude": 40.710335,
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

Call 2: Cache hit (served from ekoDB)
Response time: 6ms (27.3x faster!)
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "timezone_abbreviation": "GMT",
          "elevation": 32,
          "longitude": -73.99308,
          "utc_offset_seconds": 0,
          "generationtime_ms": 0.042557716369628906,
          "current": {
            "interval": 900,
            "time": "2026-10-08T05:45",
            "temperature_2m": 13.6
          },
          "current_units": {
            "time": "iso8601",
            "temperature_2m": "°C",
            "interval": "seconds"
          },
          "latitude": 40.710335,
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
   Name: {"type":"String","value":"User 1"}
   Department: {"type":"String","value":"engineering"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 60ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "name": "Leanne Graham",
      "phone": "1-770-736-8031 x56442",
      "id": 1,
      "email": "Sincere@april.biz",
      "company": {
       ...

Second call (cache hit - from cache):
   ⏱️  Duration: 2ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "name": "Leanne Graham",
      "phone": "1-770-736-8031 x56442",
      "id": 1,
      "email": "Sincere@april.biz",
      "company": {
        "bs": "harness real-t...
   🚀 Cache speedup: 30.0x faster!

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

✅ Function saved: d3gGPvxm7SOSx3ESJfm1sgGsa39127I7hOWZhsQgfBeC8NdGO4Q9KEH53wBjBalO2-xspBW3vTW5nvv_dqXhBw
📊 Found 5 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Function

✅ Function saved
📊 Found 3 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Function

✅ Function saved
📊 Statistics: 2 groups
   {"count":{"type":"Integer","value":5},"avg_score":{"type":"Float","value":60},"status":{"type":"String","value":"active"}}
   {"count":{"type":"Integer","value":5},"avg_score":{"value":50,"type":"Float"},"status":{"type":"String","value":"inactive"}}
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
   Furniture:
      Products: 4 | Stock: 43 | Avg Rating: ⭐4.26
   Electronics:
      Products: 6 | Stock: 232 | Avg Rating: ⭐4.52

⏱️  Total execution time: 0ms
📈 Pipeline stages:

📝 Example 5: Project Specific Fields

✅ Function saved
📊 Product summaries (10 items, showing first 3):
   1. Keyboard - $89 (⭐4.4)
   2. Webcam HD - $119 (⭐4.5)
   3. USB-C Cable - $19 (⭐4.3)
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
   1. Scalability: Vector databases can handle large amounts of data efficiently.
2. High Accuracy: Vector data model represents data with high degree of accuracy.
3. Flexibility: Vector models are extremely flexible allowing for efficient data manipulation.
4. Rich Geometric and Topological Operations: Complex operations like polygon overlay, network analysis etc. can be done effectively.
5. Compact Data Structure: Vector data is usually more compact in storage size.
6. Detailed Representation: Vector databases can represent complex geographical structures.
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

✅ Function saved: iIPT33MeGU61sphfsMun6GXtkbDDWXQH_TdD9f2xjMJJM9I-1pTRfHnVNaeeGR0itJSsyj5myZkC_mKcxOeioQ
📊 Found 2 product groups
   {"count":{"value":3,"type":"Integer"},"category":{"type":"String","value":"Electronics"},"avg_price":{"type":"Float","value":575.6666666666666}}
   {"category":{"type":"String","value":"Furniture"},"count":{"type":"Integer","value":2},"avg_price":{"type":"Float","value":474}}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   {"count":{"value":3,"type":"Integer"},"category":{"type":"String","value":"Electronics"}}
   {"count":{"type":"Integer","value":2},"category":{"value":"Furniture","type":"String"}}
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
   admin: 20.0 (3 users)
   user: 70.0 (7 users)
⏱️  Execution time: 0ms

📝 Example 4: Top Users by Score

✅ Function saved
📊 Users (showing first 5 of 10):
   1. User 3 - Score: 30
   2. User 10 - Score: 100
   3. User 4 - Score: 40
   4. User 7 - Score: 70
   5. User 6 - Score: 60
⏱️  Execution time: 0ms

📝 Example 5: User Summary Statistics

✅ Function saved
📊 User summary (2 groups):
   inactive: 3 users, Total Score: 180
   active: 7 users, Total Score: 370
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

✅ Inserted order: NcpqBu9D_xpOErtQM0nKKQrpsqBQIxHFs9JD9lWCgYhRhp55cJTOXfg9jAXQUfMriG-rgEAnvHUSuJYlrV8b_Q
✅ Inserted 2 products with wrapped types

📝 Example 2: Function with Wrapped Type Parameters

✅ Function saved: PcdpwyMb5GTXOLh7WbXOR8AjoRLnmqfhaafma8cc3J6NTSTtYf0jT3raydicwUoFCw489lRgrZO2x9BUVhAA1w
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"value":{"userId":"user_abc","role":"admin"},"type":"Object"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: g3zuaXZpYfUyl0tHSz_SYHpzv9uY_AXLxql3rPhHwyk6zNC6C5e42ezeA0TkTeCBgVV4n4yVoAK8EyjxadFVkg
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 3 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Function

✅ Function saved: sxoz5mjY1l6gRE7mqfNpXmen51xi6YgPWKEeIjSfXPgB3uLZ9OR41yCGtLPtZUB1SNW6FAazLqNjIMtJ_vK0LQ
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
   2. Vector Databases Explained (Database)
   3. Getting Started with ekoDB (Database)
   4. Natural Language Processing (AI)
   5. Database Design Principles (Database)
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
   2. Vector Databases Explained
   3. Getting Started with ekoDB
   4. Natural Language Processing
   5. Database Design Principles
⏱️  Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Function saved
📊 Projected documents (showing first 3):
   1. Introduction to Machine Learning
   2. Vector Databases Explained
   3. Getting Started with ekoDB
⏱️  Execution time: 0ms

📝 Example 5: All Document Fields

✅ Function saved
📊 All documents (5 total, showing first 2):
   1. Introduction to Machine Learning (AI)
   2. Vector Databases Explained (Database)
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
Retrieved value: { value: { username: 'john_doe', userId: 123 }, type: 'Object' }

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: { type: 'Object', value: { price: 29.99, name: 'Product 1' } }
cache:product:2: {
  type: 'Object',
  value: { price: 39.989999999999995, name: 'Product 2' }
}
cache:product:3: {
  value: { price: 49.989999999999995, name: 'Product 3' },
  type: 'Object'
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
  1. Score: 12.870, Matched: name, email
  2. Score: 6.270, Matched: name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 13.200, Matched: title, bio
  2. Score: 13.200, Matched: title, bio
  3. Score: 13.200, Matched: bio, title
  4. Score: 13.200, Matched: title, bio

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
  1. Score: 0.769, Matched:
  2. Score: 0.737, Matched:
  3. Score: 0.736, Matched:

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.508, Matched: content, title
  2. Score: 0.895, Matched: content, title
  3. Score: 0.294, Matched:

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 13.200, Matched: bio, skills

=== Cleanup ===
✅ Deleted test collections

✅ Search examples completed!
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'FYHoCH8O-mCeEOnEyA8h3jqorCIEp9ygLAcW-dv1PEYq2JcqFrEhLs5QCitRX5DMZBs8u7HcKvCOKcTzob-_dA'
}

=== Find by ID ===
Found: {
  value: { value: 42, type: 'Integer' },
  id: 'FYHoCH8O-mCeEOnEyA8h3jqorCIEp9ygLAcW-dv1PEYq2JcqFrEhLs5QCitRX5DMZBs8u7HcKvCOKcTzob-_dA',
  active: { value: true, type: 'Boolean' },
  name: { type: 'String', value: 'Test Record' }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  name: { type: 'String', value: 'Updated Record' },
  value: { type: 'Integer', value: 100 },
  id: 'FYHoCH8O-mCeEOnEyA8h3jqorCIEp9ygLAcW-dv1PEYq2JcqFrEhLs5QCitRX5DMZBs8u7HcKvCOKcTzob-_dA',
  active: { value: true, type: 'Boolean' }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: OCj2xQZO4V_P_858SoFHNhyGluBHjevwIiq2uDx3BhIe6VQcmibsyQFdEpXQ_I2DTmdfoskWlkw-McAZCYkdJQ

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
✓ Created native SWR script: github_user_native_js (GG9n7aHroopxMTz6XpzPbakCzxLnWrjFSlNvUXwlWhjORCTj9TaJ45ebyDlQDimR9KxaCbQ4ETBwe_Kjtuj1LA)

First call (cache miss - will fetch from GitHub API):
  Response time: 120ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 3ms
  Speedup: 40.0x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit_js (nu52zuni4-BsEhRm67Oc_pKlJ8ciE-7uJbdzuDNyNtQn0JmzItjeG7c71kAG1-WzFA3YOjzQUHJLbmHqsVi1WQ)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail_js' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline_js (hVKCAh9WybfMro2ePpFtWXWdOIT98A1L00E5UAhFa2p1zbvPb4O9lpc6JgpFxQXeob2yg4kY3BhCpMPc4V3CCQ)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users_swr_js' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache_js (ru3OMmx5tsISGsSGEEFPBEaX4BjqNDWATzfmmf87Cn_VQnwb_eu-pgO96sjXvJTxS0UI_RQg-R6UNNV-CHQ6dw)
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
✓ Created SWR script: fetch_api_user_js_77344_1791438615178 (70yeFu6DOO2_yqopijaDSJW-70TV19IIdgCDBnOMsqHlmF_2zxC5AYBxk3lrxNj0qCU0f4FnRQqiTcSaHz_Nhw)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "value": {
          "id": 1,
          "username": "Bret",
          "email": "Sincere@april.biz",
          "address": {
            "city": "Gwenborough",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "suite": "Apt. 556",
            "zipcode": "92998-3874",
            "street": "Kulas Light"
          },
          "website": "hildegard.org",
          "company": {
            "catchPhrase": "Multi-layered client-server neural-net",
            "bs": "harness real-time e-markets",
            "name": "Romaguera-Crona"
          },
          "phone": "1-770-736-8031 x56442",
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
Created Alice: $1000 - ID: 6fxEtk3LX1kJaCjcWDilXtCs1Uo_N19xsTCTozkw_K2Pn1uOftLnRSeOmaU5a2CUscpy02tn0yxGfCIkeEzEtg
Created Bob: $500 - ID: faC6LfegeUPm7gVqxZCBdCoILaquu8th87DBAp46nfZVct8jIgbwFSru8iO1VWAw7m9On6rJcVdq3ycQk1JmTw

=== Example 1: Begin Transaction ===
Transaction ID (server-default isolation): fbc879a2-0309-4d79-a1a5-4f9181d95057

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
New transaction: 8314440d-7582-4d93-9dc5-5a99e846ab47
Updated Bob: $700 → $600 (in transaction)
Status before rollback: Active
✓ Transaction rolled back

✓ Verified Bob remains $700 after rollback

=== Cleanup ===
✓ Deleted test account collection

✓ All client transaction examples completed
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: Fi1iS01kX3zYVue5sPc3wj4P2ZzfYeHgIx7oNiC0YWDMYWnxd-WBt0FAoQANzZ2WJVwLKmAitvGtWiWW2zfHnw

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
Inserted: aBz5il0E8sdTk_wD-V9LnJVWZOP9E-4Ev_SBfphpsl9088l_vU1XCBQOeR9nMmKLhlcI4g-ze1DACosFc-4DCg

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
✓ Created session: gcL-pVcAW4S4FX8G3ys2laryRS-jnwDKDzyPk_P-iRmo6l4v0_d9JNQhSvzEx6BDipuW5E9V3154_divUtvIyw

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** High-performance database product
- **Price:** $99

Is there anything else you would like to know?

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
✓ Created second session: 8wBSRAfifb4uXr3BlEX_R2u4fNGJaJkk4aPIKB3cLh3TgzGdlLUljQle0qhEGCO-q8KfJO1RD9fY0HUN9TgJjg
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 7

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 6

=== Cleanup ===
✓ Deleted chat session: 8wBSRAfifb4uXr3BlEX_R2u4fNGJaJkk4aPIKB3cLh3TgzGdlLUljQle0qhEGCO-q8KfJO1RD9fY0HUN9TgJjg
✓ Deleted chat session: gcL-pVcAW4S4FX8G3ys2laryRS-jnwDKDzyPk_P-iRmo6l4v0_d9JNQhSvzEx6BDipuW5E9V3154_divUtvIyw
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: 09cVkubd4wjIjfsXw1Bc8WmWV1ezshqBgWegepXDKWRl-7pENfTRV7GVNIe48AjGw5VooItyZ85bwLBU-aRLLQ

=== Sending Chat Message ===
Message ID: 5H916ZXc_A152leTKPTFFrfmLEydVC9lX2_7u_pYeFy-DjKNHJXHpevZev0BlrpTqUnNOI1UhuWBvUBvB7qA5g

=== AI Response ===
The available products along with their prices are:

1. **ekoDB** - Price: $99
   - Description: A high-performance database product with AI capabilities.

2. **ekoDB Pro** - Price: $299
   - Description: Enterprise edition product with advanced features.

3. **ekoDB Cloud** - Price: $499
   - Description: Fully managed cloud database service product.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    price: 99,
    id: 'JM2hQBors1aqoeNXl6PRs9f3VkL9TOll4mpTj7yC6oa35LeT5KxEWQsXJOZcp6SmchwDtRxKzFkW2aeifV81Rw',
    description: 'A high-performance database product with AI capabilities',
    name: 'ekoDB'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    id: 'Yh__DGjGF8KGQIfEjKmBcskXAjaUSlqKMZetdrqW_05MLoDz56VkDbXvSp5iaSAaqL7zuHgLBDOwbDxmjdYQlQ',
    description: 'Enterprise edition product with advanced features',
    name: 'ekoDB Pro',
    price: 299
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    name: 'ekoDB Cloud',
    id: '_mcCxYt4FoWsSXw6bwqeoUmsKpKH_mrfEucnp6_GUEZYlzkO0kNAjPNfqCkXJ4n9KPSMBGpHi4BXJsqzr3nHDw',
    description: 'Fully managed cloud database service product',
    price: 499
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 2212ms

=== Token Usage ===
Prompt tokens: 3413
Completion tokens: 88
Total tokens: 3501

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Message Stream (SSE) Example (TypeScript) ===

Created session: GR2Dyo77-T_nJ0qrl-mxcitxNCOZM3ZnTQ5Bi615B1fEFL8LLicDVljNpNKNkX8YW66AmMS663Vg5wx-GnFdug

Streaming response for: 'What is ekoDB?'

{"__progress":"received"}{"__progress":"generating","model":"default","provider":"openai"}**ekoDB** is an open-source database system specifically designed to optimize for **energy efficiency** in data management systems. Unlike traditional databases that primarily focus on performance or scalability, ekoDB incorporates energy consumption as a core design principle, enabling modern data platforms to substantially reduce their carbon footprint and operational costs.

### Key Features of ekoDB:
- **Energy-Aware Data Management**: Executes queries and manages data with strategies that minimize energy consumption, sometimes by trading off speed for power savings.
- **Custom Storage Engines**: Offers storage layers tailored to save energy based on hardware characteristics and workload patterns.
- **Modular Architecture**: Allows integration with other data systems or embedding within green computing pipelines.
- **Developer Tools**: Provides APIs and visualization tools to monitor and optimize energy consumption for different workloads.

### Use Cases:
- Deploying in **eco-friendly data centers** to meet sustainability goals.
- Power- and cost-sensitive environments, such as edge devices or developing regions.
- Academic research on energy-efficient computing and green IT.

**Status:**
ekoDB is developed and maintained by researchers and practitioners interested in sustainable computing. It’s available on platforms like GitHub and is typically used in research, prototyping, or pilot projects aiming for green technology.

---

**Note:** If you meant “ekoDB” in a different context or a specific product, please clarify, as the term may have other meanings in unrelated fields.

--- Stream complete ---
Message ID: S_1jqHvl2KAp9oj5AZmt8F4_7VvcEAtn9x00q2OIOWlKMrpWQkDR-cmXBX_BtwaZ_wVjJsF-5-yyMf7ui9nGdA
Execution time: 2637ms
Context window: 1000000 tokens

✓ Chat message stream example completed
✓ Client created

=== List Chat Models ===
Available chat models by provider:
  openai:
    - o3-2025-04-16
    - gpt-5.1-chat-latest
    - gpt-5.3-chat-latest
    - gpt-5.2-2025-12-11
    - sora-2
    - chatgpt-image-latest
    - gpt-4.1-mini
    - gpt-3.5-turbo
    - gpt-4o-mini
    - gpt-image-2.5-sunburst
    - gpt-image-2.5-flare-2026-09-08
    - gpt-6-luna
    - gpt-5.4-mini-2026-03-17
    - gpt-4-turbo
    - gpt-4.1-nano
    - gpt-image-2
    - gpt-5.4-pro
    - gpt-realtime-mini-2025-12-15
    - gpt-3.5-turbo-16k
    - babbage-002
    - text-embedding-ada-002
    - gpt-4o-mini-tts
    - gpt-4o-2024-08-06
    - o4-mini-2025-04-16
    - gpt-4o-2024-11-20
    - omni-moderation-latest
    - gpt-realtime
    - tts-1-hd
    - gpt-realtime-1.5
    - gpt-realtime-2
    - gpt-5-search-api
    - gpt-5.5-2026-04-23
    - gpt-5.1
    - gpt-5.1-codex
    - gpt-4o
    - o1-2024-12-17
    - gpt-5.2-chat-latest
    - gpt-4o-mini-tts-2025-12-15
    - gpt-image-1
    - davinci-002
    - gpt-image-2-2026-04-21
    - gpt-4o-mini-transcribe
    - o3-mini
    - gpt-audio
    - gpt-5.4-nano
    - o3
    - gpt-5.1-codex-max
    - gpt-realtime-2.1
    - gpt-4.1
    - gpt-5.5
    - gpt-4o-mini-search-preview-2025-03-11
    - text-embedding-3-large
    - gpt-4o-mini-search-preview
    - gpt-4o-2024-05-13
    - gpt-6-sol
    - gpt-5.3-codex
    - gpt-3.5-turbo-instruct
    - sora-2-pro
    - gpt-5
    - gpt-audio-mini-2025-12-15
    - gpt-transcribe
    - o4-mini-deep-research-2025-06-26
    - o1-pro-2025-03-19
    - gpt-4.1-nano-2025-04-14
    - gpt-5-search-api-2025-10-14
    - gpt-4-turbo-2024-04-09
    - gpt-realtime-2.1-mini
    - tts-1
    - gpt-5-mini
    - omni-moderation-2024-09-26
    - gpt-5-mini-2025-08-07
    - gpt-realtime-2025-08-28
    - gpt-5.4-2026-03-05
    - gpt-live-transcribe
    - gpt-3.5-turbo-0125
    - gpt-5-nano
    - gpt-5.6-luna
    - gpt-4
    - gpt-image-2.5-sunburst-2026-09-08
    - whisper-1
    - gpt-5.4-mini
    - gpt-5-pro
    - o1-pro
    - gpt-realtime-whisper
    - gpt-5.2-pro
    - gpt-5-chat-latest
    - gpt-live-1
    - gpt-4o-mini-transcribe-2025-12-15
    - gpt-5-2025-08-07
    - tts-1-1106
    - gpt-5.5-pro
    - gpt-5-pro-2025-10-06
    - chat-latest
    - gpt-5.2-pro-2025-12-11
    - gpt-4o-mini-transcribe-2025-03-20
    - gpt-audio-2025-08-28
    - gpt-5.4-pro-2026-03-05
    - gpt-realtime-translate
    - gpt-5.6-sol
    - gpt-audio-mini-2025-10-06
    - gpt-5.5-pro-2026-04-23
    - tts-1-hd-1106
    - gpt-realtime-mini
    - gpt-5.4
    - gpt-4o-transcribe-diarize
    - gpt-5-codex
    - gpt-image-1-mini
    - gpt-image-1.5
    - gpt-5.1-2025-11-13
    - o4-mini-deep-research
    - gpt-4o-search-preview-2025-03-11
    - o3-mini-2025-01-31
    - gpt-4-0613
    - gpt-4o-search-preview
    - gpt-5.6-terra
    - text-embedding-3-small
    - gpt-audio-1.5
    - gpt-3.5-turbo-instruct-0914
    - gpt-6-astra
    - gpt-5.2-codex
    - gpt-3.5-turbo-1106
    - gpt-5-nano-2025-08-07
    - gpt-audio-mini
    - gpt-4o-mini-tts-2025-03-20
    - gpt-5.2
    - gpt-4.1-mini-2025-04-14
    - gpt-4.1-2025-04-14
    - gpt-4o-mini-2024-07-18
    - o4-mini
    - o1
    - gpt-5.4-nano-2026-03-17
    - gpt-5.1-codex-mini
    - gpt-4o-transcribe
    - gpt-image-2.5-flare
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
Provider status:
  anthropic: ok 14 models
  gemini: not_configured (unverified) No Gemini API Key
  openai: ok 135 models
  perplexity: not_configured (unverified) No Perplexity API Key

=== Get Specific Provider Models ===
OpenAI models: o3-2025-04-16, gpt-5.1-chat-latest, gpt-5.3-chat-latest, gpt-5.2-2025-12-11, sora-2, chatgpt-image-latest, gpt-4.1-mini, gpt-3.5-turbo, gpt-4o-mini, gpt-image-2.5-sunburst, gpt-image-2.5-flare-2026-09-08, gpt-6-luna, gpt-5.4-mini-2026-03-17, gpt-4-turbo, gpt-4.1-nano, gpt-image-2, gpt-5.4-pro, gpt-realtime-mini-2025-12-15, gpt-3.5-turbo-16k, babbage-002, text-embedding-ada-002, gpt-4o-mini-tts, gpt-4o-2024-08-06, o4-mini-2025-04-16, gpt-4o-2024-11-20, omni-moderation-latest, gpt-realtime, tts-1-hd, gpt-realtime-1.5, gpt-realtime-2, gpt-5-search-api, gpt-5.5-2026-04-23, gpt-5.1, gpt-5.1-codex, gpt-4o, o1-2024-12-17, gpt-5.2-chat-latest, gpt-4o-mini-tts-2025-12-15, gpt-image-1, davinci-002, gpt-image-2-2026-04-21, gpt-4o-mini-transcribe, o3-mini, gpt-audio, gpt-5.4-nano, o3, gpt-5.1-codex-max, gpt-realtime-2.1, gpt-4.1, gpt-5.5, gpt-4o-mini-search-preview-2025-03-11, text-embedding-3-large, gpt-4o-mini-search-preview, gpt-4o-2024-05-13, gpt-6-sol, gpt-5.3-codex, gpt-3.5-turbo-instruct, sora-2-pro, gpt-5, gpt-audio-mini-2025-12-15, gpt-transcribe, o4-mini-deep-research-2025-06-26, o1-pro-2025-03-19, gpt-4.1-nano-2025-04-14, gpt-5-search-api-2025-10-14, gpt-4-turbo-2024-04-09, gpt-realtime-2.1-mini, tts-1, gpt-5-mini, omni-moderation-2024-09-26, gpt-5-mini-2025-08-07, gpt-realtime-2025-08-28, gpt-5.4-2026-03-05, gpt-live-transcribe, gpt-3.5-turbo-0125, gpt-5-nano, gpt-5.6-luna, gpt-4, gpt-image-2.5-sunburst-2026-09-08, whisper-1, gpt-5.4-mini, gpt-5-pro, o1-pro, gpt-realtime-whisper, gpt-5.2-pro, gpt-5-chat-latest, gpt-live-1, gpt-4o-mini-transcribe-2025-12-15, gpt-5-2025-08-07, tts-1-1106, gpt-5.5-pro, gpt-5-pro-2025-10-06, chat-latest, gpt-5.2-pro-2025-12-11, gpt-4o-mini-transcribe-2025-03-20, gpt-audio-2025-08-28, gpt-5.4-pro-2026-03-05, gpt-realtime-translate, gpt-5.6-sol, gpt-audio-mini-2025-10-06, gpt-5.5-pro-2026-04-23, tts-1-hd-1106, gpt-realtime-mini, gpt-5.4, gpt-4o-transcribe-diarize, gpt-5-codex, gpt-image-1-mini, gpt-image-1.5, gpt-5.1-2025-11-13, o4-mini-deep-research, gpt-4o-search-preview-2025-03-11, o3-mini-2025-01-31, gpt-4-0613, gpt-4o-search-preview, gpt-5.6-terra, text-embedding-3-small, gpt-audio-1.5, gpt-3.5-turbo-instruct-0914, gpt-6-astra, gpt-5.2-codex, gpt-3.5-turbo-1106, gpt-5-nano-2025-08-07, gpt-audio-mini, gpt-4o-mini-tts-2025-03-20, gpt-5.2, gpt-4.1-mini-2025-04-14, gpt-4.1-2025-04-14, gpt-4o-mini-2024-07-18, o4-mini, o1, gpt-5.4-nano-2026-03-17, gpt-5.1-codex-mini, gpt-4o-transcribe, gpt-image-2.5-flare, gpt-6.1-sol

=== Get Anthropic Models ===
Anthropic models: claude-haiku-5-5, claude-sonnet-5-5, claude-opus-5-5, claude-fable-5-1, claude-opus-5, claude-sonnet-5, claude-fable-5, claude-opus-4-8, claude-opus-4-7, claude-sonnet-4-6, claude-opus-4-6, claude-opus-4-5-20251101, claude-haiku-4-5-20251001, claude-sonnet-4-5-20250929

=== Get Non-Existent Provider ===
Expected error for non-existent provider: Error: Request failed with status 404: {"error":"Unknown provider 'nonexistent_provider_xyz'","error_kind":"unknown_provider"}

✓ Chat Models example complete
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: LBtNdoUOgYJOv7bTWRklLXbrw1s4OdE2oQgCbaid5H94UJriBx4-gmPFnHlP_wSHJn6oIZMw6NHIax3eyw5gQQ

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** A high-performance database product
- **Price:** $99

If you need more details or have other questions, feel free to ask!

✓ Message 2 sent
  Response: The price of ekoDB is $99. If you have any more questions or need further information, feel free to ask!

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 86P716BFAPkMWT3IrxS3KWBkPzZKDHzgthw9KwCNfm2DyyuaTrOLm6Qk71HAv25gipBl6rDt5HGJyMgF8dawGw
  Parent: LBtNdoUOgYJOv7bTWRklLXbrw1s4OdE2oQgCbaid5H94UJriBx4-gmPFnHlP_wSHJn6oIZMw6NHIax3eyw5gQQ

=== Listing Sessions ===
✓ Found 2 sessions
  Session 1: 86P716BFAPkMWT3IrxS3KWBkPzZKDHzgthw9KwCNfm2DyyuaTrOLm6Qk71HAv25gipBl6rDt5HGJyMgF8dawGw (Untitled)
  Session 2: LBtNdoUOgYJOv7bTWRklLXbrw1s4OdE2oQgCbaid5H94UJriBx4-gmPFnHlP_wSHJn6oIZMw6NHIax3eyw5gQQ (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: 86P716BFAPkMWT3IrxS3KWBkPzZKDHzgthw9KwCNfm2DyyuaTrOLm6Qk71HAv25gipBl6rDt5HGJyMgF8dawGw

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ All session management operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: iGvqxfK1Pt2kjHTRxlt0j8Jc72E5yim8MvO5oAxjaKCezLoDfxrYN7gU2NillTJfBHH0VzIF9h6DBZY16uEASw

=== List Collections ===
Total collections: 27
Sample collections: chat_agent_configs__ek0_testing,chat_goal_templates__ek0_testing,schema_documents_client_js,schema_documents_client_go,test_accounts

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
  POST /api/functions/conc_demo_pay_ts_77654_1791438637251 { "idempotency_key": "...", "amount": 100 }
  POST /api/functions/conc_demo_rl_fail_ts_77654_1791438637251 { "user_id": 42 }
  POST /api/functions/conc_demo_rl_skip_ts_77654_1791438637251 { "user_id": 42 }
  POST /api/functions/conc_demo_lock_ts_77654_1791438637251 { "resource": "queue:drain" }

✓ Cleaned up demo functions
=== ekoDB Convenience Methods Example ===

=== Native Object Creation ===
✓ Created record with plain object: {
  id: 'VF7yeTDbYbkKltwgWw-SN-MB7VCQYj8_VYkZ8mhFDZKjYmg4pa_YsLorDWfDKfsNX5vUIyN4SYRTl5HklBRpCA'
}

=== Upsert Operation ===
✓ First upsert (update): {
  id: 'VF7yeTDbYbkKltwgWw-SN-MB7VCQYj8_VYkZ8mhFDZKjYmg4pa_YsLorDWfDKfsNX5vUIyN4SYRTl5HklBRpCA',
  age: { value: 29, type: 'Integer' },
  name: { value: 'Alice Johnson', type: 'String' },
  active: { value: true, type: 'Boolean' },
  email: { type: 'String', value: 'alice.j@newdomain.com' }
}
✓ Second upsert (insert): { id: 'new-user-id' }

=== Find One Operation ===
✓ Found user by email: {
  active: { value: true, type: 'Boolean' },
  name: { value: 'Alice Johnson', type: 'String' },
  age: { value: 29, type: 'Integer' },
  email: { value: 'alice.j@newdomain.com', type: 'String' },
  id: 'VF7yeTDbYbkKltwgWw-SN-MB7VCQYj8_VYkZ8mhFDZKjYmg4pa_YsLorDWfDKfsNX5vUIyN4SYRTl5HklBRpCA'
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
✓ Inserted document: b5ZUFaIGY1a3AO1Ex5RvG_Tq6PDjqgZc8GeFsLpFJH6UipixqDe3tdezWEGdwet3lzG0KAJokLPnIpFEcKCIGA

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: 4ixr1xclCuamfB7iHhO2qdPJ8uM8mA-JJB3jzKgByLVCOklyrHbN37zDkWcLcBbxEKemZH7ShYqvqFjEY85wJg

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
✓ Edge cache script created: TiVmyS7dgk_sQ0khOcN8BmGFRncf-kMjMRy-e2q8xO_6R-0e6h22PFAVkZ3BQSiXH_HLz1IwYBBbJmQsJhEtsw

Call 1: Cache miss (fetches from API)
Response time: 150ms
Result: {
  "records": [
    {
      "value": {
        "value": {
          "name": "Leanne Graham",
          "id": 1,
          "username": "Bret",
          "company": {
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets"
          },
          "address": {
            "suite": "Apt. 556",
            "street": "Kulas Light",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "city": "Gwenborough",
            "zipcode": "92998-3874"
          },
          "phone": "1-770-736-8031 x56442",
          "website": "hildegard.org",
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
Response time: 3ms (50x faster!)
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "name": "Leanne Graham",
          "id": 1,
          "username": "Bret",
          "company": {
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets"
          },
          "address": {
            "suite": "Apt. 556",
            "street": "Kulas Light",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "city": "Gwenborough",
            "zipcode": "92998-3874"
          },
          "phone": "1-770-736-8031 x56442",
          "website": "hildegard.org",
          "email": "Sincere@april.biz"
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
   Name: {"type":"String","value":"User 1"}
   Department: {"type":"String","value":"engineering"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 69ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "name": "Leanne Graham",
      "username": "Bret",
      "website": "hildegard.org",
      "email": "Sincere@april.biz",
      "address": {
        "suite": "Apt. 5...

Second call (cache hit - from cache):
   ⏱️  Duration: 3ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "name": "Leanne Graham",
      "username": "Bret",
      "website": "hildegard.org",
      "email": "Sincere@april.biz",
      "address": {
        "suite": "Apt. 5...
   🚀 Cache speedup: 23.0x faster!

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

✅ Function saved: geAp2kpmcbW2pX-R4LqNOJm2LKSIhXjh3I5-Ej78feX4aWh97ZzjGmjAAsb5pVLGDO01JIKqVzh4cIiqymi_NA
📊 Found 5 active users

📝 Example 2: Parameterized Function

✅ Function saved: KrHzkFUIkCp3yOWb2DodTjI4VfKKa-DA2HP1DPdhl-hevNfXWnCaKDoEjewvvKtLqNxHNsYUgvAEDFViyQogTg
📊 Found 3 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: qQ5sqG2FKZMxLJ5K3UAxAAUPXXeRcu77yu8xhX3iFOncptm8--XuzOSJ4RUTlFZoLAx2A65As4jKoGyXS25uaQ
📊 Statistics: 2 groups
   {"count":{"value":5,"type":"Integer"},"avg_score":{"value":60,"type":"Float"},"status":{"type":"String","value":"active"}}
   {"count":{"value":5,"type":"Integer"},"avg_score":{"value":50,"type":"Float"},"status":{"value":"inactive","type":"String"}}

📝 Example 4: UserFunction Management

📋 Total scripts: 10
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
   {"count":{"type":"Integer","value":5},"category":{"value":"Electronics","type":"String"},"avg_price":{"type":"Float","value":367}}
   {"count":{"value":3,"type":"Integer"},"category":{"value":"Furniture","type":"String"},"avg_price":{"type":"Float","value":365.6666666666667}}
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
   Vector databases offer several benefits, including:

1. **Efficient Similarity Search**: They enable rapid retrieval of similar items based on vector embeddings, which is ideal for tasks like image, text, and recommendation systems.

2. **Scalability**: They can handle large volumes of high-dimensional data efficiently, making them suitable for big data applications.

3. **Real-Time Queries**: Vector databases support real-time querying, facilitating dynamic applications and interactive AI features.

4. **Multi-Modality Support**: They can integrate and query diverse data types (text, images, audio) by representing them as vectors.

5. **Enhanced Machine Learning Integration**: They seamlessly integrate with machine learning workflows, allowing for direct storage and querying of model outputs.

6. **High Dimensionality Handling**: Designed to manage and optimize for high-dimensional spaces effectively, avoiding the "curse of dimensionality."

7. **Indexing Techniques**: Utilize advanced indexing methods (like HNSW, IVF) to speed up nearest neighbor searches.

8. **Flexibility**: Suitable for a variety of applications, from natural language processing to computer vision.

9. **Improved Accuracy**: More accurate results in finding nearest neighbors compared to traditional databases due to vector representations.

10. **Support for Advanced Analytics**: Enables complex analytics and insights generation by leveraging the underlying vector representations.
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

✅ Function saved: 7vGqUEha7vbBCjcA55L3O0Gv-1VUZh-xQKeAzP_uQ86YiMJ3hnM_6mzEAcxywv0W7fNNYmFU41j5ixavPQx6ow
📊 Found 2 product groups
   {"avg_price":{"type":"Float","value":575.6666666666666},"category":{"value":"Electronics","type":"String"},"count":{"type":"Integer","value":3}}
   {"avg_price":{"type":"Float","value":474},"category":{"value":"Furniture","type":"String"},"count":{"type":"Integer","value":2}}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   {"count":{"value":2,"type":"Integer"},"category":{"value":"Furniture","type":"String"}}
   {"category":{"value":"Electronics","type":"String"},"count":{"value":3,"type":"Integer"}}
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
   inactive: 3 users
   active: 7 users
⏱️  Execution time: 0ms

📝 Example 3: Average Score by Role

✅ Function saved
📊 Average score by role:
   {"avg_score":{"type":"Float","value":70},"role":{"value":"user","type":"String"},"count":{"type":"Integer","value":7}}
   {"role":{"type":"String","value":"admin"},"count":{"value":3,"type":"Integer"},"avg_score":{"value":20,"type":"Float"}}
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

✅ Inserted order: ev5rl-Vt4B2K_DhChOLq9rGP6lZhwwd0QNjF3TBhWNIGvTA4AgT02M1X5HR5RZLgZWasNs8ZmgITZmrc-ijFtw
✅ Inserted 2 products with wrapped types

📝 Example 2: UserFunction with Wrapped Type Parameters

✅ Function saved: tZlGOEx0nNU3glfXFhy2Fq1q1oCNS6JTYN9QW9n83lsrJjV-v5Mc_wn1dCoYffBmywuPAngiQ4VXMIWWqw2D4g
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"type":"Object","value":{"role":"admin","userId":"user_abc"}}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: 4WpGwkbCD3kWm6dw7GXtHcMxRJAAvvzjYiftjcIkzFP6Msi5Mk3I9Lfx-t5hcc3_LzDhnawfBFYJipC8ABvMHg
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 3 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Function

✅ Function saved: TZzWfebyTz-cnIQEQuHSAhmQ3my94nv-qldU2NicFNnvQlMvB_yXaXpUBk8skQcC9VebUQ2jEoH9ybQQAPcVWA
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
   1. Database Design Principles (Database)
   2. Getting Started with ekoDB (Database)
   3. Natural Language Processing (AI)
   4. Vector Databases Explained (Database)
   5. Introduction to Machine Learning (AI)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   {"category":{"value":"Database","type":"String"},"count":{"type":"Integer","value":3}}
   {"count":{"value":2,"type":"Integer"},"category":{"type":"String","value":"AI"}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
=== ekoDB Goal Template CRUD Example (TypeScript) ===

--- Creating goal template ---
Created template: Data Migration (id: TsSw9U4UDcgcVJjzWjmoHbn7GYqqL6u08FXQjQ_csbEKArEPFTYROQtK3dCwHKk_aVjY2typpz11NNko6ztBZg)

--- Listing templates ---
Templates: {
  count: 1,
  items: [
    {
      description: [Object],
      id: 'TsSw9U4UDcgcVJjzWjmoHbn7GYqqL6u08FXQjQ_csbEKArEPFTYROQtK3dCwHKk_aVjY2typpz11NNko6ztBZg',
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
Created goal: Deploy v2.0 (id: VpVedpa7IugvOujdhZ17lBz8McAFbagb57MdXEqNCF2-BmzTVYi3oicPs61kmdjevICkLts2NQfePNQW7g1B2Q)

--- Listing goals ---
Goals: {
  "count": 1,
  "goals": [
    {
      "created_at": "2026-10-08T05:50:43.724414+00:00",
      "description": "Ship version 2.0 to production",
      "id": "VpVedpa7IugvOujdhZ17lBz8McAFbagb57MdXEqNCF2-BmzTVYi3oicPs61kmdjevICkLts2NQfePNQW7g1B2Q",
      "status": "pending",
      "steps": "[{\"description\":\"Run test suite\"},{\"description\":\"Build release artifacts\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]",
      "title": "Deploy v2.0",
      "updated_at": "2026-10-08T05:50:43.724414+00:00"
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
        "value": "2026-10-08T05:50:43.724414+00:00"
      },
      "description": {
        "type": "String",
        "value": "Ship version 2.0 with hot-fix patches"
      },
      "id": "VpVedpa7IugvOujdhZ17lBz8McAFbagb57MdXEqNCF2-BmzTVYi3oicPs61kmdjevICkLts2NQfePNQW7g1B2Q",
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
        "value": "2026-10-08T05:50:43.736268+00:00"
      }
    }
  ]
}

--- Goal step: start step 0 ---
Step 0 started on goal VpVedpa7IugvOujdhZ17lBz8McAFbagb57MdXEqNCF2-BmzTVYi3oicPs61kmdjevICkLts2NQfePNQW7g1B2Q
--- Goal step: complete step 0 ---
Step 0 completed on goal VpVedpa7IugvOujdhZ17lBz8McAFbagb57MdXEqNCF2-BmzTVYi3oicPs61kmdjevICkLts2NQfePNQW7g1B2Q
--- Goal step: fail step 1 ---
Step 1 failed on goal VpVedpa7IugvOujdhZ17lBz8McAFbagb57MdXEqNCF2-BmzTVYi3oicPs61kmdjevICkLts2NQfePNQW7g1B2Q

--- Completing goal ---
Goal status: pending_review
--- Approving goal ---
Goal status after approve: in_progress

--- Creating goal to reject ---
--- Rejecting goal ---
Goal status after reject: failed


--- Creating task ---
Created task: Hourly Health Check (id: b-ZLzoo7He4v5oHE0E6s6eOQ9XhV1Lt6hwtdWk-dbg49gGxiFpwsWE2uKOXeXYzDiYLQWM3D4sJq2h6vTvG4OQ)

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
      "id": "b-ZLzoo7He4v5oHE0E6s6eOQ9XhV1Lt6hwtdWk-dbg49gGxiFpwsWE2uKOXeXYzDiYLQWM3D4sJq2h6vTvG4OQ",
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
Created agent: SupportBot (id: zx-12eeHbW-1vWbsrbPNu8Uccu6AMlUpXHjc9p-zrFy7RS_-dJXrkIy4GQ9z4_y_bSUBdRFAp4HxsDMQJ5RNPg)

--- Listing agents ---
Agents: {
  "count": 1,
  "items": [
    {
      "deployment_id": {
        "type": "String",
        "value": "deploy_prod_1"
      },
      "id": "zx-12eeHbW-1vWbsrbPNu8Uccu6AMlUpXHjc9p-zrFy7RS_-dJXrkIy4GQ9z4_y_bSUBdRFAp4HxsDMQJ5RNPg",
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
By name: SupportBot (id: zx-12eeHbW-1vWbsrbPNu8Uccu6AMlUpXHjc9p-zrFy7RS_-dJXrkIy4GQ9z4_y_bSUBdRFAp4HxsDMQJ5RNPg)

--- Updating agent ---
Updated agent: SupportBot

--- Getting agents by deployment ---
Agents in deployment: {
  "count": 0,
  "items": []
}
WARNING: agents-by-deployment omitted created agent zx-12eeHbW-1vWbsrbPNu8Uccu6AMlUpXHjc9p-zrFy7RS_-dJXrkIy4GQ9z4_y_bSUBdRFAp4HxsDMQJ5RNPg; TODO: check/fix the server-side deployment lookup

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


✅ Join operations examples completed!
=== Cleanup ===
✅ Deleted test collections
✓ Client created
✓ ts_users_register saved
✓ ts_users_login saved
✓ ts_users_verify_token saved

=== Auth flow defined as pure stored functions ===
Call them like:
  POST /api/functions/jwt_register_ts_77704_1791438644228 { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/jwt_login_ts_77704_1791438644228 { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/jwt_verify_ts_77704_1791438644228 { "token": "<jwt>" }

Set JWT_SECRET in ekoDB's environment_vars whitelist before invoking.

✓ Cleaned up demo functions
=== ekoDB KV Document Linking Example (TypeScript) ===

--- Setup: creating KV key and documents ---
Set KV key: session:admin
Inserted document 1: T5otXiP3KE7uafMxmOKX46NXefLT1EDewEFA___LMaLcjUsiIXRt9xmV-Vo0LbVEPQfvtu6RcOuAWMWlijjfwQ
Inserted document 2: K3-0yUE03TfiU4tLZ1EDD4ThSBEUf42uWbwursWOyMIKDItqqWd1SuyPcSjzUtlYMdyoNIVCKO8z3SiFQpqJdQ

--- Linking documents to KV key ---
Linked doc T5otXiP3KE7uafMxmOKX46NXefLT1EDewEFA___LMaLcjUsiIXRt9xmV-Vo0LbVEPQfvtu6RcOuAWMWlijjfwQ: null
Linked doc K3-0yUE03TfiU4tLZ1EDD4ThSBEUf42uWbwursWOyMIKDItqqWd1SuyPcSjzUtlYMdyoNIVCKO8z3SiFQpqJdQ: null

--- Getting links for KV key ---
Links: [
  {
    "collection": "kv_links_example_ts_77706_1791438644309",
    "document_id": "T5otXiP3KE7uafMxmOKX46NXefLT1EDewEFA___LMaLcjUsiIXRt9xmV-Vo0LbVEPQfvtu6RcOuAWMWlijjfwQ",
    "field_path": null,
    "created_at": "2026-10-08T05:50:44.389902Z",
    "last_accessed": "2026-10-08T05:50:44.392615Z",
    "metadata": {}
  },
  {
    "collection": "kv_links_example_ts_77706_1791438644309",
    "document_id": "K3-0yUE03TfiU4tLZ1EDD4ThSBEUf42uWbwursWOyMIKDItqqWd1SuyPcSjzUtlYMdyoNIVCKO8z3SiFQpqJdQ",
    "field_path": null,
    "created_at": "2026-10-08T05:50:44.391336Z",
    "last_accessed": "2026-10-08T05:50:44.392615Z",
    "metadata": {}
  }
]

--- Unlinking document ---
Unlinked doc K3-0yUE03TfiU4tLZ1EDD4ThSBEUf42uWbwursWOyMIKDItqqWd1SuyPcSjzUtlYMdyoNIVCKO8z3SiFQpqJdQ: null

--- Verifying remaining links ---
Remaining links: [
  {
    "collection": "kv_links_example_ts_77706_1791438644309",
    "document_id": "T5otXiP3KE7uafMxmOKX46NXefLT1EDewEFA___LMaLcjUsiIXRt9xmV-Vo0LbVEPQfvtu6RcOuAWMWlijjfwQ",
    "field_path": null,
    "created_at": "2026-10-08T05:50:44.389902Z",
    "last_accessed": "2026-10-08T05:50:44.394857Z",
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
Retrieved value: { type: 'Object', value: { userId: 123, username: 'john_doe' } }

=== KV Batch Set ===
✓ Batch set 3 keys
  kv_ops_ts_77712_1791438644484:cache:product:1: success
  kv_ops_ts_77712_1791438644484:cache:product:2: success
  kv_ops_ts_77712_1791438644484:cache:product:3: success

=== KV Batch Get ===
✓ Batch retrieved 3 values
  kv_ops_ts_77712_1791438644484:cache:product:1: { name: 'Product 1', price: 29.99 }
  kv_ops_ts_77712_1791438644484:cache:product:2: { name: 'Product 2', price: 39.99 }
  kv_ops_ts_77712_1791438644484:cache:product:3: { price: 49.99, name: 'Product 3' }

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
  kv_ops_ts_77712_1791438644484:cache:product:1: deleted
  kv_ops_ts_77712_1791438644484:cache:product:2: deleted
  kv_ops_ts_77712_1791438644484:cache:product:3: deleted

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
    "name": "Venus",
    "diameter_km": 12104
  },
  {
    "name": "Saturn",
    "diameter_km": 116460
  }
]

--- Blocking HTTP (for comparison) ---
Blocking response: Hello! I hope you're having a wonderful day.

=== Done ===
=== ekoDB Schedule Management Example (TypeScript) ===

--- Creating schedule ---
Created schedule: Nightly Database Backup (id: 2b2b6bbe-466b-4bc8-842e-69856be32368, cron: 0 0 2 * * *)

--- Listing schedules ---
Schedules: {
  "count": 1,
  "schedules": [
    {
      "created_at": "2026-10-08T05:50:47.727746Z",
      "cron_expression": "0 0 2 * * *",
      "description": null,
      "enabled": true,
      "function_label": "schedule_noop_typescript_77731_1791438647694",
      "id": "2b2b6bbe-466b-4bc8-842e-69856be32368",
      "last_execution": null,
      "name": "Nightly Database Backup",
      "next_execution": "2026-10-09T02:00:00Z",
      "parameters": {},
      "stats": {
        "avg_execution_time_ms": 0,
        "failed_executions": 0,
        "last_error": null,
        "successful_executions": 0,
        "total_executions": 0
      },
      "timezone": "UTC",
      "updated_at": "2026-10-08T05:50:47.727746Z"
    }
  ]
}

--- Getting schedule ---
Fetched: Nightly Database Backup (cron: 0 0 2 * * *)

--- Updating schedule ---
Updated: Nightly Full Backup (new cron: 0 0 3 * * *)

--- Triggering schedule ---
Trigger response: {
  "schedule_id": "2b2b6bbe-466b-4bc8-842e-69856be32368",
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
  3. Score: 13.200, Matched: title, bio
  4. Score: 13.200, Matched: title, bio

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
  1. Score: 0.765, Matched:
  2. Score: 0.737, Matched:
  3. Score: 0.733, Matched:

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.506, Matched: content, title
  2. Score: 0.895, Matched: title, content
  3. Score: 0.293, Matched:

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 13.200, Matched: bio, skills

9. Vector search with a metadata pre-filter (category = ml):
Found 2 documents in category "ml" (NLP excluded)
  1. Introduction to Machine Learning (category: ml)
  2. Deep Learning Fundamentals (category: ml)


✅ Search examples completed!
=== Cleanup ===
✅ Deleted test collections
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'qpxDwlWynU2V1RXgtYBQxpRfYgNCag0mYOjkFl4ocRXaEjnPf5lSaZh2oHMzZcT_uJTKjSnk9bx6qcIcs7ZgnA'
}

=== Find by ID ===
Found: {
  id: 'qpxDwlWynU2V1RXgtYBQxpRfYgNCag0mYOjkFl4ocRXaEjnPf5lSaZh2oHMzZcT_uJTKjSnk9bx6qcIcs7ZgnA',
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  value: { value: 42, type: 'Integer' },
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] },
  created_at: { type: 'DateTime', value: '2026-10-08T05:50:48.225+00:00' },
  user_id: { value: '550e8400-e29b-41d4-a716-446655440000', type: 'String' },
  tags: { value: [ 'tag1', 'tag2', 'tag3' ], type: 'Array' },
  name: { value: 'Test Record', type: 'String' },
  price: { type: 'Float', value: 99.99 },
  categories: { type: 'Array', value: [ 'electronics', 'computers' ] },
  active: { value: true, type: 'Boolean' },
  metadata: { type: 'Object', value: { nested: [Object], key: 'value' } }
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-10-08T05:50:48.225Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { nested: { deep: true }, key: 'value' }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  id: 'qpxDwlWynU2V1RXgtYBQxpRfYgNCag0mYOjkFl4ocRXaEjnPf5lSaZh2oHMzZcT_uJTKjSnk9bx6qcIcs7ZgnA',
  data: 'aGVsbG8gd29ybGQ=',
  value: 42,
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  created_at: '2026-10-08T05:50:48.225+00:00',
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  tags: [ 'tag1', 'tag2', 'tag3' ],
  name: 'Test Record',
  price: 99.99,
  categories: [ 'electronics', 'computers' ],
  active: true,
  metadata: { nested: { deep: true }, key: 'value' }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  active: { value: true, type: 'Boolean' },
  created_at: { type: 'DateTime', value: '2026-10-08T05:50:48.225+00:00' },
  categories: { value: [ 'electronics', 'computers' ], type: 'Array' },
  name: { value: 'Updated Record', type: 'String' },
  id: 'qpxDwlWynU2V1RXgtYBQxpRfYgNCag0mYOjkFl4ocRXaEjnPf5lSaZh2oHMzZcT_uJTKjSnk9bx6qcIcs7ZgnA',
  value: { value: 100, type: 'Integer' },
  user_id: { value: '550e8400-e29b-41d4-a716-446655440000', type: 'String' },
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] },
  metadata: { value: { nested: [Object], key: 'value' }, type: 'Object' },
  price: { type: 'Float', value: 99.99 }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: BrunUyurA8ov0v5GDQlx6FbswsleqVTXeC-YqpH31EilIkywCtnnVmNgPeJSvpgGb2WhQbsKHr8TrZBEy7EauA

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
✓ Created native SWR script: github_user_native_ts (DKSbuidco99SRWawVi-t06tScHvvVtfQQqOgUDqsJqw3dAh9OLxsFIlG-_OKT8i-6GuP2IAqD9UjHvDDEKCXiQ)

First call (cache miss - will fetch from GitHub API):
  Response time: 128ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 3ms
  Speedup: 42.7x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit_ts (wYTHiMpcOEE73o5-t9KcH1D1z-qM3bHr0oJNkuXZQLXt31rmPLY-fCGjEr9lsqidKpVCApFskyXzTtVkSTNDHw)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail_ts' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline_ts (MYLyrPbsUxVhDjzOVRewtErBgtWiIwXlxqqnS430vk3m6tY8KIgTv0EdKa4WpCK-NeQdnegz8dZbxPlPtET3dw)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users_swr_ts' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache_ts (ufvE92pyHb7DxgQ7L3-NhO9Www0kahHc1xqq61SOEbokJve7ieNFdXP6bxx8LHJgFLU9y8cVL1UGjmKMc2Zyrw)
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
✓ Created SWR script: fetch_api_user_ts_77743_1791438649451 (0mSUSERcVENQM061_TZ2LCt3Mdf6cSyL7twI6dTztUPJMV5CCvILUef1FkRGR_N6TRb34G8ya1ItwQotx1h8Pw)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "value": {
          "address": {
            "street": "Kulas Light",
            "suite": "Apt. 556",
            "zipcode": "92998-3874",
            "city": "Gwenborough",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            }
          },
          "email": "Sincere@april.biz",
          "company": {
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona"
          },
          "id": 1,
          "name": "Leanne Graham",
          "phone": "1-770-736-8031 x56442",
          "username": "Bret",
          "website": "hildegard.org"
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
Result (cached): {
  "records": [
    {
      "value": {
        "value": {
          "address": {
            "street": "Kulas Light",
            "suite": "Apt. 556",
            "zipcode": "92998-3874",
            "city": "Gwenborough",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            }
          },
          "email": "Sincere@april.biz",
          "company": {
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona"
          },
          "id": 1,
          "name": "Leanne Graham",
          "phone": "1-770-736-8031 x56442",
          "username": "Bret",
          "website": "hildegard.org"
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
✓ Created enrichment script: fetch_product_reviews_ts_77743_1791438649451 (AXuz_Da6s-90JYjoKpxyuDRuLJjHiIYrD5lJfIC9IwoLKQIcHGSPyWi5LSeuVpk4KWWBqXuYZQb33jpYMOVW8Q)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {
      "value": {
        "value": {
          "category": "beauty",
          "meta": {
            "updatedAt": "2026-05-23T11:27:41.868Z",
            "barcode": "5784719087687",
            "qrCode": "https://cdn.dummyjson.com/public/qr-code.png",
            "createdAt": "2025-10-09T14:47:01.588Z"
          },
          "availabilityStatus": "In Stock",
          "sku": "BEA-ESS-ESS-001",
          "minimumOrderQuantity": 48,
          "shippingInformation": "Ships in 3-5 business days",
          "id": 1,
          "price": 9.99,
          "tags": [
            "beauty",
            "mascara"
          ],
          "title": "Essence Mascara Lash Princess",
          "dimensions": {
            "depth": 22.99,
            "width": 15.14,
            "height": 13.08
          },
          "warrantyInformation": "1 week warranty",
          "rating": 2.56,
          "discountPercentage": 10.48,
          "reviews": [
            {
              "reviewerEmail": "eleanor.collins@x.dummyjson.com",
              "rating": 3,
              "reviewerName": "Eleanor Collins",
              "date": "2025-04-30T09:41:02.053Z",
              "comment": "Would not recommend!"
            },
            {
              "rating": 4,
              "comment": "Very satisfied!",
              "reviewerName": "Lucas Gordon",
              "reviewerEmail": "lucas.gordon@x.dummyjson.com",
              "date": "2025-04-30T09:41:02.053Z"
            },
            {
              "reviewerEmail": "eleanor.collins@x.dummyjson.com",
              "reviewerName": "Eleanor Collins",
              "date": "2025-04-30T09:41:02.053Z",
              "comment": "Highly impressed!",
              "rating": 5
            }
          ],
          "stock": 99,
          "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
          "weight": 4,
          "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
          "brand": "Essence",
          "returnPolicy": "No return policy",
          "images": [
            "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
          ]
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
Created Alice: $1000 - ID: 5_za_a1k9HaNZ45G_Wq1-VkS1oZufr-C2OXURIThJzo3RMfjdk6gOxNkAsAJEVf1zSCB9oalnwGEQU1eKIessQ
Created Bob: $500 - ID: ScrMTld1ATE_9RX0Dy499OotcApFYIy3ino9nYAiv9tLQFFYXNRGF-TyT5LlBowesNChUQigZhvhRHZ0MPB_IA

=== Example 1: Begin Transaction ===
Transaction ID (server-default isolation): d671d860-7e9d-42a0-98b9-abe30e850923

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
New transaction: 929b70ec-8f77-4a7a-bdf8-de50a1c9d41f
Updated Bob: $700 → $600 (in transaction)
Status before rollback: Active
✓ Transaction rolled back

✓ Verified Bob remains $700 after rollback

=== Cleanup ===
✓ Deleted test account collection

✓ All client transaction examples completed
✓ Client created

=== Create User Function ===
Created user function with ID: 5Ik6wrgvEv_JSJxeeQsa2UAVFa8Tg0KGl3r8YHkIhVnfOuuHgvryZu_-bg7I4_T0icyaGnUDAyIcp2qBNCIFNg

=== Get User Function ===
Retrieved: get_active_users_ts - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 10 user functions:
  - get_active_users_client_js: Get Active Users (Updated)
  - get_active_users_ts: Get Active Users
  - fetch_product_reviews_ts_77743_1791438649451: Fetch Product with Reviews (Multi-API)
  - conc_demo_rl_skip_ts_75441_1791438514383: Rate-limit (skip mode)
  - get_active_users_client_ts_updated: Get Active Users (Updated)
  - conc_demo_rl_fail_ts_77654_1791438637251: Rate-limit (fail mode)
  - conc_demo_rl_skip_ts_77654_1791438637251: Rate-limit (skip mode)
  - conc_demo_rl_fail_ts_75441_1791438514383: Rate-limit (fail mode)
  - get_active_users_client_ts_updated: Get Active Users (Updated)
  - fetch_product_reviews_ts_76477_1791438537065: Fetch Product with Reviews (Multi-API)

=== Update User Function ===
User function updated successfully

=== Delete User Function ===
User function deleted successfully

✓ User Functions API example complete
=== WebSocket Chat Streaming Example (TypeScript) ===

Created chat session: zD0Rr3L9TArQBsaCIJF6LPB9BKGIMPULhzLrDVLeJWl4AqJ7tC2RheL0HxDG1xolsQZbWoftwzQdkyw4fXTIhw

Sending message: 'What is the capital of France?'
{"__progress":"received"}{"__progress":"generating","model":"default","provider":"openai"}The capital of France is Paris.

--- Stream ended ---
Message ID: oQOvHd7mTMt3UhCMx5xGfU5iSxYR_TQVA4q2KZ7kAd-UCkZ6cLuhtUQTGvmq-qmVkTo8B-DzSgW2FaaUPc8ZRg
Execution time: 696ms
Token usage: {"completion_tokens":8,"prompt_tokens":15,"total_tokens":23}

Full response: {"__progress":"received"}{"__progress":"generating","model":"default","provider":"openai"}The capital of France is Paris....
=== WebSocket Subscription Example ===

✓ Authentication successful
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_ts' ===
✓ Subscribed (subscription_id: sub_32b76e21ca5d4c16b83824b4fea856cd)

=== Performing mutations to trigger notifications ===
Inserting a record...
✓ Inserted record: hQfRYEpz4J3Jaldp1uK2WDNRXdaZWmbpSQMJgHbiR7lNtWcxF0F4-l6WD4oYkDWbdJfaqIalY77j341PfHbOxQ
  📡 Notification received for hQfRYEpz4J3Jaldp1uK2WDNRXdaZWmbpSQMJgHbiR7lNtWcxF0F4-l6WD4oYkDWbdJfaqIalY77j341PfHbOxQ

Inserting another record...
✓ Inserted record: Gj0esN83-J6JtWD1ikH-ZPQXajQ7vcokzSr384lE9-wjahUMCVYUVMwp-hmIKOWNIoM1PtGatnB-DecW_IAj2g
  📡 Notification received for Gj0esN83-J6JtWD1ikH-ZPQXajQ7vcokzSr384lE9-wjahUMCVYUVMwp-hmIKOWNIoM1PtGatnB-DecW_IAj2g

=== Unsubscribing ===
✓ Unsubscribed: {"collection":"ws_subscribe_example_ts","found":true,"unsubscribed":true}

✓ WebSocket subscription example completed successfully
✓ Deleted collection 'ws_subscribe_example_ts'
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: z1Lw3XA19PRiI_XxouJUsekXw3NCdpnsHgFRKR1h74gyWdgYpJ1kDKWXE7u8dlcySZWjKR7_R2WMLZlHcWVRfg

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
   Inserted with ripple: {"id":"4lsvTh8yhVGhZE54OlU2l2xQtH9CJdjgBFZmzya6zDLT8uQ0N05BOCsL70EM0wQS0qGk6ozeVj1ohS_KNvEGoQ"}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {"id":"ylshgczaKtY1_GnqTOJ_obpljAbpK0ssOVNtsr0lPKYNEq1p7lLV-uMD6kvyU5Wxrv423bro50gLDnPbVBnUog"}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {"price":{"type":"Integer","value":150},"name":{"type":"String","value":"Product 1"},"id":"4lsvTh8yhVGhZE54OlU2l2xQtH9CJdjgBFZmzya6zDLT8uQ0N05BOCsL70EM0wQS0qGk6ozeVj1ohS_KNvEGoQ"}

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
  Fields returned: ["id","name","email"]
  First user: Alice Johnson <alice@example.com>

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ["id","status","created_at","user_role","age","bio","email","avatar_url","name"]

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
    - Fields: ["api_key","avatar_url","status","age","email","password","name","secret_token","created_at","id","user_role","bio"]
  Projected query:
    - 3 fields per record
    - Fields: ["id","name","email"]
  Bandwidth savings: ~75% fewer fields

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
