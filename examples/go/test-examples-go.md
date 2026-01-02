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
Inserted: map[id:RGTY1CPGDMQpNdzFWCGJE8YZZoZ8fQPaCuOjRioY6Rr4NgcLZWNOJT3gw3q3yEoum89o4_IS7nJ2LImzeSMFbA]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] id:RGTY1CPGDMQpNdzFWCGJE8YZZoZ8fQPaCuOjRioY6Rr4NgcLZWNOJT3gw3q3yEoum89o4_IS7nJ2LImzeSMFbA name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query ===
Found 1 documents

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] id:RGTY1CPGDMQpNdzFWCGJE8YZZoZ8fQPaCuOjRioY6Rr4NgcLZWNOJT3gw3q3yEoum89o4_IS7nJ2LImzeSMFbA name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
[32m✓ simple_crud.go completed successfully[0m
[34m
=== Running simple_websocket.go ===[0m
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: gWrryU63bObSCwCnzr33fn3Q6fCJReg6OxszABE3wk-lKvHLDa60hvTOaotwo3SQ05bclY4n5TN3yVv4ChpTdQ

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
      },
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "gWrryU63bObSCwCnzr33fn3Q6fCJReg6OxszABE3wk-lKvHLDa60hvTOaotwo3SQ05bclY4n5TN3yVv4ChpTdQ",
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
Batch insert response: map[failed:[] successful:[u-Sr39-WNB2nukhMNPi0LqvwedfdO9A6WFS3-5_ZgrzLiphW50dITyB4Fky2cfXNnR8YEHcVCaHeio_gTuxStQ zUnVU1RtxQB35tog0Nw-4AlhRiKC0CcHoR8pNd8leCSt46eMDNS5USo0CGvP95nE2-20nGnW9Q9CiLRDCWK4nQ 40xjUi9kcY8jx84A5u_Tt6A_pzNtu5zxYrJIploRawwO0AFUWOmsrUHPR2lhT95N3rnxaVY9wkM6X-DsjQoCTA DPniMCylgjF0yNtA3_4u25mBcDuRsEWmfsDy-wGnZ494TmBdvIKYQGVZE-j2cYvJBSEEnF87xZgD3FQm-x0fLQ ExmWqEZFHPMMEdaZutBSIw1oPGJFIumDpkaSv_G2pXnDyVg02Q4bQ6obE53CGwZJvJ3Z4cBxC9J6FytfTRBQRQ]]
✓ Batch insert completed

=== Creating test records for update/delete ===
Created 3 test records

=== Batch Update ===
Batch update response: map[failed:[] successful:[O4sGxtjrJtxiW_WmezOEng4gmX_apJzrnr1ryzJku1zUigmcl6fdXAM8uB6DdAJaBuXyGcOKM8YoVymS5o-HaA ygQcAH_DwqFcqWpx-knQz-3qtM7Z9NzFakuDpiqO-ojVMlbfzwhQTrqCn5WDxpbaXDKCXX45SXcrfNkdkhCiIQ qlZYzxkdPqwPDRwtadHCgOJPeHnVAMzJEYeV0PfwCJvjPjpB5j1Jzkpq1-ByZTMfN-nk_xL603cckS15XGIqIQ]]
✓ Batch update completed

=== Batch Delete ===
Batch delete response: map[failed:[] successful:[O4sGxtjrJtxiW_WmezOEng4gmX_apJzrnr1ryzJku1zUigmcl6fdXAM8uB6DdAJaBuXyGcOKM8YoVymS5o-HaA ygQcAH_DwqFcqWpx-knQz-3qtM7Z9NzFakuDpiqO-ojVMlbfzwhQTrqCn5WDxpbaXDKCXX45SXcrfNkdkhCiIQ qlZYzxkdPqwPDRwtadHCgOJPeHnVAMzJEYeV0PfwCJvjPjpB5j1Jzkpq1-ByZTMfN-nk_xL603cckS15XGIqIQ]]
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
Collection created with first record: ZcdlKJ6b9_NwpruNbCPKGU-o1T8h5vlLWXwVOs4yJ3OY1beaaoV5QH4EvuhR390LNmwxTHYyRPUs8Cw6ZT0LsA

=== List Collections ===
Total collections: 15
Sample collections: [batch_users scripts__ek0_testing demo_collection schema_employees_client_py schema_documents_client_py chat_configurations__ek0_testing schema_products_client_py schema_users_client_py users ttl_cache ws_ttl_test chat_messages__ek0_testing websocket_test test_accounts test_collection]

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
✓ Inserted document: e9x8Kp7juITe6xCglOpOCFyBy6CxQoATETAIsA9Lq5HAQYLHqr4GZ5nN1zplghmkDOAy-4-h7Tgs16z7d3J-uw

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
✓ Inserted document with TTL: ThDoMvHTShrUVhMgGNANAmCZzspUxXL6WchGNG4cumRWvTWalli4fwVSMViTpTsCHL_76tCJwnYh_qVSNfZ1iA

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

✅ Script saved: aJyJd87pSWc7zFf5IZkfLUx6Amtk1t5wsHye0Kzzid5gI39o577bqs6z97XXR8_Yp_VCnP-IoB6ZQ4CeUqfy1g
📊 Found 10 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: 9bToJdScxOmMAPRGH46ml8MY_lkQqpn9WSQ9nNxuu5P5UaDUbxgKqIWhW_pxx3OkmlTdx3ecS7Pw_Q01admKSA
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: cq-e8Jpt5kehub29CF8bRZ857yiMnsq00xTbarJV8kiQ6Sn0SPCnYjCSxZQ2Qzk3xWsdYuqnqIOocIt2PkcKrQ
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
Created Alice: $1000 - ID: Jgo3NsO1kFPd-EFznf7mKTmOs7q9DBfK8g7uTV6BLDbB_u5iIQkj_MWT2znAktvgx1ydpBj16rXgE6LunCDDiw
Created Bob: $500 - ID: kdw-gTDAUnuJLpr4l-S2lLPDuwQ0MiRRAR9k-mCrOgxF9z6VgnlnOMj5wxcPV54YixkjMO0-pzsKHX0dq8XuXg

=== Example 1: Begin Transaction ===
Transaction ID: e8863f57-f3e6-425d-b9c7-c0bb8dda72d3

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
New transaction: 3f7572db-6866-491f-9f2e-af211ad27e3f
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
   ✅ Script saved: firwflfUEY2vRAFEkZd86rxrO6_jn3VRd6Uv48B-VWtbGBexcnMfSc-0ekpCL2NKLs3RM8uAlHT9Kn5_YHgGUA

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: UDN6GmOtwRT0yjrMUH0dbuQ-yhRE7ocVcU0Bim5b3xR9BeITAYm-wactRr6-WOTqNlRCoOsR0JUoUkxR6YZ6Tg
   📋 Name: map[type:String value:Alice Smith]
   📋 Email: map[type:String value:alice@example.com]
   📋 Status: map[type:String value:pending]
   📋 Credits: map[type:Integer value:0]

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: Z6csPEP3BnfzsN8DxiLT3t-ouDee7MXETsmX_qCF-gzL2l8clrv0j9KVYOWQzFO0WvyaTG5yVlj1EDcdleBxPA

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
   ✅ Script saved: ywjq4sqy0T6jN0pKOCAcjQhppBWBSg-1v0OuBeaJ6hZ8I91NpI-uidX8ykJ_QI5aRpm5EVjLdIUoQJtIUlc16g

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
   ✅ Script saved: Y7avdODyuyZR1-4RjQuic8bzHgMT14nHk_E0wy8MNfHV6ws9inhtu1x-mHDFn9gNhjp1N2dAeR6R8icAD_B0ag

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: firwflfUEY2vRAFEkZd8...
   ✅ Deleted script: Z6csPEP3BnfzsN8DxiLT...
   ✅ Deleted script: ywjq4sqy0T6jN0pKOCAc...
   ✅ Deleted script: Y7avdODyuyZR1-4RjQui...
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
Inserted: map[id:pViUzwf-VJ2q2SdZvzrfH3QXqk4gA-OK7MBG58JqqYrEHG9owQT_cc51h2tn-9tTDIFC76egDsNacvOBkopaiQ]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-01-02T05:54:49Z] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:pViUzwf-VJ2q2SdZvzrfH3QXqk4gA-OK7MBG58JqqYrEHG9owQT_cc51h2tn-9tTDIFC76egDsNacvOBkopaiQ metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Test Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:42]]

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 0
  active (Boolean): true
  price (Decimal): 99.990000
  created_at (DateTime): 2026-01-02 05:54:49 +0000 UTC
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1 tag2 tag3]
  metadata (Object): map[key:value nested:map[deep:true]]
  embedding (Vector): [0.1 0.2 0.3 0.4 0.5]
  categories (Set): [electronics computers]
  data (Bytes): 11 bytes
Plain record: map[active:true categories:[electronics computers] created_at:2026-01-02T05:54:49Z data:aGVsbG8gd29ybGQ= embedding:[0.1 0.2 0.3 0.4 0.5] id:pViUzwf-VJ2q2SdZvzrfH3QXqk4gA-OK7MBG58JqqYrEHG9owQT_cc51h2tn-9tTDIFC76egDsNacvOBkopaiQ metadata:map[key:value nested:map[deep:true]] name:Test Record price:99.99 tags:[tag1 tag2 tag3] user_id:550e8400-e29b-41d4-a716-446655440000 value:42]

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-01-02T05:54:49Z] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:pViUzwf-VJ2q2SdZvzrfH3QXqk4gA-OK7MBG58JqqYrEHG9owQT_cc51h2tn-9tTDIFC76egDsNacvOBkopaiQ metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Updated Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:100]]

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
Collection created with first record: ksJb-n3Qa5gqMog6OJxLePvEWZcjEo0olvUmlx0rSG0kjwg7dBOjmyjIug_Owgk6ftsrnOl4R28XwtI1yePq8A

=== List Collections ===
Total collections: 14
Sample collections: [batch_users scripts__ek0_testing schema_employees_client_py schema_documents_client_py client_collection_management_go]

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: QFPpA394fYoTJpK_m_AhLOuMS5ala-dsCwC6zr7R7ZCNJ39SvH2jFWHrNhrDlVW-CzUjnH2skUBn4sfRj1iynQ

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: 5TmSyH0HTiFmcPj_mxi-SrpjlTKBAdEwtbEswydKaJ-0vJIn7P9dLaMSijb40tk3kNw85A9yA7nqN8Ic6S03ZA

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
✓ Inserted test record: 6Rrm5lbv2-Nwv78VRQljeodWwOsMuS2aJw-73vaA_SUZY8W8sYqIyFPZPM0UZmzIMxD71o3sP1yVZYG7FdBkMg

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
✓ Inserted document with TTL: tgeEzwHa4NWbuuu1l7hADg8qo6tSHMt9sEVtzWk3Ek3KnQVa-zkzd8KPkL7t-A-SU0G1ChkCtKIsRsV3rIOXhQ

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
✓ Created session: v75EUAjj7PJMmulZcK-nv-L3uWabEVFw9NTNXlqSRC0zjIdx0xwdJNTBP_a22jrfVAYdObbxOvjsjnk2h_JUtQ

=== Sending Chat Message ===
Message ID: SDxY1suIussrVJ0y7plS38_HH5xMctgFO9thuCAqjLbfeT76LNnMGdpGORcA7IiZetaOFjHNc3Nimo291dzw3Q

=== AI Response ===
Based on the provided information, the available products and their prices are:

1. ekoDB Pro: This is an enterprise edition product with advanced features. Its price is $299.
2. ekoDB: This is a high-performance database product with AI capabilities. Its price is $99.
3. ekoDB Cloud: This is a fully managed cloud database service product. Its price is $499.

=== Context Used (3 snippets) ===
  Snippet 1: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Enterprise edition product with advanced features id:59i4OfANOcXq7rerOONJCV0zqKijfZFp5Oaw6R1VwRDf9xjm6844msitlJ_juNrkh7ZZI93fuR7vJy317L6RIw name:ekoDB Pro price:299] score:0.1111111111111111]
  Snippet 2: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:A high-performance database product with AI capabilities id:_syMU4EkyCAntfnaE245-k1IJL6Fv3BJ2ofOAraxJhI-0ZSNlV8dEnDg39OsTmd0RCbPZdKaWLrD7poFyRzw5Q name:ekoDB price:99] score:0.1111111111111111]
  Snippet 3: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Fully managed cloud database service product id:VRkGxsTO9KaBqC6Jw8eau49d4tLHY0P_TGg0E4uDZI3lksTeinbmP1nzLaTzHcWdt-ZyncnI3_IB-iWt0-wluw name:ekoDB Cloud price:499] score:0.1111111111111111]

Execution Time: 2656ms

=== Token Usage ===
Prompt tokens: 606
Completion tokens: 83
Total tokens: 689

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 9ooPTyK8bGLT90NGU7Lv89FzCXtNlq6M2ntR2f6tGRxIkRkbzSEKeEaLdt8xegkxdkttJFZPUiGBksyDItd1lw

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the context provided, the available product is "ekoDB". It is a high-performance database product and is priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: u4rW74t-I63-ibvTQrUuphNzR28bck5ruuGjKf58rIZ_k8UPkDLqWt6ydnYzEQIkWB0pD53uE3y4MzDj6C7-NQ
  Parent: 9ooPTyK8bGLT90NGU7Lv89FzCXtNlq6M2ntR2f6tGRxIkRkbzSEKeEaLdt8xegkxdkttJFZPUiGBksyDItd1lw

=== Listing Sessions ===
✓ Found 9 sessions
  Session 1: u4rW74t-I63-ibvTQrUuphNzR28bck5ruuGjKf58rIZ_k8UPkDLqWt6ydnYzEQIkWB0pD53uE3y4MzDj6C7-NQ (Untitled)
  Session 2: 9ooPTyK8bGLT90NGU7Lv89FzCXtNlq6M2ntR2f6tGRxIkRkbzSEKeEaLdt8xegkxdkttJFZPUiGBksyDItd1lw (Untitled)
  Session 3: v75EUAjj7PJMmulZcK-nv-L3uWabEVFw9NTNXlqSRC0zjIdx0xwdJNTBP_a22jrfVAYdObbxOvjsjnk2h_JUtQ (Untitled)
  Session 4: 8ij4JREUz3EKuqBCksCRA7svRIYYv-D5XIJB5gTr2mYoHY___fq_MEDed0_QHBNvUjkHoahbTBSyMnR4_TWC8Q (Untitled)
  Session 5: kGSJYCEbzW3WYaCWW7qVi3faqJ3rd7v582U6huy7pCWYY8f6ObSU-0M1hvqtXTDMoeMrmCByIiW54wUOC3a90A (Untitled)
  Session 6: DszLGnWdRc3qVTd1hrSCP54uqUp7i7BhSY2t8yWH1gNfoggn4Hap_8D6QTrKr9D_wYIYcLDRI8xQtzsAUY7HiQ (Untitled)
  Session 7: m0J43kGqILKtUrl5O0HN4ccU2Y0S8UshpcPMxHmfvLO8rxHA6k6TbxC3u0LD39-JDoyHbM80XLYomD3kVphUqQ (Untitled)
  Session 8: Os4Ty52VQWOwXHxmpTvxMOLnCwAgd-aT5rjepSBeASzohnp0YdUUksWfNNiVq5OzIN-7q6T_MfIkl7GZGmwH4Q (Untitled)
  Session 9: pHzBl65XmiObokt99kagXWBR2XZATZkaDRbAQw9DV4VRVNekmdIShmIRACSrvbykFI1FO3S9t0o13S8KcuTkgw (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: u4rW74t-I63-ibvTQrUuphNzR28bck5ruuGjKf58rIZ_k8UPkDLqWt6ydnYzEQIkWB0pD53uE3y4MzDj6C7-NQ

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: ar41bGOuzOD6mTcTiGu5wm4UT7WwJTYvMPOe4pLHdvdf74I2knOSU4lvpg50h8cVQ_QTHDx2wuM18jmh7YknrA

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information provided, the product available is "ekoDB". It is a high-performance database product.

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
✓ Created second session: 1lwWsuF-SpuDCtoWk_T-U8D5vc25tw6GmsM1uUb7QWJq4olAo9gNU9okLB8-xHp3ROrd89v3xm9edkETQcAvDg
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
  1. Score: 0.759
  2. Score: 0.746
  3. Score: 0.729

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.692
  2. Score: 1.499
  3. Score: 0.304

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
  - Alice Johnson: Engineering
  - Bob Smith: Sales

2. Join with filtering:
Found 1 users in Engineering
  - Alice Johnson: Building A

3. Join with user profiles:
Found 2 users with profile data
  - Alice Johnson: Senior Software Engineer
  - Bob Smith: Sales Manager

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
🚀 ekoDB Scripts Example (Go Client)

✅ Client initialized

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: Jn-6yaHYsybK6wVuZ7eHHX-uOKE2BElduj9-wkHNi4qYRRYZZkkRUd3GesQo01ioYNaXMHJVo3XMLODwe074zA
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
