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
Inserted: map[id:7jYFzmWXcPYg6GAA3J1qv4SGBo8aBvD6ABcvLYkpfvupxiVumU2UCaQv0ajgk2SUSWoyTN2NvcjZlkvuO_oSng]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] id:7jYFzmWXcPYg6GAA3J1qv4SGBo8aBvD6ABcvLYkpfvupxiVumU2UCaQv0ajgk2SUSWoyTN2NvcjZlkvuO_oSng name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query ===
Found 1 documents

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] id:7jYFzmWXcPYg6GAA3J1qv4SGBo8aBvD6ABcvLYkpfvupxiVumU2UCaQv0ajgk2SUSWoyTN2NvcjZlkvuO_oSng name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
[32m✓ simple_crud.go completed successfully[0m
[34m
=== Running simple_websocket.go ===[0m
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: dyWw3VvpsnEw96E2DvwrHXilOtsUDuOr5iAHODDmIjHmH7ZfyBwHrWaOSScoIzsIIhzhGnYlitDeQRp4peAJJA

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
        "id": "ZZP8j_9ycdafEWgY6Gv4o5YVw02nFkt05huGnFkyCMirV19KgWpN2J4AepMNXW6mBJu2DEx-dtNYUnc9a7rKIw",
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
        "id": "nnVOAK1UCMqtLlx1RjzLLmk48bJGDlOuJzKQHSasOE0HvHVCpuW8oQjNREhaixWi6DELTIQRjGxUr40Mz-SVAQ",
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
        "id": "dyWw3VvpsnEw96E2DvwrHXilOtsUDuOr5iAHODDmIjHmH7ZfyBwHrWaOSScoIzsIIhzhGnYlitDeQRp4peAJJA",
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
Batch insert response: map[failed:[] successful:[A4iiiu4mG-PeBNJnHNi_rzUN54xD0GUfc9yu9zxHUSBJSN62XXQspjgrHQp-HPoLP2AQJ08pZ9Tz6T5mw4ja1w Fp7xS7UVZbEUCQmQajI6xjSNLx_Zkn6g858GNGiLdawGsb6PozJ1bG8FDaWQP62qR5sBK7UC-cEooL2mUp3V-g ckUagZAJqfGx59yMZs_rA7SLnw8MvQNyZ_EGpBgHF4CdDbZx1P1r2aALqN62kBDMraV5oAGK-xd3Ja79uIKp-Q 3r58eX9KnEanW2kCMpktGibJMXs-L4sTuuj9vurO6XMcW4NAriQxak1dy3mW0hFRJbhpRnjiXJr9kpwkyqYB_w HmNphjMBgemds-4ds2-ONzzPkD_I9rqleg0yZC_sFHdXvXEBv7St4uiNWYYVtHL1GP3au5ldxhkf_9Njdd9M4Q]]
✓ Batch insert completed

=== Creating test records for update/delete ===
Created 3 test records

=== Batch Update ===
Batch update response: map[failed:[] successful:[_6ju3lW6ifeftJnM_K19WXUe7RuVIEv1hlmySDuHQCNxv29s4iebwRFu_GdbTl4tXtLsNGf9Gd9hoNn834py1g 6cxSCBwnLDrOvQ9ppB7Gzk-nkheaw9Y6WeA3D2nzjt2oatc4edM7Iz7lbTosWyEnQBKQ8f-CVg2RtSrBd3nU9A yFLsm7fqqeds1jq_Px6uV7KT3GwCiCk5KYnjCx2414_78jvEyRjtrz-3ZuKlWc0EvgFJgsHd7UZqG2FQac8Xmw]]
✓ Batch update completed

=== Batch Delete ===
Batch delete response: map[failed:[] successful:[_6ju3lW6ifeftJnM_K19WXUe7RuVIEv1hlmySDuHQCNxv29s4iebwRFu_GdbTl4tXtLsNGf9Gd9hoNn834py1g 6cxSCBwnLDrOvQ9ppB7Gzk-nkheaw9Y6WeA3D2nzjt2oatc4edM7Iz7lbTosWyEnQBKQ8f-CVg2RtSrBd3nU9A yFLsm7fqqeds1jq_Px6uV7KT3GwCiCk5KYnjCx2414_78jvEyRjtrz-3ZuKlWc0EvgFJgsHd7UZqG2FQac8Xmw]]
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
Collection created with first record: 8tZT_N5YLqSOX3PrdQb0y_W1fbGTh2fz2gkTKnj06OB-6POwM2dflUJ3m1sUsLTQyBNHORBIrLGIcRcs4c-aIw

=== List Collections ===
Total collections: 15
Sample collections: [ttl_cache schema_products_client_py users batch_users test_collection ws_ttl_test websocket_test demo_collection schema_users_client_py test_accounts chat_configurations__ek0_testing schema_documents_client_py schema_employees_client_py scripts__ek0_testing chat_messages__ek0_testing]

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
✓ Inserted document: bmbJKbpObXJnY_hSwQQo4J11Oi1nRpCAE-DT1wZOHr6DbOPOIsoZ6oLRi5-aV9GmN6LWzauu_kaGh15QqSxVcw

=== Insert Document with TTL (5 minutes - integer) ===
✓ Inserted document: <nil>

=== Insert Document with TTL (30 minutes - duration string) ===
✓ Inserted document with duration string TTL: fKsXyqFwl8quj5cVK3v12Rn1Db8gCQpPcomnIC1c3-wPxbx67WoZncEqipTCCDCymM30j-VBXbks9s9k1yDSdw

=== Query Documents ===
✓ Found 6 documents with TTL

=== Update Document ===
✓ Updated document

=== Delete Document ===
✓ Deleted document

✓ All document TTL operations completed successfully
[32m✓ document_ttl.go completed successfully[0m
[34m
=== Running websocket_ttl.go ===[0m
✓ Authentication successful

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: ARDK-hyRDIzcUzW7sleJ7bJy7OqPobCR1CPR8OBugEkD6IRoVpCeFLltpZDUSpRdk3zDVRMZ5YJck6Py2KMhsg

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

✅ Script saved: fPg2lXhh020oBbYD9jDX2I_o0PX3BccXjGLF9MuWj7-F6jJJaBdIUUYBi5lEit4azI8QhmTIcG0B5kmUm8Urng
📊 Found 20 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: Z2UgYRv8NhSWqrGSj0UeEKCKUbEOC54h5fI6GDOvAnIxRO7R7NxH2YEuR1riqlBCh0beC6AkrQqmQ2xnFtPOoA
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: a71NzKR9MEDgWqVJjNiyK6m9qo2_XaCM2QHFD3EPGbSW7nd9gnaqPmShsySMOb7QdHCSxxplqY41ThZsm5QMBw
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
Created Alice: $1000 - ID: EJtm3yCekw0AsV_b5iP-Y0vUetJ4YCF6LWCXMvNILGLOhoSbqzTSXDjS_ByGPZefN3KpMI44ff1m4YZwi-ksXQ
Created Bob: $500 - ID: 346NJAcoC_FWUjBMNJtDKvU6RnyZ80LXsr7xXJ4_zRqYj8kdoYUNqYemz2JL6DnpVB4dYi5k-sGw5nUFseh2ng

=== Example 1: Begin Transaction ===
Transaction ID: cb9397bb-b92e-4b9d-a539-bba7f0a4b72c

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
New transaction: 76db0b13-4426-4315-b8c4-5c1fdd1d2d3f
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
   ✅ Script saved: 2pjtTXiBIV3DFtczWznuIqzGfN8HaC7tvwrgmt4Mq6xQvYW2p_BijEaltiCXjrhYvwXLzPxxFJ3RHYCSTsjZwg

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: pLxrglii1rRNAdhk8FvOhxTiiaJuK3H0s0qZkLDMNgFUZI1dki3-r3kxtATtEji-wgnMd3CU7E6pOgKGqw5atA
   📋 Name: map[type:String value:Alice Smith]
   📋 Email: map[type:String value:alice@example.com]
   📋 Status: map[type:String value:pending]
   📋 Credits: map[type:Integer value:0]

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: MOu2OVVsH4pe4Vsu0ldAs6o-bDojja0mU0DC0nmRQkDoJFIJMMAicl0el5r5m2nbW_0W0d3lK_uzzsj4McTWhA

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
   ✅ Script saved: wwM0W41w8BBYhWiaJ7_JrcQsG3qulia0dvuDU0uOBnJ9vLiMSSmkTXosR10lPUq6SvaoTzUVeGZ24wTC1zwCtQ

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
   ✅ Script saved: sjFBeKCOkqb61NzSRnoFgNmdUTJtROjO7qhtCVzgdhSxD6NhmyVt9gyQBgBSBqGeMNbmR2Oxwfsh4jWKGqdJ4A

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: 2pjtTXiBIV3DFtczWznu...
   ✅ Deleted script: MOu2OVVsH4pe4Vsu0ldA...
   ✅ Deleted script: wwM0W41w8BBYhWiaJ7_J...
   ✅ Deleted script: sjFBeKCOkqb61NzSRnoF...
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
Inserted: map[id:D4IjbLoL_CW1aMTPoDjujfhJjT5z5G5Nv9m6ewaqMy9YNIqAxfqt3mc-koAilFHKvGa1gR0eEq6CPHtGWYHb7Q]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-01-04T00:30:52Z] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:D4IjbLoL_CW1aMTPoDjujfhJjT5z5G5Nv9m6ewaqMy9YNIqAxfqt3mc-koAilFHKvGa1gR0eEq6CPHtGWYHb7Q metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Test Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:42]]

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 0
  active (Boolean): true
  price (Decimal): 99.990000
  created_at (DateTime): 2026-01-04 00:30:52 +0000 UTC
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1 tag2 tag3]
  metadata (Object): map[key:value nested:map[deep:true]]
  embedding (Vector): [0.1 0.2 0.3 0.4 0.5]
  categories (Set): [electronics computers]
  data (Bytes): 11 bytes
Plain record: map[active:true categories:[electronics computers] created_at:2026-01-04T00:30:52Z data:aGVsbG8gd29ybGQ= embedding:[0.1 0.2 0.3 0.4 0.5] id:D4IjbLoL_CW1aMTPoDjujfhJjT5z5G5Nv9m6ewaqMy9YNIqAxfqt3mc-koAilFHKvGa1gR0eEq6CPHtGWYHb7Q metadata:map[key:value nested:map[deep:true]] name:Test Record price:99.99 tags:[tag1 tag2 tag3] user_id:550e8400-e29b-41d4-a716-446655440000 value:42]

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-01-04T00:30:52Z] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:D4IjbLoL_CW1aMTPoDjujfhJjT5z5G5Nv9m6ewaqMy9YNIqAxfqt3mc-koAilFHKvGa1gR0eEq6CPHtGWYHb7Q metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Updated Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:100]]

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

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 3 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 4

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: false

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: 1NCJVlHMZujVR48pXdsgyMdYhz3Ra1TxwshClMdmbvBJRBKrC6Q9-EjfCYT-tnKBEQH2u1FrJ8jzOaS3AP47Yg

=== List Collections ===
Total collections: 14
Sample collections: [ttl_cache schema_products_client_py batch_users test_collection ws_ttl_test]

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: glcyRnfNCL3mhw7e-K0oFsqS9LjdsWotMM6WDxDqdz1CidLXMJfLxZ5prjkMzwK-ZgG4_iWZCpvhgMcBrwOqKQ

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: nmFDSWBBAIk5tXJGx3yeA97L4Dn9joI8RRYI3T_RxakswtbB-QSxPNmWGFcFWejypySp3eoZv9iAfwZtaLzLgQ

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
✓ Inserted test record: cMEZuQNZX6oOZXoF8gAGXmTCJ5sOljH5XH25BaV5XIHjAw-1vFQMNJAsxscYB7CDfj2GrF-zsbTj7krUCD3cyw

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
✓ Inserted document with TTL: MAzX_r_VaX8_N97JueNxNv0egizx2aSwbz6CQ_qbNp2xjzN_d80XNBXV2vZsLDwa7jlRttWVIh6PHB_jS7-zoQ

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
✓ Created session: O0e5OhjkxTgEk947WT4uU8b5O7jaV8ukCVC1jT4HoUVUxWYNjIQW59gljpW6ScYuz8eBNtjDRB7XPM1v_2XhTQ

=== Sending Chat Message ===
Message ID: kRgRzhzDPUuPtRHiLt9A4jsmeXYF_6ieYfxM-s2ChpNA1YWOX0cRnSE6a-fFSpocG3f4Z7OFeeWBU4sWqyMrTQ

=== AI Response ===
Based on the context, the available products and their prices are:

1. ekoDB Pro - This is an enterprise edition product with advanced features, priced at $299.
2. ekoDB Cloud - This is a fully managed cloud database service product, priced at $499.
3. ekoDB - This is a high-performance database product with AI capabilities, priced at $99.

=== Context Used (3 snippets) ===
  Snippet 1: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Enterprise edition product with advanced features id:JoUWFT9TLLOoP-eSI2s2c8BTWiGLEkYAfslUxmrf2AdRs8lirS5byeTe7x4zbsTDtirL-1D8cTa5i-V_OneP9A name:ekoDB Pro price:299] score:0.1111111111111111]
  Snippet 2: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Fully managed cloud database service product id:N7d1O9Wa7UVkx9RPE0lH2R2SFJea9CSDBhq6iUtWF89aMMJ4hn5sWY3gd4ULa5UdngnOVrLmN3mgtdEMJNdPOw name:ekoDB Cloud price:499] score:0.1111111111111111]
  Snippet 3: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:A high-performance database product with AI capabilities id:UHaPARaRYALo8VK_ZKfznHumWW5YHfoqUpYYPhSnJm9AWBDFyeLBjkc1zqDbEGG2v14Dvpo-ysC6FKQjipKrjA name:ekoDB price:99] score:0.1111111111111111]

Execution Time: 2392ms

=== Token Usage ===
Prompt tokens: 602
Completion tokens: 79
Total tokens: 681

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: PJ8nggyQAzxxbTPNehaA9ppN_05e5b5tbmQFUBTc1fBMGrb9aGE7lYK6i9vyy0vBfrOLFoD_REB7ukGOYzECWQ

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information provided, the product available is ekoDB. It is a high-performance database product.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 1gEpiig_ooUPP3uFvWJD_fdpqf0X3YixZde5fq9_-cUVEmMssvaRGeDJ__tRl4ysGm6BfsE-t3v6zVQNtL3glw
  Parent: PJ8nggyQAzxxbTPNehaA9ppN_05e5b5tbmQFUBTc1fBMGrb9aGE7lYK6i9vyy0vBfrOLFoD_REB7ukGOYzECWQ

=== Listing Sessions ===
✓ Found 9 sessions
  Session 1: 1gEpiig_ooUPP3uFvWJD_fdpqf0X3YixZde5fq9_-cUVEmMssvaRGeDJ__tRl4ysGm6BfsE-t3v6zVQNtL3glw (Untitled)
  Session 2: PJ8nggyQAzxxbTPNehaA9ppN_05e5b5tbmQFUBTc1fBMGrb9aGE7lYK6i9vyy0vBfrOLFoD_REB7ukGOYzECWQ (Untitled)
  Session 3: O0e5OhjkxTgEk947WT4uU8b5O7jaV8ukCVC1jT4HoUVUxWYNjIQW59gljpW6ScYuz8eBNtjDRB7XPM1v_2XhTQ (Untitled)
  Session 4: 6V4Pa6UrFZn93O2rj4Xl2oPWQBRhE5NdBj-mVQPTiEg9jLGGJBmAYi4aMsrNyEos-WdgVlcHz9orGRnSM0okww (Untitled)
  Session 5: otL5_twubZAn9rGrc4bEFnd5xkY4aS2V7VeuBxE3pivjxf5HDsQBPB-gbnqrtwHNe0x-K4V_bsQIlymnQFDwSw (Untitled)
  Session 6: 9_Jjc959t0oHyhGBza3R3NQbjHYLRTpswpfQR5l9OKH7tCF_LGFPkB1rbqnKFjS45RdDuv67_0nCrE_yC0eGJw (Untitled)
  Session 7: nG-we3gQtp1gSDq6fq6xMPlJ-4gSKMIesqqHvUqGbz21aNYku8Q_W2jLmV6iL1vTODEep_b5rEryHUaH2yHB-g (Untitled)
  Session 8: fabuYeYu50GbJZH1jgr2QWukNk_sPLaArrv4BWAEY-PAj_WKkMnZMeNrYn43vbRJcrsUksRWuKDz013FF4LMuA (Untitled)
  Session 9: iqrOsUYHuTFlzfksowEe8ciD6bzA2e94cOOZ7GyKEsD15VCgG1hxJsYOpdB_3OLEg2L7-kw21Vk-ZKuf-Et5zg (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: 1gEpiig_ooUPP3uFvWJD_fdpqf0X3YixZde5fq9_-cUVEmMssvaRGeDJ__tRl4ysGm6BfsE-t3v6zVQNtL3glw

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: LDJq43na3e_QCPc0jfHz6LicBI7wKDeCTyY9SI6W2wFBXX8gkAeYuFE1Kh0DG1-L2vzf5H3n9zYqFs565hhvrQ

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the context provided, "ekoDB" is available. It is a high-performance database product. The price for this product is $99.

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
✓ Created second session: VRHWIC7kF05GxbyDMiUw3FVNimhEZRsjE5BzXOgppbkEvI0gPniZaKTNKA-Ta9tI1IjBFEdeIT2myg1F_JvQ4g
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
  2. Score: 0.754
  3. Score: 0.733

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.693
  2. Score: 1.502
  3. Score: 0.307

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

✅ Script saved: Nm67KXqG9HDRvRp-arxhnc4Jm9tDRUU1axJBQmhOCly4IiP6henUeKSON2OzquWhcD-LcTb4UWVSyQuiW871lw
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
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 0mpwjFimU-cXSsHkUr74tpgiBwY6T3C_dZRRratqPpdIPYFbKKdbHwIidTgee8bfzQ0edcRti1_oEkBSmYj83g
Created Bob: $500 - ID: xR9jTyWlflur2btckeTU9CnMuR6nxAid33LJ-Zd2K6pb30kuqN9AGJn4A2bSSrmuKScAAXeLvFvH-ilPo_jfqw

=== Example 1: Begin Transaction ===
Transaction ID: 43a4eae5-3547-45c4-a6e4-d8da65a7acbd

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 339b0fd6-3e01-4961-9aec-14dfa29475b4
Status before rollback: Active
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
✅ [32mGo client examples complete![0m
