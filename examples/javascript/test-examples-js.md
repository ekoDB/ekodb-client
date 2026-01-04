make test-examples-javascript
🧪 [36mRunning JavaScript examples (direct HTTP/WebSocket)...[0m

added 1 package, removed 1 package, and audited 9 packages in 991ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
[34m
╔════════════════════════════════════════╗[0m
[34m║  ekoDB JavaScript Examples Test Suite ║[0m
[34m╚════════════════════════════════════════╝[0m
[34m
=== Checking Server Connection ===[0m
(node:6053) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
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
(node:6056) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
Inserted: {
  id: '85AtX5Wb5lKSOv0R8nycBd_HB1kzDp13N4G1b3-stnKzJ7AuprctHdYRaTbXhoEdmDJxiZZkxyO943nmZH--ew'
}

=== Find by ID ===
Found: {
  value: { value: 42, type: 'Integer' },
  name: { type: 'String', value: 'Test Record' },
  id: '85AtX5Wb5lKSOv0R8nycBd_HB1kzDp13N4G1b3-stnKzJ7AuprctHdYRaTbXhoEdmDJxiZZkxyO943nmZH--ew',
  active: { value: true, type: 'Boolean' }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  name: { type: 'String', value: 'Updated Record' },
  id: '85AtX5Wb5lKSOv0R8nycBd_HB1kzDp13N4G1b3-stnKzJ7AuprctHdYRaTbXhoEdmDJxiZZkxyO943nmZH--ew',
  value: { value: 100, type: 'Integer' },
  active: { type: 'Boolean', value: true }
}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
[32m✓ simple_crud.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/simple_websocket.js ===[0m
✓ Authentication successful

=== Inserting Test Data ===
(node:6061) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
✓ Inserted test record: muEECKr4iZ9jcK8ybmHjWSqSJfvxiG99pBBEmhzm4bXdJ-tE5goFE_eDB2IETcvlZeGzBk_FqKEhYI_A50QKgg

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
        "id": "muEECKr4iZ9jcK8ybmHjWSqSJfvxiG99pBBEmhzm4bXdJ-tE5goFE_eDB2IETcvlZeGzBk_FqKEhYI_A50QKgg",
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
        "id": "a3HSAeDJ4dkj4hErQeiyqFAv339ot0i90jIBizcekkcFWpdStzLfwrQA-ejOxfbPFxFuF9w977m4x7DpOIeeUA",
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
        "id": "z9LcWWNnG9qb3sE9I9ROC59IDx8hjNWn2hIqE9g6JS3nfKdpi4aLfs8iMwbAqZy7BhMTRTR6uqt3iSx83kVaIw",
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
        "id": "nM-4Yeiy_NNKrQ8nYlvtKtF7PcI8tNE5I1l7Gsolj9YOSEoI5ZHPa-lL6fSZAOlMYl6LvJkAmUO3NNZv-wV4Aw",
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
(node:6074) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
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
(node:6077) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
✓ Set key: session:user123

=== KV Get ===
Retrieved value: {
  loginTime: '2026-01-04T06:33:24.098Z',
  userId: 123,
  username: 'john_doe'
}

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: { price: 29.99, name: 'Product 1' }
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
(node:6080) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
Collection created with first record: kmuijEmiUkof8O94g1pNJhrGi233YqPMMQF1UGyf9-zCiM6o4K71J0Xlldqeu3GtcliW5kh8YuquO6SzRruGxA

=== List Collections ===
Total collections: 23
Sample collections: [
  'schema_documents_client_py',
  'schema_products_client_ts',
  'schema_products_client_go',
  'batch_users',
  'schema_employees_client_ts'
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
(node:6094) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
✓ Inserted document: lnF4IkLAqcvvzQb1abbQEiVj8rOsEThCiLnYweCOjeOCJS-1GqOfbalBmFT64ireQK14sCqFefJiMbFDVAFQxQ

=== Insert Document with TTL (5 minutes - integer) ===
✓ Inserted document: created

=== Insert Document with TTL (30 minutes - duration string) ===
✓ Inserted document with duration string TTL: n3_u4F92HpeF6kOo1g6hiZg2DNmgw31yl_RwyXSmCD1uCqS_UqvJh5-wR2aWG2T82w0bXWIHXIC0a0J5R0wuEQ

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
(node:6097) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
✓ Inserted document with TTL: aGh43GD9b_dtbPmqFaBMZ6OM-nzJcHU68khMCAJ6aPLQZMoeJJpM2ldtt96kFdm9CmiChDk_jP7yBD6BIsVa5w

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

✅ Script saved: 2x0l4W1SLiuDovKBULs0NcxtMhCly-Qi8bTDmxpdDcmjFGadFJuApITNGuxumUl9gHon-8uKRKvuxORd0Wr8Qg
📊 Found 20 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: bFtwY3-W1Bg5rXL4CfMzwnBS9PSv0EH7j8RllOxrpFk0maed_tlhQj1_g1k2lMUMjWxIfAFAx6ftMyIjRrCAnA
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: P7SW6WBXgrIMTtMpRa6aq8BSzgkWynGVs6U0Y-8vfSE-nX6LH1jYg44ie97zlpz0EGWnJu6miXaV6QVa-W7DIA
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"max_score":100,"status":"active","avg_score":60,"count":10}
   {"count":10,"max_score":90,"status":"inactive","avg_score":50}

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
Created Alice: $1000 - ID: OXv4WZ8Ile04sWTgfoFQE3aa4tfhzY5-VkGXT41eaKrLjVBZT2QzVFXWHlK1I3pQC8MK4Sw2zHIgPdBZPg2dBw
Created Bob: $500 - ID: HUxYOHyN77EhGoPxjmR1gxQp4s27tfPIVycBRMpOrx00uBtqBiu7nF32tjcYJNZibfxA99aBJsUcyQsFUB5pUw

=== Example 1: Begin Transaction ===
Transaction ID: a1972fb2-1775-41bb-9187-73bafd39ecc5

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
New transaction: 537b36e9-fbd1-49d8-811c-7350b4243247
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
   ✅ Script saved: t9S4S1LHW_T1q15-uzuiAEkKqor0KaIWeFTXTgnwR6V1kvosTN74vDduX6ulDrDyCzeTKGThqhu6NbRT5MEvuQ

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
   ✅ Script saved: KCzZHzjjyzIMcc2GXfgPZJ8eyWt5yLEWDisQV4kyLpryDwq_gqqoSEP10Ut5PsdokomvpjTu9QV3r1YsdI3UVw

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
   ✅ Script saved: -IsIHyrxh5GrAUuVRekDCoHmhfX0-z0nZMG5ptwFtkLK9bRLEIjU4jsnwTTNAPpT50MrxSPMIZhBg53ZgB_rqg

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
   ✅ Script saved: MB-pwX-AvMx4vovJxO7KpfX_xN454mXwsndjPPV4PWvD2ioqLPkucriGA4X6OOdLsqMn7sTnZEZdERKZvJXk_A

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: t9S4S1LHW_T1q15-uzui...
   ✅ Deleted script: KCzZHzjjyzIMcc2GXfgP...
   ✅ Deleted script: -IsIHyrxh5GrAUuVRekD...
   ✅ Deleted script: MB-pwX-AvMx4vovJxO7K...
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

> @ekodb/ekodb-client@0.7.1 prepare
> npm run build


> @ekodb/ekodb-client@0.7.1 build
> tsc


up to date, audited 58 packages in 1s

15 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.7.1 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning JavaScript client library examples...[0m

added 1 package, removed 1 package, and audited 9 packages in 772ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'VWboFIkP7aDhsApB85jtSbtzJwjJdTCKVzhCgblfr-ZmUQYLBRKBTVXf5lt_uBP3fA5gqLuyhjp5vqZRPhJYEA'
}

=== Find by ID ===
Found: {
  name: { type: 'String', value: 'Test Record' },
  value: { type: 'Integer', value: 42 },
  id: 'VWboFIkP7aDhsApB85jtSbtzJwjJdTCKVzhCgblfr-ZmUQYLBRKBTVXf5lt_uBP3fA5gqLuyhjp5vqZRPhJYEA',
  active: { type: 'Boolean', value: true }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  active: { type: 'Boolean', value: true },
  id: 'VWboFIkP7aDhsApB85jtSbtzJwjJdTCKVzhCgblfr-ZmUQYLBRKBTVXf5lt_uBP3fA5gqLuyhjp5vqZRPhJYEA',
  name: { type: 'String', value: 'Updated Record' },
  value: { value: 100, type: 'Integer' }
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
Total keys in store: 4

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: false

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: lJnMmG5x2SQzsyEj7C1p4cleOJE7jCfOikO-gs8pZHwIHNMCMOGgagpnBxDcftkOrvfmYH6BNoUYZdFEmMpyPQ

=== List Collections ===
Total collections: 22
Sample collections: schema_documents_client_py,schema_products_client_ts,schema_products_client_go,batch_users,schema_employees_client_ts

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: hMRCtReQscKHNHVuH9GlQH0hXuLD0Nkps0dAQW1qo4PBJW3YFgcIwWfFgkGJqL_XHjCDDV0_5pUSNILkpcdHjQ

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: DkVzau5S2WC61Oat2SVXKg909TJpsosbzsQWdeJczTyNqXTUtHl_lmYt8ZjbaqQ_ANj7R56AcXLFr9Fjsk1oeg

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
✓ Inserted test record: bE-cZ1uRa9w4AmpX4nsnveajSQQ364Q25TAd8wnPEmpFl48GRIXOLykMWiPquZyqK0s9VpAj42nZPkidktVAgw

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
✓ Inserted document with TTL: Vy-hNqH_i7jw7kutDjuaSKmCU0DI3qA8NMpI6r3HXmhOvGKHbN-oJkleo1YPPCMqG8OkqbYsKnEEO4kz0GCryA

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

✅ Script saved: wDLZJeZQoYtEqgsRADseirHlHVY-Tb48XPc5Layp4EmGCG9VJahaSRTRUJ-hB71RYd0DVicylNEdJxDJg8k_Mw
📊 Found 10 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: lGY_uQHuy-IClbQLoqvuDbJ55jWsY4M8ra4jtf85VKypAzB5n9_8nSFLNRCwJ_ufyrCJxVCFOD5sFUYM7T5LJQ
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: LiKUjLZeriwUIebNfgMX-O6HuZkxMrMV7WF0VgkzFJxtC4EW3g2zvAlx9ReGG8lY5pJyldQbXqKi8iCu9cL4sQ
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"status":"active","max_score":100,"avg_score":60,"count":5}
   {"max_score":90,"status":"inactive","count":5,"avg_score":50}

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

✅ Script saved: pLehHpCGlCug2dCMGm4_18nDODevk8SsS0Qw_cbzbBKM6OLFMsr8O1C9dhIRXLewsYuLHEHJpV-3f2CkweQehw
📊 Found 20 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved
📊 Found 20 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved
📊 Statistics: 2 groups
   {"count":10,"avg_score":60,"status":"active"}
   {"avg_score":50,"status":"inactive","count":10}
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
   1. Desk Chair - $349 (⭐4.2)
   2. Bookshelf - $149 (⭐4.1)
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
   2. Natural Language Processing (AI)
   3. Vector Databases Explained (Database)
   4. Getting Started with ekoDB (Database)
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
   1. Introduction to Machine Learning
   2. Natural Language Processing
   3. Vector Databases Explained
   4. Getting Started with ekoDB
   5. Database Design Principles
⏱️  Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Script saved
📊 Projected documents (showing first 3):
   1. Introduction to Machine Learning
   2. Natural Language Processing
   3. Vector Databases Explained
⏱️  Execution time: 0ms

📝 Example 5: All Document Fields

✅ Script saved
📊 All documents (5 total, showing first 2):
   1. Introduction to Machine Learning (AI)
   2. Natural Language Processing (AI)
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
   Vector databases offer several benefits including:

1. High Precision: They store data in points, lines, and polygons, allowing for the representation of geographical features with high accuracy and detail.

2. Scalability: Vector data can be easily scaled up or down without loss of quality or resolution.

3. Small Size: They generally consume less storage space compared to raster databases.

4. Attribute Storage: Vector databases can store attribute information such as names, categories, or other characteristics, directly linked to each vector feature.

5. Better Analysis: They are suitable for advanced spatial analysis and operations such as network analysis, proximity analysis, and topological relationships.
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
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: g1D5KCmIF6wjK6ppOG7WAUrXRNtfXpu1TLnw8wneIkeLtWZAOZb4ZaulhMVG0mS3cWCDreoyLVz-kANCeNVKOg
Created Bob: $500 - ID: YZrjj0lSZo5FYKoGveSjHXhO57M2IbZ1N7kplaSh0l2iDmbeiD_EWa2ZNxAgU0mThs24rgQb2zD_58yZhT1HzQ

=== Example 1: Begin Transaction ===
Transaction ID: 264e345e-5c5a-4ab2-ac82-785ef56ec501

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 9f5e0de0-76f4-488a-b313-333fe834109b
Status before rollback: Active
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
✅ [32mJavaScript client examples complete![0m
