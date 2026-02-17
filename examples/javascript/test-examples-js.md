make test-examples-javascript
🧪 [36mRunning JavaScript examples (direct HTTP/WebSocket)...[0m

added 1 package, removed 1 package, and audited 9 packages in 831ms

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
  id: 'AGiWNwjxLIyxDPf16PdliMIukYyBaoS0OfvN17noET43S6fTMfDB_PJcQxvPLTfRsQOcyQ8NSEQt-UWXemyc0g'
}

=== Find by ID ===
Found: {
  name: 'Test Record',
  id: 'AGiWNwjxLIyxDPf16PdliMIukYyBaoS0OfvN17noET43S6fTMfDB_PJcQxvPLTfRsQOcyQ8NSEQt-UWXemyc0g',
  value: 42,
  active: true
}

=== Find with Query ===
Found documents: [
  {
    id: 'AGiWNwjxLIyxDPf16PdliMIukYyBaoS0OfvN17noET43S6fTMfDB_PJcQxvPLTfRsQOcyQ8NSEQt-UWXemyc0g',
    active: { value: true, type: 'Boolean' },
    value: { value: 42, type: 'Integer' },
    name: { value: 'Test Record', type: 'String' }
  }
]

=== Update Document ===
Updated: {
  active: { type: 'Boolean', value: true },
  name: { type: 'String', value: 'Updated Record' },
  id: 'AGiWNwjxLIyxDPf16PdliMIukYyBaoS0OfvN17noET43S6fTMfDB_PJcQxvPLTfRsQOcyQ8NSEQt-UWXemyc0g',
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
✓ Inserted test record: 2wbEcGy-RUUNvMHBTGPMYMMH_SOL4lGPy59BtQ322egL1P4f25fIAZnLkGuxVcUsZFxCstsw8aO9GfGYwjFOyg

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
        "id": "2wbEcGy-RUUNvMHBTGPMYMMH_SOL4lGPy59BtQ322egL1P4f25fIAZnLkGuxVcUsZFxCstsw8aO9GfGYwjFOyg",
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
        "id": "7U_AjsNjdsptAlQOoy_2aTwfBvYhn9ShHDJPTvso0thKw3w6ox-VSJgWOwzRo-ZwLW5FaZAot1kAEEBKIZcRTw",
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
        "id": "UbkSVO-3CVUFzkzjzUVGW03t3dCnu76ezQuoPhihNBirDB9SKVLKRu7cXXkeJqBDa1jmEzD9JK4Gj7lmJijLiQ",
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
        "id": "tMXQuh4jlWDX_OSNfnqa-eb4HPQx-COO65VPlCvqMLQlQqqHNUL6QUlTJ8wSV1qB3IvOSl-Qel4eXqpnlZacEA",
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

✅ Script saved: oVGc4bVJ0CupMzf-KHo3uyPNbxvGr2a6wuEYp-SQDBTxiYIRdlffLtoKmHvcZ7AYXhOrjqivkswX-2_o3Vtr8g
📊 Found 10 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: OMdivTWwil0qb-E0aWqiZpAKcKZJ0p4hW9aVvzh8sh-tW5xZUURfze0YeZim4MjSLb3xsxALlxxwOGIib4wEGg
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: ZBF_1ajNGnIpurGP8mO9_oHZl1adeB7_4gjg4HBQPwqATIWJlI1cL-4HJTLIPThcvajNLWIVr-Lg41dXqS2trg
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"max_score":90,"avg_score":50,"status":"inactive","count":10}
   {"max_score":100,"status":"active","avg_score":60,"count":10}

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
Collection created with first record: DH3XFP6VQxPCZlGRakbpqhV0Qgx24kS9ltpF01VMdBUiSpMZqx0TaMkyhfo9ZHdnY4GteUgWLkhbe1NzrbUD9A

=== List Collections ===
Total collections: 25
Sample collections: [
  'schema_products_client_go',
  'ttl_cache',
  'ws_ttl_test',
  'test_collection',
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
Created Alice: $1000 - ID: EbS2aip69RiJQrw8DajHlK3YwojqvHz-q0gmQ6u7xK5y0UEnuNiDGW2MD6T0-GAj0sFcJ-DoQyC5QRp-YIurbg
Created Bob: $500 - ID: ZXj3EZftYv4D00VN-gEsD40mXBNNtcy1UOirSYOs4zxaXG2hrTXdDCOKdSgih64zLMnOGzJ5RaMIYhL4pA2IiA

=== Example 1: Begin Transaction ===
Transaction ID: 093c7b3e-f445-4a94-bf05-92851695a477

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
New transaction: 54470e64-66a5-4361-87d4-7b55b4c5fbeb
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
   ✅ Script saved: at-MvbBuFxLCK-JWftQC3mC4wuxFd8-qlFJf4Ao-fQVrR9htc08IJadAORM9cHEfMmJFLcPbqCjazWbX7Dngrw

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
   ✅ Script saved: BWwBohGc_FgbjGCvX__Xh5aAaNjd_kJb5-T4fRHt5f162lID6dy2Rnp_XrzjtopdNZHPamvu6J_T1C8KAXfVZw

2️⃣ Calling Script (Query + Update + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Status updated to: {"value":"active","type":"String"}
   📋 Name: {"value":"Alice Smith","type":"String"}

============================================================
📝 Script 3: Query + Update Credits + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: h-oU48JyrLwxpk-l88jFXxtYEvv0XSZ_QNOBDyRunBZxlGgD3zI685SXxo97vT5R0oa5fEhMDRSIVj3v2lo8Bw

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
   ✅ Script saved: eepx4VvX80-OPba8saYMLDzDoCNRxtV0O7WiiccEvb6KQT_5Z1OH-uMshXlRfAoi4raqQj8iFBHOtHdpYKP4Ng

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: at-MvbBuFxLCK-JWftQC...
   ✅ Deleted script: BWwBohGc_FgbjGCvX__X...
   ✅ Deleted script: h-oU48JyrLwxpk-l88jF...
   ✅ Deleted script: eepx4VvX80-OPba8saYM...
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
  Output: Document ID = 9P1tzw9rNJL8_ObTRUZj6Z_UiQTdCOC_cUDk6Uk4-TgM8n4xgf2PGEkwoa5Hs0J2_OhD5z2_3aCS0jJ-cwB1sQ
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: findById(9P1tzw9rNJL8_ObTRUZj6Z_UiQTdCOC_cUDk6Uk4-TgM8n4xgf2PGEkwoa5Hs0J2_OhD5z2_3aCS0jJ-cwB1sQ)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: findById(9P1tzw9rNJL8_ObTRUZj6Z_UiQTdCOC_cUDk6Uk4-TgM8n4xgf2PGEkwoa5Hs0J2_OhD5z2_3aCS0jJ-cwB1sQ)
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
  Output: Document ID = O0Mx8kLbgVHkNxZpz9XI94qkNAMSpn0oztxcfSZ5n2CYlJNbLMDoamdh1W2c16T4bJjpF7ntf2nPVaMxhTT0tA
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: findById(O0Mx8kLbgVHkNxZpz9XI94qkNAMSpn0oztxcfSZ5n2CYlJNbLMDoamdh1W2c16T4bJjpF7ntf2nPVaMxhTT0tA)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: findById(O0Mx8kLbgVHkNxZpz9XI94qkNAMSpn0oztxcfSZ5n2CYlJNbLMDoamdh1W2c16T4bJjpF7ntf2nPVaMxhTT0tA)
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


up to date, audited 58 packages in 1s

15 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.11.0 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning JavaScript client library examples...[0m

added 1 package, removed 1 package, and audited 9 packages in 841ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: '--uCzXxXwLMyUw2QgsGuqL7-eTjw_k9MA3D9i0pjweeSo7sqnZHwVt76z8X5KrC0vMvulvLEPEzL_mT376t_Dw'
}

=== Find by ID ===
Found: {
  active: true,
  name: 'Test Record',
  id: '--uCzXxXwLMyUw2QgsGuqL7-eTjw_k9MA3D9i0pjweeSo7sqnZHwVt76z8X5KrC0vMvulvLEPEzL_mT376t_Dw',
  value: 42
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  id: '--uCzXxXwLMyUw2QgsGuqL7-eTjw_k9MA3D9i0pjweeSo7sqnZHwVt76z8X5KrC0vMvulvLEPEzL_mT376t_Dw',
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
✓ Inserted test record: ad3TiGow_BdHj-U2283PhWCjVlrBvsNSnvJ2xfOiflbCfXtjPshOvay4ehGRsNcNsktok4TBOqn9w2v7GEapWQ

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
Collection created with first record: 7-A_eTVTa9dQtSulqvqLOYSl2q1ehf5TjGgasX6eG4AgfpuZveSzHUsCQ8jWlJil8fWyx4vT1bxmUA3Y0CLl5A

=== List Collections ===
Total collections: 24
Sample collections: schema_products_client_go,ttl_cache,ws_ttl_test,test_collection,chat_messages__ek0_testing

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
cache:product:2: { price: 39.989999999999995, name: 'Product 2' }
cache:product:3: { price: 49.989999999999995, name: 'Product 3' }

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
Created Alice: $1000 - ID: nN-7XS7u00zpOF_qu-6YrHxN8FWHQGerX050T050v4ZNhNNFE1TkNWlHXWS5lZNkFjkyKkhF2F28ilETUm4miA
Created Bob: $500 - ID: aGCH-Youaq1rMJT3M0NY-HwgDF2mNPNEq0bM1XtNcrbRweMQbw72J2OTnfjBEaEgT1zZH8IPWARcKkxRWmgrYg

=== Example 1: Begin Transaction ===
Transaction ID: 37e06e46-a953-4c88-b1cb-860c2d3c5161

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 235afac7-7538-494b-9add-9c4bf85519d3
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
  1. Score: 25.740, Matched: email.value, name, name.value, email
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title.value, bio, title, bio.value
  2. Score: 26.400, Matched: title.value, title, bio, bio.value
  3. Score: 26.400, Matched: title, bio.value, title.value, bio
  4. Score: 26.400, Matched: title, bio.value, bio, title.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio, title, title.value, bio.value
  2. Score: 39.600, Matched: title.value, title, bio, bio.value
  3. Score: 39.600, Matched: title, bio.value, title.value, bio
  4. Score: 39.600, Matched: title, bio, bio.value, title.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio.value, bio

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.750, Matched: 
  2. Score: 0.740, Matched: 
  3. Score: 0.734, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.700, Matched: content, title, title.value, content.value
  2. Score: 1.496, Matched: title.value, content, content.value, title
  3. Score: 0.294, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: bio.value, skills, skills.value, bio

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
✓ Inserted document: RhxY6RLUDyvljSXuDBY3VERqjIzwbfiAkQNJABknysryKeVDjCwWfpXuCawqSVArcO_Ee3XxRrpjG-RSuIWPxw

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: 0R6SIPo0VtZVI2lVTvNMIK-_n6r6WOcwB2yWZvW_ZWw97-DEuutju75an93JVaqTQ1qylbeRaZphjanSk3rUOQ

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
✓ Inserted document with TTL: 46KrmNjwSmPf65tbcLsG6M9c9rOA2WUlArl_yMtN4gew3aoI7WTqZoPxT1y4Q3MBMcnAew52xA8FWTqRQEgGDw

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
✓ Edge cache script created: BdSQFg1-SPq8tszkIbmYUz8x3eInnMXrNH8jm3dWSnZQZEK7aAsvcWwVR5tJUHlgTTyqvt3BIRgwiKVXu-tBpw

Call 1: Cache miss (fetches from API)
Response time: 4ms
Result: {
  "records": [
    {
      "value": {
        "timezone_abbreviation": "GMT",
        "utc_offset_seconds": 0,
        "generationtime_ms": 0.01990795135498047,
        "longitude": -73.99308,
        "latitude": 40.710335,
        "current": {
          "time": "2026-02-17T02:45",
          "interval": 900,
          "temperature_2m": 0
        },
        "elevation": 32,
        "current_units": {
          "time": "iso8601",
          "interval": "seconds",
          "temperature_2m": "°C"
        },
        "timezone": "GMT"
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
Response time: 3ms (1.3x faster!)
Result: {
  "records": [
    {
      "value": {
        "timezone_abbreviation": "GMT",
        "utc_offset_seconds": 0,
        "generationtime_ms": 0.01990795135498047,
        "longitude": -73.99308,
        "latitude": 40.710335,
        "current": {
          "time": "2026-02-17T02:45",
          "interval": 900,
          "temperature_2m": 0
        },
        "elevation": 32,
        "current_units": {
          "time": "iso8601",
          "interval": "seconds",
          "temperature_2m": "°C"
        },
        "timezone": "GMT"
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

✅ Script saved: WPmOXNAlpW56wFZNVeRCCkyiEgM45ozYKA9CF0o706BNkuoFBtWsfBYqZThzMeTbOZMp689LxDrRwMj4cGff6w
📊 Found 10 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved
📊 Found 10 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved
📊 Statistics: 2 groups
   {"status":"inactive","count":5,"avg_score":50}
   {"status":"active","count":5,"avg_score":60}
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
   ⏱️  Duration: 2ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "email": "Sincere@april.biz",
    "phone": "1-770-736-8031 x56442",
    "address": {
      "suite": "Apt. 556",
      "zipcode": "92998-3874",
      "street": "Kulas Light",
      "...

Second call (cache hit - from cache):
   ⏱️  Duration: 2ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "email": "Sincere@april.biz",
    "phone": "1-770-736-8031 x56442",
    "address": {
      "suite": "Apt. 556",
      "zipcode": "92998-3874",
      "street": "Kulas Light",
      "...
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

✅ Script saved: 0yXEYtFS35yZHkbXvAWNvuJLrE9f32s4GEvPLa1KE5d-8A3JiMkVm4ULAhpO0D3el8Ipgn3brjUxlS25DyEZ1Q
📊 Found 2 product groups
   {"avg_price":474,"count":2,"category":"Furniture"}
   {"count":3,"category":"Electronics","avg_price":575.6666666666666}
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

✅ Inserted order: jkf4kgBRCXt0emZ_LVIDWnMceUdAMM9t_u6zxlrnfOXKy5bsVG91VwcrUb52XnC6mIttQuLbwep7zektR1fWiQ
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: 4WZZLpsdjW2JGD85bbkFthAUSwD9Me8mJwLJkl25mlbzc1WsYdg0QvCLwTQDTWpKEb3ibAoDkwg1hAq9TeoKig
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"role":"admin","userId":"user_abc"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: QWYwa3I24vtVL5yhsfPcOWzLgNXgq-OxdllWp4u9lJAJGXqMFLbETwFdwX7qopUnGIkxwOia8WDJCQEsy842eA
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Script

✅ Script saved: _6plE7M2VckuRoVVNA6TBmCqlZYTLNL4wEpNrHjD22fsIFSqGaE6KlCBY6fNylskWyIrduo0zRbDqgk3ZKP9zA
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
   1. User 1 - Score: 10
   2. User 6 - Score: 60
   3. User 2 - Score: 20
   4. User 7 - Score: 70
   5. User 5 - Score: 50
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
   AI: 2 documents
   Database: 3 documents
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
   1. Webcam HD - $119 (⭐4.5)
   2. USB-C Cable - $19 (⭐4.3)
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
   Vector databases have several benefits:

1. Precision: They provide accurate representation of geographical features as they use coordinates to store data.

2. Scalability: The quality of data is not lost when zooming in or out.

3. Flexibility: They can handle a variety of feature types, such as points, lines, and polygons.

4. Attribute Storage: They allow for the storage of additional information about each feature.

5. Analysis: They support complex spatial analysis operations such as network analysis and topology.
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embedding script saved
📊 Generated embeddings for 3 articles
   1. "Getting Started with ekoDB" - 1536D vector
   2. "Draft Article" - 1536D vector
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
✓ Created SWR script: fetch_api_user_js (jxu1XEta3hDQTLntHgjXBxsx59PEwS1lvPn_wwsGLXWBSSNSWjL8NFHiwM9e25tlEoeqzOnJm2WYW3uDTeEi6g)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "email": "Sincere@april.biz",
        "name": "Leanne Graham",
        "username": "Bret",
        "phone": "1-770-736-8031 x56442",
        "website": "hildegard.org",
        "id": 1,
        "address": {
          "city": "Gwenborough",
          "suite": "Apt. 556",
          "geo": {
            "lng": "81.1496",
            "lat": "-37.3159"
          },
          "street": "Kulas Light",
          "zipcode": "92998-3874"
        },
        "company": {
          "catchPhrase": "Multi-layered client-server neural-net",
          "name": "Romaguera-Crona",
          "bs": "harness real-time e-markets"
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
Response time: 3ms (served from cache)
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
✓ Created native SWR script: github_user_native (4kahjF5UDUdsWDJl1YetJG3xoeqsn2eCyowirVZJnP8LCjaGi71FxB9cp_K31G80owyJ6jAcnIkR6iLSOGt0wg)

First call (cache miss - will fetch from GitHub API):
  Response time: 4ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 3ms
  Speedup: 1.3x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (KJ5ub5CfuL4oUqYnkb_MxzBwghAHL05CwnbBWpoAV31x1Ow5hi33pJa8c0-DqGVNFkL6HQLWtV5EYwrDotr1mQ)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (iMc9jAiU7ItxSYLU1AxBE3ijUAmgZMN2cPuEBPM03kDLn-YAbCTmhVo7GDFM3pVZU8e2AlWYhD6-c27wJg6NNg)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (tQ4jAmIujE-66CiaxCj8WW6Ar9xv63HKUTq1oqvi7-WnK3jqIE3XtgPUDUfvbVAFT4aaTCL5MbBKZSrbS4SJnA)
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
✓ Created session: nf6X6FGnvxk-O55nbN4FrH6f8FNfpoC8mK2YkJOCQnvdbd8XapPe2ayyuGLZiwa911-yxvFiQ0QuHcY40sdi7w

=== Sending Chat Message ===
Message ID: BqazKEUKPYWDHaCAmAARDRtQJHorcYR9ZNbj9pWtCE6gq2uwDwUBiAp2L7GoYxDq_dKxfLIs_6qOY4-3qCAw5A

=== AI Response ===
We have three products available:

1. ekoDB: This is a high-performance database product with AI capabilities. The price for ekoDB is $99.

2. ekoDB Cloud: This is a fully managed cloud database service product. The price for ekoDB Cloud is $499.

3. ekoDB Pro: This is an enterprise edition product with advanced features. The price for ekoDB Pro is $299.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_js',
  record: {
    description: 'A high-performance database product with AI capabilities',
    price: 99,
    id: 'km2QrV9tSf9pOAobhutwrmWeETCxrKvdtkb55C23b7qcAWfioDi0v8pIuxExqt2_ZrzS6ImfsB02en7jPC2KRQ',
    name: 'ekoDB'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_js',
  record: {
    description: 'Fully managed cloud database service product',
    price: 499,
    id: '-hNEsVS_ChzJikd_VrbByYaJ35kEruTp5T2CRwSJ2lFdNrwwOFcXjRvPJWnMKl_XziJJbTJcKh27mkibTRihpw',
    name: 'ekoDB Cloud'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_js',
  record: {
    price: 299,
    id: '-dTlMLQVnj_FtVWFyWtKL0wTSCgXJLbE_16-UMc5LLqB_jLYiocgi1h4V86saR1xneWD-0M8xSjBQBvkta1RyQ',
    description: 'Enterprise edition product with advanced features',
    name: 'ekoDB Pro'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 2307ms

=== Token Usage ===
Prompt tokens: 444
Completion tokens: 89
Total tokens: 533

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: ZGnq6DkODAJGHV94lHwgBlD2mXd5-VQck3ZdEQBitHEO4MiMiIz1Kd2k0fh6PFctfyOwO2YFAvfZq8kMiHA6Gg

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information available, the product that is currently available is ekoDB, which is a high-performance database product. It is priced at $99.

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
✓ Created second session: xsx0JOrmUcg46KRkv-d6TKVPnVHnZVq_mt5Mhf4zVe4uGUDTIdxtHaAoX84c0q_DqCJ1O544gfy6pXhm6NTtIQ
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
✓ Created session: gUua3TKF4seheKom_7nDgQCLQ2zGeUMjLXP-IPF2Ch6stdL9_XJynCB5PEB5_xuM6NUL-zgD26tXlDDaz4uLlg

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product based on the context provided is ekoDB. This is a high-performance database product. The price of the product is $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: _6-HMkgiJITVVp7SCzL38TYW51kah2FpbWDejpn6BI-f8zy-jfTgGZLRo1bGew7ZWwo8ybJ7IAAXm7P3EHW27w
  Parent: gUua3TKF4seheKom_7nDgQCLQ2zGeUMjLXP-IPF2Ch6stdL9_XJynCB5PEB5_xuM6NUL-zgD26tXlDDaz4uLlg

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: _6-HMkgiJITVVp7SCzL38TYW51kah2FpbWDejpn6BI-f8zy-jfTgGZLRo1bGew7ZWwo8ybJ7IAAXm7P3EHW27w (Untitled)
  Session 2: gUua3TKF4seheKom_7nDgQCLQ2zGeUMjLXP-IPF2Ch6stdL9_XJynCB5PEB5_xuM6NUL-zgD26tXlDDaz4uLlg (Untitled)
  Session 3: xsx0JOrmUcg46KRkv-d6TKVPnVHnZVq_mt5Mhf4zVe4uGUDTIdxtHaAoX84c0q_DqCJ1O544gfy6pXhm6NTtIQ (Untitled)
  Session 4: nf6X6FGnvxk-O55nbN4FrH6f8FNfpoC8mK2YkJOCQnvdbd8XapPe2ayyuGLZiwa911-yxvFiQ0QuHcY40sdi7w (Untitled)
  Session 5: m8KucHrtImE8EGLl0aPaZdNe5pS377z7tyXmcORwL97_A1c-Wt6VAVt1lve1G3XNRItgF-F5imbOg7DBZVcneA (Untitled)
  Session 6: iMrrBwh_iM3CZUah3Hnm-RJaXsJ4XTkffHBoMj66TPbN83NpUVdl3RHuolCa2wTB0OFFNGDBps2n2PLm2Dgf-Q (Untitled)
  Session 7: 5c1hYhIseC1Rib781yny1RS7ji-O83D6S7pIhP8h-gK0SWpmiCHbHrvkgWICVumzQZnk3922hOLSVOtItCgETw (Untitled)
  Session 8: _R991hXOG5qQ__gaqsmnfTMeIJXiTa3qJrg-Ks_I7pzhxDxYlDbj5eRY5L_5FAtMXdl5wJk0uvEo_KxeWBLF3Q (Untitled)
  Session 9: 50wMJRC5Nz4KJWZX5q0y-qo_sw1dbZkjOvEAUZ366369Ac1XfjIIWYmUTIbaPJNsTJlBN483dwLhpBj0P-liiA (Untitled)
  Session 10: kJLX4-mw8H11aSzk1r6xbiXD2pt77Sbrqb_Dt4t1gS3Lg5qCTuMvefiER9v_dbaQJKQwXjlxY6Ch5TfwuniSuQ (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: _6-HMkgiJITVVp7SCzL38TYW51kah2FpbWDejpn6BI-f8zy-jfTgGZLRo1bGew7ZWwo8ybJ7IAAXm7P3EHW27w

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
✅ [32mJavaScript client examples complete![0m
