make test-examples
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
✅ [32mAll Rust integration tests complete![0m
🧪 [36mRunning Python examples (direct HTTP/WebSocket)...[0m
=== Simple CRUD Operations (Direct HTTP) ===

✓ Authentication successful

=== Insert Document ===
Inserted: {'id': 'MUZ9fHXO-lG9YfY3ObfTayyWeWy6eMh47MQdGBXqrPdaSP_aLI8JEBjH1bZPH_a74i9L5byfbQf7S6g604HuHA'}

=== Find by ID ===
Found: {'name': 'Test Record', 'value': 42, 'active': True, 'id': 'MUZ9fHXO-lG9YfY3ObfTayyWeWy6eMh47MQdGBXqrPdaSP_aLI8JEBjH1bZPH_a74i9L5byfbQf7S6g604HuHA'}

=== Find with Query ===
Found documents: [{'name': {'type': 'String', 'value': 'Test Record'}, 'active': {'type': 'Boolean', 'value': True}, 'value': {'type': 'Integer', 'value': 42}, 'id': 'MUZ9fHXO-lG9YfY3ObfTayyWeWy6eMh47MQdGBXqrPdaSP_aLI8JEBjH1bZPH_a74i9L5byfbQf7S6g604HuHA'}]

=== Update Document ===
Updated: {'id': 'MUZ9fHXO-lG9YfY3ObfTayyWeWy6eMh47MQdGBXqrPdaSP_aLI8JEBjH1bZPH_a74i9L5byfbQf7S6g604HuHA', 'value': {'type': 'Integer', 'value': 100}, 'name': {'value': 'Updated Record', 'type': 'String'}, 'active': {'type': 'Boolean', 'value': True}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: S-g_KsEmXxeRR7SufzogTElDqP-F9m9IRs05FO8UL78xYZaP_aAkSbHUzqAbaHHS4eeylQKCp90CHoPUwFFtkQ

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
      },
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "S-g_KsEmXxeRR7SufzogTElDqP-F9m9IRs05FO8UL78xYZaP_aAkSbHUzqAbaHHS4eeylQKCp90CHoPUwFFtkQ",
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

✅ Script saved: pSKK_aMuPXVbU23PL5qxMArZ-kd27hazbNlvMVtdg6NPGadorCFJZ3Pf6-j4d-ElWyWAZGWj2ruvuUOxw-dxUg
📊 Found 35 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: 6HWYt9tpSedqMwmQy9A3P6apNJGzwUVaL2qMib8URg2uAR55-xhc7LLX91cNqfHX4x9GLH-3m6WXXwfb18YDhg
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: Sb37UIRBb4v0h__SuBQaIaEkN6nXH8KlCmjqz3tKnz2dL4I2aR9G2zs7KY3Me3tzuZ6ZONJlb3RNFxh7w2B8xw
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {'status': 'inactive', 'count': 15, 'max_score': 90, 'avg_score': 50.0}
   {'avg_score': 60.0, 'max_score': 100, 'count': 15, 'status': 'active'}

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
Retrieved value: {'userId': 123, 'username': 'john_doe'}

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
Collection created with first record: ZHiKwMsQAFJsq1jBDGXA5Qy-w6bMf7TFFh63RQUiQBhcd9b9neu6jj5hbkY3zzkN929V2mtafGppa9cUZt4H-w

=== List Collections ===
Total collections: 12
Sample collections: ['chat_configurations__ek0_testing', 'test_accounts', 'batch_users', 'products', 'demo_collection']

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
Created Alice: $1000 - ID: QvnSno6KXTmXgYOQe5DscfJbU-1rrHihx9xG9ckE5SnI8BkQWKjvjAr5oED5t6zyb4_aSIArpp-V9LEQsdACbw
Created Bob: $500 - ID: d8OSVdb7PyoVS2dM2CTXCpYUlN3YNQBqkRMgf99eZwTqL7kAGMmsfXpCeZvGxtR8-QYfFrO-TR689AvcLSHw5g

=== Example 1: Begin Transaction ===
Transaction ID: ade7229f-949a-4f3f-8c39-5da0ffa642a5

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
New transaction: ae2495dd-69ca-4eaf-9a48-c41129980e40
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: snK-zRLdmMadVtk-gZLQHAgPCyFPmvVVc9YjBtTpok3A6xQ3vbbka3KktUHzd6f9LgFTTK4sijMvTTadT-1KNg
Created Bob: $500 - ID: Kdahk9UJNfk5y3J24I1hr7kNrNto3cAesVLxuofsRD24dYid2z1zfbC9VE7SbiwN93aD-oRsvIf2oYX8RuinlA

=== Example 1: Begin Transaction ===
Transaction ID: 7b654b00-4883-404e-8265-84019719e6d7

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
Created Alice: $1000 - ID: C22cXT4tzhfFoKrvdQKfZUl0IxMw5CjNlpyxwydwLDBEt2ummIm8s62zBx_0t_-YoFUtzMs6Fc4Lt_6SjTaHyQ
Created Bob: $500 - ID: EnSWvJyaiZxnGPYP_SU4MsT8xFVixj3Wl9t8tgKsqpb6FPDol-gEvsSy0cD7hZ2x21wOuqxu-1Yeyo80nya50Q

=== Example 1: Begin Transaction ===
Transaction ID: c047db4c-ada5-4b6b-960a-ce231f4ac11d

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
New transaction: ec2f130c-f005-473e-b696-c762ec3100bb
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: OqasfENerjTmbbysUPaneJl7JW2NMn5m8XL_V4f-Rw8IwHUN9DjXViAc0F5gssUwbZ18b6nB_PpjswB2VASL5g
Created Bob: $500 - ID: QaDzGNJ-IwxvLPjQXKwXwkwMwrAqI4lKmWK19zOIS2Sxb-jN1pOmjUwuCF3ZhxAABerFmvKBQNLYFHofJpWnAw

=== Example 1: Begin Transaction ===
Transaction ID: 258e8a5e-7947-4e4e-9daa-c19635ed1d03

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
Created Alice: $1000 - ID: 7v-RP-LPFMVIf4AloDztHdjnNteFM0bljdd8hpMB6RDLbAk-qJh3aiXZXJzXtgbcoDjjXIB-yaVqcEvPFw0O8A
Created Bob: $500 - ID: vXyxQT-F_UVibygVMDEua_RNlWnIYOcjcS4MSyijh_q8q-Bbvu2H6P1IGS9jMo_cO85O54D8VjW2H1Fkut3f2g

=== Example 1: Begin Transaction ===
Transaction ID: 43a2490b-186e-4f14-b5a1-3f0a4b7160c9

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
New transaction: 492fafe4-b18f-49fe-8e9e-e40b253fe420
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: EJtp7pTDcXUVNUz9Rrim0Qg9oaeWP4PazBvzPCydUFiraVM6OrVZ67q_3ECwu26-GdU4kJ6THMsmB22UtLlmYg
Created Bob: $500 - ID: lozqYi9Mz4zFPs4Awmv5MVVBK9HtVbVJTEPf1QGJmdnlijUfjAB-9eNmTPZWSKTU7PeGC7jLLOdyxA_6Ne9-rA

=== Example 1: Begin Transaction ===
Transaction ID: 6d944704-b05f-41e4-a65a-a287713ff22d

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
   ✅ Script saved: Upg72njwfh2wie13rsU5xZS-uVbKfDROqrjxKGyuUycaoYoMDA7gXIEVAccALoP-nV2jxZf2KQBZ-j_H1VN7Zg

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: TY_CAFTrpPPHL0K1_qX8HVUG1jp5wK-fLxD2epvQabqJfCbb-S7p4_NfOCLkXkLno8UzAgDug-V96xylgatl8w
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}
   📋 Email: {'value': 'alice@example.com', 'type': 'String'}
   📋 Status: {'value': 'pending', 'type': 'String'}
   📋 Credits: {'value': 0, 'type': 'Integer'}

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: wpNJ7OgiPJurLNWgzjLNVk_oaexCD-CejcSyLJu_psNQP2FYoN2xqKgbL_nYdyCQVuVeNS_kY93uZZgFFJCJGQ

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
   ✅ Script saved: -jYYKd9tEiCSScS76dBUtucTfZtKXn17i84zN-G-2kJc9CQ5gDk1uCACRKRrkCT4r4v7mhEMA3W2HvY1hZC1fw

2️⃣ Calling Script (Query + Update Credits + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {'type': 'Integer', 'value': 0}
   📋 Status: {'value': 'active', 'type': 'String'}
   📋 Name: {'value': 'Alice Smith', 'type': 'String'}

============================================================
📝 Script 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: bLENLKTZasxhHnxcjdVyDyWAyD0Lz3rkMtpZJd2LSlyrMXSbFabf14R0dT0zJl7coGq_MULQ_19P6IYmE-tqlQ

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: Upg72njwfh2wie13rsU5...
   ✅ Deleted script: wpNJ7OgiPJurLNWgzjLN...
   ✅ Deleted script: -jYYKd9tEiCSScS76dBU...
   ✅ Deleted script: bLENLKTZasxhHnxcjdVy...
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
  Output: Document ID = 6v83fxyp818cACIFzlNoQ-EDdvzhQ-Eia6k1WY0csnmLV0TgkNA9o0VYC1O5to5i13VDd66b9gSxYDS3K7BFHw
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: find_by_id(6v83fxyp818cACIFzlNoQ-EDdvzhQ-Eia6k1WY0csnmLV0TgkNA9o0VYC1O5to5i13VDd66b9gSxYDS3K7BFHw)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: find_by_id(6v83fxyp818cACIFzlNoQ-EDdvzhQ-Eia6k1WY0csnmLV0TgkNA9o0VYC1O5to5i13VDd66b9gSxYDS3K7BFHw)
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
  Output: Document ID = Xp8O7hwbV1AYrR-Iea_l2m60WfutRHJPQeGSwCo0zSXHrSpSO05Ifst3eaZYRje1E3rYC9gRZMHXZY0d13khjg
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: find_by_id(Xp8O7hwbV1AYrR-Iea_l2m60WfutRHJPQeGSwCo0zSXHrSpSO05Ifst3eaZYRje1E3rYC9gRZMHXZY0d13khjg)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: find_by_id(Xp8O7hwbV1AYrR-Iea_l2m60WfutRHJPQeGSwCo0zSXHrSpSO05Ifst3eaZYRje1E3rYC9gRZMHXZY0d13khjg)
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
📦 Built wheel for abi3 Python ≥ 3.8 to /Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.9.0-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel for current platform...[0m
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.9.0-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.9.0
    Uninstalling ekodb_client-0.9.0:
      Successfully uninstalled ekodb_client-0.9.0
Successfully installed ekodb-client-0.9.0
✅ [32mPython client package built and installed![0m
🧪 [36mRunning Python client library examples...[0m
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {'id': 'uWEU98EpP2th3p_XiyBuB4lHiEzzqBsxmVk8SjJm99SpXQ6NWClL3yfq1aNEWp9SXiiHI6oWFyoHjgxWw8KVeg'}

=== Find by ID ===
Found: {'value': 42, 'data': 'aGVsbG8gd29ybGQ=', 'metadata': {'key': 'value', 'nested': {'deep': True}}, 'id': 'uWEU98EpP2th3p_XiyBuB4lHiEzzqBsxmVk8SjJm99SpXQ6NWClL3yfq1aNEWp9SXiiHI6oWFyoHjgxWw8KVeg', 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5], 'user_id': '550e8400-e29b-41d4-a716-446655440000', 'categories': ['electronics', 'computers'], 'tags': ['tag1', 'tag2', 'tag3'], 'price': 99.99, 'created_at': '2026-01-27T12:13:12.667392', 'active': True, 'name': 'Test Record'}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): True
  price (Decimal): 99.99
  created_at (DateTime): 2026-01-27 12:13:12.667392
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): ['tag1', 'tag2', 'tag3']
  metadata (Object): {'key': 'value', 'nested': {'deep': True}}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): ['electronics', 'computers']
  data (Bytes): 11 bytes
Plain record: {'value': 42, 'data': 'aGVsbG8gd29ybGQ=', 'metadata': {'key': 'value', 'nested': {'deep': True}}, 'id': 'uWEU98EpP2th3p_XiyBuB4lHiEzzqBsxmVk8SjJm99SpXQ6NWClL3yfq1aNEWp9SXiiHI6oWFyoHjgxWw8KVeg', 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5], 'user_id': '550e8400-e29b-41d4-a716-446655440000', 'categories': ['electronics', 'computers'], 'tags': ['tag1', 'tag2', 'tag3'], 'price': 99.99, 'created_at': '2026-01-27T12:13:12.667392', 'active': True, 'name': 'Test Record'}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'name': {'type': 'String', 'value': 'Updated Record'}, 'active': {'value': True, 'type': 'Boolean'}, 'id': 'uWEU98EpP2th3p_XiyBuB4lHiEzzqBsxmVk8SjJm99SpXQ6NWClL3yfq1aNEWp9SXiiHI6oWFyoHjgxWw8KVeg', 'categories': {'type': 'Array', 'value': ['electronics', 'computers']}, 'data': {'type': 'String', 'value': 'aGVsbG8gd29ybGQ='}, 'metadata': {'type': 'Object', 'value': {'nested': {'deep': True}, 'key': 'value'}}, 'embedding': {'value': [0.1, 0.2, 0.3, 0.4, 0.5], 'type': 'Array'}, 'tags': {'type': 'Array', 'value': ['tag1', 'tag2', 'tag3']}, 'price': {'type': 'Float', 'value': 99.99}, 'user_id': {'type': 'String', 'value': '550e8400-e29b-41d4-a716-446655440000'}, 'created_at': {'value': '2026-01-27T12:13:12.667392', 'type': 'String'}, 'value': {'type': 'Integer', 'value': 100}}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: rQzVryRzkG9Tb2EQYbG4MTPUMFYnntzmZw1FKO4eW-rV0Ja73WsH2asRR8q4M-yvfKcxJX4LyN_kdTzsJqwqwA

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
Collection created with first record: "-uZAkaeqR_CsVGAf2zUpdPVk5FmT3AZ47DuT9NrBdNqVe91Q6a9CJF8mr1KmmV2woezh4WYaqvLf3dVrtgJd1w"

=== List Collections ===
Total collections: 11
Sample collections: ['chat_configurations__ek0_testing', 'client_collection_management_python', 'test_accounts', 'batch_users', 'products']

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
Created Alice: $1000 - ID: nbv0-GmB83mItIAemiL_8I2SfPvkjAadCSqVaLMVOciBVwABXtP9E0GGp6N5nlJoFk5cZyULemwHq3fOAC4pIQ
Created Bob: $500 - ID: dwMtsajS-csdKSBwvyjpmPqVkIKwx005vOfszHyjNyZlc-LCjpeVIZPb-c3_cBCYeazNkxjGGlXiLHGeE9Z6kA

=== Example 1: Begin Transaction ===
Transaction ID: 32c75922-448e-46c2-bc70-f62a683541b1

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 3f9f100e-6746-46d4-a0e5-49f1177b8e4f
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
  1. Score: 25.740, Matched: name, name.value, email, email.value
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title.value, bio, bio.value, title
  2. Score: 26.400, Matched: title.value, title, bio, bio.value
  3. Score: 26.400, Matched: title.value, bio, bio.value, title
  4. Score: 26.400, Matched: bio, title, title.value, bio.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title.value, bio, title, bio.value
  2. Score: 39.600, Matched: title.value, title, bio.value, bio
  3. Score: 39.600, Matched: title, title.value, bio.value, bio
  4. Score: 39.600, Matched: bio, bio.value, title.value, title

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 2 results (matches: run, running, runs)
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.746
  2. Score: 0.739
  3. Score: 0.736

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.373, Matched: content.value, content, title.value, title
  2. Score: 1.369, Matched: title.value, content, title, content.value
  3. Score: 0.368, Matched: 

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
  - Mouse ($25) by Alice Johnson
  - Laptop ($1200) by Alice Johnson

5. Complex join with multiple conditions:
Found 2 users with example.com emails:
  - Alice Johnson (alice@example.com): Building A
  - Bob Smith (bob@example.com): Building B

=== Cleanup ===
✅ Deleted test collections

✅ Join operations examples completed!
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: 4geZlFIOb2RYcDQaNgAS_WNtAsOCGyt10ZbbHZOKt9iY15zIRU-kR0qioQ_YYphWP3d-FtZ-qEiz-yNHnJTU8g

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: VLBBPC_lYrrpoyA98ORM3e-G4hdymW7Knr7TnI7ab-Zy7n2EM9AMAULb0dAH1e4WWh_BYSmgRRS2iJUMVegoBA

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
✓ Inserted document with TTL: wmCYO5hzvqgit2xgfQldPWWq2PHMufcqlgKCzONeGZclh-8AqnjNT6n7i58u3gq1IvX11bIUt8e1JfekKZtSRg

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
✓ Edge cache script created: vliMbRpmE7xse-M98tLrQWXOZcouO7ho4cCiq5SvAsZWfAvsWrjChS0_pPtnTBNWXnYrlVdRvaFKuJJKStclMA

Call 1: Cache miss (fetches from API)
Response time: 550ms
Result: {
  "records": [
    {
      "value": {
        "current": {
          "interval": 900,
          "temperature_2m": -6.7,
          "time": "2026-01-27T17:00"
        },
        "current_units": {
          "interval": "seconds",
          "temperature_2m": "\u00b0C",
          "time": "iso8601"
        },
        "elevation": 32.0,
        "generationtime_ms": 0.02646446228027344,
        "latitude": 40.710335,
        "longitude": -73.99308,
        "timezone": "GMT",
        "timezone_abbreviation": "GMT",
        "utc_offset_seconds": 0
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

Call 2: Cache hit (served from ekoDB)
Response time: 8ms (69.8x faster!)
Result: {
  "records": [
    {
      "value": {
        "current": {
          "interval": 900,
          "temperature_2m": -6.7,
          "time": "2026-01-27T17:00"
        },
        "current_units": {
          "interval": "seconds",
          "temperature_2m": "\u00b0C",
          "time": "iso8601"
        },
        "elevation": 32.0,
        "generationtime_ms": 0.02646446228027344,
        "latitude": 40.710335,
        "longitude": -73.99308,
        "timezone": "GMT",
        "timezone_abbreviation": "GMT",
        "utc_offset_seconds": 0
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

✅ Script saved: Zu_5In1XQQgMxD6ZNu4tJRcR3J4jfulj5mLhGNqDJk8DW2sMt-CxloyiLG4KnkbPVjTQmH1VaybXdA8UOSqwGw
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved: nPJx1w1l6V9SthQ00fxDiUSaTx0DE34rggiufHim-I8hEgh_l0ECVSI4hxqkJjRF_MEU_c27FJLQWRdGBIQsZw
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: P1rdFuP02BRR19WmFNhbT46K0oMoGHoP7r317PvnSEoaBuwzYjrhePSU_nR_R1ovdNnZ1boGuXdSnoBoug1xwg
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
   ⏱️  Duration: 1.5ms
   📊 Records: 1
   📦 Data: {
      "value": {
            "address": {
                  "city": "Gwenborough",
                  "geo": {
                        "lat": "-37.3159",
                        "lng": "81.1496"
    ...
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
🚀 ekoDB Python Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved: 4uzlrGIgUjd084mUPWHR4pYLMY6FKex8ThLOT3KdRfSLMft3p4LiL4jKnQYy29g8YnSpIfTQ5bFpKm65J6L61Q
📊 Found 2 product groups
   {'avg_price': 575.6666666666666, 'category': 'Electronics', 'count': 3}
   {'avg_price': 474.0, 'category': 'Furniture', 'count': 2}
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

✅ Inserted order: q3TVdw4C9ARM2WSeDeNmtpUGjSZnObUqiOtSZKQPPil5trU4KiAeI0kpYolKeBFY62GLv3Zjaotk3Lr3-wad0Q
✅ Inserted 2 products with wrapped types

📝 Example 2: Querying and Extracting Wrapped Types

📊 Found 2 products
   • Laptop Pro
   • Wireless Mouse

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

✅ Inserted order: 8x8ycC5tTZOtJ8L_y5PCHHU22HQrpPhB0sGMZwMmo9oAAP_miKfGV98zNgTBSG6xtLj3bAa8Ci2rp5rAnzv9UA
✅ Cached order status
📊 Quick status lookup: {'value': '{"status":"processing","updated_at":"2026-01-27T17:13:15.099982+00:00"}'}

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
✓ Created SWR script: fetch_api_user_py (-90q8bDP9eg69_ezZjCXbVN3IVwyBqwwwSLnumxe1_hTF7hv9JMXRPa2ABOJh69Hsn36t-ukJOam8e1QWZcNXg)

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
   Vector databases provide benefits such as efficient data management, high precision, support for complex geometries, and better representation of data. They also allow for advanced spatial analysis and operations like buffering, overlay, and network analysis.
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
   1. Introduction to Machine Learning (AI)
   2. Vector Databases Explained (Database)
   3. Database Design Principles (Database)
   4. Natural Language Processing (AI)
   5. Getting Started with ekoDB (Database)
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
✓ Created session: wI1DCXeEsJdeUO_kt02FRPdNDwDBsstg9B7FWw2CKRTfOPdsGlg_28kcDF5_qt7kKB0d1QzG3ChFwb0qYkflbw

=== Sending Chat Message ===
Message ID: eu6ml_m3wvQlFLqfdIBSsHsPUNpHvrrfnFh4_QbZDSM7bSGcaCoSnrlJWVnFbf726WQx601Bt_Q8UASU9_d85g

=== AI Response ===
We have three products available:

1. ekoDB: This is a high-performance database product with AI capabilities. It is priced at $99.

2. ekoDB Pro: This is an Enterprise edition product with advanced features. It costs $299.

3. ekoDB Cloud: This is a fully managed cloud database service product. The price for this service is $499.

Execution Time: 3007ms

=== Token Usage ===
Prompt tokens: 454
Completion tokens: 78
Total tokens: 532

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: fB0OcnySBLkCHMXR2EULSWXBzNKJlIJ9faCu9Q8zOpE-RMyfmOYISovAyxuFNwj_FUyjSiMjusnMRyKTHqvngw

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information provided, there is a high-performance database product available named "ekoDB". It is priced at $99.

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['id', 'chat_id', 'context_snippets', 'content', 'role', 'updated_at', 'token_usage', 'created_at'])
Debug: First message role: {'type': 'String', 'value': 'assistant'}
=== Feature 1: Regenerate AI Response ===
✓ Message regenerated
  New response: The price of ekoDB is $99.

=== Feature 2: Edit Message ===
✓ Message content updated

=== Feature 3: Mark Message as Forgotten ===
✓ Message marked as forgotten (excluded from LLM context)

✓ Message unmarked as forgotten

=== Feature 4: Merge Chat Sessions ===
✓ Created second session: sAzMtmJ-PQw2AMEuPiO1cVavd-TPGz-NdoWzNaWrQwQC5YnvBxHARHZQzeL1qVMGkgyKyhSGl-4SFyQ4Yp8K6w
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
✓ Created session: p4qIdxyzvVRIUsqZEFi6SQma6mUQhvAEuyfhx3AGbTcVcV_R_ZrS7yko8TuJm_H1dQAdckXP1b4GW6NMpUte0A

=== Sending Messages ===
✓ Message 1 sent
  Response: The available product based on the provided context is a high-performance database product named "ekoDB". It is priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: GJ1ARwPMu_QT0nEgly5QyyjDbcc-2UuJXLtwzNeZX3uxYzApCK0xTA4XdXJbqowdQyOHW7B_d7MQc4mHH91VdA
  Parent: p4qIdxyzvVRIUsqZEFi6SQma6mUQhvAEuyfhx3AGbTcVcV_R_ZrS7yko8TuJm_H1dQAdckXP1b4GW6NMpUte0A

=== Listing Sessions ===
✓ Found 7 sessions
  Session 1: GJ1ARwPMu_QT0nEgly5QyyjDbcc-2UuJXLtwzNeZX3uxYzApCK0xTA4XdXJbqowdQyOHW7B_d7MQc4mHH91VdA (Untitled)
  Session 2: p4qIdxyzvVRIUsqZEFi6SQma6mUQhvAEuyfhx3AGbTcVcV_R_ZrS7yko8TuJm_H1dQAdckXP1b4GW6NMpUte0A (Untitled)
  Session 3: sAzMtmJ-PQw2AMEuPiO1cVavd-TPGz-NdoWzNaWrQwQC5YnvBxHARHZQzeL1qVMGkgyKyhSGl-4SFyQ4Yp8K6w (Untitled)
  Session 4: wI1DCXeEsJdeUO_kt02FRPdNDwDBsstg9B7FWw2CKRTfOPdsGlg_28kcDF5_qt7kKB0d1QzG3ChFwb0qYkflbw (Untitled)
  Session 5: -X3r6ftFEexwrKAvlzkZrkxeHh-JiNF87XeKMozCByA1f4epxPc0neizTsZUeZo--ymhrhF980pm_eMdeb0n3w (Untitled)
  Session 6: MFbKEwGka_F5zUqzfAjBoCmSLbdAGQnNUFluQGZQBVOpiWcYKxRFe4Oy3bwrvytTuJMcL4AkRMra-kseAeNFWA (Untitled)
  Session 7: O5WSPWnxYUn00jIQyOvwuAX84ZXyGYnF_HOnwe0EBVt-TTf9oMr7iziaWfFbk57WXUb3Xd3_RSNC0h1XSgmiSg (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: GJ1ARwPMu_QT0nEgly5QyyjDbcc-2UuJXLtwzNeZX3uxYzApCK0xTA4XdXJbqowdQyOHW7B_d7MQc4mHH91VdA

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Dict Creation ===
✓ Created record with plain dict: {'id': 'dQUCCI-U44P6uPArSYQ_uXQEPw9WV2ExOG4W_sONSowQg0RglX4IZzBmH_lnUV-vPVqKjw7eSaXqUIHfNqMWgg'}

=== Upsert Operation ===
✓ Upsert (update existing record): dQUCCI-U44P6uPArSYQ_uXQEPw9WV2ExOG4W_sONSowQg0RglX4IZzBmH_lnUV-vPVqKjw7eSaXqUIHfNqMWgg
✓ Inserted second record: xrk1BD75sCISZEfZ8V5TBklc1kEICSQEZfmmx6_haF3uxoJLAvzPh7794u98bIDEqYPGQP26-yuF4MAa0C65jQ
✓ Upsert (update second record): xrk1BD75sCISZEfZ8V5TBklc1kEICSQEZfmmx6_haF3uxoJLAvzPh7794u98bIDEqYPGQP26-yuF4MAa0C65jQ

=== Find One Operation ===
✓ Found user by email: {'name': {'type': 'String', 'value': 'Alice Johnson'}, 'age': {'value': 29, 'type': 'Integer'}, 'active': {'type': 'Boolean', 'value': True}, 'id': 'dQUCCI-U44P6uPArSYQ_uXQEPw9WV2ExOG4W_sONSowQg0RglX4IZzBmH_lnUV-vPVqKjw7eSaXqUIHfNqMWgg', 'email': {'type': 'String', 'value': 'alice.j@newdomain.com'}}
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
   Inserted with ripple: {'id': 'jZ6tjMlODoGh_3z0RBodlHSIoa0XwIq_yYNU6RCFSVj39Qw4Ip5ybezv-gVG9Yorz-KhTY4lI7QzLMXzL88WCg'}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {'id': 'or4A8KvvvDeFL7pxh2Z__Vg5GIzlNrWwt892Q0O49kUAYb83UKSV2Fg1lnjVrCFGBKW4YU38f8YNYKGR6z-Elw'}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {'name': {'type': 'String', 'value': 'Product 1'}, 'price': {'value': 150, 'type': 'Integer'}, 'id': 'jZ6tjMlODoGh_3z0RBodlHSIoa0XwIq_yYNU6RCFSVj39Qw4Ip5ybezv-gVG9Yorz-KhTY4lI7QzLMXzL88WCg'}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {'price': {'value': 500, 'type': 'Integer'}, 'name': {'type': 'String', 'value': 'Upsert Product'}, 'id': 'jZ6tjMlODoGh_3z0RBodlHSIoa0XwIq_yYNU6RCFSVj39Qw4Ip5ybezv-gVG9Yorz-KhTY4lI7QzLMXzL88WCg'}

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
Inserted: map[id:1SPzmvmEzruTUag-Gz4QadtVcm89Mz_9RbpjtIFnF7ckTuCT8P_J4FYwvpa8nCecVDqAxYJ5tNyjRYApNtx0uQ]

=== Find by ID ===
Found: map[active:true id:1SPzmvmEzruTUag-Gz4QadtVcm89Mz_9RbpjtIFnF7ckTuCT8P_J4FYwvpa8nCecVDqAxYJ5tNyjRYApNtx0uQ name:Test Record value:42]

=== Find with Query ===
Found documents: [map[active:map[type:Boolean value:true] id:1SPzmvmEzruTUag-Gz4QadtVcm89Mz_9RbpjtIFnF7ckTuCT8P_J4FYwvpa8nCecVDqAxYJ5tNyjRYApNtx0uQ name:map[type:String value:Test Record] value:map[type:Integer value:42]]]

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] id:1SPzmvmEzruTUag-Gz4QadtVcm89Mz_9RbpjtIFnF7ckTuCT8P_J4FYwvpa8nCecVDqAxYJ5tNyjRYApNtx0uQ name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
[32m✓ simple_crud.go completed successfully[0m
[34m
=== Running simple_websocket.go ===[0m
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: nNbqNp0PZJeu64PMuqIrmHqPklGfWntWmBXjK_lKGx_W3ZKFCFjRngAoKH6Qz1MavLhGdZhaGsW2vaME1-AoBw

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
      },
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "nNbqNp0PZJeu64PMuqIrmHqPklGfWntWmBXjK_lKGx_W3ZKFCFjRngAoKH6Qz1MavLhGdZhaGsW2vaME1-AoBw",
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
        "id": "S-g_KsEmXxeRR7SufzogTElDqP-F9m9IRs05FO8UL78xYZaP_aAkSbHUzqAbaHHS4eeylQKCp90CHoPUwFFtkQ",
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

✅ Script saved: 3DOZvAM75Pa_3YwOElUlx-kGyKGlSatJxSXTC8liqu76uk5hmNxufuNoHrenAVKERKCx6KS97RCP-VYt6AArzw
📊 Found 10 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: p3JiUY7IAmp1SyGvC1WVL54nxQkhpluyZoFdf8CpYAsurW1OrUkD8c8ESXZKyeeH2UcF438ih6ZruFpIEj4QHQ
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: ktZwDUB59VLQWb4fnIuU8OUgGOnqDsOTPu7Ft_A6BNKKNEfhLKdhyriIZlFNj2nihsJPcDb7AgUypw94TKzbpQ
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
Collection created with first record: 5i638lJIFPx6oxWrkGdJcXxeghVSxD1qoBDxu4HPkBvgle9p4vYcsa81u3bP-533iEt2i-z2HEbwoqM8CkKhFw

=== List Collections ===
Total collections: 15
Sample collections: [chat_configurations__ek0_testing test_accounts batch_users demo_collection schema_users_client_py]

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
Created Alice: $1000 - ID: 6LlGDD4lonhjCHTYoPg3oeXE6FvhGVzTie_GsuYRTqV_kt2G6A4lCx2xd4CZTEFSxcj53Gz17AumwCPxwwn_hw
Created Bob: $500 - ID: gHOBa6YaFy1Yj1bdEE8UMWoT6E6dzxYg6j2h6u1B09-ciYZdSHobO9ZTBAesvN8uakKv9_htNwsKycQ3NHbAng

=== Example 1: Begin Transaction ===
Transaction ID: 545ccd18-0e81-41d8-b350-08f16a709f77

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
New transaction: 4a0166a8-522f-4d08-a666-5ebec4ab1564
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
   ✅ Script saved: CfLaYYX3Zt-1LpjtNROcvlmHtHGjkxE2kcjAa604nzGI2Uzsl4pluyY2nt2nt18oPNjyLhadayMEHuvQuwZjsg

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: jjMQer-jE_v8khn9SXS6ymsggAtm9Sr4eH_VTO3K1JcKzd1XH8rblDbHxhRA6jAjN7hQUhO8fDJJIn5RoeNaiQ
   📋 Name: map[type:String value:Alice Smith]
   📋 Email: map[type:String value:alice@example.com]
   📋 Status: map[type:String value:pending]
   📋 Credits: map[type:Integer value:0]

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: NLXyAYinipuOHcei0mXsMpmUrhsUTvoiEh7BACixFeFFh4rgn5V_Ua_Rpuc-D1F02N71hCT8EQb-Nh_DYiG-Zg

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
   ✅ Script saved: XQqDlWB1WuSMM9W9aXU8r9xFqc6rVTJPseBi1LQQdyNSHPXLZIi1_d4sbzKpYjY6SrnurTmrPehMmf3yi70ErA

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
   ✅ Script saved: wirTNAoPOMVpI_peeL4HUe6kn5UValIe2jDbzJTUZRwMBwklFleZWPqPhuY6aUd2u7H0K4PeexmTU8e90_UTuA

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: CfLaYYX3Zt-1LpjtNROc...
   ✅ Deleted script: NLXyAYinipuOHcei0mXs...
   ✅ Deleted script: XQqDlWB1WuSMM9W9aXU8...
   ✅ Deleted script: wirTNAoPOMVpI_peeL4H...
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

2026/01/27 12:13:46 No .env file found, using defaults
✓ Client connected

═══════════════════════════════════════════════════════════
TEST 1: Document TTL Expiration
═══════════════════════════════════════════════════════════

[Step 1] Insert document with 3 second TTL
  Input: {name: 'TTL Test', value: 'should expire'}
  TTL: 3s
  Output: Document ID = epOzbUXtrEAfT1dxBNNWFDN-R_yYYqXpPi_zQaT_I-QpXa5kW9j4rPZfpue1CP3RNAqIuPHI6nCiwCp3gG0RsA
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: FindByID(epOzbUXtrEAfT1dxBNNWFDN-R_yYYqXpPi_zQaT_I-QpXa5kW9j4rPZfpue1CP3RNAqIuPHI6nCiwCp3gG0RsA)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: FindByID(epOzbUXtrEAfT1dxBNNWFDN-R_yYYqXpPi_zQaT_I-QpXa5kW9j4rPZfpue1CP3RNAqIuPHI6nCiwCp3gG0RsA)
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

2026/01/27 12:13:53 No .env file found, using defaults
✓ Client connected

═══════════════════════════════════════════════════════════
TEST: WebSocket TTL Expiration
═══════════════════════════════════════════════════════════

[Step 1] Insert document with 3 second TTL
  Input: {name: 'WS TTL Test', value: 'should expire'}
  TTL: 3s
  Output: Document ID = SwYk2yP4BIAY4yLurdrS8I5oOmDIVfMGLtKx44CNyW7Ww5okiT0a0l3Z-t9ifRdK2qjIBKU-l2d3YOp9zcZDpw
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: FindByID(SwYk2yP4BIAY4yLurdrS8I5oOmDIVfMGLtKx44CNyW7Ww5okiT0a0l3Z-t9ifRdK2qjIBKU-l2d3YOp9zcZDpw)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: FindByID(SwYk2yP4BIAY4yLurdrS8I5oOmDIVfMGLtKx44CNyW7Ww5okiT0a0l3Z-t9ifRdK2qjIBKU-l2d3YOp9zcZDpw)
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
Inserted: map[id:ummU5LKCDgMgpl7w4IYOFSqYVfZ8Enz92CFPZRvFWhO3tvXhdSQ8KiX63UBXTQJK2hIa0yefuEl44raeHaQxyQ]

=== Find by ID ===
Found: map[active:true categories:[electronics computers] created_at:2026-01-27T12:14:00-05:00 data:aGVsbG8gd29ybGQ= embedding:[0.1 0.2 0.3 0.4 0.5] id:ummU5LKCDgMgpl7w4IYOFSqYVfZ8Enz92CFPZRvFWhO3tvXhdSQ8KiX63UBXTQJK2hIa0yefuEl44raeHaQxyQ metadata:map[key:value nested:map[deep:true]] name:Test Record price:99.99 tags:[tag1 tag2 tag3] user_id:550e8400-e29b-41d4-a716-446655440000 value:42]

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 0
  active (Boolean): true
  price (Decimal): 99.990000
  created_at (DateTime): 2026-01-27 12:14:00 -0500 EST
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1 tag2 tag3]
  metadata (Object): map[key:value nested:map[deep:true]]
  embedding (Vector): [0.1 0.2 0.3 0.4 0.5]
  categories (Set): [electronics computers]
  data (Bytes): 11 bytes
Plain record: map[active:true categories:[electronics computers] created_at:2026-01-27T12:14:00-05:00 data:aGVsbG8gd29ybGQ= embedding:[0.1 0.2 0.3 0.4 0.5] id:ummU5LKCDgMgpl7w4IYOFSqYVfZ8Enz92CFPZRvFWhO3tvXhdSQ8KiX63UBXTQJK2hIa0yefuEl44raeHaQxyQ metadata:map[key:value nested:map[deep:true]] name:Test Record price:99.99 tags:[tag1 tag2 tag3] user_id:550e8400-e29b-41d4-a716-446655440000 value:42]

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-01-27T17:14:00Z] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:ummU5LKCDgMgpl7w4IYOFSqYVfZ8Enz92CFPZRvFWhO3tvXhdSQ8KiX63UBXTQJK2hIa0yefuEl44raeHaQxyQ metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Updated Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: kVovMcG-p-oppR2L6w6IZ8XMSUm-NwUVfFOYfexnQwnrhbeCRWCDG8UVYa-M3LKg_kcqddGm3PT9gu9YtCSBLA

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
Collection created with first record: 98WkOmhnrgKXtNiJJ4oZOj5KzJOslIfxkmkUpOk3iaqUsqxN_-QgdvidH79s2j4s3ICPGnshfJnLfS9UGbzKww

=== List Collections ===
Total collections: 14
Sample collections: [chat_configurations__ek0_testing test_accounts client_collection_management_go batch_users schema_users_client_py]

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
Total keys in store: 11

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
Created Alice: $1000 - ID: igChHyeFFVaRYwug9R18axCNQ4WbID7gfZMcNxf6HfyOoZopu__DSE3rePE9_pTp3_jqzlpqAFhMsDS0zjl9sg
Created Bob: $500 - ID: kyFsy2wvNL-hxy30Wfhe7VaVSJn5zUv_jppZ_qC_NrNNZp7kWpAAqQUzVU55QWBhhkqmdTZA9tx2RJnIaECQmQ

=== Example 1: Begin Transaction ===
Transaction ID: 83c3c198-e961-4071-8350-d8d32881544b

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 20f5fbc7-9287-47c0-87a5-86b2fa9215da
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
  1. Score: 0.740
  2. Score: 0.735
  3. Score: 0.734

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.694
  2. Score: 1.494
  3. Score: 0.296

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
✓ Inserted document: bdYPai_7UWmRTK2w5Y6Oc0M9A0DdVAIzJeAeWzZSvWBImVIe8HpFQ4Vh8mLIY8AiebsDvNFl72lLoUfF_zOm0w

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: aFF-w6FXHPzErMMEKoHdzTiciCxFxSuM5qRT1r7rJ-I93GYkYSIYbwCUFMg_cpA57BFkhLh1Ol88Lb1xBSzhoA

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
✓ Inserted document with TTL: oiwaL16RXa_QPEOnsBzKctPZ1JzXc_cl6-C16XzpOi4F58lAW8HLq-g0mVnbvD9Nsrt6UxYb1HroWijYDAX1uw

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
✓ Edge cache script created: Xc4_knI33YfIKGAMdy7v9MFpSkepviVCR0cOwdtV1LlTmNBbNeAj0qHd1T3Af-Vm9Khv5E1jr3UhsHaAQlTf3g

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

✅ Script saved: 9dOcO0mGfws1ZzPL7oEhDFVdnALbgd2wYgEn55U9vvIsNSUv8WUiBywOE6RXRwHGFjYxAdbJV8DJ6x32dJwlsA
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
   ⏱️  Duration: 2.529583ms
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
   ⏱️  Duration: 2.56275ms
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

✅ Script saved: u90wEdYcBiZalZJ_3GEOlQ9hKFWGcDp45eaqcqWVhwh0U5aEcOnxDYlA_Orr_OCPOKtc3hUHKDLn2QzkeRCHGg
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

✅ Inserted order: gGID_rT4HSkta9GIAILksWDl4x_AaUXxVT_t5aPGxbsG1J-c0ZaIGfCc-YiHALBlO23XveaKnnwh5ZFum5AVGA
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: L6DIV0cPSmijCjVTgilz_ymw56_yGruHB583OAk8R9t8OM1OBHpT7AAr1IKp4iiKnwBmzYeDuODMlGGyZi_eKg
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: map[role:admin userId:user_abc]
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session
📝 Example 4: KV Operations in Scripts

✅ Script saved: yO7v6afzHhU5Y2Gy18rqguh0VTm02lxFBAZ_wQDQrWzGNIwOqm7xaO4J2w_0tiseh0RohpyoRUqbauJZaQqg4A
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Script

✅ Script saved: MDrPfBKAWzwroU5HzPxae3kuWCNVX83UqMW5J78KXkcmPYZpKwLzv4b_G7RkNxwJlvb35LEema1UXRuBYV53lg
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
✓ Created SWR script: fetch_api_user_go (nzLdpPBvyHXOOYgWqb94WT1HnjEkNuWvKJHTP6aDucNxCjhquuGxg4pPQLJt6oZ8VmN7AXGoYKr11jqN0nW66A)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "cached_at": "2026-01-27T12:14:03-05:00",
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
      "id": "vpLlOtsMmZVwh6VZ0lE7IDfRwE-gIRdnyPYCZ64R4DYEdKCz-6yECIaERHAue100B5XOV78zSvQXfOEyVOcXFQ"
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
Response time: 69ms (served from cache)
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
   map[avg_price:365.6666666666667 category:Furniture count:3]
   map[avg_price:367 category:Electronics count:5]
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
   1. map[type:String value:Vector Databases Explained] (map[type:String value:Database])
   2. map[type:String value:Database Design Principles] (map[type:String value:Database])
   3. map[type:String value:Introduction to Machine Learning] (map[type:String value:AI])
   4. map[type:String value:Natural Language Processing] (map[type:String value:AI])
   5. map[type:String value:Getting Started with ekoDB] (map[type:String value:Database])
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
✓ Created session: H5TOVzzNxakRARI8q0N8aI6MXf9khSLO60NUFeIYlRi4rNsHQf9Ft3xqdUkG0HrNylE4YlWGD3w9dHAkqNo0Ig

=== Sending Chat Message ===
Message ID: XqC4fLEgOVhXr6NU-O9h31p41jdZchHhAL5f1K6ZomvpD8-7YrzHta4gNK57Lxgq2nMuuvsUyPQA3tU6a7Glsg

=== AI Response ===
There are three products available:

1. ekoDB Pro: This is an Enterprise edition product with advanced features. It is priced at $299.

2. ekoDB: This is a high-performance database product with AI capabilities. It is priced at $99.

3. ekoDB Cloud: This is a fully managed cloud database service product. It is priced at $499.

=== Context Used (3 snippets) ===
  Snippet 1: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Enterprise edition product with advanced features id:B7WzqLgE1h3OhaBOIV3FkGGV88aGicI1UPr4TDCOYmi-KVvfK3LKI0-JhoBVaRzXOjHmJ9D-zMCWnBDH1ky7BA name:ekoDB Pro price:299] score:0.1111111111111111]
  Snippet 2: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:A high-performance database product with AI capabilities id:kXOSGaVnZTkSDoB9tqr-jCNQq9qU9C0q8uGGWcHIdh1NpO_RSc6aXq44wLkm9E5p6BOdbrc0uQi27e4DBLQtsg name:ekoDB price:99] score:0.1111111111111111]
  Snippet 3: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Fully managed cloud database service product id:fJx5_OzOqFzqWoRoz6mOboyK1BGoDEOJlubWsONjlGa_AJL0gOQmGxPscT2WlUolTzsFta6dEfl6JRGujN4oyw name:ekoDB Cloud price:499] score:0.1111111111111111]

Execution Time: 3734ms

=== Token Usage ===
Prompt tokens: 463
Completion tokens: 78
Total tokens: 541

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: cGIBO-Oliy1lNywx4nmtO2uGn8-kIrRosNIF4_SjY_uJZIbTqhQT6013xQ_r4a8IPooUFX_IKv_UaYDT0NBbAA

=== Sending Initial Message ===
✓ Message sent
  Response: The available product based on the context provided is "ekoDB". It's a high-performance database product priced at $99.

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
✓ Created second session: vCj_jR9APFVECAEupGE-eShauY1CU4zi6rs881YufAEuruBTM5UiMLTrGofiOuPSc5t7QN0jtSJMOoTnG5FXbw
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
✓ Created session: -x18cYo6AhFsWkJX4FyQFZUmxRdlu3Y5twqrLatXf6lKNHYH1n0a78OTtK_NtEP8P6K8SOBqyWoId_5FT-fAJQ

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information available, the product that is available is "ekoDB". It is a high-performance database product priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB high-performance database product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: UnIEw6fAAanw8m8sPwlTwfU-ribmd8CXmFiD8n5-KEJMRTZ4UHgk2SSHdVFgFxaDjJuzfQuAq_-Ub9suoX9XBw
  Parent: -x18cYo6AhFsWkJX4FyQFZUmxRdlu3Y5twqrLatXf6lKNHYH1n0a78OTtK_NtEP8P6K8SOBqyWoId_5FT-fAJQ

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: UnIEw6fAAanw8m8sPwlTwfU-ribmd8CXmFiD8n5-KEJMRTZ4UHgk2SSHdVFgFxaDjJuzfQuAq_-Ub9suoX9XBw (Untitled)
  Session 2: -x18cYo6AhFsWkJX4FyQFZUmxRdlu3Y5twqrLatXf6lKNHYH1n0a78OTtK_NtEP8P6K8SOBqyWoId_5FT-fAJQ (Untitled)
  Session 3: vCj_jR9APFVECAEupGE-eShauY1CU4zi6rs881YufAEuruBTM5UiMLTrGofiOuPSc5t7QN0jtSJMOoTnG5FXbw (Untitled)
  Session 4: H5TOVzzNxakRARI8q0N8aI6MXf9khSLO60NUFeIYlRi4rNsHQf9Ft3xqdUkG0HrNylE4YlWGD3w9dHAkqNo0Ig (Untitled)
  Session 5: p4qIdxyzvVRIUsqZEFi6SQma6mUQhvAEuyfhx3AGbTcVcV_R_ZrS7yko8TuJm_H1dQAdckXP1b4GW6NMpUte0A (Untitled)
  Session 6: sAzMtmJ-PQw2AMEuPiO1cVavd-TPGz-NdoWzNaWrQwQC5YnvBxHARHZQzeL1qVMGkgyKyhSGl-4SFyQ4Yp8K6w (Untitled)
  Session 7: wI1DCXeEsJdeUO_kt02FRPdNDwDBsstg9B7FWw2CKRTfOPdsGlg_28kcDF5_qt7kKB0d1QzG3ChFwb0qYkflbw (Untitled)
  Session 8: -X3r6ftFEexwrKAvlzkZrkxeHh-JiNF87XeKMozCByA1f4epxPc0neizTsZUeZo--ymhrhF980pm_eMdeb0n3w (Untitled)
  Session 9: MFbKEwGka_F5zUqzfAjBoCmSLbdAGQnNUFluQGZQBVOpiWcYKxRFe4Oy3bwrvytTuJMcL4AkRMra-kseAeNFWA (Untitled)
  Session 10: O5WSPWnxYUn00jIQyOvwuAX84ZXyGYnF_HOnwe0EBVt-TTf9oMr7iziaWfFbk57WXUb3Xd3_RSNC0h1XSgmiSg (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: UnIEw6fAAanw8m8sPwlTwfU-ribmd8CXmFiD8n5-KEJMRTZ4UHgk2SSHdVFgFxaDjJuzfQuAq_-Ub9suoX9XBw

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Map Creation ===
✓ Created record with native map: map[id:aNjMJ1beQwQ7m1Y3GuYe6rKhLblDIU7ff__NkIRbtPdppUyuarqfeF1DZtyBEOy5Rr2T4-ze8Wa2R3PeJgFcWg]

=== Upsert Operation ===
✓ First upsert (update): map[active:map[type:Boolean value:true] age:map[type:Integer value:29] email:map[type:String value:alice.j@newdomain.com] id:aNjMJ1beQwQ7m1Y3GuYe6rKhLblDIU7ff__NkIRbtPdppUyuarqfeF1DZtyBEOy5Rr2T4-ze8Wa2R3PeJgFcWg name:map[type:String value:Alice Johnson]]
✓ Second upsert (insert): map[id:zjP4jpEI9qi0V8dUNitVqeCkd_oZ3nzPbcyn5tXflIS3qLOv0JEDpeoyo4kngdsp_Aa-TtkhNQJG36slzPO3yA]

=== Find One Operation ===
✓ Found user by email: map[active:map[type:Boolean value:true] age:map[type:Integer value:29] email:map[type:String value:alice.j@newdomain.com] id:aNjMJ1beQwQ7m1Y3GuYe6rKhLblDIU7ff__NkIRbtPdppUyuarqfeF1DZtyBEOy5Rr2T4-ze8Wa2R3PeJgFcWg name:map[type:String value:Alice Johnson]]
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
Inserted with ripple: map[id:kaBrezMLia5UfsuGuT3AO8og_KkYqaivzfOl7WV0hJ3KFhrDAZQLHOotzTlsHYWcwsXQTQKyzA93ybMjzD8cIQ]
Inserted with bypass_ripple: map[id:NDQtdw8dFb67SyXs_3zQHMy8AEOrnZASZUlkD9hAjXxqY2Bfhln2BLSINs_JgSfYu2VfAlzENEYwuzRzZag_eg]
Inserted with TTL and bypass_ripple: map[id:gm-FmWhl8ocvzFM6ltYNAMIeh8CSbAopI4nZ4hUbmF0MAHrILtg-XXPnEp94qJJ1S07uH9gh2Al1je9tQp3u4A]
Updated with bypass_ripple: map[id:kaBrezMLia5UfsuGuT3AO8og_KkYqaivzfOl7WV0hJ3KFhrDAZQLHOotzTlsHYWcwsXQTQKyzA93ybMjzD8cIQ name:map[type:String value:Product 1] price:map[type:Integer value:150]]
Deleted with bypass_ripple
Batch inserted with bypass_ripple: 2 records
Upserted with bypass_ripple: map[id:39xtAUWA_RvrbYFbJ0_g9DGE-0WuzE9hfQjYguLlNoUkHDAUzkrQBQ9nWL0LyOxGeO5R8yK8mYVUJqoF5qd4Hw]
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

> @ekodb/ekodb-client@0.9.0 prepare
> npm run build


> @ekodb/ekodb-client@0.9.0 build
> tsc


up to date, audited 58 packages in 1s

15 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.9.0 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning TypeScript client library examples...[0m

added 1 package, removed 1 package, and audited 13 packages in 840ms

3 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'woVEKnrSNiHANAsmFzNKaMhxtswNWuaEBN3lk8PrByfELnMOWSplLgGUZYId3KJdkLQElipOGrN5aPg_jZ9vfA'
}

=== Find by ID ===
Found: {
  name: 'Test Record',
  categories: [ 'electronics', 'computers' ],
  created_at: '2026-01-27T17:14:22.409Z',
  metadata: { nested: { deep: true }, key: 'value' },
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  tags: [ 'tag1', 'tag2', 'tag3' ],
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  data: 'aGVsbG8gd29ybGQ=',
  value: 42,
  active: true,
  price: 99.99,
  id: 'woVEKnrSNiHANAsmFzNKaMhxtswNWuaEBN3lk8PrByfELnMOWSplLgGUZYId3KJdkLQElipOGrN5aPg_jZ9vfA'
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-01-27T17:14:22.409Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { nested: { deep: true }, key: 'value' }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  name: 'Test Record',
  categories: [ 'electronics', 'computers' ],
  created_at: '2026-01-27T17:14:22.409Z',
  metadata: { nested: { deep: true }, key: 'value' },
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  tags: [ 'tag1', 'tag2', 'tag3' ],
  user_id: '550e8400-e29b-41d4-a716-446655440000',
  data: 'aGVsbG8gd29ybGQ=',
  value: 42,
  active: true,
  price: 99.99,
  id: 'woVEKnrSNiHANAsmFzNKaMhxtswNWuaEBN3lk8PrByfELnMOWSplLgGUZYId3KJdkLQElipOGrN5aPg_jZ9vfA'
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  embedding: { value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ], type: 'Array' },
  active: { type: 'Boolean', value: true },
  categories: { value: [ 'electronics', 'computers' ], type: 'Array' },
  price: { type: 'Float', value: 99.99 },
  tags: { value: [ 'tag1', 'tag2', 'tag3' ], type: 'Array' },
  data: { value: 'aGVsbG8gd29ybGQ=', type: 'String' },
  id: 'woVEKnrSNiHANAsmFzNKaMhxtswNWuaEBN3lk8PrByfELnMOWSplLgGUZYId3KJdkLQElipOGrN5aPg_jZ9vfA',
  value: { type: 'Integer', value: 100 },
  name: { type: 'String', value: 'Updated Record' },
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' },
  created_at: { value: '2026-01-27T17:14:22.409Z', type: 'DateTime' },
  metadata: { value: { nested: [Object], key: 'value' }, type: 'Object' }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: rUYY3YbJh5DHYmGkAObAwF8EKY0f62NaTIEd7kiZ9T1b5sVarbH39_19qaerOeVpSU3lzO9Fwb7z0xAzHfKkKg

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
Collection created with first record: iLEsdH8y_67WF-uJ278dFKk7gCJxwSNatm-5KAncVMYHEtKVJ7PAu2oofUWsGnpbxTx7vV75sr1yDXKWdCdzMg

=== List Collections ===
Total collections: 20
Sample collections: chat_configurations__ek0_testing,test_accounts,schema_users_client_go,batch_users,products

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
  cache:product:1: { value: { name: 'Product 1', price: 29.99 } }
  cache:product:2: { value: { price: 39.99, name: 'Product 2' } }
  cache:product:3: { value: { name: 'Product 3', price: 49.99 } }

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 3 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 11

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
Created Alice: $1000 - ID: M2tQ-SnLR2aBJDlu7PsF8f23SN4M8QWeW2PUueCn8geMqTCoVStECh01WqTyEJ8OWr7idU90k5S8YTgeEccnmQ
Created Bob: $500 - ID: YUFTilwUPCs2e2MHlQ0DGbGqZaHo-cs6vwYC1gtcndTyL9wmlnNFskszqa-5s-PQ5qBs5aURImYI-fjwhX9aww

=== Example 1: Begin Transaction ===
Transaction ID: ff79bb91-29e9-4f07-9c0f-7950e4285aa0

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 17455b77-6398-4fdb-90c7-bbb565b94ec2
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
  1. Score: 25.740, Matched: email.value, name, name.value, email
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: bio, title.value, bio.value, title
  2. Score: 26.400, Matched: title.value, title, bio, bio.value
  3. Score: 26.400, Matched: bio.value, bio, title.value, title
  4. Score: 26.400, Matched: title, bio.value, title.value, bio

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title.value, title, bio.value, bio
  2. Score: 39.600, Matched: bio, bio.value, title.value, title
  3. Score: 39.600, Matched: bio, title, bio.value, title.value
  4. Score: 39.600, Matched: bio, title.value, title, bio.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio, bio.value
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.779, Matched: 
  2. Score: 0.777, Matched: 
  3. Score: 0.755, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.702, Matched: content.value, title.value, title, content
  2. Score: 1.511, Matched: title, content, title.value, content.value
  3. Score: 0.312, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: skills, bio, bio.value, skills.value

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
✓ Inserted document: Op4AJ67IbhGH2dhoxdlRXrJ-RXeeRzsKkKsstWjNZZ74krNX2rXcJcql_r9Yq0IjOvi-ZIeSu9d8bobe5WVxfQ

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: eAGLNw826oyIDAKo276FxZFfv77DQsvbKxfITyuWxg0Je6CZRSidkT4kW1pA1M_ixGM9NpvcZPdwrSgflJEfcQ

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
✓ Inserted document with TTL: IckGKX1xz601nYteepG2t24Z7DdLRSNHRl62QAodfvqevvLTRh0cpUmAIbQFI0GjG4k4fjgbY8YD62C-EWxXMA

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
✓ Edge cache script created: Di0Y5_V3XctSrP-9XpVGhxVuGDLmtTikD8-d4xwpZ6jvU3eqRz-SdS6TU9eG2NE7NXSFi8BA2D5dbo5kmBBI0A

Call 1: Cache miss (fetches from API)
Response time: 74ms
Result: {
  "records": [
    {
      "value": {
        "address": {
          "city": "Gwenborough",
          "geo": {
            "lng": "81.1496",
            "lat": "-37.3159"
          },
          "suite": "Apt. 556",
          "zipcode": "92998-3874",
          "street": "Kulas Light"
        },
        "company": {
          "bs": "harness real-time e-markets",
          "catchPhrase": "Multi-layered client-server neural-net",
          "name": "Romaguera-Crona"
        },
        "email": "Sincere@april.biz",
        "id": 1,
        "username": "Bret",
        "name": "Leanne Graham",
        "website": "hildegard.org",
        "phone": "1-770-736-8031 x56442"
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
Response time: 4ms (18.5x faster!)
Result: {
  "records": [
    {
      "value": {
        "address": {
          "city": "Gwenborough",
          "geo": {
            "lng": "81.1496",
            "lat": "-37.3159"
          },
          "suite": "Apt. 556",
          "zipcode": "92998-3874",
          "street": "Kulas Light"
        },
        "company": {
          "bs": "harness real-time e-markets",
          "catchPhrase": "Multi-layered client-server neural-net",
          "name": "Romaguera-Crona"
        },
        "email": "Sincere@april.biz",
        "id": 1,
        "username": "Bret",
        "name": "Leanne Graham",
        "website": "hildegard.org",
        "phone": "1-770-736-8031 x56442"
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

✅ Script saved: EfVdItjG2Ofr76la6v9yFTklOzRWLSY_80IUUeIPosX2ES81BBukO3qJgnBCfFBD9DC41mEiyIzPQ_oCv1i0PQ
📊 Found 15 active users

📝 Example 2: Parameterized Script

✅ Script saved: Qlz458qh93x0_u2MbKXrJLwPJz-p2szwjmUg5-_8l5pOyVbGyTSxSn8zfquFDIe0R7khJ6FStVzTlHmjhotfqw
📊 Found 15 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: UN-TJZV_Tu379aiU03hIkEhP6ML0zWi0KL8tf2E_Fdtn31uSv92vZ3OCsjovpL4JWj2gKdqXE4HRLxwLnG3qZQ
📊 Statistics: 3 groups
   {"status":"active","count":5,"avg_score":60}
   {"status":"null","avg_score":60,"count":5}
   {"avg_score":50,"count":5,"status":"inactive"}

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
   Name: {"value":"User 1","type":"String"}
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
    "phone": "1-770-736-8031 x56442",
    "username": "Bret",
    "website": "hildegard.org",
    "id": 1,
    "email": "Sincere@april.biz",
    "name": "Leanne Graham",
    "company": ...

Second call (cache hit - from cache):
   ⏱️  Duration: 4ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "phone": "1-770-736-8031 x56442",
    "username": "Bret",
    "website": "hildegard.org",
    "id": 1,
    "email": "Sincere@april.biz",
    "name": "Leanne Graham",
    "company": ...
   🚀 Cache speedup: 0.8x faster!

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

✅ Script saved: 7y8fDvCKMhIRuQFyKnbDPYgaM7-5y7NzhFtJA4672KjF1PxSOobrg0n0HeTzcqDPIpCjh5C5wLvMgzHSkXk0PA
📊 Found 2 product groups
   {"category":"Electronics","count":3,"avg_price":575.6666666666666}
   {"count":2,"category":"Furniture","avg_price":474}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved
📊 Found 2 categories
   {"count":2,"category":"Furniture"}
   {"count":3,"category":"Electronics"}
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

✅ Inserted order: kgUijrXH7884MwD4L50hqFieayppt9bGUYXOqIVcvM7Jdmsbuh9keDN6958W2fkXET-M-edn4elD_9tTXp9veQ
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: auiMbxQ1650iJYToZoebmjEj7JPCdf3o8KPimNdKSi1Gps7kK8YW98XVtO2nObQNqJgWXuuW24LXdS0x3j0EbQ
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"role":"admin","userId":"user_abc"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: CXE9yWkhoWA4-n5bCtvOa4BEH01PSEwAStApIK_M1YurxRUOsvZLcwKCNzhBFh_CFDG_u47JIx3zFAzvLBugOQ
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Script

✅ Script saved: UOHnRF3aFpMAwhWe-NCKPJDX9FWMKPvy_1ZsWoWVsJo3LsZ9HrHTR5qRN9NoAd5hitLKZqahs66akAPu_V8Awg
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
✓ Created SWR script: fetch_api_user (Ox_anr0xpLBhMxcVZ35nkXs6zLJ3jXeEoiu_s6xUatactjZPx7aOZxNCC_84wTfYqKyOfyfJrCPQ1CQ7UjSoHA)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "website": "hildegard.org",
        "phone": "1-770-736-8031 x56442",
        "name": "Leanne Graham",
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
        "email": "Sincere@april.biz",
        "username": "Bret",
        "company": {
          "name": "Romaguera-Crona",
          "bs": "harness real-time e-markets",
          "catchPhrase": "Multi-layered client-server neural-net"
        },
        "id": 1
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
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 4ms (served from cache)
Result (cached): {
  "records": [
    {
      "value": {
        "website": "hildegard.org",
        "phone": "1-770-736-8031 x56442",
        "name": "Leanne Graham",
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
        "email": "Sincere@april.biz",
        "username": "Bret",
        "company": {
          "name": "Romaguera-Crona",
          "bs": "harness real-time e-markets",
          "catchPhrase": "Multi-layered client-server neural-net"
        },
        "id": 1
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
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_with_reviews (nI8HjRiasJ-UeCXslL_a1Fw2CdF41ZmIzLufMy_RDW8N4esPsnezqSHC8L1Z67f27TzFa0Izo64889BLEIaKNg)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {
      "value": {
        "brand": "Essence",
        "images": [
          "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
        ],
        "sku": "BEA-ESS-ESS-001",
        "stock": 99,
        "warrantyInformation": "1 week warranty",
        "rating": 2.56,
        "minimumOrderQuantity": 48,
        "availabilityStatus": "In Stock",
        "shippingInformation": "Ships in 3-5 business days",
        "dimensions": {
          "width": 15.14,
          "depth": 22.99,
          "height": 13.08
        },
        "discountPercentage": 10.48,
        "returnPolicy": "No return policy",
        "price": 9.99,
        "meta": {
          "updatedAt": "2025-04-30T09:41:02.053Z",
          "barcode": "5784719087687",
          "createdAt": "2025-04-30T09:41:02.053Z",
          "qrCode": "https://cdn.dummyjson.com/public/qr-code.png"
        },
        "weight": 4,
        "tags": [
          "beauty",
          "mascara"
        ],
        "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
        "category": "beauty",
        "id": 1,
        "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
        "reviews": [
          {
            "rating": 3,
            "reviewerName": "Eleanor Collins",
            "reviewerEmail": "eleanor.collins@x.dummyjson.com",
            "comment": "Would not recommend!",
            "date": "2025-04-30T09:41:02.053Z"
          },
          {
            "date": "2025-04-30T09:41:02.053Z",
            "reviewerName": "Lucas Gordon",
            "rating": 4,
            "reviewerEmail": "lucas.gordon@x.dummyjson.com",
            "comment": "Very satisfied!"
          },
          {
            "date": "2025-04-30T09:41:02.053Z",
            "reviewerEmail": "eleanor.collins@x.dummyjson.com",
            "rating": 5,
            "comment": "Highly impressed!",
            "reviewerName": "Eleanor Collins"
          }
        ],
        "title": "Essence Mascara Lash Princess"
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
   {"category":"Electronics","count":5,"avg_price":367}
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
   Vector databases offer several benefits such as:

1. High Precision: They provide accurate representations of data points, lines, and polygons.
2. Scalability: They allow data to be scaled up or down without loss of quality.
3. Advanced Analysis: They enable complex spatial and analytical operations.
4. Flexibility: Data can be easily manipulated and visualized in different ways.
5. Storage Efficiency: They store data in a more compact form compared to raster databases.
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
   {"avg_score":70,"count":7,"role":"user"}
   {"avg_score":20,"count":3,"role":"admin"}
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
   2. Getting Started with ekoDB (Database)
   3. Natural Language Processing (AI)
   4. Database Design Principles (Database)
   5. Vector Databases Explained (Database)
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
✓ Created session: Rfcw3lOe2ZuFSzTrMCeubJP7IxjrSY6z_Dw9BiTfwgmTiOsSQNFOgu35bVOjFkpnyOWlF1TL_fX6LZLCFNyVbw

=== Sending Chat Message ===
Message ID: uZOsfry3k3K7SK10IkOFMpac2R5-bJcyCkpvO46VPX9cEnD4Aww5vrtTxdccwqk9zabsZ1H-hlbXoAcgen0KTQ

=== AI Response ===
We have three products available:

1. ekoDB Pro: This is our Enterprise edition product with advanced features. The price for this product is $299.

2. ekoDB: This is a high-performance database product with AI capabilities. The price for this product is $99.

3. ekoDB Cloud: This is a fully managed cloud database service product. The price for this product is $499.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    id: 'zHWmzNdufOyDh-wO1qSvJZDeako5Rbi3DgcC3Z6D4R5Oy021STzrherDeUddVltdhIrL_jn2PBWFPjX5ogx2Tg',
    name: 'ekoDB Pro',
    price: 299,
    description: 'Enterprise edition product with advanced features'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    price: 99,
    description: 'A high-performance database product with AI capabilities',
    id: 'COttlqVaOqQmC_PpuEK6DbN3wnfvLwyO8dCJImJGyKRaRJaHoASJUm0liVWVOi8ruH6SkOXmJfXjgU07ZN2U-A',
    name: 'ekoDB'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    name: 'ekoDB Cloud',
    price: 499,
    id: 'd5NSi41_G6ICMHIXiMxk1M9TMoMKO0mcnyojs-QWQHB7ak5sGIWrhCa393Vt48sZlcw0L6TZSDwc3qM-kfVPqA',
    description: 'Fully managed cloud database service product'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 2756ms

=== Token Usage ===
Prompt tokens: 453
Completion tokens: 84
Total tokens: 537

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: L8bRvTi_cddoqGud8HQstlneZuB84tn-UfOzL8Gq63cgvSsxGDp_LiCPtx3B_8DDTAONG5Qk8SOb5xw4MPLcYg

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information provided, the available product is ekoDB. This is a high-performance database product priced at $99.

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
✓ Created second session: viUGfP54D7GHkKPi8-y4oIu0sEsSzcUcy7IW8t0kM-CkKDtnbvxdblIVCqmWuoPT_hcFzoh6yh4VMXX97NNiTg
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
✓ Created session: 1LfAHPb4XlXtdOzun-Pt8V8ShLVy3wFUaQFnCwK6m_wc_1ggMQ-sf8rmo3QKUKyvLkjp98CZH7KUdjax_4Pesg

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information available, the product available is "ekoDB". It is a high-performance database product. The price for this product is $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: 4JvCHg8ADJ-gSD3oSHH3n4-9LaO-2n6PS5rXcdnmCUInC381pVjsQqfhkTyijg6Y81Lh-Wm1O_UJb9SW_Ju0vA
  Parent: 1LfAHPb4XlXtdOzun-Pt8V8ShLVy3wFUaQFnCwK6m_wc_1ggMQ-sf8rmo3QKUKyvLkjp98CZH7KUdjax_4Pesg

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: 4JvCHg8ADJ-gSD3oSHH3n4-9LaO-2n6PS5rXcdnmCUInC381pVjsQqfhkTyijg6Y81Lh-Wm1O_UJb9SW_Ju0vA (Untitled)
  Session 2: 1LfAHPb4XlXtdOzun-Pt8V8ShLVy3wFUaQFnCwK6m_wc_1ggMQ-sf8rmo3QKUKyvLkjp98CZH7KUdjax_4Pesg (Untitled)
  Session 3: viUGfP54D7GHkKPi8-y4oIu0sEsSzcUcy7IW8t0kM-CkKDtnbvxdblIVCqmWuoPT_hcFzoh6yh4VMXX97NNiTg (Untitled)
  Session 4: Rfcw3lOe2ZuFSzTrMCeubJP7IxjrSY6z_Dw9BiTfwgmTiOsSQNFOgu35bVOjFkpnyOWlF1TL_fX6LZLCFNyVbw (Untitled)
  Session 5: -x18cYo6AhFsWkJX4FyQFZUmxRdlu3Y5twqrLatXf6lKNHYH1n0a78OTtK_NtEP8P6K8SOBqyWoId_5FT-fAJQ (Untitled)
  Session 6: vCj_jR9APFVECAEupGE-eShauY1CU4zi6rs881YufAEuruBTM5UiMLTrGofiOuPSc5t7QN0jtSJMOoTnG5FXbw (Untitled)
  Session 7: H5TOVzzNxakRARI8q0N8aI6MXf9khSLO60NUFeIYlRi4rNsHQf9Ft3xqdUkG0HrNylE4YlWGD3w9dHAkqNo0Ig (Untitled)
  Session 8: p4qIdxyzvVRIUsqZEFi6SQma6mUQhvAEuyfhx3AGbTcVcV_R_ZrS7yko8TuJm_H1dQAdckXP1b4GW6NMpUte0A (Untitled)
  Session 9: sAzMtmJ-PQw2AMEuPiO1cVavd-TPGz-NdoWzNaWrQwQC5YnvBxHARHZQzeL1qVMGkgyKyhSGl-4SFyQ4Yp8K6w (Untitled)
  Session 10: wI1DCXeEsJdeUO_kt02FRPdNDwDBsstg9B7FWw2CKRTfOPdsGlg_28kcDF5_qt7kKB0d1QzG3ChFwb0qYkflbw (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: 4JvCHg8ADJ-gSD3oSHH3n4-9LaO-2n6PS5rXcdnmCUInC381pVjsQqfhkTyijg6Y81Lh-Wm1O_UJb9SW_Ju0vA

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Convenience Methods Example ===

=== Native Object Creation ===
✓ Created record with plain object: {
  id: 'cmhbfTMrlm1AbSfzQEl8EA4TSFyMTLXnG29r8jc57rLktAJYpvNqJngeAL8d6uauF1CHEedrasc3KL5o3Uq65w'
}

=== Upsert Operation ===
✓ First upsert (update): {
  id: 'cmhbfTMrlm1AbSfzQEl8EA4TSFyMTLXnG29r8jc57rLktAJYpvNqJngeAL8d6uauF1CHEedrasc3KL5o3Uq65w',
  age: { type: 'Integer', value: 29 },
  active: { value: true, type: 'Boolean' },
  name: { value: 'Alice Johnson', type: 'String' },
  email: { type: 'String', value: 'alice.j@newdomain.com' }
}
✓ Second upsert (insert): {
  id: 'Zg3ICN59ev-FFZCBek9vZQBilugIJ7_ZTZTwXD3ULEwWrAhVATAl79mYcaha_F4F_VmxcV6qtdEJ21wVYRTsOA'
}

=== Find One Operation ===
✓ Found user by email: {
  id: 'cmhbfTMrlm1AbSfzQEl8EA4TSFyMTLXnG29r8jc57rLktAJYpvNqJngeAL8d6uauF1CHEedrasc3KL5o3Uq65w',
  active: { type: 'Boolean', value: true },
  email: { value: 'alice.j@newdomain.com', type: 'String' },
  age: { type: 'Integer', value: 29 },
  name: { value: 'Alice Johnson', type: 'String' }
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
   Inserted with ripple: {"id":"mNNFGMf25izaeK_feKGNBeiKtoLjBgK1BiMcP9wnjRmWB_b4OJ0TdblMqdIulwM0I1HULQOOpr9bd0jylbQirg"}

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: {"id":"f0KqCntAZVEppT4cjtFjZlA0pEfRkTyt_srk3AJfLv4jjHAG2R7R7gCeRoSMOyRE7gAdoAudLghpLYXzxdmdnA"}

3. Update with bypass_ripple:
   Updated with bypass_ripple: {"id":"mNNFGMf25izaeK_feKGNBeiKtoLjBgK1BiMcP9wnjRmWB_b4OJ0TdblMqdIulwM0I1HULQOOpr9bd0jylbQirg","name":{"value":"Product 1","type":"String"},"price":{"value":150,"type":"Integer"}}

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: {"id":"bm6fNq4Y4f7cdK2E212k-yXRT2Im0UIoerAPviQJKZtR_JpJFHy0qxFtNbzrhvCnvQJwxqze6trPNf2nOY1fvw"}

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

added 1 package, removed 1 package, and audited 9 packages in 832ms

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
  id: 'DHOnYbTKBLtnzDKUYXfHaxXYeLifuCJqKwAXjiGSeNGNyfP2vC0Dh45u1Lrl497aYvjO_Gk7qi90xOcdiaW9UQ'
}

=== Find by ID ===
Found: {
  name: 'Test Record',
  active: true,
  id: 'DHOnYbTKBLtnzDKUYXfHaxXYeLifuCJqKwAXjiGSeNGNyfP2vC0Dh45u1Lrl497aYvjO_Gk7qi90xOcdiaW9UQ',
  value: 42
}

=== Find with Query ===
Found documents: [
  {
    name: { value: 'Test Record', type: 'String' },
    active: { type: 'Boolean', value: true },
    id: 'DHOnYbTKBLtnzDKUYXfHaxXYeLifuCJqKwAXjiGSeNGNyfP2vC0Dh45u1Lrl497aYvjO_Gk7qi90xOcdiaW9UQ',
    value: { value: 42, type: 'Integer' }
  }
]

=== Update Document ===
Updated: {
  value: { value: 100, type: 'Integer' },
  name: { value: 'Updated Record', type: 'String' },
  active: { type: 'Boolean', value: true },
  id: 'DHOnYbTKBLtnzDKUYXfHaxXYeLifuCJqKwAXjiGSeNGNyfP2vC0Dh45u1Lrl497aYvjO_Gk7qi90xOcdiaW9UQ'
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
✓ Inserted test record: olqTjqs8sFbl1pTVr8na7b1YtSu7DtLDrWrNJUQIe6_b7G81iucHulLDTIOz8ahMO81ruIGD_KgiZ1JvvTKh7w

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
      },
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "olqTjqs8sFbl1pTVr8na7b1YtSu7DtLDrWrNJUQIe6_b7G81iucHulLDTIOz8ahMO81ruIGD_KgiZ1JvvTKh7w",
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
        "id": "nNbqNp0PZJeu64PMuqIrmHqPklGfWntWmBXjK_lKGx_W3ZKFCFjRngAoKH6Qz1MavLhGdZhaGsW2vaME1-AoBw",
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
        "id": "S-g_KsEmXxeRR7SufzogTElDqP-F9m9IRs05FO8UL78xYZaP_aAkSbHUzqAbaHHS4eeylQKCp90CHoPUwFFtkQ",
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

✅ Script saved: RpCd-mGVwRxlEETRpQMUBbepbuSh42N_HXYRD72aHMSiKh7IRVPJF7gecDfdusw9YyRK6Q72EjlkhJhb5ZByCA
📊 Found 30 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: C0g7am4I7Nb6R-7KMQyb2aggRfYFOM68F1C1f_5QPp4onmQstQ3crHNVvefv5dPC01bd5j2UYXLEQpzp4Du7-A
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: Lxn0Ywzq1xZuGrl334Iz1lu94dMsIMYt5S7KWDmgdXDGXCI1wgL9xUtVaBtAHoMLk-tPcgNscPa-Zl4swor6JQ
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"status":"inactive","avg_score":50,"max_score":90,"count":10}
   {"count":10,"avg_score":60,"status":"active","max_score":100}

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
Collection created with first record: jGl8aCLnimOTbLuPhbL8b6WTE29Ir2Y8NplappzHZci6b9Xa7wQzWethobkPlsLRnEUcgE_p2JnciagYetNYtw

=== List Collections ===
Total collections: 23
Sample collections: [
  'chat_configurations__ek0_testing',
  'test_accounts',
  'schema_users_client_go',
  'batch_users',
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
Created Alice: $1000 - ID: EeVnQZ4KZUG9UeeOEsYQHlbYs77l2HmBmvj_Z4oBrDgu8O0mCRs4UOP5xzlP61CzHNg9i314xBUIjOlVx7ZojQ
Created Bob: $500 - ID: 0jfrw_fY7Ql0AlNpp_LNANR1mHtYtRnMNGtGTgWoXS_9JmJItKXwj6Jn1vHT1t7lvC6k-eCsYbH3fHNgxlfoGA

=== Example 1: Begin Transaction ===
Transaction ID: 85b2a842-26f6-46cd-b8cb-997703dc1dee

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
New transaction: c7bc1600-0947-471c-a61d-2adb89b9582c
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
   ✅ Script saved: C4ALLJuQrzAVs8XtzzWROO4ukEhHf1czI3kIEABKdYxteA_yp0OV3ir-KDFlF86E7zkoCTGrgx1FjC1-Vjlqow

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
   ✅ Script saved: cMCD5MUFcYNRzqCHYo11aeq0R9Dh2dNZNcuO0e5RGn6LTpwdvhT4d0tYu6qYbM4KNE5jHpaS1ohHUNpbXxsZJw

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
   ✅ Script saved: R5WFHTDGbBvghF-XAMq_zLxp-NTJUB5pJAnY9Uyy3vTAszH98Fd2jMrkHyThnEbSTyg_H-5YtXwnZkubPV53qw

2️⃣ Calling Script (Query + Update Credits + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {"type":"Integer","value":0}
   📋 Status: {"type":"String","value":"active"}
   📋 Name: {"type":"String","value":"Alice Smith"}

============================================================
📝 Script 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: pLuVBXkfG9-RjMb_bSJ6tgEimsPZlQ1qgUscThmhmIHTjnM2DndaVEfJ0Ju3xrTd40bdpQWk29KNXfauopS6_A

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: C4ALLJuQrzAVs8XtzzWR...
   ✅ Deleted script: cMCD5MUFcYNRzqCHYo11...
   ✅ Deleted script: R5WFHTDGbBvghF-XAMq_...
   ✅ Deleted script: pLuVBXkfG9-RjMb_bSJ6...
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
  Output: Document ID = vsZR1V-oj7rs3YvzGQ5kMI8A1n9Hg2kKc4pal-TGDJekeF_fZX-rwjOEVDgRv5ixJDLXWq_2MnfA4tV_Jga7iQ
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: findById(vsZR1V-oj7rs3YvzGQ5kMI8A1n9Hg2kKc4pal-TGDJekeF_fZX-rwjOEVDgRv5ixJDLXWq_2MnfA4tV_Jga7iQ)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: findById(vsZR1V-oj7rs3YvzGQ5kMI8A1n9Hg2kKc4pal-TGDJekeF_fZX-rwjOEVDgRv5ixJDLXWq_2MnfA4tV_Jga7iQ)
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
  Output: Document ID = 4CvReKKiYHnN_Riv1KouJLQwoeHF4rnPBPoUSP3G2kViOVJKLAIJfGZzEMaBCmOtPfEhP4wEahsuoRzrhusOGA
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: findById(4CvReKKiYHnN_Riv1KouJLQwoeHF4rnPBPoUSP3G2kViOVJKLAIJfGZzEMaBCmOtPfEhP4wEahsuoRzrhusOGA)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: findById(4CvReKKiYHnN_Riv1KouJLQwoeHF4rnPBPoUSP3G2kViOVJKLAIJfGZzEMaBCmOtPfEhP4wEahsuoRzrhusOGA)
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

> @ekodb/ekodb-client@0.9.0 prepare
> npm run build


> @ekodb/ekodb-client@0.9.0 build
> tsc


up to date, audited 58 packages in 1s

15 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.9.0 build
> tsc

✅ [32mTypeScript client built![0m
🧪 [36mRunning JavaScript client library examples...[0m

added 1 package, removed 1 package, and audited 9 packages in 831ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'TPCb9582fDTY8l2PKsj_vXJBYzlgZx0021RdNGRDT4sEmfYdoN56rkZXg2WbIXlUB61um-gzrF0Xiz_Uf2ggJQ'
}

=== Find by ID ===
Found: {
  active: true,
  name: 'Test Record',
  value: 42,
  id: 'TPCb9582fDTY8l2PKsj_vXJBYzlgZx0021RdNGRDT4sEmfYdoN56rkZXg2WbIXlUB61um-gzrF0Xiz_Uf2ggJQ'
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  id: 'TPCb9582fDTY8l2PKsj_vXJBYzlgZx0021RdNGRDT4sEmfYdoN56rkZXg2WbIXlUB61um-gzrF0Xiz_Uf2ggJQ',
  value: { type: 'Integer', value: 100 },
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
✓ Inserted test record: wMX5zQ7xkpxhSpCj4xUxJGaT43LojJ2C_cy9mUYmYuL1fJNwuPQMc64gszraXXVuGo913-61sJW6yw091WNXCg

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
Collection created with first record: mjv6EfqlCCy-aBMvdxqwYO1VGQR_QlpT0N0rh6j-8EZVyVwunHKObI9wU9O7If108Fh-ngo57XCW80DhtL_Fww

=== List Collections ===
Total collections: 22
Sample collections: chat_configurations__ek0_testing,test_accounts,schema_users_client_go,batch_users,schema_users_client_py

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
cache:product:3: { price: 49.989999999999995, name: 'Product 3' }

=== KV Exists ===
Key exists: true

=== KV Find (Pattern Query) ===
Found 0 keys matching 'cache:product:.*'

=== KV Query (Alias for Find) ===
Total keys in store: 14

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: false

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: Y0X2MdSmmqXnRg6yATHHC8XdOCVh3a2YSgNOVrZOepHLciIlz1xn8U87anhrPQ1HmMENbIrpC8D3p3azjf5d6w
Created Bob: $500 - ID: Jh8FwgeOKf7n5OTBImDnIKEFYqQEQgScqWA0gg2-Gv7df-nFRz6hbSQGQ8Q3tSlV8pjgblekorM44ofn4prk-Q

=== Example 1: Begin Transaction ===
Transaction ID: cb86b97d-89a1-47bf-a779-28a6137ef85e

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: b3f62f5a-d43b-4e67-87af-adea0abad796
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
  1. Score: 25.740, Matched: name, name.value, email.value, email
  2. Score: 12.540, Matched: name.value, name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title, title.value, bio.value, bio
  2. Score: 26.400, Matched: bio, title, title.value, bio.value
  3. Score: 26.400, Matched: title, bio, bio.value, title.value
  4. Score: 26.400, Matched: bio, title, bio.value, title.value

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title.value, bio.value, title, bio
  2. Score: 39.600, Matched: title, title.value, bio.value, bio
  3. Score: 39.600, Matched: bio.value, title, bio, title.value
  4. Score: 39.600, Matched: title, bio.value, bio, title.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio, bio.value

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.763, Matched: 
  2. Score: 0.746, Matched: 
  3. Score: 0.728, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.699, Matched: content.value, content, title, title.value
  2. Score: 1.505, Matched: title, content, content.value, title.value
  3. Score: 0.291, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: bio, skills, bio.value, skills.value

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
✓ Inserted document: QFU-FDDLaU8TQ6wUO0uTnruZNQkFpVv5Jt4AMtjvOCX39cJuwa9ZIGSVuOJJgDswvJ0thU0Y3KNtTukgY4mwow

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: VsYFTEXqNvBvMDI16FnHeUdtZ2_z1QXrRRIlu9gHePZJUkdSigRPqfczDGufhZzCXFw7g5KqRIti0EPLkeL3tg

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
✓ Inserted document with TTL: R3VBFTGjwbGcEC81y7fRKO3Vm47kWmbKV-YfUnaZScuuBxrSHrVjIjBgsEWCvkHZgsbrVuZkO-6jbNlo79ntog

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
✓ Edge cache script created: 3JK6aQlXKiGv0xAJiBwDUmyq_DeEwLtLoE_r2u3ccompVnmlghZtM8Ykr0x5XYJnH2Ud9meIewfxaSQOn-v5SQ

Call 1: Cache miss (fetches from API)
Response time: 4ms
Result: {
  "records": [
    {
      "value": {
        "elevation": 32,
        "current": {
          "interval": 900,
          "time": "2026-01-27T17:00",
          "temperature_2m": -6.7
        },
        "timezone": "GMT",
        "utc_offset_seconds": 0,
        "longitude": -73.99308,
        "generationtime_ms": 0.02646446228027344,
        "timezone_abbreviation": "GMT",
        "latitude": 40.710335,
        "current_units": {
          "interval": "seconds",
          "time": "iso8601",
          "temperature_2m": "°C"
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

Call 2: Cache hit (served from ekoDB)
Response time: 5ms (0.8x faster!)
Result: {
  "records": [
    {
      "value": {
        "elevation": 32,
        "current": {
          "interval": 900,
          "time": "2026-01-27T17:00",
          "temperature_2m": -6.7
        },
        "timezone": "GMT",
        "utc_offset_seconds": 0,
        "longitude": -73.99308,
        "generationtime_ms": 0.02646446228027344,
        "timezone_abbreviation": "GMT",
        "latitude": 40.710335,
        "current_units": {
          "interval": "seconds",
          "time": "iso8601",
          "temperature_2m": "°C"
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

✅ Script saved: ssWyFgZDuNg9ZNcK0YfSoC-F8u9-FdRRK1B4jr-K2e6NOta0S-SnAvcFGJwAhVJWQ1UMjvDxwMCHcr3a8lFyNg
📊 Found 10 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved
📊 Found 10 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved
📊 Statistics: 2 groups
   {"count":5,"avg_score":50,"status":"inactive"}
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
    "phone": "1-770-736-8031 x56442",
    "username": "Bret",
    "website": "hildegard.org",
    "id": 1,
    "email": "Sincere@april.biz",
    "name": "Leanne Graham",
    "company": ...

Second call (cache hit - from cache):
   ⏱️  Duration: 3ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "phone": "1-770-736-8031 x56442",
    "username": "Bret",
    "website": "hildegard.org",
    "id": 1,
    "email": "Sincere@april.biz",
    "name": "Leanne Graham",
    "company": ...
   🚀 Cache speedup: 1.3x faster!

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

✅ Script saved: KJJ5GPr9tr_TmkUZbKJNTJk8NGS_ui4BiRv63MpiRtAwCFGR0ozYUAsKu9DuFyMzW0kcUa-_K_1aw5_vee6lmg
📊 Found 2 product groups
   {"avg_price":575.6666666666666,"category":"Electronics","count":3}
   {"count":2,"category":"Furniture","avg_price":474}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved
📊 Found 2 categories
   {"category":"Furniture","count":2}
   {"count":3,"category":"Electronics"}
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

✅ Inserted order: FZzEBx_Eu99ZKrLfF2u5GYMuagFPGgNtf8aSHGB44f-2enJjVYhRIFelJoFdpYsKSkqkyLX7FoaETjMQjxRwHg
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: fDj5KUFt4rcbMGmgxsmFr62AfrCYsCzb4dWx_lX0LU4-MtLdT72nnw5jG4T6xP-EtJKvSIoi2Wj7d1wFEMlPbg
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"role":"admin","userId":"user_abc"}
🔍 Key exists: true
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: TZsu219POkA4m6flIx0z9aN7LLYmtlgTd1gS5cqJ0DrxbK-4vBF9cwYT464CLZ9wCSA3gDAMDyojxbskQaDz0g
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: KV Pattern Query

✅ Set 4 config entries
📊 Found 0 app config entries
📊 Found 4 total config entries

📝 Example 6: Combined Wrapped Types + KV Script

✅ Script saved: Qz3aPGGOw00aXw1rmw4VMx-Xvk22_-2sDS5xt7nGXXZLRfLT-F3zw0-hUMdJpuQzgmgM5nFbwJmMWJuc7-Y2Fg
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
   admin: 20.0 (3 users)
   user: 70.0 (7 users)
⏱️  Execution time: 0ms

📝 Example 4: Top Users by Score

✅ Script saved
📊 Users (showing first 5 of 5):
   1. User 2 - Score: 40
   2. User 5 - Score: 100
   3. User 1 - Score: 20
   4. User 3 - Score: 60
   5. User 4 - Score: 80
⏱️  Execution time: 0ms

📝 Example 5: User Summary Statistics

✅ Script saved
📊 User summary (2 groups):
   active users: 7 users, Total Score: 370
   inactive users: 3 users, Total Score: 180
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
   1. Natural Language Processing (AI)
   2. Introduction to Machine Learning (AI)
   3. Vector Databases Explained (Database)
   4. Database Design Principles (Database)
   5. Getting Started with ekoDB (Database)
⏱️  Execution time: 0ms

📝 Example 2: Count Documents by Category

✅ Script saved
📊 Documents by category:
   AI: 2 documents
   Database: 3 documents
⏱️  Execution time: 0ms

📝 Example 3: Select Specific Fields

✅ Script saved
📊 Document titles (5 docs):
   1. Natural Language Processing
   2. Introduction to Machine Learning
   3. Vector Databases Explained
   4. Database Design Principles
   5. Getting Started with ekoDB
⏱️  Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Script saved
📊 Projected documents (showing first 3):
   1. Natural Language Processing
   2. Introduction to Machine Learning
   3. Vector Databases Explained
⏱️  Execution time: 0ms

📝 Example 5: All Document Fields

✅ Script saved
📊 All documents (5 total, showing first 2):
   1. Natural Language Processing (AI)
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
   1. Webcam HD - $119 (⭐4.5)
   2. USB-C Cable - $19 (⭐4.3)
   3. Monitor 27" - $399 (⭐4.6)
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
   Vector databases offer several benefits:

1. High Precision: They maintain high accuracy of data as they use points, lines, and polygons to represent data.

2. Scalability: Vector data scales effectively without losing quality as they are not pixel-based.

3. Detailed Analysis: They are suitable for complex spatial analysis including proximity and network analysis.

4. Less Disk Space: They usually require less storage space compared to raster databases.

5. Better Representation: They are ideal for representing features that have discrete boundaries like buildings or roads.
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
✓ Created SWR script: fetch_api_user_js (3sHyn2tYqqTogBAmFbeyhLTnXj_Q-3gyq9Nu43SzZgql3U7DM8PfwJtdP7Du351WNZmSWGomCEbGcLM1_KAPrg)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "website": "hildegard.org",
        "phone": "1-770-736-8031 x56442",
        "name": "Leanne Graham",
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
        "email": "Sincere@april.biz",
        "username": "Bret",
        "company": {
          "name": "Romaguera-Crona",
          "bs": "harness real-time e-markets",
          "catchPhrase": "Multi-layered client-server neural-net"
        },
        "id": 1
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
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 6ms (served from cache)
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
✓ Created session: 5_2pt119763cuAEGndpFXUXO-G_caHq6R576pVmjxqGlGa2Iite-90L1dH1QqBL-4XPGkq96Hc1auCIXEn6akA

=== Sending Chat Message ===
Message ID: L3v-_CpIAhmrSwaoZPysRhFQVcMsEFtRjx-T7xOIDk5c-40-uaOPOak7OG7N9691o2Tf90i05bYdUMazkSaPhA

=== AI Response ===
There are three products available:

1. The ekoDB Cloud is a fully managed cloud database service product, priced at $499.
2. The ekoDB Pro is an enterprise edition product with advanced features, it is priced at $299.
3. The ekoDB is a high-performance database product with AI capabilities, and it is priced at $99.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_js',
  record: {
    description: 'Fully managed cloud database service product',
    price: 499,
    id: 'DKMFHGk_NsyTy35-c8yPKIw_YJYz8ghwaAzr-puXf7A67yLtDT6pTvwnf0SU4V_Bv-T8YxpskJ1dr9fdz5W5MA',
    name: 'ekoDB Cloud'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_js',
  record: {
    price: 299,
    id: 'MpAQDxHDLoUnHsD7rDiMnikWWKrCcU3-eJ0pGH62EmA7i15Ck0QJL9YCRHE0EaWZjPfFnTwpTBP9MhWBja2tdg',
    name: 'ekoDB Pro',
    description: 'Enterprise edition product with advanced features'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_js',
  record: {
    name: 'ekoDB',
    price: 99,
    id: 'AE-zBmxJrtX_gtdLUZTp2hMOAKfR5kiJZKcut5LX2QdDwVbXigKP3KoK6SG4tuppzAZfHUQNRvDciiQDMR5q_Q',
    description: 'A high-performance database product with AI capabilities'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 2668ms

=== Token Usage ===
Prompt tokens: 450
Completion tokens: 74
Total tokens: 524

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: hw6Lp89Be2xUM74GjA0jdb9fCJa9IHlkTcv0l812tG2uD3hH_VstkMMk5clUdpdeTt9F1VrxM68naLtFP5deZQ

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information provided, there is a product available named "ekoDB". This is a high-performance database product. The price for this product is $99.

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
✓ Created second session: G38RUkxNovijCF-8DGs1rQKzXbWxCvY-d73y3PZ_IbWl2T5m6-A2BCT6eHscs9iiXeTBsFeFjXNzmxsATtD1AA
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
✓ Created session: eMVgQibNVejyIZvXZ4r7pE2uJJPUMKTJdHepMPv6j4LFv64F15t3XEQA2J-5OldelTiPGkTIj7Q_y2Nl8_gVpw

=== Sending Messages ===
✓ Message 1 sent
  Response: The product available is called "ekoDB". It is a high-performance database product priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: PnvhTm232pozzgENm23t8mQhO7pWC81m-6Q8VTTcFHGXT_yI5nVRCqEuPtAlDUG39y0No8PKt3WpxLthM0naOw
  Parent: eMVgQibNVejyIZvXZ4r7pE2uJJPUMKTJdHepMPv6j4LFv64F15t3XEQA2J-5OldelTiPGkTIj7Q_y2Nl8_gVpw

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: PnvhTm232pozzgENm23t8mQhO7pWC81m-6Q8VTTcFHGXT_yI5nVRCqEuPtAlDUG39y0No8PKt3WpxLthM0naOw (Untitled)
  Session 2: eMVgQibNVejyIZvXZ4r7pE2uJJPUMKTJdHepMPv6j4LFv64F15t3XEQA2J-5OldelTiPGkTIj7Q_y2Nl8_gVpw (Untitled)
  Session 3: G38RUkxNovijCF-8DGs1rQKzXbWxCvY-d73y3PZ_IbWl2T5m6-A2BCT6eHscs9iiXeTBsFeFjXNzmxsATtD1AA (Untitled)
  Session 4: 5_2pt119763cuAEGndpFXUXO-G_caHq6R576pVmjxqGlGa2Iite-90L1dH1QqBL-4XPGkq96Hc1auCIXEn6akA (Untitled)
  Session 5: 1LfAHPb4XlXtdOzun-Pt8V8ShLVy3wFUaQFnCwK6m_wc_1ggMQ-sf8rmo3QKUKyvLkjp98CZH7KUdjax_4Pesg (Untitled)
  Session 6: viUGfP54D7GHkKPi8-y4oIu0sEsSzcUcy7IW8t0kM-CkKDtnbvxdblIVCqmWuoPT_hcFzoh6yh4VMXX97NNiTg (Untitled)
  Session 7: Rfcw3lOe2ZuFSzTrMCeubJP7IxjrSY6z_Dw9BiTfwgmTiOsSQNFOgu35bVOjFkpnyOWlF1TL_fX6LZLCFNyVbw (Untitled)
  Session 8: -x18cYo6AhFsWkJX4FyQFZUmxRdlu3Y5twqrLatXf6lKNHYH1n0a78OTtK_NtEP8P6K8SOBqyWoId_5FT-fAJQ (Untitled)
  Session 9: vCj_jR9APFVECAEupGE-eShauY1CU4zi6rs881YufAEuruBTM5UiMLTrGofiOuPSc5t7QN0jtSJMOoTnG5FXbw (Untitled)
  Session 10: H5TOVzzNxakRARI8q0N8aI6MXf9khSLO60NUFeIYlRi4rNsHQf9Ft3xqdUkG0HrNylE4YlWGD3w9dHAkqNo0Ig (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: PnvhTm232pozzgENm23t8mQhO7pWC81m-6Q8VTTcFHGXT_yI5nVRCqEuPtAlDUG39y0No8PKt3WpxLthM0naOw

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
✓ Inserted user: EkoRecord(fields={id=StringValue(value=py8d8YqeYyKSZNnhxpxDGHhFzho015jyRUfdx9scnQdcbYA8Lgpx07pIQKH9-R8A0zKqwMHtGnx7OidPx_TQJQ)})
  User ID: py8d8YqeYyKSZNnhxpxDGHhFzho015jyRUfdx9scnQdcbYA8Lgpx07pIQKH9-R8A0zKqwMHtGnx7OidPx_TQJQ

=== Read ===
✓ Found user by ID: EkoRecord(fields={created_at=StringValue(value=Tue Jan 27 12:16:02 EST 2026), user_id=StringValue(value=550e8400-e29b-41d4-a716-446655440000), categories=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)]), id=StringValue(value=py8d8YqeYyKSZNnhxpxDGHhFzho015jyRUfdx9scnQdcbYA8Lgpx07pIQKH9-R8A0zKqwMHtGnx7OidPx_TQJQ), embedding=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)]), name=StringValue(value=Alice Johnson), active=BooleanValue(value=true), price=FloatValue(value=99.99), age=IntegerValue(value=28), tags=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)]), metadata=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})}), email=StringValue(value=alice@example.com), data=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)])})

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Alice Johnson
  email (String): alice@example.com
  age (Integer): 28
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): Tue Jan 27 12:16:02 EST 2026
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1, tag2, tag3]
  metadata (Object): {key=value, nested={deep=true}}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): [electronics, computers]
  data (Bytes): 11 bytes
Record fields: created_at, user_id, categories, id, embedding, name, active, price, age, tags, metadata, email, data

=== Update ===
✓ Updated user: EkoRecord(fields={user_id=ObjectValue(value={value=StringValue(value=550e8400-e29b-41d4-a716-446655440000), type=StringValue(value=String)}), data=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)])}), tags=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)])}), created_at=ObjectValue(value={value=StringValue(value=Tue Jan 27 12:16:02 EST 2026), type=StringValue(value=String)}), price=ObjectValue(value={value=FloatValue(value=99.99), type=StringValue(value=Float)}), name=ObjectValue(value={value=StringValue(value=Alice Johnson), type=StringValue(value=String)}), embedding=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)])}), categories=ObjectValue(value={value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)]), type=StringValue(value=Array)}), city=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=San Francisco)}), active=ObjectValue(value={value=BooleanValue(value=true), type=StringValue(value=Boolean)}), age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=29)}), metadata=ObjectValue(value={type=StringValue(value=Object), value=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})})}), email=ObjectValue(value={value=StringValue(value=alice@example.com), type=StringValue(value=String)}), id=StringValue(value=py8d8YqeYyKSZNnhxpxDGHhFzho015jyRUfdx9scnQdcbYA8Lgpx07pIQKH9-R8A0zKqwMHtGnx7OidPx_TQJQ)})

=== Query ===
✓ Found 1 users matching query
  - EkoRecord(fields={email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice@example.com)}), embedding=ObjectValue(value={value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)]), type=StringValue(value=Array)}), metadata=ObjectValue(value={type=StringValue(value=Object), value=ObjectValue(value={key=StringValue(value=value), nested=ObjectValue(value={deep=BooleanValue(value=true)})})}), data=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)])}), age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=29)}), tags=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)])}), created_at=ObjectValue(value={value=StringValue(value=Tue Jan 27 12:16:02 EST 2026), type=StringValue(value=String)}), active=ObjectValue(value={type=StringValue(value=Boolean), value=BooleanValue(value=true)}), price=ObjectValue(value={type=StringValue(value=Float), value=FloatValue(value=99.99)}), id=StringValue(value=py8d8YqeYyKSZNnhxpxDGHhFzho015jyRUfdx9scnQdcbYA8Lgpx07pIQKH9-R8A0zKqwMHtGnx7OidPx_TQJQ), categories=ObjectValue(value={value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)]), type=StringValue(value=Array)}), name=ObjectValue(value={value=StringValue(value=Alice Johnson), type=StringValue(value=String)}), city=ObjectValue(value={value=StringValue(value=San Francisco), type=StringValue(value=String)}), user_id=ObjectValue(value={value=StringValue(value=550e8400-e29b-41d4-a716-446655440000), type=StringValue(value=String)})})

=== Delete ===
✓ Deleted user with ID: py8d8YqeYyKSZNnhxpxDGHhFzho015jyRUfdx9scnQdcbYA8Lgpx07pIQKH9-R8A0zKqwMHtGnx7OidPx_TQJQ

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
  {"payload":{"data":[{"id":"bvmc9xJDgEFwY1tmhqNrM_t9v1HyqfZTMTHRZ6JYZcNdy4CI-nw9V-GXitGp_CqvDuH6TjucDyEFW1EBI327gQ","name":{"type":"String","value":"Test User"},"status":{"type":"String","value":"active"}}]},"type":"Success"}

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
  IDs: Ebq53-YOf2GPENj9-NCSFSB6jE2G6oqnvfIMy4kBOfaGjtw_j075OIopaTXV2WruZuo2JV7T4Mt6_pDAHthXwg, JPJCONvAHlLDLhv8SP5dgJRjAqdlyAFDmUUdCmSoYOvy5xNTfyumASL6w2JbBNSFPF4KNifQuLDl_gkm8cPyvg, xsp4s2O7hXs5UBtUqyWcV-m9DmnxlnVnzAwh7-Ag79P94lLzE4whWqpcUkGZ3Skgs-6mcYcZDUT6rY--6qdb0Q...

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
  - chat_configurations__ek0_testing
  - test_accounts
  - schema_users_client_js
  - schema_products_client_js
  - schema_users_client_go

=== Check Collection Existence ===
Collection 'kotlin_collection_example' exists: false

=== Create Collection with Schema ===
✓ Created collection with schema: kotlin_collection_example

=== Get Collection Schema ===
✓ Schema: {"fields":{"name":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null}},"version":1,"created_at":"2026-01-27T17:16:15.862071Z","last_modified":"2026-01-27T17:16:15.862071Z","bypass_ripple":false,"primary_key_alias":"id"}

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
✓ Session value: {"created_at":1769534180167,"user_id":"123"}
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
  config:api: {"value":{"retries":3,"timeout":30}}

=== KV Exists ===
✓ Key exists: true

=== KV Find (Pattern Query) ===
✓ Found 3 keys matching 'config:.*'

=== KV Query (Alias for Find) ===
✓ Total keys in store: 15

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
Created Alice: $1000 - ID: aprFCXm8WkLxyU8Cb2a5HKLC3NgAiwhjc6xxQDJ4s6MJnCr-nAFCgxSa47V9NcO5ob8RNiP3XKpv8ELNYIhxjg
Created Bob: $500 - ID: gAgYe45RKe6tycpQKmTXTG__-A9bhMmgaUQijKP-mmha8iJZK2UyP3tFI2uxLVAnyiLS1k63If2gp5K3LWYZNA

=== Example 1: Begin Transaction ===
Transaction ID: 1d51af29-ede7-40f3-a906-92c7cee6759d

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 6845689d-598c-4103-9c48-de1c2e71700b
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
  {"results":[{"record":{"description":"Learn Rust programming language with hands-on examples and best practices.","title":"Rust Programming","tags":"programming,rust,tutorial","id":"3rW9oC3NPcnz0pQACIhUdoBhbKOF1e3onJCvFOCqaK3Ri8r-aP0y5-NhbjoM6Uf58ndX9i8Uc6J3gZH7AUgT4w","views":60},"score":19.8,"matched_fields":["description","title","tags"]},{"record":{"description":"Build modern web applications using JavaScript, React, and Node.js.","views":956,"tags":"programming,javascript,web","id":"aSOZ4XYDydBat5tyK3lL3dwzWnY60_YTh9nAyN0zt0h7WDVeKyiVMbwdeaXThgxQ4bawVFfEPk7cOTKmXBtfYg","title":"JavaScript Web Development"},"score":6.6000000000000005,"matched_fields":["tags"]},{"record":{"description":"Master Python for data analysis, machine learning, and visualization.","id":"MfoP9Py7mlRZLkMCJsRpwLFYgVpDqIRCpwfrI4fqdEU5L3kQF5juki5oya3yzeIt8RhZalLSY9KFB6ZXIO7k3A","title":"Python for Data Science","views":838,"tags":"programming,python,data-science"},"score":6.6000000000000005,"matched_fields":["tags"]}],"total":3,"execution_time_ms":1}

=== Search for 'machine learning' ===
✓ Found results for 'machine learning'
  {"results":[{"record":{"title":"Machine Learning Basics","description":"Introduction to machine learning algorithms and neural networks.","id":"XCYzo4h-jx8rkz-L3rC4hsWUiAt8pJTxPeIIYvOrHlNfUJBXEzeE4TvLS3HImjcjx8L2LizHqFG3R-H3D_p2hA","tags":"ai,machine-learning,python","views":471},"score":2.7,"matched_fields":["description","tags","title"]},{"record":{"views":838,"description":"Master Python for data analysis, machine learning, and visualization.","tags":"programming,python,data-science","id":"MfoP9Py7mlRZLkMCJsRpwLFYgVpDqIRCpwfrI4fqdEU5L3kQF5juki5oya3yzeIt8RhZalLSY9KFB6ZXIO7k3A","title":"Python for Data Science"},"score":1.0,"matched_fields":["description"]},{"record":{"id":"whsqDON6fmzDBul12NjO_PX6R-RGylpUPn7ikw_DW6sapZiG_VjAvTEWE7a8kofndQ109yEEqsFXTSVukjJr9g","title":"Database Design","views":200,"description":"Learn database design principles, normalization, and query optimization.","tags":"database,design,sql"},"score":0.5,"matched_fields":["description"]},{"record":{"views":60,"id":"3rW9oC3NPcnz0pQACIhUdoBhbKOF1e3onJCvFOCqaK3Ri8r-aP0y5-NhbjoM6Uf58ndX9i8Uc6J3gZH7AUgT4w","title":"Rust Programming","tags":"programming,rust,tutorial","description":"Learn Rust programming language with hands-on examples and best practices."},"score":0.5,"matched_fields":["description"]}],"total":4,"execution_time_ms":1}

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
✓ Inserted user 1: StringValue(value=9VyhWtTfcP0q6Yg2vHJuZ2pUiS52Y6a2xOBwqjy1FI9xNafftcdjsshvEf6sv-Qz8faxUdVTmXNOcc2lAObtzg)
✓ Inserted user 2: StringValue(value=9-BYNap_3EXz9utCpKQ8Gm78xMFN_tdcMEYKsZDf4cd9e7-BXVDSg_euLkvxzMDkdCjTKMsbBSZCBfvJ1GlSAw)

=== Getting Schema ===
✓ Schema for kotlin_schema_example:
  Fields: {"status":{"field_type":"String","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"email":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"title":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null}}

=== Listing Collections ===
✓ Total collections: 27
  Sample: [chat_configurations__ek0_testing, test_accounts, schema_users_client_js, schema_products_client_js, schema_users_client_go]

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
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Document TTL Example ===

=== Insert with TTL ===
✓ Inserted document with 10s TTL
  Document ID: Ci6SWclgS0XRkXu5WWCMlz2nYU1r-Htdk7J3DG74KM-f2ELRD56gCeQFVphyXJpHyUI1qitVn9bjOUyouaJVqA

=== Verify Document Exists ===
✓ Document found: user_id, session_id, id, created_at, ttl

=== Insert with Longer TTL ===
✓ Inserted document with 1h TTL
  Document ID: _6pPGQkrz-X6d_GzcpbfStRpxtRRSoCJIdFHRr-B41f28PnzuyAcQXputknJo1DF-IfiO2w4M_5u--rAC-OL2A

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
✓ Inserted document with TTL: StringValue(value=33bcc0QofpyEGzd0AmOkXkRzla6bAmx-DXcWuHOHYKG2ME23qmHMJYLpNKBvGwenDfsDZWBRja3Wupj1uuVymg)

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved data via WebSocket:
  {"payload":{"data":[{"created_at":{"type":"Integer","value":1769534210874},"id":"33bcc0QofpyEGzd0AmOkXkRzla6bAmx-DXcWuHOHYKG2ME23qmHMJYLpNKBvGwenDfsDZWBRja3Wupj1uuVymg","name":{"type":"String","value":"WebSocket TTL Test"},"ttl":"2026-01-27T18:16:50.993804Z","value":{"type":"Integer","value":42}}]},"type":"Success"}

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
✓ Edge cache script created: RSVdKi5wQupTFwE7thCHAcVYPqyEq_NLQr0BV_OtciEVfoEs0b4ZB8yKv6GtZ-9xewkIruvn7K_ka1Dy9waPbw

Call 1: Cache lookup
Found 1 cached entries
Response time: 15ms

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

✅ Script saved: QLNnEECH39MK-U6O5dTik9d46w35MNsRbwMljVdkvqgoYzTLAXECizkIiFws-aXbdSlm9vxMWLawBF1B21T8YQ
📊 Found 15 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved
📊 Found 15 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved
📊 Statistics: 3 groups
   {"avg_score":60.0,"count":5,"status":"null"}
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
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
🚀 ekoDB Kotlin Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved: q1IsWiDbkp-TztCnUUfZs1EJ2kIwzFIgGgMLeVb71m99m8Qok3ARW1Y8gwN6YF35zup09ES2_AWzgB-OmEknJA
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

✅ Inserted order: StringValue(value=SGiqMtToBfVKUDL2S8BtaQQ7nIz_Uym_fnjB-FpjL08Tl3H408rUjXEQnsNHxdEQuM4lBmrrUh12cXOL6pzC_A)
✅ Inserted 2 products

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: 89C_NhK00vBmJoiGhLEtDiSDNV4qqLhHzodTmzMzJVPCxqkwkPSXqZ6JycU3TPfafwoisfQVa5sBaZiR_OmAoQ
📊 Created order via script
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: {"role":"admin","userId":"user_abc"}
✅ Set cached data with 1 hour TTL
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: _QdQNc2wi1AwZvkmjOpWz6h1ITufBZLRCc_S9C7CiFFrAqyuC25HEdFQedClC3sUQEmJycCDRor0-0s7rOYc7g
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Script

✅ Script saved: zH5Ykg_WXGMXSNCV0wCbThm99rduS2Lq3Mi_MCMtnlYaeMp0cmW9iIi3ZPr7XLyRBtZZpoQzd8adV4mVrK2B_w
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
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Setting up cache collection...
✓ Cache entry created

Step 2: Create SWR cache lookup script
✓ Created SWR script: swr_cache_lookup_kt (EzWHMcS5hDOeLH3hpNDq8XmgzKbJ9hl7HcSDq6mioofhrnY8R1g2uqCDX3QuRUxezyWuGwe_7uav-jLvsQreFA)

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
   {"avg_price":365.6666666666667,"category":"Furniture","count":3}
   {"avg_price":367.0,"count":5,"category":"Electronics"}
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
   {"response":"Vector databases have several benefits including efficient storage, precise representation of data, easy modification or editing, and high scalability. They are ideal for applications requiring complex, multi-layered graphics, and they are not resolution-dependent, ensuring high quality at any scale.","tokens_used":95}
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
   1. {"type":"String","value":"Database Design Principles"} ({"value":"Database","type":"String"})
   2. {"type":"String","value":"Vector Databases Explained"} ({"value":"Database","type":"String"})
   3. {"type":"String","value":"Natural Language Processing"} ({"value":"AI","type":"String"})
   4. {"type":"String","value":"Getting Started with ekoDB"} ({"value":"Database","type":"String"})
   5. {"type":"String","value":"Introduction to Machine Learning"} ({"value":"AI","type":"String"})
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
✓ Created session: 9ooD3yEV8H4CeWnI2EK2Leunw1XRy4RjLbQBcX4Br9M9gMArW2Q2kXifLluwODgL63pz6CB1KDn3Oe6esPNq3g

=== Sending Chat Message ===
✓ Chat response:
  Message ID: "MoKFv4wN7tiySFpVOVr1Eo9SWG8eEmGjy-ykVPlfigzH08dx8UiqrS-XLrWc6mSEQRDOlBWzX6DFTuVi1OY55A"
  Responses: ["ekoDB is a high-performance database that comes with intelligent caching and real-time capabilities. It's distinguished for its AI integration, which notably manifests in features such as AI Chat Integration. This allows users to query their database using natural language and receive responses that are enhanced by AI, providing relevant context.\n\nMoreover, ekoDB includes robust search functionalities such as full-text search, vector search, and hybrid search. These features are further complemented by automatic context retrieval, enhancing the accuracy and relevance of search results."]

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
✓ Created session: CsoNkm7-1fIa1WlsVWzuypjk-C3rgM_BUWZKtYUOIZS34MYRqfpj7f8L98ZKC3MeWlKiTgDn06zLQFPS66fAoQ

=== Sending Initial Message ===
✓ Message sent
  Responses: ["Based on the context provided, the available product is \"ekoDB\", which is a high-performance database product. It's priced at $99."]

✓ Second message sent

=== Regenerating AI Response ===
✓ AI response regenerated
  New responses: ["Currently, the available product is the ekoDB. It is a high-performance database product which is priced at $99."]

=== Updating Message ===
✓ Updated message content

=== Toggling Forgotten Status ===
✓ Marked message as forgotten (excluded from context)

=== Creating Second Session for Merge ===
✓ Created second session: gjUCbaqp44GAIGMgoRhZysUMcRp1i_gx0E6xcWf2go6XtnI17KfNKlAo_7nWxEHxWJ4P4QTGBHZehWn6FTPHPQ

=== Merging Sessions ===
✓ Merged sessions
  Merged session ID: null

=== Deleting Message ===
✓ Deleted message

=== Cleanup ===
✓ Deleted chat sessions
✓ Deleted collection: kotlin_chat_advanced_example

✓ Advanced chat features example completed successfully

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
=== ekoDB Kotlin Client - Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: bXY8gjTuSWKL2GoiucNfialyC2XTHmh3lREP-FalD1oAyYefDqUe_ttcNdJ9zmdZmhwAjlanmDGp1g33F17wCA

=== Sending Messages ===
✓ Message 1 sent
  Responses: ["The available product is ekoDB, a high-performance database product with AI capabilities. It is priced at $99."]

✓ Message 2 sent
  Responses: ["The price of the ekoDB product is $99."]

=== Getting Message History ===
✓ Retrieved message history
  Total messages: [{"chat_id":{"type":"String","value":"bXY8gjTuSWKL2GoiucNfialyC2XTHmh3lREP-FalD1oAyYefDqUe_ttcNdJ9zmdZmhwAjlanmDGp1g33F17wCA"},"content":{"type":"String","value":"What products are available?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"s38sn9tSU0AOqae_9S1lnmbGhOgWlFijknywYVmrogYW4qGffUOCViVyw70nLKIg0VdFLEpAjd5WV84PrYiGjQ","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2026-01-27T17:17:56.929997Z"},"id":"xsKXNU8zRgpYUD0cTRu0Ypz5intdWqgJpDLYM1cyTYa8wqX-Ta1FHh1-XTV-ZN4a4HDod0RNAhIZwu0u9mYGCA","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":24,"prompt_tokens":173,"total_tokens":197}},"updated_at":{"type":"DateTime","value":"2026-01-27T17:17:56.929997Z"}},{"chat_id":{"type":"String","value":"bXY8gjTuSWKL2GoiucNfialyC2XTHmh3lREP-FalD1oAyYefDqUe_ttcNdJ9zmdZmhwAjlanmDGp1g33F17wCA"},"content":{"type":"String","value":"The available product is ekoDB, a high-performance database product with AI capabilities. It is priced at $99."},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"s38sn9tSU0AOqae_9S1lnmbGhOgWlFijknywYVmrogYW4qGffUOCViVyw70nLKIg0VdFLEpAjd5WV84PrYiGjQ","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2026-01-27T17:17:56.937632Z"},"id":"gUHBbOCRueYLsSlMuViiI79KH2SBnJWzQGoTAovKZPN7iERRbOkuz6Q28RXkGOggkcdSqNPINZOYk8NJ7MAC1g","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":24,"prompt_tokens":173,"total_tokens":197}},"updated_at":{"type":"DateTime","value":"2026-01-27T17:17:56.937632Z"}},{"chat_id":{"type":"String","value":"bXY8gjTuSWKL2GoiucNfialyC2XTHmh3lREP-FalD1oAyYefDqUe_ttcNdJ9zmdZmhwAjlanmDGp1g33F17wCA"},"content":{"type":"String","value":"What is the price?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["price"],"record":{"description":"A high-performance database product with AI capabilities","id":"s38sn9tSU0AOqae_9S1lnmbGhOgWlFijknywYVmrogYW4qGffUOCViVyw70nLKIg0VdFLEpAjd5WV84PrYiGjQ","price":99,"product":"ekoDB"},"score":0.8}]},"created_at":{"type":"DateTime","value":"2026-01-27T17:17:58.340764Z"},"id":"LSJ4nexFStb_1L1cBn_nhlfxoPtrKtkx5U5bj7figJkCHdvqJhP-kjGJro0OpDuNeFZNPSV1Mv-GtrIzhdqqQw","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":12,"prompt_tokens":173,"total_tokens":185}},"updated_at":{"type":"DateTime","value":"2026-01-27T17:17:58.340764Z"}},{"chat_id":{"type":"String","value":"bXY8gjTuSWKL2GoiucNfialyC2XTHmh3lREP-FalD1oAyYefDqUe_ttcNdJ9zmdZmhwAjlanmDGp1g33F17wCA"},"content":{"type":"String","value":"The price of the ekoDB product is $99."},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["price"],"record":{"description":"A high-performance database product with AI capabilities","id":"s38sn9tSU0AOqae_9S1lnmbGhOgWlFijknywYVmrogYW4qGffUOCViVyw70nLKIg0VdFLEpAjd5WV84PrYiGjQ","price":99,"product":"ekoDB"},"score":0.8}]},"created_at":{"type":"DateTime","value":"2026-01-27T17:17:58.345239Z"},"id":"sDAfyiMU3eWbTuB0Vw8nrtqOYlfVrzA5py1ejVRpJissC9Zh0IwlqyO0o-zjqhZOwH7ZD_KiE1RahBM1s5ZDTQ","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":12,"prompt_tokens":173,"total_tokens":185}},"updated_at":{"type":"DateTime","value":"2026-01-27T17:17:58.345239Z"}}]

=== Updating Session ===
✓ Updated session system prompt

=== Listing Sessions ===
✓ Total sessions: 16

=== Branching Session ===
✓ Created branched session: NuNkHgWpEILNQywrg8PtaHr5iYtw1r7hV6t0XWbZ52H--Woh8bsDV83ZeA3fL2Lr3XWQKt-wrCTSpqqeWnMAlQ

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
✓ Created record: EkoRecord(fields={id=StringValue(value=b5A9qS-eHOKZEHc9YyLWp3lR4-ESXyVF2_7PdPrhywBM1cvd9Cos2t7VOJ2D8s0ESC13cDiMErcXiw98xAStvA)})

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
   Inserted with ripple: EkoRecord(fields={id=StringValue(value=5cWG7vkuTJmcuwG2FeGIGm_4uS2pY4u2X7zRF_-CKI5Sid9OOj4PQp8b7flM7k022hX-7AhvfLhav_PIDzBvig)})

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: EkoRecord(fields={id=StringValue(value=77YRyLI47crhW5sqnXQVRDizmiQ1AvEnAXY93Wyd7-6TZQLpVIpvVumy9DyRqyvj20kY1JKvx5m8xBDvqGqKtw)})

3. Update with bypass_ripple:
   Updated with bypass_ripple: EkoRecord(fields={name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Product 1)}), price=ObjectValue(value={value=IntegerValue(value=150), type=StringValue(value=Integer)}), id=StringValue(value=5cWG7vkuTJmcuwG2FeGIGm_4uS2pY4u2X7zRF_-CKI5Sid9OOj4PQp8b7flM7k022hX-7AhvfLhav_PIDzBvig)})

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: EkoRecord(fields={name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Upsert Product)}), id=StringValue(value=5cWG7vkuTJmcuwG2FeGIGm_4uS2pY4u2X7zRF_-CKI5Sid9OOj4PQp8b7flM7k022hX-7AhvfLhav_PIDzBvig), price=ObjectValue(value={value=IntegerValue(value=500), type=StringValue(value=Integer)})})

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
Created Alice: $1000 - ID: 2Qo05PADaTYBSjtdUkPmx-JBnIG6g4FthDN405yQQmXjh0te6hxerdcd2dbiVPK_G7Rbc9Xe4t_uEYYNdkb2ag
Created Bob: $500 - ID: OE5DZqu5PmNFWCURoFP5lNHv3_l7XUEBFf0yCOJdYPpmUchg4Pe6g3XKFerVDi-3TUDMeo95SwM7nk7oUW9X9g

=== Example 1: Begin Transaction ===
Transaction ID: 08ed39e1-400c-4585-b688-56f8d26542e4

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 7a944bc6-4b53-4fdb-95e1-31be9c1af0fe
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
📦 Built wheel for abi3 Python ≥ 3.8 to /Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.9.0-cp38-abi3-macosx_11_0_arm64.whl
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
    ✓ Generated embedding: 1536 dimensions in 339.762458ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 454.198209ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 376.684125ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 496.345333ms
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 726.29025ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 502.34775ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 608.114083ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 509.027583ms
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 540.035834ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 506.306042ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 602.728625ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 378.030958ms
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
    ✓ Generated embedding: 1536 dimensions in 357.42775ms
    • Function auto-cleaned up by client

→ Executing hybrid_search()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 112.951791ms

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

Writing memory-safe, high-performance database code involves several steps. Here are some guidelines:

1. **Understand Your Data**: Before you start writing your code, understand the structure, type, and relationships of your data. This can help you choose the right database system. As discussed in previous contexts, if your data is unstructured or you need high write throughput, NoSQL might be a better choice. If your data is structured with well-defined relationships, SQL might be more suitable.

2. **Normalize Your Database**: As mentioned in Context 2 and 3, database normalization is a process that helps reduce data redundancy and improve data integrity. It involves dividing large tables into smaller ones and defining relationships among them. This can significantly improve the performance of your database.

3. **Optimize Your Queries**: To write high-performance database code, you need to optimize your queries. This involves techniques like using indexes, avoiding full table scans, and reducing the amount of data transferred. 

4. **Use Prepared Statements**: Prepared statements not only can help improve the performance of your database code but also can add an extra layer of security by defending against SQL injection attacks.

5. **Manage Your Resources**: Make sure you close database connections when you're done with them to free up system resources. Also, avoid unnecessary database calls by caching results when appropriate.

6. **Memory Management**: For memory-safety, it's important to properly manage the memory allocated for your application. This involves properly deallocating memory after use and avoiding memory leaks. In case of languages like Java, memory management is handled by Garbage Collection, but for languages like C++, you need to manually manage it.

7. **Concurrency Control**: To maintain data consistency and handle multiple accesses to the database, use proper concurrency control mechanisms like ACID properties in SQL.

These are just a few considerations. The specifics can vary greatly depending on your particular database system and application requirements.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 419.491417ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 2122 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 608.090417ms
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
  ✓ Text search completed in 73.914583ms

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
    ✓ Generated embedding: 1536 dimensions in 0.455s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.480s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.519s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.685s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.272s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.319s
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
    ✓ Generated embedding: 1536 dimensions in 0.368s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.432s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.387s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.581s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.239s
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
    ✓ Generated embedding: 1536 dimensions in 0.372s
    • Function auto-cleaned up by client

→ Executing hybrid_search()...
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

Writing memory-safe high-performance database code involves a number of concepts, from database design, query optimization, to the appropriate choice of database type. Here are a few guidelines:

1. Normalize Your Database: As mentioned in Context 1, database normalization is the process of organizing data to reduce redundancy and improve data integrity. It involves dividing large tables into smaller ones and defining relationships between them using foreign keys. A well-structured database contributes to the safety and performance of your code.

2. Optimize Your Queries: Query optimization is a significant part of enhancing the performance of your database code. This involves minimizing the amount of data the database needs to look through when executing your queries. You can achieve this by indexing, using joins appropriately, and avoiding redundant queries.

3. Choose the Right Database Type: As described in Context 3 and 4, the choice between SQL and NoSQL databases depends on the specific project requirements. SQL is typically used when dealing with structured data that requires complex queries and ACID transactions. On the other hand, NoSQL is used when dealing with unstructured data, when you need flexible schemas, high write throughput, or horizontal scaling. The right choice here contributes to both the performance and memory safety of your code.

4. Use Appropriate Data Types: Using appropriate data types can both enhance performance and ensure memory safety. For example, if a number field will not be used for calculations, it can be stored as a string to save memory.

5. Consider Caching: To reduce the load on the database, consider caching frequently accessed data. This can greatly improve the performance of your database operations.

6. Use Prepared Statements: Prepared statements not only help with the security of your database code, but they can also improve performance as the database can efficiently reuse the execution plan for a statement.

7. Handle Database Connections: Efficient handling of database connections is also crucial. Too many open connections can lead to memory leaks while too few can bottleneck your application. Consider using a connection pool to efficiently manage connections to your database.

8. Test Your Code: Finally, make sure to regularly test your code with realistic data volumes and usage patterns. This will help you find and address any performance bottlenecks or memory issues.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.333s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 2464 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.418s
    • Function auto-cleaned up by client
✓ New conversation stored and indexed for future retrieval

=== Step 6: Cross-Conversation Search ===
Searching for messages about 'ownership' across ALL conversations...

✓ Found 3 messages mentioning ownership:
  1. From conv_rust_programming: Rust's key features include: memory safety without garbage collection, zero-cost abstractions, ownership system, powerful type system, and excellent concurrency support.

  2. From conv_performance: Rust's ownership system provides zero-cost memory management. Use Box for heap allocation, Rc/Arc for shared ownership, and avoid cloning large data structures. The compiler optimizes away unnecessary allocations.

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
    ✓ Generated embedding: 1536 dimensions in 0.548s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.489s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.379s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.333s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.467s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.478s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.478s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.384s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.515s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.673s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.381s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.362s
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
    ✓ Generated embedding: 1536 dimensions in 0.369s
    • Function auto-cleaned up by client

→ Executing hybridSearch()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.097s
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

Writing memory-safe, high-performance database code involves a number of best practices, some of which can be derived from the context provided:

1. **Optimize Your Queries**: This is the most common way to improve the performance of your database. There are a number of ways to do this, including indexing, query rewriting, and denormalization. Indexing involves creating a data structure that improves the speed of data retrieval operations on a database table. Query rewriting involves changing your query to accomplish the same task more efficiently. Denormalization is the process of trying to improve the read performance of a database by adding redundant data or by grouping data.

2. **Choose the Appropriate Database Model**: As discussed in context 2 and 3, the choice of SQL vs NoSQL can significantly impact the performance of your database. If you need high write throughput, horizontal scalability, and flexibility with your schemas, NoSQL might be a better choice. If you need to perform complex queries and ACID transactions, SQL might be a better fit.

3. **Normalize Your Database**: As discussed in context 4 and 5, normalization is a process that improves your database's efficiency by reducing redundancy and improving data integrity. This can help improve the performance of your database by reducing the amount of space it takes up and making it easier to maintain.

4. **Manage Your Resources**: Memory-safe code is code that correctly manages memory resources. In the context of databases, this often means properly closing connections and result sets, avoiding memory leaks, and using resources in a way that doesn't lead to unexpected behavior.

5. **Concurrency Control**: Concurrency control ensures that correct results for concurrent operations are produced and that databases remain in a consistent state. This is especially important in high-performance databases where multiple operations may be happening at the same time.

6. **Use Caching**: Caching is a technique where you store the results of expensive queries in memory for quicker access in the future. This can greatly speed up your database performance by reducing the need to run expensive queries multiple times.

These techniques can help you write memory-safe, high-performance database code. However, the specific methods you use will depend heavily on the specifics of your project and the database system you're using.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.359s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 2419 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.483s
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
  ✓ Text search completed in 0.049s
✓ Found 3 messages mentioning ownership:
  1. From conv_rust_programming: Rust's key features include: memory safety without garbage collection, zero-cost abstractions, ownership system, powerful type system, and excellent concurrency support.

  2. From conv_performance: Rust's ownership system provides zero-cost memory management. Use Box for heap allocation, Rc/Arc for shared ownership, and avoid cloning large data structures. The compiler optimizes away unnecessary allocations.

  3. From conv_new_question: Writing memory-safe, high-performance database code involves a number of best practices, some of which can be derived from the context provided:

1. **Optimize Your Queries**: This is the most common way to improve the performance of your database. There are a number of ways to do this, including indexing, query rewriting, and denormalization. Indexing involves creating a data structure that improves the speed of data retrieval operations on a database table. Query rewriting involves changing your query to accomplish the same task more efficiently. Denormalization is the process of trying to improve the read performance of a database by adding redundant data or by grouping data.

2. **Choose the Appropriate Database Model**: As discussed in context 2 and 3, the choice of SQL vs NoSQL can significantly impact the performance of your database. If you need high write throughput, horizontal scalability, and flexibility with your schemas, NoSQL might be a better choice. If you need to perform complex queries and ACID transactions, SQL might be a better fit.

3. **Normalize Your Database**: As discussed in context 4 and 5, normalization is a process that improves your database's efficiency by reducing redundancy and improving data integrity. This can help improve the performance of your database by reducing the amount of space it takes up and making it easier to maintain.

4. **Manage Your Resources**: Memory-safe code is code that correctly manages memory resources. In the context of databases, this often means properly closing connections and result sets, avoiding memory leaks, and using resources in a way that doesn't lead to unexpected behavior.

5. **Concurrency Control**: Concurrency control ensures that correct results for concurrent operations are produced and that databases remain in a consistent state. This is especially important in high-performance databases where multiple operations may be happening at the same time.

6. **Use Caching**: Caching is a technique where you store the results of expensive queries in memory for quicker access in the future. This can greatly speed up your database performance by reducing the need to run expensive queries multiple times.

These techniques can help you write memory-safe, high-performance database code. However, the specific methods you use will depend heavily on the specifics of your project and the database system you're using.

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
    ✓ Generated embedding: 1536 dimensions in 0.391s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.678s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.501s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.363s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.364s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.325s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.366s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.659s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.314s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.361s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.257s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.385s
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
    ✓ Generated embedding: 1536 dimensions in 0.322s
    • Function auto-cleaned up by client

→ Executing HybridSearch()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.102s

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

Writing memory-safe, high-performance database code requires an understanding of both database design and optimization, as well as programming best practices. Here are some steps you can follow:

1. **Database Design**: Begin by designing your database in a way that minimizes redundancy and maximizes data integrity. As mentioned in Context 4 and 5, this can be achieved through database normalization, which involves dividing large tables into smaller ones and defining relationships between them with foreign keys.

2. **Choosing SQL or NoSQL**: Depending on your needs, you may want to use an SQL or NoSQL database. SQL databases are better for structured data and complex queries, and they support ACID transactions. NoSQL databases, on the other hand, are more suited for flexible schemas, horizontal scaling, high write throughput, and unstructured data. Your choice will affect how you write your code and how well your application performs (Context 2 and 3).

3. **Optimizing Queries**: To ensure high performance, you need to optimize your database queries. This could be achieved by using indexes, minimizing the use of joins, and taking advantage of stored procedures. Also, make sure to avoid N+1 query problems where one query is run, and then for each result of that query, another query is run.

4. **Memory Safety**: From a programming perspective, memory safety involves avoiding bugs that can lead to issues like buffer overflows and data races. Depending on the language you're using, this might involve careful management of pointers, judicious use of synchronization primitives, or choosing to use a language that includes built-in protections against these kinds of issues. 

5. **Connection Management**: Efficiently managing your database connections can also improve performance. Establishing a database connection can be a time-consuming operation, so connection pooling can be used to maintain a cache of database connections.

6. **Caching**: Caching can also be used to improve performance by storing the results of expensive queries for a certain period, reducing the need to access the database for repeated requests.

7. **Benchmarking and Monitoring**: Finally, always monitor the performance of your database code. Use profiling tools to identify any bottlenecks and continuously optimize based on your findings. 

Remember, the best practices for writing memory-safe, high-performance database code can vary significantly depending on your specific use case and the technologies you're using.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.294s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 2528 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.417s
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
  ✓ Text search completed in 0.071s

✓ Found 3 messages mentioning ownership:
  1. From conv_rust_programming: Rust's key features include: memory safety without garbage collection, zero-cost abstractions, ownership system, powerful type system, and excellent concurrency support.

  2. From conv_performance: Rust's ownership system provides zero-cost memory management. Use Box for heap allocation, Rc/Arc for shared ownership, and avoid cloning large data structures. The compiler optimizes away unnecessary allocations.

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
    ✓ Generated embedding: 1536 dimensions in 0.367s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.499s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 1.085s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.592s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.376s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.371s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.388s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.34s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.353s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.434s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.291s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.337s
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
    ✓ Generated embedding: 1536 dimensions in 0.599s
    • Function auto-cleaned up by client

→ Executing hybridSearch()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.098s

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
    ✓ Generated embedding: 1536 dimensions in 0.502s
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
  ✓ Text search completed in 0.055s

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

> @ekodb/ekodb-client@0.9.0 prepare
> npm run build


> @ekodb/ekodb-client@0.9.0 build
> tsc


up to date, audited 58 packages in 1s

15 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.9.0 build
> tsc

✅ [32mTypeScript client built![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning TypeScript SWR Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_api_user (rbla4llpJSEvLWwSQl-u2ozpxtgwnKikxmXhWXtCy2ZeyLCPqbRv_DW11ulWiBmysq9Ibzjt4ywMgAzvPqn2fA)

Step 2: First call - Cache miss, fetches from API
Result: {
  "records": [
    {
      "value": {
        "company": {
          "bs": "harness real-time e-markets",
          "name": "Romaguera-Crona",
          "catchPhrase": "Multi-layered client-server neural-net"
        },
        "id": 1,
        "name": "Leanne Graham",
        "address": {
          "geo": {
            "lat": "-37.3159",
            "lng": "81.1496"
          },
          "city": "Gwenborough",
          "suite": "Apt. 556",
          "street": "Kulas Light",
          "zipcode": "92998-3874"
        },
        "phone": "1-770-736-8031 x56442",
        "website": "hildegard.org",
        "email": "Sincere@april.biz",
        "username": "Bret"
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
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 7ms (served from cache)
Result (cached): {
  "records": [
    {
      "value": {
        "company": {
          "bs": "harness real-time e-markets",
          "name": "Romaguera-Crona",
          "catchPhrase": "Multi-layered client-server neural-net"
        },
        "id": 1,
        "name": "Leanne Graham",
        "address": {
          "geo": {
            "lat": "-37.3159",
            "lng": "81.1496"
          },
          "city": "Gwenborough",
          "suite": "Apt. 556",
          "street": "Kulas Light",
          "zipcode": "92998-3874"
        },
        "phone": "1-770-736-8031 x56442",
        "website": "hildegard.org",
        "email": "Sincere@april.biz",
        "username": "Bret"
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
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_with_reviews (M83wWd3PqaxRloqfJ9ciKpKWPdchSPow2VNSygj_Wu9a5J5QDAhm1T_wSeYkIs4VyvCyBBM59iNI0HB1DNganw)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {
      "value": {
        "brand": "Essence",
        "images": [
          "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
        ],
        "sku": "BEA-ESS-ESS-001",
        "stock": 99,
        "warrantyInformation": "1 week warranty",
        "rating": 2.56,
        "minimumOrderQuantity": 48,
        "availabilityStatus": "In Stock",
        "shippingInformation": "Ships in 3-5 business days",
        "dimensions": {
          "width": 15.14,
          "depth": 22.99,
          "height": 13.08
        },
        "discountPercentage": 10.48,
        "returnPolicy": "No return policy",
        "price": 9.99,
        "meta": {
          "updatedAt": "2025-04-30T09:41:02.053Z",
          "barcode": "5784719087687",
          "createdAt": "2025-04-30T09:41:02.053Z",
          "qrCode": "https://cdn.dummyjson.com/public/qr-code.png"
        },
        "weight": 4,
        "tags": [
          "beauty",
          "mascara"
        ],
        "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
        "category": "beauty",
        "id": 1,
        "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
        "reviews": [
          {
            "rating": 3,
            "reviewerName": "Eleanor Collins",
            "reviewerEmail": "eleanor.collins@x.dummyjson.com",
            "comment": "Would not recommend!",
            "date": "2025-04-30T09:41:02.053Z"
          },
          {
            "date": "2025-04-30T09:41:02.053Z",
            "reviewerName": "Lucas Gordon",
            "rating": 4,
            "reviewerEmail": "lucas.gordon@x.dummyjson.com",
            "comment": "Very satisfied!"
          },
          {
            "date": "2025-04-30T09:41:02.053Z",
            "reviewerEmail": "eleanor.collins@x.dummyjson.com",
            "rating": 5,
            "comment": "Highly impressed!",
            "reviewerName": "Eleanor Collins"
          }
        ],
        "title": "Essence Mascara Lash Princess"
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

=== ekoDB as Edge Cache - Simple Example ===

Creating edge cache function...
✓ Edge cache script created: OQnkEv8hoWw6iK1KkJKSUN2oV8_JYdhECmScSGUWhKRp5sRc9-sUtotC_XScpQLlRxYyrs5CH0U-8cIhp4mQ1w

Call 1: Cache miss (fetches from API)
Response time: 115ms
Result: {
  "records": [
    {
      "value": {
        "address": {
          "city": "Gwenborough",
          "street": "Kulas Light",
          "zipcode": "92998-3874",
          "suite": "Apt. 556",
          "geo": {
            "lng": "81.1496",
            "lat": "-37.3159"
          }
        },
        "email": "Sincere@april.biz",
        "name": "Leanne Graham",
        "username": "Bret",
        "company": {
          "name": "Romaguera-Crona",
          "catchPhrase": "Multi-layered client-server neural-net",
          "bs": "harness real-time e-markets"
        },
        "website": "hildegard.org",
        "phone": "1-770-736-8031 x56442",
        "id": 1
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
Response time: 6ms (19.2x faster!)
Result: {
  "records": [
    {
      "value": {
        "address": {
          "city": "Gwenborough",
          "street": "Kulas Light",
          "zipcode": "92998-3874",
          "suite": "Apt. 556",
          "geo": {
            "lng": "81.1496",
            "lat": "-37.3159"
          }
        },
        "email": "Sincere@april.biz",
        "name": "Leanne Graham",
        "username": "Bret",
        "company": {
          "name": "Romaguera-Crona",
          "catchPhrase": "Multi-layered client-server neural-net",
          "bs": "harness real-time e-markets"
        },
        "website": "hildegard.org",
        "phone": "1-770-736-8031 x56442",
        "id": 1
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
📦 Built wheel for abi3 Python ≥ 3.8 to /Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.9.0-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel for current platform...[0m
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.9.0-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
Successfully installed ekodb-client-0.9.0
✅ [32mPython client package built and installed![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning Python SWR Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_github_user (yL5-u9zzfSmb7q2V4sRzpOv6pEGQcJpzR77YTx96D2s4Uxaq4Rm7gRrNGusnEMEBPWuygRFKcEmUjjxT0Lv5IQ)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 156ms
Result: [
  {
    "cached_at": "1769534407",
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
        "followers": 280824,
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
    "id": "l2SnDFU2Un7tdJNgHT0MBTlWQhRLSd4q44eZXuHous-xD95SQ6myrhnjbRwxpNT1FtyZKmMFCY66GsE1q1uyfw"
  }
]
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 96ms (1.6x faster!)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (y2sDzsosk9DJLef9M6S3pwVjpp6MZ6WIVekQ7ZdpRldYgvb_YELyzM8g7salKvBIkAj-rPgYBSZsAOeOZkR1Xw)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: [
  {
    "enriched_at": "1769534407",
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
    "id": "-clF3ivCzEz80pTCSBkx14EQ7J1lP_s5z6U8jSNcsbSJlrMSdD-rN7mfjpES1Stiq6fW7i0hGqWpw3GYvbwbpA"
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
✓ Created SWR script: fetch_github_user (3zYMJ6AsfQcJcf8JB_1AhnAgR0_VaXM-63iIMjc5jxkkyaTa0u9ytTPeU_b_4_CgESkyNy9YSiV5pL6USWTGEg)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 63.978959ms
Result: [
  {
    "cached_at": "1769534407",
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
        "followers": 280824,
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
    "id": "rmNpTWtma4RegqlbpFjP1LGRPkAjn5HjUQ59YOklnQpwOva79GD55-K95EV9EWCI7dyEoAzDU6A0xp9BgZQETA"
  }
]
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 75.694833ms (0.8x faster!)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (f8eKo9lh0si-OLPP3yvyPvVhsphxeXKbttO-DJVKVLm42k90v5o25Cbux6---R576OQwJrF3L20ke2RXFRQ_ZQ)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: [
  {
    "enriched_at": "1769534407",
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
    "id": "eWCeszo3rhYVMaT7nhymKQSAoUWdpeNf4O_0xr4XmZsThyIZNLmKkLLWQkbbgJTxljULvJLB7XKG48BwwtlKYQ"
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
✅ [32mClient build complete![0m

[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
[36mRunning Rust SWR Examples...[0m
[36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/swr_pattern`
=== ekoDB SWR (Stale-While-Revalidate) Pattern ===

Step 1: Create SWR function that acts as edge cache
✓ Created SWR script: fetch_github_user (4ZUadpCb4DKISP-XGpZ_8u8cxboKLHgVCMwpqNPB-nYGKP_xBLmhQ_aFqd1voMLW2IvDfpDRUwszVRfQwa0WDg)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 161ms
Result: {
  "cached_at": "1769534407",
  "id": "B-KFBU91_CHrZmk3ijOQylJP3PkxQQ8jhNx0oMr7wkNz0SvUHs9nHS0AGdI6jn-cJUDn1UAqzA4Tyqbrsza3sA",
  "data": {
    "type": "Object",
    "value": {
      "public_repos": 11,
      "hireable": null,
      "blog": "",
      "gravatar_id": "",
      "node_id": "MDQ6VXNlcjEwMjQwMjU=",
      "login": "torvalds",
      "followers_url": "https://api.github.com/users/torvalds/followers",
      "organizations_url": "https://api.github.com/users/torvalds/orgs",
      "public_gists": 1,
      "created_at": "2011-09-03T15:26:22Z",
      "email": null,
      "html_url": "https://github.com/torvalds",
      "repos_url": "https://api.github.com/users/torvalds/repos",
      "type": "User",
      "following": 0,
      "starred_url": "https://api.github.com/users/torvalds/starred{/owner}{/repo}",
      "twitter_username": null,
      "updated_at": "2026-01-13T07:10:05Z",
      "url": "https://api.github.com/users/torvalds",
      "received_events_url": "https://api.github.com/users/torvalds/received_events",
      "following_url": "https://api.github.com/users/torvalds/following{/other_user}",
      "name": "Linus Torvalds",
      "id": 1024025,
      "avatar_url": "https://avatars.githubusercontent.com/u/1024025?v=4",
      "gists_url": "https://api.github.com/users/torvalds/gists{/gist_id}",
      "bio": null,
      "followers": 280824,
      "location": "Portland, OR",
      "subscriptions_url": "https://api.github.com/users/torvalds/subscriptions",
      "user_view_type": "public",
      "events_url": "https://api.github.com/users/torvalds/events{/privacy}",
      "company": "Linux Foundation",
      "site_admin": false
    }
  }
}
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 105ms (1.5x faster!)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (ZKwaaArp8tVo5G_CN4IHXmY1aMjDkEh41J1GWke0-XVrXBfYwW54JOGGDhVFO6Me6Eix-CSbPkKjWz1gMvN3dg)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: null
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
✓ Created SWR script: fetch_github_user (_-CaFhzZM4c0-fZon6AUMHPORDA2No7ZQPiwvBw8c8nr_QZMvAaLIfNGjoUTs71SV-GC7InWUwmFABCo_7GjgQ)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 175ms
Result: [{"cached_at":"1769534415361","data":{"type":"Object","value":{"id":1024025,"hireable":null,"public_gists":1,"url":"https://api.github.com/users/torvalds","created_at":"2011-09-03T15:26:22Z","followers":280824,"avatar_url":"https://avatars.githubusercontent.com/u/1024025?v=4","following_url":"https://api.github.com/users/torvalds/following{/other_user}","html_url":"https://github.com/torvalds","type":"User","updated_at":"2026-01-13T07:10:05Z","received_events_url":"https://api.github.com/users/torvalds/received_events","followers_url":"https://api.github.com/users/torvalds/followers","site_admin":false,"name":"Linus Torvalds","events_url":"https://api.github.com/users/torvalds/events{/privacy}","bio":null,"subscriptions_url":"https://api.github.com/users/torvalds/subscriptions","company":"Linux Foundation","login":"torvalds","following":0,"gravatar_id":"","user_view_type":"public","email":null,"blog":"","starred_url":"https://api.github.com/users/torvalds/starred{/owner}{/repo}","location":"Portland, OR","node_id":"MDQ6VXNlcjEwMjQwMjU=","organizations_url":"https://api.github.com/users/torvalds/orgs","twitter_username":null,"public_repos":11,"gists_url":"https://api.github.com/users/torvalds/gists{/gist_id}","repos_url":"https://api.github.com/users/torvalds/repos"}},"id":"AJ43P-ZvSemSSB6oyW-FST_HMc_wMTI-sKgNTu0NwtHbsMdg4mYDYverCH9yLkb3jdgfJFlJjMGUgMHNc7Rbww"}]
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 68ms
Cache hit was 2.6x faster!

✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (kPknlCrw4anTohOPBHujpnh-caRvFIjca-aB4c6gEqgSVJf6_1fgQOa48S4WTjtwuP8xUfYUmMGYq_PdS795XQ)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: []
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
   ⏱️  Duration: 86.198875ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "address": {
      "geo": {
        "lat": "-37.3159",
        "lng": "81.1496"
      },
      "city": "Gwenborough",
      "street": "Kulas Light",
      "zipcode": "92998-3874",
 ...

Second call (cache hit - from cache):
   ⏱️  Duration: 9.277958ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "company": {
      "bs": "harness real-time e-markets",
      "catchPhrase": "Multi-layered client-server neural-net",
      "name": "Romaguera-Crona"
    },
    "website": "hildega...
   🚀 Cache speedup: 9.6x faster!

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

> @ekodb/ekodb-client@0.9.0 prepare
> npm run build


> @ekodb/ekodb-client@0.9.0 build
> tsc


up to date, audited 58 packages in 1s

15 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities

> @ekodb/ekodb-client@0.9.0 build
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
   Department: {"type":"String","value":"engineering"}

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
    "company": {
      "bs": "harness real-time e-markets",
      "name": "Romaguera-Crona",
      "catchPhrase": "Multi-layered client-server neural-net"
    },
    "website": "hildega...

Second call (cache hit - from cache):
   ⏱️  Duration: 5ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "company": {
      "bs": "harness real-time e-markets",
      "name": "Romaguera-Crona",
      "catchPhrase": "Multi-layered client-server neural-net"
    },
    "website": "hildega...
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
📦 Built wheel for abi3 Python ≥ 3.8 to /Users/tek/Development/ekoDB/ekodb-client/ekodb-client-py/target/wheels/ekodb_client-0.9.0-cp38-abi3-macosx_11_0_arm64.whl
📦 [36mInstalling Python wheel for current platform...[0m
Processing ./ekodb-client-py/target/wheels/ekodb_client-0.9.0-cp38-abi3-macosx_11_0_arm64.whl
Installing collected packages: ekodb-client
  Attempting uninstall: ekodb-client
    Found existing installation: ekodb_client 0.9.0
    Uninstalling ekodb_client-0.9.0:
      Successfully uninstalled ekodb_client-0.9.0
Successfully installed ekodb-client-0.9.0
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
   ⏱️  Duration: 8.2ms
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
   ⏱️  Duration: 7.3ms
   📊 Records: 1
   📦 Data: {
      "value": {
            "address": {
                  "city": "Gwenborough",
                  "geo": {
                        "lat": "-37.3159",
                        "lng": "81.1496"
    ...
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

Using KV cache + CallFunction for fast cache-aside pattern...

✅ Saved reusable function: fetch_and_store_user (uses KV)
✅ Saved SWR function using composition: swr_user

First call (cache miss - will fetch from API):
   ⏱️  Duration: 4.984625ms
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
   ⏱️  Duration: 5.057792ms
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
   🚀 Cache speedup: 0.8x faster!

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
   Department: {"type":"String","value":"engineering"}

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
    "company": {
      "bs": "harness real-time e-markets",
      "name": "Romaguera-Crona",
      "catchPhrase": "Multi-layered client-server neural-net"
    },
    "website": "hildega...

Second call (cache hit - from cache):
   ⏱️  Duration: 6ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "company": {
      "bs": "harness real-time e-markets",
      "name": "Romaguera-Crona",
      "catchPhrase": "Multi-layered client-server neural-net"
    },
    "website": "hildega...
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

