make test-examples-rust
🧪 [36mRunning Rust examples (direct HTTP/WebSocket)...[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.20s
     Running `target/debug/examples/simple_crud`
✓ Authentication successful

=== Insert Document ===
Inserted: Object {"id": String("Iv0cEFRSfk1qBT1rASD36WDWBQq8GoDnWEDkWXvrV7om-I8kfgAc3Cjs0oEn9ZGnntjKME0XjeNkzZig6N73vQ")}

=== Find by ID ===
Found: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("Iv0cEFRSfk1qBT1rASD36WDWBQq8GoDnWEDkWXvrV7om-I8kfgAc3Cjs0oEn9ZGnntjKME0XjeNkzZig6N73vQ"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}

=== Find with Query ===
Found documents: Array [Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("Iv0cEFRSfk1qBT1rASD36WDWBQq8GoDnWEDkWXvrV7om-I8kfgAc3Cjs0oEn9ZGnntjKME0XjeNkzZig6N73vQ"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}]

=== Update Document ===
Updated: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("Iv0cEFRSfk1qBT1rASD36WDWBQq8GoDnWEDkWXvrV7om-I8kfgAc3Cjs0oEn9ZGnntjKME0XjeNkzZig6N73vQ"), "name": Object {"type": String("String"), "value": String("Updated Record")}, "value": Object {"type": String("Integer"), "value": Number(100)}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/simple_websocket`
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: "3REH3FFOJS88jlLDXdXST_xRlKUK1peqhumVvWGk4SnA-eoyIpwb9hCtPMEGaOqrhuTiQ8c5hactyKNl6qQWfg"

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
        "id": "3REH3FFOJS88jlLDXdXST_xRlKUK1peqhumVvWGk4SnA-eoyIpwb9hCtPMEGaOqrhuTiQ8c5hactyKNl6qQWfg",
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
Collection created with first record: "EFtGdnNAmKA2AdEUd6i-vNLdNoq-WOU6l-IPm2MRlCoYuCI4IDiWswvs3PSn0cytDBekcTG4yXDacjB2SQSfyw"

=== List Collections ===
Total collections: 4
Sample collections: ["batch_users", "test_collection", "demo_collection", "websocket_test"]

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/document_ttl`
✓ Authentication successful

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: "mM1kfqw-P2XDomABn8gORjT3idVfXuZmLxWX6oBsgxcZlbPTIadUhWsx8hbo3-YTfKn-_N8dksWoD3Xd8ThN7w"

=== Insert Document with TTL (5 minutes - integer) ===
✓ Inserted document: "BeDWbylR1oRiKuxdWxypIhh1w8B39W3KE-UD1Bgn8aPxfQI8ckLUf1URQ2K-p-FuC_wALPiwAJhvze8wA0dftA"

=== Insert Document with TTL (30 minutes - duration string) ===
✓ Inserted document with duration string TTL: "lwOUiLr0rRPtt5_N5Lx3voQvsIYPfv497mZvH8W_XJ3FvxLDDMvjEaMQHQ3mjuOQGHBrT182iHmm7FwDLbAx7Q"

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
✓ Inserted document with TTL: String("0qWHg5NtdJsNwWWv6UxgOQrEKiEtHEhXDVhKb7OFyIz8yXT7c2hSOi53sRmjniDgF1AtF14YV_EHAqO7T7VdrA")

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

✅ Script saved: O_nkMXM8oy2m0iLf3aSzJT_dcKsgfIbVd9nQU_R0DjjXcfrcTN8OUcNczn-tZMMTpuIUvIXTd1AeUinEXFAQ1Q
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: "2umXAaCusVTnSsgcfSh3eeC4eCfwr0hZHObNlMU6uptNY3TUpRMBDyqN6cLAxCTYfMxYLS1BKJe8toGmKE8QdQ"
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Complex Filter with Multiple Conditions

✅ Script saved: 3bZBR_D1POfS6c5IaZPv3jzVAuVcW9jnR3th-x40eHnAaAsUMz-y0RSPlnHzv7wY7ST6ljb55mUSQfFdkcPvRw
📊 Found 3 users (status=active, score>50, sorted by score)

📝 Example 4: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: 0IAiM_1Rod9cI-q2fl6MHVcB0foRh4dnEADMRm1y5ZeMKjPXzzcPJ5eaHf6LJ4alWnQy0EaHLuBFzAbTPT4_Jg
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
Created Alice: $1000 - ID: DZQON_0w_Z0NtjqlipsR3izaQ-o8MwPTolPBIyAzE87YVFULpgzpAXDgKMSXllKcVBdo1mvvy7eMKUIr6-0krg
Created Bob: $500 - ID: U4DpC2RLH60_kGkypmayQ4l1JuepW1jJG3RvtS6tOvE44Var3VgDgUL0TWZE0TZqht6EY7O03DRJf3b8klX9GA

=== Example 1: Begin Transaction ===
Transaction ID: c20a774c-1112-445c-9920-791ce8d86322

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
New transaction: 1c8cfdbb-6014-49a9-8feb-a44c479a943a
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: 700

=== Cleanup ===
✓ Deleted test accounts

✓ All transaction examples completed
✅ [32mRust direct examples complete![0m
🛠️  [36mBuilding client library...[0m
cargo build -p ekodb_client
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
✅ [32mClient build complete![0m
🧪 [36mRunning Rust client library examples...[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.07s
     Running `target/debug/examples/client_simple_crud`
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: Record { fields: {"id": String("yBc83urxNlUlxj7jxoqdp5CG1hi9UWr9YkJv3CXFFlCLIk5FV5vlW0FGjD-FraL3iaP5Z71j6ilp0oPyxxEhIQ")} }

=== Find by ID ===
Found: Record { fields: {"tags": Object({"value": Array([String("tag1"), String("tag2"), String("tag3")]), "type": String("Array")}), "user_id": Object({"type": String("String"), "value": String("550e8400-e29b-41d4-a716-446655440000")}), "data": Object({"value": String("aGVsbG8gd29ybGQ="), "type": String("String")}), "active": Object({"type": String("Boolean"), "value": Boolean(true)}), "price": Object({"type": String("Float"), "value": Float(99.99)}), "metadata": Object({"type": String("Object"), "value": Object({"nested": Object({"deep": Boolean(true)}), "key": String("value")})}), "categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])}), "embedding": Object({"value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "type": String("Array")}), "id": String("yBc83urxNlUlxj7jxoqdp5CG1hi9UWr9YkJv3CXFFlCLIk5FV5vlW0FGjD-FraL3iaP5Z71j6ilp0oPyxxEhIQ"), "created_at": Object({"value": String("2026-01-03T21:33:44.611298Z"), "type": String("DateTime")}), "value": Object({"type": String("Integer"), "value": Integer(42)}), "name": Object({"value": String("Test Record"), "type": String("String")})} }

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Some("Test Record")
  value (Integer): Some(42)
  active (Boolean): Some(true)
  price (Decimal): Some(99.99)
  created_at (DateTime): Some("2026-01-03T21:33:44.611298Z")
  user_id (UUID): Some("550e8400-e29b-41d4-a716-446655440000")
  tags (Array): 3 items
  metadata (Object): 2 keys
  embedding (Vector): 5 dims
  categories (Set): 2 items
  data (Bytes): 11 bytes

=== Find with Query ===
Found documents: [Record { fields: {"embedding": Object({"type": String("Array"), "value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)])}), "metadata": Object({"type": String("Object"), "value": Object({"nested": Object({"deep": Boolean(true)}), "key": String("value")})}), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "categories": Object({"value": Array([String("electronics"), String("computers")]), "type": String("Array")}), "user_id": Object({"type": String("String"), "value": String("550e8400-e29b-41d4-a716-446655440000")}), "name": Object({"value": String("Test Record"), "type": String("String")}), "data": Object({"type": String("String"), "value": String("aGVsbG8gd29ybGQ=")}), "tags": Object({"type": String("Array"), "value": Array([String("tag1"), String("tag2"), String("tag3")])}), "price": Object({"value": Float(99.99), "type": String("Float")}), "id": String("yBc83urxNlUlxj7jxoqdp5CG1hi9UWr9YkJv3CXFFlCLIk5FV5vlW0FGjD-FraL3iaP5Z71j6ilp0oPyxxEhIQ"), "created_at": Object({"type": String("DateTime"), "value": String("2026-01-03T21:33:44.611298Z")}), "value": Object({"type": String("Integer"), "value": Integer(42)})} }]

=== Update Document ===
Updated: Record { fields: {"value": Object({"type": String("Integer"), "value": Integer(100)}), "id": String("yBc83urxNlUlxj7jxoqdp5CG1hi9UWr9YkJv3CXFFlCLIk5FV5vlW0FGjD-FraL3iaP5Z71j6ilp0oPyxxEhIQ"), "metadata": Object({"value": Object({"nested": Object({"deep": Boolean(true)}), "key": String("value")}), "type": String("Object")}), "embedding": Object({"type": String("Array"), "value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)])}), "name": Object({"value": String("Updated Record"), "type": String("String")}), "created_at": Object({"type": String("DateTime"), "value": String("2026-01-03T21:33:44.611298Z")}), "user_id": Object({"type": String("String"), "value": String("550e8400-e29b-41d4-a716-446655440000")}), "categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])}), "price": Object({"type": String("Float"), "value": Float(99.99)}), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "tags": Object({"value": Array([String("tag1"), String("tag2"), String("tag3")]), "type": String("Array")}), "data": Object({"type": String("String"), "value": String("aGVsbG8gd29ybGQ=")})} }

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
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
     Running `target/debug/examples/client_collection_management`
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: "QIZ_MSWMSNWBNxqQFTmDZlikY6RNm9xc50GdSXwEaDVkLMHUAfYtUSPIYv4MupfiEqA6HoZ35ipdhhx_HJqmKQ"

=== List Collections ===
Total collections: 9
Sample collections: ["batch_users", "test_collection", "users", "websocket_test", "scripts__ek0_testing"]

=== Count Documents ===
Document count: 1

=== Check Collection Exists ===
Collection exists: true

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_document_ttl`
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: "J4edN8A-zz2BX9vS8BYkkGFTMoO8TwrrZlghEdOe0zO7SvwN-ibqYmihfeU52GFvz2hwbbjCZ3UlYv1n6_wchw"

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: Some(String("k7kcT3NQ9IES-HNB2-H9hYDzekfOZvEmkjL_v62x2-KQn2cuEyG4supr0z40pYVD-c1bX1KKffOceB7WJeyQRg"))

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_simple_websocket`
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: JxVLftZT7SaVR5xcjMv19j_J0rQcS0a-yap2-BW1jLHasY83baY69mKDb8IEZ0flnnMC5j2Y_oHoXLxSQFXRwQ

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_websocket_ttl`
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: Some(String("-0H7hBvHXacaqEgbSygI39LvJa3ZMV5nKXaXLDQsmPEC_dQLGB6S-ScmHWV88Zppl6W8yix3vlYzk7XQkj0Qhg"))

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_query_builder`
=== ekoDB Query Builder Example ===

=== Inserting Sample Data ===
✓ Inserted 5 users

=== Simple Equality Query ===
✓ Found 3 active users
  - Some(Object({"value": String("Bob"), "type": String("String")}))
  - Some(Object({"value": String("David"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("Alice")}))

=== Range Query (age >= 28 AND age < 35) ===
✓ Found 3 users in age range
  - Some(Object({"type": String("String"), "value": String("Bob")}))
  - Some(Object({"value": String("David"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("Eve")}))

=== IN Operator ===
✓ Found 4 users with status active or pending
  - Some(Object({"type": String("String"), "value": String("Bob")}))
  - Some(Object({"type": String("String"), "value": String("Eve")}))
  - Some(Object({"value": String("Alice"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("David")}))

=== NOT IN Operator ===
✓ Found 4 users not inactive

=== String Pattern Matching ===
✓ Found 5 users with @example.com email

=== Regex Query ===
✓ Found 0 users with names starting with A-C

=== Complex Query (active AND age >= 28 AND score > 1500) ===
✓ Found 2 users matching all conditions
  - Some(Object({"value": String("Bob"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("David")}))

=== OR Query ===
✓ Found 2 users with age < 28 OR age > 32
  - Some(Object({"value": String("Alice"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("Charlie")}))

=== Sorted Query (by score descending) ===
✓ Top 3 users by score:
  1. Some(Object({"type": String("String"), "value": String("Bob")}))
  2. Some(Object({"type": String("String"), "value": String("David")}))
  3. Some(Object({"type": String("String"), "value": String("Alice")}))

=== Pagination (page 2, size 2) ===
✓ Page 2 results:
  - Some(Object({"type": String("String"), "value": String("Charlie")}))
  - Some(Object({"type": String("String"), "value": String("David")}))

=== NOT Operator ===
✓ Found 4 users NOT inactive

=== Complex Chained Query ===
✓ Found 3 users with all conditions

=== Cleanup ===
✓ Deleted collection

✓ All query builder operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
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
     Matched: ["title", "title.value", "description.value", "description"]
  2. Score: 2.0000
     Title: Some(String("Python for Data Science"))
     Matched: ["description.value", "description"]
  3. Score: 1.0000
     Title: Some(String("Database Design"))
     Matched: ["description", "description.value"]
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
  2. Score: 6.6000 - Some(String("Python for Data Science"))

=== Cleanup ===
✓ Deleted collection

✓ All search operations completed successfully
Execution time: 1ms
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_schema_management`
=== ekoDB Schema Management Example ===

=== Creating Collection with Schema ===
✓ Created collection 'schema_client_rust' with schema

=== Inserting Valid Documents ===
✓ Inserted user 1: Some(String("HZT4DUpXmGccQmN2RcKTaBYVuBMLGvQ3iPeFIa6h_wUCj2oeOqABYV5n4bHxQ-SeGSWJnu3elph3LsxQEu6HcA"))
✓ Inserted user 2: Some(String("DVmZwiZpyQTbA_vDJmZf2QYvE0ZQMhPibTsxvfSWW_p2t8gUCzFses865oCjhYIm6QrhrN_P7mwP__-5OeVltQ"))

=== Getting Schema ===
✓ Schema for schema_client_rust:
  - title: String
    (required)
  - email: String
    (required)
  - status: String
  - age: Integer

=== Listing Collections ===
✓ Total collections: 9
  Sample: ["batch_users", "test_collection", "users", "websocket_test", "scripts__ek0_testing"]

=== Cleanup ===
✓ Deleted collection

✓ All schema management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_chat_basic`
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: DcLiiFvuMZQ0jNmXFlxO-028DyjsLecXVNmcPmQuWrR-3Dwbx99wRtK3uS-9YGYwDiFqoD-Wa7sA1oEN4Sq7Hw

=== Sending Chat Message ===
Message ID: AgDqitoCa-wYDo1v5-JMxluiewsDCtJ_37cys3mjFawEwX8AURTYwjDUeOHnBa2CFmp6JKEZEvI5rnc9Uw3iag

=== AI Response ===
Response 1: ekoDB is a high-performance database that comes with intelligent caching, real-time capabilities, and AI integration. It offers a variety of features. One of these is the AI Chat Integration, which enables you to query your database using natural language and receive AI-powered responses with relevant context. ekoDB also supports full-text search, vector search, and hybrid search with automatic context retrieval. These features aim to provide a flexible and efficient user experience in managing and retrieving data.

=== Context Used (3 snippets) ===

Snippet 1:
  Collection: client_chat_basic_rust
  Score: 0.6333
  Matched Fields: ["title", "category", "content"]
  Record: Object {"category": String("features"), "content": String("The chat feature allows you to query your database using natural language and get AI-powered responses with relevant context."), "id": String("Qo3PGnXw0U96SpYz6KB3VQwIW4lI1QeadsJ6s1ECOt4IZw7TktYA6ZAtRp02VaLfFlweTyQPqm2lZ3tuTGuQ7Q"), "title": String("AI Chat Integration")}

Snippet 2:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["category", "content", "title"]
  Record: Object {"category": String("features"), "content": String("ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval."), "id": String("zZIzb8naMsqBiuOjfJeYUUTewUoACUAfZWjdKXLtWTHPWHGJ__vgpHUNZvuepW-w7pG8EdLInuzQsXCw_yzCjg"), "title": String("Search Features")}

Snippet 3:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["title", "content"]
  Record: Object {"category": String("documentation"), "content": String("ekoDB is a high-performance database with intelligent caching, real-time capabilities, and AI integration."), "id": String("tuxk_RHEZmS6SOvy6VdOfRkMdgw0zXP22AKjYuqCzPehLSYFW52-p2LAWa5bXO1DcRKbEtn3FGGnmQz2HDOQ6Q"), "title": String("Introduction to ekoDB")}

Execution Time: 2603ms

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.64s
     Running `target/debug/examples/client_chat_sessions`
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 4BJU8CUhE7wK9eBg_tXVfWzd-xoRSNQaJvE9j4aZcul7zYysl8DFpVIexM7wACzOVLGIjvWoSqD_gOxgxQu5mA
=== Sending Messages ===
✓ Message 1 sent
  Response: The available product is "ekoDB", which is a high-performance database product with AI capabilities.

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: H5P5CaHtrPCH6stJ0jPZSyZO3rIx0vgp3jaEIiNh4UMTCO_XkkFZf9BfBuYtI2QFnCzHh-csCQEnGstzEKav_w
  Parent: 4BJU8CUhE7wK9eBg_tXVfWzd-xoRSNQaJvE9j4aZcul7zYysl8DFpVIexM7wACzOVLGIjvWoSqD_gOxgxQu5mA

=== Listing Sessions ===
✓ Found 3 sessions
  Session 1: H5P5CaHtrPCH6stJ0jPZSyZO3rIx0vgp3jaEIiNh4UMTCO_XkkFZf9BfBuYtI2QFnCzHh-csCQEnGstzEKav_w (Untitled)
  Session 2: 4BJU8CUhE7wK9eBg_tXVfWzd-xoRSNQaJvE9j4aZcul7zYysl8DFpVIexM7wACzOVLGIjvWoSqD_gOxgxQu5mA (Untitled)
  Session 3: DcLiiFvuMZQ0jNmXFlxO-028DyjsLecXVNmcPmQuWrR-3Dwbx99wRtK3uS-9YGYwDiFqoD-Wa7sA1oEN4Sq7Hw (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: H5P5CaHtrPCH6stJ0jPZSyZO3rIx0vgp3jaEIiNh4UMTCO_XkkFZf9BfBuYtI2QFnCzHh-csCQEnGstzEKav_w

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.43s
     Running `target/debug/examples/client_chat_advanced`
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: DdmY-DgMt9SkDKzHreBLSbYvdrggGv9SHfMUW2r1zDgZDx26Gvy35DPpFl3O5Or6G-oao9HJN-I4pjamN9HV2g

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information available, the product ekoDB is available. It is described as a high-performance database product. The price of this product is $99.

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
✓ Created second session: 0dBcE6CuGPe38twSLr6KwkhV4MWkOcyUILsxWuw1XuDS6lcHD9gKIUGow5S05ZNUmWnef4DZqYv4Z5Wg4QZaeg
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
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.45s
     Running `target/debug/examples/client_functions`
=== ekoDB Rust Client - Scripts Example ===

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: AV5XwOBfJMo6HFQKBVztr4vrwmj_70NoM8FtmBWk1ubut01_d1XmH1V9rk3P_pMkvl8Mrhh6CAOlD-DxqFCRag
📊 Found 20 active users

📝 Example 2: Parameterized Script

✅ Script saved: -dIb5S3-S7Ma4o7b9gombaqeLu6M7Q6RvRzbhMIMR3ugCDpMKOZctuuB76nmpl7yJ2fqIFyzF-NLdYY9sutOFg
📊 Found 20 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: ba9QCexYJw4vx5RiloPrI6XB93TUke9iPWj-hfRxPUreVZajDgoFbJ_WzTUaAcX_97dYv4l0kvbq3fcINpSnTQ
📊 Statistics: 2 groups

📝 Example 4: Script Management

📋 Total scripts: 6
🔍 Retrieved script: Get Active Users
✏️  Script updated
🗑️  Script deleted

ℹ️  Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.


✅ All examples completed!
   Compiling ekodb-examples v0.1.0 (/Users/tek/Development/ekoDB/ekodb-client/examples/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.46s
     Running `target/debug/examples/client_transactions`
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: KBZKXeM4Ih63NCXgN2eDtgCY8Emkt1cpqJl5XlHpbzIyikbS87QGDK0Pi5TfpDaS5wOmxbK4-HX7Unjh84JBew
Created Bob: $500 - ID: evOBdllRQFEgjKI41tTWLGbeYX4trqOISENSIimLMCMxRxGUNNy91POKNWo0pCbuCBCvzX_T8LFYxYR2cJUTbg

=== Example 1: Begin Transaction ===
Transaction ID: 514b16b6-e9c7-4c29-9126-eee361bba512

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: "Active"
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 5c1b4e72-f377-4063-a96d-a589961d40e6
Status before rollback: "Active"
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
✅ [32mRust client examples complete![0m
