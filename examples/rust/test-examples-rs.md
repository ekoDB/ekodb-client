make test-examples-rust
🧪 [36mRunning Rust examples (direct HTTP/WebSocket)...[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.14s
     Running `target/debug/examples/simple_crud`
✓ Authentication successful

=== Insert Document ===
Inserted: Object {"id": String("WAf2n872sOMxB2H4sjm1MWAfgTEKdeI-9lf7uUFtfnm39xzIUZEB24cHhjlJQ78BhimWKctOJEO8BNz3Jb3pPg")}

=== Find by ID ===
Found: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("WAf2n872sOMxB2H4sjm1MWAfgTEKdeI-9lf7uUFtfnm39xzIUZEB24cHhjlJQ78BhimWKctOJEO8BNz3Jb3pPg"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}

=== Find with Query ===
Found documents: Array [Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("WAf2n872sOMxB2H4sjm1MWAfgTEKdeI-9lf7uUFtfnm39xzIUZEB24cHhjlJQ78BhimWKctOJEO8BNz3Jb3pPg"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}]

=== Update Document ===
Updated: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("WAf2n872sOMxB2H4sjm1MWAfgTEKdeI-9lf7uUFtfnm39xzIUZEB24cHhjlJQ78BhimWKctOJEO8BNz3Jb3pPg"), "name": Object {"type": String("String"), "value": String("Updated Record")}, "value": Object {"type": String("Integer"), "value": Number(100)}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/simple_websocket`
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: "j7cabfp3zf1fqbuWvsaWueFgmiV_kbeJ8-iQm7irQ6aYmr3XJKzZBVeOS5sytzHo_wrxas5BYrKHWjfI6lwWXQ"

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
        "id": "j7cabfp3zf1fqbuWvsaWueFgmiV_kbeJ8-iQm7irQ6aYmr3XJKzZBVeOS5sytzHo_wrxas5BYrKHWjfI6lwWXQ",
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
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
Collection created with first record: "pnCK12VcVFq2215BmFZ8r2pdbPlExEx0XtoFClFpID2jrycPI6bYfwXFqPrLfi8ig_6H72uxi_29PC5LKDzoQA"

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
✓ Inserted document: "B-ZCmj2kB-gfQw5fl4Zafd6gk6XPNwAwe_8JAiQUqTDAN_kqc7NrzWZJzJgIiqA9JZzjki9wfbtBPqRqMYm75A"

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
✓ Inserted document with TTL: String("TjWU3ouzQqTLqRevM3nwWniT8c1l6PavUMYCa7UidyDTv16dI0njdP55ozkc_5LBb32J_V-gJsd8qjVe9_EqfQ")

=== Query via WebSocket ===
✓ WebSocket connected

✓ WebSocket TTL example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/http_functions`
🚀 ekoDB Scripts Example (Rust/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: yNooYgBM0jBnCL838EP3-EfHUjlPK4MdF44780gf8Y-EWxfhulzT5F-_pyUaJxe3LAeupwIQ7P8Ncleud4jCzA
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: "nZav0S8AbRFRz0aHHnKkPDC-qAhb78o3ua8n9wmILww1Hb5zZ3pFZvAIjDfIpEnV0yfQ-OrxD54JVzeZJYsSMA"
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Complex Filter with Multiple Conditions

✅ Script saved: _h63jLUHKMFp9Q7NfWS_OFlNbsoOCivERcol0WFT6sVHBjqlYCZg0SVUg0LvBkzBV7oiE4vV98r8VGfUoSO2tA
📊 Found 3 users (status=active, score>50, sorted by score)

📝 Example 4: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: fBByT4f0PSfKhpxkPwlRMdXqgdwDiZOA9D5175769Gy7LFYneeg0QHQCD5tYwcSfPPddwdCh5CGgboJLGhhWOw
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/transactions`
✓ Authentication successful

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 8FBiPuB1tFjcJE4nJ-EWQI7fRPhiNba-mRuOi5LE5EAX7Uqh6Ei4Oa1HdUsY927sdHCoZQxcWW0r-IoT2qs4uA
Created Bob: $500 - ID: loZFLlH5TUAvC264MAANQwFLRja-29wJmlukgHGRZv74SkTlR_bif5K-RIqSARAUSLXpRPpTYiYV-yXDopLa6Q

=== Example 1: Begin Transaction ===
Transaction ID: 153e1d3e-9057-4e48-ac49-4877fc85a13c

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
New transaction: 6a36cbaa-a338-42c5-b2a5-31db0184cabf
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_simple_crud`
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: Record { fields: {"id": String("si_ZyUSgFSZe9A_tbYEX79KRyztq-2WvD3SaRwnCwGqRLJ1LU2D3qiIjRloGEkjKJMSqEmakJYFXXLYgPikWiA")} }

=== Find by ID ===
Found: Record { fields: {"data": Object({"type": String("String"), "value": String("aGVsbG8gd29ybGQ=")}), "value": Object({"value": Integer(42), "type": String("Integer")}), "embedding": Object({"type": String("Array"), "value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)])}), "metadata": Object({"type": String("Object"), "value": Object({"key": String("value"), "nested": Object({"deep": Boolean(true)})})}), "active": Object({"type": String("Boolean"), "value": Boolean(true)}), "categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])}), "user_id": Object({"value": String("550e8400-e29b-41d4-a716-446655440000"), "type": String("String")}), "tags": Object({"type": String("Array"), "value": Array([String("tag1"), String("tag2"), String("tag3")])}), "created_at": Object({"type": String("DateTime"), "value": String("2026-01-02T02:15:55.254870Z")}), "id": String("si_ZyUSgFSZe9A_tbYEX79KRyztq-2WvD3SaRwnCwGqRLJ1LU2D3qiIjRloGEkjKJMSqEmakJYFXXLYgPikWiA"), "name": Object({"value": String("Test Record"), "type": String("String")}), "price": Object({"type": String("Float"), "value": Float(99.99)})} }

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Some("Test Record")
  value (Integer): Some(42)
  active (Boolean): Some(true)
  price (Decimal): Some(99.99)
  created_at (DateTime): Some("2026-01-02T02:15:55.254870Z")
  user_id (UUID): Some("550e8400-e29b-41d4-a716-446655440000")
  tags (Array): 3 items
  metadata (Object): 2 keys
  embedding (Vector): 5 dims
  categories (Set): 2 items
  data (Bytes): 11 bytes

=== Find with Query ===
Found documents: [Record { fields: {"created_at": Object({"type": String("DateTime"), "value": String("2026-01-02T02:15:55.254870Z")}), "id": String("si_ZyUSgFSZe9A_tbYEX79KRyztq-2WvD3SaRwnCwGqRLJ1LU2D3qiIjRloGEkjKJMSqEmakJYFXXLYgPikWiA"), "value": Object({"value": Integer(42), "type": String("Integer")}), "data": Object({"value": String("aGVsbG8gd29ybGQ="), "type": String("String")}), "active": Object({"type": String("Boolean"), "value": Boolean(true)}), "price": Object({"type": String("Float"), "value": Float(99.99)}), "embedding": Object({"type": String("Array"), "value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)])}), "categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])}), "user_id": Object({"type": String("String"), "value": String("550e8400-e29b-41d4-a716-446655440000")}), "tags": Object({"value": Array([String("tag1"), String("tag2"), String("tag3")]), "type": String("Array")}), "metadata": Object({"value": Object({"nested": Object({"deep": Boolean(true)}), "key": String("value")}), "type": String("Object")}), "name": Object({"value": String("Test Record"), "type": String("String")})} }]

=== Update Document ===
Updated: Record { fields: {"metadata": Object({"type": String("Object"), "value": Object({"nested": Object({"deep": Boolean(true)}), "key": String("value")})}), "id": String("si_ZyUSgFSZe9A_tbYEX79KRyztq-2WvD3SaRwnCwGqRLJ1LU2D3qiIjRloGEkjKJMSqEmakJYFXXLYgPikWiA"), "categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])}), "data": Object({"value": String("aGVsbG8gd29ybGQ="), "type": String("String")}), "name": Object({"value": String("Updated Record"), "type": String("String")}), "price": Object({"type": String("Float"), "value": Float(99.99)}), "embedding": Object({"type": String("Array"), "value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)])}), "active": Object({"type": String("Boolean"), "value": Boolean(true)}), "created_at": Object({"type": String("DateTime"), "value": String("2026-01-02T02:15:55.254870Z")}), "user_id": Object({"type": String("String"), "value": String("550e8400-e29b-41d4-a716-446655440000")}), "value": Object({"value": Integer(100), "type": String("Integer")}), "tags": Object({"type": String("Array"), "value": Array([String("tag1"), String("tag2"), String("tag3")])})} }

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
Collection created with first record: "AcuxyESjB34huUMeOrkiDbFUNZVNXJ2Yggj72eoY4MWhLE5zAvRnIxXLyVY8HHd77CP5X6by690WqRaOf6RQiA"

=== List Collections ===
Total collections: 9
Sample collections: ["test_accounts", "client_collection_management_rust", "test_collection", "users", "batch_users"]

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
✓ Inserted document: "X6Gs62WZsHsQEMZRWQG2bIKYpylHxOPezhTPJqfV7MJdXIza8_X_yOoHxPWEg5zzLyJnR_xhB4LdaBuJ9g-OHw"

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: Some(String("ovSUkK1hY1Gb-NIBWpwL64nHIBXunbjqozMlll-_YEgjlJjyFEwptD9ywyBvqIG_OseO-ouiN7gWXxUWvONBmg"))

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
✓ Inserted test record: nZRyyCcU7ysLEpkytGa91HJAUDzNF7J7_OOYdJllaW11daVuuAdRGgnsQSWUAlblxCa5ITD7iMu_4fsaWHLHhQ

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
✓ Inserted document with TTL: Some(String("wvnLTLyraXJiM-GA4OMFUWKK9kB0r8M1a7IItzHTDEBganffw2ERHLZD13vdKHfWk6EA2nLuicwlBmEOwxBaAA"))

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
  - Some(Object({"value": String("David"), "type": String("String")}))

=== Range Query (age >= 28 AND age < 35) ===
✓ Found 3 users in age range
  - Some(Object({"value": String("Bob"), "type": String("String")}))
  - Some(Object({"value": String("Eve"), "type": String("String")}))
  - Some(Object({"value": String("David"), "type": String("String")}))

=== IN Operator ===
✓ Found 4 users with status active or pending
  - Some(Object({"type": String("String"), "value": String("Eve")}))
  - Some(Object({"type": String("String"), "value": String("Alice")}))
  - Some(Object({"value": String("David"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("Bob")}))

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
  - Some(Object({"type": String("String"), "value": String("Alice")}))
  - Some(Object({"type": String("String"), "value": String("Charlie")}))

=== Sorted Query (by score descending) ===
✓ Top 3 users by score:
  1. Some(Object({"value": String("Bob"), "type": String("String")}))
  2. Some(Object({"value": String("David"), "type": String("String")}))
  3. Some(Object({"value": String("Alice"), "type": String("String")}))

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
  2. Score: 6.6000 - Some(String("JavaScript Web Development"))

=== Cleanup ===
✓ Deleted collection

✓ All search operations completed successfully
Execution time: 1ms
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_schema_management`
=== ekoDB Schema Management Example ===

=== Creating Collection with Schema ===
✓ Created collection 'schema_client_rust' with schema

=== Inserting Valid Documents ===
✓ Inserted user 1: Some(String("R7I8ZtnQCY0ptKTTzP71HO-vMh3Qt7eX9EJSAjutaiJBl1PrbpRwj_1RGIr5S-fomp0CPTdvPmCqT2ejbUrErg"))
✓ Inserted user 2: Some(String("-bFWNjVh58jtJqh3nIC7CpYRKajFTEkwQjRMlibWM1A-8Il2UOBnzVA43t4cHsFNxxLKd-s57LGaLId_HuCs0A"))

=== Getting Schema ===
✓ Schema for schema_client_rust:
  - status: String
  - title: String
    (required)
  - email: String
    (required)
  - age: Integer

=== Listing Collections ===
✓ Total collections: 9
  Sample: ["test_accounts", "test_collection", "schema_client_rust", "users", "batch_users"]

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
     Running `target/debug/examples/client_chat_basic`
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: EIki62qqB5lU5m6nt01y8UfVL_LgF66otheWB_KKRJMELVCaNA2lRWOQuszEK7iX-FhyXi-qmmcIhK7m9S_YMw

=== Sending Chat Message ===
Message ID: tmKfgGvmlCOnTsbSMD_6ybH34fWbm6dp7KdGH_1RjNs0sub2cu1sS7NsAiZdOeOLDv3wF7swQBqlJPuRmRg4bw

=== AI Response ===
Response 1: ekoDB is a high-performance database known for its intelligent caching, real-time capabilities, and AI integration. It offers several notable features. One of these is the AI Chat Integration, which allows you to query your database using natural language and receive AI-powered responses with relevant context. ekoDB also supports full-text search, vector search, and hybrid search with automatic context retrieval. These features make it an effective and versatile tool for database management.

=== Context Used (3 snippets) ===

Snippet 1:
  Collection: client_chat_basic_rust
  Score: 0.6333
  Matched Fields: ["category", "title", "content"]
  Record: Object {"category": String("features"), "content": String("The chat feature allows you to query your database using natural language and get AI-powered responses with relevant context."), "id": String("oq3iWKATjscUbc9f5uqBYZ_wXBVMNUDuI2O8IxS-l-sXW7uBjny-vVKiTQ1zo4lvfjpzFfbWbxxncYVVIJvYdw"), "title": String("AI Chat Integration")}

Snippet 2:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["content", "title", "category"]
  Record: Object {"category": String("features"), "content": String("ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval."), "id": String("cuRWj20CBRxdy9vofxJgNVnos_BhcznBKuV1-hy-L-icPUhXiuXZzdaBX6EJJ22OFBinpNMBv2bo44OVaK0oeA"), "title": String("Search Features")}

Snippet 3:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["content", "title"]
  Record: Object {"category": String("documentation"), "content": String("ekoDB is a high-performance database with intelligent caching, real-time capabilities, and AI integration."), "id": String("KYStf4pbkPcFkA0JbqX75a0rEr3yGk3nOyCQGqekjFL5oo8BXgmVvkmHsci9xLzXWL1XCXObfV84cEnQ8He7SA"), "title": String("Introduction to ekoDB")}

Execution Time: 2553ms

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_chat_sessions`
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: ipy1SsZHmJi1lwpkk_PUStFrqpVFSIw6gysmgKGyw0jZRdO3MHb5ubGGqXzRzjxFLn5IOhTi_xUNuuO_97A0hg
=== Sending Messages ===
✓ Message 1 sent
  Response: The product available is the ekoDB. It is a high-performance database product with AI capabilities. The cost of this product is $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product with AI capabilities, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 50twgwd55HpVRfTVuO-aKsUzOKPiIpQTw5UDhEx2AkHwkvVvliYOKXllwQzmwiG9GXoNAZ4JsYvGzcmyZJYgPg
  Parent: ipy1SsZHmJi1lwpkk_PUStFrqpVFSIw6gysmgKGyw0jZRdO3MHb5ubGGqXzRzjxFLn5IOhTi_xUNuuO_97A0hg

=== Listing Sessions ===
✓ Found 3 sessions
  Session 1: 50twgwd55HpVRfTVuO-aKsUzOKPiIpQTw5UDhEx2AkHwkvVvliYOKXllwQzmwiG9GXoNAZ4JsYvGzcmyZJYgPg (Untitled)
  Session 2: ipy1SsZHmJi1lwpkk_PUStFrqpVFSIw6gysmgKGyw0jZRdO3MHb5ubGGqXzRzjxFLn5IOhTi_xUNuuO_97A0hg (Untitled)
  Session 3: EIki62qqB5lU5m6nt01y8UfVL_LgF66otheWB_KKRJMELVCaNA2lRWOQuszEK7iX-FhyXi-qmmcIhK7m9S_YMw (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: 50twgwd55HpVRfTVuO-aKsUzOKPiIpQTw5UDhEx2AkHwkvVvliYOKXllwQzmwiG9GXoNAZ4JsYvGzcmyZJYgPg

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_chat_advanced`
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: eQmuJDrO_BDSFdXKD3qqi406LAIkWMPGuJdabi0hCinHR1QVF-xSQeNfV-2iHoa1ZYudz-v0BQDItucS7i2enA

=== Sending Initial Message ===
✓ Message sent
  Response: The available product based on the provided context is a high-performance database product named "ekoDB". It is priced at 99 units.

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
✓ Created second session: M5jfgFMLXAGf8KQ-Kc3oeMIk1LG1uVKrEIKZP_wh993kb2mGcllzluKQ5mV-o6qZU1qofEi4eQOSEA6BACYP8g
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_functions`
=== ekoDB Rust Client - Scripts Example ===

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: AtzUt7lvP5PXp1kEfiKcGZm9LoqRvYo1BwtmLRhos5SQejMbXXwwdV2URTvWuW7zCW_87xPssQg18tO_lq_6cg
📊 Found 20 active users

📝 Example 2: Parameterized Script

✅ Script saved: 4KSxXfLX6Csetm7Rq0QP7rtF0lG_4qlkKWyJEWf3aKFpPt5dTcS4y8r7izUAEKv6rXAGI4FgISyw7V7tRP5EhA
📊 Found 20 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: fqFEjtko67RdookO_mZv8VW5rcAJkEbgfMXjjfFRcZXlK3gbvQCxtGm14aDswSTDCuZyxLTHu_vbpALFlZpTbA
📊 Statistics: 2 groups

📝 Example 4: Script Management

📋 Total scripts: 6
🔍 Retrieved script: Get Active Users
✏️  Script updated
🗑️  Script deleted

ℹ️  Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.


✅ All examples completed!
✅ [32mRust client examples complete![0m
