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
  id: 'zEOSe9-pdnYWlF8kkg1ag04_c_5uyQmJPO3AaDV0p4ndjgeVB3aFB2EuYySCxyg47aFRdKrtvWO9RtTc4C2Skw'
}

=== Find by ID ===
Found: {
  name: 'Test Record',
  active: true,
  value: 42,
  id: 'zEOSe9-pdnYWlF8kkg1ag04_c_5uyQmJPO3AaDV0p4ndjgeVB3aFB2EuYySCxyg47aFRdKrtvWO9RtTc4C2Skw'
}

=== Find with Query ===
Found documents: [
  {
    active: { value: true, type: 'Boolean' },
    id: 'zEOSe9-pdnYWlF8kkg1ag04_c_5uyQmJPO3AaDV0p4ndjgeVB3aFB2EuYySCxyg47aFRdKrtvWO9RtTc4C2Skw',
    name: { value: 'Test Record', type: 'String' },
    value: { type: 'Integer', value: 42 }
  }
]

=== Update Document ===
Updated: {
  value: { value: 100, type: 'Integer' },
  active: { type: 'Boolean', value: true },
  id: 'zEOSe9-pdnYWlF8kkg1ag04_c_5uyQmJPO3AaDV0p4ndjgeVB3aFB2EuYySCxyg47aFRdKrtvWO9RtTc4C2Skw',
  name: { type: 'String', value: 'Updated Record' }
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
✓ Inserted test record: ybK_WRQ5hbJNMIFZgLlTce556xKGqzbq75nO-uNmYeky9iuUpNyx3Gzc3hf4tjIB9jOVR0LBeH-L2sPZNZ_qKg

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
        "id": "XynUeQSPaTfp_nzYBxA4ynDonvZIjV5Ok9f_VEPpIkitQWuzrXR_nHJ9cd9NPM7dOh-40i2EZzq0MpmA7LiPeQ",
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
        "id": "ybK_WRQ5hbJNMIFZgLlTce556xKGqzbq75nO-uNmYeky9iuUpNyx3Gzc3hf4tjIB9jOVR0LBeH-L2sPZNZ_qKg",
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
        "id": "xqlWFz-xcGkzGrnXOkLAPRhrmrsYJH2KINL4W-8E2QvjvbU_O7jY19A6D8om-LLjx6BB85QBiBvAz0MPk3ORjg",
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
        "id": "A4qF-pqdwkFsJuqfXLHpKbs6oAE_YPGEhnmgThECK-CXukWAgzfuHETj4unrK6t9mwdM3JK25MnDvj8OHX8BYw",
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

✅ Script saved: kY0NUhTwl_Rf9XK-_A1NqlMPe1v56eIfNSTbs5RtcOdlciUC3NBdUZlgjurJtuad4NsvxESH1kbw0db9_OkM1g
📊 Found 30 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: svFkeAdtUM4j2GMDK2F_TLoU3hPZxFtQFNzVbX5g6eiaL5xUyjZd-BKJ9D2GjVjY0mpVeHKkb8EWxtAMTnTBiw
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: zlggHCuwheVqu8N_EIZ_nUwetvo-Xf4nYk5bwCQwSW-EnlU3J1taoHW8cfbZH9xfqoeofpC1aAYdFzfIH2q4rA
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"avg_score":60,"status":"active","max_score":100,"count":10}
   {"status":"inactive","avg_score":50,"max_score":90,"count":10}

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
Collection created with first record: CsrTd7fvgR_rpHxFE-J3dohQYq6_R6BDKvZnec-HkujsCzwDqRJD6MYALAF85Mh_pVblAzlMhmVqjsIs0s6LVw

=== List Collections ===
Total collections: 24
Sample collections: [
  'chat_configurations__ek0_testing',
  'chat_messages__ek0_testing',
  'demo_collection',
  'scripts__ek0_testing',
  'schema_products_client_go'
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
Created Alice: $1000 - ID: n7OV7PBRn05g5zYo5hcMEgxzl6GW2VLVEXlka-Flpk0UIdQKTrgFSXqxE3YeZNf9iw0OHfbbU95tMjPTKCV1rQ
Created Bob: $500 - ID: uxSKqX9dYnhldkjJxAU7Ev8m2iKi7kAByVNYtf8_FHTZwn14XFfORMgZq-F9Vn4ACS2ULyfDcUGpCLf8ky-NgA

=== Example 1: Begin Transaction ===
Transaction ID: 08f3209b-a8c8-43a6-b205-dc939e3d8b83

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
New transaction: e47430ef-ac67-438b-8d0f-84fa02609049
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
   ✅ Script saved: HW2LOWHVHlbn7RdBf4R0gVL_6rOxvDXe3LsEbWYOL5eRT53vNQ0JBBjOS1Mu-qYcZWBpnKRs2KgshMd_zDw91w

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
   ✅ Script saved: unCNZzucH7mmglOITJoTJwx20tz58kpi-RWr_BIXe9Wtt7MyOLtw2J9BKXdSsBizg1g9_uPSPE-IP2VH2MkC_w

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
   ✅ Script saved: fVzHHp__BLJ3IeB6fF9yrSdR9dDGKFaxv_5nuffy6AhcNUqOJn1rhR0uX36XoV87J0QO6kywcVxatqYOFDtW0A

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
   ✅ Script saved: pOweenChVuzktl86CDrMzdxSoORkA_SaD3xDOWHJwc5FOp_J8MOeW5foCZG7RI8Mx9GsvNIIs4InVy5VUAY6Vg

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: HW2LOWHVHlbn7RdBf4R0...
   ✅ Deleted script: unCNZzucH7mmglOITJoT...
   ✅ Deleted script: fVzHHp__BLJ3IeB6fF9y...
   ✅ Deleted script: pOweenChVuzktl86CDrM...
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
  Output: Document ID = LPhTO954HfDJUseExMwvMRNal1t7eoPRnyf1FVb516SsfSfRdEvO5kIAtx417GnW3S7yEGPKKImEMWTgV0tjQg
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: findById(LPhTO954HfDJUseExMwvMRNal1t7eoPRnyf1FVb516SsfSfRdEvO5kIAtx417GnW3S7yEGPKKImEMWTgV0tjQg)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: findById(LPhTO954HfDJUseExMwvMRNal1t7eoPRnyf1FVb516SsfSfRdEvO5kIAtx417GnW3S7yEGPKKImEMWTgV0tjQg)
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
  Output: Document ID = 7rR8JggRSCxThA2MIy9koekJwaKXLzOL6S50JSbU5Lfx8kYtTtEFAwbaaBxvf6iNqAuDAhjulX--cCCEuyxZgQ
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: findById(7rR8JggRSCxThA2MIy9koekJwaKXLzOL6S50JSbU5Lfx8kYtTtEFAwbaaBxvf6iNqAuDAhjulX--cCCEuyxZgQ)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: findById(7rR8JggRSCxThA2MIy9koekJwaKXLzOL6S50JSbU5Lfx8kYtTtEFAwbaaBxvf6iNqAuDAhjulX--cCCEuyxZgQ)
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

added 1 package, removed 1 package, and audited 9 packages in 842ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'sxdSzpJXp3zEoC1o9T0O5sq5BR-DzHuryzNvF86WpWmgW6Qy0LTcfIe7xugnnYwfhADN5U8ufcp2cldy_CtDPQ'
}

=== Find by ID ===
Found: {
  active: true,
  name: 'Test Record',
  value: 42,
  id: 'sxdSzpJXp3zEoC1o9T0O5sq5BR-DzHuryzNvF86WpWmgW6Qy0LTcfIe7xugnnYwfhADN5U8ufcp2cldy_CtDPQ'
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  id: 'sxdSzpJXp3zEoC1o9T0O5sq5BR-DzHuryzNvF86WpWmgW6Qy0LTcfIe7xugnnYwfhADN5U8ufcp2cldy_CtDPQ',
  name: { value: 'Updated Record', type: 'String' },
  value: { type: 'Integer', value: 100 },
  active: { value: true, type: 'Boolean' }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: QVSJWpc-1-cst19kCbd7K1TnnFHVbttKPkeO0ozV70XxN9raPSaYSif5wpNK89tVsG4V4BE4W3mp8SguJJyRNA

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
Collection created with first record: lhMfdCNxyR_OXrdfqfuy8aDhmsxV0zCp6hLfysFT0HXCjGKPPjDq8t1bM2aiBE5r3H95YSCSZqEDd3IZy3RBKQ

=== List Collections ===
Total collections: 23
Sample collections: chat_configurations__ek0_testing,chat_messages__ek0_testing,client_collection_management_js,scripts__ek0_testing,schema_products_client_go

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
Total keys in store: 10

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: false

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: tC_XIY2jfXd9dDJlXCKA7wVQImisbn2up6_x1uU5vIjCHSFRhcxizPSgqo2SmfkAxAKANoI4ciDsBSEVODeAsQ
Created Bob: $500 - ID: s3rbbeiAkTFsH9-Cn3vmtXAeg-2okHlgSbVONwJ_feyL396HaKDgBP4q2OxjomCb1Z_D9qL9zocwVX7Kl4qI6Q

=== Example 1: Begin Transaction ===
Transaction ID: dd9553ac-b8f4-4c71-9bd5-29cd178e735c

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: fee1d621-a873-4344-b697-9fb0e1c41edd
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
  1. Score: 25.740, Matched: email, name, email.value, name.value
  2. Score: 12.540, Matched: name.value, name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title.value, bio.value, title, bio
  2. Score: 26.400, Matched: title, bio.value, title.value, bio
  3. Score: 26.400, Matched: bio, title.value, bio.value, title
  4. Score: 26.400, Matched: bio.value, title, title.value, bio

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title, bio, title.value, bio.value
  2. Score: 39.600, Matched: bio, title.value, bio.value, title
  3. Score: 39.600, Matched: bio, bio.value, title.value, title
  4. Score: 39.600, Matched: title.value, bio, title, bio.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.767, Matched: 
  2. Score: 0.764, Matched: 
  3. Score: 0.752, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.706, Matched: content, content.value, title, title.value
  2. Score: 1.507, Matched: content.value, title, title.value, content
  3. Score: 0.301, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: skills, skills.value, bio.value, bio

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
✓ Inserted document: P-DZV1gCKsFUf5P_SGc41HTK_y42jZDbAk_NKMFvHpMC3xAHyrqxbz4Kh0Mwwz0Otg_YTwyA-ay1o68EUZHxGw

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: 84Sz7_RTTfN8hv6bTIVI3qBrO7S1c2ZCDBnQ6XiwJHKDVGQNMf0cY1WyfIxYHW249d0nK96A8pp1va0uta0eSg

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
✓ Inserted document with TTL: y4NrwTcfzcYiw3mcz0nW_W7LRmVLq06dDLSDEPVnIn783ZVjevsTMJRO3zx9PA9KNYHuTeeygGhV1R5qLquXQA

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
✓ Edge cache script created: kMiH9zmXyZ2ZB0N067PRdsz3n-Z8F7TpYPGkENwP7GjmWiC_7quPF1YUaMbxyBd1ezkLgGs1THv2wDmZBW2JOQ

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
Response time: 5ms (1x faster!)
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

✅ Script saved: 3x4r3S5bkAxFrGHHcLv7W_V_otNzEG0pzfOVzREZeMEbTvcCiKVKlZCBp3pVsovrEduDSRatD_EGamFlMBSucg
📊 Found 10 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved
📊 Found 10 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved
📊 Statistics: 2 groups
   {"avg_score":60,"count":5,"status":"active"}
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
   Department: {"value":"engineering","type":"String"}

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
   ⏱️  Duration: 82ms
   📊 Records: 1
   🚀 Cache speedup: 0.8x faster!

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

✅ Script saved: juhpsSqzfn5jRqNaNKXO_N8HghwOYHeQNezomPzFXLJteYrXmybuyJtEDkgevNJJSV3F_0zwru7UR8ab0GDKTg
📊 Found 2 product groups
   {"category":"Electronics","avg_price":575.6666666666666,"count":3}
   {"category":"Furniture","count":2,"avg_price":474}
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

✅ Inserted order: DuBAEGM_Le6l4dihNXLUjzPEaD61wvqT2DqCBM-4ZcTKBv7x_AsAHaTgYaOD_rJenoQ6joYgQ6KWEwpZKPyTyg
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: YQLOaDrDzhmyWVgqG28CKSqEQjI1FbWVjMo5Orj12Kcmy7FEB0XbzMov_uB5rbOhzyU-WvXtXqPPqox2nPaozg
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"role":"admin","userId":"user_abc"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: Jk0VZhAROBExc5q81--tABnK5_g7W-TFz4pD7s77keB5GGWxrQBkZsJPH8HhfhRKL9TmjkquDTGjqpWMDceWgA
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Script

✅ Script saved: 2ojroaPHVBMCuc3ABf_md8NjYT7YE5l7rtygE-m_TbmGF-AlCpgzjnPbtJK5spRbsUCnJnNO12VcftkVY8QIqg
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
   1. User 4 - Score: 80
   2. User 2 - Score: 40
   3. User 3 - Score: 60
   4. User 1 - Score: 20
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
   1. Database Design Principles (Database)
   2. Introduction to Machine Learning (AI)
   3. Natural Language Processing (AI)
   4. Vector Databases Explained (Database)
   5. Getting Started with ekoDB (Database)
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
   1. Database Design Principles
   2. Introduction to Machine Learning
   3. Natural Language Processing
   4. Vector Databases Explained
   5. Getting Started with ekoDB
⏱️  Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Script saved
📊 Projected documents (showing first 3):
   1. Database Design Principles
   2. Introduction to Machine Learning
   3. Natural Language Processing
⏱️  Execution time: 0ms

📝 Example 5: All Document Fields

✅ Script saved
📊 All documents (5 total, showing first 2):
   1. Database Design Principles (Database)
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
   1. Bookshelf - $149 (⭐4.1)
   2. Laptop Pro - $1299 (⭐4.8)
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
   Vector databases have several benefits:

1. High Precision: They provide more accurate and detailed data representation as they use points, lines, and polygons.

2. Scalability: Vector data can be easily scaled up or down without losing quality.

3. Less Disk Space: They typically require less storage space compared to raster databases.

4. Data Manipulation: Vector data can be easily manipulated and analyzed. It supports topological analysis like network analysis and proximity operations.

5. Better Visualization: They allow for better visual representation of data, especially for features like roads, buildings, and other infrastructure.
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
✓ Created SWR script: fetch_api_user_js (na8gZdN8em110bNrownyqNejjOGdZupMUtkB3iVm3kPNYAb-3b75aGqOnyxiwnuRdbrdjBA-m1SQAloRSjNodw)

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
✓ Created session: rabMxHA-ANqjuJ_pCJzsGobYdjEFwnzVErBEqmcNUqRNqsaxFiQ2tOog_mxWfE4V0JzY3kvmelmSFoIFM3LvCA

=== Sending Chat Message ===
Message ID: o45nf6qS5Cf0g58q13DmS-SFHQCST1HQ-hutihLDvP24XGTi8DZKQiZPAPEsPEb2fI_e7GSuL70ZFA-9rUmzjw

=== AI Response ===
There are three products available:

1. ekoDB: This is a high-performance database product with AI capabilities. It is priced at $99.
2. ekoDB Pro: This is an enterprise edition product with advanced features. It is priced at $299.
3. ekoDB Cloud: This is a fully managed cloud database service product. It is priced at $499.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_js',
  record: {
    price: 99,
    id: 'ckWtyjpJLYF7n031vAw-3HzGcilNSSjqi3YPymEJm67GZ6G1VuxxVSxjPd1ztSROfzSSe4jXORO15eMOK9Mc0g',
    name: 'ekoDB',
    description: 'A high-performance database product with AI capabilities'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_js',
  record: {
    description: 'Enterprise edition product with advanced features',
    price: 299,
    id: 'rkTkFV3AaRhSbdl2Gsk9dznI5Idy5J0noTpIl3JwLm6PnXGxQTaHAZW0lyFcKUpZQa6NAdNtdBh9S8fwOKt5Fg',
    name: 'ekoDB Pro'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_js',
  record: {
    name: 'ekoDB Cloud',
    id: 'Fvbx52qBxGNyv2h11tj1SJMylfgZ4o6_fQWYyC265DB3xr7ufQZkjrBLajsaxz1OWpVrU3n1CKTTlf-Y7KklRg',
    price: 499,
    description: 'Fully managed cloud database service product'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 2436ms

=== Token Usage ===
Prompt tokens: 456
Completion tokens: 78
Total tokens: 534

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: kE4YBFaG8DBMiEQmia79C4Rwgwt2JMUI2bXqK71SyC_13m0gRmnIK-t1FXMgZi_7nCCDTaL3iKHoXTngfQ3Yuw

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information provided, "ekoDB" is a product that is available. It is a high-performance database product and it is priced at $99.

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
✓ Created second session: EsXVr0JjLOljobaQLDhs2c9zU2cehpHJnQaKF2XjfYGrUNZxk1yhZrDhlDIH2eUB6DtFii78sTN4BVCCVOS6lQ
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
✓ Created session: B8NDRQztgVLRCEzVIjKSZ_g4ISm0L0KZEtvnqwADmejdHPROpsUaCrnohlwYnsYKKRry8a-UMkSUZ5bJvDvRRA

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information available, one product that is available is "ekoDB". It is a high-performance database product and it costs $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: h2NLXfMd5VLN67oLUuLpS0MwRBYAUrIlcf980Z-43sel5nj_y6D6wvLAhBQlVDGNovBQ4xuLSgVNPhCK3zBvdw
  Parent: B8NDRQztgVLRCEzVIjKSZ_g4ISm0L0KZEtvnqwADmejdHPROpsUaCrnohlwYnsYKKRry8a-UMkSUZ5bJvDvRRA

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: h2NLXfMd5VLN67oLUuLpS0MwRBYAUrIlcf980Z-43sel5nj_y6D6wvLAhBQlVDGNovBQ4xuLSgVNPhCK3zBvdw (Untitled)
  Session 2: B8NDRQztgVLRCEzVIjKSZ_g4ISm0L0KZEtvnqwADmejdHPROpsUaCrnohlwYnsYKKRry8a-UMkSUZ5bJvDvRRA (Untitled)
  Session 3: EsXVr0JjLOljobaQLDhs2c9zU2cehpHJnQaKF2XjfYGrUNZxk1yhZrDhlDIH2eUB6DtFii78sTN4BVCCVOS6lQ (Untitled)
  Session 4: rabMxHA-ANqjuJ_pCJzsGobYdjEFwnzVErBEqmcNUqRNqsaxFiQ2tOog_mxWfE4V0JzY3kvmelmSFoIFM3LvCA (Untitled)
  Session 5: zI2BbtcsCgihdjDA9HAtYTbM1DUUPzL-XQruo1fAU8qhPzCDzdL5MAxLnDSwC7AL6V4BmzZjU6K2Nl7YGC1eNQ (Untitled)
  Session 6: zLj1i9P9gFeDCksqf0ll8FGRv6gHp6Afp1qLA45CyG3Iorxhlt0XLEsoDaei5TNm6rWTg2ArXM9kdbmY06GxVA (Untitled)
  Session 7: cCTbi43y62OYHyTbqEf8g4kL6ATnISdC8ofkyh4kcPbUARjm81LRW8sfyFGLX7T7xvbZLD_08Uci_raZrIgmVw (Untitled)
  Session 8: IrOVWEMKky4KQ6TrgSdxgT6MJoIpcGShelNCnE6DM-UxuLeKBPvkAUMsX67mVo-ZZj0hUujbe4-8KQ2yrzR3AQ (Untitled)
  Session 9: ltoCE9Mp-YHRJtsl5hQz6NbEq3eQLlHVQko1hD7fAv4kWIRFPEQ8lYFYcwbEKxKjiqcO4ZH5mj5uSn5vyDHDEw (Untitled)
  Session 10: iOoS8YtheulgTtAeIuuE8D3bhrehIrH7-gMf9kcjqZ1WhiK4u5IFgblmUK0mnvBgQq4b_DIusk1CVO0wAc81Ew (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: h2NLXfMd5VLN67oLUuLpS0MwRBYAUrIlcf980Z-43sel5nj_y6D6wvLAhBQlVDGNovBQ4xuLSgVNPhCK3zBvdw

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
✅ [32mJavaScript client examples complete![0m
