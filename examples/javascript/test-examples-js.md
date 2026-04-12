make test-examples-javascript
🧪 [36mRunning JavaScript examples (direct HTTP/WebSocket)...[0m

added 1 package, removed 1 package, and audited 9 packages in 1s

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
[34m
╔════════════════════════════════════════╗[0m
[34m║  ekoDB JavaScript Examples Test Suite ║[0m
[34m╚════════════════════════════════════════╝[0m
[34m
=== Checking Server Connection ===[0m
[32m✓ Server is ready[0m
[34m
=== Getting Authentication Token ===[0m
[32m✓ Authentication successful[0m
[33m
=== Running 10 Examples ===[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/simple_crud.js ===[0m
=== Simple CRUD Operations (Direct HTTP) ===

✓ Authentication successful

=== Insert Document ===
Inserted: {
  id: 'lUhfG_LkOWy6qyOQXBKuwR10Svi51e6NHEC2ay-wBPYxwvq8XxqQBL0ZAqi4fapGbCq2Smn_g5TjkoYYUf_sUQ'
}

=== Find by ID ===
Found: {
  value: { value: 42, type: 'Integer' },
  name: { type: 'String', value: 'Test Record' },
  active: { type: 'Boolean', value: true },
  id: 'lUhfG_LkOWy6qyOQXBKuwR10Svi51e6NHEC2ay-wBPYxwvq8XxqQBL0ZAqi4fapGbCq2Smn_g5TjkoYYUf_sUQ'
}

=== Find with Query ===
Found documents: [
  {
    value: { value: 42, type: 'Integer' },
    name: { type: 'String', value: 'Test Record' },
    active: { value: true, type: 'Boolean' },
    id: 'lUhfG_LkOWy6qyOQXBKuwR10Svi51e6NHEC2ay-wBPYxwvq8XxqQBL0ZAqi4fapGbCq2Smn_g5TjkoYYUf_sUQ'
  }
]

=== Update Document ===
Updated: {
  name: { type: 'String', value: 'Updated Record' },
  id: 'lUhfG_LkOWy6qyOQXBKuwR10Svi51e6NHEC2ay-wBPYxwvq8XxqQBL0ZAqi4fapGbCq2Smn_g5TjkoYYUf_sUQ',
  value: { value: 100, type: 'Integer' },
  active: { value: true, type: 'Boolean' }
}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
[32m✓ simple_crud.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/simple_websocket.js ===[0m
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: sK8ODMrIeauL8-kB9cqwBBWBFaAyjyb5O_Fmn0YmU4iZBXpibDBkpH2lI_Vk1LWp6rFx0pmR2G1jiygxzPQ-qw

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "messageId": "1776012329817",
  "payload": {
    "data": [
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "FeqA3vaSCqzaM09W3ES2j5OE8ThY6bgfDXBhYB9paIu6aM8tKHwgA8kvLgSA_dvnIjkt2zrbSb_ciW8KsFJ-KA",
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
        "id": "sK8ODMrIeauL8-kB9cqwBBWBFaAyjyb5O_Fmn0YmU4iZBXpibDBkpH2lI_Vk1LWp6rFx0pmR2G1jiygxzPQ-qw",
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
        "id": "MtXpLar30tru4Y354yBpOXUjpyCzRaHSnY8__zDq3z-SdZu4fg-jkmuyQAxukeHRhr6q1tC5bKDJeWdu3oDqHQ",
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
        "id": "UYjUzebVUq9jyfdiOrA690sKXzuuYoRp075pKBdtpqydEtcSDCZXMcvbBXk0YR70qiviclhcV9Mx-7bxfyFGTw",
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
✓ Retrieved 4 record(s) via WebSocket

✓ WebSocket example completed successfully
WebSocket closed
[32m✓ simple_websocket.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/http_functions.js ===[0m
🚀 ekoDB Functions Example (JavaScript/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Function saved: VSJxwM2jA96yBfma-2M1BkauRMB3rpNzTWQtmAL0xsVDwe7foStK1dFz9jEbkCnFt4_5iX0CI-M586NryyLoLg
📊 Found 10 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Function saved: TQ2ua7-086ARb9n_CCWOOEisfBXIGyOTMOh3OozZMAH78ORtivK0rWcWihfwvC1NOqdohW1ADsocpMHXlvw6KQ
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: mzfwKq9noUJhRi88RCq-8dXH9klgAlzP_K9PA1rwgfyUW67Dt7tjTvHZJE-KQweaKO4ybQKuTTLfI1bua3fnag
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"max_score":100,"count":10,"status":"active","avg_score":60}
   {"max_score":90,"status":"inactive","avg_score":50,"count":10}

📝 Example 4: Function Management

📋 Total functions: 33
🔍 Retrieved function: Get Active Users
✏️  Function updated
🗑️  Function deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

✅ All examples completed!
[32m✓ http_functions.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/batch_operations.js ===[0m
=== Batch Operations (Direct HTTP) ===

✓ Authentication successful

=== Batch Insert ===
✓ Batch inserted 5 records
✓ Verified: Found 20 total records in collection

=== Creating test records for update/delete ===
Created 3 test records

=== Batch Update ===
✓ Batch updated 3 records
✓ Verified: Record updated with status="active"

=== Batch Delete ===
✓ Batch deleted 3 records
✓ Verified: Records successfully deleted (not found)

✓ All batch operations completed successfully
[32m✓ batch_operations.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/kv_operations.js ===[0m
=== Key-Value Operations (Direct HTTP) ===

✓ Authentication successful

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: { username: 'john_doe', userId: 123 }

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: { name: 'Product 1', price: 29.99 }
cache:product:2: { name: 'Product 2', price: 39.989999999999995 }
cache:product:3: { name: 'Product 3', price: 49.989999999999995 }

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
[32m✓ kv_operations.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/collection_management.js ===[0m
=== Collection Management (Direct HTTP) ===

✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: xBhXFnCG0mKTdhr2t-g4WfOngvVRNYYnI2g6Y8qQAI9bESvtWb0luWMmCJpmjo7QqkOXZg7NCZJxHrLHjRm0fA

=== List Collections ===
Total collections: 25
Sample collections: [
  'schema_users_client_py',
  'test_collection',
  'users',
  'demo_collection',
  'schema_users_client_go'
]

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
[32m✓ collection_management.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/transactions.js ===[0m
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 3B_8y0xTkAEESvSwJ4dVezz3r0bQR5T3fa1Ny5a9HfFyz_apO083Opfqdlwjd72Z8okkABB6c0akr8hgV6YGEA
Created Bob: $500 - ID: T1sUMsEehxuvXoyif4ulfSGb1gLcAF_t-jyDkquT9GTKrPS8FKAOzavfj8OBMXBRyUOZRDAIRgIuXvRwCA3vvw

=== Example 1: Begin Transaction ===
Transaction ID: a493cbab-a0c2-47bc-bc45-b4cdc4fe04d8

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
Bob: {"type":"Integer","value":700}

=== Example 5: Rollback ===
New transaction: bc5f86be-7255-4b0c-b4ac-2a9465f7990c
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: {"value":700,"type":"Integer"}

=== Cleanup ===
✓ Deleted test accounts

✓ All transaction examples completed
[32m✓ transactions.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/crud_functions.js ===[0m
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
📝 Function 1: Insert + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: 3BwnWlDVFVRwAl7MZQydnJ5cCxERmj-B9_Vq7hzL0a_fzrMSX8o6P09QLXe8fdrEwwpnX_au9Y1jl7XQPoBH4w

2️⃣ Calling Function (Insert + Verify)...
   ✅ Function executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Name: [object Object]
   📋 Email: {"value":"alice@example.com","type":"String"}
   📋 Status: [object Object]
   📋 Credits: [object Object]

============================================================
📝 Function 2: Query + Update + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: BbCeb7c9CNtrvkv32qdOw7bH5h7EAj3tjWx0HEG2-j4VLmeXt7diqJwNEUA-P1W0ckI4Lf3GgX9YUdv6DKTsTQ

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
   ✅ Function saved: ZduogjzYmM-uiny_I_67aVVbTJKA57ht4xU6kDySm5iR1QVZecvchCCzMpOYK7nCHyJ9iQkqXgiHOHA9UqE_Ww

2️⃣ Calling Function (Query + Update Credits + Verify)...
   ✅ Function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {"value":0,"type":"Integer"}
   📋 Status: {"value":"active","type":"String"}
   📋 Name: {"value":"Alice Smith","type":"String"}

============================================================
📝 Function 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: r9DqM9BbLzJ8n4xjMH-pez-U8puEBAG0yO5kQBqWlYcZraRHfQ8vhIeU3irypivtUYseNO2ew0s9hcGbcrYFrg

2️⃣ Calling Function (Query + Delete + Verify)...
   ✅ Function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted function: 3BwnWlDVFVRwAl7MZQyd...
   ✅ Deleted function: BbCeb7c9CNtrvkv32qdO...
   ✅ Deleted function: ZduogjzYmM-uiny_I_67...
   ✅ Deleted function: r9DqM9BbLzJ8n4xjMH-p...
   ✅ Deleted collection: users

============================================================
✅ Complete CRUD Functions Example Finished!
============================================================

💡 Key Takeaways:
   ✅ Functions chain Functions together
   ✅ Each function demonstrates operation + verification
   ✅ Parameters make functions reusable
   ✅ Verification is built into the function itself
   ✅ Complete CRUD lifecycle in 4 focused functions
[32m✓ crud_functions.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/document_ttl.js ===[0m
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
  Output: Document ID = uX_ZMrQGdsmia33xfvaz0zS0z1a3WlWXL0Qsc3zI2y3nLIlrtnG6IsH1-fHskSNWezGTPyImqEH9wfePCBrzlw
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: findById(uX_ZMrQGdsmia33xfvaz0zS0z1a3WlWXL0Qsc3zI2y3nLIlrtnG6IsH1-fHskSNWezGTPyImqEH9wfePCBrzlw)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: findById(uX_ZMrQGdsmia33xfvaz0zS0z1a3WlWXL0Qsc3zI2y3nLIlrtnG6IsH1-fHskSNWezGTPyImqEH9wfePCBrzlw)
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
[32m✓ document_ttl.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/websocket_ttl.js ===[0m
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
  Output: Document ID = POca6dehQZRFZmrYSaVK_OybUrrys8vOvLzLegZHiFU0gwXdAi199CTSRvzL0jOqARTKC3qwidTdU-hWktoJnA
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: findById(POca6dehQZRFZmrYSaVK_OybUrrys8vOvLzLegZHiFU0gwXdAi199CTSRvzL0jOqARTKC3qwidTdU-hWktoJnA)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: findById(POca6dehQZRFZmrYSaVK_OybUrrys8vOvLzLegZHiFU0gwXdAi199CTSRvzL0jOqARTKC3qwidTdU-hWktoJnA)
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
[32m✓ websocket_ttl.js completed successfully[0m
[34m
╔════════════════════════════════════════╗[0m
[34m║           Test Summary                 ║[0m
[34m╚════════════════════════════════════════╝[0m
[34mTotal: 10[0m
[32mPassed: 10[0m
[32mFailed: 0[0m
✅ [32mJavaScript direct examples complete![0m
📦 [36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.16.0 prepare
> npm run build


> @ekodb/ekodb-client@0.16.0 build
> tsc


up to date, audited 52 packages in 1s

17 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.16.0 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning JavaScript client library examples...[0m

added 1 package, removed 1 package, and audited 9 packages in 999ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'LEWYyY41q_YyyOrr5KBslR4tAHOc_5t4NN30E8Aj8F8anpKDKyzclfDrAx-d3lR5SaeX7H-8yY4q5ZL-aBZEEA'
}

=== Find by ID ===
Found: {
  value: { value: 42, type: 'Integer' },
  active: { value: true, type: 'Boolean' },
  id: 'LEWYyY41q_YyyOrr5KBslR4tAHOc_5t4NN30E8Aj8F8anpKDKyzclfDrAx-d3lR5SaeX7H-8yY4q5ZL-aBZEEA',
  name: { value: 'Test Record', type: 'String' }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  value: { type: 'Integer', value: 100 },
  id: 'LEWYyY41q_YyyOrr5KBslR4tAHOc_5t4NN30E8Aj8F8anpKDKyzclfDrAx-d3lR5SaeX7H-8yY4q5ZL-aBZEEA',
  name: { type: 'String', value: 'Updated Record' },
  active: { type: 'Boolean', value: true }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: UU6GiLM41CfOI0PfncLtyWpRfnGebH3xBRHt4FMt9B5pBna9nvyQK4TGoVzHgOeaGhkwzT8T0oeKXiyzkokTBA

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
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

=== Create Collection (via insert) ===
Collection created with first record: cHR-yAQYQIyvT_GtAn5ymNoLTYyN4lYeE37uLfxVsbknDrRFNGDmwpDaGDa0WGcPPvGiwG1uKxffu_y64Ic0bg

=== List Collections ===
Total collections: 24
Sample collections: schema_users_client_py,test_collection,schema_users_client_go,chat_messages__ek0_testing,schema_documents_client_go

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: { userId: 123, username: 'john_doe' }

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: { price: 29.99, name: 'Product 1' }
cache:product:2: { name: 'Product 2', price: 39.989999999999995 }
cache:product:3: { price: 49.989999999999995, name: 'Product 3' }

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 0 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 15

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: false

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: ur7hLmVoCHoXGzC3P7bpWmPN6XmEC2iuIJSlBRUCIfz2NsNNc12ujmLiM0RVXbPE2IBVMgSugIqWl7qTWKu-pw
Created Bob: $500 - ID: n6RhPM738GpxwjvA7pIv2pdzBgQlsXEX6lzvbmeHvE_e2ly1-CWWO9IHTH2N5zLhgaw_To9Zulpoin6uU1Rx8w

=== Example 1: Begin Transaction ===
Transaction ID: aa284c96-d603-43fc-9710-28cf2b0bf543

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 684900fd-a3f5-43b7-9f7c-84283d6059e1
Status before rollback: Active
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
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
  1. Score: 25.740, Matched: name.value, email.value, name, email
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: bio.value, title, bio, title.value
  2. Score: 26.400, Matched: bio.value, bio, title, title.value
  3. Score: 26.400, Matched: title.value, bio.value, bio, title
  4. Score: 26.400, Matched: title.value, bio, bio.value, title

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio, title.value, bio.value, title
  2. Score: 39.600, Matched: bio, title.value, bio.value, title
  3. Score: 39.600, Matched: bio, title.value, title, bio.value
  4. Score: 39.600, Matched: title.value, title, bio, bio.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.757, Matched: 
  2. Score: 0.739, Matched: 
  3. Score: 0.729, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.695, Matched: content, content.value, title.value, title
  2. Score: 1.503, Matched: title, content.value, title.value, content
  3. Score: 0.291, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: bio.value, skills, skills.value, bio

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
Schema fields: [ 'age', 'email', 'name', 'status' ]
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

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: 4zq43g_B_n3_HNwar9ors2_nIUhfG58a4AJnFW6cnh0QOCKN_bRN9nKWG9w218SUF_LgxcLbHWvjmCyhume7wA

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: RVCZhPoOlBU_xfV4DqxhJyfrQJTQKHpoO9TQqWCkOIh4Jf_nDRuvjngpMJr17IcsffrkXldQs6ujpKUZs5nVZg

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

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: jacgYk-exFNrs_mARIp2PUBOsZPC53ZFRIKv0Obw5qtZAesqiT-DhKYRbEWeNQZqtq0cmZAgMKcO0SlKk4cVYA

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
=== ekoDB as Edge Cache - Simple Example ===

Creating edge cache function...
✓ Edge cache script created: gnRxF91tEr1YbdpQG1sXAMzOqmteX07Jh59HQKPR0U0qInfWWXhBb5v2XkEcW-Ir9C7t4-kuiY8SG9I_Eq0tLQ

Call 1: Cache miss (fetches from API)
Response time: 41ms
Result: {
  "records": [
    {
      "value": {
        "timezone": "GMT",
        "timezone_abbreviation": "GMT",
        "current_units": {
          "interval": "seconds",
          "time": "iso8601",
          "temperature_2m": "°C"
        },
        "latitude": 40.710335,
        "elevation": 32,
        "current": {
          "temperature_2m": 12.3,
          "time": "2026-04-12T16:30",
          "interval": 900
        },
        "longitude": -73.99308,
        "generationtime_ms": 0.025153160095214844,
        "utc_offset_seconds": 0
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
Response time: 41ms (1x faster!)
Result: {
  "records": [
    {
      "value": {
        "timezone": "GMT",
        "timezone_abbreviation": "GMT",
        "current_units": {
          "interval": "seconds",
          "time": "iso8601",
          "temperature_2m": "°C"
        },
        "latitude": 40.710335,
        "elevation": 32,
        "current": {
          "temperature_2m": 12.3,
          "time": "2026-04-12T16:30",
          "interval": 900
        },
        "longitude": -73.99308,
        "generationtime_ms": 0.025153160095214844,
        "utc_offset_seconds": 0
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

🚀 ekoDB Functions Example (JavaScript Client)

✅ Client initialized (token exchange automatic)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved: npRINyX32BsX_-CJoe0mjtjGDmBUvqOgzmvpaJBUa_oTJb0DZAwDrxe1bnXPZYftiO19RFQRg3oZlmjJhlhO8Q
📊 Found 10 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Function

✅ Function saved
📊 Found 10 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Function

✅ Function saved
📊 Statistics: 2 groups
   {"count":5,"avg_score":60,"status":"active"}
   {"status":"inactive","avg_score":50,"count":5}
⏱️  Execution time: 0ms

📝 Example 4: Function Management

📋 Total functions: 36
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

🧹 Cleaning up...
✅ Deleted collection
✅ Deleted test functions

✅ All examples completed successfully!

💡 Key Advantages of Using the Client:
   • Automatic token management
   • Type-safe Stage builders
   • ChatMessage helpers
   • Cleaner, more maintainable code
   • Built-in error handling
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
   ⏱️  Duration: 163ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "website": "hildegard.org",
    "username": "Bret",
    "email": "Sincere@april.biz",
    "phone": "1-770-736-8031 x56442",
    "company": {
      "name": "Romaguera-Crona",
      "...

Second call (cache hit - from cache):
   ⏱️  Duration: 38ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "website": "hildegard.org",
    "username": "Bret",
    "email": "Sincere@april.biz",
    "phone": "1-770-736-8031 x56442",
    "company": {
      "name": "Romaguera-Crona",
      "...
   🚀 Cache speedup: 4.3x faster!

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
🚀 ekoDB JavaScript Complete Functions Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Function saved: wDpv_aD4lzENm_uImg3USX28EvvQCkTTMy_HPMgc-q8s43UyepH9Q9udH9Kvs6lgc95mdFS1gCphlAAg7dIc5g
📊 Found 2 product groups
   {"count":2,"avg_price":474,"category":"Furniture"}
   {"count":3,"avg_price":575.6666666666666,"category":"Electronics"}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   {"category":"Furniture","count":2}
   {"category":"Electronics","count":3}
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
🚀 ekoDB JavaScript KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: xA8iETbyhlJuc89l2f-l4e7cOoZCm8hlHd1WYkP9sjjTIG9t95typQADPs999Df6UwkEGzefmxOXBUZ_rez3hw
✅ Inserted 2 products with wrapped types

📝 Example 2: Function with Wrapped Type Parameters

✅ Function saved: ff4zxcKPo763ew1Rn7zNOz_-4u4Jl4PKTV_pZ1d3Bf4zV9lCTuNfzOaVSZyaR1PI5UTHLs1fcqr-HYE5crKB2g
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"userId":"user_abc","role":"admin"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: LjmShSwT438gGqKZHlQY_f0nLvq86tnD6txKzvGIpH5HrfHFukI2Pif6qWAezl0JrLiuD5pQ6rljrXud5YiGLw
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Function

✅ Function saved: Pu4JDXunaJ1z14ImkVNKe7WtkvJZcVH6BLuOeG8XcmWRWMT6buzHNrXjDOdiLIGewWq74ckYR_livvnqLZNTcg
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
   1. User 10 - Score: 100
   2. User 9 - Score: 90
   3. User 5 - Score: 50
   4. User 7 - Score: 70
   5. User 8 - Score: 80
⏱️  Execution time: 0ms

📝 Example 5: User Summary Statistics

✅ Function saved
📊 User summary (2 groups):
   inactive users: 3 users, Total Score: 180
   active users: 7 users, Total Score: 370
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD function examples completed!
🚀 ekoDB Search Functions Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Function saved
📊 Found 5 documents
   1. Getting Started with ekoDB (Database)
   2. Introduction to Machine Learning (AI)
   3. Vector Databases Explained (Database)
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
   1. Getting Started with ekoDB
   2. Introduction to Machine Learning
   3. Vector Databases Explained
   4. Natural Language Processing
   5. Database Design Principles
⏱️  Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Function saved
📊 Projected documents (showing first 3):
   1. Getting Started with ekoDB
   2. Introduction to Machine Learning
   3. Vector Databases Explained
⏱️  Execution time: 0ms

📝 Example 5: All Document Fields

✅ Function saved
📊 All documents (5 total, showing first 2):
   1. Getting Started with ekoDB (Database)
   2. Introduction to Machine Learning (AI)
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search function examples completed!
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
   2. Monitor 27" - $399 (⭐4.6)
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
   1. Precision: Vector data is very accurate and precise as it allows the representation of data at the original resolution and form without generalization.

2. More Control: Users have more control over the data as the specifics can be manipulated and rendered based on specific needs.

3. Topological Information: Vector databases can contain topological info that allows understanding of the geographic context of data – like connectedness and adjacency.

4. Scalability: Data remains accurate and precise even when zoom in or out.

5. Lesser space: A clean vector data set usually requires less space for storage compared to raster data.

6. Supports complex analysis: Vector databases have better capabilities to support complex spatial analysis such as network analysis or topology. 

7. High-quality graphics: They maintain a high-quality visualization for graphic features like points, lines, and polygons.
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
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_api_user_js (UE-M8uOi0T5Q-ddjO1MGvnXD_sbR-8lrn5F8ao5sCAOuUeaJElcqK2OCET7TuzahrHlE-qjt3wxuG66YYoCtdg)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "id": 1,
        "phone": "1-770-736-8031 x56442",
        "email": "Sincere@april.biz",
        "company": {
          "catchPhrase": "Multi-layered client-server neural-net",
          "bs": "harness real-time e-markets",
          "name": "Romaguera-Crona"
        },
        "name": "Leanne Graham",
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
        "username": "Bret",
        "website": "hildegard.org"
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
Response time: 47ms (served from cache)
✓ Lightning fast cache hit

🧹 Cleaning up...
✓ Cleanup complete

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation
🚀 ekoDB JavaScript Client - Native SWR Function Examples

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
✓ Created native SWR script: github_user_native (mcGTqb1W3XpQWBBXHaEOMzzgJEebsxrL1ALLLEdsjHiR9ygcr1PXorgSqRxeYMSGjJ0hHXHESEI_7AwrCvyk7g)

First call (cache miss - will fetch from GitHub API):
  Response time: 16ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 21ms
  Speedup: 0.8x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (XjqKG155O6TlkRtxM0JDM5lpPEu-0jHGDcl24U_CdLnqbhVl7co5m7O0rs3g5FLfbrpteqlctqF9pxpOR83F_A)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (49F780SP8-a05_RZWzQJiM_9CsiJSbERblbYT9Ah_Gr59KWsCZikl_PpaYgfkP730rCpJEruf1XaXzMBcSQEEA)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (xtor-4qa5nbppvauBKySOwAuk4aoVdK-2cv-MyWucb83DB88ppJag7Z5r7aVsQZNu39N_eEXXN8_sfXC32Pr5Q)
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
✓ Deleted 4 test functions

✅ All examples completed!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: 3u8q01KGFjTppiBW42YtHmAL2L6XCBGEazuy5vkKZ_7CAFSzmWd7xV6cJDzEqpq5pDaZBtIOd6xl4yj-c-XMkg

=== Sending Chat Message ===
Message ID: Vs3tfl2qIRUeZdVX5npOpk38XzbxFSpeR2CRwX5zys3uw7Y7umOwxLHsShXTdvVJTiPTrsodq8YCV5nRlcyAtw

=== AI Response ===
Here are the available products along with their prices:

1. **ekoDB**  
   - **Description:** A high-performance database product with AI capabilities  
   - **Price:** $99

2. **ekoDB Cloud**  
   - **Description:** Fully managed cloud database service product  
   - **Price:** $499

3. **ekoDB Pro**  
   - **Description:** Enterprise edition product with advanced features  
   - **Price:** $299

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_js',
  record: {
    price: 99,
    description: 'A high-performance database product with AI capabilities',
    name: 'ekoDB',
    id: 'sOBsEqIUCprJD_3DzV8nmk7i2nrdF1obSiizhdC_nnX7Wpo91OCT2KdKZEoqI1BG2m4VcOoRXvFEnv0V4O7cmQ'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_js',
  record: {
    price: 499,
    description: 'Fully managed cloud database service product',
    id: '0nuQCf1U87muYXicDph9p-WbLT4_hCFdvwbaT5KYqz27pCabAWMyyKi7K978jYkmDdXWrv-9VrwziF5yzX1ZKw',
    name: 'ekoDB Cloud'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_js',
  record: {
    name: 'ekoDB Pro',
    id: 'MdRygPEQn3PuAwPcrTdEUUt1A2Oqoct7tR8hr3icQFcU1wvzJMcE8a5GopA7F3nAJsnaYr0rkDXwEMfa8_pDfA',
    price: 299,
    description: 'Enterprise edition product with advanced features'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 3105ms

=== Token Usage ===
Prompt tokens: 1298
Completion tokens: 100
Total tokens: 1398

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 2R6O0XEt_avDZYXuyuJdjUrR2-4qo4hOcGj4oZDD00E92AisWb8WMz6fDTWybUuL4Z7tFNMEExRySxPFSsahSQ

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: High-performance database product
- **Price**: $99

If you would like to know more details or have other questions, feel free to ask!

✓ Second message sent

=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of ekoDB is $99. If you have any further questions or need more information, feel free to ask!

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: g1vTMsKcRFdECWVlFg0YGCtnzoupn8YnGihUTNPVqNKPkDCvTY13rh4FjtFwH3O6Svm3okIwkqkRybqU3WwZeQ
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 6

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 5

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: AsU1Oil6uViYNupwxqcnnyBvwTZvJh1HzLipSWj0d_90ZzryQ07GkYbAY-_kVFkvyUSRy1F5HviUV2iE5Prlbw

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: A high-performance database product
- **Price**: $99

If you need more information or additional products, let me know!

✓ Message 2 sent
  Response: The price of ekoDB is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: goFpTA90HsvlHCweGwxb_ja_qBb6YW-rtlWYfyOZAA3dokSiCZFQvfYfdF7yHKkWDs6vuBx4W9wmos_NPzBddQ
  Parent: AsU1Oil6uViYNupwxqcnnyBvwTZvJh1HzLipSWj0d_90ZzryQ07GkYbAY-_kVFkvyUSRy1F5HviUV2iE5Prlbw

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: goFpTA90HsvlHCweGwxb_ja_qBb6YW-rtlWYfyOZAA3dokSiCZFQvfYfdF7yHKkWDs6vuBx4W9wmos_NPzBddQ (Untitled)
  Session 2: AsU1Oil6uViYNupwxqcnnyBvwTZvJh1HzLipSWj0d_90ZzryQ07GkYbAY-_kVFkvyUSRy1F5HviUV2iE5Prlbw (Untitled)
  Session 3: g1vTMsKcRFdECWVlFg0YGCtnzoupn8YnGihUTNPVqNKPkDCvTY13rh4FjtFwH3O6Svm3okIwkqkRybqU3WwZeQ (Untitled)
  Session 4: 3u8q01KGFjTppiBW42YtHmAL2L6XCBGEazuy5vkKZ_7CAFSzmWd7xV6cJDzEqpq5pDaZBtIOd6xl4yj-c-XMkg (Untitled)
  Session 5: oXyNlcmlgi1EJFwRbqJFi4_amcde50qpGVi13cHaHKVRLpr3h-YuqugcpR8JUdfSnb_FhzTvnWquljdcqIlFRA (Untitled)
  Session 6: sB-siCOBQMoeDbbZGWMpxBRbjV56-5IaFPUO_KWVohEzM2ZhO_fe7kEzGbHZSSkKCob-yIJYdQBhnRMJkRT54w (Untitled)
  Session 7: Wq0FBjdjyZFL5dvEiXXh9oSzxml2MaulqyBAArknQ-YjhUBicp2wm_7t2j_4Jo6SfUpQdqr9EH0zvA7Vi4vvWA (Untitled)
  Session 8: Znp6mUA8YF5IZMlJ_Gi5RO5IK4Csu0uLWqmdO2yqmd_ANoxYa250sKiTx-bmvvu2SIgeuzVldlKiXpUYweOW_g (Untitled)
  Session 9: Be84Ao5KLBzjK4zjM0uyi7WHiFGkgnGiBz913zjhkQrGrZn6A3j7uDi1wNEsih7W6bTQ2aTuD5OcL0gfaQQrvQ (Untitled)
  Session 10: 32PFek8QtgY3SxhW6Q7KVWSbd7uQimZbD3OOUBIeM8mjW6NtmYSx4TqnDEaMk5EP30UDXloORbnCr2umJk9wrw (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: goFpTA90HsvlHCweGwxb_ja_qBb6YW-rtlWYfyOZAA3dokSiCZFQvfYfdF7yHKkWDs6vuBx4W9wmos_NPzBddQ

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
✅ [32mJavaScript client examples complete![0m
