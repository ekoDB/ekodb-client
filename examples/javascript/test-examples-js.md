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
  id: 'hY8V7vkGiK2Fhhwto9OHtLx4nGI2JbtKJp75wuCANWXipqCEgdaFGU5zuBkBkTQhVTP7vOSz6ru0M1r5dzf8XQ'
}

=== Find by ID ===
Found: {
  value: { type: 'Integer', value: 42 },
  id: 'hY8V7vkGiK2Fhhwto9OHtLx4nGI2JbtKJp75wuCANWXipqCEgdaFGU5zuBkBkTQhVTP7vOSz6ru0M1r5dzf8XQ',
  active: { type: 'Boolean', value: true },
  name: { type: 'String', value: 'Test Record' }
}

=== Find with Query ===
Found documents: [
  {
    value: { value: 42, type: 'Integer' },
    active: { value: true, type: 'Boolean' },
    name: { value: 'Test Record', type: 'String' },
    id: 'hY8V7vkGiK2Fhhwto9OHtLx4nGI2JbtKJp75wuCANWXipqCEgdaFGU5zuBkBkTQhVTP7vOSz6ru0M1r5dzf8XQ'
  }
]

=== Update Document ===
Updated: {
  id: 'hY8V7vkGiK2Fhhwto9OHtLx4nGI2JbtKJp75wuCANWXipqCEgdaFGU5zuBkBkTQhVTP7vOSz6ru0M1r5dzf8XQ',
  name: { value: 'Updated Record', type: 'String' },
  active: { type: 'Boolean', value: true },
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
✓ Inserted test record: HZkH3TRe3gYoePb9QBZBsmtQGMFSBHGGK1R8JrLNBhbtB4yCilYEnpcNLxhGpQvnMIvwxtdNY_mb64BrPBezhw

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "type": "Success",
  "payload": {
    "data": [
      {
        "value": {
          "value": 42,
          "type": "Integer"
        },
        "id": "HZkH3TRe3gYoePb9QBZBsmtQGMFSBHGGK1R8JrLNBhbtB4yCilYEnpcNLxhGpQvnMIvwxtdNY_mb64BrPBezhw",
        "name": {
          "value": "WebSocket Test Record",
          "type": "String"
        },
        "active": {
          "type": "Boolean",
          "value": true
        }
      },
      {
        "name": {
          "type": "String",
          "value": "WebSocket Test Record"
        },
        "active": {
          "value": true,
          "type": "Boolean"
        },
        "id": "yUKfe5gayo-nz3tFoxV7DkJpdK6NtxseDcnXmbywpf4An4h7dCXpPeM61DAfE14McsWdNePC96131lbVK03DeQ",
        "value": {
          "type": "Integer",
          "value": 42
        }
      },
      {
        "value": {
          "type": "Integer",
          "value": 42
        },
        "name": {
          "type": "String",
          "value": "WebSocket Test Record"
        },
        "active": {
          "value": true,
          "type": "Boolean"
        },
        "id": "ZqF2GUJAHEOeZWILvAvgIISb7zXERCutgPHNZRG0CU1ZjF6-HPnJEJdG_hxJJSuUdVW2FfaumB3rP4sEwOp6Dw"
      },
      {
        "active": {
          "value": true,
          "type": "Boolean"
        },
        "value": {
          "type": "Integer",
          "value": 42
        },
        "name": {
          "value": "WebSocket Test Record",
          "type": "String"
        },
        "id": "J-tns4m3pM-F0ODn53gscuBUySaJcmCGQMqJOtKNLdItqUUJq86wM8ZSWFtOOo1K78sYoRlknEWS_bUGMjSamw"
      }
    ]
  },
  "messageId": "1780488411337"
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

✅ Function saved: u5ctxwyWaWVWfrk8L2-yOo4KmTzsPPhvaD59e-82um5oGqfSTOoDAWGSafoa2ffB9jjkO7kQlgcZ06Gv-hpywg
📊 Found 10 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Function saved: b5OENkrnu8Qdb9qiN7RwyJEJ7uId3x5mUSlaeMibG0E_O7qaHqqwlDi5siziBglpdWdTqmiWOQKeUEhQhA0NSw
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: 71Ky6qK8kG8yIHAL3eMxOxxP7H_oDW6eWQZ0pq0XZgtxL_JSVZKoJElGYWkSoL_zJb6xtHyhXHU6TdXDS_JX4w
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"count":{"value":10,"type":"Integer"},"status":{"value":"inactive","type":"String"},"max_score":{"type":"Integer","value":90},"avg_score":{"type":"Float","value":50}}
   {"max_score":{"type":"Integer","value":100},"status":{"type":"String","value":"active"},"avg_score":{"value":60,"type":"Float"},"count":{"value":10,"type":"Integer"}}

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
cache:product:1: { name: 'Product 1', price: 29.99 }
cache:product:2: { name: 'Product 2', price: 39.989999999999995 }
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
Collection created with first record: xqOm70HcmXWwEKbao_GH7d_P5fyLn7NCfvlijuCMQ5H3Vxm2ZxMvxgDSkso4F7AKh5yTlb3yRPfVb_kBNueSlA

=== List Collections ===
Total collections: 26
Sample collections: [
  'schema_employees_client_ts',
  'batch_users',
  'chat_messages__ek0_testing',
  'test_accounts',
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
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/transactions.js ===[0m
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: JxrHyGVjjzeOzIUXolRz2BJyG4OiEj3WI_drmecxPMxAbxameHzNOD3e6zyKIqUV8LGXGwgM7535eaJy2o_yzg
Created Bob: $500 - ID: gKgHOIoyaZDBa7ELaUpZB5gPsM0RqM1Gl4eoSkbpqc3TrOCxBU9IArFjBg561Fa1bSGu489sQK69IkpqcV3K6w

=== Example 1: Begin Transaction ===
Transaction ID: 34890610-dbea-41b1-b93d-6268b7e2443d

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
New transaction: 4db3d630-ddf9-430e-b07a-b15a4744f727
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
   ✅ Function saved: XrQwv7wOOND9zylkYn64ueIPzI9Jhjo6yoXyQy33TzzlobQsLIL81cTpEfc1VbFGW83fuN59sj_FDyNzJJ2rRw

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
   ✅ Function saved: pXRsh5pXU3eSeM5JB4Fx5awnjxSbmrJcfaQAbAXrA_QUCmDqcnFRH9jRnRxs1GyhyP_S8qiDJyPN_kqX5Ndl1w

2️⃣ Calling Function (Query + Update + Verify)...
   ✅ Function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Status updated to: {"type":"String","value":"active"}
   📋 Name: {"value":"Alice Smith","type":"String"}

============================================================
📝 Function 3: Query + Update Credits + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: pw1AVh6B6lj0MIJkjopXNV_m5AXpEkPniqp9xuON0Nqy1G8-ONtzAngNqiaDrzOlfqUSwa8XdMPJ6GLsbWtUUA

2️⃣ Calling Function (Query + Update Credits + Verify)...
   ✅ Function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {"value":0,"type":"Integer"}
   📋 Status: {"type":"String","value":"active"}
   📋 Name: {"value":"Alice Smith","type":"String"}

============================================================
📝 Function 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: 73hIUDVNOFvTE3a90wXyhiIF8-zAZq0gN0zBOMkw4QZlZpUv1keRKpgu1Q_ET4qWXpx7gnda_kpWLblqSv1ROw

2️⃣ Calling Function (Query + Delete + Verify)...
   ✅ Function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted function: XrQwv7wOOND9zylkYn64...
   ✅ Deleted function: pXRsh5pXU3eSeM5JB4Fx...
   ✅ Deleted function: pw1AVh6B6lj0MIJkjopX...
   ✅ Deleted function: 73hIUDVNOFvTE3a90wXy...
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
  Output: Document ID = xWB5HCEbUgZyMzL79trN4h-EH5zy5Cm97IrS-XS4rl2fSF85NREg3YC-uTV0u-vbiEbsZzFF_M25NCeMJPwtBw
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: findById(xWB5HCEbUgZyMzL79trN4h-EH5zy5Cm97IrS-XS4rl2fSF85NREg3YC-uTV0u-vbiEbsZzFF_M25NCeMJPwtBw)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: findById(xWB5HCEbUgZyMzL79trN4h-EH5zy5Cm97IrS-XS4rl2fSF85NREg3YC-uTV0u-vbiEbsZzFF_M25NCeMJPwtBw)
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
  Output: Document ID = 3-2IVEm_O-b56DU_snrSNGBFpBJVtyNAbPeteAqunCLkvdAbWQW6wInDLD2rD6fa-iOxhN7bxT-uxzC2tuH-HA
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: findById(3-2IVEm_O-b56DU_snrSNGBFpBJVtyNAbPeteAqunCLkvdAbWQW6wInDLD2rD6fa-iOxhN7bxT-uxzC2tuH-HA)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: findById(3-2IVEm_O-b56DU_snrSNGBFpBJVtyNAbPeteAqunCLkvdAbWQW6wInDLD2rD6fa-iOxhN7bxT-uxzC2tuH-HA)
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

> @ekodb/ekodb-client@0.19.0 prepare
> npm run build


> @ekodb/ekodb-client@0.19.0 build
> tsc


up to date, audited 52 packages in 1s

17 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.19.0 build
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
  id: 'PQSyE3Xs5Z0CwHQRP56MrkrUkSCb6etRrhoryh20VDSZ8tX8vr1-IlwRY-Yvv6Kl98LIIg9Kqibr0kj0TeDR4w'
}

=== Find by ID ===
Found: {
  active: { type: 'Boolean', value: true },
  value: { type: 'Integer', value: 42 },
  name: { type: 'String', value: 'Test Record' },
  id: 'PQSyE3Xs5Z0CwHQRP56MrkrUkSCb6etRrhoryh20VDSZ8tX8vr1-IlwRY-Yvv6Kl98LIIg9Kqibr0kj0TeDR4w'
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  active: { type: 'Boolean', value: true },
  id: 'PQSyE3Xs5Z0CwHQRP56MrkrUkSCb6etRrhoryh20VDSZ8tX8vr1-IlwRY-Yvv6Kl98LIIg9Kqibr0kj0TeDR4w',
  name: { type: 'String', value: 'Updated Record' },
  value: { type: 'Integer', value: 100 }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: 4UX3mGkV9vyIDU4KFslfqtCIPJM-YwD8OjDrN1tuwtfap8b4yz6vkrVY5L12loD7K-Dt5Aug9g_cvgbHJlaMsA

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
Collection created with first record: zZ1zgslVF-BoXuBP2-IRNLZhJuTReAtV_UfIf8tack_5_eCdFEb-_kGpltflUFIswrvtmddJj6icTBNU6h0bUQ

=== List Collections ===
Total collections: 25
Sample collections: schema_employees_client_ts,batch_users,chat_messages__ek0_testing,test_accounts,schema_employees_client_go

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
Total keys in store: 15

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: false

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 6nIz-NaBHfyGHNOSuvajiO6UXU9WuO0K4d3KbgO4Jj87SYW708anoHCcSSzlExha4p6oRdgQqwqgb-6v3RSRQg
Created Bob: $500 - ID: Gu6fnxVSqPrH2zw5Vct-9-hc4yTM62HulsVtzeToQpCTvAvHSCUHQrKxfDT6pHKKPxnHY1Tv269Miqa_u_RpSA

=== Example 1: Begin Transaction ===
Transaction ID: 059531c0-551a-47d4-bab4-496e5fe8b746

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 2f996ed3-5764-4b24-acae-8bfe52d92f38
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
  1. Score: 25.740, Matched: email, name.value, email.value, name
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: bio, title.value, title, bio.value
  2. Score: 26.400, Matched: bio, bio.value, title.value, title
  3. Score: 26.400, Matched: title.value, bio.value, title, bio
  4. Score: 26.400, Matched: bio.value, bio, title, title.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio, title.value, title, bio.value
  2. Score: 39.600, Matched: title.value, bio.value, bio, title
  3. Score: 39.600, Matched: title.value, bio, title, bio.value
  4. Score: 39.600, Matched: bio.value, title.value, title, bio

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio.value, bio

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio.value, bio

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.755, Matched: 
  2. Score: 0.739, Matched: 
  3. Score: 0.729, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.702, Matched: title, content, content.value, title.value
  2. Score: 1.491, Matched: content, content.value, title, title.value
  3. Score: 0.296, Matched: 

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
✓ Inserted document: 3pMTayQ-etCl8qgxDdMR3rXqRO5CpJu1T6g6-2DQBoyWrUhb0itmKacsB6jSr1O9GQj7y7aTl0lOUIWEp_EkFw

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: mOw3J25fcGiZNO1pDVevkrZEsIsbaP_ETocXH6t0w6em9fx-Z6j5B8eYMmnbiK0MRrUZ0tM9rMCAPzJwilmvEg

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
✓ Inserted document with TTL: o2PMC4lIyN0Gh2ZJaL1xyWN8gl4RMuWOSzZZUjuroygZhziPaJaHL4di_jl4iJGukdf1rypavvtpqJPqj2oU9A

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
✓ Edge cache script created: TvDR-TSXa_sc_IG2jqVuQ83CMjx4WDr2TfSYXmbVfNtofNbHCd3RPDHUrVq4Awii3NaWTOi9WnHBqctd9BoPQg

Call 1: Cache miss (fetches from API)
Response time: 55ms
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "timezone_abbreviation": "GMT",
          "current_units": {
            "interval": "seconds",
            "temperature_2m": "°C",
            "time": "iso8601"
          },
          "utc_offset_seconds": 0,
          "generationtime_ms": 15.118837356567385,
          "timezone": "GMT",
          "longitude": -73.99308,
          "current": {
            "temperature_2m": 19.6,
            "time": "2026-06-03T12:00",
            "interval": 900
          },
          "elevation": 32,
          "latitude": 40.710335
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
Response time: 44ms (1.3x faster!)
Result: {
  "records": [
    {
      "value": {
        "value": {
          "timezone_abbreviation": "GMT",
          "current_units": {
            "interval": "seconds",
            "temperature_2m": "°C",
            "time": "iso8601"
          },
          "utc_offset_seconds": 0,
          "generationtime_ms": 15.118837356567385,
          "timezone": "GMT",
          "longitude": -73.99308,
          "current": {
            "temperature_2m": 19.6,
            "time": "2026-06-03T12:00",
            "interval": 900
          },
          "elevation": 32,
          "latitude": 40.710335
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
✅ Function saved: zvtVSz3MLGQ-zgyCnRQfY2Q2mcWon_kwWj4p3EvlAhevKArzqipXPAlhmTg7F8pKCOVGqBUh1nyfVnUBpLrmIQ
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
   {"status":{"value":"active","type":"String"},"avg_score":{"type":"Float","value":60},"count":{"type":"Integer","value":5}}
   {"count":{"type":"Integer","value":5},"avg_score":{"type":"Float","value":50},"status":{"value":"inactive","type":"String"}}
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
   Name: {"type":"String","value":"User 1"}
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
   ⏱️  Duration: 158ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "address": {
        "street": "Kulas Light",
        "city": "Gwenborough",
        "suite": "Apt. 556",
        "zipcode": "92998-3874",
   ...

Second call (cache hit - from cache):
   ⏱️  Duration: 41ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "address": {
        "street": "Kulas Light",
        "city": "Gwenborough",
        "suite": "Apt. 556",
        "zipcode": "92998-3874",
        "geo": {
        ...
   🚀 Cache speedup: 3.9x faster!

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

✅ Function saved: qlZ_wYuXSK4kiTAUCepTo3syw0szof0nhuSSCQtT2y77JabAuXbdeuRJSeAoOAYUdON8vihCl5FL3tAMsTW01g
📊 Found 2 product groups
   {"category":{"value":"Electronics","type":"String"},"avg_price":{"value":575.6666666666666,"type":"Float"},"count":{"value":3,"type":"Integer"}}
   {"category":{"type":"String","value":"Furniture"},"count":{"type":"Integer","value":2},"avg_price":{"value":474,"type":"Float"}}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   {"count":{"type":"Integer","value":3},"category":{"type":"String","value":"Electronics"}}
   {"count":{"type":"Integer","value":2},"category":{"value":"Furniture","type":"String"}}
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

✅ Inserted order: lvgzGiOkta8j2mYugqj9Ua2ep7ei-OHYJGzz6RlZsGoDJeAlClLxcPGIhc4XClwDwXMW5FKzXqS0cqQK2ikKsg
✅ Inserted 2 products with wrapped types

📝 Example 2: Function with Wrapped Type Parameters

✅ Function saved: zIs3SdzH3kmMpll_vrZz7OZ-_UYKgzUMBjQWgYxaw0yx_TB1bgOi8GvH2ZK1VuLd3DEQ6-uYDtStky0IjSsrtw
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"role":"admin","userId":"user_abc"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: BN59dvsy8ochQHyBWSVXB0f8sXKtuqFRup-VZLDVdAjK9QK-1x_0c9h3x6GcrgFg7t8BT1W-KKfQ2Wl0ByyB0Q
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Function

✅ Function saved: 0IdgfnzCOPzLlQpyUoc0r6kSBwq-O4XLx-4Al8F933HJIs1DF2paYdWUNPUS_vjLIUl9MA8mUsv3Xky01x4Y3g
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
   1. User 5 - Score: 50
   2. User 4 - Score: 40
   3. User 2 - Score: 20
   4. User 8 - Score: 80
   5. User 6 - Score: 60
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
   1. Vector Databases Explained (Database)
   2. Natural Language Processing (AI)
   3. Database Design Principles (Database)
   4. Introduction to Machine Learning (AI)
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
   1. Vector Databases Explained
   2. Natural Language Processing
   3. Database Design Principles
   4. Introduction to Machine Learning
   5. Getting Started with ekoDB
⏱️  Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Function saved
📊 Projected documents (showing first 3):
   1. Vector Databases Explained
   2. Natural Language Processing
   3. Database Design Principles
⏱️  Execution time: 0ms

📝 Example 5: All Document Fields

✅ Function saved
📊 All documents (5 total, showing first 2):
   1. Vector Databases Explained (Database)
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
   Furniture:
      Products: 4 | Stock: 43 | Avg Rating: ⭐4.26
   Electronics:
      Products: 6 | Stock: 232 | Avg Rating: ⭐4.52

⏱️  Total execution time: 0ms
📈 Pipeline stages:

📝 Example 5: Project Specific Fields

✅ Function saved
📊 Product summaries (10 items, showing first 3):
   1. Office Lamp - $79 (⭐4.05)
   2. Webcam HD - $119 (⭐4.5)
   3. Laptop Pro - $1299 (⭐4.8)
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
   1. Precision: Vector data is less likely to lose detail and accuracy when zoomed in or out, making it more suitable for precision works.

2. Quality of Graphics: Vector graphics maintain a high quality at any scale.

3. Less Storage: Compared to raster data, vector data generally consumes less space.

4. Easy to Update and Modify: Changes to vector data such as updates or deletions can be done easily and quickly.

5. Attribute Storage: Vector data allows for the storage of additional information or attributes about each vector feature.

6. More Details: Vector databases can store more explicit details like roads, points of interest, etc., making them highly useful in geographic information system (GIS).

7. Efficient Queries: They facilitate efficient spatial queries and analysis. For example, finding the shortest path, overlaying information, etc.

8. Better Representation: Vector models are excellent at representing discrete data and linear features such as roads, railways, etc.

9. Topology: Vector data supports topological details by maintaining spatial associations between connected or adjacent geographic features.
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embedding function saved
📊 Generated embeddings for 3 articles
   1. "Getting Started with ekoDB" - 0D vector
   2. "Draft Article" - 0D vector
   3. "Advanced Query Patterns" - 0D vector
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
✓ Created SWR script: fetch_api_user_js (KOfLqg2kiqLGFyXilSMeivry5oy1MibgH5pxEFO1GAuZp-2Unlysh5LC-NqnYxgeMNinsnbKtTQUbn_IT5HVdw)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "address": {
            "suite": "Apt. 556",
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            },
            "street": "Kulas Light",
            "zipcode": "92998-3874",
            "city": "Gwenborough"
          },
          "company": {
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona"
          },
          "phone": "1-770-736-8031 x56442",
          "username": "Bret",
          "name": "Leanne Graham",
          "email": "Sincere@april.biz",
          "website": "hildegard.org",
          "id": 1
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
Response time: 44ms (served from cache)
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
✓ Created native SWR script: github_user_native (SvTsgsxj0xdyc90n0oHnYi2mygwkU12INrXv15pp5_U_h_ojXVIBxg3EiTMLh2DdEVq3yiOZd-Dna9h-Q5rxrg)

First call (cache miss - will fetch from GitHub API):
  Response time: 15ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 16ms
  Speedup: 0.9x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (ddB36LtiL-dTNkRliJh-SwLExL2YdsooDv8o0JnLogedW68YmB9LOa-ydp3wU9Gq73Do0aXdwqxZpBNkXGquUg)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (ZUMNCzt1AR3QhF7jnNzWBI_IyqQF9sVMaxeX3Ewmxi7dw9we4X7e1MQzw_zJ-qLrK5TGnMXEvwXjnHeGmMeKhQ)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (GrOjAyka2GqVECPoMPRrSG0mcQ69V-OiFT5yRGnzlpnDeSP5KHWnBn8wpnge0AFNpmNFoEul0-vmiEjHZtds6w)
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
✓ Created session: ep34SjGEdHc7fphlo8ZB94v0t10QAG2nCI5EFTTOJsGVckxxAXslGexwNsKgYDrZLPpzmynRgKYVf_ihASpk6g

=== Sending Chat Message ===
Message ID: Ga_kDob-nIYL3jUTHvfyX74HJ3DvAoIscdS2o6XQNkRkdQzTJJG02gH290wqpHW6whksa7mCY6DpmuR__Qu0Bw

=== AI Response ===
Here are the available products and their prices:

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
    description: 'Fully managed cloud database service product',
    name: 'ekoDB Cloud',
    id: '5LXDA1Z10P-2ixRS24UqHqumBlyAV4anymCkl-zNRqsB-0UwRdjDAjo3MbOoG30nQww8tci2Md9cp7vSTre-BQ'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_js',
  record: {
    description: 'A high-performance database product with AI capabilities',
    name: 'ekoDB',
    id: 'wqs_rm0UwEwWlKtR8KpILXDxG7SaO_Fw2AW7WCgLSM476whaxxXCoIdVGcMsmq_Gysfgp_aL2LxAv4F_MLvSHw',
    price: 99
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_js',
  record: {
    id: 'zkvpFqQXclN6Zjljx7LernyDLdmYgrlYpmHMNG2g_ITjOVD96DDDwrshgD0xiAWv00WCc-Jwert8Os2lqEuigQ',
    price: 299,
    description: 'Enterprise edition product with advanced features',
    name: 'ekoDB Pro'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 4463ms

=== Token Usage ===
Prompt tokens: 3468
Completion tokens: 89
Total tokens: 3557

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: H5O71XjC5pjaHz5baxZ974fJWBfeOxwGr6-6ubPRoYZyvNchttBm0K78b2JgNGFNvp_2FG8dBiYTQbSWKoajwg

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** High-performance database product
- **Price:** $99

If you need more information or have other questions, feel free to ask!

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
✓ Created second session: DTVJtWJP8SEgFGwhKo81a1jmzF7qfVWc1IAXZ4h7Ms19lRc1L7G9Y7_cSJtD96nFfj6MR33Ft9-V1cy7m5yNZw
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
✓ Created session: LxA4UkP4d1whXvPnwnbL7NMnCHjf8qUfXefo7wSkFBDBSkGSqCu7vR-Sn6UJ2ijfox9TGe3PlDzEpq2gyJZVgA

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: A high-performance database product
- **Price**: $99

If you need further details or information about other products, please let me know!

✓ Message 2 sent
  Response: The price of ekoDB is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 0SsGPnhZ0lZSuxrNQ7au6mrJ4SG9QDT3mrSNwGB9cYCoaMWIdXxq2IVG6KL8K_AR2fuSxJbJJ3OUm-APkBWS8w
  Parent: LxA4UkP4d1whXvPnwnbL7NMnCHjf8qUfXefo7wSkFBDBSkGSqCu7vR-Sn6UJ2ijfox9TGe3PlDzEpq2gyJZVgA

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: 0SsGPnhZ0lZSuxrNQ7au6mrJ4SG9QDT3mrSNwGB9cYCoaMWIdXxq2IVG6KL8K_AR2fuSxJbJJ3OUm-APkBWS8w (Untitled)
  Session 2: LxA4UkP4d1whXvPnwnbL7NMnCHjf8qUfXefo7wSkFBDBSkGSqCu7vR-Sn6UJ2ijfox9TGe3PlDzEpq2gyJZVgA (Untitled)
  Session 3: DTVJtWJP8SEgFGwhKo81a1jmzF7qfVWc1IAXZ4h7Ms19lRc1L7G9Y7_cSJtD96nFfj6MR33Ft9-V1cy7m5yNZw (Untitled)
  Session 4: ep34SjGEdHc7fphlo8ZB94v0t10QAG2nCI5EFTTOJsGVckxxAXslGexwNsKgYDrZLPpzmynRgKYVf_ihASpk6g (Untitled)
  Session 5: Lsf8xYElZqDSv-MPOr4tSu4CTkOgnVfrcL5WXhD5AHA1q5a-aVGXzad1znA7g7zDT0Z5N_WZRul_s6oS26ViyA (Untitled)
  Session 6: hw1YktkqKM6M8gKVwjWqYkkIYCLA6jPxYMi65XqHMVWNY_sMHxyMNQuhANhhRcrmdjNBaZwZlbRmg4s6NTqNYw (Untitled)
  Session 7: D7wAj80NgvEr9Yc0d-mLl5Skucba9uJrNch064lxMheD90lwkgeRsFS9r1jhx8md_K3UzyGWYFg_G_jQR7K29g (Untitled)
  Session 8: n_Aag-iezC8kDJvkKeosutVyUStsN3sDN3_ovArhjj10xZEKa4Ua8wr1_QKx02Vi8tRsY3sv8qIa3-RnqPl60A (Untitled)
  Session 9: w_72DfAp1ZSzXbQAedA_nPr27WvQ1no3ZuxJy-G52J1Wrxdqv1IWunC5BDjmu7daUXqM47AZ4VhqkgIEGeTCWA (Untitled)
  Session 10: 8lOgsFKTJ7YMWW61K9ANczXpSqKs60HCnKVdsVuYz6YWuAAP8V1vHEimiBeo4HFBFwkAF5g45Tfkxp-lmjSAUw (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: 0SsGPnhZ0lZSuxrNQ7au6mrJ4SG9QDT3mrSNwGB9cYCoaMWIdXxq2IVG6KL8K_AR2fuSxJbJJ3OUm-APkBWS8w

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
✅ [32mJavaScript client examples complete![0m
