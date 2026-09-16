make test-examples-javascript
🧪 Running JavaScript examples (direct HTTP/WebSocket)...

added 1 package, removed 1 package, and audited 9 packages in 456ms

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
  id: 'x7tITASD64-kKZnHuVx2Mbx0pNjnd4UO1QEO5Ztajzl_23RUiYPBpGytjeHLk5B-8JVAA2rz_qpESziumILJTg'
}

=== Find by ID ===
Found: {
  name: { value: 'Test Record', type: 'String' },
  id: 'x7tITASD64-kKZnHuVx2Mbx0pNjnd4UO1QEO5Ztajzl_23RUiYPBpGytjeHLk5B-8JVAA2rz_qpESziumILJTg',
  value: { value: 42, type: 'Integer' },
  active: { value: true, type: 'Boolean' }
}

=== Find with Query ===
Found documents: [
  {
    value: { type: 'Integer', value: 42 },
    id: 'x7tITASD64-kKZnHuVx2Mbx0pNjnd4UO1QEO5Ztajzl_23RUiYPBpGytjeHLk5B-8JVAA2rz_qpESziumILJTg',
    name: { type: 'String', value: 'Test Record' },
    active: { value: true, type: 'Boolean' }
  }
]

=== Update Document ===
Updated: {
  name: { type: 'String', value: 'Updated Record' },
  active: { value: true, type: 'Boolean' },
  value: { type: 'Integer', value: 100 },
  id: 'x7tITASD64-kKZnHuVx2Mbx0pNjnd4UO1QEO5Ztajzl_23RUiYPBpGytjeHLk5B-8JVAA2rz_qpESziumILJTg'
}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
✓ simple_crud.js completed successfully

=== Running ekoDB/ekodb-client/examples/javascript/simple_websocket.js ===
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: FCRdBhtGGeKuNwkxK1YjN0kHvaDainX21ZMM-4o6rYEyRkMalSYvVyjZC0_drW07k_Y2CaRdPhJl0mteTdUTDA

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "type": "Success",
  "payload": {
    "data": [
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "FCRdBhtGGeKuNwkxK1YjN0kHvaDainX21ZMM-4o6rYEyRkMalSYvVyjZC0_drW07k_Y2CaRdPhJl0mteTdUTDA",
        "value": {
          "value": 42,
          "type": "Integer"
        },
        "name": {
          "type": "String",
          "value": "WebSocket Test Record"
        }
      }
    ]
  },
  "messageId": "1789591605771"
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

✅ Function saved: O6wZizVODQ1dSNkaYmH9RxH83n9yhAqpeGfp8e1hgcys2vyQQjOksBloJgT07L9fZTR08Rp7jMjR3Xr7mwxHFg
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Function saved: hb6QRzh_39by1ujbA4wBGQsOu8L2nB_8tcw119V842T0nBn9b2yAxlAeZ60IiesaK_VLXWs0nkE4dCHscQNtpA
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: _B1c4bQrH6A1smGapydxmOnI3NNIoZw5fd6f1mJj6UPFO8RUL8Jk-LAaJyQlpnvUGZhuiwIyJy-GYwKnTARCHQ
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"status":{"type":"String","value":"active"},"avg_score":{"value":60,"type":"Float"},"count":{"type":"Integer","value":5},"max_score":{"type":"Integer","value":100}}
   {"status":{"type":"String","value":"inactive"},"max_score":{"type":"Integer","value":90},"avg_score":{"value":50,"type":"Float"},"count":{"type":"Integer","value":5}}

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
cache:product:1: { value: { price: 29.99, name: 'Product 1' }, type: 'Object' }
cache:product:2: {
  type: 'Object',
  value: { price: 39.989999999999995, name: 'Product 2' }
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
Collection created with first record: D0Sku9BH5LS6b-aWtwHwCynCsnl36qdkVUv1UfEd_ACc_YirZdv-5JQBrKKLnSo9w5i9gXs1nlmtXTMUDK8Nhg

=== List Collections ===
Total collections: 22
Sample collections: [
  'schema_users_client_ts',
  'demo_collection',
  'schema_documents_client_ts',
  'chat_goals__ek0_testing',
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
Created Alice: $1000 - ID: 161DAiP-k5_Eb4wz_TlT1N0EGA9ZymcJcUxUam_OWu2WlbJYRZm2rlIKB48dEr3RLTs7oLcWnKben9iktLdFtw
Created Bob: $500 - ID: f6c5GtQzQskov8caYZTBS0U4qnc7n5PPVbWSjBfDs7rFf_E2tHpkQBopMhP-DsNFsXorbR1YI1H7JVZMb23P6g

=== Example 1: Begin Transaction ===
Transaction ID: 31f07061-7323-4bdd-b7c0-1cd2dd6ab654

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
New transaction: e67f5b58-53cc-4c3c-ad55-009fc0253967
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
   ✅ Function saved: 2n0zM9-D_CdT23-2pNrEMmf49oCGxnQyj1zTWn1wXueElzBBB0KMfpwQtCKhJD4MSN-NL7lkHUeHNNUkvtVP0A

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
   ✅ Function saved: mO3Kvn0h5gQrBzLVj43uBbtSzs9pfDke_FlER6emnExzBiXhPfkM3tifGs5yH0ypvOYXntWhBM9kedQGvfIj0w

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
   ✅ Function saved: 35TyOE739fWO_cZioVF1o4X_bfGnMcDknOz51XwS_kxdxlony1MbaJiiIflxUjK5CIRjA7pZTFPbKN4cr1ie5w

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
   ✅ Function saved: p_j7ZKTVYXSe687cdk126un1frygcuozPyEPILMoPWktGkgRKMFWqR4QnXWhQuWQveWv3rCzb5F278lONXL0VA

2️⃣ Calling Function (Query + Delete + Verify)...
   ✅ Function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted function: 2n0zM9-D_CdT23-2pNrE...
   ✅ Deleted function: mO3Kvn0h5gQrBzLVj43u...
   ✅ Deleted function: 35TyOE739fWO_cZioVF1...
   ✅ Deleted function: p_j7ZKTVYXSe687cdk12...
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
  Output: Document ID = wxP6V9SxjIzFy25vAd5dQxOYS4ilI_hbUM18pfvKo63FcxmDbYMAqaf8V2tCAMomVJB6_de8x46qymtxSukErA
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: findById(wxP6V9SxjIzFy25vAd5dQxOYS4ilI_hbUM18pfvKo63FcxmDbYMAqaf8V2tCAMomVJB6_de8x46qymtxSukErA)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: findById(wxP6V9SxjIzFy25vAd5dQxOYS4ilI_hbUM18pfvKo63FcxmDbYMAqaf8V2tCAMomVJB6_de8x46qymtxSukErA)
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
  Output: Document ID = NKi0VqnseT0o-ZtNz_QxMyCkQ6WxWNQvouRc3A9mc7ubmL6G0gBxjG4hkQZh6ALXZpZ7rkAUXjte19kl1O6M-Q
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: findById(NKi0VqnseT0o-ZtNz_QxMyCkQ6WxWNQvouRc3A9mc7ubmL6G0gBxjG4hkQZh6ALXZpZ7rkAUXjte19kl1O6M-Q)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: findById(NKi0VqnseT0o-ZtNz_QxMyCkQ6WxWNQvouRc3A9mc7ubmL6G0gBxjG4hkQZh6ALXZpZ7rkAUXjte19kl1O6M-Q)
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


up to date, audited 46 packages in 561ms

12 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.26.4 build
> tsc

✅ TypeScript client built!

added 1 package, removed 1 package, and audited 9 packages in 380ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities

added 1 package, removed 1 package, and audited 13 packages in 392ms

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
✓ Created session: xYtgh5kITOPrj06iwmoIG1BATpvbFuQ72m3NmQ1hLw73FauuQS4l7V8kgN09nHmDzpyUNgLbwlm1ioTyXrARBQ

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** High-performance database product
- **Price:** $99

If you have any further questions about this product or others, feel free to ask!

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
✓ Created second session: 75mvYnnUmG7hfUiSPuo5d-CLNan_hpGOL6QzTzbDoQtxKuU6o03AJ84StW2iTtyc5ugdBbxQZLdY_GyhgIIH7A
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
✓ Created session: zQskMkECJb7UBUH73EzBLVnL3IWYZUp1sOMJbkIPLdW5yyOExZzjTfmNz0E147sOtnLyuj460LtgWka8vHBYpA

=== Sending Chat Message ===
Message ID: pdu-AOdgUf3b1_HT9nO5jtxhPiDDFBosid7vXnegYdzYwK5t3QrIE_hp2difEbVzU7dZ7_dns1Yiy51uMgj6tA

=== AI Response ===
Here are the available products and their prices:

1. **ekoDB Pro**
   - Price: $299
   - Description: Enterprise edition product with advanced features.

2. **ekoDB Cloud**
   - Price: $499
   - Description: Fully managed cloud database service product.

3. **ekoDB**
   - Price: $99
   - Description: A high-performance database product with AI capabilities.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_js',
  record: {
    price: 299,
    description: 'Enterprise edition product with advanced features',
    name: 'ekoDB Pro',
    id: 'i3Nma9XPdW7H3a46KIq_2zd30jSp4-sXcmylJw5VAyxwakFtU-6D-gcxW8W-blCcDbVF55Bj9WNI0mr8ESLkrA'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_js',
  record: {
    name: 'ekoDB Cloud',
    id: '4S-4lXl-XplQxMjQRoio_v1XU9xljf1vFJKfLnlw5hxe2skHbyE6EuiDhHyWGViNZaASmbFuIc0Wjq33ZFGllg',
    description: 'Fully managed cloud database service product',
    price: 499
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_js',
  record: {
    price: 99,
    description: 'A high-performance database product with AI capabilities',
    name: 'ekoDB',
    id: 'HkB_UmR4I53jIVwc9O1NUhzR7BYFQN8VwlkOYKjCAj2fZpBVO5eU9FspLJ-lkaAxUvOPGANciIH419RKcacjrQ'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 2890ms

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
✓ Created session: 00qamAZ7ySHioUYTZ5f7yV3Oqe3T3aRWzBoCUFO2BLutRgbiQVEQZzPcsbSXWdJXtNVgzWMSc5dICVJg7rOM0g

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: A high-performance database product
- **Price**: $99

If you need more information or have other questions, feel free to ask!

✓ Message 2 sent
  Response: The price of ekoDB is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: IX9_d9DbvxpXCHvPJXUoX56VKY6qoXlK1OZO6X0BpTFU5hBYKIhCZQrBCFb8a-CaE0jm1hUM9ExxksnNyNjx1Q
  Parent: 00qamAZ7ySHioUYTZ5f7yV3Oqe3T3aRWzBoCUFO2BLutRgbiQVEQZzPcsbSXWdJXtNVgzWMSc5dICVJg7rOM0g

=== Listing Sessions ===
✓ Found 2 sessions
  Session 1: IX9_d9DbvxpXCHvPJXUoX56VKY6qoXlK1OZO6X0BpTFU5hBYKIhCZQrBCFb8a-CaE0jm1hUM9ExxksnNyNjx1Q (Untitled)
  Session 2: 00qamAZ7ySHioUYTZ5f7yV3Oqe3T3aRWzBoCUFO2BLutRgbiQVEQZzPcsbSXWdJXtNVgzWMSc5dICVJg7rOM0g (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: IX9_d9DbvxpXCHvPJXUoX56VKY6qoXlK1OZO6X0BpTFU5hBYKIhCZQrBCFb8a-CaE0jm1hUM9ExxksnNyNjx1Q

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ All session management operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: bL0e4sWVWy2UrsB6ScEEYnEnLV_u2E9lQvna10f5s7GLhaROjKNBas1c6D7qtZknKuamozS3TaDm5umuMgrXvA

=== List Collections ===
Total collections: 23
Sample collections: schema_users_client_ts,schema_documents_client_ts,chat_goals__ek0_testing,schedules__ek0_testing,schema_products_client_go

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: Aei0SEHXt4uHRAmwUCLaLmHYLWhfizaJcQ7qkZ7irRiuy5c_6PGKhO_QnSmXt6iKecrbycMhIpOUwP2x7W-deg

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: sUS62vZfSFmnZTW1JtSp3Sn9uyBmtFCY0byHBcjl-82PcI7wj3J9YMiJRvvZHkb8WZfWy5s4u-bNXCZcjmwt7A

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
✓ Edge cache script created: 0eROV_tHzhHGlQKLwjCKdwDC1Gaw0DbiAS_vX1jQHDlnegTnnlK9CmKk2auQdOqMuq8TYDweZv11ezT6MwwNBg

Call 1: Cache miss (fetches from API)
Response time: 437ms
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "latitude": 40.710335,
          "generationtime_ms": 0.04088878631591797,
          "longitude": -73.99308,
          "current": {
            "time": "2026-09-16T20:45",
            "temperature_2m": 26,
            "interval": 900
          },
          "utc_offset_seconds": 0,
          "current_units": {
            "temperature_2m": "°C",
            "time": "iso8601",
            "interval": "seconds"
          },
          "elevation": 32,
          "timezone": "GMT",
          "timezone_abbreviation": "GMT"
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
Response time: 4ms (109.3x faster!)
Result: {
  "records": [
    {
      "value": {
        "value": {
          "latitude": 40.710335,
          "generationtime_ms": 0.04088878631591797,
          "longitude": -73.99308,
          "current": {
            "time": "2026-09-16T20:45",
            "temperature_2m": 26,
            "interval": 900
          },
          "utc_offset_seconds": 0,
          "current_units": {
            "temperature_2m": "°C",
            "time": "iso8601",
            "interval": "seconds"
          },
          "elevation": 32,
          "timezone": "GMT",
          "timezone_abbreviation": "GMT"
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
   ⏱️  Duration: 60ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "address": {
        "street": "Kulas Light",
        "zipcode": "92998-3874",
        "suite": "Apt. 556",
        "geo": {
          "lng": "81.1496",
          "...

Second call (cache hit - from cache):
   ⏱️  Duration: 2ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "address": {
        "street": "Kulas Light",
        "zipcode": "92998-3874",
        "suite": "Apt. 556",
        "geo": {
          "lng": "81.1496",
          "...
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

✅ Function saved: 374dG-zClN0tUw5aoxW9dps9RuoVO07FyZjvoQJuUTfvzceyT9IgMY75Tk2uY8MHu9hbFaS3fjj8Ykru9y8EPA
📊 Found 5 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Function

✅ Function saved
📊 Found 3 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Function

✅ Function saved
📊 Statistics: 2 groups
   {"count":{"value":5,"type":"Integer"},"avg_score":{"value":60,"type":"Float"},"status":{"value":"active","type":"String"}}
   {"count":{"value":5,"type":"Integer"},"avg_score":{"type":"Float","value":50},"status":{"type":"String","value":"inactive"}}
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
   2. Keyboard - $89 (⭐4.4)
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
   1. Precision: Vector data is more accurate and precise compared to raster data.

2. Scalability: Vector data is resolution-independent, so it can be resized and scaled without any loss of detail or accuracy.

3. Minimal Storage: It often requires less storage space than raster data.

4. Advanced Analysis: It's easier to apply more complex spatial analyses on vector data.

5. Clear Representation: Vector data often provides a more human-readable representation of features compared to raster data, making it ideal for mapping tasks.

6. Attribute Data: Vector data can store attribute data more effectively which is essential for spatial analysis and queries.
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embedding function saved
📊 Generated embeddings for 3 articles
   1. "Draft Article" - 1536D vector
   2. "Getting Started with ekoDB" - 1536D vector
   3. "Advanced Query Patterns" - 1536D vector
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

✅ Function saved: AS3aLiAwBlDYKYu-t0eJ5ov-SOAMAbc9hyIZSAT4l1iYHspy-HQdgbC-0R0ZNNIcE8hGOvX8kR4fujsdRsEzBQ
📊 Found 2 product groups
   {"avg_price":{"type":"Float","value":575.6666666666666},"category":{"type":"String","value":"Electronics"},"count":{"type":"Integer","value":3}}
   {"category":{"value":"Furniture","type":"String"},"count":{"type":"Integer","value":2},"avg_price":{"value":474,"type":"Float"}}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   {"category":{"value":"Furniture","type":"String"},"count":{"type":"Integer","value":2}}
   {"category":{"type":"String","value":"Electronics"},"count":{"type":"Integer","value":3}}
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
   1. User 4 - Score: 40
   2. User 1 - Score: 10
   3. User 9 - Score: 90
   4. User 8 - Score: 80
   5. User 5 - Score: 50
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

✅ Inserted order: 9rwgcOKm_rGHjodo7GdPInOTPw4XWVDDzfOyHdNa8YRWSirpJUA3GXMI0XRagiuZVQ5iJ78SfAgdNm7E6-3wgg
✅ Inserted 2 products with wrapped types

📝 Example 2: Function with Wrapped Type Parameters

✅ Function saved: EakaznPYU3LLVofSISyBxm59B9geIbqSUKiZtZktrukcGt_8dmjUHWKXQg6vKOmw7NpXaU1s9Iv86VFn6_RSCw
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"type":"Object","value":{"userId":"user_abc","role":"admin"}}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: PxJLHt73LYPRO2SfbyXyclIiA204lrzj6JdsY-jTn6P4vSpwKd9f-8W_NFHnVAG6yBAK0334zRfZCixAKCtb9A
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 3 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Function

✅ Function saved: EoNTzTNwdPAsZs5cePBqsMlGiA0KtRiLDC280CeLr6d_Ysi1ff3lgO6QUBUEdOSRYBiBzkGawkNq7SYtqDSaXA
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
   1. Natural Language Processing (AI)
   2. Getting Started with ekoDB (Database)
   3. Vector Databases Explained (Database)
   4. Database Design Principles (Database)
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
   1. Natural Language Processing
   2. Getting Started with ekoDB
   3. Vector Databases Explained
   4. Database Design Principles
   5. Introduction to Machine Learning
⏱️  Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Function saved
📊 Projected documents (showing first 3):
   1. Natural Language Processing
   2. Getting Started with ekoDB
   3. Vector Databases Explained
⏱️  Execution time: 0ms

📝 Example 5: All Document Fields

✅ Function saved
📊 All documents (5 total, showing first 2):
   1. Natural Language Processing (AI)
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
Retrieved value: { value: { userId: 123, username: 'john_doe' }, type: 'Object' }

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: { type: 'Object', value: { price: 29.99, name: 'Product 1' } }
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
  1. Score: 13.200, Matched: title, bio
  2. Score: 13.200, Matched: title, bio
  3. Score: 13.200, Matched: title, bio
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
  1. Score: 0.760, Matched:
  2. Score: 0.749, Matched:
  3. Score: 0.731, Matched:

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.492, Matched: title, content
  2. Score: 0.904, Matched: content, title
  3. Score: 0.299, Matched:

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 13.200, Matched: bio, skills

=== Cleanup ===
✅ Deleted test collections

✅ Search examples completed!
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'g5DdKtfwh2Mp1q_xd_5F_IJY6jSO6k1n5597646jzCx5cYz3Y5-6V2U8mBXiX-yBdaHKkusvpH-4RlgqnGD61A'
}

=== Find by ID ===
Found: {
  name: { type: 'String', value: 'Test Record' },
  value: { type: 'Integer', value: 42 },
  id: 'g5DdKtfwh2Mp1q_xd_5F_IJY6jSO6k1n5597646jzCx5cYz3Y5-6V2U8mBXiX-yBdaHKkusvpH-4RlgqnGD61A',
  active: { type: 'Boolean', value: true }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  value: { value: 100, type: 'Integer' },
  id: 'g5DdKtfwh2Mp1q_xd_5F_IJY6jSO6k1n5597646jzCx5cYz3Y5-6V2U8mBXiX-yBdaHKkusvpH-4RlgqnGD61A',
  name: { value: 'Updated Record', type: 'String' },
  active: { type: 'Boolean', value: true }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: bZFewdNT4mFxdOvE3QFWZqPerV6WPlfg5EM3Q2-THBjWl1pd9SwFuv_rf7Tl6QmKPw3rZXcbIFCN7P_hgOolkg

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
✓ Created native SWR script: github_user_native_js (BL95xkxWlDfEs5er6bsyyhppXGfzXuEbQKdmtiB5NZ8Av4E7wG58mySlVcWlq3-41KXZQB-igBx9OcHKOQ4jOA)

First call (cache miss - will fetch from GitHub API):
  Response time: 299ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 2ms
  Speedup: 149.5x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit_js (rKmqmT3tlZk_KIjLDvPFerWvK8DSjel_F6zyrWQSccLI8ELX4mizfUnDwpe2Meop2tUG05z3eyKdBbCVbE8aIw)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail_js' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline_js (q4bmq6b08ByVuqqBX1AmrI26J9R1YwMQT-akSRFzGIhN0h6UXNwGU00WSUqSaOZBcc5JrwjsAlvwwfR0X481Mw)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users_swr_js' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache_js (zEWPwrfGY1F-NjahfUvF5XJN-bEb0lRVujHB1SwI58FtyQiK1DDnv7CXKZARNGn3D_pdCu8te3gj18_Vw03tjQ)
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
✓ Created SWR script: fetch_api_user_js_70764_1789591679035 (8_Ul-STZU0abEP3MWQ9L8wx7ebtvjJEsaCV0SPkHMDKMf6x9qR7l3ckIVcslRHUWLiNnDUWIvkNBx8-OhZBZrA)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "phone": "1-770-736-8031 x56442",
          "website": "hildegard.org",
          "email": "Sincere@april.biz",
          "address": {
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            },
            "suite": "Apt. 556",
            "zipcode": "92998-3874",
            "street": "Kulas Light",
            "city": "Gwenborough"
          },
          "name": "Leanne Graham",
          "username": "Bret",
          "id": 1,
          "company": {
            "bs": "harness real-time e-markets",
            "name": "Romaguera-Crona",
            "catchPhrase": "Multi-layered client-server neural-net"
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
Created Alice: $1000 - ID: tDNsYrgnhC34mXCyNywU_SdFQOSGm5L8ba0ZyH6HDCclz187V_O6uPW8yoY5Xq3TqE9nFGogeEUcjlWxYQVEoQ
Created Bob: $500 - ID: 0fSFDn_Zl1YK1jXaB3UMRWdBbwesJqDJX9QVijys31aBuZqg5mon3qcMvQ6pfkZUBxg2hB_d5hPSfGSor52Ktg

=== Example 1: Begin Transaction ===
Transaction ID (server-default isolation): 6a34e9dc-9681-4e92-966b-5cdf4bcba965

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
New transaction: 472110e6-1950-43e6-bc42-175c3bd64406
Updated Bob: $700 → $600 (in transaction)
Status before rollback: Active
✓ Transaction rolled back

✓ Verified Bob remains $700 after rollback

=== Cleanup ===
✓ Deleted test account collection

✓ All client transaction examples completed
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: qM7ryUrbDkGs3q5e7F-K_n_im_cwhuSp1tbXBNwlJzw5ZyZkvRO37Ru6KrP_XJuJypHrA8bPRqoTcj6CQpIThQ

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
Inserted: KyC304jqmsx79oTHs1xfy2D--anS9CtNiNiemRxfflhe0D66c7VpBzpUUVNXNCHNXCQFvN8v0Uv15mBGr9EcTw

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
✓ Created session: li5WwvL3w_1cYgaxdFYhjD16Bxnk4GpHUoIH0sq1SHnVEYP2vb_NuRnETOWob30uKryqWexdp8i0CGhWu7fYzA

=== Sending Initial Message ===
✓ Message sent
  Response: It seems there are no products currently available in the database. However, I have found information about the product "ekoDB":

- **Description:** High-performance database product
- **ID:** azUfjiXPmjg_FLvfeNfsEq1CRbhlxR9OSF1cHPZfY6IzhTD8h-lQ69Xx2ZXdp-i7Jh0yh03kKyIThZ_qnjVgaA
- **Name:** ekoDB
- **Price:** $99

If you need any further details or if you're looking for a specific product, feel free to ask!

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
✓ Created second session: iTPRuU0_OTUkKSbDfKntzAZ_suipnw74-OktglURyyF4ySgApnlVIwhx9SZ4MoexbxXeekSLq3Wq2b87eeyF-g
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 7

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 6

=== Cleanup ===
✓ Deleted chat session: iTPRuU0_OTUkKSbDfKntzAZ_suipnw74-OktglURyyF4ySgApnlVIwhx9SZ4MoexbxXeekSLq3Wq2b87eeyF-g
✓ Deleted chat session: li5WwvL3w_1cYgaxdFYhjD16Bxnk4GpHUoIH0sq1SHnVEYP2vb_NuRnETOWob30uKryqWexdp8i0CGhWu7fYzA
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: jBFRyd2imjYMN9u9B7pqV6dOjn1On-Qfv_0FSEPCoUaUkAtDmNa_ghEXUvsNFRHL69GUmCSwXjZ0_AHQBPESwA

=== Sending Chat Message ===
Message ID: tsa0HbF_Aomb611_sbdR_OCBDwPVRZ943TcnG9DqU1FPK0W3t6WXn63v-YZKLhklU6chJbAtHK0ax5SlTbauAg

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
    price: 99,
    id: 'aiY6TigxcKdOhabw7gm3XSIANe2ACM4v7lDgQjHgbIlB5MpDBMmvHfQafaOABQVBqIu7WFG9WlfVN_u6A8lIwA',
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
    id: 'tcVTH7GUXcp08uSX3TGmmFhcjH5HSHVXiFOuV-Ycn2c4roR4bXWPG2_Zr9qUp7GMJBJg-T0Bi-hSovBDom3erQ'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    id: '_TceJpyhI07XUE5xWBnfBYKUNWxfoejztvgzk8XUu0PK9jRnMmf_0YdkBuDjxQ0xeSZlVvY4i87b8bvH_wGR5A',
    description: 'Fully managed cloud database service product',
    price: 499,
    name: 'ekoDB Cloud'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 1906ms

=== Token Usage ===
Prompt tokens: 3413
Completion tokens: 91
Total tokens: 3504

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Message Stream (SSE) Example (TypeScript) ===

Created session: dNNcf_DjUmk7F-dbjBpWHh4Tp8npYhL2FPuIIoBQ2h0tjgyQoNOVx128FL4RIRCMu5dIhDADnvSmkMukXiDOJg

Streaming response for: 'What is ekoDB?'

{"__progress":"received"}{"__progress":"generating","model":"default","provider":"openai"}**ekoDB** does not refer to a single, definitive concept, platform, or product with global recognition (as of June 2024). However, based on context and various uses, here are possible meanings and interpretations:

### Possible Meanings:

1. **EcoDB (Ecological Database):**
   - Sometimes stylized as "ekoDB," this can refer to an ecological or environmental database, used for storing biodiversity, species, or environmental data. These platforms are crucial for ecological research and data sharing.

2. **Project-Specific Database:**
   - In some research papers, GitHub repositories, or organizational projects, "ekoDB" may be the name of a custom database or software tool used for specific domains, such as environmental science, bioinformatics, or even financial data.

3. **NoSQL or SQL Database Technology:**
   - "ekoDB" might refer to a proprietary or open-source database management system, though, as of mid-2024, there is no widely adopted database engine officially called "ekoDB" in the mainstream database community.

4. **Company or Product Name:**
   - Some firms or startups may choose "ekoDB" as a brand or product name, particularly those involved in data storage, management, or analytics with an environmental, economical, or eco-friendly focus.

### Example Usages Found Online

- **ekoDB and Bats**: In the field of bioacoustics, there is a project known as **ekoDB** related to bat call analysis and a corresponding open-access database of bat acoustic data. ([source example](https://github.com/eko-project/ekodb))
- Some **GitHub repositories** or **Python libraries** are named “ekoDB,” but they tend to be project-specific rather than industry standards.

---

### In Summary

If you’re referring to a specific tool, platform, or database called “ekoDB,” please provide more context or its full name. In the most common usages, it relates to specialized environmental data repositories or research-oriented tools, particularly in ecology and biodiversity informatics. If you have a particular field or usage in mind, let me know and I can provide a more tailored answer!

--- Stream complete ---
Message ID: 7pPLi9__6qoneOZrcMIZVKfWVwxDNtYnZMwVs4SOKoXG6ucIfuTy41j3pONVdew2oHx_XBw_qw1xAygSRCPaxw
Execution time: 6392ms
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
✓ Created session: MD39dzsBvoWEmoGCVMHAg8m6AHJk2CNvF5e-3J3gY123kcrElbEtXdg7IxqGNP7VykFUVjq_jVdw9MtCXNauAA

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: A high-performance database product
- **Price**: $99

If you need more information or additional products, please let me know!

✓ Message 2 sent
  Response: The price of ekoDB is **$99**.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: nFLgMXuNAqOtRyddzO-bmb8-2gcx8BMd3tPRolmLV31Y2uVMDBI5YLict1eLtFBEbq55PxGbX0bizccDo5km8A
  Parent: MD39dzsBvoWEmoGCVMHAg8m6AHJk2CNvF5e-3J3gY123kcrElbEtXdg7IxqGNP7VykFUVjq_jVdw9MtCXNauAA

=== Listing Sessions ===
✓ Found 2 sessions
  Session 1: nFLgMXuNAqOtRyddzO-bmb8-2gcx8BMd3tPRolmLV31Y2uVMDBI5YLict1eLtFBEbq55PxGbX0bizccDo5km8A (Untitled)
  Session 2: MD39dzsBvoWEmoGCVMHAg8m6AHJk2CNvF5e-3J3gY123kcrElbEtXdg7IxqGNP7VykFUVjq_jVdw9MtCXNauAA (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: nFLgMXuNAqOtRyddzO-bmb8-2gcx8BMd3tPRolmLV31Y2uVMDBI5YLict1eLtFBEbq55PxGbX0bizccDo5km8A

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ All session management operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: hRvJ65sPIZvVjaxQpV0xA8Sjds1YRgJIHCG1R3FCnoIVoZQKy7NX55T7KWRrOEtjBN2U8wGbdwt7eYlhq74HEw

=== List Collections ===
Total collections: 27
Sample collections: schema_users_client_ts,schema_documents_client_ts,chat_goals__ek0_testing,schema_products_client_js,schedules__ek0_testing

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
  POST /api/functions/conc_demo_pay_ts_71396_1789591703688 { "idempotency_key": "...", "amount": 100 }
  POST /api/functions/conc_demo_rl_fail_ts_71396_1789591703688 { "user_id": 42 }
  POST /api/functions/conc_demo_rl_skip_ts_71396_1789591703688 { "user_id": 42 }
  POST /api/functions/conc_demo_lock_ts_71396_1789591703688 { "resource": "queue:drain" }

✓ Cleaned up demo functions
=== ekoDB Convenience Methods Example ===

=== Native Object Creation ===
✓ Created record with plain object: {
  id: 'kDET-8rSMrCiKDji26DaL40L774EErHJkxZantOcqyP6Oz7BKsUmU-dWVqmcsHhXVRVAPIECyKVLRVH-vQFXzg'
}

=== Upsert Operation ===
✓ First upsert (update): {
  id: 'kDET-8rSMrCiKDji26DaL40L774EErHJkxZantOcqyP6Oz7BKsUmU-dWVqmcsHhXVRVAPIECyKVLRVH-vQFXzg',
  age: { value: 29, type: 'Integer' },
  name: { type: 'String', value: 'Alice Johnson' },
  active: { type: 'Boolean', value: true },
  email: { value: 'alice.j@newdomain.com', type: 'String' }
}
✓ Second upsert (insert): {
  id: '22JZXgYx5Maq3V_ZzzuqKDiyJA8eslBFQ0n1YWc08HXvuhbRVFAcOI-K-fUaGxQgwwQOo1nxT3rxu0IJ8UUtng'
}

=== Find One Operation ===
✓ Found user by email: {
  name: { type: 'String', value: 'Alice Johnson' },
  email: { value: 'alice.j@newdomain.com', type: 'String' },
  active: { type: 'Boolean', value: true },
  id: 'kDET-8rSMrCiKDji26DaL40L774EErHJkxZantOcqyP6Oz7BKsUmU-dWVqmcsHhXVRVAPIECyKVLRVH-vQFXzg',
  age: { value: 29, type: 'Integer' }
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
✓ Inserted document: SNoGNOSfzn9CwZsOZIPINSCCET8-aoGO0UNtjinhIq28LNwC_c_M0PZR2NADDbkynh0iC19NoZUzPiK-yxnTZQ

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: 1LotCLFz3lKmIQpJNvCDiVMiZnwSXAcwUtrrP0CL-Dm2o6UqdPNBO7J1vl_m7CkjD3lnu-yYp_Akf3NrHdIpoQ

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
✓ Edge cache script created: x8MK745khba2QJx9jr18rpE1q2I_RWe7OyfaRam2a9VbgVUjaQz-eOO9AdGAht6sCMmvjYZkGnrmECxuzH_deA

Call 1: Cache miss (fetches from API)
Response time: 66ms
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "phone": "1-770-736-8031 x56442",
          "username": "Bret",
          "address": {
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "suite": "Apt. 556",
            "street": "Kulas Light",
            "city": "Gwenborough",
            "zipcode": "92998-3874"
          },
          "company": {
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona"
          },
          "email": "Sincere@april.biz",
          "website": "hildegard.org",
          "name": "Leanne Graham",
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
Response time: 2ms (33x faster!)
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "phone": "1-770-736-8031 x56442",
          "username": "Bret",
          "address": {
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "suite": "Apt. 556",
            "street": "Kulas Light",
            "city": "Gwenborough",
            "zipcode": "92998-3874"
          },
          "company": {
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona"
          },
          "email": "Sincere@april.biz",
          "website": "hildegard.org",
          "name": "Leanne Graham",
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
   Department: {"value":"engineering","type":"String"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 77ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "company": {
        "bs": "harness real-time e-markets",
        "catchPhrase": "Multi-layered client-server neural-net",
        "name": "Ro...

Second call (cache hit - from cache):
   ⏱️  Duration: 2ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "company": {
        "bs": "harness real-time e-markets",
        "catchPhrase": "Multi-layered client-server neural-net",
        "name": "Ro...
   🚀 Cache speedup: 38.5x faster!

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

✅ Function saved: y5UXcNEcdSa-rH8RbO41no23-hZ2dCMLNktaHwq481b_IxCbTZ5N9_1_dtgMk32RVJQ_UlT29auseTqpmpF8lQ
📊 Found 5 active users

📝 Example 2: Parameterized Function

✅ Function saved: Nwl1cHZyF3CUVnj-KTUKapwXzvgJyFpv45TIuypcaL_QxIN-fWbM18dqkQZPtbBtc9-yS5DJDg5lKqD6ftxOKQ
📊 Found 3 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: i2U1pPu_2nU0aqCde-0nBDR1HLbd7cByyHTiJnBa6DZciXbMVwCPa41mheJGys9G8bvicCJPqjAtVupjX3iQEA
📊 Statistics: 2 groups
   {"avg_score":{"value":50,"type":"Float"},"count":{"type":"Integer","value":5},"status":{"type":"String","value":"inactive"}}
   {"avg_score":{"value":60,"type":"Float"},"count":{"type":"Integer","value":5},"status":{"type":"String","value":"active"}}

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
   {"avg_price":{"value":365.6666666666667,"type":"Float"},"count":{"type":"Integer","value":3},"category":{"type":"String","value":"Furniture"}}
   {"avg_price":{"type":"Float","value":367},"count":{"type":"Integer","value":5},"category":{"type":"String","value":"Electronics"}}
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

1. **High-Dimensional Data Handling**: They efficiently manage and query high-dimensional data, which is common in machine learning applications.

2. **Similarity Search**: Vector databases facilitate fast nearest neighbor searches, making it easier to find similar items based on vector embeddings.

3. **Scalability**: They are designed to scale with large datasets, allowing for handling billions of vectors.

4. **Real-Time Processing**: Many vector databases support real-time querying, enabling dynamic application scenarios.

5. **Integration with AI/ML**: They seamlessly integrate with AI and machine learning workflows, supporting model outputs directly.

6. **Flexible Data Models**: Vector databases can accommodate various data types and formats, enhancing usability.

7. **Efficient Storage**: Optimized for storing and retrieving vectors, leading to better performance compared to traditional databases for specific use cases.

8. **Multi-Modal Data Support**: Capable of processing data from different modalities (e.g., text, images, audio) in a unified way.

Using a vector database can significantly enhance the performance and functionality of applications that rely on complex data representations.
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

✅ Function saved: AZ9F-AOryMq3FO_cal1aCbPwtatXASbMboQopyuSjzhEaA8eswPxtzBEmq9y3j6MZUpoqTYpGJ-R14Px_LpGUQ
📊 Found 2 product groups
   {"avg_price":{"type":"Float","value":474},"category":{"type":"String","value":"Furniture"},"count":{"value":2,"type":"Integer"}}
   {"category":{"value":"Electronics","type":"String"},"count":{"type":"Integer","value":3},"avg_price":{"type":"Float","value":575.6666666666666}}
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
   {"avg_score":{"value":20,"type":"Float"},"role":{"value":"admin","type":"String"},"count":{"type":"Integer","value":3}}
   {"role":{"type":"String","value":"user"},"avg_score":{"type":"Float","value":70},"count":{"type":"Integer","value":7}}
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

✅ Inserted order: Z7ZUcmWk11i6FQxdpuWNgPOPq2WpE2sXFG8yDnF_4v3jyya-Zm7_F7d6QaT3X0UjG406cj1e8pXuELJsCslaxw
✅ Inserted 2 products with wrapped types

📝 Example 2: UserFunction with Wrapped Type Parameters

✅ Function saved: SJzULk_i3b5E0aXbJdrcNNBzkK2DJGdjL6XiqrtM2lXeTj7kRVGb-eEVX3UAhSih9VuVPLEJ6s5sODR5OuV7cw
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"value":{"role":"admin","userId":"user_abc"},"type":"Object"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: j5CDVf3O4FGFhYlA_PlYaoMQJhMbWn1p7T8xmaxm4VQjf7D4w7LhCyJeET2cAz3neVLWA3zPzzIEeXKe3D3u4g
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 3 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Function

✅ Function saved: TqIQ1CN3-WgLwkD0iI85aRv39bMtmkyW2mwxT5vExoT8ZRx9k4HXI6Daq4a3k_bUuWtzBzOMUz2rMd06uQvZMw
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
   3. Getting Started with ekoDB (Database)
   4. Vector Databases Explained (Database)
   5. Database Design Principles (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   {"count":{"value":2,"type":"Integer"},"category":{"value":"AI","type":"String"}}
   {"count":{"type":"Integer","value":3},"category":{"value":"Database","type":"String"}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
=== ekoDB Goal Template CRUD Example (TypeScript) ===

--- Creating goal template ---
Created template: Data Migration (id: QQjXLJ9fa6Y2Is_DgqnVb-N2WN8j4mHl9t8clECmVScnB1FpOjCq6hcKIAe7FYe05shUbxGR7V9cf4Wp5oWghg)

--- Listing templates ---
Templates: {
  count: 1,
  items: [
    {
      description: [Object],
      id: 'QQjXLJ9fa6Y2Is_DgqnVb-N2WN8j4mHl9t8clECmVScnB1FpOjCq6hcKIAe7FYe05shUbxGR7V9cf4Wp5oWghg',
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
Created goal: Deploy v2.0 (id: AUWYQZHGfkr30b9xKb5dJpxvD6WTsZoWHoXbUpFR-2Y-Zc82uQflPnG1fKYAIOFweLgDw-bdH2cEeDmOJhg2Zg)

--- Listing goals ---
Goals: {
  "count": 1,
  "goals": [
    {
      "created_at": "2026-09-16T20:48:29.030857+00:00",
      "description": "Ship version 2.0 to production",
      "id": "AUWYQZHGfkr30b9xKb5dJpxvD6WTsZoWHoXbUpFR-2Y-Zc82uQflPnG1fKYAIOFweLgDw-bdH2cEeDmOJhg2Zg",
      "status": "pending",
      "steps": "[{\"description\":\"Run test suite\"},{\"description\":\"Build release artifacts\"},{\"description\":\"Deploy to staging\"},{\"description\":\"Deploy to production\"}]",
      "title": "Deploy v2.0",
      "updated_at": "2026-09-16T20:48:29.030857+00:00"
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
        "value": "2026-09-16T20:48:29.030857+00:00"
      },
      "description": {
        "type": "String",
        "value": "Ship version 2.0 with hot-fix patches"
      },
      "id": "AUWYQZHGfkr30b9xKb5dJpxvD6WTsZoWHoXbUpFR-2Y-Zc82uQflPnG1fKYAIOFweLgDw-bdH2cEeDmOJhg2Zg",
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
        "value": "2026-09-16T20:48:29.042118+00:00"
      }
    }
  ]
}

--- Goal step: start step 0 ---
Step 0 started on goal AUWYQZHGfkr30b9xKb5dJpxvD6WTsZoWHoXbUpFR-2Y-Zc82uQflPnG1fKYAIOFweLgDw-bdH2cEeDmOJhg2Zg
--- Goal step: complete step 0 ---
Step 0 completed on goal AUWYQZHGfkr30b9xKb5dJpxvD6WTsZoWHoXbUpFR-2Y-Zc82uQflPnG1fKYAIOFweLgDw-bdH2cEeDmOJhg2Zg
--- Goal step: fail step 1 ---
Step 1 failed on goal AUWYQZHGfkr30b9xKb5dJpxvD6WTsZoWHoXbUpFR-2Y-Zc82uQflPnG1fKYAIOFweLgDw-bdH2cEeDmOJhg2Zg

--- Completing goal ---
Goal status: pending_review
--- Approving goal ---
Goal status after approve: in_progress

--- Creating goal to reject ---
--- Rejecting goal ---
Goal status after reject: failed


--- Creating task ---
Created task: Hourly Health Check (id: RzC8roM0um58OwJBZMZj_c8NCISXJXUwEGwgRRPe2fE3aAmauJZhAHAvvIoy2g2KNBqSlyYCm9QXrObNOMmTJw)

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
      "id": "RzC8roM0um58OwJBZMZj_c8NCISXJXUwEGwgRRPe2fE3aAmauJZhAHAvvIoy2g2KNBqSlyYCm9QXrObNOMmTJw",
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
Created agent: SupportBot (id: SI3W6OI1EUa7hZzhrktAHZFFjhsFOmW87ZHq5y92HtaRaa99nPBBuICWOUuPyZO6N6RDBCXWcIW5ZYZGSxh6cw)

--- Listing agents ---
Agents: {
  "count": 1,
  "items": [
    {
      "deployment_id": {
        "type": "String",
        "value": "deploy_prod_1"
      },
      "id": "SI3W6OI1EUa7hZzhrktAHZFFjhsFOmW87ZHq5y92HtaRaa99nPBBuICWOUuPyZO6N6RDBCXWcIW5ZYZGSxh6cw",
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
By name: SupportBot (id: SI3W6OI1EUa7hZzhrktAHZFFjhsFOmW87ZHq5y92HtaRaa99nPBBuICWOUuPyZO6N6RDBCXWcIW5ZYZGSxh6cw)

--- Updating agent ---
Updated agent: SupportBot

--- Getting agents by deployment ---
Agents in deployment: {
  "count": 0,
  "items": []
}
WARNING: agents-by-deployment omitted created agent SI3W6OI1EUa7hZzhrktAHZFFjhsFOmW87ZHq5y92HtaRaa99nPBBuICWOUuPyZO6N6RDBCXWcIW5ZYZGSxh6cw; TODO: check/fix the server-side deployment lookup

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
  POST /api/functions/jwt_register_ts_71446_1789591709554 { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/jwt_login_ts_71446_1789591709554 { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/jwt_verify_ts_71446_1789591709554 { "token": "<jwt>" }

Set JWT_SECRET in ekoDB's environment_vars whitelist before invoking.

✓ Cleaned up demo functions
=== ekoDB KV Document Linking Example (TypeScript) ===

--- Setup: creating KV key and documents ---
Set KV key: session:admin
Inserted document 1: E3NXtElXrhlbF-iVscch9L4t_2pGN4KWPYVx4PnGbOglS34JDCtzF8tYXRV1RaEoJSR6V6bektz7RN3dL8hcLw
Inserted document 2: U1Vr65U1GQ3-j4eNcRPkOJ7MS6ksCaKmh_kPw9r479-zxBVYFs5mVFpwnngEdLQ4Kl7i-yP--Nr1TZxIk9cNgQ

--- Linking documents to KV key ---
Linked doc E3NXtElXrhlbF-iVscch9L4t_2pGN4KWPYVx4PnGbOglS34JDCtzF8tYXRV1RaEoJSR6V6bektz7RN3dL8hcLw: null
Linked doc U1Vr65U1GQ3-j4eNcRPkOJ7MS6ksCaKmh_kPw9r479-zxBVYFs5mVFpwnngEdLQ4Kl7i-yP--Nr1TZxIk9cNgQ: null

--- Getting links for KV key ---
Links: [
  {
    "collection": "kv_links_example_ts_71451_1789591709634",
    "document_id": "E3NXtElXrhlbF-iVscch9L4t_2pGN4KWPYVx4PnGbOglS34JDCtzF8tYXRV1RaEoJSR6V6bektz7RN3dL8hcLw",
    "field_path": null,
    "created_at": "2026-09-16T20:48:29.717970Z",
    "last_accessed": "2026-09-16T20:48:29.720617Z",
    "metadata": {}
  },
  {
    "collection": "kv_links_example_ts_71451_1789591709634",
    "document_id": "U1Vr65U1GQ3-j4eNcRPkOJ7MS6ksCaKmh_kPw9r479-zxBVYFs5mVFpwnngEdLQ4Kl7i-yP--Nr1TZxIk9cNgQ",
    "field_path": null,
    "created_at": "2026-09-16T20:48:29.719408Z",
    "last_accessed": "2026-09-16T20:48:29.720617Z",
    "metadata": {}
  }
]

--- Unlinking document ---
Unlinked doc U1Vr65U1GQ3-j4eNcRPkOJ7MS6ksCaKmh_kPw9r479-zxBVYFs5mVFpwnngEdLQ4Kl7i-yP--Nr1TZxIk9cNgQ: null

--- Verifying remaining links ---
Remaining links: [
  {
    "collection": "kv_links_example_ts_71451_1789591709634",
    "document_id": "E3NXtElXrhlbF-iVscch9L4t_2pGN4KWPYVx4PnGbOglS34JDCtzF8tYXRV1RaEoJSR6V6bektz7RN3dL8hcLw",
    "field_path": null,
    "created_at": "2026-09-16T20:48:29.717970Z",
    "last_accessed": "2026-09-16T20:48:29.722621Z",
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
  kv_ops_ts_71453_1789591709804:cache:product:1: success
  kv_ops_ts_71453_1789591709804:cache:product:2: success
  kv_ops_ts_71453_1789591709804:cache:product:3: success

=== KV Batch Get ===
✓ Batch retrieved 3 values
  kv_ops_ts_71453_1789591709804:cache:product:1: { price: 29.99, name: 'Product 1' }
  kv_ops_ts_71453_1789591709804:cache:product:2: { price: 39.99, name: 'Product 2' }
  kv_ops_ts_71453_1789591709804:cache:product:3: { price: 49.99, name: 'Product 3' }

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
  kv_ops_ts_71453_1789591709804:cache:product:1: deleted
  kv_ops_ts_71453_1789591709804:cache:product:2: deleted
  kv_ops_ts_71453_1789591709804:cache:product:3: deleted

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
    "name": "Earth",
    "diameter_km": 12742
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
Created schedule: Nightly Database Backup (id: 1554f21d-3f36-4100-be62-84b4c99b1f35, cron: 0 0 2 * * *)

--- Listing schedules ---
Schedules: {
  "count": 1,
  "schedules": [
    {
      "created_at": "2026-09-16T20:48:32.486415Z",
      "cron_expression": "0 0 2 * * *",
      "description": null,
      "enabled": true,
      "function_label": "schedule_noop_typescript_71471_1789591712452",
      "id": "1554f21d-3f36-4100-be62-84b4c99b1f35",
      "last_execution": null,
      "name": "Nightly Database Backup",
      "next_execution": "2026-09-17T02:00:00Z",
      "parameters": {},
      "stats": {
        "avg_execution_time_ms": 0,
        "failed_executions": 0,
        "last_error": null,
        "successful_executions": 0,
        "total_executions": 0
      },
      "timezone": "UTC",
      "updated_at": "2026-09-16T20:48:32.486415Z"
    }
  ]
}

--- Getting schedule ---
Fetched: Nightly Database Backup (cron: 0 0 2 * * *)

--- Updating schedule ---
Updated: Nightly Full Backup (new cron: 0 0 3 * * *)

--- Triggering schedule ---
Trigger response: {
  "schedule_id": "1554f21d-3f36-4100-be62-84b4c99b1f35",
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
  1. Score: 26.400, Matched: bio, title
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
  1. Score: 0.760, Matched:
  2. Score: 0.757, Matched:
  3. Score: 0.743, Matched:

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.497, Matched: content, title
  2. Score: 0.903, Matched: title, content
  3. Score: 0.304, Matched:

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 13.200, Matched: bio, skills

9. Vector search with a metadata pre-filter (category = ml):
Found 2 documents in category "ml" (NLP excluded)
  1. Deep Learning Fundamentals (category: ml)
  2. Introduction to Machine Learning (category: ml)


✅ Search examples completed!
=== Cleanup ===
✅ Deleted test collections
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: '7mdmdUFGP8vPGRai1Lt5SQXqMbjbaWTk35EFtCzyTjwJBpWjiy8PPyMb91SweoKZy0EjG9Badlw4qYYuKQessw'
}

=== Find by ID ===
Found: {
  value: { value: 42, type: 'Integer' },
  name: { type: 'String', value: 'Test Record' },
  created_at: { value: '2026-09-16T20:48:32.979+00:00', type: 'DateTime' },
  price: { value: 99.99, type: 'Float' },
  tags: { value: [ 'tag1', 'tag2', 'tag3' ], type: 'Array' },
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  active: { type: 'Boolean', value: true },
  metadata: { value: { nested: [Object], key: 'value' }, type: 'Object' },
  embedding: { value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ], type: 'Array' },
  id: '7mdmdUFGP8vPGRai1Lt5SQXqMbjbaWTk35EFtCzyTjwJBpWjiy8PPyMb91SweoKZy0EjG9Badlw4qYYuKQessw',
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' },
  categories: { type: 'Array', value: [ 'electronics', 'computers' ] }
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-09-16T20:48:32.979Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { nested: { deep: true }, key: 'value' }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  value: 42,
  name: 'Test Record',
  created_at: '2026-09-16T20:48:32.979+00:00',
  price: 99.99,
  tags: [ 'tag1', 'tag2', 'tag3' ],
  data: 'aGVsbG8gd29ybGQ=',
  active: true,
  metadata: { nested: { deep: true }, key: 'value' },
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  id: '7mdmdUFGP8vPGRai1Lt5SQXqMbjbaWTk35EFtCzyTjwJBpWjiy8PPyMb91SweoKZy0EjG9Badlw4qYYuKQessw',
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  categories: [ 'electronics', 'computers' ]
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  id: '7mdmdUFGP8vPGRai1Lt5SQXqMbjbaWTk35EFtCzyTjwJBpWjiy8PPyMb91SweoKZy0EjG9Badlw4qYYuKQessw',
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] },
  value: { type: 'Integer', value: 100 },
  name: { value: 'Updated Record', type: 'String' },
  categories: { value: [ 'electronics', 'computers' ], type: 'Array' },
  data: { value: 'aGVsbG8gd29ybGQ=', type: 'String' },
  price: { value: 99.99, type: 'Float' },
  active: { value: true, type: 'Boolean' },
  metadata: { type: 'Object', value: { nested: [Object], key: 'value' } },
  user_id: { value: '550e8400-e29b-41d4-a716-446655440000', type: 'String' },
  created_at: { type: 'DateTime', value: '2026-09-16T20:48:32.979+00:00' }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: uF1St5sR_-PJEH6sgXqQvVu0QdFC5cgdHSoMctY7Vxhk4fwHD9_YDBO1m3n2ybOBL8WX1_ZKsZNEnVRCHonyzw

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
✓ Created native SWR script: github_user_native_ts (wSDOG7IqXbcde-LrbQ_tmoDt6N1gy8LjIfQTBaWJ0pyz0HKzOsLMTJNSy3Sj1_n996IgBnmg5B0pYqHMtOfA4g)

First call (cache miss - will fetch from GitHub API):
  Response time: 63ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 2ms
  Speedup: 31.5x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit_ts (GqiYJfA6p7Xe-3qwqoFyensjn1c0790PPJTf_y2R_uklxvXZhWe0BrsvE0FNFNUlQ5yFLmRc0MQz1PKm14bjTg)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail_ts' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline_ts (ul_81pJkeazbYYhCTYuPdFfevLpXJIi0gs24HyvFQSLWLPUvo4ngEnT4SMcdX-8W2kCLbvjTfuzoDMiokt8B6g)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users_swr_ts' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache_ts (x-G8x6hnW1t4NHXAZeLG77aw5N_qlNzQ-nvlTy7RuMvR01MZRdODyrKA3FzaXB9LdCqX1VYB_CHj6JgAmCCPhw)
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
✓ Created SWR script: fetch_api_user_ts_71483_1789591713776 (E2cP-wz67AwG6iv8788JffTC8DrWDRE87tdxMjCXErTrXKAhIYnpsrBMwhMU20EZs3k72wk2tN-UiUrLMVrGOA)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "website": "hildegard.org",
          "company": {
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net"
          },
          "name": "Leanne Graham",
          "username": "Bret",
          "id": 1,
          "address": {
            "zipcode": "92998-3874",
            "street": "Kulas Light",
            "suite": "Apt. 556",
            "city": "Gwenborough",
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            }
          },
          "phone": "1-770-736-8031 x56442",
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
Result (cached): {
  "records": [
    {
      "value": {
        "value": {
          "website": "hildegard.org",
          "company": {
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net"
          },
          "name": "Leanne Graham",
          "username": "Bret",
          "id": 1,
          "address": {
            "zipcode": "92998-3874",
            "street": "Kulas Light",
            "suite": "Apt. 556",
            "city": "Gwenborough",
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            }
          },
          "phone": "1-770-736-8031 x56442",
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
✓ Created enrichment script: fetch_product_reviews_ts_71483_1789591713776 (_ylC4lYgFzElx_c6cpwB1RnHc06lkhR4C6VcVl-Pd4Py_utKf0ZtKb9bkmR42mnfwc0-HdpAqXY7Hv2fUwVK-g)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {
      "value": {
        "value": {
          "dimensions": {
            "depth": 22.99,
            "height": 13.08,
            "width": 15.14
          },
          "reviews": [
            {
              "reviewerEmail": "eleanor.collins@x.dummyjson.com",
              "date": "2025-04-30T09:41:02.053Z",
              "comment": "Would not recommend!",
              "reviewerName": "Eleanor Collins",
              "rating": 3
            },
            {
              "rating": 4,
              "comment": "Very satisfied!",
              "date": "2025-04-30T09:41:02.053Z",
              "reviewerEmail": "lucas.gordon@x.dummyjson.com",
              "reviewerName": "Lucas Gordon"
            },
            {
              "rating": 5,
              "reviewerName": "Eleanor Collins",
              "reviewerEmail": "eleanor.collins@x.dummyjson.com",
              "comment": "Highly impressed!",
              "date": "2025-04-30T09:41:02.053Z"
            }
          ],
          "images": [
            "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
          ],
          "shippingInformation": "Ships in 3-5 business days",
          "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
          "brand": "Essence",
          "category": "beauty",
          "price": 9.99,
          "discountPercentage": 10.48,
          "returnPolicy": "No return policy",
          "meta": {
            "createdAt": "2025-10-09T14:47:01.588Z",
            "updatedAt": "2026-05-23T11:27:41.868Z",
            "qrCode": "https://cdn.dummyjson.com/public/qr-code.png",
            "barcode": "5784719087687"
          },
          "id": 1,
          "weight": 4,
          "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
          "minimumOrderQuantity": 48,
          "tags": [
            "beauty",
            "mascara"
          ],
          "warrantyInformation": "1 week warranty",
          "availabilityStatus": "In Stock",
          "rating": 2.56,
          "title": "Essence Mascara Lash Princess",
          "sku": "BEA-ESS-ESS-001",
          "stock": 99
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
Created Alice: $1000 - ID: 9GBXudSUQP_JSKoTqJbk9XmagMr3h_4EnM-Rv2_k-lZVowx32dFtqtrjGhLcf47tchL1jtSPZF91S8MI3jOOHQ
Created Bob: $500 - ID: xqug9ZIR5fwvbknAObA9gvEHHarbw7ZDo4DpSj3gmPSFcgRIj5i6ttixOLc5tMm4wHzmkzKqw4TX4My7A7QbrA

=== Example 1: Begin Transaction ===
Transaction ID (server-default isolation): 777a755b-f218-4c5b-b204-6fbc9e66e583

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
New transaction: 48fd32e9-fdac-4cc5-95a1-1cd10c084d15
Updated Bob: $700 → $600 (in transaction)
Status before rollback: Active
✓ Transaction rolled back

✓ Verified Bob remains $700 after rollback

=== Cleanup ===
✓ Deleted test account collection

✓ All client transaction examples completed
✓ Client created

=== Create User Function ===
Created user function with ID: cBTtPbp8MmMOYjsczS0VwykZDdtwJIu6K8fFCSkPvrC400_veka-_mtR66nRoSchthFOkYdFzEbXb7XSolwjZA

=== Get User Function ===
Retrieved: get_active_users_ts - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 10 user functions:
  - get_active_users_client_js: Get Active Users (Updated)
  - fetch_product_reviews_ts_68859_1789591599098: Fetch Product with Reviews (Multi-API)
  - get_active_users_client_ts_updated: Get Active Users (Updated)
  - fetch_product_reviews_ts_71483_1789591713776: Fetch Product with Reviews (Multi-API)
  - conc_demo_rl_skip_ts_71396_1789591703688: Rate-limit (skip mode)
  - conc_demo_rl_skip_ts_67690_1789591578851: Rate-limit (skip mode)
  - get_active_users_ts: Get Active Users
  - conc_demo_rl_fail_ts_71396_1789591703688: Rate-limit (fail mode)
  - conc_demo_rl_fail_ts_67690_1789591578851: Rate-limit (fail mode)
  - get_active_users_client_ts_updated: Get Active Users (Updated)

=== Update User Function ===
User function updated successfully

=== Delete User Function ===
User function deleted successfully

✓ User Functions API example complete
=== WebSocket Chat Streaming Example (TypeScript) ===

Created chat session: rL609goL8DMjebdp08yQC_Dz5e_LJzmet2Su3a3hNACIrDVAAqSS7Viap6kxmqBo0G5bah-eckJ4SA6Np20izA

Sending message: 'What is the capital of France?'
{"__progress":"received"}{"__progress":"generating","model":"default","provider":"openai"}The capital of France is Paris.

--- Stream ended ---
Message ID: oAoQAOGA8C9fS2aptZj7niYc1F8S0QYQYdi59l0wd0IaJYAnInA3bUtElsCpKKyErsBDd2JtNl_4ztVo98-HYg
Execution time: 586ms
Token usage: {"completion_tokens":8,"prompt_tokens":15,"total_tokens":23}

Full response: {"__progress":"received"}{"__progress":"generating","model":"default","provider":"openai"}The capital of France is Paris....
=== WebSocket Subscription Example ===

✓ Authentication successful
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_ts' ===
✓ Subscribed (subscription_id: sub_ca3710f80ea44a8b80acaeb4b0394f78)

=== Performing mutations to trigger notifications ===
Inserting a record...
✓ Inserted record: qS7BBmHw9KyB_Mc3d8IUNUq9IX8aA93lQX-bBIaL2EN0FpoBTxv3HzBSwR8g2KzzyUG-Zm9-Mf3qgw0tjteoVg
  📡 Notification received for qS7BBmHw9KyB_Mc3d8IUNUq9IX8aA93lQX-bBIaL2EN0FpoBTxv3HzBSwR8g2KzzyUG-Zm9-Mf3qgw0tjteoVg

Inserting another record...
✓ Inserted record: xftwtN-6ivteyL4YfEOMdSSAoco8gG1mUgEhbifNTijtuZys2quW7XPq88PdNSKhy0TL0C6YUr1pZ1Pc3ahZ1g
  📡 Notification received for xftwtN-6ivteyL4YfEOMdSSAoco8gG1mUgEhbifNTijtuZys2quW7XPq88PdNSKhy0TL0C6YUr1pZ1Pc3ahZ1g

=== Unsubscribing ===
✓ Unsubscribed: {"collection":"ws_subscribe_example_ts","found":true,"unsubscribed":true}

✓ WebSocket subscription example completed successfully
✓ Deleted collection 'ws_subscribe_example_ts'
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: Y4Ij6wxkXqtJCjL6qMpbbXJH6WIiSyvPaOeZERsk31MBS9xvk6weTc2xGkuLxBRbXfZ9pm1YVxhwqqJR0xB2qQ

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
   Inserted with ripple: {"id":"iw74DS347MnDTx5FiXt8FyyiWwA7zRnooq22M8qn_oWsQklbjb5LGq461rjdTyrCGQM_21R9KC4KDPpjOZmTrg"}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {"id":"yriH3jOYEam1d-fQDCslT5ZuUv1TojLkDUZKd-mGJowd3XMxFmDRTzHipcIQV5IfBaMiplLuimSJLWlwZpkuwQ"}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {"name":{"value":"Product 1","type":"String"},"id":"iw74DS347MnDTx5FiXt8FyyiWwA7zRnooq22M8qn_oWsQklbjb5LGq461rjdTyrCGQM_21R9KC4KDPpjOZmTrg","price":{"type":"Integer","value":150}}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {"id":"leOr6aVYyxfholm0-QwMu26QWjZbEE40cfD0sKB9cW_jzZgc7MQuk8J2Ar1bKvOHVT2Ew_87rqgSOjm7sA6SNQ"}

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
  Fields returned: ["bio","name","age","created_at","email","avatar_url","status","id","user_role"]

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
    - Fields: ["age","password","api_key","avatar_url","email","id","secret_token","user_role","status","bio","name","created_at"]
  Projected query:
    - 3 fields per record
    - Fields: ["name","email","id"]
  Bandwidth savings: ~75% fewer fields

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
