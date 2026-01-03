make test-examples-javascript
🧪 [36mRunning JavaScript examples (direct HTTP/WebSocket)...[0m

added 1 package, removed 1 package, and audited 9 packages in 645ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
[34m
╔════════════════════════════════════════╗[0m
[34m║  ekoDB JavaScript Examples Test Suite ║[0m
[34m╚════════════════════════════════════════╝[0m
[34m
=== Checking Server Connection ===[0m
(node:73925) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
[32m✓ Server is ready[0m
[34m
=== Getting Authentication Token ===[0m
[32m✓ Authentication successful[0m
[33m
=== Running 10 Examples ===[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/simple_crud.js ===[0m
✓ Authentication successful

=== Insert Document ===
(node:73928) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
Inserted: {
  id: 'bAL064FoR3d47JKFRbvRLoFEWXXpfi25NX2PlHJ1rTjnk9YiblRWiiQ0Ddsijj5KxShqq9_PrmZr9QPJz25PTA'
}

=== Find by ID ===
Found: {
  id: 'bAL064FoR3d47JKFRbvRLoFEWXXpfi25NX2PlHJ1rTjnk9YiblRWiiQ0Ddsijj5KxShqq9_PrmZr9QPJz25PTA',
  active: { value: true, type: 'Boolean' },
  name: { type: 'String', value: 'Test Record' },
  value: { type: 'Integer', value: 42 }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  id: 'bAL064FoR3d47JKFRbvRLoFEWXXpfi25NX2PlHJ1rTjnk9YiblRWiiQ0Ddsijj5KxShqq9_PrmZr9QPJz25PTA',
  active: { value: true, type: 'Boolean' },
  value: { type: 'Integer', value: 100 },
  name: { type: 'String', value: 'Updated Record' }
}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
[32m✓ simple_crud.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/simple_websocket.js ===[0m
✓ Authentication successful

=== Inserting Test Data ===
(node:73941) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
✓ Inserted test record: oDJ_HExGK9xZwnyNk7gJKgs1uQMBorFqUq7ENsILVxbuLxInDFW3cAXmvYsOdlnsIOAuSPIfrbRoQwCtvn27Ag

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
        "id": "pgX3tN0CrrfdwZdfEP7IfhBBi2_OMTlOZKaFUVkClPD4NDbdz0ze2s_DBBIMQ5LTxd-lIMtq_vWFdt_XE62GnQ",
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
        "id": "hVJrJrqbHjCEisTnEM8A-FztQmjbfEL92UsvIUHFCyH4j45VgMzWEtQyv_b1grPXz9_tC-9hntazuuCZdw86tg",
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
        "id": "MC6awzEkQm2HxIFz8YHls2Ws7hmMZSn51NM_fz8xZSMVu3ZXvP8tNXq4OgMXARzOhP5i5qmyxItK6259PZlT4Q",
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
        "id": "oDJ_HExGK9xZwnyNk7gJKgs1uQMBorFqUq7ENsILVxbuLxInDFW3cAXmvYsOdlnsIOAuSPIfrbRoQwCtvn27Ag",
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
[32m✓ simple_websocket.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/batch_operations.js ===[0m
✓ Authentication successful

=== Batch Insert ===
(node:73945) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
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
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/kv_operations.js ===[0m
✓ Authentication successful

=== KV Set ===
(node:73958) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
✓ Set key: session:user123

=== KV Get ===
Retrieved value: {
  loginTime: '2026-01-03T05:27:52.841Z',
  userId: 123,
  username: 'john_doe'
}

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
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/collection_management.js ===[0m
✓ Authentication successful

=== Create Collection (via insert) ===
(node:73962) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
Collection created with first record: IZvF87tZ9N5mtE_9FEASTChhKeEZS63zZxmBkE2P024n7cOzWtvB6AWAnml5mGK4Dxwkst70cAe7RCXjcURxvQ

=== List Collections ===
Total collections: 23
Sample collections: [
  'ws_ttl_test',
  'schema_users_client_go',
  'schema_employees_client_ts',
  'schema_employees_client_py',
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
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/document_ttl.js ===[0m
✓ Authentication successful

=== Insert Document with TTL (1 hour) ===
(node:73975) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
✓ Inserted document: Kpr9U0anU7tO9j_z9H_mHM--8V2lXTO69OVra35GjBhwKlhkQxRzn8r4GEj_zQPIjWOinlW3wKrUyK4WnsIVfA

=== Insert Document with TTL (5 minutes - integer) ===
✓ Inserted document: created

=== Insert Document with TTL (30 minutes - duration string) ===
✓ Inserted document with duration string TTL: Wacdyac1J4dthssRYCGRHqGp63zXAXszwn38m_4hqRSbaVldSPJY9mihEt6WBkTykyzXbu9deUwr2W3w0rrzEA

=== Query Documents ===
✓ Found 7 documents with TTL

=== Update Document ===
✓ Updated document

=== Delete Document ===
✓ Deleted document

✓ All document TTL operations completed successfully
[32m✓ document_ttl.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/websocket_ttl.js ===[0m
✓ Authentication successful

=== Insert Test Data with TTL ===
(node:73978) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
✓ Inserted document with TTL: KMVONVt8V2JVQp5_Fg3sGrAIenMjXaHmHTuj7fMIzcbzfSPR_tjDVVz99F7kTP4EQWLI6I8bEK3b9z66lqylOw

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 4 record(s) via WebSocket

✓ WebSocket TTL example completed successfully
[32m✓ websocket_ttl.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/http_functions.js ===[0m
🚀 ekoDB Scripts Example (JavaScript/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: C80INp62a1v9l9f5N7luZXEayG0sGyWRj_jwLCzt69Vs4UFgNsxp-le-EBCznz-2MQxpPsF2XCyg4wummHjHPg
📊 Found 20 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: sS8i5FKFChznoRUBqwMkKXVsJAxduPmDYaJSWXPQ4l5T7qE6CfGP2hdkyOOvZyUlECeMblcnaT8EQrbF8HcHPw
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: uDD2A7bx7SkeM-QU1v6196qWRFv7sRzr45sTlxxR-jLx10hhQbA5INxOitwFsgAz_BlXt48owgyl3f87NV1www
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"count":10,"status":"inactive","avg_score":50,"max_score":90}
   {"avg_score":60,"count":10,"max_score":100,"status":"active"}

📝 Example 4: Function Management

📋 Total scripts: 20
🔍 Retrieved script: Get Active Users
✏️  Script updated
🗑️  Script deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

✅ All examples completed!
[32m✓ http_functions.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/transactions.js ===[0m
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: X5eGWh2KVDM4rvxJ1Sc1ThDCvhkHleiF15XlLIIXMCqSrrfS2Bj5tt4ieWFhs0QsaVjBJ7CdR0b7mHTAWAQ2gA
Created Bob: $500 - ID: uumF6o8r_DowftmezHPb6mq9A3m07_LBj-CJ1pdndinT6NobhH1jBuXUEgXAblQ6EOIPlr46qxdmRvhmR9R87g

=== Example 1: Begin Transaction ===
Transaction ID: 2c0a35fe-5071-41fe-8563-8f6f50fbd350

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
New transaction: c914606b-bac0-48e0-ab17-e6cfd73c9ad7
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
   ✅ Script saved: VTfI7EP5G5a5gvNelp_MdwtLiHnJcJXAul5QcczRJXnHFoPL8NzqIFW2F9im_LYGMlgab1V6KdF7LJNWtr_Ujg

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
   ✅ Script saved: 5QZ8ajoshV8euNjw3mHoSSjM2oqo2GM-ltuyo61c7K7x3K_rSx3x1Zlo5BVH8HHdtQtgYa9lqQ6i7oY7l4xeKg

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
   ✅ Script saved: ubVlSl69WTVIQt6TYfxZW8iY7QH7bicOF-3LbsYPNZXKFTC09Ay3IbFiLQCtCYtPas8T7FFtBqtjl70tASoIIA

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
   ✅ Script saved: 1xWjnMPSNoRptFePLDMFxTVM2LABVPZnlTV8eWV9eRhoiFkGxqJSYArxL1WHK_wQCNfOnsnL2frttHx9dN3OmQ

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: VTfI7EP5G5a5gvNelp_M...
   ✅ Deleted script: 5QZ8ajoshV8euNjw3mHo...
   ✅ Deleted script: ubVlSl69WTVIQt6TYfxZ...
   ✅ Deleted script: 1xWjnMPSNoRptFePLDMF...
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
╔════════════════════════════════════════╗[0m
[34m║           Test Summary                 ║[0m
[34m╚════════════════════════════════════════╝[0m
[34mTotal: 10[0m
[32mPassed: 10[0m
[32mFailed: 0[0m
✅ [32mJavaScript direct examples complete![0m
📦 [36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.6.1 prepare
> npm run build


> @ekodb/ekodb-client@0.6.1 build
> tsc


up to date, audited 7 packages in 823ms

found 0 vulnerabilities

> @ekodb/ekodb-client@0.6.1 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning JavaScript client library examples...[0m

added 1 package, removed 1 package, and audited 9 packages in 650ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'QaF-Hi-y0O7g4zJHoz8QqqAah0gXonnWMoGwVKPJo3cG0oc5N1EfBekv1uMamSIwehXHfzFS3GNPu-jGI8xUpg'
}

=== Find by ID ===
Found: {
  active: { type: 'Boolean', value: true },
  value: { type: 'Integer', value: 42 },
  name: { type: 'String', value: 'Test Record' },
  id: 'QaF-Hi-y0O7g4zJHoz8QqqAah0gXonnWMoGwVKPJo3cG0oc5N1EfBekv1uMamSIwehXHfzFS3GNPu-jGI8xUpg'
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  id: 'QaF-Hi-y0O7g4zJHoz8QqqAah0gXonnWMoGwVKPJo3cG0oc5N1EfBekv1uMamSIwehXHfzFS3GNPu-jGI8xUpg',
  name: { type: 'String', value: 'Updated Record' },
  active: { value: true, type: 'Boolean' },
  value: { type: 'Integer', value: 100 }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
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

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: { username: 'john_doe', userId: 123 }

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
Total keys in store: 4

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: false

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: Q-wyi46JNKyhbwPKMHny7CITAC7p0uE4TbdH1tEC3Ez9YXMFbvWMFPVczBJ_DkKX2NjQmcaIlzHZ9dBCxL2Jgw

=== List Collections ===
Total collections: 22
Sample collections: ws_ttl_test,schema_users_client_go,schema_employees_client_ts,schema_employees_client_py,test_accounts

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: hlYayxwNSQB9laT4Sl4KX84F0sEC8RW_EN_nJtnqmaFFUW70oghq6YS4zKVhwjTuItHXARTK2ONfi9WAp64n8w

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: 5LCFPraJ9wH5zqNyRldID-MVBAxAi1S0a1FYFOFWpANKKjr9Q6KYVSBlPRuTML9WMaH4thqHKAv7yJZGbwoEdA

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

=== Inserting Test Data ===
✓ Inserted test record: qJN9XUhB-tRFxDa8atxjVxPSqo8_Cwa-p6Zw6nzAkPNaQwqznAXaHM8GSpVf4XedZ0Hbr-W-zKRWKnJ2h5Ze-w

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: 0AJfLbMPQsFZ6quykgrr4pDwm_k-R8MWg9arY7d2KeInqkzrcr-HiSb3O2DjkIdmsCO4BRvdJDc3vgk5VV8OEg

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 5 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
🚀 ekoDB Scripts Example (JavaScript/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: xUFWGII_uuc1UDKTUgB9P_ORwGNo5nVR2xx4mHLIJbNXy0bnHkstzWtBYzJyyMk9wqu5z9iEkU1ec3ye2SKahg
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: iVD8_G3waUnERbKp8HQnyHHTfVdc7qLGGcJEM2CRujPgHIrkgNakyZv4-4rX7gtHoIH_0j59ZtloE6C7Tl4RAA
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: MY1W5eSsBdHOcwLq5_Sc1BFgsBI3T-oUvMYPjjHW5F2Vp-UQa5Sc-rGoF-SNT1_cSGETfU7sIHc81Q-RzMMHMQ
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"count":5,"max_score":100,"status":"active","avg_score":60}
   {"avg_score":50,"count":5,"max_score":90,"status":"inactive"}

📝 Example 4: Function Management

📋 Total scripts: 22
🔍 Retrieved script: Get Active Users
✏️  Script updated
🗑️  Script deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

✅ All examples completed!
🚀 ekoDB Scripts Example (JavaScript Client)

✅ Client initialized (token exchange automatic)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: chf3WYGOReRJXwhe1gP9jaSKSYwfpgjitchBBcxcwrq9IfJKruyvFEZuzDpssOz0L8olDw007nVzKUP9XUc54w
📊 Found 20 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved
📊 Found 20 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved
📊 Statistics: 2 groups
   {"count":10,"status":"active","avg_score":60}
   {"status":"inactive","avg_score":50,"count":10}
⏱️  Execution time: 0ms

📝 Example 4: Script Management

📋 Total scripts: 24
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
📊 Total user count: 20
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
   1. Laptop Pro - $1299 (⭐4.8)
   2. USB-C Cable - $19 (⭐4.3)
   3. Wireless Mouse - $29 (⭐4.5)
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced script examples completed!
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
📊 Users (showing first 5 of 0):
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
   1. Introduction to Machine Learning (AI)
   2. Database Design Principles (Database)
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
   1. Introduction to Machine Learning
   2. Database Design Principles
   3. Natural Language Processing
   4. Vector Databases Explained
   5. Getting Started with ekoDB
⏱️  Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Script saved
📊 Projected documents (showing first 3):
   1. Introduction to Machine Learning
   2. Database Design Principles
   3. Natural Language Processing
⏱️  Execution time: 0ms

📝 Example 5: All Document Fields

✅ Script saved
📊 All documents (5 total, showing first 2):
   1. Introduction to Machine Learning (AI)
   2. Database Design Principles (Database)
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples completed!
🚀 ekoDB AI Scripts Example

⚠️  Note: These examples require AI API credentials (OpenAI, etc.)

📋 Setting up test data...
✅ Created 3 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
🤖 AI Response:
   1. Scalability: Vector databases can handle large amounts of data and scale as the data grows.
2. Precision: They allow for accurate computations as they store data in the form of vectors.
3. Speed: Vector databases often provide faster query response times.
4. Flexibility: They can handle different types of data including spatial, sequential, and time series data.
5. Efficient storage: They use less storage space compared to other database models. 
6. Simplified data operations: Operations like filtering, sorting, and aggregating become easier with vector databases.
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
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: d7d5WPWnXOasnCZKG3dsSaX6aEamOSGHDU_Ax6clt8s9GJrh7XC7iawmnJhJgetaf46BsCBdZ9NR7nKuC2gC9Q
Created Bob: $500 - ID: axTplssmbD5UiyhJu1hVmL0td5Daw97U55YJ56ZLou01nZUqd21G6B6R0evNYUMbB3v70egsq32fjhv-H8NQvg

=== Example 1: Begin Transaction ===
Transaction ID: 29137f17-941a-4a31-8c9f-93bc98274908

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 94cde92a-1c6e-4e90-8271-2db4a8716b81
Status before rollback: Active
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
✅ [32mJavaScript client examples complete![0m
