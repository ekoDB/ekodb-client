make test-examples
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
✅ [32mAll Rust integration tests complete![0m
🧪 [36mRunning Python examples (direct HTTP/WebSocket)...[0m
=== Simple CRUD Operations (Direct HTTP) ===

✓ Authentication successful

=== Insert Document ===
Inserted: {'id': 'dcdbm6OGxfX65V3h70XvCB0PWPuuFkzEO7yIkKgO41kPT5nqshzEGWlX9uQmrdfiRnOkBQ62f5h5ZbiB-f9HWg'}

=== Find by ID ===
Found: {'name': {'type': 'String', 'value': 'Test Record'}, 'id': 'dcdbm6OGxfX65V3h70XvCB0PWPuuFkzEO7yIkKgO41kPT5nqshzEGWlX9uQmrdfiRnOkBQ62f5h5ZbiB-f9HWg', 'value': {'type': 'Integer', 'value': 42}, 'active': {'type': 'Boolean', 'value': True}}

=== Find with Query ===
Found documents: [{'active': {'type': 'Boolean', 'value': True}, 'name': {'type': 'String', 'value': 'Test Record'}, 'id': 'dcdbm6OGxfX65V3h70XvCB0PWPuuFkzEO7yIkKgO41kPT5nqshzEGWlX9uQmrdfiRnOkBQ62f5h5ZbiB-f9HWg', 'value': {'type': 'Integer', 'value': 42}}]

=== Update Document ===
Updated: {'active': {'type': 'Boolean', 'value': True}, 'name': {'type': 'String', 'value': 'Updated Record'}, 'value': {'type': 'Integer', 'value': 100}, 'id': 'dcdbm6OGxfX65V3h70XvCB0PWPuuFkzEO7yIkKgO41kPT5nqshzEGWlX9uQmrdfiRnOkBQ62f5h5ZbiB-f9HWg'}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: t72bpt21jeSg4T5Hfo6Fk9U-NA8EZPcpKDWKpNfxWbDNAhHHFhjk2RkrVZVqsb_YoUfIXZy-UOQ7xUd1LDrPFQ

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
      },
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "t72bpt21jeSg4T5Hfo6Fk9U-NA8EZPcpKDWKpNfxWbDNAhHHFhjk2RkrVZVqsb_YoUfIXZy-UOQ7xUd1LDrPFQ",
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

✅ Script saved: Q2Vco7QdhKQMxGOB3-Q6MjTnVKS2lTjly87kesssWoDQRnh9K4-Q2fX3iapG_Z-5hitDem3e8DnKqX0wSCYNKg
📊 Found 35 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: 0rl6a-uFCVIQsiqjdwaan_A1OOzlNG_62JA4yqE8oRuTX_-C22wYw9avv0oR-MhAHZlJHrUQTyPWSBYirx7pSw
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: wipRXZPeml8OWQTNQTzd7csW_Xc8zTlzZf2zTHeZDw0N8i5v5Rz74DtFKl2seTqrIGydnULmTgm49HHFh7QEsQ
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {'count': 15, 'avg_score': 60.0, 'max_score': 100, 'status': 'active'}
   {'avg_score': 50.0, 'max_score': 90, 'status': 'inactive', 'count': 15}

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
cache:product:2: {'name': 'Product 2', 'price': 39.989999999999995}
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
Collection created with first record: FBX8YSKbb0Sm4CvTeuQmAleFSMDRTaDQBT3HU2FRG7Yp27DtfRE31ynYn0AcNG_1D5q_1c5rMIkTibmxQCCcnw

=== List Collections ===
Total collections: 12
Sample collections: ['chat_configurations__ek0_testing', 'ws_ttl_test', 'websocket_test', 'test_collection', 'users']

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: False

✓ All collection management operations completed successfully
✓ Authentication successful

📋 Getting original configuration...
   Original durable_operations: False


============================================================
🔥 TEST 1: Original Config (durable=False)
============================================================

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 6y4nPqaatxZfQjJZTOzQ5kx3-4NDWVSDLfCJeFSAO25kB52DKUSeiFQ6oZCB2gZahzFhB9cdi0LCjEj9QvCHvA
Created Bob: $500 - ID: Qzb4j74yLSABtffUVwfn1yERhwW4_eWJEmqRQ-RmqZMi9ts2rJtwUgPGvSgC6hOeMCMne-LXZzrsQnpMFkxFGw

=== Example 1: Begin Transaction ===
Transaction ID: 8ac7e18d-0b18-4e37-88b2-24a1142a4497

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
New transaction: f3afd2c1-fc79-49fe-93f7-f0ea7aef532f
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: zkFlqY_FMF88AZu49LnwLvukRmLMt4YI990NZmtBKEaPya4ynrdVcUkuozpJ5uRXzapN7qA40VGxPqVZmX20ZQ
Created Bob: $500 - ID: YNUZuueY3h4sR8kmm2CIWDM1gWZHJtyc1V6oI7mUScpwyi7LwMNp-rDTVH6fZqPWAkCbRInOA4XnHmI411NjEQ

=== Example 1: Begin Transaction ===
Transaction ID: 95f55141-586b-4eae-8f84-e259e2367d2e

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
Created Alice: $1000 - ID: afZRBAth1NBO4td9xDug8DjF3sKsAb8QNM_a1DhoHDsginHDSd4JKlTZ0esvPEd_AljaTKdE7FuTQN3G3I1sfQ
Created Bob: $500 - ID: wSO6QDEutzlaCGxOS2tMvUMf4duLQn-4w8MjjzqvTMFS2d6JWY0TCyN805_foVISleCBUvg57ggh4SvLFbjkvQ

=== Example 1: Begin Transaction ===
Transaction ID: dec01d2a-4615-4991-897b-1a6103b9dca9

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
New transaction: d28ab0b3-272b-471b-8f34-2982fa313769
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: r7WNqCTcTU6gCDYIOoHgvt7ilU7g7fjBEOuN-ygifnzxUGYysMbc6GWUyn0K3JtEi2iB9INGFFAVGw5TiiFiDg
Created Bob: $500 - ID: FQ8QlNo2o7bQHhmlOrvlTZbL3znt-MpZUPZCdNGQ-E4NGXVy670M1l4Rgfhmz2Q0RqvGNITeZVUVBcEWea_mFw

=== Example 1: Begin Transaction ===
Transaction ID: 97f0731c-4323-4490-851e-6ed6d95d6d36

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
Created Alice: $1000 - ID: fiLhWYRc1ni7Il9qm6r0l2FIZOD70dvtURgg9OlQxVI7gwYFCOb_l3ozDRftmAR659aDD3-NYs20vX-n5YpSkg
Created Bob: $500 - ID: aINnbCMF5wW0aMug71tMfUnU-TzRTazTjt-e2mHmP2922JGaOW9Y-4XXBK3GmiZJ2daadu11eMOUntvvM_w0Dw

=== Example 1: Begin Transaction ===
Transaction ID: e9170096-8a55-4b06-b8ff-934f0f20a267

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
New transaction: 152b9038-17ec-483a-908d-b62526de7736
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: Cj6xXstUV4VO5eLrWrGDrZ96ZwUrkjOMRg4JVXeS1RDwkiEECaCsc5ZMj9SqZxQBBYaywiwM2tbh_7dOr_kfFw
Created Bob: $500 - ID: rBFlalGSlOBufnitvyLX2TDLBuIpLeza2eV1HmvJMccfsTOXYYppYZqzOTIK400-D0hPQ6TAkq69hE-Z4vIpcQ

=== Example 1: Begin Transaction ===
Transaction ID: 2e0e0c67-a63d-4166-9b07-9cd1f017fa95

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
   ✓ Config restored: durable_operations=False


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
   ✅ Script saved: keMLbr92zDKK2Nw2Th8wGhkeY9IkvhJT-7X8DwDQKCfbiLwSAZAoZJW6p75n_m0dos2-9BfhfvfPtMYp1UBaMQ

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: TgfN3LnXOv3ZlAcC_HPl8q0qygUrS9l6dkxmjXB1TqK8f700kVv-L7CHclhv1zB8a3dr5eENbmHn2t5K-FwWdA
   📋 Name: {'value': 'Alice Smith', 'type': 'String'}
   📋 Email: {'value': 'alice@example.com', 'type': 'String'}
   📋 Status: {'value': 'pending', 'type': 'String'}
   📋 Credits: {'type': 'Integer', 'value': 0}

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: k9FV8UqRoVCdZZW0PzH7Lm7ksZuMoiJRah24VKtXQZsWLGwp4c-64fm_Kz_NFFAYweHMK58fppxXv5eyvvyYVQ

2️⃣ Calling Script (Query + Update + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Status updated to: {'type': 'String', 'value': 'active'}
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}

============================================================
📝 Script 3: Query + Update Credits + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: V1Y6cFs7znaq7KRNTQKSJIeLX2VoJZEXCwxOou-wIpZkkeJbbMS-QDi3iLUhegaM3IwTywxJ9aayCAbWWqTCxw

2️⃣ Calling Script (Query + Update Credits + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {'value': 0, 'type': 'Integer'}
   📋 Status: {'value': 'active', 'type': 'String'}
   📋 Name: {'value': 'Alice Smith', 'type': 'String'}

============================================================
📝 Script 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: d66ar0Hkhuw5mKPWMGiR1ChwOMNSdIPkwV1VDRLpWh9m3LZ2iVZVf6ZwjWioaD9y0NSNthtSCPP5OAjJnYfQGQ

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: keMLbr92zDKK2Nw2Th8w...
   ✅ Deleted script: k9FV8UqRoVCdZZW0PzH7...
   ✅ Deleted script: V1Y6cFs7znaq7KRNTQKS...
   ✅ Deleted script: d66ar0Hkhuw5mKPWMGiR...
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
  Output: Document ID = lOTU-P0G7MvXdlXCreSatBG-_AcBNIVmjuo-IEw7couZfPXL787FHNqFFCY05Q6_mEpQQKqdF1vGo-Ipr1WVEA
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: find_by_id(lOTU-P0G7MvXdlXCreSatBG-_AcBNIVmjuo-IEw7couZfPXL787FHNqFFCY05Q6_mEpQQKqdF1vGo-Ipr1WVEA)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: find_by_id(lOTU-P0G7MvXdlXCreSatBG-_AcBNIVmjuo-IEw7couZfPXL787FHNqFFCY05Q6_mEpQQKqdF1vGo-Ipr1WVEA)
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
  Output: Document ID = RBih_U6JrqNY0lguSkPYmseTOZUJihMCpGKe_gN3kZNIk-92d1AWZ44V9932M-P5JI-DuWh14jyB55LPyQTLVg
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: find_by_id(RBih_U6JrqNY0lguSkPYmseTOZUJihMCpGKe_gN3kZNIk-92d1AWZ44V9932M-P5JI-DuWh14jyB55LPyQTLVg)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: find_by_id(RBih_U6JrqNY0lguSkPYmseTOZUJihMCpGKe_gN3kZNIk-92d1AWZ44V9932M-P5JI-DuWh14jyB55LPyQTLVg)
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
    Finished `release` profile [optimized] target(s) in 0.07s
📦 Built wheel for abi3 Python ≥ 3.8 to /Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.7.1-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel for current platform...[0m
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.7.1-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.7.1
    Uninstalling ekodb_client-0.7.1:
      Successfully uninstalled ekodb_client-0.7.1
Successfully installed ekodb-client-0.7.1
✅ [32mPython client package built and installed![0m
🧪 [36mRunning Python client library examples...[0m
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {'id': 'i66cHF1pildawzYt0dOSoVGHBypWVt6Dur1iqlEW9C01hf10Xqs6RfSornAvZxL5772pbmHs5wC2UIfs-F3o-w'}

=== Find by ID ===
Found: {'data': {'value': 'aGVsbG8gd29ybGQ=', 'type': 'String'}, 'name': {'value': 'Test Record', 'type': 'String'}, 'id': 'i66cHF1pildawzYt0dOSoVGHBypWVt6Dur1iqlEW9C01hf10Xqs6RfSornAvZxL5772pbmHs5wC2UIfs-F3o-w', 'active': {'type': 'Boolean', 'value': True}, 'embedding': {'value': [0.1, 0.2, 0.3, 0.4, 0.5], 'type': 'Array'}, 'categories': {'value': ['electronics', 'computers'], 'type': 'Array'}, 'metadata': {'type': 'Object', 'value': {'key': 'value', 'nested': {'deep': True}}}, 'created_at': {'value': '2026-01-05T10:33:30.516539', 'type': 'String'}, 'user_id': {'value': '550e8400-e29b-41d4-a716-446655440000', 'type': 'String'}, 'tags': {'value': ['tag1', 'tag2', 'tag3'], 'type': 'Array'}, 'price': {'type': 'Float', 'value': 99.99}, 'value': {'type': 'Integer', 'value': 42}}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): True
  price (Decimal): 99.99
  created_at (DateTime): 2026-01-05 10:33:30.516539
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): ['tag1', 'tag2', 'tag3']
  metadata (Object): {'key': 'value', 'nested': {'deep': True}}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): ['electronics', 'computers']
  data (Bytes): 11 bytes
Plain record: {'data': 'aGVsbG8gd29ybGQ=', 'name': 'Test Record', 'id': 'i66cHF1pildawzYt0dOSoVGHBypWVt6Dur1iqlEW9C01hf10Xqs6RfSornAvZxL5772pbmHs5wC2UIfs-F3o-w', 'active': True, 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5], 'categories': ['electronics', 'computers'], 'metadata': {'key': 'value', 'nested': {'deep': True}}, 'created_at': '2026-01-05T10:33:30.516539', 'user_id': '550e8400-e29b-41d4-a716-446655440000', 'tags': ['tag1', 'tag2', 'tag3'], 'price': 99.99, 'value': 42}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'embedding': {'value': [0.1, 0.2, 0.3, 0.4, 0.5], 'type': 'Array'}, 'price': {'type': 'Float', 'value': 99.99}, 'categories': {'value': ['electronics', 'computers'], 'type': 'Array'}, 'name': {'type': 'String', 'value': 'Updated Record'}, 'id': 'i66cHF1pildawzYt0dOSoVGHBypWVt6Dur1iqlEW9C01hf10Xqs6RfSornAvZxL5772pbmHs5wC2UIfs-F3o-w', 'created_at': {'type': 'String', 'value': '2026-01-05T10:33:30.516539'}, 'value': {'value': 100, 'type': 'Integer'}, 'tags': {'value': ['tag1', 'tag2', 'tag3'], 'type': 'Array'}, 'active': {'type': 'Boolean', 'value': True}, 'user_id': {'value': '550e8400-e29b-41d4-a716-446655440000', 'type': 'String'}, 'metadata': {'value': {'key': 'value', 'nested': {'deep': True}}, 'type': 'Object'}, 'data': {'type': 'String', 'value': 'aGVsbG8gd29ybGQ='}}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: 4vULh0fY5gMzGff1GiFMpX8U1L2ytm8AWTFNob60XtEoBXDzQ2FQXGxeL-uo6UkwRl8-ENb7AcktA6ab1bn8nQ

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
Collection created with first record: "0IKKcA-OYDRnGKLolMibYWQIlhMflvDr3BAF20s3mV2fpnkohHOjD9Udk5kTMmRumJgGYQq1Tl3rtxhhPIAyEw"

=== List Collections ===
Total collections: 11
Sample collections: ['chat_configurations__ek0_testing', 'ws_ttl_test', 'websocket_test', 'test_collection', 'test_accounts']

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

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: {'value': '{"name":"Product 1","price":29.99}'}
cache:product:2: {'value': '{"name":"Product 2","price":39.989999999999995}'}
cache:product:3: {'value': '{"name":"Product 3","price":49.989999999999995}'}

=== KV Exists ===
Key exists: True

=== KV Find (Pattern Query) ===
Found 3 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 4

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: False

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: OdZu5yrTk3HGSXHg9R92xKQOE4FiLve4oC9fcA8Ui5BnFcslKW7NH8ZL4LJjrz4k6yeIYU_8gXcZ_MQ6JfsLVA
Created Bob: $500 - ID: kzBEt6qVp67B-mfZHJ_EN7DA76srq1vniNRgh5fl7Fwz_EfVIVNTYzNZ6Go1GnhlamrYsOu8xKj66svujqk6jQ

=== Example 1: Begin Transaction ===
Transaction ID: f235cd11-fdad-4bb2-a7a2-0abe7d1cc5fe

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: ec6d1065-9b96-4714-a2ea-8f095fa72272
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
  1. Score: 25.740, Matched: name, name.value, email.value, email
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title, bio, bio.value, title.value
  2. Score: 26.400, Matched: bio.value, title, bio, title.value
  3. Score: 26.400, Matched: title.value, bio, title, bio.value
  4. Score: 26.400, Matched: title, title.value, bio, bio.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title.value, bio.value, bio, title
  2. Score: 39.600, Matched: title, title.value, bio.value, bio
  3. Score: 39.600, Matched: title, bio, title.value, bio.value
  4. Score: 39.600, Matched: title, bio, title.value, bio.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 2 results (matches: run, running, runs)
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.758
  2. Score: 0.744
  3. Score: 0.727

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.721, Matched: content.value, title, title.value, content
  2. Score: 1.109, Matched: title.value, title, content.value, content
  3. Score: 0.530, Matched: 

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
✓ Inserted document: Yg7QNePsevlsvS8wgve5u-baj0XfW5q9t17UdUO_VM3WR5LvnwtZpb-vjpb__sAEdZszLYRagRCx3n9gD6zKrw

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: vdpFNvY0OiFJPVsrPGFcimx0IeF3yeXrmySt9f-KKeHqd4Ihyv7kyRDtx2r_eUG2khLr_WmfTDBVf791MR2NBA

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
✓ Inserted document with TTL: -Xgskh6sP9I92FgDEDWf_fl2g9QDjyRDib4mpn-p1f1eTS4BhJc6kkoEUEBSS6VI0IV_gSF9Ojti5xdgBZYqwg

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
✓ Edge cache script created: jvnpLMc_W9g8sLmigZ9SwgSy1-zB0lK9MDrWrlTuPenelTjdjDIsZj-m7FTHUtqIVG4qP-pTycn4kkrEovremA

Call 1: Cache miss (fetches from API)
Response time: 557ms
Result: {
  "records": [
    {
      "cached_at": "2026-01-05T10:33:31.925240",
      "data": {
        "type": "Object",
        "value": {
          "current": {
            "interval": 900,
            "temperature_2m": 0.1,
            "time": "2026-01-05T15:30"
          },
          "current_units": {
            "interval": "seconds",
            "temperature_2m": "\u00b0C",
            "time": "iso8601"
          },
          "elevation": 32.0,
          "generationtime_ms": 0.0209808349609375,
          "latitude": 40.710335,
          "longitude": -73.99309,
          "timezone": "GMT",
          "timezone_abbreviation": "GMT",
          "utc_offset_seconds": 0
        }
      },
      "id": "aq-2vljpZcwEeUooSEqOCl_dGuAhGVQv-FrybPdMjiVM4Oi_8sMIKb2ODhr-JQjfNyUIQZFNjtuWQbbWX0tIoA"
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
Response time: 544ms (1.0x faster!)
Result: {
  "records": [
    {
      "cached_at": "2026-01-05T10:33:31.925240",
      "data": {
        "type": "Object",
        "value": {
          "current": {
            "interval": 900,
            "temperature_2m": 0.1,
            "time": "2026-01-05T15:30"
          },
          "current_units": {
            "interval": "seconds",
            "temperature_2m": "\u00b0C",
            "time": "iso8601"
          },
          "elevation": 32.0,
          "generationtime_ms": 0.02205371856689453,
          "latitude": 40.710335,
          "longitude": -73.99309,
          "timezone": "GMT",
          "timezone_abbreviation": "GMT",
          "utc_offset_seconds": 0
        }
      },
      "id": "mtZcjswxSn8EE7BOSrIDvyBSzRm3ADoYnruFOtQecvEy2kdmuSK4tvArE67kcqGO693PbZPFDZUwmpuOoviqlQ"
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

🚀 ekoDB Scripts Example (Python)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: hvwWiBrNqTXw-FTvSes_gWUtdDIBOTCdTznZcfJlKmy8eCGuqqfZWFuzUcadtAhk-Wyy_lgfS-hOUy7E9oP5cw
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved: 8V4ZZCtfnVJJzOFWC-glIun5st0Ie-2ZBzome7_CFdlEiw0dcYn4Kq8HyvBjSRaU69I7iPujDubJoq5_-yle0g
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: wBjxw4H2Hy93vYiWDrJTWgls6ELN9Yr6a_zYvUC0L5FYMbQS6GZYhAZq-X1HLzyvVlaGF8vG1KXdqU8Rse-6QQ
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

✅ Saved reusable function: fetch_and_store_github
✅ Saved SWR function using composition: swr_github_user

First call (cache miss - will fetch from GitHub):
   ⏱️  Duration: 144.1ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 83.3ms
   📊 Records: 1
   🚀 Cache speedup: 1.7x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

✅ Level 1 function: validate_user
✅ Level 2 function: fetch_slim_user (calls validate_user)
✅ Level 3 function: count_validated_user (calls fetch_slim_user)

📊 Result from 3-level nested composition:
   Records: 1
   Name: None
   Department: None
   Record count: 1

🎯 Key Benefit: Each function is independently testable and reusable!
   - validate_user: Used in 100 different workflows
   - fetch_slim_user: Used in 50 workflows
   - count_validated_user: Specific workflow


✅ All composition examples completed!
🚀 ekoDB Python Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved: Co3qC82KMFMPRAJrh8Q22G5Y8oM2AxxCnoIPT55B0blZ1PlintTGWryWEpijrzFCx9oTGaXHQgHux76_4e30OQ
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
   {'category': 'Furniture', 'count': 2}
   {'category': 'Electronics', 'count': 3}
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

✅ Inserted order: BuTxU6yuX_ptwfr3yycatNZAnuKh1eVIWpVdt4YaDZQ4QeRGp2YbGM2njTxIQZy1k69bDNI958BoTwwos2vidg
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

✅ Inserted order: p6bcTfGHAiWS_Uck_V8IXU94T_gwVffjdeTDh95aq7mYg-UYAp_ba-vSaNF3a9bcRlfCm-Goirckyvn9Pc5jQQ
✅ Cached order status
📊 Quick status lookup: {'value': '{"status":"processing","updated_at":"2026-01-05T15:33:33.748053+00:00"}'}

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
✓ Created SWR script: fetch_github_user_py (FHO7MD0fDJL8eZAtvg-UH-6Suqb-fGKpykZ4WPlAa_c3_g9Yx7OIRfky0t9FB7JitX4_r-LCLuTX7MSVvYGjDA)

Step 2: First call - Cache miss, fetches from GitHub API
Traceback (most recent call last):
  File "/Users/tek/Development/ekoDB/ekodb-client/examples/python/client_swr_pattern.py", line 119, in <module>
    asyncio.run(main())
  File "/Library/Frameworks/Python.framework/Versions/3.11/lib/python3.11/asyncio/runners.py", line 190, in run
    return runner.run(main)
           ^^^^^^^^^^^^^^^^
  File "/Library/Frameworks/Python.framework/Versions/3.11/lib/python3.11/asyncio/runners.py", line 118, in run
    return self._loop.run_until_complete(task)
           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Library/Frameworks/Python.framework/Versions/3.11/lib/python3.11/asyncio/base_events.py", line 653, in run_until_complete
    return future.result()
           ^^^^^^^^^^^^^^^
  File "/Users/tek/Development/ekoDB/ekodb-client/examples/python/client_swr_pattern.py", line 82, in main
    result1 = await client.call_script(
              ^^^^^^^^^^^^^^^^^^^^^^^^^
RuntimeError: Call script failed: API error (500): Server error: Unhandled rejection: ErrorResponse { error: "Field 'data' has incomplete field definition - must have both 'type' and 'value'" }
make[2]: *** [test-examples-python-client] Error 1
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
Inserted: map[id:spQrscJWjGnFhXpz6QrJnEsgOqmx1goTLQSE6OlGCYi57SOE7FJly53b3KwlqMCLRie-9JI8aHOBsCX2TmiudQ]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] id:spQrscJWjGnFhXpz6QrJnEsgOqmx1goTLQSE6OlGCYi57SOE7FJly53b3KwlqMCLRie-9JI8aHOBsCX2TmiudQ name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query ===
Found documents: map[]

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] id:spQrscJWjGnFhXpz6QrJnEsgOqmx1goTLQSE6OlGCYi57SOE7FJly53b3KwlqMCLRie-9JI8aHOBsCX2TmiudQ name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
[32m✓ simple_crud.go completed successfully[0m
[34m
=== Running simple_websocket.go ===[0m
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: gNJIM5t-Z90GX2S83uajBb8Cf_25wpkB7Fp-GaV-ZpiQWf0plIOqYPCcPhkA5i0lNm-RyuZvqPAbCHUrfvb1PQ

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
      },
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "t72bpt21jeSg4T5Hfo6Fk9U-NA8EZPcpKDWKpNfxWbDNAhHHFhjk2RkrVZVqsb_YoUfIXZy-UOQ7xUd1LDrPFQ",
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
        "id": "gNJIM5t-Z90GX2S83uajBb8Cf_25wpkB7Fp-GaV-ZpiQWf0plIOqYPCcPhkA5i0lNm-RyuZvqPAbCHUrfvb1PQ",
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

✅ Script saved: Ki31zNLjoj2T_O1HFSR6AU622LSqsSbPN9IoUrSQfZYpeURp8RzL9tw6OSApFm3SmAGj1pmLATFpCBc1qf0mjQ
📊 Found 25 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: ekOS-A0e8fiPtvmsz1VQAy1r-RLWaAQcvhB__24OHOvqfa745vI1SetszGyOLVAftsZwymqHp8o9uvLOb3EmWQ
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: AdcuWyR6fFmHk9idsycvwGktajYbLqBjA6VRIjP-nR6k_chgNdgLSDO_fPQw5433N9PbQRRQXleElQp2_aSdlA
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"avg_score":60,"count":10,"max_score":100,"status":"active"}
   {"avg_score":50,"count":10,"max_score":90,"status":"inactive"}

📝 Example 4: Function Management

📋 Total scripts: 28
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
Collection created with first record: BqOBnmW9cU0vqOSKuX2ZJ6gx_FRA4aJNoBm2Ew4z2UDBSHqJuPJLcBmZjNgnuQPz_U_zy5774C73S9AsyJ5eew

=== List Collections ===
Total collections: 17
Sample collections: [chat_configurations__ek0_testing schema_documents_client_py schema_users_client_py edge_cache_py ws_ttl_test]

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
Created Alice: $1000 - ID: lAfC7G-H6WJfgVQElX7JdBZNYlyQfF5JIdcBUYtjzf0TBV3AuC4suAdzLyXvHbX_qB4T5eoBJ5UM7Wnk2ubM6w
Created Bob: $500 - ID: 5hCyEcydmSqkAWTlFwjK2OhRmZzlVl-MJX9PZqPof_4lcOUdpuKL9A6U4vGbqV1GkPXE2feOmWL4dHcHprQSaQ

=== Example 1: Begin Transaction ===
Transaction ID: e7b7e7f2-d8c3-4053-b813-a3e6161bdf8c

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
New transaction: a749d827-cf87-4b22-bdc7-22c95fd087ff
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
   ✅ Script saved: 8kZRETBtWbblJMm1CjA2D8ZIOCdTSAJF5NIXnf7oOsdNWWXfgLrtxHZTphPLTGY4PigJIzt_an8qVc2c7cBuhw

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: ozLRZOc-4EBKn2sa1eiFBIG13zA78kMsmJ5-uxcETNZI7kSNaJgW3ZSjvMg20C1EKlhPOUyK7oZ3gje8AhNigg
   📋 Name: map[type:String value:Alice Smith]
   📋 Email: map[type:String value:alice@example.com]
   📋 Status: map[type:String value:pending]
   📋 Credits: map[type:Integer value:0]

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: 45zm8yN5LomC17JOiuk_ETgsHQ157ywnWSqRl_uOY0S3g6oWsLfxTh4LdrNzrZTakmNqzNd47OoNetig6UHD4g

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
   ✅ Script saved: Vydq20VY9CUSZZhSSdss9nZWGsC3dTWnCmVVJaDC-czinvyaoLPktVv-bj55eusm0-OzSNQRrv6dRa2j2t8vPg

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
   ✅ Script saved: OFvYV08606AQuQ2Hy_-h8miKLOcgaxzLpW8D0u-I-PjELf3bPUhHDCYe7IgMmPXNssMIw0oMyDxtwOqM6tMBXA

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: 8kZRETBtWbblJMm1CjA2...
   ✅ Deleted script: 45zm8yN5LomC17JOiuk_...
   ✅ Deleted script: Vydq20VY9CUSZZhSSdss...
   ✅ Deleted script: OFvYV08606AQuQ2Hy_-h...
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

2026/01/05 10:33:51 No .env file found, using defaults
✓ Client connected

═══════════════════════════════════════════════════════════
TEST 1: Document TTL Expiration
═══════════════════════════════════════════════════════════

[Step 1] Insert document with 3 second TTL
  Input: {name: 'TTL Test', value: 'should expire'}
  TTL: 3s
  Output: Document ID = JlOpdZPi5tS8rwbPJlEWb9eyj5J4NyhGrJo66aohE-TQWTgkGA-sR9YLK6-ZD9ukcxtrNV8eC1I5aNCxDJs-dA
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: FindByID(JlOpdZPi5tS8rwbPJlEWb9eyj5J4NyhGrJo66aohE-TQWTgkGA-sR9YLK6-ZD9ukcxtrNV8eC1I5aNCxDJs-dA)
  Output: Found document with name = map[type:String value:TTL Test]
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: FindByID(JlOpdZPi5tS8rwbPJlEWb9eyj5J4NyhGrJo66aohE-TQWTgkGA-sR9YLK6-ZD9ukcxtrNV8eC1I5aNCxDJs-dA)
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

2026/01/05 10:33:58 No .env file found, using defaults
✓ Client connected

═══════════════════════════════════════════════════════════
TEST: WebSocket TTL Expiration
═══════════════════════════════════════════════════════════

[Step 1] Insert document with 3 second TTL
  Input: {name: 'WS TTL Test', value: 'should expire'}
  TTL: 3s
  Output: Document ID = cGCe6TmfAU371vQxkOhkZGA6RrdoHRoHQHI7QiaCtBFA7t_aP36EFlsJKiZTjqlD0TCxN8W0xua6l_G5jK3H5Q
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: FindByID(cGCe6TmfAU371vQxkOhkZGA6RrdoHRoHQHI7QiaCtBFA7t_aP36EFlsJKiZTjqlD0TCxN8W0xua6l_G5jK3H5Q)
  Output: Found document with name = map[type:String value:WS TTL Test]
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: FindByID(cGCe6TmfAU371vQxkOhkZGA6RrdoHRoHQHI7QiaCtBFA7t_aP36EFlsJKiZTjqlD0TCxN8W0xua6l_G5jK3H5Q)
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
Inserted: map[id:koxA7K_BX4H-m5WNFiZhLUKAH_X2mP-BDva780FU0_mOyPyif0jSSuy09dqGWue2bd2xYxvn_ZNpPagFGnFavw]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-01-05T15:34:05Z] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:koxA7K_BX4H-m5WNFiZhLUKAH_X2mP-BDva780FU0_mOyPyif0jSSuy09dqGWue2bd2xYxvn_ZNpPagFGnFavw metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Test Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:42]]

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 0
  active (Boolean): true
  price (Decimal): 99.990000
  created_at (DateTime): 2026-01-05 15:34:05 +0000 UTC
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1 tag2 tag3]
  metadata (Object): map[key:value nested:map[deep:true]]
  embedding (Vector): [0.1 0.2 0.3 0.4 0.5]
  categories (Set): [electronics computers]
  data (Bytes): 11 bytes
Plain record: map[active:true categories:[electronics computers] created_at:2026-01-05T15:34:05Z data:aGVsbG8gd29ybGQ= embedding:[0.1 0.2 0.3 0.4 0.5] id:koxA7K_BX4H-m5WNFiZhLUKAH_X2mP-BDva780FU0_mOyPyif0jSSuy09dqGWue2bd2xYxvn_ZNpPagFGnFavw metadata:map[key:value nested:map[deep:true]] name:Test Record price:99.99 tags:[tag1 tag2 tag3] user_id:550e8400-e29b-41d4-a716-446655440000 value:42]

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-01-05T15:34:05Z] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:koxA7K_BX4H-m5WNFiZhLUKAH_X2mP-BDva780FU0_mOyPyif0jSSuy09dqGWue2bd2xYxvn_ZNpPagFGnFavw metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Updated Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: WbEEkYkgJFjG-TDr2voP_TO_R_dqu7BntklZUDSf47sj1Mnw6MaW52ZqgCbuqoATb9og1DfOqDoh5TGcLFn5CA

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
Collection created with first record: og2n3VqHt6uHiKdCsD1j2ImR2_MwTAeCwWSDtp8d9j7yPpUEqK2dg6fD4tFH-eglHkWWy52Ku702_mMT7RN7EA

=== List Collections ===
Total collections: 16
Sample collections: [chat_configurations__ek0_testing schema_documents_client_py schema_users_client_py edge_cache_py ws_ttl_test]

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

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: un1Dbfw_WKN6PBoKUDMc7yNSLOVbeGj7ivrIbDnEBIRM8BjO4FjJokjaPmOIZDgvP19SiLCn_muMsm6mXmMTjg
Created Bob: $500 - ID: -Mn5pQEMcM4-4AtaG0Y0oKpHjMN_cDsuojNQb7Eexxlegr7R3MjQLjQ_Ur3J1NlE0b661lux_-i0ubyi3gSulw

=== Example 1: Begin Transaction ===
Transaction ID: 0d4d3623-d9a8-4da5-b5af-9aeb6dd9a05b

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 10e152ba-fec8-4429-86c0-9cb97b0f40ed
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
  1. Score: 0.781
  2. Score: 0.774
  3. Score: 0.729

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.710
  2. Score: 1.513
  3. Score: 0.291

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
✓ Inserted document: Q9V4C9mz1GdMK2u8tz_MxM4bsqFP0Ro9pUxl6ivK1T1QzLi_-nxxzwNRKmJVhftBkJkJO7_W0o_FpeoiXPR0yQ

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: ylT5LWc7Lj-gIPQUPTiyBXCw0w49KugxXsrnejlp2vzejjjNcXW0njLHtvSSFerHqmL-9FxtNidFpH9IxJ1-jQ

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
✓ Inserted document with TTL: i6yw9nD8lXyKp1pFxOeoJ3eEGHB9A4jkM-ErcFvvO4gjnF3t0TdxFjl4_ImIrpxXyo0vsJwTm76Onfbpjk1ccg

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
✓ Edge cache script created: x3zhVyGeYEAhpU2QITQ0h_IR9rD2mTF-etXbIL6pD2J4sk9AL9-ufalUyZiRiOwkcOaVZk2msnI4t0snzHHq2Q

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

✅ Script saved: gzOny4Jfb7cx9r8-_pTV9Vh-wK7punwhrBbFuTBeTRu4y2hc4PB6ALPPSplBNMaxqcGlKBA4VvBqWpNXBNy8Xg
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

📋 Total scripts: 30
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

✅ Saved reusable function: fetch_and_store_github
✅ Saved SWR function using composition: swr_github_user

First call (cache miss - will fetch from GitHub):
   ⏱️  Duration: 136.955041ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 85.388417ms
   📊 Records: 1
   🚀 Cache speedup: 1.6x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

✅ Level 1 function: validate_user
✅ Level 2 function: fetch_slim_user (calls validate_user)
✅ Level 3 function: count_validated_user (calls fetch_slim_user)

📊 Result from 3-level nested composition:
   Records: 1
   Name: null
   Department: null
   Record count: 1

🎯 Key Benefit: Each function is independently testable and reusable!
   - validate_user: Used in 100 different workflows
   - fetch_slim_user: Used in 50 workflows
   - count_validated_user: Specific workflow


✅ All composition examples completed!
🚀 ekoDB Go Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved: fOBGaBu_dtxIQRzXkgWhzdYoq8nfwQayh4s03GvhJCo1lCWI3VRW0vadmaYTytgBOFS41sVicgK4ympkHXyCzA
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

✅ Inserted order: Mgc-urQoG1S9LnX3h2rv0udNVVKYvl8YmyS1dyex2QLQrQKsXu04hJMttlAxisiL0zJi5OCLie4hETJi5hcdLg
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: DTroZ1qsT7jgtq6XrhQh_EcchS4-hZs4W_cooVpsDtrisP-VI_5oREgYzEoGdXub0nN1mkJFmZqlcYR_VeIDfw
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: map[role:admin userId:user_abc]
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session
📝 Example 4: KV Operations in Scripts

✅ Script saved: vPV38SXyes3q4LhkE9kp8_EUKMJ0Ta4f2ygRc5MaXl8DKgnd5trXFdh28pxheSawk5VePoEvTEK6x38r6m6Elw
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Script

✅ Script saved: JhRVo_k9g0mENkRMfSMeY3si_k924a4LK1pcCMP6jmp_Wgb8M5ABUK4OPFlY4yMOcoUFHUbMWb6Kj0QRrvZwpA
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
2026/01/05 10:34:08 Save script error: request failed with status 500: Unhandled rejection: ErrorResponse { error: "JSON deserialization error: missing field `then_functions` at line 1 column 844" }
# command-line-arguments
./client_functions_advanced.go:73:36: cannot use script1 (variable of type map[string]interface{}) as ekodb.Script value in argument to client.SaveScript
./client_functions_advanced.go:78:27: invalid operation: cannot index result1 (variable of type *ekodb.FunctionResult)
./client_functions_advanced.go:81:25: invalid operation: cannot index result1 (variable of type *ekodb.FunctionResult)
./client_functions_advanced.go:107:36: cannot use script2 (variable of type map[string]interface{}) as ekodb.Script value in argument to client.SaveScript
./client_functions_advanced.go:113:27: invalid operation: cannot index result2 (variable of type *ekodb.FunctionResult)
./client_functions_advanced.go:118:25: invalid operation: cannot index result2 (variable of type *ekodb.FunctionResult)
make[2]: *** [test-examples-go-client] Error 1
✅ [32mAll Go integration tests complete![0m
📦 [36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.7.1 prepare
> npm run build


> @ekodb/ekodb-client@0.7.1 build
> tsc


up to date, audited 58 packages in 1s

15 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.7.1 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning TypeScript client library examples...[0m

added 1 package, removed 1 package, and audited 13 packages in 835ms

3 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'Fd_-IqDKfFaTgbG36gCjr0ulLsl9bsJ782tNLYkT15pZjpwup2R3-WR5KIZVCo0tdM38EjUsrcFJ3iA_-UJI5w'
}

=== Find by ID ===
Found: {
  metadata: { type: 'Object', value: { key: 'value', nested: [Object] } },
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  name: { type: 'String', value: 'Test Record' },
  price: { type: 'Float', value: 99.99 },
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' },
  active: { type: 'Boolean', value: true },
  created_at: { type: 'DateTime', value: '2026-01-05T15:34:11.604Z' },
  categories: { value: [ 'electronics', 'computers' ], type: 'Array' },
  embedding: { value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ], type: 'Array' },
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' },
  value: { value: 42, type: 'Integer' },
  id: 'Fd_-IqDKfFaTgbG36gCjr0ulLsl9bsJ782tNLYkT15pZjpwup2R3-WR5KIZVCo0tdM38EjUsrcFJ3iA_-UJI5w'
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-01-05T15:34:11.604Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { key: 'value', nested: { deep: true } }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  metadata: { key: 'value', nested: { deep: true } },
  tags: [ 'tag1', 'tag2', 'tag3' ],
  name: 'Test Record',
  price: 99.99,
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  active: true,
  created_at: '2026-01-05T15:34:11.604Z',
  categories: [ 'electronics', 'computers' ],
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  data: 'aGVsbG8gd29ybGQ=',
  value: 42,
  id: 'Fd_-IqDKfFaTgbG36gCjr0ulLsl9bsJ782tNLYkT15pZjpwup2R3-WR5KIZVCo0tdM38EjUsrcFJ3iA_-UJI5w'
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  id: 'Fd_-IqDKfFaTgbG36gCjr0ulLsl9bsJ782tNLYkT15pZjpwup2R3-WR5KIZVCo0tdM38EjUsrcFJ3iA_-UJI5w',
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' },
  embedding: { value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ], type: 'Array' },
  created_at: { value: '2026-01-05T15:34:11.604Z', type: 'DateTime' },
  categories: { type: 'Array', value: [ 'electronics', 'computers' ] },
  active: { value: true, type: 'Boolean' },
  price: { value: 99.99, type: 'Float' },
  metadata: { value: { key: 'value', nested: [Object] }, type: 'Object' },
  tags: { value: [ 'tag1', 'tag2', 'tag3' ], type: 'Array' },
  name: { value: 'Updated Record', type: 'String' },
  data: { value: 'aGVsbG8gd29ybGQ=', type: 'String' },
  value: { type: 'Integer', value: 100 }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: 3XimrUlLON2XL36IAsahkWJe_g9CatKNy4D2lSB9c8acUMO1NNWof8T8YLmmUqt5_owXxcHxnt-yyS5iMMKMbA

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
Collection created with first record: Im4Gt8daO3NU3OEywM0UYkxhiWq3_OeZ6vV5WhZsIjAXZYTmi21CmxDHKvKbgh3y4yNj1my3YocQ5qQ07tPSTA

=== List Collections ===
Total collections: 21
Sample collections: schema_employees_client_go,chat_configurations__ek0_testing,schema_documents_client_py,schema_users_client_py,edge_cache_py

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
cache:product:2: { name: 'Product 2', price: 39.989999999999995 }
cache:product:3: { price: 49.989999999999995, name: 'Product 3' }

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 0 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 4

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: false

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: ZL4hDogryOakKoo44rFKvWenMwC_IPBbUVXL7EY27czp-Wal8xNdfG2gnfDradSI2WqfW4NgabyHqTsqQbq8aA
Created Bob: $500 - ID: gy4XSoNXGozJnqqp9QVPF8kiWxeJCqK0DXADr-wbFbiWzlo9tqxAQ61gosUidQymC6NucxvB8WfT2PFNkCyt3Q

=== Example 1: Begin Transaction ===
Transaction ID: c96bb7a1-a043-4205-8ae5-4c3d03a818d3

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: de5b0e16-28f0-4b35-9a64-4426a4ee2d69
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
  1. Score: 26.400, Matched: bio, title, bio.value, title.value
  2. Score: 26.400, Matched: bio.value, title.value, bio, title
  3. Score: 26.400, Matched: title.value, title, bio, bio.value
  4. Score: 26.400, Matched: bio.value, title, title.value, bio

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio.value, title.value, title, bio
  2. Score: 39.600, Matched: bio, title.value, title, bio.value
  3. Score: 39.600, Matched: title.value, bio, bio.value, title
  4. Score: 39.600, Matched: title.value, title, bio.value, bio

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.770, Matched: 
  2. Score: 0.766, Matched: 
  3. Score: 0.747, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.708, Matched: content, title, content.value, title.value
  2. Score: 1.499, Matched: content, content.value, title, title.value
  3. Score: 0.307, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: bio, skills.value, skills, bio.value

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
✓ Inserted document: HQ2E2_G3VD7jYmfYVRd7N0EF-E-8RhnPlBabMjsFNL4_kFbHJXdcT9tIu21bGejTQHbL0HXT1I25S3Jq32P36A

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: hKHU0Gl7fyZo_vjyBs1qVPc7zDp8aeeBXczvUsbm8L9xJUWRI76QPtdvtbBnpaGwF7lVeJICSXarqwQ6IKsrQg

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
✓ Inserted document with TTL: IudBCfUFS0_d2pe2xJP-Hmhn0Gofn8LtsMEdfLh7AOw-p0sdg4eQDr3SRLwVjNfUo886iauSQMpTK0mRhOsZNw

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
✓ Edge cache script created: br0kdSjFyik_P4CoL8ePXjs-WyJe6GMIms6FPkYd-VN9a2ci_mWFMOr4eNm0qZeLqi9WKhSgQnUib4XWLzqGhg

Call 1: Cache miss (fetches from API)
Response time: 92ms
Result: {
  "records": [
    {
      "cached_at": "2026-01-05T15:34:17.014Z",
      "id": "vmDJHfdNhk89suVwI156x46lkct8KieLZeyO2v_kw9xVkx1DVX3Qy7DnSuWMHdLfOAlj6VyMR3MzHQVad_jOrQ",
      "data": {
        "value": {
          "phone": "1-770-736-8031 x56442",
          "name": "Leanne Graham",
          "address": {
            "street": "Kulas Light",
            "suite": "Apt. 556",
            "city": "Gwenborough",
            "zipcode": "92998-3874",
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            }
          },
          "username": "Bret",
          "id": 1,
          "company": {
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net"
          },
          "email": "Sincere@april.biz",
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
Response time: 62ms (1.5x faster!)
Result: {
  "records": [
    {
      "cached_at": "2026-01-05T15:34:17.014Z",
      "data": {
        "value": {
          "username": "Bret",
          "phone": "1-770-736-8031 x56442",
          "company": {
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net"
          },
          "address": {
            "street": "Kulas Light",
            "city": "Gwenborough",
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            },
            "suite": "Apt. 556",
            "zipcode": "92998-3874"
          },
          "id": 1,
          "website": "hildegard.org",
          "email": "Sincere@april.biz",
          "name": "Leanne Graham"
        },
        "type": "Object"
      },
      "id": "8Rl21izu4dG2pOh1XsyspEH2WRX4n9v_K23srfezi29LJeRT51HInLvbJyB1HcRLNO1lEx7v9L7NpXjpMjX6bg"
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

🚀 ekoDB Scripts Example (TypeScript)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: yF62WUpdHrEhE6QGuncoOcc9LP93I4eGifPDEDz3XmvXHVAIpWgBqJvetmxZRuH8UYdJqlqYmo-xmZNyUCxi9g
📊 Found 15 active users

📝 Example 2: Parameterized Script

✅ Script saved: 212-VJFuy_BlrLgMW50Yld4buroZUYMWBayl5n0b94wXcnOtiXBSzjCY11X2LFgDO9oHqYpKloZGci8euv2xKw
📊 Found 15 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: i4VlwsxGKDqce4eYISx0xAjvV-VLtQXN7hIn1P_H9Ek4M5hlyPgDV-41BEy7UVmJIMfQDJGlP4hbM3x1RGcFyw
📊 Statistics: 3 groups
   {"status":"active","avg_score":60,"count":5}
   {"avg_score":60,"status":"null","count":5}
   {"avg_score":50,"count":5,"status":"inactive"}

📝 Example 4: Script Management

📋 Total scripts: 42
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
   Department: {"type":"String","value":"engineering"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using CallFunction to replace inline logic in SWR pattern...

✅ Saved reusable function: fetch_and_store_github
✅ Saved SWR function using composition: swr_github_user

First call (cache miss - will fetch from GitHub):
   ⏱️  Duration: 84ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 90ms
   📊 Records: 1
   🚀 Cache speedup: 0.9x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

✅ Level 1 function: validate_user
✅ Level 2 function: fetch_slim_user (calls validate_user)
✅ Level 3 function: count_validated_user (calls fetch_slim_user)

📊 Result from 3-level nested composition:
   Records: 1
   Name: N/A
   Department: N/A
   Record count: 1

🎯 Key Benefit: Each function is independently testable and reusable!
   - validate_user: Used in 100 different workflows
   - fetch_slim_user: Used in 50 workflows
   - count_validated_user: Specific workflow


✅ All composition examples completed!
🚀 ekoDB TypeScript Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved: XipX7IOFNr-DBjU-Ckb3xEdK7coSNEk9uuSBq06-OlNga7x7oDvHth75XG1Tt4kIWPrsmZ9YSWBlEyXNac8-dQ
📊 Found 2 product groups
   {"category":"Furniture","count":2,"avg_price":474}
   {"avg_price":575.6666666666666,"count":3,"category":"Electronics"}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved
📊 Found 2 categories
   {"count":3,"category":"Electronics"}
   {"count":2,"category":"Furniture"}
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

✅ Inserted order: jtxWpDmM_MFxZQG-Zz5VSMC2Keji3omoRlPL_kRYGwPB3TsZEREcJvQXfV-QDPPXNZ9v9P9TCgZUviAgxNAjBg
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: -lfwRxGP30zQXhhR7C74qELdgPSOHc30ALbKOKZBbD9ifbo-kxl9b2lGlwb2NhIrI3y2fc188GStGvN-3WoeTQ
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"role":"admin","userId":"user_abc"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session: undefined

📝 Example 4: KV Operations in Scripts

✅ Script saved: jTcwmMuFAw-PERz9KocuyfxLy53bd97TIrsWJdyXhqsaRS5KL3A_lxJShzx7LO9Bdscrd9-5fBDhpYzyx14oCQ
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Script

✅ Script saved: vegu5rXZFgB5eFc2c4ROKNIIO9OrO70NnzIVaEDcM17aI7rSPs4cEtkgVBUGNj9QO-e0ME1CcrtaF7Ue4lqeBg
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
✓ Created SWR script: fetch_github_user (yhNU24BgGG5qtHkdUBZY-ImTNKB46ZugX6uSlfl3K4SXbF5Os18pXw30MORPJkfuNAlUa35FjZ7k5l_slvmORw)

Step 2: First call - Cache miss, fetches from GitHub API
Result: {
  "records": [
    {
      "cached_at": "2026-01-05T15:34:19.448Z",
      "data": {
        "type": "Object",
        "value": {
          "avatar_url": "https://avatars.githubusercontent.com/u/1024025?v=4",
          "site_admin": false,
          "name": "Linus Torvalds",
          "type": "User",
          "html_url": "https://github.com/torvalds",
          "subscriptions_url": "https://api.github.com/users/torvalds/subscriptions",
          "updated_at": "2025-11-24T04:16:14Z",
          "followers": 270115,
          "organizations_url": "https://api.github.com/users/torvalds/orgs",
          "public_repos": 9,
          "company": "Linux Foundation",
          "location": "Portland, OR",
          "following_url": "https://api.github.com/users/torvalds/following{/other_user}",
          "following": 0,
          "gists_url": "https://api.github.com/users/torvalds/gists{/gist_id}",
          "starred_url": "https://api.github.com/users/torvalds/starred{/owner}{/repo}",
          "node_id": "MDQ6VXNlcjEwMjQwMjU=",
          "repos_url": "https://api.github.com/users/torvalds/repos",
          "received_events_url": "https://api.github.com/users/torvalds/received_events",
          "created_at": "2011-09-03T15:26:22Z",
          "hireable": null,
          "blog": "",
          "login": "torvalds",
          "public_gists": 1,
          "user_view_type": "public",
          "bio": null,
          "email": null,
          "gravatar_id": "",
          "events_url": "https://api.github.com/users/torvalds/events{/privacy}",
          "id": 1024025,
          "twitter_username": null,
          "url": "https://api.github.com/users/torvalds",
          "followers_url": "https://api.github.com/users/torvalds/followers"
        }
      },
      "id": "CEk0N1QiSq7KzqWC3rboQkBs8vnArPSuJNg5n4anf06AqAzQZMAzUnixRKu_3qzS_kwIIXAJ5G-7Zk6EMt75hg"
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
Response time: 75ms (served from cache)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_with_reviews (ZPIw2fzZe4lZG-314Dwau2UIY1GajyNdYakcomnvmq0_tzudUfhxzRATD4OjwPEiZg31UVj7xS9AWxgRFT-O-w)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {
      "enriched_at": "2026-01-05T15:34:19.655Z",
      "id": "ia1rruUgxkLCcenvsNyDD5ulFS7xDCjzc-5XPEmvefvM3W91iU3G5i3o8uyk05bVDH71IjVMi9o1Q66gzOunEQ",
      "enriched_data": {
        "type": "Object",
        "value": {
          "rating": 2.56,
          "images": [
            "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
          ],
          "warrantyInformation": "1 week warranty",
          "id": 1,
          "discountPercentage": 10.48,
          "reviews": [
            {
              "comment": "Would not recommend!",
              "rating": 3,
              "reviewerEmail": "eleanor.collins@x.dummyjson.com",
              "reviewerName": "Eleanor Collins",
              "date": "2025-04-30T09:41:02.053Z"
            },
            {
              "reviewerName": "Lucas Gordon",
              "date": "2025-04-30T09:41:02.053Z",
              "comment": "Very satisfied!",
              "reviewerEmail": "lucas.gordon@x.dummyjson.com",
              "rating": 4
            },
            {
              "rating": 5,
              "date": "2025-04-30T09:41:02.053Z",
              "reviewerEmail": "eleanor.collins@x.dummyjson.com",
              "comment": "Highly impressed!",
              "reviewerName": "Eleanor Collins"
            }
          ],
          "weight": 4,
          "category": "beauty",
          "stock": 99,
          "availabilityStatus": "In Stock",
          "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
          "meta": {
            "barcode": "5784719087687",
            "createdAt": "2025-04-30T09:41:02.053Z",
            "updatedAt": "2025-04-30T09:41:02.053Z",
            "qrCode": "https://cdn.dummyjson.com/public/qr-code.png"
          },
          "dimensions": {
            "depth": 22.99,
            "width": 15.14,
            "height": 13.08
          },
          "shippingInformation": "Ships in 3-5 business days",
          "sku": "BEA-ESS-ESS-001",
          "tags": [
            "beauty",
            "mascara"
          ],
          "price": 9.99,
          "title": "Essence Mascara Lash Princess",
          "minimumOrderQuantity": 48,
          "brand": "Essence",
          "returnPolicy": "No return policy",
          "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp"
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
   {"count":3,"category":"Furniture","avg_price":365.6666666666667}
   {"avg_price":367,"category":"Electronics","count":5}
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
   Vector databases offer several benefits including high data precision, representation of geographic features in their true form, efficient spatial analysis and processing, easy updating and editing of data, and support for various types of data such as points, lines, and polygons.
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
   {"role":"user","count":7,"avg_score":70}
   {"avg_score":20,"role":"admin","count":3}
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
   1. Database Design Principles (Database)
   2. Vector Databases Explained (Database)
   3. Introduction to Machine Learning (AI)
   4. Natural Language Processing (AI)
   5. Getting Started with ekoDB (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Script saved
📊 Documents by category:
   {"category":"AI","count":2}
   {"count":3,"category":"Database"}
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples finished!
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: tTeTYVvoCpkCE8mwvywHqGTvhBESl6Y3-hK-sJYWZuTZ84GtFd9LpyY1FiSJkud0UbisY45OGTN7phcAe77HKQ

=== Sending Chat Message ===
Message ID: 0HOdjGwm2AYSrJu11uMBgzUiTbFZpGazxQj9N8q_qht94tccWihqidH4lG9p74cFCx4DIlS65wwFBOGnRbv17Q

=== AI Response ===
The available products are:

1. ekoDB Cloud: It is a fully managed cloud database service product. The price of ekoDB Cloud is $499.

2. ekoDB: This product is a high-performance database with AI capabilities. The price for ekoDB is $99.

3. ekoDB Pro: This is an enterprise edition product with advanced features. The price for ekoDB Pro is $299.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    description: 'Fully managed cloud database service product',
    id: '9F1OwvTEpdjdJK0hprmdKMCNKkuExNJO5xg9VJWWf-KnOHFVJo_qfYQoCXNsh84gZyL5J-CFKGPpb0X9ht9Wug',
    name: 'ekoDB Cloud',
    price: 499
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    description: 'A high-performance database product with AI capabilities',
    name: 'ekoDB',
    price: 99,
    id: 'InL9qWTrO0dnU2nDoOb4RhH78WMzyi-QfqiELSEaM5IU5IzPSb8WOP1lGeSt00GihQYwmc9BpO00Z0B_ST0_gw'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    name: 'ekoDB Pro',
    id: 'ATP8N8a7ju9CwlQV5TF6xhQ-VvffQyEmvoa4UlFUubysy6ONh5p-Z5tDPAU_fVp7h97YqcMzOCiPQDKEzeMPhw',
    price: 299,
    description: 'Enterprise edition product with advanced features'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 4666ms

=== Token Usage ===
Prompt tokens: 608
Completion tokens: 88
Total tokens: 696

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: _zHOReQbiG3YuA0rPmKK_LI6POxGdEy8QOAxunP8RvlHMFGaOZBoaJtOBcM7n2KwEtEX_1IC-INASNPftxXRdw

=== Sending Initial Message ===
✓ Message sent
  Response: The product available based on the provided context is "ekoDB". It's a high-performance database product and the price is $99.

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
✓ Created second session: Ln9JpayhkcQBVZmhSyUlBii4_1YO7Jv735ptsTk2UMjzkE_C8d2oCYSqlcBdq1P1D5N499ZseVzWUcLa-nEg2Q
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
✓ Created session: i__SpHRQbfx2YOk5wES38fvcZ8XvmLjLBCSPblkPuOJ6aHpl2886G9c4bWcV9IV1aEK-r5Be_60X8Ebcadde9w

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information available, we have a product named "ekoDB". It is a high-performance database product.

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: HGfxv8QGSncmCtRj1or7UTVXbEk7-ikcOztK-LZuekfPWm4SJ9W7-9jbaYWV_Kk6sGqoQMtCe7NZ_DiPiOgw0Q
  Parent: i__SpHRQbfx2YOk5wES38fvcZ8XvmLjLBCSPblkPuOJ6aHpl2886G9c4bWcV9IV1aEK-r5Be_60X8Ebcadde9w

=== Listing Sessions ===
✓ Found 7 sessions
  Session 1: HGfxv8QGSncmCtRj1or7UTVXbEk7-ikcOztK-LZuekfPWm4SJ9W7-9jbaYWV_Kk6sGqoQMtCe7NZ_DiPiOgw0Q (Untitled)
  Session 2: i__SpHRQbfx2YOk5wES38fvcZ8XvmLjLBCSPblkPuOJ6aHpl2886G9c4bWcV9IV1aEK-r5Be_60X8Ebcadde9w (Untitled)
  Session 3: Ln9JpayhkcQBVZmhSyUlBii4_1YO7Jv735ptsTk2UMjzkE_C8d2oCYSqlcBdq1P1D5N499ZseVzWUcLa-nEg2Q (Untitled)
  Session 4: tTeTYVvoCpkCE8mwvywHqGTvhBESl6Y3-hK-sJYWZuTZ84GtFd9LpyY1FiSJkud0UbisY45OGTN7phcAe77HKQ (Untitled)
  Session 5: HlgLEuO5eVHTzp6emc9vrrKYyDWQ_IXDgKzI1_2UEdimEilMntvib7YtjlQgvo4ULsDrHzblIa1vlqaZnRedrA (Untitled)
  Session 6: 4XcmiBaLEOBsWSazHwzTs9wPONLFauBk7LZdGPM9-ARmVHIYgjyE9sNHDGnBZQ3o1ShlaUag_CEdZiYGkdgiCQ (Untitled)
  Session 7: aR-Adl9yzHwPf704PXKnJCZnaJlOPXDFagZ71UxUCWKDj7zBUGseVZJaZx2ziMfvsQ20kiytyD3UqhPjfCNiVQ (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: HGfxv8QGSncmCtRj1or7UTVXbEk7-ikcOztK-LZuekfPWm4SJ9W7-9jbaYWV_Kk6sGqoQMtCe7NZ_DiPiOgw0Q

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
✅ [32mTypeScript client examples complete![0m
✅ [32mAll TypeScript integration tests complete![0m
🧪 [36mRunning JavaScript examples (direct HTTP/WebSocket)...[0m

added 1 package, removed 1 package, and audited 9 packages in 811ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
[34m
╔════════════════════════════════════════╗[0m
[34m║  ekoDB JavaScript Examples Test Suite ║[0m
[34m╚════════════════════════════════════════╝[0m
[34m
=== Checking Server Connection ===[0m
(node:91057) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
(Use `node --trace-deprecation ...` to show where the warning was created)
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
  id: 'Tcio_q3T1ZuIC5n22f-y1kR8AqwypdDPsXwtiG_EDC1YMf2TKEcl_raz54q9gQW8JpFrFKaqo6xix2A6bPvcgA'
}

=== Find by ID ===
Found: {
  active: { value: true, type: 'Boolean' },
  id: 'Tcio_q3T1ZuIC5n22f-y1kR8AqwypdDPsXwtiG_EDC1YMf2TKEcl_raz54q9gQW8JpFrFKaqo6xix2A6bPvcgA',
  value: { value: 42, type: 'Integer' },
  name: { value: 'Test Record', type: 'String' }
}

=== Find with Query ===
Found documents: [
  {
    id: 'Tcio_q3T1ZuIC5n22f-y1kR8AqwypdDPsXwtiG_EDC1YMf2TKEcl_raz54q9gQW8JpFrFKaqo6xix2A6bPvcgA',
    active: { value: true, type: 'Boolean' },
    value: { type: 'Integer', value: 42 },
    name: { type: 'String', value: 'Test Record' }
  }
]

=== Update Document ===
Updated: {
  active: { type: 'Boolean', value: true },
  value: { type: 'Integer', value: 100 },
  id: 'Tcio_q3T1ZuIC5n22f-y1kR8AqwypdDPsXwtiG_EDC1YMf2TKEcl_raz54q9gQW8JpFrFKaqo6xix2A6bPvcgA',
  name: { type: 'String', value: 'Updated Record' }
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
✓ Inserted test record: KJwI6LS-TK3tqXt4KxOdwXmHpbU_E9NlZU5kxV_lQAHkdq96PcsjcoWkSdsBWCxrTaZUWR6sEipJ18AP1ksPng

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
        "id": "gNJIM5t-Z90GX2S83uajBb8Cf_25wpkB7Fp-GaV-ZpiQWf0plIOqYPCcPhkA5i0lNm-RyuZvqPAbCHUrfvb1PQ",
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
        "id": "jCqqfNX1CH8oAOspCeoYtlibdQJHNY0vgR73fxLb98lsm2A2lhAlkEWs2K4JJ3fG0iHF4joH55M1tlEJ2xM6hw",
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
        "id": "t72bpt21jeSg4T5Hfo6Fk9U-NA8EZPcpKDWKpNfxWbDNAhHHFhjk2RkrVZVqsb_YoUfIXZy-UOQ7xUd1LDrPFQ",
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
        "id": "KJwI6LS-TK3tqXt4KxOdwXmHpbU_E9NlZU5kxV_lQAHkdq96PcsjcoWkSdsBWCxrTaZUWR6sEipJ18AP1ksPng",
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

✅ Script saved: oxgnRCPdmUVugISRkikdAWUed1eE0aUM77FNyny9zw5w68e29RSx3CBJzHzC8EauwUfkDukrlL5xTghqH1wOJA
📊 Found 30 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: 9Ncx68mwJQFvoL10P_tBBWlMpg5AqskxX77qIE1ivNfsBvur_fqgCtGIJSVqxaAEhIEu2-d0xfg96J0ey_-aTg
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: aUwLbnXTkoEz-axcmqytg5F0vCo5DTPO1DR1iu5Mfh2L0cMJawYA4EpEigSI0ZENOTYfKD7ABIh3q9OE8jm18Q
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"status":"inactive","avg_score":50,"max_score":90,"count":10}
   {"status":"active","max_score":100,"count":10,"avg_score":60}

📝 Example 4: Function Management

📋 Total scripts: 53
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
Retrieved value: { username: 'john_doe', userId: 123 }

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
=== Running /Users/tek/Development/ekoDB/ekodb-client/examples/javascript/collection_management.js ===[0m
=== Collection Management (Direct HTTP) ===

✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: FKhl-hDcMpN8_EGMU132nsYPX8zZtQr_H94GFy5xSnzcwjWyTaQDctBwZp9GhIQDMTJm1vbU4eK_1TKyl29Pow

=== List Collections ===
Total collections: 27
Sample collections: [
  'schema_employees_client_go',
  'chat_configurations__ek0_testing',
  'schema_documents_client_py',
  'schema_users_client_py',
  'edge_cache_py'
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
Created Alice: $1000 - ID: suggzqQ8KgQUPW6rc7K_ayGizEocu2mi0xwgbofWtdTykAMzcrFSLECQZAfElCLihcSbj-aBPTrdkyw5J9SFBA
Created Bob: $500 - ID: hc3PkICa5p-jqQvqGg8hpmJL35-zbkkAphHeGakba1PjaQoIQRtniwV3Zfn4imqrXAlWfn8uZ63FDjAZKhjmmw

=== Example 1: Begin Transaction ===
Transaction ID: 91d73a89-29bc-4fc3-9c18-0e6138b39562

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
New transaction: f8cdbfc9-e499-49f6-ab49-99cfb36947b5
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
   ✅ Script saved: iRralNTlPnu5WmVwWjSCRkKn7VX4HA98nQBC-0cDwQ8YS16nLezR_jPzfilciMyowqWvCjqxuFeR7DmquOCKgQ

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Name: [object Object]
   📋 Email: {"type":"String","value":"alice@example.com"}
   📋 Status: [object Object]
   📋 Credits: [object Object]

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: 51lkyWCarYYmO42A8Okon41aYxGT_u6OCLBOcHcBFWV9N3YxDgrinw-h29VBx2AtvkXPwxKzuBqTSP0FBDGmxA

2️⃣ Calling Script (Query + Update + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Status updated to: {"type":"String","value":"active"}
   📋 Name: {"value":"Alice Smith","type":"String"}

============================================================
📝 Script 3: Query + Update Credits + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: yuDEsVCEPjFrprxUM6zsco6ZMMlzYCp2EEXiyO5GJjEDwim2Ox2GJrIpV7hfaZ0i9rlgPNtMP5f1aRi0U6carA

2️⃣ Calling Script (Query + Update Credits + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {"value":0,"type":"Integer"}
   📋 Status: {"type":"String","value":"active"}
   📋 Name: {"value":"Alice Smith","type":"String"}

============================================================
📝 Script 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: dwVRZG6cFxAH0XFd5QFD14W9DMBdMfD-J5jW6tPoJmD1WiuYV8Z1T4zid-mQ3H8ZclFX1GKNKsCF1IOKItCbkA

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: iRralNTlPnu5WmVwWjSC...
   ✅ Deleted script: 51lkyWCarYYmO42A8Oko...
   ✅ Deleted script: yuDEsVCEPjFrprxUM6zs...
   ✅ Deleted script: dwVRZG6cFxAH0XFd5QFD...
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
  Output: Document ID = Lxm72X_7k0VxJJA7h_JpOAXnq5pbeC1gfgTD8QxXxy-ymceiT0E38yuzEm-4816kMmcfKOfi2G1FxdMBvbzy_w
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: findById(Lxm72X_7k0VxJJA7h_JpOAXnq5pbeC1gfgTD8QxXxy-ymceiT0E38yuzEm-4816kMmcfKOfi2G1FxdMBvbzy_w)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: findById(Lxm72X_7k0VxJJA7h_JpOAXnq5pbeC1gfgTD8QxXxy-ymceiT0E38yuzEm-4816kMmcfKOfi2G1FxdMBvbzy_w)
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
  Output: Document ID = 3-wfd6YXLZjA2kobcrPPjXRR4GAmj-fFX8WxttBQZG-tbTjKqKTX5FkYbtuWmHUFAplBEKOFUQC5m8M3q5xmAw
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: findById(3-wfd6YXLZjA2kobcrPPjXRR4GAmj-fFX8WxttBQZG-tbTjKqKTX5FkYbtuWmHUFAplBEKOFUQC5m8M3q5xmAw)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: findById(3-wfd6YXLZjA2kobcrPPjXRR4GAmj-fFX8WxttBQZG-tbTjKqKTX5FkYbtuWmHUFAplBEKOFUQC5m8M3q5xmAw)
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

> @ekodb/ekodb-client@0.7.1 prepare
> npm run build


> @ekodb/ekodb-client@0.7.1 build
> tsc


up to date, audited 58 packages in 1s

15 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.7.1 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning JavaScript client library examples...[0m

added 1 package, removed 1 package, and audited 9 packages in 814ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: '8RLE4FYQ1tb359lFPI6MYnLNb03kC6ArwW6hE6MRUb5RHhes41QTmMheOjAudBP203PNGcYB-v1Z-BM5yaYz0w'
}

=== Find by ID ===
Found: {
  name: { value: 'Test Record', type: 'String' },
  id: '8RLE4FYQ1tb359lFPI6MYnLNb03kC6ArwW6hE6MRUb5RHhes41QTmMheOjAudBP203PNGcYB-v1Z-BM5yaYz0w',
  value: { type: 'Integer', value: 42 },
  active: { value: true, type: 'Boolean' }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  name: { value: 'Updated Record', type: 'String' },
  id: '8RLE4FYQ1tb359lFPI6MYnLNb03kC6ArwW6hE6MRUb5RHhes41QTmMheOjAudBP203PNGcYB-v1Z-BM5yaYz0w',
  value: { type: 'Integer', value: 100 },
  active: { type: 'Boolean', value: true }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: mZ_gABlLLT6vDQ1StEYL-0k9hOK_2viJ8BXZ7uz6GHIgQfdGshWNpWAkylLc6JjbE29wD3xReXFBGKxS2TT2bw

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
Collection created with first record: 9S9YIfXPfbpUQIsrxB9q9oNHLgUx13iWqCLkG6RU0vqUqtUd89HgzoQiu1C7mqzgVfUu32Im0rX1ng4vM3Au-w

=== List Collections ===
Total collections: 26
Sample collections: schema_employees_client_go,chat_configurations__ek0_testing,schema_documents_client_py,schema_users_client_py,edge_cache_py

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
cache:product:2: { name: 'Product 2', price: 39.989999999999995 }
cache:product:3: { name: 'Product 3', price: 49.989999999999995 }

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 0 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 6

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: false

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: OcUnhSzI1EcbRDFqYbuBPjrLHEj-pRMWO_zjAUUdSkCYGLRKiEipfAzmGmAx8wE-MwBRkLDSXJqsuPmnI3JMuQ
Created Bob: $500 - ID: 1XIh9Ct-Z0PJq9MCyGqNoOZ-ixK4KO4Pv5ytsRxg9YThutV6CcWVshZZoNvdTn3VFsE3QyNat3JWfBgaGYn_tw

=== Example 1: Begin Transaction ===
Transaction ID: a1c394da-8257-4687-ad02-697a1d1c6f98

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 93b50bb6-ffdc-4f3b-8ac5-2017bb357cdb
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
  1. Score: 25.740, Matched: name.value, email, name, email.value
  2. Score: 12.540, Matched: name.value, name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title.value, title, bio.value, bio
  2. Score: 26.400, Matched: bio.value, title.value, title, bio
  3. Score: 26.400, Matched: bio.value, title.value, bio, title
  4. Score: 26.400, Matched: title, bio, title.value, bio.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: bio, bio.value, title.value, title
  2. Score: 39.600, Matched: bio, title, bio.value, title.value
  3. Score: 39.600, Matched: title.value, bio.value, bio, title
  4. Score: 39.600, Matched: title, title.value, bio, bio.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.774, Matched: 
  2. Score: 0.750, Matched: 
  3. Score: 0.743, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.697, Matched: title.value, title, content, content.value
  2. Score: 1.510, Matched: title, title.value, content.value, content
  3. Score: 0.300, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: bio, bio.value, skills.value, skills

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
✓ Inserted document: UKmgrVAh9atPxYwswbAlYsRyZHXUXuQO24fzkb6usioyMRxySk4iOErzWSfOlOYkj2DjhYQMwQXTeuCKI5DbGg

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: P6_wj3S8qe3sJ1TExt5b8wflAoI4vijJzdbY-WLHUEcqN7LaQHkCqQXRJohDtOfX-XlCq0Fe6v70uoOsvyLV8w

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
✓ Inserted document with TTL: FNiTcwvx4kB1xwttLw9SGe6p9wjk9kOZAuOFML5CedLULHwHjTH-1Idx5BHgZSO88lyZX8gx1PpBD944t1GwSA

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
✓ Edge cache script created: Fb4ISdJAUbhyh_SgzoniO4r6KDhWPi1MhIeJe-zoyqAYm7-vhxh5hN8r1FXbBM6TNHXe7GMVubGp3JBhbD57gQ

Call 1: Cache miss (fetches from API)
Response time: 545ms
Result: {
  "records": [
    {
      "id": "YT2NziT8RYvDnFQaNuIDlkIsmJTBaYl6jE6F6zdgn8L_xY5D9pMfb6UNLW04dC6etJk3zszqqYT9PAOsDTFKDw",
      "data": {
        "value": {
          "longitude": -73.99309,
          "elevation": 32,
          "current_units": {
            "time": "iso8601",
            "interval": "seconds",
            "temperature_2m": "°C"
          },
          "latitude": 40.710335,
          "current": {
            "time": "2026-01-05T15:30",
            "interval": 900,
            "temperature_2m": 0.1
          },
          "timezone": "GMT",
          "generationtime_ms": 0.025033950805664062,
          "timezone_abbreviation": "GMT",
          "utc_offset_seconds": 0
        },
        "type": "Object"
      },
      "cached_at": "2026-01-05T15:35:23.306Z"
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
Response time: 536ms (1x faster!)
Result: {
  "records": [
    {
      "id": "QFoIIFm1Jw6DA_ElPB8lj2c1s7fqT62XzXaZlCyXMVOdTCyI6Li1CKND5SzyrjSkk7RiGiZjOf5USFAZkXCpeA",
      "data": {
        "value": {
          "generationtime_ms": 0.022292137145996097,
          "elevation": 32,
          "current_units": {
            "interval": "seconds",
            "temperature_2m": "°C",
            "time": "iso8601"
          },
          "timezone": "GMT",
          "timezone_abbreviation": "GMT",
          "latitude": 40.710335,
          "utc_offset_seconds": 0,
          "current": {
            "interval": 900,
            "time": "2026-01-05T15:30",
            "temperature_2m": 0.1
          },
          "longitude": -73.99309
        },
        "type": "Object"
      },
      "cached_at": "2026-01-05T15:35:23.306Z"
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

🚀 ekoDB Scripts Example (JavaScript Client)

✅ Client initialized (token exchange automatic)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: BwFA5JMReVlSt30TDFJ4T9XwFOYKTz1m7740M2xlQzQ20gt5Ag_JXAo3OB5suOyBtQ3m95IRhytWkU9xjpe98A
📊 Found 10 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved
📊 Found 10 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved
📊 Statistics: 2 groups
   {"status":"active","count":5,"avg_score":60}
   {"status":"inactive","count":5,"avg_score":50}
⏱️  Execution time: 0ms

📝 Example 4: Script Management

📋 Total scripts: 56
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

✅ Saved reusable function: fetch_and_store_github
✅ Saved SWR function using composition: swr_github_user

First call (cache miss - will fetch from GitHub):
   ⏱️  Duration: 140ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 89ms
   📊 Records: 1
   🚀 Cache speedup: 1.6x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

✅ Level 1 function: validate_user
✅ Level 2 function: fetch_slim_user (calls validate_user)
✅ Level 3 function: count_validated_user (calls fetch_slim_user)

📊 Result from 3-level nested composition:
   Records: 1
   Name: N/A
   Department: N/A
   Record count: 1

🎯 Key Benefit: Each function is independently testable and reusable!
   - validate_user: Used in 100 different workflows
   - fetch_slim_user: Used in 50 workflows
   - count_validated_user: Specific workflow


✅ All composition examples completed!
🚀 ekoDB JavaScript Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved: 56XnyJU8XcfyYfXlXDnYYDmA4hHWo4BC_llQrrWaywofNoH9smNlX-lVECvd7EzDFp7ECqKpHw44H_gD4o-zlg
📊 Found 2 product groups
   {"avg_price":575.6666666666666,"category":"Electronics","count":3}
   {"category":"Furniture","avg_price":474,"count":2}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved
📊 Found 2 categories
   {"category":"Electronics","count":3}
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

✅ Inserted order: I2bHDMyPQlyatcTTJQuy5bZ4054wf284nzkO-I9nCk9GglZqQWuA73Gar3gfw-V0sLVmrS4q_lNybfaL-iLJww
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: d0QLbyQ79j66gGBQD6H84U-ZldpTS9kW5cEODfrQbceSeO2eHML2jjROBp2wO5crzzQnSGN0asrpFy0bHl0RTg
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"userId":"user_abc","role":"admin"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: WtW2dcYw51fGUXZr2lPRjdzft6iQl_FLVKJBzWaYd0WznT4ONEllb2ykUJpElB6ZQoXo9eWjhGE7QdrFsIL30g
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Script

✅ Script saved: Mh0pHujcHSJ80EnTMFF0E_bk_MesVz6KwE6_GwptgCIW-8Jati9cN-3nTROg3diV3ft0egIi3IlaXY4S7niBkg
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
📊 Users (showing first 5 of 10):
   1. User 3 (user3@example.com) - Score: 30
   2. User 7 (user7@example.com) - Score: 70
   3. User 10 (user10@example.com) - Score: 100
   4. User 2 (user2@example.com) - Score: 20
   5. User 6 (user6@example.com) - Score: 60
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
   1. Getting Started with ekoDB (Database)
   2. Introduction to Machine Learning (AI)
   3. Vector Databases Explained (Database)
   4. Natural Language Processing (AI)
   5. Database Design Principles (Database)
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
   1. Getting Started with ekoDB
   2. Introduction to Machine Learning
   3. Vector Databases Explained
   4. Natural Language Processing
   5. Database Design Principles
⏱️  Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Script saved
📊 Projected documents (showing first 3):
   1. Getting Started with ekoDB
   2. Introduction to Machine Learning
   3. Vector Databases Explained
⏱️  Execution time: 0ms

📝 Example 5: All Document Fields

✅ Script saved
📊 All documents (5 total, showing first 2):
   1. Getting Started with ekoDB (Database)
   2. Introduction to Machine Learning (AI)
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
   Electronics:
      Products: 6 | Stock: 232 | Avg Rating: ⭐4.52
   Furniture:
      Products: 4 | Stock: 43 | Avg Rating: ⭐4.26

⏱️  Total execution time: 0ms
📈 Pipeline stages:

📝 Example 5: Project Specific Fields

✅ Script saved
📊 Product summaries (10 items, showing first 3):
   1. Monitor 27" - $399 (⭐4.6)
   2. Keyboard - $89 (⭐4.4)
   3. Desk Chair - $349 (⭐4.2)
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
   1. Precision: Vector data are represented by exact coordinates, ensuring high accuracy.
2. Flexibility: Vector databases can handle different types of data including points, lines, and polygons.
3. Scalability: Vector data can be easily scaled up or down without losing quality.
4. Connectivity Information: Vector databases store topological information, enabling analysis of connectivity, adjacency, and proximity.
5. Smaller File Sizes: Vector data often require less storage space compared to raster data.
6. Easy to edit: Vector data can be easily manipulated and edited.
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embedding script saved
📊 Generated embeddings for 3 articles
   1. "Advanced Query Patterns" - 1536D vector
   2. "Draft Article" - 1536D vector
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
✓ Created SWR script: fetch_github_user_js (uHAxQem46AtLUdZWOYKkyIB9M1vQY2Ewt9KYc_vVDF90jIaXVpph1dxf5ppaE8hpEvkGVGRCTRXin3xYRnBcgg)

Step 2: First call - Cache miss, fetches from GitHub API
Result: {
  "records": [
    {
      "cached_at": "2026-01-05T15:35:32.075Z",
      "data": {
        "type": "Object",
        "value": {
          "hireable": null,
          "events_url": "https://api.github.com/users/torvalds/events{/privacy}",
          "email": null,
          "login": "torvalds",
          "starred_url": "https://api.github.com/users/torvalds/starred{/owner}{/repo}",
          "user_view_type": "public",
          "bio": null,
          "subscriptions_url": "https://api.github.com/users/torvalds/subscriptions",
          "avatar_url": "https://avatars.githubusercontent.com/u/1024025?v=4",
          "organizations_url": "https://api.github.com/users/torvalds/orgs",
          "created_at": "2011-09-03T15:26:22Z",
          "node_id": "MDQ6VXNlcjEwMjQwMjU=",
          "company": "Linux Foundation",
          "html_url": "https://github.com/torvalds",
          "location": "Portland, OR",
          "blog": "",
          "twitter_username": null,
          "updated_at": "2025-11-24T04:16:14Z",
          "following": 0,
          "id": 1024025,
          "public_gists": 1,
          "received_events_url": "https://api.github.com/users/torvalds/received_events",
          "gists_url": "https://api.github.com/users/torvalds/gists{/gist_id}",
          "followers_url": "https://api.github.com/users/torvalds/followers",
          "type": "User",
          "followers": 270115,
          "gravatar_id": "",
          "public_repos": 9,
          "site_admin": false,
          "url": "https://api.github.com/users/torvalds",
          "following_url": "https://api.github.com/users/torvalds/following{/other_user}",
          "name": "Linus Torvalds",
          "repos_url": "https://api.github.com/users/torvalds/repos"
        }
      },
      "id": "Unw90ZB-18gyHV8fbsFsSQ62RH3gJ87KIp5zHi5xNWKT7zf3qUXR7m4X-WtPBA3oIfkjN8RfoLWv7L-fYv4fwQ"
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
Response time: 87ms (served from cache)
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
✓ Created session: SROxaE3L-lNTUlKwIAeL-zpxfGSi5UxqWQ0ZGQtYT9poNPk1F3TCDkIyRlZD85pxMc3x66sj4KF8eOfNh2JZXA

=== Sending Chat Message ===
Message ID: v5wwwhOb3jdjUK-eThMwklCc8Kv3btF5v0N8fF691wvVpwqMgoFY7epgTYCv5cy5V-LmUDZ2HM6Ovx4sN-_OAQ

=== AI Response ===
Based on the information available, the following products are available:

1. ekoDB: This is a high-performance database product with AI capabilities. It is available for $99.

2. ekoDB Pro: This is an enterprise edition product with advanced features. It is priced at $299.

3. ekoDB Cloud: This is a fully managed cloud database service product. You can purchase this for $499.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_js',
  record: {
    description: 'A high-performance database product with AI capabilities',
    price: 99,
    id: 'JcBvJauexwiigmNHTiDMXxzmbVMHgDribIpo19nO5TNsqxrOnwrLtRLKOYJznR9qX_3Y2hyG_-tLj-O_nfE0iQ',
    name: 'ekoDB'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_js',
  record: {
    name: 'ekoDB Pro',
    id: 'v6dJ6uXwesybbWAfLtPtJ2PgfZiulNrEmQvRh34j25XiIJ3XYbS_tmhDC4xAj-jQIi6pgYDEoNNOGeKyeTQLbg',
    price: 299,
    description: 'Enterprise edition product with advanced features'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_js',
  record: {
    id: 'ErYQpoyJ1W6CX6t9mFK8OMixUKjNRUNhRR9fhYbiqvF98mgy-ls_u_r_9CdJ5c4Ls6Jz5XQIwU54ubdkMPAuVw',
    price: 499,
    name: 'ekoDB Cloud',
    description: 'Fully managed cloud database service product'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 4097ms

=== Token Usage ===
Prompt tokens: 601
Completion tokens: 85
Total tokens: 686

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: STun_ymJxIvap5y8xQO09XFKMhLrqmwsgaxT3yXNizT1nYqTJO8QEYeKweMr6E4HDzPAjj0B-KpOQBj2I68FDg

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information provided, ekoDB is a product available for purchase. It is a high-performance database product that costs $99.

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
✓ Created second session: KtGWRjwz3MuzuB5S-zCwO_-s3Y6bavbHySMoNnVthOWGz52p8QDEqAR0pxWNyG2FQDyxu6OEBZlEjBsVN5pzhg
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
✓ Created session: 0sOqNcDmP14nTc6QgvpkXIjfNzwoEeP21Q3aS8ppYfHz6yBQGM5nIN53XE2YYLp8xODkDCmoWi2YpCQclyGqYg

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information available, one product that is available is ekoDB. It is a high-performance database product and is priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: n7X09lcGBKee4hjaIhmeW5or6uxIWk8td-7YerjLQj00W7q5lWDZAholAZWtojI-1a1XNmMa-j6QoH4v8yVgaA
  Parent: 0sOqNcDmP14nTc6QgvpkXIjfNzwoEeP21Q3aS8ppYfHz6yBQGM5nIN53XE2YYLp8xODkDCmoWi2YpCQclyGqYg

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: n7X09lcGBKee4hjaIhmeW5or6uxIWk8td-7YerjLQj00W7q5lWDZAholAZWtojI-1a1XNmMa-j6QoH4v8yVgaA (Untitled)
  Session 2: 0sOqNcDmP14nTc6QgvpkXIjfNzwoEeP21Q3aS8ppYfHz6yBQGM5nIN53XE2YYLp8xODkDCmoWi2YpCQclyGqYg (Untitled)
  Session 3: KtGWRjwz3MuzuB5S-zCwO_-s3Y6bavbHySMoNnVthOWGz52p8QDEqAR0pxWNyG2FQDyxu6OEBZlEjBsVN5pzhg (Untitled)
  Session 4: SROxaE3L-lNTUlKwIAeL-zpxfGSi5UxqWQ0ZGQtYT9poNPk1F3TCDkIyRlZD85pxMc3x66sj4KF8eOfNh2JZXA (Untitled)
  Session 5: i__SpHRQbfx2YOk5wES38fvcZ8XvmLjLBCSPblkPuOJ6aHpl2886G9c4bWcV9IV1aEK-r5Be_60X8Ebcadde9w (Untitled)
  Session 6: Ln9JpayhkcQBVZmhSyUlBii4_1YO7Jv735ptsTk2UMjzkE_C8d2oCYSqlcBdq1P1D5N499ZseVzWUcLa-nEg2Q (Untitled)
  Session 7: tTeTYVvoCpkCE8mwvywHqGTvhBESl6Y3-hK-sJYWZuTZ84GtFd9LpyY1FiSJkud0UbisY45OGTN7phcAe77HKQ (Untitled)
  Session 8: HlgLEuO5eVHTzp6emc9vrrKYyDWQ_IXDgKzI1_2UEdimEilMntvib7YtjlQgvo4ULsDrHzblIa1vlqaZnRedrA (Untitled)
  Session 9: 4XcmiBaLEOBsWSazHwzTs9wPONLFauBk7LZdGPM9-ARmVHIYgjyE9sNHDGnBZQ3o1ShlaUag_CEdZiYGkdgiCQ (Untitled)
  Session 10: aR-Adl9yzHwPf704PXKnJCZnaJlOPXDFagZ71UxUCWKDj7zBUGseVZJaZx2ziMfvsQ20kiytyD3UqhPjfCNiVQ (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: n7X09lcGBKee4hjaIhmeW5or6uxIWk8td-7YerjLQj00W7q5lWDZAholAZWtojI-1a1XNmMa-j6QoH4v8yVgaA

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
✓ Inserted user: Record(fields={id=StringValue(value=f3wbRUP9x0RYgrrli9qtXToLhk8CWtC4AGLCFJzcZ_gN5hHDzHe2mSlu5IKGuj1QeJyPLtJSQqDwYlJY5mCsVw)})
  User ID: f3wbRUP9x0RYgrrli9qtXToLhk8CWtC4AGLCFJzcZ_gN5hHDzHe2mSlu5IKGuj1QeJyPLtJSQqDwYlJY5mCsVw

=== Read ===
✓ Found user by ID: Record(fields={name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=28)}), categories=ObjectValue(value={value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)]), type=StringValue(value=Array)}), data=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)])}), email=ObjectValue(value={value=StringValue(value=alice@example.com), type=StringValue(value=String)}), created_at=ObjectValue(value={value=StringValue(value=Mon Jan 05 10:35:52 EST 2026), type=StringValue(value=String)}), id=StringValue(value=f3wbRUP9x0RYgrrli9qtXToLhk8CWtC4AGLCFJzcZ_gN5hHDzHe2mSlu5IKGuj1QeJyPLtJSQqDwYlJY5mCsVw), user_id=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=550e8400-e29b-41d4-a716-446655440000)}), metadata=ObjectValue(value={type=StringValue(value=Object), value=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})})}), price=ObjectValue(value={type=StringValue(value=Float), value=FloatValue(value=99.99)}), embedding=ObjectValue(value={value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)]), type=StringValue(value=Array)}), tags=ObjectValue(value={value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)]), type=StringValue(value=Array)}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)})})

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Alice Johnson
  email (String): alice@example.com
  age (Integer): 28
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): Mon Jan 05 10:35:52 EST 2026
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1, tag2, tag3]
  metadata (Object): {key=value, nested={deep=true}}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): [electronics, computers]
  data (Bytes): 11 bytes
Record fields: [name, age, categories, data, email, created_at, id, user_id, metadata, price, embedding, tags, active]

=== Update ===
✓ Updated user: Record(fields={tags=ObjectValue(value={value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)]), type=StringValue(value=Array)}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), price=ObjectValue(value={type=StringValue(value=Float), value=FloatValue(value=99.99)}), email=ObjectValue(value={value=StringValue(value=alice@example.com), type=StringValue(value=String)}), id=StringValue(value=f3wbRUP9x0RYgrrli9qtXToLhk8CWtC4AGLCFJzcZ_gN5hHDzHe2mSlu5IKGuj1QeJyPLtJSQqDwYlJY5mCsVw), city=ObjectValue(value={value=StringValue(value=San Francisco), type=StringValue(value=String)}), embedding=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)])}), age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=29)}), metadata=ObjectValue(value={value=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})}), type=StringValue(value=Object)}), user_id=ObjectValue(value={value=StringValue(value=550e8400-e29b-41d4-a716-446655440000), type=StringValue(value=String)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), created_at=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Mon Jan 05 10:35:52 EST 2026)}), categories=ObjectValue(value={value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)]), type=StringValue(value=Array)}), data=ObjectValue(value={value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)]), type=StringValue(value=Array)})})

=== Query ===
✓ Found 1 users matching query
  - Record(fields={price=ObjectValue(value={value=FloatValue(value=99.99), type=StringValue(value=Float)}), id=StringValue(value=f3wbRUP9x0RYgrrli9qtXToLhk8CWtC4AGLCFJzcZ_gN5hHDzHe2mSlu5IKGuj1QeJyPLtJSQqDwYlJY5mCsVw), city=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=San Francisco)}), user_id=ObjectValue(value={value=StringValue(value=550e8400-e29b-41d4-a716-446655440000), type=StringValue(value=String)}), age=ObjectValue(value={value=IntegerValue(value=29), type=StringValue(value=Integer)}), created_at=ObjectValue(value={value=StringValue(value=Mon Jan 05 10:35:52 EST 2026), type=StringValue(value=String)}), tags=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)])}), active=ObjectValue(value={value=BooleanValue(value=true), type=StringValue(value=Boolean)}), metadata=ObjectValue(value={type=StringValue(value=Object), value=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})})}), categories=ObjectValue(value={value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)]), type=StringValue(value=Array)}), embedding=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)])}), data=ObjectValue(value={value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)]), type=StringValue(value=Array)}), email=ObjectValue(value={value=StringValue(value=alice@example.com), type=StringValue(value=String)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)})})

=== Delete ===
✓ Deleted user with ID: f3wbRUP9x0RYgrrli9qtXToLhk8CWtC4AGLCFJzcZ_gN5hHDzHe2mSlu5IKGuj1QeJyPLtJSQqDwYlJY5mCsVw

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
  {"payload":{"data":[{"id":"biM3PpBq1DpODBgu0VgxgIdTYX5coQnzUANEXUCqEdsFQa5mDLxMUdaa8Olsa-XIcJwvpJKVnSOHlOk7BCJIOg","name":{"type":"String","value":"Test User"},"status":{"type":"String","value":"active"}}]},"type":"Success"}

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
  IDs: 3QCaP4O_FNA4q_poYTqcn4zpgZPGzswji6vvfSbr-mY8Nh9XLsSMVZJ1wheUcqvtunbzWfAS9SqFeTHV5Da9bQ, 4ieogIXypTmezcUNwZo8M84MMBJ-mkhXwyPauBR3nnIt6HgctnwFbWf1SLz6F4EMG79LqqdzgzsS3bBmWjrz7w, 0IMPotfymQ4bvL2MzPxSxJUevmIZSLEpH1_WZKPGJaDmsrkdbnLC17iVBP8Wz4mHepongAkZ_Hv4ZeDVTQ0gZQ...

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
✓ Found 31 collections
  - schema_employees_client_go
  - chat_configurations__ek0_testing
  - schema_documents_client_js
  - schema_documents_client_py
  - schema_users_client_py

=== Check Collection Existence ===
Collection 'kotlin_collection_example' exists: false

=== Create Collection with Schema ===
✓ Created collection with schema: kotlin_collection_example

=== Get Collection Schema ===
✓ Schema: {"fields":{"name":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null}},"version":1,"created_at":"2026-01-05T15:36:05.412295Z","last_modified":"2026-01-05T15:36:05.412295Z","bypass_ripple":false,"primary_key_alias":"id"}

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
✓ Retrieved value: {"role":"admin","email":"alice@example.com","name":"Alice"}

=== KV Set with TTL ===
✓ Set key with 10s TTL: session:abc123

=== Verify TTL Key ===
✓ Session value: {"user_id":"123","created_at":1767627369632}
  (Will expire in 10 seconds)

=== Set Multiple Keys ===
✓ Set 3 configuration keys

=== KV Exists ===
✓ Key exists: true

=== KV Find (Pattern Query) ===
✓ Found 3 keys matching 'config:.*'

=== KV Query (Alias for Find) ===
✓ Total keys in store: 7

=== KV Delete ===
✓ Deleted key: user:123

=== Verify Deletion ===
✓ Key exists after delete: false

=== Cleanup ===
✓ Cleaned up test keys

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
Created Alice: $1000 - ID: Tj7m1GrxRQDnlJbApHlDwQXeRzIrf6JcqPZ2Y1UeUm1tQjx3lGEck-o__KETH1r3JuryXt4KDWxpUjKNfgFXRA
Created Bob: $500 - ID: xaDUV2fwWUs2cacXBP1RpV6g_fBpeEJ--gXWTMPRzpvtOZgpWIw4JdninxxD-_c7EgG8FD2Cq0nJ8FiaZLCq2w

=== Example 1: Begin Transaction ===
Transaction ID: c1ab0bcd-188f-45a9-94e4-d7fbf2b788da

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 64beab14-868b-4d1c-ac47-cdf523034dce
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
  - Score: ObjectValue(value={value=IntegerValue(value=92), type=StringValue(value=Integer)})
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
  {"results":[{"record":{"id":"lh35jCkBc3VZ5mRlXYGg-xpYLxbH5WZcHu3bBbEj0OrVeSjUuBZC4Zv5zfBKgSx1-f5u7Oj73KVUrfTlhOOsuQ","views":169,"description":"Learn Rust programming language with hands-on examples and best practices.","title":"Rust Programming","tags":"programming,rust,tutorial"},"score":19.8,"matched_fields":["tags","title","description"]},{"record":{"views":22,"id":"C_aO-5FyQTffOkWsOtwuDNtWKId6c_4eKDSoAxXM9Te8lxWKZ_nS74FNwYHUCZZK5rYMmfmc5gcBir2OG1lFxA","title":"Python for Data Science","tags":"programming,python,data-science","description":"Master Python for data analysis, machine learning, and visualization."},"score":6.6000000000000005,"matched_fields":["tags"]},{"record":{"title":"JavaScript Web Development","description":"Build modern web applications using JavaScript, React, and Node.js.","tags":"programming,javascript,web","views":43,"id":"HEANsLdMa7npSAS6FUmHKhi4gT3bXUTjPl9ouEv1vZJ5L-WoqIq4Oz-rSwfp7jtBHSECBn5oiLE30JkR1s2pUw"},"score":6.6000000000000005,"matched_fields":["tags"]}],"total":3,"execution_time_ms":1}

=== Search for 'machine learning' ===
✓ Found results for 'machine learning'
  {"results":[{"record":{"title":"Machine Learning Basics","id":"dt-RpbvsHZrAo1aEpQ_8lly71-5ItUQE46yrr-ZhRgpP-h_hNz3NVEBp9wA92KQqQz8zqVmlAJtGNuXwLTR1eg","description":"Introduction to machine learning algorithms and neural networks.","views":872,"tags":"ai,machine-learning,python"},"score":2.7,"matched_fields":["tags","description","title"]},{"record":{"views":22,"id":"C_aO-5FyQTffOkWsOtwuDNtWKId6c_4eKDSoAxXM9Te8lxWKZ_nS74FNwYHUCZZK5rYMmfmc5gcBir2OG1lFxA","tags":"programming,python,data-science","description":"Master Python for data analysis, machine learning, and visualization.","title":"Python for Data Science"},"score":1.0,"matched_fields":["description"]},{"record":{"description":"Learn Rust programming language with hands-on examples and best practices.","views":169,"id":"lh35jCkBc3VZ5mRlXYGg-xpYLxbH5WZcHu3bBbEj0OrVeSjUuBZC4Zv5zfBKgSx1-f5u7Oj73KVUrfTlhOOsuQ","tags":"programming,rust,tutorial","title":"Rust Programming"},"score":0.5,"matched_fields":["description"]},{"record":{"id":"s7jz4FZSAW_tCAeVOSoTLjjRSoy7o2Idoa-hnmhFRqQz_fbfqT3H04YHjGDsPneTCRsMWUJ0BTeAitQWv2n91Q","views":800,"title":"Database Design","tags":"database,design,sql","description":"Learn database design principles, normalization, and query optimization."},"score":0.5,"matched_fields":["description"]}],"total":4,"execution_time_ms":1}

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
✓ Inserted user 1: StringValue(value=XscPFEPT3qxGF6kMZ1QvMfYeOVBHgUQzKY39I11X6wVULx9ziYeaSRRsocK6IJ9VxL_BBJ3403icY_sbaso5gw)
✓ Inserted user 2: StringValue(value=Nv6l3xgJwBZEe5OjOPwYxGQVt_TLjBzEbErsrIr8N21elGeLT8oHywDk45uyQSBKAXVzEUA-fz33DDX2Z62C9w)

=== Getting Schema ===
✓ Schema for kotlin_schema_example:
  Fields: {"email":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"status":{"field_type":"String","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"title":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null}}

=== Listing Collections ===
✓ Total collections: 32
  Sample: [schema_employees_client_go, chat_configurations__ek0_testing, schema_documents_client_js, schema_documents_client_py, schema_users_client_py]

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
  - Mouse ($25) by Alice Johnson
  - Laptop ($1200) by Alice Johnson

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
  Document ID: wo7N6-DlfjotlxBLENRCcyb2V3FQ__LO2GUlobbROgJaSKVMlF-jhAm-UMKV7RUTuuWEXozyFyKy4fhOfpjbrA

=== Verify Document Exists ===
✓ Document found: [session_id, id, user_id, created_at, ttl]

=== Insert with Longer TTL ===
✓ Inserted document with 1h TTL
  Document ID: ktEUPctU1UNIRTLggTAfAAwAtOAkiX0s16nhM2Ecwa6S3kHT2U-qWILtFc2GFP01xaAdAZFVabcxEMhpPF5txA

=== TTL Expiration ===
✓ Document will automatically expire after 10 seconds

=== Verify Long TTL Document ===
✓ Long TTL document still exists: [value, cache_key, id, ttl]

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
✓ Inserted document with TTL: StringValue(value=LWJWR3ExwDmome7FzTLlC-7TyfSVZ-LcG8sMgOmXsN5oh-w-2TIDy-0RgpjIWsqDzx4w4JAaFrrpppXqVwoNKQ)

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved data via WebSocket:
  {"payload":{"data":[{"created_at":{"type":"Integer","value":1767627399560},"id":"LWJWR3ExwDmome7FzTLlC-7TyfSVZ-LcG8sMgOmXsN5oh-w-2TIDy-0RgpjIWsqDzx4w4JAaFrrpppXqVwoNKQ","name":{"type":"String","value":"WebSocket TTL Test"},"ttl":"2026-01-05T16:36:39.699958Z","value":{"type":"Integer","value":42}}]},"type":"Success"}

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
✓ Edge cache script created: 0Jx7oCRK26Lqy49TM30yUPWuCXg2O6zHb7AzDL46Emv62go4GxPsF0YHwVBdrANx9MQrLYAmg3LW5Sr8wQw0yA

Call 1: Cache lookup
Found 1 cached entries
Response time: 13ms

Call 2: Cache lookup (connection warm)
Found 1 cached entries
Response time: 6ms

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

✅ Script saved: wKInoJt4XmkBiByNtkAJ0Y1N9Avbmky7FWM7e8-LRiPwSVtuY7k7CySxqV-CENOLyqLZXghPwglPMWLeauOnNw
📊 Found 15 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved
📊 Found 15 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved
📊 Statistics: 3 groups
   {"count":5,"avg_score":60.0,"status":"active"}
   {"status":"null","count":5,"avg_score":60.0}
   {"count":5,"avg_score":50.0,"status":"inactive"}
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

✅ Script saved: hCDmACOTu3q_vup0zW-n__SDOwbhl8W2ipAOW4dabPVUhxuGHP1Khy8J9g46qOpUufVphjw-Z4RgnpsXFQWYww
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

✅ Inserted order: StringValue(value=s3nZHOuD81u-LQBFGe7htFQgvRf1SBVLjJuQekKp2tmq0szrnAA_tNsEjjFphFgeaRH4yUT62iIjxPLCATW7ng)
✅ Inserted 2 products

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: jlWSMZcNbBXiVEfq6k7Q8T3HSVyloVbu6cATE_NNCDwONXmm91ERd46sVcyCm7fcZtpH_SDi4R7twuZXLGiCMw
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"userId":"user_abc","role":"admin"}
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: MJ2MB8Bpm0gL9pNsIkpy1kBIS9TW1zEfmhlygfyoaYYZn_i21IbePzVM2Z-5R91j_WmqStG26Nsg8KwMh-E_ZQ
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Script

✅ Script saved: XkKhjASHaulpbgyIYCf6d7DEGjbkzgYa18F0z2V-UO5sYCwufZerXMdcs0rGbqUBeKxCgymRHZ8dI_19SDp7hA
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
✓ Created SWR script: swr_cache_lookup_kt (pOQDIzkJZsHGQH9rfS2vdh4IXqxia5D9qELPmulZ5BdKeLWV4tGIr3L4ofF-AGF38W2lWhvXC9JWs3GDrZo3Ig)

Step 3: First call - Cache lookup
Found 1 cached entries
✓ Cache lookup complete

Step 4: Second call - Fast cache hit
Response time: 6ms (served from cache)
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
   {"count":3,"avg_price":365.6666666666667,"category":"Furniture"}
   {"avg_price":367.0,"category":"Electronics","count":5}
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
   {"response":"Vector databases offer benefits such as efficient storage, easy data manipulation, accurate geographic representation, and the ability to handle complex spatial relationships and data types. They also provide higher resolution and less distortion in geographical features.","tokens_used":77}
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
   {"status":"active","count":7}
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
   1. {"type":"String","value":"Natural Language Processing"} ({"value":"AI","type":"String"})
   2. {"value":"Getting Started with ekoDB","type":"String"} ({"value":"Database","type":"String"})
   3. {"value":"Vector Databases Explained","type":"String"} ({"value":"Database","type":"String"})
   4. {"value":"Introduction to Machine Learning","type":"String"} ({"type":"String","value":"AI"})
   5. {"type":"String","value":"Database Design Principles"} ({"value":"Database","type":"String"})
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Script saved
📊 Documents by category:
   {"category":"Database","count":3}
   {"category":"AI","count":2}
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
✓ Created session: ptkO96l-RG09jaAjo3fHRCRoSfkpDGwvrVStAR6_ji98O0p0NVJek4NMPfVcBqqzZg5cRd06CbIbt6eimmAnIg

=== Sending Chat Message ===
✓ Chat response:
  Message ID: "ikA0W03YZ0LSaIouIPnG_Ec7bRE07vhH0I57Ns8FDdGPimi9Hw1DicLmIJnpQvCDhvuUlcZilHEBDAONUucp7g"
  Responses: ["ekoDB is a high-performance database with intelligent caching, real-time capabilities, and AI integration. It features a number of advanced functionalities including AI Chat Integration and Search Features. The AI Chat Integration allows you to query your database using natural language and receive AI-powered responses with relevant context. Additionally, ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval."]

=== Cleanup ===
✓ Deleted chat session
✓ Deleted collection: kotlin_chat_basic_example

✓ Basic chat example completed successfully

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
=== ekoDB Kotlin Client - Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: ETv2AI1wdR2WP3XZFEnBknq0SYiOSNbQTlIm58JLjPGpjovLszebZQ0ZN6l39NT_-kRQb01TZDoZZ6eyiWgJeg

=== Sending Initial Message ===
✓ Message sent
  Responses: ["Based on the information available, one product is the ekoDB. It's a high-performance database product priced at $99."]

✓ Second message sent

=== Regenerating AI Response ===
✓ AI response regenerated
  New responses: ["Based on the context provided, the available product is a high-performance database product named ekoDB. It is priced at $99."]

=== Updating Message ===
✓ Updated message content

=== Toggling Forgotten Status ===
✓ Marked message as forgotten (excluded from context)

=== Creating Second Session for Merge ===
✓ Created second session: b56chJeTkiGaGtT8R3Ur8iLDvmbqtLmwxlSF3TC_jiZk0tkslf8kYjjTNmJXItvZAo9WhdT6PD_J3WR6wSaI9A

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
✓ Created session: -B1_1YNUYHfkLMTCJ9ay9lKCTYTeDJb7WTlb7x-NTY642-novcaq_MpBRDbkIwzeAVqwXlt0xZiR_9hIeEEKTQ

=== Sending Messages ===
✓ Message 1 sent
  Responses: ["Based on the provided context, the available product is \"ekoDB\". This is a high-performance database product with Artificial Intelligence capabilities. It is priced at $99."]

✓ Message 2 sent
  Responses: ["The price of the ekoDB product is $99."]

=== Getting Message History ===
✓ Retrieved message history
  Total messages: [{"chat_id":{"type":"String","value":"-B1_1YNUYHfkLMTCJ9ay9lKCTYTeDJb7WTlb7x-NTY642-novcaq_MpBRDbkIwzeAVqwXlt0xZiR_9hIeEEKTQ"},"content":{"type":"String","value":"What products are available?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"N7BdLE_9rmTA0WJLsvsQOCNNHdgJMRVbRuqzZXOLBG0SRXShYmLiVJESCrfpNIJT6Sao1DYrwydcLtLBP8LqgA","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2026-01-05T15:37:43.224076Z"},"id":"kMOkh-YkXRVaJG7KyvdIWq3oc-iyJoYbCT_jRuUUhcJ040lxVwE1IjblD5ZX9sQqRjZO5qrbifdci02R741ThA","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":33,"prompt_tokens":238,"total_tokens":271}},"updated_at":{"type":"DateTime","value":"2026-01-05T15:37:43.224076Z"}},{"chat_id":{"type":"String","value":"-B1_1YNUYHfkLMTCJ9ay9lKCTYTeDJb7WTlb7x-NTY642-novcaq_MpBRDbkIwzeAVqwXlt0xZiR_9hIeEEKTQ"},"content":{"type":"String","value":"Based on the provided context, the available product is \"ekoDB\". This is a high-performance database product with Artificial Intelligence capabilities. It is priced at $99."},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"N7BdLE_9rmTA0WJLsvsQOCNNHdgJMRVbRuqzZXOLBG0SRXShYmLiVJESCrfpNIJT6Sao1DYrwydcLtLBP8LqgA","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2026-01-05T15:37:43.226997Z"},"id":"jZdS8z5dCV5WSr7B2qgl1502eDULf7_WKiwlvqjGvzXyvA-qO8hPFHgv3sXIRdvpfbjWGrznPZOKsgQ89zjyFQ","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":33,"prompt_tokens":238,"total_tokens":271}},"updated_at":{"type":"DateTime","value":"2026-01-05T15:37:43.226997Z"}},{"chat_id":{"type":"String","value":"-B1_1YNUYHfkLMTCJ9ay9lKCTYTeDJb7WTlb7x-NTY642-novcaq_MpBRDbkIwzeAVqwXlt0xZiR_9hIeEEKTQ"},"content":{"type":"String","value":"What is the price?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["price"],"record":{"description":"A high-performance database product with AI capabilities","id":"N7BdLE_9rmTA0WJLsvsQOCNNHdgJMRVbRuqzZXOLBG0SRXShYmLiVJESCrfpNIJT6Sao1DYrwydcLtLBP8LqgA","price":99,"product":"ekoDB"},"score":0.8}]},"created_at":{"type":"DateTime","value":"2026-01-05T15:37:44.555821Z"},"id":"QbSgiZIXWSWuhoZNRuualmOwHmVMjF2fivg1dywO4Bb6vCHAicOXr8CHjz1xD6Zrvj2G2S4uCeL2yPW_YkUg3Q","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":12,"prompt_tokens":238,"total_tokens":250}},"updated_at":{"type":"DateTime","value":"2026-01-05T15:37:44.555821Z"}},{"chat_id":{"type":"String","value":"-B1_1YNUYHfkLMTCJ9ay9lKCTYTeDJb7WTlb7x-NTY642-novcaq_MpBRDbkIwzeAVqwXlt0xZiR_9hIeEEKTQ"},"content":{"type":"String","value":"The price of the ekoDB product is $99."},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["price"],"record":{"description":"A high-performance database product with AI capabilities","id":"N7BdLE_9rmTA0WJLsvsQOCNNHdgJMRVbRuqzZXOLBG0SRXShYmLiVJESCrfpNIJT6Sao1DYrwydcLtLBP8LqgA","price":99,"product":"ekoDB"},"score":0.8}]},"created_at":{"type":"DateTime","value":"2026-01-05T15:37:44.558704Z"},"id":"Ll02QLRt-IWEM2ONrb1Z2E01L1_Ke3M3d6yuh8XaXItoDW7vcJeaLRn6eNltyBK79mOt_TRLk2t58es_oKsh0w","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":12,"prompt_tokens":238,"total_tokens":250}},"updated_at":{"type":"DateTime","value":"2026-01-05T15:37:44.558704Z"}}]

=== Updating Session ===
✓ Updated session system prompt

=== Listing Sessions ===
✓ Total sessions: 10

=== Branching Session ===
✓ Created branched session: gdBl-_UhPdh3_qgKaY72Fsp1b7Azds94Bcy_4g8k8dOq9dxYfXSN7j93a8Un3-KQYKZu45NXqQ23DCoBRlL_Eg

=== Cleanup ===
✓ Deleted chat sessions
✓ Deleted collection: kotlin_chat_sessions_example

✓ Chat session management example completed successfully

BUILD SUCCESSFUL in 6s
2 actionable tasks: 1 executed, 1 up-to-date
✅ [32mKotlin client examples complete![0m
🟣 [33mKotlin Transactions...[0m
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: kz6tD_Ww255e04watR7q2s7jWLo0fswbGKcRqTCd1bDzy33FWyur02mXFZh1Lut_rAOL7Ev_hQ3QfjaNyg6Lcg
Created Bob: $500 - ID: b3TX8m0LUccmOzF--nmSM_HNB316TsL9xMzuur5jCDCsiQrP0ZJgtaoE26JYdHT_-h-83kzUPH0TYamDzcgdQw

=== Example 1: Begin Transaction ===
Transaction ID: 57e8bf6b-d8c1-49fe-b115-e6081ca80161

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 17e73df6-a26c-4bd6-9cba-bbc61f45575e
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
📦 Built wheel for abi3 Python ≥ 3.8 to /Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.7.1-cp38-abi3-macosx_11_0_arm64.whl
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
    ✓ Generated embedding: 1536 dimensions in 408.946291ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 1.010538875s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 280.850584ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 1.136472625s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 1.116152292s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 948.992708ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 365.4545ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 1.010931708s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 944.366125ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 1.193054834s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 251.800458ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 445.393292ms
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
    ✓ Generated embedding: 1536 dimensions in 401.4465ms
    • Function auto-cleaned up by client

→ Executing hybrid_search()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 102.272625ms

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

Writing memory-safe, high-performance database code involves several steps:

1. **Minimize Data Volume**: Aim to only pull the data you need. Extraneous data can increase memory usage and slow down your queries. This is where database normalization, as mentioned in Context 3, can be beneficial. It helps to reduce redundancy, thus minimizing the volume of data.

2. **Optimize Queries**: Properly optimizing your SQL or NoSQL queries can greatly improve performance. Use indexing, limit the number of joins, or reduce subqueries wherever possible. Make sure to analyze and understand your query execution plans.

3. **Implement Efficient Data Structures and Algorithms**: The choice of data structures and algorithms can significantly impact memory usage and performance. 

4. **Connection Pooling**: Reuse database connections instead of creating new ones every time. Creating new database connections can be expensive and slow down your application.

5. **Caching**: Cache your most frequently accessed data. This can reduce the load on your database and make your application faster.

6. **Concurrency Control**: If you are using a SQL database and need to handle ACID transactions, proper concurrency control is important for both performance and data integrity. 

7. **Use Appropriate Database**: As mentioned in Context 1 and Context 4, the choice between SQL and NoSQL can affect performance. NoSQL databases can be a better choice for applications that require high write throughput, horizontal scaling, or have unstructured data. However, SQL databases are often better for complex queries or data with well-defined relationships.

8. **Memory Management**: In the context of the programming language you are using, ensure you are following best practices for memory management to avoid memory leaks or unnecessary memory consumption.

Remember, the key is to understand your specific use case and workload, as different strategies may be more effective in different situations.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 427.039ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 1988 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 253.557583ms
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
  ✓ Text search completed in 61.037417ms

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
    ✓ Generated embedding: 1536 dimensions in 0.793s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 1.315s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.302s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.501s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.247s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.193s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.450s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.310s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.259s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.326s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.388s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.231s
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
    ✓ Generated embedding: 1536 dimensions in 0.201s
    • Function auto-cleaned up by client

→ Executing hybrid_search()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.092s

✓ Found 5 related messages across all conversations:
  1. [Score: 0.000] From conv_database_design
     Use NoSQL when you need: flexible schemas, horizontal scaling, high write throughput, or when working with unstructured data. SQL is better for complex queries, ACID transactions, and structured data with well-defined relationships.

  2. [Score: 0.000] From conv_database_design
     What is database normalization?

  3. [Score: 0.000] From conv_database_design
     Database normalization is the process of organizing data to reduce redundancy and improve data integrity. It involves dividing large tables into smaller ones and defining relationships between them using foreign keys.

  4. [Score: 0.000] From conv_database_design
     When should I use NoSQL over SQL?

  5. [Score: 0.000] From conv_performance
     How can I optimize database queries?

=== Step 4: Generating Context-Aware Response ===
✓ AI Response (with context from 3 conversations):

Writing memory-safe, high-performance database code involves several practices:

1. **Use Appropriate Data Structures:** Data structures greatly influence memory usage and performance. For example, in many applications, using a hash map for rapid lookups can be more efficient than using an array or a list.

2. **Normalization:** As mentioned in Context 3, database normalization can reduce data redundancy and improve data integrity. This can enhance the performance of your database as it avoids unnecessary data and ensures that the database structure is efficient.

3. **Optimize Queries:** As per Context 5, optimizing your database queries can greatly improve performance. This may include techniques such as using indexes effectively, avoiding unnecessary joins, and minimizing the use of functions in WHERE clauses.

4. **Memory Management:** In languages that allow manual memory management, ensure you are handling memory allocation and deallocation properly to avoid memory leaks and crashes. In garbage-collected languages, be mindful of how your code can affect the garbage collector and potentially slow down your application.

5. **Use Prepared Statements:** Using prepared statements can both improve performance and protect against SQL injection attacks. Prepared statements can be compiled once and run many times, which can make repeated queries faster.

6. **Use Efficient Transactions:** Transactions can lock database tables and cause performance issues. Using transactions efficiently — for example, by avoiding long-running transactions — can help ensure high performance.

7. **Use Appropriate Database Type:** Depending on your needs, using SQL or NoSQL can affect the performance of your database. As per Context 1 and 4, NoSQL is better for handling big data with high write throughput and flexible schemas, while SQL is better for structured data with well-defined relationships and ACID transactions.

Remember, the choice of programming language, database type, and whether to use a ORM or raw SQL can also affect the memory safety and performance of your database code. Always measure performance and memory usage in realistic scenarios, and make informed decisions based on those measurements.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.312s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 2227 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.394s
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
    ✓ Generated embedding: 1536 dimensions in 0.492s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.594s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.249s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.313s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.302s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.620s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.309s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.321s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.326s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.278s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.457s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.279s
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
    ✓ Generated embedding: 1536 dimensions in 0.434s
    • Function auto-cleaned up by client

→ Executing hybridSearch()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.077s
✓ Found 5 related messages across all conversations:
  1. [Score: 0.000] From conv_performance
     How can I optimize database queries?

  2. [Score: 0.000] From conv_database_design
     Use NoSQL when you need: flexible schemas, horizontal scaling, high write throughput, or when working with unstructured data. SQL is better for complex queries, ACID transactions, and structured data with well-defined relationships.

  3. [Score: 0.000] From conv_database_design
     When should I use NoSQL over SQL?

  4. [Score: 0.000] From conv_database_design
     What is database normalization?

  5. [Score: 0.000] From conv_database_design
     Database normalization is the process of organizing data to reduce redundancy and improve data integrity. It involves dividing large tables into smaller ones and defining relationships between them using foreign keys.

=== Step 4: Generating Context-Aware Response ===
✓ AI Response (with context from 3 conversations):

Writing memory-safe, high-performance database code involves a combination of various techniques and best practices. Here are a few steps you can follow:

1. **Optimize your queries**: As mentioned in Context 1, optimizing your database queries can significantly improve the performance of your application. This can include limiting the amount of data returned from a query, avoiding nested queries, using joins instead of multiple queries, and using indexed columns for searches.

2. **Choose the right database**: Depending on your requirements, either SQL or NoSQL may be a better fit. As described in Context 2 and 3, NoSQL is typically better for flexible schemas, horizontal scaling, high write throughput, and unstructured data, while SQL is better for complex queries, ACID transactions, and structured data with well-defined relationships.

3. **Normalize your database**: As explained in Context 4 and 5, database normalization can help to reduce redundancy and improve data integrity. By dividing large tables into smaller ones and defining relationships between them using foreign keys, you can ensure that your data is organized in a way that is efficient and easy to query.

4. **Use proper memory management techniques**: In most programming languages, you need to properly manage the memory used by your application. This can include freeing up memory that is no longer in use, preventing memory leaks, and handling out-of-memory errors. This is especially important when dealing with large amounts of data.

5. **Concurrency control**: When multiple users are accessing and modifying the data concurrently, you need to ensure that this doesn't lead to inconsistencies or conflicts. Techniques such as locking, transactions, or optimistic concurrency control can be used to handle this.

6. **Use caching**: Caching can also improve the performance of your database code by storing the results of common queries in memory, so that they can be returned more quickly the next time the same query is run.

7. **Batch processing**: Instead of processing each database operation individually, you can often improve performance by batching operations together, especially when inserting or updating large amounts of data.

Remember that every database and application is unique, so these are general guidelines that may not apply or may need to be modified depending on your specific context.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.389s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 2403 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.373s
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
  ✓ Text search completed in 0.052s
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
    ✓ Generated embedding: 1536 dimensions in 0.301s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.309s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.326s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.261s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.408s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.249s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.290s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.275s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.311s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.243s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.288s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.320s
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
    ✓ Generated embedding: 1536 dimensions in 0.674s
    • Function auto-cleaned up by client

→ Executing HybridSearch()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.072s

✓ Found 5 related messages across all conversations:
  1. [Score: 0.000] From conv_performance
     How can I optimize database queries?

  2. [Score: 0.000] From conv_database_design
     Use NoSQL when you need: flexible schemas, horizontal scaling, high write throughput, or when working with unstructured data. SQL is better for complex queries, ACID transactions, and structured data with well-defined relationships.

  3. [Score: 0.000] From conv_database_design
     When should I use NoSQL over SQL?

  4. [Score: 0.000] From conv_database_design
     What is database normalization?

  5. [Score: 0.000] From conv_database_design
     Database normalization is the process of organizing data to reduce redundancy and improve data integrity. It involves dividing large tables into smaller ones and defining relationships between them using foreign keys.

=== Step 4: Generating Context-Aware Response ===
✓ AI Response (with context from 3 conversations):

Writing memory-safe, high-performance database code involves multiple practices and considerations. Here are some ways to do it:

1. **Optimize Database Queries**: Query optimization is critical to high-performance database code. You should always aim to write efficient queries, which means avoiding full table scans, using indexes wisely, and not fetching more data than necessary. Use 'EXPLAIN' or similar keywords to understand how your queries are being executed and optimize them accordingly.

2. **Normalization**: Use database normalization to organize data efficiently. This will reduce redundancy and improve data integrity, which can lead to improved performance and memory usage. However, remember that in some cases, a certain level of denormalization may be beneficial for query performance.

3. **Choose the Right Database Type**: Depending on your data and use-cases, you might want to choose between SQL and NoSQL databases. SQL databases are better for structured data with well-defined relationships and complex queries, while NoSQL databases are better for flexible schemas, horizontal scaling, high write throughput, or when working with unstructured data.

4. **Use Prepared Statements**: Prepared statements not only enhance security but also improve performance as they help in reducing the amount of time spent on query parsing, compiling, and optimization during execution.

5. **Proper Memory Management**: In your code, always ensure to close database connections and free up any resources that are no longer needed. This prevents memory leaks and keeps your application's memory footprint under control.

6. **Caching**: Implement caching wherever possible. This reduces the load on the database and helps to retrieve frequent data more quickly, which improves overall performance.

7. **Batch Operations**: If you have multiple similar operations to perform, batch them together. This reduces the amount of communication between your code and the database, which can significantly improve performance.  

8. **Concurrency Control**: Implement appropriate concurrency control mechanisms to ensure that multiple operations do not conflict with each other. This is particularly important in multi-user systems where multiple users might be reading and writing to the database simultaneously.

Remember, high-performance database code is not just about speed. It's about ensuring that your application can handle its workload efficiently and reliably, without compromising data integrity or security.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.397s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 2526 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.416s
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
  ✓ Text search completed in 0.053s

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
    ✓ Generated embedding: 1536 dimensions in 0.726s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.327s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.344s
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
    ✓ Generated embedding: 1536 dimensions in 0.355s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.322s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.321s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.436s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 1.313s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.307s
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
    ✓ Generated embedding: 1536 dimensions in 0.252s
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
    ✓ Generated embedding: 1536 dimensions in 0.26s
    • Function auto-cleaned up by client

→ Executing hybridSearch()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.09s

✓ Found 5 related messages across all conversations:
  1. [Score: 0.000] From conv_performance
     How can I optimize database queries?

  2. [Score: 0.000] From conv_database_design
     Use NoSQL when you need: flexible schemas, horizontal scaling, high write throughput, or when working with unstructured data. SQL is better for complex queries, ACID transactions, and structured data with well-defined relationships.

  3. [Score: 0.000] From conv_database_design
     When should I use NoSQL over SQL?

  4. [Score: 0.000] From conv_database_design
     What is database normalization?

  5. [Score: 0.000] From conv_database_design
     Database normalization is the process of organizing data to reduce redundancy and improve data integrity. It involves dividing large tables into smaller ones and defining relationships between them using foreign keys.

=== Step 4: Generating Context-Aware Response ===
✓ Context prepared from search results
✓ AI would use this context to generate comprehensive response

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.307s
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
  ✓ Text search completed in 0.047s

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

> @ekodb/ekodb-client@0.7.1 prepare
> npm run build


> @ekodb/ekodb-client@0.7.1 build
> tsc


up to date, audited 58 packages in 1s

15 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.7.1 build
> tsc

✅ [32mTypeScript client built![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning TypeScript SWR Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_github_user (yxbQbSLUiq2P6gG7qhKMM73ksk1yUkXw5VkR4UgkLgzcwnTT_Ea3XECVEwErOJpYJxHf9SbKl5jjlF3f8ffuOQ)

Step 2: First call - Cache miss, fetches from GitHub API
Result: {
  "records": [
    {
      "id": "zWRQaGM4IECPY0EKiTYwt82FOJIN-_yjWc1cmnSGQ5D-iYgHI6TGy3YOejgpHmhoUgutyJjPM0fuHXgC62oJfg",
      "data": {
        "type": "Object",
        "value": {
          "user_view_type": "public",
          "following_url": "https://api.github.com/users/torvalds/following{/other_user}",
          "gravatar_id": "",
          "email": null,
          "avatar_url": "https://avatars.githubusercontent.com/u/1024025?v=4",
          "created_at": "2011-09-03T15:26:22Z",
          "following": 0,
          "gists_url": "https://api.github.com/users/torvalds/gists{/gist_id}",
          "bio": null,
          "html_url": "https://github.com/torvalds",
          "received_events_url": "https://api.github.com/users/torvalds/received_events",
          "type": "User",
          "updated_at": "2025-11-24T04:16:14Z",
          "public_gists": 1,
          "node_id": "MDQ6VXNlcjEwMjQwMjU=",
          "site_admin": false,
          "starred_url": "https://api.github.com/users/torvalds/starred{/owner}{/repo}",
          "id": 1024025,
          "subscriptions_url": "https://api.github.com/users/torvalds/subscriptions",
          "followers_url": "https://api.github.com/users/torvalds/followers",
          "name": "Linus Torvalds",
          "blog": "",
          "company": "Linux Foundation",
          "repos_url": "https://api.github.com/users/torvalds/repos",
          "location": "Portland, OR",
          "public_repos": 9,
          "followers": 270115,
          "organizations_url": "https://api.github.com/users/torvalds/orgs",
          "url": "https://api.github.com/users/torvalds",
          "events_url": "https://api.github.com/users/torvalds/events{/privacy}",
          "login": "torvalds",
          "hireable": null,
          "twitter_username": null
        }
      },
      "cached_at": "2026-01-05T15:34:19.448Z"
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
Response time: 81ms (served from cache)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_with_reviews (dWkBdu6JXEaRUl6GZ8i_2S1MXaFyjHPCOpG1Ti42QaZNwl1zY8eEMMZ4j9dw0tDSHCkLnmIsUTsLuJf_RwqbgQ)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {
      "enriched_data": {
        "value": {
          "id": 1,
          "returnPolicy": "No return policy",
          "shippingInformation": "Ships in 3-5 business days",
          "availabilityStatus": "In Stock",
          "dimensions": {
            "depth": 22.99,
            "width": 15.14,
            "height": 13.08
          },
          "images": [
            "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
          ],
          "reviews": [
            {
              "reviewerEmail": "eleanor.collins@x.dummyjson.com",
              "reviewerName": "Eleanor Collins",
              "comment": "Would not recommend!",
              "date": "2025-04-30T09:41:02.053Z",
              "rating": 3
            },
            {
              "comment": "Very satisfied!",
              "reviewerEmail": "lucas.gordon@x.dummyjson.com",
              "date": "2025-04-30T09:41:02.053Z",
              "reviewerName": "Lucas Gordon",
              "rating": 4
            },
            {
              "rating": 5,
              "reviewerEmail": "eleanor.collins@x.dummyjson.com",
              "comment": "Highly impressed!",
              "reviewerName": "Eleanor Collins",
              "date": "2025-04-30T09:41:02.053Z"
            }
          ],
          "discountPercentage": 10.48,
          "stock": 99,
          "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
          "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
          "title": "Essence Mascara Lash Princess",
          "warrantyInformation": "1 week warranty",
          "price": 9.99,
          "meta": {
            "barcode": "5784719087687",
            "createdAt": "2025-04-30T09:41:02.053Z",
            "updatedAt": "2025-04-30T09:41:02.053Z",
            "qrCode": "https://cdn.dummyjson.com/public/qr-code.png"
          },
          "rating": 2.56,
          "sku": "BEA-ESS-ESS-001",
          "minimumOrderQuantity": 48,
          "tags": [
            "beauty",
            "mascara"
          ],
          "weight": 4,
          "brand": "Essence",
          "category": "beauty"
        },
        "type": "Object"
      },
      "id": "7YOSVEX-1SIgglslNg67lRvKxNxDVGXgPK69o3Pqi-Z14c7coGV81QKlH04AMSIWVbWU5A0JDvA2Uhb2MQw7GQ",
      "enriched_at": "2026-01-05T15:34:19.655Z"
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
✓ Edge cache script created: Y3OA0i56fRp1Xcx7aEZMmD4IquPfosc96NjEXhNy1JKHuOr2AMvZMSfNc3m3us1z7Rpn4sTToZRLrs41Z8H44g

Call 1: Cache miss (fetches from API)
Response time: 78ms
Result: {
  "records": [
    {
      "cached_at": "2026-01-05T15:34:17.014Z",
      "id": "kEqncu_HEjFB6J8JF2wukNim8U7dFdpVZwesX8Ej-Rn02H_iMvz8WjFzfW9p9-L9v1YLRqTm1YrjeItBUKIoqA",
      "data": {
        "value": {
          "username": "Bret",
          "id": 1,
          "name": "Leanne Graham",
          "phone": "1-770-736-8031 x56442",
          "address": {
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "city": "Gwenborough",
            "zipcode": "92998-3874",
            "street": "Kulas Light",
            "suite": "Apt. 556"
          },
          "website": "hildegard.org",
          "email": "Sincere@april.biz",
          "company": {
            "bs": "harness real-time e-markets",
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona"
          }
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
Response time: 61ms (1.3x faster!)
Result: {
  "records": [
    {
      "id": "9UoOdieStAUk3XYc6-NkDCGtokH_o5G0xgJt5fODWbzcqVrBXVboj8yiIZ2Z_ydMMiGufZOyYZH-fZ3QEEkyFQ",
      "data": {
        "value": {
          "name": "Leanne Graham",
          "id": 1,
          "email": "Sincere@april.biz",
          "website": "hildegard.org",
          "company": {
            "catchPhrase": "Multi-layered client-server neural-net",
            "name": "Romaguera-Crona",
            "bs": "harness real-time e-markets"
          },
          "username": "Bret",
          "phone": "1-770-736-8031 x56442",
          "address": {
            "zipcode": "92998-3874",
            "city": "Gwenborough",
            "street": "Kulas Light",
            "geo": {
              "lng": "81.1496",
              "lat": "-37.3159"
            },
            "suite": "Apt. 556"
          }
        },
        "type": "Object"
      },
      "cached_at": "2026-01-05T15:34:17.014Z"
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
🍹 Building a mixed python/rust project
🔗 Found pyo3 bindings with abi3 support
🐍 Not using a specific python interpreter
💻 Using `MACOSX_DEPLOYMENT_TARGET=11.0` for aarch64-apple-darwin by default
    Finished `release` profile [optimized] target(s) in 0.07s
📦 Built wheel for abi3 Python ≥ 3.8 to /Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.7.1-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel for current platform...[0m
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.7.1-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
Successfully installed ekodb-client-0.7.1
✅ [32mPython client package built and installed![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning Python SWR Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_github_user (4kE1KELDM5YoiodjihBuK-waXdk52PvF3r2UL7ZXfdq6wkIw7DllOHvnCx_kcScuaYx4S26fMw2MnwIO0tNVyQ)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 86ms
Result: [
  {
    "cached_at": "2026-01-05T15:34:19.448Z",
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
        "followers": 270115,
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
        "public_repos": 9,
        "received_events_url": "https://api.github.com/users/torvalds/received_events",
        "repos_url": "https://api.github.com/users/torvalds/repos",
        "site_admin": false,
        "starred_url": "https://api.github.com/users/torvalds/starred{/owner}{/repo}",
        "subscriptions_url": "https://api.github.com/users/torvalds/subscriptions",
        "twitter_username": null,
        "type": "User",
        "updated_at": "2025-11-24T04:16:14Z",
        "url": "https://api.github.com/users/torvalds",
        "user_view_type": "public"
      }
    },
    "id": "qvgMwVa1NVa3DP83ioyTc2OyjRzOHaJ8Om9j8u3lyKbHhvmXcMgcNgjB7w_bLTnyLVrF5sWokA5cZBKQqmd3Hg"
  }
]
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 78ms (1.1x faster!)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (-Y3qnwH9kEWGouj6l-oqjSydr63mWMCgZPVEdKvKvNZmMeFWeEit-FvlI19H5JbzjSCV1UZeveJhNZtKSIDJmw)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: [
  {
    "enriched_at": "1767627592",
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
    "id": "eAYjm7kXlY400_1ELcO5nAfNiqY0eDrBjAvHd0DcebHLTiw6Gsx561uulI2h4XKp8x1R7bDH_aTKI-GmpamisQ"
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
✓ Created SWR script: fetch_github_user (7Lh4ZMPP9Zt8p3k7AIjgJWmX_vwrdaYqGxz9ISAUHFV5is6h-2ChXVuOzLoDFyDuuMqojXg0-lbutomZIKc8xg)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 77.145541ms
Result: [
  {
    "cached_at": "2026-01-05T15:34:19.448Z",
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
        "followers": 270115,
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
        "public_repos": 9,
        "received_events_url": "https://api.github.com/users/torvalds/received_events",
        "repos_url": "https://api.github.com/users/torvalds/repos",
        "site_admin": false,
        "starred_url": "https://api.github.com/users/torvalds/starred{/owner}{/repo}",
        "subscriptions_url": "https://api.github.com/users/torvalds/subscriptions",
        "twitter_username": null,
        "type": "User",
        "updated_at": "2025-11-24T04:16:14Z",
        "url": "https://api.github.com/users/torvalds",
        "user_view_type": "public"
      }
    },
    "id": "tJCa5FHMpihUdBdJdKkfF_NLNiLQ3EBYrAFyO95byL5yvqR83kvwlW0YS7MsVlM-PaK35t2Mc6I0PSGu8txvjg"
  }
]
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 99.538209ms (0.8x faster!)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (o1-9gEUik7d32JIfBzGzqDwJpisC66BGAC-WwN2nfC1zBWxAR73ux4rweYS_vo8DC_Mwy6ZC9nXlYWHV0tUfTw)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: [
  {
    "enriched_at": "1767627592",
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
    "id": "sRa8uby5piIK-qO2zzJxKmZyqIoY3wsTWPVMFajGvE7Enaye4JJwtTUr5KEu6ND1zwIHXI-IKJ9fiW-cVAzz1Q"
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.07s
✅ [32mClient build complete![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning Rust SWR Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/swr_pattern`
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_github_user (temvs_ByeEUloP7U8Vleo3P3JyJuGTbqpt4TEa9HtACGV5deRV_MeyS-YHMDiFxEewelEXsPyTzwf7uSMNYarw)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 82ms
Result: {
  "id": "O77IX_lLv0Cf_qT9nnxnWFhXHbP9_LYmrrr9Zx0oYvQmECZvyXWb-d2LqVRoLamhbS0CS7w2HdlCnWkYJ40G-A",
  "data": {
    "value": {
      "following_url": "https://api.github.com/users/torvalds/following{/other_user}",
      "type": "User",
      "public_repos": 9,
      "email": null,
      "user_view_type": "public",
      "received_events_url": "https://api.github.com/users/torvalds/received_events",
      "hireable": null,
      "followers_url": "https://api.github.com/users/torvalds/followers",
      "login": "torvalds",
      "subscriptions_url": "https://api.github.com/users/torvalds/subscriptions",
      "url": "https://api.github.com/users/torvalds",
      "site_admin": false,
      "public_gists": 1,
      "twitter_username": null,
      "blog": "",
      "node_id": "MDQ6VXNlcjEwMjQwMjU=",
      "bio": null,
      "id": 1024025,
      "name": "Linus Torvalds",
      "repos_url": "https://api.github.com/users/torvalds/repos",
      "starred_url": "https://api.github.com/users/torvalds/starred{/owner}{/repo}",
      "events_url": "https://api.github.com/users/torvalds/events{/privacy}",
      "created_at": "2011-09-03T15:26:22Z",
      "following": 0,
      "html_url": "https://github.com/torvalds",
      "updated_at": "2025-11-24T04:16:14Z",
      "location": "Portland, OR",
      "avatar_url": "https://avatars.githubusercontent.com/u/1024025?v=4",
      "followers": 270115,
      "gists_url": "https://api.github.com/users/torvalds/gists{/gist_id}",
      "company": "Linux Foundation",
      "organizations_url": "https://api.github.com/users/torvalds/orgs",
      "gravatar_id": ""
    },
    "type": "Object"
  }
}
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 82ms (1.0x faster!)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (SUiBP_pnAgyj0wf17sAXjWWYACFuALAFEiDcjNoocTBu8GKyH4DO4hW6kT9hfDNERimC36iYTsv1EITCz9rfYA)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: {
  "id": "6sUDtUNesellGJDJT7mjpDJgdNaKy6KWDS4ysG0cDRDIlOStImplGJTpkeZ3zXSylvpxgtAvPYW955u8xwv0Og",
  "enriched_at": "1767627592",
  "enriched_data": {
    "type": "Object",
    "value": {
      "meta": {
        "createdAt": "2025-04-30T09:41:02.053Z",
        "qrCode": "https://cdn.dummyjson.com/public/qr-code.png",
        "barcode": "5784719087687",
        "updatedAt": "2025-04-30T09:41:02.053Z"
      },
      "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
      "weight": 4,
      "shippingInformation": "Ships in 3-5 business days",
      "sku": "BEA-ESS-ESS-001",
      "id": 1,
      "reviews": [
        {
          "comment": "Would not recommend!",
          "reviewerEmail": "eleanor.collins@x.dummyjson.com",
          "reviewerName": "Eleanor Collins",
          "date": "2025-04-30T09:41:02.053Z",
          "rating": 3
        },
        {
          "date": "2025-04-30T09:41:02.053Z",
          "comment": "Very satisfied!",
          "rating": 4,
          "reviewerEmail": "lucas.gordon@x.dummyjson.com",
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
      "images": [
        "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
      ],
      "title": "Essence Mascara Lash Princess",
      "price": 9.99,
      "brand": "Essence",
      "discountPercentage": 10.48,
      "minimumOrderQuantity": 48,
      "tags": [
        "beauty",
        "mascara"
      ],
      "dimensions": {
        "width": 15.14,
        "depth": 22.99,
        "height": 13.08
      },
      "availabilityStatus": "In Stock",
      "category": "beauty",
      "returnPolicy": "No return policy",
      "warrantyInformation": "1 week warranty",
      "stock": 99,
      "rating": 2.56,
      "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp"
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
✓ Created SWR script: fetch_github_user (pnCjL390JQUz90EjX5ZpB5d7ekkBCVpUrNotzybibfcIcBviVOAqGf0GcWuoaLmK-e2acAk6f0AlPgjNxFM_Kg)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 93ms
Result: [{"data":{"value":{"public_repos":9,"followers":270115,"gists_url":"https://api.github.com/users/torvalds/gists{/gist_id}","login":"torvalds","company":"Linux Foundation","blog":"","gravatar_id":"","name":"Linus Torvalds","received_events_url":"https://api.github.com/users/torvalds/received_events","site_admin":false,"type":"User","subscriptions_url":"https://api.github.com/users/torvalds/subscriptions","user_view_type":"public","public_gists":1,"starred_url":"https://api.github.com/users/torvalds/starred{/owner}{/repo}","updated_at":"2025-11-24T04:16:14Z","url":"https://api.github.com/users/torvalds","followers_url":"https://api.github.com/users/torvalds/followers","following_url":"https://api.github.com/users/torvalds/following{/other_user}","bio":null,"twitter_username":null,"html_url":"https://github.com/torvalds","avatar_url":"https://avatars.githubusercontent.com/u/1024025?v=4","organizations_url":"https://api.github.com/users/torvalds/orgs","location":"Portland, OR","repos_url":"https://api.github.com/users/torvalds/repos","email":null,"node_id":"MDQ6VXNlcjEwMjQwMjU=","events_url":"https://api.github.com/users/torvalds/events{/privacy}","id":1024025,"created_at":"2011-09-03T15:26:22Z","hireable":null,"following":0},"type":"Object"},"id":"q3tCICuYZ4F1nd5CxALnlYpfHmlInGhWTvo66Ap2wzCPvtsw0GmJSvlFL5_aYt_XMQKpLeafk33aSABAPc_GAg"}]
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 80ms
Cache hit was 1.2x faster!

✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (0SMjByu_tcFF1IkkFo_gAA2TL23vhsPrk5QizloeFwErjI3RZR5aIKmKpkwWi_W8M2Nb6QOfQ2RTyinAUlEp6g)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: [{"id":"d7mVe87hkFl5tdaUTvlHaTcy8Cg3no80I4XoNIb7fE37RpuGsYJHYRKYgyos3q6PGopAWrcFoRgGhnA0zhphhQ","enriched_data":{"type":"Object","value":{"category":"beauty","minimumOrderQuantity":48,"id":1,"title":"Essence Mascara Lash Princess","price":9.99,"returnPolicy":"No return policy","description":"The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.","discountPercentage":10.48,"sku":"BEA-ESS-ESS-001","stock":99,"availabilityStatus":"In Stock","tags":["beauty","mascara"],"warrantyInformation":"1 week warranty","weight":4,"images":["https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"],"brand":"Essence","meta":{"updatedAt":"2025-04-30T09:41:02.053Z","barcode":"5784719087687","qrCode":"https://cdn.dummyjson.com/public/qr-code.png","createdAt":"2025-04-30T09:41:02.053Z"},"rating":2.56,"dimensions":{"depth":22.99,"height":13.08,"width":15.14},"reviews":[{"reviewerEmail":"eleanor.collins@x.dummyjson.com","reviewerName":"Eleanor Collins","comment":"Would not recommend!","rating":3,"date":"2025-04-30T09:41:02.053Z"},{"rating":4,"reviewerEmail":"lucas.gordon@x.dummyjson.com","comment":"Very satisfied!","date":"2025-04-30T09:41:02.053Z","reviewerName":"Lucas Gordon"},{"rating":5,"comment":"Highly impressed!","date":"2025-04-30T09:41:02.053Z","reviewerEmail":"eleanor.collins@x.dummyjson.com","reviewerName":"Eleanor Collins"}],"shippingInformation":"Ships in 3-5 business days","thumbnail":"https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp"}},"enriched_at":"1767627592"}]
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.18s
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
   Name: Some(Object({"value": String("User 1"), "type": String("String")}))
   Department: Some(Object({"type": String("String"), "value": String("engineering")}))

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using CallFunction to replace inline logic in SWR pattern...

✅ Saved reusable function: fetch_and_store_github
✅ Saved SWR function using composition: swr_github_user

First call (cache miss - will fetch from GitHub):
   ⏱️  Duration: 88.376084ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 87.850458ms
   📊 Records: 1
   🚀 Cache speedup: 1.0x faster!

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
✅ [32mRust function composition examples complete![0m
📦 [36mBuilding TypeScript client library...[0m

> @ekodb/ekodb-client@0.7.1 prepare
> npm run build


> @ekodb/ekodb-client@0.7.1 build
> tsc


up to date, audited 58 packages in 1s

15 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.7.1 build
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

✅ Saved reusable function: fetch_and_store_github
✅ Saved SWR function using composition: swr_github_user

First call (cache miss - will fetch from GitHub):
   ⏱️  Duration: 88ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 95ms
   📊 Records: 1
   🚀 Cache speedup: 0.9x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

✅ Level 1 function: validate_user
✅ Level 2 function: fetch_slim_user (calls validate_user)
✅ Level 3 function: count_validated_user (calls fetch_slim_user)

📊 Result from 3-level nested composition:
   Records: 1
   Name: N/A
   Department: N/A
   Record count: 1

🎯 Key Benefit: Each function is independently testable and reusable!
   - validate_user: Used in 100 different workflows
   - fetch_slim_user: Used in 50 workflows
   - count_validated_user: Specific workflow


✅ All composition examples completed!
✅ [32mTypeScript function composition examples complete![0m
🐍 [36mBuilding Python client package...[0m
🍹 Building a mixed python/rust project
🔗 Found pyo3 bindings with abi3 support
🐍 Not using a specific python interpreter
💻 Using `MACOSX_DEPLOYMENT_TARGET=11.0` for aarch64-apple-darwin by default
    Finished `release` profile [optimized] target(s) in 0.08s
📦 Built wheel for abi3 Python ≥ 3.8 to /Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.7.1-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel for current platform...[0m
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.7.1-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.7.1
    Uninstalling ekodb_client-0.7.1:
      Successfully uninstalled ekodb_client-0.7.1
Successfully installed ekodb-client-0.7.1
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

✅ Saved reusable function: fetch_and_store_github
✅ Saved SWR function using composition: swr_github_user

First call (cache miss - will fetch from GitHub):
   ⏱️  Duration: 86.8ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 84.1ms
   📊 Records: 1
   🚀 Cache speedup: 1.0x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

✅ Level 1 function: validate_user
✅ Level 2 function: fetch_slim_user (calls validate_user)
✅ Level 3 function: count_validated_user (calls fetch_slim_user)

📊 Result from 3-level nested composition:
   Records: 1
   Name: None
   Department: None
   Record count: 1

🎯 Key Benefit: Each function is independently testable and reusable!
   - validate_user: Used in 100 different workflows
   - fetch_slim_user: Used in 50 workflows
   - count_validated_user: Specific workflow


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

✅ Saved reusable function: fetch_and_store_github
✅ Saved SWR function using composition: swr_github_user

First call (cache miss - will fetch from GitHub):
   ⏱️  Duration: 83.524708ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 80.776875ms
   📊 Records: 1
   🚀 Cache speedup: 1.0x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

✅ Level 1 function: validate_user
✅ Level 2 function: fetch_slim_user (calls validate_user)
✅ Level 3 function: count_validated_user (calls fetch_slim_user)

📊 Result from 3-level nested composition:
   Records: 1
   Name: null
   Department: null
   Record count: 1

🎯 Key Benefit: Each function is independently testable and reusable!
   - validate_user: Used in 100 different workflows
   - fetch_slim_user: Used in 50 workflows
   - count_validated_user: Specific workflow


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

✅ Saved reusable function: fetch_and_store_github
✅ Saved SWR function using composition: swr_github_user

First call (cache miss - will fetch from GitHub):
   ⏱️  Duration: 88ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 88ms
   📊 Records: 1
   🚀 Cache speedup: 1.0x faster!

📝 Example 3: Multi-Level Function Composition

Building complex workflows from small, reusable pieces...

✅ Level 1 function: validate_user
✅ Level 2 function: fetch_slim_user (calls validate_user)
✅ Level 3 function: count_validated_user (calls fetch_slim_user)

📊 Result from 3-level nested composition:
   Records: 1
   Name: N/A
   Department: N/A
   Record count: 1

🎯 Key Benefit: Each function is independently testable and reusable!
   - validate_user: Used in 100 different workflows
   - fetch_slim_user: Used in 50 workflows
   - count_validated_user: Specific workflow


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

