make test-examples-javascript
🧪 [36mRunning JavaScript examples (direct HTTP/WebSocket)...[0m

added 1 package, removed 1 package, and audited 9 packages in 2s

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
=== Running ekoDB/ekodb-client/examples/javascript/simple_crud.js ===[0m
=== Simple CRUD Operations (Direct HTTP) ===

✓ Authentication successful

=== Insert Document ===
Inserted: {
  id: 'eOopeFqJK1Y_3QMo5GL3oxkvOdIxHQP6Y1CVt_CtKzBNtNjT_TUz6ffgDu-bz6hYCcwT7LyaAjw_VBbdAvqteQ'
}

=== Find by ID ===
Found: {
  active: { type: 'Boolean', value: true },
  id: 'eOopeFqJK1Y_3QMo5GL3oxkvOdIxHQP6Y1CVt_CtKzBNtNjT_TUz6ffgDu-bz6hYCcwT7LyaAjw_VBbdAvqteQ',
  value: { type: 'Integer', value: 42 },
  name: { value: 'Test Record', type: 'String' }
}

=== Find with Query ===
Found documents: [
  {
    name: { value: 'Test Record', type: 'String' },
    active: { type: 'Boolean', value: true },
    id: 'eOopeFqJK1Y_3QMo5GL3oxkvOdIxHQP6Y1CVt_CtKzBNtNjT_TUz6ffgDu-bz6hYCcwT7LyaAjw_VBbdAvqteQ',
    value: { value: 42, type: 'Integer' }
  }
]

=== Update Document ===
Updated: {
  active: { type: 'Boolean', value: true },
  name: { type: 'String', value: 'Updated Record' },
  value: { type: 'Integer', value: 100 },
  id: 'eOopeFqJK1Y_3QMo5GL3oxkvOdIxHQP6Y1CVt_CtKzBNtNjT_TUz6ffgDu-bz6hYCcwT7LyaAjw_VBbdAvqteQ'
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
✓ Inserted test record: tVaC7Hn-7nEn_6PKSnRD-6GIxQ5lFR0YLbPm6I9EhSIFt1vlfEOhFoIUXDIsB1CzmDRMuwYlaOHsrRwghWuoug

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "type": "Success",
  "payload": {
    "data": [
      {
        "id": "YzqB7vN0r4D-3zhsgSJ8CEDDI_9gRK0zRHJnV7qaDbIYbCjfXHmDa2JMO7MiFFVbRGFPw8PepqVJ69RV3PvSHQ",
        "name": {
          "value": "WebSocket Test Record",
          "type": "String"
        },
        "value": {
          "type": "Integer",
          "value": 42
        },
        "active": {
          "value": true,
          "type": "Boolean"
        }
      },
      {
        "name": {
          "value": "WebSocket Test Record",
          "type": "String"
        },
        "active": {
          "type": "Boolean",
          "value": true
        },
        "value": {
          "value": 42,
          "type": "Integer"
        },
        "id": "tVaC7Hn-7nEn_6PKSnRD-6GIxQ5lFR0YLbPm6I9EhSIFt1vlfEOhFoIUXDIsB1CzmDRMuwYlaOHsrRwghWuoug"
      },
      {
        "value": {
          "type": "Integer",
          "value": 42
        },
        "active": {
          "value": true,
          "type": "Boolean"
        },
        "id": "u3ivjmlelJBizFkDhlxT3MSmU9WcuRpLEDNXmnUOoP5hHLMbjhJ08sFivznW63a-_FAkIwMOh6cWr8m03qkGhg",
        "name": {
          "value": "WebSocket Test Record",
          "type": "String"
        }
      },
      {
        "value": {
          "type": "Integer",
          "value": 42
        },
        "active": {
          "value": true,
          "type": "Boolean"
        },
        "id": "atjnQ2_MbhcPgzST8zEGu9Jf2fHreO5rAF3g7PXqpGDatOl5ntdvSZjqLjtL45S73QikEaPqSgzPv4YED1uK8Q",
        "name": {
          "value": "WebSocket Test Record",
          "type": "String"
        }
      }
    ]
  },
  "messageId": "1784037083834"
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

✅ Function saved: OOpnMzHL5G52sfb65iKxmXTlQZgApbi05ukkiIOmwV2bSgc5gj_LKGIG8wbirLQTDtHY4-5vAaGJShKacH-j3A
📊 Found 10 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Function saved: xEBwwvmq4QPlbbFGgcINqZ0utSaDjZGEYsXVnM82hwqPxgKjSXdPe1MG53gfdxAIOOe1BGhM-A5_esIH1VKyHQ
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: XAZiyFxeL5lE_wzZg8GYtKk8LpdKD0RVhZzzI0fds6doee_0UJ0mEVEPCPTIuc61KtI8lZdsNyiHkZBSjuiY3w
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"max_score":{"value":90,"type":"Integer"},"status":{"type":"String","value":"inactive"},"count":{"type":"Integer","value":10},"avg_score":{"value":50,"type":"Float"}}
   {"avg_score":{"type":"Float","value":60},"count":{"value":10,"type":"Integer"},"status":{"value":"active","type":"String"},"max_score":{"value":100,"type":"Integer"}}

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
Retrieved value: { type: 'Object', value: { userId: 123, username: 'john_doe' } }

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: { value: { price: 29.99, name: 'Product 1' }, type: 'Object' }
cache:product:2: {
  type: 'Object',
  value: { name: 'Product 2', price: 39.989999999999995 }
}
cache:product:3: {
  value: { name: 'Product 3', price: 49.989999999999995 },
  type: 'Object'
}

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
Collection created with first record: xYkVCuUglOzt1hlKFtjKqAklqHnUlPSFZUiLqprSSSmpj66u996s68UZS-JVZuwMJT4YTu9DUD76zXalo2OJzA

=== List Collections ===
Total collections: 26
Sample collections: [
  'schema_documents_client_ts',
  'audit__ek0_testing',
  'schema_employees_client_ts',
  'functions__ek0_testing',
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
=== Running ekoDB/ekodb-client/examples/javascript/transactions.js ===[0m
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: W-pzLbEftSVitSpSOnqZVpKaZ1DSd77KiqsOFFQfwyAYEr7An9KsfUJ57UuIkVA90geiL5EBkRyxc4KI-0HtyQ
Created Bob: $500 - ID: -snYuHbL4S2Le3cSr02XW9eNU3zqP2FUVgZz0sV6VpxTklMiql3ZfzKj6C8RVA1D1IiNwgto3e-HKXhwZLJc5w

=== Example 1: Begin Transaction ===
Transaction ID: ba1087ea-5b4f-4933-b523-545af720d064

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
New transaction: 78d91087-be68-4dd9-b4e5-0b9fb59c38ca
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
   ✅ Function saved: QX1vgUvzYG4mfq8hGiWKL0ynbipB98oXH6Yxn7dFeUSyi5TcEbi7rzVi9N2NWkrzL5UWkFjw7Os8DcUC1WhNEQ

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
   ✅ Function saved: rPT4cUZ54wpmRYBET71CCllkn1_dOCWyOExgM3dGBRB2Y3M-7FRxUIXALYMnkmMOcxd36SkrWXk7Ki16S73vOg

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
   ✅ Function saved: cIVM5EcBGuGdLycJeZrEVRls1XqNC_NopSwJCLyNiaBqqsq12ApVSmqlerFRd6NFME6idSHy-WxBRGKPR5YkEA

2️⃣ Calling Function (Query + Update Credits + Verify)...
   ✅ Function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {"type":"Integer","value":0}
   📋 Status: {"value":"active","type":"String"}
   📋 Name: {"type":"String","value":"Alice Smith"}

============================================================
📝 Function 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: l92113sFFuKlxJgwBdLUMwjdTXOrT0wZqwvDgIfAElx8YqzLc_ntwScCAtgeXKGUy0prrAE-2jYm7Drvb6k7WA

2️⃣ Calling Function (Query + Delete + Verify)...
   ✅ Function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted function: QX1vgUvzYG4mfq8hGiWK...
   ✅ Deleted function: rPT4cUZ54wpmRYBET71C...
   ✅ Deleted function: cIVM5EcBGuGdLycJeZrE...
   ✅ Deleted function: l92113sFFuKlxJgwBdLU...
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
  Output: Document ID = OKk8mv47XZjiXWvztCT2TN0SQUdBTOk2SQjjUwyXz0tRWGetxxQXnoMAG_z5dYnfEiu9DYhBvIjnDLcAQrRjdQ
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: findById(OKk8mv47XZjiXWvztCT2TN0SQUdBTOk2SQjjUwyXz0tRWGetxxQXnoMAG_z5dYnfEiu9DYhBvIjnDLcAQrRjdQ)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: findById(OKk8mv47XZjiXWvztCT2TN0SQUdBTOk2SQjjUwyXz0tRWGetxxQXnoMAG_z5dYnfEiu9DYhBvIjnDLcAQrRjdQ)
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
  Output: Document ID = 36jB-cdJgZOezkHy4EWQvHoAp2o0KdBC012-dVL1kgbA5RzXcu116JwQ33DRuoKS-4CxT37U8fQIP0yHWGA4Mw
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: findById(36jB-cdJgZOezkHy4EWQvHoAp2o0KdBC012-dVL1kgbA5RzXcu116JwQ33DRuoKS-4CxT37U8fQIP0yHWGA4Mw)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: findById(36jB-cdJgZOezkHy4EWQvHoAp2o0KdBC012-dVL1kgbA5RzXcu116JwQ33DRuoKS-4CxT37U8fQIP0yHWGA4Mw)
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

> @ekodb/ekodb-client@0.25.0 prepare
> npm run build


> @ekodb/ekodb-client@0.25.0 build
> tsc


up to date, audited 52 packages in 1s

17 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.25.0 build
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
  id: 'jNwrxz3qlov6HhCVx-ppFNgIY_txjVUW_2HE1vb1BNws1FSpmGcxHRlvy38lSadBffW205o2J16XRdTkJTGLgA'
}

=== Find by ID ===
Found: {
  name: { value: 'Test Record', type: 'String' },
  active: { type: 'Boolean', value: true },
  id: 'jNwrxz3qlov6HhCVx-ppFNgIY_txjVUW_2HE1vb1BNws1FSpmGcxHRlvy38lSadBffW205o2J16XRdTkJTGLgA',
  value: { value: 42, type: 'Integer' }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  id: 'jNwrxz3qlov6HhCVx-ppFNgIY_txjVUW_2HE1vb1BNws1FSpmGcxHRlvy38lSadBffW205o2J16XRdTkJTGLgA',
  value: { type: 'Integer', value: 100 },
  active: { value: true, type: 'Boolean' },
  name: { type: 'String', value: 'Updated Record' }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: 2nHu6QCwGJARQAUHtlnWNdtq4sF5ij4plV_gs-R-LK2U07GsOY3LKN7G4ZqfZTIEa96Kcns8GIgy6hh9MMpY2Q

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
Collection created with first record: _a3NOn4wXU-LAj7qiHb3dUworbiCArezzrlSb_B2KsEEunb-jizedgZOpTJKoy1PCS4aM784HhmXEBFEzlwkng

=== List Collections ===
Total collections: 25
Sample collections: schema_documents_client_ts,audit__ek0_testing,schema_employees_client_ts,functions__ek0_testing,schema_users_client_go

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
Retrieved value: { type: 'Object', value: { userId: 123, username: 'john_doe' } }

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: { type: 'Object', value: { price: 29.99, name: 'Product 1' } }
cache:product:2: {
  value: { price: 39.989999999999995, name: 'Product 2' },
  type: 'Object'
}
cache:product:3: {
  value: { price: 49.989999999999995, name: 'Product 3' },
  type: 'Object'
}

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
Created Alice: $1000 - ID: g-Y5cIEc6R_--MH7pb9Pjk0QW7TMMudLujLtBspdjc7FxlK4_Jwe1eErsrwp97QZIaxY7QOdYM0_9dIoE1tSQQ
Created Bob: $500 - ID: WeL6XTcqxJhi0vnKdrRNTUttCTkEslP4LO2q3qFw3Q8gjnbnXC8Xxwnl5kC_NhyNgEhEdSPlGI3sUH5KyPteYA

=== Example 1: Begin Transaction ===
Transaction ID: 39537171-2288-41f8-92fc-2e58e6f58df4

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 1f9ef7a3-470f-490f-8de8-96996d557ddf
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
  1. Score: 12.870, Matched: email, name
  2. Score: 6.270, Matched: name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 13.200, Matched: title, bio
  2. Score: 13.200, Matched: title, bio
  3. Score: 13.200, Matched: title, bio
  4. Score: 13.200, Matched: title, bio

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 26.400, Matched: bio, title
  2. Score: 26.400, Matched: bio, title
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
  1. Score: 0.780, Matched: 
  2. Score: 0.744, Matched: 
  3. Score: 0.727, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.491, Matched: content, title
  2. Score: 0.912, Matched: title, content
  3. Score: 0.298, Matched: 

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
✓ Inserted document: p4WFCgk2fJzIsWjbXTOPPHtphcn9I1aad3djZEu17hnVmC1jgrwR_Qnk7iObKprz3pL1TAfr-tgh2SHrPWvLEg

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: yVIaYGq2ik_-ms0hOkAkqOeh4FN1iQ2y_UC-E_Byvewg23idse2jm47wbKFdfTXLqPVSPaU_DsGYD35Xv32VVQ

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
✓ Inserted document with TTL: jNR-SkAfO7HrPWCT8Rw9N9vuKmVxeJvB9ImovS76xO4fqeM4vV0M4y9tdyrQXbJSizhNQ7Q6TNAsaQnDStrOlg

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
✓ Edge cache script created: Jnl5MZzrBPnQ1wk--ZCYCaY4wdhDOeegS4a1SVtdWranvbX9xiPCMbKMsxh7p_0oOqDpUN8Kzi1AvczTvt1rsQ

Call 1: Cache miss (fetches from API)
Response time: 3ms
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "utc_offset_seconds": 0,
          "generationtime_ms": 0.01919269561767578,
          "timezone": "GMT",
          "current": {
            "interval": 900,
            "temperature_2m": 27,
            "time": "2026-07-14T13:45"
          },
          "elevation": 32,
          "latitude": 40.710335,
          "longitude": -73.99308,
          "current_units": {
            "interval": "seconds",
            "temperature_2m": "°C",
            "time": "iso8601"
          },
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
Response time: 2ms (1.5x faster!)
Result: {
  "records": [
    {
      "value": {
        "value": {
          "utc_offset_seconds": 0,
          "generationtime_ms": 0.01919269561767578,
          "timezone": "GMT",
          "current": {
            "interval": 900,
            "temperature_2m": 27,
            "time": "2026-07-14T13:45"
          },
          "elevation": 32,
          "latitude": 40.710335,
          "longitude": -73.99308,
          "current_units": {
            "interval": "seconds",
            "temperature_2m": "°C",
            "time": "iso8601"
          },
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

🚀 ekoDB Functions Example (JavaScript Client)

✅ Client initialized (token exchange automatic)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

ℹ️  Function 'get_active_users' already existed — updated instead
✅ Function saved: gybocqxke9oCp3Egl6LU-iZDHlmB_rvhmmaU8Vy92vanmGAVYrkFv0XW5xQBuzo2lNBVJbUIQ1lU6rj2rqFaYA
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
   {"avg_score":{"type":"Float","value":50},"count":{"type":"Integer","value":5},"status":{"value":"inactive","type":"String"}}
   {"avg_score":{"type":"Float","value":60},"status":{"type":"String","value":"active"},"count":{"type":"Integer","value":5}}
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
   Name: {"value":"User 1","type":"String"}
   Department: {"type":"String","value":"engineering"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

ℹ️  Function 'fetch_and_store_user' already existed — updated instead
✅ Saved reusable function: fetch_and_store_user (uses KV)
ℹ️  Function 'swr_user' already existed — updated instead
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 78ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "company": {
        "catchPhrase": "Multi-layered client-server neural-net",
        "name": "Romaguera-Crona",
        "bs": "harness real-t...

Second call (cache hit - from cache):
   ⏱️  Duration: 2ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "company": {
        "catchPhrase": "Multi-layered client-server neural-net",
        "name": "Romaguera-Crona",
        "bs": "harness real-time e-markets"
      }...
   🚀 Cache speedup: 39.0x faster!

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

✅ Function saved: 8aDi7sKbzqa-sdrcZBjUW17cBJtoz7CvXrWr_yeVu5zaR-cDVGJCPQXwHo5hGXO8HzSmUvVF1dX_lzOVeT9BOw
📊 Found 2 product groups
   {"avg_price":{"value":474,"type":"Float"},"count":{"type":"Integer","value":2},"category":{"type":"String","value":"Furniture"}}
   {"avg_price":{"type":"Float","value":575.6666666666666},"category":{"type":"String","value":"Electronics"},"count":{"type":"Integer","value":3}}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   {"category":{"value":"Furniture","type":"String"},"count":{"value":2,"type":"Integer"}}
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

✅ Inserted order: 0zSv8CR93f46bydM6PxF_kIeG2Bo6Y9NBz2mTbKYxfWtnfpLREdx1mpM1k78HqobYSDY5Q2Q3ofV5suhRt1eXg
✅ Inserted 2 products with wrapped types

📝 Example 2: Function with Wrapped Type Parameters

✅ Function saved: R4b9lP4suVeKrqCeJ0wj0zuCmrkFFZIHcG-sENnl-YYdVTCb3dWKuWxlC-3l4iZxQd2TieR2NGh92PXO8FxVrw
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"type":"Object","value":{"userId":"user_abc","role":"admin"}}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: NJucTs3XjWw2zcd3Uy_-oeSjpBmAUR8ei--hwYyLU1QANH5R3FW8ehkx0CqymSChM5b_HemUTEEO7xwyxhGuew
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Function

✅ Function saved: tTnq2SQunyzM23Bygg9Gd7eqZVlLAAzyIFIOErMblrGrRRgiVOcNaD1rK5Z_-S8Rtvi2CxyQVVLEG9KShaLZgA
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
   1. User 9 - Score: 90
   2. User 5 - Score: 50
   3. User 8 - Score: 80
   4. User 3 - Score: 30
   5. User 7 - Score: 70
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
   Database: 3 documents
   AI: 2 documents
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
   2. USB-C Cable - $19 (⭐4.3)
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
   1. High Precision: Vector databases provide highly accurate and precise mapping and representation of geographic data, including points, lines, and polygons.

2. Scalability: These databases allow data to be scaled up or down without loss of quality, contrary to raster databases.

3. Compact Data Storage: Vector data is generally smaller in file size, allowing for efficient storage.

4. Less Redundancy: Since vector data is often structured and linked, it reduces redundancy in data storage.

5. Attribute Storage: Along with geographic locations, vector data can also store attributes related to those locations.

6. Complex Analysis: Vector databases support more complex spatial analyses such as topological, network and proximity analysis.
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
✓ Created SWR script: fetch_api_user_js (rqxibQnySECc0IsC9MJnzeP7vV9AwYX2Lh4jCs3Kroxu9j3KOHBSWvLFZlGhFbdsZ9HOCPWIzPoNBWzf_34d-w)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "id": 1,
          "website": "hildegard.org",
          "email": "Sincere@april.biz",
          "address": {
            "city": "Gwenborough",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "zipcode": "92998-3874",
            "suite": "Apt. 556",
            "street": "Kulas Light"
          },
          "company": {
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets"
          },
          "name": "Leanne Graham",
          "phone": "1-770-736-8031 x56442",
          "username": "Bret"
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
✓ Created native SWR script: github_user_native (wIC8fQtYnKFN27q3BkDluqWnwLiAC2NDETThD2asVAhnlVZH9k4NVHpgBhdk_gdrQ3stOARWRXYm7TYkI3y4SA)

First call (cache miss - will fetch from GitHub API):
  Response time: 3ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 13ms
  Speedup: 0.2x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (i9pgNEPXlHT8Sk-eRgKcCQmcf3Z1Wayp4W1RvBXgzG6_5iLeEjZjabjm74RaNeBQP8kXhOsZRk6U2N4ddKZscQ)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (t_vyCrNG_2bR9P8Cfi1fIHHi2EY1jQAQAGEGDsPjLsL_4NvgXMwxKKScMGzgE5P6D-6DzuHRAG1UXBrN-gJdWw)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (fd8NvTJ0Gn4c3r87SFTBy9Z0gbDKOzhBx28fMVwnn2vAw3-yaV20-HSCB9bWrIAfqP1QXaF0KKvOqqVx4iTCwQ)
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
✓ Created session: eC4q7849S3JxxpfL79scSXIVcez3XLnJQCE5YaGLJEvH27HlHDK5z7Nlue4erGw-ntWkh6PQYehD3G4JZBu5JA

=== Sending Chat Message ===
Message ID: hFM7rKUYGFTkmdCDGm8QTRIzDHh0f0lwBnyFlASyHfXGFY9i4eV4rBwpdZ-PTBAmbu60OKmS7DMTb6ZfXPRI4Q

=== AI Response ===
The available products and their prices are as follows:

1. **ekoDB**: $99
   - Description: A high-performance database product with AI capabilities.

2. **ekoDB Cloud**: $499
   - Description: Fully managed cloud database service product.

3. **ekoDB Pro**: $299
   - Description: Enterprise edition product with advanced features.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_js',
  record: {
    id: 'cKKZV2R89VS0ZbLj7HZ5M6JwvP8_n-a1r6QCfW3igv2NxMv7GdBiZkDC8uU4S2Ff5WF6d8FhzvaZ9JXsQklnGg',
    description: 'A high-performance database product with AI capabilities',
    name: 'ekoDB',
    price: 99
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_js',
  record: {
    id: 'hR4kF8W0wA5-NjiYh87V7aTZPUNuANdEogGaKd9J5BaghLJr-RKpwaXj-5M8_iA2VS_u61E1bLTJVZltIchREQ',
    name: 'ekoDB Cloud',
    price: 499,
    description: 'Fully managed cloud database service product'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_js',
  record: {
    id: '0Jp1PnFYPystGUd6neJk1FLfo7OQ7_a84a8M4Aienu7Xx5RZToLOs6vwyxKmuXTtOO5kOAGN1GzXz4_j4pBTwg',
    name: 'ekoDB Pro',
    description: 'Enterprise edition product with advanced features',
    price: 299
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 2214ms

=== Token Usage ===
Prompt tokens: 3413
Completion tokens: 83
Total tokens: 3496

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: py3B7IGZiqvE3f8monY4-9Duwy6eCEDLKYBJRZP9VsKGkBwSE0dA7I__jBCjbYpLE2M3HF3vR_0EJCB7g6PrOA

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: High-performance database product
- **Price**: $99

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
✓ Created second session: s-IdKx0xh0VZEkmdE_1iLOVKS6PHAv7_fW8kX0dGpbkgcc1tqJyZVftZTFg6Uj1aVHWaj7zbt-v96X956W2-ow
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
✓ Created session: vHR8bw-pFrVlE-b3UlML-v7S17cHM4Ni-AkZoynpjpzXbkMclahvFTkBy2t-loRfcFg90XCmvoMvEYkR4t13tg

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** A high-performance database product
- **Price:** $99

If you need more information or details about other products, feel free to ask!

✓ Message 2 sent
  Response: The price of ekoDB is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: VmMMt1uljWLBjp4zrr_SAXGuBei3ANxPh5RumxwvPBHyQwXbuCz9Ky-qEXXTKpbERQe9odW4PYpub0qcHI2FRg
  Parent: vHR8bw-pFrVlE-b3UlML-v7S17cHM4Ni-AkZoynpjpzXbkMclahvFTkBy2t-loRfcFg90XCmvoMvEYkR4t13tg

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: VmMMt1uljWLBjp4zrr_SAXGuBei3ANxPh5RumxwvPBHyQwXbuCz9Ky-qEXXTKpbERQe9odW4PYpub0qcHI2FRg (Untitled)
  Session 2: vHR8bw-pFrVlE-b3UlML-v7S17cHM4Ni-AkZoynpjpzXbkMclahvFTkBy2t-loRfcFg90XCmvoMvEYkR4t13tg (Untitled)
  Session 3: s-IdKx0xh0VZEkmdE_1iLOVKS6PHAv7_fW8kX0dGpbkgcc1tqJyZVftZTFg6Uj1aVHWaj7zbt-v96X956W2-ow (Untitled)
  Session 4: eC4q7849S3JxxpfL79scSXIVcez3XLnJQCE5YaGLJEvH27HlHDK5z7Nlue4erGw-ntWkh6PQYehD3G4JZBu5JA (Untitled)
  Session 5: DyNzgkvtjaAEoImVA8qNctSkz8BbiwUXfGUJMDK4mTzHUOfuIQUdw216S1dx6wZcldM2PCrYlhsRidPdyZwOsw (Untitled)
  Session 6: kh63L_Y5cAQ8KiVa8b7WjW98FN4ggbXSH_Kk-FUGlEqa6zM6YHI2GKnDi0pS7MR2hzRVGAMg36eRndnAO3pTrw (Untitled)
  Session 7: y13STnGRlV8yVtIOyoLZ1umXLkWfiFqDhf0HA_WGnHbpkwAk53NEgjkgJ3VT2i2YMmgrKfTc3_PH3NUNs4sd3w (Untitled)
  Session 8: JZXsdYZRowdSNnZC032ydC1IzfTYLbRWpHFsE88nJurrWEPU0PI3q-JLNCAcfF2ZKHALHQZU1v4rN6hakdJY_g (Untitled)
  Session 9: 0dx8aDH0e8RiG18HdyqkVaDYNAEO2fEpN4pWEznplWmNPHbZAEilObvPbVGmetQq8kRVsjnVTP7e5vKlDZvgSQ (Untitled)
  Session 10: G_8w79TiVJKFrYmE8bAkQud9vdaHl8IbnT9tGq9o3CVOjU90bk2JgrCBki15lCiU_ij8pEeIxRf4-tKWXu7l2w (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: VmMMt1uljWLBjp4zrr_SAXGuBei3ANxPh5RumxwvPBHyQwXbuCz9Ky-qEXXTKpbERQe9odW4PYpub0qcHI2FRg

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
✅ [32mJavaScript client examples complete![0m
