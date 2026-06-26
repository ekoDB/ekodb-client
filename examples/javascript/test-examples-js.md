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
(node:24598) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
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
  id: 'KL2NCQNxRYOqQLjPCd7dNDDg81U2Si3Pjjd-wMfQDOTDJoDIFydlY6SFyUtlx0W5RiNNOryTK-ch_GfH7dwEbA'
}

=== Find by ID ===
Found: {
  name: { value: 'Test Record', type: 'String' },
  id: 'KL2NCQNxRYOqQLjPCd7dNDDg81U2Si3Pjjd-wMfQDOTDJoDIFydlY6SFyUtlx0W5RiNNOryTK-ch_GfH7dwEbA',
  active: { value: true, type: 'Boolean' },
  value: { type: 'Integer', value: 42 }
}

=== Find with Query ===
Found documents: [
  {
    name: { value: 'Test Record', type: 'String' },
    active: { value: true, type: 'Boolean' },
    id: 'KL2NCQNxRYOqQLjPCd7dNDDg81U2Si3Pjjd-wMfQDOTDJoDIFydlY6SFyUtlx0W5RiNNOryTK-ch_GfH7dwEbA',
    value: { value: 42, type: 'Integer' }
  }
]

=== Update Document ===
Updated: {
  name: { type: 'String', value: 'Updated Record' },
  value: { type: 'Integer', value: 100 },
  id: 'KL2NCQNxRYOqQLjPCd7dNDDg81U2Si3Pjjd-wMfQDOTDJoDIFydlY6SFyUtlx0W5RiNNOryTK-ch_GfH7dwEbA',
  active: { value: true, type: 'Boolean' }
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
✓ Inserted test record: NvnjI0I_ysYi0yeaWu9DFtL7JyhxX4s4WzfTTjETVMiUNuftb_oOl7IBxFrD3TepymatBiIErzc1vj29yg0WhQ

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
        "id": "WPqvrkImxJdJQGknbhfZE_zv7HPfMd2R_AWgTwv8FDfhndwQgMgaizHru0D74aM7v4u_IW-2BVaM0HqFXLcpUg",
        "active": {
          "value": true,
          "type": "Boolean"
        },
        "name": {
          "type": "String",
          "value": "WebSocket Test Record"
        }
      },
      {
        "name": {
          "type": "String",
          "value": "WebSocket Test Record"
        },
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "3qPv3CEUDmu5T7on85zmtKGt3BFAZyaVEhodwHKMxfq7qiOuKsqIELqjjSv4-KCbxZs4B_7tAJfyw2uSUyK5IQ",
        "value": {
          "value": 42,
          "type": "Integer"
        }
      },
      {
        "id": "OZdVF92oGdfsvvOSD4yyE45G1-Ue61ATHNGkDBRj_10Z-cgCfY2PomLVC0mMdOXbhG8OuJzrg8y4cBsbOzDxyw",
        "name": {
          "type": "String",
          "value": "WebSocket Test Record"
        },
        "value": {
          "type": "Integer",
          "value": 42
        },
        "active": {
          "type": "Boolean",
          "value": true
        }
      },
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "value": {
          "value": 42,
          "type": "Integer"
        },
        "id": "NvnjI0I_ysYi0yeaWu9DFtL7JyhxX4s4WzfTTjETVMiUNuftb_oOl7IBxFrD3TepymatBiIErzc1vj29yg0WhQ",
        "name": {
          "type": "String",
          "value": "WebSocket Test Record"
        }
      }
    ]
  },
  "messageId": "1782493035790"
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

✅ Function saved: qsz_P34Ljj4Uh3eHR8mIyIsEkpHhoEHgS4saRprQ3l-nQ5IZEyJAEV8CcLhwajiT4tMQjUBpjhJzwns-bQPaKQ
📊 Found 10 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Function saved: RG8ERv6YzCarsA_aFU0HbFTaCTcOKB3NSpkYRm-0wYLwnJnR5qQsgkt7ody6GQiRBbah9Zm5HhEZjBR34tds4g
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: -yrODI1LLa-14RH7B8MnIyiVK70u4SXkxB7T2SnVZjHdrAaRvJ__IYUOXyFdrTTREWqDofsFrsptE_lzGMnsMQ
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"status":{"type":"String","value":"active"},"avg_score":{"type":"Float","value":60},"count":{"type":"Integer","value":10},"max_score":{"type":"Integer","value":100}}
   {"count":{"value":10,"type":"Integer"},"max_score":{"value":90,"type":"Integer"},"status":{"type":"String","value":"inactive"},"avg_score":{"type":"Float","value":50}}

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
cache:product:2: { name: 'Product 2', price: 39.989999999999995 }
cache:product:3: { price: 49.989999999999995, name: 'Product 3' }

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
Collection created with first record: 8Nsum8Xm6uYCZ8hJ_RsuW5-QAyMzMGLA1Ml9hdwGNMk0IcuQZAlFSmtwx9KUU_OaTJnRQIPKjedQpLbRyySxFA

=== List Collections ===
Total collections: 26
Sample collections: [
  'test_collection',
  'functions__ek0_testing',
  'demo_collection',
  'ttl_cache',
  'chat_messages__ek0_testing'
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
Created Alice: $1000 - ID: DuZ4zJ3dLEFAw7BPpqDsx0XACe7b4WQjZ1HCk0pkvTxB2sooKi6edIcxU9DniOQ_2dVkdpAFfHborqmIMT8jhA
Created Bob: $500 - ID: y5NOjaqmweZqOSEihC4C_3nfhYIa_j01X6sehqc4LcNFuzS0yVykDWsFhDj4d5L3HBoyJOP9cQq431UPY4uB7A

=== Example 1: Begin Transaction ===
Transaction ID: 4a887cd9-1e46-48b3-96d3-78282a4e9407

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
New transaction: 5b8a2724-54d2-468e-8101-feaa9a86b9d5
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: {"type":"Integer","value":700}

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
   ✅ Function saved: Yv23M3AY1T5MJnac3AGyTcKRXhod3vmufAJKFYX982ijxEiBwBwdwyUE2EgBbb_n0UZzrk-s7UHreJwh-cRPsA

2️⃣ Calling Function (Insert + Verify)...
   ✅ Function executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Name: [object Object]
   📋 Email: {"type":"String","value":"alice@example.com"}
   📋 Status: [object Object]
   📋 Credits: [object Object]

============================================================
📝 Function 2: Query + Update + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: 8k1LPAUdwg6yIl79XuNtetev-QFyOts67OBgW-9Yhnc2XeAk6MD3G11SHrqI0XC9BqTaOc_HAfoFo51bUEKAQw

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
   ✅ Function saved: azty31JMeLL5o6S8MwZMSqBo-pd19A_VxQGylrSywa4mFLax7SD6yyYPaPl009raWhseGGT8mqbNTQE2wi4cCg

2️⃣ Calling Function (Query + Update Credits + Verify)...
   ✅ Function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {"value":0,"type":"Integer"}
   📋 Status: {"type":"String","value":"active"}
   📋 Name: {"value":"Alice Smith","type":"String"}

============================================================
📝 Function 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: D80MC86-Bpuh2eFGoExn5rLvmymKLWHL2eCGC1wSWMYkfGgZMCQZdrTG1uRZ3M2uLvzgiNba4QmfqtLl09oDpA

2️⃣ Calling Function (Query + Delete + Verify)...
   ✅ Function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted function: Yv23M3AY1T5MJnac3AGy...
   ✅ Deleted function: 8k1LPAUdwg6yIl79XuNt...
   ✅ Deleted function: azty31JMeLL5o6S8MwZM...
   ✅ Deleted function: D80MC86-Bpuh2eFGoExn...
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
  Output: Document ID = r9d0Z894Y5emXbBLDk_-j-lEWEIHQjn6UXYe060phO8jnWH4yV-QxLs63GabxvNHNGTx2IWbS5niVNdMXBMVlA
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: findById(r9d0Z894Y5emXbBLDk_-j-lEWEIHQjn6UXYe060phO8jnWH4yV-QxLs63GabxvNHNGTx2IWbS5niVNdMXBMVlA)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: findById(r9d0Z894Y5emXbBLDk_-j-lEWEIHQjn6UXYe060phO8jnWH4yV-QxLs63GabxvNHNGTx2IWbS5niVNdMXBMVlA)
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
  Output: Document ID = NQtswmgYzBQblld47rd2l0wurA_VN1SIQU4h8rhmVdZs9IkNrQOS22er7neffQE4-WsS6Y7UjHmNptnBH96gBg
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: findById(NQtswmgYzBQblld47rd2l0wurA_VN1SIQU4h8rhmVdZs9IkNrQOS22er7neffQE4-WsS6Y7UjHmNptnBH96gBg)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: findById(NQtswmgYzBQblld47rd2l0wurA_VN1SIQU4h8rhmVdZs9IkNrQOS22er7neffQE4-WsS6Y7UjHmNptnBH96gBg)
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
npm warn EBADENGINE Unsupported engine {
npm warn EBADENGINE   package: 'vitest@4.1.9',
npm warn EBADENGINE   required: { node: '^20.0.0 || ^22.0.0 || >=24.0.0' },
npm warn EBADENGINE   current: { node: 'v23.6.0', npm: '10.9.2' }
npm warn EBADENGINE }

> @ekodb/ekodb-client@0.22.0 prepare
> npm run build


> @ekodb/ekodb-client@0.22.0 build
> tsc


up to date, audited 52 packages in 1s

17 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.22.0 build
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
  id: 'vbk0tjjbiJKVvpYT7YAAdyJKaC-YI8tZI44fCtcXO3bV2W0AzyruLWpDn2-sD79S-roqThHcHku50MWQs8zugw'
}

=== Find by ID ===
Found: {
  active: { value: true, type: 'Boolean' },
  name: { type: 'String', value: 'Test Record' },
  value: { value: 42, type: 'Integer' },
  id: 'vbk0tjjbiJKVvpYT7YAAdyJKaC-YI8tZI44fCtcXO3bV2W0AzyruLWpDn2-sD79S-roqThHcHku50MWQs8zugw'
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  active: { type: 'Boolean', value: true },
  id: 'vbk0tjjbiJKVvpYT7YAAdyJKaC-YI8tZI44fCtcXO3bV2W0AzyruLWpDn2-sD79S-roqThHcHku50MWQs8zugw',
  name: { type: 'String', value: 'Updated Record' },
  value: { value: 100, type: 'Integer' }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: oqMY-lZeeERfOStdHw4w9zhLsS0lGU014BMD-0SZnjhKnL-Ye3LdiBFZgm20QS2YM_0FAKck-rFru9j30Nqk0Q

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
Collection created with first record: uGKLW98b5x5RtMTpcTKGhlzZn3t6tvcYhA9L9kC3wGz0ou4q9mJKHRJ9b4ZkmJVo9whF09qs4ZJriywF_iO0cQ

=== List Collections ===
Total collections: 25
Sample collections: test_collection,functions__ek0_testing,ttl_cache,chat_messages__ek0_testing,chat_configurations__ek0_testing

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
Retrieved value: { username: 'john_doe', userId: 123 }

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
Total keys in store: 13

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: false

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: Mae0cuXoLb6BE9rbGTvcfLSuFsSzzfLwbn0JpW60iG1iMGkWxHVFSVPvlMpguyso3RCknfgqqRIQrTvlOSKQtQ
Created Bob: $500 - ID: zY0w-973bRXAhWRyaFrwCnJ-QCnRD0GakMgobgWSfvMqHI6Son_0WlML9ZMQDjGEtqKM_mHh1n-eXem5dHnWMQ

=== Example 1: Begin Transaction ===
Transaction ID: 6631afa7-de0e-4a32-9efe-47c277bdb0c7

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 154f31aa-d3ef-425a-9745-b213a77e4c8c
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
  1. Score: 12.870, Matched: name, email
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
  2. Score: 0.760, Matched: 
  3. Score: 0.730, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.492, Matched: title, content
  2. Score: 0.904, Matched: content, title
  3. Score: 0.308, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 13.200, Matched: skills, bio

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

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: ka3mL2CPHKJhJlQdhp7Qv5e74YUWzHyFQ2HMmw46GmZST-SuO_mLdx_UVxaPgvKiI4dVZzlneGrFAS2vLIwWvw

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: -K4US0KfDm_hOlWGJe2mtltGI6DQf-2WIp6GYe9NT6Pd7KK_HHzfJA_mqRwHy8ZjYAbdlUvpL4Tbyv4AQR65rA

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
✓ Inserted document with TTL: 2svFv5ZVYeVNDtGDoRySczOSpwnzBjGPFFxGS5k-jgJKMe0ysC4wmgcBqK4aoCIRPd77PTHfEaSZOLsJq0f3jg

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
✓ Edge cache script created: Qm38EsVKoVxpwi1JfoKHapf9qkRpcp1cKQz-oyI6hPPdfceLRxB7XK__9fWZ44zYfsrWaAbmzxjOaJkGJFkV4g

Call 1: Cache miss (fetches from API)
Response time: 448ms
Result: {
  "records": [
    {
      "value": {
        "value": {
          "elevation": 32,
          "latitude": 40.710335,
          "timezone": "GMT",
          "current": {
            "interval": 900,
            "time": "2026-06-26T16:45",
            "temperature_2m": 25.9
          },
          "generationtime_ms": 0.01633167266845703,
          "timezone_abbreviation": "GMT",
          "utc_offset_seconds": 0,
          "current_units": {
            "temperature_2m": "°C",
            "interval": "seconds",
            "time": "iso8601"
          },
          "longitude": -73.99308
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
Response time: 6ms (74.7x faster!)
Result: {
  "records": [
    {
      "value": {
        "value": {
          "elevation": 32,
          "latitude": 40.710335,
          "timezone": "GMT",
          "current": {
            "interval": 900,
            "time": "2026-06-26T16:45",
            "temperature_2m": 25.9
          },
          "generationtime_ms": 0.01633167266845703,
          "timezone_abbreviation": "GMT",
          "utc_offset_seconds": 0,
          "current_units": {
            "temperature_2m": "°C",
            "interval": "seconds",
            "time": "iso8601"
          },
          "longitude": -73.99308
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
✅ Function saved: BtvQOgJmxcEI3s862PBwWuo7Bz8vnH5E5s0N54HwvkVGzlLqByFBFaXDKnL7P7tZPYS6-QgJGyoXsVzvUFVNGQ
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
   {"avg_score":{"value":60,"type":"Float"},"count":{"value":5,"type":"Integer"},"status":{"type":"String","value":"active"}}
   {"status":{"value":"inactive","type":"String"},"avg_score":{"type":"Float","value":50},"count":{"value":5,"type":"Integer"}}
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
   ⏱️  Duration: 75ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "email": "Sincere@april.biz",
      "name": "Leanne Graham",
      "company": {
        "catchPhrase": "Multi-layered client-server neural-net",
        "name": "Ro...

Second call (cache hit - from cache):
   ⏱️  Duration: 4ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "email": "Sincere@april.biz",
      "name": "Leanne Graham",
      "company": {
        "catchPhrase": "Multi-layered client-server neural-net...
   🚀 Cache speedup: 18.8x faster!

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

✅ Function saved: 5Itb0KezCE7bzkhaf0UPyT8QUgNfaQX650Va4A0TxnRIFoKi5-be6z_p1VcUI8ehWrjRfWfyWRFy5X0Pf2H-xQ
📊 Found 2 product groups
   {"count":{"type":"Integer","value":2},"category":{"type":"String","value":"Furniture"},"avg_price":{"type":"Float","value":474}}
   {"avg_price":{"type":"Float","value":575.6666666666666},"count":{"type":"Integer","value":3},"category":{"value":"Electronics","type":"String"}}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   {"category":{"value":"Furniture","type":"String"},"count":{"value":2,"type":"Integer"}}
   {"category":{"value":"Electronics","type":"String"},"count":{"type":"Integer","value":3}}
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

✅ Inserted order: y9-QZeYiyUWCSZbalpPeIiDzD91xbFzzn3IfYug2UMahQAGmV4XU_fvsJ5FAVpmluN6OONpvKrFMdi8g5OkF1A
✅ Inserted 2 products with wrapped types

📝 Example 2: Function with Wrapped Type Parameters

✅ Function saved: ZDQwTHhi7KN4HlcFmRE4GyvTm9o705aiRCE1MtZzQa9jO1JVUNX28nOeUMhs6EOR0j57dXV5ik5SNhwCj-r4Pw
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"role":"admin","userId":"user_abc"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: DvvdBodEi7Rq0tIeTl9ULCmKCR7QiM7UV-hm_QcXT1-P-nEz-IemcjD_qmPfzGCr09A_sfoIcpWTDPZeF45l-A
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Function

✅ Function saved: a9hf6EeP3ZHIzts6SDR8Btq8j96wXYHubV4M13EYqWzJyAO9JuNWcPMrPCD8x7MLXjeZzZGaqfA4IAayfkmLqg
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
   user: 70.0 (7 users)
   admin: 20.0 (3 users)
⏱️  Execution time: 0ms

📝 Example 4: Top Users by Score

✅ Function saved
📊 Users (showing first 5 of 10):
   1. User 5 - Score: 50
   2. User 8 - Score: 80
   3. User 7 - Score: 70
   4. User 9 - Score: 90
   5. User 4 - Score: 40
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
   2. Database Design Principles (Database)
   3. Natural Language Processing (AI)
   4. Vector Databases Explained (Database)
   5. Introduction to Machine Learning (AI)
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
   2. Database Design Principles
   3. Natural Language Processing
   4. Vector Databases Explained
   5. Introduction to Machine Learning
⏱️  Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Function saved
📊 Projected documents (showing first 3):
   1. Getting Started with ekoDB
   2. Database Design Principles
   3. Natural Language Processing
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
   1. USB-C Cable - $19 (⭐4.3)
   2. Desk Chair - $349 (⭐4.2)
   3. Wireless Mouse - $29 (⭐4.5)
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
   1. Precision: Vector data models are beneficial when dealing with precise geography or boundaries.

2. Flexibility: They can represent different types of geographical features such as points, lines, and polygons.

3. Scalability: Vector databases permit efficient scaling and zooming, allowing users to view data at various levels of detail.

4. Smaller Size: Vector databases take up less space, making them more efficient for storage.

5. Clear Topology: They can clearly represent topological features such as connections and adjacency of polygons.

6. Attribute Storage: In vector databases, each object can have an attribute associated with it, allowing more in-depth analysis.

7. Realism: Vector data models can effectively represent real-world features like roads, rivers, and buildings more accurately.

8. Computational Efficiency: Certain spatial analyses can be processed more efficiently using vector data models.
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embedding function saved
📊 Generated embeddings for 3 articles
   1. "Getting Started with ekoDB" - 0D vector
   2. "Advanced Query Patterns" - 0D vector
   3. "Draft Article" - 0D vector
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
✓ Created SWR script: fetch_api_user_js (I6IjJDOSc39l8hgGK7L2RCIjLL5y1sHib2XfH1D7u1ZAWfwV6_XInsJR3rfD0lxz2ulbPwrLPAKa6NMNj9vK_w)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "value": {
          "id": 1,
          "email": "Sincere@april.biz",
          "username": "Bret",
          "address": {
            "suite": "Apt. 556",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "street": "Kulas Light",
            "zipcode": "92998-3874",
            "city": "Gwenborough"
          },
          "name": "Leanne Graham",
          "phone": "1-770-736-8031 x56442",
          "website": "hildegard.org",
          "company": {
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net"
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
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 4ms (served from cache)
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
✓ Created native SWR script: github_user_native (rAN9KoJSGTAEfDlulENE7L2KYQdj4RgexsabfX8RxN48-ANSWnFcIH9kPpli5qpcOrhGk6lNVIkn_0qGq22zYg)

First call (cache miss - will fetch from GitHub API):
  Response time: 4ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 2ms
  Speedup: 2.0x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (0luTiqqZrHkCiZVyDATbzR72SR0vQduj2tZUSfzA3m8VtEK_hcvK-tGz0e73LsW7E0DBd7vCMN4J-RRE_CO87Q)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (PQN0Lni56joleo8RJ9vpHfsjc5VPQQ0W-dqkSmQ6njlXD7YBRww3QAZoCWWjcp2GxzX45_IRmFgHYwG-OuaVLw)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (6F1u9hXGPAWGNqPywS52z7vGSpNbm3lgULQ4zvP-7r1SshgbD2YNBfe5Er-MOyfRNdPHQBcyvjegVpCMt8QnTg)
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
✓ Created session: FmiXG5pAKetUw81PAu4pJbahDNfvgqzQmSmRtc52_sXImga31mnJ39WXR0-kKlZ6pwhlOfgeQaLgMF3MdRZP8Q

=== Sending Chat Message ===
Message ID: a2rbL2RymN4CxmAMfcwbUfSP1DiMZ6TG9RTEy79JfuGwSUGgSaVdm5L-JKmqyQDXhX8a3sc1KeiQh1ybZqB0cg

=== AI Response ===
Here are the available products along with their prices:

1. **ekoDB**
   - Price: $99
   - Description: A high-performance database product with AI capabilities.

2. **ekoDB Cloud**
   - Price: $499
   - Description: Fully managed cloud database service product.

3. **ekoDB Pro**
   - Price: $299
   - Description: Enterprise edition product with advanced features.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_js',
  record: {
    id: 'HJ9TvzhjZIl-pLM3RRsWJdocH2cdERAbCJsuAN3M9cAwdtLOgBfqaceCC2aWfRB_PLdv7FWa55mlhZuplnRmcg',
    name: 'ekoDB',
    description: 'A high-performance database product with AI capabilities',
    price: 99
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_js',
  record: {
    price: 499,
    id: 'iK3IkcFiIZMnMgSVYXNYLZKKbCJCYWd8AyMC-nLOcg4n_keu0BJ3HBHwazugQ8L1jgEW7Eqhg4WKfGMleklE0g',
    name: 'ekoDB Cloud',
    description: 'Fully managed cloud database service product'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_js',
  record: {
    id: 'mwoi7Oy9ekAO9ECOhav5BPP-Vluq30OIhS2pT2CTGv8FJRGv0lxgBmiDVaYs4RROyF2nwWOlNHp1vzzYMQxhlw',
    description: 'Enterprise edition product with advanced features',
    name: 'ekoDB Pro',
    price: 299
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 5922ms

=== Token Usage ===
Prompt tokens: 3413
Completion tokens: 91
Total tokens: 3504

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 1RqXf6uKMyTD13GQD4Oxzbnx_3VR77caiXUQi3aZZt_CJANR27xW4xBVBAEr5FuQ6gQ9F15mJVSqT8y_z5Jg4g

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** High-performance database product
- **Price:** $99

If you need more information or additional products, just let me know!

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
✓ Created second session: 7GIO44WfERGmebZt2qQJh5w9k2caEHH5Dbhxx8kgQ5OWHTcQwsyhkeZN0RGcdaH-e0reKiAqFhwOT9SrnLsKlQ
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
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: fH-aG9uTpvzYoNzRyMn7KqpX_NfRr5CgBrSxJ9ffSj5BL1RQf4gkyHFArNecV9BqBII2s-rB7NplHL-Zhu76aQ

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: A high-performance database product
- **Price**: $99

Would you like more information or assistance with anything else?

✓ Message 2 sent
  Response: The price of the product ekoDB is $99. Would you like to know more about it or anything else?

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 05q_suoEWD2DgHvJUHYg6D3sADYe42xktB301kKNron_B8rvzLHbvMKKiR_Ek2c1b05n8pO2IULcHJxQmLnWYw
  Parent: fH-aG9uTpvzYoNzRyMn7KqpX_NfRr5CgBrSxJ9ffSj5BL1RQf4gkyHFArNecV9BqBII2s-rB7NplHL-Zhu76aQ

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: 05q_suoEWD2DgHvJUHYg6D3sADYe42xktB301kKNron_B8rvzLHbvMKKiR_Ek2c1b05n8pO2IULcHJxQmLnWYw (Untitled)
  Session 2: fH-aG9uTpvzYoNzRyMn7KqpX_NfRr5CgBrSxJ9ffSj5BL1RQf4gkyHFArNecV9BqBII2s-rB7NplHL-Zhu76aQ (Untitled)
  Session 3: 7GIO44WfERGmebZt2qQJh5w9k2caEHH5Dbhxx8kgQ5OWHTcQwsyhkeZN0RGcdaH-e0reKiAqFhwOT9SrnLsKlQ (Untitled)
  Session 4: FmiXG5pAKetUw81PAu4pJbahDNfvgqzQmSmRtc52_sXImga31mnJ39WXR0-kKlZ6pwhlOfgeQaLgMF3MdRZP8Q (Untitled)
  Session 5: tl5AgUXw9dsU27owPdl6mLfXHoIZp4TqFqUCIgszFovnGiHlT09d4XUbaEfpWaQXcPP9aneMgArcJcnrJ6QKUQ (Untitled)
  Session 6: 3DvYW-NyW3TeumsTUdqRKqI8_b6YRbC3vM0oRo8oPIOgoQG6XEkf0aLUyfOsJs3gzkw5P8vrdtWSr5pyFr3xIw (Untitled)
  Session 7: 6Wh9bCQ13lb_HCio7nnE5Hr8aCUA5gv-29SxdpvsvDtlSjZoNat3ZAXh1nqwBi-9AsG-HN5SFab1tYsSpenM2A (Untitled)
  Session 8: X8y4BQWBBNZKMk4RpdETvhBcErKX00VRLLkDmYJCw_loP0dqoXD5MaZ3rwYVtdDUG-uzHCR8Nd5SvG9SjI4_Aw (Untitled)
  Session 9: mqEaN0jW1PYv4-F-nE5E0Y9pe_1D-NBvlwKr9Hqp7Gl520k7RrXGqQpjE3cinyv438pzqdTSH71F20QHVQGxDw (Untitled)
  Session 10: dKGks_vWoqc7xBDZc623Z7LgNSCJJ5QFe-FA_xx6sKDPg8-MHLIDLF0rc3NBksrUQ-nv8W3py5ltjB6qOfhwsw (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: 05q_suoEWD2DgHvJUHYg6D3sADYe42xktB301kKNron_B8rvzLHbvMKKiR_Ek2c1b05n8pO2IULcHJxQmLnWYw

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
✅ [32mJavaScript client examples complete![0m
