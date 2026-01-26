make test-examples-javascript
🧪 [36mRunning JavaScript examples (direct HTTP/WebSocket)...[0m

added 1 package, removed 1 package, and audited 9 packages in 824ms

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
  id: '7Tj8ZUwXET-XxpOlhW84wikrjnkYY7MFXXG2l71lKB_Wz_hSF90kg4nXsNzOimKFpkWqRp4RLsGpbI8_uQvYVw'
}

=== Find by ID ===
Found: {
  id: '7Tj8ZUwXET-XxpOlhW84wikrjnkYY7MFXXG2l71lKB_Wz_hSF90kg4nXsNzOimKFpkWqRp4RLsGpbI8_uQvYVw',
  active: true,
  name: 'Test Record',
  value: 42
}

=== Find with Query ===
Found documents: [
  {
    active: { value: true, type: 'Boolean' },
    name: { value: 'Test Record', type: 'String' },
    value: { value: 42, type: 'Integer' },
    id: '7Tj8ZUwXET-XxpOlhW84wikrjnkYY7MFXXG2l71lKB_Wz_hSF90kg4nXsNzOimKFpkWqRp4RLsGpbI8_uQvYVw'
  }
]

=== Update Document ===
Updated: {
  active: { value: true, type: 'Boolean' },
  value: { value: 100, type: 'Integer' },
  name: { type: 'String', value: 'Updated Record' },
  id: '7Tj8ZUwXET-XxpOlhW84wikrjnkYY7MFXXG2l71lKB_Wz_hSF90kg4nXsNzOimKFpkWqRp4RLsGpbI8_uQvYVw'
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
✓ Inserted test record: t7EAwsRDQrq8FGI-fgAyNepa-X0p_4B8elhX6Z2gl1QO-vBDj9UauLfeBvQwdKlqwQ0_etMgoODEf7Zmn6D3jQ

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
        "id": "9qANfwDEYn4jKGx25UL0lEibKplJLTThUsJZjNTuw-Mndg3CY8pV24Wc-19KGmdxxFpyeTosNU8sJe5uomjbCQ",
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
        "id": "2KDKITf1SV3IdCNu3FWwWGoEg8fhkqMMeP7tox31rDf-3OcKECI-4LPKyr_GZjmDnd6O5hUtN5zh6TfLfBvPiQ",
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
        "id": "t7EAwsRDQrq8FGI-fgAyNepa-X0p_4B8elhX6Z2gl1QO-vBDj9UauLfeBvQwdKlqwQ0_etMgoODEf7Zmn6D3jQ",
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
        "id": "knUwKHS37sl1ZADzeM0K1tHhnVSxb_dvX1jxCdqgH1RtlgbiRpM8eMHmFU0S6jDKcKI2sjV1yBdCldhrb4N_Lg",
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

✅ Script saved: hdsJEBJu0K41V91qL5vo1tJH5isfZ9XOYwQCmarO2K5XCGptLTLyhiVmyoaoInb09wwqNKgZLp-tanIjRiKs2g
📊 Found 10 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: 4txqSdQ24XJZEMHoruZ95J74zUUHKMEfYUYyDhsgSjggenJClT1Rev_bDaTrrbGC6NZPHkTE3jfoW-k6ktwP-g
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: 80v58dwc1btOeeRyzfSw2R2auodPHEjGrKw_U6duLRKHTZuc4hPMcbPXd-2-LfAxLqwZos_itvhKYfXYLZ0lzw
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"status":"inactive","count":10,"avg_score":50,"max_score":90}
   {"avg_score":60,"max_score":100,"count":10,"status":"active"}

📝 Example 4: Function Management

📋 Total scripts: 52
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
Collection created with first record: pmAZdUSyvUk5cADFTqVzuK_t6nFlDwz6cKXVcJm0bMmt6OZdoCoXdgnr5evVMw-8I4LoqFnHqE85OKIuS1ptWA

=== List Collections ===
Total collections: 23
Sample collections: [
  'demo_collection',
  'schema_products_client_ts',
  'websocket_test',
  'schema_employees_client_go',
  'batch_users'
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
Created Alice: $1000 - ID: TZEqj88uuefGI1qU26YIwSLvarpTf7TDtd2k_dpzYjTmHIse1SEXZ7ZZv0P_XQVrE24NO75etD0uGTDjxIgk2w
Created Bob: $500 - ID: BX0fv_bJLqx9YueIL9KDkVq9JMmBpQzIdqQd1LK6gw-EM7TU__za5S_TYQBQbtv3VO2REH9qyAE3VAILJNMMoQ

=== Example 1: Begin Transaction ===
Transaction ID: 35b0ca9e-0ebf-4fde-9eaa-7e99b52ef19b

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
New transaction: 8624a5c8-6947-4c54-9e97-49f474eaa4b8
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
   ✅ Script saved: jaiuvjwrLaZPG8EZ_oY251og5pixldtxI8NhElra_Z4YB9jpZwjlepIAoxm_Y_qMW5yipJojlbv_ocRCJdl3ig

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
   ✅ Script saved: UbwNJuVHOU7KQ-6V4kdcob26DzKEi01rXhvEuDLUcQkp3vZHRvqcGhV0O4kfhhePcbKpF8Mf_YZJQWnb7FAIIw

2️⃣ Calling Script (Query + Update + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Status updated to: {"value":"active","type":"String"}
   📋 Name: {"type":"String","value":"Alice Smith"}

============================================================
📝 Script 3: Query + Update Credits + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: bfc9bNkB9-_aFX_6EnHHM5IBQJlIXlUWfx72JDGNWF5ipSPFQhVTPt1dW1oEgAvcbagrDmGdxro96qBWlvSbhA

2️⃣ Calling Script (Query + Update Credits + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {"value":0,"type":"Integer"}
   📋 Status: {"value":"active","type":"String"}
   📋 Name: {"value":"Alice Smith","type":"String"}

============================================================
📝 Script 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: w76MYhraO90JlApuKJ1IxXZ4G80fuJPL79tCr_SsQiCXmbTlRDiEb_9aE_zKAN5ErpscF6kSpZdI0N75qUcuZg

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: jaiuvjwrLaZPG8EZ_oY2...
   ✅ Deleted script: UbwNJuVHOU7KQ-6V4kdc...
   ✅ Deleted script: bfc9bNkB9-_aFX_6EnHH...
   ✅ Deleted script: w76MYhraO90JlApuKJ1I...
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
  Output: Document ID = JXtfgn065MdUQYvVNspC4bPG98dBNwOyJa4916au8hke9tlc1MLXhS8wZv-WFyF_9qbYA24IEOndv7Ya3pdlpg
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: findById(JXtfgn065MdUQYvVNspC4bPG98dBNwOyJa4916au8hke9tlc1MLXhS8wZv-WFyF_9qbYA24IEOndv7Ya3pdlpg)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: findById(JXtfgn065MdUQYvVNspC4bPG98dBNwOyJa4916au8hke9tlc1MLXhS8wZv-WFyF_9qbYA24IEOndv7Ya3pdlpg)
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
  Output: Document ID = nHM8dvgQJGzMMPknAq-TUWJwr4XkfMJxBCzYEPxtr1v7yVqu9OhoQbayRna0mafsRoffmD4aqXyHxMUMfc9fjA
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: findById(nHM8dvgQJGzMMPknAq-TUWJwr4XkfMJxBCzYEPxtr1v7yVqu9OhoQbayRna0mafsRoffmD4aqXyHxMUMfc9fjA)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: findById(nHM8dvgQJGzMMPknAq-TUWJwr4XkfMJxBCzYEPxtr1v7yVqu9OhoQbayRna0mafsRoffmD4aqXyHxMUMfc9fjA)
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

> @ekodb/ekodb-client@0.8.0 prepare
> npm run build


> @ekodb/ekodb-client@0.8.0 build
> tsc


up to date, audited 58 packages in 1s

15 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.8.0 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning JavaScript client library examples...[0m

added 1 package, removed 1 package, and audited 9 packages in 831ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'U_opFcTjJnfJ7ah5Cwj0zRea0f_rkyrQPwQuX1yQ6MjStv4iESh0jCOYvVceZc_GTw9hqjt2GJ5FUBjH8S-fiA'
}

=== Find by ID ===
Found: {
  active: true,
  value: 42,
  name: 'Test Record',
  id: 'U_opFcTjJnfJ7ah5Cwj0zRea0f_rkyrQPwQuX1yQ6MjStv4iESh0jCOYvVceZc_GTw9hqjt2GJ5FUBjH8S-fiA'
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  name: { value: 'Updated Record', type: 'String' },
  active: { type: 'Boolean', value: true },
  value: { value: 100, type: 'Integer' },
  id: 'U_opFcTjJnfJ7ah5Cwj0zRea0f_rkyrQPwQuX1yQ6MjStv4iESh0jCOYvVceZc_GTw9hqjt2GJ5FUBjH8S-fiA'
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: XWGJ1tKU7eI3JmpgDakHzn3xdYGHFQjFIr78PRHYUbaSNmmg9eqXptgp8WQLOCpU-3H6p1Ei2eGf_RJLHpAcYg

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
Collection created with first record: fL9bDp1Lw-gX_pPPWVF-1wYGTPEmI61lKyKe9dKOrIHv9VDledh2cfhT2ccavIG_2JxrWnY6XFGj9K9R338Bgw

=== List Collections ===
Total collections: 22
Sample collections: schema_products_client_ts,websocket_test,schema_employees_client_go,batch_users,schema_users_client_go

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
cache:product:3: { name: 'Product 3', price: 49.989999999999995 }

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 0 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 11

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: false

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: a3uAjVQVR3LxKDzcRvPlLM1JPaDGUG2aNcQ-nsCwMWC-187AK1oM26MRuKcAXNCdiYgbuUCNtGNZrjNuJNbQow
Created Bob: $500 - ID: 42wP9smxUkOXWbppXWxYjtFVopZEyFaJMxzp-nGNVPSwZJEO0Z1Z9BYb6P5U7u9SWq951B_ESmt2BewTto4xkg

=== Example 1: Begin Transaction ===
Transaction ID: be43691c-5437-43e9-a725-bb092780283b

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 53b29442-caa2-48f6-b0f0-5c82e649e005
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
  1. Score: 25.740, Matched: email, email.value, name.value, name
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title, title.value, bio, bio.value
  2. Score: 26.400, Matched: bio.value, title.value, title, bio
  3. Score: 26.400, Matched: bio, title.value, bio.value, title
  4. Score: 26.400, Matched: title.value, title, bio, bio.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio.value, title, title.value, bio
  2. Score: 39.600, Matched: bio, title.value, bio.value, title
  3. Score: 39.600, Matched: title, bio, bio.value, title.value
  4. Score: 39.600, Matched: bio, title, title.value, bio.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio.value, bio

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.766, Matched: 
  2. Score: 0.758, Matched: 
  3. Score: 0.742, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.697, Matched: title.value, title, content.value, content
  2. Score: 1.506, Matched: title.value, content, title, content.value
  3. Score: 0.303, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: skills.value, skills, bio.value, bio

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
✓ Inserted document: qhotDNabiJC3GvXDoPzBgBJ7opZOsUgymYJ37yNqnT9wXj4vxX5jZOo0xP9MR1J-_FflVI6WM5MihXl2LxOHBA

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: Cnhb9EJmB3ERaGRrFbzrw1pL2oP8P68XonkynhjKpp8jHt_XIde0-W-2hWHew46Qdd4vDA2nl9MxUSLqrfZe3A

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
✓ Inserted document with TTL: vGGT4kiWXTwxQLibpNb_4SS7DGdpeiBJ3j3vQfBlfZ9py456xCb7KILbRrGCClo6C8hjtfDMlRqX-SBBcIawww

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
✓ Edge cache script created: tlC0UvVR4W5rJwlBc6fB3NMyeJoTL4oLBmAsYWqMYv35GWBd4roN2MN28GHMMwygoC7Wrf6JvkOWm3eRecELRQ

Call 1: Cache miss (fetches from API)
Response time: 4ms
Result: {
  "records": [],
  "stats": {
    "input_count": 0,
    "output_count": 0,
    "execution_time_ms": 0,
    "stages_executed": 2,
    "stage_stats": []
  }
}

Call 2: Cache hit (served from ekoDB)
Response time: 3ms (1.3x faster!)
Result: {
  "records": [],
  "stats": {
    "input_count": 0,
    "output_count": 0,
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

✅ Script saved: DImEMoisJLrTsTJEiY5M9W8ES0gs9svvMNUcHWtRfy7TjwwimQYatQ8gFIkARNDDETmDdlpoZIQbqlo3oHnAMA
📊 Found 10 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved
📊 Found 10 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved
📊 Statistics: 2 groups
   {"avg_score":50,"status":"inactive","count":5}
   {"status":"active","count":5,"avg_score":60}
⏱️  Execution time: 0ms

📝 Example 4: Script Management

📋 Total scripts: 55
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

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 4ms
   📊 Records: 0

Second call (cache hit - from cache):
   ⏱️  Duration: 4ms
   📊 Records: 0
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

✅ Script saved: MISE30nOEz5r3DUIYe5OOZWVuBunCpGhVJD-p6GFaLixIH7HNpcYYKRT_EbomIzCyFF4zzhD1KhgalxjfuN9eg
📊 Found 2 product groups
   {"avg_price":575.6666666666666,"count":3,"category":"Electronics"}
   {"category":"Furniture","count":2,"avg_price":474}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved
📊 Found 2 categories
   {"count":3,"category":"Electronics"}
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

✅ Inserted order: KvWmSkscMS9uHPst3bkx3Guvd_gB25SMUjadqJSevZj53EG3tlHbhPY3IBPLgk6Cba0utrcV-s05Rpebqaz1KQ
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: i-QVrmyiH4RUHomY6p5-r5f0-jAXutLIrSHJZXylxNV9IRdS4Tx1ngXp84kI9WirwXCyL8pp-8kV_3PqIwtpEw
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"role":"admin","userId":"user_abc"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: 9bffMIzgCxxyQfa7rwte7PBuzdgEjMJM910IU9eduP8SaaQA1F3tZ-D4msFdSFtO2qb5x2XIkeaba68DxrhQyQ
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Script

✅ Script saved: yOc6ck--ooWLrWFEAI_GHiKQe4qTllM37GmH1hr82nJxElETZG7_I4Sv8hdCAwBnm0_Df2aJpE8WqTwFRCpzqA
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
   admin: 20.0 (3 users)
   user: 70.0 (7 users)
⏱️  Execution time: 0ms

📝 Example 4: Top Users by Score

✅ Script saved
📊 Users (showing first 5 of 5):
   1. User 5 - Score: 100
   2. User 4 - Score: 80
   3. User 1 - Score: 20
   4. User 2 - Score: 40
   5. User 3 - Score: 60
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
   1. Introduction to Machine Learning (AI)
   2. Database Design Principles (Database)
   3. Vector Databases Explained (Database)
   4. Getting Started with ekoDB (Database)
   5. Natural Language Processing (AI)
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
   1. Introduction to Machine Learning
   2. Database Design Principles
   3. Vector Databases Explained
   4. Getting Started with ekoDB
   5. Natural Language Processing
⏱️  Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Script saved
📊 Projected documents (showing first 3):
   1. Introduction to Machine Learning
   2. Database Design Principles
   3. Vector Databases Explained
⏱️  Execution time: 0ms

📝 Example 5: All Document Fields

✅ Script saved
📊 All documents (5 total, showing first 2):
   1. Introduction to Machine Learning (AI)
   2. Database Design Principles (Database)
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
   1. Office Lamp - $79 (⭐4.05)
   2. USB-C Cable - $19 (⭐4.3)
   3. Monitor 27" - $399 (⭐4.6)
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
   Vector databases provide several benefits:

1. Precision: They can represent data more accurately, especially for complex shapes and structures.

2. Scalability: Vector data can be easily scaled up or down without losing quality.

3. Less Storage: They typically consume less storage space compared to raster databases.

4. Detailed Analysis: Vector databases support more detailed and complex spatial analysis.

5. Enhanced Editing: They allow for more precise editing of data points, lines, and polygons.
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
✓ Created SWR script: fetch_api_user_js (d2_m_YPqLyDctqskdulmQUQRKjTU2CO4ZW86zoLMBHddvctkRhGNi80F1amzmw70o679iUEumFPLJ4bnYEbcQA)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [],
  "stats": {
    "input_count": 0,
    "output_count": 0,
    "execution_time_ms": 0,
    "stages_executed": 2,
    "stage_stats": []
  }
}
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 5ms (served from cache)
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
✓ Created session: 21FnATaQk-vzSs0mr8wuh0wKKbj0pFI3ix_38Hf5ngkiF5b10a_WpmS925O-W2oWpzLZhcxwPUTiRiH7wLLb2w

=== Sending Chat Message ===
Message ID: e43xcU5gnrXi7WxRXOUBUXvhAEvGurnum7M-Po2PJSzjnzqPpw5R2A_txKqIbEKrcoY2WT4Zh9N4R8HLFfdlcw

=== AI Response ===
There are three products available:

1. ekoDB: This is a high-performance database product with AI capabilities. The price of ekoDB is $99.

2. ekoDB Pro: This is an enterprise edition product with advanced features. The price of ekoDB Pro is $299.

3. ekoDB Cloud: This is a fully managed cloud database service product. The price of ekoDB Cloud is $499.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_js',
  record: {
    description: 'A high-performance database product with AI capabilities',
    price: 99,
    id: '4DzbwapHzLT8PtI30e2ta9H6sQGagBLX6rIbx-qeozZlOyd3xVSH0svra8gi_ewRunWu4nsggoWo5i6hHb-xtw',
    name: 'ekoDB'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_js',
  record: {
    price: 299,
    id: '5plKiTw0ZdstAvL3QDRahtqz53j1HkJe96mryt6k1qQYveaJnK39eFuBd51QS-95qvWHIAikJJhyzQQPApXYsQ',
    name: 'ekoDB Pro',
    description: 'Enterprise edition product with advanced features'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_js',
  record: {
    price: 499,
    description: 'Fully managed cloud database service product',
    id: 'F9JdL_Xur_ar4Vf7sUoa2wTBjVa2JKDnHkaVIZ-mQfwChUX5hnJs5ZpWuRQwElu4t9qGttq5Gj9YxXTx6DLVSA',
    name: 'ekoDB Cloud'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 3970ms

=== Token Usage ===
Prompt tokens: 452
Completion tokens: 89
Total tokens: 541

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: hvygl8-a_1RQpU7rEhEjyJNvln-FyQ-93Izm0I4sIEj6Qxfn2ZOLNnopKue0tq9GscItAEg4TFg2QOteSfB2jg

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the context provided, the available product is "ekoDB". It is a high-performance database product and is priced at $99.

✓ Second message sent

=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of the ekoDB is $99.

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: NYzgdKwq3Z_ct1tThOTjt3K_qHebydrYpQL3BdFWxaWO4ObcztKUuCdKkGP2a8XtJyK0LBzKBU81INEZM8p3-w
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
✓ Created session: OqFR5VKMuL9WW1VUoUGB5BclO5_vk5FsDqxM7WgcJtYDazoCVBBOrUMDfKKso1kvHuP9UkioZ0ldIrhFgAC87w

=== Sending Messages ===
✓ Message 1 sent
  Response: The product available is called "ekoDB". It is a high-performance database product. The price of this product is $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 1cp4NlToMIINlSu0rqSY7NNAmCtBf8kqewOdh7JBSNKCuzIUv5NDC1RzIOVsMZfO1Aybo_2tGbLdEGcgZbIrqg
  Parent: OqFR5VKMuL9WW1VUoUGB5BclO5_vk5FsDqxM7WgcJtYDazoCVBBOrUMDfKKso1kvHuP9UkioZ0ldIrhFgAC87w

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: 1cp4NlToMIINlSu0rqSY7NNAmCtBf8kqewOdh7JBSNKCuzIUv5NDC1RzIOVsMZfO1Aybo_2tGbLdEGcgZbIrqg (Untitled)
  Session 2: OqFR5VKMuL9WW1VUoUGB5BclO5_vk5FsDqxM7WgcJtYDazoCVBBOrUMDfKKso1kvHuP9UkioZ0ldIrhFgAC87w (Untitled)
  Session 3: NYzgdKwq3Z_ct1tThOTjt3K_qHebydrYpQL3BdFWxaWO4ObcztKUuCdKkGP2a8XtJyK0LBzKBU81INEZM8p3-w (Untitled)
  Session 4: 21FnATaQk-vzSs0mr8wuh0wKKbj0pFI3ix_38Hf5ngkiF5b10a_WpmS925O-W2oWpzLZhcxwPUTiRiH7wLLb2w (Untitled)
  Session 5: VkWkyyKMKbPtF1BB-Vt5w0NGcX1A-DR1L5gdt6j4R3eu5BWYHJ5jDBRi6XMcBjEACiT_ebrrjsB_2dOhxKYQQA (Untitled)
  Session 6: _SEGQUojBCVOpvf9Cq3wk5RyPyP4KUcV74lRMF3DDss6_4WUBBbFzXy0cn3-YbN3Pv-aVuck0zt2K9hb4BBCww (Untitled)
  Session 7: BJL5BGveOvrlTBWa7LTSiHSpT8iOJWuHoDQjzhX3oa_NErMdUJcxXMoJrTZ6VG8DTXGK2m5VAMTF_FZ_Jmzeug (Untitled)
  Session 8: 7qELxPg2lrcRGy2spbCbBuOsp81Od503t0WkQ9fDmaf6UvOG_CMVqnir7wTo_-hhK4ukmBoUp94dva071u49FQ (Untitled)
  Session 9: 1S0ufLnEeEi2Yjuhd_ouj8oaoVh694rRPUWARagAFJABjlnRGDZR32ZnP9mdoPKNgiP8Eg-tgmRmUwEGcdZ6gw (Untitled)
  Session 10: ri8J9aosDC9gMwutTRft89s9bxluPtJ7ZTaTMX8IsGsL0M2ATyINvc74OjgMfIqVXTnnKGSKkbdYewbyGG1Nqg (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: 1cp4NlToMIINlSu0rqSY7NNAmCtBf8kqewOdh7JBSNKCuzIUv5NDC1RzIOVsMZfO1Aybo_2tGbLdEGcgZbIrqg

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
✅ [32mJavaScript client examples complete![0m
