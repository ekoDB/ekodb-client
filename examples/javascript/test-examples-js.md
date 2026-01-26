make test-examples-javascript
🧪 [36mRunning JavaScript examples (direct HTTP/WebSocket)...[0m

added 1 package, removed 1 package, and audited 9 packages in 824ms

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
  id: 'YjNc33NKh-WnY6BQVDeMt3G_OAgCRrLjoWKTB2VQGGCzFlFtUcaqOBL8v44lt4nUJEcs3aXcH5YQMlmgegMt5Q'
}

=== Find by ID ===
Found: {
  name: 'Test Record',
  active: true,
  value: 42,
  id: 'YjNc33NKh-WnY6BQVDeMt3G_OAgCRrLjoWKTB2VQGGCzFlFtUcaqOBL8v44lt4nUJEcs3aXcH5YQMlmgegMt5Q'
}

=== Find with Query ===
Found documents: [
  {
    id: 'YjNc33NKh-WnY6BQVDeMt3G_OAgCRrLjoWKTB2VQGGCzFlFtUcaqOBL8v44lt4nUJEcs3aXcH5YQMlmgegMt5Q',
    active: { type: 'Boolean', value: true },
    value: { value: 42, type: 'Integer' },
    name: { value: 'Test Record', type: 'String' }
  }
]

=== Update Document ===
Updated: {
  active: { value: true, type: 'Boolean' },
  name: { type: 'String', value: 'Updated Record' },
  id: 'YjNc33NKh-WnY6BQVDeMt3G_OAgCRrLjoWKTB2VQGGCzFlFtUcaqOBL8v44lt4nUJEcs3aXcH5YQMlmgegMt5Q',
  value: { value: 100, type: 'Integer' }
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
✓ Inserted test record: SFgT0XdHCaRTf0TSkxBKvjB1Y4ZCPXTkxI83PxFlRInUe4Lug6TkoTH8wzerNFS-NJ4di5Y0nB04WK_9SRce1w

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
        "id": "1qobg-2Rq-h1oteGUGivHlWqJXt932IRY3a7Nc5Ctt0dKs5wfdgbqRO66EOezNHHx2k8Yqy0eH7nrKd18WIjPQ",
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
        "id": "AFFCCspHtdh2Z_HNMPET81S-9g9L5xZJZO7i3Ohr8RUf1gncruPep-3J_1b-bXIQ-zocLY_fH8N600eqeAkiUA",
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
        "id": "SFgT0XdHCaRTf0TSkxBKvjB1Y4ZCPXTkxI83PxFlRInUe4Lug6TkoTH8wzerNFS-NJ4di5Y0nB04WK_9SRce1w",
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
        "id": "OxC0Iqm6dKscDmJ4lvOfrE5ULerFFSPFfDhxbdGMxPG7DjUSbfjfBCkLZHOrBWGwBrzGmKldyS-4fge4dR1o9Q",
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

✅ Script saved: QNTPx_mOLe6k_s5WkfgO7kqFlAXpNrZys2yPvjRYdIkD3DzAEHmvRVB-rRgZCDYJcDZOApYEbT8ve2MDDv2EZw
📊 Found 30 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: rQCIF97cDekEHPWsbwtoVqGnM0mFDGQrgDTfH2f3avXEVcI8vj_htjZK9CeqgEUfO8gNCjl_kITM4A1siEA_zQ
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: 7-s81QzkI3N3qn-m00tx-49lNWRWJbT1teGHoDuqarJl4pLXxPebFkMVZDQDvCsDaGamhzc44drpsS48kAqJdQ
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"status":"inactive","count":10,"avg_score":50,"max_score":90}
   {"status":"active","count":10,"max_score":100,"avg_score":60}

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
Retrieved value: { username: 'john_doe', userId: 123 }

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
Collection created with first record: VmA8hbfgJr7ZhmAdwITV93D7LdxqOgt4Bz9KgQ2WYrdIUDQqPKUFILT2lsnvAqHs_rGiOwZQuF3FzmPg9Ohi9Q

=== List Collections ===
Total collections: 23
Sample collections: [
  'schema_products_client_py',
  'schema_documents_client_py',
  'batch_users',
  'schema_products_client_go',
  'ttl_cache'
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
Created Alice: $1000 - ID: FNMaGM3jpK0_j0IHoYzqAd6iXkF5beDEM9v-SIwfUAJXGTRwN5D833Mk0b0uF5fhfO1FozSDSeb8oH1S54BD2A
Created Bob: $500 - ID: dLtxC25JGXq03OIAB8W-X3NDZ15tTAw0CloBheWTL4AW5pQjppXBBgKJucR3zBg7KzMJNhPJgTQ8xWOXhBre5w

=== Example 1: Begin Transaction ===
Transaction ID: fd0f8e48-5056-4a70-b165-1e41f903a69c

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
New transaction: 27952eb5-0512-4421-abdd-8a6c2981bbef
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
   ✅ Script saved: ELuJTyYbnDuoGCUvzi254utOiy6jFo6ioBH35cnMtdp6BDOpfQcjuQSbVuXSHqAP-4E_6detrpW7ts1tp6n5dw

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
   ✅ Script saved: BB6sPBIWxW9yCi8AW-GHNT7P0bqUzDqWx2saIwcjiWuQZ8CluniwEibafEYm_Etp3va5FylykSIcyS4Xii4Z2w

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
   ✅ Script saved: YkqDZxYHDOTiuD-YGqRaDiE05FPZMp8PgaKyhFz2j_cLEvFTWc2ewXur8QyEY6X0o-eZvUSIPna-N5ATkpw4fQ

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
   ✅ Script saved: MKbcOOPrIF0LBC8XbIlmYsv_hEpeQngSQiZuW7LBOJdtGeGN05yQX7OUG3NKfHDB01yOR_-4CHmNa9i60zyUFw

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: ELuJTyYbnDuoGCUvzi25...
   ✅ Deleted script: BB6sPBIWxW9yCi8AW-GH...
   ✅ Deleted script: YkqDZxYHDOTiuD-YGqRa...
   ✅ Deleted script: MKbcOOPrIF0LBC8XbIlm...
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
  Output: Document ID = h6RUaJn7TmxIPgx8ncJnmKFHiUkbUwtziZF266CkoNpf5qok8MQLea7fCyJbmkMEsQm-4YnE1JWjdjxISUS0vw
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: findById(h6RUaJn7TmxIPgx8ncJnmKFHiUkbUwtziZF266CkoNpf5qok8MQLea7fCyJbmkMEsQm-4YnE1JWjdjxISUS0vw)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: findById(h6RUaJn7TmxIPgx8ncJnmKFHiUkbUwtziZF266CkoNpf5qok8MQLea7fCyJbmkMEsQm-4YnE1JWjdjxISUS0vw)
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
  Output: Document ID = QmlUsqptMn38z9zcO-0L7KNjSNkXICtxZQilaHtweabSRErlWGtwKBL3GttApU5kD5ROyi3u9iDDx40CSMwR5A
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: findById(QmlUsqptMn38z9zcO-0L7KNjSNkXICtxZQilaHtweabSRErlWGtwKBL3GttApU5kD5ROyi3u9iDDx40CSMwR5A)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: findById(QmlUsqptMn38z9zcO-0L7KNjSNkXICtxZQilaHtweabSRErlWGtwKBL3GttApU5kD5ROyi3u9iDDx40CSMwR5A)
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

added 1 package, removed 1 package, and audited 9 packages in 839ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'VDEKehfFLNB6B4egywW00PXSqcb4Qnw94iXdISJHId6IcLvAiCRcexY6qFm-I7bHTjQKptN7chVxs2wowdfixw'
}

=== Find by ID ===
Found: {
  value: 42,
  name: 'Test Record',
  id: 'VDEKehfFLNB6B4egywW00PXSqcb4Qnw94iXdISJHId6IcLvAiCRcexY6qFm-I7bHTjQKptN7chVxs2wowdfixw',
  active: true
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  active: { type: 'Boolean', value: true },
  value: { type: 'Integer', value: 100 },
  id: 'VDEKehfFLNB6B4egywW00PXSqcb4Qnw94iXdISJHId6IcLvAiCRcexY6qFm-I7bHTjQKptN7chVxs2wowdfixw',
  name: { value: 'Updated Record', type: 'String' }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: r2w3NxCvv7xu77ZGFIPerMHInOSO60YagfmYdrI811NfTlEjT4V2YFVip8gqZrrlriR-N2moviRa0DkP2O4wbQ

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
Collection created with first record: ZT67QW8zSkuTvnUW_Ao4q6dMMIE98VzOgYQLwXW4bBrO55UyhihUFwshryeamWexVPRUCvoynxa63wfEqK0vdQ

=== List Collections ===
Total collections: 22
Sample collections: schema_products_client_py,schema_documents_client_py,batch_users,schema_products_client_go,ttl_cache

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
cache:product:3: { price: 49.989999999999995, name: 'Product 3' }

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 0 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 11

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: false

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: BRmokLCIH-hsurznDMY8-eI-ah5FGZrn3B5sy87cnperIfm5UHfRk6_D34BsAAg26DimV_F2lRY6DGawh1CJFw
Created Bob: $500 - ID: BJhNYaGDwWRuGIJS8TgmJNUoOqblRqIA5J6vPcc09EItxGzcu8_pTksYVqunapiUd0dPT5QWapG_Y_RVe8mxIQ

=== Example 1: Begin Transaction ===
Transaction ID: 5dc9f908-1970-4050-832e-6bec9cfafa70

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 90a5ac2c-b282-4fa0-89d5-3d66d07b5492
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
  1. Score: 25.740, Matched: email, name, name.value, email.value
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title, title.value, bio, bio.value
  2. Score: 26.400, Matched: title, title.value, bio, bio.value
  3. Score: 26.400, Matched: bio, bio.value, title, title.value
  4. Score: 26.400, Matched: title.value, title, bio, bio.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title.value, title, bio, bio.value
  2. Score: 39.600, Matched: title, title.value, bio.value, bio
  3. Score: 39.600, Matched: bio, bio.value, title.value, title
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
  1. Score: 0.757, Matched: 
  2. Score: 0.756, Matched: 
  3. Score: 0.751, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.703, Matched: title, content.value, title.value, content
  2. Score: 1.501, Matched: title.value, title, content, content.value
  3. Score: 0.302, Matched: 

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
✓ Inserted document: aOShAVQ81ziYtmSuoO2Hgb9a2VpZTG1kZ4jMB8wiegMppPKRVhs652tFU2luHIFjYVWd1AdnjYdr1lC0Q18E0w

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: 5ZXTz0-Azh6NcNXN4-VkptN6N7NG8Cg69v3L3wXBbUMcIPhkJtviCXDpAeuGjIceJ4uf9iLnz5jxtPCOrLZLwQ

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
✓ Inserted document with TTL: tNfxftvfix0pf_kQ4TraTRq4fGeuOQvKeezsdyCoip6tPXHuhJZyjza-0Vzf46mVVok02kwJNUQKQlE-a_AI_Q

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
✓ Edge cache script created: eLQCqukXrNjDIG0YfMbZAonQFz06beF8KDP3vnhALNQU9DEl_P6uSVP-9xdgIlSsbYDH6J9SvhTx_3mLy4LyxA

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
Response time: 5ms (1x faster!)
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

✅ Script saved: 7oRIlsG5iHJaY9wUR3qGNfnDTlQEQDi13FSLdLf6gv2vtFyMhOvUcwe6dEpS6Fpeb3-fKUf0LAHQr3v0HN2hZg
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
   {"status":"inactive","count":5,"avg_score":50}
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
    "name": "Leanne Graham",
    "website": "hildegard.org",
    "id": 1,
    "email": "Sincere@april.biz",
    "phone": "1-770-736-8031 x56442",
    "address": {
      "suite": "Apt. 5...

Second call (cache hit - from cache):
   ⏱️  Duration: 4ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "name": "Leanne Graham",
    "website": "hildegard.org",
    "id": 1,
    "email": "Sincere@april.biz",
    "phone": "1-770-736-8031 x56442",
    "address": {
      "suite": "Apt. 5...
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

✅ Script saved: 2lm86CFOyIUexbGqJDhLSzYsHHBs1luNNbAa9fEzozIWovxNdQz03AWCF409jPoF6e8uOBoh0dx0n2ndiOjoRw
📊 Found 2 product groups
   {"count":2,"category":"Furniture","avg_price":474}
   {"avg_price":575.6666666666666,"count":3,"category":"Electronics"}
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

✅ Inserted order: HAhcXN3zUsgnK4vsdnD7VHB2TCA640B0tVmSiokW1UvlPZxA46r0ZX_AygOSIodkd5Qk4kPgXMLu9mZBZzFuHw
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: NOpO3LyzkhTNHXPcK_5ln-HwlpEKpuVqMfGXafFIDD8zags9K-NhPbqrRhTDnnAAwteFWyQloVMk0BHSG-rw5g
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"userId":"user_abc","role":"admin"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: hZiIMBOtF7R9gNpxxPJ-BQjLAVUsoofVAeKq-AvpN_fU9SS4lfhkAIzJcSG5U1Le5_j1W-aKB8d4v70b3BAUJg
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Script

✅ Script saved: 0RHyYgj4ccgU6x-cXJaldcqYNpigBavGdmGA87B7mIT70PBYZYUfApiZS94cvBkxUTyssB-yuCu_kikKQacNgA
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
   2. User 4 - Score: 40
   3. User 8 - Score: 80
   4. User 5 - Score: 50
   5. User 3 - Score: 30
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
   2. Introduction to Machine Learning (AI)
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
   1. Vector Databases Explained
   2. Introduction to Machine Learning
   3. Natural Language Processing
   4. Getting Started with ekoDB
   5. Database Design Principles
⏱️  Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Script saved
📊 Projected documents (showing first 3):
   1. Vector Databases Explained
   2. Introduction to Machine Learning
   3. Natural Language Processing
⏱️  Execution time: 0ms

📝 Example 5: All Document Fields

✅ Script saved
📊 All documents (5 total, showing first 2):
   1. Vector Databases Explained (Database)
   2. Introduction to Machine Learning (AI)
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
   1. Monitor 27" - $399 (⭐4.6)
   2. Office Lamp - $79 (⭐4.05)
   3. Wireless Mouse - $29 (⭐4.5)
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
   Vector databases have several benefits:

1. High Precision: They store data in points, lines, and polygons, allowing for accurate representation of geographical features.
2. Scalability: Vector data remains clear and crisp when zoomed in or out, maintaining the quality of the information.
3. Less Storage: They typically require less disk space for storage as compared to raster databases.
4. Detailed Analysis: They are suitable for performing complex spatial analyses such as network analysis and proximity analysis.
5. Attribute Storage: They can store attribute data directly, providing more detailed information about each geographical feature.
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
✓ Created SWR script: fetch_api_user_js (CbpMPileQHpSP2n237ZXzrfq86iVZkV-8rzWtoAiVTK9oFmDLzR7mYNWdLLnJBv1KLFfgOCYu3ufQX1ucOBV1A)

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
✓ Created session: D_cHXaBdyP6cFBRVghmlCp4SsifhNEMrwAYnH9KImFVXFwp5tlArQPc3uOn0ELVby_EkZZGDzFyVJ5I3MDCXRA

=== Sending Chat Message ===
Message ID: 4yZ0yGKgkGA-pAc8egNkbQsDyU0R4zW7uSfIfZGhhdNnUisrqO84KkfBnpmyRPUUiv2ied0iHxRdQ9Y7nPR1ig

=== AI Response ===
We have three products available:

1. ekoDB: This is a high-performance database product with AI capabilities. It is priced at $99.

2. ekoDB Pro: This is our enterprise edition product with advanced features. It is priced at $299.

3. ekoDB Cloud: This is a fully managed cloud database service product. It's the most premium offering and is priced at $499.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_js',
  record: {
    name: 'ekoDB',
    description: 'A high-performance database product with AI capabilities',
    id: 'zmC5A3C2qZM9pVmL6TVJO05SOXFucJyl7tSV32w_o_bmELouIS4AhllXYsTcVNmXIZPq9rkgVVn09QuCVh3aZA',
    price: 99
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_js',
  record: {
    description: 'Enterprise edition product with advanced features',
    price: 299,
    name: 'ekoDB Pro',
    id: '6O4R_y9Uy_fxk7HjoSqmwmaBI6rhpQR7j-th-fSEFBZ49hP9mRIS51sqPB1fMb2udw19bHxnpnlrOzozIISvkA'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_js',
  record: {
    name: 'ekoDB Cloud',
    id: 'h4PhlvQ6krKnxVYptPuavd2ZMNksajOtFqeWCX9qeU597fY_VBWcpB40AXMYQ3FulNPUKfofju_5REKIbjlwxw',
    description: 'Fully managed cloud database service product',
    price: 499
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 2411ms

=== Token Usage ===
Prompt tokens: 439
Completion tokens: 84
Total tokens: 523

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: -1F8ZJaVgtCQWbvRd8dw6Wp7yUbIDOTcn3SPBKXtzpJ8bMXvrrpUmA2YXEFvfjknlKa5RULqSPng1_NIMKwLqw

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information provided, the available product is ekoDB. It is a high-performance database product priced at $99.

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
✓ Created second session: Z64Sv02cFxEnSaMextpMhp_siQhMT2RPHwnMlnse1sbXC4dVbab3w2B9nl3s17Z2mU6kVuFXKY1cPwlqGgan9g
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
✓ Created session: QZY6aDob9AfHrnFrgTdE3-4CXLp4DiqQRNUDvYwOa17ZQJe1Ra4CrAEIDJ7WEQE47-4_lgm_sYN9JhA_aYO1Zg

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information provided, there is a product available named "ekoDB". This is described as a high-performance database product. The price for ekoDB is $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: BDXPT3TrnKaE1IbXbvK6rGPG-Y-8eDecsvlkZQmFKHF98Qrv8T1tvnN4YvQDMMQQ6joZHMFurHEwlqsnVt3eXw
  Parent: QZY6aDob9AfHrnFrgTdE3-4CXLp4DiqQRNUDvYwOa17ZQJe1Ra4CrAEIDJ7WEQE47-4_lgm_sYN9JhA_aYO1Zg

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: BDXPT3TrnKaE1IbXbvK6rGPG-Y-8eDecsvlkZQmFKHF98Qrv8T1tvnN4YvQDMMQQ6joZHMFurHEwlqsnVt3eXw (Untitled)
  Session 2: QZY6aDob9AfHrnFrgTdE3-4CXLp4DiqQRNUDvYwOa17ZQJe1Ra4CrAEIDJ7WEQE47-4_lgm_sYN9JhA_aYO1Zg (Untitled)
  Session 3: Z64Sv02cFxEnSaMextpMhp_siQhMT2RPHwnMlnse1sbXC4dVbab3w2B9nl3s17Z2mU6kVuFXKY1cPwlqGgan9g (Untitled)
  Session 4: D_cHXaBdyP6cFBRVghmlCp4SsifhNEMrwAYnH9KImFVXFwp5tlArQPc3uOn0ELVby_EkZZGDzFyVJ5I3MDCXRA (Untitled)
  Session 5: 6HMYe8f2LnvnT7htO9InHcE5ZpmMWDJiCxpKSnhFsoCMT35VT1BjOWXE_mgY6rjk4N9upYD3JdAXu-XGceqrew (Untitled)
  Session 6: bmtKFSsJIMgaxYxIJIgwDd7vqN1yertFomG__Mw_2ViZSrcjToYxhjK62OgIGfTnUdQA6Ihz7U--JY3HYEO97g (Untitled)
  Session 7: JToxVxwy9m-aQ0baFIrjo0PtqS51i8p9N88c_RHC0v4z17BNDGuTHGNEMWVwQ10UH_6RtpwtGrM63PdyqN75Aw (Untitled)
  Session 8: vIkerozaAQbKTwFKq5b79WasmEUkBW8FnwuwBtvUPKVVtL8jry26uj9VJ7-R-lJO_VSYmKBgB-YAgoXUu4WYgA (Untitled)
  Session 9: jBVTTzZ69hYnYPb7KjJmc_mUrtTOlS86_zYWhi4yc9DZgyPeDLwFr5z4S0bA13twwEabL5YuAUIjrRDTGC0VDw (Untitled)
  Session 10: kH7-Vta-h2D_ObzJ5F7JY5yXmQLoSUq2DDjvWf0gxR5lfY1gIXOlDWMqEt2vV2XJkwmsTYJKlC6SM4Mnind7dw (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: BDXPT3TrnKaE1IbXbvK6rGPG-Y-8eDecsvlkZQmFKHF98Qrv8T1tvnN4YvQDMMQQ6joZHMFurHEwlqsnVt3eXw

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
✅ [32mJavaScript client examples complete![0m
