make test-examples
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
✅ [32mAll Rust integration tests complete![0m
🧪 [36mRunning Python examples (direct HTTP/WebSocket)...[0m
=== Simple CRUD Operations (Direct HTTP) ===

✓ Authentication successful

=== Insert Document ===
Inserted: {'id': 'hHYw6zgmzw0xqo6lFBRfGFDfMrhAUBoTzTQfDzVhmpZcrNOA4LTCYRMaTuGbxR5rXvLztXf0gUQ2ep9q8OaU8Q'}

=== Find by ID ===
Found: {'name': 'Test Record', 'value': 42, 'id': 'hHYw6zgmzw0xqo6lFBRfGFDfMrhAUBoTzTQfDzVhmpZcrNOA4LTCYRMaTuGbxR5rXvLztXf0gUQ2ep9q8OaU8Q', 'active': True}

=== Find with Query ===
Found documents: [{'active': {'value': True, 'type': 'Boolean'}, 'name': {'type': 'String', 'value': 'Test Record'}, 'value': {'type': 'Integer', 'value': 42}, 'id': 'hHYw6zgmzw0xqo6lFBRfGFDfMrhAUBoTzTQfDzVhmpZcrNOA4LTCYRMaTuGbxR5rXvLztXf0gUQ2ep9q8OaU8Q'}]

=== Update Document ===
Updated: {'name': {'type': 'String', 'value': 'Updated Record'}, 'id': 'hHYw6zgmzw0xqo6lFBRfGFDfMrhAUBoTzTQfDzVhmpZcrNOA4LTCYRMaTuGbxR5rXvLztXf0gUQ2ep9q8OaU8Q', 'value': {'type': 'Integer', 'value': 100}, 'active': {'value': True, 'type': 'Boolean'}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: CUYleyzUq5ERksJhs2_ex0xZMacukDb7Fahybmg6_kYNBoaiKUftv-iDrdcvmQeuPACaYe-iqENgAa27grrarg

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
        "id": "CUYleyzUq5ERksJhs2_ex0xZMacukDb7Fahybmg6_kYNBoaiKUftv-iDrdcvmQeuPACaYe-iqENgAa27grrarg",
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
✓ Retrieved 2 record(s) via WebSocket

✓ WebSocket example completed successfully
🚀 ekoDB Scripts Example (Python/HTTP)

✓ Authentication successful

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: W-y0jvUaXDCVZAkwHuZG9Uk9yaU-jnDIaNNN8Y7rPN2v44VpHNdoVH2xUbDHhskFGZdIDKOZYQvAwUN0Bv_jHA
📊 Found 35 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: eFzAj__SnLF-W8fNt0jrDkw0NRNCDh60J3kP8bd4-xUZfaf6xEuT53w5cqyZw04OtpL-fCn0qkXwt0eZb3zDCg
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: 1nNal6n3RxMF9kc9RuXubbUUsVy4VWMIooT-gCALJuyX5lE0CjcV1cREj46RP04ZY1A52ZAsHaPO3VbyCjJkRQ
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {'avg_score': 50.0, 'max_score': 90, 'status': 'inactive', 'count': 15}
   {'status': 'active', 'max_score': 100, 'avg_score': 60.0, 'count': 15}

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
Collection created with first record: m5VHlZw85SQoUH54Kas7GScbGnmxiVieW0Rm6q0T0WlSiDy_JxthT6XYxC5kd9aftnq8frKOevnGNw1DXK4a_Q

=== List Collections ===
Total collections: 12
Sample collections: ['ws_ttl_test', 'test_collection', 'chat_configurations__ek0_testing', 'demo_collection', 'scripts__ek0_testing']

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
Created Alice: $1000 - ID: Fsb0Wuihh43JcEqlNi5ka-p4MBKDVDx5XBN_sLLf7DcVbTQCzwdUpJzLHXBnMHFOvao_l3Stk97C7CtiLQvFUw
Created Bob: $500 - ID: 9SXSYd1Die3zShpNDP7vGsI9zfOuzOvLsknOVSycDabXijvIo1HIbBICtmRhStZwmFlineBc3Yxv1QwCNfy4Vw

=== Example 1: Begin Transaction ===
Transaction ID: 356afa86-4c8c-4684-8e44-02b27528c6bd

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
New transaction: 882317b1-b6ab-44f7-a540-1486a79aa8d2
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: aFtySlNxY7BiNalf_y67v-0olAn92L3zQzQYpQX9SiDEeVgrb_ljq6Ss-EMxgyRFxxmznpV1G9TDsFiOwA7myQ
Created Bob: $500 - ID: -gTvzaQ6NTeXBf7i9WsgvWHmmVL73mgqOMFagcXzJaVgmFCbhNIbjErvZPLdsG0FYb-PmHITTxbK5wUuqXOCCA

=== Example 1: Begin Transaction ===
Transaction ID: e014de72-da36-4b1f-9101-89f0c04df6f0

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
Created Alice: $1000 - ID: zsHKG6278DBM7_sPvXjJhPSQJlkESqSam_reFBM6ilX81ZWqpqGYJWUDrg6QHaS3isrgWt73G27bWNXlXj0yiA
Created Bob: $500 - ID: DOAFCY3DWhL9eot9jRKMOmw0-OjgMRALPJ2RwgYTOYnMXbk8TtUlY5S0LlQ2PwAkuGBvkfJafyDRe8NNA6yZKQ

=== Example 1: Begin Transaction ===
Transaction ID: ae25215b-3eb8-4155-9dcc-e8bf5c60fdf5

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
New transaction: 1bdcce8b-df20-4c8d-98bf-8757d5b259d4
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 3F4HByHUSdPPpTLykRABQWFGg65v-rHf4gmySa6qGf8p8q1Q5WNtV_NExHtFM2cMHXxOQXQLtYyoTCcUS7qCuA
Created Bob: $500 - ID: lCCw03dcMjKPQIYp_BWcO0xaUqp3SzrTQZjD1apSnpLP9XPNLIRQ4Dx0AoXk29jnSxCAstaSqOuy86ViUyUvCg

=== Example 1: Begin Transaction ===
Transaction ID: 12b8235f-ed89-4b8b-84ec-85545737aead

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
Created Alice: $1000 - ID: GRF55rUfrYBPi7QBVguS-CW8OgY70DLOFaH-69ZpmwyuQMRpppW26YchdL-jVyPwxFXv5Omc322b3H0ZQxf6fg
Created Bob: $500 - ID: y1oXflHa0yeoqqekjrE70nqaOpxHlPc_-yvle4c6FAttfGzYQzVzIp5k6oEHT6m7b1b89Yr5P2AfY4OyyvPJog

=== Example 1: Begin Transaction ===
Transaction ID: 8e7232fa-44f5-4112-9518-2db080797a69

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
New transaction: 20acaf75-5ed3-4842-9cee-96ca167d061e
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: Ly5lvU4Xe0oOxoGUoITBinAPGs_xM3p4palZ8t44ARcUK5RRSW5Z8f2HFwkTpsQ_8UcFsqx5C7VC4d1-tT0rIw
Created Bob: $500 - ID: 0ltb3ksCHCR6tZWiON-mPKof91F60nu3mrzaZGON8fvhcPF-tCSWyyhg2Aiu0WrxENUdhiRJQN5yucdqXsxnmg

=== Example 1: Begin Transaction ===
Transaction ID: e5817ded-49f9-4e69-b410-e418a2ab35fe

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
   ✅ Script saved: Xe-SC6WFFsUk13AaywXNQTuKahbc_bk3_Tx1AFIaMZ7ysSVGK7bZMWmFLfgfqkfGdpMpaV6HlNo4r-FPGWM9xA

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: vO-EVJtdSwwg_E4N_vDCtad-v3VfJDJs8r3f9r_Q6aHi_YO6OfOxeNz-sJvf0ASJQ9jgz2fXU3drnoh9fnsdjw
   📋 Name: {'value': 'Alice Smith', 'type': 'String'}
   📋 Email: {'type': 'String', 'value': 'alice@example.com'}
   📋 Status: {'value': 'pending', 'type': 'String'}
   📋 Credits: {'value': 0, 'type': 'Integer'}

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: seCSnYgoK5cQ2jwZhVlEphhPZXPqJ3aqq_Zrcn_UzfoYGY0fY84vBlBa4z4wjt1LsPaTijibzTt5nTdMGp4mcw

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
   ✅ Script saved: M1gVGqndOxklWMGVTOdJNn-qHSovyQOpK8HFHyJeKAKao_1Kkoemt0WBOcxIuyRPxOuH4_gWf_W_pUrOomuCKw

2️⃣ Calling Script (Query + Update Credits + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {'type': 'Integer', 'value': 0}
   📋 Status: {'value': 'active', 'type': 'String'}
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}

============================================================
📝 Script 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: lLOlpWPISwBN1b36xvrsEhH7Y3_NhBTe7qNZT2cijZKOuoGanBIFPAxyxRuXx0SkEkYaTY0ZzsvY72FokrJhmA

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: Xe-SC6WFFsUk13AaywXN...
   ✅ Deleted script: seCSnYgoK5cQ2jwZhVlE...
   ✅ Deleted script: M1gVGqndOxklWMGVTOdJ...
   ✅ Deleted script: lLOlpWPISwBN1b36xvrs...
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
  Output: Document ID = TXbr4lqe3sP8bUkIs1pvYkxiSGatDa7JT-g6wD28HSTpPPLmqDBDY8sOX3pmYPdtaPtZGLY1gAWV0COk0ltENw
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: find_by_id(TXbr4lqe3sP8bUkIs1pvYkxiSGatDa7JT-g6wD28HSTpPPLmqDBDY8sOX3pmYPdtaPtZGLY1gAWV0COk0ltENw)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: find_by_id(TXbr4lqe3sP8bUkIs1pvYkxiSGatDa7JT-g6wD28HSTpPPLmqDBDY8sOX3pmYPdtaPtZGLY1gAWV0COk0ltENw)
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
  Output: Document ID = JVF3gYmIZhd5LI2AmtrLn80BkyRTFkJp5f3N-hdJSjWZMakukWG4W-lo8TfJUDleKyTchWdMswlx01NApQsCgA
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: find_by_id(JVF3gYmIZhd5LI2AmtrLn80BkyRTFkJp5f3N-hdJSjWZMakukWG4W-lo8TfJUDleKyTchWdMswlx01NApQsCgA)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: find_by_id(JVF3gYmIZhd5LI2AmtrLn80BkyRTFkJp5f3N-hdJSjWZMakukWG4W-lo8TfJUDleKyTchWdMswlx01NApQsCgA)
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
Inserted: {'id': 'PfTZMeURj3GGEafVnIA_Yaut-h6gFYQvIWOYwAg0CNQECk57mWRAzdhGUfZGoFvl_gDtkrMXmQ-XKj7lDZn0rQ'}

=== Find by ID ===
Found: {'data': 'aGVsbG8gd29ybGQ=', 'metadata': {'key': 'value', 'nested': {'deep': True}}, 'price': 99.99, 'created_at': '2026-01-26T18:24:34.982586', 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5], 'active': True, 'value': 42, 'user_id': '550e8400-e29b-41d4-a716-446655440000', 'categories': ['electronics', 'computers'], 'id': 'PfTZMeURj3GGEafVnIA_Yaut-h6gFYQvIWOYwAg0CNQECk57mWRAzdhGUfZGoFvl_gDtkrMXmQ-XKj7lDZn0rQ', 'tags': ['tag1', 'tag2', 'tag3'], 'name': 'Test Record'}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): True
  price (Decimal): 99.99
  created_at (DateTime): 2026-01-26 18:24:34.982586
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): ['tag1', 'tag2', 'tag3']
  metadata (Object): {'key': 'value', 'nested': {'deep': True}}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): ['electronics', 'computers']
  data (Bytes): 11 bytes
Plain record: {'data': 'aGVsbG8gd29ybGQ=', 'metadata': {'key': 'value', 'nested': {'deep': True}}, 'price': 99.99, 'created_at': '2026-01-26T18:24:34.982586', 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5], 'active': True, 'value': 42, 'user_id': '550e8400-e29b-41d4-a716-446655440000', 'categories': ['electronics', 'computers'], 'id': 'PfTZMeURj3GGEafVnIA_Yaut-h6gFYQvIWOYwAg0CNQECk57mWRAzdhGUfZGoFvl_gDtkrMXmQ-XKj7lDZn0rQ', 'tags': ['tag1', 'tag2', 'tag3'], 'name': 'Test Record'}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'data': {'type': 'String', 'value': 'aGVsbG8gd29ybGQ='}, 'id': 'PfTZMeURj3GGEafVnIA_Yaut-h6gFYQvIWOYwAg0CNQECk57mWRAzdhGUfZGoFvl_gDtkrMXmQ-XKj7lDZn0rQ', 'price': {'value': 99.99, 'type': 'Float'}, 'name': {'type': 'String', 'value': 'Updated Record'}, 'value': {'type': 'Integer', 'value': 100}, 'created_at': {'value': '2026-01-26T18:24:34.982586', 'type': 'String'}, 'categories': {'type': 'Array', 'value': ['electronics', 'computers']}, 'active': {'type': 'Boolean', 'value': True}, 'embedding': {'value': [0.1, 0.2, 0.3, 0.4, 0.5], 'type': 'Array'}, 'tags': {'type': 'Array', 'value': ['tag1', 'tag2', 'tag3']}, 'user_id': {'value': '550e8400-e29b-41d4-a716-446655440000', 'type': 'String'}, 'metadata': {'type': 'Object', 'value': {'key': 'value', 'nested': {'deep': True}}}}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: O7KhC_M3zaVzx4a482CT9TnY4uIFjR9OVyDYcsmwn52LL5cfOveE4F40-tSjdaZrR4sL04RXEg-1Z3jQjda-Lw

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
Collection created with first record: "C6qma96Llk01iZt_PeFbNWG8CJc04GcX1yR80IIqf_zlm4i6yvY6LoA00cxADEXWHCUqUHly_Pvz0UOUg-ZcHw"

=== List Collections ===
Total collections: 11
Sample collections: ['ws_ttl_test', 'test_collection', 'chat_configurations__ek0_testing', 'scripts__ek0_testing', 'client_collection_management_python']

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
  cache:product:1: {'price': 29.99, 'name': 'Product 1'}
  cache:product:2: {'price': 39.99, 'name': 'Product 2'}
  cache:product:3: {'price': 49.99, 'name': 'Product 3'}

=== KV Exists ===
Key exists: True

=== KV Find (Pattern Query) ===
Found 3 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 9

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
Created Alice: $1000 - ID: PcHBNET5A9gWF3_8D0VrkwRCQTFd1ryhB4AgENVTeetg9fa_LiFTAYEKOsDfRf0b8WjPlfz1WYnjOnCg8FkztA
Created Bob: $500 - ID: 4aipvtRQIhGO2XXAPOA9cRePqKHRbqaNpKtYN_LgNRreug1sacpnaXY-MHvQkIi-YX6WCU1KDZbfHLzZlGchgg

=== Example 1: Begin Transaction ===
Transaction ID: 42f57bfc-31d4-47c4-b410-676c5f93556f

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 9f0c86ee-52b6-4b49-b16c-b55a86552e8b
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
  1. Score: 26.400, Matched: bio.value, bio, title.value, title
  2. Score: 26.400, Matched: title, title.value, bio.value, bio
  3. Score: 26.400, Matched: title, title.value, bio, bio.value
  4. Score: 26.400, Matched: bio.value, title.value, bio, title

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title, bio.value, title.value, bio
  2. Score: 39.600, Matched: bio, title.value, bio.value, title
  3. Score: 39.600, Matched: title, bio, bio.value, title.value
  4. Score: 39.600, Matched: bio.value, bio, title, title.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 2 results (matches: run, running, runs)
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio.value, bio

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.766
  2. Score: 0.755
  3. Score: 0.741

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.370, Matched: content.value, content, title.value, title
  2. Score: 1.377, Matched: title.value, title, content, content.value
  3. Score: 0.383, Matched: 

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
✓ Inserted document: alxAbBvnFUE3buj-LlCvSTaYAzL2I4-wqI_noELLJ_VeU_Gze3HRzx6U0cHiGWlvwo2WcEdipWiTmUgpNUNl2Q

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: fcxv0EdRJuWCy_UgwRBi1nSIB-9Ya1LNspnGXYrSgFfK5zufRsaWh23NZWEBGZCECDLh89Ea7yTp7etMqfxUnw

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
✓ Inserted document with TTL: jFc4mThjbuYBRvuS6Wz6ysFQqb5sHYqnV_We-TZ4UX9xAmm38EZvigVA_CrJn1yqg3MPgx-R6NUE3VXiFI8cMA

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
✓ Edge cache script created: OiA_oJZG2HkaZVSymKqcZ2aUup0YzB71KO-d4d8wbvMmmtpkI2z2TiPa7FGjI-Lqnuz65AB0Bs2IQ_rBGg3yRQ

Call 1: Cache miss (fetches from API)
Response time: 2ms
Result: {
  "records": [
    {}
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
Response time: 2ms (1.1x faster!)
Result: {
  "records": [
    {}
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

✅ Script saved: 7OepqkX1Y_BU4AJXjeH7zd-sUqNbdFcPWTGk81vRYIgU3GmToxFNte3Hmm2mv9HGNixYVOMlDid7BzQbWOlItw
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved: JTSJOwGzsouCHEpa9WmJkluGXwXCmkadtOPD9CIIfOazmPM_aarttroNlXkOQcwI5Uta_8n2MmA3pNTeB3wtxg
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: i15V8j0cfTq8FKJJm-jhHGFeC3FajJLHnIcPprfkXKmw-n00ozUIGv80VRQ63P4ey82LNTyvui8xOufI8upiYQ
📊 Statistics: 2 groups
   {'avg_score': 50.0, 'count': 5, 'status': 'inactive'}

   {'avg_score': 60.0, 'count': 5, 'status': 'active'}

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

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 1.6ms
   📊 Records: 1
   📦 Data: {
      "value": {
            "address": {
                  "city": "Gwenborough",
                  "geo": {
                        "lat": "-37.3159",
                        "lng": "81.1496"
    ...

Second call (cache hit - from cache):
   ⏱️  Duration: 1.6ms
   📊 Records: 1
   📦 Data: {
      "value": {
            "address": {
                  "city": "Gwenborough",
                  "geo": {
                        "lat": "-37.3159",
                        "lng": "81.1496"
    ...
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
🚀 ekoDB Python Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved: zP1YlROqOpWHoVt7C9E9fZe6hFDzNrwzybfxXkbD_9xxUY-1VxEFIWD6pjJoVRSfi9R1f1ZFw1QPbTEOFP5xAg
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

✅ Inserted order: Ds3Rf9mQPaqt3wQLqOoJ7Tl0B4S2jshXEnYs2nKtL2GscJIqfKT-ydIqjzpitmFaqyEj3D96qLVSN3963c02_Q
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

✅ Inserted order: jarSxRthkMb94u51pRsJ3TkWrFJmSzbEMcCAcm3pK3f0LuBAa5Xy2T8qtq1mXR5iwINV04A2hr4IjEtwFNVl4w
✅ Cached order status
📊 Quick status lookup: {'value': '{"status":"processing","updated_at":"2026-01-26T23:24:36.846346+00:00"}'}

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
✓ Created SWR script: fetch_api_user_py (LGRC-pwueab0MZK_hTvc4EiXmo6iq7jwdHzxV82kjzYnV21q67rXHjAmB03DmX3j_U1c8shX2CfCxlTlmXyC4Q)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
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
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 3ms (served from cache)
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
   {'avg_price': 365.6666666666667, 'category': 'Furniture', 'count': 3}
   {'avg_price': 367.0, 'category': 'Electronics', 'count': 5}
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
   Vector databases offer several benefits such as high precision in data representation, capability to store complex data structures, easy scalability and manipulation of data, suitability for geographic and spatial data, and efficient storage by eliminating redundancies.
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
   inactive: 3 users
   active: 7 users
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
   1. Natural Language Processing (AI)
   2. Database Design Principles (Database)
   3. Introduction to Machine Learning (AI)
   4. Getting Started with ekoDB (Database)
   5. Vector Databases Explained (Database)
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
✓ Created session: 4-V04W3imEKGCRZjDetm--x5I8nlgyhbglc0O0ne_DpXqOAoTm4N6TjYOBYebjqZSDug4o_S4hpEvTDgsa8NVg

=== Sending Chat Message ===
Message ID: g2unJwOGKEeuYMVCAkMalsGFESGJiDjYHR1DbPShOvZmbZFkMmqpzJ2EXTltiQsykl6SdwV0ghATzvAG5wibzw

=== AI Response ===
We have three products available:

1. ekoDB Cloud - This is a fully managed cloud database service. The price for this product is $499.

2. ekoDB - This is a high-performance database product with AI capabilities. The price for this product is $99.

3. ekoDB Pro - This is an enterprise edition product with advanced features. The price for this product is $299.

Execution Time: 2584ms

=== Token Usage ===
Prompt tokens: 448
Completion tokens: 83
Total tokens: 531

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: aLWyLazou-2Zn1lvSQjQtGqqWPJ2xbxA3JdcFWV9UpfnEAAHKL6WIL6mPRhkBMG46x7twdIGDncgnrDVatFEDg

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information available, we have the ekoDB product, which is a high-performance database product. It is priced at $99.

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['context_snippets', 'created_at', 'token_usage', 'chat_id', 'updated_at', 'content', 'id', 'role'])
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
✓ Created second session: MjMzthfgQkK_RCZ5U_aYC-J-lAwwz_GwyqBHLHRntDFGFW8elnqNQCAvkbPZ32JJA4sn7fvaUDNmK7HDcAzdAw
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
✓ Created session: Vjg7gbLn2PTz6aASV1V2HqucyY9_0-mbJorwf-OYI1bz4n56cRj5AiDkY4LOfEIdWts1LdYVp-ZI7Fs0X7TGCg

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product based on the context provided is "ekoDB". It is a high-performance database product. The price of this product is $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: LVlBmUcvz-vLSHi2HTWLn2VOyjrcqwNKPRAfODUXZ3PjAp5o7D9YKbeukuFYASnM8sZtvF0PIlCXcmrxbx4KtA
  Parent: Vjg7gbLn2PTz6aASV1V2HqucyY9_0-mbJorwf-OYI1bz4n56cRj5AiDkY4LOfEIdWts1LdYVp-ZI7Fs0X7TGCg

=== Listing Sessions ===
✓ Found 7 sessions
  Session 1: LVlBmUcvz-vLSHi2HTWLn2VOyjrcqwNKPRAfODUXZ3PjAp5o7D9YKbeukuFYASnM8sZtvF0PIlCXcmrxbx4KtA (Untitled)
  Session 2: Vjg7gbLn2PTz6aASV1V2HqucyY9_0-mbJorwf-OYI1bz4n56cRj5AiDkY4LOfEIdWts1LdYVp-ZI7Fs0X7TGCg (Untitled)
  Session 3: MjMzthfgQkK_RCZ5U_aYC-J-lAwwz_GwyqBHLHRntDFGFW8elnqNQCAvkbPZ32JJA4sn7fvaUDNmK7HDcAzdAw (Untitled)
  Session 4: 4-V04W3imEKGCRZjDetm--x5I8nlgyhbglc0O0ne_DpXqOAoTm4N6TjYOBYebjqZSDug4o_S4hpEvTDgsa8NVg (Untitled)
  Session 5: 5QxYKUFTMmEPqanYhCgr6qW5GS16WR1B8OBf27ZhsDvNl5nyCnRYlQrSxnhpECUbLp6GSLR16YjBt-2IJ9PsGw (Untitled)
  Session 6: Jl4mTFpz00jWsGudEumneFR__XNqOT5dDRYYYMoqAxV6dp57xjbgUb5ihLSePZv2Ja1dspdRuGNIayfv4oEYiw (Untitled)
  Session 7: Dg8xiBnwBaVnvN7d4DIqKabLRHXxd9EOqXcGWdc16Tpz1RhUKGt1zSSxCy1IsQFvAn6SFcZ1iR5Ci5LGyCQvyQ (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: LVlBmUcvz-vLSHi2HTWLn2VOyjrcqwNKPRAfODUXZ3PjAp5o7D9YKbeukuFYASnM8sZtvF0PIlCXcmrxbx4KtA

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Dict Creation ===
✓ Created record with plain dict: {'id': 'JVvr2kvZfwAS7HoAs_K4iHQkpidI6IE8yBR3UOPtRoWdRs6Mp-KuhZj_t7L3t17R4ad5s6dZqGo1jQ3sP86QrQ'}

=== Upsert Operation ===
✓ Upsert (update existing record): JVvr2kvZfwAS7HoAs_K4iHQkpidI6IE8yBR3UOPtRoWdRs6Mp-KuhZj_t7L3t17R4ad5s6dZqGo1jQ3sP86QrQ
✓ Inserted second record: Gn0nkxvtqKxJEES_oTioDumbuI8-qq2QH9im_fjrum_Rog_F7Rt2PWErCQ3cSIF6uxaKmLQcnlgtbWiuX5r0pw
✓ Upsert (update second record): Gn0nkxvtqKxJEES_oTioDumbuI8-qq2QH9im_fjrum_Rog_F7Rt2PWErCQ3cSIF6uxaKmLQcnlgtbWiuX5r0pw

=== Find One Operation ===
✓ Found user by email: {'active': {'type': 'Boolean', 'value': True}, 'age': {'type': 'Integer', 'value': 29}, 'email': {'type': 'String', 'value': 'alice.j@newdomain.com'}, 'id': 'JVvr2kvZfwAS7HoAs_K4iHQkpidI6IE8yBR3UOPtRoWdRs6Mp-KuhZj_t7L3t17R4ad5s6dZqGo1jQ3sP86QrQ', 'name': {'type': 'String', 'value': 'Alice Johnson'}}
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
   Inserted with ripple: {'id': 'TDAFdiMbsQVKiYRD7a7ycpJqiVsShJo2AqK7cU-fD0e6GydDUnATLKScMjbXNNWPZvSq3bkHe16OMcBhM9OpBw'}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {'id': 'D3w420hJ1SXSjOsMcHOUYBP-cwy3au94uEEvlv9IiL8O7lEjg-uZ8j4U4YyV184A_xwcigFi1rLNInovO3i8VA'}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {'name': {'value': 'Product 1', 'type': 'String'}, 'price': {'value': 150, 'type': 'Integer'}, 'id': 'TDAFdiMbsQVKiYRD7a7ycpJqiVsShJo2AqK7cU-fD0e6GydDUnATLKScMjbXNNWPZvSq3bkHe16OMcBhM9OpBw'}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {'name': {'value': 'Upsert Product', 'type': 'String'}, 'price': {'value': 500, 'type': 'Integer'}, 'id': 'TDAFdiMbsQVKiYRD7a7ycpJqiVsShJo2AqK7cU-fD0e6GydDUnATLKScMjbXNNWPZvSq3bkHe16OMcBhM9OpBw'}

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
Inserted: map[id:gmUuTTcFZNq-5zDauOvuXZuRW6bcLhzyg4OHkYAjmkxJersD6c52UP-z_I6BOive3df7OrJGY_LAwQFFGOQQMQ]

=== Find by ID ===
Found: map[active:true id:gmUuTTcFZNq-5zDauOvuXZuRW6bcLhzyg4OHkYAjmkxJersD6c52UP-z_I6BOive3df7OrJGY_LAwQFFGOQQMQ name:Test Record value:42]

=== Find with Query ===
Found documents: map[]

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] id:gmUuTTcFZNq-5zDauOvuXZuRW6bcLhzyg4OHkYAjmkxJersD6c52UP-z_I6BOive3df7OrJGY_LAwQFFGOQQMQ name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
[32m✓ simple_crud.go completed successfully[0m
[34m
=== Running simple_websocket.go ===[0m
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: HcXTmrgLhE2Sv7O9HhkC6tf7c-SDSMXvqOTzKbm5zA0faWPThx9PS56pVP4iX3g7tWPKZeBg9R5v8p_06zrFIg

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
        "id": "CUYleyzUq5ERksJhs2_ex0xZMacukDb7Fahybmg6_kYNBoaiKUftv-iDrdcvmQeuPACaYe-iqENgAa27grrarg",
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
        "id": "BRWJqswv00Jc4ldtHNBYkzPv1XIMTVaTFLjDm0FPXH0wMIOwGp6SL1oZIOUqlv30gPRzSi7wdyjSsdR6-gH8Tg",
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
        "id": "HcXTmrgLhE2Sv7O9HhkC6tf7c-SDSMXvqOTzKbm5zA0faWPThx9PS56pVP4iX3g7tWPKZeBg9R5v8p_06zrFIg",
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

✅ Script saved: upCvXRCK6gfIJK_zD0Mf9d5crTs1oeldFD2O9Y7FM_kTsi8I5SRgAOGh7YNBq61rll2qEUJLCU6_fjOJHcY0Ow
📊 Found 25 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: hR8woPxLICq0MeAcBYuHhTaTCuCEIkrC0AZNUCsAK9mVapoAyzNIPeryvccXdgsTLhJnGI0ob-6Dszi5qDLOVQ
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: 1TheM3q1iVbbSPB5aJDtETp3Lc_r-bDH-5TXFsiQOJhc6pcgj9dSUWL6ZO_A2oZu0_2hmo8nagIGvnjvH9dm8g
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"avg_score":50,"count":10,"max_score":90,"status":"inactive"}
   {"avg_score":60,"count":10,"max_score":100,"status":"active"}

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
Collection created with first record: NqXppQAkn9qc4axz4G1gv8UImBv92RZr3CgX4Eobs0mhAKr6imM6OyX4zxEQM67Ew5GNo5_XmV5aLHoZWUTUIw

=== List Collections ===
Total collections: 15
Sample collections: [ws_ttl_test test_collection chat_configurations__ek0_testing demo_collection scripts__ek0_testing]

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
Created Alice: $1000 - ID: r-23f9Q1LpWjjVBAAAVej2xwGqk7i2UEc2KNPbazjV5NVae5Upi0Aqbsa_1NxptJEtMWaRsqOGO5xDTsG4Gt7w
Created Bob: $500 - ID: ufMUz98_3oF35DNMN2ju_it_S2fxJnVI2ZjqLAvdOnpGXPJMhYQBfoZ-MxtxnLRREl_oNuBFZTblDg47g3xbqg

=== Example 1: Begin Transaction ===
Transaction ID: 4af6fb76-a5f1-4fa1-8f38-11e75a0a4646

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
New transaction: 828999fa-be34-4994-bbd1-25b0a6647fae
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
   ✅ Script saved: gPkKTto2rnHnCh1rv7JuPSfQb_Rpefj8dOS2AKKe4n8H-AP6JDddsZuM0PiDAmxFjsn3wedTzBxAqY0evgDcfw

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: cDSE4GXpjsRj0ophTNrxhg7zS4rPMEftOegmbTeulQt5AWUVtM4UyCp6vl31nO-B_DIQCxKICAeJg4e5Xbm21Q
   📋 Name: map[type:String value:Alice Smith]
   📋 Email: map[type:String value:alice@example.com]
   📋 Status: map[type:String value:pending]
   📋 Credits: map[type:Integer value:0]

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: wlA9W_M7bwckQv_YgUqdLs9N_Fh5wOp0-OBQZJ1dM2mxMp7uUFajGEHTByQTfLLxG6IvXasZzl8HTDFWnx-qHA

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
   ✅ Script saved: hzc4lZDaS3NxhXivb5uoN8VN7sMEmimNuVRHEKkriOFaSPvunH_zvpCCUwkbqY7w7yPrny_kUxAGbY8uoHtv7g

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
   ✅ Script saved: g9anC-el_dzGwkAJ0ZafdCPIs05kTGu5I5vLEVe_Xrftt-CP2xDcl1mPLbqnJpKjkzXLh2UctxmJo7oYH4j1FQ

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: gPkKTto2rnHnCh1rv7Ju...
   ✅ Deleted script: wlA9W_M7bwckQv_YgUqd...
   ✅ Deleted script: hzc4lZDaS3NxhXivb5uo...
   ✅ Deleted script: g9anC-el_dzGwkAJ0Zaf...
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

2026/01/26 18:25:06 No .env file found, using defaults
✓ Client connected

═══════════════════════════════════════════════════════════
TEST 1: Document TTL Expiration
═══════════════════════════════════════════════════════════

[Step 1] Insert document with 3 second TTL
  Input: {name: 'TTL Test', value: 'should expire'}
  TTL: 3s
  Output: Document ID = VrLsvY9SxtZJl-ecQeEhDj93wXEBsbM1mrkb-cfR3IypyucJr87JnCwyqP0ZbkxOaLOOknL2kJlofwYHY5tNIA
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: FindByID(VrLsvY9SxtZJl-ecQeEhDj93wXEBsbM1mrkb-cfR3IypyucJr87JnCwyqP0ZbkxOaLOOknL2kJlofwYHY5tNIA)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: FindByID(VrLsvY9SxtZJl-ecQeEhDj93wXEBsbM1mrkb-cfR3IypyucJr87JnCwyqP0ZbkxOaLOOknL2kJlofwYHY5tNIA)
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

2026/01/26 18:25:13 No .env file found, using defaults
✓ Client connected

═══════════════════════════════════════════════════════════
TEST: WebSocket TTL Expiration
═══════════════════════════════════════════════════════════

[Step 1] Insert document with 3 second TTL
  Input: {name: 'WS TTL Test', value: 'should expire'}
  TTL: 3s
  Output: Document ID = Xkk68ityd0u2IvlElPGPcHrMET1paIESyMgZtl5sOM-mvQUGrsUlUEJAqbAZb9znMyET-2HfnbRn275sTzG3SQ
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: FindByID(Xkk68ityd0u2IvlElPGPcHrMET1paIESyMgZtl5sOM-mvQUGrsUlUEJAqbAZb9znMyET-2HfnbRn275sTzG3SQ)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: FindByID(Xkk68ityd0u2IvlElPGPcHrMET1paIESyMgZtl5sOM-mvQUGrsUlUEJAqbAZb9znMyET-2HfnbRn275sTzG3SQ)
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
Inserted: map[id:wqGC9GD0wCT18CYVpXuFnKqOVB_J0ezIahWLEQs90Rgj7--1450XudXBLxu1z3xxF3301Fr4zuUyg3X2jbGviA]

=== Find by ID ===
Found: map[active:true categories:[electronics computers] created_at:2026-01-26T18:25:20-05:00 data:aGVsbG8gd29ybGQ= embedding:[0.1 0.2 0.3 0.4 0.5] id:wqGC9GD0wCT18CYVpXuFnKqOVB_J0ezIahWLEQs90Rgj7--1450XudXBLxu1z3xxF3301Fr4zuUyg3X2jbGviA metadata:map[key:value nested:map[deep:true]] name:Test Record price:99.99 tags:[tag1 tag2 tag3] user_id:550e8400-e29b-41d4-a716-446655440000 value:42]

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 0
  active (Boolean): true
  price (Decimal): 99.990000
  created_at (DateTime): 2026-01-26 18:25:20 -0500 EST
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1 tag2 tag3]
  metadata (Object): map[key:value nested:map[deep:true]]
  embedding (Vector): [0.1 0.2 0.3 0.4 0.5]
  categories (Set): [electronics computers]
  data (Bytes): 11 bytes
Plain record: map[active:true categories:[electronics computers] created_at:2026-01-26T18:25:20-05:00 data:aGVsbG8gd29ybGQ= embedding:[0.1 0.2 0.3 0.4 0.5] id:wqGC9GD0wCT18CYVpXuFnKqOVB_J0ezIahWLEQs90Rgj7--1450XudXBLxu1z3xxF3301Fr4zuUyg3X2jbGviA metadata:map[key:value nested:map[deep:true]] name:Test Record price:99.99 tags:[tag1 tag2 tag3] user_id:550e8400-e29b-41d4-a716-446655440000 value:42]

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-01-26T23:25:20Z] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:wqGC9GD0wCT18CYVpXuFnKqOVB_J0ezIahWLEQs90Rgj7--1450XudXBLxu1z3xxF3301Fr4zuUyg3X2jbGviA metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Updated Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: lEVXrTcVw8rIbhXluRAQNN5-fgyoWQmWHX-MJJ4wAVfJrQlWWPLcxcyI_XYE5diHS4FCTDsflWRbDm65CuoTWA

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
Collection created with first record: 77tIxqm6-Guc4QPG8pVYwG4ea87ZXDMTigwKZ2szmHNd_N0kjRx7G_3AGdphWW6iNXnvrq5LTN5xdLfDE3Q5dQ

=== List Collections ===
Total collections: 14
Sample collections: [ws_ttl_test test_collection chat_configurations__ek0_testing scripts__ek0_testing websocket_test]

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
Total keys in store: 10

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
Created Alice: $1000 - ID: Bs6siC6fSpdL0bNOIrNY0HqVzvt-0LOU9cdAMlcJ7tDWV73zskf-KvnhOmoITfdRS8GLryLeP9iNSBsKlvIZDw
Created Bob: $500 - ID: jfNtkceUr4S6e4I-qGodgwTI_y6yLCaD0PwKKGNsy10UuZF-WleLGL9UFMIc66o-gY2ZN_AT5maTYuLYTW-HUQ

=== Example 1: Begin Transaction ===
Transaction ID: c36171d0-6c24-479d-a876-a4bea92f1d78

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: ba342858-51c8-442a-a42a-bf7721389418
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
  1. Score: 0.787
  2. Score: 0.754
  3. Score: 0.751

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.715
  2. Score: 1.500
  3. Score: 0.302

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
  - Alice Johnson: Engineering
  - Bob Smith: Sales

2. Join with filtering:
Found 1 users in Engineering
  - Alice Johnson: Building A

3. Join with user profiles:
Found 2 users with profile data
  - Alice Johnson: Senior Software Engineer
  - Bob Smith: Sales Manager

4. Join orders with user data:
Found 2 completed orders
  - Mouse ($0) by Alice Johnson
  - Laptop ($0) by Alice Johnson

5. Complex join with multiple conditions:
Found 2 users with example.com emails
  - Alice Johnson (alice@example.com): Building A
  - Bob Smith (bob@example.com): Building B

=== Cleanup ===
✅ Deleted test collections

✅ Join operations examples completed!
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: zYu7G9IqDzzolkrYuVHexDdw2kYtBt-h6uFvwmSXjMim8R3uGmDNYfVfcrmqVoahbcHMQnrLwjAnFUrbUcXOBw

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: nUz4j2zlnfBF7bwfCcPJGtqXDOiWXjusfolG4x3vha7BNVHujwgTPlddQqFQrUJVPGQKKzHDHke1XzqKoLVF3Q

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
✓ Inserted document with TTL: 2dGWAd3ThzPlGjixpHs4AGCeiKVod4vxXpvcWNgwk8BlOFPqTNP-acXDTtTvujoqBQ3tMN4No1SiNHyww3YMeA

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
✓ Edge cache script created: 7cBpvn0fBRxS3QGQsxP2_Qn8pM7DBbsKo8AbRsT-nSPgwCH1Yj375lrIyUCdbactYHp8emVKTHJmUX5WaGHyMw

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

✅ Script saved: qnwrz5aUuqmA72i89X37Ney7eWvxQCC5QR3F769mH4PntPUVXYN5X8Q3W9SYVKJt8-TUuElHNqoOAg2vz0eHGw
📊 Found 10 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved
📊 Found 10 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved
📊 Statistics: 2 groups
   map[avg_score:50 count:5 status:inactive]
   map[avg_score:60 count:5 status:active]
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

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 2.194291ms
   📊 Records: 1
   📦 Data: {
        "value": {
          "address": {
            "city": "Gwenborough",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "street":...

Second call (cache hit - from cache):
   ⏱️  Duration: 2.188542ms
   📊 Records: 1
   📦 Data: {
        "value": {
          "address": {
            "city": "Gwenborough",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "street":...
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

✅ Script saved: _rZiVoN83cVgNbdk-FSJf3Ieq35cZUAYRIJ2H-sbwDxZ8q8gCPXpO8xmBpIpmQF55MmC-A38vgrQXQ0rNG0qpQ
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

✅ Inserted order: 9kPXmAVEGMKHFUutKGEAkrrmAcuAi-L_u-tTCi8ilEC2_3s2TvrEWDDslgWLjfjqR1b1giygut60xTqe5h04mw
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: otrkrP0TlIMGBsOuC8zez-y01QMBVVvLhJC8-J9DXzEYkmmzvqMJhdowzwUyOm3cOCi__TIA8Sm3VXrhQFFXqA
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: map[role:admin userId:user_abc]
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session
📝 Example 4: KV Operations in Scripts

✅ Script saved: 9T274v4F6QIvu3nl8KBvR76W2jrXAU3IhsnAPm3aO6wf__DpVrMFqEy-hNk6n3GQpxukkn3nIQOgwSBkUGrfgA
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Script

✅ Script saved: EdlIowSeSMGm_bbWn7RVuVuBck0kOwC-XMejZ8HL5Ufp4AvB602ULijIGtDzDlOBoQwTRHVisXqyJI-tWiYRSw
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
✓ Created SWR script: fetch_api_user_go (eRqn7e-Dt22NyZkezQFchmObzglCLb6a3szGpU64PekvyM9MU7d1Llplk_WMe7op4HxfGpxK1WFPVnYoL-goYw)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "cached_at": "2026-01-26T18:25:23-05:00",
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
      "id": "DBzzIbLeQf2_wYnWqkojDy2TBWpYJv2RjpVNDdPRJWCO3tM4qr3wUdaUfYzOeRmfr08QSddeag0vmjQdwWz-rQ"
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
Response time: 64ms (served from cache)
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
   map[count:7 status:active]
   map[count:3 status:inactive]
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
   1. map[type:String value:Database Design Principles] (map[type:String value:Database])
   2. map[type:String value:Getting Started with ekoDB] (map[type:String value:Database])
   3. map[type:String value:Vector Databases Explained] (map[type:String value:Database])
   4. map[type:String value:Introduction to Machine Learning] (map[type:String value:AI])
   5. map[type:String value:Natural Language Processing] (map[type:String value:AI])
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
✓ Created session: 5SXXYkac4oI8ZD6t96ysIzlR4gmK3V7-R1Iwtf2MJGfi98pnTNB6zu2F6x0ZDv_w28kkWW9OAnBPVRKv9l6Mqg

=== Sending Chat Message ===
Message ID: 98Ns11LWnW7GwKa1aAXT653XnqkbSwmsC5WKGTzNLMwGG3ABOsgDVBBcrSzgVR3DxAwO7IIcRPis8UrTSWIKCA

=== AI Response ===
There are three products available:

1. ekoDB Cloud: This is a fully managed cloud database service product. The price for this product is $499.

2. ekoDB Pro: This is an enterprise edition product with advanced features. The price for this product is $299.

3. ekoDB: This is a high-performance database product with AI capabilities. The price for this product is $99.

=== Context Used (3 snippets) ===
  Snippet 1: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Fully managed cloud database service product id:D160WSNY3WSozvNUELHpudTFpRheNjY00hGadoxG4kevbmF9Dw9mfbGwAOVqJbqgxiH3E2XhYRMbQ608M5RSlg name:ekoDB Cloud price:499] score:0.1111111111111111]
  Snippet 2: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Enterprise edition product with advanced features id:H7443kVPegg5NPpQ1heaRYaAE2nfzqJgn_Fx6VVQuSgtq_X8CUhK1UYNs1-FZIu71-JLdfpjMorXLry-Wg2IOA name:ekoDB Pro price:299] score:0.1111111111111111]
  Snippet 3: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:A high-performance database product with AI capabilities id:Y6JE-ub8mMZSdw-Np646eNoVGAefVn9999w91jA1RjfHB1g0Uh3jxU9Yu_HZ2IlSKIvvk4jQ86mwW77AdGsyIQ name:ekoDB price:99] score:0.1111111111111111]

Execution Time: 2785ms

=== Token Usage ===
Prompt tokens: 443
Completion tokens: 84
Total tokens: 527

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 5aP5andDapvfEVPaWZevPjxsstyZ2apNpvwl9jtGL1D4IcfYoOsjIbaYY7FTxqfpgEGl2C5Uq6HR9QuB4TEgyA

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the available context, the product available is "ekoDB". This is a high-performance database product. The price for ekoDB is $99.

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
✓ Created second session: LfxcAjSnqvQbHtrXI6GQFVFJ4iItWlvXOyC_AE-bc4lY_Jq9FZVUs9XeBwmZxf6v5e8mRkJiFUlwD8aaU50iqA
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
✓ Created session: M9ADsELP71bK8H9eVTTm70HNQsJjIwaOy8a8JNwQ3F-33q26Vd95gmZZRp1X6oaoJpT1dfguwe9mVlq8wIPF3w

=== Sending Messages ===
✓ Message 1 sent
  Response: Currently, we have a high-performance database product available named "ekoDB". It is priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 5MrhekMHwHd3QTFa6pna3b6p3ehokT05gquzqTCuDs_HxzoLhWY2JmkRKXd_R-IAtHwEZFTUidJihYFK0NbFFA
  Parent: M9ADsELP71bK8H9eVTTm70HNQsJjIwaOy8a8JNwQ3F-33q26Vd95gmZZRp1X6oaoJpT1dfguwe9mVlq8wIPF3w

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: 5MrhekMHwHd3QTFa6pna3b6p3ehokT05gquzqTCuDs_HxzoLhWY2JmkRKXd_R-IAtHwEZFTUidJihYFK0NbFFA (Untitled)
  Session 2: M9ADsELP71bK8H9eVTTm70HNQsJjIwaOy8a8JNwQ3F-33q26Vd95gmZZRp1X6oaoJpT1dfguwe9mVlq8wIPF3w (Untitled)
  Session 3: LfxcAjSnqvQbHtrXI6GQFVFJ4iItWlvXOyC_AE-bc4lY_Jq9FZVUs9XeBwmZxf6v5e8mRkJiFUlwD8aaU50iqA (Untitled)
  Session 4: 5SXXYkac4oI8ZD6t96ysIzlR4gmK3V7-R1Iwtf2MJGfi98pnTNB6zu2F6x0ZDv_w28kkWW9OAnBPVRKv9l6Mqg (Untitled)
  Session 5: Vjg7gbLn2PTz6aASV1V2HqucyY9_0-mbJorwf-OYI1bz4n56cRj5AiDkY4LOfEIdWts1LdYVp-ZI7Fs0X7TGCg (Untitled)
  Session 6: MjMzthfgQkK_RCZ5U_aYC-J-lAwwz_GwyqBHLHRntDFGFW8elnqNQCAvkbPZ32JJA4sn7fvaUDNmK7HDcAzdAw (Untitled)
  Session 7: 4-V04W3imEKGCRZjDetm--x5I8nlgyhbglc0O0ne_DpXqOAoTm4N6TjYOBYebjqZSDug4o_S4hpEvTDgsa8NVg (Untitled)
  Session 8: 5QxYKUFTMmEPqanYhCgr6qW5GS16WR1B8OBf27ZhsDvNl5nyCnRYlQrSxnhpECUbLp6GSLR16YjBt-2IJ9PsGw (Untitled)
  Session 9: Jl4mTFpz00jWsGudEumneFR__XNqOT5dDRYYYMoqAxV6dp57xjbgUb5ihLSePZv2Ja1dspdRuGNIayfv4oEYiw (Untitled)
  Session 10: Dg8xiBnwBaVnvN7d4DIqKabLRHXxd9EOqXcGWdc16Tpz1RhUKGt1zSSxCy1IsQFvAn6SFcZ1iR5Ci5LGyCQvyQ (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: 5MrhekMHwHd3QTFa6pna3b6p3ehokT05gquzqTCuDs_HxzoLhWY2JmkRKXd_R-IAtHwEZFTUidJihYFK0NbFFA

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Map Creation ===
✓ Created record with native map: map[id:h_i5BtPjvUBhrMowg29hL6zL3M3cTZVE3T5zcwP8WbpgYSd27slkXWwUBrBvq5KhJognUjp1NnIRa3nVw9vcKw]

=== Upsert Operation ===
✓ First upsert (update): map[active:map[type:Boolean value:true] age:map[type:Integer value:29] email:map[type:String value:alice.j@newdomain.com] id:h_i5BtPjvUBhrMowg29hL6zL3M3cTZVE3T5zcwP8WbpgYSd27slkXWwUBrBvq5KhJognUjp1NnIRa3nVw9vcKw name:map[type:String value:Alice Johnson]]
✓ Second upsert (insert): map[id:sswP2P4WXgTW3hUgnlRigxfEYrJq4-QrwcxD_hsy1UKDxtBup36J_4D5xb1PRRIB7UEN6HmygthEn1yM8M0sIA]

=== Find One Operation ===
✓ Found user by email: map[active:map[type:Boolean value:true] age:map[type:Integer value:29] email:map[type:String value:alice.j@newdomain.com] id:h_i5BtPjvUBhrMowg29hL6zL3M3cTZVE3T5zcwP8WbpgYSd27slkXWwUBrBvq5KhJognUjp1NnIRa3nVw9vcKw name:map[type:String value:Alice Johnson]]
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
Inserted with ripple: map[id:BOZJDGtOZQxVO9L5-4TeGTyM-_LrSBLAdAR97bQ7T4J7z8zgR16vkzVA0Tuz86sV6VPIZNX6OIOWgkUAKtCtWw]
Inserted with bypass_ripple: map[id:MsXgxQkMDcYaIgHf6iamYt9cjGfRJc1Xk28Po0_PTPyyGZYyA_6_USqrdn_5c327vXguYHiFHCurgP2SdzypoA]
Inserted with TTL and bypass_ripple: map[id:weluE2UAO0zkjH9Ancpnn83AVYAIYQVRYqPs8lcfrADJltayLZjZbDgmkYRbcnlc7EiEwItqP7hAss-1kjDhVw]
Updated with bypass_ripple: map[id:BOZJDGtOZQxVO9L5-4TeGTyM-_LrSBLAdAR97bQ7T4J7z8zgR16vkzVA0Tuz86sV6VPIZNX6OIOWgkUAKtCtWw name:map[type:String value:Product 1] price:map[type:Integer value:150]]
Deleted with bypass_ripple
Batch inserted with bypass_ripple: 2 records
Upserted with bypass_ripple: map[id:BZrZVEh1EDV_j6jhbvmNvqQYmGqraJJxqFfubbKUoQwz7PeYb6ym_9eurP7GIL2pvr7fpvhSeRk8RLDIORz82g]
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

added 1 package, removed 1 package, and audited 13 packages in 827ms

3 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: '-cIttMLE919pUeHMl6b9Tc9eEB3K0wA-kB8P0p6yL0DJ7p01T7BaGFewa6tck1ZK8UIke5L1BZEbkS9kozJpzw'
}

=== Find by ID ===
Found: {
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  active: true,
  created_at: '2026-01-26T23:25:39.184Z',
  data: 'aGVsbG8gd29ybGQ=',
  value: 42,
  price: 99.99,
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  id: '-cIttMLE919pUeHMl6b9Tc9eEB3K0wA-kB8P0p6yL0DJ7p01T7BaGFewa6tck1ZK8UIke5L1BZEbkS9kozJpzw',
  categories: [ 'electronics', 'computers' ],
  tags: [ 'tag1', 'tag2', 'tag3' ],
  name: 'Test Record',
  metadata: { key: 'value', nested: { deep: true } }
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-01-26T23:25:39.184Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { key: 'value', nested: { deep: true } }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  active: true,
  created_at: '2026-01-26T23:25:39.184Z',
  data: 'aGVsbG8gd29ybGQ=',
  value: 42,
  price: 99.99,
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  id: '-cIttMLE919pUeHMl6b9Tc9eEB3K0wA-kB8P0p6yL0DJ7p01T7BaGFewa6tck1ZK8UIke5L1BZEbkS9kozJpzw',
  categories: [ 'electronics', 'computers' ],
  tags: [ 'tag1', 'tag2', 'tag3' ],
  name: 'Test Record',
  metadata: { key: 'value', nested: { deep: true } }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  created_at: { type: 'DateTime', value: '2026-01-26T23:25:39.184Z' },
  metadata: { type: 'Object', value: { key: 'value', nested: [Object] } },
  price: { type: 'Float', value: 99.99 },
  id: '-cIttMLE919pUeHMl6b9Tc9eEB3K0wA-kB8P0p6yL0DJ7p01T7BaGFewa6tck1ZK8UIke5L1BZEbkS9kozJpzw',
  active: { value: true, type: 'Boolean' },
  value: { type: 'Integer', value: 100 },
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] },
  user_id: { value: '550e8400-e29b-41d4-a716-446655440000', type: 'String' },
  categories: { type: 'Array', value: [ 'electronics', 'computers' ] },
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  name: { type: 'String', value: 'Updated Record' },
  data: { type: 'String', value: 'aGVsbG8gd29ybGQ=' }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: q_HCiqysoRdGo1MhOSbr6cnEk4siqz_aaiRN8b301SaGgA-VP41dZm5UlCJF2jWD_Jn8ZX2VaS10FGzp4f72JA

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
Collection created with first record: nhWIaN_23P5bdKiQ0j6uLcMSeamn1XWOPgy5VFtmDh41GPF59jP9-QTXSsuKaUeevzKmLTd-goZ6asSJKZIKPQ

=== List Collections ===
Total collections: 20
Sample collections: ws_ttl_test,test_collection,chat_configurations__ek0_testing,schema_documents_client_go,schema_users_client_go

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

=== KV Batch Set ===
✓ Batch set 3 keys
  cache:product:1: success
  cache:product:2: success
  cache:product:3: success

=== KV Batch Get ===
✓ Batch retrieved 3 values
  cache:product:1: { value: { name: 'Product 1', price: 29.99 } }
  cache:product:2: { value: { name: 'Product 2', price: 39.99 } }
  cache:product:3: { value: { name: 'Product 3', price: 49.99 } }

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 3 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 10

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
Created Alice: $1000 - ID: ISIUzgKLXWCWFpWr4n6XS00gAd6iF8a8EqQonZMc0bPnnDO95_XaZ0cnEE2DyanIj9gKGEh-9HfT2IfiJD771g
Created Bob: $500 - ID: VVpXxSC8qOQc24x1NO3A9O03R1tyEDmEv5fqUzZoqGJ5Pf6VuaRP-vrJM3Zk1SsaR9DdESFw6r0PWDgeAG7Wng

=== Example 1: Begin Transaction ===
Transaction ID: c84905e8-455a-4526-bfa0-025ee7d7934e

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 806917a6-2e5a-45ad-b456-9360bc11998d
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
  1. Score: 25.740, Matched: email, name.value, email.value, name
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: bio.value, title, bio, title.value
  2. Score: 26.400, Matched: bio, bio.value, title.value, title
  3. Score: 26.400, Matched: bio.value, title.value, bio, title
  4. Score: 26.400, Matched: bio.value, title.value, bio, title

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title.value, title, bio, bio.value
  2. Score: 39.600, Matched: title.value, bio, title, bio.value
  3. Score: 39.600, Matched: bio, title.value, title, bio.value
  4. Score: 39.600, Matched: bio.value, title.value, title, bio

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio.value, bio

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.768, Matched: 
  2. Score: 0.743, Matched: 
  3. Score: 0.735, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.707, Matched: title.value, content.value, title, content
  2. Score: 1.497, Matched: content.value, title.value, content, title
  3. Score: 0.294, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: skills, bio.value, bio, skills.value

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
✓ Inserted document: 0LkbWcb-HQi2pTzlbMtvMNXNG9ooUi66BGErUkaQfx8SYTZQh3XqDyOL_jN2i15kn0CJsgKmTZ3IXXESqGUdAA

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: nCnMhY6XH-Qtk_3KGnLisjKm_dlMqvur1kMvNTNHaWbCzAuhlm87i7QuKo4pD6pbGNdRhdjRFUyUDVX_lrFIgA

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
✓ Inserted document with TTL: cjTvymrK4Qq-fAoRILgBz2NYAVBhSiQa3EMDFxeRfpo4LBQqq6SpA4IKskO2z8X5AByPED3ND3BjB2FhvWbpVw

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
✓ Edge cache script created: twdmrUEpeQOIDvNCxPz2rO4f9npzU4gdjWNdO_DWG5yUSKHg9NqETBeQKyi93v6HAqF3d5yy7caNgkiYf1myvQ

Call 1: Cache miss (fetches from API)
Response time: 4ms
Result: {
  "records": [
    {}
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
Response time: 3ms (1.3x faster!)
Result: {
  "records": [
    {}
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

✅ Script saved: PYzCVjzBlGkjQ3627b5mgPv0z5genY_tFxCOXStPn-V9sA9XWuDrZW9xF7U0crRjbeLFV-VkNr1RbZM_vX7qsw
📊 Found 15 active users

📝 Example 2: Parameterized Script

✅ Script saved: ZL-CYy66-qb8Een065nqpEVKHv4IGjvi3_OyZCZbEPyCVWUXlGQKH5taoEqvHj5CJG8PAekO6gE7rDhu2j-dEg
📊 Found 15 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: 4dZAaz2ELD8x2ZBP9Qm64XN9MMBeYxC2_CdfrxaYrnntE_8_0jBkntyuOvQ6PE6NQmYp61dwqCvZMWVkFH9bKQ
📊 Statistics: 3 groups
   {"status":"inactive","count":5,"avg_score":50}
   {"status":"null","count":5,"avg_score":60}
   {"avg_score":60,"status":"active","count":5}

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

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 3ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "company": {
      "catchPhrase": "Multi-layered client-server neural-net",
      "bs": "harness real-time e-markets",
      "name": "Romaguera-Crona"
    },
    "id": 1,
    "addre...

Second call (cache hit - from cache):
   ⏱️  Duration: 3ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "company": {
      "catchPhrase": "Multi-layered client-server neural-net",
      "bs": "harness real-time e-markets",
      "name": "Romaguera-Crona"
    },
    "id": 1,
    "addre...
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
🚀 ekoDB TypeScript Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved: HnI5ZPsVIN05jG0ivF4cFl_Q8-6eV7pvP3QCqegGL-R7p1k36cOLLkExKRweZHTnn-fvnWz0RPoAJyoqiwCo5A
📊 Found 2 product groups
   {"avg_price":575.6666666666666,"count":3,"category":"Electronics"}
   {"count":2,"avg_price":474,"category":"Furniture"}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved
📊 Found 2 categories
   {"category":"Furniture","count":2}
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

✅ Inserted order: nUkWwdC9v9GsVOfODm94Uz_zOQtfwvZ-8yhtC5ht-FTK3JDOHe-_2bAoTNH6VbhRH5W-vvkR1dlQI82DX7jybA
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: yhnHtRQiPl1yLI1MJft-UQ0AsUJD2p9bNlcAZORkDhcmmtgQ304HY10Gwl-O5OPilqHpcu3_5Jg2zsUbLcSuag
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"role":"admin","userId":"user_abc"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: QOFfcVYHx-auQ7twZPbQHVVMoM_6ENMnih_Yl_jNzBSchcIW8144YvBS1tsqinmr6QEFcZGrZhKJelsw0keYTg
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Script

✅ Script saved: ufzQaW3-nnJv39hwt2PMES3xvk2CxV57tUtigazLkc6HU4TmPLvz2qfYXUQwfSSbqqTjUCcDPhyzCCK5jhfmzA
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
✓ Created SWR script: fetch_api_user (uraAqvR2f0qd_AfN6i6SRuR2THa0mN2OQhA62ZUkCUJTk9qSKDcwPzpfWr5yrrCPe_TrS3qKqjG3g50W8qEBHg)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {}
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
Response time: 4ms (served from cache)
Result (cached): {
  "records": [
    {}
  ],
  "stats": {
    "input_count": 0,
    "output_count": 1,
    "execution_time_ms": 0,
    "stages_executed": 2,
    "stage_stats": []
  }
}
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_with_reviews (4qKkelZEziZSnQ0181BIxqBkMc6CyIqEDTiT7_hhgVcCtmMiga0SQz-U1xhsZbrMm72VTjKsib91Alo7neMZIg)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {}
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
   {"category":"Electronics","count":5,"avg_price":367}
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
   Vector databases offer benefits such as high scalability, precise querying, and the ability to handle complex data structures. They also provide efficient storage and quick data retrieval due to their mathematical representation of data.
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
   inactive: 3 users
   active: 7 users
⏱️  Execution time: 0ms

📝 Example 3: Average Score by Role

✅ Script saved
📊 Average score by role:
   {"role":"user","avg_score":70,"count":7}
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
   1. Introduction to Machine Learning (AI)
   2. Database Design Principles (Database)
   3. Natural Language Processing (AI)
   4. Vector Databases Explained (Database)
   5. Getting Started with ekoDB (Database)
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
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: iROQ2JQ4r-HnxeynoBlz2KcWUCPcQWRtRj9k3KNE1uyiak-cJw_WhhGT4Q2EYUMf-Obas2TnD7CkUZCZzI8mcA

=== Sending Chat Message ===
Message ID: ATrjeedICxTYq4RvmJja5UXiPc3fEeHlBuBIuGRqW5N3r-Jrgj0rb0GfYSd-LgNq81CUHoLK_nMwemVu88APxA

=== AI Response ===
We have three products available:

1. ekoDB Pro: This is an enterprise edition product with advanced features. It is priced at $299.

2. ekoDB Cloud: This is a fully managed cloud database service product. It is priced at $499.

3. ekoDB: This is a high-performance database product with AI capabilities. It is priced at $99.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    id: '2I1XX-yknuOcFfFOiqCBQMgHOQzn-x3cDRXQv4DOdi5K0e7eOJ-O9rbGcDzca33JVXWncOIGeWY_CO8SrEWGtQ',
    description: 'Enterprise edition product with advanced features',
    name: 'ekoDB Pro',
    price: 299
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    name: 'ekoDB Cloud',
    id: 'gPdkSViRCtVXStTMcku9Oa6VHXGuteTIyW3PFqqmedOzGEvEDfap8YG8VTttusHHdIEIVZZpp0-1P8x9u47TDQ',
    price: 499,
    description: 'Fully managed cloud database service product'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    price: 99,
    id: '85uqKW7S72ONCPslN32W4NOvaqK3TWV4GbTQH_OTm0cWOPJ4f9O1auSMtwlk9SQ6pORffuSKFPSegKYh2kYCrA',
    description: 'A high-performance database product with AI capabilities',
    name: 'ekoDB'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 2268ms

=== Token Usage ===
Prompt tokens: 446
Completion tokens: 78
Total tokens: 524

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: quVDSCQg1oVqo4_MQKXBJo56rwAcNl0MIW3YOZQYr8e8hRaEvfHnoZhZYgCDZXeVe74NkPVsvKbOjndyPe7TCg

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information provided, the product available is a high-performance database product named "ekoDB". It is priced at $99.

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
✓ Created second session: dKl084XOstr2VQgTKEYgeN8z-0xEMvJz0Nu1HCkrpzBbUfp6iTYdtIE79Z7GDnB6i99HlDZ5Qa8zegoGHyhtmQ
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
✓ Created session: O9s_bNhs54ht7KpAhRJeLLCYKrHkl1tfsWMmIMGornTRtPrFV9uIQ35woUTAjqqFuPVgfpb7-pVhHjOXGhw9-A

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information provided, there is a product available called ekoDB. It's a high-performance database product priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: Be4uMcJLefkFIwLaWGtR8ZfAuUOYv3h8C9OcfVnlkPJotPmBZjNzK2lsgTteH32R63C9a3fFJB6Q3-Jt_4f9pQ
  Parent: O9s_bNhs54ht7KpAhRJeLLCYKrHkl1tfsWMmIMGornTRtPrFV9uIQ35woUTAjqqFuPVgfpb7-pVhHjOXGhw9-A

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: Be4uMcJLefkFIwLaWGtR8ZfAuUOYv3h8C9OcfVnlkPJotPmBZjNzK2lsgTteH32R63C9a3fFJB6Q3-Jt_4f9pQ (Untitled)
  Session 2: O9s_bNhs54ht7KpAhRJeLLCYKrHkl1tfsWMmIMGornTRtPrFV9uIQ35woUTAjqqFuPVgfpb7-pVhHjOXGhw9-A (Untitled)
  Session 3: dKl084XOstr2VQgTKEYgeN8z-0xEMvJz0Nu1HCkrpzBbUfp6iTYdtIE79Z7GDnB6i99HlDZ5Qa8zegoGHyhtmQ (Untitled)
  Session 4: iROQ2JQ4r-HnxeynoBlz2KcWUCPcQWRtRj9k3KNE1uyiak-cJw_WhhGT4Q2EYUMf-Obas2TnD7CkUZCZzI8mcA (Untitled)
  Session 5: M9ADsELP71bK8H9eVTTm70HNQsJjIwaOy8a8JNwQ3F-33q26Vd95gmZZRp1X6oaoJpT1dfguwe9mVlq8wIPF3w (Untitled)
  Session 6: LfxcAjSnqvQbHtrXI6GQFVFJ4iItWlvXOyC_AE-bc4lY_Jq9FZVUs9XeBwmZxf6v5e8mRkJiFUlwD8aaU50iqA (Untitled)
  Session 7: 5SXXYkac4oI8ZD6t96ysIzlR4gmK3V7-R1Iwtf2MJGfi98pnTNB6zu2F6x0ZDv_w28kkWW9OAnBPVRKv9l6Mqg (Untitled)
  Session 8: Vjg7gbLn2PTz6aASV1V2HqucyY9_0-mbJorwf-OYI1bz4n56cRj5AiDkY4LOfEIdWts1LdYVp-ZI7Fs0X7TGCg (Untitled)
  Session 9: MjMzthfgQkK_RCZ5U_aYC-J-lAwwz_GwyqBHLHRntDFGFW8elnqNQCAvkbPZ32JJA4sn7fvaUDNmK7HDcAzdAw (Untitled)
  Session 10: 4-V04W3imEKGCRZjDetm--x5I8nlgyhbglc0O0ne_DpXqOAoTm4N6TjYOBYebjqZSDug4o_S4hpEvTDgsa8NVg (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: Be4uMcJLefkFIwLaWGtR8ZfAuUOYv3h8C9OcfVnlkPJotPmBZjNzK2lsgTteH32R63C9a3fFJB6Q3-Jt_4f9pQ

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Object Creation ===
✓ Created record with plain object: {
  id: 'oecDCkJXm1QuPBWO3-CuoJJK_zP9SFYb1HQxUx3dee-HOk5ZhkpbMjEK4o5rCqah6a-LY6Rb0XDWiJHQe05Lfw'
}

=== Upsert Operation ===
✓ First upsert (update): {
  active: { value: true, type: 'Boolean' },
  age: { type: 'Integer', value: 29 },
  email: { value: 'alice.j@newdomain.com', type: 'String' },
  id: 'oecDCkJXm1QuPBWO3-CuoJJK_zP9SFYb1HQxUx3dee-HOk5ZhkpbMjEK4o5rCqah6a-LY6Rb0XDWiJHQe05Lfw',
  name: { type: 'String', value: 'Alice Johnson' }
}
✓ Second upsert (insert): {
  id: 'qkKIrB08GXCQhqV2wealOXpDXfa1AsjnVYoNTBobfo5aRp1mZqXBEEGoFC28Bzi0TyVfoJaVXibMnvSoBP-t6A'
}

=== Find One Operation ===
✓ Found user by email: {
  active: { value: true, type: 'Boolean' },
  email: { type: 'String', value: 'alice.j@newdomain.com' },
  age: { type: 'Integer', value: 29 },
  name: { value: 'Alice Johnson', type: 'String' },
  id: 'oecDCkJXm1QuPBWO3-CuoJJK_zP9SFYb1HQxUx3dee-HOk5ZhkpbMjEK4o5rCqah6a-LY6Rb0XDWiJHQe05Lfw'
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
   Inserted with ripple: {"id":"m96a7ULP7WJPTO2hoK1n_fj40EmhCWtK4SLYwPHnpe2aCoojYcYBKX_9pNlgzUfC1wQS6xYtgJVCOcGoz-nQGw"}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {"id":"yHHYR4PCN7BoMVR-0EeNLj5PejX6B33OYp_ySarYsNKbzvh9L1P0bxkTqLONjnZ6ug4tjAAVhPDOU51TmaZIwA"}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {"id":"m96a7ULP7WJPTO2hoK1n_fj40EmhCWtK4SLYwPHnpe2aCoojYcYBKX_9pNlgzUfC1wQS6xYtgJVCOcGoz-nQGw","name":{"type":"String","value":"Product 1"},"price":{"type":"Integer","value":150}}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {"id":"Gua6eNAR9Q4kcM-kf3H2xxWLfC-IKObW1LCSw73ErQRZdCSiH5ZdUqN0Ud2eADcGvi_5Iqh4zNmulxhcXxUoOQ"}

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

added 1 package, removed 1 package, and audited 9 packages in 838ms

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
  id: 'qjnR_xxepMdsTFbRlVdMm8YrZnGdzeOMBPjW3z0Hl4d6Ovzt4xUhk9vNKVaTveHoiaaOEhY3hNogsrRrgUqJeg'
}

=== Find by ID ===
Found: {
  value: 42,
  name: 'Test Record',
  active: true,
  id: 'qjnR_xxepMdsTFbRlVdMm8YrZnGdzeOMBPjW3z0Hl4d6Ovzt4xUhk9vNKVaTveHoiaaOEhY3hNogsrRrgUqJeg'
}

=== Find with Query ===
Found documents: [
  {
    active: { type: 'Boolean', value: true },
    id: 'qjnR_xxepMdsTFbRlVdMm8YrZnGdzeOMBPjW3z0Hl4d6Ovzt4xUhk9vNKVaTveHoiaaOEhY3hNogsrRrgUqJeg',
    value: { type: 'Integer', value: 42 },
    name: { type: 'String', value: 'Test Record' }
  }
]

=== Update Document ===
Updated: {
  name: { value: 'Updated Record', type: 'String' },
  id: 'qjnR_xxepMdsTFbRlVdMm8YrZnGdzeOMBPjW3z0Hl4d6Ovzt4xUhk9vNKVaTveHoiaaOEhY3hNogsrRrgUqJeg',
  value: { value: 100, type: 'Integer' },
  active: { value: true, type: 'Boolean' }
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
✓ Inserted test record: hu9HjYXafG0IHTEiC5nFjXkKU6EJbkxGA9P9soZ6OmvLZRGPRLL3UUJWjB9CjPHQxVF4V2Zqrh3Lr5WXk0RCWw

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
        "id": "CUYleyzUq5ERksJhs2_ex0xZMacukDb7Fahybmg6_kYNBoaiKUftv-iDrdcvmQeuPACaYe-iqENgAa27grrarg",
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
        "id": "HcXTmrgLhE2Sv7O9HhkC6tf7c-SDSMXvqOTzKbm5zA0faWPThx9PS56pVP4iX3g7tWPKZeBg9R5v8p_06zrFIg",
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
        "id": "BRWJqswv00Jc4ldtHNBYkzPv1XIMTVaTFLjDm0FPXH0wMIOwGp6SL1oZIOUqlv30gPRzSi7wdyjSsdR6-gH8Tg",
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
        "id": "hu9HjYXafG0IHTEiC5nFjXkKU6EJbkxGA9P9soZ6OmvLZRGPRLL3UUJWjB9CjPHQxVF4V2Zqrh3Lr5WXk0RCWw",
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

✅ Script saved: jGRAwuXtIUlhY7SHcB1ADbij4BduFT5et8YYZn9i2spqCVUe8mPepXjvMiZYR9QOswmHad630W9N4ws9xy3Ndw
📊 Found 30 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: Ir3aVup6Pivg25sN4jM1M6ginA3xPiDixrmPMrQ9n5iyRzS9k0-LE2skWYtVqW5yRv9njJUgOJsYh5mAmXB_PQ
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: 3U_dNjTyc8IADkuiGJ4BoyPKqecZJu7ciGbjBmf5pcsQ3VGIvh1DfNquS_Qc382ztyu9MTvoUqZV2viHWPItsg
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"avg_score":60,"max_score":100,"status":"active","count":10}
   {"count":10,"avg_score":50,"max_score":90,"status":"inactive"}

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
cache:product:1: { name: 'Product 1', price: 29.99 }
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
Collection created with first record: 6PSjSH_whnAdXPGx3Qi7rDdcXNQ0zMl0X7ooo95XBL9Y1WvUnwj7wN_lzBv1zZvjrOPgLvTd2yV8XfwwIOaCLQ

=== List Collections ===
Total collections: 23
Sample collections: [
  'ws_ttl_test',
  'schema_users_client_ts',
  'test_collection',
  'chat_configurations__ek0_testing',
  'demo_collection'
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
Created Alice: $1000 - ID: q2k0H0BkJgeXIMgZH4De6plZR3e7GY9VlWc-VV1gOhzM744lC0ZaAjp_Dt5xj9oe-rLKpaTnZ8FSkZx-OTrT4A
Created Bob: $500 - ID: AVawKdzIyaUrhiJzuLYw6T_lFVQwrldOvS9ff-rezTiICFJyfCdr5xPylH1fonc0vAyGZhhmeXBoEBNT1jNxVw

=== Example 1: Begin Transaction ===
Transaction ID: 6a785ebc-a8ed-4d02-8342-312ec77f5a2b

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
New transaction: f2e74213-6798-4651-98c1-4547346e4836
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
   ✅ Script saved: irZr8xjxoQ39pHQcKgDTN2PHdpGD7o1lcCNc0hSz2jd1gpr6DRUxy_GKYP2Z2Q6Nc9c55JjZFMD_H4zGVkmVPg

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
   ✅ Script saved: FoQlVGlM47DFujNejs74Pemkvcagq0htxKJgafNPUFCCCwJqwkjz9Xb8xNX9CTCBO0ktuPDnzZS-QkXNj0AxqA

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
   ✅ Script saved: RUeA6BAzD1Vqr8i8Sf8BV1jteX2j9tPPJHxFCE5rlcDRDNXDixVscI3qyZLgJPkj5glywOvf9h8gQ3QZXbsgNw

2️⃣ Calling Script (Query + Update Credits + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {"value":0,"type":"Integer"}
   📋 Status: {"value":"active","type":"String"}
   📋 Name: {"type":"String","value":"Alice Smith"}

============================================================
📝 Script 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: EzxDWOmxAg8BC0Ri_KT0pZOFzl0ISpjFBOFTlD7cFgNPFAbb3KymMuRaA7VxzpXjbuX6bKG7_9GBj9ymwi9pTA

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: irZr8xjxoQ39pHQcKgDT...
   ✅ Deleted script: FoQlVGlM47DFujNejs74...
   ✅ Deleted script: RUeA6BAzD1Vqr8i8Sf8B...
   ✅ Deleted script: EzxDWOmxAg8BC0Ri_KT0...
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
  Output: Document ID = Vg4leDr-KXT-ItS2ogPytJlUejC4dInidNa1Q2PPUXjOWmmSWj0-p-O72qxHdD14cexAHRirstlSHguUmq5oRA
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: findById(Vg4leDr-KXT-ItS2ogPytJlUejC4dInidNa1Q2PPUXjOWmmSWj0-p-O72qxHdD14cexAHRirstlSHguUmq5oRA)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: findById(Vg4leDr-KXT-ItS2ogPytJlUejC4dInidNa1Q2PPUXjOWmmSWj0-p-O72qxHdD14cexAHRirstlSHguUmq5oRA)
  Output: Error (expected) - Request failed with status 404: {"error":"Record has been deleted"}
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
  Output: Document ID = MBdmMxqOIlaYwwaqW8u78u1Vbq5ieqX3KhktzrtogmIOca929v4kPYIsUPBDRPG3XPvXyKEV898D3SUIG_qQVQ
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: findById(MBdmMxqOIlaYwwaqW8u78u1Vbq5ieqX3KhktzrtogmIOca929v4kPYIsUPBDRPG3XPvXyKEV898D3SUIG_qQVQ)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: findById(MBdmMxqOIlaYwwaqW8u78u1Vbq5ieqX3KhktzrtogmIOca929v4kPYIsUPBDRPG3XPvXyKEV898D3SUIG_qQVQ)
  Output: Error (expected) - Request failed with status 404: {"error":"Record has been deleted"}
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

added 1 package, removed 1 package, and audited 9 packages in 876ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'CRL4vfW4U-mz54PCAZOOM4mZ6loCegMMBxOEIff33leyrNqPxpTmj93mkrN_sE5vZCDPQpoDWStwGS_ZCq6x5w'
}

=== Find by ID ===
Found: {
  name: 'Test Record',
  active: true,
  value: 42,
  id: 'CRL4vfW4U-mz54PCAZOOM4mZ6loCegMMBxOEIff33leyrNqPxpTmj93mkrN_sE5vZCDPQpoDWStwGS_ZCq6x5w'
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  id: 'CRL4vfW4U-mz54PCAZOOM4mZ6loCegMMBxOEIff33leyrNqPxpTmj93mkrN_sE5vZCDPQpoDWStwGS_ZCq6x5w',
  value: { value: 100, type: 'Integer' },
  active: { type: 'Boolean', value: true },
  name: { type: 'String', value: 'Updated Record' }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: iDiCCpyJQyKUjXpLFnsftQPSno3ibLPRdMZ8lyGwWDQfkX1MtlnPuIrSUi-R1w8f4Flw4pNP5CHCyIZXsfTSYg

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
Collection created with first record: 14bnBS87sH1E8huj8Cd1FynWXhTDVQHWMVLU2oWKQyVahjQoh-k1CY3YTwxWPtH_RjxfEEB9VamdtuEM3u5hwg

=== List Collections ===
Total collections: 22
Sample collections: ws_ttl_test,schema_users_client_ts,test_collection,chat_configurations__ek0_testing,schema_documents_client_go

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
cache:product:1: { price: 29.99, name: 'Product 1' }
cache:product:2: { name: 'Product 2', price: 39.989999999999995 }
cache:product:3: { name: 'Product 3', price: 49.989999999999995 }

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 0 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 12

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: false

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: l_W5Sx7BeFnylH58-PxMT4Tc5FTua2oqo2yV-AMmKhF8Kje7gbXhL3RK1wJbtFuHLucxccU6f8TF_ixkhFWkRg
Created Bob: $500 - ID: HW80O3DZ0IMR-_8wdMhgYxEDKNrkCf4gqg49P2yoHrkN4Gnygld82fHPDyVwLmuJp2ObtogJstAW-SoDgViHzQ

=== Example 1: Begin Transaction ===
Transaction ID: 6b9fbca6-3d1e-4576-9692-6daa12a47e9e

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: c9e91d57-da07-4d21-938f-469f3ddf89e3
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
  1. Score: 25.740, Matched: email, name.value, email.value, name
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: bio.value, title.value, title, bio
  2. Score: 26.400, Matched: title.value, title, bio, bio.value
  3. Score: 26.400, Matched: bio.value, title, title.value, bio
  4. Score: 26.400, Matched: title.value, bio, bio.value, title

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title, title.value, bio, bio.value
  2. Score: 39.600, Matched: bio, title.value, title, bio.value
  3. Score: 39.600, Matched: bio.value, title.value, bio, title
  4. Score: 39.600, Matched: bio, bio.value, title, title.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio.value, bio

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio.value, bio

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.754, Matched: 
  2. Score: 0.740, Matched: 
  3. Score: 0.732, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.693, Matched: content, title.value, content.value, title
  2. Score: 1.501, Matched: title.value, content, content.value, title
  3. Score: 0.296, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: skills, skills.value, bio, bio.value

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
✓ Inserted document: 1oZXLXCddhcm4AmBXe9qXTtSJ6zY6FntIrIoioOf_wvOuBp9b1Kyy-fOMzMFp5O2IsJ-Bqp0kCW2lT8X3NMZTQ

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: Nzq19wj-FTdBjuq77qNn8gRfzjLoquR6x3Z_W7jH0ZJpn42075jMjkWguCzy0flU9_Cl-VG1CbkNiyphr0xJsQ

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
✓ Inserted document with TTL: dtnXeA7IGmqnu7lWKft7qW4CRDIxZ-fV5OdQlck81stiVzhPHnDjylku5StdIyC-rRIjxZjrCTTajHumgrfI4A

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
✓ Edge cache script created: mta3NiVlx8UyrH-3WYWeaIZxlaQQYz-LdrRry2vVhqMWyktNSOzxNoCrGHP-oEPgM65Z4zgaT3kGg84wFubE8A

Call 1: Cache miss (fetches from API)
Response time: 5ms
Result: {
  "records": [
    {}
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
Response time: 4ms (1.3x faster!)
Result: {
  "records": [
    {}
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

✅ Script saved: QDSH4--dXM3lDQAJ10MtIzI5Y-NdiaXuPM6uGOGT2XAnT2B513Kyq_8RqSfGKSnCE3UVq9ITsihGM7EEEEKSSw
📊 Found 10 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved
📊 Found 10 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved
📊 Statistics: 2 groups
   {"avg_score":50,"status":"inactive","count":5}
   {"count":5,"avg_score":60,"status":"active"}
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
   Name: {"type":"String","value":"User 1"}
   Department: {"type":"String","value":"engineering"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 4ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "company": {
      "catchPhrase": "Multi-layered client-server neural-net",
      "bs": "harness real-time e-markets",
      "name": "Romaguera-Crona"
    },
    "id": 1,
    "addre...

Second call (cache hit - from cache):
   ⏱️  Duration: 4ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "company": {
      "catchPhrase": "Multi-layered client-server neural-net",
      "bs": "harness real-time e-markets",
      "name": "Romaguera-Crona"
    },
    "id": 1,
    "addre...
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

✅ Script saved: KJJnyYhHMqjWM1qoNKHdR-ejjIa6NDIp_y_lti4GOHgIxZt2AU7HbVtWpCPNDUJlFO-FUhPwgU9uL6VButmPpQ
📊 Found 2 product groups
   {"category":"Furniture","count":2,"avg_price":474}
   {"category":"Electronics","count":3,"avg_price":575.6666666666666}
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

✅ Inserted order: qn2qQaNdTk5J4upKGZhfIWQfxtYB6wbLUPJG15_x61hDKVJPTp9wKF--VJOC6_Ck5FBXpNeHdgQM6HjyJIzU1w
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: LCDi1c_c03on28W2BX_wqcYMTWwncXbtav46BjHe_YNXB_QHeopEyHE0W8MIbOBHJ_RsnqxutTOGRXA4QLz3mQ
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"userId":"user_abc","role":"admin"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: Y5TEsWImhmwUrKcmeZBqHOpkKl7J1cB4wbt40P6ggED98hDqOkHgbyiqwyBS3MGgc29KcIpXCZhjOAFNB1QdEQ
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Script

✅ Script saved: uQgtrbvjpiFjusLkgTutVJdbwfFA_rTmu3G204pxrAf_LWsfoDtzT9-rsDC0ptX7lbeMUjVXWKBWSIrTWJXGwQ
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
   inactive: 3 users
   active: 7 users
⏱️  Execution time: 0ms

📝 Example 3: Average Score by Role

✅ Script saved
📊 Average scores by role:
   admin: 20.0 (3 users)
   user: 70.0 (7 users)
⏱️  Execution time: 0ms

📝 Example 4: Top Users by Score

✅ Script saved
📊 Users (showing first 5 of 5):
   1. User 5 - Score: 100
   2. User 1 - Score: 20
   3. User 4 - Score: 80
   4. User 3 - Score: 60
   5. User 2 - Score: 40
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
   2. Vector Databases Explained (Database)
   3. Introduction to Machine Learning (AI)
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
   2. Vector Databases Explained
   3. Introduction to Machine Learning
   4. Natural Language Processing
   5. Database Design Principles
⏱️  Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Script saved
📊 Projected documents (showing first 3):
   1. Getting Started with ekoDB
   2. Vector Databases Explained
   3. Introduction to Machine Learning
⏱️  Execution time: 0ms

📝 Example 5: All Document Fields

✅ Script saved
📊 All documents (5 total, showing first 2):
   1. Getting Started with ekoDB (Database)
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
   Electronics:
      Products: 6 | Stock: 232 | Avg Rating: ⭐4.52
   Furniture:
      Products: 4 | Stock: 43 | Avg Rating: ⭐4.26

⏱️  Total execution time: 0ms
📈 Pipeline stages:

📝 Example 5: Project Specific Fields

✅ Script saved
📊 Product summaries (10 items, showing first 3):
   1. USB-C Cable - $19 (⭐4.3)
   2. Laptop Pro - $1299 (⭐4.8)
   3. Office Lamp - $79 (⭐4.05)
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
   Vector databases have several advantages:

1. Precise Geometry: They retain the exact shape of spatial features even when zooming in at high levels.

2. Attribute Storage: They allow for the storage of additional information (attributes) about each spatial feature.

3. Topology Preservation: They can preserve spatial relationships ("topology") between features, such as connectivity, adjacency, and containment.

4. Space Efficiency: They are more efficient in storing simple, sparse, or irregularly distributed features.

5. Flexibility: They can represent data at its original resolution and form without generalization.

6. Scalability: They are well-suited for applications requiring detailed analysis and complex queries.
⏱️  Execution time: 0ms

📝 Example 2: Generate Embeddings

✅ Embedding script saved
📊 Generated embeddings for 3 articles
   1. "Advanced Query Patterns" - 1536D vector
   2. "Getting Started with ekoDB" - 1536D vector
   3. "Draft Article" - 1536D vector
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
✓ Created SWR script: fetch_api_user_js (hAlomqWFZa6Os9aiKkoQ8-X4HBJXWYNSEJQC0Z7Jd42dqw1Svy0kYIr1mDYotqqsjAeUf4mNasozlyeoBVsxyA)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {}
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
✓ Created session: KCFF9QV0zVxaipcxXs1ZHcPL6Xnc_F_KtUTC8bCLIBBRtn9PA1qE4syhKqR7ODVR0QFjn3t48gEXipDW5dx4Hg

=== Sending Chat Message ===
Message ID: 1N7qx2z5LjPwo6ER-LYb3IY4i6nuSRDMXa0ToO18Fmv3v1mhkXM59SY_wDeyfYgIC9M1JpX2kuWLMZwAX9mi_Q

=== AI Response ===
We have three products available:

1. The ekoDB Cloud, which is a fully managed cloud database service product. Its price is $499.
2. The ekoDB, which is a high-performance database product with AI capabilities. It is priced at $99.
3. The ekoDB Pro, an enterprise edition product with advanced features. This is available for $299.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_js',
  record: {
    price: 499,
    description: 'Fully managed cloud database service product',
    name: 'ekoDB Cloud',
    id: 'YjbQf5MTI-S4YyRp8qLodiifJC6yyVm8PORzziJy0e7caVykuoW8KMEkbbVZO7asbmNJ8_b06VOTatS_6I7ZlQ'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_js',
  record: {
    name: 'ekoDB',
    id: 'V4y6evCjFeOmM2KqsqCcgciHChAVMJEisaDjEtbwVDtUa2KRka0mPM_l8RIEXJNMqzKtO_MJudz2rsJ5aLhiHw',
    description: 'A high-performance database product with AI capabilities',
    price: 99
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_js',
  record: {
    price: 299,
    description: 'Enterprise edition product with advanced features',
    id: '15LHZWfmqGquJxJP4s3HVi50hfzagWlK5LDSodfhIeB4DGKpRmh4hibjUvfZ749JLf8QTHz6Rf7OvZqvZgvwUg',
    name: 'ekoDB Pro'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 2645ms

=== Token Usage ===
Prompt tokens: 450
Completion tokens: 78
Total tokens: 528

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: gPAwJeLEFAQkAr8shKANBuwFLMe88RQya9q3aj4X_FD9dEqbRLZUz1o9gS5ZKTKs6LaktU7-QOF7_Nv6eHCT2w

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the provided context, there is a high-performance database product available named "ekoDB". It is priced at $99.

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
✓ Created second session: cPM6c5wNcs62Rgg2vMA_nUN0_noyJG6-63pxMGUbyGwfJ88NDi8p46DBMZy7O9F8MJVTj4N-XeEGFI6kK8zx4A
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
✓ Created session: s2-q7yoJd0_NdvAIX7prHdUgAKmtiz5LMNPDOGvkiFuGB-4Ex5a8yTjIcnT_TpOGvd__hKK28zyE_1bgiTIFFw

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information provided, the available product is "ekoDB". It is a high-performance database product priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 3wBQw-BEsLAKsI3HhXu1y2Ky8-9lCPkAC0SVp8AK9feH63ck9JZQpKisN4Sjqg5Tivh4R9c2njH5L-gGG5vkbw
  Parent: s2-q7yoJd0_NdvAIX7prHdUgAKmtiz5LMNPDOGvkiFuGB-4Ex5a8yTjIcnT_TpOGvd__hKK28zyE_1bgiTIFFw

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: 3wBQw-BEsLAKsI3HhXu1y2Ky8-9lCPkAC0SVp8AK9feH63ck9JZQpKisN4Sjqg5Tivh4R9c2njH5L-gGG5vkbw (Untitled)
  Session 2: s2-q7yoJd0_NdvAIX7prHdUgAKmtiz5LMNPDOGvkiFuGB-4Ex5a8yTjIcnT_TpOGvd__hKK28zyE_1bgiTIFFw (Untitled)
  Session 3: cPM6c5wNcs62Rgg2vMA_nUN0_noyJG6-63pxMGUbyGwfJ88NDi8p46DBMZy7O9F8MJVTj4N-XeEGFI6kK8zx4A (Untitled)
  Session 4: KCFF9QV0zVxaipcxXs1ZHcPL6Xnc_F_KtUTC8bCLIBBRtn9PA1qE4syhKqR7ODVR0QFjn3t48gEXipDW5dx4Hg (Untitled)
  Session 5: O9s_bNhs54ht7KpAhRJeLLCYKrHkl1tfsWMmIMGornTRtPrFV9uIQ35woUTAjqqFuPVgfpb7-pVhHjOXGhw9-A (Untitled)
  Session 6: dKl084XOstr2VQgTKEYgeN8z-0xEMvJz0Nu1HCkrpzBbUfp6iTYdtIE79Z7GDnB6i99HlDZ5Qa8zegoGHyhtmQ (Untitled)
  Session 7: iROQ2JQ4r-HnxeynoBlz2KcWUCPcQWRtRj9k3KNE1uyiak-cJw_WhhGT4Q2EYUMf-Obas2TnD7CkUZCZzI8mcA (Untitled)
  Session 8: M9ADsELP71bK8H9eVTTm70HNQsJjIwaOy8a8JNwQ3F-33q26Vd95gmZZRp1X6oaoJpT1dfguwe9mVlq8wIPF3w (Untitled)
  Session 9: LfxcAjSnqvQbHtrXI6GQFVFJ4iItWlvXOyC_AE-bc4lY_Jq9FZVUs9XeBwmZxf6v5e8mRkJiFUlwD8aaU50iqA (Untitled)
  Session 10: 5SXXYkac4oI8ZD6t96ysIzlR4gmK3V7-R1Iwtf2MJGfi98pnTNB6zu2F6x0ZDv_w28kkWW9OAnBPVRKv9l6Mqg (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: 3wBQw-BEsLAKsI3HhXu1y2Ky8-9lCPkAC0SVp8AK9feH63ck9JZQpKisN4Sjqg5Tivh4R9c2njH5L-gGG5vkbw

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
✓ Inserted user: EkoRecord(fields={id=StringValue(value=39W0El71ST_W7AUpGYZTItvH_qx-WMTHGLg-bbtG2eC9xwsh95FJNZ_V15vkqRzYmd7vhZnKKhr_pyD40gtglA)})
  User ID: 39W0El71ST_W7AUpGYZTItvH_qx-WMTHGLg-bbtG2eC9xwsh95FJNZ_V15vkqRzYmd7vhZnKKhr_pyD40gtglA

=== Read ===
✓ Found user by ID: EkoRecord(fields={tags=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)]), categories=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)]), email=StringValue(value=alice@example.com), user_id=StringValue(value=550e8400-e29b-41d4-a716-446655440000), name=StringValue(value=Alice Johnson), price=FloatValue(value=99.99), created_at=StringValue(value=Mon Jan 26 18:27:11 EST 2026), data=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)]), metadata=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})}), embedding=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)]), active=BooleanValue(value=true), id=StringValue(value=39W0El71ST_W7AUpGYZTItvH_qx-WMTHGLg-bbtG2eC9xwsh95FJNZ_V15vkqRzYmd7vhZnKKhr_pyD40gtglA), age=IntegerValue(value=28)})

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Alice Johnson
  email (String): alice@example.com
  age (Integer): 28
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): Mon Jan 26 18:27:11 EST 2026
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1, tag2, tag3]
  metadata (Object): {key=value, nested={deep=true}}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): [electronics, computers]
  data (Bytes): 11 bytes
Record fields: tags, categories, email, user_id, name, price, created_at, data, metadata, embedding, active, id, age

=== Update ===
✓ Updated user: EkoRecord(fields={categories=ObjectValue(value={value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)]), type=StringValue(value=Array)}), created_at=ObjectValue(value={value=StringValue(value=Mon Jan 26 18:27:11 EST 2026), type=StringValue(value=String)}), user_id=ObjectValue(value={value=StringValue(value=550e8400-e29b-41d4-a716-446655440000), type=StringValue(value=String)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), embedding=ObjectValue(value={value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)]), type=StringValue(value=Array)}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), email=ObjectValue(value={value=StringValue(value=alice@example.com), type=StringValue(value=String)}), id=StringValue(value=39W0El71ST_W7AUpGYZTItvH_qx-WMTHGLg-bbtG2eC9xwsh95FJNZ_V15vkqRzYmd7vhZnKKhr_pyD40gtglA), city=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=San Francisco)}), age=ObjectValue(value={value=IntegerValue(value=29), type=StringValue(value=Integer)}), data=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)])}), tags=ObjectValue(value={value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)]), type=StringValue(value=Array)}), metadata=ObjectValue(value={type=StringValue(value=Object), value=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})})}), price=ObjectValue(value={type=StringValue(value=Float), value=FloatValue(value=99.99)})})

=== Query ===
✓ Found 1 users matching query
  - EkoRecord(fields={tags=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)])}), id=StringValue(value=39W0El71ST_W7AUpGYZTItvH_qx-WMTHGLg-bbtG2eC9xwsh95FJNZ_V15vkqRzYmd7vhZnKKhr_pyD40gtglA), city=ObjectValue(value={value=StringValue(value=San Francisco), type=StringValue(value=String)}), price=ObjectValue(value={value=FloatValue(value=99.99), type=StringValue(value=Float)}), age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=29)}), embedding=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)])}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), created_at=ObjectValue(value={value=StringValue(value=Mon Jan 26 18:27:11 EST 2026), type=StringValue(value=String)}), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice@example.com)}), data=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)])}), metadata=ObjectValue(value={value=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})}), type=StringValue(value=Object)}), name=ObjectValue(value={value=StringValue(value=Alice Johnson), type=StringValue(value=String)}), categories=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)])}), user_id=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=550e8400-e29b-41d4-a716-446655440000)})})

=== Delete ===
✓ Deleted user with ID: 39W0El71ST_W7AUpGYZTItvH_qx-WMTHGLg-bbtG2eC9xwsh95FJNZ_V15vkqRzYmd7vhZnKKhr_pyD40gtglA

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
  {"payload":{"data":[{"id":"hqkM2kkqlAK3vGFT5gmuaDaRp9AzrWgoJuP5oGj3723GK6bPkslgyuWZfX1lZKEI21DKRuamFzUSYQf074KJ6w","name":{"type":"String","value":"Test User"},"status":{"type":"String","value":"active"}}]},"type":"Success"}

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
  IDs: 5_8C-hmbFktaW3oxUyyJ-hkqhiNRKJMMkWNZdVA7mQMehpXaBaaykq3KEX97DXhnBBIPb_dotjP-xz8tmDpeig, XyrNTGG7ni4rw-BYawiMVNfuKORXYh-CQ8ui180xQ0cCMXClKCFX8TAj1Hv1ZMYUqfvxpJqP-vJF6K5bgOHexQ, 0JR22Ak2Yb6_fHYaUrjTT5SGbT8J6YgK6UsC9lruJjfnOOchU4Yx2P7SYOiao81fbe97hl2KyRgYO8GXauPZwQ...

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
✓ Found 26 collections
  - ws_ttl_test
  - schema_products_client_js
  - schema_users_client_ts
  - test_collection
  - chat_configurations__ek0_testing

=== Check Collection Existence ===
Collection 'kotlin_collection_example' exists: false

=== Create Collection with Schema ===
✓ Created collection with schema: kotlin_collection_example

=== Get Collection Schema ===
✓ Schema: {"fields":{"name":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null}},"version":1,"created_at":"2026-01-26T23:27:24.688860Z","last_modified":"2026-01-26T23:27:24.688860Z","bypass_ripple":false,"primary_key_alias":"id"}

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
✓ Retrieved value: {"email":"alice@example.com","name":"Alice","role":"admin"}

=== KV Set with TTL ===
✓ Set key with 10s TTL: session:abc123

=== Verify TTL Key ===
✓ Session value: {"created_at":1769470048965,"user_id":"123"}
  (Will expire in 10 seconds)

=== KV Batch Set ===
✓ Batch set 3 keys
  config:db: success
  config:cache: success
  config:api: success

=== KV Batch Get ===
✓ Batch retrieved 3 values
  config:db: {"value":{"host":"localhost","port":5432}}
  config:cache: {"value":{"enabled":true,"ttl":3600}}
  config:api: {"value":{"timeout":30,"retries":3}}

=== KV Exists ===
✓ Key exists: true

=== KV Find (Pattern Query) ===
✓ Found 3 keys matching 'config:.*'

=== KV Query (Alias for Find) ===
✓ Total keys in store: 13

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
Created Alice: $1000 - ID: H3occIgsLglIFOg-4V3YlVP8fyfgI8lef8qiC7Nr-4zB8t2pZ__W7_YnjnWqim4bjQjhJS-i-Bv9eGO5unzBwA
Created Bob: $500 - ID: hpWJ03dCPqUyrKGSvkgnqtHymM2wL5xD8KMp24zZdLTco9dsnByDMW4DTG35JmkBwp65GMR1Yj70Db7Zn1uaBQ

=== Example 1: Begin Transaction ===
Transaction ID: 41011f23-0a87-492d-b354-5d1d57f0af1c

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: d794ec0f-226b-48cd-b386-1988382b16d5
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
  - Score: ObjectValue(value={value=IntegerValue(value=95), type=StringValue(value=Integer)})
  - Score: ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=92)})
  - Score: ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=88)})

=== Query 4: Complex (score > 80 AND age >= 25) ===
✓ Found 4 high-scoring adults

=== Query 5: IN (city IN ['NYC', 'LA']) ===
✓ Found 4 records in NYC or LA

=== Query 6: Pagination (skip 2, limit 2) ===
✓ Page 2 (2 records):
  - ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Charlie)})
  - ObjectValue(value={value=StringValue(value=Diana), type=StringValue(value=String)})

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
  {"results":[{"record":{"id":"lPD0f5AkrHU6RsMFeBvAhPyKWyXI6e4K19CIzMAuyljMDPFJwJY1JH53iFDejefX21_qaQ91iYSRwbYVcMGZ_A","views":615,"tags":"programming,rust,tutorial","description":"Learn Rust programming language with hands-on examples and best practices.","title":"Rust Programming"},"score":19.8,"matched_fields":["tags","title","description"]},{"record":{"title":"Python for Data Science","description":"Master Python for data analysis, machine learning, and visualization.","views":856,"tags":"programming,python,data-science","id":"9YL9f5CzApa-4_ryrWJQF3W8rhJ9ae0n8oYR1u9sEOTinCPVK5op0y_iJfmTGaga0PQ9vv_o-7mxQ0xXzRunew"},"score":6.6000000000000005,"matched_fields":["tags"]},{"record":{"tags":"programming,javascript,web","id":"vCunfWZ7HDdeBuPh_CgvEQfsdtL7XbJiVUOay_UaJFD1m9gioizwkWYcHrPSOBDr36flN5Fxsb9wGA7ayDwxsQ","views":118,"description":"Build modern web applications using JavaScript, React, and Node.js.","title":"JavaScript Web Development"},"score":6.6000000000000005,"matched_fields":["tags"]}],"total":3,"execution_time_ms":1}

=== Search for 'machine learning' ===
✓ Found results for 'machine learning'
  {"results":[{"record":{"id":"gNdQBhHRmQqAxxjth6cJLe0v6pIVMzktCh0qyPmBpkEEJEPnpI9Ta7mtfrCAnHX_foEWtTbeTgWAjzpVGe1q0w","tags":"ai,machine-learning,python","description":"Introduction to machine learning algorithms and neural networks.","title":"Machine Learning Basics","views":864},"score":2.7,"matched_fields":["description","title","tags"]},{"record":{"views":856,"description":"Master Python for data analysis, machine learning, and visualization.","tags":"programming,python,data-science","title":"Python for Data Science","id":"9YL9f5CzApa-4_ryrWJQF3W8rhJ9ae0n8oYR1u9sEOTinCPVK5op0y_iJfmTGaga0PQ9vv_o-7mxQ0xXzRunew"},"score":1.0,"matched_fields":["description"]},{"record":{"title":"Database Design","id":"ZpKB8ybQvuQqciAGwgu9JzvDOZ9bRvvOVI0ufDuPnxOF8x6002_59qxjtaOwLJnv3x4T7uFRtF8f2cGJ01Vq6g","views":252,"description":"Learn database design principles, normalization, and query optimization.","tags":"database,design,sql"},"score":0.5,"matched_fields":["description"]},{"record":{"description":"Learn Rust programming language with hands-on examples and best practices.","tags":"programming,rust,tutorial","title":"Rust Programming","views":615,"id":"lPD0f5AkrHU6RsMFeBvAhPyKWyXI6e4K19CIzMAuyljMDPFJwJY1JH53iFDejefX21_qaQ91iYSRwbYVcMGZ_A"},"score":0.5,"matched_fields":["description"]}],"total":4,"execution_time_ms":1}

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
✓ Inserted user 1: StringValue(value=sQ32_N6qAd1BR9CFEpYAGQ4iMU0QAmOGobX0mnBMeODj8kbbcVg-Y2yHiuOW04cvuBtCUi67KPdiF5AnCA53VQ)
✓ Inserted user 2: StringValue(value=dQPF8N1pNh8OvT2o1V_wK9yPacfc1BIKYf5uF8YUgjp4Jx7lVlUgOCHt4yzx3djwGi-KWrjp8T3cVRgdfKbqCA)

=== Getting Schema ===
✓ Schema for kotlin_schema_example:
  Fields: {"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"status":{"field_type":"String","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"title":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"email":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null}}

=== Listing Collections ===
✓ Total collections: 27
  Sample: [ws_ttl_test, schema_products_client_js, schema_users_client_ts, test_collection, chat_configurations__ek0_testing]

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
  Document ID: J9cxmLvxMdX48fiZ6FoG5SsqJQNkYqtbEMT6A3qGtZd5mOavdcNuSBlKshwj3GCaTPCosCKRTQrj_0t_UT0Hgw

=== Verify Document Exists ===
✓ Document found: session_id, id, created_at, user_id, ttl

=== Insert with Longer TTL ===
✓ Inserted document with 1h TTL
  Document ID: MyVBvh5_FkdQZWgIR52vg3IxGLC0c3oAbENjdlveEZpQzi6fuBdSy0iTtvHcUWhY7YYF5CQx-NYlUQsNXM_j2g

=== TTL Expiration ===
✓ Document will automatically expire after 10 seconds

=== Verify Long TTL Document ===
✓ Long TTL document still exists: cache_key, value, id, ttl

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
✓ Inserted document with TTL: StringValue(value=zQ4IaBfxdTE_BUAB9144fcyrGtQnbo58HxzrRkoPWxiOVbNDv5cI4FmvPClf2aobmP9LyCUcSjIgtkJURJuC8A)

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved data via WebSocket:
  {"payload":{"data":[{"created_at":{"type":"Integer","value":1769470079120},"id":"zQ4IaBfxdTE_BUAB9144fcyrGtQnbo58HxzrRkoPWxiOVbNDv5cI4FmvPClf2aobmP9LyCUcSjIgtkJURJuC8A","name":{"type":"String","value":"WebSocket TTL Test"},"ttl":"2026-01-27T00:27:59.237936Z","value":{"type":"Integer","value":42}}]},"type":"Success"}

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
✓ Edge cache script created: qwREC1xMTfseIHn0s8QInGO45q2lKbV7SLbURCtb8jL1yKC-FAuDWeH10CnB8ZtlCaj5hOE_KTkR1pvBMuXzYw

Call 1: Cache lookup
Found 1 cached entries
Response time: 13ms

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

✅ Script saved: _wfAFZo_PkHz_h9mEmxfF5j1vOCgowpr05lq8fYlaujIGgkFLMgIPAtHUs1u04FyjRx5Hsqciger_eVATZZcOw
📊 Found 15 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved
📊 Found 15 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved
📊 Statistics: 3 groups
   {"status":"null","count":5,"avg_score":60.0}
   {"status":"active","count":5,"avg_score":60.0}
   {"status":"inactive","count":5,"avg_score":50.0}
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

✅ Script saved: EjSC_IiJ1cMklqiv73eFnaklqPj9oVd4ZWt0nJWvcIPjuDm8xi_xn4eV7DCYpKee52xlmgY1mkylr7zCjq7iXw
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

✅ Inserted order: StringValue(value=rZ6BvgrrEE0KJx48on_L7OsRtwssQ9FOINlt3ptYFRLiebOyw7eTAn9ThMsim-LOFCzzOuvJ41Xj258j5D-N4A)
✅ Inserted 2 products

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: VhwZNVwsVY6vpQiFJdX2s53GaG9g4-2nkfB-k8B_0w6o8fdBr6hO1OaVA3Q9CWn8Ng_0i87Ox7sBF6rwUbIKdA
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"userId":"user_abc","role":"admin"}
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: o0Eq9XCy-Cz8287eGt6kpYckgYqiL1YtBvSrGLrRmba_bz7iza3Zw4w9EhYWvqaSb0GD7jTKxxI-ESTbj4FwZQ
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Script

✅ Script saved: UZLQYGA2PZZspU6xVIZlOs9Uzv4mctlY0aHTZf7NHs6sx6SWRVbc9vEEMjX93DiewkaA6eSo-_bddFQVbNzLUw
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
✓ Created SWR script: swr_cache_lookup_kt (Y5Gz2b-jyAjFr5ugWhQ8ol1rATobBmgzJHl_6Oai0e_vLPDgxTtC_E46NwTn5uySnmSTvZSldy37EaLvsgtHPQ)

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
   {"count":5,"category":"Electronics","avg_price":367.0}
   {"count":3,"avg_price":365.6666666666667,"category":"Furniture"}
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
   {"tokens_used":91,"response":"Vector databases offer several benefits including high scalability, reduced storage space, improved querying speed, efficient data manipulation, and the ability to perform complex spatial analyses. They also support accurate representation of geographic features and can store data with high precision."}
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
   {"count":7,"status":"active"}
   {"count":3,"status":"inactive"}
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
   1. {"type":"String","value":"Introduction to Machine Learning"} ({"type":"String","value":"AI"})
   2. {"type":"String","value":"Database Design Principles"} ({"type":"String","value":"Database"})
   3. {"type":"String","value":"Natural Language Processing"} ({"value":"AI","type":"String"})
   4. {"value":"Getting Started with ekoDB","type":"String"} ({"value":"Database","type":"String"})
   5. {"type":"String","value":"Vector Databases Explained"} ({"type":"String","value":"Database"})
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
✓ Created session: OQWRuEnCPjeV5sXmrnP6xYOZp4gvt06zQQ8U2zSzxMmdMeOLHfbAMxYEIx80zKd7PNKjGcjPu-HfQzk96WREjA

=== Sending Chat Message ===
✓ Chat response:
  Message ID: null
  Responses: null

=== Cleanup ===
✓ Deleted chat session
✓ Deleted collection: kotlin_chat_basic_example

✓ Basic chat example completed successfully

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
=== ekoDB Kotlin Client - Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: EQKc_xlKTd8Ahchf9Ti623zYg2ZoXNcxEeRhLL8JHqcQRdTPt6I4ZWPt765GqsnCRPx2a6sbcMudMUK9CKKmfA

=== Sending Initial Message ===
✓ Message sent
  Responses: ["Based on the provided context, the available product is ekoDB. It is a high-performance database product. The price of ekoDB is $99."]

✓ Second message sent

=== Regenerating AI Response ===
✓ AI response regenerated
  New responses: ["Based on the information provided, the available product is ekoDB. This high-performance database product is priced at $99."]

=== Updating Message ===
✓ Updated message content

=== Toggling Forgotten Status ===
✓ Marked message as forgotten (excluded from context)

=== Creating Second Session for Merge ===
✓ Created second session: JgQJOpMKb0s2RQwl2_odz9J7zRj5FHedXnsgZ_ESb_GS0XSIFyCqzGwVjq8SFlBnBVFjxm_NInfq6ieGpeDXNg

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
✓ Created session: W9qHM2uMhDcbTY7oo1jlWgUYmmzVbnfLpNyk8_G2WVNImdfbIAEshWTCrx3Mcok53SkGp9IHZ_hYEP1CwoLg9Q

=== Sending Messages ===
✓ Message 1 sent
  Responses: ["The product available is \"ekoDB\", a high-performance database product with AI capabilities. It is priced at $99."]

✓ Message 2 sent
  Responses: ["The price of the ekoDB database product is $99."]

=== Getting Message History ===
✓ Retrieved message history
  Total messages: [{"chat_id":{"type":"String","value":"W9qHM2uMhDcbTY7oo1jlWgUYmmzVbnfLpNyk8_G2WVNImdfbIAEshWTCrx3Mcok53SkGp9IHZ_hYEP1CwoLg9Q"},"content":{"type":"String","value":"What products are available?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"ouUP9_YgSTG9GKpu0Nhi5KRqsgEAR7Oc0CZIOMCLlOmGAKbzNFB1wevI3cXa7f0FgI1rx0zH421UNSkKeqP0Lg","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2026-01-26T23:28:58.661239Z"},"id":"BrVla_pZooCHBtBU4YxhrAJ0k5RUzOqv1rHjVoOefluuTkg2uOG-CSv35dRU0dmyUC3Aw5BCYm1ugwjmDYiT_w","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":24,"prompt_tokens":174,"total_tokens":198}},"updated_at":{"type":"DateTime","value":"2026-01-26T23:28:58.661239Z"}},{"chat_id":{"type":"String","value":"W9qHM2uMhDcbTY7oo1jlWgUYmmzVbnfLpNyk8_G2WVNImdfbIAEshWTCrx3Mcok53SkGp9IHZ_hYEP1CwoLg9Q"},"content":{"type":"String","value":"The product available is \"ekoDB\", a high-performance database product with AI capabilities. It is priced at $99."},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"ouUP9_YgSTG9GKpu0Nhi5KRqsgEAR7Oc0CZIOMCLlOmGAKbzNFB1wevI3cXa7f0FgI1rx0zH421UNSkKeqP0Lg","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2026-01-26T23:28:58.666798Z"},"id":"A_yENPeypEkvbBuFkVmtIfZ4J_ynwpttzh7tNeu20WCvkrrsG1SqqJRqoP1Kcfu2I-S3_04G2fUwMvhkkO5tOg","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":24,"prompt_tokens":174,"total_tokens":198}},"updated_at":{"type":"DateTime","value":"2026-01-26T23:28:58.666798Z"}},{"chat_id":{"type":"String","value":"W9qHM2uMhDcbTY7oo1jlWgUYmmzVbnfLpNyk8_G2WVNImdfbIAEshWTCrx3Mcok53SkGp9IHZ_hYEP1CwoLg9Q"},"content":{"type":"String","value":"What is the price?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["price"],"record":{"description":"A high-performance database product with AI capabilities","id":"ouUP9_YgSTG9GKpu0Nhi5KRqsgEAR7Oc0CZIOMCLlOmGAKbzNFB1wevI3cXa7f0FgI1rx0zH421UNSkKeqP0Lg","price":99,"product":"ekoDB"},"score":0.8}]},"created_at":{"type":"DateTime","value":"2026-01-26T23:28:59.572622Z"},"id":"NQ-kbe6BhAsZQwtk6JC1P_syoHLpOZumelaKPOkhne1Mi-_v4d0fPqao94c6NPNWCgcVfi4XI4bfrm27Ts9gqw","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":13,"prompt_tokens":174,"total_tokens":187}},"updated_at":{"type":"DateTime","value":"2026-01-26T23:28:59.572622Z"}},{"chat_id":{"type":"String","value":"W9qHM2uMhDcbTY7oo1jlWgUYmmzVbnfLpNyk8_G2WVNImdfbIAEshWTCrx3Mcok53SkGp9IHZ_hYEP1CwoLg9Q"},"content":{"type":"String","value":"The price of the ekoDB database product is $99."},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["price"],"record":{"description":"A high-performance database product with AI capabilities","id":"ouUP9_YgSTG9GKpu0Nhi5KRqsgEAR7Oc0CZIOMCLlOmGAKbzNFB1wevI3cXa7f0FgI1rx0zH421UNSkKeqP0Lg","price":99,"product":"ekoDB"},"score":0.8}]},"created_at":{"type":"DateTime","value":"2026-01-26T23:28:59.577681Z"},"id":"FVEb15c2xs7TFI9tgiBQTEZRWrqRoAX5k9_er3EYvd7HSKR_kx_85Qz6a8n4s-NYSQe7CDcingC-TP7CymxEaQ","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":13,"prompt_tokens":174,"total_tokens":187}},"updated_at":{"type":"DateTime","value":"2026-01-26T23:28:59.577681Z"}}]

=== Updating Session ===
✓ Updated session system prompt

=== Listing Sessions ===
✓ Total sessions: 16

=== Branching Session ===
✓ Created branched session: 7JwI1fhYJ9AxN7tITkL12mPut8FFLC3Kbzf2fay5nMFi5Xr7DQwBY4qOTQatUp0AdHTzLB-E0pLBcpd8pcPeYw

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
✓ Created record: EkoRecord(fields={id=StringValue(value=WipT4nzpMN8LzFD0HZPoCh6f886PVkUCtO9Ycb_xPRJdxTAS-a1Sbd1quikWHsifx02Yk3jh2-FCWRuJXiFLUg)})

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

BUILD SUCCESSFUL in 4s
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
   Inserted with ripple: EkoRecord(fields={id=StringValue(value=9onNj-GyjJEY-TQ-6dnUm8_0F65pGmfwBDhp1Rh8nsd-Ua__i0won_EtI29YQKjavcFAm_uw03JLQTXYaZVAoA)})

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: EkoRecord(fields={id=StringValue(value=vWzI4TpAqm1ZhcMf_-c8cqlBLXdOVYTIbkJQTwEKv4ItD44dWfH_AR96OkI3p1xN29TbWqnzB4ewZX_TwK01Wg)})

3. Update with bypass_ripple:
   Updated with bypass_ripple: EkoRecord(fields={id=StringValue(value=9onNj-GyjJEY-TQ-6dnUm8_0F65pGmfwBDhp1Rh8nsd-Ua__i0won_EtI29YQKjavcFAm_uw03JLQTXYaZVAoA), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Product 1)}), price=ObjectValue(value={value=IntegerValue(value=150), type=StringValue(value=Integer)})})

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: EkoRecord(fields={name=ObjectValue(value={value=StringValue(value=Upsert Product), type=StringValue(value=String)}), id=StringValue(value=9onNj-GyjJEY-TQ-6dnUm8_0F65pGmfwBDhp1Rh8nsd-Ua__i0won_EtI29YQKjavcFAm_uw03JLQTXYaZVAoA), price=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=500)})})

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
Created Alice: $1000 - ID: oI6cRl75DnScCECXj8_UnkZLH0nonL61vy6HrHd2ZPwKAhAmq9_iIwAp97Ku8fcpsqgWQZzOTggodq1AoF7O7g
Created Bob: $500 - ID: yhFpIucymuadChmWndjSbLRXoaVay7GM8hzNAO2I7jJHUN8KSkhBTL4za1XKzkDPkG5VmSU_F3Bi5c0SzQnGDw

=== Example 1: Begin Transaction ===
Transaction ID: c744e4eb-478a-41a6-b0f0-6869521d2a3e

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 235a934c-3206-48e8-8bd1-56a2e6711a0e
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
    ✓ Generated embedding: 1536 dimensions in 450.215958ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 948.2805ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 696.146166ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 333.450541ms
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 331.307542ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 519.356917ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 375.44925ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 794.116125ms
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 514.995208ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 413.055167ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 338.824083ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 399.168167ms
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
    ✓ Generated embedding: 1536 dimensions in 706.373041ms
    • Function auto-cleaned up by client

→ Executing hybrid_search()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 109.269917ms

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

Writing memory-safe and high-performance database code involves several practices from writing efficient queries to ensuring proper resource management.

1. Write Efficient Queries: Whether using SQL or NoSQL, try to write efficient, concise, and clear queries. Unnecessary complexity can slow down execution time.

2. Limit Results: Only request the data you need. Using LIMIT in SQL, or equivalent in NoSQL databases, can reduce the amount of data transferred, thus improving performance.

3. Indexing: Make sure to use indexing on columns that are frequently queried. Indexing can significantly speed up data retrieval times.

4. Use Prepared Statements: Prepared statements not only help in preventing SQL injections but can also improve performance as they are parsed once but can be executed multiple times.

5. Normalization and Denormalization: Based on the context, if you're using SQL, normalization can help to reduce redundancy and improve data integrity. However, there might be cases where denormalization might help improve read performance.

6. Connection Pooling: Establishing and tearing down database connections are expensive processes. Connection pooling allows you to reuse connections, saving on the overhead of establishing new ones.

7. Caching: If there are frequent queries that read the data and the data does not change often, using a cache can drastically reduce the load on a database and improve performance.

8. Proper Memory Management: This is especially important in languages where memory management is explicit. Always deallocate memory that is no longer in use to prevent memory leaks.

9. Asynchronous Processing: If possible, use asynchronous processing or background jobs to perform non-time critical database operations.

10. Regularly Monitor and Optimize: Use database profiling tools to identify slow queries or performance bottlenecks. Regularly optimizing your database and queries can help maintain high performance.

Remember that the best practices can vary based on the specific database system used, application requirements, and the programming language in use. Always refer to specific best practices for your particular use-case.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 373.860875ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 2185 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 317.3245ms
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
  ✓ Text search completed in 65.416792ms

✓ Found 3 messages mentioning ownership:
  1. From conv_performance: Rust's ownership system provides zero-cost memory management. Use Box for heap allocation, Rc/Arc for shared ownership, and avoid cloning large data structures. The compiler optimizes away unnecessary allocations.

  2. From conv_rust_programming: Rust's key features include: memory safety without garbage collection, zero-cost abstractions, ownership system, powerful type system, and excellent concurrency support.

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
    ✓ Generated embedding: 1536 dimensions in 0.290s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.449s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.289s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.352s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.387s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.540s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.493s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.326s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.330s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.350s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.527s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.332s
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
    ✓ Generated embedding: 1536 dimensions in 0.590s
    • Function auto-cleaned up by client

→ Executing hybrid_search()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.095s

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

Writing memory-safe and high-performance database code requires a combination of good programming practices, efficient use of database features, and understanding the trade-offs between SQL and NoSQL databases as per your needs. Here are some tips:

1. Database Design: Start with a well-designed database. Use normalization to reduce data redundancy and improve data integrity. This often involves dividing large tables into smaller ones and defining relationships between them using foreign keys.

2. SQL or NoSQL: Choose the right type of database for your needs. SQL databases are generally better for complex queries, ACID transactions, and structured data with well-defined relationships. On the other hand, NoSQL databases excel at handling flexible schemas, horizontal scaling, high write throughput, and unstructured data.

3. Efficient Queries: Optimize your queries to reduce the load on your database. This can involve things like avoiding SELECT *, using indexes effectively, minimizing the use of joins where possible, and using the EXPLAIN statement to understand how your queries are being executed.

4. Connection Management: Efficiently manage your database connections. Open connections only when you need them and close them as soon as you're done. Use connection pooling where appropriate.

5. Memory Management: In your code, make sure to free up any resources that you no longer need, to prevent memory leaks. This includes closing database connections and freeing up any data structures you might be using to store intermediate results.

6. Error Handling: Implement robust error handling. Check the return values of your database calls and handle any error conditions appropriately.

7. Profiling and Monitoring: Regularly monitor the performance of your database and profile your code to identify any potential bottlenecks. Use this information to fine-tune your code and your database configuration.

8. Concurrent Access: If your application involves concurrent database access, ensure to handle it correctly using transactions in SQL or appropriate techniques in NoSQL to maintain data integrity and prevent race conditions.

Remember that the specific methods for implementing these tips can vary depending on the specific database system you're using and the programming language you're writing your code in.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.299s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 2339 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.309s
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
    ✓ Generated embedding: 1536 dimensions in 0.318s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.228s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.217s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.300s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.541s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.258s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.304s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.405s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.338s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.579s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.283s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.314s
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
    ✓ Generated embedding: 1536 dimensions in 0.394s
    • Function auto-cleaned up by client

→ Executing hybridSearch()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.093s
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

Writing memory-safe, high-performance database code requires a combination of good programming practices, understanding of database principles, and specific strategies for optimization. Here are some strategies:

1. Use the Right Data Structures: Use appropriate data structures that naturally fit the data you're dealing with. Incorrect data structures can lead to unnecessary memory usage and slow performance.

2. Use Indexes: Indexing is a way of sorting a database by specific fields (or columns) to speed up query performance. When you use indexes properly, your database can locate the data without scanning the entire table.

3. Database Normalization: As stated in Context 1, normalization helps to reduce redundancy and improve data integrity. It can also improve performance by reducing the amount of data that needs to be read from disk.

4. Utilize SQL or NoSQL Appropriately: As per Context 3 and 4, SQL is better for complex queries, ACID transactions, and structured data with well-defined relationships. NoSQL is more suitable for flexible schemas, horizontal scaling, high write throughput, or when working with unstructured data.

5. Write Efficient Queries: As per Context 5, optimizing your queries can greatly improve performance. This could mean avoiding SELECT *, minimizing the use of joins, and using WHERE instead of HAVING for filtering.

6. Use Connection Pooling: Instead of opening a new connection for every database query, use a pool of connections. This can significantly reduce the overhead of establishing a new connection every time a query is executed.

7. Use Caching: Caching is storing the result of an expensive calculation for later use. It can increase performance by reducing the need to re-compute or re-fetch data.

8. Memory Management: Be mindful of object creation in your code. Avoid creating unnecessary objects and ensure that objects that are no longer needed are properly disposed of to prevent memory leaks.

9. Use Prepared Statements: Prepared statements or parameterized queries not only help prevent SQL injection attacks, but they can also improve performance by allowing the database to efficiently cache and reuse query plans.

10. Monitor and Profile: Regularly monitor your application's performance and use profiling tools to identify bottlenecks. This will help you understand where you need to optimize your code.

Remember, what works best can depend on the specific use-case, database system, and infrastructure you are working with.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.245s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 2503 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.327s
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
  ✓ Text search completed in 0.064s
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
    ✓ Generated embedding: 1536 dimensions in 0.667s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.599s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.291s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.492s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.340s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.464s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.383s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.259s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.610s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.230s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.245s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.542s
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
    ✓ Generated embedding: 1536 dimensions in 0.326s
    • Function auto-cleaned up by client

→ Executing HybridSearch()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.099s

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

Writing memory-safe, high-performance database code involves several practices:

1. Write Efficient Queries: Whether you're using SQL or NoSQL, ensure your queries are efficient and optimized. This includes using proper indexing, avoiding N+1 query problems, and reducing the use of joins in SQL or denormalization in NoSQL where possible.

2. Normalization: In the context of SQL databases, normalization can help optimize the database structure by reducing redundancy and ensuring data integrity. This can lead to more efficient storage and better performance in many cases.

3. Use Appropriate Data Type: Be mindful of the data types you are using. Choosing the right data type can save memory and improve performance.

4. Connection Pooling: Establishing and closing database connections can be costly. Connection pooling can help you reuse existing connections, thus saving resources.

5. Batch Operations: If possible, batch your database operations. Instead of writing or reading one record at a time, batch operations process many records at once, which is often faster.

6. Caching: If there are data that are read frequently and seldom updated, consider caching them. This can significantly reduce database load and improve performance.

7. Use the Right Database Type: As discussed in Context 2 and 3, the choice between SQL and NoSQL can impact performance. Make sure to choose the right one according to your needs. SQL is great for complex queries and structured data with well-defined relationships, while NoSQL excels at handling unstructured data, high write throughput, and horizontal scaling.

8. Memory Management: Be mindful of memory usage. Avoid loading large amounts of data into memory at once. Use streaming or pagination to handle large data sets.

9. Error Handling: Proper error handling is important as it can help avoid memory leaks. For instance, make sure to close database connections even when errors occur.

10. Regular Monitoring and Optimization: Regularly monitor the performance of your database operations and optimize accordingly. Tools like slow query logs can help identify problematic queries that need optimization.

Remember, the specifics of how to implement these practices can vary depending on the programming language you are using and the specific database system you are working with.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.291s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 2338 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.265s
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

  3. From conv_new_question: Writing memory-safe, high-performance database code involves several practices:

1. Write Efficient Queries: Whether you're using SQL or NoSQL, ensure your queries are efficient and optimized. This includes using proper indexing, avoiding N+1 query problems, and reducing the use of joins in SQL or denormalization in NoSQL where possible.

2. Normalization: In the context of SQL databases, normalization can help optimize the database structure by reducing redundancy and ensuring data integrity. This can lead to more efficient storage and better performance in many cases.

3. Use Appropriate Data Type: Be mindful of the data types you are using. Choosing the right data type can save memory and improve performance.

4. Connection Pooling: Establishing and closing database connections can be costly. Connection pooling can help you reuse existing connections, thus saving resources.

5. Batch Operations: If possible, batch your database operations. Instead of writing or reading one record at a time, batch operations process many records at once, which is often faster.

6. Caching: If there are data that are read frequently and seldom updated, consider caching them. This can significantly reduce database load and improve performance.

7. Use the Right Database Type: As discussed in Context 2 and 3, the choice between SQL and NoSQL can impact performance. Make sure to choose the right one according to your needs. SQL is great for complex queries and structured data with well-defined relationships, while NoSQL excels at handling unstructured data, high write throughput, and horizontal scaling.

8. Memory Management: Be mindful of memory usage. Avoid loading large amounts of data into memory at once. Use streaming or pagination to handle large data sets.

9. Error Handling: Proper error handling is important as it can help avoid memory leaks. For instance, make sure to close database connections even when errors occur.

10. Regular Monitoring and Optimization: Regularly monitor the performance of your database operations and optimize accordingly. Tools like slow query logs can help identify problematic queries that need optimization.

Remember, the specifics of how to implement these practices can vary depending on the programming language you are using and the specific database system you are working with.

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
    ✓ Generated embedding: 1536 dimensions in 0.334s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.323s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.217s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.253s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.283s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.255s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.357s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.283s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.516s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.344s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.275s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.317s
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
    ✓ Generated embedding: 1536 dimensions in 0.3s
    • Function auto-cleaned up by client

→ Executing hybridSearch()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.101s

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
    ✓ Generated embedding: 1536 dimensions in 0.217s
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
  ✓ Text search completed in 0.046s

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
✓ Created SWR script: fetch_api_user (saSwxUuCWVitdrgoKm-c4ezaVdGaHFN6ebkjBE07oTRv5Rh21MpRQBlYd-9JsxXL74ee_bbk1iF6oQp_e6FNlA)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {}
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
Response time: 5ms (served from cache)
Result (cached): {
  "records": [
    {}
  ],
  "stats": {
    "input_count": 0,
    "output_count": 1,
    "execution_time_ms": 0,
    "stages_executed": 2,
    "stage_stats": []
  }
}
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_with_reviews (2UZpD1KuLdyKh2ulcz8Yk-Y8PiKz2XkCUwCm6QyB2Ga0T9C8pu2XcOvQ532ToXqGRh6JT8XLF82jRRk7eWxxPw)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {}
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
✓ Edge cache script created: EOk-Y3X4MuegR8WQlEuXyl2FjJZfuW2GT5OjF7TTPbuqOoLibSDw3F4DRKTsYXN_EfE629_lySxiPOkdPYtFvw

Call 1: Cache miss (fetches from API)
Response time: 5ms
Result: {
  "records": [
    {}
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
Response time: 5ms (1x faster!)
Result: {
  "records": [
    {}
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
    Finished `release` profile [optimized] target(s) in 0.08s
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
✓ Created SWR script: fetch_github_user (INemzBouj6fmBbJ-nDBxxAbiqmsu8H9oYrHSqBaD1K9qfTSTlP4s1a_nrp5E7K18-DKFlH4AV9FPsEu0AMbpSA)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 138ms
Result: [
  {
    "cached_at": "1769470245",
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
        "followers": 280575,
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
    "id": "EQrqcVtuKKWo4OtJEFoMXpJavjS5GoKTTKnurlnX4q9HhrRb2jvFkKVyt_EAJK0YHBck_KvQd-EEuz_a3I4sxQ"
  }
]
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 91ms (1.5x faster!)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (L5srZTsYr4c1BkMhnqzxcH4jGhRJm5X8t-4rLhEz3TS1nfQuDZEYr_Et7Xy02tv9g2PRXL_UJzci1gSI6cfHvQ)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: [
  {
    "enriched_at": "1769470245",
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
    "id": "QmuZqNB6qZyxBlBCBktQGOaKQPwxHw1WE56uToUF2hBrfie7ZrMMu8xU44m4M5sj6a3wT5fGQ3QA0AKFllsjqQ"
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
✓ Created SWR script: fetch_github_user (NZNEMqQCArQWxL1EWXHkvYGz5x8R4HjFpftn_Z7MFylBtIZq0jXIkAJSEWcoHG3K0uCE0y7ZYsHmB8o1515IxA)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 61.314292ms
Result: [
  {
    "cached_at": "1769470245",
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
        "followers": 280575,
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
    "id": "vqDGC6wbh-iNaWlUcwmIbQPMePirLXxC-ezfN9U3Pxv3saH4kW5FAGq_44t5twqdRxUUFHkU170TjFkcY_B_Fg"
  }
]
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 59.671042ms (1.0x faster!)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (QJSNDcTzeKA6OoOHs8lh4qgSj86EaUuEHRNRXEXrEQ0LzqPTS1L8cdi2LCP-eZ_h69P7xQdOtLKAoE0llrU3Og)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: [
  {
    "enriched_at": "2026-01-26T18:30:45-05:00",
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
    "id": "m4TmuZ7yKs6WueEOoNQ0Cs2u7KacnUKdecHa9IdaAlKVZQsEWdEoTJ7iX-uyc84WQtgw4eu14o1YBt6hfMZL1A"
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
✅ [32mClient build complete![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning Rust SWR Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/swr_pattern`
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_github_user (K7eP-WswAX9-MnQspyWIZZQCrvEh3ZjzdljLo42V8gHOIR9dNB52OZSStBGwm-_M00HUCmY8nf_vRYY-e2Kaug)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 75ms
Result: {
  "id": "_e64JouVab4FzchVs9NADG_oqi8mVcriW1JGY3Pk4ITdkLew0bSU9rHy00MHKpmk74Id4mFF2-73JKFBfIFKfw",
  "cached_at": "1769470245",
  "data": {
    "type": "Object",
    "value": {
      "url": "https://api.github.com/users/torvalds",
      "gravatar_id": "",
      "followers": 280575,
      "hireable": null,
      "node_id": "MDQ6VXNlcjEwMjQwMjU=",
      "type": "User",
      "created_at": "2011-09-03T15:26:22Z",
      "repos_url": "https://api.github.com/users/torvalds/repos",
      "name": "Linus Torvalds",
      "id": 1024025,
      "public_repos": 11,
      "gists_url": "https://api.github.com/users/torvalds/gists{/gist_id}",
      "events_url": "https://api.github.com/users/torvalds/events{/privacy}",
      "following": 0,
      "blog": "",
      "public_gists": 1,
      "bio": null,
      "avatar_url": "https://avatars.githubusercontent.com/u/1024025?v=4",
      "location": "Portland, OR",
      "html_url": "https://github.com/torvalds",
      "followers_url": "https://api.github.com/users/torvalds/followers",
      "updated_at": "2026-01-13T07:10:05Z",
      "user_view_type": "public",
      "organizations_url": "https://api.github.com/users/torvalds/orgs",
      "starred_url": "https://api.github.com/users/torvalds/starred{/owner}{/repo}",
      "email": null,
      "received_events_url": "https://api.github.com/users/torvalds/received_events",
      "subscriptions_url": "https://api.github.com/users/torvalds/subscriptions",
      "following_url": "https://api.github.com/users/torvalds/following{/other_user}",
      "twitter_username": null,
      "login": "torvalds",
      "site_admin": false,
      "company": "Linux Foundation"
    }
  }
}
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 78ms (1.0x faster!)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (uhM7kRWWjEAkDBNJyXHr2fdQiHdAzmoWIVk7WbG70vf_Gpdtmhshalv5YFpDO-u3os4yIGxRW0f7pRvVW_ER4A)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: {
  "enriched_data": {
    "type": "Object",
    "value": {
      "stock": 99,
      "sku": "BEA-ESS-ESS-001",
      "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
      "rating": 2.56,
      "discountPercentage": 10.48,
      "title": "Essence Mascara Lash Princess",
      "dimensions": {
        "width": 15.14,
        "height": 13.08,
        "depth": 22.99
      },
      "price": 9.99,
      "category": "beauty",
      "reviews": [
        {
          "date": "2025-04-30T09:41:02.053Z",
          "comment": "Would not recommend!",
          "reviewerEmail": "eleanor.collins@x.dummyjson.com",
          "rating": 3,
          "reviewerName": "Eleanor Collins"
        },
        {
          "date": "2025-04-30T09:41:02.053Z",
          "reviewerName": "Lucas Gordon",
          "comment": "Very satisfied!",
          "rating": 4,
          "reviewerEmail": "lucas.gordon@x.dummyjson.com"
        },
        {
          "comment": "Highly impressed!",
          "rating": 5,
          "reviewerEmail": "eleanor.collins@x.dummyjson.com",
          "date": "2025-04-30T09:41:02.053Z",
          "reviewerName": "Eleanor Collins"
        }
      ],
      "minimumOrderQuantity": 48,
      "id": 1,
      "brand": "Essence",
      "weight": 4,
      "warrantyInformation": "1 week warranty",
      "tags": [
        "beauty",
        "mascara"
      ],
      "meta": {
        "qrCode": "https://cdn.dummyjson.com/public/qr-code.png",
        "barcode": "5784719087687",
        "createdAt": "2025-04-30T09:41:02.053Z",
        "updatedAt": "2025-04-30T09:41:02.053Z"
      },
      "availabilityStatus": "In Stock",
      "returnPolicy": "No return policy",
      "images": [
        "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
      ],
      "shippingInformation": "Ships in 3-5 business days",
      "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp"
    }
  },
  "id": "bnfjMyedU6kxB4bXQYLi8VXejdpfkjJzYyjNIbF9chVKk94fj7eDtwHCyyvQT_j2viAaqDd0f0mqvI_tYp6DEA",
  "enriched_at": "2026-01-26T18:30:45-05:00"
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
✓ Created SWR script: fetch_github_user (KfYWWLNNHq5N15FIL9X6BshbWBJrUgWdpVnoOPO0zjdW7eNOwg_CKSk6lNDijtGqQRN640Q63NbofoeMMPA2cg)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 78ms
Result: [{"id":"_eay-6_5BAd332dn3yR4RXoiUbJGwUS0gbW1zloGRoNyMtwV7tFyXfRc43RhhanW83NekiAObJKAnm-k-Qy2iA","data":{"type":"Object","value":{"id":1024025,"node_id":"MDQ6VXNlcjEwMjQwMjU=","following":0,"created_at":"2011-09-03T15:26:22Z","url":"https://api.github.com/users/torvalds","bio":null,"gists_url":"https://api.github.com/users/torvalds/gists{/gist_id}","gravatar_id":"","blog":"","received_events_url":"https://api.github.com/users/torvalds/received_events","followers":280575,"following_url":"https://api.github.com/users/torvalds/following{/other_user}","type":"User","user_view_type":"public","avatar_url":"https://avatars.githubusercontent.com/u/1024025?v=4","hireable":null,"login":"torvalds","repos_url":"https://api.github.com/users/torvalds/repos","site_admin":false,"html_url":"https://github.com/torvalds","public_repos":11,"location":"Portland, OR","starred_url":"https://api.github.com/users/torvalds/starred{/owner}{/repo}","twitter_username":null,"subscriptions_url":"https://api.github.com/users/torvalds/subscriptions","public_gists":1,"updated_at":"2026-01-13T07:10:05Z","name":"Linus Torvalds","company":"Linux Foundation","organizations_url":"https://api.github.com/users/torvalds/orgs","events_url":"https://api.github.com/users/torvalds/events{/privacy}","followers_url":"https://api.github.com/users/torvalds/followers","email":null}},"cached_at":"1769470245"}]
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 63ms
Cache hit was 1.2x faster!

✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (E7IDRKxlNC-shhY8KqhaPX0Xw3B9XOyhdFZW4IRAOrxpz37aY7xJtKKD_kxdKFiDk7UgJQ4t7s-rqp8xj2IEcg)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: [{"enriched_at":"2026-01-26T18:30:45-05:00","id":"GPInwL07VlH2mISHCkBci0ZzJlsarVhTbxT98vA8ho5wzru98UpN8M3PZxzW1Cb-egsm3ylky94T09Iu58bEgQ","enriched_data":{"type":"Object","value":{"rating":2.56,"weight":4,"sku":"BEA-ESS-ESS-001","brand":"Essence","images":["https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"],"warrantyInformation":"1 week warranty","shippingInformation":"Ships in 3-5 business days","tags":["beauty","mascara"],"thumbnail":"https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp","stock":99,"dimensions":{"depth":22.99,"height":13.08,"width":15.14},"reviews":[{"reviewerEmail":"eleanor.collins@x.dummyjson.com","comment":"Would not recommend!","date":"2025-04-30T09:41:02.053Z","rating":3,"reviewerName":"Eleanor Collins"},{"comment":"Very satisfied!","reviewerEmail":"lucas.gordon@x.dummyjson.com","reviewerName":"Lucas Gordon","date":"2025-04-30T09:41:02.053Z","rating":4},{"comment":"Highly impressed!","rating":5,"reviewerEmail":"eleanor.collins@x.dummyjson.com","date":"2025-04-30T09:41:02.053Z","reviewerName":"Eleanor Collins"}],"title":"Essence Mascara Lash Princess","price":9.99,"returnPolicy":"No return policy","discountPercentage":10.48,"category":"beauty","id":1,"description":"The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.","meta":{"barcode":"5784719087687","createdAt":"2025-04-30T09:41:02.053Z","updatedAt":"2025-04-30T09:41:02.053Z","qrCode":"https://cdn.dummyjson.com/public/qr-code.png"},"minimumOrderQuantity":48,"availabilityStatus":"In Stock"}}}]
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


BUILD SUCCESSFUL in 4s
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

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 84.689458ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "website": "hildegard.org",
    "id": 1,
    "address": {
      "geo": {
        "lng": "81.1496",
        "lat": "-37.3159"
      },
      "suite": "Apt. 556",
      "zipcode": "92...

Second call (cache hit - from cache):
   ⏱️  Duration: 8.192625ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "phone": "1-770-736-8031 x56442",
    "id": 1,
    "email": "Sincere@april.biz",
    "website": "hildegard.org",
    "name": "Leanne Graham",
    "username": "Bret",
    "address": ...
   🚀 Cache speedup: 10.5x faster!

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
   Name: {"type":"String","value":"User 1"}
   Department: {"value":"engineering","type":"String"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 5ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "website": "hildegard.org",
    "name": "Leanne Graham",
    "company": {
      "name": "Romaguera-Crona",
      "catchPhrase": "Multi-layered client-server neural-net",
      "bs":...

Second call (cache hit - from cache):
   ⏱️  Duration: 5ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "website": "hildegard.org",
    "name": "Leanne Graham",
    "company": {
      "name": "Romaguera-Crona",
      "catchPhrase": "Multi-layered client-server neural-net",
      "bs":...
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

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 5.8ms
   📊 Records: 1
   📦 Data: {
      "value": {
            "address": {
                  "city": "Gwenborough",
                  "geo": {
                        "lat": "-37.3159",
                        "lng": "81.1496"
    ...

Second call (cache hit - from cache):
   ⏱️  Duration: 6.2ms
   📊 Records: 1
   📦 Data: {
      "value": {
            "address": {
                  "city": "Gwenborough",
                  "geo": {
                        "lat": "-37.3159",
                        "lng": "81.1496"
    ...
   🚀 Cache speedup: 0.9x faster!

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

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 5.19ms
   📊 Records: 1
   📦 Data: {
        "value": {
          "address": {
            "city": "Gwenborough",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "street":...

Second call (cache hit - from cache):
   ⏱️  Duration: 4.942083ms
   📊 Records: 1
   📦 Data: {
        "value": {
          "address": {
            "city": "Gwenborough",
            "geo": {
              "lat": "-37.3159",
              "lng": "81.1496"
            },
            "street":...
   🚀 Cache speedup: 1.2x faster!

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
   Name: {"value":"User 1","type":"String"}
   Department: {"value":"engineering","type":"String"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 6ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "website": "hildegard.org",
    "name": "Leanne Graham",
    "company": {
      "name": "Romaguera-Crona",
      "catchPhrase": "Multi-layered client-server neural-net",
      "bs":...

Second call (cache hit - from cache):
   ⏱️  Duration: 6ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "website": "hildegard.org",
    "name": "Leanne Graham",
    "company": {
      "name": "Romaguera-Crona",
      "catchPhrase": "Multi-layered client-server neural-net",
      "bs":...
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

