make test-examples-javascript
🧪 [36mRunning JavaScript examples (direct HTTP/WebSocket)...[0m

added 1 package, removed 1 package, and audited 9 packages in 639ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
[34m
╔════════════════════════════════════════╗[0m
[34m║  ekoDB JavaScript Examples Test Suite ║[0m
[34m╚════════════════════════════════════════╝[0m
[34m
=== Checking Server Connection ===[0m
(node:93953) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
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
(node:93956) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
Inserted: {
  id: 'pnv5yXd14wkN4BH33pLjpguOuxlsB7pmV56CjI5VAUUy3tYBK4tLWIF3CUzlOW9q18MKb_d0gHSoi9M31bofWw'
}

=== Find by ID ===
Found: {
  name: { value: 'Test Record', type: 'String' },
  active: { value: true, type: 'Boolean' },
  id: 'pnv5yXd14wkN4BH33pLjpguOuxlsB7pmV56CjI5VAUUy3tYBK4tLWIF3CUzlOW9q18MKb_d0gHSoi9M31bofWw',
  value: { type: 'Integer', value: 42 }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  value: { value: 100, type: 'Integer' },
  active: { type: 'Boolean', value: true },
  name: { type: 'String', value: 'Updated Record' },
  id: 'pnv5yXd14wkN4BH33pLjpguOuxlsB7pmV56CjI5VAUUy3tYBK4tLWIF3CUzlOW9q18MKb_d0gHSoi9M31bofWw'
}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
[32m✓ simple_crud.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/simple_websocket.js ===[0m
✓ Authentication successful

=== Inserting Test Data ===
(node:93990) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
✓ Inserted test record: y1L6y4lPHarcS5MFihkvrFZ1F2GNJQO2pHuopR8W-lTL3ofwBhVP6XVgYs-WQzQWTRIbIL6A2ZOvR7QoV74sRA

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
        "id": "y1L6y4lPHarcS5MFihkvrFZ1F2GNJQO2pHuopR8W-lTL3ofwBhVP6XVgYs-WQzQWTRIbIL6A2ZOvR7QoV74sRA",
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
        "id": "WaBLKHHaUr5EzU2ATdkHLvXtzFbO8-_txXCCsCgLbh3hqPL8p-jLVoUzdHAgJhCQlO1uSOqvXl_I5gzOCfGMxA",
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
        "id": "j7IBJs1nREsgu7DnEMCXSpYq6P2tC4E6brhLBeEkUZezwJw1xqAQ4FybRLrm8to7RsLAZdjORHzug55oAO38eA",
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
        "id": "EHEYTwN4dvGdPiFjeN68OBYQo7RXh3x50hrjyfNlRu14LvgPwHcWDqQ-lzEqpzxKv9OGmvlLZReOEVa3sk9Szw",
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
(node:93993) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
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
(node:94006) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
✓ Set key: session:user123

=== KV Get ===
Retrieved value: {
  userId: 123,
  username: 'john_doe',
  loginTime: '2025-12-31T17:05:32.522Z'
}

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
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/collection_management.js ===[0m
✓ Authentication successful

=== Create Collection (via insert) ===
(node:94009) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
Collection created with first record: YOXzAl40QEfAsfvL4oUtyEQMmhSnZ43CzvL_Hzqq_-Kqweer1iRHxMuzN8GQkQ2eh4MgT_sxdq7saYm9SRpHpA

=== List Collections ===
Total collections: 23
Sample collections: [
  'schema_products_client_ts',
  'schema_users_client_ts',
  'chat_messages__ek0_testing',
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
(node:94012) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
✓ Inserted document: B056YZ-1FoGmvagz7x3aFptXybX1UvPNs9uKYbp6NITCEmSedw2RJuUQWC_eNPWT0XpXJhzrsjQ74F724T0EAg

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: created

=== Query Documents ===
✓ Found 4 documents with TTL

=== Update Document ===
✓ Updated document

=== Delete Document ===
✓ Skipped delete (no ID returned)

✓ All document TTL operations completed successfully
[32m✓ document_ttl.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/websocket_ttl.js ===[0m
✓ Authentication successful

=== Insert Test Data with TTL ===
(node:94026) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
✓ Inserted document with TTL: TvrwmjIwp3hz-iVnQHKBU3S11JI77vqR9Bi5y8yDO0lN8twaxhWXTE5gu59tG3hRMyNtfMxLmLhyBTz73iKxbA

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

✅ Script saved: 7fqrWc5FHAbyE9j2cMuALDnTTTWhR9LwgzM0L-7YOEF2Tb1So_Exzeo9BMot0KS_cd3OKZB002DUDzuDN63dGg
📊 Found 10 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: S5PfUQ96popHJfmskFo1WcD880JbLpXr3B6SuiDtWGunxQeih25joaINXAjDlPfW2VBRC_cnTt4ot3uxRG2upg
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: 1JgZ64W9MEx7wZ55gYaJldrnyJ7Fc-lDwVQ3kfMY_tyFCkRf73eokwntw8NFyd66D_q9StXssHns2w7Eok5r_A
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"avg_score":50,"max_score":90,"count":10,"status":"inactive"}
   {"max_score":100,"status":"active","avg_score":60,"count":10}

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
Created Alice: $1000 - ID: VrjfAgydCMj4rjD483SA68eSLoNDlq2Ypp6PhJ9Ok5sDMzWOxrdCZpwztTic-_sd46bxeVNQVE4f1-sChkgfjw
Created Bob: $500 - ID: 8011TdrnFoaJbl5FXtpi4LS8iVV1zVeDyKtIqzwGYowRCZ16Ei7FcVUH16TkyEjRNdErTMKuCq1WPnqLsyzBJQ

=== Example 1: Begin Transaction ===
Transaction ID: a50341eb-28ab-4bb9-95a9-314a9761ccbb

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
New transaction: 43239e5a-2eaa-43a2-af74-d3e463ee0aa7
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
   ✅ Script saved: 9t55zn8m9kWsu5tUPxOvdzFCo37wZH6SwA1PDZBU7iPHH8iQhD5Iz48qTHjvVsFUwUOrKh2LvroOGoY9sJ5f4w

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
   ✅ Script saved: 6oIxbn1gAq8C5n1SirR2_-FYGbK4HE5vb2P6Lh7MWbxkAATSKc6Bj0tpLdUDmc486tunNkPXttkPuBYM3-Aotg

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
   ✅ Script saved: Ao7iiwplUq33L1yb-oK7cnaAF56yDbkOBzYr1_5WhT-wEVOTSBKonutwo1Qlxll_zA_liPvAJeXXolgSKfjKxw

2️⃣ Calling Script (Query + Update Credits + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {"value":0,"type":"Integer"}
   📋 Status: {"value":"active","type":"String"}
   📋 Name: {"value":"Alice Smith","type":"String"}

============================================================
📝 Script 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: jENZk0RI7LrKwz8mQmPMlfRMgoSV1y8gM0xxe4WPzxQAlbv9312f39lWTiwxl9nZTmne8DfCKibNjYEZph7jbw

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: 9t55zn8m9kWsu5tUPxOv...
   ✅ Deleted script: 6oIxbn1gAq8C5n1SirR2...
   ✅ Deleted script: Ao7iiwplUq33L1yb-oK7...
   ✅ Deleted script: jENZk0RI7LrKwz8mQmPM...
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

> @ekodb/ekodb-client@0.6.0 prepare
> npm run build


> @ekodb/ekodb-client@0.6.0 build
> tsc


up to date, audited 7 packages in 819ms

found 0 vulnerabilities

> @ekodb/ekodb-client@0.6.0 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning JavaScript client library examples...[0m

added 1 package, removed 1 package, and audited 9 packages in 633ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'pvo4hInaloYTmB19a50JUJ0_F_0W7ghPiuwHQN_jx-p3DGdeaJ_Q6WvYxPc2zzCyfrrZVdEKHEP-s_Wc4zwEkQ'
}

=== Find by ID ===
Found: {
  id: 'pvo4hInaloYTmB19a50JUJ0_F_0W7ghPiuwHQN_jx-p3DGdeaJ_Q6WvYxPc2zzCyfrrZVdEKHEP-s_Wc4zwEkQ',
  name: { value: 'Test Record', type: 'String' },
  active: { value: true, type: 'Boolean' },
  value: { type: 'Integer', value: 42 }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  name: { type: 'String', value: 'Updated Record' },
  id: 'pvo4hInaloYTmB19a50JUJ0_F_0W7ghPiuwHQN_jx-p3DGdeaJ_Q6WvYxPc2zzCyfrrZVdEKHEP-s_Wc4zwEkQ',
  value: { value: 100, type: 'Integer' },
  active: { value: true, type: 'Boolean' }
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
cache:product:2: { name: 'Product 2', price: 39.989999999999995 }
cache:product:3: { name: 'Product 3', price: 49.989999999999995 }

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: 8HBhRD65dWwLZgCIeYhlScCNDFYFswGXtBT_ry41oeMEoAvJO3EVXQ3APTA0uBW2PUycl5KUbZD7HCxPvx8_Lw

=== List Collections ===
Total collections: 22
Sample collections: schema_products_client_ts,schema_users_client_ts,chat_messages__ek0_testing,schema_employees_client_py,test_accounts

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: Zdm_UUuiX9YG4SUPaRsWvfXSJLczO253vxgi0RG1QbL78Cl6lP_dlacE2Sx3Ci9_AKIcQk_tWCdztdNt2BfJew

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: VyGcGQEBUpdW8RaRhM0rAdMJcQbWiXY1sx9OEPNg-qEV1AjjmCUJoj_uL5OUtXcgPZxzndpaWMW7IQajsTXbjQ

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
✓ Inserted test record: BXXsPCuPa5c9ElENbPlOjuTmhKXqVIbSDbJvkUa1Jd0XyNeoqaMw52vBdi9ViME_m4Q0V-Ltyuc3DbsKDAxhqA

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
✓ Inserted document with TTL: 90BtkH2TmU5OZdwpXheTp6eIJdO3ojbKweHKS1rJc2lBwElMla6BzFFsA97yG13MSDEYzENCoFAgVRDIEQLXFw

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

✅ Script saved: flpW2s51P48Ju8P3I18I1xR4rFmUPhGjs-ZIHnaIxzVo9F7P4KnF50tkUQlBhVY61J8GXeF0EQ4qQBuRDnpfXA
📊 Found 10 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: wjU6DuRTarS1jSmY_WIDSsaqnSBR8LeXz6vqjMKr6qzSqn9FOXS9_T3-wHHQpVWOuJO30OiE-2H2sgaj5Yt0AQ
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: p2OI63vgWcwbJufxylDEV5xPg-oNKgwoflvrmB0E2szEtnobSRmONqcHWrbBlBTBFq6_OJwu8c1dOVoibLoglQ
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"max_score":90,"avg_score":50,"status":"inactive","count":5}
   {"status":"active","max_score":100,"avg_score":60,"count":5}

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

✅ Script saved: QaQMGMSrTxAjYQV3mef3S7XGu2NY-McEC0IgjTXqakt70JW-U074JHGgZHFdbNxrVggFWN3wFJo-ITFR31K6Rw
📊 Found 20 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved
📊 Found 20 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved
📊 Statistics: 2 groups
   {"status":"inactive","avg_score":50,"count":10}
   {"status":"active","avg_score":60,"count":10}
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
   3. Monitor 27" - $399 (⭐4.6)
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
   admin: 20.0 (3 users)
   user: 70.0 (7 users)
⏱️  Execution time: 0ms

📝 Example 4: Top Users by Score

✅ Script saved
📊 Users (showing first 5 of 0):
⏱️  Execution time: 0ms

📝 Example 5: User Summary Statistics

✅ Script saved
📊 User summary (2 groups):
   active undefineds: 7 users, Total Score: 370
   inactive undefineds: 3 users, Total Score: 180
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
   3. Natural Language Processing (AI)
   4. Getting Started with ekoDB (Database)
   5. Introduction to Machine Learning (AI)
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
   3. Natural Language Processing
   4. Getting Started with ekoDB
   5. Introduction to Machine Learning
⏱️  Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Script saved
📊 Projected documents (showing first 3):
   1. Vector Databases Explained
   2. Database Design Principles
   3. Natural Language Processing
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
🚀 ekoDB AI Scripts Example

⚠️  Note: These examples require AI API credentials (OpenAI, etc.)

📋 Setting up test data...
✅ Created 3 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
🤖 AI Response:
   1. Efficient Data Management: Vector databases allow for efficient storage, management, and querying of spatial data.

2. High Precision: They offer a high level of detail and accuracy as they use points, lines, and polygons to represent spatial data.

3. Scalability: Vector databases are highly scalable, allowing for the addition of more data without significant degradation in performance.

4. Geographic Analysis: They support complex geographic operations like proximity analysis, network analysis, and spatial joins.

5. Topological Information: Vector databases maintain topological information which is useful for understanding spatial relationships between features.

6. Visualization: They provide better visualization capabilities as the data is represented in its original form.
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
✅ [32mJavaScript client examples complete![0m
