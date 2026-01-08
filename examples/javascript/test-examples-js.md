make test-examples-javascript
🧪 [36mRunning JavaScript examples (direct HTTP/WebSocket)...[0m

added 1 package, removed 1 package, and audited 9 packages in 852ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
[34m
╔════════════════════════════════════════╗[0m
[34m║  ekoDB JavaScript Examples Test Suite ║[0m
[34m╚════════════════════════════════════════╝[0m
[34m
=== Checking Server Connection ===[0m
(node:63304) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
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
  id: '3WFwJB9wpYFBUSrbmIbW17tnY7-Ly7vavZhbK4rsnAYGoBTL1AptxCgZ3LhXw4OhsrnfJJp7J9mNc_tuSBCqhA'
}

=== Find by ID ===
Found: {
  active: { type: 'Boolean', value: true },
  value: { value: 42, type: 'Integer' },
  id: '3WFwJB9wpYFBUSrbmIbW17tnY7-Ly7vavZhbK4rsnAYGoBTL1AptxCgZ3LhXw4OhsrnfJJp7J9mNc_tuSBCqhA',
  name: { value: 'Test Record', type: 'String' }
}

=== Find with Query ===
Found documents: [
  {
    id: '3WFwJB9wpYFBUSrbmIbW17tnY7-Ly7vavZhbK4rsnAYGoBTL1AptxCgZ3LhXw4OhsrnfJJp7J9mNc_tuSBCqhA',
    name: { type: 'String', value: 'Test Record' },
    active: { type: 'Boolean', value: true },
    value: { type: 'Integer', value: 42 }
  }
]

=== Update Document ===
Updated: {
  active: { value: true, type: 'Boolean' },
  name: { type: 'String', value: 'Updated Record' },
  value: { type: 'Integer', value: 100 },
  id: '3WFwJB9wpYFBUSrbmIbW17tnY7-Ly7vavZhbK4rsnAYGoBTL1AptxCgZ3LhXw4OhsrnfJJp7J9mNc_tuSBCqhA'
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
✓ Inserted test record: PPq435-cvMCbvNQ1dhkHcLfXO4pnoqDb6iZZbEah_XyIcxozmPQWJtPI01yeBbx7vapGjS0ZHR-AigjR8mNPZw

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
        "id": "YOrVGcy8f4jZ6600r2cgnHQI7bIj5GDda2kwQvKJfZl8UXJxG3jujZoY5ZGGQ9p_QWUhqIIav26PpAwBa1VTkg",
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
        "id": "PPq435-cvMCbvNQ1dhkHcLfXO4pnoqDb6iZZbEah_XyIcxozmPQWJtPI01yeBbx7vapGjS0ZHR-AigjR8mNPZw",
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
        "id": "YGFk_EvfaQlmIM2mRRLCf9Zt4kvlMzFYdrJHI0wrqfeoEf-hsCHJTXKuQDPjUyQoTI5uunzX9I5kMuBuDRiqhw",
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
        "id": "-RTb0MTlOEYMxwx5ED4A86uEq3B-1r56lRhmGGACz1w9b8fIe3oIuGQw-xCjK3VvTeNKqWdh9N0DIp1tdFwa1A",
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

✅ Script saved: CIa8H_kn38WBljurzOmuNEZTVKz0DFiTDt1omWb20ZUdzMKC-oIkcuNAhGzTSQUSC6ANv1uZxur6mreaVkamhQ
📊 Found 10 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: zhfUAPn5neZEfR9eEih7YH-OAuzGqsLHF_W4CKyScF1vpW4T5rm906u_g60IxMF-yr_W2aIvGP1o2LfFed7cMw
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: ZJpiH-gwdBluosYKOajoUeH3TXgV_liZhGfwmi8JiqQEakqBPCmGuIeuaRw61xsqhqS-FngHImukvGzfkYPOcQ
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"status":"active","avg_score":60,"max_score":100,"count":10}
   {"max_score":90,"avg_score":50,"status":"inactive","count":10}

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
Retrieved value: { userId: 123, username: 'john_doe' }

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
Collection created with first record: IhWS7CWxjD2GkcKS1OLxG-lQPokGnA9rrOS70cWHQlzfWCRZDFsFyBuNvrsI53jOzcgTUoBJY0btVVoUyWyjbw

=== List Collections ===
Total collections: 24
Sample collections: [
  'websocket_test',
  'demo_collection',
  'chat_configurations__ek0_testing',
  'test_accounts',
  'test_collection'
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
Created Alice: $1000 - ID: Ddbh6m63C24fF7OQHUyUdsFitTKmFFQGVBR7DxD6wRN-Xt7gEG-j5oFnKWf2Zd8OtupWwfADXuNA4PYDxPOl5A
Created Bob: $500 - ID: XqN0H-rXOL1NDmzLk-_KwUDHAQmkrCRJdIfD4zlv7UWO6kmcCIzjr_KvXGvf8lCoVzjapKV3IccrJJ_78oOOZA

=== Example 1: Begin Transaction ===
Transaction ID: d7518aa3-0de6-4d20-891c-1b114e6268ad

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
New transaction: 73f377d7-a63f-427d-834a-14fbf7ba71d2
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
   ✅ Script saved: drowZKoo2T1vLBrFLgwl6ChNn6h59f_SX5VyHRXlDgIaxL0QI2nluYctEbcgAshwJag3GPu1F8Jl0Bxcbb0fDg

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
   ✅ Script saved: bZe8a39UndrZ44G5aBxAO873gB5xk-AVVIKvqHpNHPqdtj-AdFVV5A-ndeoauI5_4nqp_PtHon-IplxkNqVcJA

2️⃣ Calling Script (Query + Update + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Status updated to: {"type":"String","value":"active"}
   📋 Name: {"type":"String","value":"Alice Smith"}

============================================================
📝 Script 3: Query + Update Credits + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: s2knjwujJt_6z1MCEuhPK8VnXTfUzQrhF5YCL3wTh4S_y5F7rheeyeXtZV6g9L72FsYCfsCpFULspGPy4lwRDQ

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
   ✅ Script saved: ivAcqfkdnoR1-7YgvNiz1BWIqqwJYWVaf1K3rWQaNYIlUgGLqFp5xEowMrFWSAqsexWsw0sIvxBACkQUMkkh_w

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: drowZKoo2T1vLBrFLgwl...
   ✅ Deleted script: bZe8a39UndrZ44G5aBxA...
   ✅ Deleted script: s2knjwujJt_6z1MCEuhP...
   ✅ Deleted script: ivAcqfkdnoR1-7YgvNiz...
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
  Output: Document ID = jC3IXdNOBIHwCWIvkH0y1MYuzi7fKLGejZauQYl6ij5Ia0CTafmyKpw3XAX1hPgmJqrdIaVnJ3TprK5qeZqXfg
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: findById(jC3IXdNOBIHwCWIvkH0y1MYuzi7fKLGejZauQYl6ij5Ia0CTafmyKpw3XAX1hPgmJqrdIaVnJ3TprK5qeZqXfg)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: findById(jC3IXdNOBIHwCWIvkH0y1MYuzi7fKLGejZauQYl6ij5Ia0CTafmyKpw3XAX1hPgmJqrdIaVnJ3TprK5qeZqXfg)
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
  Output: Document ID = AAl-QlszMhb7FyM40OT67rt8F7f7YTDUqOHW4zkZAvE-84thrsvLnrgny-w49BXv24PD-g4cwngbFVsXxk8Q7g
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: findById(AAl-QlszMhb7FyM40OT67rt8F7f7YTDUqOHW4zkZAvE-84thrsvLnrgny-w49BXv24PD-g4cwngbFVsXxk8Q7g)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: findById(AAl-QlszMhb7FyM40OT67rt8F7f7YTDUqOHW4zkZAvE-84thrsvLnrgny-w49BXv24PD-g4cwngbFVsXxk8Q7g)
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

added 1 package, removed 1 package, and audited 9 packages in 836ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'UrfJBRKjIF7suGPw0PZVYESfY0EUTuSdNBFLwsIJiIM38dgUZvP9lW7CizRQ6l6MaFCoYfI5NSBynvIJqIW8kA'
}

=== Find by ID ===
Found: {
  id: 'UrfJBRKjIF7suGPw0PZVYESfY0EUTuSdNBFLwsIJiIM38dgUZvP9lW7CizRQ6l6MaFCoYfI5NSBynvIJqIW8kA',
  value: { type: 'Integer', value: 42 },
  active: { type: 'Boolean', value: true },
  name: { type: 'String', value: 'Test Record' }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  name: { type: 'String', value: 'Updated Record' },
  id: 'UrfJBRKjIF7suGPw0PZVYESfY0EUTuSdNBFLwsIJiIM38dgUZvP9lW7CizRQ6l6MaFCoYfI5NSBynvIJqIW8kA',
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
✓ Inserted test record: l-o5-0L0P8ySJrmrQPHmtbaXsw5X7PSrPBZBAiK9gAcffHyog2M8sl1R29wisIvt8UjPAOoMpq9CkRrqekIAUw

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
Collection created with first record: XG1O5h1QH_pD9Uv3c6S3Y9GvCTV5HjjwbtTKXkV3Tu-weKNDNOyheKeiUhTNdaZ2KStGTY9YXt4dp-kqK9USBw

=== List Collections ===
Total collections: 23
Sample collections: websocket_test,chat_configurations__ek0_testing,test_accounts,test_collection,schema_users_client_ts

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
cache:product:2: { name: 'Product 2', price: 39.989999999999995 }
cache:product:3: { name: 'Product 3', price: 49.989999999999995 }

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 0 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 10

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: false

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 4_uE4BMT6nDMfsvfjNZmSBqQhfOuDspmJ6ne8DJoQXMlji2awMVvh-vIzGfjJiRvPPoikpmqXpI509STe9nqgA
Created Bob: $500 - ID: Gkob2r9a6MCtZcUpcfl74ytLPhmKYJ5h1XWHSHpriXRgO9MnD3hdAAMqkmrA4Gn12JWsFR3PRuGMQw_r5QA7vg

=== Example 1: Begin Transaction ===
Transaction ID: b8c3ef5c-a7b8-4d72-925e-34d884637d2a

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 1882783c-ed37-423b-93ea-080de8a2f3ad
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
  1. Score: 25.740, Matched: name, email.value, name.value, email
  2. Score: 12.540, Matched: name.value, name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title, bio.value, bio, title.value
  2. Score: 26.400, Matched: bio, title.value, title, bio.value
  3. Score: 26.400, Matched: title, title.value, bio.value, bio
  4. Score: 26.400, Matched: bio.value, bio, title, title.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title.value, bio.value, title, bio
  2. Score: 39.600, Matched: title, title.value, bio, bio.value
  3. Score: 39.600, Matched: title.value, bio, title, bio.value
  4. Score: 39.600, Matched: title, title.value, bio.value, bio

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.757, Matched: 
  2. Score: 0.748, Matched: 
  3. Score: 0.736, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.699, Matched: title, content.value, title.value, content
  2. Score: 1.494, Matched: title, title.value, content.value, content
  3. Score: 0.303, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: bio.value, skills.value, bio, skills

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
✓ Inserted document: Yw-ETQu43fY33D-Pk_13GkzEN9vMt4SQNrxguK-d4psAU97auDZCT-pl6gXofZDVdyWNsWNYmQEdfNxdWQEo5Q

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: FbeiGQZAG7kpjJ30t70cX-rYBBEEjY4cO_BNajvk4BGXyJZ0QDqke4YWM5fm45UdSKPx2yoRqd33JqCqN-RLIQ

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
✓ Inserted document with TTL: raTfpggXgA4Yr1wy8tYegVMMvbWSN-cRGKm8stx98ttUpqGZdA9ukT34DV7rZ0BqRTpIAHu-WbjHWZAgrj_YKA

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
✓ Edge cache script created: szYwcbwNI6BwJLVPYQghEvs1SzHjFFSPjbP8NvsfDlmkvN1ejPVFLvTOh3ELM-G_XBN1qpfwqArHbCAgZSnYIw

Call 1: Cache miss (fetches from API)
Response time: 5ms
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
Response time: 5ms (1x faster!)
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

✅ Script saved: RHLhFajsdC5uCLn1OGPgD-3h0MNI4zW4Jn0pxIjJk1Zg6Dx0KYhKjE--hTOWli7_0nXETpEuE7P68ditmO2SAQ
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
   {"status":"inactive","avg_score":50,"count":5}
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

Using CallFunction to replace inline logic in SWR pattern...

✅ Saved reusable function: fetch_and_store_github
✅ Saved SWR function using composition: swr_github_user

First call (cache miss - will fetch from GitHub):
   ⏱️  Duration: 158ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 83ms
   📊 Records: 1
   🚀 Cache speedup: 1.9x faster!

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

✅ Script saved: gARmMa96eypymZ6DfcawY4ZPPp6WNt241qQJX17nuqxZtWVTEv-ZRsqN9W4pbgSY7-XLuA_Zkf2cwO9uYdmRrQ
📊 Found 2 product groups
   {"avg_price":575.6666666666666,"count":3,"category":"Electronics"}
   {"avg_price":474,"count":2,"category":"Furniture"}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved
📊 Found 2 categories
   {"category":"Electronics","count":3}
   {"count":2,"category":"Furniture"}
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

✅ Inserted order: ydcXc2HUTiUkFyYPqC_nl8sxfkwHWlY2y-aZS-qZU4Lt-dfUvQ3cAV6D7wew7IZPwqJoMMEyQn1wPxKGmS9ikQ
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: DoLlNsTL317SvcWcvqwduXaCQN4j3GGTherlABjtB18Capz6uEjFwInIegASBK5Dje6SH0M4asmlOYb1ou18Jw
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"role":"admin","userId":"user_abc"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: dQR2i7vqNiwm00fNJ872OjwbTwaeeKwvFAiIzCMUFSjA-dyPhl-DLyY6lt47We5cQqeKbgqjZpbuozk1SiL5eg
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Script

✅ Script saved: 6dblk3zPaPPRi7x4M5PrIljqtmaR6NMtNr1QdE8-mZshNz6U4mwL72Uesf9UOscPe1UlseNiF4GJui6YdBdiBQ
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
📊 Users (showing first 5 of 10):
   1. User 2 - Score: 20
   2. User 3 - Score: 30
   3. User 7 - Score: 70
   4. User 1 - Score: 10
   5. User 9 - Score: 90
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
   1. Database Design Principles (Database)
   2. Natural Language Processing (AI)
   3. Vector Databases Explained (Database)
   4. Introduction to Machine Learning (AI)
   5. Getting Started with ekoDB (Database)
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
   1. Database Design Principles
   2. Natural Language Processing
   3. Vector Databases Explained
   4. Introduction to Machine Learning
   5. Getting Started with ekoDB
⏱️  Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Script saved
📊 Projected documents (showing first 3):
   1. Database Design Principles
   2. Natural Language Processing
   3. Vector Databases Explained
⏱️  Execution time: 0ms

📝 Example 5: All Document Fields

✅ Script saved
📊 All documents (5 total, showing first 2):
   1. Database Design Principles (Database)
   2. Natural Language Processing (AI)
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
   Electronics:
      Products: 6 | Stock: 232 | Avg Rating: ⭐4.52
   Furniture:
      Products: 4 | Stock: 43 | Avg Rating: ⭐4.26

⏱️  Total execution time: 0ms
📈 Pipeline stages:

📝 Example 5: Project Specific Fields

✅ Script saved
📊 Product summaries (10 items, showing first 3):
   1. Desk Chair - $349 (⭐4.2)
   2. Bookshelf - $149 (⭐4.1)
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
   Vector databases offer several benefits:

1. High Precision: They provide precise representation of geographic features and allow complex geometrical operations.

2. Flexibility: They are flexible in storing and manipulating different types of geometric data types like points, lines, and polygons.

3. Scalability: They handle large amounts of data efficiently.

4. Spatial Relationships: They allow for the analysis and interpretation of spatial relationships between features.

5. Topology: They support topological information, which is useful for network analysis and proximity analysis.

6. High-quality output: They enable high-quality cartographic output as they can preserve the exact shape of geographic features.
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embedding script saved
📊 Generated embeddings for 3 articles
   1. "Draft Article" - 1536D vector
   2. "Advanced Query Patterns" - 1536D vector
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
✓ Created SWR script: fetch_github_user_js (Dj7tJGCGEpPceAj5E5oqK6nRIi-hyeUthNMwM-aC7aRBvk6Da_lHtlf6e4Zhv2A1sspzcS538FM8C5CfnGtoJQ)

Step 2: First call - Cache miss, fetches from GitHub API
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
✓ Created session: XzilMQuV_wL3dFgelEIXtxWc4-xypAJ-GXeWN_11vWTA6cgTrH1tR1mAFt9g2XJVqVdaW_9x9Ydy7oy0GO1bIw

=== Sending Chat Message ===
Message ID: mvXT1KMaVLH5JaxCoNeddBh05D5KJol8-nY_Cnw0wn2EA5Y6T4MgtJuc_iqoXclezDrU40rxaRe7UaFwepNMgg

=== AI Response ===
There are three products available:

1. ekoDB Cloud: This is a fully managed cloud database service product. The price for this product is $499.

2. ekoDB: This is a high-performance database product with AI capabilities. The price for this product is $99.

3. ekoDB Pro: This is the enterprise edition product with advanced features. The price for this product is $299.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_js',
  record: {
    price: 499,
    description: 'Fully managed cloud database service product',
    id: 'ap3_81Zj74fSyicwxeq-PjKVMB0eOK-Cvic_ZFbjsXrVPs8w-qFvpukKb_dl2CaJzZxKel4F28YNq5bo4SSftg',
    name: 'ekoDB Cloud'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_js',
  record: {
    description: 'A high-performance database product with AI capabilities',
    id: 'ma0rWdoXMNJzuH0yZrYeukwZUumIORUiXoTH6EyNDX9YQrtO8vQWOCNF6td1ZjAPBHnOEBjDM6qXx-CPS3QxCg',
    price: 99,
    name: 'ekoDB'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_js',
  record: {
    description: 'Enterprise edition product with advanced features',
    id: '-jRi07yEmi-55PlFrbTWunYA1Q9ukuaRjeqjpgQc_6BiDv8scfwUDmgUWTbDO5TnoTx6yNciy7XNHZmlwQdn9w',
    name: 'ekoDB Pro',
    price: 299
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 2431ms

=== Token Usage ===
Prompt tokens: 444
Completion tokens: 84
Total tokens: 528

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: B9_Yx4gO7D4w7Tfha9UZClam1oqQlzSXD29QUVRqbJuNdLjspfSTf_07xhI3IG0hIx3vi3g9jOtCk342PDBfzA

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the provided context, there is a high-performance database product available named "ekoDB". It is priced at $99.

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
✓ Created second session: 28l7fpETAKKKj7k60S0h7CAAdMuqRZPPSUwMDIN2qVOAzUzRhcewHsqbrS3Pdz696JiZ1GTHOKs-HzmKXlEE7A
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
✓ Created session: _hsFsdrCUlOE3NmyfnmpR58QACv6Z4VLjSQ6lx3EH_FAX6RUTFslEcd3LSospUCiBYeuTNqsn_Y6A3Vc-kAbmg

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the provided context, there is a high-performance database product available named "ekoDB". It is priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: RM3NVSkb27tKIXjlTRiBL5ueuL4GxmdNfsIeCnY0YdbFf57bC_5q9b702daA1n-g3vG8-z8qTUQ_h65S-VmfHg
  Parent: _hsFsdrCUlOE3NmyfnmpR58QACv6Z4VLjSQ6lx3EH_FAX6RUTFslEcd3LSospUCiBYeuTNqsn_Y6A3Vc-kAbmg

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: RM3NVSkb27tKIXjlTRiBL5ueuL4GxmdNfsIeCnY0YdbFf57bC_5q9b702daA1n-g3vG8-z8qTUQ_h65S-VmfHg (Untitled)
  Session 2: _hsFsdrCUlOE3NmyfnmpR58QACv6Z4VLjSQ6lx3EH_FAX6RUTFslEcd3LSospUCiBYeuTNqsn_Y6A3Vc-kAbmg (Untitled)
  Session 3: 28l7fpETAKKKj7k60S0h7CAAdMuqRZPPSUwMDIN2qVOAzUzRhcewHsqbrS3Pdz696JiZ1GTHOKs-HzmKXlEE7A (Untitled)
  Session 4: XzilMQuV_wL3dFgelEIXtxWc4-xypAJ-GXeWN_11vWTA6cgTrH1tR1mAFt9g2XJVqVdaW_9x9Ydy7oy0GO1bIw (Untitled)
  Session 5: kzV0MNU7tpfkIQcxs153-OpdN_qimgUG662sOfSOlDprVZd8I7AjzRXf-eiT386Jxxu1SDLwVtjUQjKx_hiqdA (Untitled)
  Session 6: OJ9Gi-vngn4KrARAppWTtZ7ti6k5KXV6-QKU4Ac8wIAMiVB4OvmEul0OPmrl80jf6Rb2M6A10R06MDIUuPdv7w (Untitled)
  Session 7: OZ_T9jx1dAT8W36cWXYVS3t3jWv-dFVb9yE-5VMaU_Lc8pT3j8qiRUOgFqiQD7HZHCB--uMQ7mO0Pq3qkwO5rA (Untitled)
  Session 8: obmigcGQx2Qj83fWYWW49S8iF05biY_bfytJuESASiYJthxdjNJujJn5OuEjpD_Uqht3SV9bogTZN1Zu-HXTHg (Untitled)
  Session 9: l8mfhsis2iPfO0NlBWcX4WM_Pvyc0UaawYXDVAH4BZDg11Yd5hxY3iAx30wwnGhCxHUWxO6gdzuHI0g6scE3YQ (Untitled)
  Session 10: h6e74e5rz97_0LQS0bX5e6STHfC-5AUx81xH2YClP806irvsP_RuIbiX5-AwnTwRbNCOQqkRwx6A4-PeLBDrcg (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: RM3NVSkb27tKIXjlTRiBL5ueuL4GxmdNfsIeCnY0YdbFf57bC_5q9b702daA1n-g3vG8-z8qTUQ_h65S-VmfHg

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
✅ [32mJavaScript client examples complete![0m
