make test-examples-rust
🧪 [36mRunning Rust examples (direct HTTP/WebSocket)...[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.22s
     Running `target/debug/examples/simple_crud`
✓ Authentication successful

=== Insert Document ===
Inserted: Object {"id": String("f-alhCAIv353429p30J9MoUDYEISlDEbHaIyUhncjlzfYvHFhklArzVsnCqQlfBMo50bJOtr6LbqtTw7HLCGtA")}

=== Find by ID ===
Found: Object {"active": Bool(true), "id": String("f-alhCAIv353429p30J9MoUDYEISlDEbHaIyUhncjlzfYvHFhklArzVsnCqQlfBMo50bJOtr6LbqtTw7HLCGtA"), "name": String("Test Record"), "value": Number(42)}

=== Find with Query ===
Found documents: Array [Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("f-alhCAIv353429p30J9MoUDYEISlDEbHaIyUhncjlzfYvHFhklArzVsnCqQlfBMo50bJOtr6LbqtTw7HLCGtA"), "name": Object {"type": String("String"), "value": String("Test Record")}, "value": Object {"type": String("Integer"), "value": Number(42)}}]

=== Update Document ===
Updated: Object {"active": Object {"type": String("Boolean"), "value": Bool(true)}, "id": String("f-alhCAIv353429p30J9MoUDYEISlDEbHaIyUhncjlzfYvHFhklArzVsnCqQlfBMo50bJOtr6LbqtTw7HLCGtA"), "name": Object {"type": String("String"), "value": String("Updated Record")}, "value": Object {"type": String("Integer"), "value": Number(100)}}

=== Delete Document ===
Deleted document

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/simple_websocket`
✓ Authentication successful

=== Inserting Test Data ===
✓ Inserted test record: "UJ8sqjygQJh0xmSoS1YV0zvVaZmZyqqdFu1OVFWvpebBTmIknmxii-J_gDQXS07nfbIq8eBNXTvj3CxPf1LaXw"

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
        "id": "UJ8sqjygQJh0xmSoS1YV0zvVaZmZyqqdFu1OVFWvpebBTmIknmxii-J_gDQXS07nfbIq8eBNXTvj3CxPf1LaXw",
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
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
Collection created with first record: "-EhnAVdNXittMoCXtpi1Cc05iXa4wjVzUoPNXNg_DWuPrepnYrgteMfDzvk8r494HTRyoSN8cPv24O-c9dxvNA"

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
✓ Inserted document: "nhmvpa1YCqrVopz4gx7AfO_L0ddy9mpxarW1K33bP40PiIymQOCZuVGRWyUOfN_nmzwnn-uJpwXUST9bb5lxzA"

=== Insert Document with TTL (5 minutes - integer) ===
✓ Inserted document: "aEaZalylGtvmnH_-zIG-Ry2nWDZr0fOi4SxaCzXXOalQNRsbx_yrbaiGynwkQ7givwwfT3J_A0SoPcg-sDTo4g"

=== Insert Document with TTL (30 minutes - duration string) ===
✓ Inserted document with duration string TTL: "5GsWHIhr9N67RorHG8Yv6Iv6jn3ymYlmPM2cIhNjI3wrSM-fGyhNalnUdm5hLymYbzo2wV4ZQ-jep2PhqMJclA"

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
✓ Inserted document with TTL: String("tc0DhfenZAyrvD91MgChyiGqLCplySfxTBkHj4R28iXhcjW-Ar0d9GYc1QHAXidyzCf1qh2X9yoZm6PRwAXNvg")

=== Query via WebSocket ===
✓ WebSocket connected
✓ Retrieved 1 record(s) via WebSocket

Record 1:
  id: "tc0DhfenZAyrvD91MgChyiGqLCplySfxTBkHj4R28iXhcjW-Ar0d9GYc1QHAXidyzCf1qh2X9yoZm6PRwAXNvg"
  name: {"type":"String","value":"WebSocket TTL Test"}
  ttl: "2026-01-28T15:40:52.457272Z"
  value: {"type":"Integer","value":42}

✓ WebSocket TTL example completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/http_functions`
🚀 ekoDB Scripts Example (Rust/HTTP)

📋 Setting up test data...
✅ Test data ready

📝 Example 1: Simple Query Function with Filter

✅ Script saved: xmeEF-hc6C-jd1v72zTSGcPWBwMnCToOAObYBUTEaV5MRLt2EqjWF07RpY0WhDF_p3Qh6jeZuLdNAu2U-icMkg
📊 Found 5 active users

📝 Example 2: Parameterized Pagination with Limit/Skip

✅ Script saved: "a3cF4b_0M4WfQ8zyfcNijJRb0qvKfoucTyUq-ZYAVFDn6_By6j6GDiroZul7Iifqu-q2AvUBkyvVmRdfcN8r9A"
📊 Page 1: Found 3 users (limit=3, skip=0)
📊 Page 2: Found 2 users (limit=3, skip=3)

📝 Example 3: Complex Filter with Multiple Conditions

✅ Script saved: 9I4hcJbRiMNvfHP6ugFGP04m6VDQtDBPDhOQQasnStSOoWgQtepmtdYXXmsOWeMe2VFEy79cpq-0ABPpZshlZw
📊 Found 3 users (status=active, score>50, sorted by score)

📝 Example 4: Multi-Stage Pipeline (Query → Group → Calculate)

✅ Script saved: RfbyQQ7p_XoPuq4-0wu1KqBmBOTnmqJI9MBQBTn-KD0t-19kmfB308nSFMOFbTNi-45bvP9FWdq2WzYySJyiLg
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
Created Alice: $1000 - ID: uGyKlWMrcC0b-ZLhZPqCqUxlL6wBhushMzzpSNHDX7WxQRjCFfxnwGT9gOa_1LavH5ZZwg3QmyMFzq-flt5Xig
Created Bob: $500 - ID: smimj68V3z8USrkr2xrM4ng9NuCrq-I4FwGds015c_Vnst13nM96P3dYDC8IvSxcoX5iwYQjpPPezcuP6kv3-g

=== Example 1: Begin Transaction ===
Transaction ID: 3fb5f0a2-e81b-4d9a-bd91-55228b7d9ccc

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
New transaction: b055a434-9ca7-4e58-baf7-01ab12a37ce0
Updated Bob: $700 → $600 (in transaction)
✓ Transaction rolled back
Bob after rollback: 700

=== Cleanup ===
✓ Deleted test accounts

✓ All transaction examples completed
✅ [32mRust direct examples complete![0m
🛠️  [36mBuilding client library...[0m
cargo build -p ekodb_client
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.18s
✅ [32mClient build complete![0m
🧪 [36mRunning Rust client library examples...[0m
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_simple_crud`
✓ Client created (token exchange happens automatically)

=== Insert Document ===
Inserted: Record({"id": String("6nOtw3L2Tn36ZWlEuqZTrQGILl4TIw9FFSaXQACVAffqXR3S4g3eG0hHNA5VMP1TzvZcVuo9BQHuY-XzHKZVwA")})

=== Find by ID ===
Found: Record({"embedding": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "price": Float(99.99), "value": Integer(42), "categories": Array([String("electronics"), String("computers")]), "created_at": String("2026-01-28T14:40:58.391883+00:00"), "user_id": String("550e8400-e29b-41d4-a716-446655440000"), "data": String("aGVsbG8gd29ybGQ="), "metadata": Object({"key": String("value"), "nested": Object({"deep": Boolean(true)})}), "active": Boolean(true), "id": String("6nOtw3L2Tn36ZWlEuqZTrQGILl4TIw9FFSaXQACVAffqXR3S4g3eG0hHNA5VMP1TzvZcVuo9BQHuY-XzHKZVwA"), "name": String("Test Record"), "tags": Array([String("tag1"), String("tag2"), String("tag3")])})

=== Extract Field Values (All Types) ===
Extracted values:
  name (String): Some("Test Record")
  value (Integer): Some(42)
  active (Boolean): Some(true)
  price (Decimal): Some(99.99)
  created_at (DateTime): Some("2026-01-28T14:40:58.391883+00:00")
  user_id (UUID): Some("550e8400-e29b-41d4-a716-446655440000")
  tags (Array): 3 items
  metadata (Object): 2 keys
  embedding (Vector): 0 dims
  categories (Set): 0 items
  data (Bytes): 11 bytes

=== Find with Query ===
Found documents: [Record({"metadata": Object({"value": Object({"nested": Object({"deep": Boolean(true)}), "key": String("value")}), "type": String("Object")}), "name": Object({"value": String("Test Record"), "type": String("String")}), "embedding": Object({"value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "type": String("Array")}), "categories": Object({"type": String("Array"), "value": Array([String("electronics"), String("computers")])}), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "data": Object({"value": String("aGVsbG8gd29ybGQ="), "type": String("String")}), "id": String("6nOtw3L2Tn36ZWlEuqZTrQGILl4TIw9FFSaXQACVAffqXR3S4g3eG0hHNA5VMP1TzvZcVuo9BQHuY-XzHKZVwA"), "created_at": Object({"type": String("DateTime"), "value": String("2026-01-28T14:40:58.391883Z")}), "tags": Object({"type": String("Array"), "value": Array([String("tag1"), String("tag2"), String("tag3")])}), "user_id": Object({"value": String("550e8400-e29b-41d4-a716-446655440000"), "type": String("String")}), "value": Object({"type": String("Integer"), "value": Integer(42)}), "price": Object({"type": String("Float"), "value": Float(99.99)})})]

=== Update Document ===
Updated: Record({"metadata": Object({"type": String("Object"), "value": Object({"key": String("value"), "nested": Object({"deep": Boolean(true)})})}), "value": Object({"type": String("Integer"), "value": Integer(100)}), "categories": Object({"value": Array([String("electronics"), String("computers")]), "type": String("Array")}), "id": String("6nOtw3L2Tn36ZWlEuqZTrQGILl4TIw9FFSaXQACVAffqXR3S4g3eG0hHNA5VMP1TzvZcVuo9BQHuY-XzHKZVwA"), "data": Object({"value": String("aGVsbG8gd29ybGQ="), "type": String("String")}), "active": Object({"type": String("Boolean"), "value": Boolean(true)}), "price": Object({"type": String("Float"), "value": Float(99.99)}), "user_id": Object({"value": String("550e8400-e29b-41d4-a716-446655440000"), "type": String("String")}), "created_at": Object({"value": String("2026-01-28T14:40:58.391883Z"), "type": String("DateTime")}), "embedding": Object({"value": Array([Float(0.1), Float(0.2), Float(0.3), Float(0.4), Float(0.5)]), "type": String("Array")}), "tags": Object({"type": String("Array"), "value": Array([String("tag1"), String("tag2"), String("tag3")])}), "name": Object({"type": String("String"), "value": String("Updated Record")})})

=== Delete Document ===
Deleted document

=== Cleanup ===
✓ Deleted collection

✓ All CRUD operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_simple_websocket`
✓ Client created

=== Inserting Test Data ===
✓ Inserted test record: nbMKHwOvpcnE_umyxRelwTSR0a79bxooKGt3HySvfWWw-Qi6WCWpU42-4SzqyD2ZBJ4O5QQvKg_lDiD81KpXfw

=== Connecting to WebSocket ===
✓ WebSocket connected

=== Querying Data via WebSocket ===
✓ Retrieved 1 record(s) via WebSocket

Record 1:
  value: 42
  id: "nbMKHwOvpcnE_umyxRelwTSR0a79bxooKGt3HySvfWWw-Qi6WCWpU42-4SzqyD2ZBJ4O5QQvKg_lDiD81KpXfw"
  name: "WebSocket Test Record"
  active: true

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
Collection created with first record: "HNdSz7ag0JyeNrgCEnn3V5aY7WsGJ_-XHVn7aEzVLQ0bZceutPGH0DSZ3xuF3_kBOjgwkncnAUgTJFMENt061Q"

=== List Collections ===
Total collections: 9
Sample collections: ["test_collection", "client_collection_management_rust", "test_accounts", "batch_users", "websocket_test"]

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
Created Alice: $1000 - ID: r5xmZ8zoqnN1BpYAU02vmiZziZWt83OmyIF2qjHdiZJnfD0QmZOUBvGVNfa4abxXvKbq-76BHqWhYBXnevwCgw
Created Bob: $500 - ID: eqMCJcxhf-JT9lNhbEMdHS3How_eiGI-Pq5jH1E392y-d7HriQK7E_u2onsFBCdwGX_ZBxD6m9oJpAhJAMCJtg

=== Example 1: Begin Transaction ===
Transaction ID: 48384fac-e172-406b-846a-25d0750897d6

=== Example 2: Operations within Transaction ===
Updated Alice: $1000 → $800 (simulated)
Updated Bob: $500 → $700 (simulated)

=== Example 3: Transaction Status ===
Status: "Active"
Operations: 0

=== Example 4: Commit Transaction ===
✓ Transaction committed

=== Example 5: Rollback Demo ===
New transaction: 8e85518c-4cdb-4f0c-aa6f-212c65efe3e5
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
  - Some(Object({"value": String("Bob"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("Alice")}))
  - Some(Object({"value": String("David"), "type": String("String")}))

=== Range Query (age >= 28 AND age < 35) ===
✓ Found 3 users in age range
  - Some(Object({"type": String("String"), "value": String("Bob")}))
  - Some(Object({"value": String("David"), "type": String("String")}))
  - Some(Object({"value": String("Eve"), "type": String("String")}))

=== IN Operator ===
✓ Found 4 users with status active or pending
  - Some(Object({"value": String("Bob"), "type": String("String")}))
  - Some(Object({"value": String("Alice"), "type": String("String")}))
  - Some(Object({"type": String("String"), "value": String("David")}))
  - Some(Object({"type": String("String"), "value": String("Eve")}))

=== NOT IN Operator ===
✓ Found 4 users not inactive

=== String Pattern Matching ===
✓ Found 5 users with @example.com email

=== Regex Query ===
✓ Found 0 users with names starting with A-C

=== Complex Query (active AND age >= 28 AND score > 1500) ===
✓ Found 2 users matching all conditions
  - Some(Object({"value": String("Bob"), "type": String("String")}))
  - Some(Object({"value": String("David"), "type": String("String")}))

=== OR Query ===
✓ Found 2 users with age < 28 OR age > 32
  - Some(Object({"type": String("String"), "value": String("Charlie")}))
  - Some(Object({"type": String("String"), "value": String("Alice")}))

=== Sorted Query (by score descending) ===
✓ Top 3 users by score:
  1. Some(Object({"type": String("String"), "value": String("Bob")}))
  2. Some(Object({"value": String("David"), "type": String("String")}))
  3. Some(Object({"type": String("String"), "value": String("Alice")}))

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
     Matched: ["description", "description.value", "title.value", "title"]
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
Execution time: 2ms
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_schema`
=== ekoDB Schema Management Example ===

=== Creating Collection with Schema ===
✓ Created collection 'schema_client_rust' with schema

=== Inserting Valid Documents ===
✓ Inserted user 1: Some(String("iWfF2RkgY6IBiZPsFI2zEpTgZtQNetb2vsPAL6LdeYDp50CS9GSX8HE4DL2sqmbzoqC4nA8i9QPLhh5QXWS6Sw"))
✓ Inserted user 2: Some(String("lmnW0OcNt5r5oL7L4q2IpCTHWY1GAjVPW75Y3QIe9BzXZQ2TGxpPotOaXARy7Fr4gd0VWMhonqkH4z7p-vkIAg"))

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
  Sample: ["test_collection", "test_accounts", "batch_users", "websocket_test", "schema_client_rust"]

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
     Running `target/debug/examples/client_document_ttl`
✓ Client created

=== Insert Document with TTL (1 hour) ===
✓ Inserted document: "pg4S8TkaJxI1fdWJ1YbObQMKRy8p0zOEiL_aJPtMfmms6fxQiSUmnr1gX8OhCDPQzjKbTGvXh3wpJnxvE0ibmQ"

=== Insert Document with TTL (5 minutes) ===
✓ Inserted document: Some(String("Q8lY-StaCioJlq33ePrwpzqkHLEFIc0emPWzDavrWpnNgy8_tCP3XplH5m9ru1UXFWrvmuWK_BJkcofcgSGzzA"))

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
✓ Inserted document with TTL: Some(String("LR9rJJGYB_8EocfBMhDHJjpZhTgwNQbdDp56nlfL5OFFWW0lTfJVuqHNqz_QJr5LEMIGP91DXD6ols8IQMHEeA"))

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
✓ Edge cache script created: 16iXDDBkfnKtJ1QSuQEi2RnT9LgzqIZjFP33GAkH7d-eBGmisA9k_HJQ5-e6GM-re11OpPFtEi1Bm_ZKAPNiMQ

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

✅ Script saved: R7MBv0MfNgabIF3CW-q6hUFavJdGOpbizr0areVPVZZEGmGAfg1NwVvQ0Ow11RoNyhEw18C71FFCItOZVIX9lg
📊 Found 20 active users

📝 Example 2: Parameterized Script

✅ Script saved: 6Uv4wdGGbdIGGXkZIBQhJ3ZeSrQam3ksrukSoaAcvlWg3EyZO3UlyKC6NnSuY7UoFMUX41v9UyjPi-sbF1YYLg
📊 Found 20 users (limited)

📝 Example 3: Aggregation Script

✅ Script saved: TD4cmL9fqTFj-xDcOLkKxu1MlFqBJoBF5KkAcmL1cga1ReU8w8MQf5mUx0G0OW6lJZs-y3XlkCcavHYeRCSG5Q
📊 Statistics: 2 groups

📝 Example 4: Script Management

📋 Total scripts: 6
🔍 Retrieved script: Get Active Users
✏️  Script updated
🗑️  Script deleted

ℹ️  Note: GET/UPDATE/DELETE use IDs. Only CALL supports labels.


✅ All examples completed!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
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
   ⏱️  Duration: 123.746041ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "name": "Leanne Graham",
    "address": {
      "zipcode": "92998-3874",
      "geo": {
        "lng": "81.1496",
        "lat": "-37.3159"
      },
      "city": "Gwenborough",
   ...

Second call (cache hit - from cache):
   ⏱️  Duration: 2.263959ms
   📊 Records: 1
   📦 Data: {
  "value": {
    "id": 1,
    "email": "Sincere@april.biz",
    "username": "Bret",
    "company": {
      "name": "Romaguera-Crona",
      "catchPhrase": "Multi-layered client-server neural-net",
 ...
   🚀 Cache speedup: 61.5x faster!

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

✅ Script saved: MRTfuQwWY3DC4ZzosNWq8-uoczQy62tLwzO3MvN32XQwtM15NEi1pxinCawB56sub6YqBMnZzPBZaiHVuGKoSg
📊 Found 2 product groups
   Record({"avg_price": Float(575.6666666666666), "category": String("Electronics"), "count": Integer(3)})
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
   Record({"count": Integer(2), "category": String("Furniture")})
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

✅ Inserted order: Some(String("a0D27iGjPgbROAeph-tFlUPRFr2nF9JeBj2vZpyObwnFLthwVJUT6RL9FtnwH4sqKGQYN8inbam8LyxYyTfPaw"))
✅ Inserted 2 products with wrapped types

📝 Example 2: Script with Wrapped Type Parameters

✅ Script saved: z2TSBASHSSer_iTO0SbVDu_-3ThVZho7hKkXedLmtViUg77vepw68bpZ3hkdBpJzl_BQ1eprNMYONv1xVe-8uA
📊 Script executed
⏱️  Execution time: 0ms

📝 Example 3: Basic KV Store Operations

✅ Set session data
📊 Retrieved session: Some(Object {"role": String("admin"), "userId": String("user_abc")})
🔍 Key exists: true
✅ Set cached data
🗑️  Deleted session

📝 Example 4: KV Operations in Scripts

✅ Script saved: Ffgq4sBm12f0KKs8T65ezoVrvLsK0y2yjhwlZqzVSWCp_undTcJHbARbxhL3J4ghRNYXO_h4x6gBk1S_ooQtHw
📊 Cached and retrieved product data
⏱️  Execution time: 0ms

📝 Example 5: Combined Wrapped Types + KV Script

✅ Script saved: EcP6TwklD81FBa7Tl3lJxxhM_Lb2dNxmh2Y7t6h0BfPDoOZkkyVOaB_DKwIS97YPH4azmhap0Wzx48JmO0oQhQ
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
✓ Created SWR script: fetch_api_user_rs (G9bJBOdZgzrE1adoBPX_rLVdK-O6R7tfd_N2C-j3oBtQYtjHVMXqEuHvemQyt4gRb7yoawY645BGyVfMxETeug)

Step 2: First call - Cache miss, fetches from API
Result: FunctionStats { input_count: 0, output_count: 1, execution_time_ms: 0, stages_executed: 2, stage_stats: [] }
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/client_swr_native`
=== ekoDB Native SWR Function ===

This example shows the new simplified SWR function that replaces
the manual FindById → If → HttpRequest → Insert pattern.

Example 1: Basic GitHub User Cache with Native SWR
─────────────────────────────────────────────────────

✓ Created native SWR script: github_user_native (uQNzdNOFI9POuR010mLu9yapeJY2Jv4tFlRZEIMP8DPk5LXdWW6OXJiUceVtxEyaRxsPZZShweIbECK9b-tgEQ)

First call (cache miss - will fetch from GitHub API):
  Response time: 115ms
  Records returned: 1
  ✓ Data fetched from API and cached with 15m TTL

Second call (cache hit - instant from KV store):
  Response time: 5ms
  Speedup: 23.0x faster
  ✓ Lightning fast cache hit


Example 2: SWR with Audit Trail Collection
─────────────────────────────────────────────────────

✓ Created SWR script with audit trail: product_swr_audit (4wjIRNnLnmLEYbhEoDviGo2lnzVIcLHFwYNlAkIlVOpgWTExEHH6ZgOKcyLpoDpiMDAJwMvF_AfUgFxz6Ovivg)

Fetching product (will create audit trail entry):
  ✓ Product fetched and cached
  ✓ Audit record created in 'swr_audit_trail' collection
  Records: 1


Example 3: SWR in Multi-Function Pipeline
─────────────────────────────────────────────────────

Fetch external data → Process → Store in collection
✓ Created enrichment pipeline: user_enrichment_pipeline (eKyRm6HqgbqpSaUGqIx5GnDvn1uO4fGJ072XMwX-eDiQ-WZa2cqnm_vj_GDA4K3n-50-uhOi-ymT1dfGOEjDEw)

Running pipeline:
  ✓ Data fetched from API (cached 30m)
  ✓ Enriched data stored in 'enriched_users' (TTL 24h)
  Pipeline returned 1 records


Example 4: Dynamic TTL Configuration
─────────────────────────────────────────────────────

✓ Created dynamic TTL script: flexible_cache (Vba7Wo9pjjPuEghj9ISWMQRjR_DcSZTJBY7OqKUMQPIhDP_qzDYErr77Za_ELtfke5sMGC23MMREEMWzx8vuNw)
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
   Record({"count": Integer(5), "avg_price": Float(367.0), "category": String("Electronics")})
   Record({"avg_price": Float(365.6666666666667), "category": String("Furniture"), "count": Integer(3)})
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
   Vector databases provide benefits such as efficient data management, precise representation of data points, lines, and polygons, ability to perform complex spatial queries, and scalability for large datasets. They are also excellent for representing data that has discrete boundaries like country borders or land parcels.
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
   Record({"status": String("active"), "count": Integer(7)})
   Record({"count": Integer(3), "status": String("inactive")})
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
   1. Introduction to Machine Learning (AI)
   2. Database Design Principles (Database)
   3. Natural Language Processing (AI)
   4. Vector Databases Explained (Database)
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
✓ Created session: 3SpDq19pY2Ui9N9-tOtcvr4WAkrxDsWBFE3DCYNzdL6SrvNVZClJpKRk7pXCOHa5JXLXKWb2vS9fd-CNweAnjw

=== Sending Chat Message ===
Message ID: -MEialzS3xcWv5bfJRE1zZCM9EpOZRWgfnoSwkb4glD61ZtavWyPshlrIBn1ePU7XE_BccGlDntYIIO_ICKzuw

=== AI Response ===
Response 1: ekoDB is a high-performance database that features intelligent caching, real-time capabilities, and AI integration. Some of its key features include an AI Chat Integration that allows you to query your database using natural language and receive AI-powered responses with relevant context. Additionally, ekoDB supports various search features such as full-text search, vector search, and hybrid search with automatic context retrieval.

=== Context Used (3 snippets) ===

Snippet 1:
  Collection: client_chat_basic_rust
  Score: 0.6333
  Matched Fields: ["content", "title", "category"]
  Record: Object {"category": String("features"), "content": String("The chat feature allows you to query your database using natural language and get AI-powered responses with relevant context."), "id": String("D6DFJ24egfRw3B_oCkmZstEmLZpor0IGKlJ37N7Hp6p9WJDPK8aXVIRQV1qY9GDViEtMiQzWGjk-pE2qj7owAw"), "title": String("AI Chat Integration")}

Snippet 2:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["title", "content"]
  Record: Object {"category": String("documentation"), "content": String("ekoDB is a high-performance database with intelligent caching, real-time capabilities, and AI integration."), "id": String("ULpoHbM1bclOgetm5e2ltPP5Pse6NWOVfXOrkyCk6zWjC3_TSbnyU3AJdXzxkVei4osjI1B-hhZb0vfBN1qLMA"), "title": String("Introduction to ekoDB")}

Snippet 3:
  Collection: client_chat_basic_rust
  Score: 0.5222
  Matched Fields: ["title", "category", "content"]
  Record: Object {"category": String("features"), "content": String("ekoDB supports full-text search, vector search, and hybrid search with automatic context retrieval."), "id": String("IlgMY55i54ZxozVG2Gpf95opjS-y5_xOfC2Zyn2vFKpjVatE45gpCYsCh1EIqRbp-ATRHnNdxbVvQ_4pBVamsA"), "title": String("Search Features")}

Execution Time: 5845ms

=== Cleanup ===
✓ Deleted collection

✓ Chat completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_chat_advanced`
=== ekoDB Advanced Chat Features Example ===

=== Inserting Sample Data ===
✓ Inserted sample product

=== Creating Chat Session ===
✓ Created session: LYcAsppmU_7X5SsBjFQDulAM2wPdh0VUWLBUmpaJx-F1gAxrez_4AFVwktZZxwijKrvy4-asSZBXNpFQEq_iYw

=== Sending Initial Message ===
✓ Message sent
  Response: Based on the information provided, one available product is ekoDB. It is a high-performance database product, priced at $99.

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
✓ Created second session: rMNvVHF92_i7TijhVx4WnS4dvCmtzBovgjZTRYvcIN0B7CvdN7DWvV5EaABdJY8yCI4jBkpW6NQ8ZgVIrEc6Ig
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
✓ Created session: xyr5atcONAmY5b1CaBCT-SREXJ7oTLuhABQcN-smysHOm58aSEPqEZNkqsGJDwxG-Fkv9YCQvgkFZE4YWE5amw
=== Sending Messages ===
✓ Message 1 sent
  Response: The available product based on the context provided is "ekoDB". It is a high-performance database product that comes with AI capabilities. The price of ekoDB is $99.

✓ Message 2 sent
  Response: The price of the ekoDB product is $99.

=== Retrieving Session Messages ===
✓ Retrieved 4 messages

=== Updating Session ===
✓ Session updated

=== Branching Session ===
✓ Created branch: EVu8gOwaMnvDh23ZF_rfj4g7lavNi9yWJom62rkZd04NOuxL9CDiKqTc_nwVm1UEgM_s2WAJENXe8lKg0iKxtA
  Parent: xyr5atcONAmY5b1CaBCT-SREXJ7oTLuhABQcN-smysHOm58aSEPqEZNkqsGJDwxG-Fkv9YCQvgkFZE4YWE5amw

=== Listing Sessions ===
✓ Found 4 sessions
  Session 1: EVu8gOwaMnvDh23ZF_rfj4g7lavNi9yWJom62rkZd04NOuxL9CDiKqTc_nwVm1UEgM_s2WAJENXe8lKg0iKxtA (Untitled)
  Session 2: xyr5atcONAmY5b1CaBCT-SREXJ7oTLuhABQcN-smysHOm58aSEPqEZNkqsGJDwxG-Fkv9YCQvgkFZE4YWE5amw (Untitled)
  Session 3: rMNvVHF92_i7TijhVx4WnS4dvCmtzBovgjZTRYvcIN0B7CvdN7DWvV5EaABdJY8yCI4jBkpW6NQ8ZgVIrEc6Ig (Untitled)

=== Getting Session Details ===
✓ Session details retrieved
  Messages: 4

=== Deleting Branch Session ===
✓ Deleted branch session: EVu8gOwaMnvDh23ZF_rfj4g7lavNi9yWJom62rkZd04NOuxL9CDiKqTc_nwVm1UEgM_s2WAJENXe8lKg0iKxtA

=== Cleanup ===
✓ Deleted collection

✓ All session management operations completed successfully
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_chat_models`
✓ Client created

=== Get All Chat Models ===
Available chat models by provider:

OpenAI models (115):
  - gpt-4-0613
  - gpt-4
  - gpt-3.5-turbo
  - gpt-5.2-codex
  - gpt-4o-mini-tts-2025-12-15
  - gpt-realtime-mini-2025-12-15
  - gpt-audio-mini-2025-12-15
  - chatgpt-image-latest
  - davinci-002
  - babbage-002
  - gpt-3.5-turbo-instruct
  - gpt-3.5-turbo-instruct-0914
  - dall-e-3
  - dall-e-2
  - gpt-4-1106-preview
  - gpt-3.5-turbo-1106
  - tts-1-hd
  - tts-1-1106
  - tts-1-hd-1106
  - text-embedding-3-small
  - text-embedding-3-large
  - gpt-4-0125-preview
  - gpt-4-turbo-preview
  - gpt-3.5-turbo-0125
  - gpt-4-turbo
  - gpt-4-turbo-2024-04-09
  - gpt-4o
  - gpt-4o-2024-05-13
  - gpt-4o-mini-2024-07-18
  - gpt-4o-mini
  - gpt-4o-2024-08-06
  - chatgpt-4o-latest
  - gpt-4o-audio-preview
  - gpt-4o-realtime-preview
  - omni-moderation-latest
  - omni-moderation-2024-09-26
  - gpt-4o-realtime-preview-2024-12-17
  - gpt-4o-audio-preview-2024-12-17
  - gpt-4o-mini-realtime-preview-2024-12-17
  - gpt-4o-mini-audio-preview-2024-12-17
  - o1-2024-12-17
  - o1
  - gpt-4o-mini-realtime-preview
  - gpt-4o-mini-audio-preview
  - o3-mini
  - o3-mini-2025-01-31
  - gpt-4o-2024-11-20
  - gpt-4o-search-preview-2025-03-11
  - gpt-4o-search-preview
  - gpt-4o-mini-search-preview-2025-03-11
  - gpt-4o-mini-search-preview
  - gpt-4o-transcribe
  - gpt-4o-mini-transcribe
  - o1-pro-2025-03-19
  - o1-pro
  - gpt-4o-mini-tts
  - o3-2025-04-16
  - o4-mini-2025-04-16
  - o3
  - o4-mini
  - gpt-4.1-2025-04-14
  - gpt-4.1
  - gpt-4.1-mini-2025-04-14
  - gpt-4.1-mini
  - gpt-4.1-nano-2025-04-14
  - gpt-4.1-nano
  - gpt-image-1
  - codex-mini-latest
  - gpt-4o-realtime-preview-2025-06-03
  - gpt-4o-audio-preview-2025-06-03
  - o4-mini-deep-research
  - gpt-4o-transcribe-diarize
  - o4-mini-deep-research-2025-06-26
  - gpt-5-chat-latest
  - gpt-5-2025-08-07
  - gpt-5
  - gpt-5-mini-2025-08-07
  - gpt-5-mini
  - gpt-5-nano-2025-08-07
  - gpt-5-nano
  - gpt-audio-2025-08-28
  - gpt-realtime
  - gpt-realtime-2025-08-28
  - gpt-audio
  - gpt-5-codex
  - gpt-image-1-mini
  - gpt-5-pro-2025-10-06
  - gpt-5-pro
  - gpt-audio-mini
  - gpt-audio-mini-2025-10-06
  - gpt-5-search-api
  - gpt-realtime-mini
  - gpt-realtime-mini-2025-10-06
  - sora-2
  - sora-2-pro
  - gpt-5-search-api-2025-10-14
  - gpt-5.1-chat-latest
  - gpt-5.1-2025-11-13
  - gpt-5.1
  - gpt-5.1-codex
  - gpt-5.1-codex-mini
  - gpt-5.1-codex-max
  - gpt-image-1.5
  - gpt-5.2-2025-12-11
  - gpt-5.2
  - gpt-5.2-pro-2025-12-11
  - gpt-5.2-pro
  - gpt-5.2-chat-latest
  - gpt-4o-mini-transcribe-2025-12-15
  - gpt-4o-mini-transcribe-2025-03-20
  - gpt-4o-mini-tts-2025-03-20
  - gpt-3.5-turbo-16k
  - tts-1
  - whisper-1
  - text-embedding-ada-002

Anthropic models (9):
  - claude-opus-4-5-20251101
  - claude-haiku-4-5-20251001
  - claude-sonnet-4-5-20250929
  - claude-opus-4-1-20250805
  - claude-opus-4-20250514
  - claude-sonnet-4-20250514
  - claude-3-7-sonnet-20250219
  - claude-3-5-haiku-20241022
  - claude-3-haiku-20240307

Perplexity models (5):
  - sonar
  - sonar-pro
  - sonar-deep-research
  - sonar-reasoning
  - sonar-reasoning-pro

=== Get Models for Specific Provider ===

openai models (115):
  - gpt-4-0613
  - gpt-4
  - gpt-3.5-turbo
  - gpt-5.2-codex
  - gpt-4o-mini-tts-2025-12-15
  ... and 110 more

anthropic models (9):
  - claude-opus-4-5-20251101
  - claude-haiku-4-5-20251001
  - claude-sonnet-4-5-20250929
  - claude-opus-4-1-20250805
  - claude-opus-4-20250514
  ... and 4 more

perplexity models (5):
  - sonar
  - sonar-pro
  - sonar-deep-research
  - sonar-reasoning
  - sonar-reasoning-pro

✓ Chat Models API example complete
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_user_functions`
✓ Client created

=== Create User Function ===
Created user function with ID: LvuMBBbvfCowttFbgceQhi7xkQItcvv4uvVtoVVQiCTlfCAoxh3SaUMSWVhH4umTyE9_2MBtcOiFg6vV9AgE_g

=== Get User Function ===
Retrieved: get_active_users_rs - Get Active Users
Description: Fetches all users and filters by active status

=== List All User Functions ===
Found 13 user functions:
  - get_high_scoring_active_users: Get High Scoring Active Users
  - swr_user: SWR pattern for user data (KV-based)
  - get_user_wrapper: Wrapper that calls fetch_user
  - validate_user: Check if user exists
  - get_active_users: Get Active Users (Updated)
  - fetch_user: Fetch user by code
  - fetch_and_store_user: Fetch user from API and cache in KV
  - fetch_slim_user: Validate and slim down user
  - get_active_users_updated: Get Active Users (Updated)
  - get_verified_user: Get verified and validated user
  - get_active_users_rs: Get Active Users
  - get_users_by_status: Get Users By Status
  - get_active_users_paginated: Get Active Users (Paginated)

=== Update User Function ===
User function updated successfully

=== Delete User Function ===
User function deleted successfully

✓ User Functions API example complete
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/client_convenience_methods`
=== ekoDB Convenience Methods Example ===

=== Record Builder Pattern ===
✓ Created record with fluent builder: Record({"id": String("oDQ7NVxsmw_hf5UJnw5Pc8dGkpL3q25tajGOaS3rlpMyRZep8c9kUevtcKiOTlR6YSMup9NcbmBPfO4ZtjkBSg")})

=== Upsert Operation ===
✓ First upsert (insert): Record({"age": Object({"type": String("Integer"), "value": Integer(35)}), "name": Object({"type": String("String"), "value": String("Bob Smith")}), "email": Object({"value": String("bob@example.com"), "type": String("String")}), "active": Object({"value": Boolean(true), "type": String("Boolean")}), "id": String("oDQ7NVxsmw_hf5UJnw5Pc8dGkpL3q25tajGOaS3rlpMyRZep8c9kUevtcKiOTlR6YSMup9NcbmBPfO4ZtjkBSg")})
✓ Second upsert (update): Record({"active": Object({"value": Boolean(true), "type": String("Boolean")}), "email": Object({"value": String("bob.smith@newdomain.com"), "type": String("String")}), "name": Object({"value": String("Bob Smith"), "type": String("String")}), "id": String("oDQ7NVxsmw_hf5UJnw5Pc8dGkpL3q25tajGOaS3rlpMyRZep8c9kUevtcKiOTlR6YSMup9NcbmBPfO4ZtjkBSg"), "age": Object({"value": Integer(36), "type": String("Integer")})})

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
   Inserted with ripple: Record({"id": String("8mCfCBZDh7uPhV7QaR0mi9qWpKCoNzN5KwbKAlYf83ZMZg7bfu_zbnG3cFey89eIdzeQp38Wi7hDGeT2SdIwpQ")})

2. Insert with bypass_ripple:
   Inserted with bypass_ripple: Record({"id": String("p58uRHGJd7ep2WBSe6cIQDq5yj-aWYzgG157opF-CPeyGzFu4_hzxJKcouQWVKWUQGl60KXIkJp5A_D_J_xVJw")})

3. Update with bypass_ripple:
   Updated with bypass_ripple: Record({"name": Object({"type": String("String"), "value": String("Product 1")}), "price": Object({"type": String("Integer"), "value": Integer(150)}), "id": String("8mCfCBZDh7uPhV7QaR0mi9qWpKCoNzN5KwbKAlYf83ZMZg7bfu_zbnG3cFey89eIdzeQp38Wi7hDGeT2SdIwpQ")})

4. Delete with bypass_ripple:
   Deleted with bypass_ripple

5. Batch insert with bypass_ripple:
   Batch inserted with bypass_ripple: 2 records

6. Upsert with bypass_ripple:
   Upserted with bypass_ripple: Record({"id": String("4dF2_-T2tZiygA7CgvNyX0SEIKm9PIG6hhUYePCefv0tWNnDIIbIR6uAK5R84SwtOzKsrpbqfFJ30l0W0Q20cw")})

✅ All bypass_ripple operations completed successfully!
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/projection_example`
Client created

Setting up test data...
Inserted 4 test users

Example 1: Select specific fields (id, name, email only)
  Found 3 active users
  Fields returned: ["email", "id", "name"]

Example 2: Exclude sensitive fields (password, api_key, secret_token)
  Found 2 admins
  Sensitive fields excluded:
    - password: excluded
    - api_key: excluded
    - secret_token: excluded
  Fields returned: ["bio", "avatar_url", "id", "status", "email", "name", "created_at", "user_role", "age"]

Example 3: Complex query with projection (active users, ages 18-65)
  Found 3 active users (ages 18-65)

Example 4: Query inactive users with profile fields
  Found 1 inactive users

Example 5: Compare full vs projected data
  Full query:
    - 12 fields per record
    - Fields: ["user_role", "name", "status", "secret_token", "bio", "email", "age", "created_at", "avatar_url", "id", "password", "api_key"]
  Projected query:
    - 3 fields per record
    - Fields: ["id", "name", "email"]
  Bandwidth savings: ~75% fewer fields

Cleaning up test data...
Cleanup complete

All projection examples completed successfully!
✅ [32mRust client examples complete![0m
