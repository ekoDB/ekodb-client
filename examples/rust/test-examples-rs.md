make test-examples-rust
🧪 [36mRunning Rust examples (direct HTTP/WebSocket)...[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/examples/simple_crud`
✓ Authentication successful

=== Insert Document ===
Inserted: Object {"id": String("h2TtMXc-aeb66kjs-LUGXGzSn4sjQ3a94S5aE1hJBlBXxS-P6yMlhtDBrOCf_QUjjBIgzvPzR5fSRPrEB9FZvQ")}

=== Find by ID ===
Found: Object {"active": Bool(true), "id": String("h2TtMXc-aeb66kjs-LUGXGzSn4sjQ3a94S5aE1hJBlBXxS-P6yMlhtDBrOCf_QUjjBIgzvPzR5fSRPrEB9FZvQ"), "name": String("Test Record"), "value": Number(42)}

=== Find with Query ===
Found documents: Array [Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("h2TtMXc-aeb66kjs-LUGXGzSn4sjQ3a94S5aE1hJBlBXxS-P6yMlhtDBrOCf_QUjjBIgzvPzR5fSRPrEB9FZvQ"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}]

=== Update Document ===
Updated: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("h2TtMXc-aeb66kjs-LUGXGzSn4sjQ3a94S5aE1hJBlBXxS-P6yMlhtDBrOCf_QUjjBIgzvPzR5fSRPrEB9FZvQ"), "name": Object {"type": String("String"), "value": String("Updated Record")}, "value": Object {"type": String("Integer"), "value": Number(100)}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/simple_websocket`
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: "S_U13Mss9h6XuHFJAuiD6zeBJ1UT7ZOk0_SZxhcjzzoyIThIysh_qUPR8HijAPJf_mlmpEgauyRwdt0ZfUuDxA"

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
        "id": "S_U13Mss9h6XuHFJAuiD6zeBJ1UT7ZOk0_SZxhcjzzoyIThIysh_qUPR8HijAPJf_mlmpEgauyRwdt0ZfUuDxA",
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
Collection created with first record: "wlbxRs7PXKKKbWgJ8eQHCMVjoq3w9kw49yPVfr6JBuQKoMHmMQ8O3hm3Qkp8KVsGfjHKSV9MG-H7IS2LV3-csg"

=== List Collections ===
Total collections: 4
Sample collections: ["batch_users", "demo_collection", "test_collection", "websocket_test"]

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
✓ Inserted document: "Nb9pxpZVRrL6YNTVkPK1fg2XmFPWdfolRSpztSqTa4HeC2NoZfuRNZs7NkOC9M4JzjEH-fhkBBw-QvCZE_ZZyQ"

=== Insert Document with TTL (5 minutes - integer) ===
✓ Inserted document: "XUKXnCqMhuAHLbPgQns2Qd1NeD6fqxEKGq-sTcBul4xGc02l8yXKVx_YRABPcVWCR3EUM2vWXOpSVn6dB4GrzA"

=== Insert Document with TTL (30 minutes - duration string) ===
✓ Inserted document with duration string TTL: "gCaPKKFKIkZ09oWlsU5LO1y83qFcwnFRoRqgyPYg4vkx1Y_GzpnCazLA3ktKHShDXzuUnrvxUsPi9hahQ_1xnQ"

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
✓ Inserted document with TTL: String("Ti0W3iAALdTyMtI-dkxo-awwAI9pf1DDAMcOUlHz_E0YSlmgDPV-eLpjZ59Mxg8eArJZYaq8MAlnNM2soYU3iA")

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket

Record 1:
  id: "Ti0W3iAALdTyMtI-dkxo-awwAI9pf1DDAMcOUlHz_E0YSlmgDPV-eLpjZ59Mxg8eArJZYaq8MAlnNM2soYU3iA"
  name: {"type":"String","value":"WebSocket TTL Test"}
  ttl: "2026-01-27T18:11:32.989133Z"
  value: {"type":"Integer","value":42}

✓ WebSocket TTL example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/http_functions`
🚀 ekoDB Scripts Example (Rust/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: 8kSKrtoavyeJC7i10SWKnbHcN76TnVWjMrzaI9ns4sXOmrNXwczJ4jsg_ASblkIHfQVWdkGMUrkca2rsyN-mAw
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: "3o0Z0yKrbpyHU_b-UsgFauZ6cVuE6XANx_0CWFZLrq9G3j2V883ETSr2h1OwZwHA7zjdA2gvUTaAhkViKjwOCA"
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Complex Filter with Multiple Conditions

✅ Script saved: EQf2RselOXi7M_NeauGEthGIwZLM4Y27wy2BnR8CM0rkJPyQKF2DURO4G4YNg9wSAGV5oyQIqYwmZK_QPQlKvA
📊 Found 3 users (status=active, score>50, sorted by score)

📝 Example 4: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: Ik17SnKd4rIhXYhLLTLAT1TtbLGB5Es3yurSwjknHWoOg0hiF9Nyr3ZiUaJNgYQygv4AkBUV9cscRUMjT5zo5g
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
Created Alice: $1000 - ID: Iy_0j5O9lFYCv-O2zklGVTt-5oPLSvD8-iPbfkp4PGIfzrOJtjMjW7VuRWx4_3pvdSabkE8e1UslywPKF9rugA
Created Bob: $500 - ID: KIdXVg4049LKtGwYI5zsUNXB6gBnOWfnJfNcRi9k5Riy-W7mufGnVtLSSlCtWpKw2h1RNii7YyPF7UVelL3mMg

=== Example 1: Begin Transaction ===
Transaction ID: 7ec531e2-e86a-4eaa-9c08-26a142c04cc6

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
New transaction: ecb087f1-3530-4047-a384-217beacfedf6
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
Inserted: Record({"id": String("RcjromiCIJK_-lnY3nKIUFeaW0RdT48g_ahJyYAYOrxpXgD-S-lklMTGVqnnyaIk1zMyJBxZjNT9-5G0f61zfw")})

=== Find by ID ===
Found: Record({"categories": Array([String("electronics"), String("computers")]), "value": Integer(42), "embedding": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "name": String("Test Record"), "created_at": String("2026-01-27T17:11:38.670285+00:00"), "price": Float(99.99), "user_id": String("550e8400-e29b-41d4-a716-446655440000"), "id": String("RcjromiCIJK_-lnY3nKIUFeaW0RdT48g_ahJyYAYOrxpXgD-S-lklMTGVqnnyaIk1zMyJBxZjNT9-5G0f61zfw"), "active": Boolean(true), "metadata": Object({"key": String("value"), "nested": Object({"deep": Boolean(true)})}), "data": String("aGVsbG8gd29ybGQ="), "tags": Array([String("tag1"), String("tag2"), String("tag3")])})

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Some("Test Record")
  value (Integer): Some(42)
  active (Boolean): Some(true)
  price (Decimal): Some(99.99)
  created_at (DateTime): Some("2026-01-27T17:11:38.670285+00:00")
  user_id (UUID): Some("550e8400-e29b-41d4-a716-446655440000")
  tags (Array): 3 items
  metadata (Object): 2 keys
  embedding (Vector): 0 dims
  categories (Set): 0 items
  data (Bytes): 11 bytes

=== Find with Query ===
Found documents: [Record({"categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])}), "metadata": Object({"value": Object({"key": String("value"), "nested": Object({"deep": Boolean(true)})}), "type": String("Object")}), "embedding": Object({"value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "type": String("Array")}), "name": Object({"value": String("Test Record"), "type": String("String")}), "price": Object({"type": String("Float"), "value": Float(99.99)}), "data": Object({"value": String("aGVsbG8gd29ybGQ="), "type": String("String")}), "id": String("RcjromiCIJK_-lnY3nKIUFeaW0RdT48g_ahJyYAYOrxpXgD-S-lklMTGVqnnyaIk1zMyJBxZjNT9-5G0f61zfw"), "user_id": Object({"type": String("String"), "value": String("550e8400-e29b-41d4-a716-446655440000")}), "value": Object({"value": Integer(42), "type": String("Integer")}), "tags": Object({"value": Array([String("tag1"), String("tag2"), String("tag3")]), "type": String("Array")}), "active": Object({"type": String("Boolean"), "value": Boolean(true)}), "created_at": Object({"value": String("2026-01-27T17:11:38.670285Z"), "type": String("DateTime")})})]

=== Update Document ===
Updated: Record({"tags": Object({"type": String("Array"), "value": Array([String("tag1"), String("tag2"), String("tag3")])}), "data": Object({"type": String("String"), "value": String("aGVsbG8gd29ybGQ=")}), "user_id": Object({"type": String("String"), "value": String("550e8400-e29b-41d4-a716-446655440000")}), "value": Object({"type": String("Integer"), "value": Integer(100)}), "price": Object({"value": Float(99.99), "type": String("Float")}), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "categories": Object({"value": Array([String("electronics"), String("computers")]), "type": String("Array")}), "id": String("RcjromiCIJK_-lnY3nKIUFeaW0RdT48g_ahJyYAYOrxpXgD-S-lklMTGVqnnyaIk1zMyJBxZjNT9-5G0f61zfw"), "embedding": Object({"type": String("Array"), "value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)])}), "created_at": Object({"type": String("DateTime"), "value": String("2026-01-27T17:11:38.670285Z")}), "metadata": Object({"type": String("Object"), "value": Object({"key": String("value"), "nested": Object({"deep": Boolean(true)})})}), "name": Object({"value": String("Updated Record"), "type": String("String")})})

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_simple_websocket`
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: mLEkEt-BFnYFRzJJzVzi9cgtt9zcwkZF_0HGwWIjcOdB_BgcOGOkYTRNA2q7WEC2LIDpue5br4jQFUznAK3Bpg

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket

Record 1:
  value: 42
  active: true
  name: "WebSocket Test Record"
  id: "mLEkEt-BFnYFRzJJzVzi9cgtt9zcwkZF_0HGwWIjcOdB_BgcOGOkYTRNA2q7WEC2LIDpue5br4jQFUznAK3Bpg"

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
Collection created with first record: "Hw8Fy8RUTT-5XjWLPDdbzXR34__5ivJu8vcL5S5hvf_Q68lJ6rRRGRl3oisX2zSk3vMifw9wjfY-_GT4uEp_sQ"

=== List Collections ===
Total collections: 9
Sample collections: ["test_accounts", "batch_users", "test_collection", "scripts__ek0_testing", "websocket_test"]

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
Created Alice: $1000 - ID: Ak_GLry5tXyh_7zTASdkgKCuoRbu2pVE4zwtuqCT8nLKwAQTk78xd4i986HROyViig2zhW2Mz43AUMoD4Ecftw
Created Bob: $500 - ID: hAIHal28x0TS8r0DemRcxVoMZvG-_Fsn6nbD_rC1OHomSN78xajxFnAsTcmoOk6FAVkXCnZ_mK9RORY-IWne7A

=== Example 1: Begin Transaction ===
Transaction ID: e49f3654-4b30-4e08-a3b1-f9c205456264

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: "Active"
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 4d29d49a-8fb9-48bb-9c33-d5d504b9da3c
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
  - Some(Object({"value": String("David"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("Bob")}))
  - Some(Object({"type": String("String"), "value": String("Alice")}))

=== Range Query (age >= 28 AND age < 35) ===
✓ Found 3 users in age range
  - Some(Object({"value": String("Eve"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("David")}))
  - Some(Object({"value": String("Bob"), "type": String("String")}))

=== IN Operator ===
✓ Found 4 users with status active or pending
  - Some(Object({"type": String("String"), "value": String("Eve")}))
  - Some(Object({"value": String("David"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("Bob")}))
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
  - Some(Object({"type": String("String"), "value": String("Bob")}))

=== OR Query ===
✓ Found 2 users with age < 28 OR age > 32
  - Some(Object({"type": String("String"), "value": String("Charlie")}))
  - Some(Object({"type": String("String"), "value": String("Alice")}))

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
  2. Score: 6.6000 - Some(String("Python for Data Science"))

=== Cleanup ===
✓ Deleted collection

✓ All search operations completed successfully
Execution time: 3ms
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_schema`
=== ekoDB Schema Management Example ===

=== Creating Collection with Schema ===
✓ Created collection 'schema_client_rust' with schema

=== Inserting Valid Documents ===
✓ Inserted user 1: Some(String("iVd419cRZj2dnNmCWMI1pjaResGpnow0SO9QSjTTvlRfuj1tHXGa95OCYtkJq7bMM-pFzf8AzKvjlW_MzjL7lg"))
✓ Inserted user 2: Some(String("YvBY3b3T3-nWoRCjvJ0L43t4_C1lOWGTDgMMlZkA5TjLg4y26mGpLjNPNHFDfly-EgwKvAVtym8LCnv9Bke6pQ"))

=== Getting Schema ===
✓ Schema for schema_client_rust:
  - status: String
  - email: String
    (required)
  - age: Integer
  - title: String
    (required)

=== Listing Collections ===
✓ Total collections: 9
  Sample: ["test_accounts", "batch_users", "test_collection", "scripts__ek0_testing", "websocket_test"]

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
✓ Inserted document: "6HLxTcgD5ZqSqs-b688LAPnkrnTyXq1mBJQABOdeM_wYUjfeoxYWjdLq16eVnZHfceyGPs_eiAkMd6-7U70sZA"

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: Some(String("h3QV4gDJwN-5IVGIz7LNhXzFdW5xBd3vFLfi_BcKZQiVJxKRyftMCoE3FrHSUgR0E6g7j4s9COL1EDzMHDi9jw"))

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
✓ Inserted document with TTL: Some(String("aphZboXPl_l_-2O4-4fBAu4_iedaKW415lzr2uUmZbxR423D5boFaiyxHLf9BpEp2Ef1O50TvQUIlwTpP_jtYw"))

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
✓ Edge cache script created: Pmu7VNT0hdMoNh6NgPPdzsHY2Q1FRkSEQ3ixtm1N7K6EUDH4Kqc0zAaK6TdcEwhCQYrBntTGlLW0t8CkVwC_Iw

Call 1: Cache lookup
Response time: 3ms
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

✅ Script saved: PxrdBr67CSgr7lMtOltfRstHKUkr5dohDwC-25dxYd0cMCuvFpqcLBc58PnV2gUI5JL1ILVq9FPwt_PFGjNn5A
📊 Found 20 active users

📝 Example 2: Parameterized Script

✅ Script saved: 5Qs75cePJ550fuvwdvyDc63cVRU_GLj99l-zjAnJlxD4d-N_tekv-8TX5wLle6_fnjJ5ob0UCTYwboldmjBefg
📊 Found 20 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: ERRK-scHXH3S7u147MyJ-XjeyIrF_53fmk_0W6yqQ2vLt7OER4DX-xd__tMqtR5GNGQNivd4fvSccQKiaocEow
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
   ⏱️  Duration: 182.207625ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "email": "Sincere@april.biz",
    "website": "hildegard.org",
    "address": {
      "suite": "Apt. 556",
      "zipcode": "92998-3874",
      "geo": {
        "lng": "81.1496",
   ...

Second call (cache hit - from cache):
   ⏱️  Duration: 3.747209ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "address": {
      "geo": {
        "lng": "81.1496",
        "lat": "-37.3159"
      },
      "street": "Kulas Light",
      "city": "Gwenborough",
      "zipcode": "92998-3874",
 ...
   🚀 Cache speedup: 60.7x faster!

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

✅ Script saved: 6kIhAs0BSLm0Fmq5-hf-A455r4YVoqy7Zx31mX5d7rFysStn9zSBjBhAKmN5qH7tjvGziU7b7BtvgysjMIO9cQ
📊 Found 2 product groups
   Record({"avg_price": Float(474.0), "category": String("Furniture"), "count": Integer(2)})
   Record({"count": Integer(3), "avg_price": Float(575.6666666666666), "category": String("Electronics")})
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

✅ Inserted order: Some(String("wNICmWY-VHDKqb_c1cA9L7Kz1Tn9D4OIpIcQBFyl224kUBoWpowkMW-3TYCGxMJaSafi4nRQrYTBo-x9JkrATg"))
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: BnXNW7WRn4-GLYrfsN-kaxe9SLu6LJZklfNmMvFHRoWmjEmPgA3R7ibjpAcirXB6Wvfr8QQsUZJGRkI35m2X2A
📊 Script executed
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: Some(Object {"role": String("admin"), "userId": String("user_abc")})
🔍 Key exists: true
✅ Set cached data
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: lRy1A9r1q6eow57UhZJuZpeZoQTB6LGP97wvofZWevc0z81Eay2m58ZsbF9BSg3vM_xn23ACqBaLA-nrdXyDZQ
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Script

✅ Script saved: U-DPWT0GVoFC-tlFyCHAIINzBmVobNfljGrvpzZ5CHI0PhxjPZ_aDRALynUmFBcb3w_cgb99b6_n6TzvnthauQ
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
✓ Created SWR script: fetch_api_user_rs (YNwLQcgpxE1OPEJzCYFLaqddcXk_Hq_8Out0LwqBmj-hY1qWPp0thgAmo1CHKrKfwqLintAW83l7gFvK56UiDQ)

Step 2: First call - Cache miss, fetches from API
Result: FunctionStats { input_count: 0, output_count: 1, execution_time_ms: 0, stages_executed: 2, stage_stats: [] }
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 77ms (served from cache)
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

✓ Created native SWR script: github_user_native (6VbBHddUrn5GeoS36TDYauGTQ5zZYXKKPdfzov0h__yDECYpOs4hhXsfi1jHiu9tl5R-FOlom8cVrqPoda7CIA)

First call (cache miss - will fetch from GitHub API):
  Response time: 132ms
  Records returned: 1
  ✓ Data fetched from API and cached with 15m TTL

Second call (cache hit - instant from KV store):
  Response time: 11ms
  Speedup: 12.0x faster
  ✓ Lightning fast cache hit


Example 2: SWR with Audit Trail Collection
─────────────────────────────────────────────────────

✓ Created SWR script with audit trail: product_swr_audit (Xlg4DRvASR8fTYbBM-NLouN5Mvd6k_eXDhx6ohLv-3UOnQJRvFldI5eWax7_uj7Xl7Rcdj6hPKfRWylW3TCC8Q)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
─────────────────────────────────────────────────────

Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (pysDEIBWfw3FDC90VJ17T7EvDSlvBmMQM_xbH46-2P-w62-YptWNcuWGR6uuyt0aN6QaCWC-CGBCPJR8c4U8tg)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
─────────────────────────────────────────────────────

✓ Created dynamic TTL script: flexible_cache (1cDoEbh00j6Cy2jf4CB5rKNsfboV5rTY-Vj3pE9H04K5stHrUSsBSBGSmah37ETOhHhez7KhXoHE7zxLJqWdyQ)
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
   Record({"count": Integer(5), "avg_price": Float(367.0), "category": String("Electronics")})
   Record({"avg_price": Float(365.6666666666667), "count": Integer(3), "category": String("Furniture")})
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
   Vector databases offer benefits such as efficient data storage, high precision for geometric data, easy data manipulation and transformation, support for complex shapes and structures, and improved query performance.
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
   Record({"status": String("inactive"), "count": Integer(3)})
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
   2. Database Design Principles (Database)
   3. Introduction to Machine Learning (AI)
   4. Natural Language Processing (AI)
   5. Vector Databases Explained (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Script saved
📊 Documents by category:
   Record({"count": Integer(3), "category": String("Database")})
   Record({"count": Integer(2), "category": String("AI")})
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
✓ Created session: O5WSPWnxYUn00jIQyOvwuAX84ZXyGYnF_HOnwe0EBVt-TTf9oMr7iziaWfFbk57WXUb3Xd3_RSNC0h1XSgmiSg

=== Sending Chat Message ===
Message ID: 38vnN_riZUpmQjiF9p9Tc-U8luanfCsg8sv9gwJFuYk--vG89Ea5R4lDQNL-Z-Vpqb5wiXcbDakOoF0HSzmejw

=== AI Response ===
Response 1: ekoDB is a high-performance database that features intelligent caching, real-time capabilities, and AI integration. It supports full-text search, vector search, and hybrid search with automatic context retrieval, ensuring efficient and precise search results. One of its standout features is the AI Chat Integration. This feature allows you to query your database using natural language and receive AI-powered responses, making database management and data retrieval more intuitive and user-friendly.

=== Context Used (3 snippets) ===

Snippet 1:
  Collection: client_chat_basic_rust
  Score: 0.6333
  Matched Fields: ["category", "title", "content"]
  Record: Object {"category": String("features"), "content": String("The chat feature allows you to query your database using natural language and get AI-powered responses with relevant context."), "id": String("3DPjORBI9yHbhwAG2ThJCqY5QYiD6BXgvtx9lXvtyzod3FLKnkAQ4V4fr4HPvoBg3hE1TrBGKHGBFUEQShwSmg"), "title": String("AI Chat Integration")}

Snippet 2:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["content", "title"]
  Record: Object {"category": String("documentation"), "content": String("ekoDB is a high-performance database with intelligent caching, real-time capabilities, and AI integration."), "id": String("2eQ5FAH9K27SAKZn84vWkLhA_BqkEUxK4oGVpkpbzbY5dsQzodI1higQcq6PNfoCpcLowPN8B0x_lm23h86nQA"), "title": String("Introduction to ekoDB")}

Snippet 3:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["title", "category", "content"]
  Record: Object {"category": String("features"), "content": String("ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval."), "id": String("ZNLPx1XF0twA5DRw6d0C8rAWi0ubWXhyjQgDB4T6ZVFzGk7ENKS1XZP8lfQX8-0ClY7z6zJEui_C_bOegyvDsg"), "title": String("Search Features")}

Execution Time: 4095ms

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_chat_advanced`
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 0fd14TMwefHBe-JH1ERy2WbzbPA01501tf9IFbDLfRlm1WYiBE5doWuEZawoYkM0Hg9q2XNp5wtPaX3wy19aew

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information provided, there is a product available called ekoDB. It is a high-performance database product and it's priced at $99.

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
✓ Created second session: MFbKEwGka_F5zUqzfAjBoCmSLbdAGQnNUFluQGZQBVOpiWcYKxRFe4Oy3bwrvytTuJMcL4AkRMra-kseAeNFWA
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
✓ Created session: -X3r6ftFEexwrKAvlzkZrkxeHh-JiNF87XeKMozCByA1f4epxPc0neizTsZUeZo--ymhrhF980pm_eMdeb0n3w
=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information provided, the available product is ekoDB. This is a high-performance database product with AI capabilities. It is priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: AHUngMYELHYO7Qunc8NP4hbjs3vpfTr0fURvBNbdkth_N8BtD-dmjzO4xvk2gRiFpYJ_FpU93q-JPy2pxZbCZw
  Parent: -X3r6ftFEexwrKAvlzkZrkxeHh-JiNF87XeKMozCByA1f4epxPc0neizTsZUeZo--ymhrhF980pm_eMdeb0n3w

=== Listing Sessions ===
✓ Found 4 sessions
  Session 1: AHUngMYELHYO7Qunc8NP4hbjs3vpfTr0fURvBNbdkth_N8BtD-dmjzO4xvk2gRiFpYJ_FpU93q-JPy2pxZbCZw (Untitled)
  Session 2: -X3r6ftFEexwrKAvlzkZrkxeHh-JiNF87XeKMozCByA1f4epxPc0neizTsZUeZo--ymhrhF980pm_eMdeb0n3w (Untitled)
  Session 3: MFbKEwGka_F5zUqzfAjBoCmSLbdAGQnNUFluQGZQBVOpiWcYKxRFe4Oy3bwrvytTuJMcL4AkRMra-kseAeNFWA (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: AHUngMYELHYO7Qunc8NP4hbjs3vpfTr0fURvBNbdkth_N8BtD-dmjzO4xvk2gRiFpYJ_FpU93q-JPy2pxZbCZw

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_convenience_methods`
=== ekoDB Convenience Methods Example ===

=== Record Builder Pattern ===
✓ Created record with fluent builder: Record({"id": String("nXzJK6QooL5sNezs6kMZGNANiVckoul8uOa9T0jA3NX0ZJkCN7OMfwMBCeNKerm2SfcQXXemBDYQYQSTlXIhyA")})

=== Upsert Operation ===
✓ First upsert (insert): Record({"email": Object({"value": String("bob@example.com"), "type": String("String")}), "id": String("nXzJK6QooL5sNezs6kMZGNANiVckoul8uOa9T0jA3NX0ZJkCN7OMfwMBCeNKerm2SfcQXXemBDYQYQSTlXIhyA"), "age": Object({"type": String("Integer"), "value": Integer(35)}), "name": Object({"type": String("String"), "value": String("Bob Smith")}), "active": Object({"value": Boolean(true), "type": String("Boolean")})})
✓ Second upsert (update): Record({"active": Object({"type": String("Boolean"), "value": Boolean(true)}), "name": Object({"value": String("Bob Smith"), "type": String("String")}), "email": Object({"value": String("bob.smith@newdomain.com"), "type": String("String")}), "age": Object({"type": String("Integer"), "value": Integer(36)}), "id": String("nXzJK6QooL5sNezs6kMZGNANiVckoul8uOa9T0jA3NX0ZJkCN7OMfwMBCeNKerm2SfcQXXemBDYQYQSTlXIhyA")})

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
   Inserted with ripple: Record({"id": String("eOpFsaqOzU-_uYr8iFJbl2SMt2RTPWHRjKvoOk7gFbYs-fK_4ObWjDLNna88HdYCbeOuqvtxQ3LkxScA5Ihpaw")})

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: Record({"id": String("3Df1uImxbxTa3Cnyw1OpNqgWRK_Q8bwIVlGd5Ymd9LdZtKQlwj6-YqWQqg034HPdQdrIwxhN28QlOr4v-vdBWQ")})

3. Update with bypass_ripple:
   Updated with bypass_ripple: Record({"name": Object({"type": String("String"), "value": String("Product 1")}), "price": Object({"type": String("Integer"), "value": Integer(150)}), "id": String("eOpFsaqOzU-_uYr8iFJbl2SMt2RTPWHRjKvoOk7gFbYs-fK_4ObWjDLNna88HdYCbeOuqvtxQ3LkxScA5Ihpaw")})

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: Record({"id": String("K8X_6YH3pYKj9U-0IPpwFY4VbYnccmJqDqCP_xM1Yn4qLZCzkAd84FijTpkw2J9-Sv0fczFAZR64UkyMdh8TrQ")})

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
