make test-examples-javascript
🧪 [36mRunning JavaScript examples (direct HTTP/WebSocket)...[0m

added 1 package, removed 1 package, and audited 9 packages in 979ms

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
  id: 'KrMCk-96JpDWtZp7k0rHsnhDaQ4ZYhNQguoPr_ka8U5LfbaZ2Bdu1x_M2CdwfUPm5vJvqy4_wLwcisodKA7R8A'
}

=== Find by ID ===
Found: {
  id: 'KrMCk-96JpDWtZp7k0rHsnhDaQ4ZYhNQguoPr_ka8U5LfbaZ2Bdu1x_M2CdwfUPm5vJvqy4_wLwcisodKA7R8A',
  active: { value: true, type: 'Boolean' },
  value: { value: 42, type: 'Integer' },
  name: { value: 'Test Record', type: 'String' }
}

=== Find with Query ===
Found documents: [
  {
    id: 'KrMCk-96JpDWtZp7k0rHsnhDaQ4ZYhNQguoPr_ka8U5LfbaZ2Bdu1x_M2CdwfUPm5vJvqy4_wLwcisodKA7R8A',
    active: { type: 'Boolean', value: true },
    value: { type: 'Integer', value: 42 },
    name: { type: 'String', value: 'Test Record' }
  }
]

=== Update Document ===
Updated: {
  id: 'KrMCk-96JpDWtZp7k0rHsnhDaQ4ZYhNQguoPr_ka8U5LfbaZ2Bdu1x_M2CdwfUPm5vJvqy4_wLwcisodKA7R8A',
  value: { value: 100, type: 'Integer' },
  active: { type: 'Boolean', value: true },
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
✓ Inserted test record: vZwvNWwVCWrTYn8o2__XkeknmZJY-EFoUeZOT-3DBF3kmHZPUIW5YLPiC0-nV4KwI5qyLhwCPLxQjZEfAxPvvA

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "messageId": "1775455272957",
  "payload": {
    "data": [
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "aB1SJ5GLXF3qgUKQhvdMwwsI18IxsbaRPTksSu-l_ozaRxe6uI_9DWWk3CCk09ZGNdzrFpPkZlWMZO-6q6bwmQ",
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
        "id": "-Y1QkUwIUNwv7K_DDT9HDqnUZnG4p2GiYfp32sra_Et5SNn0w6eMDDWj5odCNCHBH6pvyV46cJHNCiTKCU7Qxw",
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
        "id": "VwFvCsH16slIZlDP524zmqIksUKfA_T7zROW2we1JqnoW29gEItk2bRWRXQq25rvpvVTmoGjjK2WIKpnM9u5BA",
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
        "id": "vZwvNWwVCWrTYn8o2__XkeknmZJY-EFoUeZOT-3DBF3kmHZPUIW5YLPiC0-nV4KwI5qyLhwCPLxQjZEfAxPvvA",
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

✅ Function saved: HLOSM3DXMWEcZa-FoCfoz0Oz4BKjXfkCWUCO_knBwNpWNxZcisTI34CDZ_vvZM8HRsqvIefa5Jbd8OYyqfK5XQ
📊 Found 10 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Function saved: 1AXbHGou2kjtfuc6TIY7yJPFrXvk24ool-WiRSNVau3O2yTgcZBRZ1niHWMCqCMSGxf-F01-q-m9l-DdRrLtiw
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: D-_36A-JJEcsu7sTGhgD2XVqN9a_7zukcgQqbMVHT4ypG2oYwsEr2_6Ee4_ju2VnlIl5vZFzuMZldhiIrxEIHA
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"avg_score":60,"max_score":100,"count":10,"status":"active"}
   {"max_score":90,"count":10,"status":"inactive","avg_score":50}

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
Collection created with first record: LruOZuUujVcAyjRbgPRIv-evwsHDw1vE-9ieHZp95bamTD_FsiU81DQBu--9xYe9TtJjD7Os3JGR5dy0O1Bckw

=== List Collections ===
Total collections: 25
Sample collections: [
  'schema_employees_client_py',
  'schema_documents_client_ts',
  'test_collection',
  'websocket_test',
  'schema_documents_client_go'
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
Created Alice: $1000 - ID: pO1tKTa6KpsTWLfrIpQIltGm6vU2O2B4bdni7M6JxXHeoQ8vqRsIBKq5uB1JRyNiybdCLdNZ37Zc2eP5wjuFSg
Created Bob: $500 - ID: Lmg61RfTZLScqCJr3ZNS57HhHDdzy09T9POLwEJSABwqZYMzk6K0KNJrh72NLg4uZfb85AgWsRHYng2MDNXWTQ

=== Example 1: Begin Transaction ===
Transaction ID: fb369637-2e2a-4d3e-8647-ba7d48bd2072

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
New transaction: f448c4c4-5d96-45cd-b49c-ab3a259dae85
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
   ✅ Function saved: oEIhp79TXI2aAwRuPPYmYgrxzwvBiHtAlr6ZGPk7wd0FOmpPbVwOVEQflz81ALyAMZeBW32i1iUfdpEJmAbYQw

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
   ✅ Function saved: uNnh59axwqTMpBJxP1lAZ70LcpjD7modgGPSY3MYH-JH2a8lHKDE741MH1Xiysr7nvyUPEgxHAozITml8YCK5g

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
   ✅ Function saved: Ebs3T2DGxs3i-WPmk2GMPHgMjHiOwong4cHqIdCr_ZSGWZoGZgKYHSGOMTF4dvuGU1C78AoznDvmT7yDkvGxXA

2️⃣ Calling Function (Query + Update Credits + Verify)...
   ✅ Function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {"type":"Integer","value":0}
   📋 Status: {"type":"String","value":"active"}
   📋 Name: {"value":"Alice Smith","type":"String"}

============================================================
📝 Function 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: HdQaHyBD7BMmNYcj93MK_G7v9NC2mQ7w9nzkENO3ssgrBuhZFINTNYGSr__rRdQ-yW0sSjkgzcHNJZgpkB4Gvw

2️⃣ Calling Function (Query + Delete + Verify)...
   ✅ Function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted function: oEIhp79TXI2aAwRuPPYm...
   ✅ Deleted function: uNnh59axwqTMpBJxP1lA...
   ✅ Deleted function: Ebs3T2DGxs3i-WPmk2GM...
   ✅ Deleted function: HdQaHyBD7BMmNYcj93MK...
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
  Output: Document ID = i8czp4KF5MGTCfjrF8nOY7fAAhVSfv10sCyFVn-RKrPLCiI16yG10D1JQmZIWQA7iY9NIGLUDSX3ERKme98fYg
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: findById(i8czp4KF5MGTCfjrF8nOY7fAAhVSfv10sCyFVn-RKrPLCiI16yG10D1JQmZIWQA7iY9NIGLUDSX3ERKme98fYg)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: findById(i8czp4KF5MGTCfjrF8nOY7fAAhVSfv10sCyFVn-RKrPLCiI16yG10D1JQmZIWQA7iY9NIGLUDSX3ERKme98fYg)
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
  Output: Document ID = WNogS3Vxv_59433TdcCkBFE9Jn2gua6xDB3Qa0tDw1Su8CPvPY1Exgrz5QBJkoskIVqEinYFqDB4OXPJWFi9OA
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: findById(WNogS3Vxv_59433TdcCkBFE9Jn2gua6xDB3Qa0tDw1Su8CPvPY1Exgrz5QBJkoskIVqEinYFqDB4OXPJWFi9OA)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: findById(WNogS3Vxv_59433TdcCkBFE9Jn2gua6xDB3Qa0tDw1Su8CPvPY1Exgrz5QBJkoskIVqEinYFqDB4OXPJWFi9OA)
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

> @ekodb/ekodb-client@0.15.2 prepare
> npm run build


> @ekodb/ekodb-client@0.15.2 build
> tsc


up to date, audited 52 packages in 1s

17 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.15.2 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning JavaScript client library examples...[0m

added 1 package, removed 1 package, and audited 9 packages in 980ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'w6eEzP7_BjC1ifUFsuo2M4TjPEMBoioDJkDfmwA_KRlowqvjbJLFW-CMFSC_4BpsrAHC09PwOsNMnqarY_rQFA'
}

=== Find by ID ===
Found: {
  id: 'w6eEzP7_BjC1ifUFsuo2M4TjPEMBoioDJkDfmwA_KRlowqvjbJLFW-CMFSC_4BpsrAHC09PwOsNMnqarY_rQFA',
  active: { type: 'Boolean', value: true },
  value: { value: 42, type: 'Integer' },
  name: { type: 'String', value: 'Test Record' }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  name: { value: 'Updated Record', type: 'String' },
  id: 'w6eEzP7_BjC1ifUFsuo2M4TjPEMBoioDJkDfmwA_KRlowqvjbJLFW-CMFSC_4BpsrAHC09PwOsNMnqarY_rQFA',
  active: { type: 'Boolean', value: true },
  value: { value: 100, type: 'Integer' }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: dPSjQYr7klezOW3vuN4xjC3tmzXZ00kDYQsSnNevHqTsvmAwaaUY1SYXp3aUT5N3VL_oRyizdBEENWOCrD-WkA

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
Collection created with first record: H7j03SBKa6Q2G5jAPBokNFixD2gwJhEBVyBndk-O91wccnhs_AFX2Q3n-83yddKf0n1TtmWEVd3J6JZF9c8HJw

=== List Collections ===
Total collections: 24
Sample collections: schema_employees_client_py,schema_documents_client_ts,test_collection,websocket_test,schema_documents_client_go

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
cache:product:3: { name: 'Product 3', price: 49.989999999999995 }

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
Created Alice: $1000 - ID: gY1jzuH8fxx7kvucPItB2Co2mibTkubML0LGDvj2SQ937HyuC1a1vnt0meq4enVpm1V0lQlTHESod6gZ0L_NNQ
Created Bob: $500 - ID: PyPqs-M9jWmDiDZOd8dYPQcEW-OdkqLhnb96fsbcEop2N9n89XkBTDpOf1zezvsKCAKDNmrh3g14BJAPPGiTNw

=== Example 1: Begin Transaction ===
Transaction ID: 06d10f45-fba6-4bdf-b716-b6dc2a39dfd4

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 84805656-da3a-406d-b0dd-1b11739d243f
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
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title.value, bio.value, bio, title
  2. Score: 26.400, Matched: title.value, bio.value, title, bio
  3. Score: 26.400, Matched: bio, title, bio.value, title.value
  4. Score: 26.400, Matched: bio.value, bio, title.value, title

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title.value, title, bio, bio.value
  2. Score: 39.600, Matched: title.value, bio, title, bio.value
  3. Score: 39.600, Matched: title.value, bio, title, bio.value
  4. Score: 39.600, Matched: title, title.value, bio.value, bio

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio.value, bio

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.759, Matched: 
  2. Score: 0.751, Matched: 
  3. Score: 0.747, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.704, Matched: title.value, content, title, content.value
  2. Score: 1.500, Matched: content, content.value, title, title.value
  3. Score: 0.299, Matched: 

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
✓ Inserted document: hDgJZeqOr5VTL93h_x9CgEYMnF0WwO_s4_0Syr96hjaVz1Ywq9ozvFr5w_1PFy0puedBSuE6dvqZBZjd18B04g

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: Zdw8P6h6NBDky2c8AiHtj-HtwCCC7o2rnMtRsYn4Yfu2nf3X5WWq7NVzIj_U5Lq7JEvwsICG8maVfiBlygvVbw

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
✓ Inserted document with TTL: sXLOUOGYqPhnTmnQNEzMoEl7JDE2qYipCOSpKiTvfpwEjLRyYHy26lNvOenmJBuIM_A8MtZWfto23gAMuPwlLA

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
✓ Edge cache script created: QNMKclvjxHcFd0KuZtQqZa0WkC8-x3joc1xMB9-oYZSuBfoczzWmtOAhivQtuQW_j8UT2dMY8kuTkREXJoFrnw

Call 1: Cache miss (fetches from API)
Response time: 42ms
Result: {
  "records": [
    {
      "value": {
        "timezone_abbreviation": "GMT",
        "utc_offset_seconds": 0,
        "generationtime_ms": 0.017523765563964844,
        "elevation": 32,
        "current": {
          "interval": 900,
          "time": "2026-04-06T05:45",
          "temperature_2m": 6
        },
        "latitude": 40.710335,
        "current_units": {
          "time": "iso8601",
          "interval": "seconds",
          "temperature_2m": "°C"
        },
        "timezone": "GMT",
        "longitude": -73.99308
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
Response time: 42ms (1x faster!)
Result: {
  "records": [
    {
      "value": {
        "timezone_abbreviation": "GMT",
        "utc_offset_seconds": 0,
        "generationtime_ms": 0.017523765563964844,
        "elevation": 32,
        "current": {
          "interval": 900,
          "time": "2026-04-06T05:45",
          "temperature_2m": 6
        },
        "latitude": 40.710335,
        "current_units": {
          "time": "iso8601",
          "interval": "seconds",
          "temperature_2m": "°C"
        },
        "timezone": "GMT",
        "longitude": -73.99308
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

✅ Function saved: 6oXTBUsWgOwVQoYRY39UUYvSuHicW0s-ds4a7MSJVwoTY7LN6teCXDv03foAPZEsFT5LLzpMyCnPyM8sdNlflg
📊 Found 10 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Function

✅ Function saved
📊 Found 10 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Function

✅ Function saved
📊 Statistics: 2 groups
   {"avg_score":50,"count":5,"status":"inactive"}
   {"count":5,"avg_score":60,"status":"active"}
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
   Name: {"type":"String","value":"User 1"}
   Department: {"type":"String","value":"engineering"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 180ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "phone": "1-770-736-8031 x56442",
    "id": 1,
    "username": "Bret",
    "name": "Leanne Graham",
    "email": "Sincere@april.biz",
    "company": {
      "name": "Romaguera-Crona...

Second call (cache hit - from cache):
   ⏱️  Duration: 38ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "phone": "1-770-736-8031 x56442",
    "id": 1,
    "username": "Bret",
    "name": "Leanne Graham",
    "email": "Sincere@april.biz",
    "company": {
      "name": "Romaguera-Crona...
   🚀 Cache speedup: 4.7x faster!

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

✅ Function saved: T8lSNTXiAgMnL5MXb9uKGtCzEqx3BkNvaPwvq3zsqNMol3F4iuCpbFqYWw9SSYoLeHtQwyHeP6aWpA9-rwCuwA
📊 Found 2 product groups
   {"avg_price":575.6666666666666,"category":"Electronics","count":3}
   {"count":2,"avg_price":474,"category":"Furniture"}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   {"category":"Electronics","count":3}
   {"count":2,"category":"Furniture"}
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

✅ Inserted order: mWrhSBFtIt_xcnH0I3b742TvEvdsegJuY4k85S7cu0QP0Y8ykpVz6f0hFX606Erii1_XBeoktSXzU3MVon9DvQ
✅ Inserted 2 products with wrapped types

📝 Example 2: Function with Wrapped Type Parameters

✅ Function saved: pXDXoH3GBWwa6QWHjJ_nqTU5e_18B7Q7nfkSaojDZD1Cb1YRU0CbWbcrgHGY1HbooJ3-6DGG5qWwYkocDxupGA
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"userId":"user_abc","role":"admin"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: Fr2ZZfEhpldTpdzV9Htk3TYJ9COOICTxxSSk3FEubeFYjAnLZ6EimObiTnxUQJSo1XBlfcGrsa1TvMrfBmLG2Q
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Function

✅ Function saved: ppuN7jTPeDy7fln_lfcyOdLSGXzadWBXtN0xB4X7Pz6jRnMbo82Q8b_S0VKmOjQAfMORdncJB56BxKhgBerTCA
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
   user: 70.0 (7 users)
   admin: 20.0 (3 users)
⏱️  Execution time: 0ms

📝 Example 4: Top Users by Score

✅ Function saved
📊 Users (showing first 5 of 10):
   1. User 9 - Score: 90
   2. User 10 - Score: 100
   3. User 7 - Score: 70
   4. User 6 - Score: 60
   5. User 3 - Score: 30
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
   2. Natural Language Processing (AI)
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
   1. Getting Started with ekoDB
   2. Natural Language Processing
   3. Vector Databases Explained
   4. Database Design Principles
   5. Introduction to Machine Learning
⏱️  Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Function saved
📊 Projected documents (showing first 3):
   1. Getting Started with ekoDB
   2. Natural Language Processing
   3. Vector Databases Explained
⏱️  Execution time: 0ms

📝 Example 5: All Document Fields

✅ Function saved
📊 All documents (5 total, showing first 2):
   1. Getting Started with ekoDB (Database)
   2. Natural Language Processing (AI)
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
   3. Office Lamp - $79 (⭐4.05)
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
   1. Higher Precision: Vectors maintain data fidelity even at high zoom levels.
2. Scalability: Vector data can be easily scaled up or down without any loss of quality.
3. Efficient Storage: Vector databases use less space compared to raster databases when storing complex data.
4. Easy to Edit: Vector files can be easily manipulated and edited according to requirements.
5. Signal Representation: They are more suitable for representing discrete data like boundaries, lines, points, etc.
6. Query Efficiency: Queries on vector data can be faster and more efficient as they are based on precise coordinates.
7. More Styling Options: They provide more flexibility for rendering, symbolizing, and visualizing data.
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
✓ Created SWR script: fetch_api_user_js (EA9pgCTdwULmm50UhdRUsKHmjO1gprQGOcwn7_gZPD_HvcQ0C2bn-alFc79e9dD6TSRK_VLPf6_Ru7m0nnez2w)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "username": "Bret",
        "company": {
          "bs": "harness real-time e-markets",
          "name": "Romaguera-Crona",
          "catchPhrase": "Multi-layered client-server neural-net"
        },
        "email": "Sincere@april.biz",
        "id": 1,
        "name": "Leanne Graham",
        "phone": "1-770-736-8031 x56442",
        "website": "hildegard.org",
        "address": {
          "zipcode": "92998-3874",
          "suite": "Apt. 556",
          "geo": {
            "lat": "-37.3159",
            "lng": "81.1496"
          },
          "street": "Kulas Light",
          "city": "Gwenborough"
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
Response time: 41ms (served from cache)
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
✓ Created native SWR script: github_user_native (Atd1mVtKNWgRG6ypIFC0LclHtlgh5T6fhM5bmc4ZEPPd6PNrw9Ypx6kSvNE5iwC0sLd4p5fZXNt8A6JTnjm_6Q)

First call (cache miss - will fetch from GitHub API):
  Response time: 16ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 17ms
  Speedup: 0.9x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (OnmZv3vFQc2xM35_o4Uf6cmU0p8X34b5Yq4dP9aAjTnZ-57Mrsv-rm43K8rAVnahv7a5vKFEngPMa__MgJOpWw)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (HDQqOzQ0J7egKLrcJ3IE-dZK5Qo-CTA69JoyNEbU8VYsKsyEvHWzewTxE0y-CQeJY7ui7WGSuIyQqEsqZfdD8A)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (tRaUaEfWGTO_RHs9akqq3Zn4qsOex_ipxJC4aoMlfEZt6865nAL7pLYKp_ldb8canhQPW3cR8EUoGRHai3IdYA)
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
✓ Created session: GOF93wU3_qjaY4WBFFPThD_mns8rOAtuUXjdB2bKKoER6QgNGUN6NojehBcq28bYX44EoLkIhFnOZ-I1sTrJgg

=== Sending Chat Message ===
Message ID: ltBnFLiARmao0EcchuU-bbK57FkkUX541olQfhslwDGaYzhkjzDF7NCxiB7lQbpV60QGmfg6QFmKBnAvl4PSGw

=== AI Response ===
The available products and their prices are as follows:

1. **ekoDB Pro**
   - Description: Enterprise edition product with advanced features
   - Price: $299

2. **ekoDB**
   - Description: A high-performance database product with AI capabilities
   - Price: $99

3. **ekoDB Cloud**
   - Description: Fully managed cloud database service product
   - Price: $499

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_js',
  record: {
    description: 'Enterprise edition product with advanced features',
    id: 'puIkRPy4G5nMoMjvmBjfTgC-RVIKMqXrwd8CIJ_E1Oc4ABKY6w75hyxQSGXFKyTj8QPNu53Lyr58GOb3_HiEuw',
    name: 'ekoDB Pro',
    price: 299
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_js',
  record: {
    id: 'QJW7FKtuH3K7VKgX5E8lTWx5l5pZTx8mAM7BL6Ih-rVYP6T_0jgfCyI5fcDsBfRwLnHM53cxkP2gWzHtXP6c-Q',
    name: 'ekoDB',
    description: 'A high-performance database product with AI capabilities',
    price: 99
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_js',
  record: {
    id: 'lFMAUA9YvQT4Nq6_YmJ2YZW40_3P11FrI6JoTyOsdHt4_gTKVgYUtLMRxIhbYIm8noIFku-pN-dioAnWcpWphQ',
    name: 'ekoDB Cloud',
    price: 499,
    description: 'Fully managed cloud database service product'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 2995ms

=== Token Usage ===
Prompt tokens: 1298
Completion tokens: 90
Total tokens: 1388

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: xeS7NAAU6FQPsOQcZY6dteUbj0GHa7MRvCBt0E10eDSziBGMOVUoC5t4nSsymJgGCJZJlaX98JZoWg5onVIfBg

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** High-performance database product
- **Price:** $99

If you need more information or additional products, let me know!

✓ Second message sent

=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of ekoDB is $99. If you have any other questions or need further information, feel free to ask!

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: s0aVSW_67muLx-uCRZr_QL7uB1W0yLcSA6KYe136QKsdQNcjT3MhTZkakIhU6aFtmJ16nZWyWEflYigRfX2Lkw
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
✓ Created session: a1PHZnpqpnwOh497ED2rRI6qQzipMGJU4hFbvoaTvyDyBUCpkMpm13ZGva6QhA9l6uxnoIRTdb7QtOX98U2dgA

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: A high-performance database product
- **Price**: $99

If you need more information or additional products, feel free to ask!

✓ Message 2 sent
  Response: The price of the product **ekoDB** is **$99**.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: RwjDJ6okKb97QLn3oW8GvkAJivhjbymwsnMB6QWeaAFdvQRAGInIZ4R-0lWjLQ-mzoBUYTHUt-S9gkmaCnelUw
  Parent: a1PHZnpqpnwOh497ED2rRI6qQzipMGJU4hFbvoaTvyDyBUCpkMpm13ZGva6QhA9l6uxnoIRTdb7QtOX98U2dgA

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: RwjDJ6okKb97QLn3oW8GvkAJivhjbymwsnMB6QWeaAFdvQRAGInIZ4R-0lWjLQ-mzoBUYTHUt-S9gkmaCnelUw (Untitled)
  Session 2: a1PHZnpqpnwOh497ED2rRI6qQzipMGJU4hFbvoaTvyDyBUCpkMpm13ZGva6QhA9l6uxnoIRTdb7QtOX98U2dgA (Untitled)
  Session 3: s0aVSW_67muLx-uCRZr_QL7uB1W0yLcSA6KYe136QKsdQNcjT3MhTZkakIhU6aFtmJ16nZWyWEflYigRfX2Lkw (Untitled)
  Session 4: GOF93wU3_qjaY4WBFFPThD_mns8rOAtuUXjdB2bKKoER6QgNGUN6NojehBcq28bYX44EoLkIhFnOZ-I1sTrJgg (Untitled)
  Session 5: hjv2vJ9ySHPM3JtRXgCRBxX2wCK5JRXeBOAbppTyYN2JQmYuXYVodT9BOlF_vVpqisOK7SlUT6SnflIhr12jwA (Untitled)
  Session 6: NcKy-_Uqx7KLOJl7eO9DHRmvf8mYRjOouyZWIz47G0-z83h9alj7KARdZjohQHAXz9PQI4Izg9uyrdYUXpIekQ (Untitled)
  Session 7: g22PDETf2BZ5WIw_LqSlp9gnj_QGDSemITI-T1uCPOTIIGdgmZw56o1VRzKNQ2Qkcpgiw9n9EGd7Av72tuRBfQ (Untitled)
  Session 8: BRDQKPJhMQwbLExNC6cMPew3vnZGKnvPN9rVdxVMJv05Ub77NYPhR7cu400lI63ENpbFWpM6uXtsaJB2HOsylw (Untitled)
  Session 9: Mcj0ZDVNpmAiD_2ZVd1SYnOUyXJ8RUETAc5gHzjEQRV-t2Ymi8HhmT-2Dc4vU6hWs2Wrph9U6ZUUbepRBwby4A (Untitled)
  Session 10: SjEdAoC_Va1ggnLgMzZwEjIYtdkaslNGe94NH8jfoEQpkonVZWMoHaZUudJDWMa9cHYOfEbHclul1aZBzsu1wQ (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: RwjDJ6okKb97QLn3oW8GvkAJivhjbymwsnMB6QWeaAFdvQRAGInIZ4R-0lWjLQ-mzoBUYTHUt-S9gkmaCnelUw

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
✅ [32mJavaScript client examples complete![0m
