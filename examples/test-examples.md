make test-examples
🧪 [36mRunning Rust examples (direct HTTP/WebSocket)...[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/simple_crud`
✓ Authentication successful

=== Insert Document ===
Inserted: Object {"id": String("2CyfaOjZrTQxSKQehS3OSozQipH40i5L81dpTpv50eURMWhA21QMB7yWKna-My3-es67ueEkmwNnZWyR1jCujw")}

=== Find by ID ===
Found: Object {"active": Bool(true), "id": String("2CyfaOjZrTQxSKQehS3OSozQipH40i5L81dpTpv50eURMWhA21QMB7yWKna-My3-es67ueEkmwNnZWyR1jCujw"), "name": String("Test Record"), "value": Number(42)}

=== Find with Query ===
Found documents: Array [Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("2CyfaOjZrTQxSKQehS3OSozQipH40i5L81dpTpv50eURMWhA21QMB7yWKna-My3-es67ueEkmwNnZWyR1jCujw"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}]

=== Update Document ===
Updated: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("2CyfaOjZrTQxSKQehS3OSozQipH40i5L81dpTpv50eURMWhA21QMB7yWKna-My3-es67ueEkmwNnZWyR1jCujw"), "name": Object {"type": String("String"), "value": String("Updated Record")}, "value": Object {"type": String("Integer"), "value": Number(100)}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/simple_websocket`
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: "2ohhdIKZs8t7QdU7ZWUXCc4EPm8T-Iy-RVYfPio_EofPFq65P9o3SaJhKtwl8h8xuajQE9DK3IsJAV0tEM2-dw"

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
        "id": "2ohhdIKZs8t7QdU7ZWUXCc4EPm8T-Iy-RVYfPio_EofPFq65P9o3SaJhKtwl8h8xuajQE9DK3IsJAV0tEM2-dw",
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
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
Collection created with first record: "dnXS33LvNlqEucn8efW052KedVJKRC9qxSb6QThdkVs7OoswUYf6NM0sM91A0Hyr95QiKK4VfQf0e4RK_ggtzw"

=== List Collections ===
Total collections: 4
Sample collections: ["test_collection", "demo_collection", "batch_users", "websocket_test"]

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
✓ Inserted document: "-S_8YJlDHzZbRiuR1JB9UajzX43BzpT3dAno2rNr_cwhNWJeqPT8YVASJV1RWSYQh6jf3KKCtzqSudQefy28rg"

=== Insert Document with TTL (5 minutes - integer) ===
✓ Inserted document: "WKIfLA69SCTXMnLGCc1wv_NAQ_zpC4qRy_XXt1yqXSGo_u0pcFLN-npOzx-C9ZFPljnD1ylsVKSXX9hCsMUFXg"

=== Insert Document with TTL (30 minutes - duration string) ===
✓ Inserted document with duration string TTL: "oTqgeDZ-pajWTApqXqxl6_cRvdKKq23fKn2XV4eUa51XN32h7bTsVIUv0MbW_rkaH1tSMPS8Bh7aMY-u4eMCEA"

=== Query Documents ===
✓ Found 3 documents with TTL

=== Update Document ===
✓ Updated document

=== Delete Document ===
✓ Deleted document

✓ All document TTL operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/websocket_ttl`
✓ Authentication successful

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: String("n3pstWqS82glQusjCjuI2NGjF3xIkDwIdP4VGWBjbv4ymcgSzFClcsfCS89RPgsDF17KereMmqZpXlPxN922bw")

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket

Record 1:
  id: "n3pstWqS82glQusjCjuI2NGjF3xIkDwIdP4VGWBjbv4ymcgSzFClcsfCS89RPgsDF17KereMmqZpXlPxN922bw"
  name: {"type":"String","value":"WebSocket TTL Test"}
  ttl: "2026-01-26T08:59:10.734361Z"
  value: {"type":"Integer","value":42}

✓ WebSocket TTL example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/http_functions`
🚀 ekoDB Scripts Example (Rust/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: t5U9VnueRu3JGbh2ueYEXMhRcCtBWB-CjQBy4N6lRapSFW1ZLx5e-Irv44LY4b4tfjBrNUX2HZAEWMAsVOD2bA
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: "98Z5FtfrbpOrberj_rdGb2zylhq3lAVLQAzGjUXNeqqAPl-nTCyRmEm0k1-0_FJMbpGFNGCN5vIii00CzK2c3g"
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Complex Filter with Multiple Conditions

✅ Script saved: 2JUSq7JttH8VngF1RcZWRdVz5oKqQfUYiLOUCuVMHTjLD2226sljuE7tvswv6mK-XzMQcOeNsqCABb7rnC7rtA
📊 Found 3 users (status=active, score>50, sorted by score)

📝 Example 4: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: Djbs2gOr0pR2g2oBfi278demHxL31c9pQRATqFnwue20AQmHlLEwTBXjQQUACto3napkoaxayyg63eoQC4-B1w
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
Created Alice: $1000 - ID: HkJkRNByO2IA8Ooj3FLlHoSmh2quMo_RmmIxi7CIxAEqa5HMfZEpSMo_-TtRud0ZqhIDlnL7rydmmQcWPcDgag
Created Bob: $500 - ID: 2VYsNgQeFBast8D7t2p5ranit0mlFKRSeksV20Q1WRGpNVLik60Wm6WP2L0pwwkMLY1NR3xnYRDpVD9tkQnFOw

=== Example 1: Begin Transaction ===
Transaction ID: 3154e1cf-735e-443f-a7b7-5c360bdd5a03

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
New transaction: 2ef7325b-a1b5-4f27-8e2c-06755b4fc1ea
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
Inserted: Record({"id": String("gOc5E1jGjzU5brMGzRva05GhoHgFmGbt2DbCo8M8Etr1ql2MV_5_N_i0xNH8IWSFdxeF5kZ35IE2uQa1dPi0cA")})

=== Find by ID ===
Found: Record({"embedding": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "categories": Array([String("electronics"), String("computers")]), "active": Boolean(true), "metadata": Object({"nested": Object({"deep": Boolean(true)}), "key": String("value")}), "created_at": String("2026-01-26T07:59:15.952843+00:00"), "price": Float(99.99), "tags": Array([String("tag1"), String("tag2"), String("tag3")]), "user_id": String("550e8400-e29b-41d4-a716-446655440000"), "name": String("Test Record"), "value": Integer(42), "data": String("aGVsbG8gd29ybGQ="), "id": String("gOc5E1jGjzU5brMGzRva05GhoHgFmGbt2DbCo8M8Etr1ql2MV_5_N_i0xNH8IWSFdxeF5kZ35IE2uQa1dPi0cA")})

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Some("Test Record")
  value (Integer): Some(42)
  active (Boolean): Some(true)
  price (Decimal): Some(99.99)
  created_at (DateTime): Some("2026-01-26T07:59:15.952843+00:00")
  user_id (UUID): Some("550e8400-e29b-41d4-a716-446655440000")
  tags (Array): 3 items
  metadata (Object): 2 keys
  embedding (Vector): 0 dims
  categories (Set): 0 items
  data (Bytes): 11 bytes

=== Find with Query ===
Found documents: [Record({"user_id": Object({"value": String("550e8400-e29b-41d4-a716-446655440000"), "type": String("String")}), "price": Object({"type": String("Float"), "value": Float(99.99)}), "value": Object({"type": String("Integer"), "value": Integer(42)}), "id": String("gOc5E1jGjzU5brMGzRva05GhoHgFmGbt2DbCo8M8Etr1ql2MV_5_N_i0xNH8IWSFdxeF5kZ35IE2uQa1dPi0cA"), "embedding": Object({"value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "type": String("Array")}), "name": Object({"type": String("String"), "value": String("Test Record")}), "metadata": Object({"type": String("Object"), "value": Object({"key": String("value"), "nested": Object({"deep": Boolean(true)})})}), "tags": Object({"type": String("Array"), "value": Array([String("tag1"), String("tag2"), String("tag3")])}), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "created_at": Object({"type": String("DateTime"), "value": String("2026-01-26T07:59:15.952843Z")}), "data": Object({"type": String("String"), "value": String("aGVsbG8gd29ybGQ=")}), "categories": Object({"value": Array([String("electronics"), String("computers")]), "type": String("Array")})})]

=== Update Document ===
Updated: Record({"metadata": Object({"value": Object({"key": String("value"), "nested": Object({"deep": Boolean(true)})}), "type": String("Object")}), "tags": Object({"value": Array([String("tag1"), String("tag2"), String("tag3")]), "type": String("Array")}), "active": Object({"type": String("Boolean"), "value": Boolean(true)}), "id": String("gOc5E1jGjzU5brMGzRva05GhoHgFmGbt2DbCo8M8Etr1ql2MV_5_N_i0xNH8IWSFdxeF5kZ35IE2uQa1dPi0cA"), "name": Object({"value": String("Updated Record"), "type": String("String")}), "price": Object({"value": Float(99.99), "type": String("Float")}), "categories": Object({"value": Array([String("electronics"), String("computers")]), "type": String("Array")}), "embedding": Object({"value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "type": String("Array")}), "user_id": Object({"type": String("String"), "value": String("550e8400-e29b-41d4-a716-446655440000")}), "data": Object({"value": String("aGVsbG8gd29ybGQ="), "type": String("String")}), "created_at": Object({"value": String("2026-01-26T07:59:15.952843Z"), "type": String("DateTime")}), "value": Object({"type": String("Integer"), "value": Integer(100)})})

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_simple_websocket`
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: Tf_TMDvmVUJpeRWiifqpY3keu5dYoM-9xTXa4TY6-4oFlT6IPTtNOyQJ8nhv2RbNvhmy9tSOXh0ylWaXCsVvoQ

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket

Record 1:
  id: "Tf_TMDvmVUJpeRWiifqpY3keu5dYoM-9xTXa4TY6-4oFlT6IPTtNOyQJ8nhv2RbNvhmy9tSOXh0ylWaXCsVvoQ"
  name: "WebSocket Test Record"
  active: true
  value: 42

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
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
Collection created with first record: "A0jMxr0GIkHuCQj-QkadVzOyriY4LY5yWob4m2XJDJlXsf9MBPzCQPqLKzMPhaek_TeMjrxvTmDoSOC7iBfCkQ"

=== List Collections ===
Total collections: 9
Sample collections: ["users", "ttl_cache", "test_collection", "scripts__ek0_testing", "test_accounts"]

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

=== KV Batch Set ===
✓ Batch set 3 keys
  cache:product:1: success
  cache:product:2: success
  cache:product:3: success

=== KV Batch Get ===
✓ Batch retrieved 3 values
  cache:product:1: Record({"name": String("Product 1"), "price": Float(29.99)})
  cache:product:2: Record({"price": Float(39.989999999999995), "name": String("Product 2")})
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_transactions`
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: yTyRteKy0DQFkUvTJbo_PXmeccd6SSX7h3mEXne3qO5YJIEGXybTLAJP4VdrYUJTp6kvrf7NLGycEU_6ZXB19A
Created Bob: $500 - ID: NYTvlRUQAneNyAOEqPcVoiHb1lRFtwU-wx9324uxQyWvKHDx5AEPZWTJBwAcy-B3ipg1EhRae_GCVfJQHLlm2A

=== Example 1: Begin Transaction ===
Transaction ID: a40c3e94-202d-417f-911e-a1f1a5bf9835

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: "Active"
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: a8357032-7fc9-4d16-9862-5677afb1f4d8
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
  - Some(Object({"value": String("Bob"), "type": String("String")}))
  - Some(Object({"value": String("David"), "type": String("String")}))
  - Some(Object({"value": String("Alice"), "type": String("String")}))

=== Range Query (age >= 28 AND age < 35) ===
✓ Found 3 users in age range
  - Some(Object({"value": String("Bob"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("David")}))
  - Some(Object({"value": String("Eve"), "type": String("String")}))

=== IN Operator ===
✓ Found 4 users with status active or pending
  - Some(Object({"value": String("Bob"), "type": String("String")}))
  - Some(Object({"value": String("David"), "type": String("String")}))
  - Some(Object({"value": String("Eve"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("Alice")}))

=== NOT IN Operator ===
✓ Found 4 users not inactive

=== String Pattern Matching ===
✓ Found 5 users with @example.com email

=== Regex Query ===
✓ Found 0 users with names starting with A-C

=== Complex Query (active AND age >= 28 AND score > 1500) ===
✓ Found 2 users matching all conditions
  - Some(Object({"value": String("Bob"), "type": String("String")}))
  - Some(Object({"value": String("David"), "type": String("String")}))

=== OR Query ===
✓ Found 2 users with age < 28 OR age > 32
  - Some(Object({"type": String("String"), "value": String("Charlie")}))
  - Some(Object({"type": String("String"), "value": String("Alice")}))

=== Sorted Query (by score descending) ===
✓ Top 3 users by score:
  1. Some(Object({"value": String("Bob"), "type": String("String")}))
  2. Some(Object({"value": String("David"), "type": String("String")}))
  3. Some(Object({"value": String("Alice"), "type": String("String")}))

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
     Matched: ["description", "description.value", "title", "title.value"]
  2. Score: 2.0000
     Title: Some(String("Python for Data Science"))
     Matched: ["description", "description.value"]
  3. Score: 1.0000
     Title: Some(String("Rust Programming"))
     Matched: ["description", "description.value"]
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.30s
     Running `target/debug/examples/client_schema`
=== ekoDB Schema Management Example ===

=== Creating Collection with Schema ===
✓ Created collection 'schema_client_rust' with schema

=== Inserting Valid Documents ===
✓ Inserted user 1: Some(String("JFgkAnnEhtrckURmViRroyYqkt5USsictrv2k-e4eESEK_5cRNTjoXwA2i2F-qUqEXoklEnfvmGk18oB90XM5w"))
✓ Inserted user 2: Some(String("w6CeYxQF__tVvQHHA1Eo2lcXn4btAV44NOleQ_k5QEMGt0vuyFUGre4ncfLc3q75aB0M7SSxGkhH3f6Nwf5pqw"))

=== Getting Schema ===
✓ Schema for schema_client_rust:
  - title: String
    (required)
  - status: String
  - email: String
    (required)
  - age: Integer

=== Listing Collections ===
✓ Total collections: 9
  Sample: ["users", "ttl_cache", "test_collection", "scripts__ek0_testing", "test_accounts"]

=== Cleanup ===
✓ Deleted collection

✓ All schema management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.66s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.35s
     Running `target/debug/examples/client_document_ttl`
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: "tWHpelqxRL6Jkyq_jVGHspQwYzEcf22axsCQmkyvR5F6rIDoXWafU0-gkA-gvO3oFOexyTs-TmFeCBz0rGvTFw"

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: Some(String("HHkku0ShXsBDOzfvisn_S0uRynMx6hlGxFstgvBphH7m-GYF4JmQELciRNFa0G1qcz_vBidiUyo-4n6Ofd23PA"))

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
✓ Inserted document with TTL: Some(String("e7BT4VI2akFq_di1yRyFDzRFflA3_p-wvDOB3rHgdZQ7s9v1Jd5OnNey7hXhjgAH19pl1hewWVnZ_cMkVZi7uw"))

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_edge_cache`
=== ekoDB as Edge Cache - Simple Example ===

Setting up edge cache collection...
✓ Cache entry created

Creating edge cache lookup script...
✓ Edge cache script created: mgURVr5REmLXfS09wk94AuKwmDphvzjHqtlKCsZMuUCiV7rD5_3bkEOn6JR65MDaRX1gAh8RoWe7hyZOWRfNwA

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_functions`
=== ekoDB Rust Client - Scripts Example ===

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: 0GsR5k_xL0yn-sUXH3sINyxx8ATIgLET_-A76Z3tZpxBEGJzluFcTptpozESCFb7U7dDGEecfsYIar0_KJEEkg
📊 Found 20 active users

📝 Example 2: Parameterized Script

✅ Script saved: _NaMVmCrPGoGCeHxA-rZDN0QZs_D2jeBqBwtQ9SCwytvTkZiRoVmZGl5GKPSxay9Bg4UN-3y4RX8ycdQIChtpA
📊 Found 20 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: 4-ZRumIIJWH9a2pJCHDVnXBYVBrKmOC7rXNqzMHAt7LdkroeKkpjbXiaiceq47G_dCdcyKOprIe0fqcIxAdi1A
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

✅ Saved reusable function: fetch_and_store_user
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 131.872875ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 69.901959ms
   📊 Records: 1
   🚀 Cache speedup: 1.9x faster!

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/client_functions_complete`
🚀 ekoDB Rust Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved: daH-UjThRmOkiuwwF9VoExbejUhsI07UiShybkQj_of1iNv6rPS4dKRHhCH_oicH2KzMnB-VAwqTik5KDIRSwA
📊 Found 2 product groups
   Record({"category": String("Electronics"), "count": Integer(3), "avg_price": Float(575.6666666666666)})
   Record({"category": String("Furniture"), "count": Integer(2), "avg_price": Float(474.0)})
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved
📊 Found 2 categories
   Record({"count": Integer(2), "category": String("Furniture")})
   Record({"category": String("Electronics"), "count": Integer(3)})
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_functions_kv_wrapped`
🚀 ekoDB Rust KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: Some(String("tItCRqBVUHqckfUNo_hcws1GA0DBM57em30rNFjICGEm6xxSKEDqKDKVj2iSARDFe9bnqdS700tnwPVpd738yw"))
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: gvRgHyGsMQkCcmygMTcUZUCoOfpDSb07X8l_mZmFiMI9lwYEGafjFBOyHbmny6O1jCrwGCa0E1NxIrGiDwlxJw
📊 Script executed
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: Some(Object {"role": String("admin"), "userId": String("user_abc")})
🔍 Key exists: true
✅ Set cached data
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: S4hbzAvo5rt8EYXf7QZ8k0TI5xoCklEiEZYqvkfQWc6MruvOAjIz534ywsEBakNxbW0BxQuDj5GJ6UOyiOczZQ
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Script

✅ Script saved: zc_CoUssgZydZPv4nDYKE6AtKeK06gP2UYmW4vFJHDnj1ybMHtNwIROOUbrOKzR9QN8AfarMN5HQ_3BRBLEeww
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
✓ Created SWR script: fetch_api_user_rs (DnTlQtM9vSS-u-wTCn7Q0AJi2l-CCH4Tla4XF8oMoxibSBppDtShu_lyXKa9Ns7MD64cHCeGHYs4JhQ47DbfOA)

Step 2: First call - Cache miss, fetches from API
Result: FunctionStats { input_count: 0, output_count: 1, execution_time_ms: 0, stages_executed: 2, stage_stats: [] }
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 63ms (served from cache)
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

✓ Created native SWR script: github_user_native (1GWWNa77fRC_-LVTeHIxa44t-wgQmMEImzZj11xdfsxRDwN1WHp9ObCAZRnPD5NNF88gZE5wnjxlRhMwhP5c4w)

First call (cache miss - will fetch from GitHub API):
  Response time: 120ms
  Records returned: 1
  ✓ Data fetched from API and cached with 15m TTL

Second call (cache hit - instant from KV store):
  Response time: 5ms
  Speedup: 24.0x faster
  ✓ Lightning fast cache hit


Example 2: SWR with Audit Trail Collection
─────────────────────────────────────────────────────

✓ Created SWR script with audit trail: product_swr_audit (vCTG1Ijw0wwFNCs-4sPzhNtPKbdbe1j_CmuBqH9-zl7lJvjJ3l8Jh-HYmS8r2thquJ8iZ4b3RvSaDrtw_8Yz-A)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
─────────────────────────────────────────────────────

Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (0SrP_oa0UYyJG3cbaKvmSxNMGmLQsLDskLGL2IdvSs6b0afWkmSbWEr0B9mA22VfnqvdgBAntCx41GEaAlTc1w)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
─────────────────────────────────────────────────────

✓ Created dynamic TTL script: flexible_cache (BB_ml1fL2Edtwz0k26RosrMa1SbkspiH3aVekzEu7hpbvD-mZpjutg812fDFQL8K5xWPxsraqE7uA4nB3HGoVA)
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

    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
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
   Record({"count": Integer(3), "avg_price": Float(365.6666666666667), "category": String("Furniture")})
   Record({"avg_price": Float(367.0), "count": Integer(5), "category": String("Electronics")})
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced script examples finished!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_functions_ai`
🚀 ekoDB Rust AI Scripts Example

📋 Setting up test data...
✅ Created 2 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
🤖 AI Response:
   Vector databases offer several benefits such as:

1. High precision: Vector data is more precise as it uses points, lines, and polygons to represent real-world objects.
2. Scalability: Vector data maintains its quality when zoomed in or out.
3. Attribute storage: They can store attribute information like name, type, etc., along with geometric data.
4. Less disk space: Vector databases require less disk space compared to raster databases.
5. Complex analysis: They can support complex spatial operations and analysis.
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
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
   Record({"count": Integer(3), "status": String("inactive")})
   Record({"count": Integer(7), "status": String("active")})
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD script examples finished!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_functions_search`
🚀 ekoDB Rust Search Scripts Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Script saved
📊 Found 5 documents
   1. Getting Started with ekoDB (Database)
   2. Introduction to Machine Learning (AI)
   3. Database Design Principles (Database)
   4. Natural Language Processing (AI)
   5. Vector Databases Explained (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Script saved
📊 Documents by category:
   Record({"category": String("Database"), "count": Integer(3)})
   Record({"category": String("AI"), "count": Integer(2)})
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_chat_basic`
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: fHsXg70v83-LYbtuzLXIlj7wO9HrxMk8LNNXdJIlO0x3bzdjAUXMoATyoWAIe7npLkz5Docj-wiZlS0gODtFgw

=== Sending Chat Message ===
Message ID: XYUteUTdQU2eDHdsuTtE3j28vGJgrH9qgofgiV1bNTYVnnnyERq5A3JVxDPljrk5ItAwIQE3YT8RLi5JDR0XtA

=== AI Response ===
Response 1: ekoDB is a high-performance database that comes with intelligent caching, real-time capabilities, and AI integration. It offers several features aimed at enhancing user experience and performance. 

One of the key features of ekoDB is its AI Chat Integration. This feature allows you to query your database using natural language. You can ask questions or request data in a conversational manner and get AI-powered responses with relevant context.

In addition to this, ekoDB also supports various types of searches. It offers full-text search, vector search, and hybrid search, all of which come with automatic context retrieval. This ensures that you can find precisely what you're looking for in your database quickly and efficiently.

=== Context Used (3 snippets) ===

Snippet 1:
  Collection: client_chat_basic_rust
  Score: 0.6333
  Matched Fields: ["category", "title", "content"]
  Record: Object {"category": String("features"), "content": String("The chat feature allows you to query your database using natural language and get AI-powered responses with relevant context."), "id": String("eYbYSqIya04VBqX7p1nU9Bs8-FehaLyMmvqNPoZMQRUJD9Lf3-7TDloY8TU0Rf93L7qAxIwPiNxBpOlmymN3iw"), "title": String("AI Chat Integration")}

Snippet 2:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["title", "content"]
  Record: Object {"category": String("documentation"), "content": String("ekoDB is a high-performance database with intelligent caching, real-time capabilities, and AI integration."), "id": String("myHo_67lhrfvZIKuWDOvCP2YQx9TzKZ9kXoKG3EPdeA_P4MHFIzHjnakodZZDnAMB87XwZ_2YvNIaG6Cbusurw"), "title": String("Introduction to ekoDB")}

Snippet 3:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["category", "title", "content"]
  Record: Object {"category": String("features"), "content": String("ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval."), "id": String("IwGwwgIlOReuwC13GXYgRRju5fNrgJvLI9ct7-nFUuitCRycH3QXhsS5RDUm2_m7MBiaOidGnp51AHyoadFA9w"), "title": String("Search Features")}

Execution Time: 5690ms

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_chat_advanced`
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: Gxyz8Hq6W44xC8BFTCWM4AADrJW382F16IvEhKBqMFiDuTXSJ6llYnPwflBcix39-do1w5YCQw-ZPzFzQ9WeXw

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the context provided, there is a high-performance database product available named "ekoDB". It is priced at $99.

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
✓ Created second session: FgQNl1wmCTOX8XWaikjWKnJh5jz1vLej2PnipxoS35AnUh8n2sqqRoULUiv_bYmC1vk9OP-5AowIoQaNZhkzmw
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
✓ Created session: txXTU787YDCmI_8Mu2xbUdXH1CuPDrTSreIwj10JqQxeQfx-c4ezBzvpIRsHlLBxNzJMQCuJkifsxvald9nfiw
=== Sending Messages ===
✓ Message 1 sent
  Response: The product available is ekoDB, a high-performance database product with AI capabilities. The price of this product is 99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product with AI capabilities, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: MhcOlZ2RvWSL2UE_ObrtyjRekNhMD4vkX3dy_pXykLXdzjStrRysZd4dDr7ZQFvZxO3k3GLYw252UQTG0PNNkA
  Parent: txXTU787YDCmI_8Mu2xbUdXH1CuPDrTSreIwj10JqQxeQfx-c4ezBzvpIRsHlLBxNzJMQCuJkifsxvald9nfiw

=== Listing Sessions ===
✓ Found 4 sessions
  Session 1: MhcOlZ2RvWSL2UE_ObrtyjRekNhMD4vkX3dy_pXykLXdzjStrRysZd4dDr7ZQFvZxO3k3GLYw252UQTG0PNNkA (Untitled)
  Session 2: txXTU787YDCmI_8Mu2xbUdXH1CuPDrTSreIwj10JqQxeQfx-c4ezBzvpIRsHlLBxNzJMQCuJkifsxvald9nfiw (Untitled)
  Session 3: FgQNl1wmCTOX8XWaikjWKnJh5jz1vLej2PnipxoS35AnUh8n2sqqRoULUiv_bYmC1vk9OP-5AowIoQaNZhkzmw (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: MhcOlZ2RvWSL2UE_ObrtyjRekNhMD4vkX3dy_pXykLXdzjStrRysZd4dDr7ZQFvZxO3k3GLYw252UQTG0PNNkA

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_convenience_methods`
=== ekoDB Convenience Methods Example ===

=== Record Builder Pattern ===
✓ Created record with fluent builder: Record({"id": String("PiKp5l3uR7uCgargfJX6HWBK_jCLCIOMWUm0x_IOtOm5Q8n5434tOOoELMLL1P2kJibD5qdDHvpc4OYOXPjNYg")})

=== Upsert Operation ===
✓ First upsert (insert): Record({"id": String("PiKp5l3uR7uCgargfJX6HWBK_jCLCIOMWUm0x_IOtOm5Q8n5434tOOoELMLL1P2kJibD5qdDHvpc4OYOXPjNYg"), "name": Object({"type": String("String"), "value": String("Bob Smith")}), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "email": Object({"type": String("String"), "value": String("bob@example.com")}), "age": Object({"value": Integer(35), "type": String("Integer")})})
✓ Second upsert (update): Record({"age": Object({"value": Integer(36), "type": String("Integer")}), "id": String("PiKp5l3uR7uCgargfJX6HWBK_jCLCIOMWUm0x_IOtOm5Q8n5434tOOoELMLL1P2kJibD5qdDHvpc4OYOXPjNYg"), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "name": Object({"type": String("String"), "value": String("Bob Smith")}), "email": Object({"value": String("bob.smith@newdomain.com"), "type": String("String")})})

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/bypass_ripple_example`
=== Bypass Ripple Example ===

1. Basic insert (ripple enabled):
   Inserted with ripple: Record({"id": String("kwBg1veQNosGT9YWxZu50CcKjKllbl_HerFotvFltc8E4qAnfr8V7agqIHO_HJkxIXJM2V7p4fbz0r4HPRrhgg")})

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: Record({"id": String("eC6hT4SMPGYVy4_72VZMDbYztxHbQHCYUO-YMBecBE-AZRWWdWBaphzXoVgTtL19TdsZpM_cHjAG9wljTHBQdQ")})

3. Update with bypass_ripple:
   Updated with bypass_ripple: Record({"id": String("kwBg1veQNosGT9YWxZu50CcKjKllbl_HerFotvFltc8E4qAnfr8V7agqIHO_HJkxIXJM2V7p4fbz0r4HPRrhgg"), "name": Object({"value": String("Product 1"), "type": String("String")}), "price": Object({"value": Integer(150), "type": String("Integer")})})

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: Record({"id": String("fUfK7NQPNmcD_gsJcrTz4dethJX0VW45RX-QclT2PzX_1qAKF2XrRYgA7dFLQS4CbgcH8_bP_-5emrCeGoSucw")})

✅ All bypass_ripple operations completed successfully!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/projection_example`
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
Fetched user profile: N/A

Example 5: Compare full vs projected data
Full query returned 12 fields per user
Projected query returned 12 fields per user

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
✅ [32mRust client examples complete![0m
✅ [32mAll Rust integration tests complete![0m
🧪 [36mRunning Python examples (direct HTTP/WebSocket)...[0m
=== Simple CRUD Operations (Direct HTTP) ===

✓ Authentication successful

=== Insert Document ===
Inserted: {'id': 'ACFxHhedVNMCt4mje0-aUM359sNnaBz-T4jUHccwkkxmLgcwlzBk3HEAjG6FuQ2bJhIF2eYFGyVRc2lwu6ZmKA'}

=== Find by ID ===
Found: {'name': 'Test Record', 'active': True, 'id': 'ACFxHhedVNMCt4mje0-aUM359sNnaBz-T4jUHccwkkxmLgcwlzBk3HEAjG6FuQ2bJhIF2eYFGyVRc2lwu6ZmKA', 'value': 42}

=== Find with Query ===
Found documents: [{'name': {'type': 'String', 'value': 'Test Record'}, 'id': 'ACFxHhedVNMCt4mje0-aUM359sNnaBz-T4jUHccwkkxmLgcwlzBk3HEAjG6FuQ2bJhIF2eYFGyVRc2lwu6ZmKA', 'active': {'value': True, 'type': 'Boolean'}, 'value': {'value': 42, 'type': 'Integer'}}]

=== Update Document ===
Updated: {'name': {'value': 'Updated Record', 'type': 'String'}, 'id': 'ACFxHhedVNMCt4mje0-aUM359sNnaBz-T4jUHccwkkxmLgcwlzBk3HEAjG6FuQ2bJhIF2eYFGyVRc2lwu6ZmKA', 'value': {'type': 'Integer', 'value': 100}, 'active': {'value': True, 'type': 'Boolean'}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: V-GwO5brmP15gkZCWnGtp4o8ngZYVpyshElKjClatQ0lIDPngkQSMEFFFJNE6mPPBwj0_pYxOs-mnpoye8qKTw

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
        "id": "2ohhdIKZs8t7QdU7ZWUXCc4EPm8T-Iy-RVYfPio_EofPFq65P9o3SaJhKtwl8h8xuajQE9DK3IsJAV0tEM2-dw",
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
        "id": "V-GwO5brmP15gkZCWnGtp4o8ngZYVpyshElKjClatQ0lIDPngkQSMEFFFJNE6mPPBwj0_pYxOs-mnpoye8qKTw",
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
✓ Retrieved 2 record(s) via WebSocket

✓ WebSocket example completed successfully
🚀 ekoDB Scripts Example (Python/HTTP)

✓ Authentication successful

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: R2vVTgh-4iqBZ7I1zuxVWXCZJ5sfiKUrDJ-hN7cNVUfKrRDwCYdia8X1D24BZUPyYvf5SwTxOq6sN1c9xRBv5w
📊 Found 15 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: G5qzMg0dp-hby7b2lqUPR3Qp3rKxctMeTcC1iaeV6G0I5vfDHzHN_kMzYTiStHvbzQJ2M6ZB9TmaJ0_O9wywAQ
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: J5mzNE9JNtAo5ZA9TJX5XKBQ3egfqhGSE_hVDPw4DDyL2gYzEV1ieySMLuUZF2O8ORDKfTE-r1pAYO-lmGY3KA
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {'max_score': 100, 'status': 'active', 'count': 15, 'avg_score': 60.0}
   {'avg_score': 50.0, 'count': 15, 'status': 'inactive', 'max_score': 90}

📝 Example 4: Function Management

📋 Total scripts: 15
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
Retrieved value: {'username': 'john_doe', 'userId': 123}

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: {'price': 29.99, 'name': 'Product 1'}
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
Collection created with first record: Qyg7jJrMMqEgEkT0AXLs_Ji2qVkGJY2I9kO3KG37kAWrESmNsOSYFWkE-Geurrm2vvJBgB7CTu8RKkuo27CKvg

=== List Collections ===
Total collections: 13
Sample collections: ['users', 'ttl_cache', 'test_collection', 'user_cache', 'scripts__ek0_testing']

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
Created Alice: $1000 - ID: jS-GeqSaA5GSvlR73GW3COKu7cpG2vPH6uFm0bztnyr8YurwbY8isqlJcC9AO_Hw6P5GSMcZ-zJH_D305roEVQ
Created Bob: $500 - ID: rCJK5JKvW15r_w20diEuKaOuu92GdSqcylM5iaU-2kfGFtHv6i1PQVkqsJx5lQYTgU_1pq9f2O-GbxYxT_KdUA

=== Example 1: Begin Transaction ===
Transaction ID: 430e776d-6d6f-4d3e-bf16-abb870f1ac5a

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: $800
Bob: $700

=== Example 5: Rollback ===
New transaction: 4aae93b0-b359-47ba-a04f-d0bd217074a9
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: t1Vc6ibuzi0uoLBupd92jxJVdYPO-OZfHweNe8B9D6UkOZE2NtdM_zk7fq_eonK56eWR1QV2zg29KMvVOrfTcw
Created Bob: $500 - ID: n9H61Xkz4uxwxTccXDY4ulgk4nhXkOGOhk_l0XWU13-8vmmRWm_uN41Jer0eTMBhbs3kzJiV_3MGea4fWYRh1Q

=== Example 1: Begin Transaction ===
Transaction ID: 58bf7f99-8bc1-486d-96af-61378cf016aa

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: $800
Bob: $700

=== Cleanup ===
✓ Deleted test accounts


🔄 Switching to NON-DURABLE mode...
   ✓ Config updated: durable_operations=false


============================================================
🔥 TEST 2: Non-Durable Mode (durable=false)
============================================================

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 1sZXMfWNmxXbCj_c5uY5xKsiEsy9isnoPwNq2qN9lEkmccYUPwvzOEI3BEZq6Kfln1qhIg_VjU0l52tLQQbaiQ
Created Bob: $500 - ID: 1CFefpXIPuvvpjhJX7bIsVCzA9MpP7d7wEJC7igA7GFs12IMo6IjbNFlvLP-ZlVrquV844H_pf9dAMzvbqCSnA

=== Example 1: Begin Transaction ===
Transaction ID: f22404e2-012e-4a8f-b320-abd9d3c497a1

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: $800
Bob: $700

=== Example 5: Rollback ===
New transaction: 3739fb95-dd12-4359-8f4d-d11d41f19808
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: puD6-ddeWqtwfeZh_yoWMBfFDH2DyiSU_dxxUv0QoBIjeZmTboNusS7hjWSKUljg1eA7K29BNZY_-LYCmUJtmQ
Created Bob: $500 - ID: lSbaIVulybUhfX2Whm1w23uO_-8GYBb06mr0woGmeLSwONUYAxfWKAAGzkaIcp_sTeqzhjEyEsIAPSuf59_sTw

=== Example 1: Begin Transaction ===
Transaction ID: 5d4a3902-d249-403d-9375-c6659fe5a14e

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: $800
Bob: $700

=== Cleanup ===
✓ Deleted test accounts


🔄 Switching to DURABLE mode...
   ✓ Config updated: durable_operations=true


============================================================
🔥 TEST 3: Durable Mode (durable=true)
============================================================

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: O55mer6AjOMIZUNDfznjC6b-msIvPoRKI6u9-PuXDIfo7v1NAWFZ-BFtLDMy5M6t0_Guke96o5t17zum5dCtPA
Created Bob: $500 - ID: GjT96ds9GVpXLbbuQyZwc40nIXLum8dhxXTtBvJ8aFiCyy0P_MXaFoHmee9OGa8kBS4n3ME4jwCEtPivXwVeSw

=== Example 1: Begin Transaction ===
Transaction ID: 173d70fc-7e2e-473e-a676-c38afde904d1

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 3: Transaction Status ===
Status: Active
Operations: 2

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: $800
Bob: $700

=== Example 5: Rollback ===
New transaction: 2f28daa5-92b5-40a1-b0ed-5f66785f0d27
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: JushUXofzvpPoPxDYG7sP1M1ZZJHPg5WWICe-5_y4s8t06wy-r-xoXAG0znz63zx9FO2gcW3tqJJCThbIK3EGg
Created Bob: $500 - ID: j6hhV_WEni5EQPQ9GM2B1MRDDOyQ-HyD9iMCJAFiTiLgEAngcUUEAF7V_qFToevbQKdY0jKshRfFEKuRfR9GhQ

=== Example 1: Begin Transaction ===
Transaction ID: a708b7f4-ded1-4588-92f1-23020ba5021e

=== Example 2: Operations with transaction_id ===
Updated Alice: $1000 → $800
Updated Bob: $500 → $700

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Verification ===
Alice: $800
Bob: $700

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
   ✅ Script saved: xI5rIpR_Py1pNLaBTS6rJe_qWzUv3xwx6A_oe_HPXinAQpOTjpkaA_0SXUbooIn6tSaGQKAPiLi9NgO-Rep10A

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: rkyx_dQZr8gF1rs2LTvZuNLKQ0x-DeGNfsEfGztjogTlLowzjjwD-i6ec4Ekli3QU1q8ZcaggpafGj49D5st9A
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}
   📋 Email: {'value': 'alice@example.com', 'type': 'String'}
   📋 Status: {'type': 'String', 'value': 'pending'}
   📋 Credits: {'value': 0, 'type': 'Integer'}

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: j9roLMKNsSxJd5JqqsKie5voxq4fSH-sNBGuJKw4PLVCZf4QZZ2KvjvqYOgtWtFl-JLRjK5Ht60Sq_DsShAgkA

2️⃣ Calling Script (Query + Update + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Status updated to: {'type': 'String', 'value': 'active'}
   📋 Name: {'value': 'Alice Smith', 'type': 'String'}

============================================================
📝 Script 3: Query + Update Credits + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: ec9zQizGQlLUfFrJPSpqgUyZl3SMtgC_La3qx9O9woNvVKKIkvPhJ8E4_TNdZnQ0O7WdLb-korYdhR3BHiGwQQ

2️⃣ Calling Script (Query + Update Credits + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {'value': 0, 'type': 'Integer'}
   📋 Status: {'value': 'active', 'type': 'String'}
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}

============================================================
📝 Script 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: Qe094OUrC9gPXYKkJx4Lt2aeGqrkfK05oG6Y_r7_wNapGDQBV7E393CwQ4eCiz5D9ZwikZx16bR-X4rpTxHXhQ

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: xI5rIpR_Py1pNLaBTS6r...
   ✅ Deleted script: j9roLMKNsSxJd5JqqsKi...
   ✅ Deleted script: ec9zQizGQlLUfFrJPSpq...
   ✅ Deleted script: Qe094OUrC9gPXYKkJx4L...
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
  Output: Document ID = Rq-u73tglFO3Y0oxqq6-NZCfqvNe2nSFRGG4eM-dUfJjdtJB5a5OUFac6VHgVUtfe6oCH4tFz7dK19IroqPG_Q
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: find_by_id(Rq-u73tglFO3Y0oxqq6-NZCfqvNe2nSFRGG4eM-dUfJjdtJB5a5OUFac6VHgVUtfe6oCH4tFz7dK19IroqPG_Q)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: find_by_id(Rq-u73tglFO3Y0oxqq6-NZCfqvNe2nSFRGG4eM-dUfJjdtJB5a5OUFac6VHgVUtfe6oCH4tFz7dK19IroqPG_Q)
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
  Output: Document ID = COR3bjv5PSiw-z-v1hvHN3RWR0N98x0EuwNyHgTmkHNfW9Kr-Tqr6-amaoS7skx8M-UnldzaolP8NbQ7YpXqJw
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: find_by_id(COR3bjv5PSiw-z-v1hvHN3RWR0N98x0EuwNyHgTmkHNfW9Kr-Tqr6-amaoS7skx8M-UnldzaolP8NbQ7YpXqJw)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: find_by_id(COR3bjv5PSiw-z-v1hvHN3RWR0N98x0EuwNyHgTmkHNfW9Kr-Tqr6-amaoS7skx8M-UnldzaolP8NbQ7YpXqJw)
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
    Finished `release` profile [optimized] target(s) in 0.08s
📦 Built wheel for abi3 Python ≥ 3.8 to /Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.8.0-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel for current platform...[0m
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.8.0-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.8.0
    Uninstalling ekodb_client-0.8.0:
      Successfully uninstalled ekodb_client-0.8.0
Successfully installed ekodb-client-0.8.0
✅ [32mPython client package built and installed![0m
🧪 [36mRunning Python client library examples...[0m
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {'id': 'wv-zawRXOT_z9nw5Ekrf9hhjDn1Y9sc1Dq-vs3O_VgcS9BzMp2mkpZYTwkHaGqFXc1_-73o5wjG0lSspGEem5w'}

=== Find by ID ===
Found: {'categories': ['electronics', 'computers'], 'value': 42, 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5], 'tags': ['tag1', 'tag2', 'tag3'], 'active': True, 'created_at': '2026-01-26T03:00:50.465258', 'data': 'aGVsbG8gd29ybGQ=', 'price': 99.99, 'metadata': {'nested': {'deep': True}, 'key': 'value'}, 'name': 'Test Record', 'user_id': '550e8400-e29b-41d4-a716-446655440000', 'id': 'wv-zawRXOT_z9nw5Ekrf9hhjDn1Y9sc1Dq-vs3O_VgcS9BzMp2mkpZYTwkHaGqFXc1_-73o5wjG0lSspGEem5w'}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): True
  price (Decimal): 99.99
  created_at (DateTime): 2026-01-26 03:00:50.465258
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): ['tag1', 'tag2', 'tag3']
  metadata (Object): {'nested': {'deep': True}, 'key': 'value'}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): ['electronics', 'computers']
  data (Bytes): 11 bytes
Plain record: {'categories': ['electronics', 'computers'], 'value': 42, 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5], 'tags': ['tag1', 'tag2', 'tag3'], 'active': True, 'created_at': '2026-01-26T03:00:50.465258', 'data': 'aGVsbG8gd29ybGQ=', 'price': 99.99, 'metadata': {'nested': {'deep': True}, 'key': 'value'}, 'name': 'Test Record', 'user_id': '550e8400-e29b-41d4-a716-446655440000', 'id': 'wv-zawRXOT_z9nw5Ekrf9hhjDn1Y9sc1Dq-vs3O_VgcS9BzMp2mkpZYTwkHaGqFXc1_-73o5wjG0lSspGEem5w'}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'id': 'wv-zawRXOT_z9nw5Ekrf9hhjDn1Y9sc1Dq-vs3O_VgcS9BzMp2mkpZYTwkHaGqFXc1_-73o5wjG0lSspGEem5w', 'tags': {'value': ['tag1', 'tag2', 'tag3'], 'type': 'Array'}, 'metadata': {'value': {'nested': {'deep': True}, 'key': 'value'}, 'type': 'Object'}, 'active': {'value': True, 'type': 'Boolean'}, 'data': {'value': 'aGVsbG8gd29ybGQ=', 'type': 'String'}, 'created_at': {'value': '2026-01-26T03:00:50.465258', 'type': 'String'}, 'embedding': {'value': [0.1, 0.2, 0.3, 0.4, 0.5], 'type': 'Array'}, 'value': {'type': 'Integer', 'value': 100}, 'categories': {'value': ['electronics', 'computers'], 'type': 'Array'}, 'name': {'value': 'Updated Record', 'type': 'String'}, 'price': {'value': 99.99, 'type': 'Float'}, 'user_id': {'value': '550e8400-e29b-41d4-a716-446655440000', 'type': 'String'}}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: rZAETHIc04XZeEiZfyEvawbxrqJV5tjtZOGa58_vvVq1xMK6MeEctnr-BUXtQ_Hcp6lNGCXjtilyYoK1IpQseA

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
Collection created with first record: "ZzlXi5FhVDJ_9_8WaU7bSPvYLVjw5Uvj2CPUexklafGkWaeQECo-ZEoAggX26Ueo9Be7mi6Hpg_7ihu_aEZI3w"

=== List Collections ===
Total collections: 12
Sample collections: ['ttl_cache', 'test_collection', 'user_cache', 'scripts__ek0_testing', 'products']

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
  cache:product:1: {'name': 'Product 1', 'price': 29.99}
  cache:product:2: {'price': 39.99, 'name': 'Product 2'}
  cache:product:3: {'name': 'Product 3', 'price': 49.99}

=== KV Exists ===
Key exists: True

=== KV Find (Pattern Query) ===
Found 3 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 8

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
Created Alice: $1000 - ID: op-7B3o3oKe0Yh4pn8ZksXD_ZtuqBzQ5wpegVx24yn8Fd3LJuIqn3PKW5yX12uwqneGqOVMofr9twNLP3ppu9w
Created Bob: $500 - ID: QIbcXs8cC78ZfZ2Wq3KPV6Xyh-qz9FuulQA3LLm28Zz5spgOLEhqoLMT920aonw8zDZ-wSeV8uIHpzwYQdt86g

=== Example 1: Begin Transaction ===
Transaction ID: d732af89-71aa-44a2-b271-973a535f9ee7

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: f9b51cc0-f137-41e5-a304-ee285dcf9046
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
  1. Score: 25.740, Matched: email.value, name, email, name.value
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: bio.value, title.value, title, bio
  2. Score: 26.400, Matched: title.value, bio, bio.value, title
  3. Score: 26.400, Matched: bio.value, title.value, title, bio
  4. Score: 26.400, Matched: bio, bio.value, title.value, title

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio, title, bio.value, title.value
  2. Score: 39.600, Matched: bio.value, title, bio, title.value
  3. Score: 39.600, Matched: bio.value, bio, title, title.value
  4. Score: 39.600, Matched: bio, title.value, bio.value, title

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 2 results (matches: run, running, runs)
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.763
  2. Score: 0.737
  3. Score: 0.727

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.381, Matched: title.value, title, content.value, content
  2. Score: 1.368, Matched: title, content, content.value, title.value
  3. Score: 0.363, Matched: 

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
✓ Inserted document: urCX6UNZOnO9fslJQo0DLCN1u1YK304vMDtlf8pXGmiw-pMVGDwula0xvr1XuZQ01pJUV3W-8BDYn7RNy7zW0A

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: 0lkuyAJ1LDDoVqjPvKt-_C_SBMuFSJhlVRttpU_zVPiSOFsgkQwo9kLhC6QRHhPup5Oi2mktT2zGfmOiNIt8ag

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
✓ Inserted document with TTL: mg8I_6KtP214xoq684T4epkD5IbkrJY6UoIOKHXIvmfXWXwBi9G5M1pzQux0CjJF_S3xbzoWO1CzLVkvemD0oQ

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
✓ Edge cache script created: MOOOrt7qSspbykyKQCHN-w-R0AQySWgYHoXZV-512HYO6hDJvrIpAKAWSZnIHmL7X350c0kscJ4BBo49ihGhVA

Call 1: Cache miss (fetches from API)
Response time: 2ms
Result: {
  "records": [],
  "stats": {
    "execution_time_ms": 0,
    "input_count": 0,
    "output_count": 0,
    "stage_stats": [],
    "stages_executed": 2
  }
}

Call 2: Cache hit (served from ekoDB)
Response time: 1ms (1.2x faster!)
Result: {
  "records": [],
  "stats": {
    "execution_time_ms": 0,
    "input_count": 0,
    "output_count": 0,
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

✅ Script saved: PC0cJpiDKiH7F2lWoIChHMVs1KZy8fT40gObmOjDETlV349U1bOG6afcTOCssCwQ-Gs0eaGrM9n421b8f3POKg
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved: XdPZpzqwekFPb-WPIY5WplON2lBk8DqiCWgxJ7vdkH2Dh9uRBaNXRtEnNxhwNAx3F2EuRPKAS-eSrTJO1KvjUQ
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: 5E06eKPnbRfPQ3l2DMZ7fbFgJZevJuqgdLo9I254WDH6Ggk8N3tJ14BOGXgTJixliR1DjANELqlLLxDI1SUdZw
📊 Statistics: 2 groups
   {'avg_score': 60.0, 'count': 5, 'status': 'active'}

   {'avg_score': 50.0, 'count': 5, 'status': 'inactive'}

📝 Example 4: Script Management

📋 Total scripts: 18
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

Using CallFunction to replace inline logic in SWR pattern...

✅ Saved reusable function: fetch_and_store_user
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 75.9ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 64.1ms
   📊 Records: 1
   🚀 Cache speedup: 1.2x faster!

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

✅ Script saved: tjKDOG7MYGRZqwBrQwGSqgY1Bo2ThEEjT0mtwfav2zmhxXVWslGaqixWvW2X0ZoRlBmzOXn_yw5SxJXjoJJh1g
📊 Found 2 product groups
   {'avg_price': 474.0, 'category': 'Furniture', 'count': 2}
   {'avg_price': 575.6666666666666, 'category': 'Electronics', 'count': 3}
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

✅ Inserted order: gJ1ewrCAGGxshNc3_7Z-fFpk_pbhcukNe_TrYfXEWTTkTdZPE1b83RI_jZLxMISQH5sO61sha5hYjiUVrCM1JA
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
🗑️  Deleted session: None

📝 Example 4: KV Pattern Query

✅ Set 4 config entries
📊 Found 3 app config entries
📊 Found 4 total config entries

📝 Example 5: Combined Wrapped Types + KV Usage

✅ Inserted order: 5a5E17jTyaQ_PExNa_GNK2442GDAq8zd3mXNlTA1LW8RxPYSySCR8Z2Z2kS7yGYLxwswpMXsp4FvwiZms1ECeA
✅ Cached order status
📊 Quick status lookup: {'value': '{"status":"processing","updated_at":"2026-01-26T08:00:52.368533+00:00"}'}

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
✓ Created SWR script: fetch_api_user_py (moVKimK3MttY3Kb-oIBZ88mDrUSWAtX4T9pH9xO04Ho_6g4iMpIVe62MQlNYlR45yogFTkjdjsjkvpFWeVdjaw)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [],
  "stats": {
    "execution_time_ms": 0,
    "input_count": 0,
    "output_count": 0,
    "stage_stats": [],
    "stages_executed": 2
  }
}
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 2ms (served from cache)
✓ Lightning fast cache hit

🧹 Cleaning up...
✓ Cleanup complete

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation
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
   {'avg_price': 367.0, 'category': 'Electronics', 'count': 5}
   {'avg_price': 365.6666666666667, 'category': 'Furniture', 'count': 3}
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
   Vector databases offer benefits such as efficient storage, precise representation of data, high scalability, and the ability to handle complex, multi-dimensional data. These databases are ideal for geographical data management and spatial analysis.
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
   active: 7 users
   inactive: 3 users
⏱️  Execution time: 0ms

📝 Example 3: Average Score by Role

✅ Script saved
📊 Average score by role:
   {'avg_score': 70.0, 'count': 7, 'role': 'user'}
   {'avg_score': 20.0, 'count': 3, 'role': 'admin'}
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
   1. Database Design Principles (Database)
   2. Natural Language Processing (AI)
   3. Getting Started with ekoDB (Database)
   4. Vector Databases Explained (Database)
   5. Introduction to Machine Learning (AI)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Script saved
📊 Documents by category:
   {'category': 'Database', 'count': 3}
   {'category': 'AI', 'count': 2}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: r3WEzajhiOln7bNynUKzsDgIZFkUdrh2pPsREnAIE12Hs9SDxUUNYqdLerZ05GZScJZq9i4hw9rGdStaEqZt-g

=== Sending Chat Message ===
Message ID: wKltwjXxULjzJHX3qSOGrQXv3Gp7VFQbO0s8vwLBxot-jTVxavSTakmJRwO4WKaPlLmoVh9fi3noRsVQOljrrg

=== AI Response ===
We have three products available:

1. ekoDB Pro: This is an enterprise edition product with advanced features, priced at $299.
2. ekoDB Cloud: This is a fully managed cloud database service product, priced at $499.
3. ekoDB: This is a high-performance database product with AI capabilities, priced at $99.

Execution Time: 2893ms

=== Token Usage ===
Prompt tokens: 450
Completion tokens: 72
Total tokens: 522

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: -gPuox-DNU-wzKh1HuCs130iH34S32C3i_902ZfS4nb09qVe8vl6cmwl1WZY5Maql7B-ZCXJKOn0dxuXldQtVA

=== Sending Initial Message ===
✓ Message sent
  Response: Currently, we have the "ekoDB" available. It is a high-performance database product priced at $99.

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['updated_at', 'content', 'token_usage', 'created_at', 'context_snippets', 'chat_id', 'id', 'role'])
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
✓ Created second session: GtfMq8eRFBL5hDUmOLtkGd4bjUEosRy4nkpiK5_-XmqIIiKyyeNwaHW-_s-pEBkz5CEYEcuVDBgdrN47637TsQ
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
✓ Created session: YZTKwgcM0LUFIISSEphG7T2aRGMZCoX5bTTZbWuYOXK7-hZ_QKRkBxC3Qe3kv1VD2_eD7VMd_bywYTEFyyu6zw

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information provided, the available product is "ekoDB". This is a high-performance database product. It is priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: HbtOyCjlu64LLquUo_Igx9_Fuz3JVUVHr2jJpVQU5WbqRkt2Yg1V6Si2noxeuSGu9lQXoxc201I0PaTN45hyZg
  Parent: YZTKwgcM0LUFIISSEphG7T2aRGMZCoX5bTTZbWuYOXK7-hZ_QKRkBxC3Qe3kv1VD2_eD7VMd_bywYTEFyyu6zw

=== Listing Sessions ===
✓ Found 7 sessions
  Session 1: HbtOyCjlu64LLquUo_Igx9_Fuz3JVUVHr2jJpVQU5WbqRkt2Yg1V6Si2noxeuSGu9lQXoxc201I0PaTN45hyZg (Untitled)
  Session 2: YZTKwgcM0LUFIISSEphG7T2aRGMZCoX5bTTZbWuYOXK7-hZ_QKRkBxC3Qe3kv1VD2_eD7VMd_bywYTEFyyu6zw (Untitled)
  Session 3: GtfMq8eRFBL5hDUmOLtkGd4bjUEosRy4nkpiK5_-XmqIIiKyyeNwaHW-_s-pEBkz5CEYEcuVDBgdrN47637TsQ (Untitled)
  Session 4: r3WEzajhiOln7bNynUKzsDgIZFkUdrh2pPsREnAIE12Hs9SDxUUNYqdLerZ05GZScJZq9i4hw9rGdStaEqZt-g (Untitled)
  Session 5: txXTU787YDCmI_8Mu2xbUdXH1CuPDrTSreIwj10JqQxeQfx-c4ezBzvpIRsHlLBxNzJMQCuJkifsxvald9nfiw (Untitled)
  Session 6: FgQNl1wmCTOX8XWaikjWKnJh5jz1vLej2PnipxoS35AnUh8n2sqqRoULUiv_bYmC1vk9OP-5AowIoQaNZhkzmw (Untitled)
  Session 7: fHsXg70v83-LYbtuzLXIlj7wO9HrxMk8LNNXdJIlO0x3bzdjAUXMoATyoWAIe7npLkz5Docj-wiZlS0gODtFgw (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: HbtOyCjlu64LLquUo_Igx9_Fuz3JVUVHr2jJpVQU5WbqRkt2Yg1V6Si2noxeuSGu9lQXoxc201I0PaTN45hyZg

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Dict Creation ===
✓ Created record with plain dict: {'id': 'z5K5oAd_HxOFzxeqWN4Vl4eHZtVizCSz63P87RW8sVmqePhi0Q1Uz1hSUUe77FuU06wo9LNALQj-CI32F_1ZDA'}

=== Upsert Operation ===
✓ Upsert (update existing record): z5K5oAd_HxOFzxeqWN4Vl4eHZtVizCSz63P87RW8sVmqePhi0Q1Uz1hSUUe77FuU06wo9LNALQj-CI32F_1ZDA
✓ Inserted second record: kLDj6CCkbD7YB8qQE7jWVhiInq771vfGUeXsNcqpqWxmbYgfyoxza_Jk_ptzl160shhiLnR_iWtHMK8uKJebSA
✓ Upsert (update second record): kLDj6CCkbD7YB8qQE7jWVhiInq771vfGUeXsNcqpqWxmbYgfyoxza_Jk_ptzl160shhiLnR_iWtHMK8uKJebSA

=== Find One Operation ===
✓ Found user by email: {'active': {'type': 'Boolean', 'value': True}, 'age': {'type': 'Integer', 'value': 29}, 'id': 'z5K5oAd_HxOFzxeqWN4Vl4eHZtVizCSz63P87RW8sVmqePhi0Q1Uz1hSUUe77FuU06wo9LNALQj-CI32F_1ZDA', 'email': {'value': 'alice.j@newdomain.com', 'type': 'String'}, 'name': {'value': 'Alice Johnson', 'type': 'String'}}
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
   Inserted with ripple: {'id': 'LLYSuADeemyq8UT4eM5b8C698JKUxpBT5dcHam9Lk786fN92di17FGuooSSzz7SHYYe1TqXPeHuOWASTMGQxcA'}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {'id': 'A_DYAfaG8cFug0hF89gLVkkv-aXqbEZTey0Mz7etHhAHtyxBNYiwuDaVFYj5oyd-nwnpH9-pGSG1fugQsuv6bA'}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {'name': {'value': 'Product 1', 'type': 'String'}, 'id': 'LLYSuADeemyq8UT4eM5b8C698JKUxpBT5dcHam9Lk786fN92di17FGuooSSzz7SHYYe1TqXPeHuOWASTMGQxcA', 'price': {'type': 'Integer', 'value': 150}}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {'price': {'type': 'Integer', 'value': 500}, 'id': 'LLYSuADeemyq8UT4eM5b8C698JKUxpBT5dcHam9Lk786fN92di17FGuooSSzz7SHYYe1TqXPeHuOWASTMGQxcA', 'name': {'type': 'String', 'value': 'Upsert Product'}}

✅ All bypass_ripple operations completed successfully!
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields
Fetched 3 users with only 3 fields each

Example 2: Exclude sensitive fields
Fetched 2 admins without sensitive data
  Password field excluded: False

Example 3: Complex query with projection
Fetched 3 active users with profile fields

Example 4: Find by ID with projection
Fetched user profile: {'value': 'Alice Johnson', 'type': 'String'}

Example 5: Compare full vs projected data
Full query returned 12 fields per user
Projected query returned 12 fields per user

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
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
Inserted: map[id:aCuFTML4DJklvoCsrnDBl-ZjxoTJUCQXB2rtFAH2QrEmnb0gWNu1Pqn7MOwvzhTpl2xqQ9cY89mKC-XuKdcfhw]

=== Find by ID ===
Found: map[active:true id:aCuFTML4DJklvoCsrnDBl-ZjxoTJUCQXB2rtFAH2QrEmnb0gWNu1Pqn7MOwvzhTpl2xqQ9cY89mKC-XuKdcfhw name:Test Record value:42]

=== Find with Query ===
Found documents: map[]

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] id:aCuFTML4DJklvoCsrnDBl-ZjxoTJUCQXB2rtFAH2QrEmnb0gWNu1Pqn7MOwvzhTpl2xqQ9cY89mKC-XuKdcfhw name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
[32m✓ simple_crud.go completed successfully[0m
[34m
=== Running simple_websocket.go ===[0m
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: 9EeVbFUN6VQJg44lIJj9v3V6ea92aTPNLFIVKep_EE20y-vFENwg8lMAnyr-UA0MPFZS97bncRRXhrpvyMEw2Q

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
        "id": "2ohhdIKZs8t7QdU7ZWUXCc4EPm8T-Iy-RVYfPio_EofPFq65P9o3SaJhKtwl8h8xuajQE9DK3IsJAV0tEM2-dw",
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
        "id": "V-GwO5brmP15gkZCWnGtp4o8ngZYVpyshElKjClatQ0lIDPngkQSMEFFFJNE6mPPBwj0_pYxOs-mnpoye8qKTw",
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
        "id": "9EeVbFUN6VQJg44lIJj9v3V6ea92aTPNLFIVKep_EE20y-vFENwg8lMAnyr-UA0MPFZS97bncRRXhrpvyMEw2Q",
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

✅ Script saved: BJj74Sjec-JWwmhiHy_8zfnVoNHFFyPsrGuS7atHIgZUkMhHxCLx8FdnbjcyMD3uGp4sAw-VrWkP_XF_yUgWbg
📊 Found 10 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: r2TCqM7fJlOJmLba08ZwhNarLnl44tDa-r_eo8q3WUh04K4GL0Hywqm4UCdGsP1CBlI9Puwdg3cQ-B2cZmo4fw
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: 8y8iro2WEBDslI4GlOoXFmT3qtjlMSRsIB2CGRJ3BWpjh_OneHngnMmTd06-4uAT93XtgVbUCExDDbbwLr0o-A
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
Collection created with first record: Lswnum6kiBaY4xoJ1rRYt4HHTOkeBlu60zwYYWAS87a6jQj2-VbhyQ5NyDfdMWS-5VT8Owe6OWAgFLCxJpIgRg

=== List Collections ===
Total collections: 16
Sample collections: [users ttl_cache test_collection user_cache scripts__ek0_testing]

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
Created Alice: $1000 - ID: p6KHWFGQRRr0zjIfzsUTacR8MGBlShsYQn76pSgQtvNvlf_yYgTMdDSTBEtjgbsgYIcyNh9ErJXTFc-NQ1dyow
Created Bob: $500 - ID: Yv6JfeH6rYy67j2SP9IgONf5gcQf9bDrd1PIjvMe77kkFKmeZSGLKgNbzBQUPmcSCZl6sKmasbSblWKp40YqTg

=== Example 1: Begin Transaction ===
Transaction ID: 2a8a32b5-aaae-4372-ba60-2590cdd384b1

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
New transaction: 758b1e6c-ca39-4264-ae5a-7847d5034b07
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
   ✅ Script saved: rtvoM6o3MaZMOujvWakODzW2Qm-oduKF-9yN2bVxnk50rHG24hkXzlQYb2xstHFK93F_AnI6YbgOLLq1NMIzoA

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: ufqIfy6oqesPzrO69VaVBB-7w9Q9yCC6Tkg1RxwaX9GAtVgy4TYg9xiY1RQT7F_EYkbDXXXBSt0hNcE6KdUVpA
   📋 Name: map[type:String value:Alice Smith]
   📋 Email: map[type:String value:alice@example.com]
   📋 Status: map[type:String value:pending]
   📋 Credits: map[type:Integer value:0]

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: KaTtdUBZ4UhorxOPcTSw1uJjR558nCKFqmN6Ci1PbkKBksJ567-OGnqnG2jTlRI_ItN3Vxdtuql9iPByo5vEdA

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
   ✅ Script saved: 4pmxLRWHCCSRtoxzGKYZ-ow1XAFNwJBtyswCWC7V9DXc6FFVK55CqoZS5AHIU7nrROGIVPEIDQg7xky_Ls8QOA

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
   ✅ Script saved: d2QF86XmRti8bBf5JxgKUhBWTwYlOBrHT_rxHxjOhnbUwQSiG5hnzfmC-o2IlTaCXKAmYg8glHfWJQjfViRt8g

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: rtvoM6o3MaZMOujvWakO...
   ✅ Deleted script: KaTtdUBZ4UhorxOPcTSw...
   ✅ Deleted script: 4pmxLRWHCCSRtoxzGKYZ...
   ✅ Deleted script: d2QF86XmRti8bBf5JxgK...
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

2026/01/26 03:01:22 No .env file found, using defaults
✓ Client connected

═══════════════════════════════════════════════════════════
TEST 1: Document TTL Expiration
═══════════════════════════════════════════════════════════

[Step 1] Insert document with 3 second TTL
  Input: {name: 'TTL Test', value: 'should expire'}
  TTL: 3s
  Output: Document ID = jKs1bFohFiI9rEvUo2GoXoV3OlgsiUKHVX8zGB8cTWL14t-LGYg3UQC_bRGtDLzXJQqfUE2afzpThfPmgXpaFA
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: FindByID(jKs1bFohFiI9rEvUo2GoXoV3OlgsiUKHVX8zGB8cTWL14t-LGYg3UQC_bRGtDLzXJQqfUE2afzpThfPmgXpaFA)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: FindByID(jKs1bFohFiI9rEvUo2GoXoV3OlgsiUKHVX8zGB8cTWL14t-LGYg3UQC_bRGtDLzXJQqfUE2afzpThfPmgXpaFA)
  Output: Error (expected) - request failed with status 404: ��Record has been deleted
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

2026/01/26 03:01:29 No .env file found, using defaults
✓ Client connected

═══════════════════════════════════════════════════════════
TEST: WebSocket TTL Expiration
═══════════════════════════════════════════════════════════

[Step 1] Insert document with 3 second TTL
  Input: {name: 'WS TTL Test', value: 'should expire'}
  TTL: 3s
  Output: Document ID = yldhRYv8WXQTWRX6XdcJTl4SNakOVV5JNQrI_75EebGyda7u6UyFuiam3QMoxpZMC4nziHmgpcYcy30GhfHlZQ
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: FindByID(yldhRYv8WXQTWRX6XdcJTl4SNakOVV5JNQrI_75EebGyda7u6UyFuiam3QMoxpZMC4nziHmgpcYcy30GhfHlZQ)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: FindByID(yldhRYv8WXQTWRX6XdcJTl4SNakOVV5JNQrI_75EebGyda7u6UyFuiam3QMoxpZMC4nziHmgpcYcy30GhfHlZQ)
  Output: Error (expected) - request failed with status 404: ��Record has been deleted
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
Inserted: map[id:o5PqPwidS7c7MMFR0np-JGjXYMhymD3mu3FSLyrYg0WsQJpDV5lQFmFgFxw8uLvPTwEV0HQKTfwtCrfdbNHUHg]

=== Find by ID ===
Found: map[active:true categories:[electronics computers] created_at:2026-01-26T03:01:36-05:00 data:aGVsbG8gd29ybGQ= embedding:[0.1 0.2 0.3 0.4 0.5] id:o5PqPwidS7c7MMFR0np-JGjXYMhymD3mu3FSLyrYg0WsQJpDV5lQFmFgFxw8uLvPTwEV0HQKTfwtCrfdbNHUHg metadata:map[key:value nested:map[deep:true]] name:Test Record price:99.99 tags:[tag1 tag2 tag3] user_id:550e8400-e29b-41d4-a716-446655440000 value:42]

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 0
  active (Boolean): true
  price (Decimal): 99.990000
  created_at (DateTime): 2026-01-26 03:01:36 -0500 EST
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1 tag2 tag3]
  metadata (Object): map[key:value nested:map[deep:true]]
  embedding (Vector): [0.1 0.2 0.3 0.4 0.5]
  categories (Set): [electronics computers]
  data (Bytes): 11 bytes
Plain record: map[active:true categories:[electronics computers] created_at:2026-01-26T03:01:36-05:00 data:aGVsbG8gd29ybGQ= embedding:[0.1 0.2 0.3 0.4 0.5] id:o5PqPwidS7c7MMFR0np-JGjXYMhymD3mu3FSLyrYg0WsQJpDV5lQFmFgFxw8uLvPTwEV0HQKTfwtCrfdbNHUHg metadata:map[key:value nested:map[deep:true]] name:Test Record price:99.99 tags:[tag1 tag2 tag3] user_id:550e8400-e29b-41d4-a716-446655440000 value:42]

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-01-26T08:01:36Z] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:o5PqPwidS7c7MMFR0np-JGjXYMhymD3mu3FSLyrYg0WsQJpDV5lQFmFgFxw8uLvPTwEV0HQKTfwtCrfdbNHUHg metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Updated Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: dnwhRGVd_NaNcPqeW8Ws3_Bw1CSBWoY7z3eM_SpG2sY0tM0OBXekjiFPNvh0_BuSWPFiHFdssFybjFHKI1Ur8Q

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
Collection created with first record: dMjXVhdfp2QkeQg8HkhS7Z3YL3PK-H0lDqcEY5sVO2EpmISdMEHDkxGyWn-wnLHnDXI1ev7dradu_wZKeJ90dg

=== List Collections ===
Total collections: 15
Sample collections: [ttl_cache test_collection user_cache scripts__ek0_testing schema_users_client_py]

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
Total keys in store: 8

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
Created Alice: $1000 - ID: tLdSRk2tzQDHLJXjJWcM1VO454mvV8B0QmKifXOlERChdEhS3dZd8saWDSot6cF880TsyuP7AaBuqKyvQAL62Q
Created Bob: $500 - ID: 3cRjCBDbYBdxeH_Qzq1hdflfjY1Eb51VQTstyg21ShBDxflbr2thjWE5VrJIklapXy6CZUSDHhPHPd5UDzEy-A

=== Example 1: Begin Transaction ===
Transaction ID: f0eaddde-abf1-43de-a09a-17cb49c0786e

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: a6fb394f-3710-44ff-99cc-6ce975fed120
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
  1. Score: 0.764
  2. Score: 0.748
  3. Score: 0.734

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.699
  2. Score: 1.506
  3. Score: 0.293

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
✓ Inserted document: f4xQf5-xO59HATnwqKKpH1rYa8CDAu1QvOrXxiGu6T_lc9aScMU3GuVWqAC9f09v1XmI_5clm1qt2UutKYHejQ

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: IWe9HBnwY2XMxE2UmA7GzI4NJ-aSeiZ_5R7I7EmugOG9FTCbEcaU4kZeLlEbV3pNRiOMjOJuJiSgyNrf4hYjmw

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
✓ Inserted document with TTL: W45qOG1fqytIDKmJ9vFURM7iMUWgZhc3Fi6Vdn0VCJCGMIswPsMbNLN7KYkVY8K6ifvLJqhNL-Ri4ybKK2wS6Q

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
✓ Edge cache script created: gPZ2MiCiDn6wyuDu5_2bA4m_BND6LLd5Gukw9T8WqmBYNexIP426PQtk97bgxGQRMXEzLvDmux6Fa7FGrDLtaQ

Call 1: Cache lookup
Response time: 2ms
Found 1 cached entries

Call 2: Cache lookup (connection warm)
Response time: 2ms
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

✅ Script saved: ReU1o1l5VaddkPoi5Wu_erfuqMB5Hz9INrjUiPJLXQxr353HiEQ_kNODGtyu_1a300uZrUtMPENJvUBbwJgP4Q
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

Using CallFunction to replace inline logic in SWR pattern...

✅ Saved reusable function: fetch_and_store_user
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 66.125292ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 68.192708ms
   📊 Records: 1
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

✅ Script saved: wQ-AfeuooNQxVjuMnmgwmNhHTbPpoxqGj_ZHkp41kg9afUBub9FjOPzfXyvTByM0-HmLznSCcQI19Dqa83C6rQ
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

✅ Inserted order: 4uR0I7iAtsgB9XWdIIF2QfSQLa7tHRPRdn20ZW73Kr0UHLuzMRF4NhrBcWHTigp-iGf9Ol1pUmvc3lmAXP-EgA
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: xWiW2gsbCyfEsYiKyQpnDb0yOvd8M5fGosN58BNCgg1FJ9TII87_Pzk0Ue2PYKqdlgtEM9cjohy2zaPOttvP-A
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: map[role:admin userId:user_abc]
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session
📝 Example 4: KV Operations in Scripts

✅ Script saved: _y79BKkPfQLldQBRVG3397J8bmkQ9aXNH18Rdr4bHEbkD21CxYv5TYjN6yf7RxZo7W9wAFNd1RI0GnhfrQVq-g
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Script

✅ Script saved: Jq_FEV1mFiQDDwISDegRwImuPnhvGEbGwE_1JXK3IZYVxsXHy9Gz3tRPm71sTVgZ6ArK6_EV9QD09G_4QbHKTw
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
✓ Created SWR script: fetch_api_user_go (gKE6vtvUsGTw_WTxQ72q7PHYTx15Lnjx2KDl9Ffesdnrzyhf0i2MaDDtLrjP5qRLfBZAVvTw3Qqhepp5zaj7zA)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "cached_at": "2026-01-26T03:01:39-05:00",
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
      "id": "OgwX53vIi1aMEr8wbzQnVW11vUYHEGb_7TQ8JSmwMV9CGJW51ObanIUoIFbFLI6eBHmx10Es1nA-RVHKoixYJw"
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
Response time: 67ms (served from cache)
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
   3. map[type:String value:Database Design Principles] (map[type:String value:Database])
   4. map[type:String value:Introduction to Machine Learning] (map[type:String value:AI])
   5. map[type:String value:Vector Databases Explained] (map[type:String value:Database])
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Script saved
📊 Documents by category:
   map[category:Database count:3]
   map[category:AI count:2]
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: cpHlk-ADD4R88a2saVoQSXnT-rPHLY-3TLG0Q-4D1KmO5jf_Yq15-g-qFDRzbar4P_XZYuWWMmUL3QOqQXwdeQ

=== Sending Chat Message ===
Message ID: IWCIRjwZ-ug1-h3nsSo0dvhLXbR0uLUH8fTOc3YjlqiRQFFe3W6mhkg2ky1BXzY77385N-8R7GYFI49gnkKwvw

=== AI Response ===
We have three products available:

1. ekoDB: This is a high-performance database product with AI capabilities. The price of ekoDB is $99.

2. ekoDB Pro: This is our enterprise edition product with advanced features. The price of ekoDB Pro is $299.

3. ekoDB Cloud: This is a fully managed cloud database service product. The price of ekoDB Cloud is $499.

=== Context Used (3 snippets) ===
  Snippet 1: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:A high-performance database product with AI capabilities id:mL1hDM5pQkl8JJCEwgdN6EaggUg_vpVYEodE9Xi1mbaxjc9C2Eg_Bf_XP5E-7b929IG_dw_G_nKhDNlY5v9muA name:ekoDB price:99] score:0.1111111111111111]
  Snippet 2: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Enterprise edition product with advanced features id:mIOx_g-ReeR8uZWvN8fU8AE2jNTdCKmUIHeTN0I-hxlH2IRxJu_Sx9ASbIQmvM6EsDmaoIWSFQP-AXx-QxRXtg name:ekoDB Pro price:299] score:0.1111111111111111]
  Snippet 3: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Fully managed cloud database service product id:IAC-3J8d4OwuowDA1VnVAQfuFlWbMq3gtMYBactLUKvwpHbDslDRJgIWqBzGqPVZBVma4Uyud-6N-9Y4ftnswg name:ekoDB Cloud price:499] score:0.1111111111111111]

Execution Time: 3123ms

=== Token Usage ===
Prompt tokens: 449
Completion tokens: 89
Total tokens: 538

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 6opKRrWhlX015gbHC8zT50GwgPqw9UR1RFJhw0CWOR0D85glL8ra5x2YYgy28M8JgBjCLO8gR5834EXLnwKagQ

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information available, we have the "ekoDB" available. It's a high-performance database product that costs $99.

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
✓ Created second session: L2n3N8nWclhIqgCg_HvoXpytdb71VhqTt_bw1_KjLcMzxTom1LQ9TYXYlaiJBahUKc_Q6LPU6nAMWPHPP4g4Zw
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
✓ Created session: XEMmWa39EXymaiyPjWK2XGee5O1GdzWDrVv-XIqEBYq4pg3FFS9TKL-u_VHtaFuYOaC-fjydSlX8mx-hHZg8uw

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information provided, the available product is "ekoDB". It is a high-performance database product. The price of this product is $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: jRazOc5hAjklSd2FlL-I7bHARy_SNZ7LIcKA8vJRDn2Qr8KQ6ZpxkamS4k5asXXP7naLHx-KvT_tsHEG86IItA
  Parent: XEMmWa39EXymaiyPjWK2XGee5O1GdzWDrVv-XIqEBYq4pg3FFS9TKL-u_VHtaFuYOaC-fjydSlX8mx-hHZg8uw

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: jRazOc5hAjklSd2FlL-I7bHARy_SNZ7LIcKA8vJRDn2Qr8KQ6ZpxkamS4k5asXXP7naLHx-KvT_tsHEG86IItA (Untitled)
  Session 2: XEMmWa39EXymaiyPjWK2XGee5O1GdzWDrVv-XIqEBYq4pg3FFS9TKL-u_VHtaFuYOaC-fjydSlX8mx-hHZg8uw (Untitled)
  Session 3: L2n3N8nWclhIqgCg_HvoXpytdb71VhqTt_bw1_KjLcMzxTom1LQ9TYXYlaiJBahUKc_Q6LPU6nAMWPHPP4g4Zw (Untitled)
  Session 4: cpHlk-ADD4R88a2saVoQSXnT-rPHLY-3TLG0Q-4D1KmO5jf_Yq15-g-qFDRzbar4P_XZYuWWMmUL3QOqQXwdeQ (Untitled)
  Session 5: YZTKwgcM0LUFIISSEphG7T2aRGMZCoX5bTTZbWuYOXK7-hZ_QKRkBxC3Qe3kv1VD2_eD7VMd_bywYTEFyyu6zw (Untitled)
  Session 6: GtfMq8eRFBL5hDUmOLtkGd4bjUEosRy4nkpiK5_-XmqIIiKyyeNwaHW-_s-pEBkz5CEYEcuVDBgdrN47637TsQ (Untitled)
  Session 7: r3WEzajhiOln7bNynUKzsDgIZFkUdrh2pPsREnAIE12Hs9SDxUUNYqdLerZ05GZScJZq9i4hw9rGdStaEqZt-g (Untitled)
  Session 8: txXTU787YDCmI_8Mu2xbUdXH1CuPDrTSreIwj10JqQxeQfx-c4ezBzvpIRsHlLBxNzJMQCuJkifsxvald9nfiw (Untitled)
  Session 9: FgQNl1wmCTOX8XWaikjWKnJh5jz1vLej2PnipxoS35AnUh8n2sqqRoULUiv_bYmC1vk9OP-5AowIoQaNZhkzmw (Untitled)
  Session 10: fHsXg70v83-LYbtuzLXIlj7wO9HrxMk8LNNXdJIlO0x3bzdjAUXMoATyoWAIe7npLkz5Docj-wiZlS0gODtFgw (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: jRazOc5hAjklSd2FlL-I7bHARy_SNZ7LIcKA8vJRDn2Qr8KQ6ZpxkamS4k5asXXP7naLHx-KvT_tsHEG86IItA

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Map Creation ===
✓ Created record with native map: map[id:Bv38Xf8KAaQDidMxnQm6vioGMl8opfINWvfptlMzW70rRU-uaRjUPlytp3FkNITe49-q-jFuEBUbJZP9ItD2aA]

=== Upsert Operation ===
✓ First upsert (update): map[active:map[type:Boolean value:true] age:map[type:Integer value:29] email:map[type:String value:alice.j@newdomain.com] id:Bv38Xf8KAaQDidMxnQm6vioGMl8opfINWvfptlMzW70rRU-uaRjUPlytp3FkNITe49-q-jFuEBUbJZP9ItD2aA name:map[type:String value:Alice Johnson]]
✓ Second upsert (insert): map[id:27ZIVECn1FtOymc68zsT1K1ARcQh1gK8Jv3Qx5nl8RmX-wN4-zWExHed_2ukTLMDr6nB8Q0BNDVCP09M-TVZnw]

=== Find One Operation ===
✓ Found user by email: map[active:map[type:Boolean value:true] age:map[type:Integer value:29] email:map[type:String value:alice.j@newdomain.com] id:Bv38Xf8KAaQDidMxnQm6vioGMl8opfINWvfptlMzW70rRU-uaRjUPlytp3FkNITe49-q-jFuEBUbJZP9ItD2aA name:map[type:String value:Alice Johnson]]
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
Inserted with ripple: map[id:YClLxG1PicW4gO-vrAsv-mcTyheYyDxubrwgFOvi5EV4NIOWYCV5cGcYx42jp7qqm_6azKbcchBdSwoGEOYyWg]
Inserted with bypass_ripple: map[id:JKl26vKqHQBGBZIWu_7d7s3yJSzNHe7ZB198szRj8VYqpFCsf2Hf3Km1n8MBR-cAX5lO1zNx7SHiDsSMdQl4zQ]
Inserted with TTL and bypass_ripple: map[id:62VSauD3Vi1cqfo2G8wDQfaFBlFEoJGEuaEIneKVMdUm4a-RE8cvwYXSTetYDXtqbBOBvSzDijjafODtN38bMQ]
Updated with bypass_ripple: map[id:YClLxG1PicW4gO-vrAsv-mcTyheYyDxubrwgFOvi5EV4NIOWYCV5cGcYx42jp7qqm_6azKbcchBdSwoGEOYyWg name:map[type:String value:Product 1] price:map[type:Integer value:150]]
Deleted with bypass_ripple
Batch inserted with bypass_ripple: 2 records
Upserted with bypass_ripple: map[id:uKugc0XK7bkKnMD6pPJgdg6WK-GVCd0zG9Gme4NyHAG5akLRqNWJSoIocj7_mYti1RKngO9z23USIr9WupKk5w]
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
✅ [32mAll Go integration tests complete![0m
📦 [36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.8.0 prepare
> npm run build


> @ekodb/ekodb-client@0.8.0 build
> tsc


up to date, audited 58 packages in 1s

15 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.8.0 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning TypeScript client library examples...[0m

added 1 package, removed 1 package, and audited 13 packages in 832ms

3 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'NjAf6nqkHZWq5JtfrPGqZlDILtJ1Gk4g8WR-g-x4OiU3k51omF1tGf68PwP_0_X_lk93nA25BFPJrxAFjlmcUA'
}

=== Find by ID ===
Found: {
  metadata: { key: 'value', nested: { deep: true } },
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  categories: [ 'electronics', 'computers' ],
  id: 'NjAf6nqkHZWq5JtfrPGqZlDILtJ1Gk4g8WR-g-x4OiU3k51omF1tGf68PwP_0_X_lk93nA25BFPJrxAFjlmcUA',
  price: 99.99,
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  data: 'aGVsbG8gd29ybGQ=',
  value: 42,
  created_at: '2026-01-26T08:01:56.487Z',
  name: 'Test Record',
  active: true,
  tags: [ 'tag1', 'tag2', 'tag3' ]
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-01-26T08:01:56.487Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { key: 'value', nested: { deep: true } }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  metadata: { key: 'value', nested: { deep: true } },
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  categories: [ 'electronics', 'computers' ],
  id: 'NjAf6nqkHZWq5JtfrPGqZlDILtJ1Gk4g8WR-g-x4OiU3k51omF1tGf68PwP_0_X_lk93nA25BFPJrxAFjlmcUA',
  price: 99.99,
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  data: 'aGVsbG8gd29ybGQ=',
  value: 42,
  created_at: '2026-01-26T08:01:56.487Z',
  name: 'Test Record',
  active: true,
  tags: [ 'tag1', 'tag2', 'tag3' ]
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  value: { type: 'Integer', value: 100 },
  price: { type: 'Float', value: 99.99 },
  categories: { type: 'Array', value: [ 'electronics', 'computers' ] },
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] },
  id: 'NjAf6nqkHZWq5JtfrPGqZlDILtJ1Gk4g8WR-g-x4OiU3k51omF1tGf68PwP_0_X_lk93nA25BFPJrxAFjlmcUA',
  user_id: { value: '550e8400-e29b-41d4-a716-446655440000', type: 'String' },
  metadata: { value: { key: 'value', nested: [Object] }, type: 'Object' },
  data: { value: 'aGVsbG8gd29ybGQ=', type: 'String' },
  created_at: { type: 'DateTime', value: '2026-01-26T08:01:56.487Z' },
  name: { value: 'Updated Record', type: 'String' },
  active: { type: 'Boolean', value: true }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: UdDi7uTNip0KsylYfoMECzf_lE5uh2gL-WzAerJGJYb1KnljUbTaIA_86TsrEPz2LeQf7-TYJ20pXEDvQaL2Sw

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
Collection created with first record: rp49mJuvGvFf3VwB7EJ8qM16LX2Bk8gwIm-22p0wLzkBTkKTkehto4W4pfpK-aM7RVsQf50teO5wmAHgpOQxXg

=== List Collections ===
Total collections: 21
Sample collections: users,schema_users_client_go,ttl_cache,test_collection,schema_documents_client_go

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
Retrieved value: { username: 'john_doe', userId: 123 }

=== KV Batch Set ===
✓ Batch set 3 keys
  cache:product:1: success
  cache:product:2: success
  cache:product:3: success

=== KV Batch Get ===
✓ Batch retrieved 3 values
  cache:product:1: { value: { price: 29.99, name: 'Product 1' } }
  cache:product:2: { value: { price: 39.99, name: 'Product 2' } }
  cache:product:3: { value: { name: 'Product 3', price: 49.99 } }

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 3 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 8

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
Created Alice: $1000 - ID: bQouoLAnlzFV6_pfXA-9c8mhgxOHIlzajECFFUpeHPjR3fDEQN7PsrGX6DeUu7BeZUa1Zj_bDD49smb-qf6AEA
Created Bob: $500 - ID: vwnpg2ku8zgTj9mkb0TITJSbl0SAcskov6swGGi2sbnBiK8pJaDxXOofT2iNE8Ck00yE0nd46fmJv8h15BPrJA

=== Example 1: Begin Transaction ===
Transaction ID: 02e7f5f9-90ec-4478-bb07-19f907f10c34

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: f020a502-8c45-43fc-9056-8aa12855f426
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
  1. Score: 25.740, Matched: email.value, email, name.value, name
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title, bio, bio.value, title.value
  2. Score: 26.400, Matched: bio.value, bio, title.value, title
  3. Score: 26.400, Matched: title, title.value, bio.value, bio
  4. Score: 26.400, Matched: bio.value, title, bio, title.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio.value, title, title.value, bio
  2. Score: 39.600, Matched: title, title.value, bio.value, bio
  3. Score: 39.600, Matched: title.value, bio.value, title, bio
  4. Score: 39.600, Matched: bio, title.value, title, bio.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio.value, bio

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.751, Matched: 
  2. Score: 0.737, Matched: 
  3. Score: 0.722, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.700, Matched: content.value, title, title.value, content
  2. Score: 1.495, Matched: content, title.value, title, content.value
  3. Score: 0.289, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: skills, bio, skills.value, bio.value

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
✓ Inserted document: NQbzVy4Fdp4-y10ynl2qLiSFfh1c1RjGen8UBT-hAsRu0cL73Hd7Gg1OLNNYp_Z46QN3cv8mJhGrqSVZ5BXY0Q

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: NPKmBoTtxJDC5ZI_cMeO63Za7DLhP0qKX3b6Kqvv2AUDwTjVSrvb_JqGPSllInE8XZLeqcYuFoMOvcMrl7uKUA

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
✓ Inserted document with TTL: Fo-jKeDiux2ZqASkeKwJ9EuGHOgR3WHyRgGpRdUOJTDPCzzS2f3MPQgZZGPOv3Mfo3OHb10WdIq1EK3hyB-CnA

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
✓ Edge cache script created: IPIwEohoDboPQAZb3-uLwSq5s8XzE8aa5MH9AcjRmOX3T0sFi9Jafnar3F8Ic-_NY--n-5oSAgqNMDQe-omaoA

Call 1: Cache miss (fetches from API)
Response time: 4ms
Result: {
  "records": [],
  "stats": {
    "input_count": 0,
    "output_count": 0,
    "execution_time_ms": 0,
    "stages_executed": 2,
    "stage_stats": []
  }
}

Call 2: Cache hit (served from ekoDB)
Response time: 4ms (1x faster!)
Result: {
  "records": [],
  "stats": {
    "input_count": 0,
    "output_count": 0,
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

🚀 ekoDB Scripts Example (TypeScript)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: 1513PcOj3DXurFYK-9arPYLDAVdl26HMFDsiO9grKIgWqIrbrjF5VJYMtDOzUTwLkgbGFrqeFj3hJ8bLM8h1PA
📊 Found 15 active users

📝 Example 2: Parameterized Script

✅ Script saved: 3XCPSw5x0gJ039D4Q8T8onM5G4-6wgupzfQNSSr7UTCflmmlaPW2NrX3bJm1j5kMfmoSJapYCrT0KvgfyQPZ1Q
📊 Found 15 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: 3dvL3iK7a6828TuC7M2EmCcqwaQcPIlLpOl01Ud9sgo3uFqXad8aTHHQf2HfEZ4OuEZj7Xg5YE8zS5qQ15H63g
📊 Statistics: 3 groups
   {"avg_score":50,"status":"inactive","count":5}
   {"count":5,"status":"active","avg_score":60}
   {"status":"null","count":5,"avg_score":60}

📝 Example 4: Script Management

📋 Total scripts: 41
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
   Name: {"type":"String","value":"User 1"}
   Department: {"value":"engineering","type":"String"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using CallFunction to replace inline logic in SWR pattern...

✅ Saved reusable function: fetch_and_store_user
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 74ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 68ms
   📊 Records: 1
   🚀 Cache speedup: 1.1x faster!

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
🚀 ekoDB TypeScript Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved: 6IWlwo-yxeTrGZk1HBOqD9tQIF4sxDJuz1pYx8BnFvaXZsO6S6FNKX5c95Fds-N34HOIgSGADDOA82wasLivtA
📊 Found 2 product groups
   {"avg_price":474,"category":"Furniture","count":2}
   {"avg_price":575.6666666666666,"category":"Electronics","count":3}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved
📊 Found 2 categories
   {"count":2,"category":"Furniture"}
   {"category":"Electronics","count":3}
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
🚀 ekoDB TypeScript KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: GFssfnQKkXlVXt4cvLN-FFsxEwrgeErqKqKMWTzX-VLJeznHVHw8fl5hUtitQq0o3-020Hf0JDOK6OqW9Urc6g
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: LWL2kBu2i0sVEkYbUNkEO1xY79u2bB2IR3UB40h_Uii0MZEbqiGTVuDgkDqkDRcSI5gFtSRPGJYhx4klQJNGKw
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"role":"admin","userId":"user_abc"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: 90E72IbhS3j7A37BBB5GT_WuvTcTs-lGGZC7gntRyP1DJoJ3BpnUZ9uJ_F1FrJRaQ6OiDM7K37Hw4gTJW4jcrA
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Script

✅ Script saved: sD88Ehw3LExCBsjmHPMg0p1WQ8S1mxJ6J4zNSrd1nxlcJWK_YfIgD-kKaBQEtUV8eTsYQVpFNNLromNX4qBUJA
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
✓ Created SWR script: fetch_api_user (nl0CvaPGGkdAd-ZujyoXx5fNmPj03lkpbbFcEWYyf90c70_It1pNawe9DS3YC1HXWUzVpyvPNYM59qcJRoZ_OA)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [],
  "stats": {
    "input_count": 0,
    "output_count": 0,
    "execution_time_ms": 0,
    "stages_executed": 2,
    "stage_stats": []
  }
}
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 3ms (served from cache)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_with_reviews (SK3yj6eVi02tbrtEaf88VYPo2ICeq0reQ_IUMbXcVSNEP32ElNEGMMUM7KusYLO7mtv2j4a2NEq6Pynylm2_mA)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [],
  "stats": {
    "input_count": 0,
    "output_count": 0,
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

🚀 ekoDB TypeScript Advanced Scripts Example

📋 Setting up test data...
✅ Created 8 products

📝 Example 1: List All Products

✅ Script saved
📊 Found 8 products
⏱️  Execution time: 0ms

📝 Example 2: Group Products by Category

✅ Script saved
📊 Category breakdown:
   {"count":5,"avg_price":367,"category":"Electronics"}
   {"avg_price":365.6666666666667,"category":"Furniture","count":3}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced script examples finished!
🚀 ekoDB TypeScript AI Scripts Example

📋 Setting up test data...
✅ Created 2 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
🤖 AI Response:
   Vector databases offer several benefits including high scalability, precise data representation, easy data manipulation such as scaling and rotating, and they require less storage space as they store data in mathematical formulas. They also support complex geographic operations like adjacency, connectivity, and proximity analysis.
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
🚀 ekoDB TypeScript CRUD Scripts Example

📋 Setting up test data...
✅ Created 10 test users

📝 Example 1: List All Users

✅ Script saved
📊 Found 10 users
⏱️  Execution time: 0ms

📝 Example 2: Count Users by Status

✅ Script saved
📊 User counts by status:
   active: 7 users
   inactive: 3 users
⏱️  Execution time: 0ms

📝 Example 3: Average Score by Role

✅ Script saved
📊 Average score by role:
   {"count":3,"role":"admin","avg_score":20}
   {"role":"user","avg_score":70,"count":7}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD script examples finished!
🚀 ekoDB TypeScript Search Scripts Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Script saved
📊 Found 5 documents
   1. Natural Language Processing (AI)
   2. Introduction to Machine Learning (AI)
   3. Database Design Principles (Database)
   4. Vector Databases Explained (Database)
   5. Getting Started with ekoDB (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Script saved
📊 Documents by category:
   {"count":2,"category":"AI"}
   {"category":"Database","count":3}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: doaPGJAF7e3z4i0nEIk9aJm3ASjNJ51v1ObUWXku0_s4SqDqnEBotSJ12_4wIanyzkSEcPSqTXEm0jVd_xM1Uw

=== Sending Chat Message ===
Message ID: 9TwGV4VKsCHsR13Yf_griLlFiBY5NrYC4sXaDicgO79XtzflNUVRGq6qeStktIiTrND3Cs0ZeHfgPE3dw3ARxg

=== AI Response ===
There are three products available:

1. ekoDB: This is a high-performance database product with AI capabilities. The price for this product is $99.

2. ekoDB Pro: This is an enterprise edition product with advanced features. The price for this product is $299.

3. ekoDB Cloud: This is a fully managed cloud database service product. The price for this product is $499.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    price: 99,
    description: 'A high-performance database product with AI capabilities',
    id: 'EayEgUv6MNsLFSDa_1VV6yHHgKBamQApj0wNuEn9C1u1GExaOii54-vpY4R8SuGQTeufxi37w1YyJ9Umyh_gmw',
    name: 'ekoDB'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    id: 'f7XBt6cydpFmLOai7WbfBrMC36hyno9w789GZano4tC172Jp7f04M1knEn9Ut_bx_UpWvx_HmuMIPHrXpuNKgA',
    price: 299,
    name: 'ekoDB Pro',
    description: 'Enterprise edition product with advanced features'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    id: '5zJasQZD8aOF-3_nXYJClA5bCSpy54_E6mcwiw1TPgDFl1cvG0ECsH0mubKO7jFI9-naRCehRuYiccF2eNbvLg',
    name: 'ekoDB Cloud',
    price: 499,
    description: 'Fully managed cloud database service product'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 2902ms

=== Token Usage ===
Prompt tokens: 446
Completion tokens: 84
Total tokens: 530

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: VTmAXQEgx1yETJler8SaHOomFHLfjYps11sisUIUdNOy25U0G-X2HmQwPUDMmTfXmvgsMhAGtMW_awXv27016Q

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information available, the product that is available is "ekoDB". It is a high-performance database product which is priced at $99.

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
✓ Created second session: fwouIl2cr-G6N6vNT0z14MolEjcyX2FSbskE7sSUhtpFrghvIfYLMZYeoj-7-HKa8zhavOf0bf0XN3kitgzqkQ
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
✓ Created session: adrkqaUlLO292DEXjF9g-V8TStYqHCmoKU2zeyvk7CFpO9XTqFR3qss8ZtZSXRoYXns5MrvgmZt6YarMyvxcIA

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the provided context, the available product is "ekoDB". It is a high-performance database product, priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: Hg0m-n5jbm5b1Sw5y7VymFn3_eWKJP6k22S8rme5rig4Xeiot00is67VXY6YXazscqnjtmoHEEHZ8rpSY-GdJQ
  Parent: adrkqaUlLO292DEXjF9g-V8TStYqHCmoKU2zeyvk7CFpO9XTqFR3qss8ZtZSXRoYXns5MrvgmZt6YarMyvxcIA

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: Hg0m-n5jbm5b1Sw5y7VymFn3_eWKJP6k22S8rme5rig4Xeiot00is67VXY6YXazscqnjtmoHEEHZ8rpSY-GdJQ (Untitled)
  Session 2: adrkqaUlLO292DEXjF9g-V8TStYqHCmoKU2zeyvk7CFpO9XTqFR3qss8ZtZSXRoYXns5MrvgmZt6YarMyvxcIA (Untitled)
  Session 3: fwouIl2cr-G6N6vNT0z14MolEjcyX2FSbskE7sSUhtpFrghvIfYLMZYeoj-7-HKa8zhavOf0bf0XN3kitgzqkQ (Untitled)
  Session 4: doaPGJAF7e3z4i0nEIk9aJm3ASjNJ51v1ObUWXku0_s4SqDqnEBotSJ12_4wIanyzkSEcPSqTXEm0jVd_xM1Uw (Untitled)
  Session 5: XEMmWa39EXymaiyPjWK2XGee5O1GdzWDrVv-XIqEBYq4pg3FFS9TKL-u_VHtaFuYOaC-fjydSlX8mx-hHZg8uw (Untitled)
  Session 6: L2n3N8nWclhIqgCg_HvoXpytdb71VhqTt_bw1_KjLcMzxTom1LQ9TYXYlaiJBahUKc_Q6LPU6nAMWPHPP4g4Zw (Untitled)
  Session 7: cpHlk-ADD4R88a2saVoQSXnT-rPHLY-3TLG0Q-4D1KmO5jf_Yq15-g-qFDRzbar4P_XZYuWWMmUL3QOqQXwdeQ (Untitled)
  Session 8: YZTKwgcM0LUFIISSEphG7T2aRGMZCoX5bTTZbWuYOXK7-hZ_QKRkBxC3Qe3kv1VD2_eD7VMd_bywYTEFyyu6zw (Untitled)
  Session 9: GtfMq8eRFBL5hDUmOLtkGd4bjUEosRy4nkpiK5_-XmqIIiKyyeNwaHW-_s-pEBkz5CEYEcuVDBgdrN47637TsQ (Untitled)
  Session 10: r3WEzajhiOln7bNynUKzsDgIZFkUdrh2pPsREnAIE12Hs9SDxUUNYqdLerZ05GZScJZq9i4hw9rGdStaEqZt-g (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: Hg0m-n5jbm5b1Sw5y7VymFn3_eWKJP6k22S8rme5rig4Xeiot00is67VXY6YXazscqnjtmoHEEHZ8rpSY-GdJQ

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Object Creation ===
✓ Created record with plain object: {
  id: 'S6DVbm6xA8EkWbgiH6tmRSOZ2MWlrxw_49Yfu7vNln60X4mQIAsFcpESIauHrnR4iu4fdkNCR1rh6pHIA_i7Cg'
}

=== Upsert Operation ===
✓ First upsert (update): {
  age: { type: 'Integer', value: 29 },
  email: { type: 'String', value: 'alice.j@newdomain.com' },
  active: { value: true, type: 'Boolean' },
  name: { type: 'String', value: 'Alice Johnson' },
  id: 'S6DVbm6xA8EkWbgiH6tmRSOZ2MWlrxw_49Yfu7vNln60X4mQIAsFcpESIauHrnR4iu4fdkNCR1rh6pHIA_i7Cg'
}
✓ Second upsert (insert): {
  id: '3fW5RvBtQmwB1oe7Wicap8o29hlcssubUKCmWgw7fsrv5cDMdF2Wf5aHG2oo_bhORea1H7OvBNXaPaPl5wQdtA'
}

=== Find One Operation ===
✓ Found user by email: {
  age: { type: 'Integer', value: 29 },
  name: { type: 'String', value: 'Alice Johnson' },
  active: { value: true, type: 'Boolean' },
  id: 'S6DVbm6xA8EkWbgiH6tmRSOZ2MWlrxw_49Yfu7vNln60X4mQIAsFcpESIauHrnR4iu4fdkNCR1rh6pHIA_i7Cg',
  email: { value: 'alice.j@newdomain.com', type: 'String' }
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
   Inserted with ripple: {"id":"YGqCXk4frebAfWXanNUYVSLO57HeC0Ktpm7TzT9SqC5fA_oe5CM3WqsQ0CPtSMHpYdydwDEH5Be8cUJ6qLPTuQ"}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {"id":"t9oaBP79effRNED-Y_pQvtgYUJeohkJqMsM8p6pVLK2miTpWhB6vWEiukRdzxdJXridepT5piaGslZdr5QXogA"}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {"id":"YGqCXk4frebAfWXanNUYVSLO57HeC0Ktpm7TzT9SqC5fA_oe5CM3WqsQ0CPtSMHpYdydwDEH5Be8cUJ6qLPTuQ","name":{"type":"String","value":"Product 1"},"price":{"value":150,"type":"Integer"}}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {"id":"eZ3gP1C7aGQpcUV-tVaxjB27kPxPZfblKgrUpNF6PQwe4CxMspr9poODHPj2ZmJp9tbXBTJ4ygdOscar7uywXQ"}

✅ All bypass_ripple operations completed successfully!
✓ Client created

Setting up test data...
✓ Inserted 4 test users

Example 1: Select specific fields
Fetched 3 users with only 3 fields each

Example 2: Exclude sensitive fields
Fetched 2 admins without sensitive data
  Password field excluded: false

Example 3: Complex query with projection
Fetched 3 active users with profile fields

Example 4: Find by ID with projection
Fetched user profile: [object Object]

Example 5: Compare full vs projected data
Full query returned 12 fields per user
Projected query returned 12 fields per user

🧹 Cleaning up test data...
✅ Cleanup complete

✅ All projection examples completed successfully!
✅ [32mTypeScript client examples complete![0m
✅ [32mAll TypeScript integration tests complete![0m
🧪 [36mRunning JavaScript examples (direct HTTP/WebSocket)...[0m

added 1 package, removed 1 package, and audited 9 packages in 825ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
[34m
╔════════════════════════════════════════╗[0m
[34m║  ekoDB JavaScript Examples Test Suite ║[0m
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
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/simple_crud.js ===[0m
=== Simple CRUD Operations (Direct HTTP) ===

✓ Authentication successful

=== Insert Document ===
Inserted: {
  id: 'ISzee-FEEREEXNQaxUTTwQwTksXEeSeJZrDxWS_uweUbJl9ox73YRrg35tGZEuIOQOW5SnGwiIQzSB7uFhkMdw'
}

=== Find by ID ===
Found: {
  name: 'Test Record',
  value: 42,
  active: true,
  id: 'ISzee-FEEREEXNQaxUTTwQwTksXEeSeJZrDxWS_uweUbJl9ox73YRrg35tGZEuIOQOW5SnGwiIQzSB7uFhkMdw'
}

=== Find with Query ===
Found documents: [
  {
    value: { type: 'Integer', value: 42 },
    active: { type: 'Boolean', value: true },
    id: 'ISzee-FEEREEXNQaxUTTwQwTksXEeSeJZrDxWS_uweUbJl9ox73YRrg35tGZEuIOQOW5SnGwiIQzSB7uFhkMdw',
    name: { value: 'Test Record', type: 'String' }
  }
]

=== Update Document ===
Updated: {
  name: { type: 'String', value: 'Updated Record' },
  active: { value: true, type: 'Boolean' },
  id: 'ISzee-FEEREEXNQaxUTTwQwTksXEeSeJZrDxWS_uweUbJl9ox73YRrg35tGZEuIOQOW5SnGwiIQzSB7uFhkMdw',
  value: { type: 'Integer', value: 100 }
}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
[32m✓ simple_crud.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/simple_websocket.js ===[0m
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: pAed3XzykALKu2H_RZtGGgHWD-bpVXZLcLZxJGJpsb93YUB8UNz3ACCCfeXVQVzH5g9c6HiCULpql6XqxYV7Gg

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
        "id": "V-GwO5brmP15gkZCWnGtp4o8ngZYVpyshElKjClatQ0lIDPngkQSMEFFFJNE6mPPBwj0_pYxOs-mnpoye8qKTw",
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
        "id": "9EeVbFUN6VQJg44lIJj9v3V6ea92aTPNLFIVKep_EE20y-vFENwg8lMAnyr-UA0MPFZS97bncRRXhrpvyMEw2Q",
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
        "id": "pAed3XzykALKu2H_RZtGGgHWD-bpVXZLcLZxJGJpsb93YUB8UNz3ACCCfeXVQVzH5g9c6HiCULpql6XqxYV7Gg",
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
        "id": "2ohhdIKZs8t7QdU7ZWUXCc4EPm8T-Iy-RVYfPio_EofPFq65P9o3SaJhKtwl8h8xuajQE9DK3IsJAV0tEM2-dw",
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
✓ Retrieved 4 record(s) via WebSocket

✓ WebSocket example completed successfully
WebSocket closed
[32m✓ simple_websocket.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/http_functions.js ===[0m
🚀 ekoDB Scripts Example (JavaScript/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: RpEJTqp-WLuYB8u2InNl6LaxCaaJ3VaAlm1GF5QFvo-th4P8qN5pWelMprqOW2lYZKD4FtoI8h0w1lb5H0r84g
📊 Found 30 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: aWeSLlwiC4KKZx8MrU34XxtTpaGerQm8YtRtEPZriTjtuG5gG5yBzlzMk9PK3HpAm-qZmABRuXcUWU1EzOK8zg
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: vtWb99YwZI-nMiHgQJ5DnVezswnsvYWwh80QScwbn5oNx0MMSqsXuZCsg2uptXz2lU7ZoGwv79PmDCB-YTGiuA
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"count":10,"status":"active","avg_score":60,"max_score":100}
   {"count":10,"status":"inactive","max_score":90,"avg_score":50}

📝 Example 4: Function Management

📋 Total scripts: 52
🔍 Retrieved script: Get Active Users
✏️  Script updated
🗑️  Script deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

✅ All examples completed!
[32m✓ http_functions.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/batch_operations.js ===[0m
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
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/kv_operations.js ===[0m
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
cache:product:3: { name: 'Product 3', price: 49.989999999999995 }

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
[32m✓ kv_operations.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/collection_management.js ===[0m
=== Collection Management (Direct HTTP) ===

✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: RgSYgzmhcKvckPObs_5VCzF_puACzUO0Z1Fazmlt5swT6sfqh-Mg2ykj1MwPxyPkWDkSo26cBVgZc1f7k9XpZg

=== List Collections ===
Total collections: 24
Sample collections: [
  'users',
  'schema_users_client_go',
  'ttl_cache',
  'test_collection',
  'schema_documents_client_go'
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
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/transactions.js ===[0m
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 4F2irs6s2JLgJEGrgNK_x9TXJ2ZRS4gQNOaZA-4q9-Y30V9vsFg5AUe7m1rs2xGzLHTsGgUQKHHDZyoS-IczHw
Created Bob: $500 - ID: XYEptgKpegt81ET44b9xgE7UHQQoagQsfe8MtO3kQx7c5LJUATyhLfQaDEXoHU8l30gGsw2mk0IMqIdX1v27GQ

=== Example 1: Begin Transaction ===
Transaction ID: 6e93353a-ac0b-4dbd-8c61-5bf54016dc97

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
New transaction: 2487875a-c0bc-43f3-ba6a-d27032b0955b
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: 700

=== Cleanup ===
✓ Deleted test accounts

✓ All transaction examples completed
[32m✓ transactions.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/crud_scripts.js ===[0m
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
   ✅ Script saved: xfujGgLFUVrAYktJ_lc43poyAaD42EfmT-6fGw1acRFkK2-D7CCO9_z78zgWtSPfvvWmAnfxS0cmgCfkUXL5UQ

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Name: [object Object]
   📋 Email: {"value":"alice@example.com","type":"String"}
   📋 Status: [object Object]
   📋 Credits: [object Object]

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: tZ37Ez2CFjEoUUZowXGNDF7IgDXAoqfOpigfJextgCSasvPixT3_gPoklaPCe_il-sd3MDcZCJSuk4ypSfubkg

2️⃣ Calling Script (Query + Update + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Status updated to: {"value":"active","type":"String"}
   📋 Name: {"type":"String","value":"Alice Smith"}

============================================================
📝 Script 3: Query + Update Credits + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: -wVqivgz5ApapYZ6YLfMQ7mxZmif6yLp0NalzfRllLwA2uQr7CiMjsaVfE9R_wZRfq7Z9CI6GMXoNDN_D2-1xg

2️⃣ Calling Script (Query + Update Credits + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {"value":0,"type":"Integer"}
   📋 Status: {"type":"String","value":"active"}
   📋 Name: {"type":"String","value":"Alice Smith"}

============================================================
📝 Script 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: 8JH57yUyosor5FTTUwUCwTyVx7Sslw5NMpNUFDRlYMrsoeLxophPMEQSHIyiplna6-UUJbqmQK2lpi7oSe1rkw

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: xfujGgLFUVrAYktJ_lc4...
   ✅ Deleted script: tZ37Ez2CFjEoUUZowXGN...
   ✅ Deleted script: -wVqivgz5ApapYZ6YLfM...
   ✅ Deleted script: 8JH57yUyosor5FTTUwUC...
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
[32m✓ crud_scripts.js completed successfully[0m
[34m
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl/document_ttl.js ===[0m
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
  Output: Document ID = wfOJDtIiRJVKN_HbulI7B5-QoiuWuXXu8keYH2dXFrCpYA3izHGoEUK_ZCBCmdMBO9Us-0kP3-1o5bCh95ug_A
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: findById(wfOJDtIiRJVKN_HbulI7B5-QoiuWuXXu8keYH2dXFrCpYA3izHGoEUK_ZCBCmdMBO9Us-0kP3-1o5bCh95ug_A)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: findById(wfOJDtIiRJVKN_HbulI7B5-QoiuWuXXu8keYH2dXFrCpYA3izHGoEUK_ZCBCmdMBO9Us-0kP3-1o5bCh95ug_A)
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
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/ttl/websocket_ttl.js ===[0m
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
  Output: Document ID = aJUIgHfCNqa50oCrnRjZjFBF9WFpOz_Lu53gygVBFC5heoVO3hCVp3C2L9czI59F6J4dL0E-lsDNzUrQUOytDw
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: findById(aJUIgHfCNqa50oCrnRjZjFBF9WFpOz_Lu53gygVBFC5heoVO3hCVp3C2L9czI59F6J4dL0E-lsDNzUrQUOytDw)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: findById(aJUIgHfCNqa50oCrnRjZjFBF9WFpOz_Lu53gygVBFC5heoVO3hCVp3C2L9czI59F6J4dL0E-lsDNzUrQUOytDw)
  Output: Error (expected) - Request failed with status 404: {"error":"Record not found (expired)"}
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

> @ekodb/ekodb-client@0.8.0 prepare
> npm run build


> @ekodb/ekodb-client@0.8.0 build
> tsc


up to date, audited 58 packages in 1s

15 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.8.0 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning JavaScript client library examples...[0m

added 1 package, removed 1 package, and audited 9 packages in 822ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'c-J7FjA45T_28eBWz3d1MsJOhyi3t2ZuRHP_TxqBNTrt5pKI3IXcL9dFEGmcVq4XNNEyKBgUY5IzLplRrehpeQ'
}

=== Find by ID ===
Found: {
  name: 'Test Record',
  active: true,
  value: 42,
  id: 'c-J7FjA45T_28eBWz3d1MsJOhyi3t2ZuRHP_TxqBNTrt5pKI3IXcL9dFEGmcVq4XNNEyKBgUY5IzLplRrehpeQ'
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  name: { type: 'String', value: 'Updated Record' },
  value: { type: 'Integer', value: 100 },
  id: 'c-J7FjA45T_28eBWz3d1MsJOhyi3t2ZuRHP_TxqBNTrt5pKI3IXcL9dFEGmcVq4XNNEyKBgUY5IzLplRrehpeQ',
  active: { type: 'Boolean', value: true }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: 1kSPwiBAnchWWTAr0NR_C79j4FE0s5_V6ryt5vgbiJGYU5n5FP-uH7ck-xW0a49BqkmyhacK_4rat_QTy267gw

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
Collection created with first record: qWaaWgb2rqv_zUJ_u7YoJFivxmWDTX2BiWmsju3vXNT0Sqys5AGFPrNC8PoltFo9NBVI_2BlQqIWHsP1pdYMew

=== List Collections ===
Total collections: 23
Sample collections: schema_users_client_go,ttl_cache,client_collection_management_js,test_collection,schema_documents_client_go

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
cache:product:1: { name: 'Product 1', price: 29.99 }
cache:product:2: { price: 39.989999999999995, name: 'Product 2' }
cache:product:3: { price: 49.989999999999995, name: 'Product 3' }

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 0 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 10

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: false

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: aJpn6jnSpJFfkmd27PZfaAmFvFSYIqSQaCcgqs3BmNr7Ze0pX2c8h_U2rRESjqhXlhhcS24yPQSxJACGDpcZag
Created Bob: $500 - ID: UvFeQEP1jn70j9pz2NJb0A7ZZGXS1xxT8wSq2Qea106Qc_soYBm-k_ngOmMzO7l6PBQRXYzMN5iecq83bDUNcw

=== Example 1: Begin Transaction ===
Transaction ID: 94259ab8-9359-4015-96ae-3f811a774e85

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: ba3da1fb-02a9-40fe-acb7-d9c4c557423d
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
  1. Score: 25.740, Matched: name.value, email, email.value, name
  2. Score: 12.540, Matched: name.value, name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: bio.value, bio, title.value, title
  2. Score: 26.400, Matched: bio.value, bio, title, title.value
  3. Score: 26.400, Matched: title, title.value, bio, bio.value
  4. Score: 26.400, Matched: title.value, title, bio, bio.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio, title.value, title, bio.value
  2. Score: 39.600, Matched: title, bio.value, title.value, bio
  3. Score: 39.600, Matched: title, bio, bio.value, title.value
  4. Score: 39.600, Matched: title.value, bio.value, bio, title

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.777, Matched: 
  2. Score: 0.759, Matched: 
  3. Score: 0.733, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.693, Matched: title.value, title, content.value, content
  2. Score: 1.511, Matched: content, title, title.value, content.value
  3. Score: 0.304, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: skills.value, bio.value, bio, skills

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
✓ Inserted document: S9fu35Ghe_vJ9Y7se1fPe7OO7g83FLsrDgx_OFFr51bkN1Hlz3lgAIhph0URqHYfvcHN--R2T5_YOBWg2y7qhg

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: UHVnPXecXb8X7FLu6vSMcTEx1S7Y8r24B6x6j0XLbbXMXFLz1MwYFHOJCEsVnarMCeZvkAfSH997F6iXDey5kQ

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
✓ Inserted document with TTL: yMYODEYHc2fTpFBTno_9tXpgs3SuuNG_j7JEEcRZdkaGijyZJsdOHxWm4vQPkXCdIW9iDjWxJlMjrRTp4Kq7RA

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
✓ Edge cache script created: LAL2bE-ApV3eFbz0bQY5h6BCoHh0mgiOUJP6heILjo26nH8LM1cC2YSN3L6JzC3wehymyTbywYf40vuzCegQYA

Call 1: Cache miss (fetches from API)
Response time: 4ms
Result: {
  "records": [],
  "stats": {
    "input_count": 0,
    "output_count": 0,
    "execution_time_ms": 0,
    "stages_executed": 2,
    "stage_stats": []
  }
}

Call 2: Cache hit (served from ekoDB)
Response time: 5ms (0.8x faster!)
Result: {
  "records": [],
  "stats": {
    "input_count": 0,
    "output_count": 0,
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

🚀 ekoDB Scripts Example (JavaScript Client)

✅ Client initialized (token exchange automatic)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: XBTPRj29EzpStEiourK7bTJBJAm5Z0xYqU4vIbVgij1cqXD7EwrKBiFyd2F86x-Lq5yWTH5GoXCswaGGHYmx9Q
📊 Found 10 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved
📊 Found 10 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved
📊 Statistics: 2 groups
   {"status":"inactive","avg_score":50,"count":5}
   {"status":"active","count":5,"avg_score":60}
⏱️  Execution time: 0ms

📝 Example 4: Script Management

📋 Total scripts: 55
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
   • ChatMessage helpers
   • Cleaner, more maintainable code
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
   Name: {"value":"User 1","type":"String"}
   Department: {"type":"String","value":"engineering"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using CallFunction to replace inline logic in SWR pattern...

✅ Saved reusable function: fetch_and_store_user
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 78ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 77ms
   📊 Records: 1
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
🚀 ekoDB JavaScript Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved: cnz94JF-bi569KXoSmWyxq03bsC13rBvfM7wKaGJTz0izboiuAj5guDivfXAT27ufrKgW3J5C2v09i844pBdLQ
📊 Found 2 product groups
   {"category":"Electronics","avg_price":575.6666666666666,"count":3}
   {"count":2,"category":"Furniture","avg_price":474}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved
📊 Found 2 categories
   {"count":3,"category":"Electronics"}
   {"category":"Furniture","count":2}
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
🚀 ekoDB JavaScript KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: fSpdhmtzotcbZe5dqLmXeOebk8cZqqpdC64XcA7_CjW_DFAh8zePYOLzhOD4zJJUhnnFQxz9oS36M1VCDPg26Q
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: A_kdW2GLnrWI1PEst4hLAolbUJ1sRR6uQKvXgy36OPcEDbwHWLRk4wgNW1rZqAgUIVRL6ypNYO5OONIO7G8wCA
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"userId":"user_abc","role":"admin"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: QboE5q5KvlvPLRNl0NiG_JxsRKzHmBxap-lDunnGk93BLzMDMutDC66nicB0_0sU-sWDpU7blkxD8q5XnldhzQ
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Script

✅ Script saved: NVVA6C5ZBV_fKbQfEPAdYdp-ZlIz_8LxMUjfdjXEaMilG7w_T1WmL2MDOwEEXYt9AhbSeubUee9EcRV_XUEGWA
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
🚀 ekoDB CRUD Scripts Example

📋 Setting up test data...
✅ Created 10 test users

📝 Example 1: List All Users

✅ Script saved
📊 Found 10 users
⏱️  Execution time: 0ms

📝 Example 2: Count Users by Status

✅ Script saved
📊 User counts by status:
   active: 7 users
   inactive: 3 users
⏱️  Execution time: 0ms

📝 Example 3: Average Score by Role

✅ Script saved
📊 Average scores by role:
   user: 70.0 (7 users)
   admin: 20.0 (3 users)
⏱️  Execution time: 0ms

📝 Example 4: Top Users by Score

✅ Script saved
📊 Users (showing first 5 of 5):
   1. User 1 - Score: 20
   2. User 2 - Score: 40
   3. User 4 - Score: 80
   4. User 3 - Score: 60
   5. User 5 - Score: 100
⏱️  Execution time: 0ms

📝 Example 5: User Summary Statistics

✅ Script saved
📊 User summary (2 groups):
   inactive users: 3 users, Total Score: 180
   active users: 7 users, Total Score: 370
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD script examples completed!
🚀 ekoDB Search Scripts Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Script saved
📊 Found 5 documents
   1. Introduction to Machine Learning (AI)
   2. Vector Databases Explained (Database)
   3. Getting Started with ekoDB (Database)
   4. Database Design Principles (Database)
   5. Natural Language Processing (AI)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Script saved
📊 Documents by category:
   Database: 3 documents
   AI: 2 documents
⏱️  Execution time: 0ms

📝 Example 3: Select Specific Fields

✅ Script saved
📊 Document titles (5 docs):
   1. Introduction to Machine Learning
   2. Vector Databases Explained
   3. Getting Started with ekoDB
   4. Database Design Principles
   5. Natural Language Processing
⏱️  Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Script saved
📊 Projected documents (showing first 3):
   1. Introduction to Machine Learning
   2. Vector Databases Explained
   3. Getting Started with ekoDB
⏱️  Execution time: 0ms

📝 Example 5: All Document Fields

✅ Script saved
📊 All documents (5 total, showing first 2):
   1. Introduction to Machine Learning (AI)
   2. Vector Databases Explained (Database)
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples completed!
🚀 ekoDB Advanced Scripts Example

📋 Setting up test data...
✅ Created 10 products

📝 Example 1: List All Products

✅ Script saved
📊 Found 10 products
⏱️  Execution time: 0ms

📝 Example 2: Group Products by Category

✅ Script saved
📊 Found 2 categories
   Electronics: 6 items (avg $325.67)
   Furniture: 4 items (avg $294.00)
⏱️  Execution time: 0ms

📝 Example 3: Count All Products

✅ Script saved
📊 Total products: 10
⏱️  Execution time: 0ms

📝 Example 4: Multi-Stage Aggregation

✅ Script saved
📊 Category analysis (2 categories):
   Furniture:
      Products: 4 | Stock: 43 | Avg Rating: ⭐4.26
   Electronics:
      Products: 6 | Stock: 232 | Avg Rating: ⭐4.52

⏱️  Total execution time: 0ms
📈 Pipeline stages:

📝 Example 5: Project Specific Fields

✅ Script saved
📊 Product summaries (10 items, showing first 3):
   1. USB-C Cable - $19 (⭐4.3)
   2. Desk Chair - $349 (⭐4.2)
   3. Keyboard - $89 (⭐4.4)
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced script examples completed!
🚀 ekoDB AI Scripts Example

⚠️  Note: These examples require AI API credentials (OpenAI, etc.)

📋 Setting up test data...
✅ Created 3 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
🤖 AI Response:
   Vector databases offer several benefits including:

1. High Precision: They store data in points, lines, and polygons, providing detailed and accurate spatial data representation.

2. Scalability: Vector data remains consistent in quality even when scaling up or down, unlike raster data that can lose quality when zoomed in.

3. Less Storage: Vector data typically requires less storage space compared to raster data for the same geographical area.

4. Attribute Storage: Vector databases can store attribute data directly, allowing for more complex analyses.

5. Flexibility: They allow for easy modification and manipulation of data such as changing the shape, size, and location of map features.
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embedding script saved
📊 Generated embeddings for 3 articles
   1. "Draft Article" - 1536D vector
   2. "Advanced Query Patterns" - 1536D vector
   3. "Getting Started with ekoDB" - 1536D vector
⏱️  Execution time: 0ms

📝 Example 3: List All Articles

✅ Script saved
📊 Found 3 articles
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All AI examples completed!
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_api_user_js (Wq1gwl5Z5Uzf1w4c_NMMNhCpGXMH_VmSlJy6U5Ui6tBONdta0CfDWnRmDfPXwAKSQZTjkO9sjHRtD7AM4OTlmQ)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [],
  "stats": {
    "input_count": 0,
    "output_count": 0,
    "execution_time_ms": 0,
    "stages_executed": 2,
    "stage_stats": []
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
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: iX4A4qD9JgfBih1Vxc054xhEkAmXKkEOWhaoZIGsirD_1UyyTINJTonb8AKDaBr1PKP9wm9BMQn6QlVCEt2PzA

=== Sending Chat Message ===
Message ID: eYmcuV9ntiiAhiSUeXJf3z18Fax0hXrrCmyKhQxDbAQM_3GQAZLZSBDay3rOhbKEZwdpuEAy31-OVO2Jm47g6Q

=== AI Response ===
There are three products available:

1. ekoDB is a high-performance database product with AI capabilities. It's priced at $99.

2. ekoDB Pro is the enterprise edition product with advanced features. It's priced at $299.

3. ekoDB Cloud is a fully managed cloud database service product. It's priced at $499.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_js',
  record: {
    id: '-JXHF5h8NhEbr8vEGb8HDhynonygnQS46rBoCyHJ4uUSrRXYpM3iVjEwfaD__QcBTMxKa4QlTcF3DAe-H8zxDQ',
    name: 'ekoDB',
    price: 99,
    description: 'A high-performance database product with AI capabilities'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_js',
  record: {
    price: 299,
    name: 'ekoDB Pro',
    description: 'Enterprise edition product with advanced features',
    id: 'cCYlB5gQ7-2QIhhTsp3RHjTb5R3rEo2_fRCHKni2xF2yPOVpQGc4nVkdW3AZz-0esPkMbFZ15yMa1F1NcxsQDA'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_js',
  record: {
    name: 'ekoDB Cloud',
    price: 499,
    id: 'MPL2qnoaPthYmEyB8wXqYOTjVUiCihIo6ioammpE0RdN3Ubkh8LUPMN1-3yog72T_OVcO4eCR7ZQ_iWi0QOMxw',
    description: 'Fully managed cloud database service product'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 2628ms

=== Token Usage ===
Prompt tokens: 461
Completion tokens: 72
Total tokens: 533

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: n9-pVMckb91RgK7JEeUbBtXlaJAde7ZCcB6XKKRvWMZa8dlxAO8mHZQfp9ztERdDk6b98v0wCh5q5943_na9pw

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information provided, the available product is ekoDB. It is a high-performance database product and it is priced at $99.

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
✓ Created second session: jUuIgQX4hgXbXDkD2fLLZVkHRXJV5K_MCRjq0VG5muzbwfwsDEbPJ1OjXEzhcNLV6-XNA9STIWTXfWhLh6taLA
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
✓ Created session: cje7M5VYe_B4Zahi741PxUf0cipLy1QyBHl_Djx3ExP8edV8OsSXencho0zBtJPhWX-ocFHTwxwCbpNGJuNsWQ

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information given, there is a product available named "ekoDB". It is a high-performance database product priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: zbBlLGXCJ1p1p3njqVgtwaIDQ42JNj3sqg2nuxkRBbfA0Z6u-EYAsA3TLTPB0g23ZoEApMDcJigzxq28dlALtQ
  Parent: cje7M5VYe_B4Zahi741PxUf0cipLy1QyBHl_Djx3ExP8edV8OsSXencho0zBtJPhWX-ocFHTwxwCbpNGJuNsWQ

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: zbBlLGXCJ1p1p3njqVgtwaIDQ42JNj3sqg2nuxkRBbfA0Z6u-EYAsA3TLTPB0g23ZoEApMDcJigzxq28dlALtQ (Untitled)
  Session 2: cje7M5VYe_B4Zahi741PxUf0cipLy1QyBHl_Djx3ExP8edV8OsSXencho0zBtJPhWX-ocFHTwxwCbpNGJuNsWQ (Untitled)
  Session 3: jUuIgQX4hgXbXDkD2fLLZVkHRXJV5K_MCRjq0VG5muzbwfwsDEbPJ1OjXEzhcNLV6-XNA9STIWTXfWhLh6taLA (Untitled)
  Session 4: iX4A4qD9JgfBih1Vxc054xhEkAmXKkEOWhaoZIGsirD_1UyyTINJTonb8AKDaBr1PKP9wm9BMQn6QlVCEt2PzA (Untitled)
  Session 5: adrkqaUlLO292DEXjF9g-V8TStYqHCmoKU2zeyvk7CFpO9XTqFR3qss8ZtZSXRoYXns5MrvgmZt6YarMyvxcIA (Untitled)
  Session 6: fwouIl2cr-G6N6vNT0z14MolEjcyX2FSbskE7sSUhtpFrghvIfYLMZYeoj-7-HKa8zhavOf0bf0XN3kitgzqkQ (Untitled)
  Session 7: doaPGJAF7e3z4i0nEIk9aJm3ASjNJ51v1ObUWXku0_s4SqDqnEBotSJ12_4wIanyzkSEcPSqTXEm0jVd_xM1Uw (Untitled)
  Session 8: XEMmWa39EXymaiyPjWK2XGee5O1GdzWDrVv-XIqEBYq4pg3FFS9TKL-u_VHtaFuYOaC-fjydSlX8mx-hHZg8uw (Untitled)
  Session 9: L2n3N8nWclhIqgCg_HvoXpytdb71VhqTt_bw1_KjLcMzxTom1LQ9TYXYlaiJBahUKc_Q6LPU6nAMWPHPP4g4Zw (Untitled)
  Session 10: cpHlk-ADD4R88a2saVoQSXnT-rPHLY-3TLG0Q-4D1KmO5jf_Yq15-g-qFDRzbar4P_XZYuWWMmUL3QOqQXwdeQ (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: zbBlLGXCJ1p1p3njqVgtwaIDQ42JNj3sqg2nuxkRBbfA0Z6u-EYAsA3TLTPB0g23ZoEApMDcJigzxq28dlALtQ

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
✓ Inserted user: EkoRecord(fields={id=StringValue(value=0LS2iu00tkfF96xEpfQ6N57DYE_o76nsFKC9ypXHkAZ0a9zXSD957nc3lNs37wugVKNMb6K0Zae-yiuxRoSWIg)})
  User ID: 0LS2iu00tkfF96xEpfQ6N57DYE_o76nsFKC9ypXHkAZ0a9zXSD957nc3lNs37wugVKNMb6K0Zae-yiuxRoSWIg

=== Read ===
✓ Found user by ID: EkoRecord(fields={data=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)]), name=StringValue(value=Alice Johnson), price=FloatValue(value=99.99), created_at=StringValue(value=Mon Jan 26 03:03:33 EST 2026), tags=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)]), metadata=ObjectValue(value={nested=ObjectValue(value={deep=BooleanValue(value=true)}), key=StringValue(value=value)}), user_id=StringValue(value=550e8400-e29b-41d4-a716-446655440000), embedding=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)]), email=StringValue(value=alice@example.com), active=BooleanValue(value=true), categories=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)]), id=StringValue(value=0LS2iu00tkfF96xEpfQ6N57DYE_o76nsFKC9ypXHkAZ0a9zXSD957nc3lNs37wugVKNMb6K0Zae-yiuxRoSWIg), age=IntegerValue(value=28)})

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Alice Johnson
  email (String): alice@example.com
  age (Integer): 28
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): Mon Jan 26 03:03:33 EST 2026
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1, tag2, tag3]
  metadata (Object): {nested={deep=true}, key=value}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): [electronics, computers]
  data (Bytes): 11 bytes
Record fields: data, name, price, created_at, tags, metadata, user_id, embedding, email, active, categories, id, age

=== Update ===
✓ Updated user: EkoRecord(fields={data=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)])}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), age=ObjectValue(value={value=IntegerValue(value=29), type=StringValue(value=Integer)}), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice@example.com)}), embedding=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)])}), active=ObjectValue(value={value=BooleanValue(value=true), type=StringValue(value=Boolean)}), user_id=ObjectValue(value={value=StringValue(value=550e8400-e29b-41d4-a716-446655440000), type=StringValue(value=String)}), tags=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)])}), categories=ObjectValue(value={value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)]), type=StringValue(value=Array)}), created_at=ObjectValue(value={value=StringValue(value=Mon Jan 26 03:03:33 EST 2026), type=StringValue(value=String)}), id=StringValue(value=0LS2iu00tkfF96xEpfQ6N57DYE_o76nsFKC9ypXHkAZ0a9zXSD957nc3lNs37wugVKNMb6K0Zae-yiuxRoSWIg), city=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=San Francisco)}), price=ObjectValue(value={value=FloatValue(value=99.99), type=StringValue(value=Float)}), metadata=ObjectValue(value={type=StringValue(value=Object), value=ObjectValue(value={nested=ObjectValue(value={deep=BooleanValue(value=true)}), key=StringValue(value=value)})})})

=== Query ===
✓ Found 1 users matching query
  - EkoRecord(fields={id=StringValue(value=0LS2iu00tkfF96xEpfQ6N57DYE_o76nsFKC9ypXHkAZ0a9zXSD957nc3lNs37wugVKNMb6K0Zae-yiuxRoSWIg), metadata=ObjectValue(value={type=StringValue(value=Object), value=ObjectValue(value={nested=ObjectValue(value={deep=BooleanValue(value=true)}), key=StringValue(value=value)})}), tags=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)])}), created_at=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Mon Jan 26 03:03:33 EST 2026)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), city=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=San Francisco)}), price=ObjectValue(value={type=StringValue(value=Float), value=FloatValue(value=99.99)}), embedding=ObjectValue(value={value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)]), type=StringValue(value=Array)}), categories=ObjectValue(value={value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)]), type=StringValue(value=Array)}), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice@example.com)}), age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=29)}), data=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)])}), user_id=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=550e8400-e29b-41d4-a716-446655440000)}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)})})

=== Delete ===
✓ Deleted user with ID: 0LS2iu00tkfF96xEpfQ6N57DYE_o76nsFKC9ypXHkAZ0a9zXSD957nc3lNs37wugVKNMb6K0Zae-yiuxRoSWIg

✓ Confirmed user was deleted

=== Cleanup ===
✓ Deleted collection: kotlin_users_example

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
=== ekoDB Kotlin Client - Simple WebSocket Example ===

=== Create WebSocket Client ===
✓ WebSocket client created

=== Connect to WebSocket ===
✓ Connected to WebSocket

=== Insert Test Record ===
✓ Inserted test record

=== Find All via WebSocket ===
✓ WebSocket findAll result:
  {"payload":{"data":[{"id":"E_KgBN318xZxIg_u9Vei_B55fVdNe-y7P5BdITcgRKJtfFd0STzzcNRuC3T8vTX52Fzzj53E424Zg6NpXxTBiQ","name":{"type":"String","value":"Test User"},"status":{"type":"String","value":"active"}}]},"type":"Success"}

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
  IDs: GRjda3CX2uh6yqDgQ9ku7s3I8ZOyeQbimit4LoU_RAKX4Oy-3-iFwZxexzMHXknPwHR25t2y7AL9eLrXCr6Mug, Mlj5CLbvJ6j3ZSRdwVEZl4ynLGkCs7aZer_7pUMS3IFmZw6fhzy97kXP2B5tsqMHYGtFjGrlen8CLgZFDCfnMw, 6ftmbJD83bnENF1mTsI9406PQB7kp8mrSwhYD8ykaYU4ZU4HQzfEMtGJiGpI37_fWSdSYHkH9Q73QENwgT2AtQ...

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
✓ Found 27 collections
  - schema_products_client_js
  - users
  - schema_users_client_go
  - ttl_cache
  - schema_employees_client_js

=== Check Collection Existence ===
Collection 'kotlin_collection_example' exists: false

=== Create Collection with Schema ===
✓ Created collection with schema: kotlin_collection_example

=== Get Collection Schema ===
✓ Schema: {"fields":{"name":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null}},"version":1,"created_at":"2026-01-26T08:03:46.362784Z","last_modified":"2026-01-26T08:03:46.362784Z","bypass_ripple":false,"primary_key_alias":"id"}

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
✓ Retrieved value: {"name":"Alice","role":"admin","email":"alice@example.com"}

=== KV Set with TTL ===
✓ Set key with 10s TTL: session:abc123

=== Verify TTL Key ===
✓ Session value: {"user_id":"123","created_at":1769414630604}
  (Will expire in 10 seconds)

=== KV Batch Set ===
✓ Batch set 3 keys
  config:db: success
  config:cache: success
  config:api: success

=== KV Batch Get ===
✓ Batch retrieved 3 values
  config:db: {"value":{"port":5432,"host":"localhost"}}
  config:cache: {"value":{"enabled":true,"ttl":3600}}
  config:api: {"value":{"retries":3,"timeout":30}}

=== KV Exists ===
✓ Key exists: true

=== KV Find (Pattern Query) ===
✓ Found 3 keys matching 'config:.*'

=== KV Query (Alias for Find) ===
✓ Total keys in store: 11

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

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: LxkaYbFcr5fyI2FbFO41gNnoCaN5d7Sv0dd9Xdmkj-A4msdEGVY2Ec67LeMwQk6c3HAp1VIuoVkSBawf1ydk1g
Created Bob: $500 - ID: bXfDuPKJXHHzFrpwrKk3gHCHkdQcl5vIIEDtcN9f5y_cgmrMJPR9CR87sbdqYnp0DLqodXMrlRwmihV1Uh87FQ

=== Example 1: Begin Transaction ===
Transaction ID: 631c58c3-43ac-4e81-a4eb-b6d95d8329fa

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 0df1067f-cab1-432e-9873-06a1296e3791
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
  - ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Diana)})

=== Query 7: Contains (name contains 'a') ===
✓ Found 2 names containing 'a'

=== Cleanup ===
✓ Deleted collection: kotlin_query_example

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
=== ekoDB Kotlin Client - Search Example ===

=== Inserting Sample Documents ===
✓ Inserted 5 sample documents

=== Basic Text Search ===
✓ Search results for 'programming':
  {"results":[{"record":{"tags":"programming,rust,tutorial","title":"Rust Programming","views":416,"description":"Learn Rust programming language with hands-on examples and best practices.","id":"vFcfMgtW-mx4yfaZ7wAAN2jfKK7dJ0nsMJ3njvqlNZxrkaiPyO-YpRWZ1XPFTcuDvbTF8MZ905KXZp5E-41V2Q"},"score":19.8,"matched_fields":["description","title","tags"]},{"record":{"id":"YWQI2G6rctR-hED4ad-fOopILrNnaRz4WuoS5W_I0TXFF-s4eR8L8ZoI-ZHhcdy1w21Si99TVe14RKDtvf0hnA","tags":"programming,python,data-science","description":"Master Python for data analysis, machine learning, and visualization.","title":"Python for Data Science","views":765},"score":6.6000000000000005,"matched_fields":["tags"]},{"record":{"tags":"programming,javascript,web","id":"Eos3g0ffX3HaNzUo1Q9_2_3A7Y2x_sGqGz2YK3LmGtcUlTV2NoeCOHA8v7_rLtXt6DD_hH6y14eWAxopom_rGg","description":"Build modern web applications using JavaScript, React, and Node.js.","title":"JavaScript Web Development","views":52},"score":6.6000000000000005,"matched_fields":["tags"]}],"total":3,"execution_time_ms":1}

=== Search for 'machine learning' ===
✓ Found results for 'machine learning'
  {"results":[{"record":{"id":"aTY9iwrcVlQj80Ocq4dmh3tAL92IKCax_6HIhfQF00bnh1y3vzEtreqVfdiPq4AHfuidDuvPiFm8vG9VdFANwA","title":"Machine Learning Basics","description":"Introduction to machine learning algorithms and neural networks.","tags":"ai,machine-learning,python","views":877},"score":2.7,"matched_fields":["tags","title","description"]},{"record":{"tags":"programming,python,data-science","description":"Master Python for data analysis, machine learning, and visualization.","views":765,"id":"YWQI2G6rctR-hED4ad-fOopILrNnaRz4WuoS5W_I0TXFF-s4eR8L8ZoI-ZHhcdy1w21Si99TVe14RKDtvf0hnA","title":"Python for Data Science"},"score":1.0,"matched_fields":["description"]},{"record":{"title":"Database Design","description":"Learn database design principles, normalization, and query optimization.","views":261,"tags":"database,design,sql","id":"NDpQwd_xh3XRthRQfnm0KgtdXTrxsgNIdqhfPuSAneIPxhrTDRXxLYs0VJBHCOFqQMrbb1U5_A7zTbmXjM5-XA"},"score":0.5,"matched_fields":["description"]},{"record":{"description":"Learn Rust programming language with hands-on examples and best practices.","id":"vFcfMgtW-mx4yfaZ7wAAN2jfKK7dJ0nsMJ3njvqlNZxrkaiPyO-YpRWZ1XPFTcuDvbTF8MZ905KXZp5E-41V2Q","tags":"programming,rust,tutorial","title":"Rust Programming","views":416},"score":0.5,"matched_fields":["description"]}],"total":4,"execution_time_ms":1}

=== Cleanup ===
✓ Deleted collection: kotlin_search_example

✓ All search operations completed successfully

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
=== ekoDB Kotlin Client - Schema Management Example ===

=== Creating Collection with Schema ===
✓ Created collection 'kotlin_schema_example' with schema

=== Inserting Valid Documents ===
✓ Inserted user 1: StringValue(value=7xT3tkMLsAqowCYV3M8Bx3YPEriIBlraKCR6T1NXM52oNj8UqdDD8YnklQbmEnru97dYiYHhBZC9jrQuQUSjIQ)
✓ Inserted user 2: StringValue(value=7YsnrSV9yjN-x1CoIYmGyLFhX9s_hOaWsTFxHkIazrSvoGEFv5Z2HCgEnqxyBTFNVWCzIUXuxtn-Wo7nnpAE3g)

=== Getting Schema ===
✓ Schema for kotlin_schema_example:
  Fields: {"title":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"status":{"field_type":"String","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"email":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null}}

=== Listing Collections ===
✓ Total collections: 28
  Sample: [schema_products_client_js, users, schema_users_client_go, ttl_cache, schema_employees_client_js]

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
=== ekoDB Kotlin Client - Document TTL Example ===

=== Insert with TTL ===
✓ Inserted document with 10s TTL
  Document ID: sxc6bsGtzYsa--8kjxfaiXxff_qaV7pQ15PWBvPBZ4o0jcmHKDxGnSTNviJS1vZ-sUm9OQyR7BywYcEWw2cJow

=== Verify Document Exists ===
✓ Document found: user_id, session_id, id, created_at, ttl

=== Insert with Longer TTL ===
✓ Inserted document with 1h TTL
  Document ID: eLZygE_IWf5NGmWKNPkrF8lYwUJ1CGfKDJihvyyxPuZTWTQt0dHHRlVInfYhMsFwYR2Z54XRML2E46GII0Y26A

=== TTL Expiration ===
✓ Document will automatically expire after 10 seconds

=== Verify Long TTL Document ===
✓ Long TTL document still exists: id, value, cache_key, ttl

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
✓ Inserted document with TTL: StringValue(value=Dj77Oj-y-DzXrTuk7QOxn_r5H2Cave-k50c0FcSnpqsCqtvYRgjElhSZ_wMoVnrr8qUvwUZyKZgYqzAZWaZjOw)

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved data via WebSocket:
  {"payload":{"data":[{"created_at":{"type":"Integer","value":1769414660493},"id":"Dj77Oj-y-DzXrTuk7QOxn_r5H2Cave-k50c0FcSnpqsCqtvYRgjElhSZ_wMoVnrr8qUvwUZyKZgYqzAZWaZjOw","name":{"type":"String","value":"WebSocket TTL Test"},"ttl":"2026-01-26T09:04:20.609353Z","value":{"type":"Integer","value":42}}]},"type":"Success"}

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

Creating edge cache lookup script...
✓ Edge cache script created: KWelTuopXstXeLNfDWSV_WkE7dOaKRg2gjsk59fYwzweKM9MVMySmFYKVPp42bKoOGO_aUcSb67zfAG69tuYzQ

Call 1: Cache lookup
Found 1 cached entries
Response time: 16ms

Call 2: Cache lookup (connection warm)
Found 1 cached entries
Response time: 7ms

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
🚀 ekoDB Scripts Example (Kotlin Client)

SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
✅ Client initialized

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: gxqI_6BdkMqiz48FcE74QXsSHXOfhr9qHsWcb8DhVFNOjB3dxLZjvd3SN7Og9mvmIkhelHzc9smbobNHtliIcg
📊 Found 15 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved
📊 Found 15 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved
📊 Statistics: 3 groups
   {"count":5,"avg_score":60.0,"status":"null"}
   {"avg_score":60.0,"status":"active","count":5}
   {"avg_score":50.0,"status":"inactive","count":5}
⏱️  Execution time: 0ms

📝 Example 4: Script Management

📋 Script listing skipped (some scripts contain parameter placeholders)
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
📊 Total user count: 15
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Deleted collection
⚠️  Script cleanup skipped (some scripts contain parameter placeholders)

✅ All examples completed successfully!

💡 Key Advantages of Using the Client:
   • Automatic token management
   • Type-safe Stage builders
   • Built-in error handling

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
🚀 ekoDB Kotlin Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved: uhl5wOiXes6XLhe4osI3Wdg6hkSeI0yWsTRZ_6QWe2lkICzEjx1nGblq-cty81NRE-1XfYty3S3pMnAGk09dvw
📊 Found 2 category groups

📝 Example 2: Simple Product Listing

✅ Script saved
📊 Found 5 products

📝 Example 3: Count by Category

✅ Script saved
📊 Found 2 categories

📝 Example 4: Multi-Stage Pipeline (FindAll → Group → Count)

✅ Script saved
📊 Pipeline executed with 1 results

🧹 Cleaning up...
✅ Cleanup complete

✅ All complete script examples finished!

💡 This example demonstrates ekoDB's Script system:
   ✅ FindAll operations
   ✅ Group aggregations (Count, Average)
   ✅ Multi-stage pipelines (FindAll → Group → Count)
   ✅ Script management (save, call, delete)

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
🚀 ekoDB Kotlin KV Store & Wrapped Types Example

📋 Demonstrates:
   • Wrapped type field builders (UUID, Decimal, DateTime, etc.)
   • KV store operations (get, set, delete, exists, query)
   • KV operations within scripts
   • Combined wrapped types + KV workflows

SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
✅ Client initialized

📝 Example 1: Inserting Records with Wrapped Types

✅ Inserted order: StringValue(value=fekI8KUjg9Qn2gErY-tOoUp0Fqc7AC49bba91Fcr1iAOixTKebhta17eyseAxKfDxkT_-UbdSEr8NPex4mE0dQ)
✅ Inserted 2 products

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: UzMNiU__0QgFQdX9Ms15LQGDlyDHtEpwpVi87P11hyFRTc6eQQ-Ur5Emg0ScbdOqRFMmFGbcguhvP2ABvX370Q
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"role":"admin","userId":"user_abc"}
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: qCHHmHJBIATN0OwO4x92AhD_1kCQLh01stSRUjWZQXk8a4XUaeHZXeIh2eH_59SLru6GIitkr0EGiUSn5KhUig
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Script

✅ Script saved: 6dNZaxk-WSPt9J1Ep5v3wxHWTtyCya1RmD8AABL2U8yMrbCHIFtF-kPpdhZ3CAHKN8_YrZZ7uojdpu8IGw4p4A
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
   ✅ FunctionStageConfig.Kv* classes work within scripts

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
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Setting up cache collection...
✓ Cache entry created

Step 2: Create SWR cache lookup script
✓ Created SWR script: swr_cache_lookup_kt (brIeKx095XEOqvLgOY8WA3WRtdQRA9ghDsvaKOdZ_OJ1ea_lyohWjOADLC7x6G_c7OGVHVuPrQcmV-fLg30uyw)

Step 3: First call - Cache lookup
Found 1 cached entries
✓ Cache lookup complete

Step 4: Second call - Fast cache hit
Response time: 7ms (served from cache)
✓ Lightning fast cache hit

🧹 Cleaning up...
✓ Cleanup complete

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation

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
🚀 ekoDB Kotlin Advanced Scripts Example

📋 Setting up test data...
✅ Created 8 products

📝 Example 1: List All Products

✅ Script saved
📊 Found 8 products
⏱️  Execution time: 0ms

📝 Example 2: Group Products by Category

✅ Script saved
📊 Category breakdown:
   {"avg_price":367.0,"count":5,"category":"Electronics"}
   {"count":3,"category":"Furniture","avg_price":365.6666666666667}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced script examples finished!

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
🚀 ekoDB Kotlin AI Scripts Example

📋 Setting up test data...
✅ Created 2 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
🤖 AI Response:
   {"response":"Vector databases offer several benefits such as high precision, efficient storage for complex data, scalability, and support for various geometric operations. They maintain the topological information and can handle more detailed data like points, lines, and polygons.","tokens_used":90}
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

BUILD SUCCESSFUL in 5s
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
🚀 ekoDB Kotlin CRUD Scripts Example

📋 Setting up test data...
✅ Created 10 test users

📝 Example 1: List All Users

✅ Script saved
📊 Found 10 users
⏱️  Execution time: 0ms

📝 Example 2: Count Users by Status

✅ Script saved
📊 User counts by status:
   {"status":"inactive","count":3}
   {"count":7,"status":"active"}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD script examples finished!

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
🚀 ekoDB Kotlin Search Scripts Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Script saved
📊 Found 5 documents
   1. {"value":"Getting Started with ekoDB","type":"String"} ({"type":"String","value":"Database"})
   2. {"value":"Introduction to Machine Learning","type":"String"} ({"value":"AI","type":"String"})
   3. {"type":"String","value":"Vector Databases Explained"} ({"type":"String","value":"Database"})
   4. {"value":"Database Design Principles","type":"String"} ({"type":"String","value":"Database"})
   5. {"value":"Natural Language Processing","type":"String"} ({"type":"String","value":"AI"})
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Script saved
📊 Documents by category:
   {"category":"AI","count":2}
   {"category":"Database","count":3}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!

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
✓ Created session: b6bnHcZuNJvKdZlaKUVaUMBwA5ZbTbyVt34nd4Y_fC9Zps3WVWRi6QOg7pQZYwOVbvdCjBfN_eCYUqxExF-L1A

=== Sending Chat Message ===
✓ Chat response:
  Message ID: "O4nUR1V9XZt0jBXVNfB1PO6MkPHSOOa-xcz-SjDLVoh_ELE0dQd1Oe36v1D65xylgThYnfj3Jp_0e2RyEhyNOw"
  Responses: ["ekoDB is a high-performance database that integrates advanced features to enhance its functionality. It has intelligent caching, real-time capabilities, and AI integration, allowing it to efficiently manage and process data.\n\nOne of its notable features is the AI Chat Integration. This feature enables you to query your database using natural language, and in return, you get AI-powered responses that provide relevant context. This makes it easier to interact with your database and retrieve necessary information.\n\nAnother important feature of ekoDB is its Search Features. It supports full-text search, vector search, and hybrid search with automatic context retrieval. This ensures comprehensive and efficient search capabilities within your data.\n\nIn summary, ekoDB combines performance and advanced features to provide a robust and user-friendly database solution."]

=== Cleanup ===
✓ Deleted chat session
✓ Deleted collection: kotlin_chat_basic_example

✓ Basic chat example completed successfully

BUILD SUCCESSFUL in 8s
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
✓ Created session: 3KGAwy4pJ7Wc4Sxlq9XgVwm4CqDRZ5-U4BdfgftfSSNBEPeRHR5PObQnuZCCkWJ4UbOWqP3LviqBqshovr6S2g

=== Sending Initial Message ===
✓ Message sent
  Responses: ["Based on the information available, one product that is available is ekoDB. This is a high-performance database product priced at $99."]

✓ Second message sent

=== Regenerating AI Response ===
✓ AI response regenerated
  New responses: ["Based on the context provided, the available product is \"ekoDB\". It's a high-performance database product priced at $99."]

=== Updating Message ===
✓ Updated message content

=== Toggling Forgotten Status ===
✓ Marked message as forgotten (excluded from context)

=== Creating Second Session for Merge ===
✓ Created second session: u7w6S1fTowaIZ5zjXfwwne6v-v06y-cE3odukCdN-cxCYmUUchuBfKXFnA-TvA5sbLHKqmLwuSnSr4pnemmE5g

=== Merging Sessions ===
✓ Merged sessions
  Merged session ID: null

=== Deleting Message ===
✓ Deleted message

=== Cleanup ===
✓ Deleted chat sessions
✓ Deleted collection: kotlin_chat_advanced_example

✓ Advanced chat features example completed successfully

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
=== ekoDB Kotlin Client - Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: BjPCwYUEmFLcPe8XKCVRKyN41XXhsYQ_0NJJpAjy_DBOBq0sX_xwfsbpdCgbwpxGv5BOFA4wd5SvuElR8rJkUw

=== Sending Messages ===
✓ Message 1 sent
  Responses: ["Based on the information provided, the product available is ekoDB. It's a high-performance database product with AI capabilities. The price for this product is $99."]

✓ Message 2 sent
  Responses: ["The price of the ekoDB, a high-performance database product with AI capabilities, is $99."]

=== Getting Message History ===
✓ Retrieved message history
  Total messages: [{"chat_id":{"type":"String","value":"BjPCwYUEmFLcPe8XKCVRKyN41XXhsYQ_0NJJpAjy_DBOBq0sX_xwfsbpdCgbwpxGv5BOFA4wd5SvuElR8rJkUw"},"content":{"type":"String","value":"What products are available?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"3NbcwZP0R0xDhoRF1eJoIa3fBaAGMWpfzvJuOEJk-gKX1XcHqY-Ua6xoRVFXwCy4DA67VaVsYjG1zNXmJKwOag","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2026-01-26T08:05:24.168197Z"},"id":"A-_ut-v2T-FAA_ro5U6rRQ-sAPSuRLuZQoeOciYnxYlcBnUCHh7b6-T6QHLHOLSOn_PigqeRh2ToVBJwGeDu6w","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":34,"prompt_tokens":173,"total_tokens":207}},"updated_at":{"type":"DateTime","value":"2026-01-26T08:05:24.168197Z"}},{"chat_id":{"type":"String","value":"BjPCwYUEmFLcPe8XKCVRKyN41XXhsYQ_0NJJpAjy_DBOBq0sX_xwfsbpdCgbwpxGv5BOFA4wd5SvuElR8rJkUw"},"content":{"type":"String","value":"Based on the information provided, the product available is ekoDB. It's a high-performance database product with AI capabilities. The price for this product is $99."},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"3NbcwZP0R0xDhoRF1eJoIa3fBaAGMWpfzvJuOEJk-gKX1XcHqY-Ua6xoRVFXwCy4DA67VaVsYjG1zNXmJKwOag","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2026-01-26T08:05:24.171139Z"},"id":"1WU3iJWM1IPz-KC3e2_L7563AbaCs-Ut3Vtm5cI2cbAeLFcGsSQ-QhI6jJFLdmUH7yTgrNlFgEdGTipzNwsPWA","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":34,"prompt_tokens":173,"total_tokens":207}},"updated_at":{"type":"DateTime","value":"2026-01-26T08:05:24.171139Z"}},{"chat_id":{"type":"String","value":"BjPCwYUEmFLcPe8XKCVRKyN41XXhsYQ_0NJJpAjy_DBOBq0sX_xwfsbpdCgbwpxGv5BOFA4wd5SvuElR8rJkUw"},"content":{"type":"String","value":"What is the price?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["price"],"record":{"description":"A high-performance database product with AI capabilities","id":"3NbcwZP0R0xDhoRF1eJoIa3fBaAGMWpfzvJuOEJk-gKX1XcHqY-Ua6xoRVFXwCy4DA67VaVsYjG1zNXmJKwOag","price":99,"product":"ekoDB"},"score":0.8}]},"created_at":{"type":"DateTime","value":"2026-01-26T08:05:25.193766Z"},"id":"yLMTOIj5IUddHwGef_pIr9qcHG6kQXoSvGo20ZiVm-tS7EcetX_Dtlv9En2ZxwrzwzC9lcpT2f0A0rwikVdrDQ","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":21,"prompt_tokens":173,"total_tokens":194}},"updated_at":{"type":"DateTime","value":"2026-01-26T08:05:25.193766Z"}},{"chat_id":{"type":"String","value":"BjPCwYUEmFLcPe8XKCVRKyN41XXhsYQ_0NJJpAjy_DBOBq0sX_xwfsbpdCgbwpxGv5BOFA4wd5SvuElR8rJkUw"},"content":{"type":"String","value":"The price of the ekoDB, a high-performance database product with AI capabilities, is $99."},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["price"],"record":{"description":"A high-performance database product with AI capabilities","id":"3NbcwZP0R0xDhoRF1eJoIa3fBaAGMWpfzvJuOEJk-gKX1XcHqY-Ua6xoRVFXwCy4DA67VaVsYjG1zNXmJKwOag","price":99,"product":"ekoDB"},"score":0.8}]},"created_at":{"type":"DateTime","value":"2026-01-26T08:05:25.197440Z"},"id":"wP5wOClR6F_WxfTksW2pm5LlLE3SEktiVaCQpndxsgYUMEuE4eVfQ7yDwxNHuQ-U7--jg9RXX8Ve7akHn4v8rw","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":21,"prompt_tokens":173,"total_tokens":194}},"updated_at":{"type":"DateTime","value":"2026-01-26T08:05:25.197440Z"}}]

=== Updating Session ===
✓ Updated session system prompt

=== Listing Sessions ===
✓ Total sessions: 16

=== Branching Session ===
✓ Created branched session: 5F3EzZTdKat1-Htf1-ii_bPHJcbz_pCYbl7COmssNJTvCJlJ0gYN6dW3B7BBT8yBE1I-pQfDcvXCdI__JWPFRg

=== Cleanup ===
✓ Deleted chat sessions
✓ Deleted collection: kotlin_chat_sessions_example

✓ Chat session management example completed successfully

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
=== ekoDB Convenience Methods Example ===

SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== Native Object Creation ===
✓ Created record: EkoRecord(fields={id=StringValue(value=QlDdjjRnQeYNFqjH7b3eay0fpfJz9Y7C41OdPi7XGyT87cg7v7w2GTItKipoE4gdDpW-K4tC1-yHLSn4HMU6EQ)})

=== Upsert Operation ===
✓ First upsert (update): EkoRecord(fields={error=StringValue(value=Record not found)})
✓ Second upsert (insert): EkoRecord(fields={error=StringValue(value=Record not found)})

=== Find One Operation ===
✓ User not found (as expected)

=== Exists Check ===
✓ Record exists: false
✓ Fake record exists: false (should be false)

=== Pagination ===
✓ Inserted 25 records for pagination
✓ Page 1: 10 records (expected 10)
✓ Page 2: 10 records (expected 10)
✓ Page 3: 6 records (expected ~7)

=== Cleanup ===
✓ Deleted collection

✅ All convenience methods demonstrated successfully!

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
=== Bypass Ripple Example ===

SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
1. Basic insert (ripple enabled):
   Inserted with ripple: EkoRecord(fields={id=StringValue(value=NG1hDVC9Z6vhRWmBbsX9Ghk7qOiDDnaTZzAc2MbaS9T6xUVMPZp3Smw0br1OsL9H2v3srAMbrjWyhwluGEWvlA)})

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: EkoRecord(fields={id=StringValue(value=Iy2Ch-g5HEjQ81d1BwrcP3IaXnGmVBsHqOXK5pndDEnEYOiXoY-2M2_T-cQikTR09I9Xrl8Dq4kGjUrEm-fsLA)})

3. Update with bypass_ripple:
   Updated with bypass_ripple: EkoRecord(fields={name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Product 1)}), id=StringValue(value=NG1hDVC9Z6vhRWmBbsX9Ghk7qOiDDnaTZzAc2MbaS9T6xUVMPZp3Smw0br1OsL9H2v3srAMbrjWyhwluGEWvlA), price=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=150)})})

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: EkoRecord(fields={name=ObjectValue(value={value=StringValue(value=Upsert Product), type=StringValue(value=String)}), id=StringValue(value=NG1hDVC9Z6vhRWmBbsX9Ghk7qOiDDnaTZzAc2MbaS9T6xUVMPZp3Smw0br1OsL9H2v3srAMbrjWyhwluGEWvlA), price=ObjectValue(value={value=IntegerValue(value=500), type=StringValue(value=Integer)})})

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

BUILD SUCCESSFUL in 3s
2 actionable tasks: 1 executed, 1 up-to-date
✅ [32mKotlin client examples complete![0m
🟣 [33mKotlin Transactions...[0m
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: KKemHTfzMFQJpOcA4xDazN4NW9QpYHao9SofveKIStTNoi9oMapWOL55Da_XDtkRF8sRGAB1ScblVZfbBofaLw
Created Bob: $500 - ID: uoajro3EB79eep1s1uHIiF-gbwQQwqH8WZS7aE_thTfMp4ieIy7VAHeRv3I2tfc-U2SDNp_H1ZZygU4LGfC-XQ

=== Example 1: Begin Transaction ===
Transaction ID: 1d8dbf85-72a2-4c95-8083-3058cfb6b8f4

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 55c222b1-5d21-4c45-ab65-39ecad3d813e
Status before rollback: Active
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
✅ [32mAll Kotlin integration tests complete![0m

🤖 [36mRAG Conversation System Examples[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m

[33mPrerequisites:[0m
  1. ekoDB server running (make run in ekodb/)
  2. OPENAI_API_KEY set in server environment
  3. API_BASE_URL and API_BASE_KEY exported in your shell

[36mBuilding Rust client library...[0m
✓ Rust client built

[36mBuilding Python client bindings...[0m
🍹 Building a mixed python/rust project
🔗 Found pyo3 bindings with abi3 support
🐍 Not using a specific python interpreter
💻 Using `MACOSX_DEPLOYMENT_TARGET=11.0` for aarch64-apple-darwin by default
📦 Built wheel for abi3 Python ≥ 3.8 to /Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.8.0-cp38-abi3-macosx_11_0_arm64.whl
✓ Python client built and installed

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
    ✓ Generated embedding: 1536 dimensions in 478.657208ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 509.500667ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 400.278125ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 491.64975ms
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 229.741709ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 241.359541ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 229.814583ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 235.686917ms
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 358.318709ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 224.072375ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 511.808625ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 701.376375ms
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
    ✓ Generated embedding: 1536 dimensions in 226.640959ms
    • Function auto-cleaned up by client

→ Executing hybrid_search()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 103.945417ms

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

Writing memory-safe, high-performance database code requires a combination of good practices, the right choice between SQL and NoSQL depending on your needs, and effective optimization strategies.

1. **Choose the Right Database:** First, choose whether to use SQL or NoSQL based on your needs. If you're dealing with unstructured data, need flexible schemas, high write throughput, or horizontal scaling, opt for a NoSQL database. However, if you're dealing with structured data with well-defined relationships, require ACID transactions, or need to perform complex queries, a SQL database might be more suitable.

2. **Normalizing your Database:** Normalizing your database can improve performance and memory usage by reducing redundancy and improving data integrity. It involves dividing large tables into smaller ones and defining relationships between them using foreign keys.

3. **Optimize Your Queries:** Here are a few strategies to optimize your database queries:
    - Avoid SELECT *: Instead specify the columns you need
    - Use Indexes: Indexes can greatly improve query performance by reducing the amount of data that needs to be examined
    - Limit the Amount of Data: Use LIMIT to restrict the amount of data retrieved
    - Reduce the Number of Queries: Try to get the data you need in as few queries as possible
    - Use Joins: Join tables on their indexed or primary key columns

4. **Memory-safe Practices:** Always be conscious of memory allocation and deallocation to prevent memory leaks. In languages like C++, always use smart pointers over raw pointers to ensure automatic memory management. In garbage-collected languages like Java, always be mindful of the scope and references to objects to allow for effective garbage collection.

5. **Monitoring Tools:** Use monitoring tools to identify bottlenecks or memory leaks in your application. They can provide insights into the performance of your queries and the memory usage of your application. You can then use this information to optimize your code.

6. **Testing:** Regularly test your database code under conditions that simulate real-world usage as closely as possible. This can help you identify performance issues and memory leaks that might not be apparent under lighter loads.

Remember, the needs of every database and application are different. Your best approach will depend on your specific needs and constraints.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 206.882875ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 2408 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 240.602625ms
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
  ✓ Text search completed in 66.630459ms

✓ Found 3 messages mentioning ownership:
  1. From conv_rust_programming: Rust's key features include: memory safety without garbage collection, zero-cost abstractions, ownership system, powerful type system, and excellent concurrency support.

  2. From conv_performance: Rust's ownership system provides zero-cost memory management. Use Box for heap allocation, Rc/Arc for shared ownership, and avoid cloning large data structures. The compiler optimizes away unnecessary allocations.

  3. From conv_rust_programming: The borrow checker enforces Rust's ownership rules at compile time. It ensures that references don't outlive the data they point to and prevents data races by allowing either multiple immutable references or one mutable reference.

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
    ✓ Generated embedding: 1536 dimensions in 0.362s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.543s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.213s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.574s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.218s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.268s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.279s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.273s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.304s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.364s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.301s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.244s
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
    ✓ Generated embedding: 1536 dimensions in 0.271s
    • Function auto-cleaned up by client

→ Executing hybrid_search()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.085s

✓ Found 5 related messages across all conversations:
  1. [Score: 0.000] From conv_database_design
     Database normalization is the process of organizing data to reduce redundancy and improve data integrity. It involves dividing large tables into smaller ones and defining relationships between them using foreign keys.

  2. [Score: 0.000] From conv_database_design
     What is database normalization?

  3. [Score: 0.000] From conv_database_design
     Use NoSQL when you need: flexible schemas, horizontal scaling, high write throughput, or when working with unstructured data. SQL is better for complex queries, ACID transactions, and structured data with well-defined relationships.

  4. [Score: 0.000] From conv_database_design
     When should I use NoSQL over SQL?

  5. [Score: 0.000] From conv_performance
     How can I optimize database queries?

=== Step 4: Generating Context-Aware Response ===
✓ AI Response (with context from 3 conversations):

Writing memory-safe, high-performance database code requires a combination of good coding practices, proper database design, and efficient query execution. Here are a few suggestions:

1. **Database Normalization**: As mentioned in Context 1, database normalization is a process to reduce redundancy and improve data integrity. A well-organized database helps to speed up data retrieval and update processes, and reduces the memory footprint.

2. **Choose the right database system**: Depending on your data and requirements, choosing between SQL and NoSQL can have significant performance implications. As mentioned in Context 3 and 4, NoSQL databases are typically better for flexible schemas, horizontal scaling, high write throughput, and unstructured data, while SQL databases are better for complex queries, ACID transactions, and structured data with well-defined relationships.

3. **Optimize your queries**: As mentioned in Context 5, optimizing your database queries can dramatically improve performance. This could include using indexes effectively, minimizing the amount of data retrieved with each query, and avoiding expensive operations like joins where possible.

4. **Memory-safe coding practices**: Always release any resources that you've consumed after you're done with them. This includes closing database connections and freeing up any memory you've allocated. In languages that don't offer automatic garbage collection, improper memory management can lead to memory leaks that degrade performance over time.

5. **Use caching**: If there are often repeated queries, consider using a caching system to store and quickly retrieve the results of these queries. This can significantly reduce the load on the database and improve performance.

6. **Concurrency and Parallelism**: If your application supports it, make use of concurrency and parallelism to execute multiple queries or operations simultaneously. This could mean using multithreading or multiprocessing in your code, or using database features like parallel query execution.

7. **Batch Operations**: Instead of executing multiple similar operations one by one, try to batch them together. This can reduce the overhead of initiating and executing each individual operation.

8. **Use Prepared Statements**: Prepared statements not only enhance security by preventing SQL injection attacks, but they also improve performance as the database can reuse the execution plan for these queries.

Remember, it's important to regularly profile and monitor your database performance, so that you can identify and address any bottlenecks that arise.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.420s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 2620 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.298s
    • Function auto-cleaned up by client
✓ New conversation stored and indexed for future retrieval

=== Step 6: Cross-Conversation Search ===
Searching for messages about 'ownership' across ALL conversations...

✓ Found 3 messages mentioning ownership:
  1. From conv_performance: Rust's ownership system provides zero-cost memory management. Use Box for heap allocation, Rc/Arc for shared ownership, and avoid cloning large data structures. The compiler optimizes away unnecessary allocations.

  2. From conv_rust_programming: Rust's key features include: memory safety without garbage collection, zero-cost abstractions, ownership system, powerful type system, and excellent concurrency support.

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
    ✓ Generated embedding: 1536 dimensions in 0.231s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.274s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.387s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.455s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.311s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.367s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.231s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.330s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.378s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.217s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.287s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.240s
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
  ✓ Search completed in 0.085s
✓ Found 5 related messages across all conversations:
  1. [Score: 0.000] From conv_database_design
     Database normalization is the process of organizing data to reduce redundancy and improve data integrity. It involves dividing large tables into smaller ones and defining relationships between them using foreign keys.

  2. [Score: 0.000] From conv_database_design
     Use NoSQL when you need: flexible schemas, horizontal scaling, high write throughput, or when working with unstructured data. SQL is better for complex queries, ACID transactions, and structured data with well-defined relationships.

  3. [Score: 0.000] From conv_database_design
     What is database normalization?

  4. [Score: 0.000] From conv_database_design
     When should I use NoSQL over SQL?

  5. [Score: 0.000] From conv_performance
     How can I optimize database queries?

=== Step 4: Generating Context-Aware Response ===
✓ AI Response (with context from 3 conversations):

Writing memory-safe, high-performance database code involves several practices across different aspects of coding. Here are some tips:

1. **Normalization**: As discussed in Context 1, normalization is a crucial aspect of database design that can enhance performance by reducing redundancy and improving data integrity. Proper normalization ensures that each piece of data is stored in only one place, reducing the risk of inconsistency.

2. **Use appropriate database type**: As Context 2 explains, the choice between SQL and NoSQL can significantly impact performance. SQL is generally better for structured data with complex queries and ACID transactions. In contrast, NoSQL is more suitable if you need flexible schemas, high write throughput, or if you are working with unstructured data.

3. **Indexing**: Index your database properly. Indexes can speed up query performance by allowing the database to find and retrieve data without needing to search every row in a table.

4. **Optimize Queries**: As per Context 5, optimizing queries is critical for performance. This can involve using joins instead of multiple queries, selecting only the necessary data rather than using SELECT *, avoiding using functions in predicates, and reducing the use of temporary tables.

5. **Memory Management**: To ensure memory safety, avoid practices that can lead to memory leaks, such as neglecting to free memory that was previously allocated. Also, be mindful of using memory-intensive operations. 

6. **Concurrency Control**: Use appropriate transaction isolation levels to prevent data inconsistencies and conflicts. 

7. **Use Prepared Statements**: These can improve performance and help protect against SQL injection attacks.

8. **Batch Processing**: Instead of writing or reading one record at a time, batch processing writes or reads many records at once, which can significantly improve performance.

Remember, the specific optimizations might depend on the specific database system you are using, as different systems may have their own unique set of best practices.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.263s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 2072 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.272s
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
  ✓ Text search completed in 0.057s
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
    ✓ Generated embedding: 1536 dimensions in 0.263s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.527s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.361s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.437s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.278s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.282s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.580s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.303s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.277s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.334s
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
    ✓ Generated embedding: 1536 dimensions in 0.834s
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
    ✓ Generated embedding: 1536 dimensions in 0.222s
    • Function auto-cleaned up by client

→ Executing HybridSearch()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.085s

✓ Found 5 related messages across all conversations:
  1. [Score: 0.000] From conv_database_design
     Database normalization is the process of organizing data to reduce redundancy and improve data integrity. It involves dividing large tables into smaller ones and defining relationships between them using foreign keys.

  2. [Score: 0.000] From conv_database_design
     What is database normalization?

  3. [Score: 0.000] From conv_database_design
     Use NoSQL when you need: flexible schemas, horizontal scaling, high write throughput, or when working with unstructured data. SQL is better for complex queries, ACID transactions, and structured data with well-defined relationships.

  4. [Score: 0.000] From conv_database_design
     When should I use NoSQL over SQL?

  5. [Score: 0.000] From conv_performance
     How can I optimize database queries?

=== Step 4: Generating Context-Aware Response ===
✓ AI Response (with context from 3 conversations):

Writing memory-safe high-performance database code involves multiple factors:

1. **Normalization:** As mentioned in Context 1, database normalization can improve data integrity and reduce redundancy. This can lead to more efficient use of memory and can improve the performance of certain types of queries.

2. **Choosing the right database type:** Depending on your needs, a SQL or NoSQL database might be more appropriate (Context 3 and 4). SQL databases are typically better for complex queries and structured data with well-defined relationships. NoSQL databases are typically better for flexible schemas, horizontal scaling, high write throughput, and unstructured data.

3. **Efficient querying:** Depending on the database type, there are different ways to write efficient queries. For SQL databases, for example, you can often improve performance by carefully selecting indexes, minimizing the use of joins, and avoiding subqueries when possible.

4. **Managing database connections:** Proper management of database connections is crucial for performance and memory safety. This usually involves using a connection pool to reuse database connections, rather than opening and closing a connection for each query.

5. **Error handling and cleanup:** Always ensure that your database code properly handles errors and cleans up resources. This typically involves using try-finally blocks or similar constructs to ensure that resources are always cleaned up, even in the event of an error.

6. **Caching:** Implementing caching strategies can significantly help to reduce the load on the database and improve the performance of your application. However, it’s important to ensure that the cache is properly synchronized with the database to avoid data inconsistency.

7. **Concurrency Control:** Use proper locking mechanisms to prevent data races when multiple threads or processes are accessing and potentially modifying the same data.

Remember, the specific techniques and best practices can vary depending on the specific database system and programming language you are using.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.261s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 2088 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.320s
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
  ✓ Text search completed in 0.058s

✓ Found 3 messages mentioning ownership:
  1. From conv_performance: Rust's ownership system provides zero-cost memory management. Use Box for heap allocation, Rc/Arc for shared ownership, and avoid cloning large data structures. The compiler optimizes away unnecessary allocations.

  2. From conv_rust_programming: Rust's key features include: memory safety without garbage collection, zero-cost abstractions, ownership system, powerful type system, and excellent concurrency support.

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
    ✓ Generated embedding: 1536 dimensions in 0.344s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.35s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.319s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.323s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.8s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.231s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.322s
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
    ✓ Generated embedding: 1536 dimensions in 0.233s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.264s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.316s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.256s
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
    ✓ Generated embedding: 1536 dimensions in 0.282s
    • Function auto-cleaned up by client

→ Executing hybridSearch()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.089s

✓ Found 5 related messages across all conversations:
  1. [Score: 0.000] From conv_database_design
     Database normalization is the process of organizing data to reduce redundancy and improve data integrity. It involves dividing large tables into smaller ones and defining relationships between them using foreign keys.

  2. [Score: 0.000] From conv_database_design
     Use NoSQL when you need: flexible schemas, horizontal scaling, high write throughput, or when working with unstructured data. SQL is better for complex queries, ACID transactions, and structured data with well-defined relationships.

  3. [Score: 0.000] From conv_database_design
     What is database normalization?

  4. [Score: 0.000] From conv_database_design
     When should I use NoSQL over SQL?

  5. [Score: 0.000] From conv_performance
     How can I optimize database queries?

=== Step 4: Generating Context-Aware Response ===
✓ Context prepared from search results
✓ AI would use this context to generate comprehensive response

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.336s
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
  ✓ Text search completed in 0.05s

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

> @ekodb/ekodb-client@0.8.0 prepare
> npm run build


> @ekodb/ekodb-client@0.8.0 build
> tsc


up to date, audited 58 packages in 1s

15 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.8.0 build
> tsc

✅ [32mTypeScript client built![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning TypeScript SWR Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_api_user (PWSqk0pgHmmq51VgeiDXdnTyqsBOTQZCfT91emJAViAvySycEDo2P3r4SE4nL-2Dnkex3-TjeBpeqLbvC07HZg)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [],
  "stats": {
    "input_count": 0,
    "output_count": 0,
    "execution_time_ms": 0,
    "stages_executed": 2,
    "stage_stats": []
  }
}
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 5ms (served from cache)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_with_reviews (wnAX9fd07htDxVk--QkoXFvHmgO38mUn9w8WBauOZjCA3OEXIOt1wSCy8BGI2TW3r7EcejPwkP6dd82Dta5npA)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [],
  "stats": {
    "input_count": 0,
    "output_count": 0,
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
✓ Edge cache script created: jImqjdAtyHtWGo_mZ_oNaVFnt-7mWYOQEbIv0noErdLDDqBOurj72K2wHMZuThOr9I2rHmU2HhuLBWstIkaXCQ

Call 1: Cache miss (fetches from API)
Response time: 5ms
Result: {
  "records": [],
  "stats": {
    "input_count": 0,
    "output_count": 0,
    "execution_time_ms": 0,
    "stages_executed": 2,
    "stage_stats": []
  }
}

Call 2: Cache hit (served from ekoDB)
Response time: 4ms (1.3x faster!)
Result: {
  "records": [],
  "stats": {
    "input_count": 0,
    "output_count": 0,
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
🍹 Building a mixed python/rust project
🔗 Found pyo3 bindings with abi3 support
🐍 Not using a specific python interpreter
💻 Using `MACOSX_DEPLOYMENT_TARGET=11.0` for aarch64-apple-darwin by default
    Finished `release` profile [optimized] target(s) in 0.07s
📦 Built wheel for abi3 Python ≥ 3.8 to /Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.8.0-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel for current platform...[0m
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.8.0-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
Successfully installed ekodb-client-0.8.0
✅ [32mPython client package built and installed![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning Python SWR Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_github_user (RwrFYS2fVvB3u6N52g0PYw6y4Wqz1D3cDSXhiCZM-QJGX0_qtcvKgD4seMjHsHMXZOpKudHxkr6x_zJviyWjaw)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 155ms
Result: [
  {
    "cache_key": "torvalds",
    "cached_at": "1769414825",
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
        "followers": 280340,
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
        "public_repos": 11,
        "received_events_url": "https://api.github.com/users/torvalds/received_events",
        "repos_url": "https://api.github.com/users/torvalds/repos",
        "site_admin": false,
        "starred_url": "https://api.github.com/users/torvalds/starred{/owner}{/repo}",
        "subscriptions_url": "https://api.github.com/users/torvalds/subscriptions",
        "twitter_username": null,
        "type": "User",
        "updated_at": "2026-01-13T07:10:05Z",
        "url": "https://api.github.com/users/torvalds",
        "user_view_type": "public"
      }
    },
    "id": "1Q5xeqqzDoMFCPD-Zei81l2IDZLwn6diTabf0WTkK5fNvL8kzpZHqg880O5jnQYTbE-vNOgUoM_yqHE6wmug6Q"
  }
]
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 73ms (2.1x faster!)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (H3jGkC8Sjo3gKWguXvUPwjdHP3EuEt52gr4Xr4CWUnUdUprtcI3sKCRr7B8xqPHJRWN8fz4ml4PyEzE8OgscAQ)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: [
  {
    "cache_key": "1",
    "enriched_at": "1769414826",
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
    "id": "bvyN8INLb-e1VtXUuuAcrCvzhxRw-I9u4sRDmFXrkRfNUeXJr0qR5s1EOw90nPi4foXFdmeLmFCvZheBpCqd0g"
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
✓ Created SWR script: fetch_github_user (fd65QdD6kE3V7VXhV79vPxOHfvFWAVGHDSEHZ92WmlBaxiWTLWNi2DpzkaM4JuU27HCpZl65h02EIS-U_XJ9FQ)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 65.374667ms
Result: [
  {
    "cached_at": "2026-01-26T03:07:06-05:00",
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
        "followers": 280340,
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
        "public_repos": 11,
        "received_events_url": "https://api.github.com/users/torvalds/received_events",
        "repos_url": "https://api.github.com/users/torvalds/repos",
        "site_admin": false,
        "starred_url": "https://api.github.com/users/torvalds/starred{/owner}{/repo}",
        "subscriptions_url": "https://api.github.com/users/torvalds/subscriptions",
        "twitter_username": null,
        "type": "User",
        "updated_at": "2026-01-13T07:10:05Z",
        "url": "https://api.github.com/users/torvalds",
        "user_view_type": "public"
      }
    },
    "id": "LumpDOrFlzf6ZC21YAA5F9sVvkrVpDMZes5gWhGHT-zW38D70dszqN4HBns6LlGE4_1rfdF8u6aeqF4F1yuXUw"
  }
]
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 62.092334ms (1.0x faster!)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (aQ07AVHcZhHutjbXnAW7DzrfmJ5F2hthCwrKJD_V6aovjIHoYLslpHsvQh9Hi6LKM7e3LjppFrJguNwBXzZ0Eg)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: [
  {
    "cache_key": "1",
    "enriched_at": "1769414826",
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
    "id": "ZlUqzTt-GNFLc5zae0bUYgNTZTTXRQ9HtRJMxFI7shwIIte8sQe92rqCHuiVFFuzJeyEfaFZ-CoEbY-toGk58g"
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
cargo build -p ekodb_client
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
✅ [32mClient build complete![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning Rust SWR Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/swr_pattern`
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_github_user (YYSWnvDIuJqaCoXa7bg9kdzeROyV-vJwfDVRRqjpBtALJiDMpw_h7OUEPTNeqDGPmHsWzAmX5tQzOdmh_1NXxw)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 69ms
Result: null
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 62ms (1.1x faster!)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (jj1iGxTA7fCgkKqMutJwbraSXXUf2HWIqKmH44ZrPeE9k4qeWfuqUpSQjrzFh8ljE5nP-9MvH5af1xJT6CJe1A)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: {
  "cache_key": "1",
  "enriched_data": {
    "type": "Object",
    "value": {
      "meta": {
        "barcode": "5784719087687",
        "qrCode": "https://cdn.dummyjson.com/public/qr-code.png",
        "updatedAt": "2025-04-30T09:41:02.053Z",
        "createdAt": "2025-04-30T09:41:02.053Z"
      },
      "warrantyInformation": "1 week warranty",
      "discountPercentage": 10.48,
      "sku": "BEA-ESS-ESS-001",
      "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
      "rating": 2.56,
      "reviews": [
        {
          "rating": 3,
          "date": "2025-04-30T09:41:02.053Z",
          "reviewerName": "Eleanor Collins",
          "comment": "Would not recommend!",
          "reviewerEmail": "eleanor.collins@x.dummyjson.com"
        },
        {
          "reviewerEmail": "lucas.gordon@x.dummyjson.com",
          "rating": 4,
          "comment": "Very satisfied!",
          "date": "2025-04-30T09:41:02.053Z",
          "reviewerName": "Lucas Gordon"
        },
        {
          "reviewerName": "Eleanor Collins",
          "reviewerEmail": "eleanor.collins@x.dummyjson.com",
          "rating": 5,
          "comment": "Highly impressed!",
          "date": "2025-04-30T09:41:02.053Z"
        }
      ],
      "availabilityStatus": "In Stock",
      "stock": 99,
      "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
      "weight": 4,
      "brand": "Essence",
      "minimumOrderQuantity": 48,
      "shippingInformation": "Ships in 3-5 business days",
      "dimensions": {
        "height": 13.08,
        "width": 15.14,
        "depth": 22.99
      },
      "tags": [
        "beauty",
        "mascara"
      ],
      "returnPolicy": "No return policy",
      "images": [
        "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
      ],
      "id": 1,
      "category": "beauty",
      "title": "Essence Mascara Lash Princess",
      "price": 9.99
    }
  },
  "id": "h7U3PfoNxOe00OfICmE8GDQ9idqPgeDIG2JQuyo014eJMGjYSKkhQ2mL5I_3zyCVVEkCtjcHcyl0D4RwXTiWgw",
  "enriched_at": "1769414826"
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
✓ Created SWR script: fetch_github_user (8tjEwEv6WP19mJqGbLBcU2eQu9R8BCW_Y2v1SAT2wWJoIrUmNE1aPcvYWS-uulHjSw5ZHFdX5YjX4Mc8gI4B4g)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 80ms
Result: []
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 81ms
Cache hit was 1.0x faster!

✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (3EfdBp9ZzDtbdx9Ek0HDg-HhiwGlfUR4DTNwFUkTWrFjH5nKm-HlUJLChowcq2mRdqPW6YxYbDetZ-shTpE_yg)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: [{"id":"pwOPZ7VI37Br6_4x-gTGUqF-W3FdmAKrajiYp1OOUYUEMjPEaQx4BWrga-6xLJvtnx-X8nO3vmR1-mVi_8Qhlg","cache_key":"1","enriched_data":{"value":{"stock":99,"thumbnail":"https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp","category":"beauty","rating":2.56,"returnPolicy":"No return policy","id":1,"price":9.99,"images":["https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"],"meta":{"updatedAt":"2025-04-30T09:41:02.053Z","qrCode":"https://cdn.dummyjson.com/public/qr-code.png","barcode":"5784719087687","createdAt":"2025-04-30T09:41:02.053Z"},"minimumOrderQuantity":48,"reviews":[{"rating":3,"reviewerName":"Eleanor Collins","date":"2025-04-30T09:41:02.053Z","comment":"Would not recommend!","reviewerEmail":"eleanor.collins@x.dummyjson.com"},{"comment":"Very satisfied!","rating":4,"reviewerEmail":"lucas.gordon@x.dummyjson.com","reviewerName":"Lucas Gordon","date":"2025-04-30T09:41:02.053Z"},{"date":"2025-04-30T09:41:02.053Z","rating":5,"reviewerEmail":"eleanor.collins@x.dummyjson.com","comment":"Highly impressed!","reviewerName":"Eleanor Collins"}],"description":"The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.","warrantyInformation":"1 week warranty","weight":4,"brand":"Essence","shippingInformation":"Ships in 3-5 business days","title":"Essence Mascara Lash Princess","dimensions":{"depth":22.99,"width":15.14,"height":13.08},"sku":"BEA-ESS-ESS-001","tags":["beauty","mascara"],"discountPercentage":10.48,"availabilityStatus":"In Stock"},"type":"Object"},"enriched_at":"1769414826"}]
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


BUILD SUCCESSFUL in 3s
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
cargo build -p ekodb_client
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.07s
✅ [32mClient build complete![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning Rust Function Composition Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
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

✅ Saved reusable function: fetch_and_store_user
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 87.715291ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 73.887042ms
   📊 Records: 1
   🚀 Cache speedup: 1.2x faster!

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
✅ [32mRust function composition examples complete![0m
📦 [36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.8.0 prepare
> npm run build


> @ekodb/ekodb-client@0.8.0 build
> tsc


up to date, audited 58 packages in 1s

15 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.8.0 build
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

✅ Saved reusable function: fetch_user
✅ Saved composed function: get_user_wrapper (calls fetch_user + projects fields)

📊 Result from composed function:
   Records: 1
   Name: {"value":"User 1","type":"String"}
   Department: {"value":"engineering","type":"String"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using CallFunction to replace inline logic in SWR pattern...

✅ Saved reusable function: fetch_and_store_user
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 69ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 68ms
   📊 Records: 1
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
✅ [32mTypeScript function composition examples complete![0m
🐍 [36mBuilding Python client package...[0m
🍹 Building a mixed python/rust project
🔗 Found pyo3 bindings with abi3 support
🐍 Not using a specific python interpreter
💻 Using `MACOSX_DEPLOYMENT_TARGET=11.0` for aarch64-apple-darwin by default
    Finished `release` profile [optimized] target(s) in 0.07s
📦 Built wheel for abi3 Python ≥ 3.8 to /Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.8.0-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel for current platform...[0m
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.8.0-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.8.0
    Uninstalling ekodb_client-0.8.0:
      Successfully uninstalled ekodb_client-0.8.0
Successfully installed ekodb-client-0.8.0
✅ [32mPython client package built and installed![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning Python Function Composition Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
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

Using CallFunction to replace inline logic in SWR pattern...

✅ Saved reusable function: fetch_and_store_user
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 79.8ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 75.9ms
   📊 Records: 1
   🚀 Cache speedup: 1.1x faster!

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
✅ [32mPython function composition examples complete![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning Go Function Composition Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
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

Using CallFunction to replace inline logic in SWR pattern...

✅ Saved reusable function: fetch_and_store_user
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 77.615416ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 74.626875ms
   📊 Records: 1
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
✅ [32mGo function composition examples complete![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning JavaScript Function Composition Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
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

Using CallFunction to replace inline logic in SWR pattern...

✅ Saved reusable function: fetch_and_store_user
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 72ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 70ms
   📊 Records: 1
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
✅ [32mJavaScript function composition examples complete![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
✅ [32mAll Function Composition Examples Complete![0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m

[32mWhat you just saw - CallFunction composability:[0m
  ✓ Reusable Script building blocks
  ✓ Scripts calling other Scripts
  ✓ Clean SWR patterns via composition
  ✓ Multi-level nesting (arbitrary depth)
  ✓ No code duplication
  ✓ Single source of truth

[36mBuild complex workflows from simple pieces! 🚀[0m

