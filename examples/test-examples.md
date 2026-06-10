make test-examples
🧪 [36mRunning Rust examples (direct HTTP/WebSocket)...[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.21s
     Running `target/debug/examples/simple_crud`
✓ Authentication successful

=== Insert Document ===
Inserted: Object {"id": String("HTkq-6yCFvIp0eKHaL-GUAQPpc_GWdTIzoZ5-arsF2pKbID1SaLDlmpmhPh5DHOXT6qy6Qzbk22WF61hXuNJrg")}

=== Find by ID ===
Found: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("HTkq-6yCFvIp0eKHaL-GUAQPpc_GWdTIzoZ5-arsF2pKbID1SaLDlmpmhPh5DHOXT6qy6Qzbk22WF61hXuNJrg"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}

=== Find with Query ===
Found documents: Array [Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("HTkq-6yCFvIp0eKHaL-GUAQPpc_GWdTIzoZ5-arsF2pKbID1SaLDlmpmhPh5DHOXT6qy6Qzbk22WF61hXuNJrg"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}]

=== Update Document ===
Updated: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("HTkq-6yCFvIp0eKHaL-GUAQPpc_GWdTIzoZ5-arsF2pKbID1SaLDlmpmhPh5DHOXT6qy6Qzbk22WF61hXuNJrg"), "name": Object {"type": String("String"), "value": String("Updated Record")}, "value": Object {"type": String("Integer"), "value": Number(100)}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/simple_websocket`
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: "Rx_EhUtGO6iii80P81nw7Bu_teSO3QUJk0QQ5tqdYHt_E7rl36XSAj6fL0Fa5ZVfaAmRGe0SxQ4uDM2G7ETLTQ"

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "messageId": "1781055137399241000",
  "payload": {
    "data": [
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "Rx_EhUtGO6iii80P81nw7Bu_teSO3QUJk0QQ5tqdYHt_E7rl36XSAj6fL0Fa5ZVfaAmRGe0SxQ4uDM2G7ETLTQ",
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
✓ Retrieved 1 record(s) via WebSocket

✓ WebSocket example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/batch_operations`
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/kv_operations`
✓ Authentication successful

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: Object {"userId": Number(123), "username": String("john_doe")}

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: Object {"name": String("Product 1"), "price": Number(29.99)}
cache:product:2: Object {"name": String("Product 2"), "price": Number(39.989999999999995)}
cache:product:3: Object {"name": String("Product 3"), "price": Number(49.989999999999995)}

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/collection_management`
✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: "0a-2ZfI_MH1b96dfOO8g2fcj8W2x9DhRttilNHtYTJvtjWD-dwPRMAiFnFeUxVb86kJUPFgUSbVBwMzTVezmwA"

=== List Collections ===
Total collections: 5
Sample collections: ["batch_users", "test_collection", "demo_collection", "audit__ek0_testing", "websocket_test"]

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/document_ttl`
✓ Authentication successful

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: "r-rqURjWukL_JLay8EzrbBt2atmnxggnWlc0x2iz7KXLvR37D8B45jrSe8lpDgDhNdid_56umh8zkAaeBBNcjA"

=== Insert Document with TTL (5 minutes - integer) ===
✓ Inserted document: "dK9YH_AyY1W7YT8Oh6qv9UvlWaj-7lMUujdo8LqAl8nnZCOri3tffrCcujyXmGJ4YUDENp-fOoBuv2vBn6tSHg"

=== Insert Document with TTL (30 minutes - duration string) ===
✓ Inserted document with duration string TTL: "749daJJIOKmBtH1GxtHCyUVSB-Gw35_rM7OcoD9hmwXGJpKblawjF5i6785Ur0ce3dDaOGEP0REgIQGg_CyrCg"

=== Query Documents ===
✓ Found 3 documents with TTL

=== Update Document ===
✓ Updated document

=== Delete Document ===
✓ Deleted document

✓ All document TTL operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/websocket_ttl`
✓ Authentication successful

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: String("Id2TFgrTe-MKpLWsrnzupzj5nzTb7rD7aROeSwwzgTZfLhXlPxEKdvlYhHj7m-4lTbIa02uipgapxFqLA9kM3w")

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket

Record 1:
  id: "Id2TFgrTe-MKpLWsrnzupzj5nzTb7rD7aROeSwwzgTZfLhXlPxEKdvlYhHj7m-4lTbIa02uipgapxFqLA9kM3w"
  name: {"type":"String","value":"WebSocket TTL Test"}
  ttl: "2026-06-10T02:33:36.684092Z"
  value: {"type":"Integer","value":42}

✓ WebSocket TTL example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/http_functions`
🚀 ekoDB Functions Example (Rust/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Function saved: 6opBDT97USkeM-Hl5N7aSCyWIA3Uu9o7jO1fmeScGmmwBPG7DcUTau_7nLzrkQBLUyV4-HWRawKjCuhP12kHfw
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Function saved: HoOaw1gmVROVc7IPCWFy1bi7azBP2SnZnecWzdB8sj2iP1YGc1pTHIVdsvpt_tYul8n4Tq8HD-89gWNnf_QF6g
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Complex Filter with Multiple Conditions

✅ Function saved: k2sD_0-lc6FoniGEETIO7OHhg8HPrwPSMMIs7eQnwqJNHBepRjpumeHp4ygF5tMAUIsycQLZ-dkJ9RtOX1CEkw
📊 Found 3 users (status=active, score>50, sorted by score)

📝 Example 4: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: BURxBMPmXJXrmAjNmrfSmbvK3nb9cnJrVh2Yn3TkBMDacZiu9rvb_9GzmeSVJLBqUuqInHHvPN-S_l0dXXe4ZA
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"avg_score":{"type":"Float","value":50.0},"count":{"type":"Integer","value":5},"max_score":{"type":"Integer","value":90},"status":{"type":"String","value":"inactive"}}
   {"avg_score":{"type":"Float","value":60.0},"count":{"type":"Integer","value":5},"max_score":{"type":"Integer","value":100},"status":{"type":"String","value":"active"}}

📝 Example 4: Function Management

📋 Total scripts: 4
🔍 Retrieved script: Get Active Users
✏️  Function updated
🗑️  Function deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

✅ All examples completed!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/transactions`
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: MU9ZeDiEYDuHc4mYLoHdIJaIfr39nX10Wc2k8aH55KwVm-YL_jgb3R4A7iV3jiJQHT2BiRdBcXF5sGQ3o66Znw
Created Bob: $500 - ID: ODmpw7yCE1f88aWnDzZ1xk5EnFJy8xTjMS3px2WBYtoneDP9fTiIIjDV_Yg0QzEu4qa1QYcKHS7-FC41ErFTNQ

=== Example 1: Begin Transaction ===
Transaction ID: bfddc226-1ea1-49a6-9d35-5aeea34f42f8

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: "Active"
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: {"type":"Integer","value":800}
Bob: {"type":"Integer","value":700}

=== Example 5: Rollback ===
New transaction: c6d08d0f-724f-4682-8877-1617e9949e66
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: {"type":"Integer","value":700}

=== Cleanup ===
✓ Deleted test accounts

✓ All transaction examples completed
✅ [32mRust direct examples complete![0m
🛠️  [36mBuilding client library...[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
✅ [32mClient build complete![0m
🧪 [36mRunning Rust client library examples...[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_simple_crud`
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: Record({"id": String("gfbM71O4o11d5snjzPnGoww5GqJ8OyJvPTEEFYVW8vL6US6l6DB7K_wNfC_ZDGKGHM4xDn1tf58RTsvrrGecMQ")})

=== Find by ID ===
Found: Record({"active": Object({"type": String("Boolean"), "value": Boolean(true)}), "data": Object({"value": String("aGVsbG8gd29ybGQ="), "type": String("String")}), "metadata": Object({"type": String("Object"), "value": Object({"key": String("value"), "nested": Object({"deep": Boolean(true)})})}), "created_at": Object({"value": String("2026-06-10T01:34:19.274509Z"), "type": String("DateTime")}), "name": Object({"type": String("String"), "value": String("Test Record")}), "embedding": Object({"type": String("Array"), "value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)])}), "price": Object({"type": String("Float"), "value": Float(99.99)}), "user_id": Object({"value": String("550e8400-e29b-41d4-a716-446655440000"), "type": String("String")}), "tags": Object({"type": String("Array"), "value": Array([String("tag1"), String("tag2"), String("tag3")])}), "id": String("gfbM71O4o11d5snjzPnGoww5GqJ8OyJvPTEEFYVW8vL6US6l6DB7K_wNfC_ZDGKGHM4xDn1tf58RTsvrrGecMQ"), "value": Object({"type": String("Integer"), "value": Integer(42)}), "categories": Object({"value": Array([String("electronics"), String("computers")]), "type": String("Array")})})

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Some("Test Record")
  value (Integer): Some(42)
  active (Boolean): Some(true)
  price (Decimal): Some(99.99)
  created_at (DateTime): Some("2026-06-10T01:34:19.274509Z")
  user_id (UUID): Some("550e8400-e29b-41d4-a716-446655440000")
  tags (Array): 3 items
  metadata (Object): 2 keys
  embedding (Vector): 5 dims
  categories (Set): 2 items
  data (Bytes): 11 bytes

=== Find with Query ===
Found documents: [Record({"name": Object({"value": String("Test Record"), "type": String("String")}), "id": String("gfbM71O4o11d5snjzPnGoww5GqJ8OyJvPTEEFYVW8vL6US6l6DB7K_wNfC_ZDGKGHM4xDn1tf58RTsvrrGecMQ"), "user_id": Object({"value": String("550e8400-e29b-41d4-a716-446655440000"), "type": String("String")}), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "data": Object({"value": String("aGVsbG8gd29ybGQ="), "type": String("String")}), "embedding": Object({"type": String("Array"), "value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)])}), "tags": Object({"value": Array([String("tag1"), String("tag2"), String("tag3")]), "type": String("Array")}), "value": Object({"type": String("Integer"), "value": Integer(42)}), "created_at": Object({"value": String("2026-06-10T01:34:19.274509Z"), "type": String("DateTime")}), "price": Object({"type": String("Float"), "value": Float(99.99)}), "metadata": Object({"value": Object({"key": String("value"), "nested": Object({"deep": Boolean(true)})}), "type": String("Object")}), "categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])})})]

=== Update Document ===
Updated: Record({"metadata": Object({"value": Object({"nested": Object({"deep": Boolean(true)}), "key": String("value")}), "type": String("Object")}), "price": Object({"value": Float(99.99), "type": String("Float")}), "created_at": Object({"type": String("DateTime"), "value": String("2026-06-10T01:34:19.274509Z")}), "embedding": Object({"type": String("Array"), "value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)])}), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "tags": Object({"type": String("Array"), "value": Array([String("tag1"), String("tag2"), String("tag3")])}), "value": Object({"type": String("Integer"), "value": Integer(100)}), "id": String("gfbM71O4o11d5snjzPnGoww5GqJ8OyJvPTEEFYVW8vL6US6l6DB7K_wNfC_ZDGKGHM4xDn1tf58RTsvrrGecMQ"), "categories": Object({"value": Array([String("electronics"), String("computers")]), "type": String("Array")}), "name": Object({"value": String("Updated Record"), "type": String("String")}), "data": Object({"value": String("aGVsbG8gd29ybGQ="), "type": String("String")}), "user_id": Object({"type": String("String"), "value": String("550e8400-e29b-41d4-a716-446655440000")})})

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_simple_websocket`
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: dDizunSn0IFoSOqfWhW7ORQbhTzYBasdNzoFFUgn22-6qJTXPXzRoTlSoZgn5zaHFF12K9ZxT_tQMujpSfEhXQ

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket

Record 1:
  active: true
  id: "dDizunSn0IFoSOqfWhW7ORQbhTzYBasdNzoFFUgn22-6qJTXPXzRoTlSoZgn5zaHFF12K9ZxT_tQMujpSfEhXQ"
  name: "WebSocket Test Record"
  value: 42

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_batch_operations`
✓ Client created

=== Batch Insert (via multiple inserts) ===
✓ Inserted 5 records
✓ Verified: Found 5 total records in collection

=== Update Records ===
✓ Updated 3 records

=== Delete Records ===
✓ Deleted 3 records

=== Cleanup ===
✓ Deleted collection

✓ All batch operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_collection_management`
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: "kmx0wMGNZgRPAK2olIXubtPanWJr4ORuPMm7Q5_L_Iz3cQ_p1QTc1DA4MzmQ3PzCiDR4GWB_Tw239pUI65BFAQ"

=== List Collections ===
Total collections: 11
Sample collections: ["batch_users", "functions__ek0_testing", "agent_function_versions__ek0_testing", "test_collection", "ttl_cache"]

=== Count Documents ===
Document count: 1

=== Check Collection Exists ===
Collection exists: true

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_kv_operations`
✓ Client created

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: Object {"userId": Number(123), "username": String("john_doe")}

=== KV Batch Set ===
✓ Batch set 3 keys
  cache:product:1: success
  cache:product:2: success
  cache:product:3: success

=== KV Batch Get ===
✓ Batch retrieved 3 values
  cache:product:1: Record({"price": Float(29.99), "name": String("Product 1")})
  cache:product:2: Record({"name": String("Product 2"), "price": Float(39.989999999999995)})
  cache:product:3: Record({"price": Float(49.989999999999995), "name": String("Product 3")})

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 3 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 4

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: false

=== KV Batch Delete ===
✓ Batch deleted 3 keys
  cache:product:1: deleted
  cache:product:2: deleted
  cache:product:3: deleted

✓ All KV operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_transactions`
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 2-pjsDiiPC8keeD16_UEVeKfiJ4u779tYuJ16TDSPLohRBw2BHI9yIKxLop_JK-_0zCvb4sDZEhtqJIbSBNGuw
Created Bob: $500 - ID: Uf_b87KwNP8guSUb9dP9fG72YsUL6DwIirlQGF2gPZpTY7g-TnKHeWOwbdpJGJ8tj8A-b7lAc8ogfnnwcKTT_A

=== Example 1: Begin Transaction ===
Transaction ID: 3620c5d0-7002-4e14-89ff-03e51b7a8c08

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: "Active"
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 26cc19a4-21a4-4067-bda7-dcc7e9a68dda
Status before rollback: "Active"
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_query_builder`
=== ekoDB Query Builder Example ===

=== Inserting Sample Data ===
✓ Inserted 5 users

=== Simple Equality Query ===
✓ Found 3 active users
  - Some(Object({"value": String("Bob"), "type": String("String")}))
  - Some(Object({"value": String("David"), "type": String("String")}))
  - Some(Object({"value": String("Alice"), "type": String("String")}))

=== Range Query (age >= 28 AND age < 35) ===
✓ Found 3 users in age range
  - Some(Object({"value": String("Bob"), "type": String("String")}))
  - Some(Object({"value": String("David"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("Eve")}))

=== IN Operator ===
✓ Found 4 users with status active or pending
  - Some(Object({"value": String("Bob"), "type": String("String")}))
  - Some(Object({"value": String("David"), "type": String("String")}))
  - Some(Object({"value": String("Alice"), "type": String("String")}))
  - Some(Object({"value": String("Eve"), "type": String("String")}))

=== NOT IN Operator ===
✓ Found 4 users not inactive

=== String Pattern Matching ===
✓ Found 5 users with @example.com email

=== Prefix Query (StartsWith) ===
✓ Found 1 users with names starting with A
  - Some(Object({"value": String("Alice"), "type": String("String")}))

=== Complex Query (active AND age >= 28 AND score > 1500) ===
✓ Found 2 users matching all conditions
  - Some(Object({"value": String("Bob"), "type": String("String")}))
  - Some(Object({"value": String("David"), "type": String("String")}))

=== OR Query ===
✓ Found 2 users with age < 28 OR age > 32
  - Some(Object({"type": String("String"), "value": String("Charlie")}))
  - Some(Object({"value": String("Alice"), "type": String("String")}))

=== Sorted Query (by score descending) ===
✓ Top 3 users by score:
  1. Some(Object({"value": String("Bob"), "type": String("String")}))
  2. Some(Object({"value": String("David"), "type": String("String")}))
  3. Some(Object({"type": String("String"), "value": String("Alice")}))

=== Pagination (page 2, size 2) ===
✓ Page 2 results:
  - Some(Object({"value": String("Charlie"), "type": String("String")}))
  - Some(Object({"value": String("David"), "type": String("String")}))

=== NOT Operator ===
✓ Found 4 users NOT inactive

=== Complex Chained Query ===
✓ Found 3 users with all conditions

=== Cleanup ===
✓ Deleted collection

✓ All query builder operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_search`
=== ekoDB Search Example ===

=== Inserting Sample Documents ===
✓ Inserted 5 sample documents

=== Basic Text Search ===
✓ Found 3 results for 'programming'
  1. Score: 19.8000 - Some(Object {"type": String("String"), "value": String("Rust Programming")})
  2. Score: 6.6000 - Some(Object {"type": String("String"), "value": String("JavaScript Web Development")})
  3. Score: 6.6000 - Some(Object {"type": String("String"), "value": String("Python for Data Science")})

=== Fuzzy Search ===
✓ Found 3 results for 'progamming' (typo)
  1. Score: 1.6500 - Some(Object {"type": String("String"), "value": String("Rust Programming")})
  2. Score: 0.5500 - Some(Object {"type": String("String"), "value": String("JavaScript Web Development")})
  3. Score: 0.5500 - Some(Object {"type": String("String"), "value": String("Python for Data Science")})

=== Field-Specific Search ===
✓ Found 4 results in title/description
  1. Score: 4.0000
     Title: Some(Object {"type": String("String"), "value": String("Machine Learning Basics")})
     Matched: ["description.value", "title", "title.value", "description"]
  2. Score: 2.0000
     Title: Some(Object {"type": String("String"), "value": String("Python for Data Science")})
     Matched: ["description.value", "description"]
  3. Score: 1.0000
     Title: Some(Object {"type": String("String"), "value": String("Rust Programming")})
     Matched: ["description", "description.value"]
  4. Score: 1.0000
     Title: Some(Object {"type": String("String"), "value": String("Database Design")})
     Matched: ["description.value", "description"]

=== Weighted Search ===
✓ Found 2 results with field weights
  1. Score: 23.1000 - Some(Object {"type": String("String"), "value": String("Python for Data Science")})
  2. Score: 3.3000 - Some(Object {"type": String("String"), "value": String("Machine Learning Basics")})

=== Advanced Search Options ===
✓ Found 1 results with stemming
  1. Score: 19.8000 - Some(Object {"type": String("String"), "value": String("Database Design")})

=== Search with Limit ===
✓ Limited to 2 results (requested 2)
  1. Score: 19.8000 - Some(Object {"type": String("String"), "value": String("Rust Programming")})
  2. Score: 6.6000 - Some(Object {"type": String("String"), "value": String("JavaScript Web Development")})

=== Cleanup ===
✓ Deleted collection

✓ All search operations completed successfully
Execution time: 2ms
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_schema`
=== ekoDB Schema Management Example ===

=== Creating Collection with Schema ===
✓ Created collection 'schema_client_rust' with schema

=== Inserting Valid Documents ===
✓ Inserted user 1: Some(String("QLo9IVb2Io1uhjH2n6EDv56YvYSTV6-HRCgekExFKaAi_tzpza3w_Wr_4aQpptoVr7NefS_3YF6jOzMWbZ8qfw"))
✓ Inserted user 2: Some(String("666HIiYY9Q5VePEjgk3flrn9v7rkmBUfnjpuBZVY1K81E6jZSNvx72ZjtOgHfFeG4EHnFnSoAtzUwaGW0SMjFA"))

=== Getting Schema ===
✓ Schema for schema_client_rust:
  - age: Integer
  - status: String
  - email: String
    (required)
  - title: String
    (required)

=== Listing Collections ===
✓ Total collections: 11
  Sample: ["batch_users", "functions__ek0_testing", "agent_function_versions__ek0_testing", "test_collection", "schema_client_rust"]

=== Cleanup ===
✓ Deleted collection

✓ All schema management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_joins`
=== ekoDB Rust Client - Join Operations Example ===

=== Setting up sample data ===
✓ Sample data created

=== Example 1: Single collection join (users with departments) ===
✓ Found 2 users with department data
  - Bob Smith: Sales
  - Alice Johnson: Engineering

=== Example 2: Join with filtering ===
✓ Found 1 users in Engineering
  - Alice Johnson: Building A

=== Example 3: Join with user profiles ===
✓ Found 2 users with profile data
  - Bob Smith: Sales Manager
  - Alice Johnson: Senior Software Engineer

=== Example 4: Join orders with user data ===
✓ Found 2 completed orders
  - Mouse ($25) by Alice Johnson
  - Laptop ($1200) by Alice Johnson

=== Example 5: Complex join with multiple conditions ===
✓ Found 2 users with example.com emails
  - Alice Johnson (alice@example.com): Building A
  - Bob Smith (bob@example.com): Building B

=== Cleanup ===
✓ Deleted test collections

✓ Join operations example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_document_ttl`
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: "azYO40Ie9z-6oXRgOuowD1riylPUvrePaeFaMO34OiOq4ngkWr-j1MxZhRAKL_6z4Zygg6-TGK1A-bk8xxng_A"

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: Some(String("LBItO5UeFLFFVA3Nd8nOXPvuDUCHe6w8MOBa7b-I5KuaWhikVH-HtH3Qw6MhZYyqw5Cdnzf9s5v8SdXyrFzoQA"))

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_websocket_ttl`
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: Some(String("l4SqV8fmoGwiSpYS2xLkwQERUxztYSdkBYVgFTC_xss4relkhMOYljL942h0wauyhefzcOLdwAYWoSO4ifk3HA"))

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_edge_cache`
=== ekoDB as Edge Cache - Simple Example ===

Setting up edge cache collection...
✓ Cache entry created

Creating edge cache lookup script...
✓ Edge cache script created: hqN7GZ-TzrDDGcZSjPzrvgdE2uVOU4m3efJ8C0fTHyqiPlvkmliMXhpJyjusLu1R6wK2xKG3p_7CpZzyjzFHkw

Call 1: Cache lookup
Response time: 15ms
Found 1 cached entries

Call 2: Cache lookup (connection warm)
Response time: 14ms
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_functions`
=== ekoDB Rust Client - Functions Example ===

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

ℹ️  Function 'get_active_users' already existed — updated instead
✅ Function saved: vRTn4KENrjRk4Yj3IrvagB7PDMk3yNLSW0QfL1xDEw_I6rv4CCK240FLwkEuo7ZMdsyTNGXSh6SSYcAIek_UzQ
📊 Found 20 active users

📝 Example 2: Parameterized Function

✅ Function saved: HR9nee5OQX51wf9E7JbBQ5JzKFD5qX9XrD2y3Z_TWyU1L4NZEj5LjML7Pks8Czw5I48Ub8BILwXY1LfCwQk6PA
📊 Found 20 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: lv4SBeLMe_M5SZ7xryTZsSr4ipBcy0-RznCIlIzg9TSH3bZNX4PC8-cPLjiffXyhB59JKIjg_l_V4RkHhjW4Cg
📊 Statistics: 2 groups

📝 Example 4: Function Management

📋 Total functions: 5
🔍 Retrieved function: Get Active Users
✏️  Function updated
🗑️  Function deleted

ℹ️  Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.


✅ All examples completed!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_function_composition`
=== ekoDB Function Composition Examples ===

📋 Setting up test data...

✅ Test data ready

📝 Example 1: Basic Function Composition

Building reusable functions that call each other...

✅ Saved reusable function: fetch_user
✅ Saved composed function: get_user_wrapper (calls fetch_user + projects fields)

📊 Result from composed function:
   Records: 1
   Name: User 1
   Department: engineering

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 197.843625ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "website": "hildegard.org",
      "email": "Sincere@april.biz",
      "id": 1,
      "name": "Leanne Graham",
      "address": {
        "city...

Second call (cache hit - from cache):
   ⏱️  Duration: 41.670875ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "name": "Leanne Graham",
      "address": {
        "geo": {
          "lng": "81.1496",
          "lat": "-37.3159"
        },
        "city"...
   🚀 Cache speedup: 4.8x faster!

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
   - fetch_enriched_user: Used in 50 workflows
   - get_engineering_user: Specific workflow


✅ All composition examples completed!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_functions_complete`
🚀 ekoDB Rust Complete Functions Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Function saved: cGny5vVUfFIev-57yumPohx-9txf8FHuXliNSqZcWNBxOe0mYzYmGhOryI46wkSXaTIovpTO9H5AtToEQbEZxw
📊 Found 2 product groups
   Record({"count": Object({"value": Integer(2), "type": String("Integer")}), "avg_price": Object({"type": String("Float"), "value": Float(474.0)}), "category": Object({"type": String("String"), "value": String("Furniture")})})
   Record({"avg_price": Object({"value": Float(575.6666666666666), "type": String("Float")}), "category": Object({"value": String("Electronics"), "type": String("String")}), "count": Object({"type": String("Integer"), "value": Integer(3)})})
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   Record({"category": Object({"type": String("String"), "value": String("Electronics")}), "count": Object({"value": Integer(3), "type": String("Integer")})})
   Record({"count": Object({"type": String("Integer"), "value": Integer(2)}), "category": Object({"type": String("String"), "value": String("Furniture")})})
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_functions_kv_wrapped`
🚀 ekoDB Rust KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: Some(String("3GwkRfB054Yw3MeRIzQmqJWMYBmRkSWlbkEMI2wgZodK0mkRwOuuat_ia37WEvGYYfxcY4G7Trv-UqidP0nPWA"))
✅ Inserted 2 products with wrapped types

📝 Example 2: function with Wrapped Type Parameters

✅ Function saved: AbQSma_rMoROAVWt4LH4nOx825t2U0b-vrx51rAeHQM12hzxihBznlDL7_v5s9m1ENljAkHxooFu59cPx37_eA
📊 function executed
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: Some(Object {"role": String("admin"), "userId": String("user_abc")})
🔍 Key exists: true
✅ Set cached data
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: HoRhaiTyGTA0UjgvSUrDe663mp0lNSrl9DOV5PjmMlxPZ2DCH3poQDZwb7zhOQWBYfJdISvOoGKsOhFky3RRBw
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Function

✅ Function saved: IgjTNFuNHEmTQ5bZhvx-2_sDfPDxbnXlprPleVJiVoWtgB0TbpV5BZ1CYrvkEqWGeR9n4DHHs4H2IfHR1hCP5A
📊 Processed order with caching
⏱️  Stages executed: 1
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All KV & Wrapped Types examples completed!

💡 Key takeaways:
   ✅ Use FieldType variants for type-safe wrapped values
   ✅ FieldType::Decimal preserves precision (no floating point errors)
   ✅ KV store is great for caching and quick lookups
   ✅ KV operations work within scripts
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_swr_pattern`
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_api_user_rs (MOEWQGLKN52HiTRmXDZwyXG7QJMVSS0Ez2lmlA5xAfxXzeJeNNUO2eFBZD9ksalyfvbCX11p_SwWTbdF1mkflg)

Step 2: First call - Cache miss, fetches from API
Result: FunctionStats { input_count: 0, output_count: 1, execution_time_ms: 0, stages_executed: 2, stage_stats: [] }
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 44ms (served from cache)
✓ Lightning fast cache hit

🧹 Cleaning up...
✓ Cleanup complete

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_swr_native`
=== ekoDB Native SWR Function ===

This example shows the new simplified SWR function that replaces
the manual FindById → If → HttpRequest → Insert pattern.

Example 1: Basic GitHub User Cache with Native SWR
─────────────────────────────────────────────────────

✓ Created native SWR script: github_user_native (wwO8WHaWvdSjJwRofZMZedITdjQj8UyS500NPV--y0ngN8rd6sjE8pao-a4pRIFqFZMT070KDnaaF-TCyv_nsg)

First call (cache miss - will fetch from GitHub API):
  Response time: 148ms
  Records returned: 1
  ✓ Data fetched from API and cached with 15m TTL

Second call (cache hit - instant from KV store):
  Response time: 21ms
  Speedup: 7.0x faster
  ✓ Lightning fast cache hit


Example 2: SWR with Audit Trail Collection
─────────────────────────────────────────────────────

✓ Created SWR script with audit trail: product_swr_audit (IgBCq9aIo_gwsKKrXwKpg3bmcRJTyTXlo8x8JdlbSizhAXVzvyFBZY6wfw4ogve7ZyjdFk37yf5xSi5KtoDHlg)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
─────────────────────────────────────────────────────

Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (BJtbI5Ln3cVS-Vq3_W2rYzQjWJ5Yi29FQeoOGrTdFEoYgIvbCLxYICW2MTFpIlVIDwpY-i0eRghSrqlLhwk0YQ)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
─────────────────────────────────────────────────────

✓ Created dynamic TTL script: flexible_cache (md55alXP5kXQd1vzrYD3BlY3rd59dSsTffs2uXFRLjvdOH-SoIEl3WtCTGDIrPQzIcoH2kj-LFfcnPKvTDDoBw)
  ✓ Cached with TTL: 5m (5 minutes)
  ✓ Cached with TTL: 1h (1 hour)
  ✓ Cached with TTL: 30s (30 seconds)

=== Key Benefits of Native SWR Function ===
✅ Simpler: One function instead of 4 (FindById → If → HttpRequest → Insert)
✅ Duration strings: Use '15m', '1h', '2h' instead of calculating seconds
✅ Built-in audit: Optional collection parameter for automatic logging
✅ Auto-enrichment: output_field populates params for downstream functions
✅ Transactional: Works correctly in both transactional and non-transactional contexts
✅ KV-optimized: Uses native KV store with proper TTL handling

=== Performance Comparison ===
Old pattern (manual):  ~4 function calls, complex script logic
New pattern (native):  ~1 function call, handled by server
Result: Simpler code, faster execution, easier maintenance

🧹 Cleaning up test data...
✓ Cleanup complete

=== Summary ===
The native SWR function provides a production-ready cache-aside pattern
with minimal code and maximum flexibility. Use it for:
  • API gateway caching
  • External API aggregation
  • Microservice response caching
  • Edge computing patterns
  • Real-time data enrichment pipelines

    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_functions_advanced`
🚀 ekoDB Rust Advanced Functions Example

📋 Setting up test data...
✅ Created 8 products

📝 Example 1: List All Products

✅ Function saved
📊 Found 8 products
⏱️  Execution time: 0ms

📝 Example 2: Group Products by Category

✅ Function saved
📊 Category breakdown:
   Record({"count": Object({"type": String("Integer"), "value": Integer(5)}), "avg_price": Object({"value": Float(367.0), "type": String("Float")}), "category": Object({"type": String("String"), "value": String("Electronics")})})
   Record({"count": Object({"value": Integer(3), "type": String("Integer")}), "category": Object({"value": String("Furniture"), "type": String("String")}), "avg_price": Object({"value": Float(365.6666666666667), "type": String("Float")})})
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced script examples finished!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_functions_ai`
🚀 ekoDB Rust AI Functions Example

📋 Setting up test data...
✅ Created 2 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
🤖 AI Response:
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_functions_crud`
🚀 ekoDB Rust CRUD Functions Example

📋 Setting up test data...
✅ Created 10 test users

📝 Example 1: List All Users

✅ Function saved
📊 Found 10 users
⏱️  Execution time: 0ms

📝 Example 2: Count Users by Status

✅ Function saved
📊 User counts by status:
   Record({"count": Object({"value": Integer(7), "type": String("Integer")}), "status": Object({"type": String("String"), "value": String("active")})})
   Record({"status": Object({"type": String("String"), "value": String("inactive")}), "count": Object({"value": Integer(3), "type": String("Integer")})})
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD script examples finished!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_functions_search`
🚀 ekoDB Rust Search Functions Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Function saved
📊 Found 5 documents
   1. Introduction to Machine Learning (AI)
   2. Database Design Principles (Database)
   3. Natural Language Processing (AI)
   4. Vector Databases Explained (Database)
   5. Getting Started with ekoDB (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   Record({"count": Object({"type": String("Integer"), "value": Integer(2)}), "category": Object({"value": String("AI"), "type": String("String")})})
   Record({"category": Object({"type": String("String"), "value": String("Database")}), "count": Object({"type": String("Integer"), "value": Integer(3)})})
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_chat_basic`
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: 1eyYvkAa7VtrEl2EJ-CuUCoLL3yBazPNyI9Bmx9_-oE21yh6sdcaRibFYofsJpsdK35PC6_4AQg83FBJuliVSw

=== Sending Chat Message ===
Message ID: LVtpczK_AnTVX7NVmwDhoIRI8Ku0BKAIrtS5rjsJiqgPewpOuuek0nXdgfe73zRWd65rYgKgWsW-I0v60Qv0cQ

=== AI Response ===
Response 1: ekoDB is a high-performance database that integrates AI capabilities and intelligent caching for real-time operations. It supports various advanced features aimed at enhancing user interaction and data management.

Some key features of ekoDB include:

1. **AI Chat Integration**: The chat feature allows users to query the database using natural language and receive AI-powered responses with relevant context.
2. **Search Capabilities**: ekoDB supports full-text search, vector search, and hybrid search, which come with automatic context retrieval, allowing for efficient information retrieval.
3. **Documentation**: ekoDB provides comprehensive documentation to help users understand its functionalities and integration capabilities.

These features enable users to efficiently manage their data while leveraging AI for improved query performance and contextual understanding.

=== Context Used (3 snippets) ===

Snippet 1:
  Collection: client_chat_basic_rust
  Score: 0.6333
  Matched Fields: ["title", "category", "content"]
  Record: Object {"category": String("features"), "content": String("The chat feature allows you to query your database using natural language and get AI-powered responses with relevant context."), "id": String("YExkS3ghjbUtagE22-Fbf1TBp3_FeajmMbDvvJALd573eGhKIuzBZbsaW0SZVy5Qup_cc7WxzjwWE9dNAXCAvg"), "title": String("AI Chat Integration")}

Snippet 2:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["content", "title"]
  Record: Object {"category": String("documentation"), "content": String("ekoDB is a high-performance database with intelligent caching, real-time capabilities, and AI integration."), "id": String("3UpzPENp-E-GDugkH5u7wjFEbv43uxOiymoghHdKZSv6cO8O5tywZCHXEBhrK4-onw33mziAj5VlzZSSI-z0ow"), "title": String("Introduction to ekoDB")}

Snippet 3:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["category", "content", "title"]
  Record: Object {"category": String("features"), "content": String("ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval."), "id": String("YpyhHwhA2jyyCsjVFGKFyJ3RXLjDF9aGS-X_ZwE3ZrmCJwDT6JxmIENEwM0zpcPswr88MeVYpHRLCD7-HRGXRA"), "title": String("Search Features")}

Execution Time: 12633ms

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_chat_advanced`
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: dBK6lbB98ML6HnMxChnPbZzmrMgyuW-ig2dTqVtvnaqvWWoLz6BD1Nw8LXQPOeC82XMJA6omGK79luhCrHbrYg

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** High-performance database product
- **Price:** $99

If you need additional information or have any other questions, feel free to ask!

✓ Second message sent
=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of ekoDB is $99. If you have any more questions or need further assistance, feel free to ask!

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: 8yrkhzLnqlIO89Qd08Norl2-AOodRTEiI951KYaSHod4QHbqpXjfbGshqPddEhOJKWf957vuyoz4_mb7w9lg1w
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_chat_sessions`
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 52rFAkddXArl9Wfh3XR69kFMTSMot2bVnbUXICbe3uD5lFV2kurJkCaWjWYN0VjzhpL5xT8JB6_P2h8zNpSyNg
=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Product Name:** ekoDB
- **Description:** A high-performance database product with AI capabilities
- **Price:** $99

If you need more information or if there are other products, feel free to ask!

✓ Message 2 sent
  Response: The price of the product ekoDB is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: k6-qAkQi-XobBeoevLOSO-dXzUb9jGxNvcgpqebWMHQrbO0m5J1DrO39RBTzIy_UgWBDwwfFHnEyTcf_smH7Dw
  Parent: 52rFAkddXArl9Wfh3XR69kFMTSMot2bVnbUXICbe3uD5lFV2kurJkCaWjWYN0VjzhpL5xT8JB6_P2h8zNpSyNg

=== Listing Sessions ===
✓ Found 4 sessions
  Session 1: k6-qAkQi-XobBeoevLOSO-dXzUb9jGxNvcgpqebWMHQrbO0m5J1DrO39RBTzIy_UgWBDwwfFHnEyTcf_smH7Dw (Untitled)
  Session 2: 52rFAkddXArl9Wfh3XR69kFMTSMot2bVnbUXICbe3uD5lFV2kurJkCaWjWYN0VjzhpL5xT8JB6_P2h8zNpSyNg (Untitled)
  Session 3: 8yrkhzLnqlIO89Qd08Norl2-AOodRTEiI951KYaSHod4QHbqpXjfbGshqPddEhOJKWf957vuyoz4_mb7w9lg1w (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: k6-qAkQi-XobBeoevLOSO-dXzUb9jGxNvcgpqebWMHQrbO0m5J1DrO39RBTzIy_UgWBDwwfFHnEyTcf_smH7Dw

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_chat_models`
✓ Client created

=== Get All Chat Models ===
Available chat models by provider:

OpenAI models (120):
  - text-embedding-ada-002
  - whisper-1
  - gpt-3.5-turbo
  - tts-1
  - gpt-3.5-turbo-16k
  - gpt-4-0613
  - gpt-4
  - davinci-002
  - babbage-002
  - gpt-3.5-turbo-instruct
  - gpt-3.5-turbo-instruct-0914
  - gpt-3.5-turbo-1106
  - tts-1-hd
  - tts-1-1106
  - tts-1-hd-1106
  - text-embedding-3-small
  - text-embedding-3-large
  - gpt-3.5-turbo-0125
  - gpt-4-turbo
  - gpt-4-turbo-2024-04-09
  - gpt-4o
  - gpt-4o-2024-05-13
  - gpt-4o-mini-2024-07-18
  - gpt-4o-mini
  - gpt-4o-2024-08-06
  - omni-moderation-latest
  - omni-moderation-2024-09-26
  - o1-2024-12-17
  - o1
  - o3-mini
  - o3-mini-2025-01-31
  - gpt-4o-2024-11-20
  - gpt-4o-mini-search-preview-2025-03-11
  - gpt-4o-mini-search-preview
  - gpt-4o-transcribe
  - gpt-4o-mini-transcribe
  - o1-pro-2025-03-19
  - o1-pro
  - gpt-4o-mini-tts
  - o3-2025-04-16
  - o4-mini-2025-04-16
  - o3
  - o4-mini
  - gpt-4.1-2025-04-14
  - gpt-4.1
  - gpt-4.1-mini-2025-04-14
  - gpt-4.1-mini
  - gpt-4.1-nano-2025-04-14
  - gpt-4.1-nano
  - gpt-image-1
  - o4-mini-deep-research
  - gpt-4o-transcribe-diarize
  - o4-mini-deep-research-2025-06-26
  - gpt-5-chat-latest
  - gpt-5-2025-08-07
  - gpt-5
  - gpt-5-mini-2025-08-07
  - gpt-5-mini
  - gpt-5-nano-2025-08-07
  - gpt-5-nano
  - gpt-audio-2025-08-28
  - gpt-realtime
  - gpt-realtime-2025-08-28
  - gpt-audio
  - gpt-5-codex
  - gpt-image-1-mini
  - gpt-5-pro-2025-10-06
  - gpt-5-pro
  - gpt-audio-mini
  - gpt-audio-mini-2025-10-06
  - gpt-5-search-api
  - gpt-realtime-mini
  - gpt-realtime-mini-2025-10-06
  - sora-2
  - sora-2-pro
  - gpt-5-search-api-2025-10-14
  - gpt-5.1-chat-latest
  - gpt-5.1-2025-11-13
  - gpt-5.1
  - gpt-5.1-codex
  - gpt-5.1-codex-mini
  - gpt-5.1-codex-max
  - gpt-image-1.5
  - gpt-5.2-2025-12-11
  - gpt-5.2
  - gpt-5.2-pro-2025-12-11
  - gpt-5.2-pro
  - gpt-5.2-chat-latest
  - gpt-4o-mini-transcribe-2025-12-15
  - gpt-4o-mini-transcribe-2025-03-20
  - gpt-4o-mini-tts-2025-03-20
  - gpt-4o-mini-tts-2025-12-15
  - gpt-realtime-mini-2025-12-15
  - gpt-audio-mini-2025-12-15
  - chatgpt-image-latest
  - gpt-5.2-codex
  - gpt-5.3-codex
  - gpt-realtime-1.5
  - gpt-audio-1.5
  - gpt-4o-search-preview
  - gpt-4o-search-preview-2025-03-11
  - gpt-5.3-chat-latest
  - gpt-5.4-2026-03-05
  - gpt-5.4-pro
  - gpt-5.4-pro-2026-03-05
  - gpt-5.4
  - gpt-5.4-nano-2026-03-17
  - gpt-5.4-nano
  - gpt-5.4-mini-2026-03-17
  - gpt-5.4-mini
  - gpt-image-2
  - gpt-image-2-2026-04-21
  - gpt-5.5
  - gpt-5.5-2026-04-23
  - gpt-5.5-pro
  - gpt-5.5-pro-2026-04-23
  - chat-latest
  - gpt-realtime-translate
  - gpt-realtime-2
  - gpt-realtime-whisper

Anthropic models (11):
  - claude-fable-5
  - claude-opus-4-8
  - claude-opus-4-7
  - claude-sonnet-4-6
  - claude-opus-4-6
  - claude-opus-4-5-20251101
  - claude-haiku-4-5-20251001
  - claude-sonnet-4-5-20250929
  - claude-opus-4-1-20250805
  - claude-opus-4-20250514
  - claude-sonnet-4-20250514

Perplexity models (5):
  - sonar
  - sonar-pro
  - sonar-deep-research
  - sonar-reasoning
  - sonar-reasoning-pro

=== Get Models for Specific Provider ===

openai models (120):
  - text-embedding-ada-002
  - whisper-1
  - gpt-3.5-turbo
  - tts-1
  - gpt-3.5-turbo-16k
  ... and 115 more

anthropic models (11):
  - claude-fable-5
  - claude-opus-4-8
  - claude-opus-4-7
  - claude-sonnet-4-6
  - claude-opus-4-6
  ... and 6 more

perplexity models (5):
  - sonar
  - sonar-pro
  - sonar-deep-research
  - sonar-reasoning
  - sonar-reasoning-pro

✓ Chat Models API example complete
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_user_functions`
✓ Client created

=== Create User Function ===
Created user function with ID: XoThKjGV0U6AOvzm1zi4L1-Oh-2BswHO3krtmJGBZgaC6isRRp4CSFYsxbtIL9i9Exjr8K6UvJx91St90bMgEA

=== Get User Function ===
Retrieved: get_active_users_rs - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 12 user functions:
  - get_verified_user: Get verified and validated user
  - fetch_slim_user: Validate and slim down user
  - get_active_users_updated: Get Active Users (Updated)
  - get_active_users_rs: Get Active Users
  - get_high_scoring_active_users: Get High Scoring Active Users
  - fetch_and_store_user: Fetch user from API and cache in KV
  - get_active_users_paginated: Get Active Users (Paginated)
  - fetch_user: Fetch user by code
  - validate_user: Check if user exists
  - swr_user: SWR pattern for user data (KV-based)
  - get_users_by_status: Get Users By Status
  - get_user_wrapper: Wrapper that calls fetch_user

=== Update User Function ===
User function updated successfully

=== Delete User Function ===
User function deleted successfully

✓ User Functions API example complete
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_convenience_methods`
=== ekoDB Convenience Methods Example ===

=== Record Builder Pattern ===
✓ Created record with fluent builder: Record({"id": String("bnGhC3BQqbBebwVrfGr2eO45g4Ee6amDjK23jGBksoEIIcT1IJxsamM-ibaD18skfQez3vyTuoHt8pcMMG3ayg")})

=== Upsert Operation ===
✓ First upsert (insert): Record({"id": String("psNHOPyI84DaIhelpuG2-45-6QKUs0mPiHKGdBkBifFcbHbwziycSoOAaPH0mnIf_tbsdBLXPAtP21929wrEWA")})
✓ Second upsert (update): Record({"id": String("awBZNMhUFuIGVaSVrzPkacQChbeZQbB97yx3kzrNhcNNgfL-8IUYH27xcJsGuuR3GxjQBrJOuLHH2ovOdG-Zhw")})

=== Find One Operation ===
✓ Found user by email: Record({"active": Object({"value": Boolean(true), "type": String("Boolean")}), "age": Object({"type": String("Integer"), "value": Integer(28)}), "name": Object({"value": String("Alice Johnson"), "type": String("String")}), "id": String("bnGhC3BQqbBebwVrfGr2eO45g4Ee6amDjK23jGBksoEIIcT1IJxsamM-ibaD18skfQez3vyTuoHt8pcMMG3ayg"), "email": Object({"type": String("String"), "value": String("alice@example.com")})})
✓ User not found (as expected)

=== Exists Check ===
✓ Record exists: true
✓ Fake record exists: false (should be false)

=== Pagination ===
✓ Inserted 25 records for pagination
✓ Page 1: 10 records (expected 10)
✓ Page 2: 10 records (expected 10)
✓ Page 3: 8 records (expected 7)

=== Cleanup ===
✓ Deleted collection

✅ All convenience methods demonstrated successfully!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/bypass_ripple_example`
=== Bypass Ripple Example ===

1. Basic insert (ripple enabled):
   Inserted with ripple: Record({"id": String("hFHZt4tPM0TeH45803Rzfha0A0MFJoyuvlNAkzY3w36w52qZhW9yaye7vgooRMpChWqjMxlZumtr2FvF56XfCw")})

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: Record({"id": String("lQJIEzF_y4rtaJoT4BzaiyEwPCubhin_XJTs5BFg09DzNduCX5CQWs3NwFld6P4f-AZjzJASjG8mKy4WjL5ULA")})

3. Update with bypass_ripple:
   Updated with bypass_ripple: Record({"price": Object({"value": Integer(150), "type": String("Integer")}), "id": String("hFHZt4tPM0TeH45803Rzfha0A0MFJoyuvlNAkzY3w36w52qZhW9yaye7vgooRMpChWqjMxlZumtr2FvF56XfCw"), "name": Object({"value": String("Product 1"), "type": String("String")})})

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: Record({"id": String("kSOW1DC68SKOEA0udG2-a3M3jMIhKvXWZbMO_EOxPJ6ouKqSFDkRwcdoUyNMz1NAcgZa2g4JPIf5l_BJ2VF2Bg")})

✅ All bypass_ripple operations completed successfully!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/projection_example`
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields (id, name, email only)
  Found 3 active users
  Fields returned: ["id", "name", "email"]

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ["name", "email", "id", "user_role", "age", "status", "created_at", "avatar_url", "bio"]

Example 3: Complex query with projection (active users, ages 18-65)
  Found 3 active users (ages 18-65)

Example 4: Query inactive users with profile fields
  Found 1 inactive users

Example 5: Compare full vs projected data
  Full query:
    - 12 fields per record
    - Fields: ["status", "id", "name", "secret_token", "password", "age", "bio", "avatar_url", "api_key", "created_at", "user_role", "email"]
  Projected query:
    - 3 fields per record
    - Fields: ["email", "name", "id"]
  Bandwidth savings: ~75% fewer fields

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_jwt_auth_flow`
✓ Client created
✓ rs_users_register saved
✓ rs_users_login saved
✓ rs_users_verify_token saved

=== Auth flow defined as pure stored functions ===
Call them like:
  POST /api/functions/rs_users_register { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/rs_users_login    { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/rs_users_verify_token { "token": "<jwt>" }

Set JWT_SECRET in ekoDB's environment_vars whitelist before invoking — the {{env.JWT_SECRET}} placeholder reads from that whitelist, NEVER from the function definition.

✓ Cleaned up demo functions
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_crypto_stages`
✓ Client created
✓ crypto_demo_hmac saved
✓ crypto_demo_aes saved
✓ crypto_demo_uuid saved
✓ crypto_demo_totp saved
✓ crypto_demo_encoding saved

All crypto-stage demos defined. Invoke any of them with:
  POST /api/functions/crypto_demo_hmac     { "payload": "hi" }
  POST /api/functions/crypto_demo_aes      { "plaintext": "secret" }
  POST /api/functions/crypto_demo_uuid
  POST /api/functions/crypto_demo_totp
  POST /api/functions/crypto_demo_encoding { "title": "Héllo World" }

✓ Cleaned up demo functions
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_concurrency_stages`
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_path_routed_function`
✓ Client created
✓ rs_route_admin → GET /api/route/users/admin
✓ rs_route_user_by_id → GET /api/route/users/:id
✓ rs_route_user_posts → GET /api/route/users/:id/posts/:post_id
✓ rs_route_org_create_member → POST /api/route/orgs/:org/members

Try them with curl:
  curl http://localhost:8080/api/route/users/admin
  curl http://localhost:8080/api/route/users/42
  curl http://localhost:8080/api/route/users/42/posts/7
  curl -X POST http://localhost:8080/api/route/orgs/acme/members \
       -H 'Content-Type: application/json' -d '{"name":"alice"}'

✓ Cleaned up demo functions
✅ [32mRust client examples complete![0m
✅ [32mAll Rust integration tests complete![0m
📦 [36mEnsuring Python example dependencies in .venv...[0m
🧪 [36mRunning Python examples (direct HTTP/WebSocket)...[0m
=== Simple CRUD Operations (Direct HTTP) ===

✓ Authentication successful

=== Insert Document ===
Inserted: {'id': 'zgfEPeNu_9kt2fGAUfnDJIALeMvMOqM4QCiTr3SOVsSS5bxGWbYcsSBy0SH-Hf-xzAwsxNd_9CwhLTCROhKQng'}

=== Find by ID ===
Found: {'id': 'zgfEPeNu_9kt2fGAUfnDJIALeMvMOqM4QCiTr3SOVsSS5bxGWbYcsSBy0SH-Hf-xzAwsxNd_9CwhLTCROhKQng', 'active': {'value': True, 'type': 'Boolean'}, 'value': {'type': 'Integer', 'value': 42}, 'name': {'value': 'Test Record', 'type': 'String'}}

=== Find with Query ===
Found documents: [{'active': {'value': True, 'type': 'Boolean'}, 'name': {'type': 'String', 'value': 'Test Record'}, 'id': 'zgfEPeNu_9kt2fGAUfnDJIALeMvMOqM4QCiTr3SOVsSS5bxGWbYcsSBy0SH-Hf-xzAwsxNd_9CwhLTCROhKQng', 'value': {'type': 'Integer', 'value': 42}}]

=== Update Document ===
Updated: {'active': {'value': True, 'type': 'Boolean'}, 'value': {'type': 'Integer', 'value': 100}, 'id': 'zgfEPeNu_9kt2fGAUfnDJIALeMvMOqM4QCiTr3SOVsSS5bxGWbYcsSBy0SH-Hf-xzAwsxNd_9CwhLTCROhKQng', 'name': {'type': 'String', 'value': 'Updated Record'}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: CKITd5EqDQjiV8hLrfhGPrj49ZBeSaAjnkyrLO7f0qy6xIDC3sWKcXH52YZBaj6wsf3LX2u00aSr-38Iom4dqA

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "type": "Success",
  "payload": {
    "data": [
      {
        "value": {
          "type": "Integer",
          "value": 42
        },
        "id": "Rx_EhUtGO6iii80P81nw7Bu_teSO3QUJk0QQ5tqdYHt_E7rl36XSAj6fL0Fa5ZVfaAmRGe0SxQ4uDM2G7ETLTQ",
        "name": {
          "value": "WebSocket Test Record",
          "type": "String"
        },
        "active": {
          "value": true,
          "type": "Boolean"
        }
      },
      {
        "name": {
          "type": "String",
          "value": "WebSocket Test Record"
        },
        "id": "CKITd5EqDQjiV8hLrfhGPrj49ZBeSaAjnkyrLO7f0qy6xIDC3sWKcXH52YZBaj6wsf3LX2u00aSr-38Iom4dqA",
        "value": {
          "type": "Integer",
          "value": 42
        },
        "active": {
          "type": "Boolean",
          "value": true
        }
      }
    ]
  },
  "messageId": "342173890"
}
✓ Retrieved 2 record(s) via WebSocket

✓ WebSocket example completed successfully
🚀 ekoDB Functions Example (Python/HTTP)

✓ Authentication successful

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Function saved: 0eQVQHX5p0gAOYZtmrN-JGd7fAr1uwziyLTR15ZF_5HtrR9u37Fe9VNRwJwvJK5Pl2YBvmxc9HedLcXYfvWxlQ
📊 Found 15 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

ℹ️  Function 'get_active_users_paginated' already existed — updated instead
✅ Function saved: get_active_users_paginated
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: bL6K9EHpYrwTBWJB3BvvaGnfkqx-IPiFcZm-2dwpKVhKLswBe6oMLcBuQ4AoI9JdzQnl5dO_qOzjJNo_4pZbMA
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {'count': {'type': 'Integer', 'value': 15}, 'avg_score': {'type': 'Float', 'value': 50.0}, 'status': {'value': 'inactive', 'type': 'String'}, 'max_score': {'value': 90, 'type': 'Integer'}}
   {'count': {'value': 15, 'type': 'Integer'}, 'avg_score': {'value': 60.0, 'type': 'Float'}, 'status': {'type': 'String', 'value': 'active'}, 'max_score': {'value': 100, 'type': 'Integer'}}

📝 Example 4: Function Management

📋 Total scripts: 13
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
cache:product:1: {'name': 'Product 1', 'price': 29.99}
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
Collection created with first record: 2mUQJzt11EXB428cFm9cxH2dYfn2il41n-61uIdOV4-M7LfrY6nA1ozKwew5cfn1jpreTFa8PITpFZvElP4Lbw

=== List Collections ===
Total collections: 14
Sample collections: ['chat_messages__ek0_testing', 'chat_configurations__ek0_testing', 'batch_users', 'functions__ek0_testing', 'agent_function_versions__ek0_testing']

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
Created Alice: $1000 - ID: WrJ4t11f4emHCFqhVaN5F7lSnfjnA-kkZI7lOYE0Jl-9qEYO3PqUOwxKVQJWlXMY-F-splMxkJWaloB8TCDU-A
Created Bob: $500 - ID: 9Ujd3xi0LrSgVDZe_hzrDLanesDZyjQkoOFHmVwAxSkuCjU1DNURSa9laA-NcrcEYHQ6qq9Y72ki3FSvzQhjtw

=== Example 1: Begin Transaction ===
Transaction ID: e03a59b0-2d08-4849-9361-c8c7bd0f7f50

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
New transaction: 375995d4-9d05-4bf0-8391-cb54d97efcda
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'type': 'Integer', 'value': 700}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: vUWZ1GZvGwvMUzTnXbpGyF5h8YsZIHoYC35VMimjjMF5DJ_mwyNQk-NiPUntNF9ZQKdTOzGPrCz7FIWHpQpcYg
Created Bob: $500 - ID: SlH_r8SwkFCXgOBLY11oXb_g59feJSj9zVOBPvHdBklqdR8okq7iPHJvlA_n5uJ9HoA3OBwjhwr9G5aZGj89IQ

=== Example 1: Begin Transaction ===
Transaction ID: 2cfeebca-9aeb-4422-8975-eb2f77277d2d

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
Created Alice: $1000 - ID: Jx2JxeOBPUaa1zmSHvlxOh9jqcHCxuPjdYyye2obvN1o0AJjjagx-riE0pO-HxmzHHI52b-PhI_KXuG8sQ1Wuw
Created Bob: $500 - ID: XUNIxVvwt34uQg2QktUvwDHp-27Viww3uaGkeQxJ4CozCZHv32QlHFVjL2IeQDQ5NvZVJS4q9wb6L7Pwynse9Q

=== Example 1: Begin Transaction ===
Transaction ID: eaeb9662-1702-4c76-b99c-334aa7d3d19b

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
New transaction: a734c073-cf94-425e-8dd1-85aacba890e4
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'type': 'Integer', 'value': 700}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: iq0jp8xz1mNedagSj55bJLxQz80tVY8_-3ey_Dm8ZBK5HOKzfDjfrxBC7v9xbp-58Xnn4taSHbBOXtoI9VlykA
Created Bob: $500 - ID: 7ue9ISo2m4dY-iaq4XgaYMpRdScN4dKv2JZ8d0a9_LJAlbjWCnAKPddGQowSNYXmBM3J6QP9ASMngWVCQkWTrg

=== Example 1: Begin Transaction ===
Transaction ID: 1efea7e1-dada-464b-8c00-e62848746778

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


🔄 Switching to DURABLE mode...
   ✓ Config updated: durable_operations=true


============================================================
🔥 TEST 3: Durable Mode (durable=true)
============================================================

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: Mr8BR45rh934GJDNh_eSU77AVmrVNu5zyMHaq5EV77KezcxfaeGcXfydWHCUSS8kU3HZ2UKTE5mTpX_Ej0UDGQ
Created Bob: $500 - ID: YpC3YO0V1XBF-p_2rQ1IzyemSiIDhIEME3HNDspXQXZHwLvTE9O5hl-oxjditU3rX3kSf_yKqmvODYysrg98PA

=== Example 1: Begin Transaction ===
Transaction ID: fa7aaac6-104f-4dac-9ace-33620a94079d

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
New transaction: caf5878c-cb95-4fa3-805c-27a2a4857748
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: ${'value': 700, 'type': 'Integer'}

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 8J_TiEPjNRwQSr8ECEH-4tl16RtJTXX6ytLieO6YwX4cBO_XFYjAa51E0Szgr2ryhezA_eL0n4GsrBq7DmqCSQ
Created Bob: $500 - ID: brIxO-8Gyf2TpSRMfrQc-NGWr-Sv5SCrQv1Xypc4zI7d1Yus5i6SWC12wghc8ZSERWujO6GPC-zFvQEa-QyRlw

=== Example 1: Begin Transaction ===
Transaction ID: b48844d9-ea84-4981-84f4-75843df4e696

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
   ✅ Function saved: UPuYkYIe_72ROuAGxqJQvJ9rCApx114XROPgFs3YSbOAN5Nfq8LbdreaFrew0M8oSZVijpedtrFEatW5bEcqBA

2️⃣ Calling function (Insert + Verify)...
   ✅ function executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: _fEO6annlStD_I_A5dU6SnSVlJ8dQmgMQGZBlk06z4CiQdjCtItSskz3Sdn48s7E3onmvZfFrbmh7QqWnQPNuA
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}
   📋 Email: {'type': 'String', 'value': 'alice@example.com'}
   📋 Status: {'value': 'pending', 'type': 'String'}
   📋 Credits: {'type': 'Integer', 'value': 0}

============================================================
📝 function 2: Query + Update + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: 1CS8cIFWGVglnDEw_c42Fs-VfP2BVzEuBwEFOXYVq7h-TKTwpSYYprsUvjayDikCWUemPeCPFyt0SADuFGloFw

2️⃣ Calling function (Query + Update + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Status updated to: {'type': 'String', 'value': 'active'}
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}

============================================================
📝 function 3: Query + Update Credits + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: Uw7Qm7tN5IdQ1x06D8uilVz18qPKvRFwA-GS5VJ60thUUg3dy_zki585Kpo6ikrt1B8d_9wURMJTrCoDCIKuyQ

2️⃣ Calling function (Query + Update Credits + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {'type': 'Integer', 'value': 0}
   📋 Status: {'type': 'String', 'value': 'active'}
   📋 Name: {'value': 'Alice Smith', 'type': 'String'}

============================================================
📝 function 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: OMFkJP0NVQmL6jVqqCRnmlkQWLifZov89_X3yU_9QxlwxEmAuU0vVtgZ8zLwYe0kKHBLKoloh1gRrsICflOb7g

2️⃣ Calling function (Query + Delete + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: UPuYkYIe_72ROuAGxqJQ...
   ✅ Deleted script: 1CS8cIFWGVglnDEw_c42...
   ✅ Deleted script: Uw7Qm7tN5IdQ1x06D8ui...
   ✅ Deleted script: OMFkJP0NVQmL6jVqqCRn...
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
  Output: Document ID = LsvZ9weOe-G19U0i3c4fYU66lApNRTyumJzlQFWHcVhizNkVB7S5sfv7No5jxSy4SGhC0Wwc12mgIUp9fPXLXQ
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: find_by_id(LsvZ9weOe-G19U0i3c4fYU66lApNRTyumJzlQFWHcVhizNkVB7S5sfv7No5jxSy4SGhC0Wwc12mgIUp9fPXLXQ)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: find_by_id(LsvZ9weOe-G19U0i3c4fYU66lApNRTyumJzlQFWHcVhizNkVB7S5sfv7No5jxSy4SGhC0Wwc12mgIUp9fPXLXQ)
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
  Output: Document ID = nI_5692HfRSN5kyvFMMmAfm03insDE8rGNUYb7Kmr3DZ0qWQ0YJLgfANLQU43g2mNxm1dq_4HWX8qU16_g3CXw
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: find_by_id(nI_5692HfRSN5kyvFMMmAfm03insDE8rGNUYb7Kmr3DZ0qWQ0YJLgfANLQU43g2mNxm1dq_4HWX8qU16_g3CXw)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: find_by_id(nI_5692HfRSN5kyvFMMmAfm03insDE8rGNUYb7Kmr3DZ0qWQ0YJLgfANLQU43g2mNxm1dq_4HWX8qU16_g3CXw)
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
🔧 [36mEnsuring maturin is available in .venv...[0m
🔨 [36mBuilding wheel...[0m
🍹 Building a mixed python/rust project
🐍 Found CPython 3.14 at /opt/homebrew/opt/python@3.14/bin/python3.14
🔗 Found pyo3 bindings with abi3 support
💻 Using `MACOSX_DEPLOYMENT_TARGET=11.0` for aarch64-apple-darwin by default
    Finished `release` profile [optimized] target(s) in 0.15s
📦 Built wheel for abi3 Python ≥ 3.8 to ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.21.0-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel into .venv...[0m
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.21.0-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.21.0
    Uninstalling ekodb_client-0.21.0:
      Successfully uninstalled ekodb_client-0.21.0
Successfully installed ekodb-client-0.21.0
✅ [32mPython client package built and installed![0m
🧪 [36mRunning Python client library examples...[0m
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {'id': 'V8zcOqGm2CRY0VTVyCeuoYN3g4CpjIKQo9zpeXgE04YVrmycuxgyzh1jBZnunOU0yN2KtiBQ16mnKnh3ghP-hA'}

=== Find by ID ===
Found: {'embedding': {'type': 'Array', 'value': [0.1, 0.2, 0.3, 0.4, 0.5]}, 'id': 'V8zcOqGm2CRY0VTVyCeuoYN3g4CpjIKQo9zpeXgE04YVrmycuxgyzh1jBZnunOU0yN2KtiBQ16mnKnh3ghP-hA', 'price': {'value': 99.99, 'type': 'Float'}, 'categories': {'value': ['electronics', 'computers'], 'type': 'Array'}, 'data': {'type': 'String', 'value': 'aGVsbG8gd29ybGQ='}, 'tags': {'value': ['tag1', 'tag2', 'tag3'], 'type': 'Array'}, 'value': {'type': 'Integer', 'value': 42}, 'created_at': {'value': '2026-06-09T21:36:03.620221', 'type': 'String'}, 'user_id': {'type': 'String', 'value': '550e8400-e29b-41d4-a716-446655440000'}, 'metadata': {'type': 'Object', 'value': {'key': 'value', 'nested': {'deep': True}}}, 'active': {'type': 'Boolean', 'value': True}, 'name': {'value': 'Test Record', 'type': 'String'}}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): True
  price (Decimal): 99.99
  created_at (DateTime): 2026-06-09 21:36:03.620221
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): ['tag1', 'tag2', 'tag3']
  metadata (Object): {'key': 'value', 'nested': {'deep': True}}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): ['electronics', 'computers']
  data (Bytes): 11 bytes
Plain record: {'embedding': [0.1, 0.2, 0.3, 0.4, 0.5], 'id': 'V8zcOqGm2CRY0VTVyCeuoYN3g4CpjIKQo9zpeXgE04YVrmycuxgyzh1jBZnunOU0yN2KtiBQ16mnKnh3ghP-hA', 'price': 99.99, 'categories': ['electronics', 'computers'], 'data': 'aGVsbG8gd29ybGQ=', 'tags': ['tag1', 'tag2', 'tag3'], 'value': 42, 'created_at': '2026-06-09T21:36:03.620221', 'user_id': '550e8400-e29b-41d4-a716-446655440000', 'metadata': {'key': 'value', 'nested': {'deep': True}}, 'active': True, 'name': 'Test Record'}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'id': 'V8zcOqGm2CRY0VTVyCeuoYN3g4CpjIKQo9zpeXgE04YVrmycuxgyzh1jBZnunOU0yN2KtiBQ16mnKnh3ghP-hA', 'categories': {'value': ['electronics', 'computers'], 'type': 'Array'}, 'active': {'value': True, 'type': 'Boolean'}, 'embedding': {'value': [0.1, 0.2, 0.3, 0.4, 0.5], 'type': 'Array'}, 'data': {'type': 'String', 'value': 'aGVsbG8gd29ybGQ='}, 'metadata': {'value': {'key': 'value', 'nested': {'deep': True}}, 'type': 'Object'}, 'user_id': {'value': '550e8400-e29b-41d4-a716-446655440000', 'type': 'String'}, 'price': {'value': 99.99, 'type': 'Float'}, 'tags': {'type': 'Array', 'value': ['tag1', 'tag2', 'tag3']}, 'name': {'type': 'String', 'value': 'Updated Record'}, 'value': {'value': 100, 'type': 'Integer'}, 'created_at': {'type': 'String', 'value': '2026-06-09T21:36:03.620221'}}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: XM0HMB191d3mffl-CYTLSWX9mozzLYQowkJFitczIPF_z9CZOKfqtId4XgL_4rSingCzzkpSGPqGFwUUiWPT4g

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
Collection created with first record: "mWznnXMMzikKjjGqihLYHT7rbC4jdK9lH8Pdd_7D0Y86yDeQUjLqS_7i9LXkzlvQnk-Jv-IkWRRZljx7xo5x3g"

=== List Collections ===
Total collections: 13
Sample collections: ['chat_messages__ek0_testing', 'chat_configurations__ek0_testing', 'batch_users', 'functions__ek0_testing', 'client_collection_management_python']

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
Created Alice: $1000 - ID: AhmQ7GomgtxpHjvjXHsBeve9tIN_oB3cOsBO1OsYS-yq1lNS6vvXS4XgjigSbSZuubGrbpKbLnV-pBc7A_Lrpw
Created Bob: $500 - ID: oLmjwEfnNJ1n8ywZUO2qjiSBHoBwn7zqcfz4tMpHo2eBg02ruEbhtSlHQQWVw_9eI2an_dKVTUYmO-9oGkTSSg

=== Example 1: Begin Transaction ===
Transaction ID: d96cbc04-a034-4107-9432-396de3017d0a

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: bf54e3f3-0913-4422-a7b0-3270c2cb329f
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
  1. Score: 25.740, Matched: name, email, email.value, name.value
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title.value, bio, title, bio.value
  2. Score: 26.400, Matched: title, title.value, bio, bio.value
  3. Score: 26.400, Matched: bio, bio.value, title, title.value
  4. Score: 26.400, Matched: bio, title.value, bio.value, title

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title, bio, title.value, bio.value
  2. Score: 39.600, Matched: bio, title.value, title, bio.value
  3. Score: 39.600, Matched: bio, title.value, title, bio.value
  4. Score: 39.600, Matched: title.value, bio, title, bio.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 2 results (matches: run, running, runs)
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio.value, bio

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.753
  2. Score: 0.746
  3. Score: 0.737

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.377, Matched: content.value, title.value, content, title
  2. Score: 1.373, Matched: title, content, content.value, title.value
  3. Score: 0.369, Matched: 

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
✓ Inserted document: 2JxTKapkpk8tqsqvZsaC_mb7pIoWuZ9wXTXC73gODllbjBdkPNL_cXog860sNi1KI84ODXM6_kYK-QUJQIOpmg

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: TRgJ5MkNCodjO5yxy8Vq6pRrIyLtohiPzXsSjt-pmRNlrtxxJCQA-wBJYNlVhS3fq5uIbVsskmI9R0FthzQYNg

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
✓ Inserted document with TTL: kvsSYV3f2CFe2z1_BMWHUwagL0YnOLH6xOKPyCb8AKmUlB9yFqMudYctOcC1-f4wejFViyXeomtf-DYFIEg8NQ

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
✓ Edge cache script created: gBkD4h5ltfxE46fx2lOp-DP8qZ_grFoNgmVzXgAgBBtcyUm7QAZuRe2wc2kASgcRfnvglodOUiz-QGHzUff8sw

Call 1: Cache miss (fetches from API)
Response time: 507ms
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "current": {
            "interval": 900,
            "temperature_2m": 21.5,
            "time": "2026-06-10T01:30"
          },
          "current_units": {
            "interval": "seconds",
            "temperature_2m": "\u00b0C",
            "time": "iso8601"
          },
          "elevation": 32.0,
          "generationtime_ms": 0.02396106719970703,
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
Response time: 40ms (12.8x faster!)
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "current": {
            "interval": 900,
            "temperature_2m": 21.5,
            "time": "2026-06-10T01:30"
          },
          "current_units": {
            "interval": "seconds",
            "temperature_2m": "\u00b0C",
            "time": "iso8601"
          },
          "elevation": 32.0,
          "generationtime_ms": 0.02396106719970703,
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

🚀 ekoDB Functions Example (Python)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

ℹ️  Function 'get_active_users' already existed — updated instead
✅ Function saved: get_active_users
📊 Found 10 active users

📝 Example 2: Parameterized Function

ℹ️  Function 'get_users_by_status' already existed — updated instead
✅ Function saved: get_users_by_status
📊 Found 10 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: RHZHnPpU9rcwGs589W-T_KDqLFnA8AALkF1QmSZNUlphsMOPYxKDeyAw194vqe-gsHrCfU8XiUc6lhgV0USNPA
📊 Statistics: 2 groups
   {'avg_score': {'type': 'Float', 'value': 60.0}, 'count': {'type': 'Integer', 'value': 5}, 'status': {'type': 'String', 'value': 'active'}}

   {'avg_score': {'type': 'Float', 'value': 50.0}, 'count': {'type': 'Integer', 'value': 5}, 'status': {'type': 'String', 'value': 'inactive'}}

📝 Example 4: Function Management

📋 Total functions: 14
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

ℹ️  Function 'fetch_user' already existed — updated instead
✅ Saved reusable function: fetch_user
ℹ️  Function 'get_user_wrapper' already existed — updated instead
✅ Saved composed function: get_user_wrapper (calls fetch_user + projects fields)

📊 Result from composed function:
   Records: 1
   Name: {"type": "String", "value": "User 1"}
   Department: {"type": "String", "value": "engineering"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

ℹ️  Function 'fetch_and_store_user' already existed — updated instead
✅ Saved reusable function: fetch_and_store_user (uses KV)
ℹ️  Function 'swr_user' already existed — updated instead
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 39.1ms
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
   ⏱️  Duration: 39.4ms
   📊 Records: 1
   📦 Data: {
      "value": {
            "type": "Object",
            "value": {
                  "address": {
                        "city": "Gwenborough",
                        "geo": {
                 ...
   🚀 Cache speedup: 1.0x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

ℹ️  Function 'validate_user' already existed — updated instead
✅ Level 1 function: validate_user
ℹ️  Function 'fetch_slim_user' already existed — updated instead
✅ Level 2 function: fetch_slim_user (calls validate_user)
ℹ️  Function 'get_verified_user' already existed — updated instead
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

✅ Function saved: cmzH1FYXUAcTZh5J1Hu4Gun_fqvn28G0YDNuX9MZiR7_J8T0YEkQQptyVs9z4C3_Ei3-QwSpIY1bUSJ_S2OcUA
📊 Found 2 product groups
   {'avg_price': {'type': 'Float', 'value': 474.0}, 'category': {'type': 'String', 'value': 'Furniture'}, 'count': {'type': 'Integer', 'value': 2}}
   {'avg_price': {'type': 'Float', 'value': 575.6666666666666}, 'category': {'type': 'String', 'value': 'Electronics'}, 'count': {'type': 'Integer', 'value': 3}}
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

✅ Inserted order: WZgPnGx5JqCsEoD22Ktl7w2zZGfDbYZYmhcGst87NLtP4gLnHK1vkiBhSMHCq_zRG33EzsH8mz745Ts3ZSauvA
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

✅ Inserted order: thLcXrOKgZ9rvujQ9dO2-r6IDptL5jqq_PDi_y0USopJ5lmnzz6Dw8b__7OcqU9HZ6nU79zRLJOg1nFidDkj8g
✅ Cached order status
📊 Quick status lookup: {'value': '{"status":"processing","updated_at":"2026-06-10T01:36:06.490902+00:00"}'}

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
✓ Created SWR script: fetch_api_user_py (__T3qhXcORL5cPVWOlAkeLPfz-FYYOELiP-f0ILV9QtrDn1_afVxq6v6AFT3PVW8Gh0lscNuU-Uo0SHXtR8pdQ)

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
Response time: 40ms (served from cache)
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
✓ Created native SWR script: github_user_native (2ldy48Dtqx7I8iZfF_V79vpi4tT2OkgGWW_MdnvicnJYhfWMHbA5pxYaj8aKZAwTrO2GEq6CHIdoLcRgsNWIxw)

First call (cache miss - will fetch from GitHub API):
  Response time: 123ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 20ms
  Speedup: 6.1x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (HKCO2df8XbKdJIgwRJQ4wsE1YkbjCuRlrU9OaJDg_LLj5iK9fpPRwPxVIlOauuGHqvgnbWev2naG37F50sQCIA)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (TNL29_B-xn4OdA1ti9MIH4tOSxV4Eg4V2vf1W4QNLBJfS-gb_Q4HBlRTqvZuLn9wNamjVfBHs3o1fduMJFXaOg)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (dcIIU1cQbejkuFiQ3Ob0bGOqnqEUoSgm0gQlGOxxfqz-s1AgzBd2E1OfOJSBPFjm40pU1lVKsH5Kth3GMKLNNg)
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

1. **Efficient Similarity Search**: They enable fast retrieval of similar items based on vector embeddings, which is essential for applications like recommendation systems and image search.

2. **Scalability**: Designed to handle large volumes of high-dimensional data efficiently, making it suitable for big data applications.

3. **Flexibility**: Support various data types, including text, images, and audio, enabling multi-modal search capabilities.

4. **Real-time Processing**: Allow for real-time querying and updates, essential for applications requiring quick responses.

5. **Enhanced AI/ML Integration**: Seamlessly integrate with machine learning models, improving performance in tasks such as NLP and computer vision.

6. **Dimensionality Reduction**: Offer techniques like PCA or t-SNE for managing high-dimensional data, simplifying analysis and visualization.

7. **Advanced Indexing**: Utilize specialized indexes (e.g., HNSW, Annoy) that optimize search time and accuracy for nearest neighbor queries.

8. **Community and Ecosystem**: Growing libraries and tools specifically designed for vector databases support easier implementation and integration.

Overall, vector databases are well-suited for modern AI and ML applications, enhancing the efficiency and effectiveness of data retrieval and processing tasks.
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
   {'avg_score': {'type': 'Float', 'value': 20.0}, 'count': {'type': 'Integer', 'value': 3}, 'role': {'type': 'String', 'value': 'admin'}}
   {'avg_score': {'type': 'Float', 'value': 70.0}, 'count': {'type': 'Integer', 'value': 7}, 'role': {'type': 'String', 'value': 'user'}}
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
   1. Natural Language Processing (AI)
   2. Getting Started with ekoDB (Database)
   3. Database Design Principles (Database)
   4. Introduction to Machine Learning (AI)
   5. Vector Databases Explained (Database)
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
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: _xiMGVjJpu7r4vWBMXRFRHvuAvR2EBTWY8KdSC_AcOwC4PwMb97zKEra0DwnBDRctEAN6RSpdCdwj7-4hPnZTA

=== Sending Chat Message ===
Message ID: VAZ-rmadoSuXCmpnCDXvcgmbrbCcOF0DYfUMXxs2gyTClSo60FDU0_et1CZT6_2uaqTTXAmGu9iUienH8lR1-A

=== AI Response ===
Here are the products available along with their prices:

1. **ekoDB Pro**
   - Description: Enterprise edition product with advanced features
   - Price: $299

2. **ekoDB**
   - Description: A high-performance database product with AI capabilities
   - Price: $99

3. **ekoDB Cloud**
   - Description: Fully managed cloud database service product
   - Price: $499

Execution Time: 4121ms

=== Token Usage ===
Prompt tokens: 3468
Completion tokens: 91
Total tokens: 3559

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: M-WFNJ8BJyekl1XBRxP791cMq3Ub44NGIECx2sB29tKrhjNNlYmSwRWk7nxgsgXLCn_zmg22-lKlX6J053CSwQ

=== Sending Initial Message ===
✓ Message sent
  Response: It seems there are currently no products available in the database. If you have any specific products in mind or if you would like to know about a particular category, please let me know!

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['created_at', 'chat_id', 'context_snippets', 'role', 'llm_model', 'token_usage', 'content', 'updated_at', 'llm_provider', 'id'])
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
✓ Created second session: IMk5O_Jl94E_vRIrr-4t-mn7L9CpF2IyC5Z0Ef83_ZaCDyZSCJ89KDGpGXX2_R1t3gfRiIBRPdi5_MZmmySl_w
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
✓ Created session: arDUIy0JvWFMhNR6jVL2eXTmmA8sAWo7vbbVrTChdH5D5YqwuC8lHzluHFaE8OIBJfBDRPAPZbyxjVy30ZKX7A

=== Sending Messages ===
✓ Message 1 sent
  Response: It appears that there are currently no products available in your database. If you need assistance with adding products or checking for different criteria, let me know!

✓ Message 2 sent
  Response: The price of the product "ekoDB" is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: J_sId2cl9mGCkZfwKN-G2lEOu533GZmmwNv1Xxm075RayV1Ztqni2kP9zfbRlBQGovqLkpofO_-eHB3ciUk0sQ
  Parent: arDUIy0JvWFMhNR6jVL2eXTmmA8sAWo7vbbVrTChdH5D5YqwuC8lHzluHFaE8OIBJfBDRPAPZbyxjVy30ZKX7A

=== Listing Sessions ===
✓ Found 7 sessions
  Session 1: J_sId2cl9mGCkZfwKN-G2lEOu533GZmmwNv1Xxm075RayV1Ztqni2kP9zfbRlBQGovqLkpofO_-eHB3ciUk0sQ (Untitled)
  Session 2: arDUIy0JvWFMhNR6jVL2eXTmmA8sAWo7vbbVrTChdH5D5YqwuC8lHzluHFaE8OIBJfBDRPAPZbyxjVy30ZKX7A (Untitled)
  Session 3: IMk5O_Jl94E_vRIrr-4t-mn7L9CpF2IyC5Z0Ef83_ZaCDyZSCJ89KDGpGXX2_R1t3gfRiIBRPdi5_MZmmySl_w (Untitled)
  Session 4: _xiMGVjJpu7r4vWBMXRFRHvuAvR2EBTWY8KdSC_AcOwC4PwMb97zKEra0DwnBDRctEAN6RSpdCdwj7-4hPnZTA (Untitled)
  Session 5: 52rFAkddXArl9Wfh3XR69kFMTSMot2bVnbUXICbe3uD5lFV2kurJkCaWjWYN0VjzhpL5xT8JB6_P2h8zNpSyNg (Untitled)
  Session 6: 8yrkhzLnqlIO89Qd08Norl2-AOodRTEiI951KYaSHod4QHbqpXjfbGshqPddEhOJKWf957vuyoz4_mb7w9lg1w (Untitled)
  Session 7: 1eyYvkAa7VtrEl2EJ-CuUCoLL3yBazPNyI9Bmx9_-oE21yh6sdcaRibFYofsJpsdK35PC6_4AQg83FBJuliVSw (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: J_sId2cl9mGCkZfwKN-G2lEOu533GZmmwNv1Xxm075RayV1Ztqni2kP9zfbRlBQGovqLkpofO_-eHB3ciUk0sQ

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Dict Creation ===
✓ Created record with plain dict: {'id': 'WSuVign3mR7TkL7XtSjS-lRwaJmQuIWVwr_WuSrGx588pzdnbAPjyafTJGLw_gKtObj-8GsgfAu1n5aXVBC8uQ'}

=== Upsert Operation ===
✓ Upsert (update existing record): WSuVign3mR7TkL7XtSjS-lRwaJmQuIWVwr_WuSrGx588pzdnbAPjyafTJGLw_gKtObj-8GsgfAu1n5aXVBC8uQ
✓ Inserted second record: xq10bkdnJI0c7DuOyYsqABdFSQh0YVNHVg9mXA515W4xMUvRv9AG7KNYg5gUC_jZpB3FRgEHkpUdgvNmdFPEqg
✓ Upsert (update second record): xq10bkdnJI0c7DuOyYsqABdFSQh0YVNHVg9mXA515W4xMUvRv9AG7KNYg5gUC_jZpB3FRgEHkpUdgvNmdFPEqg

=== Find One Operation ===
✓ Found user by email: {'id': 'WSuVign3mR7TkL7XtSjS-lRwaJmQuIWVwr_WuSrGx588pzdnbAPjyafTJGLw_gKtObj-8GsgfAu1n5aXVBC8uQ', 'name': {'type': 'String', 'value': 'Alice Johnson'}, 'email': {'type': 'String', 'value': 'alice.j@newdomain.com'}, 'active': {'value': True, 'type': 'Boolean'}, 'age': {'type': 'Integer', 'value': 29}}
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
   Inserted with ripple: {'id': '9gjB7JOttR11slHf8i0_EV-fe8zf1JR_xwpCaTCMfeEIAJ2vgHcqXozwOWz1yxPasUQS8amb5xogik2M5cQcXQ'}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {'id': 'ud561Ngr4L85a86zSH3NwZZqSQngC9YLNbHUju_RooXiXZVrpbJtr2hwCXtV5ZxU_DQQjD1NHUE6OdASbDtg7Q'}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {'price': {'type': 'Integer', 'value': 150}, 'id': '9gjB7JOttR11slHf8i0_EV-fe8zf1JR_xwpCaTCMfeEIAJ2vgHcqXozwOWz1yxPasUQS8amb5xogik2M5cQcXQ', 'name': {'type': 'String', 'value': 'Product 1'}}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {'id': '9gjB7JOttR11slHf8i0_EV-fe8zf1JR_xwpCaTCMfeEIAJ2vgHcqXozwOWz1yxPasUQS8amb5xogik2M5cQcXQ', 'name': {'value': 'Upsert Product', 'type': 'String'}, 'price': {'value': 500, 'type': 'Integer'}}

✅ All bypass_ripple operations completed successfully!
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields (id, name, email only)
  Found 3 active users
  Fields returned: ['id', 'name', 'email']
  First user: {'type': 'String', 'value': 'Dave Brown'} <{'value': 'dave@example.com', 'type': 'String'}>

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ['name', 'email', 'id', 'age', 'status', 'created_at', 'bio', 'user_role', 'avatar_url']

Example 3: Complex query with projection (active users, ages 18-65)
  Found 3 active users (ages 18-65)
    - {'value': 'Dave Brown', 'type': 'String'} (age {'type': 'Integer', 'value': 45})
    - {'type': 'String', 'value': 'Alice Johnson'} (age {'value': 30, 'type': 'Integer'})
    - {'value': 'Bob Smith', 'type': 'String'} (age {'value': 25, 'type': 'Integer'})

Example 4: Query inactive users with profile fields
  Found 1 inactive users
    - {'value': 'Carol White', 'type': 'String'}: {'type': 'String', 'value': 'Manager'}

Example 5: Compare full vs projected data
  Full query:
    - 12 fields per record
    - Fields: ['password', 'id', 'name', 'secret_token', 'bio', 'age', 'user_role', 'status', 'created_at', 'avatar_url', 'api_key', 'email']
  Projected query:
    - 3 fields per record
    - Fields: ['name', 'email', 'id']
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
OpenAI models: ['text-embedding-ada-002', 'whisper-1', 'gpt-3.5-turbo', 'tts-1', 'gpt-3.5-turbo-16k', 'gpt-4-0613', 'gpt-4', 'davinci-002', 'babbage-002', 'gpt-3.5-turbo-instruct', 'gpt-3.5-turbo-instruct-0914', 'gpt-3.5-turbo-1106', 'tts-1-hd', 'tts-1-1106', 'tts-1-hd-1106', 'text-embedding-3-small', 'text-embedding-3-large', 'gpt-3.5-turbo-0125', 'gpt-4-turbo', 'gpt-4-turbo-2024-04-09', 'gpt-4o', 'gpt-4o-2024-05-13', 'gpt-4o-mini-2024-07-18', 'gpt-4o-mini', 'gpt-4o-2024-08-06', 'omni-moderation-latest', 'omni-moderation-2024-09-26', 'o1-2024-12-17', 'o1', 'o3-mini', 'o3-mini-2025-01-31', 'gpt-4o-2024-11-20', 'gpt-4o-mini-search-preview-2025-03-11', 'gpt-4o-mini-search-preview', 'gpt-4o-transcribe', 'gpt-4o-mini-transcribe', 'o1-pro-2025-03-19', 'o1-pro', 'gpt-4o-mini-tts', 'o3-2025-04-16', 'o4-mini-2025-04-16', 'o3', 'o4-mini', 'gpt-4.1-2025-04-14', 'gpt-4.1', 'gpt-4.1-mini-2025-04-14', 'gpt-4.1-mini', 'gpt-4.1-nano-2025-04-14', 'gpt-4.1-nano', 'gpt-image-1', 'o4-mini-deep-research', 'gpt-4o-transcribe-diarize', 'o4-mini-deep-research-2025-06-26', 'gpt-5-chat-latest', 'gpt-5-2025-08-07', 'gpt-5', 'gpt-5-mini-2025-08-07', 'gpt-5-mini', 'gpt-5-nano-2025-08-07', 'gpt-5-nano', 'gpt-audio-2025-08-28', 'gpt-realtime', 'gpt-realtime-2025-08-28', 'gpt-audio', 'gpt-5-codex', 'gpt-image-1-mini', 'gpt-5-pro-2025-10-06', 'gpt-5-pro', 'gpt-audio-mini', 'gpt-audio-mini-2025-10-06', 'gpt-5-search-api', 'gpt-realtime-mini', 'gpt-realtime-mini-2025-10-06', 'sora-2', 'sora-2-pro', 'gpt-5-search-api-2025-10-14', 'gpt-5.1-chat-latest', 'gpt-5.1-2025-11-13', 'gpt-5.1', 'gpt-5.1-codex', 'gpt-5.1-codex-mini', 'gpt-5.1-codex-max', 'gpt-image-1.5', 'gpt-5.2-2025-12-11', 'gpt-5.2', 'gpt-5.2-pro-2025-12-11', 'gpt-5.2-pro', 'gpt-5.2-chat-latest', 'gpt-4o-mini-transcribe-2025-12-15', 'gpt-4o-mini-transcribe-2025-03-20', 'gpt-4o-mini-tts-2025-03-20', 'gpt-4o-mini-tts-2025-12-15', 'gpt-realtime-mini-2025-12-15', 'gpt-audio-mini-2025-12-15', 'chatgpt-image-latest', 'gpt-5.2-codex', 'gpt-5.3-codex', 'gpt-realtime-1.5', 'gpt-audio-1.5', 'gpt-4o-search-preview', 'gpt-4o-search-preview-2025-03-11', 'gpt-5.3-chat-latest', 'gpt-5.4-2026-03-05', 'gpt-5.4-pro', 'gpt-5.4-pro-2026-03-05', 'gpt-5.4', 'gpt-5.4-nano-2026-03-17', 'gpt-5.4-nano', 'gpt-5.4-mini-2026-03-17', 'gpt-5.4-mini', 'gpt-image-2', 'gpt-image-2-2026-04-21', 'gpt-5.5', 'gpt-5.5-2026-04-23', 'gpt-5.5-pro', 'gpt-5.5-pro-2026-04-23', 'chat-latest', 'gpt-realtime-translate', 'gpt-realtime-2', 'gpt-realtime-whisper']
Anthropic models: ['claude-fable-5', 'claude-opus-4-8', 'claude-opus-4-7', 'claude-sonnet-4-6', 'claude-opus-4-6', 'claude-opus-4-5-20251101', 'claude-haiku-4-5-20251001', 'claude-sonnet-4-5-20250929', 'claude-opus-4-1-20250805', 'claude-opus-4-20250514', 'claude-sonnet-4-20250514']
Perplexity models: ['sonar', 'sonar-pro', 'sonar-deep-research', 'sonar-reasoning', 'sonar-reasoning-pro']

=== Get OpenAI Models ===
OpenAI models: ['text-embedding-ada-002', 'whisper-1', 'gpt-3.5-turbo', 'tts-1', 'gpt-3.5-turbo-16k', 'gpt-4-0613', 'gpt-4', 'davinci-002', 'babbage-002', 'gpt-3.5-turbo-instruct', 'gpt-3.5-turbo-instruct-0914', 'gpt-3.5-turbo-1106', 'tts-1-hd', 'tts-1-1106', 'tts-1-hd-1106', 'text-embedding-3-small', 'text-embedding-3-large', 'gpt-3.5-turbo-0125', 'gpt-4-turbo', 'gpt-4-turbo-2024-04-09', 'gpt-4o', 'gpt-4o-2024-05-13', 'gpt-4o-mini-2024-07-18', 'gpt-4o-mini', 'gpt-4o-2024-08-06', 'omni-moderation-latest', 'omni-moderation-2024-09-26', 'o1-2024-12-17', 'o1', 'o3-mini', 'o3-mini-2025-01-31', 'gpt-4o-2024-11-20', 'gpt-4o-mini-search-preview-2025-03-11', 'gpt-4o-mini-search-preview', 'gpt-4o-transcribe', 'gpt-4o-mini-transcribe', 'o1-pro-2025-03-19', 'o1-pro', 'gpt-4o-mini-tts', 'o3-2025-04-16', 'o4-mini-2025-04-16', 'o3', 'o4-mini', 'gpt-4.1-2025-04-14', 'gpt-4.1', 'gpt-4.1-mini-2025-04-14', 'gpt-4.1-mini', 'gpt-4.1-nano-2025-04-14', 'gpt-4.1-nano', 'gpt-image-1', 'o4-mini-deep-research', 'gpt-4o-transcribe-diarize', 'o4-mini-deep-research-2025-06-26', 'gpt-5-chat-latest', 'gpt-5-2025-08-07', 'gpt-5', 'gpt-5-mini-2025-08-07', 'gpt-5-mini', 'gpt-5-nano-2025-08-07', 'gpt-5-nano', 'gpt-audio-2025-08-28', 'gpt-realtime', 'gpt-realtime-2025-08-28', 'gpt-audio', 'gpt-5-codex', 'gpt-image-1-mini', 'gpt-5-pro-2025-10-06', 'gpt-5-pro', 'gpt-audio-mini', 'gpt-audio-mini-2025-10-06', 'gpt-5-search-api', 'gpt-realtime-mini', 'gpt-realtime-mini-2025-10-06', 'sora-2', 'sora-2-pro', 'gpt-5-search-api-2025-10-14', 'gpt-5.1-chat-latest', 'gpt-5.1-2025-11-13', 'gpt-5.1', 'gpt-5.1-codex', 'gpt-5.1-codex-mini', 'gpt-5.1-codex-max', 'gpt-image-1.5', 'gpt-5.2-2025-12-11', 'gpt-5.2', 'gpt-5.2-pro-2025-12-11', 'gpt-5.2-pro', 'gpt-5.2-chat-latest', 'gpt-4o-mini-transcribe-2025-12-15', 'gpt-4o-mini-transcribe-2025-03-20', 'gpt-4o-mini-tts-2025-03-20', 'gpt-4o-mini-tts-2025-12-15', 'gpt-realtime-mini-2025-12-15', 'gpt-audio-mini-2025-12-15', 'chatgpt-image-latest', 'gpt-5.2-codex', 'gpt-5.3-codex', 'gpt-realtime-1.5', 'gpt-audio-1.5', 'gpt-4o-search-preview', 'gpt-4o-search-preview-2025-03-11', 'gpt-5.3-chat-latest', 'gpt-5.4-2026-03-05', 'gpt-5.4-pro', 'gpt-5.4-pro-2026-03-05', 'gpt-5.4', 'gpt-5.4-nano-2026-03-17', 'gpt-5.4-nano', 'gpt-5.4-mini-2026-03-17', 'gpt-5.4-mini', 'gpt-image-2', 'gpt-image-2-2026-04-21', 'gpt-5.5', 'gpt-5.5-2026-04-23', 'gpt-5.5-pro', 'gpt-5.5-pro-2026-04-23', 'chat-latest', 'gpt-realtime-translate', 'gpt-realtime-2', 'gpt-realtime-whisper']

=== Get Anthropic Models ===
Anthropic models: ['claude-fable-5', 'claude-opus-4-8', 'claude-opus-4-7', 'claude-sonnet-4-6', 'claude-opus-4-6', 'claude-opus-4-5-20251101', 'claude-haiku-4-5-20251001', 'claude-sonnet-4-5-20250929', 'claude-opus-4-1-20250805', 'claude-opus-4-20250514', 'claude-sonnet-4-20250514']

✓ Chat Models API example complete
✓ Client created

=== Create User Function ===
Created user function with ID: xZL1xZhySYsWf2bkUB9aFfIrTdeqz1NJqj-retIv4BG-z5irMnChlTgL1f0DxAhMy0l6bAsQBYfXdj9Q7JDzOg

=== Get User Function ===
Retrieved: get_active_users_py - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 14 user functions:
  - cache_api_call_py: Cache External API Call
  - fetch_user: Fetch user by code
  - get_active_users_updated: Get Active Users (Updated)
  - get_active_users_paginated: Get Active Users (Paginated)
  - get_active_users_py: Get Active Users
  - fetch_slim_user: Validate and slim down user
  - get_active_users_updated: Get Active Users (Updated)
  - get_users_by_status: Get Users By Status
  - get_user_wrapper: Wrapper that calls fetch_user
  - get_verified_user: Get verified and validated user
  - get_high_scoring_active_users: Get High Scoring Active Users
  - validate_user: Check if user exists
  - swr_user: SWR pattern for user data (KV-based)
  - fetch_and_store_user: Fetch user from API and cache in KV

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
✅ [32mAll Python integration tests complete![0m
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
Inserted: map[id:7sg5kj6HKY2xkmTMdiTeiQHneLVOFBnKL4LBeoqJFbk6fBWBWiUfEshBY6gMDNEj4fRehoGcqMzIteNsgHrjzA]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] id:7sg5kj6HKY2xkmTMdiTeiQHneLVOFBnKL4LBeoqJFbk6fBWBWiUfEshBY6gMDNEj4fRehoGcqMzIteNsgHrjzA name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query ===
Found documents: [map[active:map[type:Boolean value:true] id:7sg5kj6HKY2xkmTMdiTeiQHneLVOFBnKL4LBeoqJFbk6fBWBWiUfEshBY6gMDNEj4fRehoGcqMzIteNsgHrjzA name:map[type:String value:Test Record] value:map[type:Integer value:42]]]

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] id:7sg5kj6HKY2xkmTMdiTeiQHneLVOFBnKL4LBeoqJFbk6fBWBWiUfEshBY6gMDNEj4fRehoGcqMzIteNsgHrjzA name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
[32m✓ simple_crud.go completed successfully[0m
[34m
=== Running simple_websocket.go ===[0m
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: YM-G28Ye5HW60hW5TeagNc7tHBRMMevw6db7Q2dkuPwrAlFowwaaQe-qkTQlUak-0DSJviThzKxxSPS9vsDxZA

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "messageId": "1781055417268695000",
  "payload": {
    "data": [
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "Rx_EhUtGO6iii80P81nw7Bu_teSO3QUJk0QQ5tqdYHt_E7rl36XSAj6fL0Fa5ZVfaAmRGe0SxQ4uDM2G7ETLTQ",
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
        "id": "CKITd5EqDQjiV8hLrfhGPrj49ZBeSaAjnkyrLO7f0qy6xIDC3sWKcXH52YZBaj6wsf3LX2u00aSr-38Iom4dqA",
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
        "id": "YM-G28Ye5HW60hW5TeagNc7tHBRMMevw6db7Q2dkuPwrAlFowwaaQe-qkTQlUak-0DSJviThzKxxSPS9vsDxZA",
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
🚀 ekoDB Functions Example (Go/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Function saved: 5tBtF7sQAFN_Qp1O1Nc5cQdLMJH3OzqlSWynssA-brMf4WfOrZjP_4_UsITAO3aUKkza9S5t7NFVOXfsghrEag
📊 Found 10 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

ℹ️  Function 'get_active_users_paginated' already existed — updated instead
✅ Function saved: FT-GQKYGXXLFV9yxeNHZhUzY64namsbzE9s8m3UDXHdjjRd5_sQWIvVyiPEx40I4xbhiOSuIjYisePqeknyEeA
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: 0CImYEj5k_VC0V8jDp4XXgv019VdNL2esJzkbgudwdjfYHmL5_Iw64ASEN8-WBIS3_4Gy9mzEvyTD9gi0Xqn4A
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"avg_score":{"type":"Float","value":50},"count":{"type":"Integer","value":10},"max_score":{"type":"Integer","value":90},"status":{"type":"String","value":"inactive"}}
   {"avg_score":{"type":"Float","value":60},"count":{"type":"Integer","value":10},"max_score":{"type":"Integer","value":100},"status":{"type":"String","value":"active"}}

📝 Example 4: Function Management

📋 Total functions: 15
🔍 Retrieved function: Get Active Users
✏️  Function updated
🗑️  Function deleted

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
Collection created with first record: FEMyJFVh7Hp3LEpPEIKk-NEC1Q7BtuYCbWUurMeUstOXlsdcJ0TfCqtrgyw62Q1LY0TY9_YK62oZOZ0qSQBgUw

=== List Collections ===
Total collections: 18
Sample collections: [chat_messages__ek0_testing chat_configurations__ek0_testing enriched_users batch_users schema_employees_client_py]

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
Created Alice: $1000 - ID: 6UAwjAMRswraR2easL7kpVcPKx64aJzWEgb51AALsQiQnsXzAlIvekSirfTXSPvSDb4lZ6bOnHxTExn-44vvsQ
Created Bob: $500 - ID: xZAzsrvBuQeiUEXhp8BqUL1JrXzU35cZ5mTfyYOcu6VMxFWU36mJInb_zEq5VLsDfl7XXAJRHCB3xVq9jlmwyA

=== Example 1: Begin Transaction ===
Transaction ID: 5f56d473-1664-4696-9611-52b10755ebb2

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: map[type:Integer value:800]
Bob: map[type:Integer value:700]

=== Example 5: Rollback ===
New transaction: 64b86344-33a5-4911-8aeb-b4709de5bcdd
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: map[type:Integer value:700]

=== Cleanup ===
✓ Deleted test accounts

✓ All transaction examples completed
[32m✓ transactions.go completed successfully[0m
[34m
=== Running crud_functions.go ===[0m
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
   ✅ Function saved: PFNR5JjWbvCzeieFPX81w_YrzXihS2hsOdYahdIlM4gNDpWadJLozU9F7g7o7qcXQEfwk5GGGdZZpAdZEqSJtw

2️⃣ Calling function (Insert + Verify)...
   ✅ function executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: sz2udHVSfi2LUPXhWuolmNn8hneI_ycAj5euW8Yv0cZzyqG9BtzupvoXZryCHdzkmrXit3c5FeI6V7wfamy9Zw
   📋 Name: map[type:String value:Alice Smith]
   📋 Email: map[type:String value:alice@example.com]
   📋 Status: map[type:String value:pending]
   📋 Credits: map[type:Integer value:0]

============================================================
📝 function 2: Query + Update + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: n9yrilX3aSgsqN7OOkqhEQ6mAhu-0um1JghUyPRSY7s2_urmhVkiWN6EYvjYQIK-hJbLtL3dOdjRbV1dD01pLQ

2️⃣ Calling function (Query + Update + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Status updated to: map[type:String value:active]
   📋 Name: map[type:String value:Alice Smith]

============================================================
📝 function 3: Query + Update Credits + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: 2iu6WCKdIL0fBkvnc-pOsDJxAot8-41DQuPIGklkWzgJXAizdw6mg6ZhsJi3nCLQooQxq9ADUIdyO58gLacW_g

2️⃣ Calling function (Query + Update Credits + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: map[type:Integer value:0]
   📋 Status: map[type:String value:active]
   📋 Name: map[type:String value:Alice Smith]

============================================================
📝 function 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: hCxv4SOZaaPvZuCUf_VGfXCOB36xU6wZjBQo59zIQYDDjVOG_BWAqZkpe6yAXNFb-FKFqDSjhpKacj4vGOqTsg

2️⃣ Calling function (Query + Delete + Verify)...
   ✅ function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: PFNR5JjWbvCzeieFPX81...
   ✅ Deleted script: n9yrilX3aSgsqN7OOkqh...
   ✅ Deleted script: 2iu6WCKdIL0fBkvnc-pO...
   ✅ Deleted script: hCxv4SOZaaPvZuCUf_VG...
   ✅ Deleted collection: users

============================================================
✅ Complete CRUD Functions Example Finished!
============================================================

💡 Key Takeaways:
   ✅ Functions chain steps together
   ✅ Each function demonstrates operation + verification
   ✅ Parameters make functions reusable
   ✅ Verification is built into the function itself
   ✅ Complete CRUD lifecycle in 4 focused functions
[32m✓ crud_functions.go completed successfully[0m
[34m
=== Running document_ttl.go ===[0m
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
  Output: Document ID = 0sfXPI5OzCF1ueeb0z7AmgnBwLh3JADRgZ_B0KrCZ-jvjxxHT46zArv9TZ84wY1XMnES47fav-RcpOht0j-gFg
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: FindByID(0sfXPI5OzCF1ueeb0z7AmgnBwLh3JADRgZ_B0KrCZ-jvjxxHT46zArv9TZ84wY1XMnES47fav-RcpOht0j-gFg)
  Output: Found document with name = map[type:String value:TTL Test]
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: FindByID(0sfXPI5OzCF1ueeb0z7AmgnBwLh3JADRgZ_B0KrCZ-jvjxxHT46zArv9TZ84wY1XMnES47fav-RcpOht0j-gFg)
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

✓ Client connected

═══════════════════════════════════════════════════════════
TEST: WebSocket TTL Expiration
═══════════════════════════════════════════════════════════

[Step 1] Insert document with 3 second TTL
  Input: {name: 'WS TTL Test', value: 'should expire'}
  TTL: 3s
  Output: Document ID = GN2Yq0xxDRKIls9Npq-5b4QYr_RJeMVo6C6fz6tCqaAGGhcmsUc9HpZHzsNdEUq--ilEvlozkFtrvJG4UCy_-Q
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: FindByID(GN2Yq0xxDRKIls9Npq-5b4QYr_RJeMVo6C6fz6tCqaAGGhcmsUc9HpZHzsNdEUq--ilEvlozkFtrvJG4UCy_-Q)
  Output: Found document with name = map[type:String value:WS TTL Test]
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: FindByID(GN2Yq0xxDRKIls9Npq-5b4QYr_RJeMVo6C6fz6tCqaAGGhcmsUc9HpZHzsNdEUq--ilEvlozkFtrvJG4UCy_-Q)
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
Inserted: map[id:9r2Y87ErxDwgU5GOZoXj7qsd8XXMLGWK4n_-BxqmraAJHKfUhVTRT-WmzMUvZcSomTVFIRtkz_N_KP6fp4F8oA]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-06-10T01:37:27Z] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:9r2Y87ErxDwgU5GOZoXj7qsd8XXMLGWK4n_-BxqmraAJHKfUhVTRT-WmzMUvZcSomTVFIRtkz_N_KP6fp4F8oA metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Test Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:42]]

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.990000
  created_at (DateTime): 2026-06-10 01:37:27 +0000 UTC
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1 tag2 tag3]
  metadata (Object): map[key:value nested:map[deep:true]]
  embedding (Vector): [0.1 0.2 0.3 0.4 0.5]
  categories (Set): [electronics computers]
  data (Bytes): 11 bytes
Plain record: map[active:true categories:[electronics computers] created_at:2026-06-10T01:37:27Z data:aGVsbG8gd29ybGQ= embedding:[0.1 0.2 0.3 0.4 0.5] id:9r2Y87ErxDwgU5GOZoXj7qsd8XXMLGWK4n_-BxqmraAJHKfUhVTRT-WmzMUvZcSomTVFIRtkz_N_KP6fp4F8oA metadata:map[key:value nested:map[deep:true]] name:Test Record price:99.99 tags:[tag1 tag2 tag3] user_id:550e8400-e29b-41d4-a716-446655440000 value:42]

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-06-10T01:37:27Z] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:9r2Y87ErxDwgU5GOZoXj7qsd8XXMLGWK4n_-BxqmraAJHKfUhVTRT-WmzMUvZcSomTVFIRtkz_N_KP6fp4F8oA metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Updated Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: tE7ihtlsI3APMV6GvbWBnu4-RTJkaIW3MgrT2-U5iL5BkMOw0fbmO8Vat9SFCHLSGEq-1L6vJhYd1_yP8Jcn8g

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
Collection created with first record: O3zJYn27Az5H_cRjZRC8HK3IUfL7QNJenZKCXOFIHo186DRhehfMFCqLgJ2eJiG39K8t2yOwkp80hrbaH1mSpQ

=== List Collections ===
Total collections: 17
Sample collections: [chat_messages__ek0_testing chat_configurations__ek0_testing enriched_users batch_users schema_employees_client_py]

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
  cache:product:1: map[name:Product 1 price:29.99]
  cache:product:2: map[name:Product 2 price:39.99]
  cache:product:3: map[name:Product 3 price:49.99]

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 3 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 12

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
Created Alice: $1000 - ID: wnE_qGUzuJvhSRrFIxV9jhlieoXOYDbBHpoygjivXbtR7TuL1SSzfZWkGvlg8R7DLO1QlpxMm6muJsxquzriGg
Created Bob: $500 - ID: 3txKcg5hAL8aktNzfxYd54KRKXhUqrSXP_toQEFwhpvFmoDDUo7aFOTls3hHGzFBmCfHGOKM44qtSzOElDm8Bw

=== Example 1: Begin Transaction ===
Transaction ID: 8e65e5e0-bd04-4568-9649-abe4c171cece

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 64b798b8-d0a4-4329-a714-8bd77b629ce0
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
  1. Score: 0.775
  2. Score: 0.769
  3. Score: 0.748

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.699
  2. Score: 1.507
  3. Score: 0.310

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
  - Mouse ($0) by Alice Johnson
  - Laptop ($0) by Alice Johnson

5. Complex join with multiple conditions:
Found 2 users with example.com emails
  - Alice Johnson (alice@example.com): Building A
  - Bob Smith (bob@example.com): Building B

=== Cleanup ===
✅ Deleted test collections

✅ Join operations examples completed!
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: mA3_6KWkH0M0bwFmd2FvBGA-KWzdzC04R1GKqLwcMHOTTiX2jMHldDidWsjsE6NX5lm87aCMAp8q87Ia59nUjA

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: 7zUPtgL3oOOFadZ_1h8KIwYUBXolao_FjupsLD91Mw8F74ajvTik6PsoQlDDnQbZ2l5yVM6hQ5wKkWMM0YFfTA

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
✓ Inserted document with TTL: kc8y_kluSk-LZKcAzrJOoaX0pJpP_xSKxLejcnhqk5v1MdM2IN6CvbR_Z5YgXNnJhysVnDD2svd7xiiHYR3BDQ

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
✓ Edge cache script created: Hbv_DepMkzpVbfKf9mJqrjYcYx0qDOSEZ02SE-ycPnDRi2TZmJmytq06ujfSiKGs2d6ixBA5ddPQXScW1PC71g

Call 1: Cache lookup
Response time: 14ms
Found 1 cached entries

Call 2: Cache lookup (connection warm)
Response time: 14ms
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
🚀 ekoDB Functions Example (Go Client)

✅ Client initialized

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

ℹ️  Function 'get_active_users' already existed — updated instead
✅ Function saved: 2GKBqD-lb0-1nEOSMqao4KmOwozCliy6ccwPivWiCPymXlO6YVHzodPT_ukVh-luZ_ZFPLJ9ACX6tvZ8lJnsCw
📊 Found 10 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Function

ℹ️  Function 'get_users_by_status' already existed — updated instead
✅ Function saved
📊 Found 10 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Function

✅ Function saved
📊 Statistics: 2 groups
   map[avg_score:map[type:Float value:60] count:map[type:Integer value:5] status:map[type:String value:active]]
   map[avg_score:map[type:Float value:50] count:map[type:Integer value:5] status:map[type:String value:inactive]]
⏱️  Execution time: 0ms

📝 Example 4: Function Management

📋 Total functions: 15
🔍 Retrieved function: Get Active Users
✏️  Function updated
🗑️  Function deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

📝 Example 5: Multi-Stage Pipeline

✅ Multi-stage script saved
📊 Pipeline executed 2 stages
⏱️  Total execution time: 0ms
📈 Stage breakdown:

📝 Example 6: Count Users

✅ Count script saved
📊 Total user count: 0
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

Function 'fetch_user' already existed — updated instead
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

Function 'fetch_and_store_user' already existed — updated instead
✅ Saved reusable function: fetch_and_store_user (uses KV)
Function 'swr_user' already existed — updated instead
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 38.800875ms
   📊 Records: 1
   📦 Data: {
        "value": {
          "type": "Object",
          "value": {
            "address": {
              "city": "Gwenborough",
              "geo": {
                "lat": "-37.3159",
          ...

Second call (cache hit - from cache):
   ⏱️  Duration: 38.701459ms
   📊 Records: 1
   📦 Data: {
        "value": {
          "type": "Object",
          "value": {
            "address": {
              "city": "Gwenborough",
              "geo": {
                "lat": "-37.3159",
          ...
   🚀 Cache speedup: 1.0x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

Function 'validate_user' already existed — updated instead
✅ Level 1 function: validate_user
Function 'fetch_slim_user' already existed — updated instead
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
🚀 ekoDB Go Complete Functions Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Function saved: 7CzzNsz1yHl0mWGOVetUlXQfwoNimk4vFZfnpnCu3e9IqtcQlxm9erSIOITlG6oXMbEcyCu3Pbv0Uufp_w7jsQ
📊 Found 2 category stats

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories

📝 Example 4: Multi-Stage Pipeline (FindAll → Group → Count)

✅ Function saved
📊 Pipeline executed 3 stages

🧹 Cleaning up...
✅ Cleanup complete

✅ All complete script examples finished!

💡 This example demonstrates ekoDB's function system:
   ✅ FindAll operations
   ✅ Group aggregations (Count, Average)
   ✅ Multi-stage pipelines (FindAll → Group → Count)
   ✅ Function management (save, call, delete)
🚀 ekoDB Go KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: jz_4IwNi3ciXtZ81ma5AN00Tq16Ab5XL4FgqaZujW-eLtwaSyRD5oNXESZIPjqMLz85h8G626QdwYwdCmdFcqg
✅ Inserted 2 products with wrapped types

📝 Example 2: function with Wrapped Type Parameters

✅ Function saved: uWAvkVQBa9Bgt_u7tZQUlD59FVc6gKVqjWpAh2KBsoL-WCDiyjjwfMcLQuo95FfDZMdjdT--sw9uc4cVXLicKA
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: map[role:admin userId:user_abc]
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session
📝 Example 4: KV Operations in Functions

✅ Function saved: 1NTxgah8Fe1d8Jpau1_3GjTDi64h7HL8NfhnXP58C2lWd7odPMLvYLuFT0weaLOJu5OWczDwNGCw11HR-QeZmw
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Function

✅ Function saved: HA69HZ7cvwFxU7IO5XPOIh_cVaiIYRvyjz7OEZ_xd5hYOlcKKFBx5ZKZZ3cbPk1mOgLw0RKhpoYhW2f5hh0O5Q
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
✓ Created SWR script: fetch_api_user_go (TBWnYd_xZxPUTfiko7gVDCmq4S0phbcqBBp9ciMXEc-C_QjLi5PaBkwkEIPnMqLPd94HUP5jTvBnxXVWxrFdeA)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "cached_at": {
        "type": "DateTime",
        "value": "2026-06-10T01:37:33Z"
      },
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
      "id": "1"
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
Response time: 42ms (served from cache)
✓ Lightning fast cache hit

🧹 Cleaning up...
✓ Cleanup complete

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation
🚀 ekoDB Go Client - Native SWR Function Examples

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
✓ Created native SWR script: github_user_native (MIb7YlfflKF1XWiaABXrD9wej4H7GvWsR-6Oz6lF4K7ELLOramanZJPRSRZZ4-euIz6aOPosWV9QV7qivbCrWw)

First call (cache miss - will fetch from GitHub API):
  Response time: 14ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 14ms
  Speedup: 1.0x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (-rpvMnftF0TcoCk7uR2XVWarFz6p2z2Pu_kQceSCHGQX2E8oOikMOjm0k-PwBYcg9_dyLreELviTkLA-qdwl7A)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (D2kD04EmWm8Xe34n1X3w8dhC-myTH_BqvZVP13z9GTYnyiaiXnHvEsPxrkrK4VIcJLda2eGEfhNSntoSWDyiog)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (9VKYP705UzeZtaDlFF9V_2mxUyYC6A_8rBU3ZIWoBu61pgvL21vkaBnZ8Br2NAcqybquN9K2bQkV8X5llv6u_Q)
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
🚀 ekoDB Go Advanced Functions Example

📋 Setting up test data...
✅ Created 8 products

📝 Example 1: List All Products

✅ Function saved
📊 Found 8 products
⏱️  Execution time: 0ms

📝 Example 2: Group Products by Category

✅ Function saved
📊 Category breakdown:
   map[avg_price:map[type:Float value:367] category:map[type:String value:Electronics] count:map[type:Integer value:5]]
   map[avg_price:map[type:Float value:365.6666666666667] category:map[type:String value:Furniture] count:map[type:Integer value:3]]
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced script examples finished!
🚀 ekoDB Go AI Functions Example

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
🚀 ekoDB Go CRUD Functions Example

📋 Setting up test data...
✅ Created 10 test users

📝 Example 1: List All Users

✅ Function saved
📊 Found 10 users
⏱️  Execution time: 0ms

📝 Example 2: Count Users by Status

✅ Function saved
📊 User counts by status:
   map[count:map[type:Integer value:3] status:map[type:String value:inactive]]
   map[count:map[type:Integer value:7] status:map[type:String value:active]]
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD script examples finished!
🚀 ekoDB Go Search Functions Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Function saved
📊 Found 5 documents
   1. map[type:String value:Introduction to Machine Learning] (map[type:String value:AI])
   2. map[type:String value:Database Design Principles] (map[type:String value:Database])
   3. map[type:String value:Natural Language Processing] (map[type:String value:AI])
   4. map[type:String value:Vector Databases Explained] (map[type:String value:Database])
   5. map[type:String value:Getting Started with ekoDB] (map[type:String value:Database])
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   map[category:map[type:String value:Database] count:map[type:Integer value:3]]
   map[category:map[type:String value:AI] count:map[type:Integer value:2]]
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: XMcm9EI1r0XL4MrLSKCmrJUS-QsNU0ih0n75U0Gmv1MumkPwb3ie_6eM7UNzTVyEipHHZKNJ0NDjQed-w7ci4Q

=== Sending Chat Message ===
Message ID: 40ZRVsB-zzVlHAs6x3us0NQqKWiwBE-84CZ96ivOhDRVGoiaXIUV8FBgzbtcFrLt72H2CdhoiEEq_wo5yuXoow

=== AI Response ===
The available products and their prices are as follows:

1. **ekoDB Pro**
   - Price: $299
   - Description: Enterprise edition product with advanced features.

2. **ekoDB**
   - Price: $99
   - Description: A high-performance database product with AI capabilities.

3. **ekoDB Cloud**
   - Price: $499
   - Description: Fully managed cloud database service product.

=== Context Used (3 snippets) ===
  Snippet 1: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Enterprise edition product with advanced features id:lFcVrcpI74uZKSShjUYqHGrZiZmTxF3emy02qNm9Yt20lc4F-4vPVE0Ez3mx3dEzqiIMtS0JcWMKFZWxuI5aUw name:ekoDB Pro price:299] score:0.1111111111111111]
  Snippet 2: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:A high-performance database product with AI capabilities id:abss_URlE39vuCKZzN3zU23b31ozGS9OltRo-As48gjRtzHR0_Fn3vURtv4cnQ3X4prX79a011G3C4ZIq3wjKw name:ekoDB price:99] score:0.1111111111111111]
  Snippet 3: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Fully managed cloud database service product id:lBXlblxX89wpGNwFqZQFVvC7BDUjBnjzktd02kYnGP3GmYCYp9FhvGPL9oAwYgdLBEMQbKAuYoTmUjsemWsQ9A name:ekoDB Cloud price:499] score:0.1111111111111111]

Execution Time: 4853ms

=== Token Usage ===
Prompt tokens: 3468
Completion tokens: 91
Total tokens: 3559

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: qbMqeSRERW6RBedbNq-JbWT1rKD28IX86Gx0tYwP2LHtBS0vEwvxnHhNO9skzowbnx8_dgDPl9TLmPGJbJkPQQ

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: High-performance database product
- **Price**: $99

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
✓ Created second session: 02WYH5fvFCE8jEaFL6l-ZbPq2sEWjD9Le24i-ON89iHVdcsM7uDa9qzgXiCclIs5t99xOqTR6XzzdlGuMjzC8A
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
✓ Created session: zQ9sPZrEwJouHWdIPzV9BxVUsjLgEVWPvg3BrCKx8qpphMyfN-7PLO8sytR8_4cyYxwTn6ULgBUTvM1x5lGCPQ

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name**: ekoDB
- **Description**: A high-performance database product
- **Price**: $99

If you need more information or additional products, feel free to ask!

✓ Message 2 sent
  Response: The price of ekoDB is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: scHoCdqZAJVQZF6VADfZcdxVWVqrm3HWTjA6K5b3AOwACMqak3cjX1AKFRc8zWlgy1QRZXAdKddz2VWA2eYSpA
  Parent: zQ9sPZrEwJouHWdIPzV9BxVUsjLgEVWPvg3BrCKx8qpphMyfN-7PLO8sytR8_4cyYxwTn6ULgBUTvM1x5lGCPQ

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: scHoCdqZAJVQZF6VADfZcdxVWVqrm3HWTjA6K5b3AOwACMqak3cjX1AKFRc8zWlgy1QRZXAdKddz2VWA2eYSpA (Untitled)
  Session 2: zQ9sPZrEwJouHWdIPzV9BxVUsjLgEVWPvg3BrCKx8qpphMyfN-7PLO8sytR8_4cyYxwTn6ULgBUTvM1x5lGCPQ (Untitled)
  Session 3: 02WYH5fvFCE8jEaFL6l-ZbPq2sEWjD9Le24i-ON89iHVdcsM7uDa9qzgXiCclIs5t99xOqTR6XzzdlGuMjzC8A (Untitled)
  Session 4: XMcm9EI1r0XL4MrLSKCmrJUS-QsNU0ih0n75U0Gmv1MumkPwb3ie_6eM7UNzTVyEipHHZKNJ0NDjQed-w7ci4Q (Untitled)
  Session 5: arDUIy0JvWFMhNR6jVL2eXTmmA8sAWo7vbbVrTChdH5D5YqwuC8lHzluHFaE8OIBJfBDRPAPZbyxjVy30ZKX7A (Untitled)
  Session 6: IMk5O_Jl94E_vRIrr-4t-mn7L9CpF2IyC5Z0Ef83_ZaCDyZSCJ89KDGpGXX2_R1t3gfRiIBRPdi5_MZmmySl_w (Untitled)
  Session 7: _xiMGVjJpu7r4vWBMXRFRHvuAvR2EBTWY8KdSC_AcOwC4PwMb97zKEra0DwnBDRctEAN6RSpdCdwj7-4hPnZTA (Untitled)
  Session 8: 52rFAkddXArl9Wfh3XR69kFMTSMot2bVnbUXICbe3uD5lFV2kurJkCaWjWYN0VjzhpL5xT8JB6_P2h8zNpSyNg (Untitled)
  Session 9: 8yrkhzLnqlIO89Qd08Norl2-AOodRTEiI951KYaSHod4QHbqpXjfbGshqPddEhOJKWf957vuyoz4_mb7w9lg1w (Untitled)
  Session 10: 1eyYvkAa7VtrEl2EJ-CuUCoLL3yBazPNyI9Bmx9_-oE21yh6sdcaRibFYofsJpsdK35PC6_4AQg83FBJuliVSw (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: scHoCdqZAJVQZF6VADfZcdxVWVqrm3HWTjA6K5b3AOwACMqak3cjX1AKFRc8zWlgy1QRZXAdKddz2VWA2eYSpA

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Map Creation ===
✓ Created record with native map: map[id:QoNNvUYUke1CrC-k-3XDYppV3F06euyJW0Qk0FKQTgLVGLJqpWZPZ6IjUzMYDPj8r-uMunCnsbRBxW3bxdt5kg]

=== Upsert Operation ===
✓ First upsert (update): map[active:map[type:Boolean value:true] age:map[type:Integer value:29] email:map[type:String value:alice.j@newdomain.com] id:QoNNvUYUke1CrC-k-3XDYppV3F06euyJW0Qk0FKQTgLVGLJqpWZPZ6IjUzMYDPj8r-uMunCnsbRBxW3bxdt5kg name:map[type:String value:Alice Johnson]]
✓ Second upsert (insert): map[id:new-user-id]

=== Find One Operation ===
✓ Found user by email: map[active:map[type:Boolean value:true] age:map[type:Integer value:29] email:map[type:String value:alice.j@newdomain.com] id:QoNNvUYUke1CrC-k-3XDYppV3F06euyJW0Qk0FKQTgLVGLJqpWZPZ6IjUzMYDPj8r-uMunCnsbRBxW3bxdt5kg name:map[type:String value:Alice Johnson]]
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
Inserted with ripple: map[id:EQu1jFkDKdL9HoUiws9YZi8lsu1ZU3xj6p_80S1dqAInrZPIn2jBSSdonA8fu1iXrMhtaILwBz-yC_8UUvIGEw]
Inserted with bypass_ripple: map[id:gPqIj8kNjYQA26p5PUzMxjGiXhWGAnb9J7T88jEKfq9_QxrAa3CVJbNi6ScD6V2DfKxqc0DJlQnUQYpyAJluNQ]
Inserted with TTL and bypass_ripple: map[id:cJPxwAPkiRth_VZmvwwFeMVlRewKOzkNifD9Nvk_GPR-zv-NVHD1xpRjCsa6CfCFo8iwFyaZ7DgGhQOMC70KcA]
Updated with bypass_ripple: map[id:EQu1jFkDKdL9HoUiws9YZi8lsu1ZU3xj6p_80S1dqAInrZPIn2jBSSdonA8fu1iXrMhtaILwBz-yC_8UUvIGEw name:map[type:String value:Product 1] price:map[type:Integer value:150]]
Deleted with bypass_ripple
Batch inserted with bypass_ripple: 2 records
Upserted with bypass_ripple: map[id:custom-id]
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields (id, name, email only)
  Found 3 active users
  Fields returned: [email id name]
  First user: Bob Smith <bob@example.com>

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: [age avatar_url bio created_at email id name status user_role]

Example 3: Complex query with projection (active users, ages 18-65)
  Found 3 active users (ages 18-65)
    - Dave Brown (age 45)
    - Alice Johnson (age 30)
    - Bob Smith (age 25)

Example 4: Query inactive users with profile fields
  Found 1 inactive users
    - Carol White: Manager

Example 5: Compare full vs projected data
  Full query:
    - 12 fields per record
    - Fields: [age api_key avatar_url bio created_at email id name password secret_token status user_role]
  Projected query:
    - 3 fields per record
    - Fields: [email id name]
  Bandwidth savings: ~75% fewer fields

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
=== KV Precision: Float vs Decimal ===

=== Test 1: Using Go Floats (LOSES PRECISION) ===
✓ Stored products with float prices

Retrieved float prices:
  Widget A: $29.99 (expected $29.99) ✓
  Widget B: $39.99 (expected $39.99) ✓
  Widget C: $49.99 (expected $49.99) ✓

=== Test 2: Using FieldDecimal() (PRESERVES PRECISION) ===
✓ Stored products with decimal prices

Retrieved decimal prices:
  Widget A: $29.99 (expected $29.99) ✓
  Widget B: $39.99 (expected $39.99) ✓
  Widget C: $49.99 (expected $49.99) ✓

=== Test 3: Sum Calculation Comparison ===
  Float sum: $119.97 (expected $119.97)
  Decimal sum: $119.97 (expected $119.97)

=== Test 4: Extreme Precision Example ===
  Float 0.1 + 0.2 = 0.29999999999999999 (should be 0.3)
  Decimal "0.30" = 0.30 (exact!)

=== Cleanup ===
✓ Cleaned up test keys

=== Summary ===
✅ Use FieldDecimal() for monetary values, percentages, and
   any case where floating-point errors are unacceptable.
✅ FieldDecimal() stores values as strings internally,
   preserving exact precision across all operations.
✓ Client created

=== Get All Chat Models ===
OpenAI models: [text-embedding-ada-002 whisper-1 gpt-3.5-turbo tts-1 gpt-3.5-turbo-16k gpt-4-0613 gpt-4 davinci-002 babbage-002 gpt-3.5-turbo-instruct gpt-3.5-turbo-instruct-0914 gpt-3.5-turbo-1106 tts-1-hd tts-1-1106 tts-1-hd-1106 text-embedding-3-small text-embedding-3-large gpt-3.5-turbo-0125 gpt-4-turbo gpt-4-turbo-2024-04-09 gpt-4o gpt-4o-2024-05-13 gpt-4o-mini-2024-07-18 gpt-4o-mini gpt-4o-2024-08-06 omni-moderation-latest omni-moderation-2024-09-26 o1-2024-12-17 o1 o3-mini o3-mini-2025-01-31 gpt-4o-2024-11-20 gpt-4o-mini-search-preview-2025-03-11 gpt-4o-mini-search-preview gpt-4o-transcribe gpt-4o-mini-transcribe o1-pro-2025-03-19 o1-pro gpt-4o-mini-tts o3-2025-04-16 o4-mini-2025-04-16 o3 o4-mini gpt-4.1-2025-04-14 gpt-4.1 gpt-4.1-mini-2025-04-14 gpt-4.1-mini gpt-4.1-nano-2025-04-14 gpt-4.1-nano gpt-image-1 o4-mini-deep-research gpt-4o-transcribe-diarize o4-mini-deep-research-2025-06-26 gpt-5-chat-latest gpt-5-2025-08-07 gpt-5 gpt-5-mini-2025-08-07 gpt-5-mini gpt-5-nano-2025-08-07 gpt-5-nano gpt-audio-2025-08-28 gpt-realtime gpt-realtime-2025-08-28 gpt-audio gpt-5-codex gpt-image-1-mini gpt-5-pro-2025-10-06 gpt-5-pro gpt-audio-mini gpt-audio-mini-2025-10-06 gpt-5-search-api gpt-realtime-mini gpt-realtime-mini-2025-10-06 sora-2 sora-2-pro gpt-5-search-api-2025-10-14 gpt-5.1-chat-latest gpt-5.1-2025-11-13 gpt-5.1 gpt-5.1-codex gpt-5.1-codex-mini gpt-5.1-codex-max gpt-image-1.5 gpt-5.2-2025-12-11 gpt-5.2 gpt-5.2-pro-2025-12-11 gpt-5.2-pro gpt-5.2-chat-latest gpt-4o-mini-transcribe-2025-12-15 gpt-4o-mini-transcribe-2025-03-20 gpt-4o-mini-tts-2025-03-20 gpt-4o-mini-tts-2025-12-15 gpt-realtime-mini-2025-12-15 gpt-audio-mini-2025-12-15 chatgpt-image-latest gpt-5.2-codex gpt-5.3-codex gpt-realtime-1.5 gpt-audio-1.5 gpt-4o-search-preview gpt-4o-search-preview-2025-03-11 gpt-5.3-chat-latest gpt-5.4-2026-03-05 gpt-5.4-pro gpt-5.4-pro-2026-03-05 gpt-5.4 gpt-5.4-nano-2026-03-17 gpt-5.4-nano gpt-5.4-mini-2026-03-17 gpt-5.4-mini gpt-image-2 gpt-image-2-2026-04-21 gpt-5.5 gpt-5.5-2026-04-23 gpt-5.5-pro gpt-5.5-pro-2026-04-23 chat-latest gpt-realtime-translate gpt-realtime-2 gpt-realtime-whisper]
Anthropic models: [claude-fable-5 claude-opus-4-8 claude-opus-4-7 claude-sonnet-4-6 claude-opus-4-6 claude-opus-4-5-20251101 claude-haiku-4-5-20251001 claude-sonnet-4-5-20250929 claude-opus-4-1-20250805 claude-opus-4-20250514 claude-sonnet-4-20250514]
Perplexity models: [sonar sonar-pro sonar-deep-research sonar-reasoning sonar-reasoning-pro]

=== Get OpenAI Models ===
OpenAI models: [text-embedding-ada-002 whisper-1 gpt-3.5-turbo tts-1 gpt-3.5-turbo-16k gpt-4-0613 gpt-4 davinci-002 babbage-002 gpt-3.5-turbo-instruct gpt-3.5-turbo-instruct-0914 gpt-3.5-turbo-1106 tts-1-hd tts-1-1106 tts-1-hd-1106 text-embedding-3-small text-embedding-3-large gpt-3.5-turbo-0125 gpt-4-turbo gpt-4-turbo-2024-04-09 gpt-4o gpt-4o-2024-05-13 gpt-4o-mini-2024-07-18 gpt-4o-mini gpt-4o-2024-08-06 omni-moderation-latest omni-moderation-2024-09-26 o1-2024-12-17 o1 o3-mini o3-mini-2025-01-31 gpt-4o-2024-11-20 gpt-4o-mini-search-preview-2025-03-11 gpt-4o-mini-search-preview gpt-4o-transcribe gpt-4o-mini-transcribe o1-pro-2025-03-19 o1-pro gpt-4o-mini-tts o3-2025-04-16 o4-mini-2025-04-16 o3 o4-mini gpt-4.1-2025-04-14 gpt-4.1 gpt-4.1-mini-2025-04-14 gpt-4.1-mini gpt-4.1-nano-2025-04-14 gpt-4.1-nano gpt-image-1 o4-mini-deep-research gpt-4o-transcribe-diarize o4-mini-deep-research-2025-06-26 gpt-5-chat-latest gpt-5-2025-08-07 gpt-5 gpt-5-mini-2025-08-07 gpt-5-mini gpt-5-nano-2025-08-07 gpt-5-nano gpt-audio-2025-08-28 gpt-realtime gpt-realtime-2025-08-28 gpt-audio gpt-5-codex gpt-image-1-mini gpt-5-pro-2025-10-06 gpt-5-pro gpt-audio-mini gpt-audio-mini-2025-10-06 gpt-5-search-api gpt-realtime-mini gpt-realtime-mini-2025-10-06 sora-2 sora-2-pro gpt-5-search-api-2025-10-14 gpt-5.1-chat-latest gpt-5.1-2025-11-13 gpt-5.1 gpt-5.1-codex gpt-5.1-codex-mini gpt-5.1-codex-max gpt-image-1.5 gpt-5.2-2025-12-11 gpt-5.2 gpt-5.2-pro-2025-12-11 gpt-5.2-pro gpt-5.2-chat-latest gpt-4o-mini-transcribe-2025-12-15 gpt-4o-mini-transcribe-2025-03-20 gpt-4o-mini-tts-2025-03-20 gpt-4o-mini-tts-2025-12-15 gpt-realtime-mini-2025-12-15 gpt-audio-mini-2025-12-15 chatgpt-image-latest gpt-5.2-codex gpt-5.3-codex gpt-realtime-1.5 gpt-audio-1.5 gpt-4o-search-preview gpt-4o-search-preview-2025-03-11 gpt-5.3-chat-latest gpt-5.4-2026-03-05 gpt-5.4-pro gpt-5.4-pro-2026-03-05 gpt-5.4 gpt-5.4-nano-2026-03-17 gpt-5.4-nano gpt-5.4-mini-2026-03-17 gpt-5.4-mini gpt-image-2 gpt-image-2-2026-04-21 gpt-5.5 gpt-5.5-2026-04-23 gpt-5.5-pro gpt-5.5-pro-2026-04-23 chat-latest gpt-realtime-translate gpt-realtime-2 gpt-realtime-whisper]

=== Get Anthropic Models ===
Anthropic models: [claude-fable-5 claude-opus-4-8 claude-opus-4-7 claude-sonnet-4-6 claude-opus-4-6 claude-opus-4-5-20251101 claude-haiku-4-5-20251001 claude-sonnet-4-5-20250929 claude-opus-4-1-20250805 claude-opus-4-20250514 claude-sonnet-4-20250514]

✓ Chat Models API example complete
✓ Client created

=== Create User Function ===
Created user function with ID: E8hU7sgrJFVJFl6U9kx66AwpizyC32d3N2htf_BRfkD-HE4qQAvts4X0wTd9kXW1T6rog7BD7_9xtbynIft2oQ

=== Get User Function ===
Retrieved: get_active_users - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 9 user functions:
  - fetch_and_store_user: Fetch user from API and cache in KV
  - cache_api_call_py: Cache External API Call
  - validate_user: Check if user exists
  - swr_user: SWR pattern for user data (KV-based)
  - fetch_user: Fetch user by code
  - get_active_users: Get Active Users
  - fetch_slim_user: Validate and slim down user
  - get_user_wrapper: Wrapper that calls fetch_user
  - get_verified_user: Get verified and validated user

=== List User Functions by Tag ===
Found 9 user functions with 'users' tag:
  - fetch_and_store_user
  - cache_api_call_py
  - validate_user
  - swr_user
  - fetch_user
  - get_active_users
  - fetch_slim_user
  - get_user_wrapper
  - get_verified_user

=== Update User Function ===
User function updated successfully

=== Delete User Function ===
User function deleted successfully

✓ User Functions API example complete
✓ Client created

=== Check Collection Exists (Before Creation) ===
Collection 'collection_utils_test' exists: false

=== Creating Test Documents ===
Created 5 test documents

=== Check Collection Exists (After Creation) ===
Collection 'collection_utils_test' exists: true

=== Count Documents ===
Document count in 'collection_utils_test': 5

=== Check Non-Existent Collection ===
Collection 'nonexistent_collection_xyz' exists: false

=== Cleanup ===
Deleted collection 'collection_utils_test'

✓ Collection Utilities example complete
✓ Client created
✓ go_users_register saved
✓ go_users_login saved
✓ go_users_verify_token saved

=== Auth flow defined as pure stored functions ===
Call them like:
  POST /api/functions/go_users_register { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/go_users_login    { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/go_users_verify_token { "token": "<jwt>" }

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
✓ go_route_admin saved
✓ go_route_user_by_id saved
✓ go_route_user_posts saved
✓ go_route_org_create_member saved

Try them with curl:
  curl http://localhost:8080/api/route/users/admin
  curl http://localhost:8080/api/route/users/42
  curl http://localhost:8080/api/route/users/42/posts/7
  curl -X POST http://localhost:8080/api/route/orgs/acme/members \
       -H 'Content-Type: application/json' -d '{"name":"alice"}'

✓ Cleaned up demo functions
✅ [32mGo client examples complete![0m
✅ [32mAll Go integration tests complete![0m
📦 [36mBuilding TypeScript client library...[0m
npm warn EBADENGINE Unsupported engine {
npm warn EBADENGINE   package: 'vitest@4.1.8',
npm warn EBADENGINE   required: { node: '^20.0.0 || ^22.0.0 || >=24.0.0' },
npm warn EBADENGINE   current: { node: 'v23.6.0', npm: '10.9.2' }
npm warn EBADENGINE }

> @ekodb/ekodb-client@0.21.0 prepare
> npm run build


> @ekodb/ekodb-client@0.21.0 build
> tsc


up to date, audited 52 packages in 1s

17 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.21.0 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning TypeScript client library examples...[0m

added 1 package, removed 1 package, and audited 15 packages in 1s

3 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'lB0IWsbUI736phwZy8dspr3vKyUTqRwZHXML8f3vyTnPFpSFRqkwCHSb-igbWKDtxFbpRuJfQvtDTR4eJdvjuQ'
}

=== Find by ID ===
Found: {
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] },
  value: { value: 42, type: 'Integer' },
  name: { value: 'Test Record', type: 'String' },
  active: { value: true, type: 'Boolean' },
  id: 'lB0IWsbUI736phwZy8dspr3vKyUTqRwZHXML8f3vyTnPFpSFRqkwCHSb-igbWKDtxFbpRuJfQvtDTR4eJdvjuQ',
  created_at: { value: '2026-06-10T01:38:18.906Z', type: 'DateTime' },
  tags: { value: [ 'tag1', 'tag2', 'tag3' ], type: 'Array' },
  data: { value: 'aGVsbG8gd29ybGQ=', type: 'String' },
  metadata: { type: 'Object', value: { key: 'value', nested: [Object] } },
  price: { value: 99.99, type: 'Float' },
  categories: { value: [ 'electronics', 'computers' ], type: 'Array' },
  user_id: { value: '550e8400-e29b-41d4-a716-446655440000', type: 'String' }
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-06-10T01:38:18.906Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { key: 'value', nested: { deep: true } }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  value: 42,
  name: 'Test Record',
  active: true,
  id: 'lB0IWsbUI736phwZy8dspr3vKyUTqRwZHXML8f3vyTnPFpSFRqkwCHSb-igbWKDtxFbpRuJfQvtDTR4eJdvjuQ',
  created_at: '2026-06-10T01:38:18.906Z',
  tags: [ 'tag1', 'tag2', 'tag3' ],
  data: 'aGVsbG8gd29ybGQ=',
  metadata: { key: 'value', nested: { deep: true } },
  price: 99.99,
  categories: [ 'electronics', 'computers' ],
  user_id: '550e8400-e29b-41d4-a716-446655440000'
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  value: { value: 100, type: 'Integer' },
  tags: { value: [ 'tag1', 'tag2', 'tag3' ], type: 'Array' },
  id: 'lB0IWsbUI736phwZy8dspr3vKyUTqRwZHXML8f3vyTnPFpSFRqkwCHSb-igbWKDtxFbpRuJfQvtDTR4eJdvjuQ',
  active: { value: true, type: 'Boolean' },
  categories: { value: [ 'electronics', 'computers' ], type: 'Array' },
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  created_at: { value: '2026-06-10T01:38:18.906Z', type: 'DateTime' },
  price: { value: 99.99, type: 'Float' },
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' },
  name: { value: 'Updated Record', type: 'String' },
  metadata: { type: 'Object', value: { key: 'value', nested: [Object] } },
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: VJddsELseApKE-kgtFMz7YzlDtTtwEQAy37Kbyn_ghcifYLglEXWIplPOLq9KWuqWoil_61c-WuUKN-SBvL0aA

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
Collection created with first record: 0PaPNX9TnM2wyRetVTWT69IJaXYxaL6xl7i7LT54pKBTUQWQJzLjYncZKRdkAG0rkqLyWOVIfbMQG-87ICmhvg

=== List Collections ===
Total collections: 23
Sample collections: chat_messages__ek0_testing,chat_configurations__ek0_testing,enriched_users,schema_users_client_go,batch_users

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
Retrieved value: { userId: 123, username: 'john_doe' }

=== KV Batch Set ===
✓ Batch set 3 keys
  cache:product:1: success
  cache:product:2: success
  cache:product:3: success

=== KV Batch Get ===
✓ Batch retrieved 3 values
  cache:product:1: { price: 29.99, name: 'Product 1' }
  cache:product:2: { price: 39.99, name: 'Product 2' }
  cache:product:3: { name: 'Product 3', price: 49.99 }

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 3 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 12

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
Created Alice: $1000 - ID: rzo-DCGiH2BMs2j7eWRwcga7VrpyUbLKcc-GcP7GQoblJvil6WKo82ioFKVRg_53NaX_CX9KPKRGd2qs1U2hFA
Created Bob: $500 - ID: 8FDrSh2H7hXWuFyFVTG05BYw1G3uF_NEHbuGW38Svzi9anDGDv7qbCk6xL-HbnhD5OYVJS45iJHZ8U0-Bhbh3g

=== Example 1: Begin Transaction ===
Transaction ID: 0335989a-83a0-4e36-a477-58e5d7f09037

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 18bf3840-4eea-412c-b7f8-fabf811cec33
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
  1. Score: 25.740, Matched: name, email, name.value, email.value
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title.value, title, bio, bio.value
  2. Score: 26.400, Matched: bio, title, title.value, bio.value
  3. Score: 26.400, Matched: bio, bio.value, title.value, title
  4. Score: 26.400, Matched: bio, title.value, bio.value, title

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title.value, bio.value, bio, title
  2. Score: 39.600, Matched: title.value, title, bio.value, bio
  3. Score: 39.600, Matched: bio.value, title.value, bio, title
  4. Score: 39.600, Matched: bio.value, title, title.value, bio

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.735, Matched: 
  2. Score: 0.735, Matched: 
  3. Score: 0.721, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.688, Matched: title.value, content, title, content.value
  2. Score: 1.494, Matched: title.value, title, content.value, content
  3. Score: 0.294, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: bio, skills.value, bio.value, skills

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
Schema fields: [ 'age', 'email', 'name', 'status' ]
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
✓ Inserted document: 2c84dRcB9AJe9w0BNY3CHNs25s3xMn2MkOkbLJTCWFsswQMQhjKTtaq9b3Q5JW4RB1EmvDz96JFucDGxyCR9Cg

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: cdiMCcd9GDY4EHifBb3ChKvZ_Rq6pZa9YDtEoDHp_jNYyp2i0R6xh-cy9n3ikcyUYWdFX1cJ3BSi8XR37TETYg

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
✓ Inserted document with TTL: DsFkCzig84ncKrazZW20Ty4wzhaPmHOa0VT8XJqz5smzgBfmsZSC1L26VNGzxVq5Al7GX-SKXGq52BtHFdpKQg

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
✓ Edge cache script created: 4yheEeFL8mfNtySWXSURO53Kg8ombYJ2uEuJb-Zbu1OuKyZgQnFl_Y5xnLAyKky0En2-IWiknD5RHv5p9CUtJA

Call 1: Cache miss (fetches from API)
Response time: 160ms
Result: {
  "records": [
    {
      "value": {
        "value": {
          "email": "Sincere@april.biz",
          "id": 1,
          "address": {
            "street": "Kulas Light",
            "suite": "Apt. 556",
            "city": "Gwenborough",
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            },
            "zipcode": "92998-3874"
          },
          "name": "Leanne Graham",
          "company": {
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets"
          },
          "phone": "1-770-736-8031 x56442",
          "username": "Bret",
          "website": "hildegard.org"
        },
        "type": "Object"
      }
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

Call 2: Cache hit (served from ekoDB)
Response time: 42ms (3.8x faster!)
Result: {
  "records": [
    {
      "value": {
        "value": {
          "email": "Sincere@april.biz",
          "id": 1,
          "address": {
            "street": "Kulas Light",
            "suite": "Apt. 556",
            "city": "Gwenborough",
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            },
            "zipcode": "92998-3874"
          },
          "name": "Leanne Graham",
          "company": {
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets"
          },
          "phone": "1-770-736-8031 x56442",
          "username": "Bret",
          "website": "hildegard.org"
        },
        "type": "Object"
      }
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

=== The Magic ===
- Your DATABASE is your EDGE
- No Redis needed
- No CDN needed
- No cache invalidation logic needed (TTL handles it)
- With ripples: All nodes auto-sync cache
- One service: Database + Cache + Edge Functions

✓ Example complete!

🚀 ekoDB Functions Example (TypeScript)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved: dQ1aI0bPCLDEE5bfX8WCFhp-nHjJoH6YksLWWd_0KAmdCypNLYxfMxfl_Hp5IZjWahPC7CzzW_i2Mje6ISSPUQ
📊 Found 15 active users

📝 Example 2: Parameterized Function

✅ Function saved: Da_XLUV7yR0WgW77JrCmxsQ6KLs9amm4gmY-1d9utKNr7sKN0UiZyVlI_CAXKGdcjZSVRRv6mvysyCtlWbt80g
📊 Found 15 users (limited)

📝 Example 3: Aggregation Function

✅ Function saved: iLtF5fbLjcEiBYDC_99nCIlkNuXb5K_hNlnLOFbOEKq96kEfjx6NT3lWQpI8Ok6VlqbajrKtmiLVecym0R93HQ
📊 Statistics: 3 groups
   {"avg_score":{"type":"Float","value":50},"count":{"type":"Integer","value":5},"status":{"type":"String","value":"inactive"}}
   {"count":{"value":5,"type":"Integer"},"avg_score":{"type":"Float","value":60},"status":{"type":"String","value":"null"}}
   {"count":{"value":5,"type":"Integer"},"avg_score":{"value":60,"type":"Float"},"status":{"type":"String","value":"active"}}

📝 Example 4: UserFunction Management

📋 Total scripts: 12
🔍 Retrieved script: Get Active Users
✏️  function updated
🗑️  function deleted

ℹ️  Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.

✅ All examples completed!
=== ekoDB Function Composition Examples ===

📋 Setting up test data...

✅ Test data ready

📝 Example 1: Basic Function Composition

Building reusable functions that call each other...

Function 'fetch_user' already existed — updated instead
✅ Saved reusable function: fetch_user
Function 'get_user_wrapper' already existed — updated instead
✅ Saved composed function: get_user_wrapper (calls fetch_user + projects fields)

📊 Result from composed function:
   Records: 1
   Name: {"type":"String","value":"User 1"}
   Department: {"value":"engineering","type":"String"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

Function 'fetch_and_store_user' already existed — updated instead
✅ Saved reusable function: fetch_and_store_user (uses KV)
Function 'swr_user' already existed — updated instead
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 39ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "name": "Leanne Graham",
      "company": {
        "catchPhrase": "Multi-layered client-server neural-net",
        "bs": "harness real-time ...

Second call (cache hit - from cache):
   ⏱️  Duration: 39ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "name": "Leanne Graham",
      "company": {
        "catchPhrase": "Multi-layered client-server neural-net",
        "bs": "harness real-time e-markets",
        "n...
   🚀 Cache speedup: 1.0x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

Function 'validate_user' already existed — updated instead
✅ Level 1 function: validate_user
Function 'fetch_slim_user' already existed — updated instead
✅ Level 2 function: fetch_slim_user (calls validate_user)
Function 'get_verified_user' already existed — updated instead
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
🚀 ekoDB TypeScript Complete Functions Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Function saved: bybCx768JHZO-vi_KK-rUcCO3AVtdRcbtBgv8q5MJvp4fGrlLernKBhXLj-c0AxYRIepiyjMsAFNA01GtEUfdQ
📊 Found 2 product groups
   {"category":{"value":"Furniture","type":"String"},"count":{"value":2,"type":"Integer"},"avg_price":{"type":"Float","value":474}}
   {"count":{"value":3,"type":"Integer"},"category":{"type":"String","value":"Electronics"},"avg_price":{"type":"Float","value":575.6666666666666}}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   {"count":{"value":3,"type":"Integer"},"category":{"type":"String","value":"Electronics"}}
   {"category":{"value":"Furniture","type":"String"},"count":{"type":"Integer","value":2}}
⏱️  Execution time: 0ms

📝 Example 4: High Rating Products

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 5: UserFunction with Parameter Definition

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
🚀 ekoDB TypeScript KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: LHWJkaHHR6huu5Azbzuog_qWLjG8bmMkKO9mv_VaHDIUTf1Duv3D-hbOcguPQpAgBDFXLviP1_pc53IdmcT-_g
✅ Inserted 2 products with wrapped types

📝 Example 2: UserFunction with Wrapped Type Parameters

✅ Function saved: S7wyBTWqoaCCJ0q9mbfWWuveV4zx0ZqIrvoPiD1w_wDa-yN3JA29F3rDo3kI-boHLvFdfe_5w8rTXcxd88zV3w
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"role":"admin","userId":"user_abc"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: FVfweXPq-oLq6gIzV42COYjed51BVe5mr0CtnHRHseudhJWb7-FlOg4zIGc_-XKtJBt39QG0nwh1GEbuaXsmbQ
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Function

✅ Function saved: lsSh_YyRzFlvDi9Rggj27xt1tP22Ab9MkO80rjV_OwSUFLIvzfvnLdfu631q7CFd2furNhZSE3jhQvsdyPKu_w
📊 Processed order with caching
⏱️  Stages executed: 3
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All KV & Wrapped Types examples completed!

💡 Key takeaways:
   ✅ Use Field.* helpers for type-safe wrapped values
   ✅ Field.decimal() preserves precision (no floating point errors)
   ✅ KV store is great for caching and quick lookups
   ✅ Stage.kv*() functions work within scripts
   ✅ Combine KV caching with collection inserts for real workflows
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_api_user (dUAXJZFgqzhKn0tcMC_hAbRFaPTGznfQVpGp40sC8e1Xl6-MyxQ7EHX4STqGIp4k_SZvp1eCmtFSD2tBS9oh2A)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "value": {
          "name": "Leanne Graham",
          "address": {
            "street": "Kulas Light",
            "zipcode": "92998-3874",
            "city": "Gwenborough",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "suite": "Apt. 556"
          },
          "username": "Bret",
          "id": 1,
          "company": {
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net"
          },
          "phone": "1-770-736-8031 x56442",
          "website": "hildegard.org",
          "email": "Sincere@april.biz"
        },
        "type": "Object"
      }
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
Response time: 41ms (served from cache)
Result (cached): {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "name": "Leanne Graham",
          "address": {
            "street": "Kulas Light",
            "zipcode": "92998-3874",
            "city": "Gwenborough",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "suite": "Apt. 556"
          },
          "username": "Bret",
          "id": 1,
          "company": {
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net"
          },
          "phone": "1-770-736-8031 x56442",
          "website": "hildegard.org",
          "email": "Sincere@april.biz"
        }
      }
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
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_with_reviews (EMXQfpv7YaxCLgC2wtQsEsrp8zGQipulELZ0OybLZtDgHDGgyrb0ArqiISXbqj25HOXOuFyQS9Eoon6kXYS-Ag)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
          "rating": 2.56,
          "discountPercentage": 10.48,
          "title": "Essence Mascara Lash Princess",
          "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
          "warrantyInformation": "1 week warranty",
          "minimumOrderQuantity": 48,
          "reviews": [
            {
              "reviewerEmail": "eleanor.collins@x.dummyjson.com",
              "comment": "Would not recommend!",
              "rating": 3,
              "date": "2025-04-30T09:41:02.053Z",
              "reviewerName": "Eleanor Collins"
            },
            {
              "reviewerEmail": "lucas.gordon@x.dummyjson.com",
              "comment": "Very satisfied!",
              "rating": 4,
              "date": "2025-04-30T09:41:02.053Z",
              "reviewerName": "Lucas Gordon"
            },
            {
              "reviewerName": "Eleanor Collins",
              "comment": "Highly impressed!",
              "date": "2025-04-30T09:41:02.053Z",
              "rating": 5,
              "reviewerEmail": "eleanor.collins@x.dummyjson.com"
            }
          ],
          "id": 1,
          "stock": 99,
          "brand": "Essence",
          "sku": "BEA-ESS-ESS-001",
          "weight": 4,
          "returnPolicy": "No return policy",
          "category": "beauty",
          "availabilityStatus": "In Stock",
          "dimensions": {
            "depth": 22.99,
            "width": 15.14,
            "height": 13.08
          },
          "meta": {
            "qrCode": "https://cdn.dummyjson.com/public/qr-code.png",
            "barcode": "5784719087687",
            "updatedAt": "2025-04-30T09:41:02.053Z",
            "createdAt": "2025-04-30T09:41:02.053Z"
          },
          "tags": [
            "beauty",
            "mascara"
          ],
          "price": 9.99,
          "images": [
            "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
          ],
          "shippingInformation": "Ships in 3-5 business days"
        }
      }
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
✓ Multi-API data fetched, merged, and cached atomically

=== Why This Is Powerful ===

✓ No separate cache layer (Redis, Memcached) needed
✓ No manual cache invalidation (TTL handles it)
✓ No separate edge infrastructure (ekoDB IS the edge)
✓ Atomic operations (function executes as transaction)
✓ With multi-node + ripples: Auto-sync across all nodes
✓ Sub-millisecond cache hits from internal storage
✓ One service instead of many (cache + API gateway + database)

=== Real-World Use Cases ===

1. API Gateway Pattern:
   - Client → ekoDB Function → Check cache → Call microservices → Merge → Cache

2. Database Federation:
   - Query multiple DBs (Postgres, MongoDB) + external APIs
   - Merge results in one function call

3. IoT Data Enrichment:
   - Sensor data + weather API + location API
   - Enrich and cache in one atomic operation

4. E-commerce Product Pages:
   - Product info + reviews + inventory + pricing
   - All from different sources, cached together

✓ Example complete - Your database IS your edge!

🚀 ekoDB TypeScript Client - Native SWR Function Examples

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
✓ Created native SWR script: github_user_native (ARpJ05PjcWqAlHhaUkPro9gFnzdNJTPxvhFCNHZEj8uZlzdBEI_u9EBhO_0m3B9onLrgTnRiQQ8CvcUjsoEolQ)

First call (cache miss - will fetch from GitHub API):
  Response time: 16ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 16ms
  Speedup: 1.0x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (2fSppb5aszjd3sX8SZN6eTZHQUeySMFCE_tVn1R8jbeGG1L0vbPITsrvG1qFSQLls7goJ7Dzz-D9P-kFmwVSWA)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (cDlvHv8lrrAhWNB1YzsUiGoDRX5xUvdy4lWbEyvzu8YN3thWsCRqBAMWkJuELhALdqxz91Gk_AKyJIubL76erQ)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (ish_eBdn8IUc529iUF4iwUvTlnclCRzGtRYSa8xCZ-A7BFaqlagOXl2InbRup3ne_FAz_g4G4rpKPl4KNzmgSQ)
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
🚀 ekoDB TypeScript Advanced Functions Example

📋 Setting up test data...
✅ Created 8 products

📝 Example 1: List All Products

✅ Function saved
📊 Found 8 products
⏱️  Execution time: 0ms

📝 Example 2: Group Products by Category

✅ Function saved
📊 Category breakdown:
   {"count":{"type":"Integer","value":5},"category":{"type":"String","value":"Electronics"},"avg_price":{"type":"Float","value":367}}
   {"count":{"value":3,"type":"Integer"},"avg_price":{"type":"Float","value":365.6666666666667},"category":{"value":"Furniture","type":"String"}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced script examples finished!
🚀 ekoDB TypeScript AI Functions Example

📋 Setting up test data...
✅ Created 2 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
🤖 AI Response:
   Vector databases offer several benefits:

1. **Efficient Similarity Search**: They enable fast retrieval of similar items using vector embeddings, which is essential for applications like recommendation systems and image search.

2. **Scalability**: Vector databases can handle large volumes of high-dimensional data, making them suitable for big data applications.

3. **Support for Machine Learning**: They integrate well with machine learning workflows, allowing storage and querying of complex data types like text, images, and audio.

4. **Semantic Search**: Vector databases provide enhanced search capabilities by understanding contextual similarities rather than relying solely on keyword matches.

5. **Real-time Processing**: They facilitate real-time analytics and decision-making by enabling quick access to relevant data.

6. **Flexibility**: They can work seamlessly with various data types and structures, allowing for diverse use cases in different domains.

7. **Dimensionality Reduction**: Many vector databases support techniques to reduce the dimensionality of data, improving efficiency without sacrificing information. 

Overall, they enhance the performance of applications that require advanced data retrieval and analysis.
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
🚀 ekoDB TypeScript CRUD Functions Example

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
   {"role":{"type":"String","value":"admin"},"avg_score":{"type":"Float","value":20},"count":{"value":3,"type":"Integer"}}
   {"role":{"value":"user","type":"String"},"count":{"value":7,"type":"Integer"},"avg_score":{"type":"Float","value":70}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD script examples finished!
🚀 ekoDB TypeScript Search Functions Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Function saved
📊 Found 5 documents
   1. Vector Databases Explained (Database)
   2. Database Design Principles (Database)
   3. Getting Started with ekoDB (Database)
   4. Natural Language Processing (AI)
   5. Introduction to Machine Learning (AI)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   {"count":{"type":"Integer","value":2},"category":{"value":"AI","type":"String"}}
   {"count":{"value":3,"type":"Integer"},"category":{"value":"Database","type":"String"}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: Pn8r_isvnjZVuq5VU-lwW6dtY8yitGZAlms9dpC2ETllmSDz9lhBe9hrs36T9feYNukoliHZb83qTuJNKD1h_w

=== Sending Chat Message ===
Message ID: Suu7PMlhwfmqSB8CC0zCg_JwT3rW_n47FnwTsRzs2F2YmPP8FfHewgRo-Z9TI3m4h_iX-aw9vWtNz4MiTuysyQ

=== AI Response ===
Here are the available products along with their prices:

1. **ekoDB Pro**
   - Price: $299
   - Description: Enterprise edition product with advanced features

2. **ekoDB**
   - Price: $99
   - Description: A high-performance database product with AI capabilities

3. **ekoDB Cloud**
   - Price: $499
   - Description: Fully managed cloud database service product

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    price: 299,
    name: 'ekoDB Pro',
    description: 'Enterprise edition product with advanced features',
    id: 'ke46ZQjIrHt-6V6J7uZpf1m1W0a-f-xIszcbl_YXuSDtLxDdLCZahOWffsmiSI41bFH-O_msjNgmVYESIYOqiQ'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    price: 99,
    name: 'ekoDB',
    description: 'A high-performance database product with AI capabilities',
    id: 'LfKjScPlQ543Y_C-HxJLjNKLricOxCfljkHVUJlMTJTbrMKCG08WxleSX6OCuOVhi9f2C1KFIFW2Ll3_Y3_f7Q'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    name: 'ekoDB Cloud',
    id: 'V7hy5iTBWF7NMV_hfEzlc4EsUm7QUQrsWupPKlApSJMEgRGbhTIHWsawANHBNkvJU96xWPg7fV1WgZB1MuRDXQ',
    price: 499,
    description: 'Fully managed cloud database service product'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 7207ms

=== Token Usage ===
Prompt tokens: 3468
Completion tokens: 91
Total tokens: 3559

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 0HBA6NFJ5dNypv6Xuxr0s1mkWXLxP6-cOTsglK-EaybKKc90DZ3u_yPtkvj-EIjKCca7CcQXbIWPZl-twnUfBA

=== Sending Initial Message ===
✓ Message sent
  Response: It appears that there are currently no products available in the database. The specific query I executed did not return any records. 

If you'd like me to check for different criteria or provide more information on available products, please let me know!

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
✓ Created second session: sZ_pFtv-SgjN5qRqTCaVXKCAC8pSGDLdZfvZn0vhQYogZ3WN1oAJaiQ2QDYSwfSHBYwGasbWpOF0TVZpseAlLg
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
✓ Created session: kG2_CcjINKxJBETgxlXwzU5mswVn09wcpbRMt40aZYsilk5nYPkcaNZhtTzRBmIFBDdVP-Bv2ezLSAqU5IkPVA

=== Sending Messages ===
✓ Message 1 sent
  Response: The only product currently available is:

- **Name:** ekoDB
- **Description:** A high-performance database product
- **Price:** $99

If you need more information or have any specific queries regarding this product, feel free to ask!

✓ Message 2 sent
  Response: The price of ekoDB is $99. If you have any more questions or need further assistance, feel free to ask!

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: PJey8-qgnlAP4mK98B8uVp8A1psw-J747Y_sq578Sit7dNmLM_WAMgOAkMLO_K3c-YuTYrcxlygqOflS750T7w
  Parent: kG2_CcjINKxJBETgxlXwzU5mswVn09wcpbRMt40aZYsilk5nYPkcaNZhtTzRBmIFBDdVP-Bv2ezLSAqU5IkPVA

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: PJey8-qgnlAP4mK98B8uVp8A1psw-J747Y_sq578Sit7dNmLM_WAMgOAkMLO_K3c-YuTYrcxlygqOflS750T7w (Untitled)
  Session 2: kG2_CcjINKxJBETgxlXwzU5mswVn09wcpbRMt40aZYsilk5nYPkcaNZhtTzRBmIFBDdVP-Bv2ezLSAqU5IkPVA (Untitled)
  Session 3: sZ_pFtv-SgjN5qRqTCaVXKCAC8pSGDLdZfvZn0vhQYogZ3WN1oAJaiQ2QDYSwfSHBYwGasbWpOF0TVZpseAlLg (Untitled)
  Session 4: Pn8r_isvnjZVuq5VU-lwW6dtY8yitGZAlms9dpC2ETllmSDz9lhBe9hrs36T9feYNukoliHZb83qTuJNKD1h_w (Untitled)
  Session 5: zQ9sPZrEwJouHWdIPzV9BxVUsjLgEVWPvg3BrCKx8qpphMyfN-7PLO8sytR8_4cyYxwTn6ULgBUTvM1x5lGCPQ (Untitled)
  Session 6: 02WYH5fvFCE8jEaFL6l-ZbPq2sEWjD9Le24i-ON89iHVdcsM7uDa9qzgXiCclIs5t99xOqTR6XzzdlGuMjzC8A (Untitled)
  Session 7: XMcm9EI1r0XL4MrLSKCmrJUS-QsNU0ih0n75U0Gmv1MumkPwb3ie_6eM7UNzTVyEipHHZKNJ0NDjQed-w7ci4Q (Untitled)
  Session 8: arDUIy0JvWFMhNR6jVL2eXTmmA8sAWo7vbbVrTChdH5D5YqwuC8lHzluHFaE8OIBJfBDRPAPZbyxjVy30ZKX7A (Untitled)
  Session 9: IMk5O_Jl94E_vRIrr-4t-mn7L9CpF2IyC5Z0Ef83_ZaCDyZSCJ89KDGpGXX2_R1t3gfRiIBRPdi5_MZmmySl_w (Untitled)
  Session 10: _xiMGVjJpu7r4vWBMXRFRHvuAvR2EBTWY8KdSC_AcOwC4PwMb97zKEra0DwnBDRctEAN6RSpdCdwj7-4hPnZTA (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: PJey8-qgnlAP4mK98B8uVp8A1psw-J747Y_sq578Sit7dNmLM_WAMgOAkMLO_K3c-YuTYrcxlygqOflS750T7w

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Object Creation ===
✓ Created record with plain object: {
  id: 'BehXSoObhovCo1uLSD6QnnP0L22FVUW_UxoATb1CjTq3ex6AJj5qRmhzr19KJkBiKHI5F2SH7H8a-ZwFmegQtQ'
}

=== Upsert Operation ===
✓ First upsert (update): {
  active: { value: true, type: 'Boolean' },
  age: { value: 29, type: 'Integer' },
  name: { value: 'Alice Johnson', type: 'String' },
  id: 'BehXSoObhovCo1uLSD6QnnP0L22FVUW_UxoATb1CjTq3ex6AJj5qRmhzr19KJkBiKHI5F2SH7H8a-ZwFmegQtQ',
  email: { value: 'alice.j@newdomain.com', type: 'String' }
}
✓ Second upsert (insert): {
  id: 'I42AhZLcRO8MyhzPkFVlHVa5kfFH8xEm9xl1pz0iu5yaTPPPqpcZG18Njdzaph3xBRUjuUgLMbrjARWxeAP-jw'
}

=== Find One Operation ===
✓ Found user by email: {
  name: { value: 'Alice Johnson', type: 'String' },
  active: { value: true, type: 'Boolean' },
  age: { value: 29, type: 'Integer' },
  email: { type: 'String', value: 'alice.j@newdomain.com' },
  id: 'BehXSoObhovCo1uLSD6QnnP0L22FVUW_UxoATb1CjTq3ex6AJj5qRmhzr19KJkBiKHI5F2SH7H8a-ZwFmegQtQ'
}
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
=== Bypass Ripple Example ===

1. Basic insert (ripple enabled):
   Inserted with ripple: {"id":"QCFG8fJXQYixVN6Ah8E0bev7vz2LZX3ojzblixKQkpAMrhn9qsjqL3IeDWDCrsrizBCxOTA2aTdKml2T5ZHGdQ"}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {"id":"UzH3Rb4qh7ZSdsB9DUYOpWqHJOyWK62LMbcltao9fVIiKLQb2hDyzUUAuaS7z_1IxR7WrsS48syeUgfpgz164A"}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {"name":{"type":"String","value":"Product 1"},"price":{"value":150,"type":"Integer"},"id":"QCFG8fJXQYixVN6Ah8E0bev7vz2LZX3ojzblixKQkpAMrhn9qsjqL3IeDWDCrsrizBCxOTA2aTdKml2T5ZHGdQ"}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {"id":"custom-id","name":{"type":"String","value":"Upsert Product"},"price":{"value":500,"type":"Integer"}}

✅ All bypass_ripple operations completed successfully!
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields (id, name, email only)
  Found 3 active users
  Fields returned: ["id","name","email"]
  First user: [object Object] <[object Object]>

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ["age","avatar_url","created_at","bio","email","name","id","status","user_role"]

Example 3: Complex query with projection (active users, ages 18-65)
  Found 3 active users (ages 18-65)
    - [object Object] (age [object Object])
    - [object Object] (age [object Object])
    - [object Object] (age [object Object])

Example 4: Query inactive users with profile fields
  Found 1 inactive users
    - [object Object]: [object Object]

Example 5: Compare full vs projected data
  Full query:
    - 12 fields per record
    - Fields: ["api_key","status","email","password","bio","user_role","avatar_url","age","id","name","created_at","secret_token"]
  Projected query:
    - 3 fields per record
    - Fields: ["id","email","name"]
  Bandwidth savings: ~75% fewer fields

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
=== KV Precision: Float vs Decimal ===

=== Test 1: Using JavaScript Floats (LOSES PRECISION) ===
✓ Stored products with float prices

Retrieved float prices:
  Widget A: $29.99 (expected $29.99) ✓
  Widget B: $39.99 (expected $39.99) ✓
  Widget C: $49.99 (expected $49.99) ✓

=== Test 2: Using Field.decimal() (PRESERVES PRECISION) ===
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
✅ Use Field.decimal() for monetary values, percentages, and
   any case where floating-point errors are unacceptable.
✅ Field.decimal() stores values as strings internally,
   preserving exact precision across all operations.
✓ Client created

=== List Chat Models ===
Available chat models by provider:
  openai:
    - text-embedding-ada-002
    - whisper-1
    - gpt-3.5-turbo
    - tts-1
    - gpt-3.5-turbo-16k
    - gpt-4-0613
    - gpt-4
    - davinci-002
    - babbage-002
    - gpt-3.5-turbo-instruct
    - gpt-3.5-turbo-instruct-0914
    - gpt-3.5-turbo-1106
    - tts-1-hd
    - tts-1-1106
    - tts-1-hd-1106
    - text-embedding-3-small
    - text-embedding-3-large
    - gpt-3.5-turbo-0125
    - gpt-4-turbo
    - gpt-4-turbo-2024-04-09
    - gpt-4o
    - gpt-4o-2024-05-13
    - gpt-4o-mini-2024-07-18
    - gpt-4o-mini
    - gpt-4o-2024-08-06
    - omni-moderation-latest
    - omni-moderation-2024-09-26
    - o1-2024-12-17
    - o1
    - o3-mini
    - o3-mini-2025-01-31
    - gpt-4o-2024-11-20
    - gpt-4o-mini-search-preview-2025-03-11
    - gpt-4o-mini-search-preview
    - gpt-4o-transcribe
    - gpt-4o-mini-transcribe
    - o1-pro-2025-03-19
    - o1-pro
    - gpt-4o-mini-tts
    - o3-2025-04-16
    - o4-mini-2025-04-16
    - o3
    - o4-mini
    - gpt-4.1-2025-04-14
    - gpt-4.1
    - gpt-4.1-mini-2025-04-14
    - gpt-4.1-mini
    - gpt-4.1-nano-2025-04-14
    - gpt-4.1-nano
    - gpt-image-1
    - o4-mini-deep-research
    - gpt-4o-transcribe-diarize
    - o4-mini-deep-research-2025-06-26
    - gpt-5-chat-latest
    - gpt-5-2025-08-07
    - gpt-5
    - gpt-5-mini-2025-08-07
    - gpt-5-mini
    - gpt-5-nano-2025-08-07
    - gpt-5-nano
    - gpt-audio-2025-08-28
    - gpt-realtime
    - gpt-realtime-2025-08-28
    - gpt-audio
    - gpt-5-codex
    - gpt-image-1-mini
    - gpt-5-pro-2025-10-06
    - gpt-5-pro
    - gpt-audio-mini
    - gpt-audio-mini-2025-10-06
    - gpt-5-search-api
    - gpt-realtime-mini
    - gpt-realtime-mini-2025-10-06
    - sora-2
    - sora-2-pro
    - gpt-5-search-api-2025-10-14
    - gpt-5.1-chat-latest
    - gpt-5.1-2025-11-13
    - gpt-5.1
    - gpt-5.1-codex
    - gpt-5.1-codex-mini
    - gpt-5.1-codex-max
    - gpt-image-1.5
    - gpt-5.2-2025-12-11
    - gpt-5.2
    - gpt-5.2-pro-2025-12-11
    - gpt-5.2-pro
    - gpt-5.2-chat-latest
    - gpt-4o-mini-transcribe-2025-12-15
    - gpt-4o-mini-transcribe-2025-03-20
    - gpt-4o-mini-tts-2025-03-20
    - gpt-4o-mini-tts-2025-12-15
    - gpt-realtime-mini-2025-12-15
    - gpt-audio-mini-2025-12-15
    - chatgpt-image-latest
    - gpt-5.2-codex
    - gpt-5.3-codex
    - gpt-realtime-1.5
    - gpt-audio-1.5
    - gpt-4o-search-preview
    - gpt-4o-search-preview-2025-03-11
    - gpt-5.3-chat-latest
    - gpt-5.4-2026-03-05
    - gpt-5.4-pro
    - gpt-5.4-pro-2026-03-05
    - gpt-5.4
    - gpt-5.4-nano-2026-03-17
    - gpt-5.4-nano
    - gpt-5.4-mini-2026-03-17
    - gpt-5.4-mini
    - gpt-image-2
    - gpt-image-2-2026-04-21
    - gpt-5.5
    - gpt-5.5-2026-04-23
    - gpt-5.5-pro
    - gpt-5.5-pro-2026-04-23
    - chat-latest
    - gpt-realtime-translate
    - gpt-realtime-2
    - gpt-realtime-whisper
  anthropic:
    - claude-fable-5
    - claude-opus-4-8
    - claude-opus-4-7
    - claude-sonnet-4-6
    - claude-opus-4-6
    - claude-opus-4-5-20251101
    - claude-haiku-4-5-20251001
    - claude-sonnet-4-5-20250929
    - claude-opus-4-1-20250805
    - claude-opus-4-20250514
    - claude-sonnet-4-20250514
  perplexity:
    - sonar
    - sonar-pro
    - sonar-deep-research
    - sonar-reasoning
    - sonar-reasoning-pro

=== Get Specific Provider Models ===
OpenAI models: text-embedding-ada-002, whisper-1, gpt-3.5-turbo, tts-1, gpt-3.5-turbo-16k, gpt-4-0613, gpt-4, davinci-002, babbage-002, gpt-3.5-turbo-instruct, gpt-3.5-turbo-instruct-0914, gpt-3.5-turbo-1106, tts-1-hd, tts-1-1106, tts-1-hd-1106, text-embedding-3-small, text-embedding-3-large, gpt-3.5-turbo-0125, gpt-4-turbo, gpt-4-turbo-2024-04-09, gpt-4o, gpt-4o-2024-05-13, gpt-4o-mini-2024-07-18, gpt-4o-mini, gpt-4o-2024-08-06, omni-moderation-latest, omni-moderation-2024-09-26, o1-2024-12-17, o1, o3-mini, o3-mini-2025-01-31, gpt-4o-2024-11-20, gpt-4o-mini-search-preview-2025-03-11, gpt-4o-mini-search-preview, gpt-4o-transcribe, gpt-4o-mini-transcribe, o1-pro-2025-03-19, o1-pro, gpt-4o-mini-tts, o3-2025-04-16, o4-mini-2025-04-16, o3, o4-mini, gpt-4.1-2025-04-14, gpt-4.1, gpt-4.1-mini-2025-04-14, gpt-4.1-mini, gpt-4.1-nano-2025-04-14, gpt-4.1-nano, gpt-image-1, o4-mini-deep-research, gpt-4o-transcribe-diarize, o4-mini-deep-research-2025-06-26, gpt-5-chat-latest, gpt-5-2025-08-07, gpt-5, gpt-5-mini-2025-08-07, gpt-5-mini, gpt-5-nano-2025-08-07, gpt-5-nano, gpt-audio-2025-08-28, gpt-realtime, gpt-realtime-2025-08-28, gpt-audio, gpt-5-codex, gpt-image-1-mini, gpt-5-pro-2025-10-06, gpt-5-pro, gpt-audio-mini, gpt-audio-mini-2025-10-06, gpt-5-search-api, gpt-realtime-mini, gpt-realtime-mini-2025-10-06, sora-2, sora-2-pro, gpt-5-search-api-2025-10-14, gpt-5.1-chat-latest, gpt-5.1-2025-11-13, gpt-5.1, gpt-5.1-codex, gpt-5.1-codex-mini, gpt-5.1-codex-max, gpt-image-1.5, gpt-5.2-2025-12-11, gpt-5.2, gpt-5.2-pro-2025-12-11, gpt-5.2-pro, gpt-5.2-chat-latest, gpt-4o-mini-transcribe-2025-12-15, gpt-4o-mini-transcribe-2025-03-20, gpt-4o-mini-tts-2025-03-20, gpt-4o-mini-tts-2025-12-15, gpt-realtime-mini-2025-12-15, gpt-audio-mini-2025-12-15, chatgpt-image-latest, gpt-5.2-codex, gpt-5.3-codex, gpt-realtime-1.5, gpt-audio-1.5, gpt-4o-search-preview, gpt-4o-search-preview-2025-03-11, gpt-5.3-chat-latest, gpt-5.4-2026-03-05, gpt-5.4-pro, gpt-5.4-pro-2026-03-05, gpt-5.4, gpt-5.4-nano-2026-03-17, gpt-5.4-nano, gpt-5.4-mini-2026-03-17, gpt-5.4-mini, gpt-image-2, gpt-image-2-2026-04-21, gpt-5.5, gpt-5.5-2026-04-23, gpt-5.5-pro, gpt-5.5-pro-2026-04-23, chat-latest, gpt-realtime-translate, gpt-realtime-2, gpt-realtime-whisper

=== Get Anthropic Models ===
Anthropic models: claude-fable-5, claude-opus-4-8, claude-opus-4-7, claude-sonnet-4-6, claude-opus-4-6, claude-opus-4-5-20251101, claude-haiku-4-5-20251001, claude-sonnet-4-5-20250929, claude-opus-4-1-20250805, claude-opus-4-20250514, claude-sonnet-4-20250514

=== Get Non-Existent Provider ===
Expected error for non-existent provider: Error: Request failed with status 405: {"code":405,"message":"Method Not Allowed"}

✓ Chat Models example complete
✓ Client created

=== Create User Function ===
Created user function with ID: 9i9K4pZBZw-gw9eFNNTarww5T3LwJDq7-oZp-Z5obov4EvNKqD0usfB_llBKy0XR6x-7MglIGOfM_tDno5fbzA

=== Get User Function ===
Retrieved: get_active_users_ts - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 14 user functions:
  - validate_user: Check if user exists
  - cache_api_call_py: Cache External API Call
  - fetch_slim_user: Validate and slim down user
  - get_users_by_status: Get Users By Status
  - get_verified_user: Get verified and validated user
  - fetch_user: Fetch user by code
  - swr_user: SWR pattern for user data (KV-based)
  - fetch_api_user: Fetch User with Cache
  - get_active_users_ts: Get Active Users
  - fetch_product_with_reviews: Fetch Product with Reviews (Multi-API)
  - get_active_users_updated: Get Active Users (Updated)
  - get_user_wrapper: Wrapper that calls fetch_user
  - fetch_and_store_user: Fetch user from API and cache in KV
  - cache_api_call: Cache External API Call

=== Update User Function ===
User function updated successfully

=== Delete User Function ===
User function deleted successfully

✓ User Functions API example complete
✓ Client created

=== Check Collection Exists (Before Creation) ===
Collection 'collection_utils_test_ts' exists: false

=== Creating Test Documents ===
Created 5 test documents

=== Check Collection Exists (After Creation) ===
Collection 'collection_utils_test_ts' exists: true

=== Count Documents ===
Document count in 'collection_utils_test_ts': 5

=== Check Non-Existent Collection ===
Collection 'nonexistent_collection_xyz' exists: false

=== Cleanup ===
Deleted collection 'collection_utils_test_ts'

✓ Collection Utilities example complete
✓ Client created
✓ ts_users_register saved
✓ ts_users_login saved
✓ ts_users_verify_token saved

=== Auth flow defined as pure stored functions ===
Call them like:
  POST /api/functions/ts_users_register { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/ts_users_login    { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/ts_users_verify_token { "token": "<jwt>" }

Set JWT_SECRET in ekoDB's environment_vars whitelist before invoking.

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
✓ ts_route_admin → GET /api/route/users/admin
✓ ts_route_user_by_id → GET /api/route/users/:id
✓ ts_route_user_posts → GET /api/route/users/:id/posts/:post_id
✓ ts_route_org_create_member → POST /api/route/orgs/:org/members

Try them with curl:
  curl http://localhost:8080/api/route/users/admin
  curl http://localhost:8080/api/route/users/42
  curl http://localhost:8080/api/route/users/42/posts/7
  curl -X POST http://localhost:8080/api/route/orgs/acme/members \
       -H 'Content-Type: application/json' -d '{"name":"alice"}'

✓ Cleaned up demo functions
✅ [32mTypeScript client examples complete![0m
✅ [32mAll TypeScript integration tests complete![0m
🧪 [36mRunning JavaScript examples (direct HTTP/WebSocket)...[0m

added 1 package, removed 1 package, and audited 9 packages in 1s

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
[34m
╔════════════════════════════════════════╗[0m
[34m║  ekoDB JavaScript Examples Test Suite ║[0m
[34m╚════════════════════════════════════════╝[0m
[34m
=== Checking Server Connection ===[0m
(node:54920) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
[32m✓ Server is ready[0m
[34m
=== Getting Authentication Token ===[0m
[32m✓ Authentication successful[0m
[33m
=== Running 10 Examples ===[0m
[34m
=== Running ekoDB/ekodb-client/examples/javascript/simple_crud.js ===[0m
=== Simple CRUD Operations (Direct HTTP) ===

✓ Authentication successful

=== Insert Document ===
Inserted: {
  id: 'Uo_PJNopErHZmYIKu3SYwNx61ZN348H5YPHGEiXh0CJfSrGZjjpjNVAAaApCF7uTrXUC6MUOj_eWc__MEumT1Q'
}

=== Find by ID ===
Found: {
  active: { type: 'Boolean', value: true },
  id: 'Uo_PJNopErHZmYIKu3SYwNx61ZN348H5YPHGEiXh0CJfSrGZjjpjNVAAaApCF7uTrXUC6MUOj_eWc__MEumT1Q',
  name: { type: 'String', value: 'Test Record' },
  value: { value: 42, type: 'Integer' }
}

=== Find with Query ===
Found documents: [
  {
    name: { type: 'String', value: 'Test Record' },
    id: 'Uo_PJNopErHZmYIKu3SYwNx61ZN348H5YPHGEiXh0CJfSrGZjjpjNVAAaApCF7uTrXUC6MUOj_eWc__MEumT1Q',
    active: { value: true, type: 'Boolean' },
    value: { value: 42, type: 'Integer' }
  }
]

=== Update Document ===
Updated: {
  name: { type: 'String', value: 'Updated Record' },
  active: { type: 'Boolean', value: true },
  value: { type: 'Integer', value: 100 },
  id: 'Uo_PJNopErHZmYIKu3SYwNx61ZN348H5YPHGEiXh0CJfSrGZjjpjNVAAaApCF7uTrXUC6MUOj_eWc__MEumT1Q'
}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
[32m✓ simple_crud.js completed successfully[0m
[34m
=== Running ekoDB/ekodb-client/examples/javascript/simple_websocket.js ===[0m
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: 010zRLQgygy1ONFdh9V7fFkw397eoYqDJd9k6BUXMlOczqRGT8bEMmJhmo9TYhTrotBNDN4srUcV-rq4aRO1UA

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
Response: {
  "type": "Success",
  "payload": {
    "data": [
      {
        "value": {
          "type": "Integer",
          "value": 42
        },
        "id": "010zRLQgygy1ONFdh9V7fFkw397eoYqDJd9k6BUXMlOczqRGT8bEMmJhmo9TYhTrotBNDN4srUcV-rq4aRO1UA",
        "name": {
          "type": "String",
          "value": "WebSocket Test Record"
        },
        "active": {
          "type": "Boolean",
          "value": true
        }
      },
      {
        "value": {
          "value": 42,
          "type": "Integer"
        },
        "id": "Rx_EhUtGO6iii80P81nw7Bu_teSO3QUJk0QQ5tqdYHt_E7rl36XSAj6fL0Fa5ZVfaAmRGe0SxQ4uDM2G7ETLTQ",
        "active": {
          "value": true,
          "type": "Boolean"
        },
        "name": {
          "type": "String",
          "value": "WebSocket Test Record"
        }
      },
      {
        "name": {
          "value": "WebSocket Test Record",
          "type": "String"
        },
        "id": "CKITd5EqDQjiV8hLrfhGPrj49ZBeSaAjnkyrLO7f0qy6xIDC3sWKcXH52YZBaj6wsf3LX2u00aSr-38Iom4dqA",
        "active": {
          "type": "Boolean",
          "value": true
        },
        "value": {
          "type": "Integer",
          "value": 42
        }
      },
      {
        "active": {
          "value": true,
          "type": "Boolean"
        },
        "name": {
          "type": "String",
          "value": "WebSocket Test Record"
        },
        "id": "YM-G28Ye5HW60hW5TeagNc7tHBRMMevw6db7Q2dkuPwrAlFowwaaQe-qkTQlUak-0DSJviThzKxxSPS9vsDxZA",
        "value": {
          "type": "Integer",
          "value": 42
        }
      }
    ]
  },
  "messageId": "1781055552834"
}
✓ Retrieved 4 record(s) via WebSocket

✓ WebSocket example completed successfully
WebSocket closed
[32m✓ simple_websocket.js completed successfully[0m
[34m
=== Running ekoDB/ekodb-client/examples/javascript/http_functions.js ===[0m
🚀 ekoDB Functions Example (JavaScript/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Function saved: rrZ9SBBuolQ2lvN3Q5dvHRMfTnbUxRXUs0tmkPc66DwaC_W_mZNCe_lKPOwdjPwzGt9pb-p0jObSoBffAgtuIg
📊 Found 10 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Function saved: Hvyu1Iuj3aPxkEIL1ZCAh4Gp-R_qM6gXHP1SylVdXsa4ChU6O82K6IcWgttXKH1nj9ZxVuwGCcfrWL0YT9k6vQ
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Function saved: lF6WbxCTBlZlBJ5oJwGSUhaT3BRl8SCUj-lYrHHLE180u0E_LTznwfef-wufCDTWRwd--eoZrxLQzwGJskrVFQ
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"count":{"value":10,"type":"Integer"},"max_score":{"type":"Integer","value":100},"status":{"value":"active","type":"String"},"avg_score":{"value":60,"type":"Float"}}
   {"status":{"type":"String","value":"inactive"},"max_score":{"type":"Integer","value":90},"count":{"type":"Integer","value":10},"avg_score":{"type":"Float","value":50}}

📝 Example 4: Function Management

📋 Total functions: 16
🔍 Retrieved function: Get Active Users
✏️  Function updated
🗑️  Function deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

✅ All examples completed!
[32m✓ http_functions.js completed successfully[0m
[34m
=== Running ekoDB/ekodb-client/examples/javascript/batch_operations.js ===[0m
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
[32m✓ batch_operations.js completed successfully[0m
[34m
=== Running ekoDB/ekodb-client/examples/javascript/kv_operations.js ===[0m
=== Key-Value Operations (Direct HTTP) ===

✓ Authentication successful

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: { userId: 123, username: 'john_doe' }

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: { price: 29.99, name: 'Product 1' }
cache:product:2: { price: 39.989999999999995, name: 'Product 2' }
cache:product:3: { price: 49.989999999999995, name: 'Product 3' }

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
[32m✓ kv_operations.js completed successfully[0m
[34m
=== Running ekoDB/ekodb-client/examples/javascript/collection_management.js ===[0m
=== Collection Management (Direct HTTP) ===

✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: MkvWw7FlWxRGMIwkDWRMStUkYoUM-1q_hGVsVsa1vQdETksVGEb1GMNFUGitsf_iXRlvPhGvZKN74N0ceH_n9Q

=== List Collections ===
Total collections: 26
Sample collections: [
  'chat_messages__ek0_testing',
  'chat_configurations__ek0_testing',
  'enriched_users',
  'schema_users_client_go',
  'batch_users'
]

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
[32m✓ collection_management.js completed successfully[0m
[34m
=== Running ekoDB/ekodb-client/examples/javascript/transactions.js ===[0m
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: WuIBLwjz2BTYYmGHsoA-o80C2bZxdSoJ-t-9AUSmO268PCZiz-8YsN6gMmvZj0JMKCJGYUxa843jZh32qhY1qg
Created Bob: $500 - ID: AgDPgOeTgr0zUMvKKpk6No2fuqdaI8EhqNY8NJjCvxvLtOcw8pkD3IrMhkZkmj92eGTla9BLw1TbOC20oNUcZg

=== Example 1: Begin Transaction ===
Transaction ID: b239742c-bfda-4001-99a5-9c0559409f2c

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: {"type":"Integer","value":800}
Bob: {"value":700,"type":"Integer"}

=== Example 5: Rollback ===
New transaction: cd576aa6-8cf4-47c8-b6de-b79265cfc6f8
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: {"type":"Integer","value":700}

=== Cleanup ===
✓ Deleted test accounts

✓ All transaction examples completed
[32m✓ transactions.js completed successfully[0m
[34m
=== Running ekoDB/ekodb-client/examples/javascript/crud_functions.js ===[0m
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
📝 Function 1: Insert + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: TM9vvL3RsHkb9n8A5bUEgdlH0O5EaOmTwE82XYZnTJLpNCF8yfYLchgehSCfEr8PyEUbwSe-_skIM5yvI-839A

2️⃣ Calling Function (Insert + Verify)...
   ✅ Function executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Name: [object Object]
   📋 Email: {"type":"String","value":"alice@example.com"}
   📋 Status: [object Object]
   📋 Credits: [object Object]

============================================================
📝 Function 2: Query + Update + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: 08E3PQWMLHJWbw1yQ3qeKO-Gmds1jLgQ8TyrWjHIx5ARd8MwAc0S7VkzXl72G78vZwCjfgioOMtT9oKLkdx7hw

2️⃣ Calling Function (Query + Update + Verify)...
   ✅ Function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Status updated to: {"type":"String","value":"active"}
   📋 Name: {"value":"Alice Smith","type":"String"}

============================================================
📝 Function 3: Query + Update Credits + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: P8XwClO9ElR5bqplO63-fdNELdJqlUFAQMzQZpZciQ3xIByE1ybWFD5eHQBRBjW4Hkao40v529MtYjT4lwNieQ

2️⃣ Calling Function (Query + Update Credits + Verify)...
   ✅ Function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {"type":"Integer","value":0}
   📋 Status: {"type":"String","value":"active"}
   📋 Name: {"value":"Alice Smith","type":"String"}

============================================================
📝 Function 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Function...
   ✅ Function saved: qgprAtphLucJq85tj_LEKADLM4KyDrDmftmGVAOh4nNYWn4pbspFu2ZIgasj72JAJQ9MIZiaSHaMKglPtHn4IA

2️⃣ Calling Function (Query + Delete + Verify)...
   ✅ Function executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted function: TM9vvL3RsHkb9n8A5bUE...
   ✅ Deleted function: 08E3PQWMLHJWbw1yQ3qe...
   ✅ Deleted function: P8XwClO9ElR5bqplO63-...
   ✅ Deleted function: qgprAtphLucJq85tj_LE...
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
[32m✓ crud_functions.js completed successfully[0m
[34m
=== Running ekoDB/ekodb-client/examples/javascript/document_ttl.js ===[0m
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
  Output: Document ID = U7ok_QMOpeUKc2hhALPObKIq_ieUChJx-bJ0IlfRDHUgnYc17AoSdGO0eYsovtupWmc90cswHkhZ2KcQHgCa3g
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: findById(U7ok_QMOpeUKc2hhALPObKIq_ieUChJx-bJ0IlfRDHUgnYc17AoSdGO0eYsovtupWmc90cswHkhZ2KcQHgCa3g)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: findById(U7ok_QMOpeUKc2hhALPObKIq_ieUChJx-bJ0IlfRDHUgnYc17AoSdGO0eYsovtupWmc90cswHkhZ2KcQHgCa3g)
  Output: Error (expected) - Request failed with status 404: {"error":"Record not found (expired)"}
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
[32m✓ document_ttl.js completed successfully[0m
[34m
=== Running ekoDB/ekodb-client/examples/javascript/websocket_ttl.js ===[0m
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
  Output: Document ID = bASwYiF31C446vVucehItB0dzxFEBI5_ySUF5QQk5P9fOJfj-Pcs4mLvaFeqaIOf_5zotRCfJmyCPho8UpIgPw
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: findById(bASwYiF31C446vVucehItB0dzxFEBI5_ySUF5QQk5P9fOJfj-Pcs4mLvaFeqaIOf_5zotRCfJmyCPho8UpIgPw)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: findById(bASwYiF31C446vVucehItB0dzxFEBI5_ySUF5QQk5P9fOJfj-Pcs4mLvaFeqaIOf_5zotRCfJmyCPho8UpIgPw)
  Output: Error (expected) - Request failed with status 404: {"error":"Record has been deleted"}
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
  • Queries correctly return null for expired documents
[32m✓ websocket_ttl.js completed successfully[0m
[34m
╔════════════════════════════════════════╗[0m
[34m║           Test Summary                 ║[0m
[34m╚════════════════════════════════════════╝[0m
[34mTotal: 10[0m
[32mPassed: 10[0m
[32mFailed: 0[0m
✅ [32mJavaScript direct examples complete![0m
📦 [36mBuilding TypeScript client library...[0m
npm warn EBADENGINE Unsupported engine {
npm warn EBADENGINE   package: 'vitest@4.1.8',
npm warn EBADENGINE   required: { node: '^20.0.0 || ^22.0.0 || >=24.0.0' },
npm warn EBADENGINE   current: { node: 'v23.6.0', npm: '10.9.2' }
npm warn EBADENGINE }

> @ekodb/ekodb-client@0.21.0 prepare
> npm run build


> @ekodb/ekodb-client@0.21.0 build
> tsc


up to date, audited 52 packages in 1s

17 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.21.0 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning JavaScript client library examples...[0m

added 1 package, removed 1 package, and audited 9 packages in 1s

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'gTXEwqD7CQGJa1pvv68HOAG8J-f0wSTcQerh-hAgmqb2c_Z6o_K67CYwpDANeQoT8pFNa9RpbjP89pITe_HITQ'
}

=== Find by ID ===
Found: {
  id: 'gTXEwqD7CQGJa1pvv68HOAG8J-f0wSTcQerh-hAgmqb2c_Z6o_K67CYwpDANeQoT8pFNa9RpbjP89pITe_HITQ',
  name: { value: 'Test Record', type: 'String' },
  active: { type: 'Boolean', value: true },
  value: { type: 'Integer', value: 42 }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  name: { value: 'Updated Record', type: 'String' },
  value: { type: 'Integer', value: 100 },
  id: 'gTXEwqD7CQGJa1pvv68HOAG8J-f0wSTcQerh-hAgmqb2c_Z6o_K67CYwpDANeQoT8pFNa9RpbjP89pITe_HITQ',
  active: { type: 'Boolean', value: true }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: GaPKkDGa3Qan2PSkEc9sp9GY3Laazd8eiq6dB1VMA9aK5wa0SdfL0JcWGuJCNhZxng6o_pRSmYZq7VY66L6pJQ

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
Collection created with first record: Jmfsabqg8p0GJ_r-RfA8CfxXkNDeoZT3g7dtN4a9N-Z9hSNf9InXR3KlW6_LVD6aKj5fRtbC6KqrN7vswxwEIw

=== List Collections ===
Total collections: 25
Sample collections: chat_messages__ek0_testing,chat_configurations__ek0_testing,enriched_users,schema_users_client_go,batch_users

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
Retrieved value: { userId: 123, username: 'john_doe' }

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: { price: 29.99, name: 'Product 1' }
cache:product:2: { price: 39.989999999999995, name: 'Product 2' }
cache:product:3: { price: 49.989999999999995, name: 'Product 3' }

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 0 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 15

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: false

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 6ZJffHe0QvcqR9X5acM3l1RI45nLJKB0gbzKZ9G-WtCA-bbCF2ukdjCH5hEmf6Cfwxt2RyQuyyWwV14yVjZyXQ
Created Bob: $500 - ID: cf8lh4LYs7Nbf1trvyy178RGINKbnSMVA7m3qPAaH338mAOvJfO5TYp-5NpeaJLq1BbsFf3UeBUhN51c_vlN5g

=== Example 1: Begin Transaction ===
Transaction ID: 635c582e-962f-4ead-bec3-5fc36c933a9b

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: f88e140a-8427-45b8-93c0-84203160a58c
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
  1. Score: 25.740, Matched: name, email.value, email, name.value
  2. Score: 12.540, Matched: name.value, name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: bio.value, bio, title.value, title
  2. Score: 26.400, Matched: bio, title.value, bio.value, title
  3. Score: 26.400, Matched: title.value, title, bio.value, bio
  4. Score: 26.400, Matched: bio.value, title, title.value, bio

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio.value, title.value, title, bio
  2. Score: 39.600, Matched: bio.value, title, title.value, bio
  3. Score: 39.600, Matched: title, bio, title.value, bio.value
  4. Score: 39.600, Matched: bio.value, bio, title, title.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio.value, bio

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.743, Matched: 
  2. Score: 0.741, Matched: 
  3. Score: 0.730, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.697, Matched: content, content.value, title, title.value
  2. Score: 1.496, Matched: content, title, content.value, title.value
  3. Score: 0.292, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: skills, skills.value, bio, bio.value

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
Schema fields: [ 'age', 'email', 'name', 'status' ]
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
✓ Inserted document: KrFzMpRaVI1Jpk4jvtWQekqdqMQv3Acr7DCZPKnCVsyHA6zFt4i3LBuIe_ZZx0H2Uv_LLGMpUub5OViQFNfOfQ

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: 8hkKlR2LSpiuJD7SD0unxG9oTF38zulGsbI3UXhbsA1yOAsaiOjttwtyYQUKM7ET3iNoRs4WS8efDg3BWSiPcQ

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
✓ Inserted document with TTL: rP0qQMQRHRkFKYirGZkX3ZYCeM-TjiIz4JaHP4fVUgI35H5-cM0ohdO2I0WRQybTyV806Hn5J6dn0vdIaI1chw

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
=== ekoDB as Edge Cache - Simple Example ===

Creating edge cache function...
✓ Edge cache script created: zyJvNomocbO8r2JkdoRzx6HRTPKrmpjcaBTzX4yvhy2VzSJojG3X0JeZnpzhcUQ7pxxlzMqrPgxs74y2P_ZtLg

Call 1: Cache miss (fetches from API)
Response time: 42ms
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "longitude": -73.99308,
          "generationtime_ms": 0.02396106719970703,
          "latitude": 40.710335,
          "current": {
            "interval": 900,
            "temperature_2m": 21.5,
            "time": "2026-06-10T01:30"
          },
          "timezone_abbreviation": "GMT",
          "utc_offset_seconds": 0,
          "timezone": "GMT",
          "elevation": 32,
          "current_units": {
            "interval": "seconds",
            "temperature_2m": "°C",
            "time": "iso8601"
          }
        }
      }
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

Call 2: Cache hit (served from ekoDB)
Response time: 44ms (1x faster!)
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "longitude": -73.99308,
          "generationtime_ms": 0.02396106719970703,
          "latitude": 40.710335,
          "current": {
            "interval": 900,
            "temperature_2m": 21.5,
            "time": "2026-06-10T01:30"
          },
          "timezone_abbreviation": "GMT",
          "utc_offset_seconds": 0,
          "timezone": "GMT",
          "elevation": 32,
          "current_units": {
            "interval": "seconds",
            "temperature_2m": "°C",
            "time": "iso8601"
          }
        }
      }
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

=== The Magic ===
- Your DATABASE is your EDGE
- No Redis needed
- No CDN needed
- No cache invalidation logic needed (TTL handles it)
- With ripples: All nodes auto-sync cache
- One service: Database + Cache + Edge Functions

✓ Example complete!

🚀 ekoDB Functions Example (JavaScript Client)

✅ Client initialized (token exchange automatic)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

ℹ️  Function 'get_active_users' already existed — updated instead
✅ Function saved: tQLStoxuGRtoJWoaJxzQ6CL3ScNDOHvbSxFaVDfX973hzE0stZXWxf0qXFg-oRDIQ_ahwzJKZ0oAgkO8m97ksQ
📊 Found 10 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Function

ℹ️  Function 'get_users_by_status' already existed — updated instead
✅ Function saved
📊 Found 10 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Function

✅ Function saved
📊 Statistics: 2 groups
   {"count":{"type":"Integer","value":5},"status":{"value":"inactive","type":"String"},"avg_score":{"type":"Float","value":50}}
   {"count":{"value":5,"type":"Integer"},"avg_score":{"type":"Float","value":60},"status":{"value":"active","type":"String"}}
⏱️  Execution time: 0ms

📝 Example 4: Function Management

📋 Total functions: 17
🔍 Retrieved function: Get Active Users
✏️  Function updated
🗑️  Function deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

📝 Example 5: Multi-Stage Pipeline

✅ Multi-stage function saved
📊 Pipeline executed 2 stages
⏱️  Total execution time: 0ms
📈 Stage breakdown:

📝 Example 6: Count Users

✅ Count function saved
📊 Total user count: 10
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Deleted collection
✅ Deleted test functions

✅ All examples completed successfully!

💡 Key Advantages of Using the Client:
   • Automatic token management
   • Type-safe Stage builders
   • ChatMessage helpers
   • Cleaner, more maintainable code
   • Built-in error handling
=== ekoDB Function Composition Examples ===

📋 Setting up test data...

✅ Test data ready

📝 Example 1: Basic Function Composition

Building reusable functions that call each other...

ℹ️  Function 'fetch_user' already existed — updated instead
✅ Saved reusable function: fetch_user
✅ Saved composed function: get_user_wrapper (calls fetch_user + projects fields)

📊 Result from composed function:
   Records: 1
   Name: {"type":"String","value":"User 1"}
   Department: {"value":"engineering","type":"String"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

ℹ️  Function 'fetch_and_store_user' already existed — updated instead
✅ Saved reusable function: fetch_and_store_user (uses KV)
ℹ️  Function 'swr_user' already existed — updated instead
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 163ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "email": "Sincere@april.biz",
      "username": "Bret",
      "address": {
        "suite": "Apt. 556",
        "geo": {
          "lat": "-37.3159",
          "lng...

Second call (cache hit - from cache):
   ⏱️  Duration: 40ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "email": "Sincere@april.biz",
      "username": "Bret",
      "address": {
        "suite": "Apt. 556",
        "geo": {
          "lat": "-37...
   🚀 Cache speedup: 4.1x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

ℹ️  Function 'validate_user' already existed — updated instead
✅ Level 1 function: validate_user
ℹ️  Function 'fetch_slim_user' already existed — updated instead
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
🚀 ekoDB JavaScript Complete Functions Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Function saved: rX72-QH-LSsamYzwxYZNx2lrpLxKct0SnQjPAX2sSxdtRXXsoMVMHCQU7aJ9QAw_MUzzI_PsxQfQsuazWUFQTw
📊 Found 2 product groups
   {"avg_price":{"type":"Float","value":575.6666666666666},"category":{"type":"String","value":"Electronics"},"count":{"type":"Integer","value":3}}
   {"avg_price":{"value":474,"type":"Float"},"count":{"value":2,"type":"Integer"},"category":{"type":"String","value":"Furniture"}}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories
   {"category":{"value":"Furniture","type":"String"},"count":{"type":"Integer","value":2}}
   {"count":{"type":"Integer","value":3},"category":{"value":"Electronics","type":"String"}}
⏱️  Execution time: 0ms

📝 Example 4: High Rating Products

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 5: Function with Parameter Definition

✅ Function saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 6: Multi-Stage Pipeline (FindAll → Group → Count)

✅ Function saved
📊 Pipeline executed 3 stages
⏱️  Total execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All complete function examples finished!

💡 This example demonstrates ekoDB's Function system:
   ✅ FindAll operations
   ✅ Group aggregations (Count, Average)
   ✅ Multi-stage pipelines (FindAll → Group → Count)
   ✅ Parameter definitions
   ✅ Function management (save, call, delete)
🚀 ekoDB JavaScript KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: Z0xdg3ZA_UkWMMdudWIrVSpzBdEXc236r8dkEA_g_Y97uCUxw_4NT-sNLk9XbyK3WrOcZsewS3Wbb3-RSedMsg
✅ Inserted 2 products with wrapped types

📝 Example 2: Function with Wrapped Type Parameters

✅ Function saved: PE1q3WQ_-TxDQmEOGg0_6Qz2qO8P48BAUv79L0Yr9uldT3yUjPCIwtE8rnCS_Uq8T1LkysufuwuP_l9AetnbGQ
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"role":"admin","userId":"user_abc"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: eoiOTAQAFYaonCzSCj5EerlTNsGInETy64CvKp16_yh1AB8R7ygcNUr7ZOLoOjP3sWWpAUY3teKU4aUnI58mXA
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Function

✅ Function saved: XcXfdQ-G0btLe3rUl-0zUOa6veiDSVSPlUFpO2XjBJDFuxE23LByqFU2m_PyDUo4hhQyDy-ikG-C-cWG8ccO4w
📊 Processed order with caching
⏱️  Stages executed: 3
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All KV & Wrapped Types examples completed!

💡 Key takeaways:
   ✅ Use Field.* helpers for type-safe wrapped values
   ✅ Field.decimal() preserves precision (no floating point errors)
   ✅ KV store is great for caching and quick lookups
   ✅ Stage.kv*() functions work within scripts
   ✅ Combine KV caching with collection inserts for real workflows
🚀 ekoDB CRUD Functions Example

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
📊 Average scores by role:
   user: 70.0 (7 users)
   admin: 20.0 (3 users)
⏱️  Execution time: 0ms

📝 Example 4: Top Users by Score

✅ Function saved
📊 Users (showing first 5 of 10):
   1. User 7 - Score: 70
   2. User 3 - Score: 30
   3. User 10 - Score: 100
   4. User 6 - Score: 60
   5. User 2 - Score: 20
⏱️  Execution time: 0ms

📝 Example 5: User Summary Statistics

✅ Function saved
📊 User summary (2 groups):
   active users: 7 users, Total Score: 370
   inactive users: 3 users, Total Score: 180
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD function examples completed!
🚀 ekoDB Search Functions Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Function saved
📊 Found 5 documents
   1. Introduction to Machine Learning (AI)
   2. Getting Started with ekoDB (Database)
   3. Vector Databases Explained (Database)
   4. Natural Language Processing (AI)
   5. Database Design Principles (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   Database: 3 documents
   AI: 2 documents
⏱️  Execution time: 0ms

📝 Example 3: Select Specific Fields

✅ Function saved
📊 Document titles (5 docs):
   1. Introduction to Machine Learning
   2. Getting Started with ekoDB
   3. Vector Databases Explained
   4. Natural Language Processing
   5. Database Design Principles
⏱️  Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Function saved
📊 Projected documents (showing first 3):
   1. Introduction to Machine Learning
   2. Getting Started with ekoDB
   3. Vector Databases Explained
⏱️  Execution time: 0ms

📝 Example 5: All Document Fields

✅ Function saved
📊 All documents (5 total, showing first 2):
   1. Introduction to Machine Learning (AI)
   2. Getting Started with ekoDB (Database)
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search function examples completed!
🚀 ekoDB Advanced Functions Example

📋 Setting up test data...
✅ Created 10 products

📝 Example 1: List All Products

✅ Function saved
📊 Found 10 products
⏱️  Execution time: 0ms

📝 Example 2: Group Products by Category

✅ Function saved
📊 Found 2 categories
   Electronics: 6 items (avg $325.67)
   Furniture: 4 items (avg $294.00)
⏱️  Execution time: 0ms

📝 Example 3: Count All Products

✅ Function saved
📊 Total products: 10
⏱️  Execution time: 0ms

📝 Example 4: Multi-Stage Aggregation

✅ Function saved
📊 Category analysis (2 categories):
   Electronics:
      Products: 6 | Stock: 232 | Avg Rating: ⭐4.52
   Furniture:
      Products: 4 | Stock: 43 | Avg Rating: ⭐4.26

⏱️  Total execution time: 0ms
📈 Pipeline stages:

📝 Example 5: Project Specific Fields

✅ Function saved
📊 Product summaries (10 items, showing first 3):
   1. Office Lamp - $79 (⭐4.05)
   2. Monitor 27" - $399 (⭐4.6)
   3. Wireless Mouse - $29 (⭐4.5)
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced function examples completed!
🚀 ekoDB AI Functions Example

⚠️  Note: These examples require AI API credentials (OpenAI, etc.)

📋 Setting up test data...
✅ Created 3 articles

📝 Example 1: Simple Chat Completion

✅ Chat function saved
🤖 AI Response:
   1. Precision: Vector data models are highly precise as they can represent data at the original resolution and form without generalization.

2. Flexibility: They have the capability of depicting features as points, lines, and polygons, allowing for a spectrum of detail and complexity.

3. Topology: Vector databases preserve the spatial and temporal topological information, enabling the understanding of the interconnectedness and adjacency between various points.

4. Scalability: Vector databases have the ability to easily scale up and down without losing quality.

5. Less Disk Space: They usually require less disk space as compared to raster data.

6. Advanced Analysis: Vector databases are beneficial for performing complex spatial analyses, such as network analysis and proximity analysis.
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embedding function saved
📊 Generated embeddings for 3 articles
   1. "Advanced Query Patterns" - 0D vector
   2. "Draft Article" - 0D vector
   3. "Getting Started with ekoDB" - 0D vector
⏱️  Execution time: 0ms

📝 Example 3: List All Articles

✅ Function saved
📊 Found 3 articles
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All AI examples completed!
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_api_user_js (1oaKKhiV-AZiSDHyuSPiMfYssB6EcYP9eciRyu0soJ41ByfizCLSyYA7t3W7qP4N4UlFxVXbQXljTAEECYSoMQ)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "name": "Leanne Graham",
          "address": {
            "street": "Kulas Light",
            "zipcode": "92998-3874",
            "city": "Gwenborough",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "suite": "Apt. 556"
          },
          "username": "Bret",
          "id": 1,
          "company": {
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net"
          },
          "phone": "1-770-736-8031 x56442",
          "website": "hildegard.org",
          "email": "Sincere@april.biz"
        }
      }
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
Response time: 42ms (served from cache)
✓ Lightning fast cache hit

🧹 Cleaning up...
✓ Cleanup complete

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation
🚀 ekoDB JavaScript Client - Native SWR Function Examples

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
✓ Created native SWR script: github_user_native (66pGMeV6KIBYy1O93nsorPdE55vDcQMIKigExRb6Q2syu-EhoETFYFdDQbsUwSkwNqeGItcNydEe2hcBnZ5EhA)

First call (cache miss - will fetch from GitHub API):
  Response time: 16ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 15ms
  Speedup: 1.1x faster 🚀
  Records returned: 1


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR script with audit trail: product_swr_audit (_QivnKjUxqGN5YLFKI0epFUWFELiKIDH2oNPcAc_NWmBP4bRVIGUnPKamocatxj8XtHQh49dmIIzf3RYPjrkdQ)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (PiLejK_nndmt7mKBFhk58W_JETYoy73DHgOrqM-ffrCmTK8DAHIOUAW3Ns6XhM3jPg3LXXxrDFTnYGY-hJvRew)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL script: flexible_cache (biOE8dl_Sw3lFVB-ELEt3Q8NW9s03nwaPSAyQOjO7F-7FIbD6bnY3O9ycGzUQ477vUBLj1AeNld0HV-fE-_RWw)
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
✓ Deleted 4 test functions

✅ All examples completed!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: uEer3lmjKIocOCg897a__klGrSsX0Qc5Ax6nS4xwNnmqDvqGGF9p6OLaMbBRiLcwQt_7UgkBmAQqOA0mBSLkDQ

=== Sending Chat Message ===
Message ID: cLsRgWeZS0cLTs7qI2gXYgIozsjpcM10jI3tyRVLq1vTltN-xk00y6N5Y39iYwkh8TDpUxkFVYz83gk_BTt55Q

=== AI Response ===
The available products and their prices are as follows:

1. **ekoDB Cloud**
   - Price: $499
   - Description: Fully managed cloud database service product

2. **ekoDB Pro**
   - Price: $299
   - Description: Enterprise edition product with advanced features

3. **ekoDB**
   - Price: $99
   - Description: A high-performance database product with AI capabilities

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_js',
  record: {
    id: 'xYXCHzP6HRgWmrz0B81ejcLv3x6nXvodmJZfIAV1o_6TYiv1eDtJ5WDj8U9TBHQV1sxtQHQddmZ4_SB0ByfyoQ',
    name: 'ekoDB Cloud',
    price: 499,
    description: 'Fully managed cloud database service product'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_js',
  record: {
    id: 'T5T7T0wIulw29538RpmVEcW5utRmicaT08yTFypcYvG8sr7YLCYkDrAiniiOUTT3dsESVuy1nDa1VZKVDPYmzw',
    description: 'Enterprise edition product with advanced features',
    price: 299,
    name: 'ekoDB Pro'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_js',
  record: {
    description: 'A high-performance database product with AI capabilities',
    price: 99,
    id: '9xk43-aUnylrzG9GZ0wvUn1RNR3fNfe0s9UoprLAFKPcqtL9hNFdYJlIrsWOLDgRLEGLF5I6_BjCgX3yvF6COA',
    name: 'ekoDB'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 4372ms

=== Token Usage ===
Prompt tokens: 3468
Completion tokens: 90
Total tokens: 3558

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: -kQQjdb21pxHPT92UmSzQBFOLqRv74QLcY4FHTIsVyeo-wQAoka3eTz0MEMa4Zibfzelb9s2Z7ypdaUR56xJ6g

=== Sending Initial Message ===
✓ Message sent
  Response: The available product is:

- **Name:** ekoDB
- **Description:** High-performance database product
- **Price:** $99

If you need more details or have any other questions, feel free to ask!

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
✓ Created second session: sx2x3_N9ASi37OAsjAVwYl5vDgGog3Q1aedqVUUw9wa6vv2IfxrXDvQfPAFwEGZKcG8ItTWMXpSqbj-WIT-2Uw
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
✓ Created session: KuReQYBNZZ_xug4gxuSdDM5a36uHSd1VuSfE5zq4I9StxW5AvoYajyUbjJDKpSMZTxEr5tvE1qndGq0KXFOV9g

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is:

- **Name**: ekoDB
  - **Description**: A high-performance database product
  - **Price**: $99

If you need more information or details about other products, let me know!

✓ Message 2 sent
  Response: The price of ekoDB is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: TIISDRQrXAAhjlW0mHuXz1CHH5cvvKGBhd_x-OZJ3uBEZB6Mnl-moPYKqrbPLdZEYg87VPS_5c71tz8D9gO0Gw
  Parent: KuReQYBNZZ_xug4gxuSdDM5a36uHSd1VuSfE5zq4I9StxW5AvoYajyUbjJDKpSMZTxEr5tvE1qndGq0KXFOV9g

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: TIISDRQrXAAhjlW0mHuXz1CHH5cvvKGBhd_x-OZJ3uBEZB6Mnl-moPYKqrbPLdZEYg87VPS_5c71tz8D9gO0Gw (Untitled)
  Session 2: KuReQYBNZZ_xug4gxuSdDM5a36uHSd1VuSfE5zq4I9StxW5AvoYajyUbjJDKpSMZTxEr5tvE1qndGq0KXFOV9g (Untitled)
  Session 3: sx2x3_N9ASi37OAsjAVwYl5vDgGog3Q1aedqVUUw9wa6vv2IfxrXDvQfPAFwEGZKcG8ItTWMXpSqbj-WIT-2Uw (Untitled)
  Session 4: uEer3lmjKIocOCg897a__klGrSsX0Qc5Ax6nS4xwNnmqDvqGGF9p6OLaMbBRiLcwQt_7UgkBmAQqOA0mBSLkDQ (Untitled)
  Session 5: kG2_CcjINKxJBETgxlXwzU5mswVn09wcpbRMt40aZYsilk5nYPkcaNZhtTzRBmIFBDdVP-Bv2ezLSAqU5IkPVA (Untitled)
  Session 6: sZ_pFtv-SgjN5qRqTCaVXKCAC8pSGDLdZfvZn0vhQYogZ3WN1oAJaiQ2QDYSwfSHBYwGasbWpOF0TVZpseAlLg (Untitled)
  Session 7: Pn8r_isvnjZVuq5VU-lwW6dtY8yitGZAlms9dpC2ETllmSDz9lhBe9hrs36T9feYNukoliHZb83qTuJNKD1h_w (Untitled)
  Session 8: zQ9sPZrEwJouHWdIPzV9BxVUsjLgEVWPvg3BrCKx8qpphMyfN-7PLO8sytR8_4cyYxwTn6ULgBUTvM1x5lGCPQ (Untitled)
  Session 9: 02WYH5fvFCE8jEaFL6l-ZbPq2sEWjD9Le24i-ON89iHVdcsM7uDa9qzgXiCclIs5t99xOqTR6XzzdlGuMjzC8A (Untitled)
  Session 10: XMcm9EI1r0XL4MrLSKCmrJUS-QsNU0ih0n75U0Gmv1MumkPwb3ie_6eM7UNzTVyEipHHZKNJ0NDjQed-w7ci4Q (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: TIISDRQrXAAhjlW0mHuXz1CHH5cvvKGBhd_x-OZJ3uBEZB6Mnl-moPYKqrbPLdZEYg87VPS_5c71tz8D9gO0Gw

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
✅ [32mJavaScript client examples complete![0m
✅ [32mAll JavaScript integration tests complete![0m
🟣 [36mBuilding Kotlin client library...[0m
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE
> Task :jar UP-TO-DATE
> Task :assemble UP-TO-DATE
> Task :compileTestKotlin UP-TO-DATE
> Task :compileTestJava NO-SOURCE
> Task :processTestResources NO-SOURCE
> Task :testClasses UP-TO-DATE
> Task :test UP-TO-DATE
> Task :check UP-TO-DATE
> Task :build UP-TO-DATE

BUILD SUCCESSFUL in 3s
4 actionable tasks: 4 up-to-date
✅ [32mKotlin client built![0m
🧪 [36mRunning Kotlin client library examples...[0m
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Simple CRUD Example ===

=== Create ===
✓ Inserted user: EkoRecord(fields={id=StringValue(value=Nf9iFgvqD1sLfDc1L34sEq3SO_mCo3_H-3JQJc10NwKQzzmbFKaGyQMqAJXpwX8xGY0H4AZnnLEKMwgOIiqhiA)})
  User ID: Nf9iFgvqD1sLfDc1L34sEq3SO_mCo3_H-3JQJc10NwKQzzmbFKaGyQMqAJXpwX8xGY0H4AZnnLEKMwgOIiqhiA

=== Read ===
✓ Found user by ID: EkoRecord(fields={email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice@example.com)}), id=StringValue(value=Nf9iFgvqD1sLfDc1L34sEq3SO_mCo3_H-3JQJc10NwKQzzmbFKaGyQMqAJXpwX8xGY0H4AZnnLEKMwgOIiqhiA), age=ObjectValue(value={value=IntegerValue(value=28), type=StringValue(value=Integer)}), data=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)])}), embedding=ObjectValue(value={value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)]), type=StringValue(value=Array)}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), price=ObjectValue(value={type=StringValue(value=Float), value=FloatValue(value=99.99)}), created_at=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Tue Jun 09 21:41:19 EDT 2026)}), categories=ObjectValue(value={value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)]), type=StringValue(value=Array)}), tags=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)])}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), metadata=ObjectValue(value={type=StringValue(value=Object), value=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})})}), user_id=ObjectValue(value={value=StringValue(value=550e8400-e29b-41d4-a716-446655440000), type=StringValue(value=String)})})

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Alice Johnson
  email (String): alice@example.com
  age (Integer): 28
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): Tue Jun 09 21:41:19 EDT 2026
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1, tag2, tag3]
  metadata (Object): {key=value, nested={deep=true}}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): [electronics, computers]
  data (Bytes): 11 bytes
Record fields: email, id, age, data, embedding, active, price, created_at, categories, tags, name, metadata, user_id

=== Update ===
✓ Updated user: EkoRecord(fields={categories=ObjectValue(value={value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)]), type=StringValue(value=Array)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), created_at=ObjectValue(value={value=StringValue(value=Tue Jun 09 21:41:19 EDT 2026), type=StringValue(value=String)}), user_id=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=550e8400-e29b-41d4-a716-446655440000)}), city=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=San Francisco)}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), embedding=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)])}), id=StringValue(value=Nf9iFgvqD1sLfDc1L34sEq3SO_mCo3_H-3JQJc10NwKQzzmbFKaGyQMqAJXpwX8xGY0H4AZnnLEKMwgOIiqhiA), data=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)])}), tags=ObjectValue(value={value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)]), type=StringValue(value=Array)}), metadata=ObjectValue(value={type=StringValue(value=Object), value=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})})}), price=ObjectValue(value={type=StringValue(value=Float), value=FloatValue(value=99.99)}), email=ObjectValue(value={value=StringValue(value=alice@example.com), type=StringValue(value=String)}), age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=29)})})

=== Query ===
✓ Found 1 users matching query
  - EkoRecord(fields={tags=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)])}), metadata=ObjectValue(value={type=StringValue(value=Object), value=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})})}), id=StringValue(value=Nf9iFgvqD1sLfDc1L34sEq3SO_mCo3_H-3JQJc10NwKQzzmbFKaGyQMqAJXpwX8xGY0H4AZnnLEKMwgOIiqhiA), age=ObjectValue(value={value=IntegerValue(value=29), type=StringValue(value=Integer)}), categories=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)])}), embedding=ObjectValue(value={value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)]), type=StringValue(value=Array)}), created_at=ObjectValue(value={value=StringValue(value=Tue Jun 09 21:41:19 EDT 2026), type=StringValue(value=String)}), data=ObjectValue(value={value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)]), type=StringValue(value=Array)}), email=ObjectValue(value={value=StringValue(value=alice@example.com), type=StringValue(value=String)}), price=ObjectValue(value={type=StringValue(value=Float), value=FloatValue(value=99.99)}), city=ObjectValue(value={value=StringValue(value=San Francisco), type=StringValue(value=String)}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), user_id=ObjectValue(value={value=StringValue(value=550e8400-e29b-41d4-a716-446655440000), type=StringValue(value=String)}), name=ObjectValue(value={value=StringValue(value=Alice Johnson), type=StringValue(value=String)})})

=== Delete ===
✓ Deleted user with ID: Nf9iFgvqD1sLfDc1L34sEq3SO_mCo3_H-3JQJc10NwKQzzmbFKaGyQMqAJXpwX8xGY0H4AZnnLEKMwgOIiqhiA

✓ Confirmed user was deleted

=== Cleanup ===
✓ Deleted collection: kotlin_users_example

=== Example Complete ===

BUILD SUCCESSFUL in 6s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Simple WebSocket Example ===

=== Create WebSocket Client ===
✓ WebSocket client created

=== Connect to WebSocket ===
✓ Connected to WebSocket

=== Insert Test Record ===
✓ Inserted test record

=== Find All via WebSocket ===
✓ WebSocket findAll result:
  {"data":[{"id":"tkNFGbeseBH3LcbXrkCLrgH4JMD3tHdxi7Aty_gLXo6W18kOKiyZyVTfuZmxlA_8oOffaWHvatHOS0v-OV-DWw","name":{"type":"String","value":"Test User"},"status":{"value":"active","type":"String"}}]}

=== Close WebSocket ===
✓ WebSocket closed

=== Cleanup ===
✓ Deleted collection: kotlin_websocket_example

=== Example Complete ===

BUILD SUCCESSFUL in 3s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Batch Operations Example ===

=== Batch Insert ===
✓ Inserted 5 records
  IDs: tfatlOFJNpW6m1m48F0JQXxEwHCRZZFrzBBxc8Cc9hY_gogjgV-jYjHc71z5H2racjz3GbyGJxAM61f-YEpDEQ, vGaXl3XTZ4_XijXDIyCID5cYSZXHvaK4OpzyYmatlZaD5Lc52LZOidVvdwAwoj2pV0YeBHNBZgBGYLQquzUXyw, _RB4FReLHo67lxKPMinm-WjrpMtNMrhRGyD_yLUMrTBdPicKwNMs69E91NiFp76KYHdIsBESsOLz8WVpVkwlcQ...

=== Batch Update ===
✓ Updated 3 records

=== Batch Delete ===
✓ Deleted 2 records

=== Cleanup ===
✓ Deleted collection: kotlin_batch_example

=== Example Complete ===

BUILD SUCCESSFUL in 3s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Collection Management Example ===

=== List Collections ===
✓ Found 29 collections
  - schema_products_client_js
  - chat_messages__ek0_testing
  - schema_employees_client_js
  - chat_configurations__ek0_testing
  - enriched_users

=== Check Collection Existence ===
Collection 'kotlin_collection_example' exists: false

=== Create Collection with Schema ===
✓ Created collection with schema: kotlin_collection_example

=== Get Collection Schema ===
✓ Schema: {"fields":{"name":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null}},"version":1,"created_at":"2026-06-10T01:41:35.543704Z","last_modified":"2026-06-10T01:41:35.543705Z","bypass_ripple":false,"primary_key_alias":"id"}

=== Cleanup ===
✓ Deleted collection: kotlin_collection_example

=== Example Complete ===

BUILD SUCCESSFUL in 3s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - KV Operations Example ===

=== KV Set ===
✓ Set key: user:123

=== KV Get ===
✓ Retrieved value: {"email":"alice@example.com","role":"admin","name":"Alice"}

=== KV Set with TTL ===
✓ Set key with 10s TTL: session:abc123

=== Verify TTL Key ===
✓ Session value: {"user_id":"123","created_at":1781055699889}
  (Will expire in 10 seconds)

=== KV Batch Set ===
✓ Batch set 3 keys
  config:db: success
  config:cache: success
  config:api: success

=== KV Batch Get ===
✓ Batch retrieved 3 values
  config:db: {"value":{"host":"localhost","port":5432}}
  config:cache: {"value":{"ttl":3600,"enabled":true}}
  config:api: {"value":{"timeout":30,"retries":3}}

=== KV Exists ===
✓ Key exists: true

=== KV Find (Pattern Query) ===
✓ Found 3 keys matching 'config:.*'

=== KV Query (Alias for Find) ===
✓ Total keys in store: 16

=== KV Delete ===
✓ Deleted key: user:123

=== Verify Deletion ===
✓ Key exists after delete: false

=== KV Batch Delete ===
✓ Batch deleted 3 keys
  config:db: deleted
  config:cache: deleted
  config:api: deleted

=== Example Complete ===

BUILD SUCCESSFUL in 6s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 2k6Iq2GGt-ylPejMk7FLmTsIR42nGlljDixRt8aXM3if4aKi2l3ql5PnrEKI4ZjG4m1Vf257PjGQlzOlhHGCeQ
Created Bob: $500 - ID: 85n36mbWE2mFQo_jGgWaYYSTbeiwRe_weOt24h0VSbJYvDgLOODABBeOkjAEfqZUxRe_usBwCji_UF_CqVvvpg

=== Example 1: Begin Transaction ===
Transaction ID: 5b42a345-241b-4789-a0f9-152b1323e65e

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: a9cc5078-f7c6-43f1-9390-8534485dfc67
Status before rollback: Active
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed

BUILD SUCCESSFUL in 3s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Query Builder Example ===

=== Inserting Sample Data ===
✓ Inserted 5 records

=== Query 1: Equality (city = 'NYC') ===
✓ Found 2 records in NYC

=== Query 2: Range (age >= 25 AND age < 32) ===
✓ Found 3 records with age 25-31

=== Query 3: Sort by score (descending) ===
✓ Top 3 scores:
  - Score: ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=95)})
  - Score: ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=92)})
  - Score: ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=88)})

=== Query 4: Complex (score > 80 AND age >= 25) ===
✓ Found 4 high-scoring adults

=== Query 5: IN (city IN ['NYC', 'LA']) ===
✓ Found 4 records in NYC or LA

=== Query 6: Pagination (skip 2, limit 2) ===
✓ Page 2 (2 records):
  - ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Charlie)})
  - ObjectValue(value={value=StringValue(value=Diana), type=StringValue(value=String)})

=== Query 7: Contains (name contains 'a') ===
✓ Found 2 names containing 'a'

=== Cleanup ===
✓ Deleted collection: kotlin_query_example

=== Example Complete ===

BUILD SUCCESSFUL in 4s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Search Example ===

=== Inserting Sample Documents ===
✓ Inserted 5 sample documents

=== Basic Text Search ===
✓ Search results for 'programming':
  {"results":[{"record":{"description":{"value":"Learn Rust programming language with hands-on examples and best practices.","type":"String"},"title":{"type":"String","value":"Rust Programming"},"views":{"type":"Integer","value":765},"tags":{"value":"programming,rust,tutorial","type":"String"},"id":"ouYFw6HPm5X5cEpzfFZH9ECcldVpowsODPOxv3qRncLwP9xtG8M4A3r5NxhuM5gMotTM1uXHxCSQDQWczfZkfg"},"score":19.8,"matched_fields":["tags","title","description"]},{"record":{"views":{"value":486,"type":"Integer"},"id":"_JE54DNZ3WutbtHZbDO_JKLGfMrhrfMEH3zt66ZWaoXU3T9MB1x7bpGyHKRlMlqwCAwKjDh0VDssLn0XxOE3_Q","description":{"value":"Build modern web applications using JavaScript, React, and Node.js.","type":"String"},"tags":{"value":"programming,javascript,web","type":"String"},"title":{"value":"JavaScript Web Development","type":"String"}},"score":6.6000000000000005,"matched_fields":["tags"]},{"record":{"title":{"type":"String","value":"Python for Data Science"},"id":"zWewyXeheh7MxM7jimJDNaBm6RxRzJ9hj59a37DiUNVaoLDyJe_68m7e4MQcN_9hnAyroIHO6zTAAalxYe7BYg","description":{"value":"Master Python for data analysis, machine learning, and visualization.","type":"String"},"views":{"value":412,"type":"Integer"},"tags":{"value":"programming,python,data-science","type":"String"}},"score":6.6000000000000005,"matched_fields":["tags"]}],"total":3,"execution_time_ms":1}

=== Search for 'machine learning' ===
✓ Found results for 'machine learning'
  {"results":[{"record":{"tags":{"value":"ai,machine-learning,python","type":"String"},"views":{"value":770,"type":"Integer"},"id":"zWVXAqZw29HH1Jurii9h4ChsAwAVshI5Q7U4pCjcTF_X4fx0dzc1D55JpO9zZI8e1e-kyVKtpOVyvvEhVj8a6w","description":{"type":"String","value":"Introduction to machine learning algorithms and neural networks."},"title":{"type":"String","value":"Machine Learning Basics"}},"score":2.7,"matched_fields":["title","description","tags"]},{"record":{"id":"zWewyXeheh7MxM7jimJDNaBm6RxRzJ9hj59a37DiUNVaoLDyJe_68m7e4MQcN_9hnAyroIHO6zTAAalxYe7BYg","description":{"value":"Master Python for data analysis, machine learning, and visualization.","type":"String"},"tags":{"type":"String","value":"programming,python,data-science"},"views":{"type":"Integer","value":412},"title":{"value":"Python for Data Science","type":"String"}},"score":1.0,"matched_fields":["description"]},{"record":{"tags":{"type":"String","value":"database,design,sql"},"views":{"value":997,"type":"Integer"},"description":{"value":"Learn database design principles, normalization, and query optimization.","type":"String"},"id":"YUH4lYCca_fa2ISqPx60uJjL53Y6i5hoD8L8sRYLHWP0_SpjoaEffWsCVut-U4cITSVqfjRHFvsN1qODybhzjA","title":{"value":"Database Design","type":"String"}},"score":0.5,"matched_fields":["description"]},{"record":{"description":{"type":"String","value":"Learn Rust programming language with hands-on examples and best practices."},"id":"ouYFw6HPm5X5cEpzfFZH9ECcldVpowsODPOxv3qRncLwP9xtG8M4A3r5NxhuM5gMotTM1uXHxCSQDQWczfZkfg","views":{"value":765,"type":"Integer"},"title":{"value":"Rust Programming","type":"String"},"tags":{"value":"programming,rust,tutorial","type":"String"}},"score":0.5,"matched_fields":["description"]}],"total":4,"execution_time_ms":1}

=== Cleanup ===
✓ Deleted collection: kotlin_search_example

✓ All search operations completed successfully

BUILD SUCCESSFUL in 4s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Schema Management Example ===

=== Creating Collection with Schema ===
✓ Created collection 'kotlin_schema_example' with schema

=== Inserting Valid Documents ===
✓ Inserted user 1: StringValue(value=TtRjJAOlKzwL8V4PFLsuM-YNx1pY1YUwe4Krwjexikh49pr3Sx4LczyTK0gy-hdO8wd7crleFvtjc_3_6cqR0A)
✓ Inserted user 2: StringValue(value=v0EmRQA4_bnFJKoHwA3WzZjaCfRCRPy6uNlZ2xi9jxpJtaCn0RZdg3D0ReYvkhdJdP4165aGVpdMEig23PEXHQ)

=== Getting Schema ===
✓ Schema for kotlin_schema_example:
  Fields: {"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"title":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"email":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"status":{"field_type":"String","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null}}

=== Listing Collections ===
✓ Total collections: 30
  Sample: [schema_products_client_js, chat_messages__ek0_testing, schema_employees_client_js, chat_configurations__ek0_testing, enriched_users]

=== Cleanup ===
✓ Deleted collection: kotlin_schema_example

✓ All schema management operations completed successfully

BUILD SUCCESSFUL in 3s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Join Operations Example ===

=== Setting up sample data ===
✓ Sample data created

=== Example 1: Single collection join (users with departments) ===
✓ Found 2 users with department data
  - Bob Smith: Sales
  - Alice Johnson: Engineering

=== Example 2: Join with filtering ===
✓ Found 1 users in Engineering
  - Alice Johnson: Building A

=== Example 3: Join with user profiles ===
✓ Found 2 users with profile data
  - Bob Smith: Sales Manager
  - Alice Johnson: Senior Software Engineer

=== Example 4: Join orders with user data ===
✓ Found 2 completed orders
  - Laptop ($1200) by Alice Johnson
  - Mouse ($25) by Alice Johnson

=== Example 5: Complex join with multiple conditions ===
✓ Found 2 users with example.com emails
  - Alice Johnson (alice@example.com): Building A
  - Bob Smith (bob@example.com): Building B

=== Cleanup ===
✓ Deleted test collections

✓ Join operations example completed successfully

BUILD SUCCESSFUL in 4s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Document TTL Example ===

=== Insert with TTL ===
✓ Inserted document with 10s TTL
  Document ID: e2ZGX8iIsgoX7L8TqMPfoUDZRmqwK5y5e2URb05cffXP6m_a90NfDttOFvc1Ki7W4U5QzEFxdBWeTMdJVVOXtQ

=== Verify Document Exists ===
✓ Document found: created_at, user_id, id, session_id, ttl

=== Insert with Longer TTL ===
✓ Inserted document with 1h TTL
  Document ID: f1Ov85UBJEut3E34IFKAhTnkArmY4AyekRqCbVt2IVrz7eJxlMN3SYEjS-TmaC8clc1Z8DoBvDQcj9D7uINhIw

=== TTL Expiration ===
✓ Document will automatically expire after 10 seconds

=== Verify Long TTL Document ===
✓ Long TTL document still exists: id, cache_key, value, ttl

=== Delete Document ===
✓ Deleted document

=== Cleanup ===
✓ Deleted collection: kotlin_ttl_example

=== Example Complete ===

BUILD SUCCESSFUL in 3s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - WebSocket TTL Example ===

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: StringValue(value=xVUyjsg5oT19a30G8HVpDkItBdki1m-vpizSOWvto4_rQGnhJTB1HokqR20-V8BYfvt-eQSI5KsLHJDHCg9xog)

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved data via WebSocket:
  {"data":[{"id":"xVUyjsg5oT19a30G8HVpDkItBdki1m-vpizSOWvto4_rQGnhJTB1HokqR20-V8BYfvt-eQSI5KsLHJDHCg9xog","name":{"type":"String","value":"WebSocket TTL Test"},"created_at":{"type":"Integer","value":1781055734321},"value":{"type":"Integer","value":42},"ttl":"2026-06-10T02:42:14.427147Z"}]}

✓ WebSocket closed

=== Cleanup ===
✓ Deleted collection: kotlin_websocket_ttl_example

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration

BUILD SUCCESSFUL in 3s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB as Edge Cache - Simple Example ===

Setting up edge cache collection...
✓ Cache entry created

Creating edge cache lookup function...
✓ Edge cache function created: SxdbGP4JuaPWsZmLAWV1WjFmeJdlN3u4XhIDxi86kWvm41rANBU488rjyGKQs0IGdGRmcBNqY5MrOIwU5SKQcA

Call 1: Cache lookup
Found 1 cached entries
Response time: 28ms

Call 2: Cache lookup (connection warm)
Found 1 cached entries
Response time: 20ms

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

BUILD SUCCESSFUL in 3s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
🚀 ekoDB Functions Example (Kotlin Client)

SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
✅ Client initialized

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function

✅ Function saved: HHGguZrDtUv1v8bh1pSsZBHaFj1S31Il0plXE-i7nRUUTChUkSqG8tz-8hFNkSIkGyiw7ECWPuF3iqWFe0DLCA
📊 Found 15 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Function

✅ Function saved
📊 Found 15 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Function

✅ Function saved
📊 Statistics: 3 groups
   {"status":{"value":"active","type":"String"},"count":{"value":5,"type":"Integer"},"avg_score":{"type":"Float","value":60.0}}
   {"count":{"type":"Integer","value":5},"avg_score":{"type":"Float","value":50.0},"status":{"value":"inactive","type":"String"}}
   {"count":{"value":5,"type":"Integer"},"status":{"value":"null","type":"String"},"avg_score":{"value":60.0,"type":"Float"}}
⏱️  Execution time: 0ms

📝 Example 4: function Management

📋 Total functions: 16
🔍 Retrieved function: Get Active Users
✏️  function updated
🗑️  function deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

📝 Example 5: Multi-Stage Pipeline

✅ Multi-stage function saved
📊 Pipeline executed 2 stages
⏱️  Total execution time: 0ms
📈 Stage breakdown:

📝 Example 6: Count Users

ℹ️  Function 'count_users' already existed — updated instead
✅ Count function saved
📊 Total user count: {"value":15,"type":"Integer"}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Deleted collection
✅ Deleted test functions

✅ All examples completed successfully!

💡 Key Advantages of Using the Client:
   • Automatic token management
   • Type-safe Stage builders
   • Built-in error handling

BUILD SUCCESSFUL in 7s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
🚀 ekoDB Kotlin Complete Functions Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Function saved: qkiPmk75seB613rWW1HIxXIocsCBrcF65IUPPeN9-i_-zXrabq_A51LG-oI2ycbMJqv3yDMqO9W6Evz1UbriuA
📊 Found 2 category groups

📝 Example 2: Simple Product Listing

✅ Function saved
📊 Found 5 products

📝 Example 3: Count by Category

✅ Function saved
📊 Found 2 categories

📝 Example 4: Multi-Stage Pipeline (FindAll → Group → Count)

✅ Function saved
📊 Pipeline executed with 1 results

🧹 Cleaning up...
✅ Cleanup complete

✅ All complete function examples finished!

💡 This example demonstrates ekoDB's function system:
   ✅ FindAll operations
   ✅ Group aggregations (Count, Average)
   ✅ Multi-stage pipelines (FindAll → Group → Count)
   ✅ Function management (save, call, delete)

BUILD SUCCESSFUL in 4s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
🚀 ekoDB Kotlin KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within functions
   • Combined wrapped types + KV workflows

SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
✅ Client initialized

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: StringValue(value=-a31lTSL-xL2htPgYGUK_fRBs2652GNTW82-I_sRGUPeqzbNYfabe3ZsdHwwDcDC-8XB9rvwBRJIe_P4GQjWQQ)
✅ Inserted 2 products

📝 Example 2: function with Wrapped Type Parameters

✅ Function saved: i1V-oSzfyeIr-oFHVuVpRycmGBiQkM5gqE8GVsRaBea4_Y2bZOLYPpxDgsZ-rM3pueyRWLuhMOTTeNmoLY0XmQ
📊 Created order via function
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"role":"admin","userId":"user_abc"}
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Functions

✅ Function saved: Zs3LwgQdWPIahF1jboeqirfYNmGPfwRJzYKTh9Vo61oO-ADfZ3pCpCLo8eJZHUkR5anE_guzBiCgXzokkRWv6g
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Function

✅ Function saved: 4tdpKDpGH_F9shTv32lrjhTxVvA-8aqzjmXwS3UPk99IsC13x81dfWozswpGi-E-skrdggAR6Z5JV1ayVlHHhQ
📊 Processed order with caching
⏱️  Stages executed: 3
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All KV & Wrapped Types examples completed!

💡 Key takeaways:
   ✅ Use field* helpers for type-safe wrapped values
   ✅ fieldDecimal() preserves precision (no floating point errors)
   ✅ KV store is great for caching and quick lookups
   ✅ FunctionStageConfig.Kv* classes work within functions

BUILD SUCCESSFUL in 4s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Setting up cache collection...
✓ Cache entry created

Step 2: Create SWR cache lookup function
✓ Created SWR function: swr_cache_lookup_kt (Vxcf3j9YYEeG5xiQw-NTHT4djc1pSbjVRqFdNeGb1ge4ajyvJiDj83TQ4UypdXuOYQLV5mxBRipECcSxA-1e7g)

Step 3: First call - Cache lookup
Found 1 cached entries
✓ Cache lookup complete

Step 4: Second call - Fast cache hit
Response time: 20ms (served from cache)
✓ Lightning fast cache hit

🧹 Cleaning up...
✓ Cleanup complete

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation

BUILD SUCCESSFUL in 4s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
🚀 ekoDB Kotlin Client - Native SWR Function Examples

📋 Demonstrates:
   • Single-function SWR pattern (replaces 4-step pipeline)
   • Automatic cache checking, HTTP fetching, and cache setting
   • Built-in audit trail support
   • Duration string TTLs ('15m', '1h', '30s')
   • Multi-function pipeline integration
   • Dynamic TTL configuration

SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.

Example 1: Basic Native SWR
────────────────────────────────────────────────────────────────────────────────
Single function replaces KvGet → If → HttpRequest → KvSet pipeline
✓ Created native SWR function: github_user_native (KprFtBE4LHBJj_naMWW-SBTX-xNCgW3eg-Zmz5kbPeY2ZfNFPEFiWtoyUiujIBA5NIoAiVgRjqOwX8FmsM3lQw)

First call (cache miss - will fetch from GitHub API):
  Response time: 34ms
  Records returned: 1

Second call (cache hit - instant from KV store):
  Response time: 19ms
  Speedup: 1.8x faster 🚀


Example 2: SWR with Built-in Audit Trail
────────────────────────────────────────────────────────────────────────────────
Optional collection parameter for automatic request logging
✓ Created SWR function with audit trail: product_swr_audit (wuImI6lp37Wx9BNjXqFHfk7MaBp22gzim3kcuLazyiliWtJyDR1T56gBIQ4viU2-kGB76Xqg40bq9-Gp6mE0Ng)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
────────────────────────────────────────────────────────────────────────────────
Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (iJN7IFjk-R60Rpo9pvey-QG09WanXSD07ud7EkcHwjBAvggR15XT7uPYw7qsuMZLvKCL9bpM1iN-juv07QE1JQ)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
────────────────────────────────────────────────────────────────────────────────
TTL as parameter - supports duration strings, integers, ISO timestamps
✓ Created dynamic TTL function: flexible_cache (CiRXK5rNIKkzyzj8mKDFzVcODa8Q56xlvFerLq-NlupdZQXIb29zy6uIjb5Ba_udIiZGo05xBlUE5Xf0kfaXpA)
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
✓ Deleted 4 test functions

✅ All examples completed!

BUILD SUCCESSFUL in 4s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
🚀 ekoDB Kotlin Advanced Functions Example

📋 Setting up test data...
✅ Created 8 products

📝 Example 1: List All Products

✅ Function saved
📊 Found 8 products
⏱️  Execution time: 0ms

📝 Example 2: Group Products by Category

✅ Function saved
📊 Category breakdown:
   {"category":{"value":"Furniture","type":"String"},"avg_price":{"type":"Float","value":365.6666666666667},"count":{"type":"Integer","value":3}}
   {"count":{"value":5,"type":"Integer"},"avg_price":{"type":"Float","value":367.0},"category":{"value":"Electronics","type":"String"}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced function examples finished!

BUILD SUCCESSFUL in 4s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
🚀 ekoDB Kotlin AI Functions Example

📋 Setting up test data...
✅ Created 2 articles

📝 Example 1: Simple Chat Completion

✅ Chat function saved
🤖 AI Response:
   {"response":{"type":"String","value":"Vector databases offer several benefits:\n\n1. **Efficient Similarity Search**: They excel in querying high-dimensional data for similarity, such as finding nearest neighbors quickly.\n\n2. **Support for Unstructured Data**: Ideal for handling unstructured data like images, audio, and text, which can be represented as vectors.\n\n3. **Scalability**: Designed to scale with large datasets, maintaining performance as data volume increases.\n\n4. **Real-time Search**: Many vector databases enable real-time processing, making them suitable for applications like recommendation systems and natural language processing.\n\n5. **Integration with Machine Learning**: They often support direct integration with ML models, allowing for efficient storage and retrieval of embeddings.\n\n6. **Flexibility**: Can handle various data types and enable complex queries beyond simple key-value lookups.\n\n7. **Enhanced Performance**: Optimized for vector operations, improving performance in scenarios like clustering and classification tasks.\n\nOverall, vector databases are particularly beneficial in AI and machine learning applications where similarity and high dimension data handling are critical."},"tokens_used":{"value":241,"type":"Integer"}}
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embed function saved
📊 Embedding generated
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All AI function examples finished!

💡 This example demonstrates:
   ✅ Chat completions with system/user messages
   ✅ Embedding generation for text

BUILD SUCCESSFUL in 11s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
🚀 ekoDB Kotlin CRUD Functions Example

📋 Setting up test data...
✅ Created 10 test users

📝 Example 1: List All Users

✅ Function saved
📊 Found 10 users
⏱️  Execution time: 0ms

📝 Example 2: Count Users by Status

✅ Function saved
📊 User counts by status:
   {"count":{"value":7,"type":"Integer"},"status":{"value":"active","type":"String"}}
   {"status":{"type":"String","value":"inactive"},"count":{"value":3,"type":"Integer"}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD function examples finished!

BUILD SUCCESSFUL in 3s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
🚀 ekoDB Kotlin Search Functions Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Function saved
📊 Found 5 documents
   1. {"type":"String","value":"Natural Language Processing"} ({"type":"String","value":"AI"})
   2. {"type":"String","value":"Vector Databases Explained"} ({"type":"String","value":"Database"})
   3. {"value":"Getting Started with ekoDB","type":"String"} ({"type":"String","value":"Database"})
   4. {"type":"String","value":"Introduction to Machine Learning"} ({"value":"AI","type":"String"})
   5. {"type":"String","value":"Database Design Principles"} ({"type":"String","value":"Database"})
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Function saved
📊 Documents by category:
   {"category":{"type":"String","value":"Database"},"count":{"type":"Integer","value":3}}
   {"count":{"type":"Integer","value":2},"category":{"type":"String","value":"AI"}}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search function examples finished!

BUILD SUCCESSFUL in 3s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Basic Chat Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: WLX5wiOBU4I0YfftsghRbt_hJO2iNjsaoM9kzUl_MWk0VdYi_AlQ0T8evqkcYd1_uYOHJh_33oiIy51yWs2sVw

=== Sending Chat Message ===
✓ Chat response:
  Message ID: "fFEFt92Jw9qqVlKtD8ma02vQ8Zqpf1NlMvCnyegmFXPvErEco-ujmHBrMBb-juPnzesVu4dwLWAvzF748pyAkQ"
  Responses: ["ekoDB is a high-performance database that integrates AI capabilities and offers intelligent caching and real-time features. Its advanced functionalities include:\n\n1. **AI Chat Integration**: ekoDB allows users to query the database using natural language, enabling AI-powered responses that provide relevant context.\n\n2. **Search Capabilities**: It supports various search methods, including:\n   - **Full-text Search**: For finding relevant content based on keyword searches.\n   - **Vector Search**: Enabling semantic search capabilities.\n   - **Hybrid Search**: Combining full-text and vector search with automatic context retrieval.\n\nOverall, ekoDB aims to enhance database interactions by leveraging modern AI and search technologies."]

=== Cleanup ===
✓ Deleted chat session
✓ Deleted collection: kotlin_chat_basic_example

✓ Basic chat example completed successfully

BUILD SUCCESSFUL in 18s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: QH2F2U-_v_k8WaN8wJA9b0y7C8ou2sJBZN6LekzwFdhzVAiGzt5vSApZP_dT_wJ9etPVou4eh8VLzYOV2rfcLw

=== Sending Initial Message ===
✓ Message sent
  Responses: ["The available product is:\n\n- **Name**: ekoDB\n- **Description**: High-performance database product\n- **Price**: $99 \n\nIf you have any more questions or need further assistance, feel free to ask!"]

✓ Second message sent

=== Regenerating AI Response ===
✓ AI response regenerated
  New responses: ["Currently, the available product is **ekoDB**, which is a high-performance database product. It is priced at **$99**. If you need more information about this product or if you're interested in exploring additional options, let me know!"]

=== Updating Message ===
✓ Updated message content

=== Toggling Forgotten Status ===
✓ Marked message as forgotten (excluded from context)

=== Creating Second Session for Merge ===
✓ Created second session: MHdmHSkIDGVZ5QgtNoXBbcd5Oxz12Bl9xzri46E0WB8kCSlkr376rPUNxRKwl79W5PugegSYrlP_U9LVPYfRLg

=== Merging Sessions ===
✓ Merged sessions
  Total messages in merged session: 2

=== Deleting Message ===
✓ Deleted message

=== Cleanup ===
✓ Deleted chat sessions
✓ Deleted collection: kotlin_chat_advanced_example

✓ Advanced chat features example completed successfully

BUILD SUCCESSFUL in 27s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: RizGROtDy5KOUCJ8iJ0OovnCIlqVH9KN9toDBtoCtf6xCfsUwyRRM-RIJdvBFfNHtvq6cEknANwPfCbD-stHHA

=== Sending Messages ===
✓ Message 1 sent
  Responses: ["The available product is:\n\n- **Product Name:** ekoDB\n- **Description:** A high-performance database product with AI capabilities\n- **Price:** $99\n\nIf you have any further questions or need more information, feel free to ask!"]

✓ Message 2 sent
  Responses: ["The price of the product **ekoDB** is **$99**. If you have any more questions or need assistance with anything else, feel free to ask!"]

=== Getting Message History ===
✓ Retrieved message history
  Total messages: [{"chat_id":{"type":"String","value":"RizGROtDy5KOUCJ8iJ0OovnCIlqVH9KN9toDBtoCtf6xCfsUwyRRM-RIJdvBFfNHtvq6cEknANwPfCbD-stHHA"},"content":{"type":"String","value":"What products are available?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"yw1gzjFUyGHJBkFRHfEAxdPuY3g_CbI3L_a4-ZIErtb37j0xAvE1VBZXmdw8Fm3Pnrv_KJ1JXMasBktAYNeikA","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2026-06-10T01:44:26.397400Z"},"id":"6s373tvnA-yO47z38COvtetpeah_vGw_Xmc6kuyGECc60K2CUFhV3r2nsjfRZL3mrXmPImEirAEE7eHxWqdQtA","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":87,"prompt_tokens":6850,"total_tokens":6937}},"updated_at":{"type":"DateTime","value":"2026-06-10T01:44:26.397400Z"}},{"chat_id":{"type":"String","value":"RizGROtDy5KOUCJ8iJ0OovnCIlqVH9KN9toDBtoCtf6xCfsUwyRRM-RIJdvBFfNHtvq6cEknANwPfCbD-stHHA"},"content":{"type":"String","value":"The available product is:\n\n- **Product Name:** ekoDB\n- **Description:** A high-performance database product with AI capabilities\n- **Price:** $99\n\nIf you have any further questions or need more information, feel free to ask!"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"yw1gzjFUyGHJBkFRHfEAxdPuY3g_CbI3L_a4-ZIErtb37j0xAvE1VBZXmdw8Fm3Pnrv_KJ1JXMasBktAYNeikA","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2026-06-10T01:44:26.400160Z"},"id":"xaHQehxYheEhdZkOE-fjIKu8LksScQBbPsvsh91-Q7-hxCkOt30pPcjo8GmLMd_rMtrzME61XX_X4vtQqGj8Ew","llm_model":{"type":"String","value":"gpt-4o-mini"},"llm_provider":{"type":"String","value":"openai"},"role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":87,"prompt_tokens":6850,"total_tokens":6937}},"tool_call_count":{"type":"Integer","value":2},"tool_call_history":{"type":"Object","value":{"iterations":2,"tool_calls":[{"arguments":{"collection":"kotlin_chat_sessions_example","filter":{"content":{"field":"id","operator":"Ne","value":""},"type":"Condition"}},"id":"call_mH4RAPXlFvbPkPq77XHQHSHx","name":"query_collection"}],"tool_results":[{"error":null,"result":{"count":1,"records":[{"description":"A high-performance database product with AI capabilities","id":"yw1gzjFUyGHJBkFRHfEAxdPuY3g_CbI3L_a4-ZIErtb37j0xAvE1VBZXmdw8Fm3Pnrv_KJ1JXMasBktAYNeikA","price":99,"product":"ekoDB"}]},"success":true,"tool_call_id":"call_mH4RAPXlFvbPkPq77XHQHSHx","tool_name":"query_collection"}]}},"updated_at":{"type":"DateTime","value":"2026-06-10T01:44:26.400160Z"}},{"chat_id":{"type":"String","value":"RizGROtDy5KOUCJ8iJ0OovnCIlqVH9KN9toDBtoCtf6xCfsUwyRRM-RIJdvBFfNHtvq6cEknANwPfCbD-stHHA"},"content":{"type":"String","value":"What is the price?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description","product","price"],"record":{"description":"A high-performance database product with AI capabilities","id":"yw1gzjFUyGHJBkFRHfEAxdPuY3g_CbI3L_a4-ZIErtb37j0xAvE1VBZXmdw8Fm3Pnrv_KJ1JXMasBktAYNeikA","price":99,"product":"ekoDB"},"score":2.0928571428571425}]},"created_at":{"type":"DateTime","value":"2026-06-10T01:44:34.573115Z"},"id":"gE2YQEWaA6WmMl8_AKWVcUTl3MxgU2PqJwlX3CS1dI2IoCJ9Fd-WNZLO9OWCB7xAC2vkQSKsCVMVsNTR2BaKDQ","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":33,"prompt_tokens":3443,"total_tokens":3476}},"updated_at":{"type":"DateTime","value":"2026-06-10T01:44:34.573115Z"}},{"chat_id":{"type":"String","value":"RizGROtDy5KOUCJ8iJ0OovnCIlqVH9KN9toDBtoCtf6xCfsUwyRRM-RIJdvBFfNHtvq6cEknANwPfCbD-stHHA"},"content":{"type":"String","value":"The price of the product **ekoDB** is **$99**. If you have any more questions or need assistance with anything else, feel free to ask!"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description","product","price"],"record":{"description":"A high-performance database product with AI capabilities","id":"yw1gzjFUyGHJBkFRHfEAxdPuY3g_CbI3L_a4-ZIErtb37j0xAvE1VBZXmdw8Fm3Pnrv_KJ1JXMasBktAYNeikA","price":99,"product":"ekoDB"},"score":2.0928571428571425}]},"created_at":{"type":"DateTime","value":"2026-06-10T01:44:34.575635Z"},"id":"3_hr5lLmc97dDRnwqDvHxwocqT6L3YLjqohvyXaSbsn_sQ58YzDWGHwj71GYsBH3-Ns75_zN7S3USua2WOVoEw","llm_model":{"type":"String","value":"gpt-4o-mini"},"llm_provider":{"type":"String","value":"openai"},"role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":33,"prompt_tokens":3443,"total_tokens":3476}},"updated_at":{"type":"DateTime","value":"2026-06-10T01:44:34.575635Z"}}]

=== Updating Session ===
✓ Updated session system prompt

=== Listing Sessions ===
✓ Total sessions: 16

=== Branching Session ===
✓ Created branched session: MyeiszV-0VMKHX2FYPpBcYfrmfW_u08JXQyOfY1hLwGlupztVndfwhJzJ2euzc2_VfBE-qsrOPUlwR8srKng7Q

=== Cleanup ===
✓ Deleted chat sessions
✓ Deleted collection: kotlin_chat_sessions_example

✓ Chat session management example completed successfully

BUILD SUCCESSFUL in 35s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Chat Models Example ===

=== List Chat Models ===
Available chat models by provider:
  openai:
    - text-embedding-ada-002
    - whisper-1
    - gpt-3.5-turbo
    - tts-1
    - gpt-3.5-turbo-16k
    - gpt-4-0613
    - gpt-4
    - davinci-002
    - babbage-002
    - gpt-3.5-turbo-instruct
    - gpt-3.5-turbo-instruct-0914
    - gpt-3.5-turbo-1106
    - tts-1-hd
    - tts-1-1106
    - tts-1-hd-1106
    - text-embedding-3-small
    - text-embedding-3-large
    - gpt-3.5-turbo-0125
    - gpt-4-turbo
    - gpt-4-turbo-2024-04-09
    - gpt-4o
    - gpt-4o-2024-05-13
    - gpt-4o-mini-2024-07-18
    - gpt-4o-mini
    - gpt-4o-2024-08-06
    - omni-moderation-latest
    - omni-moderation-2024-09-26
    - o1-2024-12-17
    - o1
    - o3-mini
    - o3-mini-2025-01-31
    - gpt-4o-2024-11-20
    - gpt-4o-mini-search-preview-2025-03-11
    - gpt-4o-mini-search-preview
    - gpt-4o-transcribe
    - gpt-4o-mini-transcribe
    - o1-pro-2025-03-19
    - o1-pro
    - gpt-4o-mini-tts
    - o3-2025-04-16
    - o4-mini-2025-04-16
    - o3
    - o4-mini
    - gpt-4.1-2025-04-14
    - gpt-4.1
    - gpt-4.1-mini-2025-04-14
    - gpt-4.1-mini
    - gpt-4.1-nano-2025-04-14
    - gpt-4.1-nano
    - gpt-image-1
    - o4-mini-deep-research
    - gpt-4o-transcribe-diarize
    - o4-mini-deep-research-2025-06-26
    - gpt-5-chat-latest
    - gpt-5-2025-08-07
    - gpt-5
    - gpt-5-mini-2025-08-07
    - gpt-5-mini
    - gpt-5-nano-2025-08-07
    - gpt-5-nano
    - gpt-audio-2025-08-28
    - gpt-realtime
    - gpt-realtime-2025-08-28
    - gpt-audio
    - gpt-5-codex
    - gpt-image-1-mini
    - gpt-5-pro-2025-10-06
    - gpt-5-pro
    - gpt-audio-mini
    - gpt-audio-mini-2025-10-06
    - gpt-5-search-api
    - gpt-realtime-mini
    - gpt-realtime-mini-2025-10-06
    - sora-2
    - sora-2-pro
    - gpt-5-search-api-2025-10-14
    - gpt-5.1-chat-latest
    - gpt-5.1-2025-11-13
    - gpt-5.1
    - gpt-5.1-codex
    - gpt-5.1-codex-mini
    - gpt-5.1-codex-max
    - gpt-image-1.5
    - gpt-5.2-2025-12-11
    - gpt-5.2
    - gpt-5.2-pro-2025-12-11
    - gpt-5.2-pro
    - gpt-5.2-chat-latest
    - gpt-4o-mini-transcribe-2025-12-15
    - gpt-4o-mini-transcribe-2025-03-20
    - gpt-4o-mini-tts-2025-03-20
    - gpt-4o-mini-tts-2025-12-15
    - gpt-realtime-mini-2025-12-15
    - gpt-audio-mini-2025-12-15
    - chatgpt-image-latest
    - gpt-5.2-codex
    - gpt-5.3-codex
    - gpt-realtime-1.5
    - gpt-audio-1.5
    - gpt-4o-search-preview
    - gpt-4o-search-preview-2025-03-11
    - gpt-5.3-chat-latest
    - gpt-5.4-2026-03-05
    - gpt-5.4-pro
    - gpt-5.4-pro-2026-03-05
    - gpt-5.4
    - gpt-5.4-nano-2026-03-17
    - gpt-5.4-nano
    - gpt-5.4-mini-2026-03-17
    - gpt-5.4-mini
    - gpt-image-2
    - gpt-image-2-2026-04-21
    - gpt-5.5
    - gpt-5.5-2026-04-23
    - gpt-5.5-pro
    - gpt-5.5-pro-2026-04-23
    - chat-latest
    - gpt-realtime-translate
    - gpt-realtime-2
    - gpt-realtime-whisper
  anthropic:
    - claude-fable-5
    - claude-opus-4-8
    - claude-opus-4-7
    - claude-sonnet-4-6
    - claude-opus-4-6
    - claude-opus-4-5-20251101
    - claude-haiku-4-5-20251001
    - claude-sonnet-4-5-20250929
    - claude-opus-4-1-20250805
    - claude-opus-4-20250514
    - claude-sonnet-4-20250514
  perplexity:
    - sonar
    - sonar-pro
    - sonar-deep-research
    - sonar-reasoning
    - sonar-reasoning-pro

=== Get OpenAI Models ===
OpenAI models: text-embedding-ada-002, whisper-1, gpt-3.5-turbo, tts-1, gpt-3.5-turbo-16k, gpt-4-0613, gpt-4, davinci-002, babbage-002, gpt-3.5-turbo-instruct, gpt-3.5-turbo-instruct-0914, gpt-3.5-turbo-1106, tts-1-hd, tts-1-1106, tts-1-hd-1106, text-embedding-3-small, text-embedding-3-large, gpt-3.5-turbo-0125, gpt-4-turbo, gpt-4-turbo-2024-04-09, gpt-4o, gpt-4o-2024-05-13, gpt-4o-mini-2024-07-18, gpt-4o-mini, gpt-4o-2024-08-06, omni-moderation-latest, omni-moderation-2024-09-26, o1-2024-12-17, o1, o3-mini, o3-mini-2025-01-31, gpt-4o-2024-11-20, gpt-4o-mini-search-preview-2025-03-11, gpt-4o-mini-search-preview, gpt-4o-transcribe, gpt-4o-mini-transcribe, o1-pro-2025-03-19, o1-pro, gpt-4o-mini-tts, o3-2025-04-16, o4-mini-2025-04-16, o3, o4-mini, gpt-4.1-2025-04-14, gpt-4.1, gpt-4.1-mini-2025-04-14, gpt-4.1-mini, gpt-4.1-nano-2025-04-14, gpt-4.1-nano, gpt-image-1, o4-mini-deep-research, gpt-4o-transcribe-diarize, o4-mini-deep-research-2025-06-26, gpt-5-chat-latest, gpt-5-2025-08-07, gpt-5, gpt-5-mini-2025-08-07, gpt-5-mini, gpt-5-nano-2025-08-07, gpt-5-nano, gpt-audio-2025-08-28, gpt-realtime, gpt-realtime-2025-08-28, gpt-audio, gpt-5-codex, gpt-image-1-mini, gpt-5-pro-2025-10-06, gpt-5-pro, gpt-audio-mini, gpt-audio-mini-2025-10-06, gpt-5-search-api, gpt-realtime-mini, gpt-realtime-mini-2025-10-06, sora-2, sora-2-pro, gpt-5-search-api-2025-10-14, gpt-5.1-chat-latest, gpt-5.1-2025-11-13, gpt-5.1, gpt-5.1-codex, gpt-5.1-codex-mini, gpt-5.1-codex-max, gpt-image-1.5, gpt-5.2-2025-12-11, gpt-5.2, gpt-5.2-pro-2025-12-11, gpt-5.2-pro, gpt-5.2-chat-latest, gpt-4o-mini-transcribe-2025-12-15, gpt-4o-mini-transcribe-2025-03-20, gpt-4o-mini-tts-2025-03-20, gpt-4o-mini-tts-2025-12-15, gpt-realtime-mini-2025-12-15, gpt-audio-mini-2025-12-15, chatgpt-image-latest, gpt-5.2-codex, gpt-5.3-codex, gpt-realtime-1.5, gpt-audio-1.5, gpt-4o-search-preview, gpt-4o-search-preview-2025-03-11, gpt-5.3-chat-latest, gpt-5.4-2026-03-05, gpt-5.4-pro, gpt-5.4-pro-2026-03-05, gpt-5.4, gpt-5.4-nano-2026-03-17, gpt-5.4-nano, gpt-5.4-mini-2026-03-17, gpt-5.4-mini, gpt-image-2, gpt-image-2-2026-04-21, gpt-5.5, gpt-5.5-2026-04-23, gpt-5.5-pro, gpt-5.5-pro-2026-04-23, chat-latest, gpt-realtime-translate, gpt-realtime-2, gpt-realtime-whisper

=== Get Anthropic Models ===
Anthropic models: claude-fable-5, claude-opus-4-8, claude-opus-4-7, claude-sonnet-4-6, claude-opus-4-6, claude-opus-4-5-20251101, claude-haiku-4-5-20251001, claude-sonnet-4-5-20250929, claude-opus-4-1-20250805, claude-opus-4-20250514, claude-sonnet-4-20250514

=== Get Non-Existent Provider ===
Expected error for non-existent provider: Request failed with status 405: {"code":405,"message":"Method Not Allowed"}

=== Chat Models Example Complete ===

BUILD SUCCESSFUL in 6s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - User Functions Example ===

=== Create User Function ===
Created user function with ID: eUny3rBN74Ja8RuUmQGhVEKhUdWvrM8R8zEfc_j4HREQvracvK_UV7JZBnKYR-7PRUubSAOMm1YLuoGJIsEnGA

=== Get User Function ===
Retrieved: "get_active_users_kt" - "Get Active Users (Kotlin)"
Description: "Fetches all users and filters by active status"

=== List All User Functions ===
Found 11 user functions:
  - "cache_api_call_py": "Cache External API Call"
  - "get_active_users_kt": "Get Active Users (Kotlin)"
  - "cache_api_call_js": "Cache External API Call"
  - "fetch_slim_user": "Validate and slim down user"
  - "fetch_user": "Fetch user by code"

=== List User Functions by Tag ===
Found 11 user functions with 'kotlin' tag:
  - "cache_api_call_py"
  - "get_active_users_kt"
  - "cache_api_call_js"
  - "fetch_slim_user"
  - "fetch_user"
  - "fetch_and_store_user"
  - "validate_user"
  - "fetch_api_user"
  - "swr_user"
  - "fetch_product_with_reviews"
  - "cache_api_call"

=== Update User Function ===
User function updated successfully

=== Delete User Function ===
User function deleted successfully

=== User Functions Example Complete ===

BUILD SUCCESSFUL in 3s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
=== ekoDB Convenience Methods Example ===

SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== Native Object Creation ===
✓ Created record: EkoRecord(fields={id=StringValue(value=NYP20-zCBhPCkmSDjod8w85HLLvBgSzUMkOWDJ0EvDqiJMGZOnEAqih_6lA9ZyZRjZozm1MjwTylvKY1f8KI6g)})

=== Upsert Operation ===
✓ First upsert (update): EkoRecord(fields={name=ObjectValue(value={value=StringValue(value=Alice Johnson), type=StringValue(value=String)}), email=ObjectValue(value={value=StringValue(value=alice.j@newdomain.com), type=StringValue(value=String)}), id=StringValue(value=NYP20-zCBhPCkmSDjod8w85HLLvBgSzUMkOWDJ0EvDqiJMGZOnEAqih_6lA9ZyZRjZozm1MjwTylvKY1f8KI6g), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), age=ObjectValue(value={value=IntegerValue(value=29), type=StringValue(value=Integer)})})
✓ Second upsert (insert): EkoRecord(fields={id=StringValue(value=uU1R1Vbch38CJU6v4ia0RCLnuriPTwKw4Xo2x4pfwrK-Urflyx2-aMqrYCO_uhvfpmlq2YwMk4UI-Tkmp1-14g)})

=== Find One Operation ===
✓ Found user by email: EkoRecord(fields={age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=29)}), id=StringValue(value=NYP20-zCBhPCkmSDjod8w85HLLvBgSzUMkOWDJ0EvDqiJMGZOnEAqih_6lA9ZyZRjZozm1MjwTylvKY1f8KI6g), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice.j@newdomain.com)}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)})})
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

BUILD SUCCESSFUL in 10s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
=== Bypass Ripple Example ===

SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
1. Basic insert (ripple enabled):
   Inserted with ripple: EkoRecord(fields={id=StringValue(value=zLIpXeMeLglvFtvcgcrDLPiBMsBbGbMcY0UQQfAUIwuMc-x5ajhYVwZuICzq-n7ao6yZZWRxtgP3Zu-v-w-OfQ)})

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: EkoRecord(fields={id=StringValue(value=0GshNI5tWiDq77_J-MWowoFQpW3SpfXJtQm8DR5m-Ps5v-uIJf0uE47Vlkoanc-_oobqGRI7euF4aamOhKzPtg)})

3. Update with bypass_ripple:
   Updated with bypass_ripple: EkoRecord(fields={name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Product 1)}), id=StringValue(value=zLIpXeMeLglvFtvcgcrDLPiBMsBbGbMcY0UQQfAUIwuMc-x5ajhYVwZuICzq-n7ao6yZZWRxtgP3Zu-v-w-OfQ), price=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=150)})})

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: EkoRecord(fields={name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Upsert Product)}), id=StringValue(value=zLIpXeMeLglvFtvcgcrDLPiBMsBbGbMcY0UQQfAUIwuMc-x5ajhYVwZuICzq-n7ao6yZZWRxtgP3Zu-v-w-OfQ), price=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=500)})})

✅ All bypass_ripple operations completed successfully!

BUILD SUCCESSFUL in 3s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields
Fetched 3 users with only 3 fields each

Example 2: Exclude sensitive fields
Fetched 2 admins without sensitive data
  Password field excluded: true

Example 3: Complex query with projection
Fetched 3 active users with profile fields

Example 4: Find by ID with projection
Fetched user profile: Alice Johnson

Example 5: Compare full vs projected data
Full query returned 12 fields per user
Projected query returned 3 fields per user

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!

BUILD SUCCESSFUL in 3s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
✓ Client created
✓ kt_users_register saved
✓ kt_users_login saved
✓ kt_users_verify_token saved

=== Auth flow defined as pure stored functions ===
Call them like:
  POST /api/functions/kt_users_register { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/kt_users_login    { "email": "a@b.com", "password": "s3cret" }
  POST /api/functions/kt_users_verify_token { "token": "<jwt>" }

✓ Cleaned up demo functions

BUILD SUCCESSFUL in 3s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
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

BUILD SUCCESSFUL in 3s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
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

BUILD SUCCESSFUL in 3s
2 actionable tasks: 1 executed, 1 up-to-date
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
✓ Client created
✓ kt_route_admin saved
✓ kt_route_user_by_id saved
✓ kt_route_user_posts saved
✓ kt_route_org_create_member saved

Try them with curl:
  curl http://localhost:8080/api/route/users/admin
  curl http://localhost:8080/api/route/users/42
  curl http://localhost:8080/api/route/users/42/posts/7
  curl -X POST http://localhost:8080/api/route/orgs/acme/members \
       -H 'Content-Type: application/json' -d '{"name":"alice"}'

✓ Cleaned up demo functions

BUILD SUCCESSFUL in 3s
2 actionable tasks: 1 executed, 1 up-to-date
✅ [32mKotlin client examples complete![0m
🟣 [33mKotlin Transactions...[0m
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: QPFr2zJ_AqCjhrYmrbSzhS2veG8Ibqv9V32r4EJMHka3TlGoCS2SKJ5GmEhGQOweii8RxICUpkKSYWYWbZx5ZQ
Created Bob: $500 - ID: cjyVBnCNI75jQd-whAK5B_UVGE3UzF2QCS9lA22SsoR1GnUiYUFbppX8AbPo6iZeDJrtGRTpNBQ09IDB23BkwA

=== Example 1: Begin Transaction ===
Transaction ID: 32c9527c-a5ef-4dc8-8ff1-f102e22583e0

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 60cb27ce-1f9c-4885-8775-eda601ba5f81
Status before rollback: Active
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
✅ [32mAll Kotlin integration tests complete![0m
🐍 [36mBuilding Python client package...[0m
🔧 [36mEnsuring maturin is available in .venv...[0m
🔨 [36mBuilding wheel...[0m
🍹 Building a mixed python/rust project
🐍 Found CPython 3.14 at /opt/homebrew/opt/python@3.14/bin/python3.14
🔗 Found pyo3 bindings with abi3 support
💻 Using `MACOSX_DEPLOYMENT_TARGET=11.0` for aarch64-apple-darwin by default
    Finished `release` profile [optimized] target(s) in 0.29s
📦 Built wheel for abi3 Python ≥ 3.8 to ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.21.0-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel into .venv...[0m
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.21.0-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.21.0
    Uninstalling ekodb_client-0.21.0:
      Successfully uninstalled ekodb_client-0.21.0
Successfully installed ekodb-client-0.21.0
✅ [32mPython client package built and installed![0m
📦 [36mEnsuring Python example dependencies in .venv...[0m

🤖 [36mRAG Conversation System Examples[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m

[33mPrerequisites:[0m
  1. ekoDB server running (make run in ekodb/)
  2. OPENAI_API_KEY set in server environment
  3. API_BASE_URL and API_BASE_KEY exported in your shell

[36mBuilding Rust client library...[0m
✓ Rust client built

✓ Python client built and installed into .venv (via build-python-client prerequisite)

[36mBuilding TypeScript client library...[0m
✓ TypeScript client built

[36mInstalling TypeScript client in examples...[0m
✓ TypeScript client installed

[36mBuilding TypeScript example...[0m
✓ TypeScript example built

[36mBuilding Go client library...[0m
✓ Go client built

[36mBuilding Go RAG example...[0m
✓ Go example built

[36mBuilding Kotlin client library...[0m
✓ Kotlin client built

[36mBuilding Kotlin RAG example...[0m
✓ Kotlin example built

[36mRunning Rust RAG Example...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
=== ekoDB RAG Conversation System ===

This example shows how ekoDB can power a self-improving AI system
that learns from its own conversation history.

=== Step 1: Building Conversation History ===
Storing previous conversations with embeddings...

  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 34 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 315.595084ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 213.164583ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 276.639542ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 251.90975ms
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 249.766417ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 212.305958ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 212.265083ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 257.316583ms
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 206.261125ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 218.979667ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 224.06475ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 297.412625ms
    • Function auto-cleaned up by client
✓ Stored performance optimization conversation (4 messages)

=== Step 2: New User Question with Context Retrieval ===
User asks: "How do I write memory-safe high-performance database code?"

=== Step 3: Searching Related Context ===
Using hybrid search to find relevant messages from all conversations...


→ Generating embedding for user question...
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 229.19875ms
    • Function auto-cleaned up by client

→ Executing hybrid_search()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 89.921333ms

✓ Found 5 related messages across all conversations:
  1. From conv_database_design
     Use NoSQL when you need: flexible schemas, horizontal scaling, high write throughput, or when working with unstructured data. SQL is better for complex queries, ACID transactions, and structured data with well-defined relationships.

  2. From conv_database_design
     What is database normalization?

  3. From conv_database_design
     Database normalization is the process of organizing data to reduce redundancy and improve data integrity. It involves dividing large tables into smaller ones and defining relationships between them using foreign keys.

  4. From conv_database_design
     When should I use NoSQL over SQL?

  5. From conv_performance
     How can I optimize database queries?

=== Step 4: Generating Context-Aware Response ===
✓ AI Response (with context from 3 conversations):

Writing memory-safe, high-performance database code involves several key practices that help ensure both safety and efficiency. Here's a comprehensive approach:

### 1. Choose the Right Database System
- **SQL vs. NoSQL**: Select the appropriate database based on your data structure needs:
  - Use **SQL** for structured data with complex relationships and requiring ACID compliance.
  - Opt for **NoSQL** for flexible schemas, unstructured data, or high write throughput requirements.

### 2. Use Prepared Statements
- **Avoid SQL Injection**: Use prepared statements or parameterized queries to prevent SQL injection attacks.
- **Performance**: Prepared statements can also improve performance as the query parsing can be reused.

### 3. Implement Connection Pooling
- **Reuse Connections**: Instead of creating new database connections for every operation, utilize a connection pool to reuse existing connections, reducing overhead.
  
### 4. Optimize Queries
- **Indexing**: Create indexes on frequently queried columns to speed up lookups.
- **Query Analysis**: Use tools to analyze and understand query execution plans. For instance, in database systems, utilize EXPLAIN commands to see how queries are executed and optimize based on those insights.
- **Selective Columns**: Fetch only the columns you need rather than using SELECT *.

### 5. Manage Transactions Wisely
- **Atomic Operations**: Group database operations into transactions to ensure atomicity and consistency.
- **Avoid Long Transactions**: Keep the transaction scope small to minimize locking and contention.

### 6. Monitor and Adjust
- **Logging and Monitoring**: Implement logging to monitor queries and check for performance bottlenecks.
- **Performance Metrics**: Regularly review performance metrics and adapt your queries and indexes as necessary.

### 7. Memory Management
- **Data Types Optimization**: Use appropriate data types for your fields to save memory and improve performance.
- **Batch Operations**: Use batch inserts and updates to minimize the number of database hits, enhancing performance and reducing memory consumption.

### 8. Safe Concurrency
- **Locking Mechanisms**: Implement proper locking strategies to avoid race conditions while ensuring that they do not lead to deadlocks.
- **Optimistic Concurrency Control**: Use version numbers or timestamps to manage concurrent updates to data safely.

### 9. Use ORM Tools Wisely
- **Memory Efficiency**: When using Object-Relational Mappers (ORMs), ensure efficient mappings and lazy loading where necessary to avoid loading unnecessary data into memory.
  
### 10. Leverage Caching
- **In-Memory Caching**: Implement caching solutions like Redis or Memcached to reduce load on the database for frequently accessed data.

### Example Implementation in Pseudocode
Here’s a simple example of a safe and efficient database interaction code snippet using prepared statements in a pseudo-language:

```pseudocode
connection = ConnectionPool.getConnection()

try {
    // Begin Transaction
    connection.beginTransaction()

    // Prepare Statement
    stmt = connection.prepareStatement("INSERT INTO users (email, name) VALUES (?, ?)")
    stmt.setString(1, userEmail)
    stmt.setString(2, userName)

    // Execute Update
    stmt.executeUpdate()

    // Commit Transaction
    connection.commit()
} catch(SQLException e) {
    // Rollback on error
    connection.rollback()
    log.error("Database operation failed: " + e.message)
} finally {
    stmt.close()
    ConnectionPool.releaseConnection(connection)
}
```

### Conclusion
By incorporating these practices into your development strategy, you can create database code that is not only high-performance but also memory-safe. Regularly revisiting and updating your database strategies as requirements evolve will ensure sustained optimal performance.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 251.733041ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 3855 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 583.728542ms
    • Function auto-cleaned up by client
✓ New conversation stored and indexed for future retrieval

=== Step 6: Cross-Conversation Search ===
Searching for messages about 'ownership' across ALL conversations...


→ Executing text_search()...
  • Collection: rag_messages
  • Query: "ownership system"
  • Limit: 3 results
  • Search method: Full-text with fuzzy matching & stemming
  • No vector embeddings needed - pure keyword search
  ✓ Text search completed in 54.601375ms

✓ Found 3 messages mentioning ownership:
  1. From conv_rust_programming: Rust's key features include: memory safety without garbage collection, zero-cost abstractions, ownership system, powerful type system, and excellent concurrency support.

  2. From conv_performance: Rust's ownership system provides zero-cost memory management. Use Box for heap allocation, Rc/Arc for shared ownership, and avoid cloning large data structures. The compiler optimizes away unnecessary allocations.

  3. From conv_new_question: Writing memory-safe, high-performance database code involves several key practices that help ensure both safety and efficiency. Here's a comprehensive approach:

### 1. Choose the Right Database System
- **SQL vs. NoSQL**: Select the appropriate database based on your data structure needs:
  - Use **SQL** for structured data with complex relationships and requiring ACID compliance.
  - Opt for **NoSQL** for flexible schemas, unstructured data, or high write throughput requirements.

### 2. Use Prepared Statements
- **Avoid SQL Injection**: Use prepared statements or parameterized queries to prevent SQL injection attacks.
- **Performance**: Prepared statements can also improve performance as the query parsing can be reused.

### 3. Implement Connection Pooling
- **Reuse Connections**: Instead of creating new database connections for every operation, utilize a connection pool to reuse existing connections, reducing overhead.
  
### 4. Optimize Queries
- **Indexing**: Create indexes on frequently queried columns to speed up lookups.
- **Query Analysis**: Use tools to analyze and understand query execution plans. For instance, in database systems, utilize EXPLAIN commands to see how queries are executed and optimize based on those insights.
- **Selective Columns**: Fetch only the columns you need rather than using SELECT *.

### 5. Manage Transactions Wisely
- **Atomic Operations**: Group database operations into transactions to ensure atomicity and consistency.
- **Avoid Long Transactions**: Keep the transaction scope small to minimize locking and contention.

### 6. Monitor and Adjust
- **Logging and Monitoring**: Implement logging to monitor queries and check for performance bottlenecks.
- **Performance Metrics**: Regularly review performance metrics and adapt your queries and indexes as necessary.

### 7. Memory Management
- **Data Types Optimization**: Use appropriate data types for your fields to save memory and improve performance.
- **Batch Operations**: Use batch inserts and updates to minimize the number of database hits, enhancing performance and reducing memory consumption.

### 8. Safe Concurrency
- **Locking Mechanisms**: Implement proper locking strategies to avoid race conditions while ensuring that they do not lead to deadlocks.
- **Optimistic Concurrency Control**: Use version numbers or timestamps to manage concurrent updates to data safely.

### 9. Use ORM Tools Wisely
- **Memory Efficiency**: When using Object-Relational Mappers (ORMs), ensure efficient mappings and lazy loading where necessary to avoid loading unnecessary data into memory.
  
### 10. Leverage Caching
- **In-Memory Caching**: Implement caching solutions like Redis or Memcached to reduce load on the database for frequently accessed data.

### Example Implementation in Pseudocode
Here’s a simple example of a safe and efficient database interaction code snippet using prepared statements in a pseudo-language:

```pseudocode
connection = ConnectionPool.getConnection()

try {
    // Begin Transaction
    connection.beginTransaction()

    // Prepare Statement
    stmt = connection.prepareStatement("INSERT INTO users (email, name) VALUES (?, ?)")
    stmt.setString(1, userEmail)
    stmt.setString(2, userName)

    // Execute Update
    stmt.executeUpdate()

    // Commit Transaction
    connection.commit()
} catch(SQLException e) {
    // Rollback on error
    connection.rollback()
    log.error("Database operation failed: " + e.message)
} finally {
    stmt.close()
    ConnectionPool.releaseConnection(connection)
}
```

### Conclusion
By incorporating these practices into your development strategy, you can create database code that is not only high-performance but also memory-safe. Regularly revisiting and updating your database strategies as requirements evolve will ensure sustained optimal performance.

=== System Statistics ===

→ Querying database statistics...
  • Using find_all() helper - simplified query API

📊 Database Statistics:
  • Total conversations: 4
  • Total messages stored: 14
  • All messages indexed for vector search ✓
  • All messages indexed for text search ✓
  • All messages queryable by metadata ✓

=== Step 8: Dynamic Search Configuration ===
Each conversation can have its own search config...

💡 Conversations can store custom search configurations:
  • Search type: hybrid, text, or vector
  • Relevance thresholds
  • Filter by tags or metadata
  • Collection-specific settings
  • Per-conversation AI behavior

This enables context-aware search tuned to each conversation's needs!


=== Cleanup ===
Deleting example collections...

✅ All done! RAG system demonstrated successfully.

✓ Using search results to enhance AI responses (RAG)
✓ Building a self-improving knowledge base
✓ Dynamic search configurations per conversation

ekoDB provides everything needed for AI-powered applications:
  • Vector search (semantic similarity)
  • Text search (keyword matching)
  • Hybrid search (best of both)
  • AI functions (Chat, Embed)
  • Flexible querying and filtering
  • All in one database - no external dependencies!


[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning Python RAG Example...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
=== ekoDB RAG Conversation System ===

This example shows how ekoDB can power a self-improving AI system
that learns from its own conversation history.

=== Step 1: Building Conversation History ===
Storing previous conversations with embeddings...

  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 34 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 1.293s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.353s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.329s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.265s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.253s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.271s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.243s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.187s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.202s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.268s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.217s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.208s
    • Function auto-cleaned up by client
✓ Stored performance optimization conversation (4 messages)

=== Step 2: New User Question with Context Retrieval ===
User asks: "How do I write memory-safe high-performance database code?"

=== Step 3: Searching Related Context ===
Using hybrid search to find relevant messages from all conversations...


→ Generating embedding for user question...
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.242s
    • Function auto-cleaned up by client

→ Executing hybrid_search()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.070s

✓ Found 5 related messages across all conversations:
  1. [Score: 0.331] From conv_database_design
     Database normalization is the process of organizing data to reduce redundancy and improve data integrity. It involves dividing large tables into smaller ones and defining relationships between them using foreign keys.

  2. [Score: 0.300] From conv_database_design
     Use NoSQL when you need: flexible schemas, horizontal scaling, high write throughput, or when working with unstructured data. SQL is better for complex queries, ACID transactions, and structured data with well-defined relationships.

  3. [Score: 0.300] From conv_database_design
     What is database normalization?

  4. [Score: 0.256] From conv_database_design
     When should I use NoSQL over SQL?

  5. [Score: 0.188] From conv_performance
     How can I optimize database queries?

=== Step 4: Generating Context-Aware Response ===
✓ AI Response (with context from 3 conversations):

Writing memory-safe, high-performance database code is an essential skill, especially for applications that require robust data management and responsiveness. Here are several best practices and principles that can help you achieve this objective:

### 1. **Choose the Right Database Technology**
   - **Select between SQL and NoSQL** based on your use case. SQL databases are excellent for structured data with complex queries, while NoSQL is better for unstructured data and horizontal scaling.

### 2. **Use Prepared Statements**
   - **Prepared statements** can help prevent SQL injection attacks and also improve performance by allowing the database to optimize execution plans.

   ```sql
   -- Example in pseudo-SQL
   PREPARE stmt FROM 'SELECT * FROM users WHERE email = ?';
   SET @email = 'user@example.com';
   EXECUTE stmt USING @email;
   ```

### 3. **Connection Pooling**
   - Use **connection pooling** to reuse existing database connections. This reduces the overhead of establishing a new connection for every query.

### 4. **Efficient Query Design**
   - Optimize your queries by:
     - **Using indexes** wisely to speed up data retrieval.
     - **Limiting the amount of data** returned by selecting only the columns you need.
     - Employing **pagination** to retrieve data in chunks rather than all at once.

### 5. **Database Normalization**
   - **Normalize** your database design to reduce redundancy and improve data integrity. However, be cautious of over-normalization, which might lead to complex queries.

### 6. **Consider Memory Management**
   - Ensure that your code handles memory allocation and deallocation properly, especially in languages like C or C++. Use smart pointers or similar constructs to manage memory automatically where applicable.

### 7. **Asynchronous Operations**
   - Use **asynchronous programming** techniques to allow your application to continue running while waiting for database operations to complete.

### 8. **Caching**
   - Implement **caching** strategies to reduce database load and improve response times. Use in-memory caches like Redis or Memcached for frequently accessed data.

### 9. **Transaction Management**
   - Use transactions to ensure that your database remains consistent, especially when performing multiple related operations. Ensure to handle **commit** and **rollback** properly in case of errors.

### 10. **Monitor and Profile**
   - Continuously monitor your database performance using profiling tools to identify slow queries and optimize them. Tools like `EXPLAIN` in SQL can help analyze query performance.

### 11. **Error Handling**
   - Implement robust error handling for database operations to safeguard against potential runtime exceptions and to ensure that resources are cleaned up correctly.

### 12. **Testing and Debugging**
   - Write unit tests and conduct performance testing to ensure that your database code performs well under expected loads.

### 13. **Use ORM Tools Judiciously**
   - While ORM (Object-Relational Mapping) tools can simplify database interactions, be aware that they can sometimes introduce performance overhead. Profile ORM-generated queries and consider raw queries for performance-critical paths.

### Conclusion
By following these best practices, you can write memory-safe and high-performance database code. This ensures that your applications are efficient, scalable, and less prone to common pitfalls like memory leaks and performance degradation. Always be proactive in monitoring and optimizing your database interactions to adapt to changing loads and usage patterns.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.234s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 3609 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.240s
    • Function auto-cleaned up by client
✓ New conversation stored and indexed for future retrieval

=== Step 6: Cross-Conversation Search ===
Searching for messages about 'ownership' across ALL conversations...

✓ Found 3 messages mentioning ownership:
  1. From conv_rust_programming: Rust's key features include: memory safety without garbage collection, zero-cost abstractions, ownership system, powerful type system, and excellent concurrency support.

  2. From conv_performance: Rust's ownership system provides zero-cost memory management. Use Box for heap allocation, Rc/Arc for shared ownership, and avoid cloning large data structures. The compiler optimizes away unnecessary allocations.

  3. From conv_rust_programming: The borrow checker enforces Rust's ownership rules at compile time. It ensures that references don't outlive the data they point to and prevents data races by allowing either multiple immutable references or one mutable reference.

=== System Statistics ===
Total conversations: 4
Total messages stored: 14
All messages are indexed for vector search ✓
All messages are indexed for text search ✓
All messages are queryable by metadata ✓

=== Step 8: Dynamic Search Configuration ===
Each conversation can have its own search config...

💡 Conversations can store custom search configurations:
  • Search type: hybrid, text, or vector
  • Relevance thresholds
  • Filter by tags or metadata
  • Collection-specific settings
  • Per-conversation AI behavior

This enables context-aware search tuned to each conversation's needs!

=== Cleanup ===
✓ Cleanup complete


=== 📚 Summary: What This Example Showed ===

🔧 ekoDB Native Capabilities Used:
  ✓ Functions with Embed operation (AI integration)
  ✓ Hybrid Search (text + vector combined)
  ✓ Text Search (full-text with stemming)
  ✓ Automatic embedding generation
  ✓ Cross-collection queries

🚀 New Client Helper Methods:
  • client.embed(text, model) - Generate embeddings
  • client.hybrid_search() - Semantic + keyword search
  • client.text_search() - Full-text search
  • client.find_all() - Query all documents

💡 Key Takeaways:
  1. ekoDB handles AI Functions natively - no external services needed
  2. One-line embedding generation with auto-cleanup
  3. Hybrid search combines semantic understanding + keyword matching
  4. Perfect for RAG: store, search, and retrieve context
  5. All AI capabilities accessible through simple client methods

🎯 Build production RAG systems with ekoDB!
   → Set OPENAI_API_KEY in your ekoDB server environment
   → Use these client helpers to make AI integration simple
   → Scale to millions of documents with native indexing


[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning TypeScript RAG Example...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
=== ekoDB RAG Conversation System ===

This example shows how ekoDB can power a self-improving AI system
that learns from its own conversation history.

=== Step 1: Building Conversation History ===
Storing previous conversations with embeddings...

  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 34 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.198s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.192s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.204s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.389s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.205s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.298s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.220s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.225s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.208s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.623s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.230s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.330s
    • Function auto-cleaned up by client
✓ Stored performance optimization conversation (4 messages)

=== Step 2: New User Question with Context Retrieval ===
User asks: "How do I write memory-safe high-performance database code?"

=== Step 3: Searching Related Context ===
Using hybrid search to find relevant messages from all conversations...


→ Generating embedding for user question...
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.300s
    • Function auto-cleaned up by client

→ Executing hybridSearch()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.046s
✓ Found 5 related messages across all conversations:
  1. [Score: 0.199] From conv_database_design
     Database normalization is the process of organizing data to reduce redundancy and improve data integrity. It involves dividing large tables into smaller ones and defining relationships between them using foreign keys.

  2. [Score: 0.180] From conv_database_design
     Use NoSQL when you need: flexible schemas, horizontal scaling, high write throughput, or when working with unstructured data. SQL is better for complex queries, ACID transactions, and structured data with well-defined relationships.

  3. [Score: 0.180] From conv_database_design
     What is database normalization?

  4. [Score: 0.154] From conv_database_design
     When should I use NoSQL over SQL?

  5. [Score: 0.112] From conv_performance
     How can I optimize database queries?

=== Step 4: Generating Context-Aware Response ===
✓ AI Response (with context from 3 conversations):

Writing memory-safe high-performance database code involves a combination of good programming practices, understanding database interactions, and leveraging appropriate technologies. Here’s a comprehensive guide to help you achieve this:

### 1. Use Safe Programming Languages
Choose programming languages that support memory safety features, such as:

- **Rust**: Guarantees memory safety without a garbage collector.
- **Go**: Offers garbage collection and strong typing to prevent common memory issues.
- **Java/C#**: These languages handle memory management automatically, reducing the risk of memory leaks.

### 2. Optimize Database Queries
- **Use Prepared Statements**: They help to prevent SQL injection attacks and can be reused, which speeds up query execution.
- **Filter Early**: Apply filters to minimize the amount of data transferred between the database and your application.
- **Select Only Necessary Fields**: Avoid using `SELECT *`; specify only the fields you need to reduce data size.

### 3. Connection Management
- Use connection pooling to minimize the overhead of opening and closing database connections. Libraries for database access (like DBeaver, SQLAlchemy in Python, or Hibernate in Java) often include connection pooling mechanisms.

### 4. Indexing
- Properly index your databases to speed up read operations. Analyze query patterns to create indexes that optimize performance for the most common queries.

### 5. Handle Transactions Wisely
- Use transactions to ensure data integrity. Group related database operations in a single transaction to avoid partial updates.
- Keep transactions as short as possible to reduce locking and improve concurrency.

### 6. Asynchronous Operations
Consider using asynchronous database operations to prevent blocking threads. This is crucial in web applications to maintain responsiveness while waiting for database responses.

### 7. Leverage ORM Tools
Object-Relational Mapping (ORM) tools can help in managing database interactions while providing built-in safety features for data operations:
- Ensure you choose ORMs that promote safe practices and optimize generated SQL queries.

### 8. Monitor Performance
- Use database profiling tools to identify slow queries and performance bottlenecks.
- Regularly review and refactor your code and queries based on performance statistics.

### 9. Error Handling
Implement robust error handling for database interactions. This includes catching exceptions and implementing retry logic for transient failures.

### 10. Optimize Data Structures
- Choose the right data types and structures for your database schema to save space and improve performance.
- Consider denormalization for frequently-read datasets, but balance it against the potential increase in data redundancy.

### 11. Consistent Concurrency Control
- Use optimistic or pessimistic concurrency control based on the application needs to manage data integrity and avoid race conditions.

### 12. Use Framework Features Carefully
Leverage features in your development framework that promote safety, such as type-checking, garbage collection, and built-in protections against common vulnerabilities.

### 13. Regular Maintenance
Regularly update your database, apply patches, and monitor logs to catch any issues early.

### Conclusion
By following the practices above, you can write memory-safe, high-performance database code that mitigates risks of memory issues and promotes efficient database interactions. It's important to continuously learn and adapt to new tools and methodologies that improve safety and performance in database programming.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.224s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 3627 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.334s
    • Function auto-cleaned up by client
✓ New conversation stored and indexed for future retrieval

=== Step 6: Cross-Conversation Search ===
Searching for messages about 'ownership' across ALL conversations...


→ Executing textSearch()...
  • Collection: rag_messages
  • Query: "ownership system"
  • Limit: 3 results
  • Search method: Full-text with fuzzy matching & stemming
  • No vector embeddings needed - pure keyword search
  ✓ Text search completed in 0.037s
✓ Found 3 messages mentioning ownership:
  1. From conv_rust_programming: Rust's key features include: memory safety without garbage collection, zero-cost abstractions, ownership system, powerful type system, and excellent concurrency support.

  2. From conv_performance: Rust's ownership system provides zero-cost memory management. Use Box for heap allocation, Rc/Arc for shared ownership, and avoid cloning large data structures. The compiler optimizes away unnecessary allocations.

  3. From conv_rust_programming: The borrow checker enforces Rust's ownership rules at compile time. It ensures that references don't outlive the data they point to and prevents data races by allowing either multiple immutable references or one mutable reference.

=== System Statistics ===

→ Querying database statistics...
  • Using findAllWithLimit() helper - simplified query API

📊 Database Statistics:
  • Total conversations: 4
  • Total messages stored: 14
  • All messages indexed for vector search ✓
  • All messages indexed for text search ✓
  • All messages queryable by metadata ✓

=== Step 8: Dynamic Search Configuration ===
Each conversation can have its own search config...

💡 Conversations can store custom search configurations:
  • Search type: hybrid, text, or vector
  • Relevance thresholds
  • Filter by tags or metadata
  • Collection-specific settings
  • Per-conversation AI behavior

This enables context-aware search tuned to each conversation's needs!

=== Cleanup ===
✓ Cleanup complete


=== 📚 Summary: What This Example Showed ===

🔧 ekoDB Native Capabilities Used:
  ✓ Functions with Embed operation (AI integration)
  ✓ Hybrid Search (text + vector combined)
  ✓ Text Search (full-text with stemming)
  ✓ Automatic embedding generation
  ✓ Cross-collection queries

🚀 New Client Helper Methods:
  • client.embed(text, model) - Generate embeddings
  • client.hybridSearch() - Semantic + keyword search
  • client.textSearch() - Full-text search
  • client.findAllWithLimit() - Query all documents

💡 Key Takeaways:
  1. ekoDB handles AI Functions natively - no external services needed
  2. One-line embedding generation with auto-cleanup
  3. Hybrid search combines semantic understanding + keyword matching
  4. Perfect for RAG: store, search, and retrieve context
  5. All AI capabilities accessible through simple client methods

🎯 Build production RAG systems with ekoDB!
   → Set OPENAI_API_KEY in your ekoDB server environment
   → Use these client helpers to make AI integration simple
   → Scale to millions of documents with native indexing


[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning Go RAG Example...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
=== ekoDB RAG Conversation System ===

This example shows how ekoDB can power a self-improving AI system
that learns from its own conversation history.

=== Step 1: Building Conversation History ===
Storing previous conversations with embeddings...

  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 34 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.233s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.215s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.226s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.213s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.254s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.173s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 1.160s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.197s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.263s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.220s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.252s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.223s
    • Function auto-cleaned up by client
✓ Stored performance optimization conversation (4 messages)

=== Step 2: New User Question with Context Retrieval ===
User asks: "How do I write memory-safe high-performance database code?"

=== Step 3: Searching Related Context ===
Using hybrid search to find relevant messages from all conversations...


→ Generating embedding for user question...
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.183s
    • Function auto-cleaned up by client

→ Executing HybridSearch()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.081s

✓ Found 5 related messages across all conversations:
  1. [Score: 0.474] From conv_performance
     How can I optimize database queries?

  2. [Score: 0.466] From conv_database_design
     Use NoSQL when you need: flexible schemas, horizontal scaling, high write throughput, or when working with unstructured data. SQL is better for complex queries, ACID transactions, and structured data with well-defined relationships.

  3. [Score: 0.449] From conv_database_design
     When should I use NoSQL over SQL?

  4. [Score: 0.434] From conv_database_design
     What is database normalization?

  5. [Score: 0.405] From conv_database_design
     Database normalization is the process of organizing data to reduce redundancy and improve data integrity. It involves dividing large tables into smaller ones and defining relationships between them using foreign keys.

=== Step 4: Generating Context-Aware Response ===
✓ AI Response (with context from 3 conversations):

Writing memory-safe, high-performance database code involves several best practices that ensure efficient resource utilization while maintaining data integrity and security. Here are some key strategies and considerations:

### 1. **Use Parameterized Queries:**
   - Always use parameterized queries or prepared statements to prevent SQL injection attacks. This also often improves performance as the database can cache the execution plan.

### 2. **Connection Pooling:**
   - Use connection pools to manage database connections efficiently. This reduces the overhead of establishing connections for every database interaction and minimizes resource usage.

### 3. **Efficient Query Design:**
   - Write queries that retrieve only the necessary data. Use selective conditions and avoid `SELECT *`. 
   - Use indexes to speed up data retrieval when querying large datasets. Analyze query performance and adjust indexes as necessary.

### 4. **Batch Processing:**
   - For operations involving multiple records, use batch insert/update/delete statements. This reduces the number of database calls and optimizes the use of network resources.

### 5. **Error Handling:**
   - Implement robust error handling to manage exceptions gracefully. This includes rollback mechanisms for transactions to maintain data integrity.

### 6. **Memory Management:**
   - In environments where you manage memory directly (like C/C++), ensure to handle memory allocation and deallocation properly. Use smart pointers or RAII patterns where available.
   - In garbage-collected languages, be mindful of memory leaks by ensuring that unnecessary references are cleared.

### 7. **Normalize and Denormalize Wisely:**
   - Normalize your database to eliminate redundancy but keep in mind that overly complex joins can degrade performance. Sometimes, denormalization (where redundancy is reintroduced) can improve read performance for specific use cases.

### 8. **Use NoSQL for Certain Use Cases:**
   - In situations where data models are flexible or require high write throughput, consider using NoSQL databases. These are designed to handle large volumes of unstructured data efficiently.

### 9. **Asynchronous I/O:**
   - Use asynchronous operations for database calls to avoid blocking your application, especially if using a high-performance web framework or any application that needs to remain responsive.

### 10. **Caching Strategies:**
   - Implement caching to reduce database load. Use an in-memory store (like Redis or Memcached) to cache frequently accessed data.

### 11. **Testing and Profiling:**
   - Regularly test and profile your database queries and interactions. Use tools to monitor performance, identify slow queries, and analyze execution plans.

### 12. **Follow Best Practices for Transactions:**
   - When performing operations that require consistency across multiple operations, wrap them in transactions. Properly manage transaction isolation levels depending on your application requirements.

### 13. **Use ORM with Caution:**
   - If using an Object-Relational Mapping (ORM) tool, understand how it generates queries and manages connections. Make sure it suits your performance needs and data access patterns.

By adhering to these practices and regularly reviewing your database access patterns, you can write memory-safe, high-performance database code that effectively manages resources while maintaining security and data integrity.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.207s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 3448 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.192s
    • Function auto-cleaned up by client
✓ New conversation stored and indexed for future retrieval

=== Step 6: Cross-Conversation Search ===
Searching for messages about 'ownership' across ALL conversations...


→ Executing TextSearch()...
  • Collection: rag_messages
  • Query: "ownership system"
  • Limit: 3 results
  • Search method: Full-text with fuzzy matching & stemming
  • No vector embeddings needed - pure keyword search
  ✓ Text search completed in 0.050s

✓ Found 3 messages mentioning ownership:
  1. From conv_rust_programming: Rust's key features include: memory safety without garbage collection, zero-cost abstractions, ownership system, powerful type system, and excellent concurrency support.

  2. From conv_performance: Rust's ownership system provides zero-cost memory management. Use Box for heap allocation, Rc/Arc for shared ownership, and avoid cloning large data structures. The compiler optimizes away unnecessary allocations.

  3. From conv_rust_programming: The borrow checker enforces Rust's ownership rules at compile time. It ensures that references don't outlive the data they point to and prevents data races by allowing either multiple immutable references or one mutable reference.

=== System Statistics ===

→ Querying database statistics...
  • Using FindAll() helper - simplified query API

📊 Database Statistics:
  • Total conversations: 4
  • Total messages stored: 14
  • All messages indexed for vector search ✓
  • All messages indexed for text search ✓
  • All messages queryable by metadata ✓

=== Step 8: Dynamic Search Configuration ===
Each conversation can have its own search config...

💡 Conversations can store custom search configurations:
  • Search type: hybrid, text, or vector
  • Relevance thresholds
  • Filter by tags or metadata
  • Collection-specific settings
  • Per-conversation AI behavior

This enables context-aware search tuned to each conversation's needs!

=== Cleanup ===
✓ Cleanup complete


=== 📚 Summary: What This Example Showed ===

🔧 ekoDB Native Capabilities Used:
  ✓ Functions with Embed operation (AI integration)
  ✓ Hybrid Search (text + vector combined)
  ✓ Text Search (full-text with stemming)
  ✓ Automatic embedding generation
  ✓ Cross-collection queries

🚀 New Client Helper Methods:
  • client.Embed(text, model) - Generate embeddings
  • client.HybridSearch() - Semantic + keyword search
  • client.TextSearch() - Full-text search
  • client.FindAll() - Query all documents

💡 Key Takeaways:
  1. ekoDB handles AI Functions natively - no external services needed
  2. One-line embedding generation with auto-cleanup
  3. Hybrid search combines semantic understanding + keyword matching
  4. Perfect for RAG: store, search, and retrieve context
  5. All AI capabilities accessible through simple client methods

🎯 Build production RAG systems with ekoDB!
   → Set OPENAI_API_KEY in your ekoDB server environment
   → Use these client helpers to make AI integration simple
   → Scale to millions of documents with native indexing


[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning Kotlin RAG Example...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
=== ekoDB RAG Conversation System ===

This example shows how ekoDB can power a self-improving AI system
that learns from its own conversation history.

SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== Step 1: Building Conversation History ===
Storing previous conversations with embeddings...

  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 34 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.21s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.29s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.24s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.199s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.214s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.224s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.286s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.233s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.195s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.226s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.215s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.184s
    • Function auto-cleaned up by client
✓ Stored performance optimization conversation (4 messages)

=== Step 2: New User Question with Context Retrieval ===
User asks: "How do I write memory-safe high-performance database code?"

=== Step 3: Searching Related Context ===
Using hybrid search to find relevant messages from all conversations...


→ Generating embedding for user question...
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 1.567s
    • Function auto-cleaned up by client

→ Executing hybridSearch()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.082s

✓ Found 5 related messages across all conversations:
  1. [Score: 0.199] From conv_database_design
     Database normalization is the process of organizing data to reduce redundancy and improve data integrity. It involves dividing large tables into smaller ones and defining relationships between them using foreign keys.

  2. [Score: 0.180] From conv_database_design
     Use NoSQL when you need: flexible schemas, horizontal scaling, high write throughput, or when working with unstructured data. SQL is better for complex queries, ACID transactions, and structured data with well-defined relationships.

  3. [Score: 0.180] From conv_database_design
     What is database normalization?

  4. [Score: 0.154] From conv_database_design
     When should I use NoSQL over SQL?

  5. [Score: 0.112] From conv_performance
     How can I optimize database queries?

=== Step 4: Generating Context-Aware Response ===
✓ Context prepared from search results
✓ AI would use this context to generate comprehensive response

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.242s
    • Function auto-cleaned up by client
✓ New conversation stored and indexed for future retrieval

=== Step 6: Cross-Conversation Search ===
Searching for messages about 'ownership' across ALL conversations...


→ Executing textSearch()...
  • Collection: rag_messages
  • Query: "ownership system"
  • Limit: 3 results
  • Search method: Full-text with fuzzy matching & stemming
  • No vector embeddings needed - pure keyword search
  ✓ Text search completed in 0.034s

✓ Found 3 messages mentioning ownership:
  1. From conv_rust_programming: Rust's key features include: memory safety without garbage collection, zero-cost abstractions, ownership system, powerful type system, and excellent concurrency support.

  2. From conv_performance: Rust's ownership system provides zero-cost memory management. Use Box for heap allocation, Rc/Arc for shared ownership, and avoid cloning large data structures. The compiler optimizes away unnecessary allocations.

  3. From conv_rust_programming: The borrow checker enforces Rust's ownership rules at compile time. It ensures that references don't outlive the data they point to and prevents data races by allowing either multiple immutable references or one mutable reference.

=== System Statistics ===

→ Querying database statistics...
  • Using findAllWithLimit() helper - simplified query API

📊 Database Statistics:
  • Total conversations: 4
  • Total messages stored: 13
  • All messages indexed for vector search ✓
  • All messages indexed for text search ✓
  • All messages queryable by metadata ✓

=== Step 8: Dynamic Search Configuration ===
Each conversation can have its own search config...

💡 Conversations can store custom search configurations:
  • Search type: hybrid, text, or vector
  • Relevance thresholds
  • Filter by tags or metadata
  • Collection-specific settings
  • Per-conversation AI behavior

This enables context-aware search tuned to each conversation's needs!

=== Cleanup ===
✓ Cleanup complete


=== 📚 Summary: What This Example Showed ===

🔧 ekoDB Native Capabilities Used:
  ✓ Functions with Embed operation (AI integration)
  ✓ Hybrid Search (text + vector combined)
  ✓ Text Search (full-text with stemming)
  ✓ Automatic embedding generation
  ✓ Cross-collection queries

🚀 New Client Helper Methods:
  • client.embed(text, model) - Generate embeddings
  • client.hybridSearch() - Semantic + keyword search
  • client.textSearch() - Full-text search
  • client.findAllWithLimit() - Query all documents

💡 Key Takeaways:
  1. ekoDB handles AI Functions natively - no external services needed
  2. One-line embedding generation with auto-cleanup
  3. Hybrid search combines semantic understanding + keyword matching
  4. Perfect for RAG: store, search, and retrieve context
  5. All AI capabilities accessible through simple client methods

🎯 Build production RAG systems with ekoDB!
   → Set OPENAI_API_KEY in your ekoDB server environment
   → Use these client helpers to make AI integration simple
   → Scale to millions of documents with native indexing


[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
✅ [32mRAG Examples Complete![0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m

[32mWhat you just saw across 5 languages:[0m
  ✓ Embeddings generated via ekoDB Functions
  ✓ Hybrid search (semantic + keyword)
  ✓ Text search with stemming
  ✓ Cross-conversation context retrieval
  ✓ Simple client helpers wrapping powerful AI

[36mMission: AI for All 🚀[0m - Making RAG accessible to everyone!

✅ [32mAll RAG examples complete! Output saved to test-examples-rag.md[0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
🌐 [36mTesting SWR (Stale-While-Revalidate) Pattern Examples[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m

📦 [36mBuilding TypeScript client library...[0m
npm warn EBADENGINE Unsupported engine {
npm warn EBADENGINE   package: 'vitest@4.1.8',
npm warn EBADENGINE   required: { node: '^20.0.0 || ^22.0.0 || >=24.0.0' },
npm warn EBADENGINE   current: { node: 'v23.6.0', npm: '10.9.2' }
npm warn EBADENGINE }

> @ekodb/ekodb-client@0.21.0 prepare
> npm run build


> @ekodb/ekodb-client@0.21.0 build
> tsc


up to date, audited 52 packages in 1s

17 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.21.0 build
> tsc

✅ [32mTypeScript client built![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning TypeScript SWR Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
Function 'fetch_api_user' already existed — updated instead
✓ Created SWR script: fetch_api_user (J9vIT3dUY3yte9COwkBZmCXaJSpneikYoqZUbZdqvE0wiQD4AfCE2jzH7wowCgsDfnH41uS-1b8fbc1vncuKvA)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "website": "hildegard.org",
          "company": {
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net"
          },
          "address": {
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "city": "Gwenborough",
            "street": "Kulas Light",
            "suite": "Apt. 556",
            "zipcode": "92998-3874"
          },
          "email": "Sincere@april.biz",
          "id": 1,
          "username": "Bret",
          "phone": "1-770-736-8031 x56442",
          "name": "Leanne Graham"
        }
      }
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
Response time: 41ms (served from cache)
Result (cached): {
  "records": [
    {
      "value": {
        "type": "Object",
        "value": {
          "website": "hildegard.org",
          "company": {
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net"
          },
          "address": {
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "city": "Gwenborough",
            "street": "Kulas Light",
            "suite": "Apt. 556",
            "zipcode": "92998-3874"
          },
          "email": "Sincere@april.biz",
          "id": 1,
          "username": "Bret",
          "phone": "1-770-736-8031 x56442",
          "name": "Leanne Graham"
        }
      }
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
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
Function 'fetch_product_with_reviews' already existed — updated instead
✓ Created enrichment script: fetch_product_with_reviews (YCYBboEoGLq6ioWdzc_mrGazpfx-gwovGPUss-dJWN1tHMp4X6eTinqXZSFe_d-iAGrUep1lfeNfEhNyZbk5PA)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {
      "value": {
        "value": {
          "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
          "rating": 2.56,
          "discountPercentage": 10.48,
          "title": "Essence Mascara Lash Princess",
          "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
          "warrantyInformation": "1 week warranty",
          "minimumOrderQuantity": 48,
          "reviews": [
            {
              "reviewerEmail": "eleanor.collins@x.dummyjson.com",
              "comment": "Would not recommend!",
              "rating": 3,
              "date": "2025-04-30T09:41:02.053Z",
              "reviewerName": "Eleanor Collins"
            },
            {
              "reviewerEmail": "lucas.gordon@x.dummyjson.com",
              "comment": "Very satisfied!",
              "rating": 4,
              "date": "2025-04-30T09:41:02.053Z",
              "reviewerName": "Lucas Gordon"
            },
            {
              "reviewerName": "Eleanor Collins",
              "comment": "Highly impressed!",
              "date": "2025-04-30T09:41:02.053Z",
              "rating": 5,
              "reviewerEmail": "eleanor.collins@x.dummyjson.com"
            }
          ],
          "id": 1,
          "stock": 99,
          "brand": "Essence",
          "sku": "BEA-ESS-ESS-001",
          "weight": 4,
          "returnPolicy": "No return policy",
          "category": "beauty",
          "availabilityStatus": "In Stock",
          "dimensions": {
            "depth": 22.99,
            "width": 15.14,
            "height": 13.08
          },
          "meta": {
            "qrCode": "https://cdn.dummyjson.com/public/qr-code.png",
            "barcode": "5784719087687",
            "updatedAt": "2025-04-30T09:41:02.053Z",
            "createdAt": "2025-04-30T09:41:02.053Z"
          },
          "tags": [
            "beauty",
            "mascara"
          ],
          "price": 9.99,
          "images": [
            "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
          ],
          "shippingInformation": "Ships in 3-5 business days"
        },
        "type": "Object"
      }
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
✓ Multi-API data fetched, merged, and cached atomically

=== Why This Is Powerful ===

✓ No separate cache layer (Redis, Memcached) needed
✓ No manual cache invalidation (TTL handles it)
✓ No separate edge infrastructure (ekoDB IS the edge)
✓ Atomic operations (function executes as transaction)
✓ With multi-node + ripples: Auto-sync across all nodes
✓ Sub-millisecond cache hits from internal storage
✓ One service instead of many (cache + API gateway + database)

=== Real-World Use Cases ===

1. API Gateway Pattern:
   - Client → ekoDB Function → Check cache → Call microservices → Merge → Cache

2. Database Federation:
   - Query multiple DBs (Postgres, MongoDB) + external APIs
   - Merge results in one function call

3. IoT Data Enrichment:
   - Sensor data + weather API + location API
   - Enrich and cache in one atomic operation

4. E-commerce Product Pages:
   - Product info + reviews + inventory + pricing
   - All from different sources, cached together

✓ Example complete - Your database IS your edge!

=== ekoDB as Edge Cache - Simple Example ===

Creating edge cache function...
ℹ️  Function 'cache_api_call' already existed — updated instead
✓ Edge cache script updated: 5bF9XG9fzZIUGDYd2SYpoKJDocI5lXE3GcsHqrNB7rZy2QVDuPi0JpucsNU7AtPiDCwnICDOyJlFfw0l71n9_A

Call 1: Cache miss (fetches from API)
Response time: 154ms
Result: {
  "records": [
    {
      "value": {
        "value": {
          "username": "Bret",
          "address": {
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            },
            "city": "Gwenborough",
            "street": "Kulas Light",
            "suite": "Apt. 556",
            "zipcode": "92998-3874"
          },
          "website": "hildegard.org",
          "id": 1,
          "email": "Sincere@april.biz",
          "company": {
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona"
          },
          "phone": "1-770-736-8031 x56442",
          "name": "Leanne Graham"
        },
        "type": "Object"
      }
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

Call 2: Cache hit (served from ekoDB)
Response time: 41ms (3.8x faster!)
Result: {
  "records": [
    {
      "value": {
        "value": {
          "username": "Bret",
          "address": {
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            },
            "city": "Gwenborough",
            "street": "Kulas Light",
            "suite": "Apt. 556",
            "zipcode": "92998-3874"
          },
          "website": "hildegard.org",
          "id": 1,
          "email": "Sincere@april.biz",
          "company": {
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona"
          },
          "phone": "1-770-736-8031 x56442",
          "name": "Leanne Graham"
        },
        "type": "Object"
      }
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

=== The Magic ===
- Your DATABASE is your EDGE
- No Redis needed
- No CDN needed
- No cache invalidation logic needed (TTL handles it)
- With ripples: All nodes auto-sync cache
- One service: Database + Cache + Edge Functions

✓ Example complete!

✅ [32mTypeScript SWR examples complete![0m
🐍 [36mBuilding Python client package...[0m
🔧 [36mEnsuring maturin is available in .venv...[0m
🔨 [36mBuilding wheel...[0m
🍹 Building a mixed python/rust project
🐍 Found CPython 3.14 at /opt/homebrew/opt/python@3.14/bin/python3.14
🔗 Found pyo3 bindings with abi3 support
💻 Using `MACOSX_DEPLOYMENT_TARGET=11.0` for aarch64-apple-darwin by default
    Finished `release` profile [optimized] target(s) in 0.10s
📦 Built wheel for abi3 Python ≥ 3.8 to ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.21.0-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel into .venv...[0m
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.21.0-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.21.0
    Uninstalling ekodb_client-0.21.0:
      Successfully uninstalled ekodb_client-0.21.0
Successfully installed ekodb-client-0.21.0
✅ [32mPython client package built and installed![0m
📦 [36mEnsuring Python example dependencies in .venv...[0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning Python SWR Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_github_user (AK2_MzGwExsApi7rxgabjWYTlccVAxFJIorQyyQYSpALTTWVGphSrpqN-rDbqFVGasRfj6EgUNFRMix80mWkXg)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 172ms
Result: [
  {
    "cached_at": {
      "type": "String",
      "value": "1781056048"
    },
    "data": {
      "type": "Object",
      "value": {
        "avatar_url": "https://avatars.githubusercontent.com/u/1024025?v=4",
        "bio": null,
        "blog": "",
        "company": "Linux Foundation",
        "created_at": "2011-09-03T15:26:22Z",
        "email": null,
        "events_url": "https://api.github.com/users/torvalds/events{/privacy}",
        "followers": 306347,
        "followers_url": "https://api.github.com/users/torvalds/followers",
        "following": 0,
        "following_url": "https://api.github.com/users/torvalds/following{/other_user}",
        "gists_url": "https://api.github.com/users/torvalds/gists{/gist_id}",
        "gravatar_id": "",
        "hireable": null,
        "html_url": "https://github.com/torvalds",
        "id": 1024025,
        "location": "Portland, OR",
        "login": "torvalds",
        "name": "Linus Torvalds",
        "node_id": "MDQ6VXNlcjEwMjQwMjU=",
        "organizations_url": "https://api.github.com/users/torvalds/orgs",
        "public_gists": 1,
        "public_repos": 12,
        "received_events_url": "https://api.github.com/users/torvalds/received_events",
        "repos_url": "https://api.github.com/users/torvalds/repos",
        "site_admin": false,
        "starred_url": "https://api.github.com/users/torvalds/starred{/owner}{/repo}",
        "subscriptions_url": "https://api.github.com/users/torvalds/subscriptions",
        "twitter_username": null,
        "type": "User",
        "updated_at": "2026-06-02T05:33:59Z",
        "url": "https://api.github.com/users/torvalds",
        "user_view_type": "public"
      }
    },
    "id": "torvalds"
  }
]
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 47ms (3.7x faster!)
Result: [
  {
    "cached_at": {
      "type": "String",
      "value": "1781056048"
    },
    "data": {
      "type": "Object",
      "value": {
        "avatar_url": "https://avatars.githubusercontent.com/u/1024025?v=4",
        "bio": null,
        "blog": "",
        "company": "Linux Foundation",
        "created_at": "2011-09-03T15:26:22Z",
        "email": null,
        "events_url": "https://api.github.com/users/torvalds/events{/privacy}",
        "followers": 306347,
        "followers_url": "https://api.github.com/users/torvalds/followers",
        "following": 0,
        "following_url": "https://api.github.com/users/torvalds/following{/other_user}",
        "gists_url": "https://api.github.com/users/torvalds/gists{/gist_id}",
        "gravatar_id": "",
        "hireable": null,
        "html_url": "https://github.com/torvalds",
        "id": 1024025,
        "location": "Portland, OR",
        "login": "torvalds",
        "name": "Linus Torvalds",
        "node_id": "MDQ6VXNlcjEwMjQwMjU=",
        "organizations_url": "https://api.github.com/users/torvalds/orgs",
        "public_gists": 1,
        "public_repos": 12,
        "received_events_url": "https://api.github.com/users/torvalds/received_events",
        "repos_url": "https://api.github.com/users/torvalds/repos",
        "site_admin": false,
        "starred_url": "https://api.github.com/users/torvalds/starred{/owner}{/repo}",
        "subscriptions_url": "https://api.github.com/users/torvalds/subscriptions",
        "twitter_username": null,
        "type": "User",
        "updated_at": "2026-06-02T05:33:59Z",
        "url": "https://api.github.com/users/torvalds",
        "user_view_type": "public"
      }
    }
  }
]
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (OjWak825ellEjfRleEs9P3ZjQDVp27GG00n1DrJMhuiwdLW8C0qsn8PQCm9HWVe3hIvOdUOwKioJWBJZnGA35w)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: [
  {
    "enriched_at": {
      "type": "String",
      "value": "1781056048"
    },
    "enriched_data": {
      "type": "Object",
      "value": {
        "availabilityStatus": "In Stock",
        "brand": "Essence",
        "category": "beauty",
        "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
        "dimensions": {
          "depth": 22.99,
          "height": 13.08,
          "width": 15.14
        },
        "discountPercentage": 10.48,
        "id": 1,
        "images": [
          "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
        ],
        "meta": {
          "barcode": "5784719087687",
          "createdAt": "2025-04-30T09:41:02.053Z",
          "qrCode": "https://cdn.dummyjson.com/public/qr-code.png",
          "updatedAt": "2025-04-30T09:41:02.053Z"
        },
        "minimumOrderQuantity": 48,
        "price": 9.99,
        "rating": 2.56,
        "returnPolicy": "No return policy",
        "reviews": [
          {
            "comment": "Would not recommend!",
            "date": "2025-04-30T09:41:02.053Z",
            "rating": 3,
            "reviewerEmail": "eleanor.collins@x.dummyjson.com",
            "reviewerName": "Eleanor Collins"
          },
          {
            "comment": "Very satisfied!",
            "date": "2025-04-30T09:41:02.053Z",
            "rating": 4,
            "reviewerEmail": "lucas.gordon@x.dummyjson.com",
            "reviewerName": "Lucas Gordon"
          },
          {
            "comment": "Highly impressed!",
            "date": "2025-04-30T09:41:02.053Z",
            "rating": 5,
            "reviewerEmail": "eleanor.collins@x.dummyjson.com",
            "reviewerName": "Eleanor Collins"
          }
        ],
        "shippingInformation": "Ships in 3-5 business days",
        "sku": "BEA-ESS-ESS-001",
        "stock": 99,
        "tags": [
          "beauty",
          "mascara"
        ],
        "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
        "title": "Essence Mascara Lash Princess",
        "warrantyInformation": "1 week warranty",
        "weight": 4
      }
    },
    "id": "1"
  }
]
✓ Data fetched, enriched, and cached atomically

=== Why This Is Powerful ===
✓ No separate cache layer (Redis, Memcached) needed
✓ No manual cache invalidation (TTL handles it)
✓ No separate edge infrastructure (ekoDB IS the edge)
✓ Atomic operations (function executes as transaction)
✓ With multi-node + ripples: Auto-sync across all nodes
✓ Sub-millisecond cache hits from internal storage
✓ One service instead of many (cache + API gateway + database)

=== Real-World Use Cases ===
1. API Gateway Pattern:
   - Client → ekoDB Function → Check cache → Call microservices → Merge → Cache

2. Database Federation:
   - Query multiple DBs (Postgres, MongoDB) + external APIs
   - Merge results in one function call

3. IoT Data Enrichment:
   - Sensor data + weather API + location API
   - Enrich and cache in one atomic operation

4. E-commerce Product Pages:
   - Product info + reviews + inventory + pricing
   - All from different sources, cached together

✓ Example complete - Your database IS your edge!

✅ [32mPython SWR examples complete![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning Go SWR Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
Function 'fetch_github_user' already existed — updated instead
✓ Created SWR script: fetch_github_user (fetch_github_user)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 41.289791ms
Result: [
  {
    "cached_at": {
      "type": "String",
      "value": "1781056048"
    },
    "data": {
      "type": "Object",
      "value": {
        "avatar_url": "https://avatars.githubusercontent.com/u/1024025?v=4",
        "bio": null,
        "blog": "",
        "company": "Linux Foundation",
        "created_at": "2011-09-03T15:26:22Z",
        "email": null,
        "events_url": "https://api.github.com/users/torvalds/events{/privacy}",
        "followers": 306347,
        "followers_url": "https://api.github.com/users/torvalds/followers",
        "following": 0,
        "following_url": "https://api.github.com/users/torvalds/following{/other_user}",
        "gists_url": "https://api.github.com/users/torvalds/gists{/gist_id}",
        "gravatar_id": "",
        "hireable": null,
        "html_url": "https://github.com/torvalds",
        "id": 1024025,
        "location": "Portland, OR",
        "login": "torvalds",
        "name": "Linus Torvalds",
        "node_id": "MDQ6VXNlcjEwMjQwMjU=",
        "organizations_url": "https://api.github.com/users/torvalds/orgs",
        "public_gists": 1,
        "public_repos": 12,
        "received_events_url": "https://api.github.com/users/torvalds/received_events",
        "repos_url": "https://api.github.com/users/torvalds/repos",
        "site_admin": false,
        "starred_url": "https://api.github.com/users/torvalds/starred{/owner}{/repo}",
        "subscriptions_url": "https://api.github.com/users/torvalds/subscriptions",
        "twitter_username": null,
        "type": "User",
        "updated_at": "2026-06-02T05:33:59Z",
        "url": "https://api.github.com/users/torvalds",
        "user_view_type": "public"
      }
    }
  }
]
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 40.471375ms (1.0x faster!)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
Function 'fetch_product_enriched' already existed — updated instead
✓ Created enrichment script: fetch_product_enriched (fetch_product_enriched)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: [
  {
    "enriched_data": {
      "type": "Object",
      "value": {
        "availabilityStatus": "In Stock",
        "brand": "Essence",
        "category": "beauty",
        "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
        "dimensions": {
          "depth": 22.99,
          "height": 13.08,
          "width": 15.14
        },
        "discountPercentage": 10.48,
        "id": 1,
        "images": [
          "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
        ],
        "meta": {
          "barcode": "5784719087687",
          "createdAt": "2025-04-30T09:41:02.053Z",
          "qrCode": "https://cdn.dummyjson.com/public/qr-code.png",
          "updatedAt": "2025-04-30T09:41:02.053Z"
        },
        "minimumOrderQuantity": 48,
        "price": 9.99,
        "rating": 2.56,
        "returnPolicy": "No return policy",
        "reviews": [
          {
            "comment": "Would not recommend!",
            "date": "2025-04-30T09:41:02.053Z",
            "rating": 3,
            "reviewerEmail": "eleanor.collins@x.dummyjson.com",
            "reviewerName": "Eleanor Collins"
          },
          {
            "comment": "Very satisfied!",
            "date": "2025-04-30T09:41:02.053Z",
            "rating": 4,
            "reviewerEmail": "lucas.gordon@x.dummyjson.com",
            "reviewerName": "Lucas Gordon"
          },
          {
            "comment": "Highly impressed!",
            "date": "2025-04-30T09:41:02.053Z",
            "rating": 5,
            "reviewerEmail": "eleanor.collins@x.dummyjson.com",
            "reviewerName": "Eleanor Collins"
          }
        ],
        "shippingInformation": "Ships in 3-5 business days",
        "sku": "BEA-ESS-ESS-001",
        "stock": 99,
        "tags": [
          "beauty",
          "mascara"
        ],
        "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
        "title": "Essence Mascara Lash Princess",
        "warrantyInformation": "1 week warranty",
        "weight": 4
      }
    }
  }
]
✓ Data fetched, enriched, and cached atomically

=== Why This Is Powerful ===
✓ No separate cache layer (Redis, Memcached) needed
✓ No manual cache invalidation (TTL handles it)
✓ No separate edge infrastructure (ekoDB IS the edge)
✓ Atomic operations (function executes as transaction)
✓ With multi-node + ripples: Auto-sync across all nodes
✓ Sub-millisecond cache hits from internal storage
✓ One service instead of many (cache + API gateway + database)

=== Real-World Use Cases ===
1. API Gateway Pattern:
   - Client → ekoDB Function → Check cache → Call microservices → Merge → Cache

2. Database Federation:
   - Query multiple DBs (Postgres, MongoDB) + external APIs
   - Merge results in one function call

3. IoT Data Enrichment:
   - Sensor data + weather API + location API
   - Enrich and cache in one atomic operation

4. E-commerce Product Pages:
   - Product info + reviews + inventory + pricing
   - All from different sources, cached together

✓ Example complete - Your database IS your edge!

✅ [32mGo SWR examples complete![0m
🛠️  [36mBuilding client library...[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.17s
✅ [32mClient build complete![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning Rust SWR Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/swr_pattern`
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
ℹ️  Function 'fetch_github_user' already existed — updated instead
✓ Created SWR script: fetch_github_user (yhQLasmCOCKoL_WxbxPdsBWQ5j6pX1p4wyP-JCNrIEsyCPlJb2w4dwOhEHBqaJ58_i3Jj8ntFDbYyVQKwiLS-Q)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 40ms
Result: {
  "data": {
    "value": {
      "events_url": "https://api.github.com/users/torvalds/events{/privacy}",
      "gravatar_id": "",
      "user_view_type": "public",
      "subscriptions_url": "https://api.github.com/users/torvalds/subscriptions",
      "updated_at": "2026-06-02T05:33:59Z",
      "created_at": "2011-09-03T15:26:22Z",
      "repos_url": "https://api.github.com/users/torvalds/repos",
      "html_url": "https://github.com/torvalds",
      "email": null,
      "twitter_username": null,
      "node_id": "MDQ6VXNlcjEwMjQwMjU=",
      "blog": "",
      "bio": null,
      "public_gists": 1,
      "hireable": null,
      "location": "Portland, OR",
      "organizations_url": "https://api.github.com/users/torvalds/orgs",
      "avatar_url": "https://avatars.githubusercontent.com/u/1024025?v=4",
      "received_events_url": "https://api.github.com/users/torvalds/received_events",
      "starred_url": "https://api.github.com/users/torvalds/starred{/owner}{/repo}",
      "gists_url": "https://api.github.com/users/torvalds/gists{/gist_id}",
      "following": 0,
      "following_url": "https://api.github.com/users/torvalds/following{/other_user}",
      "name": "Linus Torvalds",
      "followers": 306347,
      "id": 1024025,
      "public_repos": 12,
      "type": "User",
      "site_admin": false,
      "followers_url": "https://api.github.com/users/torvalds/followers",
      "login": "torvalds",
      "url": "https://api.github.com/users/torvalds",
      "company": "Linux Foundation"
    },
    "type": "Object"
  }
}
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 41ms (1.0x faster!)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
ℹ️  Function 'fetch_product_enriched' already existed — updated instead
✓ Created enrichment script: fetch_product_enriched (X4sSo7--6ksCCtUuBTevvBO6EGx0KP6acp5MIULGXNXh1qCo1yhjJWSRuZaMP_c0r1MI5fA2ohxdDZCta7sdyw)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: {
  "enriched_data": {
    "type": "Object",
    "value": {
      "brand": "Essence",
      "sku": "BEA-ESS-ESS-001",
      "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
      "price": 9.99,
      "rating": 2.56,
      "images": [
        "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
      ],
      "discountPercentage": 10.48,
      "availabilityStatus": "In Stock",
      "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
      "meta": {
        "qrCode": "https://cdn.dummyjson.com/public/qr-code.png",
        "barcode": "5784719087687",
        "createdAt": "2025-04-30T09:41:02.053Z",
        "updatedAt": "2025-04-30T09:41:02.053Z"
      },
      "tags": [
        "beauty",
        "mascara"
      ],
      "minimumOrderQuantity": 48,
      "warrantyInformation": "1 week warranty",
      "category": "beauty",
      "reviews": [
        {
          "reviewerName": "Eleanor Collins",
          "reviewerEmail": "eleanor.collins@x.dummyjson.com",
          "comment": "Would not recommend!",
          "date": "2025-04-30T09:41:02.053Z",
          "rating": 3
        },
        {
          "rating": 4,
          "comment": "Very satisfied!",
          "reviewerName": "Lucas Gordon",
          "date": "2025-04-30T09:41:02.053Z",
          "reviewerEmail": "lucas.gordon@x.dummyjson.com"
        },
        {
          "reviewerEmail": "eleanor.collins@x.dummyjson.com",
          "comment": "Highly impressed!",
          "reviewerName": "Eleanor Collins",
          "rating": 5,
          "date": "2025-04-30T09:41:02.053Z"
        }
      ],
      "stock": 99,
      "title": "Essence Mascara Lash Princess",
      "weight": 4,
      "returnPolicy": "No return policy",
      "id": 1,
      "dimensions": {
        "height": 13.08,
        "depth": 22.99,
        "width": 15.14
      },
      "shippingInformation": "Ships in 3-5 business days"
    }
  }
}
✓ Data fetched, enriched, and cached atomically

=== Why This Is Powerful ===
✓ No separate cache layer (Redis, Memcached) needed
✓ No manual cache invalidation (TTL handles it)
✓ No separate edge infrastructure (ekoDB IS the edge)
✓ Atomic operations (function executes as transaction)
✓ With multi-node + ripples: Auto-sync across all nodes
✓ Sub-millisecond cache hits from internal storage
✓ One service instead of many (cache + API gateway + database)

=== Real-World Use Cases ===
1. API Gateway Pattern:
   - Client → ekoDB Function → Check cache → Call microservices → Merge → Cache

2. Database Federation:
   - Query multiple DBs (Postgres, MongoDB) + external APIs
   - Merge results in one function call

3. IoT Data Enrichment:
   - Sensor data + weather API + location API
   - Enrich and cache in one atomic operation

4. E-commerce Product Pages:
   - Product info + reviews + inventory + pricing
   - All from different sources, cached together

✓ Example complete - Your database IS your edge!

✅ [32mRust SWR examples complete![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning Kotlin SWR Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
ℹ️  Function 'fetch_github_user' already existed — updated instead
✓ Created SWR function: fetch_github_user (5x-XVJNs0tIAubFI3DsJHw9GMri8K6U6GcuEsa7SeFCogrwU7aYhLC3E5C_Jzmt9fpe7uffMwJB8dqzfmIDBTQ)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 62ms
Result: [{"cached_at":{"value":"1781056048","type":"String"},"data":{"value":{"html_url":"https://github.com/torvalds","followers":306347,"gists_url":"https://api.github.com/users/torvalds/gists{/gist_id}","following_url":"https://api.github.com/users/torvalds/following{/other_user}","avatar_url":"https://avatars.githubusercontent.com/u/1024025?v=4","updated_at":"2026-06-02T05:33:59Z","public_gists":1,"twitter_username":null,"following":0,"hireable":null,"received_events_url":"https://api.github.com/users/torvalds/received_events","repos_url":"https://api.github.com/users/torvalds/repos","gravatar_id":"","starred_url":"https://api.github.com/users/torvalds/starred{/owner}{/repo}","location":"Portland, OR","type":"User","blog":"","bio":null,"company":"Linux Foundation","followers_url":"https://api.github.com/users/torvalds/followers","node_id":"MDQ6VXNlcjEwMjQwMjU=","created_at":"2011-09-03T15:26:22Z","site_admin":false,"user_view_type":"public","events_url":"https://api.github.com/users/torvalds/events{/privacy}","login":"torvalds","organizations_url":"https://api.github.com/users/torvalds/orgs","url":"https://api.github.com/users/torvalds","id":1024025,"name":"Linus Torvalds","email":null,"subscriptions_url":"https://api.github.com/users/torvalds/subscriptions","public_repos":12},"type":"Object"}}]
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 45ms
Cache hit was 1.4x faster!

✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
ℹ️  Function 'fetch_product_enriched' already existed — updated instead
✓ Created enrichment function: fetch_product_enriched (Bd87Wp1fKGPVVZjMUobQUu3jbAr-rHuxWCiv-pC_NiQH5PA8JuB2qrKaBWb03LmjctmPDAfsL-jplyug5TElaw)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: [{"enriched_data":{"type":"Object","value":{"meta":{"qrCode":"https://cdn.dummyjson.com/public/qr-code.png","createdAt":"2025-04-30T09:41:02.053Z","updatedAt":"2025-04-30T09:41:02.053Z","barcode":"5784719087687"},"availabilityStatus":"In Stock","rating":2.56,"returnPolicy":"No return policy","stock":99,"sku":"BEA-ESS-ESS-001","thumbnail":"https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp","reviews":[{"rating":3,"reviewerEmail":"eleanor.collins@x.dummyjson.com","reviewerName":"Eleanor Collins","comment":"Would not recommend!","date":"2025-04-30T09:41:02.053Z"},{"comment":"Very satisfied!","reviewerEmail":"lucas.gordon@x.dummyjson.com","rating":4,"reviewerName":"Lucas Gordon","date":"2025-04-30T09:41:02.053Z"},{"comment":"Highly impressed!","date":"2025-04-30T09:41:02.053Z","rating":5,"reviewerEmail":"eleanor.collins@x.dummyjson.com","reviewerName":"Eleanor Collins"}],"category":"beauty","tags":["beauty","mascara"],"dimensions":{"height":13.08,"width":15.14,"depth":22.99},"title":"Essence Mascara Lash Princess","warrantyInformation":"1 week warranty","description":"The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.","weight":4,"id":1,"images":["https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"],"shippingInformation":"Ships in 3-5 business days","minimumOrderQuantity":48,"price":9.99,"brand":"Essence","discountPercentage":10.48}}}]
✓ Data fetched, enriched, and cached atomically

=== Why This Is Powerful ===
✓ No separate cache layer (Redis, Memcached) needed
✓ No manual cache invalidation (TTL handles it)
✓ No separate edge infrastructure (ekoDB IS the edge)
✓ Atomic operations (function executes as transaction)
✓ With multi-node + ripples: Auto-sync across all nodes
✓ Sub-millisecond cache hits from internal storage
✓ One service instead of many (cache + API gateway + database)

=== Real-World Use Cases ===
1. API Gateway Pattern:
   - Client → ekoDB Function → Check cache → Call microservices → Merge → Cache

2. Database Federation:
   - Query multiple DBs (Postgres, MongoDB) + external APIs
   - Merge results in one function call

3. IoT Data Enrichment:
   - Sensor data + weather API + location API
   - Enrich and cache in one atomic operation

4. E-commerce Product Pages:
   - Product info + reviews + inventory + pricing
   - All from different sources, cached together

✓ Example complete - Your database IS your edge!


BUILD SUCCESSFUL in 10s
2 actionable tasks: 1 executed, 1 up-to-date
✅ [32mKotlin SWR examples complete![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
✅ [32mAll SWR Examples Complete![0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m

[32mWhat you just saw - ekoDB as Edge Cache:[0m
  ✓ FindById → Check cache
  ✓ If/Else → Conditional execution
  ✓ HttpRequest → External API calls
  ✓ Insert with TTL → Auto-expiring cache
  ✓ Sub-millisecond cache hits
  ✓ No Redis, no CDN, no cache invalidation logic needed

[36mYour DATABASE is your EDGE! 🚀[0m


[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
🔗 [36mTesting Function Composition Examples[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m

🛠️  [36mBuilding client library...[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
✅ [32mClient build complete![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning Rust Function Composition Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_function_composition`
=== ekoDB Function Composition Examples ===

📋 Setting up test data...

✅ Test data ready

📝 Example 1: Basic Function Composition

Building reusable functions that call each other...

ℹ️  Function 'fetch_user' already existed — updated instead
✅ Saved reusable function: fetch_user
✅ Saved composed function: get_user_wrapper (calls fetch_user + projects fields)

📊 Result from composed function:
   Records: 1
   Name: User 1
   Department: engineering

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

ℹ️  Function 'fetch_and_store_user' already existed — updated instead
✅ Saved reusable function: fetch_and_store_user (uses KV)
ℹ️  Function 'swr_user' already existed — updated instead
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 162.663792ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "phone": "1-770-736-8031 x56442",
      "email": "Sincere@april.biz",
      "name": "Leanne Graham",
      "address": {
        "city": "Gwenb...

Second call (cache hit - from cache):
   ⏱️  Duration: 41.731916ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "id": 1,
      "phone": "1-770-736-8031 x56442",
      "username": "Bret",
      "name": "Leanne Graham",
      "company": {
        "catchPhr...
   🚀 Cache speedup: 4.0x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

ℹ️  Function 'validate_user' already existed — updated instead
✅ Level 1 function: validate_user
ℹ️  Function 'fetch_slim_user' already existed — updated instead
✅ Level 2 function: fetch_slim_user (calls validate_user)
✅ Level 3 function: get_verified_user (calls fetch_slim_user)

📊 Result from 3-level nested composition:
   Records: 1
   Name: User 1
   Department: engineering

🎯 Key Benefit: Each function is independently testable and reusable!
   - validate_user: Used in 100 different workflows
   - fetch_enriched_user: Used in 50 workflows
   - get_engineering_user: Specific workflow


✅ All composition examples completed!
✅ [32mRust function composition examples complete![0m
📦 [36mBuilding TypeScript client library...[0m
npm warn EBADENGINE Unsupported engine {
npm warn EBADENGINE   package: 'vitest@4.1.8',
npm warn EBADENGINE   required: { node: '^20.0.0 || ^22.0.0 || >=24.0.0' },
npm warn EBADENGINE   current: { node: 'v23.6.0', npm: '10.9.2' }
npm warn EBADENGINE }

> @ekodb/ekodb-client@0.21.0 prepare
> npm run build


> @ekodb/ekodb-client@0.21.0 build
> tsc


up to date, audited 52 packages in 1s

17 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.21.0 build
> tsc

✅ [32mTypeScript client built![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning TypeScript Function Composition Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
=== ekoDB Function Composition Examples ===

📋 Setting up test data...

✅ Test data ready

📝 Example 1: Basic Function Composition

Building reusable functions that call each other...

Function 'fetch_user' already existed — updated instead
✅ Saved reusable function: fetch_user
Function 'get_user_wrapper' already existed — updated instead
✅ Saved composed function: get_user_wrapper (calls fetch_user + projects fields)

📊 Result from composed function:
   Records: 1
   Name: {"value":"User 1","type":"String"}
   Department: {"value":"engineering","type":"String"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

Function 'fetch_and_store_user' already existed — updated instead
✅ Saved reusable function: fetch_and_store_user (uses KV)
Function 'swr_user' already existed — updated instead
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 42ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "username": "Bret",
      "name": "Leanne Graham",
      "id": 1,
      "address": {
        "suite": "Apt. 556",
        "city": "Gwenborough",
        "zipcode": ...

Second call (cache hit - from cache):
   ⏱️  Duration: 43ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "username": "Bret",
      "name": "Leanne Graham",
      "id": 1,
      "address": {
        "suite": "Apt. 556",
        "city": "Gwenborough",
        "zipcode": ...
   🚀 Cache speedup: 1.0x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

Function 'validate_user' already existed — updated instead
✅ Level 1 function: validate_user
Function 'fetch_slim_user' already existed — updated instead
✅ Level 2 function: fetch_slim_user (calls validate_user)
Function 'get_verified_user' already existed — updated instead
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
✅ [32mTypeScript function composition examples complete![0m
🐍 [36mBuilding Python client package...[0m
🔧 [36mEnsuring maturin is available in .venv...[0m
🔨 [36mBuilding wheel...[0m
🍹 Building a mixed python/rust project
🐍 Found CPython 3.14 at /opt/homebrew/opt/python@3.14/bin/python3.14
🔗 Found pyo3 bindings with abi3 support
💻 Using `MACOSX_DEPLOYMENT_TARGET=11.0` for aarch64-apple-darwin by default
    Finished `release` profile [optimized] target(s) in 0.10s
📦 Built wheel for abi3 Python ≥ 3.8 to ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.21.0-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel into .venv...[0m
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.21.0-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.21.0
    Uninstalling ekodb_client-0.21.0:
      Successfully uninstalled ekodb_client-0.21.0
Successfully installed ekodb-client-0.21.0
✅ [32mPython client package built and installed![0m
📦 [36mEnsuring Python example dependencies in .venv...[0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning Python Function Composition Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
=== ekoDB Function Composition Examples ===

📋 Setting up test data...

✅ Test data ready

📝 Example 1: Basic Function Composition

Building reusable functions that call each other...

ℹ️  Function 'fetch_user' already existed — updated instead
✅ Saved reusable function: fetch_user
ℹ️  Function 'get_user_wrapper' already existed — updated instead
✅ Saved composed function: get_user_wrapper (calls fetch_user + projects fields)

📊 Result from composed function:
   Records: 1
   Name: {"type": "String", "value": "User 1"}
   Department: {"type": "String", "value": "engineering"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

ℹ️  Function 'fetch_and_store_user' already existed — updated instead
✅ Saved reusable function: fetch_and_store_user (uses KV)
ℹ️  Function 'swr_user' already existed — updated instead
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 42.1ms
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
   ⏱️  Duration: 40.3ms
   📊 Records: 1
   📦 Data: {
      "value": {
            "type": "Object",
            "value": {
                  "address": {
                        "city": "Gwenborough",
                        "geo": {
                 ...
   🚀 Cache speedup: 1.0x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

ℹ️  Function 'validate_user' already existed — updated instead
✅ Level 1 function: validate_user
ℹ️  Function 'fetch_slim_user' already existed — updated instead
✅ Level 2 function: fetch_slim_user (calls validate_user)
ℹ️  Function 'get_verified_user' already existed — updated instead
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
✅ [32mPython function composition examples complete![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning Go Function Composition Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
=== ekoDB Function Composition Examples ===

📋 Setting up test data...

✅ Test data ready

📝 Example 1: Basic Function Composition

Building reusable functions that call each other...

Function 'fetch_user' already existed — updated instead
✅ Saved reusable function: fetch_user
Function 'get_user_wrapper' already existed — updated instead
✅ Saved composed function: get_user_wrapper (calls fetch_user + projects fields)

📊 Result from composed function:
   Records: 1
   Name: {"type":"String","value":"User 1"}
   Department: {"type":"String","value":"engineering"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

Function 'fetch_and_store_user' already existed — updated instead
✅ Saved reusable function: fetch_and_store_user (uses KV)
Function 'swr_user' already existed — updated instead
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 42.287541ms
   📊 Records: 1
   📦 Data: {
        "value": {
          "type": "Object",
          "value": {
            "address": {
              "city": "Gwenborough",
              "geo": {
                "lat": "-37.3159",
          ...

Second call (cache hit - from cache):
   ⏱️  Duration: 41.885875ms
   📊 Records: 1
   📦 Data: {
        "value": {
          "type": "Object",
          "value": {
            "address": {
              "city": "Gwenborough",
              "geo": {
                "lat": "-37.3159",
          ...
   🚀 Cache speedup: 1.0x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

Function 'validate_user' already existed — updated instead
✅ Level 1 function: validate_user
Function 'fetch_slim_user' already existed — updated instead
✅ Level 2 function: fetch_slim_user (calls validate_user)
Function 'get_verified_user' already existed — updated instead
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
✅ [32mGo function composition examples complete![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning JavaScript Function Composition Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
=== ekoDB Function Composition Examples ===

📋 Setting up test data...

✅ Test data ready

📝 Example 1: Basic Function Composition

Building reusable functions that call each other...

ℹ️  Function 'fetch_user' already existed — updated instead
✅ Saved reusable function: fetch_user
ℹ️  Function 'get_user_wrapper' already existed — updated instead
✅ Saved composed function: get_user_wrapper (calls fetch_user + projects fields)

📊 Result from composed function:
   Records: 1
   Name: {"type":"String","value":"User 1"}
   Department: {"type":"String","value":"engineering"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

ℹ️  Function 'fetch_and_store_user' already existed — updated instead
✅ Saved reusable function: fetch_and_store_user (uses KV)
ℹ️  Function 'swr_user' already existed — updated instead
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 42ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "value": {
      "username": "Bret",
      "name": "Leanne Graham",
      "id": 1,
      "address": {
        "suite": "Apt. 556",
        "city": "Gwenborough",
        "zipcode": ...

Second call (cache hit - from cache):
   ⏱️  Duration: 42ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "type": "Object",
    "value": {
      "username": "Bret",
      "name": "Leanne Graham",
      "id": 1,
      "address": {
        "suite": "Apt. 556",
        "city": "Gwenborough...
   🚀 Cache speedup: 1.0x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

ℹ️  Function 'validate_user' already existed — updated instead
✅ Level 1 function: validate_user
ℹ️  Function 'fetch_slim_user' already existed — updated instead
✅ Level 2 function: fetch_slim_user (calls validate_user)
ℹ️  Function 'get_verified_user' already existed — updated instead
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
✅ [32mJavaScript function composition examples complete![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
✅ [32mAll Function Composition Examples Complete![0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m

[32mWhat you just saw - CallFunction composability:[0m
  ✓ Reusable Function building blocks
  ✓ Functions calling other Functions
  ✓ Clean SWR patterns via composition
  ✓ Multi-level nesting (arbitrary depth)
  ✓ No code duplication
  ✓ Single source of truth

[36mBuild complex workflows from simple pieces! 🚀[0m


[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
📡 [36mWebSocket Subscription Tests[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m

[33mThese tests verify real-time WebSocket subscriptions by:[0m
  1. Authenticating and connecting via WebSocket
  2. Subscribing to a collection
  3. Inserting records via REST to trigger notifications
  4. Verifying MutationNotification push messages arrive
  5. Unsubscribing and cleaning up

🛠️  [36mBuilding client library...[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
✅ [32mClient build complete![0m

🦀 [33mRust WebSocket Subscription Test...[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_websocket_subscribe`
✓ Authentication successful

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_rs' ===
✓ Subscribed (subscription_id: sub_8c3992f7d89f418e98b33f6dcffbdcfe)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: "KvnIczngg_hpcW1upQ7chSW89LlkjIKIDU0Ax_nO3g72FRlNHEPVAGeHwItUd2UvCVy5aUGOS1AKudlpg-IAig"

  📡 Notification received:
     Event:      "insert"
     Collection: "ws_subscribe_example_rs"
     Record IDs: ["KvnIczngg_hpcW1upQ7chSW89LlkjIKIDU0Ax_nO3g72FRlNHEPVAGeHwItUd2UvCVy5aUGOS1AKudlpg-IAig"]
     Timestamp:  "2026-06-10T01:48:12.499458+00:00"

Inserting record 2...
✓ Inserted: "9hp9pTbU5pFH5vwlMRksDzXDPS8-UohuWukxplftHzlNd9STlqxRq9FrcTSLc_txBNKtykZ84PrrxUX-Sjt6-g"

  📡 Notification received:
     Event:      "insert"
     Record IDs: ["9hp9pTbU5pFH5vwlMRksDzXDPS8-UohuWukxplftHzlNd9STlqxRq9FrcTSLc_txBNKtykZ84PrrxUX-Sjt6-g"]

=== Unsubscribing ===
✓ Unsubscribed

✓ WebSocket subscription example completed successfully
✅ [32mRust subscription test complete![0m

🔷 [33mGo WebSocket Subscription Test...[0m
=== WebSocket Subscription Example (Go) ===

✓ Authentication successful
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_go' ===
✓ Subscribed (subscription_id: sub_94627711fc104af3b56680dc7a6da5f0)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: RyQzgi0g7WMCYRILFGHpzzhd641FeNnTfy5IQKN9o2ZiAtq7N_AkWZE42DqrH6xT6R6pcgiThgoI6cTnycFizQ

  📡 Notification received:
     Event:      insert
     Collection: ws_subscribe_example_go
     Record IDs: [RyQzgi0g7WMCYRILFGHpzzhd641FeNnTfy5IQKN9o2ZiAtq7N_AkWZE42DqrH6xT6R6pcgiThgoI6cTnycFizQ]
     Timestamp:  2026-06-10T01:48:12.629489+00:00

Inserting record 2...
✓ Inserted: MnBwEap0Ngprtpn0-MQEcJMvFXIkZHFvAjVDOOiM_NIBgaErA-jEKfvVYIUDOVgjz1D-HFRJIJ8NAz4OdGliBA

  📡 Notification received:
     Event:      insert
     Record IDs: [MnBwEap0Ngprtpn0-MQEcJMvFXIkZHFvAjVDOOiM_NIBgaErA-jEKfvVYIUDOVgjz1D-HFRJIJ8NAz4OdGliBA]

=== Unsubscribing ===
✓ Unsubscribed

✓ WebSocket subscription example completed successfully
✅ [32mGo subscription test complete![0m
📦 [36mEnsuring Python example dependencies in .venv...[0m

🐍 [33mPython WebSocket Subscription Test...[0m
=== WebSocket Subscription Example (Python) ===

✓ Authentication successful
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_py' ===
✓ Subscribed (subscription_id: sub_27b20f7ddf0740c58298af4453e7727e)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: w1Jd2RQyjIFy2iUuzaAeD7zNoIhJil24rzn2zBbUe89VY4cTHe_whWcs9QIVMPSopQK9YFRI6fJ-nKRmJNln6Q

  📡 Notification received:
     Event:      insert
     Collection: ws_subscribe_example_py
     Record IDs: w1Jd2RQyjIFy2iUuzaAeD7zNoIhJil24rzn2zBbUe89VY4cTHe_whWcs9QIVMPSopQK9YFRI6fJ-nKRmJNln6Q
     Timestamp:  2026-06-10T01:48:13.023436+00:00

Inserting record 2...
✓ Inserted: JSNMoMtyiU3uZ0cCu46bkd3yu0zAKijxkIzlBsk5B0fy9kwjY0L-m-Oa479csGevs9WNrwalT_L_Lx0ZuRVWgg

  📡 Notification received:
     Event:      insert
     Record IDs: JSNMoMtyiU3uZ0cCu46bkd3yu0zAKijxkIzlBsk5B0fy9kwjY0L-m-Oa479csGevs9WNrwalT_L_Lx0ZuRVWgg

=== Unsubscribing ===
✓ Unsubscribed: {'collection': 'ws_subscribe_example_py', 'found': True, 'unsubscribed': True}

✓ WebSocket subscription example completed successfully
✅ [32mPython subscription test complete![0m
📦 [36mBuilding TypeScript client library...[0m
npm warn EBADENGINE Unsupported engine {
npm warn EBADENGINE   package: 'vitest@4.1.8',
npm warn EBADENGINE   required: { node: '^20.0.0 || ^22.0.0 || >=24.0.0' },
npm warn EBADENGINE   current: { node: 'v23.6.0', npm: '10.9.2' }
npm warn EBADENGINE }

> @ekodb/ekodb-client@0.21.0 prepare
> npm run build


> @ekodb/ekodb-client@0.21.0 build
> tsc


up to date, audited 52 packages in 2s

17 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.21.0 build
> tsc

✅ [32mTypeScript client built![0m

📘 [33mTypeScript WebSocket Subscription Test...[0m
=== WebSocket Subscription Example ===

✓ Authentication successful
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_ts' ===
✓ Subscribed (subscription_id: sub_495cd9cd331048a09b9cc1250fb5bf15)

=== Listening for mutation notifications ===

Inserting a record...

  📡 Notification received:
     Event:      insert
     Collection: ws_subscribe_example_ts
     Record IDs: R3T1k5F_NUjclzgqIzh7h6jK_f7V41FsnNg6uwODIBZVba7_i_sCP4Geyc1xCjMcJXqTNS-w8n5Zg-EvZ8ZO4Q
     Timestamp:  2026-06-10T01:48:16.059137+00:00
     Records:    {"active":true,"id":"R3T1k5F_NUjclzgqIzh7h6jK_f7V41FsnNg6uwODIBZVba7_i_sCP4Geyc1xCjMcJXqTNS-w8n5Zg-E...
✓ Inserted record: R3T1k5F_NUjclzgqIzh7h6jK_f7V41FsnNg6uwODIBZVba7_i_sCP4Geyc1xCjMcJXqTNS-w8n5Zg-EvZ8ZO4Q

Inserting another record...

  📡 Notification received:
     Event:      insert
     Collection: ws_subscribe_example_ts
     Record IDs: wfzzEArSA2T3oOnub_CISdiqPPNbJSAR68Ex2YdJn-M3WW1-5Z-Ar33hSTUooqeW5GrrHVXfCMv9TCzKE5wM-g
     Timestamp:  2026-06-10T01:48:17.075192+00:00
     Records:    {"active":true,"id":"wfzzEArSA2T3oOnub_CISdiqPPNbJSAR68Ex2YdJn-M3WW1-5Z-Ar33hSTUooqeW5GrrHVXfCMv9TCz...

=== Unsubscribing ===
✓ Unsubscribed: {"collection":"ws_subscribe_example_ts","found":true,"unsubscribed":true}

✓ WebSocket subscription example completed successfully
✅ [32mTypeScript subscription test complete![0m

🟣 [33mKotlin WebSocket Subscription Test...[0m
To honour the JVM settings for this build a single-use Daemon process will be forked. For more on this, please refer to https://docs.gradle.org/8.11.1/userguide/gradle_daemon.html#sec:disabling_the_daemon in the Gradle documentation.
Daemon will be stopped at the end of the build 
> Task :checkKotlinGradlePluginConfigurationErrors SKIPPED
> Task :compileKotlin UP-TO-DATE
> Task :compileJava NO-SOURCE
> Task :processResources NO-SOURCE
> Task :classes UP-TO-DATE

> Task :run
=== WebSocket Subscription Example (Kotlin) ===

SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
✓ Authentication successful

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Subscribing to 'ws_subscribe_example_kt' ===
✓ Subscribed (subscription_id: sub_34891e70f86c4f5495f0b48007c6b3b7)

=== Performing mutations to trigger notifications ===
Inserting record 1...
✓ Inserted: "dSUOj_Zn6VYl80ImVkGGUxT3fjvAfyPLYJRR8uGHFbgZxEIU9VErAsvNXJI603daRbLEBLdYoZ-34a-4Z3svWQ"

  📡 Notification received:
     Event:      "insert"
     Collection: "ws_subscribe_example_kt"
     Record IDs: ["dSUOj_Zn6VYl80ImVkGGUxT3fjvAfyPLYJRR8uGHFbgZxEIU9VErAsvNXJI603daRbLEBLdYoZ-34a-4Z3svWQ"]
     Timestamp:  "2026-06-10T01:48:22.059110+00:00"

Inserting record 2...
✓ Inserted: "DCeT48dGXGkKzfkM8IY0xqdIwmK1CwB0zQ8eVVKFKmIHa1Qr1zcVsNzOSqw-eNDaCJEfWE-d7HRMCEqUAKaX6w"

  📡 Notification received:
     Event:      "insert"
     Record IDs: ["DCeT48dGXGkKzfkM8IY0xqdIwmK1CwB0zQ8eVVKFKmIHa1Qr1zcVsNzOSqw-eNDaCJEfWE-d7HRMCEqUAKaX6w"]

=== Unsubscribing ===
✓ Unsubscribed

✓ WebSocket subscription example completed successfully

BUILD SUCCESSFUL in 3s
2 actionable tasks: 1 executed, 1 up-to-date
✅ [32mKotlin subscription test complete![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
✅ [32mAll WebSocket Subscription Tests Passed![0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
