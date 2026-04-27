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
  id: 'wE3COkHeB-t_OMC1kqnRbPVDOWOL_WsWcazeqd9wQWKHT59kii0P4qizmUJbylb7PBIMQP0yzlS9tLLQTcV6AQ'
}

=== Find by ID ===
Found: {
  value: { value: 42, type: 'Integer' },
  id: 'wE3COkHeB-t_OMC1kqnRbPVDOWOL_WsWcazeqd9wQWKHT59kii0P4qizmUJbylb7PBIMQP0yzlS9tLLQTcV6AQ',
  name: { type: 'String', value: 'Test Record' },
  active: { type: 'Boolean', value: true }
}

=== Find with Query ===
Found documents: [
  {
    active: { type: 'Boolean', value: true },
    value: { value: 42, type: 'Integer' },
    name: { value: 'Test Record', type: 'String' },
    id: 'wE3COkHeB-t_OMC1kqnRbPVDOWOL_WsWcazeqd9wQWKHT59kii0P4qizmUJbylb7PBIMQP0yzlS9tLLQTcV6AQ'
  }
]

=== Update Document ===
Updated: {
  name: { type: 'String', value: 'Updated Record' },
  id: 'wE3COkHeB-t_OMC1kqnRbPVDOWOL_WsWcazeqd9wQWKHT59kii0P4qizmUJbylb7PBIMQP0yzlS9tLLQTcV6AQ',
  value: { value: 100, type: 'Integer' },
  active: { type: 'Boolean', value: true }
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
✓ Inserted test record: owF0SWs80KiB35Ca2gSNnFsIJ8BsQM0Gov9cznVb79GSfJfmZrvw-GZJ1cOA2Xr81BupNyyv2FShu78raVp7_A

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "messageId": "1777262786290",
  "payload": {
    "data": [
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "owF0SWs80KiB35Ca2gSNnFsIJ8BsQM0Gov9cznVb79GSfJfmZrvw-GZJ1cOA2Xr81BupNyyv2FShu78raVp7_A",
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
        "id": "Zkzsv36Ek1XeFo_niqTMDMtzYkIibISYbe7FdMX1QNu0v8msvDqC9HgMbsnmIf3hbUAsp7861rajEssi-NCDTg",
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
        "id": "0u0JE-c953bBlBorh6Cut9S1tabpIzBPhR7t-HABc48qoqNB34eMmA_qbIcEsYOm65dWNv21QyDETAf-mJFcBw",
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
        "id": "FYfKxH386rTM0fgtjkI39oijBp6_ViCTDaXBMPMWygAdt-dMVYHDPbo1sa3hHVpjUGzENPNJIQ2hhngC5E0NzQ",
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
        "id": "Ji4rNSPflAda95ioZOsywFx6UhtSJCgId5lqI0K7MHpEGoTZmjhuPqQQghYWglox20Yv2E-Ucjo0sVSfxi1lZw",
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
        "id": "0I6ap8UObm2WUlNNJ_fMSt7i9mn-clX96Q1xy1jpQ8m75mCdD5F8aAM_N9YbRXKPJIn6_RORN3XWg9lyL1ExoA",
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
✓ Retrieved 6 record(s) via WebSocket

✓ WebSocket example completed successfully
WebSocket closed
[32m✓ simple_websocket.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/http_functions.js ===[0m
🚀 ekoDB Functions Example (JavaScript/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Function saved: q2Fcj59UlQ2_e6AR-gHqiFNTLHZh6Sv-VYQfgDeTgJCcO3ZQUUrnsS7ilRXYppiI8oPkodCaF-M_MgWolu9d6Q
📊 Found 10 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Function saved: zZtI-99z-UjVqUHM8J1ZNtgSLN2hKZlmEQJbB2iFLyOj8vcBG067dkxWOWYPUX2na8H3bTzePOMeV3E2ccZB4A
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: h9voVTKeG3jZGcEQQHc1zvkNmpc2E3HpB3c_bv7MNNS8an4kTfkla_RzLPVudtboauxd7DzCD1uKVJhUv6m9fg
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"avg_score":50,"status":"inactive","count":10,"max_score":90}
   {"count":10,"avg_score":60,"status":"active","max_score":100}

📝 Example 4: Function Management

📋 Total functions: 65
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
✓ Verified: Found 30 total records in collection

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
Retrieved value: { userId: 123, username: 'john_doe' }

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: { price: 29.99, name: 'Product 1' }
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
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/collection_management.js ===[0m
=== Collection Management (Direct HTTP) ===

✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: fjRSE6u2EwO8TAxLd3AIcYaCZ0Fdfh53rcmAWNCdf4oh5vzl2AUsSvXo2L1XP-RgufFZ-afeYDmVIIEZqX3ojQ

=== List Collections ===
Total collections: 74
Sample collections: [
  'integ_test_ingest_748f2fd4',
  'certificate_diagnostics_staging',
  'chat_goals__ek0_testing',
  'certificate_diagnostics_testing',
  'enriched_users'
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
Created Alice: $1000 - ID: jKy03_aeT9ybfTrM3OeWvzJ4InZt6-Ew7Kyi2NTuonVFrQsgGA5UPEr19W9k7Ujnck_l1M2wra94C26F3bzgpg
Created Bob: $500 - ID: kp5jcUP5uf22f6l4mXzFn-A4JQbwJguadsY757TagRXHyQWRfBI1MNVSD-oDNE_mt_bqapfKBwraZxNzifeP9A

=== Example 1: Begin Transaction ===
Transaction ID: 55dd6ee3-4f6d-4877-b8f7-ecd732728dd0

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
New transaction: 6423987b-2b75-49d2-8f2c-625d30c75928
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: {"type":"Integer","value":700}

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
   ✅ Function saved: 91s6msH-Qhbm4w0vf05oCxTPZCguD60_o3YhWdgp91n8jywyXRLTb78ovF354LLUUScn7HVxkn2kIj7U5zpSmw

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
   ✅ Function saved: U-RKpEbQuhNqpLKQTiZYk19R61GooBLyMZ2pK_MtfqNZ4p_mCJrJlwXQOIrldih5WiTeU7pNUd2UXTJ4jhuenA

2️⃣ Calling Function (Query + Update + Verify)...
   ✅ Function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Status updated to: {"value":"active","type":"String"}
   📋 Name: {"type":"String","value":"Alice Smith"}

============================================================
📝 Function 3: Query + Update Credits + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: TzVnrCoWJIkWh7ac1yYdhTzCnEhU_M3cq0fKeoQg1NHN0HOk44ECbgeUxkQT9DRVAau_HovZ0--S_pgCLXBxDg

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
   ✅ Function saved: SroT-WpBxltUu2IKMbqCpzvZSxHqcffGF54KR5ASnizjYvvnzk51vUsENH-dFnDhl7_Bd3O4gmMen_Wu3EPEig

2️⃣ Calling Function (Query + Delete + Verify)...
   ✅ Function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted function: 91s6msH-Qhbm4w0vf05o...
   ✅ Deleted function: U-RKpEbQuhNqpLKQTiZY...
   ✅ Deleted function: TzVnrCoWJIkWh7ac1yYd...
   ✅ Deleted function: SroT-WpBxltUu2IKMbqC...
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
  Output: Document ID = Geap-_Yyh8VQBZfvz1Zf6SM6U0IJi5Isk7Vuyk6OWHBkLH9lz1ULy9cbTkgpLFe0C049G7jxI1dhUx2a60tYrA
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: findById(Geap-_Yyh8VQBZfvz1Zf6SM6U0IJi5Isk7Vuyk6OWHBkLH9lz1ULy9cbTkgpLFe0C049G7jxI1dhUx2a60tYrA)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: findById(Geap-_Yyh8VQBZfvz1Zf6SM6U0IJi5Isk7Vuyk6OWHBkLH9lz1ULy9cbTkgpLFe0C049G7jxI1dhUx2a60tYrA)
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
  Output: Document ID = h7AfkZzA3X4gWKhoNnJCzWQEu2LUgvlfozvOhDKB_k4mMpnl7_q8NeqWDEzdUH-Tf4exYh0ckHdcLzTNjIBrZA
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: findById(h7AfkZzA3X4gWKhoNnJCzWQEu2LUgvlfozvOhDKB_k4mMpnl7_q8NeqWDEzdUH-Tf4exYh0ckHdcLzTNjIBrZA)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: findById(h7AfkZzA3X4gWKhoNnJCzWQEu2LUgvlfozvOhDKB_k4mMpnl7_q8NeqWDEzdUH-Tf4exYh0ckHdcLzTNjIBrZA)
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

> @ekodb/ekodb-client@0.18.0 prepare
> npm run build


> @ekodb/ekodb-client@0.18.0 build
> tsc


up to date, audited 52 packages in 2s

17 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.18.0 build
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
  id: 'XuXNdqVrOvwfoWKZWOTNvhsB7hsA62sKObLC4bBbuXfP6Lx0TYaTuzsxSLVD8MSFh8UcWNKhTyWEB2EXZOUZ8Q'
}

=== Find by ID ===
Found: {
  id: 'XuXNdqVrOvwfoWKZWOTNvhsB7hsA62sKObLC4bBbuXfP6Lx0TYaTuzsxSLVD8MSFh8UcWNKhTyWEB2EXZOUZ8Q',
  name: { value: 'Test Record', type: 'String' },
  active: { value: true, type: 'Boolean' },
  value: { type: 'Integer', value: 42 }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  name: { value: 'Updated Record', type: 'String' },
  id: 'XuXNdqVrOvwfoWKZWOTNvhsB7hsA62sKObLC4bBbuXfP6Lx0TYaTuzsxSLVD8MSFh8UcWNKhTyWEB2EXZOUZ8Q',
  value: { type: 'Integer', value: 100 },
  active: { type: 'Boolean', value: true }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: _tyNgiNJp4u1U6wPTTL1cdnsBB0wfs3geP81d6qhZ-wGDfmxbKhwODw6411FPvyc0GSlIpXaPoKmPvvpsKcT8A

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
Collection created with first record: YXvO3RtTxpb2skqv5HFC0LNiubo8RgA4P3yaW83DU3Gi_99jdHB4MjXmubV2iRRLQsPDRFJ9EJHaOVJ1lImIKw

=== List Collections ===
Total collections: 73
Sample collections: integ_test_ingest_748f2fd4,certificate_diagnostics_staging,chat_goals__ek0_testing,certificate_diagnostics_testing,enriched_users

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
cache:product:1: { price: 29.99, name: 'Product 1' }
cache:product:2: { price: 39.989999999999995, name: 'Product 2' }
cache:product:3: { price: 49.989999999999995, name: 'Product 3' }

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 0 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 17

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: false

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: -UPHNxYG40UoOlqd7Iq_x63BLwu4Br3dfbhmgHttTGNl-Ooe6UqJMabeCG-OmSWhamypNbjt4o6xFqtvXwmMIA
Created Bob: $500 - ID: woQFVsKZPxML9MpGe8APiKrVpOxNHYsXf3GvzdHujKWLVtxm_WVwSuabRjRCq8Zp22LZqDpz0FH1ws8g3fkrDg

=== Example 1: Begin Transaction ===
Transaction ID: d72635d0-4a70-459c-a0c6-3f3d4fc01775

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: f61c2a74-4826-4dd3-a5d2-c62492b8a2a2
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
  1. Score: 26.400, Matched: title.value, title, bio, bio.value
  2. Score: 26.400, Matched: title, title.value, bio.value, bio
  3. Score: 26.400, Matched: bio.value, bio, title, title.value
  4. Score: 26.400, Matched: title, bio.value, title.value, bio

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title.value, title, bio, bio.value
  2. Score: 39.600, Matched: title, bio.value, bio, title.value
  3. Score: 39.600, Matched: title.value, title, bio.value, bio
  4. Score: 39.600, Matched: title, title.value, bio, bio.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio.value, bio

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.768, Matched: 
  2. Score: 0.767, Matched: 
  3. Score: 0.752, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.707, Matched: content, content.value, title.value, title
  2. Score: 1.507, Matched: content, content.value, title.value, title
  3. Score: 0.301, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: skills, skills.value, bio.value, bio

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
✓ Inserted document: cuswkd7Hvk_Q8fiU31eiqkAZ8KCjbpVn3M5evDopclrMk8h497YnE3Rp3JbMfKKKw5SLN1Soe5u1ufM8jFqQlw

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: wcwE-bKduvwJ73c5c0kyjMwhicUttRO-N7KX8kcetXChrkH2JLDeXDmmhZOmGRFiWZdp92lFJ75ltSK7MYraRQ

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
✓ Inserted document with TTL: fnneu1ZQ-rlkI5zEtJuhKFT6s-S2D6oJOJXjsVvJP3niWZOrEuaNnrJ0hyJ2FT5TKpxi1V9wp0S7D0ooMhk0MA

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
✓ Edge cache script created: ObvWJ06a8oppEVLls5kY_pZ6AkaRvJtdC3Z8DbJZyPYt_OTKLaM6isgcizbIiSArBhKscaXvQ7fcbdA2eTdq7A

Call 1: Cache miss (fetches from API)
Response time: 49ms
Result: {
  "records": [
    {
      "value": {
        "elevation": 32,
        "longitude": -73.99308,
        "current_units": {
          "interval": "seconds",
          "temperature_2m": "°C",
          "time": "iso8601"
        },
        "current": {
          "interval": 900,
          "temperature_2m": 6.9,
          "time": "2026-04-27T03:45"
        },
        "timezone_abbreviation": "GMT",
        "latitude": 40.710335,
        "generationtime_ms": 0.027060508728027344,
        "utc_offset_seconds": 0,
        "timezone": "GMT"
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
Response time: 46ms (1.1x faster!)
Result: {
  "records": [
    {
      "value": {
        "elevation": 32,
        "longitude": -73.99308,
        "current_units": {
          "interval": "seconds",
          "temperature_2m": "°C",
          "time": "iso8601"
        },
        "current": {
          "interval": 900,
          "temperature_2m": 6.9,
          "time": "2026-04-27T03:45"
        },
        "timezone_abbreviation": "GMT",
        "latitude": 40.710335,
        "generationtime_ms": 0.027060508728027344,
        "utc_offset_seconds": 0,
        "timezone": "GMT"
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

✅ Function saved: lLobUvx9mp-wAGd89FCg4X6WgADG-Op53jqXVsT94jwt6w5XUdEI7-r8Lq3lvqVqx-mLreM3jINsfwDJ22e51w
📊 Found 10 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Function

✅ Function saved
📊 Found 10 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Function

✅ Function saved
📊 Statistics: 2 groups
   {"avg_score":50,"status":"inactive","count":5}
   {"status":"active","count":5,"avg_score":60}
⏱️  Execution time: 0ms

📝 Example 4: Function Management

📋 Total functions: 68
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
   Name: {"type":"String","value":"User 1"}
   Department: {"value":"engineering","type":"String"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 44ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "email": "Sincere@april.biz",
    "phone": "1-770-736-8031 x56442",
    "id": 1,
    "website": "hildegard.org",
    "name": "Leanne Graham",
    "address": {
      "city": "Gwenbor...

Second call (cache hit - from cache):
   ⏱️  Duration: 40ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "email": "Sincere@april.biz",
    "phone": "1-770-736-8031 x56442",
    "id": 1,
    "website": "hildegard.org",
    "name": "Leanne Graham",
    "address": {
      "city": "Gwenbor...
   🚀 Cache speedup: 1.1x faster!

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

✅ Function saved: FpfI5PzJJcGRic9jWajtltK9ZWj4f2eewOslP6gHKB21IXM5KPXzzZN5AzSywDbA5Ci5_TwlLlhHIZCXFtB79g
📊 Found 2 product groups
   {"category":"Electronics","count":3,"avg_price":575.6666666666666}
   {"category":"Furniture","avg_price":474,"count":2}
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

✅ Inserted order: ke-GHnQLAjRs89CbiJs6DiCkkrGcL7cuUpaxXjwKfYthOjn5_qwTMO8jP76LyH4bZFyAqzLaDo2vHXlBPek00g
✅ Inserted 2 products with wrapped types

📝 Example 2: Function with Wrapped Type Parameters

✅ Function saved: phBbPT-EzlXCz5ZyCb4cPLMm-jT2J-I9snPVt656V5S5VNWnug3sKkXv3DyHeof1uHDLw1z9IFfKDuJ0le-Lxw
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"userId":"user_abc","role":"admin"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: pDfeQPsvyNATQViwy_hmCngGDKvRaTtaT9l8kb7wbTAXFugdqqlKxmVlLfboxb1b5PkdipJTUS7iP9RJaxVsyg
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Function

✅ Function saved: VpBNwW0dlxXfJayNLuYGYBIZxs7GAlu54y_zXv9vACLEPWROUGiFlUS1DYUeL1tTBT07TR41CuebBV_zDWZ8yA
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
   1. User 2 - Score: 20
   2. User 6 - Score: 60
   3. User 8 - Score: 80
   4. User 1 - Score: 10
   5. User 10 - Score: 100
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
🚀 ekoDB Search Functions Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Function saved
📊 Found 5 documents
   1. Natural Language Processing (AI)
   2. Vector Databases Explained (Database)
   3. Introduction to Machine Learning (AI)
   4. Database Design Principles (Database)
   5. Getting Started with ekoDB (Database)
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
   1. Natural Language Processing
   2. Vector Databases Explained
   3. Introduction to Machine Learning
   4. Database Design Principles
   5. Getting Started with ekoDB
⏱️  Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Function saved
📊 Projected documents (showing first 3):
   1. Natural Language Processing
   2. Vector Databases Explained
   3. Introduction to Machine Learning
⏱️  Execution time: 0ms

📝 Example 5: All Document Fields

✅ Function saved
📊 All documents (5 total, showing first 2):
   1. Natural Language Processing (AI)
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
   1. Desk Chair - $349 (⭐4.2)
   2. Standing Desk - $599 (⭐4.7)
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
   1. High Precision: Vector databases ensure high precision data storage without any data quality loss.

2. Scalability: They allow geometric scaling without a loss in data resolution, enabling flexible zoom levels.

3. Information Density: Vector databases can store a large amount of data in a compact size.

4. Versatility: It allows for more complex analysis such as proximity and network analysis.

5. Attribute Storage: Vector models can store attributes in each individual point, line and polygon allowing for a wide range of data representation.

6. Straightforward Analysis: They empower simplistic interpretations of distance, connectivity, and area.

7. No pixelation: Unlike raster database, there are no issues of pixelation in vector databases regardless of the level of zoom. 

8. Better for Certain Types of Data: Vector databases are preferable when dealing with data that requires high details, such as boundaries, lines, points and polygons.

9. Topology Support: These databases support topological concepts, which allow them to precisely determine spatial relationships between geometrical entities. 

10. Rendering: They enable easier rendering for cartographic representation of the stored data and based on specific attributes.
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
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_api_user_js (B1LBnXqCcs8SJRqmycyD-51L0Z-61aEYoipxtdqacqzmdspkjm3y2DOFbok4vZKuHdznSQkrOc1T07le1ezF4g)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "email": "Sincere@april.biz",
        "company": {
          "bs": "harness real-time e-markets",
          "name": "Romaguera-Crona",
          "catchPhrase": "Multi-layered client-server neural-net"
        },
        "name": "Leanne Graham",
        "website": "hildegard.org",
        "address": {
          "geo": {
            "lat": "-37.3159",
            "lng": "81.1496"
          },
          "street": "Kulas Light",
          "suite": "Apt. 556",
          "city": "Gwenborough",
          "zipcode": "92998-3874"
        },
        "phone": "1-770-736-8031 x56442",
        "username": "Bret",
        "id": 1
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
Response time: 46ms (served from cache)
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
✓ Created native SWR script: github_user_native (pVqi3V5JH5Shap1URFT_p7_Ag25mF456sOJNZqOdY44lz0_QcoVQozpG8E0Gr9G-7tssmfoPk0oH_jGOJfLnIw)

First call (cache miss - will fetch from GitHub API):
  Response time: 18ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 34ms
  Speedup: 0.5x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (TVHfCv24klTEjWd4vLcD3oTz8w5NXptC6lLS-x-gI5TBEkMyMI1Q_vfDepVtO3_9nt-iiRZbINbM-srscF4ejw)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (JNVwQWI6UwjscjA3NECn7c_uGqzdphzoBZNRTbTvkIizIWeVcCKKhFPBKTLqk6nFE5e0JQd-sjfameo20BJy3w)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (UpHjqd4vg2XFU4XQiVSStDLxd2j_OVW48JLNhwNEM8-JP5hyc6o-ZgKkem7J3aSks5ZI2032vNTSzyw-2tLu4w)
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
✓ Created session: cK4c1RLRzmdFY94Huqao9lMRpiRFmjyKn57uFA31IFGR35ZBurflVaBOaDUChh1bn3rYepePK8XqqIde7fBCBA

=== Sending Chat Message ===
Message ID: M5znkbl_nah3s8QBhmdldqYSaMOaOjChnoj6w74xn9n9pHKOTpQScJAedSnyQQLmqvTQCPHpwJ9_NR7Z1GgHTA

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
    price: 499,
    id: 'Fql5pGp1QSRnDdgLzB9ovJP1AxlSsy36r11xLuDmpQFCH9Mg2LZG9kSuYbkyPwYN26NqtyqKasot0lJZ8k0Pqw',
    description: 'Fully managed cloud database service product',
    name: 'ekoDB Cloud'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_js',
  record: {
    name: 'ekoDB',
    price: 99,
    description: 'A high-performance database product with AI capabilities',
    id: 'VtbAnbnxq3wnAVRAr9-fFal-mjwZqr6ch7Tn382P-U-wgHSRPZQA9x6Q7_-EqA5eQETUxOoXDwrsNCosEw3Vgw'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_js',
  record: {
    price: 299,
    name: 'ekoDB Pro',
    description: 'Enterprise edition product with advanced features',
    id: 'nHH5egZJxfyujHII3qbyI6wl2fDj83xyNTUV0xs2KwRIx6LClI9HmS-dA2JX7IiZ8xL8MWtwXAHpPQ1cKCzSHQ'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 3576ms

=== Token Usage ===
Prompt tokens: 1994
Completion tokens: 90
Total tokens: 2084

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: QIEqzBprHglQlPN4viIw37u2u7iUcYyUEIF_5vVx4W85CedFDN9EtUT-ErhtrWM_x6MJfsNVkOrqLAsAYf57zg

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: High-performance database product
- **Price**: $99

If you need more information or have any other questions, feel free to ask!

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
✓ Created second session: kasnThuOgculoicv_7nWrejS7br5hOxCmlWl_U7y2snGs50xK_CjIDvjfZX0nFpwl6x_RzNSPicWVq5Uoq9UIA
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
✓ Created session: K8iegFYmCrTa4WLLGQu0dDpLVapTbvLYr1_DwLhKyxoIVRsukxUYnXgplNkC8YWzjkwRPQ5CJyWga854j5zvGw

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: A high-performance database product
- **Price**: $99

If you need more information or a different product, feel free to ask!

✓ Message 2 sent
  Response: The price of ekoDB is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: zJHHzQmiZO2bpywFw1AVFHnXXQ8e6Tru1ZqYDHM3WIbXcOmbDlkpnBzzi3TpMAwHifyFdXNxBsGJNrMSR6GgJw
  Parent: K8iegFYmCrTa4WLLGQu0dDpLVapTbvLYr1_DwLhKyxoIVRsukxUYnXgplNkC8YWzjkwRPQ5CJyWga854j5zvGw

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: zJHHzQmiZO2bpywFw1AVFHnXXQ8e6Tru1ZqYDHM3WIbXcOmbDlkpnBzzi3TpMAwHifyFdXNxBsGJNrMSR6GgJw (Untitled)
  Session 2: K8iegFYmCrTa4WLLGQu0dDpLVapTbvLYr1_DwLhKyxoIVRsukxUYnXgplNkC8YWzjkwRPQ5CJyWga854j5zvGw (Untitled)
  Session 3: kasnThuOgculoicv_7nWrejS7br5hOxCmlWl_U7y2snGs50xK_CjIDvjfZX0nFpwl6x_RzNSPicWVq5Uoq9UIA (Untitled)
  Session 4: cK4c1RLRzmdFY94Huqao9lMRpiRFmjyKn57uFA31IFGR35ZBurflVaBOaDUChh1bn3rYepePK8XqqIde7fBCBA (Untitled)
  Session 5: fHDVuJqVDvhIUeFzclzW0otYyicL0J6bYPkhFPZBLL-07Wzl5UOrokjX7pRG6bw5oVD0UXBlcwxDIDh8618Gig (Untitled)
  Session 6: ZntAtLRoORyRSaCh09MJ2_5MIHbH4dL5fOu73E2M5AWYUK763VrnrxFk4X_-AzzWZfdeod6PRXhTAIfSmQEKDg (Untitled)
  Session 7: 21c3jnVsMuyprt_egVapuvE9a7vASWHYj6ezVjWhnrCC0BTNksTVV8MjJXMxIWZx3rqEadvQZIGbwx_3amn0ZA (Untitled)
  Session 8: lftxxoYtSpgo8WO20eBMLPLFMkF1zEALhHFuXg2pNzF_PCEUn4cHdYO-R6cik_ZGe2CXyd9AJ14EwSQ5V4ao2g (Untitled)
  Session 9: Ygey_v3geriVjwMfeFvBF8EwRJ6asQwzGYmOei9NU3OkdM9GVrLeWggPIlYjDmjhjbppdknP8R3amvT9Uh16fQ (Untitled)
  Session 10: yW3j-yJjAKOogM3nU22M-JUsM7VhdUUWGR7O46sOwBOC04blROFur1V8kp6n8G7pf_Q0tLT3MxMcCVwz5tEiKw (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: zJHHzQmiZO2bpywFw1AVFHnXXQ8e6Tru1ZqYDHM3WIbXcOmbDlkpnBzzi3TpMAwHifyFdXNxBsGJNrMSR6GgJw

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
✅ [32mJavaScript client examples complete![0m
