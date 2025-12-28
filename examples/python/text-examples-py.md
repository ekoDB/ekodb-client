make test-examples-python
🧪 [36mRunning Python examples (direct HTTP/WebSocket)...[0m
✓ Authentication successful

=== Insert Document ===
Inserted: {'id': '5chMg3FBZhiEHLOM9a_RIArVq6TZQ-RYh07mEWI4q4LqF16QdsGm4I7XpvIzx6ODQobMpTsbaGpRhbTZCj8-BA'}

=== Find by ID ===
Found: {'active': {'value': True, 'type': 'Boolean'}, 'value': {'type': 'Integer', 'value': 42}, 'name': {'value': 'Test Record', 'type': 'String'}, 'id': '5chMg3FBZhiEHLOM9a_RIArVq6TZQ-RYh07mEWI4q4LqF16QdsGm4I7XpvIzx6ODQobMpTsbaGpRhbTZCj8-BA'}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'name': {'type': 'String', 'value': 'Updated Record'}, 'id': '5chMg3FBZhiEHLOM9a_RIArVq6TZQ-RYh07mEWI4q4LqF16QdsGm4I7XpvIzx6ODQobMpTsbaGpRhbTZCj8-BA', 'value': {'type': 'Integer', 'value': 100}, 'active': {'type': 'Boolean', 'value': True}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: Kd7MuNthn11mq4fPbeELKVfNy-FciWdlj1aoIA441hvPMlKU0ZCKc0mBikFpaf1fbEYf6jXXevgjFsdeggqXNQ

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
        "id": "uNNfPR9zCQ_Wj1E21yvsz6eduv194YhIWwL8E2fBMxqSqmMVPCRQjSoBSn6ipWjaTp5S-QAoqOl_dwGFXKUdCw",
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
        "id": "Kd7MuNthn11mq4fPbeELKVfNy-FciWdlj1aoIA441hvPMlKU0ZCKc0mBikFpaf1fbEYf6jXXevgjFsdeggqXNQ",
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
Retrieved value: {'username': 'john_doe', 'userId': 123}

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: {'name': 'Product 1', 'price': 29.99}
cache:product:2: {'name': 'Product 2', 'price': 39.989999999999995}
cache:product:3: {'price': 49.989999999999995, 'name': 'Product 3'}

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: 1f9b6fdqxtOzPnlVxKo7lYhgt2iw9itioreUTcxwz-eq5ebT1P9UygTFTpwwxci_ZHYFG9EY_nSDqNR_aFlvnQ

=== List Collections ===
Total collections: 11
Sample collections: ['batch_users', 'test_collection', 'chat_configurations__ek0_testing', 'users', 'chat_messages__ek0_testing']

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: False

✓ All collection management operations completed successfully
✓ Authentication successful

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: VfPhyagjMcKIMASkVin6ICioZ07poS-WJJvu_rrWW8sbVLucnjVJfpvmJM4wlMLhNMhKHUZKnCtHyJ6bHeXZEQ

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
✓ Inserted document with TTL: ufaGNf2T7vfO6S13_KTH1_47RSvovcFsXVz_vHkY33E_HT2A1yak9hwOMu74zvDT0T3msV_DWHYVrrkUYdkLpw

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 2 record(s) via WebSocket

✓ WebSocket TTL example completed successfully
🚀 ekoDB Scripts Example (Python/HTTP)

✓ Authentication successful

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: C8d_K9sd3GSQmqvpJgoiyDPod3JTWpZBwRcRNgsJtDW7AQBqwTOPfDrgmIroBLRi_0qLUJh7t36xpCUB4As2rA
📊 Found 15 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: BKtCSovW84FhcFc_KeuB68VO3nzXWOqolExk9mG8bOpv7x7urayKLERFJIMthExSZUiHCqW8v29Qw7BmvB5BqA
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: VtMJja3BTUslQwEjyTgU3vQIzIx2cRh36eBznwkUmItDxqrsHciqVCp_WQDl9q-jSVfQ06_Iy7Ra11PwmGIxpQ
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {'max_score': 90, 'status': 'inactive', 'avg_score': 50.0, 'count': 15}
   {'count': 15, 'avg_score': 60.0, 'status': 'active', 'max_score': 100}

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
Created Alice: $1000 - ID: QW4eXfYoTZvYKtzTnjLdMp3XdRFF187p_pqzA1zM2aW6YiS7jHnrxjPEPQzGdFTBheRiFDIhdWxw1kbxoLoaQw
Created Bob: $500 - ID: Mv0586kPvqSeJl62A9pgqasZBXLbtjx6JA33voTe4Q-IZijP5zdbWWfiJQo_7-42UDNcmh0Jjlt7ALEGJnng5w

=== Example 1: Begin Transaction ===
Transaction ID: ab9039de-ddc5-45e5-96d8-2b632ba82957

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
New transaction: 71ccef37-6e01-4a21-be01-7c403c8616b0
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: u7Gb2Sn9-7c-DHYlrNAuKoMzIAXIJIWgR0yvCYyRVcyRXLHn4C-oN4PB78wM2F3aiO9SoOEe4E2u0r9h9TEQKg
Created Bob: $500 - ID: e9KY3FbjZB6xUyCKWsCHcIx0A2ECQqOpI9UH2V4CZ3SANxDcNsQbdao36xELd5-0Ajr12DDVaF6lZgt2kXpnQg

=== Example 1: Begin Transaction ===
Transaction ID: c08dffd7-d26d-4c3f-b0c4-bbccbdb75510

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
Created Alice: $1000 - ID: KQsmvZEDG-k1OK5FJb8b1lJW08Sw8V5zhI8JHdbzsjK9tYJN6KfK3ExEaA-sCMRWgmoHwSKymILnvq7OCP4wEg
Created Bob: $500 - ID: XaRPYEJwihgnVI26SSjf-bF_YHuCzHfypi_d66V6zIhTMuo5RpsJWTRYBAKwBEFg1dcK0gKYYWnGNSg9OuhfPw

=== Example 1: Begin Transaction ===
Transaction ID: a367e9a6-a5f9-4688-95ab-4db5c2414abc

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
New transaction: a06d9bdb-ab54-47ea-910b-8ed2c201e11b
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: slIIWa7SbiNEMuJsEy-DSkzCd-TXqPYcndR-73IuntnVfRkYcapYe1kJy-DTaYrxTZjdV2FTuHye8WiOkDoj6g
Created Bob: $500 - ID: OdX6-NlUZ2nwNR1G2HFgkq-2QMaCXHKcuw6KvZ9GDg-13tiB0mVLGL0OsWb9M8z5_iUXJUHYYdF5I-lAdlahKA

=== Example 1: Begin Transaction ===
Transaction ID: c25f7999-2f30-47a8-ac46-36f19516ce86

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
Created Alice: $1000 - ID: eN5cz7BfxWsjtNLldhPDq0ieecCBDrnWlR-glSdk5K_LFkbUD_r0CwCgcuasjkh0HbcUOXyAEtOldGLPTCNedQ
Created Bob: $500 - ID: W9DAfBqZ4oq03wuZxXcUAxxdN2rGg520GzeTGANVj5XC79A7qWFEbqu8HCi20t-RkI98-2Hqo-ALbTy1zH-YTQ

=== Example 1: Begin Transaction ===
Transaction ID: 8f0c4d42-99a6-420b-9812-bc4e47b2073c

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
New transaction: e1c058a4-8937-4d53-932d-96b738d5c8c8
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: coUAlSgkXqlG1smA8MQAYaPivz_PDYL9Bzdznv1rnZ3kldAYbvgxvAaGOpFN9GnrWHVtUeCThoVF0pFKVknjjw
Created Bob: $500 - ID: kUEm5nJHXRbBZ4BafSBRgEFXkYUkza_HsMCeA_ZOi7M9KhifIokdrftIkHvGH3DrIuwpt6b-QdRayy8eDroKDw

=== Example 1: Begin Transaction ===
Transaction ID: 12f6a053-8c2a-4013-9456-3ada9e5c9c98

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
   ✅ Script saved: 6UseiYyeIMC7d_UnJRWeDPaupd8x0wWb0EB5Qd0fjXl8yEkFqPxWq3KCVnN7qK7RYpZnv7RZYMbaWDzZpjhmVA

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: hjS3oC0q9i9gGgJA5L5eq90LMz06uXujmA6cNPEuzhQNIdTtuFX6gFqt_cpTfhnNuviJ5SLB0g2ma_xjaJavMQ
   📋 Name: {'value': 'Alice Smith', 'type': 'String'}
   📋 Email: {'value': 'alice@example.com', 'type': 'String'}
   📋 Status: {'type': 'String', 'value': 'pending'}
   📋 Credits: {'type': 'Integer', 'value': 0}

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: m2MutAC5UEniAW6TsOpwStxO0vW2mXBnVB8kV8VLXCXNg5LWgx1ERt45vqD_HciwoWr7yYxBqZ5y7FvevoMcKQ

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
   ✅ Script saved: 8v5HPcklLwwO5ebmIDBmbyG9AyMHx3YTzoIXQdO36IZYH10oWNdw5--6IKLMPoi5Cj02DvA13ZnH0C0MnvTorA

2️⃣ Calling Script (Query + Update Credits + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {'type': 'Integer', 'value': 0}
   📋 Status: {'value': 'active', 'type': 'String'}
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}

============================================================
📝 Script 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: NlhrnLqQUJTqNVUwY56qQBjkjgeSDLwfIqPUuuP-QcbfaVFbrMk869PBf9_vuQPx4TS6hjfqfxbdECav3wnSLg

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: 6UseiYyeIMC7d_UnJRWe...
   ✅ Deleted script: m2MutAC5UEniAW6TsOpw...
   ✅ Deleted script: 8v5HPcklLwwO5ebmIDBm...
   ✅ Deleted script: NlhrnLqQUJTqNVUwY56q...
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
    Finished `release` profile [optimized] target(s) in 0.15s
📦 Built wheel for abi3 Python ≥ 3.8 to /Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.5.0-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel for current platform...[0m
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.5.0-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
Successfully installed ekodb-client-0.5.0
✅ [32mPython client package built and installed![0m
🧪 [36mRunning Python client library examples...[0m
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {'id': 'd-CD9ir8Uffp2EiQaISWqZaESFkdCRIhDcctoKH-bgs5uCtV4iI1n8feFZEgOfJRz3MTyKSovD1Usx2OoicIRA'}

=== Find by ID ===
Found: {'value': {'type': 'Integer', 'value': 42}, 'id': 'd-CD9ir8Uffp2EiQaISWqZaESFkdCRIhDcctoKH-bgs5uCtV4iI1n8feFZEgOfJRz3MTyKSovD1Usx2OoicIRA', 'name': {'value': 'Test Record', 'type': 'String'}, 'active': {'value': True, 'type': 'Boolean'}}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'active': {'value': True, 'type': 'Boolean'}, 'value': {'value': 100, 'type': 'Integer'}, 'id': 'd-CD9ir8Uffp2EiQaISWqZaESFkdCRIhDcctoKH-bgs5uCtV4iI1n8feFZEgOfJRz3MTyKSovD1Usx2OoicIRA', 'name': {'type': 'String', 'value': 'Updated Record'}}

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
Collection created with first record: "MtT5zy7pNP-xIqlJq7kdcmzg6lqkrtnotoC5CuKUBZWT5-5lYvUqsjzueLpcom2M-V1RE4oXiPCo-JCQZxgcvw"

=== List Collections ===
Total collections: 10
Sample collections: ['client_collection_management_python', 'batch_users', 'test_collection', 'chat_configurations__ek0_testing', 'chat_messages__ek0_testing']

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: False

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: foqK5T2JwGkDK8wrybjVryiEGgyrwrIs4io7c0NCvR6rtBrCvomOWRFCuGZ-QQaNaSlZ_ftRsXe_FyTXKHBm1Q

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: P5dDd5YPFBUQJcJBL3J5SHGDmz7AWMfMwY30CGxEu157tkcPJYepdh3fghWEz3QFakWB7T4NR5rIQNL8jkNUpw

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
✓ Inserted test record: uoEugP5sLCWj-v4CEUyWih52evrcw750-plwClxug-CT1g59fCn848a1HSWwesowvVafLiwcROldh4eGindRtw

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
✓ Inserted document with TTL: prx_XHvMKPoclRlL6m9kJx4HyaAYoGTk_JR9rUM_-O88SHHHsNxI2QS7Y_2DHQJ_ZbqK4Wd7Uw2By3gAHxovfA

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
  1. Score: 25.740, Matched: name, email, email.value, name.value
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: bio, bio.value, title, title.value
  2. Score: 26.400, Matched: bio.value, title.value, title, bio
  3. Score: 26.400, Matched: title, bio.value, bio, title.value
  4. Score: 26.400, Matched: title, title.value, bio, bio.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio.value, bio, title, title.value
  2. Score: 39.600, Matched: title, title.value, bio, bio.value
  3. Score: 39.600, Matched: title.value, bio.value, title, bio
  4. Score: 39.600, Matched: title.value, bio.value, title, bio

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
  1. Score: 0.767
  2. Score: 0.760
  3. Score: 0.745

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.732, Matched: title.value, content, content.value, title
  2. Score: 1.121, Matched: title, title.value, content, content.value
  3. Score: 0.537, Matched: 

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
  - Laptop ($1200) by Alice Johnson
  - Mouse ($25) by Alice Johnson

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
✓ Created session: _1CPODs-8nj298P5KnJz-t7BEQVsQOo6EyCog69m8gTNz_pANx0bruuIZ7qnAoLYTGBamNY1ZO3vaCNcVPD0Wg

=== Sending Chat Message ===
Message ID: bA46-Y5k5Axi3TvrxVLfvAdLmDMyjl73YIuFD__xukFJ91XaFnT1MIf84W5zhnSiEZxHO7sxuO15KeywjY0aqQ

=== AI Response ===
There are three products available:

1. ekoDB: This is a high-performance database product with AI capabilities. The price for ekoDB is $99.

2. ekoDB Pro: This is an enterprise edition product with advanced features. The price for ekoDB Pro is $299.

3. ekoDB Cloud: This is a fully managed cloud database service product. The price for ekoDB Cloud is $499.

Execution Time: 2824ms

=== Token Usage ===
Prompt tokens: 609
Completion tokens: 89
Total tokens: 698

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: wm2yusSL9We4iCWtVQ8OgiRhxOTYG68-HqstgTHWeEx7_p2tmw4Wp59ymCN4bMg5hHgv9tlwAeuZZYkvGMQJiQ

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information available, we have the ekoDB, which is a high-performance database product. It is priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 9mMiWkgfMMCkgBgT1_kIRwdebmhWFhfSXVMXm6iTGiykGfhD-CYtLoT_K2R92YpC3EI0VeGLF7YpXY4CMDTQTQ
  Parent: wm2yusSL9We4iCWtVQ8OgiRhxOTYG68-HqstgTHWeEx7_p2tmw4Wp59ymCN4bMg5hHgv9tlwAeuZZYkvGMQJiQ

=== Listing Sessions ===
✓ Found 6 sessions
  Session 1: 9mMiWkgfMMCkgBgT1_kIRwdebmhWFhfSXVMXm6iTGiykGfhD-CYtLoT_K2R92YpC3EI0VeGLF7YpXY4CMDTQTQ (Untitled)
  Session 2: wm2yusSL9We4iCWtVQ8OgiRhxOTYG68-HqstgTHWeEx7_p2tmw4Wp59ymCN4bMg5hHgv9tlwAeuZZYkvGMQJiQ (Untitled)
  Session 3: _1CPODs-8nj298P5KnJz-t7BEQVsQOo6EyCog69m8gTNz_pANx0bruuIZ7qnAoLYTGBamNY1ZO3vaCNcVPD0Wg (Untitled)
  Session 4: hy9LUO2nl-RZUbTZUgrqYDj1mubOHylpZk3swm78FojI3ne4KfoLyDAjFLBhNK3WVW37L9ChwSPfRWPCGo3Ezg (Untitled)
  Session 5: GfXPY1xW_iSnX8H5hW76Z1_5bILggPqR4oM7Ap3fqJKBs31el-N2GY98rtL2-lOX7PCZ27GZhqT72QRHcbW-Rw (Untitled)
  Session 6: GnurYMIVmn2xtI9pusFTuqQgQau7C3-kRSIfPGO4vyRy9X02Wz0z5Z9WVCW6GeXPqSHyE8Vz98kCZRoZPeD1xw (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: 9mMiWkgfMMCkgBgT1_kIRwdebmhWFhfSXVMXm6iTGiykGfhD-CYtLoT_K2R92YpC3EI0VeGLF7YpXY4CMDTQTQ

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: ysATYJhDjp7xGC3dSD5lizyRhizW8zn9sRFqfgQu5E3179aPG6a0U29qFsZZeh9WtAKvgVhUeN5dx0QzlHtm3Q

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information provided, the product available is "ekoDB". It's a high-performance database product priced at $99.

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['context_snippets', 'id', 'chat_id', 'token_usage', 'created_at', 'updated_at', 'content', 'role'])
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
✓ Created second session: KVh-vuJkxdu82QEem9mEJqlsehZ-MC_e1SnOJ7Xgd_lrHTjQZhpjlMgkO11L2jZt3heiLfxZmTU0K21jAkDofA
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

✅ Script saved: h3Znj1FcDpiO7_YSwvyYS8hrhat_z7YkN-lBjLn5zl9nQRrlrZZ87n8sU2scjXJ_PulBcRtnA4jPhgR3rcSm9g
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved: 0TtruoLAc-4TAn-D_M6MG8HpBYTON-negELMqGvpwKFkb96SCNI6-XAYcrsAjw468TaX69tmtP2NeC2iq1nAAw
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: QQIZQYxV5Vz4F9NnBLMCIWLzrTCK3koSdQnlLSgq8cJbBAnXxJLr_Q2mC7rDFrUMlLw_huvNyZUbbEVXRpr0sA
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

✅ Script saved: nDYipfawWk_loiBdsqQb4a9Ax-Uf9ra4ROxd0Tf6osk9uPMW5C_92WWWwNNpM_ZrB5RbtEVCvLgwdQcd9n58hQ
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
