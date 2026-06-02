make test-examples-javascript
🧪 [36mRunning JavaScript examples (direct HTTP/WebSocket)...[0m

added 1 package, removed 1 package, and audited 9 packages in 1s

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
  id: 'MfJtnxloG9BgL7ILLHZ4z-5syAJZ2Sjn-T_0atkfiO-DAXyLISAia3nDQRG5gEz2k9snmvHW2tBKEiiwRYwHRQ'
}

=== Find by ID ===
Found: {
  name: { type: 'String', value: 'Test Record' },
  active: { value: true, type: 'Boolean' },
  value: { type: 'Integer', value: 42 },
  id: 'MfJtnxloG9BgL7ILLHZ4z-5syAJZ2Sjn-T_0atkfiO-DAXyLISAia3nDQRG5gEz2k9snmvHW2tBKEiiwRYwHRQ'
}

=== Find with Query ===
Found documents: [
  {
    name: { value: 'Test Record', type: 'String' },
    active: { type: 'Boolean', value: true },
    value: { type: 'Integer', value: 42 },
    id: 'MfJtnxloG9BgL7ILLHZ4z-5syAJZ2Sjn-T_0atkfiO-DAXyLISAia3nDQRG5gEz2k9snmvHW2tBKEiiwRYwHRQ'
  }
]

=== Update Document ===
Updated: {
  id: 'MfJtnxloG9BgL7ILLHZ4z-5syAJZ2Sjn-T_0atkfiO-DAXyLISAia3nDQRG5gEz2k9snmvHW2tBKEiiwRYwHRQ',
  active: { type: 'Boolean', value: true },
  value: { value: 100, type: 'Integer' },
  name: { value: 'Updated Record', type: 'String' }
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
✓ Inserted test record: orMNFbBzlFj0vffAEk-3mFplW4F6x4EaDwla_sGGQDHRlDWw4Bl1VXH2qoEBMYtBz2nHX20y4Plqc0C3Zde47w

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "type": "Success",
  "payload": {
    "data": [
      {
        "active": {
          "value": true,
          "type": "Boolean"
        },
        "id": "LVjLU7O4WpAfaWLnqrOmzILJPQ3TqIK_XWK7T8feWt5HbBezY_3n6h9dqeSsEcO0pgkT_XVhBjhE3wUBYDM89Q",
        "value": {
          "type": "Integer",
          "value": 42
        },
        "name": {
          "type": "String",
          "value": "WebSocket Test Record"
        }
      },
      {
        "value": {
          "type": "Integer",
          "value": 42
        },
        "id": "orMNFbBzlFj0vffAEk-3mFplW4F6x4EaDwla_sGGQDHRlDWw4Bl1VXH2qoEBMYtBz2nHX20y4Plqc0C3Zde47w",
        "name": {
          "type": "String",
          "value": "WebSocket Test Record"
        },
        "active": {
          "type": "Boolean",
          "value": true
        }
      },
      {
        "name": {
          "type": "String",
          "value": "WebSocket Test Record"
        },
        "id": "7g7GepM7rMRdPcUBq2rwkPkoy1xolwnNi1rkxiMi2YyWQr8s7yCEracFiulZIE-LkC1_yQwcApta35Ak2yOqiQ",
        "active": {
          "value": true,
          "type": "Boolean"
        },
        "value": {
          "value": 42,
          "type": "Integer"
        }
      },
      {
        "name": {
          "value": "WebSocket Test Record",
          "type": "String"
        },
        "active": {
          "value": true,
          "type": "Boolean"
        },
        "id": "ETn7_PXYcMi28a-eb7yPoEHGCRSGklpBAXdJtUrQWqWFUy-4pSydMUe8qXI-22FIoV0sZtL5U57BuwAiOipd6A",
        "value": {
          "type": "Integer",
          "value": 42
        }
      }
    ]
  },
  "messageId": "1780378736835"
}
✓ Retrieved 4 record(s) via WebSocket

✓ WebSocket example completed successfully
WebSocket closed
[32m✓ simple_websocket.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/http_functions.js ===[0m
🚀 ekoDB Functions Example (JavaScript/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Function saved: rfhoTvW4uzfF-tq_zIdljagwwlnxwQakySZ4D3y_rJ8uyz2axdxg9ozNVU55QLF8Gqkhmlf493FbfsvAc-g6dA
📊 Found 10 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Function saved: je217arsEhKYSMcfnNzTLTdkf2Vqh5bUn3N-vRDasaEMYSqULE9Mm9PIhY_80VR8cc2Lawbuput7T6_tlJP0xQ
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: l4mQDctqjayybNgtW35gMZEqf3sPxfpHs20eKXcnceCnqUP8-FNFV03zMwFO2a8RjOhyXBHwapfPVvR0za8jlA
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"status":{"type":"String","value":"active"},"count":{"value":10,"type":"Integer"},"avg_score":{"value":60,"type":"Float"},"max_score":{"value":100,"type":"Integer"}}
   {"count":{"type":"Integer","value":10},"max_score":{"type":"Integer","value":90},"avg_score":{"type":"Float","value":50},"status":{"value":"inactive","type":"String"}}

📝 Example 4: Function Management

📋 Total functions: 16
🔍 Retrieved function: Get Active Users
✏️  Function updated
🗑️  Function deleted

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
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/collection_management.js ===[0m
=== Collection Management (Direct HTTP) ===

✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: hd7wJ88x9bcbgTkJpJHpSjKbBPjAFFfPr-deFLBq6AktXlJRVImhUisGITGwIsg485iMhs3PdnZfnf7Eq-w2uQ

=== List Collections ===
Total collections: 26
Sample collections: [
  'schema_employees_client_ts',
  'schema_users_client_ts',
  'schema_documents_client_ts',
  'schema_employees_client_go',
  'functions__ek0_testing'
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
Created Alice: $1000 - ID: FF2KDCJWrOujYE37chWpKgS1nypsy7RjTvMxkjL37qaLHs-EVFAQ67nP_pP9ruVYPhEHqOVncViZQuPBr6sVkw
Created Bob: $500 - ID: loj_rFV74IDY659C4w0uUkIGGkrOpW4ApP8KU9gfHRbFF4Cpf1AflI8s8J-bShO3Fs1F4lh9kzKPIZvbLV9Q7Q

=== Example 1: Begin Transaction ===
Transaction ID: e526a330-4ef0-440a-9e8b-54245fc77125

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: {"type":"Integer","value":800}
Bob: {"value":700,"type":"Integer"}

=== Example 5: Rollback ===
New transaction: 318c47f9-0a43-4a6d-959e-ef3c481f809e
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: {"type":"Integer","value":700}

=== Cleanup ===
✓ Deleted test accounts

✓ All transaction examples completed
[32m✓ transactions.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/crud_functions.js ===[0m
🚀 ekoDB Complete CRUD Functions Example
============================================================
Demonstrates:
  • Insert + Verify (using Query)
  • Query + Update Status + Verify
  • Query + Update Credits + Verify
  • Query Before Delete + Delete + Verify Gone

Each function shows Functions chaining with proper verification
============================================================

============================================================
📝 Function 1: Insert + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: uJAiWsic-ImRkksLipGNnHbf3XrFpdc8u79tYoqcmaYttObEKESOVW-ZFNn4jyL3so3ZgajxuaNypPiGCVg35g

2️⃣ Calling Function (Insert + Verify)...
   ✅ Function executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Name: [object Object]
   📋 Email: {"type":"String","value":"alice@example.com"}
   📋 Status: [object Object]
   📋 Credits: [object Object]

============================================================
📝 Function 2: Query + Update + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: YVr05_oFBscl0AMuwsoCG96Ft91GqVxpR9vakc5cB67u14Gt-XmYP8gh7zM6iu2e_Dguqsfd9BEO8kzXTcXBIQ

2️⃣ Calling Function (Query + Update + Verify)...
   ✅ Function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Status updated to: {"value":"active","type":"String"}
   📋 Name: {"type":"String","value":"Alice Smith"}

============================================================
📝 Function 3: Query + Update Credits + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: tAw7Qa7Njsc8YXhlw1e8ERI6uiMnmbM1qPVqkPxw-dAWoMsnoIMX91zJNhCxecVkxux_neCwkewBM6SjfmNQdg

2️⃣ Calling Function (Query + Update Credits + Verify)...
   ✅ Function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {"value":0,"type":"Integer"}
   📋 Status: {"type":"String","value":"active"}
   📋 Name: {"value":"Alice Smith","type":"String"}

============================================================
📝 Function 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: xh0bEDtUExaXkNqyuGLPTUz9E3ReBi8wFf3ulzSHNgpOUXXNJzHDvxDp5WPTyk46rZo4v1cywLoSbYa7f0ikNQ

2️⃣ Calling Function (Query + Delete + Verify)...
   ✅ Function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted function: uJAiWsic-ImRkksLipGN...
   ✅ Deleted function: YVr05_oFBscl0AMuwsoC...
   ✅ Deleted function: tAw7Qa7Njsc8YXhlw1e8...
   ✅ Deleted function: xh0bEDtUExaXkNqyuGLP...
   ✅ Deleted collection: users

============================================================
✅ Complete CRUD Functions Example Finished!
============================================================

💡 Key Takeaways:
   ✅ Functions chain Functions together
   ✅ Each function demonstrates operation + verification
   ✅ Parameters make functions reusable
   ✅ Verification is built into the function itself
   ✅ Complete CRUD lifecycle in 4 focused functions
[32m✓ crud_functions.js completed successfully[0m
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
  Output: Document ID = yfuWE8b4DAbGtGrk3tNyZzTc4hnjibJrLkpzA2GxdwkWjCbvZ9O5EIzboxhwv6kg8mimSgQHSD6CMpsRYVh0cw
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: findById(yfuWE8b4DAbGtGrk3tNyZzTc4hnjibJrLkpzA2GxdwkWjCbvZ9O5EIzboxhwv6kg8mimSgQHSD6CMpsRYVh0cw)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: findById(yfuWE8b4DAbGtGrk3tNyZzTc4hnjibJrLkpzA2GxdwkWjCbvZ9O5EIzboxhwv6kg8mimSgQHSD6CMpsRYVh0cw)
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
  Output: Document ID = fINKPBe89hzQvqhfydPkitp84eXHG8IC2GpsVj-_CDX8qRXJNWOSmnwn5-gsvz9_sfO_FlwsyanhG4nT3i_YaA
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: findById(fINKPBe89hzQvqhfydPkitp84eXHG8IC2GpsVj-_CDX8qRXJNWOSmnwn5-gsvz9_sfO_FlwsyanhG4nT3i_YaA)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: findById(fINKPBe89hzQvqhfydPkitp84eXHG8IC2GpsVj-_CDX8qRXJNWOSmnwn5-gsvz9_sfO_FlwsyanhG4nT3i_YaA)
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

> @ekodb/ekodb-client@0.18.2 prepare
> npm run build


> @ekodb/ekodb-client@0.18.2 build
> tsc


up to date, audited 52 packages in 1s

17 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.18.2 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning JavaScript client library examples...[0m

added 1 package, removed 1 package, and audited 9 packages in 1s

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'wSysVKYY05_KifyNTFgJn_OVr8pd6rvXe9CUK4yf1cSYCsiXoVwmxbh09k1Ms8I9gNTW8KUizqBmDrULWaIhRQ'
}

=== Find by ID ===
Found: {
  id: 'wSysVKYY05_KifyNTFgJn_OVr8pd6rvXe9CUK4yf1cSYCsiXoVwmxbh09k1Ms8I9gNTW8KUizqBmDrULWaIhRQ',
  value: { value: 42, type: 'Integer' },
  active: { type: 'Boolean', value: true },
  name: { value: 'Test Record', type: 'String' }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  value: { value: 100, type: 'Integer' },
  id: 'wSysVKYY05_KifyNTFgJn_OVr8pd6rvXe9CUK4yf1cSYCsiXoVwmxbh09k1Ms8I9gNTW8KUizqBmDrULWaIhRQ',
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
✓ Inserted test record: EJOB4kTNQZpw9QWwfrOmdlMPykiRktKPHEDud2MJ-5sbtwQW_8W2NOUI1Hl-HEymA5IQ5s015AO2Kmjk-tYvZw

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
Collection created with first record: OWZY6CecNL2onhEAO-BzehjHE9duLm21FxX0wMIHhYrUFDmAi4kzKZjZuuqw8NTQdJlnZbB32nLC1kFqLDoqBQ

=== List Collections ===
Total collections: 25
Sample collections: schema_employees_client_ts,schema_users_client_ts,schema_documents_client_ts,schema_employees_client_go,functions__ek0_testing

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
cache:product:1: { price: 29.99, name: 'Product 1' }
cache:product:2: { name: 'Product 2', price: 39.989999999999995 }
cache:product:3: { price: 49.989999999999995, name: 'Product 3' }

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 0 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 15

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: false

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: fVpdYfnXsJQVaGdRT3mtdien8zHiydgA_7DC_ANP7JJyNzBI4SbJoW-OORPi6RNbQYqRs68jAxmueC6yjE9WSw
Created Bob: $500 - ID: Co0nYOKSD5d1VbaBAKg4cZSWZeq08kq4OPK5Es-Ca9zdjamUu2I2ATR894ImnS87L8s5KUU9D8zK6rcq5Ph97A

=== Example 1: Begin Transaction ===
Transaction ID: 8c470ebf-f894-4d19-9b99-290f545f24a5

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: de31c900-3e79-41f7-97b7-63becf825fcb
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
  1. Score: 26.400, Matched: bio.value, title, title.value, bio
  2. Score: 26.400, Matched: title, title.value, bio, bio.value
  3. Score: 26.400, Matched: bio.value, title.value, bio, title
  4. Score: 26.400, Matched: bio.value, title.value, title, bio

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio.value, bio, title, title.value
  2. Score: 39.600, Matched: title.value, bio.value, bio, title
  3. Score: 39.600, Matched: bio.value, title, title.value, bio
  4. Score: 39.600, Matched: bio, title, title.value, bio.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.759, Matched: 
  2. Score: 0.746, Matched: 
  3. Score: 0.741, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.703, Matched: content, title.value, title, content.value
  2. Score: 1.498, Matched: title.value, content, content.value, title
  3. Score: 0.297, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: skills, skills.value, bio, bio.value

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
✓ Inserted document: nNK27_gtokn--lVRizX96JdEbq4d7GJ9QAPB8EcwJkijqL4IxX1euoX8fgU7TM40IrsNdaOs5q6GPiOFh2wr-A

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: gn1THTWC1-T6ufVY3cXKgFrpBlGXEnmcoO_dg46St5qAYZqfQTn0uAGDA6pL_bzoW-rBxsxrhHrhQLc26__9tQ

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
✓ Inserted document with TTL: 5G8CF7eGp2kLzWgdrblmiwAScueIU7DqYVEFSUYk8G-ag4hnf4YUiLr05CznBfZ8dauEawAb91GYLxs2QVu_4g

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
✓ Edge cache script created: RdzRPl8_LC8DR_hVzZn9LJtlJkukM0LFbZhiwFMi683MCF9fHXO1fePpvvzR7QKfeq6MIvw_YG32n5tRoCpHdg

Call 1: Cache miss (fetches from API)
Response time: 52ms
Result: {
  "records": [
    {
      "value": {
        "value": {
          "elevation": 32,
          "utc_offset_seconds": 0,
          "current": {
            "temperature_2m": 10.7,
            "time": "2026-06-02T05:30",
            "interval": 900
          },
          "longitude": -73.99308,
          "generationtime_ms": 0.020503997802734375,
          "timezone_abbreviation": "GMT",
          "timezone": "GMT",
          "latitude": 40.710335,
          "current_units": {
            "temperature_2m": "°C",
            "interval": "seconds",
            "time": "iso8601"
          }
        },
        "type": "Object"
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
Response time: 43ms (1.2x faster!)
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "elevation": 32,
          "utc_offset_seconds": 0,
          "current": {
            "temperature_2m": 10.7,
            "time": "2026-06-02T05:30",
            "interval": 900
          },
          "longitude": -73.99308,
          "generationtime_ms": 0.020503997802734375,
          "timezone_abbreviation": "GMT",
          "timezone": "GMT",
          "latitude": 40.710335,
          "current_units": {
            "temperature_2m": "°C",
            "interval": "seconds",
            "time": "iso8601"
          }
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

=== The Magic ===
- Your DATABASE is your EDGE
- No Redis needed
- No CDN needed
- No cache invalidation logic needed (TTL handles it)
- With ripples: All nodes auto-sync cache
- One service: Database + Cache + Edge Functions

✓ Example complete!

🚀 ekoDB Functions Example (JavaScript Client)

✅ Client initialized (token exchange automatic)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

ℹ️  Function 'get_active_users' already existed — updated instead
✅ Function saved: tIdjL1weC6ktBc5W9Saw6H6a8Zf8rx5AZvvTXRxv6lWzqvmb7YMaZnk1BFgB6z6xjn9Z_gjkEtKV64hQ4BWntA
📊 Found 10 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Function

ℹ️  Function 'get_users_by_status' already existed — updated instead
✅ Function saved
📊 Found 10 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Function

✅ Function saved
📊 Statistics: 2 groups
   {"avg_score":{"type":"Float","value":60},"status":{"value":"active","type":"String"},"count":{"value":5,"type":"Integer"}}
   {"count":{"type":"Integer","value":5},"avg_score":{"type":"Float","value":50},"status":{"value":"inactive","type":"String"}}
⏱️  Execution time: 0ms

📝 Example 4: Function Management

📋 Total functions: 17
🔍 Retrieved function: Get Active Users
✏️  Function updated
🗑️  Function deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

📝 Example 5: Multi-Stage Pipeline

✅ Multi-stage function saved
📊 Pipeline executed 2 stages
⏱️  Total execution time: 0ms
📈 Stage breakdown:

📝 Example 6: Count Users

✅ Count function saved
📊 Total user count: 10
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Deleted collection
✅ Deleted test functions

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

ℹ️  Function 'fetch_user' already existed — updated instead
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

ℹ️  Function 'fetch_and_store_user' already existed — updated instead
✅ Saved reusable function: fetch_and_store_user (uses KV)
ℹ️  Function 'swr_user' already existed — updated instead
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 161ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "website": "hildegard.org",
      "username": "Bret",
      "phone": "1-770-736-8031 x56442",
      "company": {
        "name": "Romaguera-Crona",
        "catchPh...

Second call (cache hit - from cache):
   ⏱️  Duration: 43ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "website": "hildegard.org",
      "username": "Bret",
      "phone": "1-770-736-8031 x56442",
      "company": {
        "name": "Romaguera-Crona",
        "catchPh...
   🚀 Cache speedup: 3.7x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

ℹ️  Function 'validate_user' already existed — updated instead
✅ Level 1 function: validate_user
ℹ️  Function 'fetch_slim_user' already existed — updated instead
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
🚀 ekoDB JavaScript Complete Functions Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Function saved: ytbTdZM785ACJkvjBcZ5vgQ0ZIsF-b3hv2GsYbmp7GexxTaBszFgU0raiICNR4_mhP0M9MAW6HwBqrTRrM76Zw
📊 Found 2 product groups
   {"avg_price":{"type":"Float","value":474},"count":{"value":2,"type":"Integer"},"category":{"value":"Furniture","type":"String"}}
   {"avg_price":{"type":"Float","value":575.6666666666666},"category":{"value":"Electronics","type":"String"},"count":{"type":"Integer","value":3}}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   {"category":{"type":"String","value":"Electronics"},"count":{"value":3,"type":"Integer"}}
   {"category":{"type":"String","value":"Furniture"},"count":{"value":2,"type":"Integer"}}
⏱️  Execution time: 0ms

📝 Example 4: High Rating Products

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 5: Function with Parameter Definition

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 6: Multi-Stage Pipeline (FindAll → Group → Count)

✅ Function saved
📊 Pipeline executed 3 stages
⏱️  Total execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All complete function examples finished!

💡 This example demonstrates ekoDB's Function system:
   ✅ FindAll operations
   ✅ Group aggregations (Count, Average)
   ✅ Multi-stage pipelines (FindAll → Group → Count)
   ✅ Parameter definitions
   ✅ Function management (save, call, delete)
🚀 ekoDB JavaScript KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: r98dXlGewl_ne0mlFG1YFIAb7ItrNB66pa4fgXsv2RFuJFw_G9jhCXTcEmt8vBPykQHLobpBsZznobFjrNF4Eg
✅ Inserted 2 products with wrapped types

📝 Example 2: Function with Wrapped Type Parameters

✅ Function saved: sbVHhe1yDnMHsZifnLgLvU3_lurvIsK-QkIGnQij3hyQ1dgL5Zhxigny5AJeyjWPnTHnCxB0KZIApXaCysJVnA
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"userId":"user_abc","role":"admin"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: tO6UHXjoerqint0taO-IbQU5KJtQcm01olXEM3JOuRt4_NPAjwtdKJoKV9KZNZHusiKQ8R2Nh3-sXTyDFNzRBw
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Function

✅ Function saved: JwqdTWC1jcGsfGg5YOfp5Re22tiiUqrGjAG_tdNck1l6e7vofod9nMo4bdnew9cV_LLzj_qpGxAlBIjpIPv5vw
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
🚀 ekoDB CRUD Functions Example

📋 Setting up test data...
✅ Created 10 test users

📝 Example 1: List All Users

✅ Function saved
📊 Found 10 users
⏱️  Execution time: 0ms

📝 Example 2: Count Users by Status

✅ Function saved
📊 User counts by status:
   inactive: 3 users
   active: 7 users
⏱️  Execution time: 0ms

📝 Example 3: Average Score by Role

✅ Function saved
📊 Average scores by role:
   user: 70.0 (7 users)
   admin: 20.0 (3 users)
⏱️  Execution time: 0ms

📝 Example 4: Top Users by Score

✅ Function saved
📊 Users (showing first 5 of 10):
   1. User 3 - Score: 30
   2. User 9 - Score: 90
   3. User 4 - Score: 40
   4. User 8 - Score: 80
   5. User 1 - Score: 10
⏱️  Execution time: 0ms

📝 Example 5: User Summary Statistics

✅ Function saved
📊 User summary (2 groups):
   inactive users: 3 users, Total Score: 180
   active users: 7 users, Total Score: 370
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD function examples completed!
🚀 ekoDB Search Functions Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Function saved
📊 Found 5 documents
   1. Database Design Principles (Database)
   2. Introduction to Machine Learning (AI)
   3. Getting Started with ekoDB (Database)
   4. Natural Language Processing (AI)
   5. Vector Databases Explained (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   Database: 3 documents
   AI: 2 documents
⏱️  Execution time: 0ms

📝 Example 3: Select Specific Fields

✅ Function saved
📊 Document titles (5 docs):
   1. Database Design Principles
   2. Introduction to Machine Learning
   3. Getting Started with ekoDB
   4. Natural Language Processing
   5. Vector Databases Explained
⏱️  Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Function saved
📊 Projected documents (showing first 3):
   1. Database Design Principles
   2. Introduction to Machine Learning
   3. Getting Started with ekoDB
⏱️  Execution time: 0ms

📝 Example 5: All Document Fields

✅ Function saved
📊 All documents (5 total, showing first 2):
   1. Database Design Principles (Database)
   2. Introduction to Machine Learning (AI)
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search function examples completed!
🚀 ekoDB Advanced Functions Example

📋 Setting up test data...
✅ Created 10 products

📝 Example 1: List All Products

✅ Function saved
📊 Found 10 products
⏱️  Execution time: 0ms

📝 Example 2: Group Products by Category

✅ Function saved
📊 Found 2 categories
   Electronics: 6 items (avg $325.67)
   Furniture: 4 items (avg $294.00)
⏱️  Execution time: 0ms

📝 Example 3: Count All Products

✅ Function saved
📊 Total products: 10
⏱️  Execution time: 0ms

📝 Example 4: Multi-Stage Aggregation

✅ Function saved
📊 Category analysis (2 categories):
   Furniture:
      Products: 4 | Stock: 43 | Avg Rating: ⭐4.26
   Electronics:
      Products: 6 | Stock: 232 | Avg Rating: ⭐4.52

⏱️  Total execution time: 0ms
📈 Pipeline stages:

📝 Example 5: Project Specific Fields

✅ Function saved
📊 Product summaries (10 items, showing first 3):
   1. Webcam HD - $119 (⭐4.5)
   2. Desk Chair - $349 (⭐4.2)
   3. Wireless Mouse - $29 (⭐4.5)
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced function examples completed!
🚀 ekoDB AI Functions Example

⚠️  Note: These examples require AI API credentials (OpenAI, etc.)

📋 Setting up test data...
✅ Created 3 articles

📝 Example 1: Simple Chat Completion

✅ Chat function saved
🤖 AI Response:
   1. High Precision: Vector databases maintain data with high precision and accuracy.

2. Scaling: Vector data can easily be scaled up or down without losing the quality of data.

3. Complex Analysis: These databases can support complex spatial analysis operations like network analysis and proximity measurement.

4. Compatibility: Vector data can be easily converted to raster format if needed, offering flexibility in data representation.

5. Compact Storage: Vector databases require less storage space compared to raster databases for representing large areas of point, line, and polygon data. 

6. Attribute Association: Associated attribute data can be stored and accessed easily with vector databases.
  
7. Graphical Presentation: They provide better visual representation for point-based data such as locations, network-based data such as roads, and area-based data such as boundaries.
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embedding function saved
📊 Generated embeddings for 3 articles
   1. "Getting Started with ekoDB" - 0D vector
   2. "Advanced Query Patterns" - 0D vector
   3. "Draft Article" - 0D vector
⏱️  Execution time: 0ms

📝 Example 3: List All Articles

✅ Function saved
📊 Found 3 articles
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All AI examples completed!
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_api_user_js (1W7eKvx5LgX_Enz8mMHP5p73RqO5sLKAMocEh7iu6dqzla8xrnOxSYNNKCkXQNjdamtdL5Y3rcrm8lMUlNHGvA)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "value": {
          "email": "Sincere@april.biz",
          "address": {
            "street": "Kulas Light",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "city": "Gwenborough",
            "zipcode": "92998-3874",
            "suite": "Apt. 556"
          },
          "username": "Bret",
          "name": "Leanne Graham",
          "company": {
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona"
          },
          "website": "hildegard.org",
          "phone": "1-770-736-8031 x56442",
          "id": 1
        },
        "type": "Object"
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
Response time: 43ms (served from cache)
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
✓ Created native SWR script: github_user_native (Ph_vmODDuBF1tGeFZrtxQzNrFST3cr8q_qZ4tvL1vTGO0gNxkZ8YsphNrrWKaXf-fNDVvzduYBBvJqg0FaF9gA)

First call (cache miss - will fetch from GitHub API):
  Response time: 16ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 16ms
  Speedup: 1.0x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (R3kqEtAAJA7ox_RXA56rfjAuobhuIRhZaWRmsHemm7PhrwaLfU1GppVRcBasHvVHULKBGU82poSv15pUGQko7w)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (tRIhzhd_P5EZ6VfNfmTN4OGDlpuLjE4fqvWvNIVfwdJpSDwd6hatGWSnxoVd4aKOmoLtzf_qJ6QtFaJ-yaDl3Q)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (d6gbA7-jJE2T3muB6fZeRUDrpSFxifWF9Pogf47g5E8wYI0zapyd7iiqtGTc9sRHNuDqks2gaKvlQSFLADP-hQ)
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
✓ Deleted 4 test functions

✅ All examples completed!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: k_WNdSUI7BJFAnPaf7dWx_qaLs1lvZyg9saAdDRxk6IfEr8dD2ATGyaS6IzicVcDhEHqXbU7NVMf_2c8hSjozQ

=== Sending Chat Message ===
Message ID: O_EdOGumxWCx3o0t-qfDeBeTJXdDoCnVO06Xc3u6OlskQeMnYZBRjTHAZ7gNqW-1C6tk07mZAC5uocJoVNohXw

=== AI Response ===
Here are the available products along with their prices:

1. **ekoDB**
   - **Price:** $99
   - **Description:** A high-performance database product with AI capabilities.

2. **ekoDB Pro**
   - **Price:** $299
   - **Description:** Enterprise edition product with advanced features.

3. **ekoDB Cloud**
   - **Price:** $499
   - **Description:** Fully managed cloud database service product.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_js',
  record: {
    description: 'A high-performance database product with AI capabilities',
    id: 'MJAxZ7ZVNPrPUtzG5Y7erIGGeIgpdxN7CRuhNRg3b4hY34zY-rGlBT4nRQ_hrIUlT9mQvF1xLECBcqhzQiCXsw',
    price: 99,
    name: 'ekoDB'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_js',
  record: {
    name: 'ekoDB Pro',
    price: 299,
    description: 'Enterprise edition product with advanced features',
    id: 'l24c7pEm7YOq1BTrYAbTTc4RdPHII7BwDHyq14FlJv54ThALeLOqrgbapTdvPrayTV0lsKNDRehJSfqkn979Ww'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_js',
  record: {
    id: 'ZhgCSy_U2FxVd7T-hyjDVewCza5TdVmpfZVG5vwW8n-dpRYKYWkoAkG0fEqtOUHHqikgUDsHSI1zKRi_MrP8dA',
    name: 'ekoDB Cloud',
    price: 499,
    description: 'Fully managed cloud database service product'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 11096ms

=== Token Usage ===
Prompt tokens: 3468
Completion tokens: 97
Total tokens: 3565

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: Qqb28XB-tOeMjSjt4du64TG5yGS-T4lxXDsOqC2hB0YnOkVltWfrtWdbxVYlEYfhlEDcLtzoD8g961fA6aVwsg

=== Sending Initial Message ===
✓ Message sent
  Response: It seems that there are currently no products available in the database, or the filter applied did not yield any matches. However, I did retrieve the following product information:

- **Name:** ekoDB
- **Description:** High-performance database product
- **Price:** $99

If you have any other specific queries or need information on a different aspect, please let me know!

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
✓ Created second session: R31j2HsItSp8228pjeu2sw4zhK7fU-xQ4Quu4rSWjALUjADVfPbCB1DQjgpNUoY2XGxdKsyP4JQMBQVynNhlQg
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 6

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 5

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: ayKSB6VFrqyZkV3YGIg1178jK5XZxo7zXVXNJjeWLLgkRCe9GCqE_9of1wfXb9BL84RWrvcgATdCIBPbzt2dsw

=== Sending Messages ===
✓ Message 1 sent
  Response: It seems there are currently no products available in the database. The only product listed is **ekoDB** priced at $99, but it is not marked as available. If you have further questions or need information about specific products, let me know!

✓ Message 2 sent
  Response: The price of **ekoDB** is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: C879a3EZsbG8KMHkNez_J4TXCnRyWT0Pw0IObh66AHfr0TDy3JqujriZ8iLvz2xZfzeGXo6NhEyvU5vXHCx0AA
  Parent: ayKSB6VFrqyZkV3YGIg1178jK5XZxo7zXVXNJjeWLLgkRCe9GCqE_9of1wfXb9BL84RWrvcgATdCIBPbzt2dsw

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: C879a3EZsbG8KMHkNez_J4TXCnRyWT0Pw0IObh66AHfr0TDy3JqujriZ8iLvz2xZfzeGXo6NhEyvU5vXHCx0AA (Untitled)
  Session 2: ayKSB6VFrqyZkV3YGIg1178jK5XZxo7zXVXNJjeWLLgkRCe9GCqE_9of1wfXb9BL84RWrvcgATdCIBPbzt2dsw (Untitled)
  Session 3: R31j2HsItSp8228pjeu2sw4zhK7fU-xQ4Quu4rSWjALUjADVfPbCB1DQjgpNUoY2XGxdKsyP4JQMBQVynNhlQg (Untitled)
  Session 4: k_WNdSUI7BJFAnPaf7dWx_qaLs1lvZyg9saAdDRxk6IfEr8dD2ATGyaS6IzicVcDhEHqXbU7NVMf_2c8hSjozQ (Untitled)
  Session 5: 9SJ2uodiLSmRX9B1dS6VIKeZP-ewQjCBeFpy5hB9jaJ8FgRvDm2SSEMpQzb1_IvoGYTwxVd2akwcTzBgRwAAWQ (Untitled)
  Session 6: 8fgoBzxCLtyI8jVgGBfLx0KItDlPjXRI43ObK7KUT0ja_oKBOfa4zlqDl9GeiLBhJfFfEGrAs0KeiWWYM5H7xw (Untitled)
  Session 7: r4ubJGUjtopC7V5Py4fIskSHrpaSwaOBrZ5nGYD4d85jDxkOJKqEbRfP5SvVMzPyR-zvAUEjFf9bRAR7_l57CQ (Untitled)
  Session 8: lbPGeYwsMllirnTdJuGiEWYuSx9gchpzI8LRbqpgel13B-GMO9_LF-S05jwSHiejQSASop5o6mUC1c_2BaGa0g (Untitled)
  Session 9: cR6An2gVFewZFSpJtYsMyEnIJncf5Tzpoo3RfrGqjYlCs9zDlyX2dK6P12y0290EcNYoUFv_qJ4yNKntfB5FiQ (Untitled)
  Session 10: sjw3xkXIxgArVNtRuyACQsboVfY9DOim8ONxof_98hSxd10oa0qj3Fmy1VkIkfSUnFF-eYoGtjuv002PIBHhcQ (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: C879a3EZsbG8KMHkNez_J4TXCnRyWT0Pw0IObh66AHfr0TDy3JqujriZ8iLvz2xZfzeGXo6NhEyvU5vXHCx0AA

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
✅ [32mJavaScript client examples complete![0m
