make test-examples-javascript
🧪 [36mRunning JavaScript examples (direct HTTP/WebSocket)...[0m

added 1 package, removed 1 package, and audited 9 packages in 851ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
[34m
╔════════════════════════════════════════╗[0m
[34m║  ekoDB JavaScript Examples Test Suite ║[0m
[34m╚════════════════════════════════════════╝[0m
[34m
=== Checking Server Connection ===[0m
(node:19322) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
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
  id: '2aehLTJSS_KCrtmZa92SkZxU_OU5q1-6TlfrfWeAIEo6eSSlPAJl8uHGL-i5kGZ9-KUhoOFI_FXCXtkUhjydaA'
}

=== Find by ID ===
Found: {
  id: '2aehLTJSS_KCrtmZa92SkZxU_OU5q1-6TlfrfWeAIEo6eSSlPAJl8uHGL-i5kGZ9-KUhoOFI_FXCXtkUhjydaA',
  active: { value: true, type: 'Boolean' },
  value: { type: 'Integer', value: 42 },
  name: { value: 'Test Record', type: 'String' }
}

=== Find with Query ===
Found documents: [
  {
    value: { type: 'Integer', value: 42 },
    id: '2aehLTJSS_KCrtmZa92SkZxU_OU5q1-6TlfrfWeAIEo6eSSlPAJl8uHGL-i5kGZ9-KUhoOFI_FXCXtkUhjydaA',
    name: { type: 'String', value: 'Test Record' },
    active: { value: true, type: 'Boolean' }
  }
]

=== Update Document ===
Updated: {
  id: '2aehLTJSS_KCrtmZa92SkZxU_OU5q1-6TlfrfWeAIEo6eSSlPAJl8uHGL-i5kGZ9-KUhoOFI_FXCXtkUhjydaA',
  value: { type: 'Integer', value: 100 },
  active: { type: 'Boolean', value: true },
  name: { value: 'Updated Record', type: 'String' }
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
✓ Inserted test record: kv4wmyY66Mz-mDO-DfglAp1KPNyoA-AIgJM-HmESnLWwuSW9wEOjBVDyy0g2MsCD4Fgt3A0HjSP61EY9TaA9ug

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
        "id": "kv4wmyY66Mz-mDO-DfglAp1KPNyoA-AIgJM-HmESnLWwuSW9wEOjBVDyy0g2MsCD4Fgt3A0HjSP61EY9TaA9ug",
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
        "id": "psWb4szhCOYR74wQQoV1UoXtekxHu4T-M77Ilvd9C8hiC7_DrSCj_I3Eh_y1yRQ_T5-wi4HjHQHATu_Z5OBMXg",
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
        "id": "CQXM2art_zD-gjpFOHTRP740F6rffXlCn_MtjzsbpBXE-KdDU_Mp2pvMJQugfLRs2BTAgvS9oEJOdCOBue75KQ",
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
        "id": "16S9-ioNkoo3kgUUG_aqhUl2hpsuWer-r39lEDJojbJcayOVpbHl9ceplM-3Ec8RJI5RKZeFc2ZaJ1zSmmjVzw",
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

✅ Script saved: Rdb63iPV81vLDy3mXzFrRgt4asJJmz_In7HE1IqeUN4sury33o9ZW0-1pgTAW2t3vvoTG0087sneaSKbxyDsmg
📊 Found 30 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: cRjSAq4luoxyNOTG2P-JFw6frviu5lKOu3d2B-Ljlfb67Vrmg3_tvT53Ep9Nt5Oq3-3KBvtS36C-Ut2mXnLgFQ
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: A44YYw9mfiElBMZZSSJE79uLp-6LsTsYY7FTSNw3FULjZPNZZ_ren8dxv6DdCBa7jWQu94-x5HHh4EwOc2N9Gg
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"avg_score":50,"count":10,"max_score":90,"status":"inactive"}
   {"max_score":100,"status":"active","avg_score":60,"count":10}

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
Collection created with first record: z2cyJAlA5aJXTgz2xLqtphsRaY84qaA2_UDD3YPt2uC4dsFruOQBF4E4tzlx2lbUoySvTaMSgOQMWuwHQSIoRg

=== List Collections ===
Total collections: 24
Sample collections: [
  'schema_documents_client_py',
  'test_collection',
  'schema_employees_client_go',
  'schema_users_client_py',
  'github_cache'
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
Created Alice: $1000 - ID: zUAYOZyssu95Qil1KRroQVX6lvSvsyMoepDY5GPXWDiSdv_eRSrpzzgaqAF4xbfva5izdLxObnNJ8KZQr37g2w
Created Bob: $500 - ID: rOJ7zffRfnK1qlgJfu4m0iCfuCy1IrF0HWQ8nbi6F-AttT2EakYgq0YGhu0jnX2xHLMtSJmGCCxV2l7wwrk6kg

=== Example 1: Begin Transaction ===
Transaction ID: 6f587319-eae5-47e4-94e5-76c50b4f1580

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
New transaction: 510c43e4-6938-455d-9cd2-dafb0c3ad25c
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
   ✅ Script saved: gX1qyysaQJ4T_f2JBE9eTNM-cfubvPsbvzVI1LOLnWa6n6YT-MdOuxliBcZ28wvXJBm2PLp6gNeoiIYDnbJAEw

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
   ✅ Script saved: OgNW51wg3dLRgONs7hlEgCXmcadRrzEWaglJw6H0uPjWbIQrEYUuoZwPh5YXEm0vgKZxdXIZ80U_GJEZCFns3A

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
   ✅ Script saved: Rf4DP6SKRdCUGtu0RAtiGNZ2U8ULY9tqMo5F1pSk0n7tKaFTifmOxsjaMm3xST02t3mEvhBkvLlitE0irFDacQ

2️⃣ Calling Script (Query + Update Credits + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {"value":0,"type":"Integer"}
   📋 Status: {"type":"String","value":"active"}
   📋 Name: {"type":"String","value":"Alice Smith"}

============================================================
📝 Script 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: cj2AhrVRuCdXy5Db_KgGYJxAmCeBpxIFoJZ2yzIlv-nmjCbgsX9qpZ_sLwKb8rAC7sS0yTAZKU0EBiTq0-o0Ag

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: gX1qyysaQJ4T_f2JBE9e...
   ✅ Deleted script: OgNW51wg3dLRgONs7hlE...
   ✅ Deleted script: Rf4DP6SKRdCUGtu0RAti...
   ✅ Deleted script: cj2AhrVRuCdXy5Db_KgG...
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
  Output: Document ID = aO-MLr272ul9tS6eWvi7Y4v2r9vXPdPdyrcBy2DSTreZgAwMz6sgPlK-IH9AqGIgg0SQl5O0c2-HTgugq3xNmw
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: findById(aO-MLr272ul9tS6eWvi7Y4v2r9vXPdPdyrcBy2DSTreZgAwMz6sgPlK-IH9AqGIgg0SQl5O0c2-HTgugq3xNmw)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: findById(aO-MLr272ul9tS6eWvi7Y4v2r9vXPdPdyrcBy2DSTreZgAwMz6sgPlK-IH9AqGIgg0SQl5O0c2-HTgugq3xNmw)
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
  Output: Document ID = 7xy4mq_j-WOv_aLJmgdcEstf4-uhxoit_qa_xPhKmsBCJx21faO3MaGuRlVhLFjwmfafbtg8jdvgRLn0CX8M5g
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: findById(7xy4mq_j-WOv_aLJmgdcEstf4-uhxoit_qa_xPhKmsBCJx21faO3MaGuRlVhLFjwmfafbtg8jdvgRLn0CX8M5g)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: findById(7xy4mq_j-WOv_aLJmgdcEstf4-uhxoit_qa_xPhKmsBCJx21faO3MaGuRlVhLFjwmfafbtg8jdvgRLn0CX8M5g)
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

added 1 package, removed 1 package, and audited 9 packages in 863ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'xWChabJN6A4HF9JP2Mqr7pupldUlLPURTt4Y8MicHA_VKdbYqqwfckNSqHLDf7CTllzpDj7kEL-XgS6o5h6PFA'
}

=== Find by ID ===
Found: {
  value: { type: 'Integer', value: 42 },
  active: { type: 'Boolean', value: true },
  name: { value: 'Test Record', type: 'String' },
  id: 'xWChabJN6A4HF9JP2Mqr7pupldUlLPURTt4Y8MicHA_VKdbYqqwfckNSqHLDf7CTllzpDj7kEL-XgS6o5h6PFA'
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  name: { value: 'Updated Record', type: 'String' },
  id: 'xWChabJN6A4HF9JP2Mqr7pupldUlLPURTt4Y8MicHA_VKdbYqqwfckNSqHLDf7CTllzpDj7kEL-XgS6o5h6PFA',
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
✓ Inserted test record: H-CADV_5EnGBQp5A9pb8ONasikCdghhEsYJPWV6EexFzn5MQVMPyTBzIBTDYSYGx0Co2yNtzItfimDhC7st0rA

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
Collection created with first record: pSGV26fsOOdz4Gby0008lnm0neSoB6I0tXo-nzlN61zR-Jzq13krLGoWil3bCublrCDg1L4sz6PnLQRpuxVXHw

=== List Collections ===
Total collections: 23
Sample collections: schema_documents_client_py,test_collection,schema_employees_client_go,schema_users_client_py,github_cache

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
Total keys in store: 6

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: false

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: E1BDNQ2xOvP1G-RDJ6muy2n8U7vn0610EFnJy7Ubb607CG2G95-YYD-TgniAReBjQPT_LHYUo84K_Vt4rSnTAQ
Created Bob: $500 - ID: y40SDRRGGsMA2h7jev0amCfYI0wX2Vatm85_WeEHnf_GqmjYVpJfhXLtnehnm-7RixTXvJNCCg_ykZ49rPbICQ

=== Example 1: Begin Transaction ===
Transaction ID: d542e278-00fb-4502-bc1a-e06f86527888

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 4d74ac65-5aff-46a7-bed7-e8e105bc6caa
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
  1. Score: 25.740, Matched: name.value, name, email, email.value
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: bio, bio.value, title.value, title
  2. Score: 26.400, Matched: title, bio.value, bio, title.value
  3. Score: 26.400, Matched: title.value, title, bio, bio.value
  4. Score: 26.400, Matched: title, bio, title.value, bio.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title, title.value, bio.value, bio
  2. Score: 39.600, Matched: bio, bio.value, title.value, title
  3. Score: 39.600, Matched: title.value, bio, title, bio.value
  4. Score: 39.600, Matched: title, title.value, bio, bio.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio.value, bio

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.761, Matched: 
  2. Score: 0.739, Matched: 
  3. Score: 0.734, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.694, Matched: content, title, content.value, title.value
  2. Score: 1.505, Matched: content.value, content, title.value, title
  3. Score: 0.295, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: skills, skills.value, bio, bio.value

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
✓ Inserted document: -eVmQXh7O2NF9dq2_ZvUAcEfn5yjgWvc7xHESmOXbr4AqvwwvbmFwC79PDFKiuqcljAJ1udNk9YuoDkwLfrvnA

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: aeANt684aRNaowQ9YPsbcXlHhl64mb9nOxR7zB-zOX39fmQ_PZ_JSNnNKQeJaSYNif48OIFJiz8lRoenV60_gg

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
✓ Inserted document with TTL: -T2SFFu-g0-OPoSagmZo2HJL8KIgaOZsJLwEFIK00E07GydbwPLXdRaUHsjvgffcRVKV7l8TV9dkTfqjjk-Bew

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
✓ Edge cache script created: V6P7hUf8_F6J6eIUHnN6xzuyuXFp9-d_k52jJti2pQ9ajdApYG1WkLKe7E7ucpDdi4Oi84ObSQThtbDVOcpZVw

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
Response time: 4ms (1.3x faster!)
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

✅ Script saved: XrSjtZTS2N7z4mX1e6VBPwCCP7MxaGetTq5V4X-slQzivDATvqHXWq5iqfLA9WhZqyMwMwvirO5BaXTTT9G5mQ
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
   {"avg_score":50,"status":"inactive","count":5}
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
   ⏱️  Duration: 141ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 89ms
   📊 Records: 1
   🚀 Cache speedup: 1.6x faster!

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

✅ Script saved: d9eGI8alQTjIuGGo1CE01dWQsEjckhWoC3TbteNY7_EQUExhC7SnAsJOYwcB2jTRJEkAhyMqeS2VHuwiToEXtQ
📊 Found 2 product groups
   {"avg_price":575.6666666666666,"category":"Electronics","count":3}
   {"category":"Furniture","avg_price":474,"count":2}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved
📊 Found 2 categories
   {"count":2,"category":"Furniture"}
   {"category":"Electronics","count":3}
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

✅ Inserted order: KzPUd5_FvxQchSaIJCt9iDUqdMlPetlw4L0VQXhaO4-kQ60nV0ZCZ5oB3jO7EtBD1dPB4mWS34hso4-f_31tBg
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: R_3PTLj0KLUO8G6lKgOnVweHptdXoIU1I7aHxRfM-Lp1_Uqv9Mx0la8TzwCLtfjwPd-6Y5sTfpAa4a-FCAxScQ
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"role":"admin","userId":"user_abc"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: HCgXmEWrR3lku27D22d43aqTaesGrc6eTary6tPgL5A_AsIH6RKanKccVtaTQyD30jLdJu2Hm73xHg3T-L7w2Q
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Script

✅ Script saved: wd7XyyLTALC_mR0_x1pCA8ZKSu0H6qDeg28ba0el3WvRQQVtnrLqf3295AItiAtWKxhoLZONrItDjBZwYl-ZbA
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
📊 Users (showing first 5 of 10):
   1. User 5 - Score: 50
   2. User 1 - Score: 10
   3. User 4 - Score: 40
   4. User 2 - Score: 20
   5. User 7 - Score: 70
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
   2. Getting Started with ekoDB (Database)
   3. Vector Databases Explained (Database)
   4. Introduction to Machine Learning (AI)
   5. Database Design Principles (Database)
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
   2. Getting Started with ekoDB
   3. Vector Databases Explained
   4. Introduction to Machine Learning
   5. Database Design Principles
⏱️  Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Script saved
📊 Projected documents (showing first 3):
   1. Natural Language Processing
   2. Getting Started with ekoDB
   3. Vector Databases Explained
⏱️  Execution time: 0ms

📝 Example 5: All Document Fields

✅ Script saved
📊 All documents (5 total, showing first 2):
   1. Natural Language Processing (AI)
   2. Getting Started with ekoDB (Database)
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
   2. Keyboard - $89 (⭐4.4)
   3. Laptop Pro - $1299 (⭐4.8)
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
   Vector databases provide several benefits including:

1. Precision: They offer an accurate representation of data points, lines, and polygons.
2. Scalability: They can be easily zoomed in or out without losing detail or quality.
3. Topology: They support topological information which is crucial in spatial analysis and geographic information systems (GIS).
4. Size: They tend to be smaller in file size compared to raster databases for the same complex data.
5. Versatility: They can represent data like points for specific locations, lines for roads or paths, and polygons for areas such as city boundaries.
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
✓ Created SWR script: fetch_github_user_js (JuwoVOyvdQfEgc5roXNxUmc2wMF6aYIYtI_WsW28dgFm5j3IX40NVvpTlaVwJ9C2ZnaUVxs0FOFPJToXVMQB7w)

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
✓ Created session: erVyxQ7LtMBd7RRpeIT3Hr_FKrKPXKK573E2lUfvZX4YGZyr_yrHjhZTwRopTcj_IGj-YQAcxGMH4rPGEp7vaw

=== Sending Chat Message ===
Message ID: vjYlpUIgrzb70R0qV_bnIR_HMuPBLGhzJqaxX0A6LWMDud63ErmAPZba86bK8ME-wBmdkZdEYt2SGhqgFdBsoQ

=== AI Response ===
Sure, here are the available products and their prices: 

1. ekoDB Pro: This is an enterprise edition product with advanced features. The price for this product is $299.
2. ekoDB: This is a high-performance database product with AI capabilities. The price for this product is $99.
3. ekoDB Cloud: This product is a fully managed cloud database service. The price for this product is $499.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_js',
  record: {
    name: 'ekoDB Pro',
    id: 'UwLPxhVFPOneroPBD_j7UmY3Pemj0IpfRUJbIMxQVMyVjTov2vCS4MEhaiNtLGYgmAif4t8-X4_RN-9nvNcS5Q',
    description: 'Enterprise edition product with advanced features',
    price: 299
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_js',
  record: {
    description: 'A high-performance database product with AI capabilities',
    price: 99,
    name: 'ekoDB',
    id: 'uIWJuTqulH4d5_ERd2DRzsq_ZffZJotqPlQaiLnP4Mr1XkE0FCrCzixpuEp0hbheZsPO3CTGQuyIRaKw2p37Yw'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_js',
  record: {
    name: 'ekoDB Cloud',
    id: '2LcpOeMbAnCSa6_JRaK-Tv3vPKX-4IM4z1ZDNUFrPe3kO33Rd8xOdTxiCo2coBE5hHX2dEUJ2PzI--x_z9FOew',
    price: 499,
    description: 'Fully managed cloud database service product'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 3720ms

=== Token Usage ===
Prompt tokens: 451
Completion tokens: 90
Total tokens: 541

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: SnMZ-34T_FsIZQVDAwFhGTAUUNR3mEGzSsVqYxkbLyOWSDiyVvWF2YCSe9oJun44NJ_CvVjvWZDpg1QdwbBiJA

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information provided, the available product is ekoDB which is a high-performance database product. The price for this product is $99.

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
✓ Created second session: nkMo896y3NqO6xI8khJWyZ76wrEVqqICX72ES0HeriaclAr0jmbLQlNbdLelbf2pFtQA_8u7bWzLnZLutnIv5A
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
✓ Created session: 3AlPLcmuuwyUaJzXg50dl6FW-34XT8zbmegVJ_ExwcnQTWc13LqSeYSBvGBmwCwcwawzyUYSTndokLfUiI0dWA

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product based on the context provided is "ekoDB", a high-performance database product. The price of this product is $99.

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: fvM5qxQHsyyYHet1zMljXfpD6cmcDGqRQ6HepiJsrxzGB31oaNfzYy3l9xafmtcDiftlBYTNhARywcaYITfNbQ
  Parent: 3AlPLcmuuwyUaJzXg50dl6FW-34XT8zbmegVJ_ExwcnQTWc13LqSeYSBvGBmwCwcwawzyUYSTndokLfUiI0dWA

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: fvM5qxQHsyyYHet1zMljXfpD6cmcDGqRQ6HepiJsrxzGB31oaNfzYy3l9xafmtcDiftlBYTNhARywcaYITfNbQ (Untitled)
  Session 2: 3AlPLcmuuwyUaJzXg50dl6FW-34XT8zbmegVJ_ExwcnQTWc13LqSeYSBvGBmwCwcwawzyUYSTndokLfUiI0dWA (Untitled)
  Session 3: nkMo896y3NqO6xI8khJWyZ76wrEVqqICX72ES0HeriaclAr0jmbLQlNbdLelbf2pFtQA_8u7bWzLnZLutnIv5A (Untitled)
  Session 4: erVyxQ7LtMBd7RRpeIT3Hr_FKrKPXKK573E2lUfvZX4YGZyr_yrHjhZTwRopTcj_IGj-YQAcxGMH4rPGEp7vaw (Untitled)
  Session 5: QxYFWCDEy4jGOjgzQgOorxKovil9I4j8byyc7AL3vJnfd1RfZ8NUXLCYI5-WqPaXh1VbTvrZ_2pIndyADTgycw (Untitled)
  Session 6: t0Mua8_dEvpfI51B2-s0OGkS6eT0UxpoMtrdUqUIHf31FqbeBOcy6zMVuxqha9vyZSsTkjgIwqTH3vIiyvro_g (Untitled)
  Session 7: J0IrKtT3wll5y-FEb7_C_gH3g-k2RGJIOXkUqSd7qabCPA1Ss_qSU2K9dwIFGw22f0TVyiJ3eyd4-ZEsTZ8X-w (Untitled)
  Session 8: QgNaTKew4FukyFQpKPf4CDYmQLZpij-ZvQK1smny-h9u8n2GkvD5sxOPWGL5qBoaw1gpgODHHc8M8etF0zyo-A (Untitled)
  Session 9: 4ro2nsz3attRd93ZIQmS8CMbFnxwZnLU0AiIXFCx3wI71DKItAu8iu19Pv_gWkDSAkbXv578OyaCRNZlyKJN0w (Untitled)
  Session 10: VpNXcfJoGqkGtMBJo6_-992FNbN4k1fkjof5BM0K4YnCRY-6eIkZOJG7iE2pXivA2enUbqp-3-B0fo0rUMp1-A (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: fvM5qxQHsyyYHet1zMljXfpD6cmcDGqRQ6HepiJsrxzGB31oaNfzYy3l9xafmtcDiftlBYTNhARywcaYITfNbQ

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
✅ [32mJavaScript client examples complete![0m
