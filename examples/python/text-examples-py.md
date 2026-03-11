make test-examples-python
🧪 [36mRunning Python examples (direct HTTP/WebSocket)...[0m
=== Simple CRUD Operations (Direct HTTP) ===

✓ Authentication successful

=== Insert Document ===
Inserted: {'id': 'OtdX0UHUVjQbcrdJsv_Yk3jAdeRnrQC8a4W3SPIuoYJzLn8HzDxVWbN9LgYkbJm6pNUWA4IsuHlDtBuQVjQoBg'}

=== Find by ID ===
Found: {'name': {'value': 'Test Record', 'type': 'String'}, 'id': 'OtdX0UHUVjQbcrdJsv_Yk3jAdeRnrQC8a4W3SPIuoYJzLn8HzDxVWbN9LgYkbJm6pNUWA4IsuHlDtBuQVjQoBg', 'value': {'type': 'Integer', 'value': 42}, 'active': {'type': 'Boolean', 'value': True}}

=== Find with Query ===
Found documents: [{'value': {'value': 42, 'type': 'Integer'}, 'id': 'OtdX0UHUVjQbcrdJsv_Yk3jAdeRnrQC8a4W3SPIuoYJzLn8HzDxVWbN9LgYkbJm6pNUWA4IsuHlDtBuQVjQoBg', 'name': {'value': 'Test Record', 'type': 'String'}, 'active': {'type': 'Boolean', 'value': True}}]

=== Update Document ===
Updated: {'id': 'OtdX0UHUVjQbcrdJsv_Yk3jAdeRnrQC8a4W3SPIuoYJzLn8HzDxVWbN9LgYkbJm6pNUWA4IsuHlDtBuQVjQoBg', 'active': {'value': True, 'type': 'Boolean'}, 'value': {'value': 100, 'type': 'Integer'}, 'name': {'type': 'String', 'value': 'Updated Record'}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: NAx3s7Ff8DkZt9PSpE9wpo5Z2NtasI9oiUQGokyIYHkCk3StEiTomYqroVm1eSjDnAGZun89TDtqHNbwBnrvSg

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
        "id": "3iNacHlOrLs5T9hDNOvghQkwkRQEp81IZ51fafX7mvm0W7-T_AZYHMyPzDRppGPdUb895ykT2KkFB-AigS1bvA",
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
        "id": "Sb7wwI0SJG9yEJuFRPtwizcAVp3BIN9p3UyfWjfyM5n9cEdWvf9ZUo0KW0Ec26o-aNqu9lX6Zba7BE9ObFt3Rg",
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
        "id": "Zc7PqlMFMO49_B56vubOWFv8H9g5CTInhjVIV0lpLko9dqy1wx-tOd6_XeZVKvkAupa_wFsi-bkSpqVKZi10nQ",
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
        "id": "NAx3s7Ff8DkZt9PSpE9wpo5Z2NtasI9oiUQGokyIYHkCk3StEiTomYqroVm1eSjDnAGZun89TDtqHNbwBnrvSg",
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
        "id": "biFqh34zaiKvmVm_AsobaPP65GYmXSLsFcsLJGV9TsX701PhiY4YQht5Ic36wfZ3pL3js_GmW27bm485ujU8TQ",
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
        "id": "mWL_XtxbXMBH3hWNMk9n3vMoBGe_1M5OEi7hTywqjnKnkJ0V2cgt-RIlZwazqu4TZoKlkSyciuKWunFUsgBttw",
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
✓ Retrieved 6 record(s) via WebSocket

✓ WebSocket example completed successfully
🚀 ekoDB Scripts Example (Python/HTTP)

✓ Authentication successful

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: XpBVbOYIhTw722B-UAKo5ZZ89b-5QQwLUiV1PzsevJmNTfkZfL2I6Zcrv-bQG0rBmLkrejHSi7klefNGmW8ngQ
📊 Found 15 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: Qr-rB28zsPiLt4OiZ4itDYWTgvauJpxJ1QUVP16knisiXivjRwiy-yQ8xtADyKOzvPQZacU8O8-yk8u5XgnyVg
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: NHH1jcffRud4y4EvAdsUQcXLMIWEEvZN00LV2BtdWk5SG99TmqKycXhTRYfBGCqljGrjcEA0ROXTszESa7N9mg
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {'avg_score': 50.0, 'max_score': 90, 'status': 'inactive', 'count': 15}
   {'status': 'active', 'count': 15, 'max_score': 100, 'avg_score': 60.0}

📝 Example 4: Function Management

📋 Total scripts: 91
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
✓ Verified: Found 30 total records in collection

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
cache:product:1: {'name': 'Product 1', 'price': 29.99}
cache:product:2: {'name': 'Product 2', 'price': 39.989999999999995}
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
Collection created with first record: 9sr2H530RKzyMwpG5H-bbtjX8UaANHncnpB4Ks_ckxvfcA6xkaQxAmSqoDHg-5l1LstAyTf7VFgQiUTvNRpPzg

=== List Collections ===
Total collections: 36
Sample collections: ['schema_documents_client_go', 'ws_subscribe_example_go', 'schema_employees_client_js', 'schema_products_client_js', 'chat_configurations__ek0_testing']

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
Created Alice: $1000 - ID: FgoHzN5rDuySiQFT5Jev0A4XW2_FcWKifOH3CwNhY_ptXmQh44S4gBsc84RORzqo6B3fshDU5DubJMMnI9SEJw
Created Bob: $500 - ID: fYIunFAR6q7fNbWmOjTR7fCEWZBkHokpCAZ8jsZMY2idvOLcU0QdT2Hq9PAo8Gzrx9SWRGk2l0mE34eePwpa6A

=== Example 1: Begin Transaction ===
Transaction ID: 3f5fb9ec-8289-42ca-8a30-b246361f951a

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
New transaction: 254ac789-c70f-4a5e-8f84-6fce761876a6
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'value': 700, 'type': 'Integer'}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: zgGA4_QLTKOB70jHtc3nDavo-NNN3AwtCSC0uG-iUESTRNmKky93jAqklLm0mCgVx42OnfHFLSqRtP3n6OIXWw
Created Bob: $500 - ID: GyNDNq1o7Yk-a5nLn1g6iCLu-FaeWUR0YNN_mx2i6vo0crZ7qEl_MNt8vevsDIKBGdp1n7q3QPnGlIfEyX5yiA

=== Example 1: Begin Transaction ===
Transaction ID: 19eb686f-bb53-4998-b028-165f26288c56

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
Created Alice: $1000 - ID: aWMclNvVkaJrnQoOr-fxMVb5TkK2hPST_X8UqKCn7kH2HbetxfAqrgCp516Q2W8__tXTHhEjJJnHfWPEcdDH6w
Created Bob: $500 - ID: xkhP4WN_ua_tO3eklbkb-saLV-TEM3siqNaLiisMkGcIX6Dun365-f6VbshJvG3Qz1exLJs2onwMF8oY2AqPig

=== Example 1: Begin Transaction ===
Transaction ID: 1a88aea1-fc31-4798-8b87-cc7babcd17f5

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
New transaction: 5fc7611c-c505-4415-b4ad-77db49abc773
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'value': 700, 'type': 'Integer'}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: WJmpnk0-A-fm5z2MxJSzINCMl3NLzaNV_D5cqZ65ENzlfbKORGAEXahbdPLPD2m6BNV3v6guDBe6bMhtVNyErw
Created Bob: $500 - ID: VRBzxa1eYBR9tIYf0syePz3oK2TOaqSulRVALD0VKuNBieFzSMXn2fYmldwFlvOf2YwLS0buWjc81YHo9SSolA

=== Example 1: Begin Transaction ===
Transaction ID: 207defc3-c950-440c-a0c0-6da3c9e61bb7

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


🔄 Switching to DURABLE mode...
   ✓ Config updated: durable_operations=true


============================================================
🔥 TEST 3: Durable Mode (durable=true)
============================================================

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: SCua5AzrYBiV7zPRtBRLDqyPLXyBW7n4iR9qjZCqdB6g3rf9bvR8F5QTugXC8kKHZv9x0f-vTk6LDejHb-d06A
Created Bob: $500 - ID: izzs9ynvRHrc9NBSHr9OviVU-pCOYsg10K1YPdRQV319KTu2E1PeifbNFE6vyLRcOY1ov5mXF_FiXpYNhRnGGw

=== Example 1: Begin Transaction ===
Transaction ID: 3cb2aa7e-e7b5-4fde-9f70-a3a2a4c5a3a1

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
Bob: ${'value': 700, 'type': 'Integer'}

=== Example 5: Rollback ===
New transaction: dbc94cfb-8004-4d87-a2ee-20780bd13a9f
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'type': 'Integer', 'value': 700}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: k5Bcm3Li8vCCnxgiW9N_rH41Vr0J7W6CZqyUwMWBLkUrWZi--V5FZBMwoM0Rb4afwN4y9vFXJYM6XVpfqg_Acw
Created Bob: $500 - ID: XUhfJRre5uk4jveA1naX6DBEViBSu4lVaSvHxBd9bBhVg1d86wmQ0jpTzqwnWr_EbeELzzK4GmqBx8y_i6xh7w

=== Example 1: Begin Transaction ===
Transaction ID: 9f5f4245-c082-4b4e-991f-24982fc86f58

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
   ✅ Script saved: R-_wxxiy8e8dLIxouIenRFAGwChOcwrdOfYkZ95hJrZ3ZAiZujFM6AcLcwVWBeTGT26fzqpk3HrP8NOEG67_cA

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: EKXkO7nor7w3O0ld0wnfRBEWVUpyYH2BRJkJBpBKoD6WulJLsoIMqq6owkZjfLCLwvRskc5X227qfzmx7FL8AQ
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}
   📋 Email: {'value': 'alice@example.com', 'type': 'String'}
   📋 Status: {'value': 'pending', 'type': 'String'}
   📋 Credits: {'value': 0, 'type': 'Integer'}

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: sy7qpvaRcJd2sVkLIi-yqXGKgewBuZa9eKwTR9HVtyiduIrTQYt2ke29kQH3dzpqtKC6VqWmGT1LZ5m0a3cHpw

2️⃣ Calling Script (Query + Update + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Status updated to: {'value': 'active', 'type': 'String'}
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}

============================================================
📝 Script 3: Query + Update Credits + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: qlOq4QEvj0O32yD5qiUiSte8C8L015ayj7OggoD8Nx-wu3Da_ww0Glctx4CGWZeQCvrkCWKupLdX7-N1eACD7g

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
   ✅ Script saved: jowFnZQ0ErxYJS7D1S-OddV0Fp8V4Dkfkt4i3QL5-ADkTSi7Lc_9sQOZhZZZnQ6vr2dgLOKoyOSNaUvfq_wYiw

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: R-_wxxiy8e8dLIxouIen...
   ✅ Deleted script: sy7qpvaRcJd2sVkLIi-y...
   ✅ Deleted script: qlOq4QEvj0O32yD5qiUi...
   ✅ Deleted script: jowFnZQ0ErxYJS7D1S-O...
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
  Output: Document ID = l6wQXuH8uKApoRCNKLbFzX0NlM9AyJ8wF5WQyXKW7WDNd21E0idqdPTUResAFUnz6v0VxznthPEv-5sS1hub5Q
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: find_by_id(l6wQXuH8uKApoRCNKLbFzX0NlM9AyJ8wF5WQyXKW7WDNd21E0idqdPTUResAFUnz6v0VxznthPEv-5sS1hub5Q)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: find_by_id(l6wQXuH8uKApoRCNKLbFzX0NlM9AyJ8wF5WQyXKW7WDNd21E0idqdPTUResAFUnz6v0VxznthPEv-5sS1hub5Q)
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
  Output: Document ID = 8Fzr5l3UxAri4Pt9oqMRLOme-LSH40aKdaa90oIHZSyv4OKM9nVYj2cGkz1N1oBKfVsLSMY1UTn8-Atpx8NDbQ
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: find_by_id(8Fzr5l3UxAri4Pt9oqMRLOme-LSH40aKdaa90oIHZSyv4OKM9nVYj2cGkz1N1oBKfVsLSMY1UTn8-Atpx8NDbQ)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: find_by_id(8Fzr5l3UxAri4Pt9oqMRLOme-LSH40aKdaa90oIHZSyv4OKM9nVYj2cGkz1N1oBKfVsLSMY1UTn8-Atpx8NDbQ)
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
    Finished `release` profile [optimized] target(s) in 0.09s
📦 Built wheel for abi3 Python ≥ 3.8 to /Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.11.0-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel for current platform...[0m
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.11.0-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.11.0
    Uninstalling ekodb_client-0.11.0:
      Successfully uninstalled ekodb_client-0.11.0
Successfully installed ekodb-client-0.11.0
✅ [32mPython client package built and installed![0m
🧪 [36mRunning Python client library examples...[0m
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {'id': 'l1YXcwxXiqH3vQjTCl17TSIj2rOV6TKj27tjO858TVjlYJVUo4EA-fq0MBo1FMobF_-i3Loz0HKz-WgZa3zlUA'}

=== Find by ID ===
Found: {'metadata': {'value': {'key': 'value', 'nested': {'deep': True}}, 'type': 'Object'}, 'user_id': {'value': '550e8400-e29b-41d4-a716-446655440000', 'type': 'String'}, 'price': {'value': 99.99, 'type': 'Float'}, 'value': {'value': 42, 'type': 'Integer'}, 'active': {'type': 'Boolean', 'value': True}, 'tags': {'type': 'Array', 'value': ['tag1', 'tag2', 'tag3']}, 'categories': {'type': 'Array', 'value': ['electronics', 'computers']}, 'name': {'type': 'String', 'value': 'Test Record'}, 'data': {'type': 'String', 'value': 'aGVsbG8gd29ybGQ='}, 'created_at': {'value': '2026-03-11T01:05:45.843933', 'type': 'String'}, 'id': 'l1YXcwxXiqH3vQjTCl17TSIj2rOV6TKj27tjO858TVjlYJVUo4EA-fq0MBo1FMobF_-i3Loz0HKz-WgZa3zlUA', 'embedding': {'type': 'Array', 'value': [0.1, 0.2, 0.3, 0.4, 0.5]}}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): True
  price (Decimal): 99.99
  created_at (DateTime): 2026-03-11 01:05:45.843933
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): ['tag1', 'tag2', 'tag3']
  metadata (Object): {'key': 'value', 'nested': {'deep': True}}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): ['electronics', 'computers']
  data (Bytes): 11 bytes
Plain record: {'metadata': {'key': 'value', 'nested': {'deep': True}}, 'user_id': '550e8400-e29b-41d4-a716-446655440000', 'price': 99.99, 'value': 42, 'active': True, 'tags': ['tag1', 'tag2', 'tag3'], 'categories': ['electronics', 'computers'], 'name': 'Test Record', 'data': 'aGVsbG8gd29ybGQ=', 'created_at': '2026-03-11T01:05:45.843933', 'id': 'l1YXcwxXiqH3vQjTCl17TSIj2rOV6TKj27tjO858TVjlYJVUo4EA-fq0MBo1FMobF_-i3Loz0HKz-WgZa3zlUA', 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5]}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'active': {'type': 'Boolean', 'value': True}, 'value': {'value': 100, 'type': 'Integer'}, 'tags': {'type': 'Array', 'value': ['tag1', 'tag2', 'tag3']}, 'user_id': {'value': '550e8400-e29b-41d4-a716-446655440000', 'type': 'String'}, 'categories': {'value': ['electronics', 'computers'], 'type': 'Array'}, 'metadata': {'type': 'Object', 'value': {'key': 'value', 'nested': {'deep': True}}}, 'price': {'type': 'Float', 'value': 99.99}, 'created_at': {'value': '2026-03-11T01:05:45.843933', 'type': 'String'}, 'id': 'l1YXcwxXiqH3vQjTCl17TSIj2rOV6TKj27tjO858TVjlYJVUo4EA-fq0MBo1FMobF_-i3Loz0HKz-WgZa3zlUA', 'data': {'type': 'String', 'value': 'aGVsbG8gd29ybGQ='}, 'embedding': {'type': 'Array', 'value': [0.1, 0.2, 0.3, 0.4, 0.5]}, 'name': {'value': 'Updated Record', 'type': 'String'}}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: -E0tPwSMKJ3PfUkfkXA2nVGa7X1K-3cP49ZeNxhE8EntInD9rdjTscBaGhoMT1M-e9Iit8tBMpWVVcZMHLGi1A

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
Collection created with first record: "T4SgvKkIQlh2zt4eyIZL7lqi2ORl2cFj3ate-5SbmOGRpbseYrk3e38DSxA-gXWO__8B_G0HDXaih3xxNUUjVA"

=== List Collections ===
Total collections: 35
Sample collections: ['schema_documents_client_go', 'ws_subscribe_example_go', 'schema_employees_client_js', 'schema_products_client_js', 'chat_configurations__ek0_testing']

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
Total keys in store: 12

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
Created Alice: $1000 - ID: hg75XpGg8i0NFrxSkyrkCtvkbIYUnnbyxFRDoBBvWcFbzUmbRqbZVERH_R26b676jMdxUe23gZpfT2cuNzy6Iw
Created Bob: $500 - ID: 2U06TiuYc0o5o5XrV3ZRr9OVp-0NIg07Jl8celbVwdcfla5PCgw-Mv9WYzDZaG0n6m5VSG9UOFMB9rV92HF2Xg

=== Example 1: Begin Transaction ===
Transaction ID: c828e5a3-2138-4b3b-aba2-0159eb160ab0

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 6952570a-b11d-4dc2-9a83-33efe4217731
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
  1. Score: 25.740, Matched: name, email, name.value, email.value
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title, title.value, bio, bio.value
  2. Score: 26.400, Matched: title.value, bio, title, bio.value
  3. Score: 26.400, Matched: title.value, bio.value, title, bio
  4. Score: 26.400, Matched: title.value, bio.value, title, bio

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio.value, bio, title, title.value
  2. Score: 39.600, Matched: title, title.value, bio, bio.value
  3. Score: 39.600, Matched: bio, title.value, title, bio.value
  4. Score: 39.600, Matched: title, bio.value, bio, title.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 2 results (matches: run, running, runs)
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.778
  2. Score: 0.758
  3. Score: 0.741

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.389, Matched: title, title.value, content.value, content
  2. Score: 1.371, Matched: content, title, content.value, title.value
  3. Score: 0.379, Matched: 

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
✓ Inserted document: Jgza5pCJaNUiS7Gk0afqK1ktUNb2h7DJa71kc3m9fACh2q66WKmFmItXhhMjLjyoElciA4kEJ7QUf-f6hPEMLg

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: YtAI4qTfqeWgvMiO4ALUkA7O8fAk18J1WQ8t5h-3TrKh0VlMVZQUmYl1E9Y2d7lpzpNz-OAQGzXmthabRKlX2w

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
✓ Inserted document with TTL: 3LVZW8VXWz_30j2IHcsVQqfGkFBD5nAixZhVos5AklIBTTJlxDgxhHbWtsxM_kdwm1AYCkqnLqET5VLEB28UmQ

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
✓ Edge cache script created: Z-xrQhu5yi9aknstSf7CMOpQpJap9H2RhL_jfMAkcq-IqDuy6hYxcGaVixn-izEwYlu-PuixRTtrreUD83pOLA

Call 1: Cache miss (fetches from API)
Response time: 446ms
Result: {
  "records": [
    {
      "value": {
        "current": {
          "interval": 900,
          "temperature_2m": 16.3,
          "time": "2026-03-11T05:00"
        },
        "current_units": {
          "interval": "seconds",
          "temperature_2m": "\u00b0C",
          "time": "iso8601"
        },
        "elevation": 32.0,
        "generationtime_ms": 0.023365020751953125,
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
Response time: 20ms (21.8x faster!)
Result: {
  "records": [
    {
      "value": {
        "current": {
          "interval": 900,
          "temperature_2m": 16.3,
          "time": "2026-03-11T05:00"
        },
        "current_units": {
          "interval": "seconds",
          "temperature_2m": "\u00b0C",
          "time": "iso8601"
        },
        "elevation": 32.0,
        "generationtime_ms": 0.023365020751953125,
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

✅ Script saved: yUfGz_0LtQES3oVyPiKmbxgKP5CP1A4mBtech3m5JgGjMYNggRtawvDmg5wFtRO5K86-87x7vlP9X-1gLARl3Q
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved: ZP0pUROQN9kspvxVQdm8sADY78baF4PiSiquZps7dYWLVXwP4JK3P9sOwdPp61KR-VINsCsGRoYXjjfTDDQTFQ
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: 1iWXTGgPAO21FMPlQpRPTbI2dHHB_ZvBT5CdoUaR-di1QWygxHy3mVeqjRbgJrcvRxOP7ePqsC6ksgagWyV7lg
📊 Statistics: 2 groups
   {'avg_score': 60.0, 'count': 5, 'status': 'active'}

   {'avg_score': 50.0, 'count': 5, 'status': 'inactive'}

📝 Example 4: Script Management

📋 Total scripts: 94
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
   ⏱️  Duration: 3.8ms
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
   ⏱️  Duration: 3.8ms
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
🚀 ekoDB Python Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved: JhvitxQiXTDMGfurnCeU5q6AL18wLzoiKLj_QMilDAZ4RinPJLfilGrDw7Sy0dO8AsHIuOrxaM5Ar1aV9dSufw
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

✅ Inserted order: d5pIh6bEKppg7UVnIsoD33vJX5W7XEUclHdut1ymKCj1XuT_TnMAOWBFbRt9uM0BKE60oAEOG4J_-k24Poi6vw
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
🗑️  Deleted session

📝 Example 4: KV Pattern Query

✅ Set 4 config entries
📊 Found 3 app config entries
📊 Found 4 total config entries

📝 Example 5: Combined Wrapped Types + KV Usage

✅ Inserted order: t69akfuH94DMlrZ41w09F2NcQbkPmoI_YOTB68VMOcqI_1bRiXE5nw32R1rHPw9kej91SoXEhQc_FHKIFIovBQ
✅ Cached order status
📊 Quick status lookup: {'value': '{"status":"processing","updated_at":"2026-03-11T05:05:48.244961+00:00"}'}

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
✓ Created SWR script: fetch_api_user_py (nq51YrL6aMJB9TNtqCz2aN5JEGjx6JaBtyCt5UhIFuHVXTRdi2L8rGc1CGbJF6_d1ojeB6Q6ncbzrx_JHfvlmQ)

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
Response time: 5ms (served from cache)
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
✓ Created native SWR script: github_user_native (AQLOOR9MAGf7r32tD1kEyOo0CL7jbVUTV0s7WXpwwqYyLTNGKPJdWT_WNtzP6JXz6STTZogNMfi97DXdZUK0Qw)

First call (cache miss - will fetch from GitHub API):
  Response time: 119ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 10ms
  Speedup: 12.1x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (uOqACx3MBbBxwxqV3dnN1dbwgXdU5XfaefBVJI9J43y76-_tljhICi9S25ElYqES1aGCBllypIJNjMnc2F9UAA)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (z_3CjA6HjBvrjGvXHoXIYLNeW2wwri3ze0fixPwGSM8UDDLGRAJ9spoTyUe9mHJOp-2jpDiz85eLO6fGOC22kw)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (70FHns3vTEGxt0KTQzwp0G2zA8Lptp3VgBZXOzPa-LbGEefI5bYRlCevfh8rbuQpz9keR38zMcs9pSLpk6ciZQ)
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
   1. High Accuracy: Vector data is very accurate compared to raster data.
2. Less Data Storage: Requires less storage as only vertices are stored.
3. Flexible: It's easier to update and modify as you only need to edit points rather than entire images.
4. Topology: Vector databases allow for topological data like network connections and contiguity.
5. Better Visualization: They can provide clearer and more detailed representations.
6. Scalability: Vector data can be scaled up or down without losing quality.
7. Attribute Storage: It can store diverse attribute data, like object identifiers, names, categories, etc.
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
   inactive: 3 users
   active: 7 users
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
   3. Vector Databases Explained (Database)
   4. Natural Language Processing (AI)
   5. Database Design Principles (Database)
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
✓ Created session: 5xAXUajg47q6V-E_32OCJiQ01hyd3PNCnRneYDdTLcd5NM_tc5u9PSWf7j6c3nTmXxKr6ioRgg7Rrap-9Te0TQ

=== Sending Chat Message ===
Message ID: -hFRwb7myDa9BJkGL1AcSwl9nCL7KUJBAZm7CNTYH0yBMP9zTAHVW_A0pK85Apm9eVce9xfLbfsiV51i-5HVDQ

=== AI Response ===
We have three products available:

1. The "ekoDB" is a high-performance database product with AI capabilities, and it is priced at $99.

2. The "ekoDB Cloud" is a fully managed cloud database service product, the price of which is $499.

3. The "ekoDB Pro" is our Enterprise edition product with advanced features, available for $299.

Execution Time: 3903ms

=== Token Usage ===
Prompt tokens: 456
Completion tokens: 78
Total tokens: 534

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: aBdfXf-4UkjEqo8A_c56EhlKAY510NyU1YSHrSYL5EKDcm4rBTgLVfCWxvSupmZHfAfvZ6cqLZWNTPm4wgrqnQ

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the available context, one product available is the "ekoDB". It is described as a high-performance database product and the cost is $99.

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['role', 'id', 'content', 'updated_at', 'context_snippets', 'chat_id', 'created_at', 'token_usage'])
Debug: First message role: {'type': 'String', 'value': 'assistant'}
=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of the ekoDB is $99.

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: hM5YYVmZtmGzOVtaRJcmpTAHVPGSKoEQ0lYcIGxoPQQpnnvNmLZAFXS2K29vmSF-cdd2B4o7O4YePM6Jf9u6IQ
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
✓ Created session: 5-koefiH3sbQO341ilYlYK_OY5toxJC5d_pgjhiuQHQyMpKeT4QtjsFbEfFRnw2VRaFadDcquDugbBqfOniolw

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is "ekoDB". It is a high-performance database product. It's priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: ctlE_5Bk_2lDtfdo1SHEirLO92jQIW6nWG8NGNIuPP0ma9aVT0lTZLtZP3-aKnpdZJdV92GR7KuZIXE_UgP8Xg
  Parent: 5-koefiH3sbQO341ilYlYK_OY5toxJC5d_pgjhiuQHQyMpKeT4QtjsFbEfFRnw2VRaFadDcquDugbBqfOniolw

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: ctlE_5Bk_2lDtfdo1SHEirLO92jQIW6nWG8NGNIuPP0ma9aVT0lTZLtZP3-aKnpdZJdV92GR7KuZIXE_UgP8Xg (Untitled)
  Session 2: 5-koefiH3sbQO341ilYlYK_OY5toxJC5d_pgjhiuQHQyMpKeT4QtjsFbEfFRnw2VRaFadDcquDugbBqfOniolw (Untitled)
  Session 3: hM5YYVmZtmGzOVtaRJcmpTAHVPGSKoEQ0lYcIGxoPQQpnnvNmLZAFXS2K29vmSF-cdd2B4o7O4YePM6Jf9u6IQ (Untitled)
  Session 4: 5xAXUajg47q6V-E_32OCJiQ01hyd3PNCnRneYDdTLcd5NM_tc5u9PSWf7j6c3nTmXxKr6ioRgg7Rrap-9Te0TQ (Untitled)
  Session 5: CPPoqpxkc85Q7aA0TR4rdrFfCaB5JLrF473_duRXGUWp5tSYwkpxBvajc-LBkJIMN7bUFbDDrSwMz0U-WJC1xg (Untitled)
  Session 6: ax2TZvpXSW6UDWu0Xd8rZaoDJrgv69xu9g5fS83AE1dSontfpcLCH6P3BE-CLYXeCok1uzWoZpK28TjUifVgkg (Untitled)
  Session 7: 8IOhGsKhzWZ5pJ2jOMsad4ch7Ygibm8LTPidQoC0fR-AY-sP1NcEYJrMBdJbYCGjUMVAN5oUeCwVmBkz57l7fQ (Untitled)
  Session 8: Yhc2N-Pqqr8IU508CIEFc3oER9d4IOiKbgFBF2n6_BQscKjXHM6hP7nLxXnGhzIxyu_XX9x-iRYZDfaDMljRCw (Untitled)
  Session 9: AhOtBciodRbS1dfyuu2Prr7OL9iYyY6X3pThfJmnU0N7rIHR6nGLFWGxmCa9qoHd5PutUhyO6N1YjMftiGlmew (Untitled)
  Session 10: m2PRPT3uxebcBfaJ5p8MsdLhkYp17mxXs_4SGh4bfLj61Z4Oh-JcqvyXCD1mnnd_V90zya5jakxzdqlJ5cmq6w (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: ctlE_5Bk_2lDtfdo1SHEirLO92jQIW6nWG8NGNIuPP0ma9aVT0lTZLtZP3-aKnpdZJdV92GR7KuZIXE_UgP8Xg

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Dict Creation ===
✓ Created record with plain dict: {'id': 'WG0qwzokSDOpFE4N5uJ0dnP54E-lQZ70cVdyAmeDWMsRz8INBdGxrDNUr_m3_ZY81f189Zp3hdGf4Wv_YCEeQQ'}

=== Upsert Operation ===
✓ Upsert (update existing record): WG0qwzokSDOpFE4N5uJ0dnP54E-lQZ70cVdyAmeDWMsRz8INBdGxrDNUr_m3_ZY81f189Zp3hdGf4Wv_YCEeQQ
✓ Inserted second record: xr-h1YRGOY5Ky2e4Tx7VwFyN2Kysi1mWiNfDLEQisFUrPFZVc-GJ-9lQKS-6G_pERKzzHiGbV0oKmJOE_vnbIw
✓ Upsert (update second record): xr-h1YRGOY5Ky2e4Tx7VwFyN2Kysi1mWiNfDLEQisFUrPFZVc-GJ-9lQKS-6G_pERKzzHiGbV0oKmJOE_vnbIw

=== Find One Operation ===
✓ Found user by email: {'id': 'WG0qwzokSDOpFE4N5uJ0dnP54E-lQZ70cVdyAmeDWMsRz8INBdGxrDNUr_m3_ZY81f189Zp3hdGf4Wv_YCEeQQ', 'age': {'value': 29, 'type': 'Integer'}, 'active': {'value': True, 'type': 'Boolean'}, 'email': {'value': 'alice.j@newdomain.com', 'type': 'String'}, 'name': {'value': 'Alice Johnson', 'type': 'String'}}
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
   Inserted with ripple: {'id': 'Vzqeeb3UIxdr8igCweVW6mH1DcVIRYX2Uok1OF9auZLL88Yv7iBGGyFpn43Mkg0b8amEWDb20O3Bp5ADVpk_ag'}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {'id': 'OdwPAVo1iJ6kKWlRESazcJz58t1345PlB-lP047Tpbgt0aBtYwGSU-oMmOjDaTrv8o5cupjzhogNtz5V6KpPqA'}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {'id': 'Vzqeeb3UIxdr8igCweVW6mH1DcVIRYX2Uok1OF9auZLL88Yv7iBGGyFpn43Mkg0b8amEWDb20O3Bp5ADVpk_ag', 'name': {'type': 'String', 'value': 'Product 1'}, 'price': {'type': 'Integer', 'value': 150}}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {'name': {'type': 'String', 'value': 'Upsert Product'}, 'id': 'Vzqeeb3UIxdr8igCweVW6mH1DcVIRYX2Uok1OF9auZLL88Yv7iBGGyFpn43Mkg0b8amEWDb20O3Bp5ADVpk_ag', 'price': {'type': 'Integer', 'value': 500}}

✅ All bypass_ripple operations completed successfully!
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields (id, name, email only)
  Found 3 active users
  Fields returned: ['email', 'id', 'name']
  First user: {'type': 'String', 'value': 'Bob Smith'} <{'value': 'bob@example.com', 'type': 'String'}>

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ['created_at', 'avatar_url', 'name', 'user_role', 'bio', 'status', 'age', 'email', 'id']

Example 3: Complex query with projection (active users, ages 18-65)
  Found 3 active users (ages 18-65)
    - {'type': 'String', 'value': 'Dave Brown'} (age {'value': 45, 'type': 'Integer'})
    - {'value': 'Alice Johnson', 'type': 'String'} (age {'value': 30, 'type': 'Integer'})
    - {'value': 'Bob Smith', 'type': 'String'} (age {'value': 25, 'type': 'Integer'})

Example 4: Query inactive users with profile fields
  Found 1 inactive users
    - {'type': 'String', 'value': 'Carol White'}: {'value': 'Manager', 'type': 'String'}

Example 5: Compare full vs projected data
  Full query:
    - 12 fields per record
    - Fields: ['password', 'api_key', 'bio', 'status', 'age', 'created_at', 'avatar_url', 'email', 'id', 'secret_token', 'user_role', 'name']
  Projected query:
    - 3 fields per record
    - Fields: ['id', 'email', 'name']
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
OpenAI models: ['gpt-4-0613', 'gpt-4', 'gpt-3.5-turbo', 'gpt-5.4', 'gpt-5.3-chat-latest', 'gpt-5.4-2026-03-05', 'gpt-5.4-pro', 'gpt-5.4-pro-2026-03-05', 'davinci-002', 'babbage-002', 'gpt-3.5-turbo-instruct', 'gpt-3.5-turbo-instruct-0914', 'dall-e-3', 'dall-e-2', 'gpt-4-1106-preview', 'gpt-3.5-turbo-1106', 'tts-1-hd', 'tts-1-1106', 'tts-1-hd-1106', 'text-embedding-3-small', 'text-embedding-3-large', 'gpt-4-0125-preview', 'gpt-4-turbo-preview', 'gpt-3.5-turbo-0125', 'gpt-4-turbo', 'gpt-4-turbo-2024-04-09', 'gpt-4o', 'gpt-4o-2024-05-13', 'gpt-4o-mini-2024-07-18', 'gpt-4o-mini', 'gpt-4o-2024-08-06', 'gpt-4o-audio-preview', 'gpt-4o-realtime-preview', 'omni-moderation-latest', 'omni-moderation-2024-09-26', 'gpt-4o-realtime-preview-2024-12-17', 'gpt-4o-audio-preview-2024-12-17', 'gpt-4o-mini-realtime-preview-2024-12-17', 'gpt-4o-mini-audio-preview-2024-12-17', 'o1-2024-12-17', 'o1', 'gpt-4o-mini-realtime-preview', 'gpt-4o-mini-audio-preview', 'o3-mini', 'o3-mini-2025-01-31', 'gpt-4o-2024-11-20', 'gpt-4o-mini-search-preview-2025-03-11', 'gpt-4o-mini-search-preview', 'gpt-4o-transcribe', 'gpt-4o-mini-transcribe', 'o1-pro-2025-03-19', 'o1-pro', 'gpt-4o-mini-tts', 'o3-2025-04-16', 'o4-mini-2025-04-16', 'o3', 'o4-mini', 'gpt-4.1-2025-04-14', 'gpt-4.1', 'gpt-4.1-mini-2025-04-14', 'gpt-4.1-mini', 'gpt-4.1-nano-2025-04-14', 'gpt-4.1-nano', 'gpt-image-1', 'gpt-4o-realtime-preview-2025-06-03', 'gpt-4o-audio-preview-2025-06-03', 'o4-mini-deep-research', 'gpt-4o-transcribe-diarize', 'o4-mini-deep-research-2025-06-26', 'gpt-5-chat-latest', 'gpt-5-2025-08-07', 'gpt-5', 'gpt-5-mini-2025-08-07', 'gpt-5-mini', 'gpt-5-nano-2025-08-07', 'gpt-5-nano', 'gpt-audio-2025-08-28', 'gpt-realtime', 'gpt-realtime-2025-08-28', 'gpt-audio', 'gpt-5-codex', 'gpt-image-1-mini', 'gpt-5-pro-2025-10-06', 'gpt-5-pro', 'gpt-audio-mini', 'gpt-audio-mini-2025-10-06', 'gpt-5-search-api', 'gpt-realtime-mini', 'gpt-realtime-mini-2025-10-06', 'sora-2', 'sora-2-pro', 'gpt-5-search-api-2025-10-14', 'gpt-5.1-chat-latest', 'gpt-5.1-2025-11-13', 'gpt-5.1', 'gpt-5.1-codex', 'gpt-5.1-codex-mini', 'gpt-5.1-codex-max', 'gpt-image-1.5', 'gpt-5.2-2025-12-11', 'gpt-5.2', 'gpt-5.2-pro-2025-12-11', 'gpt-5.2-pro', 'gpt-5.2-chat-latest', 'gpt-4o-mini-transcribe-2025-12-15', 'gpt-4o-mini-transcribe-2025-03-20', 'gpt-4o-mini-tts-2025-03-20', 'gpt-4o-mini-tts-2025-12-15', 'gpt-realtime-mini-2025-12-15', 'gpt-audio-mini-2025-12-15', 'chatgpt-image-latest', 'gpt-5.2-codex', 'gpt-5.3-codex', 'gpt-realtime-1.5', 'gpt-audio-1.5', 'gpt-4o-search-preview', 'gpt-4o-search-preview-2025-03-11', 'gpt-3.5-turbo-16k', 'tts-1', 'whisper-1', 'text-embedding-ada-002']
Anthropic models: ['claude-sonnet-4-6', 'claude-opus-4-6', 'claude-opus-4-5-20251101', 'claude-haiku-4-5-20251001', 'claude-sonnet-4-5-20250929', 'claude-opus-4-1-20250805', 'claude-opus-4-20250514', 'claude-sonnet-4-20250514', 'claude-3-haiku-20240307']
Perplexity models: ['sonar', 'sonar-pro', 'sonar-deep-research', 'sonar-reasoning', 'sonar-reasoning-pro']

=== Get OpenAI Models ===
OpenAI models: ['gpt-4-0613', 'gpt-4', 'gpt-3.5-turbo', 'gpt-5.4', 'gpt-5.3-chat-latest', 'gpt-5.4-2026-03-05', 'gpt-5.4-pro', 'gpt-5.4-pro-2026-03-05', 'davinci-002', 'babbage-002', 'gpt-3.5-turbo-instruct', 'gpt-3.5-turbo-instruct-0914', 'dall-e-3', 'dall-e-2', 'gpt-4-1106-preview', 'gpt-3.5-turbo-1106', 'tts-1-hd', 'tts-1-1106', 'tts-1-hd-1106', 'text-embedding-3-small', 'text-embedding-3-large', 'gpt-4-0125-preview', 'gpt-4-turbo-preview', 'gpt-3.5-turbo-0125', 'gpt-4-turbo', 'gpt-4-turbo-2024-04-09', 'gpt-4o', 'gpt-4o-2024-05-13', 'gpt-4o-mini-2024-07-18', 'gpt-4o-mini', 'gpt-4o-2024-08-06', 'gpt-4o-audio-preview', 'gpt-4o-realtime-preview', 'omni-moderation-latest', 'omni-moderation-2024-09-26', 'gpt-4o-realtime-preview-2024-12-17', 'gpt-4o-audio-preview-2024-12-17', 'gpt-4o-mini-realtime-preview-2024-12-17', 'gpt-4o-mini-audio-preview-2024-12-17', 'o1-2024-12-17', 'o1', 'gpt-4o-mini-realtime-preview', 'gpt-4o-mini-audio-preview', 'o3-mini', 'o3-mini-2025-01-31', 'gpt-4o-2024-11-20', 'gpt-4o-mini-search-preview-2025-03-11', 'gpt-4o-mini-search-preview', 'gpt-4o-transcribe', 'gpt-4o-mini-transcribe', 'o1-pro-2025-03-19', 'o1-pro', 'gpt-4o-mini-tts', 'o3-2025-04-16', 'o4-mini-2025-04-16', 'o3', 'o4-mini', 'gpt-4.1-2025-04-14', 'gpt-4.1', 'gpt-4.1-mini-2025-04-14', 'gpt-4.1-mini', 'gpt-4.1-nano-2025-04-14', 'gpt-4.1-nano', 'gpt-image-1', 'gpt-4o-realtime-preview-2025-06-03', 'gpt-4o-audio-preview-2025-06-03', 'o4-mini-deep-research', 'gpt-4o-transcribe-diarize', 'o4-mini-deep-research-2025-06-26', 'gpt-5-chat-latest', 'gpt-5-2025-08-07', 'gpt-5', 'gpt-5-mini-2025-08-07', 'gpt-5-mini', 'gpt-5-nano-2025-08-07', 'gpt-5-nano', 'gpt-audio-2025-08-28', 'gpt-realtime', 'gpt-realtime-2025-08-28', 'gpt-audio', 'gpt-5-codex', 'gpt-image-1-mini', 'gpt-5-pro-2025-10-06', 'gpt-5-pro', 'gpt-audio-mini', 'gpt-audio-mini-2025-10-06', 'gpt-5-search-api', 'gpt-realtime-mini', 'gpt-realtime-mini-2025-10-06', 'sora-2', 'sora-2-pro', 'gpt-5-search-api-2025-10-14', 'gpt-5.1-chat-latest', 'gpt-5.1-2025-11-13', 'gpt-5.1', 'gpt-5.1-codex', 'gpt-5.1-codex-mini', 'gpt-5.1-codex-max', 'gpt-image-1.5', 'gpt-5.2-2025-12-11', 'gpt-5.2', 'gpt-5.2-pro-2025-12-11', 'gpt-5.2-pro', 'gpt-5.2-chat-latest', 'gpt-4o-mini-transcribe-2025-12-15', 'gpt-4o-mini-transcribe-2025-03-20', 'gpt-4o-mini-tts-2025-03-20', 'gpt-4o-mini-tts-2025-12-15', 'gpt-realtime-mini-2025-12-15', 'gpt-audio-mini-2025-12-15', 'chatgpt-image-latest', 'gpt-5.2-codex', 'gpt-5.3-codex', 'gpt-realtime-1.5', 'gpt-audio-1.5', 'gpt-4o-search-preview', 'gpt-4o-search-preview-2025-03-11', 'gpt-3.5-turbo-16k', 'tts-1', 'whisper-1', 'text-embedding-ada-002']

=== Get Anthropic Models ===
Anthropic models: ['claude-sonnet-4-6', 'claude-opus-4-6', 'claude-opus-4-5-20251101', 'claude-haiku-4-5-20251001', 'claude-sonnet-4-5-20250929', 'claude-opus-4-1-20250805', 'claude-opus-4-20250514', 'claude-sonnet-4-20250514', 'claude-3-haiku-20240307']

✓ Chat Models API example complete
✓ Client created

=== Create User Function ===
Created user function with ID: smYSiYO-NEImtcfjbfZcvJkrfHCV00vk_5LXEf-le1PJe259FMPxd4GBHpDCQPpqtHHJf_HC5wcrnssQN7s92A

=== Get User Function ===
Retrieved: get_active_users_py - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 101 user functions:
  - swr_user: SWR pattern for user data (KV-based)
  - get_verified_user: Get verified and validated user
  - fetch_user: Fetch user by code
  - get_verified_user: Get verified and validated user
  - get_active_users_updated: Get Active Users (Updated)
  - validate_user: Check if user exists
  - swr_user: SWR pattern for user data (KV-based)
  - validate_user: Check if user exists
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_product_with_reviews: Fetch Product with Reviews (Multi-API)
  - swr_user: SWR pattern for user data (KV-based)
  - get_active_users_paginated: Get Active Users (Paginated)
  - get_users_by_status: Get Users By Status
  - fetch_slim_user: Validate and slim down user
  - fetch_product_with_reviews: Fetch Product with Reviews (Multi-API)
  - swr_user: SWR pattern for user data (KV-based)
  - swr_user: SWR pattern for user data (KV-based)
  - fetch_slim_user: Validate and slim down user
  - cache_api_call: Cache External API Call
  - get_active_users: Get Active Users (Updated)
  - get_user_wrapper: Wrapper that calls fetch_user
  - get_verified_user: Get verified and validated user
  - get_user_wrapper: Wrapper that calls fetch_user
  - fetch_user: Fetch user by code
  - get_user_wrapper: Wrapper that calls fetch_user
  - cache_api_call: Cache External API Call
  - validate_user: Check if user exists
  - swr_user: SWR pattern for user data (KV-based)
  - get_active_users_paginated: Get Active Users (Paginated)
  - fetch_slim_user: Validate and slim down user
  - fetch_product_enriched: Fetch Product with Enrichment
  - validate_user: Check if user exists
  - validate_user: Check if user exists
  - fetch_and_store_user: Fetch user from API and cache in KV
  - get_user_wrapper: Wrapper that calls fetch_user
  - get_verified_user: Get verified and validated user
  - get_active_users: Get Active Users (Updated)
  - fetch_slim_user: Validate and slim down user
  - fetch_slim_user: Validate and slim down user
  - fetch_api_user: Fetch User with Cache
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_user: Fetch user by code
  - fetch_user: Fetch user by code
  - fetch_and_store_user: Fetch user from API and cache in KV
  - validate_user: Check if user exists
  - cache_api_call_py: Cache External API Call
  - fetch_slim_user: Validate and slim down user
  - swr_user: SWR pattern for user data (KV-based)
  - cache_api_call_js: Cache External API Call
  - swr_user: SWR pattern for user data (KV-based)
  - validate_user: Check if user exists
  - fetch_user: Fetch user by code
  - get_active_users_updated: Get Active Users (Updated)
  - fetch_user: Fetch user by code
  - fetch_user: Fetch user by code
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_user: Fetch user by code
  - fetch_product_enriched: Fetch Product with Enrichment
  - fetch_slim_user: Validate and slim down user
  - get_verified_user: Get verified and validated user
  - get_high_scoring_active_users: Get High Scoring Active Users
  - fetch_product_enriched: Fetch Product with Enrichment
  - fetch_slim_user: Validate and slim down user
  - validate_user: Check if user exists
  - swr_user: SWR pattern for user data (KV-based)
  - validate_user: Check if user exists
  - swr_user: SWR pattern for user data (KV-based)
  - get_active_users_py: Get Active Users
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_github_user: Fetch GitHub User with Cache
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_and_store_user: Fetch user from API and cache in KV
  - get_user_wrapper: Wrapper that calls fetch_user
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_github_user: Fetch GitHub User with Cache
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_user: Fetch user by code
  - fetch_slim_user: Validate and slim down user
  - fetch_slim_user: Validate and slim down user
  - fetch_github_user: Fetch GitHub User with Cache
  - fetch_github_user: Fetch GitHub User with Cache
  - fetch_product_enriched: Fetch Product with Enrichment
  - validate_user: Check if user exists
  - swr_user: SWR pattern for user data (KV-based)
  - get_verified_user: Get verified and validated user
  - validate_user: Check if user exists
  - get_user_wrapper: Wrapper that calls fetch_user
  - fetch_user: Fetch user by code
  - fetch_user: Fetch user by code
  - get_user_wrapper: Wrapper that calls fetch_user
  - cache_api_call_py: Cache External API Call
  - fetch_slim_user: Validate and slim down user
  - fetch_user: Fetch user by code
  - fetch_slim_user: Validate and slim down user
  - swr_user: SWR pattern for user data (KV-based)
  - fetch_api_user: Fetch User with Cache
  - get_verified_user: Get verified and validated user
  - validate_user: Check if user exists
  - get_users_by_status: Get Users By Status

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
✅ [32mPython client examples complete![0m
