make test-examples-python
🧪 [36mRunning Python examples (direct HTTP/WebSocket)...[0m
✓ Authentication successful

=== Insert Document ===
Inserted: {'id': '3-_TiKhciynTiwSzxb7X961SGdI7lbEOGQv-1ducmBxOyZw-ilhZA8CbLpRMkvTyQmFZMntYmEeXsrvtJgerpQ'}

=== Find by ID ===
Found: {'value': {'type': 'Integer', 'value': 42}, 'name': {'type': 'String', 'value': 'Test Record'}, 'active': {'value': True, 'type': 'Boolean'}, 'id': '3-_TiKhciynTiwSzxb7X961SGdI7lbEOGQv-1ducmBxOyZw-ilhZA8CbLpRMkvTyQmFZMntYmEeXsrvtJgerpQ'}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'id': '3-_TiKhciynTiwSzxb7X961SGdI7lbEOGQv-1ducmBxOyZw-ilhZA8CbLpRMkvTyQmFZMntYmEeXsrvtJgerpQ', 'name': {'type': 'String', 'value': 'Updated Record'}, 'active': {'value': True, 'type': 'Boolean'}, 'value': {'value': 100, 'type': 'Integer'}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: _fTT58ZuRyTkwYIvKXAqGOX99RHlkFNzSWeao1crNEhiYCaKvdYr9kNqOUGSBYnv0mGpuKaph-CW_Zs55-JXTg

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
        "id": "_fTT58ZuRyTkwYIvKXAqGOX99RHlkFNzSWeao1crNEhiYCaKvdYr9kNqOUGSBYnv0mGpuKaph-CW_Zs55-JXTg",
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
        "id": "F1cpxxCw_vwIzadFdpe6tOmOcdnsSU20_rYQHJu07ahsWI8XYIwfHxSbKcWsq2Aq6XbHBQXjBJZ9WE7AyIYCZg",
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
✓ Authentication successful

=== Batch Insert ===
✓ Batch inserted 5 records
✓ Verified: Found 10 total records in collection

=== Creating test records for update/delete ===
Created 3 test records

=== Batch Update ===
✓ Batch updated 3 records
✓ Verified: Record updated with status="{'value': 'active', 'type': 'String'}"

=== Batch Delete ===
✓ Batch deleted 3 records
✓ Verified: Records successfully deleted (not found)

✓ All batch operations completed successfully
✓ Authentication successful

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: {'userId': 123, 'username': 'john_doe'}

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: {'price': 29.99, 'name': 'Product 1'}
cache:product:2: {'price': 39.989999999999995, 'name': 'Product 2'}
cache:product:3: {'name': 'Product 3', 'price': 49.989999999999995}

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: 3oMZrBehrFhXl1bMFoXq-L1o7-G3AsB1AEic70tkgR3G4cm2If6b62WpAU9RiLSHGeoJCh2FxnAYu9OPPCUWiw

=== List Collections ===
Total collections: 11
Sample collections: ['batch_users', 'scripts__ek0_testing', 'demo_collection', 'chat_configurations__ek0_testing', 'users']

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: False

✓ All collection management operations completed successfully
✓ Authentication successful

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: 8zxHe8FZEr3eQHGOYlpTdEog2DPG_yPLFHHDJbOVsvVZAI6K4YbEBPit8SXC2iMNa_cUichz-ncTLL8RNWJj0A

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: created

=== Query Documents ===
✓ Found 2 documents with TTL

=== Update Document ===
✓ Updated document

=== Delete Document ===
✓ Skipped delete (no ID returned)

✓ All document TTL operations completed successfully
✓ Authentication successful

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: Z0aVrWUQTLV0-3z_akkRUtjLj8eGLLHYmq4bPV68CqFDmocxtXtuQkt4K0NN90NgD_MU_U6W_F2ktRSL6utcig

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 2 record(s) via WebSocket

✓ WebSocket TTL example completed successfully
🚀 ekoDB Scripts Example (Python/HTTP)

✓ Authentication successful

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: xndv_6zUSaHqbfRiUkh-HeGc68f8ykdjAaIJJlDObZMnMVWr07qBsOFQRRTgN7mbaBLWqLtr31vD6CExgn8OVA
📊 Found 15 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: 65Du9q_aCqvBuyazPEIxPI4Pf4578PUAr4tT8tQCvpQN69fuVCQgxqPycM2Q_MPQX7EZ4Y7QMpTjLpbAW2xfvg
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: mhF-OiseS1C4lrOBfVMscDPT4pJJdOIGnQE3J2dgUAzJunc5CsyWbG2PhZhYet05dLAt98J8hotU9QrQAOR2Pw
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {'status': 'inactive', 'count': 15, 'avg_score': 50.0, 'max_score': 90}
   {'status': 'active', 'count': 15, 'avg_score': 60.0, 'max_score': 100}

📝 Example 4: Function Management

📋 Total scripts: 8
🔍 Retrieved script: Get Active Users
✏️  Script updated
🗑️  Script deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

✅ All examples completed!
✓ Authentication successful

📋 Getting original configuration...
   Original durable_operations: False


============================================================
🔥 TEST 1: Original Config (durable=False)
============================================================

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: Wv2dXGM6ExIiTulNEYt1rekEfZj-YpOe89jS6kVVRMuL6_kovzq6MEEH39KDK8LV8EPNxOHPR8zLADJSO5V1zg
Created Bob: $500 - ID: GIHivgMIXCDD4WfWG0fHigX3duGYGye0pA_U73I-BEjKngiLGPOzVCQHa-X9ysESbg25tHdVah3dOqrSevJvqA

=== Example 1: Begin Transaction ===
Transaction ID: ee3f69fb-fa02-40d1-b7c2-8c25e2f10f29

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
New transaction: 54307c58-c37f-4aa8-adaa-b660616c38f9
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: QIBZMZ1S-rwTGhI_Si_NVHdnvfn8xDSQQwmOsysK386pB5PPb_D5ZLoRtOj-miqIs5KAzdAIuiiYtIeFffB86Q
Created Bob: $500 - ID: LSHElngYeev2MsbB3bYkSQ50Pf-b_WqDPa2fKa2edijUvjeFCD9VtpxYz3EDHbgCst1alzcmGmucqseVLZEmuw

=== Example 1: Begin Transaction ===
Transaction ID: e2d04078-5959-4399-9dfc-989a97eca6df

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
Created Alice: $1000 - ID: RaqUpSdHcvbX4U75zsWTpgyyx2S4YEulqGgXbA0ULJQu0YqxRD12rm9sT_y8Ye6as9ioEyvSU3RpaiXsDD8Bvw
Created Bob: $500 - ID: Bze7ZHDO5_kuQGgePqOn9hY1qAzaV3UYk7Zjt_mX2qoiNytyuWCt7PZdP7vwAK32_ekr-olLJ9RVwJrs79JWAQ

=== Example 1: Begin Transaction ===
Transaction ID: f52ddb1f-926e-4fc4-af85-b814e77eb580

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
New transaction: 198dfdde-4fb0-49dc-b599-137b2a17e39a
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: GKLZd7sIRB24UN5i11rTTtBaJj4LaQ29Myvi38U-ihT84A-98kpyYRmRtz6oih9iHFcZr2lQntzj9PYSdL7V1Q
Created Bob: $500 - ID: V0VI2XSRFYwFEq17ID4zGsnonQDRrnlamIjdxkTmggsuPu92yzBOeWW1ffRNBre7b_vhifQ8CkgfvJ8tOuQ5-Q

=== Example 1: Begin Transaction ===
Transaction ID: da13b76e-f473-4d12-9e42-52f0ebe8dce1

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
Created Alice: $1000 - ID: FKUtaWQ80_BtVcNqzITjLB9qE-NU7ID7SxungE3KsGddoW7d50DwvHOffYbAcA4uPV4n8rUNvbrt3qfTdfrKQQ
Created Bob: $500 - ID: Yw7tjmSB0ozDTLli-QJsTlWZ-M_4QdkJ9QQVuFEN1d7kJyF84aWE3STnqmzd4Z2eM2bUrOoGSImb4Bs_RvvdwA

=== Example 1: Begin Transaction ===
Transaction ID: b46c762c-84f6-4af2-b35e-96119ccf8c94

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
New transaction: 257079b3-88e5-4606-b481-4dd985df33b7
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 6mx0oiNjSV2LldgvP3-13_84uIw3XtbPIstGLh2pWcad5vN8Xb6a9muicGgs1fLMGG_hwSTU2BYmVjVvxFCupQ
Created Bob: $500 - ID: MNgdFxtF-0GJulWFq2eCMmn_cb2gMp8TN4YLOiiu_L_53tV7K5iuc4Nok_cdimdhQXWQjjZMCjPpxA8nS63MwQ

=== Example 1: Begin Transaction ===
Transaction ID: 738acfcc-f352-4cd1-b772-7cfb301ea16f

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
   ✓ Config restored: durable_operations=False


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
   ✅ Script saved: lO9BgUAN5mB2C3L9xY14btS4KYEWYpzNd0QGojzWBLgXXvfJaDxmpNRa1hjW-PsuE3eq9hyUWj8I9aZwRT8ckw

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: VMxbFrAjrGmoey30Ij5O8NI_ue6Hr6Am-qOGPX2m8xBAm1QunoukNFXW17aly0NgRhiCDmNo1wCbDbwiSAENGA
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}
   📋 Email: {'type': 'String', 'value': 'alice@example.com'}
   📋 Status: {'type': 'String', 'value': 'pending'}
   📋 Credits: {'value': 0, 'type': 'Integer'}

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: oh4hpcEnpcGxY_MybeMt016YtF71HdUjSYt9S0w4vzee5DX3HPI0kG8Qn31NRtUj2ABT3rUPo_6BAlST9JCiPg

2️⃣ Calling Script (Query + Update + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Status updated to: {'type': 'String', 'value': 'active'}
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}

============================================================
📝 Script 3: Query + Update Credits + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: ZaPVgcr4aKQb8LRIwtVqTrwpYrKZgaPBQ1NNd6_pdhg5DnV5Bqw5sxoDHGfQbzhykROqOeQNgkHXt_j-tgp5wg

2️⃣ Calling Script (Query + Update Credits + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {'type': 'Integer', 'value': 0}
   📋 Status: {'type': 'String', 'value': 'active'}
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}

============================================================
📝 Script 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: tBrZB5VBq_CIf3hqOrXfVDVCS3y8_H_O1aZUoJAeXcEzCzcYb7u1yCqNu5Zo1OLL9jyKgHggq3HbrRIbmGCWnw

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: lO9BgUAN5mB2C3L9xY14...
   ✅ Deleted script: oh4hpcEnpcGxY_MybeMt...
   ✅ Deleted script: ZaPVgcr4aKQb8LRIwtVq...
   ✅ Deleted script: tBrZB5VBq_CIf3hqOrXf...
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
=== Running batch_operations.py ===[0m
[32m✓ batch_operations.py completed successfully[0m
[34m
=== Running kv_operations.py ===[0m
[32m✓ kv_operations.py completed successfully[0m
[34m
=== Running collection_management.py ===[0m
[32m✓ collection_management.py completed successfully[0m
[34m
=== Running document_ttl.py ===[0m
[32m✓ document_ttl.py completed successfully[0m
[34m
=== Running websocket_ttl.py ===[0m
[32m✓ websocket_ttl.py completed successfully[0m
[34m
=== Running http_functions.py ===[0m
[32m✓ http_functions.py completed successfully[0m
[34m
=== Running transactions.py ===[0m
[32m✓ transactions.py completed successfully[0m
[34m
=== Running crud_scripts.py ===[0m
[32m✓ crud_scripts.py completed successfully[0m
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
📦 Built wheel for abi3 Python ≥ 3.8 to /Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.6.0-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel for current platform...[0m
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.6.0-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.6.0
    Uninstalling ekodb_client-0.6.0:
      Successfully uninstalled ekodb_client-0.6.0
Successfully installed ekodb-client-0.6.0
✅ [32mPython client package built and installed![0m
🧪 [36mRunning Python client library examples...[0m
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {'id': '8m2i1LySn12rf4Osaxk_e7crRRvmtDMUkYqApw5oKSRTlMkeWZalq8n4AXmtxnqA6r9p9TUhzGJrlg_3YxgpbA'}

=== Find by ID ===
Found: {'price': {'type': 'Float', 'value': 99.99}, 'name': {'value': 'Test Record', 'type': 'String'}, 'tags': {'type': 'Array', 'value': ['tag1', 'tag2', 'tag3']}, 'active': {'type': 'Boolean', 'value': True}, 'embedding': {'value': [0.1, 0.2, 0.3, 0.4, 0.5], 'type': 'Array'}, 'metadata': {'type': 'Object', 'value': {'nested': {'deep': True}, 'key': 'value'}}, 'categories': {'type': 'Array', 'value': ['electronics', 'computers']}, 'created_at': {'value': '2026-01-02T00:54:16.925085', 'type': 'String'}, 'value': {'value': 42, 'type': 'Integer'}, 'user_id': {'type': 'String', 'value': '550e8400-e29b-41d4-a716-446655440000'}, 'data': {'type': 'String', 'value': 'aGVsbG8gd29ybGQ='}, 'id': '8m2i1LySn12rf4Osaxk_e7crRRvmtDMUkYqApw5oKSRTlMkeWZalq8n4AXmtxnqA6r9p9TUhzGJrlg_3YxgpbA'}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): True
  price (Decimal): 99.99
  created_at (DateTime): 2026-01-02 00:54:16.925085
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): ['tag1', 'tag2', 'tag3']
  metadata (Object): {'nested': {'deep': True}, 'key': 'value'}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): ['electronics', 'computers']
  data (Bytes): 11 bytes
Plain record: {'price': 99.99, 'name': 'Test Record', 'tags': ['tag1', 'tag2', 'tag3'], 'active': True, 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5], 'metadata': {'nested': {'deep': True}, 'key': 'value'}, 'categories': ['electronics', 'computers'], 'created_at': '2026-01-02T00:54:16.925085', 'value': 42, 'user_id': '550e8400-e29b-41d4-a716-446655440000', 'data': 'aGVsbG8gd29ybGQ=', 'id': '8m2i1LySn12rf4Osaxk_e7crRRvmtDMUkYqApw5oKSRTlMkeWZalq8n4AXmtxnqA6r9p9TUhzGJrlg_3YxgpbA'}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'embedding': {'value': [0.1, 0.2, 0.3, 0.4, 0.5], 'type': 'Array'}, 'tags': {'value': ['tag1', 'tag2', 'tag3'], 'type': 'Array'}, 'active': {'value': True, 'type': 'Boolean'}, 'name': {'value': 'Updated Record', 'type': 'String'}, 'id': '8m2i1LySn12rf4Osaxk_e7crRRvmtDMUkYqApw5oKSRTlMkeWZalq8n4AXmtxnqA6r9p9TUhzGJrlg_3YxgpbA', 'metadata': {'type': 'Object', 'value': {'nested': {'deep': True}, 'key': 'value'}}, 'data': {'type': 'String', 'value': 'aGVsbG8gd29ybGQ='}, 'value': {'type': 'Integer', 'value': 100}, 'user_id': {'type': 'String', 'value': '550e8400-e29b-41d4-a716-446655440000'}, 'created_at': {'type': 'String', 'value': '2026-01-02T00:54:16.925085'}, 'price': {'type': 'Float', 'value': 99.99}, 'categories': {'value': ['electronics', 'computers'], 'type': 'Array'}}

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
Retrieved value: {'value': '{"userId":123,"username":"john_doe"}'}

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: {'value': '{"name":"Product 1","price":29.99}'}
cache:product:2: {'value': '{"name":"Product 2","price":39.989999999999995}'}
cache:product:3: {'value': '{"name":"Product 3","price":49.989999999999995}'}

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: "j6Xn4ksxUJ9tUbptyI0Knb7nnk0wdTFD5EznMMXo5GCNncSMZ3RYLcSlL_ywqOGLaGQAIMnNqF03BpJGOm5T2w"

=== List Collections ===
Total collections: 10
Sample collections: ['batch_users', 'client_collection_management_python', 'scripts__ek0_testing', 'chat_configurations__ek0_testing', 'ttl_cache']

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: False

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: 88VqMs8iaxyOEkzPYxr96XoOfvXvfR6B9mvjMo7DFOiqulN7y5KiJMBuINzqSITQkBLwtf8vwx__g92vFpnyxA

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: G371EppGbvutHA3iyBNLFZYpZE5PXIYhPBIvtmTN8M318aPAvZ9xQckc58ZQZGgHBnrBhSOu4_0rIIpGXnYYfA

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
✓ Inserted test record: DtYIzSZ885Kh-Gp817IJNr6RFQ-Gsr82PXfn1rjn9SvZlLkohRl86a16GVuSSvkkPccIFyRXm00X7d9omsQXUw

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
✓ Inserted document with TTL: Y43WIeycczxHR74S1PKykJ4suJvdsKqDHeMPUyO7QaXfy3VB28njW5SSIJEgLp4rhdQLSV_esYNwV4oaCsLziQ

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 5 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
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
  1. Score: 25.740, Matched: name, name.value, email.value, email
  2. Score: 12.540, Matched: name.value, name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: bio.value, title, bio, title.value
  2. Score: 26.400, Matched: bio.value, title, bio, title.value
  3. Score: 26.400, Matched: title, bio, title.value, bio.value
  4. Score: 26.400, Matched: title, bio, bio.value, title.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio.value, title, title.value, bio
  2. Score: 39.600, Matched: bio.value, title.value, bio, title
  3. Score: 39.600, Matched: title.value, bio, title, bio.value
  4. Score: 39.600, Matched: bio, bio.value, title, title.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio.value, bio

5. Search with stemming and exact match boosting:
Found 2 results (matches: run, running, runs)
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio.value, bio

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.775
  2. Score: 0.763
  3. Score: 0.725

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.742, Matched: content.value, title, title.value, content
  2. Score: 1.134, Matched: title, title.value, content.value, content
  3. Score: 0.507, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 13.200, Matched: title.value, title

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
  - Mouse ($25) by Alice Johnson
  - Laptop ($1200) by Alice Johnson

5. Complex join with multiple conditions:
Found 2 users with example.com emails:
  - Alice Johnson (alice@example.com): Building A
  - Bob Smith (bob@example.com): Building B

=== Cleanup ===
✅ Deleted test collections

✅ Join operations examples completed!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: DszLGnWdRc3qVTd1hrSCP54uqUp7i7BhSY2t8yWH1gNfoggn4Hap_8D6QTrKr9D_wYIYcLDRI8xQtzsAUY7HiQ

=== Sending Chat Message ===
Message ID: RG7sEDqP2VQJpL4GwVR6_tvoGDh__jraBUpJNikjJmLJsoOVzENBBgC9mLU3dD1wZ4b4JqDd56rixJ6CzcVgdw

=== AI Response ===
Based on the information provided, there are three products available:

1. ekoDB: This is a high-performance database product with AI capabilities. The price of this product is $99.

2. ekoDB Pro: This is an Enterprise edition product with advanced features. The price of this product is $299.

3. ekoDB Cloud: This is a fully managed cloud database service product. The price of this product is $499.

Execution Time: 2513ms

=== Token Usage ===
Prompt tokens: 610
Completion tokens: 90
Total tokens: 700

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: kGSJYCEbzW3WYaCWW7qVi3faqJ3rd7v582U6huy7pCWYY8f6ObSU-0M1hvqtXTDMoeMrmCByIiW54wUOC3a90A

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information provided, one product that is available is the ekoDB. It is described as a high-performance database product and is priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: NF65Qr0u5d_CIzW4ZPUvvdht1MeLOdHyC1MDWhmiu0PNUWDnl7YWJaFltN8rAUI2qhKt7abgg74zgLz0mZUrJA
  Parent: kGSJYCEbzW3WYaCWW7qVi3faqJ3rd7v582U6huy7pCWYY8f6ObSU-0M1hvqtXTDMoeMrmCByIiW54wUOC3a90A

=== Listing Sessions ===
✓ Found 6 sessions
  Session 1: NF65Qr0u5d_CIzW4ZPUvvdht1MeLOdHyC1MDWhmiu0PNUWDnl7YWJaFltN8rAUI2qhKt7abgg74zgLz0mZUrJA (Untitled)
  Session 2: kGSJYCEbzW3WYaCWW7qVi3faqJ3rd7v582U6huy7pCWYY8f6ObSU-0M1hvqtXTDMoeMrmCByIiW54wUOC3a90A (Untitled)
  Session 3: DszLGnWdRc3qVTd1hrSCP54uqUp7i7BhSY2t8yWH1gNfoggn4Hap_8D6QTrKr9D_wYIYcLDRI8xQtzsAUY7HiQ (Untitled)
  Session 4: m0J43kGqILKtUrl5O0HN4ccU2Y0S8UshpcPMxHmfvLO8rxHA6k6TbxC3u0LD39-JDoyHbM80XLYomD3kVphUqQ (Untitled)
  Session 5: Os4Ty52VQWOwXHxmpTvxMOLnCwAgd-aT5rjepSBeASzohnp0YdUUksWfNNiVq5OzIN-7q6T_MfIkl7GZGmwH4Q (Untitled)
  Session 6: pHzBl65XmiObokt99kagXWBR2XZATZkaDRbAQw9DV4VRVNekmdIShmIRACSrvbykFI1FO3S9t0o13S8KcuTkgw (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: NF65Qr0u5d_CIzW4ZPUvvdht1MeLOdHyC1MDWhmiu0PNUWDnl7YWJaFltN8rAUI2qhKt7abgg74zgLz0mZUrJA

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: L4nYI-D0XdjmM9_97dbLV6IHOTjy1w9smY1XQ73lCpCFn9O4nU93v1QcZ4r_50NTSR5jV7ICMM9-LAqTXTkodQ

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information available, one of the products we have is ekoDB. It is a high-performance database product, priced at $99.

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['context_snippets', 'chat_id', 'created_at', 'token_usage', 'id', 'role', 'content', 'updated_at'])
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
✓ Created second session: 8ij4JREUz3EKuqBCksCRA7svRIYYv-D5XIJB5gTr2mYoHY___fq_MEDed0_QHBNvUjkHoahbTBSyMnR4_TWC8Q
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
🚀 ekoDB Scripts Example (Python)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: UDApYCEFGX-vyWHDXaWrCatHoGv0FXpDS-JOkRDdGNrFcGP-ZQpsg15ORPItW1tMvFxsI03I6zbON0fDvo-8Dg
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved: tYcjjQuL0piddpU0eJV5kedaMqLFOvGU4cOA246expdYpdYjITeTLW3qF-Id4FNyL1yt5CEO3lAnOHXozp_S5w
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: 229aRhr70BwzWLHltDkAd2LVIyqlKPMnPd5HlJhaY8OA9S0RYtU1QSMzLoDNIutjhR1gWqTBbXwxZfiuX3IVDA
📊 Statistics: 2 groups
   {'avg_score': 60.0, 'count': 5, 'status': 'active'}

   {'avg_score': 50.0, 'count': 5, 'status': 'inactive'}

📝 Example 4: Script Management

📋 Total scripts: 10
🔍 Retrieved script: Get Active Users
✏️  Script updated
🗑️  Script deleted

ℹ️  Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.

✅ All examples completed!
🚀 ekoDB Python Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved: yFsodyU2_3te1PvVK3qsf5Rvr8oARFUPKEI7q3CeFnvaB5TYN_Fe6cGRCwEa-IFJURirhv7c9Ng_LcrXQN3GtQ
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
   {'category': 'Furniture', 'count': 2}
   {'category': 'Electronics', 'count': 3}
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
✅ [32mPython client examples complete![0m
