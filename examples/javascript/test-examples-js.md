make test-examples-javascript
🧪 [36mRunning JavaScript examples (direct HTTP/WebSocket)...[0m

added 1 package, removed 1 package, and audited 9 packages in 983ms

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
  id: 'tHGA0pH2jWeeBUJ7gHtAmEKORB3X0YhT4NsGb2OUIjmNqzqnMAf1NlPCbmMuENq8B1dTjn8W8BAekN_eMsnYxw'
}

=== Find by ID ===
Found: {
  name: { type: 'String', value: 'Test Record' },
  active: { type: 'Boolean', value: true },
  id: 'tHGA0pH2jWeeBUJ7gHtAmEKORB3X0YhT4NsGb2OUIjmNqzqnMAf1NlPCbmMuENq8B1dTjn8W8BAekN_eMsnYxw',
  value: { value: 42, type: 'Integer' }
}

=== Find with Query ===
Found documents: [
  {
    id: 'tHGA0pH2jWeeBUJ7gHtAmEKORB3X0YhT4NsGb2OUIjmNqzqnMAf1NlPCbmMuENq8B1dTjn8W8BAekN_eMsnYxw',
    name: { value: 'Test Record', type: 'String' },
    value: { value: 42, type: 'Integer' },
    active: { type: 'Boolean', value: true }
  }
]

=== Update Document ===
Updated: {
  active: { type: 'Boolean', value: true },
  name: { type: 'String', value: 'Updated Record' },
  value: { value: 100, type: 'Integer' },
  id: 'tHGA0pH2jWeeBUJ7gHtAmEKORB3X0YhT4NsGb2OUIjmNqzqnMAf1NlPCbmMuENq8B1dTjn8W8BAekN_eMsnYxw'
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
✓ Inserted test record: nelTg6jXoWDgdaNxfywOhxSl74geEBzCjmsbTQ_9sINcutq4kwCjQ0kpvqeFKIVgCdP4uxlTe0flG9xEQXAzNw

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "messageId": "1774741938999",
  "payload": {
    "data": [
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "nelTg6jXoWDgdaNxfywOhxSl74geEBzCjmsbTQ_9sINcutq4kwCjQ0kpvqeFKIVgCdP4uxlTe0flG9xEQXAzNw",
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
        "id": "_S5yYpl3xBCh_YyPvDcmKVdwN76GYR1Oecn1I3cEbjoR2IwpNSv7WkqMKf8ONNyBKeTkGJG7Gfwp6qJ1Ag1tbg",
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
        "id": "mmpnpqj8wl8y2nCQsIu6Gr0V-dAmd7j7kPjU6Xqi3-P9by1YQW8f19sL-wubi6PxdiWatf2aQJbUal9HOFNuXQ",
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
        "id": "uECQwVBLmwZzmL6GZrW53lL84vBuwZLnKCf76m2k1zWzCWK2lhqx5kfniy4ncQB0Ggm7bX1kOBzBkTnjpJKOHg",
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

✅ Script saved: SMCx65J8denZLsUsG79eqnbbnMxMIKV6LpIPzFLpDsj1xNoUSvqsfbKHx9weVQoFo2HWSsRsIz0cthwkRkfxbg
📊 Found 10 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: 5PTJHAqB51J97t21tVcx9IPxeYcDryyjg_Zd6iG0zeABU_MG5NV1nHjZ0ttTFh4-uF6DNaFkBPEO1Zk6sDE22Q
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: jCNVB7fDUeAAsBtQU6_IHWhXkpEJ4RwSwGzkvQskvXneLGm9iSpe9M8iPXb5NrgXF4_QjxgunN8E87WGLpBfSA
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"count":10,"avg_score":50,"max_score":90,"status":"inactive"}
   {"avg_score":60,"count":10,"max_score":100,"status":"active"}

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
Collection created with first record: wMVexxkcuPpseFXX_ISH1_TzF6RrUEwZIRGvz-9yZ5y6cJ3rqzlCOmLI28WQr4UXt0rSZU9jkeyQa1TJspoKkQ

=== List Collections ===
Total collections: 26
Sample collections: [
  'schema_documents_client_ts',
  'schema_products_client_ts',
  'users',
  'chat_configurations__ek0_testing',
  'demo_collection'
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
Created Alice: $1000 - ID: voigQzoEL6rQ4JvTXP2WOmSRuelPHjtwcfDWEflow59a0FkbmMTpWxuv9cblWPLrgov8rXpsPvuL_2bbeJWcMQ
Created Bob: $500 - ID: 47LDNbYK31fYCywpPmZ4EWtp4SEqZZ22WONM3niaKXtcWMu_rhFJ87w-535TXhTc4dwYxrohE0ueFgPSxUwEow

=== Example 1: Begin Transaction ===
Transaction ID: 24935fa6-25a7-4a21-a25a-4d728c6ca42c

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
New transaction: 32f9ba28-eef3-4ef7-b7cb-c398bab21be6
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
   ✅ Script saved: 2MwQNFkKl6aceqJfKHvWZl3nccGOi5ocx1h3kLECglEvPNSluIjxqaC-CNIzKnSusLUt2Hzn3VDCZ9Pi3HuGIA

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
   ✅ Script saved: xG8r8MybiYdt2wWRGkQuW9g9XBAYgoVpqZfK6GcuPBPIAO7xoUrgqA51MYbn25EImzE9Qpo47yHmD3iEBmLApw

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
   ✅ Script saved: APaKpiGen96ZOE7eCut2ZY0KtE4fuKQbYweeweZCAeLV3vdEXuhSzce78KplUA9y1KP-yzVkFkqQD1OVNAGIFA

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
   ✅ Script saved: yeSceQanrSvBUGX3NPl3UHlK_3tqcgwQv7nfYUx_ObbyW_kGn1jtR2PhPq3Ox0sa6LRzJ6YUAPtwX_FdXNR9Pg

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: 2MwQNFkKl6aceqJfKHvW...
   ✅ Deleted script: xG8r8MybiYdt2wWRGkQu...
   ✅ Deleted script: APaKpiGen96ZOE7eCut2...
   ✅ Deleted script: yeSceQanrSvBUGX3NPl3...
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
  Output: Document ID = o7fNYtg10atH9YbUUqKb1T5F6KPdVTIRrm62UstLMPCFaS2q450oqQs7jC44hLHikcy8lzLjx1PuHC6zgBv7WA
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: findById(o7fNYtg10atH9YbUUqKb1T5F6KPdVTIRrm62UstLMPCFaS2q450oqQs7jC44hLHikcy8lzLjx1PuHC6zgBv7WA)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: findById(o7fNYtg10atH9YbUUqKb1T5F6KPdVTIRrm62UstLMPCFaS2q450oqQs7jC44hLHikcy8lzLjx1PuHC6zgBv7WA)
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
  Output: Document ID = mtQfVin15v1A0Yg21i2Ntyrtgn4J-VSrNRZEtyXOQvLXgLRGw0Z63danGQFKoOEiaVAU05xWfH1gRaVvPk7D7w
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: findById(mtQfVin15v1A0Yg21i2Ntyrtgn4J-VSrNRZEtyXOQvLXgLRGw0Z63danGQFKoOEiaVAU05xWfH1gRaVvPk7D7w)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: findById(mtQfVin15v1A0Yg21i2Ntyrtgn4J-VSrNRZEtyXOQvLXgLRGw0Z63danGQFKoOEiaVAU05xWfH1gRaVvPk7D7w)
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

> @ekodb/ekodb-client@0.15.1 prepare
> npm run build


> @ekodb/ekodb-client@0.15.1 build
> tsc


up to date, audited 52 packages in 1s

17 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.15.1 build
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
  id: 'QWM_7kSU1vRf-8zK8rkj9U5OBW2tYtZAmtbzv3Z2Ruc4dV_5ldtDvw2J4ur8XKzFy4ipUCCo0HE3zc0SdGOnkQ'
}

=== Find by ID ===
Found: {
  id: 'QWM_7kSU1vRf-8zK8rkj9U5OBW2tYtZAmtbzv3Z2Ruc4dV_5ldtDvw2J4ur8XKzFy4ipUCCo0HE3zc0SdGOnkQ',
  active: { type: 'Boolean', value: true },
  name: { value: 'Test Record', type: 'String' },
  value: { type: 'Integer', value: 42 }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  id: 'QWM_7kSU1vRf-8zK8rkj9U5OBW2tYtZAmtbzv3Z2Ruc4dV_5ldtDvw2J4ur8XKzFy4ipUCCo0HE3zc0SdGOnkQ',
  active: { type: 'Boolean', value: true },
  value: { value: 100, type: 'Integer' },
  name: { type: 'String', value: 'Updated Record' }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: iolMaYKywrOq70yczt_SkVsmuYpGSuO60YJ3huK20Dx1XDdlrPKNOqp_CinLajRgv0-iBhYnOO7IxBDYVIi3Vw

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
Collection created with first record: sBcCQkbXktH40B0yY9EhaymlEqFKmakaKJu4sMMGtbJNnwzEiUog7dntyQWJuhk4qOCCssrYDGtN1VA-6FzoOQ

=== List Collections ===
Total collections: 25
Sample collections: schema_documents_client_ts,schema_products_client_ts,chat_configurations__ek0_testing,ws_ttl_test,scripts__ek0_testing

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
Total keys in store: 17

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: false

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: CkgvqA6qoXNdwcAgerD7Q6KhpfETG0nGT4sUq-XQYKjWKUP4N762yE-XW1CoyB9ajju312gT_0IdCTeUEU13iw
Created Bob: $500 - ID: 45qBkUHq3wII_NDLDEzY2I76cpZqsCl2ULJGJ43isFs2AgdPQnooHoSUf0eA5WLdEDsXr-1GlY3Br5_Fo-M73g

=== Example 1: Begin Transaction ===
Transaction ID: 6f706766-c472-4f8d-bc9b-427db8005190

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 5fcc1905-db75-4b0e-8331-aaa7a52a8295
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
  1. Score: 25.740, Matched: name, name.value, email.value, email
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title, bio, title.value, bio.value
  2. Score: 26.400, Matched: bio.value, title, title.value, bio
  3. Score: 26.400, Matched: bio.value, bio, title, title.value
  4. Score: 26.400, Matched: title, bio, bio.value, title.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title, title.value, bio, bio.value
  2. Score: 39.600, Matched: bio.value, bio, title, title.value
  3. Score: 39.600, Matched: title.value, title, bio, bio.value
  4. Score: 39.600, Matched: bio, title.value, bio.value, title

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio.value, bio

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.756, Matched: 
  2. Score: 0.735, Matched: 
  3. Score: 0.729, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.702, Matched: content.value, title, title.value, content
  2. Score: 1.492, Matched: content.value, title, content, title.value
  3. Score: 0.294, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: bio, bio.value, skills, skills.value

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
✓ Inserted document: bo-DyL4YjNYrFUy_fe2LjVPRXFXdnS4sNoMQjc76uaRHMXRaFE-gmqofiiahgttwj_W2DZh1M5qOKcOp9YgNCw

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: YQgRI-HLCwLM4s_ZJPDTBzQghVRghb8OaZZLdS6prAdk1QMDmrduejgFIid0VDvWRsz4jsED5-b7GwkxCo_j4w

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
✓ Inserted document with TTL: Cjg0pvWVSUBLNtRDmH43xfnBYmMIMyYDRgL3267UHXfTHBZmtS4BxMK13LHem4VmsDdv_MUqYwdX30BsPCyr0g

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
✓ Edge cache script created: muGJidL5_OKi45rktu8FVDNNvMS1DFdyqXEfn8KaRTRrjYZ9Bi09165MB2EF9msCe4YY9_HCyGMGi4_drIwOUw

Call 1: Cache miss (fetches from API)
Response time: 42ms
Result: {
  "records": [
    {
      "value": {
        "latitude": 40.710335,
        "longitude": -73.99308,
        "current_units": {
          "time": "iso8601",
          "temperature_2m": "°C",
          "interval": "seconds"
        },
        "timezone": "GMT",
        "elevation": 32,
        "generationtime_ms": 0.016570091247558594,
        "timezone_abbreviation": "GMT",
        "utc_offset_seconds": 0,
        "current": {
          "temperature_2m": 3.5,
          "time": "2026-03-28T23:45",
          "interval": 900
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
Response time: 42ms (1x faster!)
Result: {
  "records": [
    {
      "value": {
        "latitude": 40.710335,
        "longitude": -73.99308,
        "current_units": {
          "time": "iso8601",
          "temperature_2m": "°C",
          "interval": "seconds"
        },
        "timezone": "GMT",
        "elevation": 32,
        "generationtime_ms": 0.016570091247558594,
        "timezone_abbreviation": "GMT",
        "utc_offset_seconds": 0,
        "current": {
          "temperature_2m": 3.5,
          "time": "2026-03-28T23:45",
          "interval": 900
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

✅ Script saved: MZzn90eoYDe5uCuWasyeOkVvRfrb6ahFQ63u_ld-zoIwOb6TcuejXrlKyjV_StgplMl1M2hFNmX_9Elay7rwoA
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
   {"avg_score":50,"count":5,"status":"inactive"}
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
   Name: {"value":"User 1","type":"String"}
   Department: {"type":"String","value":"engineering"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 40ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "id": 1,
    "website": "hildegard.org",
    "address": {
      "zipcode": "92998-3874",
      "city": "Gwenborough",
      "street": "Kulas Light",
      "suite": "Apt. 556",
     ...

Second call (cache hit - from cache):
   ⏱️  Duration: 40ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "id": 1,
    "website": "hildegard.org",
    "address": {
      "zipcode": "92998-3874",
      "city": "Gwenborough",
      "street": "Kulas Light",
      "suite": "Apt. 556",
     ...
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

✅ Script saved: e8urAQY3iaD_-6krp8V-hu1uPLVCs2aXh1htmYIprBX-8ptkFhW061pumMYdyVMiV5NeKdVKoQZyXN9YopTMNw
📊 Found 2 product groups
   {"avg_price":474,"category":"Furniture","count":2}
   {"category":"Electronics","avg_price":575.6666666666666,"count":3}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved
📊 Found 2 categories
   {"count":2,"category":"Furniture"}
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

✅ Inserted order: cTOJ4sQdcHqesiVXrZv7KlkBr3-ssE0aehY5UpTFJpuc1uiAnkQPb_pocvDMzQmhrYGbJ7bE4-3iaggfd86OWA
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: hzcLImcwKw7dbM1Geh085hlAMgPL2MWqF4Z5xYbaAOnuwNdsSj0O-PeF1ZpndzPMY02aC2Wez_Ba2c1f3sX6Fw
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"role":"admin","userId":"user_abc"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: rfEDmMOskWNH6_MNFZqupguS1LerlTg8RCAtJNKsOTVNH4YIFmLBiiXoyjP0f44He3IpruV74AYkoHyg7C9qlw
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Script

✅ Script saved: 7uRRL8uf_AF_VQ0ATIQl0mgdOgXsg-A8Q7KnZb6TNWdSP2ygf2WIblSiZ2VJ1VFs-nX18DELU8GYYTrAc5nBrA
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
   admin: 20.0 (3 users)
   user: 70.0 (7 users)
⏱️  Execution time: 0ms

📝 Example 4: Top Users by Score

✅ Script saved
📊 Users (showing first 5 of 10):
   1. User 2 - Score: 20
   2. User 10 - Score: 100
   3. User 1 - Score: 10
   4. User 7 - Score: 70
   5. User 9 - Score: 90
⏱️  Execution time: 0ms

📝 Example 5: User Summary Statistics

✅ Script saved
📊 User summary (2 groups):
   active users: 7 users, Total Score: 370
   inactive users: 3 users, Total Score: 180
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
   3. Natural Language Processing (AI)
   4. Getting Started with ekoDB (Database)
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
   1. Introduction to Machine Learning
   2. Vector Databases Explained
   3. Natural Language Processing
   4. Getting Started with ekoDB
   5. Database Design Principles
⏱️  Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Script saved
📊 Projected documents (showing first 3):
   1. Introduction to Machine Learning
   2. Vector Databases Explained
   3. Natural Language Processing
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
   Furniture:
      Products: 4 | Stock: 43 | Avg Rating: ⭐4.26
   Electronics:
      Products: 6 | Stock: 232 | Avg Rating: ⭐4.52

⏱️  Total execution time: 0ms
📈 Pipeline stages:

📝 Example 5: Project Specific Fields

✅ Script saved
📊 Product summaries (10 items, showing first 3):
   1. USB-C Cable - $19 (⭐4.3)
   2. Laptop Pro - $1299 (⭐4.8)
   3. Standing Desk - $599 (⭐4.7)
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
   1. High Precision: Vector databases store data through points, lines, and polygons which allows accurate representation of geographical features.

2. Scale Independence: They can be zoomed at any level without losing the quality of data, which is not the case for raster databases.

3. Topology Support: The data in vector databases preserves spatial relationships, such as connectivity and adjacency, for geographic features.

4. Space Efficient: They store only the coordinates of the features hence occupy less storage space compared to raster databases for the same geographical coverage.

5. Rich Geographic Analysis: Vector data is well suited for complex spatial analysis and computations, such as shortest-path or network analysis.

6. Attribute Association: It is easier to associate attributes with vector data allowing better spatial queries and data analysis.
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embedding script saved
📊 Generated embeddings for 3 articles
   1. "Advanced Query Patterns" - 1536D vector
   2. "Getting Started with ekoDB" - 1536D vector
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
✓ Created SWR script: fetch_api_user_js (McEensJ2VpLKqAOgQigRjALyDQovTeVcIMk5PyM6wKjUgVW3Hl4ArwDoLTjPXFUszhJBSVtlLbsxbwerDBacHA)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "address": {
          "city": "Gwenborough",
          "street": "Kulas Light",
          "geo": {
            "lat": "-37.3159",
            "lng": "81.1496"
          },
          "suite": "Apt. 556",
          "zipcode": "92998-3874"
        },
        "email": "Sincere@april.biz",
        "name": "Leanne Graham",
        "phone": "1-770-736-8031 x56442",
        "website": "hildegard.org",
        "username": "Bret",
        "company": {
          "bs": "harness real-time e-markets",
          "catchPhrase": "Multi-layered client-server neural-net",
          "name": "Romaguera-Crona"
        },
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
✓ Created native SWR script: github_user_native (71BzLnMGDCi8PvNkrV4s2ZAj5LUjawmA_wwSclA8RJLuEMuORle5d90eE79dX9oS2I9GUenzz_sv0nS1nWd3rQ)

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
✓ Created SWR script with audit trail: product_swr_audit (DaL-MlfjrumJ_22l0_u2mzavHO1AEAsuF3nokUfnKe9pgTKQZV2wQrvYg6aopgB_0Lc7Nw9l67UpAqcCvZIxXQ)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (JXIQe8FolRtV6ySLUTVhNkk30X6U7gFvi4defGZQvXVNkyvrEf7oI_JTiunAL7_rmUUGufsxv6HNGY3stasYMQ)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (naWmVDRr11C4tjxtQmHd-8op943RKkvvgnDjxtaUVTDlb6egrjvEWkWdUbyFEG3VDN3HU6EFtorl--zFugMpmg)
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
✓ Created session: 1vdGO88s69jfwoMV9EVC6epMu5_AM_cEYfxY96zVmExzRO0-t5CvAnd-qmF_jZLGSC4BPTNB-vnAZbzFzh2vbg

=== Sending Chat Message ===
Message ID: NWR_gOj4Z3AojMghVmFUsnN2gZ-MNqJ8GTRWmvHeNrg-jGKu0zVtSuTVd2xK5qXHRerMgH2Iz5oWAXCMyVfFbw

=== AI Response ===
Here are the available products along with their prices:

1. **ekoDB Cloud**
   - Price: $499
   - Description: Fully managed cloud database service product.

2. **ekoDB Pro**
   - Price: $299
   - Description: Enterprise edition product with advanced features.

3. **ekoDB**
   - Price: $99
   - Description: A high-performance database product with AI capabilities.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_js',
  record: {
    id: 'KQcRIwAec8Pp0qbZ2AW25Qcr9PBruWFw3jRz-yJuev2X6iMRd39dEt3GKYt7GPKp1hKTqOA5xtzuh4TOxT-H9g',
    description: 'Fully managed cloud database service product',
    price: 499,
    name: 'ekoDB Cloud'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_js',
  record: {
    id: 'SbauXn4R_CGyXDIQ82SHjbxHvg00e5T9ZiLIy_VIIxB8Gu7Gh1YEA-4Za_r19POVWsIGTl9ZDlFmPzM-1U53nQ',
    description: 'Enterprise edition product with advanced features',
    name: 'ekoDB Pro',
    price: 299
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_js',
  record: {
    id: 'bfflr7cgU9S37HxXOPvpfMKpImc0Sq7JUrwOWB6qJoc7LFtcHEkFgo4PS-zG2wwm4_ngSeoY8NF4dSMrGm0qNw',
    name: 'ekoDB',
    price: 99,
    description: 'A high-performance database product with AI capabilities'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 2447ms

=== Token Usage ===
Prompt tokens: 1298
Completion tokens: 91
Total tokens: 1389

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: M5CIJEsiTBu9Wq68305hMbYvwT6VzOeWEMVe3bPPDrzKX65scQR-Xyl3vPjYKesLT_hRCWCp3UGKerB2Sd7K3w

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: High-performance database product
- **Price**: $99

If you need more information or want to explore other products, let me know!

✓ Second message sent

=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of ekoDB is $99. If you have any other questions or need further assistance, feel free to ask!

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: RIlDdF6Ajxurie3mTW0MCvjPqSV899FsfZTHKvbM-vvSlsVy0jxzaZD-1MR9716KB6SjXZHJ0HbcZgnvCd0KiA
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
✓ Created session: smzWzGqW6CfHOIK0YAga94tYAUrFq29lKFdcNxi7xnw7EOe-j3GckuddzzV_xTBzvUrRzkVolHjZ7hVniBY9_g

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: A high-performance database product
- **Price**: $99

If you need information on more products or have further questions, let me know!

✓ Message 2 sent
  Response: The price of **ekoDB** is **$99**.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: UGhI6kT5jFNiaey-yCo_K-yA715YeqHquSJ5QYqzRuIoELF2ybHg1NS2Ysj7b963CKV-oNUsxaLqNIBd7uXc9w
  Parent: smzWzGqW6CfHOIK0YAga94tYAUrFq29lKFdcNxi7xnw7EOe-j3GckuddzzV_xTBzvUrRzkVolHjZ7hVniBY9_g

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: UGhI6kT5jFNiaey-yCo_K-yA715YeqHquSJ5QYqzRuIoELF2ybHg1NS2Ysj7b963CKV-oNUsxaLqNIBd7uXc9w (Untitled)
  Session 2: smzWzGqW6CfHOIK0YAga94tYAUrFq29lKFdcNxi7xnw7EOe-j3GckuddzzV_xTBzvUrRzkVolHjZ7hVniBY9_g (Untitled)
  Session 3: RIlDdF6Ajxurie3mTW0MCvjPqSV899FsfZTHKvbM-vvSlsVy0jxzaZD-1MR9716KB6SjXZHJ0HbcZgnvCd0KiA (Untitled)
  Session 4: 1vdGO88s69jfwoMV9EVC6epMu5_AM_cEYfxY96zVmExzRO0-t5CvAnd-qmF_jZLGSC4BPTNB-vnAZbzFzh2vbg (Untitled)
  Session 5: 7DzhItH6Yj6JtcBJvapGuPhG11MJ_C-dyH_Rscif0fUE5ZQ5TahmcWWnAPwXWKeRZtzHDvouVu2s27xNRYKIWA (Untitled)
  Session 6: eLCZB-7_ZJTmrc8Qd8pEq_5M9a2M0OB-ythxrdBQj4m2_zO_0SYQgy8YHaDGFqysaY2Gk1BZ9ZCUv8wpVAGnig (Untitled)
  Session 7: tBdAM8UVRfWjR-8yVcnRq0e-bBpTeEqgdJI23IdFSQlVpuutnXMeIJ5seAKBOmFSevre9FF1vn7iSui3CY0-dA (Untitled)
  Session 8: b2GLulGwg3Lo3R1RniwXyBoFHoTgBniyXUqPk9Br72wCTWPjZMc5Uj83W3mC-Kve6hL_pEaV-_07Ul9mdYKLMQ (Untitled)
  Session 9: 1DKL_oeh4pxE-pQex_XVsXEX71hWlyhbI0SLwhPUot4_QRG0ga4H5XJJWkezKWNg1SrwpXKK3H8GhONB65JNaQ (Untitled)
  Session 10: zseqPJi_FKBvWhcszt4lzHwF2aO_Zdq5_3yK9_zLocfnKymTh48UuUu-whYP4Hua5y6KoPXDmXBOkXpwWCjc7A (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: UGhI6kT5jFNiaey-yCo_K-yA715YeqHquSJ5QYqzRuIoELF2ybHg1NS2Ysj7b963CKV-oNUsxaLqNIBd7uXc9w

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
✅ [32mJavaScript client examples complete![0m
