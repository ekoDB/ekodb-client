make test-examples-javascript
🧪 [36mRunning JavaScript examples (direct HTTP/WebSocket)...[0m

added 1 package, removed 1 package, and audited 9 packages in 811ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
[34m
╔════════════════════════════════════════╗[0m
[34m║  ekoDB JavaScript Examples Test Suite ║[0m
[34m╚════════════════════════════════════════╝[0m
[34m
=== Checking Server Connection ===[0m
(node:91057) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
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
  id: 'Tcio_q3T1ZuIC5n22f-y1kR8AqwypdDPsXwtiG_EDC1YMf2TKEcl_raz54q9gQW8JpFrFKaqo6xix2A6bPvcgA'
}

=== Find by ID ===
Found: {
  active: { value: true, type: 'Boolean' },
  id: 'Tcio_q3T1ZuIC5n22f-y1kR8AqwypdDPsXwtiG_EDC1YMf2TKEcl_raz54q9gQW8JpFrFKaqo6xix2A6bPvcgA',
  value: { value: 42, type: 'Integer' },
  name: { value: 'Test Record', type: 'String' }
}

=== Find with Query ===
Found documents: [
  {
    id: 'Tcio_q3T1ZuIC5n22f-y1kR8AqwypdDPsXwtiG_EDC1YMf2TKEcl_raz54q9gQW8JpFrFKaqo6xix2A6bPvcgA',
    active: { value: true, type: 'Boolean' },
    value: { type: 'Integer', value: 42 },
    name: { type: 'String', value: 'Test Record' }
  }
]

=== Update Document ===
Updated: {
  active: { type: 'Boolean', value: true },
  value: { type: 'Integer', value: 100 },
  id: 'Tcio_q3T1ZuIC5n22f-y1kR8AqwypdDPsXwtiG_EDC1YMf2TKEcl_raz54q9gQW8JpFrFKaqo6xix2A6bPvcgA',
  name: { type: 'String', value: 'Updated Record' }
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
✓ Inserted test record: KJwI6LS-TK3tqXt4KxOdwXmHpbU_E9NlZU5kxV_lQAHkdq96PcsjcoWkSdsBWCxrTaZUWR6sEipJ18AP1ksPng

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "payload": {
    "data": [
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "gNJIM5t-Z90GX2S83uajBb8Cf_25wpkB7Fp-GaV-ZpiQWf0plIOqYPCcPhkA5i0lNm-RyuZvqPAbCHUrfvb1PQ",
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
        "id": "jCqqfNX1CH8oAOspCeoYtlibdQJHNY0vgR73fxLb98lsm2A2lhAlkEWs2K4JJ3fG0iHF4joH55M1tlEJ2xM6hw",
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
        "id": "t72bpt21jeSg4T5Hfo6Fk9U-NA8EZPcpKDWKpNfxWbDNAhHHFhjk2RkrVZVqsb_YoUfIXZy-UOQ7xUd1LDrPFQ",
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
        "id": "KJwI6LS-TK3tqXt4KxOdwXmHpbU_E9NlZU5kxV_lQAHkdq96PcsjcoWkSdsBWCxrTaZUWR6sEipJ18AP1ksPng",
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
🚀 ekoDB Scripts Example (JavaScript/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: oxgnRCPdmUVugISRkikdAWUed1eE0aUM77FNyny9zw5w68e29RSx3CBJzHzC8EauwUfkDukrlL5xTghqH1wOJA
📊 Found 30 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: 9Ncx68mwJQFvoL10P_tBBWlMpg5AqskxX77qIE1ivNfsBvur_fqgCtGIJSVqxaAEhIEu2-d0xfg96J0ey_-aTg
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: aUwLbnXTkoEz-axcmqytg5F0vCo5DTPO1DR1iu5Mfh2L0cMJawYA4EpEigSI0ZENOTYfKD7ABIh3q9OE8jm18Q
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"status":"inactive","avg_score":50,"max_score":90,"count":10}
   {"status":"active","max_score":100,"count":10,"avg_score":60}

📝 Example 4: Function Management

📋 Total scripts: 53
🔍 Retrieved script: Get Active Users
✏️  Script updated
🗑️  Script deleted

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
cache:product:1: { price: 29.99, name: 'Product 1' }
cache:product:2: { price: 39.989999999999995, name: 'Product 2' }
cache:product:3: { price: 49.989999999999995, name: 'Product 3' }

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
Collection created with first record: FKhl-hDcMpN8_EGMU132nsYPX8zZtQr_H94GFy5xSnzcwjWyTaQDctBwZp9GhIQDMTJm1vbU4eK_1TKyl29Pow

=== List Collections ===
Total collections: 27
Sample collections: [
  'schema_employees_client_go',
  'chat_configurations__ek0_testing',
  'schema_documents_client_py',
  'schema_users_client_py',
  'edge_cache_py'
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
Created Alice: $1000 - ID: suggzqQ8KgQUPW6rc7K_ayGizEocu2mi0xwgbofWtdTykAMzcrFSLECQZAfElCLihcSbj-aBPTrdkyw5J9SFBA
Created Bob: $500 - ID: hc3PkICa5p-jqQvqGg8hpmJL35-zbkkAphHeGakba1PjaQoIQRtniwV3Zfn4imqrXAlWfn8uZ63FDjAZKhjmmw

=== Example 1: Begin Transaction ===
Transaction ID: 91d73a89-29bc-4fc3-9c18-0e6138b39562

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: 800
Bob: 700

=== Example 5: Rollback ===
New transaction: f8cdbfc9-e499-49f6-ab49-99cfb36947b5
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: 700

=== Cleanup ===
✓ Deleted test accounts

✓ All transaction examples completed
[32m✓ transactions.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/crud_scripts.js ===[0m
🚀 ekoDB Complete CRUD Scripts Example
============================================================
Demonstrates:
  • Insert + Verify (using Query)
  • Query + Update Status + Verify
  • Query + Update Credits + Verify
  • Query Before Delete + Delete + Verify Gone

Each Script shows Functions chaining with proper verification
============================================================

============================================================
📝 Script 1: Insert + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: iRralNTlPnu5WmVwWjSCRkKn7VX4HA98nQBC-0cDwQ8YS16nLezR_jPzfilciMyowqWvCjqxuFeR7DmquOCKgQ

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Name: [object Object]
   📋 Email: {"type":"String","value":"alice@example.com"}
   📋 Status: [object Object]
   📋 Credits: [object Object]

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: 51lkyWCarYYmO42A8Okon41aYxGT_u6OCLBOcHcBFWV9N3YxDgrinw-h29VBx2AtvkXPwxKzuBqTSP0FBDGmxA

2️⃣ Calling Script (Query + Update + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Status updated to: {"type":"String","value":"active"}
   📋 Name: {"value":"Alice Smith","type":"String"}

============================================================
📝 Script 3: Query + Update Credits + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: yuDEsVCEPjFrprxUM6zsco6ZMMlzYCp2EEXiyO5GJjEDwim2Ox2GJrIpV7hfaZ0i9rlgPNtMP5f1aRi0U6carA

2️⃣ Calling Script (Query + Update Credits + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {"value":0,"type":"Integer"}
   📋 Status: {"type":"String","value":"active"}
   📋 Name: {"value":"Alice Smith","type":"String"}

============================================================
📝 Script 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: dwVRZG6cFxAH0XFd5QFD14W9DMBdMfD-J5jW6tPoJmD1WiuYV8Z1T4zid-mQ3H8ZclFX1GKNKsCF1IOKItCbkA

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: iRralNTlPnu5WmVwWjSC...
   ✅ Deleted script: 51lkyWCarYYmO42A8Oko...
   ✅ Deleted script: yuDEsVCEPjFrprxUM6zs...
   ✅ Deleted script: dwVRZG6cFxAH0XFd5QFD...
   ✅ Deleted collection: users

============================================================
✅ Complete CRUD Scripts Example Finished!
============================================================

💡 Key Takeaways:
   ✅ Scripts chain Functions together
   ✅ Each Script demonstrates operation + verification
   ✅ Parameters make Scripts reusable
   ✅ Verification is built into the Script itself
   ✅ Complete CRUD lifecycle in 4 focused Scripts
[32m✓ crud_scripts.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl/document_ttl.js ===[0m
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
  Output: Document ID = Lxm72X_7k0VxJJA7h_JpOAXnq5pbeC1gfgTD8QxXxy-ymceiT0E38yuzEm-4816kMmcfKOfi2G1FxdMBvbzy_w
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: findById(Lxm72X_7k0VxJJA7h_JpOAXnq5pbeC1gfgTD8QxXxy-ymceiT0E38yuzEm-4816kMmcfKOfi2G1FxdMBvbzy_w)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: findById(Lxm72X_7k0VxJJA7h_JpOAXnq5pbeC1gfgTD8QxXxy-ymceiT0E38yuzEm-4816kMmcfKOfi2G1FxdMBvbzy_w)
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
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl/websocket_ttl.js ===[0m
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
  Output: Document ID = 3-wfd6YXLZjA2kobcrPPjXRR4GAmj-fFX8WxttBQZG-tbTjKqKTX5FkYbtuWmHUFAplBEKOFUQC5m8M3q5xmAw
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: findById(3-wfd6YXLZjA2kobcrPPjXRR4GAmj-fFX8WxttBQZG-tbTjKqKTX5FkYbtuWmHUFAplBEKOFUQC5m8M3q5xmAw)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: findById(3-wfd6YXLZjA2kobcrPPjXRR4GAmj-fFX8WxttBQZG-tbTjKqKTX5FkYbtuWmHUFAplBEKOFUQC5m8M3q5xmAw)
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

> @ekodb/ekodb-client@0.7.1 prepare
> npm run build


> @ekodb/ekodb-client@0.7.1 build
> tsc


up to date, audited 58 packages in 1s

15 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.7.1 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning JavaScript client library examples...[0m

added 1 package, removed 1 package, and audited 9 packages in 814ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: '8RLE4FYQ1tb359lFPI6MYnLNb03kC6ArwW6hE6MRUb5RHhes41QTmMheOjAudBP203PNGcYB-v1Z-BM5yaYz0w'
}

=== Find by ID ===
Found: {
  name: { value: 'Test Record', type: 'String' },
  id: '8RLE4FYQ1tb359lFPI6MYnLNb03kC6ArwW6hE6MRUb5RHhes41QTmMheOjAudBP203PNGcYB-v1Z-BM5yaYz0w',
  value: { type: 'Integer', value: 42 },
  active: { value: true, type: 'Boolean' }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  name: { value: 'Updated Record', type: 'String' },
  id: '8RLE4FYQ1tb359lFPI6MYnLNb03kC6ArwW6hE6MRUb5RHhes41QTmMheOjAudBP203PNGcYB-v1Z-BM5yaYz0w',
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
✓ Inserted test record: mZ_gABlLLT6vDQ1StEYL-0k9hOK_2viJ8BXZ7uz6GHIgQfdGshWNpWAkylLc6JjbE29wD3xReXFBGKxS2TT2bw

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
Collection created with first record: 9S9YIfXPfbpUQIsrxB9q9oNHLgUx13iWqCLkG6RU0vqUqtUd89HgzoQiu1C7mqzgVfUu32Im0rX1ng4vM3Au-w

=== List Collections ===
Total collections: 26
Sample collections: schema_employees_client_go,chat_configurations__ek0_testing,schema_documents_client_py,schema_users_client_py,edge_cache_py

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
cache:product:2: { name: 'Product 2', price: 39.989999999999995 }
cache:product:3: { name: 'Product 3', price: 49.989999999999995 }

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 0 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 6

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: false

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: OcUnhSzI1EcbRDFqYbuBPjrLHEj-pRMWO_zjAUUdSkCYGLRKiEipfAzmGmAx8wE-MwBRkLDSXJqsuPmnI3JMuQ
Created Bob: $500 - ID: 1XIh9Ct-Z0PJq9MCyGqNoOZ-ixK4KO4Pv5ytsRxg9YThutV6CcWVshZZoNvdTn3VFsE3QyNat3JWfBgaGYn_tw

=== Example 1: Begin Transaction ===
Transaction ID: a1c394da-8257-4687-ad02-697a1d1c6f98

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 93b50bb6-ffdc-4f3b-8ac5-2017bb357cdb
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
  1. Score: 25.740, Matched: name.value, email, name, email.value
  2. Score: 12.540, Matched: name.value, name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title.value, title, bio.value, bio
  2. Score: 26.400, Matched: bio.value, title.value, title, bio
  3. Score: 26.400, Matched: bio.value, title.value, bio, title
  4. Score: 26.400, Matched: title, bio, title.value, bio.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio, bio.value, title.value, title
  2. Score: 39.600, Matched: bio, title, bio.value, title.value
  3. Score: 39.600, Matched: title.value, bio.value, bio, title
  4. Score: 39.600, Matched: title, title.value, bio, bio.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.774, Matched: 
  2. Score: 0.750, Matched: 
  3. Score: 0.743, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.697, Matched: title.value, title, content, content.value
  2. Score: 1.510, Matched: title, title.value, content.value, content
  3. Score: 0.300, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: bio, bio.value, skills.value, skills

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
✓ Inserted document: UKmgrVAh9atPxYwswbAlYsRyZHXUXuQO24fzkb6usioyMRxySk4iOErzWSfOlOYkj2DjhYQMwQXTeuCKI5DbGg

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: P6_wj3S8qe3sJ1TExt5b8wflAoI4vijJzdbY-WLHUEcqN7LaQHkCqQXRJohDtOfX-XlCq0Fe6v70uoOsvyLV8w

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
✓ Inserted document with TTL: FNiTcwvx4kB1xwttLw9SGe6p9wjk9kOZAuOFML5CedLULHwHjTH-1Idx5BHgZSO88lyZX8gx1PpBD944t1GwSA

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 5 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
=== ekoDB as Edge Cache - Simple Example ===

Creating edge cache function...
✓ Edge cache script created: Fb4ISdJAUbhyh_SgzoniO4r6KDhWPi1MhIeJe-zoyqAYm7-vhxh5hN8r1FXbBM6TNHXe7GMVubGp3JBhbD57gQ

Call 1: Cache miss (fetches from API)
Response time: 545ms
Result: {
  "records": [
    {
      "id": "YT2NziT8RYvDnFQaNuIDlkIsmJTBaYl6jE6F6zdgn8L_xY5D9pMfb6UNLW04dC6etJk3zszqqYT9PAOsDTFKDw",
      "data": {
        "value": {
          "longitude": -73.99309,
          "elevation": 32,
          "current_units": {
            "time": "iso8601",
            "interval": "seconds",
            "temperature_2m": "°C"
          },
          "latitude": 40.710335,
          "current": {
            "time": "2026-01-05T15:30",
            "interval": 900,
            "temperature_2m": 0.1
          },
          "timezone": "GMT",
          "generationtime_ms": 0.025033950805664062,
          "timezone_abbreviation": "GMT",
          "utc_offset_seconds": 0
        },
        "type": "Object"
      },
      "cached_at": "2026-01-05T15:35:23.306Z"
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
Response time: 536ms (1x faster!)
Result: {
  "records": [
    {
      "id": "QFoIIFm1Jw6DA_ElPB8lj2c1s7fqT62XzXaZlCyXMVOdTCyI6Li1CKND5SzyrjSkk7RiGiZjOf5USFAZkXCpeA",
      "data": {
        "value": {
          "generationtime_ms": 0.022292137145996097,
          "elevation": 32,
          "current_units": {
            "interval": "seconds",
            "temperature_2m": "°C",
            "time": "iso8601"
          },
          "timezone": "GMT",
          "timezone_abbreviation": "GMT",
          "latitude": 40.710335,
          "utc_offset_seconds": 0,
          "current": {
            "interval": 900,
            "time": "2026-01-05T15:30",
            "temperature_2m": 0.1
          },
          "longitude": -73.99309
        },
        "type": "Object"
      },
      "cached_at": "2026-01-05T15:35:23.306Z"
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

🚀 ekoDB Scripts Example (JavaScript Client)

✅ Client initialized (token exchange automatic)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: BwFA5JMReVlSt30TDFJ4T9XwFOYKTz1m7740M2xlQzQ20gt5Ag_JXAo3OB5suOyBtQ3m95IRhytWkU9xjpe98A
📊 Found 10 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved
📊 Found 10 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved
📊 Statistics: 2 groups
   {"status":"active","count":5,"avg_score":60}
   {"status":"inactive","count":5,"avg_score":50}
⏱️  Execution time: 0ms

📝 Example 4: Script Management

📋 Total scripts: 56
🔍 Retrieved script: Get Active Users
✏️  Script updated
🗑️  Script deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

📝 Example 5: Multi-Stage Pipeline

✅ Multi-stage script saved
📊 Pipeline executed 2 stages
⏱️  Total execution time: 0ms
📈 Stage breakdown:

📝 Example 6: Count Users

✅ Count script saved
📊 Total user count: 10
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Deleted collection
✅ Deleted test scripts

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
   Department: {"type":"String","value":"engineering"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using CallFunction to replace inline logic in SWR pattern...

✅ Saved reusable function: fetch_and_store_github
✅ Saved SWR function using composition: swr_github_user

First call (cache miss - will fetch from GitHub):
   ⏱️  Duration: 140ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 89ms
   📊 Records: 1
   🚀 Cache speedup: 1.6x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

✅ Level 1 function: validate_user
✅ Level 2 function: fetch_slim_user (calls validate_user)
✅ Level 3 function: count_validated_user (calls fetch_slim_user)

📊 Result from 3-level nested composition:
   Records: 1
   Name: N/A
   Department: N/A
   Record count: 1

🎯 Key Benefit: Each function is independently testable and reusable!
   - validate_user: Used in 100 different workflows
   - fetch_slim_user: Used in 50 workflows
   - count_validated_user: Specific workflow


✅ All composition examples completed!
🚀 ekoDB JavaScript Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved: 56XnyJU8XcfyYfXlXDnYYDmA4hHWo4BC_llQrrWaywofNoH9smNlX-lVECvd7EzDFp7ECqKpHw44H_gD4o-zlg
📊 Found 2 product groups
   {"avg_price":575.6666666666666,"category":"Electronics","count":3}
   {"category":"Furniture","avg_price":474,"count":2}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved
📊 Found 2 categories
   {"category":"Electronics","count":3}
   {"category":"Furniture","count":2}
⏱️  Execution time: 0ms

📝 Example 4: High Rating Products

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 5: Script with Parameter Definition

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 6: Multi-Stage Pipeline (FindAll → Group → Count)

✅ Script saved
📊 Pipeline executed 3 stages
⏱️  Total execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All complete script examples finished!

💡 This example demonstrates ekoDB's Script system:
   ✅ FindAll operations
   ✅ Group aggregations (Count, Average)
   ✅ Multi-stage pipelines (FindAll → Group → Count)
   ✅ Parameter definitions
   ✅ Script management (save, call, delete)
🚀 ekoDB JavaScript KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: I2bHDMyPQlyatcTTJQuy5bZ4054wf284nzkO-I9nCk9GglZqQWuA73Gar3gfw-V0sLVmrS4q_lNybfaL-iLJww
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: d0QLbyQ79j66gGBQD6H84U-ZldpTS9kW5cEODfrQbceSeO2eHML2jjROBp2wO5crzzQnSGN0asrpFy0bHl0RTg
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"userId":"user_abc","role":"admin"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: WtW2dcYw51fGUXZr2lPRjdzft6iQl_FLVKJBzWaYd0WznT4ONEllb2ykUJpElB6ZQoXo9eWjhGE7QdrFsIL30g
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Script

✅ Script saved: Mh0pHujcHSJ80EnTMFF0E_bk_MesVz6KwE6_GwptgCIW-8Jati9cN-3nTROg3diV3ft0egIi3IlaXY4S7niBkg
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
🚀 ekoDB CRUD Scripts Example

📋 Setting up test data...
✅ Created 10 test users

📝 Example 1: List All Users

✅ Script saved
📊 Found 10 users
⏱️  Execution time: 0ms

📝 Example 2: Count Users by Status

✅ Script saved
📊 User counts by status:
   active: 7 users
   inactive: 3 users
⏱️  Execution time: 0ms

📝 Example 3: Average Score by Role

✅ Script saved
📊 Average scores by role:
   user: 70.0 (7 users)
   admin: 20.0 (3 users)
⏱️  Execution time: 0ms

📝 Example 4: Top Users by Score

✅ Script saved
📊 Users (showing first 5 of 10):
   1. User 3 (user3@example.com) - Score: 30
   2. User 7 (user7@example.com) - Score: 70
   3. User 10 (user10@example.com) - Score: 100
   4. User 2 (user2@example.com) - Score: 20
   5. User 6 (user6@example.com) - Score: 60
⏱️  Execution time: 0ms

📝 Example 5: User Summary Statistics

✅ Script saved
📊 User summary (2 groups):
   inactive users: 3 users, Total Score: 180
   active users: 7 users, Total Score: 370
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD script examples completed!
🚀 ekoDB Search Scripts Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Script saved
📊 Found 5 documents
   1. Getting Started with ekoDB (Database)
   2. Introduction to Machine Learning (AI)
   3. Vector Databases Explained (Database)
   4. Natural Language Processing (AI)
   5. Database Design Principles (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Script saved
📊 Documents by category:
   Database: 3 documents
   AI: 2 documents
⏱️  Execution time: 0ms

📝 Example 3: Select Specific Fields

✅ Script saved
📊 Document titles (5 docs):
   1. Getting Started with ekoDB
   2. Introduction to Machine Learning
   3. Vector Databases Explained
   4. Natural Language Processing
   5. Database Design Principles
⏱️  Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Script saved
📊 Projected documents (showing first 3):
   1. Getting Started with ekoDB
   2. Introduction to Machine Learning
   3. Vector Databases Explained
⏱️  Execution time: 0ms

📝 Example 5: All Document Fields

✅ Script saved
📊 All documents (5 total, showing first 2):
   1. Getting Started with ekoDB (Database)
   2. Introduction to Machine Learning (AI)
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples completed!
🚀 ekoDB Advanced Scripts Example

📋 Setting up test data...
✅ Created 10 products

📝 Example 1: List All Products

✅ Script saved
📊 Found 10 products
⏱️  Execution time: 0ms

📝 Example 2: Group Products by Category

✅ Script saved
📊 Found 2 categories
   Electronics: 6 items (avg $325.67)
   Furniture: 4 items (avg $294.00)
⏱️  Execution time: 0ms

📝 Example 3: Count All Products

✅ Script saved
📊 Total products: 10
⏱️  Execution time: 0ms

📝 Example 4: Multi-Stage Aggregation

✅ Script saved
📊 Category analysis (2 categories):
   Electronics:
      Products: 6 | Stock: 232 | Avg Rating: ⭐4.52
   Furniture:
      Products: 4 | Stock: 43 | Avg Rating: ⭐4.26

⏱️  Total execution time: 0ms
📈 Pipeline stages:

📝 Example 5: Project Specific Fields

✅ Script saved
📊 Product summaries (10 items, showing first 3):
   1. Monitor 27" - $399 (⭐4.6)
   2. Keyboard - $89 (⭐4.4)
   3. Desk Chair - $349 (⭐4.2)
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced script examples completed!
🚀 ekoDB AI Scripts Example

⚠️  Note: These examples require AI API credentials (OpenAI, etc.)

📋 Setting up test data...
✅ Created 3 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
🤖 AI Response:
   1. Precision: Vector data are represented by exact coordinates, ensuring high accuracy.
2. Flexibility: Vector databases can handle different types of data including points, lines, and polygons.
3. Scalability: Vector data can be easily scaled up or down without losing quality.
4. Connectivity Information: Vector databases store topological information, enabling analysis of connectivity, adjacency, and proximity.
5. Smaller File Sizes: Vector data often require less storage space compared to raster data.
6. Easy to edit: Vector data can be easily manipulated and edited.
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embedding script saved
📊 Generated embeddings for 3 articles
   1. "Advanced Query Patterns" - 1536D vector
   2. "Draft Article" - 1536D vector
   3. "Getting Started with ekoDB" - 1536D vector
⏱️  Execution time: 0ms

📝 Example 3: List All Articles

✅ Script saved
📊 Found 3 articles
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All AI examples completed!
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_github_user_js (uHAxQem46AtLUdZWOYKkyIB9M1vQY2Ewt9KYc_vVDF90jIaXVpph1dxf5ppaE8hpEvkGVGRCTRXin3xYRnBcgg)

Step 2: First call - Cache miss, fetches from GitHub API
Result: {
  "records": [
    {
      "cached_at": "2026-01-05T15:35:32.075Z",
      "data": {
        "type": "Object",
        "value": {
          "hireable": null,
          "events_url": "https://api.github.com/users/torvalds/events{/privacy}",
          "email": null,
          "login": "torvalds",
          "starred_url": "https://api.github.com/users/torvalds/starred{/owner}{/repo}",
          "user_view_type": "public",
          "bio": null,
          "subscriptions_url": "https://api.github.com/users/torvalds/subscriptions",
          "avatar_url": "https://avatars.githubusercontent.com/u/1024025?v=4",
          "organizations_url": "https://api.github.com/users/torvalds/orgs",
          "created_at": "2011-09-03T15:26:22Z",
          "node_id": "MDQ6VXNlcjEwMjQwMjU=",
          "company": "Linux Foundation",
          "html_url": "https://github.com/torvalds",
          "location": "Portland, OR",
          "blog": "",
          "twitter_username": null,
          "updated_at": "2025-11-24T04:16:14Z",
          "following": 0,
          "id": 1024025,
          "public_gists": 1,
          "received_events_url": "https://api.github.com/users/torvalds/received_events",
          "gists_url": "https://api.github.com/users/torvalds/gists{/gist_id}",
          "followers_url": "https://api.github.com/users/torvalds/followers",
          "type": "User",
          "followers": 270115,
          "gravatar_id": "",
          "public_repos": 9,
          "site_admin": false,
          "url": "https://api.github.com/users/torvalds",
          "following_url": "https://api.github.com/users/torvalds/following{/other_user}",
          "name": "Linus Torvalds",
          "repos_url": "https://api.github.com/users/torvalds/repos"
        }
      },
      "id": "Unw90ZB-18gyHV8fbsFsSQ62RH3gJ87KIp5zHi5xNWKT7zf3qUXR7m4X-WtPBA3oIfkjN8RfoLWv7L-fYv4fwQ"
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
Response time: 87ms (served from cache)
✓ Lightning fast cache hit

🧹 Cleaning up...
✓ Cleanup complete

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: SROxaE3L-lNTUlKwIAeL-zpxfGSi5UxqWQ0ZGQtYT9poNPk1F3TCDkIyRlZD85pxMc3x66sj4KF8eOfNh2JZXA

=== Sending Chat Message ===
Message ID: v5wwwhOb3jdjUK-eThMwklCc8Kv3btF5v0N8fF691wvVpwqMgoFY7epgTYCv5cy5V-LmUDZ2HM6Ovx4sN-_OAQ

=== AI Response ===
Based on the information available, the following products are available:

1. ekoDB: This is a high-performance database product with AI capabilities. It is available for $99.

2. ekoDB Pro: This is an enterprise edition product with advanced features. It is priced at $299.

3. ekoDB Cloud: This is a fully managed cloud database service product. You can purchase this for $499.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_js',
  record: {
    description: 'A high-performance database product with AI capabilities',
    price: 99,
    id: 'JcBvJauexwiigmNHTiDMXxzmbVMHgDribIpo19nO5TNsqxrOnwrLtRLKOYJznR9qX_3Y2hyG_-tLj-O_nfE0iQ',
    name: 'ekoDB'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_js',
  record: {
    name: 'ekoDB Pro',
    id: 'v6dJ6uXwesybbWAfLtPtJ2PgfZiulNrEmQvRh34j25XiIJ3XYbS_tmhDC4xAj-jQIi6pgYDEoNNOGeKyeTQLbg',
    price: 299,
    description: 'Enterprise edition product with advanced features'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_js',
  record: {
    id: 'ErYQpoyJ1W6CX6t9mFK8OMixUKjNRUNhRR9fhYbiqvF98mgy-ls_u_r_9CdJ5c4Ls6Jz5XQIwU54ubdkMPAuVw',
    price: 499,
    name: 'ekoDB Cloud',
    description: 'Fully managed cloud database service product'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 4097ms

=== Token Usage ===
Prompt tokens: 601
Completion tokens: 85
Total tokens: 686

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: STun_ymJxIvap5y8xQO09XFKMhLrqmwsgaxT3yXNizT1nYqTJO8QEYeKweMr6E4HDzPAjj0B-KpOQBj2I68FDg

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information provided, ekoDB is a product available for purchase. It is a high-performance database product that costs $99.

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
✓ Created second session: KtGWRjwz3MuzuB5S-zCwO_-s3Y6bavbHySMoNnVthOWGz52p8QDEqAR0pxWNyG2FQDyxu6OEBZlEjBsVN5pzhg
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 4

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 3

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 0sOqNcDmP14nTc6QgvpkXIjfNzwoEeP21Q3aS8ppYfHz6yBQGM5nIN53XE2YYLp8xODkDCmoWi2YpCQclyGqYg

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information available, one product that is available is ekoDB. It is a high-performance database product and is priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: n7X09lcGBKee4hjaIhmeW5or6uxIWk8td-7YerjLQj00W7q5lWDZAholAZWtojI-1a1XNmMa-j6QoH4v8yVgaA
  Parent: 0sOqNcDmP14nTc6QgvpkXIjfNzwoEeP21Q3aS8ppYfHz6yBQGM5nIN53XE2YYLp8xODkDCmoWi2YpCQclyGqYg

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: n7X09lcGBKee4hjaIhmeW5or6uxIWk8td-7YerjLQj00W7q5lWDZAholAZWtojI-1a1XNmMa-j6QoH4v8yVgaA (Untitled)
  Session 2: 0sOqNcDmP14nTc6QgvpkXIjfNzwoEeP21Q3aS8ppYfHz6yBQGM5nIN53XE2YYLp8xODkDCmoWi2YpCQclyGqYg (Untitled)
  Session 3: KtGWRjwz3MuzuB5S-zCwO_-s3Y6bavbHySMoNnVthOWGz52p8QDEqAR0pxWNyG2FQDyxu6OEBZlEjBsVN5pzhg (Untitled)
  Session 4: SROxaE3L-lNTUlKwIAeL-zpxfGSi5UxqWQ0ZGQtYT9poNPk1F3TCDkIyRlZD85pxMc3x66sj4KF8eOfNh2JZXA (Untitled)
  Session 5: i__SpHRQbfx2YOk5wES38fvcZ8XvmLjLBCSPblkPuOJ6aHpl2886G9c4bWcV9IV1aEK-r5Be_60X8Ebcadde9w (Untitled)
  Session 6: Ln9JpayhkcQBVZmhSyUlBii4_1YO7Jv735ptsTk2UMjzkE_C8d2oCYSqlcBdq1P1D5N499ZseVzWUcLa-nEg2Q (Untitled)
  Session 7: tTeTYVvoCpkCE8mwvywHqGTvhBESl6Y3-hK-sJYWZuTZ84GtFd9LpyY1FiSJkud0UbisY45OGTN7phcAe77HKQ (Untitled)
  Session 8: HlgLEuO5eVHTzp6emc9vrrKYyDWQ_IXDgKzI1_2UEdimEilMntvib7YtjlQgvo4ULsDrHzblIa1vlqaZnRedrA (Untitled)
  Session 9: 4XcmiBaLEOBsWSazHwzTs9wPONLFauBk7LZdGPM9-ARmVHIYgjyE9sNHDGnBZQ3o1ShlaUag_CEdZiYGkdgiCQ (Untitled)
  Session 10: aR-Adl9yzHwPf704PXKnJCZnaJlOPXDFagZ71UxUCWKDj7zBUGseVZJaZx2ziMfvsQ20kiytyD3UqhPjfCNiVQ (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: n7X09lcGBKee4hjaIhmeW5or6uxIWk8td-7YerjLQj00W7q5lWDZAholAZWtojI-1a1XNmMa-j6QoH4v8yVgaA

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
✅ [32mJavaScript client examples complete![0m
