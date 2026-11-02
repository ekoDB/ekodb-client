make test-examples-javascript
🧪 Running JavaScript examples (direct HTTP/WebSocket)...

added 1 package, removed 1 package, and audited 9 packages in 479ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
npm warn install-scripts 1 package has install scripts not yet covered by allowScripts:
npm warn install-scripts   ekodb-client-ts@0.31.0 (prepare: npm run build)
npm warn install-scripts
npm warn install-scripts Run `npm install-scripts ls` to review, or `npm install-scripts approve <pkg>` to allow.

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
  id: '6Nl87InT5xuS_O9AMlMWrN9_ISMwinpWaNHLjYHj2BFzRH4iC8Tue2p9UZ_rYS9EF0vh1EvFW6GAG0Nk4yLAqw'
}

=== Find by ID ===
Found: {
  name: { type: 'String', value: 'Test Record' },
  id: '6Nl87InT5xuS_O9AMlMWrN9_ISMwinpWaNHLjYHj2BFzRH4iC8Tue2p9UZ_rYS9EF0vh1EvFW6GAG0Nk4yLAqw',
  active: { value: true, type: 'Boolean' },
  value: { value: 42, type: 'Integer' }
}

=== Find with Query ===
Found documents: [
  {
    active: { type: 'Boolean', value: true },
    id: '6Nl87InT5xuS_O9AMlMWrN9_ISMwinpWaNHLjYHj2BFzRH4iC8Tue2p9UZ_rYS9EF0vh1EvFW6GAG0Nk4yLAqw',
    value: { value: 42, type: 'Integer' },
    name: { value: 'Test Record', type: 'String' }
  }
]

=== Update Document ===
Updated: {
  id: '6Nl87InT5xuS_O9AMlMWrN9_ISMwinpWaNHLjYHj2BFzRH4iC8Tue2p9UZ_rYS9EF0vh1EvFW6GAG0Nk4yLAqw',
  active: { value: true, type: 'Boolean' },
  name: { value: 'Updated Record', type: 'String' },
  value: { type: 'Integer', value: 100 }
}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
✓ simple_crud.js completed successfully

=== Running ekoDB/ekodb-client/examples/javascript/simple_websocket.js ===
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: jNy2s8Ks8Bt2QJ--hAI3yylTOyt5MwxgGLxtpGqMaze7ZnE8u2DJZ43rBbxIzF2PgKnSXgDkFlWxd7MabFteIA

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
        "name": {
          "type": "String",
          "value": "WebSocket Test Record"
        },
        "active": {
          "value": true,
          "type": "Boolean"
        },
        "id": "jNy2s8Ks8Bt2QJ--hAI3yylTOyt5MwxgGLxtpGqMaze7ZnE8u2DJZ43rBbxIzF2PgKnSXgDkFlWxd7MabFteIA"
      }
    ]
  },
  "messageId": "1793594721529"
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

✅ Function saved: w6Yf7HDFlUxUAKGl3HAKd9wtayyw-vFNYeMMHXVJjKLLbz30yi6b6T_iExdka1CmUv7j3zeR9TBs6gA8BNcvSw
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Function saved: txQulunq4KAIq9RIpuAcVSH6kJ1JnMOnsQgM_Io9axzR6jGHTfc8U1kS6tHa-93aAR4WcYMuwQWuVVjlvslxRg
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: GFKHl_GQ9HauwPyxR1FBU4NWUgn88DqfAdD0TG1W89kBS2o92UfTchV0CiEwpdr_loDZH_INQte1pbD2Eb42Qw
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"count":{"type":"Integer","value":5},"avg_score":{"value":60,"type":"Float"},"status":{"type":"String","value":"active"},"max_score":{"value":100,"type":"Integer"}}
   {"count":{"value":5,"type":"Integer"},"max_score":{"type":"Integer","value":90},"avg_score":{"value":50,"type":"Float"},"status":{"type":"String","value":"inactive"}}

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
Collection created with first record: XdNdHTaIMIq8gKk8Eh_zaEpCuDN36RIAVv_vbWiBZOTwIygyKqtnAJmRw2bE6_PKSGJUGomHSE7ofLVVx4Yv_w

=== List Collections ===
Total collections: 22
Sample collections: [
  'schema_users_client_go',
  'audit__ek0_testing',
  'schema_documents_client_ts',
  'schema_employees_client_ts',
  'schema_employees_client_go'
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
Created Alice: $1000 - ID: GcZfZwFc5yzyz3mI1itAnRZse9LWvnXVO3XeYGdyCMJ-fl3xfb3Hp1U91dotK9QVVGWsCJQNvZAWvOTS6Pw_bw
Created Bob: $500 - ID: l1lJJqry1tXDlXP2l9-ECSbLKooZR8QuJAGjcrm9vK0TDZca5I-_2X0xz3GjVNSm3Pck2WuxbCO9BTschQu15w

=== Example 1: Begin Transaction ===
Transaction ID: 76df91b1-3249-42c2-80f7-d098606bcb5c

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
New transaction: 0cd059bc-0038-4550-b826-21b850aa674e
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
   ✅ Function saved: y-LsU3R0yDCG2Y0GprGuKl_8Cw2faOcnm54HAEDws0JzeItPoyZwlUWSYFjXAfLoxxGQTzPK3UsMvCA3NXMJDw

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
   ✅ Function saved: dtFGqmbgOFTLaZiVfMZST-LQxIAF1Pq78wzRwq5fcCuSXI4hKlCKZuvGsZVMk94bkXhAB7y9tvxd4VKcHVoh1A

2️⃣ Calling Function (Query + Update + Verify)...
   ✅ Function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Status updated to: {"value":"active","type":"String"}
   📋 Name: {"value":"Alice Smith","type":"String"}

============================================================
📝 Function 3: Query + Update Credits + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: jpgssmfItfaMqyCdY736LhuwlrXqYao6XRVIz9Mrunb99DDbHrQ_LiEMLByZscFZA8U5F34hkP2zJOmv77CjIw

2️⃣ Calling Function (Query + Update Credits + Verify)...
   ✅ Function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {"type":"Integer","value":100}
   📋 Status: {"value":"active","type":"String"}
   📋 Name: {"type":"String","value":"Alice Smith"}

============================================================
📝 Function 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: y2hjrKaJjW9eHsFS_cNU88JtMOQPs4FKSYDtSRbox5G3-afZlW5vmbTKGbFEHWu5jZ-NTN8l3WxKB9HQpQoUqQ

2️⃣ Calling Function (Query + Delete + Verify)...
   ✅ Function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted function: y-LsU3R0yDCG2Y0GprGu...
   ✅ Deleted function: dtFGqmbgOFTLaZiVfMZS...
   ✅ Deleted function: jpgssmfItfaMqyCdY736...
   ✅ Deleted function: y2hjrKaJjW9eHsFS_cNU...
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
  Output: Document ID = XcxWAYWWNPYNl_st1kgxf6j4OVu1yNdlZU5zoqFiqxeypDN-Ocr1EP9vsDkPPpeimUEMRWBqiuYF2Bo79JCoRA
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: findById(XcxWAYWWNPYNl_st1kgxf6j4OVu1yNdlZU5zoqFiqxeypDN-Ocr1EP9vsDkPPpeimUEMRWBqiuYF2Bo79JCoRA)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: findById(XcxWAYWWNPYNl_st1kgxf6j4OVu1yNdlZU5zoqFiqxeypDN-Ocr1EP9vsDkPPpeimUEMRWBqiuYF2Bo79JCoRA)
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
  Output: Document ID = 0tHOg0QM-TuUbYiYS24pPhCWD9MlFSuVY9CZJ39biyqwLa2N4NjlCWNXt86Iq0UHWy0MiAJzjb3YvPhzJYQR8A
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: findById(0tHOg0QM-TuUbYiYS24pPhCWD9MlFSuVY9CZJ39biyqwLa2N4NjlCWNXt86Iq0UHWy0MiAJzjb3YvPhzJYQR8A)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: findById(0tHOg0QM-TuUbYiYS24pPhCWD9MlFSuVY9CZJ39biyqwLa2N4NjlCWNXt86Iq0UHWy0MiAJzjb3YvPhzJYQR8A)
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

> @ekodb/ekodb-client@0.31.0 prepare
> npm run build


> @ekodb/ekodb-client@0.31.0 build
> tsc


up to date, audited 44 packages in 596ms

12 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
npm warn install-scripts 1 package has install scripts not yet covered by allowScripts:
npm warn install-scripts   fsevents@2.3.3 (install: (install scripts present))
npm warn install-scripts
npm warn install-scripts Run `npm install-scripts ls` to review, or `npm install-scripts approve <pkg>` to allow.

> @ekodb/ekodb-client@0.31.0 build
> tsc

✅ TypeScript client built!

added 1 package, removed 1 package, and audited 9 packages in 417ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
npm warn install-scripts 1 package has install scripts not yet covered by allowScripts:
npm warn install-scripts   ekodb-client-ts@0.31.0 (prepare: npm run build)
npm warn install-scripts
npm warn install-scripts Run `npm install-scripts ls` to review, or `npm install-scripts approve <pkg>` to allow.

added 1 package, removed 1 package, and audited 13 packages in 428ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
npm warn install-scripts 3 packages have install scripts not yet covered by allowScripts:
npm warn install-scripts   ekodb-client-ts@0.31.0 (prepare: npm run build)
npm warn install-scripts   esbuild@0.28.1 (postinstall: node install.js)
npm warn install-scripts   fsevents@2.3.3 (install: (install scripts present))
npm warn install-scripts
npm warn install-scripts Run `npm install-scripts ls` to review, or `npm install-scripts approve <pkg>` to allow.

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
✓ Created session: jz9QPY6BLrmY_lgPCThGQt6JHTeu1asDQKWWPFLpSn4BsQpu3yIvjewGwHsKy5wi62aclcBMiju0E8_NORwk0A

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is **ekoDB**, a high-performance database product, priced at **$99**.

✓ Second message sent

=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: ekoDB costs **99**. The currency isn’t specified.

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: owaftWBhJb3vgtXqGTkcSZ6294kRpbD7lIyMtEoqv1bFlxEPauvhf30p1m8o5-oQPqSQOvLVEpmL7szteZv9lA
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
✓ Created session: hvzIVyja9ggFktxel9TixkIu-xW_aGIq-tu0TjP5jGgx_853aHAaQ88Nk1N96L41MP5a6VeAPMgeU-jvUAhhwg

=== Sending Chat Message ===
Message ID: mcQ30hwpuU3tlvN_04R5FmvCUF51W7A88KVoLBJITjxt8bNMJMsPXzarIyrxitbNSiNIuBhjp67fqUGC6_Pp4w

=== AI Response ===
Available products and prices:

- ekoDB — $99
- ekoDB Pro — $299
- ekoDB Cloud — $499

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_js',
  record: {
    id: 'Y_p4nwxrjfiWIn-d7cWneyldaaRtg7pkUCrtk6WCUBPJq-F8ieHu2j0XpJU0c1ytZ3Bjt3LevN9Rv_jT_IzpMg',
    price: 299,
    name: 'ekoDB Pro',
    description: 'Enterprise edition product with advanced features'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_js',
  record: {
    description: 'A high-performance database product with AI capabilities',
    price: 99,
    name: 'ekoDB',
    id: 'Yi1IDugORLhETtn7Dzlg59xzeHi5jaqXWG3tqRVR_h3C2-LJM76Cv5xYfJ1jom8b5orp2WPn-f0ftQu2YlNqTQ'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_js',
  record: {
    description: 'Fully managed cloud database service product',
    id: 'y2gK3bP5qsbvRUXjZackdNKH7iUYTOXf7JTk8e9MSx5fWWKOen4bfCBQ-YZ62Y9fV3Mu3pC38MnRoM7CaSPZSA',
    price: 499,
    name: 'ekoDB Cloud'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 1990ms

=== Token Usage ===
Prompt tokens: 13714
Completion tokens: 34
Total tokens: 13748

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
    - gpt-5.5-pro
    - gpt-audio-2025-08-28
    - gpt-4o-mini-transcribe-2025-03-20
    - gpt-4-turbo-2024-04-09
    - gpt-realtime-2.1-mini
    - gpt-realtime-2025-08-28
    - gpt-4
    - tts-1-hd
    - gpt-realtime-1.5
    - gpt-realtime
    - gpt-5.2-pro
    - gpt-5-chat-latest
    - gpt-5.4-mini-2026-03-17
    - babbage-002
    - gpt-4o-2024-05-13
    - text-embedding-3-large
    - tts-1
    - omni-moderation-2024-09-26
    - gpt-live-transcribe
    - gpt-5.6-luna
    - gpt-5-2025-08-07
    - gpt-5-mini
    - gpt-5-mini-2025-08-07
    - omni-moderation-latest
    - o3-mini-2025-01-31
    - gpt-audio-1.5
    - gpt-image-1.5
    - tts-1-1106
    - gpt-5.5-pro-2026-04-23
    - gpt-audio-mini-2025-10-06
    - gpt-5.2-pro-2025-12-11
    - o4-mini-2025-04-16
    - gpt-4o-2024-11-20
    - gpt-5.1
    - whisper-1
    - gpt-5-pro
    - gpt-realtime-whisper
    - gpt-realtime-2
    - gpt-5-search-api
    - gpt-5.5-2026-04-23
    - gpt-4.1-nano
    - gpt-image-2
    - gpt-3.5-turbo-16k
    - text-embedding-ada-002
    - gpt-realtime-2.1
    - gpt-4.1
    - gpt-5.5
    - chat-latest
    - gpt-realtime-translate
    - gpt-5.4-pro-2026-03-05
    - gpt-4.1-mini
    - gpt-4o-mini
    - gpt-4o-mini-transcribe-2025-12-15
    - gpt-realtime-mini
    - gpt-image-1-mini
    - gpt-5.6-terra
    - gpt-3.5-turbo-instruct-0914
    - gpt-4-turbo
    - gpt-4o-2024-08-06
    - o3
    - gpt-4o-mini-search-preview-2025-03-11
    - o1-2024-12-17
    - gpt-5.1-chat-latest
    - gpt-5-pro-2025-10-06
    - gpt-5.6-sol
    - gpt-4o-transcribe-diarize
    - gpt-5-codex
    - gpt-4-0613
    - gpt-5.1-2025-11-13
    - gpt-4o-search-preview
    - gpt-4o-mini-tts-2025-12-15
    - gpt-image-1
    - gpt-audio
    - gpt-5.3-codex
    - gpt-5-search-api-2025-10-14
    - gpt-5.4-pro
    - gpt-4o-mini-tts
    - gpt-realtime-mini-2025-12-15
    - gpt-5.1-codex-max
    - sora-2
    - gpt-4o
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
    - gpt-3.5-turbo-1106
    - gpt-4o-mini-tts-2025-03-20
    - gpt-5.2
    - gpt-5.4-nano-2026-03-17
    - o1
    - tts-1-hd-1106
    - gpt-5.2-codex
    - gpt-5.4-mini
    - o1-pro
    - gpt-5.1-codex
    - o4-mini-deep-research
    - gpt-5.4
    - gpt-4o-search-preview-2025-03-11
    - text-embedding-3-small
    - gpt-5-nano-2025-08-07
    - gpt-4o-transcribe
    - o4-mini
    - gpt-5.1-codex-mini
    - gpt-6-astra
    - gpt-5.3-chat-latest
    - o3-2025-04-16
    - gpt-5.2-2025-12-11
    - chatgpt-image-latest
    - gpt-3.5-turbo
    - gpt-image-2.5-sunburst-2026-09-08
    - gpt-image-2.5-flare
    - davinci-002
    - gpt-5.4-nano
    - o3-mini
    - o4-mini-deep-research-2025-06-26
    - gpt-audio-mini-2025-12-15
    - o1-pro-2025-03-19
    - gpt-live-1
    - gpt-image-2.5-sunburst
    - gpt-image-2.5-flare-2026-09-08
    - gpt-6.1-terra
    - gpt-audio-mini
    - gpt-4.1-mini-2025-04-14
    - gpt-6-luna
    - gpt-4o-mini-search-preview
    - gpt-6-sol
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
  gemini: ok 60 models
  openai: ok 136 models
  perplexity: not_configured (unverified) No Perplexity API Key

=== Get Specific Provider Models ===
OpenAI models: gpt-3.5-turbo-0125, gpt-5.4-2026-03-05, gpt-5-nano, gpt-5.5-pro, gpt-audio-2025-08-28, gpt-4o-mini-transcribe-2025-03-20, gpt-4-turbo-2024-04-09, gpt-realtime-2.1-mini, gpt-realtime-2025-08-28, gpt-4, tts-1-hd, gpt-realtime-1.5, gpt-realtime, gpt-5.2-pro, gpt-5-chat-latest, gpt-5.4-mini-2026-03-17, babbage-002, gpt-4o-2024-05-13, text-embedding-3-large, tts-1, omni-moderation-2024-09-26, gpt-live-transcribe, gpt-5.6-luna, gpt-5-2025-08-07, gpt-5-mini, gpt-5-mini-2025-08-07, omni-moderation-latest, o3-mini-2025-01-31, gpt-audio-1.5, gpt-image-1.5, tts-1-1106, gpt-5.5-pro-2026-04-23, gpt-audio-mini-2025-10-06, gpt-5.2-pro-2025-12-11, o4-mini-2025-04-16, gpt-4o-2024-11-20, gpt-5.1, whisper-1, gpt-5-pro, gpt-realtime-whisper, gpt-realtime-2, gpt-5-search-api, gpt-5.5-2026-04-23, gpt-4.1-nano, gpt-image-2, gpt-3.5-turbo-16k, text-embedding-ada-002, gpt-realtime-2.1, gpt-4.1, gpt-5.5, chat-latest, gpt-realtime-translate, gpt-5.4-pro-2026-03-05, gpt-4.1-mini, gpt-4o-mini, gpt-4o-mini-transcribe-2025-12-15, gpt-realtime-mini, gpt-image-1-mini, gpt-5.6-terra, gpt-3.5-turbo-instruct-0914, gpt-4-turbo, gpt-4o-2024-08-06, o3, gpt-4o-mini-search-preview-2025-03-11, o1-2024-12-17, gpt-5.1-chat-latest, gpt-5-pro-2025-10-06, gpt-5.6-sol, gpt-4o-transcribe-diarize, gpt-5-codex, gpt-4-0613, gpt-5.1-2025-11-13, gpt-4o-search-preview, gpt-4o-mini-tts-2025-12-15, gpt-image-1, gpt-audio, gpt-5.3-codex, gpt-5-search-api-2025-10-14, gpt-5.4-pro, gpt-4o-mini-tts, gpt-realtime-mini-2025-12-15, gpt-5.1-codex-max, sora-2, gpt-4o, gpt-4o-mini-2024-07-18, gpt-4.1-2025-04-14, gpt-5, gpt-4o-mini-transcribe, gpt-image-2-2026-04-21, gpt-5.2-chat-latest, sora-2-pro, gpt-3.5-turbo-instruct, gpt-transcribe, gpt-4.1-nano-2025-04-14, gpt-3.5-turbo-1106, gpt-4o-mini-tts-2025-03-20, gpt-5.2, gpt-5.4-nano-2026-03-17, o1, tts-1-hd-1106, gpt-5.2-codex, gpt-5.4-mini, o1-pro, gpt-5.1-codex, o4-mini-deep-research, gpt-5.4, gpt-4o-search-preview-2025-03-11, text-embedding-3-small, gpt-5-nano-2025-08-07, gpt-4o-transcribe, o4-mini, gpt-5.1-codex-mini, gpt-6-astra, gpt-5.3-chat-latest, o3-2025-04-16, gpt-5.2-2025-12-11, chatgpt-image-latest, gpt-3.5-turbo, gpt-image-2.5-sunburst-2026-09-08, gpt-image-2.5-flare, davinci-002, gpt-5.4-nano, o3-mini, o4-mini-deep-research-2025-06-26, gpt-audio-mini-2025-12-15, o1-pro-2025-03-19, gpt-live-1, gpt-image-2.5-sunburst, gpt-image-2.5-flare-2026-09-08, gpt-6.1-terra, gpt-audio-mini, gpt-4.1-mini-2025-04-14, gpt-6-luna, gpt-4o-mini-search-preview, gpt-6-sol, gpt-6.1-sol

=== Get Anthropic Models ===
Anthropic models: claude-haiku-5-5, claude-sonnet-5-5, claude-opus-5-5, claude-fable-5-1, claude-opus-5, claude-sonnet-5, claude-fable-5, claude-opus-4-8, claude-opus-4-7, claude-sonnet-4-6, claude-opus-4-6, claude-opus-4-5-20251101, claude-haiku-4-5-20251001, claude-sonnet-4-5-20250929

=== Get Non-Existent Provider ===
Expected error for non-existent provider: Error: Request failed with status 404: {"error":"Unknown provider 'nonexistent_provider_xyz'","error_kind":"unknown_provider"}

✓ Chat Models example complete
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: SP5n20tjsam_1Ba6qoMynmqwtRu2-uzvCCEKEdPBb9sKV7mRJf8qC3pcDkdwELQtXPTEaKkRKSwh4iZJSMkkBA

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is **ekoDB**, described as a high-performance database product. Its listed price is **$99**.

✓ Message 2 sent
  Response: ekoDB is priced at **$99**.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: N1NWqQbHBe2qBIPUnHGit_SndMn-6Lh4DbwEMIlffryFH5e_f-4wKqnUOyv1Ti8nt2mSDEQKpi27k_U8t0-pPA
  Parent: SP5n20tjsam_1Ba6qoMynmqwtRu2-uzvCCEKEdPBb9sKV7mRJf8qC3pcDkdwELQtXPTEaKkRKSwh4iZJSMkkBA

=== Listing Sessions ===
✓ Found 2 sessions
  Session 1: N1NWqQbHBe2qBIPUnHGit_SndMn-6Lh4DbwEMIlffryFH5e_f-4wKqnUOyv1Ti8nt2mSDEQKpi27k_U8t0-pPA (Untitled)
  Session 2: SP5n20tjsam_1Ba6qoMynmqwtRu2-uzvCCEKEdPBb9sKV7mRJf8qC3pcDkdwELQtXPTEaKkRKSwh4iZJSMkkBA (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: N1NWqQbHBe2qBIPUnHGit_SndMn-6Lh4DbwEMIlffryFH5e_f-4wKqnUOyv1Ti8nt2mSDEQKpi27k_U8t0-pPA

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ All session management operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: hzoi3T7T0T6jTzfoPgUlWGxZi1DPWfDTvDvhrXCQGmI0w8r7IyMu5QZGOilasnrIyxftouxvPnRytUKJ_eo_lA

=== List Collections ===
Total collections: 23
Sample collections: schema_users_client_go,audit__ek0_testing,schema_documents_client_ts,schema_employees_client_ts,schema_employees_client_go

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: GpbIz6IQo9e_QI1NhqdGGwtMcGHcahLnmsxj-oFPgeWErUopqESGO-nZR1gTFz_32v2TyteU4VmQL2CfZJoYkA

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: QhwwkTRcaQmgGcBqCHdn-UnWg6BGesNCf4twNn9n4ILcCyxZKuYDa0GIUUZ5Um6FS_fbYq6DiSuCZl2BlK4zDg

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
✓ Edge cache script created: 4tHCebwI09JC0f6HZk3FjrX8HAmGucM-GA4T8Cb1LqaeHh9zqMJQHchu9xb4_GpNcP-4AK-Cs4sSaLSJW4n3lA

Call 1: Cache miss (fetches from API)
Response time: 165ms
Result: {
  "records": [
    {
      "value": {
        "value": {
          "generationtime_ms": 0.033020973205566406,
          "latitude": 40.710335,
          "timezone": "GMT",
          "timezone_abbreviation": "GMT",
          "current": {
            "time": "2026-11-02T04:45",
            "interval": 900,
            "temperature_2m": 8.4
          },
          "current_units": {
            "temperature_2m": "°C",
            "interval": "seconds",
            "time": "iso8601"
          },
          "elevation": 32,
          "longitude": -73.99308,
          "utc_offset_seconds": 0
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
Response time: 4ms (41.3x faster!)
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "generationtime_ms": 0.033020973205566406,
          "latitude": 40.710335,
          "timezone": "GMT",
          "timezone_abbreviation": "GMT",
          "current": {
            "time": "2026-11-02T04:45",
            "interval": 900,
            "temperature_2m": 8.4
          },
          "current_units": {
            "temperature_2m": "°C",
            "interval": "seconds",
            "time": "iso8601"
          },
          "elevation": 32,
          "longitude": -73.99308,
          "utc_offset_seconds": 0
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
   ⏱️  Duration: 76ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "id": 1,
      "email": "Sincere@april.biz",
      "website": "hildegard.org",
      "name": "Leanne Graham",
      "username": "Bret",
      ...

Second call (cache hit - from cache):
   ⏱️  Duration: 2ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "id": 1,
      "email": "Sincere@april.biz",
      "website": "hildegard.org",
      "name": "Leanne Graham",
      "username": "Bret",
      "address": {
        "...
   🚀 Cache speedup: 38.0x faster!

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

✅ Function saved: KXAiHTiUxBuvnAdctdezHQQdx5ilDOysMg6KzqIoM619SOQLOLtcg-RptDRtVT-xYaEE6PqC2Msf7viMu1KkSw
📊 Found 5 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Function

✅ Function saved
📊 Found 3 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Function

✅ Function saved
📊 Statistics: 2 groups
   {"status":{"value":"inactive","type":"String"},"count":{"value":5,"type":"Integer"},"avg_score":{"type":"Float","value":50}}
   {"count":{"type":"Integer","value":5},"status":{"type":"String","value":"active"},"avg_score":{"type":"Float","value":60}}
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
   Furniture: 4 items (avg $294.00)
   Electronics: 6 items (avg $325.67)
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
   1. Bookshelf - $149 (⭐4.1)
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
   Vector databases are designed to store and search **embeddings**—numeric representations of text, images, audio, or other data. Their main benefits include:

- **Semantic search:** Find items with similar meaning or content, even when they use different wording.
- **Fast similarity search at scale:** Specialized indexes make nearest-neighbor searches practical across large collections.
- **Support for AI applications:** They provide a foundation for recommendation systems, image and document search, and retrieval-augmented generation (RAG).
- **Multimodal search:** Compare embeddings from different media types, such as matching a text query to images.
- **Filtering and hybrid search:** Combine vector similarity with metadata filters or keyword search to improve relevance.
- **Production-ready operations:** Many offer persistence, indexing, updates, and distributed scaling for applications.

They complement rather than replace relational databases: use a vector database when similarity search is central, and a conventional database for structured data, transactions, and exact queries.
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embedding function saved
📊 Generated embeddings for 3 articles
   1. "Draft Article" - 1536D vector
   2. "Advanced Query Patterns" - 1536D vector
   3. "Getting Started with ekoDB" - 1536D vector
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

✅ Function saved: n8W3X0ZC19P-DF0IBHW3_94_Zi3H0OntNb4sv3IIGu2m_97VSfExuaNrzl9FooFXFMIGOLEImd-HU9lnLwJbZg
📊 Found 2 product groups
   {"count":{"type":"Integer","value":2},"category":{"value":"Furniture","type":"String"},"avg_price":{"value":474,"type":"Float"}}
   {"avg_price":{"value":575.6666666666666,"type":"Float"},"category":{"value":"Electronics","type":"String"},"count":{"value":3,"type":"Integer"}}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   {"category":{"type":"String","value":"Furniture"},"count":{"type":"Integer","value":2}}
   {"count":{"type":"Integer","value":3},"category":{"type":"String","value":"Electronics"}}
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
   2. User 1 - Score: 10
   3. User 5 - Score: 50
   4. User 9 - Score: 90
   5. User 3 - Score: 30
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

✅ Inserted order: dYmoVH-N4fVnScpEArrizpY0QO_WWlltM4TlStRnhw3SuXXUVbPoiIOLFYVccN7ocJSVsiWJz0Vps9PTlDAiyA
✅ Inserted 2 products with wrapped types

📝 Example 2: Function with Wrapped Type Parameters

✅ Function saved: PmeotJbySJXvUL3n7HpQ1b6ddNq6TLS-cNSVyWp7GY8VFVEbv8yAt_iQfftrho952nM6C0Umm55FUrgSYtLrbg
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"value":{"userId":"user_abc","role":"admin"},"type":"Object"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: H0faZwkr0r_ph5u7RRPYF9PItRx9qoq7ygHTHJdk31rbQU_KXl8mnaSN6z88UPlU7kvtcbXCTblJTRfp868uhw
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 3 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Function

✅ Function saved: Zo0gIGWsoE-dhwQqLCSRVoIWzzPTSEtLjirMpkDUwK6JMLfhsmP1FntbkTOtL8hEs94p1o1KTtiFrk1ECV9Ibg
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
   1. Getting Started with ekoDB (Database)
   2. Database Design Principles (Database)
   3. Vector Databases Explained (Database)
   4. Natural Language Processing (AI)
   5. Introduction to Machine Learning (AI)
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
   1. Getting Started with ekoDB
   2. Database Design Principles
   3. Vector Databases Explained
   4. Natural Language Processing
   5. Introduction to Machine Learning
⏱️  Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Function saved
📊 Projected documents (showing first 3):
   1. Getting Started with ekoDB
   2. Database Design Principles
   3. Vector Databases Explained
⏱️  Execution time: 0ms

📝 Example 5: All Document Fields

✅ Function saved
📊 All documents (5 total, showing first 2):
   1. Getting Started with ekoDB (Database)
   2. Database Design Principles (Database)
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
Retrieved value: { value: { userId: 123, username: 'john_doe' }, type: 'Object' }

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: { value: { price: 29.99, name: 'Product 1' }, type: 'Object' }
cache:product:2: {
  type: 'Object',
  value: { name: 'Product 2', price: 39.989999999999995 }
}
cache:product:3: {
  value: { name: 'Product 3', price: 49.989999999999995 },
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
  1. Score: 13.200, Matched: bio, title
  2. Score: 13.200, Matched: bio, title
  3. Score: 13.200, Matched: bio, title
  4. Score: 13.200, Matched: bio, title

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 26.400, Matched: title, bio
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
  2. Score: 0.741, Matched:
  3. Score: 0.739, Matched:

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.496, Matched: content, title
  2. Score: 0.896, Matched: content, title
  3. Score: 0.298, Matched:

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 13.200, Matched: skills, bio

=== Cleanup ===
✅ Deleted test collections

✅ Search examples completed!
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'eEOKE27vPhELGoDsG6X9-7JcKAnisWkBEdNATe6SJx94ShljTUUPN7omoOnRq7Hp0J4CYdYy98YqSIuDPs3gvw'
}

=== Find by ID ===
Found: {
  name: { value: 'Test Record', type: 'String' },
  id: 'eEOKE27vPhELGoDsG6X9-7JcKAnisWkBEdNATe6SJx94ShljTUUPN7omoOnRq7Hp0J4CYdYy98YqSIuDPs3gvw',
  active: { type: 'Boolean', value: true },
  value: { value: 42, type: 'Integer' }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  active: { type: 'Boolean', value: true },
  name: { value: 'Updated Record', type: 'String' },
  id: 'eEOKE27vPhELGoDsG6X9-7JcKAnisWkBEdNATe6SJx94ShljTUUPN7omoOnRq7Hp0J4CYdYy98YqSIuDPs3gvw',
  value: { value: 100, type: 'Integer' }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: ImPWUT9whE-xY49Uf8Hoqhz7dD4kxSVxmzu6vPK78jdUUv5K_QBVEBwpcsp8fUtaqQ5B_2EPYjlRY3XiM_XPpg

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
✓ Created native SWR script: github_user_native_js (25oeZPFC1PCyKp2BZl2nWLj9jsvXVrCzT6MHeD0ku21-pYkRi2wt_JAccpcRNdgzf36L0ZZQEQCJ2CSDlU_mJA)

First call (cache miss - will fetch from GitHub API):
  Response time: 125ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 3ms
  Speedup: 41.7x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit_js (Y7GZCu5ff154RdqsezFrj1da34RI1kxq-rucxtqi8lecPbiRegg-KMoxxmhyxxDhLkD6KZwoRHOes8c0g6KaIQ)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail_js' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline_js (VUicGH4RnDOyLyHXDwatRNV--brFdunOKGf6IcEGyGQ8mDNtLW6RBrUzmgRH-Hwi5mk_S-LHFv-HHBS1VKxzqA)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users_swr_js' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache_js (RV17JEDpOhuDf1kqNjubO1Xkb82MHSQN5Zy59Cbe2yZK7k-GKw_Hh8l0kRiCE0ukO4E4Ci3QroroXv-tP_EoTQ)
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
✓ Created SWR script: fetch_api_user_js_70662_1793594795441 (kNl2d-Kcdn15dDRjVkxk-zCcD9yAeVJ_8Q1ckQm0TL-Gsv_ZWmQ6pVGkJGbNZsZXDC4_ZddqpSuozEKxkhHbig)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "address": {
            "suite": "Apt. 556",
            "city": "Gwenborough",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "zipcode": "92998-3874",
            "street": "Kulas Light"
          },
          "email": "Sincere@april.biz",
          "name": "Leanne Graham",
          "website": "hildegard.org",
          "username": "Bret",
          "company": {
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net"
          },
          "phone": "1-770-736-8031 x56442",
          "id": 1
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
Response time: 5ms (served from cache)
✓ Lightning fast cache hit

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation
🧹 Cleaning up...
✓ Cleanup complete

✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: d0Jhm67xK0UHRh7Xy1KRaugM4ZWwCuYwP690979EnXUBKJrWpPNmeh0VIP8lOrE130dUEOfGPrQ2WlWyLwO3oA
Created Bob: $500 - ID: xN6CsFzr9B-eMZ1k5ulVhZ-II4ISj-7pCrasqbdRWBHl1pj9MvTVTrjT7ElfO-hoHzs4tXwWfFm_GSnhtEsRwg

=== Example 1: Begin Transaction ===
Transaction ID (server-default isolation): 5c51a5c7-20f4-46b0-a1c3-d03786a4ece3

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
New transaction: 485985e8-1e04-4253-8782-81741b27e545
Updated Bob: $700 → $600 (in transaction)
Status before rollback: Active
✓ Transaction rolled back

✓ Verified Bob remains $700 after rollback

=== Cleanup ===
✓ Deleted test account collection

✓ All client transaction examples completed
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: 2TGW6ti0lU5pcbNAzWdDgStnTYouuLLpzrwZ8zmtJp1Tpcc4bmGFdJ_MfvpM-qY_ZXCqx4NdmCRf70lRsrfWbw

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
Admin permissions read successfully
=== ekoDB Advanced CRUD Example (TypeScript) ===

--- Inserting base record ---
Inserted: KIHSjU9_0XJPaTNuGdz95PqvcKlN0iMpyMpXUf1lp9nPDgPuFk1dJCjD2NW3KIeLROKqEAjcAjWGl8WeumU5Tg

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
✓ Created session: ck8F-npLlDLaTH5LoxgN7rzWiNwl-pet2BKBoennreGFI5FxLqYoSYaFTqGgyJfWEXwQaljwvUyq0m7tYnpWbw

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is **ekoDB** — a high-performance database product priced at **$99**.

✓ Second message sent

=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: ekoDB is priced at **$99**.

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: Tx0nLywYAGogDZLzfvhRCfFxT_fodrCBXKB8w35YEEodAhMQhh7X-mch0Fw5xrbKJ4uImgTLmjjC2V95ARkOcg
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 7

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 6

=== Cleanup ===
✓ Deleted chat session: Tx0nLywYAGogDZLzfvhRCfFxT_fodrCBXKB8w35YEEodAhMQhh7X-mch0Fw5xrbKJ4uImgTLmjjC2V95ARkOcg
✓ Deleted chat session: ck8F-npLlDLaTH5LoxgN7rzWiNwl-pet2BKBoennreGFI5FxLqYoSYaFTqGgyJfWEXwQaljwvUyq0m7tYnpWbw
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: NQD8V_zjQ2jKyJkMNrfNYxYhwl9MIyM_n2gCMlnUQWCA_bJyoBvbiJC40PlMszfplyhY1MsZ-YHQIaU8Qn94ew

=== Sending Chat Message ===
Message ID: U_FGNHREy3YI02_8px32QfLFrCzuEzvkq7nmuGtqxyRb2sfGh_DtNSXOFPm2PN5lcuCDD8v6prqmicYuo7JIdw

=== AI Response ===
- **ekoDB Cloud** — 499
- **ekoDB Pro** — 299
- **ekoDB** — 99

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    price: 499,
    description: 'Fully managed cloud database service product',
    id: 'DPpsBiL3gylnWTztRIIDvSZHx7baEXKw3xCI65jzUhiJqXrMfGHTj4gbvOcn0sN0H656Vb_rO1fRzcZbzRzn8g',
    name: 'ekoDB Cloud'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    name: 'ekoDB Pro',
    price: 299,
    description: 'Enterprise edition product with advanced features',
    id: 'IK8hP_eYedD5WT5sFVTZq85EOM0TkoQ98dbNGXBw9cVEwXXcn6snWFtXZ__FjQSfPP0z35DsjNOZ3_yXno69NA'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    id: 'p5PcwtKR6x4NXLMl1TVO0qZEPudWIE74N7pRHy0yENM1nQnviEZQaEYbEAmPsHSq2C3_YFN1MF1v0bqzRkrH0g',
    price: 99,
    name: 'ekoDB',
    description: 'A high-performance database product with AI capabilities'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 1678ms

=== Token Usage ===
Prompt tokens: 13716
Completion tokens: 32
Total tokens: 13748

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Message Stream (SSE) Example (TypeScript) ===

Created session: xFKV2VgP6BbI9PiFnzJka9Z8qYeSzlSnmhFqST_em7wZGsDLPeOucLjpGzOs40W_noyD930ohMF1LGByCoYB1g

Streaming response for: 'What is ekoDB?'

{"__progress":"received"}{"__progress":"generating","model":"gpt-6-luna","provider":"openai"}“ekoDB” isn’t a standard database category, and the name may refer to different products or projects. If you share where you saw it—a link, app, or article—I can explain that specific ekoDB.

--- Stream complete ---
Message ID: RafUui2O2cyM2vYANaeP4N-gJ5AR6HgXenzk2eVUo_vP5Kh5CabtxhUF1g_TUSFDNabYhv8VanNBeWM0eyTmxQ
Execution time: 13294ms
Context window: 128000 tokens

✓ Chat message stream example completed
✓ Client created

=== List Chat Models ===
Available chat models by provider:
  openai:
    - gpt-3.5-turbo-0125
    - gpt-5.4-2026-03-05
    - gpt-5-nano
    - gpt-5.5-pro
    - gpt-audio-2025-08-28
    - gpt-4o-mini-transcribe-2025-03-20
    - gpt-4-turbo-2024-04-09
    - gpt-realtime-2.1-mini
    - gpt-realtime-2025-08-28
    - gpt-4
    - tts-1-hd
    - gpt-realtime-1.5
    - gpt-realtime
    - gpt-5.2-pro
    - gpt-5-chat-latest
    - gpt-5.4-mini-2026-03-17
    - babbage-002
    - gpt-4o-2024-05-13
    - text-embedding-3-large
    - tts-1
    - omni-moderation-2024-09-26
    - gpt-live-transcribe
    - gpt-5.6-luna
    - gpt-5-2025-08-07
    - gpt-5-mini
    - gpt-5-mini-2025-08-07
    - omni-moderation-latest
    - o3-mini-2025-01-31
    - gpt-audio-1.5
    - gpt-image-1.5
    - tts-1-1106
    - gpt-5.5-pro-2026-04-23
    - gpt-audio-mini-2025-10-06
    - gpt-5.2-pro-2025-12-11
    - o4-mini-2025-04-16
    - gpt-4o-2024-11-20
    - gpt-5.1
    - whisper-1
    - gpt-5-pro
    - gpt-realtime-whisper
    - gpt-realtime-2
    - gpt-5-search-api
    - gpt-5.5-2026-04-23
    - gpt-4.1-nano
    - gpt-image-2
    - gpt-3.5-turbo-16k
    - text-embedding-ada-002
    - gpt-realtime-2.1
    - gpt-4.1
    - gpt-5.5
    - chat-latest
    - gpt-realtime-translate
    - gpt-5.4-pro-2026-03-05
    - gpt-4.1-mini
    - gpt-4o-mini
    - gpt-4o-mini-transcribe-2025-12-15
    - gpt-realtime-mini
    - gpt-image-1-mini
    - gpt-5.6-terra
    - gpt-3.5-turbo-instruct-0914
    - gpt-4-turbo
    - gpt-4o-2024-08-06
    - o3
    - gpt-4o-mini-search-preview-2025-03-11
    - o1-2024-12-17
    - gpt-5.1-chat-latest
    - gpt-5-pro-2025-10-06
    - gpt-5.6-sol
    - gpt-4o-transcribe-diarize
    - gpt-5-codex
    - gpt-4-0613
    - gpt-5.1-2025-11-13
    - gpt-4o-search-preview
    - gpt-4o-mini-tts-2025-12-15
    - gpt-image-1
    - gpt-audio
    - gpt-5.3-codex
    - gpt-5-search-api-2025-10-14
    - gpt-5.4-pro
    - gpt-4o-mini-tts
    - gpt-realtime-mini-2025-12-15
    - gpt-5.1-codex-max
    - sora-2
    - gpt-4o
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
    - gpt-3.5-turbo-1106
    - gpt-4o-mini-tts-2025-03-20
    - gpt-5.2
    - gpt-5.4-nano-2026-03-17
    - o1
    - tts-1-hd-1106
    - gpt-5.2-codex
    - gpt-5.4-mini
    - o1-pro
    - gpt-5.1-codex
    - o4-mini-deep-research
    - gpt-5.4
    - gpt-4o-search-preview-2025-03-11
    - text-embedding-3-small
    - gpt-5-nano-2025-08-07
    - gpt-4o-transcribe
    - o4-mini
    - gpt-5.1-codex-mini
    - gpt-6-astra
    - gpt-5.3-chat-latest
    - o3-2025-04-16
    - gpt-5.2-2025-12-11
    - chatgpt-image-latest
    - gpt-3.5-turbo
    - gpt-image-2.5-sunburst-2026-09-08
    - gpt-image-2.5-flare
    - davinci-002
    - gpt-5.4-nano
    - o3-mini
    - o4-mini-deep-research-2025-06-26
    - gpt-audio-mini-2025-12-15
    - o1-pro-2025-03-19
    - gpt-live-1
    - gpt-image-2.5-sunburst
    - gpt-image-2.5-flare-2026-09-08
    - gpt-6.1-terra
    - gpt-audio-mini
    - gpt-4.1-mini-2025-04-14
    - gpt-6-luna
    - gpt-4o-mini-search-preview
    - gpt-6-sol
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
  gemini: ok 60 models
  openai: ok 136 models
  perplexity: not_configured (unverified) No Perplexity API Key

=== Get Specific Provider Models ===
OpenAI models: gpt-3.5-turbo-0125, gpt-5.4-2026-03-05, gpt-5-nano, gpt-5.5-pro, gpt-audio-2025-08-28, gpt-4o-mini-transcribe-2025-03-20, gpt-4-turbo-2024-04-09, gpt-realtime-2.1-mini, gpt-realtime-2025-08-28, gpt-4, tts-1-hd, gpt-realtime-1.5, gpt-realtime, gpt-5.2-pro, gpt-5-chat-latest, gpt-5.4-mini-2026-03-17, babbage-002, gpt-4o-2024-05-13, text-embedding-3-large, tts-1, omni-moderation-2024-09-26, gpt-live-transcribe, gpt-5.6-luna, gpt-5-2025-08-07, gpt-5-mini, gpt-5-mini-2025-08-07, omni-moderation-latest, o3-mini-2025-01-31, gpt-audio-1.5, gpt-image-1.5, tts-1-1106, gpt-5.5-pro-2026-04-23, gpt-audio-mini-2025-10-06, gpt-5.2-pro-2025-12-11, o4-mini-2025-04-16, gpt-4o-2024-11-20, gpt-5.1, whisper-1, gpt-5-pro, gpt-realtime-whisper, gpt-realtime-2, gpt-5-search-api, gpt-5.5-2026-04-23, gpt-4.1-nano, gpt-image-2, gpt-3.5-turbo-16k, text-embedding-ada-002, gpt-realtime-2.1, gpt-4.1, gpt-5.5, chat-latest, gpt-realtime-translate, gpt-5.4-pro-2026-03-05, gpt-4.1-mini, gpt-4o-mini, gpt-4o-mini-transcribe-2025-12-15, gpt-realtime-mini, gpt-image-1-mini, gpt-5.6-terra, gpt-3.5-turbo-instruct-0914, gpt-4-turbo, gpt-4o-2024-08-06, o3, gpt-4o-mini-search-preview-2025-03-11, o1-2024-12-17, gpt-5.1-chat-latest, gpt-5-pro-2025-10-06, gpt-5.6-sol, gpt-4o-transcribe-diarize, gpt-5-codex, gpt-4-0613, gpt-5.1-2025-11-13, gpt-4o-search-preview, gpt-4o-mini-tts-2025-12-15, gpt-image-1, gpt-audio, gpt-5.3-codex, gpt-5-search-api-2025-10-14, gpt-5.4-pro, gpt-4o-mini-tts, gpt-realtime-mini-2025-12-15, gpt-5.1-codex-max, sora-2, gpt-4o, gpt-4o-mini-2024-07-18, gpt-4.1-2025-04-14, gpt-5, gpt-4o-mini-transcribe, gpt-image-2-2026-04-21, gpt-5.2-chat-latest, sora-2-pro, gpt-3.5-turbo-instruct, gpt-transcribe, gpt-4.1-nano-2025-04-14, gpt-3.5-turbo-1106, gpt-4o-mini-tts-2025-03-20, gpt-5.2, gpt-5.4-nano-2026-03-17, o1, tts-1-hd-1106, gpt-5.2-codex, gpt-5.4-mini, o1-pro, gpt-5.1-codex, o4-mini-deep-research, gpt-5.4, gpt-4o-search-preview-2025-03-11, text-embedding-3-small, gpt-5-nano-2025-08-07, gpt-4o-transcribe, o4-mini, gpt-5.1-codex-mini, gpt-6-astra, gpt-5.3-chat-latest, o3-2025-04-16, gpt-5.2-2025-12-11, chatgpt-image-latest, gpt-3.5-turbo, gpt-image-2.5-sunburst-2026-09-08, gpt-image-2.5-flare, davinci-002, gpt-5.4-nano, o3-mini, o4-mini-deep-research-2025-06-26, gpt-audio-mini-2025-12-15, o1-pro-2025-03-19, gpt-live-1, gpt-image-2.5-sunburst, gpt-image-2.5-flare-2026-09-08, gpt-6.1-terra, gpt-audio-mini, gpt-4.1-mini-2025-04-14, gpt-6-luna, gpt-4o-mini-search-preview, gpt-6-sol, gpt-6.1-sol

=== Get Anthropic Models ===
Anthropic models: claude-haiku-5-5, claude-sonnet-5-5, claude-opus-5-5, claude-fable-5-1, claude-opus-5, claude-sonnet-5, claude-fable-5, claude-opus-4-8, claude-opus-4-7, claude-sonnet-4-6, claude-opus-4-6, claude-opus-4-5-20251101, claude-haiku-4-5-20251001, claude-sonnet-4-5-20250929

=== Get Non-Existent Provider ===
Expected error for non-existent provider: Error: Request failed with status 404: {"error":"Unknown provider 'nonexistent_provider_xyz'","error_kind":"unknown_provider"}

✓ Chat Models example complete
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 37ZyDVEyQ3lYvfAQlI-o7jp5TgZHb5-AzcnzeN7e8lnqZEF2FL1uPWLJkeyIsts4ZHApga1liE9iCn7GfbG-qg

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is **ekoDB** — a high-performance database product, priced at **$99**.

✓ Message 2 sent
  Response: ekoDB costs **$99**.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: K6CFFsIMDaPwC3vFTmP90fIBWbJ6vMbD9zhA4dFAElJz2u3DPJ9Wi3YyqS-4c7vO97KQnm4hM9c4im3eKP_ljg
  Parent: 37ZyDVEyQ3lYvfAQlI-o7jp5TgZHb5-AzcnzeN7e8lnqZEF2FL1uPWLJkeyIsts4ZHApga1liE9iCn7GfbG-qg

=== Listing Sessions ===
✓ Found 2 sessions
  Session 1: K6CFFsIMDaPwC3vFTmP90fIBWbJ6vMbD9zhA4dFAElJz2u3DPJ9Wi3YyqS-4c7vO97KQnm4hM9c4im3eKP_ljg (Untitled)
  Session 2: 37ZyDVEyQ3lYvfAQlI-o7jp5TgZHb5-AzcnzeN7e8lnqZEF2FL1uPWLJkeyIsts4ZHApga1liE9iCn7GfbG-qg (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: K6CFFsIMDaPwC3vFTmP90fIBWbJ6vMbD9zhA4dFAElJz2u3DPJ9Wi3YyqS-4c7vO97KQnm4hM9c4im3eKP_ljg

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ All session management operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: C0FfhgKUWSOkZdGbaKfuvGPvBMb5YGHyD3UuIojrWfP-koGol4X7c3DNcg2PlB1l2afCAPyae23ICcctXKpsDA

=== List Collections ===
Total collections: 27
Sample collections: schema_users_client_go,audit__ek0_testing,schema_documents_client_js,schema_documents_client_ts,schema_employees_client_ts

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
  POST /api/functions/conc_demo_pay_ts_76798_1793594830125 { "idempotency_key": "...", "amount": 100 }
  POST /api/functions/conc_demo_rl_fail_ts_76798_1793594830125 { "user_id": 42 }
  POST /api/functions/conc_demo_rl_skip_ts_76798_1793594830125 { "user_id": 42 }
  POST /api/functions/conc_demo_lock_ts_76798_1793594830125 { "resource": "queue:drain" }

✓ Cleaned up demo functions
=== ekoDB Convenience Methods Example ===

=== Native Object Creation ===
✓ Created record with plain object: {
  id: 'R-W7LMfk9dPUiKaaxoIRInZmJGONFlvUTPKZWB4tzHz50YCAAdgQ61sVIYeENhdFAkhPPa4YqnrKHnHNydjPaA'
}

=== Upsert Operation ===
✓ First upsert (update): {
  name: { type: 'String', value: 'Alice Johnson' },
  age: { type: 'Integer', value: 29 },
  id: 'R-W7LMfk9dPUiKaaxoIRInZmJGONFlvUTPKZWB4tzHz50YCAAdgQ61sVIYeENhdFAkhPPa4YqnrKHnHNydjPaA',
  email: { value: 'alice.j@newdomain.com', type: 'String' },
  active: { type: 'Boolean', value: true }
}
✓ Second upsert (insert): { id: 'new-user-id' }

=== Find One Operation ===
✓ Found user by email: {
  email: { value: 'alice.j@newdomain.com', type: 'String' },
  name: { type: 'String', value: 'Alice Johnson' },
  age: { value: 29, type: 'Integer' },
  active: { type: 'Boolean', value: true },
  id: 'R-W7LMfk9dPUiKaaxoIRInZmJGONFlvUTPKZWB4tzHz50YCAAdgQ61sVIYeENhdFAkhPPa4YqnrKHnHNydjPaA'
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
✓ Inserted document: W2OSiQAV_frj_pANWSvxpqisGwm7zk9KSCI0FUAXj4WZXrqQOA1fxLvH2MTkxzTuGnpK-T5ubRI1QBUZHJCeBQ

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: tRSJ70k-L9lBJCXQczWgiwqAo4nGdTbmk05-W91Z0TNt_C4OtWH036j9I-NmM6ijzzVmhkPnTAaojzb_DlZv2g

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
✓ Edge cache script created: USs_jv7_74UW4wfvfgGclmUYIJjaTWn50X4geZo-p6KUXwi1tzLuIYa0JtM74t8dlvYgVqnPW2oh7I17xArIBw

Call 1: Cache miss (fetches from API)
Response time: 71ms
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "company": {
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets"
          },
          "website": "hildegard.org",
          "id": 1,
          "username": "Bret",
          "name": "Leanne Graham",
          "phone": "1-770-736-8031 x56442",
          "email": "Sincere@april.biz",
          "address": {
            "zipcode": "92998-3874",
            "street": "Kulas Light",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "suite": "Apt. 556",
            "city": "Gwenborough"
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
Response time: 3ms (23.7x faster!)
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "company": {
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets"
          },
          "website": "hildegard.org",
          "id": 1,
          "username": "Bret",
          "name": "Leanne Graham",
          "phone": "1-770-736-8031 x56442",
          "email": "Sincere@april.biz",
          "address": {
            "zipcode": "92998-3874",
            "street": "Kulas Light",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "suite": "Apt. 556",
            "city": "Gwenborough"
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

✅ Saved reusable function: fetch_user
✅ Saved composed function: get_user_wrapper (calls fetch_user + projects fields)

📊 Result from composed function:
   Records: 1
   Name: {"value":"User 1","type":"String"}
   Department: {"value":"engineering","type":"String"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 72ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "email": "Sincere@april.biz",
      "phone": "1-770-736-8031 x56442",
      "username": "Bret",
      "id": 1,
      "address": {
        "geo": {
          "lng": ...

Second call (cache hit - from cache):
   ⏱️  Duration: 3ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "email": "Sincere@april.biz",
      "phone": "1-770-736-8031 x56442",
      "username": "Bret",
      "id": 1,
      "address": {
        "geo": {
          "lng": ...
   🚀 Cache speedup: 24.0x faster!

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

ℹ️  Function 'get_active_users_client_ts' already existed — updated instead
✅ Function saved: xOrKhF-29hy2j8GEvAiPGODiNV-o7jUwiLi9usRTD-59J9DnAFKeJr8VtSJ-YH505bSSHns4al5mbipcfK3Qqw
📊 Found 5 active users

📝 Example 2: Parameterized Function

✅ Function saved: 3_01AZ0Pyh8a0U5x83lXqIDLhINiLARVukxQ6ZQtYkHxk7Dy5gmRDS7cTcuQfp2CXvjsM8gWzk_5Cyti5lZA3Q
📊 Found 3 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: Vzyd-sfTt6w4LgaaVMjSPCjfB-HhANeQgDaMC0ljgUmntMjw_QHk5jS5XTXdwLyYoMJD9StTRX0_pJLXVMg6cw
📊 Statistics: 2 groups
   {"avg_score":{"value":60,"type":"Float"},"count":{"value":5,"type":"Integer"},"status":{"type":"String","value":"active"}}
   {"count":{"type":"Integer","value":5},"avg_score":{"value":50,"type":"Float"},"status":{"value":"inactive","type":"String"}}

📝 Example 4: UserFunction Management

📋 Total scripts: 9
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
   {"avg_price":{"value":365.6666666666667,"type":"Float"},"count":{"type":"Integer","value":3},"category":{"value":"Furniture","type":"String"}}
   {"count":{"value":5,"type":"Integer"},"category":{"value":"Electronics","type":"String"},"avg_price":{"value":367,"type":"Float"}}
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
   Vector databases are designed to store and search **embeddings**—numeric representations of text, images, audio, or other data. Their main benefits are:

- **Meaning-based search:** Find items that are conceptually similar, even when they don’t share exact keywords.
- **Fast search at scale:** Specialized indexes make nearest-neighbor searches practical across large collections.
- **Richer retrieval:** Combine similarity search with metadata filters, such as date, category, or access permissions.
- **Useful for AI applications:** They can retrieve relevant information for recommendation systems, image search, and retrieval-augmented generation (RAG).
- **Flexible data types:** The same approach can support embeddings from different kinds of content.

They’re most useful when similarity matters more than exact matching; traditional databases are often better for structured queries and exact lookups.
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

✅ Function saved: 6GsLNMCAl6jOXKEv80L1QUWAuoKJ_FWI-8insFgiTAshcX2XJkDQq0YKpRvY5QtZojpWQJaarhctotT7h15eyw
📊 Found 2 product groups
   {"category":{"type":"String","value":"Furniture"},"avg_price":{"type":"Float","value":474},"count":{"type":"Integer","value":2}}
   {"category":{"value":"Electronics","type":"String"},"count":{"type":"Integer","value":3},"avg_price":{"value":575.6666666666666,"type":"Float"}}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   {"category":{"type":"String","value":"Furniture"},"count":{"value":2,"type":"Integer"}}
   {"category":{"value":"Electronics","type":"String"},"count":{"type":"Integer","value":3}}
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
   {"avg_score":{"value":20,"type":"Float"},"count":{"type":"Integer","value":3},"role":{"type":"String","value":"admin"}}
   {"avg_score":{"type":"Float","value":70},"count":{"value":7,"type":"Integer"},"role":{"value":"user","type":"String"}}
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

✅ Inserted order: 5EX8QAz8vPCG2JogWTHuRGRwelfmz1iEhS5Wf0kFVLt9YcDTA0kGCPcNZpvxfOPv5ODsCjWtOLhDkrQArt81wg
✅ Inserted 2 products with wrapped types

📝 Example 2: UserFunction with Wrapped Type Parameters

✅ Function saved: A8xeCPIoNiUZyeLWMQj7tMgwk3Q87KpqeRqZ0saPu2h-8mp4Q_8iWTYvJ2COdgbLDnPCHJb7EVq_DLet8HsIjw
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"type":"Object","value":{"userId":"user_abc","role":"admin"}}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: M74kiCeCNLcJJfjwIK96nlvha5nns7kebirxr5Gep7pbz0-hpJ0RoASfwP3lJ3UFH1M3x-iPvHvzXtwe3Hr-mw
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 3 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Function

✅ Function saved: G3KMjsT7mK-KiL0ih7l3xH2gHIlAd02fxqQzzITKnbd1Uv00SO74ptOPkmyoD5skEmjGepgogjwDmKimSUqTKQ
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
   2. Natural Language Processing (AI)
   3. Getting Started with ekoDB (Database)
   4. Introduction to Machine Learning (AI)
   5. Vector Databases Explained (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   {"category":{"type":"String","value":"AI"},"count":{"value":2,"type":"Integer"}}
   {"category":{"value":"Database","type":"String"},"count":{"type":"Integer","value":3}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
=== ekoDB Goal Template CRUD Example (TypeScript) ===

--- Creating goal template ---
Created template: Data Migration (id: DJr7eNxJP8cnt2wkaMzO2QvxD7Le7f-8b5McaZbWzwYfQWAhmZentJk9JVHbB80lSv6oU2OhFG1FwJvvryoRPQ)

--- Listing templates ---
Templates: {
  count: 1,
  items: [
    {
      description: [Object],
      id: 'DJr7eNxJP8cnt2wkaMzO2QvxD7Le7f-8b5McaZbWzwYfQWAhmZentJk9JVHbB80lSv6oU2OhFG1FwJvvryoRPQ',
      owner: [Object],
      steps: [Object],
      title: [Object]
    }
  ],
  total: 1
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
Created goal: Deploy v2.0 (id: 4OV3E9QO4PNEmSxSCsXNFWxOmz04AdcPaMMsr6Ly_NkE6ruYGHvKwC6zv9nx3lkl6x60-qU3TJ5uAwtGt-5GOA)

--- Listing goals ---
Goals: {
  "count": 1,
  "goals": [
    {
      "created_at": {
        "type": "DateTime",
        "value": "2026-11-02T04:47:17.319877+00:00"
      },
      "description": {
        "type": "String",
        "value": "Ship version 2.0 to production"
      },
      "id": "4OV3E9QO4PNEmSxSCsXNFWxOmz04AdcPaMMsr6Ly_NkE6ruYGHvKwC6zv9nx3lkl6x60-qU3TJ5uAwtGt-5GOA",
      "owner": {
        "type": "Object",
        "value": {
          "key_id": "3a0cdba0da7c060610042783cc758f1f",
          "key_label": "default_admin"
        }
      },
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
        "value": "2026-11-02T04:47:17.319877+00:00"
      }
    }
  ],
  "total": 1
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
        "value": "2026-11-02T04:47:17.319877+00:00"
      },
      "description": {
        "type": "String",
        "value": "Ship version 2.0 with hot-fix patches"
      },
      "id": "4OV3E9QO4PNEmSxSCsXNFWxOmz04AdcPaMMsr6Ly_NkE6ruYGHvKwC6zv9nx3lkl6x60-qU3TJ5uAwtGt-5GOA",
      "owner": {
        "type": "Object",
        "value": {
          "key_id": "3a0cdba0da7c060610042783cc758f1f",
          "key_label": "default_admin"
        }
      },
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
        "value": "2026-11-02T04:47:17.333655+00:00"
      }
    }
  ]
}

--- Goal step: start step 0 ---
Step 0 started on goal 4OV3E9QO4PNEmSxSCsXNFWxOmz04AdcPaMMsr6Ly_NkE6ruYGHvKwC6zv9nx3lkl6x60-qU3TJ5uAwtGt-5GOA
--- Goal step: complete step 0 ---
Step 0 completed on goal 4OV3E9QO4PNEmSxSCsXNFWxOmz04AdcPaMMsr6Ly_NkE6ruYGHvKwC6zv9nx3lkl6x60-qU3TJ5uAwtGt-5GOA
--- Goal step: fail step 1 ---
Step 1 failed on goal 4OV3E9QO4PNEmSxSCsXNFWxOmz04AdcPaMMsr6Ly_NkE6ruYGHvKwC6zv9nx3lkl6x60-qU3TJ5uAwtGt-5GOA

--- Completing goal ---
Goal status: pending_review
--- Approving goal ---
Goal status after approve: in_progress

--- Creating goal to reject ---
--- Rejecting goal ---
Goal status after reject: failed


--- Creating task ---
Created task: Hourly Health Check (id: 2IS-hi9SoEQdNf7qx24wqmzsBLDDQlofSdYKpTBnmeZIhsHUXemGo08xmna8UxGHBAmortCtzyg3Dcp9M8u7Sg)

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
      "id": "2IS-hi9SoEQdNf7qx24wqmzsBLDDQlofSdYKpTBnmeZIhsHUXemGo08xmna8UxGHBAmortCtzyg3Dcp9M8u7Sg",
      "name": {
        "type": "String",
        "value": "Hourly Health Check"
      },
      "owner": {
        "type": "Object",
        "value": {
          "key_id": "3a0cdba0da7c060610042783cc758f1f",
          "key_label": "default_admin"
        }
      }
    }
  ],
  "total": 1
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
Created agent: SupportBot (id: jCSR6JQqPNa9YBEEAGGEgQ30Z9TMXS2Gn-PZ-woy87N2msByfJWjjEk5PU_QgRn0dtmmItsR2TDaD8gnzRyJIg)

--- Listing agents ---
Agents: {
  "count": 1,
  "items": [
    {
      "deployment_id": {
        "type": "String",
        "value": "deploy_prod_1"
      },
      "id": "jCSR6JQqPNa9YBEEAGGEgQ30Z9TMXS2Gn-PZ-woy87N2msByfJWjjEk5PU_QgRn0dtmmItsR2TDaD8gnzRyJIg",
      "llm_model": {
        "type": "String",
        "value": "gpt-6-luna"
      },
      "name": {
        "type": "String",
        "value": "SupportBot"
      },
      "owner": {
        "type": "Object",
        "value": {
          "key_id": "3a0cdba0da7c060610042783cc758f1f",
          "key_label": "default_admin"
        }
      },
      "run_policy": {
        "type": "String",
        "value": "{\"trigger\":{\"allow_write_operations\":false,\"write_tools\":[]},\"eval\":{\"allow_write_operations\":false,\"write_tools\":[]},\"customer\":{\"allow_write_operations\":false,\"write_tools\":[]}}"
      },
      "system_prompt": {
        "type": "String",
        "value": "You are a helpful customer support agent."
      }
    }
  ],
  "total": 1
}

--- Getting agent by ID ---
Fetched: SupportBot

--- Getting agent by name ---
By name: SupportBot (id: jCSR6JQqPNa9YBEEAGGEgQ30Z9TMXS2Gn-PZ-woy87N2msByfJWjjEk5PU_QgRn0dtmmItsR2TDaD8gnzRyJIg)

--- Updating agent ---
Updated agent: SupportBot

--- Getting agents by deployment ---
Agents in deployment: {
  "count": 0,
  "items": []
}
WARNING: agents-by-deployment omitted created agent jCSR6JQqPNa9YBEEAGGEgQ30Z9TMXS2Gn-PZ-woy87N2msByfJWjjEk5PU_QgRn0dtmmItsR2TDaD8gnzRyJIg; TODO: check/fix the server-side deployment lookup

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
  POST /api/functions/jwt_register_ts_78254_1793594837865 { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/jwt_login_ts_78254_1793594837865 { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/jwt_verify_ts_78254_1793594837865 { "token": "<jwt>" }

Set JWT_SECRET in ekoDB's environment_vars whitelist before invoking.

✓ Cleaned up demo functions
=== ekoDB KV Document Linking Example (TypeScript) ===

--- Setup: creating KV key and documents ---
Set KV key: session:admin
Inserted document 1: mku1kiv1Tv9tLDv7oc-dIg6t5BDFFM6HCRFRBDuPnlUOpH5BMl-fN-bmQxxDEiN1pj-T5RbB3AuS5N5YVEYgeg
Inserted document 2: 5ql-EhhqMuS5XQ6aP0KxT95b0_bunSThjvdCZn_u_e8iENXu8cp0PzKnroq6d5tMoyqw6EBfXPnyQMsj0uq-gg

--- Linking documents to KV key ---
Linked doc mku1kiv1Tv9tLDv7oc-dIg6t5BDFFM6HCRFRBDuPnlUOpH5BMl-fN-bmQxxDEiN1pj-T5RbB3AuS5N5YVEYgeg: null
Linked doc 5ql-EhhqMuS5XQ6aP0KxT95b0_bunSThjvdCZn_u_e8iENXu8cp0PzKnroq6d5tMoyqw6EBfXPnyQMsj0uq-gg: null

--- Getting links for KV key ---
Links: [
  {
    "collection": "kv_links_example_ts_78261_1793594837950",
    "document_id": "5ql-EhhqMuS5XQ6aP0KxT95b0_bunSThjvdCZn_u_e8iENXu8cp0PzKnroq6d5tMoyqw6EBfXPnyQMsj0uq-gg",
    "field_path": null,
    "created_at": "2026-11-02T04:47:18.036492Z",
    "last_accessed": "2026-11-02T04:47:18.038112Z",
    "metadata": {}
  },
  {
    "collection": "kv_links_example_ts_78261_1793594837950",
    "document_id": "mku1kiv1Tv9tLDv7oc-dIg6t5BDFFM6HCRFRBDuPnlUOpH5BMl-fN-bmQxxDEiN1pj-T5RbB3AuS5N5YVEYgeg",
    "field_path": null,
    "created_at": "2026-11-02T04:47:18.034788Z",
    "last_accessed": "2026-11-02T04:47:18.038112Z",
    "metadata": {}
  }
]

--- Unlinking document ---
Unlinked doc 5ql-EhhqMuS5XQ6aP0KxT95b0_bunSThjvdCZn_u_e8iENXu8cp0PzKnroq6d5tMoyqw6EBfXPnyQMsj0uq-gg: null

--- Verifying remaining links ---
Remaining links: [
  {
    "collection": "kv_links_example_ts_78261_1793594837950",
    "document_id": "mku1kiv1Tv9tLDv7oc-dIg6t5BDFFM6HCRFRBDuPnlUOpH5BMl-fN-bmQxxDEiN1pj-T5RbB3AuS5N5YVEYgeg",
    "field_path": null,
    "created_at": "2026-11-02T04:47:18.034788Z",
    "last_accessed": "2026-11-02T04:47:18.041067Z",
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
  kv_ops_ts_78286_1793594838133:cache:product:1: success
  kv_ops_ts_78286_1793594838133:cache:product:2: success
  kv_ops_ts_78286_1793594838133:cache:product:3: success

=== KV Batch Get ===
✓ Batch retrieved 3 values
  kv_ops_ts_78286_1793594838133:cache:product:1: { name: 'Product 1', price: 29.99 }
  kv_ops_ts_78286_1793594838133:cache:product:2: { name: 'Product 2', price: 39.99 }
  kv_ops_ts_78286_1793594838133:cache:product:3: { price: 49.99, name: 'Product 3' }

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
  kv_ops_ts_78286_1793594838133:cache:product:1: deleted
  kv_ops_ts_78286_1793594838133:cache:product:2: deleted
  kv_ops_ts_78286_1793594838133:cache:product:3: deleted

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
Response: In traditional art, the primary colors are **red, yellow, and blue**. For light, they’re **red, green, and blue**.

--- Structured Output via SSE ---
JSON response: [
  {"name": "Earth", "diameter_km": 12742},
  {"name": "Jupiter", "diameter_km": 139820},
  {"name": "Mars", "diameter_km": 6779}
]

--- Blocking HTTP (for comparison) ---
Blocking response: Hello! 👋

=== Done ===
=== ekoDB Schedule Management Example (TypeScript) ===

--- Creating schedule ---
Created schedule: Nightly Database Backup (id: 408d24b6-796c-4103-aaa6-c5e4088e0a47, cron: 0 0 2 * * *)

--- Listing schedules ---
Schedules: {
  "count": 1,
  "schedules": [
    {
      "created_at": "2026-11-02T04:47:23.912916Z",
      "cron_expression": "0 0 2 * * *",
      "description": null,
      "enabled": true,
      "function_label": "schedule_noop_typescript_79281_1793594843875",
      "id": "408d24b6-796c-4103-aaa6-c5e4088e0a47",
      "last_execution": null,
      "name": "Nightly Database Backup",
      "next_execution": "2026-11-03T02:00:00Z",
      "parameters": {},
      "stats": {
        "avg_execution_time_ms": 0,
        "failed_executions": 0,
        "last_error": null,
        "successful_executions": 0,
        "total_executions": 0
      },
      "timezone": "UTC",
      "updated_at": "2026-11-02T04:47:23.912916Z"
    }
  ]
}

--- Getting schedule ---
Fetched: Nightly Database Backup (cron: 0 0 2 * * *)

--- Updating schedule ---
Updated: Nightly Full Backup (new cron: 0 0 3 * * *)

--- Triggering schedule ---
Trigger response: {
  "schedule_id": "408d24b6-796c-4103-aaa6-c5e4088e0a47",
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
  1. Score: 13.200, Matched: bio, title
  2. Score: 13.200, Matched: bio, title
  3. Score: 13.200, Matched: title, bio
  4. Score: 13.200, Matched: title, bio

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 26.400, Matched: title, bio
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
  1. Score: 0.771, Matched:
  2. Score: 0.763, Matched:
  3. Score: 0.759, Matched:

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.505, Matched: title, content
  2. Score: 0.904, Matched: content, title
  3. Score: 0.308, Matched:

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
  id: 'NObYoJXW1psJBoEn3vvLS8KK8HMluz3DDK-kMeVmV4QDAZHjpT0ZYQsRZfkHHGNNLP0ndtRtS8jAlLqnB4cARA'
}

=== Find by ID ===
Found: {
  active: { type: 'Boolean', value: true },
  user_id: { value: '550e8400-e29b-41d4-a716-446655440000', type: 'String' },
  metadata: { value: { nested: [Object], key: 'value' }, type: 'Object' },
  categories: { type: 'Array', value: [ 'electronics', 'computers' ] },
  price: { value: 99.99, type: 'Float' },
  tags: { value: [ 'tag1', 'tag2', 'tag3' ], type: 'Array' },
  value: { type: 'Integer', value: 42 },
  created_at: { type: 'DateTime', value: '2026-11-02T04:47:24.651+00:00' },
  name: { value: 'Test Record', type: 'String' },
  id: 'NObYoJXW1psJBoEn3vvLS8KK8HMluz3DDK-kMeVmV4QDAZHjpT0ZYQsRZfkHHGNNLP0ndtRtS8jAlLqnB4cARA',
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  embedding: { value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ], type: 'Array' }
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-11-02T04:47:24.651Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { nested: { deep: true }, key: 'value' }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  active: true,
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  metadata: { nested: { deep: true }, key: 'value' },
  categories: [ 'electronics', 'computers' ],
  price: 99.99,
  tags: [ 'tag1', 'tag2', 'tag3' ],
  value: 42,
  created_at: '2026-11-02T04:47:24.651+00:00',
  name: 'Test Record',
  id: 'NObYoJXW1psJBoEn3vvLS8KK8HMluz3DDK-kMeVmV4QDAZHjpT0ZYQsRZfkHHGNNLP0ndtRtS8jAlLqnB4cARA',
  data: 'aGVsbG8gd29ybGQ=',
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  id: 'NObYoJXW1psJBoEn3vvLS8KK8HMluz3DDK-kMeVmV4QDAZHjpT0ZYQsRZfkHHGNNLP0ndtRtS8jAlLqnB4cARA',
  created_at: { value: '2026-11-02T04:47:24.651+00:00', type: 'DateTime' },
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' },
  categories: { value: [ 'electronics', 'computers' ], type: 'Array' },
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  embedding: { value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ], type: 'Array' },
  price: { value: 99.99, type: 'Float' },
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  name: { type: 'String', value: 'Updated Record' },
  active: { value: true, type: 'Boolean' },
  value: { value: 100, type: 'Integer' },
  metadata: { value: { nested: [Object], key: 'value' }, type: 'Object' }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: pQOg2z86JNapleGOTjcllYQJh_pBKjJ6jtZEsF7X4c-FgFObfz1qL9V8yeo9A957uAPK-81Pb_DQaPHKLkYF3w

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
✓ Created native SWR script: github_user_native_ts (2Xlq7Tt9P2jWy1Epgrk8FTwUjaxDNcN0bsod-b92fBELwCSGYwAnj7buaWq2rN7crpDzTUt03SbI7maW1R1u7w)

First call (cache miss - will fetch from GitHub API):
  Response time: 134ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 3ms
  Speedup: 44.7x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit_ts (QAkNWeLQcTxO8VrZFYSnsyRYJFdfb35No2GLjEaoDOCxKju7yBIG1GLDyJlAlrM6IkAfc5C7oxAPj8_EQgQzjg)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail_ts' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline_ts (J4f48TbS3VTSt6N7nTPq4CGo8NpeolxMuYAvS84jC-mKp4bKZwIjeSZzytax8vykKyrE1Pq-CUoXsFLCSsB1nQ)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users_swr_ts' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache_ts (l-tXZibIFn86Xr8gdk4PFNuTXmPpxDFxMfb8RwNZ1ktHJ5ge1vmomUvsxg4fL1-uz5sj7i5dqwlbHy-KVHj44Q)
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
✓ Created SWR script: fetch_api_user_ts_79580_1793594845872 (9-SS6MX-uciljJ31RpQKmGQ4QZHezuRTjYsBMmAx3mbq3Pps1QjeAi1RcTl-xbtYIaicHpMQTNGu3MIexRyjVA)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "phone": "1-770-736-8031 x56442",
          "id": 1,
          "company": {
            "catchPhrase": "Multi-layered client-server neural-net",
            "bs": "harness real-time e-markets",
            "name": "Romaguera-Crona"
          },
          "email": "Sincere@april.biz",
          "name": "Leanne Graham",
          "username": "Bret",
          "address": {
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            },
            "suite": "Apt. 556",
            "street": "Kulas Light",
            "zipcode": "92998-3874",
            "city": "Gwenborough"
          },
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
          "phone": "1-770-736-8031 x56442",
          "id": 1,
          "company": {
            "catchPhrase": "Multi-layered client-server neural-net",
            "bs": "harness real-time e-markets",
            "name": "Romaguera-Crona"
          },
          "email": "Sincere@april.biz",
          "name": "Leanne Graham",
          "username": "Bret",
          "address": {
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            },
            "suite": "Apt. 556",
            "street": "Kulas Light",
            "zipcode": "92998-3874",
            "city": "Gwenborough"
          },
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
✓ Created enrichment script: fetch_product_reviews_ts_79580_1793594845872 (bHNu8IWx57a6ksDyR-cSYFR333KbZRujTL4Wu9ozke-AqRwfEC0KgEYq72H6iu1IM9nwFZBaTP7zChEBuRv5pg)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "weight": 4,
          "availabilityStatus": "In Stock",
          "brand": "Essence",
          "id": 1,
          "returnPolicy": "No return policy",
          "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
          "stock": 99,
          "tags": [
            "beauty",
            "mascara"
          ],
          "dimensions": {
            "height": 13.08,
            "width": 15.14,
            "depth": 22.99
          },
          "category": "beauty",
          "title": "Essence Mascara Lash Princess",
          "warrantyInformation": "1 week warranty",
          "reviews": [
            {
              "date": "2025-04-30T09:41:02.053Z",
              "reviewerName": "Eleanor Collins",
              "comment": "Would not recommend!",
              "rating": 3,
              "reviewerEmail": "eleanor.collins@x.dummyjson.com"
            },
            {
              "reviewerEmail": "lucas.gordon@x.dummyjson.com",
              "date": "2025-04-30T09:41:02.053Z",
              "reviewerName": "Lucas Gordon",
              "rating": 4,
              "comment": "Very satisfied!"
            },
            {
              "comment": "Highly impressed!",
              "rating": 5,
              "reviewerEmail": "eleanor.collins@x.dummyjson.com",
              "date": "2025-04-30T09:41:02.053Z",
              "reviewerName": "Eleanor Collins"
            }
          ],
          "shippingInformation": "Ships in 3-5 business days",
          "meta": {
            "qrCode": "https://cdn.dummyjson.com/public/qr-code.png",
            "createdAt": "2025-10-09T14:47:01.588Z",
            "updatedAt": "2026-05-23T11:27:41.868Z",
            "barcode": "5784719087687"
          },
          "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
          "price": 9.99,
          "images": [
            "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
          ],
          "minimumOrderQuantity": 48,
          "discountPercentage": 10.48,
          "sku": "BEA-ESS-ESS-001",
          "rating": 2.56
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
Created Alice: $1000 - ID: EOsBqByPbPQQ_UhnWM69V85ZFBnMY3kKAIC9DaMLxviuuokSOcd6kr8yjl8mSnJezwZf7LEJMSZGc2LZivZLgQ
Created Bob: $500 - ID: WbsSkCDdPY-K63ID7jbsVVhbK4Ncp3Bkf9JeyrIBYVvVkOL0JdSj6mlAvo3pCiKFgr-LmtNHqvpuRwgyT7hvHg

=== Example 1: Begin Transaction ===
Transaction ID (server-default isolation): 91b3d619-e8fa-4f2b-a21b-8b69b6a7010d

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
New transaction: 275cdc00-1671-43a1-87e3-a656bc31e888
Updated Bob: $700 → $600 (in transaction)
Status before rollback: Active
✓ Transaction rolled back

✓ Verified Bob remains $700 after rollback

=== Cleanup ===
✓ Deleted test account collection

✓ All client transaction examples completed
✓ Client created

=== Create User Function ===
Created user function with ID: YpmVW2Df1ESFOhZksENdFaLAqecmGlurEaPPrjm2AUOqImuqGhdc3qk434F3XFl5TwX68gjRL7Q0wAKfle0zlg

=== Get User Function ===
Retrieved: get_active_users_ts - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 9 user functions:
  - conc_demo_rl_skip_ts_50152_1793594691316: Rate-limit (skip mode)
  - conc_demo_rl_fail_ts_50152_1793594691316: Rate-limit (fail mode)
  - get_active_users_client_ts: Get Active Users (Updated)
  - get_active_users_ts: Get Active Users
  - get_active_users_client_js: Get Active Users (Updated)
  - fetch_product_reviews_ts_79580_1793594845872: Fetch Product with Reviews (Multi-API)
  - conc_demo_rl_fail_ts_76798_1793594830125: Rate-limit (fail mode)
  - fetch_product_reviews_ts_55079_1793594714450: Fetch Product with Reviews (Multi-API)
  - conc_demo_rl_skip_ts_76798_1793594830125: Rate-limit (skip mode)

=== Update User Function ===
User function updated successfully

=== Delete User Function ===
User function deleted successfully

✓ User Functions API example complete
=== WebSocket Chat Streaming Example (TypeScript) ===

Created chat session: kZjYfbZMNt5RFihvODNwyZpDZnD5SypljeIt5Sk7GUmUo7g_ObWknWLlrot0OvD5BkJ5ji1UPbUctQ94rbNaqA

Sending message: 'What is the capital of France?'
{"__progress":"received"}{"__progress":"generating","model":"gpt-5.6-luna","provider":"openai"}The capital of France is **Paris**.

--- Stream ended ---
Message ID: pm8g_n9cb4QULQn0HoMh3EOAYPkAYm4h4-j6MTlqnqTSuT7aU-Dd9kcOzEQywtlooTAz1llxKVVW8eF_L80Fow
Execution time: 980ms
Token usage: {"cached_input_tokens":0,"completion_tokens":13,"prompt_tokens":23,"thinking_tokens":0,"total_tokens":36}

Full response: {"__progress":"received"}{"__progress":"generating","model":"gpt-5.6-luna","provider":"openai"}The capital of France is **Paris**....
=== WebSocket Subscription Example ===

✓ Authentication successful
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_ts' ===
✓ Subscribed (subscription_id: sub_706942bd73194d8a8dee012a62f4f863)

=== Performing mutations to trigger notifications ===
Inserting a record...
✓ Inserted record: Ck5zIDztEqVwVEbO6ll11ThuRcBMb35u5OfjKS3zPhvo2G8D0qENgxAL2V_0KT1x79aIKP_b7Bl-ZNZm4u6_sQ
  📡 Notification received for Ck5zIDztEqVwVEbO6ll11ThuRcBMb35u5OfjKS3zPhvo2G8D0qENgxAL2V_0KT1x79aIKP_b7Bl-ZNZm4u6_sQ

Inserting another record...
✓ Inserted record: itq4TVs-eR8Q02axzec_7nXqtoeCD88AjrL1ibPJrAkwE8Bq4IY0bbRjRWIPi9HBOshbkRy4N6wYsWyX_A34JQ
  📡 Notification received for itq4TVs-eR8Q02axzec_7nXqtoeCD88AjrL1ibPJrAkwE8Bq4IY0bbRjRWIPi9HBOshbkRy4N6wYsWyX_A34JQ

=== Unsubscribing ===
✓ Unsubscribed: {"collection":"ws_subscribe_example_ts","found":true,"unsubscribed":true}

✓ WebSocket subscription example completed successfully
✓ Deleted collection 'ws_subscribe_example_ts'
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: kNcl_2h00A3D2gRMBgzh1S9lVjo3wa4SYjPSkADyiVulOO3AM1vTJx2nVQGaRo_1vikN4Jp500VeadyQn7gxMQ

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
   Inserted with ripple: {"id":"YQbwAe7EpQvnVuf-4Rtgdepc2IG4v3oC6ONVh8b4biQPd6M0E74cN90EhJD97BtMMTaWDXEuXFcHRzw00BtJDQ"}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {"id":"vjLeYu0nJxyYO6l0PXjFrvw0dCv1420cQNwnDPqpqwD7u338rQzSzAmGkU7tNK3QVKjvW-KLL9GJZOoAqG1XzQ"}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {"name":{"value":"Product 1","type":"String"},"id":"YQbwAe7EpQvnVuf-4Rtgdepc2IG4v3oC6ONVh8b4biQPd6M0E74cN90EhJD97BtMMTaWDXEuXFcHRzw00BtJDQ","price":{"value":150,"type":"Integer"}}

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
  Fields returned: ["email","id","name"]
  First user: Dave Brown <dave@example.com>

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ["name","email","avatar_url","id","status","bio","age","created_at","user_role"]

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
    - Fields: ["bio","email","secret_token","user_role","status","id","password","created_at","avatar_url","name","api_key","age"]
  Projected query:
    - 3 fields per record
    - Fields: ["email","name","id"]
  Bandwidth savings: ~75% fewer fields

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
