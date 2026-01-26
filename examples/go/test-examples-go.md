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
Inserted: map[id:yR9ZkvT6PUfpYsr9vx_LW7BTgFSyY2sgq0wyu3I3BMsodKUILm5lu37kCqOtu9tZqOzGlJKLt1THzrvbU3I4Ew]

=== Find by ID ===
Found: map[active:true id:yR9ZkvT6PUfpYsr9vx_LW7BTgFSyY2sgq0wyu3I3BMsodKUILm5lu37kCqOtu9tZqOzGlJKLt1THzrvbU3I4Ew name:Test Record value:42]

=== Find with Query ===
Found documents: map[]

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] id:yR9ZkvT6PUfpYsr9vx_LW7BTgFSyY2sgq0wyu3I3BMsodKUILm5lu37kCqOtu9tZqOzGlJKLt1THzrvbU3I4Ew name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
[32m✓ simple_crud.go completed successfully[0m
[34m
=== Running simple_websocket.go ===[0m
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: knUwKHS37sl1ZADzeM0K1tHhnVSxb_dvX1jxCdqgH1RtlgbiRpM8eMHmFU0S6jDKcKI2sjV1yBdCldhrb4N_Lg

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
        "id": "2KDKITf1SV3IdCNu3FWwWGoEg8fhkqMMeP7tox31rDf-3OcKECI-4LPKyr_GZjmDnd6O5hUtN5zh6TfLfBvPiQ",
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
        "id": "9qANfwDEYn4jKGx25UL0lEibKplJLTThUsJZjNTuw-Mndg3CY8pV24Wc-19KGmdxxFpyeTosNU8sJe5uomjbCQ",
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
        "id": "knUwKHS37sl1ZADzeM0K1tHhnVSxb_dvX1jxCdqgH1RtlgbiRpM8eMHmFU0S6jDKcKI2sjV1yBdCldhrb4N_Lg",
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

✅ Script saved: 9LMFVKE3WMu6UK4_UKqAN5ApcqLOaMh9EbCF6S-orQVqC4N02FNxSv-bGBO-5ZP9jfInGG8yXwSXkCATcrDySg
📊 Found 10 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: mdfOyxBw-V9uVUe9NrvoY-ex383YH6iDm6RdTyKQV-DAWucg8fy2_ihQZPRSJBGtitFxAcmo8rUTBH56I_hH2A
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: IZrrLSIxru3IxYVP-G0Itf5o8g4l53qLpNDFPpT5gXLfZaqWgchJII_3WOno3orHrhfCwOXcxGErwyA9kRe8Lw
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"avg_score":60,"count":10,"max_score":100,"status":"active"}
   {"avg_score":50,"count":10,"max_score":90,"status":"inactive"}

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
Collection created with first record: re364h0Qx9UKIjksNWn_7v5oU1VWRMNJrPd1fTFD6gAcCQ8Ly9uxB6D61mPUYKLFRnfDAybd5EuqHpgILv3gwQ

=== List Collections ===
Total collections: 15
Sample collections: [demo_collection websocket_test batch_users scripts__ek0_testing test_accounts]

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
Created Alice: $1000 - ID: l5-HO3DNJc3Ha1NYGdHDd0Qu2LNYmZ0ZVtjc4fRr609wodlmHYMYd3eOukRc5D0XL7oaDcmzMy18mAvizd4flA
Created Bob: $500 - ID: MDS9-QkdfHcNa5ZSIbtgoJmvvKhiuqwGnk4lIJhFteD7ToCeYy0gF03awKVAM1zuE1WyL7o_BmMlaOcA8AIG7w

=== Example 1: Begin Transaction ===
Transaction ID: 0bfbf068-831c-4e54-b5f7-738b5a59f4b5

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
New transaction: a5817f43-aa55-49da-b99c-24fa60315a9f
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
   ✅ Script saved: 8PAI0zNpkd7ftoVq_0H2X070xwdiWfGgYoDua7YtG6yuxDXX6sCCCcgCvq1FlkM_6CRgJ7jHK-avI97gULErLA

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: LGJ2NjQI0-Z7Bqwn292qTcWU5XcG1WdPqhLh5ssPYHRx2x65CUcY4xp4Ep6wGdYk5bc6BV-sdkzixpPibyVLFQ
   📋 Name: map[type:String value:Alice Smith]
   📋 Email: map[type:String value:alice@example.com]
   📋 Status: map[type:String value:pending]
   📋 Credits: map[type:Integer value:0]

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: qYq34Cnnp2cWhz6bkfrPs3U1izP4bUKO_Xyq90UNmLw9PDAWZuaIPLiZ47DvHHtIrHFtjPHm_hLSWCUs1VqjJQ

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
   ✅ Script saved: Gy-QMEwyX3bclIxZci-f2s0EQMaH4VyC6eAOQYLeWMGkHr2UfJ-vWaOUtGvxgbmn-yMvhRK98imOzRm46CWBhg

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
   ✅ Script saved: Qi0xP5N5ZM8nAdZPLGzJNlVMeXeF5yGtrdWf7mdHWpDMXf6nqxPEk96pPieXCWB2ZaipDFOZJWTMa2UhDrxvZg

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: 8PAI0zNpkd7ftoVq_0H2...
   ✅ Deleted script: qYq34Cnnp2cWhz6bkfrP...
   ✅ Deleted script: Gy-QMEwyX3bclIxZci-f...
   ✅ Deleted script: Qi0xP5N5ZM8nAdZPLGzJ...
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

2026/01/26 15:20:00 No .env file found, using defaults
✓ Client connected

═══════════════════════════════════════════════════════════
TEST 1: Document TTL Expiration
═══════════════════════════════════════════════════════════

[Step 1] Insert document with 3 second TTL
  Input: {name: 'TTL Test', value: 'should expire'}
  TTL: 3s
  Output: Document ID = 2kf9p2uq7XsO_nRUcZOiBlnm2lPH0EDntqCSP1dcWihPPwVKJzSl3BwdSd5vqFKGDkKtYcpjeUp6uEGr-SsYgg
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: FindByID(2kf9p2uq7XsO_nRUcZOiBlnm2lPH0EDntqCSP1dcWihPPwVKJzSl3BwdSd5vqFKGDkKtYcpjeUp6uEGr-SsYgg)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: FindByID(2kf9p2uq7XsO_nRUcZOiBlnm2lPH0EDntqCSP1dcWihPPwVKJzSl3BwdSd5vqFKGDkKtYcpjeUp6uEGr-SsYgg)
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

2026/01/26 15:20:07 No .env file found, using defaults
✓ Client connected

═══════════════════════════════════════════════════════════
TEST: WebSocket TTL Expiration
═══════════════════════════════════════════════════════════

[Step 1] Insert document with 3 second TTL
  Input: {name: 'WS TTL Test', value: 'should expire'}
  TTL: 3s
  Output: Document ID = fdwkzt3RGlefX_QugFJHw4ldsAASDiy9i5-YopUGsjxc-P9GqkwVovhdfa20D3PB365Wd4eC3H_drF8a6sHZaQ
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: FindByID(fdwkzt3RGlefX_QugFJHw4ldsAASDiy9i5-YopUGsjxc-P9GqkwVovhdfa20D3PB365Wd4eC3H_drF8a6sHZaQ)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: FindByID(fdwkzt3RGlefX_QugFJHw4ldsAASDiy9i5-YopUGsjxc-P9GqkwVovhdfa20D3PB365Wd4eC3H_drF8a6sHZaQ)
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
Inserted: map[id:1oqkL_LOlfK7VdeM9hZhDMDfkQAk5Q51Nn5Gchju2yR8sbJky0nf-T_reZfaPlxk6oyL1VYVIPs2fnf1fyM3bw]

=== Find by ID ===
Found: map[active:true categories:[electronics computers] created_at:2026-01-26T15:20:14-05:00 data:aGVsbG8gd29ybGQ= embedding:[0.1 0.2 0.3 0.4 0.5] id:1oqkL_LOlfK7VdeM9hZhDMDfkQAk5Q51Nn5Gchju2yR8sbJky0nf-T_reZfaPlxk6oyL1VYVIPs2fnf1fyM3bw metadata:map[key:value nested:map[deep:true]] name:Test Record price:99.99 tags:[tag1 tag2 tag3] user_id:550e8400-e29b-41d4-a716-446655440000 value:42]

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 0
  active (Boolean): true
  price (Decimal): 99.990000
  created_at (DateTime): 2026-01-26 15:20:14 -0500 EST
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1 tag2 tag3]
  metadata (Object): map[key:value nested:map[deep:true]]
  embedding (Vector): [0.1 0.2 0.3 0.4 0.5]
  categories (Set): [electronics computers]
  data (Bytes): 11 bytes
Plain record: map[active:true categories:[electronics computers] created_at:2026-01-26T15:20:14-05:00 data:aGVsbG8gd29ybGQ= embedding:[0.1 0.2 0.3 0.4 0.5] id:1oqkL_LOlfK7VdeM9hZhDMDfkQAk5Q51Nn5Gchju2yR8sbJky0nf-T_reZfaPlxk6oyL1VYVIPs2fnf1fyM3bw metadata:map[key:value nested:map[deep:true]] name:Test Record price:99.99 tags:[tag1 tag2 tag3] user_id:550e8400-e29b-41d4-a716-446655440000 value:42]

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-01-26T20:20:14Z] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:1oqkL_LOlfK7VdeM9hZhDMDfkQAk5Q51Nn5Gchju2yR8sbJky0nf-T_reZfaPlxk6oyL1VYVIPs2fnf1fyM3bw metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Updated Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: 31Aeo2uPtGZuB759aOqBz6j-jsonaS_vvjyy36vKi_uRWpfZkSDKU8wQyxmqRvudcQYCI6CiQ3eizOGKXKYG0Q

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
Collection created with first record: Yd8JturoKV6km7pL5xJi6HYPOd_6GpMo3KaikixLnig4Rwa6hr3MnoiE7Z_Y0BYQADcBG0sJgyxmwcnOAIfpIg

=== List Collections ===
Total collections: 14
Sample collections: [websocket_test batch_users scripts__ek0_testing test_accounts schema_products_client_py]

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
Created Alice: $1000 - ID: e6Xo61ml6S0JC8cRBARWCmRtu9BPtDM9ggfxeAYNeDzR2R18-hWG8A_hX-xN6sde6VVUZ6mYe06KrCB3pZGNJQ
Created Bob: $500 - ID: -Yezdm-nBbJUFTouTNtDTOphQK6OGEQGmCieWOvAvPAPjIZFtYVTbxnwMC7w63mqMNQr8JruXR6FSuBMrcOCDw

=== Example 1: Begin Transaction ===
Transaction ID: cad413ba-6a80-4d67-a7f4-bc3592947af7

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: d5f88e96-a6dd-4efe-9207-6b09b52407ae
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
  1. Score: 0.747
  2. Score: 0.742
  3. Score: 0.724

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.690
  2. Score: 1.497
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
✓ Inserted document: 3iDCTtAv00hMAUahaotfRHp3zP6UHoG4A2jVz18-LI6yrvzMD7_8m6J3-6MlfDsmfBT5WZkFknDQk0CJ7NJCXw

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: V5cqcErJrGVGokjS5OdAKYi1Jzss2fVqRHLximHULJwajP9Q4BPVOB7u8ApskBFiV9EXStwvPiopgOhRKpzqBg

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
✓ Inserted document with TTL: q6fwKLq6ZQDV_mScbB9eYV3baNKQ_wsYy9Yzf5g4eUiN-7Ur0K6M07lhQsMG2554pLTfm0yao3Dtt8TsDrVBKw

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
✓ Edge cache script created: nfLr8Fc4N3at9qj19XsU5tnHZVSxLY6dDlu1FqUp21rjJXt9yuFtWkVj_sXcAU0TDVncPkHb_H_RktEfdab4lw

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

✅ Script saved: p6JXBDTFpeZaQzLH9Iker22jyKkSFKTPkk_AIZYlWh8J5O040dHMTVLs18LNAMce_1ImjsyqqBY9Hg5GEy7N2g
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
   ⏱️  Duration: 2.204208ms
   📊 Records: 0

Second call (cache hit - from cache):
   ⏱️  Duration: 2.250417ms
   📊 Records: 0
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

✅ Script saved: 3-O80KSOAhx8s-sXB3cWWiG1Gcb9Rn2e4ch1_6hv2lSr8xFGVe27u99PRAfdnkm5TEdO_9UIojuLS8aicBrfrQ
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

✅ Inserted order: A8YJrOgjKcRIGrUv4GQEJUxRCP6Sem3_21Unj53PFAdhx6Qxa0qTo2wWmqvHJwtjKD4Ir-0Myf-ituhc2KaWqQ
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: KLiUpdaRS2tBCRSJFwvIdlKgMtD9aBxJ6vLHsNdhpLTs-x7MdBPZYBs3TeUhqmFqeOgpz6EUbWLO4R-gBQlY6w
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: map[role:admin userId:user_abc]
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session
📝 Example 4: KV Operations in Scripts

✅ Script saved: 2bXT9fn2WwSW1SNd8L7ifcpJlg3vxury2A0pR9oZYS9Q2HrwedAUeVKLjJv1AFX1Npd7mRYUOCF2aKY0X4eyZw
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Script

✅ Script saved: Gem5qeuuFgsoJnK0jkQ6b89Cb9NMgvSnDAOGmEV9cRaI4ewujyZ4jJW29p_uvyeVov3_k9W17lF928J-5-zXsQ
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
✓ Created SWR script: fetch_api_user_go (VciMPOsrLw6WIwfjyvh2bQjtKxo8ECqQpkZkd3pYzXXbMgLgY-k8JfEZK0wrw93vLzkX3yquMxbHsSsmBmJq9g)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "cached_at": "2026-01-26T15:20:17-05:00",
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
      "id": "Kt0RFK_cFG7lY_Bi_hW8JE3PedtIgkwGPJizZiEYsT0pgxlq3KyzLds9nan0xuKV6OeBTk9PaP8QF5GgP6apRg"
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
   1. map[type:String value:Natural Language Processing] (map[type:String value:AI])
   2. map[type:String value:Getting Started with ekoDB] (map[type:String value:Database])
   3. map[type:String value:Introduction to Machine Learning] (map[type:String value:AI])
   4. map[type:String value:Database Design Principles] (map[type:String value:Database])
   5. map[type:String value:Vector Databases Explained] (map[type:String value:Database])
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
✓ Created session: ri8J9aosDC9gMwutTRft89s9bxluPtJ7ZTaTMX8IsGsL0M2ATyINvc74OjgMfIqVXTnnKGSKkbdYewbyGG1Nqg

=== Sending Chat Message ===
Message ID: wue7TeHSHokOkIBScmF2jZxCVuxfdaGi80Uc3m647BK6Y0pH4C8FVbei6B2Y4H61jreUc6dX_sCQXzg1esbfhg

=== AI Response ===
Based on the information provided, the available products and their prices are as follows:

1. ekoDB Cloud: It is a fully managed cloud database service product. The price for this product is $499.

2. ekoDB: This is a high-performance database product with AI capabilities. The price for this product is $99.

3. ekoDB Pro: This is an enterprise edition product with advanced features. The price for this product is $299.

=== Context Used (3 snippets) ===
  Snippet 1: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Fully managed cloud database service product id:t03hVwReRZ2K_NMCtJqvnDmO4M5u_8k0wu-MgFR8ZpDBML9feciEDnO6IyaxEkthILMPGmtXLJ2FwPy4ezQNDw name:ekoDB Cloud price:499] score:0.1111111111111111]
  Snippet 2: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:A high-performance database product with AI capabilities id:ODO9UHlhTTcGAjlZtcmmwg1RjZ2uF5X4E87RTs9Q3A8_XASN2Ep9rU05jTVvAo9atE4sqyD5gciGVuyCUBeoHg name:ekoDB price:99] score:0.1111111111111111]
  Snippet 3: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Enterprise edition product with advanced features id:gqVnn1LID78s_SvJrneSTb9MngYzfxNP4BmjGYu7zDmAzsTrd0cQ6u6MyQWk_Y6iQU-hHO8y_E7-LqUrUVuoiw name:ekoDB Pro price:299] score:0.1111111111111111]

Execution Time: 4779ms

=== Token Usage ===
Prompt tokens: 453
Completion tokens: 94
Total tokens: 547

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 5zi6i6xU5qmRA2wjAtl_hHf3Fr6oNNRsjN8gCPjk3w4DIZJI7MU02J5bsLgaVnxOQ4uGB_s7b2GL4Bg2lkdsXw

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information available, one product that is available is the ekoDB. It is a high-performance database product. The price for this product is $99.

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
✓ Created second session: 1S0ufLnEeEi2Yjuhd_ouj8oaoVh694rRPUWARagAFJABjlnRGDZR32ZnP9mdoPKNgiP8Eg-tgmRmUwEGcdZ6gw
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
✓ Created session: 7qELxPg2lrcRGy2spbCbBuOsp81Od503t0WkQ9fDmaf6UvOG_CMVqnir7wTo_-hhK4ukmBoUp94dva071u49FQ

=== Sending Messages ===
✓ Message 1 sent
  Response: The product available is "ekoDB", which is a high-performance database product. The price for this product is $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: c5ygJQnA0wemCGHKb195ghBvdvW64JPg8tpP5yf61d16A30LVweXjt1iBxtf7wd9aBDiHnKzc1PBbCbJ1hQcww
  Parent: 7qELxPg2lrcRGy2spbCbBuOsp81Od503t0WkQ9fDmaf6UvOG_CMVqnir7wTo_-hhK4ukmBoUp94dva071u49FQ

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: c5ygJQnA0wemCGHKb195ghBvdvW64JPg8tpP5yf61d16A30LVweXjt1iBxtf7wd9aBDiHnKzc1PBbCbJ1hQcww (Untitled)
  Session 2: 7qELxPg2lrcRGy2spbCbBuOsp81Od503t0WkQ9fDmaf6UvOG_CMVqnir7wTo_-hhK4ukmBoUp94dva071u49FQ (Untitled)
  Session 3: 1S0ufLnEeEi2Yjuhd_ouj8oaoVh694rRPUWARagAFJABjlnRGDZR32ZnP9mdoPKNgiP8Eg-tgmRmUwEGcdZ6gw (Untitled)
  Session 4: ri8J9aosDC9gMwutTRft89s9bxluPtJ7ZTaTMX8IsGsL0M2ATyINvc74OjgMfIqVXTnnKGSKkbdYewbyGG1Nqg (Untitled)
  Session 5: K4Zl3YC8nD5iX6ndffmischiOraG9LEaYA26OQQC0MoUl9OpC9dq8LjD56bzWFfOvTHSry7KQ7fIVXNuqRK-Ng (Untitled)
  Session 6: IU5sdisPG6DUPq-ArVY1FAD_5uVNxM8FNtb3Mt3yyAzGdsVe3Ob_TyahjMtCqe5FF_gblxFglE6d7dzXhye3qw (Untitled)
  Session 7: 1QxWJqEU5iXvj8VlydtoGuU4bMr3VU6FwXWamrjsnxkrO5EcgEEzdzqt0TIbdGXppbLVvhddduxa-EooZD6DwA (Untitled)
  Session 8: B34WCoxf8nH3TbbVuxeVT7jxixrqEQikzOAkSa98N7JrmbrCvCu6OW1i12WHAEuGg8KNoC7IvEpOHjea5Xgvuw (Untitled)
  Session 9: BwYFP0uVQGNlc6VCBff2PHtOXg5E3CgP3FUZ8bM1h3h0KsAH26riq9qTJpUkDs-zK5YmRyXuizzVil7impBBOA (Untitled)
  Session 10: haPvVPHhURHG2gR4H89asUbBWgrhgXueRh4xFZptY19J391BZb892uDfR09WI-AiyvEjF8XAXJ-0axNcGlFJew (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: c5ygJQnA0wemCGHKb195ghBvdvW64JPg8tpP5yf61d16A30LVweXjt1iBxtf7wd9aBDiHnKzc1PBbCbJ1hQcww

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Map Creation ===
✓ Created record with native map: map[id:5kWXm3ifLfe9W7Ye0LXqhY3X0S_amo9y77I2I4SlBb0sAzHv9g0a7kIHzSTzsaCI5kEfl9weHw2o95Ffz5GMVQ]

=== Upsert Operation ===
✓ First upsert (update): map[active:map[type:Boolean value:true] age:map[type:Integer value:29] email:map[type:String value:alice.j@newdomain.com] id:5kWXm3ifLfe9W7Ye0LXqhY3X0S_amo9y77I2I4SlBb0sAzHv9g0a7kIHzSTzsaCI5kEfl9weHw2o95Ffz5GMVQ name:map[type:String value:Alice Johnson]]
✓ Second upsert (insert): map[id:W8R1rshfETvspz3Wk-phi28JoBsdJF9lv7wtNviRiyj2OQ3GqIYQ0NhOnQsx92wonueiVAK3DN6hXObsN6muqg]

=== Find One Operation ===
✓ Found user by email: map[active:map[type:Boolean value:true] age:map[type:Integer value:29] email:map[type:String value:alice.j@newdomain.com] id:5kWXm3ifLfe9W7Ye0LXqhY3X0S_amo9y77I2I4SlBb0sAzHv9g0a7kIHzSTzsaCI5kEfl9weHw2o95Ffz5GMVQ name:map[type:String value:Alice Johnson]]
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
Inserted with ripple: map[id:2TkO_NrmwTf3xQGAuHpQEAKS7rW9eXCMz82_3Qk19kd3XRZK8RFdhGkd9zSXxoWS8JVgc-uS4F5kSMdEDYX1_Q]
Inserted with bypass_ripple: map[id:UyywFSTJONbYP6ekW8IMhypxL2f-VkljqpUKdiildJbHISqdwbsl0c4JGHOt5v8Lx4C9-FtJnLbHuGsX0zU9mA]
Inserted with TTL and bypass_ripple: map[id:NUdWIaNx7IBLYuJlYQsh16n7iTQ6OA7bcVumYCzuWI3f-IkmyZ2PU0c156P8D_WpXFosQk9lSEBfGV6wJecB5A]
Updated with bypass_ripple: map[id:2TkO_NrmwTf3xQGAuHpQEAKS7rW9eXCMz82_3Qk19kd3XRZK8RFdhGkd9zSXxoWS8JVgc-uS4F5kSMdEDYX1_Q name:map[type:String value:Product 1] price:map[type:Integer value:150]]
Deleted with bypass_ripple
Batch inserted with bypass_ripple: 2 records
Upserted with bypass_ripple: map[id:ViB7wFqkkoy4K6F7y8wdiF3jaI50enyuNixsp5fS2zG9uXL3HSnNI0AlyHG59UoW6ESRSaK_N56eaFHNIKioVw]
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
