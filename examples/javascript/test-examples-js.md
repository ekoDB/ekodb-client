make test-examples-javascript
🧪 [36mRunning JavaScript examples (direct HTTP/WebSocket)...[0m

added 1 package, removed 1 package, and audited 9 packages in 829ms

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
  id: 'tw-6jr-FkONBBsVBcX1iINuX6Uw5hMUWki1xG6CM7_6sczeybC_cNhFqR9I7MlPeWFBieWwPxwx2X8lABLFEnw'
}

=== Find by ID ===
Found: {
  value: 42,
  active: true,
  id: 'tw-6jr-FkONBBsVBcX1iINuX6Uw5hMUWki1xG6CM7_6sczeybC_cNhFqR9I7MlPeWFBieWwPxwx2X8lABLFEnw',
  name: 'Test Record'
}

=== Find with Query ===
Found documents: [
  {
    active: { value: true, type: 'Boolean' },
    id: 'tw-6jr-FkONBBsVBcX1iINuX6Uw5hMUWki1xG6CM7_6sczeybC_cNhFqR9I7MlPeWFBieWwPxwx2X8lABLFEnw',
    value: { type: 'Integer', value: 42 },
    name: { value: 'Test Record', type: 'String' }
  }
]

=== Update Document ===
Updated: {
  name: { value: 'Updated Record', type: 'String' },
  active: { type: 'Boolean', value: true },
  id: 'tw-6jr-FkONBBsVBcX1iINuX6Uw5hMUWki1xG6CM7_6sczeybC_cNhFqR9I7MlPeWFBieWwPxwx2X8lABLFEnw',
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
✓ Inserted test record: vtbgkHIxQW1zE1V2jD3ldgOVT7gAmWJ6AikiU0K3eEqlEc6S-310DB6rWBFTxJNjgIcvO8Gyo9os6no1JE5Cag

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
        "id": "vtbgkHIxQW1zE1V2jD3ldgOVT7gAmWJ6AikiU0K3eEqlEc6S-310DB6rWBFTxJNjgIcvO8Gyo9os6no1JE5Cag",
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
        "id": "5hJY8xNIE_yBjOKap9rIhD9xSjQwmq22-i2n8dejL3ukAO1L14NWPI-e_ItHJxYuPlZccDwnp8KqS_oNQYYPVg",
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
        "id": "pcH7DNujG1CstAOdqZ9y9CWA6UOITf7bMp3t8WIPEoFoCrnix0MgigTh0pgJ8vQZKPG4SajrkXJxvH3_GZD-MA",
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
        "id": "hqv3ZeE80SGWiiGHjNPjmodyzWaTYy5FkLK3O98nFYXHTzeIusk1MyTtfMT4roJOF9NhJp4yMqsxgGZOgbaAiA",
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

✅ Script saved: FMuROjikVxfUeTvSWhsCtJ9Ee_6d1J7aI8owSJqQ-Yl05WWe9WDtZauwTKFb7dfjL_2cgodI94eWvpYuBB-H_A
📊 Found 30 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: KvXd0NRwQcgC3QWwDghkSkxIdfHwuzy4pkx6ZzruQFgUE4tpps5rxBmAlWech69WuA_0JTsw4TpxjkZSQ4oXMg
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: 183z41mqR5ojp1fx7t07ljZFwx2_H385Gd2qGtHbDaj758Ql341cl_ErC1Aj6kj4oTJ6bulZ8cpkR7V_fik3_A
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"status":"inactive","max_score":90,"avg_score":50,"count":10}
   {"status":"active","avg_score":60,"count":10,"max_score":100}

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
Collection created with first record: miHuzz45vI_lRiydF_U8YXlHKXY_8ppe2j30gJvTisWr4-8cvhipwgv65fsqex_YjBlY5U43RzE1Wau1dvsJAw

=== List Collections ===
Total collections: 24
Sample collections: [
  'schema_documents_client_ts',
  'schema_employees_client_ts',
  'batch_users',
  'schema_employees_client_py',
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
Created Alice: $1000 - ID: g15N0wWYXahRAcKRSHhLBwoU3cBHDQxKhDVMeEzcxZbKdmo7PLqFY8oXtIpf_D94IcrOoZRecNPp_fSvcpqdEg
Created Bob: $500 - ID: 2gQ-E5W2ej72Az-ZLQrT0DzcgBqji8LpEajqr27APVoRCEZDvseuYGULESiEP-22IT5tnscfrDQAObMh-jbcmw

=== Example 1: Begin Transaction ===
Transaction ID: 2a45df62-d284-4dc1-8705-3ffbf7a1049c

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
New transaction: 9bc7290e-58c2-4dc3-8eeb-0e96e23e23a6
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
   ✅ Script saved: 8OYkC7dQqRlZWEJyPWspvFbdk1CKv5XrlX-Q1TNiwYPF8iso2DA1QoRfNpnvzo2mKkY5Ytls3v6O4vMKwf9XmA

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
   ✅ Script saved: 4WfCtUWjXjD9YT0H-b38pflUEAuMhEaZGrmQZ87UCPf0KbvZkTHR3cWAjWZ27c772UZFKuPR_BkvhyZ08CL_JA

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
   ✅ Script saved: qE_3yuW5-k-FWzVigkt7T3csiEDQYwjl5ztDdKhruBscY72mC-U4shku1FybWAkNXc_7HvbIlhN8vyRLZDYAtw

2️⃣ Calling Script (Query + Update Credits + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {"type":"Integer","value":0}
   📋 Status: {"type":"String","value":"active"}
   📋 Name: {"value":"Alice Smith","type":"String"}

============================================================
📝 Script 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: Qg5qcu81kqadbNnPAOWbmaVIHy4__vGikU5RW4ABXfeBeJuErraK9OtaXeRwhvkdpM5rJKBT50yyPnxQ8tv_ng

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: 8OYkC7dQqRlZWEJyPWsp...
   ✅ Deleted script: 4WfCtUWjXjD9YT0H-b38...
   ✅ Deleted script: qE_3yuW5-k-FWzVigkt7...
   ✅ Deleted script: Qg5qcu81kqadbNnPAOWb...
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
  Output: Document ID = I9zHl1zIqlRH7faEwwA8iN6XXTOwQPQQ7yU7jG-G5-mLx084tJ_Km7NA2yApKCjq2MWXoUCxqzQxto91hf7n9w
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: findById(I9zHl1zIqlRH7faEwwA8iN6XXTOwQPQQ7yU7jG-G5-mLx084tJ_Km7NA2yApKCjq2MWXoUCxqzQxto91hf7n9w)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: findById(I9zHl1zIqlRH7faEwwA8iN6XXTOwQPQQ7yU7jG-G5-mLx084tJ_Km7NA2yApKCjq2MWXoUCxqzQxto91hf7n9w)
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
  Output: Document ID = BZ8y0X12_X25d9XOobKQWpmaEit9e0miS1aeoM5xgF2y-nOOE9YRDRf5p9l4Ob6DhoX-r5OO2hpgxr7bYu7Yjg
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: findById(BZ8y0X12_X25d9XOobKQWpmaEit9e0miS1aeoM5xgF2y-nOOE9YRDRf5p9l4Ob6DhoX-r5OO2hpgxr7bYu7Yjg)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: findById(BZ8y0X12_X25d9XOobKQWpmaEit9e0miS1aeoM5xgF2y-nOOE9YRDRf5p9l4Ob6DhoX-r5OO2hpgxr7bYu7Yjg)
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

added 1 package, removed 1 package, and audited 9 packages in 816ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'bUM7FhvD2iG5uKLrPV19rHvOoLvJVSmlrykobQOcyupqsPX1K4DcGVPZs396sRVPEt8PWlH-RJjlDn2j1mzT_Q'
}

=== Find by ID ===
Found: {
  name: 'Test Record',
  id: 'bUM7FhvD2iG5uKLrPV19rHvOoLvJVSmlrykobQOcyupqsPX1K4DcGVPZs396sRVPEt8PWlH-RJjlDn2j1mzT_Q',
  value: 42,
  active: true
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  value: { type: 'Integer', value: 100 },
  id: 'bUM7FhvD2iG5uKLrPV19rHvOoLvJVSmlrykobQOcyupqsPX1K4DcGVPZs396sRVPEt8PWlH-RJjlDn2j1mzT_Q',
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
✓ Inserted test record: 6mFkCigWOgLFaF-Th8d_j2jmJ6xexIF3dGvzLWoVHyq891R3b3Fe4C08ZCZtoxAwVOSYR52wBrb64NdIZzUluA

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
Collection created with first record: 8kAw49EDSAW2a5deXr2aMt6bphDCKebYgBSf3qxnvSXel51dyofvh2pSi9Mj7A0bUnP1zWPcdbASIIOqrfNkgg

=== List Collections ===
Total collections: 23
Sample collections: schema_documents_client_ts,schema_employees_client_ts,batch_users,client_collection_management_js,schema_employees_client_py

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
Total keys in store: 10

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: false

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: JRGYzMMmAjeARJDG3QalH8e5roHO1KbfcvwQUrxSNHjrm-RQOk8OYCaV0AH9PZqdIJ-gZf48joFWJ56N9TFWpw
Created Bob: $500 - ID: kJ2cIOLpFPJlafraCNJb_c5sKY33SmtVXCpvVj2xiovnETzdvoSosGPNupMKTl_t-VCJmkMhmrQSmOJc3RQZ0Q

=== Example 1: Begin Transaction ===
Transaction ID: cceadd5c-6fa7-4665-9211-c172df841afc

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 7b5f40ea-3594-4678-9a52-1aeccd8688be
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
  1. Score: 25.740, Matched: email.value, name, email, name.value
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: bio.value, title, title.value, bio
  2. Score: 26.400, Matched: bio, bio.value, title.value, title
  3. Score: 26.400, Matched: bio.value, title, title.value, bio
  4. Score: 26.400, Matched: title, title.value, bio.value, bio

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title.value, title, bio, bio.value
  2. Score: 39.600, Matched: title, title.value, bio.value, bio
  3. Score: 39.600, Matched: title, bio, bio.value, title.value
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
  1. Score: 0.753, Matched: 
  2. Score: 0.747, Matched: 
  3. Score: 0.745, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.698, Matched: title.value, title, content.value, content
  2. Score: 1.499, Matched: content, title.value, content.value, title
  3. Score: 0.301, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: skills, bio, skills.value, bio.value

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
✓ Inserted document: rRhDvtBgVsLwegknu_hMZNyxbayoK-dKXMy9YCmkj40bGdI35mONEBEkwqRwiTChxthPLdz1At0ErflLEummlQ

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: nGUJKOaLjnUWi1x3IWI4UU98tJ7qsAhosfaPScKzjnrTgfShV7H3cKdX8XD8IDkwQqiGvgnSPJJ4Lq-h_TDYHA

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
✓ Inserted document with TTL: UEaUrJYfjWDUJuiSTc2xsI1nz-LchmW_88oY2r6aTC0qSwcADfiKzbiYz-plKkxWthkjTnfbzUH6Pmj-brWuNw

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
✓ Edge cache script created: sCqT8LC8MmPDFo1q0NDHX93e2qh3SGGv3dxUSRKsPx4xp3r6nFjDBg-lYc9XwSiBzTc4vQQbP1XDKuFI9Lf31Q

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
Response time: 5ms (0.8x faster!)
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

✅ Script saved: kdDn5CFo0IvY7jNfg-HtIuB4x1mZTUKVbDVQxp7P2ole6rfjZYL7XK-odsRcbczCKIki-9zbemoQvU0MoeBY8Q
📊 Found 10 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved
📊 Found 10 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved
📊 Statistics: 2 groups
   {"count":5,"status":"inactive","avg_score":50}
   {"avg_score":60,"count":5,"status":"active"}
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

✅ Saved reusable function: fetch_and_store_user
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 68ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 68ms
   📊 Records: 1
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

✅ Script saved: rD0q8-UKzHWHFfm722pL0evrPXW5nrWQ15R-h8zZ_PEEkteZFh_V1B88A8aGooe9BGtwDSL5hSTklKV8Af3u8Q
📊 Found 2 product groups
   {"category":"Electronics","count":3,"avg_price":575.6666666666666}
   {"count":2,"avg_price":474,"category":"Furniture"}
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

✅ Inserted order: SvYJEdKBcGOHE4ERFWgTIDpcANry2z0Hhq3uZugV45Sf1AshJPVHXBvpgyjGXhK-GWsAXaStDWlcFo2CqhQ9Og
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: acPtUE6SrO3kd0uZ_0gP3A11MGmWpRwMeLXoxWddNH9Nt9Dc8i9rPzJNUfykmRUs8Udxo5kxpW4fyCA9ThcymQ
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"role":"admin","userId":"user_abc"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: gm56B29OdA-8WKZNcF2i3TSaeWPHTJfbwXnOayTMEwLquVD5_9C8XAdeRw6dnntACrnaCAdZkrV-HAv6ZAmPhA
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Script

✅ Script saved: 50zXDw2bnCTgWqFdJItFfNbpPSBJlLuILbCUNcAdkhLpnXFYJdQSVKvPH4w0q1w9UZ_aWu_N3gbiyK4bc90RuQ
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
   3. User 1 - Score: 10
   4. User 9 - Score: 90
   5. User 10 - Score: 100
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
   1. Database Design Principles (Database)
   2. Vector Databases Explained (Database)
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
   1. Database Design Principles
   2. Vector Databases Explained
   3. Getting Started with ekoDB
   4. Introduction to Machine Learning
   5. Natural Language Processing
⏱️  Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Script saved
📊 Projected documents (showing first 3):
   1. Database Design Principles
   2. Vector Databases Explained
   3. Getting Started with ekoDB
⏱️  Execution time: 0ms

📝 Example 5: All Document Fields

✅ Script saved
📊 All documents (5 total, showing first 2):
   1. Database Design Principles (Database)
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
   1. Office Lamp - $79 (⭐4.05)
   2. Desk Chair - $349 (⭐4.2)
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
1. High precision: They can store data with accurate geographical details.
2. Scalability: They can maintain the quality of the data over different scales.
3. Rich annotations: They can store a wide range of attributes for each element.
4. Complex shapes: They support point, line, and polygon features, enabling the representation of complex geographical features.
5. Topological information: They can store information about spatial relationships between features, such as adjacency or connectivity.
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
✓ Created SWR script: fetch_api_user_js (cFaWwGJVoUzXIZ0CTYrrFDfmoFhHpko-dliwplTtE8YbXPCD1zJ4NuQq3X3gKfnxHGHUdI8B-j30QBGaHPHCMw)

Step 2: First call - Cache miss, fetches from API
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
✓ Created session: CTY2iSovTRK8XIpq56MQw_dZ4hzfMznrK3E_CcmQRxGpkPoh93XuNEDtvjwV1Dpd0_X4lAozCtGlUZ4NRiQ6wg

=== Sending Chat Message ===
Message ID: 0biQIC3PEFV08lbuV_gzfBns9NR3udPCGUI2E-UYTHHU2k5Plf3o6oDQniJ2IWd00mZOnWpfVa6RPyKhY-b5kw

=== AI Response ===
Sure, here are the available products along with their prices:

1. ekoDB Cloud: This is a fully managed cloud database service product. The price for this service is $499.

2. ekoDB: This is a high-performance database product with AI capabilities. You can avail of this product for $99.

3. ekoDB Pro: This is an enterprise edition product with advanced features. The cost of this product is $299.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_js',
  record: {
    name: 'ekoDB Cloud',
    id: 'Kk-Qc7ekFFicNMLW8Rx7EyNYE4xZiwvi7mW-ceAlX90kpBvFIO8ZFDLu0jqwVkcJV-3tmdI_mLoKpeEXUio7rA',
    description: 'Fully managed cloud database service product',
    price: 499
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_js',
  record: {
    name: 'ekoDB',
    price: 99,
    description: 'A high-performance database product with AI capabilities',
    id: 'gWhOtaflUTxdt_52AzEXQAWu36Av2zP62PGIzKmMHEvl4ADQ7UBQ9jOfI9HXWxyTlhCDeM6JLwZLovLTPXbc3Q'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_js',
  record: {
    description: 'Enterprise edition product with advanced features',
    price: 299,
    name: 'ekoDB Pro',
    id: '9E0svAU-VQ2qSMaZeiZGX57yDTxUhOU3FArfifjVElw_aHu2_vkEe89grTLLg32r0MSc-FrF_i5UkpmoKVIBmA'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 2851ms

=== Token Usage ===
Prompt tokens: 442
Completion tokens: 91
Total tokens: 533

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: DKZeDa3UDK7D_WgoBTkUvHWy83o2Vw-BFxBLlzUUzvNnz9sHaNimdQKuT0vtfZOYSvlhkk5pmhM49ztiiinPfA

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information provided, the available product is ekoDB. This is a high-performance database product priced at $99.

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
✓ Created second session: N72_FIqT-cj-5knzLGnwG9Km20_X6V88ePF0IKQCAvZEJ7EWyFqF9KMJmXTkE875A0S5gLj7YKGh86kxNw-y7A
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
✓ Created session: C6kRR7rbxNouGf6iqeUeoQQXKPOveZywoUzRq2ixYaNbdMmnxCfmZTj34P0gdCJWlewH17UEUIlj06ZuhHd3Qg

=== Sending Messages ===
✓ Message 1 sent
  Response: The product available is called ekoDB. It is a high-performance database product that is priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: ueu6y1FyT417FJd4mHMv7j-4I6_qWCIP7QCxV7C6ilfNUJW5heCVZhrF8W82JVr-BadwUh1NsdFiDaXPlKsS8A
  Parent: C6kRR7rbxNouGf6iqeUeoQQXKPOveZywoUzRq2ixYaNbdMmnxCfmZTj34P0gdCJWlewH17UEUIlj06ZuhHd3Qg

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: ueu6y1FyT417FJd4mHMv7j-4I6_qWCIP7QCxV7C6ilfNUJW5heCVZhrF8W82JVr-BadwUh1NsdFiDaXPlKsS8A (Untitled)
  Session 2: C6kRR7rbxNouGf6iqeUeoQQXKPOveZywoUzRq2ixYaNbdMmnxCfmZTj34P0gdCJWlewH17UEUIlj06ZuhHd3Qg (Untitled)
  Session 3: N72_FIqT-cj-5knzLGnwG9Km20_X6V88ePF0IKQCAvZEJ7EWyFqF9KMJmXTkE875A0S5gLj7YKGh86kxNw-y7A (Untitled)
  Session 4: CTY2iSovTRK8XIpq56MQw_dZ4hzfMznrK3E_CcmQRxGpkPoh93XuNEDtvjwV1Dpd0_X4lAozCtGlUZ4NRiQ6wg (Untitled)
  Session 5: XzoY0Q3plKT5z1-cw0LeEN9F4tZpkI3AZHbLfdgmKiuv84NhQsbRCNGQ32gP4u3I4UqWGHNh27Ve1pYiMjVjpw (Untitled)
  Session 6: FB4-TYOgQSPy0uF6q52_r9rLImQOmn3HVKf6BvDduWMjeTC4gkL3_7fb-FEDBXCnat2j-KLUZw3ERScwlqxqZg (Untitled)
  Session 7: P9Gtt__FZAGXMtDCzI9MT2N1gzXCMhjEAa1-TF9ILjQ1ACRA4eq914qs0HtLB7mmAy3pjzxtcO4hRA1YTE2bOw (Untitled)
  Session 8: XVLAxRIZ2pkRmHp9G4_7Fxu3Q3sWMtIfMsNr_xL7N20PJT9DHMvXAk9fn_d3yknl2b_WHniFz3mwSyofDnGjVQ (Untitled)
  Session 9: D4Z_YfTWI7OhM6qRxOYYxzbG94AXkSb6FBwL_qCLZ2a2Xy4gOwGLjEmD0XswFmozj2pgAXGv_Tmm9VUd1b--hQ (Untitled)
  Session 10: pk7aA3dAX2d3eAynAmikTPNZOCiz1gvd-t23welEmVxmgiKenDhkabrvAxJJCY4a187SCvG2PnwjZv7g2Sk8Ow (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: ueu6y1FyT417FJd4mHMv7j-4I6_qWCIP7QCxV7C6ilfNUJW5heCVZhrF8W82JVr-BadwUh1NsdFiDaXPlKsS8A

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
✅ [32mJavaScript client examples complete![0m
