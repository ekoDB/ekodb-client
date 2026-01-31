make test-examples-javascript
🧪 [36mRunning JavaScript examples (direct HTTP/WebSocket)...[0m

added 1 package, removed 1 package, and audited 9 packages in 826ms

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
  id: 'uFydC8YZD_-AKsvhgXz1qyonAK_w7wvQHi9rMz2J7YJ4MQKQNju5SJf1vTjvpHwAoHK0hF7UbYwffXLo3_rWDA'
}

=== Find by ID ===
Found: {
  name: 'Test Record',
  active: true,
  id: 'uFydC8YZD_-AKsvhgXz1qyonAK_w7wvQHi9rMz2J7YJ4MQKQNju5SJf1vTjvpHwAoHK0hF7UbYwffXLo3_rWDA',
  value: 42
}

=== Find with Query ===
Found documents: [
  {
    active: { type: 'Boolean', value: true },
    id: 'uFydC8YZD_-AKsvhgXz1qyonAK_w7wvQHi9rMz2J7YJ4MQKQNju5SJf1vTjvpHwAoHK0hF7UbYwffXLo3_rWDA',
    name: { type: 'String', value: 'Test Record' },
    value: { value: 42, type: 'Integer' }
  }
]

=== Update Document ===
Updated: {
  name: { value: 'Updated Record', type: 'String' },
  value: { type: 'Integer', value: 100 },
  active: { value: true, type: 'Boolean' },
  id: 'uFydC8YZD_-AKsvhgXz1qyonAK_w7wvQHi9rMz2J7YJ4MQKQNju5SJf1vTjvpHwAoHK0hF7UbYwffXLo3_rWDA'
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
✓ Inserted test record: QZWEIpxX_cDEYXaBkP2pVf-Qe3o5fi5pNMj7dRgy1xMnfMFjNL4nZKQoCc0FWWzAHeg_plO0-i8UfDwXkmVOdQ

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
        "id": "UJ8sqjygQJh0xmSoS1YV0zvVaZmZyqqdFu1OVFWvpebBTmIknmxii-J_gDQXS07nfbIq8eBNXTvj3CxPf1LaXw",
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
        "id": "QZWEIpxX_cDEYXaBkP2pVf-Qe3o5fi5pNMj7dRgy1xMnfMFjNL4nZKQoCc0FWWzAHeg_plO0-i8UfDwXkmVOdQ",
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
        "id": "NIBJ21jtUelEp3g_m7rMhyUHgt9uWYX6SJNxAPo9eSmRjgGCWthulWkYUu0t1Fkm8e3TYIwv0ZOWp0E8q4LuIg",
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
        "id": "V7V0v4mKQGqSaPv_0ST4Gl51DVujlFQPWBPeIOOJJtJ2hekrrygil20odbMkN3rEhNuAxv79PUhjp6aJPJgDMg",
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

✅ Script saved: JE1bhM4PuwzKEkkycXelq5-YOdYB7sEoSKFSOuhXsZKRyfWhsSK46XFVHW92k9MrxONVHyJPu6OfGwJJjXHQVw
📊 Found 10 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: aHmnBOJld8jHFcW6RrLQum4hISPzAmFYBT88cPzVVELEuqsHrA9SiXBb28ZBrvGsqRocP74vXogVhPRydebHhw
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: ERe6MQ-enig6hFhHTdiNcGwQ085FE7N2lc8HOSg88j6VGtGsTgHXSg9hILPpU3hx_IwHldOzcfIegLKOTFuy5Q
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"status":"active","count":10,"max_score":100,"avg_score":60}
   {"count":10,"max_score":90,"status":"inactive","avg_score":50}

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
[32m✓ kv_operations.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/collection_management.js ===[0m
=== Collection Management (Direct HTTP) ===

✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: KPFnSrNhRB_Rugqaw8oAZRUiiKeHA-6NGzMqQlEm2BxMLMJYKlMKwH1BiYQdkyofIrsj78npiUbCus3fACRxqg

=== List Collections ===
Total collections: 24
Sample collections: [
  'test_collection',
  'demo_collection',
  'schema_employees_client_py',
  'schema_products_client_go',
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
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/transactions.js ===[0m
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: ELfOh6CQKAwWiaE0DMKAeEzGAsRWDZfAJUvPvzsceT4wSaP-fSh7wyD3FJU7MiXIimb_T5tmcx5k0EXTEtrmpw
Created Bob: $500 - ID: BLvLUXZX7c0vqEEMnxJsky7-8b7uk4dJCkSl4KCLRkI_iXUXsw-mD1j_jfS2DVIZMEYl3Q8BxOWrI6FVo57nbQ

=== Example 1: Begin Transaction ===
Transaction ID: e9891cc7-7917-4b4b-a3b8-e2bf2cf10db4

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
New transaction: 81b7a59c-22ff-4b5c-8b08-3c0f2df648f2
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
   ✅ Script saved: 3HzXaNSBhZpg7E7WWasDFDDl8h1vGAnFbRWng5bYAnCMfqxE7C0xl1WXvwiUkZyrZEqFc32QlhC-_1OjKsw0ZA

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
   ✅ Script saved: VDDhDgArfnIjVe2_0KAhC0ek5i-2eMIk0UWPMT4mXrLV_d0yUBRVA_xlqaEILYgpUsDXcu702_C77FJE6xSJmA

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
   ✅ Script saved: 5X96M5jKDRtXqPnNp7OnspHo-bfjLbptzy7VeNIS9Ng7wbeozDn1-O9MsXFW5GN88s7tqpn0JIIg089XUso_pQ

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
   ✅ Script saved: 5-vjlS32dcdxG4UJuLNLVqc_7bT4oQ29JgCGGKPYSYg0sI-qom32ulpaeH9Y3jAEzkdjApWrN1_UOe9UoqpwLg

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: 3HzXaNSBhZpg7E7WWasD...
   ✅ Deleted script: VDDhDgArfnIjVe2_0KAh...
   ✅ Deleted script: 5X96M5jKDRtXqPnNp7On...
   ✅ Deleted script: 5-vjlS32dcdxG4UJuLNL...
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
  Output: Document ID = D2b-TN-lNy4907Afzwat6_TcEVJ30mPJwC4URnzrA05debIu65wjIQR8dkSuRx_9LtuvBZb7yzxZoNUOqUVz9g
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: findById(D2b-TN-lNy4907Afzwat6_TcEVJ30mPJwC4URnzrA05debIu65wjIQR8dkSuRx_9LtuvBZb7yzxZoNUOqUVz9g)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: findById(D2b-TN-lNy4907Afzwat6_TcEVJ30mPJwC4URnzrA05debIu65wjIQR8dkSuRx_9LtuvBZb7yzxZoNUOqUVz9g)
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
  Output: Document ID = WE-Yx_DdacWCEnSB5HcbuiA6qEdfSWaF5FsH5eZKUthYrWAva0W6S_a7sfQtvFroQcSuGRYsFr8N1i3gEnpg4g
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: findById(WE-Yx_DdacWCEnSB5HcbuiA6qEdfSWaF5FsH5eZKUthYrWAva0W6S_a7sfQtvFroQcSuGRYsFr8N1i3gEnpg4g)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: findById(WE-Yx_DdacWCEnSB5HcbuiA6qEdfSWaF5FsH5eZKUthYrWAva0W6S_a7sfQtvFroQcSuGRYsFr8N1i3gEnpg4g)
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

> @ekodb/ekodb-client@0.10.0 prepare
> npm run build


> @ekodb/ekodb-client@0.10.0 build
> tsc


up to date, audited 58 packages in 1s

15 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.10.0 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning JavaScript client library examples...[0m

added 1 package, removed 1 package, and audited 9 packages in 824ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'JQf7WvplfVFS-xH_kB44HgFwvRpGpjssDL4BFrClUYLt6fSjt_pQr9Jlrv8JgLnNCUoJIscX9f-rwCKDrGZBmA'
}

=== Find by ID ===
Found: {
  active: true,
  id: 'JQf7WvplfVFS-xH_kB44HgFwvRpGpjssDL4BFrClUYLt6fSjt_pQr9Jlrv8JgLnNCUoJIscX9f-rwCKDrGZBmA',
  name: 'Test Record',
  value: 42
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  id: 'JQf7WvplfVFS-xH_kB44HgFwvRpGpjssDL4BFrClUYLt6fSjt_pQr9Jlrv8JgLnNCUoJIscX9f-rwCKDrGZBmA',
  value: { value: 100, type: 'Integer' },
  name: { value: 'Updated Record', type: 'String' },
  active: { value: true, type: 'Boolean' }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: btVsbQQT__6hNX2cFx3ilhk230mToNGXPbdXPjnAgq7rf33PqZszCSEYa3mz-L1lHXX1uUQQsCeqfMrwnt9xTQ

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
Collection created with first record: aUqhbMSk-J4xWRnUJWcTyEI-Fdxa_ZC6m73wTKcWAzZv3jLUhciv22CR6bbqMkQm2XJD-5uZKcl83vsnyjuWcw

=== List Collections ===
Total collections: 23
Sample collections: client_collection_management_js,test_collection,schema_employees_client_py,schema_products_client_go,schema_documents_client_ts

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
Created Alice: $1000 - ID: Z5HN5f-TbJTVAdvopE4jQyTYKlDvD-GLK_5UMN2nANnW0T7mcL2at_iT5Gmk6EjOsDCbVwFybOUNokoZ0NaFPg
Created Bob: $500 - ID: H_dlrrEzd4FcZs9x6iJ-KU5iQIjjPgHISQ1N2EsWsooyeFyG8CcXNf10Io_AuuU3dphUWbn7ItuP1RX3bTGKoQ

=== Example 1: Begin Transaction ===
Transaction ID: 848069cb-883a-4398-839f-773c85f28946

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 0b1a551d-6229-4f45-929e-67c7d8cee723
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
  1. Score: 26.400, Matched: title.value, bio, title, bio.value
  2. Score: 26.400, Matched: bio.value, title.value, title, bio
  3. Score: 26.400, Matched: title, title.value, bio.value, bio
  4. Score: 26.400, Matched: bio.value, title.value, title, bio

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio.value, title.value, bio, title
  2. Score: 39.600, Matched: bio, title.value, bio.value, title
  3. Score: 39.600, Matched: bio.value, title, title.value, bio
  4. Score: 39.600, Matched: bio.value, title, title.value, bio

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
  2. Score: 0.759, Matched: 
  3. Score: 0.747, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.704, Matched: title, title.value, content.value, content
  2. Score: 1.499, Matched: content, content.value, title, title.value
  3. Score: 0.303, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: skills, bio, bio.value, skills.value

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
✓ Inserted document: 1n0qRdECIuNFr9pATPInimsgoF4x3ZT71AgFjcGKQu_dzIxlTVmdO4HSjX2JuQREVgED0TyeE47ZAcJnhc4ozA

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: ajfM5lplOj2F8zuVVZpHWQ0mrFPQtto2E9N5zGEiaCJnO4Dnv9mdWU13wmmfBIwFNGC5Cu3htFQ1U90tOX4qvA

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
✓ Inserted document with TTL: CFetq-PT3yRHq5fmRtP_dC7H2GbH_XgBYtXLqgSnn9Xe5fk6LGFlS3OM98W82jLjEcAwdbKTVpq55R2Sp-ZBKw

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
✓ Edge cache script created: aNEAtbElafLDfMVqLTBxmX_mnHPK7jubgf07bN0P2jdOyvNezrGyqwFRSp1KrsbKaHfwb6RgDcxQSe3b3ALzjQ

Call 1: Cache miss (fetches from API)
Response time: 3ms
Result: {
  "records": [
    {
      "value": {
        "current_units": {
          "interval": "seconds",
          "time": "iso8601",
          "temperature_2m": "°C"
        },
        "elevation": 32,
        "timezone": "GMT",
        "timezone_abbreviation": "GMT",
        "longitude": -73.99308,
        "latitude": 40.710335,
        "current": {
          "time": "2026-01-28T14:30",
          "interval": 900,
          "temperature_2m": -8.1
        },
        "generationtime_ms": 0.02181529998779297,
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
Response time: 4ms (0.8x faster!)
Result: {
  "records": [
    {
      "value": {
        "current_units": {
          "interval": "seconds",
          "time": "iso8601",
          "temperature_2m": "°C"
        },
        "elevation": 32,
        "timezone": "GMT",
        "timezone_abbreviation": "GMT",
        "longitude": -73.99308,
        "latitude": 40.710335,
        "current": {
          "time": "2026-01-28T14:30",
          "interval": 900,
          "temperature_2m": -8.1
        },
        "generationtime_ms": 0.02181529998779297,
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

✅ Script saved: oQBTo54G6g_TFakrLbjSuyu7C2fMYXn94rEu5mXQYPwU62NpqgKvH3fjeUOwvxZBO2bCROQu5PMXvMZ4WFhPqQ
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
    "name": "Leanne Graham",
    "phone": "1-770-736-8031 x56442",
    "company": {
      "name": "Romaguera-Crona",
      "catchPhrase": "Multi-layered client-server neural-net",
     ...

Second call (cache hit - from cache):
   ⏱️  Duration: 2ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "name": "Leanne Graham",
    "phone": "1-770-736-8031 x56442",
    "company": {
      "name": "Romaguera-Crona",
      "catchPhrase": "Multi-layered client-server neural-net",
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

✅ Script saved: 1033bgC7sg09bw83v8WGWgn3jo72t1QjkkQXkbjv1_LCBiH9ApIUZrMPzehrkwoS8t1F-Z_qDBPSYDZaCPBQXQ
📊 Found 2 product groups
   {"count":3,"avg_price":575.6666666666666,"category":"Electronics"}
   {"category":"Furniture","avg_price":474,"count":2}
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

✅ Inserted order: 0EwU3FeYXY31P7z8XOqntO53QdqFVxQANLRbQa6__VNQyf8xwyDcMrR4b09ESiRBUePfG9oqH2w5M_4AXFySmQ
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: 4N58Cr4UlCfXfv6TDsc6NdmAN3AYZMRB5HRWfyeSxST_O06Io_HLCXExEzHBEM0vfZ57XoyA9k7Ai861TRu7TA
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"userId":"user_abc","role":"admin"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: Gp7ppVYJA3zSaWpS1ZL76E-qveVbvVcCrq-IiOchAVghkeuCd7bO_FeWHnu2AJnrnYE_2ES1Xnwhi65MiH7bag
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Script

✅ Script saved: WWkgACpoOpSIz513TDcAqIKbSo6A2S3qUkFFcaqg5oBW6Y3WFicgiKXoKRkWkmh0rfLr2LWEc-yy_lVpxWlfiw
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
   1. User 3 - Score: 30
   2. User 4 - Score: 40
   3. User 1 - Score: 10
   4. User 10 - Score: 100
   5. User 2 - Score: 20
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
   2. Introduction to Machine Learning (AI)
   3. Vector Databases Explained (Database)
   4. Database Design Principles (Database)
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
   1. Getting Started with ekoDB
   2. Introduction to Machine Learning
   3. Vector Databases Explained
   4. Database Design Principles
   5. Natural Language Processing
⏱️  Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Script saved
📊 Projected documents (showing first 3):
   1. Getting Started with ekoDB
   2. Introduction to Machine Learning
   3. Vector Databases Explained
⏱️  Execution time: 0ms

📝 Example 5: All Document Fields

✅ Script saved
📊 All documents (5 total, showing first 2):
   1. Getting Started with ekoDB (Database)
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
   1. Wireless Mouse - $29 (⭐4.5)
   2. Monitor 27" - $399 (⭐4.6)
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
   Vector databases offer several advantages including:

1. Precise Geometric Representation: They can accurately represent geographic features like points, lines, and polygons.

2. Topology: Vector databases can store topological information which is helpful in understanding the spatial relationship and connectivity between features.

3. Less Disk Space: They typically require less storage space compared to raster databases for representing the same level of detail.

4. Scalability: Features in a vector database can be easily scaled up or down without losing their shape or details.

5. Attribute Storage: Each entity in a vector database can have a set of attributes associated with it, providing more detailed information.
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
✓ Created SWR script: fetch_api_user_js (uY3jaCX_2Vb54ctjouz9KR9Y4-y6RQh9iYH9J7hoTRSwdxd2RlZOrEu72g6DJtc3pX1f8SHPsjSvmJBRqeR7KA)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "email": "Sincere@april.biz",
        "name": "Leanne Graham",
        "address": {
          "zipcode": "92998-3874",
          "geo": {
            "lng": "81.1496",
            "lat": "-37.3159"
          },
          "city": "Gwenborough",
          "street": "Kulas Light",
          "suite": "Apt. 556"
        },
        "id": 1,
        "phone": "1-770-736-8031 x56442",
        "company": {
          "name": "Romaguera-Crona",
          "bs": "harness real-time e-markets",
          "catchPhrase": "Multi-layered client-server neural-net"
        },
        "username": "Bret",
        "website": "hildegard.org"
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
✓ Created native SWR script: github_user_native (LBZEi3WVei7uSWVm_vNLisT_SiSrJoQ30vdJtv2dZDtmz9hs76WGrmh-tctn5P0dMAinhNcm8sOmx_5ql88ZmA)

First call (cache miss - will fetch from GitHub API):
  Response time: 3ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 3ms
  Speedup: 1.0x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (0TKHqthQK9VUsuTmQuXTpVQh-KOEZNsCOmYp1-_bmOdsDZoxWQ9Eo41GW7DSMGotocTD2wweaR_xFm8N4Mim0w)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (clj-CEWLEhFWR-bIUKYPrSjaBSW_IbYGu66otQbRfW6tcszOOGDSEMY_uTGxdnthho0VdO616F73NNkRN_dkrw)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (tZp8zYCW3s-4jqZTz8FxWzP7D-WAMrKEa4lXoAXtFHBObENHBB8__zEkBruvTxj9vdWYttwSXItUeztaQRJpKg)
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
✓ Created session: z9HVkSRnCr1NqfIZJeFw7KLlN6LsYL9cEZgyjPXNM8DF98qseebhYOGf3Vme6HWOhdfRir1EwIDaNMjv9_SZvw

=== Sending Chat Message ===
Message ID: PJJz_9TxggdZdt99n8YWYgklrQyU6f31B5SVPmfZyKbYgzbWqHCiNYafBOqD44z4U8-fR4wHDmXWbdksIEB86g

=== AI Response ===
There are three products available:

1. ekoDB: This is a high-performance database product with AI capabilities. It is priced at $99.

2. ekoDB Pro: This is the Enterprise edition product with advanced features. The price for this product is $299.

3. ekoDB Cloud: This is a fully managed cloud database service product. It costs $499.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_js',
  record: {
    name: 'ekoDB',
    price: 99,
    id: 'hgFxHUfRoM4MVMqV7KUqbgdGYpVrW-rP1qJsLS9xagABdSNU_9TdHa9DLmpaHZshQOWY3oFzLwDhKDPFf0ZDjg',
    description: 'A high-performance database product with AI capabilities'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_js',
  record: {
    name: 'ekoDB Pro',
    id: 'D3265Yv3fY6OJBqG_4G9QB1EVPApnziU3zQAGr68YsTjOU_w5hLTsw2zz7FKgo_Hi8qm5odsTYLKqlgY24cPIQ',
    description: 'Enterprise edition product with advanced features',
    price: 299
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_js',
  record: {
    description: 'Fully managed cloud database service product',
    id: 'vW9X_9t20VktSblhrRTkAakxpIVRdY8EVAwpHysQP7E8pudVASfbm71gJx9O75Vhg2zOKRo4XLgEIm9_f0b5Rw',
    name: 'ekoDB Cloud',
    price: 499
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 2990ms

=== Token Usage ===
Prompt tokens: 454
Completion tokens: 78
Total tokens: 532

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: OWgYeEj8YAJPcqzlQYocYK4Fn6uv2ytv0yX845EuUUGBYHMybUG4Ei6XLea0SbSUdCLdlifTdavvCAWF6yw-2Q

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the context provided, one product available is the ekoDB. This is a high-performance database product priced at $99.

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
✓ Created second session: f-Iz4fkpBDCNVNuo8ZhyHuWsxLAErr8sZSsTljmQmHfLmWcaGJ1OtIPqVGM2ivSRpTqD_ZjmkvynG3uAo6Kj6w
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
✓ Created session: dyMn_CQXuRuU0VD172c0jwXj_dfp95ZOf0vXpoYOWSlQnw8ZbKbpd5nqNVv3kL_dpAn4Hv1K7V_isF8BjyDlvA

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the provided context, the product available is "ekoDB". This is a high-performance database product. The price for this product is $99.

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 3E15vBJGGbVz-uikV38UfU1otIEdqw9pBt-NROycPxm4OFoVT0g4f5qxtCzrblISAV8QIYMgdXwZDuUoLOnVRQ
  Parent: dyMn_CQXuRuU0VD172c0jwXj_dfp95ZOf0vXpoYOWSlQnw8ZbKbpd5nqNVv3kL_dpAn4Hv1K7V_isF8BjyDlvA

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: 3E15vBJGGbVz-uikV38UfU1otIEdqw9pBt-NROycPxm4OFoVT0g4f5qxtCzrblISAV8QIYMgdXwZDuUoLOnVRQ (Untitled)
  Session 2: dyMn_CQXuRuU0VD172c0jwXj_dfp95ZOf0vXpoYOWSlQnw8ZbKbpd5nqNVv3kL_dpAn4Hv1K7V_isF8BjyDlvA (Untitled)
  Session 3: f-Iz4fkpBDCNVNuo8ZhyHuWsxLAErr8sZSsTljmQmHfLmWcaGJ1OtIPqVGM2ivSRpTqD_ZjmkvynG3uAo6Kj6w (Untitled)
  Session 4: z9HVkSRnCr1NqfIZJeFw7KLlN6LsYL9cEZgyjPXNM8DF98qseebhYOGf3Vme6HWOhdfRir1EwIDaNMjv9_SZvw (Untitled)
  Session 5: GYHT9zYOVdXgEhk0b__a4zDFK7BlE_ssfCwNvfmSW2frp_PTbUEb9CvLMTtU0EKpKmM865Zjja1r3tcJvKdOxg (Untitled)
  Session 6: hmbx1ZZxoOWmGj_0AQLn9Wt9YqAeO0AIij2BjuxQaZLAjfrCUTPC3zXHd38c6P1fQIBtCv9KCmzOYXUAGv5AmQ (Untitled)
  Session 7: 5VxkK5XqmQNMMNMqnUslIdNje9jUtma8DquZ8oESCZX6aO0IsEeDoYvnjt-Pw0wrm8ML8rvmXXA3AuZJvQs-Pg (Untitled)
  Session 8: qwggQOSdymEFPxI-CDWMeoPhlfjoc_c1TSR2kf9tVaz5J364voZ9PmYnY_vM7b2H8YbjJrJUPoClKCM7UXlxKQ (Untitled)
  Session 9: 42HtILejTOAtLG-rbpmagdu_I4oYxQIwSEEJMpClKJ4GPCTHbTL1eyQ6kk0UjMjmyOttAupCiyTkqM_E7rjXLg (Untitled)
  Session 10: Laqlb5tCJrvEV9bzczXZAW85eaZP17Cwy2HXexrLvLAifTB6Psy2cwD0WnEoNgtIgHdpP3Jb-P94S0MsoRE-UA (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: 3E15vBJGGbVz-uikV38UfU1otIEdqw9pBt-NROycPxm4OFoVT0g4f5qxtCzrblISAV8QIYMgdXwZDuUoLOnVRQ

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
✅ [32mJavaScript client examples complete![0m
