make test-examples-javascript
🧪 [36mRunning JavaScript examples (direct HTTP/WebSocket)...[0m

added 1 package, removed 1 package, and audited 9 packages in 629ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
[34m
╔════════════════════════════════════════╗[0m
[34m║  ekoDB JavaScript Examples Test Suite ║[0m
[34m╚════════════════════════════════════════╝[0m
[34m
=== Checking Server Connection ===[0m
(node:40357) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
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
(node:40360) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
Inserted: {
  id: '7o2uUhZL5-bmHrfmnsKQOfBGp7EttkhG0fwxjO6B6v2oUDe4LVRlN43w8RE7XhBH0ijs9TdLLVnaCMga-YHUYw'
}

=== Find by ID ===
Found: {
  id: '7o2uUhZL5-bmHrfmnsKQOfBGp7EttkhG0fwxjO6B6v2oUDe4LVRlN43w8RE7XhBH0ijs9TdLLVnaCMga-YHUYw',
  active: { value: true, type: 'Boolean' },
  name: { value: 'Test Record', type: 'String' },
  value: { value: 42, type: 'Integer' }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  name: { value: 'Updated Record', type: 'String' },
  id: '7o2uUhZL5-bmHrfmnsKQOfBGp7EttkhG0fwxjO6B6v2oUDe4LVRlN43w8RE7XhBH0ijs9TdLLVnaCMga-YHUYw',
  active: { type: 'Boolean', value: true },
  value: { value: 100, type: 'Integer' }
}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
[32m✓ simple_crud.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/simple_websocket.js ===[0m
✓ Authentication successful

=== Inserting Test Data ===
(node:40382) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
✓ Inserted test record: oSyC2C2D8L2hZE9j_Ho030C0ubOVbSDAbXEnU-y-WVhshprVbA1mPP-zl3KwtqEf3sq7lgksYClPBMyC-Wjm6w

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
        "id": "uNNfPR9zCQ_Wj1E21yvsz6eduv194YhIWwL8E2fBMxqSqmMVPCRQjSoBSn6ipWjaTp5S-QAoqOl_dwGFXKUdCw",
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
        "id": "Kd7MuNthn11mq4fPbeELKVfNy-FciWdlj1aoIA441hvPMlKU0ZCKc0mBikFpaf1fbEYf6jXXevgjFsdeggqXNQ",
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
        "id": "oSyC2C2D8L2hZE9j_Ho030C0ubOVbSDAbXEnU-y-WVhshprVbA1mPP-zl3KwtqEf3sq7lgksYClPBMyC-Wjm6w",
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
        "id": "rOrZMWqDh7WWVTLDxIikzWv8TgnmEIw4VFgPZB6WZt6mnTipX67B6IVw3iRtb_yijeZztQjufYQyn0AN2m3wDQ",
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
(node:40398) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
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
(node:40411) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
✓ Set key: session:user123

=== KV Get ===
Retrieved value: {
  loginTime: '2025-12-28T23:18:36.082Z',
  userId: 123,
  username: 'john_doe'
}

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
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/collection_management.js ===[0m
✓ Authentication successful

=== Create Collection (via insert) ===
(node:40418) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
Collection created with first record: uBBeQZKUfxhEo4KXFgTnRRgcMFoFTi-lcUnv_Y3nH3gknJiQLb5Pgs9pWqEJo2dl-J4Az4VQo2iaDtUILfr1qA

=== List Collections ===
Total collections: 23
Sample collections: [
  'schema_employees_client_py',
  'batch_users',
  'schema_products_client_py',
  'test_collection',
  'schema_documents_client_ts'
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
(node:40422) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
✓ Inserted document: xfGbx_SLsXqWJvQ4qzQBWvAAF9hzgXQ6VEmKHpKW5Y9jXkq_pFEz8eFlg8zTKu9DCDdczfVmAk1jEMyBGgJ4Qg

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
(node:40437) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
✓ Inserted document with TTL: dbOBgMKN483WF3fzpDI0rCrIgICl5ecghsjXp87C0vPDG9AMisVAsDtzBrq5jk1eTcjvjUGkSwouA4gHzpYg2w

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

✅ Script saved: Y9zsBjOJ-t0D5S2ossC6bXE3lLawKJD5K2qJteBIexu8fD6aYpw9_-cpnuVdKZzsTGn4KMRMrPxHcARqYCwRbg
📊 Found 10 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: pXnadNXvQMF2CJ5Uv0Nxv5qujrq9WeVgqn58kv3hr2XZlapzOHKzWec61z_XCZRjxCPr91-XXq9TL2COJLgAKg
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: 8B0DHwPmyW4sMjbC_3mAG-iCfqAAy7hFA-efRXLqGbDS3pEVNHeIQeLUz_kJlxLcAPxxjFPy_GIB5JROE3Y1Lw
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"status":"inactive","count":10,"avg_score":50,"max_score":90}
   {"avg_score":60,"status":"active","count":10,"max_score":100}

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
Created Alice: $1000 - ID: E38LWi88fGe2V8pCX1dL57efqfeo42u_Gtid9phym6asmsanAXuYLMCK-pMQ-wDm43PKDq9Sz6ekHkVp2GTIgA
Created Bob: $500 - ID: JahHKpprr7MdOuA7amqZ1JuwEbXAzB9_m824PI5Ac-v2oALtOhZkSoALDItS12W2XBcQG9MgIsnXXKUL1YioCA

=== Example 1: Begin Transaction ===
Transaction ID: 2fc03ec3-f837-4023-a3af-d5c1b5029d18

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
New transaction: fe05f818-2ece-42ed-b2ae-55fa762781bf
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
   ✅ Script saved: wR-Pj1lSODT9IFD7RTg2u7668YjfYhg2AGYLRCldLLHVwDFQBBQIW6mFusDErxsq9PGh2kFtJnJOPxmxh6eUzA

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
   ✅ Script saved: mxLQwvrmKOwnvi8z21qOhqCiTXvg1ntEBsMDTkMC_rwCWobB_AzF6a68fuVU4J5CHJkCyURT_lvC9gb2cpaJUg

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
   ✅ Script saved: THsSDujnA8XZz0FbDorYsFyriZdXMrWZPou0phYqYQvyKzEl6VARhJUjagm0iVnQu7B4zh1X259JwdzcZE2Nig

2️⃣ Calling Script (Query + Update Credits + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {"type":"Integer","value":0}
   📋 Status: {"value":"active","type":"String"}
   📋 Name: {"type":"String","value":"Alice Smith"}

============================================================
📝 Script 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: HvGweZMHpvcvpko7NU6_2tZwH8Or_OQdkVGY9H0lb0vL5pF4hSrHHkA8EsH5Qu8yQC1TMjYUfIinBybyEhhFew

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: wR-Pj1lSODT9IFD7RTg2...
   ✅ Deleted script: mxLQwvrmKOwnvi8z21qO...
   ✅ Deleted script: THsSDujnA8XZz0FbDorY...
   ✅ Deleted script: HvGweZMHpvcvpko7NU6_...
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


up to date, audited 7 packages in 849ms

found 0 vulnerabilities

> @ekodb/ekodb-client@0.5.0 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning JavaScript client library examples...[0m

added 1 package, removed 1 package, and audited 9 packages in 669ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'JTmdZ2iMz3AOHgnXyHG1yvkrkQzfpeO1C0JmO-3FCLvnugB3O-rkGeQghai9LR9aoL_wbJIx8pOnBe9cD5iSqg'
}

=== Find by ID ===
Found: {
  value: { type: 'Integer', value: 42 },
  name: { type: 'String', value: 'Test Record' },
  id: 'JTmdZ2iMz3AOHgnXyHG1yvkrkQzfpeO1C0JmO-3FCLvnugB3O-rkGeQghai9LR9aoL_wbJIx8pOnBe9cD5iSqg',
  active: { type: 'Boolean', value: true }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  value: { type: 'Integer', value: 100 },
  active: { type: 'Boolean', value: true },
  name: { value: 'Updated Record', type: 'String' },
  id: 'JTmdZ2iMz3AOHgnXyHG1yvkrkQzfpeO1C0JmO-3FCLvnugB3O-rkGeQghai9LR9aoL_wbJIx8pOnBe9cD5iSqg'
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
Retrieved value: { username: 'john_doe', userId: 123 }

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
Collection created with first record: E3YcJZpDiWHTUdSo7ofUkXJGJp38OjantvnWsMcZ3QAPB3Nn7kh6Gj54jYgIGZwl1BbvOUDffYv_FEXqg10-6A

=== List Collections ===
Total collections: 22
Sample collections: client_collection_management_js,schema_employees_client_py,batch_users,schema_products_client_py,test_collection

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: AJzCLtqfo1LYxVfJVQ8AQonx6X9CHzHEEKAwvhVgR6Nvj3L0rFZ49ZAg4lEcHXXMBwmPkseDa6bwtZe0qTsbGA

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: 0UQrVjkCKrEc9Yc0Ew0oAEaD0GWLhH4ny6FUbCVI58qFI8kTzBjFY2mPcWemxtPUZu1l3OWzYzBjz01akIDGtA

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
✓ Inserted test record: bLNOTApLb41EeGMAs-w34aWZql0MpY_YakaMt3yo-SHGSNyugXAo1zSLT8hjQQf2Yu8-9kFjj6KlB1MUh31Otg

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
✓ Inserted document with TTL: N4-FiVGrwZULeu7ILlp35OuI5CHHZ3KVmIkXVkipu3vQd-zpiO-P0xty0Gf2z9FWk358EixgulMkFcTuKFdUfA

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

✅ Script saved: rUFPt5_cCtt77CL-7zsF1pa1VJqCRotKcfO9iKF1BQNQ3yItawSvFrmscNqjeoPGiRNqPLstFg9ryASBJWNwzg
📊 Found 10 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: js6mFiPDU8D5MMRjxlfEqlFWN3ydzpfvPb4Sj54_vBBa3Aiq2UqZGJbELgKRyoaSllvMq9nrBiFY_lyRPs1Ypw
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: 3gCHJWkOJMMAUOh4_zADXucd8yC93K2i3f6ZNTE-lFxf9gPn2DJhqJtEd7kJbN1pJFJyXIW4hiofdWdOoasqzA
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"count":5,"avg_score":50,"max_score":90,"status":"inactive"}
   {"count":5,"max_score":100,"avg_score":60,"status":"active"}

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

✅ Script saved: WnAu3G9pLhdhUIR5HM1MGB8H9vjNuna-8q2T1Q4y47FhzObMeQTjIY_u25RSD62lC63XXIePkFpZv6jNPp1n8w
📊 Found 20 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved
📊 Found 20 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved
📊 Statistics: 2 groups
   {"avg_score":50,"status":"inactive","count":10}
   {"count":10,"avg_score":60,"status":"active"}
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
   2. Bookshelf - $149 (⭐4.1)
   3. Wireless Mouse - $29 (⭐4.5)
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
   user: 70.0 (7 users)
   admin: 20.0 (3 users)
⏱️  Execution time: 0ms

📝 Example 4: Top Users by Score

✅ Script saved
📊 Users (showing first 5 of 10):
   1. User 10 (user10@example.com) - Score: 100
   2. User 2 (user2@example.com) - Score: 20
   3. User 5 (user5@example.com) - Score: 50
   4. User 1 (user1@example.com) - Score: 10
   5. User 7 (user7@example.com) - Score: 70
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
   1. Natural Language Processing (AI)
   2. Vector Databases Explained (Database)
   3. Introduction to Machine Learning (AI)
   4. Database Design Principles (Database)
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
   1. Natural Language Processing
   2. Vector Databases Explained
   3. Introduction to Machine Learning
   4. Database Design Principles
   5. Getting Started with ekoDB
⏱️  Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Script saved
📊 Projected documents (showing first 3):
   1. Natural Language Processing
   2. Vector Databases Explained
   3. Introduction to Machine Learning
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
🚀 ekoDB AI Scripts Example

⚠️  Note: These examples require AI API credentials (OpenAI, etc.)

📋 Setting up test data...
✅ Created 3 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
🤖 AI Response:
   Vector databases offer several benefits:

1. High Precision: Vector data is represented in points, lines, and polygons, providing a high level of detail and accuracy.

2. Scalability: Vector data can be easily scaled up or down without losing quality, which is crucial for zooming or mapping at various scales.

3. Geographical Analysis: Vector databases allow for complex geographical operations like calculating area, perimeter, or distance.

4. Data Storage: They use less storage space compared to raster databases for representing simple, sparse geographical features.

5. Integration Ability: Vector databases can easily integrate with other data types, allowing for more complex and layered GIS analysis.
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
✅ [32mJavaScript client examples complete![0m
