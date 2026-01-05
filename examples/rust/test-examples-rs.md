make test-examples-rust
🧪 [36mRunning Rust examples (direct HTTP/WebSocket)...[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/simple_crud`
✓ Authentication successful

=== Insert Document ===
Inserted: Object {"id": String("gyYKhV5sU7Q_WgbDSntxWBbekXg8911a-O-pc089pFo1SxE9og2uj0cUEpAeMZM0yECWtF0QseQDUGzEM4KMnw")}

=== Find by ID ===
Found: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("gyYKhV5sU7Q_WgbDSntxWBbekXg8911a-O-pc089pFo1SxE9og2uj0cUEpAeMZM0yECWtF0QseQDUGzEM4KMnw"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}

=== Find with Query ===
Found documents: Array [Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("gyYKhV5sU7Q_WgbDSntxWBbekXg8911a-O-pc089pFo1SxE9og2uj0cUEpAeMZM0yECWtF0QseQDUGzEM4KMnw"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}]

=== Update Document ===
Updated: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("gyYKhV5sU7Q_WgbDSntxWBbekXg8911a-O-pc089pFo1SxE9og2uj0cUEpAeMZM0yECWtF0QseQDUGzEM4KMnw"), "name": Object {"type": String("String"), "value": String("Updated Record")}, "value": Object {"type": String("Integer"), "value": Number(100)}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/simple_websocket`
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: "jCqqfNX1CH8oAOspCeoYtlibdQJHNY0vgR73fxLb98lsm2A2lhAlkEWs2K4JJ3fG0iHF4joH55M1tlEJ2xM6hw"

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
        "id": "jCqqfNX1CH8oAOspCeoYtlibdQJHNY0vgR73fxLb98lsm2A2lhAlkEWs2K4JJ3fG0iHF4joH55M1tlEJ2xM6hw",
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
Collection created with first record: "8uT7i7Pv3516D388l-JQgOZ1adz5lwfdaSk1YRp6458eE6O9SgE9guI15LpdObNNxRfijlQFHive-KM5lcs2kw"

=== List Collections ===
Total collections: 4
Sample collections: ["websocket_test", "test_collection", "batch_users", "demo_collection"]

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
✓ Inserted document: "tTu0cFwCDt68Kn3rOMDFaJOgGJCHwdaQbjKQPKphLhEZ6WfiP7KVmH-DjXQkkMHSahG9ZlNSPbcQl30eG2P_zg"

=== Insert Document with TTL (5 minutes - integer) ===
✓ Inserted document: "tFMIEvjkrFIGBl022GoVpdgRVg6qQxw2aMPBq7CE4kUhq1Xk-As9Q4WoyDxeT45dcTyzUplmganYTt3fn0Thgg"

=== Insert Document with TTL (30 minutes - duration string) ===
✓ Inserted document with duration string TTL: "ch2HlpIxpXKNcLrHlHONugR27rQdx0YGBRFKzFQrImEg9oBiTwIBIrPNxYEQFiv5TST3OaOjqsZ1ir-lgw8GmQ"

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
✓ Inserted document with TTL: String("FpDyhgwU3sWYlbTw7skQJnsCrTU9dtALse1m1DW8eKPLYWfMIMP6KQ1ExEiJQ7vWxJS-jjI_1J1T5YpYdMWHtA")

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

✅ Script saved: ERTxMiz_XZO31Q9TUmuiNaHXdWAHM7VTqvCZIJ_3QALryflIYKJj7tS6pr83MyonVtbueK0vou-ZS9gFJLbGUA
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: "hitl9JF3sOo_dGKjAzuwaNqyseeNlfDBrwIHYbs4Hu1Z8gM8GescrlhlIJFzPoijkqWE3t-lXtSXRiS71qP4bA"
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Complex Filter with Multiple Conditions

✅ Script saved: NNojmvIzcoNla91fZ3ILo72GVtlQ7Re-BccmJiH7w_lpEWRpSRylltilBQvn0P_xY491-ijygLudDYT60jeasA
📊 Found 3 users (status=active, score>50, sorted by score)

📝 Example 4: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: JCI2qIkFywv4ls5kYB9fWYv8en8p6SnRqZtBuONJltTY7xjBg6QcQbrwNVaCoDLTAWQjbJF1Udm9inRQzV5_Kg
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
Created Alice: $1000 - ID: 1YbfL3Q3_KjphoJENr0iya31ENA3f7w57JF5-uOq1Ix9XAuZCQ6HX6OQOiBEcMS5u4j-qpJfL5AhehB61KY4og
Created Bob: $500 - ID: evnCwt6H-dW7m-d9u4LMpx67dqdSwbaNLvpKchQJzq-wOrG4EXWMBj2ZIbAhOSNACiWhkYh67gbwGsIt2Ps5qw

=== Example 1: Begin Transaction ===
Transaction ID: e4a34485-37f1-43a1-a3f4-e517ca0645ff

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
New transaction: 5c9364bc-8127-48b3-af42-78024cfc3c02
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: 700

=== Cleanup ===
✓ Deleted test accounts

✓ All transaction examples completed
✅ [32mRust direct examples complete![0m
🛠️  [36mBuilding client library...[0m
cargo build -p ekodb_client
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.07s
✅ [32mClient build complete![0m
🧪 [36mRunning Rust client library examples...[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_simple_crud`
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: Record { fields: {"id": String("Kyh4qQFD8ifMCAsu9UG8Kjs0vVXChJKH_6oEA1gDiCXPutCz1zsioZohgZSPshgqEqEoubZcrYze9zrVc-aSQQ")} }

=== Find by ID ===
Found: Record { fields: {"metadata": Object({"type": String("Object"), "value": Object({"nested": Object({"deep": Boolean(true)}), "key": String("value")})}), "created_at": Object({"value": String("2026-01-05T15:32:11.670382Z"), "type": String("DateTime")}), "user_id": Object({"type": String("String"), "value": String("550e8400-e29b-41d4-a716-446655440000")}), "data": Object({"type": String("String"), "value": String("aGVsbG8gd29ybGQ=")}), "value": Object({"value": Integer(42), "type": String("Integer")}), "embedding": Object({"value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "type": String("Array")}), "price": Object({"value": Float(99.99), "type": String("Float")}), "categories": Object({"value": Array([String("electronics"), String("computers")]), "type": String("Array")}), "tags": Object({"type": String("Array"), "value": Array([String("tag1"), String("tag2"), String("tag3")])}), "active": Object({"type": String("Boolean"), "value": Boolean(true)}), "id": String("Kyh4qQFD8ifMCAsu9UG8Kjs0vVXChJKH_6oEA1gDiCXPutCz1zsioZohgZSPshgqEqEoubZcrYze9zrVc-aSQQ"), "name": Object({"type": String("String"), "value": String("Test Record")})} }

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Some("Test Record")
  value (Integer): Some(42)
  active (Boolean): Some(true)
  price (Decimal): Some(99.99)
  created_at (DateTime): Some("2026-01-05T15:32:11.670382Z")
  user_id (UUID): Some("550e8400-e29b-41d4-a716-446655440000")
  tags (Array): 3 items
  metadata (Object): 2 keys
  embedding (Vector): 5 dims
  categories (Set): 2 items
  data (Bytes): 11 bytes

=== Find with Query ===
Found documents: [Record { fields: {"name": Object({"value": String("Test Record"), "type": String("String")}), "tags": Object({"type": String("Array"), "value": Array([String("tag1"), String("tag2"), String("tag3")])}), "id": String("Kyh4qQFD8ifMCAsu9UG8Kjs0vVXChJKH_6oEA1gDiCXPutCz1zsioZohgZSPshgqEqEoubZcrYze9zrVc-aSQQ"), "value": Object({"type": String("Integer"), "value": Integer(42)}), "metadata": Object({"type": String("Object"), "value": Object({"key": String("value"), "nested": Object({"deep": Boolean(true)})})}), "categories": Object({"value": Array([String("electronics"), String("computers")]), "type": String("Array")}), "active": Object({"type": String("Boolean"), "value": Boolean(true)}), "user_id": Object({"type": String("String"), "value": String("550e8400-e29b-41d4-a716-446655440000")}), "embedding": Object({"value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "type": String("Array")}), "price": Object({"value": Float(99.99), "type": String("Float")}), "created_at": Object({"value": String("2026-01-05T15:32:11.670382Z"), "type": String("DateTime")}), "data": Object({"value": String("aGVsbG8gd29ybGQ="), "type": String("String")})} }]

=== Update Document ===
Updated: Record { fields: {"user_id": Object({"value": String("550e8400-e29b-41d4-a716-446655440000"), "type": String("String")}), "price": Object({"value": Float(99.99), "type": String("Float")}), "created_at": Object({"value": String("2026-01-05T15:32:11.670382Z"), "type": String("DateTime")}), "tags": Object({"type": String("Array"), "value": Array([String("tag1"), String("tag2"), String("tag3")])}), "value": Object({"value": Integer(100), "type": String("Integer")}), "name": Object({"type": String("String"), "value": String("Updated Record")}), "data": Object({"type": String("String"), "value": String("aGVsbG8gd29ybGQ=")}), "id": String("Kyh4qQFD8ifMCAsu9UG8Kjs0vVXChJKH_6oEA1gDiCXPutCz1zsioZohgZSPshgqEqEoubZcrYze9zrVc-aSQQ"), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "embedding": Object({"type": String("Array"), "value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)])}), "categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])}), "metadata": Object({"type": String("Object"), "value": Object({"nested": Object({"deep": Boolean(true)}), "key": String("value")})})} }

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_simple_websocket`
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: UMuKZNss3X0Zdy1rZsiCUOzZBQEriMKAVfec5Gcsa4XCP5pRM1dcsXlmOzf47YqR37vRDeBZqiaNTF7C1e3_sg

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

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
Collection created with first record: "HbmUohz7za7-HcLdebE9BFgmUhNyuU6N-28CLHytuvwW2jbcIFpBaynvPDSrR3a6sQgCyB8t3YMFBx8s1Mjg_g"

=== List Collections ===
Total collections: 9
Sample collections: ["ws_ttl_test", "websocket_test", "client_collection_management_rust", "test_collection", "users"]

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
Created Alice: $1000 - ID: EEUUfvvB0DzpAlAa8Z8jIKs10RWTsN807Af3gac2JYeqobPs_OC8ZdiY-HH97sBtEj395jq45CD9x3yHWjQSpA
Created Bob: $500 - ID: u5qiM5-GNJuHlDF-zLWhY9MjTwP_Sqtltl7WczEj01lFksp0MJd74gNxmIFlL54pcOWwHocW0TeTleMc7yVKOQ

=== Example 1: Begin Transaction ===
Transaction ID: b178102d-428c-453e-8d2f-470ff2dbee0c

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: "Active"
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 38fa1f88-834d-47bc-a9c5-199ad87ecc7a
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
  - Some(Object({"type": String("String"), "value": String("Alice")}))
  - Some(Object({"value": String("Bob"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("David")}))

=== Range Query (age >= 28 AND age < 35) ===
✓ Found 3 users in age range
  - Some(Object({"value": String("Bob"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("Eve")}))
  - Some(Object({"value": String("David"), "type": String("String")}))

=== IN Operator ===
✓ Found 4 users with status active or pending
  - Some(Object({"type": String("String"), "value": String("Bob")}))
  - Some(Object({"type": String("String"), "value": String("Eve")}))
  - Some(Object({"type": String("String"), "value": String("David")}))
  - Some(Object({"value": String("Alice"), "type": String("String")}))

=== NOT IN Operator ===
✓ Found 4 users not inactive

=== String Pattern Matching ===
✓ Found 5 users with @example.com email

=== Regex Query ===
✓ Found 0 users with names starting with A-C

=== Complex Query (active AND age >= 28 AND score > 1500) ===
✓ Found 2 users matching all conditions
  - Some(Object({"type": String("String"), "value": String("David")}))
  - Some(Object({"value": String("Bob"), "type": String("String")}))

=== OR Query ===
✓ Found 2 users with age < 28 OR age > 32
  - Some(Object({"value": String("Charlie"), "type": String("String")}))
  - Some(Object({"value": String("Alice"), "type": String("String")}))

=== Sorted Query (by score descending) ===
✓ Top 3 users by score:
  1. Some(Object({"type": String("String"), "value": String("Bob")}))
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
     Matched: ["title", "description.value", "title.value", "description"]
  2. Score: 2.0000
     Title: Some(String("Python for Data Science"))
     Matched: ["description", "description.value"]
  3. Score: 1.0000
     Title: Some(String("Database Design"))
     Matched: ["description.value", "description"]
  4. Score: 1.0000
     Title: Some(String("Rust Programming"))
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
Execution time: 1ms
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_schema`
=== ekoDB Schema Management Example ===

=== Creating Collection with Schema ===
✓ Created collection 'schema_client_rust' with schema

=== Inserting Valid Documents ===
✓ Inserted user 1: Some(String("B2L6eSQHkfz1yX_Zsuu2Ma1ydUmZousawPHsem028zu15Cgd_DZzGM2ChHybct5zC71NZvn0b5UM5nPGOzoX9Q"))
✓ Inserted user 2: Some(String("lSl9nAqViUR4XA1o2ZegB4oAQNSYKYeP6Dk02HH1HlmIRGbGJeFa2AArPx8l7J5IT4u50JFTq7ySznZrLO7Fog"))

=== Getting Schema ===
✓ Schema for schema_client_rust:
  - email: String
    (required)
  - title: String
    (required)
  - status: String
  - age: Integer

=== Listing Collections ===
✓ Total collections: 9
  Sample: ["ws_ttl_test", "websocket_test", "test_collection", "users", "test_accounts"]

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_document_ttl`
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: "V8XeyrAcX61qD-4YY2IVhfPasal4-H1hRH51-ZT-0s5zx5jPe7meWl4RYHVten9mOc1719xMDcNKrZRRnhOY4A"

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: Some(String("PVw1S6QCf5v2W3xkCty4IeN4WMNeVb74Sh9kC3xVx21bEui5TNybEDVu7x4EUaDsx2N5npSTWlD20LenypHWCg"))

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
✓ Inserted document with TTL: Some(String("wij1RKmWRB2tyyor2IlYxph2ZOCIl61795wFaCFqb_wKMaffWinDC7jQnMKAZqkkBVZ_ySSIIT6PrEHif9Oh1A"))

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
✓ Edge cache script created: Go3jX69WTOPhaGFZa5-x8AZuSMr1I8CeK4e2coZnS9vNmXSnML-8cpUHfaOeQ8JlRWkf6Q2husGTtDUq5PWNGA

Call 1: Cache lookup
Response time: 1ms
Found 1 cached entries

Call 2: Cache lookup (connection warm)
Response time: 0ms
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

✅ Script saved: Ozq_-PDD7Tya8jWj0P6j-Fb6dZWuM1RC9V2-rkTLNg0FpJdTyE6HtDiacQTIC5ZzXFlzl-zUE0ZxnjtjdAslQQ
📊 Found 20 active users

📝 Example 2: Parameterized Script

✅ Script saved: wuj_uC-EgTUucN37Jp28nrLCO0YaPeJ2mEGXlX8Q4mfnv29LpKccv9BrtOTwrGOZsYWn3nei5bRlFLHqLWZEtw
📊 Found 20 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: SrOvE6R7CjDe-2SNM6towrloyPbcTkA6LYRwOAHcBHQ89dsD3kK7nqBsKZLyk47e4BXG2XJIXvTkC0Gd6KL_eA
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
   Name: Some(Object({"type": String("String"), "value": String("User 1")}))
   Department: Some(Object({"value": String("engineering"), "type": String("String")}))

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using CallFunction to replace inline logic in SWR pattern...

✅ Saved reusable function: fetch_and_store_github
✅ Saved SWR function using composition: swr_github_user

First call (cache miss - will fetch from GitHub):
   ⏱️  Duration: 161.640958ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 79.519667ms
   📊 Records: 1
   🚀 Cache speedup: 2.0x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

✅ Level 1 function: validate_user
✅ Level 2 function: fetch_slim_user (calls validate_user)
✅ Level 3 function: count_validated_user (calls fetch_slim_user)

📊 Result from 3-level nested composition:
   Records: 1
   Name: None
   Department: None
   Record count: Some(Integer(1))

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

✅ Script saved: 9V1be90KoqiNn66A5ZkdaYQQgAyz-Of6aKh4_6HwhDB4qxvEAX0tkXHAWG7qkdA4aKgbxL0lW6YGaIJOVpLqUA
📊 Found 2 product groups
   Record { fields: {"category": String("Electronics"), "avg_price": Float(575.6666666666666), "count": Integer(3)} }
   Record { fields: {"count": Integer(2), "category": String("Furniture"), "avg_price": Float(474.0)} }
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved
📊 Found 2 categories
   Record { fields: {"category": String("Electronics"), "count": Integer(3)} }
   Record { fields: {"count": Integer(2), "category": String("Furniture")} }
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

✅ Inserted order: Some(String("8xou0XWpmmzj-fdUvwWOa816xNBbmGCLgqXhwPb7qZrIrsSTehdOIeRiF2IbmR8mm4UaabQLYfmsP7494Z3WFw"))
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: 3_HPg-GM7ocP1S7PXX7kW4MPcmtlhCQ7EbRvzzJ3G_DdicRopyOsBJdWdQO6FVojQXHup-uZ5v-Rifjfx0BMfg
📊 Script executed
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: Some(Object {"role": String("admin"), "userId": String("user_abc")})
🔍 Key exists: true
✅ Set cached data
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: F15AL_A8QoAXajL1EjCCaOlT5KznP8os3orevGu66pv9n5p2L9mVzRHKi7WUc8F7qcgoYhhDGjnclotzpCXGAw
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Script

✅ Script saved: TTq9aDGouedXOU-SV8zpJTAfztrQX814dkMgg5ncEmwy4Vs32fbG8iMRxyV2rlkb5DHm57CLjHJ9PErFFBGTXg
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
✓ Created SWR script: fetch_github_user_rs (iAoVU0nOrh2yyvprBIcr0fmMHb7IV7AF-K1VwCofk4MChJEzPSXdsp5TsGISXsMq3taVXY8EcR5ayto7m1N0kg)

Step 2: First call - Cache miss, fetches from GitHub API
Result: FunctionStats { input_count: 0, output_count: 1, execution_time_ms: 0, stages_executed: 2, stage_stats: [] }
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 78ms (served from cache)
✓ Lightning fast cache hit

🧹 Cleaning up...
✓ Cleanup complete

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_functions_ai`
🚀 ekoDB Rust AI Scripts Example

📋 Setting up test data...
✅ Created 2 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
🤖 AI Response:
   1. Efficiency: Vector databases allow efficient data management and retrieval due to their structured format.
2. Precision: They offer high precision in storing and representing data, especially geographical data.
3. Scalability: Vector databases can be easily scaled up or down without losing data quality.
4. Flexibility: They support various types of data (points, lines, polygons), providing flexibility.
5. Complex Analysis: These databases allow complex spatial analyses and operations like overlaying, buffering, and path finding.
6. Visualization: They provide better visualization of data, useful in fields like GIS, CAD, and computer graphics.
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
   Record { fields: {"status": String("active"), "count": Integer(7)} }
   Record { fields: {"count": Integer(3), "status": String("inactive")} }
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
   1. ? (?)
   2. ? (?)
   3. ? (?)
   4. ? (?)
   5. ? (?)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Script saved
📊 Documents by category:
   Record { fields: {"count": Integer(2), "category": String("AI")} }
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
✓ Created session: aR-Adl9yzHwPf704PXKnJCZnaJlOPXDFagZ71UxUCWKDj7zBUGseVZJaZx2ziMfvsQ20kiytyD3UqhPjfCNiVQ

=== Sending Chat Message ===
Message ID: ot649VbTlyX_d9lbby5oCVJf2Grygtqi8x72g6bWDnw976IUGIirWzF4RKSyyLwtg10d2MLWvBmlcVLvA4cKrA

=== AI Response ===
Response 1: ekoDB is a high-performance database that offers intelligent caching, real-time capabilities, and AI integration. It comes with a range of powerful features designed to provide a seamless user experience. 

One of its notable features is the AI Chat Integration. This feature allows you to query your database using natural language and get AI-powered responses with relevant context. This means you can interact with your database much like you would in a regular conversation, which could significantly enhance your productivity and efficiency.

In addition to this, ekoDB also supports full-text search, vector search, and hybrid search with automatic context retrieval. This makes it easier for you to locate specific information within your database, thus improving your ability to manage and manipulate data.

In essence, ekoDB is designed to provide an efficient, user-friendly platform for all your database needs, with a focus on real-time capabilities and intelligent AI integration.

=== Context Used (3 snippets) ===

Snippet 1:
  Collection: client_chat_basic_rust
  Score: 0.6333
  Matched Fields: ["content", "title", "category"]
  Record: Object {"category": String("features"), "content": String("The chat feature allows you to query your database using natural language and get AI-powered responses with relevant context."), "id": String("kaumlgB2dcXir7-FZNIFBJ5F0ziHvJVHaJvEINiy4I_8KKbsMAL_8La9x-BVa4MNuGE4DlBGhwx_zY0Lj8VdjA"), "title": String("AI Chat Integration")}

Snippet 2:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["title", "content"]
  Record: Object {"category": String("documentation"), "content": String("ekoDB is a high-performance database with intelligent caching, real-time capabilities, and AI integration."), "id": String("DDXOjw1T-8j55XHUg6UviHg1GyoHDsLE0yJneWpc03gT-UaCmEbzZx6aCONH1JoZE5TEEgTCR837wQwngzgL-A"), "title": String("Introduction to ekoDB")}

Snippet 3:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["category", "content", "title"]
  Record: Object {"category": String("features"), "content": String("ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval."), "id": String("UPGeVfMDpi2UjMKPfzDs1koUrG4xMVVQOf8Iisp_iywj4kiyb9H7w5Zd6gIttXVp6EEFGzzPWX4HyRReEqycJQ"), "title": String("Search Features")}

Execution Time: 6635ms

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_chat_advanced`
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: KLP3R3vWrk8eVpkLYSNCyd3D9upSlvhHO_S8lvkCf0pV-uMv6k45IltEFWb0-D36LWrk_chda1n2FEXEW74zIQ

=== Sending Initial Message ===
✓ Message sent
  Response: The available product based on the context provided is ekoDB. It's a high-performance database product.

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
✓ Created second session: 4XcmiBaLEOBsWSazHwzTs9wPONLFauBk7LZdGPM9-ARmVHIYgjyE9sNHDGnBZQ3o1ShlaUag_CEdZiYGkdgiCQ
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
✓ Created session: HlgLEuO5eVHTzp6emc9vrrKYyDWQ_IXDgKzI1_2UEdimEilMntvib7YtjlQgvo4ULsDrHzblIa1vlqaZnRedrA
=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information available, there is a product called ekoDB. It's a high-performance database product with AI capabilities. The price of this product is 99.

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: -jXiEe9uGaqOaLVm8RbOj4nPQJcLWNzZnILku1hD6wgfE2eNRDQsfHlNuNApNsEapRu_UMJ1tnUW4_izUSA6nQ
  Parent: HlgLEuO5eVHTzp6emc9vrrKYyDWQ_IXDgKzI1_2UEdimEilMntvib7YtjlQgvo4ULsDrHzblIa1vlqaZnRedrA

=== Listing Sessions ===
✓ Found 4 sessions
  Session 1: -jXiEe9uGaqOaLVm8RbOj4nPQJcLWNzZnILku1hD6wgfE2eNRDQsfHlNuNApNsEapRu_UMJ1tnUW4_izUSA6nQ (Untitled)
  Session 2: HlgLEuO5eVHTzp6emc9vrrKYyDWQ_IXDgKzI1_2UEdimEilMntvib7YtjlQgvo4ULsDrHzblIa1vlqaZnRedrA (Untitled)
  Session 3: 4XcmiBaLEOBsWSazHwzTs9wPONLFauBk7LZdGPM9-ARmVHIYgjyE9sNHDGnBZQ3o1ShlaUag_CEdZiYGkdgiCQ (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: -jXiEe9uGaqOaLVm8RbOj4nPQJcLWNzZnILku1hD6wgfE2eNRDQsfHlNuNApNsEapRu_UMJ1tnUW4_izUSA6nQ

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
✅ [32mRust client examples complete![0m
