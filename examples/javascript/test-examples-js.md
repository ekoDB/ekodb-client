make test-examples-javascript
🧪 [36mRunning JavaScript examples (direct HTTP/WebSocket)...[0m

added 1 package, removed 1 package, and audited 9 packages in 838ms

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
  id: 'qjnR_xxepMdsTFbRlVdMm8YrZnGdzeOMBPjW3z0Hl4d6Ovzt4xUhk9vNKVaTveHoiaaOEhY3hNogsrRrgUqJeg'
}

=== Find by ID ===
Found: {
  value: 42,
  name: 'Test Record',
  active: true,
  id: 'qjnR_xxepMdsTFbRlVdMm8YrZnGdzeOMBPjW3z0Hl4d6Ovzt4xUhk9vNKVaTveHoiaaOEhY3hNogsrRrgUqJeg'
}

=== Find with Query ===
Found documents: [
  {
    active: { type: 'Boolean', value: true },
    id: 'qjnR_xxepMdsTFbRlVdMm8YrZnGdzeOMBPjW3z0Hl4d6Ovzt4xUhk9vNKVaTveHoiaaOEhY3hNogsrRrgUqJeg',
    value: { type: 'Integer', value: 42 },
    name: { type: 'String', value: 'Test Record' }
  }
]

=== Update Document ===
Updated: {
  name: { value: 'Updated Record', type: 'String' },
  id: 'qjnR_xxepMdsTFbRlVdMm8YrZnGdzeOMBPjW3z0Hl4d6Ovzt4xUhk9vNKVaTveHoiaaOEhY3hNogsrRrgUqJeg',
  value: { value: 100, type: 'Integer' },
  active: { value: true, type: 'Boolean' }
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
✓ Inserted test record: hu9HjYXafG0IHTEiC5nFjXkKU6EJbkxGA9P9soZ6OmvLZRGPRLL3UUJWjB9CjPHQxVF4V2Zqrh3Lr5WXk0RCWw

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
        "id": "CUYleyzUq5ERksJhs2_ex0xZMacukDb7Fahybmg6_kYNBoaiKUftv-iDrdcvmQeuPACaYe-iqENgAa27grrarg",
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
        "id": "HcXTmrgLhE2Sv7O9HhkC6tf7c-SDSMXvqOTzKbm5zA0faWPThx9PS56pVP4iX3g7tWPKZeBg9R5v8p_06zrFIg",
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
        "id": "BRWJqswv00Jc4ldtHNBYkzPv1XIMTVaTFLjDm0FPXH0wMIOwGp6SL1oZIOUqlv30gPRzSi7wdyjSsdR6-gH8Tg",
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
        "id": "hu9HjYXafG0IHTEiC5nFjXkKU6EJbkxGA9P9soZ6OmvLZRGPRLL3UUJWjB9CjPHQxVF4V2Zqrh3Lr5WXk0RCWw",
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

✅ Script saved: jGRAwuXtIUlhY7SHcB1ADbij4BduFT5et8YYZn9i2spqCVUe8mPepXjvMiZYR9QOswmHad630W9N4ws9xy3Ndw
📊 Found 30 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: Ir3aVup6Pivg25sN4jM1M6ginA3xPiDixrmPMrQ9n5iyRzS9k0-LE2skWYtVqW5yRv9njJUgOJsYh5mAmXB_PQ
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: 3U_dNjTyc8IADkuiGJ4BoyPKqecZJu7ciGbjBmf5pcsQ3VGIvh1DfNquS_Qc382ztyu9MTvoUqZV2viHWPItsg
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"avg_score":60,"max_score":100,"status":"active","count":10}
   {"count":10,"avg_score":50,"max_score":90,"status":"inactive"}

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
cache:product:2: { price: 39.989999999999995, name: 'Product 2' }
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
Collection created with first record: 6PSjSH_whnAdXPGx3Qi7rDdcXNQ0zMl0X7ooo95XBL9Y1WvUnwj7wN_lzBv1zZvjrOPgLvTd2yV8XfwwIOaCLQ

=== List Collections ===
Total collections: 23
Sample collections: [
  'ws_ttl_test',
  'schema_users_client_ts',
  'test_collection',
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
Created Alice: $1000 - ID: q2k0H0BkJgeXIMgZH4De6plZR3e7GY9VlWc-VV1gOhzM744lC0ZaAjp_Dt5xj9oe-rLKpaTnZ8FSkZx-OTrT4A
Created Bob: $500 - ID: AVawKdzIyaUrhiJzuLYw6T_lFVQwrldOvS9ff-rezTiICFJyfCdr5xPylH1fonc0vAyGZhhmeXBoEBNT1jNxVw

=== Example 1: Begin Transaction ===
Transaction ID: 6a785ebc-a8ed-4d02-8342-312ec77f5a2b

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
New transaction: f2e74213-6798-4651-98c1-4547346e4836
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
   ✅ Script saved: irZr8xjxoQ39pHQcKgDTN2PHdpGD7o1lcCNc0hSz2jd1gpr6DRUxy_GKYP2Z2Q6Nc9c55JjZFMD_H4zGVkmVPg

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
   ✅ Script saved: FoQlVGlM47DFujNejs74Pemkvcagq0htxKJgafNPUFCCCwJqwkjz9Xb8xNX9CTCBO0ktuPDnzZS-QkXNj0AxqA

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
   ✅ Script saved: RUeA6BAzD1Vqr8i8Sf8BV1jteX2j9tPPJHxFCE5rlcDRDNXDixVscI3qyZLgJPkj5glywOvf9h8gQ3QZXbsgNw

2️⃣ Calling Script (Query + Update Credits + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {"value":0,"type":"Integer"}
   📋 Status: {"value":"active","type":"String"}
   📋 Name: {"type":"String","value":"Alice Smith"}

============================================================
📝 Script 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: EzxDWOmxAg8BC0Ri_KT0pZOFzl0ISpjFBOFTlD7cFgNPFAbb3KymMuRaA7VxzpXjbuX6bKG7_9GBj9ymwi9pTA

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: irZr8xjxoQ39pHQcKgDT...
   ✅ Deleted script: FoQlVGlM47DFujNejs74...
   ✅ Deleted script: RUeA6BAzD1Vqr8i8Sf8B...
   ✅ Deleted script: EzxDWOmxAg8BC0Ri_KT0...
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
  Output: Document ID = Vg4leDr-KXT-ItS2ogPytJlUejC4dInidNa1Q2PPUXjOWmmSWj0-p-O72qxHdD14cexAHRirstlSHguUmq5oRA
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: findById(Vg4leDr-KXT-ItS2ogPytJlUejC4dInidNa1Q2PPUXjOWmmSWj0-p-O72qxHdD14cexAHRirstlSHguUmq5oRA)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: findById(Vg4leDr-KXT-ItS2ogPytJlUejC4dInidNa1Q2PPUXjOWmmSWj0-p-O72qxHdD14cexAHRirstlSHguUmq5oRA)
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
  Output: Document ID = MBdmMxqOIlaYwwaqW8u78u1Vbq5ieqX3KhktzrtogmIOca929v4kPYIsUPBDRPG3XPvXyKEV898D3SUIG_qQVQ
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: findById(MBdmMxqOIlaYwwaqW8u78u1Vbq5ieqX3KhktzrtogmIOca929v4kPYIsUPBDRPG3XPvXyKEV898D3SUIG_qQVQ)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: findById(MBdmMxqOIlaYwwaqW8u78u1Vbq5ieqX3KhktzrtogmIOca929v4kPYIsUPBDRPG3XPvXyKEV898D3SUIG_qQVQ)
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

added 1 package, removed 1 package, and audited 9 packages in 876ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'CRL4vfW4U-mz54PCAZOOM4mZ6loCegMMBxOEIff33leyrNqPxpTmj93mkrN_sE5vZCDPQpoDWStwGS_ZCq6x5w'
}

=== Find by ID ===
Found: {
  name: 'Test Record',
  active: true,
  value: 42,
  id: 'CRL4vfW4U-mz54PCAZOOM4mZ6loCegMMBxOEIff33leyrNqPxpTmj93mkrN_sE5vZCDPQpoDWStwGS_ZCq6x5w'
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  id: 'CRL4vfW4U-mz54PCAZOOM4mZ6loCegMMBxOEIff33leyrNqPxpTmj93mkrN_sE5vZCDPQpoDWStwGS_ZCq6x5w',
  value: { value: 100, type: 'Integer' },
  active: { type: 'Boolean', value: true },
  name: { type: 'String', value: 'Updated Record' }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: iDiCCpyJQyKUjXpLFnsftQPSno3ibLPRdMZ8lyGwWDQfkX1MtlnPuIrSUi-R1w8f4Flw4pNP5CHCyIZXsfTSYg

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
Collection created with first record: 14bnBS87sH1E8huj8Cd1FynWXhTDVQHWMVLU2oWKQyVahjQoh-k1CY3YTwxWPtH_RjxfEEB9VamdtuEM3u5hwg

=== List Collections ===
Total collections: 22
Sample collections: ws_ttl_test,schema_users_client_ts,test_collection,chat_configurations__ek0_testing,schema_documents_client_go

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
cache:product:3: { name: 'Product 3', price: 49.989999999999995 }

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 0 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 12

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: false

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: l_W5Sx7BeFnylH58-PxMT4Tc5FTua2oqo2yV-AMmKhF8Kje7gbXhL3RK1wJbtFuHLucxccU6f8TF_ixkhFWkRg
Created Bob: $500 - ID: HW80O3DZ0IMR-_8wdMhgYxEDKNrkCf4gqg49P2yoHrkN4Gnygld82fHPDyVwLmuJp2ObtogJstAW-SoDgViHzQ

=== Example 1: Begin Transaction ===
Transaction ID: 6b9fbca6-3d1e-4576-9692-6daa12a47e9e

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: c9e91d57-da07-4d21-938f-469f3ddf89e3
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
  1. Score: 26.400, Matched: bio.value, title.value, title, bio
  2. Score: 26.400, Matched: title.value, title, bio, bio.value
  3. Score: 26.400, Matched: bio.value, title, title.value, bio
  4. Score: 26.400, Matched: title.value, bio, bio.value, title

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title, title.value, bio, bio.value
  2. Score: 39.600, Matched: bio, title.value, title, bio.value
  3. Score: 39.600, Matched: bio.value, title.value, bio, title
  4. Score: 39.600, Matched: bio, bio.value, title, title.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio.value, bio

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio.value, bio

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.754, Matched: 
  2. Score: 0.740, Matched: 
  3. Score: 0.732, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.693, Matched: content, title.value, content.value, title
  2. Score: 1.501, Matched: title.value, content, content.value, title
  3. Score: 0.296, Matched: 

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
✓ Inserted document: 1oZXLXCddhcm4AmBXe9qXTtSJ6zY6FntIrIoioOf_wvOuBp9b1Kyy-fOMzMFp5O2IsJ-Bqp0kCW2lT8X3NMZTQ

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: Nzq19wj-FTdBjuq77qNn8gRfzjLoquR6x3Z_W7jH0ZJpn42075jMjkWguCzy0flU9_Cl-VG1CbkNiyphr0xJsQ

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
✓ Inserted document with TTL: dtnXeA7IGmqnu7lWKft7qW4CRDIxZ-fV5OdQlck81stiVzhPHnDjylku5StdIyC-rRIjxZjrCTTajHumgrfI4A

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
✓ Edge cache script created: mta3NiVlx8UyrH-3WYWeaIZxlaQQYz-LdrRry2vVhqMWyktNSOzxNoCrGHP-oEPgM65Z4zgaT3kGg84wFubE8A

Call 1: Cache miss (fetches from API)
Response time: 5ms
Result: {
  "records": [
    {}
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
Response time: 4ms (1.3x faster!)
Result: {
  "records": [
    {}
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

✅ Script saved: QDSH4--dXM3lDQAJ10MtIzI5Y-NdiaXuPM6uGOGT2XAnT2B513Kyq_8RqSfGKSnCE3UVq9ITsihGM7EEEEKSSw
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
   {"count":5,"avg_score":60,"status":"active"}
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

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 4ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "company": {
      "catchPhrase": "Multi-layered client-server neural-net",
      "bs": "harness real-time e-markets",
      "name": "Romaguera-Crona"
    },
    "id": 1,
    "addre...

Second call (cache hit - from cache):
   ⏱️  Duration: 4ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "company": {
      "catchPhrase": "Multi-layered client-server neural-net",
      "bs": "harness real-time e-markets",
      "name": "Romaguera-Crona"
    },
    "id": 1,
    "addre...
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

✅ Script saved: KJJnyYhHMqjWM1qoNKHdR-ejjIa6NDIp_y_lti4GOHgIxZt2AU7HbVtWpCPNDUJlFO-FUhPwgU9uL6VButmPpQ
📊 Found 2 product groups
   {"category":"Furniture","count":2,"avg_price":474}
   {"category":"Electronics","count":3,"avg_price":575.6666666666666}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved
📊 Found 2 categories
   {"category":"Electronics","count":3}
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

✅ Inserted order: qn2qQaNdTk5J4upKGZhfIWQfxtYB6wbLUPJG15_x61hDKVJPTp9wKF--VJOC6_Ck5FBXpNeHdgQM6HjyJIzU1w
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: LCDi1c_c03on28W2BX_wqcYMTWwncXbtav46BjHe_YNXB_QHeopEyHE0W8MIbOBHJ_RsnqxutTOGRXA4QLz3mQ
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"userId":"user_abc","role":"admin"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: Y5TEsWImhmwUrKcmeZBqHOpkKl7J1cB4wbt40P6ggED98hDqOkHgbyiqwyBS3MGgc29KcIpXCZhjOAFNB1QdEQ
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Script

✅ Script saved: uQgtrbvjpiFjusLkgTutVJdbwfFA_rTmu3G204pxrAf_LWsfoDtzT9-rsDC0ptX7lbeMUjVXWKBWSIrTWJXGwQ
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
📊 Users (showing first 5 of 5):
   1. User 5 - Score: 100
   2. User 1 - Score: 20
   3. User 4 - Score: 80
   4. User 3 - Score: 60
   5. User 2 - Score: 40
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
   1. Getting Started with ekoDB (Database)
   2. Vector Databases Explained (Database)
   3. Introduction to Machine Learning (AI)
   4. Natural Language Processing (AI)
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
   1. Getting Started with ekoDB
   2. Vector Databases Explained
   3. Introduction to Machine Learning
   4. Natural Language Processing
   5. Database Design Principles
⏱️  Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Script saved
📊 Projected documents (showing first 3):
   1. Getting Started with ekoDB
   2. Vector Databases Explained
   3. Introduction to Machine Learning
⏱️  Execution time: 0ms

📝 Example 5: All Document Fields

✅ Script saved
📊 All documents (5 total, showing first 2):
   1. Getting Started with ekoDB (Database)
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
   1. USB-C Cable - $19 (⭐4.3)
   2. Laptop Pro - $1299 (⭐4.8)
   3. Office Lamp - $79 (⭐4.05)
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
   Vector databases have several advantages:

1. Precise Geometry: They retain the exact shape of spatial features even when zooming in at high levels.

2. Attribute Storage: They allow for the storage of additional information (attributes) about each spatial feature.

3. Topology Preservation: They can preserve spatial relationships ("topology") between features, such as connectivity, adjacency, and containment.

4. Space Efficiency: They are more efficient in storing simple, sparse, or irregularly distributed features.

5. Flexibility: They can represent data at its original resolution and form without generalization.

6. Scalability: They are well-suited for applications requiring detailed analysis and complex queries.
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
✓ Created SWR script: fetch_api_user_js (hAlomqWFZa6Os9aiKkoQ8-X4HBJXWYNSEJQC0Z7Jd42dqw1Svy0kYIr1mDYotqqsjAeUf4mNasozlyeoBVsxyA)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {}
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
✓ Created session: KCFF9QV0zVxaipcxXs1ZHcPL6Xnc_F_KtUTC8bCLIBBRtn9PA1qE4syhKqR7ODVR0QFjn3t48gEXipDW5dx4Hg

=== Sending Chat Message ===
Message ID: 1N7qx2z5LjPwo6ER-LYb3IY4i6nuSRDMXa0ToO18Fmv3v1mhkXM59SY_wDeyfYgIC9M1JpX2kuWLMZwAX9mi_Q

=== AI Response ===
We have three products available:

1. The ekoDB Cloud, which is a fully managed cloud database service product. Its price is $499.
2. The ekoDB, which is a high-performance database product with AI capabilities. It is priced at $99.
3. The ekoDB Pro, an enterprise edition product with advanced features. This is available for $299.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_js',
  record: {
    price: 499,
    description: 'Fully managed cloud database service product',
    name: 'ekoDB Cloud',
    id: 'YjbQf5MTI-S4YyRp8qLodiifJC6yyVm8PORzziJy0e7caVykuoW8KMEkbbVZO7asbmNJ8_b06VOTatS_6I7ZlQ'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_js',
  record: {
    name: 'ekoDB',
    id: 'V4y6evCjFeOmM2KqsqCcgciHChAVMJEisaDjEtbwVDtUa2KRka0mPM_l8RIEXJNMqzKtO_MJudz2rsJ5aLhiHw',
    description: 'A high-performance database product with AI capabilities',
    price: 99
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_js',
  record: {
    price: 299,
    description: 'Enterprise edition product with advanced features',
    id: '15LHZWfmqGquJxJP4s3HVi50hfzagWlK5LDSodfhIeB4DGKpRmh4hibjUvfZ749JLf8QTHz6Rf7OvZqvZgvwUg',
    name: 'ekoDB Pro'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 2645ms

=== Token Usage ===
Prompt tokens: 450
Completion tokens: 78
Total tokens: 528

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: gPAwJeLEFAQkAr8shKANBuwFLMe88RQya9q3aj4X_FD9dEqbRLZUz1o9gS5ZKTKs6LaktU7-QOF7_Nv6eHCT2w

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
✓ Created second session: cPM6c5wNcs62Rgg2vMA_nUN0_noyJG6-63pxMGUbyGwfJ88NDi8p46DBMZy7O9F8MJVTj4N-XeEGFI6kK8zx4A
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
✓ Created session: s2-q7yoJd0_NdvAIX7prHdUgAKmtiz5LMNPDOGvkiFuGB-4Ex5a8yTjIcnT_TpOGvd__hKK28zyE_1bgiTIFFw

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information provided, the available product is "ekoDB". It is a high-performance database product priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 3wBQw-BEsLAKsI3HhXu1y2Ky8-9lCPkAC0SVp8AK9feH63ck9JZQpKisN4Sjqg5Tivh4R9c2njH5L-gGG5vkbw
  Parent: s2-q7yoJd0_NdvAIX7prHdUgAKmtiz5LMNPDOGvkiFuGB-4Ex5a8yTjIcnT_TpOGvd__hKK28zyE_1bgiTIFFw

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: 3wBQw-BEsLAKsI3HhXu1y2Ky8-9lCPkAC0SVp8AK9feH63ck9JZQpKisN4Sjqg5Tivh4R9c2njH5L-gGG5vkbw (Untitled)
  Session 2: s2-q7yoJd0_NdvAIX7prHdUgAKmtiz5LMNPDOGvkiFuGB-4Ex5a8yTjIcnT_TpOGvd__hKK28zyE_1bgiTIFFw (Untitled)
  Session 3: cPM6c5wNcs62Rgg2vMA_nUN0_noyJG6-63pxMGUbyGwfJ88NDi8p46DBMZy7O9F8MJVTj4N-XeEGFI6kK8zx4A (Untitled)
  Session 4: KCFF9QV0zVxaipcxXs1ZHcPL6Xnc_F_KtUTC8bCLIBBRtn9PA1qE4syhKqR7ODVR0QFjn3t48gEXipDW5dx4Hg (Untitled)
  Session 5: O9s_bNhs54ht7KpAhRJeLLCYKrHkl1tfsWMmIMGornTRtPrFV9uIQ35woUTAjqqFuPVgfpb7-pVhHjOXGhw9-A (Untitled)
  Session 6: dKl084XOstr2VQgTKEYgeN8z-0xEMvJz0Nu1HCkrpzBbUfp6iTYdtIE79Z7GDnB6i99HlDZ5Qa8zegoGHyhtmQ (Untitled)
  Session 7: iROQ2JQ4r-HnxeynoBlz2KcWUCPcQWRtRj9k3KNE1uyiak-cJw_WhhGT4Q2EYUMf-Obas2TnD7CkUZCZzI8mcA (Untitled)
  Session 8: M9ADsELP71bK8H9eVTTm70HNQsJjIwaOy8a8JNwQ3F-33q26Vd95gmZZRp1X6oaoJpT1dfguwe9mVlq8wIPF3w (Untitled)
  Session 9: LfxcAjSnqvQbHtrXI6GQFVFJ4iItWlvXOyC_AE-bc4lY_Jq9FZVUs9XeBwmZxf6v5e8mRkJiFUlwD8aaU50iqA (Untitled)
  Session 10: 5SXXYkac4oI8ZD6t96ysIzlR4gmK3V7-R1Iwtf2MJGfi98pnTNB6zu2F6x0ZDv_w28kkWW9OAnBPVRKv9l6Mqg (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: 3wBQw-BEsLAKsI3HhXu1y2Ky8-9lCPkAC0SVp8AK9feH63ck9JZQpKisN4Sjqg5Tivh4R9c2njH5L-gGG5vkbw

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
✅ [32mJavaScript client examples complete![0m
