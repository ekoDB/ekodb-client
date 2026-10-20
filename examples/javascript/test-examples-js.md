make test-examples-javascript
🧪 Running JavaScript examples (direct HTTP/WebSocket)...

added 1 package, removed 1 package, and audited 9 packages in 484ms

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
  id: 'Xf3mJyiP7ohsfms7rw9cYZt6X6oJTp-_WOxeUlMdPxbkWHwrBP__FmQZmv6Z1jpPq1Tvn8FRJUyk-WLivoJLSg'
}

=== Find by ID ===
Found: {
  value: { type: 'Integer', value: 42 },
  name: { type: 'String', value: 'Test Record' },
  id: 'Xf3mJyiP7ohsfms7rw9cYZt6X6oJTp-_WOxeUlMdPxbkWHwrBP__FmQZmv6Z1jpPq1Tvn8FRJUyk-WLivoJLSg',
  active: { type: 'Boolean', value: true }
}

=== Find with Query ===
Found documents: [
  {
    active: { type: 'Boolean', value: true },
    value: { value: 42, type: 'Integer' },
    name: { type: 'String', value: 'Test Record' },
    id: 'Xf3mJyiP7ohsfms7rw9cYZt6X6oJTp-_WOxeUlMdPxbkWHwrBP__FmQZmv6Z1jpPq1Tvn8FRJUyk-WLivoJLSg'
  }
]

=== Update Document ===
Updated: {
  name: { value: 'Updated Record', type: 'String' },
  id: 'Xf3mJyiP7ohsfms7rw9cYZt6X6oJTp-_WOxeUlMdPxbkWHwrBP__FmQZmv6Z1jpPq1Tvn8FRJUyk-WLivoJLSg',
  value: { value: 100, type: 'Integer' },
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
✓ Inserted test record: myf81OcLYuV07JOL-SvctJ88jRO8sZOlxX19pkGKtLILR2Ct0UqG-nz2nZzN9VcuUSW2OJXxm5eSyT4_4klx3g

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "type": "Success",
  "payload": {
    "data": [
      {
        "name": {
          "value": "WebSocket Test Record",
          "type": "String"
        },
        "active": {
          "type": "Boolean",
          "value": true
        },
        "value": {
          "value": 42,
          "type": "Integer"
        },
        "id": "myf81OcLYuV07JOL-SvctJ88jRO8sZOlxX19pkGKtLILR2Ct0UqG-nz2nZzN9VcuUSW2OJXxm5eSyT4_4klx3g"
      }
    ]
  },
  "messageId": "1792526731757"
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

✅ Function saved: aTOBOXy5zeFnMi-_DhzbqgLaOfdY-vmh1_3wJsKDynTByhnsUguhpdcWccZtYHRYIsQ2c6_H-Weck71nNxW6kg
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Function saved: Up5iK5ZTCcqpJHQPfSCIAVuVpJioHtdqbo9QGVbz56Gx8nplLb1LNQoXkTdsnc4pIYCBxmUNrV51BQ_9lGb7jQ
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: A_GAhBxoTu-NgE-M0VmKM-ugjLNSbbI5i-9R0ezWRNvNvjHAXU-cYtVd7f10qZP-h80MjFwqZrBOr2M3Nhk4Sg
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"count":{"type":"Integer","value":5},"avg_score":{"type":"Float","value":60},"status":{"type":"String","value":"active"},"max_score":{"value":100,"type":"Integer"}}
   {"count":{"type":"Integer","value":5},"avg_score":{"value":50,"type":"Float"},"max_score":{"type":"Integer","value":90},"status":{"value":"inactive","type":"String"}}

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
cache:product:1: { type: 'Object', value: { price: 29.99, name: 'Product 1' } }
cache:product:2: {
  value: { price: 39.989999999999995, name: 'Product 2' },
  type: 'Object'
}
cache:product:3: {
  value: { price: 49.989999999999995, name: 'Product 3' },
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
Collection created with first record: KA0hEkp9cc7QUptQSgW1sXxXyTyteUyFuL45VSo_CfMYeu_pkV0iJNrh4VkUU71YP2AKu8LMBjqFQKkxe-TQOQ

=== List Collections ===
Total collections: 22
Sample collections: [
  'schema_employees_client_ts',
  'chat_goals__ek0_testing',
  'chat_agent_configs__ek0_testing',
  'chat_tasks__ek0_testing',
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
Created Alice: $1000 - ID: 0Y4SKxLM9emWXNgv3a4BIWGDMyiWjHFtR4A5b3kYOw2dpqsZ6QQ1MEpOXQyEeErs6chBeMpnuldTZU8IWy8UtQ
Created Bob: $500 - ID: QcKMoJTUs6eWaJvx5HqYIF_-eOI6RMjQaGFoC6Ew7scrzr95mHVVZn2Ig6tx01sRUdntGzAsvzCOJLlKQs2zwg

=== Example 1: Begin Transaction ===
Transaction ID: d20bb1f9-044c-488b-a5cf-3189f5ec3201

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
New transaction: ad157a41-75cc-4997-b11e-2db1b8c76a50
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
   ✅ Function saved: xdzN_FHdbvMtEc4PmJKDePsAQ-fxo-lZujDrMwQcA0yCnnwMfPWnSqLRnttzDdlfI7Xe1vfRJ9koDMXrZslAeQ

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
   ✅ Function saved: nXR1Zc5egBEso3dRzkcJ5t_GmT3tDAiFNE8owyRY9G0hjkNHLfGG7SvgleAz5mILIAilIdzcngQx9hGpLsz9FQ

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
   ✅ Function saved: dKQHWe4aT0VxqzW1pK71HNjgJjJJw4RqB6r8e6QPp3PVZUsVo3X4NqndGW_-FVQaB2kf9gaJlzquAy5xN--IbQ

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
   ✅ Function saved: Rf8n-PoPxQ__D1NdV72DTUAC_zHFkO9g_PEPr2G09foQF4nd_VJkWgm8M2iyVOlgSd2zk_ozJ1smd8vGDV9-tA

2️⃣ Calling Function (Query + Delete + Verify)...
   ✅ Function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted function: xdzN_FHdbvMtEc4PmJKD...
   ✅ Deleted function: nXR1Zc5egBEso3dRzkcJ...
   ✅ Deleted function: dKQHWe4aT0VxqzW1pK71...
   ✅ Deleted function: Rf8n-PoPxQ__D1NdV72D...
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
  Output: Document ID = eiiJG0G-g1giro96vhQcSjxduKlDVqbnu6iJYCIqZVbtd2DvTaWjYFOzYuNTz5HQxZdg42H1oquxPztOhze5mA
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: findById(eiiJG0G-g1giro96vhQcSjxduKlDVqbnu6iJYCIqZVbtd2DvTaWjYFOzYuNTz5HQxZdg42H1oquxPztOhze5mA)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: findById(eiiJG0G-g1giro96vhQcSjxduKlDVqbnu6iJYCIqZVbtd2DvTaWjYFOzYuNTz5HQxZdg42H1oquxPztOhze5mA)
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
  Output: Document ID = 7lWVEf4B988__d1km2jzCVU0ZYevKz7TeSBh_4wTX9c5ySDYUUBSQFFGqm5oMwjJLbbbZ7tO8eLu4T-5zoMm-A
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: findById(7lWVEf4B988__d1km2jzCVU0ZYevKz7TeSBh_4wTX9c5ySDYUUBSQFFGqm5oMwjJLbbbZ7tO8eLu4T-5zoMm-A)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: findById(7lWVEf4B988__d1km2jzCVU0ZYevKz7TeSBh_4wTX9c5ySDYUUBSQFFGqm5oMwjJLbbbZ7tO8eLu4T-5zoMm-A)
  Output: Error (expected) - Request failed with status 404: {"error":"Record has been deleted"}
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

> @ekodb/ekodb-client@0.30.1 prepare
> npm run build


> @ekodb/ekodb-client@0.30.1 build
> tsc


up to date, audited 44 packages in 688ms

12 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.30.1 build
> tsc

✅ TypeScript client built!

added 1 package, removed 1 package, and audited 9 packages in 461ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities

added 1 package, removed 1 package, and audited 13 packages in 447ms

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
✓ Created session: 9YVGpqmGEa3PgNsJXZETuNBswRu4ajlOALMv6OIjM7T9yPg6rjoRT2AKSbu-V3vguSDLMY84ds5Snrxz1o4BJA

=== Sending Initial Message ===
✓ Message sent
  Response: It appears that there are currently no products available in the database that match the criteria for availability. The only product I found is `ekoDB`, but it is not listed as available.

If you have specific criteria or different parameters to search for, please let me know!

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
✓ Created second session: _BdioloqanbZNQOqp31ocHyQHRvpj-OKXBk25ETrIgLY0UfscjrKqfr5dvkLxeSB1aveAx6bLau_EOKHyGFAgg
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
✓ Created session: 46NYvo0vE8LlH7FeIt0-TMX20-Ib79WpmWL6rz716lsF30kONGm0F-sCGrBCCznSOCZc-shvjptSBnTGSz0KCg

=== Sending Chat Message ===
Message ID: mjHP9vog998PppcYJHsMInEsbh1GLzMJHsPkTAP_l47RQ_N-32Gkeef6BBddPv9g06VCv7v_IchsyQZqYzP-9Q

=== AI Response ===
The available products and their prices are as follows:

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
    name: 'ekoDB Cloud',
    description: 'Fully managed cloud database service product',
    id: 'FcjDRUl0791n5q881zHUdJ_gJMnzH3MyVwL92fJp3sHhCwl9lBPchZ7Ul0lDPJA4JrQS0iSejGbu1lm9iIfJuA',
    price: 499
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_js',
  record: {
    name: 'ekoDB',
    id: 'OS_TN3pQkdJzpVKFjpdmMoEfX2T78MlAZE4Yz4tuYBk4ngYAvmjixN1fd3tAWAPseU26FX9H-AldeBPEshtp2w',
    description: 'A high-performance database product with AI capabilities',
    price: 99
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_js',
  record: {
    price: 299,
    id: 'qXMB8WJAdd2llvesk40ltJzXBJOBWpHxGK6Mv3igOLy1tsgpEpcSXqS7fTNcQePDrDFIcfaB_m0nOnIMpJjDqA',
    description: 'Enterprise edition product with advanced features',
    name: 'ekoDB Pro'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 3131ms

=== Token Usage ===
Prompt tokens: 3413
Completion tokens: 90
Total tokens: 3503

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ Chat completed successfully
✓ Client created

=== List Chat Models ===
Available chat models by provider:
  openai:
    - gpt-4-turbo-2024-04-09
    - gpt-realtime-2.1-mini
    - gpt-realtime-2025-08-28
    - gpt-4
    - tts-1-hd
    - gpt-realtime-1.5
    - gpt-realtime
    - gpt-5.2-pro
    - gpt-5-chat-latest
    - gpt-3.5-turbo-0125
    - gpt-5.4-2026-03-05
    - gpt-5-nano
    - gpt-5.4-mini
    - o1-pro
    - gpt-5.1-codex
    - tts-1
    - gpt-5-mini
    - gpt-5-mini-2025-08-07
    - omni-moderation-2024-09-26
    - gpt-live-transcribe
    - gpt-5.6-luna
    - omni-moderation-latest
    - gpt-5-2025-08-07
    - o4-mini-2025-04-16
    - gpt-4o-2024-11-20
    - gpt-5.1
    - whisper-1
    - gpt-5-pro
    - gpt-realtime-whisper
    - gpt-5.5-pro
    - gpt-audio-2025-08-28
    - gpt-4o-mini-transcribe-2025-03-20
    - gpt-realtime-2
    - gpt-5-search-api
    - gpt-5.5-2026-04-23
    - chat-latest
    - gpt-realtime-translate
    - gpt-5.4-pro-2026-03-05
    - gpt-5.4-mini-2026-03-17
    - babbage-002
    - gpt-4o-2024-05-13
    - text-embedding-3-large
    - gpt-4o-mini-transcribe-2025-12-15
    - tts-1-1106
    - gpt-5.5-pro-2026-04-23
    - gpt-audio-mini-2025-10-06
    - gpt-5.2-pro-2025-12-11
    - gpt-4-turbo
    - gpt-4o-2024-08-06
    - o3
    - gpt-4o-mini-search-preview-2025-03-11
    - o3-mini-2025-01-31
    - gpt-audio-1.5
    - gpt-image-1.5
    - gpt-5-pro-2025-10-06
    - gpt-5.6-sol
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
    - gpt-4.1-mini
    - gpt-4o-mini
    - gpt-5.4-pro
    - gpt-4o-mini-tts
    - gpt-realtime-mini-2025-12-15
    - gpt-5.1-codex-max
    - gpt-realtime-mini
    - gpt-image-1-mini
    - gpt-5.6-terra
    - gpt-3.5-turbo-instruct-0914
    - o1-2024-12-17
    - sora-2
    - tts-1-hd-1106
    - gpt-4o-mini-tts-2025-12-15
    - gpt-image-1
    - gpt-audio
    - gpt-5.3-codex
    - gpt-5-search-api-2025-10-14
    - gpt-5
    - gpt-4o-mini-transcribe
    - gpt-image-2-2026-04-21
    - gpt-5.2-chat-latest
    - sora-2-pro
    - gpt-3.5-turbo-instruct
    - gpt-transcribe
    - gpt-4.1-nano-2025-04-14
    - o4-mini-deep-research
    - gpt-5.4
    - gpt-4o-search-preview-2025-03-11
    - text-embedding-3-small
    - gpt-5.1-chat-latest
    - gpt-4o-mini-2024-07-18
    - gpt-4.1-2025-04-14
    - gpt-5.3-chat-latest
    - o3-2025-04-16
    - gpt-5.2-2025-12-11
    - chatgpt-image-latest
    - gpt-3.5-turbo
    - gpt-5.2-codex
    - gpt-4o
    - davinci-002
    - gpt-5.4-nano
    - o3-mini
    - o4-mini-deep-research-2025-06-26
    - gpt-audio-mini-2025-12-15
    - o1-pro-2025-03-19
    - gpt-5-nano-2025-08-07
    - gpt-4o-transcribe
    - o4-mini
    - gpt-5.1-codex-mini
    - gpt-3.5-turbo-1106
    - gpt-4o-mini-tts-2025-03-20
    - gpt-5.2
    - gpt-5.4-nano-2026-03-17
    - o1
    - gpt-image-2.5-sunburst-2026-09-08
    - gpt-6-luna
    - gpt-audio-mini
    - gpt-4.1-mini-2025-04-14
    - gpt-image-2.5-flare
    - gpt-4o-mini-search-preview
    - gpt-6-sol
    - gpt-6-astra
    - gpt-6.1-sol
    - gpt-live-1
    - gpt-image-2.5-sunburst
    - gpt-image-2.5-flare-2026-09-08
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
OpenAI models: gpt-4-turbo-2024-04-09, gpt-realtime-2.1-mini, gpt-realtime-2025-08-28, gpt-4, tts-1-hd, gpt-realtime-1.5, gpt-realtime, gpt-5.2-pro, gpt-5-chat-latest, gpt-3.5-turbo-0125, gpt-5.4-2026-03-05, gpt-5-nano, gpt-5.4-mini, o1-pro, gpt-5.1-codex, tts-1, gpt-5-mini, gpt-5-mini-2025-08-07, omni-moderation-2024-09-26, gpt-live-transcribe, gpt-5.6-luna, omni-moderation-latest, gpt-5-2025-08-07, o4-mini-2025-04-16, gpt-4o-2024-11-20, gpt-5.1, whisper-1, gpt-5-pro, gpt-realtime-whisper, gpt-5.5-pro, gpt-audio-2025-08-28, gpt-4o-mini-transcribe-2025-03-20, gpt-realtime-2, gpt-5-search-api, gpt-5.5-2026-04-23, chat-latest, gpt-realtime-translate, gpt-5.4-pro-2026-03-05, gpt-5.4-mini-2026-03-17, babbage-002, gpt-4o-2024-05-13, text-embedding-3-large, gpt-4o-mini-transcribe-2025-12-15, tts-1-1106, gpt-5.5-pro-2026-04-23, gpt-audio-mini-2025-10-06, gpt-5.2-pro-2025-12-11, gpt-4-turbo, gpt-4o-2024-08-06, o3, gpt-4o-mini-search-preview-2025-03-11, o3-mini-2025-01-31, gpt-audio-1.5, gpt-image-1.5, gpt-5-pro-2025-10-06, gpt-5.6-sol, gpt-4.1-nano, gpt-image-2, gpt-3.5-turbo-16k, text-embedding-ada-002, gpt-realtime-2.1, gpt-4.1, gpt-5.5, gpt-4o-transcribe-diarize, gpt-5-codex, gpt-4-0613, gpt-5.1-2025-11-13, gpt-4o-search-preview, gpt-4.1-mini, gpt-4o-mini, gpt-5.4-pro, gpt-4o-mini-tts, gpt-realtime-mini-2025-12-15, gpt-5.1-codex-max, gpt-realtime-mini, gpt-image-1-mini, gpt-5.6-terra, gpt-3.5-turbo-instruct-0914, o1-2024-12-17, sora-2, tts-1-hd-1106, gpt-4o-mini-tts-2025-12-15, gpt-image-1, gpt-audio, gpt-5.3-codex, gpt-5-search-api-2025-10-14, gpt-5, gpt-4o-mini-transcribe, gpt-image-2-2026-04-21, gpt-5.2-chat-latest, sora-2-pro, gpt-3.5-turbo-instruct, gpt-transcribe, gpt-4.1-nano-2025-04-14, o4-mini-deep-research, gpt-5.4, gpt-4o-search-preview-2025-03-11, text-embedding-3-small, gpt-5.1-chat-latest, gpt-4o-mini-2024-07-18, gpt-4.1-2025-04-14, gpt-5.3-chat-latest, o3-2025-04-16, gpt-5.2-2025-12-11, chatgpt-image-latest, gpt-3.5-turbo, gpt-5.2-codex, gpt-4o, davinci-002, gpt-5.4-nano, o3-mini, o4-mini-deep-research-2025-06-26, gpt-audio-mini-2025-12-15, o1-pro-2025-03-19, gpt-5-nano-2025-08-07, gpt-4o-transcribe, o4-mini, gpt-5.1-codex-mini, gpt-3.5-turbo-1106, gpt-4o-mini-tts-2025-03-20, gpt-5.2, gpt-5.4-nano-2026-03-17, o1, gpt-image-2.5-sunburst-2026-09-08, gpt-6-luna, gpt-audio-mini, gpt-4.1-mini-2025-04-14, gpt-image-2.5-flare, gpt-4o-mini-search-preview, gpt-6-sol, gpt-6-astra, gpt-6.1-sol, gpt-live-1, gpt-image-2.5-sunburst, gpt-image-2.5-flare-2026-09-08

=== Get Anthropic Models ===
Anthropic models: claude-haiku-5-5, claude-sonnet-5-5, claude-opus-5-5, claude-fable-5-1, claude-opus-5, claude-sonnet-5, claude-fable-5, claude-opus-4-8, claude-opus-4-7, claude-sonnet-4-6, claude-opus-4-6, claude-opus-4-5-20251101, claude-haiku-4-5-20251001, claude-sonnet-4-5-20250929

=== Get Non-Existent Provider ===
Expected error for non-existent provider: Error: Request failed with status 404: {"error":"Unknown provider 'nonexistent_provider_xyz'","error_kind":"unknown_provider"}

✓ Chat Models example complete
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: H2jbBt6jkz__-wRM4hHwxHrL0PQ7u1z2tsVjPKPF2xc6WtG9kL0fV-m-7BPldf1kveq5KMBQgzH2ENg2fK96gw

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: A high-performance database product
- **Price**: $99

If you have more questions or need further assistance, feel free to ask!

✓ Message 2 sent
  Response: The price of ekoDB is $99. If you have any more questions or need further assistance, feel free to ask!

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: bNufRHxuwvgpE42NXsSPTgmobjvaeXtDfOin17NDBGuPcc-86gkcrRPvJDNumBHASt5rqg-GJKKBEX9D6azKpA
  Parent: H2jbBt6jkz__-wRM4hHwxHrL0PQ7u1z2tsVjPKPF2xc6WtG9kL0fV-m-7BPldf1kveq5KMBQgzH2ENg2fK96gw

=== Listing Sessions ===
✓ Found 2 sessions
  Session 1: bNufRHxuwvgpE42NXsSPTgmobjvaeXtDfOin17NDBGuPcc-86gkcrRPvJDNumBHASt5rqg-GJKKBEX9D6azKpA (Untitled)
  Session 2: H2jbBt6jkz__-wRM4hHwxHrL0PQ7u1z2tsVjPKPF2xc6WtG9kL0fV-m-7BPldf1kveq5KMBQgzH2ENg2fK96gw (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: bNufRHxuwvgpE42NXsSPTgmobjvaeXtDfOin17NDBGuPcc-86gkcrRPvJDNumBHASt5rqg-GJKKBEX9D6azKpA

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ All session management operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: 1D2UHS_inm2drZpxVPVG_EpIoS7fX3tSM8_Gf2-qTyHa_kq1OQXXejOdYRg2DPwqhVOTwO6QdPMCkKDBm2ltDw

=== List Collections ===
Total collections: 23
Sample collections: schema_employees_client_ts,chat_goals__ek0_testing,client_collection_management_js,chat_agent_configs__ek0_testing,chat_tasks__ek0_testing

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: rjnpZ1ho1BKhtSpIzGH5LXyXmSPmhhI0AvQMtsu-5-S7DP4eABoaV83YPigff1ZxURqJPA8wM8jdSYjUQKxMWA

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: i8anBO6MnU7incUmH8-_K3nBvtDK6Zz1-VYPVwx4kCgRC_zXVSZrvviTx17p8FkfK4oln_uEkQxsq-0cL4QdTQ

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
✓ Edge cache script created: F7IUD7U2Ffg6BYtpCNsn_Hcuh__B1mN2YYWsR0RSWReVBj_qNIiQiN4BRtgAebXp4rwt9lDgxy52zE6zAiR8OA

Call 1: Cache miss (fetches from API)
Response time: 428ms
Result: {
  "records": [
    {
      "value": {
        "value": {
          "current": {
            "temperature_2m": 20.6,
            "interval": 900,
            "time": "2026-10-20T20:00"
          },
          "latitude": 40.710335,
          "longitude": -73.99308,
          "timezone_abbreviation": "GMT",
          "utc_offset_seconds": 0,
          "timezone": "GMT",
          "elevation": 32,
          "generationtime_ms": 0.028371810913085938,
          "current_units": {
            "time": "iso8601",
            "temperature_2m": "°C",
            "interval": "seconds"
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
Response time: 5ms (85.6x faster!)
Result: {
  "records": [
    {
      "value": {
        "value": {
          "current": {
            "temperature_2m": 20.6,
            "interval": 900,
            "time": "2026-10-20T20:00"
          },
          "latitude": 40.710335,
          "longitude": -73.99308,
          "timezone_abbreviation": "GMT",
          "utc_offset_seconds": 0,
          "timezone": "GMT",
          "elevation": 32,
          "generationtime_ms": 0.028371810913085938,
          "current_units": {
            "time": "iso8601",
            "temperature_2m": "°C",
            "interval": "seconds"
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
   Name: {"value":"User 1","type":"String"}
   Department: {"type":"String","value":"engineering"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 75ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "phone": "1-770-736-8031 x56442",
      "name": "Leanne Graham",
      "username": "Bret",
      "address": {
        "zipcode": "92998-3874",...

Second call (cache hit - from cache):
   ⏱️  Duration: 1ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "phone": "1-770-736-8031 x56442",
      "name": "Leanne Graham",
      "username": "Bret",
      "address": {
        "zipcode": "92998-3874",
        "geo": {
    ...
   🚀 Cache speedup: 75.0x faster!

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

✅ Function saved: H2QUOq7Oy6Js05FQNl98iYNOe8ugv5-gI_Ux_34f56ftnXS7g9PGeXHIsTQu-qkEl_f-sqT00yLN4uvbyim2XA
📊 Found 5 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Function

✅ Function saved
📊 Found 3 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Function

✅ Function saved
📊 Statistics: 2 groups
   {"count":{"value":5,"type":"Integer"},"avg_score":{"type":"Float","value":50},"status":{"type":"String","value":"inactive"}}
   {"avg_score":{"type":"Float","value":60},"count":{"value":5,"type":"Integer"},"status":{"value":"active","type":"String"}}
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
   1. USB-C Cable - $19 (⭐4.3)
   2. Standing Desk - $599 (⭐4.7)
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
   1. High Precision: Vector databases maintain a high degree of accuracy. The coordinates of points can be defined precisely in space, maintaining the original details.

2. Scalability: Vector data can be easily scaled. Regardless of how much you zoom in or out, vector data remains clear and undistorted.

3. Small Size: Vector files can often be smaller in size as compared to raster data since they store only vertices and paths information.

4. Easy to Edit: Vector data are easy to edit and manipulate. New elements can be added, and existing ones can be deleted or altered.

5. Selected Analysis: Vector databases are perfect for applications needing analysis on a selected discrete dataset, such as location of stores in a city.

6. Data Attributes: Associated attributes or metadata can be easily attached to the vector objects, allowing for rich, multi-dimensional data.

7. Better Representation: They offer a better representation for features like points (city locations, trees), lines (rivers, roads, rail tracks) and polygons (park boundaries, states).
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embedding function saved
📊 Generated embeddings for 3 articles
   1. "Getting Started with ekoDB" - 1536D vector
   2. "Advanced Query Patterns" - 1536D vector
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

✅ Function saved: IP5gCrEVb0MrgT-gOpNg_Jrw4LcB9j2hgS14MB-jGI15Tv1z3i1X65yjc0Ln0jHuCVs4NdcHmZgx8E5ehKj8FA
📊 Found 2 product groups
   {"avg_price":{"value":575.6666666666666,"type":"Float"},"category":{"type":"String","value":"Electronics"},"count":{"type":"Integer","value":3}}
   {"avg_price":{"type":"Float","value":474},"category":{"value":"Furniture","type":"String"},"count":{"value":2,"type":"Integer"}}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   {"category":{"type":"String","value":"Electronics"},"count":{"type":"Integer","value":3}}
   {"category":{"type":"String","value":"Furniture"},"count":{"value":2,"type":"Integer"}}
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
   inactive: 3 users
   active: 7 users
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
   3. User 7 - Score: 70
   4. User 2 - Score: 20
   5. User 8 - Score: 80
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

✅ Inserted order: PsR3YrwQwQ4J6jXkclnRZZEqSk_aQ0MOsQiXH7R1b38Or5fjbOqVJmTWgXzoDaw4AymOXdy18h5f7J1BD-bQYA
✅ Inserted 2 products with wrapped types

📝 Example 2: Function with Wrapped Type Parameters

✅ Function saved: Epx2lyywgzHCeOO40pOUEq6HJbGFJFq8iOkLMMwsrDjvZ_FooZ7IrgWgdm1-42GkfxphE-iKWxNhjiR1fzP1kQ
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"value":{"role":"admin","userId":"user_abc"},"type":"Object"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: IJ3EGI3inryy1zBX9XbLUR2RdqKEUA_lCVCGBX8--Z-rTPfjiyiiYsQew7XcmDqIR_vQdYVpgMchyeJH60abgQ
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 3 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Function

✅ Function saved: pxw6WQ1Hlq8m8AyevzUqkOEkuTD7cFvAfGoWE45w9ogpi47squKTR6p0NVw4B2FB93Qx920wRfboUPXeK-xi-w
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
   2. Natural Language Processing (AI)
   3. Getting Started with ekoDB (Database)
   4. Vector Databases Explained (Database)
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
   2. Natural Language Processing
   3. Getting Started with ekoDB
   4. Vector Databases Explained
   5. Database Design Principles
⏱️  Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Function saved
📊 Projected documents (showing first 3):
   1. Introduction to Machine Learning
   2. Natural Language Processing
   3. Getting Started with ekoDB
⏱️  Execution time: 0ms

📝 Example 5: All Document Fields

✅ Function saved
📊 All documents (5 total, showing first 2):
   1. Introduction to Machine Learning (AI)
   2. Natural Language Processing (AI)
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
Retrieved value: { type: 'Object', value: { username: 'john_doe', userId: 123 } }

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: { type: 'Object', value: { name: 'Product 1', price: 29.99 } }
cache:product:2: {
  type: 'Object',
  value: { price: 39.989999999999995, name: 'Product 2' }
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
  1. Score: 13.200, Matched: bio, title
  2. Score: 13.200, Matched: title, bio
  3. Score: 13.200, Matched: title, bio
  4. Score: 13.200, Matched: bio, title

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 26.400, Matched: title, bio
  2. Score: 26.400, Matched: bio, title
  3. Score: 26.400, Matched: bio, title
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
  2. Score: 0.755, Matched:
  3. Score: 0.729, Matched:

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.492, Matched: content, title
  2. Score: 0.909, Matched: title, content
  3. Score: 0.302, Matched:

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 13.200, Matched: bio, skills

=== Cleanup ===
✅ Deleted test collections

✅ Search examples completed!
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'TWLJuYxWt1f9k3ZvspUTi3TaKbbwkqXR9zF-bvSPg1cuB1JWtVme5hQYNM9SYt_lc1dTrOhVg_pHNlB-NfyKnQ'
}

=== Find by ID ===
Found: {
  name: { type: 'String', value: 'Test Record' },
  value: { type: 'Integer', value: 42 },
  active: { value: true, type: 'Boolean' },
  id: 'TWLJuYxWt1f9k3ZvspUTi3TaKbbwkqXR9zF-bvSPg1cuB1JWtVme5hQYNM9SYt_lc1dTrOhVg_pHNlB-NfyKnQ'
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  value: { type: 'Integer', value: 100 },
  id: 'TWLJuYxWt1f9k3ZvspUTi3TaKbbwkqXR9zF-bvSPg1cuB1JWtVme5hQYNM9SYt_lc1dTrOhVg_pHNlB-NfyKnQ',
  active: { value: true, type: 'Boolean' },
  name: { value: 'Updated Record', type: 'String' }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: G3QHyf45PDKcGp6XzG-fm97lxZwEiya_RVXSkyda_SSqUvnavdUViDPV3JFbYTDGxo_vt0q8urGqH7iXNfI6SQ

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
✓ Created native SWR script: github_user_native_js (vYBJhOPIw-dmsm465D58xPOKSTqkT1G-mp5MEItsYEwSue8LjIDsabqkFWa26A332OI7AznExCyKNwyvldZUBQ)

First call (cache miss - will fetch from GitHub API):
  Response time: 159ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 2ms
  Speedup: 79.5x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit_js (UT98bXwVJ4xXJkGk6VuFQef650MPrlYuQwAzqJPM44kZh5F6tP_uMUH211QuA2GwhelF_OrEtQivl3quLhlRNA)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail_js' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline_js (UZyRXB-vYHkkyMsvI148MqyTyB5rfsCotPFyhO758xWIUUmtBuinB3kpwg2rdAxyPApblmWtyKWPKZXrcZSTug)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users_swr_js' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache_js (BK2pRiYodFcfeERuGWRoEtgv-h0FtXHMyzYIDhAicaw_YGGDecEzJisKYdZgIuBniFxFNw2r396hssnNEXlWjQ)
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
✓ Created SWR script: fetch_api_user_js_39714_1792526808655 (N6boBq3JJXH7HMLrTNZlv9FXy3r9PugaPCHS3z8GhSi002WYe6NhK4KfOh9QkfEBbIOL0iS__m31gwoN2wZGvA)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "value": {
          "company": {
            "catchPhrase": "Multi-layered client-server neural-net",
            "bs": "harness real-time e-markets",
            "name": "Romaguera-Crona"
          },
          "website": "hildegard.org",
          "phone": "1-770-736-8031 x56442",
          "name": "Leanne Graham",
          "email": "Sincere@april.biz",
          "id": 1,
          "address": {
            "zipcode": "92998-3874",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "city": "Gwenborough",
            "suite": "Apt. 556",
            "street": "Kulas Light"
          },
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
Response time: 1ms (served from cache)
✓ Lightning fast cache hit

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation
🧹 Cleaning up...
✓ Cleanup complete

✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 7Z3nvoto4Aioppub3bhmkWWZp06kafUxJQhoou5RFCnqTia1i2UFbO3NKijaxiAGCsNo_L9AQ1-zY2KrBTDYsg
Created Bob: $500 - ID: 3w7LC8mHWh6izvDIQZe22x4fIotlLSHwZgmwSYcMSmXZJQ2FTd_-cY9-eXtJ3XHeXiQInib_-6oWDMiNrLreWQ

=== Example 1: Begin Transaction ===
Transaction ID (server-default isolation): e5c62eb8-80b7-4639-bb1b-8fe365f676db

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
New transaction: 554b0d96-bb1d-4c8e-acce-60daa5d0006d
Updated Bob: $700 → $600 (in transaction)
Status before rollback: Active
✓ Transaction rolled back

✓ Verified Bob remains $700 after rollback

=== Cleanup ===
✓ Deleted test account collection

✓ All client transaction examples completed
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: FjmVTkZOdQQmm7xKPHqRiZElmwohmq6rk66bFhyhQtc_FW1c2dlzv-FGtFL-ITCkGi2KcSKdzhhFgV855Lw9XQ

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
Inserted: bNLKWBzApjUuB0hyP_hrNPH6b_q8umZQdRZXmh-jjjIElNC4QeRsBqZMM19tio9aVqeAS3fz2t6Zr4DUHjUj0w

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
✓ Created session: qEzru4MncNDIkzFOCQLM2cRYvJagwMwXHMRUtpBnDUcaNRys0f-Z4unjyJi_lROGVdJFim9u3t9fJFvIeehllg

=== Sending Initial Message ===
✓ Message sent
  Response: It appears that there are currently no products marked as available in the database. The only product we have context for is "ekoDB," which has a price of $99, but it isn't explicitly indicated as available.

If there's anything else you'd like to know or if you want to perform a different search, please let me know!

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
✓ Created second session: rFTbR8ekEncOOTonIZzK7tAtUdGJzNBzaAfy7Ttt82PljIPDHixlhLeb8agtKMhuWBbnAYUt1EtVCkv-Y9A6Iw
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 7

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 6

=== Cleanup ===
✓ Deleted chat session: rFTbR8ekEncOOTonIZzK7tAtUdGJzNBzaAfy7Ttt82PljIPDHixlhLeb8agtKMhuWBbnAYUt1EtVCkv-Y9A6Iw
✓ Deleted chat session: qEzru4MncNDIkzFOCQLM2cRYvJagwMwXHMRUtpBnDUcaNRys0f-Z4unjyJi_lROGVdJFim9u3t9fJFvIeehllg
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: RjjqlbsMVMIoadFgm1AR5oFsUkshg_BOA8oucCOZC-yAxkP9MsfHF1WMQa7C_NUas9NNF-1oa2ZPAyREPAOU0Q

=== Sending Chat Message ===
Message ID: foPUGDz-bJskaLNg_egd4LtKFftYwjHFfF5VBWBAqjYjErroWpD-S2NItsoPZTSwgt7SebC_uBh-SYJzpwUxQA

=== AI Response ===
The available products and their prices are as follows:

1. **ekoDB**
   - Price: $99
   - Description: A high-performance database product with AI capabilities.

2. **ekoDB Pro**
   - Price: $299
   - Description: Enterprise edition product with advanced features.

3. **ekoDB Cloud**
   - Price: $499
   - Description: Fully managed cloud database service product.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    name: 'ekoDB',
    id: '4Sw7QBDkyLpanDqa7uPQYOTFS0rBbDIrhpRXwephpWtBKaniTx14136vM9xzrQFhvyczfTPAvgWXkzRThXNo3w',
    price: 99,
    description: 'A high-performance database product with AI capabilities'
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
    id: 'Vj2-xgR0TOS2j7MiS4iWL3R1fy_MHpAA7QCuoFFkFjOBHsS8Krsp232TQ_D96p5nD2DrNgKINZRkgoP87FtYtA'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    description: 'Fully managed cloud database service product',
    price: 499,
    id: 't7BHfTEB0GyvGcZugw5vitp7GuPKIhOg40JIt85NlauLWZCgtrDuSXl89xlFUi7oAOjWAmu4A_wKWJNfm2ZTUw',
    name: 'ekoDB Cloud'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 3406ms

=== Token Usage ===
Prompt tokens: 3413
Completion tokens: 93
Total tokens: 3506

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Message Stream (SSE) Example (TypeScript) ===

Created session: Mcr3wJMZM0n3LwB0maSHRgPECmYEqPV67Ev07tz4dLAzyFzJAzNN6Mpg4BXhjNHMjPfo5Mn96uWK14cbrS4oWw

Streaming response for: 'What is ekoDB?'

{"__progress":"received"}{"__progress":"generating","model":"default","provider":"openai"}**ekoDB** is a database and web portal focused on human gene essentiality and knockout/knockdown data. Its key features include:

- **Comprehensive Data Collection:** ekoDB compiles data from various genome-wide experimental studies, particularly those that use gene knockout (e.g., CRISPR, TALEN) or knockdown (e.g., RNAi) approaches in human cells. The database contains information about which genes are essential (required for cell survival and proliferation) or non-essential under specific conditions or in particular cell types.

- **Gene Essentiality:** Gene essentiality refers to whether loss of a gene impairs cell viability or has significant functional effects. ekoDB aggregates and standardizes essentiality data, helping researchers quickly determine which human genes are considered "essential" based on current experimental evidence.

- **User Tools and Visualization:** The web interface allows users to query specific genes, visualize essentiality scores, compare experimental datasets, and explore gene function and pathways associated with essential genes.

- **Applications:** ekoDB is widely used in genetics, functional genomics, drug target discovery, cancer research, and synthetic lethality studies.

**References:**
- Wang T, et al. "Gene Essentiality Profiling Reveals a Gene Network Underlying Cellular Fitness." *bioRxiv* (2017). [Link](https://www.biorxiv.org/content/10.1101/182691v1)
- [ekoDB official website](https://ekodb.genetics.ac.cn/) (if available)

**In summary:**
ekoDB is a valuable research resource for exploring which human genes are essential, based on large-scale genetic perturbation studies.

--- Stream complete ---
Message ID: Cavkn75GIy-0fvTpSvjZfkUH4sD8khuTBwg1bY5U5ddfyx6L2oZBRUFbQCrO5uqz9llOdrLfvD6a-f_gMnQtfw
Execution time: 4276ms
Context window: 1000000 tokens

✓ Chat message stream example completed
✓ Client created

=== List Chat Models ===
Available chat models by provider:
  openai:
    - gpt-4-turbo-2024-04-09
    - gpt-realtime-2.1-mini
    - gpt-realtime-2025-08-28
    - gpt-4
    - tts-1-hd
    - gpt-realtime-1.5
    - gpt-realtime
    - gpt-5.2-pro
    - gpt-5-chat-latest
    - gpt-3.5-turbo-0125
    - gpt-5.4-2026-03-05
    - gpt-5-nano
    - gpt-5.4-mini
    - o1-pro
    - gpt-5.1-codex
    - tts-1
    - gpt-5-mini
    - gpt-5-mini-2025-08-07
    - omni-moderation-2024-09-26
    - gpt-live-transcribe
    - gpt-5.6-luna
    - omni-moderation-latest
    - gpt-5-2025-08-07
    - o4-mini-2025-04-16
    - gpt-4o-2024-11-20
    - gpt-5.1
    - whisper-1
    - gpt-5-pro
    - gpt-realtime-whisper
    - gpt-5.5-pro
    - gpt-audio-2025-08-28
    - gpt-4o-mini-transcribe-2025-03-20
    - gpt-realtime-2
    - gpt-5-search-api
    - gpt-5.5-2026-04-23
    - chat-latest
    - gpt-realtime-translate
    - gpt-5.4-pro-2026-03-05
    - gpt-5.4-mini-2026-03-17
    - babbage-002
    - gpt-4o-2024-05-13
    - text-embedding-3-large
    - gpt-4o-mini-transcribe-2025-12-15
    - tts-1-1106
    - gpt-5.5-pro-2026-04-23
    - gpt-audio-mini-2025-10-06
    - gpt-5.2-pro-2025-12-11
    - gpt-4-turbo
    - gpt-4o-2024-08-06
    - o3
    - gpt-4o-mini-search-preview-2025-03-11
    - o3-mini-2025-01-31
    - gpt-audio-1.5
    - gpt-image-1.5
    - gpt-5-pro-2025-10-06
    - gpt-5.6-sol
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
    - gpt-4.1-mini
    - gpt-4o-mini
    - gpt-5.4-pro
    - gpt-4o-mini-tts
    - gpt-realtime-mini-2025-12-15
    - gpt-5.1-codex-max
    - gpt-realtime-mini
    - gpt-image-1-mini
    - gpt-5.6-terra
    - gpt-3.5-turbo-instruct-0914
    - o1-2024-12-17
    - sora-2
    - tts-1-hd-1106
    - gpt-4o-mini-tts-2025-12-15
    - gpt-image-1
    - gpt-audio
    - gpt-5.3-codex
    - gpt-5-search-api-2025-10-14
    - gpt-5
    - gpt-4o-mini-transcribe
    - gpt-image-2-2026-04-21
    - gpt-5.2-chat-latest
    - sora-2-pro
    - gpt-3.5-turbo-instruct
    - gpt-transcribe
    - gpt-4.1-nano-2025-04-14
    - o4-mini-deep-research
    - gpt-5.4
    - gpt-4o-search-preview-2025-03-11
    - text-embedding-3-small
    - gpt-5.1-chat-latest
    - gpt-4o-mini-2024-07-18
    - gpt-4.1-2025-04-14
    - gpt-5.3-chat-latest
    - o3-2025-04-16
    - gpt-5.2-2025-12-11
    - chatgpt-image-latest
    - gpt-3.5-turbo
    - gpt-5.2-codex
    - gpt-4o
    - davinci-002
    - gpt-5.4-nano
    - o3-mini
    - o4-mini-deep-research-2025-06-26
    - gpt-audio-mini-2025-12-15
    - o1-pro-2025-03-19
    - gpt-5-nano-2025-08-07
    - gpt-4o-transcribe
    - o4-mini
    - gpt-5.1-codex-mini
    - gpt-3.5-turbo-1106
    - gpt-4o-mini-tts-2025-03-20
    - gpt-5.2
    - gpt-5.4-nano-2026-03-17
    - o1
    - gpt-image-2.5-sunburst-2026-09-08
    - gpt-6-luna
    - gpt-audio-mini
    - gpt-4.1-mini-2025-04-14
    - gpt-image-2.5-flare
    - gpt-4o-mini-search-preview
    - gpt-6-sol
    - gpt-6-astra
    - gpt-6.1-sol
    - gpt-live-1
    - gpt-image-2.5-sunburst
    - gpt-image-2.5-flare-2026-09-08
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
OpenAI models: gpt-4-turbo-2024-04-09, gpt-realtime-2.1-mini, gpt-realtime-2025-08-28, gpt-4, tts-1-hd, gpt-realtime-1.5, gpt-realtime, gpt-5.2-pro, gpt-5-chat-latest, gpt-3.5-turbo-0125, gpt-5.4-2026-03-05, gpt-5-nano, gpt-5.4-mini, o1-pro, gpt-5.1-codex, tts-1, gpt-5-mini, gpt-5-mini-2025-08-07, omni-moderation-2024-09-26, gpt-live-transcribe, gpt-5.6-luna, omni-moderation-latest, gpt-5-2025-08-07, o4-mini-2025-04-16, gpt-4o-2024-11-20, gpt-5.1, whisper-1, gpt-5-pro, gpt-realtime-whisper, gpt-5.5-pro, gpt-audio-2025-08-28, gpt-4o-mini-transcribe-2025-03-20, gpt-realtime-2, gpt-5-search-api, gpt-5.5-2026-04-23, chat-latest, gpt-realtime-translate, gpt-5.4-pro-2026-03-05, gpt-5.4-mini-2026-03-17, babbage-002, gpt-4o-2024-05-13, text-embedding-3-large, gpt-4o-mini-transcribe-2025-12-15, tts-1-1106, gpt-5.5-pro-2026-04-23, gpt-audio-mini-2025-10-06, gpt-5.2-pro-2025-12-11, gpt-4-turbo, gpt-4o-2024-08-06, o3, gpt-4o-mini-search-preview-2025-03-11, o3-mini-2025-01-31, gpt-audio-1.5, gpt-image-1.5, gpt-5-pro-2025-10-06, gpt-5.6-sol, gpt-4.1-nano, gpt-image-2, gpt-3.5-turbo-16k, text-embedding-ada-002, gpt-realtime-2.1, gpt-4.1, gpt-5.5, gpt-4o-transcribe-diarize, gpt-5-codex, gpt-4-0613, gpt-5.1-2025-11-13, gpt-4o-search-preview, gpt-4.1-mini, gpt-4o-mini, gpt-5.4-pro, gpt-4o-mini-tts, gpt-realtime-mini-2025-12-15, gpt-5.1-codex-max, gpt-realtime-mini, gpt-image-1-mini, gpt-5.6-terra, gpt-3.5-turbo-instruct-0914, o1-2024-12-17, sora-2, tts-1-hd-1106, gpt-4o-mini-tts-2025-12-15, gpt-image-1, gpt-audio, gpt-5.3-codex, gpt-5-search-api-2025-10-14, gpt-5, gpt-4o-mini-transcribe, gpt-image-2-2026-04-21, gpt-5.2-chat-latest, sora-2-pro, gpt-3.5-turbo-instruct, gpt-transcribe, gpt-4.1-nano-2025-04-14, o4-mini-deep-research, gpt-5.4, gpt-4o-search-preview-2025-03-11, text-embedding-3-small, gpt-5.1-chat-latest, gpt-4o-mini-2024-07-18, gpt-4.1-2025-04-14, gpt-5.3-chat-latest, o3-2025-04-16, gpt-5.2-2025-12-11, chatgpt-image-latest, gpt-3.5-turbo, gpt-5.2-codex, gpt-4o, davinci-002, gpt-5.4-nano, o3-mini, o4-mini-deep-research-2025-06-26, gpt-audio-mini-2025-12-15, o1-pro-2025-03-19, gpt-5-nano-2025-08-07, gpt-4o-transcribe, o4-mini, gpt-5.1-codex-mini, gpt-3.5-turbo-1106, gpt-4o-mini-tts-2025-03-20, gpt-5.2, gpt-5.4-nano-2026-03-17, o1, gpt-image-2.5-sunburst-2026-09-08, gpt-6-luna, gpt-audio-mini, gpt-4.1-mini-2025-04-14, gpt-image-2.5-flare, gpt-4o-mini-search-preview, gpt-6-sol, gpt-6-astra, gpt-6.1-sol, gpt-live-1, gpt-image-2.5-sunburst, gpt-image-2.5-flare-2026-09-08

=== Get Anthropic Models ===
Anthropic models: claude-haiku-5-5, claude-sonnet-5-5, claude-opus-5-5, claude-fable-5-1, claude-opus-5, claude-sonnet-5, claude-fable-5, claude-opus-4-8, claude-opus-4-7, claude-sonnet-4-6, claude-opus-4-6, claude-opus-4-5-20251101, claude-haiku-4-5-20251001, claude-sonnet-4-5-20250929

=== Get Non-Existent Provider ===
Expected error for non-existent provider: Error: Request failed with status 404: {"error":"Unknown provider 'nonexistent_provider_xyz'","error_kind":"unknown_provider"}

✓ Chat Models example complete
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: y0m8Pi58AkfPqm0WLB5O_Lw2pWX7rmsNc5REXcXkWWmioz8Lz7LiqJIuK8BuWC4YSifPxVRK5NeGP-mGJ7H3LA

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
✓ Created branch: -xg0Q2q5kUryxfODZa0h7B1CX6kiHdme1qi6W9wl-A4REqekSt6T3bc86SrjBxMQ8eiya3xENt7p_DhkQGvCvg
  Parent: y0m8Pi58AkfPqm0WLB5O_Lw2pWX7rmsNc5REXcXkWWmioz8Lz7LiqJIuK8BuWC4YSifPxVRK5NeGP-mGJ7H3LA

=== Listing Sessions ===
✓ Found 2 sessions
  Session 1: -xg0Q2q5kUryxfODZa0h7B1CX6kiHdme1qi6W9wl-A4REqekSt6T3bc86SrjBxMQ8eiya3xENt7p_DhkQGvCvg (Untitled)
  Session 2: y0m8Pi58AkfPqm0WLB5O_Lw2pWX7rmsNc5REXcXkWWmioz8Lz7LiqJIuK8BuWC4YSifPxVRK5NeGP-mGJ7H3LA (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: -xg0Q2q5kUryxfODZa0h7B1CX6kiHdme1qi6W9wl-A4REqekSt6T3bc86SrjBxMQ8eiya3xENt7p_DhkQGvCvg

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ All session management operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: sRwpNXoOpSMCKm-RDHlGCm3F5bsMsBA03koxjmBauAvDDypHV6lJdlCyOCZ_cmbjqj-h1UQ1hM6I7SmYxxzWTA

=== List Collections ===
Total collections: 27
Sample collections: schema_employees_client_ts,chat_goals__ek0_testing,schema_documents_client_js,chat_agent_configs__ek0_testing,chat_tasks__ek0_testing

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
  POST /api/functions/conc_demo_pay_ts_70491_1792526834418 { "idempotency_key": "...", "amount": 100 }
  POST /api/functions/conc_demo_rl_fail_ts_70491_1792526834418 { "user_id": 42 }
  POST /api/functions/conc_demo_rl_skip_ts_70491_1792526834418 { "user_id": 42 }
  POST /api/functions/conc_demo_lock_ts_70491_1792526834418 { "resource": "queue:drain" }

✓ Cleaned up demo functions
=== ekoDB Convenience Methods Example ===

=== Native Object Creation ===
✓ Created record with plain object: {
  id: 'ICiAcNnUlM_cbBjmcotPttbv7IHBFQmCGMpkJQsEU6qYMy5VPVuvCOdoAbFwzOY2YO03ZZMR2Rf_Z9XQxEAhKQ'
}

=== Upsert Operation ===
✓ First upsert (update): {
  active: { type: 'Boolean', value: true },
  age: { type: 'Integer', value: 29 },
  email: { type: 'String', value: 'alice.j@newdomain.com' },
  name: { type: 'String', value: 'Alice Johnson' },
  id: 'ICiAcNnUlM_cbBjmcotPttbv7IHBFQmCGMpkJQsEU6qYMy5VPVuvCOdoAbFwzOY2YO03ZZMR2Rf_Z9XQxEAhKQ'
}
✓ Second upsert (insert): { id: 'new-user-id' }

=== Find One Operation ===
✓ Found user by email: {
  name: { value: 'Alice Johnson', type: 'String' },
  active: { type: 'Boolean', value: true },
  email: { value: 'alice.j@newdomain.com', type: 'String' },
  id: 'ICiAcNnUlM_cbBjmcotPttbv7IHBFQmCGMpkJQsEU6qYMy5VPVuvCOdoAbFwzOY2YO03ZZMR2Rf_Z9XQxEAhKQ',
  age: { type: 'Integer', value: 29 }
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
✓ Inserted document: yR1LLqLQ2I76VBDvhbr5DVQiQtTUEWq9sp0P9EnizoQ5zZnpApD0UI-ntF2hz-BabcguCizReJ7SuLcfgHUcZg

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: g-8SZ1MkvVndbfbpFBPhcYngwhtplYDGYafH8uyvPPtHrwMU4cso-JFPmF_J-9duDGzRokqoPJZjcqvhNNAYjg

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
✓ Edge cache script created: enh8NTr72FrEiMKv0AQ8dq6rQyUT-zEzqEHZb3LqZTOSoDp3vr0u0k6Wrv_QaJpyyJZzL0-Zmb7Yoqay-bQJtw

Call 1: Cache miss (fetches from API)
Response time: 72ms
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "name": "Leanne Graham",
          "username": "Bret",
          "company": {
            "catchPhrase": "Multi-layered client-server neural-net",
            "bs": "harness real-time e-markets",
            "name": "Romaguera-Crona"
          },
          "address": {
            "zipcode": "92998-3874",
            "street": "Kulas Light",
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            },
            "suite": "Apt. 556",
            "city": "Gwenborough"
          },
          "phone": "1-770-736-8031 x56442",
          "website": "hildegard.org",
          "email": "Sincere@april.biz",
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

Call 2: Cache hit (served from ekoDB)
Response time: 1ms (72x faster!)
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "name": "Leanne Graham",
          "username": "Bret",
          "company": {
            "catchPhrase": "Multi-layered client-server neural-net",
            "bs": "harness real-time e-markets",
            "name": "Romaguera-Crona"
          },
          "address": {
            "zipcode": "92998-3874",
            "street": "Kulas Light",
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            },
            "suite": "Apt. 556",
            "city": "Gwenborough"
          },
          "phone": "1-770-736-8031 x56442",
          "website": "hildegard.org",
          "email": "Sincere@april.biz",
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
   ⏱️  Duration: 53ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "phone": "1-770-736-8031 x56442",
      "company": {
        "bs": "harness real-time e-markets",
        "name": "Romaguera-Crona",
        "catchPhrase": "Multi-l...

Second call (cache hit - from cache):
   ⏱️  Duration: 1ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "phone": "1-770-736-8031 x56442",
      "company": {
        "bs": "harness real-time e-markets",
        "name": "Romaguera-Crona",
        "catchPhrase": "Multi-l...
   🚀 Cache speedup: 53.0x faster!

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

✅ Function saved: 7ie8qxvJ2K4o5tbMzCg68T3P-oINR6puxZXVG3E5QyheEbNvilSSCvrpQWd0F8XMm9JBmQKVh73V7xnX6ql97w
📊 Found 5 active users

📝 Example 2: Parameterized Function

✅ Function saved: pWdGUKxaiF3B4dCoDc9wlsnJS8R-e_cpb6TtxCpbH05AHrobQPeEIhUdXelDWu_Fjh_WYkjtilXeaAUBbg2__g
📊 Found 3 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: smt-n6WWcyDeroROcMk9qrFAUjLNYG3jRjblL8hpgM9mnqF4NzS58KgVRlximkos_Jcv5dqZbfEIdC1NjO2USQ
📊 Statistics: 2 groups
   {"count":{"type":"Integer","value":5},"status":{"value":"inactive","type":"String"},"avg_score":{"value":50,"type":"Float"}}
   {"status":{"value":"active","type":"String"},"avg_score":{"type":"Float","value":60},"count":{"type":"Integer","value":5}}

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
   {"category":{"value":"Furniture","type":"String"},"count":{"type":"Integer","value":3},"avg_price":{"type":"Float","value":365.6666666666667}}
   {"count":{"value":5,"type":"Integer"},"avg_price":{"value":367,"type":"Float"},"category":{"type":"String","value":"Electronics"}}
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

1. **Efficient Similarity Search**: They enable quick retrieval of similar items by computing distances in high-dimensional spaces, facilitating tasks like recommendation systems and image similarity.

2. **Scalability**: Designed to handle large datasets and high-dimensional vectors, vector databases can scale horizontally, accommodating growing amounts of data.

3. **Flexibility**: They support various data types and embedding techniques, making them suitable for applications like natural language processing and computer vision.

4. **Performance Optimization**: Many vector databases use indexing techniques such as HNSW or Annoy to speed up search times while maintaining accuracy.

5. **Real-time Processing**: They allow for fast querying and data insertion, making them ideal for applications requiring real-time responses.

6. **Integrated Machine Learning**: Vector databases often integrate seamlessly with machine learning workflows, supporting model training and deployment.

7. **Handling Unstructured Data**: They excel in managing unstructured data by converting it into vector representations, making it easier to analyze and retrieve.

8. **Support for Embeddings**: They can easily manage embeddings from various AI models, enabling tasks like semantic search and clustering.

Overall, vector databases are crucial for applications that need to perform similarity searches and handle high-dimensional data efficiently.
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

✅ Function saved: A_WuAc75DgVE-Xa1WQxwIFLX3tQAnnryUSLCbW-xEDFnToRYV96j2xQUkA9lvR_pYmA4QrxEPE_H4DDUE4XhGw
📊 Found 2 product groups
   {"count":{"type":"Integer","value":3},"avg_price":{"type":"Float","value":575.6666666666666},"category":{"type":"String","value":"Electronics"}}
   {"category":{"type":"String","value":"Furniture"},"avg_price":{"value":474,"type":"Float"},"count":{"value":2,"type":"Integer"}}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   {"category":{"value":"Furniture","type":"String"},"count":{"value":2,"type":"Integer"}}
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
   active: 7 users
   inactive: 3 users
⏱️  Execution time: 0ms

📝 Example 3: Average Score by Role

✅ Function saved
📊 Average score by role:
   {"count":{"value":7,"type":"Integer"},"role":{"type":"String","value":"user"},"avg_score":{"type":"Float","value":70}}
   {"avg_score":{"value":20,"type":"Float"},"role":{"value":"admin","type":"String"},"count":{"type":"Integer","value":3}}
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

✅ Inserted order: 0M-AaLOOC3oz09mWCXunK45X0qo86cNNmK-3onemLkE2itjT71oHlM1iJV7b-dIJMN1YsXfON6d31rqtTtgnRA
✅ Inserted 2 products with wrapped types

📝 Example 2: UserFunction with Wrapped Type Parameters

✅ Function saved: 9RvoGgeIbhWD46NO44G1o8BO5Dod7vbe6ga1ZTbwtip9yOcMjbxD3EVyNwry4NGOFW8X6gysbeJtk1y4HqUUGQ
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"value":{"userId":"user_abc","role":"admin"},"type":"Object"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: wfPAbVNY6wO_NietKIuGti7Tz_Nf_eg28Dl6h22Q9dIjjSimmIkKKcczEFr4L-TqUiwHdjtYmhFNNf9pGq4hlg
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 3 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Function

✅ Function saved: BCvE4s5plw21tznRmP-AiKPSFtC6bQeVU--MCa8CEStfNMPradqw5PcseYfkbQpMzUktQHI9fbHDii6NAiy4Lg
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
   2. Database Design Principles (Database)
   3. Vector Databases Explained (Database)
   4. Natural Language Processing (AI)
   5. Getting Started with ekoDB (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   {"count":{"type":"Integer","value":3},"category":{"value":"Database","type":"String"}}
   {"category":{"value":"AI","type":"String"},"count":{"type":"Integer","value":2}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
=== ekoDB Goal Template CRUD Example (TypeScript) ===

--- Creating goal template ---
Created template: Data Migration (id: llw8k-5AqDIRCwtQBOTa3Q2k3CsQ8mi0hukqjWvnsY1ekpY4QujPIYnG7aLLFw3sw2eBWyTpIVTD3qprLYckWg)

--- Listing templates ---
Templates: {
  count: 1,
  items: [
    {
      description: [Object],
      id: 'llw8k-5AqDIRCwtQBOTa3Q2k3CsQ8mi0hukqjWvnsY1ekpY4QujPIYnG7aLLFw3sw2eBWyTpIVTD3qprLYckWg',
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
Created goal: Deploy v2.0 (id: OVSvlbHoM-TNnQy3_yaAjk3Dg_8lqvBYz39pI24T9zv0jCqzzF8ckcPK1_xRcJ4vOoXw2KywTm7crwLKnbXDyw)

--- Listing goals ---
Goals: {
  "count": 1,
  "goals": [
    {
      "created_at": "2026-10-20T20:07:20.655004+00:00",
      "description": "Ship version 2.0 to production",
      "id": "OVSvlbHoM-TNnQy3_yaAjk3Dg_8lqvBYz39pI24T9zv0jCqzzF8ckcPK1_xRcJ4vOoXw2KywTm7crwLKnbXDyw",
      "status": "pending",
      "steps": "[{\"description\":\"Run test suite\"},{\"description\":\"Build release artifacts\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]",
      "title": "Deploy v2.0",
      "updated_at": "2026-10-20T20:07:20.655004+00:00"
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
        "value": "2026-10-20T20:07:20.655004+00:00"
      },
      "description": {
        "type": "String",
        "value": "Ship version 2.0 with hot-fix patches"
      },
      "id": "OVSvlbHoM-TNnQy3_yaAjk3Dg_8lqvBYz39pI24T9zv0jCqzzF8ckcPK1_xRcJ4vOoXw2KywTm7crwLKnbXDyw",
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
        "value": "2026-10-20T20:07:20.664626+00:00"
      }
    }
  ]
}

--- Goal step: start step 0 ---
Step 0 started on goal OVSvlbHoM-TNnQy3_yaAjk3Dg_8lqvBYz39pI24T9zv0jCqzzF8ckcPK1_xRcJ4vOoXw2KywTm7crwLKnbXDyw
--- Goal step: complete step 0 ---
Step 0 completed on goal OVSvlbHoM-TNnQy3_yaAjk3Dg_8lqvBYz39pI24T9zv0jCqzzF8ckcPK1_xRcJ4vOoXw2KywTm7crwLKnbXDyw
--- Goal step: fail step 1 ---
Step 1 failed on goal OVSvlbHoM-TNnQy3_yaAjk3Dg_8lqvBYz39pI24T9zv0jCqzzF8ckcPK1_xRcJ4vOoXw2KywTm7crwLKnbXDyw

--- Completing goal ---
Goal status: pending_review
--- Approving goal ---
Goal status after approve: in_progress

--- Creating goal to reject ---
--- Rejecting goal ---
Goal status after reject: failed


--- Creating task ---
Created task: Hourly Health Check (id: 2JoCmgYOOvmOROMd5vgA8i21bOQttx4ogYLU0eYaIP29Lb0M2MwYCkTdYL1PcpZWAFmTQTP18kV6rdiTGTly3A)

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
      "id": "2JoCmgYOOvmOROMd5vgA8i21bOQttx4ogYLU0eYaIP29Lb0M2MwYCkTdYL1PcpZWAFmTQTP18kV6rdiTGTly3A",
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
Created agent: SupportBot (id: cNc14qRQnKZRROjWMx5eIhH7WASyM0bWoRI4DEHre5bt390K915nSd4cfbUlBJFZOtxtyjlgetstE-nbf2UhJQ)

--- Listing agents ---
Agents: {
  "count": 1,
  "items": [
    {
      "deployment_id": {
        "type": "String",
        "value": "deploy_prod_1"
      },
      "id": "cNc14qRQnKZRROjWMx5eIhH7WASyM0bWoRI4DEHre5bt390K915nSd4cfbUlBJFZOtxtyjlgetstE-nbf2UhJQ",
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
By name: SupportBot (id: cNc14qRQnKZRROjWMx5eIhH7WASyM0bWoRI4DEHre5bt390K915nSd4cfbUlBJFZOtxtyjlgetstE-nbf2UhJQ)

--- Updating agent ---
Updated agent: SupportBot

--- Getting agents by deployment ---
Agents in deployment: {
  "count": 0,
  "items": []
}
WARNING: agents-by-deployment omitted created agent cNc14qRQnKZRROjWMx5eIhH7WASyM0bWoRI4DEHre5bt390K915nSd4cfbUlBJFZOtxtyjlgetstE-nbf2UhJQ; TODO: check/fix the server-side deployment lookup

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
  POST /api/functions/jwt_register_ts_78518_1792526841109 { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/jwt_login_ts_78518_1792526841109 { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/jwt_verify_ts_78518_1792526841109 { "token": "<jwt>" }

Set JWT_SECRET in ekoDB's environment_vars whitelist before invoking.

✓ Cleaned up demo functions
=== ekoDB KV Document Linking Example (TypeScript) ===

--- Setup: creating KV key and documents ---
Set KV key: session:admin
Inserted document 1: _ox6sF-jDBjuaFFxRhlEzDpNnlAHU_QC5b8R3uBmrRIaYRoEoXWqYjH3HWQSrpZRUw25cWUwzxSWTrmVSUSfZw
Inserted document 2: VlfO28WAL-JdIY-dbnYNPWoiLvFp9sLOwgo38zZLWq3I7DqYfRWRA9qygaJfYFLpYq2ppashaqEIQM46sBNgMQ

--- Linking documents to KV key ---
Linked doc _ox6sF-jDBjuaFFxRhlEzDpNnlAHU_QC5b8R3uBmrRIaYRoEoXWqYjH3HWQSrpZRUw25cWUwzxSWTrmVSUSfZw: null
Linked doc VlfO28WAL-JdIY-dbnYNPWoiLvFp9sLOwgo38zZLWq3I7DqYfRWRA9qygaJfYFLpYq2ppashaqEIQM46sBNgMQ: null

--- Getting links for KV key ---
Links: [
  {
    "collection": "kv_links_example_ts_78669_1792526841196",
    "document_id": "_ox6sF-jDBjuaFFxRhlEzDpNnlAHU_QC5b8R3uBmrRIaYRoEoXWqYjH3HWQSrpZRUw25cWUwzxSWTrmVSUSfZw",
    "field_path": null,
    "created_at": "2026-10-20T20:07:21.276619Z",
    "last_accessed": "2026-10-20T20:07:21.279357Z",
    "metadata": {}
  },
  {
    "collection": "kv_links_example_ts_78669_1792526841196",
    "document_id": "VlfO28WAL-JdIY-dbnYNPWoiLvFp9sLOwgo38zZLWq3I7DqYfRWRA9qygaJfYFLpYq2ppashaqEIQM46sBNgMQ",
    "field_path": null,
    "created_at": "2026-10-20T20:07:21.277851Z",
    "last_accessed": "2026-10-20T20:07:21.279357Z",
    "metadata": {}
  }
]

--- Unlinking document ---
Unlinked doc VlfO28WAL-JdIY-dbnYNPWoiLvFp9sLOwgo38zZLWq3I7DqYfRWRA9qygaJfYFLpYq2ppashaqEIQM46sBNgMQ: null

--- Verifying remaining links ---
Remaining links: [
  {
    "collection": "kv_links_example_ts_78669_1792526841196",
    "document_id": "_ox6sF-jDBjuaFFxRhlEzDpNnlAHU_QC5b8R3uBmrRIaYRoEoXWqYjH3HWQSrpZRUw25cWUwzxSWTrmVSUSfZw",
    "field_path": null,
    "created_at": "2026-10-20T20:07:21.276619Z",
    "last_accessed": "2026-10-20T20:07:21.282121Z",
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
Retrieved value: { value: { userId: 123, username: 'john_doe' }, type: 'Object' }

=== KV Batch Set ===
✓ Batch set 3 keys
  kv_ops_ts_78847_1792526841376:cache:product:1: success
  kv_ops_ts_78847_1792526841376:cache:product:2: success
  kv_ops_ts_78847_1792526841376:cache:product:3: success

=== KV Batch Get ===
✓ Batch retrieved 3 values
  kv_ops_ts_78847_1792526841376:cache:product:1: { name: 'Product 1', price: 29.99 }
  kv_ops_ts_78847_1792526841376:cache:product:2: { name: 'Product 2', price: 39.99 }
  kv_ops_ts_78847_1792526841376:cache:product:3: { name: 'Product 3', price: 49.99 }

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
  kv_ops_ts_78847_1792526841376:cache:product:1: deleted
  kv_ops_ts_78847_1792526841376:cache:product:2: deleted
  kv_ops_ts_78847_1792526841376:cache:product:3: deleted

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
Blocking response: Hello! I hope you're having a wonderful day.

=== Done ===
=== ekoDB Schedule Management Example (TypeScript) ===

--- Creating schedule ---
Created schedule: Nightly Database Backup (id: 464de3d4-b691-4362-956e-fbf8e02ab78f, cron: 0 0 2 * * *)

--- Listing schedules ---
Schedules: {
  "count": 1,
  "schedules": [
    {
      "created_at": "2026-10-20T20:07:24.312030Z",
      "cron_expression": "0 0 2 * * *",
      "description": null,
      "enabled": true,
      "function_label": "schedule_noop_typescript_82326_1792526844277",
      "id": "464de3d4-b691-4362-956e-fbf8e02ab78f",
      "last_execution": null,
      "name": "Nightly Database Backup",
      "next_execution": "2026-10-21T02:00:00Z",
      "parameters": {},
      "stats": {
        "avg_execution_time_ms": 0,
        "failed_executions": 0,
        "last_error": null,
        "successful_executions": 0,
        "total_executions": 0
      },
      "timezone": "UTC",
      "updated_at": "2026-10-20T20:07:24.312030Z"
    }
  ]
}

--- Getting schedule ---
Fetched: Nightly Database Backup (cron: 0 0 2 * * *)

--- Updating schedule ---
Updated: Nightly Full Backup (new cron: 0 0 3 * * *)

--- Triggering schedule ---
Trigger response: {
  "schedule_id": "464de3d4-b691-4362-956e-fbf8e02ab78f",
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
  3. Score: 13.200, Matched: bio, title
  4. Score: 13.200, Matched: title, bio

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 26.400, Matched: title, bio
  2. Score: 26.400, Matched: title, bio
  3. Score: 26.400, Matched: bio, title
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
  1. Score: 0.783, Matched:
  2. Score: 0.756, Matched:
  3. Score: 0.714, Matched:

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.513, Matched: title, content
  2. Score: 0.885, Matched: title, content
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
  id: 'SmTrUvRmT7qAkeXfnHpMSGBquVTLd0H1fPtJalnmTj_Qt4pEFeIVeVoNpVi7B1HpY81QK6GBPnlYPUbuvfLuxA'
}

=== Find by ID ===
Found: {
  name: { type: 'String', value: 'Test Record' },
  active: { type: 'Boolean', value: true },
  metadata: { type: 'Object', value: { nested: [Object], key: 'value' } },
  created_at: { type: 'DateTime', value: '2026-10-20T20:07:24.845+00:00' },
  tags: { value: [ 'tag1', 'tag2', 'tag3' ], type: 'Array' },
  categories: { type: 'Array', value: [ 'electronics', 'computers' ] },
  id: 'SmTrUvRmT7qAkeXfnHpMSGBquVTLd0H1fPtJalnmTj_Qt4pEFeIVeVoNpVi7B1HpY81QK6GBPnlYPUbuvfLuxA',
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' },
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  price: { type: 'Float', value: 99.99 },
  value: { value: 42, type: 'Integer' },
  embedding: { value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ], type: 'Array' }
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-10-20T20:07:24.845Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { nested: { deep: true }, key: 'value' }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  name: 'Test Record',
  active: true,
  metadata: { nested: { deep: true }, key: 'value' },
  created_at: '2026-10-20T20:07:24.845+00:00',
  tags: [ 'tag1', 'tag2', 'tag3' ],
  categories: [ 'electronics', 'computers' ],
  id: 'SmTrUvRmT7qAkeXfnHpMSGBquVTLd0H1fPtJalnmTj_Qt4pEFeIVeVoNpVi7B1HpY81QK6GBPnlYPUbuvfLuxA',
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  data: 'aGVsbG8gd29ybGQ=',
  price: 99.99,
  value: 42,
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  user_id: { value: '550e8400-e29b-41d4-a716-446655440000', type: 'String' },
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  value: { value: 100, type: 'Integer' },
  id: 'SmTrUvRmT7qAkeXfnHpMSGBquVTLd0H1fPtJalnmTj_Qt4pEFeIVeVoNpVi7B1HpY81QK6GBPnlYPUbuvfLuxA',
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  name: { type: 'String', value: 'Updated Record' },
  active: { value: true, type: 'Boolean' },
  price: { type: 'Float', value: 99.99 },
  embedding: { value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ], type: 'Array' },
  categories: { type: 'Array', value: [ 'electronics', 'computers' ] },
  metadata: { value: { nested: [Object], key: 'value' }, type: 'Object' },
  created_at: { type: 'DateTime', value: '2026-10-20T20:07:24.845+00:00' }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: 16umZM2RiGCwfIZGjv6ULjiiyeW86mFs8VsYq1m64fKkH0oRtL1BRsm1Ssbmr4BtHhaq9Tt9DnFGfj6SZlNsGg

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
✓ Created native SWR script: github_user_native_ts (MrJkSk-sa2VwESNo3ypeV9rldtUezAL-xUHxQDbKRb-SECzHFOIbeDm1Z5ZVjkOpXwTeUygtlFxkVJq6Wk0RVQ)

First call (cache miss - will fetch from GitHub API):
  Response time: 49ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 1ms
  Speedup: 49.0x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit_ts (fupSd_shvZWVnrCawKNqb3qMJk7L5iW4w4MPHtucvMO4RnBlfAd4arD0wuwm9cOAA2VmvvjObG4WF6UFCuecCA)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail_ts' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline_ts (VKocyNriM0OclvLSeuASFAkWgg-RZcS7WUxT9AkwPH16uoiAERHZjl7eu-mqacOrUkRlZw55QGsm0FTypPAk2Q)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users_swr_ts' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache_ts (F1vGJncTxEyBlbM3m9aAwCAdsyJhr4Bws_0RimR7_8VvGH1o4ai1l9nT5xnZB2mHoLxFiEECQEhAAL_X5Ehg0Q)
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
✓ Created SWR script: fetch_api_user_ts_84316_1792526845924 (N2qpgqIqBLp9EZeC5pr880hYVZMREduwAuqULfDITgZhBsCJl9Kw_Mb3JtUBcXjnxYhjvLuWipAPjjtGhsIbGw)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "company": {
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net"
          },
          "name": "Leanne Graham",
          "address": {
            "suite": "Apt. 556",
            "city": "Gwenborough",
            "street": "Kulas Light",
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            },
            "zipcode": "92998-3874"
          },
          "email": "Sincere@april.biz",
          "phone": "1-770-736-8031 x56442",
          "username": "Bret",
          "website": "hildegard.org",
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
Response time: 1ms (served from cache)
Result (cached): {
  "records": [
    {
      "value": {
        "value": {
          "company": {
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net"
          },
          "name": "Leanne Graham",
          "address": {
            "suite": "Apt. 556",
            "city": "Gwenborough",
            "street": "Kulas Light",
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            },
            "zipcode": "92998-3874"
          },
          "email": "Sincere@april.biz",
          "phone": "1-770-736-8031 x56442",
          "username": "Bret",
          "website": "hildegard.org",
          "id": 1
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
✓ Created enrichment script: fetch_product_reviews_ts_84316_1792526845924 (oKFP34Lq5Ny6nxgLsMXriyng0DJbJWZhxRgRn7F6gDoqxV1bXld9cwVICzOCV-ELcbqpGcGnFI1CL_6SCrymcw)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {
      "value": {
        "value": {
          "availabilityStatus": "In Stock",
          "shippingInformation": "Ships in 3-5 business days",
          "warrantyInformation": "1 week warranty",
          "price": 9.99,
          "weight": 4,
          "sku": "BEA-ESS-ESS-001",
          "reviews": [
            {
              "rating": 3,
              "date": "2025-04-30T09:41:02.053Z",
              "reviewerEmail": "eleanor.collins@x.dummyjson.com",
              "reviewerName": "Eleanor Collins",
              "comment": "Would not recommend!"
            },
            {
              "date": "2025-04-30T09:41:02.053Z",
              "rating": 4,
              "comment": "Very satisfied!",
              "reviewerEmail": "lucas.gordon@x.dummyjson.com",
              "reviewerName": "Lucas Gordon"
            },
            {
              "rating": 5,
              "comment": "Highly impressed!",
              "date": "2025-04-30T09:41:02.053Z",
              "reviewerName": "Eleanor Collins",
              "reviewerEmail": "eleanor.collins@x.dummyjson.com"
            }
          ],
          "category": "beauty",
          "title": "Essence Mascara Lash Princess",
          "dimensions": {
            "width": 15.14,
            "depth": 22.99,
            "height": 13.08
          },
          "discountPercentage": 10.48,
          "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
          "images": [
            "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
          ],
          "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
          "brand": "Essence",
          "id": 1,
          "meta": {
            "barcode": "5784719087687",
            "qrCode": "https://cdn.dummyjson.com/public/qr-code.png",
            "updatedAt": "2026-05-23T11:27:41.868Z",
            "createdAt": "2025-10-09T14:47:01.588Z"
          },
          "stock": 99,
          "tags": [
            "beauty",
            "mascara"
          ],
          "returnPolicy": "No return policy",
          "minimumOrderQuantity": 48,
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
Created Alice: $1000 - ID: ILnvyn9v6pspu3HWnE48fphMbxqWjNIoSzTwDsQijop76WinpR0glTPsA77j9jdYbjDqHwA9OcjGP_vCguUHyA
Created Bob: $500 - ID: JgA-Sy1VMKguAp9D29ucfwCiXbHH56dUuIlpxvrtQR2-zuwypj1j4BJgdvQAszfMzTVUoRNbN4xuZQZjSHi8qg

=== Example 1: Begin Transaction ===
Transaction ID (server-default isolation): 90a4cad4-f9cb-485c-9491-e91102926fb1

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
New transaction: 3c5d630e-3b75-4585-a179-18e395f7a6f0
Updated Bob: $700 → $600 (in transaction)
Status before rollback: Active
✓ Transaction rolled back

✓ Verified Bob remains $700 after rollback

=== Cleanup ===
✓ Deleted test account collection

✓ All client transaction examples completed
✓ Client created

=== Create User Function ===
Created user function with ID: 0GN1SCtK2mAT2lhge9TrfED21l9YGTarrwdjP2pvRMpOUMwJs04RAfRsTdB4k0tnPXCGeONEGxdf1s3dKk6y6A

=== Get User Function ===
Retrieved: get_active_users_ts - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 10 user functions:
  - get_active_users_ts: Get Active Users
  - conc_demo_rl_skip_ts_12240_1792526703280: Rate-limit (skip mode)
  - conc_demo_rl_fail_ts_12240_1792526703280: Rate-limit (fail mode)
  - get_active_users_client_js: Get Active Users (Updated)
  - conc_demo_rl_fail_ts_70491_1792526834418: Rate-limit (fail mode)
  - conc_demo_rl_skip_ts_70491_1792526834418: Rate-limit (skip mode)
  - fetch_product_reviews_ts_38404_1792526724530: Fetch Product with Reviews (Multi-API)
  - get_active_users_client_ts_updated: Get Active Users (Updated)
  - fetch_product_reviews_ts_84316_1792526845924: Fetch Product with Reviews (Multi-API)
  - get_active_users_client_ts_updated: Get Active Users (Updated)

=== Update User Function ===
User function updated successfully

=== Delete User Function ===
User function deleted successfully

✓ User Functions API example complete
=== WebSocket Chat Streaming Example (TypeScript) ===

Created chat session: Leu7iZ8HISwLM6fnGJnnswINfN8I8xn-Q0dfPHLmKdKmRuDZmmU9fEOsyLwh4JvGm41CJ1SOmMZYqNOCq-DmyA

Sending message: 'What is the capital of France?'
{"__progress":"received"}{"__progress":"generating","model":"default","provider":"openai"}The capital of France is Paris.

--- Stream ended ---
Message ID: 0L4al5f12d5S90L3DvJniTJzisxu4YN00cK59RvOs7_9Dubxybs1XINz7qEXGOnFoPX6lls8yl10ceuUbNxqmw
Execution time: 676ms
Token usage: {"completion_tokens":8,"prompt_tokens":15,"total_tokens":23}

Full response: {"__progress":"received"}{"__progress":"generating","model":"default","provider":"openai"}The capital of France is Paris....
=== WebSocket Subscription Example ===

✓ Authentication successful
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_ts' ===
✓ Subscribed (subscription_id: sub_881c3132b3644e6285daa6e7b5aef2d3)

=== Performing mutations to trigger notifications ===
Inserting a record...
✓ Inserted record: ma9gu24xVgTLZnXcehriPXp72SLioVRYtC3hTWaj33MOXVtuTaQuzeeNGAedSzVSVbtSxFpqClFVCpRm5OIy-g
  📡 Notification received for ma9gu24xVgTLZnXcehriPXp72SLioVRYtC3hTWaj33MOXVtuTaQuzeeNGAedSzVSVbtSxFpqClFVCpRm5OIy-g

Inserting another record...
✓ Inserted record: FnHtgjXazBsfC2Zxo7p8edk7f5hSwHL8f4tzz-35CFGgWmZiySvw-hz8yEmOVQWmAPwCLesaRfIsv7eBPFZv3g
  📡 Notification received for FnHtgjXazBsfC2Zxo7p8edk7f5hSwHL8f4tzz-35CFGgWmZiySvw-hz8yEmOVQWmAPwCLesaRfIsv7eBPFZv3g

=== Unsubscribing ===
✓ Unsubscribed: {"collection":"ws_subscribe_example_ts","found":true,"unsubscribed":true}

✓ WebSocket subscription example completed successfully
✓ Deleted collection 'ws_subscribe_example_ts'
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: a-t0zWfebHZuIpuxZ9bWmw11r5O-7nSM2xhGUbAQJ5J0TpSY5dn-jbRKwhqDuECVSdtDxMPjmgALMljJphw7zw

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
   Inserted with ripple: {"id":"dShvHVlBYOePf1wwbAPYkoIF7py1tNvGzMTx8EI6-ebl8Q20Xpg2ifkyo4KhUfixEzh-QoCUa_91OlX7rI9cPw"}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {"id":"n0BrLIhoWH_G7KCMGylpY6uD5vAXr154txAWruQ1kOWGE2y5geZjbyQyoF_-I5FrBrdo4CYtCut8SL13be53DQ"}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {"price":{"type":"Integer","value":150},"name":{"type":"String","value":"Product 1"},"id":"dShvHVlBYOePf1wwbAPYkoIF7py1tNvGzMTx8EI6-ebl8Q20Xpg2ifkyo4KhUfixEzh-QoCUa_91OlX7rI9cPw"}

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
  Fields returned: ["name","email","id"]
  First user: Bob Smith <bob@example.com>

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ["user_role","status","created_at","avatar_url","age","email","id","name","bio"]

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
    - Fields: ["avatar_url","api_key","age","created_at","bio","email","name","secret_token","id","password","status","user_role"]
  Projected query:
    - 3 fields per record
    - Fields: ["email","id","name"]
  Bandwidth savings: ~75% fewer fields

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
