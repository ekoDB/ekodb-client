make test-examples-javascript
🧪 [36mRunning JavaScript examples (direct HTTP/WebSocket)...[0m

added 1 package, removed 1 package, and audited 9 packages in 638ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
[34m
╔════════════════════════════════════════╗[0m
[34m║  ekoDB JavaScript Examples Test Suite ║[0m
[34m╚════════════════════════════════════════╝[0m
[34m
=== Checking Server Connection ===[0m
(node:3711) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
[32m✓ Server is ready[0m
[34m
=== Getting Authentication Token ===[0m
[32m✓ Authentication successful[0m
[33m
=== Running 10 Examples ===[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/simple_crud.js ===[0m
✓ Authentication successful

=== Insert Document ===
(node:3714) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
Inserted: {
  id: 'Ymbe2onJ7B0eI4-yfIk1UtJ5mzFW52fQRRwKRzXXXZfEME1Y4awpaIahI5AICAvDrwjj_DwX4bDVRiH7YjeVPA'
}

=== Find by ID ===
Found: {
  id: 'Ymbe2onJ7B0eI4-yfIk1UtJ5mzFW52fQRRwKRzXXXZfEME1Y4awpaIahI5AICAvDrwjj_DwX4bDVRiH7YjeVPA',
  name: { type: 'String', value: 'Test Record' },
  active: { type: 'Boolean', value: true },
  value: { type: 'Integer', value: 42 }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  name: { value: 'Updated Record', type: 'String' },
  id: 'Ymbe2onJ7B0eI4-yfIk1UtJ5mzFW52fQRRwKRzXXXZfEME1Y4awpaIahI5AICAvDrwjj_DwX4bDVRiH7YjeVPA',
  value: { type: 'Integer', value: 100 },
  active: { type: 'Boolean', value: true }
}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
[32m✓ simple_crud.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/simple_websocket.js ===[0m
✓ Authentication successful

=== Inserting Test Data ===
(node:3742) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
✓ Inserted test record: 6OxKBx0mTRZ2QZpJVbpnIRwUxR5C4Eys8c6jqNSnrZ3ZK9VrtocO0pq_yGeFdF6OjcLKIT-8YYS1cph6jD-eLw

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
        "id": "1keYPJvW6FB7hGaXsqhyCZ7CwoMvaiXk_VaFjue1b3Rx9dZwlmdq3_LXzLW6-GYcNI_dkQ0efAoScYSbPFuB4g",
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
        "id": "rncnCNXtn9TifyjIRBuSS9NwEa4LkEujohxC3uUuEvKmgfRNVOk0eFL9shR9g3HdyKsv0S0RKI11q-cTGLTC6Q",
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
        "id": "6OxKBx0mTRZ2QZpJVbpnIRwUxR5C4Eys8c6jqNSnrZ3ZK9VrtocO0pq_yGeFdF6OjcLKIT-8YYS1cph6jD-eLw",
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
        "id": "OzuHNu3HBCQJv4o_AkTX0qjzwIKg-qfalYHGh1k66UfW3VutE3O01Y0n_pg4otwK3O-pyXBVTeGvX6uut0kfyg",
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
[32m✓ simple_websocket.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/batch_operations.js ===[0m
✓ Authentication successful

=== Batch Insert ===
(node:3745) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
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
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/kv_operations.js ===[0m
✓ Authentication successful

=== KV Set ===
(node:3758) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
✓ Set key: session:user123

=== KV Get ===
Retrieved value: {
  username: 'john_doe',
  loginTime: '2025-12-31T07:14:01.454Z',
  userId: 123
}

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
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/collection_management.js ===[0m
✓ Authentication successful

=== Create Collection (via insert) ===
(node:3766) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
Collection created with first record: qi5aAZ_sgSRjbl17I8n3s5u0kjm5oJI_21h3yZQuez76OPWNNM-3ckvRdwFki0iBgq8BjcXJjosIEA2SQk-vHA

=== List Collections ===
Total collections: 23
Sample collections: [
  'schema_products_client_go',
  'schema_users_client_ts',
  'test_collection',
  'schema_employees_client_ts',
  'schema_employees_client_go'
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
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/document_ttl.js ===[0m
✓ Authentication successful

=== Insert Document with TTL (1 hour) ===
(node:3769) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
✓ Inserted document: QnMykBYxUy8ZQV9zLeMzuQlsqwSkyfWzAB7MkZjbmt1AXNQPUJfSVDWab2rYCbLqW0s76_KygYO2DIiBcBee5w

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: created

=== Query Documents ===
✓ Found 4 documents with TTL

=== Update Document ===
✓ Updated document

=== Delete Document ===
✓ Skipped delete (no ID returned)

✓ All document TTL operations completed successfully
[32m✓ document_ttl.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/websocket_ttl.js ===[0m
✓ Authentication successful

=== Insert Test Data with TTL ===
(node:3789) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
✓ Inserted document with TTL: qOgPd5JqZ_P-0Z9jxSYssIZfU1_b-Vx2Zp51eO2UBxd8D0H2YfjIJN0aJl3o2xO27YP-n16TN1k0yHeyQA47vg

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 4 record(s) via WebSocket

✓ WebSocket TTL example completed successfully
[32m✓ websocket_ttl.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/http_functions.js ===[0m
🚀 ekoDB Scripts Example (JavaScript/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: zceM0jXyoHHXerSXpKFWE4zKrhyEkL04whK49HUnosGaHk29et7V-dIlPjxbLGVkh8gKQ3iXzziArJNQrotb1w
📊 Found 10 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: KNs3W8Yn5sLoIHd1f1VmYD7qu8lvxA4zGrMAH1kMHDvOwsRjARsdedLNuTiKma_udRR_EW3hjBRja_GGWthGfA
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: Z2oVRPhugcAf9eTXD-CJ2HymTicpiOrC_CVJQ5bLv-Lh4QNScu4AlNFFYpPNeyyT0U3B9MhLm7xcWvbUnzHPRg
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"count":10,"avg_score":60,"status":"active","max_score":100}
   {"max_score":90,"status":"inactive","avg_score":50,"count":10}

📝 Example 4: Function Management

📋 Total scripts: 20
🔍 Retrieved script: Get Active Users
✏️  Script updated
🗑️  Script deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

✅ All examples completed!
[32m✓ http_functions.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/transactions.js ===[0m
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: xNPfd30pxmYE-c-7yGUckTGsWh2cWfLJhmJHxscB6KW_C9v30sVX0rPOUUNza00x2nS2-xZnFxpzvaaiFs3MXQ
Created Bob: $500 - ID: q_Wh87SlJJcNrMToa7Gd5UgMvq8xWQB-nQEWVcSve6IeI3DHjNYY77i10BAhALZhAUHo9U4duJHtxZbKtMhUCg

=== Example 1: Begin Transaction ===
Transaction ID: f0ce5e24-f9ab-4711-b008-690c129717ba

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
New transaction: 81a43c18-4880-4ddd-b62b-fe5c4ced87e7
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
   ✅ Script saved: awh6cV0Qi8ltOKyLOPOpsnLUeXPzRvdLFL3V9PqU7FYxPu2od6yw67saaoBs_EbWuEmUsUaqUbsVFNP7LLEtig

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
   ✅ Script saved: sh6cF1JQbFTZEyyH5VaxB9Lvww2ZtQqC263CRusF19a1itCFMrA_-ocD1JAhGO-iLQAP_R5X8uJWhcRnrfvbKg

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
   ✅ Script saved: dcFS6Txfrc6peV10GCC99DKB03ms8l0QCMKDTgVNnly1oj7BTO0ei5rj6tgntlxvBA3WUI7U1RFxhZ5Y6cFOAA

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
   ✅ Script saved: q4Pm3dCieYj4HBIDOQwESOm1BxoT8gccoonFqhJehvhfmtb_FD8Nd0H_iEMyA3DzrbMKIez5jlulUX3u8z7yBQ

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: awh6cV0Qi8ltOKyLOPOp...
   ✅ Deleted script: sh6cF1JQbFTZEyyH5Vax...
   ✅ Deleted script: dcFS6Txfrc6peV10GCC9...
   ✅ Deleted script: q4Pm3dCieYj4HBIDOQwE...
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
╔════════════════════════════════════════╗[0m
[34m║           Test Summary                 ║[0m
[34m╚════════════════════════════════════════╝[0m
[34mTotal: 10[0m
[32mPassed: 10[0m
[32mFailed: 0[0m
✅ [32mJavaScript direct examples complete![0m
📦 [36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.5.0 prepare
> npm run build


> @ekodb/ekodb-client@0.5.0 build
> tsc


up to date, audited 7 packages in 823ms

found 0 vulnerabilities

> @ekodb/ekodb-client@0.5.0 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning JavaScript client library examples...[0m

added 1 package, removed 1 package, and audited 9 packages in 633ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: '9KejwXA3ApHmN871Af1CCUW6YKe_xFDeITDv9jrRGRaR6ZKuN39vsE1H3cvJAlLiowF6YpJ26kCOftKZtWNq5Q'
}

=== Find by ID ===
Found: {
  active: { value: true, type: 'Boolean' },
  id: '9KejwXA3ApHmN871Af1CCUW6YKe_xFDeITDv9jrRGRaR6ZKuN39vsE1H3cvJAlLiowF6YpJ26kCOftKZtWNq5Q',
  name: { value: 'Test Record', type: 'String' },
  value: { type: 'Integer', value: 42 }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  name: { value: 'Updated Record', type: 'String' },
  value: { type: 'Integer', value: 100 },
  active: { value: true, type: 'Boolean' },
  id: '9KejwXA3ApHmN871Af1CCUW6YKe_xFDeITDv9jrRGRaR6ZKuN39vsE1H3cvJAlLiowF6YpJ26kCOftKZtWNq5Q'
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
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
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: b7_YVVqCvFgGoupxWv14E4F-RTSaXv8P0KGdlSUn9rjjNLQSEdRuG3qNgQNQ7wAx6_nh_u8VHPmNEJqYUdIQtQ

=== List Collections ===
Total collections: 22
Sample collections: schema_products_client_go,schema_users_client_ts,test_collection,schema_employees_client_ts,schema_employees_client_go

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: 0wb_GNAyAB2vJBNjxTLu9zZnM4kg4EaxZdMtAPl6Ej1yk1Xli3AxJMI-iUfXNaBUgqTuCWwLKUY7FeX4tjlVjQ

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: XqXIuRhDH2Ddv3Psb4rylsEaNffY1tDnC-vT1sNQLdYUlexd8Dgck-yNZdWWeabeJXvT3vHGy5odWDcWKLBezQ

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

=== Inserting Test Data ===
✓ Inserted test record: gc7oF1UPGH_FpY1MruEjccHPylfphX0rwYSRUKXzjFQ_LoB4SDmCzMlBmoF4KJ6FrAzwQooWGgYRF9g_3ppOTg

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: fEw09gynuiR7dHZ9AI3yIBvtS8OrvGWKMkGQ6xTNu38DgdLb6LCK_UTlpi1OEnEeFYPYeJroRATbLUrG38sVrw

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 5 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
🚀 ekoDB Scripts Example (JavaScript/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: _mcO7uO1anLmcPSY28CL_fomR0ke5fpwMLWRtAnEhNY8iBp6BB5eSN4MNMZZRlP1DrXJknhQdWt8Wwjx02qmEg
📊 Found 10 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: kvwNmiCxlYTLlvFaitiU9_fREsAMB3TaZ719LwxzC_PjVVu2WQtpJBTD4aNvupEOGAiHyWGMaBRUMh6oV52YNQ
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: gNiEzCrTd0RMwT1556ZYonnEbslkM5Gvktcy39yhIcN-OE7X6yhXOKijoT4CafZbhJdecFN4iNVjlLWZm8_f2w
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"max_score":90,"count":5,"avg_score":50,"status":"inactive"}
   {"count":5,"avg_score":60,"status":"active","max_score":100}

📝 Example 4: Function Management

📋 Total scripts: 22
🔍 Retrieved script: Get Active Users
✏️  Script updated
🗑️  Script deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

✅ All examples completed!
🚀 ekoDB Scripts Example (JavaScript Client)

✅ Client initialized (token exchange automatic)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: WrFAnddTIJ8GPKWbEphFwAKINwKhjKVQycLzkN_vXLZRj5pxMHTQicHcpc34qDKfc_OmLu7_weJ7U51CQ-i6TQ
📊 Found 20 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved
📊 Found 20 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved
📊 Statistics: 2 groups
   {"avg_score":60,"status":"active","count":10}
   {"avg_score":50,"status":"inactive","count":10}
⏱️  Execution time: 0ms

📝 Example 4: Script Management

📋 Total scripts: 24
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
📊 Total user count: 20
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
   Furniture: 4 items (avg $294.00)
   Electronics: 6 items (avg $325.67)
⏱️  Execution time: 0ms

📝 Example 3: Count All Products

✅ Script saved
📊 Total products: 10
⏱️  Execution time: 0ms

📝 Example 4: Multi-Stage Aggregation

✅ Script saved
📊 Category analysis (2 categories):
   Furniture:
      Products: 4 | Stock: 43 | Avg Rating: ⭐4.26
   Electronics:
      Products: 6 | Stock: 232 | Avg Rating: ⭐4.52

⏱️  Total execution time: 0ms
📈 Pipeline stages:

📝 Example 5: Project Specific Fields

✅ Script saved
📊 Product summaries (10 items, showing first 3):
   1. Monitor 27" - $399 (⭐4.6)
   2. Standing Desk - $599 (⭐4.7)
   3. Laptop Pro - $1299 (⭐4.8)
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced script examples completed!
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
   admin: 20.0 (3 users)
   user: 70.0 (7 users)
⏱️  Execution time: 0ms

📝 Example 4: Top Users by Score

✅ Script saved
📊 Users (showing first 5 of 0):
⏱️  Execution time: 0ms

📝 Example 5: User Summary Statistics

✅ Script saved
📊 User summary (2 groups):
   inactive undefineds: 3 users, Total Score: 180
   active undefineds: 7 users, Total Score: 370
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
   2. Vector Databases Explained (Database)
   3. Database Design Principles (Database)
   4. Getting Started with ekoDB (Database)
   5. Natural Language Processing (AI)
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
   1. Introduction to Machine Learning
   2. Vector Databases Explained
   3. Database Design Principles
   4. Getting Started with ekoDB
   5. Natural Language Processing
⏱️  Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Script saved
📊 Projected documents (showing first 3):
   1. Introduction to Machine Learning
   2. Vector Databases Explained
   3. Database Design Principles
⏱️  Execution time: 0ms

📝 Example 5: All Document Fields

✅ Script saved
📊 All documents (5 total, showing first 2):
   1. Introduction to Machine Learning (AI)
   2. Vector Databases Explained (Database)
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples completed!
🚀 ekoDB AI Scripts Example

⚠️  Note: These examples require AI API credentials (OpenAI, etc.)

📋 Setting up test data...
✅ Created 3 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
🤖 AI Response:
   1. High Precision: Vector data is represented as points, lines and polygons, enabling high precision and accurate representation of spatial data.

2. Scalability: Vector data can be easily scaled up or down without any loss in resolution or detail.

3. Detailed Analysis: Allows for complex and detailed spatial analysis such as network analysis, proximity analysis, and topological analysis.

4. Less Disk Space: Vector data usually requires less disk space than raster data.

5. Clear Boundaries: It provides clear boundaries for features like roads, buildings, and land use parcels. 

6. Attribute Storage: Each entity in vector data can hold multiple attributes, allowing for a rich dataset.
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embedding script saved
📊 Generated embeddings for 3 articles
   1. "Getting Started with ekoDB" - 1536D vector
   2. "Draft Article" - 1536D vector
   3. "Advanced Query Patterns" - 1536D vector
⏱️  Execution time: 0ms

📝 Example 3: List All Articles

✅ Script saved
📊 Found 3 articles
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All AI examples completed!
✅ [32mJavaScript client examples complete![0m
