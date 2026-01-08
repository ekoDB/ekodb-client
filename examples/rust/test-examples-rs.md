make test-examples-rust
🧪 [36mRunning Rust examples (direct HTTP/WebSocket)...[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/simple_crud`
✓ Authentication successful

=== Insert Document ===
Inserted: Object {"id": String("7pTFzehYLeEOvB6cd3NkU9eshBVSH3jCnSiVjDTaXvD5T_efrBKRECEFN4LioF2Ell-8Wowj6H9Ti3XIYxx3GA")}

=== Find by ID ===
Found: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("7pTFzehYLeEOvB6cd3NkU9eshBVSH3jCnSiVjDTaXvD5T_efrBKRECEFN4LioF2Ell-8Wowj6H9Ti3XIYxx3GA"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}

=== Find with Query ===
Found documents: Array [Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("7pTFzehYLeEOvB6cd3NkU9eshBVSH3jCnSiVjDTaXvD5T_efrBKRECEFN4LioF2Ell-8Wowj6H9Ti3XIYxx3GA"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}]

=== Update Document ===
Updated: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("7pTFzehYLeEOvB6cd3NkU9eshBVSH3jCnSiVjDTaXvD5T_efrBKRECEFN4LioF2Ell-8Wowj6H9Ti3XIYxx3GA"), "name": Object {"type": String("String"), "value": String("Updated Record")}, "value": Object {"type": String("Integer"), "value": Number(100)}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/simple_websocket`
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: "cX2Z1oH6GY0QcoSgEOkcOE0yeXaFzl8J2doqY5FDy9tWJiGwQq3QrOV5nwNMdXFinE85BmIE5v1uuBN3a-cZ1Q"

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
        "id": "cX2Z1oH6GY0QcoSgEOkcOE0yeXaFzl8J2doqY5FDy9tWJiGwQq3QrOV5nwNMdXFinE85BmIE5v1uuBN3a-cZ1Q",
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/collection_management`
✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: "QN3skoUoLHpAvbsxbkRsf6G6iO0rh4mgeojCAJZaffxSmWvxMOOw5UzZeU7Pny_sQEnJsp2n6W5j290BqPQ1Tg"

=== List Collections ===
Total collections: 4
Sample collections: ["websocket_test", "test_collection", "demo_collection", "batch_users"]

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/document_ttl`
✓ Authentication successful

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: "Htu-XK7xWs8MyzSEOA3DXjt_atsvIBcmx-kzy-Ac4gy1hGk-kLk9rM6RqR_i7Yh37BTqAQyR80pKaLiPomm1ag"

=== Insert Document with TTL (5 minutes - integer) ===
✓ Inserted document: "bFc5p-UtrHLx4UTfPy6mYPfeh9pOl6VpbqziudXEevKGsrmpzraJJb0i21y-nxgWCZ4jglkUgcqrTBRpFnM-cA"

=== Insert Document with TTL (30 minutes - duration string) ===
✓ Inserted document with duration string TTL: "BU_JMkLtxWNj-rmDt9btZdWYTUEHY9WjOR7A2rt1SLHkRUdhLN-bxMXmJXDCV3tklXW6EoYl61FzRrclZlfdAw"

=== Query Documents ===
✓ Found 3 documents with TTL

=== Update Document ===
✓ Updated document

=== Delete Document ===
✓ Deleted document

✓ All document TTL operations completed successfully
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.90s
     Running `target/debug/examples/websocket_ttl`
✓ Authentication successful

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: String("cvf3ITM-Nku6S974F-RC0YlyOU1X8vkzVseULMoMm3VNNu6hDbkEBRLS5xWWuq0Gx6973Nkd9jB0-YwpXUKG9Q")

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket

Record 1:
  id: "cvf3ITM-Nku6S974F-RC0YlyOU1X8vkzVseULMoMm3VNNu6hDbkEBRLS5xWWuq0Gx6973Nkd9jB0-YwpXUKG9Q"
  name: {"type":"String","value":"WebSocket TTL Test"}
  ttl: "2026-01-08T02:28:15.037835Z"
  value: {"type":"Integer","value":42}

✓ WebSocket TTL example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/http_functions`
🚀 ekoDB Scripts Example (Rust/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: wpfUbLwmPdrYbCCL1QfLmEg8Pf1XzKJVJUca1wuHzKgQ5RlyTzJK9wXiHLj-ud_sQ8Anl3fc9xn0TnWmH2FNBQ
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: "j_P2dIc0KgfB5YDHuWKoFOPdbAObHqxZN-UZE88zDJ6r5AWnc4-IWf2hVfo-CN-dzngrjjNdHnZOfeFfPTkYOw"
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Complex Filter with Multiple Conditions

✅ Script saved: Yk0t58I0_PwPaHLYyX5HrJoPmc7KdVFILzQ-CQ4FmkmaE5erixymeCYWJwq_4tIfUKiQRCR_cdSJLqxaBZpYPQ
📊 Found 3 users (status=active, score>50, sorted by score)

📝 Example 4: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: fRxzBTmOxl0lwjVjxN_2hRPkJcH1UhyrpFCYldsQ-auYqYNPmpekHxBZjZahWchDL4slCSerI5FWPTsb0b_pKg
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"avg_score":50.0,"count":5,"max_score":90,"status":"inactive"}
   {"avg_score":60.0,"count":5,"max_score":100,"status":"active"}

📝 Example 4: Function Management

📋 Total scripts: 4
🔍 Retrieved script: Get Active Users
✏️  Script updated
🗑️  Script deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

✅ All examples completed!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/transactions`
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: ABQGhc8aUiS5VumQGZljX1i7MFyQ6ZOKicp8AC2mdjHlZgbUkADdF1m0xeN5QS6KEpoXD2GPmqFjnztE2TXekQ
Created Bob: $500 - ID: FjPPlbvpCTijijDQvGjJ0gZb1DcI-Vfp1aVwCRQqlijPO3esCVeJ6xKndCuJ8bG3o2_v5s7ZychxKFZFXRv3dw

=== Example 1: Begin Transaction ===
Transaction ID: 2dc1867c-5c6a-4ae7-8a38-9d51cb92113f

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: "Active"
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: 800
Bob: 700

=== Example 5: Rollback ===
New transaction: f40a0d06-6981-4946-9651-74920dab7459
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: 700

=== Cleanup ===
✓ Deleted test accounts

✓ All transaction examples completed
✅ [32mRust direct examples complete![0m
🛠️  [36mBuilding client library...[0m
cargo build -p ekodb_client
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
✅ [32mClient build complete![0m
🧪 [36mRunning Rust client library examples...[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_simple_crud`
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: Record { fields: {"id": String("ZrC9jEyb4ssRrkKfhlRm0LyVqpmihKXZnJ8dygPo7-nR6wETbKVRB_Nc3N5gP0MWsuuIIxs-GOaNrr_NuseXKQ")} }

=== Find by ID ===
Found: Record { fields: {"user_id": Object({"type": String("String"), "value": String("550e8400-e29b-41d4-a716-446655440000")}), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "price": Object({"type": String("Float"), "value": Float(99.99)}), "categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])}), "data": Object({"value": String("aGVsbG8gd29ybGQ="), "type": String("String")}), "created_at": Object({"type": String("DateTime"), "value": String("2026-01-08T01:28:19.641913Z")}), "tags": Object({"value": Array([String("tag1"), String("tag2"), String("tag3")]), "type": String("Array")}), "id": String("ZrC9jEyb4ssRrkKfhlRm0LyVqpmihKXZnJ8dygPo7-nR6wETbKVRB_Nc3N5gP0MWsuuIIxs-GOaNrr_NuseXKQ"), "name": Object({"value": String("Test Record"), "type": String("String")}), "metadata": Object({"value": Object({"nested": Object({"deep": Boolean(true)}), "key": String("value")}), "type": String("Object")}), "value": Object({"type": String("Integer"), "value": Integer(42)}), "embedding": Object({"type": String("Array"), "value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)])})} }

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Some("Test Record")
  value (Integer): Some(42)
  active (Boolean): Some(true)
  price (Decimal): Some(99.99)
  created_at (DateTime): Some("2026-01-08T01:28:19.641913Z")
  user_id (UUID): Some("550e8400-e29b-41d4-a716-446655440000")
  tags (Array): 3 items
  metadata (Object): 2 keys
  embedding (Vector): 5 dims
  categories (Set): 2 items
  data (Bytes): 11 bytes

=== Find with Query ===
Found documents: [Record { fields: {"price": Object({"type": String("Float"), "value": Float(99.99)}), "tags": Object({"value": Array([String("tag1"), String("tag2"), String("tag3")]), "type": String("Array")}), "name": Object({"value": String("Test Record"), "type": String("String")}), "user_id": Object({"value": String("550e8400-e29b-41d4-a716-446655440000"), "type": String("String")}), "categories": Object({"value": Array([String("electronics"), String("computers")]), "type": String("Array")}), "created_at": Object({"type": String("DateTime"), "value": String("2026-01-08T01:28:19.641913Z")}), "embedding": Object({"type": String("Array"), "value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)])}), "value": Object({"type": String("Integer"), "value": Integer(42)}), "data": Object({"type": String("String"), "value": String("aGVsbG8gd29ybGQ=")}), "id": String("ZrC9jEyb4ssRrkKfhlRm0LyVqpmihKXZnJ8dygPo7-nR6wETbKVRB_Nc3N5gP0MWsuuIIxs-GOaNrr_NuseXKQ"), "metadata": Object({"value": Object({"nested": Object({"deep": Boolean(true)}), "key": String("value")}), "type": String("Object")}), "active": Object({"type": String("Boolean"), "value": Boolean(true)})} }]

=== Update Document ===
Updated: Record { fields: {"user_id": Object({"type": String("String"), "value": String("550e8400-e29b-41d4-a716-446655440000")}), "tags": Object({"value": Array([String("tag1"), String("tag2"), String("tag3")]), "type": String("Array")}), "value": Object({"value": Integer(100), "type": String("Integer")}), "active": Object({"type": String("Boolean"), "value": Boolean(true)}), "id": String("ZrC9jEyb4ssRrkKfhlRm0LyVqpmihKXZnJ8dygPo7-nR6wETbKVRB_Nc3N5gP0MWsuuIIxs-GOaNrr_NuseXKQ"), "data": Object({"value": String("aGVsbG8gd29ybGQ="), "type": String("String")}), "categories": Object({"value": Array([String("electronics"), String("computers")]), "type": String("Array")}), "price": Object({"value": Float(99.99), "type": String("Float")}), "created_at": Object({"type": String("DateTime"), "value": String("2026-01-08T01:28:19.641913Z")}), "metadata": Object({"value": Object({"nested": Object({"deep": Boolean(true)}), "key": String("value")}), "type": String("Object")}), "embedding": Object({"value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "type": String("Array")}), "name": Object({"value": String("Updated Record"), "type": String("String")})} }

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_simple_websocket`
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: Mtnz_wDzaVFmAifFOmVCkqvLMIdX8B2l8Etn-nQjRUC4O7mPRlQHl6VlZ_xkjHUptz5CwuHyd-zKOjeAlQgiCA

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket

Record 1:
  active: true
  value: 42
  id: "Mtnz_wDzaVFmAifFOmVCkqvLMIdX8B2l8Etn-nQjRUC4O7mPRlQHl6VlZ_xkjHUptz5CwuHyd-zKOjeAlQgiCA"
  name: "WebSocket Test Record"

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_collection_management`
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: "aeRaZSEwWe2OK4IXVzQz2dW0EkNZXpLDMjLcG3oC-z2DYD2n6I15zLaoG2elVZDQF-xTjn-g_tAAqNiRvLTfxg"

=== List Collections ===
Total collections: 9
Sample collections: ["websocket_test", "test_collection", "client_collection_management_rust", "users", "ws_ttl_test"]

=== Count Documents ===
Document count: 1

=== Check Collection Exists ===
Collection exists: true

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_kv_operations`
✓ Client created

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_transactions`
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 9WuRuwzufPIJbf8ZWRincw7K6hfHBKKseMAGLhtOGXgK-et71xXoFwUAcEVzLWXZEeWbM_3nKHrvFUgzI3j2tw
Created Bob: $500 - ID: Cbsb2jDWMMLZG_6ATXNNqFtAr98j2YNfhO8SPDMbSttY6_b6eumniTu3BUGc0jdAmVrbaLL_TZ9W_itTinG35A

=== Example 1: Begin Transaction ===
Transaction ID: 6b1dd2d3-95bf-4e7b-809d-4f376ee48305

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: "Active"
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 907a1150-a845-4925-a01e-7abb26fb639e
Status before rollback: "Active"
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_query_builder`
=== ekoDB Query Builder Example ===

=== Inserting Sample Data ===
✓ Inserted 5 users

=== Simple Equality Query ===
✓ Found 3 active users
  - Some(Object({"type": String("String"), "value": String("Bob")}))
  - Some(Object({"value": String("David"), "type": String("String")}))
  - Some(Object({"value": String("Alice"), "type": String("String")}))

=== Range Query (age >= 28 AND age < 35) ===
✓ Found 3 users in age range
  - Some(Object({"value": String("Eve"), "type": String("String")}))
  - Some(Object({"value": String("Bob"), "type": String("String")}))
  - Some(Object({"value": String("David"), "type": String("String")}))

=== IN Operator ===
✓ Found 4 users with status active or pending
  - Some(Object({"value": String("Eve"), "type": String("String")}))
  - Some(Object({"value": String("Bob"), "type": String("String")}))
  - Some(Object({"value": String("David"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("Alice")}))

=== NOT IN Operator ===
✓ Found 4 users not inactive

=== String Pattern Matching ===
✓ Found 5 users with @example.com email

=== Regex Query ===
✓ Found 0 users with names starting with A-C

=== Complex Query (active AND age >= 28 AND score > 1500) ===
✓ Found 2 users matching all conditions
  - Some(Object({"type": String("String"), "value": String("Bob")}))
  - Some(Object({"type": String("String"), "value": String("David")}))

=== OR Query ===
✓ Found 2 users with age < 28 OR age > 32
  - Some(Object({"value": String("Alice"), "type": String("String")}))
  - Some(Object({"value": String("Charlie"), "type": String("String")}))

=== Sorted Query (by score descending) ===
✓ Top 3 users by score:
  1. Some(Object({"type": String("String"), "value": String("Bob")}))
  2. Some(Object({"type": String("String"), "value": String("David")}))
  3. Some(Object({"type": String("String"), "value": String("Alice")}))

=== Pagination (page 2, size 2) ===
✓ Page 2 results:
  - Some(Object({"type": String("String"), "value": String("Charlie")}))
  - Some(Object({"value": String("David"), "type": String("String")}))

=== NOT Operator ===
✓ Found 4 users NOT inactive

=== Complex Chained Query ===
✓ Found 3 users with all conditions

=== Cleanup ===
✓ Deleted collection

✓ All query builder operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_search`
=== ekoDB Search Example ===

=== Inserting Sample Documents ===
✓ Inserted 5 sample documents

=== Basic Text Search ===
✓ Found 3 results for 'programming'
  1. Score: 19.8000 - Some(String("Rust Programming"))
  2. Score: 6.6000 - Some(String("Python for Data Science"))
  3. Score: 6.6000 - Some(String("JavaScript Web Development"))

=== Fuzzy Search ===
✓ Found 3 results for 'progamming' (typo)
  1. Score: 1.6500 - Some(String("Rust Programming"))
  2. Score: 0.5500 - Some(String("Python for Data Science"))
  3. Score: 0.5500 - Some(String("JavaScript Web Development"))

=== Field-Specific Search ===
✓ Found 4 results in title/description
  1. Score: 4.0000
     Title: Some(String("Machine Learning Basics"))
     Matched: ["title.value", "description", "description.value", "title"]
  2. Score: 2.0000
     Title: Some(String("Python for Data Science"))
     Matched: ["description", "description.value"]
  3. Score: 1.0000
     Title: Some(String("Rust Programming"))
     Matched: ["description", "description.value"]
  4. Score: 1.0000
     Title: Some(String("Database Design"))
     Matched: ["description.value", "description"]

=== Weighted Search ===
✓ Found 2 results with field weights
  1. Score: 23.1000 - Some(String("Python for Data Science"))
  2. Score: 3.3000 - Some(String("Machine Learning Basics"))

=== Advanced Search Options ===
✓ Found 1 results with stemming
  1. Score: 19.8000 - Some(String("Database Design"))

=== Search with Limit ===
✓ Limited to 2 results (requested 2)
  1. Score: 19.8000 - Some(String("Rust Programming"))
  2. Score: 6.6000 - Some(String("Python for Data Science"))

=== Cleanup ===
✓ Deleted collection

✓ All search operations completed successfully
Execution time: 1ms
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_schema`
=== ekoDB Schema Management Example ===

=== Creating Collection with Schema ===
✓ Created collection 'schema_client_rust' with schema

=== Inserting Valid Documents ===
✓ Inserted user 1: Some(String("jF3_q6avvxbWt6BAG2wQdmizliI62Pfl8Ggrg8tyfC74J8xqmCAnnuLos_Mx21NjJV5-zwKC4NpN4Hd6OcYC_w"))
✓ Inserted user 2: Some(String("4w7XagBqQ0MfvTGR3ATQ9cr8nWEGRE7973N4xY4j4tFkOw_x9XHlHXosCPwwZhWDG7gsgSYZXZqjoiYQMCV_Ew"))

=== Getting Schema ===
✓ Schema for schema_client_rust:
  - status: String
  - age: Integer
  - title: String
    (required)
  - email: String
    (required)

=== Listing Collections ===
✓ Total collections: 9
  Sample: ["websocket_test", "test_collection", "users", "schema_client_rust", "ws_ttl_test"]

=== Cleanup ===
✓ Deleted collection

✓ All schema management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_joins`
=== ekoDB Rust Client - Join Operations Example ===

=== Setting up sample data ===
✓ Sample data created

=== Example 1: Single collection join (users with departments) ===
✓ Found 2 users with department data
  - Alice Johnson: Engineering
  - Bob Smith: Sales

=== Example 2: Join with filtering ===
✓ Found 1 users in Engineering
  - Alice Johnson: Building A

=== Example 3: Join with user profiles ===
✓ Found 2 users with profile data
  - Alice Johnson: Senior Software Engineer
  - Bob Smith: Sales Manager

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_document_ttl`
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: "c1bR9SZlPbvjBiOK6Xg0hTE2SPuXUMNF26X_mvuTRjrnG7I6CROSiV9ML1BTxy86UleyJMhZp7h14Nn92xNvHg"

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: Some(String("ZjycCHWn5rcuxfHdOUstk9eUHDSSNSbwGFPH0vVGXO8PQRCg3Kr4vyTKU9ZnG8SFoLuliJfWODAYB8OVETOiiA"))

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_websocket_ttl`
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: Some(String("HSpWjHUnZovibaNGNK4GceSNCHiGanjEd7eBwXgLhVSdLdGYiCYgap3sRnFC2lEZc3jOImhTZD89RAMb34bhqw"))

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_edge_cache`
=== ekoDB as Edge Cache - Simple Example ===

Setting up edge cache collection...
✓ Cache entry created

Creating edge cache lookup script...
✓ Edge cache script created: s31rwtDigGUZXAobL6vNauHvK5tcr6V3S27w77PkpW2YlzFJaiakG11tfHvMX-ehNvmHel_jOjaoYt4v7QIPkQ

Call 1: Cache lookup
Response time: 1ms
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_functions`
=== ekoDB Rust Client - Scripts Example ===

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: NP3yySrXFNComOq4l0_uoWqnD5xekkorvwgpInuubhqqylU3yt3TcwgADXtKNI6l1Lvu_mm7WFIwQ86xeRwYgw
📊 Found 20 active users

📝 Example 2: Parameterized Script

✅ Script saved: 4S82seFrozu0ql52ziLhF-GGrjPzkllbR7KpICJKBNhoqRgW_U16YUH4y8RICG4Kh1Ay3di2abdiwVkEJWMTlQ
📊 Found 20 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: 6cqW9Xjhi8ndeiLDlwZIeVMtBrP2VjOLEfd6iWsgYRG39fTS3TaBByv2UJeJRUO4ahZ-jIPDYXtzNzDqNbR_ww
📊 Statistics: 2 groups

📝 Example 4: Script Management

📋 Total scripts: 6
🔍 Retrieved script: Get Active Users
✏️  Script updated
🗑️  Script deleted

ℹ️  Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.


✅ All examples completed!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
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

Using CallFunction to replace inline logic in SWR pattern...

✅ Saved reusable function: fetch_and_store_github
✅ Saved SWR function using composition: swr_github_user

First call (cache miss - will fetch from GitHub):
   ⏱️  Duration: 213.358209ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 89.888ms
   📊 Records: 1
   🚀 Cache speedup: 2.4x faster!

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_functions_complete`
🚀 ekoDB Rust Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved: oULtMeHn7wGg0_zt5WgzXm00M8uGpWMQRSPi006u071FUxMaRedwJzbiaW2cdnLlPtGDaaUQvwY-74HH3wyV8Q
📊 Found 2 product groups
   Record { fields: {"category": String("Furniture"), "count": Integer(2), "avg_price": Float(474.0)} }
   Record { fields: {"count": Integer(3), "category": String("Electronics"), "avg_price": Float(575.6666666666666)} }
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved
📊 Found 2 categories
   Record { fields: {"category": String("Furniture"), "count": Integer(2)} }
   Record { fields: {"category": String("Electronics"), "count": Integer(3)} }
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_functions_kv_wrapped`
🚀 ekoDB Rust KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: Some(String("jtS3qCN7wBaLkyiMUcnVbUVxSWpzNOz_7cMw5b88tDvmhxdtsM6eGJ-fiwgCoi94ziHqgVq4ihN2C6B6Y7J8iQ"))
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: hm7TeHkKNY8wBXNP5MDhkiklwp6BgomQAAZIs-t8DmChf0YtpWCahWSIFyrq1TxohoWA-r4HqGo5VxGShJNKhQ
📊 Script executed
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: Some(Object {"role": String("admin"), "userId": String("user_abc")})
🔍 Key exists: true
✅ Set cached data
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: oZT0G-_xUHaGgoCYv0F4JWFy1dZbkK2V-aV-ND3dWK-SRza4vl4VlPfElvvKRudzsTjwMxDvCNvDeTT_7c0s0Q
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Script

✅ Script saved: qnRIC_xE3Y9yA90U3xf6n7npkLLTzLAjZSlwMxPdYc9bNxJ7GHiyQDwkw-jjq0Q80ZMnSu1E8ZfVefsnDUtSCw
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_swr_pattern`
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_github_user_rs (opanR6WAuo-1FU0LgsLST-98NyQUW98X8hDvUbPFmEKL20hiJb5ZVLlCNDRNg8-UQiSN_GuaIuD8RG0MwhJsmw)

Step 2: First call - Cache miss, fetches from GitHub API
Result: FunctionStats { input_count: 0, output_count: 1, execution_time_ms: 0, stages_executed: 2, stage_stats: [] }
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 85ms (served from cache)
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

✓ Created native SWR script: github_user_native (poA7XdX84wNdPTOUJKDzxRkAW1NK0mCnr6C_u5ndK3B3rtzTcL3zKGnUzOsPZAYp0vB6GucvY6OyNrxi8azjqQ)

First call (cache miss - will fetch from GitHub API):
  Response time: 79ms
  Records returned: 1
  ✓ Data fetched from API and cached with 15m TTL

Second call (cache hit - instant from KV store):
  Response time: 3ms
  Speedup: 26.3x faster
  ✓ Lightning fast cache hit


Example 2: SWR with Audit Trail Collection
─────────────────────────────────────────────────────

✓ Created SWR script with audit trail: product_swr_audit (wySXF3rw3T7U_mj90L-9K_3aaU3KNSOXQgD9Y4nJ1F2Ez60UqEgpAGeDRJKXHXTm354awVMVLT-FJKHdj3iKvg)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
─────────────────────────────────────────────────────

Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (XC8aVSkfExFbWK0OCMakM4Gq5R53IuGBYEIsL9IW0eN7xDYRH5iCPNbbtx79dko8fG624QlBORMzCZ_oA2G_aQ)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
─────────────────────────────────────────────────────

✓ Created dynamic TTL script: flexible_cache (cMz8mAEMTAE6eV354ybkDjnkNy1XPcD53KIP6LfPT0LEqheP745_zkijK4Xw44zK_ihaSsK5NC0-swi7RChpCA)
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

    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/client_functions_advanced`
🚀 ekoDB Rust Advanced Scripts Example

📋 Setting up test data...
✅ Created 8 products

📝 Example 1: List All Products

✅ Script saved
📊 Found 8 products
⏱️  Execution time: 0ms

📝 Example 2: Group Products by Category

✅ Script saved
📊 Category breakdown:
   Record { fields: {"category": String("Electronics"), "avg_price": Float(367.0), "count": Integer(5)} }
   Record { fields: {"avg_price": Float(365.6666666666667), "count": Integer(3), "category": String("Furniture")} }
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced script examples finished!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_functions_ai`
🚀 ekoDB Rust AI Scripts Example

📋 Setting up test data...
✅ Created 2 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
🤖 AI Response:
   Vector databases offer several benefits such as high precision, efficient storage, scalability, and easy manipulation of data. They allow complex geographic features to be accurately represented and can handle large amounts of spatial data. They also support various operations like overlay, network analysis, and proximity analysis.
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_functions_crud`
🚀 ekoDB Rust CRUD Scripts Example

📋 Setting up test data...
✅ Created 10 test users

📝 Example 1: List All Users

✅ Script saved
📊 Found 10 users
⏱️  Execution time: 0ms

📝 Example 2: Count Users by Status

✅ Script saved
📊 User counts by status:
   Record { fields: {"count": Integer(7), "status": String("active")} }
   Record { fields: {"status": String("inactive"), "count": Integer(3)} }
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD script examples finished!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_functions_search`
🚀 ekoDB Rust Search Scripts Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Script saved
📊 Found 5 documents
   1. Database Design Principles (Database)
   2. Natural Language Processing (AI)
   3. Vector Databases Explained (Database)
   4. Introduction to Machine Learning (AI)
   5. Getting Started with ekoDB (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Script saved
📊 Documents by category:
   Record { fields: {"category": String("AI"), "count": Integer(2)} }
   Record { fields: {"count": Integer(3), "category": String("Database")} }
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_chat_basic`
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: xgjltuke0C_qDSPtweH8VAF3KfSfnfgBJ4PjqyqAMi-AoDgZ4i_wBNOiYKudw-nNbSovEz51Xtd5rBRnxzjoqQ

=== Sending Chat Message ===
Message ID: UwHecmUsCqymf62UHXf1uRH0vTuXNa82bXwtBh795CBZdOTwJIpgdCfIQAhUdgSOkGEu-qeiWvKlea2eYi1a1Q

=== AI Response ===
Response 1: ekoDB is a high-performance database that offers intelligent caching and real-time capabilities. It stands out due to its AI integration, which powers several of its features. One of these features is the AI Chat Integration, which allows you to query your database using natural language. The AI provides responses that are contextually relevant, simplifying the task of retrieving information from your database.

In addition to this, ekoDB also supports various types of searches. This includes full-text search, vector search, and hybrid search. These searching capabilities are designed to automatically retrieve the context, making it easier for you to find the specific data you're looking for in your database.

=== Context Used (3 snippets) ===

Snippet 1:
  Collection: client_chat_basic_rust
  Score: 0.6333
  Matched Fields: ["content", "title", "category"]
  Record: Object {"category": String("features"), "content": String("The chat feature allows you to query your database using natural language and get AI-powered responses with relevant context."), "id": String("TTytfhBDp_BtkBfY3k2JOObSMe7Im2rhrV1RHZmH1eiR0_A3DlihfwZNOR07vX_SsRsfvJufDzr9gFtkB1mUjQ"), "title": String("AI Chat Integration")}

Snippet 2:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["content", "title"]
  Record: Object {"category": String("documentation"), "content": String("ekoDB is a high-performance database with intelligent caching, real-time capabilities, and AI integration."), "id": String("go7TW0kbMM4OKIjvPY9BrFA4Y2HOGchF62GCdqT0XguuiIN1cQs79PaQIwcK-OLnXM17UdrLsRPMt9ClVaCxbw"), "title": String("Introduction to ekoDB")}

Snippet 3:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["category", "content", "title"]
  Record: Object {"category": String("features"), "content": String("ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval."), "id": String("dHr3EwkRuZMhWB2x2I1tlGMha7HAom87rnTy9Bv1yMSsbxjoBInYl4PKSNTqghSUdvbrbslrb8j5k5tTwjmkIA"), "title": String("Search Features")}

Execution Time: 3543ms

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_chat_advanced`
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: sAK_BiFiHVZsEeAIRJTu7Agt64ptNfvyO9UrZvUNjj1Qgxd4giRozf5Pvh227BG5MV1HhVSeXMiOKfwJX7HfSw

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information provided, one product available is the ekoDB. It is a high-performance database product priced at $99.

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
✓ Created second session: gHD2h5JPNfpx9rJoPSe4MEonGmLZ-u8DV0WHh6_0bxs8OKVYTa2KGw0JAOagkqWNwxX0p4tvw47Uc7cPfZGWjg
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_chat_sessions`
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: xRFW70IPVqR2QQuDqmIrLI2dXG6VEibWIj7VCaTTiuibcgUTbrgNe4Z6s4_TLO-bPBxdS81NWCIPv8qbaMqoAg
=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information available, we have a product called "ekoDB". It is a high-performance database product with AI capabilities. The price of this product is $99.

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: p-66_8DjM2jGPdq8SzMYDdxYKC9ugxRgvBFDCeXCP70Duz1OFUzVqIRTrSm49nG3PLFZGBPY-6JMkA4q7SsYKA
  Parent: xRFW70IPVqR2QQuDqmIrLI2dXG6VEibWIj7VCaTTiuibcgUTbrgNe4Z6s4_TLO-bPBxdS81NWCIPv8qbaMqoAg

=== Listing Sessions ===
✓ Found 4 sessions
  Session 1: p-66_8DjM2jGPdq8SzMYDdxYKC9ugxRgvBFDCeXCP70Duz1OFUzVqIRTrSm49nG3PLFZGBPY-6JMkA4q7SsYKA (Untitled)
  Session 2: xRFW70IPVqR2QQuDqmIrLI2dXG6VEibWIj7VCaTTiuibcgUTbrgNe4Z6s4_TLO-bPBxdS81NWCIPv8qbaMqoAg (Untitled)
  Session 3: gHD2h5JPNfpx9rJoPSe4MEonGmLZ-u8DV0WHh6_0bxs8OKVYTa2KGw0JAOagkqWNwxX0p4tvw47Uc7cPfZGWjg (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: p-66_8DjM2jGPdq8SzMYDdxYKC9ugxRgvBFDCeXCP70Duz1OFUzVqIRTrSm49nG3PLFZGBPY-6JMkA4q7SsYKA

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_convenience_methods`
=== ekoDB Convenience Methods Example ===

=== Record Builder Pattern ===
✓ Created record with fluent builder: Record { fields: {"id": String("h1Jb4JNkhxRhO3Mnm8lsET6CBk3_jsMwO6lONMyOAIctBIps1xXZfFPXFfPpb8sjy9XImwpzDDBL0tXlDWAe9g")} }

=== Upsert Operation ===
✓ First upsert (insert): Record { fields: {"age": Object({"value": Integer(35), "type": String("Integer")}), "email": Object({"type": String("String"), "value": String("bob@example.com")}), "id": String("h1Jb4JNkhxRhO3Mnm8lsET6CBk3_jsMwO6lONMyOAIctBIps1xXZfFPXFfPpb8sjy9XImwpzDDBL0tXlDWAe9g"), "name": Object({"value": String("Bob Smith"), "type": String("String")}), "active": Object({"value": Boolean(true), "type": String("Boolean")})} }
✓ Second upsert (update): Record { fields: {"email": Object({"value": String("bob.smith@newdomain.com"), "type": String("String")}), "name": Object({"value": String("Bob Smith"), "type": String("String")}), "age": Object({"type": String("Integer"), "value": Integer(36)}), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "id": String("h1Jb4JNkhxRhO3Mnm8lsET6CBk3_jsMwO6lONMyOAIctBIps1xXZfFPXFfPpb8sjy9XImwpzDDBL0tXlDWAe9g")} }

=== Find One Operation ===
✗ User not found
✓ User not found (as expected)

=== Exists Check ===
✓ Record exists: true
✓ Fake record exists: false (should be false)

=== Pagination ===
✓ Inserted 25 records for pagination
✓ Page 1: 10 records (expected 10)
✓ Page 2: 10 records (expected 10)
✓ Page 3: 6 records (expected 7)

=== Cleanup ===
✓ Deleted collection

✅ All convenience methods demonstrated successfully!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/bypass_ripple_example`
=== Bypass Ripple Example ===

1. Basic insert (ripple enabled):
   Inserted with ripple: Record { fields: {"id": String("UxE0vFNTabGw2uZkN4MLnJObXATlqHX9L_RgRRAkBCb5wwKFwfrMMYH5WMjzqHA1O8C14-6OBgWEISL0hKOzuw")} }

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: Record { fields: {"id": String("9tCL1y0qluYG7s_S4Hn0sBeI0URdqbo6VnMhYOc7D-PiSEJdF1QKpS2tH4V1rNENPeoNNjO335fRmpYbaDydtQ")} }

3. Update with bypass_ripple:
   Updated with bypass_ripple: Record { fields: {"name": Object({"type": String("String"), "value": String("Product 1")}), "price": Object({"value": Integer(150), "type": String("Integer")}), "id": String("UxE0vFNTabGw2uZkN4MLnJObXATlqHX9L_RgRRAkBCb5wwKFwfrMMYH5WMjzqHA1O8C14-6OBgWEISL0hKOzuw")} }

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: Record { fields: {"id": String("3YJw0A-ODb6NAsB17JpjT0pq3yBLnyk2BzUq8sb5JgLHUizSlQsz8clCtQ1wutz_ceEEJms0Qdj_9UlGSpv2Qg")} }

✅ All bypass_ripple operations completed successfully!
✅ [32mRust client examples complete![0m
