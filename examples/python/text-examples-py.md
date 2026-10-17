make test-examples-python
📦 Ensuring Python example dependencies in .venv...
🧪 Running Python examples (direct HTTP/WebSocket)...
=== Simple CRUD Operations (Direct HTTP) ===

✓ Authentication successful

=== Insert Document ===
Inserted: {'id': 'y5y8P1XKid63YlC0a6arE1MUg3Wxh9X5g1SkYN--_irWMWQaWZggpza7z8TN0jFrQT-p7LRPZrT6gkPPZb0fwg'}

=== Find by ID ===
Found: {'value': {'type': 'Integer', 'value': 42}, 'id': 'y5y8P1XKid63YlC0a6arE1MUg3Wxh9X5g1SkYN--_irWMWQaWZggpza7z8TN0jFrQT-p7LRPZrT6gkPPZb0fwg', 'active': {'value': True, 'type': 'Boolean'}, 'name': {'type': 'String', 'value': 'Test Record'}}

=== Find with Query ===
Found documents: [{'name': {'type': 'String', 'value': 'Test Record'}, 'active': {'type': 'Boolean', 'value': True}, 'value': {'type': 'Integer', 'value': 42}, 'id': 'y5y8P1XKid63YlC0a6arE1MUg3Wxh9X5g1SkYN--_irWMWQaWZggpza7z8TN0jFrQT-p7LRPZrT6gkPPZb0fwg'}]

=== Update Document ===
Updated: {'name': {'type': 'String', 'value': 'Updated Record'}, 'id': 'y5y8P1XKid63YlC0a6arE1MUg3Wxh9X5g1SkYN--_irWMWQaWZggpza7z8TN0jFrQT-p7LRPZrT6gkPPZb0fwg', 'value': {'type': 'Integer', 'value': 100}, 'active': {'type': 'Boolean', 'value': True}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: IjIB7yLYrTRr3IqAzcyizJ2CAHvZbEMy1XjT3rtDNsjq3GdzOKEXzgTKMJRXnmb8sdPOBfz6zBcTFm3HvyFHmQ

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "type": "Success",
  "payload": {
    "data": [
      {
        "value": {
          "value": 42,
          "type": "Integer"
        },
        "id": "IjIB7yLYrTRr3IqAzcyizJ2CAHvZbEMy1XjT3rtDNsjq3GdzOKEXzgTKMJRXnmb8sdPOBfz6zBcTFm3HvyFHmQ",
        "active": {
          "type": "Boolean",
          "value": true
        },
        "name": {
          "value": "WebSocket Test Record",
          "type": "String"
        }
      }
    ]
  },
  "messageId": "1847759617"
}
✓ Retrieved 1 record via WebSocket

✓ WebSocket example completed successfully
🚀 ekoDB Functions Example (Python/HTTP)

✓ Authentication successful

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Function saved: XfsA6SC8Bm-Qiw9hmBem9Zz411suiu7vEEafSptHsmXPvNJfk9mT1KbWpWl9omMliGOl_UGG7fV3-y9hOB5gSw
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Function saved: Ge-EcNy1isFw9I38XyJ_I8nlnEWJLcDavX9skUfmk6wZLuP3Up5GZWncBMVSiM3BXNG9VrdqHTQqJPFIzE_jeg
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: IJlsrYZE8TuKpMoqOBesNrx4J2g9bpTcxJMJJXwRLiE1Vz_7dXk60r_jejSTCsPf2_Q8V8NwphghLykVhqZ3xw
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {'status': {'type': 'String', 'value': 'active'}, 'max_score': {'value': 100, 'type': 'Integer'}, 'avg_score': {'value': 60.0, 'type': 'Float'}, 'count': {'type': 'Integer', 'value': 5}}
   {'avg_score': {'value': 50.0, 'type': 'Float'}, 'max_score': {'type': 'Integer', 'value': 90}, 'status': {'type': 'String', 'value': 'inactive'}, 'count': {'value': 5, 'type': 'Integer'}}

📝 Example 4: Function Management

📋 Total scripts: 3
🔍 Retrieved script: Get Active Users
✏️  Function updated
🗑️  Function deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

✅ All examples completed!
=== Batch Operations (Direct HTTP) ===

✓ Authentication successful

=== Batch Insert ===
✓ Batch inserted 5 records
✓ Verified: Found 5 total records in collection

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
Retrieved value: {'type': 'Object', 'value': {'username': 'john_doe', 'userId': 123}}

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
kv_operations:direct:py:cache:product:1: {'type': 'Object', 'value': {'name': 'Product 1', 'price': 29.99}}
kv_operations:direct:py:cache:product:2: {'type': 'Object', 'value': {'name': 'Product 2', 'price': 39.989999999999995}}
kv_operations:direct:py:cache:product:3: {'value': {'price': 49.989999999999995, 'name': 'Product 3'}, 'type': 'Object'}

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
=== Collection Management (Direct HTTP) ===

✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: IUE2MTLXM8GJ8Ckia3W3z34sSBNCQX4OI4Y-lDjO51hfU2dn3B5l3C5zT-TnUETMCH4EC8cbiBpUzxsF6jyP9w

=== List Collections ===
Total collections: 13
Sample collections: ['chat_goals__ek0_testing', 'chat_messages__ek0_testing', 'schedules__ek0_testing', 'chat_raw_completions__ek0_testing', 'chat_agent_configs__ek0_testing']

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
Created Alice: $1000 - ID: sl9W8LqkD73HDYpSy3eFnyd20e_vklpjQEtga7rhYXUTK08FoV2-Y5IrVrdPkfaAQYwZIeQiyAKuMoyEso5c_Q
Created Bob: $500 - ID: lOyd0EzLzFZqZTYLZob0MzWUSgmjyIjZFJrVuGfQuJQ6hogXA1lceokMiI2JBfydyL4iRXTOu6OASlekwks_vw

=== Example 1: Begin Transaction ===
Transaction ID: 971da3e0-1fcc-4616-8a19-cef2ca130181

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: ${'type': 'Integer', 'value': 800}
Bob: ${'type': 'Integer', 'value': 700}

=== Example 5: Rollback ===
New transaction: a9c1cf1a-3c1b-432f-8c8a-1dd2d6796883
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'type': 'Integer', 'value': 700}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: K5QueZ3ik7-_8dIDcJTAl_okPH678QlXFxCGLnw7SOUzPvUieqjBHA8qPVNdzDnptupUCEZdhlf60I2CHHbRLg
Created Bob: $500 - ID: unoteICJmL6d_CMtxQI_qYwywzhkU9dHJe4OapOyhJZPAzBAEG_WYDWq1CMk45ddrw9C0dmebNoo9XDF74Mwnw

=== Example 1: Begin Transaction ===
Transaction ID: f6a3ac1c-23c8-4101-a651-680402acacf4

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: ${'type': 'Integer', 'value': 800}
Bob: ${'type': 'Integer', 'value': 700}

=== Cleanup ===
✓ Deleted test accounts


🔄 Switching to NON-DURABLE mode...
   ✓ Config updated: durable_operations=false


============================================================
🔥 TEST 2: Non-Durable Mode (durable=false)
============================================================

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 31mlY4iVos5aRqnbYxxupUd960wZK--efhMteihP0hSeloAcXCUKRsj5SniYaVpvQMRS_7kMdwoK-sZcGsBmbw
Created Bob: $500 - ID: j9YDnyg-oH-qmxbz3iLWMBl3pP5AcH3Hd1a-Ra1vZZMM6h81dFEvRIwZQGxLmuLk7wIB5vfUCROROjaYrLCwcQ

=== Example 1: Begin Transaction ===
Transaction ID: 35204f5a-f563-4084-9ae3-87d022e568d5

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: ${'value': 800, 'type': 'Integer'}
Bob: ${'value': 700, 'type': 'Integer'}

=== Example 5: Rollback ===
New transaction: f16bfbb8-b1b8-4e48-855e-01f70e4a05ae
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'type': 'Integer', 'value': 700}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: vF7Ix6fi9XblqAmbnrf7jvraJlLs2NpbugN6ajXPX0_E2vLear11Ftxmbd9jrkrtKuFYWUeHlR8tPh53o8xJrw
Created Bob: $500 - ID: 6I-6HYyQP-JMUYwsrCTyy23jgSSWWLHAO8d8eKZiygUr8W0vVZ0ztMLdP3lSlWZ1h8Qi_d8oM4WLD9FebhvTbA

=== Example 1: Begin Transaction ===
Transaction ID: eee68f08-5511-4383-877a-c464092f4dd1

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: ${'value': 800, 'type': 'Integer'}
Bob: ${'value': 700, 'type': 'Integer'}

=== Cleanup ===
✓ Deleted test accounts


🔄 Switching to DURABLE mode...
   ✓ Config updated: durable_operations=true


============================================================
🔥 TEST 3: Durable Mode (durable=true)
============================================================

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: vlaCmiTzhaWwMelCtWL7dywOoujARYoPnVX4Wl1mpcgBTxDxe87l-Kr53Ewx2nJNMLjC2UJvo-aWAyQf14Hubw
Created Bob: $500 - ID: F_WcxbNrU_8-VCN0b7W2sLJCDsoeIKhtM1fk_SPs2O9T2SeWN7cglnPgUjABxNfR3FKFWv9rIgXmAgNJXHCSOw

=== Example 1: Begin Transaction ===
Transaction ID: c81e7ecd-b2d1-4f41-bc56-ba40762f725b

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: ${'type': 'Integer', 'value': 800}
Bob: ${'type': 'Integer', 'value': 700}

=== Example 5: Rollback ===
New transaction: 683a4b3d-c781-4a36-808e-66672e44d12a
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'value': 700, 'type': 'Integer'}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: pLWrof2r7YmRF_GlUUMd6-lWskiFdVT7tWSexumxwLlJBjg0iLzE9n2smi2X2tT02rkNjPXcSR-10885gVkgAQ
Created Bob: $500 - ID: YbqnPqEiLuU88CH2Zf3JK7DWc4ErebxB5qz_zBxjHPyyxKH21ViNnRio4itZXTwHZlDPB68ZiX6rgu_tyXyJ5g

=== Example 1: Begin Transaction ===
Transaction ID: d40516a2-84ea-44f1-8066-db8d0f120b43

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: ${'value': 800, 'type': 'Integer'}
Bob: ${'type': 'Integer', 'value': 700}

=== Cleanup ===
✓ Deleted test accounts


🔄 Restoring original configuration...
   ✓ Config restored: durable_operations=True


============================================================
✅ ALL TESTS PASSED - Transactions successful
============================================================

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
🧹 Cleanup
============================================================
   ✅ Deleted collection: crud_functions_users_py
============================================================
📝 function 1: Insert + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: fflcCbalndEO89Sx9k9qdbtgYHuJTicPivdFdB9dqQE9Gleciq2mQdf_mPxYfowG9_gIQnxcqLkREOSIYU3xQQ

2️⃣ Calling function (Insert + Verify)...
   ✅ function executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: tYOvfSJPeBRioJOCdb3tLansZp0RNbn7T2ShZ-3bMPeOkKuInRSiKF4ouZ-UTwkrukWnAF22SUxFvZYY3d_h-A
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}
   📋 Email: {'type': 'String', 'value': 'alice@example.com'}
   📋 Status: {'type': 'String', 'value': 'pending'}
   📋 Credits: {'value': 0, 'type': 'Integer'}

============================================================
📝 function 2: Query + Update + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: 1YvuxzMXFVGj1L8OlcJo_L368zlVz18-QMDqOa19IL2y2XkFq7x2F67w2ceMI1xBdTYlVEGLkHBA7vKXFnp5Hg

2️⃣ Calling function (Query + Update + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Status updated to: {'type': 'String', 'value': 'active'}
   📋 Name: {'value': 'Alice Smith', 'type': 'String'}

============================================================
📝 function 3: Query + Update Credits + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: UrQFEei8u5VPKSPRXMbQ3yxBRLhQJkMtgTY2JE1of-ZHulFsyhSYolaqlGY5aN77TDidY_VtLg6tMt1-CyNv_g

2️⃣ Calling function (Query + Update Credits + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {'value': 100, 'type': 'Integer'}
   📋 Status: {'value': 'active', 'type': 'String'}
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}

============================================================
📝 function 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: 51yfZCOAynnuqq60qqxb9pSWznrAwzLKvgJWTTOQsx02h2YIfCTTKfdBuccZTqXAl-NDKEG5Bu8nNrJLs5i7sA

2️⃣ Calling function (Query + Delete + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: fflcCbalndEO89Sx9k9q...
   ✅ Deleted script: 1YvuxzMXFVGj1L8OlcJo...
   ✅ Deleted script: UrQFEei8u5VPKSPRXMbQ...
   ✅ Deleted script: 51yfZCOAynnuqq60qqxb...
   ✅ Deleted collection: crud_functions_users_py

============================================================
✅ Complete CRUD Functions Example Finished!
============================================================

💡 Key Takeaways:
   ✅ Functions chain Functions together
   ✅ Each function demonstrates operation + verification
   ✅ Parameters make functions reusable
   ✅ Verification is built into the function itself
   ✅ Complete CRUD lifecycle in 4 focused functions
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
  Output: Document ID = qXs1YEnrTYR7o0_7Uso9M31MNgcJsRGeduLxYnOihJc9kGVI3FPbQc5ekDiNh2h_5QBZrAcm-dcXgDcTPU-_fA
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: find_by_id(qXs1YEnrTYR7o0_7Uso9M31MNgcJsRGeduLxYnOihJc9kGVI3FPbQc5ekDiNh2h_5QBZrAcm-dcXgDcTPU-_fA)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: find_by_id(qXs1YEnrTYR7o0_7Uso9M31MNgcJsRGeduLxYnOihJc9kGVI3FPbQc5ekDiNh2h_5QBZrAcm-dcXgDcTPU-_fA)
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
  Output: Document ID = GRa59bhFgU0OIAiE4pKqT-yZTeBj_OPEeNn9aUExQc5gE-xhr1nXPWovVWT8TnwdtELu3ElfXdH9_ovho_1Kow
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: find_by_id(GRa59bhFgU0OIAiE4pKqT-yZTeBj_OPEeNn9aUExQc5gE-xhr1nXPWovVWT8TnwdtELu3ElfXdH9_ovho_1Kow)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: find_by_id(GRa59bhFgU0OIAiE4pKqT-yZTeBj_OPEeNn9aUExQc5gE-xhr1nXPWovVWT8TnwdtELu3ElfXdH9_ovho_1Kow)
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

╔════════════════════════════════════════╗
║   ekoDB Python Examples Test Suite    ║
╚════════════════════════════════════════╝

=== Checking Server Connection ===
✓ Server is ready

=== Getting Authentication Token ===
✓ Authentication successful

=== Running 10 Examples ===

=== Running simple_crud.py ===
✓ simple_crud.py completed successfully

=== Running simple_websocket.py ===
✓ simple_websocket.py completed successfully

=== Running http_functions.py ===
✓ http_functions.py completed successfully

=== Running batch_operations.py ===
✓ batch_operations.py completed successfully

=== Running kv_operations.py ===
✓ kv_operations.py completed successfully

=== Running collection_management.py ===
✓ collection_management.py completed successfully

=== Running transactions.py ===
✓ transactions.py completed successfully

=== Running crud_functions.py ===
✓ crud_functions.py completed successfully

=== Running document_ttl.py ===
✓ document_ttl.py completed successfully

=== Running websocket_ttl.py ===
✓ websocket_ttl.py completed successfully

╔════════════════════════════════════════╗
║           Test Summary                 ║
╚════════════════════════════════════════╝
Total: 10
Passed: 10
Failed: 0
✅ Python direct examples complete!
🐍 Building Python client package...
🔧 Ensuring maturin is available in .venv...
🔨 Building wheel...
🍹 Building a mixed python/rust project
🐍 Found CPython 3.11 at /Library/Frameworks/Python.framework/Versions/3.11/bin/python3
🔗 Found pyo3 bindings with abi3-py3.8 support
💻 Using `MACOSX_DEPLOYMENT_TARGET=11.0` for aarch64-apple-darwin by default
    Finished `release` profile [optimized] target(s) in 0.21s
📦 Built wheel for abi3 Python ≥ 3.8 to ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.30.0-cp38-abi3-macosx_11_0_arm64.whl
📦 Installing Python wheel into .venv...
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.30.0-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.30.0
    Uninstalling ekodb_client-0.30.0:
      Successfully uninstalled ekodb_client-0.30.0
Successfully installed ekodb-client-0.30.0
🧪 Ensuring test dependencies (pytest) in .venv...
✅ Python client package built and installed!
=== ekoDB Advanced CRUD Example (Python) ===

--- insert ---
Inserted: DFXu_oRwFE_9a0TRrK8mmxs9wP1q9j8T_tTpFWzLw3a59o6fA4UPMMtsfm__B-4QDqfp7EDfrRuy9muigBVDVg

--- update_with_action (increment) ---
After increment count by 5: {'score': {'type': 'Float', 'value': 50.0}, 'tags': {'type': 'Array', 'value': ['initial']}, 'name': {'type': 'String', 'value': 'Counter Record'}, 'id': 'DFXu_oRwFE_9a0TRrK8mmxs9wP1q9j8T_tTpFWzLw3a59o6fA4UPMMtsfm__B-4QDqfp7EDfrRuy9muigBVDVg', 'count': {'type': 'Integer', 'value': 15}}

--- update_with_action (decrement) ---
After decrement score by 10: {'name': {'type': 'String', 'value': 'Counter Record'}, 'tags': {'value': ['initial'], 'type': 'Array'}, 'id': 'DFXu_oRwFE_9a0TRrK8mmxs9wP1q9j8T_tTpFWzLw3a59o6fA4UPMMtsfm__B-4QDqfp7EDfrRuy9muigBVDVg', 'count': {'value': 15, 'type': 'Integer'}, 'score': {'type': 'Float', 'value': 40.0}}

--- update_with_action_sequence ---
After action sequence: {'name': {'value': 'Counter Record', 'type': 'String'}, 'id': 'DFXu_oRwFE_9a0TRrK8mmxs9wP1q9j8T_tTpFWzLw3a59o6fA4UPMMtsfm__B-4QDqfp7EDfrRuy9muigBVDVg', 'count': {'type': 'Integer', 'value': 115}, 'score': {'type': 'Float', 'value': 40.0}, 'tags': {'type': 'Array', 'value': ['initial', 'sequenced']}}

--- cleanup ---
Cleaned up collection

=== Advanced CRUD example completed ===
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
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: -PQUFSVnd7-dDMyRbV1hLqT4brIg8URLkBVuV4LLPn4YSMMVc6VJCE1RGprqPK5CAdy9Fz8LJAwDAO2D2ABmuw

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: High-performance database product
- **Price**: $99

If you need more details or have other questions, feel free to ask!

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['llm_provider', 'content', 'created_at', 'llm_model', 'context_snippets', 'role', 'id', 'updated_at', 'token_usage', 'chat_id'])
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
✓ Created second session: t3aTseu_CQ01Mp_eXeiA67hAaeuZorY6KCZ30X-aez7I-falwUQ_HQDQURYkvtFdGk7dnfSpiXRriFXTRMd1MA
✓ Sent message in second session
✓ Sessions merged successfully
  Total messages in merged session: 7

=== Feature 5: Delete Message ===
✓ Message deleted

✓ Messages remaining: 6

=== Cleanup ===
✓ Deleted chat session: t3aTseu_CQ01Mp_eXeiA67hAaeuZorY6KCZ30X-aez7I-falwUQ_HQDQURYkvtFdGk7dnfSpiXRriFXTRMd1MA
✓ Deleted chat session: -PQUFSVnd7-dDMyRbV1hLqT4brIg8URLkBVuV4LLPn4YSMMVc6VJCE1RGprqPK5CAdy9Fz8LJAwDAO2D2ABmuw
✓ Deleted collection

✓ All advanced chat features demonstrated successfully!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: TL850Ud2qN1ikcEN5-kh2pXzHtq2PRf1fR9Ld2iCv2TPB6Mw9QugEl_s--uDc12_BKmitZHLnkGTDcRGrSv9wQ

=== Sending Chat Message ===
Message ID: RIM1ONAFhgiMoKNCjEGJa5J5rl1_gRPUFM1ooehdYQpUdH6BSX33okQelsGeMXx41H63KzGtcGdTrFxVGUT8gQ

=== AI Response ===
The available products and their prices are as follows:

1. **ekoDB Cloud**
   - Price: $499
   - Description: Fully managed cloud database service product.

2. **ekoDB**
   - Price: $99
   - Description: A high-performance database product with AI capabilities.

3. **ekoDB Pro**
   - Price: $299
   - Description: Enterprise edition product with advanced features.

Execution Time: 2903ms

=== Token Usage ===
Prompt tokens: 3413
Completion tokens: 91
Total tokens: 3504

=== Cleanup ===
✓ Deleted session
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Message Stream (SSE) Example (Python) ===

Created session: TI_JRjNjvkQ7H_ofgJTaa93kGSmCQzu4RHqOd3Zpld78FFdEIMUceXQG84ilyz7HXKp5XBJayw9OdKdRcHKipQ

Streaming response for: 'What is ekoDB?'

{"__progress":"received"}{"__progress":"generating","model":"gpt-4o-mini","provider":"openai"}EkoDB is an API-friendly database designed to provide fast access to data. It is built to be lightweight and provides a simple interface for data storage and retrieval. As of my last knowledge update in October 2023, EkoDB may support various features suited for developers looking to handle data efficiently in applications that require quick read/write capabilities.

EkoDB could also be tailored for specific uses, such as real-time applications or projects with high-performance requirements. However, since specific features and updates can vary over time, I recommend checking the official EkoDB website or documentation for the latest information on its capabilities, use cases, and any recent developments.

--- Stream complete ---
Message ID: L4Zuc40lnYu6mMnyHI7_vj6V-zEGO5X-CREJrTMaxOVa9-fY3PLwrF41G2SLmdk4dUJNARBtAQzAGSjXAmtvyQ
Execution time: 1444ms
Context window: 128000 tokens

✓ Chat message stream example completed
✓ Client created

=== Get All Chat Models ===
OpenAI models: ['gpt-3.5-turbo-0125', 'gpt-5.4-2026-03-05', 'gpt-5-nano', 'gpt-5.4-mini', 'o1-pro', 'gpt-5.1-codex', 'tts-1', 'gpt-5-mini', 'gpt-5-mini-2025-08-07', 'gpt-4-turbo-2024-04-09', 'gpt-realtime-2.1-mini', 'gpt-realtime-2025-08-28', 'gpt-4', 'tts-1-hd', 'gpt-realtime-1.5', 'gpt-realtime', 'gpt-5.2-pro', 'gpt-5-chat-latest', 'gpt-5.5-pro', 'gpt-audio-2025-08-28', 'gpt-4o-mini-transcribe-2025-03-20', 'omni-moderation-2024-09-26', 'gpt-live-transcribe', 'gpt-5.6-luna', 'omni-moderation-latest', 'o4-mini-2025-04-16', 'gpt-4o-2024-11-20', 'gpt-5.1', 'whisper-1', 'gpt-5-pro', 'gpt-realtime-whisper', 'gpt-5-2025-08-07', 'gpt-5.4-mini-2026-03-17', 'babbage-002', 'gpt-4o-2024-05-13', 'text-embedding-3-large', 'chat-latest', 'gpt-realtime-translate', 'gpt-5.4-pro-2026-03-05', 'gpt-realtime-2', 'gpt-5-search-api', 'gpt-5.5-2026-04-23', 'tts-1-1106', 'gpt-5.5-pro-2026-04-23', 'gpt-audio-mini-2025-10-06', 'gpt-5.2-pro-2025-12-11', 'o3-mini-2025-01-31', 'gpt-audio-1.5', 'gpt-image-1.5', 'gpt-4-turbo', 'gpt-4o-2024-08-06', 'o3', 'gpt-4o-mini-search-preview-2025-03-11', 'gpt-4o-mini-transcribe-2025-12-15', 'gpt-4.1-mini', 'gpt-4o-mini', 'gpt-4.1-nano', 'gpt-image-2', 'gpt-3.5-turbo-16k', 'text-embedding-ada-002', 'gpt-realtime-2.1', 'gpt-4.1', 'gpt-5.5', 'gpt-4o-transcribe-diarize', 'gpt-5-codex', 'gpt-4-0613', 'gpt-5.1-2025-11-13', 'gpt-4o-search-preview', 'o1-2024-12-17', 'gpt-5-pro-2025-10-06', 'gpt-5.6-sol', 'gpt-4o-mini-tts-2025-12-15', 'gpt-image-1', 'gpt-audio', 'gpt-5.3-codex', 'gpt-5-search-api-2025-10-14', 'gpt-realtime-mini', 'gpt-image-1-mini', 'gpt-5.6-terra', 'gpt-3.5-turbo-instruct-0914', 'sora-2', 'gpt-5.4-pro', 'gpt-4o-mini-tts', 'gpt-realtime-mini-2025-12-15', 'gpt-5.1-codex-max', 'gpt-4o-mini-2024-07-18', 'gpt-4.1-2025-04-14', 'gpt-5', 'gpt-4o-mini-transcribe', 'gpt-image-2-2026-04-21', 'gpt-5.2-chat-latest', 'sora-2-pro', 'gpt-3.5-turbo-instruct', 'gpt-transcribe', 'gpt-4.1-nano-2025-04-14', 'gpt-5.1-chat-latest', 'tts-1-hd-1106', 'gpt-5.2-codex', 'gpt-4o', 'o4-mini-deep-research', 'gpt-5.4', 'gpt-4o-search-preview-2025-03-11', 'text-embedding-3-small', 'gpt-5-nano-2025-08-07', 'gpt-4o-transcribe', 'o4-mini', 'gpt-5.1-codex-mini', 'gpt-image-2.5-sunburst-2026-09-08', 'gpt-5.3-chat-latest', 'o3-2025-04-16', 'gpt-5.2-2025-12-11', 'chatgpt-image-latest', 'gpt-3.5-turbo', 'gpt-3.5-turbo-1106', 'gpt-4o-mini-tts-2025-03-20', 'gpt-5.2', 'gpt-5.4-nano-2026-03-17', 'o1', 'davinci-002', 'gpt-5.4-nano', 'o3-mini', 'o4-mini-deep-research-2025-06-26', 'gpt-audio-mini-2025-12-15', 'o1-pro-2025-03-19', 'gpt-6-luna', 'gpt-image-2.5-flare', 'gpt-6-astra', 'gpt-audio-mini', 'gpt-4.1-mini-2025-04-14', 'gpt-image-2.5-sunburst', 'gpt-image-2.5-flare-2026-09-08', 'gpt-4o-mini-search-preview', 'gpt-6-sol', 'gpt-live-1', 'gpt-6.1-sol']
Anthropic models: ['claude-haiku-5-5', 'claude-sonnet-5-5', 'claude-opus-5-5', 'claude-fable-5-1', 'claude-opus-5', 'claude-sonnet-5', 'claude-fable-5', 'claude-opus-4-8', 'claude-opus-4-7', 'claude-sonnet-4-6', 'claude-opus-4-6', 'claude-opus-4-5-20251101', 'claude-haiku-4-5-20251001', 'claude-sonnet-4-5-20250929']
Perplexity models: []
Gemini models: []

=== Get OpenAI Models ===
OpenAI models: ['gpt-3.5-turbo-0125', 'gpt-5.4-2026-03-05', 'gpt-5-nano', 'gpt-5.4-mini', 'o1-pro', 'gpt-5.1-codex', 'tts-1', 'gpt-5-mini', 'gpt-5-mini-2025-08-07', 'gpt-4-turbo-2024-04-09', 'gpt-realtime-2.1-mini', 'gpt-realtime-2025-08-28', 'gpt-4', 'tts-1-hd', 'gpt-realtime-1.5', 'gpt-realtime', 'gpt-5.2-pro', 'gpt-5-chat-latest', 'gpt-5.5-pro', 'gpt-audio-2025-08-28', 'gpt-4o-mini-transcribe-2025-03-20', 'omni-moderation-2024-09-26', 'gpt-live-transcribe', 'gpt-5.6-luna', 'omni-moderation-latest', 'o4-mini-2025-04-16', 'gpt-4o-2024-11-20', 'gpt-5.1', 'whisper-1', 'gpt-5-pro', 'gpt-realtime-whisper', 'gpt-5-2025-08-07', 'gpt-5.4-mini-2026-03-17', 'babbage-002', 'gpt-4o-2024-05-13', 'text-embedding-3-large', 'chat-latest', 'gpt-realtime-translate', 'gpt-5.4-pro-2026-03-05', 'gpt-realtime-2', 'gpt-5-search-api', 'gpt-5.5-2026-04-23', 'tts-1-1106', 'gpt-5.5-pro-2026-04-23', 'gpt-audio-mini-2025-10-06', 'gpt-5.2-pro-2025-12-11', 'o3-mini-2025-01-31', 'gpt-audio-1.5', 'gpt-image-1.5', 'gpt-4-turbo', 'gpt-4o-2024-08-06', 'o3', 'gpt-4o-mini-search-preview-2025-03-11', 'gpt-4o-mini-transcribe-2025-12-15', 'gpt-4.1-mini', 'gpt-4o-mini', 'gpt-4.1-nano', 'gpt-image-2', 'gpt-3.5-turbo-16k', 'text-embedding-ada-002', 'gpt-realtime-2.1', 'gpt-4.1', 'gpt-5.5', 'gpt-4o-transcribe-diarize', 'gpt-5-codex', 'gpt-4-0613', 'gpt-5.1-2025-11-13', 'gpt-4o-search-preview', 'o1-2024-12-17', 'gpt-5-pro-2025-10-06', 'gpt-5.6-sol', 'gpt-4o-mini-tts-2025-12-15', 'gpt-image-1', 'gpt-audio', 'gpt-5.3-codex', 'gpt-5-search-api-2025-10-14', 'gpt-realtime-mini', 'gpt-image-1-mini', 'gpt-5.6-terra', 'gpt-3.5-turbo-instruct-0914', 'sora-2', 'gpt-5.4-pro', 'gpt-4o-mini-tts', 'gpt-realtime-mini-2025-12-15', 'gpt-5.1-codex-max', 'gpt-4o-mini-2024-07-18', 'gpt-4.1-2025-04-14', 'gpt-5', 'gpt-4o-mini-transcribe', 'gpt-image-2-2026-04-21', 'gpt-5.2-chat-latest', 'sora-2-pro', 'gpt-3.5-turbo-instruct', 'gpt-transcribe', 'gpt-4.1-nano-2025-04-14', 'gpt-5.1-chat-latest', 'tts-1-hd-1106', 'gpt-5.2-codex', 'gpt-4o', 'o4-mini-deep-research', 'gpt-5.4', 'gpt-4o-search-preview-2025-03-11', 'text-embedding-3-small', 'gpt-5-nano-2025-08-07', 'gpt-4o-transcribe', 'o4-mini', 'gpt-5.1-codex-mini', 'gpt-image-2.5-sunburst-2026-09-08', 'gpt-5.3-chat-latest', 'o3-2025-04-16', 'gpt-5.2-2025-12-11', 'chatgpt-image-latest', 'gpt-3.5-turbo', 'gpt-3.5-turbo-1106', 'gpt-4o-mini-tts-2025-03-20', 'gpt-5.2', 'gpt-5.4-nano-2026-03-17', 'o1', 'davinci-002', 'gpt-5.4-nano', 'o3-mini', 'o4-mini-deep-research-2025-06-26', 'gpt-audio-mini-2025-12-15', 'o1-pro-2025-03-19', 'gpt-6-luna', 'gpt-image-2.5-flare', 'gpt-6-astra', 'gpt-audio-mini', 'gpt-4.1-mini-2025-04-14', 'gpt-image-2.5-sunburst', 'gpt-image-2.5-flare-2026-09-08', 'gpt-4o-mini-search-preview', 'gpt-6-sol', 'gpt-live-1', 'gpt-6.1-sol']

=== Get Anthropic Models ===
Anthropic models: ['claude-haiku-5-5', 'claude-sonnet-5-5', 'claude-opus-5-5', 'claude-fable-5-1', 'claude-opus-5', 'claude-sonnet-5', 'claude-fable-5', 'claude-opus-4-8', 'claude-opus-4-7', 'claude-sonnet-4-6', 'claude-opus-4-6', 'claude-opus-4-5-20251101', 'claude-haiku-4-5-20251001', 'claude-sonnet-4-5-20250929']

✓ Chat Models API example complete
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 1TDljyFM6zVElbWR3KJPFWw2oCVSA2cS5WxU-1Id-RKnOzGgqnweN9R3EK8abe8xUuMPb_ZS3ADX3qEbNtBIig

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** A high-performance database product
- **Price:** $99

If you have any more questions or need further assistance, feel free to ask!

✓ Message 2 sent
  Response: The price of the product ekoDB is **$99**. If you have any more questions, feel free to ask!

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 0LoG9gPNp860hzUZ-9sxcNB5E7aF2PGHup9rQpBMe7uzF30u-nzrzHo-UrGkzBmNxI_eiwoiGu1TpFHCMPKsBg
  Parent: 1TDljyFM6zVElbWR3KJPFWw2oCVSA2cS5WxU-1Id-RKnOzGgqnweN9R3EK8abe8xUuMPb_ZS3ADX3qEbNtBIig

=== Listing Sessions ===
✓ Found 2 sessions
  Session 1: 0LoG9gPNp860hzUZ-9sxcNB5E7aF2PGHup9rQpBMe7uzF30u-nzrzHo-UrGkzBmNxI_eiwoiGu1TpFHCMPKsBg (Untitled)
  Session 2: 1TDljyFM6zVElbWR3KJPFWw2oCVSA2cS5WxU-1Id-RKnOzGgqnweN9R3EK8abe8xUuMPb_ZS3ADX3qEbNtBIig (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: 0LoG9gPNp860hzUZ-9sxcNB5E7aF2PGHup9rQpBMe7uzF30u-nzrzHo-UrGkzBmNxI_eiwoiGu1TpFHCMPKsBg

=== Cleanup ===
✓ Deleted sessions and collection

✓ All session management operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: "1ZENvSUSUHlNoqJhHS4wsY96QK_hUfpWHesCi9bAP_8sySDJI_2G3VH7id5O67pbmt2o5jYd0RSeBfM6vSh4Xw"

=== List Collections ===
Total collections: 13
Sample collections: ['chat_goals__ek0_testing', 'chat_messages__ek0_testing', 'schedules__ek0_testing', 'chat_raw_completions__ek0_testing', 'client_collection_management_python']

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: False

✓ All collection management operations completed successfully
✓ Client created

=== Check Collection Exists (Before Creation) ===
Collection 'collection_utils_test_py' exists: False

=== Creating Test Documents ===
Created 5 test documents

=== Check Collection Exists (After Creation) ===
Collection 'collection_utils_test_py' exists: True

=== Count Documents ===
Document count in 'collection_utils_test_py': 5

=== Check Non-Existent Collection ===
Collection 'nonexistent_collection_xyz' exists: False

=== Cleanup ===
Deleted collection 'collection_utils_test_py'

✓ Collection Utilities example complete
✓ Client created
✓ conc_demo_pay saved
✓ conc_demo_rl_fail saved
✓ conc_demo_rl_skip saved
✓ conc_demo_lock saved

Invoke them like:
  POST /api/functions/conc_demo_pay        { "idempotency_key": "...", "amount": 100 }
  POST /api/functions/conc_demo_rl_fail    { "user_id": 42 }
  POST /api/functions/conc_demo_rl_skip    { "user_id": 42 }
  POST /api/functions/conc_demo_lock       { "resource": "queue:drain" }

✓ Cleaned up demo functions
=== ekoDB Convenience Methods Example ===

=== Native Dict Creation ===
✓ Created record with plain dict: {'id': 'O_g2mTPjxrdNJBA2uuTqkPuqJ4EdSXUXXuUYPcI52wfLx-AiLfZ8VO7YztRZ5Pqb3Tl6Xb3G_mNZrB180BjwlA'}

=== Upsert Operation ===
✓ Upsert (update existing record): O_g2mTPjxrdNJBA2uuTqkPuqJ4EdSXUXXuUYPcI52wfLx-AiLfZ8VO7YztRZ5Pqb3Tl6Xb3G_mNZrB180BjwlA
✓ Inserted second record: 18DzZnuIAJpLPwkfCFJNEB6xjRnuyE2sm8Wht5grZ1mIoCfpSSht_bm9yor4cCdxWtLZb9-zTfgnGB9cdLA7qg
✓ Upsert (update second record): 18DzZnuIAJpLPwkfCFJNEB6xjRnuyE2sm8Wht5grZ1mIoCfpSSht_bm9yor4cCdxWtLZb9-zTfgnGB9cdLA7qg

=== Find One Operation ===
✓ Found user by email: {'email': {'value': 'alice.j@newdomain.com', 'type': 'String'}, 'name': {'value': 'Alice Johnson', 'type': 'String'}, 'active': {'type': 'Boolean', 'value': True}, 'age': {'type': 'Integer', 'value': 29}, 'id': 'O_g2mTPjxrdNJBA2uuTqkPuqJ4EdSXUXXuUYPcI52wfLx-AiLfZ8VO7YztRZ5Pqb3Tl6Xb3G_mNZrB180BjwlA'}
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
✓ Client created
✓ crypto_demo_hmac_py saved
✓ crypto_demo_aes_py saved
✓ crypto_demo_uuid_py saved
✓ crypto_demo_totp_py saved
✓ crypto_demo_encoding_py saved

Invoke them with:
  POST /api/functions/crypto_demo_hmac_py { "payload": "hi" }
  POST /api/functions/crypto_demo_aes_py { "plaintext": "secret" }
  POST /api/functions/crypto_demo_uuid_py
  POST /api/functions/crypto_demo_totp_py
  POST /api/functions/crypto_demo_encoding_py { "title": "Héllo World" }

✓ Cleaned up demo functions
=== Distinct Values Example ===

Inserting sample products...
Inserted 8 products

=== Distinct Categories (all products) ===
Found 3 distinct categories:
  - {'type': 'String', 'value': 'books'}
  - {'type': 'String', 'value': 'clothing'}
  - {'type': 'String', 'value': 'electronics'}

=== Distinct Statuses (all products) ===
Found 3 distinct statuses:
  - {'type': 'String', 'value': 'active'}
  - {'type': 'String', 'value': 'archived'}
  - {'type': 'String', 'value': 'discontinued'}

=== Distinct Statuses in Electronics ===
Found 2 distinct statuses for electronics:
  - {'type': 'String', 'value': 'active'}
  - {'type': 'String', 'value': 'discontinued'}

Cleanup done.
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: 9k0W1oypJSKvimoTbFn9qcYTeEwPH4VnrAVjDO6JoDQjLV7MI95nWSJPOYsWNJwPL2Np0znbfxd8HFl_OPmY-g

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: xED3Dr2XasFqxgg-MrbI6XIcLyHEsNTmiRkS1rw_ZSr7Douk9ehfFJ9z0DoApyEWSR6q2JXOezn1AR_5gQKIbA

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
=== ekoDB as Edge Cache - Simple Example ===

Creating edge cache function...
✓ Edge cache script created: ZvGtCWxq2c6BdkUp7Nf53dUgDvtSZX7NmJCM197VJktaxSysvUFnXNCsUMY_MycR01k_89j5Zowqz5l9KdWy7w

Call 1: Cache miss (fetches from API)
Response time: 1278ms
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "current": {
            "interval": 900,
            "temperature_2m": 11.4,
            "time": "2026-10-17T08:30"
          },
          "current_units": {
            "interval": "seconds",
            "temperature_2m": "\u00b0C",
            "time": "iso8601"
          },
          "elevation": 32.0,
          "generationtime_ms": 9.208202362060549,
          "latitude": 40.710335,
          "longitude": -73.99308,
          "timezone": "GMT",
          "timezone_abbreviation": "GMT",
          "utc_offset_seconds": 0
        }
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
Response time: 10ms (122.5x faster!)
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "current": {
            "interval": 900,
            "temperature_2m": 11.4,
            "time": "2026-10-17T08:30"
          },
          "current_units": {
            "interval": "seconds",
            "temperature_2m": "\u00b0C",
            "time": "iso8601"
          },
          "elevation": 32.0,
          "generationtime_ms": 9.208202362060549,
          "latitude": 40.710335,
          "longitude": -73.99308,
          "timezone": "GMT",
          "timezone_abbreviation": "GMT",
          "utc_offset_seconds": 0
        }
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

=== ekoDB Function Composition Examples ===

📋 Setting up test data...

✅ Test data ready

📝 Example 1: Basic Function Composition

Building reusable functions that call each other...

✅ Saved reusable function: fetch_user_py
✅ Saved composed function: get_user_wrapper_py (calls fetch_user_py + projects fields)

📊 Result from composed function:
   Records: 1
   Name: {"type": "String", "value": "User 1"}
   Department: {"type": "String", "value": "engineering"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user_py (uses KV)
✅ Saved SWR function using composition: swr_user_py

First call (cache miss - will fetch from API):
   ⏱️  Duration: 72.2ms
   📊 Records: 1
   📦 Data: {
      "value": {
            "type": "Object",
            "value": {
                  "address": {
                        "city": "Gwenborough",
                        "geo": {
                 ...

Second call (cache hit - from cache):
   ⏱️  Duration: 1.9ms
   📊 Records: 1
   📦 Data: {
      "value": {
            "type": "Object",
            "value": {
                  "address": {
                        "city": "Gwenborough",
                        "geo": {
                 ...
   🚀 Cache speedup: 37.8x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

✅ Level 1 function: validate_user_py
✅ Level 2 function: fetch_slim_user_py (calls validate_user_py)
✅ Level 3 function: get_verified_user_py (calls fetch_slim_user_py)

📊 Result from 3-level nested composition:
   Records: 1
   Name: User 1
   Department: engineering

🎯 Key Benefit: Each function is independently testable and reusable!
   - validate_user: Used in 100 different workflows
   - fetch_slim_user: Used in 50 workflows
   - get_verified_user: Specific workflow


✅ Cleanup complete
✅ All composition examples completed!
client_function_contract: ok
🚀 ekoDB Functions Example (Python)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved: elnBV6UGi-JmgPv3SGv7t6SKwV3EGOBm7fTKFYLd65tjHa2lLNJrljMTqLYdKtlVZjAVcY0UjQU4f8sgFzbM9Q
📊 Found 5 active users

📝 Example 2: Parameterized Function

✅ Function saved: k3JiWydENs49C7PRlJ7WC3ssRryyRLObSt7HPy6vonTG43ntIs5YfxLjhqWLSvOSCNPk0ovYkVqvYBrzKilq6Q
📊 Found 3 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: l27gxobpHWr6uy_RI4n7JP5XdJNInysUGAYctbiPi5qTFf7VJT55_ESu2OQbzCQNmASLR2bsI8D0TKFKDIy2NQ
📊 Statistics: 2 groups
   {'avg_score': {'type': 'Float', 'value': 50.0}, 'count': {'type': 'Integer', 'value': 5}, 'status': {'type': 'String', 'value': 'inactive'}}

   {'avg_score': {'type': 'Float', 'value': 60.0}, 'count': {'type': 'Integer', 'value': 5}, 'status': {'type': 'String', 'value': 'active'}}

📝 Example 4: Function Management

📋 Total functions: 3
🔍 Retrieved function: Get Active Users
✏️  Function updated
🗑️  Function deleted

ℹ️  Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.

✅ All examples completed!
🚀 ekoDB Python Advanced Functions Example

📋 Setting up test data...
✅ Created 8 products

📝 Example 1: List All Products

✅ Function saved
📊 Found 8 products
⏱️  Execution time: 0ms

📝 Example 2: Group Products by Category

✅ Function saved
📊 Category breakdown:
   {'avg_price': {'type': 'Float', 'value': 365.6666666666667}, 'category': {'type': 'String', 'value': 'Furniture'}, 'count': {'type': 'Integer', 'value': 3}}
   {'avg_price': {'type': 'Float', 'value': 367.0}, 'category': {'type': 'String', 'value': 'Electronics'}, 'count': {'type': 'Integer', 'value': 5}}
⏱️  Execution time: 0ms

📝 Example 3: Count Total Products

✅ Function saved
📊 Total products: [{'total': {'type': 'Integer', 'value': 8}}]
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced script examples finished!
🚀 ekoDB Python AI Functions Example

📋 Setting up test data...
✅ Created 2 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
🤖 AI Response:
   Vector databases offer several benefits:

1. **Efficient Similarity Search**: They excel in nearest neighbor searches, making them ideal for applications like recommendation systems and image retrieval.

2. **High Dimensional Handling**: They manage high-dimensional data effectively, which is crucial for processing embeddings from machine learning models.

3. **Scalability**: Many vector databases are designed to scale horizontally, accommodating large datasets seamlessly.

4. **Performance**: They provide fast query performance through optimized indexing techniques, such as HNSW (Hierarchical Navigable Small World) or IVF (Inverted File).

5. **Flexibility**: Vector databases can store and process various data types, including text, images, and audio, allowing for multi-modal applications.

6. **Integration with ML Models**: They easily integrate with machine learning pipelines for real-time inference and data retrieval.

7. **Advanced Querying**: Support for complex queries, such as filtering and combining vector similarities with traditional conditions.

8. **Community and Tooling**: A growing ecosystem of tools and libraries supports development and integration, enhancing usability.

Overall, vector databases are tailored for applications that rely on the representation of data as vectors, particularly in AI and machine learning contexts.
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embed script saved
📊 Generated 2 embeddings
   Dimensions: 1536
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All AI script examples finished!

💡 This example demonstrates:
   ✅ Chat completions with system/user messages
   ✅ Embedding generation for text
🚀 ekoDB Python Complete Functions Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Function saved: AHsS_okd-xYqRWn4wXBobOVhQP8SuVHOPBcPrSguBxTLOrBaSEvZMDjLjGmpnYEfWRt7xbPrPchgn3HU677CKw
📊 Found 2 product groups
   {'avg_price': {'type': 'Float', 'value': 575.6666666666666}, 'category': {'type': 'String', 'value': 'Electronics'}, 'count': {'type': 'Integer', 'value': 3}}
   {'avg_price': {'type': 'Float', 'value': 474.0}, 'category': {'type': 'String', 'value': 'Furniture'}, 'count': {'type': 'Integer', 'value': 2}}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   {'category': {'type': 'String', 'value': 'Furniture'}, 'count': {'type': 'Integer', 'value': 2}}
   {'category': {'type': 'String', 'value': 'Electronics'}, 'count': {'type': 'Integer', 'value': 3}}
⏱️  Execution time: 0ms

📝 Example 4: High Rating Products

✅ Function saved
📊 Found 3 products
⏱️  Execution time: 0ms

📝 Example 5: function with Parameter Definition

✅ Function saved
📊 Found 3 products
⏱️  Execution time: 0ms

📝 Example 6: Multi-Stage Pipeline (FindAll → Group → Count)

✅ Function saved
📊 Pipeline executed 3 stages
⏱️  Total execution time: 0ms

✅ All complete script examples finished!

💡 This example demonstrates ekoDB's function system:
   ✅ FindAll operations
   ✅ Group aggregations (Count, Average)
   ✅ Multi-stage pipelines (FindAll → Group → Count)
   ✅ Parameter definitions
   ✅ Function management (save, call, delete)
🧹 Cleaning up...
✅ Cleanup complete

🚀 ekoDB Python CRUD Functions Example

📋 Setting up test data...
✅ Created 10 test users

📝 Example 1: List All Users

✅ Function saved
📊 Found 10 users
⏱️  Execution time: 0ms

📝 Example 2: Count Users by Status

✅ Function saved
📊 User counts by status:
   active: 7 users
   inactive: 3 users
⏱️  Execution time: 0ms

📝 Example 3: Average Score by Role

✅ Function saved
📊 Average score by role:
   {'avg_score': {'type': 'Float', 'value': 20.0}, 'count': {'type': 'Integer', 'value': 3}, 'role': {'type': 'String', 'value': 'admin'}}
   {'avg_score': {'type': 'Float', 'value': 70.0}, 'count': {'type': 'Integer', 'value': 7}, 'role': {'type': 'String', 'value': 'user'}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD script examples finished!
🚀 ekoDB Python KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: ODk2Cs6imHAKLtxXUib-EzIjvU-GyFEkRNs3MCqVD4Bax9DuOfVLH52u2DlIRsRRzk7hyRRyjorbEGMcIE6vdQ
✅ Inserted 2 products with wrapped types

📝 Example 2: Querying and Extracting Wrapped Types

📊 Found 2 products
   • Laptop Pro
   • Wireless Mouse

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {'value': '{"type":"Object","value":{"role":"admin","userId":"user_abc"}}'}
🔍 Key exists: True
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Pattern Query

✅ Set 4 config entries
📊 Found 3 app config entries
📊 Found 4 total config entries

📝 Example 5: Combined Wrapped Types + KV Usage

✅ Inserted order: BkSKZ_CyoNtGu8IWKBNgfZM9J6JEvXzoBDrDj8cKlZdF4VIVe1CciVqdmG9FWq9MqOojS_UCy9cJ7YZ_aac9dw
✅ Cached order status
📊 Quick status lookup: {'value': '{"type":"Object","value":{"status":"processing","updated_at":"2026-10-17T08:30:14.491698+00:00"}}'}

🧹 Cleaning up...
✅ Cleanup complete

✅ All KV & Wrapped Types examples completed!

💡 Key takeaways:
   ✅ Use field_* helpers for type-safe wrapped values
   ✅ field_decimal() preserves precision (no floating point errors)
   ✅ KV store is great for caching and quick lookups
   ✅ Combine KV caching with collection inserts for real workflows
🚀 ekoDB Python Search Functions Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Function saved
📊 Found 5 documents
   1. Getting Started with ekoDB (Database)
   2. Database Design Principles (Database)
   3. Vector Databases Explained (Database)
   4. Natural Language Processing (AI)
   5. Introduction to Machine Learning (AI)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   {'category': {'type': 'String', 'value': 'AI'}, 'count': {'type': 'Integer', 'value': 2}}
   {'category': {'type': 'String', 'value': 'Database'}, 'count': {'type': 'Integer', 'value': 3}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
=== ekoDB Goal Template CRUD Example (Python) ===

--- Creating goal template ---
Created template: Data Migration (id: YJIbnspxTbGRec_hMUZfBHbLyN1oivJBlYm50NIox8_By3wIPJfJ-osZa_vS-vpC0RhNFjHWWCXVFfaJUkc-YQ)

--- Listing templates ---
Templates: {'count': 1, 'items': [{'description': {'type': 'String', 'value': 'Template for migrating data between schemas'}, 'id': 'YJIbnspxTbGRec_hMUZfBHbLyN1oivJBlYm50NIox8_By3wIPJfJ-osZa_vS-vpC0RhNFjHWWCXVFfaJUkc-YQ', 'steps': {'type': 'Array', 'value': [{'description': 'Analyze source schema'}, {'description': 'Create target schema'}, {'description': 'Migrate records'}, {'description': 'Validate results'}]}, 'title': {'type': 'String', 'value': 'Data Migration'}}]}

--- Getting template ---
Fetched: {'type': 'String', 'value': 'Data Migration'}

--- Updating template ---
Updated description: {'type': 'String', 'value': 'Updated: comprehensive data migration workflow'}

--- Deleting template ---
Template deleted successfully

✓ Goal template CRUD example completed
=== ekoDB Goals / Tasks / Agents Integration Example (Python) ===

--- goal_create ---
Created goal: Deploy v3 (id: ajN7etekCJ-twEcBwHlcjJIgv7VL_MIM1PG22QrM5OC5J7oym4iGsbZ9FhOuz0bRKRJ5reoHGSJR3x5I4feTsg)

--- goal_list ---
Goals: {'count': 1, 'goals': [{'created_at': '2026-10-17T08:30:14.855009+00:00', 'description': 'Deploy version 3 to production', 'id': 'ajN7etekCJ-twEcBwHlcjJIgv7VL_MIM1PG22QrM5OC5J7oym4iGsbZ9FhOuz0bRKRJ5reoHGSJR3x5I4feTsg', 'status': 'pending', 'steps': '[{"description":"Build Docker image","status":"pending"},{"description":"Run migrations","status":"pending"},{"description":"Smoke test","status":"pending"}]', 'title': 'Deploy v3', 'updated_at': '2026-10-17T08:30:14.855009+00:00'}]}

--- goal_get ---
Fetched: {'type': 'String', 'value': 'Deploy v3'}

--- goal_update ---
Updated description: {'type': 'String', 'value': 'Deploy version 3 to production (updated)'}

--- goal_search ---
Search results: {'count': 1, 'items': [{'_score': 13.2, 'created_at': {'type': 'DateTime', 'value': '2026-10-17T08:30:14.855009+00:00'}, 'description': {'type': 'String', 'value': 'Deploy version 3 to production (updated)'}, 'id': 'ajN7etekCJ-twEcBwHlcjJIgv7VL_MIM1PG22QrM5OC5J7oym4iGsbZ9FhOuz0bRKRJ5reoHGSJR3x5I4feTsg', 'status': {'type': 'String', 'value': 'pending'}, 'steps': {'type': 'String', 'value': '[{"description":"Build Docker image","status":"pending"},{"description":"Run migrations","status":"pending"},{"description":"Smoke test","status":"pending"}]'}, 'title': {'type': 'String', 'value': 'Deploy v3'}, 'updated_at': {'type': 'DateTime', 'value': '2026-10-17T08:30:14.864739+00:00'}}]}

--- goal_step_start (step 0) ---
Step 0 started: {'created_at': {'type': 'DateTime', 'value': '2026-10-17T08:30:14.855009+00:00'}, 'description': {'type': 'String', 'value': 'Deploy version 3 to production (updated)'}, 'id': 'ajN7etekCJ-twEcBwHlcjJIgv7VL_MIM1PG22QrM5OC5J7oym4iGsbZ9FhOuz0bRKRJ5reoHGSJR3x5I4feTsg', 'status': {'type': 'String', 'value': 'in_progress'}, 'steps': {'type': 'String', 'value': '[{"description":"Build Docker image","status":"InProgress"},{"description":"Run migrations","status":"pending"},{"description":"Smoke test","status":"pending"}]'}, 'title': {'type': 'String', 'value': 'Deploy v3'}, 'updated_at': {'type': 'DateTime', 'value': '2026-10-17T08:30:14.873106+00:00'}}

--- goal_step_complete (step 0) ---
Step 0 completed: {'created_at': {'type': 'DateTime', 'value': '2026-10-17T08:30:14.855009+00:00'}, 'description': {'type': 'String', 'value': 'Deploy version 3 to production (updated)'}, 'id': 'ajN7etekCJ-twEcBwHlcjJIgv7VL_MIM1PG22QrM5OC5J7oym4iGsbZ9FhOuz0bRKRJ5reoHGSJR3x5I4feTsg', 'status': {'type': 'String', 'value': 'in_progress'}, 'steps': {'type': 'String', 'value': '[{"description":"Build Docker image","result":"Docker image built: sha256:abc123","status":"Completed"},{"description":"Run migrations","status":"pending"},{"description":"Smoke test","status":"pending"}]'}, 'title': {'type': 'String', 'value': 'Deploy v3'}, 'updated_at': {'type': 'DateTime', 'value': '2026-10-17T08:30:14.879429+00:00'}}

--- goal_step_fail (step 1) ---
Step 1 failed: {'created_at': {'type': 'DateTime', 'value': '2026-10-17T08:30:14.855009+00:00'}, 'description': {'type': 'String', 'value': 'Deploy version 3 to production (updated)'}, 'id': 'ajN7etekCJ-twEcBwHlcjJIgv7VL_MIM1PG22QrM5OC5J7oym4iGsbZ9FhOuz0bRKRJ5reoHGSJR3x5I4feTsg', 'status': {'type': 'String', 'value': 'in_progress'}, 'steps': {'type': 'String', 'value': '[{"description":"Build Docker image","result":"Docker image built: sha256:abc123","status":"Completed"},{"description":"Run migrations","error":"Migration failed: column already exists","status":"Failed"},{"description":"Smoke test","status":"pending"}]'}, 'title': {'type': 'String', 'value': 'Deploy v3'}, 'updated_at': {'type': 'DateTime', 'value': '2026-10-17T08:30:14.885732+00:00'}}

--- goal_complete ---
Goal completed: {'type': 'String', 'value': 'pending_review'}

--- goal_approve ---
Goal approved: {'type': 'String', 'value': 'in_progress'}

--- goal_reject ---
Goal rejected: {'type': 'String', 'value': 'failed'}

--- task_create ---
Created task: Nightly Backup (id: vEB9o-_wKYIhw37lthEaVsOfkLOiLyXN-QLtlS0t523h_qsvuq5UD72cCD15TG1wEWQqk8ebDxFX0On4SVK92Q)

--- task_list ---
Tasks: {'count': 1, 'items': [{'cron': {'type': 'String', 'value': '0 2 * * *'}, 'description': {'type': 'String', 'value': 'Backup all databases nightly'}, 'id': 'vEB9o-_wKYIhw37lthEaVsOfkLOiLyXN-QLtlS0t523h_qsvuq5UD72cCD15TG1wEWQqk8ebDxFX0On4SVK92Q', 'max_consecutive_failures': {'type': 'Integer', 'value': 3}, 'name': {'type': 'String', 'value': 'Nightly Backup'}}]}

--- task_get ---
Fetched: {'type': 'String', 'value': 'Nightly Backup'}

--- task_start ---
Task started: {'type': 'String', 'value': 'running'}

--- task_succeed ---
Task succeeded: {'consecutive_failures': {'type': 'Integer', 'value': 0}, 'cron': {'type': 'String', 'value': '0 2 * * *'}, 'description': {'type': 'String', 'value': 'Backup all databases nightly'}, 'id': 'vEB9o-_wKYIhw37lthEaVsOfkLOiLyXN-QLtlS0t523h_qsvuq5UD72cCD15TG1wEWQqk8ebDxFX0On4SVK92Q', 'last_error': {'type': 'Null', 'value': None}, 'last_run': {'type': 'DateTime', 'value': '2026-10-17T08:30:14.947843+00:00'}, 'max_consecutive_failures': {'type': 'Integer', 'value': 3}, 'name': {'type': 'String', 'value': 'Nightly Backup'}, 'run_count': {'type': 'Integer', 'value': 1}, 'status': {'type': 'String', 'value': 'active'}, 'updated_at': {'type': 'DateTime', 'value': '2026-10-17T08:30:14.947843+00:00'}}

--- task_pause ---
Task paused: {'type': 'String', 'value': 'paused'}

--- task_resume ---
Task resumed: {'type': 'String', 'value': 'active'}

--- task_fail ---
Task failed: {'consecutive_failures': {'type': 'Integer', 'value': 1}, 'cron': {'type': 'String', 'value': '0 2 * * *'}, 'description': {'type': 'String', 'value': 'Backup all databases nightly'}, 'id': 'vEB9o-_wKYIhw37lthEaVsOfkLOiLyXN-QLtlS0t523h_qsvuq5UD72cCD15TG1wEWQqk8ebDxFX0On4SVK92Q', 'last_error': {'type': 'String', 'value': 'Disk full'}, 'last_run': {'type': 'DateTime', 'value': '2026-10-17T08:30:14.979490+00:00'}, 'max_consecutive_failures': {'type': 'Integer', 'value': 3}, 'name': {'type': 'String', 'value': 'Nightly Backup'}, 'next_run': {'type': 'DateTime', 'value': '2026-03-22T02:00:00+00:00'}, 'run_count': {'type': 'Integer', 'value': 1}, 'status': {'type': 'String', 'value': 'active'}, 'updated_at': {'type': 'DateTime', 'value': '2026-10-17T08:30:14.979490+00:00'}}

--- task_due ---
Due tasks: {'count': 0, 'items': []}

--- task_delete ---
Task deleted

--- agent_create ---
Created agent: CodeReviewer (id: wke-rqajOTZLD79AI0G4gdy5Ak6ptnV1kGVbkFc_tjXksXkVaFLaPPMqj_3fmq1govqGo87REwXDmFUBJqSN0g)

--- agent_list ---
Agents: {'count': 1, 'items': [{'deployment_id': {'type': 'String', 'value': 'deploy_test'}, 'id': 'wke-rqajOTZLD79AI0G4gdy5Ak6ptnV1kGVbkFc_tjXksXkVaFLaPPMqj_3fmq1govqGo87REwXDmFUBJqSN0g', 'llm_model': {'type': 'String', 'value': 'gpt-4o'}, 'name': {'type': 'String', 'value': 'CodeReviewer'}, 'system_prompt': {'type': 'String', 'value': 'You review code for correctness and style.'}, 'tools': {'type': 'Array', 'value': ['web_search', 'file_read']}}]}

--- agent_get ---
Fetched: {'type': 'String', 'value': 'CodeReviewer'}

--- agent_get_by_name ---
By name: {'type': 'String', 'value': 'CodeReviewer'}

--- agent_update ---
Updated agent prompt: {'type': 'String', 'value': 'You review code. Be concise.'}

--- agents_by_deployment ---
Agents in deploy_test: {'count': 0, 'items': []}
WARNING: agents_by_deployment omitted created agent wke-rqajOTZLD79AI0G4gdy5Ak6ptnV1kGVbkFc_tjXksXkVaFLaPPMqj_3fmq1govqGo87REwXDmFUBJqSN0g; TODO: check/fix the server-side deployment lookup

--- agent_delete ---
Agent deleted

--- cleanup ---
Goals cleaned up

=== All goals/tasks/agents operations completed successfully ===
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
✓ py_users_register saved
✓ py_users_login saved
✓ py_users_verify_token saved

=== Auth flow defined as pure stored functions ===
Call them like:
  POST /api/functions/py_users_register { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/py_users_login    { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/py_users_verify_token { "token": "<jwt>" }

✓ Cleaned up demo functions
=== ekoDB KV Links Example (Python) ===

--- kv_set ---
Set KV key: app:config

--- insert document ---
Inserted document: oA5XGhnc2mhFi1dOPaYHqdBbGxWU827yKZVT9om9-WRyFHA2LxUbekg3t5VlWNj02-Ouvsdt0fZiQQMPPc1qow

--- kv_link ---
Linked: None

--- kv_get_links ---
Links for app:config: [{'collection': 'kv_links_example_py', 'created_at': '2026-10-17T08:30:15.501757Z', 'document_id': 'oA5XGhnc2mhFi1dOPaYHqdBbGxWU827yKZVT9om9-WRyFHA2LxUbekg3t5VlWNj02-Ouvsdt0fZiQQMPPc1qow', 'field_path': None, 'last_accessed': '2026-10-17T08:30:15.502404Z', 'metadata': {}}]

--- kv_unlink ---
Unlinked: None

--- kv_get_links (verify empty) ---
Links after unlink: []

--- cleanup ---
Cleaned up KV key and collection

=== KV links example completed ===
✓ Client created

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: {'value': '{"type":"Object","value":{"userId":123,"username":"john_doe"}}'}

=== KV Batch Set ===
✓ Batch set 3 keys
  kv_operations:py:cache:product:1: success
  kv_operations:py:cache:product:2: success
  kv_operations:py:cache:product:3: success

=== KV Batch Get ===
✓ Batch retrieved 3 values
  kv_operations:py:cache:product:1: {'name': 'Product 1', 'price': 29.99}
  kv_operations:py:cache:product:2: {'price': 39.99, 'name': 'Product 2'}
  kv_operations:py:cache:product:3: {'price': 49.99, 'name': 'Product 3'}

=== KV Exists ===
Key exists: True

=== KV Find (Pattern Query) ===
Found 3 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 4

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: False

=== KV Batch Delete ===
✓ Batch deleted 3 keys
  kv_operations:py:cache:product:1: deleted
  kv_operations:py:cache:product:2: deleted
  kv_operations:py:cache:product:3: deleted

✓ All KV operations completed successfully
=== KV Precision: Float vs Decimal ===

=== Test 1: Using Python Floats (LOSES PRECISION) ===
✓ Stored products with float prices

Retrieved float prices:
  Widget A: $29.99 (expected $29.99) ✓
  Widget B: $39.99 (expected $39.99) ✓
  Widget C: $49.99 (expected $49.99) ✓

=== Test 2: Using field_decimal() (PRESERVES PRECISION) ===
✓ Stored products with decimal prices

Retrieved decimal prices:
  Widget A: $29.99 (expected $29.99) ✓
  Widget B: $39.99 (expected $39.99) ✓
  Widget C: $49.99 (expected $49.99) ✓

=== Test 3: Sum Calculation Comparison ===
  Float sum: $119.97 (expected $119.97)
  Decimal sum: $119.97 (expected $119.97)

=== Test 4: Extreme Precision Example ===
  Float 0.1 + 0.2 = 0.30000000000000004 (should be 0.3)
  Decimal "0.30" = 0.30 (exact!)

=== Cleanup ===
✓ Cleaned up test keys

=== Summary ===
✅ Use field_decimal() for monetary values, percentages, and
   any case where floating-point errors are unacceptable.
✅ field_decimal() stores values as strings internally,
   preserving exact precision across all operations.
✓ Client created
✓ py_route_admin saved
✓ py_route_user_by_id saved
✓ py_route_user_posts saved
✓ py_route_org_create_member saved

Try them with curl:
  curl http://localhost:8080/api/route/users/admin
  curl http://localhost:8080/api/route/users/42
  curl http://localhost:8080/api/route/users/42/posts/7
  curl -X POST http://localhost:8080/api/route/orgs/acme/members \
       -H 'Content-Type: application/json' -d '{"name":"alice"}'

✓ Cleaned up demo functions
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
=== ekoDB Raw Completion Stream (SSE) Example ===

--- Basic SSE Raw Completion ---
Response: The three primary colors are red, blue, and yellow.

--- Structured Output via SSE ---
JSON response: [
  {
    "name": "Earth",
    "diameter_km": 12742
  },
  {
    "name": "Mars",
    "diameter_km": 6779
  },
  {
    "name": "Jupiter",
    "diameter_km": 139820
  }
]

--- Blocking HTTP (for comparison) ---
Blocking response: Hello! I hope you're having a wonderful day.

=== Done ===
=== ekoDB Schedule Management Example (Python) ===

--- create_schedule ---
Created schedule: Nightly Report (id: b8499574-d731-4177-bf46-cfbf58d7cc8a)

--- list_schedules ---
Schedules: {'count': 1, 'schedules': [{'created_at': '2026-10-17T08:30:18.336378Z', 'cron_expression': '0 0 0 * * *', 'description': 'Generate and email nightly analytics report', 'enabled': True, 'function_label': 'schedule_noop_python_84987', 'id': 'b8499574-d731-4177-bf46-cfbf58d7cc8a', 'last_execution': None, 'name': 'Nightly Report', 'next_execution': '2026-10-18T00:00:00Z', 'parameters': {}, 'stats': {'avg_execution_time_ms': 0.0, 'failed_executions': 0, 'last_error': None, 'successful_executions': 0, 'total_executions': 0}, 'timezone': 'UTC', 'updated_at': '2026-10-17T08:30:18.336378Z'}]}

--- get_schedule ---
Fetched: Nightly Report cron=0 0 0 * * *

--- update_schedule ---
Updated cron: 0 30 1 * * *

--- trigger_schedule ---
Triggered: {'schedule_id': 'b8499574-d731-4177-bf46-cfbf58d7cc8a', 'status': 'triggered'}

--- pause_schedule ---
Paused: enabled=False

--- resume_schedule ---
Resumed: enabled=True
Schedule deleted successfully

=== Schedule management example completed ===
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
=== Search Examples ===

Setting up test data...
✅ Test data created

1. Basic full-text search:
Found 2 results
  1. Score: 12.870, Matched: name, email
  2. Score: 6.270, Matched: name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 13.200, Matched: bio, title
  2. Score: 13.200, Matched: bio, title
  3. Score: 13.200, Matched: title, bio
  4. Score: 13.200, Matched: title, bio

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 26.400, Matched: bio, title
  2. Score: 26.400, Matched: bio, title
  3. Score: 26.400, Matched: bio, title
  4. Score: 26.400, Matched: bio, title

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 6.600, Matched: bio
  2. Score: 6.600, Matched: bio

5. Search with stemming and exact match boosting:
Found 2 results (matches: run, running, runs)
  1. Score: 6.600, Matched: bio
  2. Score: 6.600, Matched: bio

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.781
  2. Score: 0.771
  3. Score: 0.763

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.391, Matched: content, title
  2. Score: 0.886, Matched: title, content
  3. Score: 0.382, Matched:

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 6.600, Matched: title

9. Vector search with a metadata pre-filter (category = ml):
Found 2 documents in category "ml" (NLP excluded)
  1. Introduction to Machine Learning (category: ml)
  2. Deep Learning Fundamentals (category: ml)

Paired vector search: document=Rust Programming Answer, query=Rust Programming Question
=== Cleanup ===
✅ Deleted test collections

✅ Search examples completed!
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {'id': 'FYOBaqxDqCbqS4oDRr_op7gKOCbiaik2aGEPE67ffhkzWWdtuxrrDba6815N7ZCMkFybdkD7ZiyU06fqiG9a5Q'}

=== Find by ID ===
Found: {'id': 'FYOBaqxDqCbqS4oDRr_op7gKOCbiaik2aGEPE67ffhkzWWdtuxrrDba6815N7ZCMkFybdkD7ZiyU06fqiG9a5Q', 'tags': {'type': 'Array', 'value': ['tag1', 'tag2', 'tag3']}, 'price': {'type': 'Float', 'value': 99.99}, 'categories': {'value': ['electronics', 'computers'], 'type': 'Array'}, 'user_id': {'value': '550e8400-e29b-41d4-a716-446655440000', 'type': 'String'}, 'value': {'value': 42, 'type': 'Integer'}, 'name': {'type': 'String', 'value': 'Test Record'}, 'metadata': {'value': {'nested': {'deep': True}, 'key': 'value'}, 'type': 'Object'}, 'active': {'type': 'Boolean', 'value': True}, 'created_at': {'type': 'String', 'value': '2026-10-17T04:30:18.883863'}, 'data': {'type': 'String', 'value': 'aGVsbG8gd29ybGQ='}, 'embedding': {'value': [0.1, 0.2, 0.3, 0.4, 0.5], 'type': 'Array'}}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): True
  price (Decimal): 99.99
  created_at (DateTime): 2026-10-17 04:30:18.883863
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): ['tag1', 'tag2', 'tag3']
  metadata (Object): {'nested': {'deep': True}, 'key': 'value'}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): ['electronics', 'computers']
  data (Bytes): 11 bytes
Plain record: {'id': 'FYOBaqxDqCbqS4oDRr_op7gKOCbiaik2aGEPE67ffhkzWWdtuxrrDba6815N7ZCMkFybdkD7ZiyU06fqiG9a5Q', 'tags': ['tag1', 'tag2', 'tag3'], 'price': 99.99, 'categories': ['electronics', 'computers'], 'user_id': '550e8400-e29b-41d4-a716-446655440000', 'value': 42, 'name': 'Test Record', 'metadata': {'nested': {'deep': True}, 'key': 'value'}, 'active': True, 'created_at': '2026-10-17T04:30:18.883863', 'data': 'aGVsbG8gd29ybGQ=', 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5]}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'id': 'FYOBaqxDqCbqS4oDRr_op7gKOCbiaik2aGEPE67ffhkzWWdtuxrrDba6815N7ZCMkFybdkD7ZiyU06fqiG9a5Q', 'tags': {'type': 'Array', 'value': ['tag1', 'tag2', 'tag3']}, 'metadata': {'type': 'Object', 'value': {'nested': {'deep': True}, 'key': 'value'}}, 'categories': {'type': 'Array', 'value': ['electronics', 'computers']}, 'user_id': {'type': 'String', 'value': '550e8400-e29b-41d4-a716-446655440000'}, 'price': {'value': 99.99, 'type': 'Float'}, 'embedding': {'type': 'Array', 'value': [0.1, 0.2, 0.3, 0.4, 0.5]}, 'data': {'type': 'String', 'value': 'aGVsbG8gd29ybGQ='}, 'name': {'value': 'Updated Record', 'type': 'String'}, 'active': {'type': 'Boolean', 'value': True}, 'value': {'type': 'Integer', 'value': 100}, 'created_at': {'type': 'String', 'value': '2026-10-17T04:30:18.883863'}}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: Hvr4MdqEghPfmWAhZA7I27t412cr98bGUt63K1CX4ywpXTo5GzhakJVut_JRzgKFHwi4iplfQOv9zBOCPhGiWQ

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
🚀 ekoDB Python Client - Native SWR Function Examples

📋 Demonstrates:
   • Single-function SWR pattern (replaces 4-step pipeline)
   • Automatic cache checking, HTTP fetching, and cache setting
   • Built-in audit trail support
   • Duration string TTLs ('15m', '1h', '30s')
   • Multi-function pipeline integration
   • Dynamic TTL configuration


🧹 Cleaning up...
✓ Deleted 0 test scripts and owned SWR resources

Example 1: Basic Native SWR
────────────────────────────────────────────────────────────────────────────────
Single function replaces KvGet → If → HttpRequest → KvSet pipeline
✓ Created native SWR script: github_user_native_py (FAqzaqWdul8s5ly9t4iSS5bQf9amtKnbKOcd3b9joQYDPSw8GhiL6_w9DWhqbfW2GPb-txhCXqvtVAbnCfq9oA)

First call (cache miss - will fetch from GitHub API):
  Response time: 131ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 2ms
  Speedup: 65.0x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit_py (LNzR6lHoxflU3Rc94GKWD_jjCMu5fSpiiObHxxwzC-dyKptRyPivxkKF0Sy3J_y4rSmXC7NlwpUeNRt8VXMMTQ)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail_py' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline_py (PXHI5rioI-98b9NUZpVziQD65v-LDwWG6FTHksxuFhfxPDCX870ru5auIgSClFdQQnZc7OtStt2eBfcmIEmDJw)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users_swr_py' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache_py (Sspn0WpD8Ch-2-qaZpBoI4Mc39INUnkethFaY_y5AIR6F8qhaW5zniU09vYQXYl-x1YOiyz5QwIffr8pHrX30A)
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
✓ Deleted 4 test scripts and owned SWR resources

✅ All examples completed!
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_api_user_py (7FAGFJjtCLmqukRk0v3Kh3AmdOjllBx_DsurCnNimEN8VRJF0WkAAPqgeYD0-rZZmAutXV11Oba8LQbOU7eFHw)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
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
Response time: 2ms (served from cache)
Result: {
  "records": [
    {
      "value": {
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
✓ Lightning fast cache hit

🧹 Cleaning up...
✓ Cleanup complete

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 4Nq2WzM3cME9P2ZSgZpoW4oS2yUW22OOtqGR14_vvi9h31sF2HHtqbnr0kJVCMVYJGD2RMwsCxwAXoMNpcrEtg
Created Bob: $500 - ID: FklGwV9BU8cy7RS3EHaCr-ATGgr9Te-S4l36yALrXULIaW4Mp3criY2ZR8lsRIQvS6OWKudJTgR_A_N84CQApQ

=== Example 1: Begin Transaction ===
Transaction ID (server-default isolation): da32f9b1-3eb5-4adb-a1f2-2745fc7949cb

=== Example 2: Operations within Transaction ===
Alice in transaction: {'name': {'type': 'String', 'value': 'Alice'}, 'id': '4Nq2WzM3cME9P2ZSgZpoW4oS2yUW22OOtqGR14_vvi9h31sF2HHtqbnr0kJVCMVYJGD2RMwsCxwAXoMNpcrEtg', 'balance': {'value': 800, 'type': 'Integer'}, 'account_id': {'value': 'ACC001', 'type': 'String'}}
Bob in transaction: {'balance': {'value': 700, 'type': 'Integer'}, 'id': 'FklGwV9BU8cy7RS3EHaCr-ATGgr9Te-S4l36yALrXULIaW4Mp3criY2ZR8lsRIQvS6OWKudJTgR_A_N84CQApQ', 'name': {'value': 'Bob', 'type': 'String'}, 'account_id': {'value': 'ACC002', 'type': 'String'}}
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

✓ Verified committed balances: Alice=$800, Bob=$700

=== Example 5: Rollback Demo ===
New transaction: 4736e78f-86ee-42ac-bf02-666153b8549d
Updated Bob: $700 → $600 (in transaction)
Status before rollback: Active
✓ Transaction rolled back

✓ Verified Bob remains $700 after rollback

=== Cleanup ===
✓ Deleted test account collection

✓ All client transaction examples completed
✓ Client created

=== Create User Function ===
Created user function with ID: SPG8sUVVzYNSlSdtXl4rz-zLsfJuhtJ5lM4QnmBDBd9CiLDqP3lAxNflOBDZx9wlOTxoQs5j3vRVzsYwRdU4hQ

=== Get User Function ===
Retrieved: get_active_users_py - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 1 user functions:
  - get_active_users_py: Get Active Users

=== Update User Function ===
User function updated successfully

=== Delete User Function ===
User function deleted successfully

✓ User Functions API example complete
=== WebSocket Chat Streaming Example (Python) ===

Created chat session: MjvfFWbX5Kowdyk-tQGbsprzqfOLm5BOW91efO6oiNgv6PbgPzAmVOnBdOuK7HapSLeBVau3IXsPy7ggJbi5-w

Sending message: 'What is the capital of France?'
{"__progress":"received"}{"__progress":"generating","model":"gpt-4o-mini","provider":"openai"}The capital of France is Paris.

--- Stream ended ---
Message ID: UvcPUy8wdCKh_HbWN-qiF7EDYzPPZCAX-4g_AMkMAwcvuhur5htHo0qbrC9q_MX2PI0HjI5yUG6qBkA6GHGU9w
Execution time: 530ms
Token usage: {'completion_tokens': 8, 'prompt_tokens': 15, 'total_tokens': 23}

Full response: {"__progress":"received"}{"__progress":"generating","model":"gpt-4o-mini","provider":"openai"}The capital of France is Paris....
=== WebSocket Subscription Example (Python) ===

✓ Authentication successful
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_py' ===
✓ Subscribed (subscription_id: sub_c9f0e9818eb14c2895a8190c191214d8)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: psguO-f9UVCCHW0AaOePR38noxiAK6U_oYU5v5cp1NLqdABoHDeMkkjG7fr9VPeOS-145VdljUdYFOCzQTLszQ

  📡 Notification received:
     Event:      insert
     Collection: ws_subscribe_example_py
     Record IDs: psguO-f9UVCCHW0AaOePR38noxiAK6U_oYU5v5cp1NLqdABoHDeMkkjG7fr9VPeOS-145VdljUdYFOCzQTLszQ
     Timestamp:  2026-10-17T08:30:21.408432+00:00

Inserting record 2...
✓ Inserted: 1pr7flEonrtikCaYjgZe6WJTb-aN1ZVbc8P0mlsX7KHgdsXrBpohC5vnIXBmdeOJhfcqSsT_XfAl1joIHhAnjQ

  📡 Notification received:
     Event:      insert
     Record IDs: 1pr7flEonrtikCaYjgZe6WJTb-aN1ZVbc8P0mlsX7KHgdsXrBpohC5vnIXBmdeOJhfcqSsT_XfAl1joIHhAnjQ

=== Unsubscribing ===
✓ Unsubscribed: {'collection': 'ws_subscribe_example_py', 'found': True, 'unsubscribed': True}

=== Cleanup ===
✓ Deleted collection 'ws_subscribe_example_py'

✓ WebSocket subscription example completed successfully
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: 9mcEAqtypN06ekeuVmpKANBGVrPrTk63-mPY9G7rRhKFXx3xgDclor-KBLGG11NoHq1Ai2GZy_P-TNu9Ozt_Cg

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 5 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
=== Bypass Ripple Example ===

1. Basic insert (ripple enabled):
   Inserted with ripple: {'id': 'E14xf4fkkiChWitN6rt0rJcrIH4d5lEnVshKwDM_2PC0vBOkFNBWhJsg8gveO4vn1zoLLK-SuAfrKG5lFQlD3A'}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {'id': 'sHNfEZ-LgPeHYiW9vOx6jKPHqY3ksuD3mWiHs0ycD-gTIs2FNKSpJ1l8XpoH3i3AdyeNsdtYoHQOmUcBPKXLKg'}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {'id': 'E14xf4fkkiChWitN6rt0rJcrIH4d5lEnVshKwDM_2PC0vBOkFNBWhJsg8gveO4vn1zoLLK-SuAfrKG5lFQlD3A', 'name': {'value': 'Product 1', 'type': 'String'}, 'price': {'value': 150, 'type': 'Integer'}}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {'id': 'E14xf4fkkiChWitN6rt0rJcrIH4d5lEnVshKwDM_2PC0vBOkFNBWhJsg8gveO4vn1zoLLK-SuAfrKG5lFQlD3A', 'price': {'type': 'Integer', 'value': 500}, 'name': {'value': 'Upsert Product', 'type': 'String'}}

✅ All bypass_ripple operations completed successfully!
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields (id, name, email only)
  Found 3 active users
  Fields returned: ['name', 'id', 'email']
  First user: {'type': 'String', 'value': 'Alice Johnson'} <{'value': 'alice@example.com', 'type': 'String'}>

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ['avatar_url', 'user_role', 'status', 'created_at', 'age', 'bio', 'id', 'name', 'email']

Example 3: Complex query with projection (active users, ages 18-65)
  Found 3 active users (ages 18-65)
    - {'value': 'Dave Brown', 'type': 'String'} (age {'type': 'Integer', 'value': 45})
    - {'value': 'Alice Johnson', 'type': 'String'} (age {'type': 'Integer', 'value': 30})
    - {'type': 'String', 'value': 'Bob Smith'} (age {'type': 'Integer', 'value': 25})

Example 4: Query inactive users with profile fields
  Found 1 inactive users
    - {'value': 'Carol White', 'type': 'String'}: {'type': 'String', 'value': 'Manager'}

Example 5: Compare full vs projected data
  Full query:
    - 12 fields per record
    - Fields: ['secret_token', 'password', 'bio', 'id', 'api_key', 'email', 'created_at', 'age', 'status', 'user_role', 'name', 'avatar_url']
  Projected query:
    - 3 fields per record
    - Fields: ['name', 'id', 'email']
  Bandwidth savings: ~75% fewer fields

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
