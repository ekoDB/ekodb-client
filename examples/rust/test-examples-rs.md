make test-examples-rust
🧪 [36mRunning Rust examples (direct HTTP/WebSocket)...[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/simple_crud`
✓ Authentication successful

=== Insert Document ===
Inserted: Object {"id": String("RPMfdB-QXlsfzqiLZ8uFFCOPwvOQ0XdIgW0oqIqfgojM48MrtHwPFOmJom8IxCg_4G4fopsWUlEZ0E4JWu0EOw")}

=== Find by ID ===
Found: Object {"active": Bool(true), "id": String("RPMfdB-QXlsfzqiLZ8uFFCOPwvOQ0XdIgW0oqIqfgojM48MrtHwPFOmJom8IxCg_4G4fopsWUlEZ0E4JWu0EOw"), "name": String("Test Record"), "value": Number(42)}

=== Find with Query ===
Found documents: Array [Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("RPMfdB-QXlsfzqiLZ8uFFCOPwvOQ0XdIgW0oqIqfgojM48MrtHwPFOmJom8IxCg_4G4fopsWUlEZ0E4JWu0EOw"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}]

=== Update Document ===
Updated: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("RPMfdB-QXlsfzqiLZ8uFFCOPwvOQ0XdIgW0oqIqfgojM48MrtHwPFOmJom8IxCg_4G4fopsWUlEZ0E4JWu0EOw"), "name": Object {"type": String("String"), "value": String("Updated Record")}, "value": Object {"type": String("Integer"), "value": Number(100)}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/simple_websocket`
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: "9qANfwDEYn4jKGx25UL0lEibKplJLTThUsJZjNTuw-Mndg3CY8pV24Wc-19KGmdxxFpyeTosNU8sJe5uomjbCQ"

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
        "id": "9qANfwDEYn4jKGx25UL0lEibKplJLTThUsJZjNTuw-Mndg3CY8pV24Wc-19KGmdxxFpyeTosNU8sJe5uomjbCQ",
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
Collection created with first record: "_oexNV6Ky-DIDtTkGndYMVSPUFBE72shIMu9qt9T-tt93xqWNdqrouHeFbqeqd2L2DVw82JCIFoTnwSzFnnXwQ"

=== List Collections ===
Total collections: 4
Sample collections: ["demo_collection", "websocket_test", "batch_users", "test_collection"]

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
✓ Inserted document: "YAMKtk-GkOkcu1Nto-9xIoCS0A5WxVG3fyy5FPHenwVzWGrcTwN4DRWZ-j_Wp-cbd8YCEZAx4Hl99Qem_biigA"

=== Insert Document with TTL (5 minutes - integer) ===
✓ Inserted document: "Wnc9177pAFPkWbKu6Qudb-GYCcypcny1rdMRN3FCCRszL5e0gEJzkkLHTWg1Vo90jdRQ_mUIfnCelBvXgQLmYQ"

=== Insert Document with TTL (30 minutes - duration string) ===
✓ Inserted document with duration string TTL: "1DafgC3gU4taG7VFsu1hJgAQ8d1e1Xsn6XXzu3pUNTXtuWCLwR8kZ_V5RKlg6OB_L5iisXDbG9yiaPBpZZ3UhQ"

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
✓ Inserted document with TTL: String("K6K_mlvC-6x1uBJHxAA6Uqhhh69G0fmvfHR9bvW77mivmlOHYl8_zbm9o1wBOMTrMDWD09PqV79AAzAqvBSUng")

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket

Record 1:
  id: "K6K_mlvC-6x1uBJHxAA6Uqhhh69G0fmvfHR9bvW77mivmlOHYl8_zbm9o1wBOMTrMDWD09PqV79AAzAqvBSUng"
  name: {"type":"String","value":"WebSocket TTL Test"}
  ttl: "2026-01-26T21:17:45.615415Z"
  value: {"type":"Integer","value":42}

✓ WebSocket TTL example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/http_functions`
🚀 ekoDB Scripts Example (Rust/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: 3GisEPK432tHGs6CsmAlUF7KZrWH9QAzWvVTrVPdwvSlnRKkwH_T6VMuARJlXIv-nERbAyOd2YvzFoHn_Up7mw
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: "LTvj4D4_AGiZxxbyJcFgkLXfsk4AhIsuLssnjJwuTQrjWF-P2yhp3U3912gq3w5DTY819usiGqgQls9N_fzYFQ"
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Complex Filter with Multiple Conditions

✅ Script saved: B1MdmA9PkaLQNDp6ROrDje-c7sSofJN2uv36QQbcV2JVivOdpigi7bBl6-k58_hSDlM12HvO_L6XiMyMveH4Kw
📊 Found 3 users (status=active, score>50, sorted by score)

📝 Example 4: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: Xkc-rox2FbRYyxJEcLzjLAumXwK6eShkLvlK4ua5zmcmI7AzaaaJATM7nUnkkHNgHVx8wkdyCt2EhZ7yMCkt7w
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
Created Alice: $1000 - ID: HOqkSahDX1_kqJ7G5neYEReigk7PLL9DeQcqqmCDMU7XrlAexXcJ6ajwqO3tMRPrSNoW3pZtG0P6iZ_sL2dXxg
Created Bob: $500 - ID: DmSQDpfgXXxYMhKdN2Nm-XglNTYRljt5NbaYFCtSwUz0YkwjhCW-yLP-Q9bxtE5BdbzBs9RHBKYc4Neg8qQtTg

=== Example 1: Begin Transaction ===
Transaction ID: 169aacb0-c3b5-4c90-a09a-05c3e3bd4790

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
New transaction: dd62ddf1-af6e-4977-95a9-50e4b3505dc3
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_simple_crud`
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: Record({"id": String("c6jEkc5aUVCzfG0cX8Yo8pq6-QfBE13aPoHdAInsvGnEhpsqQb59JBIycF0vzlJmj2zh4v0O1w9tatdWPpiu3Q")})

=== Find by ID ===
Found: Record({"metadata": Object({"key": String("value"), "nested": Object({"deep": Boolean(true)})}), "price": Float(99.99), "created_at": String("2026-01-26T20:17:51.132513+00:00"), "embedding": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "categories": Array([String("electronics"), String("computers")]), "user_id": String("550e8400-e29b-41d4-a716-446655440000"), "data": String("aGVsbG8gd29ybGQ="), "id": String("c6jEkc5aUVCzfG0cX8Yo8pq6-QfBE13aPoHdAInsvGnEhpsqQb59JBIycF0vzlJmj2zh4v0O1w9tatdWPpiu3Q"), "tags": Array([String("tag1"), String("tag2"), String("tag3")]), "value": Integer(42), "active": Boolean(true), "name": String("Test Record")})

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Some("Test Record")
  value (Integer): Some(42)
  active (Boolean): Some(true)
  price (Decimal): Some(99.99)
  created_at (DateTime): Some("2026-01-26T20:17:51.132513+00:00")
  user_id (UUID): Some("550e8400-e29b-41d4-a716-446655440000")
  tags (Array): 3 items
  metadata (Object): 2 keys
  embedding (Vector): 0 dims
  categories (Set): 0 items
  data (Bytes): 11 bytes

=== Find with Query ===
Found documents: [Record({"categories": Object({"value": Array([String("electronics"), String("computers")]), "type": String("Array")}), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "metadata": Object({"value": Object({"key": String("value"), "nested": Object({"deep": Boolean(true)})}), "type": String("Object")}), "embedding": Object({"value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "type": String("Array")}), "data": Object({"type": String("String"), "value": String("aGVsbG8gd29ybGQ=")}), "id": String("c6jEkc5aUVCzfG0cX8Yo8pq6-QfBE13aPoHdAInsvGnEhpsqQb59JBIycF0vzlJmj2zh4v0O1w9tatdWPpiu3Q"), "tags": Object({"type": String("Array"), "value": Array([String("tag1"), String("tag2"), String("tag3")])}), "user_id": Object({"type": String("String"), "value": String("550e8400-e29b-41d4-a716-446655440000")}), "created_at": Object({"value": String("2026-01-26T20:17:51.132513Z"), "type": String("DateTime")}), "name": Object({"type": String("String"), "value": String("Test Record")}), "value": Object({"type": String("Integer"), "value": Integer(42)}), "price": Object({"value": Float(99.99), "type": String("Float")})})]

=== Update Document ===
Updated: Record({"price": Object({"value": Float(99.99), "type": String("Float")}), "user_id": Object({"type": String("String"), "value": String("550e8400-e29b-41d4-a716-446655440000")}), "value": Object({"type": String("Integer"), "value": Integer(100)}), "metadata": Object({"type": String("Object"), "value": Object({"nested": Object({"deep": Boolean(true)}), "key": String("value")})}), "data": Object({"type": String("String"), "value": String("aGVsbG8gd29ybGQ=")}), "created_at": Object({"value": String("2026-01-26T20:17:51.132513Z"), "type": String("DateTime")}), "embedding": Object({"type": String("Array"), "value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)])}), "active": Object({"type": String("Boolean"), "value": Boolean(true)}), "name": Object({"value": String("Updated Record"), "type": String("String")}), "categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])}), "id": String("c6jEkc5aUVCzfG0cX8Yo8pq6-QfBE13aPoHdAInsvGnEhpsqQb59JBIycF0vzlJmj2zh4v0O1w9tatdWPpiu3Q"), "tags": Object({"type": String("Array"), "value": Array([String("tag1"), String("tag2"), String("tag3")])})})

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_simple_websocket`
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: dWgiVnMQrcwza29PLjS8NihLXwmrvV8vMgJXYnS-mOY8A4NYfjbGnJT6NjyLSYY98G1nTvCChmRU2wixMy9vbg

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket

Record 1:
  active: true
  id: "dWgiVnMQrcwza29PLjS8NihLXwmrvV8vMgJXYnS-mOY8A4NYfjbGnJT6NjyLSYY98G1nTvCChmRU2wixMy9vbg"
  name: "WebSocket Test Record"
  value: 42

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
Collection created with first record: "DcNt4D4TD0zPGbmDVGAjI2Ouhki8BeBQxt3jrTpBmm7EqUAzLL49NqWu3r_waFx4HUpxAPlU5DkroXcuMFLPYQ"

=== List Collections ===
Total collections: 9
Sample collections: ["websocket_test", "batch_users", "scripts__ek0_testing", "test_accounts", "client_collection_management_rust"]

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
Created Alice: $1000 - ID: Hok3dwE7tEVPV7AHJArfROjvi_PeJihGdcDvPaYHo3bes5VpBugCEl0UMh-IwLGFwdD1MieCn0puiy7eIKxExQ
Created Bob: $500 - ID: kMNUuzPIHUmRDLnChSOJ1p2MSTvl6V7chrqUM8Vg7QohvwB5aHbmZ9sQD8vd3_JoaISK2XCjuUhgvmJdSh6RMw

=== Example 1: Begin Transaction ===
Transaction ID: 03269339-b0a6-411e-8d3a-646ab1960b72

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: "Active"
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 68dd988d-ef19-416d-affe-db5ecbde4b83
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
  - Some(Object({"value": String("Alice"), "type": String("String")}))
  - Some(Object({"value": String("David"), "type": String("String")}))
  - Some(Object({"value": String("Bob"), "type": String("String")}))

=== Range Query (age >= 28 AND age < 35) ===
✓ Found 3 users in age range
  - Some(Object({"type": String("String"), "value": String("David")}))
  - Some(Object({"value": String("Bob"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("Eve")}))

=== IN Operator ===
✓ Found 4 users with status active or pending
  - Some(Object({"value": String("Alice"), "type": String("String")}))
  - Some(Object({"value": String("David"), "type": String("String")}))
  - Some(Object({"value": String("Bob"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("Eve")}))

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
  - Some(Object({"value": String("Alice"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("Charlie")}))

=== Sorted Query (by score descending) ===
✓ Top 3 users by score:
  1. Some(Object({"value": String("Bob"), "type": String("String")}))
  2. Some(Object({"type": String("String"), "value": String("David")}))
  3. Some(Object({"value": String("Alice"), "type": String("String")}))

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
     Matched: ["title", "description.value", "description", "title.value"]
  2. Score: 2.0000
     Title: Some(String("Python for Data Science"))
     Matched: ["description", "description.value"]
  3. Score: 1.0000
     Title: Some(String("Database Design"))
     Matched: ["description.value", "description"]
  4. Score: 1.0000
     Title: Some(String("Rust Programming"))
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
Execution time: 2ms
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_schema`
=== ekoDB Schema Management Example ===

=== Creating Collection with Schema ===
✓ Created collection 'schema_client_rust' with schema

=== Inserting Valid Documents ===
✓ Inserted user 1: Some(String("tT_mZ9eVwmz7rCh2lf5u6X51YUSiaXSFnT07Wy5PCWsKKNQNq8rgY2YsHEyc-MoLUCHvYyd5BW4eLRKiaa8X4A"))
✓ Inserted user 2: Some(String("gGlk3HzYqRv0IcnlCi0mCsJHENdOH0tNVMTVHH6BfKspTvxXxUkxfo5AHwKDjcIyw9n_KIlF_oJvMUv-dIOKnA"))

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
  Sample: ["websocket_test", "batch_users", "scripts__ek0_testing", "test_accounts", "users"]

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_document_ttl`
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: "AEqaE0539NyOpy3URl0pAxeUcPbUSwiwaXQkzfBYDIGX2_IM8ejn9P0exr9LKARrEC24SP3_-FCJbCCk8DFOxw"

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: Some(String("MerAF0qIGtJabOWQ0LUoTp2xTiWdNU9MnUCg0DmL0QsP1UztQD2ciwXP1hux6HaKS-wOnGDz2v7tHLmqg5dzCA"))

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
✓ Inserted document with TTL: Some(String("wuevybBxWzYan-Z3VnjVHksXhl_ELsfNS9hre8nGRdTtCumzv5c6Rh7xx3hCv6vPk6k3BY9CPiCzMhOlM_F9Fg"))

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
✓ Edge cache script created: R0jT1i_E3M1wYCYw1AoC1AiyTX2Qd2jdqgrGo3TJ-9iEhpfhKkeDZbcPUR25Zhsfhj7YoSaL58F36IWGqgTpNQ

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_functions`
=== ekoDB Rust Client - Scripts Example ===

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: jH_V3jFsBKbEjxhxNSgsy5_D39Fmz_hsq_9rnWlXO8gp9_GbGM2NnafReQOOc3Pi0b2moFTzQVssoXLH04oPLw
📊 Found 20 active users

📝 Example 2: Parameterized Script

✅ Script saved: BIRO1VzuJrSkPjw-3MboBO5OF0gfjo7EYYLTentRxP0YMSNZvLo61QJg2OEzUGL3aNC4ozPmTQDQ1dX0Y9TGwg
📊 Found 20 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: pquUjXDmfpf21y_vxSlU2eIrgA8DM9IkoWAx6OZs0WHORqqZppiYd7YD_gMEesNGHMtAhw8-XfI-wxQ3Rb6h2w
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

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 141.708416ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 3.957791ms
   📊 Records: 0
   🚀 Cache speedup: 47.0x faster!

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

✅ Script saved: Kj_qWNIjS8hfuOmHDQJN2tTQiT5rQlwDLQJ9zYHfwwGOaVQs1s7YA-pLrmqB6_BnkkadfYO_o8agQgGp5yrvFg
📊 Found 2 product groups
   Record({"count": Integer(3), "category": String("Electronics"), "avg_price": Float(575.6666666666666)})
   Record({"avg_price": Float(474.0), "category": String("Furniture"), "count": Integer(2)})
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved
📊 Found 2 categories
   Record({"category": String("Electronics"), "count": Integer(3)})
   Record({"category": String("Furniture"), "count": Integer(2)})
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

✅ Inserted order: Some(String("Iz-Sfis9xm9nKzR3h35JK5ytBe4zPMC8gWdBIf9MwtHFCb65dX9chPGLeFuwzt8l8w0Nkeb0TTr-I4R5Il8W8Q"))
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: 9ZiRkKBTzH3LzbfUHP0aYfqWPnsp7Lk-2z_J1Lk7L_V0cTdT2MzsETjFm3wO5iqI6eelPdK0y90uibtEv6AiIQ
📊 Script executed
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: Some(Object {"role": String("admin"), "userId": String("user_abc")})
🔍 Key exists: true
✅ Set cached data
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: Y1EX7keDGn-vzYV9UU-3W7k0Uao1kgDHlsymS295YIgzYzghWNTZgimKrcxjTbsU95HMOm0sNQfhSEoVBsRoVg
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Script

✅ Script saved: owhOB78oGAYoKjos0b28ZaKm_PNPe6VqA6ZoJ53lYJi2CgJegZdvYBUTvAtOt6oTazk2jrRsVyM_OLZikKqHQA
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_swr_pattern`
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_api_user_rs (3Fk8eZ8pyOWuQ50Z-liIE_frIJ1vkWGHoXzmTYxSAhL04-J3GLmofLp-jaclNDRrv4Hoyx2ALw7B6_M8WGPjxA)

Step 2: First call - Cache miss, fetches from API
Result: FunctionStats { input_count: 0, output_count: 1, execution_time_ms: 0, stages_executed: 2, stage_stats: [] }
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 88ms (served from cache)
✓ Lightning fast cache hit

🧹 Cleaning up...
✓ Cleanup complete

=== SWR Pattern Summary ===
✅ Cache miss → Fetch from API → Store in ekoDB
✅ Cache hit → Instant response from ekoDB
✅ TTL handles automatic cache invalidation
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_swr_native`
=== ekoDB Native SWR Function ===

This example shows the new simplified SWR function that replaces
the manual FindById → If → HttpRequest → Insert pattern.

Example 1: Basic GitHub User Cache with Native SWR
─────────────────────────────────────────────────────

✓ Created native SWR script: github_user_native (of3wXVEk4xGv_rCtOh3HPpKtLO9Mzo2EEAO_ykk9JCPMqPqq25phJhJlKUB9ThtltJulpBmR4MeuoOM-M5nH8Q)

First call (cache miss - will fetch from GitHub API):
  Response time: 145ms
  Records returned: 1
  ✓ Data fetched from API and cached with 15m TTL

Second call (cache hit - instant from KV store):
  Response time: 12ms
  Speedup: 12.1x faster
  ✓ Lightning fast cache hit


Example 2: SWR with Audit Trail Collection
─────────────────────────────────────────────────────

✓ Created SWR script with audit trail: product_swr_audit (LNKCEMcTSIzExf1G5mFI-_Oj1c8dlSqy50rxPV4PPeZ7Xw951B9YEypAUAJpGtYhmscjqJh_nvt59StyjxsxSg)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
─────────────────────────────────────────────────────

Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (Sm6UqRaawE6dwW7PdGUvDxS3_2nKfVk-_WnleRFUwmeFAaON30Af-4eesH1-EDRNBf72Sey9PwCTqL6ia7QUdA)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
─────────────────────────────────────────────────────

✓ Created dynamic TTL script: flexible_cache (MP4fioJYc9togqTYaqYRqKqU35A_Rbw6jJf4rd2Rme2YQh5OoW3Ahdjcei44sbfMyIWqMkFpWg7nhT7EDLlaEw)
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
   Record({"avg_price": Float(365.6666666666667), "count": Integer(3), "category": String("Furniture")})
   Record({"count": Integer(5), "avg_price": Float(367.0), "category": String("Electronics")})
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
   Vector databases offer benefits such as:

1. High Precision: They provide accurate data because they use points, lines, and polygons to represent real-world features.
2. Scalability: Vector data can be easily scaled up or down without losing quality.
3. Small File Size: They generally occupy less storage space compared to raster databases.
4. Complex Analysis: They support complex spatial analysis, such as network analysis and topology.
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
   Record({"status": String("active"), "count": Integer(7)})
   Record({"status": String("inactive"), "count": Integer(3)})
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All CRUD script examples finished!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_functions_search`
🚀 ekoDB Rust Search Scripts Example

📋 Setting up test data...
✅ Inserted 5 documents

📝 Example 1: List All Documents

✅ Script saved
📊 Found 5 documents
   1. Vector Databases Explained (Database)
   2. Introduction to Machine Learning (AI)
   3. Natural Language Processing (AI)
   4. Database Design Principles (Database)
   5. Getting Started with ekoDB (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Script saved
📊 Documents by category:
   Record({"count": Integer(2), "category": String("AI")})
   Record({"count": Integer(3), "category": String("Database")})
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
✓ Created session: haPvVPHhURHG2gR4H89asUbBWgrhgXueRh4xFZptY19J391BZb892uDfR09WI-AiyvEjF8XAXJ-0axNcGlFJew

=== Sending Chat Message ===
Message ID: Vi1Z4l_EEFeWOjTkbeFxIM874V4dYbC_C8j26E_Ki8qlCNP8m8hME5JVkw5N6XVqOPdwxp6-4bxo7qmEy6JHiw

=== AI Response ===
Response 1: ekoDB is a high-performance database that is designed with intelligent caching, real-time capabilities, and AI integration. This allows for quick and seamless database operations. One of the standout features of ekoDB is the AI Chat Integration. This feature enables you to query your database using natural language and receive AI-powered responses with relevant context, making it easier and more intuitive to navigate your database.

Moreover, ekoDB also supports full-text search, vector search, and hybrid search, with automatic context retrieval. This means you can search your database in various ways depending on your needs, and the system will automatically retrieve the relevant context for you. These features make ekoDB a powerful tool for managing and querying your data.

=== Context Used (3 snippets) ===

Snippet 1:
  Collection: client_chat_basic_rust
  Score: 0.6333
  Matched Fields: ["title", "category", "content"]
  Record: Object {"category": String("features"), "content": String("The chat feature allows you to query your database using natural language and get AI-powered responses with relevant context."), "id": String("yqSU1k9jF9N_9DchWhA45I-vufRFGt14sKq1anpH9Dr4wzX6OAKjkuH1S5gBoCGUntHsr95bqqFdbvhJilLiLA"), "title": String("AI Chat Integration")}

Snippet 2:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["title", "content"]
  Record: Object {"category": String("documentation"), "content": String("ekoDB is a high-performance database with intelligent caching, real-time capabilities, and AI integration."), "id": String("BTAYYGXml3tDb6KOKnrKRWoJptTG7G1JWUczeMyUcRHTHgdwJQv2DNZJ5A9JPodTJvtWBMThovn6St9-lhbppw"), "title": String("Introduction to ekoDB")}

Snippet 3:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["content", "category", "title"]
  Record: Object {"category": String("features"), "content": String("ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval."), "id": String("0J50ZcCYsVHvz7nJauFfXKuK_LVlJMUvdws-E5ebjCgGdi9Q0vmPj5raeOj8KcBukcnFLXnLOuqnhHVRKaTlvQ"), "title": String("Search Features")}

Execution Time: 5531ms

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_chat_advanced`
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: RE7GT_AIsa73SdbphTmKB0xAMhEMK9BBLqAzK12xH6A-6uDJ00UBrsxtbG5BH2zl1dwHS52tfUw_3PqmW5BpZQ

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
✓ Created second session: BwYFP0uVQGNlc6VCBff2PHtOXg5E3CgP3FUZ8bM1h3h0KsAH26riq9qTJpUkDs-zK5YmRyXuizzVil7impBBOA
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
     Running `target/debug/examples/client_chat_sessions`
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: B34WCoxf8nH3TbbVuxeVT7jxixrqEQikzOAkSa98N7JrmbrCvCu6OW1i12WHAEuGg8KNoC7IvEpOHjea5Xgvuw
=== Sending Messages ===
✓ Message 1 sent
  Response: The product available is called "ekoDB". It is a high-performance database product that comes with AI capabilities. The price of this product is $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product with AI capabilities, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: GTZjBahUI-vl-fIokCOQX_8VVbdsjuIJU0yqyxvm7rWGsZmxhcsFjbIRCwghATafuN-Mw1Ppc7Lwz6W1k3Q9xw
  Parent: B34WCoxf8nH3TbbVuxeVT7jxixrqEQikzOAkSa98N7JrmbrCvCu6OW1i12WHAEuGg8KNoC7IvEpOHjea5Xgvuw

=== Listing Sessions ===
✓ Found 4 sessions
  Session 1: GTZjBahUI-vl-fIokCOQX_8VVbdsjuIJU0yqyxvm7rWGsZmxhcsFjbIRCwghATafuN-Mw1Ppc7Lwz6W1k3Q9xw (Untitled)
  Session 2: B34WCoxf8nH3TbbVuxeVT7jxixrqEQikzOAkSa98N7JrmbrCvCu6OW1i12WHAEuGg8KNoC7IvEpOHjea5Xgvuw (Untitled)
  Session 3: BwYFP0uVQGNlc6VCBff2PHtOXg5E3CgP3FUZ8bM1h3h0KsAH26riq9qTJpUkDs-zK5YmRyXuizzVil7impBBOA (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: GTZjBahUI-vl-fIokCOQX_8VVbdsjuIJU0yqyxvm7rWGsZmxhcsFjbIRCwghATafuN-Mw1Ppc7Lwz6W1k3Q9xw

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_convenience_methods`
=== ekoDB Convenience Methods Example ===

=== Record Builder Pattern ===
✓ Created record with fluent builder: Record({"id": String("eUXrRTzQ7QkqGB6_sZ-bxJOuLc5x9a1BTrBesatYVA0Upm3O0-V9iKj7pZ96KP7Z5SDxgnLhSnT8vAT7gdrLLg")})

=== Upsert Operation ===
✓ First upsert (insert): Record({"active": Object({"value": Boolean(true), "type": String("Boolean")}), "id": String("eUXrRTzQ7QkqGB6_sZ-bxJOuLc5x9a1BTrBesatYVA0Upm3O0-V9iKj7pZ96KP7Z5SDxgnLhSnT8vAT7gdrLLg"), "name": Object({"value": String("Bob Smith"), "type": String("String")}), "email": Object({"type": String("String"), "value": String("bob@example.com")}), "age": Object({"value": Integer(35), "type": String("Integer")})})
✓ Second upsert (update): Record({"email": Object({"type": String("String"), "value": String("bob.smith@newdomain.com")}), "active": Object({"type": String("Boolean"), "value": Boolean(true)}), "name": Object({"value": String("Bob Smith"), "type": String("String")}), "age": Object({"type": String("Integer"), "value": Integer(36)}), "id": String("eUXrRTzQ7QkqGB6_sZ-bxJOuLc5x9a1BTrBesatYVA0Upm3O0-V9iKj7pZ96KP7Z5SDxgnLhSnT8vAT7gdrLLg")})

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
   Inserted with ripple: Record({"id": String("F7oC6_SyiypunbAft1lYYZ-nOhbV000RnpOadMkisQFp6dpcmde7_l0sVBv36DopwMc3Jt0-HicpJdjbtKDEWA")})

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: Record({"id": String("jvo03ItTEmhhiZnIdxATDpJVuA_nVosdgLbRU6phKyrd3vJBIPYVDaFzek_vO5Y6bDNoB-lGremwT5j8CjF7zA")})

3. Update with bypass_ripple:
   Updated with bypass_ripple: Record({"price": Object({"type": String("Integer"), "value": Integer(150)}), "id": String("F7oC6_SyiypunbAft1lYYZ-nOhbV000RnpOadMkisQFp6dpcmde7_l0sVBv36DopwMc3Jt0-HicpJdjbtKDEWA"), "name": Object({"type": String("String"), "value": String("Product 1")})})

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: Record({"id": String("piSi98ulqgonhuOwMiPr2jyA7gjkW3MS4EdOm5q7bSvpyvSAvowzQIrkqNje6lFwzsTPHOgolHeyiJm8qsaNXg")})

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
