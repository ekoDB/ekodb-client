make test-examples-javascript
🧪 [36mRunning JavaScript examples (direct HTTP/WebSocket)...[0m

added 1 package, removed 1 package, and audited 9 packages in 807ms

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
  id: 'meaQKomkr-VsHGC3gy8EScsO57aLYmnoPN5BMud4S8OHu8WmtctfSHJ17KQ_Ob5A0wc7QeaVhkisY7CAVELyPg'
}

=== Find by ID ===
Found: {
  name: { type: 'String', value: 'Test Record' },
  value: { value: 42, type: 'Integer' },
  active: { type: 'Boolean', value: true },
  id: 'meaQKomkr-VsHGC3gy8EScsO57aLYmnoPN5BMud4S8OHu8WmtctfSHJ17KQ_Ob5A0wc7QeaVhkisY7CAVELyPg'
}

=== Find with Query ===
Found documents: [
  {
    id: 'meaQKomkr-VsHGC3gy8EScsO57aLYmnoPN5BMud4S8OHu8WmtctfSHJ17KQ_Ob5A0wc7QeaVhkisY7CAVELyPg',
    active: { type: 'Boolean', value: true },
    name: { type: 'String', value: 'Test Record' },
    value: { type: 'Integer', value: 42 }
  }
]

=== Update Document ===
Updated: {
  value: { type: 'Integer', value: 100 },
  id: 'meaQKomkr-VsHGC3gy8EScsO57aLYmnoPN5BMud4S8OHu8WmtctfSHJ17KQ_Ob5A0wc7QeaVhkisY7CAVELyPg',
  name: { type: 'String', value: 'Updated Record' },
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
✓ Inserted test record: PgUxCDk4BXq486r8u760k9x_XS0xrv0eSw92gL4TLtQKRoZm5tHOvPX1s75W6w7W-tp6_3B32i7psm694fLjxw

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
        "id": "3iNacHlOrLs5T9hDNOvghQkwkRQEp81IZ51fafX7mvm0W7-T_AZYHMyPzDRppGPdUb895ykT2KkFB-AigS1bvA",
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
        "id": "Sb7wwI0SJG9yEJuFRPtwizcAVp3BIN9p3UyfWjfyM5n9cEdWvf9ZUo0KW0Ec26o-aNqu9lX6Zba7BE9ObFt3Rg",
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
        "id": "qGW4J1Pg9H03R3PLX-rH5pCQ5Kk6cktQrBW9s5inyjsDyAyHo0oL8sDMevixpuqZB4_oGrcGHPaL30-4f3GtKA",
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
        "id": "Zc7PqlMFMO49_B56vubOWFv8H9g5CTInhjVIV0lpLko9dqy1wx-tOd6_XeZVKvkAupa_wFsi-bkSpqVKZi10nQ",
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
        "id": "NAx3s7Ff8DkZt9PSpE9wpo5Z2NtasI9oiUQGokyIYHkCk3StEiTomYqroVm1eSjDnAGZun89TDtqHNbwBnrvSg",
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
        "id": "biFqh34zaiKvmVm_AsobaPP65GYmXSLsFcsLJGV9TsX701PhiY4YQht5Ic36wfZ3pL3js_GmW27bm485ujU8TQ",
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
        "id": "mWL_XtxbXMBH3hWNMk9n3vMoBGe_1M5OEi7hTywqjnKnkJ0V2cgt-RIlZwazqu4TZoKlkSyciuKWunFUsgBttw",
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
        "id": "PgUxCDk4BXq486r8u760k9x_XS0xrv0eSw92gL4TLtQKRoZm5tHOvPX1s75W6w7W-tp6_3B32i7psm694fLjxw",
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
✓ Retrieved 8 record(s) via WebSocket

✓ WebSocket example completed successfully
WebSocket closed
[32m✓ simple_websocket.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/http_functions.js ===[0m
🚀 ekoDB Scripts Example (JavaScript/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: 2oJ_DGsFwKpd6Y56RSMccgwcbNCw-rdOusMXHGXpSiZV1DI6E_YblsBVgs-WTP1tP-XG9Q8t9amGDn634I5i4g
📊 Found 10 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: khqHpksM2Y4dAlzDH0m5TyQT73UfI75oT5NR9Hs2MXx2qENH0sEVfJzkCvzVaACvrmCpr-zXCCagBRxwxhLpuw
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: y9_2Rnx8nwZswrAQ7dbA8wPeCU8SVhIcJfsY5NmnFs7iDqD6pUttQuBTOBYUg307h7IYrCqSfrQv5ZBcgHVIkA
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"max_score":100,"avg_score":60,"status":"active","count":10}
   {"status":"inactive","count":10,"avg_score":50,"max_score":90}

📝 Example 4: Function Management

📋 Total scripts: 99
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
✓ Verified: Found 40 total records in collection

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
Collection created with first record: JqfcBZNSyLfNA5-_HQhIPk2VybfgXAtzdhz7DCm4Y1LsEB3TLxB5gpCGMiQNGV0yjWaaUn2JNn1PxoHC8jmIKA

=== List Collections ===
Total collections: 36
Sample collections: [
  'schema_documents_client_go',
  'ws_subscribe_example_go',
  'schema_employees_client_js',
  'schema_products_client_js',
  'chat_configurations__ek0_testing'
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
Created Alice: $1000 - ID: SWUZC7iTbvvVsn6DGmaObPV4AtLTw57PUFSc8n-W3kK-FBy2D9lq0a809p_YdVGvoK1F098OTzqpvr5mpZK4tA
Created Bob: $500 - ID: oCT4lRSsYyFUgn_RBHv-djNk-zhZCb3rWaaxcQmfTzbe-LGSnXv8qwf3UK5hvRUcye7QP0SXZP9e9_256Uq7vQ

=== Example 1: Begin Transaction ===
Transaction ID: 7aee23cc-b825-41bf-ba27-c9ad7c1aca43

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
New transaction: 8837c010-2010-4ff4-a181-982c811943f6
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
   ✅ Script saved: 1Wbp_WscWgTQzhL3ssxJQAygEx5d6Ow5Avz_APlCa86j4PcKRf5x87S9bYVsRpAoa99IwLW4ToVBe2N3uQQ9HA

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
   ✅ Script saved: 3vniXhE5E_mSJmpPc6KP9qTU1IbPtcV7fZzKihyMNnPpoYrp1Mt3bC-YgsYJR81dRu5vEB2ciTY9P2oJDQhJ2w

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
   ✅ Script saved: PuNiX_me4VpN1W3tkwM7Xbni0VJ-CSo-dIMNpJWhyUziK-kPZ9m-kNVtKtcdLqnpPYRh2iedfrptEv_VluzteA

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
   ✅ Script saved: Tarto1wTEbWPcVC82zLfsQVKXtYN7Tm_9lWUik2doxC-BR2qwQYAbzNu3zEcwMjDi1aGl-pE_Qt_5426yYb-Tg

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: 1Wbp_WscWgTQzhL3ssxJ...
   ✅ Deleted script: 3vniXhE5E_mSJmpPc6KP...
   ✅ Deleted script: PuNiX_me4VpN1W3tkwM7...
   ✅ Deleted script: Tarto1wTEbWPcVC82zLf...
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
  Output: Document ID = DfClYm2wydA91XLVdq0LXhsZSfEIcjizWtMsJCW5hoEn580SRKJGPOdzj8Foja7Ys7FkrfMkOZJir4aA80g-TA
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: findById(DfClYm2wydA91XLVdq0LXhsZSfEIcjizWtMsJCW5hoEn580SRKJGPOdzj8Foja7Ys7FkrfMkOZJir4aA80g-TA)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: findById(DfClYm2wydA91XLVdq0LXhsZSfEIcjizWtMsJCW5hoEn580SRKJGPOdzj8Foja7Ys7FkrfMkOZJir4aA80g-TA)
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
  Output: Document ID = PuDba_1KZWo2R8YaSpVGe7o6KBMhPdgjQfJDtEgEmZsbn-Qi8fcnWWw1tYJPTV85NA-uGVaDflr2RlOYpKFjZA
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: findById(PuDba_1KZWo2R8YaSpVGe7o6KBMhPdgjQfJDtEgEmZsbn-Qi8fcnWWw1tYJPTV85NA-uGVaDflr2RlOYpKFjZA)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: findById(PuDba_1KZWo2R8YaSpVGe7o6KBMhPdgjQfJDtEgEmZsbn-Qi8fcnWWw1tYJPTV85NA-uGVaDflr2RlOYpKFjZA)
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

> @ekodb/ekodb-client@0.11.0 prepare
> npm run build


> @ekodb/ekodb-client@0.11.0 build
> tsc


up to date, audited 48 packages in 1s

14 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.11.0 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning JavaScript client library examples...[0m

added 1 package, removed 1 package, and audited 9 packages in 813ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: '_p4jUW0ltROvnWGJY-lUBvX73wTwKb2InRRQ0pRT01eOD_ez0yXC575VDIy6hdOXWBDvYpxzOBwVcD8rclRCAA'
}

=== Find by ID ===
Found: {
  name: { type: 'String', value: 'Test Record' },
  value: { value: 42, type: 'Integer' },
  id: '_p4jUW0ltROvnWGJY-lUBvX73wTwKb2InRRQ0pRT01eOD_ez0yXC575VDIy6hdOXWBDvYpxzOBwVcD8rclRCAA',
  active: { value: true, type: 'Boolean' }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  name: { type: 'String', value: 'Updated Record' },
  id: '_p4jUW0ltROvnWGJY-lUBvX73wTwKb2InRRQ0pRT01eOD_ez0yXC575VDIy6hdOXWBDvYpxzOBwVcD8rclRCAA',
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
✓ Inserted test record: nhx5YT2B_s-LKjZ-3KQcbk3y9KJ57P1V-nzk4pbrbViQR3PCXc_OXQVmUc0Qu7VJpJRB8dxzjEoKnRQ71ttuYQ

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
Collection created with first record: JsM12VkoSmgn2hE460QsiaKVL82c_DrW_-ChKvnHfRRpTI3xBqWnrHM2eZsFmcr6MkZo9gY-Y2jdTkePg7FuSQ

=== List Collections ===
Total collections: 35
Sample collections: client_collection_management_js,schema_documents_client_go,ws_subscribe_example_go,schema_employees_client_js,schema_products_client_js

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
Total keys in store: 18

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: false

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 6tet-FlhiL25XIQOXDOZMo3JU9dy8IYod3KdYtik2f0Yx-YDx66KKczurjZRR1zLDQQsvMS5So7XFJ5g6AISxA
Created Bob: $500 - ID: 43kkJsq0kHe1q49tZak66RxEbdz7Jua-phalHvfU9g5TJhd6P7C3p2xAR1RqbCxz-o2eldg1ngEPr2SiPyQjfQ

=== Example 1: Begin Transaction ===
Transaction ID: ae7b624d-84ec-4386-ad3c-487d7a975f57

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: c7303eba-b37a-4d47-9258-f691678ba4a2
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
  1. Score: 25.740, Matched: email.value, email, name, name.value
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title, bio.value, bio, title.value
  2. Score: 26.400, Matched: title, title.value, bio, bio.value
  3. Score: 26.400, Matched: title, title.value, bio, bio.value
  4. Score: 26.400, Matched: title.value, title, bio.value, bio

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio.value, title.value, title, bio
  2. Score: 39.600, Matched: bio.value, title, title.value, bio
  3. Score: 39.600, Matched: bio, title, bio.value, title.value
  4. Score: 39.600, Matched: title, bio.value, bio, title.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio.value, bio

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.762, Matched: 
  2. Score: 0.747, Matched: 
  3. Score: 0.743, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.699, Matched: title.value, content.value, content, title
  2. Score: 1.497, Matched: title.value, title, content, content.value
  3. Score: 0.305, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: bio.value, skills, bio, skills.value

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
✓ Inserted document: Jg6bjeUt6vEQ_bOQL7bVPgEOYHE4yHdP-taX6vc1Pq9_MIRopZPPovr2ahPv26HizLnqpWlLEj9hJqTta3LPVg

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: 0Aqt_GjLdNJ6s9xhQnCVl6PQdYcEi6eTraw4n8oKkf_zqTafevzs50QTSIyCBfrzaUxtDJYy9VUq0KMjXVl7Lg

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
✓ Inserted document with TTL: z9Mk48bSzoVksa0BxKefouTjWxjGDCFIAoF7KQNnYbY4FmMxz7FtdIUDIEuwJSUZlkMsMU0t4GBM2XykSlIQVA

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
✓ Edge cache script created: DfY8WQPiMfaoMcaa-LBy1gtQ0K7Lu5DL95IcbrstE6Q099zZRYtfkdLB8N3GIkiirxbyyCSjtf0Ot-qHsNE4bQ

Call 1: Cache miss (fetches from API)
Response time: 6ms
Result: {
  "records": [
    {
      "value": {
        "timezone": "GMT",
        "latitude": 40.710335,
        "timezone_abbreviation": "GMT",
        "elevation": 32,
        "longitude": -73.99308,
        "utc_offset_seconds": 0,
        "generationtime_ms": 0.023365020751953125,
        "current_units": {
          "temperature_2m": "°C",
          "time": "iso8601",
          "interval": "seconds"
        },
        "current": {
          "temperature_2m": 16.3,
          "interval": 900,
          "time": "2026-03-11T05:00"
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
Response time: 6ms (1x faster!)
Result: {
  "records": [
    {
      "value": {
        "timezone": "GMT",
        "latitude": 40.710335,
        "timezone_abbreviation": "GMT",
        "elevation": 32,
        "longitude": -73.99308,
        "utc_offset_seconds": 0,
        "generationtime_ms": 0.023365020751953125,
        "current_units": {
          "temperature_2m": "°C",
          "time": "iso8601",
          "interval": "seconds"
        },
        "current": {
          "temperature_2m": 16.3,
          "interval": 900,
          "time": "2026-03-11T05:00"
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

✅ Script saved: UYSKaMTFdlTdkNXvvL2_44quEXQxuZeA4PZy1hv5IWhgarRyTsBUKtD9pg_1c8PFvPi0zJsdj5SBcyyKP6M1Iw
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
   {"count":5,"status":"inactive","avg_score":50}
⏱️  Execution time: 0ms

📝 Example 4: Script Management

📋 Total scripts: 102
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
   Department: {"value":"engineering","type":"String"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 5ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "address": {
      "street": "Kulas Light",
      "geo": {
        "lat": "-37.3159",
        "lng": "81.1496"
      },
      "suite": "Apt. 556",
      "city": "Gwenborough",
     ...

Second call (cache hit - from cache):
   ⏱️  Duration: 5ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "address": {
      "street": "Kulas Light",
      "geo": {
        "lat": "-37.3159",
        "lng": "81.1496"
      },
      "suite": "Apt. 556",
      "city": "Gwenborough",
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

✅ Script saved: _DfoPDxh6iTTcjMIjbhcbbD7IyTtDG2IyqJG7JzPTRoY0Lv6nqtj9-ABvns8gJmzdjl9ZuBfMSl5JQZK-husxA
📊 Found 2 product groups
   {"category":"Electronics","count":3,"avg_price":575.6666666666666}
   {"category":"Furniture","count":2,"avg_price":474}
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

✅ Inserted order: cz1Ex0jSr7IORPT_q6uax1xbLKWBaZAMupoBFjtzeogjGXKvKDG6hUP0v1ZNAwCfHZ3LzKK30jqUgWPmunTo3w
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: hyaYcq7-CqdAq8KNA94HfU6DliqkOtZrJB82e2H4bR_JBd8dErAtI2cNHuwR9lzyvnr1nhObtipcZcA_yWP-9w
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"userId":"user_abc","role":"admin"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: jN3JCwxmx5FOkrlqPKCbz_go9443FBxDkp08VkRRIRt_tGbss4PjDue5JmoNAp8DSttP20NxvcosKYmFlbIuJg
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Script

✅ Script saved: tqu3GVE1AsFAa2Ki6iSPOPl4eYTf3Z5fY7Ht-LLjrU0xY-Mky_jRgLGdQZpQXRaV0RyCigCCn87Tg4nBHRCT3w
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
   1. User 1 - Score: 10
   2. User 5 - Score: 50
   3. User 10 - Score: 100
   4. User 4 - Score: 40
   5. User 8 - Score: 80
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
   1. Natural Language Processing (AI)
   2. Getting Started with ekoDB (Database)
   3. Introduction to Machine Learning (AI)
   4. Database Design Principles (Database)
   5. Vector Databases Explained (Database)
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
   3. Introduction to Machine Learning
   4. Database Design Principles
   5. Vector Databases Explained
⏱️  Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Script saved
📊 Projected documents (showing first 3):
   1. Natural Language Processing
   2. Getting Started with ekoDB
   3. Introduction to Machine Learning
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
   2. Monitor 27" - $399 (⭐4.6)
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
   1. High Precision: Vector databases give high accuracy and rich geometric details by representing spatial data using points, lines and polygons.

2. Scalability: Vector models can be easily scaled. You can zoom in and zoom out without losing the quality of data representation.

3. Less Space: Since they store data as points, lines, and polygons rather than grid cells, vector databases often require less storage space.

4. Representation of Data: Vector databases are excellent for representing discrete data like highways, boundaries etc. and topology (relationships between features).

5. Attribute Linkage: Vector databases allow easy linkage to non-spatial descriptive attribute data. 

6. Editability: Features in a vector database can be individually edited as they are distinct entities. 

Remember, however, that the choice between vector and raster databases often depends on the data characteristics and the specific application needs.
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embedding script saved
📊 Generated embeddings for 3 articles
   1. "Draft Article" - 1536D vector
   2. "Getting Started with ekoDB" - 1536D vector
   3. "Advanced Query Patterns" - 1536D vector
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
✓ Created SWR script: fetch_api_user_js (fJUXLAZ-brQs3fyKazEXHUvdlfvRsrxr3s7InR6oAzx0MFKAgVGNOf5lqPBsobhxOBRrRVDZEBIbC5e-81FecQ)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "name": "Leanne Graham",
        "address": {
          "geo": {
            "lng": "81.1496",
            "lat": "-37.3159"
          },
          "city": "Gwenborough",
          "suite": "Apt. 556",
          "street": "Kulas Light",
          "zipcode": "92998-3874"
        },
        "website": "hildegard.org",
        "phone": "1-770-736-8031 x56442",
        "company": {
          "name": "Romaguera-Crona",
          "bs": "harness real-time e-markets",
          "catchPhrase": "Multi-layered client-server neural-net"
        },
        "username": "Bret",
        "email": "Sincere@april.biz",
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
Response time: 13ms (served from cache)
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
✓ Created native SWR script: github_user_native (WysGQhNroYoe6eKXNhhZBQV2-snHjSBz8fZriG-wCdPmTGXnSdLc-UfjanVtdQYbBE9GNVAPTGSQxzfzG5v98A)

First call (cache miss - will fetch from GitHub API):
  Response time: 6ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 7ms
  Speedup: 0.9x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (5BR_HqkjnuajqXD_wpTW0RAPTh3R3TCX5SpK_uPzOqZZYy1A_msAyVWZU7vZ_pRTX9kG-BtxEmuD992BE2GVFw)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (TcgMvOcpOxklkq8FjaQPRoUJmYbyJKGog6UPziLNN3VL0NZ7U0sW1DaKqjHwq6WfyQhi8O4aUGE5-ZTS4yncew)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (Ql-LtXzg_o2FzFbE2Ms6IJS5xaHPPU0GKOyoIvPYbYzMjHzMLOdpdLC8VG2tNOwy8p04zfkTMHQ9ng0sya2PXQ)
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
✓ Created session: Nfvj2AzUSrZzN7HGED2cCurVwN6aQKy02RtBSMVXEcMT0GRDZypeIZz4wjiyXFuTdTDaLH7GSmibMiYCY77T1g

=== Sending Chat Message ===
Message ID: Q0Ws7uEky_W0xemO_ZrWUCM8Y9HCDpZtp3ayf2YlFcaAw15gCk4xKuYpU1NiEflFpgeNPzsM0KvSYIzLFaYAGQ

=== AI Response ===
Three products are available:

1. ekoDB Pro, which provides advanced features, is priced at $299.
2. ekoDB, a high-performance database product with AI capabilities, is priced at $99.
3. ekoDB Cloud, a fully managed cloud database service product, is priced at $499.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_js',
  record: {
    id: 'rUCaZYf6PKbUXa4rOEG3S-m7GMgHjTb_tH5hTZp2AxvTtp8_cILKD6uhYMyGqU1XC6VkbM1cZ0XZ32Al-YXhaw',
    name: 'ekoDB Pro',
    description: 'Enterprise edition product with advanced features',
    price: 299
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_js',
  record: {
    price: 99,
    description: 'A high-performance database product with AI capabilities',
    id: 'ZvwEmnMIbnISJnTT84ZSOEASL0t63mJlgEEehCTFxQtM8NH89orGYUO1ch-Olryo9hCurmg2jNtZHaVtg91Rtw',
    name: 'ekoDB'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_js',
  record: {
    id: 'uv8h52J1V5iJ73zAXzD9wprDL5gguLC6Hzo5qs0Qx71sKC79wfX1H4KASn2YnlwmhuL6u2vJO2Rt8Qj69U-vUA',
    name: 'ekoDB Cloud',
    price: 499,
    description: 'Fully managed cloud database service product'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 4042ms

=== Token Usage ===
Prompt tokens: 450
Completion tokens: 65
Total tokens: 515

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 2ceHrBpC0zkPNK7ffb-KrAcldGlWo3B2ucHn5hCoSv2kMJ_DWu8JPGMXhc6jNPj-c8FM6MwjhBNni2dXOGV7dA

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the context provided, the available product is a high-performance database product named "ekoDB". It costs $99.

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
✓ Created second session: eQffRKfFNTFxQPZSH2UKAch5VQcwQk224FUJzCDiM0e0hmA7z52LF6AoxcHExaI22Wg30cEobCCgw3YBZkW85w
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
✓ Created session: pMDQVmz7-foxBuYBdSzUNeLHteCH3aQDl7ynYByTmkb815JOE-5maBtxIalk5WY9cTxqk9KdgY6W--c7TtV5og

=== Sending Messages ===
✓ Message 1 sent
  Response: Currently, we have the "ekoDB" product available. It's a high-performance database product priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: k-a7XJtSkc9e0cW_2kt7wzCLPr0iMP_z8PPGTwuSFGQcT4sxTaPJSDVx0G5XHBxrC7HAGo4q6RCBU2Dk-6yIqg
  Parent: pMDQVmz7-foxBuYBdSzUNeLHteCH3aQDl7ynYByTmkb815JOE-5maBtxIalk5WY9cTxqk9KdgY6W--c7TtV5og

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: k-a7XJtSkc9e0cW_2kt7wzCLPr0iMP_z8PPGTwuSFGQcT4sxTaPJSDVx0G5XHBxrC7HAGo4q6RCBU2Dk-6yIqg (Untitled)
  Session 2: pMDQVmz7-foxBuYBdSzUNeLHteCH3aQDl7ynYByTmkb815JOE-5maBtxIalk5WY9cTxqk9KdgY6W--c7TtV5og (Untitled)
  Session 3: eQffRKfFNTFxQPZSH2UKAch5VQcwQk224FUJzCDiM0e0hmA7z52LF6AoxcHExaI22Wg30cEobCCgw3YBZkW85w (Untitled)
  Session 4: Nfvj2AzUSrZzN7HGED2cCurVwN6aQKy02RtBSMVXEcMT0GRDZypeIZz4wjiyXFuTdTDaLH7GSmibMiYCY77T1g (Untitled)
  Session 5: KGBDqDGylzhGPBadcvcxKjwKm1c0HIQuj7KCZBiJsqJ0FvUe30Koi_my84mDI6vhvjE8tWdYtzfO2VbTA8JlZQ (Untitled)
  Session 6: syUDDGcZanFnWH2riQW4JI3JS9dnexrjHs1WEZhl-c7SMR4Orsuvdb0mRBuYipEQ2tdS_14sQ7ULTPjT1QWMfQ (Untitled)
  Session 7: 7WzWaO-_k65ftagvlYbGTUUc2IPgFZtqzaQN-3OdyiQvO5ihTm739mmGE9OaWVBA14UuxWP_HCCgVjksuecArA (Untitled)
  Session 8: DRavkpC7grjiyHyWQvY0pvKVm_ThJKy8IfyMiBUcirGFFBVw_JFsljqhMbqIm0n9yBrzSDuAhqi334Cx-dGsWw (Untitled)
  Session 9: c78LckyBlkvvU8eQYK4J2nLlh7fIdaD4jFfCR3rEp5KVSc9YldexJ16rwzaA_4I85FDxhbt-21k3NxhU5T1GRg (Untitled)
  Session 10: zsfTJPjo8BJW27P0qEtZ8_a5UhoJvv0n3b2HbNwaOCwU7Nov_kWwz2BRBkGtPwbeYUV9TGNROtHpCsO1L338oQ (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: k-a7XJtSkc9e0cW_2kt7wzCLPr0iMP_z8PPGTwuSFGQcT4sxTaPJSDVx0G5XHBxrC7HAGo4q6RCBU2Dk-6yIqg

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
✅ [32mJavaScript client examples complete![0m
