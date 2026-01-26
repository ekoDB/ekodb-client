make test-examples-rust
🧪 [36mRunning Rust examples (direct HTTP/WebSocket)...[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/simple_crud`
✓ Authentication successful

=== Insert Document ===
Inserted: Object {"id": String("D4OnUlKM-TIOlkvDIJIbMJJHBwwzSVVUE4LEohbG9WqNnr_RpRbb6m3s_mV2BvGmCw4oQRFKOenDejzPpTNFxg")}

=== Find by ID ===
Found: Object {"active": Bool(true), "id": String("D4OnUlKM-TIOlkvDIJIbMJJHBwwzSVVUE4LEohbG9WqNnr_RpRbb6m3s_mV2BvGmCw4oQRFKOenDejzPpTNFxg"), "name": String("Test Record"), "value": Number(42)}

=== Find with Query ===
Found documents: Array [Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("D4OnUlKM-TIOlkvDIJIbMJJHBwwzSVVUE4LEohbG9WqNnr_RpRbb6m3s_mV2BvGmCw4oQRFKOenDejzPpTNFxg"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}]

=== Update Document ===
Updated: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("D4OnUlKM-TIOlkvDIJIbMJJHBwwzSVVUE4LEohbG9WqNnr_RpRbb6m3s_mV2BvGmCw4oQRFKOenDejzPpTNFxg"), "name": Object {"type": String("String"), "value": String("Updated Record")}, "value": Object {"type": String("Integer"), "value": Number(100)}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/simple_websocket`
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: "BRWJqswv00Jc4ldtHNBYkzPv1XIMTVaTFLjDm0FPXH0wMIOwGp6SL1oZIOUqlv30gPRzSi7wdyjSsdR6-gH8Tg"

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
        "id": "BRWJqswv00Jc4ldtHNBYkzPv1XIMTVaTFLjDm0FPXH0wMIOwGp6SL1oZIOUqlv30gPRzSi7wdyjSsdR6-gH8Tg",
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
Collection created with first record: "78Mifvq_jbG6cnu8zhReJdlgD8GBfzNe6-RYymgnZVsJoYPSNP_PbDB_rmowqUUyM90sxHiEvI3Uko9uuOOdBA"

=== List Collections ===
Total collections: 4
Sample collections: ["test_collection", "demo_collection", "websocket_test", "batch_users"]

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
✓ Inserted document: "3vT7tOtJubRqejuSTJIfGyHHDs84Fmh6AdiSh8ceXizD3B_32ODvdPK-Ic2X_V30LU21vmdTrG0uO3LyD1mJCA"

=== Insert Document with TTL (5 minutes - integer) ===
✓ Inserted document: "5BnijQOHv_o_L7ytnfexPQBhOApx06bt_jRZO0nS7oal7Fo93W1D7AcI4YqPlByGAB7bguatMawFilaroO8gTQ"

=== Insert Document with TTL (30 minutes - duration string) ===
✓ Inserted document with duration string TTL: "ZPSZ7NPp60RQpVRCr5zZV4IuKxVfgTI3q1to6NR5sF-6ewqa0c7agxaTBtfgGwmwTudgJUt86ad14G7RdT1Zag"

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
✓ Inserted document with TTL: String("m5JaF6wO49oHy_6hZorPyKcONMPPSOHri8oILjJ8AUpHaE48tWiiPAnFqaPDi0R4KKIRRtPJ8KtPJU5bElojDA")

=== Query via WebSocket ===
✓ WebSocket connected

✓ WebSocket TTL example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/http_functions`
🚀 ekoDB Scripts Example (Rust/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: _LMaxuphtNdTzj72BjIvYH_tnZs8xJt26VDRYuXN4OybWwQuZ-GBy-kkaybuYu2DrphaRzbsNNago0l8bzuOeg
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: "YcRjmzMqxDYrUkenH1VPIUp-UsLaeMVXi0egSKdKYq4VrXX1HcNvVrUxIXuyuicG7lDram7xOwNr0QiOXTbxtg"
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Complex Filter with Multiple Conditions

✅ Script saved: RArNy9KneEnEkMxFl6oSnviQ1HCAXdW6ewXkZOdvx5Nr1YcLjurDX-G_wwhKQcUfZWI2w_-OaeyDZQ6dVuJ1lg
📊 Found 3 users (status=active, score>50, sorted by score)

📝 Example 4: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: HDnQYI2KBdRHDj4HLXMTLY1oHClEqFcil355Vy_iU-lO5m4wwOK9fn8wVjKLunWY6LW8UZvEN2_1HBHFpgul5w
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
Created Alice: $1000 - ID: AzCTAoZEczgR1OFWO0JjiFfDaY_IS11lWI2w9y3JoIFTKl3VfCRhcaMqrtlOdpsQhl0TdiTlQpuntUZpkfUpDw
Created Bob: $500 - ID: TrkCq4BLdblkwVjUzIRTtwDm7PkdcJV6IoeeDCw2T1nz18TpJuXLly2XVSq0GRxWe3uV8fgs-1E6L1L4XJEJKg

=== Example 1: Begin Transaction ===
Transaction ID: 082df8a7-b498-4979-849a-2ee81d77d48a

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
New transaction: a4219d77-f295-44b5-9b11-b33332bfdba1
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
Inserted: Record({"id": String("kZpTHtKKga0tce1vwkTc5fBj4xmhhabyOs65aE63f4VgrJXrVOA8gQOtbjySjjIg1wpVZ9BmNp64zAGtlwQT2Q")})

=== Find by ID ===
Found: Record({"metadata": Object({"key": String("value"), "nested": Object({"deep": Boolean(true)})}), "data": String("aGVsbG8gd29ybGQ="), "id": String("kZpTHtKKga0tce1vwkTc5fBj4xmhhabyOs65aE63f4VgrJXrVOA8gQOtbjySjjIg1wpVZ9BmNp64zAGtlwQT2Q"), "categories": Array([String("electronics"), String("computers")]), "price": Float(99.99), "name": String("Test Record"), "user_id": String("550e8400-e29b-41d4-a716-446655440000"), "active": Boolean(true), "embedding": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "created_at": String("2026-01-26T23:23:03.437420+00:00"), "tags": Array([String("tag1"), String("tag2"), String("tag3")]), "value": Integer(42)})

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Some("Test Record")
  value (Integer): Some(42)
  active (Boolean): Some(true)
  price (Decimal): Some(99.99)
  created_at (DateTime): Some("2026-01-26T23:23:03.437420+00:00")
  user_id (UUID): Some("550e8400-e29b-41d4-a716-446655440000")
  tags (Array): 3 items
  metadata (Object): 2 keys
  embedding (Vector): 0 dims
  categories (Set): 0 items
  data (Bytes): 11 bytes

=== Find with Query ===
Found documents: [Record({"tags": Object({"type": String("Array"), "value": Array([String("tag1"), String("tag2"), String("tag3")])}), "value": Object({"type": String("Integer"), "value": Integer(42)}), "categories": Object({"value": Array([String("electronics"), String("computers")]), "type": String("Array")}), "data": Object({"type": String("String"), "value": String("aGVsbG8gd29ybGQ=")}), "metadata": Object({"value": Object({"key": String("value"), "nested": Object({"deep": Boolean(true)})}), "type": String("Object")}), "price": Object({"value": Float(99.99), "type": String("Float")}), "name": Object({"type": String("String"), "value": String("Test Record")}), "embedding": Object({"type": String("Array"), "value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)])}), "created_at": Object({"type": String("DateTime"), "value": String("2026-01-26T23:23:03.437420Z")}), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "user_id": Object({"value": String("550e8400-e29b-41d4-a716-446655440000"), "type": String("String")}), "id": String("kZpTHtKKga0tce1vwkTc5fBj4xmhhabyOs65aE63f4VgrJXrVOA8gQOtbjySjjIg1wpVZ9BmNp64zAGtlwQT2Q")})]

=== Update Document ===
Updated: Record({"name": Object({"type": String("String"), "value": String("Updated Record")}), "active": Object({"type": String("Boolean"), "value": Boolean(true)}), "categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])}), "id": String("kZpTHtKKga0tce1vwkTc5fBj4xmhhabyOs65aE63f4VgrJXrVOA8gQOtbjySjjIg1wpVZ9BmNp64zAGtlwQT2Q"), "user_id": Object({"value": String("550e8400-e29b-41d4-a716-446655440000"), "type": String("String")}), "price": Object({"type": String("Float"), "value": Float(99.99)}), "created_at": Object({"type": String("DateTime"), "value": String("2026-01-26T23:23:03.437420Z")}), "tags": Object({"type": String("Array"), "value": Array([String("tag1"), String("tag2"), String("tag3")])}), "value": Object({"value": Integer(100), "type": String("Integer")}), "data": Object({"type": String("String"), "value": String("aGVsbG8gd29ybGQ=")}), "embedding": Object({"type": String("Array"), "value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)])}), "metadata": Object({"type": String("Object"), "value": Object({"key": String("value"), "nested": Object({"deep": Boolean(true)})})})})

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_simple_websocket`
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: 8EmxfXrJ8K8xrUWS5Eavp4GsWnCGmPvqiRiRiYOq19IwIggNoUQF_KBbvLfs2je4odSjqanhL0o0qXWIkleYRg

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket

Record 1:
  name: "WebSocket Test Record"
  value: 42
  active: true
  id: "8EmxfXrJ8K8xrUWS5Eavp4GsWnCGmPvqiRiRiYOq19IwIggNoUQF_KBbvLfs2je4odSjqanhL0o0qXWIkleYRg"

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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_collection_management`
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: "xe8DZzCTnIJZCVP8eQ3d6whkCMWuzV9r2MPwpxxeZFxFIOtFFD1OUSS4OY-eFTRAYfU4XHZj8MkitQYUi4w4PQ"

=== List Collections ===
Total collections: 9
Sample collections: ["ws_ttl_test", "test_collection", "scripts__ek0_testing", "websocket_test", "test_accounts"]

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
  cache:product:1: Record({"price": Float(29.99), "name": String("Product 1")})
  cache:product:2: Record({"name": String("Product 2"), "price": Float(39.989999999999995)})
  cache:product:3: Record({"name": String("Product 3"), "price": Float(49.989999999999995)})

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
Created Alice: $1000 - ID: M9oko-jKQTQqiXzfHvTCBugbOxwjWY_OA0PlZSCT5hn_uH_D3b1WHErXSRX-PA5eU1BhTF2kkIGV6JAoN2mxoQ
Created Bob: $500 - ID: 3qlkmzrHgDh3y_GSQSRygB7opsg9gGz1DAnBehDDQwhbBzerBpVQXflx_mSlkjScwmaYtpoXZs1QLj40bHs40Q

=== Example 1: Begin Transaction ===
Transaction ID: 4ba3b1f0-43cc-4e76-b1d9-770a338ba3ec

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: "Active"
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 50534e89-2877-44b0-9141-1d206fc27703
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
  - Some(Object({"type": String("String"), "value": String("Bob")}))
  - Some(Object({"type": String("String"), "value": String("David")}))

=== Range Query (age >= 28 AND age < 35) ===
✓ Found 3 users in age range
  - Some(Object({"type": String("String"), "value": String("Bob")}))
  - Some(Object({"type": String("String"), "value": String("David")}))
  - Some(Object({"type": String("String"), "value": String("Eve")}))

=== IN Operator ===
✓ Found 4 users with status active or pending
  - Some(Object({"value": String("Alice"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("Bob")}))
  - Some(Object({"type": String("String"), "value": String("David")}))
  - Some(Object({"value": String("Eve"), "type": String("String")}))

=== NOT IN Operator ===
✓ Found 4 users not inactive

=== String Pattern Matching ===
✓ Found 5 users with @example.com email

=== Regex Query ===
✓ Found 0 users with names starting with A-C

=== Complex Query (active AND age >= 28 AND score > 1500) ===
✓ Found 2 users matching all conditions
  - Some(Object({"type": String("String"), "value": String("Bob")}))
  - Some(Object({"value": String("David"), "type": String("String")}))

=== OR Query ===
✓ Found 2 users with age < 28 OR age > 32
  - Some(Object({"value": String("Alice"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("Charlie")}))

=== Sorted Query (by score descending) ===
✓ Top 3 users by score:
  1. Some(Object({"type": String("String"), "value": String("Bob")}))
  2. Some(Object({"type": String("String"), "value": String("David")}))
  3. Some(Object({"value": String("Alice"), "type": String("String")}))

=== Pagination (page 2, size 2) ===
✓ Page 2 results:
  - Some(Object({"value": String("Charlie"), "type": String("String")}))
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
     Matched: ["description.value", "title.value", "description", "title"]
  2. Score: 2.0000
     Title: Some(String("Python for Data Science"))
     Matched: ["description.value", "description"]
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
  2. Score: 6.6000 - Some(String("JavaScript Web Development"))

=== Cleanup ===
✓ Deleted collection

✓ All search operations completed successfully
Execution time: 1ms
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_schema`
=== ekoDB Schema Management Example ===

=== Creating Collection with Schema ===
✓ Created collection 'schema_client_rust' with schema

=== Inserting Valid Documents ===
✓ Inserted user 1: Some(String("ySuW0zGm60At-TBOQD5QStt7xcRCHdpf_kAdFNr8ZrJWsTgfX2gHLLExf3DNbhyYTA7cGgYoyvKD5VNE_b8S6w"))
✓ Inserted user 2: Some(String("A8E1MtS6PQeKHoFSTVM1aqIjnWuYJhrx8qrO6G0MPdUw1fhR-CvQP3JzmGBHSOnS8nBBEmOiaYC-qB92s6KciQ"))

=== Getting Schema ===
✓ Schema for schema_client_rust:
  - age: Integer
  - title: String
    (required)
  - status: String
  - email: String
    (required)

=== Listing Collections ===
✓ Total collections: 9
  Sample: ["ws_ttl_test", "schema_client_rust", "test_collection", "scripts__ek0_testing", "websocket_test"]

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
     Running `target/debug/examples/client_document_ttl`
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: "J0bWCQbiSNzRUufJiAy6I24efKhYB2ryyaL-u-LJRmEzh4Fjj4LVqbZyMeUckUm_fGWoDA56xI7PoYHaUl6KiQ"

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: Some(String("b317tV1muZhgq4q3WRsXMIZjzq9XA7tVaNsUQPRHkZLohsDfr4gPPZ3OCnSCy5vYS7Z29aw4J4uAXS7wDc5x1A"))

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
     Running `target/debug/examples/client_websocket_ttl`
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: Some(String("CxddAranWDxF-UeEbn_Eee5NHjvmhqDSzFV0kKUPRyodJIJ1rwH1rR4_YORL7Pht_OazBJhDf-dH8RkIe2cfjA"))

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
✓ Edge cache script created: 5vXbjBTmogbDa-LpQIH-7hRogNsahwKw1GEyr_1SBmWdk7iM3iPScylVwi9rF6FM1_ThZY67ecoSNOzm0yXXbw

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

✅ Script saved: 7l2nXAf5VZxf0hDASNLPBK_sWZTP0zHj-QPIQ9dXYhg2-9eRLNe5xShrgEs38fp-rugMgu22W-IaOQrvOO8JSw
📊 Found 20 active users

📝 Example 2: Parameterized Script

✅ Script saved: GjZx3cX9jJrPf_dyG119mRRWDomVdcAk8V-S8d7-NBy-ZJ2Eq3uSn_OnUBnZVg-mNQZyxR_7sr-zkvuLnlkBrw
📊 Found 20 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: wyabiqqk9ai_pP13BX_KnZVvUHVi1pBOCsbElqWCHDOJNvDXmsOg3Aq8QehAMoQIZkrdYsHD1SnBif-MIUEvjw
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
   ⏱️  Duration: 134.994084ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "name": "Leanne Graham",
    "website": "hildegard.org",
    "address": {
      "geo": {
        "lat": "-37.3159",
        "lng": "81.1496"
      },
      "city": "Gwenborough",
  ...

Second call (cache hit - from cache):
   ⏱️  Duration: 2.098625ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "username": "Bret",
    "phone": "1-770-736-8031 x56442",
    "website": "hildegard.org",
    "address": {
      "zipcode": "92998-3874",
      "geo": {
        "lng": "81.1496",
  ...
   🚀 Cache speedup: 67.0x faster!

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

✅ Script saved: rmkphpEPABIVCnU-bAGIjCJiRUA-r-Jbea5jF5zgBlg5-tUlMVOAgpiSu1wKX7zgIk5Fo3E0ZFYc89m_nWS7Qg
📊 Found 2 product groups
   Record({"avg_price": Float(575.6666666666666), "count": Integer(3), "category": String("Electronics")})
   Record({"category": String("Furniture"), "avg_price": Float(474.0), "count": Integer(2)})
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved
📊 Found 2 categories
   Record({"category": String("Furniture"), "count": Integer(2)})
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

✅ Inserted order: Some(String("Wrd6Lw6aQ4HWxUOzQ8qQl5uG2FnP6vJaJXn-yoQunQVnPPLuXn5VCyjPRCSEo8CsQ-R4IOD_A70lTxjJ43r8xA"))
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: yfy4hneFulcrgW5YeXIgThYGt0P8SikplCdcR9_e60W1VNnBDIODebDfuEPC1JRmmcInzhZj3LHNWlURnxt6OA
📊 Script executed
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: Some(Object {"role": String("admin"), "userId": String("user_abc")})
🔍 Key exists: true
✅ Set cached data
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: FY-z_ZU0BVxyb2Q5N9ddG9eI7rbTb3MTNRAQp3VC3oakn5VzMQmd-SSYMWbbKP0ZGsScmIHI_Vt53VVnrxgZ0g
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Script

✅ Script saved: WzbDinTtAiI0xsbDTtXE47XQ54axk851gYwKf9RTfFlQ6bAs6Qvh5OsNP0nqJJ4cJloSmSHMlhD7Plge10kKzw
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
✓ Created SWR script: fetch_api_user_rs (KSx5U2_XUyH52uuP4tJrNkVYy07MHd6YSg8iR2knFsb-fj42vLc-8junvsHI64Pa1V56OSL4HQClKHd7y9NMgQ)

Step 2: First call - Cache miss, fetches from API
Result: FunctionStats { input_count: 0, output_count: 1, execution_time_ms: 0, stages_executed: 2, stage_stats: [] }
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 66ms (served from cache)
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

✓ Created native SWR script: github_user_native (oDyVky0iMzNV10V233KsIRe0kinIsfjT-rzcTBx5pJ7DFrM3hinRlGkCk1iILNRZ7uRbuMj_aaOktTREKxJaBg)

First call (cache miss - will fetch from GitHub API):
  Response time: 108ms
  Records returned: 1
  ✓ Data fetched from API and cached with 15m TTL

Second call (cache hit - instant from KV store):
  Response time: 3ms
  Speedup: 36.0x faster
  ✓ Lightning fast cache hit


Example 2: SWR with Audit Trail Collection
─────────────────────────────────────────────────────

✓ Created SWR script with audit trail: product_swr_audit (Ns5g-SYJDCpk2swUV50mIfZLCBjKcGBbAB9rIGu8GUSxKotgzldTAGHaBBhZXISBw4ZLeMIlmsTHinDlqTTSBQ)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
─────────────────────────────────────────────────────

Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (YI80oEbSLsM6RKC-yzjTLrS98jaFnTyHZIBzfRp8rawmqUxtQWgGmg4BoX7k7wxYjO4lgKNH-pJIL9qaUv4cWg)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
─────────────────────────────────────────────────────

✓ Created dynamic TTL script: flexible_cache (LyxQZow0UW0WE8-NCOliRlNJVBdT15Jz0gvBssxYv8j3RlPKCjc5Aut3yyuNLTWKoLqtyZuhuinZkbHhU0z6cg)
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
   Record({"avg_price": Float(367.0), "count": Integer(5), "category": String("Electronics")})
   Record({"count": Integer(3), "category": String("Furniture"), "avg_price": Float(365.6666666666667)})
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
   Vector databases offer several benefits including accurate representation of geographic data, high quality visualization, efficient data manipulation, ability to represent complex data like points, lines, and polygons, and capability to handle topological information.
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
   Record({"status": String("inactive"), "count": Integer(3)})
   Record({"status": String("active"), "count": Integer(7)})
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
   1. Vector Databases Explained (Database)
   2. Database Design Principles (Database)
   3. Natural Language Processing (AI)
   4. Introduction to Machine Learning (AI)
   5. Getting Started with ekoDB (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Script saved
📊 Documents by category:
   Record({"count": Integer(2), "category": String("AI")})
   Record({"category": String("Database"), "count": Integer(3)})
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
✓ Created session: Dg8xiBnwBaVnvN7d4DIqKabLRHXxd9EOqXcGWdc16Tpz1RhUKGt1zSSxCy1IsQFvAn6SFcZ1iR5Ci5LGyCQvyQ

=== Sending Chat Message ===
Message ID: 0PD5Qc55ixS_2KNRjtkPY-ZKwOmFEvBNVjOYUa3mn96hDdtv6aM7j6s6AwhG7R76YLLw9wcRWp8tk94akn9-bg

=== AI Response ===
Response 1: ekoDB is a high-performance database that offers intelligent caching and real-time capabilities. One of its key features is its AI integration, which enables you to query your database using natural language and receive AI-powered responses with relevant context, thanks to its AI Chat Integration feature.

In addition to this, ekoDB supports a variety of search options. It allows for full-text search, vector search, and hybrid search, all with automatic context retrieval, making it a versatile tool for managing and retrieving data.

=== Context Used (3 snippets) ===

Snippet 1:
  Collection: client_chat_basic_rust
  Score: 0.6333
  Matched Fields: ["title", "category", "content"]
  Record: Object {"category": String("features"), "content": String("The chat feature allows you to query your database using natural language and get AI-powered responses with relevant context."), "id": String("FaO3gB_5kbEQjzzigh-KHDRvVf3Yvqqhia7HTyHgQlj_JqGpHotpYPEoOkWdhgtblR-hyLI_6l3USzjph4uD8Q"), "title": String("AI Chat Integration")}

Snippet 2:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["content", "title"]
  Record: Object {"category": String("documentation"), "content": String("ekoDB is a high-performance database with intelligent caching, real-time capabilities, and AI integration."), "id": String("1dRzvvxk3_1NhcpMnbhb23vdz0VrUwWRZFI6n0VYYxjS6D3HnvWRHk8QZ2MxJwtxjfrweK7sXARLaY5DwAEGAA"), "title": String("Introduction to ekoDB")}

Snippet 3:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["title", "content", "category"]
  Record: Object {"category": String("features"), "content": String("ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval."), "id": String("yw-vQm3LIS09PkEwgfh1jJTdd95fAF7JbQf6qMIFYXI7YeMmUwkDSFkbe-ukxN59u0Z_4QPl_2u_RXcg-2V3ww"), "title": String("Search Features")}

Execution Time: 2718ms

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/client_chat_advanced`
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: eGjUZ7setNFmelCbcQ8E_uj9SXEmG1XA8U3DAtsjdVFE3mGYjRLucH-1e1GOlMlhxFoYqEmnR9C0OlTi1qdnIw

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information provided, we have a high-performance database product available called ekoDB. It is priced at $99.

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
✓ Created second session: Jl4mTFpz00jWsGudEumneFR__XNqOT5dDRYYYMoqAxV6dp57xjbgUb5ihLSePZv2Ja1dspdRuGNIayfv4oEYiw
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
✓ Created session: 5QxYKUFTMmEPqanYhCgr6qW5GS16WR1B8OBf27ZhsDvNl5nyCnRYlQrSxnhpECUbLp6GSLR16YjBt-2IJ9PsGw
=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information provided, the product available is "ekoDB". This is a high-performance database product that comes with AI capabilities. The price of ekoDB is 99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product with AI capabilities, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: GafZhrxHTduozTrGHXVksSwZ1Voragrd08BsMLoS18ostEmn_7qhTltQQWYHdxsoFLadbk6w4foUPsu5LXt-wA
  Parent: 5QxYKUFTMmEPqanYhCgr6qW5GS16WR1B8OBf27ZhsDvNl5nyCnRYlQrSxnhpECUbLp6GSLR16YjBt-2IJ9PsGw

=== Listing Sessions ===
✓ Found 4 sessions
  Session 1: GafZhrxHTduozTrGHXVksSwZ1Voragrd08BsMLoS18ostEmn_7qhTltQQWYHdxsoFLadbk6w4foUPsu5LXt-wA (Untitled)
  Session 2: 5QxYKUFTMmEPqanYhCgr6qW5GS16WR1B8OBf27ZhsDvNl5nyCnRYlQrSxnhpECUbLp6GSLR16YjBt-2IJ9PsGw (Untitled)
  Session 3: Jl4mTFpz00jWsGudEumneFR__XNqOT5dDRYYYMoqAxV6dp57xjbgUb5ihLSePZv2Ja1dspdRuGNIayfv4oEYiw (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: GafZhrxHTduozTrGHXVksSwZ1Voragrd08BsMLoS18ostEmn_7qhTltQQWYHdxsoFLadbk6w4foUPsu5LXt-wA

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_convenience_methods`
=== ekoDB Convenience Methods Example ===

=== Record Builder Pattern ===
✓ Created record with fluent builder: Record({"id": String("OA-8sWpuG4IQIb1RA6-PPB3c9zxv8j4TpJrwPVWxuPnE0UAKS_fOwMd3rCTDHDxoKGJyyUwUAA5J8P2fJN8u-Q")})

=== Upsert Operation ===
✓ First upsert (insert): Record({"age": Object({"type": String("Integer"), "value": Integer(35)}), "name": Object({"value": String("Bob Smith"), "type": String("String")}), "id": String("OA-8sWpuG4IQIb1RA6-PPB3c9zxv8j4TpJrwPVWxuPnE0UAKS_fOwMd3rCTDHDxoKGJyyUwUAA5J8P2fJN8u-Q"), "active": Object({"type": String("Boolean"), "value": Boolean(true)}), "email": Object({"type": String("String"), "value": String("bob@example.com")})})
✓ Second upsert (update): Record({"active": Object({"value": Boolean(true), "type": String("Boolean")}), "name": Object({"type": String("String"), "value": String("Bob Smith")}), "age": Object({"type": String("Integer"), "value": Integer(36)}), "id": String("OA-8sWpuG4IQIb1RA6-PPB3c9zxv8j4TpJrwPVWxuPnE0UAKS_fOwMd3rCTDHDxoKGJyyUwUAA5J8P2fJN8u-Q"), "email": Object({"type": String("String"), "value": String("bob.smith@newdomain.com")})})

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
   Inserted with ripple: Record({"id": String("OguAoVJlrHaGezf0EIKkbnbUMssZCFP204KhYEXrjCTO-NNuCK8IS21rV_6l4BExgWQzYyQao5EiaS61Vf1K_w")})

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: Record({"id": String("h4iPQZm1EJ-L8ixmT1hPR3i7EL-g7yIsdIqSlCN4XE7mF5yaNEAsk6RAOWiJigwG0IBT_i0HpN3VYbd8g4rngw")})

3. Update with bypass_ripple:
   Updated with bypass_ripple: Record({"price": Object({"type": String("Integer"), "value": Integer(150)}), "id": String("OguAoVJlrHaGezf0EIKkbnbUMssZCFP204KhYEXrjCTO-NNuCK8IS21rV_6l4BExgWQzYyQao5EiaS61Vf1K_w"), "name": Object({"type": String("String"), "value": String("Product 1")})})

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: Record({"id": String("RMi3COCHR9ij8G1SnG41S4NTgzIhIrSX7KIZHcdQePcsJLi4_ru35aJ4hCNiTcBAbJiScrv8gui9eDCya__OjQ")})

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
