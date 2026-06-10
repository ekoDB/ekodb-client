make test-examples-rust
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
