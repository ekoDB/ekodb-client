make test-examples-javascript
🧪 [36mRunning JavaScript examples (direct HTTP/WebSocket)...[0m

added 1 package, removed 1 package, and audited 9 packages in 640ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
[34m
╔════════════════════════════════════════╗[0m
[34m║  ekoDB JavaScript Examples Test Suite ║[0m
[34m╚════════════════════════════════════════╝[0m
[34m
=== Checking Server Connection ===[0m
(node:75641) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
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
(node:75644) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
Inserted: {
  id: 'HExkc71eokHYIpompSBj1e6mkszJI5d4SjOJ2D-stwBm3GTFV6Lb5pJCT_v5dtbN-ecFHgcqHv54UvHeFY-BQA'
}

=== Find by ID ===
Found: {
  active: { value: true, type: 'Boolean' },
  name: { type: 'String', value: 'Test Record' },
  value: { type: 'Integer', value: 42 },
  id: 'HExkc71eokHYIpompSBj1e6mkszJI5d4SjOJ2D-stwBm3GTFV6Lb5pJCT_v5dtbN-ecFHgcqHv54UvHeFY-BQA'
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  active: { value: true, type: 'Boolean' },
  name: { type: 'String', value: 'Updated Record' },
  id: 'HExkc71eokHYIpompSBj1e6mkszJI5d4SjOJ2D-stwBm3GTFV6Lb5pJCT_v5dtbN-ecFHgcqHv54UvHeFY-BQA',
  value: { type: 'Integer', value: 100 }
}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
[32m✓ simple_crud.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl-caching/simple_websocket.js ===[0m
✓ Authentication successful

=== Inserting Test Data ===
(node:75657) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
✓ Inserted test record: DPEkqZIh-i23QxrxTy9w5u3GHzN4R4xthmy2IGmxlMQOc7Ui8r2egTNy5jwHONuAeiAishu1GBMezwJqfwWegQ

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
        "id": "IclmydnNxR1P-E0R7bBIuBKTF1Ka65Q1s32FLDl28hTOjoHAqNCeuCWCZsfDHQ4J5KOp9vNZEg0ayYtaSPXtQA",
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
        "id": "EHRclmzT3prR90JJIQl3JV0OnzQ_zdm5GHUZwl1JG0Dv33EQi6Elici_xRW6FSSDrWR4Yjl4f0a7s25WdNchNw",
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
        "id": "DPEkqZIh-i23QxrxTy9w5u3GHzN4R4xthmy2IGmxlMQOc7Ui8r2egTNy5jwHONuAeiAishu1GBMezwJqfwWegQ",
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
        "id": "fGqA_U1JCaXbYlEiFASIABYGZ-Ckhe9tGhJXqSIoaSMrfQnRLfvqHABUQDbp2Vb2nk6WYTT1jQOIKE8UhS3SGQ",
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
(node:75660) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
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
(node:75674) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
✓ Set key: session:user123

=== KV Get ===
Retrieved value: {
  loginTime: '2025-12-31T16:20:54.765Z',
  userId: 123,
  username: 'john_doe'
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
(node:75677) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
Collection created with first record: u5FEveCERNtg9a4Dv5rOA1H9V_2HlzVVD8p-fVJ_wqLlNqNORqPlNFca6zuwen44sx-PVzTE9qXeslgSCTI10A

=== List Collections ===
Total collections: 23
Sample collections: [
  'schema_documents_client_go',
  'websocket_test',
  'ttl_cache',
  'schema_employees_client_py',
  'schema_users_client_ts'
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
(node:75680) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
✓ Inserted document: retYL8l1z7i4GWviJXvLONp-thw6XxRj3TsjIdCQADrF418OntsRn631amABxHHYMliQ7L2jsuVCTGuW_zUznQ

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
(node:75693) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
✓ Inserted document with TTL: pPJxHIus6ZtR9_5hRjPWgyf9zwwcknEc4bM2V5Mase92CpxoJ9ICzEvGmjuPS47Bke6LAqngjIvQxxF9lISm1Q

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

✅ Script saved: OFuMWZq8Zoa7mWig38AowNIcs17nGXYqcb1IRSUuiD4fJLEQ_rbvXhSz9YT0DD5A-vQERfuYKhjVByapxKevqg
📊 Found 20 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: 5eS2EGqH7JgBg57FB1gd_o8qIVI5s_hnBWAiE3bIi9INafELjytyNcDER1vYJaK2AXvCf7AMcEQPW9zBLuY2fA
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: sPcBdbgaOTwRmuvR9NUN8UKuEY-dZBiYIzggd8ZUmXfIYJ_gRQ5V1wBMGhCNTwnKwx9nI05KNj0MeKo-_agACw
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"count":10,"avg_score":50,"max_score":90,"status":"inactive"}
   {"max_score":100,"avg_score":60,"status":"active","count":10}

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
Created Alice: $1000 - ID: M3gxo-A4kX3hYfkpSZYQZXKPCQIZAGQ6e2V32lVQz_OKzBiTJLUODbYvpdVlAJFTFGMrp0gLgQPabbi-GWb4lA
Created Bob: $500 - ID: Pe50VxMvVmQszvJ7jcNVqOk2PhGua1OrBM8Z-vO0eybTccUoCu-k_1CaYWfnGmD_F-HKE5bGkHUDxzyx0T5qLQ

=== Example 1: Begin Transaction ===
Transaction ID: 133c1327-856b-4107-a22c-78cc3afe258a

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
New transaction: bd7280a2-a17b-47af-984c-30e1909b725c
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
   ✅ Script saved: fvUS4vRJID5roMA8N5fvIIR8Cd4cre-gyeoWzPnv5TsiuuynmxLlBRn--npKyjFUXJu9dlB6tjuKSV7N0LmIRg

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
   ✅ Script saved: eqgNPlJYGPgCFhFO6_REsG8MG4eC_bx5eiKNvWP43Xr8hpb7ENfdYjzkxx4e_qIUA3hPXACO1izJ1eE--yqTyw

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
   ✅ Script saved: AfFwn6tJcsj68aw-Dw1yP_VoYBw_ZebZ4YaTesP-J3Ijw8V_BSrgVQt7PCaUxkFDTO86lumstcNAd6PouvIlGQ

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
   ✅ Script saved: Pkzastp935yfHC-kRQEcgoNHhGksx5xkNPOQ8kiMdhwspyYE85TqVioPRURv3JrIHyTVrduKQY2bV2m5lqgcPQ

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: fvUS4vRJID5roMA8N5fv...
   ✅ Deleted script: eqgNPlJYGPgCFhFO6_RE...
   ✅ Deleted script: AfFwn6tJcsj68aw-Dw1y...
   ✅ Deleted script: Pkzastp935yfHC-kRQEc...
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

> @ekodb/ekodb-client@0.5.0 prepare
> npm run build


> @ekodb/ekodb-client@0.5.0 build
> tsc


up to date, audited 7 packages in 798ms

found 0 vulnerabilities

> @ekodb/ekodb-client@0.5.0 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning JavaScript client library examples...[0m

added 1 package, removed 1 package, and audited 9 packages in 624ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'IcvqG-d4OGn_0p0LHfDp9t827lKUsKv9OGB1U-9MRrA3fof7KimyTTvjgGWpdcZ0S8lM83gf7SyQ4i8w0WnrFA'
}

=== Find by ID ===
Found: {
  value: { value: 42, type: 'Integer' },
  active: { type: 'Boolean', value: true },
  name: { type: 'String', value: 'Test Record' },
  id: 'IcvqG-d4OGn_0p0LHfDp9t827lKUsKv9OGB1U-9MRrA3fof7KimyTTvjgGWpdcZ0S8lM83gf7SyQ4i8w0WnrFA'
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  name: { type: 'String', value: 'Updated Record' },
  value: { value: 100, type: 'Integer' },
  id: 'IcvqG-d4OGn_0p0LHfDp9t827lKUsKv9OGB1U-9MRrA3fof7KimyTTvjgGWpdcZ0S8lM83gf7SyQ4i8w0WnrFA',
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
Retrieved value: { userId: 123, username: 'john_doe' }

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
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: 6Dm0FDIChSdIo1w7LTKnLBbN7oKm-l8AFwPGXyZSHSRxWltmdHW_vdkzcalCU5zAEo0-XKAkhwEdxIBmsSUIQg

=== List Collections ===
Total collections: 22
Sample collections: schema_documents_client_go,websocket_test,ttl_cache,schema_employees_client_py,schema_users_client_ts

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: FFBL9K9IFSYpSRFCzvXW1wWAF5w41Wc5B4SK9EuCTTM2DS9PVH1-REG8NFoPsJALmIDrOwH1VpUcurY-fQebQg

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: SBhFCs-muIASj8Q0wOe_cCTCmY1rs47-cNvzObXRuIPg3P5mAbPBqf-MgQQHWcjfDeJ-LO2-clxmdNQVILe-sg

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
✓ Inserted test record: PqgiUDC5XrejtedsiPNOn2PrX8NQ_Qz-uSfNCdvznHBe74Iij845NbRwQayZl3IROMkUn39tUYnzEwKhFqY4Zg

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
✓ Inserted document with TTL: 0ZQwQvshN1itz-CVzHv07JTt11GzxVJDwwRdceEI7JeuSC1biPfgGD_2VmV50ydwNTErIB2qcx9gp0paaA7Ryw

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

✅ Script saved: V2OmV7Y_UUPB233XnSHN37rXUfkcjsc3kogJmtGB_CHU_9C1wbcP1oZXZx_Q0jwBXS4wOUFPsXsKkfH7FAUvPg
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: MJ31QwISP4VeaOzkR49ExNpSOaRPKzrttPTEu3e-1OW1On1UCuRItSLnQ021eo_7SIEmehp-GBy82j1dZPJl3A
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: u4EMnO60ImpgKXIDiokHH8I0bppzozgMvmHihWT1xutLf-wKnYqsKpkgR4OS5Egh4vDE_AsA2qF6jnl3EyIEHQ
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"max_score":90,"count":5,"status":"inactive","avg_score":50}
   {"avg_score":60,"count":5,"status":"active","max_score":100}

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

✅ Script saved: N1N7I4mWd-lqpCAq7ObrRZlHkzwkRwC7HaMN7jdnWc3rs7Rqy7h0I5vEHRs64YwZq3ZAEokyup4H1c24UEBmdw
📊 Found 20 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved
📊 Found 20 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved
📊 Statistics: 2 groups
   {"status":"active","count":10,"avg_score":60}
   {"count":10,"status":"inactive","avg_score":50}
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
   1. Wireless Mouse - $29 (⭐4.5)
   2. Monitor 27" - $399 (⭐4.6)
   3. Office Lamp - $79 (⭐4.05)
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
📊 Users (showing first 5 of 10):
   1. User 4 (user4@example.com) - Score: 40
   2. User 7 (user7@example.com) - Score: 70
   3. User 9 (user9@example.com) - Score: 90
   4. User 5 (user5@example.com) - Score: 50
   5. User 2 (user2@example.com) - Score: 20
⏱️  Execution time: 0ms

📝 Example 5: User Summary Statistics

✅ Script saved
📊 User summary (2 groups):
   inactive undefineds: 3 users, Total Score: 180
   active undefineds: 7 users, Total Score: 370
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
   3. Natural Language Processing (AI)
   4. Database Design Principles (Database)
   5. Vector Databases Explained (Database)
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
   3. Natural Language Processing
   4. Database Design Principles
   5. Vector Databases Explained
⏱️  Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Script saved
📊 Projected documents (showing first 3):
   1. Getting Started with ekoDB
   2. Introduction to Machine Learning
   3. Natural Language Processing
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
🚀 ekoDB AI Scripts Example

⚠️  Note: These examples require AI API credentials (OpenAI, etc.)

📋 Setting up test data...
✅ Created 3 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
🤖 AI Response:
   1. Scalability: Vector databases can handle large amounts of data and are highly scalable.
2. High Precision: They support a high level of detail and precision, with no loss of quality when zooming in or out.
3. Flexibility: These databases can handle various types of data, including points, lines, and polygons.
4. Fast Processing: Vector databases allow for quick data processing and analysis.
5. Better Visualization: They provide better visualization options due to their ability to represent data in a graphical manner.
6. Efficient Storage: Vector databases use less disk space compared to raster databases.
7. Easy to Update: Updating data in a vector database is relatively easy since it involves changing the attributes of the vector features.
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
✅ [32mJavaScript client examples complete![0m
