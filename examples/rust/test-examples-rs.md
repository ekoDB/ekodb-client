make test-examples-rust
🧪 [36mRunning Rust examples (direct HTTP/WebSocket)...[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.19s
     Running `target/debug/examples/simple_crud`
✓ Authentication successful

=== Insert Document ===
Inserted: Object {"id": String("A5r1kLy8gPB7jvE1cKIXu26mysu2re7gVe6gO9myxfOtpYkhWzLUm0lqb1UuUIztdQI9t726QMz1ZIIXp0jaqQ")}

=== Find by ID ===
Found: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("A5r1kLy8gPB7jvE1cKIXu26mysu2re7gVe6gO9myxfOtpYkhWzLUm0lqb1UuUIztdQI9t726QMz1ZIIXp0jaqQ"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}

=== Find with Query ===
Found documents: Array [Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("A5r1kLy8gPB7jvE1cKIXu26mysu2re7gVe6gO9myxfOtpYkhWzLUm0lqb1UuUIztdQI9t726QMz1ZIIXp0jaqQ"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}]

=== Update Document ===
Updated: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("A5r1kLy8gPB7jvE1cKIXu26mysu2re7gVe6gO9myxfOtpYkhWzLUm0lqb1UuUIztdQI9t726QMz1ZIIXp0jaqQ"), "name": Object {"type": String("String"), "value": String("Updated Record")}, "value": Object {"type": String("Integer"), "value": Number(100)}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/simple_websocket`
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: "F1cpxxCw_vwIzadFdpe6tOmOcdnsSU20_rYQHJu07ahsWI8XYIwfHxSbKcWsq2Aq6XbHBQXjBJZ9WE7AyIYCZg"

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
        "id": "F1cpxxCw_vwIzadFdpe6tOmOcdnsSU20_rYQHJu07ahsWI8XYIwfHxSbKcWsq2Aq6XbHBQXjBJZ9WE7AyIYCZg",
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/collection_management`
✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: "Q2e2NngfKBI177283r0CN6QbER70WNwvHVyLNlqQ397JkWV9mgEUNLumtduVeFyUvcjjRbWfoyXJZmRpxVcT8w"

=== List Collections ===
Total collections: 4
Sample collections: ["batch_users", "demo_collection", "websocket_test", "test_collection"]

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/document_ttl`
✓ Authentication successful

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: "umZtUMfuWHqLhlId6oz1X72Yoxdg1zCA3ojUOBcd02QFiEuqbY7uFxBPJstZy1NRg73NgnnDzmc44tu-Mtl9-Q"

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: null

=== Query Documents ===
✓ Found 1 documents with TTL

=== Update Document ===
✓ Updated document

=== Delete Document ===
✓ Deleted document

✓ All document TTL operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/websocket_ttl`
✓ Authentication successful

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: String("d43sP9XHHxeX8ykKCzjcGo_XmHtMd3wv-YzT8LCWvaIJ5w1qeKQdIgenD_nGY0856FpW-Ez-HMtl5OmAHDbeRA")

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket

✓ WebSocket TTL example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/http_functions`
🚀 ekoDB Scripts Example (Rust/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: 7xeIApWkBlSvfb7XNGm8GAGeote1fjSF_-5KmNOTqrjCGKrSdqWAf-WFxH9hC--KUZgiWep9Rv1S1IDSTUpQ3g
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: "_SRs6eEZDM1yGOJ9D76vmzuU5RX8-7CzpqCsB4y3fvU7GFFa83DrijYmsfccH0FhdDMVFc-fQojquCzpdzSgBw"
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Complex Filter with Multiple Conditions

✅ Script saved: MJT1GUikVN16WN72vAj7pJeppSzqxbFiCOINVPsEN7nHM6U_avtW6qrPy5LD-n8X1cZ3Ekpi5KKsDLSgYQQ1CA
📊 Found 3 users (status=active, score>50, sorted by score)

📝 Example 4: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: PoGDVCclxdftnlu86y2uRrp7L6BljDLlhq2RmFOPksopM1Op4bxsWTtfDQDhhKEzzw5ALWip9naJYe4xOo-erw
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"avg_score":60.0,"count":5,"max_score":100,"status":"active"}
   {"avg_score":50.0,"count":5,"max_score":90,"status":"inactive"}

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
Created Alice: $1000 - ID: -7Zu3amFVYJWxRO772gTvJviii_N9j6kqCh9JKSIyfYc9iHLRftf_-GZ0ghYEofOGRF7QH0lFiu7oP3xKiuYEA
Created Bob: $500 - ID: opGvJKtGCp8FCD3BXzm2mLFLk8DWLQosXOZsaYvQtjKO7UjqLVZeqRijRejGLoCTyG526EiBZvii33Ez9txowQ

=== Example 1: Begin Transaction ===
Transaction ID: 44b4584b-0290-4db2-94be-c2a575e2b821

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
New transaction: c3aba75d-dad8-4366-b6c4-8dfe8ac454cd
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: 700

=== Cleanup ===
✓ Deleted test accounts

✓ All transaction examples completed
✅ [32mRust direct examples complete![0m
🛠️  [36mBuilding client library...[0m
cargo build -p ekodb_client
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
✅ [32mClient build complete![0m
🧪 [36mRunning Rust client library examples...[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_simple_crud`
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: Record { fields: {"id": String("NPs0_tbmmM10roGF1_3q72B3dzrsz_yAFarP9sot6CMFiJjMmeAIUUz75kskjir58GUXSva5LWa_uX8qHToKiw")} }

=== Find by ID ===
Found: Record { fields: {"id": String("NPs0_tbmmM10roGF1_3q72B3dzrsz_yAFarP9sot6CMFiJjMmeAIUUz75kskjir58GUXSva5LWa_uX8qHToKiw"), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "metadata": Object({"type": String("Object"), "value": Object({"key": String("value"), "nested": Object({"deep": Boolean(true)})})}), "user_id": Object({"value": String("550e8400-e29b-41d4-a716-446655440000"), "type": String("String")}), "categories": Object({"value": Array([String("electronics"), String("computers")]), "type": String("Array")}), "price": Object({"type": String("Float"), "value": Float(99.99)}), "name": Object({"value": String("Test Record"), "type": String("String")}), "tags": Object({"value": Array([String("tag1"), String("tag2"), String("tag3")]), "type": String("Array")}), "data": Object({"type": String("String"), "value": String("aGVsbG8gd29ybGQ=")}), "embedding": Object({"type": String("Array"), "value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)])}), "value": Object({"type": String("Integer"), "value": Integer(42)}), "created_at": Object({"type": String("DateTime"), "value": String("2026-01-02T05:53:31.671637Z")})} }

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Some("Test Record")
  value (Integer): Some(42)
  active (Boolean): Some(true)
  price (Decimal): Some(99.99)
  created_at (DateTime): Some("2026-01-02T05:53:31.671637Z")
  user_id (UUID): Some("550e8400-e29b-41d4-a716-446655440000")
  tags (Array): 3 items
  metadata (Object): 2 keys
  embedding (Vector): 5 dims
  categories (Set): 2 items
  data (Bytes): 11 bytes

=== Find with Query ===
Found documents: [Record { fields: {"metadata": Object({"value": Object({"key": String("value"), "nested": Object({"deep": Boolean(true)})}), "type": String("Object")}), "tags": Object({"type": String("Array"), "value": Array([String("tag1"), String("tag2"), String("tag3")])}), "value": Object({"type": String("Integer"), "value": Integer(42)}), "categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])}), "price": Object({"value": Float(99.99), "type": String("Float")}), "user_id": Object({"type": String("String"), "value": String("550e8400-e29b-41d4-a716-446655440000")}), "created_at": Object({"value": String("2026-01-02T05:53:31.671637Z"), "type": String("DateTime")}), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "embedding": Object({"type": String("Array"), "value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)])}), "id": String("NPs0_tbmmM10roGF1_3q72B3dzrsz_yAFarP9sot6CMFiJjMmeAIUUz75kskjir58GUXSva5LWa_uX8qHToKiw"), "name": Object({"type": String("String"), "value": String("Test Record")}), "data": Object({"type": String("String"), "value": String("aGVsbG8gd29ybGQ=")})} }]

=== Update Document ===
Updated: Record { fields: {"price": Object({"type": String("Float"), "value": Float(99.99)}), "value": Object({"type": String("Integer"), "value": Integer(100)}), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "id": String("NPs0_tbmmM10roGF1_3q72B3dzrsz_yAFarP9sot6CMFiJjMmeAIUUz75kskjir58GUXSva5LWa_uX8qHToKiw"), "tags": Object({"value": Array([String("tag1"), String("tag2"), String("tag3")]), "type": String("Array")}), "metadata": Object({"value": Object({"nested": Object({"deep": Boolean(true)}), "key": String("value")}), "type": String("Object")}), "embedding": Object({"type": String("Array"), "value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)])}), "created_at": Object({"type": String("DateTime"), "value": String("2026-01-02T05:53:31.671637Z")}), "data": Object({"type": String("String"), "value": String("aGVsbG8gd29ybGQ=")}), "user_id": Object({"type": String("String"), "value": String("550e8400-e29b-41d4-a716-446655440000")}), "categories": Object({"value": Array([String("electronics"), String("computers")]), "type": String("Array")}), "name": Object({"type": String("String"), "value": String("Updated Record")})} }

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
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

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_collection_management`
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: "VncX2e1TQNjzaUO3q3LcEabaGzpRykdSAUHQBE79f3lrGDzi40mczekZioHSm1Eruz4zrcIbQR2kiPyv_K29vw"

=== List Collections ===
Total collections: 9
Sample collections: ["batch_users", "scripts__ek0_testing", "client_collection_management_rust", "users", "ttl_cache"]

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
     Running `target/debug/examples/client_document_ttl`
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: "PPTIYIwW0r-1Q2H_RYUctsJXGOjZxhOKqftL9NkbNE5yXDsOkWuORAzP7A5S9oK3Cf39PpRKj3KRMU1chqRZOQ"

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: Some(String("Bchff87ojms8MVgOCGcuQ_eOCK82gZz2RYQS4zra0oum6F1zBwbPEHEF8Qi0XO9ihaon1WnZyF4mGcosOcBTSQ"))

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
     Running `target/debug/examples/client_simple_websocket`
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: sGg81ZaYvBf52be7WO1e94tbjpDUBWkQtfNt4aSROZNZoXFkLgVRpoDkd5CSbQ-_mt9aO_Jgw-LUyZ5KfO7ozA

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_websocket_ttl`
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: Some(String("leDgihkk1yoltjVhGS52WigXAaVe78vvbIhZO881sOHRGf7FQP80T5eecPDh1a_nMfz03Tdvr06Jxx3Ep5xkAg"))

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_query_builder`
=== ekoDB Query Builder Example ===

=== Inserting Sample Data ===
✓ Inserted 5 users

=== Simple Equality Query ===
✓ Found 3 active users
  - Some(Object({"type": String("String"), "value": String("Alice")}))
  - Some(Object({"value": String("Bob"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("David")}))

=== Range Query (age >= 28 AND age < 35) ===
✓ Found 3 users in age range
  - Some(Object({"value": String("David"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("Bob")}))
  - Some(Object({"value": String("Eve"), "type": String("String")}))

=== IN Operator ===
✓ Found 4 users with status active or pending
  - Some(Object({"value": String("Eve"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("David")}))
  - Some(Object({"value": String("Alice"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("Bob")}))

=== NOT IN Operator ===
✓ Found 4 users not inactive

=== String Pattern Matching ===
✓ Found 5 users with @example.com email

=== Regex Query ===
✓ Found 0 users with names starting with A-C

=== Complex Query (active AND age >= 28 AND score > 1500) ===
✓ Found 2 users matching all conditions
  - Some(Object({"value": String("David"), "type": String("String")}))
  - Some(Object({"value": String("Bob"), "type": String("String")}))

=== OR Query ===
✓ Found 2 users with age < 28 OR age > 32
  - Some(Object({"type": String("String"), "value": String("Alice")}))
  - Some(Object({"value": String("Charlie"), "type": String("String")}))

=== Sorted Query (by score descending) ===
✓ Top 3 users by score:
  1. Some(Object({"type": String("String"), "value": String("Bob")}))
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_search`
=== ekoDB Search Example ===

=== Inserting Sample Documents ===
✓ Inserted 5 sample documents

=== Basic Text Search ===
✓ Found 3 results for 'programming'
  1. Score: 19.8000 - Some(String("Rust Programming"))
  2. Score: 6.6000 - Some(String("JavaScript Web Development"))
  3. Score: 6.6000 - Some(String("Python for Data Science"))

=== Fuzzy Search ===
✓ Found 3 results for 'progamming' (typo)
  1. Score: 1.6500 - Some(String("Rust Programming"))
  2. Score: 0.5500 - Some(String("JavaScript Web Development"))
  3. Score: 0.5500 - Some(String("Python for Data Science"))

=== Field-Specific Search ===
✓ Found 4 results in title/description
  1. Score: 4.0000
     Title: Some(String("Machine Learning Basics"))
     Matched: ["description.value", "description", "title", "title.value"]
  2. Score: 2.0000
     Title: Some(String("Python for Data Science"))
     Matched: ["description.value", "description"]
  3. Score: 1.0000
     Title: Some(String("Rust Programming"))
     Matched: ["description.value", "description"]
  4. Score: 1.0000
     Title: Some(String("Database Design"))
     Matched: ["description", "description.value"]

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
  2. Score: 6.6000 - Some(String("JavaScript Web Development"))

=== Cleanup ===
✓ Deleted collection

✓ All search operations completed successfully
Execution time: 2ms
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_schema_management`
=== ekoDB Schema Management Example ===

=== Creating Collection with Schema ===
✓ Created collection 'schema_client_rust' with schema

=== Inserting Valid Documents ===
✓ Inserted user 1: Some(String("1_LEV5MwqAod-fDhkD2cByiqv2WAhtztqRewveQE7r3_zUOvVLzGRkYFVsQdFtXkpMWSGHF7UUoNlHhlBsnxZg"))
✓ Inserted user 2: Some(String("ehcm5CsH3SuqQrLwpWGfb2FG3gTsVAmZqq-BgvuyuYn5HScNdjRpSKIUq0dv3UTMkg-JKsdLSTLvSFL8WyZrDg"))

=== Getting Schema ===
✓ Schema for schema_client_rust:
  - age: Integer
  - email: String
    (required)
  - title: String
    (required)
  - status: String

=== Listing Collections ===
✓ Total collections: 9
  Sample: ["batch_users", "scripts__ek0_testing", "schema_client_rust", "users", "ttl_cache"]

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
  - Laptop ($1200) by Alice Johnson
  - Mouse ($25) by Alice Johnson

=== Example 5: Complex join with multiple conditions ===
✓ Found 2 users with example.com emails
  - Alice Johnson (alice@example.com): Building A
  - Bob Smith (bob@example.com): Building B

=== Cleanup ===
✓ Deleted test collections

✓ Join operations example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_chat_basic`
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: pHzBl65XmiObokt99kagXWBR2XZATZkaDRbAQw9DV4VRVNekmdIShmIRACSrvbykFI1FO3S9t0o13S8KcuTkgw

=== Sending Chat Message ===
Message ID: 0NcdUpSCH1eVr_4lKu7h5jvBOA_vxHN1BmId00dp1JAxdQjC296EvusCsPDuNIoFlwCKtO3-n6j2eUHZQafIFw

=== AI Response ===
Response 1: ekoDB is a high-performance database that includes intelligent caching, real-time capabilities, and AI integration. It comes with various distinctive features. One of these is the AI Chat Integration feature, which allows you to query your database using natural language and get AI-powered responses with relevant context. Another significant aspect of ekoDB is its robust search features. It supports full-text search, vector search, and hybrid search with automatic context retrieval.

=== Context Used (3 snippets) ===

Snippet 1:
  Collection: client_chat_basic_rust
  Score: 0.6333
  Matched Fields: ["category", "title", "content"]
  Record: Object {"category": String("features"), "content": String("The chat feature allows you to query your database using natural language and get AI-powered responses with relevant context."), "id": String("5jtAjeUmod5oGwEQ-MPNQUQaV8bzCEXqK2dLf6OKdYIP6H25E4x7f13hUN1O7et8NkrVTFM9AJ-SfKuMDVm0Dg"), "title": String("AI Chat Integration")}

Snippet 2:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["content", "title"]
  Record: Object {"category": String("documentation"), "content": String("ekoDB is a high-performance database with intelligent caching, real-time capabilities, and AI integration."), "id": String("Rvj4bfjt6smNhwyuHDrShhfCX9FvHY2HVuy3pXKFkWSVUknsoyOB7dAK0g-9mVgO9b13uYrCNiTjw80GhZ2G7Q"), "title": String("Introduction to ekoDB")}

Snippet 3:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["content", "title", "category"]
  Record: Object {"category": String("features"), "content": String("ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval."), "id": String("XEROmcOi_Jmnv0oEaVvCKfkwQQ34PmmCXZHzELJG9UJdEL0QiNggNo_VbqDZ0HftrTXZoAdTp2x1HLdsVCRkFg"), "title": String("Search Features")}

Execution Time: 2700ms

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_chat_sessions`
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: Os4Ty52VQWOwXHxmpTvxMOLnCwAgd-aT5rjepSBeASzohnp0YdUUksWfNNiVq5OzIN-7q6T_MfIkl7GZGmwH4Q
=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information provided, the available product is ekoDB. It's a high-performance database product with AI capabilities. The price of this product is $99.

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: gNxEw7D2DxicR36CxgYyNEQR-t2NQzSOpgjKuGTkLxCQB7ZwkHnAbK85f5isjphXR_8kxz-0uJSKJdwsK0n8yA
  Parent: Os4Ty52VQWOwXHxmpTvxMOLnCwAgd-aT5rjepSBeASzohnp0YdUUksWfNNiVq5OzIN-7q6T_MfIkl7GZGmwH4Q

=== Listing Sessions ===
✓ Found 3 sessions
  Session 1: gNxEw7D2DxicR36CxgYyNEQR-t2NQzSOpgjKuGTkLxCQB7ZwkHnAbK85f5isjphXR_8kxz-0uJSKJdwsK0n8yA (Untitled)
  Session 2: Os4Ty52VQWOwXHxmpTvxMOLnCwAgd-aT5rjepSBeASzohnp0YdUUksWfNNiVq5OzIN-7q6T_MfIkl7GZGmwH4Q (Untitled)
  Session 3: pHzBl65XmiObokt99kagXWBR2XZATZkaDRbAQw9DV4VRVNekmdIShmIRACSrvbykFI1FO3S9t0o13S8KcuTkgw (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: gNxEw7D2DxicR36CxgYyNEQR-t2NQzSOpgjKuGTkLxCQB7ZwkHnAbK85f5isjphXR_8kxz-0uJSKJdwsK0n8yA

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_chat_advanced`
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: YzSaQ6HvYu_6lbUu_peeOF_Eos-_ZRrjYqF8nMWd6kigkTuM-9BuFTiFUNFh7anAoMympQ--xDcR8AONocai1Q

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the context provided, one available product is the ekoDB, which is a high-performance database product. It is priced at $99.

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
✓ Created second session: m0J43kGqILKtUrl5O0HN4ccU2Y0S8UshpcPMxHmfvLO8rxHA6k6TbxC3u0LD39-JDoyHbM80XLYomD3kVphUqQ
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
     Running `target/debug/examples/client_functions`
=== ekoDB Rust Client - Scripts Example ===

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: 9eS7h29hWV5xMBq4MvjdpckZ5i7bKZ49vBeyKTbSdHuIfwoMQq5dl31xoRp3tbYKhJpSbwF4lN1DNrWoE8CkxQ
📊 Found 20 active users

📝 Example 2: Parameterized Script

✅ Script saved: g-fYtFvz4MJoS1pJsOzHeImcvSLJg-soYCot2V4gEvAmTXbu77iU4dE9iWVOBN-EIi05a9p1wS0uIdccUNZUQQ
📊 Found 20 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: Dxq0lwd0_bgR8VwXg5OXjlBMdQQKhFj_Qsyaldyv1S4e-nALPY8Vexj2kUb9hxZOnor3u1k78CDo-R587xzHtg
📊 Statistics: 2 groups

📝 Example 4: Script Management

📋 Total scripts: 6
🔍 Retrieved script: Get Active Users
✏️  Script updated
🗑️  Script deleted

ℹ️  Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.


✅ All examples completed!
✅ [32mRust client examples complete![0m
