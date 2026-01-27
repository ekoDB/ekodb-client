make test-examples-javascript
🧪 [36mRunning JavaScript examples (direct HTTP/WebSocket)...[0m

added 1 package, removed 1 package, and audited 9 packages in 849ms

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
  id: '2qYxPpQCE3EGU-Zc_GcVE8cbzsiyjj-1JiQ0frwifWvhYeUsJiy_60fzusLZe6XRGfQH1rJ2m5Mhm0hZDJtQWg'
}

=== Find by ID ===
Found: {
  id: '2qYxPpQCE3EGU-Zc_GcVE8cbzsiyjj-1JiQ0frwifWvhYeUsJiy_60fzusLZe6XRGfQH1rJ2m5Mhm0hZDJtQWg',
  active: true,
  value: 42,
  name: 'Test Record'
}

=== Find with Query ===
Found documents: [
  {
    id: '2qYxPpQCE3EGU-Zc_GcVE8cbzsiyjj-1JiQ0frwifWvhYeUsJiy_60fzusLZe6XRGfQH1rJ2m5Mhm0hZDJtQWg',
    name: { value: 'Test Record', type: 'String' },
    active: { type: 'Boolean', value: true },
    value: { value: 42, type: 'Integer' }
  }
]

=== Update Document ===
Updated: {
  name: { value: 'Updated Record', type: 'String' },
  active: { type: 'Boolean', value: true },
  id: '2qYxPpQCE3EGU-Zc_GcVE8cbzsiyjj-1JiQ0frwifWvhYeUsJiy_60fzusLZe6XRGfQH1rJ2m5Mhm0hZDJtQWg',
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
✓ Inserted test record: afyb7CNA-oKXxEfRyBMG1uTij8n5NFM9Wn6VQwgNPd2Z7WeQ9mSQ0Ll0JppbzlqC0Nn-sPMz30-N0AW-bj-Big

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
        "id": "i5vbX_z97nJ1E56GlHzp2wC_FmlBv1Wm-Ipn3OO4Ra22X_MSsZWRGj03UKV_xtN8YVAD3vqj8KPRCFrFF9WyLA",
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
        "id": "SNyEc7R-CZkxYBn4G05Ra_eW5cYKuJXomLyBtWiFJVcI8VEUsZ4LRygII3U0d05JOVnjULyCL6HURn3kXafJ2Q",
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
        "id": "UTmrXQDM_xk24AvjXafv75Ex54tPoRRcsv09e6t_5bW_U4BC3Vkjl5mrH9x0OXidPmHqBPVf2zcWdZvSCusrYw",
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
        "id": "afyb7CNA-oKXxEfRyBMG1uTij8n5NFM9Wn6VQwgNPd2Z7WeQ9mSQ0Ll0JppbzlqC0Nn-sPMz30-N0AW-bj-Big",
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

✅ Script saved: 39TSrLV_WDDCaNiLxHfnSA-R_L20sDoTrio-GDGG9LQWrDs0kYGj8bbmuLMly-ohCCGxz31ZsICXMpQri9AOIA
📊 Found 30 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: sXlpcbwUWl2wNK2gjL9MWYjKE_YMFZ7mnZc1Uc-V5T-NDa-zFOM01TOxlb3IigwI7f9LiH60v3tnXW-ti5tlJQ
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: bosO_gWnsh_-OhRtGJdRpfPfmExM_H6gSKw1T88eAiK7mvUfqgLFlcy8QXrqOEIcbcfW8X9iLNQzB4cdFRTRcg
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"count":10,"avg_score":60,"max_score":100,"status":"active"}
   {"status":"inactive","max_score":90,"avg_score":50,"count":10}

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
cache:product:1: { price: 29.99, name: 'Product 1' }
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
Collection created with first record: IKvTKT-NNE4q0v8z0O9lw1DpziL0QO7bz3UKWcXk0g0s-Q6o0a2qX29qH_zBcJo9PoN-5on4tT0AXu2LE1eDmg

=== List Collections ===
Total collections: 23
Sample collections: [
  'schema_employees_client_py',
  'scripts__ek0_testing',
  'schema_employees_client_go',
  'ws_ttl_test',
  'test_accounts'
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
Created Alice: $1000 - ID: AX0cOHE68X7U1ooxxHGCnVbHzXxhxc_L8cBc4oJPX2w8s4RK5dtPBe5qnQ9iGL3zHpgfBvfdz2sM82q-H-6RHQ
Created Bob: $500 - ID: 8To2UM2kF0klXNarAllQGVMxYN5mtMvc51tMBzqmH9kPt5xtQLuttamk8CQci038jV6FbOAPydbvjQP2U32OtA

=== Example 1: Begin Transaction ===
Transaction ID: 44d2bd17-0cad-4e6c-815d-2502914e12b8

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
New transaction: 09cab908-dbd8-4bfa-8038-3d3004748150
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
   ✅ Script saved: WaZwOGT_WOhGXKCy64jhW_Y69OtwDl-PrVMkJUmTCffigQwPzxhczdCgoDNmHO1RARgmmeMBL7OypAwcB__fYA

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
   ✅ Script saved: _nlOYQ4-wAD1EFngip3vJPndLiB-Pkc65oQrJk7uMAZTwPPWirJOMWXpL-SPyDUhyaVtARNSNPIZKXhEEfbeWA

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
   ✅ Script saved: uC_sjXIvJy2yklj9Sc_0k-MBqAyonbLqtE0RzVG5nsi9qZZyDk-Z8pTqpc2rfKEcu6ZuNiJocJhJ8WmEBFhjcQ

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
   ✅ Script saved: bGro7Kw4NRJj4qaI43FpNzBGrBp5ok5cpw9mBcZuaZMJyhroQ0Azg7wMDz9y7TDCiWRDwVt5Rx_sS3CssCaWZg

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: WaZwOGT_WOhGXKCy64jh...
   ✅ Deleted script: _nlOYQ4-wAD1EFngip3v...
   ✅ Deleted script: uC_sjXIvJy2yklj9Sc_0...
   ✅ Deleted script: bGro7Kw4NRJj4qaI43Fp...
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
  Output: Document ID = h5YZHpGA8fUPo4Ve_p92hOR4jKNqzJQ83rX4HEeAfohmYGi5tSFJBZLTcgcU4a2chxvNOaHQN8sNC-Yw2lW19Q
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: findById(h5YZHpGA8fUPo4Ve_p92hOR4jKNqzJQ83rX4HEeAfohmYGi5tSFJBZLTcgcU4a2chxvNOaHQN8sNC-Yw2lW19Q)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: findById(h5YZHpGA8fUPo4Ve_p92hOR4jKNqzJQ83rX4HEeAfohmYGi5tSFJBZLTcgcU4a2chxvNOaHQN8sNC-Yw2lW19Q)
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
  Output: Document ID = MEEnImriKz3BcRzFYdH-IrH43LooOM5Ju3OhGM27in4lmx17QHwrzCKyaY3ttL3MySghuwCSduGD8yGsI2DoAg
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: findById(MEEnImriKz3BcRzFYdH-IrH43LooOM5Ju3OhGM27in4lmx17QHwrzCKyaY3ttL3MySghuwCSduGD8yGsI2DoAg)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: findById(MEEnImriKz3BcRzFYdH-IrH43LooOM5Ju3OhGM27in4lmx17QHwrzCKyaY3ttL3MySghuwCSduGD8yGsI2DoAg)
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

added 1 package, removed 1 package, and audited 9 packages in 838ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'YlXPUyfuWVoMWOj9g07p14inXz2CXGkNdRtTDVUBHw-pzNChk0bZlUu-6O4CVi362fTdeLo6pdOJyp-ySV2M-A'
}

=== Find by ID ===
Found: {
  active: true,
  name: 'Test Record',
  value: 42,
  id: 'YlXPUyfuWVoMWOj9g07p14inXz2CXGkNdRtTDVUBHw-pzNChk0bZlUu-6O4CVi362fTdeLo6pdOJyp-ySV2M-A'
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  id: 'YlXPUyfuWVoMWOj9g07p14inXz2CXGkNdRtTDVUBHw-pzNChk0bZlUu-6O4CVi362fTdeLo6pdOJyp-ySV2M-A',
  value: { value: 100, type: 'Integer' },
  active: { value: true, type: 'Boolean' },
  name: { type: 'String', value: 'Updated Record' }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: scb-LM5utmzwjfYJGp_UrjqXLFT5dxIzGEp7xFT4-GaY6u5r7X-K7g1jU9MccwClg-p9g-E_K6k7XCl-fp-6Vw

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
Collection created with first record: nby-r8aJcCGfAy1HaM6Fx1ShzpTnhPLvQVho9fFSXbEn-kFxVSkFb6g3y-Fov1UxzbctpcjI5EuM3qpAiatbxg

=== List Collections ===
Total collections: 22
Sample collections: schema_employees_client_py,scripts__ek0_testing,schema_employees_client_go,ws_ttl_test,test_accounts

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
cache:product:3: { name: 'Product 3', price: 49.989999999999995 }

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 0 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 14

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: false

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: mg15l4TfbWNHUvXlwuLWj59XynzyEw6CxWK5VOumHB3O28OJxmozO8FxDFh-z5q0MD58GSBtAvrC85bQDnq1Dg
Created Bob: $500 - ID: 1Tf0MBeaStF69FJlWvEuWb7ZauuDS4yCEVFHDD8rt_gm7HC41ul8Tyqvpr4gWsa_4nn6BazdIKSaJitmI_5hvQ

=== Example 1: Begin Transaction ===
Transaction ID: 237e560b-8b6d-491a-8208-fd6f48460185

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 5f7e0ad7-ec57-4c0b-a913-29730027b0c6
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
  1. Score: 25.740, Matched: name, email.value, email, name.value
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: bio.value, bio, title, title.value
  2. Score: 26.400, Matched: title, bio, bio.value, title.value
  3. Score: 26.400, Matched: title, title.value, bio.value, bio
  4. Score: 26.400, Matched: bio, title.value, bio.value, title

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title.value, title, bio, bio.value
  2. Score: 39.600, Matched: title, title.value, bio.value, bio
  3. Score: 39.600, Matched: title, title.value, bio.value, bio
  4. Score: 39.600, Matched: title.value, bio, bio.value, title

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio.value, bio

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.772, Matched: 
  2. Score: 0.761, Matched: 
  3. Score: 0.738, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.695, Matched: title, title.value, content.value, content
  2. Score: 1.509, Matched: title, content.value, title.value, content
  3. Score: 0.304, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: skills, bio.value, bio, skills.value

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
✓ Inserted document: KC3lu1X3QIYj_ZQA9z2EGJgNOO0xavJaGsO48tVWTVbGW5GoXiENFSiPltORVAX45wyaxz8FT6PR8Ut5H5RWdg

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: 0vUJdvCPNFqv8KZtO2hgVBjqe6mnP9QHM69Iym_sIvUFqa9BO2h2ymUOr6J_I20nndFd1wG1e-Xn3jnDojFdgg

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
✓ Inserted document with TTL: B1Ynx-o1Q-2mxB-fo-dMY9UfZIyoNZx33aJmlV307Cls4bVXEC6og09XeixmSxDSBMIEFPCfEogmDFgJeiu7BA

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
✓ Edge cache script created: fBRssuY1pdiibIlXrshi7ZYkkMNE1wFxINXsvpM3rjgDQ_1JX7GXSleWGX6a54dXv0-501mR0mWdYRN4HV3PTA

Call 1: Cache miss (fetches from API)
Response time: 4ms
Result: {
  "records": [
    {
      "value": {
        "current": {
          "temperature_2m": -9.3,
          "interval": 900,
          "time": "2026-01-27T00:45"
        },
        "latitude": 40.710335,
        "generationtime_ms": 0.025033950805664062,
        "longitude": -73.99308,
        "current_units": {
          "temperature_2m": "°C",
          "interval": "seconds",
          "time": "iso8601"
        },
        "elevation": 32,
        "timezone": "GMT",
        "timezone_abbreviation": "GMT",
        "utc_offset_seconds": 0
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
Response time: 5ms (0.8x faster!)
Result: {
  "records": [
    {
      "value": {
        "current": {
          "temperature_2m": -9.3,
          "interval": 900,
          "time": "2026-01-27T00:45"
        },
        "latitude": 40.710335,
        "generationtime_ms": 0.025033950805664062,
        "longitude": -73.99308,
        "current_units": {
          "temperature_2m": "°C",
          "interval": "seconds",
          "time": "iso8601"
        },
        "elevation": 32,
        "timezone": "GMT",
        "timezone_abbreviation": "GMT",
        "utc_offset_seconds": 0
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

✅ Script saved: d_KRYkKCmY6XpBbKvjFbA7PktajNrrksQuY_BA9FIcZMWfUKxr4PJ2_nhtl8b40DrVDxnCTcEho9-LtvII0G0w
📊 Found 10 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved
📊 Found 10 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved
📊 Statistics: 2 groups
   {"count":5,"avg_score":60,"status":"active"}
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
   Name: {"value":"User 1","type":"String"}
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
    "username": "Bret",
    "id": 1,
    "website": "hildegard.org",
    "address": {
      "suite": "Apt. 556",
      "zipcode": "92998-3874",
      "geo":...

Second call (cache hit - from cache):
   ⏱️  Duration: 4ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "name": "Leanne Graham",
    "username": "Bret",
    "id": 1,
    "website": "hildegard.org",
    "address": {
      "suite": "Apt. 556",
      "zipcode": "92998-3874",
      "geo":...
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

✅ Script saved: rIXopKKpCQKSG0ImvWZxzIZFb2CKj0D2okJpItQTIEcsXwTslhwrpJvMThdRUg3Kylkv_EvRvUfIN4UPhuuNzQ
📊 Found 2 product groups
   {"category":"Furniture","avg_price":474,"count":2}
   {"category":"Electronics","avg_price":575.6666666666666,"count":3}
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

✅ Inserted order: MYDzVGiZoDCeMIZ43Cre5AleFAZi4AxwwNIGBxHcQAiqXfwkAKgcIZjkv06W5kILUiX3jTM-x8naRzTY9jUQGQ
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: oQYXz3zBsqfJvMDzddlOIPxraoDle_aIFdw2it2KTTqPQzRuudu6p1V62Mjv9PwfHVo8ObH5JaVu374XSpMQHQ
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"userId":"user_abc","role":"admin"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: a3a7AleEYMF-gpoq-v076zm8G8h5nLyj9WHKFKe4ZAKyNGfvSGXMh5Zu2w1KU1-x3BchovW9wc9UVbnZID7EEA
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Script

✅ Script saved: uol_2sgX0qylrwxtJMonew8j-7oPZflU7sVEjZ-ZA2AsuCXeRwPZY5W3FEcMjnAXa1AnAFrD-_K0xlkTSLFKfA
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
   user: 70.0 (7 users)
   admin: 20.0 (3 users)
⏱️  Execution time: 0ms

📝 Example 4: Top Users by Score

✅ Script saved
📊 Users (showing first 5 of 5):
   1. User 5 - Score: 100
   2. User 3 - Score: 60
   3. User 1 - Score: 20
   4. User 2 - Score: 40
   5. User 4 - Score: 80
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
   3. Introduction to Machine Learning (AI)
   4. Getting Started with ekoDB (Database)
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
   3. Introduction to Machine Learning
   4. Getting Started with ekoDB
   5. Natural Language Processing
⏱️  Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Script saved
📊 Projected documents (showing first 3):
   1. Vector Databases Explained
   2. Database Design Principles
   3. Introduction to Machine Learning
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
   1. Bookshelf - $149 (⭐4.1)
   2. Wireless Mouse - $29 (⭐4.5)
   3. Keyboard - $89 (⭐4.4)
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

1. Precision: They provide high accuracy in representing data points, lines, and polygons.
2. Scalability: They allow for easy zooming and scaling without loss of data quality.
3. Detailed Information: They can store additional data attributes for each element, enabling complex analysis.
4. Flexibility: They support a wide range of operations including topological, network, and proximity analysis. 
5. No data redundancy: Each geographical feature is stored once, reducing data redundancy.
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embedding script saved
📊 Generated embeddings for 3 articles
   1. "Advanced Query Patterns" - 1536D vector
   2. "Draft Article" - 1536D vector
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
✓ Created SWR script: fetch_api_user_js (_JhvyIqGoZDgqCCtf1deVz9VBZH7RKbSoN6qiMGwa2X_74QrszK2kfEwYsBDkDoU5nULWbu-ODpdh5w8k4HVsw)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "website": "hildegard.org",
        "email": "Sincere@april.biz",
        "id": 1,
        "username": "Bret",
        "address": {
          "city": "Gwenborough",
          "geo": {
            "lat": "-37.3159",
            "lng": "81.1496"
          },
          "suite": "Apt. 556",
          "zipcode": "92998-3874",
          "street": "Kulas Light"
        },
        "company": {
          "catchPhrase": "Multi-layered client-server neural-net",
          "bs": "harness real-time e-markets",
          "name": "Romaguera-Crona"
        },
        "phone": "1-770-736-8031 x56442",
        "name": "Leanne Graham"
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
✓ Created session: 11HFPGkhd_hF9LmwsPlm_PEHPhLqvw1bibZ2lYYe-9BfaCgkM2gDajU8mOEVgdN64SpsSPl6WI9hWskhXoSfLQ

=== Sending Chat Message ===
Message ID: Y-riW469nweM54yeU6KHcNVE0h52ov5iBDJqfRTp3D9cr2SNjeKT3iLwiwayklNT8RyNbN634owMoXWg3yeLuQ

=== AI Response ===
Based on the information provided, three products are available:

1. ekoDB: This is a high-performance database product with AI capabilities. The price of ekoDB is $99.

2. ekoDB Pro: This is an enterprise edition product with advanced features. The price of ekoDB Pro is $299.

3. ekoDB Cloud: This is a fully managed cloud database service product. The price of ekoDB Cloud is $499.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_js',
  record: {
    name: 'ekoDB',
    description: 'A high-performance database product with AI capabilities',
    id: 'SvmviP-jbQX5lxTAB3UkKIUx0-8VLYsPPaIOsIZmitLSjoOBCrnG0llyX8w_1BbjcygDWDAdBFNiwGmXdyMF9A',
    price: 99
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_js',
  record: {
    name: 'ekoDB Pro',
    id: 'k3IYlsDw4gpD8-mNUaRijJ4wA237X8mxC64cpnngxiFj_bz3uhsQHNgahoWI2OP_1ChX5KXHLHT-2mvFXTY-_w',
    price: 299,
    description: 'Enterprise edition product with advanced features'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_js',
  record: {
    id: 'bBZH1N7vrHeJ3mhMpIJ-JPoqRAXaBD2A_FzPQd9_MRk6DrT9eK2XwuMOCX_1evvsPlGHe18ydvPwI9rEdphP9w',
    price: 499,
    name: 'ekoDB Cloud',
    description: 'Fully managed cloud database service product'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 2156ms

=== Token Usage ===
Prompt tokens: 450
Completion tokens: 94
Total tokens: 544

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 1xQzxnMLQogjxhGM_YD2NKbT5M1g1G3UHij3AbsBeusKrpi3GkRMmpV8V0cBHcEwH3YzjPQKe6IoJvmipWo6Uw

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information provided, the available product is "ekoDB". It is a high-performance database product and it costs $99.

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
✓ Created second session: _XUA9f8IGp4l8g7rAqE0yHucl4McjXzgSYrXYS0ONU5-Z-t1R5vvW9uwslUcwkpTMWBC_0IzLpcsQ004jHb4lA
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
✓ Created session: 0m9MlN3xoq0iuYjkQr0KQwKt2cwawUIoyJHFGPEULCUh0qCdCNX6tvI5l8K0EGM_AIStM5dpoK0yFD1eKd3X5g

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the provided context, the product available is "ekoDB". It is a high-performance database product priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB high-performance database product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: ePRTUavE7J4VEgHntqRKMxHmz8uirFBq7c2Ibt9yC0cW7ZgWLdz6qTy2WwV5Zmvht0moyQXH_tkkjIxu3AbhyQ
  Parent: 0m9MlN3xoq0iuYjkQr0KQwKt2cwawUIoyJHFGPEULCUh0qCdCNX6tvI5l8K0EGM_AIStM5dpoK0yFD1eKd3X5g

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: ePRTUavE7J4VEgHntqRKMxHmz8uirFBq7c2Ibt9yC0cW7ZgWLdz6qTy2WwV5Zmvht0moyQXH_tkkjIxu3AbhyQ (Untitled)
  Session 2: 0m9MlN3xoq0iuYjkQr0KQwKt2cwawUIoyJHFGPEULCUh0qCdCNX6tvI5l8K0EGM_AIStM5dpoK0yFD1eKd3X5g (Untitled)
  Session 3: _XUA9f8IGp4l8g7rAqE0yHucl4McjXzgSYrXYS0ONU5-Z-t1R5vvW9uwslUcwkpTMWBC_0IzLpcsQ004jHb4lA (Untitled)
  Session 4: 11HFPGkhd_hF9LmwsPlm_PEHPhLqvw1bibZ2lYYe-9BfaCgkM2gDajU8mOEVgdN64SpsSPl6WI9hWskhXoSfLQ (Untitled)
  Session 5: dpg_aOZtkhEvHo8iJpUKoqa55h9hgs3sBNiM71LBROeSc3an4F6XM8JhUYKx016FvWx96YXa51QLZQ0Po9a26Q (Untitled)
  Session 6: MYsswK26dIhK0ipLj8nJFRTbG8IQiyF_4mKpg5mEtX4HXEnEH7Vew-lQtKJa9eHlLmxdL_cfNftqvm8d8SR6BA (Untitled)
  Session 7: NhhawogDR-W1uyrubf4Vy3LpqE-Mm_jmIIxwHzWJ2SHLplFr2q2vhzNbxdlZiEvsdOHBNp8QHDfD4gdK_rv-Jw (Untitled)
  Session 8: iNZEpgS6r8Uqw0x8SEIEnhHg4JXAtjjC-KqEzM7n8XtjCj_svrdOBRIH8JMbHviLioiCVq76SLfW74PxEZma7Q (Untitled)
  Session 9: woXubsbUeD5ZkrfMPFhh4KzorIMwlqY44nzU-yATGRo-tEmr6MvBd3MketHD7BynjDK0wDfBu7EvT86fiOCBHg (Untitled)
  Session 10: hOirRCXEPeKhjl3cqR-FjGWJYz-CC5616S4qL_3_e5F9XF2l-2x_HP-5_OkQmN-iOwXBNvT7oE538Kvzwf8R4Q (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: ePRTUavE7J4VEgHntqRKMxHmz8uirFBq7c2Ibt9yC0cW7ZgWLdz6qTy2WwV5Zmvht0moyQXH_tkkjIxu3AbhyQ

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
✅ [32mJavaScript client examples complete![0m
