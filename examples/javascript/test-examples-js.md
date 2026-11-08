make test-examples-javascript
🧪 Running JavaScript examples (direct HTTP/WebSocket)...

added 1 package, removed 1 package, and audited 9 packages in 457ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
npm warn install-scripts 1 package has install scripts not yet covered by allowScripts:
npm warn install-scripts   ekodb-client-ts@0.32.0 (prepare: npm run build)
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
  id: '7HXLGVIaRA5rYgL9BZMa71WFAOqs9CbZekHQYg0bH8OeIiyj8oUyrpantD3C0ySsvCIR4bEfA9rg3cGaKE-OIg'
}

=== Find by ID ===
Found: {
  value: { type: 'Integer', value: 42 },
  name: { value: 'Test Record', type: 'String' },
  id: '7HXLGVIaRA5rYgL9BZMa71WFAOqs9CbZekHQYg0bH8OeIiyj8oUyrpantD3C0ySsvCIR4bEfA9rg3cGaKE-OIg',
  active: { value: true, type: 'Boolean' }
}

=== Find with Query ===
Found documents: [
  {
    value: { type: 'Integer', value: 42 },
    active: { value: true, type: 'Boolean' },
    name: { value: 'Test Record', type: 'String' },
    id: '7HXLGVIaRA5rYgL9BZMa71WFAOqs9CbZekHQYg0bH8OeIiyj8oUyrpantD3C0ySsvCIR4bEfA9rg3cGaKE-OIg'
  }
]

=== Update Document ===
Updated: {
  name: { type: 'String', value: 'Updated Record' },
  value: { value: 100, type: 'Integer' },
  active: { type: 'Boolean', value: true },
  id: '7HXLGVIaRA5rYgL9BZMa71WFAOqs9CbZekHQYg0bH8OeIiyj8oUyrpantD3C0ySsvCIR4bEfA9rg3cGaKE-OIg'
}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
✓ simple_crud.js completed successfully

=== Running ekoDB/ekodb-client/examples/javascript/simple_websocket.js ===
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: VLIh4AG7OtIYv7zCL_FBvllB89Wh8fRtjPYg6nbwi9CW4ZBRkBOIVP5tbbVi8w74VlV9BefQvx9xEr5LpM90KA

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
        "id": "VLIh4AG7OtIYv7zCL_FBvllB89Wh8fRtjPYg6nbwi9CW4ZBRkBOIVP5tbbVi8w74VlV9BefQvx9xEr5LpM90KA",
        "active": {
          "type": "Boolean",
          "value": true
        },
        "name": {
          "value": "WebSocket Test Record",
          "type": "String"
        }
      }
    ]
  },
  "messageId": "1794156899851"
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

✅ Function saved: CJ_vdzBhoRz8d45dCvJIheZZytcyTnskLuf9OWN-0t2Dr8QwUVt-m571HfEStD7XfsKan31VEsnoyOpSxG6m_w
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Function saved: 1L5kpJ7S7j3PhDC2x2D5AW69F6fY3zP9fzMX9sCHFdYoevTbdLv_4wHb9YP-r1Y5wgAcQHzWVhp0e52VTXZuOQ
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: gp1YdHIqutRtcHF_n43nNtL1vNOTOmQlnPR8yxFHyt-mzE2jadtOzFhl6BZddocvHEBR6r872WVZAq7RzR1Mtg
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"avg_score":{"value":50,"type":"Float"},"count":{"value":5,"type":"Integer"},"status":{"type":"String","value":"inactive"},"max_score":{"type":"Integer","value":90}}
   {"count":{"type":"Integer","value":5},"avg_score":{"value":60,"type":"Float"},"status":{"value":"active","type":"String"},"max_score":{"type":"Integer","value":100}}

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
  type: 'Object',
  value: { name: 'Product 2', price: 39.989999999999995 }
}
cache:product:3: {
  type: 'Object',
  value: { name: 'Product 3', price: 49.989999999999995 }
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
Collection created with first record: A3keD60H-V-gT1w37Nj1NMf0veWMEKGmkTdM-MACUoj_zWnxIwWioQKDje3ics24S5Ed_tfz9CiQu4nFAMRFtA

=== List Collections ===
Total collections: 22
Sample collections: [
  'schema_products_client_go',
  'schema_documents_client_ts',
  'chat_raw_completions__ek0_testing',
  'chat_agent_configs__ek0_testing',
  'schedules__ek0_testing'
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
Created Alice: $1000 - ID: R7vLrpPKPQIVXlX0Gb45zhNJSqDt2NCAbkxb705Y1WQ0_dc2-kgxydqKt36tvsACvkaH0GLSb1SPZbgVKDmN1A
Created Bob: $500 - ID: wrz7T6KGz-FU6OuDPcQXtPYwxQ72zRueJQEuvGjSIw4xxfN2PwP3C94Z3D6L-kqwn_yVyQOrI_D9Q-qn6AGzmA

=== Example 1: Begin Transaction ===
Transaction ID: e57ea19a-934c-49f2-b6e5-4b2b8dc01e06

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
New transaction: b29a50b5-8df0-437f-acc8-afc815078e37
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
   ✅ Function saved: w56Yu1a4hJOAhyvk3PdUXsfo_-a4Rld6726Mu9PasxTGgGElS6CA96V1ZJIb6vYKKjyYYg8oMDKta76_wJW8FA

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
   ✅ Function saved: brHP6cTZU7VtVv0R9TzDiEov1cxSjTXZVr5zzmZBsX3E2qUMZq44wvgCUFYl3Ri1aL4AKaLT_dPI8lqdxztDLw

2️⃣ Calling Function (Query + Update + Verify)...
   ✅ Function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Status updated to: {"type":"String","value":"active"}
   📋 Name: {"value":"Alice Smith","type":"String"}

============================================================
📝 Function 3: Query + Update Credits + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: S9-cGjCWeCjZu1AVDu_BuDsO5nWSechsQMzUO1Q6WH1kcOn9Br8Y3jj0jrkTK5TCSERVTSTr5iHmrhl82jzNng

2️⃣ Calling Function (Query + Update Credits + Verify)...
   ✅ Function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {"value":100,"type":"Integer"}
   📋 Status: {"value":"active","type":"String"}
   📋 Name: {"value":"Alice Smith","type":"String"}

============================================================
📝 Function 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: JeLSfb01y76VnYZ2GRZPheJ-aRKltLVQDEsQEmFVfCg3vLmKX-Wq9uR-c04GMNRujyaNKIDbPsQciutsCW4tJg

2️⃣ Calling Function (Query + Delete + Verify)...
   ✅ Function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted function: w56Yu1a4hJOAhyvk3PdU...
   ✅ Deleted function: brHP6cTZU7VtVv0R9TzD...
   ✅ Deleted function: S9-cGjCWeCjZu1AVDu_B...
   ✅ Deleted function: JeLSfb01y76VnYZ2GRZP...
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
  Output: Document ID = 4TVn8_b9Ba5pnS_EsEESJnR0dp62aJpCNUwsQuXuZtncZBSW0P9s1GmVoMhDVLT1u-SPt-7lXbu5ZU9_9xXz5w
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: findById(4TVn8_b9Ba5pnS_EsEESJnR0dp62aJpCNUwsQuXuZtncZBSW0P9s1GmVoMhDVLT1u-SPt-7lXbu5ZU9_9xXz5w)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: findById(4TVn8_b9Ba5pnS_EsEESJnR0dp62aJpCNUwsQuXuZtncZBSW0P9s1GmVoMhDVLT1u-SPt-7lXbu5ZU9_9xXz5w)
  Output: Error (expected) - Request failed with status 404: {"error":"Record has been deleted"}
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
  Output: Document ID = PgtizoSMOrxceuLaKVVM462tWZFMDMYDUsh2-H8uMFnCWncegVfGjLEi9-EDyj4YNFfm_wum6-q9tNiPtHDO3Q
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: findById(PgtizoSMOrxceuLaKVVM462tWZFMDMYDUsh2-H8uMFnCWncegVfGjLEi9-EDyj4YNFfm_wum6-q9tNiPtHDO3Q)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: findById(PgtizoSMOrxceuLaKVVM462tWZFMDMYDUsh2-H8uMFnCWncegVfGjLEi9-EDyj4YNFfm_wum6-q9tNiPtHDO3Q)
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

> @ekodb/ekodb-client@0.32.0 prepare
> npm run build


> @ekodb/ekodb-client@0.32.0 build
> tsc


up to date, audited 44 packages in 615ms

12 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
npm warn install-scripts 1 package has install scripts not yet covered by allowScripts:
npm warn install-scripts   fsevents@2.3.3 (install: (install scripts present))
npm warn install-scripts
npm warn install-scripts Run `npm install-scripts ls` to review, or `npm install-scripts approve <pkg>` to allow.

> @ekodb/ekodb-client@0.32.0 build
> tsc

✅ TypeScript client built!

added 1 package, removed 1 package, and audited 9 packages in 442ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
npm warn install-scripts 1 package has install scripts not yet covered by allowScripts:
npm warn install-scripts   ekodb-client-ts@0.32.0 (prepare: npm run build)
npm warn install-scripts
npm warn install-scripts Run `npm install-scripts ls` to review, or `npm install-scripts approve <pkg>` to allow.

added 1 package, removed 1 package, and audited 13 packages in 441ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
npm warn install-scripts 3 packages have install scripts not yet covered by allowScripts:
npm warn install-scripts   ekodb-client-ts@0.32.0 (prepare: npm run build)
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
✓ Created session: yXXGXYQW9PUz8QAMkq9RAUYuJG90NXB9tV-2T2zTbHPnj-eJWv2KOaAW-JI7x3qxdmUhXjT9H-mgL4-3JHUfOw

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is **ekoDB**, a high-performance database product priced at **$99**.

✓ Second message sent

=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: ekoDB costs **$99**.

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: lUeW_k4iBXLL5j414uVTSrzImDDKxJPpqQs3dLoJeYqX8ygixGol9leVAvpX93P9_HoVyftZXOB5yHXE83L8Zw
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
✓ Created session: rEmxX2vSKzutF0Cno1VZdJDRX8IoY42SQqEjsYrNiRiS1duJoFB_jz0eQEMHRfEPKJqJJIMK6UVFBIwf3D7jhg

=== Sending Chat Message ===
Message ID: XbkEHwl8D6EvDL4i3YOeGAByZRR2CTmsy9Wt3XG4oPpDms2jMdX746H8WB98MiXt7vbj5ougYEoGsskRjfc_HA

=== AI Response ===
- **ekoDB** — $99
- **ekoDB Pro** — $299
- **ekoDB Cloud** — $499

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_js',
  record: {
    description: 'A high-performance database product with AI capabilities',
    id: '1x4O65dguiglYqvPoqumtiuBvah9L99-UOqXi5HzKMR4QNvtYC_rWkIB7pMDIgTr6CEoRsQDkU70AXfI_2dTKg',
    price: 99,
    name: 'ekoDB'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_js',
  record: {
    id: 'DVabTHICOVfS8KdR8R4amVXosxW9WxdRzKmG1_XoGoXH3KyJGXltyMcBvwBNlWATRarRnkk2miE_Nr1zxzm-FA',
    name: 'ekoDB Pro',
    description: 'Enterprise edition product with advanced features',
    price: 299
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_js',
  record: {
    name: 'ekoDB Cloud',
    price: 499,
    description: 'Fully managed cloud database service product',
    id: 'LTeTdD5sKzL03iJqJA0vhsYYjKFcFVcY3ExaM6oICc90yv3BTLCciTgiC-zL75ouq2Itm_7woOcYvNP2ouZAew'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 3760ms

=== Token Usage ===
Prompt tokens: 41732
Completion tokens: 117
Total tokens: 41849

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ Chat completed successfully
✓ Client created

=== List Chat Models ===
Available chat models by provider:
  openai:
    - tts-1
    - gpt-5-mini
    - gpt-5-mini-2025-08-07
    - gpt-5.4-2026-03-05
    - gpt-5-nano
    - gpt-5.4-mini
    - omni-moderation-2024-09-26
    - gpt-live-transcribe
    - gpt-5.6-luna
    - gpt-4o-2024-11-20
    - gpt-5.1
    - whisper-1
    - gpt-5-pro
    - gpt-realtime-whisper
    - omni-moderation-latest
    - gpt-realtime-2
    - gpt-5-search-api
    - gpt-5.5-2026-04-23
    - gpt-5.5-pro
    - gpt-audio-2025-08-28
    - gpt-4o-mini-transcribe-2025-03-20
    - chat-latest
    - gpt-realtime-translate
    - gpt-5.4-pro-2026-03-05
    - gpt-4o-mini-transcribe-2025-12-15
    - gpt-5-pro-2025-10-06
    - gpt-5.6-sol
    - gpt-5.4-mini-2026-03-17
    - text-embedding-3-large
    - gpt-4o-2024-08-06
    - o3
    - gpt-5.4-pro
    - gpt-4o-mini-tts
    - gpt-realtime-mini-2025-12-15
    - gpt-realtime-2.1-mini
    - gpt-realtime-2025-08-28
    - tts-1-hd-1106
    - tts-1-hd
    - gpt-realtime-1.5
    - gpt-realtime
    - gpt-5.2-pro
    - gpt-4o-transcribe-diarize
    - gpt-5.1-2025-11-13
    - gpt-5.4
    - text-embedding-3-small
    - gpt-5-2025-08-07
    - o3-2025-04-16
    - gpt-5.2-2025-12-11
    - chatgpt-image-latest
    - tts-1-1106
    - gpt-5.5-pro-2026-04-23
    - gpt-5.2-pro-2025-12-11
    - gpt-5.4-nano
    - gpt-audio-mini-2025-12-15
    - gpt-audio-mini
    - gpt-4.1-mini-2025-04-14
    - gpt-5
    - gpt-4o-mini-transcribe
    - gpt-image-2-2026-04-21
    - gpt-transcribe
    - gpt-image-2
    - text-embedding-ada-002
    - gpt-realtime-2.1
    - gpt-4.1
    - gpt-5.5
    - gpt-audio-1.5
    - gpt-image-1.5
    - gpt-6.1-sol
    - gpt-6-sol
    - gpt-realtime-mini
    - gpt-image-1-mini
    - gpt-5.6-terra
    - gpt-5-nano-2025-08-07
    - gpt-4o-transcribe
    - gpt-4.1-mini
    - gpt-4o-mini
    - gpt-6.1-terra
    - gpt-6-luna
    - gpt-4o-mini-tts-2025-12-15
    - gpt-audio
    - gpt-5.3-codex
    - gpt-5-search-api-2025-10-14
    - gpt-4o
    - gpt-image-2.6-flare
    - gpt-4o-mini-2024-07-18
    - gpt-4.1-2025-04-14
    - gpt-image-2.5-flare
    - gpt-4o-mini-tts-2025-03-20
    - gpt-5.2
    - gpt-5.4-nano-2026-03-17
    - gpt-6-astra
    - gpt-image-2.5-sunburst-2026-09-08
    - gpt-image-2.5-sunburst
    - gpt-image-2.5-flare-2026-09-08
    - gpt-image-2.6-sunburst
    - gpt-live-1
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
    - gemini-omni-1.2-flash
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
  gemini: ok 61 models
  openai: ok 96 models
  perplexity: not_configured (unverified) No Perplexity API Key

=== Get Specific Provider Models ===
OpenAI models: tts-1, gpt-5-mini, gpt-5-mini-2025-08-07, gpt-5.4-2026-03-05, gpt-5-nano, gpt-5.4-mini, omni-moderation-2024-09-26, gpt-live-transcribe, gpt-5.6-luna, gpt-4o-2024-11-20, gpt-5.1, whisper-1, gpt-5-pro, gpt-realtime-whisper, omni-moderation-latest, gpt-realtime-2, gpt-5-search-api, gpt-5.5-2026-04-23, gpt-5.5-pro, gpt-audio-2025-08-28, gpt-4o-mini-transcribe-2025-03-20, chat-latest, gpt-realtime-translate, gpt-5.4-pro-2026-03-05, gpt-4o-mini-transcribe-2025-12-15, gpt-5-pro-2025-10-06, gpt-5.6-sol, gpt-5.4-mini-2026-03-17, text-embedding-3-large, gpt-4o-2024-08-06, o3, gpt-5.4-pro, gpt-4o-mini-tts, gpt-realtime-mini-2025-12-15, gpt-realtime-2.1-mini, gpt-realtime-2025-08-28, tts-1-hd-1106, tts-1-hd, gpt-realtime-1.5, gpt-realtime, gpt-5.2-pro, gpt-4o-transcribe-diarize, gpt-5.1-2025-11-13, gpt-5.4, text-embedding-3-small, gpt-5-2025-08-07, o3-2025-04-16, gpt-5.2-2025-12-11, chatgpt-image-latest, tts-1-1106, gpt-5.5-pro-2026-04-23, gpt-5.2-pro-2025-12-11, gpt-5.4-nano, gpt-audio-mini-2025-12-15, gpt-audio-mini, gpt-4.1-mini-2025-04-14, gpt-5, gpt-4o-mini-transcribe, gpt-image-2-2026-04-21, gpt-transcribe, gpt-image-2, text-embedding-ada-002, gpt-realtime-2.1, gpt-4.1, gpt-5.5, gpt-audio-1.5, gpt-image-1.5, gpt-6.1-sol, gpt-6-sol, gpt-realtime-mini, gpt-image-1-mini, gpt-5.6-terra, gpt-5-nano-2025-08-07, gpt-4o-transcribe, gpt-4.1-mini, gpt-4o-mini, gpt-6.1-terra, gpt-6-luna, gpt-4o-mini-tts-2025-12-15, gpt-audio, gpt-5.3-codex, gpt-5-search-api-2025-10-14, gpt-4o, gpt-image-2.6-flare, gpt-4o-mini-2024-07-18, gpt-4.1-2025-04-14, gpt-image-2.5-flare, gpt-4o-mini-tts-2025-03-20, gpt-5.2, gpt-5.4-nano-2026-03-17, gpt-6-astra, gpt-image-2.5-sunburst-2026-09-08, gpt-image-2.5-sunburst, gpt-image-2.5-flare-2026-09-08, gpt-image-2.6-sunburst, gpt-live-1

=== Get Anthropic Models ===
Anthropic models: claude-haiku-5-5, claude-sonnet-5-5, claude-opus-5-5, claude-fable-5-1, claude-opus-5, claude-sonnet-5, claude-fable-5, claude-opus-4-8, claude-opus-4-7, claude-sonnet-4-6, claude-opus-4-6, claude-opus-4-5-20251101, claude-haiku-4-5-20251001, claude-sonnet-4-5-20250929

=== Get Non-Existent Provider ===
Expected error for non-existent provider: Error: Request failed with status 404: {"error":"Unknown provider 'nonexistent_provider_xyz'","error_kind":"unknown_provider"}

✓ Chat Models example complete
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: xi-uP9IZ-PQ5AVptI_c7rtll-GxBwYyoWeusYcjExJNSvqFlB_6xAZRBOjVtXRthabQxebx9R2Gn3aGncfBxZQ

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is **ekoDB**, a high-performance database product, priced at **$99**.

✓ Message 2 sent
  Response: ekoDB costs **$99**.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: tn2MkWyQjnaioHYDy1UHH5q2C8bAF46ZpvVAJ3W9wBNBd7eMgu15DDVh0FG5MeFy5lYwI3nc0VJq5XcXbToIxw
  Parent: xi-uP9IZ-PQ5AVptI_c7rtll-GxBwYyoWeusYcjExJNSvqFlB_6xAZRBOjVtXRthabQxebx9R2Gn3aGncfBxZQ

=== Listing Sessions ===
✓ Found 2 sessions
  Session 1: tn2MkWyQjnaioHYDy1UHH5q2C8bAF46ZpvVAJ3W9wBNBd7eMgu15DDVh0FG5MeFy5lYwI3nc0VJq5XcXbToIxw (Untitled)
  Session 2: xi-uP9IZ-PQ5AVptI_c7rtll-GxBwYyoWeusYcjExJNSvqFlB_6xAZRBOjVtXRthabQxebx9R2Gn3aGncfBxZQ (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: tn2MkWyQjnaioHYDy1UHH5q2C8bAF46ZpvVAJ3W9wBNBd7eMgu15DDVh0FG5MeFy5lYwI3nc0VJq5XcXbToIxw

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ All session management operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: NYE1HvXJMJDhzizIvnOJuSYWOqbvv94Ngha6tIAmonpAhVBWZ5SKsTEhWX4SY9ruJsnsvlvSfGcNXPCwUBaLkg

=== List Collections ===
Total collections: 23
Sample collections: schema_products_client_go,schema_documents_client_ts,chat_raw_completions__ek0_testing,chat_agent_configs__ek0_testing,schedules__ek0_testing

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: ZTROywyyKat4gFpSV0TDkpclN5NAf6eeGDRz2H0H7pry_c8s2hiWEcZCRcgh1cCLiTHHM5c4nWwUYKahPp7Bhg

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: EFpQ-r_A_e7FHJyvBkGnI8PEKcMS0WaxZy2JbUEKa40qRD9d5r5WZv20w3PTd_PCQdUQuTQLoc-uf1IYUVhLqA

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
✓ Edge cache script created: ruBXlgsTROBp7oy4OdpTd9UHL9YjlO3J6rZ0Pjfb7UAItrAh1CD0KXuaj9W4HhyAkPqI_fq4vkeONLa5wk_KOQ

Call 1: Cache miss (fetches from API)
Response time: 151ms
Result: {
  "records": [
    {
      "value": {
        "value": {
          "elevation": 32,
          "timezone": "GMT",
          "timezone_abbreviation": "GMT",
          "longitude": -73.99308,
          "current": {
            "interval": 900,
            "temperature_2m": 14.1,
            "time": "2026-11-08T16:45"
          },
          "utc_offset_seconds": 0,
          "latitude": 40.710335,
          "generationtime_ms": 0.030875205993652344,
          "current_units": {
            "interval": "seconds",
            "temperature_2m": "°C",
            "time": "iso8601"
          }
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
Response time: 2ms (75.5x faster!)
Result: {
  "records": [
    {
      "value": {
        "value": {
          "elevation": 32,
          "timezone": "GMT",
          "timezone_abbreviation": "GMT",
          "longitude": -73.99308,
          "current": {
            "interval": 900,
            "temperature_2m": 14.1,
            "time": "2026-11-08T16:45"
          },
          "utc_offset_seconds": 0,
          "latitude": 40.710335,
          "generationtime_ms": 0.030875205993652344,
          "current_units": {
            "interval": "seconds",
            "temperature_2m": "°C",
            "time": "iso8601"
          }
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
    "value": {
      "address": {
        "street": "Kulas Light",
        "city": "Gwenborough",
        "geo": {
          "lat": "-37.3159",
          "lng": "81.1496"
        },
   ...

Second call (cache hit - from cache):
   ⏱️  Duration: 1ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "address": {
        "street": "Kulas Light",
        "city": "Gwenborough",
        "geo": {
          "lat": "-37.3159",
          "lng": "81.1496"
        },
   ...
   🚀 Cache speedup: 60.0x faster!

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
   Electronics:
      Products: 6 | Stock: 232 | Avg Rating: ⭐4.52
   Furniture:
      Products: 4 | Stock: 43 | Avg Rating: ⭐4.26

⏱️  Total execution time: 0ms
📈 Pipeline stages:

📝 Example 5: Project Specific Fields

✅ Function saved
📊 Product summaries (10 items, showing first 3):
   1. Webcam HD - $119 (⭐4.5)
   2. Laptop Pro - $1299 (⭐4.8)
   3. Bookshelf - $149 (⭐4.1)
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
   Vector databases store and search high-dimensional embeddings, which makes them useful when “similar in meaning” matters more than an exact text or field match.

Key benefits include:

- **Semantic search:** Find relevant text, images, audio, or other content even when queries use different wording.
- **Efficient nearest-neighbor search:** Specialized indexes can retrieve similar vectors quickly across large collections.
- **RAG support:** Retrieve relevant documents to provide context for language models, improving grounded answers.
- **Multimodal retrieval:** Search across embeddings from different data types, depending on the embedding model and database.
- **Metadata filtering and hybrid search:** Combine similarity results with filters or keyword search to improve relevance.
- **Scalable operations:** Many systems support indexing, updates, and distributed storage as collections grow.

They’re most useful when similarity-based retrieval is central; for exact lookups or structured queries, a conventional database may be simpler.
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embedding function saved
📊 Generated embeddings for 3 articles
   1. "Advanced Query Patterns" - 1536D vector
   2. "Draft Article" - 1536D vector
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

✅ Function saved: ZUiR_9PLM-RseOGMd49KqvhrUTNcIg5u9hKfvxS4woVRv2Oq7HvvXzZwnxSOxn_kEOVSojlCJCLALuFYAJaqGw
📊 Found 2 product groups
   {"count":{"value":2,"type":"Integer"},"category":{"type":"String","value":"Furniture"},"avg_price":{"type":"Float","value":474}}
   {"avg_price":{"type":"Float","value":575.6666666666666},"category":{"value":"Electronics","type":"String"},"count":{"type":"Integer","value":3}}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   {"category":{"type":"String","value":"Electronics"},"count":{"value":3,"type":"Integer"}}
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
   user: 70.0 (7 users)
   admin: 20.0 (3 users)
⏱️  Execution time: 0ms

📝 Example 4: Top Users by Score

✅ Function saved
📊 Users (showing first 5 of 10):
   1. User 6 - Score: 60
   2. User 10 - Score: 100
   3. User 9 - Score: 90
   4. User 2 - Score: 20
   5. User 8 - Score: 80
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

✅ Inserted order: IUEM1B6Kl1gSUJBxHCSM1HpUUAoSfvN_VCIkOigJ7kyKxqkAH8M2N6p8B_sOsiWM9e4VYhDXmTZHk7jRFG_DTw
✅ Inserted 2 products with wrapped types

📝 Example 2: Function with Wrapped Type Parameters

✅ Function saved: CUUdFP1Eq5BKqDtdHcaTVwVi3IEhpRr3YBKKV55rxpbi-H9qW47OJYcn0dr1EprkDsYIPqcE2O8puzigRoms2A
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"value":{"role":"admin","userId":"user_abc"},"type":"Object"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: y2YP6Xa8mISPXgj6EAuwTF1l4AIxMP3JJXW3Bj_yLHEFU98O2vbbwD8B5Qan-F3w7dNNL961ktfv-mSx-kMNng
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 3 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Function

✅ Function saved: JPhiqEErxez9BLn_7AxYZ0OjV8m1ojBKC7BfehTFUBW10d-0vBeEYMgApYoYXfzvjOgo_Z_gphRkeUYH_aUSQw
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
   2. Database Design Principles (Database)
   3. Getting Started with ekoDB (Database)
   4. Natural Language Processing (AI)
   5. Vector Databases Explained (Database)
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
   2. Database Design Principles
   3. Getting Started with ekoDB
   4. Natural Language Processing
   5. Vector Databases Explained
⏱️  Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Function saved
📊 Projected documents (showing first 3):
   1. Introduction to Machine Learning
   2. Database Design Principles
   3. Getting Started with ekoDB
⏱️  Execution time: 0ms

📝 Example 5: All Document Fields

✅ Function saved
📊 All documents (5 total, showing first 2):
   1. Introduction to Machine Learning (AI)
   2. Database Design Principles (Database)
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search function examples completed!
🚀 ekoDB Functions Example (JavaScript Client)

✅ Client initialized (token exchange automatic)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved: JXEetr5XEJX41IJePSHXZMbET8o0gY5YM4AxuOhgFwgOBBiORC5XDhVSBVf1YD8zKHGlV-33cAfoDDyHA9FIBA
📊 Found 5 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Function

✅ Function saved
📊 Found 3 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Function

✅ Function saved
📊 Statistics: 2 groups
   {"avg_score":{"type":"Float","value":60},"status":{"type":"String","value":"active"},"count":{"type":"Integer","value":5}}
   {"status":{"type":"String","value":"inactive"},"count":{"type":"Integer","value":5},"avg_score":{"value":50,"type":"Float"}}
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
Retrieved value: { type: 'Object', value: { username: 'john_doe', userId: 123 } }

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: { type: 'Object', value: { name: 'Product 1', price: 29.99 } }
cache:product:2: {
  type: 'Object',
  value: { name: 'Product 2', price: 39.989999999999995 }
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
  1. Score: 26.400, Matched: bio, title
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
  1. Score: 0.744, Matched:
  2. Score: 0.736, Matched:
  3. Score: 0.726, Matched:

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.497, Matched: content, title
  2. Score: 0.891, Matched: title, content
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
  id: 'dwnhACa-rVk4Jr7B9vJO1rujJj6586ePZB80J5tVs2sOg4poRrNpQUqqfU29U0grJr8Qj-6ol6KLMP4kbMsrCw'
}

=== Find by ID ===
Found: {
  id: 'dwnhACa-rVk4Jr7B9vJO1rujJj6586ePZB80J5tVs2sOg4poRrNpQUqqfU29U0grJr8Qj-6ol6KLMP4kbMsrCw',
  name: { type: 'String', value: 'Test Record' },
  active: { value: true, type: 'Boolean' },
  value: { value: 42, type: 'Integer' }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  name: { type: 'String', value: 'Updated Record' },
  active: { value: true, type: 'Boolean' },
  value: { type: 'Integer', value: 100 },
  id: 'dwnhACa-rVk4Jr7B9vJO1rujJj6586ePZB80J5tVs2sOg4poRrNpQUqqfU29U0grJr8Qj-6ol6KLMP4kbMsrCw'
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: Tlt8kSs_OIUnUUVddObdPm91SgldzmOOJDLWw4nvGLVqyRvd5ytrE5eJEE7JC2SmZiRpV-0lI-gsmiNY9XJdAA

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
✓ Created native SWR script: github_user_native_js (BMRrFi4W1Cy-3SQt9Z53h3vFfiEhfRcIOEMA3AwYlFziA0W7R2uGbs2twr41njwHmP9GxBY-Bsc_gezHxZ0hLA)

First call (cache miss - will fetch from GitHub API):
  Response time: 128ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 2ms
  Speedup: 64.0x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit_js (9bDBP2LkYupDOBH1N8vuJ2ErVmDbXRcY-qCa1G6zjfBpm1vTkb3pD1iymFDuDGOS4U9yDJQyxDwPfEJxOIU-JA)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail_js' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline_js (HgRYwttZX2_HXV5h7XHs88hZT-Pc3jhqqPXUe2hNN1YcBNGSYQPc7SD8Ud8jPkMi0XM0G_wObpBAunt0Ua3Erg)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users_swr_js' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache_js (NAtvhQKXanEak6JJOvh73MIjTRg48doD_V4ed4Q4FjDhtuRJNKOWo7qFzb0tXxw26GLJW75FenVCU0mSk7GjVA)
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
✓ Created SWR script: fetch_api_user_js_75830_1794156971445 (vlV3EckrZcS8Z4HSBb1AWyt4SjqUisl8qq-vrnE2n7esXqrLhSm3rnJub4Cjzg0zQ0UeKMy2IFI73L6PjMuuZA)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "username": "Bret",
          "phone": "1-770-736-8031 x56442",
          "company": {
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona"
          },
          "id": 1,
          "website": "hildegard.org",
          "name": "Leanne Graham",
          "address": {
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "street": "Kulas Light",
            "city": "Gwenborough",
            "suite": "Apt. 556",
            "zipcode": "92998-3874"
          },
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
Created Alice: $1000 - ID: Xrgl4S8tBaHP7LqsaJUGrYCmMyfJ42-aG0Y2WvWyj2Fzqrl-qZYSAlpDD0CbkdiiPYvLXN0Oi8x-WHbDMxob8g
Created Bob: $500 - ID: gyNr8dYC50cehJTI1QnJVJBSnJg3ot9yShCF73Nfwq-KzDIpCmpJ6HwxY-swfEFYIIDz-8WC6hMnHg8Uogb9_Q

=== Example 1: Begin Transaction ===
Transaction ID (server-default isolation): 076daf08-be15-4c79-a0a5-aff1da6f1593

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
New transaction: 419e30d4-4b48-4279-8415-64572d42d923
Updated Bob: $700 → $600 (in transaction)
Status before rollback: Active
✓ Transaction rolled back

✓ Verified Bob remains $700 after rollback

=== Cleanup ===
✓ Deleted test account collection

✓ All client transaction examples completed
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: igRKCgVyDlxgOPRa-aryEvq3Jf60l-q1a8InKkN5oEHtSSFV9B77-V6iT3R2Vscx_qE5a1hfGOTf6jLK2VWDbg

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
Inserted: HJpkvL-0F4OL_ZODK1KSfunKBCyhr-_SaJjNoiz2CfMv4bOdaOw9ul1oYWLzHdXYqlxCMmy-qBORrgp7nBPp7g

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
✓ Created session: UWngJK028EGWraZLGSftwLDsP2zL7SYHrjn_Gmy4iR_UOhsJCpau54dfKMtkc07-XYIRdkUEGVDF9ViaYK35YA

=== Sending Initial Message ===
✓ Message sent
  Response: The available product listed is **ekoDB**, described as a high-performance database product, priced at **$99**.

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
✓ Created second session: oPGr_QJtuIow8MVvR0R1Fg7tk9aSoGFQh1feI35Ps7SiUSNcCgrezKC-XIJrqT9upGR2QXLBPkfq3dFvYzGZMw
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 7

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 6

=== Cleanup ===
✓ Deleted chat session: oPGr_QJtuIow8MVvR0R1Fg7tk9aSoGFQh1feI35Ps7SiUSNcCgrezKC-XIJrqT9upGR2QXLBPkfq3dFvYzGZMw
✓ Deleted chat session: UWngJK028EGWraZLGSftwLDsP2zL7SYHrjn_Gmy4iR_UOhsJCpau54dfKMtkc07-XYIRdkUEGVDF9ViaYK35YA
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: 13IdId8HKSLdpc4-JF99Jl9DJS92hQ4IIT0Oo1t7_u9T7Apt5W0-kxvMF1gIRu8pBmYkXooPd5BQFAOzglkgqw

=== Sending Chat Message ===
Message ID: lRboxAgvoeTSBAuX40xxte60lgfAbUjf52w9z5htAcYCAxG3yFM5tLA8g_IVxR3x6yNrTGYz2I43zz1Ovg932g

=== AI Response ===
- **ekoDB** — 99
- **ekoDB Pro** — 299
- **ekoDB Cloud** — 499

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    id: 'C1fy9c3WEr3CvqUDGMeknVSFn4OZKLX7ACPuyATX9raWR3e-4Kdt83Cv-Oh8woiiGzV9vzxlnhX2g3WV0fhLnw',
    price: 99,
    name: 'ekoDB',
    description: 'A high-performance database product with AI capabilities'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    description: 'Enterprise edition product with advanced features',
    price: 299,
    name: 'ekoDB Pro',
    id: 'qWBTpUJpl9xRZmmfGnU2qh0G8hd_1aPM9En-_nmcMToRzPMnCNQsdNXJ-rye6A2uzKE13uCz_CL3pLwqlgJ6sQ'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    price: 499,
    name: 'ekoDB Cloud',
    id: 'rga7z6D-lGPB_M6WpiVKdqjdIcixu6kQ1asQoq_pH_A5LE64zdc2tL3YTm0pXwhJmYfdiNJMoWT_fJVj6Ve94g',
    description: 'Fully managed cloud database service product'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 1976ms

=== Token Usage ===
Prompt tokens: 13718
Completion tokens: 64
Total tokens: 13782

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Message Stream (SSE) Example (TypeScript) ===

Created session: Gxr5IlHlncmrbPlqlaetLy54_fecxluPczj8hBgpnAnowo2vgEszBjrEvuboyfwjQL-v0FVBxO_sjQVQjVuTuw

Streaming response for: 'What is ekoDB?'

{"__progress":"received"}{"__progress":"generating","model":"gpt-6-luna","provider":"openai"}**EkoDB** usually refers to a decentralized database platform: instead of keeping application data on one centrally managed server, it aims to distribute storage across a network. It’s often presented for Web3 applications and data-ownership use cases.

The name may refer to different projects, so if you share a link or where you saw it, I can identify the specific one.

--- Stream complete ---
Message ID: UrgRksUkZL96S2KtIr29Ft8MJ3zeH4Qr0bU2blFc7Vug6pFFsMMv-4ziUXu8k2gu6_wp8crrLINYY8WFKhmuhg
Execution time: 6438ms
Context window: 128000 tokens

✓ Chat message stream example completed
✓ Client created

=== List Chat Models ===
Available chat models by provider:
  openai:
    - tts-1
    - gpt-5-mini
    - gpt-5-mini-2025-08-07
    - gpt-5.4-2026-03-05
    - gpt-5-nano
    - gpt-5.4-mini
    - omni-moderation-2024-09-26
    - gpt-live-transcribe
    - gpt-5.6-luna
    - gpt-4o-2024-11-20
    - gpt-5.1
    - whisper-1
    - gpt-5-pro
    - gpt-realtime-whisper
    - omni-moderation-latest
    - gpt-realtime-2
    - gpt-5-search-api
    - gpt-5.5-2026-04-23
    - gpt-5.5-pro
    - gpt-audio-2025-08-28
    - gpt-4o-mini-transcribe-2025-03-20
    - chat-latest
    - gpt-realtime-translate
    - gpt-5.4-pro-2026-03-05
    - gpt-4o-mini-transcribe-2025-12-15
    - gpt-5-pro-2025-10-06
    - gpt-5.6-sol
    - gpt-5.4-mini-2026-03-17
    - text-embedding-3-large
    - gpt-4o-2024-08-06
    - o3
    - gpt-5.4-pro
    - gpt-4o-mini-tts
    - gpt-realtime-mini-2025-12-15
    - gpt-realtime-2.1-mini
    - gpt-realtime-2025-08-28
    - tts-1-hd-1106
    - tts-1-hd
    - gpt-realtime-1.5
    - gpt-realtime
    - gpt-5.2-pro
    - gpt-4o-transcribe-diarize
    - gpt-5.1-2025-11-13
    - gpt-5.4
    - text-embedding-3-small
    - gpt-5-2025-08-07
    - o3-2025-04-16
    - gpt-5.2-2025-12-11
    - chatgpt-image-latest
    - tts-1-1106
    - gpt-5.5-pro-2026-04-23
    - gpt-5.2-pro-2025-12-11
    - gpt-5.4-nano
    - gpt-audio-mini-2025-12-15
    - gpt-audio-mini
    - gpt-4.1-mini-2025-04-14
    - gpt-5
    - gpt-4o-mini-transcribe
    - gpt-image-2-2026-04-21
    - gpt-transcribe
    - gpt-image-2
    - text-embedding-ada-002
    - gpt-realtime-2.1
    - gpt-4.1
    - gpt-5.5
    - gpt-audio-1.5
    - gpt-image-1.5
    - gpt-6.1-sol
    - gpt-6-sol
    - gpt-realtime-mini
    - gpt-image-1-mini
    - gpt-5.6-terra
    - gpt-5-nano-2025-08-07
    - gpt-4o-transcribe
    - gpt-4.1-mini
    - gpt-4o-mini
    - gpt-6.1-terra
    - gpt-6-luna
    - gpt-4o-mini-tts-2025-12-15
    - gpt-audio
    - gpt-5.3-codex
    - gpt-5-search-api-2025-10-14
    - gpt-4o
    - gpt-image-2.6-flare
    - gpt-4o-mini-2024-07-18
    - gpt-4.1-2025-04-14
    - gpt-image-2.5-flare
    - gpt-4o-mini-tts-2025-03-20
    - gpt-5.2
    - gpt-5.4-nano-2026-03-17
    - gpt-6-astra
    - gpt-image-2.5-sunburst-2026-09-08
    - gpt-image-2.5-sunburst
    - gpt-image-2.5-flare-2026-09-08
    - gpt-image-2.6-sunburst
    - gpt-live-1
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
    - gemini-omni-1.2-flash
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
  gemini: ok 61 models
  openai: ok 96 models
  perplexity: not_configured (unverified) No Perplexity API Key

=== Get Specific Provider Models ===
OpenAI models: tts-1, gpt-5-mini, gpt-5-mini-2025-08-07, gpt-5.4-2026-03-05, gpt-5-nano, gpt-5.4-mini, omni-moderation-2024-09-26, gpt-live-transcribe, gpt-5.6-luna, gpt-4o-2024-11-20, gpt-5.1, whisper-1, gpt-5-pro, gpt-realtime-whisper, omni-moderation-latest, gpt-realtime-2, gpt-5-search-api, gpt-5.5-2026-04-23, gpt-5.5-pro, gpt-audio-2025-08-28, gpt-4o-mini-transcribe-2025-03-20, chat-latest, gpt-realtime-translate, gpt-5.4-pro-2026-03-05, gpt-4o-mini-transcribe-2025-12-15, gpt-5-pro-2025-10-06, gpt-5.6-sol, gpt-5.4-mini-2026-03-17, text-embedding-3-large, gpt-4o-2024-08-06, o3, gpt-5.4-pro, gpt-4o-mini-tts, gpt-realtime-mini-2025-12-15, gpt-realtime-2.1-mini, gpt-realtime-2025-08-28, tts-1-hd-1106, tts-1-hd, gpt-realtime-1.5, gpt-realtime, gpt-5.2-pro, gpt-4o-transcribe-diarize, gpt-5.1-2025-11-13, gpt-5.4, text-embedding-3-small, gpt-5-2025-08-07, o3-2025-04-16, gpt-5.2-2025-12-11, chatgpt-image-latest, tts-1-1106, gpt-5.5-pro-2026-04-23, gpt-5.2-pro-2025-12-11, gpt-5.4-nano, gpt-audio-mini-2025-12-15, gpt-audio-mini, gpt-4.1-mini-2025-04-14, gpt-5, gpt-4o-mini-transcribe, gpt-image-2-2026-04-21, gpt-transcribe, gpt-image-2, text-embedding-ada-002, gpt-realtime-2.1, gpt-4.1, gpt-5.5, gpt-audio-1.5, gpt-image-1.5, gpt-6.1-sol, gpt-6-sol, gpt-realtime-mini, gpt-image-1-mini, gpt-5.6-terra, gpt-5-nano-2025-08-07, gpt-4o-transcribe, gpt-4.1-mini, gpt-4o-mini, gpt-6.1-terra, gpt-6-luna, gpt-4o-mini-tts-2025-12-15, gpt-audio, gpt-5.3-codex, gpt-5-search-api-2025-10-14, gpt-4o, gpt-image-2.6-flare, gpt-4o-mini-2024-07-18, gpt-4.1-2025-04-14, gpt-image-2.5-flare, gpt-4o-mini-tts-2025-03-20, gpt-5.2, gpt-5.4-nano-2026-03-17, gpt-6-astra, gpt-image-2.5-sunburst-2026-09-08, gpt-image-2.5-sunburst, gpt-image-2.5-flare-2026-09-08, gpt-image-2.6-sunburst, gpt-live-1

=== Get Anthropic Models ===
Anthropic models: claude-haiku-5-5, claude-sonnet-5-5, claude-opus-5-5, claude-fable-5-1, claude-opus-5, claude-sonnet-5, claude-fable-5, claude-opus-4-8, claude-opus-4-7, claude-sonnet-4-6, claude-opus-4-6, claude-opus-4-5-20251101, claude-haiku-4-5-20251001, claude-sonnet-4-5-20250929

=== Get Non-Existent Provider ===
Expected error for non-existent provider: Error: Request failed with status 404: {"error":"Unknown provider 'nonexistent_provider_xyz'","error_kind":"unknown_provider"}

✓ Chat Models example complete
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 1vKB38VkkJtWBjE0EIjhzYMIxxgJvd3sb3oHKBeTtBjtSR-1qk9RnGkNH1f5JGDuFPpLoEKoNnJheMOhI3SLyw

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is **ekoDB**, a high-performance database product, priced at **99**.

✓ Message 2 sent
  Response: ekoDB is priced at **99**. The currency isn’t specified.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: obOWxwYrzyRmDdwtsDhIoT0YNtvt2EVexVTB-g9G8AisoX-CvW45PoHIFzvdXRctkZ75bPf0J36yHhCNIXVQcQ
  Parent: 1vKB38VkkJtWBjE0EIjhzYMIxxgJvd3sb3oHKBeTtBjtSR-1qk9RnGkNH1f5JGDuFPpLoEKoNnJheMOhI3SLyw

=== Listing Sessions ===
✓ Found 2 sessions
  Session 1: obOWxwYrzyRmDdwtsDhIoT0YNtvt2EVexVTB-g9G8AisoX-CvW45PoHIFzvdXRctkZ75bPf0J36yHhCNIXVQcQ (Untitled)
  Session 2: 1vKB38VkkJtWBjE0EIjhzYMIxxgJvd3sb3oHKBeTtBjtSR-1qk9RnGkNH1f5JGDuFPpLoEKoNnJheMOhI3SLyw (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: obOWxwYrzyRmDdwtsDhIoT0YNtvt2EVexVTB-g9G8AisoX-CvW45PoHIFzvdXRctkZ75bPf0J36yHhCNIXVQcQ

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ All session management operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: lHxVzFZ3XfsVB1iQzlRUAKxJ3rGlj394IoU6UC4QfsFYZB7sDXKDG2FC-S4XMFOl46vWGTKXSCrejiybW2Aqvg

=== List Collections ===
Total collections: 27
Sample collections: schema_products_client_go,schema_documents_client_ts,chat_raw_completions__ek0_testing,chat_agent_configs__ek0_testing,schedules__ek0_testing

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
  POST /api/functions/conc_demo_pay_ts_75979_1794156993426 { "idempotency_key": "...", "amount": 100 }
  POST /api/functions/conc_demo_rl_fail_ts_75979_1794156993426 { "user_id": 42 }
  POST /api/functions/conc_demo_rl_skip_ts_75979_1794156993426 { "user_id": 42 }
  POST /api/functions/conc_demo_lock_ts_75979_1794156993426 { "resource": "queue:drain" }

✓ Cleaned up demo functions
=== ekoDB Convenience Methods Example ===

=== Native Object Creation ===
✓ Created record with plain object: {
  id: 'mvRUM_qhZE4YCrbrFqgk19SbyJoVYOG2Y9eYitJtTn1zc3khGCFuI5TJNgIk6jZ0tAjG_EOmnsVYLov046lZyw'
}

=== Upsert Operation ===
✓ First upsert (update): {
  id: 'mvRUM_qhZE4YCrbrFqgk19SbyJoVYOG2Y9eYitJtTn1zc3khGCFuI5TJNgIk6jZ0tAjG_EOmnsVYLov046lZyw',
  active: { type: 'Boolean', value: true },
  name: { type: 'String', value: 'Alice Johnson' },
  email: { type: 'String', value: 'alice.j@newdomain.com' },
  age: { type: 'Integer', value: 29 }
}
✓ Second upsert (insert): { id: 'new-user-id' }

=== Find One Operation ===
✓ Found user by email: {
  name: { type: 'String', value: 'Alice Johnson' },
  email: { value: 'alice.j@newdomain.com', type: 'String' },
  active: { type: 'Boolean', value: true },
  age: { type: 'Integer', value: 29 },
  id: 'mvRUM_qhZE4YCrbrFqgk19SbyJoVYOG2Y9eYitJtTn1zc3khGCFuI5TJNgIk6jZ0tAjG_EOmnsVYLov046lZyw'
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
✓ Inserted document: E-Y-XkOVEWkQ2bUuGQTZxJJqQRw3CGvbT_vVI3fuzvvWuFmlgyRBUyMeRBdG2rZ87beBleXYHlHXhUzJU4apqA

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: Qaoly7BJhd71kTOScRq9UAL-NmBMDvTx3UTCrLabVA5bX4CTWfDBvQaIeHpi5AudIwujV4NxIfoQ83jhBoZhjA

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
✓ Edge cache script created: _pjD8gRK8zq4LVd0X-NK5BMCusWzyYuhlVj6dC-XpNgtHgBWOl2vVtDoVd-ZvPRet4hBZ3RWNNH8fQ82eKujYA

Call 1: Cache miss (fetches from API)
Response time: 54ms
Result: {
  "records": [
    {
      "value": {
        "value": {
          "id": 1,
          "email": "Sincere@april.biz",
          "phone": "1-770-736-8031 x56442",
          "website": "hildegard.org",
          "username": "Bret",
          "address": {
            "zipcode": "92998-3874",
            "suite": "Apt. 556",
            "city": "Gwenborough",
            "street": "Kulas Light",
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            }
          },
          "name": "Leanne Graham",
          "company": {
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona"
          }
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
Response time: 2ms (27x faster!)
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "id": 1,
          "email": "Sincere@april.biz",
          "phone": "1-770-736-8031 x56442",
          "website": "hildegard.org",
          "username": "Bret",
          "address": {
            "zipcode": "92998-3874",
            "suite": "Apt. 556",
            "city": "Gwenborough",
            "street": "Kulas Light",
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            }
          },
          "name": "Leanne Graham",
          "company": {
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona"
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
   Name: {"type":"String","value":"User 1"}
   Department: {"type":"String","value":"engineering"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 55ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "phone": "1-770-736-8031 x56442",
      "username": "Bret",
      "company": {
        "name": "Romaguera-Crona",
        "bs": "harness real-...

Second call (cache hit - from cache):
   ⏱️  Duration: 1ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "phone": "1-770-736-8031 x56442",
      "username": "Bret",
      "company": {
        "name": "Romaguera-Crona",
        "bs": "harness real-time e-markets",
     ...
   🚀 Cache speedup: 55.0x faster!

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
   {"count":{"value":5,"type":"Integer"},"category":{"type":"String","value":"Electronics"},"avg_price":{"type":"Float","value":367}}
   {"category":{"type":"String","value":"Furniture"},"count":{"type":"Integer","value":3},"avg_price":{"value":365.6666666666667,"type":"Float"}}
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
   Vector databases are designed to store and search **vector embeddings**—numerical representations of text, images, audio, or other data. Their main benefits include:

- **Semantic search:** Find items by meaning or similarity, not just exact keywords.
- **Fast similarity lookup at scale:** Specialized indexes can search large collections of vectors efficiently.
- **Richer retrieval:** Combine vector similarity with metadata filters, such as date, category, or access permissions.
- **AI application support:** They’re commonly used for retrieval-augmented generation (RAG), recommendations, image search, and deduplication.
- **Operational features:** Many provide tools for updating data, managing indexes, and scaling storage and search.

They’re most useful when similarity matters; for exact-match or simple structured queries, a traditional database may be a better fit.
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

✅ Function saved: mB8DhI8H21WfW_s1WFA79deo7FQpV7g_q_PbbLWjtj7AS09OGU91jF0uJIVpZKlWl1z4dCxbVI3k4hONPZ7d1w
📊 Found 2 product groups
   {"avg_price":{"value":575.6666666666666,"type":"Float"},"category":{"value":"Electronics","type":"String"},"count":{"value":3,"type":"Integer"}}
   {"avg_price":{"type":"Float","value":474},"category":{"value":"Furniture","type":"String"},"count":{"type":"Integer","value":2}}
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
   {"role":{"type":"String","value":"admin"},"avg_score":{"type":"Float","value":20},"count":{"value":3,"type":"Integer"}}
   {"count":{"type":"Integer","value":7},"avg_score":{"type":"Float","value":70},"role":{"type":"String","value":"user"}}
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

✅ Inserted order: vuVloonmM1RSRn4ROtMD9QQWAiszEA13iXpP7OEw5JVhei0OnkS3j9Ycp-VBdL14tABs1OgtQV3vpMN9BPsuIw
✅ Inserted 2 products with wrapped types

📝 Example 2: UserFunction with Wrapped Type Parameters

✅ Function saved: uYgcfi6nIANJzhofgJPJsI0MEuI7dswPclOhuBRH0BPY19687_o6z0L-Yt0XW8FC9EyRshOVv8wmydoi-JZwWA
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"type":"Object","value":{"role":"admin","userId":"user_abc"}}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: _120e6CRxMsNaCzF6ryTCL7NSLg2rQ_1m4Haa5lfSerAqkCfrd0jM7_882SbTT_VCPQvIzj5cb4I_-mU9DnEHw
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 3 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Function

✅ Function saved: 5dS0UEhs5k4bOU-fQflL9n04emmNpExtpozcFaHURqCNd2Fi-EM_dD4jtzpQakgtIE5Weasae9u0Sawx1ny32g
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
   1. Natural Language Processing (AI)
   2. Introduction to Machine Learning (AI)
   3. Database Design Principles (Database)
   4. Getting Started with ekoDB (Database)
   5. Vector Databases Explained (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   {"category":{"type":"String","value":"Database"},"count":{"type":"Integer","value":3}}
   {"count":{"type":"Integer","value":2},"category":{"type":"String","value":"AI"}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
🚀 ekoDB Functions Example (TypeScript)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

ℹ️  Function 'get_active_users_client_ts' already existed — updated instead
✅ Function saved: 1pdngGQZ-kjO_tpMvWyBJCM1Hkx1wcWStb4bLZRtEjmsiZige2_JrxbjQcfjd340sUpnwRX9hyilREBGb13pVQ
📊 Found 5 active users

📝 Example 2: Parameterized Function

✅ Function saved: pZz6TtBdnqugn_2jTX_Zofn_s3N6J5bKZd_7Ap2VHmYMiPDgH4b1mw4GtaK1C4E8ky2Wfw1Cv_tpcaG-ORdi4Q
📊 Found 3 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: ShhEm8NoxaDrEdSDqZBdiZ74WGxtMpTI6aBnX2K8fDCNhO_mNweIjAVK4ZihzFJZ076a7RfzwVEYj6_QGmMcXA
📊 Statistics: 2 groups
   {"status":{"type":"String","value":"active"},"count":{"value":5,"type":"Integer"},"avg_score":{"value":60,"type":"Float"}}
   {"count":{"type":"Integer","value":5},"avg_score":{"type":"Float","value":50},"status":{"type":"String","value":"inactive"}}

📝 Example 4: UserFunction Management

📋 Total scripts: 9
🔍 Retrieved script: Get Active Users
✏️  function updated
🗑️  function deleted

ℹ️  Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.

✅ All examples completed!
=== ekoDB Goal Template CRUD Example (TypeScript) ===

--- Creating goal template ---
Created template: Data Migration (id: 3eIW1oE54EkbuzxN7akoZzIockhcqYGcf6Ot8KXVp__ZPwaAJal4fY4BkReaJC5zCs1lZQYAmAFFlbb-1H-IMw)

--- Listing templates ---
Templates: {
  count: 1,
  items: [
    {
      description: [Object],
      id: '3eIW1oE54EkbuzxN7akoZzIockhcqYGcf6Ot8KXVp__ZPwaAJal4fY4BkReaJC5zCs1lZQYAmAFFlbb-1H-IMw',
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
Created goal: Deploy v2.0 (id: LQa6LqB0gQEpLZ0-4CkjnYQ6lr7vEb4P1tk9JqWxEgiZ4Ou1NxW7ngzSK0PFSVL3X1cdUtJnwDTbpNmbYrNXfQ)

--- Listing goals ---
Goals: {
  "count": 1,
  "goals": [
    {
      "created_at": {
        "type": "DateTime",
        "value": "2026-11-08T16:56:39.441132+00:00"
      },
      "description": {
        "type": "String",
        "value": "Ship version 2.0 to production"
      },
      "id": "LQa6LqB0gQEpLZ0-4CkjnYQ6lr7vEb4P1tk9JqWxEgiZ4Ou1NxW7ngzSK0PFSVL3X1cdUtJnwDTbpNmbYrNXfQ",
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
        "value": "2026-11-08T16:56:39.441132+00:00"
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
        "value": "2026-11-08T16:56:39.441132+00:00"
      },
      "description": {
        "type": "String",
        "value": "Ship version 2.0 with hot-fix patches"
      },
      "id": "LQa6LqB0gQEpLZ0-4CkjnYQ6lr7vEb4P1tk9JqWxEgiZ4Ou1NxW7ngzSK0PFSVL3X1cdUtJnwDTbpNmbYrNXfQ",
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
        "value": "2026-11-08T16:56:39.448694+00:00"
      }
    }
  ]
}

--- Goal step: start step 0 ---
Step 0 started on goal LQa6LqB0gQEpLZ0-4CkjnYQ6lr7vEb4P1tk9JqWxEgiZ4Ou1NxW7ngzSK0PFSVL3X1cdUtJnwDTbpNmbYrNXfQ
--- Goal step: complete step 0 ---
Step 0 completed on goal LQa6LqB0gQEpLZ0-4CkjnYQ6lr7vEb4P1tk9JqWxEgiZ4Ou1NxW7ngzSK0PFSVL3X1cdUtJnwDTbpNmbYrNXfQ
--- Goal step: fail step 1 ---
Step 1 failed on goal LQa6LqB0gQEpLZ0-4CkjnYQ6lr7vEb4P1tk9JqWxEgiZ4Ou1NxW7ngzSK0PFSVL3X1cdUtJnwDTbpNmbYrNXfQ

--- Completing goal ---
Goal status: pending_review
--- Approving goal ---
Goal status after approve: in_progress

--- Creating goal to reject ---
--- Rejecting goal ---
Goal status after reject: failed


--- Creating task ---
Created task: Hourly Health Check (id: -5-T84AZ1pu2vOuKA6A4E79ooS4H1-cdpn4oOHO4uaKqkPZ14hGPwJ0PAOxMSfb14zFnVz8W9BD4UczrMQGKQQ)

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
      "id": "-5-T84AZ1pu2vOuKA6A4E79ooS4H1-cdpn4oOHO4uaKqkPZ14hGPwJ0PAOxMSfb14zFnVz8W9BD4UczrMQGKQQ",
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
Created agent: SupportBot (id: BWrGPDRocUdSBqNqWqzVNWJBOZZH7LjpJr6zANtc077zIibFC9VhGyas6WbB8FzGkQpuTye4SxvZ_p2Wyrmfig)

--- Listing agents ---
Agents: {
  "count": 1,
  "items": [
    {
      "deployment_id": {
        "type": "String",
        "value": "deploy_prod_1"
      },
      "id": "BWrGPDRocUdSBqNqWqzVNWJBOZZH7LjpJr6zANtc077zIibFC9VhGyas6WbB8FzGkQpuTye4SxvZ_p2Wyrmfig",
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
By name: SupportBot (id: BWrGPDRocUdSBqNqWqzVNWJBOZZH7LjpJr6zANtc077zIibFC9VhGyas6WbB8FzGkQpuTye4SxvZ_p2Wyrmfig)

--- Updating agent ---
Updated agent: SupportBot

--- Getting agents by deployment ---
Agents in deployment: {
  "count": 0,
  "items": []
}
WARNING: agents-by-deployment omitted created agent BWrGPDRocUdSBqNqWqzVNWJBOZZH7LjpJr6zANtc077zIibFC9VhGyas6WbB8FzGkQpuTye4SxvZ_p2Wyrmfig; TODO: check/fix the server-side deployment lookup

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
  POST /api/functions/jwt_register_ts_76042_1794156999982 { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/jwt_login_ts_76042_1794156999982 { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/jwt_verify_ts_76042_1794156999982 { "token": "<jwt>" }

Set JWT_SECRET in ekoDB's environment_vars whitelist before invoking.

✓ Cleaned up demo functions
=== ekoDB KV Document Linking Example (TypeScript) ===

--- Setup: creating KV key and documents ---
Set KV key: session:admin
Inserted document 1: U6aXHFp9rSUT4SFFeeJtF77SauoWzk1Z_P25JfNNlk6kXRFI7Z-_Mn20k84fbbBPghM3B-1qLav9HfukFuKQEw
Inserted document 2: lE2cGG8nbRmBcjU9sVDuS2tVS-jYfVXiWeuMykGLe3DccloBNvdUkiZZFXvjtwcmaqFWJMdTI8QeR2wERsztDQ

--- Linking documents to KV key ---
Linked doc U6aXHFp9rSUT4SFFeeJtF77SauoWzk1Z_P25JfNNlk6kXRFI7Z-_Mn20k84fbbBPghM3B-1qLav9HfukFuKQEw: null
Linked doc lE2cGG8nbRmBcjU9sVDuS2tVS-jYfVXiWeuMykGLe3DccloBNvdUkiZZFXvjtwcmaqFWJMdTI8QeR2wERsztDQ: null

--- Getting links for KV key ---
Links: [
  {
    "collection": "kv_links_example_ts_76044_1794157000065",
    "document_id": "U6aXHFp9rSUT4SFFeeJtF77SauoWzk1Z_P25JfNNlk6kXRFI7Z-_Mn20k84fbbBPghM3B-1qLav9HfukFuKQEw",
    "field_path": null,
    "created_at": "2026-11-08T16:56:40.136893Z",
    "last_accessed": "2026-11-08T16:56:40.138463Z",
    "metadata": {}
  },
  {
    "collection": "kv_links_example_ts_76044_1794157000065",
    "document_id": "lE2cGG8nbRmBcjU9sVDuS2tVS-jYfVXiWeuMykGLe3DccloBNvdUkiZZFXvjtwcmaqFWJMdTI8QeR2wERsztDQ",
    "field_path": null,
    "created_at": "2026-11-08T16:56:40.137835Z",
    "last_accessed": "2026-11-08T16:56:40.138463Z",
    "metadata": {}
  }
]

--- Unlinking document ---
Unlinked doc lE2cGG8nbRmBcjU9sVDuS2tVS-jYfVXiWeuMykGLe3DccloBNvdUkiZZFXvjtwcmaqFWJMdTI8QeR2wERsztDQ: null

--- Verifying remaining links ---
Remaining links: [
  {
    "collection": "kv_links_example_ts_76044_1794157000065",
    "document_id": "U6aXHFp9rSUT4SFFeeJtF77SauoWzk1Z_P25JfNNlk6kXRFI7Z-_Mn20k84fbbBPghM3B-1qLav9HfukFuKQEw",
    "field_path": null,
    "created_at": "2026-11-08T16:56:40.136893Z",
    "last_accessed": "2026-11-08T16:56:40.139569Z",
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
Retrieved value: { type: 'Object', value: { username: 'john_doe', userId: 123 } }

=== KV Batch Set ===
✓ Batch set 3 keys
  kv_ops_ts_76046_1794157000231:cache:product:1: success
  kv_ops_ts_76046_1794157000231:cache:product:2: success
  kv_ops_ts_76046_1794157000231:cache:product:3: success

=== KV Batch Get ===
✓ Batch retrieved 3 values
  kv_ops_ts_76046_1794157000231:cache:product:1: { name: 'Product 1', price: 29.99 }
  kv_ops_ts_76046_1794157000231:cache:product:2: { price: 39.99, name: 'Product 2' }
  kv_ops_ts_76046_1794157000231:cache:product:3: { price: 49.99, name: 'Product 3' }

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
  kv_ops_ts_76046_1794157000231:cache:product:1: deleted
  kv_ops_ts_76046_1794157000231:cache:product:2: deleted
  kv_ops_ts_76046_1794157000231:cache:product:3: deleted

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
Response: In traditional art, the three primary colors are **red, yellow, and blue**. For light and screens, they’re **red, green, and blue (RGB)**.

--- Structured Output via SSE ---
JSON response: [
  {"name":"Earth","diameter_km":12742},
  {"name":"Mars","diameter_km":6779},
  {"name":"Jupiter","diameter_km":139820}
]

--- Blocking HTTP (for comparison) ---
Blocking response: Hello!

=== Done ===
=== ekoDB Schedule Management Example (TypeScript) ===

--- Creating schedule ---
Created schedule: Nightly Database Backup (id: 45bf27e0-9347-49ae-874b-aa55adb5a199, cron: 0 0 2 * * *)

--- Listing schedules ---
Schedules: {
  "count": 1,
  "schedules": [
    {
      "created_at": "2026-11-08T16:56:47.212294Z",
      "created_by": {
        "key_id": "3a0cdba0da7c060610042783cc758f1f",
        "key_label": "default_admin"
      },
      "cron_expression": "0 0 2 * * *",
      "description": null,
      "enabled": true,
      "function_label": "schedule_noop_typescript_76072_1794157007180",
      "id": "45bf27e0-9347-49ae-874b-aa55adb5a199",
      "last_execution": null,
      "name": "Nightly Database Backup",
      "next_execution": "2026-11-09T02:00:00Z",
      "parameters": {},
      "stats": {
        "avg_execution_time_ms": 0,
        "failed_executions": 0,
        "last_error": null,
        "successful_executions": 0,
        "total_executions": 0
      },
      "timezone": "UTC",
      "updated_at": "2026-11-08T16:56:47.212294Z"
    }
  ]
}

--- Getting schedule ---
Fetched: Nightly Database Backup (cron: 0 0 2 * * *)

--- Updating schedule ---
Updated: Nightly Full Backup (new cron: 0 0 3 * * *)

--- Triggering schedule ---
Trigger response: {
  "schedule_id": "45bf27e0-9347-49ae-874b-aa55adb5a199",
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
  4. Score: 13.200, Matched: bio, title

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
  1. Score: 0.772, Matched:
  2. Score: 0.757, Matched:
  3. Score: 0.755, Matched:

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.502, Matched: content, title
  2. Score: 0.903, Matched: content, title
  3. Score: 0.309, Matched:

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 13.200, Matched: skills, bio

9. Vector search with a metadata pre-filter (category = ml):
Found 2 documents in category "ml" (NLP excluded)
  1. Deep Learning Fundamentals (category: ml)
  2. Introduction to Machine Learning (category: ml)

Paired vector search: document=Rust Programming Answer, query=Rust Programming Question

✅ Search examples completed!
=== Cleanup ===
✅ Deleted test collections
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'tePisxITjXDEegCkvLZdopR5FmgMYBKeEWpNsZ1JrlwwSsc9p_3Pr-Aw6x-MbtfG1qri4MJJW8-vXfoPLTffOQ'
}

=== Find by ID ===
Found: {
  metadata: { type: 'Object', value: { nested: [Object], key: 'value' } },
  categories: { value: [ 'electronics', 'computers' ], type: 'Array' },
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] },
  tags: { value: [ 'tag1', 'tag2', 'tag3' ], type: 'Array' },
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  name: { value: 'Test Record', type: 'String' },
  value: { type: 'Integer', value: 42 },
  active: { value: true, type: 'Boolean' },
  created_at: { value: '2026-11-08T16:56:47.734+00:00', type: 'DateTime' },
  id: 'tePisxITjXDEegCkvLZdopR5FmgMYBKeEWpNsZ1JrlwwSsc9p_3Pr-Aw6x-MbtfG1qri4MJJW8-vXfoPLTffOQ',
  price: { type: 'Float', value: 99.99 },
  user_id: { value: '550e8400-e29b-41d4-a716-446655440000', type: 'String' }
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-11-08T16:56:47.734Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { nested: { deep: true }, key: 'value' }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  metadata: { nested: { deep: true }, key: 'value' },
  categories: [ 'electronics', 'computers' ],
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  tags: [ 'tag1', 'tag2', 'tag3' ],
  data: 'aGVsbG8gd29ybGQ=',
  name: 'Test Record',
  value: 42,
  active: true,
  created_at: '2026-11-08T16:56:47.734+00:00',
  id: 'tePisxITjXDEegCkvLZdopR5FmgMYBKeEWpNsZ1JrlwwSsc9p_3Pr-Aw6x-MbtfG1qri4MJJW8-vXfoPLTffOQ',
  price: 99.99,
  user_id: '550e8400-e29b-41d4-a716-446655440000'
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  price: { type: 'Float', value: 99.99 },
  embedding: { value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ], type: 'Array' },
  value: { value: 100, type: 'Integer' },
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  active: { type: 'Boolean', value: true },
  metadata: { type: 'Object', value: { nested: [Object], key: 'value' } },
  categories: { value: [ 'electronics', 'computers' ], type: 'Array' },
  name: { type: 'String', value: 'Updated Record' },
  created_at: { type: 'DateTime', value: '2026-11-08T16:56:47.734+00:00' },
  id: 'tePisxITjXDEegCkvLZdopR5FmgMYBKeEWpNsZ1JrlwwSsc9p_3Pr-Aw6x-MbtfG1qri4MJJW8-vXfoPLTffOQ',
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: Q0HfOcuodhOdhbORk-COVxX7oLdpfNDqe4JU6kIcvIyeLH1DaaNmAavoYzlB9t8RVg-UiXXxNtPSP3nX7XURWw

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
✓ Created native SWR script: github_user_native_ts (ghW1ixw8HQjbBiBvGCJZsp1Ixtmwcn3NZHq6NM7b1dttr2yvDr3cR2rlTTLrIBjj9DwcSo4xfMOcCOqM2gOdIA)

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
✓ Created SWR script with audit trail: product_swr_audit_ts (RFevTUUnpd0fcO3QuDqSmnNURZYc283cGTzp350uixCVEOG3h-eq2OIZBoU-U2meCGzSgLDUWGqk5jHkKql1Iw)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail_ts' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline_ts (1IpGw9ZTEk-HEyQWOoq3KZ3HpJ4qauicYtFNUNBlbDg-EFQqornZxxBfFTOS3eOzbW-fRRjhag4o8nA8z2CVug)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users_swr_ts' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache_ts (N3meY34C_4chiLGbDH1g2jiFpHV9o_IKUJn2UDoVkXx50A_YpOHWaOCtB8Gj6AlyhadX6riJe5dEdxsO3dXhpw)
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
✓ Created SWR script: fetch_api_user_ts_76092_1794157008856 (QUn8O5evddyT7axEggnzyxelUmQsv-4UsRLju7d5RPrZiKnKdJw6UUTMoslbr7ViuTbAyowAMMVDD-8IWCSujQ)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "value": {
          "phone": "1-770-736-8031 x56442",
          "username": "Bret",
          "company": {
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net"
          },
          "name": "Leanne Graham",
          "address": {
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            },
            "street": "Kulas Light",
            "suite": "Apt. 556",
            "zipcode": "92998-3874",
            "city": "Gwenborough"
          },
          "website": "hildegard.org",
          "id": 1,
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
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 2ms (served from cache)
Result (cached): {
  "records": [
    {
      "value": {
        "value": {
          "phone": "1-770-736-8031 x56442",
          "username": "Bret",
          "company": {
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net"
          },
          "name": "Leanne Graham",
          "address": {
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            },
            "street": "Kulas Light",
            "suite": "Apt. 556",
            "zipcode": "92998-3874",
            "city": "Gwenborough"
          },
          "website": "hildegard.org",
          "id": 1,
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
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_reviews_ts_76092_1794157008856 (TSW5xCPvTD1X29lJB-4HLCHcmOLW59_3C8-H1nyp8eWDLWLq1MlSnSogd3sDbEOYpQfmDpLxazHufx-vxv22JA)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {
      "value": {
        "value": {
          "stock": 99,
          "warrantyInformation": "1 week warranty",
          "weight": 4,
          "tags": [
            "beauty",
            "mascara"
          ],
          "id": 1,
          "meta": {
            "createdAt": "2025-10-09T14:47:01.588Z",
            "barcode": "5784719087687",
            "updatedAt": "2026-05-23T11:27:41.868Z",
            "qrCode": "https://cdn.dummyjson.com/public/qr-code.png"
          },
          "discountPercentage": 10.48,
          "category": "beauty",
          "dimensions": {
            "width": 15.14,
            "height": 13.08,
            "depth": 22.99
          },
          "minimumOrderQuantity": 48,
          "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
          "shippingInformation": "Ships in 3-5 business days",
          "title": "Essence Mascara Lash Princess",
          "sku": "BEA-ESS-ESS-001",
          "availabilityStatus": "In Stock",
          "brand": "Essence",
          "price": 9.99,
          "returnPolicy": "No return policy",
          "reviews": [
            {
              "rating": 3,
              "date": "2025-04-30T09:41:02.053Z",
              "reviewerEmail": "eleanor.collins@x.dummyjson.com",
              "reviewerName": "Eleanor Collins",
              "comment": "Would not recommend!"
            },
            {
              "reviewerEmail": "lucas.gordon@x.dummyjson.com",
              "comment": "Very satisfied!",
              "rating": 4,
              "reviewerName": "Lucas Gordon",
              "date": "2025-04-30T09:41:02.053Z"
            },
            {
              "date": "2025-04-30T09:41:02.053Z",
              "reviewerName": "Eleanor Collins",
              "reviewerEmail": "eleanor.collins@x.dummyjson.com",
              "comment": "Highly impressed!",
              "rating": 5
            }
          ],
          "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
          "images": [
            "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
          ],
          "rating": 2.56
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
Created Alice: $1000 - ID: ZOAGxbW4jRB3OzCn12sKnujflzw9dPHCTyovq4NC9j9q9bQCIQ_fdcPmmt97G9r7OBTU3100gv7NvX4VNKmAVw
Created Bob: $500 - ID: 8rQnU1fKNoOljxcyJsmyxh19IwOlDFxW2A4ImDatZQONAPjR38UoHvlE4WCsU485_hcMVevPZ2iPBAW2LuCOMw

=== Example 1: Begin Transaction ===
Transaction ID (server-default isolation): 46753eed-06fd-45e4-9c0e-c0e8f13e53ab

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
New transaction: 5b1b0a40-0ed3-4825-b9a9-2d4fe055c89f
Updated Bob: $700 → $600 (in transaction)
Status before rollback: Active
✓ Transaction rolled back

✓ Verified Bob remains $700 after rollback

=== Cleanup ===
✓ Deleted test account collection

✓ All client transaction examples completed
✓ Client created

=== Create User Function ===
Created user function with ID: 7up90GWJ5FxKSahMqTz_nnyyhJFZZPw8XENgRF_6Az0mBULqwTMlw1VxKLpx2jxQixfsZIj19IIAvbDulG3SRA

=== Get User Function ===
Retrieved: get_active_users_ts - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 9 user functions:
  - conc_demo_rl_fail_ts_72807_1794156869177: Rate-limit (fail mode)
  - get_active_users_ts: Get Active Users
  - get_active_users_client_ts: Get Active Users (Updated)
  - get_active_users_client_js: Get Active Users (Updated)
  - conc_demo_rl_skip_ts_75979_1794156993426: Rate-limit (skip mode)
  - fetch_product_reviews_ts_76092_1794157008856: Fetch Product with Reviews (Multi-API)
  - conc_demo_rl_fail_ts_75979_1794156993426: Rate-limit (fail mode)
  - fetch_product_reviews_ts_73755_1794156892739: Fetch Product with Reviews (Multi-API)
  - conc_demo_rl_skip_ts_72807_1794156869177: Rate-limit (skip mode)

=== Update User Function ===
User function updated successfully

=== Delete User Function ===
User function deleted successfully

✓ User Functions API example complete
=== WebSocket Chat Streaming Example (TypeScript) ===

Created chat session: XDzhK4MfhcFgJqRUjamWFzdF0uep1fiNqzUXmstUb28krGt1uLW8RhXr1AmAmp62HluuZLE_7ucisTlR11VJ6Q

Sending message: 'What is the capital of France?'
{"__progress":"received"}{"__progress":"generating","model":"gpt-5.6-luna","provider":"openai"}The capital of France is **Paris**.

--- Stream ended ---
Message ID: 18luysfN7sdCpSDNoaeHwlU_6D5cIWrVhVp6mUULq28YoFmswWMqCrx92X6v2UPD0aTrcbPtj9Mpy46W_8Vtpw
Execution time: 657ms
Token usage: {"cached_input_tokens":0,"completion_tokens":13,"prompt_tokens":23,"thinking_tokens":0,"total_tokens":36}

Full response: {"__progress":"received"}{"__progress":"generating","model":"gpt-5.6-luna","provider":"openai"}The capital of France is **Paris**....
=== WebSocket Subscription Example ===

✓ Authentication successful
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_ts' ===
✓ Subscribed (subscription_id: sub_19023f527cef42daa3f4d01e7f9c1828)

=== Performing mutations to trigger notifications ===
Inserting a record...
✓ Inserted record: RhUvQzy5bNDOa8WUBcFh-2cS8-N_3b-2i9tzcJCmMUSIurTeQaA-bYp83-CO9ZzFw8oMvb3glGnULTPzBBPgKA
  📡 Notification received for RhUvQzy5bNDOa8WUBcFh-2cS8-N_3b-2i9tzcJCmMUSIurTeQaA-bYp83-CO9ZzFw8oMvb3glGnULTPzBBPgKA

Inserting another record...
✓ Inserted record: ZCk58Q-l_x9pMllAoWr-6eYP1X8_ZTiQSa8GUb7Hg7JnrKgqK1xpldnS5UsWguTlzRT1SyJjjZanyyN1bEnNyA
  📡 Notification received for ZCk58Q-l_x9pMllAoWr-6eYP1X8_ZTiQSa8GUb7Hg7JnrKgqK1xpldnS5UsWguTlzRT1SyJjjZanyyN1bEnNyA

=== Unsubscribing ===
✓ Unsubscribed: {"collection":"ws_subscribe_example_ts","found":true,"unsubscribed":true}

✓ WebSocket subscription example completed successfully
✓ Deleted collection 'ws_subscribe_example_ts'
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: umeJwAVWYrNqs27DyL2hddP6jPhlTHa5bOlsnmQRbO8c84PCQBQIYs6qS50Oj47go8b-6s40PKUPQWJWt66yyQ

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
   Inserted with ripple: {"id":"WRnFlihQH3ll-acxbHQCmP0mTHUayJEAfxLTGZcshLcFLnGRTxhPges5Y7peMw24Q1TE9KxKEByjSe4Uvw98aw"}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {"id":"VNBOvqUW7jNsynx5YqROPeKqPEsEbpsyvTlPrWivkFLarL84t3NFZ5qyrJJVm3pusS2ibiVEGwFDhxYtakkKVQ"}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {"id":"WRnFlihQH3ll-acxbHQCmP0mTHUayJEAfxLTGZcshLcFLnGRTxhPges5Y7peMw24Q1TE9KxKEByjSe4Uvw98aw","name":{"value":"Product 1","type":"String"},"price":{"type":"Integer","value":150}}

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
  Fields returned: ["name","id","email"]
  First user: Dave Brown <dave@example.com>

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ["user_role","id","status","created_at","bio","avatar_url","age","email","name"]

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
    - Fields: ["bio","age","secret_token","id","created_at","email","avatar_url","name","status","api_key","user_role","password"]
  Projected query:
    - 3 fields per record
    - Fields: ["email","id","name"]
  Bandwidth savings: ~75% fewer fields

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
