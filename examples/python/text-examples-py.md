make test-examples-python
🧪 [36mRunning Python examples (direct HTTP/WebSocket)...[0m
=== Simple CRUD Operations (Direct HTTP) ===

✓ Authentication successful

=== Insert Document ===
Inserted: {'id': 'Z2xsrNJCaHT6QjRjAoPMNnsXRxFgZtZZcxMc98wdCKA06W5T87C1Wcv9QU6zD9Sb1xYZ5WItDd-zk0-CtCDMFg'}

=== Find by ID ===
Found: {'id': 'Z2xsrNJCaHT6QjRjAoPMNnsXRxFgZtZZcxMc98wdCKA06W5T87C1Wcv9QU6zD9Sb1xYZ5WItDd-zk0-CtCDMFg', 'value': 42, 'active': True, 'name': 'Test Record'}

=== Find with Query ===
Found documents: [{'active': {'value': True, 'type': 'Boolean'}, 'value': {'type': 'Integer', 'value': 42}, 'name': {'type': 'String', 'value': 'Test Record'}, 'id': 'Z2xsrNJCaHT6QjRjAoPMNnsXRxFgZtZZcxMc98wdCKA06W5T87C1Wcv9QU6zD9Sb1xYZ5WItDd-zk0-CtCDMFg'}]

=== Update Document ===
Updated: {'value': {'value': 100, 'type': 'Integer'}, 'id': 'Z2xsrNJCaHT6QjRjAoPMNnsXRxFgZtZZcxMc98wdCKA06W5T87C1Wcv9QU6zD9Sb1xYZ5WItDd-zk0-CtCDMFg', 'active': {'value': True, 'type': 'Boolean'}, 'name': {'value': 'Updated Record', 'type': 'String'}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: 2KDKITf1SV3IdCNu3FWwWGoEg8fhkqMMeP7tox31rDf-3OcKECI-4LPKyr_GZjmDnd6O5hUtN5zh6TfLfBvPiQ

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
        "id": "2KDKITf1SV3IdCNu3FWwWGoEg8fhkqMMeP7tox31rDf-3OcKECI-4LPKyr_GZjmDnd6O5hUtN5zh6TfLfBvPiQ",
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
        "id": "9qANfwDEYn4jKGx25UL0lEibKplJLTThUsJZjNTuw-Mndg3CY8pV24Wc-19KGmdxxFpyeTosNU8sJe5uomjbCQ",
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
✓ Retrieved 2 record(s) via WebSocket

✓ WebSocket example completed successfully
🚀 ekoDB Scripts Example (Python/HTTP)

✓ Authentication successful

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: f52H-VIiX2_-XO5KLIIRTtSXtlh6Ua3T449Pis5HQH0CLBBdJPIYDbnu_Izt4LerGzeF7fXUzgk3ffZPS31l9A
📊 Found 35 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: BFYNziEO4ZEWSBBVoywYXHMAQshL7OtV9trKqA_bAHOx1pcm1s6L5UwbipOJseTDjPhDD4JUkzqpUpd3Y_AF6Q
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: WT51z7KpnIXjBnpN2VbNgVB3LG_T7EFHSSby0lJmeigJxw86XqCPNHYgNegzQMK06Y7cPvz1h59TQOqB-6N_vg
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {'count': 15, 'avg_score': 50.0, 'status': 'inactive', 'max_score': 90}
   {'max_score': 100, 'status': 'active', 'count': 15, 'avg_score': 60.0}

📝 Example 4: Function Management

📋 Total scripts: 15
🔍 Retrieved script: Get Active Users
✏️  Script updated
🗑️  Script deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

✅ All examples completed!
=== Batch Operations (Direct HTTP) ===

✓ Authentication successful

=== Batch Insert ===
✓ Batch inserted 5 records
✓ Verified: Found 10 total records in collection

=== Creating test records for update/delete ===
Created 3 test records

=== Batch Update ===
✓ Batch updated 3 records
✓ Verified: Record updated with status="active"

=== Batch Delete ===
✓ Batch deleted 3 records
✓ Verified: Records successfully deleted (not found)

✓ All batch operations completed successfully
=== Key-Value Operations (Direct HTTP) ===

✓ Authentication successful

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: {'username': 'john_doe', 'userId': 123}

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: {'name': 'Product 1', 'price': 29.99}
cache:product:2: {'name': 'Product 2', 'price': 39.989999999999995}
cache:product:3: {'name': 'Product 3', 'price': 49.989999999999995}

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
=== Collection Management (Direct HTTP) ===

✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: kQs8895oA1WIgTpI72x_5PuRiWY7XuWHJA-Ada2QKlJqI5un9bc2R2EQtipaEJFDeswMTFbjUzObNzI7diUJfQ

=== List Collections ===
Total collections: 12
Sample collections: ['demo_collection', 'websocket_test', 'batch_users', 'scripts__ek0_testing', 'test_accounts']

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: False

✓ All collection management operations completed successfully
✓ Authentication successful

📋 Getting original configuration...
   Original durable_operations: True


============================================================
🔥 TEST 1: Original Config (durable=True)
============================================================

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: aRQQLJV1fES8bJC1cGlXSvCtcKkMlCbqlX3tANyGsMzK7I38phihU8Bc_Yl4itXffV719vOjTvi-EavM9-DXfw
Created Bob: $500 - ID: PiaDrIh1MTQFGK9BoHVHHvj0IExMC-dVZgI9mlscfFOo5d4LMgs_f16uc7K2OZZaRrYE-YgRlTutEthN2H1JTw

=== Example 1: Begin Transaction ===
Transaction ID: f0dc4d54-ad31-492c-a58b-a3bbe69c0cbc

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: $800
Bob: $700

=== Example 5: Rollback ===
New transaction: b8c3bd69-a9a2-46a5-a47c-60d2edbdbb49
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: eFm_K-9bwC8y6v9lfcZDA_SYfNdoRoccxCz1imlyCGHn-bp4bXNcZTw6fFGPQiEbE1TmmiCk1EX4yNDdjIrzxQ
Created Bob: $500 - ID: aBLh52E1WvmNtzW6k_nsJzFXtvAnRQ7jRxXYcgJ4tywwOCDwB9Ychiv72E793buUSz0-gT_GLl3qvnl-RA9nvg

=== Example 1: Begin Transaction ===
Transaction ID: 11a22d39-e2a5-486c-929a-6e3c2f7a3e4d

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: $800
Bob: $700

=== Cleanup ===
✓ Deleted test accounts


🔄 Switching to NON-DURABLE mode...
   ✓ Config updated: durable_operations=false


============================================================
🔥 TEST 2: Non-Durable Mode (durable=false)
============================================================

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: kY11TYlQq0eZLZXOVdjfSOORMwDZFhBB9a818BSvye4rtGUSmgJVv45TaEmTlU6NuvOLSy2SeDI7WRDz8PgS2g
Created Bob: $500 - ID: sxZwQXqLEdWJvEZjleOkrj2K6NBSQ7O7Tdaqp_hVZjeVtv-p7kVZQW9ZVTV_cVWhOfyVfkgQsa9GQjjqQ_7asQ

=== Example 1: Begin Transaction ===
Transaction ID: 30a969d3-d7a2-4507-825d-eb9493fc737c

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: $800
Bob: $700

=== Example 5: Rollback ===
New transaction: 7f5a355f-ea90-4cfd-becf-938c3e9633a4
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: YBLrOBcGdVQDPX89OEg2uyli4p6hV7tmEz1iLln_myoy6VBvxwjxOJxOlRrkJJFpBKKfqUnJsNp3k8DrANTL8g
Created Bob: $500 - ID: PsDfEzjVPOJEuoGxXy8CWMfzM7l2CjQWYoeHDnr4GvqiHfpIwTckRHlpwfvpEFqC-NuytK8gMStvASThHuRyZQ

=== Example 1: Begin Transaction ===
Transaction ID: 0039632c-2068-43bf-aa2c-710b138447f7

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: $800
Bob: $700

=== Cleanup ===
✓ Deleted test accounts


🔄 Switching to DURABLE mode...
   ✓ Config updated: durable_operations=true


============================================================
🔥 TEST 3: Durable Mode (durable=true)
============================================================

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: gseXKiSOLgRzTIFxpmKaLWh-fBcD8yabDa3_jUMbgtWH09BB7NsXK-u0cISkdSXsqs374bztxbVRBeh6IxzyNQ
Created Bob: $500 - ID: zIbGD6tpw8rT5KvN69imny3xM4exNZUGQsnUicFRZgnw2fzLFpQpUqsh79AyBSMulzdgyCkzT1sJfZX0IQ179g

=== Example 1: Begin Transaction ===
Transaction ID: f5e33b83-d96b-4cd0-aef4-e2f3a1b899e8

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: $800
Bob: $700

=== Example 5: Rollback ===
New transaction: 29671454-9779-44fc-89fa-f561f7d52e2d
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: zlWZTKMfX1QEfpb2RvA4iv53xO-eJj75Z7iz9p1gZOrRRNdmtR5zDMNYCRv_1oYM1tIypWzNkvrJYkG0ujoqEg
Created Bob: $500 - ID: dguM7rZBg0nTaBGQv8oCqboBPwCKoZCPfnxv2DWjYIW-m2f5HgaUltr7mMzgoWumQPMRuQCaw3Aczf10NGqNRQ

=== Example 1: Begin Transaction ===
Transaction ID: b8ca6d0e-84d3-400e-a377-de60821d46d5

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: $800
Bob: $700

=== Cleanup ===
✓ Deleted test accounts


🔄 Restoring original configuration...
   ✓ Config restored: durable_operations=True


============================================================
✅ ALL TESTS PASSED - Transactions successful
============================================================

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
   ✅ Script saved: ICPmQlvsnhBzoU77FY5DuQw9csmESCzGt0b-hHIz_jIJOX3P_gVR1oWn7vje0RRmxDHEzKUaWcFmBiTpn8UzfA

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: Dys6IkEvBI2PqMcXe0AfHapLIq5cMN8BexDQEJzPmBxCCzvDzkq34mcP5gM9EkIGhOw_44D9lyLa8UvB3Y4Bfw
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}
   📋 Email: {'type': 'String', 'value': 'alice@example.com'}
   📋 Status: {'value': 'pending', 'type': 'String'}
   📋 Credits: {'value': 0, 'type': 'Integer'}

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: RcqSiZOkUy366tJAFU-oghVzYjs_ndvYkUDXPnrDbbB4IUkaCAypouDf2vRMYUssZdh90xaZUMiAL_4HYfCZFQ

2️⃣ Calling Script (Query + Update + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Status updated to: {'value': 'active', 'type': 'String'}
   📋 Name: {'value': 'Alice Smith', 'type': 'String'}

============================================================
📝 Script 3: Query + Update Credits + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: l0MvIPYeh9t7qULzq1J67Y48v_gaNfv2_PKCW6KSC38kC-qK19fcpcuOk0SSbEJsUO0piijSQPT7PC01GuK6cw

2️⃣ Calling Script (Query + Update Credits + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {'value': 0, 'type': 'Integer'}
   📋 Status: {'type': 'String', 'value': 'active'}
   📋 Name: {'value': 'Alice Smith', 'type': 'String'}

============================================================
📝 Script 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: 9UdvHKu9LtpNevBw2eiMnq7UlCvNf5Kdaxmdz9Cn6eRaQGryOZWLPMYbBMLOcIWs991bFzWBhGH4M0iFkHmABQ

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: ICPmQlvsnhBzoU77FY5D...
   ✅ Deleted script: RcqSiZOkUy366tJAFU-o...
   ✅ Deleted script: l0MvIPYeh9t7qULzq1J6...
   ✅ Deleted script: 9UdvHKu9LtpNevBw2eiM...
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
  Output: Document ID = uHjHfJhTgvOTrGMksAN_GcZa9b335BNSDWgyhuaqbX4USGzzHK90hhBYq7DS0wvYERoET7K5RwlHMZcUrectnQ
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: find_by_id(uHjHfJhTgvOTrGMksAN_GcZa9b335BNSDWgyhuaqbX4USGzzHK90hhBYq7DS0wvYERoET7K5RwlHMZcUrectnQ)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: find_by_id(uHjHfJhTgvOTrGMksAN_GcZa9b335BNSDWgyhuaqbX4USGzzHK90hhBYq7DS0wvYERoET7K5RwlHMZcUrectnQ)
  Output: Error (expected) - Find failed: Record not found
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
  Output: Document ID = vmsvwY_oDTmVtMStR0SZUPwob3ZoUkeBR5BbAT1MDfUOUKt3esBN6HF38heTY8zyfVZou4r3USxY4m91t5FxcQ
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: find_by_id(vmsvwY_oDTmVtMStR0SZUPwob3ZoUkeBR5BbAT1MDfUOUKt3esBN6HF38heTY8zyfVZou4r3USxY4m91t5FxcQ)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: find_by_id(vmsvwY_oDTmVtMStR0SZUPwob3ZoUkeBR5BbAT1MDfUOUKt3esBN6HF38heTY8zyfVZou4r3USxY4m91t5FxcQ)
  Output: Error (expected) - Find failed: Record not found
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
  • Queries correctly return None for expired documents
[34m
╔════════════════════════════════════════╗[0m
[34m║   ekoDB Python Examples Test Suite    ║[0m
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
=== Running simple_crud.py ===[0m
[32m✓ simple_crud.py completed successfully[0m
[34m
=== Running simple_websocket.py ===[0m
[32m✓ simple_websocket.py completed successfully[0m
[34m
=== Running http_functions.py ===[0m
[32m✓ http_functions.py completed successfully[0m
[34m
=== Running batch_operations.py ===[0m
[32m✓ batch_operations.py completed successfully[0m
[34m
=== Running kv_operations.py ===[0m
[32m✓ kv_operations.py completed successfully[0m
[34m
=== Running collection_management.py ===[0m
[32m✓ collection_management.py completed successfully[0m
[34m
=== Running transactions.py ===[0m
[32m✓ transactions.py completed successfully[0m
[34m
=== Running crud_scripts.py ===[0m
[32m✓ crud_scripts.py completed successfully[0m
[34m
=== Running document_ttl.py ===[0m
[32m✓ document_ttl.py completed successfully[0m
[34m
=== Running websocket_ttl.py ===[0m
[32m✓ websocket_ttl.py completed successfully[0m
[34m
╔════════════════════════════════════════╗[0m
[34m║           Test Summary                 ║[0m
[34m╚════════════════════════════════════════╝[0m
[34mTotal: 10[0m
[32mPassed: 10[0m
[32mFailed: 0[0m
✅ [32mPython direct examples complete![0m
🐍 [36mBuilding Python client package...[0m
🍹 Building a mixed python/rust project
🔗 Found pyo3 bindings with abi3 support
🐍 Not using a specific python interpreter
💻 Using `MACOSX_DEPLOYMENT_TARGET=11.0` for aarch64-apple-darwin by default
    Finished `release` profile [optimized] target(s) in 0.08s
📦 Built wheel for abi3 Python ≥ 3.8 to /Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.8.0-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel for current platform...[0m
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.8.0-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.8.0
    Uninstalling ekodb_client-0.8.0:
      Successfully uninstalled ekodb_client-0.8.0
Successfully installed ekodb-client-0.8.0
✅ [32mPython client package built and installed![0m
🧪 [36mRunning Python client library examples...[0m
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {'id': '8tSA_wyFdqJ1CaCO-ux65eARhMBkE4d71R-0MSjQ6apVo_h06fgaLm7tC9_8ayFENyjCGWalmZkXf0gh6KQ83g'}

=== Find by ID ===
Found: {'value': 42, 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5], 'categories': ['electronics', 'computers'], 'created_at': '2026-01-26T15:19:26.442969', 'tags': ['tag1', 'tag2', 'tag3'], 'id': '8tSA_wyFdqJ1CaCO-ux65eARhMBkE4d71R-0MSjQ6apVo_h06fgaLm7tC9_8ayFENyjCGWalmZkXf0gh6KQ83g', 'active': True, 'price': 99.99, 'metadata': {'nested': {'deep': True}, 'key': 'value'}, 'user_id': '550e8400-e29b-41d4-a716-446655440000', 'name': 'Test Record', 'data': 'aGVsbG8gd29ybGQ='}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): True
  price (Decimal): 99.99
  created_at (DateTime): 2026-01-26 15:19:26.442969
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): ['tag1', 'tag2', 'tag3']
  metadata (Object): {'nested': {'deep': True}, 'key': 'value'}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): ['electronics', 'computers']
  data (Bytes): 11 bytes
Plain record: {'value': 42, 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5], 'categories': ['electronics', 'computers'], 'created_at': '2026-01-26T15:19:26.442969', 'tags': ['tag1', 'tag2', 'tag3'], 'id': '8tSA_wyFdqJ1CaCO-ux65eARhMBkE4d71R-0MSjQ6apVo_h06fgaLm7tC9_8ayFENyjCGWalmZkXf0gh6KQ83g', 'active': True, 'price': 99.99, 'metadata': {'nested': {'deep': True}, 'key': 'value'}, 'user_id': '550e8400-e29b-41d4-a716-446655440000', 'name': 'Test Record', 'data': 'aGVsbG8gd29ybGQ='}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'user_id': {'value': '550e8400-e29b-41d4-a716-446655440000', 'type': 'String'}, 'name': {'type': 'String', 'value': 'Updated Record'}, 'tags': {'value': ['tag1', 'tag2', 'tag3'], 'type': 'Array'}, 'price': {'value': 99.99, 'type': 'Float'}, 'data': {'type': 'String', 'value': 'aGVsbG8gd29ybGQ='}, 'categories': {'type': 'Array', 'value': ['electronics', 'computers']}, 'embedding': {'value': [0.1, 0.2, 0.3, 0.4, 0.5], 'type': 'Array'}, 'metadata': {'value': {'key': 'value', 'nested': {'deep': True}}, 'type': 'Object'}, 'active': {'type': 'Boolean', 'value': True}, 'created_at': {'type': 'String', 'value': '2026-01-26T15:19:26.442969'}, 'value': {'type': 'Integer', 'value': 100}, 'id': '8tSA_wyFdqJ1CaCO-ux65eARhMBkE4d71R-0MSjQ6apVo_h06fgaLm7tC9_8ayFENyjCGWalmZkXf0gh6KQ83g'}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: Ve5TJ4rUub2Eg609O3j3uw3JttUBauipeAUaAQYMgWAdk3nSeU2kL7aelKE5TS9zD6m8EIoHclizeWOOFpNAfg

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
Collection created with first record: "0o_YrHJyDHIChlQ1d7OtBEFjEKPFdzbSfGbpW_OIKCaFF9rP4U0HD9SCTi4l4Ma3iAQS0tD7fHiPsRQoD6950A"

=== List Collections ===
Total collections: 11
Sample collections: ['websocket_test', 'batch_users', 'client_collection_management_python', 'scripts__ek0_testing', 'test_accounts']

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: False

✓ All collection management operations completed successfully
✓ Client created

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: {'value': '{"userId":123,"username":"john_doe"}'}

=== KV Batch Set ===
✓ Batch set 3 keys
  cache:product:1: success
  cache:product:2: success
  cache:product:3: success

=== KV Batch Get ===
✓ Batch retrieved 3 values
  cache:product:1: {'name': 'Product 1', 'price': 29.99}
  cache:product:2: {'price': 39.99, 'name': 'Product 2'}
  cache:product:3: {'price': 49.99, 'name': 'Product 3'}

=== KV Exists ===
Key exists: True

=== KV Find (Pattern Query) ===
Found 3 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 9

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: False

=== KV Batch Delete ===
✓ Batch deleted 3 keys
  cache:product:1: deleted
  cache:product:2: deleted
  cache:product:3: deleted

✓ All KV operations completed successfully
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: CL829QCIBuW_RHQABaFefnH5gY_SVkabo0NN0LzkrBZ99CpaO9CiRR7MDZTLVbV56OtWTNsOirmQ6qVd10ETMQ
Created Bob: $500 - ID: dyfLONiIZW-MRof4DI7LxFV-ATsvveNpXZnAVy02f_TnbBkYnHIGz8OUAstNN7P0T1uBughP1wrwdMaKsrmYtw

=== Example 1: Begin Transaction ===
Transaction ID: b51184c3-aa1e-4c8a-a063-684db637c180

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: a78b236f-063a-4ce9-a035-414a79544871
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
Found 2 valid users

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
  1. Score: 26.400, Matched: title.value, bio, bio.value, title
  2. Score: 26.400, Matched: bio, title, bio.value, title.value
  3. Score: 26.400, Matched: bio.value, bio, title, title.value
  4. Score: 26.400, Matched: title.value, bio.value, bio, title

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title, title.value, bio.value, bio
  2. Score: 39.600, Matched: bio.value, title, bio, title.value
  3. Score: 39.600, Matched: bio.value, title, bio, title.value
  4. Score: 39.600, Matched: bio, bio.value, title.value, title

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 2 results (matches: run, running, runs)
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.777
  2. Score: 0.746
  3. Score: 0.708

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.388, Matched: title, title.value, content, content.value
  2. Score: 1.354, Matched: content.value, title.value, title, content
  3. Score: 0.373, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 13.200, Matched: title, title.value

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
Schema fields: ['age', 'email', 'name', 'status']
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
  - Mouse ($25) by Alice Johnson
  - Laptop ($1200) by Alice Johnson

5. Complex join with multiple conditions:
Found 2 users with example.com emails:
  - Alice Johnson (alice@example.com): Building A
  - Bob Smith (bob@example.com): Building B

=== Cleanup ===
✅ Deleted test collections

✅ Join operations examples completed!
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: pKVN9j-0LxezlaY30uZkvDBv8wqBtyDOyB5TKLFV9UZFId_SEkj2JtDShxzyvTC7t_QEEiOYvTzsucUBA6t5Eg

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: MN4eoFCLPTln8IyiMtc04zpL3QgDPjBu5TLeOapbYdv3U_ClujQZqJxt-9nm1XaVz--xmXoYNE-1DYzGezc5NA

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
✓ Inserted document with TTL: Y0Q0UuBh3mmlyJaR5IgSueQJ2QX10QV2b6en2nFEWnFxiat7rOwkKZcVvzG8EAbQexOgX3jBEX97cBtfQz9ADg

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 5 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
=== ekoDB as Edge Cache - Simple Example ===

Creating edge cache function...
✓ Edge cache script created: AETWkc8wj3p3HrcUoWcD2NZexyugsTqTF3LBLlf7a9qZc5xder7EYRGsc19eJiw-AuOjzDJ7MyJOmbxkiNBIGg

Call 1: Cache miss (fetches from API)
Response time: 1ms
Result: {
  "records": [],
  "stats": {
    "execution_time_ms": 0,
    "input_count": 0,
    "output_count": 0,
    "stage_stats": [],
    "stages_executed": 2
  }
}

Call 2: Cache hit (served from ekoDB)
Response time: 2ms (0.8x faster!)
Result: {
  "records": [],
  "stats": {
    "execution_time_ms": 0,
    "input_count": 0,
    "output_count": 0,
    "stage_stats": [],
    "stages_executed": 2
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

🚀 ekoDB Scripts Example (Python)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: 43_aKnpkbMhF9AfjG_6Og54fkzx9tq2Vs1L3QMXcOzUyDlqrwK0NDjDdxVRnosg0AK7g87LWsPjn-6GY_ffr0Q
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved: tgnTa5Ndq9xws6qAY7PtHfL7VCAViQiOUCbkCWZBDgVJl-DDImEf8cUX4j35nMT5jEI2CwYBxtCoVhvDNth0TA
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: y0p2pIrm8F_qITBWGarL4MY2u4IvZfiK_gdg7eY6tGarC6Z63L_5TOAIvGSg53OfPtWOFT-hJIUmAJvOgOOjMA
📊 Statistics: 2 groups
   {'avg_score': 60.0, 'count': 5, 'status': 'active'}

   {'avg_score': 50.0, 'count': 5, 'status': 'inactive'}

📝 Example 4: Script Management

📋 Total scripts: 18
🔍 Retrieved script: Get Active Users
✏️  Script updated
🗑️  Script deleted

ℹ️  Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.

✅ All examples completed!
=== ekoDB Function Composition Examples ===

📋 Setting up test data...

✅ Test data ready

📝 Example 1: Basic Function Composition

Building reusable functions that call each other...

✅ Saved reusable function: fetch_user
✅ Saved composed function: get_user_wrapper (calls fetch_user + projects fields)

📊 Result from composed function:
   Records: 1
   Name: {"type": "String", "value": "User 1"}
   Department: {"type": "String", "value": "engineering"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 1.9ms
   📊 Records: 0

Second call (cache hit - from cache):
   ⏱️  Duration: 1.9ms
   📊 Records: 0
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
🚀 ekoDB Python Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved: j7CFBRcX1V-WbMjyd5LMglFE1wcJh4WvjfAT1I44MRQDZm5GIxRUOA2W9Xair-jtbxEQue9JpR9SIhchSiKbzA
📊 Found 2 product groups
   {'avg_price': 474.0, 'category': 'Furniture', 'count': 2}
   {'avg_price': 575.6666666666666, 'category': 'Electronics', 'count': 3}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved
📊 Found 2 categories
   {'category': 'Electronics', 'count': 3}
   {'category': 'Furniture', 'count': 2}
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
🚀 ekoDB Python KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: cFgTXluLju2K_rCs5Neajv0q0p0aDCJ7bhGdjf-e26MavIeg1u7EQoKftoC8rQJoCL0fKwH1rHtKg9B1Wbzk1Q
✅ Inserted 2 products with wrapped types

📝 Example 2: Querying and Extracting Wrapped Types

📊 Found 2 products
   • Wireless Mouse
   • Laptop Pro

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {'value': '{"role":"admin","userId":"user_abc"}'}
🔍 Key exists: True
✅ Set cached data with 1 hour TTL
🗑️  Deleted session: None

📝 Example 4: KV Pattern Query

✅ Set 4 config entries
📊 Found 3 app config entries
📊 Found 4 total config entries

📝 Example 5: Combined Wrapped Types + KV Usage

✅ Inserted order: ByD3b3Yq4Uj_7RXZ1NaXLmzSM4GFQDfitbZmyZ_kZlMV2-2NHLTcovYKBnu7IYyrQf37plAQs9qUnt0iD_DVpw
✅ Cached order status
📊 Quick status lookup: {'value': '{"status":"processing","updated_at":"2026-01-26T20:19:28.280706+00:00"}'}

🧹 Cleaning up...
✅ Cleanup complete

✅ All KV & Wrapped Types examples completed!

💡 Key takeaways:
   ✅ Use field_* helpers for type-safe wrapped values
   ✅ field_decimal() preserves precision (no floating point errors)
   ✅ KV store is great for caching and quick lookups
   ✅ Combine KV caching with collection inserts for real workflows
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_api_user_py (Y-nVfd-F1oTFZLoWMZe1dqt8RP0J7zfNUAmIYITcYuOpYyW8EklrU3wIoQ2ozwvjkFSGm_V6EY0Hh-0KS1NX1w)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [],
  "stats": {
    "execution_time_ms": 0,
    "input_count": 0,
    "output_count": 0,
    "stage_stats": [],
    "stages_executed": 2
  }
}
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 2ms (served from cache)
✓ Lightning fast cache hit

🧹 Cleaning up...
✓ Cleanup complete

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation
🚀 ekoDB Python Advanced Scripts Example

📋 Setting up test data...
✅ Created 8 products

📝 Example 1: List All Products

✅ Script saved
📊 Found 8 products
⏱️  Execution time: 0ms

📝 Example 2: Group Products by Category

✅ Script saved
📊 Category breakdown:
   {'avg_price': 365.6666666666667, 'category': 'Furniture', 'count': 3}
   {'avg_price': 367.0, 'category': 'Electronics', 'count': 5}
⏱️  Execution time: 0ms

📝 Example 3: Count Total Products

✅ Script saved
📊 Total products: [{'total': 8}]
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced script examples finished!
🚀 ekoDB Python AI Scripts Example

📋 Setting up test data...
✅ Created 2 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
🤖 AI Response:
   Vector databases provide benefits such as precise representation of geographic features, high-quality graphical output, efficient data storage, and the ability to perform complex spatial operations such as topology and network analysis. They also allow for easy editing and manipulation of data.
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embed script saved
📊 Embedding generated
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All AI script examples finished!

💡 This example demonstrates:
   ✅ Chat completions with system/user messages
   ✅ Embedding generation for text
🚀 ekoDB Python CRUD Scripts Example

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
📊 Average score by role:
   {'avg_score': 20.0, 'count': 3, 'role': 'admin'}
   {'avg_score': 70.0, 'count': 7, 'role': 'user'}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD script examples finished!
🚀 ekoDB Python Search Scripts Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Script saved
📊 Found 5 documents
   1. Introduction to Machine Learning (AI)
   2. Getting Started with ekoDB (Database)
   3. Natural Language Processing (AI)
   4. Database Design Principles (Database)
   5. Vector Databases Explained (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Script saved
📊 Documents by category:
   {'category': 'AI', 'count': 2}
   {'category': 'Database', 'count': 3}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: 1QxWJqEU5iXvj8VlydtoGuU4bMr3VU6FwXWamrjsnxkrO5EcgEEzdzqt0TIbdGXppbLVvhddduxa-EooZD6DwA

=== Sending Chat Message ===
Message ID: sC7rllVc8o3j2urK5zNpdgbNWpYI6g8fUBaYLbWFLnkZ-ByDsNsKXWOCy7etrIL6jWXZf1YwiEs64xGITtlE2A

=== AI Response ===
There are three products available:

1. ekoDB: This is a high-performance database product with AI capabilities. The price is $99.

2. ekoDB Cloud: This is a fully managed cloud database service product. It costs $499.

3. ekoDB Pro: This is the enterprise edition product with advanced features. It is priced at $299.

Execution Time: 3309ms

=== Token Usage ===
Prompt tokens: 453
Completion tokens: 75
Total tokens: 528

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: EC4s3AkjVWHw77Mx3kHVBEDVUkMT2KaclYgjXKbsL19Qe5JmaivrFhAxfSv_UoMGaujir0GEZEfROv__IC_KRQ

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the context provided, the available product is ekoDB. It is a high-performance database product priced at $99.

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['content', 'id', 'chat_id', 'context_snippets', 'token_usage', 'role', 'created_at', 'updated_at'])
Debug: First message role: {'type': 'String', 'value': 'assistant'}
=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of ekoDB is $99.

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: IU5sdisPG6DUPq-ArVY1FAD_5uVNxM8FNtb3Mt3yyAzGdsVe3Ob_TyahjMtCqe5FF_gblxFglE6d7dzXhye3qw
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
✓ Created session: K4Zl3YC8nD5iX6ndffmischiOraG9LEaYA26OQQC0MoUl9OpC9dq8LjD56bzWFfOvTHSry7KQ7fIVXNuqRK-Ng

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information provided, there is a high-performance database product called "ekoDB". It is priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: ZZskMBzqAYnhJkUCLIWM6kDIOdO6tOXleTR8fIslhAJWz89mttc98blXYB0mYWW2oPhzcdV04dijAlG0yrRPVQ
  Parent: K4Zl3YC8nD5iX6ndffmischiOraG9LEaYA26OQQC0MoUl9OpC9dq8LjD56bzWFfOvTHSry7KQ7fIVXNuqRK-Ng

=== Listing Sessions ===
✓ Found 7 sessions
  Session 1: ZZskMBzqAYnhJkUCLIWM6kDIOdO6tOXleTR8fIslhAJWz89mttc98blXYB0mYWW2oPhzcdV04dijAlG0yrRPVQ (Untitled)
  Session 2: K4Zl3YC8nD5iX6ndffmischiOraG9LEaYA26OQQC0MoUl9OpC9dq8LjD56bzWFfOvTHSry7KQ7fIVXNuqRK-Ng (Untitled)
  Session 3: IU5sdisPG6DUPq-ArVY1FAD_5uVNxM8FNtb3Mt3yyAzGdsVe3Ob_TyahjMtCqe5FF_gblxFglE6d7dzXhye3qw (Untitled)
  Session 4: 1QxWJqEU5iXvj8VlydtoGuU4bMr3VU6FwXWamrjsnxkrO5EcgEEzdzqt0TIbdGXppbLVvhddduxa-EooZD6DwA (Untitled)
  Session 5: B34WCoxf8nH3TbbVuxeVT7jxixrqEQikzOAkSa98N7JrmbrCvCu6OW1i12WHAEuGg8KNoC7IvEpOHjea5Xgvuw (Untitled)
  Session 6: BwYFP0uVQGNlc6VCBff2PHtOXg5E3CgP3FUZ8bM1h3h0KsAH26riq9qTJpUkDs-zK5YmRyXuizzVil7impBBOA (Untitled)
  Session 7: haPvVPHhURHG2gR4H89asUbBWgrhgXueRh4xFZptY19J391BZb892uDfR09WI-AiyvEjF8XAXJ-0axNcGlFJew (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: ZZskMBzqAYnhJkUCLIWM6kDIOdO6tOXleTR8fIslhAJWz89mttc98blXYB0mYWW2oPhzcdV04dijAlG0yrRPVQ

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Dict Creation ===
✓ Created record with plain dict: {'id': 'ubVmlQ0RB3Zz8CqguqIgvDCkGTUyfE1i_wVVD-0GHYLQWv47ne3J4gCPS9me2JvnVpynNupueHgPVpGTBHi8jA'}

=== Upsert Operation ===
✓ Upsert (update existing record): ubVmlQ0RB3Zz8CqguqIgvDCkGTUyfE1i_wVVD-0GHYLQWv47ne3J4gCPS9me2JvnVpynNupueHgPVpGTBHi8jA
✓ Inserted second record: EM7vb_GtGn24xjrLMaffwH1-rH57m-4-wonQvxY-L4Q22Oj1R4NnIDDz1d8QZusqpCHAN9noMAH5tqByZvLMfQ
✓ Upsert (update second record): EM7vb_GtGn24xjrLMaffwH1-rH57m-4-wonQvxY-L4Q22Oj1R4NnIDDz1d8QZusqpCHAN9noMAH5tqByZvLMfQ

=== Find One Operation ===
✓ Found user by email: {'age': {'type': 'Integer', 'value': 29}, 'active': {'value': True, 'type': 'Boolean'}, 'email': {'value': 'alice.j@newdomain.com', 'type': 'String'}, 'name': {'type': 'String', 'value': 'Alice Johnson'}, 'id': 'ubVmlQ0RB3Zz8CqguqIgvDCkGTUyfE1i_wVVD-0GHYLQWv47ne3J4gCPS9me2JvnVpynNupueHgPVpGTBHi8jA'}
✓ User not found (as expected)

=== Exists Check ===
✓ Record exists: True
✓ Fake record exists: False (should be False)

=== Pagination ===
✓ Inserted 25 records for pagination
✓ Page 1: 10 records (expected 10)
✓ Page 2: 10 records (expected 10)
✓ Page 3: 7 records (expected ~7)

=== Cleanup ===
✓ Deleted collection

✅ All convenience methods demonstrated successfully!
=== Bypass Ripple Example ===

1. Basic insert (ripple enabled):
   Inserted with ripple: {'id': 'G99FFztazv0VdSLXmvD7UB60Tms1ITJv7KKuHBgew--ScH8OP5snLQkSZfsGgAQvNIo6wgF7uVRMUnYuho0nKA'}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {'id': 'xiTXHsz_48sljtkC3GgH4iCYRTbkto9O4JJs-0g-25lKmfTm4CjdOb79yJo9ArmoyOrhWDsuWan-BUOQD6xYAA'}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {'id': 'G99FFztazv0VdSLXmvD7UB60Tms1ITJv7KKuHBgew--ScH8OP5snLQkSZfsGgAQvNIo6wgF7uVRMUnYuho0nKA', 'name': {'type': 'String', 'value': 'Product 1'}, 'price': {'value': 150, 'type': 'Integer'}}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {'id': 'G99FFztazv0VdSLXmvD7UB60Tms1ITJv7KKuHBgew--ScH8OP5snLQkSZfsGgAQvNIo6wgF7uVRMUnYuho0nKA', 'price': {'value': 500, 'type': 'Integer'}, 'name': {'value': 'Upsert Product', 'type': 'String'}}

✅ All bypass_ripple operations completed successfully!
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields
Fetched 3 users with only 3 fields each

Example 2: Exclude sensitive fields
Fetched 2 admins without sensitive data
  Password field excluded: False

Example 3: Complex query with projection
Fetched 3 active users with profile fields

Example 4: Find by ID with projection
Fetched user profile: {'type': 'String', 'value': 'Alice Johnson'}

Example 5: Compare full vs projected data
Full query returned 12 fields per user
Projected query returned 12 fields per user

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
✅ [32mPython client examples complete![0m
