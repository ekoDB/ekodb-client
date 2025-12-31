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
✓ Authentication successful

=== Insert Document ===
Inserted: map[id:P-NkL2dyd6ykhbukAhUBo4RAaE-Da2FXVx4RMgf2iyRV11SjMCFDObNLpfZkZ59wAZJH4E2bYLFJGsrRibKXww]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] id:P-NkL2dyd6ykhbukAhUBo4RAaE-Da2FXVx4RMgf2iyRV11SjMCFDObNLpfZkZ59wAZJH4E2bYLFJGsrRibKXww name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query ===
Found 1 documents

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] id:P-NkL2dyd6ykhbukAhUBo4RAaE-Da2FXVx4RMgf2iyRV11SjMCFDObNLpfZkZ59wAZJH4E2bYLFJGsrRibKXww name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
[32m✓ simple_crud.go completed successfully[0m
[34m
=== Running simple_websocket.go ===[0m
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: 3Awc9NloAhbSMUXXm8y07-9ANeZvyAbv1gjbq8vfkVThYcYC1A8SY_IsZLqK_tk6hCHn8vk_slodW9-uHJelZA

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
        "id": "BExdULEoFaYZPfAMdPX2qmL8ebqnPB_ZMAWtSsyK8bboegtIiDf1HsJ6a_WgAl-vfT93UVHT98iu3psuczswtA",
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
        "id": "3Awc9NloAhbSMUXXm8y07-9ANeZvyAbv1gjbq8vfkVThYcYC1A8SY_IsZLqK_tk6hCHn8vk_slodW9-uHJelZA",
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
        "id": "OIcUeukTb5AoykKpQDNuJeW36cIo-7vd-q8gp7gPtlnHoXGHToZKYM_zNufISLbfCfwaSx6cT6aRhWPJ5ALqJA",
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
=== Running batch_operations.go ===[0m
✓ Authentication successful

=== Batch Insert ===
Batch insert response: map[failed:[] successful:[9w-UYmeSLzGmaiPbTEXp5wtybjXhMny4fwLWX6RJOqslGjqbFXav76geILwf8yUFskXz8eMRXz8JidZMOT95TA dfL3kfoQIB-MEsJop9oec_TL_asvA4qXsjP1BrojZ1KzQI0dzadHJZed2Ml_Em0F7fgekqmPvg8bqUNikOt_cQ 3dYOuVzz28WN0d-qLYiKrahHupXLCHkpZM-4ffEQYDWlmISO9lJwC30X6Z8q995u1mVPXQO--mwYx7bqckf17A 4BDumCnuE1Y2oM1MjK-RoUDT81Au9Vec_ZO_GSaevEp8D3fhKzxl7bQsWn3l_LXb2VFXkNMaNFLE6uqAXtslaA 6-KcwlHcnMBeB4YgebeNk2hArXnSyiRBPZcxQK891aJMqmIumW6NgXes-9VJ8MgO1lokV3tlmUZy5jSyhc8WPg]]
✓ Batch insert completed

=== Creating test records for update/delete ===
Created 3 test records

=== Batch Update ===
Batch update response: map[failed:[] successful:[nCScBqZZfhdeRflwqkD2drTOO-0JwIdwXBdTZWQNDzxZ8sL8xsuVQVmM9MIgGVt39C3khlOJM4y8XYvjlXFgag 4Jk6JB4g-dm9vsrpHB4aRWCGlGN7brlwL_y2NMW91lygLPPqFz8gH5MdGRV4B-YAp-nULAlr78Z4Cp-luuQ-Lw Io_vJrpWx943ku8rcJ1tn8-jo3xoP90dgA2wUGWWv6pwwZy740zwvBMCpNwsXyjQx0ihUSAh4h3H8KpAn1Pzfw]]
✓ Batch update completed

=== Batch Delete ===
Batch delete response: map[failed:[] successful:[nCScBqZZfhdeRflwqkD2drTOO-0JwIdwXBdTZWQNDzxZ8sL8xsuVQVmM9MIgGVt39C3khlOJM4y8XYvjlXFgag 4Jk6JB4g-dm9vsrpHB4aRWCGlGN7brlwL_y2NMW91lygLPPqFz8gH5MdGRV4B-YAp-nULAlr78Z4Cp-luuQ-Lw Io_vJrpWx943ku8rcJ1tn8-jo3xoP90dgA2wUGWWv6pwwZy740zwvBMCpNwsXyjQx0ihUSAh4h3H8KpAn1Pzfw]]
✓ Batch delete completed

✓ All batch operations completed successfully
[32m✓ batch_operations.go completed successfully[0m
[34m
=== Running kv_operations.go ===[0m
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
✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: aWlSeq-QpUCWg2_uh4jaGjZJxxi5lyguFzIiBZkV3UuhgLEOJyWZ0Z2LbjA2SHA5QkeMqKmUXY1HEnK9cCxN6A

=== List Collections ===
Total collections: 15
Sample collections: [schema_documents_client_py test_accounts ttl_cache demo_collection schema_employees_client_py chat_messages__ek0_testing schema_products_client_py schema_users_client_py chat_configurations__ek0_testing test_collection websocket_test scripts__ek0_testing users batch_users ws_ttl_test]

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
[32m✓ collection_management.go completed successfully[0m
[34m
=== Running document_ttl.go ===[0m
✓ Authentication successful

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: xE6gCfFeU-7aIBWKXXrhtGhpOT8l6fGHAzdiBShj7feBoI-s6lGwWmGhnwz7BYsihtxXcgXjrMcT6dL1z6BSgQ

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: created

=== Query Documents ===
✓ Found 3 documents with TTL

=== Update Document ===
✓ Updated document

=== Delete Document ===
✓ Skipped delete (no ID returned)

✓ All document TTL operations completed successfully
[32m✓ document_ttl.go completed successfully[0m
[34m
=== Running websocket_ttl.go ===[0m
✓ Authentication successful

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: BdkWoO3aXk-WFKmOX53W9w6jj3lwGJGhbcA3UBUSomtsDrGIYU956jJJxgF-rz7udUp9GAWhYQhZAklnC63_JQ

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 3 record(s) via WebSocket

✓ WebSocket TTL example completed successfully
[32m✓ websocket_ttl.go completed successfully[0m
[34m
=== Running http_functions.go ===[0m
🚀 ekoDB Scripts Example (Go/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: 8vujMDffyvyZkGbkrT0S8iYe4BxdXXfdwo7RGo7qm192G-HFXE20dqyN3F1jQmUjjjxRjVNXH8g4SFmaogxQGg
📊 Found 20 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: tpWeB1_CrDgj7wxRq5-nUjsN4qKMiqN--Gh5FGPxAc0i2s-zMO4yOl0u8YoY4CSEOEdARntdO9mhcWeE1ApD4w
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: 6SUWkBXhaxelU1xIOX-WoEchVv6IJDuoblbx4d0b_GbyboDAHGL7baDOxzztED3jQDTmhTbApjbMZeb-hYO1Lw
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"avg_score":60,"count":10,"max_score":100,"status":"active"}
   {"avg_score":50,"count":10,"max_score":90,"status":"inactive"}

📝 Example 4: Function Management

📋 Total scripts: 12
🔍 Retrieved script: Get Active Users
✏️  Script updated
🗑️  Script deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

✅ All examples completed!
[32m✓ http_functions.go completed successfully[0m
[34m
=== Running transactions.go ===[0m
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 5jol88MeR0Hj3tTVb3Hv00rUUJbpIZvLUmIs1RZNXy5MfLBWqu8bmwIm_KSi4BMK1SDA65p1Oov7Nh6ZCNWqEg
Created Bob: $500 - ID: Qt5_bVlhu39ZzF75vqM2ynoYbEnSjWfFwfxwsrLgafyU50ojFOIFMTIFhkYvwvfCnoZkVmvoAxRhcglIFL612w

=== Example 1: Begin Transaction ===
Transaction ID: c406721a-d7ae-4948-8153-e292d2454bca

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
New transaction: 29130e4b-b40f-426d-8625-65a482bfd909
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
   ✅ Script saved: q3y65ABVnIEdQqIOT9j1_Orw6NAunS5mm5mR2uwZ6AwmdjGSbHc6TdYmNBgpPXZ78RVMXPFd50svVhpj0tQqzw

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: 0JwyJNVDEI7aHs4Rv3Tw9mrR6qbOfuXVWZKWu71IsgqWGdN7drQihgMHtFK40nwsanceaOhvTHBqlSumGTeXWg
   📋 Name: map[type:String value:Alice Smith]
   📋 Email: map[type:String value:alice@example.com]
   📋 Status: map[type:String value:pending]
   📋 Credits: map[type:Integer value:0]

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: _198lcqapOrDLeSfz3TQvCViNpqDEXJ23P1q05jMVRPn0xdNr1M47KBWij7ycHri6-GKfRPdxnT8Jk02cBBuMw

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
   ✅ Script saved: TCLyw4ch5UuVZ77CdzGUBruKOzKW6W6tjyyxWJ3wtWOwjg0gYcDAvU46J_ps93QSwSCduyMqyco_WYD7B-l4Ug

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
   ✅ Script saved: n0l63Hu5QQEOfYMHgZhizz4ydEn8np9a-Tdr8KKLHV6JQ84-_9dMzxn3yyz26AEVgHNmr7PaanpwXgn03GmEpw

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: q3y65ABVnIEdQqIOT9j1...
   ✅ Deleted script: _198lcqapOrDLeSfz3TQ...
   ✅ Deleted script: TCLyw4ch5UuVZ77CdzGU...
   ✅ Deleted script: n0l63Hu5QQEOfYMHgZhi...
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
Inserted: map[id:HbHVnAfkF_KIQZlMjCB-lyxd6_sMTfnB6czuC6yhFIxcZsu4lf7-qPgASvxK5hGEMqKsIfRMZEazrT6GJhTp1Q]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] id:HbHVnAfkF_KIQZlMjCB-lyxd6_sMTfnB6czuC6yhFIxcZsu4lf7-qPgASvxK5hGEMqKsIfRMZEazrT6GJhTp1Q name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] id:HbHVnAfkF_KIQZlMjCB-lyxd6_sMTfnB6czuC6yhFIxcZsu4lf7-qPgASvxK5hGEMqKsIfRMZEazrT6GJhTp1Q name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

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
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: Uj-0AIYlsj-jsRxfhEbAhQA1rAY8-4JlfejjN19CcCZSYf6PowSuN_OC1438t0-w0sIAz00YjJvTQcmhzw7Uzg

=== List Collections ===
Total collections: 14
Sample collections: [schema_documents_client_py test_accounts ttl_cache client_collection_management_go schema_employees_client_py]

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: v0c4l5pdvDnSBM_BrEHZXOtKBW8gB0PnjUsM_ilnF6cyaNDC9mk22j6HsAzWmH1Z16DhMn1PvAkPSqiu_MGpZA

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: P9sfWehva8yv9YgvecEbI0QdoTzn23goXZp5mHNs_832t60myFzQT787DuSt6Jkn7sIgEsADUPKIlQpQCIHHeg

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
✓ Inserted test record: ep5JcWH4ESe3f5nJFy1yRb1A7VcHtJ_bghWqD7xwptcPhIaXr5DQYIZY-oW7lwFhSixc1UKtnRvCHM0FexLcwA

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
✓ Inserted document with TTL: H2jiQ7OZNMri6NnNxKUosjcc1ZVijyhOYo2bzWXMGQzLV9VW1-ZkSBNpWYphFbAJ_0ydbLhRcL9_ZHy3dzKgGg

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 5 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: pQWCgFFgYFbFf0-jWzKOK9RLY4tsqicjbx8LxOuYK0LBIYghOzqeIR9HfgO3zC-uTqYpJXGDlQCeUP-hr4dx-Q

=== Sending Chat Message ===
Message ID: REQh9Jh4u_CX9dlrDPdM3jczgI8831m1Nh3OKnBOoWK70gBAwqaHM8aMyju06IH_3ceXiji1lqcFML1oKTwYkA

=== AI Response ===
We have three products available:

1. ekoDB Cloud: This is a fully managed cloud database service product and it is priced at $499.
2. ekoDB Pro: This is an enterprise edition product with advanced features and it costs $299.
3. ekoDB: This is a high-performance database product with AI capabilities and it's priced at $99.

=== Context Used (3 snippets) ===
  Snippet 1: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Fully managed cloud database service product id:z9tf3Oiffyo5KmIOT2SyBinRDg0Xa2ktcyRFWMtzrnD0cImH47vPabt4-oqVKZcgrYsw-OOlo5-g1rVgOFvc3A name:ekoDB Cloud price:499] score:0.1111111111111111]
  Snippet 2: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Enterprise edition product with advanced features id:-a3HgAxbTl6I75vo7m3EuW_YuE-8ROx_lk_CK67Wy9pUIH1FQ61XaC5jIM8lq5zkL2Xk_UJFLCaoLMcME2B1LQ name:ekoDB Pro price:299] score:0.1111111111111111]
  Snippet 3: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:A high-performance database product with AI capabilities id:Yg7EG8R1H2AKQLrsCWpQ29YUinp8TmKX0sDW0cDL7fZFdC2Rrcrm7slFl6U4le4FXY7tXhVvfTLhIKK5pdOJCg name:ekoDB price:99] score:0.1111111111111111]

Execution Time: 2282ms

=== Token Usage ===
Prompt tokens: 612
Completion tokens: 76
Total tokens: 688

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: ktjiFm4CMvu9friJvcO-UNMXZW-N5pvOqhSV5J0RQaumxSwqCx_9DOTqv_D4m8xDuwia2e8IlDKi6GMyvMAJ1A

=== Sending Messages ===
✓ Message 1 sent
  Response: Answer: One product available is ekoDB, a high-performance database product. It is priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: E91K5oUwrv2t-1DY-MFs0kA_QcjeUl4V7geaRUEg5PxTIIhSQF4ckDGSQ0g2H7p0RHeR9t680bt_BApSoa1NaA
  Parent: ktjiFm4CMvu9friJvcO-UNMXZW-N5pvOqhSV5J0RQaumxSwqCx_9DOTqv_D4m8xDuwia2e8IlDKi6GMyvMAJ1A

=== Listing Sessions ===
✓ Found 9 sessions
  Session 1: E91K5oUwrv2t-1DY-MFs0kA_QcjeUl4V7geaRUEg5PxTIIhSQF4ckDGSQ0g2H7p0RHeR9t680bt_BApSoa1NaA (Untitled)
  Session 2: ktjiFm4CMvu9friJvcO-UNMXZW-N5pvOqhSV5J0RQaumxSwqCx_9DOTqv_D4m8xDuwia2e8IlDKi6GMyvMAJ1A (Untitled)
  Session 3: pQWCgFFgYFbFf0-jWzKOK9RLY4tsqicjbx8LxOuYK0LBIYghOzqeIR9HfgO3zC-uTqYpJXGDlQCeUP-hr4dx-Q (Untitled)
  Session 4: MeoMxNtgzw5KEIQyZX_n66uvkZPV8Fq5Dj2SVWOUflZxg7wgntyjQc2cHezCDrKXgAXeh3-ZzT4XJZ1weC8g0g (Untitled)
  Session 5: cCXRhshO2K1u4oEgzhUn5CeenXcQlZ06uX7cjsO_4W1GoVAHNY_mU7IYMPGxynYUv3_wNHDa1sL0B9uJ7os78A (Untitled)
  Session 6: escAHAP-FcRf2PLZuTgFmENQqrE6LEvMLOpY5U5tFaxBmw5IVW1Sydk9FL3pfilwUfCqQCyiCuC8Wv5vC3eE0Q (Untitled)
  Session 7: hyGmrj0Z3bTf09rh4aobr5bED81Zs2p4vAC3-HmLu1uJjUvxMG6N94KSZw2qnG7vBqdW60KHJcX7K5l_eD_Yag (Untitled)
  Session 8: LDJLO-5YuyhN72uoGjH8oZkLQWc_B7ARiZYwyG-8rDhRttuosxeeXgu_2eRIr0XKRfQH1xKZk8_QkxAiUoAEeA (Untitled)
  Session 9: OYjJR7pVbKoYxcBDhajVQoErv88QB3r08vuGOXPIKr3sG79g5agf2DVqZivkEABQRz95uBQ_hm8b9krr1yc3mw (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: E91K5oUwrv2t-1DY-MFs0kA_QcjeUl4V7geaRUEg5PxTIIhSQF4ckDGSQ0g2H7p0RHeR9t680bt_BApSoa1NaA

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: GQvup186beTL5mRy1QaWbbMfrdCLKRKl51qJ7G_16-CwtLdHMWeWyIhJEJKskvWw-hk7qHPC1QrF5RWTTSrxWQ

=== Sending Initial Message ===
✓ Message sent
  Response: The available product based on the context provided is ekoDB. It is a high-performance database product. The price for this product is $99.

✓ Second message sent

=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of ekoDB is 99.

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: VA7G5nZiKrl6H8Ktj8nH23Bj4oSh0YEpFgFWaZxXLiRKU0n1n5G0bE1a4-s4iz-JVycQt-QexpJHbpdRQfue-A
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
  2. Score: 0.756
  3. Score: 0.750

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.707
  2. Score: 1.502
  3. Score: 0.300

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
🚀 ekoDB Scripts Example (Go Client)

✅ Client initialized

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: kcIOhzQkOFdOVn3n-zSxtSxkBE0X6jxjz_MUIpexHDtwGc32M_QfimzcupI_WkxTcP_tgu5YGIoMsGISEAJ6vA
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

📋 Total scripts: 14
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
✅ [32mGo client examples complete![0m
