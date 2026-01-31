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
Inserted: map[id:lLHx5c8cvQIFYnHOyNhdSQpTAkAzJy4x7ov0LYqusYIHEZZvh6Y3BCyefvEK27_ZHw3kDBRj7aK37-q5BQ7KNA]

=== Find by ID ===
Found: map[active:true id:lLHx5c8cvQIFYnHOyNhdSQpTAkAzJy4x7ov0LYqusYIHEZZvh6Y3BCyefvEK27_ZHw3kDBRj7aK37-q5BQ7KNA name:Test Record value:42]

=== Find with Query ===
Found documents: [map[active:map[type:Boolean value:true] id:lLHx5c8cvQIFYnHOyNhdSQpTAkAzJy4x7ov0LYqusYIHEZZvh6Y3BCyefvEK27_ZHw3kDBRj7aK37-q5BQ7KNA name:map[type:String value:Test Record] value:map[type:Integer value:42]]]

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] id:lLHx5c8cvQIFYnHOyNhdSQpTAkAzJy4x7ov0LYqusYIHEZZvh6Y3BCyefvEK27_ZHw3kDBRj7aK37-q5BQ7KNA name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
[32m✓ simple_crud.go completed successfully[0m
[34m
=== Running simple_websocket.go ===[0m
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: NIBJ21jtUelEp3g_m7rMhyUHgt9uWYX6SJNxAPo9eSmRjgGCWthulWkYUu0t1Fkm8e3TYIwv0ZOWp0E8q4LuIg

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
✓ Retrieved 3 record(s) via WebSocket

✓ WebSocket example completed successfully
[32m✓ simple_websocket.go completed successfully[0m
[34m
=== Running http_functions.go ===[0m
🚀 ekoDB Scripts Example (Go/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: yMwhfPeOTuPnUp_-utuUL-T1XfJe_xd3D42joeIJPAjPs0Kk4ghkINSYpIR8RPKQxRqPfPqN7CmZL-C64LHSIA
📊 Found 25 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: l4WXmQyBiwFCiii-TtTdDz_tcOum8BH8zkNJoFhJmQJeroFcFEEaT-Seg6BjmvNLSr1k1lJq31oz2FzduoKjvw
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: xlXaynf_uHvgAunYKCx_h3QyBFhn_XF7F_lI9mmb39IfTWBgi9RlbEucpT_6uGue7-RuSiO4SPLi4llJAsLr5Q
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
Collection created with first record: 1QmRJHcFS7Wzv__Mw658i4bwqczBghTlxeMNTmGPzxPJBKV1j0bI6puOOnNCrlhofTIz8AUY45BQENbk1b7qQg

=== List Collections ===
Total collections: 16
Sample collections: [test_collection demo_collection schema_employees_client_py chat_messages__ek0_testing schema_users_client_py]

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
Created Alice: $1000 - ID: t1EG1mm1AOXlOXlH_aZ8nL9tyEK3PZABoBXF8Sul8o2UqRKzGwGLg37aDNpz__TVhVS0Bd2LrVdV7eW59UzVKQ
Created Bob: $500 - ID: eq2bWqh12zADWbaxqCX3VcBC5RsbU-1Pjuj3wTMO7oQXQ3JhPN8CwbnsZHSP5DCQ5T0RAgBKFgaTZtWRo7EldA

=== Example 1: Begin Transaction ===
Transaction ID: 3622d816-5318-4fee-81ff-909501463c25

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
New transaction: 7e927fea-2190-4eee-b0d2-3837c6470d16
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
   ✅ Script saved: BB10CFzIBalAEfEZgnSby5H43cIwzlSTsE4Bltzeagsztv5nhTEEGhF-jWGgm1XvkYeGMfd0ESXB9vq1pqbyWQ

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: BqAetFPicAHXgoGJvyaEWdcFHxHo8tXvslF_iTueF9k-3ZZa6do-N3aewdHCbCYX4d5jK792a0lXSdjjcI8B1Q
   📋 Name: map[type:String value:Alice Smith]
   📋 Email: map[type:String value:alice@example.com]
   📋 Status: map[type:String value:pending]
   📋 Credits: map[type:Integer value:0]

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: kCT9axGDSGZtasj4iMMGhD3iZn2g-lToxJRMiCL_v7flJOyvFn5PJRtvoKL01mUhoDbQ1y_ML2qsP-3draNGew

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
   ✅ Script saved: I0uaOiiEpOIunayiFUP-C1RMd9D5COBVQ52OFSru8xd08gg1B8fELkyWLVFbd2iIABkkp691bsev4groDu3dvQ

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
   ✅ Script saved: _InyJkh9jb_9AZg6tImSqt-9GOjGlM_HS_VedciRekfqqL8d3ej8JIp3yQIW-mlw2HLe0p-fXMdds0B5gyxpRA

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: BB10CFzIBalAEfEZgnSb...
   ✅ Deleted script: kCT9axGDSGZtasj4iMMG...
   ✅ Deleted script: I0uaOiiEpOIunayiFUP-...
   ✅ Deleted script: _InyJkh9jb_9AZg6tImS...
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
  Output: Document ID = HgVtZmXgmQJDaFl1m-YGBSllxr6n7glUyF7AWLQwHS5OkiXgMipSk5U7sgukEGGh8VdxKSptJ-Z_sj7JvTKCPw
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: FindByID(HgVtZmXgmQJDaFl1m-YGBSllxr6n7glUyF7AWLQwHS5OkiXgMipSk5U7sgukEGGh8VdxKSptJ-Z_sj7JvTKCPw)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: FindByID(HgVtZmXgmQJDaFl1m-YGBSllxr6n7glUyF7AWLQwHS5OkiXgMipSk5U7sgukEGGh8VdxKSptJ-Z_sj7JvTKCPw)
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
  Output: Document ID = DeOqlYU3UcrrOOIDDGzqatAhdIgpd_EHesoQn7oy1sUmRe4L2fuXYAA5hsIeVGr6TDWvRKn_hBvdwRKOBBkGfw
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: FindByID(DeOqlYU3UcrrOOIDDGzqatAhdIgpd_EHesoQn7oy1sUmRe4L2fuXYAA5hsIeVGr6TDWvRKn_hBvdwRKOBBkGfw)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: FindByID(DeOqlYU3UcrrOOIDDGzqatAhdIgpd_EHesoQn7oy1sUmRe4L2fuXYAA5hsIeVGr6TDWvRKn_hBvdwRKOBBkGfw)
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
Inserted: map[id:9lcnzyrQw2BA6gGlMkiVvxTLomTZ5BeKjITyiaBeM7OIUN628cEgpuKmv-MT5CQlG7SnVDSZEdR3wRi9acp1Mg]

=== Find by ID ===
Found: map[active:true categories:[electronics computers] created_at:2026-01-28T09:43:38-05:00 data:aGVsbG8gd29ybGQ= embedding:[0.1 0.2 0.3 0.4 0.5] id:9lcnzyrQw2BA6gGlMkiVvxTLomTZ5BeKjITyiaBeM7OIUN628cEgpuKmv-MT5CQlG7SnVDSZEdR3wRi9acp1Mg metadata:map[key:value nested:map[deep:true]] name:Test Record price:99.99 tags:[tag1 tag2 tag3] user_id:550e8400-e29b-41d4-a716-446655440000 value:42]

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 0
  active (Boolean): true
  price (Decimal): 99.990000
  created_at (DateTime): 2026-01-28 09:43:38 -0500 EST
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1 tag2 tag3]
  metadata (Object): map[key:value nested:map[deep:true]]
  embedding (Vector): [0.1 0.2 0.3 0.4 0.5]
  categories (Set): [electronics computers]
  data (Bytes): 11 bytes
Plain record: map[active:true categories:[electronics computers] created_at:2026-01-28T09:43:38-05:00 data:aGVsbG8gd29ybGQ= embedding:[0.1 0.2 0.3 0.4 0.5] id:9lcnzyrQw2BA6gGlMkiVvxTLomTZ5BeKjITyiaBeM7OIUN628cEgpuKmv-MT5CQlG7SnVDSZEdR3wRi9acp1Mg metadata:map[key:value nested:map[deep:true]] name:Test Record price:99.99 tags:[tag1 tag2 tag3] user_id:550e8400-e29b-41d4-a716-446655440000 value:42]

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-01-28T14:43:38Z] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:9lcnzyrQw2BA6gGlMkiVvxTLomTZ5BeKjITyiaBeM7OIUN628cEgpuKmv-MT5CQlG7SnVDSZEdR3wRi9acp1Mg metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Updated Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: N9IzhBvqeJ76sB97uFOD4UFPVSrm3sYRg-clvURNx4G-tk8McrnXrF7HnAndkca9QwZzW0NaZydvQRta_Xl0Iw

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
Collection created with first record: -O3z0S-5w_g9yzBL_NV7EUEtFlXQCYtEiD1MGZ-tQivDaThNhBOlZuCFxkl4p-qXGfXB7N65uYT9yPTlxzOThQ

=== List Collections ===
Total collections: 15
Sample collections: [test_collection schema_employees_client_py chat_messages__ek0_testing schema_users_client_py test_accounts]

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
Created Alice: $1000 - ID: 9cOmynpdTgwFh8I4RFLSyLzadGCVVuTjhNsSS5nC60tFNFqibxiW0wFZBp_uNKRufdiwV_gygKlSQV2fNo_o6w
Created Bob: $500 - ID: a15QCmBMUe-vEtprm_M63OU3HCxAKGjY1YgwlYriRDSQc6h7uLKi3UwC_m8d0Asw3OJ7s2ZQJ0JL8HtVOCyMUQ

=== Example 1: Begin Transaction ===
Transaction ID: 78b14bb5-9a62-48f0-bd19-c637c07b7e91

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: ef6dc54a-68f0-40aa-9203-3160ea431914
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
  1. Score: 0.768
  2. Score: 0.759
  3. Score: 0.740

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.704
  2. Score: 1.496
  3. Score: 0.307

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
  - Alice Johnson: Engineering
  - Bob Smith: Sales

2. Join with filtering:
Found 1 users in Engineering
  - Alice Johnson: Building A

3. Join with user profiles:
Found 2 users with profile data
  - Alice Johnson: Senior Software Engineer
  - Bob Smith: Sales Manager

4. Join orders with user data:
Found 2 completed orders
  - Laptop ($0) by Alice Johnson
  - Mouse ($0) by Alice Johnson

5. Complex join with multiple conditions:
Found 2 users with example.com emails
  - Alice Johnson (alice@example.com): Building A
  - Bob Smith (bob@example.com): Building B

=== Cleanup ===
✅ Deleted test collections

✅ Join operations examples completed!
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: 4rnO0uJ4NUUCw59dPiAp4t1paQSBmw7GdBK_PXSBJVYvDN6Hek_8SMAAtN9pJ-wbBnR_vKZQOuKAHa9f_AGDMw

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: ya3DwxQczMS2BhjAoa4ZEeD9qzNxIBc-q8rUTYQRan7lO49Za33m_JZD8zPeUfGpztjZAnOtqKDHDOogy_6_qw

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
✓ Inserted document with TTL: AMun_AP-XwwKua0TNuZmrCxOXw0nR9t7xOdq67gzWKRkxPaMoJfsgstr_dgxtJPNAEw3fMG5G28wKZ6TRg2M3A

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
✓ Edge cache script created: LTpyeRvZ6-hoP1rSsZCbVY5kl7rd-tupK_aGGrMzL01fnurtDFdzYvHKS-D6XNX_lTT9DooGiEeCOjgXVwAhbQ

Call 1: Cache lookup
Response time: 1ms
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

✅ Script saved: Uup9QmMvOZaEVY7UFYQTzMZ3hs73Ed_NiSY5AGk3BqD_m-ME_Z5dOrbBd0_mKSpoM8sZQlguigSkDGEh9aeSaQ
📊 Found 10 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved
📊 Found 10 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved
📊 Statistics: 2 groups
   map[avg_score:60 count:5 status:active]
   map[avg_score:50 count:5 status:inactive]
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
   ⏱️  Duration: 1.436958ms
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
   ⏱️  Duration: 1.5545ms
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

✅ Script saved: CLOvr4bbgpImejMxfSjkcI8dlPyivC8_pDDYa5QuwG5h8yRioq31Zm1kqn305iYBf451bxtaYbfdP6qs8bQQ8A
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

✅ Inserted order: WSgdMmMUe9sP3kCCniZ8JuzQX9rwKNZTgWEUkDaM_uXci5SG2h7svrUOve0ZdU3RGBRnqTt4MEHuRIemZDoiMA
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: g4MnC5v20X7luE2hQS0nSAf7p5HXWtJJo-zAo1zc_6-57WcT1vyDFL2JycAz9X8YHW2nm5NLF1RWVu6a1Mb7AA
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: map[role:admin userId:user_abc]
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session
📝 Example 4: KV Operations in Scripts

✅ Script saved: t2_erxLQwy2ahDx_dm_5fUj8Ze_o5gQFNr6sCklMQFaAGJc4S4CBVOaaGsFTL0T4DGkeNs0i78Af3FJyRm3OXg
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Script

✅ Script saved: 9pPUojAas7YTHtNETBHkLVJNSy1BSNqR3vtVcN-iMhwstR0NEHa0PYRYzWaCcHZNuIRxkDuQDVK5cWbQxlV-pQ
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
✓ Created SWR script: fetch_api_user_go (dSVEvQNTtG_bcQlzOp7EhG5BK1G6ZsZNzOPADdvAjdfrpvd6E0MNlNSm2neUhAjXEcSvUaj39NHep5mULZpSMA)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "cached_at": "2026-01-28T09:43:40-05:00",
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
      "id": "OoRF6nt8mfFjXFYU0Qj8tJzBEvFcIpRk8oYpqaIGAo3X-YWOQ8eSFvPqnycfx-ZjIkK2IgK1vyIcccaDb9qEBA"
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
Response time: 65ms (served from cache)
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
✓ Created native SWR script: github_user_native (z6xUuMLTl525I_kVF_vDWnKYcJCWG_WfEJUvU5Y8zwVAK7fSH1kkeRbmsmp-60ECJ3om20QGGxUCQBAIYSle1Q)

First call (cache miss - will fetch from GitHub API):
  Response time: 1ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 1ms
  Speedup: 1.0x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (RiB_nRAgmNaEa4NfCoiWqYsBZCls_Zat3wlvf4NT4aPlHac9tgrpeW3N8Ulj8YupqevL48mfI8wX7n8Duwn3Zw)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (LVJacdnO--3y69e6xp4dYeeT3s7aBTI8F6vCnuWLvuYLwlkm5Z2MEZBY3XwvsIFSRc37s8J5iEbAEP8PbleZdw)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (Gl6tpGhcL7VkgKMK9jhX5T6WB659OX4n6aufJxU90jIwh1U0LTGbnrfWbATpNeTJbd1BQmD-8zrV_rUEqN2VoA)
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
   1. map[type:String value:Database Design Principles] (map[type:String value:Database])
   2. map[type:String value:Natural Language Processing] (map[type:String value:AI])
   3. map[type:String value:Getting Started with ekoDB] (map[type:String value:Database])
   4. map[type:String value:Vector Databases Explained] (map[type:String value:Database])
   5. map[type:String value:Introduction to Machine Learning] (map[type:String value:AI])
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
✓ Created session: Laqlb5tCJrvEV9bzczXZAW85eaZP17Cwy2HXexrLvLAifTB6Psy2cwD0WnEoNgtIgHdpP3Jb-P94S0MsoRE-UA

=== Sending Chat Message ===
Message ID: NQvTAxsCgMwIt1B3cbW2B9UkyONhYRcRalg9xC9YYKx9gWRrrrkuLY1m9zOHOLh3G3XqVw2U-IcbExGeOZIwag

=== AI Response ===
There are three products available:

1. ekoDB Cloud: This is a fully managed cloud database service product. The price for this product is 499.

2. ekoDB Pro: This is an enterprise edition product with advanced features. The price for this product is 299.

3. ekoDB: This is a high-performance database product with AI capabilities. The price for this product is 99.

=== Context Used (3 snippets) ===
  Snippet 1: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Fully managed cloud database service product id:_EqgC4q1SBIUQGOK6FZFr6nJW1QjhJfWfJIDSuzN4wJc15a501ele-cu6EFLpdJYZLlcSVKjTE7EEsBhY7twFA name:ekoDB Cloud price:499] score:0.1111111111111111]
  Snippet 2: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Enterprise edition product with advanced features id:Xu_Ae4BmR3D4lZBAv67Xonc8bmlV6aUyUelZzV_UgDxRFqS1Uxw8JqBcQByzHSykn-b2ZTcmP1WWbMBXFKDz_Q name:ekoDB Pro price:299] score:0.1111111111111111]
  Snippet 3: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:A high-performance database product with AI capabilities id:xUkg_SUFN8wcO92IlPsq1D3jCW8S_b6N0KqlK63TlL125IaU1mwecz1wtTExJxd65rM3fIpXBL64nUSPyTWM_g name:ekoDB price:99] score:0.1111111111111111]

Execution Time: 3108ms

=== Token Usage ===
Prompt tokens: 457
Completion tokens: 84
Total tokens: 541

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 5SFpRQ53wY8TG2Jps227_R1AogWpCUqgAvrLN96B-tfboA8UnH5-eBjg3OLhzSRNQpC0-mQA7b1AXS1yrguflQ

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information I have, there is a product available named ekoDB. It's a high-performance database product priced at $99.

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
✓ Created second session: 42HtILejTOAtLG-rbpmagdu_I4oYxQIwSEEJMpClKJ4GPCTHbTL1eyQ6kk0UjMjmyOttAupCiyTkqM_E7rjXLg
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
✓ Created session: qwggQOSdymEFPxI-CDWMeoPhlfjoc_c1TSR2kf9tVaz5J364voZ9PmYnY_vM7b2H8YbjJrJUPoClKCM7UXlxKQ

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information available, there is a product named "ekoDB". It is a high-performance database product and it is priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: ZAScu74LZ683m2BMwnSpddXqSsRW2AxrPwc6f6AQHwNggYSBbHnZpQi34zhf-fX1seCWYbmkQUHfgi0psPjoPQ
  Parent: qwggQOSdymEFPxI-CDWMeoPhlfjoc_c1TSR2kf9tVaz5J364voZ9PmYnY_vM7b2H8YbjJrJUPoClKCM7UXlxKQ

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: ZAScu74LZ683m2BMwnSpddXqSsRW2AxrPwc6f6AQHwNggYSBbHnZpQi34zhf-fX1seCWYbmkQUHfgi0psPjoPQ (Untitled)
  Session 2: qwggQOSdymEFPxI-CDWMeoPhlfjoc_c1TSR2kf9tVaz5J364voZ9PmYnY_vM7b2H8YbjJrJUPoClKCM7UXlxKQ (Untitled)
  Session 3: 42HtILejTOAtLG-rbpmagdu_I4oYxQIwSEEJMpClKJ4GPCTHbTL1eyQ6kk0UjMjmyOttAupCiyTkqM_E7rjXLg (Untitled)
  Session 4: Laqlb5tCJrvEV9bzczXZAW85eaZP17Cwy2HXexrLvLAifTB6Psy2cwD0WnEoNgtIgHdpP3Jb-P94S0MsoRE-UA (Untitled)
  Session 5: sPGFz1pxKMHqtytbF2S3VC_9uARlLrXyILW09b_AhA0I0x5Y8cEGwrMkL4N2OrUftkAzOXJDGpEE9toFKkvkaw (Untitled)
  Session 6: ZH-TQGGOgKc2eAuwfdVl88A3nXpv_Tl1Nr5J8T1NOkKYel4TOeJPIYe6gKx9IFSkDGJC33fac-ZKKorWwnc4lg (Untitled)
  Session 7: sahPJix1CZjQI-8zJFD156k6RFBRsC-UV_3q3KGgtuWemXtIe1cBz22bzC8zpB9-49pi4S9dDYXF-Xl7wTTVWg (Untitled)
  Session 8: xyr5atcONAmY5b1CaBCT-SREXJ7oTLuhABQcN-smysHOm58aSEPqEZNkqsGJDwxG-Fkv9YCQvgkFZE4YWE5amw (Untitled)
  Session 9: rMNvVHF92_i7TijhVx4WnS4dvCmtzBovgjZTRYvcIN0B7CvdN7DWvV5EaABdJY8yCI4jBkpW6NQ8ZgVIrEc6Ig (Untitled)
  Session 10: 3SpDq19pY2Ui9N9-tOtcvr4WAkrxDsWBFE3DCYNzdL6SrvNVZClJpKRk7pXCOHa5JXLXKWb2vS9fd-CNweAnjw (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: ZAScu74LZ683m2BMwnSpddXqSsRW2AxrPwc6f6AQHwNggYSBbHnZpQi34zhf-fX1seCWYbmkQUHfgi0psPjoPQ

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Map Creation ===
✓ Created record with native map: map[id:YKiT8Iknxs5v--HxjM6cAijBafaFCkYcG-7ixKqIkARLo_MdS8z9_W_GT1MSvfxxOS6JupaIdcDdQPner0ykgQ]

=== Upsert Operation ===
✓ First upsert (update): map[active:map[type:Boolean value:true] age:map[type:Integer value:29] email:map[type:String value:alice.j@newdomain.com] id:YKiT8Iknxs5v--HxjM6cAijBafaFCkYcG-7ixKqIkARLo_MdS8z9_W_GT1MSvfxxOS6JupaIdcDdQPner0ykgQ name:map[type:String value:Alice Johnson]]
✓ Second upsert (insert): map[id:HQ7V1L_BO-vaJYoiphCR_R2oBxopl-LLt82H5-huLqnM3_E73hxy8SxQnlntNsln2h8_AVaS4jZNUKHlabgR9Q]

=== Find One Operation ===
✓ Found user by email: map[active:map[type:Boolean value:true] age:map[type:Integer value:29] email:map[type:String value:alice.j@newdomain.com] id:YKiT8Iknxs5v--HxjM6cAijBafaFCkYcG-7ixKqIkARLo_MdS8z9_W_GT1MSvfxxOS6JupaIdcDdQPner0ykgQ name:map[type:String value:Alice Johnson]]
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
Inserted with ripple: map[id:wMmmFptekqD7G9BGVeF3P-Tu3jL_wMT03KRmHHu0W5dcxDWITDsQIlTRpTc4efFwniKHvBL9KiWlwNqbyIdSrA]
Inserted with bypass_ripple: map[id:N3Z7rMyadR-1GEdPVZAEniQsFV5uf16AB9pQdAhcsH4_VdAubcHss-BaNY4Pfs9bOdPlOInkpbaTd7CGRwvg9w]
Inserted with TTL and bypass_ripple: map[id:xySngQejGtWp97fl0V6gJb0EVn9XjgY4PWXs9tvQKLayPUo1wvnt5xn2pDSePo-64-nbPLlN7ZKBmxsLC73lrQ]
Updated with bypass_ripple: map[id:wMmmFptekqD7G9BGVeF3P-Tu3jL_wMT03KRmHHu0W5dcxDWITDsQIlTRpTc4efFwniKHvBL9KiWlwNqbyIdSrA name:map[type:String value:Product 1] price:map[type:Integer value:150]]
Deleted with bypass_ripple
Batch inserted with bypass_ripple: 2 records
Upserted with bypass_ripple: map[id:q3ZI5PWe9oAQczusqhs_vIRLrk7AmspDbWEwD1W7QhUQMzi8EV6ud1IGRSaCUG5Iaf9_VJAtOAnWBXJGtfQt9Q]
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields (id, name, email only)
  Found 3 active users
  Fields returned: [email id name]
  First user: Dave Brown <dave@example.com>

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: [age avatar_url bio created_at email id name status user_role]

Example 3: Complex query with projection (active users, ages 18-65)
  Found 3 active users (ages 18-65)
    - Dave Brown (age 0)
    - Alice Johnson (age 0)
    - Bob Smith (age 0)

Example 4: Query inactive users with profile fields
  Found 1 inactive users
    - Carol White: Manager

Example 5: Compare full vs projected data
  Full query:
    - 12 fields per record
    - Fields: [age api_key avatar_url bio created_at email id name password secret_token status user_role]
  Projected query:
    - 3 fields per record
    - Fields: [email id name]
  Bandwidth savings: ~75% fewer fields

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
  Float 0.1 + 0.2 = 0.29999999999999999 (should be 0.3)
  Decimal "0.30" = 0.30 (exact!)

=== Cleanup ===
✓ Cleaned up test keys

=== Summary ===
✅ Use FieldDecimal() for monetary values, percentages, and
   any case where floating-point errors are unacceptable.
✅ FieldDecimal() stores values as strings internally,
   preserving exact precision across all operations.
✓ Client created

=== Get All Chat Models ===
OpenAI models: [gpt-4-0613 gpt-4 gpt-3.5-turbo gpt-5.2-codex gpt-4o-mini-tts-2025-12-15 gpt-realtime-mini-2025-12-15 gpt-audio-mini-2025-12-15 chatgpt-image-latest davinci-002 babbage-002 gpt-3.5-turbo-instruct gpt-3.5-turbo-instruct-0914 dall-e-3 dall-e-2 gpt-4-1106-preview gpt-3.5-turbo-1106 tts-1-hd tts-1-1106 tts-1-hd-1106 text-embedding-3-small text-embedding-3-large gpt-4-0125-preview gpt-4-turbo-preview gpt-3.5-turbo-0125 gpt-4-turbo gpt-4-turbo-2024-04-09 gpt-4o gpt-4o-2024-05-13 gpt-4o-mini-2024-07-18 gpt-4o-mini gpt-4o-2024-08-06 chatgpt-4o-latest gpt-4o-audio-preview gpt-4o-realtime-preview omni-moderation-latest omni-moderation-2024-09-26 gpt-4o-realtime-preview-2024-12-17 gpt-4o-audio-preview-2024-12-17 gpt-4o-mini-realtime-preview-2024-12-17 gpt-4o-mini-audio-preview-2024-12-17 o1-2024-12-17 o1 gpt-4o-mini-realtime-preview gpt-4o-mini-audio-preview o3-mini o3-mini-2025-01-31 gpt-4o-2024-11-20 gpt-4o-search-preview-2025-03-11 gpt-4o-search-preview gpt-4o-mini-search-preview-2025-03-11 gpt-4o-mini-search-preview gpt-4o-transcribe gpt-4o-mini-transcribe o1-pro-2025-03-19 o1-pro gpt-4o-mini-tts o3-2025-04-16 o4-mini-2025-04-16 o3 o4-mini gpt-4.1-2025-04-14 gpt-4.1 gpt-4.1-mini-2025-04-14 gpt-4.1-mini gpt-4.1-nano-2025-04-14 gpt-4.1-nano gpt-image-1 codex-mini-latest gpt-4o-realtime-preview-2025-06-03 gpt-4o-audio-preview-2025-06-03 o4-mini-deep-research gpt-4o-transcribe-diarize o4-mini-deep-research-2025-06-26 gpt-5-chat-latest gpt-5-2025-08-07 gpt-5 gpt-5-mini-2025-08-07 gpt-5-mini gpt-5-nano-2025-08-07 gpt-5-nano gpt-audio-2025-08-28 gpt-realtime gpt-realtime-2025-08-28 gpt-audio gpt-5-codex gpt-image-1-mini gpt-5-pro-2025-10-06 gpt-5-pro gpt-audio-mini gpt-audio-mini-2025-10-06 gpt-5-search-api gpt-realtime-mini gpt-realtime-mini-2025-10-06 sora-2 sora-2-pro gpt-5-search-api-2025-10-14 gpt-5.1-chat-latest gpt-5.1-2025-11-13 gpt-5.1 gpt-5.1-codex gpt-5.1-codex-mini gpt-5.1-codex-max gpt-image-1.5 gpt-5.2-2025-12-11 gpt-5.2 gpt-5.2-pro-2025-12-11 gpt-5.2-pro gpt-5.2-chat-latest gpt-4o-mini-transcribe-2025-12-15 gpt-4o-mini-transcribe-2025-03-20 gpt-4o-mini-tts-2025-03-20 gpt-3.5-turbo-16k tts-1 whisper-1 text-embedding-ada-002]
Anthropic models: [claude-opus-4-5-20251101 claude-haiku-4-5-20251001 claude-sonnet-4-5-20250929 claude-opus-4-1-20250805 claude-opus-4-20250514 claude-sonnet-4-20250514 claude-3-7-sonnet-20250219 claude-3-5-haiku-20241022 claude-3-haiku-20240307]
Perplexity models: [sonar sonar-pro sonar-deep-research sonar-reasoning sonar-reasoning-pro]

=== Get OpenAI Models ===
OpenAI models: [gpt-4-0613 gpt-4 gpt-3.5-turbo gpt-5.2-codex gpt-4o-mini-tts-2025-12-15 gpt-realtime-mini-2025-12-15 gpt-audio-mini-2025-12-15 chatgpt-image-latest davinci-002 babbage-002 gpt-3.5-turbo-instruct gpt-3.5-turbo-instruct-0914 dall-e-3 dall-e-2 gpt-4-1106-preview gpt-3.5-turbo-1106 tts-1-hd tts-1-1106 tts-1-hd-1106 text-embedding-3-small text-embedding-3-large gpt-4-0125-preview gpt-4-turbo-preview gpt-3.5-turbo-0125 gpt-4-turbo gpt-4-turbo-2024-04-09 gpt-4o gpt-4o-2024-05-13 gpt-4o-mini-2024-07-18 gpt-4o-mini gpt-4o-2024-08-06 chatgpt-4o-latest gpt-4o-audio-preview gpt-4o-realtime-preview omni-moderation-latest omni-moderation-2024-09-26 gpt-4o-realtime-preview-2024-12-17 gpt-4o-audio-preview-2024-12-17 gpt-4o-mini-realtime-preview-2024-12-17 gpt-4o-mini-audio-preview-2024-12-17 o1-2024-12-17 o1 gpt-4o-mini-realtime-preview gpt-4o-mini-audio-preview o3-mini o3-mini-2025-01-31 gpt-4o-2024-11-20 gpt-4o-search-preview-2025-03-11 gpt-4o-search-preview gpt-4o-mini-search-preview-2025-03-11 gpt-4o-mini-search-preview gpt-4o-transcribe gpt-4o-mini-transcribe o1-pro-2025-03-19 o1-pro gpt-4o-mini-tts o3-2025-04-16 o4-mini-2025-04-16 o3 o4-mini gpt-4.1-2025-04-14 gpt-4.1 gpt-4.1-mini-2025-04-14 gpt-4.1-mini gpt-4.1-nano-2025-04-14 gpt-4.1-nano gpt-image-1 codex-mini-latest gpt-4o-realtime-preview-2025-06-03 gpt-4o-audio-preview-2025-06-03 o4-mini-deep-research gpt-4o-transcribe-diarize o4-mini-deep-research-2025-06-26 gpt-5-chat-latest gpt-5-2025-08-07 gpt-5 gpt-5-mini-2025-08-07 gpt-5-mini gpt-5-nano-2025-08-07 gpt-5-nano gpt-audio-2025-08-28 gpt-realtime gpt-realtime-2025-08-28 gpt-audio gpt-5-codex gpt-image-1-mini gpt-5-pro-2025-10-06 gpt-5-pro gpt-audio-mini gpt-audio-mini-2025-10-06 gpt-5-search-api gpt-realtime-mini gpt-realtime-mini-2025-10-06 sora-2 sora-2-pro gpt-5-search-api-2025-10-14 gpt-5.1-chat-latest gpt-5.1-2025-11-13 gpt-5.1 gpt-5.1-codex gpt-5.1-codex-mini gpt-5.1-codex-max gpt-image-1.5 gpt-5.2-2025-12-11 gpt-5.2 gpt-5.2-pro-2025-12-11 gpt-5.2-pro gpt-5.2-chat-latest gpt-4o-mini-transcribe-2025-12-15 gpt-4o-mini-transcribe-2025-03-20 gpt-4o-mini-tts-2025-03-20 gpt-3.5-turbo-16k tts-1 whisper-1 text-embedding-ada-002]

=== Get Anthropic Models ===
Anthropic models: [claude-opus-4-5-20251101 claude-haiku-4-5-20251001 claude-sonnet-4-5-20250929 claude-opus-4-1-20250805 claude-opus-4-20250514 claude-sonnet-4-20250514 claude-3-7-sonnet-20250219 claude-3-5-haiku-20241022 claude-3-haiku-20240307]

✓ Chat Models API example complete
✓ Client created

=== Create User Function ===
Created user function with ID: omejYbVS7J2IW14dJUbYkT0tY-hAB_PtP-gp9pnWvVZSDdWjRShfml8aDuAmskGszjLCOOeZmwRPVpqfpoWAoQ

=== Get User Function ===
Retrieved: get_active_users - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 19 user functions:
  - fetch_user: Fetch user by code
  - swr_user: SWR pattern for user data (KV-based)
  - validate_user: Check if user exists
  - get_user_wrapper: Wrapper that calls fetch_user
  - fetch_user: Fetch user by code
  - fetch_and_store_user: Fetch user from API and cache in KV
  - validate_user: Check if user exists
  - fetch_slim_user: Validate and slim down user
  - fetch_and_store_user: Fetch user from API and cache in KV
  - swr_user: SWR pattern for user data (KV-based)
  - get_active_users: Get Active Users
  - fetch_slim_user: Validate and slim down user
  - swr_user: SWR pattern for user data (KV-based)
  - fetch_and_store_user: Fetch user from API and cache in KV
  - get_verified_user: Get verified and validated user
  - validate_user: Check if user exists
  - fetch_slim_user: Validate and slim down user
  - fetch_user: Fetch user by code
  - cache_api_call_py: Cache External API Call

=== List User Functions by Tag ===
Found 19 user functions with 'users' tag:
  - fetch_user
  - swr_user
  - validate_user
  - get_user_wrapper
  - fetch_user
  - fetch_and_store_user
  - validate_user
  - fetch_slim_user
  - fetch_and_store_user
  - swr_user
  - get_active_users
  - fetch_slim_user
  - swr_user
  - fetch_and_store_user
  - get_verified_user
  - validate_user
  - fetch_slim_user
  - fetch_user
  - cache_api_call_py

=== Update User Function ===
User function updated successfully

=== Delete User Function ===
User function deleted successfully

✓ User Functions API example complete
✓ Client created

=== Check Collection Exists (Before Creation) ===
Collection 'collection_utils_test' exists: false

=== Creating Test Documents ===
Created 5 test documents

=== Check Collection Exists (After Creation) ===
Collection 'collection_utils_test' exists: true

=== Count Documents ===
Document count in 'collection_utils_test': 5

=== Check Non-Existent Collection ===
Collection 'nonexistent_collection_xyz' exists: false

=== Cleanup ===
Deleted collection 'collection_utils_test'

✓ Collection Utilities example complete
✅ [32mGo client examples complete![0m
