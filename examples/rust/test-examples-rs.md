make test-examples-rust
🧪 [36mRunning Rust examples (direct HTTP/WebSocket)...[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/simple_crud`
✓ Authentication successful

=== Insert Document ===
Inserted: Object {"id": String("Waw2CKvloj2OGKkxdvA30q5qM2Z97Vz-prAglekfk3-hNyD7EDcpHjMOAF7HZCwP0eX6ewUxV_clDokoQoyacA")}

=== Find by ID ===
Found: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("Waw2CKvloj2OGKkxdvA30q5qM2Z97Vz-prAglekfk3-hNyD7EDcpHjMOAF7HZCwP0eX6ewUxV_clDokoQoyacA"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}

=== Find with Query ===
Found documents: Array [Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("Waw2CKvloj2OGKkxdvA30q5qM2Z97Vz-prAglekfk3-hNyD7EDcpHjMOAF7HZCwP0eX6ewUxV_clDokoQoyacA"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}]

=== Update Document ===
Updated: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("Waw2CKvloj2OGKkxdvA30q5qM2Z97Vz-prAglekfk3-hNyD7EDcpHjMOAF7HZCwP0eX6ewUxV_clDokoQoyacA"), "name": Object {"type": String("String"), "value": String("Updated Record")}, "value": Object {"type": String("Integer"), "value": Number(100)}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/simple_websocket`
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: "ggUkVE8p-4-sgFujbNnfoo55fNFXQjXjuC4CFyYjUOz6nF6tI0-pZ_LBzcVVwNTxaLqn3-UX4dKYH11ySN0p4A"

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
        "id": "ggUkVE8p-4-sgFujbNnfoo55fNFXQjXjuC4CFyYjUOz6nF6tI0-pZ_LBzcVVwNTxaLqn3-UX4dKYH11ySN0p4A",
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
Collection created with first record: "iBLFeU27B65-LAUkjch8Ry236kJuboeGCmYRT_qMHS-Gx_7IYkUgvXNrZiYnrbQyuSksOLIwnPoNKj4OoEZT3g"

=== List Collections ===
Total collections: 4
Sample collections: ["websocket_test", "test_collection", "demo_collection", "batch_users"]

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
✓ Inserted document: "WGaAQDYKr1zPiX1naZHlu8Ple9xfptu1FQ6fjHlzbjseBN3uHUoP_ZGNKpQ1HQ1qCnt13eX-c3oT31_e5ibBZQ"

=== Insert Document with TTL (5 minutes - integer) ===
✓ Inserted document: "gMV-A-9FlIci23yq2PUjp1Asg8x_oLIwMw09p6GNT-rRev9l7oc2jSj-IDr6Tbgc2LnMNazM9LYmV2PF7-w3Ow"

=== Insert Document with TTL (30 minutes - duration string) ===
✓ Inserted document with duration string TTL: "C2swj67HYKC47-BiYasc0XPbNLyV3NiVPUztNjDHXYeV6LDepcb9e9Z3ehf2NpZQycoDMORR-bK37zyxxHMDWw"

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
✓ Inserted document with TTL: String("xayRPxTCGelcgdjWpeBZVA5E-LwihwMPlM9nx3UN1ZUDUc8T6Dbdv_wFqfspO0ldMR78Sk-_H3wzhpLK6KIblA")

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket

✓ WebSocket TTL example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/http_functions`
🚀 ekoDB Scripts Example (Rust/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: 7DY-tz2oZxENv4cD1dGeFK-y2gV8uRlmpw7QDT6AlWHXWfs-kSzUctILVywXcz9nLYITQYffjaqnVtUG-v78Bg
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: "fpfPNxdRPCVW1EdP0cREisdsBlWyrI0-wf2kvglS3uq67Z2mEfDe1COWWO7dz48vN3QAfzbAXfeJkHIrSOk_cQ"
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Complex Filter with Multiple Conditions

✅ Script saved: lQT2qMi5Jqehhm5nufln31uUHkDoB2622lSHUMwToyFt4H9XPENEzeTVBeCusNhoCdPTzP10BVKb8XnvQFuoYw
📊 Found 3 users (status=active, score>50, sorted by score)

📝 Example 4: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: ba_kubk4Ydvewghy2wuNQJM7N5yiUi0Zg25oHGaVvPgyHgvlGdTVkI1IfZ9VXGPqi5Oidat27HjQuB5SCyWO4Q
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
Created Alice: $1000 - ID: 9wACB1pbekZi3fjssy4k8YZ2-0zUoBnF-728ELa69TAv0bDqISP0PYuO9AHf0jqOuej5GB3E74L6kpttrUp0SA
Created Bob: $500 - ID: adbkXU9hlBJXZZV4HltmV5JeW_RLf_3B99rnZrKygtdzgFxafuj40PnTZTNh1AKET8bpnLCna02Q4dkIxYiDQQ

=== Example 1: Begin Transaction ===
Transaction ID: 62bfda78-de00-4bef-a73f-6e84f67932c3

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
New transaction: bf58df6d-dfd8-412e-afab-124dd33b986a
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
Inserted: Record { fields: {"id": String("Aq_EFBG72Uql9xvqFgU3sVYu4g_Rf_MV5G00303YGB6BvxcRjvDPT4DPxfFetzqCYnzl7w2LVmpV6_VugWIuQg")} }

=== Find by ID ===
Found: Record { fields: {"user_id": Object({"value": String("550e8400-e29b-41d4-a716-446655440000"), "type": String("String")}), "name": Object({"value": String("Test Record"), "type": String("String")}), "data": Object({"value": String("aGVsbG8gd29ybGQ="), "type": String("String")}), "embedding": Object({"type": String("Array"), "value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)])}), "created_at": Object({"type": String("DateTime"), "value": String("2026-01-04T22:28:48.883807Z")}), "price": Object({"type": String("Float"), "value": Float(99.99)}), "value": Object({"type": String("Integer"), "value": Integer(42)}), "id": String("Aq_EFBG72Uql9xvqFgU3sVYu4g_Rf_MV5G00303YGB6BvxcRjvDPT4DPxfFetzqCYnzl7w2LVmpV6_VugWIuQg"), "tags": Object({"value": Array([String("tag1"), String("tag2"), String("tag3")]), "type": String("Array")}), "metadata": Object({"type": String("Object"), "value": Object({"key": String("value"), "nested": Object({"deep": Boolean(true)})})}), "categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])}), "active": Object({"type": String("Boolean"), "value": Boolean(true)})} }

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Some("Test Record")
  value (Integer): Some(42)
  active (Boolean): Some(true)
  price (Decimal): Some(99.99)
  created_at (DateTime): Some("2026-01-04T22:28:48.883807Z")
  user_id (UUID): Some("550e8400-e29b-41d4-a716-446655440000")
  tags (Array): 3 items
  metadata (Object): 2 keys
  embedding (Vector): 5 dims
  categories (Set): 2 items
  data (Bytes): 11 bytes

=== Find with Query ===
Found documents: [Record { fields: {"categories": Object({"value": Array([String("electronics"), String("computers")]), "type": String("Array")}), "name": Object({"type": String("String"), "value": String("Test Record")}), "metadata": Object({"value": Object({"key": String("value"), "nested": Object({"deep": Boolean(true)})}), "type": String("Object")}), "user_id": Object({"value": String("550e8400-e29b-41d4-a716-446655440000"), "type": String("String")}), "value": Object({"value": Integer(42), "type": String("Integer")}), "active": Object({"type": String("Boolean"), "value": Boolean(true)}), "price": Object({"value": Float(99.99), "type": String("Float")}), "data": Object({"value": String("aGVsbG8gd29ybGQ="), "type": String("String")}), "tags": Object({"value": Array([String("tag1"), String("tag2"), String("tag3")]), "type": String("Array")}), "id": String("Aq_EFBG72Uql9xvqFgU3sVYu4g_Rf_MV5G00303YGB6BvxcRjvDPT4DPxfFetzqCYnzl7w2LVmpV6_VugWIuQg"), "created_at": Object({"value": String("2026-01-04T22:28:48.883807Z"), "type": String("DateTime")}), "embedding": Object({"type": String("Array"), "value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)])})} }]

=== Update Document ===
Updated: Record { fields: {"user_id": Object({"type": String("String"), "value": String("550e8400-e29b-41d4-a716-446655440000")}), "categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])}), "metadata": Object({"value": Object({"nested": Object({"deep": Boolean(true)}), "key": String("value")}), "type": String("Object")}), "created_at": Object({"value": String("2026-01-04T22:28:48.883807Z"), "type": String("DateTime")}), "id": String("Aq_EFBG72Uql9xvqFgU3sVYu4g_Rf_MV5G00303YGB6BvxcRjvDPT4DPxfFetzqCYnzl7w2LVmpV6_VugWIuQg"), "value": Object({"type": String("Integer"), "value": Integer(100)}), "name": Object({"value": String("Updated Record"), "type": String("String")}), "tags": Object({"value": Array([String("tag1"), String("tag2"), String("tag3")]), "type": String("Array")}), "data": Object({"type": String("String"), "value": String("aGVsbG8gd29ybGQ=")}), "price": Object({"type": String("Float"), "value": Float(99.99)}), "active": Object({"type": String("Boolean"), "value": Boolean(true)}), "embedding": Object({"type": String("Array"), "value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)])})} }

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
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
Collection created with first record: "sOWZVnNvhia1eFrpnxhRxtnljgaC6PG76B1oMZjy8rLIT5o7QvXdAqJZPd9xMAmGBPuSO3YCzYha4XxGblZERg"

=== List Collections ===
Total collections: 9
Sample collections: ["client_collection_management_rust", "ttl_cache", "users", "websocket_test", "test_collection"]

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
✓ Inserted document: "W2VBnF2JUFILw2rHIBDoBbPJF0ySeQ3iVzMT8TJwQ03hDLxXXbXEUMtwBhcqavJGdJ9T3e-H0PNxNxV1b5iRTw"

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: Some(String("oKv0XoQvtG71VQisdvsJ3mGvFKC3gJNgyZ1ttt6uWrYI6eRVruOYuTExXqTzHWRM5fm9_jdxUUMnxE5-bVj-LQ"))

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
✓ Inserted test record: Q0O8m7BcVhlLkrRZ1Ntlv5yp9-ZTcOB6GxbV-6fE-fI15ZE4olPGqYKkpl8tlyGkZQnpoWDdKwRMyeTtiSONKw

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
✓ Inserted document with TTL: Some(String("tzhN-kJKIuUHzHQah9SsOkE33mtOl1k6ndYpmCalogMjrWNJqoFDvOgMMsBfAz1o6N6kLT5FOVWzbfDVslkFdA"))

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
  - Some(Object({"type": String("String"), "value": String("Alice")}))
  - Some(Object({"value": String("Bob"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("David")}))

=== Range Query (age >= 28 AND age < 35) ===
✓ Found 3 users in age range
  - Some(Object({"type": String("String"), "value": String("David")}))
  - Some(Object({"value": String("Eve"), "type": String("String")}))
  - Some(Object({"value": String("Bob"), "type": String("String")}))

=== IN Operator ===
✓ Found 4 users with status active or pending
  - Some(Object({"value": String("Alice"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("David")}))
  - Some(Object({"type": String("String"), "value": String("Eve")}))
  - Some(Object({"type": String("String"), "value": String("Bob")}))

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
  1. Some(Object({"type": String("String"), "value": String("Bob")}))
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
     Matched: ["title.value", "title", "description", "description.value"]
  2. Score: 2.0000
     Title: Some(String("Python for Data Science"))
     Matched: ["description.value", "description"]
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
Execution time: 1ms
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_schema_management`
=== ekoDB Schema Management Example ===

=== Creating Collection with Schema ===
✓ Created collection 'schema_client_rust' with schema

=== Inserting Valid Documents ===
✓ Inserted user 1: Some(String("sdNTP_n_tFIDcyxImrnS2-ZStel8TJn1kxe4M-5nGJ2w_Yh-c0wHt3BJiDYJnKzdL3kbzsytGMd5lrWDPzOTzQ"))
✓ Inserted user 2: Some(String("VaDAUBB5vVvhkn9KP2QD1UNeDKkkk1VdyVa8XETlHfPsS7cMIFn7m8HkL7CROuu9Xlzg93ySZOT7xWFMuSyq9w"))

=== Getting Schema ===
✓ Schema for schema_client_rust:
  - age: Integer
  - email: String
    (required)
  - status: String
  - title: String
    (required)

=== Listing Collections ===
✓ Total collections: 9
  Sample: ["ttl_cache", "users", "websocket_test", "test_collection", "scripts__ek0_testing"]

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
  - Mouse ($25) by Alice Johnson
  - Laptop ($1200) by Alice Johnson

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
✓ Created session: DhIh4JKtzyz-P_VwbAkLSXZCjZtY0TlcbaSniFvYPnmyL6GKGRUO4aUqqhc3KeIZqzMADTi3NFtcGZeLK9UFTQ

=== Sending Chat Message ===
Message ID: Wr0rmfWIPkOnljs6I3QreuU09Lbxf2Dkc4dJmixi6ZoqzrWzOfEeabVQceZzpLVbuXMilS9XF3lC-jUTZjXIfg

=== AI Response ===
Response 1: ekoDB is a high-performance database with intelligent caching, real-time capabilities, and AI integration. It has several key features including an AI Chat Integration which allows you to query your database using natural language and get AI-powered responses with relevant context. It also supports full-text search, vector search, and hybrid search with automatic context retrieval, enhancing the user's ability to find and access data conveniently and efficiently.

=== Context Used (3 snippets) ===

Snippet 1:
  Collection: client_chat_basic_rust
  Score: 0.6333
  Matched Fields: ["title", "category", "content"]
  Record: Object {"category": String("features"), "content": String("The chat feature allows you to query your database using natural language and get AI-powered responses with relevant context."), "id": String("mZ89IpGvwGw6Qf8rMUQe-IDcgkDOknSYUYv-CTP05y7Zt1k25EOIPhm-rnY-gwY49gAlZ5mJyHF3XypJn8UoEw"), "title": String("AI Chat Integration")}

Snippet 2:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["category", "content", "title"]
  Record: Object {"category": String("features"), "content": String("ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval."), "id": String("1uFf4zEb0VYKb8nZK5hZ6aIsi6aRsavz0QuODVAn2Dk7NGAOTtqxelm_cXHR70xMCK-vA4NR_bxJbOTw7eyLPQ"), "title": String("Search Features")}

Snippet 3:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["content", "title"]
  Record: Object {"category": String("documentation"), "content": String("ekoDB is a high-performance database with intelligent caching, real-time capabilities, and AI integration."), "id": String("7UZfXCii_qtMZfU-nx2JOXgd1eJZmaSsYwEi5csPAWzPIcpe9qdcOR9ao71tci_-5LX_It3vfE_09wgReETbtA"), "title": String("Introduction to ekoDB")}

Execution Time: 2702ms

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_chat_sessions`
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: mA61Yn2joaX-S0mdSdsaajZiffk0LtVAs6i9T-bDcik-YewWD2js2py8GGeFemTIxR6pDVrFEWI6WsaCqFGwUA
=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information provided, the product available is ekoDB. It is a high-performance database product with AI capabilities and it is priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: QKHL1Tc7R0FGa65TzZq-oCZhiiqlCxQnyQpjdL-sXDqERqczVN263nuw6oy6uKGNyHjLgSgdCsRiqJ-7c8ws-A
  Parent: mA61Yn2joaX-S0mdSdsaajZiffk0LtVAs6i9T-bDcik-YewWD2js2py8GGeFemTIxR6pDVrFEWI6WsaCqFGwUA

=== Listing Sessions ===
✓ Found 3 sessions
  Session 1: QKHL1Tc7R0FGa65TzZq-oCZhiiqlCxQnyQpjdL-sXDqERqczVN263nuw6oy6uKGNyHjLgSgdCsRiqJ-7c8ws-A (Untitled)
  Session 2: mA61Yn2joaX-S0mdSdsaajZiffk0LtVAs6i9T-bDcik-YewWD2js2py8GGeFemTIxR6pDVrFEWI6WsaCqFGwUA (Untitled)
  Session 3: DhIh4JKtzyz-P_VwbAkLSXZCjZtY0TlcbaSniFvYPnmyL6GKGRUO4aUqqhc3KeIZqzMADTi3NFtcGZeLK9UFTQ (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: QKHL1Tc7R0FGa65TzZq-oCZhiiqlCxQnyQpjdL-sXDqERqczVN263nuw6oy6uKGNyHjLgSgdCsRiqJ-7c8ws-A

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_chat_advanced`
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: c__ZmnF2c-tw6Om2ikM_V1MX5qS9qc7gZL64DTQB0EOZWv1OhazsTsDybQMw1qTQWnL2It8iEcxwK9Hibf3CrA

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information available, one of the products we have is "ekoDB", which is a high-performance database product. It is priced at $99.

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
✓ Created second session: zgZmlmVFLWGNLEdEB26uHagZbr2ZF7802mz_wjqfZqon0azZ3gofLs6k7o8B_odODTl89yzN_di8oFEaFf0y8g
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
     Running `target/debug/examples/client_functions`
=== ekoDB Rust Client - Scripts Example ===

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: RGzxSzSfk5bgI_WBMyl_z3Pg5iyB7bUSCoPe0Wl8HhcGvN6H5fLkqxnI4cokC8rIo0DDJlN-asb48btDsY2-eQ
📊 Found 20 active users

📝 Example 2: Parameterized Script

✅ Script saved: 4mSrUgmjEW7a5Kd5hUrp6YUmbj7-Jixt4vssVFoTJORG7-dQEIpyC0IZjMGgR5hqkRLLyvtERP4_UE5r4gnqYg
📊 Found 20 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: U9cjarjaJPFZRzb87koi_mKBcM25F3laJ-NkaJMrisj8yIKndjlZ5BKLAP6b3BCYXgdBipaicxOspsAkmUVyXA
📊 Statistics: 2 groups

📝 Example 4: Script Management

📋 Total scripts: 6
🔍 Retrieved script: Get Active Users
✏️  Script updated
🗑️  Script deleted

ℹ️  Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.


✅ All examples completed!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_transactions`
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: yPwYLSXXNKAIYrElTPqs1FRb9CBR-b2U5xo-5RP_2yNxwEvbOT94e30qzv_plEKq8it3h3u53T2Kowsu-DerAw
Created Bob: $500 - ID: Qan5_M0D1AKcraVW52dBySZP8au_vsnMFkZ69ivgp0pGBTBbT2fGCYu2ajqqpPfKOHfMRCz_pwdv_jfqoqs2uA

=== Example 1: Begin Transaction ===
Transaction ID: e9237fdd-7ed9-4004-ac5c-cb7c5b6a9498

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: "Active"
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: f97f7998-80ea-481e-adb5-3995d2873c8d
Status before rollback: "Active"
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
✅ [32mRust client examples complete![0m
