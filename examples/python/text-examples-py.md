make test-examples-python
🧪 [36mRunning Python examples (direct HTTP/WebSocket)...[0m
=== Simple CRUD Operations (Direct HTTP) ===

✓ Authentication successful

=== Insert Document ===
Inserted: {'id': 'MUZ9fHXO-lG9YfY3ObfTayyWeWy6eMh47MQdGBXqrPdaSP_aLI8JEBjH1bZPH_a74i9L5byfbQf7S6g604HuHA'}

=== Find by ID ===
Found: {'name': 'Test Record', 'value': 42, 'active': True, 'id': 'MUZ9fHXO-lG9YfY3ObfTayyWeWy6eMh47MQdGBXqrPdaSP_aLI8JEBjH1bZPH_a74i9L5byfbQf7S6g604HuHA'}

=== Find with Query ===
Found documents: [{'name': {'type': 'String', 'value': 'Test Record'}, 'active': {'type': 'Boolean', 'value': True}, 'value': {'type': 'Integer', 'value': 42}, 'id': 'MUZ9fHXO-lG9YfY3ObfTayyWeWy6eMh47MQdGBXqrPdaSP_aLI8JEBjH1bZPH_a74i9L5byfbQf7S6g604HuHA'}]

=== Update Document ===
Updated: {'id': 'MUZ9fHXO-lG9YfY3ObfTayyWeWy6eMh47MQdGBXqrPdaSP_aLI8JEBjH1bZPH_a74i9L5byfbQf7S6g604HuHA', 'value': {'type': 'Integer', 'value': 100}, 'name': {'value': 'Updated Record', 'type': 'String'}, 'active': {'type': 'Boolean', 'value': True}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: S-g_KsEmXxeRR7SufzogTElDqP-F9m9IRs05FO8UL78xYZaP_aAkSbHUzqAbaHHS4eeylQKCp90CHoPUwFFtkQ

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
        "id": "S_U13Mss9h6XuHFJAuiD6zeBJ1UT7ZOk0_SZxhcjzzoyIThIysh_qUPR8HijAPJf_mlmpEgauyRwdt0ZfUuDxA",
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
        "id": "S-g_KsEmXxeRR7SufzogTElDqP-F9m9IRs05FO8UL78xYZaP_aAkSbHUzqAbaHHS4eeylQKCp90CHoPUwFFtkQ",
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

✅ Script saved: pSKK_aMuPXVbU23PL5qxMArZ-kd27hazbNlvMVtdg6NPGadorCFJZ3Pf6-j4d-ElWyWAZGWj2ruvuUOxw-dxUg
📊 Found 35 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: 6HWYt9tpSedqMwmQy9A3P6apNJGzwUVaL2qMib8URg2uAR55-xhc7LLX91cNqfHX4x9GLH-3m6WXXwfb18YDhg
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: Sb37UIRBb4v0h__SuBQaIaEkN6nXH8KlCmjqz3tKnz2dL4I2aR9G2zs7KY3Me3tzuZ6ZONJlb3RNFxh7w2B8xw
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {'status': 'inactive', 'count': 15, 'max_score': 90, 'avg_score': 50.0}
   {'avg_score': 60.0, 'max_score': 100, 'count': 15, 'status': 'active'}

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
Retrieved value: {'userId': 123, 'username': 'john_doe'}

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: {'price': 29.99, 'name': 'Product 1'}
cache:product:2: {'price': 39.989999999999995, 'name': 'Product 2'}
cache:product:3: {'price': 49.989999999999995, 'name': 'Product 3'}

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
=== Collection Management (Direct HTTP) ===

✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: ZHiKwMsQAFJsq1jBDGXA5Qy-w6bMf7TFFh63RQUiQBhcd9b9neu6jj5hbkY3zzkN929V2mtafGppa9cUZt4H-w

=== List Collections ===
Total collections: 12
Sample collections: ['chat_configurations__ek0_testing', 'test_accounts', 'batch_users', 'products', 'demo_collection']

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
Created Alice: $1000 - ID: QvnSno6KXTmXgYOQe5DscfJbU-1rrHihx9xG9ckE5SnI8BkQWKjvjAr5oED5t6zyb4_aSIArpp-V9LEQsdACbw
Created Bob: $500 - ID: d8OSVdb7PyoVS2dM2CTXCpYUlN3YNQBqkRMgf99eZwTqL7kAGMmsfXpCeZvGxtR8-QYfFrO-TR689AvcLSHw5g

=== Example 1: Begin Transaction ===
Transaction ID: ade7229f-949a-4f3f-8c39-5da0ffa642a5

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
New transaction: ae2495dd-69ca-4eaf-9a48-c41129980e40
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: snK-zRLdmMadVtk-gZLQHAgPCyFPmvVVc9YjBtTpok3A6xQ3vbbka3KktUHzd6f9LgFTTK4sijMvTTadT-1KNg
Created Bob: $500 - ID: Kdahk9UJNfk5y3J24I1hr7kNrNto3cAesVLxuofsRD24dYid2z1zfbC9VE7SbiwN93aD-oRsvIf2oYX8RuinlA

=== Example 1: Begin Transaction ===
Transaction ID: 7b654b00-4883-404e-8265-84019719e6d7

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
Created Alice: $1000 - ID: C22cXT4tzhfFoKrvdQKfZUl0IxMw5CjNlpyxwydwLDBEt2ummIm8s62zBx_0t_-YoFUtzMs6Fc4Lt_6SjTaHyQ
Created Bob: $500 - ID: EnSWvJyaiZxnGPYP_SU4MsT8xFVixj3Wl9t8tgKsqpb6FPDol-gEvsSy0cD7hZ2x21wOuqxu-1Yeyo80nya50Q

=== Example 1: Begin Transaction ===
Transaction ID: c047db4c-ada5-4b6b-960a-ce231f4ac11d

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
New transaction: ec2f130c-f005-473e-b696-c762ec3100bb
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: OqasfENerjTmbbysUPaneJl7JW2NMn5m8XL_V4f-Rw8IwHUN9DjXViAc0F5gssUwbZ18b6nB_PpjswB2VASL5g
Created Bob: $500 - ID: QaDzGNJ-IwxvLPjQXKwXwkwMwrAqI4lKmWK19zOIS2Sxb-jN1pOmjUwuCF3ZhxAABerFmvKBQNLYFHofJpWnAw

=== Example 1: Begin Transaction ===
Transaction ID: 258e8a5e-7947-4e4e-9daa-c19635ed1d03

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
Created Alice: $1000 - ID: 7v-RP-LPFMVIf4AloDztHdjnNteFM0bljdd8hpMB6RDLbAk-qJh3aiXZXJzXtgbcoDjjXIB-yaVqcEvPFw0O8A
Created Bob: $500 - ID: vXyxQT-F_UVibygVMDEua_RNlWnIYOcjcS4MSyijh_q8q-Bbvu2H6P1IGS9jMo_cO85O54D8VjW2H1Fkut3f2g

=== Example 1: Begin Transaction ===
Transaction ID: 43a2490b-186e-4f14-b5a1-3f0a4b7160c9

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
New transaction: 492fafe4-b18f-49fe-8e9e-e40b253fe420
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: EJtp7pTDcXUVNUz9Rrim0Qg9oaeWP4PazBvzPCydUFiraVM6OrVZ67q_3ECwu26-GdU4kJ6THMsmB22UtLlmYg
Created Bob: $500 - ID: lozqYi9Mz4zFPs4Awmv5MVVBK9HtVbVJTEPf1QGJmdnlijUfjAB-9eNmTPZWSKTU7PeGC7jLLOdyxA_6Ne9-rA

=== Example 1: Begin Transaction ===
Transaction ID: 6d944704-b05f-41e4-a65a-a287713ff22d

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
   ✅ Script saved: Upg72njwfh2wie13rsU5xZS-uVbKfDROqrjxKGyuUycaoYoMDA7gXIEVAccALoP-nV2jxZf2KQBZ-j_H1VN7Zg

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: TY_CAFTrpPPHL0K1_qX8HVUG1jp5wK-fLxD2epvQabqJfCbb-S7p4_NfOCLkXkLno8UzAgDug-V96xylgatl8w
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}
   📋 Email: {'value': 'alice@example.com', 'type': 'String'}
   📋 Status: {'value': 'pending', 'type': 'String'}
   📋 Credits: {'value': 0, 'type': 'Integer'}

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: wpNJ7OgiPJurLNWgzjLNVk_oaexCD-CejcSyLJu_psNQP2FYoN2xqKgbL_nYdyCQVuVeNS_kY93uZZgFFJCJGQ

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
   ✅ Script saved: -jYYKd9tEiCSScS76dBUtucTfZtKXn17i84zN-G-2kJc9CQ5gDk1uCACRKRrkCT4r4v7mhEMA3W2HvY1hZC1fw

2️⃣ Calling Script (Query + Update Credits + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {'type': 'Integer', 'value': 0}
   📋 Status: {'value': 'active', 'type': 'String'}
   📋 Name: {'value': 'Alice Smith', 'type': 'String'}

============================================================
📝 Script 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: bLENLKTZasxhHnxcjdVyDyWAyD0Lz3rkMtpZJd2LSlyrMXSbFabf14R0dT0zJl7coGq_MULQ_19P6IYmE-tqlQ

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: Upg72njwfh2wie13rsU5...
   ✅ Deleted script: wpNJ7OgiPJurLNWgzjLN...
   ✅ Deleted script: -jYYKd9tEiCSScS76dBU...
   ✅ Deleted script: bLENLKTZasxhHnxcjdVy...
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
  Output: Document ID = 6v83fxyp818cACIFzlNoQ-EDdvzhQ-Eia6k1WY0csnmLV0TgkNA9o0VYC1O5to5i13VDd66b9gSxYDS3K7BFHw
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: find_by_id(6v83fxyp818cACIFzlNoQ-EDdvzhQ-Eia6k1WY0csnmLV0TgkNA9o0VYC1O5to5i13VDd66b9gSxYDS3K7BFHw)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: find_by_id(6v83fxyp818cACIFzlNoQ-EDdvzhQ-Eia6k1WY0csnmLV0TgkNA9o0VYC1O5to5i13VDd66b9gSxYDS3K7BFHw)
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
  Output: Document ID = Xp8O7hwbV1AYrR-Iea_l2m60WfutRHJPQeGSwCo0zSXHrSpSO05Ifst3eaZYRje1E3rYC9gRZMHXZY0d13khjg
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: find_by_id(Xp8O7hwbV1AYrR-Iea_l2m60WfutRHJPQeGSwCo0zSXHrSpSO05Ifst3eaZYRje1E3rYC9gRZMHXZY0d13khjg)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: find_by_id(Xp8O7hwbV1AYrR-Iea_l2m60WfutRHJPQeGSwCo0zSXHrSpSO05Ifst3eaZYRje1E3rYC9gRZMHXZY0d13khjg)
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
📦 Built wheel for abi3 Python ≥ 3.8 to /Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.9.0-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel for current platform...[0m
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.9.0-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.9.0
    Uninstalling ekodb_client-0.9.0:
      Successfully uninstalled ekodb_client-0.9.0
Successfully installed ekodb-client-0.9.0
✅ [32mPython client package built and installed![0m
🧪 [36mRunning Python client library examples...[0m
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {'id': 'uWEU98EpP2th3p_XiyBuB4lHiEzzqBsxmVk8SjJm99SpXQ6NWClL3yfq1aNEWp9SXiiHI6oWFyoHjgxWw8KVeg'}

=== Find by ID ===
Found: {'value': 42, 'data': 'aGVsbG8gd29ybGQ=', 'metadata': {'key': 'value', 'nested': {'deep': True}}, 'id': 'uWEU98EpP2th3p_XiyBuB4lHiEzzqBsxmVk8SjJm99SpXQ6NWClL3yfq1aNEWp9SXiiHI6oWFyoHjgxWw8KVeg', 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5], 'user_id': '550e8400-e29b-41d4-a716-446655440000', 'categories': ['electronics', 'computers'], 'tags': ['tag1', 'tag2', 'tag3'], 'price': 99.99, 'created_at': '2026-01-27T12:13:12.667392', 'active': True, 'name': 'Test Record'}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): True
  price (Decimal): 99.99
  created_at (DateTime): 2026-01-27 12:13:12.667392
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): ['tag1', 'tag2', 'tag3']
  metadata (Object): {'key': 'value', 'nested': {'deep': True}}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): ['electronics', 'computers']
  data (Bytes): 11 bytes
Plain record: {'value': 42, 'data': 'aGVsbG8gd29ybGQ=', 'metadata': {'key': 'value', 'nested': {'deep': True}}, 'id': 'uWEU98EpP2th3p_XiyBuB4lHiEzzqBsxmVk8SjJm99SpXQ6NWClL3yfq1aNEWp9SXiiHI6oWFyoHjgxWw8KVeg', 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5], 'user_id': '550e8400-e29b-41d4-a716-446655440000', 'categories': ['electronics', 'computers'], 'tags': ['tag1', 'tag2', 'tag3'], 'price': 99.99, 'created_at': '2026-01-27T12:13:12.667392', 'active': True, 'name': 'Test Record'}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'name': {'type': 'String', 'value': 'Updated Record'}, 'active': {'value': True, 'type': 'Boolean'}, 'id': 'uWEU98EpP2th3p_XiyBuB4lHiEzzqBsxmVk8SjJm99SpXQ6NWClL3yfq1aNEWp9SXiiHI6oWFyoHjgxWw8KVeg', 'categories': {'type': 'Array', 'value': ['electronics', 'computers']}, 'data': {'type': 'String', 'value': 'aGVsbG8gd29ybGQ='}, 'metadata': {'type': 'Object', 'value': {'nested': {'deep': True}, 'key': 'value'}}, 'embedding': {'value': [0.1, 0.2, 0.3, 0.4, 0.5], 'type': 'Array'}, 'tags': {'type': 'Array', 'value': ['tag1', 'tag2', 'tag3']}, 'price': {'type': 'Float', 'value': 99.99}, 'user_id': {'type': 'String', 'value': '550e8400-e29b-41d4-a716-446655440000'}, 'created_at': {'value': '2026-01-27T12:13:12.667392', 'type': 'String'}, 'value': {'type': 'Integer', 'value': 100}}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: rQzVryRzkG9Tb2EQYbG4MTPUMFYnntzmZw1FKO4eW-rV0Ja73WsH2asRR8q4M-yvfKcxJX4LyN_kdTzsJqwqwA

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
Collection created with first record: "-uZAkaeqR_CsVGAf2zUpdPVk5FmT3AZ47DuT9NrBdNqVe91Q6a9CJF8mr1KmmV2woezh4WYaqvLf3dVrtgJd1w"

=== List Collections ===
Total collections: 11
Sample collections: ['chat_configurations__ek0_testing', 'client_collection_management_python', 'test_accounts', 'batch_users', 'products']

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
Created Alice: $1000 - ID: nbv0-GmB83mItIAemiL_8I2SfPvkjAadCSqVaLMVOciBVwABXtP9E0GGp6N5nlJoFk5cZyULemwHq3fOAC4pIQ
Created Bob: $500 - ID: dwMtsajS-csdKSBwvyjpmPqVkIKwx005vOfszHyjNyZlc-LCjpeVIZPb-c3_cBCYeazNkxjGGlXiLHGeE9Z6kA

=== Example 1: Begin Transaction ===
Transaction ID: 32c75922-448e-46c2-bc70-f62a683541b1

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 3f9f100e-6746-46d4-a0e5-49f1177b8e4f
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
  1. Score: 25.740, Matched: name, name.value, email, email.value
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title.value, bio, bio.value, title
  2. Score: 26.400, Matched: title.value, title, bio, bio.value
  3. Score: 26.400, Matched: title.value, bio, bio.value, title
  4. Score: 26.400, Matched: bio, title, title.value, bio.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title.value, bio, title, bio.value
  2. Score: 39.600, Matched: title.value, title, bio.value, bio
  3. Score: 39.600, Matched: title, title.value, bio.value, bio
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
  1. Score: 0.746
  2. Score: 0.739
  3. Score: 0.736

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.373, Matched: content.value, content, title.value, title
  2. Score: 1.369, Matched: title.value, content, title, content.value
  3. Score: 0.368, Matched: 

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
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: 4geZlFIOb2RYcDQaNgAS_WNtAsOCGyt10ZbbHZOKt9iY15zIRU-kR0qioQ_YYphWP3d-FtZ-qEiz-yNHnJTU8g

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: VLBBPC_lYrrpoyA98ORM3e-G4hdymW7Knr7TnI7ab-Zy7n2EM9AMAULb0dAH1e4WWh_BYSmgRRS2iJUMVegoBA

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
✓ Inserted document with TTL: wmCYO5hzvqgit2xgfQldPWWq2PHMufcqlgKCzONeGZclh-8AqnjNT6n7i58u3gq1IvX11bIUt8e1JfekKZtSRg

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
✓ Edge cache script created: vliMbRpmE7xse-M98tLrQWXOZcouO7ho4cCiq5SvAsZWfAvsWrjChS0_pPtnTBNWXnYrlVdRvaFKuJJKStclMA

Call 1: Cache miss (fetches from API)
Response time: 550ms
Result: {
  "records": [
    {
      "value": {
        "current": {
          "interval": 900,
          "temperature_2m": -6.7,
          "time": "2026-01-27T17:00"
        },
        "current_units": {
          "interval": "seconds",
          "temperature_2m": "\u00b0C",
          "time": "iso8601"
        },
        "elevation": 32.0,
        "generationtime_ms": 0.02646446228027344,
        "latitude": 40.710335,
        "longitude": -73.99308,
        "timezone": "GMT",
        "timezone_abbreviation": "GMT",
        "utc_offset_seconds": 0
      }
    }
  ],
  "stats": {
    "execution_time_ms": 0,
    "input_count": 0,
    "output_count": 1,
    "stage_stats": [],
    "stages_executed": 2
  }
}

Call 2: Cache hit (served from ekoDB)
Response time: 8ms (69.8x faster!)
Result: {
  "records": [
    {
      "value": {
        "current": {
          "interval": 900,
          "temperature_2m": -6.7,
          "time": "2026-01-27T17:00"
        },
        "current_units": {
          "interval": "seconds",
          "temperature_2m": "\u00b0C",
          "time": "iso8601"
        },
        "elevation": 32.0,
        "generationtime_ms": 0.02646446228027344,
        "latitude": 40.710335,
        "longitude": -73.99308,
        "timezone": "GMT",
        "timezone_abbreviation": "GMT",
        "utc_offset_seconds": 0
      }
    }
  ],
  "stats": {
    "execution_time_ms": 0,
    "input_count": 0,
    "output_count": 1,
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

✅ Script saved: Zu_5In1XQQgMxD6ZNu4tJRcR3J4jfulj5mLhGNqDJk8DW2sMt-CxloyiLG4KnkbPVjTQmH1VaybXdA8UOSqwGw
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved: nPJx1w1l6V9SthQ00fxDiUSaTx0DE34rggiufHim-I8hEgh_l0ECVSI4hxqkJjRF_MEU_c27FJLQWRdGBIQsZw
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: P1rdFuP02BRR19WmFNhbT46K0oMoGHoP7r317PvnSEoaBuwzYjrhePSU_nR_R1ovdNnZ1boGuXdSnoBoug1xwg
📊 Statistics: 2 groups
   {'avg_score': 50.0, 'count': 5, 'status': 'inactive'}

   {'avg_score': 60.0, 'count': 5, 'status': 'active'}

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
   ⏱️  Duration: 1.6ms
   📊 Records: 1
   📦 Data: {
      "value": {
            "address": {
                  "city": "Gwenborough",
                  "geo": {
                        "lat": "-37.3159",
                        "lng": "81.1496"
    ...

Second call (cache hit - from cache):
   ⏱️  Duration: 1.5ms
   📊 Records: 1
   📦 Data: {
      "value": {
            "address": {
                  "city": "Gwenborough",
                  "geo": {
                        "lat": "-37.3159",
                        "lng": "81.1496"
    ...
   🚀 Cache speedup: 1.1x faster!

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

✅ Script saved: 4uzlrGIgUjd084mUPWHR4pYLMY6FKex8ThLOT3KdRfSLMft3p4LiL4jKnQYy29g8YnSpIfTQ5bFpKm65J6L61Q
📊 Found 2 product groups
   {'avg_price': 575.6666666666666, 'category': 'Electronics', 'count': 3}
   {'avg_price': 474.0, 'category': 'Furniture', 'count': 2}
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

✅ Inserted order: q3TVdw4C9ARM2WSeDeNmtpUGjSZnObUqiOtSZKQPPil5trU4KiAeI0kpYolKeBFY62GLv3Zjaotk3Lr3-wad0Q
✅ Inserted 2 products with wrapped types

📝 Example 2: Querying and Extracting Wrapped Types

📊 Found 2 products
   • Laptop Pro
   • Wireless Mouse

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

✅ Inserted order: 8x8ycC5tTZOtJ8L_y5PCHHU22HQrpPhB0sGMZwMmo9oAAP_miKfGV98zNgTBSG6xtLj3bAa8Ci2rp5rAnzv9UA
✅ Cached order status
📊 Quick status lookup: {'value': '{"status":"processing","updated_at":"2026-01-27T17:13:15.099982+00:00"}'}

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
✓ Created SWR script: fetch_api_user_py (-90q8bDP9eg69_ezZjCXbVN3IVwyBqwwwSLnumxe1_hTF7hv9JMXRPa2ABOJh69Hsn36t-ukJOam8e1QWZcNXg)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
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
    }
  ],
  "stats": {
    "execution_time_ms": 0,
    "input_count": 0,
    "output_count": 1,
    "stage_stats": [],
    "stages_executed": 2
  }
}
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 3ms (served from cache)
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
   Vector databases provide benefits such as efficient data management, high precision, support for complex geometries, and better representation of data. They also allow for advanced spatial analysis and operations like buffering, overlay, and network analysis.
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
   {'avg_score': 70.0, 'count': 7, 'role': 'user'}
   {'avg_score': 20.0, 'count': 3, 'role': 'admin'}
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
   2. Vector Databases Explained (Database)
   3. Database Design Principles (Database)
   4. Natural Language Processing (AI)
   5. Getting Started with ekoDB (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Script saved
📊 Documents by category:
   {'category': 'Database', 'count': 3}
   {'category': 'AI', 'count': 2}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: wI1DCXeEsJdeUO_kt02FRPdNDwDBsstg9B7FWw2CKRTfOPdsGlg_28kcDF5_qt7kKB0d1QzG3ChFwb0qYkflbw

=== Sending Chat Message ===
Message ID: eu6ml_m3wvQlFLqfdIBSsHsPUNpHvrrfnFh4_QbZDSM7bSGcaCoSnrlJWVnFbf726WQx601Bt_Q8UASU9_d85g

=== AI Response ===
We have three products available:

1. ekoDB: This is a high-performance database product with AI capabilities. It is priced at $99.

2. ekoDB Pro: This is an Enterprise edition product with advanced features. It costs $299.

3. ekoDB Cloud: This is a fully managed cloud database service product. The price for this service is $499.

Execution Time: 3007ms

=== Token Usage ===
Prompt tokens: 454
Completion tokens: 78
Total tokens: 532

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: fB0OcnySBLkCHMXR2EULSWXBzNKJlIJ9faCu9Q8zOpE-RMyfmOYISovAyxuFNwj_FUyjSiMjusnMRyKTHqvngw

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information provided, there is a high-performance database product available named "ekoDB". It is priced at $99.

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['id', 'chat_id', 'context_snippets', 'content', 'role', 'updated_at', 'token_usage', 'created_at'])
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
✓ Created second session: sAzMtmJ-PQw2AMEuPiO1cVavd-TPGz-NdoWzNaWrQwQC5YnvBxHARHZQzeL1qVMGkgyKyhSGl-4SFyQ4Yp8K6w
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
✓ Created session: p4qIdxyzvVRIUsqZEFi6SQma6mUQhvAEuyfhx3AGbTcVcV_R_ZrS7yko8TuJm_H1dQAdckXP1b4GW6NMpUte0A

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product based on the provided context is a high-performance database product named "ekoDB". It is priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: GJ1ARwPMu_QT0nEgly5QyyjDbcc-2UuJXLtwzNeZX3uxYzApCK0xTA4XdXJbqowdQyOHW7B_d7MQc4mHH91VdA
  Parent: p4qIdxyzvVRIUsqZEFi6SQma6mUQhvAEuyfhx3AGbTcVcV_R_ZrS7yko8TuJm_H1dQAdckXP1b4GW6NMpUte0A

=== Listing Sessions ===
✓ Found 7 sessions
  Session 1: GJ1ARwPMu_QT0nEgly5QyyjDbcc-2UuJXLtwzNeZX3uxYzApCK0xTA4XdXJbqowdQyOHW7B_d7MQc4mHH91VdA (Untitled)
  Session 2: p4qIdxyzvVRIUsqZEFi6SQma6mUQhvAEuyfhx3AGbTcVcV_R_ZrS7yko8TuJm_H1dQAdckXP1b4GW6NMpUte0A (Untitled)
  Session 3: sAzMtmJ-PQw2AMEuPiO1cVavd-TPGz-NdoWzNaWrQwQC5YnvBxHARHZQzeL1qVMGkgyKyhSGl-4SFyQ4Yp8K6w (Untitled)
  Session 4: wI1DCXeEsJdeUO_kt02FRPdNDwDBsstg9B7FWw2CKRTfOPdsGlg_28kcDF5_qt7kKB0d1QzG3ChFwb0qYkflbw (Untitled)
  Session 5: -X3r6ftFEexwrKAvlzkZrkxeHh-JiNF87XeKMozCByA1f4epxPc0neizTsZUeZo--ymhrhF980pm_eMdeb0n3w (Untitled)
  Session 6: MFbKEwGka_F5zUqzfAjBoCmSLbdAGQnNUFluQGZQBVOpiWcYKxRFe4Oy3bwrvytTuJMcL4AkRMra-kseAeNFWA (Untitled)
  Session 7: O5WSPWnxYUn00jIQyOvwuAX84ZXyGYnF_HOnwe0EBVt-TTf9oMr7iziaWfFbk57WXUb3Xd3_RSNC0h1XSgmiSg (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: GJ1ARwPMu_QT0nEgly5QyyjDbcc-2UuJXLtwzNeZX3uxYzApCK0xTA4XdXJbqowdQyOHW7B_d7MQc4mHH91VdA

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Dict Creation ===
✓ Created record with plain dict: {'id': 'dQUCCI-U44P6uPArSYQ_uXQEPw9WV2ExOG4W_sONSowQg0RglX4IZzBmH_lnUV-vPVqKjw7eSaXqUIHfNqMWgg'}

=== Upsert Operation ===
✓ Upsert (update existing record): dQUCCI-U44P6uPArSYQ_uXQEPw9WV2ExOG4W_sONSowQg0RglX4IZzBmH_lnUV-vPVqKjw7eSaXqUIHfNqMWgg
✓ Inserted second record: xrk1BD75sCISZEfZ8V5TBklc1kEICSQEZfmmx6_haF3uxoJLAvzPh7794u98bIDEqYPGQP26-yuF4MAa0C65jQ
✓ Upsert (update second record): xrk1BD75sCISZEfZ8V5TBklc1kEICSQEZfmmx6_haF3uxoJLAvzPh7794u98bIDEqYPGQP26-yuF4MAa0C65jQ

=== Find One Operation ===
✓ Found user by email: {'name': {'type': 'String', 'value': 'Alice Johnson'}, 'age': {'value': 29, 'type': 'Integer'}, 'active': {'type': 'Boolean', 'value': True}, 'id': 'dQUCCI-U44P6uPArSYQ_uXQEPw9WV2ExOG4W_sONSowQg0RglX4IZzBmH_lnUV-vPVqKjw7eSaXqUIHfNqMWgg', 'email': {'type': 'String', 'value': 'alice.j@newdomain.com'}}
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
   Inserted with ripple: {'id': 'jZ6tjMlODoGh_3z0RBodlHSIoa0XwIq_yYNU6RCFSVj39Qw4Ip5ybezv-gVG9Yorz-KhTY4lI7QzLMXzL88WCg'}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {'id': 'or4A8KvvvDeFL7pxh2Z__Vg5GIzlNrWwt892Q0O49kUAYb83UKSV2Fg1lnjVrCFGBKW4YU38f8YNYKGR6z-Elw'}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {'name': {'type': 'String', 'value': 'Product 1'}, 'price': {'value': 150, 'type': 'Integer'}, 'id': 'jZ6tjMlODoGh_3z0RBodlHSIoa0XwIq_yYNU6RCFSVj39Qw4Ip5ybezv-gVG9Yorz-KhTY4lI7QzLMXzL88WCg'}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {'price': {'value': 500, 'type': 'Integer'}, 'name': {'type': 'String', 'value': 'Upsert Product'}, 'id': 'jZ6tjMlODoGh_3z0RBodlHSIoa0XwIq_yYNU6RCFSVj39Qw4Ip5ybezv-gVG9Yorz-KhTY4lI7QzLMXzL88WCg'}

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
Fetched user profile: {'value': 'Alice Johnson', 'type': 'String'}

Example 5: Compare full vs projected data
Full query returned 12 fields per user
Projected query returned 12 fields per user

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
✅ [32mPython client examples complete![0m
