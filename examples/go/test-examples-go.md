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
Inserted: map[id:Y8FKg6f9KyR7VMMK9ebVoR_aXZXCKlBnFyN1qz-g5qhB519nZYYfbXlAelousMRwnzrnqupk3fX9tSat0zjZ7Q]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] id:Y8FKg6f9KyR7VMMK9ebVoR_aXZXCKlBnFyN1qz-g5qhB519nZYYfbXlAelousMRwnzrnqupk3fX9tSat0zjZ7Q name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query ===
Found 1 documents

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] id:Y8FKg6f9KyR7VMMK9ebVoR_aXZXCKlBnFyN1qz-g5qhB519nZYYfbXlAelousMRwnzrnqupk3fX9tSat0zjZ7Q name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
[32m✓ simple_crud.go completed successfully[0m
[34m
=== Running simple_websocket.go ===[0m
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: EHRclmzT3prR90JJIQl3JV0OnzQ_zdm5GHUZwl1JG0Dv33EQi6Elici_xRW6FSSDrWR4Yjl4f0a7s25WdNchNw

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
        "id": "IclmydnNxR1P-E0R7bBIuBKTF1Ka65Q1s32FLDl28hTOjoHAqNCeuCWCZsfDHQ4J5KOp9vNZEg0ayYtaSPXtQA",
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
        "id": "EHRclmzT3prR90JJIQl3JV0OnzQ_zdm5GHUZwl1JG0Dv33EQi6Elici_xRW6FSSDrWR4Yjl4f0a7s25WdNchNw",
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
        "id": "fGqA_U1JCaXbYlEiFASIABYGZ-Ckhe9tGhJXqSIoaSMrfQnRLfvqHABUQDbp2Vb2nk6WYTT1jQOIKE8UhS3SGQ",
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
Batch insert response: map[failed:[] successful:[o-OIpgSkzcKq6rzMP7ENJOrwHRKXYoYTcPx5r2RtI5jO64iyATu6OMAMXNNPai0W1ER8HJom7eYgbhpq-3m6_A CdqAfMnNOUbuppm5Z9FCc-bSDu6BIaR5TvA7-dWy_b5ekm_mYrpH-HpckcKtLu5uUSMndxq7bs0XaXwfoNKqPQ vWGWGozqwLS2hAeB3r13TXzI_7pye9pGNm-54T5n8hqmmqyPIi53RiwbaeQYByx7rpiE57-l8Ck964Y9otSyZQ vUbjWZ5n6WIq7-gj2MA20XDxUoc2QqMeN-dBlq0cqKGQoae_L833Kk9X_7ZBXMSjzvHGTUEPPWOyZvpPxIXwog Z88usFFBqZBhsxc3sQORW2-Oim-eSqc6r7cQfwPGXbBlj-mrhgRzYfh2Q8d9CMccMqKbgW-KLbhWlVHb5U9LHw]]
✓ Batch insert completed

=== Creating test records for update/delete ===
Created 3 test records

=== Batch Update ===
Batch update response: map[failed:[] successful:[BXI3_WHUuiNf9ryzxMGabatr_xZYkdLge4wgTwqIoZnhQHM89L-RdvG6spb4r3rai7kTa8WPyvi3TWOVnOSGHA kKFZnPw6UORaQ3JdRirXzJg5onRF-_ikhoSLvTgTaGMfMgwFXYjCcJ6ecFVQWwUsop2RF-sP0aDqzZAsy_HSvw VGeFUgkc6hwTIT80taX_cKL8Lfk82AiyJyQnQvOfvSiIyspy4JTg01dizRp3UYK4yLKS0T6HXUB28jGPL_G9qQ]]
✓ Batch update completed

=== Batch Delete ===
Batch delete response: map[failed:[] successful:[BXI3_WHUuiNf9ryzxMGabatr_xZYkdLge4wgTwqIoZnhQHM89L-RdvG6spb4r3rai7kTa8WPyvi3TWOVnOSGHA kKFZnPw6UORaQ3JdRirXzJg5onRF-_ikhoSLvTgTaGMfMgwFXYjCcJ6ecFVQWwUsop2RF-sP0aDqzZAsy_HSvw VGeFUgkc6hwTIT80taX_cKL8Lfk82AiyJyQnQvOfvSiIyspy4JTg01dizRp3UYK4yLKS0T6HXUB28jGPL_G9qQ]]
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
Collection created with first record: 6yg94eSOflTCrjJWdLpAotYockSlE_gHOtFJ9Lg5pOBX5HJ71bhZitKQFx80v_Mt141JRqFvf1T0chL_c1k_EQ

=== List Collections ===
Total collections: 15
Sample collections: [websocket_test ttl_cache schema_employees_client_py scripts__ek0_testing users test_accounts test_collection chat_messages__ek0_testing demo_collection schema_products_client_py ws_ttl_test schema_documents_client_py chat_configurations__ek0_testing schema_users_client_py batch_users]

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
✓ Inserted document: OwEAdE5wlWrZ4vj1gKcg-1DM7DgSSUEkXCgD471X8ebpAyaaifqcpVRiL9GYC25yNH6gEtlJeGn_8TkzWRKcdg

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
✓ Inserted document with TTL: 3Qvlu7sRUFQe1VliCtGbZDYgPMkE888dYMeDGi6s5UxEg_3BSYikTEXITqfFh68xpuwohf97IyFWfmLtWd51VA

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

✅ Script saved: WxqBvWoFmJyDNndQtPGDFViMedUpGBtKL_m-otK66wOJbf2PopXxNZ3ms2A38AjPdxbKICmgddY67N-_m-rccA
📊 Found 20 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: iOgWp9RqkpWBwZI_TdERo_tlrt1QRSxxyRQvCxGYI7xEK35X4iUwKx_K1PSKgn8jpR-6PIvepNLIdSVz87nAtA
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: f39VXPsnS648G3VZuQw0EewfHyMMDKePcA6sBLobYPVeyzw-6s5rQK00xaY_W9HAN7xTZdiznYJH0m4LiFXwIQ
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"avg_score":50,"count":10,"max_score":90,"status":"inactive"}
   {"avg_score":60,"count":10,"max_score":100,"status":"active"}

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
Created Alice: $1000 - ID: 6tuk1-ogCKuUnxnw0Y6kDhQx1We-dmT1dE-2sRLoewsIXsdX_bkZ2yUlzda8ZjXqr-T9RWQ2-gFnLa92V8Ct1A
Created Bob: $500 - ID: O6EZbzaPAbFJvkMH49nUjBgWUzgW4IR-W0z8sSu6OXU42OYpd74OFQ10L0GlSvvgP926kFIHIZTS_PEeAbkQNA

=== Example 1: Begin Transaction ===
Transaction ID: 0f4014cd-9bd3-49b1-a61e-9123428cca6b

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
New transaction: 866ab7c1-e5dc-4d67-aaa1-bfadba597d07
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
   ✅ Script saved: zwRK1GgTsHj_7vatX4Tij24H-Kpv796CTwxdtTnsuvpCWtYVRCRqKklOmz6JlS7kwWVig6VwxxJWLtCY9IP-ww

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: cY5mRt2XWFIrbS3JMobcVqBqnzAEfyjG427mCUD5Fgtp-3BLPnS4fCO1apWwoDOrlf2oheRKWi8a9EK3i7nykQ
   📋 Name: map[type:String value:Alice Smith]
   📋 Email: map[type:String value:alice@example.com]
   📋 Status: map[type:String value:pending]
   📋 Credits: map[type:Integer value:0]

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: MXPgwDPbJ9jAMAUSDlO5vupytBhv9IhYUHSQGOPvMpoVZE_mLWIvF-XPuZI9dz6P3idrcggJ91W2lcu5Hl1ejw

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
   ✅ Script saved: 38VAUcWDDVgsA4p9rnLJgLmVDq-gW-jSAf5K1J2zsMi2fh2Ea2e2Zx9i92jxyJjnGevLAsvQbd1fkvXwMO4MUQ

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
   ✅ Script saved: N67jrfCFhTK18SRtlBv6M_ijCjeGe5iq6AESob7B_d1pzE8fqVudQ9o4bmCmds_rhG1jdb8Va9-1cxKKFjne8g

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: zwRK1GgTsHj_7vatX4Ti...
   ✅ Deleted script: MXPgwDPbJ9jAMAUSDlO5...
   ✅ Deleted script: 38VAUcWDDVgsA4p9rnLJ...
   ✅ Deleted script: N67jrfCFhTK18SRtlBv6...
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
Inserted: map[id:NhoI2JCCw_7uQdS0k5QMRTPKtqekd5E0-MgKUGfAKqhkaxKEQ0AOjsO28f_UwNSy8Ks8vMM4QUVElGiAgvs-Mw]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] id:NhoI2JCCw_7uQdS0k5QMRTPKtqekd5E0-MgKUGfAKqhkaxKEQ0AOjsO28f_UwNSy8Ks8vMM4QUVElGiAgvs-Mw name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] id:NhoI2JCCw_7uQdS0k5QMRTPKtqekd5E0-MgKUGfAKqhkaxKEQ0AOjsO28f_UwNSy8Ks8vMM4QUVElGiAgvs-Mw name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

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
Collection created with first record: OFz5NdyLxAc_EsAKBHGM7mpvws_LGoYtrkub-OdNaZW-yNxzoxF0hdMkAgGE6QxgBSmPCDC12V2453tu8i3cyg

=== List Collections ===
Total collections: 14
Sample collections: [websocket_test ttl_cache schema_employees_client_py scripts__ek0_testing test_accounts]

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: pnLsOSRBwoWoaZXY43rbj-naHts_hEKguyRaJ_9rjcwTISuE94Se1Fg5RnSXkHXsxrgsdNJQxondxU8_a2O_2g

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: 5q0m_DYEB1lzDQ2z2Rjw8-hrpxbZJrtcK1S5gBrT7kO0wmMzSRgQtr6_tHPntiKRKGtlhuaovhre5-XmF0kxXg

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
✓ Inserted test record: pjQ1BvqnjONb7fBPOOkkvSfmQayUWIt5nln3jD6K8Ze-_5lSFTKwUpn8Ohgg2Mm3QJUU_jZMm6of2lDcQG73DQ

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
✓ Inserted document with TTL: -R1rxlBUSivVIY-kIqV49_qt1YyUnMjuaOViA2Hmjy998OhI3hAnc7leokZT73bJ59Ij8AmxOZP2nMTBikHQWQ

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
✓ Created session: BFktfJOMJ7YljgfQGEQDsMgJIZ-0uhKET-2tVlgR9fOhkzOHWvpdYvlxe2DZcxoZd1fmY8aweNcjmyR3ZbNI-A

=== Sending Chat Message ===
Message ID: o5Qa0Kucc4gTDzEw2DKsfLKLwp9Aq90swocF0OBKV0rCQxWfRVsHyrH140ptz7nuLnnsetg-TLF_XaGcJNNXCg

=== AI Response ===
The available products along with their prices are as follows:

1. ekoDB Cloud: It is a fully managed cloud database service product. The price for this product is $499.

2. ekoDB Pro: This is an enterprise edition product with advanced features. The price for this product is $299.

3. ekoDB: A high-performance database product with AI capabilities. The price for this product is $99.

=== Context Used (3 snippets) ===
  Snippet 1: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Fully managed cloud database service product id:fRcvIiN9GQdWlg4Nds2p8bfh4xnAxl5Yd_2NP5N6qnGiVxP_XMs6dbtX8PJ8dS1o5V9EqmOVOOpndG7BBys2wg name:ekoDB Cloud price:499] score:0.1111111111111111]
  Snippet 2: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Enterprise edition product with advanced features id:Yz-PUvMHTM7LNvqeCqhoUYzSglDOtPYTtP935tYuT1shI9tbElgJPeNnw6zgHSxYmrhB8YkrEj7sPZ_o1QV2fw name:ekoDB Pro price:299] score:0.1111111111111111]
  Snippet 3: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:A high-performance database product with AI capabilities id:SGjq3diLpx3bIGR0TDsTmVQsq2cr9FLhOxcw66pqtB4Fs7gKg_X1gWyCTKYm-WECjV3s-LbP4LatCL4EVlxEIg name:ekoDB price:99] score:0.1111111111111111]

Execution Time: 2256ms

=== Token Usage ===
Prompt tokens: 614
Completion tokens: 87
Total tokens: 701

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: CssK2C9NwDqB6gnxHjQmOEWnlPFpUh7XiAwg7Wvu-k6ZAZLAZe5IkReQusVCWlsU96NjKPvgu-n6XzcfI2kidw

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information available, the product that is available is ekoDB. It is described as a high-performance database product and is priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: -tnnQtOaa4IvPZEJpHUZR2YW9b8JT1MW43Gfl_uAWknkvRgu1FMJCepHu-XTfGxDMEqTyFrkTWq1OM6RTxxXTQ
  Parent: CssK2C9NwDqB6gnxHjQmOEWnlPFpUh7XiAwg7Wvu-k6ZAZLAZe5IkReQusVCWlsU96NjKPvgu-n6XzcfI2kidw

=== Listing Sessions ===
✓ Found 9 sessions
  Session 1: -tnnQtOaa4IvPZEJpHUZR2YW9b8JT1MW43Gfl_uAWknkvRgu1FMJCepHu-XTfGxDMEqTyFrkTWq1OM6RTxxXTQ (Untitled)
  Session 2: CssK2C9NwDqB6gnxHjQmOEWnlPFpUh7XiAwg7Wvu-k6ZAZLAZe5IkReQusVCWlsU96NjKPvgu-n6XzcfI2kidw (Untitled)
  Session 3: BFktfJOMJ7YljgfQGEQDsMgJIZ-0uhKET-2tVlgR9fOhkzOHWvpdYvlxe2DZcxoZd1fmY8aweNcjmyR3ZbNI-A (Untitled)
  Session 4: w3uwW8-60AB64JP2kQHBpGnrWGOXfXp289TQ8dj3GhYDE9QI6-wPz9Ctj-OMc9v_ttISNovA-FnkX8mqAqCyCA (Untitled)
  Session 5: fluLxb7w4wPDUOdSIOtQjVhdiR09WlaNCjyUoqemlxqDIa8uzcpRQBBHqtePk3jKPnZbmg_r0mkJp5Gqo6XIbw (Untitled)
  Session 6: JehJVzy0fGzjb5QdDQEKy5wozk9uYujxIkjq0V5YTjSRI0ad3j65tSRgHdY1djrSCKaYCyqxoZ0QuX97tozN6Q (Untitled)
  Session 7: FCexZoTUlOX6KTMmjH8vgzqs3v-JG_bq367FxD8kJTuYbPFS0SQ--AfP1uiO3tCjYL3IFjfW2XJv984i2JMbig (Untitled)
  Session 8: JI9ALeP9q7XIpzqiQkD8s_hsGsDcCR2eUhCkRH_5XNItOPrTyc0CeMrkI485fLp7sANlcLMTZeGqjhUlhuyHUw (Untitled)
  Session 9: HWO_Ih0OA303BqPhCgWr6vgXp3n0ydpNFOkFouLtHWO6i67yp0lhJOAGkd3BR00nWokmyL7toIpPhMwWY0WFsA (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: -tnnQtOaa4IvPZEJpHUZR2YW9b8JT1MW43Gfl_uAWknkvRgu1FMJCepHu-XTfGxDMEqTyFrkTWq1OM6RTxxXTQ

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: z7-WuIfx0TZWO_wVEJRYrpdLkAUwYQxau_EP1WTjtv_oP0ZLgYclsIc4klT8aYQoqwK26WQFBgPsixYseADITg

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information available, one product that is available is the ekoDB. This is a high-performance database product.

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
✓ Created second session: EoxFsNGvDGoht1Q_yKy8wRxk6vja0w6JhYWhzZrain-YcVDqpDEU6v2mJoT9pNfcSopyQbQVaAcUDJECztvOhQ
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
  1. Score: 0.769
  2. Score: 0.756
  3. Score: 0.747

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.702
  2. Score: 1.499
  3. Score: 0.308

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

✅ Script saved: bzAFy24MAWHkU7f68A5CI7bvK68cJ4Pz4a3OBTk1h5rIyVteMGDGeBytV1otf0AothI0WaAUrLf1u-ttF58yQg
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
