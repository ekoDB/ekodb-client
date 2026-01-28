make test-examples-go
🧪 [36mRunning Go examples (direct HTTP/WebSocket)...[0m
[34m
╔════════════════════════════════════════╗[0m
[34m║     ekoDB Go Examples Test Suite      ║[0m
[34m╚════════════════════════════════════════╝[0m
[34m
=== Checking Server Connection ===[0m
[32m✓ Server is ready[0m
[33m
=== Running 10 Examples ===[0m
[34m
=== Running simple_crud.go ===[0m
=== Simple CRUD Operations (Direct HTTP) ===

✓ Authentication successful

=== Insert Document ===
Inserted: map[id:Hdb0e3AmT_h9f949JNuP5bHIL_o1IeAiq0i3wDq5CXV621kb5Icd7BPPuLt5KzMYaa2rmJxB8AoiNS-g_VyAtw]

=== Find by ID ===
Found: map[active:true id:Hdb0e3AmT_h9f949JNuP5bHIL_o1IeAiq0i3wDq5CXV621kb5Icd7BPPuLt5KzMYaa2rmJxB8AoiNS-g_VyAtw name:Test Record value:42]

=== Find with Query ===
Found documents: [map[active:map[type:Boolean value:true] id:Hdb0e3AmT_h9f949JNuP5bHIL_o1IeAiq0i3wDq5CXV621kb5Icd7BPPuLt5KzMYaa2rmJxB8AoiNS-g_VyAtw name:map[type:String value:Test Record] value:map[type:Integer value:42]]]

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] id:Hdb0e3AmT_h9f949JNuP5bHIL_o1IeAiq0i3wDq5CXV621kb5Icd7BPPuLt5KzMYaa2rmJxB8AoiNS-g_VyAtw name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
[32m✓ simple_crud.go completed successfully[0m
[34m
=== Running simple_websocket.go ===[0m
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: 0LJADB61uaAeguYwN2eepp9qhGcqs5GjvarSJQCE3epOh5jnZ1-_6iMblhDP7snnYDeYGoTNJuOLfjFj3G1CFA

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
        "id": "_hwl46HALMS28RzJD93jwOtMeWUYANFIwzG5gSnz1rqwWezvbHOgeMS3LMVgSh3lKcXnZdihlCqDmX0udb80oA",
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
        "id": "0LJADB61uaAeguYwN2eepp9qhGcqs5GjvarSJQCE3epOh5jnZ1-_6iMblhDP7snnYDeYGoTNJuOLfjFj3G1CFA",
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
        "id": "oJm5UDTCfxwFeKzYk0FPOeqI3I76oqgMzD9mKIGAVecxCXdowuT5fEPyBstw_M5Ym1RAJ-tWx-mzm-V2sens6A",
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
✓ Retrieved 3 record(s) via WebSocket

✓ WebSocket example completed successfully
[32m✓ simple_websocket.go completed successfully[0m
[34m
=== Running http_functions.go ===[0m
🚀 ekoDB Scripts Example (Go/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: 9fpaKzec3L4pANGD4zg5HWLBN7dr3C9cPo4MPNbo0KD7qK1azl9xizm4wjXXGkCKHDeK00kdtK3rmz8S0LyAbg
📊 Found 10 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: 7jlgsGXSceVtsZBC5phrhRzl7XL_AY-UGUG0DygSb1NZgE7kNfmu47UPhN5vIV9A3EEoJaef2qYh3ez98LyIxg
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: cpdPCr5_ClURdJf_yjYpP-5KfkCW_b8rqw5-AXuOmEQ0gxGFz7y4oUphgP19DpaML-sGHu4QSkKYrmT26i_1fA
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"avg_score":50,"count":10,"max_score":90,"status":"inactive"}
   {"avg_score":60,"count":10,"max_score":100,"status":"active"}

📝 Example 4: Function Management

📋 Total scripts: 27
🔍 Retrieved script: Get Active Users
✏️  Script updated
🗑️  Script deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

✅ All examples completed!
[32m✓ http_functions.go completed successfully[0m
[34m
=== Running batch_operations.go ===[0m
=== Batch Operations (Direct HTTP) ===

✓ Authentication successful

=== Batch Insert ===
✓ Batch inserted 5 records

=== Creating test records for update/delete ===
Created 3 test records

=== Batch Update ===
✓ Batch updated 3 records

=== Batch Delete ===
✓ Batch deleted 3 records
✓ Verified: Records successfully deleted (not found)

✓ All batch operations completed successfully
[32m✓ batch_operations.go completed successfully[0m
[34m
=== Running kv_operations.go ===[0m
=== Key-Value Operations (Direct HTTP) ===

✓ Authentication successful

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: map[userId:123 username:john_doe]

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: map[name:Product 1 price:29.99]
cache:product:2: map[name:Product 2 price:39.989999999999995]
cache:product:3: map[name:Product 3 price:49.989999999999995]

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
[32m✓ kv_operations.go completed successfully[0m
[34m
=== Running collection_management.go ===[0m
=== Collection Management (Direct HTTP) ===

✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: -0RIE01vEKllYJ9AKyWGVOuiGRilsCM4TMReQ4rAKpcXoEWLDnfVU3tvDytbzZj1dEY2aIWax1LznnhHo6_sGA

=== List Collections ===
Total collections: 16
Sample collections: [test_accounts schema_documents_client_py schema_products_client_py schema_users_client_py ttl_cache]

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
[32m✓ collection_management.go completed successfully[0m
[34m
=== Running transactions.go ===[0m
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: hQ8GoNCTU1ShSJKwBuf8FpMoXhL3_0OwZzOadc4O4vU41SgSXwLawzxaEH3mpHf5kqOLbH5eyyo9J-VDpWilqA
Created Bob: $500 - ID: cf1UJ1LAbfxpF5TxjS7CiGod-OMzJQe4O1suxgHSY4IFGfVu9bF_j5Q8Y3BhhyDObaQclmMdtU4JnmfKn3ttxw

=== Example 1: Begin Transaction ===
Transaction ID: 9c940df5-9346-42ca-8a3e-3fa48673735d

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
New transaction: 14d038bc-29d6-477b-a134-52d200c30946
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: 700

=== Cleanup ===
✓ Deleted test accounts

✓ All transaction examples completed
[32m✓ transactions.go completed successfully[0m
[34m
=== Running crud_scripts.go ===[0m
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
   ✅ Script saved: FPGJaqr8yGD7Gi0nx37FmzJSTMysPa_mrAq8sfFtaMKgz7cvb8kgpAaWSt-VXuccVk9mIxRpCy0sI9Bhk2Mo9A

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: nFQdY28jzeh7BGjtyyyQSa4e8QcC2bJCytte26KnJ2ATCTaA8m28laSaDNIpl7pHn78Vs0OLoxf5Tq-BXAsoqA
   📋 Name: map[type:String value:Alice Smith]
   📋 Email: map[type:String value:alice@example.com]
   📋 Status: map[type:String value:pending]
   📋 Credits: map[type:Integer value:0]

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: _kqPoldjPx9XHBbbqwuuzA5_88dDqYNtqIdhG1lsRgsZ5pC1R4c5Ej_nNF9husSwRNL-rLwwHW26nvKvQBbfDQ

2️⃣ Calling Script (Query + Update + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Status updated to: map[type:String value:active]
   📋 Name: map[type:String value:Alice Smith]

============================================================
📝 Script 3: Query + Update Credits + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: n0-UtyHe0pw-WMEIMVJTLqOkuiQdArVX2wlBVpQv-J-ii7vrfFLER_cR-bSIKNLwrDPbG2bJSxDFSBo37kaLBA

2️⃣ Calling Script (Query + Update Credits + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: map[type:Integer value:0]
   📋 Status: map[type:String value:active]
   📋 Name: map[type:String value:Alice Smith]

============================================================
📝 Script 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: 4TrYZT-1Ob1kqDyVKpJjHmKuYMeKqyjICHLudlAbKVhTyioFlc7dDro6g5k2RBNeXg8K1XQvxYsZkHCTgOrhZg

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: FPGJaqr8yGD7Gi0nx37F...
   ✅ Deleted script: _kqPoldjPx9XHBbbqwuu...
   ✅ Deleted script: n0-UtyHe0pw-WMEIMVJT...
   ✅ Deleted script: 4TrYZT-1Ob1kqDyVKpJj...
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
[32m✓ crud_scripts.go completed successfully[0m
[34m
=== Running document_ttl.go ===[0m
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
  Output: Document ID = jZAp-MAe77bU8Icmm7IkmNx_Bl4L-HyfE-5toPVMX9V8BNMqyJSwmXNpM346qku13G9YSTG2-XE4466w0uenFQ
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: FindByID(jZAp-MAe77bU8Icmm7IkmNx_Bl4L-HyfE-5toPVMX9V8BNMqyJSwmXNpM346qku13G9YSTG2-XE4466w0uenFQ)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: FindByID(jZAp-MAe77bU8Icmm7IkmNx_Bl4L-HyfE-5toPVMX9V8BNMqyJSwmXNpM346qku13G9YSTG2-XE4466w0uenFQ)
  Output: Error (expected) - request failed with status 404: ��Record not found (expired)
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
[32m✓ document_ttl.go completed successfully[0m
[34m
=== Running websocket_ttl.go ===[0m
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
  Output: Document ID = s6MKsmt0rKWPCTFnw6wLHx0gElByY5qULaV3460j_XsM4uAb1IVZRQeoUSroz2GYsSqIu6ZKzwbR7h2NDofdSQ
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: FindByID(s6MKsmt0rKWPCTFnw6wLHx0gElByY5qULaV3460j_XsM4uAb1IVZRQeoUSroz2GYsSqIu6ZKzwbR7h2NDofdSQ)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: FindByID(s6MKsmt0rKWPCTFnw6wLHx0gElByY5qULaV3460j_XsM4uAb1IVZRQeoUSroz2GYsSqIu6ZKzwbR7h2NDofdSQ)
  Output: Error (expected) - request failed with status 404: ��Record not found (expired)
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
  • Queries correctly return nil for expired documents
[32m✓ websocket_ttl.go completed successfully[0m
[34m
╔════════════════════════════════════════╗[0m
[34m║           Test Summary                 ║[0m
[34m╚════════════════════════════════════════╝[0m
[34mTotal: 10[0m
[32mPassed: 10[0m
[32mFailed: 0[0m
✅ [32mGo direct examples complete![0m
🧪 [36mRunning Go client library examples...[0m
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: map[id:rbOYtVBEFtbyBvvUSq5h28NBw5YuhfqnPgulzWY69U8i6QNZ1B4xvH9HahUHXiX_8cmDmiyTlM74Jkd-dxXKdQ]

=== Find by ID ===
Found: map[active:true categories:[electronics computers] created_at:2026-01-27T19:12:41-05:00 data:aGVsbG8gd29ybGQ= embedding:[0.1 0.2 0.3 0.4 0.5] id:rbOYtVBEFtbyBvvUSq5h28NBw5YuhfqnPgulzWY69U8i6QNZ1B4xvH9HahUHXiX_8cmDmiyTlM74Jkd-dxXKdQ metadata:map[key:value nested:map[deep:true]] name:Test Record price:99.99 tags:[tag1 tag2 tag3] user_id:550e8400-e29b-41d4-a716-446655440000 value:42]

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 0
  active (Boolean): true
  price (Decimal): 99.990000
  created_at (DateTime): 2026-01-27 19:12:41 -0500 EST
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1 tag2 tag3]
  metadata (Object): map[key:value nested:map[deep:true]]
  embedding (Vector): [0.1 0.2 0.3 0.4 0.5]
  categories (Set): [electronics computers]
  data (Bytes): 11 bytes
Plain record: map[active:true categories:[electronics computers] created_at:2026-01-27T19:12:41-05:00 data:aGVsbG8gd29ybGQ= embedding:[0.1 0.2 0.3 0.4 0.5] id:rbOYtVBEFtbyBvvUSq5h28NBw5YuhfqnPgulzWY69U8i6QNZ1B4xvH9HahUHXiX_8cmDmiyTlM74Jkd-dxXKdQ metadata:map[key:value nested:map[deep:true]] name:Test Record price:99.99 tags:[tag1 tag2 tag3] user_id:550e8400-e29b-41d4-a716-446655440000 value:42]

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-01-28T00:12:41Z] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:rbOYtVBEFtbyBvvUSq5h28NBw5YuhfqnPgulzWY69U8i6QNZ1B4xvH9HahUHXiX_8cmDmiyTlM74Jkd-dxXKdQ metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Updated Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: d1lJv-L6XD97JqptqrDalP4FQwzTysGwhvc1R2dHWTS52egZgYWQ5HVhMuYh1IOmbPbs89KKxCE2SVdi07lMxw

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
Collection created with first record: 1O4v3ySalGPoSJ1bJxtzvg8t99ptGhY7yCDNYXOhyBfE2k4fqWxyoka-x3hodC-U7DfnV057Q2QrYtI_GvUAwg

=== List Collections ===
Total collections: 15
Sample collections: [test_accounts schema_documents_client_py schema_products_client_py schema_users_client_py ttl_cache]

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
Retrieved value: map[userId:123 username:john_doe]

=== KV Batch Set ===
✓ Batch set 3 keys
  cache:product:1: success
  cache:product:2: success
  cache:product:3: success

=== KV Batch Get ===
✓ Batch retrieved 3 values
  cache:product:1: map[name:Product 1 price:29.99]
  cache:product:2: map[name:Product 2 price:39.99]
  cache:product:3: map[name:Product 3 price:49.99]

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 3 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 12

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: false

=== KV Batch Delete ===
✓ Batch deleted 3 keys
  cache:product:1: deleted
  cache:product:2: deleted
  cache:product:3: deleted

✓ All KV operations completed successfully
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: pT5T0K0n807-7RmMrsh7O1xEj1bqurLvmduFbOp8vEmEr3XcWOqg4YM74XsagMWyHDj_IWGPkGIN2tF_yYjwkg
Created Bob: $500 - ID: 0WzVrpKboPaRpK45nO8jMpUjzLagNbxcq8MxhpRjHY6Rh5gWDLi2GD3j2OnKuoub3jWIo0Rh2DlUqICwKzgQ-Q

=== Example 1: Begin Transaction ===
Transaction ID: c4979cbf-84ae-473b-bc15-4ae6271bb9a6

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 9df5454a-44c0-497c-83f3-73dc298ff35f
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
  1. Score: 25.740
  2. Score: 12.540

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400
  2. Score: 26.400
  3. Score: 26.400
  4. Score: 26.400

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600
  2. Score: 39.600
  3. Score: 39.600
  4. Score: 39.600

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200
  2. Score: 13.200

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.773
  2. Score: 0.769
  3. Score: 0.751

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.707
  2. Score: 1.509
  3. Score: 0.301

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400

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
Schema fields: 4 fields
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
Found 2 users with department data
  - Bob Smith: Sales
  - Alice Johnson: Engineering

2. Join with filtering:
Found 1 users in Engineering
  - Alice Johnson: Building A

3. Join with user profiles:
Found 2 users with profile data
  - Bob Smith: Sales Manager
  - Alice Johnson: Senior Software Engineer

4. Join orders with user data:
Found 2 completed orders
  - Mouse ($0) by Alice Johnson
  - Laptop ($0) by Alice Johnson

5. Complex join with multiple conditions:
Found 2 users with example.com emails
  - Alice Johnson (alice@example.com): Building A
  - Bob Smith (bob@example.com): Building B

=== Cleanup ===
✅ Deleted test collections

✅ Join operations examples completed!
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: 3FB2jAdyEXNGAeacQFN6cODKbdPhAU9be11A5T9rZevVGX5QLqP1baMGP1aIa6mDF95AKn9wj3zPRgAdVnisIA

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: THweRLaq9_jCgsuUdIwVSxqrum2Qj82UUKistRA7CfCulOptj3cV8HcUNX2f7l2nsYHuB96nWCi2E9Yg0lsJRg

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
✓ Inserted document with TTL: I7wdhLVJCGzMmMfN1Z_RKecr2a03cuiNbhTu0l8gf5DKp_LCcAOj6XkTf1GGi6OiskG7Iqclelt0md66v28GWw

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 5 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
=== ekoDB as Edge Cache - Simple Example ===

Setting up edge cache collection...
✓ Cache entry created

Creating edge cache lookup script...
✓ Edge cache script created: 0_tSb8oBcDtzTMh4HJEGzjFiKPY8crd92zNHBggnpQbjTBtbC_q5AsuNq62JCwHqVYeGaZkruDgrvb6fyaon9g

Call 1: Cache lookup
Response time: 2ms
Found 1 cached entries

Call 2: Cache lookup (connection warm)
Response time: 1ms
Found 1 cached entries

🧹 Cleaning up...
✓ Cleanup complete

=== The Magic ===
- Your DATABASE is your EDGE
- No Redis needed
- No CDN needed
- No cache invalidation logic needed (TTL handles it)
- With ripples: All nodes auto-sync cache
- One service: Database + Cache + Edge Functions

✓ Example complete!
🚀 ekoDB Scripts Example (Go Client)

✅ Client initialized

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: NI6b1OxbGjAwLrjPSmJMeDUfa7RnHpHNE5rKlgf6hIPzFzBmtpooA_Z0twGWJVoTeMCuHURE2BdHzaQAv398tQ
📊 Found 10 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved
📊 Found 10 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved
📊 Statistics: 2 groups
   map[avg_score:50 count:5 status:inactive]
   map[avg_score:60 count:5 status:active]
⏱️  Execution time: 0ms

📝 Example 4: Script Management

📋 Total scripts: 29
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

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 3.46925ms
   📊 Records: 1
   📦 Data: {
        "value": {
          "address": {
            "city": "Gwenborough",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "street":...

Second call (cache hit - from cache):
   ⏱️  Duration: 3.275875ms
   📊 Records: 1
   📦 Data: {
        "value": {
          "address": {
            "city": "Gwenborough",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "street":...
   🚀 Cache speedup: 1.0x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

✅ Level 1 function: validate_user
✅ Level 2 function: fetch_slim_user (calls validate_user)
✅ Level 3 function: get_verified_user (calls fetch_slim_user)

📊 Result from 3-level nested composition:
   Records: 1
   Name: {"type":"String","value":"User 1"}
   Department: {"type":"String","value":"engineering"}

🎯 Key Benefit: Each function is independently testable and reusable!
   - validate_user: Used in 100 different workflows
   - fetch_slim_user: Used in 50 workflows
   - get_verified_user: Specific workflow


✅ All composition examples completed!
🚀 ekoDB Go Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved: 5bPoeaxCeGxPERuVOxb55ZsQku32HhXCwTySZFAhzfkolOnpEyR4XKffvMxKH16bs1GozfZpOxOGRT04iQ-RMA
📊 Found 2 category stats

📝 Example 2: Simple Product Listing

✅ Script saved
📊 Found 5 products

📝 Example 3: Count by Category

✅ Script saved
📊 Found 2 categories

📝 Example 4: Multi-Stage Pipeline (FindAll → Group → Count)

✅ Script saved
📊 Pipeline executed 3 stages

🧹 Cleaning up...
✅ Cleanup complete

✅ All complete script examples finished!

💡 This example demonstrates ekoDB's Script system:
   ✅ FindAll operations
   ✅ Group aggregations (Count, Average)
   ✅ Multi-stage pipelines (FindAll → Group → Count)
   ✅ Script management (save, call, delete)
🚀 ekoDB Go KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: qql6XzqjoBqfw6255Zt_6ajGDhHOzIxOhwwAyO3OX2KKiHCCNOnxhB1S-lFlGksb0x5H9tSKAFklzVzcSx6YPg
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: M5w8gHa50FghrvvPs66ZiGbpPMhaXTBVwD63LvnOqST4MVxKOgBudlnJLYwf5sxxoAQcKjA9dADye90LZcBlJw
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: map[role:admin userId:user_abc]
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session
📝 Example 4: KV Operations in Scripts

✅ Script saved: YTzXXcXdYF06o5bljZCy4pWp3qsmwaXDq0w3epHrvxAGBOvEzhmnvwDdI__OOb45UBPhLaZ05lSIqKXOXuZJXw
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Script

✅ Script saved: TfB5JKj6BYHKQ5subDUES34CoML1Ad_Q1kctgKZKZUP5SNdQScsu6ns6v6Kiif98MTkxIbb5OdEMjXaLn3S7RQ
📊 Processed order with caching
⏱️  Stages executed: 3
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All KV & Wrapped Types examples completed!

💡 Key takeaways:
   ✅ Use Field* helpers for type-safe wrapped values
   ✅ FieldDecimal() preserves precision (no floating point errors)
   ✅ KV store is great for caching and quick lookups
   ✅ StageKv*() functions work within scripts
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_api_user_go (PRSfSVjSoErHe1tbS4z-hRzGO0JuNCJOh2vN6ZFiwg8CGfENDG9puhJLRv_ySrAiweEIHMF4QQV1-n6FzeKvCQ)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "cached_at": "2026-01-27T19:12:49-05:00",
      "data": {
        "type": "Object",
        "value": {
          "address": {
            "city": "Gwenborough",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "street": "Kulas Light",
            "suite": "Apt. 556",
            "zipcode": "92998-3874"
          },
          "company": {
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona"
          },
          "email": "Sincere@april.biz",
          "id": 1,
          "name": "Leanne Graham",
          "phone": "1-770-736-8031 x56442",
          "username": "Bret",
          "website": "hildegard.org"
        }
      },
      "id": "IxA_gGJ-wvL1EtwfNJQVXLlxjf454QKdhp4GKjhgh4UITQC6t5YcipGMB3QRAigTBE2bwGNOJ9VYZCKoBakjhQ"
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
Response time: 64ms (served from cache)
✓ Lightning fast cache hit

🧹 Cleaning up...
✓ Cleanup complete

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation
🚀 ekoDB Go Client - Native SWR Function Examples

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
✓ Created native SWR script: github_user_native (rNrC_LzQWS7-gpu0Q7wOBcMVlFfW0QU3MKozGybvvhijmnNRny6LOChELhYIOsLyvnj3hAzAhvNywHB0aIY43A)

First call (cache miss - will fetch from GitHub API):
  Response time: 4ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 4ms
  Speedup: 1.0x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (Bc8PLyUprCTGk-Aey_UOTfHn0I1ybdy4hB3SAFZeiBln7YJOTSkOB693ITIDaIVfPeBbrvvSYxD5XWolvhz8SQ)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (RtRt-ynxTatsaFdUvjuzXHklS6Qgq-ncXGcE2pbrhKL_QilVlwdDW1Yv2hkOP7igCYGMZYT3vD46aIPdZomwyQ)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (jceO540i88-HDK8azcfRpVw09woZD18cYjCI6rRM9xWQOb0wWQxhB9YDo5wdhpXD7FsOeEDje8VhNmyxCtQNOg)
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
🚀 ekoDB Go Advanced Scripts Example

📋 Setting up test data...
✅ Created 8 products

📝 Example 1: List All Products

✅ Script saved
📊 Found 8 products
⏱️  Execution time: 0ms

📝 Example 2: Group Products by Category

✅ Script saved
📊 Category breakdown:
   map[avg_price:365.6666666666667 category:Furniture count:3]
   map[avg_price:367 category:Electronics count:5]
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced script examples finished!
🚀 ekoDB Go AI Scripts Example

📋 Setting up test data...
✅ Created 2 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
📊 AI Response generated
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embed script saved
🧹 Cleaning up...
✅ Cleanup complete

✅ All AI script examples finished!

💡 This example demonstrates:
   ✅ Chat completions with system/user messages
   ✅ Embedding generation for text
🚀 ekoDB Go CRUD Scripts Example

📋 Setting up test data...
✅ Created 10 test users

📝 Example 1: List All Users

✅ Script saved
📊 Found 10 users
⏱️  Execution time: 0ms

📝 Example 2: Count Users by Status

✅ Script saved
📊 User counts by status:
   map[count:3 status:inactive]
   map[count:7 status:active]
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD script examples finished!
🚀 ekoDB Go Search Scripts Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Script saved
📊 Found 5 documents
   1. map[type:String value:Introduction to Machine Learning] (map[type:String value:AI])
   2. map[type:String value:Database Design Principles] (map[type:String value:Database])
   3. map[type:String value:Vector Databases Explained] (map[type:String value:Database])
   4. map[type:String value:Natural Language Processing] (map[type:String value:AI])
   5. map[type:String value:Getting Started with ekoDB] (map[type:String value:Database])
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Script saved
📊 Documents by category:
   map[category:AI count:2]
   map[category:Database count:3]
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: 5HlYZzepTIGvK8nKfiHDDimKGkZe_gVOvDe1xEdj-yE_eDBZBijc4ne1Q4SstWoYaWF52WMpcUodRD87Ef9tLw

=== Sending Chat Message ===
Message ID: MShroDB2ZV5lHMgmYG6UAZac0JbOQId1OO0LdK8blf4pnyCNCXFYIhs9fXcDhgfIpFbyFFPkBzGNytGeRgCTBw

=== AI Response ===
There are three products available:

1. ekoDB Pro: This is an enterprise edition product with advanced features. The price is $299.

2. ekoDB Cloud: This is a fully managed cloud database service product. The price is $499.

3. ekoDB: This is a high-performance database product with AI capabilities. The price is $99.

=== Context Used (3 snippets) ===
  Snippet 1: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Enterprise edition product with advanced features id:E6SN0nII9-qpf1DSc6uW_asr3H93H7nQWe7wdOCDXxWjKC2BSD3Tcl40nDSaRvcz7c3ybL_yh5I949Do3wYSRg name:ekoDB Pro price:299] score:0.1111111111111111]
  Snippet 2: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Fully managed cloud database service product id:KWa8F9h1L45UX66lCps5Lg2i5dEKhZ4slef3CjhBwuHDOZZ27qgNI9rGZqiHm6OrWyOxTznAvS4II0jYVn94Kg name:ekoDB Cloud price:499] score:0.1111111111111111]
  Snippet 3: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:A high-performance database product with AI capabilities id:H4WQInGMwbO1MtupTLe0y6C2LYMrdPCAFWKSxqc0rarq5xhIXMvh_RVYFjLzuooYV_THscNSjKBepD7KkHU-og name:ekoDB price:99] score:0.1111111111111111]

Execution Time: 2261ms

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
✓ Created session: hJY7AqVnqrO5ejHnoGSQzoLqbLSKLFKWhkUDJ22FlPmMV_XXG26KHsqgb-aqZEGYbORbcyshRMJQS4-d4PJQdQ

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information provided, there is a product available named "ekoDB". It is a high-performance database product and it's priced at $99.

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
✓ Created second session: _XwItNsgyui8eFwQdJ6vlZ3UIvBQDyuKd_-KbvxG5BgvgtF-S9jyqIDFa40sGXoYzxX4EjtHa9ev55NghF-r5g
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
✓ Created session: GWN7n_LCJc5sI8y74tHllA5-oBLAoo9mdo1YYk47xcv89lSXIaEmVmie9ZzHja53uGONrfdAsiQZICuwn5wOhw

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is ekoDB, a high-performance database product. It is priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: VUbiKl13Nt443uznH_xfWVTFqNz-agb_eI0MwIt0mRh9dqUMdrUw5Rb9v95derAog2xL6aJOBK585RxEmQCHZQ
  Parent: GWN7n_LCJc5sI8y74tHllA5-oBLAoo9mdo1YYk47xcv89lSXIaEmVmie9ZzHja53uGONrfdAsiQZICuwn5wOhw

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: VUbiKl13Nt443uznH_xfWVTFqNz-agb_eI0MwIt0mRh9dqUMdrUw5Rb9v95derAog2xL6aJOBK585RxEmQCHZQ (Untitled)
  Session 2: GWN7n_LCJc5sI8y74tHllA5-oBLAoo9mdo1YYk47xcv89lSXIaEmVmie9ZzHja53uGONrfdAsiQZICuwn5wOhw (Untitled)
  Session 3: _XwItNsgyui8eFwQdJ6vlZ3UIvBQDyuKd_-KbvxG5BgvgtF-S9jyqIDFa40sGXoYzxX4EjtHa9ev55NghF-r5g (Untitled)
  Session 4: 5HlYZzepTIGvK8nKfiHDDimKGkZe_gVOvDe1xEdj-yE_eDBZBijc4ne1Q4SstWoYaWF52WMpcUodRD87Ef9tLw (Untitled)
  Session 5: 6AhoM-pLbXz0EkVBNngDDFBYloDHpeJWLOckv2klHQDr0Vo2qoDXVPNFCm0hoI-6JepnpIg60RTqwWMYpwc9dg (Untitled)
  Session 6: vBYgCKA7tCIFVCp4q8I18qr8arv5t1VHkvH8epnh6z8K-qtc4ACIAQ8D9tijYnjG5uegaQ52hu-xo1IcjJHWTw (Untitled)
  Session 7: VofpQp9wru7iR1LwyfXa3iLZcXzr9oQBFOB34zHu_xa9TrylGOYBgMtqMcdLtGSAcC0VP6elo5_67K8BE79w7g (Untitled)
  Session 8: vd5EO4q7AGtzgy6rMLWE_cVehblRhF5qYGcg6b_LzAk5HEuCM4Py_U3E7SJQDW_euOhnS4KcItAtKBXvLKkibg (Untitled)
  Session 9: p-ZCLCGlwHgVx28M8OWhlqK2IUXGNo3BEjAUu1504UE3P7KKhpdpHzzfIE8pn0ivZxDV81GWHmCLZzzHlEGMmg (Untitled)
  Session 10: 1C-L1tqnYk1eKUb3g3_TcFE6VIqGbd8bFPOUYLln1Uu4Fcu5rTWtTyI4UqGIYCE5ksDdntSQVJfrdS4sBBSpNg (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: VUbiKl13Nt443uznH_xfWVTFqNz-agb_eI0MwIt0mRh9dqUMdrUw5Rb9v95derAog2xL6aJOBK585RxEmQCHZQ

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Map Creation ===
✓ Created record with native map: map[id:q8tsc3Q0sa7kgjRW6vl9gNRJEZ-6x91vTrhT2jvxERJEJIsOf_mAdR81ou4QVMNNPrwechK8yveLYOR5yWyJoQ]

=== Upsert Operation ===
✓ First upsert (update): map[active:map[type:Boolean value:true] age:map[type:Integer value:29] email:map[type:String value:alice.j@newdomain.com] id:q8tsc3Q0sa7kgjRW6vl9gNRJEZ-6x91vTrhT2jvxERJEJIsOf_mAdR81ou4QVMNNPrwechK8yveLYOR5yWyJoQ name:map[type:String value:Alice Johnson]]
✓ Second upsert (insert): map[id:rdo0TXeAJJwumju-jcpCuwyL0yo5P_TFrXHznwH_Z9h6n1KTXZcoPff2tNKI63wA3HG1NPzlsBveAYxHUl9Hqg]

=== Find One Operation ===
✓ Found user by email: map[active:map[type:Boolean value:true] age:map[type:Integer value:29] email:map[type:String value:alice.j@newdomain.com] id:q8tsc3Q0sa7kgjRW6vl9gNRJEZ-6x91vTrhT2jvxERJEJIsOf_mAdR81ou4QVMNNPrwechK8yveLYOR5yWyJoQ name:map[type:String value:Alice Johnson]]
✓ User not found (as expected)

=== Exists Check ===
✓ Record exists: true
✓ Fake record exists: false (should be false)

=== Pagination ===
✓ Inserted 25 records for pagination
✓ Page 1: 10 records (expected 10)
✓ Page 2: 10 records (expected 10)
✓ Page 3: 7 records (expected ~7)

=== Cleanup ===
✓ Deleted collection

✅ All convenience methods demonstrated successfully!
Inserted with ripple: map[id:wVPl6NmuFWWZwE3yW3ZDg-cEZttWFpRYYzxyzwa3EdMiX2r0gtWVz6baF0wZ5BObiNYVAmWSD4awFJfq20RHaA]
Inserted with bypass_ripple: map[id:PUoFNGIhw7PJ2eQFTh-CQdCLXZYwpz6KZ6L09ZPXVAjg9pLkGYt_lyQd02iraSngSMa1p1lhokEK9MLwumm8og]
Inserted with TTL and bypass_ripple: map[id:RAX8s8Avybj2zetfKTCe6yfB5zqrzijX4dA372e_xm9EnNu4WW0tTPZJrqygBLOUd5Ht9SO9dQlOZk712Oz-xg]
Updated with bypass_ripple: map[id:wVPl6NmuFWWZwE3yW3ZDg-cEZttWFpRYYzxyzwa3EdMiX2r0gtWVz6baF0wZ5BObiNYVAmWSD4awFJfq20RHaA name:map[type:String value:Product 1] price:map[type:Integer value:150]]
Deleted with bypass_ripple
Batch inserted with bypass_ripple: 2 records
Upserted with bypass_ripple: map[id:luRJH45qksqtBNrFfkutZHwDexMpB5DJYQ1XkwqhnV2GNai2EVEAGONf7lR82dBFtuHWqqrabvl6mZqHU6A-4w]
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields
Fetched 3 users with only 3 fields each

Example 2: Exclude sensitive fields
Fetched 2 admins without sensitive data
  Password field excluded: false

Example 3: Complex query with projection
Fetched 3 active users with profile fields

Example 4: Find by ID with projection
Fetched user profile: Alice Johnson

Example 5: Compare full vs projected data
Full query returned 12 fields per user
Projected query returned 12 fields per user

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
=== KV Precision: Float vs Decimal ===

=== Test 1: Using Go Floats (LOSES PRECISION) ===
✓ Stored products with float prices

Retrieved float prices:
  Widget A: $29.99 (expected $29.99) ✓
  Widget B: $39.99 (expected $39.99) ✓
  Widget C: $49.99 (expected $49.99) ✓

=== Test 2: Using FieldDecimal() (PRESERVES PRECISION) ===
✓ Stored products with decimal prices

Retrieved decimal prices:
  Widget A: $29.99 (expected $29.99) ✓
  Widget B: $39.99 (expected $39.99) ✓
  Widget C: $49.99 (expected $49.99) ✓

=== Test 3: Sum Calculation Comparison ===
  Float sum: $119.97 (expected $119.97)
  Decimal sum: $119.97 (expected $119.97)

=== Test 4: Extreme Precision Example ===
  Float 0.1 + 0.2 = 0.3 (should be 0.3)
  Decimal "0.30" = 0.30 (exact!)

=== Cleanup ===
✓ Cleaned up test keys

=== Summary ===
✅ Use FieldDecimal() for monetary values, percentages, and
   any case where floating-point errors are unacceptable.
✅ FieldDecimal() stores values as strings internally,
   preserving exact precision across all operations.
✅ [32mGo client examples complete![0m
