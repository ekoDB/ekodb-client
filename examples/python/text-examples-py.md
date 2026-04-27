make test-examples-python
🧪 [36mRunning Python examples (direct HTTP/WebSocket)...[0m
=== Simple CRUD Operations (Direct HTTP) ===

✓ Authentication successful

=== Insert Document ===
Inserted: {'id': 'BtY6G4CK2QQYRoV5ACxlZufp1RRwFJFnxKPpSgwzP8W0KBzv0Xo_qJH5fg9FUiGyLQth1yX6l7Ce8Adafv-Cig'}

=== Find by ID ===
Found: {'value': {'value': 42, 'type': 'Integer'}, 'active': {'type': 'Boolean', 'value': True}, 'id': 'BtY6G4CK2QQYRoV5ACxlZufp1RRwFJFnxKPpSgwzP8W0KBzv0Xo_qJH5fg9FUiGyLQth1yX6l7Ce8Adafv-Cig', 'name': {'value': 'Test Record', 'type': 'String'}}

=== Find with Query ===
Found documents: [{'active': {'type': 'Boolean', 'value': True}, 'id': 'BtY6G4CK2QQYRoV5ACxlZufp1RRwFJFnxKPpSgwzP8W0KBzv0Xo_qJH5fg9FUiGyLQth1yX6l7Ce8Adafv-Cig', 'name': {'type': 'String', 'value': 'Test Record'}, 'value': {'type': 'Integer', 'value': 42}}]

=== Update Document ===
Updated: {'id': 'BtY6G4CK2QQYRoV5ACxlZufp1RRwFJFnxKPpSgwzP8W0KBzv0Xo_qJH5fg9FUiGyLQth1yX6l7Ce8Adafv-Cig', 'name': {'value': 'Updated Record', 'type': 'String'}, 'value': {'type': 'Integer', 'value': 100}, 'active': {'type': 'Boolean', 'value': True}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: Zkzsv36Ek1XeFo_niqTMDMtzYkIibISYbe7FdMX1QNu0v8msvDqC9HgMbsnmIf3hbUAsp7861rajEssi-NCDTg

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "messageId": "527703905",
  "payload": {
    "data": [
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "Zkzsv36Ek1XeFo_niqTMDMtzYkIibISYbe7FdMX1QNu0v8msvDqC9HgMbsnmIf3hbUAsp7861rajEssi-NCDTg",
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
        "id": "0u0JE-c953bBlBorh6Cut9S1tabpIzBPhR7t-HABc48qoqNB34eMmA_qbIcEsYOm65dWNv21QyDETAf-mJFcBw",
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
        "id": "FYfKxH386rTM0fgtjkI39oijBp6_ViCTDaXBMPMWygAdt-dMVYHDPbo1sa3hHVpjUGzENPNJIQ2hhngC5E0NzQ",
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
        "id": "0I6ap8UObm2WUlNNJ_fMSt7i9mn-clX96Q1xy1jpQ8m75mCdD5F8aAM_N9YbRXKPJIn6_RORN3XWg9lyL1ExoA",
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
🚀 ekoDB Functions Example (Python/HTTP)

✓ Authentication successful

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Function saved: DFcixdiPtfgVxuJl8sXYQr7UwnhveyzPlJvbtL-xs_T5jorufwDSp8RTzGC_iofrKDR8cTrB1OVaGZeX-6drXA
📊 Found 50 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Function saved: oKRZjD7-kk3ZqKEi1PDJ1ijEqa3nMirdwLn6YPYuePqwYaJ8W3dpZuFa_XAVm3aJFgnAcMwskDtlkOjAzD7PtQ
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: fy9wm3umjqRcNFD4rcojeNNC1GwmGdIomRKPxdXAWdArjqbLiMCy0DA5Sf1vhekwmX7CVeNQnonxYlHhH1pKDA
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {'count': 20, 'avg_score': 60.0, 'status': 'active', 'max_score': 100}
   {'count': 20, 'avg_score': 50.0, 'max_score': 90, 'status': 'inactive'}

📝 Example 4: Function Management

📋 Total scripts: 60
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
=== Key-Value Operations (Direct HTTP) ===

✓ Authentication successful

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: {'username': 'john_doe', 'userId': 123}

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
=== Collection Management (Direct HTTP) ===

✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: kvuc9Kc8dOP4qnrktzp9ifUTLfeFvA-xjyWzlcWXKaAOUO65qK5sPw8-yZSBBYHtqJXqDy4eRd4DT7aPJNaO1g

=== List Collections ===
Total collections: 67
Sample collections: ['integ_test_ingest_748f2fd4', 'certificate_diagnostics_staging', 'chat_goals__ek0_testing', 'certificate_diagnostics_testing', 'chat_configurations__ek0_testing']

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
Created Alice: $1000 - ID: EjkzBE0Bb6E190oSA2njYqI8TO-l-Q8MxvWtMLGwecoOZ0PFawmTlsSwaUsYfAFmeCeK-bS0Sx9dKyxQAbw9BQ
Created Bob: $500 - ID: Is9Rxi3d31GCFnaIrHPMNjDbilFMXZ51RBphN9irHuiULIvulfN0mVWdj3Q1YnPFNPQhkkGIQ1ChO1epNbXrxQ

=== Example 1: Begin Transaction ===
Transaction ID: 7cd45029-2aa0-4685-88a0-0c79ea88f1ff

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
New transaction: 0caff123-13ab-412f-8386-2f0043289963
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'type': 'Integer', 'value': 700}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: GxXlDCFfKELiJf-1bSMhbCUPPRc31IBolIFz0sPfNVhvE1T03LOh47S8KLiSobhASnwN0T6hWhQOcbUbezTq0w
Created Bob: $500 - ID: _bGSwSn-oxsuTJa_w5rR_34AptzPCCy0NLYzi5N_082e8u3nkjwUoWVNAxRRgSlUtt25clffa3lFKMrQnKAG3Q

=== Example 1: Begin Transaction ===
Transaction ID: 4acaa8d2-58a2-4cc9-b59e-03bc81166ca4

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


🔄 Switching to NON-DURABLE mode...
   ✓ Config updated: durable_operations=false


============================================================
🔥 TEST 2: Non-Durable Mode (durable=false)
============================================================

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 2wTmXGaeBVw6PS-sL3wT4T7nBb4hR_1w6Mvcf6hDOEXkfe_3VuePiraI3FDdSbExvCwrfRSouMyWNOfpyxwacw
Created Bob: $500 - ID: Xpx7PK1DXOU88J0iqgtUSV3tOGAV_Fl6z97XRfZ3EA5OrdxZhopwb8wGsc-UbDHnXFKZMXj3cIa4huyqQTbWyg

=== Example 1: Begin Transaction ===
Transaction ID: 7b6a86ed-16e9-4af3-94ab-c9bda8bf3bbd

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
Bob: ${'type': 'Integer', 'value': 700}

=== Example 5: Rollback ===
New transaction: 9d97d5e7-b652-4ee6-8f04-14f1b6cbf561
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'type': 'Integer', 'value': 700}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: MXjw2ZTR83P4KPg_D1oxWDN6CvctNugAiUp6Ynqa0_Xj8Inq6V-3NE_JZD9cYCORIfEjWOL5-E7Zf1faI_L2Fw
Created Bob: $500 - ID: ciAipoXAIlNJHsusLawiNb97AIuhJwufPzzO0uuSPFkIqyjFr8A-iXT5ckLf1dz8Slcf8GtJUBT84gcyRMU6iQ

=== Example 1: Begin Transaction ===
Transaction ID: 29623ce5-4be8-4d09-ba68-e6d4d43a3f53

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
Created Alice: $1000 - ID: k5sfixJVn9r1VTp2g0Q3A0XuGR900oaXQDKDHNcZEqB9AN2-sug8YT7xEbOKZMV17t6W3ZC73i1_RGA3oOnm3Q
Created Bob: $500 - ID: gzbF-jb-NXH3l_JQdeZot8M9LT7_84P37tKhAb9pX6oNBErEF5SW5ACxY07ariRFUlgsY3BJsQfkKNUUtlQxxA

=== Example 1: Begin Transaction ===
Transaction ID: 37e6a84c-f6e4-4754-a358-6f56553a10e6

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
New transaction: 004e29fe-cd5f-4ca2-8af2-71a7b7d12954
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'type': 'Integer', 'value': 700}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: Zn68RoM1Yqsrcthtkqh-V7UrYy5eJ0D0FAAaDRu5oen9_U-w54sGyo9DtZ0stit8FfFSEnaJklOOuysC2y2tdw
Created Bob: $500 - ID: wpsQiRgiqA7A3OcdYlGo3YAk7_1MQf0ADik3wP2qUObk_YwydGNMk-Zx2U6VollS71PYuhMThPZnZW3kVoZ0aQ

=== Example 1: Begin Transaction ===
Transaction ID: 3e9ec5c5-dd25-4cf4-beec-985926d4651d

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
📝 function 1: Insert + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: sSAtr9ZSHiRJ59R51F-b3W-j9OUfSeYeXyeilnN5weg0tQ4uiR4rz-InJlx-H3vsXun0W1OMkN_miNCceue7rA

2️⃣ Calling function (Insert + Verify)...
   ✅ function executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: ZzgHXFZKuMdRYiwH-djTWj-eLTYyGcgR0jurtdBlle4yTcI5afgEtHanRv1xKZPRQhU6XxVpYoUNXab7ip7hJQ
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}
   📋 Email: {'value': 'alice@example.com', 'type': 'String'}
   📋 Status: {'value': 'pending', 'type': 'String'}
   📋 Credits: {'type': 'Integer', 'value': 0}

============================================================
📝 function 2: Query + Update + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: DoQhok5cL6W1U8ZQVfZFT975d4Ipxsi6ZCwln5ZUsYPI9ml2fMy5i5jLtC6UaelSv_Ygna5oH8a0lofwczoDMA

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
   ✅ Function saved: 7EFoObtA2_bCyVPvwl1lpc9hlIvGutJQYjxW9pQpwgSXmCVAThlCeY8ppASNDCPN_SwJS81Megyxv6k_wjH_-A

2️⃣ Calling function (Query + Update Credits + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {'value': 0, 'type': 'Integer'}
   📋 Status: {'type': 'String', 'value': 'active'}
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}

============================================================
📝 function 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: POmAK_rJ7z3yQMb-dcaa0YHMa3URP4wHsqBNr9Tu3Jc6heJAicU1QJEBKeukeV1VHURG8mjM7bdhSuupVdIIzA

2️⃣ Calling function (Query + Delete + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: sSAtr9ZSHiRJ59R51F-b...
   ✅ Deleted script: DoQhok5cL6W1U8ZQVfZF...
   ✅ Deleted script: 7EFoObtA2_bCyVPvwl1l...
   ✅ Deleted script: POmAK_rJ7z3yQMb-dcaa...
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
  Output: Document ID = C3O2CoY5C3_zwRQ5O7KInlc7BDY2ERA5sgnk86DGc-6JJ4M4yZSBAqN_PmbO-fDO4R8q1f09ntzX6PGfGlQ2Cg
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: find_by_id(C3O2CoY5C3_zwRQ5O7KInlc7BDY2ERA5sgnk86DGc-6JJ4M4yZSBAqN_PmbO-fDO4R8q1f09ntzX6PGfGlQ2Cg)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: find_by_id(C3O2CoY5C3_zwRQ5O7KInlc7BDY2ERA5sgnk86DGc-6JJ4M4yZSBAqN_PmbO-fDO4R8q1f09ntzX6PGfGlQ2Cg)
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
  Output: Document ID = gnRb8OmQB0TAOrzq1qWU_m9fP_KOiDAj4BuJgtxHOdsCHahzuEx04mOHffvc4DdvW-C5KAu-ERFAn18j_a8qMg
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: find_by_id(gnRb8OmQB0TAOrzq1qWU_m9fP_KOiDAj4BuJgtxHOdsCHahzuEx04mOHffvc4DdvW-C5KAu-ERFAn18j_a8qMg)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: find_by_id(gnRb8OmQB0TAOrzq1qWU_m9fP_KOiDAj4BuJgtxHOdsCHahzuEx04mOHffvc4DdvW-C5KAu-ERFAn18j_a8qMg)
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
=== Running crud_functions.py ===[0m
[32m✓ crud_functions.py completed successfully[0m
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
🐍 Found CPython 3.11 at /Library/Frameworks/Python.framework/Versions/3.11/bin/python3
💻 Using `MACOSX_DEPLOYMENT_TARGET=11.0` for aarch64-apple-darwin by default
    Finished `release` profile [optimized] target(s) in 0.20s
📦 Built wheel for abi3 Python ≥ 3.8 to /Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.18.0-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel for current platform...[0m
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.18.0-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.18.0
    Uninstalling ekodb_client-0.18.0:
      Successfully uninstalled ekodb_client-0.18.0
Successfully installed ekodb-client-0.18.0
✅ [32mPython client package built and installed![0m
🧪 [36mRunning Python client library examples...[0m
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {'id': 'BbX29QHR8c6lYb5JE2w2iBia-oWfNv5DXU8RWj2nR4uq0uRRVpKcluJ0csrwMWNp8uuddjXMenvBLTH1AKer0A'}

=== Find by ID ===
Found: {'user_id': {'value': '550e8400-e29b-41d4-a716-446655440000', 'type': 'String'}, 'id': 'BbX29QHR8c6lYb5JE2w2iBia-oWfNv5DXU8RWj2nR4uq0uRRVpKcluJ0csrwMWNp8uuddjXMenvBLTH1AKer0A', 'categories': {'value': ['electronics', 'computers'], 'type': 'Array'}, 'data': {'type': 'String', 'value': 'aGVsbG8gd29ybGQ='}, 'embedding': {'value': [0.1, 0.2, 0.3, 0.4, 0.5], 'type': 'Array'}, 'metadata': {'value': {'nested': {'deep': True}, 'key': 'value'}, 'type': 'Object'}, 'value': {'value': 42, 'type': 'Integer'}, 'tags': {'type': 'Array', 'value': ['tag1', 'tag2', 'tag3']}, 'created_at': {'value': '2026-04-27T00:02:04.700295', 'type': 'String'}, 'active': {'value': True, 'type': 'Boolean'}, 'price': {'value': 99.99, 'type': 'Float'}, 'name': {'value': 'Test Record', 'type': 'String'}}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): True
  price (Decimal): 99.99
  created_at (DateTime): 2026-04-27 00:02:04.700295
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): ['tag1', 'tag2', 'tag3']
  metadata (Object): {'nested': {'deep': True}, 'key': 'value'}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): ['electronics', 'computers']
  data (Bytes): 11 bytes
Plain record: {'user_id': '550e8400-e29b-41d4-a716-446655440000', 'id': 'BbX29QHR8c6lYb5JE2w2iBia-oWfNv5DXU8RWj2nR4uq0uRRVpKcluJ0csrwMWNp8uuddjXMenvBLTH1AKer0A', 'categories': ['electronics', 'computers'], 'data': 'aGVsbG8gd29ybGQ=', 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5], 'metadata': {'nested': {'deep': True}, 'key': 'value'}, 'value': 42, 'tags': ['tag1', 'tag2', 'tag3'], 'created_at': '2026-04-27T00:02:04.700295', 'active': True, 'price': 99.99, 'name': 'Test Record'}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'id': 'BbX29QHR8c6lYb5JE2w2iBia-oWfNv5DXU8RWj2nR4uq0uRRVpKcluJ0csrwMWNp8uuddjXMenvBLTH1AKer0A', 'name': {'type': 'String', 'value': 'Updated Record'}, 'created_at': {'value': '2026-04-27T00:02:04.700295', 'type': 'String'}, 'categories': {'type': 'Array', 'value': ['electronics', 'computers']}, 'metadata': {'value': {'key': 'value', 'nested': {'deep': True}}, 'type': 'Object'}, 'active': {'type': 'Boolean', 'value': True}, 'price': {'value': 99.99, 'type': 'Float'}, 'user_id': {'type': 'String', 'value': '550e8400-e29b-41d4-a716-446655440000'}, 'tags': {'type': 'Array', 'value': ['tag1', 'tag2', 'tag3']}, 'embedding': {'value': [0.1, 0.2, 0.3, 0.4, 0.5], 'type': 'Array'}, 'value': {'type': 'Integer', 'value': 100}, 'data': {'value': 'aGVsbG8gd29ybGQ=', 'type': 'String'}}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: P4hYfm2LRKEsHzrVclEir6BYmRnqaPksjU0u4o89qGCbjioFWKkPAi17UXK6MwfjXVqS92sGzrBAL-X5dBX0_g

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
Collection created with first record: "Co5yiWVhn4_H2a-9JLq4tuNo7L2sWYvkzMA77ngaoQqYLTomImh5XBzBuqi9xN_T6kMdrfTdNKiWPiwvKafmRA"

=== List Collections ===
Total collections: 66
Sample collections: ['integ_test_ingest_748f2fd4', 'certificate_diagnostics_staging', 'chat_goals__ek0_testing', 'certificate_diagnostics_testing', 'chat_configurations__ek0_testing']

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
  cache:product:1: {'price': 29.99, 'name': 'Product 1'}
  cache:product:2: {'price': 39.99, 'name': 'Product 2'}
  cache:product:3: {'price': 49.99, 'name': 'Product 3'}

=== KV Exists ===
Key exists: True

=== KV Find (Pattern Query) ===
Found 3 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 11

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
Created Alice: $1000 - ID: fWuE-SD3gAy7IwJ7mbGB0n3e-exgYmfEzHpmaGv7wgNZnN2xFu3I4YqrNCfZT6NstTeFWHJZa7prU-hIsJk0QQ
Created Bob: $500 - ID: xzNEiAkHpvj3KIac35kN5wicplpqvyhVfLoJi5QK4U_mjW_0rA5EZMetBr9op22EWB1OFHh5lyaPpxtOLxpuvw

=== Example 1: Begin Transaction ===
Transaction ID: ade86924-cb03-4420-9132-2d265aab7c5c

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: cbcf191d-882f-400e-94f8-f1a10410370b
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
  1. Score: 25.740, Matched: email.value, name, email, name.value
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title.value, bio, title, bio.value
  2. Score: 26.400, Matched: title.value, bio.value, title, bio
  3. Score: 26.400, Matched: bio.value, bio, title.value, title
  4. Score: 26.400, Matched: title, bio, title.value, bio.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title, bio, title.value, bio.value
  2. Score: 39.600, Matched: title, title.value, bio.value, bio
  3. Score: 39.600, Matched: title, title.value, bio, bio.value
  4. Score: 39.600, Matched: bio.value, title.value, title, bio

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 2 results (matches: run, running, runs)
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio.value, bio

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.763
  2. Score: 0.737
  3. Score: 0.727

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.381, Matched: title.value, content.value, title, content
  2. Score: 1.368, Matched: title.value, title, content, content.value
  3. Score: 0.364, Matched: 

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
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: 9bitEGczLDoPW37EpUufU5ti-oOs_E0OBV9aDHfMKFGfOhNnJj6HxETLwtaERpxIuNAnK65JjA2P4lE_W4DG6w

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: KnzY0-adr7B7SImRQswx-FFLkETO05XUONaY5GtwM3BV2UtCRb_H_UIp5U8jZfbjwZO5O67WkUgv4jOCdOgU3w

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
✓ Inserted document with TTL: ApL-3l-u-0sr91o1OuoKoJqy6Ih9PsM4U1xWs09UBORZMmsrH2g5sFGcIdPfE20VdYK4AQWrE5oNIw7ueSNVyQ

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
✓ Edge cache script created: Rj59yWZo7IeSC964HBB5UjFkes5CmICgoqn78z85oi6l-Cl3438cvYWKROxdhbjJNLn4WXOHfOYAsvvJMhZZQQ

Call 1: Cache miss (fetches from API)
Response time: 46ms
Result: {
  "records": [
    {
      "value": {
        "current": {
          "interval": 900,
          "temperature_2m": 6.9,
          "time": "2026-04-27T03:45"
        },
        "current_units": {
          "interval": "seconds",
          "temperature_2m": "\u00b0C",
          "time": "iso8601"
        },
        "elevation": 32.0,
        "generationtime_ms": 0.027060508728027344,
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
Response time: 46ms (1.0x faster!)
Result: {
  "records": [
    {
      "value": {
        "current": {
          "interval": 900,
          "temperature_2m": 6.9,
          "time": "2026-04-27T03:45"
        },
        "current_units": {
          "interval": "seconds",
          "temperature_2m": "\u00b0C",
          "time": "iso8601"
        },
        "elevation": 32.0,
        "generationtime_ms": 0.027060508728027344,
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

🚀 ekoDB Functions Example (Python)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved: dRkEuo37YxfUDIll-q35XWP8m9HbW_rB3_VZUus5JY0tGq9BHwcJ2lJKmbKGXtg7OYN9tflJEIm-hlHYAkbiBQ
📊 Found 10 active users

📝 Example 2: Parameterized Function

✅ Function saved: h8gDG0wI7FlP-wae0QL26LUbjWjKygpRURsnpou5h7Q3LLi6fTWIDF0MiLN5-ZDat1xD2LoNVxgqTuzEGrrpIA
📊 Found 10 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: Kn-dYxDuN4gN5SAzp4cv-5U2XIYRhmr_fpReuXSr7AQnKhjKV9sOveqY5SK8vr4c3sGZz9GOnEF0sO8nXpuZMA
📊 Statistics: 2 groups
   {'avg_score': 60.0, 'count': 5, 'status': 'active'}

   {'avg_score': 50.0, 'count': 5, 'status': 'inactive'}

📝 Example 4: Function Management

📋 Total functions: 63
🔍 Retrieved function: Get Active Users
✏️  Function updated
🗑️  Function deleted

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
   ⏱️  Duration: 42.1ms
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
   ⏱️  Duration: 42.7ms
   📊 Records: 1
   📦 Data: {
      "value": {
            "address": {
                  "city": "Gwenborough",
                  "geo": {
                        "lat": "-37.3159",
                        "lng": "81.1496"
    ...
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
🚀 ekoDB Python Complete Functions Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Function saved: XmsShypTqQIo7GpsasRLh_XJ4HSWzDt52ecyL8EJicCbTafvph-TFSfEjw2tHwfr020uEPA1REAHhyicKZCkaQ
📊 Found 2 product groups
   {'avg_price': 575.6666666666666, 'category': 'Electronics', 'count': 3}
   {'avg_price': 474.0, 'category': 'Furniture', 'count': 2}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   {'category': 'Electronics', 'count': 3}
   {'category': 'Furniture', 'count': 2}
⏱️  Execution time: 0ms

📝 Example 4: High Rating Products

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 5: function with Parameter Definition

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 6: Multi-Stage Pipeline (FindAll → Group → Count)

✅ Function saved
📊 Pipeline executed 3 stages
⏱️  Total execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All complete script examples finished!

💡 This example demonstrates ekoDB's function system:
   ✅ FindAll operations
   ✅ Group aggregations (Count, Average)
   ✅ Multi-stage pipelines (FindAll → Group → Count)
   ✅ Parameter definitions
   ✅ Function management (save, call, delete)
🚀 ekoDB Python KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: 61u2HzzS_Nt_GVqg4CB1Qs8bawLwZziqI3OGwc3FI9eOssRdDiiuMVUofynkghmeHICmSG_oMouW5KWNIuJNxQ
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
🗑️  Deleted session

📝 Example 4: KV Pattern Query

✅ Set 4 config entries
📊 Found 3 app config entries
📊 Found 4 total config entries

📝 Example 5: Combined Wrapped Types + KV Usage

✅ Inserted order: IDiVBeq0c7CE36G0df0lBKE8sgZtQnj0R3YnODSHO1V1gBoMDPT1ITwWjxXCdAuZQfIwHrHwfNblWlxwEEe2Yg
✅ Cached order status
📊 Quick status lookup: {'value': '{"status":"processing","updated_at":"2026-04-27T04:02:07.756829+00:00"}'}

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
✓ Created SWR script: fetch_api_user_py (FHl5AVm_QBjK-f0p3eOqBNfJP06R785wqHIb2887cV9HJa0Z5L0SDg9pIQi0q2RXQhfjUlU4RaHZUkrRN4Cc0g)

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
Response time: 46ms (served from cache)
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
✓ Lightning fast cache hit

🧹 Cleaning up...
✓ Cleanup complete

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation
🚀 ekoDB Python Client - Native SWR Function Examples

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
✓ Created native SWR script: github_user_native (tCh-ClMLDM0LwnIoFBKlKa7GC7FvwHICRlOLoZgKFP531Px5MxpJcS6b_o277qY2rVJxzWjTi-2Tky7g733c-w)

First call (cache miss - will fetch from GitHub API):
  Response time: 214ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 20ms
  Speedup: 10.8x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (savBPV4z3OF0q3bCrq_prw4r2eei7w_yCLdxhZiRsQCGwyTPOvITuxpil86zNjaHWBhcdp6Nvb9NQjTvy2Xp4w)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (3XZNirjCxYTUOXgbwdumxsZ93TGvFuChPuOORgOOlPp1tOid0ofVBUF_u6tfKWUI4IiQF4BWZ8qeG8PDwhwq6w)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (bAt5ukN9yaWBLNnN8ArHq5TO0bRs44ADQOVZnvAL1KR3zNmCmM4tdTWGKL6rT7Z_L-As1rHlBHKWQfD53hX8KQ)
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
   {'avg_price': 365.6666666666667, 'category': 'Furniture', 'count': 3}
   {'avg_price': 367.0, 'category': 'Electronics', 'count': 5}
⏱️  Execution time: 0ms

📝 Example 3: Count Total Products

✅ Function saved
📊 Total products: [{'total': 8}]
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

1. **Efficient Similarity Search**: Optimize retrieval of similar items using vector embeddings, enabling quick access to relevant data.

2. **Scalability**: Handle large-scale datasets efficiently, making them suitable for big data applications.

3. **High-Dimensional Data**: Effectively manage and query high-dimensional data, commonly used in AI and machine learning.

4. **Integration with AI**: Streamline integration with machine learning models, improving the ability to perform tasks like recommendations and image recognition.

5. **Real-Time Applications**: Support real-time analytics and responses, beneficial for applications like chatbots and search engines.

6. **Flexibility**: Can accommodate various data types (text, images, etc.) that can be represented as vectors.

7. **Enhanced Accuracy**: Improve accuracy in tasks such as clustering and classification through advanced indexing techniques.
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
   {'avg_score': 20.0, 'count': 3, 'role': 'admin'}
   {'avg_score': 70.0, 'count': 7, 'role': 'user'}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD script examples finished!
🚀 ekoDB Python Search Functions Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Function saved
📊 Found 5 documents
   1. Vector Databases Explained (Database)
   2. Getting Started with ekoDB (Database)
   3. Natural Language Processing (AI)
   4. Introduction to Machine Learning (AI)
   5. Database Design Principles (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
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
✓ Created session: 84N1aDRgR8ue4aSOQ-05ORWNmsepe0e6P_UCKM7_h1PTmfWbKGC3gYAIjHydMF0wHBQ-QkzeVYdYc3PJY1UGXQ

=== Sending Chat Message ===
Message ID: rDmhZXhaVC7NHSo2Lv0Z2Pvs6QQUm0JJeTCnoxLE-gDN_jfIks8dNhrXJTQfdwXSzcxyhzyX1xigSvt3EqPtUw

=== AI Response ===
The available products and their prices are as follows:

1. **ekoDB**
   - Price: $99
   - Description: A high-performance database product with AI capabilities.

2. **ekoDB Cloud**
   - Price: $499
   - Description: Fully managed cloud database service product.

3. **ekoDB Pro**
   - Price: $299
   - Description: Enterprise edition product with advanced features.

Execution Time: 3306ms

=== Token Usage ===
Prompt tokens: 1994
Completion tokens: 91
Total tokens: 2085

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 12I74R2co5OTqa_WMGsK4FGkwgcYuzxQu8Tc8nU5MTIk4YkBlFXlmyylR4Mug_KZP8jmrA_kf3_esS_7Rt-ZSQ

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: High-performance database product
- **Price**: $99

If you need more information or assistance, feel free to ask!

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['role', 'content', 'context_snippets', 'llm_provider', 'chat_id', 'updated_at', 'created_at', 'llm_model', 'token_usage', 'id'])
Debug: First message role: {'value': 'assistant', 'type': 'String'}
=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of ekoDB is $99.

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: Zhe9N0uaSBpEewM0N8nS75NBHaFcYo6o9eLr7LfNUINHj7b7cv5RlKreQw4xmg-iOvnSXIrQciB1NZ5WY9ggxQ
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
✓ Created session: Zs9LILfnylIiSu1nKL6810rrrzqa5NTxoOCL8euTG7i2rkVS0plbwX9BCdG5BecSlp5YiKJ9K6lgHuQrxQYb8w

=== Sending Messages ===
✓ Message 1 sent
  Response: The following products are available:

1. **Product Name:** ekoDB
   - **Description:** A high-performance database product.
   - **Price:** $99

2. **Product Name:** ekoDB
   - **Description:** A high-performance database product.
   - **Price:** $99

Both entries refer to the same product but have different IDs in the database. If you need more information or want to explore further options, let me know!

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 5QIbi6FP25R2M7RS8oPL00zDmGN1hbeeDX3wxNyONoDCijugtc5Q634Th3lVKK_ZBNm27tR5DDzv7yTR2e_l0Q
  Parent: Zs9LILfnylIiSu1nKL6810rrrzqa5NTxoOCL8euTG7i2rkVS0plbwX9BCdG5BecSlp5YiKJ9K6lgHuQrxQYb8w

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: 5QIbi6FP25R2M7RS8oPL00zDmGN1hbeeDX3wxNyONoDCijugtc5Q634Th3lVKK_ZBNm27tR5DDzv7yTR2e_l0Q (Untitled)
  Session 2: Zs9LILfnylIiSu1nKL6810rrrzqa5NTxoOCL8euTG7i2rkVS0plbwX9BCdG5BecSlp5YiKJ9K6lgHuQrxQYb8w (Untitled)
  Session 3: Zhe9N0uaSBpEewM0N8nS75NBHaFcYo6o9eLr7LfNUINHj7b7cv5RlKreQw4xmg-iOvnSXIrQciB1NZ5WY9ggxQ (Untitled)
  Session 4: 84N1aDRgR8ue4aSOQ-05ORWNmsepe0e6P_UCKM7_h1PTmfWbKGC3gYAIjHydMF0wHBQ-QkzeVYdYc3PJY1UGXQ (Untitled)
  Session 5: 5XPKuizkmR3ypDQ_t5WiFCgjePHLoyt7e3lPokHKvZQOC0bdGUS1hnyBEZ31WDB6i15z1efqoeDbuUFpkJCvPw (Untitled)
  Session 6: 7QABNA53RzBcnLMmJc9amE0GVxWHMMq7Oueo8VT7gcY5763MC499atxCLTvhm5QgEwwa4IgVmrbvoP36oZt54Q (Untitled)
  Session 7: ZJBsEqiQwqcx9ekHH1LJwIvzvzTdnC7Q3gKepN4uFDH24lQvEw86Zt4OjYemdR-kAl1wDJfQUyB27GHW_M8zAA (Untitled)
  Session 8: zjCzvNuIBJD-j7FP0sUgcv7rbLGwQIT2flBbSZ6qbaqRM9Yh8ZJiG41MxeifM5ZoKAIDvp3bTjr6tN49o50Vqg (Untitled)
  Session 9: mNUpjyLF_6S4npa5I0Zn-NNyho77bivYaNsJBamhPedc3c0Dg0IggOQ14m5yVcd0Vu6M5bmSACJyR5HPyU9RbQ (Untitled)
  Session 10: A4fmRct5-4QRoVpCfdmUo-QQDQeroTIx5rYOab2sXhzmkYojQEcMcvX3fst5aHZq8zvdptTIBQUDhQ2oitp6fw (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: 5QIbi6FP25R2M7RS8oPL00zDmGN1hbeeDX3wxNyONoDCijugtc5Q634Th3lVKK_ZBNm27tR5DDzv7yTR2e_l0Q

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Dict Creation ===
✓ Created record with plain dict: {'id': '8IbT3pOlz4oevPzmUYFiCIGmnjaAa_Yrv_2NhD_8pCtRaZ6OBHtzOIatBWP4QuOCooKJwAs9O_2GA7CWSWKfrw'}

=== Upsert Operation ===
✓ Upsert (update existing record): 8IbT3pOlz4oevPzmUYFiCIGmnjaAa_Yrv_2NhD_8pCtRaZ6OBHtzOIatBWP4QuOCooKJwAs9O_2GA7CWSWKfrw
✓ Inserted second record: QMtsvBT7GD55Y1mEI13B48-41lk2gOvy4oxgW3EqCanVU6aIEoOfEuMjSAhKLU3LDnrDaOKGcHgaf7AqsH6TjQ
✓ Upsert (update second record): QMtsvBT7GD55Y1mEI13B48-41lk2gOvy4oxgW3EqCanVU6aIEoOfEuMjSAhKLU3LDnrDaOKGcHgaf7AqsH6TjQ

=== Find One Operation ===
✓ Found user by email: {'name': {'value': 'Alice Johnson', 'type': 'String'}, 'age': {'type': 'Integer', 'value': 29}, 'email': {'value': 'alice.j@newdomain.com', 'type': 'String'}, 'active': {'value': True, 'type': 'Boolean'}, 'id': '8IbT3pOlz4oevPzmUYFiCIGmnjaAa_Yrv_2NhD_8pCtRaZ6OBHtzOIatBWP4QuOCooKJwAs9O_2GA7CWSWKfrw'}
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
   Inserted with ripple: {'id': 'x1yljTyezCW0V4vr5LqbNnE3n77NJ6KUQZ4294rExL2z9QR8VmaXKJKEPXcNqfsZtK4m4pgOlqZS2mUpBwWE4A'}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {'id': 'Jis1muhhK6vldQf5Wcd-AjEMxF0uQHNdNDOUdQx-2j3MzgXW_eZwo-1k1c-4CT3dANb9FBnx_po1GkUzs7kqiA'}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {'name': {'type': 'String', 'value': 'Product 1'}, 'price': {'type': 'Integer', 'value': 150}, 'id': 'x1yljTyezCW0V4vr5LqbNnE3n77NJ6KUQZ4294rExL2z9QR8VmaXKJKEPXcNqfsZtK4m4pgOlqZS2mUpBwWE4A'}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {'price': {'type': 'Integer', 'value': 500}, 'id': 'x1yljTyezCW0V4vr5LqbNnE3n77NJ6KUQZ4294rExL2z9QR8VmaXKJKEPXcNqfsZtK4m4pgOlqZS2mUpBwWE4A', 'name': {'type': 'String', 'value': 'Upsert Product'}}

✅ All bypass_ripple operations completed successfully!
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields (id, name, email only)
  Found 3 active users
  Fields returned: ['email', 'name', 'id']
  First user: {'type': 'String', 'value': 'Alice Johnson'} <{'value': 'alice@example.com', 'type': 'String'}>

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ['status', 'age', 'id', 'name', 'avatar_url', 'email', 'created_at', 'bio', 'user_role']

Example 3: Complex query with projection (active users, ages 18-65)
  Found 3 active users (ages 18-65)
    - {'value': 'Dave Brown', 'type': 'String'} (age {'value': 45, 'type': 'Integer'})
    - {'value': 'Alice Johnson', 'type': 'String'} (age {'type': 'Integer', 'value': 30})
    - {'value': 'Bob Smith', 'type': 'String'} (age {'value': 25, 'type': 'Integer'})

Example 4: Query inactive users with profile fields
  Found 1 inactive users
    - {'value': 'Carol White', 'type': 'String'}: {'value': 'Manager', 'type': 'String'}

Example 5: Compare full vs projected data
  Full query:
    - 12 fields per record
    - Fields: ['status', 'password', 'name', 'api_key', 'bio', 'id', 'age', 'email', 'user_role', 'avatar_url', 'secret_token', 'created_at']
  Projected query:
    - 3 fields per record
    - Fields: ['email', 'id', 'name']
  Bandwidth savings: ~75% fewer fields

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
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

=== Get All Chat Models ===
OpenAI models: ['gpt-4-0613', 'gpt-4', 'gpt-3.5-turbo', 'gpt-5.5-pro-2026-04-23', 'gpt-image-2-2026-04-21', 'gpt-5.5', 'gpt-5.5-2026-04-23', 'gpt-5.5-pro', 'davinci-002', 'babbage-002', 'gpt-3.5-turbo-instruct', 'gpt-3.5-turbo-instruct-0914', 'dall-e-3', 'dall-e-2', 'gpt-3.5-turbo-1106', 'tts-1-hd', 'tts-1-1106', 'tts-1-hd-1106', 'text-embedding-3-small', 'text-embedding-3-large', 'gpt-3.5-turbo-0125', 'gpt-4-turbo', 'gpt-4-turbo-2024-04-09', 'gpt-4o', 'gpt-4o-2024-05-13', 'gpt-4o-mini-2024-07-18', 'gpt-4o-mini', 'gpt-4o-2024-08-06', 'gpt-4o-audio-preview', 'gpt-4o-realtime-preview', 'omni-moderation-latest', 'omni-moderation-2024-09-26', 'gpt-4o-realtime-preview-2024-12-17', 'gpt-4o-audio-preview-2024-12-17', 'gpt-4o-mini-realtime-preview-2024-12-17', 'gpt-4o-mini-audio-preview-2024-12-17', 'o1-2024-12-17', 'o1', 'gpt-4o-mini-realtime-preview', 'gpt-4o-mini-audio-preview', 'o3-mini', 'o3-mini-2025-01-31', 'gpt-4o-2024-11-20', 'gpt-4o-mini-search-preview-2025-03-11', 'gpt-4o-mini-search-preview', 'gpt-4o-transcribe', 'gpt-4o-mini-transcribe', 'o1-pro-2025-03-19', 'o1-pro', 'gpt-4o-mini-tts', 'o3-2025-04-16', 'o4-mini-2025-04-16', 'o3', 'o4-mini', 'gpt-4.1-2025-04-14', 'gpt-4.1', 'gpt-4.1-mini-2025-04-14', 'gpt-4.1-mini', 'gpt-4.1-nano-2025-04-14', 'gpt-4.1-nano', 'gpt-image-1', 'gpt-4o-realtime-preview-2025-06-03', 'gpt-4o-audio-preview-2025-06-03', 'o4-mini-deep-research', 'gpt-4o-transcribe-diarize', 'o4-mini-deep-research-2025-06-26', 'gpt-5-chat-latest', 'gpt-5-2025-08-07', 'gpt-5', 'gpt-5-mini-2025-08-07', 'gpt-5-mini', 'gpt-5-nano-2025-08-07', 'gpt-5-nano', 'gpt-audio-2025-08-28', 'gpt-realtime', 'gpt-realtime-2025-08-28', 'gpt-audio', 'gpt-5-codex', 'gpt-image-1-mini', 'gpt-5-pro-2025-10-06', 'gpt-5-pro', 'gpt-audio-mini', 'gpt-audio-mini-2025-10-06', 'gpt-5-search-api', 'gpt-realtime-mini', 'gpt-realtime-mini-2025-10-06', 'sora-2', 'sora-2-pro', 'gpt-5-search-api-2025-10-14', 'gpt-5.1-chat-latest', 'gpt-5.1-2025-11-13', 'gpt-5.1', 'gpt-5.1-codex', 'gpt-5.1-codex-mini', 'gpt-5.1-codex-max', 'gpt-image-1.5', 'gpt-5.2-2025-12-11', 'gpt-5.2', 'gpt-5.2-pro-2025-12-11', 'gpt-5.2-pro', 'gpt-5.2-chat-latest', 'gpt-4o-mini-transcribe-2025-12-15', 'gpt-4o-mini-transcribe-2025-03-20', 'gpt-4o-mini-tts-2025-03-20', 'gpt-4o-mini-tts-2025-12-15', 'gpt-realtime-mini-2025-12-15', 'gpt-audio-mini-2025-12-15', 'chatgpt-image-latest', 'gpt-5.2-codex', 'gpt-5.3-codex', 'gpt-realtime-1.5', 'gpt-audio-1.5', 'gpt-4o-search-preview', 'gpt-4o-search-preview-2025-03-11', 'gpt-5.3-chat-latest', 'gpt-5.4-2026-03-05', 'gpt-5.4-pro', 'gpt-5.4-pro-2026-03-05', 'gpt-5.4', 'gpt-5.4-nano-2026-03-17', 'gpt-5.4-nano', 'gpt-5.4-mini-2026-03-17', 'gpt-5.4-mini', 'gpt-image-2', 'gpt-3.5-turbo-16k', 'tts-1', 'whisper-1', 'text-embedding-ada-002']
Anthropic models: ['claude-opus-4-7', 'claude-sonnet-4-6', 'claude-opus-4-6', 'claude-opus-4-5-20251101', 'claude-haiku-4-5-20251001', 'claude-sonnet-4-5-20250929', 'claude-opus-4-1-20250805', 'claude-opus-4-20250514', 'claude-sonnet-4-20250514']
Perplexity models: ['sonar', 'sonar-pro', 'sonar-deep-research', 'sonar-reasoning', 'sonar-reasoning-pro']

=== Get OpenAI Models ===
OpenAI models: ['gpt-4-0613', 'gpt-4', 'gpt-3.5-turbo', 'gpt-5.5-pro-2026-04-23', 'gpt-image-2-2026-04-21', 'gpt-5.5', 'gpt-5.5-2026-04-23', 'gpt-5.5-pro', 'davinci-002', 'babbage-002', 'gpt-3.5-turbo-instruct', 'gpt-3.5-turbo-instruct-0914', 'dall-e-3', 'dall-e-2', 'gpt-3.5-turbo-1106', 'tts-1-hd', 'tts-1-1106', 'tts-1-hd-1106', 'text-embedding-3-small', 'text-embedding-3-large', 'gpt-3.5-turbo-0125', 'gpt-4-turbo', 'gpt-4-turbo-2024-04-09', 'gpt-4o', 'gpt-4o-2024-05-13', 'gpt-4o-mini-2024-07-18', 'gpt-4o-mini', 'gpt-4o-2024-08-06', 'gpt-4o-audio-preview', 'gpt-4o-realtime-preview', 'omni-moderation-latest', 'omni-moderation-2024-09-26', 'gpt-4o-realtime-preview-2024-12-17', 'gpt-4o-audio-preview-2024-12-17', 'gpt-4o-mini-realtime-preview-2024-12-17', 'gpt-4o-mini-audio-preview-2024-12-17', 'o1-2024-12-17', 'o1', 'gpt-4o-mini-realtime-preview', 'gpt-4o-mini-audio-preview', 'o3-mini', 'o3-mini-2025-01-31', 'gpt-4o-2024-11-20', 'gpt-4o-mini-search-preview-2025-03-11', 'gpt-4o-mini-search-preview', 'gpt-4o-transcribe', 'gpt-4o-mini-transcribe', 'o1-pro-2025-03-19', 'o1-pro', 'gpt-4o-mini-tts', 'o3-2025-04-16', 'o4-mini-2025-04-16', 'o3', 'o4-mini', 'gpt-4.1-2025-04-14', 'gpt-4.1', 'gpt-4.1-mini-2025-04-14', 'gpt-4.1-mini', 'gpt-4.1-nano-2025-04-14', 'gpt-4.1-nano', 'gpt-image-1', 'gpt-4o-realtime-preview-2025-06-03', 'gpt-4o-audio-preview-2025-06-03', 'o4-mini-deep-research', 'gpt-4o-transcribe-diarize', 'o4-mini-deep-research-2025-06-26', 'gpt-5-chat-latest', 'gpt-5-2025-08-07', 'gpt-5', 'gpt-5-mini-2025-08-07', 'gpt-5-mini', 'gpt-5-nano-2025-08-07', 'gpt-5-nano', 'gpt-audio-2025-08-28', 'gpt-realtime', 'gpt-realtime-2025-08-28', 'gpt-audio', 'gpt-5-codex', 'gpt-image-1-mini', 'gpt-5-pro-2025-10-06', 'gpt-5-pro', 'gpt-audio-mini', 'gpt-audio-mini-2025-10-06', 'gpt-5-search-api', 'gpt-realtime-mini', 'gpt-realtime-mini-2025-10-06', 'sora-2', 'sora-2-pro', 'gpt-5-search-api-2025-10-14', 'gpt-5.1-chat-latest', 'gpt-5.1-2025-11-13', 'gpt-5.1', 'gpt-5.1-codex', 'gpt-5.1-codex-mini', 'gpt-5.1-codex-max', 'gpt-image-1.5', 'gpt-5.2-2025-12-11', 'gpt-5.2', 'gpt-5.2-pro-2025-12-11', 'gpt-5.2-pro', 'gpt-5.2-chat-latest', 'gpt-4o-mini-transcribe-2025-12-15', 'gpt-4o-mini-transcribe-2025-03-20', 'gpt-4o-mini-tts-2025-03-20', 'gpt-4o-mini-tts-2025-12-15', 'gpt-realtime-mini-2025-12-15', 'gpt-audio-mini-2025-12-15', 'chatgpt-image-latest', 'gpt-5.2-codex', 'gpt-5.3-codex', 'gpt-realtime-1.5', 'gpt-audio-1.5', 'gpt-4o-search-preview', 'gpt-4o-search-preview-2025-03-11', 'gpt-5.3-chat-latest', 'gpt-5.4-2026-03-05', 'gpt-5.4-pro', 'gpt-5.4-pro-2026-03-05', 'gpt-5.4', 'gpt-5.4-nano-2026-03-17', 'gpt-5.4-nano', 'gpt-5.4-mini-2026-03-17', 'gpt-5.4-mini', 'gpt-image-2', 'gpt-3.5-turbo-16k', 'tts-1', 'whisper-1', 'text-embedding-ada-002']

=== Get Anthropic Models ===
Anthropic models: ['claude-opus-4-7', 'claude-sonnet-4-6', 'claude-opus-4-6', 'claude-opus-4-5-20251101', 'claude-haiku-4-5-20251001', 'claude-sonnet-4-5-20250929', 'claude-opus-4-1-20250805', 'claude-opus-4-20250514', 'claude-sonnet-4-20250514']

✓ Chat Models API example complete
✓ Client created

=== Create User Function ===
Created user function with ID: z-KeSSF-LmT_5lVPhnFDGe4Ux_zCJgh68GkKDsSUY54XdeWaWC3_gfb4mCj5bs58ASm5lJCI3QwZx8IawzbKGQ

=== Get User Function ===
Retrieved: get_active_users_py - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 70 user functions:
  - validate_user: Check if user exists
  - ingest_integ_test_ingest_748f2fd4: Ingest integ_test_ingest_748f2fd4 from API
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_slim_user: Validate and slim down user
  - get_active_users: Get Active Users (Updated)
  - get_active_users: Get Active Users (Updated)
  - add_measure_inventory_1776919713: Add
  - fetch_slim_user: Validate and slim down user
  - validate_user: Check if user exists
  - fetch_and_store_user: Fetch user from API and cache in KV
  - add_measure_inventory_1776947441: Add
  - add_measure_inventory_1777161754: Add
  - add_measure_inventory_1777127393: Add
  - get_users_by_status: Get Users By Status
  - add_measure_inventory_1777130146: Add
  - add_measure_inventory_1776866928: Add
  - add_measure_inventory_1777125865: Add
  - get_user_wrapper: Wrapper that calls fetch_user
  - get_active_users: Get Active Users (Updated)
  - cache_api_call_py: Cache External API Call
  - fetch_and_store_user: Fetch user from API and cache in KV
  - get_active_users_paginated: Get Active Users (Paginated)
  - fetch_and_store_user: Fetch user from API and cache in KV
  - get_active_users_updated: Get Active Users (Updated)
  - add_measure_inventory_1777160826: Add
  - get_user_wrapper: Wrapper that calls fetch_user
  - add_measure_inventory_1777165093: Add
  - add_measure_inventory_1777065029: Add
  - get_user_wrapper: Wrapper that calls fetch_user
  - fetch_slim_user: Validate and slim down user
  - get_verified_user: Get verified and validated user
  - get_active_users_py: Get Active Users
  - get_verified_user: Get verified and validated user
  - get_verified_user: Get verified and validated user
  - get_high_scoring_active_users: Get High Scoring Active Users
  - fetch_user: Fetch user by code
  - get_user_wrapper: Wrapper that calls fetch_user
  - add_measure_inventory_1777123108: Add
  - get_high_scoring_active_users: Get High Scoring Active Users
  - add_measure_inventory_1777129886: Add
  - get_active_users_paginated: Get Active Users (Paginated)
  - fetch_slim_user: Validate and slim down user
  - get_verified_user: Get verified and validated user
  - add_measure_inventory_1777129909: Add
  - get_active_users_paginated: Get Active Users (Paginated)
  - get_active_users_updated: Get Active Users (Updated)
  - integ_test_bb71e36e: Integration Test Function
  - fetch_user: Fetch user by code
  - fetch_user: Fetch user by code
  - get_active_users_updated: Get Active Users (Updated)
  - cache_api_call_py: Cache External API Call
  - get_active_users_updated: Get Active Users (Updated)
  - get_users_by_status: Get Users By Status
  - add_measure_inventory_1776786378: Add
  - get_users_by_status: Get Users By Status
  - add_measure_inventory_1777164986: Add
  - add_measure_inventory_1777165033: Add
  - validate_user: Check if user exists
  - fetch_user: Fetch user by code
  - add_measure_inventory_1776948968: Add
  - get_active_users: Get Active Users (Updated)
  - swr_user: SWR pattern for user data (KV-based)
  - swr_user: SWR pattern for user data (KV-based)
  - get_users_by_status: Get Users By Status
  - add_measure_inventory_1777124546: Add
  - swr_user: SWR pattern for user data (KV-based)
  - swr_user: SWR pattern for user data (KV-based)
  - validate_user: Check if user exists
  - add_measure_inventory_1776866845: Add
  - get_active_users_paginated: Get Active Users (Paginated)

=== Update User Function ===
User function updated successfully

=== Delete User Function ===
User function deleted successfully

✓ User Functions API example complete
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
✓ py_users_register saved
✓ py_users_login saved
✓ py_users_verify_token saved

=== Auth flow defined as pure stored functions ===
Call them like:
  POST /api/functions/py_users_register { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/py_users_login    { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/py_users_verify_token { "token": "<jwt>" }

✓ Cleaned up demo functions
✓ Client created
✓ crypto_demo_hmac saved
✓ crypto_demo_aes saved
✓ crypto_demo_uuid saved
✓ crypto_demo_totp saved
✓ crypto_demo_encoding saved

Invoke them with:
  POST /api/functions/crypto_demo_hmac     { "payload": "hi" }
  POST /api/functions/crypto_demo_aes      { "plaintext": "secret" }
  POST /api/functions/crypto_demo_uuid
  POST /api/functions/crypto_demo_totp
  POST /api/functions/crypto_demo_encoding { "title": "Héllo World" }

✓ Cleaned up demo functions
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
✅ [32mPython client examples complete![0m
