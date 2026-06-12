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
(node:37055) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
[32m✓ Server is ready[0m
[34m
=== Getting Authentication Token ===[0m
[32m✓ Authentication successful[0m
[33m
=== Running 10 Examples ===[0m
[34m
=== Running ekoDB/ekodb-client/examples/javascript/simple_crud.js ===[0m
=== Simple CRUD Operations (Direct HTTP) ===

✓ Authentication successful

=== Insert Document ===
Inserted: {
  id: 'v2MGvTsJrI9-gG1vax-jn1dcTkPG7f2W-eWuRssxOGP2jNW033uyeIltFxebc2mrEIQW0gs7T-kIjIkwpzhpMg'
}

=== Find by ID ===
Found: {
  name: { value: 'Test Record', type: 'String' },
  value: { value: 42, type: 'Integer' },
  id: 'v2MGvTsJrI9-gG1vax-jn1dcTkPG7f2W-eWuRssxOGP2jNW033uyeIltFxebc2mrEIQW0gs7T-kIjIkwpzhpMg',
  active: { type: 'Boolean', value: true }
}

=== Find with Query ===
Found documents: [
  {
    id: 'v2MGvTsJrI9-gG1vax-jn1dcTkPG7f2W-eWuRssxOGP2jNW033uyeIltFxebc2mrEIQW0gs7T-kIjIkwpzhpMg',
    value: { value: 42, type: 'Integer' },
    name: { value: 'Test Record', type: 'String' },
    active: { type: 'Boolean', value: true }
  }
]

=== Update Document ===
Updated: {
  name: { value: 'Updated Record', type: 'String' },
  id: 'v2MGvTsJrI9-gG1vax-jn1dcTkPG7f2W-eWuRssxOGP2jNW033uyeIltFxebc2mrEIQW0gs7T-kIjIkwpzhpMg',
  active: { value: true, type: 'Boolean' },
  value: { value: 100, type: 'Integer' }
}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
[32m✓ simple_crud.js completed successfully[0m
[34m
=== Running ekoDB/ekodb-client/examples/javascript/simple_websocket.js ===[0m
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: NHpQgSBySm6UvI8buRgrLpdv6qEQWEX8Ne2QxxpyqzI877chNBXc3gMgGLCNsGxFpTF2uuYFQdhjKxelSRPx5Q

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "type": "Success",
  "payload": {
    "data": [
      {
        "name": {
          "type": "String",
          "value": "WebSocket Test Record"
        },
        "active": {
          "value": true,
          "type": "Boolean"
        },
        "id": "hfPRXtYcPhZ5bEcqW7UFPrWECxK9kTD_ZpDUkrpEChJyQXKIHaLVvQbqri3UbRRfPQ8US3HdHhaSx9nWuyBQyA",
        "value": {
          "type": "Integer",
          "value": 42
        }
      },
      {
        "name": {
          "type": "String",
          "value": "WebSocket Test Record"
        },
        "id": "NHpQgSBySm6UvI8buRgrLpdv6qEQWEX8Ne2QxxpyqzI877chNBXc3gMgGLCNsGxFpTF2uuYFQdhjKxelSRPx5Q",
        "value": {
          "value": 42,
          "type": "Integer"
        },
        "active": {
          "type": "Boolean",
          "value": true
        }
      },
      {
        "name": {
          "type": "String",
          "value": "WebSocket Test Record"
        },
        "value": {
          "type": "Integer",
          "value": 42
        },
        "active": {
          "value": true,
          "type": "Boolean"
        },
        "id": "czKfitlhADJBSeJ6Vn4Ur0K2LoVsFxUu7XLyaRuzKPL63Fp5cbOKlpZpnuAowvwZferwiLUx6wMBIke-dOd6cw"
      },
      {
        "active": {
          "value": true,
          "type": "Boolean"
        },
        "name": {
          "type": "String",
          "value": "WebSocket Test Record"
        },
        "id": "rWn1H-INPJp8yuJW-Lgd7Fabz1lzn5mvYGxUc18G2XjIHgDfC81lPvMwVhfGR8O28ZDcv15KNl8HgFXoXYLlLw",
        "value": {
          "type": "Integer",
          "value": 42
        }
      }
    ]
  },
  "messageId": "1781294380724"
}
✓ Retrieved 4 record(s) via WebSocket

✓ WebSocket example completed successfully
WebSocket closed
[32m✓ simple_websocket.js completed successfully[0m
[34m
=== Running ekoDB/ekodb-client/examples/javascript/http_functions.js ===[0m
🚀 ekoDB Functions Example (JavaScript/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Function saved: Z_Re1INrEwcciP2OZlnpAXic2qcuVg3ts86KA12dK00xpJZm1RnfW_9G9VvhR5BIBpPkbVytl1zfD_m36nXVFg
📊 Found 10 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Function saved: WXICFbadUMRLu6_74ES3a6l0ibl5uAAlrqrEAOH5yi0WAVhSFJB9Ipegr7TG5EDxPtURxZJdyZK_WUomX8LFhQ
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: 68O-PQov2j7hZAihpSFHHo15mev7mSerkYA5hXZ_66Wkd6ENz6z6S8FntjE9oWcDHjyXjDuOJmYPctELX2YxQw
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"count":{"value":10,"type":"Integer"},"max_score":{"value":100,"type":"Integer"},"avg_score":{"type":"Float","value":60},"status":{"value":"active","type":"String"}}
   {"count":{"type":"Integer","value":10},"status":{"value":"inactive","type":"String"},"max_score":{"type":"Integer","value":90},"avg_score":{"value":50,"type":"Float"}}

📝 Example 4: Function Management

📋 Total functions: 16
🔍 Retrieved function: Get Active Users
✏️  Function updated
🗑️  Function deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

✅ All examples completed!
[32m✓ http_functions.js completed successfully[0m
[34m
=== Running ekoDB/ekodb-client/examples/javascript/batch_operations.js ===[0m
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
=== Running ekoDB/ekodb-client/examples/javascript/kv_operations.js ===[0m
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
cache:product:2: { price: 39.989999999999995, name: 'Product 2' }
cache:product:3: { name: 'Product 3', price: 49.989999999999995 }

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
[32m✓ kv_operations.js completed successfully[0m
[34m
=== Running ekoDB/ekodb-client/examples/javascript/collection_management.js ===[0m
=== Collection Management (Direct HTTP) ===

✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: CE-8WBhUKJPu59OJLQ6JdLyjksEHBibRhoWMQiMsPuSPuRAb-qQmNYT2v9egP8f8nUXIQRewtHA4eYOR8x6l1g

=== List Collections ===
Total collections: 26
Sample collections: [
  'schema_users_client_go',
  'chat_configurations__ek0_testing',
  'schema_users_client_py',
  'schema_documents_client_ts',
  'schema_documents_client_py'
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
=== Running ekoDB/ekodb-client/examples/javascript/transactions.js ===[0m
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: Uf1kr0nOR_oxjbh32-93dn0KjnGVas3MKoPF_D1gfUHKc1oDL7lJaXawxfpxeNl6BGGdWDv2Qude_PyihCVUsg
Created Bob: $500 - ID: Qciz41r3tj-lOhakuTdJZy207pIaq1KzU6rbef2QFk88c_-r2_ubGlTWf4zwzVh7HxsgQzkKO89A6VlTX-4zsw

=== Example 1: Begin Transaction ===
Transaction ID: f535d945-793d-4a3e-be2d-49b3bcb1a34e

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
New transaction: db892488-fcea-4ba6-b9bd-d7e893a2ccee
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: {"value":700,"type":"Integer"}

=== Cleanup ===
✓ Deleted test accounts

✓ All transaction examples completed
[32m✓ transactions.js completed successfully[0m
[34m
=== Running ekoDB/ekodb-client/examples/javascript/crud_functions.js ===[0m
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
   ✅ Function saved: WmkDBI0TlAyYLDi_v1UjKfl2RErB3cXffQl4wTVXziY5slEHNdFlH8r7oGNEMB1sksMoD43bKmMCGJtncWOT_A

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
   ✅ Function saved: nRBXZ8ykzMBQHNeNNb36lCk-M53G2rqZ0VTfFthBjL90oTbmp9es9lhN4pwdVN94V0nmCCxapY8a7OtwdT1ILA

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
   ✅ Function saved: 2Z0g6mUwk0D9QHKn-SQDP4LHZvfRDYm0AxLQFytEj_RcPafyq6gG09uWDgMYit-ARSgmynGV1snV_91IsO4I1w

2️⃣ Calling Function (Query + Update Credits + Verify)...
   ✅ Function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {"type":"Integer","value":0}
   📋 Status: {"value":"active","type":"String"}
   📋 Name: {"value":"Alice Smith","type":"String"}

============================================================
📝 Function 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: h3eGYGeJvd1M7UXrVAUs9RqLqZcYJai9BO222NWeTqc98OCu1k-o9HWcyPGvdya2zIKYvfgCnF7sTIt1YhgE3A

2️⃣ Calling Function (Query + Delete + Verify)...
   ✅ Function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted function: WmkDBI0TlAyYLDi_v1Uj...
   ✅ Deleted function: nRBXZ8ykzMBQHNeNNb36...
   ✅ Deleted function: 2Z0g6mUwk0D9QHKn-SQD...
   ✅ Deleted function: h3eGYGeJvd1M7UXrVAUs...
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
=== Running ekoDB/ekodb-client/examples/javascript/document_ttl.js ===[0m
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
  Output: Document ID = _ew3OLV5aXefFeL-iOtzixSU_-_2GK_3kp8XDNDu2YxNw5SgDFgKQsTTCtjK1Xoj3L9UuO6izG_okx32X77WhQ
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: findById(_ew3OLV5aXefFeL-iOtzixSU_-_2GK_3kp8XDNDu2YxNw5SgDFgKQsTTCtjK1Xoj3L9UuO6izG_okx32X77WhQ)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: findById(_ew3OLV5aXefFeL-iOtzixSU_-_2GK_3kp8XDNDu2YxNw5SgDFgKQsTTCtjK1Xoj3L9UuO6izG_okx32X77WhQ)
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
[32m✓ document_ttl.js completed successfully[0m
[34m
=== Running ekoDB/ekodb-client/examples/javascript/websocket_ttl.js ===[0m
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
  Output: Document ID = mZWjB-QupOC8nO7cW_z-UX06IB64-RgbSQTtX13Mu4457OjYXDzcF-EgQ3-hIuCLdTStnzAvoGtkhdA00n2goQ
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: findById(mZWjB-QupOC8nO7cW_z-UX06IB64-RgbSQTtX13Mu4457OjYXDzcF-EgQ3-hIuCLdTStnzAvoGtkhdA00n2goQ)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: findById(mZWjB-QupOC8nO7cW_z-UX06IB64-RgbSQTtX13Mu4457OjYXDzcF-EgQ3-hIuCLdTStnzAvoGtkhdA00n2goQ)
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
npm warn EBADENGINE Unsupported engine {
npm warn EBADENGINE   package: 'vitest@4.1.8',
npm warn EBADENGINE   required: { node: '^20.0.0 || ^22.0.0 || >=24.0.0' },
npm warn EBADENGINE   current: { node: 'v23.6.0', npm: '10.9.2' }
npm warn EBADENGINE }

> @ekodb/ekodb-client@0.21.0 prepare
> npm run build


> @ekodb/ekodb-client@0.21.0 build
> tsc


up to date, audited 52 packages in 1s

17 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.21.0 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning JavaScript client library examples...[0m

added 1 package, removed 1 package, and audited 9 packages in 1s

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'IlJ6SKx66qrvTE-OTGdmwY7Aj63nb3Oq7w0DyBZKsv_-2n-2QKWanZPY-wwVCD7tjxwmOumi8knJUDP2o0vnJQ'
}

=== Find by ID ===
Found: {
  value: { value: 42, type: 'Integer' },
  name: { value: 'Test Record', type: 'String' },
  active: { type: 'Boolean', value: true },
  id: 'IlJ6SKx66qrvTE-OTGdmwY7Aj63nb3Oq7w0DyBZKsv_-2n-2QKWanZPY-wwVCD7tjxwmOumi8knJUDP2o0vnJQ'
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  name: { value: 'Updated Record', type: 'String' },
  value: { value: 100, type: 'Integer' },
  active: { value: true, type: 'Boolean' },
  id: 'IlJ6SKx66qrvTE-OTGdmwY7Aj63nb3Oq7w0DyBZKsv_-2n-2QKWanZPY-wwVCD7tjxwmOumi8knJUDP2o0vnJQ'
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: zYMSUaYXJfkji45cOWrlu3896XVYcnB0BuvIyg90blU87WMsunt9Gpe-CqpyUT0I_QzeUbZTf5SuHp7jj7VuxA

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
Collection created with first record: lm0x5sCy_-o-X5C7cKRlwQhQLYK2VHJ3lxum4PZMuUb052If8nXl8X9-O0Rbhy9AC1YZb92Yn2yQK6owunEh2w

=== List Collections ===
Total collections: 25
Sample collections: schema_users_client_go,chat_configurations__ek0_testing,schema_users_client_py,schema_documents_client_ts,schema_documents_client_py

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
cache:product:1: { name: 'Product 1', price: 29.99 }
cache:product:2: { price: 39.989999999999995, name: 'Product 2' }
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
Created Alice: $1000 - ID: GJdrMIiZIYsZH5d7ruohz55efOVrLjWiG4L1WYPN9ohgw4SkD4g8Jbu6UQzW3hE8bLLqii57pSmaRdkQQEJNEg
Created Bob: $500 - ID: IxMoFIYT8YtXuieETs-sXM5mKsWkZP0K_j6w0Kou0PNpTD5PalMlSLjdgMsWoKkQDkx7NL_n7xdjaD1qQVDEAw

=== Example 1: Begin Transaction ===
Transaction ID: 91aebfd8-feac-4c35-887c-25399d0f50a0

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: c55098f7-f7b0-4afb-8d96-3df929b3de24
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
  2. Score: 12.540, Matched: name.value, name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title, bio, title.value, bio.value
  2. Score: 26.400, Matched: title, bio, bio.value, title.value
  3. Score: 26.400, Matched: title.value, bio, title, bio.value
  4. Score: 26.400, Matched: title.value, bio.value, bio, title

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title, title.value, bio.value, bio
  2. Score: 39.600, Matched: title, bio, title.value, bio.value
  3. Score: 39.600, Matched: title.value, bio, title, bio.value
  4. Score: 39.600, Matched: bio, title.value, title, bio.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio.value, bio

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.746, Matched: 
  2. Score: 0.738, Matched: 
  3. Score: 0.726, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.698, Matched: content, title, title.value, content.value
  2. Score: 1.491, Matched: content, content.value, title, title.value
  3. Score: 0.295, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: bio, skills, bio.value, skills.value

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
✓ Inserted document: KNN0ioUu_WUW27CA5uowttfuorZTQYa-gjwjtnOIZBNWUBa1k7g8HleB0RNZA-6nOf4QZhDhJPssfDut25_wTA

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: rsUY8gyiftLdm7c0ZOOwnUzoaY2Kl_cy_NkWy8uw2GZink_GZ-WfYfOpzru-bL6ie47AkLE0Ih3Fkmqoo-OaRQ

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
✓ Inserted document with TTL: wzZGdy5m3zoIyxyswL39knIZ7yMDx2PC3bPXOoxPHzvW7XXxycv2a7EIKot2Jnawp97eUEoGFudmpNVehfBhLA

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
✓ Edge cache script created: wYRYMko7zL-FMivX6rGAW_3oOBm4wPHt2hEGcM6CAXNDRMiSJiPwO-DxbwVKDt6Z-a_dY56iLjWslEtJ1CTP1A

Call 1: Cache miss (fetches from API)
Response time: 40ms
Result: {
  "records": [
    {
      "value": {
        "value": {
          "elevation": 32,
          "generationtime_ms": 0.02038478851318359,
          "utc_offset_seconds": 0,
          "current": {
            "temperature_2m": 34.5,
            "interval": 900,
            "time": "2026-06-12T19:45"
          },
          "longitude": -73.99308,
          "timezone_abbreviation": "GMT",
          "timezone": "GMT",
          "current_units": {
            "interval": "seconds",
            "temperature_2m": "°C",
            "time": "iso8601"
          },
          "latitude": 40.710335
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
Response time: 41ms (1x faster!)
Result: {
  "records": [
    {
      "value": {
        "value": {
          "elevation": 32,
          "generationtime_ms": 0.02038478851318359,
          "utc_offset_seconds": 0,
          "current": {
            "temperature_2m": 34.5,
            "interval": 900,
            "time": "2026-06-12T19:45"
          },
          "longitude": -73.99308,
          "timezone_abbreviation": "GMT",
          "timezone": "GMT",
          "current_units": {
            "interval": "seconds",
            "temperature_2m": "°C",
            "time": "iso8601"
          },
          "latitude": 40.710335
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

🚀 ekoDB Functions Example (JavaScript Client)

✅ Client initialized (token exchange automatic)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

ℹ️  Function 'get_active_users' already existed — updated instead
✅ Function saved: sCui84wzVoQEvTtVpCkGxVpbukWtuXMarvcBvSE5qithWlgAfy-Sj0bcb-Mzm5T5LwPuoi9RXV_XcYEr9iV5dQ
📊 Found 10 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Function

ℹ️  Function 'get_users_by_status' already existed — updated instead
✅ Function saved
📊 Found 10 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Function

✅ Function saved
📊 Statistics: 2 groups
   {"status":{"value":"inactive","type":"String"},"avg_score":{"type":"Float","value":50},"count":{"value":5,"type":"Integer"}}
   {"count":{"type":"Integer","value":5},"status":{"value":"active","type":"String"},"avg_score":{"value":60,"type":"Float"}}
⏱️  Execution time: 0ms

📝 Example 4: Function Management

📋 Total functions: 17
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

ℹ️  Function 'fetch_user' already existed — updated instead
✅ Saved reusable function: fetch_user
✅ Saved composed function: get_user_wrapper (calls fetch_user + projects fields)

📊 Result from composed function:
   Records: 1
   Name: {"type":"String","value":"User 1"}
   Department: {"value":"engineering","type":"String"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

ℹ️  Function 'fetch_and_store_user' already existed — updated instead
✅ Saved reusable function: fetch_and_store_user (uses KV)
ℹ️  Function 'swr_user' already existed — updated instead
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 318ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "name": "Leanne Graham",
      "website": "hildegard.org",
      "username": "Bret",
      "email": "Sincere@april.biz",
      "id": 1,
      "address": {
        "...

Second call (cache hit - from cache):
   ⏱️  Duration: 41ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "name": "Leanne Graham",
      "website": "hildegard.org",
      "username": "Bret",
      "email": "Sincere@april.biz",
      "id": 1,
      ...
   🚀 Cache speedup: 7.8x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

ℹ️  Function 'validate_user' already existed — updated instead
✅ Level 1 function: validate_user
ℹ️  Function 'fetch_slim_user' already existed — updated instead
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

✅ Function saved: z4rFCjvLJapZjVItycgVskgnzgEAKZQjvzEdYMUMb4AQbyc7P8ZRmhR58DlUTC6zT860-YBSwdjgseIqf12crw
📊 Found 2 product groups
   {"category":{"type":"String","value":"Furniture"},"avg_price":{"value":474,"type":"Float"},"count":{"value":2,"type":"Integer"}}
   {"count":{"value":3,"type":"Integer"},"category":{"type":"String","value":"Electronics"},"avg_price":{"type":"Float","value":575.6666666666666}}
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
🚀 ekoDB JavaScript KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: NHhfvwdmSv2stERSG9K30yRiUSjzUkiSCr09kjOJhPN6U8KPY_JshX1l6XUarfgLHbSlIeERvHYTphdw4KYvPg
✅ Inserted 2 products with wrapped types

📝 Example 2: Function with Wrapped Type Parameters

✅ Function saved: Unq0hD8b5d6lbBB4ElPhxxmawiRvmEcX38Uq_Bg4VwBogHwAtVYZy6tA3Jf6Od6RhlLjggulvQacuNGtP43kYw
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"userId":"user_abc","role":"admin"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: BIJcpBieGyMb8YrIrIxWLX477LB9k32bPaJZhO0QgCIMGtGjbDnRkEx6ebNDoKFDKOMI91wKhdmB5C5KadNCFQ
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Function

✅ Function saved: -KbrK_eAXaDHEbnzdvILWlNKWk503MYPJ28Lc2eDosMIaczBzyO5sqHSJu0LbN7DUYtb50AsljBgbk7iThwYXQ
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
   1. User 6 - Score: 60
   2. User 10 - Score: 100
   3. User 1 - Score: 10
   4. User 8 - Score: 80
   5. User 2 - Score: 20
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
   1. Introduction to Machine Learning (AI)
   2. Vector Databases Explained (Database)
   3. Database Design Principles (Database)
   4. Natural Language Processing (AI)
   5. Getting Started with ekoDB (Database)
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
   2. Vector Databases Explained
   3. Database Design Principles
   4. Natural Language Processing
   5. Getting Started with ekoDB
⏱️  Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Function saved
📊 Projected documents (showing first 3):
   1. Introduction to Machine Learning
   2. Vector Databases Explained
   3. Database Design Principles
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
   1. Office Lamp - $79 (⭐4.05)
   2. Bookshelf - $149 (⭐4.1)
   3. Standing Desk - $599 (⭐4.7)
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
   1. Precision: Vector databases provide accurate and detailed data, representing geographic features as they are. They can effectively represent points, lines, and polygons.

2. Scalability: Unlike raster data, vector data is not pixel-based, so it can be zoomed in or out without any loss of quality. 

3. Smaller File Size: Vector data is typically stored as coordinates so the file size can often be smaller than raster data.

4. Attribute Storage: In vector databases, each object has its own record in an attribute table where each attribute is stored, making it an efficient tool for geospatial analysis.

5. Layer Organization: Vector databases allow for better layer organization. Each layer can represent a different feature type, making it easier to analyze and visualize data.

6. Data Integration: Different datasets can be easily integrated and, given the nature of vector data creation, they are well-suited for integrating with other spatial layers.

7. Interactivity: Vector databases allow for high interactivity with individual elements, such as marking, selecting, or changing them.
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
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_api_user_js (_s-SOBcVwKXeGgV_N_inswPxFEGeqZ4tqM8kSWQ3DbRw4f5DkQvYcy-Uw-zo_Ti1kQJqzdgdSknZmTuhIDtkXg)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "value": {
          "website": "hildegard.org",
          "email": "Sincere@april.biz",
          "address": {
            "zipcode": "92998-3874",
            "street": "Kulas Light",
            "city": "Gwenborough",
            "suite": "Apt. 556",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            }
          },
          "company": {
            "catchPhrase": "Multi-layered client-server neural-net",
            "bs": "harness real-time e-markets",
            "name": "Romaguera-Crona"
          },
          "phone": "1-770-736-8031 x56442",
          "username": "Bret",
          "name": "Leanne Graham",
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
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 48ms (served from cache)
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
✓ Created native SWR script: github_user_native (5KRWtur9G-DxbEiaGZnrjmCcYM6n24hXrxnRDXyEakVEAlv3L8EtqNQkRDnfdINciNXqlKGAp8TSeBSSFbf0iw)

First call (cache miss - will fetch from GitHub API):
  Response time: 16ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 15ms
  Speedup: 1.1x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (AdNDJFwNsYbx6ScMQddk5NKe6SGM6mlQaL8se1hNsR4sP-EuTFJD0WX0LOaR_MhQAxxKvDZgCUZYaztuXHwgkg)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (ZeS6dSzCKQzycSXU_3l6Sr-ymZx_zRDuW1epMu5v4xJS7YB79XeGKuCJcxs0AO9heMEqKH7Z_PfhlSTbP21Oog)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (EKjhwTVo9I_tAH-YFb4IIsQky_q0BJ7bSv_00vAqdc3q_wsYmpm_-a9fZAEICITyKZSPbBRDA12WsSQZw1DuRQ)
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
✓ Created session: YMOzAl3CtdY9cg3ejrZAJb9Ck93Bek1ByTaghlavdZmT29ZZpH-8EzEJhdx0gqJ48xRxOKB_FFzaMcy8Jk2NYQ

=== Sending Chat Message ===
Message ID: rVL7O11OepQq_amcYM-XTy3Gyz2kp4fS8XVlvlgoYh2VuVYaDh77xbCQJMsfGOvN5TYkV2IsxbTxTbHZwKdiHw

=== AI Response ===
Here are the products available along with their prices:

1. **ekoDB** - Price: **$99**
   - Description: A high-performance database product with AI capabilities.

2. **ekoDB Cloud** - Price: **$499**
   - Description: Fully managed cloud database service product.

3. **ekoDB Pro** - Price: **$299**
   - Description: Enterprise edition product with advanced features.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_js',
  record: {
    name: 'ekoDB',
    description: 'A high-performance database product with AI capabilities',
    id: 'hUjcsq78kCgGIOX7kFaT0IVVgGlO9UISbHvcDUJhsJV93h9g28g0Kyhpcxr022vtGxNo3EfFLCU18TSzKm6nMw',
    price: 99
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_js',
  record: {
    id: 'D9nGhB1ajB-Q1EXl7rEpruBHC62bZ_y4HM4hrdDmrWRn_kagRgYphPZnTOMroyyWxNJG7eCypSVUtPwWJexbjA',
    name: 'ekoDB Cloud',
    description: 'Fully managed cloud database service product',
    price: 499
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_js',
  record: {
    description: 'Enterprise edition product with advanced features',
    id: '7Gtdy6W1SI7zh5bnoqQx93X71ONWyv2X7YEOBEkaanXV3kaxWl1HH5htPi8OjGTFsFb3T2p-H63N1QS-OheXgA',
    name: 'ekoDB Pro',
    price: 299
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 10406ms

=== Token Usage ===
Prompt tokens: 3468
Completion tokens: 92
Total tokens: 3560

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 9mZc_4GT8b3ObXSw789ymIL3bH5rPXOWQ3NzUfsevlJmNjulPpbrY9Jst2kmQJFQYAYOYwt5vax84ZsFYZyWGw

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: High-performance database product
- **Price**: $99

If you need more information or have further questions, let me know!

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
✓ Created second session: mxDEUVq8iDW9LQdIlXjLaNHYaVitY6qg9BVVzotmf_au0JndmK-dO3FVQvnbndlKqUTzSaevWy2Q-81c2BA5kQ
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
✓ Created session: i_ew6XyrHalyOmVCly0NpSdgFYoP0Wikx7OXRhV84GPh01MrgfdfFH_ziJZC9qkZBqwvad0sTABfbUY8U81z3g

=== Sending Messages ===
✓ Message 1 sent
  Response: It appears that there are currently no products available in the database. The only product I found is ekoDB, which is priced at $99, but there may be additional products that are either sold out or not listed as "available." If you have more specific criteria or another query in mind, please let me know!

✓ Message 2 sent
  Response: The price of ekoDB is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: dWUFzoj68F_-EfJXauJGe5E6roBT8gnN7RJy6fmlLAxEL8FMb7vy_sRAAGpXgRJN261t-QkDUHMI60s5eFA5jw
  Parent: i_ew6XyrHalyOmVCly0NpSdgFYoP0Wikx7OXRhV84GPh01MrgfdfFH_ziJZC9qkZBqwvad0sTABfbUY8U81z3g

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: dWUFzoj68F_-EfJXauJGe5E6roBT8gnN7RJy6fmlLAxEL8FMb7vy_sRAAGpXgRJN261t-QkDUHMI60s5eFA5jw (Untitled)
  Session 2: i_ew6XyrHalyOmVCly0NpSdgFYoP0Wikx7OXRhV84GPh01MrgfdfFH_ziJZC9qkZBqwvad0sTABfbUY8U81z3g (Untitled)
  Session 3: mxDEUVq8iDW9LQdIlXjLaNHYaVitY6qg9BVVzotmf_au0JndmK-dO3FVQvnbndlKqUTzSaevWy2Q-81c2BA5kQ (Untitled)
  Session 4: YMOzAl3CtdY9cg3ejrZAJb9Ck93Bek1ByTaghlavdZmT29ZZpH-8EzEJhdx0gqJ48xRxOKB_FFzaMcy8Jk2NYQ (Untitled)
  Session 5: iW4sWA01lAMEGKTy7W-Iluozh-OZOk4yAjBqqe16541Ha4uR-GhFRotJHxAv7pkIR9eYPjx0FqiZwwTfWl_ZRA (Untitled)
  Session 6: bwSUkxYKHOYj3rNGm12xSKBhyXRE4Rtbq2d9bKW-m9PCqf6bPhMLgIVBk69_CKbjUSIpT8ujAseOH_5FBqIChg (Untitled)
  Session 7: yvdXBXWCDsDbED0u2_D1Vki6219cUKhYsehO29osPEEhsxdsFPhrNvi3-mX6PL144_s-gnVkKJqTk79-_nwUtg (Untitled)
  Session 8: AamKgNzxJS2q6Gz46wtBnhpZjvOhy-BBj3ZLVzJm_GJfYimPAUIAqZJ9scJ88lG4JvJ6xmfey22QgnydrzCvZw (Untitled)
  Session 9: 5xZ93JgeSif7bmVc8N4s0KrnEGobUJvBYvx3qWFNtBNdf2PVmGtHAAY4tX2L2F6KEe_5Ci20Q0xEGtDaRfY5Gg (Untitled)
  Session 10: -CHGypZHQioO4FD5R17EMmh9ALEBZqRo9lWZnYJb3FqXkGaLB6yWE7QSVyMjHqrDRr_IwdIiP687MKwrwQoK1Q (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: dWUFzoj68F_-EfJXauJGe5E6roBT8gnN7RJy6fmlLAxEL8FMb7vy_sRAAGpXgRJN261t-QkDUHMI60s5eFA5jw

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
✅ [32mJavaScript client examples complete![0m
