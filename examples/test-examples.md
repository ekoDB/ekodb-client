make test-examples
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
✅ [32mAll Rust integration tests complete![0m
🧪 [36mRunning Python examples (direct HTTP/WebSocket)...[0m
=== Simple CRUD Operations (Direct HTTP) ===

✓ Authentication successful

=== Insert Document ===
Inserted: {'id': 'L9cpBD0Eljx2Ncf21z-_jzCQ6vf2MRIHkp4lvfrqdFGz-up_gg7uugVcVhFK3J9n6JJagH1DJjGG_rFo-ojnaw'}

=== Find by ID ===
Found: {'id': 'L9cpBD0Eljx2Ncf21z-_jzCQ6vf2MRIHkp4lvfrqdFGz-up_gg7uugVcVhFK3J9n6JJagH1DJjGG_rFo-ojnaw', 'name': {'value': 'Test Record', 'type': 'String'}, 'value': {'value': 42, 'type': 'Integer'}, 'active': {'value': True, 'type': 'Boolean'}}

=== Find with Query ===
Found documents: [{'active': {'type': 'Boolean', 'value': True}, 'value': {'type': 'Integer', 'value': 42}, 'name': {'value': 'Test Record', 'type': 'String'}, 'id': 'L9cpBD0Eljx2Ncf21z-_jzCQ6vf2MRIHkp4lvfrqdFGz-up_gg7uugVcVhFK3J9n6JJagH1DJjGG_rFo-ojnaw'}]

=== Update Document ===
Updated: {'value': {'value': 100, 'type': 'Integer'}, 'active': {'type': 'Boolean', 'value': True}, 'name': {'type': 'String', 'value': 'Updated Record'}, 'id': 'L9cpBD0Eljx2Ncf21z-_jzCQ6vf2MRIHkp4lvfrqdFGz-up_gg7uugVcVhFK3J9n6JJagH1DJjGG_rFo-ojnaw'}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: d2VJG5iv4Y2GM8EF5v5Bvs8hZucupbWnljtU8V09E6kHsVSL76g8IaS5XdJ9hRAvPBx73kHkr6R-756nGUvxHg

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
      },
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "d2VJG5iv4Y2GM8EF5v5Bvs8hZucupbWnljtU8V09E6kHsVSL76g8IaS5XdJ9hRAvPBx73kHkr6R-756nGUvxHg",
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

✅ Script saved: KPBv7JXT5qmEhVw3uFi45QSVAnE3axQWH4Tb4J7Jeu5I-qJwlk3waHXfzEiuTJWb6C1ikfNOTnoFgA8M2h0E1A
📊 Found 30 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: A3xs7UqCZfx7yUIZy4HdfuS-IHE-fy1E2DnhwZMJ-zcXKyit-BnYHwawjXDMKDtpg1tQ6Ux6swPvk3GtiseEDg
📊 Page 1: Found 3 users (limit=3, skip=0)

📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: 9z-ifzRfhMLtn5iwJdnvH1b22E5zbCFrDwJkSYFkl-VC57Peq1_L_5eJaEmrXKHLm6m0bbFof4yx-xO3UCShxQ
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {'status': 'active', 'count': 15, 'avg_score': 60.0, 'max_score': 100}
   {'max_score': 90, 'count': 15, 'avg_score': 50.0, 'status': 'inactive'}

📝 Example 4: Function Management

📋 Total scripts: 8
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
cache:product:1: {'name': 'Product 1', 'price': 29.99}
cache:product:2: {'name': 'Product 2', 'price': 39.989999999999995}
cache:product:3: {'name': 'Product 3', 'price': 49.989999999999995}

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key successfully deleted (not found)

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
=== Collection Management (Direct HTTP) ===

✓ Authentication successful

=== Create Collection (via insert) ===
Collection created with first record: Qfzh6EF258UEJGL2tDDTXY5tOdF2-JWmgiQPfG3OHV42Qd0Tzy-o1ZRZcjS_e2kGjp_p0NclMkdbNIbpRyh_pA

=== List Collections ===
Total collections: 11
Sample collections: ['ttl_cache', 'users', 'websocket_test', 'chat_messages__ek0_testing', 'test_collection']

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
Created Alice: $1000 - ID: e_CzE76gy-iHzi4_-y-ZqtW5FgWLQwTU3yJuI81oXbZ1ZA2mVcnNakkTBELCst31I3dsW2KhJ9r0gUjCgTQfRQ
Created Bob: $500 - ID: nddoH0ndf-3OB--MGWeDzJgphSL9yCmvxQRvZKU2T-uXQrokxz9DOFMMtUAylhG7B1TOqv3WvQUW6md7RAzYpw

=== Example 1: Begin Transaction ===
Transaction ID: 832d3e33-7c8c-4bb3-b51d-fbb605341bb9

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
New transaction: 20b61822-5769-405d-bc83-74ac4bde0513
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: sfS-nBmWTm_Wi80-QC5sdaWYoo9Os3UpSPPyb9xcXZ-TGLonMHzjXRHpT9M52TgqXcOHwRVwjbsx-ouXE-J8yg
Created Bob: $500 - ID: BXX2wsVqKVwrD5BWgBF5IwtYwNmqupSEN-A_ggDzCHeJMyx-VVYTBK9YsINBGp0T1s5lHm9hrG-lG4-dbWlwKQ

=== Example 1: Begin Transaction ===
Transaction ID: c8dfe23b-d7c9-4d76-9911-1a5b7d54ce36

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
Created Alice: $1000 - ID: rjgZnM-wNOLAnITFU0oaveeTYbgl9lVJ_i_OsejkiZDGefUl3rXQkTUnmE3RktsN1pil3FiIy1U80MeUAzpyNA
Created Bob: $500 - ID: cd83vRnzkeSyIBa9vQINnzJXb6kDeQrfyzXbbr9cm_oDMz_iMhaCnXObCwLFTjCw9wr5max4QLrFWQ5UZWIaiw

=== Example 1: Begin Transaction ===
Transaction ID: b7e58552-f391-4979-99ad-cd8eee8020d9

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
New transaction: 1adf9671-3492-49bc-a9cb-b5b9e84b85b5
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: hMnsp-cKHmhXAHi96W6B2Nm64YxgBln4dw_OO0GU9ZkmZe9Oh579y_WPfenP59FY_fgutN_p5F2yc4PUNLtQ-A
Created Bob: $500 - ID: uXRshnYu77TKhjPYuaNAk8HCzsjxHEsUJU_PNIey6agKDEGW7SK7jHcQ5TVIdNivJmImbxPFankGMya0g7rv3w

=== Example 1: Begin Transaction ===
Transaction ID: 8c85499c-dd1c-47f4-9407-ee564cff109e

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
Created Alice: $1000 - ID: v5Km4S-cfp8pb-dnCQ6oPAmkqLnXZcWk3xoa6SkdwrNfCLJQynFz53bFzr8keJzW7bxBONz8kMIR4QA0tr1OlQ
Created Bob: $500 - ID: qTAg_sgYqRtTNHyhYAWWz49Z5jQPEntRhkDBNn3eiT49n5YY6-dNULzklpEGQL_zPZ4_V8F2Pptc6aLwC2q_Sg

=== Example 1: Begin Transaction ===
Transaction ID: e2bd48ff-3d34-4c32-8269-633f63790d42

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
New transaction: c53f9483-8cf0-42a6-bf07-148ea8c10800
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: $700

=== Cleanup ===
✓ All transaction examples completed
=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: q-le2yU1IFfFKzEcyDwAQ8qF-GuKSbZDM1rrBjCL-U6EwqGYZ8GZudyeSbBNTM2XWaVrg786KuGlSLY-_MJPXw
Created Bob: $500 - ID: u8Qz67d1BlaBXeDyBEk9rZdr5bq9Xfecc5FDE_YbzAAKuqSEGPd5VMC33Ou-M-WPf1nLaHycT-2zZ1TBmkAFbA

=== Example 1: Begin Transaction ===
Transaction ID: c7ddea9d-268b-4df1-8b10-453ee480b1c8

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
   ✅ Script saved: T9tSacurfr9Ifa0nyRRBIMMoOc4UE7p8dOPgI-fLY7mhq0Qnf2FunVSsjZPJfFk8oHC-QnhJ_OPeLDdXoTUlEA

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: NVBaoRYSGbZ6UPndQjmtFQ9LwN7nGcWHJaDDc58eqDCYW283TWam65FO3VJeyni9u-LP7_eBy7nuCIBZ5YPIhA
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}
   📋 Email: {'value': 'alice@example.com', 'type': 'String'}
   📋 Status: {'type': 'String', 'value': 'pending'}
   📋 Credits: {'type': 'Integer', 'value': 0}

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: vKkiZCHouPMY6lB835UoYI1oELsOM_RQaLQl0rn-6b2uyyfWvj7rbEvO9M5jKXVwWaOGsjKZd_rprqaI3jjKZA

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
   ✅ Script saved: 4aDZAZuAZCLYMbuitVrVFpZAU_e8u1C0ECQZrxBs9RUzSzSiReO2Cb5zeZgL7mG4JFFLPKJmhWB0dqKJVVeo-Q

2️⃣ Calling Script (Query + Update Credits + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 Credits updated to: {'value': 0, 'type': 'Integer'}
   📋 Status: {'value': 'active', 'type': 'String'}
   📋 Name: {'type': 'String', 'value': 'Alice Smith'}

============================================================
📝 Script 4: Query Before Delete + Delete + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: bGJBLRkFUgqCtnMe1Or2LlJoHwg98W5sFrQ73OJHP1xKcRqDofs-fohJ86p0191xIpGrjQ8W-1ODO3v8QqnvNA

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: T9tSacurfr9Ifa0nyRRB...
   ✅ Deleted script: vKkiZCHouPMY6lB835Uo...
   ✅ Deleted script: 4aDZAZuAZCLYMbuitVrV...
   ✅ Deleted script: bGJBLRkFUgqCtnMe1Or2...
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
  Output: Document ID = MeB5hdZHxsKnsFHpjxeyb_Zel7T1nyGF9A3E5kak7-CWdhymBaTwMvOfvddVnBgBxm0EMYQVu5qQJEixEeN90Q
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: find_by_id(MeB5hdZHxsKnsFHpjxeyb_Zel7T1nyGF9A3E5kak7-CWdhymBaTwMvOfvddVnBgBxm0EMYQVu5qQJEixEeN90Q)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: find_by_id(MeB5hdZHxsKnsFHpjxeyb_Zel7T1nyGF9A3E5kak7-CWdhymBaTwMvOfvddVnBgBxm0EMYQVu5qQJEixEeN90Q)
  Output: Error (expected) - Find failed: Record not found
  ✓ PASS: Document expired (not found error)

═══════════════════════════════════════════════════════════
TEST 2: Multiple Documents with Different TTLs
═══════════════════════════════════════════════════════════

[Step 1] Insert 3 documents with different TTLs
  Inserted: 3s TTL=wpnOJF9l, 10s TTL=q8ds6yD7, Permanent=GNHZft8k
  ✓ PASS: All documents inserted

[Step 2] Wait 5 seconds (3s doc should expire)
  ✓ PASS: 3s TTL document expired
  ✓ PASS: 10s TTL document still exists
  ✓ PASS: Permanent document still exists

[Step 3] Wait 7 more seconds (10s doc should expire)
  ✓ PASS: 10s TTL document expired
  ✓ PASS: Permanent document still exists

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
  Output: Document ID = oSsSVC9TYHx15JyD0OAN2TTq8_66tICTFSR9Md9MsHup9NTwvxFh_TjoutSkCPUjj8dnDTWP0LkjBlOynY_qkw
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: find_by_id(oSsSVC9TYHx15JyD0OAN2TTq8_66tICTFSR9Md9MsHup9NTwvxFh_TjoutSkCPUjj8dnDTWP0LkjBlOynY_qkw)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: find_by_id(oSsSVC9TYHx15JyD0OAN2TTq8_66tICTFSR9Md9MsHup9NTwvxFh_TjoutSkCPUjj8dnDTWP0LkjBlOynY_qkw)
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
Inserted: {'id': '-o6rjOx0bvwBc1yxgk4ZESk6sizVlFWxpmst1LpEd5w4h3yRnoM1b-MWcPg8S7X7cVg5rvaHj6kpWmX31TA2gw'}

=== Find by ID ===
Found: {'metadata': {'type': 'Object', 'value': {'nested': {'deep': True}, 'key': 'value'}}, 'price': {'type': 'Float', 'value': 99.99}, 'active': {'type': 'Boolean', 'value': True}, 'data': {'type': 'String', 'value': 'aGVsbG8gd29ybGQ='}, 'value': {'value': 42, 'type': 'Integer'}, 'tags': {'value': ['tag1', 'tag2', 'tag3'], 'type': 'Array'}, 'embedding': {'type': 'Array', 'value': [0.1, 0.2, 0.3, 0.4, 0.5]}, 'id': '-o6rjOx0bvwBc1yxgk4ZESk6sizVlFWxpmst1LpEd5w4h3yRnoM1b-MWcPg8S7X7cVg5rvaHj6kpWmX31TA2gw', 'name': {'value': 'Test Record', 'type': 'String'}, 'categories': {'type': 'Array', 'value': ['electronics', 'computers']}, 'user_id': {'type': 'String', 'value': '550e8400-e29b-41d4-a716-446655440000'}, 'created_at': {'value': '2026-01-04T17:29:58.710058', 'type': 'String'}}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): True
  price (Decimal): 99.99
  created_at (DateTime): 2026-01-04 17:29:58.710058
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): ['tag1', 'tag2', 'tag3']
  metadata (Object): {'nested': {'deep': True}, 'key': 'value'}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): ['electronics', 'computers']
  data (Bytes): 11 bytes
Plain record: {'metadata': {'nested': {'deep': True}, 'key': 'value'}, 'price': 99.99, 'active': True, 'data': 'aGVsbG8gd29ybGQ=', 'value': 42, 'tags': ['tag1', 'tag2', 'tag3'], 'embedding': [0.1, 0.2, 0.3, 0.4, 0.5], 'id': '-o6rjOx0bvwBc1yxgk4ZESk6sizVlFWxpmst1LpEd5w4h3yRnoM1b-MWcPg8S7X7cVg5rvaHj6kpWmX31TA2gw', 'name': 'Test Record', 'categories': ['electronics', 'computers'], 'user_id': '550e8400-e29b-41d4-a716-446655440000', 'created_at': '2026-01-04T17:29:58.710058'}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {'created_at': {'value': '2026-01-04T17:29:58.710058', 'type': 'String'}, 'value': {'value': 100, 'type': 'Integer'}, 'metadata': {'value': {'nested': {'deep': True}, 'key': 'value'}, 'type': 'Object'}, 'price': {'value': 99.99, 'type': 'Float'}, 'tags': {'value': ['tag1', 'tag2', 'tag3'], 'type': 'Array'}, 'name': {'type': 'String', 'value': 'Updated Record'}, 'categories': {'value': ['electronics', 'computers'], 'type': 'Array'}, 'user_id': {'type': 'String', 'value': '550e8400-e29b-41d4-a716-446655440000'}, 'data': {'value': 'aGVsbG8gd29ybGQ=', 'type': 'String'}, 'embedding': {'type': 'Array', 'value': [0.1, 0.2, 0.3, 0.4, 0.5]}, 'active': {'value': True, 'type': 'Boolean'}, 'id': '-o6rjOx0bvwBc1yxgk4ZESk6sizVlFWxpmst1LpEd5w4h3yRnoM1b-MWcPg8S7X7cVg5rvaHj6kpWmX31TA2gw'}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
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

=== Create Collection (via insert) ===
Collection created with first record: "Ima8WlHg8cnB7eTw92BBag5QsNO16LuOBUOek_b7b_n8pzGW6N5DsOegP4BiBB_ypAe9wlVCaBsSJI0T8b2WMA"

=== List Collections ===
Total collections: 10
Sample collections: ['ttl_cache', 'websocket_test', 'chat_messages__ek0_testing', 'test_collection', 'scripts__ek0_testing']

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: False

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: A4Z7R535QDG6fZi26kOWDtToIo_Np80jxl9wommmpSmf7br9VRtGQaAShTlf3JhbCVIg39hmZP8MReV6KS0WfQ

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: XkP9XmlOZIcFwvQnNAWDQRGPFATdVl64s4Pg1F-WQ5LASYdoX_8GIsNWIdla2TebfJpXG-2Zg_4Dw0q9u_h_Xw

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

=== Inserting Test Data ===
✓ Inserted test record: DsLAlFAk8qvjYXpi0BGSzqX51ul77RA5mR_QqNvnFIMwD2dpWeOwPt_org1WZCMQl5zZtCG1s1FfmicxaE88Zw

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: 4qOBs_c7Xtd4aScXUA-9PAYgVswNoNPLx4rL9xlzhThC7VtUzRt1OjhuyCOwhqJRsg6oivOOzL7OBmD1Bl1XuA

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 5 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
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
  1. Score: 25.740, Matched: email.value, name, email, name.value
  2. Score: 12.540, Matched: name, name.value

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: bio, title, title.value, bio.value
  2. Score: 26.400, Matched: title, bio, title.value, bio.value
  3. Score: 26.400, Matched: title, bio, title.value, bio.value
  4. Score: 26.400, Matched: bio, title.value, bio.value, title

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title.value, bio, title, bio.value
  2. Score: 39.600, Matched: bio.value, title, title.value, bio
  3. Score: 39.600, Matched: title, title.value, bio, bio.value
  4. Score: 39.600, Matched: bio, bio.value, title.value, title

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
  1. Score: 0.778
  2. Score: 0.774
  3. Score: 0.769

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 1.738, Matched: title.value, title, content, content.value
  2. Score: 1.142, Matched: title, content, title.value, content.value
  3. Score: 0.544, Matched: 

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
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: LJ5cZyqHoXs52cGWUxESsBowN9WD_4okFMLRbpD8-ggFUhYqKZPFaRlUDCmTo8J2LkXgnMPzPinMwwzCxLmR8w

=== Sending Chat Message ===
Message ID: tCIVlD1ch28TaM-M3mXJDWXk_jHVGbeXwHGt19m14KQFVJq2YpcliZRGMweffhmsS3bnROh8Dzqu15MhCIpHXw

=== AI Response ===
Based on the information available, there are three products available:

1. ekoDB: This is a high-performance database product with AI capabilities. It is priced at $99.

2. ekoDB Pro: This is the enterprise edition of the product with advanced features. It is priced at $299.

3. ekoDB Cloud: This is a fully managed cloud database service product. It is priced at $499.

Execution Time: 3025ms

=== Token Usage ===
Prompt tokens: 609
Completion tokens: 86
Total tokens: 695

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: j6jNCKZTN-N6CxuK9wi6SxzdbbBMKWEGWrM8AniwBQpg_SWyD86sGW2NZ5rI63Z7R7RlpXjn8nsPwHQcT3VslQ

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information available, the product that is available is "ekoDB". It is a high-performance database product and is priced at $99.

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: aaKLSKj40uqNyixJJzdzSUcjBDOspmlzLEWTt8fF9wYPvXhpdXARdyITA0AwQ2XhgzDVChN_UxJiool54iA60g
  Parent: j6jNCKZTN-N6CxuK9wi6SxzdbbBMKWEGWrM8AniwBQpg_SWyD86sGW2NZ5rI63Z7R7RlpXjn8nsPwHQcT3VslQ

=== Listing Sessions ===
✓ Found 6 sessions
  Session 1: aaKLSKj40uqNyixJJzdzSUcjBDOspmlzLEWTt8fF9wYPvXhpdXARdyITA0AwQ2XhgzDVChN_UxJiool54iA60g (Untitled)
  Session 2: j6jNCKZTN-N6CxuK9wi6SxzdbbBMKWEGWrM8AniwBQpg_SWyD86sGW2NZ5rI63Z7R7RlpXjn8nsPwHQcT3VslQ (Untitled)
  Session 3: LJ5cZyqHoXs52cGWUxESsBowN9WD_4okFMLRbpD8-ggFUhYqKZPFaRlUDCmTo8J2LkXgnMPzPinMwwzCxLmR8w (Untitled)
  Session 4: zgZmlmVFLWGNLEdEB26uHagZbr2ZF7802mz_wjqfZqon0azZ3gofLs6k7o8B_odODTl89yzN_di8oFEaFf0y8g (Untitled)
  Session 5: mA61Yn2joaX-S0mdSdsaajZiffk0LtVAs6i9T-bDcik-YewWD2js2py8GGeFemTIxR6pDVrFEWI6WsaCqFGwUA (Untitled)
  Session 6: DhIh4JKtzyz-P_VwbAkLSXZCjZtY0TlcbaSniFvYPnmyL6GKGRUO4aUqqhc3KeIZqzMADTi3NFtcGZeLK9UFTQ (Untitled)

=== Deleting Branch Session ===
✓ Deleted branch session: aaKLSKj40uqNyixJJzdzSUcjBDOspmlzLEWTt8fF9wYPvXhpdXARdyITA0AwQ2XhgzDVChN_UxJiool54iA60g

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: nyVWDeAHuPqxSgnotyLNk9_AC-2-G8KC8UyJ7dJfngs1xbENkT2WtIqa_PuOAO9xM4mnGCwX2T9STGBfGMnC4w

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information available, the product that is available is ekoDB, which is a high-performance database product. It is priced at $99.

✓ Second message sent

Debug: Found 4 messages
Debug: First message keys: dict_keys(['context_snippets', 'token_usage', 'chat_id', 'role', 'id', 'updated_at', 'content', 'created_at'])
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
✓ Created second session: Io01vXhsW8BkPYUCUUTLdDWRPT21aGynTNwl8dKLlJ6X24xSeN2mBCOkKES2IJnJ9F6h51vrnyjWWQLYmuyOMQ
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
🚀 ekoDB Scripts Example (Python)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: FkH0MpldIfEtEx7Guu6vQ1mFUMBfTffZ0AOIH4zMqJRSkH1ydwWe5vYONzt2M1if_NFXiyfOzdlCUhtzxEHHew
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved: MpxPQFJtzpSLL3U77ZaITLFQ-qaU98WswRgfI6IDNRPJZEpfJB8H-Pn13Eel9SuwHPoS1N2LWOtfrucFM4Gu9A
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: 5MVqI6WnUKgbTY1Fersp0gQ-HCIl9UfMH8J9zkCxE18SChZE_nBoD6v_Jyte1_9b56StYQsoz61qMTiiwhgj9g
📊 Statistics: 2 groups
   {'avg_score': 60.0, 'count': 5, 'status': 'active'}

   {'avg_score': 50.0, 'count': 5, 'status': 'inactive'}

📝 Example 4: Script Management

📋 Total scripts: 10
🔍 Retrieved script: Get Active Users
✏️  Script updated
🗑️  Script deleted

ℹ️  Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.

✅ All examples completed!
🚀 ekoDB Python Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved: CDDSlFpkoRk3ZnPqmGrHrJo3Ack6gDl_sfOVHXXU1q3cU4h_mFZk93KIMuS-RuAcM4UT9GK_BKcYTGJhzOFr2g
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
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: QylQN2h49AIkA4l8TLreKE3a_g1sJstt1X8t4LmnDNsHaYdaJLEsKR_RSM5IEIaWx7ja_NpkrHj4r3w41SRmXg
Created Bob: $500 - ID: OB1N_ViWmVN9Ej4PicD0BE-71SDG4NK5WVc7qj7CW2KL0oao7jgyTsZ5Bukac7qugxDYou7PoIBTxWC3_ydowg

=== Example 1: Begin Transaction ===
Transaction ID: 10d1cc1e-69fb-4c88-99a2-1f25c422e76e

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 5665db11-1db9-47b8-a43f-8d6d6697fbe2
Status before rollback: Active
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
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
Inserted: map[id:hoDT7JQ2XpK1kyzMHhtxjFobaIMKbwCpyX8KXxV3CKGFGvZxG3dOwZYpY0teV64fqJAmXvB1VRqQ1_0sULtOqA]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] id:hoDT7JQ2XpK1kyzMHhtxjFobaIMKbwCpyX8KXxV3CKGFGvZxG3dOwZYpY0teV64fqJAmXvB1VRqQ1_0sULtOqA name:map[type:String value:Test Record] value:map[type:Integer value:42]]

=== Find with Query ===
Found documents: map[]

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] id:hoDT7JQ2XpK1kyzMHhtxjFobaIMKbwCpyX8KXxV3CKGFGvZxG3dOwZYpY0teV64fqJAmXvB1VRqQ1_0sULtOqA name:map[type:String value:Updated Record] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
[32m✓ simple_crud.go completed successfully[0m
[34m
=== Running simple_websocket.go ===[0m
=== Simple WebSocket Operations (Direct API) ===

✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: US1URz1oElP11pCefhxXASu8-cl5XivMknbWJiWq1kXVtN-5hRHQyja-29vCYjgUA0cRpJ_hJZcKdqayTZhPww

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
      },
      {
        "active": {
          "type": "Boolean",
          "value": true
        },
        "id": "d2VJG5iv4Y2GM8EF5v5Bvs8hZucupbWnljtU8V09E6kHsVSL76g8IaS5XdJ9hRAvPBx73kHkr6R-756nGUvxHg",
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
        "id": "US1URz1oElP11pCefhxXASu8-cl5XivMknbWJiWq1kXVtN-5hRHQyja-29vCYjgUA0cRpJ_hJZcKdqayTZhPww",
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

✅ Script saved: Jrr9ewxBb2ZA2zGTfgDHMm4q3wYSvZ0uznztg7bGC--MbEP8tfxLSeb-dp8DOt4zIvWI96qXwi-5Vca24q5cfA
📊 Found 20 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: IiefJwTNsPNiSZujejg8A7aWd1ra818A6sYjYGzQ_llkvL-xcdpG3mYKXT5gX3agtdvo6flu9-nnyFJ-QPQiJg
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: F8Q7zB6I0PNLbfMh8J9fDCj5YmwXmHA-KrIDDDpOKMFrF_HbIZeOuglm65zV6xKSfa7immA8FKJGlyeNw6niMQ
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"avg_score":50,"count":10,"max_score":90,"status":"inactive"}
   {"avg_score":60,"count":10,"max_score":100,"status":"active"}

📝 Example 4: Function Management

📋 Total scripts: 12
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
Collection created with first record: za2YBWtPRoNpEXlk32vYnPp7IOPRyk6vxVeH2BB_m184IhlnX9U7KgCgORYs06VeORU7ui9j546L_u8sFM5OKg

=== List Collections ===
Total collections: 15
Sample collections: [ttl_cache users websocket_test chat_messages__ek0_testing test_collection]

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
Created Alice: $1000 - ID: 39s8XrkvJsBubPF_7hjUQet04MLLnED_eNiKlTGGMGDYvHOFAEMorSI_Iq75qus1gBtBNFnwzrWHtSRr7qFmqQ
Created Bob: $500 - ID: TYcUJPSBhmiM9aa5fmSfaBk4Aii2ImAzq1olE_GAfWRRMe2H2hPEcYmmm2jzBBllhouODuzywUy96BAxzw9-jg

=== Example 1: Begin Transaction ===
Transaction ID: 9ebeb764-60e6-4554-8d73-35fc042a2364

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
New transaction: 7cf8bc22-7e25-481e-a367-558adc7e1d22
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
   ✅ Script saved: 4ENwO71Wwx9C-KrQT7mbMRDQg4jjT-UxxW5w5jnKW6zesK9HDG3xlL4P-QJ8oEEuJgWmAEVv90RFIpFNtHhwOw

2️⃣ Calling Script (Insert + Verify)...
   ✅ Script executed: 2 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   ✅ Found 1 record(s)
   📋 User ID: n8CHzyd_fhTYM_08DmRW6jKdFmg8OYyy3rSKnAhk2KRL80lPc21kQdDIMp809MX761YgXVUXQPVnTRtxXCWmSA
   📋 Name: map[type:String value:Alice Smith]
   📋 Email: map[type:String value:alice@example.com]
   📋 Status: map[type:String value:pending]
   📋 Credits: map[type:Integer value:0]

============================================================
📝 Script 2: Query + Update + Verify
============================================================

1️⃣ Saving Script...
   ✅ Script saved: FtE0581ce19U7QUKeJz0v-DRltdZH5NEkKpFCOH6y5qdMwruAEd_JibfGwZPw4iVreGTWGaknKiL_dABCK5LHA

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
   ✅ Script saved: i-gY6g4xGQ0pW3yc7H0AH9B1eHGGP7Md2ikfAzfUktTZAHfftt53U4SIk0OIzEpta9ezX_QKXcA1jOHBCxBJzg

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
   ✅ Script saved: JWOtdrh611kMyL5UVOtfNC3nM63RYK5tMmXKLfb1bw1vDKczkFCiN8Zt3u0AMPh3R17eWdbC4thx90KzEp8lOw

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: 4ENwO71Wwx9C-KrQT7mb...
   ✅ Deleted script: FtE0581ce19U7QUKeJz0...
   ✅ Deleted script: i-gY6g4xGQ0pW3yc7H0A...
   ✅ Deleted script: JWOtdrh611kMyL5UVOtf...
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

2026/01/04 17:30:28 No .env file found, using defaults
✓ Client connected

═══════════════════════════════════════════════════════════
TEST 1: Document TTL Expiration
═══════════════════════════════════════════════════════════

[Step 1] Insert document with 3 second TTL
  Input: {name: 'TTL Test', value: 'should expire'}
  TTL: 3s
  Output: Document ID = HxBqnZ-4W4RO6YoHeSFy5CE5ziXshc78UZ6qktf0T6UDeX19zs-nisE86xW0PSBR1ds-91ZUE_BSiWWjiWxONA
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: FindByID(HxBqnZ-4W4RO6YoHeSFy5CE5ziXshc78UZ6qktf0T6UDeX19zs-nisE86xW0PSBR1ds-91ZUE_BSiWWjiWxONA)
  Output: Found document with name = map[type:String value:TTL Test]
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: FindByID(HxBqnZ-4W4RO6YoHeSFy5CE5ziXshc78UZ6qktf0T6UDeX19zs-nisE86xW0PSBR1ds-91ZUE_BSiWWjiWxONA)
  Output: Error (expected) - request failed with status 404: ��Record has been deleted
  ✓ PASS: Document expired (not found error)

═══════════════════════════════════════════════════════════
TEST 2: Multiple Documents with Different TTLs
═══════════════════════════════════════════════════════════

[Step 1] Insert 3 documents with different TTLs
  Inserted: 3s TTL=04r2kqzI, 10s TTL=pIWo5_iY, Permanent=_idygP-o
  ✓ PASS: All documents inserted

[Step 2] Wait 5 seconds (3s doc should expire)
  ✓ PASS: 3s TTL document expired
  ✓ PASS: 10s TTL document still exists
  ✓ PASS: Permanent document still exists

[Step 3] Wait 7 more seconds (10s doc should expire)
  ✓ PASS: 10s TTL document expired
  ✓ PASS: Permanent document still exists

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

2026/01/04 17:30:47 No .env file found, using defaults
✓ Client connected

═══════════════════════════════════════════════════════════
TEST: WebSocket TTL Expiration
═══════════════════════════════════════════════════════════

[Step 1] Insert document with 3 second TTL
  Input: {name: 'WS TTL Test', value: 'should expire'}
  TTL: 3s
  Output: Document ID = vQhAnH3IZOth5rxV1ecfIx1FM_GS1WCKz51BzRkroHx1LkzOXTI6_yhjMO8yf5JEhQeDMhXaiBoQnImBcb6leA
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: FindByID(vQhAnH3IZOth5rxV1ecfIx1FM_GS1WCKz51BzRkroHx1LkzOXTI6_yhjMO8yf5JEhQeDMhXaiBoQnImBcb6leA)
  Output: Found document with name = map[type:String value:WS TTL Test]
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: FindByID(vQhAnH3IZOth5rxV1ecfIx1FM_GS1WCKz51BzRkroHx1LkzOXTI6_yhjMO8yf5JEhQeDMhXaiBoQnImBcb6leA)
  Output: Error (expected) - request failed with status 404: ��Record has been deleted
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
Inserted: map[id:yGn8nPXGLgsRv2r1D1-GqYqV_mu-bQH1xWi-_UAH416YGvx1hnfRvKRepIb7hKNiNOtcr5-XS5t8nec6I28b3A]

=== Find by ID ===
Found: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-01-04T22:30:55Z] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:yGn8nPXGLgsRv2r1D1-GqYqV_mu-bQH1xWi-_UAH416YGvx1hnfRvKRepIb7hKNiNOtcr5-XS5t8nec6I28b3A metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Test Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:42]]

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 0
  active (Boolean): true
  price (Decimal): 99.990000
  created_at (DateTime): 2026-01-04 22:30:55 +0000 UTC
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1 tag2 tag3]
  metadata (Object): map[key:value nested:map[deep:true]]
  embedding (Vector): [0.1 0.2 0.3 0.4 0.5]
  categories (Set): [electronics computers]
  data (Bytes): 11 bytes
Plain record: map[active:true categories:[electronics computers] created_at:2026-01-04T22:30:55Z data:aGVsbG8gd29ybGQ= embedding:[0.1 0.2 0.3 0.4 0.5] id:yGn8nPXGLgsRv2r1D1-GqYqV_mu-bQH1xWi-_UAH416YGvx1hnfRvKRepIb7hKNiNOtcr5-XS5t8nec6I28b3A metadata:map[key:value nested:map[deep:true]] name:Test Record price:99.99 tags:[tag1 tag2 tag3] user_id:550e8400-e29b-41d4-a716-446655440000 value:42]

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: map[active:map[type:Boolean value:true] categories:map[type:Array value:[electronics computers]] created_at:map[type:DateTime value:2026-01-04T22:30:55Z] data:map[type:String value:aGVsbG8gd29ybGQ=] embedding:map[type:Array value:[0.1 0.2 0.3 0.4 0.5]] id:yGn8nPXGLgsRv2r1D1-GqYqV_mu-bQH1xWi-_UAH416YGvx1hnfRvKRepIb7hKNiNOtcr5-XS5t8nec6I28b3A metadata:map[type:Object value:map[key:value nested:map[deep:true]]] name:map[type:String value:Updated Record] price:map[type:Float value:99.99] tags:map[type:Array value:[tag1 tag2 tag3]] user_id:map[type:String value:550e8400-e29b-41d4-a716-446655440000] value:map[type:Integer value:100]]

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
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

=== Create Collection (via insert) ===
Collection created with first record: D0VfzeWpEyAg4mdrr3mB1qGxIzuSrTfl4OKV_xQGSduJzzz7W_FYIHfMvUKJoBOTmFaIJ8XILgUAVTYrI0QYkg

=== List Collections ===
Total collections: 14
Sample collections: [client_collection_management_go ttl_cache websocket_test chat_messages__ek0_testing test_collection]

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: I59o_ug3zNuxP1qQyG-6XuBmr142W7oR5Uw7IuxM9dosKq56_52JY6SUQmXxaOyg0CjBvJEYsSb7BQATIEeTQQ

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: ob9vx1TbMsr1Jgh4POhpJvSeDB2H9w-yJ5fVkqrqkkFZUDhqNRy16UCfqd5c3SuN1qodh8lz2TLNn2-DQCflTw

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

=== Inserting Test Data ===
✓ Inserted test record: twkefirzEY4ou6HbEmlDW6Vh0DbyWw6RF0HEKKiXK-8JP-sKK8XX8jwTcm5bRxomqGisLEqD-ASLO5mu1sWi5g

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: Pv7Y3NV2Kzks3BfFE8rmyI9C478HP8b_5VeG95_ftbEfwpEd2KdtG5VooSkW21N9szSNna1qfwp1QN49PwG8Fw

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 5 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: Qyvsy0PzVbiBeghb9hUiKkm9jkqwn0zqoSrh-sUl0FxvsU7IdcSFGYFMzXIP0bqVZJLQrKxGOYxDiL50740Fuw

=== Sending Chat Message ===
Message ID: G6qd-a-7T9j5SK2SuPxwqnQmfk6lflQtCZT53MrEbmCPq4MSb8fi_GRMsd-EF2yyHM8p3DG5oqiWyTOX46IWtA

=== AI Response ===
The available products and their prices are as follows:

1. ekoDB: This is a high-performance database product with AI capabilities. The price of this product is $99.

2. ekoDB Pro: This is an Enterprise edition product with advanced features. It is priced at $299.

3. ekoDB Cloud: This is a fully managed cloud database service product. The cost of this product is $499.

=== Context Used (3 snippets) ===
  Snippet 1: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:A high-performance database product with AI capabilities id:QHE4Fzf6BzLZ2_7o10QaIxQyowUWJ9B3W0U7sThM_dcz1SP3mLxZYAUBOkMUQAf78lPQ1ClY4GPNrcML4jXlaA name:ekoDB price:99] score:0.1111111111111111]
  Snippet 2: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Enterprise edition product with advanced features id:CS0jE23tMSkjekiT6j_VbzyPGE2jFfk_Oof7nwIwTLYkj272DQVk7UgxRTqJEJO26jFJN-jGx3I_32IU8zNq2Q name:ekoDB Pro price:299] score:0.1111111111111111]
  Snippet 3: map[collection:client_chat_basic_go matched_fields:[description] record:map[description:Fully managed cloud database service product id:QzsmVSm2UEUX5x7--spGpYE1TOst9Hg-OrCb9ZNrOXFCSjslJo_tWCD4KjF7r8QSqxQIyYOk-17YA6t6qpRVlQ name:ekoDB Cloud price:499] score:0.1111111111111111]

Execution Time: 2775ms

=== Token Usage ===
Prompt tokens: 610
Completion tokens: 86
Total tokens: 696

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: Gu_xIGD5MefdoNTeJDkJ0I4d3RSsP9ER4PGpxn03S00E8P1XDAGDW9F6VOqwiNH5nVOvIOO25KjqjHwrAQDw4w

=== Sending Messages ===
✓ Message 1 sent
  Response: Based on the information provided, the available product is ekoDB. It is a high-performance database product that costs $99.

✓ Message 2 sent
  Response: The price of the ekoDB, a high-performance database product, is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: Ss7H6_sdaz85IWQS28SqRuXgfrQ8rlcG5DCeeNeR35czVlG7EMZTAOUG3ln83VYvXAZ5EHObpVmCVb_dLwUJLw
  Parent: Gu_xIGD5MefdoNTeJDkJ0I4d3RSsP9ER4PGpxn03S00E8P1XDAGDW9F6VOqwiNH5nVOvIOO25KjqjHwrAQDw4w

=== Listing Sessions ===
✓ Found 9 sessions
  Session 1: Ss7H6_sdaz85IWQS28SqRuXgfrQ8rlcG5DCeeNeR35czVlG7EMZTAOUG3ln83VYvXAZ5EHObpVmCVb_dLwUJLw (Untitled)
  Session 2: Gu_xIGD5MefdoNTeJDkJ0I4d3RSsP9ER4PGpxn03S00E8P1XDAGDW9F6VOqwiNH5nVOvIOO25KjqjHwrAQDw4w (Untitled)
  Session 3: Qyvsy0PzVbiBeghb9hUiKkm9jkqwn0zqoSrh-sUl0FxvsU7IdcSFGYFMzXIP0bqVZJLQrKxGOYxDiL50740Fuw (Untitled)
  Session 4: Io01vXhsW8BkPYUCUUTLdDWRPT21aGynTNwl8dKLlJ6X24xSeN2mBCOkKES2IJnJ9F6h51vrnyjWWQLYmuyOMQ (Untitled)
  Session 5: j6jNCKZTN-N6CxuK9wi6SxzdbbBMKWEGWrM8AniwBQpg_SWyD86sGW2NZ5rI63Z7R7RlpXjn8nsPwHQcT3VslQ (Untitled)
  Session 6: LJ5cZyqHoXs52cGWUxESsBowN9WD_4okFMLRbpD8-ggFUhYqKZPFaRlUDCmTo8J2LkXgnMPzPinMwwzCxLmR8w (Untitled)
  Session 7: zgZmlmVFLWGNLEdEB26uHagZbr2ZF7802mz_wjqfZqon0azZ3gofLs6k7o8B_odODTl89yzN_di8oFEaFf0y8g (Untitled)
  Session 8: mA61Yn2joaX-S0mdSdsaajZiffk0LtVAs6i9T-bDcik-YewWD2js2py8GGeFemTIxR6pDVrFEWI6WsaCqFGwUA (Untitled)
  Session 9: DhIh4JKtzyz-P_VwbAkLSXZCjZtY0TlcbaSniFvYPnmyL6GKGRUO4aUqqhc3KeIZqzMADTi3NFtcGZeLK9UFTQ (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: Ss7H6_sdaz85IWQS28SqRuXgfrQ8rlcG5DCeeNeR35czVlG7EMZTAOUG3ln83VYvXAZ5EHObpVmCVb_dLwUJLw

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: m9jHcYUkhcwOfx9DOjIggjk8gqd0AAOZDufKhLBbQoPqEgPjHX5D1b3NfdrQ7qUD3qxdWrC-J8eUVHIeX3-PKA

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information I have, the available product is "ekoDB", which is a high-performance database product. It is priced at $99.

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
✓ Created second session: NZ3y677q6jvHnfJfB7cTEZQef4i2DBvbBXefm61pJxn-HBf9p7So7p4zJUjILngY864c_I2FcDV6_j51na4qAA
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
  1. Score: 0.763
  2. Score: 0.761
  3. Score: 0.748

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.704
  2. Score: 1.505
  3. Score: 0.299

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
🚀 ekoDB Scripts Example (Go Client)

✅ Client initialized

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: EfV84tbVU9_UwKIXGOiHykxXJOWEUmkkZ9VPbixL8sNzaz2AV-DhFbBayoUl7bY061IFCLUCIZej7bm1Qd0GTQ
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

📋 Total scripts: 14
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
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: uAz0RZXUo-aFO2RBl-wyeRH8GHkVVweyPifV6I08F9z46tG6um2kT1csHhc_XYwgjkdbKfbG9glfEzwZYTCxOQ
Created Bob: $500 - ID: 0INQG-q2NOfpiXhaXmhUfTVQazzJNAyPFFNI1FFplY8iJWRWhoNd2oAdQjTnSguz72ZtnPlW7ZCviaRjCFFAhA

=== Example 1: Begin Transaction ===
Transaction ID: f9c60b28-d29c-44cf-a739-251c05e091d9

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 6046a809-275e-420e-b6b0-9b14e85827a8
Status before rollback: Active
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
✅ [32mGo client examples complete![0m
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

added 1 package, removed 1 package, and audited 13 packages in 825ms

3 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'wbnlv7IPVfQLzd4Oo90Toe0WBX_Q9-wdqKNHGwymPD2dfWjNq4Jmkxglaesx0GvhTMr2YXr6eBQ8qT1Z-MkZLw'
}

=== Find by ID ===
Found: {
  categories: { value: [ 'electronics', 'computers' ], type: 'Array' },
  data: { value: 'aGVsbG8gd29ybGQ=', type: 'String' },
  active: { type: 'Boolean', value: true },
  created_at: { value: '2026-01-04T22:31:11.159Z', type: 'DateTime' },
  price: { value: 99.99, type: 'Float' },
  id: 'wbnlv7IPVfQLzd4Oo90Toe0WBX_Q9-wdqKNHGwymPD2dfWjNq4Jmkxglaesx0GvhTMr2YXr6eBQ8qT1Z-MkZLw',
  value: { type: 'Integer', value: 42 },
  name: { type: 'String', value: 'Test Record' },
  metadata: { type: 'Object', value: { nested: [Object], key: 'value' } },
  tags: { type: 'Array', value: [ 'tag1', 'tag2', 'tag3' ] },
  embedding: { type: 'Array', value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ] },
  user_id: { value: '550e8400-e29b-41d4-a716-446655440000', type: 'String' }
}

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Test Record
  value (Integer): 42
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): 2026-01-04T22:31:11.159Z
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [ 'tag1', 'tag2', 'tag3' ]
  metadata (Object): { nested: { deep: true }, key: 'value' }
  embedding (Vector): [ 0.1, 0.2, 0.3, 0.4, 0.5 ]
  categories (Set): [ 'electronics', 'computers' ]
  data (Bytes): 11 bytes
Plain record: {
  categories: [ 'electronics', 'computers' ],
  data: 'aGVsbG8gd29ybGQ=',
  active: true,
  created_at: '2026-01-04T22:31:11.159Z',
  price: 99.99,
  id: 'wbnlv7IPVfQLzd4Oo90Toe0WBX_Q9-wdqKNHGwymPD2dfWjNq4Jmkxglaesx0GvhTMr2YXr6eBQ8qT1Z-MkZLw',
  value: 42,
  name: 'Test Record',
  metadata: { nested: { deep: true }, key: 'value' },
  tags: [ 'tag1', 'tag2', 'tag3' ],
  embedding: [ 0.1, 0.2, 0.3, 0.4, 0.5 ],
  user_id: '550e8400-e29b-41d4-a716-446655440000'
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  embedding: { value: [ 0.1, 0.2, 0.3, 0.4, 0.5 ], type: 'Array' },
  tags: { value: [ 'tag1', 'tag2', 'tag3' ], type: 'Array' },
  price: { value: 99.99, type: 'Float' },
  categories: { type: 'Array', value: [ 'electronics', 'computers' ] },
  user_id: { type: 'String', value: '550e8400-e29b-41d4-a716-446655440000' },
  value: { type: 'Integer', value: 100 },
  created_at: { value: '2026-01-04T22:31:11.159Z', type: 'DateTime' },
  data: { value: 'aGVsbG8gd29ybGQ=', type: 'String' },
  metadata: { value: { nested: [Object], key: 'value' }, type: 'Object' },
  name: { value: 'Updated Record', type: 'String' },
  id: 'wbnlv7IPVfQLzd4Oo90Toe0WBX_Q9-wdqKNHGwymPD2dfWjNq4Jmkxglaesx0GvhTMr2YXr6eBQ8qT1Z-MkZLw',
  active: { value: true, type: 'Boolean' }
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
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
Total keys in store: 4

=== KV Delete ===
✓ Deleted key: session:user123
✓ Verified: Key exists after delete: false

=== Delete Multiple Keys ===
✓ Deleted 3 keys

✓ All KV operations completed successfully
✓ Client created

=== Create Collection (via insert) ===
Collection created with first record: nk6M0DROdHaT11ffY374uQsLhqjMtp6KL5f0dhCzajVpEh3IGldy69-YY0vCQbJEyOKtVEaQjs2NRRBwtL9chA

=== List Collections ===
Total collections: 18
Sample collections: ttl_cache,websocket_test,chat_messages__ek0_testing,client_collection_management_ts,test_collection

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: HtNhBt4EA_Us2yXUZrSCAJQe7fFzFBxPDe0HgoQnP9OW3mtQHal5hnY2y9RdVmuDTLOys044bgzXFnY9Q3LBOA

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: Xet_sVTaUa4fDct0XyvXmLOQ1iYY2vI5ikpEmgEvMEYzvDwK8i-MldxaQl_Wc2HCIF1E5cYP1NuKtgXsDq9GHw

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

=== Inserting Test Data ===
✓ Inserted test record: FmVg0_o_2Ct2pKSFBJb6pBB-1nZkSV2-nektxWPjVysHQJgBZVvJfhr-NObq5c2RkH1Dersc93XjcCNCvUQhig

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: RZH9ayJlEtO1U2g9X8oexgkYnKXZ0B-qyLwl6oPWmGjISDC96-SAVN23ZerBr4r_bhHGPKgOjHwgZ33L-xv4BQ

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 5 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
=== ekoDB Chat Basic Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: k7sW1byJTicQYj0VGGVxPgDYMIhyRWcHbXxcasr2-4ia6FiATCUPrkmgSr3sy9XBd0edmgaMTUOkgHhVPmarFg

=== Sending Chat Message ===
Message ID: 8zoJ9Bmpvd-5vFnYMEjocAqzfWh-lQz7kRFC4tjesIQ_VSWd9zsdNX0ZsjfF5moJrtV5wKW5suPb2YEk5Bt4NA

=== AI Response ===
We have three products available:

1. "ekoDB Pro" is an enterprise edition product with advanced features. Its price is $299.
2. "ekoDB Cloud" is a fully managed cloud database service product. Its price is $499.
3. "ekoDB" is a high-performance database product with AI capabilities. Its price is $99.

=== Context Used (3 snippets) ===
  Snippet 1: {
  collection: 'client_chat_basic_ts',
  record: {
    description: 'Enterprise edition product with advanced features',
    id: 'xUSs7eSBeqWWhEsU10Ic_9ckdDyrSA5rbJrhAZXjowTtG_tXKq2sDm4T91S2Kd21upyxGFlY-FeVnSWsiyeeCw',
    name: 'ekoDB Pro',
    price: 299
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 2: {
  collection: 'client_chat_basic_ts',
  record: {
    id: 'PsDmy7UkJSGOKu4Uk8UENjBGDP49ecC-5lc-yCacxXBK6Wyh5_Q_p4ZgNS-ejBBeUknw2ak_HlEN6K5jXVlCNw',
    description: 'Fully managed cloud database service product',
    price: 499,
    name: 'ekoDB Cloud'
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}
  Snippet 3: {
  collection: 'client_chat_basic_ts',
  record: {
    id: '4KUxTbp22Q9jpHMAkNnzHarKJQRhh3YRAUHOSpuPWmwQZpG2dmL8EG7Drdq4-y85MD4W8XXjxdZKlJAPcJ7I-A',
    description: 'A high-performance database product with AI capabilities',
    name: 'ekoDB',
    price: 99
  },
  score: 0.1111111111111111,
  matched_fields: [ 'description' ]
}

Execution Time: 2367ms

=== Token Usage ===
Prompt tokens: 610
Completion tokens: 72
Total tokens: 682

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
=== ekoDB Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: 5Iga9LK7In9cz8yTSFeG0tRBNbckjkFWiXeq2Xv3uH0pgjnB21Z7tHAhaFfSsUs4P0PCzKq9whyY3jewFh-fjw

=== Sending Messages ===
✓ Message 1 sent
  Response: Answer: One product available is ekoDB. It is a high-performance database product and it costs 99 units of your local currency.

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: w9nJCzvongrZC33ecIgcoT70258Eh1UWGJHZBbdGPwvJGIsJ0CBeOG1-wRpek3t5SLuSjrpqgPSd0PUgj1bfqA
  Parent: 5Iga9LK7In9cz8yTSFeG0tRBNbckjkFWiXeq2Xv3uH0pgjnB21Z7tHAhaFfSsUs4P0PCzKq9whyY3jewFh-fjw

=== Listing Sessions ===
✓ Found 10 sessions
  Session 1: w9nJCzvongrZC33ecIgcoT70258Eh1UWGJHZBbdGPwvJGIsJ0CBeOG1-wRpek3t5SLuSjrpqgPSd0PUgj1bfqA (Untitled)
  Session 2: 5Iga9LK7In9cz8yTSFeG0tRBNbckjkFWiXeq2Xv3uH0pgjnB21Z7tHAhaFfSsUs4P0PCzKq9whyY3jewFh-fjw (Untitled)
  Session 3: k7sW1byJTicQYj0VGGVxPgDYMIhyRWcHbXxcasr2-4ia6FiATCUPrkmgSr3sy9XBd0edmgaMTUOkgHhVPmarFg (Untitled)
  Session 4: NZ3y677q6jvHnfJfB7cTEZQef4i2DBvbBXefm61pJxn-HBf9p7So7p4zJUjILngY864c_I2FcDV6_j51na4qAA (Untitled)
  Session 5: Gu_xIGD5MefdoNTeJDkJ0I4d3RSsP9ER4PGpxn03S00E8P1XDAGDW9F6VOqwiNH5nVOvIOO25KjqjHwrAQDw4w (Untitled)
  Session 6: Qyvsy0PzVbiBeghb9hUiKkm9jkqwn0zqoSrh-sUl0FxvsU7IdcSFGYFMzXIP0bqVZJLQrKxGOYxDiL50740Fuw (Untitled)
  Session 7: Io01vXhsW8BkPYUCUUTLdDWRPT21aGynTNwl8dKLlJ6X24xSeN2mBCOkKES2IJnJ9F6h51vrnyjWWQLYmuyOMQ (Untitled)
  Session 8: j6jNCKZTN-N6CxuK9wi6SxzdbbBMKWEGWrM8AniwBQpg_SWyD86sGW2NZ5rI63Z7R7RlpXjn8nsPwHQcT3VslQ (Untitled)
  Session 9: LJ5cZyqHoXs52cGWUxESsBowN9WD_4okFMLRbpD8-ggFUhYqKZPFaRlUDCmTo8J2LkXgnMPzPinMwwzCxLmR8w (Untitled)
  Session 10: zgZmlmVFLWGNLEdEB26uHagZbr2ZF7802mz_wjqfZqon0azZ3gofLs6k7o8B_odODTl89yzN_di8oFEaFf0y8g (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: w9nJCzvongrZC33ecIgcoT70258Eh1UWGJHZBbdGPwvJGIsJ0CBeOG1-wRpek3t5SLuSjrpqgPSd0PUgj1bfqA

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: wepjtZ8JUYkWErNBcJsUilMOTat_9Mhm6JrvNrekcxl5_rEHSE9KBA-tNZytQB0-_YigHneVXXHAO3rPdRRxag

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information provided, "ekoDB" is available. It is a high-performance database product and the price is $99.

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
✓ Created second session: V29yi8ko2by95UxL-GMdsvrCm7M-ixzOUXY4rfI59cL85dNf777_dbogc3M-f-C_TDnYWHKtfjL75z72NzwOwQ
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
  1. Score: 25.740, Matched: email.value, email, name.value, name
  2. Score: 12.540, Matched: name.value, name

2. Fuzzy search (typo tolerance):
Found 4 results with fuzzy matching
  1. Score: 26.400, Matched: title, bio, bio.value, title.value
  2. Score: 26.400, Matched: bio, title, bio.value, title.value
  3. Score: 26.400, Matched: title, bio, title.value, bio.value
  4. Score: 26.400, Matched: title, title.value, bio.value, bio

3. Search with field weights:
Found 4 results with weighted fields
  1. Score: 39.600, Matched: title, title.value, bio, bio.value
  2. Score: 39.600, Matched: title.value, bio, bio.value, title
  3. Score: 39.600, Matched: title, title.value, bio.value, bio
  4. Score: 39.600, Matched: title, title.value, bio, bio.value

4. Search with minimum score threshold:
Found 2 results with score >= 0.3
  1. Score: 13.200, Matched: bio.value, bio
  2. Score: 13.200, Matched: bio.value, bio

5. Search with stemming and exact match boosting:
Found 1 results (matches: work, working, worked)
  1. Score: 13.200, Matched: bio, bio.value

6. Vector search (semantic search):
Found 3 semantically similar documents
  1. Score: 0.765, Matched: 
  2. Score: 0.752, Matched: 
  3. Score: 0.752, Matched: 

7. Hybrid search (text + vector):
Found 3 results using hybrid search (text + vector)
  1. Score: 2.701, Matched: title, content, content.value, title.value
  2. Score: 1.506, Matched: content.value, content, title.value, title
  3. Score: 0.301, Matched: 

8. Case-sensitive search:
Found 1 results (case-sensitive)
  1. Score: 26.400, Matched: skills.value, skills, bio.value, bio

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
🚀 ekoDB Scripts Example (TypeScript)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: 3-ZQD-GsVD71qHmxaBrHSzkI4Il2hrgmvCbGYV_MH7sSVpoNqY2Hu2FXPTsIuKp7qkZ14Ijaerwd336X1zdBZw
📊 Found 10 active users

📝 Example 2: Parameterized Script

✅ Script saved: CIXxeDBWYb0CH8XUcF3nB5yB2Bqz9eb-3nrzz8YaUa5UBOmA2JHvJMTyPFiiJKNLhHaySOiaNGGBs5nIw7Fq8g
📊 Found 10 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: aV1PY6O9PCxQ1rdPnkzVHEixSL9nkXU_uXJ201YEs5usECCj4IWAYlJ1A-teKQDG78B6OTenUmInakuiba3S6g
📊 Statistics: 2 groups
   {"count":5,"avg_score":50,"status":"inactive"}
   {"status":"active","count":5,"avg_score":60}

📝 Example 4: Script Management

📋 Total scripts: 18
🔍 Retrieved script: Get Active Users
✏️  Script updated
🗑️  Script deleted

ℹ️  Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.

✅ All examples completed!
🚀 ekoDB TypeScript Complete Scripts Example

📋 Demonstrates: FindAll, Group, Count, Multi-stage Pipelines

📋 Setting up complete test data...
✅ Created 5 products

📝 Example 1: FindAll + Group (Simple Aggregation)

✅ Script saved: rdfpirVtpfbAfB4KgPMNyx4tiwjdnvNmzea2B5eZUxjnTf0Z8Pphd9OQuZo8R41yLrXpICnoyNpMLCPwTlcg0Q
📊 Found 2 product groups
   {"avg_price":474,"category":"Furniture","count":2}
   {"category":"Electronics","avg_price":575.6666666666666,"count":3}
⏱️  Execution time: 0ms

📝 Example 2: Simple Product Listing

✅ Script saved
📊 Found 5 products
⏱️  Execution time: 0ms

📝 Example 3: Count by Category

✅ Script saved
📊 Found 2 categories
   {"count":3,"category":"Electronics"}
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
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: I0Pz-f5IduXtsG2am_2uJCXAiG-EUKDTrdsVX-IEGPUE7f7NU013c-ihNcKEgxc5eTUgK892JLf7frJCdWs5Uw
Created Bob: $500 - ID: JuR5-QzLqokyOWCi-cPC3IIFdm9Fy4QcRpwTTVapLgik50oMYPMpZ8DYRuzcGPgVCy0ip9sgC7d-k5x7fs67_A

=== Example 1: Begin Transaction ===
Transaction ID: 115d991f-2c5f-40d3-9767-8556ab840b3a

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: d695ad83-3276-49d4-90c1-015531d704bd
Status before rollback: Active
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
✅ [32mTypeScript client examples complete![0m
✅ [32mAll TypeScript integration tests complete![0m
🧪 [36mRunning JavaScript examples (direct HTTP/WebSocket)...[0m

added 1 package, removed 1 package, and audited 9 packages in 851ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
[34m
╔════════════════════════════════════════╗[0m
[34m║  ekoDB JavaScript Examples Test Suite ║[0m
[34m╚════════════════════════════════════════╝[0m
[34m
=== Checking Server Connection ===[0m
(node:71112) [DEP0040] DeprecationWarning: The `punycode` module is deprecated. Please use a userland alternative instead.
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
  id: 'hbvG-g7y4P7IlJimbtfDN0-d3VSHuU-YRaL8YdE-I1aDlB6SfSfnaeeBjaTnSCW2D2GoxA7eJeMu1ksGSKJEBA'
}

=== Find by ID ===
Found: {
  name: { type: 'String', value: 'Test Record' },
  active: { value: true, type: 'Boolean' },
  id: 'hbvG-g7y4P7IlJimbtfDN0-d3VSHuU-YRaL8YdE-I1aDlB6SfSfnaeeBjaTnSCW2D2GoxA7eJeMu1ksGSKJEBA',
  value: { value: 42, type: 'Integer' }
}

=== Find with Query ===
Found documents: [
  {
    active: { value: true, type: 'Boolean' },
    value: { value: 42, type: 'Integer' },
    id: 'hbvG-g7y4P7IlJimbtfDN0-d3VSHuU-YRaL8YdE-I1aDlB6SfSfnaeeBjaTnSCW2D2GoxA7eJeMu1ksGSKJEBA',
    name: { type: 'String', value: 'Test Record' }
  }
]

=== Update Document ===
Updated: {
  value: { value: 100, type: 'Integer' },
  name: { type: 'String', value: 'Updated Record' },
  active: { value: true, type: 'Boolean' },
  id: 'hbvG-g7y4P7IlJimbtfDN0-d3VSHuU-YRaL8YdE-I1aDlB6SfSfnaeeBjaTnSCW2D2GoxA7eJeMu1ksGSKJEBA'
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
✓ Inserted test record: _0Q3Oudgg4ouoqLCGbtJ7sEpLv6v9qtiZuhhettIAPlJl-X7iHBn3HEVGUzTbb5aZE9PGIWS6mOdCIrTTxJoDg

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
        "id": "d2VJG5iv4Y2GM8EF5v5Bvs8hZucupbWnljtU8V09E6kHsVSL76g8IaS5XdJ9hRAvPBx73kHkr6R-756nGUvxHg",
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
        "id": "_0Q3Oudgg4ouoqLCGbtJ7sEpLv6v9qtiZuhhettIAPlJl-X7iHBn3HEVGUzTbb5aZE9PGIWS6mOdCIrTTxJoDg",
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
        "id": "US1URz1oElP11pCefhxXASu8-cl5XivMknbWJiWq1kXVtN-5hRHQyja-29vCYjgUA0cRpJ_hJZcKdqayTZhPww",
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

✅ Script saved: MD0Tbf7C-tk_6PygovHIgEjpPDNm9226GEdF9iuHOMtkTLKbTYzZWYoGtoKmbK_7gnWo8kZv2-S7kfwgvDLY3A
📊 Found 20 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: Zr1bxN4XenwFs8FiZzZ_ZPiJToV_bHbYq6YczNkrpG3jP_q03OgN76Civmt1i-BXjscex6mS2fzXBxF1fmc1Gw
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 3 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: MzfwedvICb2bVXQ-Yy_iE1s0-uYrcSQYI3xlLcJ8LUQnQK1ST9u_z6QWzKHxDoQxc2pWRKdw3O1QNvgHtHUjEA
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"count":10,"avg_score":50,"status":"inactive","max_score":90}
   {"avg_score":60,"status":"active","count":10,"max_score":100}

📝 Example 4: Function Management

📋 Total scripts: 20
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
cache:product:1: { price: 29.99, name: 'Product 1' }
cache:product:2: { price: 39.989999999999995, name: 'Product 2' }
cache:product:3: { name: 'Product 3', price: 49.989999999999995 }

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
Collection created with first record: Zlfx3I0NGEQl4HDUy3kh_cNlNPFaNgvbuSFOZYaHNqWRZePIR2yTzvkA7ibwvXV_9qPQvJmU1EspQduVSdYFFw

=== List Collections ===
Total collections: 23
Sample collections: [
  'schema_documents_client_ts',
  'ttl_cache',
  'schema_users_client_ts',
  'users',
  'websocket_test'
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
Created Alice: $1000 - ID: wLYPadg5L7aP_bBYvLDXhmG0OA_cNO4gy2_5ocXz-W8-Po7TMstdrix1eRJKznkyRBLeNqTHqHl9iGRpOtnJDQ
Created Bob: $500 - ID: BdZi4K4Wej7FyuqyTq0A1Xxw8xP99iysOawPBuCe1XqSonsRx17rLTeGxE_eyF3iEfr6Y51Ty6JygOM_5QaOKg

=== Example 1: Begin Transaction ===
Transaction ID: f87d4f98-edae-4f03-8cec-b9c7b9c942c6

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
New transaction: 8a242188-f552-4a35-a5c2-8df155d36277
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
   ✅ Script saved: 3TfRNrHGUXDleqbnC5Xyzz3e2HcPPJ9iHTN-d9aLgXbMnz-UyL50zi68vSOYUsiYFN6uqeXRfw9vlrvwY5BlzA

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
   ✅ Script saved: 85h2ropiMCiyf1M9MvWRo_ayMIfKz8TJlKF75Keadqq0ihlruBBUqnqHb84Bc4xsQ8oIO_VEMZbVJFZxLLhjQA

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
   ✅ Script saved: m6O8hOzYnUMvck5uqFhRievI8Jz7YdPnXClYUAlQ9skHnrfENTEHtdUuJmlR6quAPZRbjb_7lI2UvGR5npi9JA

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
   ✅ Script saved: j79faR_3o86f5SgyQNPbNBBKYorrGOkwVyTfl3H8U3ug_9uYUTdeinFZVMSqk_9faXdmuSPApUyqQYgb4F6rmA

2️⃣ Calling Script (Query + Delete + Verify)...
   ✅ Script executed: 3 Functions
   ⏱️  Execution time: 0ms

3️⃣ Verification Results:
   📊 Before delete: Record existed (verified by first Query)
   ✅ After delete: Record successfully deleted (Query returned 0 records)

============================================================
🧹 Cleanup
============================================================
   ✅ Deleted script: 3TfRNrHGUXDleqbnC5Xy...
   ✅ Deleted script: 85h2ropiMCiyf1M9MvWR...
   ✅ Deleted script: m6O8hOzYnUMvck5uqFhR...
   ✅ Deleted script: j79faR_3o86f5SgyQNPb...
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
  Output: Document ID = LJ7tN0F7HYmFTRh2SZ_CT4d3-tvS6X39GdiIc9qWE8l6XbSW3OgjYukpUQX75McTd4aZsjzNe_b6T1x1p575KQ
  ✓ PASS: Document inserted

[Step 2] Verify document exists immediately
  Input: findById(LJ7tN0F7HYmFTRh2SZ_CT4d3-tvS6X39GdiIc9qWE8l6XbSW3OgjYukpUQX75McTd4aZsjzNe_b6T1x1p575KQ)
  Output: Found document with name = TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Verify document has expired
  Input: findById(LJ7tN0F7HYmFTRh2SZ_CT4d3-tvS6X39GdiIc9qWE8l6XbSW3OgjYukpUQX75McTd4aZsjzNe_b6T1x1p575KQ)
  Output: Error (expected) - Request failed with status 404: {"error":"Record not found (expired)"}
  ✓ PASS: Document expired (not found error)

═══════════════════════════════════════════════════════════
TEST 2: Multiple Documents with Different TTLs
═══════════════════════════════════════════════════════════

[Step 1] Insert 3 documents with different TTLs
  Inserted: 3s TTL=TRDlCq6n, 10s TTL=8D40T62j, Permanent=Dz4l_BIM
  ✓ PASS: All documents inserted

[Step 2] Wait 5 seconds (3s doc should expire)
  ✓ PASS: 3s TTL document expired
  ✓ PASS: 10s TTL document still exists
  ✓ PASS: Permanent document still exists

[Step 3] Wait 7 more seconds (10s doc should expire)
  ✓ PASS: 10s TTL document expired
  ✓ PASS: Permanent document still exists

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
  Output: Document ID = DbGP5QmjiPdu69nWGqb5ECPOApW1ZRM0c-OnivDsqwo4jQlE5hURHsk-gSJ_fXtt0ZLC9-ddxRAwj1tK6m1Y3Q
  ✓ PASS: Document inserted

[Step 2] Query to verify document exists
  Input: findById(DbGP5QmjiPdu69nWGqb5ECPOApW1ZRM0c-OnivDsqwo4jQlE5hURHsk-gSJ_fXtt0ZLC9-ddxRAwj1tK6m1Y3Q)
  Output: Found document with name = WS TTL Test
  ✓ PASS: Document exists

[Step 3] Wait for TTL to expire (5s)
  Waiting..... done
  ✓ PASS: Wait complete

[Step 4] Query to verify document has expired
  Input: findById(DbGP5QmjiPdu69nWGqb5ECPOApW1ZRM0c-OnivDsqwo4jQlE5hURHsk-gSJ_fXtt0ZLC9-ddxRAwj1tK6m1Y3Q)
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

added 1 package, removed 1 package, and audited 9 packages in 849ms

1 package is looking for funding
  run `npm fund` for details

found 0 vulnerabilities
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: {
  id: 'Mo-EPH1LnusDM7EbvrdHjwYvsa_RSuHOOol1hTQzRK0dngX6ttcbyR9rLiTCOfq5GBq4EVVkb9BbF9xFoPTC_Q'
}

=== Find by ID ===
Found: {
  active: { value: true, type: 'Boolean' },
  id: 'Mo-EPH1LnusDM7EbvrdHjwYvsa_RSuHOOol1hTQzRK0dngX6ttcbyR9rLiTCOfq5GBq4EVVkb9BbF9xFoPTC_Q',
  name: { type: 'String', value: 'Test Record' },
  value: { type: 'Integer', value: 42 }
}

=== Find with Query ===
Found documents: 1

=== Update Document ===
Updated: {
  value: { type: 'Integer', value: 100 },
  name: { value: 'Updated Record', type: 'String' },
  active: { value: true, type: 'Boolean' },
  id: 'Mo-EPH1LnusDM7EbvrdHjwYvsa_RSuHOOol1hTQzRK0dngX6ttcbyR9rLiTCOfq5GBq4EVVkb9BbF9xFoPTC_Q'
}

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
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

=== KV Set ===
✓ Set key: session:user123

=== KV Get ===
Retrieved value: { username: 'john_doe', userId: 123 }

=== Set Multiple Keys ===
✓ Set 3 keys

=== Get Multiple Keys ===
cache:product:1: { price: 29.99, name: 'Product 1' }
cache:product:2: { price: 39.989999999999995, name: 'Product 2' }
cache:product:3: { name: 'Product 3', price: 49.989999999999995 }

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

=== Create Collection (via insert) ===
Collection created with first record: KbgJw-xf-UfaAjAR31jsr3kugA6OSaxsD41eKirwifUDdDQa7WXlNAq2RskrniHrhOBDJj7VhqdlW5WyFpKZeQ

=== List Collections ===
Total collections: 22
Sample collections: schema_documents_client_ts,client_collection_management_js,ttl_cache,schema_users_client_ts,websocket_test

=== Count Documents ===
Document count: 1

=== Delete Collection ===
Collection deleted successfully

=== Verify Deletion ===
Collection still exists: false

✓ All collection management operations completed successfully
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: qzFC79isXWrs78jhqwAJvjOfQ8nO3yTxUlDrumEwVxS_qv887FWhuEgcCaGauGU6E7fQ2smdieTpz2_Ikibu0w

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: D4dXyHu_GXC-z-fHoLqlJFqOkIDdpEtyxTYRONDecesvmLRyBaSfZgy5D4owK-D78jA1FqDmk8Y2KHMQnULGkA

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

=== Inserting Test Data ===
✓ Inserted test record: fVZjJhrmmXzvShmSuJ9iLh6KLh7SxA9dv2tbRLHkyW0f0Ooa5r5LGvY3292j5hJ8sSnrAWGw6lnjexpRgk6OLw

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 4 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket example completed successfully
✓ Client created

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: l0GW1pEOpm_qw-irQpanbz6iOaAhegntSzcNggo2s-_Wd3g3ynOh0Z1ich0i3g8PosKEHQ3lY1SPxjTJlEYBLQ

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket
  Record 1: 5 fields

=== Cleanup ===
✓ Deleted collection

✓ WebSocket TTL example completed successfully

💡 Note: Documents with TTL will automatically expire after the specified duration
🚀 ekoDB Scripts Example (JavaScript/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: o6bsEqY3XXhNkLKsgOhf0CkyNfO2VOre9DliZpIFQoqTNl2aoDiWBevDv6176wt8MVWam4Oh5j3Dis41J5V6Cw
📊 Found 10 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: ggXwzYp_z1eltafBZDiw4RO__qcgcQdif8_iJRCHpwP8uquP0gIJHp3AnnW4Tvcbvsyh7bdmHBCuD6elgbd-Xw
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: -9ZhE-87PccBT4opjjunVKk3lmgWZnSiai8kJdApQvDIOWA5mfTEI8sNq30K_aKnR9GEkeMmgosL6r04p356rA
📊 Pipeline Results: Filtered (age>20) → Grouped by status → 2 groups
   {"count":5,"avg_score":60,"max_score":100,"status":"active"}
   {"count":5,"avg_score":50,"max_score":90,"status":"inactive"}

📝 Example 4: Function Management

📋 Total scripts: 22
🔍 Retrieved script: Get Active Users
✏️  Script updated
🗑️  Script deleted

ℹ️  Note: GET/UPDATE/DELETE operations require the encrypted ID
ℹ️  Only CALL can use either ID or label

✅ All examples completed!
🚀 ekoDB Scripts Example (JavaScript Client)

✅ Client initialized (token exchange automatic)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: qb84O3Ubfb1Kt_GN9cK0j4gzkYdE4cOc-3kVK_Ch3HBSEi03BfeTJc9xRFaUZA7ONoC1oBPbj5T4Nyn_JH1mjA
📊 Found 20 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved
📊 Found 20 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved
📊 Statistics: 2 groups
   {"avg_score":50,"status":"inactive","count":10}
   {"count":10,"status":"active","avg_score":60}
⏱️  Execution time: 0ms

📝 Example 4: Script Management

📋 Total scripts: 24
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
📊 Total user count: 20
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
   Furniture: 4 items (avg $294.00)
   Electronics: 6 items (avg $325.67)
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
   2. Wireless Mouse - $29 (⭐4.5)
   3. Webcam HD - $119 (⭐4.5)
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All advanced script examples completed!
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
📊 Users (showing first 5 of 10):
   1. User 2 (user2@example.com) - Score: 20
   2. User 9 (user9@example.com) - Score: 90
   3. User 10 (user10@example.com) - Score: 100
   4. User 6 (user6@example.com) - Score: 60
   5. User 1 (user1@example.com) - Score: 10
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
   1. Database Design Principles (Database)
   2. Vector Databases Explained (Database)
   3. Natural Language Processing (AI)
   4. Introduction to Machine Learning (AI)
   5. Getting Started with ekoDB (Database)
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
   1. Database Design Principles
   2. Vector Databases Explained
   3. Natural Language Processing
   4. Introduction to Machine Learning
   5. Getting Started with ekoDB
⏱️  Execution time: 0ms

📝 Example 4: Project Document Fields

✅ Script saved
📊 Projected documents (showing first 3):
   1. Database Design Principles
   2. Vector Databases Explained
   3. Natural Language Processing
⏱️  Execution time: 0ms

📝 Example 5: All Document Fields

✅ Script saved
📊 All documents (5 total, showing first 2):
   1. Database Design Principles (Database)
   2. Vector Databases Explained (Database)
⏱️  Execution time: 0ms

🧹 Cleaning up...
✅ Cleanup complete

✅ All search script examples completed!
🚀 ekoDB AI Scripts Example

⚠️  Note: These examples require AI API credentials (OpenAI, etc.)

📋 Setting up test data...
✅ Created 3 articles

📝 Example 1: Simple Chat Completion

✅ Chat script saved
🤖 AI Response:
   1. Efficient Spatial Analysis: Vector databases allow for more precise calculations and spatial analyses compared to raster databases.
2. High Accuracy: They provide high geometric accuracy due to the use of points, lines, and polygons to represent spatial data.
3. Less Storage Space: They typically require less storage space compared to raster databases.
4. Scalability: Vector data can be easily scaled from small to large without losing any quality or detail.
5. Flexibility: They offer more flexibility in terms of data representation, as they can be easily edited and manipulated.
6. Better Visualization: Vector databases can provide better visual representation for features like points of interest, roads, and boundaries.
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
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 77P0Sed4T8RJbJdED1dBLh8R1Eo3ER6nmYWhvflV2NL0PGh-dpNqAIi3XEbkdIxQpZQYaG680sZrgOwe-82d3w
Created Bob: $500 - ID: zE-qmo5i3__5HiINDww0NhICYeVCcyh50qsGUnr7wAzHy9p5yUu6Uj-nmAdlZKZQkX08bKUOD-fUJ3l0GowSMQ

=== Example 1: Begin Transaction ===
Transaction ID: d9058756-f78d-4727-8477-e5a0e86d943b

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 7672cc52-8b7d-4a78-8474-0971dbb6466e
Status before rollback: Active
✓ Transaction rolled back

=== Cleanup ===
✓ Deleted test accounts

✓ All client transaction examples completed
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
✓ Inserted user: Record(fields={id=StringValue(value=73mkvPW3NeqZYBh2JM6eDMpzB6nblFPFbj1AaEocM9L1xw67l-TGfWoDUO-UiwfyoZPyhbcx91-0LuzrE2E77A)})
  User ID: 73mkvPW3NeqZYBh2JM6eDMpzB6nblFPFbj1AaEocM9L1xw67l-TGfWoDUO-UiwfyoZPyhbcx91-0LuzrE2E77A

=== Read ===
✓ Found user by ID: Record(fields={created_at=ObjectValue(value={value=StringValue(value=Sun Jan 04 17:32:39 EST 2026), type=StringValue(value=String)}), id=StringValue(value=73mkvPW3NeqZYBh2JM6eDMpzB6nblFPFbj1AaEocM9L1xw67l-TGfWoDUO-UiwfyoZPyhbcx91-0LuzrE2E77A), price=ObjectValue(value={type=StringValue(value=Float), value=FloatValue(value=99.99)}), age=ObjectValue(value={value=IntegerValue(value=28), type=StringValue(value=Integer)}), metadata=ObjectValue(value={type=StringValue(value=Object), value=ObjectValue(value={nested=ObjectValue(value={deep=BooleanValue(value=true)}), key=StringValue(value=value)})}), data=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)])}), active=ObjectValue(value={value=BooleanValue(value=true), type=StringValue(value=Boolean)}), tags=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)])}), categories=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)])}), embedding=ObjectValue(value={value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)]), type=StringValue(value=Array)}), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice@example.com)}), user_id=ObjectValue(value={value=StringValue(value=550e8400-e29b-41d4-a716-446655440000), type=StringValue(value=String)}), name=ObjectValue(value={value=StringValue(value=Alice Johnson), type=StringValue(value=String)})})

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Alice Johnson
  email (String): alice@example.com
  age (Integer): 28
  active (Boolean): true
  price (Decimal): 99.99
  created_at (DateTime): Sun Jan 04 17:32:39 EST 2026
  user_id (UUID): 550e8400-e29b-41d4-a716-446655440000
  tags (Array): [tag1, tag2, tag3]
  metadata (Object): {nested={deep=true}, key=value}
  embedding (Vector): [0.1, 0.2, 0.3, 0.4, 0.5]
  categories (Set): [electronics, computers]
  data (Bytes): 11 bytes
Record fields: [created_at, id, price, age, metadata, data, active, tags, categories, embedding, email, user_id, name]

=== Update ===
✓ Updated user: Record(fields={embedding=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)])}), categories=ObjectValue(value={value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)]), type=StringValue(value=Array)}), age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=29)}), city=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=San Francisco)}), metadata=ObjectValue(value={type=StringValue(value=Object), value=ObjectValue(value={nested=ObjectValue(value={deep=BooleanValue(value=true)}), key=StringValue(value=value)})}), active=ObjectValue(value={value=BooleanValue(value=true), type=StringValue(value=Boolean)}), user_id=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=550e8400-e29b-41d4-a716-446655440000)}), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice@example.com)}), created_at=ObjectValue(value={value=StringValue(value=Sun Jan 04 17:32:39 EST 2026), type=StringValue(value=String)}), price=ObjectValue(value={type=StringValue(value=Float), value=FloatValue(value=99.99)}), id=StringValue(value=73mkvPW3NeqZYBh2JM6eDMpzB6nblFPFbj1AaEocM9L1xw67l-TGfWoDUO-UiwfyoZPyhbcx91-0LuzrE2E77A), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), tags=ObjectValue(value={value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)]), type=StringValue(value=Array)}), data=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)])})})

=== Query ===
✓ Found 1 users matching query
  - Record(fields={metadata=ObjectValue(value={value=ObjectValue(value={nested=ObjectValue(value={deep=BooleanValue(value=true)}), key=StringValue(value=value)}), type=StringValue(value=Object)}), name=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=Alice Johnson)}), email=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=alice@example.com)}), created_at=ObjectValue(value={value=StringValue(value=Sun Jan 04 17:32:39 EST 2026), type=StringValue(value=String)}), age=ObjectValue(value={type=StringValue(value=Integer), value=IntegerValue(value=29)}), id=StringValue(value=73mkvPW3NeqZYBh2JM6eDMpzB6nblFPFbj1AaEocM9L1xw67l-TGfWoDUO-UiwfyoZPyhbcx91-0LuzrE2E77A), tags=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=tag1), StringValue(value=tag2), StringValue(value=tag3)])}), price=ObjectValue(value={type=StringValue(value=Float), value=FloatValue(value=99.99)}), user_id=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=550e8400-e29b-41d4-a716-446655440000)}), categories=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[StringValue(value=electronics), StringValue(value=computers)])}), city=ObjectValue(value={type=StringValue(value=String), value=StringValue(value=San Francisco)}), active=ObjectValue(value={value=BooleanValue(value=true), type=StringValue(value=Boolean)}), data=ObjectValue(value={type=StringValue(value=Array), value=ArrayValue(value=[IntegerValue(value=104), IntegerValue(value=101), IntegerValue(value=108), IntegerValue(value=108), IntegerValue(value=111), IntegerValue(value=32), IntegerValue(value=119), IntegerValue(value=111), IntegerValue(value=114), IntegerValue(value=108), IntegerValue(value=100)])}), embedding=ObjectValue(value={value=ArrayValue(value=[FloatValue(value=0.1), FloatValue(value=0.2), FloatValue(value=0.3), FloatValue(value=0.4), FloatValue(value=0.5)]), type=StringValue(value=Array)})})

=== Delete ===
✓ Deleted user with ID: 73mkvPW3NeqZYBh2JM6eDMpzB6nblFPFbj1AaEocM9L1xw67l-TGfWoDUO-UiwfyoZPyhbcx91-0LuzrE2E77A

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
=== ekoDB Kotlin Client - Batch Operations Example ===

=== Batch Insert ===
✓ Inserted 5 records
  IDs: i8-OBwva7cE3-Alwo7FiNQQGVTr5Dg-fg3CYOVMVNLof_ThwFQ3NnqVoFsFCmBehFWg3E0ugQfP2nSKrO6dAgg, tWi_r2ulpiVbAxlVXI5_pJ8bYx91F15SGdzlYuEwKSGjLu1fBjxx6hh_qIXHQF39BJzPOUCIr3OBELBUbspU5Q, cUFx4VJSuTV1oBVibLkrV_x6RaMpcVcEL0r_gyDZqUs2-pz4kGaTUdKftG5EnnqyVbbPv6zvR1TOxB6DD5TmwA...

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
  - Score: ObjectValue(value={value=IntegerValue(value=88), type=StringValue(value=Integer)})

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
=== ekoDB Kotlin Client - KV Operations Example ===

=== KV Set ===
✓ Set key: user:123

=== KV Get ===
✓ Retrieved value: {"name":"Alice","email":"alice@example.com","role":"admin"}

=== KV Set with TTL ===
✓ Set key with 10s TTL: session:abc123

=== Verify TTL Key ===
✓ Session value: {"created_at":1767565972759,"user_id":"123"}
  (Will expire in 10 seconds)

=== Set Multiple Keys ===
✓ Set 3 configuration keys

=== KV Exists ===
✓ Key exists: true

=== KV Find (Pattern Query) ===
✓ Found 3 keys matching 'config:.*'

=== KV Query (Alias for Find) ===
✓ Total keys in store: 5

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
=== ekoDB Kotlin Client - Collection Management Example ===

=== List Collections ===
✓ Found 21 collections
  - schema_documents_client_ts
  - ttl_cache
  - schema_users_client_ts
  - websocket_test
  - chat_messages__ek0_testing

=== Check Collection Existence ===
Collection 'kotlin_collection_example' exists: false

=== Create Collection with Schema ===
✓ Created collection with schema: kotlin_collection_example

=== Get Collection Schema ===
✓ Schema: {"fields":{"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"name":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null}},"version":1,"created_at":"2026-01-04T22:32:57.094455Z","last_modified":"2026-01-04T22:32:57.094455Z","bypass_ripple":false,"primary_key_alias":"id"}

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
=== ekoDB Kotlin Client - Document TTL Example ===

=== Insert with TTL ===
✓ Inserted document with 10s TTL
  Document ID: 2tauopborPjih7-TYQLA00aouoNI6MqQoOrDT_371oBYL-6W_q5jnRXLfjnOd-5AcnEPdIwLwVMrsyIAS9_gRg

=== Verify Document Exists ===
✓ Document found: [session_id, created_at, id, user_id, ttl]

=== Insert with Longer TTL ===
✓ Inserted document with 1h TTL
  Document ID: 85ZK_GXMPzkSjAORgtcy3VGYhkjhV8sERzPFJMOdbYkmN9NU4T8Fc2DdVN5YxXb94RHeSS2p_ucM7HW8SfLa7g

=== TTL Expiration ===
✓ Document will automatically expire after 10 seconds

=== Verify Long TTL Document ===
✓ Long TTL document still exists: [value, id, cache_key, ttl]

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
=== ekoDB Kotlin Client - Simple WebSocket Example ===

=== Create WebSocket Client ===
✓ WebSocket client created

=== Connect to WebSocket ===
✓ Connected to WebSocket

=== Insert Test Record ===
✓ Inserted test record

=== Find All via WebSocket ===
✓ WebSocket findAll result:
  {"payload":{"data":[{"id":"QOcZvX6oL0dXja73dnT2_y3CLr5qlCZdUzj-177gvaMrarjq94mZabs7UjoAKRpZmiDeFxSOP1ISiL_wUWJkIg","name":{"type":"String","value":"Test User"},"status":{"type":"String","value":"active"}}]},"type":"Success"}

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
=== ekoDB Kotlin Client - Schema Management Example ===

=== Creating Collection with Schema ===
✓ Created collection 'kotlin_schema_example' with schema

=== Inserting Valid Documents ===
✓ Inserted user 1: StringValue(value=Q0KLZw7PmarRrsI_qmfiQ3kevw_bot5-evyBo40gVl2QuVWvlPT4xIV2LpO-aoHKnEodbiXIihJIwBqIkTM_xw)
✓ Inserted user 2: StringValue(value=mv4l_xmjLox0vtdrKkY3N8ZUZU4LZZWLsNWe9jpa_9S_uNqxwN-5HQgkAQG0eLGN6bZ0O1HhHbGfae2cAxfC1Q)

=== Getting Schema ===
✓ Schema for kotlin_schema_example:
  Fields: {"email":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"title":{"field_type":"String","default":null,"unique":false,"required":true,"enums":[],"max":null,"min":null,"regex":null},"age":{"field_type":"Integer","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null},"status":{"field_type":"String","default":null,"unique":false,"required":false,"enums":[],"max":null,"min":null,"regex":null}}

=== Listing Collections ===
✓ Total collections: 22
  Sample: [schema_documents_client_ts, ttl_cache, schema_users_client_ts, websocket_test, chat_messages__ek0_testing]

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
=== ekoDB Kotlin Client - Search Example ===

=== Inserting Sample Documents ===
✓ Inserted 5 sample documents

=== Basic Text Search ===
✓ Search results for 'programming':
  {"results":[{"record":{"tags":"programming,rust,tutorial","description":"Learn Rust programming language with hands-on examples and best practices.","id":"hAHJ6Cuek9p2BVH8_1RYFWcMeZiwLRU0Q7nGVtrxGl4rzMTizd8m4YWwlTLMhQgjT3Aqn3aYatJ3kblq6m32sw","title":"Rust Programming","views":832},"score":19.8,"matched_fields":["title","tags","description"]},{"record":{"views":920,"description":"Master Python for data analysis, machine learning, and visualization.","title":"Python for Data Science","id":"NYpmcasQAH2WfIqGakiN1k6wTF1IRE0GRvm4sD0VrQxrA-3InngKiKTuSDIAU11gnSY6CONa9Sh1GX9CefdXEA","tags":"programming,python,data-science"},"score":6.6000000000000005,"matched_fields":["tags"]},{"record":{"id":"6O-p3FWXzRJ9FRXpTuLaPTPQZzRWOqrNHBMPItSiQCbiMFIkP5lrGiGzTeDTdGc-7KGghizkdQYehdQ3BccyNw","description":"Build modern web applications using JavaScript, React, and Node.js.","tags":"programming,javascript,web","views":767,"title":"JavaScript Web Development"},"score":6.6000000000000005,"matched_fields":["tags"]}],"total":3,"execution_time_ms":1}

=== Search for 'machine learning' ===
✓ Found results for 'machine learning'
  {"results":[{"record":{"tags":"ai,machine-learning,python","id":"HWYCGtR_mJXsbuf7INwT3OxrYo6WDHBjysWwsOhH-vwGwYDLOagSZ5jw2FKJiWCTPzO9BmRiuqw9WSu-1g5iUQ","title":"Machine Learning Basics","description":"Introduction to machine learning algorithms and neural networks.","views":395},"score":2.7,"matched_fields":["title","description","tags"]},{"record":{"views":920,"description":"Master Python for data analysis, machine learning, and visualization.","tags":"programming,python,data-science","id":"NYpmcasQAH2WfIqGakiN1k6wTF1IRE0GRvm4sD0VrQxrA-3InngKiKTuSDIAU11gnSY6CONa9Sh1GX9CefdXEA","title":"Python for Data Science"},"score":1.0,"matched_fields":["description"]},{"record":{"views":832,"id":"hAHJ6Cuek9p2BVH8_1RYFWcMeZiwLRU0Q7nGVtrxGl4rzMTizd8m4YWwlTLMhQgjT3Aqn3aYatJ3kblq6m32sw","tags":"programming,rust,tutorial","description":"Learn Rust programming language with hands-on examples and best practices.","title":"Rust Programming"},"score":0.5,"matched_fields":["description"]},{"record":{"title":"Database Design","id":"BoinHVF1f9wSghK4ug_hEXWpHmeFUJC6ZH5dCMUxzNNSf4jkK5dcZBZhgbf8UV7_5Vd-st9G9DSuGDZeVT1ECw","views":156,"tags":"database,design,sql","description":"Learn database design principles, normalization, and query optimization."},"score":0.5,"matched_fields":["description"]}],"total":4,"execution_time_ms":1}

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
=== ekoDB Kotlin Client - WebSocket TTL Example ===

=== Insert Test Data with TTL ===
✓ Inserted document with TTL: StringValue(value=wuxiQOqKaWef-ZykXtfSlb5kFBsa3ZrVRDQWrkwVJLgrdi8CbVBDZ7h2ZNvTZFnZHF8lrgIBGmypwWIJ1b0OOQ)

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved data via WebSocket:
  {"payload":{"data":[{"created_at":{"type":"Integer","value":1767565998535},"id":"wuxiQOqKaWef-ZykXtfSlb5kFBsa3ZrVRDQWrkwVJLgrdi8CbVBDZ7h2ZNvTZFnZHF8lrgIBGmypwWIJ1b0OOQ","name":{"type":"String","value":"WebSocket TTL Test"},"ttl":"2026-01-04T23:33:17.679138Z","value":{"type":"Integer","value":42}}]},"type":"Success"}

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
=== ekoDB Kotlin Client - Basic Chat Example ===

=== Inserting Sample Data ===
✓ Inserted 3 sample documents

=== Creating Chat Session ===
✓ Created session: gHL_1zvJXHgXVlQap1Rs4C1hsUwI9uvIeJxocx9NtfnrdshNWM4_9vaGa4oUxJwEWlaJBzNx0uUGlLxKF1Ukjg

=== Sending Chat Message ===
✓ Chat response:
  Message ID: "9vikJdoiEm1JhG17ZIeTDaKIsAZriK8lekckMsMtCnQ789BfsO8lwISOwYo6UTvNyHM5uyIo2m0u1XybsZ7cYA"
  Responses: ["ekoDB is a high-performance database system with intelligent caching and real-time capabilities. It offers AI integration features and supports a wide range of search methods, including full-text search, vector search, and hybrid search with automatic context retrieval. One of its key features is the AI Chat Integration, which allows you to query your database using natural language and receive AI-powered responses with relevant context."]

=== Cleanup ===
✓ Deleted chat session
✓ Deleted collection: kotlin_chat_basic_example

✓ Basic chat example completed successfully

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
=== ekoDB Kotlin Client - Chat Session Management Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: euf1sG57BSwkmZDTs6M3Hl4uCvYuXJZEw37kEcFs4-b7igcXkRIvsOwzyMwzJnyB4ZpoTpewYbl3VQU8O61C7A

=== Sending Messages ===
✓ Message 1 sent
  Responses: ["Based on the provided context, the available product is \"ekoDB\", a high-performance database product with AI capabilities. It is priced at $99."]

✓ Message 2 sent
  Responses: ["The price of the ekoDB product is $99."]

=== Getting Message History ===
✓ Retrieved message history
  Total messages: [{"chat_id":{"type":"String","value":"euf1sG57BSwkmZDTs6M3Hl4uCvYuXJZEw37kEcFs4-b7igcXkRIvsOwzyMwzJnyB4ZpoTpewYbl3VQU8O61C7A"},"content":{"type":"String","value":"What products are available?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"atx8sAJPtjmUxZL7Htr731LKKfJgzcyEkraa_EqM_tdZ8ISPapIJkAHCwF0NHakWrM-uv4kKgRQ_Kb7VKx5t7g","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2026-01-04T22:33:36.463987Z"},"id":"Uh_qFprQen0q9cxBnq7W6d-U4aO10empfH2CtpJ6obHDsuE6UA3KMidfCfhVK7H8pt7u-nC1AQicjm6QEBDvcg","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":30,"prompt_tokens":239,"total_tokens":269}},"updated_at":{"type":"DateTime","value":"2026-01-04T22:33:36.463987Z"}},{"chat_id":{"type":"String","value":"euf1sG57BSwkmZDTs6M3Hl4uCvYuXJZEw37kEcFs4-b7igcXkRIvsOwzyMwzJnyB4ZpoTpewYbl3VQU8O61C7A"},"content":{"type":"String","value":"Based on the provided context, the available product is \"ekoDB\", a high-performance database product with AI capabilities. It is priced at $99."},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["description"],"record":{"description":"A high-performance database product with AI capabilities","id":"atx8sAJPtjmUxZL7Htr731LKKfJgzcyEkraa_EqM_tdZ8ISPapIJkAHCwF0NHakWrM-uv4kKgRQ_Kb7VKx5t7g","price":99,"product":"ekoDB"},"score":0.25}]},"created_at":{"type":"DateTime","value":"2026-01-04T22:33:36.464953Z"},"id":"TVvDhcRDnelFAJ3WOQb51o7xjwOD0hxo95guStilxoHixinW10dnVxaUpBO_reRRe_cq1wKH930_7o_lwIrMFg","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":30,"prompt_tokens":239,"total_tokens":269}},"updated_at":{"type":"DateTime","value":"2026-01-04T22:33:36.464953Z"}},{"chat_id":{"type":"String","value":"euf1sG57BSwkmZDTs6M3Hl4uCvYuXJZEw37kEcFs4-b7igcXkRIvsOwzyMwzJnyB4ZpoTpewYbl3VQU8O61C7A"},"content":{"type":"String","value":"What is the price?"},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["price"],"record":{"description":"A high-performance database product with AI capabilities","id":"atx8sAJPtjmUxZL7Htr731LKKfJgzcyEkraa_EqM_tdZ8ISPapIJkAHCwF0NHakWrM-uv4kKgRQ_Kb7VKx5t7g","price":99,"product":"ekoDB"},"score":0.8}]},"created_at":{"type":"DateTime","value":"2026-01-04T22:33:37.694417Z"},"id":"pyUDna-wauy3YD9QXVj7NmBc-ArMqBhLcjcekqJiVFOJGH7QAWCBpZlC0RL9u_WJCRSeFqrhgUAP_znNHzqSjQ","role":{"type":"String","value":"user"},"token_usage":{"type":"Object","value":{"completion_tokens":12,"prompt_tokens":239,"total_tokens":251}},"updated_at":{"type":"DateTime","value":"2026-01-04T22:33:37.694417Z"}},{"chat_id":{"type":"String","value":"euf1sG57BSwkmZDTs6M3Hl4uCvYuXJZEw37kEcFs4-b7igcXkRIvsOwzyMwzJnyB4ZpoTpewYbl3VQU8O61C7A"},"content":{"type":"String","value":"The price of the ekoDB product is $99."},"context_snippets":{"type":"Array","value":[{"collection":"kotlin_chat_sessions_example","matched_fields":["price"],"record":{"description":"A high-performance database product with AI capabilities","id":"atx8sAJPtjmUxZL7Htr731LKKfJgzcyEkraa_EqM_tdZ8ISPapIJkAHCwF0NHakWrM-uv4kKgRQ_Kb7VKx5t7g","price":99,"product":"ekoDB"},"score":0.8}]},"created_at":{"type":"DateTime","value":"2026-01-04T22:33:37.697257Z"},"id":"9c_R6pK0Lai--ksR6PGaQ4-essE8Hi32d74d0wBFbmKAM67LNdpWSqAEGbhUZHJ4kTcIT_4pCe8YiNqxsgtX1g","role":{"type":"String","value":"assistant"},"token_usage":{"type":"Object","value":{"completion_tokens":12,"prompt_tokens":239,"total_tokens":251}},"updated_at":{"type":"DateTime","value":"2026-01-04T22:33:37.697257Z"}}]

=== Updating Session ===
✓ Updated session system prompt

=== Listing Sessions ===
✓ Total sessions: 13

=== Branching Session ===
✓ Created branched session: NWVU8IiDXezcDpMgr2IbQYjRCJdz3Ol9ZuBcdyfmlDmPAHFUVWD8eMmjJ5EsB_vIxKf4amZvqzyCLJPdU8M5dg

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
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
=== ekoDB Kotlin Client - Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: lCKecKySEtNiD9b94KloExenkKFzKa5uOul8-1efXxPgM5J1rNbIo1t--XJZewqn31JcaRufNm-ePRpMPlSVBg

=== Sending Initial Message ===
✓ Message sent
  Responses: ["The available product is \"ekoDB\", a high-performance database product. It's priced at $99."]

✓ Second message sent

=== Regenerating AI Response ===
✓ AI response regenerated
  New responses: ["The product available is \"ekoDB\", which is a high-performance database product. It is priced at $99."]

=== Updating Message ===
✓ Updated message content

=== Toggling Forgotten Status ===
✓ Marked message as forgotten (excluded from context)

=== Creating Second Session for Merge ===
✓ Created second session: bDySUfAgGyud9yUAZ-_wXv4OqhErzouH15NZLo3hnvPPFw1fALhGm8K98g1igV6W5b9iC497sSmdGJeoxJfC_Q

=== Merging Sessions ===
✓ Merged sessions
  Merged session ID: null

=== Deleting Message ===
✓ Deleted message

=== Cleanup ===
✓ Deleted chat sessions
✓ Deleted collection: kotlin_chat_advanced_example

✓ Advanced chat features example completed successfully

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
🚀 ekoDB Scripts Example (Kotlin Client)

SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
✅ Client initialized

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Script

✅ Script saved: c1lxmuyXpnFqCszrX0_SDj4PpiCkdPdKiCaXDAkJbEmnzlvXRcehkuUzSnsMCRKmFjGICLoga1qPANRQH7gyHQ
📊 Found 10 records
⏱️  Execution time: 0ms

📝 Example 2: Parameterized Script

✅ Script saved
📊 Found 10 users (limited)
⏱️  Execution time: 0ms

📝 Example 3: Aggregation Script

✅ Script saved
📊 Statistics: 2 groups
   {"avg_score":50.0,"status":"inactive","count":5}
   {"count":5,"avg_score":60.0,"status":"active"}
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
📊 Total user count: 10
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
✅ [32mKotlin client examples complete![0m
🟣 [33mKotlin Transactions...[0m
SLF4J(W): No SLF4J providers were found.
SLF4J(W): Defaulting to no-operation (NOP) logger implementation
SLF4J(W): See https://www.slf4j.org/codes.html#noProviders for further details.
✓ Client created

=== Setup: Creating Test Accounts ===
Created Alice: $1000 - ID: 7S13gMGm2LP84PqfeNn-5QROwdFE2MsWXKDY-zKMUVjXgdg4atRwmksjVLZqlfDyKtdBnwd0ABarX8vKIMRL0Q
Created Bob: $500 - ID: ZTfzzcXP9X6U6yE8Tn9V4c9xrN0JllYGRHNEWEuhOzUo0kKLAdYcqoVqX7s_ZqCBiPMYnUEMtd4lplMahlXpaA

=== Example 1: Begin Transaction ===
Transaction ID: 36561d3c-d57b-4d6f-9e6c-e92f002377c8

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: Active
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 4e1f02c4-4033-435e-90a8-4cb3151b0f03
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
    ✓ Generated embedding: 1536 dimensions in 504.861708ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 402.024334ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 278.977375ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 933.013375ms
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 281.500875ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 257.393625ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 439.904375ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 304.521083ms
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 316.464583ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 488.683417ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 573.020959ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 222.918208ms
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
    ✓ Generated embedding: 1536 dimensions in 510.965041ms
    • Function auto-cleaned up by client

→ Executing hybrid_search()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 90.666333ms

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

Writing memory-safe high-performance database code involves several aspects. Here are some general tips:

1. Use Prepared Statements: In SQL, prepared statements not only help prevent SQL injection attacks but also provide improved performance. When a database prepares a statement, it creates an execution plan for the query which can be reused. This reduces the overhead of parsing and compiling the SQL statement every time it's executed.

2. Efficient Data Fetching: Fetch only the data that you need. Using SELECT * can bring unnecessary data into memory. Be explicit in your queries about the data you want to retrieve.

3. Indexing: Proper indexing can dramatically increase the performance of your database queries by reducing the amount of data that needs to be examined.

4. Normalize Your Database: As per context 2 and 3, database normalization is a process to organize data to reduce redundancy and improve data integrity. This can often lead to performance improvements.

5. Use Appropriate Data Types: Using the appropriate data type not only reduces memory usage but can also improve performance. For example, if an integer is sufficient, there's no need to use a larger data type like BIGINT.

6. Use Transactions: Transactions in SQL ensure ACID properties (Atomicity, Consistency, Isolation, Durability). However, they should be used judiciously as open transactions lock resources and can decrease overall database performance.

7. Connection Pooling: Creating a new database connection every time one is needed can be a time-consuming process. Connection pooling allows you to reuse existing active database connections, reducing the overhead of establishing a new connection every time one is required.

8. Keep Your Database Updated: Database management systems (DBMS) are regularly updated with optimizations and enhancements that can improve performance.

9. Avoid N+1 Query Problem: In ORM frameworks, avoid N+1 query problems by using eager fetching or batch fetching when you know you need to retrieve a collection of objects and their associated objects.

10. Profiling and Optimizing Queries: Regularly monitor, profile, and optimize your database queries. Slow queries can be identified using the EXPLAIN statement in SQL which shows how a query will be executed.

Ultimately, the specific techniques and methods you use will depend on the specific database system (SQL, NoSQL, etc.) and the specific requirements of your application.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 436.049042ms
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 2465 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 309.690709ms
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
  ✓ Text search completed in 61.3995ms

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
    ✓ Generated embedding: 1536 dimensions in 0.277s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.261s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.306s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.477s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.211s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.288s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.240s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.452s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.285s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.287s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.241s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.319s
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
    ✓ Generated embedding: 1536 dimensions in 0.253s
    • Function auto-cleaned up by client

→ Executing hybrid_search()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.090s

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

Writing memory-safe, high-performance database code involves several general practices:

1. **Understanding your Database System**: Whether you're using SQL or NoSQL, you need to understand how your particular database system works. This includes understanding its indexing mechanisms, query execution plans, etc. For instance, if you're using SQL, understanding normalization (as described in Context 2 and 3) can help you design efficient schemas that reduce data redundancy and improve data integrity.

2. **Efficient Query Writing**: Writing efficient queries is essential for high-performance database code. This means avoiding costly operations like full table scans wherever possible, using indexes effectively, and minimizing the use of functions or operations that can't take advantage of indexes.

3. **Optimizing Database Structure**: Depending on whether you're using a SQL or NoSQL database, you may need to design your database structure differently. As mentioned in Context 1, NoSQL is better for flexible schemas, horizontal scaling, high write throughput, or when working with unstructured data, while SQL is better for complex queries, ACID transactions, and structured data with well-defined relationships.

4. **Managing Connections and Resources**: Make sure to properly manage your database connections and other resources. This includes closing connections when they're no longer needed, managing your transaction isolation levels appropriately to prevent unnecessary locking, and handling any potential exceptions or errors that can occur during database operations.

5. **Memory Management**: For memory safety, avoid practices that can lead to memory leaks. For instance, in languages like C++, remember to deallocate any memory that you've allocated once you're done using it. In managed languages like Java or C#, be aware of how the garbage collector works and how to avoid creating unnecessary objects.

6. **Caching**: Use caching wisely to reduce database load and increase performance. This could involve caching frequently accessed data in memory or using a caching layer like Redis.

7. **Performance Testing**: Regularly measure the performance of your database operations. This can help you identify any potential bottlenecks or performance issues that can be addressed.

Remember that the specific techniques you'll need to use can vary depending on the specific database system you're using, the programming language you're writing your code in, and the specific requirements of your application.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.510s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 2535 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.212s
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
    ✓ Generated embedding: 1536 dimensions in 0.292s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.300s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.220s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.311s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.269s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.229s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.240s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.278s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.292s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.329s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.281s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.194s
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
    ✓ Generated embedding: 1536 dimensions in 0.398s
    • Function auto-cleaned up by client

→ Executing hybridSearch()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.081s
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

Writing memory-safe, high-performance database code involves a number of best practices across different areas.

1. Understanding the Database System: Whether it's SQL or NoSQL, understanding the internals of the database system allows you to write efficient queries and use the database features effectively. This includes understanding the database structure, indexes, execution plans, etc.

2. Use Prepared Statements: Prepared statements can not only help prevent SQL injection attacks but also optimize your SQL queries. The database can cache the execution plan for a prepared statement, which avoids the overhead of parsing and compiling the SQL statement each time it is executed.

3. Normalize Your Database: As mentioned in Context 1, database normalization can help to reduce redundancy and improve data integrity. This can lead to more efficient storage and query execution.

4. Optimizing Queries: There are many strategies to optimize database queries as mentioned in Context 5. Some of these include using appropriate indexes, limiting the amount of data retrieved from the database, using joins effectively, avoiding unnecessary calculations, etc.

5. Connection Pooling: Connection pooling can greatly enhance the performance of your database code. Establishing a database connection can be a time-consuming process. By reusing existing connections, you can eliminate the overhead of establishing a new connection for each query.

6. Use Transactions Appropriately: Transactions can ensure data integrity but they can also lock database resources. Use transactions judiciously to avoid unnecessary locks. And remember, SQL is better for ACID transactions as mentioned in Context 3.

7. Handle Exceptions: Always code for exception handling to manage any database errors and to ensure your code doesn't break unexpectedly. This can also help in avoiding memory leaks.

8. Memory Management: Depending on the programming language you are using, ensure you are managing memory correctly. This includes freeing up resources after they are no longer needed and understanding how your language's garbage collector works. 

Remember, high-performance code is often the result of a deep understanding of the system you're working with, careful planning, and constant testing and optimization.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.269s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 2301 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.309s
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
  ✓ Text search completed in 0.054s
✓ Found 3 messages mentioning ownership:
  1. From conv_rust_programming: Rust's key features include: memory safety without garbage collection, zero-cost abstractions, ownership system, powerful type system, and excellent concurrency support.

  2. From conv_performance: Rust's ownership system provides zero-cost memory management. Use Box for heap allocation, Rc/Arc for shared ownership, and avoid cloning large data structures. The compiler optimizes away unnecessary allocations.

  3. From conv_new_question: Writing memory-safe, high-performance database code involves a number of best practices across different areas.

1. Understanding the Database System: Whether it's SQL or NoSQL, understanding the internals of the database system allows you to write efficient queries and use the database features effectively. This includes understanding the database structure, indexes, execution plans, etc.

2. Use Prepared Statements: Prepared statements can not only help prevent SQL injection attacks but also optimize your SQL queries. The database can cache the execution plan for a prepared statement, which avoids the overhead of parsing and compiling the SQL statement each time it is executed.

3. Normalize Your Database: As mentioned in Context 1, database normalization can help to reduce redundancy and improve data integrity. This can lead to more efficient storage and query execution.

4. Optimizing Queries: There are many strategies to optimize database queries as mentioned in Context 5. Some of these include using appropriate indexes, limiting the amount of data retrieved from the database, using joins effectively, avoiding unnecessary calculations, etc.

5. Connection Pooling: Connection pooling can greatly enhance the performance of your database code. Establishing a database connection can be a time-consuming process. By reusing existing connections, you can eliminate the overhead of establishing a new connection for each query.

6. Use Transactions Appropriately: Transactions can ensure data integrity but they can also lock database resources. Use transactions judiciously to avoid unnecessary locks. And remember, SQL is better for ACID transactions as mentioned in Context 3.

7. Handle Exceptions: Always code for exception handling to manage any database errors and to ensure your code doesn't break unexpectedly. This can also help in avoiding memory leaks.

8. Memory Management: Depending on the programming language you are using, ensure you are managing memory correctly. This includes freeing up resources after they are no longer needed and understanding how your language's garbage collector works. 

Remember, high-performance code is often the result of a deep understanding of the system you're working with, careful planning, and constant testing and optimization.

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
    ✓ Generated embedding: 1536 dimensions in 0.212s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.239s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.214s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.262s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.261s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.239s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.239s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.245s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.290s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.321s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.268s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.764s
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
    ✓ Generated embedding: 1536 dimensions in 0.240s
    • Function auto-cleaned up by client

→ Executing HybridSearch()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.080s

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

Writing memory-safe, high-performance database code involves several considerations:

1. **Use Efficient Queries**: The efficiency of your database code significantly depends on the design of your queries. Optimize your SQL queries by reducing the number of joins, using indexing, and only fetching the data you need. Remember to avoid "select *" statements which can cause unnecessary load on the database.

2. **Normalization**: As mentioned in Context 1, database normalization can reduce redundancy and improve data integrity. It also makes the database more efficient by reducing the amount of space that the data takes up.

3. **Choosing the Right Database**: As noted in Context 3 and 4, the type of database you use can also affect performance. SQL databases are good for structured data and complex queries, while NoSQL databases are better for unstructured data and have high write throughput.

4. **Memory Management**: In terms of memory safety, ensure to free up memory that's no longer in use. If you're using a language like C++, make sure to deallocate memory after it's used to prevent memory leaks. In languages with automatic garbage collection like Java, make sure objects that are no longer in use are ready for garbage collection by setting their references to null.

5. **Connection Pooling**: Repeatedly opening and closing database connections can be costly. Use connection pooling to reuse existing database connections.

6. **Concurrent Processing**: Use concurrent processing where applicable to allow multiple parts of the program to run simultaneously, increasing performance.

7. **Caching**: Implement caching to store the results of common queries in memory, reducing the need for repeated database calls.

8. **Batch Processing**: Instead of performing database operations one at a time, group them together and perform them in batches.

Remember, the key to high-performance database code is keeping data access operations to a minimum, optimizing queries, and efficiently managing memory.

=== Step 5: Storing New Conversation ===
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 58 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.349s
    • Function auto-cleaned up by client
  → Calling ekoDB Embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 2024 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.275s
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
  ✓ Text search completed in 0.054s

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
    ✓ Generated embedding: 1536 dimensions in 0.25s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 169 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.225s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.191s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 230 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.232s
    • Function auto-cleaned up by client
✓ Stored Rust programming conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 31 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.271s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 217 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.261s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 33 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.305s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 232 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.349s
    • Function auto-cleaned up by client
✓ Stored database design conversation (4 messages)
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 36 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.33s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 178 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.243s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 37 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.292s
    • Function auto-cleaned up by client
  → Calling ekoDB embed() helper...
    • Using model: text-embedding-3-small
    • Text length: 213 characters
    • Behind the scenes: Creating temp Function with Embed operation
    ✓ Generated embedding: 1536 dimensions in 0.258s
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
    ✓ Generated embedding: 1536 dimensions in 0.208s
    • Function auto-cleaned up by client

→ Executing hybridSearch()...
  • Collection: rag_messages
  • Query text: "How do I write memory-safe high-performance database code?"
  • Vector dimensions: 1536
  • Limit: 5 results
  • Search type: Semantic (vector) + Keyword (text)
  • Server combines both scores for relevance ranking
  ✓ Search completed in 0.082s

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
    ✓ Generated embedding: 1536 dimensions in 0.252s
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
  ✓ Text search completed in 0.043s

✓ Found 3 messages mentioning ownership:
  1. From conv_performance: Rust's ownership system provides zero-cost memory management. Use Box for heap allocation, Rc/Arc for shared ownership, and avoid cloning large data structures. The compiler optimizes away unnecessary allocations.

  2. From conv_rust_programming: Rust's key features include: memory safety without garbage collection, zero-cost abstractions, ownership system, powerful type system, and excellent concurrency support.

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


up to date, audited 58 packages in 981ms

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
✓ Created SWR script: fetch_github_user (f_xyKP3Kd5ZGPYh4ObixhsgIskiVUcxsW6PNA_msPsmjm6fRbEb2cr8RqtHAg8gX58EDeQRAJlGT6kCqyVzscg)

Step 2: First call - Cache miss, fetches from GitHub API
Result: {
  "records": [
    {
      "id": "8lXS7WrBKR8FCO-i819nhSX9IKdlUkqDa3lwhEqlE6QMiLAsi9mGG-yt-VS6PzzMuU5iCLYdSQ0h8J4ctwXWWQ",
      "cached_at": "2026-01-04T22:35:07.498Z",
      "data": {
        "value": {
          "hireable": null,
          "location": "Portland, OR",
          "starred_url": "https://api.github.com/users/torvalds/starred{/owner}{/repo}",
          "node_id": "MDQ6VXNlcjEwMjQwMjU=",
          "public_gists": 1,
          "subscriptions_url": "https://api.github.com/users/torvalds/subscriptions",
          "bio": null,
          "html_url": "https://github.com/torvalds",
          "organizations_url": "https://api.github.com/users/torvalds/orgs",
          "following_url": "https://api.github.com/users/torvalds/following{/other_user}",
          "name": "Linus Torvalds",
          "events_url": "https://api.github.com/users/torvalds/events{/privacy}",
          "id": 1024025,
          "repos_url": "https://api.github.com/users/torvalds/repos",
          "site_admin": false,
          "created_at": "2011-09-03T15:26:22Z",
          "user_view_type": "public",
          "login": "torvalds",
          "updated_at": "2025-11-24T04:16:14Z",
          "company": "Linux Foundation",
          "url": "https://api.github.com/users/torvalds",
          "avatar_url": "https://avatars.githubusercontent.com/u/1024025?v=4",
          "received_events_url": "https://api.github.com/users/torvalds/received_events",
          "blog": "",
          "following": 0,
          "gravatar_id": "",
          "public_repos": 9,
          "twitter_username": null,
          "type": "User",
          "gists_url": "https://api.github.com/users/torvalds/gists{/gist_id}",
          "email": null,
          "followers_url": "https://api.github.com/users/torvalds/followers",
          "followers": 269948
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
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 76ms (served from cache)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_with_reviews (42Zip67KRKAlLwcQzVCeEpNhArGv_J11jruW8mtfXKJV4S59uByPfqcwDnjgtf1hqWio0iXewSF5RVfjenR99Q)

Step 4: Call enrichment function - Fetches from 2 APIs + stores merged result
Enriched data: {
  "records": [
    {
      "enriched_at": "2026-01-04T22:35:07.769Z",
      "id": "AAzYszANtkNBty9CyE4XvU2YJ6N-JtIvZ8rh_HvOjSm5aTntWLaabgjZGouPPlK0u0e4GvaduB7oEtg3BznBaw",
      "enriched_data": {
        "value": {
          "weight": 4,
          "dimensions": {
            "height": 13.08,
            "depth": 22.99,
            "width": 15.14
          },
          "images": [
            "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
          ],
          "price": 9.99,
          "availabilityStatus": "In Stock",
          "shippingInformation": "Ships in 3-5 business days",
          "tags": [
            "beauty",
            "mascara"
          ],
          "meta": {
            "createdAt": "2025-04-30T09:41:02.053Z",
            "barcode": "5784719087687",
            "qrCode": "https://cdn.dummyjson.com/public/qr-code.png",
            "updatedAt": "2025-04-30T09:41:02.053Z"
          },
          "warrantyInformation": "1 week warranty",
          "minimumOrderQuantity": 48,
          "reviews": [
            {
              "reviewerName": "Eleanor Collins",
              "comment": "Would not recommend!",
              "rating": 3,
              "date": "2025-04-30T09:41:02.053Z",
              "reviewerEmail": "eleanor.collins@x.dummyjson.com"
            },
            {
              "reviewerEmail": "lucas.gordon@x.dummyjson.com",
              "reviewerName": "Lucas Gordon",
              "comment": "Very satisfied!",
              "rating": 4,
              "date": "2025-04-30T09:41:02.053Z"
            },
            {
              "date": "2025-04-30T09:41:02.053Z",
              "reviewerEmail": "eleanor.collins@x.dummyjson.com",
              "rating": 5,
              "reviewerName": "Eleanor Collins",
              "comment": "Highly impressed!"
            }
          ],
          "rating": 2.56,
          "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
          "category": "beauty",
          "brand": "Essence",
          "returnPolicy": "No return policy",
          "title": "Essence Mascara Lash Princess",
          "sku": "BEA-ESS-ESS-001",
          "id": 1,
          "discountPercentage": 10.48,
          "stock": 99,
          "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp"
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
✓ Edge cache script created: sh85mTUUHjjwTZG58v12M7yQ-qTSVvBYd0SMxchcEXbmGNiAkF7H4GKFkkOf9Z4_tUB3XsQ936armLVn-XAoBg

Call 1: Cache miss (fetches from API)
Response time: 555ms
Result: {
  "records": [
    {
      "id": "uBkKheA_QRzKegL5yRQ3pNzEZ_onfxWkHikogrBCes2rxMySmY9ZYpsPz72yTAWrmRYd-RyvNats9KO3wQ3tuw",
      "cached_at": "2026-01-04T22:35:08.217Z",
      "data": {
        "type": "Object",
        "value": {
          "current_units": {
            "temperature_2m": "°C",
            "time": "iso8601",
            "interval": "seconds"
          },
          "longitude": -73.99309,
          "utc_offset_seconds": 0,
          "latitude": 40.710335,
          "timezone": "GMT",
          "timezone_abbreviation": "GMT",
          "elevation": 32,
          "generationtime_ms": 0.01883506774902344,
          "current": {
            "interval": 900,
            "time": "2026-01-04T22:30",
            "temperature_2m": -2.5
          }
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
Response time: 544ms (1x faster!)
Result: {
  "records": [
    {
      "data": {
        "value": {
          "generationtime_ms": 0.02288818359375,
          "current_units": {
            "temperature_2m": "°C",
            "interval": "seconds",
            "time": "iso8601"
          },
          "elevation": 32,
          "latitude": 40.710335,
          "longitude": -73.99309,
          "current": {
            "time": "2026-01-04T22:30",
            "interval": 900,
            "temperature_2m": -2.5
          },
          "timezone": "GMT",
          "timezone_abbreviation": "GMT",
          "utc_offset_seconds": 0
        },
        "type": "Object"
      },
      "id": "LNLvqYgG-24vsPbx47uB6oz999emOLnMWA-wpXJxJXAzXedF71cohHq9enBu8o1wkwSak01FDYjMv-5_7dfCsg",
      "cached_at": "2026-01-04T22:35:08.217Z"
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
✓ Created SWR script: fetch_github_user (G8423RDQHqjAD_5CgyRTwKtpm7lKh29PWe0YZoO-IfUp9aBjlK-jmhQyXSMRuoUjiSEyYGY-4iZRpNcY8JMGFg)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 79ms
Result: [
  {
    "cached_at": "1767566110",
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
        "followers": 269948,
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
    "id": "OlxYDGTQyCGw57wgmI8ELoJYWhcv_4EVDblEYQ8coteejVjB193pUlgnW_1vgQWYaiBMT0t5hCEBDFUH9BFZsg"
  }
]
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 78ms (1.0x faster!)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (We8k8pJTrT0BCVSAWSmNblCbkC73wKnJVR9swuAyTiy2N6k7Nkkeb2vayJNL7B-8Lvn4hwS0sR_auGnvtFltDg)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: [
  {
    "enriched_at": "1767566111",
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
    "id": "NL-bqThypPQ6Uo2tfcZE6jioAIpc-Wk14nhc_zuw7CHts23gUFtauP2cDLp9thpTSwBUr146iEKZlYYQz2EQuA"
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
✓ Created SWR script: fetch_github_user (HASDwpBBl8IF5BUiZDz7hsaqRb8nxhNyPD1dJ9UQWh0TSKniatvEXbTM_uIa0XXv2WvmJMOW5d5mO8uCtuoVNg)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 78.959792ms
Result: [
  {
    "cached_at": "1767566110",
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
        "followers": 269948,
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
    "id": "7U4PF_2w2YCbd9XukjL3qvVCym_UUcDAS3Yms0NU5m1kYggefoD-AaMje_MnYdcU_F-TYvHpIkqPvS5KXSHJVA"
  }
]
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 80.3905ms (1.0x faster!)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (J-y0eD_XQScQ6eBOhy_GqXtJrOK_eVCA9_bjPDAYhi87hw8CAeuNPPcm-h12_PIrblXCQ8LAR4C1IwfPry6ZCw)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: [
  {
    "enriched_at": "2026-01-04T17:35:11-05:00",
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
    "id": "KGBXwIKF7wUV1hjS1opLtDarBSooE7xFMe6M2JdFAv3dwOFv0JBeGUra59J_4Ibb3oF58tnjP-dWWtOgo2d5FQ"
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
✓ Created SWR script: fetch_github_user (eTMuQmslghHPIl3KmYMNRSLgp9nhBdHhEhx3UjJNCL6frf6v15KF_eFxoGGZq-67-TetsNKtVf_f95wYxR7-tg)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 79ms
Result: {
  "data": {
    "type": "Object",
    "value": {
      "subscriptions_url": "https://api.github.com/users/torvalds/subscriptions",
      "following": 0,
      "followers_url": "https://api.github.com/users/torvalds/followers",
      "company": "Linux Foundation",
      "updated_at": "2025-11-24T04:16:14Z",
      "id": 1024025,
      "gravatar_id": "",
      "site_admin": false,
      "twitter_username": null,
      "email": null,
      "name": "Linus Torvalds",
      "user_view_type": "public",
      "type": "User",
      "gists_url": "https://api.github.com/users/torvalds/gists{/gist_id}",
      "repos_url": "https://api.github.com/users/torvalds/repos",
      "blog": "",
      "location": "Portland, OR",
      "hireable": null,
      "login": "torvalds",
      "followers": 269948,
      "bio": null,
      "organizations_url": "https://api.github.com/users/torvalds/orgs",
      "avatar_url": "https://avatars.githubusercontent.com/u/1024025?v=4",
      "received_events_url": "https://api.github.com/users/torvalds/received_events",
      "url": "https://api.github.com/users/torvalds",
      "public_repos": 9,
      "html_url": "https://github.com/torvalds",
      "public_gists": 1,
      "node_id": "MDQ6VXNlcjEwMjQwMjU=",
      "starred_url": "https://api.github.com/users/torvalds/starred{/owner}{/repo}",
      "following_url": "https://api.github.com/users/torvalds/following{/other_user}",
      "created_at": "2011-09-03T15:26:22Z",
      "events_url": "https://api.github.com/users/torvalds/events{/privacy}"
    }
  },
  "id": "6nzp6Vb-qHlr-OTvuPdW6My-SI9dO8psN8Yv2DS7wH2KfYmXZRAr4K94V1UT92Mkvk5sTc1vQiU7zNGfnm8cJQ"
}
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 85ms (0.9x faster!)
✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (7bRA9PIUMW6T2NJ4ayrGjQX62c8QVsHhLSd1gE2OpjyBxw49Hq0eOwA_wfbiiELaI6dxXaNX6OgMp5KJo5P6EQ)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: {
  "id": "1Q6-lbe--7pvdSiqbOb1QHW1k0xMSZ0U__F6F6Bk5TeGLtwpvMhfBQWiAn7ZkK4S3wbkXGfP1tmy8W6ktw4H6w",
  "enriched_at": "2026-01-04T17:35:11-05:00",
  "enriched_data": {
    "type": "Object",
    "value": {
      "meta": {
        "qrCode": "https://cdn.dummyjson.com/public/qr-code.png",
        "updatedAt": "2025-04-30T09:41:02.053Z",
        "barcode": "5784719087687",
        "createdAt": "2025-04-30T09:41:02.053Z"
      },
      "price": 9.99,
      "dimensions": {
        "height": 13.08,
        "width": 15.14,
        "depth": 22.99
      },
      "availabilityStatus": "In Stock",
      "stock": 99,
      "reviews": [
        {
          "rating": 3,
          "reviewerEmail": "eleanor.collins@x.dummyjson.com",
          "comment": "Would not recommend!",
          "date": "2025-04-30T09:41:02.053Z",
          "reviewerName": "Eleanor Collins"
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
          "rating": 5,
          "date": "2025-04-30T09:41:02.053Z",
          "comment": "Highly impressed!",
          "reviewerEmail": "eleanor.collins@x.dummyjson.com"
        }
      ],
      "sku": "BEA-ESS-ESS-001",
      "warrantyInformation": "1 week warranty",
      "weight": 4,
      "tags": [
        "beauty",
        "mascara"
      ],
      "discountPercentage": 10.48,
      "category": "beauty",
      "description": "The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.",
      "rating": 2.56,
      "id": 1,
      "brand": "Essence",
      "thumbnail": "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp",
      "title": "Essence Mascara Lash Princess",
      "returnPolicy": "No return policy",
      "shippingInformation": "Ships in 3-5 business days",
      "minimumOrderQuantity": 48,
      "images": [
        "https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"
      ]
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
✓ Created SWR script: fetch_github_user (BT4rL1ERHPcKx3zBpGBDa63yvPOR2XZegjPwA36_9xWk05gnIZoD3gSoj3Tp7NiNmoMg5GH7wVeSu-mf3ggnYQ)

Step 2: First call - Cache miss, fetches from GitHub API
Response time: 93ms
Result: [{"data":{"value":{"location":"Portland, OR","node_id":"MDQ6VXNlcjEwMjQwMjU=","site_admin":false,"gravatar_id":"","company":"Linux Foundation","created_at":"2011-09-03T15:26:22Z","following_url":"https://api.github.com/users/torvalds/following{/other_user}","hireable":null,"name":"Linus Torvalds","user_view_type":"public","organizations_url":"https://api.github.com/users/torvalds/orgs","blog":"","received_events_url":"https://api.github.com/users/torvalds/received_events","html_url":"https://github.com/torvalds","followers_url":"https://api.github.com/users/torvalds/followers","starred_url":"https://api.github.com/users/torvalds/starred{/owner}{/repo}","email":null,"updated_at":"2025-11-24T04:16:14Z","login":"torvalds","type":"User","twitter_username":null,"followers":269948,"following":0,"bio":null,"gists_url":"https://api.github.com/users/torvalds/gists{/gist_id}","id":1024025,"public_repos":9,"events_url":"https://api.github.com/users/torvalds/events{/privacy}","url":"https://api.github.com/users/torvalds","avatar_url":"https://avatars.githubusercontent.com/u/1024025?v=4","public_gists":1,"subscriptions_url":"https://api.github.com/users/torvalds/subscriptions","repos_url":"https://api.github.com/users/torvalds/repos"},"type":"Object"},"cached_at":"1767566116410","id":"VnP-QxIv2rEOkvt7r-ijDBpqcPrXgTCEaRTnDNloKF3WJOsEzHtCdtGsYj8LcAAIm0x3FHLixixuMwFqhe9U1g"}]
✓ Data fetched from external API and cached

Step 3: Second call - Cache hit, instant response from ekoDB
Response time: 79ms
Cache hit was 1.2x faster!

✓ Lightning fast cache hit

=== Advanced: SWR with Data Enrichment ===

Creating product enrichment function...
✓ Created enrichment script: fetch_product_enriched (urx8HLhJJcpKoCeJ41rdNXQB5IlD_Nsi8YuWJ9_n2BMh-ln6ntksHDoUxL5eXcULhj4Rm1ytQYcbudFyRzIqKQ)

Step 4: Call enrichment function - Fetches from API + stores enriched result
Enriched data: [{"enriched_data":{"type":"Object","value":{"meta":{"updatedAt":"2025-04-30T09:41:02.053Z","barcode":"5784719087687","createdAt":"2025-04-30T09:41:02.053Z","qrCode":"https://cdn.dummyjson.com/public/qr-code.png"},"returnPolicy":"No return policy","tags":["beauty","mascara"],"thumbnail":"https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/thumbnail.webp","minimumOrderQuantity":48,"sku":"BEA-ESS-ESS-001","reviews":[{"date":"2025-04-30T09:41:02.053Z","reviewerName":"Eleanor Collins","comment":"Would not recommend!","reviewerEmail":"eleanor.collins@x.dummyjson.com","rating":3},{"date":"2025-04-30T09:41:02.053Z","reviewerEmail":"lucas.gordon@x.dummyjson.com","rating":4,"comment":"Very satisfied!","reviewerName":"Lucas Gordon"},{"reviewerEmail":"eleanor.collins@x.dummyjson.com","date":"2025-04-30T09:41:02.053Z","rating":5,"comment":"Highly impressed!","reviewerName":"Eleanor Collins"}],"shippingInformation":"Ships in 3-5 business days","warrantyInformation":"1 week warranty","id":1,"description":"The Essence Mascara Lash Princess is a popular mascara known for its volumizing and lengthening effects. Achieve dramatic lashes with this long-lasting and cruelty-free formula.","price":9.99,"category":"beauty","dimensions":{"depth":22.99,"width":15.14,"height":13.08},"images":["https://cdn.dummyjson.com/product-images/beauty/essence-mascara-lash-princess/1.webp"],"stock":99,"weight":4,"brand":"Essence","discountPercentage":10.48,"title":"Essence Mascara Lash Princess","availabilityStatus":"In Stock","rating":2.56}},"enriched_at":"2026-01-04T17:35:11-05:00","id":"bDLYD5hqnUjLSFO-Af3os5lXSLL7o3g3SX8d6v199tEUEo6CISXeMKaM15rwgYHosDNORcTx2V6SLExaw2tRRQ"}]
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.06s
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
   Name: Some(Object({"value": String("User 1"), "type": String("String")}))
   Department: Some(Object({"type": String("String"), "value": String("engineering")}))

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using CallFunction to replace inline logic in SWR pattern...

✅ Saved reusable function: fetch_and_store_github
✅ Saved SWR function using composition: swr_github_user

First call (cache miss - will fetch from GitHub):
   ⏱️  Duration: 87.349833ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 83.680375ms
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


up to date, audited 58 packages in 999ms

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
   Name: {"type":"String","value":"User 1"}
   Department: {"value":"engineering","type":"String"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using CallFunction to replace inline logic in SWR pattern...

✅ Saved reusable function: fetch_and_store_github
✅ Saved SWR function using composition: swr_github_user

First call (cache miss - will fetch from GitHub):
   ⏱️  Duration: 86ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 81ms
   📊 Records: 1
   🚀 Cache speedup: 1.1x faster!

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
   ⏱️  Duration: 89.7ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 79.9ms
   📊 Records: 1
   🚀 Cache speedup: 1.1x faster!

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
   ⏱️  Duration: 80.495375ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 82.437ms
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
   Name: {"value":"User 1","type":"String"}
   Department: {"type":"String","value":"engineering"}

🎯 Key Benefit: fetch_user can be reused by ANY function!
   No code duplication, single source of truth

📝 Example 2: SWR Pattern with Function Composition

Using CallFunction to replace inline logic in SWR pattern...

✅ Saved reusable function: fetch_and_store_github
✅ Saved SWR function using composition: swr_github_user

First call (cache miss - will fetch from GitHub):
   ⏱️  Duration: 82ms
   📊 Records: 1

Second call (cache hit - from cache):
   ⏱️  Duration: 82ms
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

