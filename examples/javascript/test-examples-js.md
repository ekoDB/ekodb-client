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
  id: 'SrDgeIUvHsC7ZB1WEo3iWCpKeOPPj-OM9eVS-EQAKtgksNlvfJFfcU_m4DtbfUWF4JbQ2K8w6nuLUCE0G6Vvzw'
}

=== Find by ID ===
Found: {
  value: { value: 42, type: 'Integer' },
  id: 'SrDgeIUvHsC7ZB1WEo3iWCpKeOPPj-OM9eVS-EQAKtgksNlvfJFfcU_m4DtbfUWF4JbQ2K8w6nuLUCE0G6Vvzw',
  active: { value: true, type: 'Boolean' },
  name: { type: 'String', value: 'Test Record' }
}

=== Find with Query ===
Found documents: [
  {
    name: { value: 'Test Record', type: 'String' },
    id: 'SrDgeIUvHsC7ZB1WEo3iWCpKeOPPj-OM9eVS-EQAKtgksNlvfJFfcU_m4DtbfUWF4JbQ2K8w6nuLUCE0G6Vvzw',
    active: { type: 'Boolean', value: true },
    value: { type: 'Integer', value: 42 }
  }
]

=== Update Document ===
Updated: {
  id: 'SrDgeIUvHsC7ZB1WEo3iWCpKeOPPj-OM9eVS-EQAKtgksNlvfJFfcU_m4DtbfUWF4JbQ2K8w6nuLUCE0G6Vvzw',
  name: { value: 'Updated Record', type: 'String' },
  active: { value: true, type: 'Boolean' },
  value: { type: 'Integer', value: 100 }
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
✓ Inserted test record: d7ufeJ19LQodv5Nyn9hccKvdpqIxdwolwbndN7ZZ6jAmXVR4b2xbT2Fzy8N0QSu19Ity0PJMVaGeDdaRHTTG5w

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "messageId": "1774661545918",
  "payload": {
    "data": [
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "Zxg4rrsaToHy1gqhQ372119VvXB9cXvTvADsBiN3M1fCo3cVAolvtvHgzpJtCxvSXk4cw8sWZqQoSLdyK83fUg",
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
        "id": "d7ufeJ19LQodv5Nyn9hccKvdpqIxdwolwbndN7ZZ6jAmXVR4b2xbT2Fzy8N0QSu19Ity0PJMVaGeDdaRHTTG5w",
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
        "id": "YUh2b8XZyTHuIgTZtA5_G9DtDh9EbjwWuWysboVj8-xSYZng0mEWCxRyJxIytK8MhJthhGQFfkWxmzXAU-lINw",
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
        "id": "aKdQ63M13kgIZbKZYZYTPFYm5-7xdsO5vUT2KRNbXKxwYoqBTDCGw4lYYqVpoGBpk8sWjYSKmgNryV2xruD46Q",
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

✅ Script saved: b8TncvnX0QgAwhVU6Cx8hbN_I8xFxn5lXC-W9ePNjujH3666jfY0cb9nl1LwhY4e7-EQdRcvlIXU0XrdywRV9Q
📊 Found 10 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: vDU0cS8PgDpyoVuDBjsX_Fj6GUHUQEg3qmOMh3KGtVfhkNlP7-0tqUipDYGsIb1x0shAb1Z6ytOs1y72CnUhFA
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: 1eyS1nAaXv5dCKWMAiBwXBrSpXCHkJrKOqjPHR2yK6ox13sLgAn7ogx4v7EI8uFcktsJCbmnYf9DujPocu7c_w
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"avg_score":60,"max_score":100,"count":10,"status":"active"}
   {"avg_score":50,"status":"inactive","count":10,"max_score":90}

📝 Example 4: Function Management

📋 Total scripts: 33
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
Retrieved value: { userId: 123, username: 'john_doe' }

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
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/collection_management.js ===[0m
=== Collection Management (Direct HTTP) ===

✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: GOJJoQhx5ZTX7s5cbAW8loUA-EdnT2PHbEXmB1DZCQulQe2nt24CL_kGXi5ytYBTyBsrwWRwHHSRUSRSH6Qixg

=== List Collections ===
Total collections: 25
Sample collections: [
  'schema_documents_client_ts',
  'batch_users',
  'schema_users_client_py',
  'schema_employees_client_go',
  'chat_configurations__ek0_testing'
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
Created Alice: $1000 - ID: PDleptXE9cHoFwvuaMyh3b9GM2XxU9BZ8QkIuDxiqih8tyjVI3KesgjSs1m18IeMB7b3hwtmNaNkOFxsYe8fMg
Created Bob: $500 - ID: Sf3CxSBzohbNkmKTHTLumj1WiuQ-8NbwmMplfnYxdOc5QcbDhUZXKG5ezlImr-5Nmmmv_5eXd0DRThKB905aGg

=== Example 1: Begin Transaction ===
Transaction ID: 09829619-344e-4fad-a572-2adda6346c80

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
New transaction: 54264982-791a-4d08-8979-224aa2e06e0e
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: {"type":"Integer","value":700}

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
   ✅ Script saved: UF2z93IHj08nwPfd5M5HfK59Mnl8rBquHgAdu_5k8LB-JCuIjp39YLLY4LUXfueAiIuvPqYWY4tai-DjzWH4KQ

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Name: [object Object]
   📋 Email: {"value":"alice@example.com","type":"String"}
   📋 Status: [object Object]
   📋 Credits: [object Object]

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: ZbhZfd5MemOwbHA495Krrcqd4o7538MwWLyKLkuPBxUzC7Ew_AlZalzc1hCcghiFVPIFjba6y77uPTYH135i3w

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
   ✅ Script saved: DUem-8eJPkJXaoCqXEMfIbZ6yrWJ1nrS5Xb0L2ff2ki_4PuNEA46wV99cwhPay6dKebRmoqU6kwsuikea-lQcQ

2️⃣ Calling Script (Query + Update Credits + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {"type":"Integer","value":0}
   📋 Status: {"type":"String","value":"active"}
   📋 Name: {"value":"Alice Smith","type":"String"}

============================================================
📝 Script 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: cIHGNIAwDL7hqJuCy5fxoz1qj58sTcDx_jq7kpmDzf3A_1hc6Im9GUZrwEjyYWArWEqhw7ipsf76Qr5Xq5I27Q

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: UF2z93IHj08nwPfd5M5H...
   ✅ Deleted script: ZbhZfd5MemOwbHA495Kr...
   ✅ Deleted script: DUem-8eJPkJXaoCqXEMf...
   ✅ Deleted script: cIHGNIAwDL7hqJuCy5fx...
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
  Output: Document ID = Q7KUkX1y_p7ggZXEKRSOinQoplZWhu9KEMb9kOJ1TxqWyMLkOZeTlVgKUt-qNyVLkHHRZ5yNGF8wAR2optMMEQ
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: findById(Q7KUkX1y_p7ggZXEKRSOinQoplZWhu9KEMb9kOJ1TxqWyMLkOZeTlVgKUt-qNyVLkHHRZ5yNGF8wAR2optMMEQ)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: findById(Q7KUkX1y_p7ggZXEKRSOinQoplZWhu9KEMb9kOJ1TxqWyMLkOZeTlVgKUt-qNyVLkHHRZ5yNGF8wAR2optMMEQ)
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
  Output: Document ID = 1CgbTegZ9nUSG3O97HDvhZqfBl8mndjjnhoHtgaOZJQpDNZctm4FX030MY88bLbGMZKuQELzGx-07iQz-HO8JA
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: findById(1CgbTegZ9nUSG3O97HDvhZqfBl8mndjjnhoHtgaOZJQpDNZctm4FX030MY88bLbGMZKuQELzGx-07iQz-HO8JA)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: findById(1CgbTegZ9nUSG3O97HDvhZqfBl8mndjjnhoHtgaOZJQpDNZctm4FX030MY88bLbGMZKuQELzGx-07iQz-HO8JA)
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

added 1 package, removed 1 package, and audited 9 packages in 1s

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'XYQe6gh8BG7wmA6BZDGKEnwTcszhNaeAgQ9-MWYlWTZWliWukreGKMIJa0ngqcL4_eZWXvHrMb2vv1VoL6_Orw'
}

=== Find by ID ===
Found: {
  name: { value: 'Test Record', type: 'String' },
  id: 'XYQe6gh8BG7wmA6BZDGKEnwTcszhNaeAgQ9-MWYlWTZWliWukreGKMIJa0ngqcL4_eZWXvHrMb2vv1VoL6_Orw',
  value: { value: 42, type: 'Integer' },
  active: { type: 'Boolean', value: true }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  value: { type: 'Integer', value: 100 },
  active: { value: true, type: 'Boolean' },
  name: { value: 'Updated Record', type: 'String' },
  id: 'XYQe6gh8BG7wmA6BZDGKEnwTcszhNaeAgQ9-MWYlWTZWliWukreGKMIJa0ngqcL4_eZWXvHrMb2vv1VoL6_Orw'
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: 4vOWNSz-i4Ggb2dr8JsnRTXafXYUrDC5hyg-3076AOx-2XQmNgN1jHXb1thwoWd6zmfmULhj4gkmPdRFWGuXPw

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
Collection created with first record: 4LNowRkwZVSUZfGEQp2wRzRp7qWOpBHtAkCP1l4N_jcBvrErJ5Fz5zpamSMnQBRF0bBWzO75Ftt72lK3gWTfSQ

=== List Collections ===
Total collections: 24
Sample collections: client_collection_management_js,schema_documents_client_ts,batch_users,schema_users_client_py,schema_employees_client_go

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
Total keys in store: 17

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: false

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: HGvrJWr-tnJKJEvq8Fa3SWJAgFzHdpHRKJ4dXhwiwCDPaMGYS1hJ_Bozk5kivzEldEM6V3mDOC1xcCpfrWsDBg
Created Bob: $500 - ID: LhAssq_MrAdZJWtgjv8KzdcM9HUoK72SWmyZyyNhUmvWKFmlTMlSuEle3LLdTTFwkw170_kkISgZg1vhpdAs_w

=== Example 1: Begin Transaction ===
Transaction ID: c9235bd8-ec57-41f9-b585-7f43c86bc6d1

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: f792ea4d-cad9-4221-92b7-00727cd3e17c
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
  1. Score: 25.740, Matched: name.value, email.value, email, name
  2. Score: 12.540, Matched: name.value, name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: bio, title, title.value, bio.value
  2. Score: 26.400, Matched: bio, title.value, title, bio.value
  3. Score: 26.400, Matched: bio.value, title, title.value, bio
  4. Score: 26.400, Matched: title, bio.value, title.value, bio

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title.value, bio.value, bio, title
  2. Score: 39.600, Matched: title, title.value, bio.value, bio
  3. Score: 39.600, Matched: bio, title.value, title, bio.value
  4. Score: 39.600, Matched: bio.value, title.value, bio, title

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio.value, bio

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.777, Matched: 
  2. Score: 0.757, Matched: 
  3. Score: 0.746, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.703, Matched: content.value, title, title.value, content
  2. Score: 1.499, Matched: title.value, content, content.value, title
  3. Score: 0.311, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: bio, skills.value, bio.value, skills

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
✓ Inserted document: 5e7MMAb6WS_iEf8q3yaLTOjGKNsDO-5swnq_zLTzffYWpSHurs8VoyrNDdmABplx7qUFZGIpZlTr7s7zacQ-qg

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: 7UM0kYDUDwnkFoZqEebV-g1dtuxbLP4Uquo6kLjBqYHUGtBEg1tiHEpxNRsn2xb9A2NCmjlCXmZNAeatgm2_sA

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
✓ Inserted document with TTL: XpvuYAfH6VQaY0c4Op3rI1Kf-xsKtuP2C3r5tiBrHUpJycWKn1vYd0V91aTHWNaqghVFX9Nam0KKtfbBKVPmWQ

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
✓ Edge cache script created: j5IyKKtTlV7J_dD9ocGvW79-3apjLaVI0XAitDT4vMf0JV5IkDHaQ2DHg7EjzUcUc7NxK_3F9F1ydB3PtEocqA

Call 1: Cache miss (fetches from API)
Response time: 43ms
Result: {
  "records": [
    {
      "value": {
        "latitude": 40.710335,
        "current": {
          "temperature_2m": 5.9,
          "interval": 900,
          "time": "2026-03-28T01:15"
        },
        "generationtime_ms": 0.027298927307128903,
        "elevation": 32,
        "longitude": -73.99308,
        "timezone": "GMT",
        "current_units": {
          "temperature_2m": "°C",
          "time": "iso8601",
          "interval": "seconds"
        },
        "utc_offset_seconds": 0,
        "timezone_abbreviation": "GMT"
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
Response time: 43ms (1x faster!)
Result: {
  "records": [
    {
      "value": {
        "latitude": 40.710335,
        "current": {
          "temperature_2m": 5.9,
          "interval": 900,
          "time": "2026-03-28T01:15"
        },
        "generationtime_ms": 0.027298927307128903,
        "elevation": 32,
        "longitude": -73.99308,
        "timezone": "GMT",
        "current_units": {
          "temperature_2m": "°C",
          "time": "iso8601",
          "interval": "seconds"
        },
        "utc_offset_seconds": 0,
        "timezone_abbreviation": "GMT"
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

🚀 ekoDB Scripts Example (JavaScript Client)

✅ Client initialized (token exchange automatic)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: QSwSUfQR7NK4s68e5rY_cokv6y6gOac7QoxUCESItfZ1lml3No6cNsduWCAjKF_d9j8VBAt4nVUxT_vh7J7Z1A
📊 Found 10 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved
📊 Found 10 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved
📊 Statistics: 2 groups
   {"status":"active","avg_score":60,"count":5}
   {"avg_score":50,"status":"inactive","count":5}
⏱️  Execution time: 0ms

📝 Example 4: Script Management

📋 Total scripts: 36
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
   Name: {"type":"String","value":"User 1"}
   Department: {"value":"engineering","type":"String"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 42ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "phone": "1-770-736-8031 x56442",
    "name": "Leanne Graham",
    "username": "Bret",
    "email": "Sincere@april.biz",
    "website": "hildegard.org",
    "company": {
      "bs":...

Second call (cache hit - from cache):
   ⏱️  Duration: 41ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "phone": "1-770-736-8031 x56442",
    "name": "Leanne Graham",
    "username": "Bret",
    "email": "Sincere@april.biz",
    "website": "hildegard.org",
    "company": {
      "bs":...
   🚀 Cache speedup: 1.0x faster!

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
🚀 ekoDB JavaScript Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved: 5QI2YDxCwSTMeaVMO4KyR7dX2FM7tONgYyWjcODDMee72UW8egr5o-g-YCQllLGirzKo2V7hzkraytyoz3WFAA
📊 Found 2 product groups
   {"count":3,"category":"Electronics","avg_price":575.6666666666666}
   {"avg_price":474,"count":2,"category":"Furniture"}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved
📊 Found 2 categories
   {"category":"Furniture","count":2}
   {"count":3,"category":"Electronics"}
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

✅ Inserted order: C2NVUvi9dkHNV-xYGCbJ4eEhBIiBiIYc5x0ya6JD5pJ0ciR8VoAJehNI7gCc9Qpv4otOQo4DtjAI8F3i54FaWw
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: 0_hEIu17WkTkg8QAjBXPHveAcZUhwfrdcZIgXnBwmoZxGzjbO7dnyNGoTnqn7kJBt2NWN6a3SHDuHQSp8cUqMw
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"role":"admin","userId":"user_abc"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: kKVpkAskCYNvi5IwJyA6BcQKBiy3s3KiaXB4tuGzD_tofF38KGW7JOrgBhZh2w917UPCpkd8CV5LZbRyJS44mw
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Script

✅ Script saved: SSnx6Q-wbDXl5YbExpX14b6jyRiiAMS-kxknk9eGRQESGstegpwt9NwVYH2vcKV0gu-H6aZNph6uvd4GZDI0gw
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
   inactive: 3 users
   active: 7 users
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
   1. User 10 - Score: 100
   2. User 9 - Score: 90
   3. User 1 - Score: 10
   4. User 4 - Score: 40
   5. User 2 - Score: 20
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
   1. Natural Language Processing (AI)
   2. Vector Databases Explained (Database)
   3. Getting Started with ekoDB (Database)
   4. Database Design Principles (Database)
   5. Introduction to Machine Learning (AI)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Script saved
📊 Documents by category:
   AI: 2 documents
   Database: 3 documents
⏱️  Execution time: 0ms

📝 Example 3: Select Specific Fields

✅ Script saved
📊 Document titles (5 docs):
   1. Natural Language Processing
   2. Vector Databases Explained
   3. Getting Started with ekoDB
   4. Database Design Principles
   5. Introduction to Machine Learning
⏱️  Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Script saved
📊 Projected documents (showing first 3):
   1. Natural Language Processing
   2. Vector Databases Explained
   3. Getting Started with ekoDB
⏱️  Execution time: 0ms

📝 Example 5: All Document Fields

✅ Script saved
📊 All documents (5 total, showing first 2):
   1. Natural Language Processing (AI)
   2. Vector Databases Explained (Database)
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
   2. Office Lamp - $79 (⭐4.05)
   3. USB-C Cable - $19 (⭐4.3)
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
   1. Precision: Vector data are represented with high precision and accuracy, allowing detailed illustrations of features.

2. Scalability: You can scale vector data without losing quality, making it ideal for various applications such as cartography and 3D modeling.

3. Compact Storage: Despite their complexity, vector databases are typically smaller in size compared to raster data for large datasets.

4. Flexibility: Vector data can be easily edited and manipulated. Multiple attributes can be associated with each vector feature.

5. Analytical Operations: Vector databases are ideal for performing various GIS operations such as overlay, buffering, and network analysis. 

6. Better Representation: Vector data is better at representing discrete data i.e. separate objects like buildings or roads.
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embedding script saved
📊 Generated embeddings for 3 articles
   1. "Getting Started with ekoDB" - 1536D vector
   2. "Advanced Query Patterns" - 1536D vector
   3. "Draft Article" - 1536D vector
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
✓ Created SWR script: fetch_api_user_js (ykWAKG02gjhuwLDsIv68e7rx-j-QuqxhrxB-7DmsYVKbB_BQPQJN3QlV9T7lEFk9fvU2DEA_0vLiblAJd0es3Q)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "email": "Sincere@april.biz",
        "phone": "1-770-736-8031 x56442",
        "company": {
          "catchPhrase": "Multi-layered client-server neural-net",
          "bs": "harness real-time e-markets",
          "name": "Romaguera-Crona"
        },
        "id": 1,
        "username": "Bret",
        "name": "Leanne Graham",
        "address": {
          "zipcode": "92998-3874",
          "suite": "Apt. 556",
          "geo": {
            "lat": "-37.3159",
            "lng": "81.1496"
          },
          "city": "Gwenborough",
          "street": "Kulas Light"
        },
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
✓ Created native SWR script: github_user_native (LpTzk7-_HAQvKDKHJLutsmsv6rxndKmWPdTmNlWbhHIBW2kK872dqOgYpoX1ECowH8AMKsedRNiL-jQlaGi6gw)

First call (cache miss - will fetch from GitHub API):
  Response time: 19ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 20ms
  Speedup: 0.9x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (OR2RfzqnUxHCDcrNRPBdo43RSlnyIRL_58lwO12By2fyfDqcN3VEKKweech_p9SN96GX8kleD4Bh5zYARXePeQ)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (MjWRzXzu2bwFMoh7DQoIvkaj7-UaaePnb9cWf5U4fk8Wcf-ItbJ-WID_ms6g733A060V09OJ67tnWC-xGAkU5g)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (R_LdTskafVfrUCbf1pFu4kEBJ7LDc9gM91UXuXaIwFOfirmqMaeNa--pdRgRMg0RaX8r-ou0zSEeM-IKl1cEOw)
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
✓ Deleted 4 test scripts

✅ All examples completed!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: ZOtYFsOdOJVhUHJ0sAYPn4HLS7tOzYuZcqB9EsGfcXDrtVD-xVLsx-5LlTm0NjdV-2R4qiUcMHQu5KYorFd-uQ

=== Sending Chat Message ===
Message ID: euujhQ9_O3VFmAlE0WzpaoouSfGuFI6T6oT5XAwVcGLLyBNr8BdvFPRTtKb1Ro_zaV-DjLPnpoOoc22bvaOUyA

=== AI Response ===
The available products and their prices are as follows:

1. **ekoDB Cloud**
   - Description: Fully managed cloud database service product
   - Price: $499

2. **ekoDB Pro**
   - Description: Enterprise edition product with advanced features
   - Price: $299

3. **ekoDB**
   - Description: A high-performance database product with AI capabilities
   - Price: $99

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_js',
  record: {
    description: 'Fully managed cloud database service product',
    name: 'ekoDB Cloud',
    price: 499,
    id: '8dFTpNlwlK1LTy-wpWO9rxd6voPtL4f5yQi-gXkMRt_Mq3mPJy9pR_7ksdmt6V2uHC3ktme53qkcSX3Anpwvyw'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_js',
  record: {
    description: 'Enterprise edition product with advanced features',
    name: 'ekoDB Pro',
    price: 299,
    id: 'SUoBlLC8Bgp_uUmGMafQRHaa7Z-cE4Q1dxlltJNItV3_Mw5mKlBHDaN11nHYg8RWZPua3l44vrWo4PTDpQuRPA'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_js',
  record: {
    id: 'saHXKM_07nnQizYCZtoC2KcEqtPdC5LBj1u6prtfQ7aOceR67s95dR6lxA0pMk5Wo9soZ9RekZOe06uGVjxgXw',
    name: 'ekoDB',
    description: 'A high-performance database product with AI capabilities',
    price: 99
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 2592ms

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
✓ Created session: rYlthrv8Jjm-XXqpBXJEI-467p8c3rEZJ5YrAK4AjGO1IFMU4rXJXuU8dCs_GbxAeh9QldF2nBBCUCTqV6IpKg

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name:** ekoDB
- **Price:** $99

If you need more details or additional products, let me know!

✓ Second message sent

=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of ekoDB is $99. If you have any more questions or need further assistance, feel free to ask!

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: 1X4bkqTYi3ID_CP2AZ175eUNls2C_oM7oM2oSHWtU07MqwHj8YQIj5KpXIj3IEQ6W8_6qg30XAMtVTjc6nDrqg
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
✓ Created session: e1WACwP-kN0ENlihlVCmxs7yJCwGQdQW_opdgvtjdPdvaF_D0TkZbt2BREA2AG3PeWIYumUTuc0kTjyS7ZvrpQ

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: A high-performance database product
- **Price**: $99

If you'd like more information or additional products, please let me know!

✓ Message 2 sent
  Response: The price of ekoDB is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: L2VXL8c_oVfidSWsNOvYp4N1Qw5gFWuOWx8DssVt-flkgUQCrJ0fXkJ340W5Kgk3hn6Q0uIYGPiref9WmMOyLQ
  Parent: e1WACwP-kN0ENlihlVCmxs7yJCwGQdQW_opdgvtjdPdvaF_D0TkZbt2BREA2AG3PeWIYumUTuc0kTjyS7ZvrpQ

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: L2VXL8c_oVfidSWsNOvYp4N1Qw5gFWuOWx8DssVt-flkgUQCrJ0fXkJ340W5Kgk3hn6Q0uIYGPiref9WmMOyLQ (Untitled)
  Session 2: e1WACwP-kN0ENlihlVCmxs7yJCwGQdQW_opdgvtjdPdvaF_D0TkZbt2BREA2AG3PeWIYumUTuc0kTjyS7ZvrpQ (Untitled)
  Session 3: 1X4bkqTYi3ID_CP2AZ175eUNls2C_oM7oM2oSHWtU07MqwHj8YQIj5KpXIj3IEQ6W8_6qg30XAMtVTjc6nDrqg (Untitled)
  Session 4: ZOtYFsOdOJVhUHJ0sAYPn4HLS7tOzYuZcqB9EsGfcXDrtVD-xVLsx-5LlTm0NjdV-2R4qiUcMHQu5KYorFd-uQ (Untitled)
  Session 5: kdddoXIMCkiBsK2xeeqvR-L2_6yGxywJgrIhocfrSafmuo-9E4CR9DIqvygGikcdrpNibVF1oormiJV-XKrW6A (Untitled)
  Session 6: 9XYLyXR-WKO32G9ffTAYqFTwdwu3A4rGcqPeOsNP-JY7PbgxsGCsVfEjgz8kKH0buHQ7rEeibA7nvwrRh6F_JQ (Untitled)
  Session 7: p7d972nn8DG9xmqauZZiInZH0v8-g2KyiPCbGUFGKgP_T9sVUS6PbLC2cZqMZYps4lC9_vYisidfGcJhKpFDsQ (Untitled)
  Session 8: c1_RO_QP34cKI_TlyiZs58NH0eKgbglVF5pUZ5nI0bfURzFItyZnLRtst3tIRq5_euS3CTBYC0nRSvsmfzB2XQ (Untitled)
  Session 9: xfbU1WY2wg4r-wHRMecRShWwDf1QoLaSA5MrTzxpXmuo7MXJPuNZUAW9GTaVZKjujf9TfBP1SV3OmmmVpEiBOA (Untitled)
  Session 10: foY-Abf0nZXi6ccJ2DkWldsigP3w65sA0QbRi39VLaZlKYP5tl6YJ8nTj8t9cu0sdxYpnzliRvrnvHbMK4_cnA (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: L2VXL8c_oVfidSWsNOvYp4N1Qw5gFWuOWx8DssVt-flkgUQCrJ0fXkJ340W5Kgk3hn6Q0uIYGPiref9WmMOyLQ

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
✅ [32mJavaScript client examples complete![0m
