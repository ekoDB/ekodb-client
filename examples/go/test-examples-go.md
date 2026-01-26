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
Inserted: map[id:mXFRuDRqg-2viK32WP5eUdY_d6DMrxWz8ue_qYWJvji6QU4eOFJSJLE1pYkk67BGCeIAXCwK8qYr4D7c9JUFWw]

=== Find by ID ===
Found: map[active:true id:mXFRuDRqg-2viK32WP5eUdY_d6DMrxWz8ue_qYWJvji6QU4eOFJSJLE1pYkk67BGCeIAXCwK8qYr4D7c9JUFWw name:Test Record value:42]

=== Find with Query ===
Found documents: map[]

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] id:mXFRuDRqg-2viK32WP5eUdY_d6DMrxWz8ue_qYWJvji6QU4eOFJSJLE1pYkk67BGCeIAXCwK8qYr4D7c9JUFWw name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
[32m✓ simple_crud.go completed successfully[0m
[34m
=== Running simple_websocket.go ===[0m
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: AFFCCspHtdh2Z_HNMPET81S-9g9L5xZJZO7i3Ohr8RUf1gncruPep-3J_1b-bXIQ-zocLY_fH8N600eqeAkiUA

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
        "id": "1qobg-2Rq-h1oteGUGivHlWqJXt932IRY3a7Nc5Ctt0dKs5wfdgbqRO66EOezNHHx2k8Yqy0eH7nrKd18WIjPQ",
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
        "id": "OxC0Iqm6dKscDmJ4lvOfrE5ULerFFSPFfDhxbdGMxPG7DjUSbfjfBCkLZHOrBWGwBrzGmKldyS-4fge4dR1o9Q",
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
        "id": "AFFCCspHtdh2Z_HNMPET81S-9g9L5xZJZO7i3Ohr8RUf1gncruPep-3J_1b-bXIQ-zocLY_fH8N600eqeAkiUA",
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

✅ Script saved: 7FGz5rcVEr4G1E_x5kDRgGqX2W9vKg2fYIRifOgvPWybD3rR2LUGcxIHjZLxUXuyaKzyvHYv1dp9QP0lxdEUgQ
📊 Found 25 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: OzxCBpsQbuM6nh0e3YDNPE677T7Qw4ecxmJ-oHqzVIu939_P2wQlm0iuALcL6ekXAvJ0TlvTNiwaS25CfpbBcQ
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: zMt_7YxgCQNtILr_UOW9aq8M2RyW7JiM45zwslcnre8GRo0O3p5e3ayeBZGcGAgAxSe5mzsOduq5P7anrFTEpg
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
Collection created with first record: 9xzblgqB8lEOdREN2vDUEGFKWjbNXx_bJNHUqpNaTQFY2Iz0SOBPMIycyDof3dGAbIJUOtdwbZ0aFAGGzi_GTg

=== List Collections ===
Total collections: 15
Sample collections: [schema_products_client_py schema_documents_client_py batch_users ttl_cache test_accounts]

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
Created Alice: $1000 - ID: V0v5GicL9BJ-nE_MQCMkDBUXdLgiA8WypGzvzfzjQXBHtZ_43aWhDcVPGXNmBToJ99T1kd7siSoPFagq2hvlVQ
Created Bob: $500 - ID: Hr4f_MupYmj1lVPJea5cyKzT8hu7SLCTrxFXDinowGIQutzQWTf3YV4W5C_k-a6yRzs3uzC7ffd8GMmPv6sAaA

=== Example 1: Begin Transaction ===
Transaction ID: e7272a86-2a0b-41d9-9460-f0eb805c4dd6

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
New transaction: c364c374-a84b-4094-976f-09e74b23fea8
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
   ✅ Script saved: PIgmQPUFawfC3-1VaCPAn4erFUOkkuwQigQ4J9jPax942R8Vc9zkA8x5UbcSSsTitXt8QJjLCB0KPwbqvK3ezw

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: zzE58FAtsrpdq1icYlFt1wM700C_lflzqtjS73pSd4-Mbxa_-ZaI0amfPCbTkQKUn9C86BOjHo8him2gSNWfDg
   📋 Name: map[type:String value:Alice Smith]
   📋 Email: map[type:String value:alice@example.com]
   📋 Status: map[type:String value:pending]
   📋 Credits: map[type:Integer value:0]

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: FGRoteCYZJjRHNpnisATA-dImHTlXpyHDwTG0u1rmhkwbZSkYzEw71rdjxIPc_dULFigeGZxRceqh3ozcoxKzQ

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
   ✅ Script saved: tx2p8R3tPR2UA1ZBB3LrOrzclzJxESYCTXMfL-OkBIQvdY4rm2pgdHE2ChlvJpzU8p8vuCYMBPIV698AixOffA

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
   ✅ Script saved: Lysf3jjnsp3kkyvV2nXPogh0NavL3wsos_nMYCRfBO6M1HMtUnzZmOHM1lLGHvh_rvOVTvAXIfvPk8mE6-w4Dw

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: PIgmQPUFawfC3-1VaCPA...
   ✅ Deleted script: FGRoteCYZJjRHNpnisAT...
   ✅ Deleted script: tx2p8R3tPR2UA1ZBB3Lr...
   ✅ Deleted script: Lysf3jjnsp3kkyvV2nXP...
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

2026/01/26 17:55:23 No .env file found, using defaults
✓ Client connected

═══════════════════════════════════════════════════════════
TEST 1: Document TTL Expiration
═══════════════════════════════════════════════════════════

[Step 1] Insert document with 3 second TTL
  Input: {name: 'TTL Test', value: 'should expire'}
  TTL: 3s
  Output: Document ID = z5V1nK0jftNLmoTjhvqUDPfhOdF7xpfS8P3DTpFq50PZttiMl4MbeKFz7ld-_ZSd4OzSSvwQh_frpai5UVNC3Q
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: FindByID(z5V1nK0jftNLmoTjhvqUDPfhOdF7xpfS8P3DTpFq50PZttiMl4MbeKFz7ld-_ZSd4OzSSvwQh_frpai5UVNC3Q)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: FindByID(z5V1nK0jftNLmoTjhvqUDPfhOdF7xpfS8P3DTpFq50PZttiMl4MbeKFz7ld-_ZSd4OzSSvwQh_frpai5UVNC3Q)
  Output: Error (expected) - request failed with status 404: ��Record has been deleted
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

2026/01/26 17:55:30 No .env file found, using defaults
✓ Client connected

═══════════════════════════════════════════════════════════
TEST: WebSocket TTL Expiration
═══════════════════════════════════════════════════════════

[Step 1] Insert document with 3 second TTL
  Input: {name: 'WS TTL Test', value: 'should expire'}
  TTL: 3s
  Output: Document ID = pcKVBqCAxREIWs3DDD_uljzn9POnTUrxhkHgaiOL2MvkuF6n35RKAo4RPOZwvBN1Ba_-ctv5kzB-JA_pdnTGaA
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: FindByID(pcKVBqCAxREIWs3DDD_uljzn9POnTUrxhkHgaiOL2MvkuF6n35RKAo4RPOZwvBN1Ba_-ctv5kzB-JA_pdnTGaA)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: FindByID(pcKVBqCAxREIWs3DDD_uljzn9POnTUrxhkHgaiOL2MvkuF6n35RKAo4RPOZwvBN1Ba_-ctv5kzB-JA_pdnTGaA)
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
Inserted: map[id:P4AV-7NC79_5j-jMmeOQk5UKSpNroGIrZyb0RGakvRe0HfDpQZB4xNtMNvRJRjvGCRYofS1PTK4a6EUru3QK6A]

=== Find by ID ===
Found: map[active:true categories:[electronics computers] created_at:2026-01-26T17:55:37-05:00 data:aGVsbG8gd29ybGQ= embedding:[0.1 0.2 0.3 0.4 0.5] id:P4AV-7NC79_5j-jMmeOQk5UKSpNroGIrZyb0RGakvRe0HfDpQZB4xNtMNvRJRjvGCRYofS1PTK4a6EUru3QK6A metadata:map[key:value nested:map[deep:true]] name:Test Record price:99.99 tags:[tag1 tag2 tag3] user_id:550e8400-e29b-41d4-a716-446655440000 value:42]

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 0
  active (Boolean): true
  price (Decimal): 99.990000
  created_at (DateTime): 2026-01-26 17:55:37 -0500 EST
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1 tag2 tag3]
  metadata (Object): map[key:value nested:map[deep:true]]
  embedding (Vector): [0.1 0.2 0.3 0.4 0.5]
  categories (Set): [electronics computers]
  data (Bytes): 11 bytes
Plain record: map[active:true categories:[electronics computers] created_at:2026-01-26T17:55:37-05:00 data:aGVsbG8gd29ybGQ= embedding:[0.1 0.2 0.3 0.4 0.5] id:P4AV-7NC79_5j-jMmeOQk5UKSpNroGIrZyb0RGakvRe0HfDpQZB4xNtMNvRJRjvGCRYofS1PTK4a6EUru3QK6A metadata:map[key:value nested:map[deep:true]] name:Test Record price:99.99 tags:[tag1 tag2 tag3] user_id:550e8400-e29b-41d4-a716-446655440000 value:42]

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-01-26T22:55:37Z] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:P4AV-7NC79_5j-jMmeOQk5UKSpNroGIrZyb0RGakvRe0HfDpQZB4xNtMNvRJRjvGCRYofS1PTK4a6EUru3QK6A metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Updated Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: 6uIw5A-W1YJAiYQboiX0_J7qLwliSjDoCsSsL7NiEZis87gA5jBgEHmMbKDXK6j5AN5FrkmYwKxwwMfqp-52oA

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
Collection created with first record: fmzhoCkwoktXhXXI7ax9lb3AzZCd8zFLQizzLo65YoSo7RY1VeI9X9jMe2a7gEka6_xsk5sPc8LH_o85olgKAw

=== List Collections ===
Total collections: 14
Sample collections: [schema_products_client_py schema_documents_client_py batch_users ttl_cache test_accounts]

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
  cache:product:1: map[value:map[name:Product 1 price:29.99]]
  cache:product:2: map[value:map[name:Product 2 price:39.99]]
  cache:product:3: map[value:map[name:Product 3 price:49.99]]

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 3 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 9

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
Created Alice: $1000 - ID: YbKInHl0iEodxJWlFPNWykDdqKDaa7sUslEYYSLhKMboT2tp790uP7FivgshEy3IkPMyFSzpjNNWqg0lp7Cxsg
Created Bob: $500 - ID: _0K-ngjkNBp3x9nfyB3a3KrTPqm9H1uWKQItkAePiAV2QjUk1PJK3dHyTs-NzV4W0aNo3Z0jgiBMWI7txfZlqw

=== Example 1: Begin Transaction ===
Transaction ID: 2f62d599-34fd-45b3-a672-e588cfa46ca9

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 589d206a-43f2-40c5-b4a9-b9ab7c6a0cc0
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
  1. Score: 0.766
  2. Score: 0.748
  3. Score: 0.734

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.706
  2. Score: 1.494
  3. Score: 0.299

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
✓ Inserted document: h3VVNsWwdTlKWMiBkz-oMUKSt21N8q_gTXcgpTlB7ySHqMVhjzC3WeJxS3lKPhszFjLVoXqTvdv-7KytT8Mnrw

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: gV2vq9KSI6_3V4DQ7JyDL6f1G9mXJGZKvtBufiSHZC0U2DxGdNSi9wVe1H_uzRoGuTaNvGmOqLDWtw_NMAl5yg

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
✓ Inserted document with TTL: 61Ff9tdVlR-qCcWub-ngrfOUvk0H3KjGniDJMgitb9sMZAYLgN148vgEJ45alIYSKiS7XbsGSMufrWhdXjPmIg

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
✓ Edge cache script created: ptBHkMAeGLWJVrHrjKx8oGjuJ50CLgWlY0h8tlAsqXcA26eygolbLOBf1oOgNhiYUM4uzrUuH2m0WuiE-WzZ9w

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

✅ Script saved: S4dsegmHpml18QBwW-qhX4BYW6bgjfIBGAOp2WPH4Bh7hHWMlVZWf7bfWHNCZOHndVyh3UCTdLwMDpsoxwlXVQ
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
   ⏱️  Duration: 2.291459ms
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
   ⏱️  Duration: 2.562333ms
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

✅ Script saved: 96Adlu4afzfPek3i0ki_cUkchwuznwLj1luHoDHO3aaDQLCMNBGAPYOtGVmnKQqWTmfINEan_f54eZt2m7t3yA
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

✅ Inserted order: OHEgUpfBCWumN8ZDBZcb6OoQpP8m32BUoJYpYU_yYpFgE8LyVIHWud4lzAUqBv8jrNF-V1sqYP5R1IW_0rBh1w
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: xKhjvk7tYw4mGaQKtfUkEfBvc9vPARtAs2gWAi29oc26pGCVCo6_Nw-YCxJVpk4vzNlSTCukiKEg_EUuP9-CgA
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: map[role:admin userId:user_abc]
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session
📝 Example 4: KV Operations in Scripts

✅ Script saved: VGkDNeEfg8SaHT_mx1Msv_zdlasy2b3hLmlY_g0jtlsD9dtPOqpjkUDIHISf_xT4-RKoVT3zi77RLtIIaKKrsA
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Script

✅ Script saved: POxZHCNDxXj5D97jaB8we5TaWJkLLWiPSCa2R5gCqAd-b5YglAG7KEgT-45zDs6DHOzp2gH3sIIBAkXZDeJawA
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
✓ Created SWR script: fetch_api_user_go (CBxgMc5bVzVErbqOIcEgEYCJejX7DKy-_JtIwaAoheghAzoJpflosdGDi6MKWp3MYAbK4_HQeRtYdHSt7d02zw)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "cached_at": "2026-01-26T17:55:40-05:00",
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
      "id": "lT2GXTLtH4WqWYze2P9NkDVBGNlrNsXPAxsqMm38dzc7Meh7OnE7bDvp2yfyD5MWhJijV4Biu6_xWIzZJARGYw"
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
Response time: 62ms (served from cache)
✓ Lightning fast cache hit

🧹 Cleaning up...
✓ Cleanup complete

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation
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
   map[avg_price:367 category:Electronics count:5]
   map[avg_price:365.6666666666667 category:Furniture count:3]
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
   map[count:7 status:active]
   map[count:3 status:inactive]
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
   1. map[type:String value:Vector Databases Explained] (map[type:String value:Database])
   2. map[type:String value:Getting Started with ekoDB] (map[type:String value:Database])
   3. map[type:String value:Introduction to Machine Learning] (map[type:String value:AI])
   4. map[type:String value:Database Design Principles] (map[type:String value:Database])
   5. map[type:String value:Natural Language Processing] (map[type:String value:AI])
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
✓ Created session: kH7-Vta-h2D_ObzJ5F7JY5yXmQLoSUq2DDjvWf0gxR5lfY1gIXOlDWMqEt2vV2XJkwmsTYJKlC6SM4Mnind7dw

=== Sending Chat Message ===
Message ID: _K3LPKk49X0QofiKYWyNCAa4rVnf3lUjiqkdM399AWchcnxI336FsVLY85p1i9uGPrCqsu18IEtkFv71eAHIbA

=== AI Response ===
There are three products available:

1. ekoDB Pro: This is an Enterprise edition product with advanced features. The price for this product is $299. 

2. ekoDB Cloud: This is a fully managed cloud database service product. The price for this product is $499.

3. ekoDB: This is a high-performance database product with AI capabilities. The price for this product is $99.

=== Context Used (3 snippets) ===
  Snippet 1: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Enterprise edition product with advanced features id:ZpZAFg21cj3wm3Aaq1mSlaBddhrfhlnSRUA9RsAEd3hUnHhRKWEnHD25ZW3_okeuUW8_CXeW3BIYbPFKDadoBA name:ekoDB Pro price:299] score:0.1111111111111111]
  Snippet 2: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Fully managed cloud database service product id:fAFphbInz5pG1x_psTJLhC5e4yH77L_rxqZJ2cstvZHx2_hDkm7lSMuHyzFdu1CVPrvOGavbjhtr4AuZRFuYtg name:ekoDB Cloud price:499] score:0.1111111111111111]
  Snippet 3: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:A high-performance database product with AI capabilities id:5funuy-DlTnqE2-fREBeAvyOrt7BA4I4bkztHitO_gxiAk8AmOeCv7_EfHd3UWacPIKdgh64ronl48pi-nYSxg name:ekoDB price:99] score:0.1111111111111111]

Execution Time: 3109ms

=== Token Usage ===
Prompt tokens: 443
Completion tokens: 85
Total tokens: 528

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: G8r6AKkQbJDf3AYuxW83gZHubVFBBjLCKStD2SNWup9hzuOEHqwmo35_fQ5l0eFX0u67LDNHCNaAbQRqrvYvKQ

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information provided, one of the available products is ekoDB. This is a high-performance database product and it is priced at $99.

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
✓ Created second session: jBVTTzZ69hYnYPb7KjJmc_mUrtTOlS86_zYWhi4yc9DZgyPeDLwFr5z4S0bA13twwEabL5YuAUIjrRDTGC0VDw
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
✓ Created session: vIkerozaAQbKTwFKq5b79WasmEUkBW8FnwuwBtvUPKVVtL8jry26uj9VJ7-R-lJO_VSYmKBgB-YAgoXUu4WYgA

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product based on the provided context is "ekoDB". It is a high-performance database product priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: wBnrcJaBRWhN08x3Yhg9H2Kmnw6QtHHeDE_6JX7_lp-9NQ_eMO4-N0OE3fUwR0DKQMN3ybglMYJkShDQn0-C_w
  Parent: vIkerozaAQbKTwFKq5b79WasmEUkBW8FnwuwBtvUPKVVtL8jry26uj9VJ7-R-lJO_VSYmKBgB-YAgoXUu4WYgA

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: wBnrcJaBRWhN08x3Yhg9H2Kmnw6QtHHeDE_6JX7_lp-9NQ_eMO4-N0OE3fUwR0DKQMN3ybglMYJkShDQn0-C_w (Untitled)
  Session 2: vIkerozaAQbKTwFKq5b79WasmEUkBW8FnwuwBtvUPKVVtL8jry26uj9VJ7-R-lJO_VSYmKBgB-YAgoXUu4WYgA (Untitled)
  Session 3: jBVTTzZ69hYnYPb7KjJmc_mUrtTOlS86_zYWhi4yc9DZgyPeDLwFr5z4S0bA13twwEabL5YuAUIjrRDTGC0VDw (Untitled)
  Session 4: kH7-Vta-h2D_ObzJ5F7JY5yXmQLoSUq2DDjvWf0gxR5lfY1gIXOlDWMqEt2vV2XJkwmsTYJKlC6SM4Mnind7dw (Untitled)
  Session 5: kyC0Ud3IJFGfhFk2Y2madFyLgiMvCvPmsvGcY1ih02i2fp016-68MsK1BxLwHr8CiBTZH7_6rwrr4c85w1mskw (Untitled)
  Session 6: abgQ5GG0yWu3q7SuaGVLCcUVlpN9Lzr4K_Ny5-l2OTbo_TIKL8_i_0l6B09fmXoNwSeA7l-_BUQDVow0CLWaqQ (Untitled)
  Session 7: uFJ8rSkbpXH0q3zdp1u6zQl-s1cb-aPG7sUj-qT6fhtiUDubtV-rfrCcwCAbv0AHO4McRBQPiLBRyERPvdYQ5A (Untitled)
  Session 8: BA_ndoTnr5r03-371eTaiV8HcAFSw0mV27F1OLGme-UgwiGVwrPuPA1_Kp9gyHW2wlVLEZqBv0vfCO4udTlanA (Untitled)
  Session 9: pcvxPqW253IbKRKd-rHxq-mW7fHJhDl4VlInFqDEDbDIVNBA5RUJBXB2SDuGmSYhgC8rUos6lwrMWjZ1mnEJVg (Untitled)
  Session 10: JJp8VIsGR4wyMvu17JEv-OLpzuXsTUFvjS_alTXh2kLaX0urem-vKy4enOAO1M-uRfBK8cMBjeUSkp6euPePLw (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: wBnrcJaBRWhN08x3Yhg9H2Kmnw6QtHHeDE_6JX7_lp-9NQ_eMO4-N0OE3fUwR0DKQMN3ybglMYJkShDQn0-C_w

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Map Creation ===
✓ Created record with native map: map[id:ys6Lbj2JZxjGW2dwvUFsFicflozR-pFdXKLwC6H35UJrZ9xX_etyXqCOJTNDv6kVjeOOqZ1HeMQ4NbXif7Y_Kw]

=== Upsert Operation ===
✓ First upsert (update): map[active:map[type:Boolean value:true] age:map[type:Integer value:29] email:map[type:String value:alice.j@newdomain.com] id:ys6Lbj2JZxjGW2dwvUFsFicflozR-pFdXKLwC6H35UJrZ9xX_etyXqCOJTNDv6kVjeOOqZ1HeMQ4NbXif7Y_Kw name:map[type:String value:Alice Johnson]]
✓ Second upsert (insert): map[id:PlgQmXMtHn7VRA4sIMIKDx7hx6QbXpa8CHYIs5FhmZ9EPphAWr06el7FIYzMCtvPacpKxXielG6Q9KGzIA2hvw]

=== Find One Operation ===
✓ Found user by email: map[active:map[type:Boolean value:true] age:map[type:Integer value:29] email:map[type:String value:alice.j@newdomain.com] id:ys6Lbj2JZxjGW2dwvUFsFicflozR-pFdXKLwC6H35UJrZ9xX_etyXqCOJTNDv6kVjeOOqZ1HeMQ4NbXif7Y_Kw name:map[type:String value:Alice Johnson]]
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
Inserted with ripple: map[id:w8GjWd_AZ-diWwi1Clx0_0OXhupmA_doVDeE5G6lT-rp4IYfWpWU5IRa_WMAX-L8PJoDJ2Ydm-ULdA1ejdVwow]
Inserted with bypass_ripple: map[id:__TUxOflRQ7xPTy2TY-yk37rcMiEc1vN1BAY0tOJd0XFI-Wvs35v-3UREmGNTj4DGrDfeex5vg4BM1H53KEMfw]
Inserted with TTL and bypass_ripple: map[id:OdWZHgyvxu0JyJjAtk2fRpMBkaDWmgX4ix12_1vHhUZhw8astIDfCDLT43LeN0MzkI8GzlCj_zia4j4FUHuP8Q]
Updated with bypass_ripple: map[id:w8GjWd_AZ-diWwi1Clx0_0OXhupmA_doVDeE5G6lT-rp4IYfWpWU5IRa_WMAX-L8PJoDJ2Ydm-ULdA1ejdVwow name:map[type:String value:Product 1] price:map[type:Integer value:150]]
Deleted with bypass_ripple
Batch inserted with bypass_ripple: 2 records
Upserted with bypass_ripple: map[id:_jWKk2rKpcmwG_snXi8YmCN1XLmqRgWMbwd3JhJ4mefm7Ng-vz3d_f29jJptgB8vn4LNwiKnx50x_-aqQaTIiQ]
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
✅ [32mGo client examples complete![0m
