make test-examples-javascript
🧪 [36mRunning JavaScript examples (direct HTTP/WebSocket)...[0m

added 1 package, removed 1 package, and audited 9 packages in 871ms

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
  id: 'YCr-OwPkGLvhvowS44pRR7IZFEDmsDu0MblW4o9PV41gFrPTamjBcAacenHajkQt8AyE20F1ec2GtwiNJ6wPrg'
}

=== Find by ID ===
Found: {
  active: true,
  name: 'Test Record',
  value: 42,
  id: 'YCr-OwPkGLvhvowS44pRR7IZFEDmsDu0MblW4o9PV41gFrPTamjBcAacenHajkQt8AyE20F1ec2GtwiNJ6wPrg'
}

=== Find with Query ===
Found documents: [
  {
    value: { type: 'Integer', value: 42 },
    active: { value: true, type: 'Boolean' },
    name: { type: 'String', value: 'Test Record' },
    id: 'YCr-OwPkGLvhvowS44pRR7IZFEDmsDu0MblW4o9PV41gFrPTamjBcAacenHajkQt8AyE20F1ec2GtwiNJ6wPrg'
  }
]

=== Update Document ===
Updated: {
  id: 'YCr-OwPkGLvhvowS44pRR7IZFEDmsDu0MblW4o9PV41gFrPTamjBcAacenHajkQt8AyE20F1ec2GtwiNJ6wPrg',
  active: { type: 'Boolean', value: true },
  name: { type: 'String', value: 'Updated Record' },
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
✓ Inserted test record: hsBEYksk_N6zkkzB9-tkqrnaCLvYVuUYMFtehcwaVrw5rIH6TnPg6Xlr0e05XeeTkwHIzA2-KNLTC-MbbVbxEQ

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
        "id": "GVRDwJ1ctChqDW-nHlur23wm5ZxClykHrnNvvUfdGMGCmcMMLu6HmAOOGOX7dcCRGFO11nhRc7UzzQMSIvLOHw",
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
        "id": "hsBEYksk_N6zkkzB9-tkqrnaCLvYVuUYMFtehcwaVrw5rIH6TnPg6Xlr0e05XeeTkwHIzA2-KNLTC-MbbVbxEQ",
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
        "id": "GEv26dPSRPt4XxnEv67qCooubqCkgLUZqEpx5QsGzWhwRUF_PZGZzp29lz7Zn0D3Vf0Doykedto4X2Ei1hZe2w",
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
        "id": "ajOM9grEuxTgPap_lS26dHJIjqMNVtCi0kY26-XoI-_V6fIVLgzgWGF1AbVnsyVjJ7f6teDdL69dUR6se7jbXA",
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

✅ Script saved: S4PgIgAOSRsBLSHvFg7K6p2xW0RVAuMuGYOJ8agGpKtv5602HlLqBfbuomQ2D1RfST8TWKRxIloBlGtZlIHFiA
📊 Found 30 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: Qo6DQhdEZkvGdLIWI_BykHDJ6U15LVPphwvpeLdrbeCjcwVt1z09gJCB72xKRUSloym5IjXiQpjyVnC77OeOXA
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: 9SfcaJnUB8IIcWWs-2v05VcKe7Vo7GyTqHmrVR4zftnFZE_NCpPssMQu9xA_ILqfeyPZaindZ0EgYPfMLKozvw
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"avg_score":60,"status":"active","max_score":100,"count":10}
   {"avg_score":50,"max_score":90,"status":"inactive","count":10}

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
Collection created with first record: LINAd-q2gqo-cryTJGSc_Xw9WDyYJNMjJmqV82uN1CEIY7AzbV4HXqoA_9NAs_mR3o8JuRNocS_jt_f-vwWmbw

=== List Collections ===
Total collections: 24
Sample collections: [
  'schema_documents_client_go',
  'schema_products_client_py',
  'schema_employees_client_ts',
  'schema_users_client_ts',
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
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/transactions.js ===[0m
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: BoIPmiOW-2fubryEBjKum3LAt2Iv741Xtg3N7-nhFZAUd2CnGQOYfHLyrFwQU3MPkbOVwW9jexU4xIZXEoiofg
Created Bob: $500 - ID: vpJb7Sz6eQzYsL9WQsr0i8z0ENDv11eOzPBEdakaVKTfT2xteV48YMuOTJndsMV0PMUeXWv6Y_-ISsqKbc3QeA

=== Example 1: Begin Transaction ===
Transaction ID: c47f55ec-bee4-4ac1-ba89-62ab0af14432

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
New transaction: a3cdccda-b062-4403-8439-c7d223dd9f3a
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
   ✅ Script saved: -n5AJQqkgWtN04RCiJB1nyJoGBHYLsDvOQ3OIrnVa5ffYIJbqa3_OliVvOApLSocjISozN2Yl58LXcQeytieUQ

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
   ✅ Script saved: kXIyoUuGr9i4E3iwLXhgd3n9L0-9rNsomYAIiCuQ2EaSZCGqTdcJPjQKEv8LfOa7RbzEsnPAlJieTqLz80bWfA

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
   ✅ Script saved: 0UU-0FmprFEocJSyYrDcWyrzNODvigoA_GSzTAHrYp4r2OzOdLxmRzHy0auJyEUo0L3hwzYBVxXciYhoAWJUXw

2️⃣ Calling Script (Query + Update Credits + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {"type":"Integer","value":0}
   📋 Status: {"type":"String","value":"active"}
   📋 Name: {"type":"String","value":"Alice Smith"}

============================================================
📝 Script 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: sqyuMhslLR0A9VWURxKE2eh8tElCY9Gt8lhvMSDzQxDQhXb_x2afbAzCC5UC6Ca86rpOlm-AFBqQbtOJTGnLSA

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: -n5AJQqkgWtN04RCiJB1...
   ✅ Deleted script: kXIyoUuGr9i4E3iwLXhg...
   ✅ Deleted script: 0UU-0FmprFEocJSyYrDc...
   ✅ Deleted script: sqyuMhslLR0A9VWURxKE...
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
  Output: Document ID = aTbrAd-oAfJsat0bSfCsbup1vMVoPUDV7Kzrzc4FaLlJU-eKobvYsbj54MQyzwViyt7ew3A9umF9deA8zgKCbA
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: findById(aTbrAd-oAfJsat0bSfCsbup1vMVoPUDV7Kzrzc4FaLlJU-eKobvYsbj54MQyzwViyt7ew3A9umF9deA8zgKCbA)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: findById(aTbrAd-oAfJsat0bSfCsbup1vMVoPUDV7Kzrzc4FaLlJU-eKobvYsbj54MQyzwViyt7ew3A9umF9deA8zgKCbA)
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
  Output: Document ID = gTSrgQXowYMHsFJ8rLhf1aaQEcrxOObrvZnGyLs6OfYxG_fO4tloqhnS06IQKQlIpnOsgPhyc02OaxpCayeMqg
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: findById(gTSrgQXowYMHsFJ8rLhf1aaQEcrxOObrvZnGyLs6OfYxG_fO4tloqhnS06IQKQlIpnOsgPhyc02OaxpCayeMqg)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: findById(gTSrgQXowYMHsFJ8rLhf1aaQEcrxOObrvZnGyLs6OfYxG_fO4tloqhnS06IQKQlIpnOsgPhyc02OaxpCayeMqg)
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

added 1 package, removed 1 package, and audited 9 packages in 830ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'hnDaYX2CuaSlgpWJ4FeEm3hNfYdZJBlguzfWYcy2um58QW6_ttFZAHQw0wToi7ziVFozpLIY1KENsuLgp4TTvw'
}

=== Find by ID ===
Found: {
  name: 'Test Record',
  active: true,
  value: 42,
  id: 'hnDaYX2CuaSlgpWJ4FeEm3hNfYdZJBlguzfWYcy2um58QW6_ttFZAHQw0wToi7ziVFozpLIY1KENsuLgp4TTvw'
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  id: 'hnDaYX2CuaSlgpWJ4FeEm3hNfYdZJBlguzfWYcy2um58QW6_ttFZAHQw0wToi7ziVFozpLIY1KENsuLgp4TTvw',
  value: { type: 'Integer', value: 100 },
  active: { type: 'Boolean', value: true },
  name: { value: 'Updated Record', type: 'String' }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: aUEKQyqsrH66zsDGmplb04FVMFxzugsSOL9mW9NP7B8Ct4OHMge8qu6eKQBMvXmQxIEoC097OKhl1bbMdkZB2g

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
Collection created with first record: MdFA0RUBh3VsFEJFU5tmuR0cVu5UYzTGnd3APTVT_gt3JtN5KednI3QJt7WNqNySSZ-dp_XHRNKPXbMcipNdhw

=== List Collections ===
Total collections: 23
Sample collections: schema_documents_client_go,schema_products_client_py,schema_employees_client_ts,schema_users_client_ts,schema_users_client_go

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
cache:product:1: { price: 29.99, name: 'Product 1' }
cache:product:2: { price: 39.989999999999995, name: 'Product 2' }
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
Created Alice: $1000 - ID: 28MJ3cLMfEHiRN1hkTsu4ekAToBiosdloFOhurNsyRM5JeIMrCwrWul_LUzB0cng17B8jj-TyRiQiB7M3Kz1sw
Created Bob: $500 - ID: 26tkvYhp0BEbpIAwKBii8TSGQocsLO9fOl8wZATOmwjQY73LkBrK-yA5FNq8bSgJtAGnjQVPUdSnR6a-GLoqIw

=== Example 1: Begin Transaction ===
Transaction ID: 79910039-85e5-4c8f-aebc-f719068be8ab

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: a2f538f2-f411-4746-a765-1a61952a1543
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
  1. Score: 26.400, Matched: title, bio, bio.value, title.value
  2. Score: 26.400, Matched: bio.value, title.value, title, bio
  3. Score: 26.400, Matched: bio, bio.value, title, title.value
  4. Score: 26.400, Matched: bio, title, title.value, bio.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio, title, title.value, bio.value
  2. Score: 39.600, Matched: bio.value, bio, title, title.value
  3. Score: 39.600, Matched: title.value, bio, bio.value, title
  4. Score: 39.600, Matched: title.value, bio, title, bio.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.761, Matched: 
  2. Score: 0.743, Matched: 
  3. Score: 0.737, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.705, Matched: content, content.value, title, title.value
  2. Score: 1.497, Matched: content.value, title.value, title, content
  3. Score: 0.295, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: skills.value, bio, skills, bio.value

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
✓ Inserted document: XnhBxlAbnq3T7E_CWGs6ZDcXX_PNYg5uFub4oFQKMapksNdOXJWJGPKD3AIRBDbEEE-Rv0P6njfXpAAI8-ePMA

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: i1nO9Uqmz22uhrS2TsE3wC6QkN1A7lmAQNbsORPEWWTiB_2uNOXEE6VBv-St_tKIT_hUisheBkJH0qlwscjsEQ

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
✓ Inserted document with TTL: 8wuCeaeuJt-YXksF7LVI5U5xD8EK1PB_4PTn4bIuGtUIFNFbzOdhS-JyBD5_3VMwHzYPO1S2uuUrVt-_0X_KnA

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
✓ Edge cache script created: 4adZNQlC5AIKw_ucvCfFUHOhveTV2WrVy6b5JXXZ_JvnbcWZhoW8X9eZYQgjleQ5KWgDTbadAdzxz3sPPgFHRA

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
Response time: 4ms (1x faster!)
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

✅ Script saved: B6Gg3LZp9NsSMl_CKpBzTfbJ6JbeGndPyOpHoHMTpTwgAOKDSfRvLCuik_CxzCRJaCPWmMy6_AKnlK2tA2o7wA
📊 Found 10 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved
📊 Found 10 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved
📊 Statistics: 2 groups
   {"status":"inactive","avg_score":50,"count":5}
   {"count":5,"status":"active","avg_score":60}
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
   Name: {"type":"String","value":"User 1"}
   Department: {"type":"String","value":"engineering"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using CallFunction to replace inline logic in SWR pattern...

✅ Saved reusable function: fetch_and_store_github
✅ Saved SWR function using composition: swr_github_user

First call (cache miss - will fetch from GitHub):
   ⏱️  Duration: 131ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 69ms
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

✅ Script saved: FnfZTLBct-9pANsfjF6aAN5KDK6xH7X4DXh4XYq6KmERni4dp35wCqw95RTekV4O8gc3Ad0ta_Rjq6cuK_rArA
📊 Found 2 product groups
   {"count":3,"category":"Electronics","avg_price":575.6666666666666}
   {"category":"Furniture","avg_price":474,"count":2}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved
📊 Found 2 categories
   {"count":3,"category":"Electronics"}
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

✅ Inserted order: Pkiuhtl6dl8IzvWL54LBgAry0QTNTyZNT4jyPR8i6nEjX3RHEkVVhL5EP3QXAEK-yoDmI_OwSprd5tliCwGZ6w
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: qU4TNYJZrU5Om70Lnkw6waUzPmaM8rBSnjToJHCw9qBjt0fb0--numGYAigwTLFzR6gF8oxeqxQxtH7IE1UjFg
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"role":"admin","userId":"user_abc"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: bXPdbk7zDA82dgPs0t_JWqhx_5njbC2cICuicG6yvgkeBLwnL_PsWTwi56QLpSKVfqg_Hm-vmcEEb4xNEUPS8A
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Script

✅ Script saved: wBntr-b8TfnENpSuPJSiDvOv_2wiWzE77OMoBFzVBVHviD8cZh0q2hhxz0I_G3ki7242aZukbxf03t9SU-lkfg
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
📊 Users (showing first 5 of 5):
   1. User 1 - Score: 20
   2. User 2 - Score: 40
   3. User 4 - Score: 80
   4. User 3 - Score: 60
   5. User 5 - Score: 100
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
   1. Vector Databases Explained (Database)
   2. Database Design Principles (Database)
   3. Getting Started with ekoDB (Database)
   4. Introduction to Machine Learning (AI)
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
   1. Vector Databases Explained
   2. Database Design Principles
   3. Getting Started with ekoDB
   4. Introduction to Machine Learning
   5. Natural Language Processing
⏱️  Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Script saved
📊 Projected documents (showing first 3):
   1. Vector Databases Explained
   2. Database Design Principles
   3. Getting Started with ekoDB
⏱️  Execution time: 0ms

📝 Example 5: All Document Fields

✅ Script saved
📊 All documents (5 total, showing first 2):
   1. Vector Databases Explained (Database)
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
   1. Standing Desk - $599 (⭐4.7)
   2. USB-C Cable - $19 (⭐4.3)
   3. Webcam HD - $119 (⭐4.5)
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
   Vector databases offer several benefits such as:

1. High Precision: They provide accurate and detailed data representation as they use points, lines, and polygons.

2. Scalability: They can be easily scaled without losing data quality.

3. Flexibility: They allow complex geographic operations like calculating the shortest path, overlaying layers of data, etc.

4. Data Integration: They can integrate different types of data on the same map.

5. Less Storage: They typically require less storage space compared to raster databases. 

6. Interactivity: They offer the ability to add, delete, and modify features without impacting the entire database.
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
✓ Created SWR script: fetch_github_user_js (MZEOoECXDYq7pTBr1z-KAWz2hj1h0_mIClpyBM03jzPFwkeI7nwIYfLn2YmI1lLQheeFih0PaxXTsJ8Z_9p2QQ)

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
✓ Created session: AFoDIZhNX5jrNT2zdex209rENlsik0TMuGzK1WIK3YAuo8DzR8V0J5eZAV0r-TUkngiWwxAmIfZ5-9HP2ytnDQ

=== Sending Chat Message ===
Message ID: 9-VVFkywqKSf384wCucTRhJVWqOzw_SPoM9SbTqJeT5vs-x6f3gX7XKatDThWw1r0Q1VMoLUUSaDcmvmpnZhKQ

=== AI Response ===
There are three products available:

1. ekoDB: This is a high-performance database product with AI capabilities. The price is $99.

2. ekoDB Pro: This is an Enterprise edition product with advanced features. The price is $299.

3. ekoDB Cloud: This is a fully managed cloud database service product. The price is $499.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_js',
  record: {
    description: 'A high-performance database product with AI capabilities',
    id: '_Vo3ujp6qeTb8IEDw3sULflq5_TEu5ydocdiDpa_8_cabzFi_8smyXxhAMjd79PFQOqmUTJAzYnUibxHWB282A',
    name: 'ekoDB',
    price: 99
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_js',
  record: {
    id: 'Qn78GdPiSb2LC_sQD47Xyy4wjecBs5rBpdqmjgxwCOHdYrG5zeEW7nKmrUcWWoD56DOTyTMRSRrtyLWxE2eomQ',
    description: 'Enterprise edition product with advanced features',
    price: 299,
    name: 'ekoDB Pro'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_js',
  record: {
    price: 499,
    id: 'VkxKoyI_0XnYEa7nD0akyHvxNWfJoJ8YXJziV820pw7NPU_LOfR9canPTrmgJI7lxW8ZFDthkuy1ITT4SjxZIg',
    description: 'Fully managed cloud database service product',
    name: 'ekoDB Cloud'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 4969ms

=== Token Usage ===
Prompt tokens: 445
Completion tokens: 75
Total tokens: 520

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: w3Vqd-86slnazs08_8-kNlhCjQ1em5RBl2WLrXWO5m5wzPACGu8iFJQ_ZzfMChj4kNoFlcPquRaPr21ROCLCNQ

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the context provided, one available product is the ekoDB. It is a high-performance database product priced at $99.

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
✓ Created second session: gFZcgrXsqr51sNN9ZOZXiask5Tk-D33f2FxNAAn67FNdvLnBCoVnlHd7eG_XhKvT5YxRkZbIxSXpEEKyfA7f6w
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
✓ Created session: dDyPuL0ArR-2xoiQr1VhNJfkjEN0RL2Q4OjX-9nrbrYKMl_x72o58cw4K38ar6OZTFyf7YgCGE4zsUweuIj1EQ

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is a high-performance database product named "ekoDB". It is priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 292rI2955FBXmE7idanvPrzX_Wiceb8OcQbFBiPnErWSVWmqGmFx-ajUWuRCEkCWQ6kE-twc6u8xQ2vMhpsykg
  Parent: dDyPuL0ArR-2xoiQr1VhNJfkjEN0RL2Q4OjX-9nrbrYKMl_x72o58cw4K38ar6OZTFyf7YgCGE4zsUweuIj1EQ

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: 292rI2955FBXmE7idanvPrzX_Wiceb8OcQbFBiPnErWSVWmqGmFx-ajUWuRCEkCWQ6kE-twc6u8xQ2vMhpsykg (Untitled)
  Session 2: dDyPuL0ArR-2xoiQr1VhNJfkjEN0RL2Q4OjX-9nrbrYKMl_x72o58cw4K38ar6OZTFyf7YgCGE4zsUweuIj1EQ (Untitled)
  Session 3: gFZcgrXsqr51sNN9ZOZXiask5Tk-D33f2FxNAAn67FNdvLnBCoVnlHd7eG_XhKvT5YxRkZbIxSXpEEKyfA7f6w (Untitled)
  Session 4: AFoDIZhNX5jrNT2zdex209rENlsik0TMuGzK1WIK3YAuo8DzR8V0J5eZAV0r-TUkngiWwxAmIfZ5-9HP2ytnDQ (Untitled)
  Session 5: N7x7pp55YXw3bqdV-U7D_qqytgmhA6oUr6eEbffzzFO5Fpem9r1yPKwYwlDbOSWNFwpwNJ6GxPfwL_VjF7Y-7A (Untitled)
  Session 6: 99Pn4ItmLj9yNXdJ9MHXaR_S6DJOS752ZPS7m0yTb1fLPcBFhVyB3y20uMRIHqLljr2_IjUyTEO3fa5MBjqN5w (Untitled)
  Session 7: pOGOOxmPEbbRsJQtqdm6xQIKSSnyBzYzNv2ua9eCwXmKq3v8FBzOS04-JYqwjkZtfjTvWdttPNU8rzG222inrw (Untitled)
  Session 8: FbZ0jfAMEmukOnFz-TIuFApCVXAa_o4foAsdfdf0x8Okg3OrSYwzqJjwjXFpqe2WJZD-Ap7dEfDxqFEgyAEjOQ (Untitled)
  Session 9: 8djIhno3iGEPxBoj-qxsGY1Zpx_yFB-AEO3e3JulWsFnGJE73TCbQ-BN95kEtrpkrz4vmqG1UpPQSqVXz7qukA (Untitled)
  Session 10: GFi9nITIlmmDiWZfKTsqe6vfG12LC7fx6h3DUO-A8GnXij598gQSbfMBnLLFF371mGI3CoYy-ndoWqrvEIJSvA (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: 292rI2955FBXmE7idanvPrzX_Wiceb8OcQbFBiPnErWSVWmqGmFx-ajUWuRCEkCWQ6kE-twc6u8xQ2vMhpsykg

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
✅ [32mJavaScript client examples complete![0m
